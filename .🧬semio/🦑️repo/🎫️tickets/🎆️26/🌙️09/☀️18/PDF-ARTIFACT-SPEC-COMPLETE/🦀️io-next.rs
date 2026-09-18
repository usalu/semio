//! 🚪️ IO stdio.pdf (1.7/🧱️base) — the codec entry points over the engine modules: `decode_pdf`
//! (sniff → cross-reference → decrypt → retained graph → typed lanes), `encode_pdf` (typed lanes
//! → COS graph, reconciled onto a retained graph when one is carried → bytes), the streaming
//! [`DocumentStream`] a guest can drive one page per step, and the typed builders every consumer
//! starts from ([`text_document`], [`PdfTextLayout`]). Reads PDF 1.0–2.0 leniently
//! (`declared_version` records the header verbatim).
//!
//! Laws (proven in `🧪️tests`): `lift(lower(t)) == t` on the typed lanes; `decode(encode(s)) == s`
//! after one write (a retained graph that no longer spells the typed lanes is regenerated once,
//! after which the file is its own fixed point); every retained object the typed lanes do not
//! own survives a write untouched, renumbered only where an id moved.

use crate::standards::v1_7::subsets::base::modules::content::content_references;
use crate::standards::v1_7::subsets::base::modules::encryption::open_standard_security;
use crate::standards::v1_7::subsets::base::modules::fonts::{standard_font, FontCodec};
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::modules::lift::{lift_document_with, Category};
use crate::standards::v1_7::subsets::base::modules::lower::{lower_acro_form_standalone, lower_document, lower_document_headless, lower_page_standalone, LowerOptions, LoweredDocument};
use crate::standards::v1_7::subsets::base::modules::writer::{serialize_document, DocumentTrailer, PdfWriter, WriteOptions};
use crate::standards::v1_7::subsets::base::modules::xref::{build_xref, startxref_offset, GraphSource, ObjectSource, Resolver};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use std::collections::{HashMap, HashSet};

pub use crate::standards::v1_7::subsets::base::modules::lexer::PdfEngineError as EngineError;

//#region 🔖️Sniff
/// 🔍️ Real magic + version probe: `%PDF-` header, version digits parsed and reported.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_pdf(bytes: &[u8]) -> Option<String> {
    let start = bytes.windows(5).take(1024).position(|window| window == b"%PDF-")?;
    let rest = &bytes[start + 5..];
    let end = rest.iter().take(8).position(|&b| b == b'\n' || b == b'\r' || b == b' ' || b == b'\t').unwrap_or(rest.len().min(8));
    let version = String::from_utf8_lossy(&rest[..end]).trim().to_string();
    (!version.is_empty() && version.chars().all(|c| c.is_ascii_digit() || c == '.')).then_some(version)
}
//#endregion 🔖️Sniff

//#region 🔖️Decode
/// 📥️ Decodes a file with the empty user password.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_pdf(data: &[u8]) -> PResult<PdfSnapshot> {
    decode_pdf_with_password(data, "")
}

