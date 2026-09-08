mod tests {
    use super::*;

    #[test]
    fn shapes_produce_paths() {
        let c = circle(Point::ZERO, 1.0, Color::BLUE, None, 0.0);
        assert!(!c.paths.is_empty());
        let a = arrow(Point::ZERO, Point::new(2.0, 0.0), Color::RED, 2.0, 0.3);
        assert!(!a.paths.is_empty());
    }

    #[test]
    fn star_has_vertices() {
        let s = star(5, 1.0, 0.4, Point::ZERO, Color::YELLOW, None, 0.0);
        assert!(s.paths[0].elements().len() > 4);
    }

    #[test]
    fn ellipse_and_regular_polygon_build() {
        let e = ellipse(Point::ZERO, 2.0, 1.0, Color::BLUE, None, 0.0);
        assert!(!e.paths.is_empty());
        let p = regular_polygon(6, 1.0, Point::ZERO, Color::GREEN, None, 0.0);
        assert!(!p.paths.is_empty());
    }

    #[test]
    fn dashed_line_has_multiple_segments() {
        let d = dashed_line(Point::ZERO, Point::new(4.0, 0.0), Color::WHITE, 2.0, 0.3, 0.2);
        assert!(d.paths.len() > 1);
    }

    #[test]
    fn boolean_ops_combine_paths() {
        let a = circle(Point::ZERO, 1.0, Color::BLUE, None, 0.0);
        let b = circle(Point::new(0.5, 0.0), 1.0, Color::RED, None, 0.0);
        let u = boolean_union(&a, &b, Color::PURPLE, None, 0.0);
        assert!(u.paths.len() >= 2);
        let diff = boolean_difference(&a, &b, Color::YELLOW, None, 0.0);
        assert!(diff.paths.len() >= 2);
    }

    #[test]
    fn vector_field_helpers_build() {
        let vf = arrow_vector_field((-1.0, 1.0), (-1.0, 1.0), 3, 3, |x, _| Vec2::new(x, 1.0), Color::TEAL, 0.2);
        assert!(!vf.children.is_empty());
        let sl = stream_lines(&[(0.0, 0.0)], |_, y| Vec2::new(1.0, y), Color::WHITE, 8, 0.1);
        assert!(!sl.children.is_empty());
    }

    #[test]
    fn vector_field_skips_zero_length_vectors() {
        let vf = arrow_vector_field((-1.0, 1.0), (-1.0, 1.0), 2, 2, |_, _| Vec2::new(0.0, 0.0), Color::TEAL, 0.2);
        assert!(vf.children.is_empty());
    }

    #[test]
    fn stream_lines_stops_on_zero_length_field() {
        let sl = stream_lines(&[(0.0, 0.0)], |_, _| Vec2::new(0.0, 0.0), Color::WHITE, 8, 0.1);
        assert_eq!(sl.children.len(), 1);
    }

    #[test]
    fn point_dot_and_line_build_paths() {
        let p = point(Point::ZERO, 0.1, Color::RED);
        assert!(!p.paths.is_empty());
        let d = dot(Point::ZERO, 0.1, Color::RED);
        assert!(!d.paths.is_empty());
        let l = line(Point::ZERO, Point::new(1.0, 1.0), Color::BLUE, 1.0);
        assert!(!l.paths.is_empty());
    }

    #[test]
    fn square_triangle_and_polygon_build() {
        let sq = square(2.0, Point::ZERO, Color::RED, None, 0.0);
        assert!(!sq.paths.is_empty());
        let tri = triangle(2.0, Point::ZERO, Color::GREEN, None, 0.0);
        assert!(!tri.paths.is_empty());
        let empty_poly = polygon(&[], Color::WHITE, None, 0.0);
        assert!(empty_poly.paths[0].elements().is_empty());
    }

    #[test]
    fn annulus_and_sector_build() {
        let a = annulus(Point::ZERO, 0.5, 1.0, Color::BLUE, None, 0.0);
        assert!(!a.paths.is_empty());
        let s = sector(Point::ZERO, 1.0, 0.0, PI / 2.0, Color::YELLOW, None, 0.0);
        assert!(!s.paths.is_empty());
    }

    #[test]
    fn brace_and_angle_build() {
        let b = brace(Point::new(-1.0, 0.0), Point::new(1.0, 0.0), Vec2::new(0.0, -1.0), Color::WHITE, 1.0);
        assert!(!b.paths.is_empty());
        let b_default_dir = brace(Point::new(-1.0, 0.0), Point::new(1.0, 0.0), Vec2::new(0.0, 0.0), Color::WHITE, 1.0);
        assert!(!b_default_dir.paths.is_empty());
        let ang = angle(Point::ZERO, Point::new(1.0, 0.0), Point::new(0.0, 1.0), 0.3, Color::ORANGE, 1.0);
        assert!(!ang.paths.is_empty());
    }

    #[test]
    fn surrounding_rectangle_pads_bounds() {
        let c = circle(Point::ZERO, 1.0, Color::BLUE, None, 0.0);
        let r = surrounding_rectangle(&c, 0.5, Color::TRANSPARENT, Some(Color::WHITE), 1.0);
        assert!(!r.paths.is_empty());
    }

    #[test]
    fn dashed_line_degenerate_endpoints_falls_back_to_line() {
        let d = dashed_line(Point::new(1.0, 1.0), Point::new(1.0, 1.0), Color::WHITE, 2.0, 0.3, 0.2);
        assert_eq!(d.paths.len(), 1);
    }
}
