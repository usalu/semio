//! 🧬️ Wires snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`reasoning/dag→C:graph`): `content` composes
//! stdio's neutral `s.stdio.semio.graph` subset (nodes/edges) instead of an inline `board_fixture`
//! blob. `camera`/`meta` stay their own small persisted `DslValue` fields (view state / app config,
//! never part of the neutral graph subset — see `crate::artifacts::wires`'s module doc).
//!
//! The old `WiresSnapshotDsl` mirror (`#[derive(dsl::DslRecord)]`) is gone: `ArtifactChild<S>` has no
//! `dsl::DslRecord` derive support (`📓️migration-recipe.md` §2), so this hand-rolls
//! `ArtifactDsl`/`ArtifactPack` for the whole struct instead. Both codecs carry the REAL node/edge
//! JSON, never just the opaque `(child_id, target)` handle pair — a bare-handle codec is
//! unrecoverable on a fresh process (see `dag`'s `📓️wave4-reports/dag-report.md`, "a real bug found
//! and fixed during this pass"); `parse_dsl`/`decode_pack` decode the real `nodes`/`edges` JSON and
//! mint+cache a fresh, deterministic, content-addressed handle from them every time (same data ⇒ same
//! handle, so peers replaying the same bytes converge).

use crate::artifacts::wires::{wires_working_scene, WiresContentChild};
use dsl::DslValue;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted wires document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresSnapshot {
    #[state(artifact)]
    pub wires_fixture: DslValue,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.graph")]
    pub content: WiresContentChild,
    #[state(artifact)]
    pub camera: DslValue,
    #[state(artifact)]
    pub meta: DslValue,
}
//#endregion 🔖️Snapshot

//#region 🔖️CodecPrimitives
/// 🧪️ Real hex-encoded text primitives — one `key=<hex>` line per field (`📓️migration-recipe.md`
/// §2's convention), duplicated locally rather than imported across facets (keeps this file
/// independently compilable, matching `✳️graph`'s own `🔖️GraphPrimitives` precedent in stdio).
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

/// ⚠️ Serializes/deserializes `DslValue` DIRECTLY (`serde_json::to_string`/`from_str::<DslValue>`),
/// never via the `dsl_to_json`/`serde_json::Value` intermediate `crate::artifacts::wires::schema`'s
/// `fixture_json_string`/`dsl_to_json` use elsewhere: `serde_json::Value::Object` normalizes key
/// order (alphabetical, no `preserve_order` feature), which silently reordered `wires_fixture`'s
/// object keys on every round trip and broke `DslValue::Object`'s (order-sensitive, `Vec`-backed)
/// `PartialEq` — a real bug this pass's round-trip tests caught (not just latent risk). `DslValue`'s
/// own hand-written `Serialize`/`Deserialize` impl (`dsl_value_serde.rs`) preserves entry order
/// end-to-end, so encoding/decoding it directly (bypassing `serde_json::Value` entirely) is lossless.
fn enc_dsl(value: &DslValue) -> String {
    hex_encode(serde_json::to_string(value).unwrap_or_default().as_bytes())
}
fn dec_dsl(s: &str) -> Result<DslValue, String> {
    let bytes = hex_decode(s)?;
    let text = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    serde_json::from_str::<DslValue>(&text).map_err(|e| e.to_string())
}
fn enc_dsl_list(values: &[DslValue]) -> String {
    hex_encode(serde_json::to_string(values).unwrap_or_default().as_bytes())
}
fn dec_dsl_list(s: &str) -> Result<Vec<DslValue>, String> {
    let bytes = hex_decode(s)?;
    let text = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    serde_json::from_str::<Vec<DslValue>>(&text).map_err(|e| e.to_string())
}

fn to_text_error(message: String) -> store::TextError {
    store::TextError::new(message, dsl::TextSpan::at(1, 1))
}

/// 📄️ The real structured body: `wires=<hex>` / `nodes=[<hex>...]` / `edges=[<hex>...]` /
/// `camera=<hex>` / `meta=<hex>` — five lines, each independently hex-decodable.
fn print_wires_snapshot_body(snapshot: &WiresSnapshot) -> String {
    let scene = wires_working_scene(snapshot);
    format!(
        "wires={}\nnodes={}\nedges={}\ncamera={}\nmeta={}",
        enc_dsl(&snapshot.wires_fixture),
        enc_dsl_list(&scene.nodes),
        enc_dsl_list(&scene.edges),
        enc_dsl(&snapshot.camera),
        enc_dsl(&snapshot.meta),
    )
}

fn parse_wires_snapshot_body(body: &str) -> Result<WiresSnapshot, store::TextError> {
    let mut wires_fixture = None;
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut camera = None;
    let mut meta = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("wires=") {
            wires_fixture = Some(dec_dsl(rest).map_err(to_text_error)?);
        } else if let Some(rest) = line.strip_prefix("nodes=") {
            nodes = dec_dsl_list(rest).map_err(to_text_error)?;
        } else if let Some(rest) = line.strip_prefix("edges=") {
            edges = dec_dsl_list(rest).map_err(to_text_error)?;
        } else if let Some(rest) = line.strip_prefix("camera=") {
            camera = Some(dec_dsl(rest).map_err(to_text_error)?);
        } else if let Some(rest) = line.strip_prefix("meta=") {
            meta = Some(dec_dsl(rest).map_err(to_text_error)?);
        } else {
            return Err(to_text_error(format!("wires snapshot: unknown line {line:?}")));
        }
    }
    let content = crate::artifacts::wires::wires_content_child_handle_and_cache(nodes, edges);
    Ok(WiresSnapshot {
        wires_fixture: wires_fixture.ok_or_else(|| to_text_error("wires snapshot: missing wires line".into()))?,
        content,
        camera: camera.unwrap_or_else(crate::artifacts::wires::empty_camera),
        meta: meta.unwrap_or(DslValue::Null),
    })
}

