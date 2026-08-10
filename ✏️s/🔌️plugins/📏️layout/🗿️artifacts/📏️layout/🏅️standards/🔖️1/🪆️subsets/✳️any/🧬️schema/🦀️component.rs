//! 🧬️ Layout artifact schema — every field of the artifact with its state class.

use crate::artifacts::layout::{
    CharacterStyle, GridSettings, ImageLink, LayoutDropPreviewState, Page, ParagraphStyle, ParentPage, Spread, TextStory,
    LAYOUT_DOCUMENT_SCHEMA,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full layout artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.layout.layout")]
pub struct LayoutArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub name: String,
    #[state(persistent)]
    pub grid: GridSettings,
    #[state(persistent)]
    pub paragraph_styles: Vec<ParagraphStyle>,
    #[state(persistent)]
    pub character_styles: Vec<CharacterStyle>,
    #[state(persistent)]
    pub stories: Vec<TextStory>,
    #[state(persistent)]
    pub links: Vec<ImageLink>,
    #[state(persistent)]
    pub parent_pages: Vec<ParentPage>,
    #[state(persistent)]
    pub spreads: Vec<Spread>,
    #[state(persistent)]
    pub pages: Vec<Page>,
    #[state(persistent)]
    pub print_target: Option<String>,
    #[state(persistent)]
    pub data_fields_json: Option<String>,
    #[state(shared_ui)]
    pub selected_ids: Vec<String>,
    #[state(local_ui)]
    pub active_page_id: String,
    #[state(local_ui)]
    pub engagement_input: String,
    #[state(local_ui)]
    pub camera_x: f64,
    #[state(local_ui)]
    pub camera_y: f64,
    #[state(local_ui)]
    pub camera_zoom: f64,
    #[state(local_ui)]
    pub preview_camera_x: f64,
    #[state(local_ui)]
    pub preview_camera_y: f64,
    #[state(local_ui)]
    pub preview_camera_zoom: f64,
    #[state(local_ui)]
    pub drop_preview: LayoutDropPreviewState,
    #[state(local_ui)]
    pub locale: String,
    #[state(preview)]
    pub hovered_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for LayoutArtifact {
    fn default() -> Self {
        Self {
            schema: LAYOUT_DOCUMENT_SCHEMA.into(),
            name: String::new(),
            grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
            paragraph_styles: Vec::new(),
            character_styles: Vec::new(),
            stories: Vec::new(),
            links: Vec::new(),
            parent_pages: Vec::new(),
            spreads: Vec::new(),
            pages: Vec::new(),
            print_target: None,
            data_fields_json: None,
            selected_ids: Vec::new(),
            active_page_id: "page-1".into(),
            engagement_input: String::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            preview_camera_x: 0.0,
            preview_camera_y: 0.0,
            preview_camera_zoom: 1.0,
            drop_preview: LayoutDropPreviewState::default(),
            locale: "en-US".into(),
            hovered_id: None,
        }
    }
}

impl LayoutArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::layout::LayoutSnapshot {
        crate::artifacts::layout::LayoutSnapshot {
            schema: self.schema.clone(),
            name: self.name.clone(),
            grid: self.grid.clone(),
            paragraph_styles: self.paragraph_styles.clone(),
            character_styles: self.character_styles.clone(),
            stories: self.stories.clone(),
            links: self.links.clone(),
            parent_pages: self.parent_pages.clone(),
            spreads: self.spreads.clone(),
            pages: self.pages.clone(),
            print_target: self.print_target.clone(),
            data_fields_json: self.data_fields_json.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::layout::LayoutSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            name: snapshot.name,
            grid: snapshot.grid,
            paragraph_styles: snapshot.paragraph_styles,
            character_styles: snapshot.character_styles,
            stories: snapshot.stories,
            links: snapshot.links,
            parent_pages: snapshot.parent_pages,
            spreads: snapshot.spreads,
            pages: snapshot.pages,
            print_target: snapshot.print_target,
            data_fields_json: snapshot.data_fields_json,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::layout::LayoutSnapshot) {
        self.schema = snapshot.schema;
        self.name = snapshot.name;
        self.grid = snapshot.grid;
        self.paragraph_styles = snapshot.paragraph_styles;
        self.character_styles = snapshot.character_styles;
        self.stories = snapshot.stories;
        self.links = snapshot.links;
        self.parent_pages = snapshot.parent_pages;
        self.spreads = snapshot.spreads;
        self.pages = snapshot.pages;
        self.print_target = snapshot.print_target;
        self.data_fields_json = snapshot.data_fields_json;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.layout.layout` — twenty handcrafted schema leaves.
pub fn layout_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.layout.layout",
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
