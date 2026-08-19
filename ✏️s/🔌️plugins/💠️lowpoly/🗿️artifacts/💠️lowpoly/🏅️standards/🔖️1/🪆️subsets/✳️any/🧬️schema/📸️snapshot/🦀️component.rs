//! 🧬️ Lowpoly snapshot schema — artifact-lane fields only.
//!
//! P6 handcrafted `ArtifactDsl`/`ArtifactPack` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`):
//! `LowpolyObject` carries a real `store::ArtifactChild<SemioMeshSnapshot>` handle for its `mesh`
//! slot, which `dsl::DslRecord`'s derive cannot represent (no `DslField` impl for `ArtifactChild<S>`)
//! — the same reason `✳️object`/`✳️kit` (stdio) hand-roll their own codecs rather than deriving. This
//! file follows their exact hex/bracket convention (`w2c-object-kit-report.md`), never a hand-written
//! slot list — `#[derive(ArtifactSchema)]` still emits `field_states()` for the top-level facets.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolyPaintLayer, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted lowpoly document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolySnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub objects: Vec<LowpolyObject>,
}

impl Default for LowpolySnapshot {
    async fn default() -> Self {
        Self { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️CodecPrimitives
async fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 { return Err(format!("odd hex length: {s:?}")); }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) async fn enc_str(s: &str) -> String { hex_encode(s.as_bytes()) }
pub(crate) async fn dec_str(s: &str) -> Result<String, String> { String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string()) }

use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};

pub(crate) async fn enc_ref(r: &store::os_io::ArtifactRef) -> String { enc_str(&r.to_uri()) }
pub(crate) async fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> { store::os_io::ArtifactRef::parse_uri(&dec_str(s)?) }

/// 🪪️ `[<hex child_id>,<hex target-uri>]` — the two-string handle, real and complete, never content.
pub(crate) async fn enc_child<S>(c: &store::ArtifactChild<S>) -> String { format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target)) }
pub(crate) async fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
pub(crate) async fn enc_child_opt<S>(c: &Option<store::ArtifactChild<S>>) -> String {
    match c { Some(c) => enc_child(c), None => "[]".to_string() }
}
pub(crate) async fn dec_child_opt<S>(s: &str) -> Result<Option<store::ArtifactChild<S>>, String> {
    if s == "[]" { return Ok(None); }
    Ok(Some(dec_child(s)?))
}

pub(crate) async fn enc_lowpoly_transform(t: &LowpolyTransform) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{}]",
        t.position[0], t.position[1], t.position[2],
        t.rotation[0], t.rotation[1], t.rotation[2],
        t.scale[0], t.scale[1], t.scale[2],
    )
}
pub(crate) async fn dec_lowpoly_transform(s: &str) -> Result<LowpolyTransform, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [px, py, pz, rx, ry, rz, sx, sy, sz] = parts.as_slice() else {
        return Err(format!("transform: expected 9 fields, got {}", parts.len()));
    };
    let f = |s: &str| s.trim().parse::<f32>().map_err(|e| e.to_string());
    Ok(LowpolyTransform { position: [f(px)?, f(py)?, f(pz)?], rotation: [f(rx)?, f(ry)?, f(rz)?], scale: [f(sx)?, f(sy)?, f(sz)?] })
}

pub(crate) async fn enc_paint_layer(l: &LowpolyPaintLayer) -> String {
    use base64::Engine;
    format!("[{},{},{},{},{}]", enc_str(&l.name), l.visible, l.opacity, enc_str(&l.blend_mode), base64::engine::general_purpose::STANDARD.encode(&l.pixels))
}
pub(crate) async fn dec_paint_layer(s: &str) -> Result<LowpolyPaintLayer, String> {
    use base64::Engine;
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, visible, opacity, blend_mode, pixels] = parts.as_slice() else {
        return Err(format!("paint layer: expected 5 fields, got {}", parts.len()));
    };
    Ok(LowpolyPaintLayer {
        name: dec_str(name)?,
        visible: visible.trim().parse().map_err(|e: std::str::ParseBoolError| e.to_string())?,
        opacity: opacity.trim().parse().map_err(|e: std::num::ParseFloatError| e.to_string())?,
        blend_mode: dec_str(blend_mode)?,
        pixels: base64::engine::general_purpose::STANDARD.decode(pixels.as_bytes()).map_err(|e| e.to_string())?,
    })
}
pub(crate) async fn enc_paint_layer_list(list: &[LowpolyPaintLayer]) -> String {
    format!("[{}]", list.iter().map(enc_paint_layer).collect::<Vec<_>>().join(","))
}
pub(crate) async fn dec_paint_layer_list(s: &str) -> Result<Vec<LowpolyPaintLayer>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_paint_layer).collect()
}

