//! ⚙️ Wires artifact — headless compute over the `MindmapWiresDocument` projection (constitutional:
//! engine). Everything here is pure over `crate::artifacts::wires` types; the rule for what lands here
//! rather than next to a single caller: a helper with MORE THAN ONE consumer across the taxonomy tree
//! lives here (or takes only generic `DslValue`/document-shaped data — never an app-only view-state
//! type like `crate::apps::wires::config::WiresConfig`); a helper with exactly one consumer, or one that
//! reads an app-only type, lives in that consumer's own component file instead.

use crate::artifacts::wires::MindmapWiresDocument;
use dsl::DslValue;
use serde_json::Value;

//#region 🔖️Register
/// 🗂️ Registers `MindmapWiresDocument`'s pack↔dsl codec so `framework/sync`'s `FolderEndpoint::Pack`
/// (and any other schema-string-keyed caller) can print/parse it without depending on this crate's
/// concrete `Projection`/`Mutation` types. Called from the plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::wires::ReasoningWiresPlayApp>(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "wires.document",
        extension: Some("wires"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::wires::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::wires::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::wires::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::wires::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("wires.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "wires.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::wires::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::wires::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::wires::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::wires::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("wires.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "wires.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::wires::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::wires::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("wires.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "wires.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::wires::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::wires::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("wires.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "wires.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::wires::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::wires::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("wires.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️DocumentHelpers
pub fn array_mut<'a>(fixture: &'a mut DslValue, key: &str) -> &'a mut Vec<DslValue> {
    if !matches!(fixture, DslValue::Object(_)) {
        *fixture = DslValue::Object(vec![]);
    }
    let DslValue::Object(entries) = fixture else {
        unreachable!("fixture coerced to object above");
    };
    if let Some(idx) = entries.iter().position(|(entry_key, _)| entry_key == key) {
        let value = &mut entries[idx].1;
        if !matches!(value, DslValue::Array(_)) {
            *value = DslValue::Array(vec![]);
        }
        match value {
            DslValue::Array(items) => items,
            _ => unreachable!("array coerced above"),
        }
    } else {
        entries.push((key.to_string(), DslValue::Array(vec![])));
        match &mut entries.last_mut().expect("just pushed").1 {
            DslValue::Array(items) => items,
            _ => unreachable!("just pushed array"),
        }
    }
}

pub fn entity_id<'a>(entity: &'a DslValue, key: &str) -> Option<&'a str> {
    entity.get(key).and_then(|value| value.as_str())
}

/// 🔢️ `identityId`/`sourceIdentityId`/`targetIdentityId` (and similar numeric-id fields) read as a
/// whole `u64` regardless of whether the source JSON number is an integer or a float literal. Fixtures
/// round-tripped through the `.wires` DSL text arrive as exact JSON integers (`Number(1)`, see
/// `IdentityDsl`/`RelationshipDsl`'s plain `u64` fields), so this fallback stays for documents built or
/// patched outside that DSL path (e.g. hand-constructed `Value` fixtures), where nothing enforces the
/// integer representation.
pub fn dsl_id(value: Option<&DslValue>) -> Option<u64> {
    value.and_then(|value| value.as_f64().map(|float| float as u64))
}

pub fn dsl_to_json(value: &DslValue) -> Value {
    dsl::from_dsl_value(value.clone()).unwrap_or(Value::Null)
}

pub fn fixture_json_string(fixture: &DslValue) -> String {
    serde_json::to_string(&dsl_to_json(fixture)).unwrap_or_else(|_| "{}".into())
}

pub fn fixture_camera(fixture: &DslValue) -> (f64, f64, f64) {
    let camera = fixture.get("camera");
    (
        camera.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()).unwrap_or(1.0),
    )
}

pub fn fixture_nodes(fixture: &DslValue) -> &[DslValue] {
    fixture.get("nodes").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn fixture_edges(fixture: &DslValue) -> &[DslValue] {
    fixture.get("edges").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn wires_identities(wires: &DslValue) -> &[DslValue] {
    wires.get("identities").and_then(|value| value.as_array()).unwrap_or(&[])
}

pub fn wires_relationships(wires: &DslValue) -> &[DslValue] {
    wires.get("relationships").and_then(|value| value.as_array()).unwrap_or(&[])
}

/// 📐️ A JSON node's position, defaulting missing coordinates to the origin.
pub fn node_position(node: &DslValue) -> (f64, f64) {
    (node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0), node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0))
}

pub fn find_board_node<'a>(document: &'a MindmapWiresDocument, node_id: &str) -> Option<&'a DslValue> {
    document.board_fixture.get("nodes").and_then(|value| value.as_array()).into_iter().flatten().find(|node| entity_id(node, "id") == Some(node_id))
}

