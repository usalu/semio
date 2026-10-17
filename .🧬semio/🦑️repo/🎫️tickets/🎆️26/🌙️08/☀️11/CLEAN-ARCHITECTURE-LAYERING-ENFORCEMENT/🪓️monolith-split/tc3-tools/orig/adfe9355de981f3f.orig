mod tests {
    use super::*;
    use crate::{integer::Integer, modular::ModInt, rational::Rational};

    fn rat(n: i64, d: i64) -> Rational {
        Rational::from_i64(n, d).unwrap()
    }

    fn rat_mat(rows: Vec<Vec<i64>>) -> MatG<Rational> {
        MatG::from_rows(rows.into_iter().map(|r| r.into_iter().map(Rational::from).collect()).collect())
    }

    #[test]
    fn rref_and_rank_hand_case() {
        let m = rat_mat(vec![vec![1, 2, 1], vec![2, 4, 2], vec![1, 1, 1]]);
        assert_eq!(m.rank(), 2);
    }

    #[test]
    fn det_matches_cofactor_hand_case() {
        let m = rat_mat(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(m.det(), rat(-2, 1));
    }

    #[test]
    fn inverse_round_trips_to_identity() {
        let m = rat_mat(vec![vec![2, 1], vec![1, 1]]);
        let inv = m.inverse().expect("invertible");
        let product = m.matmul(&inv);
        assert_eq!(product, MatG::identity(2));
    }

    #[test]
    fn solve_matches_hand_solved_system() {
        let m = rat_mat(vec![vec![2, 1], vec![1, 3]]);
        let b = VecG::from_vec(vec![rat(5, 1), rat(10, 1)]);
        let x = m.solve(&b).expect("solvable");
        assert_eq!(*x.get(0), rat(1, 1));
        assert_eq!(*x.get(1), rat(3, 1));
    }

    #[test]
    fn nullspace_vectors_are_in_kernel() {
        let m = rat_mat(vec![vec![1, 2, 3], vec![2, 4, 6]]);
        let basis = m.nullspace();
        assert_eq!(basis.len(), 2);
        for v in &basis {
            let zero = m.mul_vec(v);
            for i in 0..zero.len() {
                assert_eq!(*zero.get(i), Rational::zero());
            }
        }
    }

    #[test]
    fn cayley_hamilton_holds_for_3x3() {
        let m = rat_mat(vec![vec![2, 0, 0], vec![0, 3, 4], vec![0, 4, 9]]);
        let poly = m.charpoly(); // low-to-high coefficients, char(A)(x) = poly[0] + poly[1] x + ... + poly[n] x^n
        // Evaluate poly(M) = sum poly[i] * M^i and check it is the zero matrix.
        let n = m.rows;
        let mut power = MatG::<Rational>::identity(n);
        let mut acc = MatG::<Rational>::zeros(n, n);
        for coeff in &poly {
            acc = acc.add(&power.scale(coeff));
            power = power.matmul(&m);
        }
        for v in &acc.data {
            assert_eq!(*v, Rational::zero(), "Cayley-Hamilton violated: {acc:?}");
        }
    }

    #[test]
    fn bareiss_det_matches_field_det() {
        let m = rat_mat(vec![vec![4, 3, 2], vec![1, 5, 6], vec![7, 8, 9]]);
        let via_field = m.det();
        let via_bareiss = m.det_bareiss();
        assert_eq!(via_field, via_bareiss);
    }

    #[test]
    fn bareiss_det_over_integer_matches_hand_case() {
        let m = MatG::<Integer>::from_rows(vec![vec![Integer::from_i64(2), Integer::from_i64(3)], vec![Integer::from_i64(1), Integer::from_i64(4)]]);
        assert_eq!(m.det_bareiss(), Integer::from_i64(5));
    }

    #[test]
    fn berkowitz_over_modint_matches_brute_force_2x2() {
        let p = 13u64;
        let a = ModInt::new(2, p);
        let b = ModInt::new(1, p);
        let c = ModInt::new(1, p);
        let d = ModInt::new(3, p);
        let m = MatG::from_rows(vec![vec![a, b], vec![c, d]]);
        let poly = m.charpoly();
        // trace = a+d, det = ad-bc; char poly(x) = det - trace*x + x^2 (low-to-high: [det, -trace, 1])
        let trace = a.add(&d);
        let det = a.mul(&d).sub(&b.mul(&c));
        assert_eq!(poly[0], det);
        assert_eq!(poly[1], trace.neg());
        assert_eq!(poly[2], ModInt::new(1, p));
    }

    #[test]
    fn smith_normal_form_divisibility_chain() {
        let m = MatG::<Integer>::from_rows(vec![vec![Integer::from_i64(2), Integer::from_i64(4)], vec![Integer::from_i64(4), Integer::from_i64(2)]]);
        let (s, u, v) = m.smith_normal_form();
        let reconstructed = u.matmul(&m).matmul(&v);
        assert_eq!(reconstructed, s);
        // off-diagonal entries are zero and each diagonal entry divides the next.
        assert!(s.get(0, 1).is_zero());
        assert!(s.get(1, 0).is_zero());
        assert!(s.get(1, 1).exact_div(s.get(0, 0)).is_some());
    }

    #[test]
    fn vecg_basic_ops_match_hand_computation() {
        let a = VecG::from_vec(vec![rat(1, 1), rat(2, 1)]);
        let b = VecG::from_vec(vec![rat(3, 1), rat(4, 1)]);
        assert_eq!(a.len(), 2);
        assert!(!a.is_empty());
        assert!(VecG::<Rational>::zeros(0).is_empty());
        let sum = a.add(&b);
        assert_eq!(*sum.get(0), rat(4, 1));
        assert_eq!(*sum.get(1), rat(6, 1));
        let diff = b.sub(&a);
        assert_eq!(*diff.get(0), rat(2, 1));
        assert_eq!(*diff.get(1), rat(2, 1));
        let scaled = a.scale(&rat(2, 1));
        assert_eq!(*scaled.get(0), rat(2, 1));
        assert_eq!(*scaled.get(1), rat(4, 1));
        assert_eq!(a.dot(&b), rat(11, 1));
        let mut c = VecG::zeros(2);
        c.set(0, rat(9, 1));
        assert_eq!(*c.get(0), rat(9, 1));
    }

    #[test]
    fn det_of_singular_matrix_is_zero() {
        let m = rat_mat(vec![vec![1, 2], vec![2, 4]]);
        assert_eq!(m.det(), Rational::zero());
    }

    #[test]
    fn inverse_of_singular_matrix_is_none() {
        let m = rat_mat(vec![vec![1, 2], vec![2, 4]]);
        assert!(m.inverse().is_none());
    }

    #[test]
    fn solve_of_singular_system_is_none() {
        let m = rat_mat(vec![vec![1, 2], vec![2, 4]]);
        let b = VecG::from_vec(vec![rat(1, 1), rat(2, 1)]);
        assert!(m.solve(&b).is_none());
    }

    #[test]
    fn rank_bareiss_hand_cases_full_and_deficient_rank() {
        let full = MatG::<Integer>::from_rows(vec![
            vec![Integer::from_i64(2), Integer::from_i64(1), Integer::from_i64(1)],
            vec![Integer::from_i64(1), Integer::from_i64(3), Integer::from_i64(2)],
            vec![Integer::from_i64(1), Integer::from_i64(0), Integer::from_i64(0)],
        ]);
        assert_eq!(full.rank_bareiss(), 3);
        let deficient = MatG::<Integer>::from_rows(vec![
            vec![Integer::from_i64(1), Integer::from_i64(2), Integer::from_i64(3)],
            vec![Integer::from_i64(2), Integer::from_i64(4), Integer::from_i64(6)],
            vec![Integer::from_i64(1), Integer::from_i64(0), Integer::from_i64(1)],
        ]);
        assert_eq!(deficient.rank_bareiss(), 2);
    }

    // #region 🔖️QuickExactTests
    mod quick {
        use super::*;

        #[test]
        fn random_rational_matrix_inverse_round_trips() {
            let mut seed = 0xABCD_EF01_2345_6789u64;
            let mut next = move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                (seed % 10) as i64 - 5
            };
            for _ in 0..20 {
                let n = 4;
                let mut rows = vec![vec![0i64; n]; n];
                for row in rows.iter_mut() {
                    for cell in row.iter_mut() {
                        *cell = next();
                    }
                }
                let m = rat_mat(rows);
                if let Some(inv) = m.inverse() {
                    let product = m.matmul(&inv);
                    assert_eq!(product, MatG::identity(n));
                    assert_eq!(m.rank(), n);
                }
            }
        }

        #[test]
        fn bareiss_matches_field_det_on_random_integer_matrices() {
            let mut seed = 0x1122_3344_5566_7788u64;
            let mut next = move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                (seed % 7) as i64 - 3
            };
            for _ in 0..20 {
                let n = 3;
                let mut rows_i = vec![vec![Integer::from_i64(0); n]; n];
                let mut rows_r = vec![vec![rat(0, 1); n]; n];
                for i in 0..n {
                    for j in 0..n {
                        let v = next();
                        rows_i[i][j] = Integer::from_i64(v);
                        rows_r[i][j] = rat(v, 1);
                    }
                }
                let mi = MatG::from_rows(rows_i);
                let mr = MatG::from_rows(rows_r);
                let via_bareiss = mi.det_bareiss();
                let via_field = mr.det();
                assert_eq!(Rational::from_integer(via_bareiss), via_field);
            }
        }

        #[test]
        fn rank_bareiss_matches_field_rank_on_random_integer_matrices() {
            let mut seed = 0x9988_7766_5544_3322u64;
            let mut next = move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                (seed % 5) as i64 - 2
            };
            for _ in 0..20 {
                let n = 3;
                let mut rows_i = vec![vec![Integer::from_i64(0); n]; n];
                let mut rows_r = vec![vec![rat(0, 1); n]; n];
                for i in 0..n {
                    for j in 0..n {
                        let v = next();
                        rows_i[i][j] = Integer::from_i64(v);
                        rows_r[i][j] = rat(v, 1);
                    }
                }
                let mi = MatG::from_rows(rows_i);
                let mr = MatG::from_rows(rows_r);
                assert_eq!(mi.rank_bareiss(), mr.rank());
            }
        }
    }
    // #endregion 🔖️QuickExactTests
}
