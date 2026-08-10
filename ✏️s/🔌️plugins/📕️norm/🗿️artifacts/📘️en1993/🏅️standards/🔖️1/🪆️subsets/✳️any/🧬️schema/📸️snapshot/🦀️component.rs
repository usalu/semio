//! 🧬️ En1993 snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1993", layout = "lines")]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Snapshot {
    #[state(persistent)]
    pub annex: AnnexChoice,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub n_ed_kn: f64,
    #[state(persistent)]
    pub m_ed_knm: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub v_ed_kn: f64,
    #[dsl(unit = "mm2")]
    #[state(persistent)]
    pub a_mm2: f64,
    #[dsl(unit = "mm2")]
    #[state(persistent)]
    pub a_v_mm2: f64,
    #[state(persistent)]
    pub w_pl_mm3: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub f_y_mpa: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub f_u_mpa: f64,
    #[state(persistent)]
    pub chi: f64,
    #[dsl(unit = "mm2")]
    #[state(persistent)]
    pub a_net_mm2: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub tension_n_ed_kn: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub fire_thickness_mm: f64,
    #[state(persistent)]
    pub fire_rating: String,
    #[state(persistent)]
    pub fire_massivity: f64,
    #[state(persistent)]
    pub fire_mu_0: f64,
    #[state(persistent)]
    pub fire_design_temperature_c: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub cf_b_bar_mm: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub cf_t_mm: f64,
    #[state(persistent)]
    pub cf_k_sigma: f64,
    #[state(persistent)]
    pub cf_psi: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub cf_n_ed_kn: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub cf_gross_resistance_kn: f64,
    #[state(persistent)]
    pub stainless_m_ed_knm: f64,
    #[state(persistent)]
    pub stainless_w_pl_mm3: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub stainless_f_y_mpa: f64,
    #[state(persistent)]
    pub plated_lambda_p: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub plated_sigma_ed_mpa: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub silo_t_mm: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub silo_r_mm: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub shell_sigma_x_ed_mpa: f64,
    #[state(persistent)]
    pub silo_k: f64,
    #[state(persistent)]
    pub silo_gamma_kn_m3: f64,
    #[dsl(unit = "m")]
    #[state(persistent)]
    pub silo_depth_m: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub bolt_f_ed_kn: f64,
    #[state(persistent)]
    pub bolt_n_bolts: u32,
    #[dsl(unit = "mm2")]
    #[state(persistent)]
    pub bolt_a_s_mm2: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub bolt_e1_mm: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub bolt_e2_mm: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub bolt_d0_mm: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub bolt_d_mm: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub bolt_t_mm: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub bolt_f_u_mpa: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub bolt_f_ub_mpa: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub weld_a_mm: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub weld_l_mm: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub weld_f_u_mpa: f64,
    #[state(persistent)]
    pub weld_steel_grade: String,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub weld_f_ed_kn: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub delta_sigma_mpa: f64,
    #[state(persistent)]
    pub fatigue_category: u8,
    #[state(persistent)]
    pub fatigue_method: String,
    #[state(persistent)]
    pub t10_steel_subgrade: String,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub t10_actual_thickness_mm: f64,
    #[state(persistent)]
    pub t10_t_ed_c: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub tension_component_f_uk_kn: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub tension_component_f_k_kn: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub tension_component_n_ed_kn: f64,
    #[state(persistent)]
    pub hss_w_el_mm3: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub hss_f_y_mpa: f64,
    #[state(persistent)]
    pub hss_section_class: u8,
    #[state(persistent)]
    pub hss_m_ed_knm: f64,
    #[state(persistent)]
    pub bridge_lambda: f64,
    #[state(persistent)]
    pub bridge_phi_2: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub bridge_delta_sigma_p_mpa: f64,
    #[state(persistent)]
    pub tower_wind_factor: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub tower_n_ed_kn: f64,
    #[dsl(unit = "MPa")]
    #[state(persistent)]
    pub pile_sigma_mpa: f64,
    #[state(persistent)]
    pub pile_k_red: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub pile_n_ed_kn: f64,
    #[dsl(unit = "kN")]
    #[state(persistent)]
    pub crane_f_z_ed_kn: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub crane_wheel_contact_length_mm: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub crane_dispersion_mm: f64,
    #[dsl(unit = "mm")]
    #[state(persistent)]
    pub crane_t_w_mm: f64,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted En1993SnapshotDsl/En1993SnapshotPack (derive no longer emits these traits).
