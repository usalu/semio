//! 🧬️ Din16798 snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.din16798", layout = "lines")]
#[artifact_schema(id = "s.norm.din16798")]
pub struct Din16798Snapshot {
    pub annex: AnnexChoice,
    pub occupancy: String,
    pub comfort_category: String,
    pub t_op_c: f64,
    #[dsl(unit = "pct")]
    pub rh_percent: f64,
    #[dsl(unit = "m/s")]
    pub air_speed_m_s: f64,
    pub theta_rm_c: f64,
    pub co2_ppm: f64,
    #[dsl(unit = "pct")]
    pub df_percent: f64,
    pub l_aeq_db: f64,

    pub persons: u32,
    // Not `#[dsl(ident)]`: values like `"2"` are bare digits, which the lexer always tokenizes as
    // an integer, never as an identifier — quoted `Text` (the default String shape) has no such
    // ambiguity.
    pub ida_class: String,
    pub ventilation_m3_h: f64,
    #[dsl(unit = "m2")]
    pub floor_area_m2: f64,
    pub bedrooms: u32,
    pub dwelling_ventilation_m3_h: f64,
    pub occupants: u32,
    pub residential_ventilation_m3_h: f64,
    pub sfp_w_m3_s: f64,
    pub sfp_required_class: u8,
    pub heat_recovery_eta: f64,
    pub heat_recovery_eta_min: f64,
    pub system_type: String,
    pub years_since_inspection: u32,
    pub humidification_required_kg_h: f64,
    pub humidification_provided_kg_h: f64,

    pub fan_q_v_m3_s: f64,
    #[dsl(unit = "h")]
    pub fan_t_run_h: f64,
    pub fan_energy_reference_kwh: f64,
    #[dsl(unit = "K")]
    pub night_setback_k: f64,

    pub hr_m_dot_kg_s: f64,
    pub hr_cp_j_kgk: f64,
    pub hr_delta_t_c: f64,
    #[dsl(unit = "h")]
    pub hr_t_h: f64,
    pub hr_savings_reference_kwh: f64,

    pub n50_h_inv: f64,
    #[dsl(unit = "m3")]
    pub volume_m3: f64,
    pub infiltration_allowance_m3_h: f64,
    #[dsl(unit = "m2")]
    pub cellar_area_m2: f64,
    pub cellar_ventilation_m3_h: f64,

    pub h_tr_w_k: f64,
    pub h_ve_w_k: f64,
    pub theta_e_c: f64,
    pub theta_set_c: f64,
    pub cooling_delta_t_h: f64,
    pub cooling_gains_kwh: f64,
    pub cooling_utilization_factor: f64,
    pub cooling_reference_kwh: f64,

    pub chiller_type: String,
    pub eer_actual: f64,
    pub q_c_kwh: f64,
    pub generation_reference_kwh: f64,
    pub data_center_supply_c: f64,

    pub h_st_w_k: f64,
    pub theta_st_c: f64,
    pub theta_amb_c: f64,
    #[dsl(unit = "h")]
    pub storage_t_h: f64,
    pub storage_allowance_kwh: f64,
    pub dhw_delivery_c: f64,

    pub duct_class: String,
    #[dsl(unit = "Pa")]
    pub duct_test_pressure_pa: f64,
    pub duct_leakage_m3_s_m2: f64,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for Din16798Snapshot {
    const EXTENSION: &'static str = "din16798";
    fn envelope_id() -> &'static str { "norm.din16798" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for Din16798Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs

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
