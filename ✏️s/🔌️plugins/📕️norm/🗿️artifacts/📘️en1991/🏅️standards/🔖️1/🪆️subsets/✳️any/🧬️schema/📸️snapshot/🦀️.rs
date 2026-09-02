//! 🧬️ En1991 snapshot schema — artifact-lane fields only.

use crate::document::{AnnexChoice, ImposedCategory};
use schema::ArtifactSchema;

//#region 🔖️Snapshot
pub mod part_1_2 {
    pub use crate::artifacts::en1991::part_1_2::FireCurve;
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1991", layout = "lines")]
#[artifact_schema(id = "s.norm.en1991")]
pub struct En1991Snapshot {
    #[dsl(unit = "m2")]
    #[state(artifact)]
    pub area_m2: f64,
    #[state(artifact)]
    pub category: ImposedCategory,
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub self_weight_material: String,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub self_weight_thickness_m: f64,
    #[dsl(unit = "kN/m2")]
    #[state(artifact)]
    pub assumed_g_k_kn_m2: f64,
    #[state(artifact)]
    pub fire_curve: part_1_2::FireCurve,
    #[state(artifact)]
    pub fire_resistance_min: f64,
    #[state(artifact)]
    pub fire_member_capacity_c: f64,
    #[state(artifact)]
    pub snow_zone: u8,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub snow_altitude_m: f64,
    #[dsl(unit = "kN/m2")]
    #[state(artifact)]
    pub en_s_k_kn_m2: f64,
    #[state(artifact)]
    pub wind_zone: u8,
    #[dsl(unit = "m/s")]
    #[state(artifact)]
    pub en_v_b_m_s: f64,
    #[dsl(unit = "K")]
    #[state(artifact)]
    pub delta_t_k: f64,
    #[state(artifact)]
    pub construction_activity: String,
    #[dsl(unit = "t")]
    #[state(artifact)]
    pub accidental_mass_t: f64,
    #[state(artifact)]
    pub accidental_speed_km_h: f64,
    #[state(artifact)]
    pub bridge_lane: u8,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub bridge_span_m: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub bridge_lane_width_m: f64,
    #[state(artifact)]
    pub bridge_moment_resistance_knm: f64,
    #[state(artifact)]
    pub crane_class: String,
    #[state(artifact)]
    pub hoist_class: String,
    #[dsl(unit = "m/s")]
    #[state(artifact)]
    pub hoisting_speed_m_s: f64,
    #[state(artifact)]
    pub silo_bulk_density_kn_m3: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub silo_height_m: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub silo_hydraulic_radius_m: f64,
    #[state(artifact)]
    pub silo_mu: f64,
    #[state(artifact)]
    pub silo_k: f64,
    #[state(artifact)]
    pub c_s: f64,
    #[state(artifact)]
    pub c_d: f64,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(En1991Snapshot, extension = "en1991", envelope_id = "norm.en1991");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1991Snapshot {
    fn default() -> Self {
        Self {
            area_m2: 50.0,
            category: ImposedCategory::B,
            annex: AnnexChoice::De,
            self_weight_material: "reinforced_concrete".into(),
            self_weight_thickness_m: 0.2,
            assumed_g_k_kn_m2: 6.0,
            fire_curve: part_1_2::FireCurve::Standard,
            fire_resistance_min: 30.0,
            fire_member_capacity_c: 900.0,
            snow_zone: 2,
            snow_altitude_m: 150.0,
            en_s_k_kn_m2: 0.85,
            wind_zone: 2,
            en_v_b_m_s: 25.0,
            delta_t_k: 30.0,
            construction_activity: "scaffolding".into(),
            accidental_mass_t: 30.0,
            accidental_speed_km_h: 80.0,
            bridge_lane: 1,
            bridge_span_m: 20.0,
            bridge_lane_width_m: 3.0,
            bridge_moment_resistance_knm: 3000.0,
            crane_class: "HC2".into(),
            hoist_class: "HC2".into(),
            hoisting_speed_m_s: 0.5,
            silo_bulk_density_kn_m3: 8.0,
            silo_height_m: 12.0,
            silo_hydraulic_radius_m: 1.5,
            silo_mu: 0.4,
            silo_k: 0.4,
            c_s: 1.0,
            c_d: 1.0,
        }
    }
}
//#endregion 🔖️Snapshot


//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`En1991Snapshot`] — the surface
/// `../../../../../🧪️tests/mutate-en1991-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1991_snapshot_json(snapshot: &En1991Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_en1991_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`En1991Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1991_snapshot_json(text: &str) -> Result<En1991Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`En1991Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1991_dsl(text: &str) -> Result<En1991Snapshot, String> {
    <En1991Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`En1991Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1991_dsl(snapshot: &En1991Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`En1991Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1991_pack(bytes: &[u8]) -> Result<En1991Snapshot, String> {
    <En1991Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`En1991Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1991_pack(snapshot: &En1991Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
