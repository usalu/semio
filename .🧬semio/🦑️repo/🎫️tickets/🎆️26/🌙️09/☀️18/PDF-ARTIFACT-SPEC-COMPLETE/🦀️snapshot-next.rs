//! 🧬️ PdfSnapshot schema (1.7) — the typed ISO 32000-1 document model. Ticket
//! 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE widened the former text-only page view into the full
//! vocabulary: pages carry their content stream as typed operators (§8 graphics, §9 text) over
//! named resources; fonts (§9.6–9.10), images and form XObjects (§8.9–8.10), colour spaces,
//! functions, shadings and patterns (§8.6–8.7), extended graphics states (§8.4.5), annotations and
//! actions (§12.5–12.6), outlines, destinations and page labels (§12.3–12.4), embedded files
//! (§7.11), interactive forms (§12.7), optional content (§8.11), document metadata (§14.3) and
//! the standard security handler (§7.6) are document-level collections keyed by id.
//!
//! Two lanes, one carrier: the typed lanes are what mutations edit and what every consumer builds;
//! `objects`/`trailer` is the retained COS graph the file was read from (lossless carrier, what the
//! conformance subsets inspect). `🚪️io` lifts the typed lanes out of the graph on decode and lowers
//! them back onto it on encode — see that module for the fixed-point law `lift(lower(t)) == t`.
//!
//! Ground rule: byte/token parsing lives in `🚪️io`; this module is the *typed* logical model.
//! Native lexical choices and encoded stream representations never enter the typed lanes.

use framework_schema::ArtifactSchema;
use std::fmt;

/// 🏷️ Document schema id for `stdio.pdf` (1.7) -- deliberately distinct from 1.4's flat
/// `stdio.pdf` (avoids colliding with 1.4's own `store::register_document_codec` registration,
/// which the flat pre-D4 registry can't disambiguate by standard; same shape as gif 89a's
/// `STDIO_GIF89A_DOCUMENT_SCHEMA`, see that module's doc comment).
pub const STDIO_PDF17_DOCUMENT_SCHEMA: &str = "stdio.pdf.1.7";

//#region 🔖️ObjectModel
/// 🔗️ An indirect-object reference `N G R` — also the `objects` collection's diff KEY (the
/// `(id,gen)` pair per the recipe's "numeric id" key kind; `Hash` is needed by the diff module's
/// key-transport absorb maps).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjRef {
    pub num: u32,
    pub gen: u16,
}

/// 🧩 One `key`/`value` pair of a PDF dictionary. A `Vec` (not a map) so parse order survives
/// losslessly -- PDF dictionaries have no canonical key order and real files vary widely.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfDictEntry {
    pub key: String,
    pub value: PdfObject,
}

impl PdfDictEntry {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(key: impl Into<String>, value: PdfObject) -> Self {
        Self { key: key.into(), value }
    }
}

/// 🎛️ Logical predictor parameters attached to a Flate/LZW stream filter (§7.4.4.4).
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPredictor {
    pub predictor: u32,
    pub colors: u32,
    pub bits_per_component: u32,
    pub columns: u32,
}

/// 📠️ CCITTFaxDecode parameters (§7.4.6, Table 11).
#[derive(Clone, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfCcittParameters {
    #[value(default)]
    pub k: i32,
    #[value(default = "PdfCcittParameters::default_columns")]
    pub columns: u32,
    #[value(default)]
    pub rows: u32,
    #[value(default)]
    pub black_is_1: bool,
    #[value(default)]
    pub encoded_byte_align: bool,
    #[value(default)]
    pub end_of_line: bool,
    #[value(default = "PdfCcittParameters::default_end_of_block")]
    pub end_of_block: bool,
    #[value(default)]
    pub damaged_rows_before_error: u32,
}

impl PdfCcittParameters {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn default_columns() -> u32 {
        1728
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn default_end_of_block() -> bool {
        true
    }
}

/// 🗜️ Stream filter concepts (§7.4). The first five have logical codecs: their `data` is the
/// DECODED byte sequence. The image codecs (`Dct`, `Jpx`, `Ccitt`, `Jbig2`) are retained as they
/// stand — `data` is still the encoded image; a conforming reader hands such a stream to its image
/// decoder, and this codec hands it back byte for byte. `Crypt` names an identity/explicit crypt
/// filter (§7.6.5) and is likewise transparent.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfStreamFilter {
    Flate { predictor: Option<PdfPredictor> },
    Lzw { predictor: Option<PdfPredictor>, early_change: bool },
    AsciiHex,
    Ascii85,
    RunLength,
    Dct { color_transform: Option<u32> },
    Jpx,
    Ccitt { parameters: PdfCcittParameters },
    Jbig2 { globals: Option<Vec<u8>> },
    Crypt { name: Option<String> },
}

impl PdfStreamFilter {
    /// 🖼️ Whether this filter is an image codec whose payload stays encoded in the typed model.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_image_codec(&self) -> bool {
        matches!(self, PdfStreamFilter::Dct { .. } | PdfStreamFilter::Jpx | PdfStreamFilter::Ccitt { .. } | PdfStreamFilter::Jbig2 { .. })
    }
    /// 🏷️ The `/Filter` name this concept is written as.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn name(&self) -> &'static str {
        match self {
            PdfStreamFilter::Flate { .. } => "FlateDecode",
            PdfStreamFilter::Lzw { .. } => "LZWDecode",
            PdfStreamFilter::AsciiHex => "ASCIIHexDecode",
            PdfStreamFilter::Ascii85 => "ASCII85Decode",
            PdfStreamFilter::RunLength => "RunLengthDecode",
            PdfStreamFilter::Dct { .. } => "DCTDecode",
            PdfStreamFilter::Jpx => "JPXDecode",
            PdfStreamFilter::Ccitt { .. } => "CCITTFaxDecode",
            PdfStreamFilter::Jbig2 { .. } => "JBIG2Decode",
            PdfStreamFilter::Crypt { .. } => "Crypt",
        }
    }
}

/// 🔢️ Exact logical PDF real number represented as decimal coefficient and scale.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfDecimal {
    pub negative: bool,
    pub coefficient: String,
    pub scale: u32,
}

impl PdfDecimal {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn parse(text: &str) -> Result<Self, String> {
        let (negative, unsigned) = match text.as_bytes().first() {
            Some(b'-') => (true, &text[1..]),
            Some(b'+') => (false, &text[1..]),
            _ => (false, text),
        };
        let (integer, fraction) = unsigned.split_once('.').ok_or_else(|| format!("PDF real requires decimal point: {text:?}"))?;
        if (integer.is_empty() && fraction.is_empty()) || !integer.bytes().all(|byte| byte.is_ascii_digit()) || !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(format!("invalid PDF real: {text:?}"));
        }
        let integer = if integer.is_empty() { "0" } else { integer };
        Ok(Self { negative, coefficient: format!("{integer}{fraction}"), scale: fraction.len() as u32 })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_f64(value: f64) -> Self {
        let text = format!("{value}");
        let (mantissa, exponent) = match text.find(['e', 'E']) {
            Some(index) => (&text[..index], text[index + 1..].parse::<i32>().expect("finite f64 exponent")),
            None => (text.as_str(), 0),
        };
        let normalized = if mantissa.contains('.') { mantissa.to_string() } else { format!("{mantissa}.") };
        let mut decimal = Self::parse(&normalized).expect("finite f64 has valid decimal mantissa");
        let scale = decimal.scale as i64 - exponent as i64;
        if scale < 0 {
            decimal.coefficient.extend(std::iter::repeat_n('0', -scale as usize));
            decimal.scale = 0;
        } else {
            decimal.scale = scale as u32;
        }
        decimal
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_f64(&self) -> Option<f64> {
        self.to_string().parse().ok()
    }
}

impl From<f64> for PdfDecimal {
    fn from(value: f64) -> Self {
        Self::from_f64(value)
    }
}

impl fmt::Display for PdfDecimal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            formatter.write_str("-")?;
        }
        let scale = self.scale as usize;
        if scale >= self.coefficient.len() {
            formatter.write_str("0.")?;
            for _ in 0..scale.saturating_sub(self.coefficient.len()) {
                formatter.write_str("0")?;
            }
            formatter.write_str(&self.coefficient)
        } else {
            let split = self.coefficient.len() - scale;
            write!(formatter, "{}.{}", &self.coefficient[..split], &self.coefficient[split..])
        }
    }
}

