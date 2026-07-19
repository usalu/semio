//! 🖨️ Human-readable output: precedence-aware infix `Display` and a LaTeX emitter, both walking the
//! same canonical tree (so output is deterministic and stable across runs).

use crate::expr::{Constant, Expr, Kind, RelOp};
use crate::fnkind::FnKind;
use std::ops::Neg;

// #region 🔖Precedence
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
// #endregion 🔖Precedence

// #region 🔖Display
pub fn display_string(e: &Expr) -> String {
    let mut s = String::new();
    write_expr(e, &mut s);
    s
}

/// ✖️➗ Recovers the canonical `-a` / `a/b` encodings (`Mul([-1, a])`, `Mul([a, Pow(b,-1)])`) into
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
        Kind::Rel(op, a, b) => {
            write_expr(a, out);
            out.push_str(rel_symbol(*op));
            write_expr(b, out);
        }
        Kind::Wild(id, _) => out.push_str(&format!("_w{id}")),
    }
}

fn rel_symbol(op: RelOp) -> &'static str {
    match op {
        RelOp::Eq => " == ",
        RelOp::Ne => " != ",
        RelOp::Lt => " < ",
        RelOp::Le => " <= ",
        RelOp::Gt => " > ",
        RelOp::Ge => " >= ",
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

/// ➕ Prints in the conventional "highest-degree/most-complex term first, plain constant last" order —
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

/// ➖ Detects the `-1 * rest` / negative-literal encoding of a negated term and returns
/// `(true, positive_rest)`, or `(false, term)` if the term isn't negative.
fn extract_negation(term: &Expr) -> (bool, Expr) {
    match term.kind() {
        Kind::Integer(n) if n.is_negative() => return (true, Expr::from(n.abs_integer())),
        Kind::Rational(r) if r.numer().is_negative() => return (true, Expr::from(r.abs())),
        Kind::Mul(factors) => {
            if let Some(Kind::Integer(n)) = factors.first().map(|f| f.kind()) {
                if n.is_negative() {
                    let mut rest = factors[1..].to_vec();
                    if *n != mathematical_number::Integer::from_i64(-1) {
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
            if r.numer().abs_integer() != mathematical_number::Integer::one() {
                numer.push(Expr::from(r.numer().abs_integer()));
            }
            if r.numer().is_negative() {
                numer.insert(0, Expr::integer(-1));
            }
            denom.push(Expr::from(mathematical_number::Integer::from_natural(r.denom().clone())));
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
// #endregion 🔖Display

// #region 🔖Latex
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
        Kind::Rel(op, a, b) => {
            write_latex(a, out);
            out.push_str(latex_rel(*op));
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

fn latex_rel(op: RelOp) -> &'static str {
    match op {
        RelOp::Eq => " = ",
        RelOp::Ne => " \\neq ",
        RelOp::Lt => " < ",
        RelOp::Le => " \\leq ",
        RelOp::Gt => " > ",
        RelOp::Ge => " \\geq ",
    }
}
// #endregion 🔖Latex

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_simple_polynomial() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::integer(1)]);
        assert_eq!(display_string(&e), "x^2 + 1");
    }

    #[test]
    fn display_negative_term() {
        let x = Expr::symbol("x");
        let e = x.clone() - Expr::integer(1);
        assert_eq!(display_string(&e), "x - 1");
    }

    #[test]
    fn display_division() {
        let x = Expr::symbol("x");
        let e = x.clone() / Expr::integer(2);
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
// #endregion 🔖Tests
