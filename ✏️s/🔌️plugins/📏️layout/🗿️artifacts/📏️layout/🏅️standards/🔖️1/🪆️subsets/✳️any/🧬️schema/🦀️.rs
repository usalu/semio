//! 🧬️ Layout artifact schema — every field of the artifact with its state class.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;


use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::snapshot::LayoutSnapshot;
use semio_framework_value::DslValue as Value;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_io_ports::{EmbeddedChildSlotV1, VectorScene};
use protocol::{MutationKind, SemanticDescriptor};
use schema::ArtifactSchema;
use semio_framework_artifact_reference::schema::SubsetId;
//#region 🔖️Artifact
/// 🧬️ layout document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
    pub data_fields: Option<semio_framework_value_dictionary::QuestionDictionary>,
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
    pub fn to_snapshot(&self) -> crate::snapshot::LayoutSnapshot {
        crate::snapshot::LayoutSnapshot {
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
    pub fn from_snapshot(snapshot: crate::snapshot::LayoutSnapshot) -> Self {
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
    pub fn set_snapshot(&mut self, snapshot: crate::snapshot::LayoutSnapshot) {
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

pub fn resolve_page<'a>(doc: &'a crate::snapshot::LayoutSnapshot, page: &'a Page) -> Vec<ResolvedFrame> {
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

/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.


//#endregion 🔁️Re-exports

pub const LAYOUT_DOCUMENT_SCHEMA: &str = "layout.layout";

/// 🪪️ This artifact's compile-time surface coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level (not
/// under `✏️editor`/`👁️viewer`) specifically so a viewer file can read it without ever importing
/// through the sibling editor module. `artifact_kind = "s.layout.layout"` matches
/// `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`'s own `#[artifact_schema(id =
/// "s.layout.layout")]`; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is `s.layout.layout@1/*#editor`
/// / `s.layout.layout@1/*#viewer`, exactly the contract §1 grammar. NOT the same type as
/// `semio_framework_artifact_reference::ArtifactDialect` used a few lines below by `background_drawing_child_handle` (a
/// wire `ArtifactDialect` describing STDIO's drawing subset for composition purposes, unrelated to
/// this artifact's own surface identity) — this is the SDK's compile-time `Dialect`.
pub const LAYOUT_DIALECT: Dialect = Dialect { artifact_kind: "s.layout.layout", standard: StandardId("1"), subset: SubsetId::ANY };

/// 🌉️ Layout carries one persisted background child (`LayoutSnapshot::background_drawing`), the imported plan a sheet is laid out on.
/// The content is an opaque [`Value`]: layout never names the owner of the child's standard. Everything layout needs from it, validating, reading as a
/// [`VectorScene`], writing one back, its SQLite DDL and its external formats, goes through the codec the composition root binds to
/// [`LAYOUT_BACKGROUND_DRAWING`]. `LayoutSnapshot::referenced_model` is the forward `ArtifactLink` slot for a referenced `model` artifact, schema and codec
/// complete but otherwise inert (no `LinkResolver` seam, no mutation dispatch).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields, retire_with="crate::standards::v1::subsets::any::schema::snapshot::drawing_child::retire")]
pub struct LayoutDrawingChild {
    pub handle: store::os_store::artifact_child::ArtifactChild<LayoutBackgroundDrawing>,
    #[value(deserialize_with = "validated_drawing_content")]
    pub content: Value,
}

/// 🧸️ Compile-time-only name of the child `LayoutDrawingChild::handle` points at; it never appears on the wire.
pub struct LayoutBackgroundDrawing;

/// 🧩️ The position the composition root fills with the codec of the background drawing's owner.
pub static LAYOUT_BACKGROUND_DRAWING: EmbeddedChildSlotV1 = EmbeddedChildSlotV1::new("layout.background-drawing");

impl semio_framework_dsl_record::BorrowedDslRecord for LayoutDrawingChild {
    const RECORD: semio_framework_dsl_record::BorrowedRecordSpec = semio_framework_dsl_record::BorrowedRecordSpec { keyword: None, layout: semio_framework_dsl_record::RecordLayout::Inline, fields: &[
        semio_framework_dsl_record::BorrowedFieldSpec { id: 0, key: "handle", position: None, shape: <store::os_store::artifact_child::ArtifactChild<LayoutBackgroundDrawing> as semio_framework_dsl_record::BorrowedDslField>::SHAPE, optional: false, flatten: false, defines: None, is_call_name: false },
        semio_framework_dsl_record::BorrowedFieldSpec { id: 1, key: "content", position: None, shape: semio_framework_dsl_record::BorrowedShape::Value, optional: false, flatten: false, defines: None, is_call_name: false },
    ] };
}

impl semio_framework_dsl_record::BorrowedDslField for LayoutDrawingChild {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
}

impl semio_framework_schema_composition::ChildFieldRefs for LayoutDrawingChild {
    const MANY: bool = false;
    fn visit_child_field<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error> {
        visitor.step()?;
        semio_framework_schema_composition::ChildFieldRefs::visit_child_field(&self.handle, slot, visitor)
    }
}

fn layout_drawing_child_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Inline, vec![semio_framework_dsl_record::FieldSpec::new(0, "handle", <store::os_store::artifact_child::ArtifactChild<LayoutBackgroundDrawing> as semio_framework_dsl_record::DslField>::shape()), semio_framework_dsl_record::FieldSpec::new(1, "content", semio_framework_dsl_record::Shape::Value)])
}

fn layout_drawing_child_spec_producer()->semio_framework_dsl_record::RecordSpecProducer{crate::snapshot::drawing_child::producer()}

/// 🧬️ Record binding for the derived `LayoutSnapshot` pack: the child handle through the framework's own `ArtifactChild` record, the content as its first-party value, admitted by the bound codec.
impl semio_framework_dsl_record::DslField for LayoutDrawingChild {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(layout_drawing_child_spec_producer()))}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{crate::snapshot::drawing_child::decode(value,control)}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{crate::snapshot::drawing_child::encode(self,control)}
    fn retire_decoded(self){crate::snapshot::drawing_child::retire(self)}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(layout_drawing_child_spec_producer())
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        record.fields.insert(0, semio_framework_dsl_record::DslField::to_value(&self.handle));
        record.fields.insert(1, semio_framework_dsl_record::FieldValue::Value(self.content.clone()));
        semio_framework_dsl_record::FieldValue::Record(record)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else { return Err(format!("expected Record, found {value:?}")) };
        let handle = <store::os_store::artifact_child::ArtifactChild<LayoutBackgroundDrawing> as semio_framework_dsl_record::DslField>::from_value(record.get(0).ok_or("missing handle")?)?;
        let Some(semio_framework_dsl_record::FieldValue::Value(content)) = record.get(1) else { return Err("expected content value".into()) };
        let content = content.clone();
        let content = validated_drawing_content(content).map_err(|error| error.to_string())?;
        Ok(Self { handle, content })
    }
}

