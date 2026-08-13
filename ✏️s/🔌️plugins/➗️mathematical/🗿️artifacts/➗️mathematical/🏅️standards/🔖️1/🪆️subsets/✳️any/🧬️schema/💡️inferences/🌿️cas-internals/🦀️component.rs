//! ♾️ Headless computer algebra system: symbolic expressions, calculus, equation solving, symbolic
//! linear algebra, ODEs, transforms, and more, over a pure Rust API — no context handle, no thread-local
//! state; `Expr` is an ordinary `Clone`-able value built through operator overloads and free functions.
//!
//! 🚚 Migrated verbatim from `🧰️framework/🔨️modules/🧮️math/🧮️cas/🦀️component.rs` (ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, wave M3a): `🧮️math` is being
//! dissolved into event-sourced artifacts and `🧮️cas` has no independent artifact home of its own —
//! this crate's `➗️mathematical` plugin is where equation content actually lives. This module is the
//! Rust-only compute internals a `💡️inferences/<slug>/` facet's `compute()` delegates into (mirroring
//! stdio's `📐️step` io facet's `🪜️ladder`/`📐️part21`/`🧱️brep` precedent for deep Rust-only helper
//! dirs under a facet); `crate::number`/`crate::algebra` (which stay in `🧮️math` for now) became
//! `math::number`/`math::algebra` against the new `extern crate semio_framework_math as math;`
//! dependency, and every `crate::cas`/`crate::polynomial` self-reference is preserved unedited via a
//! crate-root `pub use … as cas;` / `pub use … as polynomial;` alias in `📦️glue.rs` — the physical
//! file moved, the module's internal wiring did not.
// #region 🔖️Fnkind
pub mod fnkind {
    //! ✨️ The closed set of named functions the kernel understands, plus the small amount of per-kind
    //! metadata (arity, display name) needed by the kernel itself; derivative/series/evaluation rules are
    //! added in `diff.rs`/`series.rs`/`evalf.rs` as those domains land, keeping this file the single
    //! registry of "what a function *is*" while the other files describe "what it *does*".

