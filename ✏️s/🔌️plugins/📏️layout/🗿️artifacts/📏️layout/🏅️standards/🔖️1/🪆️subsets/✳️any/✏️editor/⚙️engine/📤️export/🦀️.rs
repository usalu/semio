//! 📤️ Deterministic, resumable Layout export jobs.

use crate::artifacts::layout::{Frame, GridSettings, LayoutBounds, LayoutSnapshot, Page, PageOverride};
use crate::editor::layout::config::LayoutConfigMutation;
use crate::editor::layout::LayoutPlayApp;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework::{InteractiveJobClassification, RetainedToolWireInput, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::{
    BatchDriveConfig, BatchJobParams, Checkpoint, CommitCandidate, Generation, InteractiveJob, InteractiveJobCloseStep, InteractiveStage, JobFault, JobPayloadCloseStep, JobPayloadStream, Operation, RetainedJobPayload, RetainedJobPayloadWriter,
    RevisionId, StepContext, StepOutcome,
};
use semio_framework_plugin::app::{
    ArtifactDownloadOutput, ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactOwnedToolJobFactory, ArtifactReservedToolJob, ArtifactSnapshotCloseLease, ArtifactToolCompletion,
};
use semio_framework_plugin::{ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactReservedJob, EditorApp, EphemeralEmit, Fault, MediaClass, MediaForm, MediaType, PluginCloseStep};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, PathSegment};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

//#region 🔖️Contract
pub const LAYOUT_EXPORT_TOOL_IDS: &[&str] = &["exportPng", "exportSvg", "exportPdf", "exportPackage"];
pub const LAYOUT_EXPORT_PAYLOAD_SCHEMA: &str = "layout.layout.tool-command.v1";
pub const LAYOUT_MEDIA_EXPORT_PAYLOAD_SCHEMA: &str = "layout.layout.media-export.v1";
pub const LAYOUT_MEDIA_EXPORT_TOOL_ID: &str = "export-media:layout:out";
pub const LAYOUT_MEDIA_EXPORT_SCHEMA: &str = "2d.layout";
pub const LAYOUT_PREFLIGHT_REPORT_SCHEMA: &str = "layout.preflight-report.array.v1";
pub const MAX_LAYOUT_EXPORT_RAW_BYTES: usize = 2 << 20;
pub const MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES: usize = 4_096;
pub const MAX_LAYOUT_EXPORT_PAGES: usize = 64;
pub const MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE: usize = 8;
pub const MAX_LAYOUT_EXPORT_TOTAL_FRAMES: usize = 1_024;
pub const MAX_LAYOUT_EXPORT_STORIES: usize = 512;
pub const MAX_LAYOUT_EXPORT_LINKS: usize = 512;
pub const MAX_LAYOUT_EXPORT_PARENT_PAGES: usize = 64;
pub const MAX_LAYOUT_EXPORT_SPREADS: usize = 64;
pub const MAX_LAYOUT_EXPORT_STYLES: usize = 256;
pub const MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE: usize = 16;
pub const MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE: usize = 64;
pub const MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS: usize = 64;
pub const MAX_LAYOUT_EXPORT_JSON_NODES: usize = 2_048;
pub const MAX_LAYOUT_EXPORT_STRING_BYTES: usize = 8 << 10;
pub const MAX_LAYOUT_EXPORT_DIMENSION: u32 = 2_048;
pub const MAX_LAYOUT_EXPORT_PIXELS: u64 = 4_194_304;
pub const MAX_LAYOUT_EXPORT_FILES: usize = 3;
pub const MAX_LAYOUT_EXPORT_OUTPUT_BYTES: usize = 32 << 20;
pub const MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES: usize = 634;
pub const MAX_LAYOUT_EXPORT_DECODED_ITEMS: usize = 131_072;
pub const MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES: usize = 64 << 10;
const OUTPUT_CHUNK_BYTES: usize = 4_096;
const MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS: usize = (MAX_LAYOUT_EXPORT_OUTPUT_BYTES + OUTPUT_CHUNK_BYTES - 1) / OUTPUT_CHUNK_BYTES;
const BASE64_INPUT_BYTES_PER_UNIT: usize = 3_072;
const PNG_PIXELS_PER_UNIT: u32 = 256;
const JSON_INPUT_BYTES_PER_UNIT: usize = 2_048;
const JSON_OUTPUT_BYTES_PER_UNIT: usize = 1_024;
const MAX_LAYOUT_EXPORT_AUTHORITY_BYTES: usize = 256;
const LAYOUT_EXPORT_CHECKPOINT_MAGIC: &[u8; 4] = b"LXC2";

fn empty_close_snapshot() -> LayoutSnapshot {
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
        data_fields_json: None,
        background_drawing: None,
        referenced_model: None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub enum LayoutExportKind {
    Png,
    Svg,
    Pdf,
    Package,
}

impl LayoutExportKind {
    pub fn tool_id(self) -> &'static str {
        match self {
            Self::Png => "exportPng",
            Self::Svg => "exportSvg",
            Self::Pdf => "exportPdf",
            Self::Package => "exportPackage",
        }
    }

    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Svg => "image/svg+xml",
            Self::Pdf => "application/pdf",
            Self::Package => "application/zip",
        }
    }

    pub(crate) fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Svg => "svg",
            Self::Pdf => "pdf",
            Self::Package => "layout-package.zip",
        }
    }

    fn binary(self) -> bool {
        !matches!(self, Self::Svg)
    }
}