/// 🧊️ One object: `[id,name,transform,smooth-shading,mesh-handle,paint-layers]`. The live half-edge
/// mesh JSON content is DELIBERATELY absent — it is not a field of `LowpolyObject` at all (moved to
/// `✏️editor/🖌️session::LowpolyScratch`'s session-local `mesh_workspace` cache, ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` round-trip law fix round 2).
pub(crate) async fn enc_object(o: &LowpolyObject) -> String {
    format!(
        "[{},{},{},{},{},{}]",
        enc_str(&o.id), enc_str(&o.name), enc_lowpoly_transform(&o.transform), o.smooth_shading,
        enc_child_opt(&o.mesh), enc_paint_layer_list(&o.paint_layers),
    )
}
pub(crate) async fn dec_object(s: &str) -> Result<LowpolyObject, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, name, transform, smooth_shading, mesh, paint_layers] = parts.as_slice() else {
        return Err(format!("object: expected 6 fields, got {}", parts.len()));
    };
    Ok(LowpolyObject {
        id: dec_str(id)?,
        name: dec_str(name)?,
        transform: dec_lowpoly_transform(transform)?,
        smooth_shading: smooth_shading.trim().parse().map_err(|e: std::str::ParseBoolError| e.to_string())?,
        mesh: dec_child_opt(mesh)?,
        paint_layers: dec_paint_layer_list(paint_layers)?,
    })
}
pub(crate) async fn enc_object_list(list: &[LowpolyObject]) -> String {
    format!("[{}]", list.iter().map(enc_object).collect::<Vec<_>>().join(","))
}
pub(crate) async fn dec_object_list(s: &str) -> Result<Vec<LowpolyObject>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_object).collect()
}
//#endregion 🔖️CodecPrimitives

//#region 🔖️TextPrimitives
async fn print_lowpoly_snapshot_body(s: &LowpolySnapshot) -> String {
    format!("schema={}\nobjects={}", enc_str(&s.schema), enc_object_list(&s.objects))
}
async fn parse_lowpoly_snapshot_body(body: &str) -> Result<LowpolySnapshot, String> {
    let mut schema = None;
    let mut objects = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        if let Some(rest) = line.strip_prefix("schema=") { schema = Some(dec_str(rest)?); }
        else if let Some(rest) = line.strip_prefix("objects=") { objects = dec_object_list(rest)?; }
        else { return Err(format!("lowpoly snapshot: unknown line {line:?}")); }
    }
    Ok(LowpolySnapshot {
        schema: schema.ok_or_else(|| "lowpoly snapshot: missing schema line".to_string())?,
        objects,
    })
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
pub(crate) async fn write_str_lp(out: &mut Vec<u8>, s: &str) { write_bytes_lp(out, s.as_bytes()); }
pub(crate) async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> { String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string()) }

pub(crate) async fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) { write_str_lp(out, &r.to_uri()); }
pub(crate) async fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
pub(crate) async fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
pub(crate) async fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}
pub(crate) async fn write_child_opt<S>(out: &mut Vec<u8>, c: &Option<store::ArtifactChild<S>>) {
    match c {
        Some(c) => { out.push(1); write_child(out, c); }
        None => out.push(0),
    }
}
pub(crate) async fn read_child_opt<S>(reader: &mut store::ByteReader<'_>) -> Result<Option<store::ArtifactChild<S>>, String> {
    let presence = reader.read_u8().map_err(|e| e.to_string())?;
    if presence == 0 { Ok(None) } else { Ok(Some(read_child(reader)?)) }
}

async fn write_lowpoly_transform(out: &mut Vec<u8>, t: &LowpolyTransform) {
    for v in t.position.iter().chain(t.rotation.iter()).chain(t.scale.iter()) {
        out.extend_from_slice(&v.to_le_bytes());
    }
}
async fn read_lowpoly_transform(reader: &mut store::ByteReader<'_>) -> Result<LowpolyTransform, String> {
    let mut next = || -> Result<f32, String> { Ok(f32::from_le_bytes(reader.read_bytes(4).map_err(|e| e.to_string())?.try_into().map_err(|_| "transform: short read".to_string())?)) };
    Ok(LowpolyTransform { position: [next()?, next()?, next()?], rotation: [next()?, next()?, next()?], scale: [next()?, next()?, next()?] })
}