pub fn find_board_edge<'a>(document: &'a MindmapWiresDocument, edge_id: &str) -> Option<&'a DslValue> {
    document.board_fixture.get("edges").and_then(|value| value.as_array()).into_iter().flatten().find(|edge| entity_id(edge, "id") == Some(edge_id))
}

pub fn find_relationship<'a>(document: &'a MindmapWiresDocument, edge_id: &str) -> Option<&'a DslValue> {
    document.wires_fixture.get("relationships").and_then(|value| value.as_array()).into_iter().flatten().find(|relationship| entity_id(relationship, "edgeId") == Some(edge_id))
}

/// 🕸️ Re-lays out the board with the neutral `infinite_board_port_directed` force-graph solver — the
/// same shared mechanism `puzzle/2d`'s `forceLayout`/`reorganize` uses, depended on directly rather
/// than through puzzle's app program (mindmap's board schema is on its allowlist).
pub fn force_layout_board(board: &mut DslValue) {
    let Ok(layout_json) = infinite_board_port_directed::apply_force_graph_layout_to_fixture_v1_json(&fixture_json_string(board), r#"{"mode":"force-graph"}"#) else {
        return;
    };
    if let Ok(parsed) = serde_json::from_str::<Value>(&layout_json) {
        *board = dsl::to_dsl_value(&parsed).unwrap_or(DslValue::Null);
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️ExampleFixture
/// 📄️ The `metabolism` example, parsed once from `crate::artifacts::wires::dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT`
/// — falls back to the empty document if the fixture ever fails to parse.
pub fn metabolism_wires_example_document() -> MindmapWiresDocument {
    <MindmapWiresDocument as store::DocumentDsl>::parse_dsl(crate::artifacts::wires::dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT).unwrap_or_else(|_| crate::artifacts::wires::empty_mindmap_wires_document())
}
//#endregion 🔖️ExampleFixture

//#region 🔖️WiresExtension
pub use infinite_board_normal_undirected as graph;
pub use infinite_canvas as canvas;
pub use semio_s_mindmap as mindmap;

//#region ⚠️ Errors
/// 🧯️ WIRES extension errors — fixture (de)serialization and fixed-identity-set validation failures.
#[derive(Debug, thiserror::Error)]
pub enum WiresError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("fixture root must be object")]
    FixtureRootNotObject,
    #[error("schema must be reasoning.wires.fixture")]
    SchemaMismatch,
    #[error("identities array missing")]
    IdentitiesMissing,
    #[error("relationships array missing")]
    RelationshipsMissing,
    #[error("identity {0} is not in the fixed WIRES identity set")]
    IdentityNotAllowed(mindmap::TopicId),
}
//#endregion ⚠️ Errors

// #region 🔖️RelationshipKind
/// 🔗️ One of the four WIRES relationship kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RelationshipKind {
    Owns,
    Is,
    References,
    Has,
}

impl RelationshipKind {
    /// 🏷️ Stable relationship slug for fixtures and UI.
    pub fn label(self) -> &'static str {
        match self {
            Self::Owns => "owns",
            Self::Is => "is",
            Self::References => "references",
            Self::Has => "has",
        }
    }

    /// 🔢️ All relationship kinds in declaration order.
    pub const ALL: [Self; 4] = [Self::Owns, Self::Is, Self::References, Self::Has];
}
// #endregion 🔖️RelationshipKind

// #region 🔖️WiresExtensionTrait
/// 🔗️ WIRES semantics over a mindmap (normal undirected graph).
pub trait WiresExtension: mindmap::MindmapExtension {
    fn relationship_kind_label(&self, relationship_id: graph::EdgeId) -> Option<&str>;
    fn validate_identity_set(&self, identities: &[mindmap::TopicId]) -> Result<(), WiresError>;
}

/// 🧭️ Default WIRES extension with fixed identity vocabulary and relationship kinds.
#[derive(Clone, Debug, Default)]
pub struct DefaultWiresExtension {
    pub mindmap: mindmap::DefaultMindmapExtension,
    pub relationships: std::collections::BTreeMap<graph::EdgeId, RelationshipKind>,
    pub allowed_identities: std::collections::BTreeSet<mindmap::TopicId>,
}

impl canvas::CanvasExtension for DefaultWiresExtension {
    fn extension_id(&self) -> &str {
        "reasoning.mindmap/wires"
    }
}

impl graph::GraphExtension for DefaultWiresExtension {}

impl mindmap::MindmapExtension for DefaultWiresExtension {
    fn topic_label(&self, node_id: mindmap::TopicId) -> Option<&str> {
        self.mindmap.topic_label(node_id)
    }
}

