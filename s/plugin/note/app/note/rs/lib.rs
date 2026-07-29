//! 📝 Note app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖Constants
pub const NOTE_DOCUMENT_SCHEMA: &str = "note.document";
//#endregion 🔖Constants

//#region 🔖Types
// No `#[dsl(keyword = ...)]` here: every field of this type (`NoteDocument::camera`,
// `NoteOperation::SetCamera::camera`) is itself `#[dsl(block)]`, which already supplies the bare
// leading keyword from the FIELD's own name — an inner keyword too would print `camera { camera
// x=0 ... }`, doubled for no reason.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct NoteCamera {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_zoom")]
    pub zoom: f64,
}

pub fn default_zoom() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum NoteBlockNode {
    #[serde(rename = "text", rename_all = "camelCase")]
    Text {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        paragraphs: Vec<NoteTextParagraph>,
        font_size: f64,
        font_weight: String,
        align: String,
    },
    #[serde(rename = "image", rename_all = "camelCase")]
    Image {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        image_key: String,
    },
    #[serde(rename = "table", rename_all = "camelCase")]
    Table {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        columns: Vec<String>,
        rows: Vec<Vec<NoteTableCell>>,
    },
    #[serde(rename = "math", rename_all = "camelCase")]
    Math {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        tex: String,
        display_mode: bool,
    },
    #[serde(rename = "stroke", rename_all = "camelCase")]
    #[dsl(key = "stroke")]
    Ink {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        points: Vec<[f64; 2]>,
        stroke_width: f64,
        color: [f64; 4],
    },
    #[serde(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default)]
        rotation: f64,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default)]
        locked: bool,
        #[dsl(statements, block)]
        children: Vec<NoteBlockNode>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "r")]
pub struct NoteTextRun {
    #[dsl(positional)]
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "p")]
pub struct NoteTextParagraph {
    pub runs: Vec<NoteTextRun>,
}

pub fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct NoteTableCell {
    #[dsl(positional)]
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct NoteImageAsset {
    pub mime: String,
    pub data: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "note", layout = "lines")]
pub struct NoteDocument {
    pub schema: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default = "default_camera")]
    #[dsl(block)]
    pub camera: NoteCamera,
    #[serde(default)]
    #[dsl(statements, block)]
    pub blocks: Vec<NoteBlockNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_subdivisions: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_grid_spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pencil_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eraser_radius: Option<f64>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub assets: BTreeMap<String, NoteImageAsset>,
}

pub fn default_camera() -> NoteCamera {
    NoteCamera { x: 0.0, y: 0.0, zoom: 1.0 }
}
//#endregion 🔖Types

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_document_round_trips_assets_and_grid_settings() {
        let mut document = NoteDocument {
            schema: NOTE_DOCUMENT_SCHEMA.into(),
            id: "empty".into(),
            title: None,
            camera: default_camera(),
            blocks: Vec::new(),
            grid_visible: Some(true),
            grid_spacing: Some(32.0),
            grid_subdivisions: Some(4.0),
            grid_opacity: Some(0.35),
            snap_enabled: Some(false),
            snap_grid_spacing: Some(8.0),
            pencil_width: Some(3.0),
            eraser_radius: Some(12.0),
            assets: BTreeMap::new(),
        };
        document.assets.insert(
            "asset-1".into(),
            NoteImageAsset {
                mime: "image/png".into(),
                data: "data:image/png;base64,abc".into(),
                width: Some(10.0),
                height: Some(20.0),
            },
        );
        document.grid_subdivisions = Some(6.0);
        document.grid_opacity = Some(0.5);
        let json_text = serde_json::to_string(&document).unwrap();
        let parsed: NoteDocument = serde_json::from_str(&json_text).unwrap();
        assert_eq!(parsed.assets.get("asset-1").unwrap().mime, "image/png");
        assert_eq!(parsed.grid_subdivisions, Some(6.0));
        assert_eq!(parsed.grid_opacity, Some(0.5));
    }
}
//#endregion 🧪Tests