async fn write_paint_layer(out: &mut Vec<u8>, l: &LowpolyPaintLayer) {
    write_str_lp(out, &l.name);
    out.push(l.visible as u8);
    out.extend_from_slice(&l.opacity.to_le_bytes());
    write_str_lp(out, &l.blend_mode);
    write_bytes_lp(out, &l.pixels);
}
async fn read_paint_layer(reader: &mut store::ByteReader<'_>) -> Result<LowpolyPaintLayer, String> {
    let name = read_str_lp(reader)?;
    let visible = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let opacity = f32::from_le_bytes(reader.read_bytes(4).map_err(|e| e.to_string())?.try_into().map_err(|_| "paint layer: short read".to_string())?);
    let blend_mode = read_str_lp(reader)?;
    let pixels = read_bytes_lp(reader)?;
    Ok(LowpolyPaintLayer { name, visible, opacity, blend_mode, pixels })
}
async fn write_paint_layer_list(out: &mut Vec<u8>, list: &[LowpolyPaintLayer]) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for l in list { write_paint_layer(out, l); }
}
async fn read_paint_layer_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<LowpolyPaintLayer>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_paint_layer(reader)).collect()
}

async fn write_object(out: &mut Vec<u8>, o: &LowpolyObject) {
    write_str_lp(out, &o.id);
    write_str_lp(out, &o.name);
    write_lowpoly_transform(out, &o.transform);
    out.push(o.smooth_shading as u8);
    write_child_opt(out, &o.mesh);
    write_paint_layer_list(out, &o.paint_layers);
}
async fn read_object(reader: &mut store::ByteReader<'_>) -> Result<LowpolyObject, String> {
    let id = read_str_lp(reader)?;
    let name = read_str_lp(reader)?;
    let transform = read_lowpoly_transform(reader)?;
    let smooth_shading = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let mesh = read_child_opt(reader)?;
    let paint_layers = read_paint_layer_list(reader)?;
    Ok(LowpolyObject { id, name, transform, smooth_shading, mesh, paint_layers })
}
async fn write_object_list(out: &mut Vec<u8>, list: &[LowpolyObject]) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for o in list { write_object(out, o); }
}
async fn read_object_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<LowpolyObject>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_object(reader)).collect()
}

async fn encode_lowpoly_snapshot_binary(s: &LowpolySnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_object_list(&mut out, &s.objects);
    out
}
async fn decode_lowpoly_snapshot_binary(bytes: &[u8]) -> Result<LowpolySnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT { return Err(format!("unsupported pack format {format}")); }
    let schema = read_str_lp(&mut reader)?;
    let objects = read_object_list(&mut reader)?;
    Ok(LowpolySnapshot { schema, objects })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack — hand-rolled (not derive-generated) because
/// `LowpolyObject.mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>` has no `DslField` impl for
/// the derive to bind against; see this file's own module doc comment.
impl store::ArtifactDsl for LowpolySnapshot {
    const EXTENSION: &'static str = "lowpoly";
    async fn envelope_id() -> &'static str { "lowpoly.lowpoly" }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_lowpoly_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let body = print_lowpoly_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for LowpolySnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_lowpoly_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_lowpoly_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️DocumentHelpers
/// 🏗️ Builds a single-object snapshot from mesh JSON — only the real persisted `mesh` handle
/// (content-addressed off `mesh_json` via `mesh_child_handle`, identical geometry always resolving
/// to the identical handle). The caller is responsible for seeding its OWN session-local
/// `mesh_workspace` cache (`✏️editor/🖌️session::LowpolyScratch`) with `mesh_json` under
/// `object_id` — this function no longer does it implicitly (round 2 of this ticket's round-trip
/// law fix: `LowpolyObject` carries no live mesh content at all any more, see that struct's own doc
/// comment).
pub async fn snapshot_from_mesh_json(mesh_json: &str, object_id: &str, object_name: &str) -> LowpolySnapshot {
    LowpolySnapshot {
        schema: LOWPOLY_DOCUMENT_SCHEMA.into(),
        objects: vec![LowpolyObject {
            id: object_id.into(),
            name: object_name.into(),
            transform: LowpolyTransform::default(),
            smooth_shading: false,
            mesh: Some(crate::artifacts::lowpoly::mesh_child_handle(object_id, mesh_json)),
            paint_layers: vec![LowpolyPaintLayer::new("Base")],
        }],
    }
}
//#endregion 🔖️DocumentHelpers
