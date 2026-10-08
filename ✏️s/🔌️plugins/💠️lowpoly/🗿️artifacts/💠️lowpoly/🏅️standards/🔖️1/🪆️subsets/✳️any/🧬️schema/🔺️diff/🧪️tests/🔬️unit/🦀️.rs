use super::*;

fn layer(name: &str) -> LowpolyPaintLayer {
    LowpolyPaintLayer::new(name)
}

fn layers(names: &[&str]) -> Vec<LowpolyPaintLayer> {
    names.iter().map(|name| layer(name)).collect()
}

#[semio_framework_async_macros::async_test]
async fn empty_diff_is_a_no_operation() {
    let base = LowpolySnapshot::default();
    assert_eq!(protocol::apply_diff(&LowpolyDiff::default(), &base).expect("valid mutation diff"), base);
}

/// ⚖️ LAW: paint edits fold per layer and their negative sequence restores the list, including a middle layer.
#[semio_framework_async_macros::async_test]
async fn paint_edits_fold_and_invert() {
    let base = layers(&["a", "b", "c"]);
    let remove_then_insert = canonical_paint_edits(vec![LowpolyPaintEdit::Remove { index: 1 }, LowpolyPaintEdit::Insert { index: 1, layer: layer("x") }]);
    assert_eq!(remove_then_insert, vec![LowpolyPaintEdit::Replace { index: 1, layer: layer("x") }]);
    assert!(canonical_paint_edits(vec![LowpolyPaintEdit::Insert { index: 2, layer: layer("x") }, LowpolyPaintEdit::Remove { index: 2 }]).is_empty());
    let edits = vec![LowpolyPaintEdit::Remove { index: 1 }, LowpolyPaintEdit::Patch { index: 0, patch: LowpolyPaintLayerPatch { opacity: Some(0.5), ..Default::default() } }];
    let after = paint_edits_after(&base, &edits).expect("edits apply");
    assert_eq!(after.iter().map(|layer| layer.name.as_str()).collect::<Vec<_>>(), ["a", "c"]);
    let restored = paint_edits_after(&after, &paint_edits_inverse(&base, &edits)).expect("negative edits apply");
    assert_eq!(restored, base);
}

/// ⚖️ LAW: a stroke on a never-painted layer is exactly undone by the stroke of the old bytes (the sparse layer is restored).
#[semio_framework_async_macros::async_test]
async fn a_stroke_on_a_sparse_layer_inverts_to_the_sparse_layer() {
    let base = layers(&["a"]);
    let edits = vec![LowpolyPaintEdit::Stroke { index: 0, runs: vec![PixelRun { offset: 4, bytes: vec![1, 2, 3, 4] }] }];
    let after = paint_edits_after(&base, &edits).expect("stroke applies");
    assert!(!after[0].pixels.is_empty());
    let restored = paint_edits_after(&after, &paint_edits_inverse(&base, &edits)).expect("negative stroke applies");
    assert_eq!(restored, base);
}