/// 🎯 A parsed PDF object -- the full COS object grammar (ISO 32000-1 §7.3), including
/// streams. Stream `data` is the logical byte sequence after applying every filter that has a
/// logical decoder (@see [`PdfStreamFilter`]); `/Filter`, `/F`, `/DecodeParms`, and `/DP` are
/// removed during native deserialization and regenerated from `filters` on write.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[derive(Default)]
pub enum PdfObject {
    #[default]
    Null,
    Bool(bool),
    Int(i64),
    Real(PdfDecimal),
    Str(Vec<u8>),
    Name(String),
    Array(Vec<PdfObject>),
    Dict(Vec<PdfDictEntry>),
    Ref(ObjRef),
    Stream {
        dict: Vec<PdfDictEntry>,
        data: Vec<u8>,
        filters: Vec<PdfStreamFilter>,
    },
}

impl PdfObject {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_dict(&self) -> Option<&[PdfDictEntry]> {
        match self {
            PdfObject::Dict(d) => Some(d),
            PdfObject::Stream { dict, .. } => Some(dict),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn dict_get<'a>(&'a self, key: &str) -> Option<&'a PdfObject> {
        self.as_dict()?.iter().find(|e| e.key == key).map(|e| &e.value)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_name(&self) -> Option<&str> {
        match self {
            PdfObject::Name(n) => Some(n.as_str()),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_ref(&self) -> Option<ObjRef> {
        match self {
            PdfObject::Ref(r) => Some(*r),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_array(&self) -> Option<&[PdfObject]> {
        match self {
            PdfObject::Array(a) => Some(a),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_str_bytes(&self) -> Option<&[u8]> {
        match self {
            PdfObject::Str(bytes) => Some(bytes),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            PdfObject::Bool(value) => Some(*value),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            PdfObject::Int(i) => Some(*i as f64),
            PdfObject::Real(r) => r.to_f64(),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            PdfObject::Int(i) => Some(*i),
            PdfObject::Real(r) => r.to_f64().map(|value| value as i64),
            _ => None,
        }
    }
    /// 🔢️ A number operand written in canonical form: integers stay integers, everything else is
    /// an exact decimal (never exponent notation, which PDF has no syntax for).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn number(value: f64) -> PdfObject {
        if value.is_finite() && value.fract() == 0.0 && value.abs() < 1e15 {
            PdfObject::Int(value as i64)
        } else if value.is_finite() {
            PdfObject::Real(PdfDecimal::from_f64(value))
        } else {
            PdfObject::Int(0)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn name(name: impl Into<String>) -> PdfObject {
        PdfObject::Name(name.into())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn numbers(values: &[f64]) -> PdfObject {
        PdfObject::Array(values.iter().map(|value| PdfObject::number(*value)).collect())
    }
}

/// 🗄️ One `N G obj ... endobj` indirect object, keyed by its `id`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfIndirectObject {
    pub id: ObjRef,
    pub value: PdfObject,
}
//#endregion 🔖️ObjectModel

//#region 🔖️Geometry
/// 📐️ `[llx lly urx ury]` rectangle in default user space (§7.9.5).
pub type PdfRect = [f64; 4];
/// 🔀️ `[a b c d e f]` transformation matrix (§8.3.3).
pub type PdfMatrix = [f64; 6];
/// 🆔 Identity matrix.
pub const PDF_IDENTITY_MATRIX: PdfMatrix = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
//#endregion 🔖️Geometry

//#region 🔖️Content
/// 🔤️ A string operand of a text-showing operator. `Text` is Unicode recovered through the
/// selected font's encoding (and written back through it); `Codes` are the raw character codes
/// whenever the font cannot map them to Unicode and back, so nothing is fabricated (§9.4.3).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfTextString {
    Text { text: String },
    Codes { bytes: Vec<u8> },
}

impl PdfTextString {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }
}

/// 🧵 One element of a `TJ` array: a string or a horizontal adjustment in thousandths of text
/// space (§9.4.3, Table 109).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfTextArrayItem {
    Text { text: String },
    Codes { bytes: Vec<u8> },
    Adjust { amount: f64 },
}

/// 🏷️ The property-list operand of `DP`/`BDC` (§14.6): a named resource or an inline dictionary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfPropertyList {
    Named { name: String },
    Inline { entries: Vec<PdfDictEntry> },
}

/// 🖼️ An inline image (`BI … ID … EI`, §8.9.7). Abbreviated keys are expanded on read; `data` is
/// decoded except for image codecs, exactly as for [`PdfObject::Stream`].
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfInlineImage {
    pub width: u32,
    pub height: u32,
    #[value(default)]
    pub bits_per_component: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color_space: Option<PdfColorSpace>,
    #[value(default)]
    pub image_mask: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub decode: Vec<f64>,
    #[value(default)]
    pub interpolate: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<PdfStreamFilter>,
    pub data: Vec<u8>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

/// ✂️ Line cap style (§8.4.3.3).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PdfLineCap {
    #[default]
    Butt,
    Round,
    Square,
}

/// 🔗️ Line join style (§8.4.3.4).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PdfLineJoin {
    #[default]
    Miter,
    Round,
    Bevel,
}

/// 🖋️ One content-stream operator with typed operands — every operator of ISO 32000-1 Table 51.
/// Painting order is the `Vec<PdfOp>` order; nothing is inferred or re-ordered on either side.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "op", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfOp {
    // 🎛️ General graphics state (Table 57)
    SetLineWidth { width: f64 },
    SetLineCap { cap: PdfLineCap },
    SetLineJoin { join: PdfLineJoin },
    SetMiterLimit { limit: f64 },
    SetDash { array: Vec<f64>, phase: f64 },
    SetRenderingIntent { intent: String },
    SetFlatness { flatness: f64 },
    SetExtGState { name: String },
    // 🧷 Special graphics state
    Save,
    Restore,
    Transform { matrix: PdfMatrix },
    // ✏️ Path construction (Table 59)
    MoveTo { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    CurveTo { x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64 },
    CurveToInitial { x2: f64, y2: f64, x3: f64, y3: f64 },
    CurveToFinal { x1: f64, y1: f64, x3: f64, y3: f64 },
    ClosePath,
    Rectangle { x: f64, y: f64, width: f64, height: f64 },
    // 🎨 Path painting (Table 60)
    Stroke,
    CloseStroke,
    Fill,
    FillEvenOdd,
    FillStroke,
    FillStrokeEvenOdd,
    CloseFillStroke,
    CloseFillStrokeEvenOdd,
    EndPath,
    // ✂️ Clipping (Table 61)
    Clip,
    ClipEvenOdd,
    // 🔤 Text objects, state, positioning, showing (Tables 107, 105, 108, 109)
    BeginText,
    EndText,
    SetCharSpacing { spacing: f64 },
    SetWordSpacing { spacing: f64 },
    SetHorizontalScale { scale: f64 },
    SetLeading { leading: f64 },
    SetFont { name: String, size: f64 },
    SetTextRenderingMode { mode: u32 },
    SetTextRise { rise: f64 },
    MoveText { tx: f64, ty: f64 },
    MoveTextSetLeading { tx: f64, ty: f64 },
    SetTextMatrix { matrix: PdfMatrix },
    NextLine,
    ShowText { text: PdfTextString },
    ShowTextArray { items: Vec<PdfTextArrayItem> },
    NextLineShowText { text: PdfTextString },
    NextLineShowTextSpaced { word_spacing: f64, char_spacing: f64, text: PdfTextString },
    // 🔠 Type 3 glyph metrics (Table 113)
    SetGlyphWidth { wx: f64, wy: f64 },
    SetGlyphWidthAndBox { wx: f64, wy: f64, llx: f64, lly: f64, urx: f64, ury: f64 },
    // 🌈 Colour (Table 74)
    SetStrokeColorSpace { name: String },
    SetFillColorSpace { name: String },
    SetStrokeColor { components: Vec<f64> },
    SetStrokeColorN { components: Vec<f64>, pattern: Option<String> },
    SetFillColor { components: Vec<f64> },
    SetFillColorN { components: Vec<f64>, pattern: Option<String> },
    SetStrokeGray { gray: f64 },
    SetFillGray { gray: f64 },
    SetStrokeRgb { r: f64, g: f64, b: f64 },
    SetFillRgb { r: f64, g: f64, b: f64 },
    SetStrokeCmyk { c: f64, m: f64, y: f64, k: f64 },
    SetFillCmyk { c: f64, m: f64, y: f64, k: f64 },
    // 🖼️ Shading, XObjects, inline images (Tables 77, 87, 92)
    PaintShading { name: String },
    PaintXObject { name: String },
    InlineImage { image: PdfInlineImage },
    // 🏷️ Marked content (Table 320)
    MarkedContentPoint { tag: String },
    MarkedContentPointWithProperties { tag: String, properties: PdfPropertyList },
    BeginMarkedContent { tag: String },
    BeginMarkedContentWithProperties { tag: String, properties: PdfPropertyList },
    EndMarkedContent,
    // 🧯 Compatibility (Table 32)
    BeginCompatibility,
    EndCompatibility,
    // 🧳 An operator this codec does not know, retained with its operands so nothing is dropped.
    Unknown { operator: String, operands: Vec<PdfObject> },
}
//#endregion 🔖️Content

//#region 🔖️Colour
/// 🌈 A colour space (§8.6). `Named` refers to an entry of the current resource dictionary's
/// `/ColorSpace` sub-dictionary and only appears where the spec allows a name.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfColorSpace {
    DeviceGray,
    DeviceRgb,
    DeviceCmyk,
    CalGray { white_point: [f64; 3], black_point: Option<[f64; 3]>, gamma: Option<f64> },
    CalRgb { white_point: [f64; 3], black_point: Option<[f64; 3]>, gamma: Option<[f64; 3]>, matrix: Option<[f64; 9]> },
    Lab { white_point: [f64; 3], black_point: Option<[f64; 3]>, range: Option<[f64; 4]> },
    IccBased { components: u32, profile: Vec<u8>, alternate: Option<Box<PdfColorSpace>>, range: Option<Vec<f64>> },
    Indexed { base: Box<PdfColorSpace>, hival: u32, lookup: Vec<u8> },
    Separation { name: String, alternate: Box<PdfColorSpace>, tint_transform: PdfFunction },
    DeviceN { names: Vec<String>, alternate: Box<PdfColorSpace>, tint_transform: PdfFunction, attributes: Option<Vec<PdfDictEntry>> },
    Pattern { base: Option<Box<PdfColorSpace>> },
    Named { name: String },
}

impl PdfColorSpace {
    /// 🔢️ Number of components a colour in this space carries (`None` for `Pattern`/`Named`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn components(&self) -> Option<u32> {
        Some(match self {
            PdfColorSpace::DeviceGray | PdfColorSpace::CalGray { .. } | PdfColorSpace::Indexed { .. } | PdfColorSpace::Separation { .. } => 1,
            PdfColorSpace::DeviceRgb | PdfColorSpace::CalRgb { .. } | PdfColorSpace::Lab { .. } => 3,
            PdfColorSpace::DeviceCmyk => 4,
            PdfColorSpace::IccBased { components, .. } => *components,
            PdfColorSpace::DeviceN { names, .. } => names.len() as u32,
            PdfColorSpace::Pattern { .. } | PdfColorSpace::Named { .. } => return None,
        })
    }
}

/// 🧮 A PDF function (§7.10): sampled (type 0), exponential (2), stitching (3), PostScript
/// calculator (4, retained as its source text), or an array of 1-out functions.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfFunction {
    Sampled { domain: Vec<f64>, range: Vec<f64>, size: Vec<u32>, bits_per_sample: u32, order: Option<u32>, encode: Option<Vec<f64>>, decode: Option<Vec<f64>>, samples: Vec<u8> },
    Exponential { domain: Vec<f64>, range: Option<Vec<f64>>, c0: Vec<f64>, c1: Vec<f64>, n: f64 },
    Stitching { domain: Vec<f64>, range: Option<Vec<f64>>, functions: Vec<PdfFunction>, bounds: Vec<f64>, encode: Vec<f64> },
    PostScript { domain: Vec<f64>, range: Vec<f64>, code: String },
    Array { functions: Vec<PdfFunction> },
}

