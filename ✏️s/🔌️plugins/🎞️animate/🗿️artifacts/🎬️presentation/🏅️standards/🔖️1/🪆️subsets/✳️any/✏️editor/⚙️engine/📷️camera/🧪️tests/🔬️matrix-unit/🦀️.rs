mod tests {
    use super::*;

    #[test]
    fn decimal_matrix_lerps() {
        let a = DecimalMatrix::new(vec![vec![0.0, 1.0]]);
        let b = DecimalMatrix::new(vec![vec![2.0, 3.0]]);
        let m = a.lerp(&b, 0.5);
        assert!((m.values[0][0] - 1.0).abs() < 1e-9);
    }

    #[test]
    fn matrix_grid_layout() {
        let m = Matrix::from_rows(vec![vec!["a".into(), "b".into()], vec!["c".into(), "d".into()]], (1.0, 1.0), Color::WHITE);
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 2);
        assert_eq!(m.group.children.len(), 4);
    }

    #[test]
    fn table_has_header_and_rows() {
        let t = Table::new(vec!["x".into()], &[vec!["1".into()]], (1.0, 1.0), Color::WHITE);
        assert_eq!(t.rows, 2);
        assert_eq!(t.cols, 1);
    }

    #[test]
    fn table_with_frame_adds_border_child() {
        let t = Table::new(vec!["a".into(), "b".into()], &[vec!["1".into()]], (1.0, 1.0), Color::WHITE);
        let before = t.group.children.len();
        let framed = t.with_frame(Color::WHITE, 0.2);
        assert_eq!(framed.group.children.len(), before + 1);
    }

    #[test]
    fn matrix_math_lays_out_entries() {
        let m = Matrix::math(&["1", "2", "3", "4"], (1.0, 1.0), Color::WHITE);
        assert_eq!(m.group.children.len(), 4);
        assert_eq!(m.cols, 2);
        assert_eq!(m.rows, 2);
    }

    #[test]
    fn matrix_with_brackets_adds_frame_child() {
        let m = Matrix::from_rows(vec![vec!["a".into()]], (1.0, 1.0), Color::WHITE);
        let before = m.group.children.len();
        let bracketed = m.with_brackets(Color::WHITE, 0.1);
        assert_eq!(bracketed.group.children.len(), before + 1);
    }

    #[test]
    fn decimal_matrix_to_matrix_sobject_formats_values() {
        let d = DecimalMatrix::new(vec![vec![1.5, 2.25]]);
        let m = d.to_matrix_sobject((1.0, 1.0), Color::WHITE);
        assert_eq!(m.group.children.len(), 2);
    }
}
