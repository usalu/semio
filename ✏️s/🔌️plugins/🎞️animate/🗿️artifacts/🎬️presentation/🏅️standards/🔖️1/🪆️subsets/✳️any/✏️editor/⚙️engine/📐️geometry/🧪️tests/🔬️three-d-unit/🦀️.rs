mod tests {
    use super::*;

    #[test]
    fn cube_has_twelve_edges() {
        let g = cube(2.0, (0.0, 0.0, 0.0), Color::WHITE);
        assert_eq!(g.children.len(), 12);
    }

    #[test]
    fn projection_moves_points() {
        let td = ThreeDVSobject::new(VSobject::new());
        let p = td.project_point((1.0, 0.0, 0.0));
        assert!(p.x().is_finite());
    }

    #[test]
    fn three_d_vobject_is_sobject() {
        let td = ThreeDVSobject::new(VSobject::new());
        assert_eq!(td.opacity(), 1.0);
    }

    #[test]
    fn solid_cube_has_faces() {
        let g = solid_cube(2.0, (0.0, 0.0, 0.0), Color::BLUE, Some(Color::WHITE), 1.0);
        assert!(g.children.len() >= 6);
    }

    #[test]
    fn sphere_builds_wireframe_lines() {
        let g = sphere(1.0, (0.0, 0.0, 0.0), Color::WHITE);
        assert!(!g.children.is_empty());
    }

    #[test]
    fn face_and_disc_build_projected_shapes() {
        let f = face(2.0, 1.0, Point::ZERO, Color::RED);
        assert!(!f.paths.is_empty());
        let d = disc(1.0, Point::ZERO, Color::BLUE);
        assert!(!d.paths.is_empty());
    }
}