/// 🌅 A shading (§8.7.4.5). Mesh types 4–7 keep their packed vertex data with the decode
/// parameters needed to read it.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfShadingKind {
    FunctionBased { domain: Option<[f64; 4]>, matrix: Option<PdfMatrix>, function: PdfFunction },
    Axial { coords: [f64; 4], domain: Option<[f64; 2]>, function: PdfFunction, extend: [bool; 2] },
    Radial { coords: [f64; 6], domain: Option<[f64; 2]>, function: PdfFunction, extend: [bool; 2] },
    Mesh { shading_type: u32, bits_per_coordinate: u32, bits_per_component: u32, bits_per_flag: Option<u32>, vertices_per_row: Option<u32>, decode: Vec<f64>, function: Option<PdfFunction>, data: Vec<u8> },
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfShading {
    pub id: String,
    pub color_space: PdfColorSpace,
    pub kind: PdfShadingKind,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<Vec<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bbox: Option<PdfRect>,
    #[value(default)]
    pub anti_alias: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

/// 🧩 A pattern (§8.7.3): tiling patterns paint a content cell, shading patterns reference a
/// shading.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfPatternKind {
    Tiling { paint_type: u32, tiling_type: u32, bbox: PdfRect, x_step: f64, y_step: f64, content: Vec<PdfOp> },
    Shading { shading: String, ext_g_state: Option<String> },
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPattern {
    pub id: String,
    #[value(default = "PdfPattern::identity")]
    pub matrix: PdfMatrix,
    pub kind: PdfPatternKind,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfPattern {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn identity() -> PdfMatrix {
        PDF_IDENTITY_MATRIX
    }
}
//#endregion 🔖️Colour

//#region 🔖️GraphicsState
/// 🫥 The `/SMask` entry of an extended graphics state (§11.6.5): `None` (the name `/None`,
/// which switches soft masking off), or an alpha/luminosity mask drawn from the form XObject
/// `group`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfSoftMask {
    None,
    Alpha { group: String, transfer: Option<PdfFunction> },
    Luminosity { group: String, backdrop: Option<Vec<f64>>, transfer: Option<PdfFunction> },
}

/// 🎛️ An extended graphics state parameter dictionary (§8.4.5, Table 58). Every entry is optional;
/// absent means "leave as is".
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfExtGState {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub line_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub line_cap: Option<PdfLineCap>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub line_join: Option<PdfLineJoin>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub miter_limit: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dash: Option<(Vec<f64>, f64)>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rendering_intent: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overprint_stroke: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overprint_fill: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overprint_mode: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<(String, f64)>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<Vec<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub soft_mask: Option<PdfSoftMask>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke_alpha: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fill_alpha: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alpha_is_shape: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke_adjust: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub flatness: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub smoothness: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text_knockout: Option<bool>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}
//#endregion 🔖️GraphicsState

//#region 🔖️Resources
/// 🌈 One `name → colour space` entry of the document's `/ColorSpace` resources. Content
/// operators (`cs`/`CS`) reference the name; every page and form binds what it uses on write.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfNamedColorSpace {
    pub name: String,
    pub color_space: PdfColorSpace,
}

/// 🏷️ One `name → property list` entry of the document's `/Properties` resources (§14.6.2),
/// referenced by `DP`/`BDC` operators.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfNamedProperties {
    pub name: String,
    pub entries: Vec<PdfDictEntry>,
}
//#endregion 🔖️Resources

//#region 🔖️Fonts
/// 🔡 A predefined simple-font base encoding (§9.6.6, Annex D).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PdfBaseEncoding {
    Standard,
    WinAnsi,
    MacRoman,
    MacExpert,
}

/// 🔡 One `/Differences` entry: `code` shows the glyph named `glyph`.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfEncodingDifference {
    pub code: u32,
    pub glyph: String,
}

/// 🔡 A simple font's encoding: an optional base encoding (`None` = the font's built-in encoding)
/// plus `/Differences` (§9.6.6.1).
#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfSimpleEncoding {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<PdfBaseEncoding>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub differences: Vec<PdfEncodingDifference>,
}

impl PdfSimpleEncoding {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn win_ansi() -> Self {
        Self { base: Some(PdfBaseEncoding::WinAnsi), differences: Vec::new() }
    }
}

/// 📏 A font descriptor (§9.8, Table 122). Widths live on the font itself.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfFontDescriptor {
    pub font_name: String,
    #[value(default)]
    pub flags: u32,
    #[value(default)]
    pub font_bbox: PdfRect,
    #[value(default)]
    pub italic_angle: f64,
    #[value(default)]
    pub ascent: f64,
    #[value(default)]
    pub descent: f64,
    #[value(default)]
    pub cap_height: f64,
    #[value(default)]
    pub stem_v: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stem_h: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x_height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub leading: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub avg_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub missing_width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font_stretch: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub char_set: Option<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

/// 💾 An embedded font program (§9.9, Table 126).
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfFontProgram {
    /// `FontFile`: Type 1 with its clear-text/encrypted/zeros segment lengths.
    Type1 { data: Vec<u8>, length1: u32, length2: u32, length3: u32 },
    /// `FontFile2`: a TrueType (sfnt) program.
    TrueType { data: Vec<u8> },
    /// `FontFile3` with `/Subtype /Type1C`.
    Cff { data: Vec<u8> },
    /// `FontFile3` with `/Subtype /CIDFontType0C`.
    CidCff { data: Vec<u8> },
    /// `FontFile3` with `/Subtype /OpenType`.
    OpenType { data: Vec<u8> },
}

