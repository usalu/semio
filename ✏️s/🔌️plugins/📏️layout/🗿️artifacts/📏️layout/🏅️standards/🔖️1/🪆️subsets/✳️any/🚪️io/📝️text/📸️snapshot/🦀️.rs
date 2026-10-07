//! 📜️ Layout artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::LayoutSnapshot;

/// 📄️ The bundled sample fixture, handcrafted in the `.layout` DSL.
pub const LAYOUT_SAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.layout` DSL text into a `LayoutSnapshot`.
pub fn parse_dsl(text: &str) -> Result<LayoutSnapshot, semio_framework_diagnostic::TextError> {
    <LayoutSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `LayoutSnapshot` back to `.layout` DSL text.
pub fn print_dsl(document: &LayoutSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type LayoutSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
#[cfg(test)]
use crate::Frame;
use crate::{CharacterStyle, GridSettings, ImageLink, LayoutDrawingChild, Page, ParagraphStyle, ParentPage, Spread, TextStory, LAYOUT_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

/// ✉️ `ArtifactDsl` and `ArtifactPack` over the one derived record spec.
impl store::ArtifactDsl for LayoutSnapshot {
    const EXTENSION: &'static str = "layout";
    fn envelope_id() -> &'static str {
        LAYOUT_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{LayoutDiff, LayoutSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::standards::v1::subsets::any::schema::mutations::{change_data_fields,change_frame_columns,change_frame_fill,change_frame_stroke,change_frame_wrap_mode,change_link_path,change_page_height,change_page_width,change_print_target,create_frame,create_link,create_page,create_story,delete_frame,delete_link,delete_page,delete_story,drag_frames,edit_story,move_frame,rename_layout,rotate_frames,scale_frames,rename_page,reorder_pages,resize_frame,rotate_frame,set_frame_flags,update_grid,create_character_style,delete_character_style,set_page_guides,set_page_parent,set_story_runs,update_link,set_page_overrides,create_layer,set_frame_layer,set_drawing_text,reorder_frame,update_character_style,update_layer,update_page_columns,update_page_margins,update_paragraph_style,update_parent_page,update_spread,update_text_frame};


/// 🔁️ Parses the committed `.dsl.semio` example, prints it back and parses that, answering
/// `{"printed": …, "snapshot": …, "reparsed": …}` so a caller can weigh the identity law's two
/// halves — the bytes against the committed artifact, and the projection against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn round_trip_layout_dsl(text: &str) -> Result<String, String> {
    use store::ArtifactDsl;
    let parsed = <LayoutSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed layout example does not parse: {error:?}"))?;
    let printed = <LayoutSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <LayoutSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted layout document does not parse: {error:?}"))?;
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::json!({ "printed": printed, "snapshot": parsed, "reparsed": reparsed })))
}
}
pub use mutations_codec::*;


#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::{LayoutDrawingChild, LAYOUT_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::CharacterStyle;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::GridSettings;
use crate::ImageLink;
use crate::LayoutDropPreviewState;
use crate::Page;
use crate::ParagraphStyle;
use crate::ParentPage;
use crate::Spread;
use crate::TextStory;

/// 📄️ The bundled sample fixture, parsed once — the source of truth for `LayoutPlayApp::initial_snapshot`
/// and the app manifest's `.example(...)` document. Relocated from the deleted `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub fn default_document() -> crate::LayoutSnapshot {
    build_demo_layout_snapshot()
}

