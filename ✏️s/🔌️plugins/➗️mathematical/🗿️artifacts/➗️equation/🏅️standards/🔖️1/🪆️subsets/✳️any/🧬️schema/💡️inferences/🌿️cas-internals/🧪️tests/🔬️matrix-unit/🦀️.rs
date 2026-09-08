mod tests {
    use super::*;

    fn e(v: i64) -> Expr {
        Expr::integer(v)
    }

    #[semio_framework_async_macros::async_test]
    async fn det_2x2_hand_case() {
        let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(3), e(4)]]);
        assert_eq!(m.det(), e(-2));
    }

    #[semio_framework_async_macros::async_test]
    async fn det_symbolic_2x2() {
        let a = Expr::symbol("a");
        let b = Expr::symbol("b");
        let c = Expr::symbol("c");
        let d = Expr::symbol("d");
        let m = SymMatrix::from_rows(vec![vec![a.clone(), b.clone()], vec![c.clone(), d.clone()]]);
        let expected = a * d - b * c;
        assert_eq!(m.det(), expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_times_original_is_identity() {
        let m = SymMatrix::from_rows(vec![vec![e(2), e(1)], vec![e(1), e(1)]]);
        let inv = m.inverse().unwrap();
        let product = m.matmul(&inv);
        for r in 0..2 {
            for c in 0..2 {
                let expected = if r == c { e(1) } else { e(0) };
                assert_eq!(crate::cas::simplify::cancel(product.get(r, c)), expected);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn singular_matrix_has_no_inverse() {
        let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(2), e(4)]]);
        assert!(m.inverse().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn charpoly_and_eigenvalues_of_diagonal_matrix() {
        let m = SymMatrix::from_rows(vec![vec![e(2), e(0)], vec![e(0), e(5)]]);
        match m.eigenvalues() {
            SolutionSet::Finite(mut vals) => {
                vals.sort();
                assert_eq!(vals, vec![e(2), e(5)]);
            }
            other => panic!("expected Finite eigenvalues, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn cayley_hamilton_holds_for_a_3x3_matrix() {
        // Verify A^2 - tr(A)*A + det(A)*I == 0 for a 2x2 matrix (Cayley-Hamilton).
        let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(3), e(4)]]);
        let a2 = m.matmul(&m);
        let tr_a = m.trace();
        let det_a = m.det();
        let lhs = a2.sub(&m.scale(&tr_a)).add(&SymMatrix::identity(2).scale(&det_a));
        for r in 0..2 {
            for c in 0..2 {
                assert_eq!(crate::cas::simplify::simplify(lhs.get(r, c)), e(0));
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn rank_of_numeric_matrix() {
        let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(2), e(4)]]);
        assert_eq!(m.rank(), Some(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn rref_of_numeric_matrix() {
        let m = SymMatrix::from_rows(vec![vec![e(2), e(4)], vec![e(1), e(1)]]);
        let (rref, _pivots, rank) = m.rref().unwrap();
        assert_eq!(rank, 2);
        assert_eq!(rref, SymMatrix::identity(2));
    }

    #[semio_framework_async_macros::async_test]
    async fn solve_numeric_linear_system() {
        let m = SymMatrix::from_rows(vec![vec![e(2), e(1)], vec![e(1), e(3)]]);
        let x = m.solve_numeric(&[e(5), e(10)]).unwrap();
        assert_eq!(x, vec![e(1), e(3)]);
    }
}