#[derive(Clone, Debug)]
pub struct LayoutExportRequest {
    pub kind: LayoutExportKind,
    pub page_id: Option<String>,
    pub snapshot: Arc<LayoutSnapshot>,
    pub preflight_json: Option<String>,
    pub parent_document_id: String,
    pub canonical_base_revision_hex: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutExportCheckpoint {
    pub completed_units: u64,
    pub output_bytes: u64,
    pub output_digest: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutExportCommit {
    pub filename: String,
    pub mime_type: String,
    pub encoding: Option<String>,
    pub data: String,
}

impl LayoutExportCommit {
    pub(crate) fn from_output(kind: LayoutExportKind, name: &str, bytes: Vec<u8>) -> Self {
        // SAFETY: every final route emits only ASCII: SVG's owned numeric vocabulary or the owned
        // base64 encoder. The invariant is enforced before bytes enter the final commit cursor.
        let data = unsafe { String::from_utf8_unchecked(bytes) };
        Self { filename: format!("{}.{}", sanitize_filename(name), kind.extension()), mime_type: kind.mime_type().into(), encoding: kind.binary().then(|| "base64".into()), data }
    }

    fn from_chunks(kind: LayoutExportKind, name: &str, chunks: &ArtifactOutputChunks) -> Result<Self, String> {
        let mut bytes = Vec::with_capacity(chunks.bytes());
        while let Some(chunk) = chunks.take_chunk().map_err(|error| error.to_string())? {
            bytes.extend_from_slice(&chunk);
        }
        Ok(Self::from_output(kind, name, bytes))
    }
}
//#endregion 🔖️Contract

//#region 🧱️Storage
#[derive(Debug)]
struct ChunkRope {
    chunks: std::collections::VecDeque<Vec<u8>>,
    len: usize,
    digest: u64,
    front_byte_cursor: usize,
}

impl ChunkRope {
    fn close_take_chunk(&mut self, maximum_bytes: usize) -> Option<Result<usize, ()>> {
        let bytes = self.chunks.front().map_or(0, Vec::len);
        if bytes == 0 {
            return None;
        }
        if bytes > maximum_bytes {
            return Some(Err(()));
        }
        drop(self.chunks.pop_front());
        Some(Ok(bytes))
    }
}

impl ChunkRope {
    fn new() -> Self {
        Self { chunks: std::collections::VecDeque::with_capacity(MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS), len: 0, digest: 0xcbf29ce484222325, front_byte_cursor: 0 }
    }

    fn append(&mut self, mut bytes: &[u8]) -> Result<(), String> {
        if self.len.checked_add(bytes.len()).is_none_or(|len| len > MAX_LAYOUT_EXPORT_OUTPUT_BYTES) {
            return Err("layout-export-output-limit".into());
        }
        while !bytes.is_empty() {
            if self.chunks.back().is_none_or(|chunk| chunk.len() == OUTPUT_CHUNK_BYTES) {
                if self.chunks.len() == MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS {
                    return Err("layout-export-output-limit".into());
                }
                self.chunks.push_back(Vec::with_capacity(OUTPUT_CHUNK_BYTES));
            }
            let chunk = self.chunks.back_mut().expect("chunk exists");
            let take = bytes.len().min(OUTPUT_CHUNK_BYTES - chunk.len());
            for byte in &bytes[..take] {
                self.digest = (self.digest ^ u64::from(*byte)).wrapping_mul(0x100000001b3);
            }
            chunk.extend_from_slice(&bytes[..take]);
            self.len += take;
            bytes = &bytes[take..];
        }
        Ok(())
    }

    fn take_chunk(&mut self) -> Option<Vec<u8>> {
        if self.front_byte_cursor != 0 {
            return None;
        }
        self.chunks.pop_front()
    }

    fn take_prefix(&mut self, maximum: usize) -> Vec<u8> {
        let mut output = Vec::with_capacity(maximum);
        while output.len() < maximum {
            let Some(chunk) = self.chunks.front() else { break };
            let take = (maximum - output.len()).min(chunk.len() - self.front_byte_cursor);
            output.extend_from_slice(&chunk[self.front_byte_cursor..self.front_byte_cursor + take]);
            self.front_byte_cursor += take;
            if self.front_byte_cursor == chunk.len() {
                self.chunks.pop_front();
                self.front_byte_cursor = 0;
            }
        }
        output
    }
}

#[derive(Clone, Debug)]
struct ExportRect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    rgba: [u8; 4],
}

#[derive(Clone, Debug)]
struct ZipEntry {
    name: String,
    crc: u32,
    size: u32,
    offset: u32,
}

#[derive(Clone, Debug, Default)]
struct ZipState {
    entries: Vec<ZipEntry>,
    current_name: Option<String>,
    current_crc: u32,
    current_size: u32,
    current_offset: u32,
}

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
                        let low = matches!(escape, JsonStringEscape::LowUnicode { .. });
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
            if !matches!(self.token, JsonToken::None) {
                if self.advance_token(bytes)? {
                    continue;
                }
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
    SpreadPageIds(usize),
    PageLayerIds(usize),
    ParentLayerIds(usize),
    LayerObjectIds { owner: RecordOwner, layer: usize },
}

#[derive(Clone, Debug)]
enum StringSource {
    DocumentSchema,
    DocumentName,
    PrintTarget,
    DataFieldsJson,
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
enum TypedJsonNode {
    FragmentStart,
    FragmentEnd,
    Static { bytes: &'static [u8], cursor: usize },
    Scalar { bytes: Vec<u8>, cursor: usize },
    String { source: StringSource, cursor: JsonStringWriteCursor },
    OwnedString { value: String, cursor: JsonStringWriteCursor },
    Document,
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
struct TypedJsonCursor {
    stack: Vec<TypedJsonNode>,
    emitted_nodes: usize,
    fragment_bytes: Option<usize>,
}

fn typed_string_source_owned_bytes(source: &StringSource) -> usize {
    match source {
        StringSource::DrawNodeValue { path, .. } | StringSource::DrawNodeStyle { path, .. } | StringSource::DrawNodeMime { path, .. } => path.len().saturating_mul(std::mem::size_of::<usize>()),
        _ => 0,
    }
}

fn typed_json_node_owned_bytes(node: &TypedJsonNode) -> usize {
    match node {
        TypedJsonNode::Scalar { bytes, .. } => bytes.len(),
        TypedJsonNode::String { source, .. } => typed_string_source_owned_bytes(source),
        TypedJsonNode::OwnedString { value, .. } => value.len(),
        TypedJsonNode::DrawNode { path, .. } | TypedJsonNode::DrawChildren { path, .. } | TypedJsonNode::DrawSegments { path, .. } | TypedJsonNode::DrawSegment { path, .. } | TypedJsonNode::DrawBytes { path, .. } => {
            path.len().saturating_mul(std::mem::size_of::<usize>())
        }
        _ => 0,
    }
}

trait JsonScalar {
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

integer_json_scalar!(u8, u32, u64, usize);

impl JsonScalar for bool {
    fn json_bytes(&self) -> Result<Vec<u8>, String> {
        Ok(if *self { b"true".to_vec() } else { b"false".to_vec() })
    }
}

fn finite_json_number(value: f64) -> Result<Vec<u8>, String> {
    if !value.is_finite() {
        return Err("layout-export-json-number".into());
    }
    let mut text = value.to_string();
    if !text.contains('.') && !text.contains('e') && !text.contains('E') {
        text.push_str(".0");
    }
    Ok(text.into_bytes())
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
            StringSource::DataFieldsJson => snapshot.data_fields_json.as_deref().ok_or_else(missing)?,
            StringSource::StringArrayElement { source, index } => match source {
                StringArraySource::SpreadPageIds(owner) => snapshot.spreads.get(*owner).and_then(|value| value.page_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
                StringArraySource::PageLayerIds(owner) => snapshot.pages.get(*owner).and_then(|value| value.layer_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
                StringArraySource::ParentLayerIds(owner) => snapshot.parent_pages.get(*owner).and_then(|value| value.layer_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
                StringArraySource::LayerObjectIds { owner, layer } => Self::owner_page(snapshot, *owner)?.layers().get(*layer).and_then(|value| value.object_ids.get(*index)).map(String::as_str).ok_or_else(missing)?,
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

    fn escaped_string_step(value: &str, cursor: &mut JsonStringWriteCursor) -> Result<(Vec<u8>, bool), String> {
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
        Ok((output, cursor.closed))
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
                let maximum = if matches!(&source, StringSource::DataFieldsJson) { MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES } else { MAX_LAYOUT_EXPORT_STRING_BYTES };
                let value = Self::resolve_string(&source, snapshot)?;
                if value.len() > maximum {
                    return Err("layout-export-json-string-limit".into());
                }
                let (bytes, done) = Self::escaped_string_step(value, &mut cursor)?;
                output = bytes;
                if !done {
                    self.stack.push(TypedJsonNode::String { source, cursor });
                }
            }
            TypedJsonNode::OwnedString { value, mut cursor } => {
                self.account_node()?;
                if value.len() > MAX_LAYOUT_EXPORT_STRING_BYTES {
                    return Err("layout-export-json-string-limit".into());
                }
                let (bytes, done) = Self::escaped_string_step(&value, &mut cursor)?;
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
                if snapshot.data_fields_json.is_some() {
                    nodes.extend([Self::static_node(b",\"dataFieldsJson\":"), Self::string(StringSource::DataFieldsJson)]);
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
                    TypedJsonNode::StringArray { source: StringArraySource::ParentLayerIds(index), index: 0, opened: false },
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
                TypedJsonNode::StringArray { source: StringArraySource::SpreadPageIds(index), index: 0, opened: false },
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
                    TypedJsonNode::StringArray { source: StringArraySource::PageLayerIds(index), index: 0, opened: false },
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
                    StringArraySource::SpreadPageIds(owner) => &snapshot.spreads.get(owner).ok_or("layout-export-json-index")?.page_ids,
                    StringArraySource::PageLayerIds(owner) => &snapshot.pages.get(owner).ok_or("layout-export-json-index")?.layer_ids,
                    StringArraySource::ParentLayerIds(owner) => &snapshot.parent_pages.get(owner).ok_or("layout-export-json-index")?.layer_ids,
                    StringArraySource::LayerObjectIds { owner, layer } => &Self::owner_page(snapshot, owner)?.layers().get(layer).ok_or("layout-export-json-index")?.object_ids,
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
                    TypedJsonNode::StringArray { source: StringArraySource::LayerObjectIds { owner, layer }, index: 0, opened: false },
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
                self.push_sequence(vec![Self::static_node(b"{\"childId\":"), Self::string(StringSource::ChildId), Self::static_node(b",\"target\":"), TypedJsonNode::ArtifactRef { referenced: false }, Self::static_node(b"}")])
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
                self.push_sequence(vec![Self::static_node(b"{\"x\":"), Self::scalar(&value.x)?, Self::static_node(b",\"y\":"), Self::scalar(&value.y)?, Self::static_node(b",\"z\":"), Self::scalar(&value.z)?, Self::static_node(b"}")])
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
                self.push_sequence(vec![Self::static_node(b"{\"kind\":\"missing-link\",\"linkId\":"), Self::string(StringSource::LinkId(index)), Self::static_node(b",\"path\":"), Self::string(StringSource::LinkPath(index)), Self::static_node(b"}")])
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
            TypedJsonNode::FragmentStart | TypedJsonNode::FragmentEnd | TypedJsonNode::Static { .. } | TypedJsonNode::Scalar { .. } | TypedJsonNode::String { .. } | TypedJsonNode::OwnedString { .. } => {
                unreachable!("leaf nodes are handled before expansion")
            }
        }
        Ok(())
    }
}

enum OwnerPage<'a> {
    Page(&'a crate::artifacts::layout::Page),
    Parent(&'a crate::artifacts::layout::ParentPage),
}

impl OwnerPage<'_> {
    fn layers(&self) -> &[crate::artifacts::layout::Layer] {
        match self {
            Self::Page(value) => &value.layers,
            Self::Parent(value) => &value.layers,
        }
    }

    fn frames(&self) -> &[Frame] {
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
//#endregion 🧱️Storage

//#region 🧩️Job
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExportStage {
    Validate,
    Plan,
    Encode,
    Base64,
    PackageCommit,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PdfSection {
    Header,
    Catalog,
    Pages,
    PageBegin,
    PageKids,
    PageEnd,
    Rects,
    XrefBegin,
    XrefEntries,
    Trailer,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PngRowSection {
    Initialize,
    Fill,
    Header,
    Data,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PackageSection {
    Begin,
    Scalars,
    Preflight,
    Manifest,
    CentralDirectory,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LayoutExportCloseStage {
    Publication,
    JsonValidation,
    TypedValidation,
    PackageJson,
    Rects,
    Output,
    Encoded,
    Base64Tail,
    PngRow,
    PdfOffsets,
    ZipEntries,
    ZipCurrentName,
    PageAuthority,
    Preflight,
    ParentAuthority,
    RevisionAuthority,
    OutputChunks,
    MediaCredit,
    Snapshot,
    SnapshotOwner,
    Complete,
}

#[derive(Clone, Copy)]
enum LayoutExportPublicationKind {
    Preview,
    Checkpoint { applied_progress: u64 },
    Commit,
    Fault,
}

struct LayoutExportPublication {
    kind: LayoutExportPublicationKind,
    bytes: [u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES],
    length: usize,
    cursor: usize,
    writer: Option<RetainedJobPayloadWriter>,
}

impl LayoutExportPublication {
    fn new(kind: LayoutExportPublicationKind, bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES {
            return Err("layout-export-publication-limit".into());
        }
        let mut storage = [0; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES];
        storage[..bytes.len()].copy_from_slice(bytes);
        let stream = match kind {
            LayoutExportPublicationKind::Preview => JobPayloadStream::Preview,
            LayoutExportPublicationKind::Checkpoint { .. } => JobPayloadStream::CheckpointState,
            LayoutExportPublicationKind::Commit => JobPayloadStream::CommitState,
            LayoutExportPublicationKind::Fault => JobPayloadStream::Fault,
        };
        Ok(Self { kind, bytes: storage, length: bytes.len(), cursor: 0, writer: Some(RetainedJobPayloadWriter::new(stream)) })
    }

    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        let writer = self.writer.as_mut().expect("layout publication owns writer until finish");
        match writer.write_slice_page(context, &self.bytes[..self.length], &mut self.cursor) {
            Ok(false) | Err(_) => StepOutcome::Yield,
            Ok(true) => {
                let writer = self.writer.take().expect("layout publication owns completed writer");
                let payload = match writer.finish() {
                    Ok(payload) => payload,
                    Err(writer) => {
                        self.writer = Some(writer);
                        return StepOutcome::Yield;
                    }
                };
                match self.kind {
                    LayoutExportPublicationKind::Preview => StepOutcome::PreviewReady(payload),
                    LayoutExportPublicationKind::Checkpoint { applied_progress } => StepOutcome::CheckpointReady(Checkpoint { state: payload, applied_progress }),
                    LayoutExportPublicationKind::Commit => StepOutcome::Complete(CommitCandidate { state: payload, output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) }),
                    LayoutExportPublicationKind::Fault => StepOutcome::Fault(JobFault { detail: payload }),
                }
            }
        }
    }

    fn begin_close(&mut self) {
        if let Some(writer) = self.writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> JobPayloadCloseStep {
        let Some(writer) = self.writer.as_mut() else { return JobPayloadCloseStep::Complete };
        let step = writer.close_step(maximum_items, maximum_bytes);
        if writer.terminal_is_empty() {
            self.writer = None;
        }
        step
    }

    fn terminal_is_empty(&self) -> bool {
        self.writer.is_none()
    }
}

pub struct LayoutExportJob {
    operation: Operation,
    request: LayoutExportRequest,
    stage: ExportStage,
    validation_group: u8,
    validation_index: usize,
    validation_total_frames: usize,
    validation_total_items: usize,
    json_validation: Option<JsonValidationCursor>,
    typed_validation: Option<TypedJsonCursor>,
    target_page: Option<usize>,
    plan_cursor: usize,
    parent_frame_count: usize,
    rects: Vec<ExportRect>,
    encode_cursor: usize,
    output: ChunkRope,
    encoded: ChunkRope,
    base64_cursor: usize,
    base64_tail: Vec<u8>,
    png_adler_s1: u32,
    png_adler_s2: u32,
    png_idat_crc: u32,
    png_row: Vec<u8>,
    png_row_section: PngRowSection,
    png_row_byte_cursor: usize,
    png_rect_cursor: usize,
    png_fill_x: u32,
    pdf_offsets: Vec<u32>,
    pdf_section: PdfSection,
    pdf_cursor: usize,
    pdf_xref_offset: usize,
    package_section: PackageSection,
    package_index: usize,
    package_byte_cursor: usize,
    package_json: Option<TypedJsonCursor>,
    zip_central_offset: u32,
    zip: ZipState,
    media_output_credit: Option<ArtifactMediaExportCredit>,
    output_chunks: ArtifactOutputChunks,
    commit_cursor: usize,
    completed_units: u64,
    restore_target: Option<LayoutExportCheckpoint>,
    snapshot_close: Option<ArtifactSnapshotCloseLease<LayoutSnapshot>>,
    snapshot_placeholder: Option<Arc<LayoutSnapshot>>,
    publication: Option<LayoutExportPublication>,
    closing: bool,
    close_stage: LayoutExportCloseStage,
}

pub struct LayoutExportToolPayload {
    pub request: LayoutExportRequest,
    pub output_chunks: ArtifactOutputChunks,
    pub completion: Option<ArtifactToolCompletion<EditorApp<LayoutPlayApp>>>,
}

#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct LayoutExportWireCommand {
    #[value(default)]
    page_id: Option<String>,
}

pub struct LayoutExportToolJob {
    inner: Option<LayoutExportJob>,
    pending_operation: Option<Operation>,
    pending_request: Option<LayoutExportRequest>,
    pending_output_chunks: Option<ArtifactOutputChunks>,
    completion: Option<ArtifactToolCompletion<EditorApp<LayoutPlayApp>>>,
    kind: LayoutExportKind,
    name: String,
    raw_input: Option<RetainedToolWireInput>,
    raw_bytes: Vec<u8>,
    raw_page_cursor: usize,
    raw_scan_cursor: usize,
    raw_stack: [u8; 64],
    raw_stack_len: usize,
    raw_in_string: bool,
    raw_escape: bool,
    raw_invalid: bool,
    raw_validated: bool,
    completed: bool,
}

impl LayoutExportToolJob {
    fn materialize_decoded_job(&mut self) -> bool {
        if self.inner.is_some() {
            return true;
        }
        let (Some(operation), Some(request), Some(output_chunks)) = (self.pending_operation.take(), self.pending_request.take(), self.pending_output_chunks.take()) else {
            return false;
        };
        self.name = output_name(&request);
        self.inner = LayoutExportJob::new(operation, request).ok().map(|job| job.with_output_chunks(output_chunks));
        self.inner.is_some()
    }

    fn scan_raw_byte(&mut self, byte: u8) {
        if self.raw_in_string {
            if self.raw_escape {
                self.raw_escape = false;
            } else if byte == b'\\' {
                self.raw_escape = true;
            } else if byte == b'"' {
                self.raw_in_string = false;
            } else if byte < 0x20 {
                self.raw_invalid = true;
            }
            return;
        }
        match byte {
            b'"' => self.raw_in_string = true,
            b'[' | b'{' if self.raw_stack_len < self.raw_stack.len() => {
                self.raw_stack[self.raw_stack_len] = if byte == b'[' { b']' } else { b'}' };
                self.raw_stack_len += 1;
            }
            b'[' | b'{' => self.raw_invalid = true,
            b']' | b'}' if self.raw_stack_len != 0 && self.raw_stack[self.raw_stack_len - 1] == byte => self.raw_stack_len -= 1,
            b']' | b'}' => self.raw_invalid = true,
            _ => {}
        }
    }

    fn decoded_wire_command_matches(&self) -> bool {
        let Ok((verb, command)) = serde_json::from_slice::<(String, Option<LayoutExportWireCommand>)>(&self.raw_bytes) else { return false };
        if verb != self.kind.tool_id() {
            return false;
        }
        let page_id = command.and_then(|command| command.page_id);
        match self.kind {
            LayoutExportKind::Package => page_id.is_none(),
            _ => page_id.as_ref().is_none_or(|page_id| self.pending_request.as_ref().and_then(|request| request.page_id.as_ref()) == Some(page_id)),
        }
    }
}

impl InteractiveJob for LayoutExportToolJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if !self.raw_validated {
            if context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            if context.should_yield() || context.fuel_remaining() == 0 {
                return StepOutcome::Yield;
            }
            context.set_stage("layout-export-retained-wire-decode");
            let Some(input) = self.raw_input.as_ref() else {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            };
            if let Some(page) = input.page(self.raw_page_cursor) {
                self.raw_bytes.extend_from_slice(page);
                self.raw_page_cursor = self.raw_page_cursor.saturating_add(1);
                context.consume_fuel(1);
                let cursor = (self.raw_page_cursor as u64).to_le_bytes();
                let state = context.payload_from_bytes(JobPayloadStream::CheckpointState, &cursor).unwrap_or_else(|rejected| {
                    drop(rejected.into_source());
                    RetainedJobPayload::empty(JobPayloadStream::CheckpointState)
                });
                return StepOutcome::CheckpointReady(Checkpoint { state, applied_progress: self.raw_bytes.len() as u64 });
            }
            if self.raw_scan_cursor < self.raw_bytes.len() {
                let byte = self.raw_bytes[self.raw_scan_cursor];
                self.scan_raw_byte(byte);
                self.raw_scan_cursor += 1;
                context.consume_fuel(1);
                let cursor = (self.raw_scan_cursor as u64).to_le_bytes();
                let state = context.payload_from_bytes(JobPayloadStream::CheckpointState, &cursor).unwrap_or_else(|rejected| {
                    drop(rejected.into_source());
                    RetainedJobPayload::empty(JobPayloadStream::CheckpointState)
                });
                return StepOutcome::CheckpointReady(Checkpoint { state, applied_progress: self.raw_bytes.len().saturating_add(self.raw_scan_cursor) as u64 });
            }
            if self.raw_invalid || self.raw_in_string || self.raw_escape || self.raw_stack_len != 0 || !self.decoded_wire_command_matches() {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            }
            if !self.materialize_decoded_job() {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            }
            self.raw_validated = true;
            context.consume_fuel(1);
            let cursor = (self.raw_page_cursor as u64).to_le_bytes();
            let state = context.payload_from_bytes(JobPayloadStream::CheckpointState, &cursor).unwrap_or_else(|rejected| {
                drop(rejected.into_source());
                RetainedJobPayload::empty(JobPayloadStream::CheckpointState)
            });
            return StepOutcome::CheckpointReady(Checkpoint { state, applied_progress: self.raw_bytes.len().saturating_add(1) as u64 });
        }
        let Some(inner) = self.inner.as_mut() else {
            return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
        };
        match inner.step(context) {
            StepOutcome::Complete(candidate) => {
                if self.completed {
                    return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                }
                if self.completion.is_none() {
                    self.completed = true;
                    return StepOutcome::Complete(candidate);
                }
                let download = ArtifactDownloadOutput::new(format!("{}.{}", sanitize_filename(&self.name), self.kind.extension()), self.kind.mime_type(), self.kind.binary().then(|| "base64".into()), inner.output_chunks.clone());
                if let Some(completion) = &self.completion {
                    if let Err(error) = completion.complete_download(download, EphemeralEmit::<EditorApp<LayoutPlayApp>>::default()) {
                        let _ = error;
                        return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                    }
                }
                self.completed = true;
                StepOutcome::Complete(candidate)
            }
            outcome => outcome,
        }
    }

    fn begin_close(&mut self) {
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
        let _ = self.materialize_decoded_job();
        if let Some(inner) = self.inner.as_mut() {
            inner.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.begin_close();
        if let Some(inner) = self.inner.as_mut() {
            match InteractiveJob::close_step(inner, maximum_items, maximum_bytes) {
                InteractiveJobCloseStep::Complete => self.inner = None,
                step => return step,
            }
        }
        if !self.raw_bytes.is_empty() {
            if maximum_bytes == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            let released_bytes = self.raw_bytes.len().min(maximum_bytes);
            self.raw_bytes.truncate(self.raw_bytes.len() - released_bytes);
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes };
        }
        if let Some(input) = self.raw_input.as_mut() {
            let step = input.close_step(maximum_items.min(1), maximum_bytes);
            if input.terminal_is_empty() {
                self.raw_input = None;
            }
            return match step {
                InteractiveJobCloseStep::Complete => InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                other => other,
            };
        }
        if !self.name.is_empty() {
            if maximum_items == 0 || maximum_bytes < self.name.len() {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            let released_bytes = self.name.len();
            self.name.clear();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
        }
        if self.completion.is_none() {
            return InteractiveJobCloseStep::Complete;
        }
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.completion = None;
        InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    fn terminal_is_empty(&self) -> bool {
        self.name.is_empty() && self.completion.is_none() && self.raw_input.is_none() && self.raw_bytes.is_empty() && self.inner.is_none() && self.pending_operation.is_none() && self.pending_request.is_none() && self.pending_output_chunks.is_none()
    }
}

pub struct LayoutExportJobFactory {
    keys: Vec<ToolFactoryKey>,
}

pub struct LayoutMediaExportJob {
    inner: LayoutExportJob,
    completion: Option<ArtifactMediaExportCompletion>,
    completed: bool,
}

impl LayoutMediaExportJob {
    pub fn new(inner: LayoutExportJob, completion: ArtifactMediaExportCompletion) -> Self {
        Self { inner, completion: Some(completion), completed: false }
    }
}

impl InteractiveJob for LayoutMediaExportJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        match self.inner.step(context) {
            StepOutcome::Complete(candidate) => {
                if self.completed {
                    return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                }
                if let Some(credit) = &self.inner.media_output_credit {
                    if let Err(error) = credit.credit(LAYOUT_MEDIA_EXPORT_SCHEMA.len()) {
                        let _ = error;
                        return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                    }
                }
                let media = match ArtifactMediaExportResult::structured(MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, LAYOUT_MEDIA_EXPORT_SCHEMA, self.inner.output_chunks.clone()) {
                    Ok(media) => media,
                    Err(error) => {
                        let _ = error;
                        return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                    }
                };
                let Some(completion) = self.completion.as_ref() else {
                    return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                };
                if let Err(error) = completion.complete(Ok(media)) {
                    let _ = error;
                    return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                }
                self.completed = true;
                StepOutcome::Complete(candidate)
            }
            outcome => outcome,
        }
    }

    fn begin_close(&mut self) {
        self.inner.begin_close();
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        match ArtifactReservedJob::close_step(self, maximum_items, maximum_bytes) {
            Ok(PluginCloseStep::Pending { released_items, released_bytes }) => InteractiveJobCloseStep::Pending { released_items, released_bytes },
            Ok(PluginCloseStep::Complete) => InteractiveJobCloseStep::Complete,
            Err(_) => InteractiveJobCloseStep::Blocked,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        ArtifactReservedJob::terminal_is_empty(self)
    }
}

impl ArtifactReservedJob for LayoutMediaExportJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.inner.close_step(maximum_items, maximum_bytes)? {
            PluginCloseStep::Complete => {}
            step => return Ok(step),
        }
        let Some(completion) = self.completion.as_ref() else { return Ok(PluginCloseStep::Complete) };
        if self.inner.output_chunks.chunks_remaining() != 0 {
            return Err(Fault::from("layout-media-export-close-completion-before-chunk-drain"));
        }
        let had_result = completion.close_take()?;
        if had_result && !self.completed {
            return Err(Fault::from("layout-media-export-close-unowned-completion-result"));
        }
        drop(self.completion.take());
        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool {
        self.completion.is_none() && self.inner.close_terminal_is_empty()
    }
}

pub struct LayoutMediaExportJobFactory {
    keys: [ToolFactoryKey; 1],
}

impl LayoutMediaExportJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: [ToolFactoryKey::new(controller_id, LAYOUT_MEDIA_EXPORT_TOOL_ID)] }
    }
}

impl semio_framework::ToolJobFactory for LayoutMediaExportJobFactory {
    type Payload = ArtifactReservedToolJob;
    type Job = ArtifactReservedToolJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        LAYOUT_MEDIA_EXPORT_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(MAX_LAYOUT_EXPORT_RAW_BYTES, MAX_LAYOUT_EXPORT_DECODED_ITEMS, 1, MAX_LAYOUT_EXPORT_OUTPUT_BYTES, 2_000, 64, 1)
    }

    fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(payload)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for LayoutMediaExportJobFactory {
    type Owner = semio_framework_plugin::EditorApp<LayoutPlayApp>;
    const TOOL_IDS: &'static [&'static str] = &[LAYOUT_MEDIA_EXPORT_TOOL_ID];
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::layout::LAYOUT_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: LAYOUT_MEDIA_EXPORT_TOOL_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}

impl LayoutExportJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: LAYOUT_EXPORT_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for LayoutExportJobFactory {
    type Payload = LayoutExportToolPayload;
    type Job = LayoutExportToolJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        LAYOUT_EXPORT_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES, MAX_LAYOUT_EXPORT_DECODED_ITEMS, 1, MAX_LAYOUT_EXPORT_OUTPUT_BYTES, 2_000, 64, 1)
    }

    fn create_job(&mut self, operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        let kind = payload.request.kind;
        let name = output_name(&payload.request);
        let inner = LayoutExportJob::new(operation, payload.request).map_err(ToolJobFactoryError::new)?.with_output_chunks(payload.output_chunks);
        Ok(LayoutExportToolJob {
            inner: Some(inner),
            pending_operation: None,
            pending_request: None,
            pending_output_chunks: None,
            completion: payload.completion,
            kind,
            name,
            raw_input: None,
            raw_bytes: Vec::new(),
            raw_page_cursor: 0,
            raw_scan_cursor: 0,
            raw_stack: [0; 64],
            raw_stack_len: 0,
            raw_in_string: false,
            raw_escape: false,
            raw_invalid: false,
            raw_validated: true,
            completed: false,
        })
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        operation: Operation,
        payload: Self::Payload,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        if checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("layout export retained ingress does not accept an unvalidated checkpoint owner"), input, checkpoint));
        }
        let declared_bytes = input.declared_bytes();
        if declared_bytes > MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES {
            return Err((ToolJobFactoryError::new("layout export retained ingress exceeds its command byte envelope"), input, None));
        }
        let kind = payload.request.kind;
        let mut job = LayoutExportToolJob {
            inner: None,
            pending_operation: Some(operation),
            pending_request: Some(payload.request),
            pending_output_chunks: Some(payload.output_chunks),
            completion: payload.completion,
            kind,
            name: String::new(),
            raw_input: None,
            raw_bytes: Vec::new(),
            raw_page_cursor: 0,
            raw_scan_cursor: 0,
            raw_stack: [0; 64],
            raw_stack_len: 0,
            raw_in_string: false,
            raw_escape: false,
            raw_invalid: false,
            raw_validated: false,
            completed: false,
        };
        if job.raw_bytes.try_reserve_exact(declared_bytes).is_err() {
            return Err((ToolJobFactoryError::new("layout export retained decoder capacity was not admitted"), input, None));
        }
        job.raw_input = Some(input);
        Ok(job)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for LayoutExportJobFactory {
    type Owner = semio_framework_plugin::EditorApp<LayoutPlayApp>;
    const TOOL_IDS: &'static [&'static str] = LAYOUT_EXPORT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::layout::LAYOUT_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "exportPng", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportSvg", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportPdf", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportPackage", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

impl LayoutExportJob {
    pub fn new(operation: Operation, request: LayoutExportRequest) -> Result<Self, String> {
        if !bounded_id(&request.parent_document_id) || request.canonical_base_revision_hex.len() != 64 || !request.canonical_base_revision_hex.bytes().all(|byte| byte.is_ascii_hexdigit()) || request.page_id.as_ref().is_some_and(|id| !bounded_id(id))
        {
            return Err("layout-export-authority-invalid".into());
        }
        Ok(Self {
            operation,
            request,
            stage: ExportStage::Validate,
            validation_group: 0,
            validation_index: 0,
            validation_total_frames: 0,
            validation_total_items: 0,
            json_validation: None,
            typed_validation: None,
            target_page: None,
            plan_cursor: 0,
            parent_frame_count: 0,
            rects: Vec::new(),
            encode_cursor: 0,
            output: ChunkRope::new(),
            encoded: ChunkRope::new(),
            base64_cursor: 0,
            base64_tail: Vec::new(),
            png_adler_s1: 1,
            png_adler_s2: 0,
            png_idat_crc: 0,
            png_row: Vec::new(),
            png_row_section: PngRowSection::Initialize,
            png_row_byte_cursor: 0,
            png_rect_cursor: 0,
            png_fill_x: 0,
            pdf_offsets: Vec::new(),
            pdf_section: PdfSection::Header,
            pdf_cursor: 0,
            pdf_xref_offset: 0,
            package_section: PackageSection::Begin,
            package_index: 0,
            package_byte_cursor: 0,
            package_json: None,
            zip_central_offset: 0,
            zip: ZipState::default(),
            media_output_credit: None,
            output_chunks: ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES),
            commit_cursor: 0,
            completed_units: 0,
            restore_target: None,
            snapshot_close: None,
            snapshot_placeholder: Some(Arc::new(empty_close_snapshot())),
            publication: None,
            closing: false,
            close_stage: LayoutExportCloseStage::JsonValidation,
        })
    }

    pub fn with_snapshot_close_lease(mut self, snapshot_close: ArtifactSnapshotCloseLease<LayoutSnapshot>) -> Self {
        self.snapshot_close = Some(snapshot_close);
        self
    }

    pub fn with_media_output_credit(mut self, credit: ArtifactMediaExportCredit) -> Self {
        self.media_output_credit = Some(credit);
        self
    }

    pub fn with_output_chunks(mut self, output_chunks: ArtifactOutputChunks) -> Self {
        self.output_chunks = output_chunks;
        self
    }

    fn close_pending(released_items: usize, released_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(PluginCloseStep::Pending { released_items, released_bytes })
    }

    fn close_string(value: &mut String, maximum_bytes: usize) -> Option<usize> {
        if value.is_empty() {
            return None;
        }
        let mut start = value.len().saturating_sub(maximum_bytes.min(value.len()));
        while start < value.len() && !value.is_char_boundary(start) {
            start += 1;
        }
        if start == value.len() {
            return Some(0);
        }
        let released = value.len() - start;
        value.truncate(start);
        Some(released)
    }

    fn close_byte_buffer(value: &mut Vec<u8>, maximum_bytes: usize) -> Option<usize> {
        if value.is_empty() {
            return None;
        }
        let released = maximum_bytes.min(value.len());
        value.truncate(value.len() - released);
        Some(released)
    }

    fn close_usize_buffer(value: &mut Vec<usize>, maximum_bytes: usize) -> usize {
        let released_items = (maximum_bytes / std::mem::size_of::<usize>()).min(value.len());
        value.truncate(value.len() - released_items);
        released_items.saturating_mul(std::mem::size_of::<usize>())
    }

    fn close_string_source(source: &mut StringSource, maximum_bytes: usize) -> usize {
        match source {
            StringSource::DrawNodeValue { path, .. } | StringSource::DrawNodeStyle { path, .. } | StringSource::DrawNodeMime { path, .. } => Self::close_usize_buffer(path, maximum_bytes),
            _ => 0,
        }
    }

    fn close_typed_node_payload(node: &mut TypedJsonNode, maximum_bytes: usize) -> usize {
        match node {
            TypedJsonNode::Scalar { bytes, .. } => Self::close_byte_buffer(bytes, maximum_bytes).unwrap_or(0),
            TypedJsonNode::String { source, .. } => Self::close_string_source(source, maximum_bytes),
            TypedJsonNode::OwnedString { value, .. } => Self::close_string(value, maximum_bytes).unwrap_or(0),
            TypedJsonNode::DrawNode { path, .. } | TypedJsonNode::DrawChildren { path, .. } | TypedJsonNode::DrawSegments { path, .. } | TypedJsonNode::DrawSegment { path, .. } | TypedJsonNode::DrawBytes { path, .. } => {
                Self::close_usize_buffer(path, maximum_bytes)
            }
            _ => 0,
        }
    }

    fn close_json_cursor(cursor: &mut Option<JsonValidationCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage) -> Result<PluginCloseStep, Fault> {
        if let Some(cursor) = cursor.as_mut() {
            if cursor.stack.pop().is_some() {
                return Self::close_pending(1, 0);
            }
            debug_assert!(cursor.stack.is_empty());
        }
        drop(cursor.take());
        *stage = next;
        Self::close_pending(1, 0)
    }

    fn close_typed_cursor(cursor: &mut Option<TypedJsonCursor>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if let Some(cursor) = cursor.as_mut() {
            if let Some(node) = cursor.stack.last() {
                let bytes = typed_json_node_owned_bytes(node);
                if bytes > maximum_bytes {
                    let released = Self::close_typed_node_payload(cursor.stack.last_mut().expect("typed close node remains owned"), maximum_bytes);
                    return Self::close_pending(usize::from(released != 0), released);
                }
                drop(cursor.stack.pop());
                return Self::close_pending(1, bytes);
            }
            debug_assert!(cursor.stack.is_empty());
        }
        drop(cursor.take());
        *stage = next;
        Self::close_pending(1, 0)
    }

    fn close_optional_string(value: &mut Option<String>, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if let Some(value) = value.as_mut() {
            if let Some(released) = Self::close_string(value, maximum_bytes) {
                return Self::close_pending(usize::from(released != 0), released);
            }
            debug_assert!(value.is_empty());
        }
        drop(value.take());
        *stage = next;
        Self::close_pending(1, 0)
    }

    fn close_required_string(value: &mut String, next: LayoutExportCloseStage, stage: &mut LayoutExportCloseStage, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if let Some(released) = Self::close_string(value, maximum_bytes) {
            return Self::close_pending(usize::from(released != 0), released);
        }
        debug_assert!(value.is_empty());
        drop(std::mem::take(value));
        *stage = next;
        Self::close_pending(1, 0)
    }

    fn close_export_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Self::close_pending(0, 0);
        }
        match self.close_stage {
            LayoutExportCloseStage::Publication => Err(Fault::from("layout-export-publication-close-not-dispatched")),
            LayoutExportCloseStage::JsonValidation => Self::close_json_cursor(&mut self.json_validation, LayoutExportCloseStage::TypedValidation, &mut self.close_stage),
            LayoutExportCloseStage::TypedValidation => Self::close_typed_cursor(&mut self.typed_validation, LayoutExportCloseStage::PackageJson, &mut self.close_stage, maximum_bytes),
            LayoutExportCloseStage::PackageJson => Self::close_typed_cursor(&mut self.package_json, LayoutExportCloseStage::Rects, &mut self.close_stage, maximum_bytes),
            LayoutExportCloseStage::Rects => {
                if self.rects.pop().is_some() {
                    return Self::close_pending(1, 0);
                }
                debug_assert!(self.rects.is_empty());
                drop(std::mem::take(&mut self.rects));
                self.close_stage = LayoutExportCloseStage::Output;
                Self::close_pending(1, 0)
            }
            LayoutExportCloseStage::Output => match self.output.close_take_chunk(maximum_bytes) {
                Some(Ok(bytes)) => Self::close_pending(1, bytes),
                Some(Err(())) => Self::close_pending(0, 0),
                None => {
                    debug_assert!(self.output.chunks.is_empty());
                    drop(std::mem::take(&mut self.output.chunks));
                    self.close_stage = LayoutExportCloseStage::Encoded;
                    Self::close_pending(1, 0)
                }
            },
            LayoutExportCloseStage::Encoded => match self.encoded.close_take_chunk(maximum_bytes) {
                Some(Ok(bytes)) => Self::close_pending(1, bytes),
                Some(Err(())) => Self::close_pending(0, 0),
                None => {
                    debug_assert!(self.encoded.chunks.is_empty());
                    drop(std::mem::take(&mut self.encoded.chunks));
                    self.close_stage = LayoutExportCloseStage::Base64Tail;
                    Self::close_pending(1, 0)
                }
            },
            LayoutExportCloseStage::Base64Tail => {
                if let Some(released) = Self::close_byte_buffer(&mut self.base64_tail, maximum_bytes) {
                    return Self::close_pending(usize::from(released != 0), released);
                }
                debug_assert!(self.base64_tail.is_empty());
                drop(std::mem::take(&mut self.base64_tail));
                self.close_stage = LayoutExportCloseStage::PngRow;
                Self::close_pending(1, 0)
            }
            LayoutExportCloseStage::PngRow => {
                if let Some(released) = Self::close_byte_buffer(&mut self.png_row, maximum_bytes) {
                    return Self::close_pending(usize::from(released != 0), released);
                }
                debug_assert!(self.png_row.is_empty());
                drop(std::mem::take(&mut self.png_row));
                self.close_stage = LayoutExportCloseStage::PdfOffsets;
                Self::close_pending(1, 0)
            }
            LayoutExportCloseStage::PdfOffsets => {
                if self.pdf_offsets.pop().is_some() {
                    return Self::close_pending(1, 0);
                }
                debug_assert!(self.pdf_offsets.is_empty());
                drop(std::mem::take(&mut self.pdf_offsets));
                self.close_stage = LayoutExportCloseStage::ZipEntries;
                Self::close_pending(1, 0)
            }
            LayoutExportCloseStage::ZipEntries => {
                let bytes = self.zip.entries.last().map_or(0, |entry| entry.name.len());
                if bytes > maximum_bytes {
                    let released = Self::close_string(&mut self.zip.entries.last_mut().expect("zip close entry remains owned").name, maximum_bytes).unwrap_or(0);
                    return Self::close_pending(usize::from(released != 0), released);
                }
                if self.zip.entries.pop().is_some() {
                    return Self::close_pending(1, bytes);
                }
                debug_assert!(self.zip.entries.is_empty());
                drop(std::mem::take(&mut self.zip.entries));
                self.close_stage = LayoutExportCloseStage::ZipCurrentName;
                Self::close_pending(1, 0)
            }
            LayoutExportCloseStage::ZipCurrentName => Self::close_optional_string(&mut self.zip.current_name, LayoutExportCloseStage::PageAuthority, &mut self.close_stage, maximum_bytes),
            LayoutExportCloseStage::PageAuthority => Self::close_optional_string(&mut self.request.page_id, LayoutExportCloseStage::Preflight, &mut self.close_stage, maximum_bytes),
            LayoutExportCloseStage::Preflight => Self::close_optional_string(&mut self.request.preflight_json, LayoutExportCloseStage::ParentAuthority, &mut self.close_stage, maximum_bytes),
            LayoutExportCloseStage::ParentAuthority => Self::close_required_string(&mut self.request.parent_document_id, LayoutExportCloseStage::RevisionAuthority, &mut self.close_stage, maximum_bytes),
            LayoutExportCloseStage::RevisionAuthority => Self::close_required_string(&mut self.request.canonical_base_revision_hex, LayoutExportCloseStage::OutputChunks, &mut self.close_stage, maximum_bytes),
            LayoutExportCloseStage::OutputChunks => {
                if maximum_bytes < OUTPUT_CHUNK_BYTES {
                    return Self::close_pending(0, 0);
                }
                match self.output_chunks.close_take_chunk()? {
                    Some(chunk) => Self::close_pending(1, chunk.len()),
                    None => {
                        self.close_stage = LayoutExportCloseStage::MediaCredit;
                        Self::close_pending(1, 0)
                    }
                }
            }
            LayoutExportCloseStage::MediaCredit => {
                drop(self.media_output_credit.take());
                self.close_stage = LayoutExportCloseStage::Snapshot;
                Self::close_pending(1, 0)
            }
            LayoutExportCloseStage::Snapshot => {
                if let Some(lease) = self.snapshot_close.as_ref() {
                    if !lease.can_release(&self.request.snapshot) {
                        return Err(Fault::from("layout-export-close-snapshot-owner-missing"));
                    }
                } else if Arc::strong_count(&self.request.snapshot) <= 1 {
                    return Err(Fault::from("layout-export-close-snapshot-unwitnessed"));
                }
                let snapshot = std::mem::replace(&mut self.request.snapshot, self.snapshot_placeholder.take().expect("layout close owns pre-admitted snapshot placeholder"));
                drop(snapshot);
                self.close_stage = LayoutExportCloseStage::SnapshotOwner;
                Self::close_pending(1, 0)
            }
            LayoutExportCloseStage::SnapshotOwner => {
                drop(self.snapshot_close.take());
                self.close_stage = LayoutExportCloseStage::Complete;
                Self::close_pending(1, 0)
            }
            LayoutExportCloseStage::Complete => Ok(PluginCloseStep::Complete),
        }
    }