/// 🧮️ The decode budget of a validation that no caller controls; a controlled decode validates under its own budget.
const UNCONTROLLED_VALIDATION_BYTES: usize = 64 << 20;

/// 🔎️ The codec bound to [`LAYOUT_BACKGROUND_DRAWING`], or a refusal naming the unbound slot.
pub fn drawing_codec() -> Result<&'static dyn semio_framework_io_ports::EmbeddedChildCodecV1, semio_framework_io_ports::EmbeddedChildSlotError> {
    LAYOUT_BACKGROUND_DRAWING.codec()
}

fn drawing_error(error: impl std::fmt::Display) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("background drawing: {error}"))
}

/// ✅️ The wire reader of the child content: the bound codec admits the value, or the child is refused.
fn validated_drawing_content(value: Value) -> Result<Value, semio_framework_value::ValueError> {
    drawing_codec().map_err(drawing_error)?.validate(&value, &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(UNCONTROLLED_VALIDATION_BYTES, &mut |_| true))?;
    Ok(value)
}

/// 🪪️ Mints one source-independent identity from the canonical drawing payload.
pub fn background_drawing_child_handle(_source_tag: &str, content: &Value) -> LayoutDrawingChild {
    let content_json = semio_framework_pack_json::to_json_string(content);
    let child_id = store::content_id("background-drawing", content_json.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    LayoutDrawingChild { handle: store::os_store::artifact_child::ArtifactChild::new(child_id, target), content: content.clone() }
}

/// 🔎️ The one accessor every render/export call site funnels through; `None` only when the
/// document has no snapshot-owned `background_drawing` record.
pub fn background_drawing_content(snapshot: &LayoutSnapshot) -> Option<Value> {
    snapshot.background_drawing.as_ref().map(|child| child.content.clone())
}

/// 🔽️ The background drawing read as a neutral vector scene: `Ok(None)` when the document has no background drawing, a refusal when the slot is unbound or the content is not admitted.
pub fn background_drawing_scene(snapshot: &LayoutSnapshot) -> Result<Option<VectorScene>, semio_framework_value::ValueError> {
    let Some(child) = snapshot.background_drawing.as_ref() else { return Ok(None) };
    drawing_codec().map_err(drawing_error)?.lower(&child.content).map(Some)
}

/// ✏️ The central apply of one plan-text edit: the text node at `index` (depth-first over the layers) now reads `text`; `false` when the plan has no such node.
pub fn set_drawing_text(content: &mut Value, index: u32, text: &str) -> Result<bool, semio_framework_value::ValueError> {
    let codec = drawing_codec().map_err(drawing_error)?;
    let mut scene = codec.lower(content)?;
    if !scene.set_text(index, text) {
        return Ok(false);
    }
    *content = codec.lift(&scene)?;
    Ok(true)
}

/// 👻️ Ephemeral catalogue drag-ghost state (layout app config / artifact local-ui).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct LayoutDropPreviewState {
    pub kind: String,
    pub x: f64,
    pub y: f64,
}