/// 🈴 One `ToUnicode` mapping (§9.10.3): a character code (of `byte_width` bytes) to a Unicode
/// string, or a contiguous range whose destinations increment from `text`'s last code point.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfToUnicodeMapping {
    Char { code: u32, text: String },
    Range { low: u32, high: u32, text: String },
}

/// 🈴 A `ToUnicode` CMap in typed form.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfToUnicode {
    pub byte_width: u32,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<PdfToUnicodeMapping>,
}

/// 🗺️ One code-space range of a CMap (§9.7.5.2).
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfCodespaceRange {
    pub byte_width: u32,
    pub low: u32,
    pub high: u32,
}

/// 🗺️ One `cidchar`/`cidrange` entry of an embedded CMap.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfCidMapping {
    Char { code: u32, cid: u32 },
    Range { low: u32, high: u32, cid: u32 },
}

/// 🗺️ An embedded CMap stream (§9.7.5.3) in typed form.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfEmbeddedCMap {
    pub name: String,
    #[value(default)]
    pub vertical: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub codespace: Vec<PdfCodespaceRange>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<PdfCidMapping>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub use_cmap: Option<String>,
}

/// 🗺️ The `/Encoding` of a Type 0 font (§9.7.5): a predefined CMap by name, or an embedded one.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfCMap {
    Predefined { name: String },
    Embedded { cmap: PdfEmbeddedCMap },
}

impl PdfCMap {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn identity_h() -> Self {
        Self::Predefined { name: "Identity-H".into() }
    }
}

/// 📇 `/CIDSystemInfo` (§9.7.3).
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfCidSystemInfo {
    pub registry: String,
    pub ordering: String,
    pub supplement: u32,
}

impl Default for PdfCidSystemInfo {
    fn default() -> Self {
        Self { registry: "Adobe".into(), ordering: "Identity".into(), supplement: 0 }
    }
}

/// 📏 One run of a CIDFont `/W` array: consecutive widths from `start_cid` (§9.7.4.3).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfCidWidthRun {
    pub start_cid: u32,
    pub widths: Vec<f64>,
}

/// 📏 One `/W2` vertical-metrics run (§9.7.4.3): per-CID `[w1y vx vy]` triples.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfCidVerticalRun {
    pub start_cid: u32,
    pub metrics: Vec<[f64; 3]>,
}

/// 🔢 `/CIDToGIDMap` (§9.7.4.2).
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfCidToGid {
    Identity,
    Map { data: Vec<u8> },
}

/// 🔤 A CIDFont (§9.7.4), the descendant of a Type 0 font.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfCidFont {
    pub true_type: bool,
    pub base_font: String,
    #[value(default)]
    pub system_info: PdfCidSystemInfo,
    pub descriptor: PdfFontDescriptor,
    #[value(default = "PdfCidFont::default_width")]
    pub default_width: f64,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub widths: Vec<PdfCidWidthRun>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_vertical: Option<[f64; 2]>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub vertical_metrics: Vec<PdfCidVerticalRun>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cid_to_gid: Option<PdfCidToGid>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub program: Option<PdfFontProgram>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfCidFont {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn default_width() -> f64 {
        1000.0
    }
}

/// 🔠 One Type 3 glyph procedure (§9.6.5): a content stream drawing the glyph named `name`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfCharProc {
    pub name: String,
    pub content: Vec<PdfOp>,
}

/// 🔤 A font's subtype-specific data (§9.5–9.7). The standard 14 fonts are `Type1` fonts with
/// no program and no descriptor; their metrics come from the writer's built-in AFM tables.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfFontKind {
    Type1 { base_font: String, encoding: PdfSimpleEncoding, first_char: u32, widths: Vec<f64>, descriptor: Option<PdfFontDescriptor>, program: Option<PdfFontProgram> },
    TrueType { base_font: String, encoding: PdfSimpleEncoding, first_char: u32, widths: Vec<f64>, descriptor: Option<PdfFontDescriptor>, program: Option<PdfFontProgram> },
    Type3 { font_matrix: PdfMatrix, font_bbox: PdfRect, encoding: PdfSimpleEncoding, first_char: u32, widths: Vec<f64>, char_procs: Vec<PdfCharProc>, descriptor: Option<PdfFontDescriptor> },
    Type0 { base_font: String, cmap: PdfCMap, descendant: PdfCidFont },
}

/// 🔤 A document font, keyed by `id` — the resource name content operators (`Tf`) use.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfFont {
    pub id: String,
    pub kind: PdfFontKind,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub to_unicode: Option<PdfToUnicode>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfFont {
    /// 🅰️ One of the standard 14 Type 1 fonts (§9.6.2.2) with WinAnsi encoding (or the built-in
    /// encoding for Symbol/ZapfDingbats), no descriptor and no program.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn standard(id: impl Into<String>, base_font: &str) -> Self {
        let symbolic = matches!(base_font, "Symbol" | "ZapfDingbats");
        Self { id: id.into(), kind: PdfFontKind::Type1 { base_font: base_font.into(), encoding: if symbolic { PdfSimpleEncoding::default() } else { PdfSimpleEncoding::win_ansi() }, first_char: 0, widths: Vec::new(), descriptor: None, program: None }, to_unicode: None, extra: Vec::new() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn base_font(&self) -> &str {
        match &self.kind {
            PdfFontKind::Type1 { base_font, .. } | PdfFontKind::TrueType { base_font, .. } | PdfFontKind::Type0 { base_font, .. } => base_font,
            PdfFontKind::Type3 { .. } => "",
        }
    }
    /// 🔢 Whether character codes of this font are two bytes wide (Type 0 with a two-byte CMap).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_composite(&self) -> bool {
        matches!(self.kind, PdfFontKind::Type0 { .. })
    }
}
//#endregion 🔖️Fonts

//#region 🔖️XObjects
/// 🖼️ How an image XObject's `data` is encoded: raw packed samples (rows padded to byte
/// boundaries, `bits_per_component` bits each) or a retained image codec bitstream.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfImageCodec {
    Raw,
    Dct { color_transform: Option<u32> },
    Jpx,
    Ccitt { parameters: PdfCcittParameters },
    Jbig2 { globals: Option<Vec<u8>> },
}

/// 🎭 An image's explicit mask (§8.9.6): a stencil image id or colour-key ranges.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfImageMask {
    Stencil { image: String },
    ColorKey { ranges: Vec<u32> },
}

/// 🖼️ An image XObject (§8.9.5, Table 89), keyed by `id`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfImage {
    pub id: String,
    pub width: u32,
    pub height: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color_space: Option<PdfColorSpace>,
    #[value(default)]
    pub bits_per_component: u32,
    #[value(default)]
    pub image_mask: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub decode: Vec<f64>,
    #[value(default)]
    pub interpolate: bool,
    #[value(default = "PdfImage::raw")]
    pub codec: PdfImageCodec,
    pub data: Vec<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub soft_mask: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub soft_mask_in_data: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<PdfImageMask>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub matte: Option<Vec<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub optional_content: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub struct_parent: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfImage {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn raw() -> PdfImageCodec {
        PdfImageCodec::Raw
    }
    /// 🖼️ An 8-bit RGB image from packed `rgb` rows.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rgb8(id: impl Into<String>, width: u32, height: u32, rgb: Vec<u8>) -> Self {
        Self::samples(id, width, height, PdfColorSpace::DeviceRgb, 8, rgb)
    }
    /// 🖼️ An 8-bit grayscale image (also the shape of a soft mask) from packed rows.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn gray8(id: impl Into<String>, width: u32, height: u32, gray: Vec<u8>) -> Self {
        Self::samples(id, width, height, PdfColorSpace::DeviceGray, 8, gray)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn samples(id: impl Into<String>, width: u32, height: u32, color_space: PdfColorSpace, bits_per_component: u32, data: Vec<u8>) -> Self {
        Self {
            id: id.into(),
            width,
            height,
            color_space: Some(color_space),
            bits_per_component,
            image_mask: false,
            decode: Vec::new(),
            interpolate: false,
            codec: PdfImageCodec::Raw,
            data,
            soft_mask: None,
            soft_mask_in_data: None,
            mask: None,
            matte: None,
            intent: None,
            optional_content: None,
            struct_parent: None,
            extra: Vec::new(),
        }
    }
    /// 📷 A JPEG (DCT) image whose bitstream is embedded as is.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn jpeg(id: impl Into<String>, width: u32, height: u32, color_space: PdfColorSpace, jpeg: Vec<u8>) -> Self {
        let mut image = Self::samples(id, width, height, color_space, 8, jpeg);
        image.codec = PdfImageCodec::Dct { color_transform: None };
        image
    }
    /// 📐 Bytes per row of a raw image (`None` for retained codecs).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn row_bytes(&self) -> Option<usize> {
        if self.codec != PdfImageCodec::Raw {
            return None;
        }
        let components = if self.image_mask { 1 } else { self.color_space.as_ref().and_then(PdfColorSpace::components).unwrap_or(1) };
        let bits = if self.image_mask { 1 } else { self.bits_per_component.max(1) };
        Some((self.width as usize * components as usize * bits as usize).div_ceil(8))
    }
}

