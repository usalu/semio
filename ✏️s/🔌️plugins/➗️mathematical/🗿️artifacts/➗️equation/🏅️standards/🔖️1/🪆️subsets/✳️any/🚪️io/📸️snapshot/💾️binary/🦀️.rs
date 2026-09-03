//! 📦️ Equation artifact — binary document surface + laws (constitutional: pack). The
//! `store::ArtifactPack` impl for `EquationSnapshot` lives here rather than next to
//! `EquationSnapshot` itself (design.md §1 CORRECTION: unsplit, one bidirectional impl per
//! representation, sitting directly under `🚪️io/<facet>/<representation>/`) — moved here verbatim
//! from `🧬️schema/📸️snapshot/🦀️.rs`.

use crate::artifacts::equation::standards::v1::subsets::any::schema::snapshot::EquationExprSnapshot;
use crate::artifacts::equation::EquationSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

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

async fn write_equation(out: &mut Vec<u8>, e: &EquationExprSnapshot) {
    write_bytes_lp(out, pack::json::to_json_string(e).as_bytes());
}
async fn read_equation(reader: &mut store::ByteReader<'_>) -> Result<EquationExprSnapshot, String> {
    let bytes = read_bytes_lp(reader)?;
    let text = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    pack::json::from_json_str(&text).map_err(|e| e.to_string())
}

async fn encode_equation_snapshot_binary(s: &EquationSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_child(&mut out, &s.notation);
    write_child(&mut out, &s.results);
    write_child(&mut out, &s.computed);
    write_equation(&mut out, &s.mathematical);
    out
}
async fn decode_equation_snapshot_binary(bytes: &[u8]) -> Result<EquationSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    Ok(EquationSnapshot { notation: read_child(&mut reader)?, results: read_child(&mut reader)?, computed: read_child(&mut reader)?, equation: read_equation(&mut reader)? })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactPack
/// ✉️ P6 handcrafted `ArtifactPack`, real LEB128 binary primitives — moved here verbatim from
/// `🧬️schema/📸️snapshot/🦀️.rs`.
impl store::ArtifactPack for EquationSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let _ = options;
        let raw = encode_equation_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_equation_snapshot_binary(&inner).map_err(PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactPack

/// 📦️ Encodes a `EquationSnapshot` to its binary pack form.
pub async fn encode(snapshot: &EquationSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

/// 📖️ Decodes a `EquationSnapshot` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<EquationSnapshot, PackError> {
    <EquationSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::equation::{EquationGeometry, EquationGraph};

    #[semio_framework_async_macros::async_test]
    async fn equation_snapshot_dsl_pack_equivalence_default() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&EquationSnapshot::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn equation_snapshot_dsl_pack_equivalence_with_seed_and_empty_collections() {
        let mut graph = EquationGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..EquationGraph::default() };
        graph.nodes.clear();
        graph.edges.clear();
        let snapshot = crate::artifacts::equation::equation_snapshot_with_state(graph, EquationGeometry { points: Vec::new() });
        store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
    }

    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::equation::standards::v1::subsets::graph::schema::mutations::update_graph_algorithm::mutation::UpdateGraphAlgorithm;
        use crate::artifacts::equation::op::EquationMutation;
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let mut store: ArtifactStore<EquationSnapshot, EquationMutation> = ArtifactStore::new(create_document_envelope("semio.equation/v1", "math-demo", EquationSnapshot::default(), None)).expect("valid artifact store fixture");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: "components".into(), new_algorithm_seed: None })], description: None }).expect("apply");
        let edit: &Edit<EquationMutation> = store.envelope().vcs.edits.last().expect("edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<EquationSnapshot, EquationMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
}
//#endregion 🧪️Tests