/// 📥️ Decodes a file, opening the standard security handler with `password` when it is
/// encrypted (`Unsupported` when the password does not open it or the handler is not the
/// standard one — never garbage).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_pdf_with_password(data: &[u8], password: &str) -> PResult<PdfSnapshot> {
    let declared_version = sniff_pdf(data).ok_or(PdfEngineError::NotPdf)?;
    let start = data.windows(5).position(|window| window == b"%PDF-").unwrap_or(0);
    let data = &data[start..];
    let xref = build_xref(data, startxref_offset(data));
    let mut resolver = Resolver::new(data, xref.entries.clone());
    let mut encryption = None;
    let mut encrypt_object: Option<u32> = None;
    if let Some(encrypt) = dict_get(&xref.trailer, "Encrypt") {
        encrypt_object = encrypt.as_ref().map(|reference| reference.num);
        let dictionary = match encrypt {
            PdfObject::Ref(reference) => resolver.resolve(reference.num).ok_or_else(|| PdfEngineError::Malformed("/Encrypt reference does not resolve".into()))?,
            other => other.clone(),
        };
        let dictionary = dictionary.as_dict().ok_or_else(|| PdfEngineError::Malformed("/Encrypt is not a dictionary".into()))?.to_vec();
        let document_id = dict_get(&xref.trailer, "ID").and_then(PdfObject::as_array).and_then(|items| items.first()).and_then(PdfObject::as_str_bytes).unwrap_or(&[]).to_vec();
        let (decryptor, parameters) = open_standard_security(&dictionary, &document_id, password)?;
        resolver.set_decryptor(Some(decryptor));
        encryption = Some(parameters);
    }
    let mut objects = resolver.resolve_all()?;
    if let Some(number) = encrypt_object {
        objects.retain(|object| object.id.num != number);
    }
    let trailer: Vec<PdfDictEntry> = xref.trailer.iter().filter(|entry| matches!(entry.key.as_str(), "Root" | "Info" | "ID")).cloned().collect();
    if !trailer.iter().any(|entry| entry.key == "Root") {
        return Err(PdfEngineError::Malformed("no /Root in any trailer and no /Catalog object to recover one from".into()));
    }
    let mut source = GraphSource::new(&objects);
    let lifter = lift_document_with(&trailer, &declared_version, &mut source);
    let mut snapshot = lifter.snapshot;
    snapshot.schema = STDIO_PDF17_DOCUMENT_SCHEMA.into();
    snapshot.declared_version = declared_version;
    snapshot.encryption = encryption;
    snapshot.objects = objects;
    snapshot.trailer = trailer;
    Ok(snapshot)
}
//#endregion 🔖️Decode

//#region 🔖️Encode
/// 🎛️ Every knob of a write in one place.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EncodeOptions {
    pub lower: LowerOptions,
    pub write: WriteOptions,
}

impl EncodeOptions {
    /// 📦 The export profile consumers want: subset fonts, compressed streams.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export() -> Self {
        Self { lower: LowerOptions { subset_fonts: true, compress: true }, write: WriteOptions::default() }
    }
}

/// 📤️ Writes the snapshot with default options (no font subsetting, classic cross-reference
/// table, the snapshot's own `encryption`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_pdf(snapshot: &PdfSnapshot) -> PResult<Vec<u8>> {
    encode_pdf_with(snapshot, &EncodeOptions::default())
}

/// 🧭 A source over the retained graph that remembers every object the typed lanes reached.
struct RecordingSource<'a> {
    inner: GraphSource<'a>,
    seen: HashSet<u32>,
}

impl ObjectSource for RecordingSource<'_> {
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
        self.seen.insert(reference.num);
        self.inner.get(reference)
    }
}

/// 🔁 Rewrites references inside a retained object through `map`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rewrite_refs(value: &PdfObject, map: &HashMap<u32, ObjRef>) -> PdfObject {
    match value {
        PdfObject::Ref(reference) => PdfObject::Ref(map.get(&reference.num).copied().unwrap_or(*reference)),
        PdfObject::Array(items) => PdfObject::Array(items.iter().map(|item| rewrite_refs(item, map)).collect()),
        PdfObject::Dict(entries) => PdfObject::Dict(entries.iter().map(|entry| PdfDictEntry { key: entry.key.clone(), value: rewrite_refs(&entry.value, map) }).collect()),
        PdfObject::Stream { dict, data, filters } => PdfObject::Stream { dict: dict.iter().map(|entry| PdfDictEntry { key: entry.key.clone(), value: rewrite_refs(&entry.value, map) }).collect(), data: data.clone(), filters: filters.clone() },
        other => other.clone(),
    }
}