impl DefaultWiresExtension {
    /// 🔗️ Hydrate extension state from `reasoning.wires.fixture` JSON.
    pub fn from_fixture_json(json: &str) -> Result<Self, WiresError> {
        let root: Value = serde_json::from_str(json)?;
        let Some(obj) = root.as_object() else {
            return Err(WiresError::FixtureRootNotObject);
        };
        if obj.get("schema").and_then(|v| v.as_str()) != Some("reasoning.wires.fixture") {
            return Err(WiresError::SchemaMismatch);
        }
        let mut ext = Self::default();
        let Some(identities) = obj.get("identities").and_then(|v| v.as_array()) else {
            return Err(WiresError::IdentitiesMissing);
        };
        for identity in identities {
            let Some(row) = identity.as_object() else {
                continue;
            };
            let Some(identity_id) = row.get("identityId").and_then(|v| v.as_u64().or_else(|| v.as_f64().map(|float| float as u64))) else {
                continue;
            };
            let label = row.get("label").and_then(|v| v.as_str()).unwrap_or("").to_string();
            ext.mindmap.topics.insert(identity_id, label);
            ext.allowed_identities.insert(identity_id);
        }
        let Some(relationships) = obj.get("relationships").and_then(|v| v.as_array()) else {
            return Err(WiresError::RelationshipsMissing);
        };
        for rel in relationships {
            let Some(row) = rel.as_object() else {
                continue;
            };
            let Some(relationship_id) = row.get("relationshipId").and_then(|v| v.as_u64().or_else(|| v.as_f64().map(|float| float as u64))) else {
                continue;
            };
            let kind = match row.get("kind").and_then(|v| v.as_str()) {
                Some("owns") => RelationshipKind::Owns,
                Some("is") => RelationshipKind::Is,
                Some("references") => RelationshipKind::References,
                Some("has") => RelationshipKind::Has,
                _ => continue,
            };
            ext.relationships.insert(relationship_id, kind);
        }
        Ok(ext)
    }
}

impl WiresExtension for DefaultWiresExtension {
    fn relationship_kind_label(&self, relationship_id: graph::EdgeId) -> Option<&str> {
        self.relationships.get(&relationship_id).map(|r| r.label())
    }

    fn validate_identity_set(&self, identities: &[mindmap::TopicId]) -> Result<(), WiresError> {
        if self.allowed_identities.is_empty() {
            return Ok(());
        }
        for id in identities {
            if !self.allowed_identities.contains(id) {
                return Err(WiresError::IdentityNotAllowed(*id));
            }
        }
        Ok(())
    }
}
// #endregion 🔖️WiresExtensionTrait
//#endregion 🔖️WiresExtension

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relationship_kind_labels() {
        assert_eq!(RelationshipKind::Owns.label(), "owns");
        assert_eq!(RelationshipKind::Has.label(), "has");
    }

    #[test]
    fn fixed_identity_set_validation() {
        let mut ext = DefaultWiresExtension::default();
        ext.allowed_identities.insert(1);
        ext.allowed_identities.insert(2);
        assert!(ext.validate_identity_set(&[1, 2]).is_ok());
        assert!(ext.validate_identity_set(&[1, 3]).is_err());
    }

    #[test]
    fn relationship_lookup() {
        let mut ext = DefaultWiresExtension::default();
        ext.relationships.insert(7, RelationshipKind::References);
        assert_eq!(ext.relationship_kind_label(7), Some("references"));
    }

    #[test]
    fn metabolism_fixture_hydrates_extension() {
        // 📜️ The `.wires` fixture is handcrafted in `crate::artifacts::wires::dsl`'s DSL — parse it,
        // then hydrate this crate's JSON-facing extension from its `wires_fixture` value, the same
        // shape `from_fixture_json` has always expected.
        let document = metabolism_wires_example_document();
        let json = serde_json::to_string(&dsl_to_json(&document.wires_fixture)).expect("json");
        let ext = DefaultWiresExtension::from_fixture_json(&json).expect("metabolism fixture");
        assert_eq!(ext.mindmap.topics.len(), 7);
        assert_eq!(ext.relationships.len(), 9);
        assert_eq!(ext.relationship_kind_label(8), Some("is"));
        assert!(ext.validate_identity_set(&[1, 2, 3]).is_ok());
    }
}
//#endregion 🧪️Tests


//#region 🔖️ArtifactEngine
pub struct MindmapWiresEngine {
    projection: crate::artifacts::wires::MindmapWiresDocument,
}

impl MindmapWiresEngine {
    pub fn new(projection: crate::artifacts::wires::MindmapWiresDocument) -> Self {
        Self { projection }
    }
}

impl protocol::ArtifactEngine for MindmapWiresEngine {
    type Projection = crate::artifacts::wires::MindmapWiresDocument;
    type Mutation = crate::artifacts::wires::mutations::MindmapWiresMutation;
    type Diff = crate::artifacts::wires::diff::MindmapWiresDiff;

    fn projection(&self) -> &Self::Projection {
        &self.projection
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        crate::artifacts::wires::mutations::apply_mindmap_wires_mutation(&mut self.projection, mutation);
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }
}
//#endregion 🔖️ArtifactEngine
