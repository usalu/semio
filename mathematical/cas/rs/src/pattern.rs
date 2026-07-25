//! 🃏 Structural pattern matching and rule-based rewriting over the canonical `Expr` tree. `Add`/`Mul`
//! matching is associative-commutative (subject terms may match pattern terms in any order), handled by
//! bounded backtracking rather than full (NP-hard) AC unification — a budget caps the search so a
//! pathologically wide subject conservatively fails to match instead of hanging.

use crate::expr::{Expr, Kind, WildKind};
use std::collections::BTreeMap;
use std::rc::Rc;

// #region 🔖Wildcards
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
// #endregion 🔖Wildcards

// #region 🔖Bindings
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
// #endregion 🔖Bindings

// #region 🔖Matcher
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

/// 🧩 Matches an unordered term list against another: assigns each non-`Seq` pattern term to a distinct
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
// #endregion 🔖Matcher

// #region 🔖Instantiate
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
// #endregion 🔖Instantiate

// #region 🔖Rules
pub enum RuleRhs {
    Template(Expr),
    Builder(Rc<dyn Fn(&Bindings) -> Expr>),
}

pub struct Rule {
    lhs: Expr,
    rhs: RuleRhs,
    cond: Option<Rc<dyn Fn(&Bindings) -> bool>>,
}

impl Rule {
    pub fn new(lhs: Expr, rhs: Expr) -> Self {
        Self { lhs, rhs: RuleRhs::Template(rhs), cond: None }
    }

    pub fn with_condition(lhs: Expr, rhs: Expr, cond: Rc<dyn Fn(&Bindings) -> bool>) -> Self {
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
            Strategy::BottomUpOnce => crate::visit::replace_bottom_up(e, &mut |sub| self.try_apply_one(sub)),
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
        crate::visit::map_children(&rewritten, &mut |c| self.apply_top_down_once(c))
    }
}
// #endregion 🔖Rules

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::Expr;

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
        let matched: std::collections::BTreeSet<Expr> = bindings.values().map(|b| match b { Binding::One(e) => e.clone(), _ => panic!() }).collect();
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
        let lhs = Expr::add(vec![Expr::pow(Expr::func(crate::fnkind::FnKind::Sin, vec![wild(0)]), Expr::integer(2)), Expr::pow(Expr::func(crate::fnkind::FnKind::Cos, vec![wild(0)]), Expr::integer(2))]);
        let rule = Rule::new(lhs, Expr::integer(1));
        let subject = Expr::add(vec![Expr::pow(Expr::func(crate::fnkind::FnKind::Sin, vec![w.clone()]), Expr::integer(2)), Expr::pow(Expr::func(crate::fnkind::FnKind::Cos, vec![w]), Expr::integer(2))]);
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
// #endregion 🔖Tests
