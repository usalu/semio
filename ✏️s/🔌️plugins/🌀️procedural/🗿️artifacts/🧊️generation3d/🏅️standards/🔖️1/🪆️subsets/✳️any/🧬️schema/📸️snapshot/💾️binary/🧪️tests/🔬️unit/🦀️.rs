
use super::*;
use crate::dsl as generation3d_dsl;
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_os_kernel::os_store::test_support;

#[test]
fn dsl_pack_equivalence_empty_projection() {
    test_support::assert_dsl_pack_equivalence(&Generation3dSnapshot::default());
}

#[test]
fn dsl_pack_equivalence_example_fixture() {
    let projection = generation3d_dsl::parse_dsl(generation3d_dsl::GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT).expect("parse 🌀️default.generation3d fixture");
    test_support::assert_dsl_pack_equivalence(&projection);
}

#[test]
fn dsl_pack_equivalence_with_generation_state() {
    let mut projection = Generation3dSnapshot::default();
    let mut values: semio_framework_artifact_playbook_playbook::PlaybookValues = std::collections::HashMap::new();
    // 🌱️ Fractional (not whole-number) so `dsl::from_dsl_value`'s int-normalization of whole
    // `DslValue::Number`s (an engine-owned behavior, see the sibling dsl test) doesn't make this
    // round trip spuriously unequal.
    values.insert("count".into(), dsl::DslValue::float(3.5));
    projection.generation = semio_framework_artifact_playbook_playbook::GenerationPlayState {
        generations: vec![semio_framework_artifact_playbook_playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values }],
        selected_generation_id: Some("generation-1".into()),
        preview_text: Some("42".into()),
    }
    .into();
    test_support::assert_dsl_pack_equivalence(&projection);
}

#[test]
fn dsl_pack_equivalence_covers_every_widget_kind() {
    let mut projection = Generation3dSnapshot::default();
    projection.fixture.widgets = vec![
        Widget::InputSlider { id: "slider".into(), label: "Number".into(), value: 2.0, min: 0.0, max: 10.0, step: 0.5 },
        Widget::InputImage { id: "image".into(), src: "data:image/png;base64,abc".into() },
        Widget::Variable { id: "variable".into(), name: "value".into(), schema: "dictionary".into() },
        Widget::OutputAction { id: "action".into(), action: "export".into() },
        Widget::OutputExport { id: "export".into(), format: "svg".into() },
        Widget::Cluster { id: "cluster".into(), name: "Group".into(), tree: Default::default(), flow: Default::default() },
    ];
    projection.fixture.synapses = vec![];
    test_support::assert_dsl_pack_equivalence(&projection);
}

#[test]
fn pack_round_trips() {
    let projection = generation3d_dsl::parse_dsl(generation3d_dsl::GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT).expect("parse fixture");
    let bytes = encode(&projection);
    assert!(bytes.starts_with(b"P3D3"));
    assert_eq!(decode(&bytes).expect("decode"), projection);
    let mut wrong = bytes;
    wrong[..4].copy_from_slice(b"P2D2");
    assert!(decode(&wrong).is_err());
}
