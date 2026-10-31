//! 🧬️ Layout snapshot schema — artifact-lane fields only.

#[cfg(test)]
#[cfg(test)] use crate::Frame;
use crate::{CharacterStyle, GridSettings, ImageLink, LayoutDrawingMember, Page, ParagraphStyle, ParentPage, Spread, TextStory};
#[cfg(test)] #[cfg(test)] use crate::LAYOUT_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Snapshot
/// 📸️ Persists Layout document fields and genuine Drawing member references.
/// Current child heads belong to their independent member stores; authored initial payloads remain in separate Drawing births.
/// Derived child/link slot tables and text/pack codecs share the same reference-only schema.
/// `ArtifactSchema` emits slot tables; text and pack are the
/// derived spec-driven encodings of the one `dsl::DslRecord` spec, composed child and link slot included.
/// Current child heads belong to their member stores.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields, retire_with="crate::snapshot::retire")]
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
    #[child(kind = "s.draw.drawing")]
    pub pages: Vec<Page>,
    #[state(artifact)]
    #[value(rename = "printTarget")]
    pub print_target: Option<String>,
    #[state(artifact)]
    #[value(rename = "dataFields", default, skip_serializing_if = "Option::is_none")]
    pub data_fields: Option<semio_framework_value_dictionary::QuestionDictionary>,
    #[state(artifact)]
    #[child(kind = "s.draw.drawing")]
    #[value(rename = "backgroundDrawing", default, skip_serializing_if = "Option::is_none")]
    pub background_drawing: Option<LayoutDrawingMember>,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    #[value(rename = "referencedModel", default, skip_serializing_if = "Option::is_none")]
    pub referenced_model: Option<store::ArtifactLink>,
}

/// 🧷️ Real "empty" constructor used as the parse/decode starting point (mirrors cad's
/// `empty_cad_snapshot`) — `default_document()` at `crate::schema` seeds a full
/// demo document instead, so this can't reuse a `Default` impl (this type has none).
pub fn empty_layout_snapshot() -> LayoutSnapshot {
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

//#endregion 🧪️Tests




/// 🧹️ Releases the original dictionary before the plain snapshot and member reference fields.
pub fn retire(mut value: LayoutSnapshot) {
    if let Some(dictionary) = value.data_fields.take() { <semio_framework_value_dictionary::QuestionDictionary as semio_framework_value::FromValue>::retire_decoded(dictionary); }
    drop(value);
}
