//! 🧬️ Wires artifact schema — every field of the artifact with its state class.

use dsl::DslValue;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full wires artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresArtifact {
    #[state(persistent)]
    pub wires_fixture: DslValue,
    #[state(persistent)]
    pub board_fixture: DslValue,
    #[state(shared_ui)]
    pub selected_ids: Vec<String>,
    #[state(preview)]
    pub drag_node_id: Option<String>,
    #[state(preview)]
    pub drag_last_x: f64,
    #[state(preview)]
    pub drag_last_y: f64,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for WiresArtifact {
    fn default() -> Self {
        Self {
            wires_fixture: crate::artifacts::wires::empty_wires_fixture(),
            board_fixture: crate::artifacts::wires::empty_board_fixture(),
            selected_ids: Vec::new(),
            drag_node_id: None,
            drag_last_x: 0.0,
            drag_last_y: 0.0,
            locale: "en-US".into(),
        }
    }
}

impl WiresArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::wires::WiresSnapshot {
        crate::artifacts::wires::WiresSnapshot {
            wires_fixture: self.wires_fixture.clone(),
            board_fixture: self.board_fixture.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::wires::WiresSnapshot) -> Self {
        Self {
            wires_fixture: snapshot.wires_fixture,
            board_fixture: snapshot.board_fixture,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::wires::WiresSnapshot) {
        self.wires_fixture = snapshot.wires_fixture;
        self.board_fixture = snapshot.board_fixture;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.reasoning.wires` — twenty handcrafted schema leaves.
pub fn wires_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.reasoning.wires",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
