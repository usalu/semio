//! 🩻 Block 2D app — headless compute (constitutional: engine).

use block_2d::Block2dDefinition;
use serde_json::{json, Value};

//#region 🔖DocumentHelpers
pub fn empty_block2d_definition() -> Block2dDefinition {
    Block2dDefinition::default()
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
/// 🌉 Maps this `NodeKind` definition into the `s/plugin/puzzle` 2d manifest shape (`portKinds`/
/// `wireKinds`/`edgeKinds`/`nodeKinds`/`kindCompatibility` — see
/// `s/plugin/puzzle/app/2d/manifest/🛂manifest.jsonconcrete-forest.manifest.json`), the seam puzzle imports through
/// its `Kit×Type` media port. Block owns no wire/edge-kind rows (`AGENTS.md`: referenced by
/// `default_wire_kind` only), so those arrays stay empty here — a merge keeps the puzzle manifest's
/// existing rows.
pub fn puzzle2d_manifest_fragment(definition: &Block2dDefinition) -> Value {
    let port_kinds: Vec<Value> = definition
        .handle_kinds
        .iter()
        .map(|kind| json!({ "id": kind.id, "name": kind.name, "presentation": { "color": kind.color, "defaultWireKind": kind.default_wire_kind } }))
        .collect();
    let handles: Vec<Value> = definition.handles.iter().map(|handle| json!({ "handleKind": handle.handle_kind, "angle": handle.angle, "radius": handle.radius })).collect();
    let node_kind = json!({
        "id": definition.node_kind.id,
        "name": definition.node_kind.name,
        "presentation": {
            "meshUrl": Value::Null,
            "handles": handles,
        },
    });
    let kind_compatibility: Vec<Value> = definition
        .compatibility
        .iter()
        .map(|rule| json!({ "bidirectional": rule.bidirectional, "specificity": "handle", "source": rule.source, "target": rule.target }))
        .collect();
    json!({
        "schema": "manifest",
        "id": definition.node_kind.id,
        "name": definition.node_kind.name,
        "axes": { "portModel": "ported", "directedness": "directed" },
        "portKinds": port_kinds,
        "wireKinds": Vec::<Value>::new(),
        "edgeKinds": Vec::<Value>::new(),
        "nodeKinds": [node_kind],
        "kindCompatibility": kind_compatibility,
    })
}
//#endregion 🔖PuzzleCatalogFragment

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use block_2d::{Block2dHandleKind, Block2dHandleTemplate, BLOCK_2D_SCHEMA};
    use block_shared::BlockKindIdentity;

    #[test]
    fn empty_definition_matches_default() {
        assert_eq!(empty_block2d_definition(), Block2dDefinition::default());
    }

    #[test]
    fn next_id_skips_existing() {
        let existing = ["h0", "h1"];
        assert_eq!(next_id(existing.into_iter(), "h"), "h2");
        assert_eq!(next_id(std::iter::empty(), "h"), "h0");
    }

    #[test]
    fn puzzle2d_manifest_fragment_maps_kind_identity_and_handles() {
        let mut definition = Block2dDefinition { schema: BLOCK_2D_SCHEMA.into(), node_kind: BlockKindIdentity { id: "left".into(), name: "left".into(), label: "Left".into(), ..Default::default() }, ..Block2dDefinition::default() };
        definition.handle_kinds.push(Block2dHandleKind { id: "b-l".into(), name: "b-l".into(), label: "b-l".into(), color: "hsl(206 52% 48%)".into(), default_wire_kind: "cable.link".into() });
        definition.handles.push(Block2dHandleTemplate { id: "h0".into(), handle_kind: "b-l".into(), angle: -1.57, radius: 0.36 });
        let fragment = puzzle2d_manifest_fragment(&definition);
        assert_eq!(fragment["nodeKinds"][0]["id"], "left");
        assert_eq!(fragment["nodeKinds"][0]["presentation"]["handles"][0]["handleKind"], "b-l");
        assert_eq!(fragment["portKinds"][0]["id"], "b-l");
    }
}
//#endregion 🧪Tests
