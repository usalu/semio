//! 📜 Block 5D app — textual document grammar surface + laws (constitutional: dsl).

use block_5d::Block5dDefinition;

/// 📄 The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block5d` DSL —
/// the `PartKind` slice of `s/plugin/puzzle/app/5d/example/concrete-forest.puzzle5d`.
pub const BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/🧱block/🎛️app/🖐️5d/⚡️implementation/🦀rust/📚example/hexagonal-cut-concrete-forest-left.block5d");
/// 📄 The `nakagin-capsule` example fixture, handcrafted in the `.block5d` DSL.
pub const BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌plugin/🧱block/🎛️app/🖐️5d/⚡️implementation/🦀rust/📚example/nakagin-capsule.block5d");

/// 📖 Parses `.block5d` DSL text into a `Block5dDefinition`.
pub fn parse_dsl(text: &str) -> Result<Block5dDefinition, store::TextError> {
    <Block5dDefinition as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block5dDefinition` back to `.block5d` DSL text.
pub fn print_dsl(document: &Block5dDefinition) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use block_5d::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d};
    use block_shared::{BlockCamera2d, BlockCamera3d, BlockKindIdentity, BlockRepresentation};

    pub fn hexagonal_cut_concrete_forest_left() -> Block5dDefinition {
        let mut definition = Block5dDefinition {
            part_kind: BlockKindIdentity { id: "Hexagonal Cut Concrete Forest Left".into(), name: "Hexagonal Cut Concrete Forest Left".into(), label: "Hexagonal Cut Concrete Forest Left".into(), ..Default::default() },
            part_2d: Block5dPart2d { shape: Some("circle".into()), radius: Some(20.0), ..Default::default() },
            part_3d: Block5dPart3d { orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None },
            camera2d: BlockCamera2d { x: 230.7, y: 93.5, zoom: 2.0 },
            camera3d: BlockCamera3d { position: [30.0, -30.0, 20.0], target: [7.0, 0.0, 3.0], zoom: 3.0 },
            ..Block5dDefinition::default()
        };
        definition.representations.push(BlockRepresentation { id: "r0".into(), name: "default".into(), mesh_url: Some("/mesh/hexagonal-cut-concrete-forest-left.glb".into()), tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() });
        definition.grip_kinds.push(Block5dGripKind { id: "b-l".into(), name: "b-l".into(), label: "b-l".into(), color: "hsl(206 52% 48%)".into(), default_rope_kind: "rope.link".into() });
        definition.grips.push(Block5dGripTemplate { id: "g0".into(), grip_kind: "b-l".into(), angle: -0.1, radius_2d: 3.0, position: [4.05, 4.68, 3.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 });
        definition
    }

    #[test]
    fn block5d_definition_dsl_round_trips() {
        let empty = Block5dDefinition::default();
        store::test_support::assert_dsl_round_trip(&empty);
        store::test_support::assert_dsl_pack_equivalence(&empty);
        let definition = hexagonal_cut_concrete_forest_left();
        store::test_support::assert_dsl_round_trip(&definition);
        store::test_support::assert_dsl_pack_equivalence(&definition);
    }

    #[test]
    fn block5d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT, BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT] {
            let definition = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            store::test_support::assert_dsl_round_trip(&definition);
        }
    }
}
//#endregion 🧪Tests
