use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn set_data_fields_diff_applies_onto_the_base_snapshot() {
    let base = LayoutSnapshot {
        schema: crate::LAYOUT_DOCUMENT_SCHEMA.into(),
        name: "t".into(),
        grid: crate::GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
        paragraph_styles: Vec::new(),
        character_styles: Vec::new(),
        stories: Vec::new(),
        links: Vec::new(),
        parent_pages: Vec::new(),
        spreads: Vec::new(),
        pages: Vec::new(),
        print_target: None,
        data_fields_json: None,
        background_drawing: None,
        referenced_model: None,
    };
    let operation = crate::mutations::LayoutMutation::ChangeDataFields(crate::mutations::change_data_fields::ChangeDataFields { new_json: Some("{}".into()) });
    let diff: LayoutDiff = operation.diff(&base).into_parts().0;
    let applied = diff.apply(&base).expect("valid mutation diff");
    assert_eq!(applied.data_fields_json.as_deref(), Some("{}"));
}

#[semio_framework_async_macros::async_test]
async fn absorb_replaces_with_whole_artifact_diff() {
    let mut diff = LayoutDiff::default();
    let snap = LayoutSnapshot {
        schema: crate::LAYOUT_DOCUMENT_SCHEMA.into(),
        name: "x".into(),
        grid: crate::GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
        paragraph_styles: Vec::new(),
        character_styles: Vec::new(),
        stories: Vec::new(),
        links: Vec::new(),
        parent_pages: Vec::new(),
        spreads: Vec::new(),
        pages: Vec::new(),
        print_target: None,
        data_fields_json: None,
        background_drawing: None,
        referenced_model: None,
    };
    diff.absorb(diff_set_snapshot(&snap));
    assert!(diff.artifact.is_some());
}
