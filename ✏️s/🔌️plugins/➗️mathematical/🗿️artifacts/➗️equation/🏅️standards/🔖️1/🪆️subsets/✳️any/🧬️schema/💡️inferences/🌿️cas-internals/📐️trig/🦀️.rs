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
include!("../🧪️tests/🔬️trig-unit/🦀️.rs");
// #endregion 🔖️Tests
