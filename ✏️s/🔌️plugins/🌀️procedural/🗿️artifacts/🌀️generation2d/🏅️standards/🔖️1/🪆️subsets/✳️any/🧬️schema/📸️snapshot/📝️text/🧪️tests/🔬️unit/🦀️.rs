use super::*;
use crate::GENERATION_2D_SCHEMA;
use semio_framework_os_kernel::os_store::test_support;
use store::ArtifactDsl;

//#region 🔖️DslTests
#[test]
fn dsl_round_trip_empty_projection() {
    test_support::assert_dsl_round_trip(&Generation2dSnapshot::default());
    test_support::assert_dsl_pack_equivalence(&Generation2dSnapshot::default());
}

#[test]
fn dsl_round_trip_example_fixture() {
    let projection = Generation2dSnapshot::parse_dsl(GENERATION2D_EXAMPLE_TEXT).expect("parse 🌀️default.generation2d fixture");
    test_support::assert_dsl_round_trip(&projection);
    test_support::assert_dsl_pack_equivalence(&projection);
}

#[test]
fn dsl_round_trip_with_generation_state() {
    let mut projection = Generation2dSnapshot::default();
    let mut values = semio_framework_artifact_playbook_playbook::PlaybookValues::new();
    // 🌱️ A fractional literal, not a whole number: a whole-number float still normalizes to an
    // integer-backed `serde_json::Number` somewhere on this round trip — a real, engine-owned
    // behavior, not a bug in this crate's mirror/conversion code — so a whole-number input like
    // `3.0` would legitimately compare unequal to its round-tripped `3` here. `3.5` has no such
    // ambiguity.
    values.insert("count".into(), dsl::DslValue::float(3.5));
    projection.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values });
    projection.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = Some("generation-1".into());
    projection.generation.cold_builder_mut().expect("unique cold generation owner").preview_text = Some("42".into());
    test_support::assert_dsl_round_trip(&projection);
    test_support::assert_dsl_pack_equivalence(&projection);
}

#[test]
fn dsl_round_trip_covers_every_widget_kind() {
    let mut projection = Generation2dSnapshot::default();
    projection.fixture.widgets = vec![
        Widget::InputSlider { id: "slider".into(), label: "Number".into(), value: 2.0, min: 0.0, max: 10.0, step: 0.5 },
        Widget::InputImage { id: "image".into(), src: "data:image/png;base64,abc".into() },
        Widget::Variable { id: "variable".into(), name: "value".into(), schema: "dictionary".into() },
        Widget::OutputAction { id: "action".into(), action: "export".into() },
        Widget::OutputExport { id: "export".into(), format: "svg".into() },
        Widget::Cluster { id: "cluster".into(), name: "Group".into(), tree: Default::default(), flow: Default::default() },
    ];
    projection.fixture.synapses = vec![];
    test_support::assert_dsl_round_trip(&projection);
    test_support::assert_dsl_pack_equivalence(&projection);
}
//#endregion 🔖️DslTests

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law: proves `Generation2dMutation`'s `Edit` round-trips through
/// `protocol::MutationEnvelope`s beside this file's existing dsl/pack round-trip laws.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

    let mut store: ArtifactStore<Generation2dSnapshot, Generation2dMutation> = ArtifactStore::new(create_document_envelope(GENERATION_2D_SCHEMA, "generation2d", Generation2dSnapshot::default(), None)).await.expect("valid artifact store fixture");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::text::replace_widget(Widget::InputNote { id: "note-9".into(), text: String::new() })], description: None }).await.expect("apply");
    let edit: &Edit<Generation2dMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    test_support::assert_command_envelope_round_trip::<Generation2dSnapshot, Generation2dMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests

//#region 🔖️DslErrorTests
#[test]
fn dsl_parse_rejects_malformed_text() {
    let error = Generation2dSnapshot::parse_dsl("schema=\"flow.fixture").unwrap_err();
    assert!(error.message.contains("unterminated string literal"), "unexpected error: {}", error.message);
}

#[test]
fn dsl_parse_rejects_missing_required_field() {
    let text = "camera { x=0 y=0 zoom=1 }\nwidgets { }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
    let error = Generation2dSnapshot::parse_dsl(text).unwrap_err();
    assert!(error.message.contains("found Absent"), "unexpected error: {}", error.message);
}

#[test]
fn dsl_parse_rejects_missing_camera_block() {
    let error = Generation2dSnapshot::parse_dsl("schema=\"flow.fixture\"\n").unwrap_err();
    assert!(error.message.contains("expected Record, found Absent"), "unexpected error: {}", error.message);
}

#[test]
fn dsl_parse_rejects_unquoted_value_for_string_field() {
    let text = "schema=123\ncamera { x=0 y=0 zoom=1 }\nwidgets { }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
    let error = Generation2dSnapshot::parse_dsl(text).unwrap_err();
    assert!(error.message.contains("expected Text"), "unexpected error: {}", error.message);
}

#[test]
fn dsl_parse_rejects_non_numeric_value_for_number_field() {
    let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { input-slider id=\"s\" value=abc min=0 max=1 step=1 }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
    let error = Generation2dSnapshot::parse_dsl(text).unwrap_err();
    assert!(error.message.contains("expected a float"), "unexpected error: {}", error.message);
}

#[test]
fn dsl_parse_rejects_invalid_bool_value() {
    let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { neuron id=\"n\" neuron-kind=math.add preview=maybe input-ports= [ ] output-ports= [ ] params= [ ] }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
    let error = Generation2dSnapshot::parse_dsl(text).unwrap_err();
    assert!(error.message.contains("expected 'true' or 'false'"), "unexpected error: {}", error.message);
}

#[test]
fn dsl_parse_rejects_malformed_value_literal() {
    let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { cluster id=\"n\" name=\"n\" tree=bogusvalue flow= [ ] }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
    let error = Generation2dSnapshot::parse_dsl(text).unwrap_err();
    assert!(error.message.contains("expected a value literal"), "unexpected error: {}", error.message);
}

#[test]
fn dsl_parse_rejects_unknown_widget_kind() {
    let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { bogus id=\"n\" }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
    let error = Generation2dSnapshot::parse_dsl(text).unwrap_err();
    assert!(error.message.contains("expected RBrace"), "unexpected error: {}", error.message);
}
//#endregion 🔖️DslErrorTests
