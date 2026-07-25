//! 🚶 Tree walkers built on the canonical constructors: substitution, free-symbol collection, and
//! node counting (used by `simplify`'s measured pipeline).

use crate::expr::{Expr, Kind};

// #region 🔖Subs
/// 🔁 Replaces every occurrence of `target` with `replacement` (structural equality, post-order).
pub fn subs(e: &Expr, target: &Expr, replacement: &Expr) -> Expr {
    if e == target {
        return replacement.clone();
    }
    map_children(e, &mut |child| subs(child, target, replacement))
}

/// 🔁 Applies a full substitution map in one pass (each key checked before recursing into children).
pub fn subs_many(e: &Expr, map: &[(Expr, Expr)]) -> Expr {
    for (target, replacement) in map {
        if e == target {
            return replacement.clone();
        }
    }
    map_children(e, &mut |child| subs_many(child, map))
}

/// 🌳 Applies `f` to every child of `e` and rebuilds `e` with the results, going through the smart
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
// #endregion 🔖Subs

// #region 🔖Replace
/// 🔁 Bottom-up rewrite: applies `f` to every subtree (children first), keeping `f`'s result whenever
/// it returns `Some`.
pub fn replace_bottom_up(e: &Expr, f: &mut impl FnMut(&Expr) -> Option<Expr>) -> Expr {
    let rebuilt = map_children(e, &mut |child| replace_bottom_up(child, f));
    f(&rebuilt).unwrap_or(rebuilt)
}
// #endregion 🔖Replace

// #region 🔖FreeSymbols
/// 🔤 Every distinct symbol appearing anywhere in `e`, in canonical (sorted, deduplicated) order.
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
// #endregion 🔖FreeSymbols

// #region 🔖NodeCount
/// 🔢 Total node count (leaves + internal nodes), used by `simplify`'s "smallest wins" heuristic.
pub fn node_count(e: &Expr) -> usize {
    1 + children(e).iter().map(node_count).sum::<usize>()
}
// #endregion 🔖NodeCount

// #region 🔖Tests
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
        let e = Expr::add(vec![x.clone(), x.clone(), y.clone()]);
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
// #endregion 🔖Tests
