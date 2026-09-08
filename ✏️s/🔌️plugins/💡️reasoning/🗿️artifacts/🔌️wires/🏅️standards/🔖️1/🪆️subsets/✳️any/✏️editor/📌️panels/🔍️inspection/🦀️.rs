//! 🔍️ Wires play app panel — the inspector: a document-wide summary (was field editors for the
//! current selection; see `render`'s doc comment for why that's gone).

use crate::artifacts::wires::schema::{fixture_json_string, fixture_nodes};
use crate::artifacts::wires::{WiresSnapshot, MINDMAP_WIRES_SCHEMA};
use semio_framework_plugin::{BuiltNode, UiAssemblyResult, PanelTreeBuilder, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use dsl::os_pack::json::Value;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};

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
pub fn render(document: &WiresSnapshot, labels: &crate::editor::wires::terminology::WiresLabels) -> UiAssemblyResult<BuiltNode> {
    let board = crate::artifacts::wires::wires_working_board(document);
    let extension = DefaultWiresExtension::from_fixture_json(&fixture_json_string(&document.wires_fixture)).ok();
    let namespace = PanelTreeBuilder::new("wires-inspection")?;
    let rows = [
        format!("{}: {MINDMAP_WIRES_SCHEMA}", labels.schema.as_str()),
        format!("{}: {}", labels.identities.as_str(), extension.as_ref().map_or(0, |ext| ext.topics.len())),
        format!("{}: {}", labels.relationships.as_str(), extension.as_ref().map_or(0, |ext| ext.relationships.len())),
        format!("{}: {}", labels.board_nodes.as_str(), fixture_nodes(&board).len()),
    ];
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for (index, row) in rows.iter().enumerate() {
        let node = semio_framework_ui_contract::text(crate::editor::wires::ui_label(row)?).try_id(format!("wires-inspection.summary-{index}"))
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "wires summary id admission failed"))?.try_build()
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "wires summary text admission failed"))?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "wires summary admission failed"))?;
    }
    namespace.section("wires-inspection.summary", None, true, nodes)?.build()
}
//#endregion 🔖️Render

//#region 🔖️WiresExtension
/// 🧠️ Dissolved from the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — `DefaultWiresExtension` has exactly one consumer, `render` above (via `from_fixture_json`), so it
/// lives here rather than in the artifact's `🧬️schema` (the single- vs multi-consumer split the former
/// engine file's own module doc already drew: a helper with more than one consumer lives in the
/// artifact, one with exactly one consumer lives in that consumer's own file).
pub use infinite_canvas as graph;
pub use infinite_canvas as canvas;

pub type TopicId = canvas::board::NodeId;

//#region ⚠️ Errors
/// 🧯️ WIRES extension errors — fixture (de)serialization and fixed-identity-set validation failures.
#[derive(Debug)]
pub enum WiresError {
    Json(dsl::os_pack::json::JsonError),
    FixtureRootNotObject,
    SchemaMismatch,
    IdentitiesMissing,
    RelationshipsMissing,
    IdentityNotAllowed(TopicId),
}

impl std::fmt::Display for WiresError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::FixtureRootNotObject => formatter.write_str("fixture root must be object"),
            Self::SchemaMismatch => formatter.write_str("schema must be reasoning.wires.fixture"),
            Self::IdentitiesMissing => formatter.write_str("identities array missing"),
            Self::RelationshipsMissing => formatter.write_str("relationships array missing"),
            Self::IdentityNotAllowed(identity) => write!(formatter, "identity {identity} is not in the fixed WIRES identity set"),
        }
    }
}

impl std::error::Error for WiresError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => std::error::Error::source(error),
            _ => None,
        }
    }
}

