use super::*;

#[test]
fn mat2_identity_apply_is_noop() {
    assert_eq!(Mat2::IDENTITY.apply((3.0, -4.0)), (3.0, -4.0));
}

#[test]
fn mat2_apply_matches_matrix_vector_multiply() {
    let m = Mat2::new(2.0, 0.0, 0.0, 3.0);
    assert_eq!(m.apply((1.0, 1.0)), (2.0, 3.0));
}

#[test]
fn mat2_det_of_scale_matrix() {
    let m = Mat2::new(2.0, 0.0, 0.0, 3.0);
    assert!((m.det() - 6.0).abs() < 1e-9);
}

#[test]
fn mat2_inverse_round_trips() {
    let m = Mat2::new(2.0, 1.0, 1.0, 3.0);
    let inv = m.inverse().expect("invertible");
    let round = m.mul(inv);
    assert!((round.cols[0][0] - 1.0).abs() < 1e-9);
    assert!((round.cols[1][1] - 1.0).abs() < 1e-9);
    assert!(round.cols[1][0].abs() < 1e-9);
    assert!(round.cols[0][1].abs() < 1e-9);
}

#[test]
fn mat2_singular_matrix_has_no_inverse() {
    let m = Mat2::new(1.0, 2.0, 2.0, 4.0);
    assert!(m.inverse().is_none());
}

#[test]
fn mat2_transpose_swaps_off_diagonal() {
    let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
    let t = m.transpose();
    assert_eq!(t.apply((1.0, 0.0)), (1.0, 2.0));
    assert_eq!(t.apply((0.0, 1.0)), (3.0, 4.0));
}

#[test]
fn mat2_eigenvalues_of_diagonal_matrix_are_the_diagonal() {
    let m = Mat2::new(2.0, 0.0, 0.0, 5.0);
    let (l1, l2) = m.eigenvalues().expect("real eigenvalues");
    let mut vals = [l1, l2];
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!((vals[0] - 2.0).abs() < 1e-9);
    assert!((vals[1] - 5.0).abs() < 1e-9);
}

#[test]
fn mat2_rotation_like_matrix_has_complex_eigenvalues() {
    let m = Mat2::new(0.0, -1.0, 1.0, 0.0);
    assert!(m.eigenvalues().is_none());
}

#[test]
fn vecd_dot_and_norm() {
    let a = VecD::from_vec(vec![3.0, 4.0]);
    let b = VecD::from_vec(vec![1.0, 0.0]);
    assert!((a.dot(&b) - 3.0).abs() < 1e-12);
    assert!((a.norm2() - 5.0).abs() < 1e-12);
    assert!((a.norm_inf() - 4.0).abs() < 1e-12);
}

#[test]
fn vecd_add_sub_scale_round_trip() {
    let a = VecD::from_vec(vec![1.0, 2.0, 3.0]);
    let b = VecD::from_vec(vec![0.5, 0.5, 0.5]);
    let sum = a.add(&b);
    let back = sum.sub(&b);
    for i in 0..3 {
        assert!((back.get(i) - a.get(i)).abs() < 1e-12);
    }
    assert!((a.scale(2.0).get(1) - 4.0).abs() < 1e-12);
}

#[test]
fn matd_matmul_identity_is_noop() {
    let mut m = MatD::zeros(2, 2);
    m.set(0, 0, 1.0);
    m.set(0, 1, 2.0);
    m.set(1, 0, 3.0);
    m.set(1, 1, 4.0);
    let id = MatD::identity(2);
    let out = m.matmul(&id);
    assert_eq!(out, m);
}

#[test]
fn matd_transpose_round_trips() {
    let mut m = MatD::zeros(2, 3);
    for row in 0..2 {
        for col in 0..3 {
            m.set(row, col, (row * 3 + col) as f64);
        }
    }
    assert_eq!(m.transpose().transpose(), m);
}

#[test]
fn matd_mul_vec_matches_matrix_vector_multiply() {
    let mut m = MatD::zeros(2, 2);
    m.set(0, 0, 2.0);
    m.set(0, 1, 0.0);
    m.set(1, 0, 0.0);
    m.set(1, 1, 3.0);
    let x = VecD::from_vec(vec![1.0, 1.0]);
    let y = m.mul_vec(&x);
    assert!((y.get(0) - 2.0).abs() < 1e-12);
    assert!((y.get(1) - 3.0).abs() < 1e-12);
}

