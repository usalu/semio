use super::*;

#[test]
fn artifact_kind_schema_matches_the_document_schema() {
    assert_eq!(artifact_kind().schema, GENERATION_2D_SCHEMA);
}

#[test]
fn widget_id_covers_every_widget_kind() {
    let widgets = vec![
        Widget::Neuron { id: "w-neuron".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: true },
        Widget::InputSlider { id: "w-slider".into(), label: "Width".into(), value: 1.0, min: 0.0, max: 2.0, step: 0.5 },
        Widget::InputNote { id: "w-note".into(), text: String::new() },
        Widget::InputImage { id: "w-image".into(), src: String::new() },
        Widget::Variable { id: "w-variable".into(), name: "value".into(), schema: "dictionary".into() },
        Widget::OutputPreview { id: "w-preview".into(), preview: Default::default(), expanded: Default::default() },
        Widget::OutputAction { id: "w-action".into(), action: String::new() },
        Widget::OutputExport { id: "w-export".into(), format: "svg".into() },
        Widget::Cluster { id: "w-cluster".into(), name: String::new(), tree: Default::default(), flow: Default::default() },
    ];
    for widget in &widgets {
        assert!(!widget_id(widget).is_empty());
    }
}
