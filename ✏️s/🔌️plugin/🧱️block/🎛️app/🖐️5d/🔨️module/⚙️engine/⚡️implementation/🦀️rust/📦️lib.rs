//! 👯️ Block 5D app — headless compute (constitutional: engine).

use block_5d::{Block5dDefinition, BLOCK_5D_SCHEMA};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🔖️DocumentHelpers
pub fn empty_block5d_definition() -> Block5dDefinition {
    Block5dDefinition::default()
}

/// 🪪️ Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
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
//#endregion 🔖️DocumentHelpers

//#region 🔖️PuzzleCatalogFragment
/// 🌉️ Maps this `PartKind` definition into the `s/plugin/puzzle` 5d catalog shape
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
    let grip_kinds: Vec<Value> = definition.grip_kinds.iter().map(|kind| json!({ "id": kind.id, "name": kind.name, "label": kind.label, "color": kind.color, "defaultRopeKind": kind.default_rope_kind })).collect();
    json!({
        "schema": "manifest",
        "parts": [part],
        "grips": grip_kinds,
        "fasteners": Vec::<Value>::new(),
        "ropes": Vec::<Value>::new(),
        "kindCompatibility": definition.compatibility.iter().map(|rule| json!({ "source": rule.source, "target": rule.target, "bidirectional": rule.bidirectional })).collect::<Vec<_>>(),
    })
}
//#endregion 🔖️PuzzleCatalogFragment

//#region 🔖️Config
/// 🧮️ `Block5dPlayApp`'s real `DocumentApp::Config` — B1 pure-trait conversion. Absorbs the former
/// `Block5dPlayApp::selected_ids` `RefCell` field plus the locale this app now resolves itself
/// (mirrors `shooting_engine::ShootingConfig::locale`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "block5dcfg")]
#[dsl(layout = "lines")]
pub struct Block5dConfig {
    /// 👁️ Multi-selected row ids in the document tree — was `Block5dPlayApp::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewState.locale`.
    pub locale: String,
}

impl Default for Block5dConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), locale: "en-US".into() }
    }
}

impl store::ConfigRecord for Block5dConfig {}

/// @emoji 🧮️ Whole-record diff for `block_5d_op::Block5dConfigOperation` — lives here (not in
/// `block_5d_op`) since `protocol::OperationDiff`/`Block5dConfig` are both foreign to that crate (the
/// orphan rule needs one local type); mirrors `shooting_engine`'s identical pattern.
impl protocol::OperationDiff<Block5dConfig> for Block5dConfig {
    fn apply(&self, _base: &Block5dConfig) -> Block5dConfig {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ `Block5dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"5d.block"` artifact kind) plus a `"catalog:out"` port giving
/// `puzzle5d_catalog_fragment` a real caller (see `🖱️ui`'s `Block5dPlayApp::export_media`).
pub fn block5d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        BLOCK_5D_SCHEMA,
        semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        semio_framework_plugin::ArtifactPresentation { id: "5d.block".into(), name: "Part Kind".into(), dimension: "5d".into(), component_kind: "block5d".into() },
    )
    .with_ports(vec![semio_framework_plugin::MediaPortSpec {
        id: "catalog:out".into(),
        label: "Kit Catalog".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        kind_id: Some("kit.catalog".into()),
        required: false,
        multiplicity: semio_framework_plugin::PortMultiplicity::Many,
    }])
}
//#endregion 🔖️Io

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use block_5d::Block5dGripTemplate;
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

    #[test]
    fn block5d_config_default_has_no_selection() {
        let config = Block5dConfig::default();
        assert!(config.selected_ids.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn block5d_io_declares_the_catalog_out_port() {
        let io = block5d_io();
        assert_eq!(io.document_schema, BLOCK_5D_SCHEMA);
        let ports = io.all_ports();
        let catalog = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog.direction, semio_framework_plugin::MediaPortDirection::Out);
    }
}
//#endregion 🧪️Tests
