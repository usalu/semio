//! 📜 Procedural 2D app — textual document grammar surface + laws (constitutional: dsl).

use procedural_2d::Procedural2dDocument;

/// 📦 The `procedural2d-play` "default" example, embedded at compile time as handcrafted `.procedural2d`
/// DSL text — shared by the manifest's `.example(...)` registration, the `default_projection` fallback,
/// and every test fixture.
pub const PROCEDURAL2D_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/🌀procedural/🎛️app/◻️2d/⚡️implementation/🦀rust/📚example/default.procedural2d");

/// 📖 Parses `.procedural2d` DSL text into a `Procedural2dDocument`.
pub fn parse_dsl(text: &str) -> Result<Procedural2dDocument, store::TextError> {
    <Procedural2dDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Procedural2dDocument` back to `.procedural2d` DSL text.
pub fn print_dsl(document: &Procedural2dDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_core::Widget;
    use store::{test_support, DocumentDsl};

    //#region 🔖DslTests
    #[test]
    fn dsl_round_trip_empty_projection() {
        test_support::assert_dsl_round_trip(&Procedural2dDocument::default());
        test_support::assert_dsl_pack_equivalence(&Procedural2dDocument::default());
    }

    #[test]
    fn dsl_round_trip_example_fixture() {
        let projection = Procedural2dDocument::parse_dsl(PROCEDURAL2D_EXAMPLE_TEXT).expect("parse default.procedural2d fixture");
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_round_trip_with_generation_state() {
        let mut projection = Procedural2dDocument::default();
        let mut values = serde_json::Map::new();
        // 🌱 A float literal, not `json!(3)` (an integer-backed `serde_json::Number`): the DSL
        // engine's `Shape::Value`/`DslValue::Number` is a single `f64` variant (see `dsl/rs/lib.rs`'s
        // own documented int-vs-float caveat), so a value round tripping through generation `values`
        // always comes back float-backed — this is the known, accepted engine limitation, not a bug
        // in this crate's mirror/conversion code.
        values.insert("count".into(), serde_json::json!(3.0));
        projection.generation.generations.push(playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values });
        projection.generation.selected_generation_id = Some("generation-1".into());
        projection.generation.preview_text = Some("42".into());
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_round_trip_covers_every_widget_kind() {
        let mut projection = Procedural2dDocument::default();
        projection.fixture.widgets = vec![
            Widget::InputSlider { id: "slider".into(), value: 2.0, min: 0.0, max: 10.0, step: 0.5 },
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
    //#endregion 🔖DslTests

    //#region 🔖DslErrorTests
    /// 📜 The derive-engine grammar (see `procedural_2d`'s `🔖DslMirror`) has no leading
    /// `document`/`widget`/`synapse` keyword and no document-level "trailing content is rejected"
    /// check (a `RecordSpec`'s own `parse` simply stops once every field is read — see
    /// `dsl_schema::parse`, out of this crate's ownership scope) — so error assertions below target
    /// the engine's OWN real error text for the equivalent malformed-input shape, not the OLD
    /// hand-rolled grammar's wording.
    #[test]
    fn dsl_parse_rejects_malformed_text() {
        let error = Procedural2dDocument::parse_dsl("schema=\"flow.fixture").unwrap_err();
        assert!(error.message.contains("unterminated string literal"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_missing_required_field() {
        let text = "camera { x=0 y=0 zoom=1 }\nwidgets { }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("found Absent"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_missing_camera_block() {
        let error = Procedural2dDocument::parse_dsl("schema=\"flow.fixture\"\n").unwrap_err();
        assert!(error.message.contains("expected Record, found Absent"), "unexpected error: {}", error.message);
    }

    /// 🌱 A bare (unquoted) value is now legitimately accepted for any `Shape::Text` field (the
    /// unified syntax law's "bare-preferred" strings) — this asserts the genuinely-still-rejected
    /// shape mismatch instead: a raw number token, which is neither `Ident` nor `Text`.
    #[test]
    fn dsl_parse_rejects_unquoted_value_for_string_field() {
        let text = "schema=123\ncamera { x=0 y=0 zoom=1 }\nwidgets { }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected Text"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_non_numeric_value_for_number_field() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { input-slider id=\"s\" value=abc min=0 max=1 step=1 }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected a float"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_invalid_bool_value() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { neuron id=\"n\" neuron-kind=math.add preview=maybe input-ports= [ ] output-ports= [ ] params= [ ] }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected 'true' or 'false'"), "unexpected error: {}", error.message);
    }

    /// 🧬 `Widget::Cluster`'s `tree`/`flow` fields are the only remaining genuinely free-form value
    /// literal (bound via the engine's `Shape::Value`, see `procedural_2d`'s `🔖DslMirror`) —
    /// `params`/`preview` moved to typed `DictEntryDsl` records, so a malformed *value literal* (not
    /// JSON text) is now only reachable through `tree`/`flow`.
    #[test]
    fn dsl_parse_rejects_malformed_value_literal() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { cluster id=\"n\" name=\"n\" tree=bogusvalue flow= [ ] }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected a value literal"), "unexpected error: {}", error.message);
    }

    /// 🏷️ An unrecognized widget kind keyword is simply left unconsumed by `Shape::Statements`
    /// (the engine breaks its variant-matching loop rather than erroring — see `dsl_schema::parse`,
    /// out of this crate's ownership scope), so parsing ultimately fails at the enclosing `widgets
    /// { }` block's closing brace instead of with a dedicated "unknown widget kind" message.
    #[test]
    fn dsl_parse_rejects_unknown_widget_kind() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { bogus id=\"n\" }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected RBrace"), "unexpected error: {}", error.message);
    }
    //#endregion 🔖DslErrorTests
}
//#endregion 🧪Tests
