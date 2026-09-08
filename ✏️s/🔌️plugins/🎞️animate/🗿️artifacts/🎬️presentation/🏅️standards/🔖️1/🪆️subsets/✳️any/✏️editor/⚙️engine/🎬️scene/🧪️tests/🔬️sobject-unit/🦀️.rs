mod tests {
    use super::*;
    use geometry::Circle;

    #[test]
    fn vobject_has_finite_bounds() {
        let dot = VSobject::from_shape(&Circle::new(Point::new(0.0, 0.0), 1.0));
        let b = dot.bounds();
        assert!(b.max.x() > b.min.x());
    }

    #[test]
    fn parent_opacity_multiplies() {
        let mut v = VSobject::new();
        v.set_opacity(0.5);
        v.set_parent_opacity(0.5);
        assert!((v.effective_opacity() - 0.25).abs() < 1e-9);
    }

    #[test]
    fn group_propagates_parent_opacity() {
        let mut g = Group::new(vec![VSobject::new().into()]);
        g.set_opacity(0.5);
        assert!((g.children[0].effective_opacity() - 0.5).abs() < 1e-9);
    }

    fn square_vobj(center: Point, half: f64) -> VSobject {
        let mut path = BezPath::new();
        path.move_to(Point::new(center.x() - half, center.y() - half));
        path.line_to(Point::new(center.x() + half, center.y() - half));
        path.line_to(Point::new(center.x() + half, center.y() + half));
        path.line_to(Point::new(center.x() - half, center.y() + half));
        path.close_path();
        VSobject::from_path(path)
    }

    #[test]
    fn next_to_places_mover_right_of_anchor() {
        let anchor: Sobjects = square_vobj(Point::ZERO, 1.0).into();
        let mut mover: Sobjects = square_vobj(Point::ZERO, 0.5).into();
        next_to(&mut mover, &anchor, Vec2::new(1.0, 0.0), 0.2);
        let b = mover.bounds();
        assert!((b.min.x() - 1.2).abs() < 1e-6);
    }

    #[test]
    fn next_to_places_mover_below_anchor() {
        let anchor: Sobjects = square_vobj(Point::ZERO, 1.0).into();
        let mut mover: Sobjects = square_vobj(Point::ZERO, 0.5).into();
        next_to(&mut mover, &anchor, Vec2::new(0.0, -1.0), 0.2);
        let b = mover.bounds();
        assert!((b.max.y() - (-1.2)).abs() < 1e-6);
    }

    #[test]
    fn next_to_zero_direction_defaults_to_right() {
        let anchor: Sobjects = square_vobj(Point::ZERO, 1.0).into();
        let mut mover: Sobjects = square_vobj(Point::ZERO, 0.5).into();
        next_to(&mut mover, &anchor, Vec2::new(0.0, 0.0), 0.2);
        let b = mover.bounds();
        assert!((b.min.x() - 1.2).abs() < 1e-6);
    }

    #[test]
    fn arrange_lays_children_along_direction() {
        let children: Vec<Sobjects> = vec![square_vobj(Point::ZERO, 0.5).into(), square_vobj(Point::ZERO, 0.5).into(), square_vobj(Point::ZERO, 0.5).into()];
        let mut g = Group::new(children);
        arrange(&mut g, Vec2::new(1.0, 0.0), 0.5);
        assert!((g.children[0].center().x() - 0.0).abs() < 1e-6);
        assert!((g.children[1].center().x() - 1.0).abs() < 1e-6);
        assert!((g.children[2].center().x() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn arrange_on_empty_group_is_noop() {
        let mut g = Group::empty();
        arrange(&mut g, Vec2::new(1.0, 0.0), 0.5);
        assert!(g.children.is_empty());
    }

    #[test]
    fn align_to_all_edges() {
        let anchor: Sobjects = square_vobj(Point::ZERO, 1.0).into();
        for edge in [AlignEdge::Left, AlignEdge::Right, AlignEdge::Up, AlignEdge::Down, AlignEdge::Center] {
            let mut mover: Sobjects = square_vobj(Point::new(5.0, 5.0), 0.5).into();
            align_to(&mut mover, &anchor, edge);
            let b = mover.bounds();
            match edge {
                AlignEdge::Left => assert!((b.min.x() - (-1.0)).abs() < 1e-6),
                AlignEdge::Right => assert!((b.max.x() - 1.0).abs() < 1e-6),
                AlignEdge::Up => assert!((b.max.y() - 1.0).abs() < 1e-6),
                AlignEdge::Down => assert!((b.min.y() - (-1.0)).abs() < 1e-6),
                AlignEdge::Center => {
                    assert!(b.center().x().abs() < 1e-6);
                    assert!(b.center().y().abs() < 1e-6);
                }
            }
        }
    }

    #[test]
    fn center_of_points_empty_is_zero() {
        assert_eq!(center_of_points(&[]), Point::ZERO);
    }

    #[test]
    fn center_of_points_nonempty_matches_centroid() {
        let pts = [Point::new(-1.0, -1.0), Point::new(1.0, -1.0), Point::new(1.0, 1.0), Point::new(-1.0, 1.0)];
        let c = center_of_points(&pts);
        assert!(c.x().abs() < 1e-9);
        assert!(c.y().abs() < 1e-9);
    }

    #[test]
    fn trim_path_at_ratio_boundary_and_partial() {
        let mut path = BezPath::new();
        path.move_to(Point::new(0.0, 0.0));
        path.line_to(Point::new(10.0, 0.0));
        let full = trim_path_at_ratio(&path, 1.0);
        assert_eq!(full.elements().len(), path.elements().len());
        let none = trim_path_at_ratio(&path, 0.0);
        assert!(none.elements().is_empty());
        let half = trim_path_at_ratio(&path, 0.5);
        assert!(!half.elements().is_empty());
        let over = trim_path_at_ratio(&path, 2.0);
        assert_eq!(over.elements().len(), path.elements().len());
    }
}