/// 🫧 A transparency group attribute dictionary (§11.6.6).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfTransparencyGroup {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color_space: Option<PdfColorSpace>,
    #[value(default)]
    pub isolated: bool,
    #[value(default)]
    pub knockout: bool,
}

/// 📄 A form XObject (§8.10), keyed by `id`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfFormXObject {
    pub id: String,
    pub bbox: PdfRect,
    #[value(default = "PdfFormXObject::identity")]
    pub matrix: PdfMatrix,
    #[value(default)]
    pub content: Vec<PdfOp>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<PdfTransparencyGroup>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub optional_content: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub struct_parent: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfFormXObject {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn identity() -> PdfMatrix {
        PDF_IDENTITY_MATRIX
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(id: impl Into<String>, bbox: PdfRect, content: Vec<PdfOp>) -> Self {
        Self { id: id.into(), bbox, matrix: PDF_IDENTITY_MATRIX, content, group: None, optional_content: None, struct_parent: None, extra: Vec::new() }
    }
}
//#endregion 🔖️XObjects

//#region 🔖️Navigation
/// 🎯 How a destination positions the page (§12.3.2.2, Table 151).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfDestinationFit {
    Xyz { left: Option<f64>, top: Option<f64>, zoom: Option<f64> },
    Fit,
    FitHorizontal { top: Option<f64> },
    FitVertical { left: Option<f64> },
    FitRectangle { rect: PdfRect },
    FitBoundingBox,
    FitBoundingBoxHorizontal { top: Option<f64> },
    FitBoundingBoxVertical { left: Option<f64> },
}

/// 🎯 A destination (§12.3.2): a page of this document by index, a page of a remote document by
/// number, or a named destination.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfDestination {
    Page { page: u32, fit: PdfDestinationFit },
    RemotePage { page: u32, fit: PdfDestinationFit },
    Named { name: String },
}

/// 📎 A file specification (§7.11.3): a path or an embedded file by id.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfFileSpecification {
    Path { path: String },
    Embedded { file: String },
}

/// 🎬 An action (§12.6.4). `next` chains follow-up actions.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfActionKind {
    GoTo { destination: PdfDestination },
    GoToRemote { file: PdfFileSpecification, destination: PdfDestination, new_window: Option<bool> },
    GoToEmbedded { destination: PdfDestination, new_window: Option<bool> },
    Launch { file: PdfFileSpecification, new_window: Option<bool> },
    Thread { file: Option<PdfFileSpecification>, thread: u32 },
    Uri { uri: String, is_map: bool },
    Sound { sound: String, volume: Option<f64>, synchronous: bool, repeat: bool, mix: bool },
    Movie { annotation: Option<String>, operation: Option<String> },
    Hide { annotations: Vec<String>, hide: bool },
    Named { name: String },
    SubmitForm { url: String, fields: Vec<String>, flags: u32 },
    ResetForm { fields: Vec<String>, flags: u32 },
    ImportData { file: PdfFileSpecification },
    JavaScript { script: String },
    SetOptionalContentState { states: Vec<PdfDictEntry>, preserve_radio_buttons: bool },
    Rendition { entries: Vec<PdfDictEntry> },
    Transition { entries: Vec<PdfDictEntry> },
    GoTo3dView { entries: Vec<PdfDictEntry> },
    Unknown { subtype: String, entries: Vec<PdfDictEntry> },
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfAction {
    pub kind: PdfActionKind,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub next: Vec<PdfAction>,
}

impl PdfAction {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn uri(uri: impl Into<String>) -> Self {
        Self { kind: PdfActionKind::Uri { uri: uri.into(), is_map: false }, next: Vec::new() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn go_to(page: u32) -> Self {
        Self { kind: PdfActionKind::GoTo { destination: PdfDestination::Page { page, fit: PdfDestinationFit::Fit } }, next: Vec::new() }
    }
}

/// 📑 One outline (bookmark) item (§12.3.3) with its nested children.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfOutlineItem {
    pub title: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<PdfDestination>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<PdfAction>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<[f64; 3]>,
    #[value(default)]
    pub italic: bool,
    #[value(default)]
    pub bold: bool,
    #[value(default)]
    pub open: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<PdfOutlineItem>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfOutlineItem {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_page(title: impl Into<String>, page: u32) -> Self {
        Self { title: title.into(), destination: Some(PdfDestination::Page { page, fit: PdfDestinationFit::Fit }), action: None, color: None, italic: false, bold: false, open: false, children: Vec::new(), extra: Vec::new() }
    }
}

/// 📛 One entry of the document's `/Dests` name tree (§12.3.2.3).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfNamedDestination {
    pub name: String,
    pub destination: PdfDestination,
}

/// 🔢 Page label numbering style (§12.4.2, Table 159).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PdfPageLabelStyle {
    Decimal,
    RomanUpper,
    RomanLower,
    LettersUpper,
    LettersLower,
}

/// 🔢 One page-label range starting at page `start_index`.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageLabelRange {
    pub start_index: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<PdfPageLabelStyle>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[value(default = "PdfPageLabelRange::one")]
    pub start: u32,
}

impl PdfPageLabelRange {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn one() -> u32 {
        1
    }
}
//#endregion 🔖️Navigation

//#region 🔖️Annotations
/// 🖼️ One appearance sub-dictionary entry: the form XObject `form` shown in appearance `state`.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfAppearanceState {
    pub state: String,
    pub form: String,
}

/// 🖼️ The appearance of one state: a single form XObject id, or a form per appearance state.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfAppearanceEntry {
    Single { form: String },
    States { states: Vec<PdfAppearanceState> },
}

/// 🖼️ An appearance dictionary (§12.5.5).
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfAppearance {
    pub normal: PdfAppearanceEntry,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rollover: Option<PdfAppearanceEntry>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub down: Option<PdfAppearanceEntry>,
}

/// 🟦 An annotation border (§12.5.4 `/Border` or the `/BS` border style dictionary).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfBorderStyle {
    pub width: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dash: Option<Vec<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radii: Option<[f64; 2]>,
}

/// 💬 Fields shared by markup annotations (§12.5.6.2, Table 170).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfMarkupAnnotation {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub popup: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rich_contents: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<PdfDate>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reply_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
}

/// 🏷️ Subtype-specific annotation data (§12.5.6, Table 169 — every subtype).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfAnnotationKind {
    Text { open: bool, icon: Option<String>, state: Option<String>, state_model: Option<String> },
    Link { action: Option<PdfAction>, destination: Option<PdfDestination>, highlight: Option<String>, quad_points: Vec<f64> },
    FreeText { default_appearance: String, quadding: u32, callout: Option<Vec<f64>>, line_ending: Option<String>, rich_text: Option<String> },
    Line { points: [f64; 4], line_endings: Option<[String; 2]>, interior_color: Option<Vec<f64>>, leader_length: Option<f64>, caption: bool },
    Square { interior_color: Option<Vec<f64>>, rect_differences: Option<PdfRect> },
    Circle { interior_color: Option<Vec<f64>>, rect_differences: Option<PdfRect> },
    Polygon { vertices: Vec<f64>, interior_color: Option<Vec<f64>> },
    PolyLine { vertices: Vec<f64>, line_endings: Option<[String; 2]>, interior_color: Option<Vec<f64>> },
    Highlight { quad_points: Vec<f64> },
    Underline { quad_points: Vec<f64> },
    Squiggly { quad_points: Vec<f64> },
    StrikeOut { quad_points: Vec<f64> },
    Stamp { icon: Option<String> },
    Caret { rect_differences: Option<PdfRect>, symbol: Option<String> },
    Ink { paths: Vec<Vec<f64>> },
    Popup { parent: Option<usize>, open: bool },
    FileAttachment { file: PdfFileSpecification, icon: Option<String> },
    Sound { sound: Vec<PdfDictEntry>, icon: Option<String> },
    Movie { title: Option<String>, movie: Vec<PdfDictEntry>, activation: Option<Vec<PdfDictEntry>> },
    Widget { field: Option<String>, highlight: Option<String>, characteristics: Vec<PdfDictEntry>, action: Option<PdfAction>, additional_actions: Vec<PdfDictEntry> },
    Screen { title: Option<String>, characteristics: Vec<PdfDictEntry>, action: Option<PdfAction>, additional_actions: Vec<PdfDictEntry> },
    PrinterMark { mark_style: Option<String>, colorants: Vec<PdfDictEntry> },
    TrapNet { entries: Vec<PdfDictEntry> },
    Watermark { fixed_print: Option<Vec<PdfDictEntry>> },
    ThreeD { entries: Vec<PdfDictEntry> },
    Redact { quad_points: Vec<f64>, interior_color: Option<Vec<f64>>, overlay_text: Option<String>, repeat: bool, default_appearance: Option<String>, quadding: u32 },
    Unknown { subtype: String, entries: Vec<PdfDictEntry> },
}