/// 📷️ Per-window camera pose shared by the exact Blueprint and Preview config owner records.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct LayoutCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for LayoutCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct LayoutRect {
    pub x: f64,
    pub y: f64,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "w"))]
    #[value(rename = "w")]
    pub width: f64,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "h"))]
    #[value(rename = "h")]
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct LayoutBounds {
    pub x: f64,
    pub y: f64,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "w"))]
    #[value(rename = "w")]
    pub width: f64,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "h"))]
    #[value(rename = "h")]
    pub height: f64,
    pub rotation: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct PageMargins {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct PageColumns {
    pub count: u32,
    pub gutter: f64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct Layer {
    #[dsl(defines = "layer")]
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "objectIds"))]
    #[value(rename = "objectIds")]
    pub object_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(tag = "kind"))]
#[value(tag = "kind", deny_unknown_fields)]
pub enum Frame {
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "rect"))]
    #[value(rename = "rect")]
    Rect {
        #[dsl(defines = "frame")]
        id: String,
        #[cfg_attr(any(test, feature = "testkit"), serde(rename = "layerId"))]
        #[value(rename = "layerId")]
        #[dsl(refs = "layer")]
        layer_id: String,
        #[dsl(block)]
        bounds: LayoutBounds,
        locked: Option<bool>,
        visible: Option<bool>,
        fill: Option<[f32; 4]>,
        stroke: Option<[f32; 4]>,
    },
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "text"))]
    #[value(rename = "text")]
    Text {
        #[dsl(defines = "frame")]
        id: String,
        #[cfg_attr(any(test, feature = "testkit"), serde(rename = "layerId"))]
        #[value(rename = "layerId")]
        #[dsl(refs = "layer")]
        layer_id: String,
        #[dsl(block)]
        bounds: LayoutBounds,
        locked: Option<bool>,
        visible: Option<bool>,
        #[cfg_attr(any(test, feature = "testkit"), serde(rename = "storyId"))]
        #[value(rename = "storyId")]
        #[dsl(refs = "story")]
        story_id: String,
        #[cfg_attr(any(test, feature = "testkit"), serde(rename = "threadNext"))]
        #[value(rename = "threadNext")]
        #[dsl(refs = "frame")]
        thread_next: Option<String>,
        columns: u32,
        #[dsl(block)]
        inset: LayoutRect,
        #[cfg_attr(any(test, feature = "testkit"), serde(rename = "wrapMode"))]
        #[value(rename = "wrapMode")]
        wrap_mode: String,
    },
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "image"))]
    #[value(rename = "image")]
    Image {
        #[dsl(defines = "frame")]
        id: String,
        #[cfg_attr(any(test, feature = "testkit"), serde(rename = "layerId"))]
        #[value(rename = "layerId")]
        #[dsl(refs = "layer")]
        layer_id: String,
        #[dsl(block)]
        bounds: LayoutBounds,
        locked: Option<bool>,
        visible: Option<bool>,
        #[cfg_attr(any(test, feature = "testkit"), serde(rename = "linkId"))]
        #[value(rename = "linkId")]
        #[dsl(refs = "link")]
        link_id: String,
    },
}

