//! 🧬️ En1994 snapshot schema — artifact-lane fields only.

use crate::document::AnnexChoice;
use schema::ArtifactSchema;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1994", layout = "lines")]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub m_ed_knm: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[state(artifact)]
    pub m_pla: f64,
    #[state(artifact)]
    pub m_pl_rd: f64,
    #[state(artifact)]
    pub eta: f64,
    #[state(artifact)]
    pub v_l_rd: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub insulation_thickness_mm: f64,
    #[state(artifact)]
    pub fire_rating: String,
    #[state(artifact)]
    pub deck_type: String,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub delta_sigma_mpa: f64,
    #[state(artifact)]
    pub fatigue_detail: String,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub d_mm: f64,
    #[dsl(unit = "mm")]
    #[state(artifact)]
    pub h_sc_mm: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub f_ck_mpa: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub f_u_mpa: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub e_cm_mpa: f64,
    #[dsl(unit = "kN")]
    #[state(artifact)]
    pub v_ed_per_stud_kn: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub span_m: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub f_y_mpa: f64,
    #[state(artifact)]
    pub n_cycles_stud: f64,
    #[dsl(unit = "MPa")]
    #[state(artifact)]
    pub delta_tau_stud_mpa: f64,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1994Snapshot, extension = "en1994", envelope_id = "norm.en1994");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1994Snapshot {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            m_ed_knm: 200.0,
            v_ed_kn: 120.0,
            m_pla: 80.0,
            m_pl_rd: 250.0,
            eta: 0.75,
            v_l_rd: 150.0,
            insulation_thickness_mm: 20.0,
            fire_rating: "r60".into(),
            deck_type: "trapezoidal".into(),
            delta_sigma_mpa: 65.0,
            fatigue_detail: "stud_welded".into(),
            d_mm: 19.0,
            h_sc_mm: 95.0,
            f_ck_mpa: 30.0,
            f_u_mpa: 450.0,
            e_cm_mpa: 33_000.0,
            v_ed_per_stud_kn: 40.0,
            span_m: 8.0,
            f_y_mpa: 355.0,
            n_cycles_stud: 2_000_000.0,
            delta_tau_stud_mpa: 40.0,
        }
    }
}
//#endregion 🔖️Snapshot


//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`En1994Snapshot`] — the surface
/// `../../../../../🧪️tests/🧭️mutate-en1994-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1994_snapshot_json(snapshot: &En1994Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_en1994_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`En1994Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1994_snapshot_json(text: &str) -> Result<En1994Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`En1994Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1994_dsl(text: &str) -> Result<En1994Snapshot, String> {
    <En1994Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`En1994Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1994_dsl(snapshot: &En1994Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`En1994Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1994_pack(bytes: &[u8]) -> Result<En1994Snapshot, String> {
    <En1994Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`En1994Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1994_pack(snapshot: &En1994Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