    fn close_terminal_is_empty(&self) -> bool {
        self.close_stage == LayoutExportCloseStage::Complete
            && self.json_validation.is_none()
            && self.typed_validation.is_none()
            && self.package_json.is_none()
            && self.rects.is_empty()
            && self.output.chunks.is_empty()
            && self.encoded.chunks.is_empty()
            && self.base64_tail.is_empty()
            && self.png_row.is_empty()
            && self.pdf_offsets.is_empty()
            && self.zip.entries.is_empty()
            && self.zip.current_name.is_none()
            && self.request.page_id.is_none()
            && self.request.preflight_json.is_none()
            && self.request.parent_document_id.is_empty()
            && self.request.canonical_base_revision_hex.is_empty()
            && self.output_chunks.chunks_remaining() == 0
            && self.media_output_credit.is_none()
            && self.snapshot_close.is_none()
            && self.snapshot_placeholder.is_none()
    }

    pub fn restore(operation: Operation, request: LayoutExportRequest, state: &[u8]) -> Result<Self, String> {
        let checkpoint = decode_checkpoint(&operation, &request, state)?;
        let mut job = Self::new(operation, request)?;
        job.restore_target = Some(checkpoint);
        Ok(job)
    }

    fn active_output(&self) -> &ChunkRope {
        if self.request.kind.binary() && matches!(self.stage, ExportStage::Base64 | ExportStage::PackageCommit | ExportStage::Complete) {
            &self.encoded
        } else {
            &self.output
        }
    }