impl Frame {
    pub fn id(&self) -> &str {
        match self {
            Frame::Rect { id, .. } | Frame::Text { id, .. } | Frame::Image { id, .. } => id,
        }
    }

    pub fn bounds(&self) -> &LayoutBounds {
        match self {
            Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds,
        }
    }

    pub fn kind_str(&self) -> &str {
        match self {
            Frame::Rect { .. } => "rect",
            Frame::Text { .. } => "text",
            Frame::Image { .. } => "image",
        }
    }

    pub fn visible(&self) -> bool {
        match self {
            Frame::Rect { visible, .. } | Frame::Text { visible, .. } | Frame::Image { visible, .. } => visible.unwrap_or(true),
        }
    }

    pub fn locked(&self) -> bool {
        match self {
            Frame::Rect { locked, .. } | Frame::Text { locked, .. } | Frame::Image { locked, .. } => locked.unwrap_or(false),
        }
    }

    pub fn layer_id(&self) -> &str {
        match self {
            Frame::Rect { layer_id, .. } | Frame::Text { layer_id, .. } | Frame::Image { layer_id, .. } => layer_id,
        }
    }
}

/// 🔒 A frame on a locked layer, or a frame that is itself locked, stays where it is.
pub fn frame_edits_blocked(document: &LayoutSnapshot, page: &Page, frame: &Frame) -> bool {
    frame.locked() || layer_locked(document, page, frame.layer_id())
}

/// 🔒 The layer is locked on this page, or on the page's parent when the frame is inherited.
pub fn layer_locked(document: &LayoutSnapshot, page: &Page, layer_id: &str) -> bool {
    if page.layers.iter().any(|layer| layer.id == layer_id && layer.locked) {
        return true;
    }
    page.parent_page_id.as_ref().and_then(|id| document.parent_pages.iter().find(|parent| parent.id == *id)).is_some_and(|parent| parent.layers.iter().any(|layer| layer.id == layer_id && layer.locked))
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct TextStyleRun {
    pub start: u64,
    pub end: u64,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "paragraphStyleId"))]
    #[value(rename = "paragraphStyleId")]
    #[dsl(refs = "paragraph-style")]
    pub paragraph_style_id: Option<String>,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "characterStyleId"))]
    #[value(rename = "characterStyleId")]
    #[dsl(refs = "character-style")]
    pub character_style_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct TextStory {
    #[dsl(defines = "story")]
    pub id: String,
    pub content: String,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "styleRuns"))]
    #[value(rename = "styleRuns")]
    #[dsl(table)]
    pub style_runs: Vec<TextStyleRun>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct ParagraphStyle {
    #[dsl(defines = "paragraph-style")]
    pub id: String,
    pub name: String,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "fontFamily"))]
    #[value(rename = "fontFamily")]
    pub font_family: String,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "fontSize"))]
    #[value(rename = "fontSize")]
    pub font_size: f64,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "fontWeight"))]
    #[value(rename = "fontWeight")]
    pub font_weight: u32,
    pub leading: f64,
    pub tracking: f64,
    pub alignment: String,
}

