mod tests {
    use super::*;

    #[test]
    fn moving_camera_interpolates_center() {
        let mut cam = MovingCamera::new(Camera::default());
        cam.set_target(Point::new(2.0, 2.0), 8.0);
        cam.interpolate(0.5);
        assert!(cam.camera.frame_center.x().abs() < 2.0);
    }

    #[test]
    fn three_d_camera_projects_finite_point() {
        let cam = ThreeDCamera::new(Camera::default());
        let p = cam.project(1.0, 1.0, 1.0);
        assert!(p.x().is_finite() && p.y().is_finite());
    }
}
