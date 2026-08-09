//! 🧬️ Present artifact schema — every field of the artifact with its state class.

use crate::artifacts::present::{FigureTileDraft, FigureTileSource, PRESENT_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full present artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.present")]
pub struct PresentArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub source: FigureTileSource,
    #[state(persistent)]
    pub tiles: Vec<FigureTileDraft>,
    #[state(shared_ui)]
    pub selected_ids: Vec<String>,
    #[state(local_ui)]
    pub engagement_input: String,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PresentArtifact {
    fn default() -> Self {
        Self {
            schema: PRESENT_DOCUMENT_SCHEMA.into(),
            source: crate::artifacts::present::default_figure_tile_source(),
            tiles: Vec::new(),
            selected_ids: Vec::new(),
            engagement_input: String::new(),
            locale: "en-US".into(),
        }
    }
}

impl PresentArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::present::PresentSnapshot {
        crate::artifacts::present::PresentSnapshot {
            schema: self.schema.clone(),
            source: self.source.clone(),
            tiles: self.tiles.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::present::PresentSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            source: snapshot.source,
            tiles: snapshot.tiles,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::present::PresentSnapshot) {
        self.schema = snapshot.schema;
        self.source = snapshot.source;
        self.tiles = snapshot.tiles;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.animate.present` — fifteen handcrafted schema leaves.
pub fn present_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.animate.present",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
