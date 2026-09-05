//! 🧬️ Din16798 snapshot schema — artifact-lane fields only.

use crate::document::AnnexChoice;
use schema::ArtifactSchema;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.din16798", layout = "lines")]
#[artifact_schema(id = "s.norm.din16798")]
pub struct Din16798Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub occupancy: String,
    #[state(artifact)]
    pub comfort_category: String,
    #[state(artifact)]
    pub t_op_c: f64,
    #[dsl(unit = "pct")]
    #[state(artifact)]
    pub rh_percent: f64,
    #[dsl(unit = "m/s")]
    #[state(artifact)]
    pub air_speed_m_s: f64,
    #[state(artifact)]
    pub theta_rm_c: f64,
    #[state(artifact)]
    pub co2_ppm: f64,
    #[dsl(unit = "pct")]
    #[state(artifact)]
    pub df_percent: f64,
    #[state(artifact)]
    pub l_aeq_db: f64,

    #[state(artifact)]
    pub persons: u32,
    // Not `#[dsl(ident)]`: values like `"2"` are bare digits, which the lexer always tokenizes as
    // an integer, never as an identifier — quoted `Text` (the default String shape) has no such
    // ambiguity.
    #[state(artifact)]
    pub ida_class: String,
    #[state(artifact)]
    pub ventilation_m3_h: f64,
    #[dsl(unit = "m2")]
    #[state(artifact)]
    pub floor_area_m2: f64,
    #[state(artifact)]
    pub bedrooms: u32,
    #[state(artifact)]
    pub dwelling_ventilation_m3_h: f64,
    #[state(artifact)]
    pub occupants: u32,
    #[state(artifact)]
    pub residential_ventilation_m3_h: f64,
    #[state(artifact)]
    pub sfp_w_m3_s: f64,
    #[state(artifact)]
    pub sfp_required_class: u8,
    #[state(artifact)]
    pub heat_recovery_eta: f64,
    #[state(artifact)]
    pub heat_recovery_eta_min: f64,
    #[state(artifact)]
    pub system_type: String,
    #[state(artifact)]
    pub years_since_inspection: u32,
    #[state(artifact)]
    pub humidification_required_kg_h: f64,
    #[state(artifact)]
    pub humidification_provided_kg_h: f64,

    #[state(artifact)]
    pub fan_q_v_m3_s: f64,
    #[dsl(unit = "h")]
    #[state(artifact)]
    pub fan_t_run_h: f64,
    #[state(artifact)]
    pub fan_energy_reference_kwh: f64,
    #[dsl(unit = "K")]
    #[state(artifact)]
    pub night_setback_k: f64,

    #[state(artifact)]
    pub hr_m_dot_kg_s: f64,
    #[state(artifact)]
    pub hr_cp_j_kgk: f64,
    #[state(artifact)]
    pub hr_delta_t_c: f64,
    #[dsl(unit = "h")]
    #[state(artifact)]
    pub hr_t_h: f64,
    #[state(artifact)]
    pub hr_savings_reference_kwh: f64,

    #[state(artifact)]
    pub n50_h_inv: f64,
    #[dsl(unit = "m3")]
    #[state(artifact)]
    pub volume_m3: f64,
    #[state(artifact)]
    pub infiltration_allowance_m3_h: f64,
    #[dsl(unit = "m2")]
    #[state(artifact)]
    pub cellar_area_m2: f64,
    #[state(artifact)]
    pub cellar_ventilation_m3_h: f64,

    #[state(artifact)]
    pub h_tr_w_k: f64,
    #[state(artifact)]
    pub h_ve_w_k: f64,
    #[state(artifact)]
    pub theta_e_c: f64,
    #[state(artifact)]
    pub theta_set_c: f64,
    #[state(artifact)]
    pub cooling_delta_t_h: f64,
    #[state(artifact)]
    pub cooling_gains_kwh: f64,
    #[state(artifact)]
    pub cooling_utilization_factor: f64,
    #[state(artifact)]
    pub cooling_reference_kwh: f64,

    #[state(artifact)]
    pub chiller_type: String,
    #[state(artifact)]
    pub eer_actual: f64,
    #[state(artifact)]
    pub q_c_kwh: f64,
    #[state(artifact)]
    pub generation_reference_kwh: f64,
    #[state(artifact)]
    pub data_center_supply_c: f64,

    #[state(artifact)]
    pub h_st_w_k: f64,
    #[state(artifact)]
    pub theta_st_c: f64,
    #[state(artifact)]
    pub theta_amb_c: f64,
    #[dsl(unit = "h")]
    #[state(artifact)]
    pub storage_t_h: f64,
    #[state(artifact)]
    pub storage_allowance_kwh: f64,
    #[state(artifact)]
    pub dhw_delivery_c: f64,

    #[state(artifact)]
    pub duct_class: String,
    #[dsl(unit = "Pa")]
    #[state(artifact)]
    pub duct_test_pressure_pa: f64,
    #[state(artifact)]
    pub duct_leakage_m3_s_m2: f64,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).
