//! 📜️ Block 3D app — textual document grammar surface + laws (constitutional: dsl).

use block_3d::Block3dDefinition;

/// 📄️ The `nakagin-capsule` example fixture, handcrafted in the `.block3d` DSL — the `ObjectKind` half
/// of compose's metabolism-kit `Capsule` type.
pub const BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/🧱️block/🎛️app/🧊️3d/⚡️implementation/🦀️rust/📚️example/🧱️nakagin-capsule.block3d");
/// 📄️ The `hexagonal-cut-concrete-forest-left` example fixture, handcrafted in the `.block3d` DSL.
pub const BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/🧱️block/🎛️app/🧊️3d/⚡️implementation/🦀️rust/📚️example/🧱️hexagonal-cut-concrete-forest-left.block3d");

/// 📖️ Parses `.block3d` DSL text into a `Block3dDefinition`.
pub fn parse_dsl(text: &str) -> Result<Block3dDefinition, store::TextError> {
    <Block3dDefinition as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Block3dDefinition` back to `.block3d` DSL text.
pub fn print_dsl(document: &Block3dDefinition) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use block_3d::{Block3dVortexKind, Block3dVortexTemplate};
    use block_shared::{BlockCamera3d, BlockKindIdentity, BlockRepresentation};

    pub fn nakagin_capsule() -> Block3dDefinition {
        let mut definition = Block3dDefinition {
            object_kind: BlockKindIdentity { id: "Capsule J".into(), name: "Capsule J".into(), label: "Capsule J".into(), ..Default::default() },
            camera3d: BlockCamera3d { position: [10.0, -10.0, 6.0], target: [0.0, 0.0, 1.0], zoom: 1.0 },
            ..Block3dDefinition::default()
        };
        definition.representations.push(BlockRepresentation { id: "r0".into(), name: "Full Detail".into(), mesh_url: Some("/mesh/🧊️capsule_J.glb".into()), tags: vec!["full".into()], lod: Some("full".into()), description: String::new(), attributes: Vec::new() });
        definition.representations.push(BlockRepresentation { id: "r1".into(), name: "1:500".into(), mesh_url: Some("/mesh/capsule_J.1to500.glb".into()), tags: vec!["1to500".into()], lod: Some("low".into()), description: String::new(), attributes: Vec::new() });
        definition.vortex_kinds.push(Block3dVortexKind { id: "door".into(), name: "door".into(), label: "Door".into(), color: "hsl(206 52% 48%)".into(), default_cable_kind: "cable.link".into() });
        definition.vortices.push(Block3dVortexTemplate { id: "v0".into(), vortex_kind: "door".into(), position: [0.0, -1.6, 1.2], direction: [0.0, -1.0, 0.0], radius: 0.3, label: Some("door".into()) });
        definition
    }

    #[test]
    fn block3d_definition_dsl_round_trips() {
        let empty = Block3dDefinition::default();
        store::test_support::assert_dsl_round_trip(&empty);
        store::test_support::assert_dsl_pack_equivalence(&empty);
        let definition = nakagin_capsule();
        store::test_support::assert_dsl_round_trip(&definition);
        store::test_support::assert_dsl_pack_equivalence(&definition);
    }

    #[test]
    fn block3d_example_fixtures_parse_and_round_trip_as_dsl() {
        for dsl_text in [BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT, BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT] {
            let definition = parse_dsl(dsl_text).expect("example fixture parses as dsl");
            store::test_support::assert_dsl_round_trip(&definition);
        }
    }
}
//#endregion 🧪️Tests
