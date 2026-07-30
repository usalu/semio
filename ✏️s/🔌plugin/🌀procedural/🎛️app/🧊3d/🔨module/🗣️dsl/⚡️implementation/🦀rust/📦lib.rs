//! 📜 Procedural 3D app — textual document grammar surface + laws (constitutional: dsl).

use procedural_3d::Procedural3dDocument;

/// 📦 The `procedural3d-play` "hexagonal mushroom column" example, embedded at compile time as
/// handcrafted `.procedural3d` DSL text — shared by the manifest's `.example(...)` registration, the
/// `default_projection`/`example_projection` fallbacks, and every test fixture.
pub const PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/🌀procedural/🎛️app/🧊3d/⚡️implementation/🦀rust/📚example/🌀hexagonal-mushroom-column.procedural3d");
/// 📦 The `procedural3d-play` "rectangle extrude volume" example.
pub const PROCEDURAL3D_EXAMPLE_RECT_EXTRUDE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/🌀procedural/🎛️app/🧊3d/⚡️implementation/🦀rust/📚example/🌀rectangle-extrude-volume.procedural3d");
/// 📦 The `procedural3d-play` "sphere cut with torus" example.
pub const PROCEDURAL3D_EXAMPLE_SPHERE_TORUS_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/🌀procedural/🎛️app/🧊3d/⚡️implementation/🦀rust/📚example/🌀sphere-cut-with-torus.procedural3d");

/// 📖 Parses `.procedural3d` DSL text into a `Procedural3dDocument`.
pub fn parse_dsl(text: &str) -> Result<Procedural3dDocument, store::TextError> {
    <Procedural3dDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Procedural3dDocument` back to `.procedural3d` DSL text.
pub fn print_dsl(document: &Procedural3dDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_core::Widget;
    use store::{test_support, DocumentDsl};

    #[test]
    fn dsl_round_trip_empty_projection() {
        test_support::assert_dsl_round_trip(&Procedural3dDocument::default());
        test_support::assert_dsl_pack_equivalence(&Procedural3dDocument::default());
    }

    #[test]
    fn dsl_round_trip_hexagonal_mushroom_column_fixture() {
        let projection = Procedural3dDocument::parse_dsl(PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT).expect("parse 🌀hexagonal-mushroom-column.procedural3d fixture");
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_round_trip_rectangle_extrude_volume_fixture() {
        let projection = Procedural3dDocument::parse_dsl(PROCEDURAL3D_EXAMPLE_RECT_EXTRUDE_TEXT).expect("parse 🌀rectangle-extrude-volume.procedural3d fixture");
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_round_trip_sphere_cut_with_torus_fixture() {
        let projection = Procedural3dDocument::parse_dsl(PROCEDURAL3D_EXAMPLE_SPHERE_TORUS_TEXT).expect("parse 🌀sphere-cut-with-torus.procedural3d fixture");
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_round_trip_with_generation_state() {
        let mut projection = Procedural3dDocument::default();
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
    fn dsl_round_trip_covers_remaining_widget_kinds() {
        let mut projection = Procedural3dDocument::default();
        projection.fixture.widgets = vec![
            Widget::InputNote { id: "note-1".into(), text: "hello \"world\"".into() },
            Widget::InputImage { id: "image-1".into(), src: "https://example.test/a.png".into() },
            Widget::Variable { id: "variable-1".into(), name: "height".into(), schema: "number".into() },
            Widget::OutputAction { id: "action-1".into(), action: "export".into() },
            Widget::OutputExport { id: "export-1".into(), format: "gltf".into() },
            Widget::Cluster { id: "cluster-1".into(), name: "Cluster".into(), tree: Default::default(), flow: Default::default() },
        ];
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    //#region 🔖ParseErrorTests
    /// 🏷️ An unrecognized widget kind keyword is simply left unconsumed by `Shape::Statements`
    /// (the engine breaks its variant-matching loop rather than erroring — see `dsl_schema::parse`,
    /// out of this crate's ownership scope), so parsing ultimately fails at the enclosing `widgets
    /// { }` block's closing brace instead of with a dedicated "unknown widget kind" message.
    #[test]
    fn parse_dsl_rejects_unknown_widget_kind() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { bogus id=\"w-1\" }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural3dDocument::parse_dsl(text).expect_err("unknown widget kind must fail to parse");
        assert!(error.to_string().contains("expected RBrace"), "unexpected error: {error}");
    }
    //#endregion 🔖ParseErrorTests
}
//#endregion 🧪Tests
