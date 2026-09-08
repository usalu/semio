mod tests {
    use super::*;

    fn i(v: i64) -> Integer {
        Integer::from_i64(v)
    }

    fn ipoly(coeffs: Vec<i64>) -> PolyU<Integer> {
        PolyU::from_coeffs(coeffs.into_iter().map(Integer::from_i64).collect())
    }

    #[semio_framework_async_macros::async_test]
    async fn factor_x2_minus_1() {
        let f = ipoly(vec![-1, 0, 1]); // x^2 - 1 = (x-1)(x+1)
        let (content, factors) = factor_integer_poly(&f);
        assert_eq!(content, i(1));
        let mut product = PolyU::constant(content);
        for (factor, mult) in &factors {
            product = product.mul(&factor.pow(*mult as u64));
        }
        assert_eq!(product, f);
        assert_eq!(factors.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn factor_x4_minus_1() {
        let f = ipoly(vec![-1, 0, 0, 0, 1]); // x^4 - 1 = (x-1)(x+1)(x^2+1)
        let (_, factors) = factor_integer_poly(&f);
        let mut product = PolyU::<Integer>::one();
        for (factor, mult) in &factors {
            product = product.mul(&factor.pow(*mult as u64));
        }
        assert_eq!(product, f);
        assert!(factors.len() >= 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn factor_repeated_linear_factor() {
        let base = ipoly(vec![-1, 1]); // x - 1
        let f = base.mul(&base).mul(&base); // (x-1)^3
        let (_, factors) = factor_integer_poly(&f);
        let mut product = PolyU::<Integer>::one();
        for (factor, mult) in &factors {
            product = product.mul(&factor.pow(*mult as u64));
        }
        assert_eq!(product, f);
        assert!(factors.iter().any(|(factor, mult)| *factor == base && *mult == 3));
    }

    #[semio_framework_async_macros::async_test]
    async fn factor_irreducible_quadratic_stays_whole() {
        let f = ipoly(vec![1, 0, 1]); // x^2 + 1, irreducible over Q
        let (_, factors) = factor_integer_poly(&f);
        assert_eq!(factors.len(), 1);
        assert_eq!(factors[0].1, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn factor_nonmonic_quadratic() {
        let f = ipoly(vec![-3, -1, 2]); // 2x^2 - x - 3 = (2x - 3)(x + 1)
        let (content, factors) = factor_integer_poly(&f);
        let mut product = PolyU::constant(content);
        for (factor, mult) in &factors {
            product = product.mul(&factor.pow(*mult as u64));
        }
        assert_eq!(product, f);
    }

    #[semio_framework_async_macros::async_test]
    async fn rational_roots_of_quadratic() {
        let f = PolyU::from_coeffs(vec![Rational::from_i64(1, 1).unwrap(), Rational::from_i64(-5, 1).unwrap(), Rational::from_i64(6, 1).unwrap()]); // 6x^2 - 5x + 1
        let roots = rational_roots(&f);
        assert_eq!(roots.len(), 2);
        for r in &roots {
            assert!(f.eval(r).is_zero());
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn rational_roots_with_zero_root() {
        let f = PolyU::from_coeffs(vec![Rational::zero(), Rational::from_i64(-1, 1).unwrap(), Rational::from_i64(1, 1).unwrap()]); // x^2 - x = x(x-1)
        let roots = rational_roots(&f);
        assert_eq!(roots.len(), 2);
    }
}