    fn checkpoint_publication(&self) -> Result<LayoutExportPublication, String> {
        let checkpoint = LayoutExportCheckpoint { completed_units: self.completed_units, output_bytes: self.active_output().len as u64, output_digest: self.active_output().digest };
        let state = encode_checkpoint(&self.operation, &self.request, &checkpoint)?;
        LayoutExportPublication::new(LayoutExportPublicationKind::Checkpoint { applied_progress: self.completed_units }, &state)
    }

    fn preview_publication(&self) -> Result<LayoutExportPublication, String> {
        LayoutExportPublication::new(LayoutExportPublicationKind::Preview, self.stage_name().as_bytes())
    }

    fn fault_publication(error: &str) -> LayoutExportPublication {
        let bytes = &error.as_bytes()[..error.len().min(MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES)];
        LayoutExportPublication::new(LayoutExportPublicationKind::Fault, bytes).expect("bounded layout fault publication")
    }

    fn drive_publication(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        let outcome = self.publication.as_mut().expect("scheduled layout publication").step(context);
        if !matches!(outcome, StepOutcome::Yield) {
            self.publication = None;
        }
        outcome
    }

    fn page(&self) -> Result<&Page, String> {
        self.target_page.and_then(|index| self.request.snapshot.pages.get(index)).ok_or_else(|| "layout-export-page-missing".into())
    }