/// 📤️ Writes the snapshot. A retained graph that still spells the typed lanes is written as it
/// stands; otherwise the objects the typed lanes own are regenerated onto it and every other
/// retained object is kept, with references to moved objects rewritten.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_pdf_with(snapshot: &PdfSnapshot, options: &EncodeOptions) -> PResult<Vec<u8>> {
    let version = if snapshot.declared_version.is_empty() { "1.7".to_string() } else { snapshot.declared_version.clone() };
    if !version.bytes().all(|byte| byte.is_ascii_digit() || byte == b'.') {
        return Err(PdfEngineError::Malformed("declared PDF version is not numeric".into()));
    }
    let version = if options.write.xref_stream && version.as_str() < "1.5" { "1.5".to_string() } else { version };
    let mut write = options.write.clone();
    if write.encryption.is_none() {
        write.encryption = snapshot.encryption.clone();
    }
    let retained_root = dict_get(&snapshot.trailer, "Root").and_then(PdfObject::as_ref);
    if let Some(root) = retained_root.filter(|_| !snapshot.objects.is_empty() && options.lower == LowerOptions::default()) {
        let mut recording = RecordingSource { inner: GraphSource::new(&snapshot.objects), seen: HashSet::new() };
        let lifter = lift_document_with(&snapshot.trailer, &snapshot.declared_version, &mut recording);
        let (lifted_snapshot, lifted_ids, lifted_page_refs, lifted_annotation_refs) = (lifter.snapshot, lifter.ids, lifter.page_refs, lifter.annotation_refs);
        let mut retained_view = lifted_snapshot;
        retained_view.schema = snapshot.schema.clone();
        retained_view.declared_version = snapshot.declared_version.clone();
        retained_view.encryption = snapshot.encryption.clone();
        retained_view.objects = snapshot.objects.clone();
        retained_view.trailer = snapshot.trailer.clone();
        if retained_view == *snapshot {
            let info = dict_get(&snapshot.trailer, "Info").and_then(PdfObject::as_ref);
            let trailer = DocumentTrailer { root, info, id: snapshot.document_id.clone(), extra: Vec::new() };
            return Ok(serialize_document(&version, &snapshot.objects, &trailer, &write));
        }
        let owned: HashSet<u32> = recording.seen.iter().copied().chain(std::iter::once(root.num)).chain(dict_get(&snapshot.trailer, "Info").and_then(PdfObject::as_ref).map(|r| r.num)).collect();
        let first_number = snapshot.objects.iter().map(|object| object.id.num).max().unwrap_or(0) + 1;
        let lowered = lower_document(snapshot, first_number, options.lower.clone())?;
        let mut rewrite: HashMap<u32, ObjRef> = HashMap::new();
        for ((category, old_ref), id) in &lifted_ids {
            if let Some(new_ref) = lowered.refs.get(&(*category, id.clone())) {
                rewrite.insert(old_ref.num, *new_ref);
            }
        }
        for (index, old_ref) in lifted_page_refs.iter().enumerate() {
            if let Some(new_ref) = lowered.refs.get(&(Category::Page, index.to_string())) {
                rewrite.insert(old_ref.num, *new_ref);
            }
        }
        for (old_ref, (page, index)) in &lifted_annotation_refs {
            if let Some(new_ref) = lowered.annotation_refs.get(*page as usize).and_then(|refs| refs.get(*index as usize)) {
                rewrite.insert(old_ref.num, *new_ref);
            }
        }
        rewrite.insert(root.num, lowered.root);
        let mut objects: Vec<PdfIndirectObject> = snapshot.objects.iter().filter(|object| !owned.contains(&object.id.num)).map(|object| PdfIndirectObject { id: object.id, value: rewrite_refs(&object.value, &rewrite) }).collect();
        objects.extend(lowered.objects);
        objects.sort_by_key(|object| object.id.num);
        let trailer = DocumentTrailer { root: lowered.root, info: lowered.info, id: snapshot.document_id.clone(), extra: Vec::new() };
        return Ok(serialize_document(&version, &objects, &trailer, &write));
    }
    let LoweredDocument { objects, root, info, .. } = lower_document(snapshot, 1, options.lower.clone())?;
    let trailer = DocumentTrailer { root, info, id: snapshot.document_id.clone(), extra: Vec::new() };
    Ok(serialize_document(&version, &objects, &trailer, &write))
}
//#endregion 🔖️Encode

