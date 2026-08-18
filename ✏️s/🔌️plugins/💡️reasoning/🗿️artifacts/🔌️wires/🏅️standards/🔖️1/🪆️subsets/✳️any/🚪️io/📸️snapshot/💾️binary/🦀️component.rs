//! 🎁 Wires artifact — native `.wires` binary pack codec (ticket
//! `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` design.md §1 CORRECTION: the native codec is
//! one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>/`, unsplit —
//! relocated here verbatim from `🧬️schema/📸️snapshot/💾️binary`, taking `impl store::ArtifactPack for
//! WiresSnapshot` with it from `🧬️schema/📸️snapshot/🦀️component.rs`'s former
//! `🔖️HandcraftedArtifactCodecs` region). Mirrors the sibling `📝️text` codec exactly, field for field,
//! length-prefixed (`store::pack_rt::write_varint_u64`/`store::ByteReader`, the same varint-length-
//! prefix primitives `✳️graph`'s own `🔖️BinaryPrimitives` uses).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");

use crate::artifacts::wires::{wires_working_scene, WiresSnapshot};
use dsl::DslValue;

//#region 🔖️BinaryPrimitives
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}
/// ⚠️ Same order-preserving direct `DslValue` (de)serialization as `📝️text`'s `enc_dsl`/`dec_dsl` —
/// never via `fixture_json_string`/`dsl_to_json`'s `serde_json::Value` intermediate (key-order-losing).
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
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactPack
/// ✉️ P6 handcrafted `ArtifactPack` (derive no longer emits this trait once `content` drops to a
/// composed `ArtifactChild` — see the sibling `📝️text` file's module doc).
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
//#endregion 🔖️HandcraftedArtifactPack

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::{empty_wires_snapshot, wires_working_board};

    #[test]
    fn pack_round_trips_empty() {
        let snapshot = empty_wires_snapshot();
        let bytes = <WiresSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let back = <WiresSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(back.wires_fixture, snapshot.wires_fixture);
        assert_eq!(wires_working_board(&back), wires_working_board(&snapshot));
    }
}
//#endregion 🧪️Tests
