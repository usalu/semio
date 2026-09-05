//! 🪵️ EN 1995 snapshot schema — artifact-lane fields only.

use crate::document::AnnexChoice;
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1995 document snapshot.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1995", layout = "lines")]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub m_ed_knm: f64,
    #[state(artifact)]
    pub n_ed_kn: f64,
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[state(artifact)]
    pub w_mm3: f64,
    #[state(artifact)]
    pub a_mm2: f64,
    #[state(artifact)]
    pub b_mm: f64,
    #[state(artifact)]
    pub h_mm: f64,
    #[state(artifact)]
    pub f_m_k: f64,
    #[state(artifact)]
    pub f_c_0_k: f64,
    #[state(artifact)]
    pub service_class: String,
    #[state(artifact)]
    pub load_duration: String,
    #[state(artifact)]
    pub m_crit_knm: f64,
    #[state(artifact)]
    pub f_ed_kn: f64,
    #[state(artifact)]
    pub a_ef_mm2: f64,
    #[state(artifact)]
    pub f_v_k: f64,
    #[state(artifact)]
    pub fire_duration_min: f64,
    #[state(artifact)]
    pub section_depth_mm: f64,
    #[state(artifact)]
    pub a_vert_m_s2: f64,
    #[state(artifact)]
    pub n_cycles_bridge: f64,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1995Snapshot, extension = "en1995", envelope_id = "norm.en1995");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1995Snapshot {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            m_ed_knm: 25.0,
            n_ed_kn: 50.0,
            v_ed_kn: 15.0,
            w_mm3: 1_000_000.0,
            a_mm2: 20_000.0,
            b_mm: 200.0,
            h_mm: 300.0,
            f_m_k: 24.0,
            f_c_0_k: 21.0,
            service_class: "sc1".into(),
            load_duration: "medium".into(),
            m_crit_knm: 80.0,
            f_ed_kn: 18.0,
            a_ef_mm2: 12_000.0,
            f_v_k: 4.0,
            fire_duration_min: 30.0,
            section_depth_mm: 300.0,
            a_vert_m_s2: 0.3,
            n_cycles_bridge: 500_000.0,
        }
    }
}


//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`En1995Snapshot`] — the surface
/// `../../../../../🧪️tests/🟫️mutate-en1995-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1995_snapshot_json(snapshot: &En1995Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_en1995_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`En1995Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1995_snapshot_json(text: &str) -> Result<En1995Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`En1995Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1995_dsl(text: &str) -> Result<En1995Snapshot, String> {
    <En1995Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`En1995Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1995_dsl(snapshot: &En1995Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`En1995Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1995_pack(bytes: &[u8]) -> Result<En1995Snapshot, String> {
    <En1995Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`En1995Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1995_pack(snapshot: &En1995Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