//#region 🔖️Streaming
/// 🌊 A page-at-a-time document writer for guests with per-step budgets: the document-level
/// lanes are lowered up front, then every `page` call yields that page's bytes, and `finish`
/// closes the file. Pages appended this way reference the snapshot's fonts, images, forms and
/// graphics states by id exactly like [`PdfPage::content`] does.
pub struct DocumentStream {
    writer: PdfWriter,
    lowered: LoweredDocument,
    pages_ref: ObjRef,
    page_refs: Vec<ObjRef>,
    written_pages: usize,
    next_number: u32,
    snapshot: PdfSnapshot,
    options: LowerOptions,
    root_resources: PdfObject,
    annotation_refs: Vec<Vec<ObjRef>>,
}

impl DocumentStream {
    /// 🏁 Lowers the document-level lanes of `snapshot` (its `pages` are ignored — they arrive
    /// through [`DocumentStream::page`]) for `expected_pages` pages and returns the header +
    /// resource bytes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn begin(snapshot: &PdfSnapshot, expected_pages: usize, options: EncodeOptions) -> PResult<(Self, Vec<u8>)> {
        let (lowered, pages_ref, page_refs, root_resources) = lower_document_headless(snapshot, expected_pages, options.lower.clone())?;
        let mut write = options.write.clone();
        if write.encryption.is_none() {
            write.encryption = snapshot.encryption.clone();
        }
        let version = if snapshot.declared_version.is_empty() { "1.7".to_string() } else { snapshot.declared_version.clone() };
        let seed = snapshot.document_id.as_ref().map(|id| id[0].clone()).unwrap_or_default();
        let (mut writer, mut out) = PdfWriter::begin(&version, &write, &seed, &seed);
        for object in lowered.objects.iter().filter(|object| object.id != pages_ref && object.id != lowered.root) {
            out.extend_from_slice(&writer.object(object));
        }
        let next_number = lowered.objects.iter().map(|object| object.id.num).max().unwrap_or(0).max(page_refs.iter().map(|r| r.num).max().unwrap_or(0)) + 1;
        let mut headless = snapshot.clone();
        headless.pages.clear();
        headless.objects.clear();
        headless.trailer.clear();
        Ok((Self { writer, lowered, pages_ref, page_refs, written_pages: 0, next_number, snapshot: headless, options: options.lower, root_resources, annotation_refs: Vec::new() }, out))
    }

    /// 📄 Appends the next page; returns its bytes (page object, content stream, annotations).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn page(&mut self, page: &PdfPage) -> PResult<Vec<u8>> {
        let index = self.written_pages;
        let page_ref = match self.page_refs.get(index) {
            Some(reference) => *reference,
            None => {
                let reference = ObjRef { num: self.next_number, gen: 0 };
                self.next_number += 1;
                self.page_refs.push(reference);
                reference
            }
        };
        let (objects, annotation_refs) = lower_page_standalone(&self.snapshot, page, index, page_ref, self.pages_ref, self.next_number, self.options.clone(), &self.lowered.refs, &self.lowered.widget_parents)?;
        self.annotation_refs.push(annotation_refs);
        let mut out = Vec::new();
        for object in &objects {
            self.next_number = self.next_number.max(object.id.num + 1);
            out.extend_from_slice(&self.writer.object(object));
        }
        self.written_pages += 1;
        Ok(out)
    }

    /// 🏁 Writes the page tree, catalog and cross-reference section; returns the closing bytes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn finish(mut self) -> PResult<Vec<u8>> {
        let mut out = Vec::new();
        let kids: Vec<PdfObject> = self.page_refs.iter().take(self.written_pages).map(|r| PdfObject::Ref(*r)).collect();
        let pages = PdfObject::Dict(vec![PdfDictEntry::new("Type", PdfObject::name("Pages")), PdfDictEntry::new("Kids", PdfObject::Array(kids)), PdfDictEntry::new("Count", PdfObject::Int(self.written_pages as i64)), PdfDictEntry::new("Resources", self.root_resources.clone())]);
        out.extend_from_slice(&self.writer.object(&PdfIndirectObject { id: self.pages_ref, value: pages }));
        let mut acro_form: Option<ObjRef> = None;
        if let Some((objects, reference)) = lower_acro_form_standalone(&self.snapshot, self.next_number, self.options.clone(), &self.lowered.refs, std::mem::take(&mut self.annotation_refs), self.lowered.field_refs.clone())? {
            for object in &objects {
                self.next_number = self.next_number.max(object.id.num + 1);
                out.extend_from_slice(&self.writer.object(object));
            }
            acro_form = Some(reference);
        }
        if let Some(mut catalog) = self.lowered.objects.iter().find(|object| object.id == self.lowered.root).cloned() {
            if let (Some(reference), PdfObject::Dict(entries)) = (acro_form, &mut catalog.value) {
                entries.push(PdfDictEntry::new("AcroForm", PdfObject::Ref(reference)));
            }
            out.extend_from_slice(&self.writer.object(&catalog));
        }
        let trailer = DocumentTrailer { root: self.lowered.root, info: self.lowered.info, id: self.snapshot.document_id.clone(), extra: Vec::new() };
        out.extend_from_slice(&self.writer.finish(&trailer));
        Ok(out)
    }
}
//#endregion 🔖️Streaming

