//! 📦️ Equation artifact — binary document surface + laws (constitutional: pack). The
//! `store::ArtifactPack` impl for `EquationSnapshot` lives here rather than next to
//! `EquationSnapshot` itself (design.md §1 CORRECTION: unsplit, one bidirectional impl per
//! representation, sitting directly under `🚪️io/<facet>/<representation>/`) — moved here verbatim
//! from `🧬️schema/📸️snapshot/🦀️.rs`.

use crate::standards::v1::subsets::any::schema::snapshot::EquationExprSnapshot;
use crate::EquationSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

//#region 🔖️BinaryPrimitives
/// 🔢️ Pack format 2 — format 1 carried no scene (see `write_scene`).
const PACK_BINARY_FORMAT: u8 = 2;

fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

fn write_equation(out: &mut Vec<u8>, e: &EquationExprSnapshot) {
    write_bytes_lp(out, pack::json::to_json_string(e).as_bytes());
}
fn read_equation(reader: &mut store::ByteReader<'_>) -> Result<EquationExprSnapshot, String> {
    let bytes = read_bytes_lp(reader)?;
    let text = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    pack::json::from_json_str(&text).map_err(|e| e.to_string())
}

/// 🕸️ The live `(graph, geometry)` behind the composed-child triple, carried as two length-prefixed
/// first-party JSON blocks — the same "real codec, minimal grammar" trade `equation` above already
/// makes for its own opaque payload.
///
/// 🐛️ Format 1 wrote the three children as BARE HANDLES and nothing else, so the scene — which lives
/// only in the handles' `EquationWorkingScene` local owner — did not survive a single round trip.
/// `🕸️dag`'s own snapshot module states the rule that was being broken: "A codec that persisted only
/// the bare handle would produce an UNRECOVERABLE snapshot the instant a fresh process parses it."
/// It made the committed `🎬️demo` asset carry no graph at all, which is why the mathematical play
/// pane rendered an empty grid and a bare cursor (ticket 26/09/19, `📓️knowledge.md` §4).
fn write_scene(out: &mut Vec<u8>, scene: &crate::EquationWorkingScene) {
    write_bytes_lp(out, pack::json::to_json_string(&scene.graph).as_bytes());
    write_bytes_lp(out, pack::json::to_json_string(&scene.geometry).as_bytes());
}
fn read_scene(reader: &mut store::ByteReader<'_>) -> Result<crate::EquationWorkingScene, String> {
    let graph_text = String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())?;
    let geometry_text = String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())?;
    Ok(crate::EquationWorkingScene {
        graph: pack::json::from_json_str(&graph_text).map_err(|e| e.to_string())?,
        geometry: pack::json::from_json_str(&geometry_text).map_err(|e| e.to_string())?,
    })
}

fn encode_equation_snapshot_binary(s: &EquationSnapshot) -> Vec<u8> {
    let mut out = vec![PACK_BINARY_FORMAT];
    write_child(&mut out, &s.notation);
    write_child(&mut out, &s.results);
    write_child(&mut out, &s.computed);
    write_equation(&mut out, &s.equation);
    write_scene(&mut out, &crate::equation_scene(s));
    out
}
fn decode_equation_snapshot_binary(bytes: &[u8]) -> Result<EquationSnapshot, String> {
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let (notation, results, computed, equation) = (read_child(&mut reader)?, read_child(&mut reader)?, read_child(&mut reader)?, read_equation(&mut reader)?);
    // 🏗️ The decoded scene is attached to the exact handles this pack names, never to freshly minted
    // ones: `equation_children_from_state` derives the same ids, but re-minting would silently
    // discard whatever identity the pack actually carried.
    let owner = std::sync::Arc::new(read_scene(&mut reader)?);
    Ok(EquationSnapshot {
        notation: notation.with_local_owner(owner.clone()),
        results: results.with_local_owner(owner.clone()),
        computed: computed.with_local_owner(owner),
        equation,
    })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactPack
/// ✉️ P6 handcrafted `ArtifactPack`, real LEB128 binary primitives — moved here verbatim from
/// `🧬️schema/📸️snapshot/🦀️.rs`.
impl store::ArtifactPack for EquationSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let _ = options;
        let raw = encode_equation_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let _ = options;
        decode_equation_snapshot_binary(&inner).map_err(PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactPack

/// 📦️ Encodes a `EquationSnapshot` to its binary pack form.
pub fn encode(snapshot: &EquationSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

/// 📖️ Decodes a `EquationSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<EquationSnapshot, PackError> {
    <EquationSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🔖️Store
pub type EquationEnvelope = store::ArtifactEnvelope<crate::EquationSnapshot, crate::schema::mutations::EquationMutation>;
pub type EquationStore = store::ArtifactStore<crate::EquationSnapshot, crate::schema::mutations::EquationMutation>;

/// 🔐️ Opens a Equation store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `EquationStore::new` can be
/// read but never mutated, undone or closed. The app installs the same catalog through
/// `build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_equation_store(envelope: EquationEnvelope) -> Result<OwnedEquationStore, store::VcsError> {
    let mut store = EquationStore::new(envelope).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<crate::EquationSnapshot, crate::schema::mutations::EquationMutation>());
    Ok(OwnedEquationStore(store))
}

/// 🔚 A standalone Equation store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedEquationStore(EquationStore);

impl OwnedEquationStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Equation document store closes through its exact bounded owners");
        }
    }
}

impl std::ops::Deref for OwnedEquationStore {
    type Target = EquationStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedEquationStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedEquationStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