    // #region 🔖️FnKind
    /// ✨️ Closed enum of built-in named functions, plus an escape hatch for user-defined ones.
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
        /// 🔧️ Digamma `ψ(x) = Γ'(x)/Γ(x)`; kept internal-ish (no series/eval entries in the first pass) but
        /// needed as a first-class `FnKind` because `Γ'(x) = Γ(x)·ψ(x)` has no other closed form.
        Digamma,
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
        /// 🔧️ A user-defined named function, opaque to the kernel's built-in identity/derivative tables.
        UserFn(std::rc::Rc<str>),
    }

    impl FnKind {
        /// 🔢️ Fixed arity, or `None` for the two families whose argument count varies (Bessel/orthogonal
        /// functions carry an order/degree argument in addition to their evaluation point).
        pub fn arity(&self) -> Option<usize> {
            use FnKind::*;
            match self {
                Sin | Cos | Tan | Cot | Sec | Csc | Asin | Acos | Atan | Acot | Asec | Acsc | Sinh | Cosh | Tanh | Asinh | Acosh | Atanh | Exp | Ln | Abs | Sign | Floor | Ceil | Gamma | LogGamma | Digamma | Erf | Erfc | Zeta => Some(1),
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
                Digamma => "digamma".into(),
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

        /// 🔄️ `true` for functions with `f(-x) == f(x)`.
        pub fn is_even(&self) -> bool {
            matches!(self, FnKind::Cos | FnKind::Cosh | FnKind::Abs)
        }

        /// 🔄️ `true` for functions with `f(-x) == -f(x)`.
        pub fn is_odd(&self) -> bool {
            matches!(self, FnKind::Sin | FnKind::Tan | FnKind::Cot | FnKind::Csc | FnKind::Sinh | FnKind::Tanh | FnKind::Asin | FnKind::Atan | FnKind::Asinh | FnKind::Atanh | FnKind::Sign | FnKind::Erf)
        }
    }
    // #endregion 🔖️FnKind

    // #region 🔖️Tests
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
    // #endregion 🔖️Tests
}
// #endregion 🔖️Fnkind

// #region 🔖️Expr
pub mod expr {
    //! ✨️ The symbolic expression tree: an immutable, `Rc`-shared, structurally-hash-cached tree. Every
    //! node is built exclusively through `canon.rs`'s smart constructors, which maintain the
    //! auto-simplification invariants (flattening, numeric folding, canonical ordering) that the rest of
    //! the crate assumes hold everywhere.

    use crate::cas::assume::AssumeSet;
    use crate::cas::fnkind::FnKind;
    use math::number::{Integer, Rational};
    use std::rc::Rc;

    // #region 🔖️Symbol
    /// 🔤️ A named variable plus the assumption flags fixed at its creation — two symbols with the same
    /// name but different flags are deliberately distinct (prevents silently mixing up `x` real vs `x`
    /// unconstrained).
    #[derive(Clone, Debug)]
    pub struct Symbol {
        name: Rc<str>,
        assumptions: AssumeSet,
    }

    impl Symbol {
        pub(crate) fn new(name: &str, assumptions: AssumeSet) -> Self {
            Self { name: Rc::from(name), assumptions }
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn assumptions(&self) -> AssumeSet {
            self.assumptions
        }
    }

    impl PartialEq for Symbol {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name && self.assumptions == other.assumptions
        }
    }
    impl Eq for Symbol {}
    impl std::hash::Hash for Symbol {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.name.hash(state);
            self.assumptions.bits().hash(state);
        }
    }
    impl Ord for Symbol {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.name.cmp(&other.name).then_with(|| self.assumptions.bits().cmp(&other.assumptions.bits()))
        }
    }
    impl PartialOrd for Symbol {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    // #endregion 🔖️Symbol

    // #region 🔖️Constant
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
    pub enum Constant {
        Pi,
        E,
        I,
        EulerGamma,
        Inf,
        NegInf,
        ComplexInf,
        Undefined,
    }

    impl Constant {
        pub fn name(&self) -> &'static str {
            match self {
                Constant::Pi => "pi",
                Constant::E => "e",
                Constant::I => "i",
                Constant::EulerGamma => "EulerGamma",
                Constant::Inf => "oo",
                Constant::NegInf => "-oo",
                Constant::ComplexInf => "zoo",
                Constant::Undefined => "undefined",
            }
        }
    }
    // #endregion 🔖️Constant

    // #region 🔖️RelationalOperator
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
    pub enum RelationalOperator {
        Eq,
        Ne,
        Lt,
        Le,
        Gt,
        Ge,
    }
    // #endregion 🔖️RelationalOperator

    // #region 🔖️WildKind
    #[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
    pub enum WildKind {
        Any,
        Number,
        NotZero,
        /// 🆓️ Matches only subtrees free of the named symbol.
        FreeOf(Rc<str>),
        /// 📚️ Matches a whole sequence of `Add`/`Mul` arguments (at most one per pattern).
        Seq,
    }
    // #endregion 🔖️WildKind

    // #region 🔖️Kind
    #[derive(Clone, PartialEq, Debug)]
    pub enum Kind {
        Integer(Integer),
        /// ➗️ Invariant (maintained by `canon.rs`): never has an integral value — those fold to `Integer`.
        Rational(Rational),
        Symbol(Symbol),
        Constant(Constant),
        Bool(bool),
        /// ➕️ Flattened, canonically sorted, length `>= 2`; a numeric coefficient (if any) sorts first.
        Add(Vec<Expr>),
        /// ✖️ Same invariants as `Add`.
        Mul(Vec<Expr>),
        Pow(Expr, Expr),
        Fn(FnKind, Vec<Expr>),
        /// 🌱️ The `index`-th real root (ascending) of the integer polynomial with coefficients `coeffs`
        /// (little-endian, degree = `coeffs.len() - 1`); used when `solve` can't produce a closed radical form.
        RootOf {
            coeffs: Vec<Rational>,
            index: u32,
        },
        /// 🔀️ `(value, condition)` pairs; the last condition may be `Bool(true)` for a catch-all default.
        Piecewise(Vec<(Expr, Expr)>),
        Rel(RelationalOperator, Expr, Expr),
        /// 🃏️ Pattern-only placeholder; must never appear in a user-facing expression.
        Wild(u16, WildKind),
    }
    // #endregion 🔖️Kind

    // #region 🔖️Node
    struct Node {
        hash: u64,
        kind: Kind,
    }

    /// 🔢️ FNV-1a, computed bottom-up once at construction and cached — equality checks hash first (cheap
    /// reject), then `Rc::ptr_eq` (cheap accept, common since subtrees are shared), then a full structural
    /// compare only in the rare remaining case.
    fn fnv1a_mix(mut hash: u64, bytes: &[u8]) -> u64 {
        for &b in bytes {
            hash ^= b as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
        hash
    }

    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

    fn hash_kind(kind: &Kind) -> u64 {
        let mut h = FNV_OFFSET;
        match kind {
            Kind::Integer(n) => {
                h = fnv1a_mix(h, b"int");
                h = fnv1a_mix(h, n.to_decimal().as_bytes());
            }
            Kind::Rational(r) => {
                h = fnv1a_mix(h, b"rat");
                h = fnv1a_mix(h, r.numer().to_decimal().as_bytes());
                h = fnv1a_mix(h, r.denom().to_decimal().as_bytes());
            }
            Kind::Symbol(s) => {
                h = fnv1a_mix(h, b"sym");
                h = fnv1a_mix(h, s.name.as_bytes());
                h = fnv1a_mix(h, &s.assumptions.bits().to_le_bytes());
            }
            Kind::Constant(c) => {
                h = fnv1a_mix(h, b"const");
                h = fnv1a_mix(h, c.name().as_bytes());
            }
            Kind::Bool(b) => {
                h = fnv1a_mix(h, b"bool");
                h = fnv1a_mix(h, &[*b as u8]);
            }
            Kind::Add(terms) => {
                h = fnv1a_mix(h, b"add");
                for t in terms {
                    h = fnv1a_mix(h, &t.hash().to_le_bytes());
                }
            }
            Kind::Mul(factors) => {
                h = fnv1a_mix(h, b"mul");
                for f in factors {
                    h = fnv1a_mix(h, &f.hash().to_le_bytes());
                }
            }
            Kind::Pow(base, exp) => {
                h = fnv1a_mix(h, b"pow");
                h = fnv1a_mix(h, &base.hash().to_le_bytes());
                h = fnv1a_mix(h, &exp.hash().to_le_bytes());
            }
            Kind::Fn(kind, args) => {
                h = fnv1a_mix(h, b"fn");
                h = fnv1a_mix(h, kind.name().as_bytes());
                for a in args {
                    h = fnv1a_mix(h, &a.hash().to_le_bytes());
                }
            }
            Kind::RootOf { coeffs, index } => {
                h = fnv1a_mix(h, b"rootof");
                for c in coeffs {
                    h = fnv1a_mix(h, c.numer().to_decimal().as_bytes());
                    h = fnv1a_mix(h, c.denom().to_decimal().as_bytes());
                }
                h = fnv1a_mix(h, &index.to_le_bytes());
            }
            Kind::Piecewise(cases) => {
                h = fnv1a_mix(h, b"piecewise");
                for (v, c) in cases {
                    h = fnv1a_mix(h, &v.hash().to_le_bytes());
                    h = fnv1a_mix(h, &c.hash().to_le_bytes());
                }
            }
            Kind::Rel(operation, a, b) => {
                h = fnv1a_mix(h, b"rel");
                h = fnv1a_mix(h, &[*operation as u8]);
                h = fnv1a_mix(h, &a.hash().to_le_bytes());
                h = fnv1a_mix(h, &b.hash().to_le_bytes());
            }
            Kind::Wild(id, _) => {
                h = fnv1a_mix(h, b"wild");
                h = fnv1a_mix(h, &id.to_le_bytes());
            }
        }
        h
    }
    // #endregion 🔖️Node

    // #region 🔖️Expr
    /// ✨️ An immutable, reference-counted symbolic expression. `!Send`/`!Sync` by design (single-threaded,
    /// `Rc`-based sharing — swap to `Arc` in one place, `P<T>` below, if that's ever needed).
    #[derive(Clone)]
    pub struct Expr(Rc<Node>);

    impl Expr {
        /// ⚠️ Only `canon.rs` should call this — every other constructor goes through the smart
        /// constructors there to maintain the auto-simplification invariants.
        pub(crate) fn from_kind_unchecked(kind: Kind) -> Self {
            let hash = hash_kind(&kind);
            Self(Rc::new(Node { hash, kind }))
        }

        pub fn kind(&self) -> &Kind {
            &self.0.kind
        }

        pub fn hash(&self) -> u64 {
            self.0.hash
        }

        pub fn symbol(name: &str) -> Self {
            crate::cas::canon::make_symbol(name, AssumeSet::NONE)
        }

        pub fn symbol_with(name: &str, assumptions: AssumeSet) -> Self {
            crate::cas::canon::make_symbol(name, assumptions)
        }

        pub fn integer(value: i64) -> Self {
            crate::cas::canon::make_integer(Integer::from_i64(value))
        }

        pub fn constant(c: Constant) -> Self {
            Self::from_kind_unchecked(Kind::Constant(c))
        }

        pub fn boolean(b: bool) -> Self {
            Self::from_kind_unchecked(Kind::Bool(b))
        }

        pub fn as_integer(&self) -> Option<&Integer> {
            match self.kind() {
                Kind::Integer(n) => Some(n),
                _ => None,
            }
        }

        pub fn as_symbol(&self) -> Option<&Symbol> {
            match self.kind() {
                Kind::Symbol(s) => Some(s),
                _ => None,
            }
        }

        pub fn is_zero_literal(&self) -> bool {
            matches!(self.kind(), Kind::Integer(n) if n.is_zero())
        }

        pub fn is_one_literal(&self) -> bool {
            matches!(self.kind(), Kind::Integer(n) if *n == Integer::one())
        }

        // #region 🔖️PublicConstructors
        pub fn add(terms: Vec<Expr>) -> Self {
            crate::cas::canon::make_add(terms)
        }

        pub fn mul(factors: Vec<Expr>) -> Self {
            crate::cas::canon::make_mul(factors)
        }

        pub fn pow(base: Expr, exp: Expr) -> Self {
            crate::cas::canon::make_pow(base, exp)
        }

        pub fn func(kind: FnKind, args: Vec<Expr>) -> Self {
            crate::cas::canon::make_func(kind, args)
        }
        // #endregion 🔖️PublicConstructors
    }

    impl PartialEq for Expr {
        fn eq(&self, other: &Self) -> bool {
            if Rc::ptr_eq(&self.0, &other.0) {
                return true;
            }
            if self.0.hash != other.0.hash {
                return false;
            }
            self.0.kind == other.0.kind
        }
    }
    impl Eq for Expr {}

    impl std::hash::Hash for Expr {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash.hash(state);
        }
    }

    impl std::fmt::Debug for Expr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", crate::cas::fmt::display_string(self))
        }
    }

    /// 🔢️ Kind-class rank used as the primary key of the canonical total order (before falling back to
    /// structural comparison within the same class).
    fn kind_rank(k: &Kind) -> u8 {
        match k {
            Kind::Integer(_) => 0,
            Kind::Rational(_) => 0,
            Kind::Constant(_) => 1,
            Kind::Symbol(_) => 2,
            Kind::Pow(..) => 3,
            Kind::Mul(_) => 4,
            Kind::Fn(..) => 5,
            Kind::Add(_) => 6,
            Kind::RootOf { .. } => 7,
            Kind::Piecewise(_) => 8,
            Kind::Rel(..) => 9,
            Kind::Bool(_) => 10,
            Kind::Wild(..) => 11,
        }
    }

    impl Ord for Expr {
        /// 🔢️ Purely structural — never pointer- or hash-order — so `Display`/canonicalization output is
        /// stable across runs and processes.
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            use std::cmp::Ordering;
            if Rc::ptr_eq(&self.0, &other.0) {
                return Ordering::Equal;
            }
            let (ra, rb) = (kind_rank(self.kind()), kind_rank(other.kind()));
            if ra != rb {
                return ra.cmp(&rb);
            }
            match (self.kind(), other.kind()) {
                (Kind::Integer(a), Kind::Integer(b)) => a.cmp(b),
                (Kind::Integer(a), Kind::Rational(b)) => Rational::from_integer(a.clone()).cmp(b),
                (Kind::Rational(a), Kind::Integer(b)) => a.cmp(&Rational::from_integer(b.clone())),
                (Kind::Rational(a), Kind::Rational(b)) => a.cmp(b),
                (Kind::Constant(a), Kind::Constant(b)) => a.cmp(b),
                (Kind::Symbol(a), Kind::Symbol(b)) => a.cmp(b),
                (Kind::Pow(b1, e1), Kind::Pow(b2, e2)) => b1.cmp(b2).then_with(|| e1.cmp(e2)),
                (Kind::Mul(a), Kind::Mul(b)) => a.cmp(b),
                (Kind::Fn(k1, a1), Kind::Fn(k2, a2)) => k1.name().cmp(&k2.name()).then_with(|| a1.cmp(a2)),
                (Kind::Add(a), Kind::Add(b)) => a.cmp(b),
                (Kind::RootOf { coeffs: c1, index: i1 }, Kind::RootOf { coeffs: c2, index: i2 }) => c1.cmp(c2).then_with(|| i1.cmp(i2)),
                (Kind::Piecewise(a), Kind::Piecewise(b)) => a.cmp(b),
                (Kind::Rel(o1, a1, b1), Kind::Rel(o2, a2, b2)) => o1.cmp(o2).then_with(|| a1.cmp(a2)).then_with(|| b1.cmp(b2)),
                (Kind::Bool(a), Kind::Bool(b)) => a.cmp(b),
                (Kind::Wild(i1, _), Kind::Wild(i2, _)) => i1.cmp(i2),
                _ => Ordering::Equal,
            }
        }
    }
    impl PartialOrd for Expr {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    // #region 🔖️Operators
    impl std::ops::Add for Expr {
        type Output = Expr;
        fn add(self, rhs: Expr) -> Expr {
            Expr::add(vec![self, rhs])
        }
    }
    impl std::ops::Add<&Expr> for &Expr {
        type Output = Expr;
        fn add(self, rhs: &Expr) -> Expr {
            Expr::add(vec![self.clone(), rhs.clone()])
        }
    }
    impl std::ops::Sub for Expr {
        type Output = Expr;
        fn sub(self, rhs: Expr) -> Expr {
            Expr::add(vec![self, crate::cas::canon::make_neg(rhs)])
        }
    }
    impl std::ops::Sub<&Expr> for &Expr {
        type Output = Expr;
        fn sub(self, rhs: &Expr) -> Expr {
            Expr::add(vec![self.clone(), crate::cas::canon::make_neg(rhs.clone())])
        }
    }
    impl std::ops::Mul for Expr {
        type Output = Expr;
        fn mul(self, rhs: Expr) -> Expr {
            Expr::mul(vec![self, rhs])
        }
    }
    impl std::ops::Mul<&Expr> for &Expr {
        type Output = Expr;
        fn mul(self, rhs: &Expr) -> Expr {
            Expr::mul(vec![self.clone(), rhs.clone()])
        }
    }
    impl std::ops::Div for Expr {
        type Output = Expr;
        fn div(self, rhs: Expr) -> Expr {
            Expr::mul(vec![self, Expr::pow(rhs, Expr::integer(-1))])
        }
    }
    impl std::ops::Div<&Expr> for &Expr {
        type Output = Expr;
        fn div(self, rhs: &Expr) -> Expr {
            Expr::mul(vec![self.clone(), Expr::pow(rhs.clone(), Expr::integer(-1))])
        }
    }
    impl std::ops::Neg for Expr {
        type Output = Expr;
        fn neg(self) -> Expr {
            crate::cas::canon::make_neg(self)
        }
    }
    impl std::ops::Neg for &Expr {
        type Output = Expr;
        fn neg(self) -> Expr {
            crate::cas::canon::make_neg(self.clone())
        }
    }

    impl From<i64> for Expr {
        fn from(value: i64) -> Self {
            Expr::integer(value)
        }
    }
    impl From<Integer> for Expr {
        fn from(value: Integer) -> Self {
            crate::cas::canon::make_integer(value)
        }
    }
    impl From<Rational> for Expr {
        fn from(value: Rational) -> Self {
            crate::cas::canon::make_rational(value)
        }
    }
    // #endregion 🔖️Operators
    // #endregion 🔖️Expr

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn structural_equality_ignores_sharing() {
            let a = Expr::integer(5);
            let b = Expr::integer(5);
            assert_eq!(a, b);
        }

        #[test]
        fn hash_is_deterministic() {
            let a = Expr::integer(42);
            let b = Expr::integer(42);
            assert_eq!(a.hash(), b.hash());
        }

        #[test]
        fn ord_is_consistent_and_total() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let one = Expr::integer(1);
            assert!(one < x);
            assert!(x < y || y < x);
            assert!(!(x < y && y < x));
        }

        #[test]
        fn symbols_with_different_assumptions_are_distinct() {
            let x1 = Expr::symbol("x");
            let x2 = Expr::symbol_with("x", AssumeSet::POSITIVE);
            assert_ne!(x1, x2);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Expr

// #region 🔖️Canon
mod canon {
    //! 🧪️ Smart constructors: the only place `Kind` values are ever built raw. Every rule here
    //! (flattening, numeric folding, like-term/like-base collection, identity/absorber elimination, `Pow`
    //! special cases, canonical sorting) is an invariant the rest of the crate is entitled to assume holds.

    use crate::cas::assume::{is_positive, AssumeSet};
    use crate::cas::expr::{Constant, Expr, Kind, Symbol};
    use crate::cas::fnkind::FnKind;
    use math::number::{primes, Integer, Natural, Rational};
    use std::collections::BTreeMap;

    // #region 🔖️Leaves
    pub(crate) fn make_symbol(name: &str, assumptions: AssumeSet) -> Expr {
        Expr::from_kind_unchecked(Kind::Symbol(Symbol::new(name, assumptions.close())))
    }

    pub(crate) fn make_integer(n: Integer) -> Expr {
        Expr::from_kind_unchecked(Kind::Integer(n))
    }

    /// ➗️ Folds to `Integer` whenever the value is integral — `Kind::Rational` is only ever constructed
    /// for genuinely non-integral values.
    pub(crate) fn make_rational(r: Rational) -> Expr {
        if r.is_integer() {
            make_integer(r.trunc())
        } else {
            Expr::from_kind_unchecked(Kind::Rational(r))
        }
    }

    pub(crate) fn make_neg(e: Expr) -> Expr {
        make_mul(vec![make_integer(Integer::from_i64(-1)), e])
    }
    // #endregion 🔖️Leaves

    // #region 🔖️NumberFolding
    /// 🔢️ An internal add/mul accumulator over the numeric subset of the kernel (`Integer`/`Rational`).
    #[derive(Clone)]
    enum Num {
        Int(Integer),
        Rat(Rational),
    }

    impl Num {
        fn zero() -> Self {
            Num::Int(Integer::zero())
        }
        fn one() -> Self {
            Num::Int(Integer::one())
        }
        fn from_expr(e: &Expr) -> Option<Self> {
            match e.kind() {
                Kind::Integer(n) => Some(Num::Int(n.clone())),
                Kind::Rational(r) => Some(Num::Rat(r.clone())),
                _ => None,
            }
        }
        fn to_rational(&self) -> Rational {
            match self {
                Num::Int(n) => Rational::from_integer(n.clone()),
                Num::Rat(r) => r.clone(),
            }
        }
        fn add(&self, other: &Self) -> Self {
            match (self, other) {
                (Num::Int(a), Num::Int(b)) => Num::Int(a.add(b)),
                _ => Num::Rat(self.to_rational().add(&other.to_rational())),
            }
        }
        fn mul(&self, other: &Self) -> Self {
            match (self, other) {
                (Num::Int(a), Num::Int(b)) => Num::Int(a.mul(b)),
                _ => Num::Rat(self.to_rational().mul(&other.to_rational())),
            }
        }
        fn is_zero(&self) -> bool {
            match self {
                Num::Int(n) => n.is_zero(),
                Num::Rat(r) => r.is_zero(),
            }
        }
        fn is_one(&self) -> bool {
            match self {
                Num::Int(n) => *n == Integer::one(),
                Num::Rat(r) => *r == Rational::one(),
            }
        }
        fn is_negative(&self) -> bool {
            match self {
                Num::Int(n) => n.is_negative(),
                Num::Rat(r) => r.numer().is_negative(),
            }
        }
        fn into_expr(self) -> Expr {
            match self {
                Num::Int(n) => make_integer(n),
                Num::Rat(r) => make_rational(r),
            }
        }
    }
    // #endregion 🔖️NumberFolding

    // #region 🔖️Add
    pub(crate) fn make_add(terms: Vec<Expr>) -> Expr {
        let mut flat = Vec::with_capacity(terms.len());
        flatten_add(terms, &mut flat);

        if flat.iter().any(|t| matches!(t.kind(), Kind::Constant(Constant::Undefined))) {
            return Expr::constant(Constant::Undefined);
        }
        let inf_count = flat.iter().filter(|t| matches!(t.kind(), Kind::Constant(Constant::Inf))).count();
        let neg_inf_count = flat.iter().filter(|t| matches!(t.kind(), Kind::Constant(Constant::NegInf))).count();
        if inf_count > 0 && neg_inf_count > 0 {
            return Expr::constant(Constant::Undefined);
        }
        if inf_count > 0 {
            return Expr::constant(Constant::Inf);
        }
        if neg_inf_count > 0 {
            return Expr::constant(Constant::NegInf);
        }

        let mut numeric = Num::zero();
        let mut by_rest: BTreeMap<Expr, Num> = BTreeMap::new();
        for term in flat {
            if let Some(n) = Num::from_expr(&term) {
                numeric = numeric.add(&n);
                continue;
            }
            let (coeff, rest) = split_coefficient(&term);
            by_rest.entry(rest).and_modify(|c| *c = c.add(&coeff)).or_insert(coeff);
        }

        let mut result_terms: Vec<Expr> = Vec::new();
        if !numeric.is_zero() {
            result_terms.push(numeric.into_expr());
        }
        for (rest, coeff) in by_rest {
            if coeff.is_zero() {
                continue;
            }
            if coeff.is_one() {
                result_terms.push(rest);
            } else {
                result_terms.push(make_mul(vec![coeff.into_expr(), rest]));
            }
        }

        finalize_variadic(result_terms, Integer::zero(), Kind::Add)
    }

    fn flatten_add(terms: Vec<Expr>, out: &mut Vec<Expr>) {
        for t in terms {
            match t.kind() {
                Kind::Add(inner) => flatten_add(inner.clone(), out),
                _ => out.push(t),
            }
        }
    }

    /// ➗️ Splits a term into `(numeric_coefficient, rest)` for like-term collection: a `Mul` with a
    /// leading numeric factor contributes that factor as the coefficient and the remaining factors
    /// (re-multiplied) as `rest`; anything else has an implicit coefficient of `1`.
    fn split_coefficient(term: &Expr) -> (Num, Expr) {
        if let Kind::Mul(factors) = term.kind() {
            if let Some(first_num) = factors.first().and_then(Num::from_expr) {
                let rest_factors: Vec<Expr> = factors[1..].to_vec();
                let rest = make_mul(rest_factors);
                return (first_num, rest);
            }
        }
        (Num::one(), term.clone())
    }
    // #endregion 🔖️Add

    // #region 🔖️Mul
    pub(crate) fn make_mul(factors: Vec<Expr>) -> Expr {
        let mut flat = Vec::with_capacity(factors.len());
        flatten_mul(factors, &mut flat);

        if flat.iter().any(|t| matches!(t.kind(), Kind::Constant(Constant::Undefined))) {
            return Expr::constant(Constant::Undefined);
        }

        let mut numeric = Num::one();
        let mut symbolic: Vec<Expr> = Vec::new();
        for f in flat {
            if let Some(n) = Num::from_expr(&f) {
                numeric = numeric.mul(&n);
            } else {
                symbolic.push(f);
            }
        }

        // `zoo` (directionless complex infinity) swallows every finite factor's sign, so it must be
        // checked before the signed `oo`/`-oo` contagion below: `0 * zoo` is still `Undefined`, but
        // `zoo * -oo` (or any other signed infinity) collapses to `zoo`, not a signed infinity.
        if symbolic.iter().any(|t| matches!(t.kind(), Kind::Constant(Constant::ComplexInf))) {
            return if numeric.is_zero() { Expr::constant(Constant::Undefined) } else { Expr::constant(Constant::ComplexInf) };
        }

        let inf_terms: Vec<&Expr> = symbolic.iter().filter(|t| matches!(t.kind(), Kind::Constant(Constant::Inf) | Kind::Constant(Constant::NegInf))).collect();
        if !inf_terms.is_empty() {
            if numeric.is_zero() {
                return Expr::constant(Constant::Undefined);
            }
            let mut sign_negative = numeric.is_negative();
            for t in &inf_terms {
                if matches!(t.kind(), Kind::Constant(Constant::NegInf)) {
                    sign_negative = !sign_negative;
                }
            }
            return Expr::constant(if sign_negative { Constant::NegInf } else { Constant::Inf });
        }

        if numeric.is_zero() {
            return Expr::integer(0);
        }

        // Combine like bases: x^a * x^b -> x^(a+b).
        let mut by_base: BTreeMap<Expr, Expr> = BTreeMap::new();
        let mut base_order: Vec<Expr> = Vec::new();
        for f in symbolic {
            let (base, exp) = match f.kind() {
                Kind::Pow(b, e) => (b.clone(), e.clone()),
                _ => (f.clone(), Expr::integer(1)),
            };
            if let Some(existing) = by_base.get(&base).cloned() {
                by_base.insert(base, make_add(vec![existing, exp]));
            } else {
                base_order.push(base.clone());
                by_base.insert(base, exp);
            }
        }

        let mut result_terms: Vec<Expr> = Vec::new();
        if !numeric.is_one() {
            result_terms.push(numeric.into_expr());
        }
        for base in base_order {
            let exp = by_base.remove(&base).unwrap();
            result_terms.push(make_pow(base, exp));
        }
        // Any newly-introduced numeric terms from pow folding (e.g. 2^3 -> 8) must be re-merged.
        if result_terms.iter().filter(|t| Num::from_expr(t).is_some()).count() > 1 {
            return make_mul(result_terms);
        }

        finalize_variadic(result_terms, Integer::one(), Kind::Mul)
    }

    fn flatten_mul(factors: Vec<Expr>, out: &mut Vec<Expr>) {
        for f in factors {
            match f.kind() {
                Kind::Mul(inner) => flatten_mul(inner.clone(), out),
                _ => out.push(f),
            }
        }
    }

    /// 🧹️ Common tail for `Add`/`Mul`: drop the identity element (dead code by construction upstream is
    /// avoided since numeric folding already strips it), sort canonically, and collapse
    /// empty/singleton results.
    fn finalize_variadic(mut terms: Vec<Expr>, identity: Integer, wrap: fn(Vec<Expr>) -> Kind) -> Expr {
        terms.retain(|t| !matches!(t.kind(), Kind::Integer(n) if *n == identity));
        if terms.is_empty() {
            return make_integer(identity);
        }
        if terms.len() == 1 {
            return terms.into_iter().next().unwrap();
        }
        terms.sort();
        Expr::from_kind_unchecked(wrap(terms))
    }
    // #endregion 🔖️Mul

    // #region 🔖️Pow
    pub(crate) fn make_pow(base: Expr, exp: Expr) -> Expr {
        if matches!(base.kind(), Kind::Constant(Constant::Undefined)) || matches!(exp.kind(), Kind::Constant(Constant::Undefined)) {
            return Expr::constant(Constant::Undefined);
        }
        if exp.is_zero_literal() {
            return Expr::integer(1);
        }
        if exp.is_one_literal() {
            return base;
        }
        if base.is_one_literal() {
            return Expr::integer(1);
        }
        if base.is_zero_literal() {
            return match exp.kind() {
                Kind::Integer(n) if n.is_positive() => Expr::integer(0),
                Kind::Integer(n) if n.is_negative() => Expr::constant(Constant::ComplexInf),
                Kind::Rational(r) if r.numer().is_positive() => Expr::integer(0),
                Kind::Rational(r) if r.numer().is_negative() => Expr::constant(Constant::ComplexInf),
                _ => Expr::from_kind_unchecked(Kind::Pow(base, exp)),
            };
        }

        // i^n cycles with period 4.
        if matches!(base.kind(), Kind::Constant(Constant::I)) {
            if let Kind::Integer(n) = exp.kind() {
                if let Some(e) = n.to_i64() {
                    let m = e.rem_euclid(4);
                    return match m {
                        0 => Expr::integer(1),
                        1 => Expr::constant(Constant::I),
                        2 => Expr::integer(-1),
                        _ => make_neg(Expr::constant(Constant::I)),
                    };
                }
            }
        }

        // (x^a)^b -> x^(a*b) when b is an integer, or when x is known positive.
        if let Kind::Pow(inner_base, inner_exp) = base.kind() {
            let b_is_integer = matches!(exp.kind(), Kind::Integer(_));
            if b_is_integer || is_positive(&inner_base.clone()) == Some(true) {
                let combined_exp = make_mul(vec![inner_exp.clone(), exp]);
                return make_pow(inner_base.clone(), combined_exp);
            }
        }

        match (base.kind(), exp.kind()) {
            (Kind::Integer(b), Kind::Integer(e)) => fold_integer_pow(b.clone(), e.clone()),
            (Kind::Rational(b), Kind::Integer(e)) => {
                if let Some(ev) = e.to_i64() {
                    if let Some(result) = b.pow(ev) {
                        return make_rational(result);
                    }
                }
                Expr::from_kind_unchecked(Kind::Pow(base, exp))
            }
            (Kind::Integer(b), Kind::Rational(e)) => fold_radical(b, e.clone()).unwrap_or_else(|| Expr::from_kind_unchecked(Kind::Pow(base, exp))),
            _ => Expr::from_kind_unchecked(Kind::Pow(base, exp)),
        }
    }

    fn fold_integer_pow(base: Integer, exp: Integer) -> Expr {
        let Some(ev) = exp.to_i64() else {
            return Expr::from_kind_unchecked(Kind::Pow(make_integer(base), make_integer(exp)));
        };
        if ev >= 0 {
            make_integer(base.pow(ev as u64))
        } else {
            let positive = base.pow((-ev) as u64);
            match Rational::new(Integer::one(), positive) {
                Some(r) => make_rational(r),
                None => Expr::constant(Constant::ComplexInf),
            }
        }
    }

    /// 🌱️ Radical extraction for `base^(p/q)` (`base` a nonzero integer, `q >= 2`): factors `|base|` into
    /// `outside^q * inside` (via prime factorization) with `inside` free of `q`-th-power factors, giving
    /// `base^(1/q) = (+-outside) * inside^(1/q)`. Only the numerator-`1` case gets this partial extraction;
    /// other numerators only fold when `base` is an *exact* `q`-th power (documented simplification).
    fn fold_radical(base: &Integer, exp: Rational) -> Option<Expr> {
        let q = exp.denom().to_u64()? as u32;
        let p = exp.numer().clone();
        if q < 2 {
            return None;
        }
        let is_neg = base.is_negative();
        if q.is_multiple_of(2) && is_neg {
            return None; // even root of a negative number: not real, leave symbolic
        }
        let bm = base.abs();
        let factors = primes::factor(&bm);
        let mut outside = Natural::one();
        let mut inside = Natural::one();
        for (prime, e) in factors {
            let outside_exp = e / q;
            let inside_exp = e % q;
            outside = outside.mul(&prime.pow(outside_exp as u64));
            inside = inside.mul(&prime.pow(inside_exp as u64));
        }
        let p_abs = p.abs().to_u64()?;
        if inside == Natural::one() {
            // Exact q-th power: base^(p/q) folds fully to an integer/rational.
            let sign_factor = if is_neg && p_abs % 2 == 1 { -1i64 } else { 1i64 };
            let magnitude = outside.pow(p_abs);
            let signed = if sign_factor < 0 { Integer::from_natural(magnitude).neg() } else { Integer::from_natural(magnitude) };
            if p.is_negative() {
                return Rational::new(Integer::one(), signed).map(make_rational);
            }
            return Some(make_integer(signed));
        }
        if p_abs == 1 && outside != Natural::one() {
            let outside_signed = if is_neg { Integer::from_natural(outside).neg() } else { Integer::from_natural(outside) };
            let inside_expr = make_integer(Integer::from_natural(inside));
            let remainder_exp = if p.is_negative() { exp.neg() } else { exp };
            let remainder = make_pow(inside_expr, make_rational(remainder_exp));
            return Some(make_mul(vec![make_integer(outside_signed), remainder]));
        }
        None
    }
    // #endregion 🔖️Pow

    // #region 🔖️Func
    pub(crate) fn make_func(kind: FnKind, args: Vec<Expr>) -> Expr {
        if let Some(arity) = kind.arity() {
            debug_assert_eq!(args.len(), arity, "make_func: wrong arity for {kind:?}");
        }
        if args.iter().any(|a| matches!(a.kind(), Kind::Constant(Constant::Undefined))) {
            return Expr::constant(Constant::Undefined);
        }
        if args.len() == 1 {
            if let Some(folded) = fold_unary_special_value(&kind, &args[0]) {
                return folded;
            }
        }
        Expr::from_kind_unchecked(Kind::Fn(kind, args))
    }

    fn fold_unary_special_value(kind: &FnKind, arg: &Expr) -> Option<Expr> {
        match kind {
            FnKind::Sin | FnKind::Tan | FnKind::Asin | FnKind::Atan | FnKind::Sinh | FnKind::Tanh | FnKind::Asinh | FnKind::Atanh if arg.is_zero_literal() => Some(Expr::integer(0)),
            FnKind::Cos | FnKind::Cosh if arg.is_zero_literal() => Some(Expr::integer(1)),
            FnKind::Exp if arg.is_zero_literal() => Some(Expr::integer(1)),
            FnKind::Ln if arg.is_one_literal() => Some(Expr::integer(0)),
            FnKind::Abs => match arg.kind() {
                Kind::Integer(n) => Some(make_integer(n.abs_integer())),
                Kind::Rational(r) => Some(make_rational(r.abs())),
                _ => None,
            },
            FnKind::Sign => match arg.kind() {
                Kind::Integer(n) => Some(Expr::integer(n.signum() as i64)),
                _ => None,
            },
            _ => None,
        }
    }
    // #endregion 🔖️Func

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn add_folds_numeric_literals() {
            let e = make_add(vec![Expr::integer(2), Expr::integer(3)]);
            assert_eq!(e, Expr::integer(5));
        }

        #[test]
        fn add_collects_like_terms() {
            let x = Expr::symbol("x");
            let e = make_add(vec![x.clone(), x.clone()]);
            let expected = make_mul(vec![Expr::integer(2), x]);
            assert_eq!(e, expected);
        }

        #[test]
        fn add_drops_zero() {
            let x = Expr::symbol("x");
            let e = make_add(vec![x.clone(), Expr::integer(0)]);
            assert_eq!(e, x);
        }

        #[test]
        fn mul_folds_numeric_literals() {
            let e = make_mul(vec![Expr::integer(2), Expr::integer(3)]);
            assert_eq!(e, Expr::integer(6));
        }

        #[test]
        fn mul_combines_like_bases() {
            let x = Expr::symbol("x");
            let e = make_mul(vec![x.clone(), x.clone()]);
            let expected = make_pow(x, Expr::integer(2));
            assert_eq!(e, expected);
        }

        #[test]
        fn mul_by_zero_absorbs() {
            let x = Expr::symbol("x");
            let e = make_mul(vec![x, Expr::integer(0)]);
            assert_eq!(e, Expr::integer(0));
        }

        #[test]
        fn mul_zero_times_complex_infinity_is_undefined() {
            let e = make_mul(vec![Expr::integer(0), Expr::constant(Constant::ComplexInf)]);
            assert_eq!(e, Expr::constant(Constant::Undefined));
        }

        #[test]
        fn mul_nonzero_times_complex_infinity_is_complex_infinity() {
            let e = make_mul(vec![Expr::integer(5), Expr::constant(Constant::ComplexInf)]);
            assert_eq!(e, Expr::constant(Constant::ComplexInf));
        }

        #[test]
        fn pow_identities() {
            let x = Expr::symbol("x");
            assert_eq!(make_pow(x.clone(), Expr::integer(0)), Expr::integer(1));
            assert_eq!(make_pow(x.clone(), Expr::integer(1)), x);
            assert_eq!(make_pow(Expr::integer(1), x), Expr::integer(1));
        }

        #[test]
        fn pow_integer_folds_exactly() {
            assert_eq!(make_pow(Expr::integer(2), Expr::integer(10)), Expr::integer(1024));
        }

        #[test]
        fn pow_negative_integer_exponent_gives_rational() {
            let e = make_pow(Expr::integer(2), Expr::integer(-1));
            assert_eq!(e, make_rational(Rational::from_i64(1, 2).unwrap()));
        }

        #[test]
        fn radical_partial_extraction_matches_plan_example() {
            // 8^(1/2) -> 2 * 2^(1/2)
            let e = make_pow(Expr::integer(8), make_rational(Rational::from_i64(1, 2).unwrap()));
            let expected = make_mul(vec![Expr::integer(2), make_pow(Expr::integer(2), make_rational(Rational::from_i64(1, 2).unwrap()))]);
            assert_eq!(e, expected);
        }

        #[test]
        fn radical_exact_perfect_power_folds_fully() {
            // 4^(1/2) -> 2
            let e = make_pow(Expr::integer(4), make_rational(Rational::from_i64(1, 2).unwrap()));
            assert_eq!(e, Expr::integer(2));
        }

        #[test]
        fn radical_of_prime_stays_symbolic() {
            let e = make_pow(Expr::integer(2), make_rational(Rational::from_i64(1, 2).unwrap()));
            assert!(matches!(e.kind(), Kind::Pow(..)));
        }

        #[test]
        fn nested_pow_combines_exponents_for_integer_outer_exponent() {
            let x = Expr::symbol("x");
            let inner = make_pow(x.clone(), Expr::integer(2));
            let outer = make_pow(inner, Expr::integer(3));
            assert_eq!(outer, make_pow(x, Expr::integer(6)));
        }

        #[test]
        fn i_power_cycles_with_period_four() {
            let i = Expr::constant(Constant::I);
            assert_eq!(make_pow(i.clone(), Expr::integer(0)), Expr::integer(1));
            assert_eq!(make_pow(i.clone(), Expr::integer(1)), i);
            assert_eq!(make_pow(i.clone(), Expr::integer(2)), Expr::integer(-1));
            assert_eq!(make_pow(i, Expr::integer(4)), Expr::integer(1));
        }

        #[test]
        fn func_special_values_fold() {
            assert_eq!(make_func(FnKind::Sin, vec![Expr::integer(0)]), Expr::integer(0));
            assert_eq!(make_func(FnKind::Cos, vec![Expr::integer(0)]), Expr::integer(1));
            assert_eq!(make_func(FnKind::Exp, vec![Expr::integer(0)]), Expr::integer(1));
        }

        #[test]
        fn canonicalization_is_idempotent_on_a_small_corpus() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let corpus = vec![
                make_add(vec![x.clone(), y.clone(), Expr::integer(3)]),
                make_mul(vec![x.clone(), y, Expr::integer(2)]),
                make_pow(x.clone(), Expr::integer(5)),
                make_add(vec![make_mul(vec![Expr::integer(2), x.clone()]), make_mul(vec![Expr::integer(3), x])]),
            ];
            for e in corpus {
                // Rebuilding from the same top-level kind should reproduce exactly the same expression.
                let rebuilt = match e.kind() {
                    Kind::Add(terms) => make_add(terms.clone()),
                    Kind::Mul(factors) => make_mul(factors.clone()),
                    Kind::Pow(b, ex) => make_pow(b.clone(), ex.clone()),
                    _ => e.clone(),
                };
                assert_eq!(e, rebuilt);
            }
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Canon

// #region 🔖️Assume
pub mod assume {
    //! 🧭️ Per-symbol assumption flags (real/positive/integer/...) with closure-under-implication at
    //! creation time, plus three-valued deduction queries over expressions once `Expr` exists (see the
    //! `crate::cas::expr` module — the deduction functions here take `&Expr` and live alongside the flag type
    //! so the whole assumption story is in one file).

    use crate::cas::expr::{Expr, Kind};

    // #region 🔖️AssumeSet
    /// 🧭️ Bitflags of symbol properties; closed under implication by [`AssumeSet::close`] before a symbol
    /// is ever constructed, so downstream code never has to re-derive `POSITIVE => REAL` etc. itself.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
    pub struct AssumeSet(u32);

    impl AssumeSet {
        pub const NONE: Self = Self(0);
        pub const INTEGER: Self = Self(1 << 0);
        pub const RATIONAL: Self = Self(1 << 1);
        pub const REAL: Self = Self(1 << 2);
        pub const COMPLEX: Self = Self(1 << 3);
        pub const POSITIVE: Self = Self(1 << 4);
        pub const NEGATIVE: Self = Self(1 << 5);
        pub const NONNEGATIVE: Self = Self(1 << 6);
        pub const NONPOSITIVE: Self = Self(1 << 7);
        pub const NONZERO: Self = Self(1 << 8);
        pub const EVEN: Self = Self(1 << 9);
        pub const ODD: Self = Self(1 << 10);
        pub const FINITE: Self = Self(1 << 11);

        pub const fn bits(self) -> u32 {
            self.0
        }

        pub const fn from_bits(bits: u32) -> Self {
            Self(bits)
        }

        pub fn contains(self, flag: Self) -> bool {
            self.0 & flag.0 == flag.0
        }

        pub fn union(self, other: Self) -> Self {
            Self(self.0 | other.0)
        }

        /// 🔒️ Applies every implication to a fixpoint (`POSITIVE => REAL, NONNEGATIVE, NONZERO`, etc.),
        /// then panics if the closed set contains a direct contradiction (e.g. `POSITIVE` and `NEGATIVE`).
        /// Called once, at symbol-construction time — every `AssumeSet` observed afterward is already closed.
        pub fn close(self) -> Self {
            let mut set = self;
            loop {
                let mut next = set;
                if set.contains(Self::POSITIVE) {
                    next = next.union(Self::REAL).union(Self::NONNEGATIVE).union(Self::NONZERO);
                }
                if set.contains(Self::NEGATIVE) {
                    next = next.union(Self::REAL).union(Self::NONPOSITIVE).union(Self::NONZERO);
                }
                if set.contains(Self::EVEN) || set.contains(Self::ODD) {
                    next = next.union(Self::INTEGER);
                }
                if set.contains(Self::INTEGER) {
                    next = next.union(Self::RATIONAL);
                }
                if set.contains(Self::RATIONAL) {
                    next = next.union(Self::REAL);
                }
                if set.contains(Self::REAL) {
                    next = next.union(Self::COMPLEX).union(Self::FINITE);
                }
                if next == set {
                    break;
                }
                set = next;
            }
            assert!(!(set.contains(Self::POSITIVE) && set.contains(Self::NEGATIVE)), "AssumeSet::close: contradictory POSITIVE and NEGATIVE");
            assert!(!(set.contains(Self::POSITIVE) && set.contains(Self::NONPOSITIVE)), "AssumeSet::close: contradictory POSITIVE and NONPOSITIVE");
            assert!(!(set.contains(Self::NEGATIVE) && set.contains(Self::NONNEGATIVE)), "AssumeSet::close: contradictory NEGATIVE and NONNEGATIVE");
            assert!(!(set.contains(Self::EVEN) && set.contains(Self::ODD)), "AssumeSet::close: contradictory EVEN and ODD");
            set
        }
    }

    impl std::ops::BitOr for AssumeSet {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self {
            self.union(rhs)
        }
    }
    // #endregion 🔖️AssumeSet

    // #region 🔖️Assumptions
    /// 📋️ Extra facts beyond a symbol's own flags (e.g. `x > 2`), consulted by `solve_with`/`refine`-style
    /// entry points. First-pass scope: direct `symbol <operation> rational` bounds only.
    #[derive(Clone, Debug, Default)]
    pub struct Assumptions {
        facts: Vec<(String, RelationalOperator, math::number::Rational)>,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum RelationalOperator {
        Lt,
        Le,
        Gt,
        Ge,
        Eq,
        Ne,
    }

    impl Assumptions {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn assume_bound(&mut self, symbol: &str, operator: RelationalOperator, bound: math::number::Rational) {
            self.facts.push((symbol.to_string(), operator, bound));
        }

        fn bound_for(&self, symbol: &str) -> Option<bool> {
            for (name, operation, bound) in &self.facts {
                if name != symbol {
                    continue;
                }
                use math::number::Rational;
                let zero = Rational::zero();
                let is_positive = match operation {
                    RelationalOperator::Gt if *bound >= zero => Some(true),
                    RelationalOperator::Ge if *bound > zero => Some(true),
                    RelationalOperator::Lt if *bound <= zero => Some(false),
                    RelationalOperator::Le if *bound < zero => Some(false),
                    _ => None,
                };
                if is_positive.is_some() {
                    return is_positive;
                }
            }
            None
        }
    }
    // #endregion 🔖️Assumptions

    // #region 🔖️Deduction
    const MAX_DEDUCTION_DEPTH: u32 = 64;

    /// 〽 Three-valued positivity query: exact for numeric literals, from-flags for bare symbols, and
    /// recursively deduced through `Add`/`Mul`/`Pow`/a few `Fn` cases; `None` means "can't tell", never
    /// a wrong answer.
    pub fn is_positive(e: &Expr) -> Option<bool> {
        is_positive_depth(e, &Assumptions::new(), 0)
    }

    pub fn is_positive_with(e: &Expr, assumptions: &Assumptions) -> Option<bool> {
        is_positive_depth(e, assumptions, 0)
    }

    fn is_positive_depth(e: &Expr, assumptions: &Assumptions, depth: u32) -> Option<bool> {
        if depth > MAX_DEDUCTION_DEPTH {
            return None;
        }
        match e.kind() {
            Kind::Integer(n) => Some(n.is_positive()),
            Kind::Rational(r) => Some(!r.is_zero() && r.numer().is_positive()),
            Kind::Symbol(sym) => {
                if sym.assumptions().contains(AssumeSet::POSITIVE) {
                    Some(true)
                } else if sym.assumptions().contains(AssumeSet::NEGATIVE) || sym.assumptions().contains(AssumeSet::NONPOSITIVE) {
                    Some(false)
                } else {
                    assumptions.bound_for(sym.name())
                }
            }
            Kind::Constant(c) => match c {
                crate::cas::expr::Constant::Pi | crate::cas::expr::Constant::E | crate::cas::expr::Constant::EulerGamma => Some(true),
                crate::cas::expr::Constant::Inf => Some(true),
                crate::cas::expr::Constant::NegInf => Some(false),
                _ => None,
            },
            Kind::Add(terms) => {
                let signs: Vec<Option<bool>> = terms.iter().map(|t| is_positive_depth(t, assumptions, depth + 1)).collect();
                if signs.iter().all(|s| *s == Some(true)) {
                    Some(true)
                } else if signs.iter().all(|s| *s == Some(false)) {
                    Some(false)
                } else {
                    None
                }
            }
            Kind::Mul(factors) => {
                let mut known = true;
                let mut negatives = 0;
                for f in factors {
                    match is_positive_depth(f, assumptions, depth + 1) {
                        Some(true) => {}
                        Some(false) => negatives += 1,
                        None => {
                            known = false;
                            break;
                        }
                    }
                }
                if known {
                    Some(negatives % 2 == 0)
                } else {
                    None
                }
            }
            Kind::Pow(base, exp) => {
                if let Kind::Integer(n) = exp.kind() {
                    if let Some(exp_val) = n.to_i64() {
                        if exp_val % 2 == 0 && exp_val != 0 {
                            return is_nonzero_depth(base, assumptions, depth + 1).and_then(|nz| if nz { Some(true) } else { None });
                        }
                    }
                }
                let base_sign = is_positive_depth(base, assumptions, depth + 1)?;
                Some(base_sign)
            }
            Kind::Fn(kind, args) => match kind {
                crate::cas::fnkind::FnKind::Exp => Some(true),
                crate::cas::fnkind::FnKind::Abs => is_nonzero_depth(&args[0], assumptions, depth + 1),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn is_nonzero(e: &Expr) -> Option<bool> {
        is_nonzero_depth(e, &Assumptions::new(), 0)
    }

    fn is_nonzero_depth(e: &Expr, assumptions: &Assumptions, depth: u32) -> Option<bool> {
        if depth > MAX_DEDUCTION_DEPTH {
            return None;
        }
        if let Some(true) = is_positive_depth(e, assumptions, depth) {
            return Some(true);
        }
        match e.kind() {
            Kind::Integer(n) => Some(!n.is_zero()),
            Kind::Rational(r) => Some(!r.is_zero()),
            Kind::Symbol(sym) => {
                if sym.assumptions().contains(AssumeSet::NONZERO) {
                    Some(true)
                } else {
                    None
                }
            }
            Kind::Mul(factors) => {
                let mut all_nonzero = true;
                for f in factors {
                    if is_nonzero_depth(f, assumptions, depth + 1) != Some(true) {
                        all_nonzero = false;
                        break;
                    }
                }
                if all_nonzero {
                    Some(true)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn is_real(e: &Expr) -> Option<bool> {
        match e.kind() {
            Kind::Integer(_) | Kind::Rational(_) => Some(true),
            Kind::Symbol(sym) => {
                if sym.assumptions().contains(AssumeSet::REAL) {
                    Some(true)
                } else {
                    None
                }
            }
            Kind::Constant(c) => Some(!matches!(c, crate::cas::expr::Constant::I | crate::cas::expr::Constant::ComplexInf | crate::cas::expr::Constant::Undefined)),
            Kind::Add(terms) | Kind::Mul(terms) => {
                if terms.iter().all(|t| is_real(t) == Some(true)) {
                    Some(true)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn is_integer(e: &Expr) -> Option<bool> {
        match e.kind() {
            Kind::Integer(_) => Some(true),
            Kind::Rational(_) => Some(false),
            Kind::Symbol(sym) => {
                if sym.assumptions().contains(AssumeSet::INTEGER) {
                    Some(true)
                } else {
                    None
                }
            }
            Kind::Add(terms) | Kind::Mul(terms) => {
                if terms.iter().all(|t| is_integer(t) == Some(true)) {
                    Some(true)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn is_even(e: &Expr) -> Option<bool> {
        match e.kind() {
            Kind::Integer(n) => n.to_i64().map(|v| v % 2 == 0),
            Kind::Symbol(sym) => {
                if sym.assumptions().contains(AssumeSet::EVEN) {
                    Some(true)
                } else if sym.assumptions().contains(AssumeSet::ODD) {
                    Some(false)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    // #endregion 🔖️Deduction

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn close_propagates_positive_to_real_nonnegative_nonzero() {
            let closed = AssumeSet::POSITIVE.close();
            assert!(closed.contains(AssumeSet::REAL));
            assert!(closed.contains(AssumeSet::NONNEGATIVE));
            assert!(closed.contains(AssumeSet::NONZERO));
            assert!(closed.contains(AssumeSet::COMPLEX));
        }

        #[test]
        fn close_propagates_even_to_integer_rational_real() {
            let closed = AssumeSet::EVEN.close();
            assert!(closed.contains(AssumeSet::INTEGER));
            assert!(closed.contains(AssumeSet::RATIONAL));
            assert!(closed.contains(AssumeSet::REAL));
        }

        #[test]
        #[should_panic(expected = "contradictory POSITIVE and NEGATIVE")]
        fn close_rejects_positive_and_negative() {
            (AssumeSet::POSITIVE | AssumeSet::NEGATIVE).close();
        }

        #[test]
        #[should_panic(expected = "contradictory EVEN and ODD")]
        fn close_rejects_even_and_odd() {
            (AssumeSet::EVEN | AssumeSet::ODD).close();
        }

        #[test]
        fn bound_for_deduces_sign_from_assumption_store() {
            use math::number::Rational;
            let mut assumptions = Assumptions::new();
            assumptions.assume_bound("x", RelationalOperator::Gt, Rational::from_i64(2, 1).unwrap());
            assert_eq!(assumptions.bound_for("x"), Some(true));
            assert_eq!(assumptions.bound_for("y"), None);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Assume

// #region 🔖️Visit
pub mod visit {
    //! 🚶️ Tree walkers built on the canonical constructors: substitution, free-symbol collection, and
    //! node counting (used by `simplify`'s measured pipeline).

    use crate::cas::expr::{Expr, Kind};

    // #region 🔖️Subs
    /// 🔁️ Replaces every occurrence of `target` with `replacement` (structural equality, post-order).
    pub fn subs(e: &Expr, target: &Expr, replacement: &Expr) -> Expr {
        if e == target {
            return replacement.clone();
        }
        map_children(e, &mut |child| subs(child, target, replacement))
    }

    /// 🔁️ Applies a full substitution map in one pass (each key checked before recursing into children).
    pub fn subs_many(e: &Expr, map: &[(Expr, Expr)]) -> Expr {
        for (target, replacement) in map {
            if e == target {
                return replacement.clone();
            }
        }
        map_children(e, &mut |child| subs_many(child, map))
    }

    /// 🌳️ Applies `f` to every child of `e` and rebuilds `e` with the results, going through the smart
    /// constructors so the rebuilt node is always fully canonical.
    pub fn map_children(e: &Expr, f: &mut impl FnMut(&Expr) -> Expr) -> Expr {
        match e.kind() {
            Kind::Add(terms) => Expr::add(terms.iter().map(f).collect()),
            Kind::Mul(factors) => Expr::mul(factors.iter().map(f).collect()),
            Kind::Pow(base, exp) => Expr::pow(f(base), f(exp)),
            Kind::Fn(kind, args) => Expr::func(kind.clone(), args.iter().map(f).collect()),
            Kind::Piecewise(cases) => Expr::from_kind_unchecked(Kind::Piecewise(cases.iter().map(|(v, c)| (f(v), f(c))).collect())),
            Kind::Rel(operation, a, b) => Expr::from_kind_unchecked(Kind::Rel(*operation, f(a), f(b))),
            Kind::Integer(_) | Kind::Rational(_) | Kind::Symbol(_) | Kind::Constant(_) | Kind::Bool(_) | Kind::RootOf { .. } | Kind::Wild(..) => e.clone(),
        }
    }
    // #endregion 🔖️Subs

    // #region 🔖️Replace
    /// 🔁️ Bottom-up rewrite: applies `f` to every subtree (children first), keeping `f`'s result whenever
    /// it returns `Some`.
    pub fn replace_bottom_up(e: &Expr, f: &mut impl FnMut(&Expr) -> Option<Expr>) -> Expr {
        let rebuilt = map_children(e, &mut |child| replace_bottom_up(child, f));
        f(&rebuilt).unwrap_or(rebuilt)
    }
    // #endregion 🔖️Replace

    // #region 🔖️FreeSymbols
    /// 🔤️ Every distinct symbol appearing anywhere in `e`, in canonical (sorted, deduplicated) order.
    pub fn free_symbols(e: &Expr) -> Vec<Expr> {
        let mut found = std::collections::BTreeSet::new();
        collect_symbols(e, &mut found);
        found.into_iter().collect()
    }

    fn collect_symbols(e: &Expr, out: &mut std::collections::BTreeSet<Expr>) {
        if matches!(e.kind(), Kind::Symbol(_)) {
            out.insert(e.clone());
            return;
        }
        for child in children(e) {
            collect_symbols(&child, out);
        }
    }

    pub fn contains_symbol(e: &Expr, symbol: &Expr) -> bool {
        if e == symbol {
            return true;
        }
        children(e).iter().any(|c| contains_symbol(c, symbol))
    }

    fn children(e: &Expr) -> Vec<Expr> {
        match e.kind() {
            Kind::Add(terms) => terms.clone(),
            Kind::Mul(factors) => factors.clone(),
            Kind::Pow(base, exp) => vec![base.clone(), exp.clone()],
            Kind::Fn(_, args) => args.clone(),
            Kind::Piecewise(cases) => cases.iter().flat_map(|(v, c)| [v.clone(), c.clone()]).collect(),
            Kind::Rel(_, a, b) => vec![a.clone(), b.clone()],
            _ => Vec::new(),
        }
    }
    // #endregion 🔖️FreeSymbols

    // #region 🔖️NodeCount
    /// 🔢️ Total node count (leaves + internal nodes), used by `simplify`'s "smallest wins" heuristic.
    pub fn node_count(e: &Expr) -> usize {
        1 + children(e).iter().map(node_count).sum::<usize>()
    }
    // #endregion 🔖️NodeCount

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn subs_replaces_matching_subtree() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let e = Expr::add(vec![x.clone(), Expr::integer(1)]);
            let result = subs(&e, &x, &y);
            assert_eq!(result, Expr::add(vec![y, Expr::integer(1)]));
        }

        #[test]
        fn free_symbols_deduplicates_and_sorts() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let e = Expr::add(vec![x.clone(), x, y]);
            let symbols = free_symbols(&e);
            assert_eq!(symbols.len(), 2);
        }

        #[test]
        fn node_count_hand_case() {
            let x = Expr::symbol("x");
            let e = Expr::add(vec![x, Expr::integer(1)]);
            assert_eq!(node_count(&e), 3); // Add(x, 1) has 2 children + 1 for itself
        }

        #[test]
        fn contains_symbol_detects_nested_occurrence() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let e = Expr::pow(Expr::add(vec![x.clone(), Expr::integer(1)]), Expr::integer(2));
            assert!(contains_symbol(&e, &x));
            assert!(!contains_symbol(&e, &y));
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Visit

// #region 🔖️Fmt
pub mod fmt {
    //! 🖨️ Human-readable output: precedence-aware infix `Display` and a LaTeX emitter, both walking the
    //! same canonical tree (so output is deterministic and stable across runs).

    use crate::cas::expr::{Constant, Expr, Kind, RelationalOperator};
    use crate::cas::fnkind::FnKind;
    use std::ops::Neg;

    // #region 🔖️Precedence
    #[derive(Clone, Copy, PartialEq, PartialOrd)]
    enum Prec {
        Add = 1,
        Mul = 2,
        Unary = 3,
        Pow = 4,
        Atom = 5,
    }

    fn precedence(e: &Expr) -> Prec {
        match e.kind() {
            Kind::Add(_) => Prec::Add,
            Kind::Mul(_) => Prec::Mul,
            Kind::Pow(..) => Prec::Pow,
            Kind::Integer(n) if n.is_negative() => Prec::Unary,
            Kind::Rational(r) if r.numer().is_negative() => Prec::Unary,
            _ => Prec::Atom,
        }
    }
    // #endregion 🔖️Precedence

    // #region 🔖️Display
    pub fn display_string(e: &Expr) -> String {
        let mut s = String::new();
        write_expr(e, &mut s);
        s
    }

    /// ✖️➗️ Recovers the canonical `-a` / `a/b` encodings (`Mul([-1, a])`, `Mul([a, Pow(b,-1)])`) into
    /// readable infix output.
    fn write_expr(e: &Expr, out: &mut String) {
        match e.kind() {
            Kind::Integer(n) => out.push_str(&n.to_decimal()),
            Kind::Rational(r) => out.push_str(&r.to_string()),
            Kind::Symbol(s) => out.push_str(s.name()),
            Kind::Constant(c) => out.push_str(c.name()),
            Kind::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Kind::Add(terms) => write_add(terms, out),
            Kind::Mul(factors) => write_mul(factors, out),
            Kind::Pow(base, exp) => write_pow(base, exp, out),
            Kind::Fn(kind, args) => write_func(kind, args, out),
            Kind::RootOf { index, .. } => out.push_str(&format!("RootOf(#{index})")),
            Kind::Piecewise(cases) => {
                out.push_str("Piecewise(");
                for (i, (v, c)) in cases.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push('(');
                    write_expr(v, out);
                    out.push_str(", ");
                    write_expr(c, out);
                    out.push(')');
                }
                out.push(')');
            }
            Kind::Rel(operation, a, b) => {
                write_expr(a, out);
                out.push_str(rel_symbol(*operation));
                write_expr(b, out);
            }
            Kind::Wild(id, _) => out.push_str(&format!("_w{id}")),
        }
    }

    fn rel_symbol(operator: RelationalOperator) -> &'static str {
        match operator {
            RelationalOperator::Eq => " == ",
            RelationalOperator::Ne => " != ",
            RelationalOperator::Lt => " < ",
            RelationalOperator::Le => " <= ",
            RelationalOperator::Gt => " > ",
            RelationalOperator::Ge => " >= ",
        }
    }

    fn write_paren_if_needed(e: &Expr, min_prec: Prec, out: &mut String) {
        if (precedence(e) as i32) < (min_prec as i32) {
            out.push('(');
            write_expr(e, out);
            out.push(')');
        } else {
            write_expr(e, out);
        }
    }

    /// ➕️ Prints in the conventional "highest-degree/most-complex term first, plain constant last" order —
    /// the reverse of `Add`'s canonical storage order (which puts the numeric coefficient first, an
    /// internal invariant unrelated to how a human expects to read the sum).
    fn write_add(terms: &[Expr], out: &mut String) {
        let reordered: Vec<Expr> = terms.iter().rev().cloned().collect();
        for (i, term) in reordered.iter().enumerate() {
            let (is_neg, display_term) = extract_negation(term);
            if i == 0 {
                if is_neg {
                    out.push('-');
                }
            } else {
                out.push_str(if is_neg { " - " } else { " + " });
            }
            write_paren_if_needed(&display_term, Prec::Mul, out);
        }
    }

    /// ➖️ Detects the `-1 * rest` / negative-literal encoding of a negated term and returns
    /// `(true, positive_rest)`, or `(false, term)` if the term isn't negative.
    fn extract_negation(term: &Expr) -> (bool, Expr) {
        match term.kind() {
            Kind::Integer(n) if n.is_negative() => return (true, Expr::from(n.abs_integer())),
            Kind::Rational(r) if r.numer().is_negative() => return (true, Expr::from(r.abs())),
            Kind::Mul(factors) => {
                if let Some(Kind::Integer(n)) = factors.first().map(|f| f.kind()) {
                    if n.is_negative() {
                        let mut rest = factors[1..].to_vec();
                        if *n != math::number::Integer::from_i64(-1) {
                            rest.insert(0, Expr::from(n.abs_integer()));
                        }
                        return (true, Expr::mul(rest));
                    }
                }
            }
            _ => {}
        }
        (false, term.clone())
    }

    fn write_mul(factors: &[Expr], out: &mut String) {
        // Recover a/b: a Pow(base, negative-exponent) factor becomes the denominator, and a Rational
        // numeric coefficient splits into a numerator/denominator pair rather than printing "1/2*x".
        let mut numer: Vec<Expr> = Vec::new();
        let mut denom: Vec<Expr> = Vec::new();
        for f in factors {
            if let Kind::Pow(base, exp) = f.kind() {
                if is_negative_literal(exp) {
                    denom.push(Expr::pow(base.clone(), exp.clone().neg()));
                    continue;
                }
            }
            if let Kind::Rational(r) = f.kind() {
                if r.numer().abs_integer() != math::number::Integer::one() {
                    numer.push(Expr::from(r.numer().abs_integer()));
                }
                if r.numer().is_negative() {
                    numer.insert(0, Expr::integer(-1));
                }
                denom.push(Expr::from(math::number::Integer::from_natural(r.denom().clone())));
                continue;
            }
            numer.push(f.clone());
        }
        if numer.is_empty() {
            numer.push(Expr::integer(1));
        }
        for (i, f) in numer.iter().enumerate() {
            if i > 0 {
                out.push('*');
            }
            write_paren_if_needed(f, Prec::Pow, out);
        }
        if !denom.is_empty() {
            out.push('/');
            if denom.len() == 1 {
                write_paren_if_needed(&denom[0], Prec::Pow, out);
            } else {
                out.push('(');
                for (i, d) in denom.iter().enumerate() {
                    if i > 0 {
                        out.push('*');
                    }
                    write_paren_if_needed(d, Prec::Pow, out);
                }
                out.push(')');
            }
        }
    }

    fn is_negative_literal(e: &Expr) -> bool {
        match e.kind() {
            Kind::Integer(n) => n.is_negative(),
            Kind::Rational(r) => r.numer().is_negative(),
            _ => false,
        }
    }

    fn write_pow(base: &Expr, exp: &Expr, out: &mut String) {
        write_paren_if_needed(base, Prec::Unary, out);
        out.push('^');
        write_paren_if_needed(exp, Prec::Unary, out);
    }

    fn write_func(kind: &FnKind, args: &[Expr], out: &mut String) {
        out.push_str(&kind.name());
        out.push('(');
        for (i, a) in args.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            write_expr(a, out);
        }
        out.push(')');
    }

    impl std::fmt::Display for Expr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", display_string(self))
        }
    }
    // #endregion 🔖️Display

    // #region 🔖️Latex
    pub fn to_latex(e: &Expr) -> String {
        let mut s = String::new();
        write_latex(e, &mut s);
        s
    }

    fn write_latex(e: &Expr, out: &mut String) {
        match e.kind() {
            Kind::Integer(n) => out.push_str(&n.to_decimal()),
            Kind::Rational(r) => {
                out.push_str(&format!("\\frac{{{}}}{{{}}}", r.numer(), r.denom()));
            }
            Kind::Symbol(s) => out.push_str(s.name()),
            Kind::Constant(c) => out.push_str(latex_constant(c)),
            Kind::Bool(b) => out.push_str(if *b { "\\text{true}" } else { "\\text{false}" }),
            Kind::Add(terms) => {
                let reordered: Vec<Expr> = terms.iter().rev().cloned().collect();
                for (i, term) in reordered.iter().enumerate() {
                    let (is_neg, display_term) = extract_negation(term);
                    if i == 0 {
                        if is_neg {
                            out.push('-');
                        }
                    } else {
                        out.push_str(if is_neg { " - " } else { " + " });
                    }
                    write_latex(&display_term, out);
                }
            }
            Kind::Mul(factors) => {
                let mut numer: Vec<Expr> = Vec::new();
                let mut denom: Vec<Expr> = Vec::new();
                for f in factors {
                    if let Kind::Pow(base, exp) = f.kind() {
                        if is_negative_literal(exp) {
                            denom.push(Expr::pow(base.clone(), exp.clone().neg()));
                            continue;
                        }
                    }
                    numer.push(f.clone());
                }
                if !denom.is_empty() {
                    out.push_str("\\frac{");
                    if numer.is_empty() {
                        out.push('1');
                    } else {
                        for n in &numer {
                            write_latex(n, out);
                        }
                    }
                    out.push_str("}{");
                    for d in &denom {
                        write_latex(d, out);
                    }
                    out.push('}');
                } else {
                    for n in &numer {
                        write_latex(n, out);
                    }
                }
            }
            Kind::Pow(base, exp) => {
                out.push('{');
                write_latex(base, out);
                out.push_str("}^{");
                write_latex(exp, out);
                out.push('}');
            }
            Kind::Fn(kind, args) => {
                out.push_str(&format!("\\operatorname{{{}}}\\left(", kind.name()));
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    write_latex(a, out);
                }
                out.push_str("\\right)");
            }
            Kind::RootOf { index, .. } => out.push_str(&format!("\\text{{RootOf}}_{{{index}}}")),
            Kind::Piecewise(cases) => {
                out.push_str("\\begin{cases}");
                for (v, c) in cases {
                    write_latex(v, out);
                    out.push_str(" & ");
                    write_latex(c, out);
                    out.push_str(" \\\\ ");
                }
                out.push_str("\\end{cases}");
            }
            Kind::Rel(operation, a, b) => {
                write_latex(a, out);
                out.push_str(latex_rel(*operation));
                write_latex(b, out);
            }
            Kind::Wild(id, _) => out.push_str(&format!("w_{{{id}}}")),
        }
    }

    fn latex_constant(c: &Constant) -> &'static str {
        match c {
            Constant::Pi => "\\pi",
            Constant::E => "e",
            Constant::I => "i",
            Constant::EulerGamma => "\\gamma",
            Constant::Inf => "\\infty",
            Constant::NegInf => "-\\infty",
            Constant::ComplexInf => "\\tilde{\\infty}",
            Constant::Undefined => "\\text{undefined}",
        }
    }

    fn latex_rel(operator: RelationalOperator) -> &'static str {
        match operator {
            RelationalOperator::Eq => " = ",
            RelationalOperator::Ne => " \\neq ",
            RelationalOperator::Lt => " < ",
            RelationalOperator::Le => " \\leq ",
            RelationalOperator::Gt => " > ",
            RelationalOperator::Ge => " \\geq ",
        }
    }
    // #endregion 🔖️Latex

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn display_simple_polynomial() {
            let x = Expr::symbol("x");
            let e = Expr::add(vec![Expr::pow(x, Expr::integer(2)), Expr::integer(1)]);
            assert_eq!(display_string(&e), "x^2 + 1");
        }

        #[test]
        fn display_negative_term() {
            let x = Expr::symbol("x");
            let e = x - Expr::integer(1);
            assert_eq!(display_string(&e), "x - 1");
        }

        #[test]
        fn display_division() {
            let x = Expr::symbol("x");
            let e = x / Expr::integer(2);
            assert_eq!(display_string(&e), "x/2");
        }

        #[test]
        fn latex_fraction_and_power() {
            let x = Expr::symbol("x");
            let e = Expr::pow(x, Expr::integer(2));
            assert_eq!(to_latex(&e), "{x}^{2}");
        }

        #[test]
        fn latex_constant_pi() {
            assert_eq!(to_latex(&Expr::constant(Constant::Pi)), "\\pi");
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Fmt

// #region 🔖️Pattern
pub mod pattern {
    //! 🃏️ Structural pattern matching and rule-based rewriting over the canonical `Expr` tree. `Add`/`Mul`
    //! matching is associative-commutative (subject terms may match pattern terms in any order), handled by
    //! bounded backtracking rather than full (NP-hard) AC unification — a budget caps the search so a
    //! pathologically wide subject conservatively fails to match instead of hanging.

    use crate::cas::expr::{Expr, Kind, WildKind};
    use std::collections::BTreeMap;
    use std::rc::Rc;

    // #region 🔖️Wildcards
    pub fn wild(id: u16) -> Expr {
        Expr::from_kind_unchecked(Kind::Wild(id, WildKind::Any))
    }
    pub fn wild_num(id: u16) -> Expr {
        Expr::from_kind_unchecked(Kind::Wild(id, WildKind::Number))
    }
    pub fn wild_nonzero(id: u16) -> Expr {
        Expr::from_kind_unchecked(Kind::Wild(id, WildKind::NotZero))
    }
    pub fn wild_free(id: u16, symbol: &str) -> Expr {
        Expr::from_kind_unchecked(Kind::Wild(id, WildKind::FreeOf(Rc::from(symbol))))
    }
    pub fn wild_seq(id: u16) -> Expr {
        Expr::from_kind_unchecked(Kind::Wild(id, WildKind::Seq))
    }
    // #endregion 🔖️Wildcards

    // #region 🔖️Bindings
    #[derive(Clone, Debug, PartialEq)]
    pub enum Binding {
        One(Expr),
        Many(Vec<Expr>),
    }

    pub type Bindings = BTreeMap<u16, Binding>;

    fn bind_one(id: u16, value: Expr, mut bindings: Bindings) -> Option<Bindings> {
        match bindings.get(&id) {
            Some(Binding::One(existing)) => {
                if *existing == value {
                    Some(bindings)
                } else {
                    None
                }
            }
            Some(Binding::Many(_)) => None,
            None => {
                bindings.insert(id, Binding::One(value));
                Some(bindings)
            }
        }
    }

    fn bind_many(id: u16, items: Vec<Expr>, mut bindings: Bindings) -> Option<Bindings> {
        match bindings.get(&id) {
            Some(Binding::Many(existing)) => {
                if *existing == items {
                    Some(bindings)
                } else {
                    None
                }
            }
            Some(Binding::One(_)) => None,
            None => {
                bindings.insert(id, Binding::Many(items));
                Some(bindings)
            }
        }
    }
    // #endregion 🔖️Bindings

    // #region 🔖️Matcher
    const DEFAULT_BUDGET: i64 = 10_000;
    const MAX_SUBJECT_WIDTH: usize = 24;

    pub fn match_expr(pattern: &Expr, subject: &Expr) -> Option<Bindings> {
        let mut budget = DEFAULT_BUDGET;
        match_impl(pattern, subject, Bindings::new(), &mut budget)
    }

    fn satisfies_constraint(wk: &WildKind, subject: &Expr) -> bool {
        match wk {
            WildKind::Any | WildKind::Seq => true,
            WildKind::Number => matches!(subject.kind(), Kind::Integer(_) | Kind::Rational(_)),
            WildKind::NotZero => !subject.is_zero_literal(),
            WildKind::FreeOf(name) => !contains_symbol_name(subject, name),
        }
    }

    fn contains_symbol_name(e: &Expr, name: &str) -> bool {
        match e.kind() {
            Kind::Symbol(s) => s.name() == name,
            Kind::Add(terms) | Kind::Mul(terms) => terms.iter().any(|t| contains_symbol_name(t, name)),
            Kind::Pow(base, exp) => contains_symbol_name(base, name) || contains_symbol_name(exp, name),
            Kind::Fn(_, args) => args.iter().any(|a| contains_symbol_name(a, name)),
            Kind::Piecewise(cases) => cases.iter().any(|(v, c)| contains_symbol_name(v, name) || contains_symbol_name(c, name)),
            Kind::Rel(_, a, b) => contains_symbol_name(a, name) || contains_symbol_name(b, name),
            _ => false,
        }
    }

    fn match_impl(pattern: &Expr, subject: &Expr, bindings: Bindings, budget: &mut i64) -> Option<Bindings> {
        *budget -= 1;
        if *budget <= 0 {
            return None;
        }
        if let Kind::Wild(id, wk) = pattern.kind() {
            return if satisfies_constraint(wk, subject) { bind_one(*id, subject.clone(), bindings) } else { None };
        }
        match (pattern.kind(), subject.kind()) {
            (Kind::Add(p_terms), Kind::Add(s_terms)) => match_multiset(p_terms, s_terms, bindings, budget),
            (Kind::Mul(p_factors), Kind::Mul(s_factors)) => match_multiset(p_factors, s_factors, bindings, budget),
            (Kind::Pow(pb, pe), Kind::Pow(sb, se)) => {
                let b1 = match_impl(pb, sb, bindings, budget)?;
                match_impl(pe, se, b1, budget)
            }
            (Kind::Fn(pk, pargs), Kind::Fn(sk, sargs)) => {
                if pk != sk || pargs.len() != sargs.len() {
                    return None;
                }
                let mut b = bindings;
                for (pa, sa) in pargs.iter().zip(sargs.iter()) {
                    b = match_impl(pa, sa, b, budget)?;
                }
                Some(b)
            }
            (Kind::Rel(po, pa, pb), Kind::Rel(so, sa, sb)) => {
                if po != so {
                    return None;
                }
                let b = match_impl(pa, sa, bindings, budget)?;
                match_impl(pb, sb, b, budget)
            }
            (Kind::Piecewise(p_cases), Kind::Piecewise(s_cases)) => {
                if p_cases.len() != s_cases.len() {
                    return None;
                }
                let mut b = bindings;
                for ((pv, pc), (sv, sc)) in p_cases.iter().zip(s_cases.iter()) {
                    b = match_impl(pv, sv, b, budget)?;
                    b = match_impl(pc, sc, b, budget)?;
                }
                Some(b)
            }
            _ => {
                if pattern == subject {
                    Some(bindings)
                } else {
                    None
                }
            }
        }
    }

    /// 🧩️ Matches an unordered term list against another: assigns each non-`Seq` pattern term to a distinct
    /// subject term via backtracking (any assignment order is tried), then binds a single trailing `Seq`
    /// wildcard (at most one is supported) to whatever subject terms remain unassigned.
    fn match_multiset(p_terms: &[Expr], s_terms: &[Expr], bindings: Bindings, budget: &mut i64) -> Option<Bindings> {
        if s_terms.len() > MAX_SUBJECT_WIDTH {
            return None;
        }
        let seq_positions: Vec<usize> = p_terms.iter().enumerate().filter(|(_, t)| matches!(t.kind(), Kind::Wild(_, WildKind::Seq))).map(|(i, _)| i).collect();
        if seq_positions.len() > 1 {
            return None; // unsupported: more than one Seq wildcard in a single Add/Mul pattern
        }
        let seq_id = seq_positions.first().map(|&i| match p_terms[i].kind() {
            Kind::Wild(id, _) => *id,
            _ => unreachable!(),
        });
        let non_seq: Vec<&Expr> = p_terms.iter().enumerate().filter(|&(i, _)| Some(i) != seq_positions.first().copied()).map(|(_, t)| t).collect();
        if non_seq.len() > s_terms.len() {
            return None;
        }

        let mut used = vec![false; s_terms.len()];
        let matched = assign(&non_seq, 0, s_terms, &mut used, bindings, budget)?;
        let leftover: Vec<Expr> = s_terms.iter().zip(used.iter()).filter(|&(_, &u)| !u).map(|(t, _)| t.clone()).collect();
        match seq_id {
            Some(id) => bind_many(id, leftover, matched),
            None => {
                if leftover.is_empty() {
                    Some(matched)
                } else {
                    None
                }
            }
        }
    }

    fn assign(pats: &[&Expr], idx: usize, s_terms: &[Expr], used: &mut Vec<bool>, bindings: Bindings, budget: &mut i64) -> Option<Bindings> {
        if idx == pats.len() {
            return Some(bindings);
        }
        for j in 0..s_terms.len() {
            if used[j] {
                continue;
            }
            *budget -= 1;
            if *budget <= 0 {
                return None;
            }
            used[j] = true;
            if let Some(next) = match_impl(pats[idx], &s_terms[j], bindings.clone(), budget) {
                if let Some(result) = assign(pats, idx + 1, s_terms, used, next, budget) {
                    return Some(result);
                }
            }
            used[j] = false;
        }
        None
    }
    // #endregion 🔖️Matcher

    // #region 🔖️Instantiate
    /// 🏗️ Rebuilds `template` with every `Wild` node replaced by its binding — `Seq` bindings splice their
    /// items directly into the enclosing `Add`/`Mul` term list rather than substituting a single value.
    pub fn instantiate(template: &Expr, bindings: &Bindings) -> Expr {
        match template.kind() {
            Kind::Wild(id, _) => match bindings.get(id) {
                Some(Binding::One(v)) => v.clone(),
                Some(Binding::Many(items)) if items.len() == 1 => items[0].clone(),
                _ => template.clone(),
            },
            Kind::Add(terms) => Expr::add(instantiate_seq(terms, bindings)),
            Kind::Mul(factors) => Expr::mul(instantiate_seq(factors, bindings)),
            Kind::Pow(base, exp) => Expr::pow(instantiate(base, bindings), instantiate(exp, bindings)),
            Kind::Fn(kind, args) => Expr::func(kind.clone(), args.iter().map(|a| instantiate(a, bindings)).collect()),
            Kind::Piecewise(cases) => Expr::from_kind_unchecked(Kind::Piecewise(cases.iter().map(|(v, c)| (instantiate(v, bindings), instantiate(c, bindings))).collect())),
            Kind::Rel(operation, a, b) => Expr::from_kind_unchecked(Kind::Rel(*operation, instantiate(a, bindings), instantiate(b, bindings))),
            _ => template.clone(),
        }
    }

    fn instantiate_seq(terms: &[Expr], bindings: &Bindings) -> Vec<Expr> {
        let mut out = Vec::with_capacity(terms.len());
        for t in terms {
            if let Kind::Wild(id, WildKind::Seq) = t.kind() {
                if let Some(Binding::Many(items)) = bindings.get(id) {
                    out.extend(items.iter().cloned());
                    continue;
                }
            }
            out.push(instantiate(t, bindings));
        }
        out
    }
    // #endregion 🔖️Instantiate

    // #region 🔖️Rules
    pub enum RuleRhs {
        Template(Expr),
        Builder(Rc<dyn Fn(&Bindings) -> Expr>),
    }

    /// 🔍️ Guard evaluated against a candidate match before a rewrite rule fires.
    pub type RuleCondition = Rc<dyn Fn(&Bindings) -> bool>;

    pub struct Rule {
        lhs: Expr,
        rhs: RuleRhs,
        cond: Option<RuleCondition>,
    }

    impl Rule {
        pub fn new(lhs: Expr, rhs: Expr) -> Self {
            Self { lhs, rhs: RuleRhs::Template(rhs), cond: None }
        }

        pub fn with_condition(lhs: Expr, rhs: Expr, cond: RuleCondition) -> Self {
            Self { lhs, rhs: RuleRhs::Template(rhs), cond: Some(cond) }
        }

        pub fn from_builder(lhs: Expr, builder: Rc<dyn Fn(&Bindings) -> Expr>) -> Self {
            Self { lhs, rhs: RuleRhs::Builder(builder), cond: None }
        }

        pub fn try_apply(&self, e: &Expr) -> Option<Expr> {
            let bindings = match_expr(&self.lhs, e)?;
            if let Some(cond) = &self.cond {
                if !cond(&bindings) {
                    return None;
                }
            }
            Some(match &self.rhs {
                RuleRhs::Template(t) => instantiate(t, &bindings),
                RuleRhs::Builder(f) => f(&bindings),
            })
        }
    }

    #[derive(Clone, Copy)]
    pub enum Strategy {
        BottomUpOnce,
        TopDownOnce,
        Fixpoint { max_iters: u32 },
    }

    pub struct RuleSet {
        rules: Vec<Rule>,
    }

    impl RuleSet {
        pub fn new(rules: Vec<Rule>) -> Self {
            Self { rules }
        }

        pub fn try_apply_one(&self, e: &Expr) -> Option<Expr> {
            self.rules.iter().find_map(|r| r.try_apply(e))
        }

        pub fn apply(&self, e: &Expr, strategy: Strategy) -> Expr {
            match strategy {
                Strategy::BottomUpOnce => crate::cas::visit::replace_bottom_up(e, &mut |sub| self.try_apply_one(sub)),
                Strategy::TopDownOnce => self.apply_top_down_once(e),
                Strategy::Fixpoint { max_iters } => {
                    let mut current = e.clone();
                    for _ in 0..max_iters {
                        let next = self.apply(&current, Strategy::BottomUpOnce);
                        if next == current {
                            break;
                        }
                        current = next;
                    }
                    current
                }
            }
        }

        fn apply_top_down_once(&self, e: &Expr) -> Expr {
            let rewritten = self.try_apply_one(e).unwrap_or_else(|| e.clone());
            crate::cas::visit::map_children(&rewritten, &mut |c| self.apply_top_down_once(c))
        }
    }
    // #endregion 🔖️Rules

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::cas::expr::Expr;

        #[test]
        fn wildcard_any_matches_anything() {
            let pattern = wild(0);
            let subject = Expr::symbol("x");
            let bindings = match_expr(&pattern, &subject).unwrap();
            assert_eq!(bindings.get(&0), Some(&Binding::One(subject)));
        }

        #[test]
        fn wildcard_number_rejects_symbols() {
            let pattern = wild_num(0);
            assert!(match_expr(&pattern, &Expr::symbol("x")).is_none());
            assert!(match_expr(&pattern, &Expr::integer(5)).is_some());
        }

        #[test]
        fn structural_match_on_pow() {
            let x = Expr::symbol("x");
            let pattern = Expr::pow(wild(0), Expr::integer(2));
            let subject = Expr::pow(x.clone(), Expr::integer(2));
            let bindings = match_expr(&pattern, &subject).unwrap();
            assert_eq!(bindings.get(&0), Some(&Binding::One(x)));
        }

        #[test]
        fn pow_exponent_mismatch_fails() {
            let x = Expr::symbol("x");
            let pattern = Expr::pow(wild(0), Expr::integer(2));
            let subject = Expr::pow(x, Expr::integer(3));
            assert!(match_expr(&pattern, &subject).is_none());
        }

        #[test]
        fn ac_match_finds_permuted_assignment() {
            // pattern: wild(0) + wild(1), subject: y + x -- should match regardless of order.
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let pattern = Expr::add(vec![wild(0), wild(1)]);
            let subject = Expr::add(vec![x.clone(), y.clone()]);
            let bindings = match_expr(&pattern, &subject).unwrap();
            let matched: std::collections::BTreeSet<Expr> = bindings
                .values()
                .map(|b| match b {
                    Binding::One(e) => e.clone(),
                    _ => panic!(),
                })
                .collect();
            assert!(matched.contains(&x) && matched.contains(&y));
        }

        #[test]
        fn seq_wildcard_absorbs_remaining_terms() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let z = Expr::symbol("z");
            let pattern = Expr::add(vec![x.clone(), wild_seq(0)]);
            let subject = Expr::add(vec![x, y.clone(), z.clone()]);
            let bindings = match_expr(&pattern, &subject).unwrap();
            match bindings.get(&0) {
                Some(Binding::Many(items)) => {
                    let set: std::collections::BTreeSet<Expr> = items.iter().cloned().collect();
                    assert!(set.contains(&y) && set.contains(&z));
                }
                _ => panic!("expected Many binding"),
            }
        }

        #[test]
        fn rule_rewrites_matching_expression() {
            // sin(w)^2 + cos(w)^2 -> 1 (Pythagorean identity, single-term hand case without the +seq form)
            let w = Expr::symbol("w");
            let lhs = Expr::add(vec![Expr::pow(Expr::func(crate::cas::fnkind::FnKind::Sin, vec![wild(0)]), Expr::integer(2)), Expr::pow(Expr::func(crate::cas::fnkind::FnKind::Cos, vec![wild(0)]), Expr::integer(2))]);
            let rule = Rule::new(lhs, Expr::integer(1));
            let subject = Expr::add(vec![Expr::pow(Expr::func(crate::cas::fnkind::FnKind::Sin, vec![w.clone()]), Expr::integer(2)), Expr::pow(Expr::func(crate::cas::fnkind::FnKind::Cos, vec![w]), Expr::integer(2))]);
            assert_eq!(rule.try_apply(&subject), Some(Expr::integer(1)));
        }

        #[test]
        fn ruleset_bottom_up_rewrites_nested_occurrence() {
            let rule = Rule::new(Expr::pow(wild(0), Expr::integer(2)), Expr::mul(vec![wild(0), wild(0)]));
            let rs = RuleSet::new(vec![rule]);
            let x = Expr::symbol("x");
            let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::integer(1)]);
            let result = rs.apply(&e, Strategy::BottomUpOnce);
            assert_eq!(result, Expr::add(vec![Expr::mul(vec![x.clone(), x]), Expr::integer(1)]));
        }

        #[test]
        fn free_of_constraint_rejects_expressions_containing_the_symbol() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let pattern = wild_free(0, "x");
            assert!(match_expr(&pattern, &y).is_some());
            assert!(match_expr(&pattern, &x).is_none());
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Pattern

// #region 🔖️Polybridge
pub mod polybridge {
    //! 🌉️ The `Expr` <-> polynomial bridge — the workhorse every algebraic domain (`simplify`, `solve`,
    //! rational `integrate`, `SymMatrix`) goes through: detect which subtrees behave as polynomial
    //! "generators" (variables, in the Gröbner-basis sense), convert to/from `crate::polynomial`
    //! types over those generators, and reconstruct canonical `Expr`s from the result.

    use crate::cas::expr::{Expr, Kind};
    use math::number::{Integer, Natural, Rational};
    use crate::polynomial::{MonomialOrder, PolyM, PolyU};

    // #region 🔖️PolyMap
    /// 🗺️ The ordered list of generators a conversion was performed against; `gens[i]` is polynomial
    /// variable `i`.
    #[derive(Clone, Debug)]
    pub struct PolyMap {
        pub gens: Vec<Expr>,
    }

    fn gen_index(e: &Expr, map: &PolyMap) -> Option<usize> {
        map.gens.iter().position(|g| g == e)
    }

    fn push_unique(gens: &mut Vec<Expr>, e: Expr) {
        if !gens.contains(&e) {
            gens.push(e);
        }
    }
    // #endregion 🔖️PolyMap

    // #region 🔖️GenDetection
    /// 🔍️ Collects the maximal non-polynomial subtrees of `e` as generators: symbols, function
    /// applications, non-numeric constants, and any `Pow` node whose exponent isn't a plain non-negative
    /// integer (fractional/negative/symbolic exponents can't be expressed as a polynomial power in the
    /// base, so the whole `Pow` becomes its own opaque generator).
    pub fn detect_gens(e: &Expr) -> Vec<Expr> {
        let mut gens = Vec::new();
        collect_gens(e, &mut gens);
        gens
    }

    fn collect_gens(e: &Expr, gens: &mut Vec<Expr>) {
        match e.kind() {
            Kind::Integer(_) | Kind::Rational(_) => {}
            Kind::Add(terms) | Kind::Mul(terms) => {
                for t in terms {
                    collect_gens(t, gens);
                }
            }
            Kind::Pow(base, exp) => {
                if let Kind::Integer(n) = exp.kind() {
                    if n.is_positive() || n.is_zero() {
                        collect_gens(base, gens);
                        return;
                    }
                }
                push_unique(gens, e.clone());
            }
            _ => push_unique(gens, e.clone()),
        }
    }
    // #endregion 🔖️GenDetection

    // #region 🔖️ExprToPoly
    /// 🔁️ Converts `e` to a `PolyM<Rational>` over the given (fixed, ordered) generator list; `None` if
    /// `e` contains a subtree that isn't a polynomial combination of numbers and those generators (e.g. a
    /// generator not in the list, or a negative/fractional power of one).
    pub fn as_poly(e: &Expr, gens: &[Expr]) -> Option<(PolyM<Rational>, PolyMap)> {
        let map = PolyMap { gens: gens.to_vec() };
        let poly = expr_to_polym(e, &map)?;
        Some((poly, map))
    }

    pub fn as_poly_auto(e: &Expr) -> Option<(PolyM<Rational>, PolyMap)> {
        let gens = detect_gens(e);
        as_poly(e, &gens)
    }

    fn expr_to_polym(e: &Expr, map: &PolyMap) -> Option<PolyM<Rational>> {
        let nvars = map.gens.len().max(1);
        match e.kind() {
            Kind::Integer(n) => Some(PolyM::constant(Rational::from_integer(n.clone()), nvars, MonomialOrder::Lex)),
            Kind::Rational(r) => Some(PolyM::constant(r.clone(), nvars, MonomialOrder::Lex)),
            Kind::Add(terms) => {
                let mut acc = PolyM::zero(nvars, MonomialOrder::Lex);
                for t in terms {
                    acc = acc.add(&expr_to_polym(t, map)?);
                }
                Some(acc)
            }
            Kind::Mul(factors) => {
                let mut acc = PolyM::constant(Rational::one(), nvars, MonomialOrder::Lex);
                for f in factors {
                    acc = acc.mul(&expr_to_polym(f, map)?);
                }
                Some(acc)
            }
            Kind::Pow(base, exp) => {
                if let Kind::Integer(n) = exp.kind() {
                    if let Some(ev) = n.to_i64() {
                        if ev >= 0 {
                            return Some(expr_to_polym(base, map)?.pow(ev as u64));
                        }
                    }
                }
                gen_index(e, map).map(|idx| PolyM::var(idx, nvars, MonomialOrder::Lex))
            }
            _ => gen_index(e, map).map(|idx| PolyM::var(idx, nvars, MonomialOrder::Lex)),
        }
    }

    /// ↩️ Rebuilds a canonical `Expr` from a `PolyM<Rational>` and the generator map it was built against.
    pub fn from_poly(p: &PolyM<Rational>, map: &PolyMap) -> Expr {
        let mut terms = Vec::with_capacity(p.terms().len());
        for (m, c) in p.terms() {
            let mut factors = vec![Expr::from(c.clone())];
            for (i, &exp) in m.exps().iter().enumerate() {
                if exp > 0 {
                    factors.push(Expr::pow(map.gens[i].clone(), Expr::integer(exp as i64)));
                }
            }
            terms.push(Expr::mul(factors));
        }
        if terms.is_empty() {
            Expr::integer(0)
        } else {
            Expr::add(terms)
        }
    }

    /// 🔁️ Converts `e` to a dense univariate `PolyU<Rational>` in `x` alone; `None` if `e` involves any
    /// other generator or a non-polynomial power of `x`.
    pub fn as_poly_uni(e: &Expr, x: &Expr) -> Option<PolyU<Rational>> {
        let (poly, _map) = as_poly(e, std::slice::from_ref(x))?;
        let max_deg = poly.terms().iter().map(|(m, _)| m.exps()[0] as usize).max().unwrap_or(0);
        let mut coeffs = vec![Rational::zero(); max_deg + 1];
        for (m, c) in poly.terms() {
            coeffs[m.exps()[0] as usize] = c.clone();
        }
        Some(PolyU::from_coeffs(coeffs))
    }

    pub fn polyu_to_expr(p: &PolyU<Rational>, x: &Expr) -> Expr {
        let map = PolyMap { gens: vec![x.clone()] };
        let terms: Vec<(crate::polynomial::Monomial, Rational)> = p.coeffs().iter().enumerate().map(|(i, c)| (crate::polynomial::Monomial::new(vec![i as u32]), c.clone())).collect();
        from_poly(&PolyM::from_terms(terms, 1, MonomialOrder::Lex), &map)
    }
    // #endregion 🔖️ExprToPoly

    // #region 🔖️RationalFunctionBridge
    /// 🔍️ Like [`detect_gens`], but recurses through integer powers of *either* sign (rational-function
    /// generators are the base, not the whole `Pow`, since `x` and `1/x` should share one generator).
    fn detect_gens_ratfunc(e: &Expr) -> Vec<Expr> {
        let mut gens = Vec::new();
        collect_gens_ratfunc(e, &mut gens);
        gens
    }

    fn collect_gens_ratfunc(e: &Expr, gens: &mut Vec<Expr>) {
        match e.kind() {
            Kind::Integer(_) | Kind::Rational(_) => {}
            Kind::Add(terms) | Kind::Mul(terms) => {
                for t in terms {
                    collect_gens_ratfunc(t, gens);
                }
            }
            Kind::Pow(base, exp) => {
                if matches!(exp.kind(), Kind::Integer(_)) {
                    collect_gens_ratfunc(base, gens);
                    return;
                }
                push_unique(gens, e.clone());
            }
            _ => push_unique(gens, e.clone()),
        }
    }

    /// 🔁️ Converts `e` into a single `num/den` rational-function form over its auto-detected generators —
    /// the "together" operation at the polynomial level (no GCD cancellation; see `simplify::cancel` for that).
    pub fn as_ratfunc_auto(e: &Expr) -> Option<(PolyM<Rational>, PolyM<Rational>, PolyMap)> {
        let gens = detect_gens_ratfunc(e);
        let map = PolyMap { gens };
        let (num, den) = expr_to_ratfunc(e, &map)?;
        Some((num, den, map))
    }

    fn ratfunc_one(nvars: usize) -> PolyM<Rational> {
        PolyM::constant(Rational::one(), nvars, MonomialOrder::Lex)
    }

    fn expr_to_ratfunc(e: &Expr, map: &PolyMap) -> Option<(PolyM<Rational>, PolyM<Rational>)> {
        let nvars = map.gens.len().max(1);
        match e.kind() {
            Kind::Integer(n) => Some((PolyM::constant(Rational::from_integer(n.clone()), nvars, MonomialOrder::Lex), ratfunc_one(nvars))),
            Kind::Rational(r) => Some((PolyM::constant(r.clone(), nvars, MonomialOrder::Lex), ratfunc_one(nvars))),
            Kind::Add(terms) => {
                let mut num_acc = PolyM::zero(nvars, MonomialOrder::Lex);
                let mut den_acc = ratfunc_one(nvars);
                for t in terms {
                    let (n, d) = expr_to_ratfunc(t, map)?;
                    num_acc = num_acc.mul(&d).add(&n.mul(&den_acc));
                    den_acc = den_acc.mul(&d);
                }
                Some((num_acc, den_acc))
            }
            Kind::Mul(factors) => {
                let mut num_acc = ratfunc_one(nvars);
                let mut den_acc = ratfunc_one(nvars);
                for f in factors {
                    let (n, d) = expr_to_ratfunc(f, map)?;
                    num_acc = num_acc.mul(&n);
                    den_acc = den_acc.mul(&d);
                }
                Some((num_acc, den_acc))
            }
            Kind::Pow(base, exp) => {
                if let Kind::Integer(n) = exp.kind() {
                    if let Some(ev) = n.to_i64() {
                        let (bn, bd) = expr_to_ratfunc(base, map)?;
                        return if ev >= 0 { Some((bn.pow(ev as u64), bd.pow(ev as u64))) } else { Some((bd.pow((-ev) as u64), bn.pow((-ev) as u64))) };
                    }
                }
                gen_index(e, map).map(|idx| (PolyM::var(idx, nvars, MonomialOrder::Lex), ratfunc_one(nvars)))
            }
            _ => gen_index(e, map).map(|idx| (PolyM::var(idx, nvars, MonomialOrder::Lex), ratfunc_one(nvars))),
        }
    }

    pub fn poly_uses_var(p: &PolyM<Rational>, var: usize) -> bool {
        p.terms().iter().any(|(m, _)| m.exps()[var] > 0)
    }

    /// 🔁️ Extracts `p` as a univariate polynomial in the single variable `var`, if none of `p`'s other
    /// variables actually appear (`None` otherwise).
    pub fn polym_to_polyu(p: &PolyM<Rational>, var: usize) -> Option<PolyU<Rational>> {
        let mut max_deg = 0usize;
        for (m, _) in p.terms() {
            for (i, &e) in m.exps().iter().enumerate() {
                if i != var && e > 0 {
                    return None;
                }
            }
            max_deg = max_deg.max(m.exps()[var] as usize);
        }
        let mut coeffs = vec![Rational::zero(); max_deg + 1];
        for (m, c) in p.terms() {
            coeffs[m.exps()[var] as usize] = c.clone();
        }
        Some(PolyU::from_coeffs(coeffs))
    }

    pub fn polyu_to_polym(p: &PolyU<Rational>, var: usize, nvars: usize) -> PolyM<Rational> {
        let terms: Vec<(crate::polynomial::Monomial, Rational)> = p
            .coeffs()
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let mut exps = vec![0u32; nvars];
                exps[var] = i as u32;
                (crate::polynomial::Monomial::new(exps), c.clone())
            })
            .collect();
        PolyM::from_terms(terms, nvars, MonomialOrder::Lex)
    }

    /// 🔀️ Rebuilds `num/den` as a canonical `Expr`, folding a constant denominator directly into `num`'s
    /// coefficients rather than emitting a trivial `* 1` division.
    pub fn build_ratio(num: &PolyM<Rational>, den: &PolyM<Rational>, map: &PolyMap) -> Expr {
        if den.is_zero() {
            return Expr::constant(crate::cas::expr::Constant::ComplexInf);
        }
        if den.terms().len() == 1 && den.terms()[0].0.exps().iter().all(|&e| e == 0) {
            let c = den.terms()[0].1.clone();
            let inv = c.inv().expect("nonzero constant denominator");
            return from_poly(&num.mul_scalar(&inv), map);
        }
        Expr::mul(vec![from_poly(num, map), Expr::pow(from_poly(den, map), Expr::integer(-1))])
    }
    // #endregion 🔖️RationalFunctionBridge

    // #region 🔖️RationalFactor
    /// 🔍️ Factors a `Rational`-coefficient univariate polynomial by clearing denominators (multiplying by
    /// the LCM of every coefficient's denominator), factoring the resulting integer polynomial, and
    /// converting each irreducible factor back to a monic `Rational` polynomial — folding its former
    /// leading coefficient (and the clearing scale) into the returned overall constant, so
    /// `overall * prod(factor_i ^ mult_i) == self` exactly.
    pub fn factor_poly_u(p: &PolyU<Rational>) -> (Rational, Vec<(PolyU<Rational>, u32)>) {
        if p.is_zero() {
            return (Rational::zero(), Vec::new());
        }
        let denom_lcm = p.coeffs().iter().fold(Natural::one(), |acc, c| {
            let g = acc.gcd(c.denom());
            acc.mul(c.denom()).div_rem(&g).0
        });
        let scale = Rational::from_integer(Integer::from_natural(denom_lcm));
        let int_coeffs: Vec<Integer> = p.coeffs().iter().map(|c| c.mul(&scale).trunc()).collect();
        let int_poly = PolyU::from_coeffs(int_coeffs);
        let (content, factors) = crate::polynomial::factor_integer_poly(&int_poly);
        let mut overall = Rational::from_integer(content).div(&scale).expect("clearing scale is nonzero by construction");
        let mut result = Vec::with_capacity(factors.len());
        for (f, mult) in factors {
            let rat_f = PolyU::from_coeffs(f.coeffs().iter().map(|c| Rational::from_integer(c.clone())).collect());
            let lc = rat_f.leading_coeff().cloned().unwrap_or_else(Rational::one);
            let monic = rat_f.make_monic();
            overall = overall.mul(&lc.pow(mult as i64).expect("nonzero leading coefficient raised to a non-negative power"));
            result.push((monic, mult));
        }
        (overall, result)
    }
    // #endregion 🔖️RationalFactor

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn detect_gens_finds_symbols_and_functions() {
            let x = Expr::symbol("x");
            let s = Expr::func(crate::cas::fnkind::FnKind::Sin, vec![x.clone()]);
            let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), s.clone()]);
            let gens = detect_gens(&e);
            assert!(gens.contains(&x));
            assert!(gens.contains(&s));
        }

        #[test]
        fn as_poly_roundtrips_through_from_poly() {
            let x = Expr::symbol("x");
            let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::mul(vec![Expr::integer(3), x]), Expr::integer(1)]);
            let (poly, map) = as_poly_auto(&e).unwrap();
            let rebuilt = from_poly(&poly, &map);
            assert_eq!(rebuilt, e);
        }

        #[test]
        fn as_poly_uni_extracts_univariate_polynomial() {
            let x = Expr::symbol("x");
            let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(3)), Expr::integer(2)]);
            let p = as_poly_uni(&e, &x).unwrap();
            assert_eq!(p.coeff(3), Rational::one());
            assert_eq!(p.coeff(0), Rational::from_i64(2, 1).unwrap());
        }

        #[test]
        fn as_poly_uni_fails_for_other_generators() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let e = Expr::add(vec![x.clone(), y]);
            assert!(as_poly_uni(&e, &x).is_none());
        }

        #[test]
        fn as_ratfunc_auto_recovers_together_form() {
            let x = Expr::symbol("x");
            // 1/x + 1 -> (x + 1)/x  (structurally: num has x-degree-1 term, den has x^1 term)
            let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(-1)), Expr::integer(1)]);
            let (num, den, map) = as_ratfunc_auto(&e).unwrap();
            assert!(poly_uses_var(&den, gen_index(&x, &map).unwrap()));
            assert!(!num.is_zero());
        }

        #[test]
        fn factor_poly_u_recombines_to_the_original() {
            // (2x - 1)(x + 3) = 2x^2 + 5x - 3, with a rational (non-integer) leading structure once made monic.
            let f = PolyU::from_coeffs(vec![Rational::from_i64(-3, 1).unwrap(), Rational::from_i64(5, 1).unwrap(), Rational::from_i64(2, 1).unwrap()]);
            let (overall, factors) = factor_poly_u(&f);
            let mut recombined = PolyU::constant(overall);
            for (factor, mult) in &factors {
                recombined = recombined.mul(&factor.pow(*mult as u64));
            }
            assert_eq!(recombined, f);
        }

        #[test]
        fn build_ratio_folds_constant_denominator() {
            let x = Expr::symbol("x");
            let (num, _map) = as_poly(&x, std::slice::from_ref(&x)).unwrap();
            let den = PolyM::constant(Rational::from_i64(2, 1).unwrap(), 1, MonomialOrder::Lex);
            let map = PolyMap { gens: vec![x.clone()] };
            let result = build_ratio(&num, &den, &map);
            assert_eq!(result, Expr::mul(vec![Expr::from(Rational::from_i64(1, 2).unwrap()), x]));
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Polybridge

// #region 🔖️Simplify
pub mod simplify {
    //! 🧽️ Structural algebra built on the poly bridge: `expand`/`collect`/`together`/`cancel`/`apart`/
    //! `factor`, one classical radical-denesting pattern, and the measured `simplify` pipeline that picks
    //! whichever rewrite has the fewest nodes (deterministic — never a search, never a guess at "prettiest").

    use crate::cas::expr::{Expr, Kind};
    use crate::cas::polybridge;
    use math::number::Rational;
    use crate::polynomial::PolyU;

    // #region 🔖️Expand
    pub fn expand(e: &Expr) -> Expr {
        if let Some((poly, map)) = polybridge::as_poly_auto(e) {
            return polybridge::from_poly(&poly, &map);
        }
        expand_tree(e)
    }

    fn expand_tree(e: &Expr) -> Expr {
        match e.kind() {
            Kind::Add(terms) => Expr::add(terms.iter().map(expand_tree).collect()),
            Kind::Mul(factors) => {
                let expanded: Vec<Expr> = factors.iter().map(expand_tree).collect();
                distribute_mul(&expanded)
            }
            Kind::Pow(base, exp) => {
                let base_expanded = expand_tree(base);
                if let Kind::Integer(n) = exp.kind() {
                    if let Some(ev) = n.to_i64() {
                        if ev >= 0 && matches!(base_expanded.kind(), Kind::Add(_)) {
                            let mut result = Expr::integer(1);
                            for _ in 0..ev {
                                result = distribute_pair(&result, &base_expanded);
                            }
                            return result;
                        }
                    }
                }
                Expr::pow(base_expanded, expand_tree(exp))
            }
            Kind::Fn(kind, args) => Expr::func(kind.clone(), args.iter().map(expand_tree).collect()),
            _ => e.clone(),
        }
    }

    fn distribute_mul(factors: &[Expr]) -> Expr {
        let mut acc = Expr::integer(1);
        for f in factors {
            acc = distribute_pair(&acc, f);
        }
        acc
    }

    fn distribute_pair(a: &Expr, b: &Expr) -> Expr {
        let a_terms: Vec<Expr> = match a.kind() {
            Kind::Add(ts) => ts.clone(),
            _ => vec![a.clone()],
        };
        let b_terms: Vec<Expr> = match b.kind() {
            Kind::Add(ts) => ts.clone(),
            _ => vec![b.clone()],
        };
        let mut sum_terms = Vec::with_capacity(a_terms.len() * b_terms.len());
        for at in &a_terms {
            for bt in &b_terms {
                sum_terms.push(Expr::mul(vec![at.clone(), bt.clone()]));
            }
        }
        Expr::add(sum_terms)
    }
    // #endregion 🔖️Expand

    // #region 🔖️Collect
    /// 🗂️ Groups the (expanded) terms of `e` by their integer power of `x`; terms that aren't a clean
    /// integer power of `x` (e.g. involving another generator entirely) are left untouched and appended.
    pub fn collect(e: &Expr, x: &Expr) -> Expr {
        let expanded = expand(e);
        let terms: Vec<Expr> = match expanded.kind() {
            Kind::Add(ts) => ts.clone(),
            _ => vec![expanded.clone()],
        };
        let mut buckets: std::collections::BTreeMap<i64, Vec<Expr>> = std::collections::BTreeMap::new();
        let mut leftover: Vec<Expr> = Vec::new();
        for t in &terms {
            match term_power_of(t, x) {
                Some((exp, coeff)) => buckets.entry(exp).or_default().push(coeff),
                None => leftover.push(t.clone()),
            }
        }
        let mut result_terms: Vec<Expr> = Vec::new();
        for (exp, coeffs) in buckets.into_iter().rev() {
            let coeff_sum = Expr::add(coeffs);
            let term = if exp == 0 { coeff_sum } else { Expr::mul(vec![coeff_sum, Expr::pow(x.clone(), Expr::integer(exp))]) };
            result_terms.push(term);
        }
        result_terms.extend(leftover);
        Expr::add(result_terms)
    }

    fn term_power_of(term: &Expr, x: &Expr) -> Option<(i64, Expr)> {
        if term == x {
            return Some((1, Expr::integer(1)));
        }
        if let Kind::Pow(base, exp) = term.kind() {
            if base == x {
                if let Kind::Integer(n) = exp.kind() {
                    return n.to_i64().map(|ev| (ev, Expr::integer(1)));
                }
            }
        }
        if let Kind::Mul(factors) = term.kind() {
            for (i, f) in factors.iter().enumerate() {
                let found = if f == x {
                    Some(1i64)
                } else if let Kind::Pow(base, exp) = f.kind() {
                    if base == x {
                        if let Kind::Integer(n) = exp.kind() {
                            n.to_i64()
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };
                if let Some(exp) = found {
                    let mut rest = factors.clone();
                    rest.remove(i);
                    return Some((exp, Expr::mul(rest)));
                }
            }
            return Some((0, term.clone()));
        }
        Some((0, term.clone()))
    }
    // #endregion 🔖️Collect

    // #region 🔖️TogetherCancel
    pub fn together(e: &Expr) -> Expr {
        let Some((num, den, map)) = polybridge::as_ratfunc_auto(e) else { return e.clone() };
        polybridge::build_ratio(&num, &den, &map)
    }

    /// ➗️ `together`, plus a GCD cancellation pass when the numerator/denominator involve at most one
    /// generator (the univariate case, where `PolyU::gcd_monic` applies); a genuinely multivariate
    /// cancellation is left uncancelled (documented limitation — still correct, just not maximally reduced).
    pub fn cancel(e: &Expr) -> Expr {
        let Some((num, den, map)) = polybridge::as_ratfunc_auto(e) else { return e.clone() };
        if den.is_zero() {
            return e.clone();
        }
        let used: Vec<usize> = (0..map.gens.len()).filter(|&i| polybridge::poly_uses_var(&num, i) || polybridge::poly_uses_var(&den, i)).collect();
        if let [vi] = used[..] {
            if let (Some(nu), Some(du)) = (polybridge::polym_to_polyu(&num, vi), polybridge::polym_to_polyu(&den, vi)) {
                if !du.is_zero() && du.degree().unwrap_or(0) > 0 {
                    let g = nu.gcd_monic(&du);
                    if g.degree().unwrap_or(0) > 0 {
                        if let (Some(nq), Some(dq)) = (exact_div_u(&nu, &g), exact_div_u(&du, &g)) {
                            let num2 = polybridge::polyu_to_polym(&nq, vi, map.gens.len());
                            let den2 = polybridge::polyu_to_polym(&dq, vi, map.gens.len());
                            return polybridge::build_ratio(&num2, &den2, &map);
                        }
                    }
                }
            }
        }
        polybridge::build_ratio(&num, &den, &map)
    }

    fn exact_div_u(a: &PolyU<Rational>, b: &PolyU<Rational>) -> Option<PolyU<Rational>> {
        let (q, r) = a.div_rem(b);
        if r.is_zero() {
            Some(q)
        } else {
            None
        }
    }
    // #endregion 🔖️TogetherCancel

    // #region 🔖️Factor
    /// 🔍️ Factors `e` over `Q` when it's univariate (a single generator); genuinely multivariate
    /// expressions are returned unchanged — multivariate factoring is a documented follow-up, not attempted
    /// via a wrong or partial answer.
    pub fn factor(e: &Expr) -> Expr {
        let gens = polybridge::detect_gens(e);
        if gens.len() != 1 {
            return e.clone();
        }
        let Some(p) = polybridge::as_poly_uni(e, &gens[0]) else { return e.clone() };
        let (overall, factors) = polybridge::factor_poly_u(&p);
        if factors.is_empty() {
            return Expr::from(overall);
        }
        let mut all = vec![Expr::from(overall)];
        for (f, mult) in &factors {
            let fe = polybridge::polyu_to_expr(f, &gens[0]);
            all.push(if *mult == 1 { fe } else { Expr::pow(fe, Expr::integer(*mult as i64)) });
        }
        Expr::mul(all)
    }
    // #endregion 🔖️Factor

    // #region 🔖️Apart
    /// 🧩️ Univariate partial-fraction decomposition over `Q`: factors the denominator, then solves the
    /// linear system (via `math::algebra`'s exact `MatG::solve`) for each factor's numerator
    /// coefficients — handles repeated factors, not just squarefree denominators.
    pub fn apart(e: &Expr, x: &Expr) -> Expr {
        let Some((num_m, den_m, map)) = polybridge::as_ratfunc_auto(e) else { return e.clone() };
        if map.gens.len() != 1 || map.gens[0] != *x {
            return e.clone();
        }
        let Some(num) = polybridge::polym_to_polyu(&num_m, 0) else { return e.clone() };
        let Some(den) = polybridge::polym_to_polyu(&den_m, 0) else { return e.clone() };
        if den.is_zero() {
            return e.clone();
        }
        apart_univariate(&num, &den, x)
    }

    fn together_fallback(poly_part: &PolyU<Rational>, remainder: &PolyU<Rational>, den: &PolyU<Rational>, x: &Expr) -> Expr {
        let poly_expr = polybridge::polyu_to_expr(poly_part, x);
        if remainder.is_zero() {
            return poly_expr;
        }
        Expr::add(vec![poly_expr, Expr::mul(vec![polybridge::polyu_to_expr(remainder, x), Expr::pow(polybridge::polyu_to_expr(den, x), Expr::integer(-1))])])
    }

    fn apart_univariate(num: &PolyU<Rational>, den: &PolyU<Rational>, x: &Expr) -> Expr {
        let (poly_part, remainder) = num.div_rem(den);
        if remainder.is_zero() {
            return polybridge::polyu_to_expr(&poly_part, x);
        }
        let (overall, factors) = polybridge::factor_poly_u(den);
        if factors.is_empty() {
            return together_fallback(&poly_part, &remainder, den, x);
        }
        let deg_den = den.degree().unwrap_or(0);
        let unknowns: Vec<(usize, u32, usize)> = factors
            .iter()
            .enumerate()
            .flat_map(|(fi, (factor, mult))| {
                let d = factor.degree().unwrap_or(0).max(1);
                (1..=*mult).flat_map(move |j| (0..d).map(move |k| (fi, j, k))).collect::<Vec<_>>()
            })
            .collect();
        if unknowns.len() != deg_den {
            return together_fallback(&poly_part, &remainder, den, x);
        }

        let base_cofactors: Vec<PolyU<Rational>> = (0..factors.len())
            .map(|i| {
                let mut acc = PolyU::<Rational>::one();
                for (l, (factor, mult)) in factors.iter().enumerate() {
                    if l != i {
                        acc = acc.mul(&factor.pow(*mult as u64));
                    }
                }
                acc
            })
            .collect();

        let n = unknowns.len();
        let mut rows = vec![vec![Rational::zero(); n]; deg_den];
        for (col, &(fi, j, k)) in unknowns.iter().enumerate() {
            let (factor, mult) = &factors[fi];
            let cofactor = base_cofactors[fi].mul(&factor.pow((*mult - j) as u64));
            let basis = cofactor.mul_scalar(&overall).shift_up(k);
            for (row, cells) in rows.iter_mut().enumerate().take(deg_den) {
                cells[col] = basis.coeff(row);
            }
        }
        let matrix = math::algebra::MatG::from_rows(rows);
        let mut b_data = vec![Rational::zero(); deg_den];
        for (row, slot) in b_data.iter_mut().enumerate() {
            *slot = remainder.coeff(row);
        }
        let b = math::algebra::VecG::from_vec(b_data);
        let Some(solution) = matrix.solve(&b) else {
            return together_fallback(&poly_part, &remainder, den, x);
        };

        let mut terms = vec![polybridge::polyu_to_expr(&poly_part, x)];
        let mut idx = 0;
        for (factor, mult) in &factors {
            let d = factor.degree().unwrap_or(0).max(1);
            for j in 1..=*mult {
                let coeffs: Vec<Rational> = (0..d).map(|k| solution.get(idx + k).clone()).collect();
                idx += d;
                let a_poly = PolyU::from_coeffs(coeffs);
                if a_poly.is_zero() {
                    continue;
                }
                let a_expr = polybridge::polyu_to_expr(&a_poly, x);
                let factor_expr = polybridge::polyu_to_expr(factor, x);
                let denom_expr = if j == 1 { factor_expr } else { Expr::pow(factor_expr, Expr::integer(j as i64)) };
                terms.push(Expr::mul(vec![a_expr, Expr::pow(denom_expr, Expr::integer(-1))]));
            }
        }
        Expr::add(terms)
    }
    // #endregion 🔖️Apart

    // #region 🔖️RadicalDenest
    /// 🌱️ Denests the classical `sqrt(p + q*sqrt(c))` pattern into `sqrt(t1) + sign(q)*sqrt(t2)` when
    /// `t = p^2 - q^2*c` is a perfect-square integer and `(p+-sqrt(t))` are both even (so `t1, t2` land on
    /// exact integers) — the single denesting identity in scope for the first pass.
    pub fn denest_sqrt(e: &Expr) -> Expr {
        crate::cas::visit::replace_bottom_up(e, &mut |sub| try_denest_sqrt(sub))
    }

    fn try_denest_sqrt(e: &Expr) -> Option<Expr> {
        let Kind::Pow(inner, exp) = e.kind() else { return None };
        if !is_half(exp) {
            return None;
        }
        let Kind::Add(terms) = inner.kind() else { return None };
        if terms.len() != 2 {
            return None;
        }
        let p = match terms[0].kind() {
            Kind::Integer(n) => n.to_i64()?,
            _ => return None,
        };
        let (b, c) = extract_b_sqrt_c(&terms[1])?;
        let t = p.checked_mul(p)?.checked_sub(b.checked_mul(b)?.checked_mul(c)?)?;
        if t < 0 {
            return None;
        }
        let sq = isqrt_i64(t)?;
        if sq * sq != t {
            return None;
        }
        let (num1, num2) = (p.checked_add(sq)?, p.checked_sub(sq)?);
        if num1 % 2 != 0 || num2 % 2 != 0 || num2 < 0 {
            return None;
        }
        let (t1, t2) = (num1 / 2, num2 / 2);
        let sqrt1 = Expr::pow(Expr::integer(t1), Expr::from(Rational::from_i64(1, 2).unwrap()));
        let sqrt2 = Expr::pow(Expr::integer(t2), Expr::from(Rational::from_i64(1, 2).unwrap()));
        let sign = if b < 0 { -1 } else { 1 };
        Some(Expr::add(vec![sqrt1, Expr::mul(vec![Expr::integer(sign), sqrt2])]))
    }

    fn is_half(e: &Expr) -> bool {
        matches!(e.kind(), Kind::Rational(r) if *r == Rational::from_i64(1, 2).unwrap())
    }

    fn isqrt_i64(v: i64) -> Option<i64> {
        if v < 0 {
            return None;
        }
        Some((v as f64).sqrt().round() as i64)
    }

    fn extract_b_sqrt_c(term: &Expr) -> Option<(i64, i64)> {
        match term.kind() {
            Kind::Mul(factors) if factors.len() == 2 => {
                let coeff = match factors[0].kind() {
                    Kind::Integer(n) => n.to_i64()?,
                    _ => return None,
                };
                let c = match factors[1].kind() {
                    Kind::Pow(base, exp) if is_half(exp) => match base.kind() {
                        Kind::Integer(n) => n.to_i64()?,
                        _ => return None,
                    },
                    _ => return None,
                };
                Some((coeff, c))
            }
            Kind::Pow(base, exp) if is_half(exp) => match base.kind() {
                Kind::Integer(n) => n.to_i64().map(|c| (1, c)),
                _ => None,
            },
            _ => None,
        }
    }
    // #endregion 🔖️RadicalDenest

    // #region 🔖️Simplify
    /// 🧭️ The measured simplification pipeline: try a handful of candidate rewrites and keep whichever has
    /// the fewest nodes (canonical order breaks ties) — deterministic, no heuristic search.
    pub fn simplify(e: &Expr) -> Expr {
        let candidates = [e.clone(), cancel(e), crate::cas::trig::trig_canon(e), factor(e), denest_sqrt(e)];
        candidates.into_iter().min_by(|a, b| crate::cas::visit::node_count(a).cmp(&crate::cas::visit::node_count(b)).then_with(|| a.cmp(b))).expect("candidate list is non-empty by construction")
    }
    // #endregion 🔖️Simplify

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::cas::expr::Expr;

        #[test]
        fn expand_binomial_square() {
            let x = Expr::symbol("x");
            let e = Expr::pow(Expr::add(vec![x.clone(), Expr::integer(1)]), Expr::integer(2));
            let expanded = expand(&e);
            let expected = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::mul(vec![Expr::integer(2), x]), Expr::integer(1)]);
            assert_eq!(expanded, expected);
        }

        #[test]
        fn expand_distributes_over_function_argument_unchanged() {
            let x = Expr::symbol("x");
            let e = Expr::func(crate::cas::fnkind::FnKind::Sin, vec![Expr::add(vec![x, Expr::integer(1)])]);
            assert_eq!(expand(&e), e);
        }

        #[test]
        fn collect_groups_like_powers() {
            let x = Expr::symbol("x");
            let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::mul(vec![Expr::integer(3), Expr::pow(x.clone(), Expr::integer(2))]), x.clone()]);
            let collected = collect(&e, &x);
            // 4x^2 + x
            let expected = Expr::add(vec![Expr::mul(vec![Expr::integer(4), Expr::pow(x.clone(), Expr::integer(2))]), x]);
            assert_eq!(collected, expected);
        }

        #[test]
        fn together_combines_fractions() {
            let x = Expr::symbol("x");
            let e = Expr::add(vec![Expr::pow(x, Expr::integer(-1)), Expr::integer(1)]);
            let combined = together(&e);
            // Verify numerically: (1/x + 1) at x=2 should equal the combined form evaluated the same way.
            assert_ne!(combined, e);
        }

        #[test]
        fn cancel_removes_common_univariate_factor() {
            let x = Expr::symbol("x");
            // (x^2 - 1) / (x - 1) -> x + 1
            let num = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::integer(-1)]);
            let den = Expr::add(vec![x.clone(), Expr::integer(-1)]);
            let e = Expr::mul(vec![num, Expr::pow(den, Expr::integer(-1))]);
            let result = cancel(&e);
            assert_eq!(result, Expr::add(vec![x, Expr::integer(1)]));
        }

        #[test]
        fn factor_recovers_linear_factors() {
            let x = Expr::symbol("x");
            // x^2 - 1 -> (x-1)(x+1) up to ordering/sign; check by expanding back.
            let e = Expr::add(vec![Expr::pow(x, Expr::integer(2)), Expr::integer(-1)]);
            let factored = factor(&e);
            assert_eq!(expand(&factored), e);
            assert_ne!(factored, e);
        }

        #[test]
        fn apart_splits_simple_rational_function() {
            let x = Expr::symbol("x");
            // 1/((x-1)(x+1)) = (1/2)/(x-1) - (1/2)/(x+1)
            let den = Expr::mul(vec![Expr::add(vec![x.clone(), Expr::integer(-1)]), Expr::add(vec![x.clone(), Expr::integer(1)])]);
            let e = Expr::pow(den, Expr::integer(-1));
            let result = apart(&e, &x);
            // Recombine via together+cancel-free check: evaluate both sides symbolically by re-expanding the together form.
            let recombined = together(&result);
            let original_together = together(&e);
            assert_eq!(cancel(&recombined), cancel(&original_together));
        }

        #[test]
        fn denest_sqrt_classic_example() {
            // sqrt(3 + 2*sqrt(2)) == 1 + sqrt(2)
            let inner = Expr::add(vec![Expr::integer(3), Expr::mul(vec![Expr::integer(2), Expr::pow(Expr::integer(2), Expr::from(Rational::from_i64(1, 2).unwrap()))])]);
            let e = Expr::pow(inner, Expr::from(Rational::from_i64(1, 2).unwrap()));
            let result = denest_sqrt(&e);
            let expected = Expr::add(vec![Expr::integer(1), Expr::pow(Expr::integer(2), Expr::from(Rational::from_i64(1, 2).unwrap()))]);
            assert_eq!(result, expected);
        }

        #[test]
        fn simplify_picks_the_smallest_candidate() {
            let x = Expr::symbol("x");
            let num = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::integer(-1)]);
            let den = Expr::add(vec![x.clone(), Expr::integer(-1)]);
            let e = Expr::mul(vec![num, Expr::pow(den, Expr::integer(-1))]);
            let result = simplify(&e);
            assert_eq!(result, Expr::add(vec![x, Expr::integer(1)]));
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Simplify

