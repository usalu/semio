//! ⚙️ Reasoning wires app — headless compute (constitutional: engine).

use reasoning_wires::MindmapWiresDocument;
use serde_json::Value;

//#region 🔖️DocumentHelpers
pub fn empty_board_fixture() -> Value {
    serde_json::json!({
        "schema": reasoning_wires::MINDMAP_BOARD_SCHEMA,
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [],
        "edges": [],
        "wires": []
    })
}

pub fn empty_wires_fixture() -> Value {
    serde_json::json!({
        "schema": reasoning_wires::MINDMAP_WIRES_SCHEMA,
        "identities": [],
        "relationships": [],
        "board": empty_board_fixture()
    })
}

pub fn empty_mindmap_wires_document() -> MindmapWiresDocument {
    MindmapWiresDocument { wires_fixture: empty_wires_fixture(), board_fixture: empty_board_fixture() }
}

pub fn array_mut<'a>(fixture: &'a mut Value, key: &str) -> &'a mut Vec<Value> {
    let object = fixture.as_object_mut().expect("mindmap fixture must be a JSON object");
    object.entry(key.to_string()).or_insert_with(|| Value::Array(Vec::new()));
    object
        .get_mut(key)
        .and_then(|value| {
            if !value.is_array() {
                *value = Value::Array(Vec::new());
            }
            value.as_array_mut()
        })
        .expect("array coerced above")
}

pub fn entity_id<'a>(entity: &'a Value, key: &str) -> Option<&'a str> {
    entity.get(key).and_then(|value| value.as_str())
}

pub fn find_board_node<'a>(document: &'a MindmapWiresDocument, node_id: &str) -> Option<&'a Value> {
    document
        .board_fixture
        .get("nodes")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .find(|node| entity_id(node, "id") == Some(node_id))
}

pub fn find_board_edge<'a>(document: &'a MindmapWiresDocument, edge_id: &str) -> Option<&'a Value> {
    document
        .board_fixture
        .get("edges")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .find(|edge| entity_id(edge, "id") == Some(edge_id))
}

pub fn find_relationship<'a>(document: &'a MindmapWiresDocument, edge_id: &str) -> Option<&'a Value> {
    document
        .wires_fixture
        .get("relationships")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .find(|relationship| entity_id(relationship, "edgeId") == Some(edge_id))
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️ExampleFixture
/// 📄️ The `metabolism` example, handcrafted in the `.wires` DSL (see
/// `reasoning_wires_dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT`) instead of JSON — source of truth
/// for every "metabolism" example call site (`setActiveExample`, `.example` manifest registration, tests).
const METABOLISM_WIRES_EXAMPLE_TEXT: &str = reasoning_wires_dsl::REASONING_WIRES_EXAMPLE_METABOLISM_TEXT;

/// 📄️ The `metabolism` example, parsed once from {@link METABOLISM_WIRES_EXAMPLE_TEXT} — falls back to
/// the empty document if the fixture ever fails to parse.
pub fn metabolism_wires_example_document() -> MindmapWiresDocument {
    <MindmapWiresDocument as store::DocumentDsl>::parse_dsl(METABOLISM_WIRES_EXAMPLE_TEXT).unwrap_or_else(|_| empty_mindmap_wires_document())
}
//#endregion 🔖️ExampleFixture

//#region 🔖️WiresExtension
pub use infinite_board_normal_undirected as graph;
pub use infinite_cavas as cavas;
pub use reasoning_mindmap as mindmap;

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

impl cavas::CanvasExtension for DefaultWiresExtension {
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

/// 🔢️ `identityId`/`relationshipId` read as a whole `u64` regardless of whether the source JSON
/// number is an integer or a float literal. `MindmapWiresDocument`'s `wires_fixture`/`board_fixture`
/// are opaque `serde_json::Value` at rest (see `reasoning_wires::MindmapWiresDocument`'s doc), but
/// the `.wires` DSL's own `IdentityDsl::identity_id`/`RelationshipDsl::relationship_id` type these as
/// plain `u64` (see `reasoning_wires`'s `🔖️DslMirror` region), so ids round-tripped through the
/// `.wires` DSL text now arrive here as exact JSON integers (`Number(1)`); this fallback stays for
/// documents built or patched outside that DSL path (e.g. hand-constructed `Value` fixtures), where
/// nothing enforces the integer representation.
fn json_id(value: Option<&Value>) -> Option<mindmap::TopicId> {
    value.and_then(|value| value.as_u64().or_else(|| value.as_f64().map(|float| float as u64)))
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
            let Some(identity_id) = json_id(row.get("identityId")) else {
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
            let Some(relationship_id) = json_id(row.get("relationshipId")) else {
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
        // 📜️ The `.wires` fixture is handcrafted in `reasoning_wires_dsl`'s DSL — parse it, then
        // hydrate this crate's JSON-facing extension from its `wires_fixture` value, the same shape
        // `from_fixture_json` has always expected.
        let document = metabolism_wires_example_document();
        let ext = DefaultWiresExtension::from_fixture_json(&document.wires_fixture.to_string()).expect("metabolism fixture");
        assert_eq!(ext.mindmap.topics.len(), 7);
        assert_eq!(ext.relationships.len(), 9);
        assert_eq!(ext.relationship_kind_label(8), Some("is"));
        assert!(ext.validate_identity_set(&[1, 2, 3]).is_ok());
    }
}
//#endregion 🧪️Tests
