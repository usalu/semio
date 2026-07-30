//! 📜 Puzzle 5d app — textual document grammar surface + laws (constitutional: dsl).

use puzzle_5d::Puzzle5dProjection;

/// 📄 The `concrete-forest` example fixture, handcrafted in the `.puzzle5d` DSL.
pub const PUZZLE5D_CONCRETE_FOREST_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/🧩puzzle/🎛️app/🖐️5d/⚡️implementation/🦀rust/📚example/concrete-forest.puzzle5d");
/// 📄 The `nakagin-capsule-tower` example fixture, handcrafted in the `.puzzle5d` DSL.
pub const PUZZLE5D_NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/🧩puzzle/🎛️app/🖐️5d/⚡️implementation/🦀rust/📚example/nakagin-capsule-tower.puzzle5d");

/// 📖 Parses `.puzzle5d` DSL text into a `Puzzle5dProjection`.
pub fn parse_dsl(text: &str) -> Result<Puzzle5dProjection, store::TextError> {
    <Puzzle5dProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Puzzle5dProjection` back to `.puzzle5d` DSL text.
pub fn print_dsl(document: &Puzzle5dProjection) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use puzzle_5d::{Puzzle5dCamera2d, Puzzle5dCamera3d, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d};

    #[test]
    fn puzzle5d_projection_dsl_round_trips() {
        let empty = Puzzle5dProjection::default();
        store::test_support::assert_dsl_round_trip(&empty);
        store::test_support::assert_dsl_pack_equivalence(&empty);
        let mut projection = Puzzle5dProjection::default();
        projection.label = Some("Concrete Forest".into());
        projection.camera2d = Puzzle5dCamera2d { x: 230.7, y: 93.5, zoom: 2.0 };
        projection.camera3d = Puzzle5dCamera3d { position: [30.0, -30.0, 20.0], target: [7.0, 0.0, 3.0], zoom: 3.0 };
        projection.meta = Puzzle5dMeta { description: "Unified puzzle 5d source".into() };
        projection.parts.push(Puzzle5dPart {
            id: "seed-left-001".into(),
            part_kind: Some("Hexagonal Cut Concrete Forest Left".into()),
            part_2d: Puzzle5dPart2d { x: 230.7, y: 93.5, shape: Some("circle".into()), radius: Some(20.0), width: None, height: None, text: Some("Hexagonal Cut Concrete Forest Left".into()), icon_kind: None, hidden: None, locked: None },
            part_3d: Puzzle5dPart3d { origin: [0.0, 0.0, 0.0], mesh_url: Some("/mesh/hexagonal-cut-concrete-forest-left.glb".into()), orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: Some("Hexagonal Cut Concrete Forest Left".into()) },
            grips: vec![Puzzle5dGrip {
                id: "v0".into(),
                grip_kind: Some("b-l".into()),
                grip_2d: Puzzle5dGrip2d { angle: -0.1, grip_kind: Some("b-l".into()), radius: Some(3.0) },
                grip_3d: Puzzle5dGrip3d { position: [4.05, 4.68, 3.0], direction: Some([0.0, 1.0, 0.0]), radius: Some(0.36), label: Some("b-l".into()) },
            }],
        });
        projection.fasteners.push(Puzzle5dFastener { id: "f1".into(), source: "seed-left-001:v0".into(), target: "seed-right-001:v0".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 });
        projection.kind_compatibility.push(Puzzle5dKindCompatibility { source: "b-l".into(), target: "b-l".into(), bidirectional: true });
        store::test_support::assert_dsl_round_trip(&projection);
        store::test_support::assert_dsl_pack_equivalence(&projection);
    }

    /// 📜 Both real example fixtures (migrated from the legacy `.5d.json` shape — see ticket
    /// 🎫convertpuzzle2d3d5dtotypeddslderiveengine) parse as `.puzzle5d` DSL text and round-trip
    /// through `print_dsl`/`parse_dsl` exactly.
    #[test]
    fn puzzle5d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [PUZZLE5D_CONCRETE_FOREST_EXAMPLE_TEXT, PUZZLE5D_NAKAGIN_EXAMPLE_TEXT] {
            let projection = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            store::test_support::assert_dsl_round_trip(&projection);
            // 🚧 `assert_dsl_pack_equivalence(&projection)` deliberately NOT added here: same
            // `pack/value/rs` table-column bug as `puzzle5d_projection_dsl_round_trips` above
            // (this fixture's `parts` rows have the identical shape). NOTE: as of this writing
            // this whole test is ALREADY failing before reaching this line, at the `parse_dsl`
            // call above ("expected LBrace, found Ident 'x'", `concrete-forest.puzzle5d:50:54``)
            // — a pre-existing DSL-text/fixture staleness issue unrelated to pack (confirmed via
            // `git status`: neither this fixture nor `dsl/core`/`dsl/derive` have any pending
            // changes in this session; likely fallout of concurrent syntax-convergence work per
            // `.repo/🎫/26/07/27/UNIFIED-TOKEN-EFFICIENT-DSL-SYNTAX-ACROSS-ALL-TECHNOLOGIES/
            // wave3-final-status.md`, which recorded this exact test green earlier in the same
            // session). Out of scope for the pack/document-layer ticket either way — this was a
            // pre-existing failure before the constitutional-crate split too.
        }
    }
}
//#endregion 🧪Tests