#[test]
fn matd_lu_solve_matches_hand_solved_system() {
    // 2x + y = 5, x + 3y = 10  =>  x = 1, y = 3
    let mut a = MatD::zeros(2, 2);
    a.set(0, 0, 2.0);
    a.set(0, 1, 1.0);
    a.set(1, 0, 1.0);
    a.set(1, 1, 3.0);
    let b = VecD::from_vec(vec![5.0, 10.0]);
    let x = a.lu_solve(&b).expect("solvable");
    assert!((x.get(0) - 1.0).abs() < 1e-9);
    assert!((x.get(1) - 3.0).abs() < 1e-9);
}

#[test]
fn matd_lu_solve_detects_singular_matrix() {
    let mut a = MatD::zeros(2, 2);
    a.set(0, 0, 1.0);
    a.set(0, 1, 2.0);
    a.set(1, 0, 2.0);
    a.set(1, 1, 4.0);
    let b = VecD::from_vec(vec![1.0, 2.0]);
    assert!(a.lu_solve(&b).is_none());
}

#[test]
fn matd_add_triple_product_matches_btdb() {
    let mut b = MatD::zeros(1, 2);
    b.set(0, 0, 1.0);
    b.set(0, 1, 2.0);
    let mut d = MatD::zeros(1, 1);
    d.set(0, 0, 3.0);
    let mut ke = MatD::zeros(2, 2);
    ke.add_triple_product(&b, &d, 1.0);
    // Bᵀ D B = [1;2] * 3 * [1 2] = [[3,6],[6,12]]
    assert!((ke.get(0, 0) - 3.0).abs() < 1e-12);
    assert!((ke.get(0, 1) - 6.0).abs() < 1e-12);
    assert!((ke.get(1, 1) - 12.0).abs() < 1e-12);
}

