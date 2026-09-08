mod tests {
    use super::*;

    fn ipoly(coeffs: Vec<i64>) -> PolyU<Integer> {
        PolyU::from_coeffs(coeffs.into_iter().map(Integer::from_i64).collect())
    }

    #[semio_framework_async_macros::async_test]
    async fn isolate_roots_of_simple_quadratic() {
        let f = ipoly(vec![-2, 0, 1]); // x^2 - 2, roots +-sqrt(2)
        let intervals = isolate_real_roots(&f);
        assert_eq!(intervals.len(), 2);
        for (lo, hi) in &intervals {
            let seq = sturm_sequence(&f);
            assert_eq!(count_roots_in(&seq, lo, hi), 1);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn isolate_roots_matches_known_integer_roots() {
        // (x-1)(x-3)(x+2)
        let f = ipoly(vec![6, -1, -4, 1]);
        let intervals = isolate_real_roots(&f);
        assert_eq!(intervals.len(), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn refine_root_converges_to_sqrt2() {
        let f = ipoly(vec![-2, 0, 1]);
        let intervals = isolate_real_roots(&f);
        let positive = intervals.iter().find(|(lo, hi)| lo.is_zero() || (!lo.numer().is_negative() && !hi.numer().is_negative())).cloned().unwrap();
        let width = Rational::from_i64(1, 1_000_000).unwrap();
        let (lo, hi) = refine_root(&f, &positive.0, &positive.1, &width);
        let approx = (lo.to_f64() + hi.to_f64()) / 2.0;
        assert!((approx - std::f64::consts::SQRT_2).abs() < 1e-5);
    }

    #[semio_framework_async_macros::async_test]
    async fn cauchy_bound_contains_all_roots() {
        let f = ipoly(vec![6, -1, -4, 1]); // roots -2, 1, 3
        let bound = cauchy_root_bound(&f);
        assert!(bound >= Rational::from_i64(3, 1).unwrap());
    }

    #[semio_framework_async_macros::async_test]
    async fn wilkinson_like_small_case_root_count() {
        // (x-1)(x-2)(x-3)(x-4)
        let f = ipoly(vec![-1, 1]).mul(&ipoly(vec![-2, 1])).mul(&ipoly(vec![-3, 1])).mul(&ipoly(vec![-4, 1]));
        let intervals = isolate_real_roots(&f);
        assert_eq!(intervals.len(), 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn zero_polynomial_and_constant_have_no_roots() {
        assert!(isolate_real_roots(&ipoly(vec![])).is_empty());
        assert!(isolate_real_roots(&ipoly(vec![5])).is_empty());
    }
}