    fn validate_one(&mut self) -> Result<(), String> {
        let snapshot = &self.request.snapshot;
        match self.validation_group {
            0 => {
                if snapshot.pages.len() > MAX_LAYOUT_EXPORT_PAGES
                    || snapshot.stories.len() > MAX_LAYOUT_EXPORT_STORIES
                    || snapshot.links.len() > MAX_LAYOUT_EXPORT_LINKS
                    || snapshot.parent_pages.len() > MAX_LAYOUT_EXPORT_PARENT_PAGES
                    || snapshot.spreads.len() > MAX_LAYOUT_EXPORT_SPREADS
                    || snapshot.paragraph_styles.len() > MAX_LAYOUT_EXPORT_STYLES
                    || snapshot.character_styles.len() > MAX_LAYOUT_EXPORT_STYLES
                    || !bounded_string(&snapshot.schema)
                    || !bounded_string(&snapshot.name)
                {
                    return Err("layout-export-document-envelope".into());
                }
                self.validation_total_items = snapshot.pages.len() + snapshot.stories.len() + snapshot.links.len() + snapshot.parent_pages.len() + snapshot.spreads.len() + snapshot.paragraph_styles.len() + snapshot.character_styles.len();
                self.validation_group = 1;
            }
            1 => match snapshot.pages.get(self.validation_index) {
                Some(page) => {
                    let width = finite_dimension(page.width)?;
                    let height = finite_dimension(page.height)?;
                    if u64::from(width) * u64::from(height) > MAX_LAYOUT_EXPORT_PIXELS
                        || page.frames.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || page.overrides.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || page.guides.len() > MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE
                        || page.layers.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || page.layer_ids.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || !bounded_id(&page.id)
                        || !bounded_string(&page.name)
                        || !bounded_id(&page.spread_id)
                        || page.parent_page_id.as_ref().is_some_and(|id| !bounded_id(id))
                        || page.frames.iter().any(|frame| !valid_frame(frame))
                        || page.overrides.iter().any(|value| !bounded_id(&value.object_id) || value.bounds.as_ref().is_some_and(|bounds| !valid_bounds(bounds)))
                        || page.guides.iter().any(|guide| !valid_rect(guide))
                        || !page.margins.top.is_finite()
                        || !page.margins.right.is_finite()
                        || !page.margins.bottom.is_finite()
                        || !page.margins.left.is_finite()
                        || page.columns.count as usize > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || !page.columns.gutter.is_finite()
                        || page.layers.iter().any(|layer| layer.object_ids.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE || !bounded_id(&layer.id) || !bounded_string(&layer.name) || layer.object_ids.iter().any(|id| !bounded_id(id)))
                        || page.layer_ids.iter().any(|id| !bounded_id(id))
                    {
                        return Err("layout-export-page-envelope".into());
                    }
                    let nested = page.frames.len() + page.overrides.len() + page.guides.len() + page.layers.len() + page.layer_ids.len() + page.layers.iter().map(|layer| layer.object_ids.len()).sum::<usize>();
                    self.validation_total_frames = self.validation_total_frames.checked_add(page.frames.len()).ok_or("layout-export-item-limit")?;
                    add_validated_items(&mut self.validation_total_items, nested)?;
                    if self.request.page_id.as_deref().is_some_and(|id| id == page.id) || self.request.page_id.is_none() && self.target_page.is_none() {
                        self.target_page = Some(self.validation_index);
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 2;
                    self.validation_index = 0;
                }
            },
            2 => match snapshot.stories.get(self.validation_index) {
                Some(story) => {
                    if !bounded_id(&story.id)
                        || !bounded_string(&story.content)
                        || story.style_runs.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || story.style_runs.iter().any(|run| run.paragraph_style_id.as_ref().is_some_and(|id| !bounded_id(id)) || run.character_style_id.as_ref().is_some_and(|id| !bounded_id(id)))
                    {
                        return Err("layout-export-story-envelope".into());
                    }
                    add_validated_items(&mut self.validation_total_items, story.style_runs.len())?;
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 3;
                    self.validation_index = 0;
                }
            },
            3 => match snapshot.links.get(self.validation_index) {
                Some(link) => {
                    if !bounded_id(&link.id)
                        || !bounded_string(&link.path)
                        || link.hash.len() > 256
                        || link.width > MAX_LAYOUT_EXPORT_DIMENSION
                        || link.height > MAX_LAYOUT_EXPORT_DIMENSION
                        || u64::from(link.width) * u64::from(link.height) > MAX_LAYOUT_EXPORT_PIXELS
                        || link.dpi > 9_600
                        || link.proxy_data_url.as_ref().is_some_and(|value| !bounded_string(value))
                        || link.color_profile.as_ref().is_some_and(|value| !bounded_string(value))
                        || link.state.as_ref().is_some_and(|value| !bounded_string(value))
                    {
                        return Err("layout-export-link-envelope".into());
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 4;
                    self.validation_index = 0;
                }
            },
            4 => match snapshot.parent_pages.get(self.validation_index) {
                Some(parent) => {
                    finite_dimension(parent.width)?;
                    finite_dimension(parent.height)?;
                    if parent.frames.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || parent.layers.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || parent.layer_ids.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || !bounded_id(&parent.id)
                        || !bounded_string(&parent.name)
                        || parent.frames.iter().any(|frame| !valid_frame(frame))
                        || parent.layers.iter().any(|layer| layer.object_ids.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE || !bounded_id(&layer.id) || !bounded_string(&layer.name) || layer.object_ids.iter().any(|id| !bounded_id(id)))
                        || parent.layer_ids.iter().any(|id| !bounded_id(id))
                    {
                        return Err("layout-export-parent-page-envelope".into());
                    }
                    let nested = parent.frames.len() + parent.layers.len() + parent.layer_ids.len() + parent.layers.iter().map(|layer| layer.object_ids.len()).sum::<usize>();
                    self.validation_total_frames = self.validation_total_frames.checked_add(parent.frames.len()).ok_or("layout-export-item-limit")?;
                    add_validated_items(&mut self.validation_total_items, nested)?;
                    self.validation_index += 1;
                }
                None => {
                    if self.validation_total_frames > MAX_LAYOUT_EXPORT_TOTAL_FRAMES {
                        return Err("layout-export-frame-envelope".into());
                    }
                    self.validation_group = 5;
                    self.validation_index = 0;
                }
            },
            5 => match snapshot.paragraph_styles.get(self.validation_index) {
                Some(style) => {
                    if !bounded_id(&style.id) || !bounded_string(&style.name) || !bounded_string(&style.font_family) || !bounded_string(&style.alignment) || !style.font_size.is_finite() || !style.leading.is_finite() || !style.tracking.is_finite() {
                        return Err("layout-export-paragraph-style-envelope".into());
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 6;
                    self.validation_index = 0;
                }
            },
            6 => match snapshot.character_styles.get(self.validation_index) {
                Some(style) => {
                    if !bounded_id(&style.id)
                        || style.name.as_ref().is_some_and(|value| !bounded_string(value))
                        || style.font_family.as_ref().is_some_and(|value| !bounded_string(value))
                        || style.font_size.is_some_and(|value| !value.is_finite())
                        || style.tracking.is_some_and(|value| !value.is_finite())
                        || style.color.is_some_and(|color| color.iter().any(|channel| !channel.is_finite()))
                    {
                        return Err("layout-export-character-style-envelope".into());
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 7;
                    self.validation_index = 0;
                }
            },
            7 => match snapshot.spreads.get(self.validation_index) {
                Some(spread) => {
                    if spread.page_ids.len() > MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS || !bounded_id(&spread.id) || !bounded_string(&spread.name) || spread.page_ids.iter().any(|id| !bounded_id(id)) {
                        return Err("layout-export-spread-envelope".into());
                    }
                    add_validated_items(&mut self.validation_total_items, spread.page_ids.len())?;
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 8;
                    self.validation_index = 0;
                }
            },
            8 => {
                if snapshot.print_target.as_ref().is_some_and(|value| !bounded_string(value)) {
                    return Err("layout-export-print-target-envelope".into());
                }
                if !snapshot.grid.baseline_grid.is_finite() || !snapshot.grid.baseline_offset.is_finite() {
                    return Err("layout-export-grid-envelope".into());
                }
                self.validation_group = 9;
            }
            9 => {
                let Some(value) = &snapshot.data_fields_json else {
                    self.validation_group = 10;
                    return Ok(());
                };
                if self.json_validation.is_none() {
                    self.json_validation = Some(JsonValidationCursor::new(value, false)?);
                }
                if self.json_validation.as_mut().ok_or("layout-export-json-validator")?.advance(value)? {
                    self.json_validation = None;
                    self.validation_group = 10;
                }
            }
            10 => {
                let Some(value) = &self.request.preflight_json else {
                    self.validation_group = 11;
                    return Ok(());
                };
                if self.json_validation.is_none() {
                    self.json_validation = Some(JsonValidationCursor::new(value, true)?);
                }
                if self.json_validation.as_mut().ok_or("layout-export-json-validator")?.advance(value)? {
                    self.json_validation = None;
                    self.validation_group = 11;
                }
            }
            11 => {
                if snapshot.background_drawing.is_none() {
                    self.validation_group = 12;
                    return Ok(());
                }
                if self.typed_validation.is_none() {
                    self.typed_validation = Some(TypedJsonCursor::validating(TypedJsonNode::BackgroundDrawing));
                }
                if self.typed_validation.as_mut().ok_or("layout-export-json-validator")?.advance(snapshot)?.1 {
                    self.typed_validation = None;
                    self.validation_group = 12;
                }
            }
            12 => {
                if snapshot.referenced_model.is_none() {
                    self.validation_group = 13;
                    return Ok(());
                }
                if self.typed_validation.is_none() {
                    self.typed_validation = Some(TypedJsonCursor::validating(TypedJsonNode::ReferencedModel));
                }
                if self.typed_validation.as_mut().ok_or("layout-export-json-validator")?.advance(snapshot)?.1 {
                    self.typed_validation = None;
                    self.validation_group = 13;
                }
            }
            13 => {
                if !matches!(self.request.kind, LayoutExportKind::Package) && self.target_page.is_none() {
                    return Err("layout-export-page-missing".into());
                }
                self.parent_frame_count =
                    self.target_page.and_then(|index| snapshot.pages.get(index)).and_then(|page| page.parent_page_id.as_ref().and_then(|id| snapshot.parent_pages.iter().find(|parent| &parent.id == id))).map_or(0, |parent| parent.frames.len());
                if matches!(self.request.kind, LayoutExportKind::Package) {
                    self.begin_zip_entry("document.json")?;
                    self.stage = ExportStage::Encode;
                } else {
                    self.stage = ExportStage::Plan;
                }
            }
            _ => unreachable!("closed validation state"),
        }
        Ok(())
    }

    fn plan_one(&mut self) -> Result<(), String> {
        let page = self.page()?;
        let parent = page.parent_page_id.as_ref().and_then(|id| self.request.snapshot.parent_pages.iter().find(|candidate| &candidate.id == id));
        let frame = if self.plan_cursor < self.parent_frame_count { parent.and_then(|value| value.frames.get(self.plan_cursor)) } else { page.frames.get(self.plan_cursor - self.parent_frame_count) };
        let Some(frame) = frame else {
            self.begin_encoder()?;
            self.stage = ExportStage::Encode;
            return Ok(());
        };
        let inherited = self.plan_cursor < self.parent_frame_count;
        let override_value = if inherited { override_for(page, frame) } else { None };
        if let Some(rect) = export_rect(frame, override_value, finite_dimension(page.width)?, finite_dimension(page.height)?) {
            self.rects.push(rect);
        }
        self.plan_cursor += 1;
        Ok(())
    }

    fn begin_encoder(&mut self) -> Result<(), String> {
        match self.request.kind {
            LayoutExportKind::Svg => {
                let page = self.page()?;
                let width = finite_dimension(page.width)?;
                let height = finite_dimension(page.height)?;
                self.output.append(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\"><rect width=\"100%\" height=\"100%\" fill=\"white\"/>").as_bytes())?;
            }
            LayoutExportKind::Pdf => self.begin_pdf()?,
            LayoutExportKind::Png => self.begin_png()?,
            LayoutExportKind::Package => unreachable!("package begins after validation"),
        }
        Ok(())
    }

    fn encode_one(&mut self) -> Result<(), String> {
        match self.request.kind {
            LayoutExportKind::Svg => self.encode_svg_one(),
            LayoutExportKind::Pdf => self.encode_pdf_one(),
            LayoutExportKind::Png => self.encode_png_one(),
            LayoutExportKind::Package => self.encode_package_one(),
        }
    }

    fn encode_svg_one(&mut self) -> Result<(), String> {
        if let Some(rect) = self.rects.get(self.encode_cursor) {
            self.output.append(
                format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{:.4}\"/>", rect.x, rect.y, rect.width, rect.height, rect.rgba[0], rect.rgba[1], rect.rgba[2], f32::from(rect.rgba[3]) / 255.0)
                    .as_bytes(),
            )?;
            self.encode_cursor += 1;
        } else {
            self.output.append(b"</svg>")?;
            self.stage = ExportStage::PackageCommit;
        }
        Ok(())
    }

    fn begin_pdf(&mut self) -> Result<(), String> {
        self.output.append(b"%PDF-1.4\n")?;
        self.pdf_section = PdfSection::Catalog;
        Ok(())
    }

    fn pdf_object(&mut self, id: usize, body: &[u8]) -> Result<(), String> {
        if self.pdf_offsets.len() + 1 != id {
            return Err("layout-export-pdf-object-order".into());
        }
        self.pdf_offsets.push(u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?);
        self.output.append(format!("{id} 0 obj\n").as_bytes())?;
        self.output.append(body)?;
        self.output.append(b"\nendobj\n")
    }

    fn encode_pdf_one(&mut self) -> Result<(), String> {
        match self.pdf_section {
            PdfSection::Header => return Err("layout-export-pdf-header-state".into()),
            PdfSection::Catalog => {
                self.pdf_object(1, b"<< /Type /Catalog /Pages 2 0 R >>")?;
                self.pdf_section = PdfSection::Pages;
            }
            PdfSection::Pages => {
                self.pdf_object(2, b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>")?;
                self.pdf_section = PdfSection::PageBegin;
            }
            PdfSection::PageBegin => {
                let (width, height) = {
                    let page = self.page()?;
                    (finite_dimension(page.width)?, finite_dimension(page.height)?)
                };
                self.pdf_offsets.push(u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?);
                self.output.append(format!("3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {width} {height}] /Contents [").as_bytes())?;
                self.pdf_section = PdfSection::PageKids;
            }
            PdfSection::PageKids => {
                if self.pdf_cursor < self.rects.len() {
                    self.output.append(format!("{} 0 R ", self.pdf_cursor + 4).as_bytes())?;
                    self.pdf_cursor += 1;
                } else {
                    self.pdf_section = PdfSection::PageEnd;
                }
            }
            PdfSection::PageEnd => {
                self.output.append(b"] >>\nendobj\n")?;
                self.pdf_section = PdfSection::Rects;
            }
            PdfSection::Rects => {
                if let Some(rect) = self.rects.get(self.encode_cursor).cloned() {
                    let content = format!("q {:.6} {:.6} {:.6} rg {} {} {} {} re f Q\n", f32::from(rect.rgba[0]) / 255.0, f32::from(rect.rgba[1]) / 255.0, f32::from(rect.rgba[2]) / 255.0, rect.x, rect.y, rect.width, rect.height);
                    let body = format!("<< /Length {} >>\nstream\n{}endstream", content.len(), content);
                    self.pdf_object(self.encode_cursor + 4, body.as_bytes())?;
                    self.encode_cursor += 1;
                } else {
                    self.pdf_section = PdfSection::XrefBegin;
                }
            }
            PdfSection::XrefBegin => {
                self.pdf_xref_offset = self.output.len;
                self.output.append(format!("xref\n0 {}\n0000000000 65535 f \n", self.pdf_offsets.len() + 1).as_bytes())?;
                self.pdf_cursor = 0;
                self.pdf_section = PdfSection::XrefEntries;
            }
            PdfSection::XrefEntries => {
                if let Some(offset) = self.pdf_offsets.get(self.pdf_cursor).copied() {
                    self.output.append(format!("{offset:010} 00000 n \n").as_bytes())?;
                    self.pdf_cursor += 1;
                } else {
                    self.pdf_section = PdfSection::Trailer;
                }
            }
            PdfSection::Trailer => {
                self.output.append(format!("trailer<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n", self.pdf_offsets.len() + 1, self.pdf_xref_offset).as_bytes())?;
                self.pdf_section = PdfSection::Complete;
            }
            PdfSection::Complete => self.stage = ExportStage::Base64,
        }
        Ok(())
    }

    fn begin_png(&mut self) -> Result<(), String> {
        let page = self.page()?;
        let width = finite_dimension(page.width)?;
        let height = finite_dimension(page.height)?;
        self.output.append(b"\x89PNG\r\n\x1a\n")?;
        let mut ihdr = Vec::with_capacity(13);
        ihdr.extend_from_slice(&width.to_be_bytes());
        ihdr.extend_from_slice(&height.to_be_bytes());
        ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
        self.png_chunk(b"IHDR", &ihdr)?;
        let row_bytes = width as usize * 4 + 1;
        let idat_len = 2usize.checked_add(height as usize * (row_bytes + 5)).and_then(|value| value.checked_add(4)).ok_or("layout-export-output-limit")?;
        self.output.append(&u32::try_from(idat_len).map_err(|_| "layout-export-output-limit")?.to_be_bytes())?;
        self.output.append(b"IDAT")?;
        self.png_idat_crc = crc32_update(0, b"IDAT");
        self.append_idat(&[0x78, 0x01])
    }

    fn png_chunk(&mut self, kind: &[u8; 4], data: &[u8]) -> Result<(), String> {
        self.output.append(&u32::try_from(data.len()).map_err(|_| "layout-export-output-limit")?.to_be_bytes())?;
        self.output.append(kind)?;
        self.output.append(data)?;
        self.output.append(&crc32_update(crc32_update(0, kind), data).to_be_bytes())
    }

    fn append_idat(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.png_idat_crc = crc32_update(self.png_idat_crc, bytes);
        self.output.append(bytes)
    }

    fn encode_png_one(&mut self) -> Result<(), String> {
        let (width, height) = {
            let page = self.page()?;
            (finite_dimension(page.width)?, finite_dimension(page.height)?)
        };
        if self.encode_cursor < height as usize {
            let y = self.encode_cursor as u32;
            match self.png_row_section {
                PngRowSection::Initialize => {
                    let row_len = width as usize * 4 + 1;
                    if self.png_row.is_empty() {
                        self.png_row = Vec::with_capacity(row_len);
                        self.png_row.push(0);
                    }
                    let end = self.png_row.len().saturating_add(OUTPUT_CHUNK_BYTES).min(row_len);
                    self.png_row.resize(end, 255);
                    if end == row_len {
                        self.png_rect_cursor = 0;
                        self.png_fill_x = 0;
                        self.png_row_section = PngRowSection::Fill;
                    }
                    return Ok(());
                }
                PngRowSection::Fill => {
                    if let Some(rect) = self.rects.get(self.png_rect_cursor) {
                        if y < rect.y || y >= rect.y + rect.height {
                            self.png_rect_cursor += 1;
                            self.png_fill_x = 0;
                            return Ok(());
                        }
                        let start = self.png_fill_x.max(rect.x);
                        let rect_end = rect.x + rect.width;
                        let end = start.saturating_add(PNG_PIXELS_PER_UNIT).min(rect_end);
                        for x in start..end {
                            let index = x as usize * 4 + 1;
                            self.png_row[index..index + 4].copy_from_slice(&rect.rgba);
                        }
                        if end == rect_end {
                            self.png_rect_cursor += 1;
                            self.png_fill_x = 0;
                        } else {
                            self.png_fill_x = end;
                        }
                    } else {
                        self.png_row_section = PngRowSection::Header;
                    }
                    return Ok(());
                }
                PngRowSection::Header => {
                    let len = u16::try_from(self.png_row.len()).map_err(|_| "layout-export-png-row-limit")?;
                    let mut header = [0u8; 5];
                    header[0] = u8::from(self.encode_cursor + 1 == height as usize);
                    header[1..3].copy_from_slice(&len.to_le_bytes());
                    header[3..5].copy_from_slice(&(!len).to_le_bytes());
                    self.append_idat(&header)?;
                    self.png_row_byte_cursor = 0;
                    self.png_row_section = PngRowSection::Data;
                    return Ok(());
                }
                PngRowSection::Data => {
                    let end = self.png_row_byte_cursor.saturating_add(OUTPUT_CHUNK_BYTES).min(self.png_row.len());
                    if self.png_row_byte_cursor < end {
                        let bytes = &self.png_row[self.png_row_byte_cursor..end];
                        for byte in bytes {
                            self.png_adler_s1 = (self.png_adler_s1 + u32::from(*byte)) % 65_521;
                            self.png_adler_s2 = (self.png_adler_s2 + self.png_adler_s1) % 65_521;
                        }
                        self.png_idat_crc = crc32_update(self.png_idat_crc, bytes);
                        self.output.append(bytes)?;
                        self.png_row_byte_cursor = end;
                        return Ok(());
                    }
                    self.png_row.clear();
                    self.png_row_section = PngRowSection::Initialize;
                    self.png_row_byte_cursor = 0;
                    self.png_rect_cursor = 0;
                    self.encode_cursor += 1;
                    return Ok(());
                }
            }
        }
        let adler = (self.png_adler_s2 << 16) | self.png_adler_s1;
        self.append_idat(&adler.to_be_bytes())?;
        self.output.append(&self.png_idat_crc.to_be_bytes())?;
        self.png_chunk(b"IEND", &[])?;
        self.stage = ExportStage::Base64;
        Ok(())
    }

    fn begin_zip_entry(&mut self, name: &str) -> Result<(), String> {
        if self.zip.current_name.is_some() || self.zip.entries.len() >= MAX_LAYOUT_EXPORT_FILES {
            return Err("layout-export-package-entry-state".into());
        }
        let mut header = Vec::with_capacity(30 + name.len());
        push_u32(&mut header, 0x04034b50);
        push_u16(&mut header, 20);
        push_u16(&mut header, 0x0008);
        push_u16(&mut header, 0);
        push_u16(&mut header, 0);
        push_u16(&mut header, 0);
        push_u32(&mut header, 0);
        push_u32(&mut header, 0);
        push_u32(&mut header, 0);
        push_u16(&mut header, u16::try_from(name.len()).map_err(|_| "layout-export-package-name-limit")?);
        push_u16(&mut header, 0);
        header.extend_from_slice(name.as_bytes());
        self.zip.current_offset = u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?;
        self.output.append(&header)?;
        self.zip.current_name = Some(name.into());
        self.zip.current_crc = 0;
        self.zip.current_size = 0;
        Ok(())
    }

    fn append_zip_data(&mut self, bytes: &[u8]) -> Result<(), String> {
        if self.zip.current_name.is_none() {
            return Err("layout-export-package-no-entry".into());
        }
        self.zip.current_crc = crc32_update(self.zip.current_crc, bytes);
        self.zip.current_size = self.zip.current_size.checked_add(u32::try_from(bytes.len()).map_err(|_| "layout-export-output-limit")?).ok_or("layout-export-output-limit")?;
        self.output.append(bytes)
    }

    fn end_zip_entry(&mut self) -> Result<(), String> {
        let name = self.zip.current_name.take().ok_or("layout-export-package-no-entry")?;
        let mut descriptor = Vec::with_capacity(16);
        push_u32(&mut descriptor, 0x08074b50);
        push_u32(&mut descriptor, self.zip.current_crc);
        push_u32(&mut descriptor, self.zip.current_size);
        push_u32(&mut descriptor, self.zip.current_size);
        self.output.append(&descriptor)?;
        self.zip.entries.push(ZipEntry { name, crc: self.zip.current_crc, size: self.zip.current_size, offset: self.zip.current_offset });
        Ok(())
    }

    fn encode_package_one(&mut self) -> Result<(), String> {
        match self.package_section {
            PackageSection::Begin => {
                self.package_json = Some(TypedJsonCursor::document());
                self.package_section = PackageSection::Scalars;
            }
            PackageSection::Scalars => {
                let (bytes, done) = self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.snapshot)?;
                if !bytes.is_empty() {
                    self.append_zip_data(&bytes)?;
                }
                if done {
                    self.package_json = None;
                    self.end_zip_entry()?;
                    self.begin_zip_entry("preflight-report.json")?;
                    self.package_byte_cursor = 0;
                    if self.request.preflight_json.is_none() {
                        self.package_json = Some(TypedJsonCursor::preflight());
                    }
                    self.package_section = PackageSection::Preflight;
                }
            }
            PackageSection::Preflight => {
                if let Some(preflight) = &self.request.preflight_json {
                    let end = self.package_byte_cursor.saturating_add(OUTPUT_CHUNK_BYTES).min(preflight.len());
                    if self.package_byte_cursor < end {
                        let bytes = preflight.as_bytes()[self.package_byte_cursor..end].to_vec();
                        self.append_zip_data(&bytes)?;
                        self.package_byte_cursor = end;
                    } else {
                        self.finish_preflight_entry()?;
                    }
                    return Ok(());
                }
                let (bytes, done) = self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.snapshot)?;
                if !bytes.is_empty() {
                    self.append_zip_data(&bytes)?;
                }
                if done {
                    self.package_json = None;
                    self.finish_preflight_entry()?;
                }
            }
            PackageSection::Manifest => {
                let (bytes, done) = self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.snapshot)?;
                if !bytes.is_empty() {
                    self.append_zip_data(&bytes)?;
                }
                if done {
                    self.package_json = None;
                    self.end_zip_entry()?;
                    self.package_index = 0;
                    self.zip_central_offset = u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?;
                    self.package_section = PackageSection::CentralDirectory;
                }
            }
            PackageSection::CentralDirectory => {
                if let Some(entry) = self.zip.entries.get(self.package_index).cloned() {
                    let mut central = Vec::with_capacity(46 + entry.name.len());
                    push_u32(&mut central, 0x02014b50);
                    push_u16(&mut central, 20);
                    push_u16(&mut central, 20);
                    push_u16(&mut central, 0x0008);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u32(&mut central, entry.crc);
                    push_u32(&mut central, entry.size);
                    push_u32(&mut central, entry.size);
                    push_u16(&mut central, u16::try_from(entry.name.len()).map_err(|_| "layout-export-package-name-limit")?);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u16(&mut central, 0);
                    push_u32(&mut central, 0);
                    push_u32(&mut central, entry.offset);
                    central.extend_from_slice(entry.name.as_bytes());
                    self.output.append(&central)?;
                    self.package_index += 1;
                } else {
                    let end = u32::try_from(self.output.len).map_err(|_| "layout-export-output-limit")?;
                    let count = u16::try_from(self.zip.entries.len()).map_err(|_| "layout-export-package-entry-limit")?;
                    let mut eocd = Vec::with_capacity(22);
                    push_u32(&mut eocd, 0x06054b50);
                    push_u16(&mut eocd, 0);
                    push_u16(&mut eocd, 0);
                    push_u16(&mut eocd, count);
                    push_u16(&mut eocd, count);
                    push_u32(&mut eocd, end - self.zip_central_offset);
                    push_u32(&mut eocd, self.zip_central_offset);
                    push_u16(&mut eocd, 0);
                    self.output.append(&eocd)?;
                    self.package_section = PackageSection::Complete;
                }
            }
            PackageSection::Complete => self.stage = ExportStage::Base64,
        }
        Ok(())
    }

    fn finish_preflight_entry(&mut self) -> Result<(), String> {
        self.end_zip_entry()?;
        self.begin_zip_entry("package-manifest.json")?;
        self.package_json = Some(TypedJsonCursor::manifest());
        self.package_index = 0;
        self.package_section = PackageSection::Manifest;
        Ok(())
    }

    fn base64_one(&mut self) -> Result<(), String> {
        const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let remaining = self.output.len.saturating_sub(self.base64_cursor);
        if remaining == 0 {
            if !self.base64_tail.is_empty() {
                let a = self.base64_tail[0];
                let b = self.base64_tail.get(1).copied().unwrap_or(0);
                self.encoded.append(&[TABLE[(a >> 2) as usize], TABLE[((a & 3) << 4 | b >> 4) as usize], if self.base64_tail.len() > 1 { TABLE[((b & 15) << 2) as usize] } else { b'=' }, b'='])?;
                self.base64_tail.clear();
            }
            self.stage = ExportStage::PackageCommit;
            return Ok(());
        }
        let take = remaining.min(BASE64_INPUT_BYTES_PER_UNIT);
        let mut input = Vec::with_capacity(self.base64_tail.len() + take);
        input.extend_from_slice(&self.base64_tail);
        let source = self.output.take_prefix(take);
        if source.len() != take {
            return Err("layout-export-base64-cursor".into());
        }
        input.extend_from_slice(&source);
        self.base64_cursor += take;
        let complete = input.len() / 3 * 3;
        let mut encoded = Vec::with_capacity(complete / 3 * 4);
        for triple in input[..complete].chunks_exact(3) {
            encoded.extend_from_slice(&[TABLE[(triple[0] >> 2) as usize], TABLE[((triple[0] & 3) << 4 | triple[1] >> 4) as usize], TABLE[((triple[1] & 15) << 2 | triple[2] >> 6) as usize], TABLE[(triple[2] & 63) as usize]]);
        }
        self.encoded.append(&encoded)?;
        self.base64_tail.clear();
        self.base64_tail.extend_from_slice(&input[complete..]);
        Ok(())
    }

