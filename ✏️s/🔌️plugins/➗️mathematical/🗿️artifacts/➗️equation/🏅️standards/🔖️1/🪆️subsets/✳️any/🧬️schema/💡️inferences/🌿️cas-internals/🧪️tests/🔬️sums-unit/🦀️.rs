mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sum_of_k_from_1_to_n_is_gauss_formula() {
        let n = Expr::symbol("n");
        let k = Expr::symbol("k");
        // sum_{k=1}^{n} k -- but sum_closed evaluates a polynomial in the SAME variable used for the
        // bound substitution, so pass `k` itself as both the summand's variable and the closed-form target.
        let result = sum_closed(&k, &k, &Expr::integer(1), &n).unwrap();
        let expected = crate::cas::simplify::expand(&(n.clone() * (n + Expr::integer(1)) * Expr::from(Rational::from_i64(1, 2).unwrap())));
        assert_eq!(crate::cas::simplify::expand(&result), expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn sum_of_k_squared_matches_known_hand_values() {
        let k = Expr::symbol("k");
        // sum_{k=1}^{3} k^2 = 1+4+9 = 14
        let e = Expr::pow(k.clone(), Expr::integer(2));
        let result = sum_closed(&e, &k, &Expr::integer(1), &Expr::integer(3)).unwrap();
        assert_eq!(result, Expr::integer(14));
    }

    #[semio_framework_async_macros::async_test]
    async fn sum_geometric_series_hand_case() {
        let k = Expr::symbol("k");
        // sum_{k=0}^{3} 2^k = 1+2+4+8 = 15
        let e = Expr::pow(Expr::integer(2), k.clone());
        let result = sum_closed(&e, &k, &Expr::integer(0), &Expr::integer(3)).unwrap();
        assert_eq!(crate::cas::simplify::simplify(&result), Expr::integer(15));
    }

    #[semio_framework_async_macros::async_test]
    async fn fourier_coefficients_of_a_polynomial_smoke_test() {
        let x = Expr::symbol("x");
        let l = Expr::constant(Constant::Pi);
        let f = x.clone();
        let result = fourier_coefficients(&f, &x, &l, 2);
        assert!(result.is_some());
    }
}
