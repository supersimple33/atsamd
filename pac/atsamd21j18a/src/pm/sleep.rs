#[doc = "Reader of register SLEEP"]
pub type R = crate::R<u8, super::SLEEP>;
#[doc = "Writer for register SLEEP"]
pub type W = crate::W<u8, super::SLEEP>;
#[doc = "Register SLEEP `reset()`'s with value 0"]
impl crate::ResetValue for super::SLEEP {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_A {
    #[doc = "The CPU clock domain is stopped"]
    CPU,
    #[doc = "The CPU and AHB clock domains are stopped"]
    AHB,
    #[doc = "The CPU, AHB and APB clock domains are stopped"]
    APB,
}
impl crate::ToBits<u8> for IDLE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            IDLE_A::CPU => 0,
            IDLE_A::AHB => 1,
            IDLE_A::APB => 2,
        }
    }
}
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<u8, IDLE_A>;
impl IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDLE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IDLE_A::CPU),
            1 => Val(IDLE_A::AHB),
            2 => Val(IDLE_A::APB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CPU`"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == IDLE_A::CPU
    }
    #[doc = "Checks if the value of the field is `AHB`"]
    #[inline(always)]
    pub fn is_ahb(&self) -> bool {
        *self == IDLE_A::AHB
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == IDLE_A::APB
    }
}
#[doc = "Write proxy for field `IDLE`"]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLE_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The CPU clock domain is stopped"]
    #[inline(always)]
    pub fn cpu(self) -> &'a mut W {
        self.variant(IDLE_A::CPU)
    }
    #[doc = "The CPU and AHB clock domains are stopped"]
    #[inline(always)]
    pub fn ahb(self) -> &'a mut W {
        self.variant(IDLE_A::AHB)
    }
    #[doc = "The CPU, AHB and APB clock domains are stopped"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(IDLE_A::APB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
    }
}