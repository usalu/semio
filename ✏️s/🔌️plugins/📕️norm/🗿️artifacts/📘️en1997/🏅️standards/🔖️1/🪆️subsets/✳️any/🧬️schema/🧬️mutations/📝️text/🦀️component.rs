//! 🔧️ En1997 artifact — OpText/OpBinary codecs for `En1997Mutation`. Mutation apply/inverse
//! live in `🧬️mutations`; this facet only handcrafts the op wire forms (the shared
//! whole-document-replace macro no longer applies now that the whole-document-replace variant is
//! gone).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::en1997::schema::mutations::En1997Mutation;
use crate::artifacts::en1997::schema::mutations::{
    change_alpha_s, change_annex, change_b_m, change_c_kpa, change_d_f_m, change_design_approach, change_e_s_mpa, change_footing_area_m2, change_gamma_kn_m3, change_h_ed_kn, change_n_pile_ed_kn, change_nu, change_phi_deg, change_pile_base_area_m2,
    change_pile_d_m, change_pile_l_m, change_pile_n_profiles, change_q_b_kpa, change_q_s_kpa, change_settlement_limit_mm, change_v_ed_kn, change_z_investigated_m,
};
use crate::document::AnnexChoice;
use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `En1997Mutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only; `En1997Mutation` itself,
/// and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum En1997MutationDsl {
    ChangeVEdKn { new_v_ed_kn: f64 },
    ChangeHEdKn { new_h_ed_kn: f64 },
    ChangeFootingAreaM2 { new_footing_area_m2: f64 },
    ChangePhiDeg { new_phi_deg: f64 },
    ChangeCKpa { new_c_kpa: f64 },
    ChangeGammaKnM3 { new_gamma_kn_m3: f64 },
    ChangeBM { new_b_m: f64 },
    ChangeDFM { new_d_f_m: f64 },
    ChangeESMpa { new_e_s_mpa: f64 },
    ChangeNu { new_nu: f64 },
    ChangeDesignApproach { new_design_approach: String },
    ChangeAnnex { new_annex: AnnexChoice },
    ChangeSettlementLimitMm { new_settlement_limit_mm: f64 },
    ChangeNPileEdKn { new_n_pile_ed_kn: f64 },
    ChangeAlphaS { new_alpha_s: f64 },
    ChangePileDM { new_pile_d_m: f64 },
    ChangeQSKpa { new_q_s_kpa: f64 },
    ChangePileLM { new_pile_l_m: f64 },
    ChangeQBKpa { new_q_b_kpa: f64 },
    ChangePileBaseAreaM2 { new_pile_base_area_m2: f64 },
    ChangePileNProfiles { new_pile_n_profiles: u32 },
    ChangeZInvestigatedM { new_z_investigated_m: f64 },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for En1997MutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for En1997MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn en1997_mutation_to_dsl(mutation: &En1997Mutation) -> En1997MutationDsl {
    match mutation {
        En1997Mutation::ChangeVEdKn(payload) => En1997MutationDsl::ChangeVEdKn { new_v_ed_kn: payload.new_v_ed_kn.clone() },
        En1997Mutation::ChangeHEdKn(payload) => En1997MutationDsl::ChangeHEdKn { new_h_ed_kn: payload.new_h_ed_kn.clone() },
        En1997Mutation::ChangeFootingAreaM2(payload) => En1997MutationDsl::ChangeFootingAreaM2 { new_footing_area_m2: payload.new_footing_area_m2.clone() },
        En1997Mutation::ChangePhiDeg(payload) => En1997MutationDsl::ChangePhiDeg { new_phi_deg: payload.new_phi_deg.clone() },
        En1997Mutation::ChangeCKpa(payload) => En1997MutationDsl::ChangeCKpa { new_c_kpa: payload.new_c_kpa.clone() },
        En1997Mutation::ChangeGammaKnM3(payload) => En1997MutationDsl::ChangeGammaKnM3 { new_gamma_kn_m3: payload.new_gamma_kn_m3.clone() },
        En1997Mutation::ChangeBM(payload) => En1997MutationDsl::ChangeBM { new_b_m: payload.new_b_m.clone() },
        En1997Mutation::ChangeDFM(payload) => En1997MutationDsl::ChangeDFM { new_d_f_m: payload.new_d_f_m.clone() },
        En1997Mutation::ChangeESMpa(payload) => En1997MutationDsl::ChangeESMpa { new_e_s_mpa: payload.new_e_s_mpa.clone() },
        En1997Mutation::ChangeNu(payload) => En1997MutationDsl::ChangeNu { new_nu: payload.new_nu.clone() },
        En1997Mutation::ChangeDesignApproach(payload) => En1997MutationDsl::ChangeDesignApproach { new_design_approach: payload.new_design_approach.clone() },
        En1997Mutation::ChangeAnnex(payload) => En1997MutationDsl::ChangeAnnex { new_annex: payload.new_annex.clone() },
        En1997Mutation::ChangeSettlementLimitMm(payload) => En1997MutationDsl::ChangeSettlementLimitMm { new_settlement_limit_mm: payload.new_settlement_limit_mm.clone() },
        En1997Mutation::ChangeNPileEdKn(payload) => En1997MutationDsl::ChangeNPileEdKn { new_n_pile_ed_kn: payload.new_n_pile_ed_kn.clone() },
        En1997Mutation::ChangeAlphaS(payload) => En1997MutationDsl::ChangeAlphaS { new_alpha_s: payload.new_alpha_s.clone() },
        En1997Mutation::ChangePileDM(payload) => En1997MutationDsl::ChangePileDM { new_pile_d_m: payload.new_pile_d_m.clone() },
        En1997Mutation::ChangeQSKpa(payload) => En1997MutationDsl::ChangeQSKpa { new_q_s_kpa: payload.new_q_s_kpa.clone() },
        En1997Mutation::ChangePileLM(payload) => En1997MutationDsl::ChangePileLM { new_pile_l_m: payload.new_pile_l_m.clone() },
        En1997Mutation::ChangeQBKpa(payload) => En1997MutationDsl::ChangeQBKpa { new_q_b_kpa: payload.new_q_b_kpa.clone() },
        En1997Mutation::ChangePileBaseAreaM2(payload) => En1997MutationDsl::ChangePileBaseAreaM2 { new_pile_base_area_m2: payload.new_pile_base_area_m2.clone() },
        En1997Mutation::ChangePileNProfiles(payload) => En1997MutationDsl::ChangePileNProfiles { new_pile_n_profiles: payload.new_pile_n_profiles.clone() },
        En1997Mutation::ChangeZInvestigatedM(payload) => En1997MutationDsl::ChangeZInvestigatedM { new_z_investigated_m: payload.new_z_investigated_m.clone() },
    }
}

fn en1997_mutation_from_dsl(mutation: En1997MutationDsl) -> En1997Mutation {
    match mutation {
        En1997MutationDsl::ChangeVEdKn { new_v_ed_kn } => En1997Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn }),
        En1997MutationDsl::ChangeHEdKn { new_h_ed_kn } => En1997Mutation::ChangeHEdKn(change_h_ed_kn::ChangeHEdKn { new_h_ed_kn }),
        En1997MutationDsl::ChangeFootingAreaM2 { new_footing_area_m2 } => En1997Mutation::ChangeFootingAreaM2(change_footing_area_m2::ChangeFootingAreaM2 { new_footing_area_m2 }),
        En1997MutationDsl::ChangePhiDeg { new_phi_deg } => En1997Mutation::ChangePhiDeg(change_phi_deg::ChangePhiDeg { new_phi_deg }),
        En1997MutationDsl::ChangeCKpa { new_c_kpa } => En1997Mutation::ChangeCKpa(change_c_kpa::ChangeCKpa { new_c_kpa }),
        En1997MutationDsl::ChangeGammaKnM3 { new_gamma_kn_m3 } => En1997Mutation::ChangeGammaKnM3(change_gamma_kn_m3::ChangeGammaKnM3 { new_gamma_kn_m3 }),
        En1997MutationDsl::ChangeBM { new_b_m } => En1997Mutation::ChangeBM(change_b_m::ChangeBM { new_b_m }),
        En1997MutationDsl::ChangeDFM { new_d_f_m } => En1997Mutation::ChangeDFM(change_d_f_m::ChangeDFM { new_d_f_m }),
        En1997MutationDsl::ChangeESMpa { new_e_s_mpa } => En1997Mutation::ChangeESMpa(change_e_s_mpa::ChangeESMpa { new_e_s_mpa }),
        En1997MutationDsl::ChangeNu { new_nu } => En1997Mutation::ChangeNu(change_nu::ChangeNu { new_nu }),
        En1997MutationDsl::ChangeDesignApproach { new_design_approach } => En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach }),
        En1997MutationDsl::ChangeAnnex { new_annex } => En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex }),
        En1997MutationDsl::ChangeSettlementLimitMm { new_settlement_limit_mm } => En1997Mutation::ChangeSettlementLimitMm(change_settlement_limit_mm::ChangeSettlementLimitMm { new_settlement_limit_mm }),
        En1997MutationDsl::ChangeNPileEdKn { new_n_pile_ed_kn } => En1997Mutation::ChangeNPileEdKn(change_n_pile_ed_kn::ChangeNPileEdKn { new_n_pile_ed_kn }),
        En1997MutationDsl::ChangeAlphaS { new_alpha_s } => En1997Mutation::ChangeAlphaS(change_alpha_s::ChangeAlphaS { new_alpha_s }),
        En1997MutationDsl::ChangePileDM { new_pile_d_m } => En1997Mutation::ChangePileDM(change_pile_d_m::ChangePileDM { new_pile_d_m }),
        En1997MutationDsl::ChangeQSKpa { new_q_s_kpa } => En1997Mutation::ChangeQSKpa(change_q_s_kpa::ChangeQSKpa { new_q_s_kpa }),
        En1997MutationDsl::ChangePileLM { new_pile_l_m } => En1997Mutation::ChangePileLM(change_pile_l_m::ChangePileLM { new_pile_l_m }),
        En1997MutationDsl::ChangeQBKpa { new_q_b_kpa } => En1997Mutation::ChangeQBKpa(change_q_b_kpa::ChangeQBKpa { new_q_b_kpa }),
        En1997MutationDsl::ChangePileBaseAreaM2 { new_pile_base_area_m2 } => En1997Mutation::ChangePileBaseAreaM2(change_pile_base_area_m2::ChangePileBaseAreaM2 { new_pile_base_area_m2 }),
        En1997MutationDsl::ChangePileNProfiles { new_pile_n_profiles } => En1997Mutation::ChangePileNProfiles(change_pile_n_profiles::ChangePileNProfiles { new_pile_n_profiles }),
        En1997MutationDsl::ChangeZInvestigatedM { new_z_investigated_m } => En1997Mutation::ChangeZInvestigatedM(change_z_investigated_m::ChangeZInvestigatedM { new_z_investigated_m }),
    }
}

impl OpText for En1997Mutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(en1997_mutation_from_dsl(<En1997MutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <En1997MutationDsl as OpText>::print_op(&en1997_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `En1997MutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for En1997Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        en1997_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(en1997_mutation_from_dsl(En1997MutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn op_text_round_trips_change_v_ed_kn() {
        store::os_store::test_support::assert_op_line_round_trip(&En1997Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 620.0 }));
    }

    #[semio_framework_async_macros::async_test]
    fn op_text_round_trips_change_annex() {
        store::os_store::test_support::assert_op_line_round_trip(&En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }));
    }

    #[semio_framework_async_macros::async_test]
    fn op_text_round_trips_change_design_approach() {
        store::os_store::test_support::assert_op_line_round_trip(&En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: "da2".to_string() }));
    }

    /// ⚖️ Every variant, not just the hand-picked ones above — full-coverage `OpText` round trip
    /// over the closed vocabulary, one sample value per field.
    #[semio_framework_async_macros::async_test]
    fn every_variant_op_text_round_trips() {
        for mutation in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&mutation);
        }
    }

    fn every_mutation() -> Vec<En1997Mutation> {
        vec![
            En1997Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 620.0 }),
            En1997Mutation::ChangeHEdKn(change_h_ed_kn::ChangeHEdKn { new_h_ed_kn: 95.0 }),
            En1997Mutation::ChangeFootingAreaM2(change_footing_area_m2::ChangeFootingAreaM2 { new_footing_area_m2: 2.4 }),
            En1997Mutation::ChangePhiDeg(change_phi_deg::ChangePhiDeg { new_phi_deg: 32.0 }),
            En1997Mutation::ChangeCKpa(change_c_kpa::ChangeCKpa { new_c_kpa: 5.0 }),
            En1997Mutation::ChangeGammaKnM3(change_gamma_kn_m3::ChangeGammaKnM3 { new_gamma_kn_m3: 19.0 }),
            En1997Mutation::ChangeBM(change_b_m::ChangeBM { new_b_m: 2.2 }),
            En1997Mutation::ChangeDFM(change_d_f_m::ChangeDFM { new_d_f_m: 1.8 }),
            En1997Mutation::ChangeESMpa(change_e_s_mpa::ChangeESMpa { new_e_s_mpa: 32_000.0 }),
            En1997Mutation::ChangeNu(change_nu::ChangeNu { new_nu: 0.32 }),
            En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: "da2".to_string() }),
            En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
            En1997Mutation::ChangeSettlementLimitMm(change_settlement_limit_mm::ChangeSettlementLimitMm { new_settlement_limit_mm: 20.0 }),
            En1997Mutation::ChangeNPileEdKn(change_n_pile_ed_kn::ChangeNPileEdKn { new_n_pile_ed_kn: 900.0 }),
            En1997Mutation::ChangeAlphaS(change_alpha_s::ChangeAlphaS { new_alpha_s: 0.75 }),
            En1997Mutation::ChangePileDM(change_pile_d_m::ChangePileDM { new_pile_d_m: 0.65 }),
            En1997Mutation::ChangeQSKpa(change_q_s_kpa::ChangeQSKpa { new_q_s_kpa: 90.0 }),
            En1997Mutation::ChangePileLM(change_pile_l_m::ChangePileLM { new_pile_l_m: 14.0 }),
            En1997Mutation::ChangeQBKpa(change_q_b_kpa::ChangeQBKpa { new_q_b_kpa: 2700.0 }),
            En1997Mutation::ChangePileBaseAreaM2(change_pile_base_area_m2::ChangePileBaseAreaM2 { new_pile_base_area_m2: 0.33 }),
            En1997Mutation::ChangePileNProfiles(change_pile_n_profiles::ChangePileNProfiles { new_pile_n_profiles: 3 }),
            En1997Mutation::ChangeZInvestigatedM(change_z_investigated_m::ChangeZInvestigatedM { new_z_investigated_m: 10.0 }),
        ]
    }
}
//#endregion 🧪️Tests
