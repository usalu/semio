//! 🔍️ Wires play app panel — the inspector: a document-wide summary (was field editors for the
//! current selection; see `render`'s doc comment for why that's gone).

use crate::artifacts::wires::schema::{fixture_json_string, fixture_nodes};
use crate::artifacts::wires::{WiresSnapshot, MINDMAP_WIRES_SCHEMA};
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use serde_json::Value;

//#region 🔖️Constants
pub const WIRES_PLAY_BODY_PROPERTIES: &str = "reasoning.wires.properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(WIRES_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to switch on
/// `config.selected_ids` to show one selected node's fields (id, identity label, kind, position).
/// Selection is now framework-owned (`InteractionView`, threaded only into `handle`/`copy_fragment`/
/// `cut_operations`) and `ArtifactApp::render` never gained that parameter, so this panel has no live
/// selection to render against and always falls through to the document summary below — the same gap
/// layout's/gis2d's/puzzle3d's inspection panels flag (see this ticket's w3b-summary.md). Not fixed
/// here (framework file, out of this crate's remit).
pub fn render(document: &WiresSnapshot) -> UiNode {
    let board = crate::artifacts::wires::wires_working_board(document);
    let extension = DefaultWiresExtension::from_fixture_json(&fixture_json_string(&document.wires_fixture)).ok();
    ui_stack_vertical(vec![
        ui_text(Label::data(format!("Schema: {MINDMAP_WIRES_SCHEMA}"))),
        ui_text(Label::data(format!("Identities: {}", extension.as_ref().map_or(0, |ext| ext.mindmap.topics.len())))),
        ui_text(Label::data(format!("Relationships: {}", extension.as_ref().map_or(0, |ext| ext.relationships.len())))),
        ui_text(Label::data(format!("Board nodes: {}", fixture_nodes(&board).len()))),
    ])
}
//#endregion 🔖️Render

//#region 🔖️WiresExtension
/// 🧠️ Dissolved from the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — `DefaultWiresExtension` has exactly one consumer, `render` above (via `from_fixture_json`), so it
/// lives here rather than in the artifact's `🧬️schema` (the single- vs multi-consumer split the former
/// engine file's own module doc already drew: a helper with more than one consumer lives in the
/// artifact, one with exactly one consumer lives in that consumer's own file).
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

impl mindmap::GraphExtension for DefaultWiresExtension {}

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
    use crate::apps::wires::testkit::{metabolism_app, render as render_body};

    #[test]
    fn empty_selection_shows_document_summary() {
        let mut app = metabolism_app();
        let json = render_body(&mut app, WIRES_PLAY_BODY_PROPERTIES);
        assert!(json.contains("Schema:"));
        assert!(json.contains("Board nodes:"));
    }

    #[test]
    fn definition_binds_the_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(WIRES_PLAY_BODY_PROPERTIES));
    }

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
        let document = crate::artifacts::wires::schema::metabolism_wires_example_snapshot();
        let json = serde_json::to_string(&crate::artifacts::wires::schema::dsl_to_json(&document.wires_fixture)).expect("json");
        let ext = DefaultWiresExtension::from_fixture_json(&json).expect("metabolism fixture");
        assert_eq!(ext.mindmap.topics.len(), 7);
        assert_eq!(ext.relationships.len(), 9);
        assert_eq!(ext.relationship_kind_label(8), Some("is"));
        assert!(ext.validate_identity_set(&[1, 2, 3]).is_ok());
    }
}
//#endregion 🧪️Tests
