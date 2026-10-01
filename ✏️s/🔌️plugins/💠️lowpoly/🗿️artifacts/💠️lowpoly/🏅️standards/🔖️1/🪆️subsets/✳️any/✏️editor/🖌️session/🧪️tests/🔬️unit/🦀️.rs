use super::*;
use crate::schema::default_snapshot;

fn stroke_gesture(object_id: &str, u: f32) -> LowpolyPaintGesture {
    LowpolyPaintGesture {
        states: vec!["streaming".into()],
        authoring_seed: "seed".into(),
        base_revision: "00".into(),
        transaction: protocol::TransactionRef { id: "tx-0000000000000001".into(), tool: "s.lowpoly.lowpoly@1/*#editor#paint".into() },
        leaf: LowpolyMutation::ApplyPaintStroke(crate::mutations::apply_paint_stroke::ApplyPaintStroke { object_id: object_id.into(), layer_index: 0, eraser: false, color: [200, 30, 30, 255], radius: 4.0, hardness: 1.0, opacity: 1.0, points: vec![[u, 0.5]] }),
    }
}

/// 🫧️ The transient's pack and its scratch rehydration are exact: an open paint gesture survives both, and a
/// republished cache never drops it.
#[test]
fn transient_pack_and_scratch_round_trip_exactly() {
    let transient = LowpolyTransient::default().with_paint("lowpoly-main", Some(stroke_gesture("obj-1", 0.5)));
    let pack = transient.encode_pack();
    assert_eq!(LowpolyTransient::decode_pack(&pack).expect("transient pack"), transient);
    let scratch = LowpolyScratch::from_transient(&transient, LowpolySelection::default());
    assert_eq!(scratch.transient_snapshot(), transient, "the scratch republishes the gesture it rehydrated");
}

/// 🪟️ Paint gestures are keyed by their owning window: clearing one leaves the other, and the mesh root stays shared.
#[test]
fn paint_gestures_are_keyed_by_window_and_share_the_mesh_root() {
    let transient = LowpolyTransient::with_test_workspace_bytes(LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4);
    let both = transient.with_paint("lowpoly-main", Some(stroke_gesture("obj-1", 0.25))).with_paint("lowpoly-uv", Some(stroke_gesture("obj-1", 0.75)));
    let uv_only = both.with_paint("lowpoly-main", None);
    assert!(Arc::ptr_eq(&transient.state.mesh_workspace, &uv_only.state.mesh_workspace), "a paint gesture never copies the mesh root");
    assert!(uv_only.paint("lowpoly-main").is_none());
    assert_eq!(uv_only.paint("lowpoly-uv"), both.paint("lowpoly-uv"));
}

/// 👁️ The preview applies every window's provisional leaf and is `None` without an open gesture.
#[test]
fn paint_preview_applies_every_open_leaf() {
    let document = default_snapshot();
    assert!(LowpolyTransient::default().paint_preview(&document).is_none(), "nothing open, nothing to preview");
    let preview = LowpolyTransient::default().with_paint("lowpoly-main", Some(stroke_gesture("obj-1", 0.5))).paint_preview(&document).expect("an open stroke previews");
    assert_ne!(preview.objects[0].paint_layers[0].materialized_pixels(), document.objects[0].paint_layers[0].materialized_pixels(), "the provisional dab shows");
    assert_eq!(preview.objects[0].mesh_content, document.objects[0].mesh_content, "a paint preview never touches the mesh");
}