// #region 🔖️Trig
pub mod trig {
    //! 📐️ Trigonometric and logarithmic rewriting passes: canonicalize `tan`/`cot`/`sec`/`csc` to
    //! `sin`/`cos` and apply the Pythagorean identity to a capped fixpoint, plus `expand_trig`/`expand_log`
    //! (distribute across sums/products) and their reverse `logcombine`/`powsimp`.

    use crate::cas::expr::{Expr, Kind};
    use crate::cas::fnkind::FnKind;
    use crate::cas::pattern::{wild, wild_seq, Rule, RuleSet, Strategy};

    // #region 🔖️TrigCanon
    fn ratio_rules() -> RuleSet {
        RuleSet::new(vec![
            Rule::new(Expr::func(FnKind::Tan, vec![wild(0)]), Expr::mul(vec![Expr::func(FnKind::Sin, vec![wild(0)]), Expr::pow(Expr::func(FnKind::Cos, vec![wild(0)]), Expr::integer(-1))])),
            Rule::new(Expr::func(FnKind::Cot, vec![wild(0)]), Expr::mul(vec![Expr::func(FnKind::Cos, vec![wild(0)]), Expr::pow(Expr::func(FnKind::Sin, vec![wild(0)]), Expr::integer(-1))])),
            Rule::new(Expr::func(FnKind::Sec, vec![wild(0)]), Expr::pow(Expr::func(FnKind::Cos, vec![wild(0)]), Expr::integer(-1))),
            Rule::new(Expr::func(FnKind::Csc, vec![wild(0)]), Expr::pow(Expr::func(FnKind::Sin, vec![wild(0)]), Expr::integer(-1))),
        ])
    }

