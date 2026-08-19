//! 🔧️ En1998 artifact — OpText/OpBinary codecs for `En1998Mutation`. Mutation apply/inverse
//! live in `🧬️mutations`; this facet only handcrafts the op wire forms (the shared
//! whole-document-replace macro no longer applies now that the whole-document-replace variant is
//! gone).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::en1998::schema::mutations::En1998Mutation;
use crate::artifacts::en1998::schema::mutations::{
    change_annex, change_bearing_d_ed_mm, change_bearing_d_rd_mm, change_bridge_v_rd_kn, change_drift_mm, change_en_a_gr, change_en_ground_type, change_en_spectrum_type, change_foundation_area_m2, change_foundation_h_ed_kn,
    change_foundation_h_rd_kn, change_foundation_p_rd_kpa, change_ground_type, change_height_m, change_importance_class, change_k_foundation, change_k_soil, change_mass_t, change_multiple_resisting_systems, change_period_ratio,
    change_retrofit_e_d_kn, change_retrofit_gamma_el, change_retrofit_knowledge_level, change_retrofit_limit_state, change_retrofit_r_k_kn, change_seismic_zone, change_silo_height_m, change_silo_n_rd_kn, change_silo_q_nominal, change_silo_radius_m,
    change_silo_v_ed_kn, change_silo_v_rd_kn, change_structural_system, change_t1_s, change_tank_height_m, change_tank_mass_t, change_tank_radius_m, change_tank_v_rd_kn, change_tower_is_chimney, change_tower_m_ed_knm, change_tower_m_rd_knm,
    change_tower_mass_t, change_tower_q_nominal, change_v_rd_kn, change_wall_h_rd_kn, change_wall_height_m, change_wall_phi_deg, change_wall_r, change_wall_soil_gamma_kn_m3,
};