/// 🎁 Binary mirrors the text codec exactly, field for field, length-prefixed
/// (`store::pack_rt::write_varint_u64`/`store::ByteReader`, the same varint-length-prefix primitives
/// `✳️graph`'s own `🔖️BinaryPrimitives` uses).
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}
/// ⚠️ Same order-preserving direct `DslValue` (de)serialization as `enc_dsl`/`dec_dsl` above — never
/// via `fixture_json_string`/`dsl_to_json`'s `serde_json::Value` intermediate (key-order-losing).
fn write_dsl(out: &mut Vec<u8>, value: &DslValue) {
    write_str_lp(out, &serde_json::to_string(value).unwrap_or_default());
}
fn read_dsl(reader: &mut store::ByteReader<'_>) -> Result<DslValue, String> {
    let text = read_str_lp(reader)?;
    serde_json::from_str::<DslValue>(&text).map_err(|e| e.to_string())
}
fn write_dsl_list(out: &mut Vec<u8>, values: &[DslValue]) {
    store::pack_rt::write_varint_u64(out, values.len() as u64);
    for value in values {
        write_dsl(out, value);
    }
}
fn read_dsl_list(reader: &mut store::ByteReader<'_>) -> Result<Vec<DslValue>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_dsl(reader)).collect()
}

fn encode_wires_snapshot_binary(snapshot: &WiresSnapshot) -> Vec<u8> {
    let scene = wires_working_scene(snapshot);
    let mut out = Vec::new();
    write_dsl(&mut out, &snapshot.wires_fixture);
    write_dsl_list(&mut out, &scene.nodes);
    write_dsl_list(&mut out, &scene.edges);
    write_dsl(&mut out, &snapshot.camera);
    write_dsl(&mut out, &snapshot.meta);
    out
}

fn decode_wires_snapshot_binary(bytes: &[u8]) -> Result<WiresSnapshot, String> {
    let mut reader = store::ByteReader::new(bytes);
    let wires_fixture = read_dsl(&mut reader)?;
    let nodes = read_dsl_list(&mut reader)?;
    let edges = read_dsl_list(&mut reader)?;
    let camera = read_dsl(&mut reader)?;
    let meta = read_dsl(&mut reader)?;
    let content = crate::artifacts::wires::wires_content_child_handle_and_cache(nodes, edges);
    Ok(WiresSnapshot { wires_fixture, content, camera, meta })
}
//#endregion 🔖️CodecPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits once `content`
/// drops to a composed `ArtifactChild` — see this file's module doc).
impl store::ArtifactDsl for WiresSnapshot {
    const EXTENSION: &'static str = "wires";
    fn envelope_id() -> &'static str {
        "reasoning.wires"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_wires_snapshot_body(body)
    }
    fn print_dsl(&self) -> String {
        let body = print_wires_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for WiresSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_wires_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_wires_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::{empty_wires_snapshot, wires_working_board};

    fn populated() -> WiresSnapshot {
        let mut snapshot = empty_wires_snapshot();
        let node = dsl::to_dsl_value(&serde_json::json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 1.0, "y": 2.0, "radius": 24.0, "text": "Alpha", "handles": [] })).unwrap();
        snapshot = store::apply_mutation(&snapshot, &crate::artifacts::wires::mutations::create_node(node))
            .expect("valid mutation")
            .0;
        snapshot
    }

    #[test]
    fn dsl_text_round_trips_empty() {
        let snapshot = empty_wires_snapshot();
        let text = <WiresSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        let back = <WiresSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(back.wires_fixture, snapshot.wires_fixture);
        assert_eq!(wires_working_board(&back), wires_working_board(&snapshot));
    }

    #[test]
    fn pack_round_trips_empty() {
        let snapshot = empty_wires_snapshot();
        let bytes = <WiresSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let back = <WiresSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(back.wires_fixture, snapshot.wires_fixture);
        assert_eq!(wires_working_board(&back), wires_working_board(&snapshot));
    }

    /// ⚖️ codec_retention_law: a populated snapshot (real node content, not just the default) survives
    /// BOTH codecs — this is what a bare-handle-only codec would silently fail (see this file's module
    /// doc, `dag`'s bug writeup).
    #[test]
    fn codec_retention_law_carries_real_node_content_not_just_the_handle() {
        let snapshot = populated();
        let text = <WiresSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        let back_text = <WiresSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(wires_working_board(&back_text).get("nodes").and_then(|v| v.as_array()).map(|a| a.len()), Some(1), "node content must survive a FRESH decode, not just round-trip in-process");
        let bytes = <WiresSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let back_pack = <WiresSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(wires_working_board(&back_pack).get("nodes").and_then(|v| v.as_array()).map(|a| a.len()), Some(1));
    }
}
//#endregion 🧪️Tests