    fn stage_name(&self) -> &'static str {
        match self.stage {
            ExportStage::Validate => "layout.export.validate",
            ExportStage::Plan => "layout.export.plan",
            ExportStage::Encode => "layout.export.encode",
            ExportStage::Base64 => "layout.export.base64",
            ExportStage::PackageCommit => "layout.export.package-commit",
            ExportStage::Complete => "layout.export.complete",
        }
    }

    fn advance_one(&mut self) -> Result<(), String> {
        match self.stage {
            ExportStage::Validate => self.validate_one(),
            ExportStage::Plan => self.plan_one(),
            ExportStage::Encode => self.encode_one(),
            ExportStage::Base64 => self.base64_one(),
            ExportStage::PackageCommit => self.package_commit_one(),
            ExportStage::Complete => Ok(()),
        }
    }

    fn verify_restore_target(&mut self) -> Result<bool, String> {
        let Some(target) = &self.restore_target else { return Ok(false) };
        if self.completed_units < target.completed_units {
            return Ok(false);
        }
        if self.completed_units != target.completed_units || self.active_output().len as u64 != target.output_bytes || self.active_output().digest != target.output_digest {
            return Err("layout-export-checkpoint-replay-mismatch".into());
        }
        self.restore_target = None;
        Ok(true)
    }

    fn package_commit_one(&mut self) -> Result<(), String> {
        let chunk = if self.request.kind.binary() { self.encoded.take_chunk() } else { self.output.take_chunk() };
        if let Some(bytes) = chunk {
            if let Some(credit) = &self.media_output_credit {
                credit.credit(bytes.len()).map_err(|error| error.to_string())?;
            }
            self.output_chunks.push(bytes).map_err(|error| error.to_string())?;
            self.commit_cursor += 1;
        } else {
            self.output_chunks.seal().map_err(|error| error.to_string())?;
            self.stage = ExportStage::Complete;
        }
        Ok(())
    }

    fn commit_publication(&mut self) -> Result<LayoutExportPublication, String> {
        let checkpoint = LayoutExportCheckpoint { completed_units: self.completed_units, output_bytes: self.active_output().len as u64, output_digest: self.active_output().digest };
        let state = encode_checkpoint(&self.operation, &self.request, &checkpoint)?;
        LayoutExportPublication::new(LayoutExportPublicationKind::Commit, &state)
    }
}

impl InteractiveJob for LayoutExportJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if let Some(publication) = self.publication.as_mut() {
            let outcome = publication.step(context);
            if !matches!(outcome, StepOutcome::Yield) {
                self.publication = None;
            }
            return outcome;
        }
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            self.publication = Some(Self::fault_publication("layout-export-stale-operation"));
            return self.drive_publication(context);
        }
        context.set_stage(self.stage_name());
        loop {
            if let Err(error) = self.advance_one() {
                self.publication = Some(Self::fault_publication(&error));
                return self.drive_publication(context);
            }
            self.completed_units = self.completed_units.saturating_add(1);
            context.consume_fuel(1);
            if context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            match self.verify_restore_target() {
                Ok(true) => {
                    self.publication = self.preview_publication().ok();
                    return self.drive_publication(context);
                }
                Ok(false) => {}
                Err(error) => {
                    self.publication = Some(Self::fault_publication(&error));
                    return self.drive_publication(context);
                }
            }
            if matches!(self.stage, ExportStage::Complete) {
                self.publication = Some(match self.commit_publication() {
                    Ok(publication) => publication,
                    Err(error) => Self::fault_publication(&error),
                });
                return self.drive_publication(context);
            }
            if self.completed_units % 64 == 0 {
                self.publication = Some(match self.checkpoint_publication() {
                    Ok(publication) => publication,
                    Err(error) => Self::fault_publication(&error),
                });
                return self.drive_publication(context);
            }
            if self.completed_units % 16 == 0 {
                self.publication = self.preview_publication().ok();
                return self.drive_publication(context);
            }
            if context.should_yield() {
                return StepOutcome::Yield;
            }
        }
    }

    fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        self.close_stage = LayoutExportCloseStage::Publication;
        if let Some(publication) = self.publication.as_mut() {
            publication.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if self.close_stage == LayoutExportCloseStage::Publication {
            if let Some(publication) = self.publication.as_mut() {
                match publication.close_step(maximum_items, maximum_bytes) {
                    JobPayloadCloseStep::Pending { released_items, released_bytes } => return InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    JobPayloadCloseStep::Complete if !publication.terminal_is_empty() => return InteractiveJobCloseStep::Blocked,
                    JobPayloadCloseStep::Complete => self.publication = None,
                }
            }
            self.close_stage = LayoutExportCloseStage::JsonValidation;
            return InteractiveJobCloseStep::Pending { released_items: usize::from(maximum_items > 0), released_bytes: 0 };
        }
        match self.close_export_step(maximum_items, maximum_bytes) {
            Ok(PluginCloseStep::Pending { released_items, released_bytes }) => InteractiveJobCloseStep::Pending { released_items, released_bytes },
            Ok(PluginCloseStep::Complete) => InteractiveJobCloseStep::Complete,
            Err(_) => InteractiveJobCloseStep::Blocked,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.publication.as_ref().is_none_or(LayoutExportPublication::terminal_is_empty) && self.close_terminal_is_empty()
    }
}

impl ArtifactReservedJob for LayoutExportJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        self.close_export_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.close_terminal_is_empty()
    }
}
//#endregion 🧩️Job

//#region 🔧️PlanPrimitives
fn frame_bounds(frame: &Frame) -> &LayoutBounds {
    match frame {
        Frame::Rect { bounds, .. } | Frame::Text { bounds, .. } | Frame::Image { bounds, .. } => bounds,
    }
}

fn frame_visible(frame: &Frame) -> bool {
    match frame {
        Frame::Rect { visible, .. } | Frame::Text { visible, .. } | Frame::Image { visible, .. } => visible.unwrap_or(true),
    }
}

fn frame_id(frame: &Frame) -> &str {
    match frame {
        Frame::Rect { id, .. } | Frame::Text { id, .. } | Frame::Image { id, .. } => id,
    }
}

fn override_for<'a>(page: &'a Page, frame: &Frame) -> Option<&'a PageOverride> {
    page.overrides.iter().find(|candidate| candidate.object_id == frame_id(frame))
}

fn export_rect(frame: &Frame, override_value: Option<&PageOverride>, page_width: u32, page_height: u32) -> Option<ExportRect> {
    if !override_value.and_then(|value| value.visible).unwrap_or_else(|| frame_visible(frame)) {
        return None;
    }
    let bounds = override_value.and_then(|value| value.bounds.as_ref()).unwrap_or_else(|| frame_bounds(frame));
    let x = bounds.x.max(0.0).min(f64::from(page_width)) as u32;
    let y = bounds.y.max(0.0).min(f64::from(page_height)) as u32;
    let x1 = (bounds.x + bounds.width).max(0.0).min(f64::from(page_width)) as u32;
    let y1 = (bounds.y + bounds.height).max(0.0).min(f64::from(page_height)) as u32;
    if x1 <= x || y1 <= y {
        return None;
    }
    let rgba = match frame {
        Frame::Rect { fill, .. } => fill.map(|color| color.map(|channel| (channel.clamp(0.0, 1.0) * 255.0).round() as u8)).unwrap_or([0, 0, 0, 0]),
        Frame::Text { .. } => [24, 24, 24, 255],
        Frame::Image { .. } => [220, 220, 220, 255],
    };
    Some(ExportRect { x, y, width: x1 - x, height: y1 - y, rgba })
}

fn finite_dimension(value: f64) -> Result<u32, String> {
    if !value.is_finite() || value < 1.0 || value > f64::from(MAX_LAYOUT_EXPORT_DIMENSION) {
        return Err("layout-export-dimension-limit".into());
    }
    Ok(value.ceil() as u32)
}

fn bounded_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 256
}

fn bounded_string(value: &str) -> bool {
    value.len() <= MAX_LAYOUT_EXPORT_STRING_BYTES
}

fn valid_frame(frame: &Frame) -> bool {
    match frame {
        Frame::Rect { id, layer_id, bounds, fill, stroke, .. } => bounded_id(id) && bounded_id(layer_id) && valid_bounds(bounds) && fill.is_none_or(valid_color) && stroke.is_none_or(valid_color),
        Frame::Text { id, layer_id, bounds, story_id, thread_next, columns, inset, wrap_mode, .. } => {
            bounded_id(id)
                && bounded_id(layer_id)
                && valid_bounds(bounds)
                && bounded_id(story_id)
                && thread_next.as_ref().is_none_or(|id| bounded_id(id))
                && (*columns as usize) <= MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                && valid_rect(inset)
                && bounded_string(wrap_mode)
        }
        Frame::Image { id, layer_id, bounds, link_id, .. } => bounded_id(id) && bounded_id(layer_id) && valid_bounds(bounds) && bounded_id(link_id),
    }
}

fn valid_bounds(bounds: &LayoutBounds) -> bool {
    bounds.x.is_finite() && bounds.y.is_finite() && bounds.width.is_finite() && bounds.height.is_finite() && bounds.rotation.is_finite()
}

fn valid_rect(rect: &crate::artifacts::layout::LayoutRect) -> bool {
    rect.x.is_finite() && rect.y.is_finite() && rect.width.is_finite() && rect.height.is_finite()
}

fn valid_color(color: [f32; 4]) -> bool {
    color.iter().all(|channel| channel.is_finite())
}

fn add_validated_items(total: &mut usize, count: usize) -> Result<(), String> {
    *total = total.checked_add(count).ok_or("layout-export-item-limit")?;
    if *total > MAX_LAYOUT_EXPORT_DECODED_ITEMS {
        return Err("layout-export-item-limit".into());
    }
    Ok(())
}

//#endregion 🔧️PlanPrimitives

//#region 🔧️CodecPrimitives
struct CheckpointReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> CheckpointReader<'a> {
    fn take(&mut self, count: usize) -> Result<&'a [u8], String> {
        let end = self.cursor.checked_add(count).ok_or("layout-export-checkpoint-decode")?;
        let value = self.bytes.get(self.cursor..end).ok_or("layout-export-checkpoint-decode")?;
        self.cursor = end;
        Ok(value)
    }

    fn byte(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    fn fixed_authority(&mut self) -> Result<&'a [u8], String> {
        let length = u16::from_le_bytes(self.take(2)?.try_into().map_err(|_| "layout-export-checkpoint-decode")?) as usize;
        if length > MAX_LAYOUT_EXPORT_AUTHORITY_BYTES {
            return Err("layout-export-checkpoint-decode".into());
        }
        let storage = self.take(MAX_LAYOUT_EXPORT_AUTHORITY_BYTES)?;
        if storage[length..].iter().any(|byte| *byte != 0) {
            return Err("layout-export-checkpoint-decode".into());
        }
        Ok(&storage[..length])
    }

    fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().map_err(|_| "layout-export-checkpoint-decode")?))
    }
}

fn checkpoint_kind(kind: LayoutExportKind) -> u8 {
    match kind {
        LayoutExportKind::Png => 0,
        LayoutExportKind::Svg => 1,
        LayoutExportKind::Pdf => 2,
        LayoutExportKind::Package => 3,
    }
}

struct CheckpointWriter {
    bytes: [u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES],
    cursor: usize,
}

impl CheckpointWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
        let end = self.cursor.checked_add(bytes.len()).ok_or("layout-export-checkpoint-encode")?;
        self.bytes.get_mut(self.cursor..end).ok_or("layout-export-checkpoint-encode")?.copy_from_slice(bytes);
        self.cursor = end;
        Ok(())
    }

    fn authority(&mut self, value: &str) -> Result<(), String> {
        if value.len() > MAX_LAYOUT_EXPORT_AUTHORITY_BYTES {
            return Err("layout-export-authority-invalid".into());
        }
        self.write(&(value.len() as u16).to_le_bytes())?;
        self.write(value.as_bytes())?;
        self.write(&[0; MAX_LAYOUT_EXPORT_AUTHORITY_BYTES][..MAX_LAYOUT_EXPORT_AUTHORITY_BYTES - value.len()])
    }
}

fn encode_checkpoint(operation: &Operation, request: &LayoutExportRequest, checkpoint: &LayoutExportCheckpoint) -> Result<[u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], String> {
    let mut state = CheckpointWriter { bytes: [0; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], cursor: 0 };
    state.write(LAYOUT_EXPORT_CHECKPOINT_MAGIC)?;
    state.write(&[checkpoint_kind(request.kind), u8::from(request.page_id.is_some())])?;
    state.authority(request.page_id.as_deref().unwrap_or_default())?;
    state.authority(&request.parent_document_id)?;
    state.write(request.canonical_base_revision_hex.as_bytes())?;
    for value in [operation.operation.0, operation.base_revision.0, operation.generation.0, checkpoint.completed_units, checkpoint.output_bytes, checkpoint.output_digest] {
        state.write(&value.to_le_bytes())?;
    }
    if state.cursor != MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES {
        return Err("layout-export-checkpoint-encode".into());
    }
    Ok(state.bytes)
}

fn decode_checkpoint(operation: &Operation, request: &LayoutExportRequest, state: &[u8]) -> Result<LayoutExportCheckpoint, String> {
    if state.len() != MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES {
        return Err("layout-export-checkpoint-limit".into());
    }
    let mut reader = CheckpointReader { bytes: state, cursor: 0 };
    if reader.take(LAYOUT_EXPORT_CHECKPOINT_MAGIC.len())? != LAYOUT_EXPORT_CHECKPOINT_MAGIC {
        return Err("layout-export-checkpoint-decode".into());
    }
    let kind = reader.byte()?;
    let page_present = reader.byte()?;
    let page_id = reader.fixed_authority()?;
    let parent_document_id = reader.fixed_authority()?;
    let canonical_base_revision_hex = reader.take(64)?;
    let operation_id = reader.u64()?;
    let base_revision = reader.u64()?;
    let generation = reader.u64()?;
    let checkpoint = LayoutExportCheckpoint { completed_units: reader.u64()?, output_bytes: reader.u64()?, output_digest: reader.u64()? };
    let expected_page = request.page_id.as_deref().map(str::as_bytes);
    let decoded_page = match page_present {
        0 if page_id.is_empty() => None,
        1 => Some(page_id),
        _ => return Err("layout-export-checkpoint-decode".into()),
    };
    if reader.cursor != state.len()
        || kind != checkpoint_kind(request.kind)
        || decoded_page != expected_page
        || parent_document_id != request.parent_document_id.as_bytes()
        || canonical_base_revision_hex != request.canonical_base_revision_hex.as_bytes()
        || operation_id != operation.operation.0
        || base_revision != operation.base_revision.0
        || generation != operation.generation.0
    {
        return Err("layout-export-checkpoint-authority-mismatch".into());
    }
    Ok(checkpoint)
}