/// 🔤️ A named run-level style override (bold/italic/color emphasis) applied on top of a
/// {@link ParagraphStyle} via {@link TextStyleRun.character_style_id}. Unlike `ParagraphStyle`,
/// every field besides `id` is optional: a character style typically overrides only one or two
/// attributes and inherits the rest from the paragraph it's layered onto.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct CharacterStyle {
    #[dsl(defines = "character-style")]
    pub id: String,
    pub name: Option<String>,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "fontFamily"))]
    #[value(rename = "fontFamily")]
    pub font_family: Option<String>,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "fontSize"))]
    #[value(rename = "fontSize")]
    pub font_size: Option<f64>,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "fontWeight"))]
    #[value(rename = "fontWeight")]
    pub font_weight: Option<u32>,
    pub italic: Option<bool>,
    pub color: Option<[f32; 4]>,
    pub tracking: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct ImageLink {
    #[dsl(defines = "link")]
    pub id: String,
    pub path: String,
    pub hash: String,
    pub width: u32,
    pub height: u32,
    pub dpi: u32,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "colorProfile"))]
    #[value(rename = "colorProfile")]
    pub color_profile: Option<String>,
    pub state: Option<String>,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "proxyDataUrl"))]
    #[value(rename = "proxyDataUrl")]
    pub proxy_data_url: Option<String>,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "artifactKind", default))]
    #[value(rename = "artifactKind", default, skip_serializing_if = "String::is_empty")]
    pub artifact_kind: String,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "artifactRef", default))]
    #[value(rename = "artifactRef", default, skip_serializing_if = "String::is_empty")]
    pub artifact_ref: String,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct PageOverride {
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "objectId"))]
    #[value(rename = "objectId")]
    #[dsl(refs = "frame")]
    pub object_id: String,
    #[dsl(block)]
    pub bounds: Option<LayoutBounds>,
    pub visible: Option<bool>,
    pub locked: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct ParentPage {
    #[dsl(defines = "parent-page")]
    pub id: String,
    pub name: String,
    pub width: f64,
    pub height: f64,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "layerIds"))]
    #[value(rename = "layerIds")]
    pub layer_ids: Vec<String>,
    #[dsl(table)]
    pub layers: Vec<Layer>,
    #[dsl(statements, block)]
    pub frames: Vec<Frame>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct Page {
    pub id: String,
    pub name: String,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "spreadId"))]
    #[value(rename = "spreadId")]
    #[dsl(refs = "spread")]
    pub spread_id: String,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "parentPageId"))]
    #[value(rename = "parentPageId")]
    #[dsl(refs = "parent-page")]
    pub parent_page_id: Option<String>,
    pub width: f64,
    pub height: f64,
    #[dsl(block)]
    pub margins: PageMargins,
    #[dsl(block)]
    pub columns: PageColumns,
    #[dsl(table)]
    pub guides: Vec<LayoutRect>,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "layerIds"))]
    #[value(rename = "layerIds")]
    pub layer_ids: Vec<String>,
    #[dsl(table)]
    pub layers: Vec<Layer>,
    #[dsl(statements, block)]
    pub frames: Vec<Frame>,
    #[dsl(table)]
    pub overrides: Vec<PageOverride>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct Spread {
    #[dsl(defines = "spread")]
    pub id: String,
    pub name: String,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "pageIds"))]
    #[value(rename = "pageIds")]
    pub page_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(deny_unknown_fields)]
pub struct GridSettings {
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "baselineGrid"))]
    #[value(rename = "baselineGrid")]
    pub baseline_grid: f64,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "baselineOffset"))]
    #[value(rename = "baselineOffset")]
    pub baseline_offset: f64,
    #[cfg_attr(any(test, feature = "testkit"), serde(rename = "snapToBaseline"))]
    #[value(rename = "snapToBaseline")]
    pub snap_to_baseline: bool,
}

/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::layout::create_layout_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.layout".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Layout", "Layout"),
        source_format: LAYOUT_DOCUMENT_SCHEMA.into(),
        component_kind: "layout".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        schema: LAYOUT_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.dxf".into(), "stdio.json".into(), "stdio.png".into(), "stdio.svg".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.dxf".into(), "stdio.json".into(), "stdio.svg".into()],
    }
}