impl From<dsl::os_pack::json::JsonError> for WiresError {
    fn from(error: dsl::os_pack::json::JsonError) -> Self {
        Self::Json(error)
    }
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
pub trait WiresExtension: canvas::board::GraphExtension {
    fn topic_label(&self, topic_id: TopicId) -> Option<&str>;
    fn relationship_kind_label(&self, relationship_id: graph::EdgeId) -> Option<&str>;
    fn validate_identity_set(&self, identities: &[TopicId]) -> Result<(), WiresError>;
}

/// 🧭️ Default WIRES extension with fixed identity vocabulary and relationship kinds.
#[derive(Clone, Debug, Default)]
pub struct DefaultWiresExtension {
    pub topics: std::collections::BTreeMap<TopicId, String>,
    pub relationships: std::collections::BTreeMap<graph::EdgeId, RelationshipKind>,
    pub allowed_identities: std::collections::BTreeSet<TopicId>,
}

impl canvas::CanvasExtension for DefaultWiresExtension {
    fn extension_id(&self) -> &str {
        "reasoning.mindmap/wires"
    }
}

impl graph::GraphExtension for DefaultWiresExtension {}

impl canvas::board::GraphExtension for DefaultWiresExtension {}

impl DefaultWiresExtension {
    /// 🔗️ Hydrate extension state from `reasoning.wires.fixture` JSON.
    pub fn from_fixture_json(json: &str) -> Result<Self, WiresError> {
        let root: Value = dsl::os_pack::json::parse(json)?;
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
            ext.topics.insert(identity_id, label);
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
    fn topic_label(&self, topic_id: TopicId) -> Option<&str> {
        self.topics.get(&topic_id).map(String::as_str)
    }

    fn relationship_kind_label(&self, relationship_id: graph::EdgeId) -> Option<&str> {
        self.relationships.get(&relationship_id).map(|r| r.label())
    }

    fn validate_identity_set(&self, identities: &[TopicId]) -> Result<(), WiresError> {
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
    use crate::editor::wires::testkit::{metabolism_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn empty_selection_shows_document_summary() {
        let mut app = metabolism_app().await;
        let json = render_body(&mut app, WIRES_PLAY_BODY_PROPERTIES).await;
        assert!(json.contains("Schema:"));
        assert!(json.contains("Board nodes:"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(WIRES_PLAY_BODY_PROPERTIES));
    }

    #[semio_framework_async_macros::async_test]
    async fn relationship_kind_labels() {
        assert_eq!(RelationshipKind::Owns.label(), "owns");
        assert_eq!(RelationshipKind::Has.label(), "has");
    }

    #[semio_framework_async_macros::async_test]
    async fn fixed_identity_set_validation() {
        let mut ext = DefaultWiresExtension::default();
        ext.allowed_identities.insert(1);
        ext.allowed_identities.insert(2);
        assert!(ext.validate_identity_set(&[1, 2]).is_ok());
        assert!(ext.validate_identity_set(&[1, 3]).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn relationship_lookup() {
        let mut ext = DefaultWiresExtension::default();
        ext.relationships.insert(7, RelationshipKind::References);
        assert_eq!(ext.relationship_kind_label(7), Some("references"));
    }

    #[semio_framework_async_macros::async_test]
    async fn topic_lookup_stays_local_to_the_wires_extension() {
        let mut ext = DefaultWiresExtension::default();
        ext.topics.insert(7, "Context".into());
        assert_eq!(ext.topic_label(7), Some("Context"));
    }

    #[semio_framework_async_macros::async_test]
    async fn metabolism_fixture_hydrates_extension() {
        // 📜️ The `.wires` fixture is handcrafted in `crate::artifacts::wires::dsl`'s DSL — parse it,
        // then hydrate this crate's JSON-facing extension from its `wires_fixture` value, the same
        // shape `from_fixture_json` has always expected.
        let document = crate::artifacts::wires::schema::metabolism_wires_example_snapshot().expect("valid metabolism fixture mutations");
        let json = dsl::os_pack::json::to_string(&crate::artifacts::wires::schema::dsl_to_json(&document.wires_fixture));
        let ext = DefaultWiresExtension::from_fixture_json(&json).expect("metabolism fixture");
        assert_eq!(ext.topics.len(), 7);
        assert_eq!(ext.relationships.len(), 9);
        assert_eq!(ext.relationship_kind_label(8), Some("is"));
        assert!(ext.validate_identity_set(&[1, 2, 3]).is_ok());
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semantic_contract {
    use super::*;

    fn project(node: BuiltNode) -> serde_json::Value {
        let text = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
        serde_json::from_str(&text).expect("independent UI oracle")
    }

    #[test]
    fn wires_semantic_panels_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️panels.json")).expect("neutral UI vectors");
        let document = crate::artifacts::wires::empty_wires_snapshot();
        for row in vectors["cases"].as_array().expect("locales") {
            let labels = semio_framework_plugin::resolve_labels_for_locale::<crate::editor::wires::terminology::WiresLabels>(row["locale"].as_str().expect("locale"));
            let tree = project(crate::editor::wires::panels::document::render(&document, labels).expect("document"));
            assert_eq!(tree["children"][0]["component"]["label"], row["identities"]);
            assert_eq!(tree["children"][1]["component"]["label"], row["relationships"]);
            let tree = project(crate::editor::wires::panels::catalogue::render(&document.wires_fixture, labels).expect("catalogue"));
            assert_eq!(tree["children"][0]["component"]["label"], row["identityKinds"]);
            assert_eq!(tree["children"][1]["component"]["label"], row["relationshipKinds"]);
            let tree = project(render(&document, labels).expect("inspection"));
            let lines = tree["children"][0]["children"].as_array().expect("summary").iter().map(|node| node["component"]["value"].clone()).collect::<Vec<_>>();
            assert_eq!(serde_json::Value::Array(lines), row["summary"]);
        }
        let board = crate::artifacts::wires::wires_working_board(&document);
        for node in [
            crate::editor::wires::modes::edit::windows::canvas::render(&board, &document.wires_fixture).expect("editor canvas"),
            crate::viewer::wires::modes::view::windows::canvas::render(&document).expect("viewer canvas"),
        ] {
            let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("canvas surface") };
            let scene: semio_framework_plugin::Canvas2dScene = semio_framework_ui_scene::decode(props).expect("packed canvas");
            assert_eq!(scene.camera_x, vectors["canvas"]["cameraX"].as_f64().expect("camera x"));
            assert_eq!(scene.camera_y, vectors["canvas"]["cameraY"].as_f64().expect("camera y"));
            assert_eq!(scene.zoom, vectors["canvas"]["zoom"].as_f64().expect("zoom"));
            assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.layers_json).expect("independent layers oracle"), vectors["canvas"]["layers"]);
            project(node);
        }
    }
}