impl store::ArtifactDsl for En1993Snapshot {
    const EXTENSION: &'static str = "en1993";
    fn envelope_id() -> &'static str { "norm.en1993" }
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
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for En1993Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1993Snapshot {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            n_ed_kn: 500.0,
            m_ed_knm: 150.0,
            v_ed_kn: 80.0,
            a_mm2: 5000.0,
            a_v_mm2: 2500.0,
            w_pl_mm3: 500_000.0,
            f_y_mpa: 355.0,
            f_u_mpa: 510.0,
            chi: 0.75,
            a_net_mm2: 4250.0,
            tension_n_ed_kn: 400.0,
            fire_thickness_mm: 20.0,
            fire_rating: "r60".into(),
            fire_massivity: 150.0,
            fire_mu_0: 0.5,
            fire_design_temperature_c: 550.0,
            cf_b_bar_mm: 90.0,
            cf_t_mm: 2.0,
            cf_k_sigma: 4.0,
            cf_psi: 1.0,
            cf_n_ed_kn: 20.0,
            cf_gross_resistance_kn: 50.0,
            stainless_m_ed_knm: 40.0,
            stainless_w_pl_mm3: 300_000.0,
            stainless_f_y_mpa: 220.0,
            plated_lambda_p: 0.8,
            plated_sigma_ed_mpa: 200.0,
            silo_t_mm: 8.0,
            silo_r_mm: 3000.0,
            shell_sigma_x_ed_mpa: 150.0,
            silo_k: 0.4,
            silo_gamma_kn_m3: 18.0,
            silo_depth_m: 5.0,
            bolt_f_ed_kn: 120.0,
            bolt_n_bolts: 2,
            bolt_a_s_mm2: 245.0,
            bolt_e1_mm: 40.0,
            bolt_e2_mm: 40.0,
            bolt_d0_mm: 22.0,
            bolt_d_mm: 20.0,
            bolt_t_mm: 10.0,
            bolt_f_u_mpa: 510.0,
            bolt_f_ub_mpa: 800.0,
            weld_a_mm: 5.0,
            weld_l_mm: 100.0,
            weld_f_u_mpa: 510.0,
            weld_steel_grade: "S355".into(),
            weld_f_ed_kn: 80.0,
            delta_sigma_mpa: 50.0,
            fatigue_category: 71,
            fatigue_method: "damage_tolerant".into(),
            t10_steel_subgrade: "J2".into(),
            t10_actual_thickness_mm: 25.0,
            t10_t_ed_c: 0.0,
            tension_component_f_uk_kn: 500.0,
            tension_component_f_k_kn: 350.0,
            tension_component_n_ed_kn: 250.0,
            hss_w_el_mm3: 400_000.0,
            hss_f_y_mpa: 460.0,
            hss_section_class: 2,
            hss_m_ed_knm: 100.0,
            bridge_lambda: 1.0,
            bridge_phi_2: 1.0,
            bridge_delta_sigma_p_mpa: 30.0,
            tower_wind_factor: 1.1,
            tower_n_ed_kn: 300.0,
            pile_sigma_mpa: 280.0,
            pile_k_red: 0.85,
            pile_n_ed_kn: 400.0,
            crane_f_z_ed_kn: 50.0,
            crane_wheel_contact_length_mm: 100.0,
            crane_dispersion_mm: 50.0,
            crane_t_w_mm: 10.0,
        }
    }
}
//#endregion 🔖️Snapshot
