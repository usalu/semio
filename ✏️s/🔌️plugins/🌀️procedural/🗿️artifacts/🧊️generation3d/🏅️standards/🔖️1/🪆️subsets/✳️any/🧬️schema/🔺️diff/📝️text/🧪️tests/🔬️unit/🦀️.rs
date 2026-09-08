
use super::*;
use crate::standards::v1::subsets::any::schema::Generation3dPreviewCamera;

#[test]
fn diff_absorb_prefers_incoming_scalars() {
    let mut first = Generation3dDiff { show_mode: Some("shaded".into()), ..Generation3dDiff::default() };
    first.absorb(Generation3dDiff { locale: Some("de-DE".into()), preview_camera: Some(Generation3dPreviewCamera::default()), ..Generation3dDiff::default() });
    assert_eq!(first.show_mode.as_deref(), Some("shaded"));
    assert_eq!(first.locale.as_deref(), Some("de-DE"));
    assert!(first.preview_camera.is_some());
}