//#region 🔖️Text
impl PdfSnapshot {
    /// 🔤 The Unicode text page `index` shows, raw codes decoded through the page's fonts.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn page_text(&self, index: usize) -> String {
        let Some(page) = self.pages.get(index) else { return String::new() };
        let codecs: HashMap<String, FontCodec> = self.fonts.iter().map(|font| (font.id.clone(), FontCodec::new(font))).collect();
        crate::standards::v1_7::subsets::base::modules::content::extract_text(&page.content, &codecs)
    }
}
//#endregion 🔖️Text

//#region 🔖️Builders
/// 📐 Simple text layout over a font: line breaking on words by real advance widths, returning
/// the operators that show the lines top-down from `(x, top)`.
pub struct PdfTextLayout<'a> {
    pub font: &'a PdfFont,
    pub size: f64,
    pub leading: f64,
    codec: FontCodec,
}

impl<'a> PdfTextLayout<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(font: &'a PdfFont, size: f64) -> Self {
        Self { font, size, leading: size * 1.2, codec: FontCodec::new(font) }
    }

    /// 📏 Width of `text` at this size in user space (`None` when the font cannot show it).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn width(&self, text: &str) -> Option<f64> {
        self.codec.text_width(text).map(|width| width * self.size / 1000.0)
    }

    /// 📏 Ascent of the face at this size (AFM/descriptor metrics, else 0.8 em).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn ascent(&self) -> f64 {
        let ascent = match &self.font.kind {
            PdfFontKind::Type1 { descriptor: Some(d), .. } | PdfFontKind::TrueType { descriptor: Some(d), .. } => d.ascent,
            PdfFontKind::Type0 { descendant, .. } => descendant.descriptor.ascent,
            _ => standard_font(self.font.base_font()).map(|metrics| metrics.ascender).unwrap_or(800.0),
        };
        (if ascent == 0.0 { 800.0 } else { ascent }) * self.size / 1000.0
    }

    /// 🔤 Breaks `text` into lines no wider than `max_width` (paragraphs split on `\n`; a word
    /// wider than the line is split by characters).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn wrap(&self, text: &str, max_width: f64) -> Vec<String> {
        let mut lines = Vec::new();
        for paragraph in text.split('\n') {
            let mut line = String::new();
            for word in paragraph.split(' ') {
                let candidate = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
                if self.width(&candidate).unwrap_or(0.0) <= max_width || line.is_empty() && self.width(word).unwrap_or(0.0) <= max_width {
                    line = candidate;
                    continue;
                }
                if !line.is_empty() {
                    lines.push(std::mem::take(&mut line));
                }
                let mut piece = String::new();
                for character in word.chars() {
                    let next = format!("{piece}{character}");
                    if self.width(&next).unwrap_or(0.0) > max_width && !piece.is_empty() {
                        lines.push(std::mem::take(&mut piece));
                    }
                    piece.push(character);
                }
                line = piece;
            }
            lines.push(line);
        }
        lines
    }

    /// 🖋️ Operators showing `lines` from `(x, top)` downwards (the first baseline sits one ascent
    /// below `top`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn show_lines(&self, lines: &[String], x: f64, top: f64) -> Vec<PdfOp> {
        let mut ops = vec![PdfOp::BeginText, PdfOp::SetFont { name: self.font.id.clone(), size: self.size }, PdfOp::SetLeading { leading: self.leading }, PdfOp::MoveText { tx: x, ty: top - self.ascent() }];
        for (index, line) in lines.iter().enumerate() {
            if index > 0 {
                ops.push(PdfOp::NextLine);
            }
            if !line.is_empty() {
                ops.push(PdfOp::ShowText { text: PdfTextString::text(line.clone()) });
            }
        }
        ops.push(PdfOp::EndText);
        ops
    }

    /// 🖋️ Wraps and shows `text` inside the rectangle `[x, top - height, x + width, top]`,
    /// dropping lines that do not fit; returns the operators and how many lines fit.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn show_paragraph(&self, text: &str, x: f64, top: f64, width: f64, height: f64) -> (Vec<PdfOp>, usize) {
        let lines = self.wrap(text, width);
        let fitting = ((height - self.ascent()) / self.leading).floor().max(0.0) as usize + 1;
        let shown: Vec<String> = lines.into_iter().take(fitting).collect();
        let count = shown.len();
        (self.show_lines(&shown, x, top), count)
    }
}