fn crc32_update(mut crc: u32, bytes: &[u8]) -> u32 {
    crc = !crc;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb88320 & 0u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn sanitize_filename(value: &str) -> String {
    let value: String = value.chars().take(128).map(|character| if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') { character } else { '-' }).collect();
    if value.is_empty() {
        "layout".into()
    } else {
        value
    }
}

pub(crate) fn output_name(request: &LayoutExportRequest) -> String {
    if matches!(request.kind, LayoutExportKind::Package) {
        sanitize_filename(&request.snapshot.name)
    } else {
        request.page_id.as_deref().map(sanitize_filename).unwrap_or_else(|| "layout".into())
    }
}

fn decode_base64(value: &str) -> Result<Vec<u8>, String> {
    let mut result = Vec::with_capacity(value.len() / 4 * 3);
    for quartet in value.as_bytes().chunks_exact(4) {
        let a = base64_value(quartet[0])?;
        let b = base64_value(quartet[1])?;
        let c = base64_value(quartet[2])?;
        let d = base64_value(quartet[3])?;
        result.push(a << 2 | b >> 4);
        if c != 64 {
            result.push(b << 4 | c >> 2);
        }
        if d != 64 {
            result.push(c << 6 | d);
        }
    }
    if value.len() % 4 != 0 {
        return Err("layout-export-invalid-base64".into());
    }
    Ok(result)
}

fn base64_value(byte: u8) -> Result<u8, String> {
    match byte {
        b'A'..=b'Z' => Ok(byte - b'A'),
        b'a'..=b'z' => Ok(byte - b'a' + 26),
        b'0'..=b'9' => Ok(byte - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        b'=' => Ok(64),
        _ => Err("layout-export-invalid-base64".into()),
    }
}
//#endregion 🔧️CodecPrimitives

//#region 🧪️TestOracle
#[cfg(test)]
pub fn run_layout_export_headless_batch(operation: Operation, request: LayoutExportRequest) -> Result<LayoutExportCommit, String> {
    let kind = request.kind;
    let name = output_name(&request);
    let snapshot_owner = Arc::clone(&request.snapshot);
    let job = LayoutExportJob::new(operation, request)?;
    let output_chunks = job.output_chunks.clone();
    let params = BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: semio_framework_job::root_cancel_token(),
        config: BatchDriveConfig { site: "layout.export.batch", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = match semio_framework_job::BatchJobSession::try_new(job, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            return Err("layout-export-test-oracle-admission-rejected".into());
        }
    };
    loop {
        session.step().map_err(|error| format!("layout-export-test-oracle-contention:{error:?}"))?;
        let Some(mut outcome) = session.take_outcome() else { continue };
        let terminal = outcome.is_terminal();
        let result = match &outcome {
            StepOutcome::Complete(_) => Some(LayoutExportCommit::from_chunks(kind, &name, &output_chunks)),
            StepOutcome::Cancelled => Some(Err("layout-export-cancelled".into())),
            StepOutcome::Fault(fault) => Some(Err(fault.detail.single_page().and_then(|page| std::str::from_utf8(page).ok()).unwrap_or("layout-export-fault").to_owned())),
            StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => None,
        };
        while !outcome.terminal_is_empty() {
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        if terminal {
            session.begin_close();
            while !session.terminal_is_empty() {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            drop(snapshot_owner);
            return result.expect("terminal layout test oracle result");
        }
        session.resume().map_err(|error| format!("layout-export-test-oracle-resume:{error:?}"))?;
    }
}

#[cfg(test)]
fn headless_batch_export(kind: LayoutExportKind, snapshot: &LayoutSnapshot, page_id: Option<&str>, preflight_json: Option<&str>) -> Result<LayoutExportCommit, String> {
    let operation = Operation::new(semio_framework_job::allocate_operation_id(), RevisionId(0), Generation(0), 0);
    run_layout_export_headless_batch(
        operation,
        LayoutExportRequest {
            kind,
            page_id: page_id.map(str::to_owned),
            snapshot: Arc::new(snapshot.clone()),
            preflight_json: preflight_json.map(str::to_owned),
            parent_document_id: "layout.headless-batch".into(),
            canonical_base_revision_hex: "0000000000000000000000000000000000000000000000000000000000000000".into(),
        },
    )
}

#[cfg(test)]
pub fn export_document_svg_headless_batch(doc: &LayoutSnapshot, page_id: &str) -> Result<String, crate::artifacts::layout::io::LayoutError> {
    if !doc.pages.iter().any(|page| page.id == page_id) {
        return Err(crate::artifacts::layout::io::LayoutError::PageNotFound(page_id.into()));
    }
    headless_batch_export(LayoutExportKind::Svg, doc, Some(page_id), None).map(|commit| commit.data).map_err(crate::artifacts::layout::io::LayoutError::Svg)
}

#[cfg(test)]
pub fn export_document_pdf_headless_batch(doc: &LayoutSnapshot, page_id: &str) -> Result<Vec<u8>, crate::artifacts::layout::io::LayoutError> {
    if !doc.pages.iter().any(|page| page.id == page_id) {
        return Err(crate::artifacts::layout::io::LayoutError::PageNotFound(page_id.into()));
    }
    let commit = headless_batch_export(LayoutExportKind::Pdf, doc, Some(page_id), None).map_err(crate::artifacts::layout::io::LayoutError::Svg)?;
    decode_base64(&commit.data).map_err(crate::artifacts::layout::io::LayoutError::Svg)
}

#[cfg(test)]
pub fn export_document_png_headless_batch(doc: &LayoutSnapshot, page_id: &str) -> Result<Vec<u8>, crate::artifacts::layout::io::LayoutError> {
    if !doc.pages.iter().any(|page| page.id == page_id) {
        return Err(crate::artifacts::layout::io::LayoutError::PageNotFound(page_id.into()));
    }
    let commit = headless_batch_export(LayoutExportKind::Png, doc, Some(page_id), None).map_err(crate::artifacts::layout::io::LayoutError::Svg)?;
    decode_base64(&commit.data).map_err(crate::artifacts::layout::io::LayoutError::Svg)
}

#[cfg(test)]
pub fn export_package_zip_headless_batch(doc_json: &str, preflight_json: &str) -> Result<Vec<u8>, crate::artifacts::layout::io::LayoutError> {
    let snapshot: LayoutSnapshot = serde_json::from_str(doc_json)?;
    let commit = headless_batch_export(LayoutExportKind::Package, &snapshot, None, Some(preflight_json)).map_err(crate::artifacts::layout::io::LayoutError::Svg)?;
    decode_base64(&commit.data).map_err(crate::artifacts::layout::io::LayoutError::Svg)
}
//#endregion 🧪️TestOracle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn drive_test_job<J: InteractiveJob + 'static>(job: J, params: BatchJobParams) -> StepOutcome {
        let mut session = match semio_framework_job::BatchJobSession::try_new(job, params) {
            Ok(session) => session,
            Err(mut rejected) => {
                rejected.begin_close();
                while !rejected.terminal_is_empty() {
                    let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                }
                panic!("test oracle admits retained session");
            }
        };
        loop {
            session.step().expect("test oracle caller opportunity");
            let Some(mut outcome) = session.take_outcome() else { continue };
            let terminal = outcome.is_terminal();
            while !outcome.terminal_is_empty() {
                let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            if terminal {
                session.begin_close();
                while !session.terminal_is_empty() {
                    let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                }
                return outcome;
            }
            session.resume().expect("test oracle resumes exact owner");
        }
    }

    fn operation() -> Operation {
        Operation::new(semio_framework_job::OperationId(71), RevisionId(9), Generation(3), 17)
    }

    #[test]
    fn layout_retained_publication_zero_grant_and_exact_writer_close_are_exact() {
        let mut publication = LayoutExportPublication::new(LayoutExportPublicationKind::Preview, b"layout-preview").expect("bounded retained preview");
        publication.begin_close();
        assert_eq!(publication.close_step(0, 0), JobPayloadCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert!(!publication.terminal_is_empty());
        let _ = publication.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        assert!(publication.terminal_is_empty());
    }

    fn request(kind: LayoutExportKind) -> LayoutExportRequest {
        let snapshot = crate::artifacts::layout::schema::default_document();
        let page_id = (!matches!(kind, LayoutExportKind::Package)).then(|| snapshot.pages[0].id.clone());
        LayoutExportRequest { kind, page_id, snapshot: Arc::new(snapshot), preflight_json: None, parent_document_id: "layout-test-document".into(), canonical_base_revision_hex: "09".repeat(32) }
    }

    fn drive_dispatched_worker(kind: LayoutExportKind, worker_count: usize, generation: Generation, cancel_before_start: bool) -> (StepOutcome, ArtifactOutputChunks) {
        let operation = operation();
        let bus = semio_framework::ActionBus::new();
        bus.register(LayoutExportJobFactory::new("layout-test")).expect("factory registration");
        let output_chunks = ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES);
        let request = request(kind);
        let snapshot_owner = Arc::clone(&request.snapshot);
        let spec = semio_framework::ToolOperationSpec::new("layout-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, LayoutExportToolPayload { request, output_chunks: output_chunks.clone(), completion: None }, operation);
        let dispatch = bus.dispatch(spec).expect("exact factory dispatch");
        assert_eq!(bus.dispatch_count(), 1);
        let cancel = semio_framework_job::root_cancel_token();
        if cancel_before_start {
            cancel.cancel_now();
        }
        let params = BatchJobParams {
            operation: operation.operation,
            generation,
            cancel,
            config: BatchDriveConfig { site: "layout.export.worker-test", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
            now_us: semio_framework_job::default_now_us,
        };
        let _ = worker_count;
        let outcome = drive_test_job(dispatch.job, params);
        drop(snapshot_owner);
        (outcome, output_chunks)
    }

    #[test]
    fn exact_factory_dispatch_is_deterministic_across_real_one_two_four_and_default_worker_pools() {
        let default_workers = std::thread::available_parallelism().map_or(1, usize::from);
        for kind in [LayoutExportKind::Png, LayoutExportKind::Svg, LayoutExportKind::Pdf, LayoutExportKind::Package] {
            let mut outputs = Vec::new();
            for worker_count in [1, 2, 4, default_workers] {
                match drive_dispatched_worker(kind, worker_count, operation().generation, false) {
                    (StepOutcome::Complete(_), chunks) => outputs.push(LayoutExportCommit::from_chunks(kind, "test", &chunks).expect("segmented output").data),
                    outcome => panic!("unexpected {kind:?}/{worker_count} outcome: {outcome:?}"),
                }
            }
            assert!(outputs.windows(2).all(|pair| pair[0] == pair[1]), "{kind:?} bytes diverged across real worker pools");
        }
    }

    #[test]
    fn production_retained_wire_factory_decodes_before_reducer_and_closes_cancel_fault_and_success_owners() {
        fn dispatch(raw_verb: &str, kind: LayoutExportKind) -> semio_framework::ToolJobDispatch {
            let operation = operation();
            let bus = semio_framework::ActionBus::new();
            bus.register(LayoutExportJobFactory::new("layout-retained-test")).expect("retained factory registration");
            assert!(bus.begin_exact_wire("layout-retained-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES + 1).is_err());
            let (admission, mut input) = bus.begin_exact_wire("layout-retained-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES).expect("maximum extent before encoding");
            let raw = serde_json::to_vec(&(raw_verb, serde_json::json!({ "pageId": null }))).expect("fixture wire");
            for bytes in raw.chunks(semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES) {
                input.admit_page(semio_framework::action_bus::ToolWirePage::try_copy_from(bytes).expect("bounded raw page")).expect("preadmitted page");
            }
            input.seal_admitted_prefix().expect("truthful encoded prefix");
            let output_chunks = ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES);
            let payload = LayoutExportToolPayload { request: request(kind), output_chunks, completion: None };
            let spec = semio_framework::ToolOperationSpec::new("layout-retained-test", kind.tool_id(), LAYOUT_EXPORT_PAYLOAD_SCHEMA, payload, operation);
            bus.dispatch_wire_retained_with_spec(admission, input, None, spec).unwrap_or_else(|_| panic!("retained production dispatch"))
        }

        let params = |cancel: semio_framework_job::CancelToken| BatchJobParams {
            operation: operation().operation,
            generation: operation().generation,
            cancel,
            config: BatchDriveConfig { site: "layout.retained-wire.worker-test", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
            now_us: semio_framework_job::default_now_us,
        };
        assert!(matches!(drive_test_job(dispatch("exportSvg", LayoutExportKind::Svg).job, params(semio_framework_job::root_cancel_token())), StepOutcome::Complete(_)));
        assert!(matches!(drive_test_job(dispatch("exportPdf", LayoutExportKind::Svg).job, params(semio_framework_job::root_cancel_token())), StepOutcome::Fault(_)));
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        assert!(matches!(drive_test_job(dispatch("exportSvg", LayoutExportKind::Svg).job, params(cancel)), StepOutcome::Cancelled));
    }

    #[test]
    fn exact_layout_out_media_factory_dispatches_a_real_reserved_job() {
        let operation = operation();
        let bus = semio_framework::ActionBus::new();
        bus.register(LayoutMediaExportJobFactory::new("layout-media-test")).expect("media factory registration");
        let request = request(LayoutExportKind::Svg);
        let snapshot_owner = Arc::clone(&request.snapshot);
        let payload = ArtifactReservedToolJob::new(LayoutExportJob::new(operation, request).expect("media payload job"));
        let spec = semio_framework::ToolOperationSpec::new("layout-media-test", LAYOUT_MEDIA_EXPORT_TOOL_ID, LAYOUT_MEDIA_EXPORT_PAYLOAD_SCHEMA, payload, operation);
        let dispatch = bus.dispatch(spec).expect("exact media factory dispatch");
        assert_eq!(bus.dispatch_count(), 1);
        let params = BatchJobParams {
            operation: operation.operation,
            generation: operation.generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: BatchDriveConfig { site: "layout.media-export.worker-test", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
            now_us: semio_framework_job::default_now_us,
        };
        assert!(matches!(drive_test_job(dispatch.job, params), StepOutcome::Complete(_)));
        drop(snapshot_owner);
    }

    #[test]
    fn reserved_close_disposes_every_export_buffer_in_bounded_slices_and_rejects_an_unwitnessed_snapshot() {
        let mut job = LayoutExportJob::new(operation(), request(LayoutExportKind::Svg)).expect("job");
        job.json_validation = Some(JsonValidationCursor::new("[]", true).expect("json cursor"));
        job.json_validation.as_mut().expect("json cursor").stack.push(JsonContainer::Array(JsonArrayState::FirstValueOrEnd));
        job.typed_validation = Some(TypedJsonCursor { stack: vec![TypedJsonNode::Scalar { bytes: vec![1; 256], cursor: 0 }], emitted_nodes: 0, fragment_bytes: None });
        job.package_json = Some(TypedJsonCursor { stack: vec![TypedJsonNode::OwnedString { value: "z".repeat(OUTPUT_CHUNK_BYTES * 2), cursor: JsonStringWriteCursor::default() }], emitted_nodes: 0, fragment_bytes: None });
        job.rects.push(ExportRect { x: 0, y: 0, width: 1, height: 1, rgba: [0; 4] });
        job.output.append(&vec![1; OUTPUT_CHUNK_BYTES * 2]).expect("raw chunks");
        job.encoded.append(&vec![2; OUTPUT_CHUNK_BYTES * 2]).expect("encoded chunks");
        job.base64_tail = vec![3; 2];
        job.png_row = vec![4; OUTPUT_CHUNK_BYTES * 2];
        job.pdf_offsets.push(7);
        job.zip.entries.push(ZipEntry { name: "e".repeat(OUTPUT_CHUNK_BYTES * 2), crc: 0, size: 0, offset: 0 });
        job.zip.current_name = Some("current".into());
        job.request.preflight_json = Some("p".repeat(OUTPUT_CHUNK_BYTES * 2));
        job.output_chunks.push(vec![5; OUTPUT_CHUNK_BYTES]).expect("shared chunk");

        let error = loop {
            match ArtifactReservedJob::close_step(&mut job, 1, OUTPUT_CHUNK_BYTES) {
                Ok(PluginCloseStep::Pending { released_items, released_bytes }) => {
                    assert_eq!(released_items, 1);
                    assert!(released_bytes <= OUTPUT_CHUNK_BYTES);
                }
                Err(error) => break error,
                Ok(step) => panic!("unwitnessed close must not reach {step:?}"),
            }
        };
        assert!(error.to_string().contains("snapshot-unwitnessed"));
        assert!(job.json_validation.is_none());
        assert!(job.typed_validation.is_none());
        assert!(job.package_json.is_none());
        assert!(job.rects.is_empty());
        assert!(job.output.chunks.is_empty());
        assert!(job.encoded.chunks.is_empty());
        assert!(job.base64_tail.is_empty());
        assert!(job.png_row.is_empty());
        assert!(job.pdf_offsets.is_empty());
        assert!(job.zip.entries.is_empty());
        assert!(job.zip.current_name.is_none());
        assert!(job.request.page_id.is_none());
        assert!(job.request.preflight_json.is_none());
        assert!(job.request.parent_document_id.is_empty());
        assert!(job.request.canonical_base_revision_hex.is_empty());
        assert_eq!(job.output_chunks.chunks_remaining(), 0);
    }

    #[test]
    fn reserved_close_zero_budget_preserves_the_exact_cursor_and_owner() {
        let mut job = LayoutExportJob::new(operation(), request(LayoutExportKind::Svg)).expect("job");
        job.output.append(&vec![1; OUTPUT_CHUNK_BYTES]).expect("raw chunk");
        job.close_stage = LayoutExportCloseStage::Output;
        assert_eq!(ArtifactReservedJob::close_step(&mut job, 0, OUTPUT_CHUNK_BYTES).expect("zero item slice"), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(job.close_stage, LayoutExportCloseStage::Output);
        assert_eq!(job.output.chunks.len(), 1);
        assert_eq!(ArtifactReservedJob::close_step(&mut job, 1, 0).expect("zero byte slice"), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(job.close_stage, LayoutExportCloseStage::Output);
        assert_eq!(job.output.chunks.len(), 1);
    }

    #[test]
    fn reserved_close_releases_the_witness_after_snapshot_handback() {
        let mut job = LayoutExportJob::new(operation(), request(LayoutExportKind::Svg)).expect("job");
        job.close_stage = LayoutExportCloseStage::SnapshotOwner;
        assert_eq!(ArtifactReservedJob::close_step(&mut job, 1, OUTPUT_CHUNK_BYTES).expect("witness handback"), PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        assert_eq!(job.close_stage, LayoutExportCloseStage::Complete);
    }

    #[test]
    fn dimension_max_plus_one_is_rejected_before_encoding() {
        let mut request = request(LayoutExportKind::Png);
        Arc::make_mut(&mut request.snapshot).pages[0].width = f64::from(MAX_LAYOUT_EXPORT_DIMENSION + 1);
        let error = run_layout_export_headless_batch(operation(), request).expect_err("max + 1 must fail");
        assert!(error.contains("dimension-limit"));
    }

    #[test]
    fn real_worker_dispatch_observes_cancel_and_stale_generation() {
        assert!(matches!(drive_dispatched_worker(LayoutExportKind::Png, 1, operation().generation, true).0, StepOutcome::Cancelled));
        assert!(matches!(drive_dispatched_worker(LayoutExportKind::Svg, 1, Generation(operation().generation.0 + 1), false).0, StepOutcome::Fault(_)));
    }

    #[test]
    fn collection_and_json_envelopes_accept_max_and_reject_max_plus_one() {
        let mut max = request(LayoutExportKind::Svg);
        let snapshot = Arc::make_mut(&mut max.snapshot);
        let page = snapshot.pages[0].clone();
        snapshot.pages.resize(MAX_LAYOUT_EXPORT_PAGES, page);
        let style = snapshot.paragraph_styles[0].clone();
        snapshot.paragraph_styles.resize(MAX_LAYOUT_EXPORT_STYLES, style);
        let character =
            snapshot.character_styles.first().cloned().unwrap_or(crate::artifacts::layout::CharacterStyle { id: "character.test".into(), name: None, font_family: None, font_size: None, font_weight: None, italic: None, color: None, tracking: None });
        snapshot.character_styles.resize(MAX_LAYOUT_EXPORT_STYLES, character);
        snapshot.data_fields_json = Some(format!("{}[]", " ".repeat(MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES - 2)));
        assert!(run_layout_export_headless_batch(operation(), max).is_ok());

        let mut plus_one = request(LayoutExportKind::Svg);
        let snapshot = Arc::make_mut(&mut plus_one.snapshot);
        let page = snapshot.pages[0].clone();
        snapshot.pages.resize(MAX_LAYOUT_EXPORT_PAGES + 1, page);
        assert!(run_layout_export_headless_batch(operation(), plus_one).expect_err("page max + 1").contains("document-envelope"));

        let mut json_plus_one = request(LayoutExportKind::Svg);
        Arc::make_mut(&mut json_plus_one.snapshot).data_fields_json = Some(format!("{}[]", " ".repeat(MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES - 1)));
        assert!(run_layout_export_headless_batch(operation(), json_plus_one).expect_err("json max + 1").contains("json-byte-limit"));
    }

    #[test]
    fn every_top_level_collection_rejects_its_max_plus_one() {
        let mut parent_pages = request(LayoutExportKind::Svg);
        Arc::make_mut(&mut parent_pages.snapshot).parent_pages = (0..=MAX_LAYOUT_EXPORT_PARENT_PAGES)
            .map(|index| crate::artifacts::layout::ParentPage { id: format!("parent-{index}"), name: "Parent".into(), width: 100.0, height: 100.0, layer_ids: Vec::new(), layers: Vec::new(), frames: Vec::new() })
            .collect();
        assert!(run_layout_export_headless_batch(operation(), parent_pages).expect_err("parent max + 1").contains("document-envelope"));

        let mut spreads = request(LayoutExportKind::Svg);
        Arc::make_mut(&mut spreads.snapshot).spreads = (0..=MAX_LAYOUT_EXPORT_SPREADS).map(|index| crate::artifacts::layout::Spread { id: format!("spread-{index}"), name: "Spread".into(), page_ids: Vec::new() }).collect();
        assert!(run_layout_export_headless_batch(operation(), spreads).expect_err("spread max + 1").contains("document-envelope"));

        let mut stories = request(LayoutExportKind::Svg);
        Arc::make_mut(&mut stories.snapshot).stories = (0..=MAX_LAYOUT_EXPORT_STORIES).map(|index| crate::artifacts::layout::TextStory { id: format!("story-{index}"), content: String::new(), style_runs: Vec::new() }).collect();
        assert!(run_layout_export_headless_batch(operation(), stories).expect_err("story max + 1").contains("document-envelope"));

        let mut links = request(LayoutExportKind::Svg);
        Arc::make_mut(&mut links.snapshot).links = (0..=MAX_LAYOUT_EXPORT_LINKS)
            .map(|index| crate::artifacts::layout::ImageLink { id: format!("link-{index}"), path: "image.png".into(), hash: String::new(), width: 1, height: 1, dpi: 72, color_profile: None, state: None, proxy_data_url: None })
            .collect();
        assert!(run_layout_export_headless_batch(operation(), links).expect_err("link max + 1").contains("document-envelope"));

        for character_styles in [false, true] {
            let mut styles = request(LayoutExportKind::Svg);
            let snapshot = Arc::make_mut(&mut styles.snapshot);
            if character_styles {
                let seed = crate::artifacts::layout::CharacterStyle { id: "character.seed".into(), name: None, font_family: None, font_size: None, font_weight: None, italic: None, color: None, tracking: None };
                snapshot.character_styles.resize(MAX_LAYOUT_EXPORT_STYLES + 1, seed);
            } else {
                let seed = snapshot.paragraph_styles[0].clone();
                snapshot.paragraph_styles.resize(MAX_LAYOUT_EXPORT_STYLES + 1, seed);
            }
            assert!(run_layout_export_headless_batch(operation(), styles).expect_err("style max + 1").contains("document-envelope"));
        }
    }

    #[test]
    fn nested_collection_string_and_json_caps_accept_max_and_reject_max_plus_one() {
        let mut max = request(LayoutExportKind::Svg);
        let snapshot = Arc::make_mut(&mut max.snapshot);
        let frame = snapshot.pages[0].frames[0].clone();
        let layer = snapshot.pages[0].layers[0].clone();
        let guide = crate::artifacts::layout::LayoutRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
        let page = &mut snapshot.pages[0];
        page.frames.resize(MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE, frame);
        page.overrides.resize(MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE, crate::artifacts::layout::PageOverride { object_id: "frame-1".into(), bounds: None, visible: None, locked: None });
        page.guides.resize(MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE, guide);
        page.layers.resize(MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE, layer);
        for layer in &mut page.layers {
            layer.object_ids.resize(MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE, "frame-1".into());
        }
        page.layer_ids.resize(MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE, "layer-1".into());
        snapshot.spreads[0].page_ids.resize(MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS, "page-1".into());
        snapshot.name = "n".repeat(MAX_LAYOUT_EXPORT_STRING_BYTES);
        snapshot.data_fields_json = Some(format!("[{}]", std::iter::repeat_n("null", MAX_LAYOUT_EXPORT_JSON_NODES - 1).collect::<Vec<_>>().join(",")));
        assert!(run_layout_export_headless_batch(operation(), max).is_ok());

        let mut frames = request(LayoutExportKind::Svg);
        let seed = frames.snapshot.pages[0].frames[0].clone();
        Arc::make_mut(&mut frames.snapshot).pages[0].frames.resize(MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE + 1, seed);
        assert!(run_layout_export_headless_batch(operation(), frames).expect_err("frame max + 1").contains("page-envelope"));

        let mut guides = request(LayoutExportKind::Svg);
        Arc::make_mut(&mut guides.snapshot).pages[0].guides.resize(MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE + 1, crate::artifacts::layout::LayoutRect { x: 0.0, y: 0.0, width: 1.0, height: 1.0 });
        assert!(run_layout_export_headless_batch(operation(), guides).expect_err("guide max + 1").contains("page-envelope"));

        let mut spread_ids = request(LayoutExportKind::Svg);
        Arc::make_mut(&mut spread_ids.snapshot).spreads[0].page_ids.resize(MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS + 1, "page-1".into());
        assert!(run_layout_export_headless_batch(operation(), spread_ids).expect_err("spread page id max + 1").contains("spread-envelope"));

        let mut string = request(LayoutExportKind::Svg);
        Arc::make_mut(&mut string.snapshot).name = "n".repeat(MAX_LAYOUT_EXPORT_STRING_BYTES + 1);
        assert!(run_layout_export_headless_batch(operation(), string).expect_err("string max + 1").contains("document-envelope"));

        let mut json_nodes = request(LayoutExportKind::Svg);
        Arc::make_mut(&mut json_nodes.snapshot).data_fields_json = Some(format!("[{}]", std::iter::repeat_n("null", MAX_LAYOUT_EXPORT_JSON_NODES).collect::<Vec<_>>().join(",")));
        assert!(run_layout_export_headless_batch(operation(), json_nodes).expect_err("json node max + 1").contains("json-node-limit"));

        let mut preflight_schema = request(LayoutExportKind::Package);
        preflight_schema.preflight_json = Some("{}".into());
        assert!(run_layout_export_headless_batch(operation(), preflight_schema).expect_err("preflight schema").contains("preflight-schema"));
    }

    #[test]
    fn exact_output_cap_accepts_max_and_rejects_one_more_byte() {
        let mut rope = ChunkRope::new();
        let chunk = vec![0u8; OUTPUT_CHUNK_BYTES];
        for _ in 0..MAX_LAYOUT_EXPORT_OUTPUT_BYTES / OUTPUT_CHUNK_BYTES {
            rope.append(&chunk).expect("exact output max");
        }
        assert_eq!(rope.len, MAX_LAYOUT_EXPORT_OUTPUT_BYTES);
        assert!(rope.append(&[0]).is_err());
        let mut drained = 0;
        while rope.take_chunk().is_some() {
            drained += 1;
        }
        assert_eq!(drained, MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS);
        assert!(rope.chunks.is_empty());
        assert_eq!(rope.front_byte_cursor, 0);
    }

    #[test]
    fn chunk_rope_outer_storage_is_stable_across_growth_boundaries() {
        let mut rope = ChunkRope::new();
        assert!(rope.chunks.capacity() >= MAX_LAYOUT_EXPORT_OUTPUT_CHUNKS);
        let chunk = vec![0u8; OUTPUT_CHUNK_BYTES];
        rope.append(&chunk).expect("first pre-admitted chunk");
        let storage = rope.chunks.as_slices().0.as_ptr();
        for _ in 1..257 {
            rope.append(&chunk).expect("pre-admitted chunk");
            assert_eq!(rope.chunks.as_slices().0.as_ptr(), storage);
        }
        assert_eq!(rope.chunks.len(), 257);
    }

    #[test]
    fn owned_crc32_is_standard_and_incremental() {
        assert_eq!(crc32_update(0, b"123456789"), 0xcbf4_3926);
        assert_eq!(crc32_update(crc32_update(0, b"1234"), b"56789"), 0xcbf4_3926);
    }

    #[test]
    fn typed_document_json_matches_serde_and_every_write_is_credit_bounded() {
        let mut snapshot = crate::artifacts::layout::schema::default_document();
        snapshot.name = "\u{1f642}\n".repeat(MAX_LAYOUT_EXPORT_STRING_BYTES / 5);
        snapshot.background_drawing = Some(crate::artifacts::layout::LayoutDrawingChild {
            handle: store::ArtifactChild::new("drawing-child".into(), store::os_io::ArtifactRef::parse_uri("document!s.stdio.semio@v1/drawing").expect("child reference")),
            content: Default::default(),
        });
        snapshot.referenced_model = Some(store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri("document!s.stdio.semio@v1/model").expect("model reference"), pin: store::LinkPin::Head, role: "model".into() });
        let expected = serde_json::to_vec(&snapshot).expect("reference JSON");
        let mut cursor = TypedJsonCursor::document();
        let mut actual = Vec::new();
        loop {
            let (bytes, done) = cursor.advance(&snapshot).expect("typed JSON step");
            assert!(bytes.len() <= JSON_OUTPUT_BYTES_PER_UNIT);
            actual.extend_from_slice(&bytes);
            if done {
                break;
            }
        }
        assert_eq!(actual, expected);
    }

    #[test]
    fn dynamic_json_validation_never_scans_more_than_its_input_credit() {
        let text = format!("[{}]", std::iter::repeat_n("null", MAX_LAYOUT_EXPORT_JSON_NODES - 1).collect::<Vec<_>>().join(","));
        let mut cursor = JsonValidationCursor::new(&text, true).expect("validator");
        loop {
            let before = cursor.byte_cursor;
            let done = cursor.advance(&text).expect("bounded validation step");
            assert!(cursor.byte_cursor - before <= JSON_INPUT_BYTES_PER_UNIT);
            if done {
                break;
            }
        }
    }

    #[test]
    fn terminal_candidate_is_empty_and_owned_chunks_never_exceed_four_kibibytes() {
        let operation = operation();
        let mut job = LayoutExportJob::new(operation, request(LayoutExportKind::Png)).expect("job");
        let snapshot_owner = Arc::clone(&job.request.snapshot);
        let chunks = job.output_chunks.clone();
        let params = BatchJobParams {
            operation: operation.operation,
            generation: operation.generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: BatchDriveConfig { site: "layout.export.segment-test", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
            now_us: semio_framework_job::default_now_us,
        };
        drop(snapshot_owner);
        let candidate = match drive_test_job(job, params.clone()) {
            StepOutcome::Complete(candidate) => candidate,
            outcome => panic!("unexpected terminal outcome: {outcome:?}"),
        };
        assert!(candidate.output.is_empty());
        let mut drained = 0;
        while let Some(chunk) = chunks.take_chunk().expect("sealed chunks") {
            assert!(!chunk.is_empty());
            assert!(chunk.len() <= OUTPUT_CHUNK_BYTES);
            drained += chunk.len();
        }
        assert!(drained > 0);
    }

    #[test]
    fn supplied_preflight_array_is_preserved_byte_for_byte_in_package_entry() {
        let snapshot = crate::artifacts::layout::schema::default_document();
        let supplied = r#"[{"kind":"custom","severity":"warning"}]"#;
        let json = serde_json::to_string(&snapshot).expect("document json");
        let package = export_package_zip_headless_batch(&json, supplied).expect("package");
        assert!(package.windows(supplied.len()).any(|window| window == supplied.as_bytes()));
    }

    #[test]
    fn one_unit_budget_forces_multiple_yields_and_stale_context_faults() {
        let operation = operation();
        let mut job = LayoutExportJob::new(operation, request(LayoutExportKind::Svg)).expect("job");
        let mut sequence = 0;
        for _ in 0..2 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence);
            assert_eq!(job.step(&mut context), StepOutcome::Yield);
        }
        let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence);
        assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
    }

    #[test]
    fn checkpoint_is_lossless_bounded_and_authority_qualified() {
        let operation = operation();
        let request_value = request(LayoutExportKind::Package);
        let mut job = LayoutExportJob::new(operation, request_value.clone()).expect("job");
        let cancel = semio_framework_job::root_cancel_token();
        let mut sequence = 0;
        let mut checkpoint = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
                break checkpoint;
            }
        };
        assert_eq!(checkpoint.state.len(), MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES);
        let checkpoint_state = checkpoint.state.single_page().expect("checkpoint owns one retained page").to_vec();
        while !checkpoint.state.terminal_is_empty() {
            let _ = checkpoint.state.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        let output_chunks = ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES);
        let restored = LayoutExportJob::restore(operation, request_value.clone(), &checkpoint_state).expect("matching authority").with_output_chunks(output_chunks.clone());
        let params = BatchJobParams {
            operation: operation.operation,
            generation: operation.generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: BatchDriveConfig { site: "layout.export.restore-test", stage: InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
            now_us: semio_framework_job::default_now_us,
        };
        match drive_test_job(restored, params.clone()) {
            StepOutcome::Complete(candidate) => assert!(candidate.output.is_empty()),
            outcome => panic!("resumed outcome: {outcome:?}"),
        }
        let resumed = LayoutExportCommit::from_chunks(LayoutExportKind::Package, "layout", &output_chunks).expect("drained resumed output").data.into_bytes();
        let uninterrupted = run_layout_export_headless_batch(operation, request_value).expect("uninterrupted").data.into_bytes();
        assert_eq!(resumed, uninterrupted);
        let stale = Operation::new(semio_framework_job::OperationId(72), RevisionId(9), Generation(3), 17);
        assert!(LayoutExportJob::restore(stale, request(LayoutExportKind::Package), &checkpoint_state).is_err());
        let mut malformed = checkpoint_state;
        malformed[0] ^= 0xff;
        assert!(LayoutExportJob::restore(operation, request(LayoutExportKind::Package), &malformed).is_err());
    }
}
//#endregion 🧪️Tests
