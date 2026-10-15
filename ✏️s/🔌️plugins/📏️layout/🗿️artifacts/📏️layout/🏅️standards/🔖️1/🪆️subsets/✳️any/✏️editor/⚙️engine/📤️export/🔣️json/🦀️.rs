//! 🔣️ Layout export JSON: the bounded JSON validation and typed JSON writing cursors of the layout export job.

use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use semio_s_artifact_layout_layout_model::{Frame, GridSettings, LayoutBounds, Page, PageOverride};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::{Generation, InteractiveStage, RevisionId};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, Operation, StepContext};
use semio_framework_plugin::app::{ArtifactDownloadOutput, ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactReservedToolJob, ArtifactSnapshotCloseLease, ArtifactToolCompletion};
use semio_framework_plugin::{ArtifactReservedJob, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, EphemeralEmit, Fault, MediaClass, MediaForm, MediaType};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawNode, PathSegment};
use std::sync::Arc;
use crate::dictionary::DictionaryJsonCursor;
pub const MAX_LAYOUT_EXPORT_JSON_NODES: usize = 2_048;

pub const MAX_LAYOUT_EXPORT_STRING_BYTES: usize = 8 << 10;

pub const MAX_LAYOUT_EXPORT_DECODED_ITEMS: usize = 131_072;

pub const MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES: usize = 64 << 10;

const JSON_INPUT_BYTES_PER_UNIT: usize = 2_048;