pub(crate) fn build_demo_layout_snapshot() -> crate::LayoutSnapshot {
    crate::LayoutSnapshot {
        schema: LAYOUT_DOCUMENT_SCHEMA.into(),
        name: "Demo".into(),
        grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: true },
        paragraph_styles: vec![ParagraphStyle { id: "paragraph.body".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() }],
        character_styles: Vec::new(),
        stories: vec![TextStory { id: "story-1".into(), content: "Hello layout".into(), style_runs: Vec::new() }],
        links: vec![ImageLink { id: "link-missing".into(), path: "assets/missing.png".into(), hash: "sha256:missing".into(), width: 100, height: 100, dpi: 300, color_profile: None, state: Some("missing".into()), proxy_data_url: None, artifact_kind: String::new(), artifact_ref: String::new() }],
        parent_pages: vec![ParentPage {
            id: "parent-1".into(),
            name: "Master".into(),
            width: 400.0,
            height: 500.0,
            layer_ids: vec!["layer-parent".into()],
            layers: vec![crate::Layer { id: "layer-parent".into(), name: "Master".into(), visible: true, locked: false, object_ids: vec!["frame-inherited".into()] }],
            frames: vec![crate::Frame::Rect {
                id: "frame-inherited".into(),
                layer_id: "layer-parent".into(),
                bounds: crate::LayoutBounds { x: 50.0, y: 50.0, width: 100.0, height: 80.0, rotation: 0.0 },
                locked: None,
                visible: None,
                fill: None,
                stroke: Some([0.4, 0.5, 0.7, 0.8]),
            }],
        }],
        spreads: vec![Spread { id: "spread-1".into(), name: "Spread 1".into(), page_ids: vec!["page-1".into(), "page-2".into()] }],
        pages: vec![
            Page {
                id: "page-1".into(),
                name: "Page 1".into(),
                spread_id: "spread-1".into(),
                parent_page_id: Some("parent-1".into()),
                width: 400.0,
                height: 500.0,
                margins: crate::PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
                columns: crate::PageColumns { count: 1, gutter: 0.0 },
                guides: Vec::new(),
                layer_ids: vec!["layer-1".into()],
                layers: vec![crate::Layer { id: "layer-1".into(), name: "Content".into(), visible: true, locked: false, object_ids: vec!["frame-text-1".into(), "frame-image-1".into(), "frame-1".into()] }],
                frames: vec![
                    crate::Frame::Text {
                        id: "frame-text-1".into(),
                        layer_id: "layer-1".into(),
                        bounds: crate::LayoutBounds { x: 156.0, y: 220.0, width: 80.0, height: 40.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        story_id: "story-1".into(),
                        thread_next: None,
                        columns: 1,
                        inset: crate::LayoutRect { x: 0.0, y: 0.0, width: 80.0, height: 40.0 },
                        wrap_mode: "box".into(),
                    },
                    crate::Frame::Image {
                        id: "frame-image-1".into(),
                        layer_id: "layer-1".into(),
                        bounds: crate::LayoutBounds { x: 136.0, y: 435.0, width: 60.0, height: 40.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        link_id: "link-missing".into(),
                    },
                    crate::Frame::Rect {
                        id: "frame-1".into(),
                        layer_id: "layer-1".into(),
                        bounds: crate::LayoutBounds { x: 10.0, y: 10.0, width: 40.0, height: 40.0, rotation: 0.0 },
                        locked: None,
                        visible: None,
                        fill: Some([1.0, 1.0, 1.0, 1.0]),
                        stroke: None,
                    },
                ],
                overrides: Vec::new(),
            },
            Page {
                id: "page-2".into(),
                name: "Page 2".into(),
                spread_id: "spread-1".into(),
                parent_page_id: None,
                width: 400.0,
                height: 500.0,
                margins: crate::PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
                columns: crate::PageColumns { count: 1, gutter: 0.0 },
                guides: Vec::new(),
                layer_ids: Vec::new(),
                layers: Vec::new(),
                frames: Vec::new(),
                overrides: Vec::new(),
            },
        ],
        print_target: None,
        data_fields: None,
        background_drawing: None,
        referenced_model: None,
    }
}

/// 🌉️ JSON bridge for `semio_framework_plugin::App::example`, which hardcodes `serde_json::from_str`
/// on its `document_json` parameter (shared framework machinery, out of scope for this DSL migration) —
/// derives the JSON from the DSL fixture rather than keeping a second, redundant JSON copy of it on disk.
pub fn layout_sample_document_json() -> String {
    semio_framework_pack_json::to_json_string(&default_document())
}
}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::{LayoutDrawingChild, LAYOUT_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::CharacterStyle;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::GridSettings;
use crate::ImageLink;
use crate::LayoutDropPreviewState;
use crate::Page;
use crate::ParagraphStyle;
use crate::ParentPage;
use crate::Spread;
use crate::TextStory;

/// 📄️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — pure over `LayoutSnapshot`/`Page`, no engine state, no app type.
pub fn parse_layout_document(json: &str) -> Result<crate::LayoutSnapshot, crate::standards::v1::subsets::any::io::LayoutError> {
    let doc: crate::LayoutSnapshot = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
    if doc.schema != LAYOUT_DOCUMENT_SCHEMA {
        return Err(crate::standards::v1::subsets::any::io::LayoutError::UnexpectedSchema(doc.schema));
    }
    Ok(doc)
}
}
pub use snapshot_wire2_codec::*;
