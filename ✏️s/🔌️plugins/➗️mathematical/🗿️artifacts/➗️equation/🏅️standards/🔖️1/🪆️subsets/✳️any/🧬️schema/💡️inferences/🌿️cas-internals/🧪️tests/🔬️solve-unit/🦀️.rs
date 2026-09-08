mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn solve_linear_equation() {
        let x = Expr::symbol("x");
        // 2x - 6 = 0 -> x = 3
        let e = Expr::mul(vec![Expr::integer(2), x.clone()]) - Expr::integer(6);
        assert_eq!(solve_univariate(&e, &x), SolutionSet::Finite(vec![Expr::integer(3)]));
    }

    #[semio_framework_async_macros::async_test]
    async fn solve_quadratic_with_real_roots() {
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

    #[semio_framework_async_macros::async_test]
    async fn solve_quadratic_with_complex_roots() {
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

    #[semio_framework_async_macros::async_test]
    async fn solve_high_degree_gives_rootof() {
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

    #[semio_framework_async_macros::async_test]
    async fn solve_exp_equation() {
        let x = Expr::symbol("x");
        // 2*exp(x) - 6 = 0 -> x = ln(3)
        let e = Expr::mul(vec![Expr::integer(2), Expr::func(FnKind::Exp, vec![x.clone()])]) - Expr::integer(6);
        let result = solve_univariate(&e, &x);
        assert_eq!(result, SolutionSet::Finite(vec![Expr::func(FnKind::Ln, vec![Expr::integer(3)])]));
    }

    #[semio_framework_async_macros::async_test]
    async fn solve_sin_equation_gives_parametric_family() {
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

    #[semio_framework_async_macros::async_test]
    async fn solve_2x2_linear_system() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        // 2x + y = 5, x - y = 1 -> x=2, y=1
        let eq1 = Expr::mul(vec![Expr::integer(2), x.clone()]) + y.clone() - Expr::integer(5);
        let eq2 = x.clone() - y.clone() - Expr::integer(1);
        let result = solve_linear_system(&[eq1, eq2], &[x, y]);
        assert_eq!(result, SolutionSet::Finite(vec![Expr::integer(2), Expr::integer(1)]));
    }

    #[semio_framework_async_macros::async_test]
    async fn solve_inequality_simple_quadratic() {
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