const JSON_OUTPUT_BYTES_PER_UNIT: usize = 1_024;

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonRootState {
    Value,
    Done,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonArrayState {
    FirstValueOrEnd,
    Value,
    CommaOrEnd,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonObjectState {
    FirstKeyOrEnd,
    Key,
    Colon,
    Value,
    CommaOrEnd,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonContainer {
    Array(JsonArrayState),
    Object(JsonObjectState),
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonStringEscape {
    None,
    Escape,
    Unicode { digits: u8, value: u16 },
    LowSlash,
    LowU,
    LowUnicode { digits: u8, value: u16 },
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonNumberState {
    Sign,
    Zero,
    Integer,
    FractionStart,
    Fraction,
    ExponentStart,
    ExponentSign,
    Exponent,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonLiteral {
    True,
    False,
    Null,
}

impl JsonLiteral {
    fn bytes(&self) -> &'static [u8] {
        match self {
            Self::True => b"true",
            Self::False => b"false",
            Self::Null => b"null",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
enum JsonToken {
    None,
    String { key: bool, decoded_bytes: usize, escape: JsonStringEscape },
    Number(JsonNumberState),
    Literal { literal: JsonLiteral, cursor: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
pub struct JsonValidationCursor {
    byte_cursor: usize,
    nodes: usize,
    require_array: bool,
    root: JsonRootState,
    stack: Vec<JsonContainer>,
    token: JsonToken,
}

impl JsonValidationCursor {
    fn new(text: &str, require_array: bool) -> Result<Self, String> {
        if text.len() > MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES {
            return Err("layout-export-json-byte-limit".into());
        }
        Ok(Self { byte_cursor: 0, nodes: 0, require_array, root: JsonRootState::Value, stack: Vec::new(), token: JsonToken::None })
    }

    fn add_node(&mut self) -> Result<(), String> {
        self.nodes = self.nodes.checked_add(1).ok_or("layout-export-json-node-limit")?;
        if self.nodes > MAX_LAYOUT_EXPORT_JSON_NODES || self.stack.len() > 64 {
            return Err("layout-export-json-node-limit".into());
        }
        Ok(())
    }

    fn start_value(&mut self, byte: u8) -> Result<(), String> {
        if self.require_array && self.root == JsonRootState::Value && self.stack.is_empty() && byte != b'[' {
            return Err("layout-export-preflight-schema".into());
        }
        match self.stack.last_mut() {
            Some(JsonContainer::Array(state @ (JsonArrayState::FirstValueOrEnd | JsonArrayState::Value))) => *state = JsonArrayState::CommaOrEnd,
            Some(JsonContainer::Object(state @ JsonObjectState::Value)) => *state = JsonObjectState::CommaOrEnd,
            None if self.root == JsonRootState::Value => self.root = JsonRootState::Done,
            _ => return Err("layout-export-json-structure".into()),
        }
        self.add_node()?;
        match byte {
            b'{' => self.stack.push(JsonContainer::Object(JsonObjectState::FirstKeyOrEnd)),
            b'[' => self.stack.push(JsonContainer::Array(JsonArrayState::FirstValueOrEnd)),
            b'"' => self.token = JsonToken::String { key: false, decoded_bytes: 0, escape: JsonStringEscape::None },
            b'-' => self.token = JsonToken::Number(JsonNumberState::Sign),
            b'0' => self.token = JsonToken::Number(JsonNumberState::Zero),
            b'1'..=b'9' => self.token = JsonToken::Number(JsonNumberState::Integer),
            b't' => self.token = JsonToken::Literal { literal: JsonLiteral::True, cursor: 1 },
            b'f' => self.token = JsonToken::Literal { literal: JsonLiteral::False, cursor: 1 },
            b'n' => self.token = JsonToken::Literal { literal: JsonLiteral::Null, cursor: 1 },
            _ => return Err("layout-export-json-value".into()),
        }
        if self.stack.len() > 64 {
            return Err("layout-export-json-depth-limit".into());
        }
        Ok(())
    }

    fn finish_string(&mut self, key: bool) -> Result<(), String> {
        if key {
            match self.stack.last_mut() {
                Some(JsonContainer::Object(state @ (JsonObjectState::FirstKeyOrEnd | JsonObjectState::Key))) => {
                    *state = JsonObjectState::Colon;
                    Ok(())
                }
                _ => Err("layout-export-json-key".into()),
            }
        } else {
            Ok(())
        }
    }

    fn add_string_bytes(decoded_bytes: &mut usize, bytes: usize) -> Result<(), String> {
        *decoded_bytes = decoded_bytes.checked_add(bytes).ok_or("layout-export-json-string-limit")?;
        if *decoded_bytes > MAX_LAYOUT_EXPORT_STRING_BYTES {
            return Err("layout-export-json-string-limit".into());
        }
        Ok(())
    }

    fn hex(byte: u8) -> Option<u16> {
        match byte {
            b'0'..=b'9' => Some(u16::from(byte - b'0')),
            b'a'..=b'f' => Some(u16::from(byte - b'a' + 10)),
            b'A'..=b'F' => Some(u16::from(byte - b'A' + 10)),
            _ => None,
        }
    }

    fn finish_unicode(decoded_bytes: &mut usize, value: u16, low: bool) -> Result<JsonStringEscape, String> {
        if low {
            if !(0xdc00..=0xdfff).contains(&value) {
                return Err("layout-export-json-unicode".into());
            }
            Self::add_string_bytes(decoded_bytes, 4)?;
            return Ok(JsonStringEscape::None);
        }
        if (0xd800..=0xdbff).contains(&value) {
            return Ok(JsonStringEscape::LowSlash);
        }
        if (0xdc00..=0xdfff).contains(&value) {
            return Err("layout-export-json-unicode".into());
        }
        let scalar = char::from_u32(u32::from(value)).ok_or("layout-export-json-unicode")?;
        Self::add_string_bytes(decoded_bytes, scalar.len_utf8())?;
        Ok(JsonStringEscape::None)
    }

    fn number_delimiter(byte: u8) -> bool {
        byte.is_ascii_whitespace() || matches!(byte, b',' | b']' | b'}')
    }

    fn advance_token(&mut self, bytes: &[u8]) -> Result<bool, String> {
        match &mut self.token {
            JsonToken::None => Ok(false),
            JsonToken::String { key, decoded_bytes, escape } => {
                let byte = *bytes.get(self.byte_cursor).ok_or("layout-export-json-string")?;
                let low = matches!(escape, JsonStringEscape::LowUnicode { .. });
                match escape {
                    JsonStringEscape::None => match byte {
                        b'"' => {
                            let key = *key;
                            self.byte_cursor += 1;
                            self.token = JsonToken::None;
                            self.finish_string(key)?;
                        }
                        b'\\' => {
                            self.byte_cursor += 1;
                            *escape = JsonStringEscape::Escape;
                        }
                        0..=0x1f => return Err("layout-export-json-string".into()),
                        _ => {
                            self.byte_cursor += 1;
                            Self::add_string_bytes(decoded_bytes, 1)?;
                        }
                    },
                    JsonStringEscape::Escape => {
                        self.byte_cursor += 1;
                        match byte {
                            b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => {
                                Self::add_string_bytes(decoded_bytes, 1)?;
                                *escape = JsonStringEscape::None;
                            }
                            b'u' => *escape = JsonStringEscape::Unicode { digits: 0, value: 0 },
                            _ => return Err("layout-export-json-escape".into()),
                        }
                    }
                    JsonStringEscape::Unicode { digits, value } | JsonStringEscape::LowUnicode { digits, value } => {
                        let digit = Self::hex(byte).ok_or("layout-export-json-unicode")?;
                        self.byte_cursor += 1;
                        *value = (*value << 4) | digit;
                        *digits += 1;
                        if *digits == 4 {
                            *escape = Self::finish_unicode(decoded_bytes, *value, low)?;
                        }
                    }
                    JsonStringEscape::LowSlash => {
                        if byte != b'\\' {
                            return Err("layout-export-json-unicode".into());
                        }
                        self.byte_cursor += 1;
                        *escape = JsonStringEscape::LowU;
                    }
                    JsonStringEscape::LowU => {
                        if byte != b'u' {
                            return Err("layout-export-json-unicode".into());
                        }
                        self.byte_cursor += 1;
                        *escape = JsonStringEscape::LowUnicode { digits: 0, value: 0 };
                    }
                }
                Ok(true)
            }
            JsonToken::Literal { literal, cursor } => {
                let expected = literal.bytes();
                let byte = *bytes.get(self.byte_cursor).ok_or("layout-export-json-literal")?;
                if expected.get(*cursor).copied() != Some(byte) {
                    return Err("layout-export-json-literal".into());
                }
                self.byte_cursor += 1;
                *cursor += 1;
                if *cursor == expected.len() {
                    self.token = JsonToken::None;
                }
                Ok(true)
            }
            JsonToken::Number(state) => {
                let Some(byte) = bytes.get(self.byte_cursor).copied() else {
                    return Ok(false);
                };
                let next = match *state {
                    JsonNumberState::Sign => match byte {
                        b'0' => JsonNumberState::Zero,
                        b'1'..=b'9' => JsonNumberState::Integer,
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::Zero => match byte {
                        b'.' => JsonNumberState::FractionStart,
                        b'e' | b'E' => JsonNumberState::ExponentStart,
                        _ if Self::number_delimiter(byte) => {
                            self.token = JsonToken::None;
                            return Ok(false);
                        }
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::Integer => match byte {
                        b'0'..=b'9' => JsonNumberState::Integer,
                        b'.' => JsonNumberState::FractionStart,
                        b'e' | b'E' => JsonNumberState::ExponentStart,
                        _ if Self::number_delimiter(byte) => {
                            self.token = JsonToken::None;
                            return Ok(false);
                        }
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::FractionStart => match byte {
                        b'0'..=b'9' => JsonNumberState::Fraction,
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::Fraction => match byte {
                        b'0'..=b'9' => JsonNumberState::Fraction,
                        b'e' | b'E' => JsonNumberState::ExponentStart,
                        _ if Self::number_delimiter(byte) => {
                            self.token = JsonToken::None;
                            return Ok(false);
                        }
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::ExponentStart => match byte {
                        b'+' | b'-' => JsonNumberState::ExponentSign,
                        b'0'..=b'9' => JsonNumberState::Exponent,
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::ExponentSign => match byte {
                        b'0'..=b'9' => JsonNumberState::Exponent,
                        _ => return Err("layout-export-json-number".into()),
                    },
                    JsonNumberState::Exponent => match byte {
                        b'0'..=b'9' => JsonNumberState::Exponent,
                        _ if Self::number_delimiter(byte) => {
                            self.token = JsonToken::None;
                            return Ok(false);
                        }
                        _ => return Err("layout-export-json-number".into()),
                    },
                };
                *state = next;
                self.byte_cursor += 1;
                Ok(true)
            }
        }
    }

    fn start_key(&mut self) {
        self.token = JsonToken::String { key: true, decoded_bytes: 0, escape: JsonStringEscape::None };
    }

    fn close_container(&mut self) -> Result<(), String> {
        self.stack.pop().ok_or_else(|| "layout-export-json-structure".into()).map(|_| ())
    }

    fn advance(&mut self, text: &str) -> Result<bool, String> {
        let bytes = text.as_bytes();
        let end = self.byte_cursor.saturating_add(JSON_INPUT_BYTES_PER_UNIT).min(bytes.len());
        while self.byte_cursor < end {
            if !matches!(self.token, JsonToken::None) && self.advance_token(bytes)? {
                continue;
            }
            let byte = bytes[self.byte_cursor];
            if byte.is_ascii_whitespace() {
                self.byte_cursor += 1;
                continue;
            }
            match self.stack.last().cloned() {
                None => match self.root {
                    JsonRootState::Value => {
                        self.byte_cursor += 1;
                        self.start_value(byte)?;
                    }
                    JsonRootState::Done => return Err("layout-export-json-trailing".into()),
                },
                Some(JsonContainer::Array(JsonArrayState::FirstValueOrEnd)) => {
                    if byte == b']' {
                        self.byte_cursor += 1;
                        self.close_container()?;
                    } else {
                        self.byte_cursor += 1;
                        self.start_value(byte)?;
                    }
                }
                Some(JsonContainer::Array(JsonArrayState::Value)) => {
                    self.byte_cursor += 1;
                    self.start_value(byte)?;
                }
                Some(JsonContainer::Array(JsonArrayState::CommaOrEnd)) => match byte {
                    b',' => {
                        self.byte_cursor += 1;
                        if let Some(JsonContainer::Array(state)) = self.stack.last_mut() {
                            *state = JsonArrayState::Value;
                        }
                    }
                    b']' => {
                        self.byte_cursor += 1;
                        self.close_container()?;
                    }
                    _ => return Err("layout-export-json-array".into()),
                },
                Some(JsonContainer::Object(JsonObjectState::FirstKeyOrEnd)) => {
                    if byte == b'}' {
                        self.byte_cursor += 1;
                        self.close_container()?;
                    } else if byte == b'"' {
                        self.byte_cursor += 1;
                        self.start_key();
                    } else {
                        return Err("layout-export-json-key".into());
                    }
                }
                Some(JsonContainer::Object(JsonObjectState::Key)) => {
                    if byte != b'"' {
                        return Err("layout-export-json-key".into());
                    }
                    self.byte_cursor += 1;
                    self.start_key();
                }
                Some(JsonContainer::Object(JsonObjectState::Colon)) => {
                    if byte != b':' {
                        return Err("layout-export-json-colon".into());
                    }
                    self.byte_cursor += 1;
                    if let Some(JsonContainer::Object(state)) = self.stack.last_mut() {
                        *state = JsonObjectState::Value;
                    }
                }
                Some(JsonContainer::Object(JsonObjectState::Value)) => {
                    self.byte_cursor += 1;
                    self.start_value(byte)?;
                }
                Some(JsonContainer::Object(JsonObjectState::CommaOrEnd)) => match byte {
                    b',' => {
                        self.byte_cursor += 1;
                        if let Some(JsonContainer::Object(state)) = self.stack.last_mut() {
                            *state = JsonObjectState::Key;
                        }
                    }
                    b'}' => {
                        self.byte_cursor += 1;
                        self.close_container()?;
                    }
                    _ => return Err("layout-export-json-object".into()),
                },
            }
        }
        if self.byte_cursor != bytes.len() {
            return Ok(false);
        }
        match &self.token {
            JsonToken::None => {}
            JsonToken::Number(JsonNumberState::Zero | JsonNumberState::Integer | JsonNumberState::Fraction | JsonNumberState::Exponent) => self.token = JsonToken::None,
            _ => return Err("layout-export-json-incomplete".into()),
        }
        if self.stack.is_empty() && self.root == JsonRootState::Done {
            Ok(true)
        } else {
            Err("layout-export-json-incomplete".into())
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum TopArrayKind {
    ParagraphStyles,
    CharacterStyles,
    Stories,
    Links,
    ParentPages,
    Spreads,
    Pages,
}

#[derive(Clone, Copy, Debug)]
enum RecordOwner {
    Page(usize),
    Parent(usize),
}

#[derive(Clone, Copy, Debug)]
enum StringArraySource {
    SpreadPage(usize),
    PageLayer(usize),
    ParentLayer(usize),
    LayerObject { owner: RecordOwner, layer: usize },
}

#[derive(Clone, Debug)]
pub enum StringSource {
    DocumentSchema,
    DocumentName,
    PrintTarget,
    StringArrayElement { source: StringArraySource, index: usize },
    ParagraphId(usize),
    ParagraphName(usize),
    ParagraphFontFamily(usize),
    ParagraphAlignment(usize),
    CharacterId(usize),
    CharacterName(usize),
    CharacterFontFamily(usize),
    StoryId(usize),
    StoryContent(usize),
    StoryRunParagraph { story: usize, run: usize },
    StoryRunCharacter { story: usize, run: usize },
    LinkId(usize),
    LinkPath(usize),
    LinkHash(usize),
    LinkColorProfile(usize),
    LinkState(usize),
    LinkProxy(usize),
    ParentId(usize),
    ParentName(usize),
    SpreadId(usize),
    SpreadName(usize),
    PageId(usize),
    PageName(usize),
    PageSpreadId(usize),
    PageParentId(usize),
    LayerId { owner: RecordOwner, layer: usize },
    LayerName { owner: RecordOwner, layer: usize },
    FrameId { owner: RecordOwner, frame: usize },
    FrameLayerId { owner: RecordOwner, frame: usize },
    FrameStoryId { owner: RecordOwner, frame: usize },
    FrameThreadNext { owner: RecordOwner, frame: usize },
    FrameWrapMode { owner: RecordOwner, frame: usize },
    FrameLinkId { owner: RecordOwner, frame: usize },
    OverrideObjectId { page: usize, value: usize },
    ChildId,
    ChildTargetArtifactId,
    ChildTargetArtifactKind,
    ChildTargetStandard,
    ChildTargetSubset,
    DrawingSchema,
    DrawStyleName(usize),
    DrawLayerId(usize),
    DrawLayerName(usize),
    DrawNodeValue { layer: usize, path: Vec<usize> },
    DrawNodeStyle { layer: usize, path: Vec<usize> },
    DrawNodeMime { layer: usize, path: Vec<usize> },
    ReferencedTargetArtifactId,
    ReferencedTargetArtifactKind,
    ReferencedTargetStandard,
    ReferencedTargetSubset,
    ReferencedRole,
    ReferencedCheckpointId,
    ReferencedBlobHash,
    ReferencedBlobMediaType,
}

#[derive(Clone, Debug, Default)]
struct JsonStringWriteCursor {
    byte_cursor: usize,
    opened: bool,
    closed: bool,
}

#[derive(Clone, Debug)]
pub enum TypedJsonNode {
    FragmentStart,
    FragmentEnd,
    Static { bytes: &'static [u8], cursor: usize },
    Scalar { bytes: Vec<u8>, cursor: usize },
    String { source: StringSource, cursor: JsonStringWriteCursor },
    OwnedString { value: String, cursor: JsonStringWriteCursor },
    Document,
    Dictionary(DictionaryJsonCursor),
    Grid,
    TopArray { kind: TopArrayKind, index: usize, opened: bool },
    Paragraph(usize),
    Character(usize),
    Story(usize),
    StoryRuns { story: usize, index: usize, opened: bool },
    StoryRun { story: usize, run: usize },
    Link(usize),
    Parent(usize),
    Spread(usize),
    Page(usize),
    StringArray { source: StringArraySource, index: usize, opened: bool },
    Layers { owner: RecordOwner, index: usize, opened: bool },
    Layer { owner: RecordOwner, layer: usize },
    Frames { owner: RecordOwner, index: usize, opened: bool },
    Frame { owner: RecordOwner, frame: usize },
    Bounds { owner: RecordOwner, frame: usize },
    Rect { page: usize, guide: usize },
    TextInset { owner: RecordOwner, frame: usize },
    Margins(usize),
    Columns(usize),
    Guides { page: usize, index: usize, opened: bool },
    Overrides { page: usize, index: usize, opened: bool },
    Override { page: usize, value: usize },
    OverrideBounds { page: usize, value: usize },
    BackgroundDrawing,
    ChildHandle,
    ArtifactRef { referenced: bool },
    Dialect { referenced: bool },
    Drawing,
    DrawCanvas,
    DrawRgba(SemioRgba),
    DrawStyles { index: usize, opened: bool },
    DrawStyle(usize),
    DrawLayers { index: usize, opened: bool },
    DrawLayer(usize),
    DrawNode { layer: usize, path: Vec<usize> },
    DrawChildren { layer: usize, path: Vec<usize>, index: usize, opened: bool },
    DrawSegments { layer: usize, path: Vec<usize>, index: usize, opened: bool },
    DrawSegment { layer: usize, path: Vec<usize>, segment: usize },
    DrawPoint2(SemioPoint2),
    DrawPoint3(SemioPoint3),
    DrawQuaternion(SemioQuaternion),
    DrawTransform(SemioTransform),
    DrawBytes { layer: usize, path: Vec<usize>, index: usize, opened: bool },
    ReferencedModel,
    LinkPin,
    BlobRef,
    MissingLinks { index: usize, opened: bool, emitted: bool },
    MissingLink(usize),
    Manifest,
    ManifestLinks { index: usize, opened: bool },
    ManifestLink(usize),
}

#[derive(Clone, Debug, Default)]
pub struct TypedJsonCursor {
    stack: Vec<TypedJsonNode>,
    emitted_nodes: usize,
    fragment_bytes: Option<usize>,
}

fn typed_string_source_owned_bytes(source: &StringSource) -> usize {
    match source {
        StringSource::DrawNodeValue { path, .. } | StringSource::DrawNodeStyle { path, .. } | StringSource::DrawNodeMime { path, .. } => path.len().saturating_mul(size_of::<usize>()),
        _ => 0,
    }
}

pub fn typed_json_node_owned_bytes(node: &TypedJsonNode) -> usize {
    match node {
        TypedJsonNode::Scalar { bytes, .. } => bytes.len(),
        TypedJsonNode::Dictionary(cursor) => cursor.owned_bytes(),
        TypedJsonNode::String { source, .. } => typed_string_source_owned_bytes(source),
        TypedJsonNode::OwnedString { value, .. } => value.len(),
        TypedJsonNode::DrawNode { path, .. } | TypedJsonNode::DrawChildren { path, .. } | TypedJsonNode::DrawSegments { path, .. } | TypedJsonNode::DrawSegment { path, .. } | TypedJsonNode::DrawBytes { path, .. } => {
            path.len().saturating_mul(size_of::<usize>())
        }
        _ => 0,
    }
}

pub trait JsonScalar {
    fn json_bytes(&self) -> Result<Vec<u8>, String>;
}

macro_rules! integer_json_scalar {
    ($($scalar:ty),* $(,)?) => {
        $(impl JsonScalar for $scalar {
            fn json_bytes(&self) -> Result<Vec<u8>, String> {
                Ok(self.to_string().into_bytes())
            }
        })*
    };
}

pub fn finite_json_number(value: f64) -> Result<Vec<u8>, String> {
    if !value.is_finite() {
        return Err("layout-export-json-number".into());
    }
    let mut text = value.to_string();
    if !text.contains('.') && !text.contains('e') && !text.contains('E') {
        text.push_str(".0");
    }
    Ok(text.into_bytes())
}

impl TypedJsonCursor {
    fn document() -> Self {
        Self { stack: vec![TypedJsonNode::Document], emitted_nodes: 0, fragment_bytes: None }
    }

    fn preflight() -> Self {
        Self { stack: vec![TypedJsonNode::MissingLinks { index: 0, opened: false, emitted: false }], emitted_nodes: 0, fragment_bytes: None }
    }

    fn manifest() -> Self {
        Self { stack: vec![TypedJsonNode::Manifest], emitted_nodes: 0, fragment_bytes: None }
    }

    fn validating(node: TypedJsonNode) -> Self {
        Self { stack: Self::fragment(node).into_iter().rev().collect(), emitted_nodes: 0, fragment_bytes: None }
    }

    fn static_node(bytes: &'static [u8]) -> TypedJsonNode {
        TypedJsonNode::Static { bytes, cursor: 0 }
    }

    fn scalar<T: JsonScalar + ?Sized>(value: &T) -> Result<TypedJsonNode, String> {
        let bytes = value.json_bytes()?;
        if bytes.len() > 256 {
            return Err("layout-export-json-scalar-limit".into());
        }
        Ok(TypedJsonNode::Scalar { bytes, cursor: 0 })
    }

    fn string(source: StringSource) -> TypedJsonNode {
        TypedJsonNode::String { source, cursor: JsonStringWriteCursor::default() }
    }

    fn owned_string(value: String) -> TypedJsonNode {
        TypedJsonNode::OwnedString { value, cursor: JsonStringWriteCursor::default() }
    }

    fn push_sequence(&mut self, nodes: Vec<TypedJsonNode>) {
        self.stack.extend(nodes.into_iter().rev());
    }

    fn fragment(node: TypedJsonNode) -> Vec<TypedJsonNode> {
        vec![TypedJsonNode::FragmentStart, node, TypedJsonNode::FragmentEnd]
    }

    fn owner_page<'a>(snapshot: &'a LayoutSnapshot, owner: RecordOwner) -> Result<OwnerPage<'a>, String> {
        match owner {
            RecordOwner::Page(index) => snapshot.pages.get(index).map(OwnerPage::Page),
            RecordOwner::Parent(index) => snapshot.parent_pages.get(index).map(OwnerPage::Parent),
        }
        .ok_or_else(|| "layout-export-json-index".into())
    }

    fn draw_node<'a>(snapshot: &'a LayoutSnapshot, layer: usize, path: &[usize]) -> Result<&'a DrawNode, String> {
        let drawing = &snapshot.background_drawing.as_ref().ok_or("layout-export-json-background")?.content;
        let mut node = &drawing.layers.get(layer).ok_or("layout-export-json-draw-layer")?.root;
        for index in path {
            node = match node {
                DrawNode::Group { children, .. } => children.get(*index).ok_or("layout-export-json-draw-child")?,
                _ => return Err("layout-export-json-draw-path".into()),
            };
        }
        Ok(node)
    }

    fn resolve_string<'a>(source: &StringSource, snapshot: &'a LayoutSnapshot) -> Result<&'a str, String> {
        let missing = || "layout-export-json-string-source".to_string();
        Ok(match source {
            StringSource::DocumentSchema => &snapshot.schema,
            StringSource::DocumentName => &snapshot.name,
            StringSource::PrintTarget => snapshot.print_target.as_deref().ok_or_else(missing)?,
            StringSource::StringArrayElement { source, index } => match source {
                StringArraySource::SpreadPage(owner) => snapshot.spreads.get(*owner).and_then(|value| value.page_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
                StringArraySource::PageLayer(owner) => snapshot.pages.get(*owner).and_then(|value| value.layer_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
                StringArraySource::ParentLayer(owner) => snapshot.parent_pages.get(*owner).and_then(|value| value.layer_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
                StringArraySource::LayerObject { owner, layer } => Self::owner_page(snapshot, *owner)?.layers().get(*layer).and_then(|value| value.object_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
            },
            StringSource::ParagraphId(index) => &snapshot.paragraph_styles.get(*index).ok_or_else(missing)?.id,
            StringSource::ParagraphName(index) => &snapshot.paragraph_styles.get(*index).ok_or_else(missing)?.name,
            StringSource::ParagraphFontFamily(index) => &snapshot.paragraph_styles.get(*index).ok_or_else(missing)?.font_family,
            StringSource::ParagraphAlignment(index) => &snapshot.paragraph_styles.get(*index).ok_or_else(missing)?.alignment,
            StringSource::CharacterId(index) => &snapshot.character_styles.get(*index).ok_or_else(missing)?.id,
            StringSource::CharacterName(index) => snapshot.character_styles.get(*index).and_then(|value| value.name.as_deref()).ok_or_else(missing)?,
            StringSource::CharacterFontFamily(index) => snapshot.character_styles.get(*index).and_then(|value| value.font_family.as_deref()).ok_or_else(missing)?,
            StringSource::StoryId(index) => &snapshot.stories.get(*index).ok_or_else(missing)?.id,
            StringSource::StoryContent(index) => &snapshot.stories.get(*index).ok_or_else(missing)?.content,
            StringSource::StoryRunParagraph { story, run } => snapshot.stories.get(*story).and_then(|value| value.style_runs.get(*run)).and_then(|value| value.paragraph_style_id.as_deref()).ok_or_else(missing)?,
            StringSource::StoryRunCharacter { story, run } => snapshot.stories.get(*story).and_then(|value| value.style_runs.get(*run)).and_then(|value| value.character_style_id.as_deref()).ok_or_else(missing)?,
            StringSource::LinkId(index) => &snapshot.links.get(*index).ok_or_else(missing)?.id,
            StringSource::LinkPath(index) => &snapshot.links.get(*index).ok_or_else(missing)?.path,
            StringSource::LinkHash(index) => &snapshot.links.get(*index).ok_or_else(missing)?.hash,
            StringSource::LinkColorProfile(index) => snapshot.links.get(*index).and_then(|value| value.color_profile.as_deref()).ok_or_else(missing)?,
            StringSource::LinkState(index) => snapshot.links.get(*index).and_then(|value| value.state.as_deref()).ok_or_else(missing)?,
            StringSource::LinkProxy(index) => snapshot.links.get(*index).and_then(|value| value.proxy_data_url.as_deref()).ok_or_else(missing)?,
            StringSource::ParentId(index) => &snapshot.parent_pages.get(*index).ok_or_else(missing)?.id,
            StringSource::ParentName(index) => &snapshot.parent_pages.get(*index).ok_or_else(missing)?.name,
            StringSource::SpreadId(index) => &snapshot.spreads.get(*index).ok_or_else(missing)?.id,
            StringSource::SpreadName(index) => &snapshot.spreads.get(*index).ok_or_else(missing)?.name,
            StringSource::PageId(index) => &snapshot.pages.get(*index).ok_or_else(missing)?.id,
            StringSource::PageName(index) => &snapshot.pages.get(*index).ok_or_else(missing)?.name,
            StringSource::PageSpreadId(index) => &snapshot.pages.get(*index).ok_or_else(missing)?.spread_id,
            StringSource::PageParentId(index) => snapshot.pages.get(*index).and_then(|value| value.parent_page_id.as_deref()).ok_or_else(missing)?,
            StringSource::LayerId { owner, layer } => Self::owner_page(snapshot, *owner)?.layers().get(*layer).map(|value| value.id.as_str()).ok_or_else(missing)?,
            StringSource::LayerName { owner, layer } => Self::owner_page(snapshot, *owner)?.layers().get(*layer).map(|value| value.name.as_str()).ok_or_else(missing)?,
            StringSource::FrameId { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::Id).ok_or_else(missing)?,
            StringSource::FrameLayerId { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::LayerId).ok_or_else(missing)?,
            StringSource::FrameStoryId { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::StoryId).ok_or_else(missing)?,
            StringSource::FrameThreadNext { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::ThreadNext).ok_or_else(missing)?,
            StringSource::FrameWrapMode { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::WrapMode).ok_or_else(missing)?,
            StringSource::FrameLinkId { owner, frame } => frame_string(Self::owner_page(snapshot, *owner)?.frames().get(*frame).ok_or_else(missing)?, FrameStringField::LinkId).ok_or_else(missing)?,
            StringSource::OverrideObjectId { page, value } => &snapshot.pages.get(*page).and_then(|page| page.overrides.get(*value)).ok_or_else(missing)?.object_id,
            StringSource::ChildId => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.handle.child_id,
            StringSource::ChildTargetArtifactId => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.handle.target.artifact_id,
            StringSource::ChildTargetArtifactKind => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.handle.target.dialect.artifact_kind,
            StringSource::ChildTargetStandard => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.handle.target.dialect.standard,
            StringSource::ChildTargetSubset => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.handle.target.dialect.subset,
            StringSource::DrawingSchema => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.content.schema,
            StringSource::DrawStyleName(index) => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.content.styles.get(*index).ok_or_else(missing)?.name,
            StringSource::DrawLayerId(index) => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.content.layers.get(*index).ok_or_else(missing)?.id,
            StringSource::DrawLayerName(index) => &snapshot.background_drawing.as_ref().ok_or_else(missing)?.content.layers.get(*index).ok_or_else(missing)?.name,
            StringSource::DrawNodeValue { layer, path } => match Self::draw_node(snapshot, *layer, path)? {
                DrawNode::Text { value, .. } => value,
                _ => return Err(missing()),
            },
            StringSource::DrawNodeStyle { layer, path } => match Self::draw_node(snapshot, *layer, path)? {
                DrawNode::Path { style: Some(value), .. } | DrawNode::Text { style: Some(value), .. } => value,
                _ => return Err(missing()),
            },
            StringSource::DrawNodeMime { layer, path } => match Self::draw_node(snapshot, *layer, path)? {
                DrawNode::Image { mime, .. } => mime,
                _ => return Err(missing()),
            },
            StringSource::ReferencedTargetArtifactId => &snapshot.referenced_model.as_ref().ok_or_else(missing)?.target.artifact_id,
            StringSource::ReferencedTargetArtifactKind => &snapshot.referenced_model.as_ref().ok_or_else(missing)?.target.dialect.artifact_kind,
            StringSource::ReferencedTargetStandard => &snapshot.referenced_model.as_ref().ok_or_else(missing)?.target.dialect.standard,
            StringSource::ReferencedTargetSubset => &snapshot.referenced_model.as_ref().ok_or_else(missing)?.target.dialect.subset,
            StringSource::ReferencedRole => &snapshot.referenced_model.as_ref().ok_or_else(missing)?.role,
            StringSource::ReferencedCheckpointId => match &snapshot.referenced_model.as_ref().ok_or_else(missing)?.pin {
                store::LinkPin::Checkpoint { id } => id,
                _ => return Err(missing()),
            },
            StringSource::ReferencedBlobHash => match &snapshot.referenced_model.as_ref().ok_or_else(missing)?.pin {
                store::LinkPin::Snapshot { blob } => &blob.hash,
                _ => return Err(missing()),
            },
            StringSource::ReferencedBlobMediaType => match &snapshot.referenced_model.as_ref().ok_or_else(missing)?.pin {
                store::LinkPin::Snapshot { blob } => &blob.media_type,
                _ => return Err(missing()),
            },
        })
    }

    fn escaped_string_step(value: &str, cursor: &mut JsonStringWriteCursor) -> (Vec<u8>, bool) {
        let mut output = Vec::with_capacity(JSON_OUTPUT_BYTES_PER_UNIT);
        if !cursor.opened {
            output.push(b'"');
            cursor.opened = true;
        }
        let bytes = value.as_bytes();
        while cursor.byte_cursor < bytes.len() {
            let byte = bytes[cursor.byte_cursor];
            let escaped: &[u8] = match byte {
                b'"' => b"\\\"",
                b'\\' => b"\\\\",
                b'\x08' => b"\\b",
                b'\t' => b"\\t",
                b'\n' => b"\\n",
                b'\x0c' => b"\\f",
                b'\r' => b"\\r",
                0..=0x1f => &[],
                _ => std::slice::from_ref(&bytes[cursor.byte_cursor]),
            };
            let needed = if byte <= 0x1f && escaped.is_empty() { 6 } else { escaped.len() };
            if output.len() + needed + 1 > JSON_OUTPUT_BYTES_PER_UNIT {
                break;
            }
            if byte <= 0x1f && escaped.is_empty() {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                output.extend_from_slice(&[b'\\', b'u', b'0', b'0', HEX[(byte >> 4) as usize], HEX[(byte & 15) as usize]]);
            } else {
                output.extend_from_slice(escaped);
            }
            cursor.byte_cursor += 1;
        }
        if cursor.byte_cursor == bytes.len() && !cursor.closed {
            output.push(b'"');
            cursor.closed = true;
        }
        (output, cursor.closed)
    }

    fn optional_string(present: bool, source: StringSource) -> TypedJsonNode {
        if present {
            Self::string(source)
        } else {
            Self::static_node(b"null")
        }
    }

    fn account_output(&mut self, bytes: usize) -> Result<(), String> {
        if let Some(fragment_bytes) = &mut self.fragment_bytes {
            *fragment_bytes = fragment_bytes.checked_add(bytes).ok_or("layout-export-json-fragment-limit")?;
            if *fragment_bytes > MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES {
                return Err("layout-export-json-fragment-limit".into());
            }
        }
        Ok(())
    }

    fn account_node(&mut self) -> Result<(), String> {
        self.emitted_nodes = self.emitted_nodes.checked_add(1).ok_or("layout-export-json-node-limit")?;
        let maximum = if self.fragment_bytes.is_some() { MAX_LAYOUT_EXPORT_JSON_NODES } else { MAX_LAYOUT_EXPORT_DECODED_ITEMS };
        if self.emitted_nodes > maximum {
            return Err("layout-export-json-node-limit".into());
        }
        Ok(())
    }

    fn advance(&mut self, snapshot: &LayoutSnapshot) -> Result<(Vec<u8>, bool), String> {
        let Some(node) = self.stack.pop() else { return Ok((Vec::new(), true)) };
        let mut output = Vec::new();
        match node {
            TypedJsonNode::FragmentStart => {
                if self.fragment_bytes.replace(0).is_some() {
                    return Err("layout-export-json-fragment-nesting".into());
                }
                self.emitted_nodes = 0;
            }
            TypedJsonNode::FragmentEnd => {
                if self.fragment_bytes.take().is_none() {
                    return Err("layout-export-json-fragment-state".into());
                }
            }
            TypedJsonNode::Static { bytes, cursor } => {
                let end = cursor.saturating_add(JSON_OUTPUT_BYTES_PER_UNIT).min(bytes.len());
                output.extend_from_slice(&bytes[cursor..end]);
                if end < bytes.len() {
                    self.stack.push(TypedJsonNode::Static { bytes, cursor: end });
                }
            }
            TypedJsonNode::Scalar { bytes, cursor } => {
                self.account_node()?;
                let end = cursor.saturating_add(JSON_OUTPUT_BYTES_PER_UNIT).min(bytes.len());
                output.extend_from_slice(&bytes[cursor..end]);
                if end < bytes.len() {
                    self.stack.push(TypedJsonNode::Scalar { bytes, cursor: end });
                }
            }
            TypedJsonNode::String { source, mut cursor } => {
                self.account_node()?;
                let maximum = MAX_LAYOUT_EXPORT_STRING_BYTES;
                let value = Self::resolve_string(&source, snapshot)?;
                if value.len() > maximum {
                    return Err("layout-export-json-string-limit".into());
                }
                let (bytes, done) = Self::escaped_string_step(value, &mut cursor);
                output = bytes;
                if !done {
                    self.stack.push(TypedJsonNode::String { source, cursor });
                }
            }
            TypedJsonNode::Dictionary(mut cursor) => {
                let (bytes, done) = cursor.advance(snapshot)?;
                output = bytes;
                if !done { self.stack.push(TypedJsonNode::Dictionary(cursor)); }
            }
            TypedJsonNode::OwnedString { value, mut cursor } => {
                self.account_node()?;
                if value.len() > MAX_LAYOUT_EXPORT_STRING_BYTES {
                    return Err("layout-export-json-string-limit".into());
                }
                let (bytes, done) = Self::escaped_string_step(&value, &mut cursor);
                output = bytes;
                if !done {
                    self.stack.push(TypedJsonNode::OwnedString { value, cursor });
                }
            }
            node => {
                self.account_node()?;
                self.expand(node, snapshot)?;
            }
        }
        self.account_output(output.len())?;
        Ok((output, self.stack.is_empty()))
    }

    fn expand(&mut self, node: TypedJsonNode, snapshot: &LayoutSnapshot) -> Result<(), String> {
        match node {
            TypedJsonNode::Document => {
                let mut nodes = vec![
                    Self::static_node(b"{\"schema\":"),
                    Self::string(StringSource::DocumentSchema),
                    Self::static_node(b",\"name\":"),
                    Self::string(StringSource::DocumentName),
                    Self::static_node(b",\"grid\":"),
                    TypedJsonNode::Grid,
                    Self::static_node(b",\"paragraphStyles\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::ParagraphStyles, index: 0, opened: false },
                    Self::static_node(b",\"characterStyles\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::CharacterStyles, index: 0, opened: false },
                    Self::static_node(b",\"stories\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::Stories, index: 0, opened: false },
                    Self::static_node(b",\"links\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::Links, index: 0, opened: false },
                    Self::static_node(b",\"parentPages\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::ParentPages, index: 0, opened: false },
                    Self::static_node(b",\"spreads\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::Spreads, index: 0, opened: false },
                    Self::static_node(b",\"pages\":"),
                    TypedJsonNode::TopArray { kind: TopArrayKind::Pages, index: 0, opened: false },
                    Self::static_node(b",\"printTarget\":"),
                    Self::optional_string(snapshot.print_target.is_some(), StringSource::PrintTarget),
                ];
                if snapshot.data_fields.is_some() {
                    nodes.extend([Self::static_node(b",\"dataFields\":"), TypedJsonNode::Dictionary(DictionaryJsonCursor::new())]);
                }
                if snapshot.background_drawing.is_some() {
                    nodes.extend([Self::static_node(b",\"backgroundDrawing\":"), TypedJsonNode::BackgroundDrawing]);
                }
                if snapshot.referenced_model.is_some() {
                    nodes.extend([Self::static_node(b",\"referencedModel\":"), TypedJsonNode::ReferencedModel]);
                }
                nodes.push(Self::static_node(b"}"));
                self.push_sequence(nodes);
            }
            TypedJsonNode::Grid => self.push_sequence(vec![
                Self::static_node(b"{\"baselineGrid\":"),
                Self::scalar(&snapshot.grid.baseline_grid)?,
                Self::static_node(b",\"baselineOffset\":"),
                Self::scalar(&snapshot.grid.baseline_offset)?,
                Self::static_node(b",\"snapToBaseline\":"),
                Self::scalar(&snapshot.grid.snap_to_baseline)?,
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::TopArray { kind, index, opened } => {
                let len = match kind {
                    TopArrayKind::ParagraphStyles => snapshot.paragraph_styles.len(),
                    TopArrayKind::CharacterStyles => snapshot.character_styles.len(),
                    TopArrayKind::Stories => snapshot.stories.len(),
                    TopArrayKind::Links => snapshot.links.len(),
                    TopArrayKind::ParentPages => snapshot.parent_pages.len(),
                    TopArrayKind::Spreads => snapshot.spreads.len(),
                    TopArrayKind::Pages => snapshot.pages.len(),
                };
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::TopArray { kind, index, opened: true }]);
                } else if index < len {
                    let item = match kind {
                        TopArrayKind::ParagraphStyles => TypedJsonNode::Paragraph(index),
                        TopArrayKind::CharacterStyles => TypedJsonNode::Character(index),
                        TopArrayKind::Stories => TypedJsonNode::Story(index),
                        TopArrayKind::Links => TypedJsonNode::Link(index),
                        TopArrayKind::ParentPages => TypedJsonNode::Parent(index),
                        TopArrayKind::Spreads => TypedJsonNode::Spread(index),
                        TopArrayKind::Pages => TypedJsonNode::Page(index),
                    };
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, item, TypedJsonNode::TopArray { kind, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::Paragraph(index) => {
                let value = snapshot.paragraph_styles.get(index).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::ParagraphId(index)),
                    Self::static_node(b",\"name\":"),
                    Self::string(StringSource::ParagraphName(index)),
                    Self::static_node(b",\"fontFamily\":"),
                    Self::string(StringSource::ParagraphFontFamily(index)),
                    Self::static_node(b",\"fontSize\":"),
                    Self::scalar(&value.font_size)?,
                    Self::static_node(b",\"fontWeight\":"),
                    Self::scalar(&value.font_weight)?,
                    Self::static_node(b",\"leading\":"),
                    Self::scalar(&value.leading)?,
                    Self::static_node(b",\"tracking\":"),
                    Self::scalar(&value.tracking)?,
                    Self::static_node(b",\"alignment\":"),
                    Self::string(StringSource::ParagraphAlignment(index)),
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Character(index) => {
                let value = snapshot.character_styles.get(index).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::CharacterId(index)),
                    Self::static_node(b",\"name\":"),
                    Self::optional_string(value.name.is_some(), StringSource::CharacterName(index)),
                    Self::static_node(b",\"fontFamily\":"),
                    Self::optional_string(value.font_family.is_some(), StringSource::CharacterFontFamily(index)),
                    Self::static_node(b",\"fontSize\":"),
                    Self::scalar(&value.font_size)?,
                    Self::static_node(b",\"fontWeight\":"),
                    Self::scalar(&value.font_weight)?,
                    Self::static_node(b",\"italic\":"),
                    Self::scalar(&value.italic)?,
                    Self::static_node(b",\"color\":"),
                    Self::scalar(&value.color)?,
                    Self::static_node(b",\"tracking\":"),
                    Self::scalar(&value.tracking)?,
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Story(index) => self.push_sequence(vec![
                Self::static_node(b"{\"id\":"),
                Self::string(StringSource::StoryId(index)),
                Self::static_node(b",\"content\":"),
                Self::string(StringSource::StoryContent(index)),
                Self::static_node(b",\"styleRuns\":"),
                TypedJsonNode::StoryRuns { story: index, index: 0, opened: false },
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::StoryRuns { story, index, opened } => {
                let runs = &snapshot.stories.get(story).ok_or("layout-export-json-index")?.style_runs;
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::StoryRuns { story, index, opened: true }]);
                } else if index < runs.len() {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::StoryRun { story, run: index }, TypedJsonNode::StoryRuns { story, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::StoryRun { story, run } => {
                let value = snapshot.stories.get(story).and_then(|story| story.style_runs.get(run)).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"start\":"),
                    Self::scalar(&value.start)?,
                    Self::static_node(b",\"end\":"),
                    Self::scalar(&value.end)?,
                    Self::static_node(b",\"paragraphStyleId\":"),
                    Self::optional_string(value.paragraph_style_id.is_some(), StringSource::StoryRunParagraph { story, run }),
                    Self::static_node(b",\"characterStyleId\":"),
                    Self::optional_string(value.character_style_id.is_some(), StringSource::StoryRunCharacter { story, run }),
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Link(index) => {
                let value = snapshot.links.get(index).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::LinkId(index)),
                    Self::static_node(b",\"path\":"),
                    Self::string(StringSource::LinkPath(index)),
                    Self::static_node(b",\"hash\":"),
                    Self::string(StringSource::LinkHash(index)),
                    Self::static_node(b",\"width\":"),
                    Self::scalar(&value.width)?,
                    Self::static_node(b",\"height\":"),
                    Self::scalar(&value.height)?,
                    Self::static_node(b",\"dpi\":"),
                    Self::scalar(&value.dpi)?,
                    Self::static_node(b",\"colorProfile\":"),
                    Self::optional_string(value.color_profile.is_some(), StringSource::LinkColorProfile(index)),
                    Self::static_node(b",\"state\":"),
                    Self::optional_string(value.state.is_some(), StringSource::LinkState(index)),
                    Self::static_node(b",\"proxyDataUrl\":"),
                    Self::optional_string(value.proxy_data_url.is_some(), StringSource::LinkProxy(index)),
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Parent(index) => {
                let value = snapshot.parent_pages.get(index).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::ParentId(index)),
                    Self::static_node(b",\"name\":"),
                    Self::string(StringSource::ParentName(index)),
                    Self::static_node(b",\"width\":"),
                    Self::scalar(&value.width)?,
                    Self::static_node(b",\"height\":"),
                    Self::scalar(&value.height)?,
                    Self::static_node(b",\"layerIds\":"),
                    TypedJsonNode::StringArray { source: StringArraySource::ParentLayer(index), index: 0, opened: false },
                    Self::static_node(b",\"layers\":"),
                    TypedJsonNode::Layers { owner: RecordOwner::Parent(index), index: 0, opened: false },
                    Self::static_node(b",\"frames\":"),
                    TypedJsonNode::Frames { owner: RecordOwner::Parent(index), index: 0, opened: false },
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Spread(index) => self.push_sequence(vec![
                Self::static_node(b"{\"id\":"),
                Self::string(StringSource::SpreadId(index)),
                Self::static_node(b",\"name\":"),
                Self::string(StringSource::SpreadName(index)),
                Self::static_node(b",\"pageIds\":"),
                TypedJsonNode::StringArray { source: StringArraySource::SpreadPage(index), index: 0, opened: false },
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::Page(index) => {
                let value = snapshot.pages.get(index).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::PageId(index)),
                    Self::static_node(b",\"name\":"),
                    Self::string(StringSource::PageName(index)),
                    Self::static_node(b",\"spreadId\":"),
                    Self::string(StringSource::PageSpreadId(index)),
                    Self::static_node(b",\"parentPageId\":"),
                    Self::optional_string(value.parent_page_id.is_some(), StringSource::PageParentId(index)),
                    Self::static_node(b",\"width\":"),
                    Self::scalar(&value.width)?,
                    Self::static_node(b",\"height\":"),
                    Self::scalar(&value.height)?,
                    Self::static_node(b",\"margins\":"),
                    TypedJsonNode::Margins(index),
                    Self::static_node(b",\"columns\":"),
                    TypedJsonNode::Columns(index),
                    Self::static_node(b",\"guides\":"),
                    TypedJsonNode::Guides { page: index, index: 0, opened: false },
                    Self::static_node(b",\"layerIds\":"),
                    TypedJsonNode::StringArray { source: StringArraySource::PageLayer(index), index: 0, opened: false },
                    Self::static_node(b",\"layers\":"),
                    TypedJsonNode::Layers { owner: RecordOwner::Page(index), index: 0, opened: false },
                    Self::static_node(b",\"frames\":"),
                    TypedJsonNode::Frames { owner: RecordOwner::Page(index), index: 0, opened: false },
                    Self::static_node(b",\"overrides\":"),
                    TypedJsonNode::Overrides { page: index, index: 0, opened: false },
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::StringArray { source, index, opened } => {
                let values = match source {
                    StringArraySource::SpreadPage(owner) => &snapshot.spreads.get(owner).ok_or("layout-export-json-index")?.page_ids,
                    StringArraySource::PageLayer(owner) => &snapshot.pages.get(owner).ok_or("layout-export-json-index")?.layer_ids,
                    StringArraySource::ParentLayer(owner) => &snapshot.parent_pages.get(owner).ok_or("layout-export-json-index")?.layer_ids,
                    StringArraySource::LayerObject { owner, layer } => &Self::owner_page(snapshot, owner)?.layers().get(layer).ok_or("layout-export-json-index")?.object_ids,
                };
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::StringArray { source, index, opened: true }]);
                } else if values.get(index).is_some() {
                    self.push_sequence(vec![
                        if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") },
                        Self::string(StringSource::StringArrayElement { source, index }),
                        TypedJsonNode::StringArray { source, index: index + 1, opened: true },
                    ]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::Layers { owner, index, opened } => {
                let len = Self::owner_page(snapshot, owner)?.layers().len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::Layers { owner, index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::Layer { owner, layer: index }, TypedJsonNode::Layers { owner, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::Layer { owner, layer } => {
                let value = Self::owner_page(snapshot, owner)?.layers().get(layer).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::LayerId { owner, layer }),
                    Self::static_node(b",\"name\":"),
                    Self::string(StringSource::LayerName { owner, layer }),
                    Self::static_node(b",\"visible\":"),
                    Self::scalar(&value.visible)?,
                    Self::static_node(b",\"locked\":"),
                    Self::scalar(&value.locked)?,
                    Self::static_node(b",\"objectIds\":"),
                    TypedJsonNode::StringArray { source: StringArraySource::LayerObject { owner, layer }, index: 0, opened: false },
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Frames { owner, index, opened } => {
                let len = Self::owner_page(snapshot, owner)?.frames().len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::Frames { owner, index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::Frame { owner, frame: index }, TypedJsonNode::Frames { owner, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::Frame { owner, frame } => {
                let value = Self::owner_page(snapshot, owner)?.frames().get(frame).ok_or("layout-export-json-index")?;
                let mut nodes = vec![
                    Self::static_node(b"{\"kind\":"),
                    Self::static_node(match value {
                        Frame::Rect { .. } => b"\"rect\"",
                        Frame::Text { .. } => b"\"text\"",
                        Frame::Image { .. } => b"\"image\"",
                    }),
                    Self::static_node(b",\"id\":"),
                    Self::string(StringSource::FrameId { owner, frame }),
                    Self::static_node(b",\"layerId\":"),
                    Self::string(StringSource::FrameLayerId { owner, frame }),
                    Self::static_node(b",\"bounds\":"),
                    TypedJsonNode::Bounds { owner, frame },
                ];
                match value {
                    Frame::Rect { locked, visible, fill, stroke, .. } => nodes.extend([
                        Self::static_node(b",\"locked\":"),
                        Self::scalar(locked)?,
                        Self::static_node(b",\"visible\":"),
                        Self::scalar(visible)?,
                        Self::static_node(b",\"fill\":"),
                        Self::scalar(fill)?,
                        Self::static_node(b",\"stroke\":"),
                        Self::scalar(stroke)?,
                    ]),
                    Frame::Text { locked, visible, thread_next, columns, .. } => nodes.extend([
                        Self::static_node(b",\"locked\":"),
                        Self::scalar(locked)?,
                        Self::static_node(b",\"visible\":"),
                        Self::scalar(visible)?,
                        Self::static_node(b",\"storyId\":"),
                        Self::string(StringSource::FrameStoryId { owner, frame }),
                        Self::static_node(b",\"threadNext\":"),
                        Self::optional_string(thread_next.is_some(), StringSource::FrameThreadNext { owner, frame }),
                        Self::static_node(b",\"columns\":"),
                        Self::scalar(columns)?,
                        Self::static_node(b",\"inset\":"),
                        TypedJsonNode::TextInset { owner, frame },
                        Self::static_node(b",\"wrapMode\":"),
                        Self::string(StringSource::FrameWrapMode { owner, frame }),
                    ]),
                    Frame::Image { locked, visible, .. } => nodes.extend([
                        Self::static_node(b",\"locked\":"),
                        Self::scalar(locked)?,
                        Self::static_node(b",\"visible\":"),
                        Self::scalar(visible)?,
                        Self::static_node(b",\"linkId\":"),
                        Self::string(StringSource::FrameLinkId { owner, frame }),
                    ]),
                }
                nodes.push(Self::static_node(b"}"));
                self.push_sequence(nodes);
            }
            TypedJsonNode::Bounds { owner, frame } => {
                let value = frame_bounds(Self::owner_page(snapshot, owner)?.frames().get(frame).ok_or("layout-export-json-index")?);
                self.push_sequence(vec![
                    Self::static_node(b"{\"x\":"),
                    Self::scalar(&value.x)?,
                    Self::static_node(b",\"y\":"),
                    Self::scalar(&value.y)?,
                    Self::static_node(b",\"w\":"),
                    Self::scalar(&value.width)?,
                    Self::static_node(b",\"h\":"),
                    Self::scalar(&value.height)?,
                    Self::static_node(b",\"rotation\":"),
                    Self::scalar(&value.rotation)?,
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Rect { page, guide } => {
                let value = snapshot.pages.get(page).and_then(|page| page.guides.get(guide)).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"x\":"),
                    Self::scalar(&value.x)?,
                    Self::static_node(b",\"y\":"),
                    Self::scalar(&value.y)?,
                    Self::static_node(b",\"w\":"),
                    Self::scalar(&value.width)?,
                    Self::static_node(b",\"h\":"),
                    Self::scalar(&value.height)?,
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::TextInset { owner, frame } => {
                let value = match Self::owner_page(snapshot, owner)?.frames().get(frame).ok_or("layout-export-json-index")? {
                    Frame::Text { inset, .. } => inset,
                    _ => return Err("layout-export-json-frame".into()),
                };
                self.push_sequence(vec![
                    Self::static_node(b"{\"x\":"),
                    Self::scalar(&value.x)?,
                    Self::static_node(b",\"y\":"),
                    Self::scalar(&value.y)?,
                    Self::static_node(b",\"w\":"),
                    Self::scalar(&value.width)?,
                    Self::static_node(b",\"h\":"),
                    Self::scalar(&value.height)?,
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Margins(page) => {
                let value = &snapshot.pages.get(page).ok_or("layout-export-json-index")?.margins;
                self.push_sequence(vec![
                    Self::static_node(b"{\"top\":"),
                    Self::scalar(&value.top)?,
                    Self::static_node(b",\"right\":"),
                    Self::scalar(&value.right)?,
                    Self::static_node(b",\"bottom\":"),
                    Self::scalar(&value.bottom)?,
                    Self::static_node(b",\"left\":"),
                    Self::scalar(&value.left)?,
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Columns(page) => {
                let value = &snapshot.pages.get(page).ok_or("layout-export-json-index")?.columns;
                self.push_sequence(vec![Self::static_node(b"{\"count\":"), Self::scalar(&value.count)?, Self::static_node(b",\"gutter\":"), Self::scalar(&value.gutter)?, Self::static_node(b"}")]);
            }
            TypedJsonNode::Guides { page, index, opened } => {
                let len = snapshot.pages.get(page).ok_or("layout-export-json-index")?.guides.len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::Guides { page, index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::Rect { page, guide: index }, TypedJsonNode::Guides { page, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::Overrides { page, index, opened } => {
                let len = snapshot.pages.get(page).ok_or("layout-export-json-index")?.overrides.len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::Overrides { page, index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::Override { page, value: index }, TypedJsonNode::Overrides { page, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::Override { page, value } => {
                let override_value = snapshot.pages.get(page).and_then(|page| page.overrides.get(value)).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"objectId\":"),
                    Self::string(StringSource::OverrideObjectId { page, value }),
                    Self::static_node(b",\"bounds\":"),
                    if override_value.bounds.is_some() { TypedJsonNode::OverrideBounds { page, value } } else { Self::static_node(b"null") },
                    Self::static_node(b",\"visible\":"),
                    Self::scalar(&override_value.visible)?,
                    Self::static_node(b",\"locked\":"),
                    Self::scalar(&override_value.locked)?,
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::OverrideBounds { page, value } => {
                let bounds = snapshot.pages.get(page).and_then(|page| page.overrides.get(value)).and_then(|value| value.bounds.as_ref()).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"x\":"),
                    Self::scalar(&bounds.x)?,
                    Self::static_node(b",\"y\":"),
                    Self::scalar(&bounds.y)?,
                    Self::static_node(b",\"w\":"),
                    Self::scalar(&bounds.width)?,
                    Self::static_node(b",\"h\":"),
                    Self::scalar(&bounds.height)?,
                    Self::static_node(b",\"rotation\":"),
                    Self::scalar(&bounds.rotation)?,
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::BackgroundDrawing => self.push_sequence(vec![Self::static_node(b"{\"handle\":"), TypedJsonNode::ChildHandle, Self::static_node(b",\"content\":"), TypedJsonNode::Drawing, Self::static_node(b"}")]),
            TypedJsonNode::ChildHandle => {
                self.push_sequence(vec![Self::static_node(b"{\"childId\":"), Self::string(StringSource::ChildId), Self::static_node(b",\"target\":"), TypedJsonNode::ArtifactRef { referenced: false }, Self::static_node(b"}")]);
            }
            TypedJsonNode::ArtifactRef { referenced } => self.push_sequence(vec![
                Self::static_node(b"{\"artifactId\":"),
                Self::string(if referenced { StringSource::ReferencedTargetArtifactId } else { StringSource::ChildTargetArtifactId }),
                Self::static_node(b",\"dialect\":"),
                TypedJsonNode::Dialect { referenced },
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::Dialect { referenced } => self.push_sequence(vec![
                Self::static_node(b"{\"artifactKind\":"),
                Self::string(if referenced { StringSource::ReferencedTargetArtifactKind } else { StringSource::ChildTargetArtifactKind }),
                Self::static_node(b",\"standard\":"),
                Self::string(if referenced { StringSource::ReferencedTargetStandard } else { StringSource::ChildTargetStandard }),
                Self::static_node(b",\"subset\":"),
                Self::string(if referenced { StringSource::ReferencedTargetSubset } else { StringSource::ChildTargetSubset }),
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::Drawing => self.push_sequence(vec![
                Self::static_node(b"{\"schema\":"),
                Self::string(StringSource::DrawingSchema),
                Self::static_node(b",\"canvas\":"),
                TypedJsonNode::DrawCanvas,
                Self::static_node(b",\"styles\":"),
                TypedJsonNode::DrawStyles { index: 0, opened: false },
                Self::static_node(b",\"layers\":"),
                TypedJsonNode::DrawLayers { index: 0, opened: false },
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::DrawCanvas => {
                let value = &snapshot.background_drawing.as_ref().ok_or("layout-export-json-background")?.content.canvas;
                let mut nodes = vec![Self::static_node(b"{\"width\":"), Self::scalar(&value.width)?, Self::static_node(b",\"height\":"), Self::scalar(&value.height)?];
                if let Some(background) = value.background {
                    nodes.extend([Self::static_node(b",\"background\":"), TypedJsonNode::DrawRgba(background)]);
                }
                nodes.push(Self::static_node(b"}"));
                self.push_sequence(nodes);
            }
            TypedJsonNode::DrawRgba(value) => self.push_sequence(vec![
                Self::static_node(b"{\"r\":"),
                Self::scalar(&value.r)?,
                Self::static_node(b",\"g\":"),
                Self::scalar(&value.g)?,
                Self::static_node(b",\"b\":"),
                Self::scalar(&value.b)?,
                Self::static_node(b",\"a\":"),
                Self::scalar(&value.a)?,
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::DrawStyles { index, opened } => {
                let len = snapshot.background_drawing.as_ref().ok_or("layout-export-json-background")?.content.styles.len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::DrawStyles { index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::DrawStyle(index), TypedJsonNode::DrawStyles { index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::DrawStyle(index) => {
                let value = snapshot.background_drawing.as_ref().ok_or("layout-export-json-background")?.content.styles.get(index).ok_or("layout-export-json-index")?;
                let mut nodes = vec![Self::static_node(b"{\"name\":"), Self::string(StringSource::DrawStyleName(index))];
                if let Some(fill) = value.fill {
                    nodes.extend([Self::static_node(b",\"fill\":"), TypedJsonNode::DrawRgba(fill)]);
                }
                if let Some(stroke) = value.stroke {
                    nodes.extend([Self::static_node(b",\"stroke\":"), TypedJsonNode::DrawRgba(stroke)]);
                }
                if let Some(width) = value.stroke_width {
                    nodes.extend([Self::static_node(b",\"strokeWidth\":"), Self::scalar(&width)?]);
                }
                if let Some(opacity) = value.opacity {
                    nodes.extend([Self::static_node(b",\"opacity\":"), Self::scalar(&opacity)?]);
                }
                nodes.push(Self::static_node(b"}"));
                self.push_sequence(nodes);
            }
            TypedJsonNode::DrawLayers { index, opened } => {
                let len = snapshot.background_drawing.as_ref().ok_or("layout-export-json-background")?.content.layers.len();
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::DrawLayers { index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::DrawLayer(index), TypedJsonNode::DrawLayers { index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::DrawLayer(index) => {
                let value = snapshot.background_drawing.as_ref().ok_or("layout-export-json-background")?.content.layers.get(index).ok_or("layout-export-json-index")?;
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::DrawLayerId(index)),
                    Self::static_node(b",\"name\":"),
                    Self::string(StringSource::DrawLayerName(index)),
                    Self::static_node(b",\"visible\":"),
                    Self::scalar(&value.visible)?,
                    Self::static_node(b",\"root\":"),
                    TypedJsonNode::DrawNode { layer: index, path: Vec::new() },
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::DrawNode { layer, path } => {
                if path.len() > 64 {
                    return Err("layout-export-json-depth-limit".into());
                }
                let value = Self::draw_node(snapshot, layer, &path)?;
                match value {
                    DrawNode::Path { style, .. } => self.push_sequence(vec![
                        Self::static_node(b"{\"kind\":\"path\",\"segments\":"),
                        TypedJsonNode::DrawSegments { layer, path: path.clone(), index: 0, opened: false },
                        if style.is_some() { Self::static_node(b",\"style\":") } else { Self::static_node(b"") },
                        if style.is_some() { Self::string(StringSource::DrawNodeStyle { layer, path }) } else { Self::static_node(b"") },
                        Self::static_node(b"}"),
                    ]),
                    DrawNode::Text { at, style, .. } => self.push_sequence(vec![
                        Self::static_node(b"{\"kind\":\"text\",\"value\":"),
                        Self::string(StringSource::DrawNodeValue { layer, path: path.clone() }),
                        Self::static_node(b",\"at\":"),
                        TypedJsonNode::DrawPoint2(*at),
                        if style.is_some() { Self::static_node(b",\"style\":") } else { Self::static_node(b"") },
                        if style.is_some() { Self::string(StringSource::DrawNodeStyle { layer, path }) } else { Self::static_node(b"") },
                        Self::static_node(b"}"),
                    ]),
                    DrawNode::Group { transform, .. } => self.push_sequence(vec![
                        Self::static_node(b"{\"kind\":\"group\",\"transform\":"),
                        TypedJsonNode::DrawTransform(*transform),
                        Self::static_node(b",\"children\":"),
                        TypedJsonNode::DrawChildren { layer, path, index: 0, opened: false },
                        Self::static_node(b"}"),
                    ]),
                    DrawNode::Image { at, width, height, .. } => self.push_sequence(vec![
                        Self::static_node(b"{\"kind\":\"image\",\"at\":"),
                        TypedJsonNode::DrawPoint2(*at),
                        Self::static_node(b",\"width\":"),
                        Self::scalar(width)?,
                        Self::static_node(b",\"height\":"),
                        Self::scalar(height)?,
                        Self::static_node(b",\"mime\":"),
                        Self::string(StringSource::DrawNodeMime { layer, path: path.clone() }),
                        Self::static_node(b",\"bytes\":"),
                        TypedJsonNode::DrawBytes { layer, path, index: 0, opened: false },
                        Self::static_node(b"}"),
                    ]),
                }
            }
            TypedJsonNode::DrawChildren { layer, path, index, opened } => {
                if path.len() > 64 {
                    return Err("layout-export-json-depth-limit".into());
                }
                let len = match Self::draw_node(snapshot, layer, &path)? {
                    DrawNode::Group { children, .. } => children.len(),
                    _ => return Err("layout-export-json-draw-path".into()),
                };
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::DrawChildren { layer, path, index, opened: true }]);
                } else if index < len {
                    let mut child = path.clone();
                    child.push(index);
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::DrawNode { layer, path: child }, TypedJsonNode::DrawChildren { layer, path, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::DrawSegments { layer, path, index, opened } => {
                let len = match Self::draw_node(snapshot, layer, &path)? {
                    DrawNode::Path { segments, .. } => segments.len(),
                    _ => return Err("layout-export-json-draw-path".into()),
                };
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::DrawSegments { layer, path, index, opened: true }]);
                } else if index < len {
                    self.push_sequence(vec![
                        if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") },
                        TypedJsonNode::DrawSegment { layer, path: path.clone(), segment: index },
                        TypedJsonNode::DrawSegments { layer, path, index: index + 1, opened: true },
                    ]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::DrawSegment { layer, path, segment } => {
                let value = match Self::draw_node(snapshot, layer, &path)? {
                    DrawNode::Path { segments, .. } => segments.get(segment).ok_or("layout-export-json-index")?,
                    _ => return Err("layout-export-json-draw-path".into()),
                };
                let nodes = match value {
                    PathSegment::MoveTo { to } => vec![Self::static_node(b"{\"kind\":\"moveTo\",\"to\":"), TypedJsonNode::DrawPoint2(*to), Self::static_node(b"}")],
                    PathSegment::LineTo { to } => vec![Self::static_node(b"{\"kind\":\"lineTo\",\"to\":"), TypedJsonNode::DrawPoint2(*to), Self::static_node(b"}")],
                    PathSegment::CubicTo { c1, c2, to } => vec![
                        Self::static_node(b"{\"kind\":\"cubicTo\",\"c1\":"),
                        TypedJsonNode::DrawPoint2(*c1),
                        Self::static_node(b",\"c2\":"),
                        TypedJsonNode::DrawPoint2(*c2),
                        Self::static_node(b",\"to\":"),
                        TypedJsonNode::DrawPoint2(*to),
                        Self::static_node(b"}"),
                    ],
                    PathSegment::QuadTo { c, to } => vec![Self::static_node(b"{\"kind\":\"quadTo\",\"c\":"), TypedJsonNode::DrawPoint2(*c), Self::static_node(b",\"to\":"), TypedJsonNode::DrawPoint2(*to), Self::static_node(b"}")],
                    PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } => vec![
                        Self::static_node(b"{\"kind\":\"arcTo\",\"rx\":"),
                        Self::scalar(rx)?,
                        Self::static_node(b",\"ry\":"),
                        Self::scalar(ry)?,
                        Self::static_node(b",\"xRotation\":"),
                        Self::scalar(x_rotation)?,
                        Self::static_node(b",\"largeArc\":"),
                        Self::scalar(large_arc)?,
                        Self::static_node(b",\"sweep\":"),
                        Self::scalar(sweep)?,
                        Self::static_node(b",\"to\":"),
                        TypedJsonNode::DrawPoint2(*to),
                        Self::static_node(b"}"),
                    ],
                    PathSegment::Close => vec![Self::static_node(b"{\"kind\":\"close\"}")],
                };
                self.push_sequence(nodes);
            }
            TypedJsonNode::DrawPoint2(value) => self.push_sequence(vec![Self::static_node(b"{\"x\":"), Self::scalar(&value.x)?, Self::static_node(b",\"y\":"), Self::scalar(&value.y)?, Self::static_node(b"}")]),
            TypedJsonNode::DrawPoint3(value) => {
                self.push_sequence(vec![Self::static_node(b"{\"x\":"), Self::scalar(&value.x)?, Self::static_node(b",\"y\":"), Self::scalar(&value.y)?, Self::static_node(b",\"z\":"), Self::scalar(&value.z)?, Self::static_node(b"}")]);
            }
            TypedJsonNode::DrawQuaternion(value) => self.push_sequence(vec![
                Self::static_node(b"{\"x\":"),
                Self::scalar(&value.x)?,
                Self::static_node(b",\"y\":"),
                Self::scalar(&value.y)?,
                Self::static_node(b",\"z\":"),
                Self::scalar(&value.z)?,
                Self::static_node(b",\"w\":"),
                Self::scalar(&value.w)?,
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::DrawTransform(value) => self.push_sequence(vec![
                Self::static_node(b"{\"translation\":"),
                TypedJsonNode::DrawPoint3(value.translation),
                Self::static_node(b",\"rotation\":"),
                TypedJsonNode::DrawQuaternion(value.rotation),
                Self::static_node(b",\"scale\":"),
                TypedJsonNode::DrawPoint3(value.scale),
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::DrawBytes { layer, path, index, opened } => {
                let bytes = match Self::draw_node(snapshot, layer, &path)? {
                    DrawNode::Image { bytes, .. } => bytes,
                    _ => return Err("layout-export-json-draw-path".into()),
                };
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::DrawBytes { layer, path, index, opened: true }]);
                } else if let Some(value) = bytes.get(index) {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, Self::scalar(value)?, TypedJsonNode::DrawBytes { layer, path, index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::ReferencedModel => self.push_sequence(vec![
                Self::static_node(b"{\"target\":"),
                TypedJsonNode::ArtifactRef { referenced: true },
                Self::static_node(b",\"pin\":"),
                TypedJsonNode::LinkPin,
                Self::static_node(b",\"role\":"),
                Self::string(StringSource::ReferencedRole),
                Self::static_node(b"}"),
            ]),
            TypedJsonNode::LinkPin => {
                let pin = &snapshot.referenced_model.as_ref().ok_or("layout-export-json-reference")?.pin;
                match pin {
                    store::LinkPin::Head => self.stack.push(Self::static_node(b"{\"kind\":\"head\"}")),
                    store::LinkPin::Checkpoint { .. } => self.push_sequence(vec![Self::static_node(b"{\"kind\":\"checkpoint\",\"id\":"), Self::string(StringSource::ReferencedCheckpointId), Self::static_node(b"}")]),
                    store::LinkPin::Snapshot { .. } => self.push_sequence(vec![Self::static_node(b"{\"kind\":\"snapshot\",\"blob\":"), TypedJsonNode::BlobRef, Self::static_node(b"}")]),
                }
            }
            TypedJsonNode::BlobRef => {
                let blob = match &snapshot.referenced_model.as_ref().ok_or("layout-export-json-reference")?.pin {
                    store::LinkPin::Snapshot { blob } => blob,
                    _ => return Err("layout-export-json-reference".into()),
                };
                self.push_sequence(vec![
                    Self::static_node(b"{\"hash\":"),
                    Self::string(StringSource::ReferencedBlobHash),
                    Self::static_node(b",\"size\":"),
                    Self::scalar(&blob.size)?,
                    Self::static_node(b",\"mediaType\":"),
                    Self::string(StringSource::ReferencedBlobMediaType),
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::MissingLinks { index, opened, emitted } => {
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::MissingLinks { index, opened: true, emitted }]);
                } else if let Some(link) = snapshot.links.get(index) {
                    if link.state.as_deref() == Some("missing") {
                        self.push_sequence(vec![if emitted { Self::static_node(b",") } else { Self::static_node(b"") }, TypedJsonNode::MissingLink(index), TypedJsonNode::MissingLinks { index: index + 1, opened: true, emitted: true }]);
                    } else {
                        self.stack.push(TypedJsonNode::MissingLinks { index: index + 1, opened: true, emitted });
                    }
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::MissingLink(index) => {
                self.push_sequence(vec![Self::static_node(b"{\"kind\":\"missing-link\",\"linkId\":"), Self::string(StringSource::LinkId(index)), Self::static_node(b",\"path\":"), Self::string(StringSource::LinkPath(index)), Self::static_node(b"}")]);
            }
            TypedJsonNode::Manifest => self.push_sequence(vec![
                Self::static_node(b"{\"schema\":\"layout.package-manifest/v1\",\"document\":\"document.json\",\"preflight\":\"preflight-report.json\",\"links\":"),
                TypedJsonNode::ManifestLinks { index: 0, opened: false },
                Self::static_node(b",\"generatedAt\":\"2026-07-02T00:00:00Z\"}"),
            ]),
            TypedJsonNode::ManifestLinks { index, opened } => {
                if !opened {
                    self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::ManifestLinks { index, opened: true }]);
                } else if index < snapshot.links.len() {
                    self.push_sequence(vec![if index == 0 { Self::static_node(b"") } else { Self::static_node(b",") }, TypedJsonNode::ManifestLink(index), TypedJsonNode::ManifestLinks { index: index + 1, opened: true }]);
                } else {
                    self.stack.push(Self::static_node(b"]"));
                }
            }
            TypedJsonNode::ManifestLink(index) => {
                let link = snapshot.links.get(index).ok_or("layout-export-json-index")?;
                let hash = if link.hash.is_empty() { Self::owned_string(format!("sha256:{}", semio_framework_hash::sha256_hex(link.path.as_bytes()))) } else { Self::string(StringSource::LinkHash(index)) };
                self.push_sequence(vec![
                    Self::static_node(b"{\"id\":"),
                    Self::string(StringSource::LinkId(index)),
                    Self::static_node(b",\"path\":"),
                    Self::string(StringSource::LinkPath(index)),
                    Self::static_node(b",\"hash\":"),
                    hash,
                    Self::static_node(b",\"state\":"),
                    Self::optional_string(link.state.is_some(), StringSource::LinkState(index)),
                    Self::static_node(b",\"missing\":"),
                    Self::scalar(&(link.state.as_deref() == Some("missing")))?,
                    Self::static_node(b"}"),
                ]);
            }
            TypedJsonNode::Dictionary(_) | TypedJsonNode::FragmentStart | TypedJsonNode::FragmentEnd | TypedJsonNode::Static { .. } | TypedJsonNode::Scalar { .. } | TypedJsonNode::String { .. } | TypedJsonNode::OwnedString { .. } => {
                unreachable!("leaf nodes are handled before expansion")
            }
        }
        Ok(())
    }
}

enum OwnerPage<'a> {
    Page(&'a Page),
    Parent(&'a semio_s_artifact_layout_layout_model::ParentPage),
}

impl<'a> OwnerPage<'a> {
    fn layers(&self) -> &'a [semio_s_artifact_layout_layout_model::Layer] {
        match self {
            Self::Page(value) => &value.layers,
            Self::Parent(value) => &value.layers,
        }
    }

    fn frames(&self) -> &'a [Frame] {
        match self {
            Self::Page(value) => &value.frames,
            Self::Parent(value) => &value.frames,
        }
    }
}

#[derive(Clone, Copy)]
enum FrameStringField {
    Id,
    LayerId,
    StoryId,
    ThreadNext,
    WrapMode,
    LinkId,
}

fn frame_string(frame: &Frame, field: FrameStringField) -> Option<&str> {
    match (frame, field) {
        (Frame::Rect { id, .. } | Frame::Text { id, .. } | Frame::Image { id, .. }, FrameStringField::Id) => Some(id),
        (Frame::Rect { layer_id, .. } | Frame::Text { layer_id, .. } | Frame::Image { layer_id, .. }, FrameStringField::LayerId) => Some(layer_id),
        (Frame::Text { story_id, .. }, FrameStringField::StoryId) => Some(story_id),
        (Frame::Text { thread_next: Some(value), .. }, FrameStringField::ThreadNext) => Some(value),
        (Frame::Text { wrap_mode, .. }, FrameStringField::WrapMode) => Some(wrap_mode),
        (Frame::Image { link_id, .. }, FrameStringField::LinkId) => Some(link_id),
        _ => None,
    }
}

pub fn frame_bounds(frame: &Frame) -> &LayoutBounds {
    match frame {
        Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds,
    }
}

//#region 🪢️TaxonomyMounts
#[path = "../🧾️dictionary/🦀️.rs"]
pub mod dictionary;
//#endregion 🪢️TaxonomyMounts

impl JsonScalar for bool {
    fn json_bytes(&self) -> Result<Vec<u8>, String> {
        Ok(if *self { b"true".to_vec() } else { b"false".to_vec() })
    }
}
impl JsonScalar for f64 {
    fn json_bytes(&self) -> Result<Vec<u8>, String> {
        finite_json_number(*self)
    }
}
impl JsonScalar for f32 {
    fn json_bytes(&self) -> Result<Vec<u8>, String> {
        if !self.is_finite() {
            return Err("layout-export-json-number".into());
        }
        let mut text = self.to_string();
        if !text.contains('.') && !text.contains('e') && !text.contains('E') {
            text.push_str(".0");
        }
        Ok(text.into_bytes())
    }
}
impl<T: JsonScalar> JsonScalar for Option<T> {
    fn json_bytes(&self) -> Result<Vec<u8>, String> {
        self.as_ref().map_or_else(|| Ok(b"null".to_vec()), JsonScalar::json_bytes)
    }
}
impl JsonScalar for [f32; 4] {
    fn json_bytes(&self) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::with_capacity(64);
        bytes.push(b'[');
        for (index, value) in self.iter().enumerate() {
            if index > 0 {
                bytes.push(b',');
            }
            bytes.extend_from_slice(&value.json_bytes()?);
        }
        bytes.push(b']');
        Ok(bytes)
    }
}