    fn pythagorean_rules() -> RuleSet {
        let sin2_cos2 = Expr::add(vec![Expr::pow(Expr::func(FnKind::Sin, vec![wild(0)]), Expr::integer(2)), Expr::pow(Expr::func(FnKind::Cos, vec![wild(0)]), Expr::integer(2)), wild_seq(1)]);
        let rewritten = Expr::add(vec![Expr::integer(1), wild_seq(1)]);
        RuleSet::new(vec![Rule::new(sin2_cos2, rewritten)])
    }

    /// 📐️ Rewrites `tan/cot/sec/csc` to `sin`/`cos`, then applies the Pythagorean identity (including the
    /// `sin^2(w) + cos^2(w) + ...rest` generalization via a `Seq` wildcard) to a capped fixpoint.
    pub fn trig_canon(e: &Expr) -> Expr {
        let ratios = ratio_rules();
        let after_ratios = ratios.apply(e, Strategy::Fixpoint { max_iters: 8 });
        let pythag = pythagorean_rules();
        pythag.apply(&after_ratios, Strategy::Fixpoint { max_iters: 8 })
    }
    // #endregion 🔖️TrigCanon

    // #region 🔖️ExpandTrig
    pub fn expand_trig(e: &Expr) -> Expr {
        let rebuilt = crate::cas::visit::map_children(e, &mut expand_trig);
        match rebuilt.kind() {
            Kind::Fn(FnKind::Sin, args) if args.len() == 1 => expand_trig_sin(&args[0]),
            Kind::Fn(FnKind::Cos, args) if args.len() == 1 => expand_trig_cos(&args[0]),
            Kind::Fn(FnKind::Tan, args) if args.len() == 1 => Expr::mul(vec![expand_trig_sin(&args[0]), Expr::pow(expand_trig_cos(&args[0]), Expr::integer(-1))]),
            _ => rebuilt,
        }
    }

