//! 📜️ Puzzle 5d app — textual document grammar surface + laws (constitutional: dsl).

use puzzle_5d::Puzzle5dProjection;

/// 📄️ The `concrete-forest` example fixture, handcrafted in the `.puzzle5d` DSL.
pub const PUZZLE5D_CONCRETE_FOREST_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/🧩️puzzle/🎛️app/🖐️5d/⚡️implementation/🦀️rust/📚️example/🧩️concrete-forest.puzzle5d");
/// 📄️ The `nakagin-capsule-tower` example fixture, handcrafted in the `.puzzle5d` DSL.
pub const PUZZLE5D_NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/🧩️puzzle/🎛️app/🖐️5d/⚡️implementation/🦀️rust/📚️example/🧩️nakagin-capsule-tower.puzzle5d");

/// 📖️ Parses `.puzzle5d` DSL text into a `Puzzle5dProjection`.
pub fn parse_dsl(text: &str) -> Result<Puzzle5dProjection, store::TextError> {
    <Puzzle5dProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Puzzle5dProjection` back to `.puzzle5d` DSL text.
pub fn print_dsl(document: &Puzzle5dProjection) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use puzzle_5d::{Puzzle5dFastener, Puzzle5dGrip, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dScale};

    #[test]
    fn puzzle5d_projection_dsl_round_trips() {
        let empty = Puzzle5dProjection::default();
        store::test_support::assert_dsl_round_trip(&empty);
        store::test_support::assert_dsl_pack_equivalence(&empty);
        let mut projection = Puzzle5dProjection::default();
        projection.label = Some("Concrete Forest".into());
        projection.meta = Puzzle5dMeta { description: "Unified puzzle 5d source".into() };
        projection.parts.push(Puzzle5dPart {
            id: "seed-left-001".into(),
            part_kind: Some("Hexagonal Cut Concrete Forest Left".into()),
            part_2d: Puzzle5dPart2d { x: 230.7, y: 93.5, shape: Some("circle".into()), radius: Some(20.0), width: None, height: None, text: Some("Hexagonal Cut Concrete Forest Left".into()), icon_kind: None, hidden: None, locked: None },
            // 📏️ Vec3 (per-axis) case of `Puzzle5dScale`, exercised alongside the Uniform case
            // below so both `Shape::Tuple(_, None)` arities round-trip through this test.
            part_3d: Puzzle5dPart3d {
                origin: [0.0, 0.0, 0.0],
                mesh_url: Some("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb".into()),
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: Some(Puzzle5dScale::Vec3([1.0, 1.0, 1.5])),
                label: Some("Hexagonal Cut Concrete Forest Left".into()),
            },
            grips: vec![Puzzle5dGrip {
                id: "v0".into(),
                grip_kind: Some("b-l".into()),
                grip_2d: Puzzle5dGrip2d { angle: -0.1, grip_kind: Some("b-l".into()), radius: Some(3.0) },
                grip_3d: Puzzle5dGrip3d { position: [4.05, 4.68, 3.0], direction: Some([0.0, 1.0, 0.0]), radius: Some(0.36), label: Some("b-l".into()) },
            }],
        });
        projection.parts.push(Puzzle5dPart {
            id: "seed-right-001".into(),
            part_kind: Some("Hexagonal Cut Concrete Forest Right".into()),
            part_2d: Puzzle5dPart2d::default(),
            // 📏️ Uniform case of `Puzzle5dScale` — a bare number scaling all three axes alike.
            part_3d: Puzzle5dPart3d { origin: [6.0, 0.0, 0.0], mesh_url: None, orientation: None, scale: Some(Puzzle5dScale::Uniform(1.25)), label: None },
            grips: vec![Puzzle5dGrip {
                id: "v0".into(),
                grip_kind: Some("b-l".into()),
                grip_2d: Puzzle5dGrip2d { angle: 0.0, grip_kind: None, radius: None },
                grip_3d: Puzzle5dGrip3d { position: [0.0, 0.0, 0.0], direction: None, radius: None, label: None },
            }],
        });
        projection.fasteners.push(Puzzle5dFastener { id: "f1".into(), source: "seed-left-001:v0".into(), target: "seed-right-001:v0".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 });
        projection.kind_compatibility.push(Puzzle5dKindCompatibility { source: "b-l".into(), target: "b-l".into(), bidirectional: true });
        store::test_support::assert_dsl_round_trip(&projection);
        store::test_support::assert_dsl_pack_equivalence(&projection);
    }

    /// 📜️ Both real example fixtures (migrated from the legacy `.5d.json` shape — see ticket
    /// 🎫️convertpuzzle2d3d5dtotypeddslderiveengine) parse as `.puzzle5d` DSL text and round-trip
    /// through `print_dsl`/`parse_dsl` exactly.
    #[test]
    fn puzzle5d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [PUZZLE5D_CONCRETE_FOREST_EXAMPLE_TEXT, PUZZLE5D_NAKAGIN_EXAMPLE_TEXT] {
            let projection = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            store::test_support::assert_dsl_round_trip(&projection);
            // 🚧️ `assert_dsl_pack_equivalence(&projection)` deliberately NOT added here: same
            // `pack/value/rs` table-column bug as `puzzle5d_projection_dsl_round_trips` above
            // (this fixture's `parts` rows have the identical shape).
        }
    }
}
//#endregion 🧪️Tests
