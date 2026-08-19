//! 🧬️ Layout snapshot schema — artifact-lane fields only.

use crate::artifacts::layout::{
    CharacterStyle, Frame, GridSettings, ImageLink, Layer, LayoutDrawingChild, Page, PageColumns, PageMargins, ParagraphStyle,
    ParentPage, Spread, TextStory, LAYOUT_DOCUMENT_SCHEMA,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted layout document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `background_drawing` composes stdio's real
/// `s.stdio.semio/v1/drawing` subset as a genuine child slot (see the artifact root's
/// `🔖️ComposedTypes` region doc for the full before/after); `referenced_model` is a forward
/// `ArtifactLink` reference slot, both new. `#[child(...)]`/`#[link_slot(...)]` drive
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Dropped the
/// `dsl::DslArtifact` derive this struct used to carry — `ArtifactChild<S>`/`ArtifactLink` have no
/// `dsl::DslField` impl reachable from this crate, the same wall `✳️object`/`✳️kit`/cad hit; every
/// field's text/binary shape is now hand-rolled below instead (JSON-then-hex for structured fields,
/// same convention cad's `📸️snapshot/🦀️component.rs` established for this ticket).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.layout.layout")]
pub struct LayoutSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    pub grid: GridSettings,
    #[state(artifact)]
    #[serde(rename = "paragraphStyles")]
    pub paragraph_styles: Vec<ParagraphStyle>,
    #[state(artifact)]
    #[serde(rename = "characterStyles")]
    pub character_styles: Vec<CharacterStyle>,
    #[state(artifact)]
    pub stories: Vec<TextStory>,
    #[state(artifact)]
    pub links: Vec<ImageLink>,
    #[state(artifact)]
    #[serde(rename = "parentPages")]
    pub parent_pages: Vec<ParentPage>,
    #[state(artifact)]
    pub spreads: Vec<Spread>,
    #[state(artifact)]
    pub pages: Vec<Page>,
    #[state(artifact)]
    #[serde(rename = "printTarget")]
    pub print_target: Option<String>,
    #[state(artifact)]
    #[serde(rename = "dataFieldsJson", default, skip_serializing_if = "Option::is_none")]
    pub data_fields_json: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.drawing")]
    #[serde(rename = "backgroundDrawing", default, skip_serializing_if = "Option::is_none")]
    pub background_drawing: Option<LayoutDrawingChild>,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    #[serde(rename = "referencedModel", default, skip_serializing_if = "Option::is_none")]
    pub referenced_model: Option<store::ArtifactLink>,
}

/// 🧷️ Real "empty" constructor used as the parse/decode starting point (mirrors cad's
/// `empty_cad_snapshot`) — `default_document()` at `crate::artifacts::layout::schema` seeds a full
/// demo document instead, so this can't reuse a `Default` impl (this type has none).
pub(crate) async fn empty_layout_snapshot() -> LayoutSnapshot {
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
//#endregion 🔖️Snapshot

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec — identical shape to cad's own `enc_child`/`dec_child`
/// (the working reference for a composite subset's child-handle primitives): a handle is exactly two
/// strings (`child_id`, the target's `ArtifactRef` flattened via `to_uri()`), never the child's own
/// content.
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) async fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
pub(crate) async fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}