#[test]
fn mat3d_identity_axes_is_identity() {
    let m = Mat3d::from_axes([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
    assert_eq!(m, Mat3d::IDENTITY);
}

#[test]
fn mat3d_transpose_is_inverse_for_orthonormal_basis() {
    let x = vec3d_normalize([1.0, 1.0, 0.0]);
    let z = [0.0, 0.0, 1.0];
    let y = vec3d_cross(z, x);
    let m = Mat3d::from_axes(x, y, z);
    let round = m.mul(m.transpose());
    for row in 0..3 {
        for col in 0..3 {
            let expected = if row == col { 1.0 } else { 0.0 };
            assert!((round.cols[col][row] - expected).abs() < 1e-9);
        }
    }
}

#[test]
fn mat3d_mul_vec3_transforms_basis_vector() {
    let m = Mat3d::from_axes([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let out = m.mul_vec3([1.0, 0.0, 0.0]);
    assert!((out[0] - 0.0).abs() < 1e-12);
    assert!((out[1] - 1.0).abs() < 1e-12);
    assert!((out[2] - 0.0).abs() < 1e-12);
}

#[test]
fn vec3d_cross_is_perpendicular_to_inputs() {
    let a = [1.0, 0.0, 0.0];
    let b = [0.0, 1.0, 0.0];
    let c = vec3d_cross(a, b);
    assert!((c[2] - 1.0).abs() < 1e-12);
}

#[test]
fn csr_from_triplets_dedupes_and_sorts() {
    let triplets = [(0, 1, 2.0), (0, 1, 3.0), (0, 0, 1.0), (1, 0, 4.0)];
    let m = CsrMatrix::from_triplets(2, 2, &triplets);
    assert_eq!(m.nnz(), 3);
    let row0: Vec<(usize, f64)> = m.row(0).collect();
    assert_eq!(row0, vec![(0, 1.0), (1, 5.0)]);
}

#[test]
fn csr_spmv_matches_dense_mul_vec() {
    let triplets = [(0, 0, 2.0), (0, 1, 1.0), (1, 1, 3.0)];
    let m = CsrMatrix::from_triplets(2, 2, &triplets);
    let x = VecD::from_vec(vec![1.0, 2.0]);
    let sparse_result = m.spmv(&x);
    let dense_result = m.to_dense().mul_vec(&x);
    assert_eq!(sparse_result, dense_result);
}

#[test]
fn csr_transpose_swaps_rows_and_cols() {
    let triplets = [(0, 1, 5.0)];
    let m = CsrMatrix::from_triplets(2, 3, &triplets);
    let t = m.transpose();
    assert_eq!(t.rows, 3);
    assert_eq!(t.cols, 2);
    assert_eq!(t.row(1).collect::<Vec<_>>(), vec![(0, 5.0)]);
}

#[test]
fn algebra_error_display_is_human_readable() {
    assert_eq!(AlgebraError::Singular.to_string(), "matrix is singular");
    assert_eq!(AlgebraError::NotSymmetric.to_string(), "matrix is not symmetric");
}

#[test]
fn cholesky_matches_hand_solved_spd_system() {
    let mut a = MatD::zeros(2, 2);
    a.set(0, 0, 4.0);
    a.set(0, 1, 2.0);
    a.set(1, 0, 2.0);
    a.set(1, 1, 3.0);
    let l = cholesky(&a).expect("SPD");
    let reconstructed = l.matmul(&l.transpose());
    for row in 0..2 {
        for col in 0..2 {
            assert!((reconstructed.get(row, col) - a.get(row, col)).abs() < 1e-9);
        }
    }
    let b = VecD::from_vec(vec![6.0, 5.0]);
    let x = cholesky_solve(&l, &b);
    let x_lu = a.lu_solve(&b).expect("solvable");
    for i in 0..2 {
        assert!((x.get(i) - x_lu.get(i)).abs() < 1e-9);
    }
}

#[test]
fn cholesky_rejects_non_positive_definite() {
    let mut a = MatD::zeros(2, 2);
    a.set(0, 0, 1.0);
    a.set(0, 1, 2.0);
    a.set(1, 0, 2.0);
    a.set(1, 1, 1.0);
    assert_eq!(cholesky(&a), Err(AlgebraError::NotPositiveDefinite));
}

#[test]
fn qr_householder_reconstructs_and_is_orthogonal() {
    let mut a = MatD::zeros(3, 2);
    a.set(0, 0, 1.0);
    a.set(0, 1, 0.0);
    a.set(1, 0, 0.0);
    a.set(1, 1, 1.0);
    a.set(2, 0, 1.0);
    a.set(2, 1, 1.0);
    let (q, r) = qr_householder(&a);
    let product = q.matmul(&r);
    for row in 0..3 {
        for col in 0..2 {
            assert!((product.get(row, col) - a.get(row, col)).abs() < 1e-9);
            if row > col {
                assert!(r.get(row, col).abs() < 1e-9);
            }
        }
    }
    let qt_q = q.transpose().matmul(&q);
    for i in 0..3 {
        for j in 0..3 {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!((qt_q.get(i, j) - expected).abs() < 1e-9);
        }
    }
}

#[test]
fn jacobi_eigen_diagonal_matrix_returns_sorted_diagonal() {
    let mut a = MatD::zeros(3, 3);
    a.set(0, 0, 5.0);
    a.set(1, 1, 1.0);
    a.set(2, 2, 3.0);
    let (vals, vecs) = jacobi_eigen_symmetric(&a, 100).expect("converges");
    assert!((vals[0] - 1.0).abs() < 1e-9);
    assert!((vals[1] - 3.0).abs() < 1e-9);
    assert!((vals[2] - 5.0).abs() < 1e-9);
    for col in 0..3 {
        let mut norm = 0.0;
        for row in 0..3 {
            norm += vecs.get(row, col) * vecs.get(row, col);
        }
        assert!((norm - 1.0).abs() < 1e-9);
    }
}

#[test]
fn jacobi_eigen_matches_characteristic_polynomial_roots() {
    let mut a = MatD::zeros(2, 2);
    a.set(0, 0, 2.0);
    a.set(0, 1, 1.0);
    a.set(1, 0, 1.0);
    a.set(1, 1, 2.0);
    let (vals, _) = jacobi_eigen_symmetric(&a, 100).expect("converges");
    assert!((vals[0] - 1.0).abs() < 1e-9);
    assert!((vals[1] - 3.0).abs() < 1e-9);
}

#[test]
fn jacobi_eigen_rejects_asymmetric_matrix() {
    let mut a = MatD::zeros(2, 2);
    a.set(0, 0, 1.0);
    a.set(0, 1, 2.0);
    a.set(1, 0, 3.0);
    a.set(1, 1, 4.0);
    assert_eq!(jacobi_eigen_symmetric(&a, 100), Err(AlgebraError::NotSymmetric));
}

#[test]
fn jacobi_eigen_reports_non_convergence_with_zero_sweeps() {
    let mut a = MatD::zeros(2, 2);
    a.set(0, 0, 2.0);
    a.set(0, 1, 1.0);
    a.set(1, 0, 1.0);
    a.set(1, 1, 2.0);
    assert_eq!(jacobi_eigen_symmetric(&a, 0), Err(AlgebraError::PowerIterationFailedConvergence { iterations: 0 }));
}

#[test]
fn power_iteration_finds_dominant_eigenpair() {
    let triplets = [(0, 0, 1.0), (1, 1, 5.0), (2, 2, 2.0)];
    let a = CsrMatrix::from_triplets(3, 3, &triplets);
    let (lambda, v) = power_iteration(&a, 200, 1e-10, 7).expect("converges");
    assert!((lambda - 5.0).abs() < 1e-6);
    assert!(v.get(1).abs() > 0.99);
}

#[test]
fn power_iteration_rejects_non_square() {
    let a = CsrMatrix::from_triplets(2, 3, &[]);
    assert!(matches!(power_iteration(&a, 10, 1e-6, 1), Err(AlgebraError::DimensionMismatch { .. })));
}

#[test]
fn conjugate_gradient_matches_hand_solved_system() {
    let triplets = [(0, 0, 2.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 3.0)];
    let a = CsrMatrix::from_triplets(2, 2, &triplets);
    let b = VecD::from_vec(vec![5.0, 10.0]);
    let x = conjugate_gradient(&a, &b, 1e-10, 100).expect("converges");
    assert!((x.get(0) - 1.0).abs() < 1e-6);
    assert!((x.get(1) - 3.0).abs() < 1e-6);
}

#[test]
fn conjugate_gradient_reports_non_convergence_with_zero_iterations() {
    let triplets = [(0, 0, 2.0), (1, 1, 2.0)];
    let a = CsrMatrix::from_triplets(2, 2, &triplets);
    let b = VecD::from_vec(vec![1.0, 1.0]);
    assert_eq!(conjugate_gradient(&a, &b, 1e-12, 0), Err(AlgebraError::PowerIterationFailedConvergence { iterations: 0 }));
}

#[test]
fn lanczos_extreme_eigen_finds_largest_on_diagonal_matrix() {
    let triplets = [(0, 0, 1.0), (1, 1, 4.0), (2, 2, 2.0), (3, 3, 3.0)];
    let a = CsrMatrix::from_triplets(4, 4, &triplets);
    let (vals, _) = lanczos_extreme_eigen(&a, 2, true, 4, 11).expect("converges");
    let mut sorted = vals;
    sorted.sort_by(|x, y| y.partial_cmp(x).unwrap());
    assert!((sorted[0] - 4.0).abs() < 1e-6);
    assert!((sorted[1] - 3.0).abs() < 1e-6);
}

#[test]
fn lanczos_extreme_eigen_finds_smallest_on_diagonal_matrix() {
    let triplets = [(0, 0, 1.0), (1, 1, 4.0), (2, 2, 2.0), (3, 3, 3.0)];
    let a = CsrMatrix::from_triplets(4, 4, &triplets);
    let (vals, _) = lanczos_extreme_eigen(&a, 2, false, 4, 11).expect("converges");
    let mut sorted = vals;
    sorted.sort_by(|x, y| x.partial_cmp(y).unwrap());
    assert!((sorted[0] - 1.0).abs() < 1e-6);
    assert!((sorted[1] - 2.0).abs() < 1e-6);
}

#[test]
fn lanczos_extreme_eigen_rejects_non_square() {
    let a = CsrMatrix::from_triplets(2, 3, &[]);
    assert!(matches!(lanczos_extreme_eigen(&a, 1, true, 5, 1), Err(AlgebraError::DimensionMismatch { .. })));
}

#[test]
fn expm_pade_of_zero_matrix_is_identity() {
    let z = MatD::zeros(3, 3);
    let e = expm_pade(&z);
    assert_eq!(e, MatD::identity(3));
}

#[test]
fn expm_pade_of_diagonal_matrix_is_elementwise_exp() {
    let mut a = MatD::zeros(2, 2);
    a.set(0, 0, 1.0);
    a.set(1, 1, 2.0);
    let e = expm_pade(&a);
    assert!((e.get(0, 0) - 1.0_f64.exp()).abs() < 1e-8);
    assert!((e.get(1, 1) - 2.0_f64.exp()).abs() < 1e-8);
    assert!(e.get(0, 1).abs() < 1e-12);
    assert!(e.get(1, 0).abs() < 1e-12);
}

fn seeded_mat(rows: usize, cols: usize, seed: u64) -> MatD {
    let mut state = seed;
    let mut m = MatD::zeros(rows, cols);
    for value in m.data.iter_mut() {
        state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        *value = ((state >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0;
    }
    m
}

fn planted_rank_deficient() -> MatD {
    let base = seeded_mat(6, 3, 77);
    let mut a = MatD::zeros(6, 4);
    for row in 0..6 {
        a.set(row, 0, base.get(row, 0));
        a.set(row, 1, base.get(row, 1));
        a.set(row, 2, base.get(row, 2));
        a.set(row, 3, base.get(row, 0) + base.get(row, 1));
    }
    a
}

fn assert_svd_round_trips(a: &MatD, u: &MatD, sigma: &[f64], v: &MatD) {
    let k = sigma.len();
    let mut s_mat = MatD::zeros(k, k);
    for (i, &s) in sigma.iter().enumerate() {
        s_mat.set(i, i, s);
    }
    let recon = u.matmul(&s_mat).matmul(&v.transpose());
    for row in 0..a.rows {
        for col in 0..a.cols {
            assert!((recon.get(row, col) - a.get(row, col)).abs() < 1e-9);
        }
    }
}

fn assert_orthonormal_columns(m: &MatD) {
    let gram = m.transpose().matmul(m);
    for i in 0..gram.rows {
        for j in 0..gram.cols {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!((gram.get(i, j) - expected).abs() < 1e-9);
        }
    }
}

#[test]
fn svd_reconstructs_tall_random_matrix() {
    let a = seeded_mat(8, 5, 20_260_719);
    let (u, sigma, v) = svd(&a).expect("converges");
    assert_eq!((u.rows, u.cols), (8, 5));
    assert_eq!(sigma.len(), 5);
    assert_eq!((v.rows, v.cols), (5, 5));
    assert!(sigma.iter().all(|s| *s >= 0.0));
    assert!(sigma.windows(2).all(|pair| pair[0] >= pair[1]));
    assert_svd_round_trips(&a, &u, &sigma, &v);
    assert_orthonormal_columns(&u);
    assert_orthonormal_columns(&v);
}

#[test]
fn svd_reconstructs_wide_random_matrix() {
    let a = seeded_mat(5, 8, 31);
    let (u, sigma, v) = svd(&a).expect("converges");
    assert_eq!((u.rows, u.cols), (5, 5));
    assert_eq!(sigma.len(), 5);
    assert_eq!((v.rows, v.cols), (8, 5));
    assert!(sigma.windows(2).all(|pair| pair[0] >= pair[1]));
    assert_svd_round_trips(&a, &u, &sigma, &v);
    assert_orthonormal_columns(&u);
    assert_orthonormal_columns(&v);
}

#[test]
fn svd_nullvector_finds_planted_kernel() {
    let a = planted_rank_deficient();
    let v = svd_nullvector(&a).expect("converges");
    assert!((v.norm2() - 1.0).abs() < 1e-9);
    assert!(a.mul_vec(&v).norm2() < 1e-9);
}

#[test]
fn solve_llsq_matches_normal_equations() {
    let a = seeded_mat(9, 3, 5);
    let b = VecD::from_vec((0..9).map(|i| (i as f64) * 0.5 - 2.0).collect());
    let x = solve_llsq(&a, &b).expect("full rank");
    let at = a.transpose();
    let x_ne = at.matmul(&a).lu_solve(&at.mul_vec(&b)).expect("solvable");
    for i in 0..3 {
        assert!((x.get(i) - x_ne.get(i)).abs() < 1e-9);
    }
}

#[test]
fn pseudo_inverse_satisfies_penrose_identity() {
    let a = planted_rank_deficient();
    let pinv = pseudo_inverse(&a, 1e-10).expect("converges");
    let round = a.matmul(&pinv).matmul(&a);
    for row in 0..a.rows {
        for col in 0..a.cols {
            assert!((round.get(row, col) - a.get(row, col)).abs() < 1e-9);
        }
    }
}

#[test]
fn weighted_normal_equations_match_hand_assembly() {
    let mut a = MatD::zeros(3, 2);
    a.set(0, 0, 1.0);
    a.set(0, 1, 2.0);
    a.set(1, 0, 3.0);
    a.set(1, 1, 4.0);
    a.set(2, 0, 5.0);
    a.set(2, 1, 6.0);
    let b = VecD::from_vec(vec![1.0, 2.0, 3.0]);
    let (ata, atb) = weighted_normal_equations(&a, &b, &[2.0, 1.0, 3.0]);
    assert!((ata.get(0, 0) - 86.0).abs() < 1e-12);
    assert!((ata.get(0, 1) - 106.0).abs() < 1e-12);
    assert!((ata.get(1, 0) - 106.0).abs() < 1e-12);
    assert!((ata.get(1, 1) - 132.0).abs() < 1e-12);
    assert!((atb.get(0) - 53.0).abs() < 1e-12);
    assert!((atb.get(1) - 66.0).abs() < 1e-12);
}

fn planted_diagonal_conjugated(planted: &[f64], seed: u64) -> MatD {
    let n = planted.len();
    let (_, q) = hessenberg(&seeded_mat(n, n, seed));
    let mut d = MatD::zeros(n, n);
    for (i, &value) in planted.iter().enumerate() {
        d.set(i, i, value);
    }
    q.matmul(&d).matmul(&q.transpose())
}

#[test]
fn hessenberg_similarity_orthogonality_and_shape() {
    for (n, seed) in [(4, 101), (4, 202), (5, 303), (5, 404), (6, 505)] {
        let a = seeded_mat(n, n, seed);
        let (h, q) = hessenberg(&a);
        assert_orthonormal_columns(&q);
        for row in 2..n {
            for col in 0..(row - 1) {
                assert!(h.get(row, col).abs() < 1e-8, "n={n} seed={seed} row={row} col={col} not zero: {}", h.get(row, col));
            }
        }
        let recon = q.matmul(&h).matmul(&q.transpose());
        for row in 0..n {
            for col in 0..n {
                assert!((recon.get(row, col) - a.get(row, col)).abs() < 1e-7, "n={n} seed={seed} mismatch at ({row},{col})");
            }
        }
    }
}

#[test]
#[should_panic(expected = "hessenberg requires a square matrix")]
fn hessenberg_rejects_non_square() {
    let a = MatD::zeros(2, 3);
    hessenberg(&a);
}

#[test]
fn real_schur_recovers_planted_real_spectrum() {
    let planted = [1.0, -2.0, 3.5, 0.25, -7.0];
    let a = planted_diagonal_conjugated(&planted, 909);
    let mut eigs = real_eigenvalues(&a).expect("converges");
    eigs.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
    let mut expected = planted.to_vec();
    expected.sort_by(|x, y| x.partial_cmp(y).unwrap());
    for (got, exp) in eigs.iter().zip(expected.iter()) {
        assert!(got.1.abs() < 1e-6, "expected real eigenvalue, got {got:?}");
        assert!((got.0 - exp).abs() < 1e-6 * exp.abs().max(1.0), "got {got:?} expected {exp}");
    }
}

#[test]
fn real_schur_recovers_planted_complex_conjugate_pairs() {
    let blocks = [(2.0, 3.0), (-1.0, 5.0)];
    let n = blocks.len() * 2;
    let (_, q) = hessenberg(&seeded_mat(n, n, 1717));
    let mut d = MatD::zeros(n, n);
    for (idx, &(re, im)) in blocks.iter().enumerate() {
        let base = idx * 2;
        d.set(base, base, re);
        d.set(base, base + 1, -im);
        d.set(base + 1, base, im);
        d.set(base + 1, base + 1, re);
    }
    let a = q.matmul(&d).matmul(&q.transpose());
    let eigs = real_eigenvalues(&a).expect("converges");
    let mut got: Vec<(f64, f64)> = eigs.iter().map(|&(re, im)| (re, im.abs())).collect();
    got.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap().then(x.1.partial_cmp(&y.1).unwrap()));
    let mut expected: Vec<(f64, f64)> = blocks.iter().flat_map(|&(re, im)| [(re, im.abs()), (re, im.abs())]).collect();
    expected.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap().then(x.1.partial_cmp(&y.1).unwrap()));
    for (g, e) in got.iter().zip(expected.iter()) {
        assert!((g.0 - e.0).abs() < 1e-6, "real part mismatch: got {g:?} expected {e:?}");
        assert!((g.1 - e.1).abs() < 1e-6, "imag magnitude mismatch: got {g:?} expected {e:?}");
    }
}

#[test]
fn real_schur_rejects_non_square() {
    let a = MatD::zeros(2, 3);
    assert!(matches!(real_schur(&a), Err(AlgebraError::DimensionMismatch { .. })));
}

#[test]
fn poly_roots_companion_matches_known_real_quartic_roots() {
    let coeffs = [24.0, -50.0, 35.0, -10.0, 1.0];
    let mut roots = poly_roots_companion(&coeffs).expect("converges");
    roots.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
    let expected = [1.0, 2.0, 3.0, 4.0];
    for (got, exp) in roots.iter().zip(expected.iter()) {
        assert!(got.1.abs() < 1e-6, "expected real root, got {got:?}");
        assert!((got.0 - exp).abs() < 1e-6, "got {got:?} expected {exp}");
    }
}

#[test]
fn poly_roots_companion_matches_known_complex_conjugate_pairs() {
    let coeffs = [4.0, 0.0, 5.0, 0.0, 1.0];
    let roots = poly_roots_companion(&coeffs).expect("converges");
    let mut mags: Vec<f64> = roots
        .iter()
        .map(|&(re, im)| {
            assert!(re.abs() < 1e-6, "expected zero real part, got {re}");
            im.abs()
        })
        .collect();
    mags.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!((mags[0] - 1.0).abs() < 1e-6);
    assert!((mags[1] - 1.0).abs() < 1e-6);
    assert!((mags[2] - 2.0).abs() < 1e-6);
    assert!((mags[3] - 2.0).abs() < 1e-6);
}

#[test]
fn poly_roots_companion_rejects_zero_leading_coefficient() {
    let coeffs = [1.0, 2.0, 0.0];
    assert_eq!(poly_roots_companion(&coeffs), Err(AlgebraError::Singular));
}

#[test]
fn poly_roots_companion_rejects_too_short_input() {
    assert_eq!(poly_roots_companion(&[5.0]), Err(AlgebraError::Singular));
}

#[test]
fn vec3d_sub_and_length_match_hand_computation() {
    let diff = vec3d_sub([3.0, 4.0, 0.0], [1.0, 1.0, 0.0]);
    assert_eq!(diff, [2.0, 3.0, 0.0]);
    assert!((vec3d_length([3.0, 4.0, 0.0]) - 5.0).abs() < 1e-12);
}

#[test]
fn algebra_error_display_covers_remaining_variants() {
    assert_eq!(AlgebraError::NotPositiveDefinite.to_string(), "matrix is not positive definite");
    assert_eq!(AlgebraError::DimensionMismatch { expected: (2, 2), got: (3, 3) }.to_string(), "dimension mismatch: expected (2, 2), got (3, 3)");
    assert_eq!(AlgebraError::PowerIterationFailedConvergence { iterations: 42 }.to_string(), "iterative solver failed to converge after 42 iterations");
}

#[test]
fn matd_matmul_skips_zero_entries_and_still_correct() {
    let mut m = MatD::zeros(2, 2);
    m.set(0, 0, 0.0);
    m.set(0, 1, 2.0);
    m.set(1, 0, 3.0);
    m.set(1, 1, 0.0);
    let out = m.matmul(&MatD::identity(2));
    assert_eq!(out, m);
}

#[test]
fn weighted_normal_equations_skips_zero_entries_in_a() {
    let mut a = MatD::zeros(2, 2);
    a.set(0, 0, 0.0);
    a.set(0, 1, 1.0);
    a.set(1, 0, 2.0);
    a.set(1, 1, 3.0);
    let b = VecD::from_vec(vec![1.0, 2.0]);
    let (ata, atb) = weighted_normal_equations(&a, &b, &[1.0, 1.0]);
    assert!((ata.get(0, 0) - 4.0).abs() < 1e-12);
    assert!((ata.get(0, 1) - 6.0).abs() < 1e-12);
    assert!((ata.get(1, 1) - 10.0).abs() < 1e-12);
    assert!((atb.get(0) - 4.0).abs() < 1e-12);
    assert!((atb.get(1) - 7.0).abs() < 1e-12);
}

#[test]
fn hessenberg_skips_already_zero_subdiagonal_column() {
    let mut a = MatD::zeros(4, 4);
    for i in 0..4 {
        a.set(i, i, (i + 1) as f64);
    }
    let (h, q) = hessenberg(&a);
    assert_eq!(h, a);
    assert_eq!(q, MatD::identity(4));
}

#[test]
fn qr_householder_skips_already_zero_column_segment() {
    let mut a = MatD::zeros(3, 3);
    a.set(0, 0, 1.0);
    a.set(0, 1, 5.0);
    a.set(0, 2, 3.0);
    a.set(1, 2, 1.0);
    a.set(2, 2, 5.0);
    let (q, r) = qr_householder(&a);
    let product = q.matmul(&r);
    for row in 0..3 {
        for col in 0..3 {
            assert!((product.get(row, col) - a.get(row, col)).abs() < 1e-7);
        }
    }
}

#[test]
fn svd_nullvector_rejects_matrix_with_zero_columns() {
    let a = MatD::zeros(2, 0);
    assert!(matches!(svd_nullvector(&a), Err(AlgebraError::DimensionMismatch { .. })));
}

#[test]
fn solve_llsq_detects_rank_deficient_system() {
    let a = planted_rank_deficient();
    let b = VecD::from_vec((0..6).map(|i| i as f64).collect());
    assert!(matches!(solve_llsq(&a, &b), Err(AlgebraError::Singular)));
}

fn tridiagonal_spd(n: usize) -> CsrMatrix {
    let mut triplets = Vec::new();
    for i in 0..n {
        triplets.push((i, i, 2.0));
        if i + 1 < n {
            triplets.push((i, i + 1, -1.0));
            triplets.push((i + 1, i, -1.0));
        }
    }
    CsrMatrix::from_triplets(n, n, &triplets)
}

mod long {
    use super::*;

    #[test]
    fn conjugate_gradient_converges_on_tridiagonal_system() {
        let n = 40;
        let a = tridiagonal_spd(n);
        let x_true = VecD::from_vec((0..n).map(|i| (i as f64) + 1.0).collect());
        let b = a.spmv(&x_true);
        let x = conjugate_gradient(&a, &b, 1e-10, 1000).expect("converges");
        for i in 0..n {
            assert!((x.get(i) - x_true.get(i)).abs() < 1e-6);
        }
    }

    #[test]
    fn lanczos_matches_dense_jacobi_on_tridiagonal() {
        let n = 16;
        let a_sparse = tridiagonal_spd(n);
        let a_dense = a_sparse.to_dense();
        let (lanczos_vals, _) = lanczos_extreme_eigen(&a_sparse, 3, true, n, 42).expect("lanczos succeeds");
        let (dense_vals, _) = jacobi_eigen_symmetric(&a_dense, 500).expect("jacobi succeeds");
        let mut dense_top3: Vec<f64> = dense_vals;
        dense_top3.sort_by(|a, b| b.partial_cmp(a).unwrap());
        dense_top3.truncate(3);
        let mut lanczos_sorted = lanczos_vals;
        lanczos_sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        for (l, d) in lanczos_sorted.iter().zip(dense_top3.iter()) {
            assert!((l - d).abs() < 1e-6);
        }
    }
}
