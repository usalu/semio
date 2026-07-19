//! ✨ The closed set of named functions the kernel understands, plus the small amount of per-kind
//! metadata (arity, display name) needed by the kernel itself; derivative/series/evaluation rules are
//! added in `diff.rs`/`series.rs`/`evalf.rs` as those domains land, keeping this file the single
//! registry of "what a function *is*" while the other files describe "what it *does*".

// #region 🔖FnKind
/// ✨ Closed enum of built-in named functions, plus an escape hatch for user-defined ones.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum FnKind {
    Sin,
    Cos,
    Tan,
    Cot,
    Sec,
    Csc,
    Asin,
    Acos,
    Atan,
    Acot,
    Asec,
    Acsc,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Exp,
    Ln,
    Abs,
    Sign,
    Floor,
    Ceil,
    Gamma,
    LogGamma,
    Erf,
    Erfc,
    Zeta,
    BesselJ,
    BesselY,
    BesselI,
    BesselK,
    LegendreP,
    ChebyshevT,
    ChebyshevU,
    HermiteH,
    LaguerreL,
    LambertW,
    /// 🔧 A user-defined named function, opaque to the kernel's built-in identity/derivative tables.
    UserFn(std::rc::Rc<str>),
}

impl FnKind {
    /// 🔢 Fixed arity, or `None` for the two families whose argument count varies (Bessel/orthogonal
    /// functions carry an order/degree argument in addition to their evaluation point).
    pub fn arity(&self) -> Option<usize> {
        use FnKind::*;
        match self {
            Sin | Cos | Tan | Cot | Sec | Csc | Asin | Acos | Atan | Acot | Asec | Acsc | Sinh | Cosh | Tanh | Asinh | Acosh | Atanh | Exp | Ln | Abs | Sign | Floor | Ceil | Gamma
            | LogGamma | Erf | Erfc | Zeta => Some(1),
            BesselJ | BesselY | BesselI | BesselK | LegendreP | ChebyshevT | ChebyshevU | HermiteH | LaguerreL => Some(2),
            LambertW => Some(1),
            UserFn(_) => None,
        }
    }

    pub fn name(&self) -> std::borrow::Cow<'static, str> {
        use FnKind::*;
        match self {
            Sin => "sin".into(),
            Cos => "cos".into(),
            Tan => "tan".into(),
            Cot => "cot".into(),
            Sec => "sec".into(),
            Csc => "csc".into(),
            Asin => "asin".into(),
            Acos => "acos".into(),
            Atan => "atan".into(),
            Acot => "acot".into(),
            Asec => "asec".into(),
            Acsc => "acsc".into(),
            Sinh => "sinh".into(),
            Cosh => "cosh".into(),
            Tanh => "tanh".into(),
            Asinh => "asinh".into(),
            Acosh => "acosh".into(),
            Atanh => "atanh".into(),
            Exp => "exp".into(),
            Ln => "ln".into(),
            Abs => "abs".into(),
            Sign => "sign".into(),
            Floor => "floor".into(),
            Ceil => "ceil".into(),
            Gamma => "gamma".into(),
            LogGamma => "loggamma".into(),
            Erf => "erf".into(),
            Erfc => "erfc".into(),
            Zeta => "zeta".into(),
            BesselJ => "besselj".into(),
            BesselY => "bessely".into(),
            BesselI => "besseli".into(),
            BesselK => "besselk".into(),
            LegendreP => "legendreP".into(),
            ChebyshevT => "chebyshevT".into(),
            ChebyshevU => "chebyshevU".into(),
            HermiteH => "hermiteH".into(),
            LaguerreL => "laguerreL".into(),
            LambertW => "lambertw".into(),
            UserFn(name) => name.to_string().into(),
        }
    }

    /// 🔄 `true` for functions with `f(-x) == f(x)`.
    pub fn is_even(&self) -> bool {
        matches!(self, FnKind::Cos | FnKind::Cosh | FnKind::Abs)
    }

    /// 🔄 `true` for functions with `f(-x) == -f(x)`.
    pub fn is_odd(&self) -> bool {
        matches!(self, FnKind::Sin | FnKind::Tan | FnKind::Cot | FnKind::Csc | FnKind::Sinh | FnKind::Tanh | FnKind::Asin | FnKind::Atan | FnKind::Asinh | FnKind::Atanh | FnKind::Sign | FnKind::Erf)
    }
}
// #endregion 🔖FnKind

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arity_hand_cases() {
        assert_eq!(FnKind::Sin.arity(), Some(1));
        assert_eq!(FnKind::BesselJ.arity(), Some(2));
        assert_eq!(FnKind::UserFn("f".into()).arity(), None);
    }

    #[test]
    fn parity_hand_cases() {
        assert!(FnKind::Cos.is_even());
        assert!(FnKind::Sin.is_odd());
        assert!(!FnKind::Exp.is_even() && !FnKind::Exp.is_odd());
    }

    #[test]
    fn name_hand_cases() {
        assert_eq!(FnKind::Sin.name(), "sin");
        assert_eq!(FnKind::UserFn("myFunc".into()).name(), "myFunc");
    }
}
// #endregion 🔖Tests