/// 📌 An annotation (§12.5.2, Table 164) on a page. `contents`/`name`/`modified`/`flags`/
/// `border`/`color`/`appearance` are the common entries; `markup` the markup-annotation ones.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfAnnotation {
    pub rect: PdfRect,
    pub kind: PdfAnnotationKind,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[value(default)]
    pub flags: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub border: Option<PdfBorderStyle>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub color: Vec<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub appearance: Option<PdfAppearance>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub appearance_state: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub markup: Option<PdfMarkupAnnotation>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub optional_content: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub struct_parent: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfAnnotation {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(rect: PdfRect, kind: PdfAnnotationKind) -> Self {
        Self { rect, kind, contents: None, name: None, modified: None, flags: 4, border: None, color: Vec::new(), appearance: None, appearance_state: None, markup: None, optional_content: None, struct_parent: None, extra: Vec::new() }
    }
    /// 🔗 A URI link over `rect` with no visible border.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn link(rect: PdfRect, uri: impl Into<String>) -> Self {
        let mut annotation = Self::new(rect, PdfAnnotationKind::Link { action: Some(PdfAction::uri(uri)), destination: None, highlight: None, quad_points: Vec::new() });
        annotation.border = Some(PdfBorderStyle { width: 0.0, style: None, dash: None, radii: None });
        annotation
    }
}
//#endregion 🔖️Annotations

//#region 🔖️Forms
/// 📝 Interactive form field data by field type (§12.7.4).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfFormFieldKind {
    Button { value: Option<String>, default_value: Option<String>, options: Vec<String> },
    Text { value: Option<String>, default_value: Option<String>, max_length: Option<u32>, rich_value: Option<String> },
    Choice { values: Vec<String>, default_values: Vec<String>, options: Vec<(String, String)>, top_index: Option<u32> },
    Signature { value: Option<Vec<PdfDictEntry>> },
    Container,
}

/// 📝 One field of the interactive form tree (§12.7.3). `widgets` are `(page index, annotation
/// index)` pairs of the Widget annotations presenting this field.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfFormField {
    pub name: String,
    pub kind: PdfFormFieldKind,
    #[value(default)]
    pub flags: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alternate_name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mapping_name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_appearance: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub quadding: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub widgets: Vec<[u32; 2]>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<PdfFormField>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_actions: Vec<PdfDictEntry>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

/// 📝 The document's interactive form dictionary (§12.7.2).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfAcroForm {
    #[value(default)]
    pub fields: Vec<PdfFormField>,
    #[value(default)]
    pub need_appearances: bool,
    #[value(default)]
    pub signature_flags: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_appearance: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub quadding: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub default_fonts: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}
//#endregion 🔖️Forms

//#region 🔖️OptionalContent
/// 👁️ An optional content group (§8.11.2).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfOptionalContentGroup {
    pub id: String,
    pub name: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub intent: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub usage: Vec<PdfDictEntry>,
}

/// 👁️ The optional content properties dictionary (§8.11.4): groups plus the default configuration.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfOptionalContent {
    #[value(default)]
    pub groups: Vec<PdfOptionalContentGroup>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default)]
    pub base_state_off: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub on: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub off: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub order: Vec<PdfObject>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}
//#endregion 🔖️OptionalContent

//#region 🔖️Document
/// 📅 A PDF date (§7.9.4) with an optional UTC offset in minutes.
#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfDate {
    pub year: i32,
    #[value(default = "PdfDate::one")]
    pub month: u32,
    #[value(default = "PdfDate::one")]
    pub day: u32,
    #[value(default)]
    pub hour: u32,
    #[value(default)]
    pub minute: u32,
    #[value(default)]
    pub second: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset_minutes: Option<i32>,
}

impl PdfDate {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn one() -> u32 {
        1
    }
    /// 📅 Parses `D:YYYYMMDDHHmmSSOHH'mm'` (every field after the year optional).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn parse(text: &str) -> Option<Self> {
        let body = text.strip_prefix("D:").unwrap_or(text);
        let digits = |from: usize, len: usize| body.get(from..from + len).filter(|s| s.len() == len && s.bytes().all(|b| b.is_ascii_digit())).and_then(|s| s.parse::<u32>().ok());
        let year = digits(0, 4)? as i32;
        let month = digits(4, 2).unwrap_or(1).clamp(1, 12);
        let day = digits(6, 2).unwrap_or(1).clamp(1, 31);
        let hour = digits(8, 2).unwrap_or(0).min(23);
        let minute = digits(10, 2).unwrap_or(0).min(59);
        let second = digits(12, 2).unwrap_or(0).min(59);
        let marker = body.bytes().position(|b| !b.is_ascii_digit()).unwrap_or(body.len()).min(14);
        let offset_minutes = match body.as_bytes().get(marker) {
            Some(b'Z') => Some(0),
            Some(sign @ (b'+' | b'-')) => {
                let hours = digits(marker + 1, 2).unwrap_or(0) as i32;
                let minutes = digits(marker + 4, 2).unwrap_or(0) as i32;
                let total = hours * 60 + minutes;
                Some(if *sign == b'-' { -total } else { total })
            }
            _ => None,
        };
        Some(Self { year, month, day, hour, minute, second, offset_minutes })
    }
}

impl fmt::Display for PdfDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "D:{:04}{:02}{:02}{:02}{:02}{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)?;
        match self.offset_minutes {
            Some(0) => formatter.write_str("Z"),
            Some(offset) => write!(formatter, "{}{:02}'{:02}'", if offset < 0 { '-' } else { '+' }, offset.abs() / 60, offset.abs() % 60),
            None => Ok(()),
        }
    }
}

/// 📇️ Document `/Info` dictionary (§14.3.3, Table 317).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfInfo {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<PdfDate>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<PdfDate>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trapped: Option<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl PdfInfo {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self == &PdfInfo::default()
    }
}

/// 📎 An embedded file (§7.11.4) reachable through the `/EmbeddedFiles` name tree or a file
/// attachment annotation, keyed by `id`.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfEmbeddedFile {
    pub id: String,
    pub file_name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    pub data: Vec<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<PdfDate>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<PdfDate>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    #[value(default)]
    pub listed: bool,
}

/// 🏳️ An output intent (§14.11.5) — what PDF/A and PDF/X conformance is declared against.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfOutputIntent {
    pub subtype: String,
    pub condition_identifier: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<Vec<u8>>,
}

/// 📖 Page layout to use when the document is opened (§12.2, Table 28).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PdfPageLayout {
    SinglePage,
    OneColumn,
    TwoColumnLeft,
    TwoColumnRight,
    TwoPageLeft,
    TwoPageRight,
}

/// 📖 How the document is displayed when opened (§12.2, Table 28).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PdfPageMode {
    UseNone,
    UseOutlines,
    UseThumbs,
    FullScreen,
    UseOc,
    UseAttachments,
}

/// 🖥️ Viewer preferences (§12.2, Table 150).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfViewerPreferences {
    #[value(default)]
    pub hide_toolbar: bool,
    #[value(default)]
    pub hide_menubar: bool,
    #[value(default)]
    pub hide_window_ui: bool,
    #[value(default)]
    pub fit_window: bool,
    #[value(default)]
    pub center_window: bool,
    #[value(default)]
    pub display_doc_title: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub non_full_screen_page_mode: Option<PdfPageMode>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub view_area: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub view_clip: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub print_area: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub print_clip: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub print_scaling: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub duplex: Option<String>,
    #[value(default)]
    pub pick_tray_by_pdf_size: bool,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub print_page_range: Vec<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub num_copies: Option<u32>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

/// 🚪 What happens when the document opens (§7.7.2 `/OpenAction`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfOpenAction {
    Destination { destination: PdfDestination },
    Action { action: PdfAction },
}

/// 🔐 The standard security handler's algorithm (§7.6.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PdfEncryptionAlgorithm {
    Rc4_40,
    Rc4_128,
    Aes128,
    Aes256,
}

