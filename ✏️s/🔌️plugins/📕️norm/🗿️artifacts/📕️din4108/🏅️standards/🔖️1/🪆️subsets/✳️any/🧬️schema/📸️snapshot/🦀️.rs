//! 🧬️ Din4108 snapshot schema — artifact-lane fields only.

use crate::artifacts::din4108::LayerDocument;
use crate::document::ClimateZoneDe;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.din4108", layout = "lines")]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Snapshot {
    #[state(artifact)]
    pub category: String,
    #[dsl(table)]
    #[state(artifact)]
    pub layers: Vec<LayerDocument>,
    #[state(artifact)]
    pub climate: ClimateZoneDe,
    #[state(artifact)]
    pub airtightness_n50: f64,
    #[state(artifact)]
    pub psi_times_l_sum: f64,
    #[state(artifact)]
    pub rh_int: f64,
    #[state(artifact)]
    pub catalog_id: String,
    #[state(artifact)]
    pub material_id: String,
    #[state(artifact)]
    pub airtightness_class: String,
    #[state(artifact)]
    pub t_int_c: f64,
    #[state(artifact)]
    pub solar_absorptance: f64,
    #[state(artifact)]
    pub irradiance_w_m2: f64,
    #[state(artifact)]
    pub moisture_mu_exterior: f64,
    #[state(artifact)]
    pub moisture_mu_interior: f64,
    #[dsl(unit = "m2")]
    #[state(artifact)]
    pub envelope_area_m2: f64,
    #[state(artifact)]
    pub bb2_details_conform: bool,
    #[state(artifact)]
    pub application_type: String,
    #[state(artifact)]
    pub declared_application_class: String,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(Din4108Snapshot, extension = "din4108", envelope_id = "norm.din4108");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Din4108Snapshot {
    fn default() -> Self {
        Self {
            category: "residential".into(),
            layers: vec![LayerDocument { thickness_m: 0.24, lambda_w_mk: 0.81 }, LayerDocument { thickness_m: 0.14, lambda_w_mk: 0.035 }],
            climate: ClimateZoneDe::Zone2,
            airtightness_n50: 2.5,
            psi_times_l_sum: 0.02,
            rh_int: 0.5,
            catalog_id: "AW-01".into(),
            material_id: "mineral_wool".into(),
            airtightness_class: "class2".into(),
            t_int_c: 20.0,
            solar_absorptance: 0.6,
            irradiance_w_m2: 600.0,
            moisture_mu_exterior: 15.0,
            moisture_mu_interior: 1.3,
            envelope_area_m2: 100.0,
            bb2_details_conform: true,
            application_type: "DEO".into(),
            declared_application_class: "dk".into(),
        }
    }
}
//#endregion 🔖️Snapshot


//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`Din4108Snapshot`] — the surface
/// `../../../../../🧪️tests/mutate-din4108-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din4108_snapshot_json(snapshot: &Din4108Snapshot) -> String {
    serde_json::to_string(snapshot).expect("Din4108Snapshot serialization is infallible")
}

/// 📥️ The `serde_json` inverse of [`encode_din4108_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`Din4108Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din4108_snapshot_json(text: &str) -> Result<Din4108Snapshot, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`Din4108Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din4108_dsl(text: &str) -> Result<Din4108Snapshot, String> {
    <Din4108Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`Din4108Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din4108_dsl(snapshot: &Din4108Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`Din4108Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din4108_pack(bytes: &[u8]) -> Result<Din4108Snapshot, String> {
    <Din4108Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`Din4108Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din4108_pack(snapshot: &Din4108Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
