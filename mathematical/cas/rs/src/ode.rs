//! 🌊 ODE solving. The kernel's `Expr` models closed-form values, not an unknown function with
//! derivatives (there's no "y''" node kind), so this module has two distinct front doors:
//! `solve_ode_first_order` takes `y' = f(x, y)` as an ordinary `Expr` in the two symbols `x, y` and
//! classifies it (separable / linear / Bernoulli); `solve_linear_constant_coeff_homogeneous` takes the
//! characteristic coefficients directly, since there's no `Expr` syntax for "the equation
//! `y''' - 2y'' + y = 0`" to parse in the first place.

use crate::expr::{Expr, Kind, RelationalOperator};
use crate::fnkind::FnKind;
use mathematical_number::{Integer, Rational};
use mathematical_polynomial::PolyU;

// #region 🔖OdeSolution
#[derive(Clone, Debug, PartialEq)]
pub struct OdeSolution {
    /// For first-order results this is the solved (or implicit) relation for `y`; for the
    /// constant-coefficient homogeneous case it's the general solution `y(x)` itself.
    pub rhs: Expr,
    pub constants: Vec<Expr>,
}
// #endregion 🔖OdeSolution

// #region 🔖FirstOrder
/// 🌊 Classifies and solves `y' = f(x, y)`: separable, linear, then Bernoulli, in that order.
pub fn solve_ode_first_order(f: &Expr, x: &Expr, y: &Expr) -> Option<OdeSolution> {
    try_separable(f, x, y).or_else(|| try_linear_first_order(f, x, y)).or_else(|| try_bernoulli(f, x, y))
}

fn try_separable(f: &Expr, x: &Expr, y: &Expr) -> Option<OdeSolution> {
    let factors: Vec<Expr> = match f.kind() {
        Kind::Mul(fs) => fs.clone(),
        _ => vec![f.clone()],
    };
    let (free_of_y, rest): (Vec<Expr>, Vec<Expr>) = factors.into_iter().partition(|fac| !crate::visit::contains_symbol(fac, y));
    if rest.iter().any(|fac| crate::visit::contains_symbol(fac, x)) {
        return None;
    }
    let g = Expr::mul(free_of_y);
    let h = if rest.is_empty() { Expr::integer(1) } else { Expr::mul(rest) };
    let lhs = crate::integrate::integrate(&Expr::pow(h, Expr::integer(-1)), y)?;
    let rhs = crate::integrate::integrate(&g, x)?;
    let c1 = Expr::symbol("§C1");
    Some(OdeSolution { rhs: Expr::from_kind_unchecked(Kind::Rel(RelationalOperator::Eq, lhs, rhs + c1.clone())), constants: vec![c1] })
}

/// 🧩 Extracts `(coeff, constant)` such that `f == coeff * y + constant`, both free of `y`; `None` if
/// `f` isn't affine in `y`.
fn affine_in_y(f: &Expr, y: &Expr) -> Option<(Expr, Expr)> {
    let expanded = crate::simplify::expand(f);
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
    if crate::visit::contains_symbol(&coeff, y) || crate::visit::contains_symbol(&q, y) {
        return None;
    }
    let p = Expr::integer(-1) * coeff;
    let integral_p = crate::integrate::integrate(&p, x)?;
    let mu = Expr::func(FnKind::Exp, vec![integral_p]);
    let integrand = crate::simplify::cancel(&(mu.clone() * q));
    let integral_mu_q = crate::integrate::integrate(&integrand, x)?;
    let c1 = Expr::symbol("§C1");
    let y_sol = crate::simplify::cancel(&((integral_mu_q + c1.clone()) * Expr::pow(mu, Expr::integer(-1))));
    Some(OdeSolution { rhs: y_sol, constants: vec![c1] })
}

fn try_bernoulli(f: &Expr, x: &Expr, y: &Expr) -> Option<OdeSolution> {
    let expanded = crate::simplify::expand(f);
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
        if crate::visit::contains_symbol(&coeff, y) {
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
// #endregion 🔖FirstOrder

// #region 🔖LinearConstantCoefficient
/// 🌊 General solution of `a_n y^(n) + ... + a_1 y' + a_0 y = 0`, given `coeffs = [a_0, ..., a_n]`
/// directly (see the module doc for why there's no `Expr`-equation front door for this case). Handles
/// real roots (with multiplicity, giving `x^k e^{rx}` terms) and complex-conjugate pairs from
/// irreducible quadratic factors (giving `x^k e^{alpha x} {cos,sin}(beta x)` terms); an irreducible
/// factor of degree >= 3 in the characteristic polynomial is a documented gap (`None`).
pub fn solve_linear_constant_coeff_homogeneous(coeffs: &[Rational], x: &Expr) -> Option<OdeSolution> {
    let char_poly = PolyU::from_coeffs(coeffs.to_vec());
    if char_poly.is_zero() || char_poly.degree().unwrap_or(0) == 0 {
        return None;
    }
    let (_, factors) = crate::polybridge::factor_poly_u(&char_poly);
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
                let beta = crate::solve::sqrt_of_rational(&beta_sq);
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
// #endregion 🔖LinearConstantCoefficient

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn satisfies(sol: &Expr, x: &Expr, y: &Expr, rhs_of_ode: &Expr) -> bool {
        // Substitutes the solution in for y and checks y' == rhs_of_ode(x, sol) structurally after simplify.
        let dy = crate::diff::diff(sol, x).unwrap();
        let substituted_rhs = crate::visit::subs(rhs_of_ode, y, sol);
        crate::simplify::simplify(&(dy - substituted_rhs)).is_zero_literal()
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
        let d1 = crate::diff::diff(&y_ex, &x).unwrap();
        let d2 = crate::diff::diff(&d1, &x).unwrap();
        let residual = d2 - Expr::integer(2) * d1 + y_ex;
        assert_eq!(crate::simplify::simplify(&residual), Expr::integer(0));
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
// #endregion 🔖Tests
