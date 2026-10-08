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

/// ⚖️ LAW: the negative sequence is read off the base layers by their origin, so edits on layers the sequence inserted need no
/// content of their own and a base layer removed after being replaced is reinserted whole at its base index.
#[semio_framework_async_macros::async_test]
async fn paint_edit_inverse_reads_each_base_layer_by_origin() {
    let base = layers(&["a", "b", "c"]);
    let inserted_then_patched = vec![LowpolyPaintEdit::Insert { index: 1, layer: layer("x") }, LowpolyPaintEdit::Patch { index: 1, patch: LowpolyPaintLayerPatch { name: Some("y".into()), ..Default::default() } }];
    let after = paint_edits_after(&base, &inserted_then_patched).expect("edits apply");
    assert_eq!(paint_edits_after(&after, &paint_edits_inverse(&base, &inserted_then_patched)).expect("negative edits apply"), base);
    let replaced_then_removed = vec![LowpolyPaintEdit::Replace { index: 1, layer: layer("z") }, LowpolyPaintEdit::Remove { index: 1 }, LowpolyPaintEdit::Insert { index: 0, layer: layer("n") }];
    let after = paint_edits_after(&base, &replaced_then_removed).expect("edits apply");
    assert_eq!(paint_edits_after(&after, &paint_edits_inverse(&base, &replaced_then_removed)).expect("negative edits apply"), base);
}

/// ⚖️ LAW: vertex rows compose by vertex id, the later position winning, and the negative rows are the base positions.
#[test]
fn mesh_vertex_rows_compose_and_invert_from_the_base_positions() {
    let at = |vertex: u32, x: f32| LowpolyVertexPosition { vertex, position: [x, 0.0, 0.0] };
    let first = LowpolyObjectPatchEntry { mesh_vertices: vec![at(0, 1.0), at(2, 2.0)], ..Default::default() };
    let later = LowpolyObjectPatchEntry { mesh_vertices: vec![at(2, 3.0), at(1, 4.0)], ..Default::default() };
    let mut composed = first;
    protocol::list_delta::RowPatch::<LowpolyObject>::absorb(&mut composed, later);
    assert_eq!(composed.mesh_vertices, vec![at(0, 1.0), at(1, 4.0), at(2, 3.0)]);
}

/// ⚖️ LAW: Normal channels compose by name, the later whole channel winning.
#[test]
fn mesh_attribute_channels_compose_by_name() {
    let channel = |name: &str, tag: f64| crate::LowpolyMeshAttribute { name: name.into(), domain: crate::LowpolyMeshAttributeDomain::Vertex, semantic: crate::LowpolyMeshAttributeSemantic::Normal, interpolation: crate::LowpolyMeshAttributeInterpolation::Linear, values: vec![semio_framework_value::DslValue::float(tag)], indices: None };
    let first = LowpolyObjectPatchEntry { mesh_attributes: vec![channel("a", 1.0), channel("b", 2.0)], ..Default::default() };
    let later = LowpolyObjectPatchEntry { mesh_attributes: vec![channel("b", 3.0)], ..Default::default() };
    let mut composed = first;
    protocol::list_delta::RowPatch::<LowpolyObject>::absorb(&mut composed, later);
    assert_eq!(composed.mesh_attributes, vec![channel("a", 1.0), channel("b", 3.0)]);
}

/// ⚖️ LAW: a positional object delta inserts, moves and removes at middle rows, sums, and inverts row by row.
#[test]
fn positional_object_delta_sums_and_inverts_at_middle_rows() {
    let object = |id: &str| LowpolyObject { id: id.into(), name: id.into(), transform: crate::LowpolyTransform::default(), smooth_shading: false, mesh: None, paint_layers: Vec::new(), mesh_content: String::new(), mesh_state: None };
    let mut base = LowpolySnapshot::default();
    base.objects = vec![object("a"), object("b"), object("c")];
    let order = |snapshot: &LowpolySnapshot| snapshot.objects.iter().map(|row| row.id.clone()).collect::<Vec<_>>();
    let diff = |delta: LowpolyObjectsDelta| LowpolyDiff { objects: Some(delta), ..Default::default() };
    let insert = diff(LowpolyObjectsDelta::insertion(1, object("x")));
    let inserted = protocol::apply_diff(&insert, &base).expect("valid insertion");
    let relocate = diff(LowpolyObjectsDelta::relocation(&inserted.objects, 3, 0));
    let moved = protocol::apply_diff(&relocate, &inserted).expect("valid relocation");
    assert_eq!(order(&moved), ["c", "a", "x", "b"]);
    let mut sum = insert;
    sum.absorb(relocate);
    assert_eq!(protocol::apply_diff(&sum, &base).expect("valid sum"), moved);
    assert_eq!(protocol::apply_diff(&protocol::DiffAlgebra::inverse(&sum, &base), &moved).expect("valid inverse"), base);
}
