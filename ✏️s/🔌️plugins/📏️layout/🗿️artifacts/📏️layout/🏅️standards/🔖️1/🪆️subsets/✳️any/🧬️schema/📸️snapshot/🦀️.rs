//! 🧬️ Layout snapshot schema — artifact-lane fields only.

#[cfg(test)]
use crate::Frame;
use crate::{CharacterStyle, GridSettings, ImageLink, LayoutDrawingChild, Page, ParagraphStyle, ParentPage, Spread, TextStory, LAYOUT_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Snapshot
/// 📸️ Persisted layout document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `background_drawing` composes stdio's real
/// `s.stdio.semio/v1/drawing` subset as a genuine child slot (see the artifact root's
/// `🔖️ComposedTypes` region doc for the full before/after); `referenced_model` is a forward
/// `ArtifactLink` reference slot, both new. `#[child(...)]`/`#[link_slot(...)]` drive
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Text and pack are the
/// derived spec-driven encodings of the one `dsl::DslRecord` spec, composed child and link slot included.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields, retire_with="sqlite::retire")]
#[dsl(extension = "layout")]
#[artifact_schema(id = "s.layout.layout")]
pub struct LayoutSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    pub grid: GridSettings,
    #[state(artifact)]
    #[value(rename = "paragraphStyles")]
    pub paragraph_styles: Vec<ParagraphStyle>,
    #[state(artifact)]
    #[value(rename = "characterStyles")]
    pub character_styles: Vec<CharacterStyle>,
    #[state(artifact)]
    pub stories: Vec<TextStory>,
    #[state(artifact)]
    pub links: Vec<ImageLink>,
    #[state(artifact)]
    #[value(rename = "parentPages")]
    pub parent_pages: Vec<ParentPage>,
    #[state(artifact)]
    pub spreads: Vec<Spread>,
    #[state(artifact)]
    pub pages: Vec<Page>,
    #[state(artifact)]
    #[value(rename = "printTarget")]
    pub print_target: Option<String>,
    #[state(artifact)]
    #[value(rename = "dataFields", default, skip_serializing_if = "Option::is_none")]
    pub data_fields: Option<crate::FormDictionary>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(rename = "backgroundDrawing", default, skip_serializing_if = "Option::is_none")]
    pub background_drawing: Option<LayoutDrawingChild>,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    #[value(rename = "referencedModel", default, skip_serializing_if = "Option::is_none")]
    pub referenced_model: Option<store::ArtifactLink>,
}

/// 🧷️ Real "empty" constructor used as the parse/decode starting point (mirrors cad's
/// `empty_cad_snapshot`) — `default_document()` at `crate::schema` seeds a full
/// demo document instead, so this can't reuse a `Default` impl (this type has none).
pub(crate) fn empty_layout_snapshot() -> LayoutSnapshot {
    LayoutSnapshot {
        schema: String::new(),
        name: String::new(),
        grid: GridSettings { baseline_grid: 0.0, baseline_offset: 0.0, snap_to_baseline: false },
        paragraph_styles: Vec::new(),
        character_styles: Vec::new(),
        stories: Vec::new(),
        links: Vec::new(),
        parent_pages: Vec::new(),
        spreads: Vec::new(),
        pages: Vec::new(),
        print_target: None,
        data_fields: None,
        background_drawing: None,
        referenced_model: None,
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️round-trip/🦀️.rs"]
mod round_trip_tests;
//#endregion 🧪️Tests



#[path="🧩️component/🦀️.rs"]
pub mod drawing_child;
