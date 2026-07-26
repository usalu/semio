//! 🔗 WIRES mindmap: Owns, Is, References, Has on a flexible identity-kind set.

pub use infinite_board_normal_undirected as graph;
pub use infinite_cavas as cavas;
pub use reasoning_mindmap as mindmap;

//#region ⚠️ Errors
/// 🧯 WIRES extension errors — fixture (de)serialization and fixed-identity-set validation failures.
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

// #region 🔖RelationshipKind
/// 🔗 One of the four WIRES relationship kinds.
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

    /// 🔢 All relationship kinds in declaration order.
    pub const ALL: [Self; 4] = [Self::Owns, Self::Is, Self::References, Self::Has];
}
// #endregion 🔖RelationshipKind

// #region 🔖WiresExtension
/// 🔗 WIRES semantics over a mindmap (normal undirected graph).
pub trait WiresExtension: mindmap::MindmapExtension {
    fn relationship_kind_label(&self, relationship_id: graph::EdgeId) -> Option<&str>;
    fn validate_identity_set(&self, identities: &[mindmap::TopicId]) -> Result<(), WiresError>;
}

/// 🧭 Default WIRES extension with fixed identity vocabulary and relationship kinds.
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

impl DefaultWiresExtension {
    /// 🔗 Hydrate extension state from `reasoning.wires.fixture` JSON.
    pub fn from_fixture_json(json: &str) -> Result<Self, WiresError> {
        let root: serde_json::Value = serde_json::from_str(json)?;
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
            let Some(identity_id) = row.get("identityId").and_then(|v| v.as_u64()) else {
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
            let Some(relationship_id) = row.get("relationshipId").and_then(|v| v.as_u64()) else {
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
// #endregion 🔖WiresExtension

// #region 🔖Tests
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
        // 📜 The `.wires` fixture is handcrafted in mindmap's DSL (see `reasoning_mindmap::mindmap_text`
        // via {@link vcs::DocumentDsl}) — parse it, then hydrate this crate's JSON-facing extension from
        // its `wires_fixture` value, the same shape `from_fixture_json` has always expected.
        let text = include_str!("../example/metabolism.wires");
        let document = <mindmap::MindmapWiresDocument as vcs::DocumentDsl>::parse_dsl(text).expect("metabolism fixture dsl");
        let ext = DefaultWiresExtension::from_fixture_json(&document.wires_fixture.to_string()).expect("metabolism fixture");
        assert_eq!(ext.mindmap.topics.len(), 7);
        assert_eq!(ext.relationships.len(), 9);
        assert_eq!(ext.relationship_kind_label(8), Some("is"));
        assert!(ext.validate_identity_set(&[1, 2, 3]).is_ok());
    }
}
// #endregion 🔖Tests
