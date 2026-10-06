//! 🧬️ Layout artifact schema — every field of the artifact with its state class.

use crate::{LayoutDrawingChild, LAYOUT_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Artifact
/// 🧬️ layout document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.layout.layout")]
pub struct LayoutArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    pub grid: GridSettings,
    #[state(artifact)]
    pub paragraph_styles: Vec<ParagraphStyle>,
    #[state(artifact)]
    pub character_styles: Vec<CharacterStyle>,
    #[state(artifact)]
    pub stories: Vec<TextStory>,
    #[state(artifact)]
    pub links: Vec<ImageLink>,
    #[state(artifact)]
    pub parent_pages: Vec<ParentPage>,
    #[state(artifact)]
    pub spreads: Vec<Spread>,
    #[state(artifact)]
    pub pages: Vec<Page>,
    #[state(artifact)]
    pub print_target: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data_fields: Option<crate::FormDictionary>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub background_drawing: Option<LayoutDrawingChild>,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub referenced_model: Option<store::ArtifactLink>,
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
            data_fields: None,
            background_drawing: None,
            referenced_model: None,
        }
    }
}

impl LayoutArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::LayoutSnapshot {
        crate::LayoutSnapshot {
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
            data_fields: self.data_fields.clone(),
            background_drawing: self.background_drawing.clone(),
            referenced_model: self.referenced_model.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::LayoutSnapshot) -> Self {
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
            data_fields: snapshot.data_fields,
            background_drawing: snapshot.background_drawing,
            referenced_model: snapshot.referenced_model,

        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::LayoutSnapshot) {
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
        self.data_fields = snapshot.data_fields;
        self.background_drawing = snapshot.background_drawing;
        self.referenced_model = snapshot.referenced_model;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.layout.layout` — twenty handcrafted schema leaves.
pub fn layout_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.layout.layout",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"), typescript: include_str!("🔺️diff/🟦️.ts"), graphql: include_str!("🔺️diff/🔗️.graphql"), json_schema: include_str!("🔺️diff/🔣️.json"), proto: include_str!("🔺️diff/🛰️.proto")
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 📄️Document


pub struct ResolvedFrame {
    pub frame: crate::Frame,
    pub inherited: bool,
}

pub fn apply_page_override(frame: &mut crate::Frame, page_override: &crate::PageOverride) {
    if let Some(bounds) = &page_override.bounds {
        match frame {
            crate::Frame::Rect { bounds: target, .. } | crate::Frame::Text { bounds: target, .. } | crate::Frame::Image { bounds: target, .. } => *target = bounds.clone(),
        }
    }
    match frame {
        crate::Frame::Rect { visible, locked, .. } | crate::Frame::Text { visible, locked, .. } | crate::Frame::Image { visible, locked, .. } => {
            if page_override.visible.is_some() {
                *visible = page_override.visible;
            }
            if page_override.locked.is_some() {
                *locked = page_override.locked;
            }
        }
    }
}

pub fn resolve_page<'a>(doc: &'a crate::LayoutSnapshot, page: &'a Page) -> Vec<ResolvedFrame> {
    let mut frames = Vec::new();
    if let Some(parent_id) = &page.parent_page_id {
        if let Some(parent) = doc.parent_pages.iter().find(|p| p.id == *parent_id) {
            for frame in &parent.frames {
                let page_override = page.overrides.iter().find(|item| item.object_id == frame.id());
                let inherited = page_override.is_none();
                let mut resolved = frame.clone();
                if let Some(page_override) = page_override {
                    apply_page_override(&mut resolved, page_override);
                }
                frames.push(ResolvedFrame { frame: resolved, inherited });
            }
        }
    }
    for frame in &page.frames {
        frames.push(ResolvedFrame { frame: frame.clone(), inherited: false });
    }
    frames
}
//#endregion 📄️Document

//#region 🔖️DocumentHelpers






/// 🎨️ Formats an optional RGBA color as a comma-separated text field value; two consumers
/// (`📌️panels/🔍️inspection` reads it, `🎮️commands/🖼️add-frame` parses it back via `text_to_rgba`).
pub fn rgba_to_text(color: &Option<[f32; 4]>) -> String {
    color.map(|channels| channels.iter().map(|channel| channel.to_string()).collect::<Vec<_>>().join(", ")).unwrap_or_default()
}

fn color_channel(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// 🎨️ `#rrggbb` for a color input. An absent color is black so the picker still has a value.
pub fn rgba_to_hex(color: &Option<[f32; 4]>) -> String {
    let [red, green, blue, _] = color.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    format!("#{:02x}{:02x}{:02x}", color_channel(red), color_channel(green), color_channel(blue))
}

fn hex_byte(text: &str) -> Option<u8> {
    u8::from_str_radix(text, 16).ok()
}

/// 🎨️ Parses `r, g, b, a` or `#rgb` / `#rrggbb` / `#rrggbbaa` into an RGBA color.
pub fn text_to_rgba(text: &str) -> Option<[f32; 4]> {
    let text = text.trim();
    if let Some(hex) = text.strip_prefix('#') {
        let (red, green, blue, alpha) = match hex.len() {
            3 => {
                let chars: Vec<char> = hex.chars().collect();
                (hex_byte(&format!("{}{}", chars[0], chars[0]))?, hex_byte(&format!("{}{}", chars[1], chars[1]))?, hex_byte(&format!("{}{}", chars[2], chars[2]))?, 255)
            }
            6 => (hex_byte(&hex[0..2])?, hex_byte(&hex[2..4])?, hex_byte(&hex[4..6])?, 255),
            8 => (hex_byte(&hex[0..2])?, hex_byte(&hex[2..4])?, hex_byte(&hex[4..6])?, hex_byte(&hex[6..8])?),
            _ => return None,
        };
        return Some([red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0, alpha as f32 / 255.0]);
    }
    let parts: Vec<f32> = text.split(',').filter_map(|part| part.trim().parse::<f32>().ok()).collect();
    (parts.len() == 4).then(|| [parts[0], parts[1], parts[2], parts[3]])
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️DocumentTests
#[cfg(test)]
#[path = "🧪️tests/🔬️document/🦀️.rs"]
mod document_tests;
//#endregion 🧪️DocumentTests

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔁️Re-exports
pub use crate::CharacterStyle;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::GridSettings;
pub use crate::ImageLink;
pub use crate::LayoutDropPreviewState;
pub use crate::Page;
pub use crate::ParagraphStyle;
pub use crate::ParentPage;
pub use crate::Spread;
pub use crate::TextStory;
//#endregion 🔁️Re-exports