impl Identified<String> for Page {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for TextStory {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for ImageLink {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for ParagraphStyle {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for CharacterStyle {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for ParentPage {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for Spread {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🗂️ Re-derives what a page's frames and layers fix: the layer id list and every layer's frames in paint order.
pub fn derive_page_layers(page: &mut Page) {
    page.layer_ids = page.layers.iter().map(|layer| layer.id.clone()).collect();
    for layer in &mut page.layers {
        layer.object_ids = page.frames.iter().filter(|frame| frame.layer_id() == layer.id).map(|frame| frame.id().to_string()).collect();
    }
}

/// 🗂️ The frame placed on `layer_id`.
pub fn frame_in_layer(frame: &Frame, layer_id: &str) -> Frame {
    let mut next = frame.clone();
    set_frame_layer(&mut next, layer_id);
    next
}

pub fn set_frame_layer(frame: &mut Frame, layer_id: &str) {
    match frame {
        Frame::Rect { layer_id: slot, .. } | Frame::Text { layer_id: slot, .. } | Frame::Image { layer_id: slot, .. } => *slot = layer_id.to_string(),
    }
}

/// 🔢️ A label number in both locales, `(en, de)`: rounded to two decimals, trailing zeros dropped, German decimal comma.
pub fn layout_label_number(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}

/// 🔠️ A label's counted frames, `(en, de)`: "1 frame" / "1 Rahmen", "3 frames" / "3 Rahmen".
pub fn layout_label_frames(count: usize) -> (String, String) {
    match count {
        1 => ("1 frame".to_string(), "1 Rahmen".to_string()),
        count => (format!("{count} frames"), format!("{count} Rahmen")),
    }
}

/// 🚧️ All fallible layout-engine operations funnel through this — document parsing, scene/hit-test
/// resolution, and export (SVG/PDF/PNG/zip package). Relocated from the deleted `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES); the app engine's scene/export functions
/// (`✏️editor/⚙️engine`) reach it by qualified path — an app depending on its artifact is normal
/// direction, not a layering violation.
#[derive(Debug)]
pub enum LayoutError {
    Json(semio_framework_value::ValueError),
    UnexpectedSchema(String),
    PageNotFound(String),
    Io(std::io::Error),
    Svg(String),
}

impl std::fmt::Display for LayoutError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "json: {error}"),
            Self::UnexpectedSchema(schema) => write!(formatter, "unexpected schema {schema}"),
            Self::PageNotFound(page) => write!(formatter, "page {page} not found"),
            Self::Io(error) => write!(formatter, "io: {error}"),
            Self::Svg(message) => write!(formatter, "svg: {message}"),
        }
    }
}

impl std::error::Error for LayoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::UnexpectedSchema(_) | Self::PageNotFound(_) | Self::Svg(_) => None,
        }
    }
}

impl From<semio_framework_value::ValueError> for LayoutError {
    fn from(error: semio_framework_value::ValueError) -> Self {
        Self::Json(error)
    }
}

impl From<std::io::Error> for LayoutError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub fn drawing_labels(snapshot: &LayoutSnapshot) -> Vec<String> {
    match background_drawing_scene(snapshot) {
        Ok(Some(scene)) => scene.texts().into_iter().map(str::to_string).collect(),
        Ok(None) | Err(_) => Vec::new(),
    }
}

//#region 🪢️TaxonomyMounts
#[path = "💡️inferences/🦀️.rs"]
pub mod inferences;
#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
    #[path = "."]
    pub mod sqlite {
        #[path = "../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
    #[path = "."]
    pub mod text {
        #[path = "../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
}
#[path = "📸️snapshot/🦀️.rs"]
pub mod snapshot;
//#endregion 🪢️TaxonomyMounts

/// 🔑️ Frames key their list deltas by frame id.
impl protocol::list_delta::Keyed for Frame {
    type Key = String;
    fn key(&self) -> String {
        self.id().to_string()
    }
}
