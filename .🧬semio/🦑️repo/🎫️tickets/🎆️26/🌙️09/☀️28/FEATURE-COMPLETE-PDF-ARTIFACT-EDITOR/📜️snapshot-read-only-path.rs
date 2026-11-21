pub(super) fn schema_read_only(root: &DslValue, schema: &DslValue) -> bool {
    let annotated = |value| matches!(object_field(value, "readOnly"), Some(DslValue::Bool(true)));
    annotated(schema) || resolved_schema(root, schema, 0).is_some_and(annotated)
}

pub(super) fn schema_path_read_only(
    root: &DslValue,
    path: &[SnapshotDetailPathSegment],
    read: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<DslValue>,
    shape: &mut impl FnMut(&[SnapshotDetailPathSegment]) -> Option<ValueShape>,
) -> bool {
    let mut schema = root;
    for depth in 0..=path.len() {
        let prefix = &path[..depth];
        if schema_read_only(root, schema) { return true; }
        if let Some(selected) = selected_schema(root, schema, prefix, read, shape) {
            if schema_read_only(root, selected) { return true; }
            schema = selected;
        }
        let Some(segment) = path.get(depth) else { break; };
        let Some(child) = schema_for_segment(root, schema, prefix, segment, read, shape) else { break; };
        schema = child;
    }
    false
}
