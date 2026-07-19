//! ✨ The symbolic expression tree: an immutable, `Rc`-shared, structurally-hash-cached tree. Every
//! node is built exclusively through `canon.rs`'s smart constructors, which maintain the
//! auto-simplification invariants (flattening, numeric folding, canonical ordering) that the rest of
//! the crate assumes hold everywhere.

use crate::assume::AssumeSet;
use crate::fnkind::FnKind;
use mathematical_number::{Integer, Rational};
use std::rc::Rc;

// #region 🔖Symbol
/// 🔤 A named variable plus the assumption flags fixed at its creation — two symbols with the same
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
// #endregion 🔖Symbol

// #region 🔖Constant
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
// #endregion 🔖Constant

// #region 🔖RelOp
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum RelOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
// #endregion 🔖RelOp

// #region 🔖WildKind
#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum WildKind {
    Any,
    Number,
    NotZero,
    /// 🆓 Matches only subtrees free of the named symbol.
    FreeOf(Rc<str>),
    /// 📚 Matches a whole sequence of `Add`/`Mul` arguments (at most one per pattern).
    Seq,
}
// #endregion 🔖WildKind

// #region 🔖Kind
#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
    Integer(Integer),
    /// ➗ Invariant (maintained by `canon.rs`): never has an integral value — those fold to `Integer`.
    Rational(Rational),
    Symbol(Symbol),
    Constant(Constant),
    Bool(bool),
    /// ➕ Flattened, canonically sorted, length `>= 2`; a numeric coefficient (if any) sorts first.
    Add(Vec<Expr>),
    /// ✖️ Same invariants as `Add`.
    Mul(Vec<Expr>),
    Pow(Expr, Expr),
    Fn(FnKind, Vec<Expr>),
    /// 🌱 The `index`-th real root (ascending) of the integer polynomial with coefficients `coeffs`
    /// (little-endian, degree = `coeffs.len() - 1`); used when `solve` can't produce a closed radical form.
    RootOf { coeffs: Vec<Rational>, index: u32 },
    /// 🔀 `(value, condition)` pairs; the last condition may be `Bool(true)` for a catch-all default.
    Piecewise(Vec<(Expr, Expr)>),
    Rel(RelOp, Expr, Expr),
    /// 🃏 Pattern-only placeholder; must never appear in a user-facing expression.
    Wild(u16, WildKind),
}
// #endregion 🔖Kind

// #region 🔖Node
struct Node {
    hash: u64,
    kind: Kind,
}

/// 🔢 FNV-1a, computed bottom-up once at construction and cached — equality checks hash first (cheap
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
        Kind::Rel(op, a, b) => {
            h = fnv1a_mix(h, b"rel");
            h = fnv1a_mix(h, &[*op as u8]);
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
// #endregion 🔖Node

// #region 🔖Expr
/// ✨ An immutable, reference-counted symbolic expression. `!Send`/`!Sync` by design (single-threaded,
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
        crate::canon::make_symbol(name, AssumeSet::NONE)
    }

    pub fn symbol_with(name: &str, assumptions: AssumeSet) -> Self {
        crate::canon::make_symbol(name, assumptions)
    }

    pub fn integer(value: i64) -> Self {
        crate::canon::make_integer(Integer::from_i64(value))
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

    // #region 🔖PublicConstructors
    pub fn add(terms: Vec<Expr>) -> Self {
        crate::canon::make_add(terms)
    }

    pub fn mul(factors: Vec<Expr>) -> Self {
        crate::canon::make_mul(factors)
    }

    pub fn pow(base: Expr, exp: Expr) -> Self {
        crate::canon::make_pow(base, exp)
    }

    pub fn func(kind: FnKind, args: Vec<Expr>) -> Self {
        crate::canon::make_func(kind, args)
    }
    // #endregion 🔖PublicConstructors
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
        write!(f, "{}", crate::fmt::display_string(self))
    }
}

/// 🔢 Kind-class rank used as the primary key of the canonical total order (before falling back to
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
    /// 🔢 Purely structural — never pointer- or hash-order — so `Display`/canonicalization output is
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

// #region 🔖Operators
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
        Expr::add(vec![self, crate::canon::make_neg(rhs)])
    }
}
impl std::ops::Sub<&Expr> for &Expr {
    type Output = Expr;
    fn sub(self, rhs: &Expr) -> Expr {
        Expr::add(vec![self.clone(), crate::canon::make_neg(rhs.clone())])
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
        crate::canon::make_neg(self)
    }
}
impl std::ops::Neg for &Expr {
    type Output = Expr;
    fn neg(self) -> Expr {
        crate::canon::make_neg(self.clone())
    }
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Expr::integer(value)
    }
}
impl From<Integer> for Expr {
    fn from(value: Integer) -> Self {
        crate::canon::make_integer(value)
    }
}
impl From<Rational> for Expr {
    fn from(value: Rational) -> Self {
        crate::canon::make_rational(value)
    }
}
// #endregion 🔖Operators
// #endregion 🔖Expr

// #region 🔖Tests
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
// #endregion 🔖Tests