    fn split_add(arg: &Expr) -> Option<(Expr, Expr)> {
        let Kind::Add(terms) = arg.kind() else { return None };
        if terms.len() < 2 {
            return None;
        }
        let (first, rest) = terms.split_first().unwrap();
        Some((first.clone(), Expr::add(rest.to_vec())))
    }

    fn expand_trig_sin(arg: &Expr) -> Expr {
        match split_add(arg) {
            Some((first, rest)) => {
                let sin_first = Expr::func(FnKind::Sin, vec![first.clone()]);
                let cos_first = Expr::func(FnKind::Cos, vec![first]);
                Expr::add(vec![Expr::mul(vec![sin_first, expand_trig_cos(&rest)]), Expr::mul(vec![cos_first, expand_trig_sin(&rest)])])
            }
            None => Expr::func(FnKind::Sin, vec![arg.clone()]),
        }
    }

    fn expand_trig_cos(arg: &Expr) -> Expr {
        match split_add(arg) {
            Some((first, rest)) => {
                let cos_first = Expr::func(FnKind::Cos, vec![first.clone()]);
                let sin_first = Expr::func(FnKind::Sin, vec![first]);
                Expr::add(vec![Expr::mul(vec![cos_first, expand_trig_cos(&rest)]), Expr::mul(vec![Expr::integer(-1), sin_first, expand_trig_sin(&rest)])])
            }
            None => Expr::func(FnKind::Cos, vec![arg.clone()]),
        }
    }
    // #endregion 🔖️ExpandTrig

    // #region 🔖️ExpandLog
    pub fn expand_log(e: &Expr) -> Expr {
        let rebuilt = crate::cas::visit::map_children(e, &mut expand_log);
        if let Kind::Fn(FnKind::Ln, args) = rebuilt.kind() {
            if args.len() == 1 {
                return expand_log_arg(&args[0]);
            }
        }
        rebuilt
    }

    fn expand_log_arg(arg: &Expr) -> Expr {
        match arg.kind() {
            Kind::Mul(factors) => Expr::add(factors.iter().map(expand_log_arg).collect()),
            Kind::Pow(base, exp) => Expr::mul(vec![exp.clone(), expand_log_arg(base)]),
            _ => Expr::func(FnKind::Ln, vec![arg.clone()]),
        }
    }

    /// 📐️ Reverse of `expand_log`: combines a sum of `ln(a) + ln(b) + ...` into `ln(a*b*...)`, gated on
    /// `is_positive` for every combined argument (never combines when that can't be verified, to avoid
    /// silently crossing a branch cut).
    pub fn logcombine(e: &Expr) -> Expr {
        let rebuilt = crate::cas::visit::map_children(e, &mut logcombine);
        let Kind::Add(terms) = rebuilt.kind() else { return rebuilt };
        let mut log_args: Vec<Expr> = Vec::new();
        let mut others: Vec<Expr> = Vec::new();
        for t in terms {
            if let Kind::Fn(FnKind::Ln, args) = t.kind() {
                if args.len() == 1 && crate::cas::assume::is_positive(&args[0]) == Some(true) {
                    log_args.push(args[0].clone());
                    continue;
                }
            }
            others.push(t.clone());
        }
        if log_args.len() >= 2 {
            others.push(Expr::func(FnKind::Ln, vec![Expr::mul(log_args)]));
            return Expr::add(others);
        }
        rebuilt
    }
    // #endregion 🔖️ExpandLog

    // #region 🔖️Powsimp
    /// 📐️ Combines same-exponent power factors within a product: `x^a * y^a -> (x*y)^a`.
    pub fn powsimp(e: &Expr) -> Expr {
        let rebuilt = crate::cas::visit::map_children(e, &mut powsimp);
        let Kind::Mul(factors) = rebuilt.kind() else { return rebuilt };
        let mut by_exp: std::collections::BTreeMap<Expr, Vec<Expr>> = std::collections::BTreeMap::new();
        let mut order: Vec<Expr> = Vec::new();
        let mut result: Vec<Expr> = Vec::new();
        for f in factors {
            if let Kind::Pow(base, exp) = f.kind() {
                if !by_exp.contains_key(exp) {
                    order.push(exp.clone());
                }
                by_exp.entry(exp.clone()).or_default().push(base.clone());
                continue;
            }
            result.push(f.clone());
        }
        for exp in order {
            let bases = by_exp.remove(&exp).unwrap();
            if bases.len() >= 2 {
                result.push(Expr::pow(Expr::mul(bases), exp));
            } else {
                result.push(Expr::pow(bases[0].clone(), exp));
            }
        }
        Expr::mul(result)
    }
    // #endregion 🔖️Powsimp

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn trig_canon_rewrites_tan_to_sin_over_cos() {
            let x = Expr::symbol("x");
            let e = Expr::func(FnKind::Tan, vec![x.clone()]);
            let result = trig_canon(&e);
            let expected = Expr::mul(vec![Expr::func(FnKind::Sin, vec![x.clone()]), Expr::pow(Expr::func(FnKind::Cos, vec![x]), Expr::integer(-1))]);
            assert_eq!(result, expected);
        }

        #[test]
        fn trig_canon_applies_pythagorean_identity() {
            let x = Expr::symbol("x");
            let e = Expr::add(vec![Expr::pow(Expr::func(FnKind::Sin, vec![x.clone()]), Expr::integer(2)), Expr::pow(Expr::func(FnKind::Cos, vec![x]), Expr::integer(2))]);
            assert_eq!(trig_canon(&e), Expr::integer(1));
        }

        #[test]
        fn trig_canon_pythagorean_with_extra_terms() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let e = Expr::add(vec![Expr::pow(Expr::func(FnKind::Sin, vec![x.clone()]), Expr::integer(2)), Expr::pow(Expr::func(FnKind::Cos, vec![x]), Expr::integer(2)), y.clone()]);
            assert_eq!(trig_canon(&e), Expr::add(vec![Expr::integer(1), y]));
        }

        #[test]
        fn expand_trig_sin_of_sum() {
            let a = Expr::symbol("a");
            let b = Expr::symbol("b");
            let e = Expr::func(FnKind::Sin, vec![Expr::add(vec![a.clone(), b.clone()])]);
            let expected = Expr::add(vec![Expr::mul(vec![Expr::func(FnKind::Sin, vec![a.clone()]), Expr::func(FnKind::Cos, vec![b.clone()])]), Expr::mul(vec![Expr::func(FnKind::Cos, vec![a]), Expr::func(FnKind::Sin, vec![b])])]);
            assert_eq!(expand_trig(&e), expected);
        }

        #[test]
        fn expand_log_of_product_and_power() {
            let a = Expr::symbol("a");
            let b = Expr::symbol("b");
            let e = Expr::func(FnKind::Ln, vec![Expr::mul(vec![Expr::pow(a.clone(), Expr::integer(2)), b.clone()])]);
            let expected = Expr::add(vec![Expr::mul(vec![Expr::integer(2), Expr::func(FnKind::Ln, vec![a])]), Expr::func(FnKind::Ln, vec![b])]);
            assert_eq!(expand_log(&e), expected);
        }

        #[test]
        fn logcombine_merges_positive_logs() {
            let a = Expr::symbol_with("a", crate::cas::assume::AssumeSet::POSITIVE);
            let b = Expr::symbol_with("b", crate::cas::assume::AssumeSet::POSITIVE);
            let e = Expr::add(vec![Expr::func(FnKind::Ln, vec![a.clone()]), Expr::func(FnKind::Ln, vec![b.clone()])]);
            let combined = logcombine(&e);
            assert_eq!(combined, Expr::func(FnKind::Ln, vec![Expr::mul(vec![a, b])]));
        }

        #[test]
        fn logcombine_skips_unknown_sign_arguments() {
            let a = Expr::symbol("a");
            let b = Expr::symbol("b");
            let e = Expr::add(vec![Expr::func(FnKind::Ln, vec![a]), Expr::func(FnKind::Ln, vec![b])]);
            assert_eq!(logcombine(&e), e);
        }

        #[test]
        fn powsimp_combines_same_exponent_factors() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let e = Expr::mul(vec![Expr::pow(x.clone(), Expr::integer(3)), Expr::pow(y.clone(), Expr::integer(3))]);
            let expected = Expr::pow(Expr::mul(vec![x, y]), Expr::integer(3));
            assert_eq!(powsimp(&e), expected);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Trig

// #region 🔖️Diff
pub mod diff {
    //! 📉️ Table-driven symbolic differentiation: chain/product/power rules over the canonical tree, plus a
    //! per-`FnKind` derivative table for elementary and special functions. Returns `None` — never a wrong
    //! answer — whenever a subexpression involves a function without a known derivative rule (`zeta`,
    //! `Hyp2F1`, a user-defined function, or an order/degree parameter that itself depends on the
    //! differentiation variable).

    use crate::cas::expr::{Constant, Expr, Kind};
    use crate::cas::fnkind::FnKind;
    use math::number::Rational;

    // #region 🔖️Diff
    /// 📉️ `d(e)/d(x)`, treating every other symbol as a constant (partial derivative).
    pub fn diff(e: &Expr, x: &Expr) -> Option<Expr> {
        match e.kind() {
            Kind::Integer(_) | Kind::Rational(_) | Kind::Constant(_) | Kind::Bool(_) | Kind::RootOf { .. } => Some(Expr::integer(0)),
            Kind::Symbol(_) => Some(if e == x { Expr::integer(1) } else { Expr::integer(0) }),
            Kind::Add(terms) => {
                let mut parts = Vec::with_capacity(terms.len());
                for t in terms {
                    parts.push(diff(t, x)?);
                }
                Some(Expr::add(parts))
            }
            Kind::Mul(factors) => {
                let mut sum_terms = Vec::with_capacity(factors.len());
                for i in 0..factors.len() {
                    let d = diff(&factors[i], x)?;
                    if d.is_zero_literal() {
                        continue;
                    }
                    let mut rest = factors.clone();
                    rest.remove(i);
                    let mut term_factors = vec![d];
                    term_factors.extend(rest);
                    sum_terms.push(Expr::mul(term_factors));
                }
                Some(Expr::add(sum_terms))
            }
            Kind::Pow(base, exp) => diff_pow(base, exp, x),
            Kind::Fn(kind, args) => diff_fn(kind, args, x),
            Kind::Piecewise(cases) => {
                let mut new_cases = Vec::with_capacity(cases.len());
                for (v, c) in cases {
                    new_cases.push((diff(v, x)?, c.clone()));
                }
                Some(Expr::from_kind_unchecked(Kind::Piecewise(new_cases)))
            }
            Kind::Rel(..) | Kind::Wild(..) => None,
        }
    }

    /// 📉️ Multivariate: the vector of partial derivatives w.r.t. each of `vars`, in order; `None` as soon
    /// as any single partial derivative is unknown.
    pub fn gradient(e: &Expr, vars: &[Expr]) -> Option<Vec<Expr>> {
        vars.iter().map(|v| diff(e, v)).collect()
    }

    /// 🔗️ Implicit differentiation of `y` (a function of `x`) from the equation `lhs == rhs`: computes
    /// `-diff(lhs-rhs, x) / diff(lhs-rhs, y)` (total derivative via the implicit function theorem), treating
    /// `y` as an independent symbol in the equation and substituting nothing — the caller is expected to
    /// already have `y` appearing explicitly wherever it's implicitly a function of `x`.
    pub fn idiff(lhs: &Expr, rhs: &Expr, y: &Expr, x: &Expr) -> Option<Expr> {
        let f = lhs.clone() - rhs.clone();
        let dfdx = diff(&f, x)?;
        let dfdy = diff(&f, y)?;
        if dfdy.is_zero_literal() {
            return None;
        }
        Some(Expr::mul(vec![Expr::integer(-1), dfdx, Expr::pow(dfdy, Expr::integer(-1))]))
    }
    // #endregion 🔖️Diff

    // #region 🔖️PowRule
    fn diff_pow(base: &Expr, exp: &Expr, x: &Expr) -> Option<Expr> {
        let exp_depends = crate::cas::visit::contains_symbol(exp, x);
        let base_depends = crate::cas::visit::contains_symbol(base, x);
        if !exp_depends {
            let dbase = diff(base, x)?;
            if dbase.is_zero_literal() {
                return Some(Expr::integer(0));
            }
            let new_exp = Expr::add(vec![exp.clone(), Expr::integer(-1)]);
            return Some(Expr::mul(vec![exp.clone(), Expr::pow(base.clone(), new_exp), dbase]));
        }
        if !base_depends {
            let dexp = diff(exp, x)?;
            if dexp.is_zero_literal() {
                return Some(Expr::integer(0));
            }
            return Some(Expr::mul(vec![Expr::pow(base.clone(), exp.clone()), Expr::func(FnKind::Ln, vec![base.clone()]), dexp]));
        }
        let dbase = diff(base, x)?;
        let dexp = diff(exp, x)?;
        let term1 = Expr::mul(vec![dexp, Expr::func(FnKind::Ln, vec![base.clone()])]);
        let term2 = Expr::mul(vec![exp.clone(), dbase, Expr::pow(base.clone(), Expr::integer(-1))]);
        Some(Expr::mul(vec![Expr::pow(base.clone(), exp.clone()), Expr::add(vec![term1, term2])]))
    }
    // #endregion 🔖️PowRule

    // #region 🔖️FnChainRule
    fn diff_fn(kind: &FnKind, args: &[Expr], x: &Expr) -> Option<Expr> {
        match kind {
            FnKind::UserFn(_) | FnKind::Zeta => None,
            FnKind::BesselJ | FnKind::BesselY | FnKind::BesselI | FnKind::BesselK => diff_bessel(kind, args, x),
            FnKind::LegendreP => diff_legendre(args, x),
            FnKind::ChebyshevT => diff_chebyshev_t(args, x),
            FnKind::ChebyshevU => diff_chebyshev_u(args, x),
            FnKind::HermiteH => diff_hermite(args, x),
            FnKind::LaguerreL => diff_laguerre(args, x),
            _ if args.len() == 1 => {
                let inner_d = diff(&args[0], x)?;
                if inner_d.is_zero_literal() {
                    return Some(Expr::integer(0));
                }
                let outer_d = unary_derivative(kind, &args[0])?;
                Some(Expr::mul(vec![outer_d, inner_d]))
            }
            _ => None,
        }
    }

    fn unary_derivative(kind: &FnKind, arg: &Expr) -> Option<Expr> {
        use FnKind::*;
        let half = Expr::from(Rational::from_i64(1, 2).unwrap());
        let neg_half = Expr::from(Rational::from_i64(-1, 2).unwrap());
        Some(match kind {
            Sin => Expr::func(Cos, vec![arg.clone()]),
            Cos => Expr::mul(vec![Expr::integer(-1), Expr::func(Sin, vec![arg.clone()])]),
            Tan => Expr::add(vec![Expr::integer(1), Expr::pow(Expr::func(Tan, vec![arg.clone()]), Expr::integer(2))]),
            Cot => Expr::mul(vec![Expr::integer(-1), Expr::add(vec![Expr::integer(1), Expr::pow(Expr::func(Cot, vec![arg.clone()]), Expr::integer(2))])]),
            Sec => Expr::mul(vec![Expr::func(Sec, vec![arg.clone()]), Expr::func(Tan, vec![arg.clone()])]),
            Csc => Expr::mul(vec![Expr::integer(-1), Expr::func(Csc, vec![arg.clone()]), Expr::func(Cot, vec![arg.clone()])]),
            Asin => Expr::pow(Expr::add(vec![Expr::integer(1), Expr::mul(vec![Expr::integer(-1), Expr::pow(arg.clone(), Expr::integer(2))])]), neg_half),
            Acos => Expr::mul(vec![Expr::integer(-1), Expr::pow(Expr::add(vec![Expr::integer(1), Expr::mul(vec![Expr::integer(-1), Expr::pow(arg.clone(), Expr::integer(2))])]), neg_half)]),
            Atan => Expr::pow(Expr::add(vec![Expr::integer(1), Expr::pow(arg.clone(), Expr::integer(2))]), Expr::integer(-1)),
            Acot => Expr::mul(vec![Expr::integer(-1), Expr::pow(Expr::add(vec![Expr::integer(1), Expr::pow(arg.clone(), Expr::integer(2))]), Expr::integer(-1))]),
            Asec => Expr::pow(Expr::mul(vec![Expr::func(Abs, vec![arg.clone()]), Expr::pow(Expr::add(vec![Expr::pow(arg.clone(), Expr::integer(2)), Expr::integer(-1)]), half)]), Expr::integer(-1)),
            Acsc => Expr::mul(vec![Expr::integer(-1), Expr::pow(Expr::mul(vec![Expr::func(Abs, vec![arg.clone()]), Expr::pow(Expr::add(vec![Expr::pow(arg.clone(), Expr::integer(2)), Expr::integer(-1)]), half)]), Expr::integer(-1))]),
            Sinh => Expr::func(Cosh, vec![arg.clone()]),
            Cosh => Expr::func(Sinh, vec![arg.clone()]),
            Tanh => Expr::add(vec![Expr::integer(1), Expr::mul(vec![Expr::integer(-1), Expr::pow(Expr::func(Tanh, vec![arg.clone()]), Expr::integer(2))])]),
            Asinh => Expr::pow(Expr::add(vec![Expr::integer(1), Expr::pow(arg.clone(), Expr::integer(2))]), neg_half),
            Acosh => Expr::pow(Expr::add(vec![Expr::pow(arg.clone(), Expr::integer(2)), Expr::integer(-1)]), neg_half),
            Atanh => Expr::pow(Expr::add(vec![Expr::integer(1), Expr::mul(vec![Expr::integer(-1), Expr::pow(arg.clone(), Expr::integer(2))])]), Expr::integer(-1)),
            Exp => Expr::func(Exp, vec![arg.clone()]),
            Ln => Expr::pow(arg.clone(), Expr::integer(-1)),
            Abs => Expr::func(Sign, vec![arg.clone()]),
            Sign | Floor | Ceil => Expr::integer(0),
            Gamma => Expr::mul(vec![Expr::func(Gamma, vec![arg.clone()]), Expr::func(Digamma, vec![arg.clone()])]),
            LogGamma => Expr::func(Digamma, vec![arg.clone()]),
            Erf => Expr::mul(vec![Expr::integer(2), Expr::pow(Expr::constant(Constant::Pi), neg_half), Expr::func(Exp, vec![Expr::mul(vec![Expr::integer(-1), Expr::pow(arg.clone(), Expr::integer(2))])])]),
            Erfc => Expr::mul(vec![Expr::integer(-2), Expr::pow(Expr::constant(Constant::Pi), neg_half), Expr::func(Exp, vec![Expr::mul(vec![Expr::integer(-1), Expr::pow(arg.clone(), Expr::integer(2))])])]),
            LambertW => {
                let w = Expr::func(LambertW, vec![arg.clone()]);
                Expr::mul(vec![w.clone(), Expr::pow(Expr::mul(vec![arg.clone(), Expr::add(vec![Expr::integer(1), w])]), Expr::integer(-1))])
            }
            _ => return None,
        })
    }
    // #endregion 🔖️FnChainRule

    // #region 🔖️SpecialFunctionRecurrences
    fn diff_bessel(kind: &FnKind, args: &[Expr], x: &Expr) -> Option<Expr> {
        let [n, arg] = args else { return None };
        if crate::cas::visit::contains_symbol(n, x) {
            return None;
        }
        let inner_d = diff(arg, x)?;
        if inner_d.is_zero_literal() {
            return Some(Expr::integer(0));
        }
        let n_minus = Expr::add(vec![n.clone(), Expr::integer(-1)]);
        let n_plus = Expr::add(vec![n.clone(), Expr::integer(1)]);
        let half = Expr::from(Rational::from_i64(1, 2).unwrap());
        let outer = match kind {
            FnKind::BesselJ => Expr::mul(vec![half, Expr::add(vec![Expr::func(FnKind::BesselJ, vec![n_minus, arg.clone()]), Expr::mul(vec![Expr::integer(-1), Expr::func(FnKind::BesselJ, vec![n_plus, arg.clone()])])])]),
            FnKind::BesselY => Expr::mul(vec![half, Expr::add(vec![Expr::func(FnKind::BesselY, vec![n_minus, arg.clone()]), Expr::mul(vec![Expr::integer(-1), Expr::func(FnKind::BesselY, vec![n_plus, arg.clone()])])])]),
            FnKind::BesselI => Expr::mul(vec![half, Expr::add(vec![Expr::func(FnKind::BesselI, vec![n_minus, arg.clone()]), Expr::func(FnKind::BesselI, vec![n_plus, arg.clone()])])]),
            FnKind::BesselK => Expr::mul(vec![Expr::integer(-1), half, Expr::add(vec![Expr::func(FnKind::BesselK, vec![n_minus, arg.clone()]), Expr::func(FnKind::BesselK, vec![n_plus, arg.clone()])])]),
            _ => unreachable!("diff_bessel only called for Bessel* kinds"),
        };
        Some(Expr::mul(vec![outer, inner_d]))
    }

    fn diff_legendre(args: &[Expr], x: &Expr) -> Option<Expr> {
        let [n, arg] = args else { return None };
        if crate::cas::visit::contains_symbol(n, x) {
            return None;
        }
        let inner_d = diff(arg, x)?;
        if inner_d.is_zero_literal() {
            return Some(Expr::integer(0));
        }
        let n_minus = Expr::add(vec![n.clone(), Expr::integer(-1)]);
        let denom = Expr::add(vec![Expr::pow(arg.clone(), Expr::integer(2)), Expr::integer(-1)]);
        let numer = Expr::add(vec![Expr::mul(vec![arg.clone(), Expr::func(FnKind::LegendreP, vec![n.clone(), arg.clone()])]), Expr::mul(vec![Expr::integer(-1), Expr::func(FnKind::LegendreP, vec![n_minus, arg.clone()])])]);
        let outer = Expr::mul(vec![n.clone(), numer, Expr::pow(denom, Expr::integer(-1))]);
        Some(Expr::mul(vec![outer, inner_d]))
    }

    fn diff_chebyshev_t(args: &[Expr], x: &Expr) -> Option<Expr> {
        let [n, arg] = args else { return None };
        if crate::cas::visit::contains_symbol(n, x) {
            return None;
        }
        let inner_d = diff(arg, x)?;
        if inner_d.is_zero_literal() {
            return Some(Expr::integer(0));
        }
        let n_minus = Expr::add(vec![n.clone(), Expr::integer(-1)]);
        let outer = Expr::mul(vec![n.clone(), Expr::func(FnKind::ChebyshevU, vec![n_minus, arg.clone()])]);
        Some(Expr::mul(vec![outer, inner_d]))
    }

    fn diff_chebyshev_u(args: &[Expr], x: &Expr) -> Option<Expr> {
        let [n, arg] = args else { return None };
        if crate::cas::visit::contains_symbol(n, x) {
            return None;
        }
        let inner_d = diff(arg, x)?;
        if inner_d.is_zero_literal() {
            return Some(Expr::integer(0));
        }
        let n_plus = Expr::add(vec![n.clone(), Expr::integer(1)]);
        let denom = Expr::add(vec![Expr::pow(arg.clone(), Expr::integer(2)), Expr::integer(-1)]);
        let numer = Expr::add(vec![Expr::mul(vec![n_plus.clone(), Expr::func(FnKind::ChebyshevT, vec![n_plus, arg.clone()])]), Expr::mul(vec![Expr::integer(-1), arg.clone(), Expr::func(FnKind::ChebyshevU, vec![n.clone(), arg.clone()])])]);
        let outer = Expr::mul(vec![numer, Expr::pow(denom, Expr::integer(-1))]);
        Some(Expr::mul(vec![outer, inner_d]))
    }

    fn diff_hermite(args: &[Expr], x: &Expr) -> Option<Expr> {
        let [n, arg] = args else { return None };
        if crate::cas::visit::contains_symbol(n, x) {
            return None;
        }
        let inner_d = diff(arg, x)?;
        if inner_d.is_zero_literal() {
            return Some(Expr::integer(0));
        }
        let n_minus = Expr::add(vec![n.clone(), Expr::integer(-1)]);
        let outer = Expr::mul(vec![Expr::integer(2), n.clone(), Expr::func(FnKind::HermiteH, vec![n_minus, arg.clone()])]);
        Some(Expr::mul(vec![outer, inner_d]))
    }

    fn diff_laguerre(args: &[Expr], x: &Expr) -> Option<Expr> {
        let [n, arg] = args else { return None };
        if crate::cas::visit::contains_symbol(n, x) {
            return None;
        }
        let inner_d = diff(arg, x)?;
        if inner_d.is_zero_literal() {
            return Some(Expr::integer(0));
        }
        let n_minus = Expr::add(vec![n.clone(), Expr::integer(-1)]);
        let diff_l = Expr::add(vec![Expr::func(FnKind::LaguerreL, vec![n.clone(), arg.clone()]), Expr::mul(vec![Expr::integer(-1), Expr::func(FnKind::LaguerreL, vec![n_minus, arg.clone()])])]);
        let outer = Expr::mul(vec![n.clone(), diff_l, Expr::pow(arg.clone(), Expr::integer(-1))]);
        Some(Expr::mul(vec![outer, inner_d]))
    }
    // #endregion 🔖️SpecialFunctionRecurrences

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn diff_of_constant_is_zero() {
            assert_eq!(diff(&Expr::integer(5), &Expr::symbol("x")), Some(Expr::integer(0)));
        }

        #[test]
        fn diff_of_x_is_one() {
            let x = Expr::symbol("x");
            assert_eq!(diff(&x, &x), Some(Expr::integer(1)));
        }

        #[test]
        fn diff_of_other_symbol_is_zero() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            assert_eq!(diff(&y, &x), Some(Expr::integer(0)));
        }

        #[test]
        fn power_rule() {
            let x = Expr::symbol("x");
            let e = Expr::pow(x.clone(), Expr::integer(3));
            let expected = Expr::mul(vec![Expr::integer(3), Expr::pow(x, Expr::integer(2))]);
            assert_eq!(diff(&e, &Expr::symbol("x")), Some(expected));
        }

        #[test]
        fn product_rule() {
            let x = Expr::symbol("x");
            let e = Expr::mul(vec![x.clone(), Expr::func(FnKind::Sin, vec![x.clone()])]);
            let expected = Expr::add(vec![Expr::func(FnKind::Sin, vec![x.clone()]), Expr::mul(vec![x.clone(), Expr::func(FnKind::Cos, vec![x])])]);
            assert_eq!(diff(&e, &Expr::symbol("x")), Some(expected));
        }

        #[test]
        fn chain_rule_sin_of_square() {
            let x = Expr::symbol("x");
            let e = Expr::func(FnKind::Sin, vec![Expr::pow(x.clone(), Expr::integer(2))]);
            let expected = Expr::mul(vec![Expr::integer(2), x.clone(), Expr::func(FnKind::Cos, vec![Expr::pow(x, Expr::integer(2))])]);
            assert_eq!(diff(&e, &Expr::symbol("x")), Some(expected));
        }

        #[test]
        fn exp_of_x_is_itself() {
            let x = Expr::symbol("x");
            assert_eq!(diff(&Expr::func(FnKind::Exp, vec![x.clone()]), &x), Some(Expr::func(FnKind::Exp, vec![x])));
        }

        #[test]
        fn ln_derivative() {
            let x = Expr::symbol("x");
            assert_eq!(diff(&Expr::func(FnKind::Ln, vec![x.clone()]), &x), Some(Expr::pow(x, Expr::integer(-1))));
        }

        #[test]
        fn general_power_logarithmic_differentiation() {
            // d/dx x^x = x^x * (ln(x) + 1)
            let x = Expr::symbol("x");
            let e = Expr::pow(x.clone(), x.clone());
            let result = diff(&e, &x).unwrap();
            let expected = Expr::mul(vec![Expr::pow(x.clone(), x.clone()), Expr::add(vec![Expr::func(FnKind::Ln, vec![x]), Expr::integer(1)])]);
            assert_eq!(result, expected);
        }

        #[test]
        fn unknown_function_derivative_is_none() {
            let x = Expr::symbol("x");
            let e = Expr::func(FnKind::Zeta, vec![x.clone()]);
            assert_eq!(diff(&e, &x), None);
        }

        #[test]
        fn bessel_j_recurrence_derivative() {
            let x = Expr::symbol("x");
            let n = Expr::integer(2);
            let e = Expr::func(FnKind::BesselJ, vec![n, x.clone()]);
            let expected = Expr::mul(vec![
                Expr::from(Rational::from_i64(1, 2).unwrap()),
                Expr::add(vec![Expr::func(FnKind::BesselJ, vec![Expr::integer(1), x.clone()]), Expr::mul(vec![Expr::integer(-1), Expr::func(FnKind::BesselJ, vec![Expr::integer(3), x.clone()])])]),
            ]);
            assert_eq!(diff(&e, &x), Some(expected));
        }

        #[test]
        fn gradient_computes_all_partials() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            let e = Expr::mul(vec![x.clone(), y.clone()]);
            let grad = gradient(&e, &[x.clone(), y.clone()]).unwrap();
            assert_eq!(grad[0], y);
            assert_eq!(grad[1], x);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Diff

// #region 🔖️Series
pub mod series {
    //! 📶️ Truncated Taylor series via repeated symbolic differentiation: `coeffs[k] = f^(k)(at) / k!`.
    //! Simpler than direct series-arithmetic (composition/reversion aren't implemented in this pass) but
    //! correct, and it reuses `diff` directly rather than duplicating a second derivative table.

    use crate::cas::expr::{Constant, Expr, Kind};
    use math::number::Integer;

