use super::*;

#[semio_framework_async_macros::async_test]
async fn a_whole_artifact_diff_wins_over_every_field_diff() {
    let base = crate::empty_process3d_snapshot();
    let replacement = Process3dSnapshot { stock_label: "Beam".into(), ..crate::empty_process3d_snapshot() };
    let mut diff = Process3dDiff { stock_label: Some("Ignored".into()), ..Default::default() };
    diff.absorb(diff_set_snapshot(&replacement));
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), replacement);
    println!("[DEBUG] Process3D schema diff absorption preserves the complete replacement snapshot");
}

#[semio_framework_async_macros::async_test]
async fn stock_solid_handle_swap_applies() {
    let base = crate::empty_process3d_snapshot();
    let new_content = crate::brep_snapshot_for_working_solid(&crate::WorkingSolid::Sphere { radius: 0.5 });
    let new_handle = crate::brep_child_handle("stock", &new_content);
    let diff = Process3dDiff { stock_solid: Some(new_handle.clone()), ..Default::default() };
    let next = diff.apply(&base).expect("valid mutation diff");
    assert_eq!(next.stock_solid, new_handle);
    println!("[DEBUG] Process3D schema diff replaces the exact stock child identity");
}
