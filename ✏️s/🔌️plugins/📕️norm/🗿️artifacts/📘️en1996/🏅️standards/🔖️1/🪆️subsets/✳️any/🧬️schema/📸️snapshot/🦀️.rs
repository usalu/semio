//! 🧱️ EN 1996 snapshot schema — artifact-lane fields only.

use crate::artifacts::en1996::{part_2, MasonryClass};
use crate::document::{AnnexChoice, DesignSituation};
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1996 document snapshot.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1996", layout = "lines")]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Snapshot {
    #[state(artifact)]
    pub m_ed_knm: f64,
    #[state(artifact)]
    pub n_ed_kn: f64,
    #[state(artifact)]
    pub v_ed_kn: f64,
    #[state(artifact)]
    pub h_ed_kn: f64,
    #[state(artifact)]
    pub z_mm3: f64,
    #[state(artifact)]
    pub area_mm2: f64,
    #[state(artifact)]
    pub shear_area_mm2: f64,
    #[state(artifact)]
    pub f_k_mpa: f64,
    #[state(artifact)]
    pub f_vk_mpa: f64,
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub masonry_class: MasonryClass,
    #[state(artifact)]
    pub design_situation: DesignSituation,
    #[state(artifact)]
    pub mu: f64,
    #[state(artifact)]
    pub wall_thickness_mm: f64,
    #[state(artifact)]
    pub fire_resistance_min: u32,
    #[state(artifact)]
    pub unit: String,
    #[state(artifact)]
    pub exposure: part_2::ExposureClass,
    #[state(artifact)]
    pub mortar: part_2::MortarClass,
    #[state(artifact)]
    pub bed_joint_thickness_mm: f64,
    #[state(artifact)]
    pub storeys: u32,
    #[state(artifact)]
    pub h_ef_mm: f64,
    #[state(artifact)]
    pub t_ef_mm: f64,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1996Snapshot, extension = "en1996", envelope_id = "norm.en1996");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1996Snapshot {
    fn default() -> Self {
        Self {
            m_ed_knm: 8.0,
            n_ed_kn: 200.0,
            v_ed_kn: 35.0,
            h_ed_kn: 20.0,
            z_mm3: 8_000_000.0,
            area_mm2: 500_000.0,
            shear_area_mm2: 300_000.0,
            f_k_mpa: 5.0,
            f_vk_mpa: 0.15,
            annex: AnnexChoice::De,
            masonry_class: MasonryClass::default(),
            design_situation: DesignSituation::Persistent,
            mu: 0.4,
            wall_thickness_mm: 240.0,
            fire_resistance_min: 60,
            unit: "clay".into(),
            exposure: part_2::ExposureClass::Mx1,
            mortar: part_2::MortarClass::M5,
            bed_joint_thickness_mm: 12.0,
            storeys: 2,
            h_ef_mm: 2500.0,
            t_ef_mm: 240.0,
        }
    }
}


//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`En1996Snapshot`] — the surface
/// `../../../../../🧪️tests/mutate-en1996-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1996_snapshot_json(snapshot: &En1996Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_en1996_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`En1996Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1996_snapshot_json(text: &str) -> Result<En1996Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`En1996Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1996_dsl(text: &str) -> Result<En1996Snapshot, String> {
    <En1996Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`En1996Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1996_dsl(snapshot: &En1996Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`En1996Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1996_pack(bytes: &[u8]) -> Result<En1996Snapshot, String> {
    <En1996Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`En1996Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1996_pack(snapshot: &En1996Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