use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `En1998Mutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only; `En1998Mutation` itself,
/// and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum En1998MutationDsl {
    ChangeSeismicZone { new_seismic_zone: u8 },
    ChangeGroundType { new_ground_type: String },
    ChangeImportanceClass { new_importance_class: String },
    ChangeStructuralSystem { new_structural_system: String },
    ChangeT1S { new_t1_s: f64 },
    ChangeMassT { new_mass_t: f64 },
    ChangeVRdKn { new_v_rd_kn: f64 },
    ChangeDriftMm { new_drift_mm: f64 },
    ChangeHeightM { new_height_m: f64 },
    ChangeMultipleResistingSystems { new_multiple_resisting_systems: bool },
    ChangeAnnex { new_annex: String },
    ChangeEnAGr { new_en_a_gr: f64 },
    ChangeEnGroundType { new_en_ground_type: String },
    ChangeEnSpectrumType { new_en_spectrum_type: String },
    ChangePeriodRatio { new_period_ratio: f64 },
    ChangeBridgeVRdKn { new_bridge_v_rd_kn: f64 },
    ChangeBearingDEdMm { new_bearing_d_ed_mm: f64 },
    ChangeBearingDRdMm { new_bearing_d_rd_mm: f64 },
    ChangeRetrofitKnowledgeLevel { new_retrofit_knowledge_level: String },
    ChangeRetrofitLimitState { new_retrofit_limit_state: String },
    ChangeRetrofitEDKn { new_retrofit_e_d_kn: f64 },
    ChangeRetrofitRKKn { new_retrofit_r_k_kn: f64 },
    ChangeRetrofitGammaEl { new_retrofit_gamma_el: f64 },
    ChangeSiloHeightM { new_silo_height_m: f64 },
    ChangeSiloRadiusM { new_silo_radius_m: f64 },
    ChangeSiloNRdKn { new_silo_n_rd_kn: f64 },
    ChangeSiloVEdKn { new_silo_v_ed_kn: f64 },
    ChangeSiloVRdKn { new_silo_v_rd_kn: f64 },
    ChangeSiloQNominal { new_silo_q_nominal: f64 },
    ChangeTankHeightM { new_tank_height_m: f64 },
    ChangeTankRadiusM { new_tank_radius_m: f64 },
    ChangeTankMassT { new_tank_mass_t: f64 },
    ChangeTankVRdKn { new_tank_v_rd_kn: f64 },
    ChangeTowerMEdKnm { new_tower_m_ed_knm: f64 },
    ChangeTowerMRdKnm { new_tower_m_rd_knm: f64 },
    ChangeTowerIsChimney { new_tower_is_chimney: bool },
    ChangeTowerQNominal { new_tower_q_nominal: f64 },
    ChangeTowerMassT { new_tower_mass_t: f64 },
    ChangeFoundationAreaM2 { new_foundation_area_m2: f64 },
    ChangeFoundationPRdKpa { new_foundation_p_rd_kpa: f64 },
    ChangeFoundationHEdKn { new_foundation_h_ed_kn: f64 },
    ChangeFoundationHRdKn { new_foundation_h_rd_kn: f64 },
    ChangeKFoundation { new_k_foundation: f64 },
    ChangeKSoil { new_k_soil: f64 },
    ChangeWallHeightM { new_wall_height_m: f64 },
    ChangeWallPhiDeg { new_wall_phi_deg: f64 },
    ChangeWallSoilGammaKnM3 { new_wall_soil_gamma_kn_m3: f64 },
    ChangeWallR { new_wall_r: f64 },
    ChangeWallHRdKn { new_wall_h_rd_kn: f64 },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for En1998MutationDsl {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for En1998MutationDsl {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

async fn en1998_mutation_to_dsl(mutation: &En1998Mutation) -> En1998MutationDsl {
    match mutation {
        En1998Mutation::ChangeSeismicZone(payload) => En1998MutationDsl::ChangeSeismicZone { new_seismic_zone: payload.new_seismic_zone.clone() },
        En1998Mutation::ChangeGroundType(payload) => En1998MutationDsl::ChangeGroundType { new_ground_type: payload.new_ground_type.clone() },
        En1998Mutation::ChangeImportanceClass(payload) => En1998MutationDsl::ChangeImportanceClass { new_importance_class: payload.new_importance_class.clone() },
        En1998Mutation::ChangeStructuralSystem(payload) => En1998MutationDsl::ChangeStructuralSystem { new_structural_system: payload.new_structural_system.clone() },
        En1998Mutation::ChangeT1S(payload) => En1998MutationDsl::ChangeT1S { new_t1_s: payload.new_t1_s.clone() },
        En1998Mutation::ChangeMassT(payload) => En1998MutationDsl::ChangeMassT { new_mass_t: payload.new_mass_t.clone() },
        En1998Mutation::ChangeVRdKn(payload) => En1998MutationDsl::ChangeVRdKn { new_v_rd_kn: payload.new_v_rd_kn.clone() },
        En1998Mutation::ChangeDriftMm(payload) => En1998MutationDsl::ChangeDriftMm { new_drift_mm: payload.new_drift_mm.clone() },
        En1998Mutation::ChangeHeightM(payload) => En1998MutationDsl::ChangeHeightM { new_height_m: payload.new_height_m.clone() },
        En1998Mutation::ChangeMultipleResistingSystems(payload) => En1998MutationDsl::ChangeMultipleResistingSystems { new_multiple_resisting_systems: payload.new_multiple_resisting_systems.clone() },
        En1998Mutation::ChangeAnnex(payload) => En1998MutationDsl::ChangeAnnex { new_annex: payload.new_annex.clone() },
        En1998Mutation::ChangeEnAGr(payload) => En1998MutationDsl::ChangeEnAGr { new_en_a_gr: payload.new_en_a_gr.clone() },
        En1998Mutation::ChangeEnGroundType(payload) => En1998MutationDsl::ChangeEnGroundType { new_en_ground_type: payload.new_en_ground_type.clone() },
        En1998Mutation::ChangeEnSpectrumType(payload) => En1998MutationDsl::ChangeEnSpectrumType { new_en_spectrum_type: payload.new_en_spectrum_type.clone() },
        En1998Mutation::ChangePeriodRatio(payload) => En1998MutationDsl::ChangePeriodRatio { new_period_ratio: payload.new_period_ratio.clone() },
        En1998Mutation::ChangeBridgeVRdKn(payload) => En1998MutationDsl::ChangeBridgeVRdKn { new_bridge_v_rd_kn: payload.new_bridge_v_rd_kn.clone() },
        En1998Mutation::ChangeBearingDEdMm(payload) => En1998MutationDsl::ChangeBearingDEdMm { new_bearing_d_ed_mm: payload.new_bearing_d_ed_mm.clone() },
        En1998Mutation::ChangeBearingDRdMm(payload) => En1998MutationDsl::ChangeBearingDRdMm { new_bearing_d_rd_mm: payload.new_bearing_d_rd_mm.clone() },
        En1998Mutation::ChangeRetrofitKnowledgeLevel(payload) => En1998MutationDsl::ChangeRetrofitKnowledgeLevel { new_retrofit_knowledge_level: payload.new_retrofit_knowledge_level.clone() },
        En1998Mutation::ChangeRetrofitLimitState(payload) => En1998MutationDsl::ChangeRetrofitLimitState { new_retrofit_limit_state: payload.new_retrofit_limit_state.clone() },
        En1998Mutation::ChangeRetrofitEDKn(payload) => En1998MutationDsl::ChangeRetrofitEDKn { new_retrofit_e_d_kn: payload.new_retrofit_e_d_kn.clone() },
        En1998Mutation::ChangeRetrofitRKKn(payload) => En1998MutationDsl::ChangeRetrofitRKKn { new_retrofit_r_k_kn: payload.new_retrofit_r_k_kn.clone() },
        En1998Mutation::ChangeRetrofitGammaEl(payload) => En1998MutationDsl::ChangeRetrofitGammaEl { new_retrofit_gamma_el: payload.new_retrofit_gamma_el.clone() },
        En1998Mutation::ChangeSiloHeightM(payload) => En1998MutationDsl::ChangeSiloHeightM { new_silo_height_m: payload.new_silo_height_m.clone() },
        En1998Mutation::ChangeSiloRadiusM(payload) => En1998MutationDsl::ChangeSiloRadiusM { new_silo_radius_m: payload.new_silo_radius_m.clone() },
        En1998Mutation::ChangeSiloNRdKn(payload) => En1998MutationDsl::ChangeSiloNRdKn { new_silo_n_rd_kn: payload.new_silo_n_rd_kn.clone() },
        En1998Mutation::ChangeSiloVEdKn(payload) => En1998MutationDsl::ChangeSiloVEdKn { new_silo_v_ed_kn: payload.new_silo_v_ed_kn.clone() },
        En1998Mutation::ChangeSiloVRdKn(payload) => En1998MutationDsl::ChangeSiloVRdKn { new_silo_v_rd_kn: payload.new_silo_v_rd_kn.clone() },
        En1998Mutation::ChangeSiloQNominal(payload) => En1998MutationDsl::ChangeSiloQNominal { new_silo_q_nominal: payload.new_silo_q_nominal.clone() },
        En1998Mutation::ChangeTankHeightM(payload) => En1998MutationDsl::ChangeTankHeightM { new_tank_height_m: payload.new_tank_height_m.clone() },
        En1998Mutation::ChangeTankRadiusM(payload) => En1998MutationDsl::ChangeTankRadiusM { new_tank_radius_m: payload.new_tank_radius_m.clone() },
        En1998Mutation::ChangeTankMassT(payload) => En1998MutationDsl::ChangeTankMassT { new_tank_mass_t: payload.new_tank_mass_t.clone() },
        En1998Mutation::ChangeTankVRdKn(payload) => En1998MutationDsl::ChangeTankVRdKn { new_tank_v_rd_kn: payload.new_tank_v_rd_kn.clone() },
        En1998Mutation::ChangeTowerMEdKnm(payload) => En1998MutationDsl::ChangeTowerMEdKnm { new_tower_m_ed_knm: payload.new_tower_m_ed_knm.clone() },
        En1998Mutation::ChangeTowerMRdKnm(payload) => En1998MutationDsl::ChangeTowerMRdKnm { new_tower_m_rd_knm: payload.new_tower_m_rd_knm.clone() },
        En1998Mutation::ChangeTowerIsChimney(payload) => En1998MutationDsl::ChangeTowerIsChimney { new_tower_is_chimney: payload.new_tower_is_chimney.clone() },
        En1998Mutation::ChangeTowerQNominal(payload) => En1998MutationDsl::ChangeTowerQNominal { new_tower_q_nominal: payload.new_tower_q_nominal.clone() },
        En1998Mutation::ChangeTowerMassT(payload) => En1998MutationDsl::ChangeTowerMassT { new_tower_mass_t: payload.new_tower_mass_t.clone() },
        En1998Mutation::ChangeFoundationAreaM2(payload) => En1998MutationDsl::ChangeFoundationAreaM2 { new_foundation_area_m2: payload.new_foundation_area_m2.clone() },
        En1998Mutation::ChangeFoundationPRdKpa(payload) => En1998MutationDsl::ChangeFoundationPRdKpa { new_foundation_p_rd_kpa: payload.new_foundation_p_rd_kpa.clone() },
        En1998Mutation::ChangeFoundationHEdKn(payload) => En1998MutationDsl::ChangeFoundationHEdKn { new_foundation_h_ed_kn: payload.new_foundation_h_ed_kn.clone() },
        En1998Mutation::ChangeFoundationHRdKn(payload) => En1998MutationDsl::ChangeFoundationHRdKn { new_foundation_h_rd_kn: payload.new_foundation_h_rd_kn.clone() },
        En1998Mutation::ChangeKFoundation(payload) => En1998MutationDsl::ChangeKFoundation { new_k_foundation: payload.new_k_foundation.clone() },
        En1998Mutation::ChangeKSoil(payload) => En1998MutationDsl::ChangeKSoil { new_k_soil: payload.new_k_soil.clone() },
        En1998Mutation::ChangeWallHeightM(payload) => En1998MutationDsl::ChangeWallHeightM { new_wall_height_m: payload.new_wall_height_m.clone() },
        En1998Mutation::ChangeWallPhiDeg(payload) => En1998MutationDsl::ChangeWallPhiDeg { new_wall_phi_deg: payload.new_wall_phi_deg.clone() },
        En1998Mutation::ChangeWallSoilGammaKnM3(payload) => En1998MutationDsl::ChangeWallSoilGammaKnM3 { new_wall_soil_gamma_kn_m3: payload.new_wall_soil_gamma_kn_m3.clone() },
        En1998Mutation::ChangeWallR(payload) => En1998MutationDsl::ChangeWallR { new_wall_r: payload.new_wall_r.clone() },
        En1998Mutation::ChangeWallHRdKn(payload) => En1998MutationDsl::ChangeWallHRdKn { new_wall_h_rd_kn: payload.new_wall_h_rd_kn.clone() },
    }
}

async fn en1998_mutation_from_dsl(mutation: En1998MutationDsl) -> En1998Mutation {
    match mutation {
        En1998MutationDsl::ChangeSeismicZone { new_seismic_zone } => En1998Mutation::ChangeSeismicZone(change_seismic_zone::mutation::ChangeSeismicZone { new_seismic_zone }),
        En1998MutationDsl::ChangeGroundType { new_ground_type } => En1998Mutation::ChangeGroundType(change_ground_type::mutation::ChangeGroundType { new_ground_type }),
        En1998MutationDsl::ChangeImportanceClass { new_importance_class } => En1998Mutation::ChangeImportanceClass(change_importance_class::mutation::ChangeImportanceClass { new_importance_class }),
        En1998MutationDsl::ChangeStructuralSystem { new_structural_system } => En1998Mutation::ChangeStructuralSystem(change_structural_system::mutation::ChangeStructuralSystem { new_structural_system }),
        En1998MutationDsl::ChangeT1S { new_t1_s } => En1998Mutation::ChangeT1S(change_t1_s::mutation::ChangeT1S { new_t1_s }),
        En1998MutationDsl::ChangeMassT { new_mass_t } => En1998Mutation::ChangeMassT(change_mass_t::mutation::ChangeMassT { new_mass_t }),
        En1998MutationDsl::ChangeVRdKn { new_v_rd_kn } => En1998Mutation::ChangeVRdKn(change_v_rd_kn::mutation::ChangeVRdKn { new_v_rd_kn }),
        En1998MutationDsl::ChangeDriftMm { new_drift_mm } => En1998Mutation::ChangeDriftMm(change_drift_mm::mutation::ChangeDriftMm { new_drift_mm }),
        En1998MutationDsl::ChangeHeightM { new_height_m } => En1998Mutation::ChangeHeightM(change_height_m::mutation::ChangeHeightM { new_height_m }),
        En1998MutationDsl::ChangeMultipleResistingSystems { new_multiple_resisting_systems } => {
            En1998Mutation::ChangeMultipleResistingSystems(change_multiple_resisting_systems::mutation::ChangeMultipleResistingSystems { new_multiple_resisting_systems })
        }
        En1998MutationDsl::ChangeAnnex { new_annex } => En1998Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex }),
        En1998MutationDsl::ChangeEnAGr { new_en_a_gr } => En1998Mutation::ChangeEnAGr(change_en_a_gr::mutation::ChangeEnAGr { new_en_a_gr }),
        En1998MutationDsl::ChangeEnGroundType { new_en_ground_type } => En1998Mutation::ChangeEnGroundType(change_en_ground_type::mutation::ChangeEnGroundType { new_en_ground_type }),
        En1998MutationDsl::ChangeEnSpectrumType { new_en_spectrum_type } => En1998Mutation::ChangeEnSpectrumType(change_en_spectrum_type::mutation::ChangeEnSpectrumType { new_en_spectrum_type }),
        En1998MutationDsl::ChangePeriodRatio { new_period_ratio } => En1998Mutation::ChangePeriodRatio(change_period_ratio::mutation::ChangePeriodRatio { new_period_ratio }),
        En1998MutationDsl::ChangeBridgeVRdKn { new_bridge_v_rd_kn } => En1998Mutation::ChangeBridgeVRdKn(change_bridge_v_rd_kn::mutation::ChangeBridgeVRdKn { new_bridge_v_rd_kn }),
        En1998MutationDsl::ChangeBearingDEdMm { new_bearing_d_ed_mm } => En1998Mutation::ChangeBearingDEdMm(change_bearing_d_ed_mm::mutation::ChangeBearingDEdMm { new_bearing_d_ed_mm }),
        En1998MutationDsl::ChangeBearingDRdMm { new_bearing_d_rd_mm } => En1998Mutation::ChangeBearingDRdMm(change_bearing_d_rd_mm::mutation::ChangeBearingDRdMm { new_bearing_d_rd_mm }),
        En1998MutationDsl::ChangeRetrofitKnowledgeLevel { new_retrofit_knowledge_level } => En1998Mutation::ChangeRetrofitKnowledgeLevel(change_retrofit_knowledge_level::mutation::ChangeRetrofitKnowledgeLevel { new_retrofit_knowledge_level }),
        En1998MutationDsl::ChangeRetrofitLimitState { new_retrofit_limit_state } => En1998Mutation::ChangeRetrofitLimitState(change_retrofit_limit_state::mutation::ChangeRetrofitLimitState { new_retrofit_limit_state }),
        En1998MutationDsl::ChangeRetrofitEDKn { new_retrofit_e_d_kn } => En1998Mutation::ChangeRetrofitEDKn(change_retrofit_e_d_kn::mutation::ChangeRetrofitEDKn { new_retrofit_e_d_kn }),
        En1998MutationDsl::ChangeRetrofitRKKn { new_retrofit_r_k_kn } => En1998Mutation::ChangeRetrofitRKKn(change_retrofit_r_k_kn::mutation::ChangeRetrofitRKKn { new_retrofit_r_k_kn }),
        En1998MutationDsl::ChangeRetrofitGammaEl { new_retrofit_gamma_el } => En1998Mutation::ChangeRetrofitGammaEl(change_retrofit_gamma_el::mutation::ChangeRetrofitGammaEl { new_retrofit_gamma_el }),
        En1998MutationDsl::ChangeSiloHeightM { new_silo_height_m } => En1998Mutation::ChangeSiloHeightM(change_silo_height_m::mutation::ChangeSiloHeightM { new_silo_height_m }),
        En1998MutationDsl::ChangeSiloRadiusM { new_silo_radius_m } => En1998Mutation::ChangeSiloRadiusM(change_silo_radius_m::mutation::ChangeSiloRadiusM { new_silo_radius_m }),
        En1998MutationDsl::ChangeSiloNRdKn { new_silo_n_rd_kn } => En1998Mutation::ChangeSiloNRdKn(change_silo_n_rd_kn::mutation::ChangeSiloNRdKn { new_silo_n_rd_kn }),
        En1998MutationDsl::ChangeSiloVEdKn { new_silo_v_ed_kn } => En1998Mutation::ChangeSiloVEdKn(change_silo_v_ed_kn::mutation::ChangeSiloVEdKn { new_silo_v_ed_kn }),
        En1998MutationDsl::ChangeSiloVRdKn { new_silo_v_rd_kn } => En1998Mutation::ChangeSiloVRdKn(change_silo_v_rd_kn::mutation::ChangeSiloVRdKn { new_silo_v_rd_kn }),
        En1998MutationDsl::ChangeSiloQNominal { new_silo_q_nominal } => En1998Mutation::ChangeSiloQNominal(change_silo_q_nominal::mutation::ChangeSiloQNominal { new_silo_q_nominal }),
        En1998MutationDsl::ChangeTankHeightM { new_tank_height_m } => En1998Mutation::ChangeTankHeightM(change_tank_height_m::mutation::ChangeTankHeightM { new_tank_height_m }),
        En1998MutationDsl::ChangeTankRadiusM { new_tank_radius_m } => En1998Mutation::ChangeTankRadiusM(change_tank_radius_m::mutation::ChangeTankRadiusM { new_tank_radius_m }),
        En1998MutationDsl::ChangeTankMassT { new_tank_mass_t } => En1998Mutation::ChangeTankMassT(change_tank_mass_t::mutation::ChangeTankMassT { new_tank_mass_t }),
        En1998MutationDsl::ChangeTankVRdKn { new_tank_v_rd_kn } => En1998Mutation::ChangeTankVRdKn(change_tank_v_rd_kn::mutation::ChangeTankVRdKn { new_tank_v_rd_kn }),
        En1998MutationDsl::ChangeTowerMEdKnm { new_tower_m_ed_knm } => En1998Mutation::ChangeTowerMEdKnm(change_tower_m_ed_knm::mutation::ChangeTowerMEdKnm { new_tower_m_ed_knm }),
        En1998MutationDsl::ChangeTowerMRdKnm { new_tower_m_rd_knm } => En1998Mutation::ChangeTowerMRdKnm(change_tower_m_rd_knm::mutation::ChangeTowerMRdKnm { new_tower_m_rd_knm }),
        En1998MutationDsl::ChangeTowerIsChimney { new_tower_is_chimney } => En1998Mutation::ChangeTowerIsChimney(change_tower_is_chimney::mutation::ChangeTowerIsChimney { new_tower_is_chimney }),
        En1998MutationDsl::ChangeTowerQNominal { new_tower_q_nominal } => En1998Mutation::ChangeTowerQNominal(change_tower_q_nominal::mutation::ChangeTowerQNominal { new_tower_q_nominal }),
        En1998MutationDsl::ChangeTowerMassT { new_tower_mass_t } => En1998Mutation::ChangeTowerMassT(change_tower_mass_t::mutation::ChangeTowerMassT { new_tower_mass_t }),
        En1998MutationDsl::ChangeFoundationAreaM2 { new_foundation_area_m2 } => En1998Mutation::ChangeFoundationAreaM2(change_foundation_area_m2::mutation::ChangeFoundationAreaM2 { new_foundation_area_m2 }),
        En1998MutationDsl::ChangeFoundationPRdKpa { new_foundation_p_rd_kpa } => En1998Mutation::ChangeFoundationPRdKpa(change_foundation_p_rd_kpa::mutation::ChangeFoundationPRdKpa { new_foundation_p_rd_kpa }),
        En1998MutationDsl::ChangeFoundationHEdKn { new_foundation_h_ed_kn } => En1998Mutation::ChangeFoundationHEdKn(change_foundation_h_ed_kn::mutation::ChangeFoundationHEdKn { new_foundation_h_ed_kn }),
        En1998MutationDsl::ChangeFoundationHRdKn { new_foundation_h_rd_kn } => En1998Mutation::ChangeFoundationHRdKn(change_foundation_h_rd_kn::mutation::ChangeFoundationHRdKn { new_foundation_h_rd_kn }),
        En1998MutationDsl::ChangeKFoundation { new_k_foundation } => En1998Mutation::ChangeKFoundation(change_k_foundation::mutation::ChangeKFoundation { new_k_foundation }),
        En1998MutationDsl::ChangeKSoil { new_k_soil } => En1998Mutation::ChangeKSoil(change_k_soil::mutation::ChangeKSoil { new_k_soil }),
        En1998MutationDsl::ChangeWallHeightM { new_wall_height_m } => En1998Mutation::ChangeWallHeightM(change_wall_height_m::mutation::ChangeWallHeightM { new_wall_height_m }),
        En1998MutationDsl::ChangeWallPhiDeg { new_wall_phi_deg } => En1998Mutation::ChangeWallPhiDeg(change_wall_phi_deg::mutation::ChangeWallPhiDeg { new_wall_phi_deg }),
        En1998MutationDsl::ChangeWallSoilGammaKnM3 { new_wall_soil_gamma_kn_m3 } => En1998Mutation::ChangeWallSoilGammaKnM3(change_wall_soil_gamma_kn_m3::mutation::ChangeWallSoilGammaKnM3 { new_wall_soil_gamma_kn_m3 }),
        En1998MutationDsl::ChangeWallR { new_wall_r } => En1998Mutation::ChangeWallR(change_wall_r::mutation::ChangeWallR { new_wall_r }),
        En1998MutationDsl::ChangeWallHRdKn { new_wall_h_rd_kn } => En1998Mutation::ChangeWallHRdKn(change_wall_h_rd_kn::mutation::ChangeWallHRdKn { new_wall_h_rd_kn }),
    }
}

impl OpText for En1998Mutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(en1998_mutation_from_dsl(<En1998MutationDsl as OpText>::parse_op(line)?))
    }

    async fn print_op(&self) -> String {
        <En1998MutationDsl as OpText>::print_op(&en1998_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `En1998MutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for En1998Mutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        en1998_mutation_to_dsl(self).encode_op()
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(en1998_mutation_from_dsl(En1998MutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn op_text_round_trips_change_seismic_zone() {
        store::os_store::test_support::assert_op_line_round_trip(&En1998Mutation::ChangeSeismicZone(change_seismic_zone::mutation::ChangeSeismicZone { new_seismic_zone: 3 }));
    }

    #[test]
    async fn op_text_round_trips_change_ground_type() {
        store::os_store::test_support::assert_op_line_round_trip(&En1998Mutation::ChangeGroundType(change_ground_type::mutation::ChangeGroundType { new_ground_type: "c".to_string() }));
    }

    #[test]
    async fn op_text_round_trips_change_multiple_resisting_systems() {
        store::os_store::test_support::assert_op_line_round_trip(&En1998Mutation::ChangeMultipleResistingSystems(change_multiple_resisting_systems::mutation::ChangeMultipleResistingSystems { new_multiple_resisting_systems: false }));
    }

    /// ⚖️ Every variant, not just the hand-picked ones above — full-coverage `OpText` round trip
    /// over the closed vocabulary, one sample value per field.
    #[test]
    async fn every_variant_op_text_round_trips() {
        for mutation in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&mutation);
        }
    }

    async fn every_mutation() -> Vec<En1998Mutation> {
        vec![
            En1998Mutation::ChangeSeismicZone(change_seismic_zone::mutation::ChangeSeismicZone { new_seismic_zone: 3 }),
            En1998Mutation::ChangeGroundType(change_ground_type::mutation::ChangeGroundType { new_ground_type: "c".to_string() }),
            En1998Mutation::ChangeImportanceClass(change_importance_class::mutation::ChangeImportanceClass { new_importance_class: "cc3".to_string() }),
            En1998Mutation::ChangeStructuralSystem(change_structural_system::mutation::ChangeStructuralSystem { new_structural_system: "wall_dcm".to_string() }),
            En1998Mutation::ChangeT1S(change_t1_s::mutation::ChangeT1S { new_t1_s: 0.35 }),
            En1998Mutation::ChangeMassT(change_mass_t::mutation::ChangeMassT { new_mass_t: 550.0 }),
            En1998Mutation::ChangeVRdKn(change_v_rd_kn::mutation::ChangeVRdKn { new_v_rd_kn: 850.0 }),
            En1998Mutation::ChangeDriftMm(change_drift_mm::mutation::ChangeDriftMm { new_drift_mm: 22.0 }),
            En1998Mutation::ChangeHeightM(change_height_m::mutation::ChangeHeightM { new_height_m: 14.0 }),
            En1998Mutation::ChangeMultipleResistingSystems(change_multiple_resisting_systems::mutation::ChangeMultipleResistingSystems { new_multiple_resisting_systems: false }),
            En1998Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: "en".to_string() }),
            En1998Mutation::ChangeEnAGr(change_en_a_gr::mutation::ChangeEnAGr { new_en_a_gr: 0.2 }),
            En1998Mutation::ChangeEnGroundType(change_en_ground_type::mutation::ChangeEnGroundType { new_en_ground_type: "c".to_string() }),
            En1998Mutation::ChangeEnSpectrumType(change_en_spectrum_type::mutation::ChangeEnSpectrumType { new_en_spectrum_type: "type2".to_string() }),
            En1998Mutation::ChangePeriodRatio(change_period_ratio::mutation::ChangePeriodRatio { new_period_ratio: 1.8 }),
            En1998Mutation::ChangeBridgeVRdKn(change_bridge_v_rd_kn::mutation::ChangeBridgeVRdKn { new_bridge_v_rd_kn: 650.0 }),
            En1998Mutation::ChangeBearingDEdMm(change_bearing_d_ed_mm::mutation::ChangeBearingDEdMm { new_bearing_d_ed_mm: 130.0 }),
            En1998Mutation::ChangeBearingDRdMm(change_bearing_d_rd_mm::mutation::ChangeBearingDRdMm { new_bearing_d_rd_mm: 260.0 }),
            En1998Mutation::ChangeRetrofitKnowledgeLevel(change_retrofit_knowledge_level::mutation::ChangeRetrofitKnowledgeLevel { new_retrofit_knowledge_level: "kl3".to_string() }),
            En1998Mutation::ChangeRetrofitLimitState(change_retrofit_limit_state::mutation::ChangeRetrofitLimitState { new_retrofit_limit_state: "near_collapse".to_string() }),
            En1998Mutation::ChangeRetrofitEDKn(change_retrofit_e_d_kn::mutation::ChangeRetrofitEDKn { new_retrofit_e_d_kn: 270.0 }),
            En1998Mutation::ChangeRetrofitRKKn(change_retrofit_r_k_kn::mutation::ChangeRetrofitRKKn { new_retrofit_r_k_kn: 420.0 }),
            En1998Mutation::ChangeRetrofitGammaEl(change_retrofit_gamma_el::mutation::ChangeRetrofitGammaEl { new_retrofit_gamma_el: 1.15 }),
            En1998Mutation::ChangeSiloHeightM(change_silo_height_m::mutation::ChangeSiloHeightM { new_silo_height_m: 11.0 }),
            En1998Mutation::ChangeSiloRadiusM(change_silo_radius_m::mutation::ChangeSiloRadiusM { new_silo_radius_m: 5.5 }),
            En1998Mutation::ChangeSiloNRdKn(change_silo_n_rd_kn::mutation::ChangeSiloNRdKn { new_silo_n_rd_kn: 520.0 }),
            En1998Mutation::ChangeSiloVEdKn(change_silo_v_ed_kn::mutation::ChangeSiloVEdKn { new_silo_v_ed_kn: 190.0 }),
            En1998Mutation::ChangeSiloVRdKn(change_silo_v_rd_kn::mutation::ChangeSiloVRdKn { new_silo_v_rd_kn: 320.0 }),
            En1998Mutation::ChangeSiloQNominal(change_silo_q_nominal::mutation::ChangeSiloQNominal { new_silo_q_nominal: 2.2 }),
            En1998Mutation::ChangeTankHeightM(change_tank_height_m::mutation::ChangeTankHeightM { new_tank_height_m: 9.0 }),
            En1998Mutation::ChangeTankRadiusM(change_tank_radius_m::mutation::ChangeTankRadiusM { new_tank_radius_m: 4.5 }),
            En1998Mutation::ChangeTankMassT(change_tank_mass_t::mutation::ChangeTankMassT { new_tank_mass_t: 320.0 }),
            En1998Mutation::ChangeTankVRdKn(change_tank_v_rd_kn::mutation::ChangeTankVRdKn { new_tank_v_rd_kn: 420.0 }),
            En1998Mutation::ChangeTowerMEdKnm(change_tower_m_ed_knm::mutation::ChangeTowerMEdKnm { new_tower_m_ed_knm: 1300.0 }),
            En1998Mutation::ChangeTowerMRdKnm(change_tower_m_rd_knm::mutation::ChangeTowerMRdKnm { new_tower_m_rd_knm: 2600.0 }),
            En1998Mutation::ChangeTowerIsChimney(change_tower_is_chimney::mutation::ChangeTowerIsChimney { new_tower_is_chimney: false }),
            En1998Mutation::ChangeTowerQNominal(change_tower_q_nominal::mutation::ChangeTowerQNominal { new_tower_q_nominal: 2.8 }),
            En1998Mutation::ChangeTowerMassT(change_tower_mass_t::mutation::ChangeTowerMassT { new_tower_mass_t: 85.0 }),
            En1998Mutation::ChangeFoundationAreaM2(change_foundation_area_m2::mutation::ChangeFoundationAreaM2 { new_foundation_area_m2: 110.0 }),
            En1998Mutation::ChangeFoundationPRdKpa(change_foundation_p_rd_kpa::mutation::ChangeFoundationPRdKpa { new_foundation_p_rd_kpa: 520.0 }),
            En1998Mutation::ChangeFoundationHEdKn(change_foundation_h_ed_kn::mutation::ChangeFoundationHEdKn { new_foundation_h_ed_kn: 160.0 }),
            En1998Mutation::ChangeFoundationHRdKn(change_foundation_h_rd_kn::mutation::ChangeFoundationHRdKn { new_foundation_h_rd_kn: 420.0 }),
            En1998Mutation::ChangeKFoundation(change_k_foundation::mutation::ChangeKFoundation { new_k_foundation: 520_000.0 }),
            En1998Mutation::ChangeKSoil(change_k_soil::mutation::ChangeKSoil { new_k_soil: 210_000.0 }),
            En1998Mutation::ChangeWallHeightM(change_wall_height_m::mutation::ChangeWallHeightM { new_wall_height_m: 4.5 }),
            En1998Mutation::ChangeWallPhiDeg(change_wall_phi_deg::mutation::ChangeWallPhiDeg { new_wall_phi_deg: 32.0 }),
            En1998Mutation::ChangeWallSoilGammaKnM3(change_wall_soil_gamma_kn_m3::mutation::ChangeWallSoilGammaKnM3 { new_wall_soil_gamma_kn_m3: 19.0 }),
            En1998Mutation::ChangeWallR(change_wall_r::mutation::ChangeWallR { new_wall_r: 1.4 }),
            En1998Mutation::ChangeWallHRdKn(change_wall_h_rd_kn::mutation::ChangeWallHRdKn { new_wall_h_rd_kn: 160.0 }),
        ]
    }
}
//#endregion 🧪️Tests