pub(crate) async fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
pub(crate) async fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
pub(crate) async fn enc_child_opt<S>(c: &Option<store::ArtifactChild<S>>) -> String {
    match c {
        Some(c) => enc_child(c),
        None => "[]".to_string(),
    }
}
pub(crate) async fn dec_child_opt<S>(s: &str) -> Result<Option<store::ArtifactChild<S>>, String> {
    if s == "[]" {
        return Ok(None);
    }
    Ok(Some(dec_child(s)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️JsonFieldPrimitives
/// 🧾️ Every collection/nested-record field on `LayoutSnapshot` (`grid`, the style/story/link/page
/// tables, `parent_pages`, `spreads`, `pages` — each already `Serialize`/`Deserialize`) is
/// JSON-serialized then hex-encoded, one line per field, matching every scalar field's own
/// `enc_str`/`dec_str` convention (see cad's identically-named region for precedent). `referenced_model`
/// (an `Option<store::ArtifactLink>`) uses the same helper — `ArtifactLink`/`LinkPin`/`BlobRef` are
/// themselves plain `Serialize`/`Deserialize`, so no bespoke hex/bracket encoder was needed for it.
async fn enc_json<T: Serialize>(value: &T) -> String {
    enc_str(&serde_json::to_string(value).expect("LayoutSnapshot structured fields are always JSON-serializable"))
}
async fn dec_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
}
//#endregion 🔖️JsonFieldPrimitives

//#region 🔖️TextPrimitives
async fn print_layout_snapshot_body(s: &LayoutSnapshot) -> String {
    format!(
        "schema={}\nname={}\ngrid={}\nparagraphStyles={}\ncharacterStyles={}\nstories={}\nlinks={}\nparentPages={}\nspreads={}\npages={}\nprintTarget={}\ndataFieldsJson={}\nbackgroundDrawing={}\nreferencedModel={}",
        enc_str(&s.schema),
        enc_str(&s.name),
        enc_json(&s.grid),
        enc_json(&s.paragraph_styles),
        enc_json(&s.character_styles),
        enc_json(&s.stories),
        enc_json(&s.links),
        enc_json(&s.parent_pages),
        enc_json(&s.spreads),
        enc_json(&s.pages),
        enc_json(&s.print_target),
        enc_json(&s.data_fields_json),
        enc_child_opt(&s.background_drawing),
        enc_json(&s.referenced_model),
    )
}
async fn parse_layout_snapshot_body(body: &str) -> Result<LayoutSnapshot, String> {
    let mut snapshot = empty_layout_snapshot();
    let mut saw_schema = false;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            snapshot.schema = dec_str(rest)?;
            saw_schema = true;
        } else if let Some(rest) = line.strip_prefix("name=") {
            snapshot.name = dec_str(rest)?;
        } else if let Some(rest) = line.strip_prefix("grid=") {
            snapshot.grid = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("paragraphStyles=") {
            snapshot.paragraph_styles = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("characterStyles=") {
            snapshot.character_styles = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("stories=") {
            snapshot.stories = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("links=") {
            snapshot.links = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("parentPages=") {
            snapshot.parent_pages = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("spreads=") {
            snapshot.spreads = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("pages=") {
            snapshot.pages = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("printTarget=") {
            snapshot.print_target = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("dataFieldsJson=") {
            snapshot.data_fields_json = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("backgroundDrawing=") {
            snapshot.background_drawing = dec_child_opt(rest)?;
        } else if let Some(rest) = line.strip_prefix("referencedModel=") {
            snapshot.referenced_model = dec_json(rest)?;
        } else {
            return Err(format!("layout snapshot: unknown line {line:?}"));
        }
    }
    if !saw_schema {
        return Err("layout snapshot: missing schema line".to_string());
    }
    Ok(snapshot)
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
async fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
async fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
async fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
async fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
async fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}
async fn write_child_opt<S>(out: &mut Vec<u8>, c: &Option<store::ArtifactChild<S>>) {
    match c {
        Some(c) => {
            out.push(1);
            write_child(out, c);
        }
        None => out.push(0),
    }
}
async fn read_child_opt<S>(reader: &mut store::ByteReader<'_>) -> Result<Option<store::ArtifactChild<S>>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        _ => Ok(Some(read_child(reader)?)),
    }
}
async fn write_json<T: Serialize>(out: &mut Vec<u8>, value: &T) {
    write_str_lp(out, &serde_json::to_string(value).expect("LayoutSnapshot structured fields are always JSON-serializable"));
}
async fn read_json<T: serde::de::DeserializeOwned>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    serde_json::from_str(&read_str_lp(reader)?).map_err(|e| e.to_string())
}

async fn encode_layout_snapshot_binary(s: &LayoutSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_str_lp(&mut out, &s.name);
    write_json(&mut out, &s.grid);
    write_json(&mut out, &s.paragraph_styles);
    write_json(&mut out, &s.character_styles);
    write_json(&mut out, &s.stories);
    write_json(&mut out, &s.links);
    write_json(&mut out, &s.parent_pages);
    write_json(&mut out, &s.spreads);
    write_json(&mut out, &s.pages);
    write_json(&mut out, &s.print_target);
    write_json(&mut out, &s.data_fields_json);
    write_child_opt(&mut out, &s.background_drawing);
    write_json(&mut out, &s.referenced_model);
    out
}
async fn decode_layout_snapshot_binary(bytes: &[u8]) -> Result<LayoutSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let mut snapshot = empty_layout_snapshot();
    snapshot.schema = read_str_lp(&mut reader)?;
    snapshot.name = read_str_lp(&mut reader)?;
    snapshot.grid = read_json(&mut reader)?;
    snapshot.paragraph_styles = read_json(&mut reader)?;
    snapshot.character_styles = read_json(&mut reader)?;
    snapshot.stories = read_json(&mut reader)?;
    snapshot.links = read_json(&mut reader)?;
    snapshot.parent_pages = read_json(&mut reader)?;
    snapshot.spreads = read_json(&mut reader)?;
    snapshot.pages = read_json(&mut reader)?;
    snapshot.print_target = read_json(&mut reader)?;
    snapshot.data_fields_json = read_json(&mut reader)?;
    snapshot.background_drawing = read_child_opt(&mut reader)?;
    snapshot.referenced_model = read_json(&mut reader)?;
    Ok(snapshot)
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack`, real hex/bracket text + LEB128-length-prefixed binary
/// primitives — same upgrade `✳️object`/`✳️kit`/cad made when they gained real `ArtifactChild<S>`
/// slots (the old `dsl::DslArtifact`-derive-driven `Self::__dsl_spec()` path cannot express a
/// composed child slot or a link slot, neither of which has a `dsl::DslField` impl reachable from
/// this crate).
impl store::ArtifactDsl for LayoutSnapshot {
    const EXTENSION: &'static str = "layout";
    async fn envelope_id() -> &'static str {
        LAYOUT_DOCUMENT_SCHEMA
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_layout_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let body = print_layout_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for LayoutSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_layout_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_layout_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
mod round_trip_tests {
    use super::*;
    use crate::artifacts::layout::{LayoutBounds, Page, PageColumns, PageMargins};

    async fn sample_with_composition() -> LayoutSnapshot {
        let mut snapshot = empty_layout_snapshot();
        snapshot.schema = LAYOUT_DOCUMENT_SCHEMA.into();
        snapshot.name = "Composed".into();
        snapshot.paragraph_styles = vec![ParagraphStyle {
            id: "paragraph.body".into(),
            name: "Body".into(),
            font_family: "Layout Sans".into(),
            font_size: 12.0,
            font_weight: 400,
            leading: 14.4,
            tracking: 0.0,
            alignment: "left".into(),
        }];
        snapshot.pages = vec![Page {
            id: "page-1".into(),
            name: "Page 1".into(),
            spread_id: "spread-1".into(),
            parent_page_id: None,
            width: 200.0,
            height: 200.0,
            margins: PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
            columns: PageColumns { count: 1, gutter: 0.0 },
            guides: Vec::new(),
            layer_ids: Vec::new(),
            layers: Vec::new(),
            frames: vec![Frame::Rect {
                id: "frame-1".into(),
                layer_id: "layer-1".into(),
                bounds: LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 },
                locked: None,
                visible: None,
                fill: Some([1.0, 1.0, 1.0, 1.0]),
                stroke: None,
            }],
            overrides: Vec::new(),
        }];
        snapshot.background_drawing = Some(store::ArtifactChild::new(
            "child-drawing-1".to_string(),
            store::os_io::ArtifactRef::parse_uri("doc-1!s.stdio.semio@v1/drawing").expect("valid child ref uri"),
        ));
        snapshot.referenced_model = Some(store::ArtifactLink {
            target: store::os_io::ArtifactRef::parse_uri("doc-2!s.stdio.semio@v1/model").expect("valid link ref uri"),
            pin: store::LinkPin::Head,
            role: "model".into(),
        });
        snapshot
    }

    /// 🧪️ Every field on `LayoutSnapshot` — including the two new composition slots — must survive
    /// both hand-rolled codecs (text and binary), independently. Codec completeness is not caught by
    /// `cargo check`; this is the real round-trip proof the migration recipe requires.
    #[test]
    async fn background_drawing_and_referenced_model_round_trip_through_text_and_binary() {
        let snapshot = sample_with_composition();
        let text = store::ArtifactDsl::print_dsl(&snapshot);
        let from_text = <LayoutSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse round-tripped text");
        assert_eq!(from_text, snapshot);

        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let from_binary = <LayoutSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode round-tripped binary");
        assert_eq!(from_binary, snapshot);
    }

    #[test]
    async fn absent_composition_slots_round_trip_as_none() {
        let mut snapshot = sample_with_composition();
        snapshot.background_drawing = None;
        snapshot.referenced_model = None;
        let text = store::ArtifactDsl::print_dsl(&snapshot);
        assert_eq!(<LayoutSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), snapshot);
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        assert_eq!(<LayoutSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode"), snapshot);
    }
}
//#endregion 🧪️Tests