/// 📄️ A document of text pages: one page per `(width, height, text)` in Helvetica 12 pt with
/// 72 pt margins, wrapped by real metrics — the shape every text-only exporter shares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn text_document(pages: &[(f64, f64, &str)]) -> PdfSnapshot {
    let mut snapshot = PdfSnapshot::default();
    let font = PdfFont::standard("F1", "Helvetica");
    let layout = PdfTextLayout::new(&font, 12.0);
    for (width, height, text) in pages {
        let mut page = PdfPage::new(*width, *height);
        if !text.is_empty() {
            let margin = (width.min(*height) * 0.1).min(72.0);
            let (ops, _) = layout.show_paragraph(text, margin, height - margin, width - 2.0 * margin, height - 2.0 * margin);
            page.content = ops;
        }
        snapshot.pages.push(page);
    }
    if snapshot.pages.iter().any(|page| !page.content.is_empty()) {
        snapshot.fonts.push(font);
    }
    snapshot
}

/// 🔤 An embedded TrueType font as a Type 0 / CIDFontType2 (Identity-H, glyph ids as CIDs) with
/// widths and a ToUnicode map for every character of `text` (the whole cmap when `None`) — the
/// font shape any consumer with a `.ttf` in hand wants.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn embedded_true_type_font(id: &str, program: &[u8], base_font: &str, text: Option<&str>) -> Result<PdfFont, String> {
    use crate::standards::v1_7::subsets::base::modules::fonts::TrueTypeFont;
    let font = TrueTypeFont::parse(program)?;
    let characters: Vec<char> = match text {
        Some(text) => {
            let mut set: Vec<char> = text.chars().collect();
            set.sort_unstable();
            set.dedup();
            set
        }
        None => font.unicode_map.keys().filter_map(|code| char::from_u32(*code)).collect(),
    };
    let mut widths: std::collections::BTreeMap<u32, f64> = std::collections::BTreeMap::new();
    let mut mappings = Vec::new();
    for character in characters {
        if let Some(gid) = font.glyph_for_char(character) {
            widths.insert(gid as u32, font.advance_1000(gid));
            mappings.push(PdfToUnicodeMapping::Char { code: gid as u32, text: character.to_string() });
        }
    }
    let mut runs: Vec<PdfCidWidthRun> = Vec::new();
    for (gid, width) in widths {
        match runs.last_mut() {
            Some(run) if run.start_cid + run.widths.len() as u32 == gid => run.widths.push(width),
            _ => runs.push(PdfCidWidthRun { start_cid: gid, widths: vec![width] }),
        }
    }
    let flags = if font.fixed_pitch { 1 } else { 0 } | 32 | if font.italic_angle != 0.0 { 64 } else { 0 } | if font.weight_class.unwrap_or(400) >= 600 { 1 << 18 } else { 0 };
    let descriptor = PdfFontDescriptor {
        font_name: base_font.to_string(),
        flags,
        font_bbox: [font.scale_1000(font.bbox[0]), font.scale_1000(font.bbox[1]), font.scale_1000(font.bbox[2]), font.scale_1000(font.bbox[3])],
        italic_angle: font.italic_angle,
        ascent: font.scale_1000(font.ascender),
        descent: font.scale_1000(font.descender),
        cap_height: font.cap_height.map(|v| font.scale_1000(v)).unwrap_or_else(|| font.scale_1000(font.ascender)),
        stem_v: 80.0,
        x_height: font.x_height.map(|v| font.scale_1000(v)),
        ..PdfFontDescriptor::default()
    };
    Ok(PdfFont {
        id: id.to_string(),
        kind: PdfFontKind::Type0 {
            base_font: base_font.to_string(),
            cmap: PdfCMap::identity_h(),
            descendant: PdfCidFont { true_type: true, base_font: base_font.to_string(), system_info: PdfCidSystemInfo::default(), descriptor, default_width: 1000.0, widths: runs, default_vertical: None, vertical_metrics: Vec::new(), cid_to_gid: Some(PdfCidToGid::Identity), program: Some(PdfFontProgram::TrueType { data: program.to_vec() }), extra: Vec::new() },
        },
        to_unicode: Some(PdfToUnicode { byte_width: 2, mappings }),
        extra: Vec::new(),
    })
}

