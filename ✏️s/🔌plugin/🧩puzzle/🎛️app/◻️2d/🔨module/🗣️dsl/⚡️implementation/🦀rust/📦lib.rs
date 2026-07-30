//! 📜 Puzzle 2d app — textual document grammar surface + laws (constitutional: dsl).

use puzzle_2d::Puzzle2dProjection;

/// 📄 The `concrete-forest` example fixture, handcrafted in the `.puzzle2d` DSL.
pub const PUZZLE2D_CONCRETE_FOREST_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/🧩puzzle/🎛️app/◻️2d/⚡️implementation/🦀rust/📚example/🧩concrete-forest.puzzle2d");
/// 📄 The `nakagin-capsule-tower` example fixture, handcrafted in the `.puzzle2d` DSL.
pub const PUZZLE2D_NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/🧩puzzle/🎛️app/◻️2d/⚡️implementation/🦀rust/📚example/🧩nakagin-capsule-tower.puzzle2d");

/// 📖 Parses `.puzzle2d` DSL text into a `Puzzle2dProjection`.
pub fn parse_dsl(text: &str) -> Result<Puzzle2dProjection, store::TextError> {
    <Puzzle2dProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Puzzle2dProjection` back to `.puzzle2d` DSL text.
pub fn print_dsl(document: &Puzzle2dProjection) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use puzzle_2d::{Puzzle2dCamera, Puzzle2dCompatSpecificity, Puzzle2dEdge, Puzzle2dHandle, Puzzle2dKindCompatibility, Puzzle2dMeta, Puzzle2dNode};

    /// 📜 Both real example fixtures (migrated from the legacy `.2d.json` shape — see ticket
    /// 🎫convertpuzzle2d3d5dtotypeddslderiveengine) parse as `.puzzle2d` DSL text and round-trip
    /// through `print_dsl`/`parse_dsl` exactly.
    #[test]
    fn puzzle2d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [PUZZLE2D_CONCRETE_FOREST_EXAMPLE_TEXT, PUZZLE2D_NAKAGIN_EXAMPLE_TEXT] {
            let projection = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            store::test_support::assert_dsl_round_trip(&projection);
            store::test_support::assert_dsl_pack_equivalence(&projection);
        }
    }

    #[test]
    fn puzzle2d_projection_dsl_round_trips() {
        let empty = Puzzle2dProjection::default();
        store::test_support::assert_dsl_round_trip(&empty);
        store::test_support::assert_dsl_pack_equivalence(&empty);
        let mut with_content = Puzzle2dProjection::default();
        with_content.camera = Puzzle2dCamera { x: 12.0, y: -4.0, zoom: 2.5 };
        with_content.nodes.push(Puzzle2dNode {
            id: "n1".into(),
            node_kind: Some("seed".into()),
            shape: Some("circle".into()),
            x: 1.0,
            y: 2.0,
            radius: Some(24.0),
            width: None,
            height: None,
            text: Some("Seed".into()),
            icon_kind: None,
            root: None,
            scale: None,
            visible: None,
            locked: None,
            handles: vec![Puzzle2dHandle { id: "n1:v0".into(), handle_kind: Some("b-l".into()), angle: 0.0, radius: Some(3.0), color: None, icon_kind: None, scale: None, visible: None, locked: None }],
        });
        with_content.nodes.push(Puzzle2dNode {
            id: "n2".into(),
            node_kind: Some("seed".into()),
            shape: Some("rectangle".into()),
            x: 30.0,
            y: 40.0,
            radius: None,
            width: Some(48.0),
            height: Some(24.0),
            text: None,
            icon_kind: Some("door".into()),
            root: Some(true),
            scale: Some(1.5),
            visible: Some(false),
            locked: Some(true),
            handles: Vec::new(),
        });
        with_content.edges.push(Puzzle2dEdge { id: "e1".into(), source: "n1:v0".into(), target: "n2".into(), edge_kind: Some("edge.link".into()), source_tip: None, target_tip: Some("arrow".into()), visible: None, locked: None });
        with_content.meta = Puzzle2dMeta {
            manifest_id: Some("concrete-forest".into()),
            kind_compatibility: vec![Puzzle2dKindCompatibility { bidirectional: true, specificity: Puzzle2dCompatSpecificity::Vortex, source: "b-l".into(), target: "b-l".into() }],
            kind_catalogs: None,
        };
        store::test_support::assert_dsl_round_trip(&with_content);
        store::test_support::assert_dsl_pack_equivalence(&with_content);
    }
}
//#endregion 🧪Tests
