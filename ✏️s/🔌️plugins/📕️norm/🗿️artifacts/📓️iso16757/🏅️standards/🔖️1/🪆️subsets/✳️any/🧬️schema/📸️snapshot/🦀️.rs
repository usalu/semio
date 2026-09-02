//! 🧬️ Iso16757 snapshot schema — artifact-lane fields only.

use crate::artifacts::iso16757::{part_1, part_2, part_4, part_5, CatalogueValue};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.iso16757", layout = "lines")]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Snapshot {
    #[state(artifact)]
    pub catalogue: part_1::Catalogue,
    #[state(artifact)]
    pub dictionary: part_4::Dictionary,
    #[state(artifact)]
    pub geometry: part_2::GeometryCatalogue,
    #[state(artifact)]
    pub selection: part_1::SelectionRequest,
    #[state(artifact)]
    pub part_number_rule: part_5::PartNumberRule,
    #[state(artifact)]
    pub part_number_inputs: BTreeMap<String, CatalogueValue>,
    #[state(artifact)]
    pub script_limits: part_5::ScriptLimits,
    #[state(artifact)]
    pub exchange_process: part_5::ExchangeProcess,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(Iso16757Snapshot, extension = "iso16757", envelope_id = "norm.iso16757");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Iso16757Snapshot {
    fn default() -> Self {
        Self::reference_fixture()
    }
}
//#endregion 🔖️Snapshot


//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`Iso16757Snapshot`] — the surface
/// `../../../../../🧪️tests/mutate-iso16757-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_iso16757_snapshot_json(snapshot: &Iso16757Snapshot) -> String {
    serde_json::to_string(snapshot).expect("Iso16757Snapshot serialization is infallible")
}

/// 📥️ The `serde_json` inverse of [`encode_iso16757_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`Iso16757Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_iso16757_snapshot_json(text: &str) -> Result<Iso16757Snapshot, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`Iso16757Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_iso16757_dsl(text: &str) -> Result<Iso16757Snapshot, String> {
    <Iso16757Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`Iso16757Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_iso16757_dsl(snapshot: &Iso16757Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`Iso16757Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_iso16757_pack(bytes: &[u8]) -> Result<Iso16757Snapshot, String> {
    <Iso16757Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`Iso16757Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_iso16757_pack(snapshot: &Iso16757Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
