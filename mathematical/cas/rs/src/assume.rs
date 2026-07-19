//! 🧭 Per-symbol assumption flags (real/positive/integer/...) with closure-under-implication at
//! creation time, plus three-valued deduction queries over expressions once `Expr` exists (see the
//! `crate::expr` module — the deduction functions here take `&Expr` and live alongside the flag type
//! so the whole assumption story is in one file).

use crate::expr::{Expr, Kind};

// #region 🔖AssumeSet
/// 🧭 Bitflags of symbol properties; closed under implication by [`AssumeSet::close`] before a symbol
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

    /// 🔒 Applies every implication to a fixpoint (`POSITIVE => REAL, NONNEGATIVE, NONZERO`, etc.),
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
// #endregion 🔖AssumeSet

// #region 🔖Assumptions
/// 📋 Extra facts beyond a symbol's own flags (e.g. `x > 2`), consulted by `solve_with`/`refine`-style
/// entry points. First-pass scope: direct `symbol <op> rational` bounds only.
#[derive(Clone, Debug, Default)]
pub struct Assumptions {
    facts: Vec<(String, RelOp, mathematical_number::Rational)>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RelOp {
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

    pub fn assume_bound(&mut self, symbol: &str, op: RelOp, bound: mathematical_number::Rational) {
        self.facts.push((symbol.to_string(), op, bound));
    }

    fn bound_for(&self, symbol: &str) -> Option<bool> {
        for (name, op, bound) in &self.facts {
            if name != symbol {
                continue;
            }
            use mathematical_number::Rational;
            let zero = Rational::zero();
            let is_positive = match op {
                RelOp::Gt if *bound >= zero => Some(true),
                RelOp::Ge if *bound > zero => Some(true),
                RelOp::Lt if *bound <= zero => Some(false),
                RelOp::Le if *bound < zero => Some(false),
                _ => None,
            };
            if is_positive.is_some() {
                return is_positive;
            }
        }
        None
    }
}
// #endregion 🔖Assumptions

// #region 🔖Deduction
const MAX_DEDUCTION_DEPTH: u32 = 64;

/// 〽️ Three-valued positivity query: exact for numeric literals, from-flags for bare symbols, and
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
            crate::expr::Constant::Pi | crate::expr::Constant::E | crate::expr::Constant::EulerGamma => Some(true),
            crate::expr::Constant::Inf => Some(true),
            crate::expr::Constant::NegInf => Some(false),
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
            crate::fnkind::FnKind::Exp => Some(true),
            crate::fnkind::FnKind::Abs => is_nonzero_depth(&args[0], assumptions, depth + 1),
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
        Kind::Constant(c) => Some(!matches!(c, crate::expr::Constant::I | crate::expr::Constant::ComplexInf | crate::expr::Constant::Undefined)),
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
// #endregion 🔖Deduction

// #region 🔖Tests
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
        use mathematical_number::Rational;
        let mut assumptions = Assumptions::new();
        assumptions.assume_bound("x", RelOp::Gt, Rational::from_i64(2, 1).unwrap());
        assert_eq!(assumptions.bound_for("x"), Some(true));
        assert_eq!(assumptions.bound_for("y"), None);
    }
}
// #endregion 🔖Tests
