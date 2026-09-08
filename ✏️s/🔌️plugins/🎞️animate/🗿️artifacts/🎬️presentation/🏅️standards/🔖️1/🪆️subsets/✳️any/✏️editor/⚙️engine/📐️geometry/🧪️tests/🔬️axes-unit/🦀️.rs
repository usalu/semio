mod tests {
    use super::*;

    #[test]
    fn axes_map_coordinates() {
        let axes = Axes::new(4.0, 3.0, Point::ZERO, Color::WHITE);
        let p = axes.coords_to_point(1.0, 2.0);
        assert!((p.x() - 1.0).abs() < 1e-9);
        assert!((p.y() - 2.0).abs() < 1e-9);
    }

    #[test]
    fn number_line_maps_values() {
        let nl = NumberLine::new(Point::ZERO, 10.0, Color::WHITE);
        assert!((nl.number_to_point(5.0).x() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn integer_line_maps_values() {
        let il = IntegerLine::new(Point::ZERO, 0, 10, 1.0, Color::WHITE);
        assert!((il.integer_to_point(5).x() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn axes_tick_labels_and_graphs() {
        let axes = Axes::new(4.0, 3.0, Point::ZERO, Color::WHITE).with_tick_labels(&[1.0, 2.0], &[1.0], Color::WHITE);
        assert!(axes.group.children.len() > 2);
        let fg = FunctionGraph::new((0.0, 2.0), &axes, |x| x * x, 16, Color::YELLOW, 2.0);
        assert!(!fg.inner.paths.is_empty());
        let pf = ParametricFunction::new((0.0, std::f64::consts::TAU), &axes, |t| (t.cos(), t.sin()), 32, Color::GREEN, 2.0);
        assert!(!pf.inner.paths.is_empty());
    }
}