crate::impl_norm_artifact_record!(Din16798Snapshot, extension = "din16798", envelope_id = "norm.din16798");
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Din16798Snapshot {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            occupancy: "residential".into(),
            comfort_category: "II".into(),
            t_op_c: 22.0,
            rh_percent: 50.0,
            air_speed_m_s: 0.1,
            theta_rm_c: 15.0,
            co2_ppm: 800.0,
            df_percent: 2.5,
            l_aeq_db: 24.0,

            persons: 10,
            ida_class: "2".into(),
            ventilation_m3_h: 280.0,
            floor_area_m2: 90.0,
            bedrooms: 3,
            dwelling_ventilation_m3_h: 63.0,
            occupants: 3,
            residential_ventilation_m3_h: 80.0,
            sfp_w_m3_s: 1500.0,
            sfp_required_class: 4,
            heat_recovery_eta: 0.75,
            heat_recovery_eta_min: 0.70,
            system_type: "central_mech".into(),
            years_since_inspection: 1,
            humidification_required_kg_h: 2.0,
            humidification_provided_kg_h: 2.0,

            fan_q_v_m3_s: 1.0,
            fan_t_run_h: 8.0,
            fan_energy_reference_kwh: 15.0,
            night_setback_k: 3.5,

            hr_m_dot_kg_s: 0.5,
            hr_cp_j_kgk: 1005.0,
            hr_delta_t_c: 15.0,
            hr_t_h: 10.0,
            hr_savings_reference_kwh: 50.0,

            n50_h_inv: 1.5,
            volume_m3: 500.0,
            infiltration_allowance_m3_h: 45.0,
            cellar_area_m2: 50.0,
            cellar_ventilation_m3_h: 15.0,

            h_tr_w_k: 200.0,
            h_ve_w_k: 100.0,
            theta_e_c: 32.0,
            theta_set_c: 26.0,
            cooling_delta_t_h: 10.0,
            cooling_gains_kwh: 5.0,
            cooling_utilization_factor: 0.8,
            cooling_reference_kwh: 20.0,

            chiller_type: "air_cooled".into(),
            eer_actual: 3.0,
            q_c_kwh: 1000.0,
            generation_reference_kwh: 400.0,
            data_center_supply_c: 22.0,

            h_st_w_k: 5.0,
            theta_st_c: 60.0,
            theta_amb_c: 20.0,
            storage_t_h: 24.0,
            storage_allowance_kwh: 6.0,
            dhw_delivery_c: 58.0,

            duct_class: "C".into(),
            duct_test_pressure_pa: 400.0,
            duct_leakage_m3_s_m2: 0.10,
        }
    }
}
//#endregion 🔖️Snapshot


//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`Din16798Snapshot`] — the surface
/// `../../../../../🧪️tests/🌬️mutate-din16798-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din16798_snapshot_json(snapshot: &Din16798Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_din16798_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`Din16798Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din16798_snapshot_json(text: &str) -> Result<Din16798Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`Din16798Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din16798_dsl(text: &str) -> Result<Din16798Snapshot, String> {
    <Din16798Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`Din16798Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din16798_dsl(snapshot: &Din16798Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`Din16798Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din16798_pack(bytes: &[u8]) -> Result<Din16798Snapshot, String> {
    <Din16798Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`Din16798Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din16798_pack(snapshot: &Din16798Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