/// 🔐 Standard security handler parameters (§7.6.3, Table 21). Passwords are in-memory only:
/// the persistent form of this snapshot is the encrypted file itself, whose `/O`/`/U` entries
/// are derived from them on write. A decoded document records the algorithm, permissions and
/// the user password it was opened with (`""` for the empty user password).
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfEncryption {
    pub algorithm: PdfEncryptionAlgorithm,
    #[value(default = "PdfEncryption::all_permissions")]
    pub permissions: i32,
    #[value(default)]
    pub user_password: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub owner_password: Option<String>,
    #[value(default = "PdfEncryption::yes")]
    pub encrypt_metadata: bool,
}

impl PdfEncryption {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn all_permissions() -> i32 {
        -1
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn yes() -> bool {
        true
    }
}

/// 🏷️ `/MarkInfo` (§14.7.1).
#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfMarkInfo {
    #[value(default)]
    pub marked: bool,
    #[value(default)]
    pub user_properties: bool,
    #[value(default)]
    pub suspects: bool,
}
//#endregion 🔖️Document

//#region 🔖️PageModel
/// 📄️ One page (§7.7.3.3, Table 30) -- inherited `/MediaBox`/`/CropBox`/`/Rotate` already
/// applied (§7.7.3.4). `content` is the concatenation of the page's content streams as typed
/// operators whose resource names are the document-level ids ([`PdfSnapshot::fonts`], images,
/// forms, …); the page's own `/Resources` dictionary is regenerated from them on write.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPage {
    pub media_box: PdfRect,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub crop_box: Option<PdfRect>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bleed_box: Option<PdfRect>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trim_box: Option<PdfRect>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub art_box: Option<PdfRect>,
    #[value(default)]
    pub rotate: i32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_unit: Option<f64>,
    #[value(default)]
    pub content: Vec<PdfOp>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<PdfAnnotation>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<PdfTransparencyGroup>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub struct_parents: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<Vec<PdfDictEntry>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_actions: Vec<PdfDictEntry>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<PdfDictEntry>,
}

impl Default for PdfPage {
    fn default() -> Self {
        Self::new(612.0, 792.0)
    }
}

impl PdfPage {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            media_box: [0.0, 0.0, width, height],
            crop_box: None,
            bleed_box: None,
            trim_box: None,
            art_box: None,
            rotate: 0,
            user_unit: None,
            content: Vec::new(),
            annotations: Vec::new(),
            group: None,
            thumbnail: None,
            struct_parents: None,
            transition: None,
            duration: None,
            metadata: None,
            additional_actions: Vec::new(),
            extra: Vec::new(),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn width(&self) -> f64 {
        self.media_box[2] - self.media_box[0]
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn height(&self) -> f64 {
        self.media_box[3] - self.media_box[1]
    }
    /// 🔤️ The Unicode text this page shows, in painting order — every `Text` operand of every
    /// text-showing operator, lines separated where the stream starts a new line.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn text(&self) -> String {
        fn push(out: &mut String, text: &PdfTextString) {
            if let PdfTextString::Text { text } = text {
                out.push_str(text);
            }
        }
        fn newline(out: &mut String) {
            if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
        }
        let mut out = String::new();
        for op in &self.content {
            match op {
                PdfOp::ShowText { text } => push(&mut out, text),
                PdfOp::NextLineShowText { text } | PdfOp::NextLineShowTextSpaced { text, .. } => {
                    newline(&mut out);
                    push(&mut out, text);
                }
                PdfOp::ShowTextArray { items } => {
                    for item in items {
                        if let PdfTextArrayItem::Text { text } = item {
                            out.push_str(text);
                        }
                    }
                }
                PdfOp::NextLine | PdfOp::MoveText { .. } | PdfOp::MoveTextSetLeading { .. } | PdfOp::SetTextMatrix { .. } => newline(&mut out),
                _ => {}
            }
        }
        while out.ends_with('\n') {
            out.pop();
        }
        out
    }
}
//#endregion 🔖️PageModel

//#region 🔖️Snapshot
/// 🧬️ `stdio.pdf` (1.7) persistent snapshot: the typed document lanes plus the retained COS
/// carrier (`objects`/`trailer`, the full logical indirect-object graph as read — lossless
/// retention per D2 ground rules, what the conformance subsets inspect).
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.stdio.pdf.1.7")]
pub struct PdfSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub declared_version: String,
    #[state(artifact)]
    pub pages: Vec<PdfPage>,
    #[state(artifact)]
    pub fonts: Vec<PdfFont>,
    #[state(artifact)]
    pub images: Vec<PdfImage>,
    #[state(artifact)]
    pub forms: Vec<PdfFormXObject>,
    #[state(artifact)]
    pub ext_g_states: Vec<PdfExtGState>,
    #[state(artifact)]
    pub shadings: Vec<PdfShading>,
    #[state(artifact)]
    pub patterns: Vec<PdfPattern>,
    #[state(artifact)]
    pub color_spaces: Vec<PdfNamedColorSpace>,
    #[state(artifact)]
    pub properties: Vec<PdfNamedProperties>,
    #[state(artifact)]
    pub outlines: Vec<PdfOutlineItem>,
    #[state(artifact)]
    pub named_destinations: Vec<PdfNamedDestination>,
    #[state(artifact)]
    pub page_labels: Vec<PdfPageLabelRange>,
    #[state(artifact)]
    pub embedded_files: Vec<PdfEmbeddedFile>,
    #[state(artifact)]
    pub output_intents: Vec<PdfOutputIntent>,
    #[state(artifact)]
    pub acro_form: Option<PdfAcroForm>,
    #[state(artifact)]
    pub optional_content: Option<PdfOptionalContent>,
    #[state(artifact)]
    pub page_layout: Option<PdfPageLayout>,
    #[state(artifact)]
    pub page_mode: Option<PdfPageMode>,
    #[state(artifact)]
    pub viewer_preferences: Option<PdfViewerPreferences>,
    #[state(artifact)]
    pub open_action: Option<PdfOpenAction>,
    #[state(artifact)]
    pub language: Option<String>,
    #[state(artifact)]
    pub mark_info: Option<PdfMarkInfo>,
    #[state(artifact)]
    pub metadata: Option<String>,
    #[state(artifact)]
    pub document_id: Option<[Vec<u8>; 2]>,
    #[state(artifact)]
    pub encryption: Option<PdfEncryption>,
    #[state(artifact)]
    pub info: PdfInfo,
    #[state(artifact)]
    pub catalog_extra: Vec<PdfDictEntry>,
    #[state(artifact)]
    pub objects: Vec<PdfIndirectObject>,
    #[state(artifact)]
    pub trailer: Vec<PdfDictEntry>,
}

impl Default for PdfSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
            declared_version: "1.7".into(),
            pages: Vec::new(),
            fonts: Vec::new(),
            images: Vec::new(),
            forms: Vec::new(),
            ext_g_states: Vec::new(),
            shadings: Vec::new(),
            patterns: Vec::new(),
            color_spaces: Vec::new(),
            properties: Vec::new(),
            outlines: Vec::new(),
            named_destinations: Vec::new(),
            page_labels: Vec::new(),
            embedded_files: Vec::new(),
            output_intents: Vec::new(),
            acro_form: None,
            optional_content: None,
            page_layout: None,
            page_mode: None,
            viewer_preferences: None,
            open_action: None,
            language: None,
            mark_info: None,
            metadata: None,
            document_id: None,
            encryption: None,
            info: PdfInfo::default(),
            catalog_extra: Vec::new(),
            objects: Vec::new(),
            trailer: Vec::new(),
        }
    }
}

