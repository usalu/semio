//! 👯 Block 5D app — headless compute (constitutional: engine).

use block_5d::Block5dDefinition;
use serde_json::{json, Value};

//#region 🔖DocumentHelpers
pub fn empty_block5d_definition() -> Block5dDefinition {
    Block5dDefinition::default()
}

/// 🪪 Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
pub fn next_id<'a>(existing: impl Iterator<Item = &'a str>, prefix: &str) -> String {
    let ids: std::collections::HashSet<&str> = existing.collect();
    let mut i = ids.len();
    loop {
        let candidate = format!("{prefix}{i}");
        if !ids.iter().any(|id| *id == candidate) {
            return candidate;
        }
        i += 1;
    }
}
//#endregion 🔖DocumentHelpers

//#region 🔖PuzzleCatalogFragment
/// 🌉 Maps this `PartKind` definition into the `s/plugin/puzzle` 5d catalog shape
/// (`Puzzle5dKindCatalogs`: `parts`/`grips`/`fasteners`/`ropes`), the seam puzzle imports through its
/// `Kit×Type` media port. Block owns no fastener/rope-kind rows, so those arrays stay empty here.
pub fn puzzle5d_catalog_fragment(definition: &Block5dDefinition) -> Value {
    let grips: Vec<Value> = definition
        .grips
        .iter()
        .map(|grip| {
            json!({
                "gripKind": grip.grip_kind,
                "2d": { "angle": grip.angle, "gripKind": grip.grip_kind, "radius": grip.radius_2d },
                "3d": { "position": grip.position, "direction": grip.direction, "radius": grip.radius_3d },
            })
        })
        .collect();
    let mesh_url = definition.representations.first().and_then(|representation| representation.mesh_url.clone());
    let part = json!({
        "id": definition.part_kind.id,
        "name": definition.part_kind.name,
        "label": definition.part_kind.label,
        "meshUrl": mesh_url,
        "grips": grips,
    });
    let grip_kinds: Vec<Value> = definition
        .grip_kinds
        .iter()
        .map(|kind| json!({ "id": kind.id, "name": kind.name, "label": kind.label, "color": kind.color, "defaultRopeKind": kind.default_rope_kind }))
        .collect();
    json!({
        "schema": "manifest",
        "parts": [part],
        "grips": grip_kinds,
        "fasteners": Vec::<Value>::new(),
        "ropes": Vec::<Value>::new(),
        "kindCompatibility": definition.compatibility.iter().map(|rule| json!({ "source": rule.source, "target": rule.target, "bidirectional": rule.bidirectional })).collect::<Vec<_>>(),
    })
}
//#endregion 🔖PuzzleCatalogFragment

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use block_5d::{Block5dGripTemplate, BLOCK_5D_SCHEMA};
    use block_shared::BlockKindIdentity;

    #[test]
    fn empty_definition_matches_default() {
        assert_eq!(empty_block5d_definition(), Block5dDefinition::default());
    }

    #[test]
    fn puzzle5d_catalog_fragment_maps_grips() {
        let mut definition = Block5dDefinition { schema: BLOCK_5D_SCHEMA.into(), part_kind: BlockKindIdentity { id: "left".into(), name: "left".into(), label: "Left".into(), ..Default::default() }, ..Block5dDefinition::default() };
        definition.grips.push(Block5dGripTemplate { id: "g0".into(), grip_kind: "b-l".into(), angle: -1.57, radius_2d: 0.36, position: [4.05, 4.68, 3.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 });
        let fragment = puzzle5d_catalog_fragment(&definition);
        assert_eq!(fragment["parts"][0]["id"], "left");
        assert_eq!(fragment["parts"][0]["grips"][0]["gripKind"], "b-l");
    }
}
//#endregion 🧪Tests