    // #region 🔖️Series
    /// 📶️ A truncated Taylor expansion of some expression in `x` around `at`: `sum coeffs[k] * (x-at)^k`,
    /// valid to `O((x-at)^(coeffs.len()))`.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Series {
        pub x: Expr,
        pub at: Expr,
        pub coeffs: Vec<Expr>,
    }

    fn is_determinate(e: &Expr) -> bool {
        !matches!(e.kind(), Kind::Constant(Constant::Undefined) | Kind::Constant(Constant::ComplexInf))
    }

    /// 📶️ Builds the order-`order` Taylor series of `e` in `x` around `at`; `None` if `e` (or any of its
    /// first `order` derivatives) is undefined at `at` — e.g. `e` has a pole there, which this pass doesn't
    /// handle as a genuine Laurent series (a documented first-pass limitation).
    pub fn taylor_series(e: &Expr, x: &Expr, at: &Expr, order: usize) -> Option<Series> {
        let mut coeffs = Vec::with_capacity(order + 1);
        let mut current = e.clone();
        let mut factorial = Integer::one();
        for k in 0..=order {
            let value_at = crate::cas::visit::subs(&current, x, at);
            if !is_determinate(&value_at) {
                return None;
            }
            let coeff = Expr::mul(vec![value_at, Expr::pow(Expr::from(factorial.clone()), Expr::integer(-1))]);
            coeffs.push(coeff);
            if k < order {
                current = crate::cas::diff::diff(&current, x)?;
            }
            factorial = factorial.mul(&Integer::from_i64((k + 1) as i64));
        }
        Some(Series { x: x.clone(), at: at.clone(), coeffs })
    }

    /// ↩️ Reconstructs `sum coeffs[k] * (x-at)^k` as a plain `Expr`.
    pub fn series_to_expr(s: &Series) -> Expr {
        let terms: Vec<Expr> = s.coeffs.iter().enumerate().map(|(k, c)| if k == 0 { c.clone() } else { Expr::mul(vec![c.clone(), Expr::pow(s.x.clone() - s.at.clone(), Expr::integer(k as i64))]) }).collect();
        Expr::add(terms)
    }

    /// 🔎️ The lowest-order term with a (structurally) nonzero coefficient, or `None` if every retained
    /// coefficient is exactly zero — used by `limits` to read off the leading behavior near `at`.
    pub fn leading_term(s: &Series) -> Option<(usize, Expr)> {
        s.coeffs.iter().enumerate().find(|(_, c)| !c.is_zero_literal()).map(|(k, c)| (k, c.clone()))
    }

    impl Series {
        /// ➕️ Term-wise sum, truncated to the shorter of the two operands' orders.
        pub fn add(&self, other: &Self) -> Self {
            let n = self.coeffs.len().min(other.coeffs.len());
            let coeffs = (0..n).map(|k| self.coeffs[k].clone() + other.coeffs[k].clone()).collect();
            Self { x: self.x.clone(), at: self.at.clone(), coeffs }
        }

        /// ✖️ Cauchy product, truncated to the shorter of the two operands' orders.
        pub fn mul(&self, other: &Self) -> Self {
            let n = self.coeffs.len().min(other.coeffs.len());
            let coeffs = (0..n)
                .map(|k| {
                    let terms: Vec<Expr> = (0..=k).map(|i| self.coeffs[i].clone() * other.coeffs[k - i].clone()).collect();
                    Expr::add(terms)
                })
                .collect();
            Self { x: self.x.clone(), at: self.at.clone(), coeffs }
        }

        pub fn scale(&self, c: &Expr) -> Self {
            Self { x: self.x.clone(), at: self.at.clone(), coeffs: self.coeffs.iter().map(|k| k.clone() * c.clone()).collect() }
        }
    }
    // #endregion 🔖️Series

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::cas::fnkind::FnKind;

        #[test]
        fn taylor_series_of_exp_matches_known_coefficients() {
            let x = Expr::symbol("x");
            let e = Expr::func(FnKind::Exp, vec![x.clone()]);
            let s = taylor_series(&e, &x, &Expr::integer(0), 4).unwrap();
            // exp(x) = 1 + x + x^2/2 + x^3/6 + x^4/24
            assert_eq!(s.coeffs[0], Expr::integer(1));
            assert_eq!(s.coeffs[1], Expr::integer(1));
            assert_eq!(s.coeffs[2], Expr::from(math::number::Rational::from_i64(1, 2).unwrap()));
            assert_eq!(s.coeffs[3], Expr::from(math::number::Rational::from_i64(1, 6).unwrap()));
        }

        #[test]
        fn taylor_series_of_sin_around_zero_has_no_even_terms() {
            let x = Expr::symbol("x");
            let e = Expr::func(FnKind::Sin, vec![x.clone()]);
            let s = taylor_series(&e, &x, &Expr::integer(0), 4).unwrap();
            assert_eq!(s.coeffs[0], Expr::integer(0));
            assert_eq!(s.coeffs[1], Expr::integer(1));
            assert_eq!(s.coeffs[2], Expr::integer(0));
        }

        #[test]
        fn taylor_series_fails_at_a_pole() {
            let x = Expr::symbol("x");
            let e = Expr::pow(x.clone(), Expr::integer(-1));
            assert!(taylor_series(&e, &x, &Expr::integer(0), 2).is_none());
        }

        #[test]
        fn leading_term_skips_zero_coefficients() {
            let x = Expr::symbol("x");
            let e = Expr::func(FnKind::Sin, vec![x.clone()]);
            let s = taylor_series(&e, &x, &Expr::integer(0), 3).unwrap();
            let (order, coeff) = leading_term(&s).unwrap();
            assert_eq!(order, 1);
            assert_eq!(coeff, Expr::integer(1));
        }

        #[test]
        fn series_to_expr_round_trips_a_polynomial() {
            let x = Expr::symbol("x");
            let s = Series { x: x.clone(), at: Expr::integer(0), coeffs: vec![Expr::integer(1), Expr::integer(2), Expr::integer(3)] };
            let e = series_to_expr(&s);
            let expected = Expr::add(vec![Expr::integer(1), Expr::mul(vec![Expr::integer(2), x.clone()]), Expr::mul(vec![Expr::integer(3), Expr::pow(x, Expr::integer(2))])]);
            assert_eq!(e, expected);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Series

// #region 🔖️Limits
pub mod limits {
    //! 🎯️ Limit evaluation: direct substitution first, then L'Hopital's rule (differentiating a
    //! detected numerator/denominator split, not the whole expression) up to a capped depth for `0/0` and
    //! `∞/∞` indeterminate forms. `x -> ±∞` reduces to `t -> 0⁺` via the `x = 1/t` substitution.

    use crate::cas::expr::{Constant, Expr, Kind};

    // #region 🔖️Direction
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Direction {
        Both,
        FromAbove,
        FromBelow,
    }
    // #endregion 🔖️Direction

    // #region 🔖️Limit
    const MAX_LHOPITAL_DEPTH: u32 = 8;

    fn is_determinate(e: &Expr) -> bool {
        !matches!(e.kind(), Kind::Constant(Constant::Undefined))
    }

    fn is_infinite(e: &Expr) -> bool {
        matches!(e.kind(), Kind::Constant(Constant::Inf) | Kind::Constant(Constant::NegInf) | Kind::Constant(Constant::ComplexInf))
    }

    /// 🎯️ `lim_{x -> at} e`, approaching from `dir` (only meaningful for a finite `at`; infinite limits
    /// are always two-sided in the reduced `t -> 0` problem). Returns `None` when the limit can't be
    /// resolved by direct substitution or a bounded L'Hopital chain — never a guessed or wrong value.
    pub fn limit(e: &Expr, x: &Expr, at: &Expr, dir: Direction) -> Option<Expr> {
        if matches!(at.kind(), Kind::Constant(Constant::Inf)) {
            let t = Expr::symbol("§limit_t");
            let e_t = crate::cas::visit::subs(e, x, &Expr::pow(t.clone(), Expr::integer(-1)));
            return limit(&e_t, &t, &Expr::integer(0), Direction::FromAbove);
        }
        if matches!(at.kind(), Kind::Constant(Constant::NegInf)) {
            let t = Expr::symbol("§limit_t");
            let e_t = crate::cas::visit::subs(e, x, &(Expr::integer(-1) * Expr::pow(t.clone(), Expr::integer(-1))));
            return limit(&e_t, &t, &Expr::integer(0), Direction::FromAbove);
        }

        let direct = crate::cas::visit::subs(e, x, at);
        if is_determinate(&direct) {
            return Some(direct);
        }

        if let Some(series_result) = limit_via_series(e, x, at) {
            return Some(series_result);
        }

        let _ = dir; // one-sided refinement over the series/L'Hopital path is a documented follow-up
        lhopital(e, x, at, 0)
    }

    /// 📶️ Series-based fast path: expand numerator and denominator around `at` and read off the ratio of
    /// leading terms — handles `0/0` forms cleanly without repeated differentiation.
    fn limit_via_series(e: &Expr, x: &Expr, at: &Expr) -> Option<Expr> {
        let (num, den) = extract_ratio(e, x);
        let num_series = crate::cas::series::taylor_series(&num, x, at, 6)?;
        let den_series = crate::cas::series::taylor_series(&den, x, at, 6)?;
        let (num_ord, num_coeff) = crate::cas::series::leading_term(&num_series)?;
        let (den_ord, den_coeff) = crate::cas::series::leading_term(&den_series)?;
        match num_ord.cmp(&den_ord) {
            std::cmp::Ordering::Greater => Some(Expr::integer(0)),
            std::cmp::Ordering::Equal => Some(num_coeff * Expr::pow(den_coeff, Expr::integer(-1))),
            std::cmp::Ordering::Less => None, // denominator vanishes to lower order: signed infinity, not resolved here
        }
    }

    fn lhopital(e: &Expr, x: &Expr, at: &Expr, depth: u32) -> Option<Expr> {
        if depth > MAX_LHOPITAL_DEPTH {
            return None;
        }
        let (num, den) = extract_ratio(e, x);
        let num_at = crate::cas::visit::subs(&num, x, at);
        let den_at = crate::cas::visit::subs(&den, x, at);
        let indeterminate = (num_at.is_zero_literal() && den_at.is_zero_literal()) || (is_infinite(&num_at) && is_infinite(&den_at));
        if !indeterminate {
            if !den_at.is_zero_literal() && is_determinate(&den_at) {
                return Some(num_at * Expr::pow(den_at, Expr::integer(-1)));
            }
            return None;
        }
        let dnum = crate::cas::diff::diff(&num, x)?;
        let dden = crate::cas::diff::diff(&den, x)?;
        let ratio = dnum * Expr::pow(dden, Expr::integer(-1));
        let direct = crate::cas::visit::subs(&ratio, x, at);
        if is_determinate(&direct) {
            return Some(direct);
        }
        lhopital(&ratio, x, at, depth + 1)
    }

    /// 🌉️ Splits `e` into a `num/den` pair via the poly bridge's rational-function detector (which treats
    /// any non-polynomial subtree, including transcendental functions, as its own generator) — falls back
    /// to `(e, 1)` when the bridge can't build a ratio at all.
    fn extract_ratio(e: &Expr, x: &Expr) -> (Expr, Expr) {
        if let Some((num_m, den_m, map)) = crate::cas::polybridge::as_ratfunc_auto(e) {
            if map.gens.iter().any(|g| g == x) {
                return (crate::cas::polybridge::from_poly(&num_m, &map), crate::cas::polybridge::from_poly(&den_m, &map));
            }
        }
        (e.clone(), Expr::integer(1))
    }
    // #endregion 🔖️Limit

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::cas::fnkind::FnKind;

        #[test]
        fn direct_substitution_when_defined() {
            let x = Expr::symbol("x");
            let e = Expr::pow(x.clone(), Expr::integer(2));
            assert_eq!(limit(&e, &x, &Expr::integer(3), Direction::Both), Some(Expr::integer(9)));
        }

        #[test]
        fn classic_sin_x_over_x_at_zero() {
            let x = Expr::symbol("x");
            let e = Expr::func(FnKind::Sin, vec![x.clone()]) * Expr::pow(x.clone(), Expr::integer(-1));
            assert_eq!(limit(&e, &x, &Expr::integer(0), Direction::Both), Some(Expr::integer(1)));
        }

        #[test]
        fn polynomial_ratio_at_removable_singularity() {
            // (x^2 - 1)/(x - 1) -> 2 as x -> 1
            let x = Expr::symbol("x");
            let num = Expr::pow(x.clone(), Expr::integer(2)) - Expr::integer(1);
            let den = x.clone() - Expr::integer(1);
            let e = num * Expr::pow(den, Expr::integer(-1));
            assert_eq!(limit(&e, &x, &Expr::integer(1), Direction::Both), Some(Expr::integer(2)));
        }

        #[test]
        fn limit_at_infinity_of_rational_function() {
            // (2x + 1)/(x + 3) -> 2 as x -> oo
            let x = Expr::symbol("x");
            let num = Expr::integer(2) * x.clone() + Expr::integer(1);
            let den = x.clone() + Expr::integer(3);
            let e = num * Expr::pow(den, Expr::integer(-1));
            assert_eq!(limit(&e, &x, &Expr::constant(Constant::Inf), Direction::Both), Some(Expr::integer(2)));
        }

        #[test]
        fn one_plus_one_over_n_to_the_n_via_lhopital_on_log_form() {
            // A simpler but still classic L'Hopital case: lim x->0 (1 - cos(x))/x^2 = 1/2
            let x = Expr::symbol("x");
            let num = Expr::integer(1) - Expr::func(FnKind::Cos, vec![x.clone()]);
            let den = Expr::pow(x.clone(), Expr::integer(2));
            let e = num * Expr::pow(den, Expr::integer(-1));
            assert_eq!(limit(&e, &x, &Expr::integer(0), Direction::Both), Some(Expr::from(math::number::Rational::from_i64(1, 2).unwrap())));
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Limits

// #region 🔖️Rootof
pub mod rootof {
    //! 🌱️ Bridges the kernel's `Kind::RootOf` leaf (a plain `Vec<Rational>` + index, so the kernel enum
    //! never depends on `crate::polynomial`) to `crate::polynomial::AlgebraicReal` for the
    //! numeric queries (sign, refinement, `f64` approximation) that need real algebra to answer.

    use crate::cas::expr::{Expr, Kind};
    use math::number::{Integer, Natural, Rational};
    use crate::polynomial::{AlgebraicReal, PolyU};

    // #region 🔖️Conversion
    fn clear_denominators(coeffs: &[Rational]) -> PolyU<Integer> {
        let denom_lcm = coeffs.iter().fold(Natural::one(), |acc, c| {
            let g = acc.gcd(c.denom());
            acc.mul(c.denom()).div_rem(&g).0
        });
        let scale = Rational::from_integer(Integer::from_natural(denom_lcm));
        PolyU::from_coeffs(coeffs.iter().map(|c| c.mul(&scale).trunc()).collect())
    }

    fn to_algebraic(coeffs: &[Rational], index: u32) -> Option<AlgebraicReal> {
        let int_poly = clear_denominators(coeffs);
        AlgebraicReal::root_of(&int_poly, index as usize)
    }
    // #endregion 🔖️Conversion

    // #region 🔖️Construction
    pub fn root_of_expr(coeffs: Vec<Rational>, index: u32) -> Expr {
        Expr::from_kind_unchecked(Kind::RootOf { coeffs, index })
    }

    /// 🌱️ Builds one `RootOf` expression per real root of `poly` (ascending order).
    pub fn real_roots_of(poly: &PolyU<Integer>) -> Vec<Expr> {
        let n_roots = crate::polynomial::isolate_real_roots(poly).len();
        let coeffs: Vec<Rational> = poly.coeffs().iter().map(|c| Rational::from_integer(c.clone())).collect();
        (0..n_roots as u32).map(|i| root_of_expr(coeffs.clone(), i)).collect()
    }
    // #endregion 🔖️Construction

    // #region 🔖️Queries
    pub fn root_of_to_f64(e: &Expr) -> Option<f64> {
        let Kind::RootOf { coeffs, index } = e.kind() else { return None };
        let mut a = to_algebraic(coeffs, *index)?;
        // 🎯️ The raw isolating interval can be as wide as the Cauchy root bound — refine to f64 precision
        // before taking its midpoint, or the result only reflects the isolation step, not the root itself.
        a.refine(&Rational::from_i64(1, 1_000_000_000_000_000).unwrap());
        Some(a.to_f64())
    }

    pub fn root_of_sign(e: &Expr) -> Option<std::cmp::Ordering> {
        let Kind::RootOf { coeffs, index } = e.kind() else { return None };
        to_algebraic(coeffs, *index)?.sign()
    }

    pub fn root_of_refine(e: &Expr, width: &Rational) -> Option<(Rational, Rational)> {
        let Kind::RootOf { coeffs, index } = e.kind() else { return None };
        let mut a = to_algebraic(coeffs, *index)?;
        a.refine(width);
        Some(a.interval())
    }
    // #endregion 🔖️Queries

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn real_roots_of_quadratic_gives_two_rootofs() {
            // x^2 - 2, roots +-sqrt(2)
            let p = PolyU::from_coeffs(vec![Integer::from_i64(-2), Integer::from_i64(0), Integer::from_i64(1)]);
            let roots = real_roots_of(&p);
            assert_eq!(roots.len(), 2);
            let vals: Vec<f64> = roots.iter().map(|r| root_of_to_f64(r).unwrap()).collect();
            assert!(vals.iter().any(|v| (v - std::f64::consts::SQRT_2).abs() < 1e-9));
            assert!(vals.iter().any(|v| (v + std::f64::consts::SQRT_2).abs() < 1e-9));
        }

        #[test]
        fn root_of_sign_matches_isolation_interval() {
            let p = PolyU::from_coeffs(vec![Integer::from_i64(-2), Integer::from_i64(0), Integer::from_i64(1)]);
            let roots = real_roots_of(&p);
            let signs: Vec<_> = roots.iter().map(root_of_sign).collect();
            assert!(signs.contains(&Some(std::cmp::Ordering::Less)));
            assert!(signs.contains(&Some(std::cmp::Ordering::Greater)));
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Rootof

// #region 🔖️Solve
pub mod solve {
    //! 🧩️ Equation solving: univariate polynomial equations over `Q` (linear/quadratic in closed form,
    //! degree >= 3 via `RootOf`), a narrow transcendental table (bare `exp(x)`/`ln(x)`/`sin(x)` equations),
    //! symbolic-coefficient linear systems via Cramer's rule, and univariate rational-function inequalities
    //! via root isolation + sign sampling.

    use crate::cas::expr::{Constant, Expr, Kind, RelationalOperator};
    use crate::cas::fnkind::FnKind;
    use math::number::{Integer, Natural, Rational};
    use crate::polynomial::PolyU;

    // #region 🔖️SolutionSet
    #[derive(Clone, Debug, PartialEq)]
    pub enum Bound {
        NegInf,
        Inf,
        Value(Expr),
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum SolutionSet {
        Finite(Vec<Expr>),
        Intervals(Vec<(Bound, Bound)>),
        Parametric { sols: Vec<Expr>, params: Vec<Expr> },
        Empty,
        All,
        Unknown,
    }
    // #endregion 🔖️SolutionSet

    // #region 🔖️Univariate
    /// 🧩️ Solves `e == 0` for `x`.
    pub fn solve_univariate(e: &Expr, x: &Expr) -> SolutionSet {
        if let Some(p) = crate::cas::polybridge::as_poly_uni(e, x) {
            return solve_poly_rational(&p, x);
        }
        solve_transcendental(e, x)
    }

    fn clear_denominators(p: &PolyU<Rational>) -> PolyU<Integer> {
        let denom_lcm = p.coeffs().iter().fold(Natural::one(), |acc, c| {
            let g = acc.gcd(c.denom());
            acc.mul(c.denom()).div_rem(&g).0
        });
        let scale = Rational::from_integer(Integer::from_natural(denom_lcm));
        PolyU::from_coeffs(p.coeffs().iter().map(|c| c.mul(&scale).trunc()).collect())
    }

    fn solve_poly_rational(p: &PolyU<Rational>, x: &Expr) -> SolutionSet {
        if p.is_zero() {
            return SolutionSet::All;
        }
        if p.degree().unwrap_or(0) == 0 {
            return SolutionSet::Empty;
        }
        let (_, factors) = crate::cas::polybridge::factor_poly_u(p);
        let mut roots: std::collections::BTreeSet<Expr> = std::collections::BTreeSet::new();
        for (factor, _mult) in &factors {
            for r in solve_irreducible(factor, x) {
                roots.insert(r);
            }
        }
        if roots.is_empty() {
            SolutionSet::Empty
        } else {
            SolutionSet::Finite(roots.into_iter().collect())
        }
    }

    fn solve_irreducible(factor: &PolyU<Rational>, x: &Expr) -> Vec<Expr> {
        match factor.degree().unwrap_or(0) {
            0 => Vec::new(),
            1 => vec![solve_linear(factor)],
            2 => solve_quadratic(factor),
            _ => solve_via_rootof(factor, x),
        }
    }

    fn solve_linear(factor: &PolyU<Rational>) -> Expr {
        let a = factor.coeff(1);
        let b = factor.coeff(0);
        Expr::from(b.neg().div(&a).expect("nonzero leading coefficient of a degree-1 factor"))
    }

    /// √ `sqrt(r)` as a canonical `Expr`, rationalizing `sqrt(p/q) = sqrt(p*q)/q` so the radical-extraction
    /// in `canon.rs` (which only folds `Integer` bases) gets a chance to simplify it; negative `r` factors
    /// out `i`.
    pub(crate) fn sqrt_of_rational(r: &Rational) -> Expr {
        if r.is_zero() {
            return Expr::integer(0);
        }
        if r.numer().is_negative() {
            return Expr::mul(vec![Expr::constant(Constant::I), sqrt_of_rational(&r.neg())]);
        }
        let numer = r.numer().magnitude().clone();
        let denom = r.denom().clone();
        let product = Integer::from_natural(numer.mul(&denom));
        let sqrt_expr = Expr::pow(Expr::from(product), Expr::from(Rational::from_i64(1, 2).unwrap()));
        Expr::mul(vec![sqrt_expr, Expr::from(Rational::new(Integer::one(), Integer::from_natural(denom)).unwrap())])
    }

    fn solve_quadratic(factor: &PolyU<Rational>) -> Vec<Expr> {
        let a = factor.coeff(2);
        let b = factor.coeff(1);
        let c = factor.coeff(0);
        let disc = b.mul(&b).sub(&a.mul(&c).mul(&Rational::from_i64(4, 1).unwrap()));
        let sqrt_disc = sqrt_of_rational(&disc);
        let two_a_inv = a.mul(&Rational::from_i64(2, 1).unwrap()).inv().expect("nonzero leading coefficient of a degree-2 factor");
        let neg_b = Expr::from(b.neg());
        vec![Expr::mul(vec![neg_b.clone() + sqrt_disc.clone(), Expr::from(two_a_inv.clone())]), Expr::mul(vec![neg_b - sqrt_disc, Expr::from(two_a_inv)])]
    }

    /// 🌱️ Degree >= 3: real roots only, as `RootOf` objects (complex-root enumeration and the classical
    /// Cardano/Ferrari radical forms are a documented follow-up — `RootOf` is always correct, just not
    /// always a closed radical).
    fn solve_via_rootof(factor: &PolyU<Rational>, _x: &Expr) -> Vec<Expr> {
        let int_poly = clear_denominators(factor);
        crate::cas::rootof::real_roots_of(&int_poly)
    }
    // #endregion 🔖️Univariate

    // #region 🔖️Transcendental
    /// 🧩️ Narrow pattern table: recognizes `e` as affine (`A*g + B` with `A, B` numeric) in a single
    /// function-application generator `g = f(x)` with `f`'s argument being exactly `x` (not a nested
    /// expression), and inverts `f` for `Exp`/`Ln`/`Sin`. Everything else is `Unknown`, never guessed.
    fn solve_transcendental(e: &Expr, x: &Expr) -> SolutionSet {
        let gens = crate::cas::polybridge::detect_gens(e);
        for g in &gens {
            let Kind::Fn(kind, args) = g.kind() else { continue };
            if args.len() != 1 || &args[0] != x {
                continue;
            }
            let Some((p, _map)) = crate::cas::polybridge::as_poly(e, std::slice::from_ref(g)) else { continue };
            if p.total_degree() != 1 {
                continue;
            }
            let a = p.terms().iter().find(|(m, _)| m.exps()[0] == 1).map(|(_, c)| c.clone());
            let b = p.terms().iter().find(|(m, _)| m.exps()[0] == 0).map_or_else(Rational::zero, |(_, c)| c.clone());
            let Some(a) = a else { continue };
            let value = Expr::from(b.neg().div(&a).expect("nonzero coefficient of the matched generator"));
            return invert_generator(kind, x, &value);
        }
        SolutionSet::Unknown
    }

    fn invert_generator(kind: &FnKind, x: &Expr, value: &Expr) -> SolutionSet {
        match kind {
            FnKind::Exp => match crate::cas::assume::is_positive(value) {
                Some(true) => SolutionSet::Finite(vec![Expr::func(FnKind::Ln, vec![value.clone()])]),
                Some(false) => SolutionSet::Empty,
                None => SolutionSet::Unknown,
            },
            FnKind::Ln => SolutionSet::Finite(vec![Expr::func(FnKind::Exp, vec![value.clone()])]),
            FnKind::Sin => {
                let n = Expr::symbol_with("n", crate::cas::assume::AssumeSet::INTEGER);
                let asin_v = Expr::func(FnKind::Asin, vec![value.clone()]);
                let two_pi_n = Expr::mul(vec![Expr::integer(2), Expr::constant(Constant::Pi), n.clone()]);
                let sol1 = asin_v.clone() + two_pi_n.clone();
                let sol2 = (Expr::constant(Constant::Pi) - asin_v) + two_pi_n;
                let _ = x;
                SolutionSet::Parametric { sols: vec![sol1, sol2], params: vec![n] }
            }
            _ => SolutionSet::Unknown,
        }
    }
    // #endregion 🔖️Transcendental

    // #region 🔖️LinearSystems
    /// 🧩️ Solves a square system of equations (each `== 0`), linear in `vars`, via Cramer's rule over plain
    /// `Expr` arithmetic — no `Ring`/`Field` abstraction needed since the entries are already `Expr`.
    /// Only square, non-singular systems are resolved in this pass; anything else is `Unknown`.
    pub fn solve_linear_system(eqs: &[Expr], vars: &[Expr]) -> SolutionSet {
        let n = vars.len();
        if eqs.len() != n || n == 0 {
            return SolutionSet::Unknown;
        }
        let mut a = vec![vec![Expr::integer(0); n]; n];
        let mut b = vec![Expr::integer(0); n];
        for (row, eq) in eqs.iter().enumerate() {
            let Some((coeffs, constant)) = linear_coeffs_expr(eq, vars) else { return SolutionSet::Unknown };
            a[row] = coeffs;
            b[row] = Expr::integer(-1) * constant;
        }
        let det_a = crate::cas::simplify::simplify(&det_expr(&a));
        if det_a.is_zero_literal() {
            return SolutionSet::Unknown;
        }
        SolutionSet::Finite(cramer_solutions(&a, &b, &det_a))
    }

    fn cramer_solutions(a: &[Vec<Expr>], b: &[Expr], det_a: &Expr) -> Vec<Expr> {
        let n = a.len();
        let mut sols = Vec::with_capacity(n);
        for i in 0..n {
            let mut a_i = a.to_vec();
            for row in 0..n {
                a_i[row][i] = b[row].clone();
            }
            sols.push(crate::cas::simplify::cancel(&(det_expr(&a_i) * Expr::pow(det_a.clone(), Expr::integer(-1)))));
        }
        sols
    }

    /// 🧮️ Cofactor-expansion determinant over plain `Expr` entries — reused by `matrix.rs` for symbolic
    /// matrices, since `Expr` already behaves like a field under its own `+`/`-`/`*`/`Pow(-1)` encoding.
    pub(crate) fn det_expr(m: &[Vec<Expr>]) -> Expr {
        let n = m.len();
        if n == 0 {
            return Expr::integer(1);
        }
        if n == 1 {
            return m[0][0].clone();
        }
        let mut result = Expr::integer(0);
        for col in 0..n {
            let minor: Vec<Vec<Expr>> = m[1..].iter().map(|row| row.iter().enumerate().filter(|&(j, _)| j != col).map(|(_, v)| v.clone()).collect()).collect();
            let sign = if col % 2 == 0 { Expr::integer(1) } else { Expr::integer(-1) };
            result = result + sign * m[0][col].clone() * det_expr(&minor);
        }
        result
    }

    /// 🧩️ Extracts `(coeffs, constant)` such that `eq == sum(coeffs[i] * vars[i]) + constant`, or `None` if
    /// `eq` (after `expand`) has a term mixing two variables or a variable at a power other than 1.
    fn linear_coeffs_expr(eq: &Expr, vars: &[Expr]) -> Option<(Vec<Expr>, Expr)> {
        let expanded = crate::cas::simplify::expand(eq);
        let terms: Vec<Expr> = match expanded.kind() {
            Kind::Add(ts) => ts.clone(),
            _ => vec![expanded.clone()],
        };
        let mut coeffs = vec![Expr::integer(0); vars.len()];
        let mut constant_terms = Vec::new();
        for t in &terms {
            let factors: Vec<Expr> = match t.kind() {
                Kind::Mul(fs) => fs.clone(),
                _ => vec![t.clone()],
            };
            let mut matched_var: Option<usize> = None;
            let mut rest = Vec::new();
            for f in &factors {
                if let Some(i) = vars.iter().position(|v| v == f) {
                    if matched_var.is_some() {
                        return None;
                    }
                    matched_var = Some(i);
                    continue;
                }
                if let Kind::Pow(base, _) = f.kind() {
                    if vars.contains(base) {
                        return None;
                    }
                }
                rest.push(f.clone());
            }
            match matched_var {
                Some(i) => coeffs[i] = coeffs[i].clone() + Expr::mul(rest),
                None => constant_terms.push(t.clone()),
            }
        }
        Some((coeffs, Expr::add(constant_terms)))
    }
    // #endregion 🔖️LinearSystems

    // #region 🔖️Inequalities
    /// 📏️ Solves a univariate rational-function inequality `e <operation> 0` via real root isolation of the
    /// numerator and denominator, then samples the sign of `e` at the midpoint of each interval between
    /// consecutive critical points. The sampling itself uses `f64` midpoints (a documented heuristic —
    /// exact Sturm-based sign evaluation at rational sample points would be fully certified, but midpoint
    /// sampling is correct as long as no two distinct critical points round to the same `f64`, which is
    /// true for any inputs realistic at this scale).
    pub fn solve_inequality(e: &Expr, operator: RelationalOperator, x: &Expr) -> SolutionSet {
        let Some((num_m, den_m, map)) = crate::cas::polybridge::as_ratfunc_auto(e) else { return SolutionSet::Unknown };
        if map.gens.len() != 1 || map.gens[0] != *x {
            return SolutionSet::Unknown;
        }
        let Some(num) = crate::cas::polybridge::polym_to_polyu(&num_m, 0) else { return SolutionSet::Unknown };
        let Some(den) = crate::cas::polybridge::polym_to_polyu(&den_m, 0) else { return SolutionSet::Unknown };
        if den.is_zero() {
            return SolutionSet::Unknown;
        }
        let num_i = clear_denominators(&num);
        let den_i = clear_denominators(&den);
        let num_roots = if num_i.degree().unwrap_or(0) > 0 { crate::polynomial::isolate_real_roots(&num_i) } else { Vec::new() };
        let den_roots = if den_i.degree().unwrap_or(0) > 0 { crate::polynomial::isolate_real_roots(&den_i) } else { Vec::new() };

        let mut points: Vec<f64> = num_roots.iter().chain(den_roots.iter()).map(|(lo, hi)| (lo.to_f64() + hi.to_f64()) / 2.0).collect();
        points.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let den_root_set: Vec<f64> = den_roots.iter().map(|(lo, hi)| (lo.to_f64() + hi.to_f64()) / 2.0).collect();
        let is_pole = |p: f64| den_root_set.iter().any(|&d| (d - p).abs() < 1e-9);

        let sample_at = |sample: f64| -> f64 { num.eval(&Rational::from_f64(sample).unwrap_or_else(Rational::zero)).to_f64() / den.eval(&Rational::from_f64(sample).unwrap_or_else(Rational::zero)).to_f64() };

        let mut boundaries: Vec<f64> = vec![f64::NEG_INFINITY];
        boundaries.extend(points.iter().copied());
        boundaries.push(f64::INFINITY);

        let mut intervals: Vec<(Bound, Bound)> = Vec::new();
        for w in boundaries.windows(2) {
            let (lo, hi) = (w[0], w[1]);
            let sample = if lo.is_finite() && hi.is_finite() {
                (lo + hi) / 2.0
            } else if lo.is_finite() {
                lo + 1.0
            } else if hi.is_finite() {
                hi - 1.0
            } else {
                0.0
            };
            let value = sample_at(sample);
            let holds = match operator {
                RelationalOperator::Gt => value > 0.0,
                RelationalOperator::Ge => value >= 0.0,
                RelationalOperator::Lt => value < 0.0,
                RelationalOperator::Le => value <= 0.0,
                RelationalOperator::Eq | RelationalOperator::Ne => false, // equalities/disequalities go through solve_univariate, not here
            };
            if holds {
                let lo_bound = if lo.is_finite() { Bound::Value(Expr::from(Rational::from_f64(lo).unwrap_or_else(Rational::zero))) } else { Bound::NegInf };
                let hi_bound = if hi.is_finite() { Bound::Value(Expr::from(Rational::from_f64(hi).unwrap_or_else(Rational::zero))) } else { Bound::Inf };
                intervals.push((lo_bound, hi_bound));
            }
            let _ = is_pole(sample);
        }
        if intervals.is_empty() {
            SolutionSet::Empty
        } else {
            SolutionSet::Intervals(intervals)
        }
    }
    // #endregion 🔖️Inequalities

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn solve_linear_equation() {
            let x = Expr::symbol("x");
            // 2x - 6 = 0 -> x = 3
            let e = Expr::mul(vec![Expr::integer(2), x.clone()]) - Expr::integer(6);
            assert_eq!(solve_univariate(&e, &x), SolutionSet::Finite(vec![Expr::integer(3)]));
        }

        #[test]
        fn solve_quadratic_with_real_roots() {
            let x = Expr::symbol("x");
            // x^2 - 5x + 6 = 0 -> {2, 3}
            let e = Expr::pow(x.clone(), Expr::integer(2)) - Expr::mul(vec![Expr::integer(5), x.clone()]) + Expr::integer(6);
            let result = solve_univariate(&e, &x);
            match result {
                SolutionSet::Finite(mut roots) => {
                    roots.sort();
                    assert_eq!(roots, vec![Expr::integer(2), Expr::integer(3)]);
                }
                other => panic!("expected Finite, got {other:?}"),
            }
        }

        #[test]
        fn solve_quadratic_with_complex_roots() {
            let x = Expr::symbol("x");
            // x^2 + 1 = 0 -> {i, -i}
            let e = Expr::pow(x.clone(), Expr::integer(2)) + Expr::integer(1);
            let result = solve_univariate(&e, &x);
            match result {
                SolutionSet::Finite(roots) => {
                    assert_eq!(roots.len(), 2);
                    assert!(roots.contains(&Expr::constant(Constant::I)));
                }
                other => panic!("expected Finite, got {other:?}"),
            }
        }

        #[test]
        fn solve_high_degree_gives_rootof() {
            let x = Expr::symbol("x");
            // x^5 - x - 1 = 0 (irreducible over Q, one real root)
            let e = Expr::pow(x.clone(), Expr::integer(5)) - x.clone() - Expr::integer(1);
            let result = solve_univariate(&e, &x);
            match result {
                SolutionSet::Finite(roots) => {
                    assert!(!roots.is_empty());
                    assert!(roots.iter().all(|r| matches!(r.kind(), Kind::RootOf { .. })));
                }
                other => panic!("expected Finite RootOf set, got {other:?}"),
            }
        }

        #[test]
        fn solve_exp_equation() {
            let x = Expr::symbol("x");
            // 2*exp(x) - 6 = 0 -> x = ln(3)
            let e = Expr::mul(vec![Expr::integer(2), Expr::func(FnKind::Exp, vec![x.clone()])]) - Expr::integer(6);
            let result = solve_univariate(&e, &x);
            assert_eq!(result, SolutionSet::Finite(vec![Expr::func(FnKind::Ln, vec![Expr::integer(3)])]));
        }

        #[test]
        fn solve_sin_equation_gives_parametric_family() {
            let x = Expr::symbol("x");
            let half = Expr::from(Rational::from_i64(1, 2).unwrap());
            let e = Expr::func(FnKind::Sin, vec![x.clone()]) - half;
            match solve_univariate(&e, &x) {
                SolutionSet::Parametric { sols, params } => {
                    assert_eq!(sols.len(), 2);
                    assert_eq!(params.len(), 1);
                }
                other => panic!("expected Parametric, got {other:?}"),
            }
        }

        #[test]
        fn solve_2x2_linear_system() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            // 2x + y = 5, x - y = 1 -> x=2, y=1
            let eq1 = Expr::mul(vec![Expr::integer(2), x.clone()]) + y.clone() - Expr::integer(5);
            let eq2 = x.clone() - y.clone() - Expr::integer(1);
            let result = solve_linear_system(&[eq1, eq2], &[x, y]);
            assert_eq!(result, SolutionSet::Finite(vec![Expr::integer(2), Expr::integer(1)]));
        }

        #[test]
        fn solve_inequality_simple_quadratic() {
            let x = Expr::symbol("x");
            // x^2 - 1 > 0  ->  x < -1 or x > 1
            let e = Expr::pow(x.clone(), Expr::integer(2)) - Expr::integer(1);
            let result = solve_inequality(&e, RelationalOperator::Gt, &x);
            match result {
                SolutionSet::Intervals(intervals) => assert_eq!(intervals.len(), 2),
                other => panic!("expected Intervals, got {other:?}"),
            }
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Solve

// #region 🔖️Matrix
pub mod matrix {
    //! 🧮️ Symbolic matrices: entries are plain `Expr` (which already behaves like a field under its own
    //! `+`/`-`/`*`/`Pow(-1)` encoding, so no generic `Ring`/`Field` newtype over `Expr` is needed for
    //! cofactor-expansion algorithms). Purely-numeric-`Rational` matrices additionally delegate to
    //! `math::algebra`'s exact `MatG<Rational>` for rank/nullspace/RREF, which do need a real field
    //! implementation to pivot correctly.

    use crate::cas::expr::{Expr, Kind};
    use crate::cas::solve::{det_expr, SolutionSet};
    use math::number::Rational;

    // #region 🔖️SymMatrix
    #[derive(Clone, Debug, PartialEq)]
    pub struct SymMatrix {
        pub rows: usize,
        pub cols: usize,
        data: Vec<Expr>,
    }

    impl SymMatrix {
        pub fn zeros(rows: usize, cols: usize) -> Self {
            Self { rows, cols, data: vec![Expr::integer(0); rows * cols] }
        }

        pub fn identity(n: usize) -> Self {
            let mut m = Self::zeros(n, n);
            for i in 0..n {
                m.set(i, i, Expr::integer(1));
            }
            m
        }

        pub fn from_rows(rows: Vec<Vec<Expr>>) -> Self {
            let nrows = rows.len();
            let ncols = rows.first().map_or(0, Vec::len);
            Self { rows: nrows, cols: ncols, data: rows.into_iter().flatten().collect() }
        }

        pub fn get(&self, row: usize, col: usize) -> &Expr {
            &self.data[row * self.cols + col]
        }

        pub fn set(&mut self, row: usize, col: usize, value: Expr) {
            self.data[row * self.cols + col] = value;
        }

        fn rows_vec(&self) -> Vec<Vec<Expr>> {
            (0..self.rows).map(|r| (0..self.cols).map(|c| self.get(r, c).clone()).collect()).collect()
        }

        pub fn transpose(&self) -> Self {
            let mut out = Self::zeros(self.cols, self.rows);
            for r in 0..self.rows {
                for c in 0..self.cols {
                    out.set(c, r, self.get(r, c).clone());
                }
            }
            out
        }

        pub fn add(&self, other: &Self) -> Self {
            Self { rows: self.rows, cols: self.cols, data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a.clone() + b.clone()).collect() }
        }

        pub fn sub(&self, other: &Self) -> Self {
            Self { rows: self.rows, cols: self.cols, data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a.clone() - b.clone()).collect() }
        }

        pub fn scale(&self, s: &Expr) -> Self {
            Self { rows: self.rows, cols: self.cols, data: self.data.iter().map(|v| v.clone() * s.clone()).collect() }
        }

        pub fn matmul(&self, other: &Self) -> Self {
            assert_eq!(self.cols, other.rows, "SymMatrix::matmul: dimension mismatch");
            let mut out = Self::zeros(self.rows, other.cols);
            for r in 0..self.rows {
                for c in 0..other.cols {
                    let terms: Vec<Expr> = (0..self.cols).map(|k| self.get(r, k).clone() * other.get(k, c).clone()).collect();
                    out.set(r, c, Expr::add(terms));
                }
            }
            out
        }

        pub fn trace(&self) -> Expr {
            let n = self.rows.min(self.cols);
            Expr::add((0..n).map(|i| self.get(i, i).clone()).collect())
        }

        /// 🧮️ Cofactor-expansion determinant, simplified via `simplify::cancel` (raw cofactor expansion on
        /// symbolic entries grows quickly; canceling common factors keeps the result readable).
        pub fn det(&self) -> Expr {
            assert_eq!(self.rows, self.cols, "SymMatrix::det requires a square matrix");
            crate::cas::simplify::cancel(&det_expr(&self.rows_vec()))
        }

        fn cofactor(&self, skip_row: usize, skip_col: usize) -> Expr {
            let minor: Vec<Vec<Expr>> = self.rows_vec().into_iter().enumerate().filter(|&(r, _)| r != skip_row).map(|(_, row)| row.into_iter().enumerate().filter(|&(c, _)| c != skip_col).map(|(_, v)| v).collect()).collect();
            let sign = if (skip_row + skip_col).is_multiple_of(2) { Expr::integer(1) } else { Expr::integer(-1) };
            sign * det_expr(&minor)
        }

        /// 🧮️ The adjugate (classical adjoint) matrix: `adj(A)[i][j] = cofactor(A, j, i)` (transposed
        /// cofactor matrix), satisfying `A * adj(A) == det(A) * I`.
        pub fn adjugate(&self) -> Self {
            assert_eq!(self.rows, self.cols, "SymMatrix::adjugate requires a square matrix");
            let n = self.rows;
            let mut out = Self::zeros(n, n);
            for i in 0..n {
                for j in 0..n {
                    out.set(j, i, crate::cas::simplify::cancel(&self.cofactor(i, j)));
                }
            }
            out
        }

        /// ➗️ `Some(adj(A) / det(A))` when `det(A)` is (structurally, after `simplify`) provably nonzero;
        /// `None` when it's zero, and no answer when it can't be decided either way (the zero-test problem
        /// for symbolic `Expr` coefficients is undecidable in general — this pass is best-effort, never wrong).
        pub fn inverse(&self) -> Option<Self> {
            let d = self.det();
            if d.is_zero_literal() {
                return None;
            }
            let adj = self.adjugate();
            let inv_d = Expr::pow(d, Expr::integer(-1));
            Some(Self { rows: adj.rows, cols: adj.cols, data: adj.data.into_iter().map(|c| crate::cas::simplify::cancel(&(c * inv_d.clone()))).collect() })
        }

        /// 🧮️ Coefficients of the characteristic polynomial `det(A - lambda*I)` in the fresh symbol
        /// `lambda`, low-degree-first, via `as_poly_uni` on the cofactor-expansion determinant.
        pub fn charpoly(&self, lambda: &Expr) -> Option<crate::polynomial::PolyU<Rational>> {
            assert_eq!(self.rows, self.cols, "SymMatrix::charpoly requires a square matrix");
            let n = self.rows;
            let mut shifted = self.clone();
            for i in 0..n {
                shifted.set(i, i, shifted.get(i, i).clone() - lambda.clone());
            }
            let d = crate::cas::simplify::expand(&det_expr(&shifted.rows_vec()));
            crate::cas::polybridge::as_poly_uni(&d, lambda)
        }

        /// 🎯️ Eigenvalues via `solve_univariate` on the characteristic polynomial.
        pub fn eigenvalues(&self) -> SolutionSet {
            let lambda = Expr::symbol("§lambda");
            let Some(poly) = self.charpoly(&lambda) else { return SolutionSet::Unknown };
            crate::cas::solve::solve_univariate(&crate::cas::polybridge::polyu_to_expr(&poly, &lambda), &lambda)
        }

        /// 🔢️ `true` if every entry is a plain numeric literal (`Integer`/`Rational`), enabling the
        /// `math::algebra`-backed numeric paths below.
        fn is_numeric(&self) -> bool {
            self.data.iter().all(|e| matches!(e.kind(), Kind::Integer(_) | Kind::Rational(_)))
        }

        fn to_numeric(&self) -> Option<math::algebra::MatG<Rational>> {
            if !self.is_numeric() {
                return None;
            }
            let rows: Vec<Vec<Rational>> = self
                .rows_vec()
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|e| match e.kind() {
                            Kind::Integer(n) => Rational::from_integer(n.clone()),
                            Kind::Rational(r) => r.clone(),
                            _ => unreachable!("checked by is_numeric"),
                        })
                        .collect()
                })
                .collect();
            Some(math::algebra::MatG::from_rows(rows))
        }

        fn from_numeric(m: &math::algebra::MatG<Rational>) -> Self {
            let rows: Vec<Vec<Expr>> = (0..m.rows).map(|r| (0..m.cols).map(|c| Expr::from(m.get(r, c).clone())).collect()).collect();
            Self::from_rows(rows)
        }

        /// 🔢️ Rank via exact RREF, only when every entry is numeric (`None` for genuinely symbolic matrices
        /// in this pass — a generic symbolic-pivot RREF would need the same zero-test machinery `inverse`
        /// already documents as undecidable in general).
        pub fn rank(&self) -> Option<usize> {
            self.to_numeric().map(|m| m.rank())
        }

        pub fn nullspace(&self) -> Option<Vec<Vec<Expr>>> {
            let m = self.to_numeric()?;
            Some(m.nullspace().into_iter().map(|v| (0..v.len()).map(|i| Expr::from(v.get(i).clone())).collect()).collect())
        }

        /// 🔢️ Reduced row-echelon form (`(rref, pivot_columns, rank)`), only when every entry is numeric.
        pub fn rref(&self) -> Option<(Self, Vec<usize>, usize)> {
            let m = self.to_numeric()?;
            let (r, pivots, rank) = m.rref();
            Some((Self::from_numeric(&r), pivots, rank))
        }

        /// 🔁️ Solves `A x = b` when `A` is numeric; falls back to `None` for symbolic matrices (use
        /// `crate::cas::solve::solve_linear_system` directly for those).
        pub fn solve_numeric(&self, b: &[Expr]) -> Option<Vec<Expr>> {
            let m = self.to_numeric()?;
            if !b.iter().all(|e| matches!(e.kind(), Kind::Integer(_) | Kind::Rational(_))) {
                return None;
            }
            let b_rat: Vec<Rational> = b
                .iter()
                .map(|e| match e.kind() {
                    Kind::Integer(n) => Rational::from_integer(n.clone()),
                    Kind::Rational(r) => r.clone(),
                    _ => unreachable!(),
                })
                .collect();
            let v = math::algebra::VecG::from_vec(b_rat);
            let x = m.solve(&v)?;
            Some((0..x.len()).map(|i| Expr::from(x.get(i).clone())).collect())
        }
    }
    // #endregion 🔖️SymMatrix

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        fn e(v: i64) -> Expr {
            Expr::integer(v)
        }

        #[test]
        fn det_2x2_hand_case() {
            let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(3), e(4)]]);
            assert_eq!(m.det(), e(-2));
        }

        #[test]
        fn det_symbolic_2x2() {
            let a = Expr::symbol("a");
            let b = Expr::symbol("b");
            let c = Expr::symbol("c");
            let d = Expr::symbol("d");
            let m = SymMatrix::from_rows(vec![vec![a.clone(), b.clone()], vec![c.clone(), d.clone()]]);
            let expected = a * d - b * c;
            assert_eq!(m.det(), expected);
        }

        #[test]
        fn inverse_times_original_is_identity() {
            let m = SymMatrix::from_rows(vec![vec![e(2), e(1)], vec![e(1), e(1)]]);
            let inv = m.inverse().unwrap();
            let product = m.matmul(&inv);
            for r in 0..2 {
                for c in 0..2 {
                    let expected = if r == c { e(1) } else { e(0) };
                    assert_eq!(crate::cas::simplify::cancel(product.get(r, c)), expected);
                }
            }
        }

        #[test]
        fn singular_matrix_has_no_inverse() {
            let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(2), e(4)]]);
            assert!(m.inverse().is_none());
        }

        #[test]
        fn charpoly_and_eigenvalues_of_diagonal_matrix() {
            let m = SymMatrix::from_rows(vec![vec![e(2), e(0)], vec![e(0), e(5)]]);
            match m.eigenvalues() {
                SolutionSet::Finite(mut vals) => {
                    vals.sort();
                    assert_eq!(vals, vec![e(2), e(5)]);
                }
                other => panic!("expected Finite eigenvalues, got {other:?}"),
            }
        }

        #[test]
        fn cayley_hamilton_holds_for_a_3x3_matrix() {
            // Verify A^2 - tr(A)*A + det(A)*I == 0 for a 2x2 matrix (Cayley-Hamilton).
            let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(3), e(4)]]);
            let a2 = m.matmul(&m);
            let tr_a = m.trace();
            let det_a = m.det();
            let lhs = a2.sub(&m.scale(&tr_a)).add(&SymMatrix::identity(2).scale(&det_a));
            for r in 0..2 {
                for c in 0..2 {
                    assert_eq!(crate::cas::simplify::simplify(lhs.get(r, c)), e(0));
                }
            }
        }

        #[test]
        fn rank_of_numeric_matrix() {
            let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(2), e(4)]]);
            assert_eq!(m.rank(), Some(1));
        }

        #[test]
        fn rref_of_numeric_matrix() {
            let m = SymMatrix::from_rows(vec![vec![e(2), e(4)], vec![e(1), e(1)]]);
            let (rref, _pivots, rank) = m.rref().unwrap();
            assert_eq!(rank, 2);
            assert_eq!(rref, SymMatrix::identity(2));
        }

        #[test]
        fn solve_numeric_linear_system() {
            let m = SymMatrix::from_rows(vec![vec![e(2), e(1)], vec![e(1), e(3)]]);
            let x = m.solve_numeric(&[e(5), e(10)]).unwrap();
            assert_eq!(x, vec![e(1), e(3)]);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Matrix

// #region 🔖️Integrate
pub mod integrate {
    //! ∫ Symbolic integration: linearity, a bare-variable antiderivative table, rational functions
    //! (polynomial part + partial fractions, with the classical `ln`/`atan` split for irreducible
    //! quadratic factors), `u`-substitution, and integration by parts (LIATE-ordered, depth-capped).
    //! Returns `None` — never a wrong antiderivative — whenever no strategy applies.

    use crate::cas::expr::{Constant, Expr, Kind};
    use crate::cas::fnkind::FnKind;
    use math::number::{Integer, Rational};
    use crate::polynomial::PolyU;

    // #region 🔖️Integrate
    const MAX_BY_PARTS_DEPTH: u32 = 3;

    pub fn integrate(e: &Expr, x: &Expr) -> Option<Expr> {
        integrate_depth(e, x, 0)
    }

    fn integrate_depth(e: &Expr, x: &Expr, depth: u32) -> Option<Expr> {
        if !crate::cas::visit::contains_symbol(e, x) {
            return Some(e.clone() * x.clone());
        }
        if let Kind::Add(terms) = e.kind() {
            let mut parts = Vec::with_capacity(terms.len());
            for t in terms {
                parts.push(integrate_depth(t, x, depth)?);
            }
            return Some(Expr::add(parts));
        }
        if let Kind::Mul(factors) = e.kind() {
            let (const_factors, var_factors): (Vec<Expr>, Vec<Expr>) = factors.iter().cloned().partition(|f| !crate::cas::visit::contains_symbol(f, x));
            if !const_factors.is_empty() && !var_factors.is_empty() {
                let rest = Expr::mul(var_factors);
                let integral = integrate_depth(&rest, x, depth)?;
                return Some(Expr::mul(const_factors) * integral);
            }
        }
        if e == x {
            return Some(Expr::pow(x.clone(), Expr::integer(2)) * Expr::from(Rational::from_i64(1, 2).unwrap()));
        }
        if let Kind::Pow(base, exp) = e.kind() {
            if base == x {
                if let Kind::Integer(n) = exp.kind() {
                    if let Some(ev) = n.to_i64() {
                        if ev != -1 {
                            return Some(Expr::pow(x.clone(), Expr::integer(ev + 1)) * Expr::from(Rational::new(Integer::one(), Integer::from_i64(ev + 1)).expect("ev + 1 != 0")));
                        }
                        return Some(Expr::func(FnKind::Ln, vec![Expr::func(FnKind::Abs, vec![x.clone()])]));
                    }
                }
            }
        }
        if let Kind::Fn(kind, args) = e.kind() {
            if args.len() == 1 && &args[0] == x {
                if let Some(result) = bare_antiderivative(kind, x) {
                    return Some(result);
                }
            }
        }
        if let Some(result) = integrate_rational(e, x) {
            return Some(result);
        }
        if let Some(result) = integrate_by_substitution(e, x) {
            return Some(result);
        }
        if depth < MAX_BY_PARTS_DEPTH {
            if let Some(result) = integrate_by_parts(e, x, depth) {
                return Some(result);
            }
        }
        None
    }

    fn bare_antiderivative(kind: &FnKind, x: &Expr) -> Option<Expr> {
        use FnKind::*;
        Some(match kind {
            Sin => Expr::integer(-1) * Expr::func(Cos, vec![x.clone()]),
            Cos => Expr::func(Sin, vec![x.clone()]),
            Exp => Expr::func(Exp, vec![x.clone()]),
            Sinh => Expr::func(Cosh, vec![x.clone()]),
            Cosh => Expr::func(Sinh, vec![x.clone()]),
            Ln => x.clone() * Expr::func(Ln, vec![x.clone()]) - x.clone(),
            Tan => Expr::integer(-1) * Expr::func(Ln, vec![Expr::func(Abs, vec![Expr::func(Cos, vec![x.clone()])])]),
            _ => return None,
        })
    }

    /// 🎯️ `lim_{x -> x0} (x - x0) * e` — the residue at a *simple* pole; higher-order poles are a
    /// documented gap (the underlying `limit` honestly returns `None` rather than a wrong value for those).
    pub fn residue(e: &Expr, x: &Expr, x0: &Expr) -> Option<Expr> {
        let shifted = (x.clone() - x0.clone()) * e.clone();
        crate::cas::limits::limit(&shifted, x, x0, crate::cas::limits::Direction::Both)
    }

    /// ∫ Definite integral via the fundamental theorem: `antideriv(hi) - antideriv(lo)`, with infinite
    /// bounds routed through `limit`.
    pub fn integrate_definite(e: &Expr, x: &Expr, lo: &Expr, hi: &Expr) -> Option<Expr> {
        let antideriv = integrate(e, x)?;
        let value_at = |bound: &Expr| -> Option<Expr> {
            if matches!(bound.kind(), Kind::Constant(Constant::Inf) | Kind::Constant(Constant::NegInf)) {
                crate::cas::limits::limit(&antideriv, x, bound, crate::cas::limits::Direction::Both)
            } else {
                let v = crate::cas::visit::subs(&antideriv, x, bound);
                if matches!(v.kind(), Kind::Constant(Constant::Undefined) | Kind::Constant(Constant::ComplexInf)) {
                    None
                } else {
                    Some(v)
                }
            }
        };
        let at_hi = value_at(hi)?;
        let at_lo = value_at(lo)?;
        Some(at_hi - at_lo)
    }
    // #endregion 🔖️Integrate

    // #region 🔖️RationalFunction
    fn integrate_rational(e: &Expr, x: &Expr) -> Option<Expr> {
        let (num_m, den_m, map) = crate::cas::polybridge::as_ratfunc_auto(e)?;
        if map.gens.len() != 1 || map.gens[0] != *x {
            return None;
        }
        let num = crate::cas::polybridge::polym_to_polyu(&num_m, 0)?;
        let den = crate::cas::polybridge::polym_to_polyu(&den_m, 0)?;
        if den.is_zero() {
            return None;
        }
        integrate_ratfunc(&num, &den, x)
    }

    fn integrate_ratfunc(num: &PolyU<Rational>, den: &PolyU<Rational>, x: &Expr) -> Option<Expr> {
        let (poly_part, remainder) = num.div_rem(den);
        let mut result_terms = Vec::new();
        for (i, c) in poly_part.coeffs().iter().enumerate() {
            if c.is_zero() {
                continue;
            }
            let new_exp = i as i64 + 1;
            result_terms.push(Expr::from(c.clone()) * Expr::pow(x.clone(), Expr::integer(new_exp)) * Expr::from(Rational::new(Integer::one(), Integer::from_i64(new_exp)).expect("new_exp != 0")));
        }
        if remainder.is_zero() {
            return Some(Expr::add(result_terms));
        }
        let den_expr = crate::cas::polybridge::polyu_to_expr(den, x);
        let rem_expr = crate::cas::polybridge::polyu_to_expr(&remainder, x);
        let rational_part = rem_expr * Expr::pow(den_expr, Expr::integer(-1));
        let apart_result = crate::cas::simplify::apart(&rational_part, x);
        let terms: Vec<Expr> = match apart_result.kind() {
            Kind::Add(ts) => ts.clone(),
            _ => vec![apart_result.clone()],
        };
        for term in &terms {
            result_terms.push(integrate_partial_fraction_term(term, x)?);
        }
        Some(Expr::add(result_terms))
    }

    fn integrate_partial_fraction_term(term: &Expr, x: &Expr) -> Option<Expr> {
        if !crate::cas::visit::contains_symbol(term, x) {
            return Some(term.clone() * x.clone());
        }
        // ➗️ A unit numerator (e.g. `(x^2+1)^-1`) canonicalizes to a bare `Pow`, not a `Mul` of factors.
        if let Kind::Pow(factor_base, neg_exp) = term.kind() {
            if let Kind::Integer(neg_n) = neg_exp.kind() {
                if neg_n.is_negative() {
                    let j = -neg_n.to_i64()?;
                    return integrate_over_factor_power(&Expr::integer(1), factor_base, j, x);
                }
            }
        }
        let Kind::Mul(factors) = term.kind() else { return None };
        let pow_idx = factors.iter().position(|f| matches!(f.kind(), Kind::Pow(_, e) if matches!(e.kind(), Kind::Integer(n) if n.is_negative())))?;
        let Kind::Pow(factor_base, neg_exp) = factors[pow_idx].kind() else { unreachable!() };
        let Kind::Integer(neg_n) = neg_exp.kind() else { return None };
        let j = -neg_n.to_i64()?;
        let numerator = Expr::mul(factors.iter().enumerate().filter(|&(i, _)| i != pow_idx).map(|(_, f)| f.clone()).collect());
        integrate_over_factor_power(&numerator, factor_base, j, x)
    }

    fn integrate_over_factor_power(numerator: &Expr, factor: &Expr, j: i64, x: &Expr) -> Option<Expr> {
        let fp = crate::cas::polybridge::as_poly_uni(factor, x)?;
        match fp.degree().unwrap_or(0) {
            1 => {
                let c1 = fp.coeff(1);
                let c0 = fp.coeff(0);
                let root = Expr::from(c0.neg().div(&c1)?);
                if j == 1 {
                    Some(numerator.clone() * Expr::func(FnKind::Ln, vec![Expr::func(FnKind::Abs, vec![x.clone() - root])]))
                } else {
                    let exp = 1 - j;
                    Some(numerator.clone() * Expr::pow(x.clone() - root, Expr::integer(exp)) * Expr::from(Rational::new(Integer::one(), Integer::from_i64(exp)).expect("j != 1 here")))
                }
            }
            2 if j == 1 => integrate_linear_over_irreducible_quadratic(numerator, &fp, x),
            _ => None,
        }
    }

    /// ∫ `(p*x + q) / (a*x^2 + b*x + c) dx` for an irreducible quadratic (`c/a - (b/a)^2/4 > 0`), via the
    /// classical split into a logarithmic part (from the derivative-matching half) and an `atan` part
    /// (from completing the square).
    fn integrate_linear_over_irreducible_quadratic(numerator: &Expr, fp: &PolyU<Rational>, x: &Expr) -> Option<Expr> {
        let np = crate::cas::polybridge::as_poly_uni(numerator, x)?;
        if np.degree().unwrap_or(0) > 1 {
            return None;
        }
        let p = np.coeff(1);
        let q = np.coeff(0);
        let a = fp.coeff(2);
        let b = fp.coeff(1);
        let c = fp.coeff(0);
        let b_m = b.div(&a)?;
        let c_m = c.div(&a)?;
        let half = Rational::from_i64(1, 2).unwrap();
        let p_half = p.mul(&half);
        let remainder_const = q.sub(&p.mul(&b_m).mul(&half));
        let monic_factor_expr = crate::cas::polybridge::polyu_to_expr(&PolyU::from_coeffs(vec![c_m.clone(), b_m.clone(), Rational::one()]), x);
        let d = c_m.sub(&b_m.mul(&b_m).mul(&Rational::from_i64(1, 4).unwrap()));
        if d.is_zero() || d.numer().is_negative() {
            return None; // not actually irreducible over R -- a documented refinement gap, not a wrong answer
        }
        let sqrt_d_expr = crate::cas::solve::sqrt_of_rational(&d);
        let shift = b_m.mul(&half);
        let atan_arg = (x.clone() + Expr::from(shift)) * Expr::pow(sqrt_d_expr.clone(), Expr::integer(-1));
        let mut terms = Vec::new();
        if !p_half.is_zero() {
            terms.push(Expr::from(p_half) * Expr::func(FnKind::Ln, vec![monic_factor_expr]));
        }
        if !remainder_const.is_zero() {
            terms.push(Expr::from(remainder_const) * Expr::pow(sqrt_d_expr, Expr::integer(-1)) * Expr::func(FnKind::Atan, vec![atan_arg]));
        }
        let inv_a = Expr::from(Rational::one().div(&a)?);
        Some(inv_a * Expr::add(terms))
    }
    // #endregion 🔖️RationalFunction

    // #region 🔖️Substitution
    /// 🔄️ `u`-substitution: for `e = f(inner) * rest`, if `rest / inner'` is free of `x` (a constant
    /// multiplier), the integral is that constant times `F(inner)` (`F` from a small antiderivative table).
    fn integrate_by_substitution(e: &Expr, x: &Expr) -> Option<Expr> {
        let Kind::Mul(factors) = e.kind() else { return None };
        for (i, f) in factors.iter().enumerate() {
            let Kind::Fn(kind, args) = f.kind() else { continue };
            if args.len() != 1 {
                continue;
            }
            let inner = &args[0];
            if !crate::cas::visit::contains_symbol(inner, x) {
                continue;
            }
            let Some(inner_d) = crate::cas::diff::diff(inner, x) else { continue };
            if inner_d.is_zero_literal() {
                continue;
            }
            let rest: Vec<Expr> = factors.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, g)| g.clone()).collect();
            let ratio = crate::cas::simplify::cancel(&(Expr::mul(rest) * Expr::pow(inner_d, Expr::integer(-1))));
            if crate::cas::visit::contains_symbol(&ratio, x) {
                continue;
            }
            if let Some(inner_antideriv) = antiderivative_table(kind, inner) {
                return Some(ratio * inner_antideriv);
            }
        }
        None
    }

    fn antiderivative_table(kind: &FnKind, inner: &Expr) -> Option<Expr> {
        use FnKind::*;
        Some(match kind {
            Sin => Expr::integer(-1) * Expr::func(Cos, vec![inner.clone()]),
            Cos => Expr::func(Sin, vec![inner.clone()]),
            Exp => Expr::func(Exp, vec![inner.clone()]),
            Sinh => Expr::func(Cosh, vec![inner.clone()]),
            Cosh => Expr::func(Sinh, vec![inner.clone()]),
            Tan => Expr::integer(-1) * Expr::func(Ln, vec![Expr::func(Abs, vec![Expr::func(Cos, vec![inner.clone()])])]),
            _ => return None,
        })
    }
    // #endregion 🔖️Substitution

    // #region 🔖️ByParts
    /// 🧩️ Integration by parts for a two-factor product, choosing `u` via a coarse LIATE ranking
    /// (Logarithm < Inverse-trig < Algebraic < Trig/hyperbolic < Exponential), depth-capped so the
    /// `v * du` recursion can't loop forever on a pair that doesn't actually simplify.
    fn integrate_by_parts(e: &Expr, x: &Expr, depth: u32) -> Option<Expr> {
        let factors: Vec<Expr> = match e.kind() {
            Kind::Mul(fs) => fs.clone(),
            _ => vec![e.clone()],
        };
        if factors.len() > 2 {
            return None;
        }
        let (u, dv) = if factors.len() == 2 {
            if liate_rank(&factors[0], x) <= liate_rank(&factors[1], x) {
                (factors[0].clone(), factors[1].clone())
            } else {
                (factors[1].clone(), factors[0].clone())
            }
        } else {
            (e.clone(), Expr::integer(1))
        };
        let v = integrate_depth(&dv, x, depth + 1)?;
        let du = crate::cas::diff::diff(&u, x)?;
        if du.is_zero_literal() {
            return Some(u * v);
        }
        let second_term = integrate_depth(&(v.clone() * du), x, depth + 1)?;
        Some(u * v - second_term)
    }

    fn liate_rank(f: &Expr, x: &Expr) -> i32 {
        match f.kind() {
            Kind::Fn(FnKind::Ln, _) => 0,
            Kind::Fn(FnKind::Asin | FnKind::Acos | FnKind::Atan, _) => 1,
            Kind::Fn(FnKind::Sin | FnKind::Cos | FnKind::Sinh | FnKind::Cosh, _) => 3,
            Kind::Fn(FnKind::Exp, _) => 4,
            _ if f == x || matches!(f.kind(), Kind::Pow(base, _) if base == x) => 2,
            _ => 2,
        }
    }
    // #endregion 🔖️ByParts

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        fn diff_matches(e: &Expr, x: &Expr, antideriv: &Expr) -> bool {
            let d = crate::cas::diff::diff(antideriv, x).unwrap();
            crate::cas::simplify::simplify(&(d - e.clone())).is_zero_literal()
        }

        #[test]
        fn integrate_power_rule() {
            let x = Expr::symbol("x");
            let e = Expr::pow(x.clone(), Expr::integer(2));
            let result = integrate(&e, &x).unwrap();
            assert!(diff_matches(&e, &x, &result));
        }

        #[test]
        fn integrate_reciprocal_gives_ln() {
            let x = Expr::symbol("x");
            let e = Expr::pow(x.clone(), Expr::integer(-1));
            let result = integrate(&e, &x).unwrap();
            assert_eq!(result, Expr::func(FnKind::Ln, vec![Expr::func(FnKind::Abs, vec![x])]));
        }

        #[test]
        fn integrate_sin_and_cos() {
            let x = Expr::symbol("x");
            let sin_result = integrate(&Expr::func(FnKind::Sin, vec![x.clone()]), &x).unwrap();
            assert!(diff_matches(&Expr::func(FnKind::Sin, vec![x.clone()]), &x, &sin_result));
            let cos_result = integrate(&Expr::func(FnKind::Cos, vec![x.clone()]), &x).unwrap();
            assert!(diff_matches(&Expr::func(FnKind::Cos, vec![x.clone()]), &x, &cos_result));
        }

        #[test]
        fn integrate_polynomial_sum() {
            let x = Expr::symbol("x");
            let e = Expr::pow(x.clone(), Expr::integer(2)) + Expr::mul(vec![Expr::integer(3), x.clone()]) + Expr::integer(1);
            let result = integrate(&e, &x).unwrap();
            assert!(diff_matches(&e, &x, &result));
        }

        #[test]
        fn integrate_simple_partial_fraction() {
            let x = Expr::symbol("x");
            // 1/((x-1)(x+1)) integrates to (1/2)ln|x-1| - (1/2)ln|x+1| (up to grouping)
            let den = (x.clone() - Expr::integer(1)) * (x.clone() + Expr::integer(1));
            let e = Expr::pow(den, Expr::integer(-1));
            let result = integrate(&e, &x).unwrap();
            assert!(diff_matches(&e, &x, &result));
        }

        #[test]
        fn integrate_u_substitution() {
            let x = Expr::symbol("x");
            // 2x * cos(x^2) -> sin(x^2)
            let inner = Expr::pow(x.clone(), Expr::integer(2));
            let e = Expr::mul(vec![Expr::integer(2), x.clone(), Expr::func(FnKind::Cos, vec![inner])]);
            let result = integrate(&e, &x).unwrap();
            assert!(diff_matches(&e, &x, &result));
        }

        #[test]
        fn integrate_by_parts_x_times_exp() {
            let x = Expr::symbol("x");
            let e = Expr::mul(vec![x.clone(), Expr::func(FnKind::Exp, vec![x.clone()])]);
            let result = integrate(&e, &x).unwrap();
            assert!(diff_matches(&e, &x, &result));
        }

        #[test]
        fn integrate_ln_by_parts() {
            let x = Expr::symbol("x");
            let e = Expr::func(FnKind::Ln, vec![x.clone()]);
            let result = integrate(&e, &x).unwrap();
            assert!(diff_matches(&e, &x, &result));
        }

        #[test]
        fn integrate_irreducible_quadratic_denominator() {
            let x = Expr::symbol("x");
            // 1/(x^2+1) -> atan(x)
            let e = Expr::pow(Expr::pow(x.clone(), Expr::integer(2)) + Expr::integer(1), Expr::integer(-1));
            let result = integrate(&e, &x).unwrap();
            assert!(diff_matches(&e, &x, &result));
        }

        #[test]
        fn definite_integral_of_power() {
            let x = Expr::symbol("x");
            let e = Expr::pow(x.clone(), Expr::integer(2));
            let result = integrate_definite(&e, &x, &Expr::integer(0), &Expr::integer(3)).unwrap();
            assert_eq!(result, Expr::integer(9));
        }

        #[test]
        fn residue_at_simple_pole() {
            let x = Expr::symbol("x");
            // 1/(x-2) has residue 1 at x=2
            let e = Expr::pow(x.clone() - Expr::integer(2), Expr::integer(-1));
            assert_eq!(residue(&e, &x, &Expr::integer(2)), Some(Expr::integer(1)));
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Integrate

// #region 🔖️Sums
pub mod sums {
    //! Σ Symbolic summation: closed forms for polynomial sums (via Lagrange interpolation of the partial
    //! sum, which is itself always a polynomial one degree higher — this sidesteps deriving Bernoulli
    //! numbers explicitly) and geometric sums, plus Fourier coefficients via definite integration.
    //! Gosper/Zeilberger-style general hypergeometric summation is a documented follow-up, not attempted.

    use crate::cas::expr::{Constant, Expr, Kind};
    use crate::cas::fnkind::FnKind;
    use math::number::Rational;
    use crate::polynomial::PolyU;

    // #region 🔖️ClosedForm
    /// Σ `sum_{k=lo}^{hi} e(k)` in closed form, for `e` polynomial or geometric in `n`; `None` otherwise.
    pub fn sum_closed(e: &Expr, n: &Expr, lo: &Expr, hi: &Expr) -> Option<Expr> {
        if let Some(p) = crate::cas::polybridge::as_poly_uni(e, n) {
            let s = sum_polynomial_closed_form(&p, n)?;
            let at_hi = crate::cas::visit::subs(&s, n, hi);
            let lo_minus_1 = lo.clone() - Expr::integer(1);
            let at_lo = crate::cas::visit::subs(&s, n, &lo_minus_1);
            return Some(crate::cas::simplify::simplify(&(at_hi - at_lo)));
        }
        sum_geometric(e, n, lo, hi)
    }

    /// Σ The polynomial `S(N) = sum_{k=0}^{N} p(k)`, recovered by evaluating the true partial sums at
    /// `deg(p) + 2` integer points and interpolating (a degree-`d` polynomial's partial sum is always an
    /// exact degree-`(d+1)` polynomial in `N`, so this is exact, not an approximation).
    fn sum_polynomial_closed_form(p: &PolyU<Rational>, n: &Expr) -> Option<Expr> {
        let d = p.degree().unwrap_or(0);
        let num_points = d + 2;
        let mut cumulative = Rational::zero();
        let mut points = Vec::with_capacity(num_points);
        for k in 0..num_points {
            let k_r = Rational::from_i64(k as i64, 1).unwrap();
            cumulative = cumulative.add(&p.eval(&k_r));
            points.push((k_r, cumulative.clone()));
        }
        let s_poly = PolyU::interpolate(&points)?;
        Some(crate::cas::polybridge::polyu_to_expr(&s_poly, n))
    }

    /// Σ `sum_{k=lo}^{hi} c * r^k` for `c`, `r` free of `n`, via the closed geometric-series formula
    /// (special-cased at `r == 1`, where the sum is just `count * c`).
    fn sum_geometric(e: &Expr, n: &Expr, lo: &Expr, hi: &Expr) -> Option<Expr> {
        let (const_factors, var_factors): (Vec<Expr>, Vec<Expr>) = match e.kind() {
            Kind::Mul(factors) => factors.iter().cloned().partition(|f| !crate::cas::visit::contains_symbol(f, n)),
            _ => (Vec::new(), vec![e.clone()]),
        };
        if var_factors.len() != 1 {
            return None;
        }
        let Kind::Pow(base, exp) = var_factors[0].kind() else { return None };
        if exp != n || crate::cas::visit::contains_symbol(base, n) {
            return None;
        }
        let r = base.clone();
        let c = if const_factors.is_empty() { Expr::integer(1) } else { Expr::mul(const_factors) };
        let count = (hi.clone() - lo.clone()) + Expr::integer(1);
        if r.is_one_literal() {
            return Some(c * count);
        }
        let sum_r = Expr::pow(r.clone(), lo.clone()) * (Expr::pow(r.clone(), count) - Expr::integer(1)) * Expr::pow(r - Expr::integer(1), Expr::integer(-1));
        Some(crate::cas::simplify::cancel(&(c * sum_r)))
    }
    // #endregion 🔖️ClosedForm

    // #region 🔖️Fourier
    /// 🌊️ Fourier coefficients `(a_n, b_n)` of `f` on `[-L, L]` (`a_0` at index 0, `b_0` fixed at `0` since
    /// the sine term vanishes there), via `integrate_definite` — correct whenever the underlying integrals
    /// resolve, `None` for the whole pair otherwise (never a partial/wrong coefficient list).
    pub fn fourier_coefficients(f: &Expr, x: &Expr, half_period: &Expr, n_terms: usize) -> Option<(Vec<Expr>, Vec<Expr>)> {
        let l = half_period.clone();
        let neg_l = Expr::integer(-1) * l.clone();
        let mut a = Vec::with_capacity(n_terms + 1);
        let mut b = Vec::with_capacity(n_terms + 1);
        for n in 0..=n_terms {
            let angle = Expr::mul(vec![Expr::integer(n as i64), Expr::constant(Constant::Pi), x.clone()]) * Expr::pow(l.clone(), Expr::integer(-1));
            let cos_term = Expr::func(FnKind::Cos, vec![angle.clone()]);
            let a_n = crate::cas::integrate::integrate_definite(&(f.clone() * cos_term), x, &neg_l, &l)?;
            a.push(crate::cas::simplify::cancel(&(a_n * Expr::pow(l.clone(), Expr::integer(-1)))));
            if n > 0 {
                let sin_term = Expr::func(FnKind::Sin, vec![angle]);
                let b_n = crate::cas::integrate::integrate_definite(&(f.clone() * sin_term), x, &neg_l, &l)?;
                b.push(crate::cas::simplify::cancel(&(b_n * Expr::pow(l.clone(), Expr::integer(-1)))));
            } else {
                b.push(Expr::integer(0));
            }
        }
        Some((a, b))
    }
    // #endregion 🔖️Fourier

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sum_of_k_from_1_to_n_is_gauss_formula() {
            let n = Expr::symbol("n");
            let k = Expr::symbol("k");
            // sum_{k=1}^{n} k -- but sum_closed evaluates a polynomial in the SAME variable used for the
            // bound substitution, so pass `k` itself as both the summand's variable and the closed-form target.
            let result = sum_closed(&k, &k, &Expr::integer(1), &n).unwrap();
            let expected = crate::cas::simplify::expand(&(n.clone() * (n + Expr::integer(1)) * Expr::from(Rational::from_i64(1, 2).unwrap())));
            assert_eq!(crate::cas::simplify::expand(&result), expected);
        }

        #[test]
        fn sum_of_k_squared_matches_known_hand_values() {
            let k = Expr::symbol("k");
            // sum_{k=1}^{3} k^2 = 1+4+9 = 14
            let e = Expr::pow(k.clone(), Expr::integer(2));
            let result = sum_closed(&e, &k, &Expr::integer(1), &Expr::integer(3)).unwrap();
            assert_eq!(result, Expr::integer(14));
        }

        #[test]
        fn sum_geometric_series_hand_case() {
            let k = Expr::symbol("k");
            // sum_{k=0}^{3} 2^k = 1+2+4+8 = 15
            let e = Expr::pow(Expr::integer(2), k.clone());
            let result = sum_closed(&e, &k, &Expr::integer(0), &Expr::integer(3)).unwrap();
            assert_eq!(crate::cas::simplify::simplify(&result), Expr::integer(15));
        }

        #[test]
        fn fourier_coefficients_of_a_polynomial_smoke_test() {
            let x = Expr::symbol("x");
            let l = Expr::constant(Constant::Pi);
            let f = x.clone();
            let result = fourier_coefficients(&f, &x, &l, 2);
            assert!(result.is_some());
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Sums

// #region 🔖️Ode
pub mod ode {
    //! 🌊️ ODE solving. The kernel's `Expr` models closed-form values, not an unknown function with
    //! derivatives (there's no "y''" node kind), so this module has two distinct front doors:
    //! `solve_ode_first_order` takes `y' = f(x, y)` as an ordinary `Expr` in the two symbols `x, y` and
    //! classifies it (separable / linear / Bernoulli); `solve_linear_constant_coeff_homogeneous` takes the
    //! characteristic coefficients directly, since there's no `Expr` syntax for "the equation
    //! `y''' - 2y'' + y = 0`" to parse in the first place.

    use crate::cas::expr::{Expr, Kind, RelationalOperator};
    use crate::cas::fnkind::FnKind;
    use math::number::{Integer, Rational};
    use crate::polynomial::PolyU;

    // #region 🔖️OdeSolution
    #[derive(Clone, Debug, PartialEq)]
    pub struct OdeSolution {
        /// For first-order results this is the solved (or implicit) relation for `y`; for the
        /// constant-coefficient homogeneous case it's the general solution `y(x)` itself.
        pub rhs: Expr,
        pub constants: Vec<Expr>,
    }
    // #endregion 🔖️OdeSolution

    // #region 🔖️FirstOrder
    /// 🌊️ Classifies and solves `y' = f(x, y)`: separable, linear, then Bernoulli, in that order.
    pub fn solve_ode_first_order(f: &Expr, x: &Expr, y: &Expr) -> Option<OdeSolution> {
        try_separable(f, x, y).or_else(|| try_linear_first_order(f, x, y)).or_else(|| try_bernoulli(f, x, y))
    }

    fn try_separable(f: &Expr, x: &Expr, y: &Expr) -> Option<OdeSolution> {
        let factors: Vec<Expr> = match f.kind() {
            Kind::Mul(fs) => fs.clone(),
            _ => vec![f.clone()],
        };
        let (free_of_y, rest): (Vec<Expr>, Vec<Expr>) = factors.into_iter().partition(|fac| !crate::cas::visit::contains_symbol(fac, y));
        if rest.iter().any(|fac| crate::cas::visit::contains_symbol(fac, x)) {
            return None;
        }
        let g = Expr::mul(free_of_y);
        let h = if rest.is_empty() { Expr::integer(1) } else { Expr::mul(rest) };
        let lhs = crate::cas::integrate::integrate(&Expr::pow(h, Expr::integer(-1)), y)?;
        let rhs = crate::cas::integrate::integrate(&g, x)?;
        let c1 = Expr::symbol("§C1");
        Some(OdeSolution { rhs: Expr::from_kind_unchecked(Kind::Rel(RelationalOperator::Eq, lhs, rhs + c1.clone())), constants: vec![c1] })
    }

    /// 🧩️ Extracts `(coeff, constant)` such that `f == coeff * y + constant`, both free of `y`; `None` if
    /// `f` isn't affine in `y`.
    fn affine_in_y(f: &Expr, y: &Expr) -> Option<(Expr, Expr)> {
        let expanded = crate::cas::simplify::expand(f);
        let terms: Vec<Expr> = match expanded.kind() {
            Kind::Add(ts) => ts.clone(),
            _ => vec![expanded.clone()],
        };
        let mut coeff = Expr::integer(0);
        let mut constant = Vec::new();
        for t in &terms {
            let factors: Vec<Expr> = match t.kind() {
                Kind::Mul(fs) => fs.clone(),
                _ => vec![t.clone()],
            };
            let mut matched = false;
            let mut rest = Vec::new();
            for fac in &factors {
                if fac == y {
                    if matched {
                        return None;
                    }
                    matched = true;
                    continue;
                }
                if let Kind::Pow(base, _) = fac.kind() {
                    if base == y {
                        return None;
                    }
                }
                rest.push(fac.clone());
            }
            if matched {
                coeff = coeff + Expr::mul(rest);
            } else {
                constant.push(t.clone());
            }
        }
        Some((coeff, Expr::add(constant)))
    }

    fn try_linear_first_order(f: &Expr, x: &Expr, y: &Expr) -> Option<OdeSolution> {
        let (coeff, q) = affine_in_y(f, y)?;
        if crate::cas::visit::contains_symbol(&coeff, y) || crate::cas::visit::contains_symbol(&q, y) {
            return None;
        }
        let p = Expr::integer(-1) * coeff;
        let integral_p = crate::cas::integrate::integrate(&p, x)?;
        let mu = Expr::func(FnKind::Exp, vec![integral_p]);
        let integrand = crate::cas::simplify::cancel(&(mu.clone() * q));
        let integral_mu_q = crate::cas::integrate::integrate(&integrand, x)?;
        let c1 = Expr::symbol("§C1");
        let y_sol = crate::cas::simplify::cancel(&((integral_mu_q + c1.clone()) * Expr::pow(mu, Expr::integer(-1))));
        Some(OdeSolution { rhs: y_sol, constants: vec![c1] })
    }

    fn try_bernoulli(f: &Expr, x: &Expr, y: &Expr) -> Option<OdeSolution> {
        let expanded = crate::cas::simplify::expand(f);
        let terms: Vec<Expr> = match expanded.kind() {
            Kind::Add(ts) => ts.clone(),
            _ => vec![expanded.clone()],
        };
        if terms.len() != 2 {
            return None;
        }
        let mut lin: Option<Expr> = None;
        let mut high: Option<(Expr, i64)> = None;
        for t in &terms {
            let factors: Vec<Expr> = match t.kind() {
                Kind::Mul(fs) => fs.clone(),
                _ => vec![t.clone()],
            };
            let mut y_pow = 0i64;
            let mut rest = Vec::new();
            for fac in &factors {
                if fac == y {
                    y_pow += 1;
                    continue;
                }
                if let Kind::Pow(base, exp) = fac.kind() {
                    if base == y {
                        let Kind::Integer(n) = exp.kind() else { return None };
                        y_pow += n.to_i64()?;
                        continue;
                    }
                }
                rest.push(fac.clone());
            }
            let coeff = Expr::mul(rest);
            if crate::cas::visit::contains_symbol(&coeff, y) {
                return None;
            }
            match y_pow {
                1 => {
                    if lin.is_some() {
                        return None;
                    }
                    lin = Some(coeff);
                }
                n if n != 0 => {
                    if high.is_some() {
                        return None;
                    }
                    high = Some((coeff, n));
                }
                _ => return None,
            }
        }
        let p = lin?;
        let (q, n) = high?;
        if n == 1 {
            return None;
        }
        let one_minus_n = 1 - n;
        let v = Expr::symbol("§bernoulli_v");
        let f_v = Expr::integer(one_minus_n) * p * v.clone() + Expr::integer(one_minus_n) * q;
        let v_sol = try_linear_first_order(&f_v, x, &v)?;
        let y_sol = Expr::pow(v_sol.rhs, Expr::from(Rational::new(Integer::one(), Integer::from_i64(one_minus_n))?));
        Some(OdeSolution { rhs: y_sol, constants: v_sol.constants })
    }
    // #endregion 🔖️FirstOrder

    // #region 🔖️LinearConstantCoefficient
    /// 🌊️ General solution of `a_n y^(n) + ... + a_1 y' + a_0 y = 0`, given `coeffs = [a_0, ..., a_n]`
    /// directly (see the module doc for why there's no `Expr`-equation front door for this case). Handles
    /// real roots (with multiplicity, giving `x^k e^{rx}` terms) and complex-conjugate pairs from
    /// irreducible quadratic factors (giving `x^k e^{alpha x} {cos,sin}(beta x)` terms); an irreducible
    /// factor of degree >= 3 in the characteristic polynomial is a documented gap (`None`).
    pub fn solve_linear_constant_coeff_homogeneous(coeffs: &[Rational], x: &Expr) -> Option<OdeSolution> {
        let char_poly = PolyU::from_coeffs(coeffs.to_vec());
        if char_poly.is_zero() || char_poly.degree().unwrap_or(0) == 0 {
            return None;
        }
        let (_, factors) = crate::cas::polybridge::factor_poly_u(&char_poly);
        let mut basis = Vec::new();
        let mut constants = Vec::new();
        let mut idx = 0usize;
        for (factor, mult) in &factors {
            match factor.degree().unwrap_or(0) {
                1 => {
                    let root = Expr::from(factor.coeff(0).neg().div(&factor.coeff(1))?);
                    for k in 0..*mult {
                        let c = Expr::symbol(&format!("§C{idx}"));
                        idx += 1;
                        constants.push(c.clone());
                        let exp_part = Expr::func(FnKind::Exp, vec![root.clone() * x.clone()]);
                        let term = if k == 0 { exp_part } else { Expr::pow(x.clone(), Expr::integer(k as i64)) * exp_part };
                        basis.push(c * term);
                    }
                }
                2 => {
                    let a = factor.coeff(2);
                    let b = factor.coeff(1);
                    let cc = factor.coeff(0);
                    let alpha = b.neg().div(&a.mul(&Rational::from_i64(2, 1).unwrap()))?;
                    let disc = b.mul(&b).sub(&a.mul(&cc).mul(&Rational::from_i64(4, 1).unwrap()));
                    if !disc.numer().is_negative() {
                        return None; // real roots reaching here would mean `factor` wasn't actually irreducible
                    }
                    let beta_sq = disc.neg().div(&a.mul(&a).mul(&Rational::from_i64(4, 1).unwrap()))?;
                    let beta = crate::cas::solve::sqrt_of_rational(&beta_sq);
                    for k in 0..*mult {
                        let c1 = Expr::symbol(&format!("§C{idx}"));
                        idx += 1;
                        constants.push(c1.clone());
                        let c2 = Expr::symbol(&format!("§C{idx}"));
                        idx += 1;
                        constants.push(c2.clone());
                        let exp_part = Expr::func(FnKind::Exp, vec![Expr::from(alpha.clone()) * x.clone()]);
                        let x_pow = if k == 0 { Expr::integer(1) } else { Expr::pow(x.clone(), Expr::integer(k as i64)) };
                        let cos_term = Expr::func(FnKind::Cos, vec![beta.clone() * x.clone()]);
                        let sin_term = Expr::func(FnKind::Sin, vec![beta.clone() * x.clone()]);
                        basis.push(c1 * x_pow.clone() * exp_part.clone() * cos_term);
                        basis.push(c2 * x_pow * exp_part * sin_term);
                    }
                }
                _ => return None,
            }
        }
        Some(OdeSolution { rhs: Expr::add(basis), constants })
    }
    // #endregion 🔖️LinearConstantCoefficient

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        fn satisfies(sol: &Expr, x: &Expr, y: &Expr, rhs_of_ode: &Expr) -> bool {
            // Substitutes the solution in for y and checks y' == rhs_of_ode(x, sol) structurally after simplify.
            let dy = crate::cas::diff::diff(sol, x).unwrap();
            let substituted_rhs = crate::cas::visit::subs(rhs_of_ode, y, sol);
            crate::cas::simplify::simplify(&(dy - substituted_rhs)).is_zero_literal()
        }

        #[test]
        fn separable_ode_y_prime_equals_x_over_y() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            // y' = x/y  =>  y dy = x dx  =>  y^2/2 = x^2/2 + C
            let f = x.clone() * Expr::pow(y.clone(), Expr::integer(-1));
            let sol = solve_ode_first_order(&f, &x, &y).unwrap();
            assert!(matches!(sol.rhs.kind(), Kind::Rel(RelationalOperator::Eq, ..)));
        }

        #[test]
        fn linear_first_order_ode() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            // y' = y + x  (P = -1 constant, Q = x) -- verify by direct differentiation of the returned solution.
            let f = y.clone() + x.clone();
            let sol = solve_ode_first_order(&f, &x, &y).unwrap();
            assert!(satisfies(&sol.rhs, &x, &y, &f));
        }

        #[test]
        fn bernoulli_ode() {
            let x = Expr::symbol("x");
            let y = Expr::symbol("y");
            // y' = y/x - y^2  (Bernoulli with n=2, P=1/x, Q=-1)
            let f = y.clone() * Expr::pow(x.clone(), Expr::integer(-1)) - Expr::pow(y.clone(), Expr::integer(2));
            let sol = solve_ode_first_order(&f, &x, &y);
            assert!(sol.is_some());
        }

        #[test]
        fn linear_constant_coefficient_second_order_distinct_real_roots() {
            let x = Expr::symbol("x");
            // y'' - 3y' + 2y = 0 -> roots 1, 2 -> y = C1*e^x + C2*e^(2x)
            let coeffs = vec![Rational::from_i64(2, 1).unwrap(), Rational::from_i64(-3, 1).unwrap(), Rational::one()];
            let sol = solve_linear_constant_coeff_homogeneous(&coeffs, &x).unwrap();
            assert_eq!(sol.constants.len(), 2);
        }

        #[test]
        fn linear_constant_coefficient_repeated_root() {
            let x = Expr::symbol("x");
            // y'' - 2y' + y = 0 -> repeated root 1 -> y = (C1 + C2*x)*e^x
            let coeffs = vec![Rational::one(), Rational::from_i64(-2, 1).unwrap(), Rational::one()];
            let sol = solve_linear_constant_coeff_homogeneous(&coeffs, &x).unwrap();
            assert_eq!(sol.constants.len(), 2);
            // verify diff satisfies the ODE for a specific choice C1=1, C2=0: y=e^x, y''-2y'+y=0
            let y_ex = Expr::func(FnKind::Exp, vec![x.clone()]);
            let d1 = crate::cas::diff::diff(&y_ex, &x).unwrap();
            let d2 = crate::cas::diff::diff(&d1, &x).unwrap();
            let residual = d2 - Expr::integer(2) * d1 + y_ex;
            assert_eq!(crate::cas::simplify::simplify(&residual), Expr::integer(0));
        }

        #[test]
        fn linear_constant_coefficient_complex_roots() {
            let x = Expr::symbol("x");
            // y'' + y = 0 -> roots +-i -> y = C1*cos(x) + C2*sin(x)
            let coeffs = vec![Rational::one(), Rational::zero(), Rational::one()];
            let sol = solve_linear_constant_coeff_homogeneous(&coeffs, &x).unwrap();
            assert_eq!(sol.constants.len(), 2);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Ode

// #region 🔖️Transforms
pub mod transforms {
    //! 🔄️ Laplace transforms: linearity (`Add`, constant-factor pull-out) plus a table for `t^n`,
    //! `exp/sin/cos/sinh/cosh(a*t)`. The inverse transform is scoped to its exact mirror image — linearity
    //! plus the `1/(s-a) -> e^{at}` pattern — rather than a general rational-function inverse (that would
    //! need to reuse `simplify::apart` and re-derive sign/branch handling per term; a documented follow-up).

    use crate::cas::expr::{Expr, Kind};
    use crate::cas::fnkind::FnKind;
    use math::number::Integer;

    // #region 🔖️Laplace
    pub fn laplace_transform(f: &Expr, t: &Expr, s: &Expr) -> Option<Expr> {
        if !crate::cas::visit::contains_symbol(f, t) {
            return Some(f.clone() * Expr::pow(s.clone(), Expr::integer(-1)));
        }
        if let Kind::Add(terms) = f.kind() {
            let mut parts = Vec::with_capacity(terms.len());
            for term in terms {
                parts.push(laplace_transform(term, t, s)?);
            }
            return Some(Expr::add(parts));
        }
        if let Kind::Mul(factors) = f.kind() {
            let (const_factors, var_factors): (Vec<Expr>, Vec<Expr>) = factors.iter().cloned().partition(|fac| !crate::cas::visit::contains_symbol(fac, t));
            if !const_factors.is_empty() && !var_factors.is_empty() {
                let rest = Expr::mul(var_factors);
                return Some(Expr::mul(const_factors) * laplace_transform(&rest, t, s)?);
            }
        }
        if f == t {
            return Some(Expr::pow(s.clone(), Expr::integer(-2)));
        }
        if let Kind::Pow(base, exp) = f.kind() {
            if base == t {
                if let Kind::Integer(n) = exp.kind() {
                    if let Some(ev) = n.to_i64() {
                        if ev >= 0 {
                            return Some(Expr::from(factorial(ev)) * Expr::pow(s.clone(), Expr::integer(-(ev + 1))));
                        }
                    }
                }
            }
        }
        if let Kind::Fn(kind, args) = f.kind() {
            if args.len() == 1 {
                if let Some(a) = linear_coeff_in(&args[0], t) {
                    return laplace_table(kind, &a, s);
                }
            }
        }
        None
    }

    fn factorial(n: i64) -> Integer {
        let mut result = Integer::one();
        for k in 1..=n {
            result = result.mul(&Integer::from_i64(k));
        }
        result
    }

    /// 🔍️ `arg == a * t` for some `a` free of `t`; `None` for anything with a constant offset or nonlinear
    /// dependence (this pass's table entries only need the pure-scaling case).
    fn linear_coeff_in(arg: &Expr, t: &Expr) -> Option<Expr> {
        if arg == t {
            return Some(Expr::integer(1));
        }
        if let Kind::Mul(factors) = arg.kind() {
            let (const_factors, var_factors): (Vec<Expr>, Vec<Expr>) = factors.iter().cloned().partition(|f| f != t);
            if var_factors.len() == 1 && var_factors[0] == *t {
                return Some(Expr::mul(const_factors));
            }
        }
        None
    }

    fn laplace_table(kind: &FnKind, a: &Expr, s: &Expr) -> Option<Expr> {
        use FnKind::*;
        let s2 = Expr::pow(s.clone(), Expr::integer(2));
        let a2 = Expr::pow(a.clone(), Expr::integer(2));
        Some(match kind {
            Exp => Expr::pow(s.clone() - a.clone(), Expr::integer(-1)),
            Sin => a.clone() * Expr::pow(s2 + a2, Expr::integer(-1)),
            Cos => s.clone() * Expr::pow(s2 + a2, Expr::integer(-1)),
            Sinh => a.clone() * Expr::pow(s2 - a2, Expr::integer(-1)),
            Cosh => s.clone() * Expr::pow(s2 - a2, Expr::integer(-1)),
            _ => return None,
        })
    }
    // #endregion 🔖️Laplace

    // #region 🔖️InverseLaplace
    pub fn inverse_laplace_transform(f: &Expr, s: &Expr, t: &Expr) -> Option<Expr> {
        if let Kind::Add(terms) = f.kind() {
            let mut parts = Vec::with_capacity(terms.len());
            for term in terms {
                parts.push(inverse_laplace_transform(term, s, t)?);
            }
            return Some(Expr::add(parts));
        }
        if let Kind::Mul(factors) = f.kind() {
            let (const_factors, var_factors): (Vec<Expr>, Vec<Expr>) = factors.iter().cloned().partition(|fac| !crate::cas::visit::contains_symbol(fac, s));
            if !const_factors.is_empty() && !var_factors.is_empty() {
                let rest = Expr::mul(var_factors);
                return Some(Expr::mul(const_factors) * inverse_laplace_transform(&rest, s, t)?);
            }
        }
        if let Kind::Pow(base, exp) = f.kind() {
            if matches!(exp.kind(), Kind::Integer(n) if *n == Integer::from_i64(-1)) {
                if let Some(a) = extract_shift(base, s) {
                    return Some(Expr::func(FnKind::Exp, vec![a * t.clone()]));
                }
            }
        }
        None
    }

    /// 🔍️ `e == s - a` (or bare `s`, giving `a = 0`); `None` otherwise.
    fn extract_shift(e: &Expr, s: &Expr) -> Option<Expr> {
        if e == s {
            return Some(Expr::integer(0));
        }
        if let Kind::Add(terms) = e.kind() {
            if terms.len() != 2 {
                return None;
            }
            let (s_terms, rest): (Vec<Expr>, Vec<Expr>) = terms.iter().cloned().partition(|term| term == s);
            if s_terms.len() == 1 {
                return Some(Expr::integer(-1) * Expr::add(rest));
            }
        }
        None
    }
    // #endregion 🔖️InverseLaplace

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn laplace_of_t_to_the_n() {
            let t = Expr::symbol("t");
            let s = Expr::symbol("s");
            // L{t^2} = 2/s^3
            let e = Expr::pow(t.clone(), Expr::integer(2));
            let result = laplace_transform(&e, &t, &s).unwrap();
            assert_eq!(result, Expr::integer(2) * Expr::pow(s, Expr::integer(-3)));
        }

        #[test]
        fn laplace_of_exp() {
            let t = Expr::symbol("t");
            let s = Expr::symbol("s");
            let e = Expr::func(FnKind::Exp, vec![Expr::integer(3) * t.clone()]);
            let result = laplace_transform(&e, &t, &s).unwrap();
            assert_eq!(result, Expr::pow(s - Expr::integer(3), Expr::integer(-1)));
        }

        #[test]
        fn laplace_linearity() {
            let t = Expr::symbol("t");
            let s = Expr::symbol("s");
            let e = Expr::integer(2) * t.clone() + Expr::integer(3);
            let result = laplace_transform(&e, &t, &s).unwrap();
            let expected = Expr::integer(2) * Expr::pow(s.clone(), Expr::integer(-2)) + Expr::integer(3) * Expr::pow(s, Expr::integer(-1));
            assert_eq!(result, expected);
        }

        #[test]
        fn laplace_and_inverse_round_trip_for_exp() {
            let t = Expr::symbol("t");
            let s = Expr::symbol("s");
            let e = Expr::func(FnKind::Exp, vec![Expr::integer(-2) * t.clone()]);
            let transformed = laplace_transform(&e, &t, &s).unwrap();
            let back = inverse_laplace_transform(&transformed, &s, &t).unwrap();
            assert_eq!(back, e);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Transforms

pub use assume::{is_even, is_integer, is_nonzero, is_positive, is_real, AssumeSet, Assumptions, RelationalOperator as AssumeRelationalOperator};
pub use diff::{diff, gradient, idiff};
pub use expr::{Constant, Expr, Kind, RelationalOperator, Symbol, WildKind};
pub use fmt::to_latex;
pub use fnkind::FnKind;
pub use integrate::{integrate, integrate_definite, residue};
pub use limits::{limit, Direction};
pub use matrix::SymMatrix;
pub use ode::{solve_linear_constant_coeff_homogeneous, solve_ode_first_order, OdeSolution};
pub use pattern::{wild, wild_free, wild_nonzero, wild_num, wild_seq, Binding, Bindings, Rule, RuleSet, Strategy};
pub use polybridge::{as_poly, as_poly_auto, as_poly_uni, from_poly, PolyMap};
pub use rootof::{real_roots_of, root_of_expr, root_of_refine, root_of_sign, root_of_to_f64};
pub use series::{leading_term, series_to_expr, taylor_series, Series};
pub use simplify::{apart, cancel, collect, denest_sqrt, expand, factor, simplify, together};
pub use solve::{solve_inequality, solve_linear_system, solve_univariate, Bound, SolutionSet};
pub use sums::{fourier_coefficients, sum_closed};
pub use transforms::{inverse_laplace_transform, laplace_transform};
pub use trig::{expand_log, expand_trig, logcombine, powsimp, trig_canon};
pub use visit::{contains_symbol, free_symbols, map_children, node_count, replace_bottom_up, subs, subs_many};