impl PdfSnapshot {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn font(&self, id: &str) -> Option<&PdfFont> {
        self.fonts.iter().find(|font| font.id == id)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn image(&self, id: &str) -> Option<&PdfImage> {
        self.images.iter().find(|image| image.id == id)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn form(&self, id: &str) -> Option<&PdfFormXObject> {
        self.forms.iter().find(|form| form.id == id)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn ext_g_state(&self, id: &str) -> Option<&PdfExtGState> {
        self.ext_g_states.iter().find(|state| state.id == id)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn shading(&self, id: &str) -> Option<&PdfShading> {
        self.shadings.iter().find(|shading| shading.id == id)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn pattern(&self, id: &str) -> Option<&PdfPattern> {
        self.patterns.iter().find(|pattern| pattern.id == id)
    }
    /// 🆔 Every id the document-level collections currently use (fonts, XObjects, graphics
    /// states, shadings, patterns, embedded files, optional content groups).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn resource_ids(&self) -> std::collections::BTreeSet<String> {
        let mut ids = std::collections::BTreeSet::new();
        ids.extend(self.fonts.iter().map(|font| font.id.clone()));
        ids.extend(self.images.iter().map(|image| image.id.clone()));
        ids.extend(self.forms.iter().map(|form| form.id.clone()));
        ids.extend(self.ext_g_states.iter().map(|state| state.id.clone()));
        ids.extend(self.shadings.iter().map(|shading| shading.id.clone()));
        ids.extend(self.patterns.iter().map(|pattern| pattern.id.clone()));
        ids.extend(self.color_spaces.iter().map(|space| space.name.clone()));
        ids.extend(self.properties.iter().map(|properties| properties.name.clone()));
        ids.extend(self.embedded_files.iter().map(|file| file.id.clone()));
        if let Some(optional_content) = &self.optional_content {
            ids.extend(optional_content.groups.iter().map(|group| group.id.clone()));
        }
        ids
    }
    /// 🆔 A fresh id of the form `{prefix}{n}` not yet used by any collection.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fresh_id(&self, prefix: &str) -> String {
        let ids = self.resource_ids();
        (1..).map(|index| format!("{prefix}{index}")).find(|candidate| !ids.contains(candidate)).expect("an unbounded id sequence has a free member")
    }
}

/// 🌱️ First-party value encoding for the public PDF snapshot, preserving the camelCase object
/// shape `value_derive::ToValue` would emit; `schema` is the only required key on decode.
impl pack::value::ToValue for PdfSnapshot {
    fn to_value(&self) -> pack::value::DslValue {
        pack::value::DslValue::object([
            ("schema".to_string(), self.schema.to_value()),
            ("declaredVersion".to_string(), self.declared_version.to_value()),
            ("pages".to_string(), self.pages.to_value()),
            ("fonts".to_string(), self.fonts.to_value()),
            ("images".to_string(), self.images.to_value()),
            ("forms".to_string(), self.forms.to_value()),
            ("extGStates".to_string(), self.ext_g_states.to_value()),
            ("shadings".to_string(), self.shadings.to_value()),
            ("patterns".to_string(), self.patterns.to_value()),
            ("colorSpaces".to_string(), self.color_spaces.to_value()),
            ("properties".to_string(), self.properties.to_value()),
            ("outlines".to_string(), self.outlines.to_value()),
            ("namedDestinations".to_string(), self.named_destinations.to_value()),
            ("pageLabels".to_string(), self.page_labels.to_value()),
            ("embeddedFiles".to_string(), self.embedded_files.to_value()),
            ("outputIntents".to_string(), self.output_intents.to_value()),
            ("acroForm".to_string(), self.acro_form.to_value()),
            ("optionalContent".to_string(), self.optional_content.to_value()),
            ("pageLayout".to_string(), self.page_layout.to_value()),
            ("pageMode".to_string(), self.page_mode.to_value()),
            ("viewerPreferences".to_string(), self.viewer_preferences.to_value()),
            ("openAction".to_string(), self.open_action.to_value()),
            ("language".to_string(), self.language.to_value()),
            ("markInfo".to_string(), self.mark_info.to_value()),
            ("metadata".to_string(), self.metadata.to_value()),
            ("documentId".to_string(), self.document_id.to_value()),
            ("encryption".to_string(), self.encryption.to_value()),
            ("info".to_string(), self.info.to_value()),
            ("catalogExtra".to_string(), self.catalog_extra.to_value()),
            ("objects".to_string(), self.objects.to_value()),
            ("trailer".to_string(), self.trailer.to_value()),
        ])
    }
}

/// 🔀️ First-party value decoding for the public PDF snapshot. `schema` remains required while
/// every other field falls back to its default.
impl pack::value::FromValue for PdfSnapshot {
    fn from_value(value: pack::value::DslValue) -> Result<Self, pack::value::ValueError> {
        use pack::value::FromValue;
        let entries = value.into_object()?;
        let field = |key: &str| entries.iter().find(|(candidate, _)| candidate == key).map(|(_, value)| value.clone());
        fn decode_or_default<T: FromValue + Default>(value: Option<pack::value::DslValue>, key: &str) -> Result<T, pack::value::ValueError> {
            match value {
                Some(pack::value::DslValue::Null) | None => Ok(T::default()),
                Some(value) => T::from_value(value).map_err(|error| error.under(key)),
            }
        }
        let schema = field("schema").ok_or_else(|| pack::value::ValueError::new("missing field `schema`"))?;
        Ok(Self {
            schema: String::from_value(schema).map_err(|error| error.under("schema"))?,
            declared_version: decode_or_default(field("declaredVersion"), "declaredVersion")?,
            pages: decode_or_default(field("pages"), "pages")?,
            fonts: decode_or_default(field("fonts"), "fonts")?,
            images: decode_or_default(field("images"), "images")?,
            forms: decode_or_default(field("forms"), "forms")?,
            ext_g_states: decode_or_default(field("extGStates"), "extGStates")?,
            shadings: decode_or_default(field("shadings"), "shadings")?,
            patterns: decode_or_default(field("patterns"), "patterns")?,
            color_spaces: decode_or_default(field("colorSpaces"), "colorSpaces")?,
            properties: decode_or_default(field("properties"), "properties")?,
            outlines: decode_or_default(field("outlines"), "outlines")?,
            named_destinations: decode_or_default(field("namedDestinations"), "namedDestinations")?,
            page_labels: decode_or_default(field("pageLabels"), "pageLabels")?,
            embedded_files: decode_or_default(field("embeddedFiles"), "embeddedFiles")?,
            output_intents: decode_or_default(field("outputIntents"), "outputIntents")?,
            acro_form: decode_or_default(field("acroForm"), "acroForm")?,
            optional_content: decode_or_default(field("optionalContent"), "optionalContent")?,
            page_layout: decode_or_default(field("pageLayout"), "pageLayout")?,
            page_mode: decode_or_default(field("pageMode"), "pageMode")?,
            viewer_preferences: decode_or_default(field("viewerPreferences"), "viewerPreferences")?,
            open_action: decode_or_default(field("openAction"), "openAction")?,
            language: decode_or_default(field("language"), "language")?,
            mark_info: decode_or_default(field("markInfo"), "markInfo")?,
            metadata: decode_or_default(field("metadata"), "metadata")?,
            document_id: decode_or_default(field("documentId"), "documentId")?,
            encryption: decode_or_default(field("encryption"), "encryption")?,
            info: decode_or_default(field("info"), "info")?,
            catalog_extra: decode_or_default(field("catalogExtra"), "catalogExtra")?,
            objects: decode_or_default(field("objects"), "objects")?,
            trailer: decode_or_default(field("trailer"), "trailer")?,
        })
    }
}

#[cfg(test)]
#[path = "🦀️tests-snapshot-value.rs"]
mod pdf_snapshot_value_tests;

impl store::ArtifactDsl for PdfSnapshot {
    const EXTENSION: &'static str = "pdf";
    fn envelope_id() -> &'static str {
        STDIO_PDF17_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if !hex.len().is_multiple_of(2) {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?);
        }
        crate::standards::v1_7::subsets::base::io::decode_pdf(&bytes).map_err(|e| store::TextError::new(format!("{e:?}"), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::standards::v1_7::subsets::base::io::encode_pdf(self).expect("PDF snapshot must encode before DSL transport");
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PdfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::standards::v1_7::subsets::base::io::encode_pdf(self).map_err(|e| store::PackError::Schema(format!("{e:?}")))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::standards::v1_7::subsets::base::io::decode_pdf(&inner).map_err(|e| store::PackError::Schema(format!("{e:?}")))
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SnapshotFixtures
/// 🦑 Pure snapshot constructors, no codec/IO concern.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_pdf_snapshot() -> PdfSnapshot {
    PdfSnapshot::default()
}

/// 📄️ The demo `stdio.pdf.1.7` document -- the single source of truth for `🏅️standards/7️⃣1.7/
/// 📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law`).
///
/// Deliberately the real `decode_pdf(encode_pdf(seed))` FIXED POINT: the typed lanes survive the
/// round trip by the `lift(lower(t)) == t` law, and `objects`/`trailer` are whatever the fresh
/// write produced, read back — a hand-built snapshot with empty `objects` would not equal its
/// own decoded print.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_pdf17_snapshot() -> PdfSnapshot {
    let seed = crate::standards::v1_7::subsets::base::io::text_document(&[(200.0, 300.0, "Semio")]);
    let bytes = crate::standards::v1_7::subsets::base::io::encode_pdf(&seed).expect("encode_pdf(seed) must succeed");
    crate::standards::v1_7::subsets::base::io::decode_pdf(&bytes).expect("decode_pdf(encode_pdf(seed)) must succeed")
}
//#endregion 🔖️SnapshotFixtures