/// 🧾 Ids of every resource `ops` reference that the snapshot does not define — what a consumer
/// asserts is empty before writing.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unresolved_resources(snapshot: &PdfSnapshot, ops: &[PdfOp]) -> Vec<String> {
    let references = content_references(ops);
    let mut missing = Vec::new();
    missing.extend(references.fonts.iter().filter(|id| snapshot.font(id).is_none()).map(|id| format!("font {id}")));
    missing.extend(references.x_objects.iter().filter(|id| snapshot.image(id).is_none() && snapshot.form(id).is_none()).map(|id| format!("xobject {id}")));
    missing.extend(references.ext_g_states.iter().filter(|id| snapshot.ext_g_state(id).is_none()).map(|id| format!("extgstate {id}")));
    missing.extend(references.shadings.iter().filter(|id| snapshot.shading(id).is_none()).map(|id| format!("shading {id}")));
    missing.extend(references.patterns.iter().filter(|id| snapshot.pattern(id).is_none()).map(|id| format!("pattern {id}")));
    missing.extend(references.color_spaces.iter().filter(|id| !snapshot.color_spaces.iter().any(|space| &space.name == *id)).map(|id| format!("colorspace {id}")));
    missing
}
//#endregion 🔖️Builders

//#region 🧪️Tests
#[cfg(test)]
#[path = "🦀️tests-io.rs"]
mod tests;
//#endregion 🧪️Tests
