mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn add_folds_numeric_literals() {
        let e = make_add(vec![Expr::integer(2), Expr::integer(3)]);
        assert_eq!(e, Expr::integer(5));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_collects_like_terms() {
        let x = Expr::symbol("x");
        let e = make_add(vec![x.clone(), x.clone()]);
        let expected = make_mul(vec![Expr::integer(2), x]);
        assert_eq!(e, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_drops_zero() {
        let x = Expr::symbol("x");
        let e = make_add(vec![x.clone(), Expr::integer(0)]);
        assert_eq!(e, x);
    }

    #[semio_framework_async_macros::async_test]
    async fn mul_folds_numeric_literals() {
        let e = make_mul(vec![Expr::integer(2), Expr::integer(3)]);
        assert_eq!(e, Expr::integer(6));
    }

    #[semio_framework_async_macros::async_test]
    async fn mul_combines_like_bases() {
        let x = Expr::symbol("x");
        let e = make_mul(vec![x.clone(), x.clone()]);
        let expected = make_pow(x, Expr::integer(2));
        assert_eq!(e, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn mul_by_zero_absorbs() {
        let x = Expr::symbol("x");
        let e = make_mul(vec![x, Expr::integer(0)]);
        assert_eq!(e, Expr::integer(0));
    }

    #[semio_framework_async_macros::async_test]
    async fn mul_zero_times_complex_infinity_is_undefined() {
        let e = make_mul(vec![Expr::integer(0), Expr::constant(Constant::ComplexInf)]);
        assert_eq!(e, Expr::constant(Constant::Undefined));
    }

    #[semio_framework_async_macros::async_test]
    async fn mul_nonzero_times_complex_infinity_is_complex_infinity() {
        let e = make_mul(vec![Expr::integer(5), Expr::constant(Constant::ComplexInf)]);
        assert_eq!(e, Expr::constant(Constant::ComplexInf));
    }

    #[semio_framework_async_macros::async_test]
    async fn pow_identities() {
        let x = Expr::symbol("x");
        assert_eq!(make_pow(x.clone(), Expr::integer(0)), Expr::integer(1));
        assert_eq!(make_pow(x.clone(), Expr::integer(1)), x);
        assert_eq!(make_pow(Expr::integer(1), x), Expr::integer(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn pow_integer_folds_exactly() {
        assert_eq!(make_pow(Expr::integer(2), Expr::integer(10)), Expr::integer(1024));
    }

    #[semio_framework_async_macros::async_test]
    async fn pow_negative_integer_exponent_gives_rational() {
        let e = make_pow(Expr::integer(2), Expr::integer(-1));
        assert_eq!(e, make_rational(Rational::from_i64(1, 2).unwrap()));
    }

    #[semio_framework_async_macros::async_test]
    async fn radical_partial_extraction_matches_plan_example() {
        // 8^(1/2) -> 2 * 2^(1/2)
        let e = make_pow(Expr::integer(8), make_rational(Rational::from_i64(1, 2).unwrap()));
        let expected = make_mul(vec![Expr::integer(2), make_pow(Expr::integer(2), make_rational(Rational::from_i64(1, 2).unwrap()))]);
        assert_eq!(e, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn radical_exact_perfect_power_folds_fully() {
        // 4^(1/2) -> 2
        let e = make_pow(Expr::integer(4), make_rational(Rational::from_i64(1, 2).unwrap()));
        assert_eq!(e, Expr::integer(2));
    }

    #[semio_framework_async_macros::async_test]
    async fn radical_of_prime_stays_symbolic() {
        let e = make_pow(Expr::integer(2), make_rational(Rational::from_i64(1, 2).unwrap()));
        assert!(matches!(e.kind(), Kind::Pow(..)));
    }

    #[semio_framework_async_macros::async_test]
    async fn nested_pow_combines_exponents_for_integer_outer_exponent() {
        let x = Expr::symbol("x");
        let inner = make_pow(x.clone(), Expr::integer(2));
        let outer = make_pow(inner, Expr::integer(3));
        assert_eq!(outer, make_pow(x, Expr::integer(6)));
    }

    #[semio_framework_async_macros::async_test]
    async fn i_power_cycles_with_period_four() {
        let i = Expr::constant(Constant::I);
        assert_eq!(make_pow(i.clone(), Expr::integer(0)), Expr::integer(1));
        assert_eq!(make_pow(i.clone(), Expr::integer(1)), i);
        assert_eq!(make_pow(i.clone(), Expr::integer(2)), Expr::integer(-1));
        assert_eq!(make_pow(i, Expr::integer(4)), Expr::integer(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn func_special_values_fold() {
        assert_eq!(make_func(FnKind::Sin, vec![Expr::integer(0)]), Expr::integer(0));
        assert_eq!(make_func(FnKind::Cos, vec![Expr::integer(0)]), Expr::integer(1));
        assert_eq!(make_func(FnKind::Exp, vec![Expr::integer(0)]), Expr::integer(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn canonicalization_is_idempotent_on_a_small_corpus() {
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
