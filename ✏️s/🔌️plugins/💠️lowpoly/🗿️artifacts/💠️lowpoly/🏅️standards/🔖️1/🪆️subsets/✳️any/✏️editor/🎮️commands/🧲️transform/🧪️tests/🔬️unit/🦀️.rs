use crate::editor::lowpoly::unit_tests::context::{act, app, committed_edits, dispatch};
use crate::editor::lowpoly::LowpolyCommand;

#[semio_framework_async_macros::async_test]
async fn gumball_drag_coalesces_to_one_committed_edit() {
    // 🧲️ THE COALESCING REGRESSION: a multi-tick gumball translate must commit NOTHING mid-drag and
    // exactly ONE document edit (base → final mesh) on drag end — never a full-mesh patch per tick.
    let mut a = app().await;
    let before_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
    let edits_before = committed_edits(&mut a).await;
    dispatch(&mut a, LowpolyCommand::TransformBegin(super::transform_begin::TransformBegin {})).await;
    dispatch(&mut a, LowpolyCommand::TranslateSelection(super::translate_selection::TranslateSelection { mode: Some("mesh".into()), ids: Some(vec![]), dx: 0.5, dy: 0.0, dz: 0.0 })).await;
    dispatch(&mut a, LowpolyCommand::TranslateSelection(super::translate_selection::TranslateSelection { mode: Some("mesh".into()), ids: Some(vec![]), dx: 0.25, dy: 0.0, dz: 0.0 })).await;
    assert_eq!(committed_edits(&mut a).await, edits_before, "mid-drag transform ticks commit no document edit");
    assert_eq!(a.snapshot().expect("projection").objects[0].mesh, before_mesh, "no operation reached the document mid-drag");
    dispatch(&mut a, LowpolyCommand::TransformEnd(super::transform_end::TransformEnd {})).await;
    assert_eq!(committed_edits(&mut a).await, edits_before + 1, "the whole drag commits as exactly one document edit");
    let after_mesh = a.snapshot().expect("projection").objects[0].mesh.clone();
    assert_ne!(after_mesh, before_mesh, "the drag moved the mesh");
    act(&mut a, "undo", serde_json::json!({})).await;
    assert_eq!(a.snapshot().expect("projection").objects[0].mesh, before_mesh, "one undo reverts the whole coalesced gumball drag");
}
