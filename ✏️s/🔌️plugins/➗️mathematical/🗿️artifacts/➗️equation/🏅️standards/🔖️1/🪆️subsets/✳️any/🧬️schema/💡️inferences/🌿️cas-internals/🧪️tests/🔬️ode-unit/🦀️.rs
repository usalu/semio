mod tests {
    use super::*;

    fn satisfies(sol: &Expr, x: &Expr, y: &Expr, rhs_of_ode: &Expr) -> bool {
        let dy = crate::cas::diff::diff(sol, x).unwrap();
        let substituted_rhs = crate::cas::visit::subs(rhs_of_ode, y, sol);
        crate::cas::simplify::simplify(&(dy - substituted_rhs)).is_zero_literal()
    }

    #[semio_framework_async_macros::async_test]
    async fn separable_ode_y_prime_equals_x_over_y() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let f = x.clone() * Expr::pow(y.clone(), Expr::integer(-1));
        let sol = solve_ode_first_order(&f, &x, &y).unwrap();
        assert!(matches!(sol.rhs.kind(), Kind::Rel(RelationalOperator::Eq, ..)));
    }

    #[semio_framework_async_macros::async_test]
    async fn linear_first_order_ode() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let f = y.clone() + x.clone();
        let sol = solve_ode_first_order(&f, &x, &y).unwrap();
        assert!(satisfies(&sol.rhs, &x, &y, &f));
    }

    #[semio_framework_async_macros::async_test]
    async fn bernoulli_ode() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let f = y.clone() * Expr::pow(x.clone(), Expr::integer(-1)) - Expr::pow(y.clone(), Expr::integer(2));
        let sol = solve_ode_first_order(&f, &x, &y);
        assert!(sol.is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn linear_constant_coefficient_second_order_distinct_real_roots() {
        let x = Expr::symbol("x");
        let coeffs = vec![Rational::from_i64(2, 1).unwrap(), Rational::from_i64(-3, 1).unwrap(), Rational::one()];
        let sol = solve_linear_constant_coeff_homogeneous(&coeffs, &x).unwrap();
        assert_eq!(sol.constants.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn linear_constant_coefficient_repeated_root() {
        let x = Expr::symbol("x");
        let coeffs = vec![Rational::one(), Rational::from_i64(-2, 1).unwrap(), Rational::one()];
        let sol = solve_linear_constant_coeff_homogeneous(&coeffs, &x).unwrap();
        assert_eq!(sol.constants.len(), 2);
        let y_ex = Expr::func(FnKind::Exp, vec![x.clone()]);
        let d1 = crate::cas::diff::diff(&y_ex, &x).unwrap();
        let d2 = crate::cas::diff::diff(&d1, &x).unwrap();
        let residual = d2 - Expr::integer(2) * d1 + y_ex;
        assert_eq!(crate::cas::simplify::simplify(&residual), Expr::integer(0));
    }

    #[semio_framework_async_macros::async_test]
    async fn linear_constant_coefficient_complex_roots() {
        let x = Expr::symbol("x");
        let coeffs = vec![Rational::one(), Rational::zero(), Rational::one()];
        let sol = solve_linear_constant_coeff_homogeneous(&coeffs, &x).unwrap();
        assert_eq!(sol.constants.len(), 2);
    }
}
