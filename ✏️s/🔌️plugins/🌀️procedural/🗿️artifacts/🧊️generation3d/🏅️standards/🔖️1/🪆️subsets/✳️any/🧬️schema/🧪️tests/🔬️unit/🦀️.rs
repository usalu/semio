use super::*;

#[test]
fn artifact_kind_schema_matches_the_document_schema() {
    assert_eq!(artifact_kind().schema, GENERATION_3D_SCHEMA);
}

#[test]
fn dialect_artifact_kind_matches_the_schema_capability_descriptor() {
    assert_eq!(GENERATION3D_DIALECT.artifact_kind, "s.procedural.generation3d");
    assert_eq!(GENERATION3D_DIALECT.standard, StandardId("1"));
    assert_eq!(GENERATION3D_DIALECT.subset, SubsetId::ANY);
}

#[test]
fn widget_id_covers_all_widget_kinds() {
    let widgets: Vec<Widget> = vec![
        Widget::Neuron { id: "neuron-1".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: true },
        Widget::InputSlider { id: "slider-1".into(), label: "Number".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 },
        Widget::InputNote { id: "note-1".into(), text: String::new() },
        Widget::InputImage { id: "image-1".into(), src: String::new() },
        Widget::Variable { id: "variable-1".into(), name: "x".into(), schema: "number".into() },
        Widget::OutputPreview { id: "preview-1".into(), preview: Default::default(), expanded: Default::default() },
        Widget::OutputAction { id: "action-1".into(), action: "run".into() },
        Widget::OutputExport { id: "export-1".into(), format: "gltf".into() },
        Widget::Cluster { id: "cluster-1".into(), name: "c".into(), tree: Default::default(), flow: Default::default() },
    ];
    let ids: Vec<&str> = widgets.iter().map(widget_id).collect();
    assert_eq!(ids, vec!["neuron-1", "slider-1", "note-1", "image-1", "variable-1", "preview-1", "action-1", "export-1", "cluster-1"]);
}
