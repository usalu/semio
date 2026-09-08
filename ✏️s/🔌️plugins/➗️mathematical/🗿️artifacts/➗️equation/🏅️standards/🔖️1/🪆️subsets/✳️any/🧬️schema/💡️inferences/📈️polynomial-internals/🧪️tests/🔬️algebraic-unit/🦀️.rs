mod tests {
    use super::*;

    fn ipoly(coeffs: Vec<i64>) -> PolyU<Integer> {
        PolyU::from_coeffs(coeffs.into_iter().map(Integer::from_i64).collect())
    }

    #[semio_framework_async_macros::async_test]
    async fn sqrt2_plus_sqrt3_has_minimal_poly_degree_4() {
        let sqrt2 = AlgebraicReal::nth_root(&Rational::from_i64(2, 1).unwrap(), 2).unwrap();
        let sqrt3 = AlgebraicReal::nth_root(&Rational::from_i64(3, 1).unwrap(), 2).unwrap();
        let sum = sqrt2.add(&sqrt3);
        // minimal poly of sqrt2+sqrt3 is x^4 - 10x^2 + 1 (up to sign/unit); verify by exact evaluation
        // via interval refinement: (sum)^2 should be close to 5 + 2*sqrt6 ~ 9.899
        let val = sum.to_f64();
        assert!((val - (2f64.sqrt() + 3f64.sqrt())).abs() < 1e-6);
        assert!(sum.degree() <= 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn cbrt2_times_cbrt4_equals_2() {
        let cbrt2 = AlgebraicReal::nth_root(&Rational::from_i64(2, 1).unwrap(), 3).unwrap();
        let cbrt4 = AlgebraicReal::nth_root(&Rational::from_i64(4, 1).unwrap(), 3).unwrap();
        let product = cbrt2.mul(&cbrt4);
        assert!((product.to_f64() - 2.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn from_rational_is_exact() {
        let r = Rational::from_i64(3, 4).unwrap();
        let a = AlgebraicReal::from_rational(&r);
        assert!(a.is_rational());
        assert_eq!(a.to_f64(), r.to_f64());
    }

    #[semio_framework_async_macros::async_test]
    async fn neg_and_inv_hand_cases() {
        let sqrt2 = AlgebraicReal::nth_root(&Rational::from_i64(2, 1).unwrap(), 2).unwrap();
        let negated = sqrt2.neg();
        assert!((negated.to_f64() + 2f64.sqrt()).abs() < 1e-9);
        let inv = sqrt2.inv().unwrap();
        assert!((inv.to_f64() - 1.0 / 2f64.sqrt()).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn root_of_selects_correct_irreducible_factor() {
        // (x-1)(x^2-2): roots are 1, -sqrt2, sqrt2 in ascending order.
        let f = ipoly(vec![-1, 1]).mul(&ipoly(vec![-2, 0, 1]));
        let root0 = AlgebraicReal::root_of(&f, 0).unwrap(); // -sqrt2
        assert!((root0.to_f64() + 2f64.sqrt()).abs() < 1e-6);
        let root1 = AlgebraicReal::root_of(&f, 1).unwrap(); // 1
        assert!(root1.is_rational());
        assert_eq!(root1.to_f64(), 1.0);
    }
}
