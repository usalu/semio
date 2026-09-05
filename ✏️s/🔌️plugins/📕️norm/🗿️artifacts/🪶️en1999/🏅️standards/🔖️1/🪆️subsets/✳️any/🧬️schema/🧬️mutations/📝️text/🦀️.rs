//! 🔧️ En1999 artifact — OpText/OpBinary codecs for `En1999Mutation`. Mutation apply/inverse
//! live in `🧬️mutations`; this facet only handcrafts the op wire forms (the shared
//! whole-document-replace macro no longer applies now that the whole-document-replace variant is
//! gone).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::en1999::schema::mutations::En1999Mutation;
use crate::artifacts::en1999::schema::mutations::{
    change_a_mm2, change_alloy, change_annex, change_beta_w, change_chi, change_delta_sigma_c, change_delta_sigma_ed, change_fatigue_m, change_i_t_mm4, change_l_cr_mm, change_m_ed_knm, change_n_cycles, change_n_ed_kn, change_sheet_b_mm,
    change_sheet_k_sigma, change_sheet_m_ed_knm, change_sheet_t_mm, change_sheet_w_el_mm3, change_shell_r_mm, change_shell_t_mm, change_sigma_ed_shell_mpa, change_theta_c, change_v_weld_ed_kn, change_w_el_mm3, change_weld_length_mm,
    change_weld_throat_mm,
};
use crate::document::AnnexChoice;
use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `En1999Mutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only; `En1999Mutation` itself,
/// and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum En1999MutationDsl {
    ChangeNEdKn { new_n_ed_kn: f64 },
    ChangeMEdKnm { new_m_ed_knm: f64 },
    ChangeAMm2 { new_a_mm2: f64 },
    ChangeWElMm3 { new_w_el_mm3: f64 },
    ChangeAlloy { new_alloy: String },
    ChangeChi { new_chi: f64 },
    ChangeITMm4 { new_i_t_mm4: f64 },
    ChangeLCrMm { new_l_cr_mm: f64 },
    ChangeThetaC { new_theta_c: f64 },
    ChangeDeltaSigmaEd { new_delta_sigma_ed: f64 },
    ChangeDeltaSigmaC { new_delta_sigma_c: f64 },
    ChangeFatigueM { new_fatigue_m: f64 },
    ChangeNCycles { new_n_cycles: f64 },
    ChangeVWeldEdKn { new_v_weld_ed_kn: f64 },
    ChangeWeldThroatMm { new_weld_throat_mm: f64 },
    ChangeWeldLengthMm { new_weld_length_mm: f64 },
    ChangeBetaW { new_beta_w: f64 },
    ChangeSheetBMm { new_sheet_b_mm: f64 },
    ChangeSheetTMm { new_sheet_t_mm: f64 },
    ChangeSheetKSigma { new_sheet_k_sigma: f64 },
    ChangeSheetWElMm3 { new_sheet_w_el_mm3: f64 },
    ChangeSheetMEdKnm { new_sheet_m_ed_knm: f64 },
    ChangeShellTMm { new_shell_t_mm: f64 },
    ChangeShellRMm { new_shell_r_mm: f64 },
    ChangeSigmaEdShellMpa { new_sigma_ed_shell_mpa: f64 },
    ChangeAnnex { new_annex: AnnexChoice },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for En1999MutationDsl {
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

impl protocol::OpBinary for En1999MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn en1999_mutation_to_dsl(mutation: &En1999Mutation) -> En1999MutationDsl {
    match mutation {
        En1999Mutation::ChangeNEdKn(payload) => En1999MutationDsl::ChangeNEdKn { new_n_ed_kn: payload.new_n_ed_kn.clone() },
        En1999Mutation::ChangeMEdKnm(payload) => En1999MutationDsl::ChangeMEdKnm { new_m_ed_knm: payload.new_m_ed_knm.clone() },
        En1999Mutation::ChangeAMm2(payload) => En1999MutationDsl::ChangeAMm2 { new_a_mm2: payload.new_a_mm2.clone() },
        En1999Mutation::ChangeWElMm3(payload) => En1999MutationDsl::ChangeWElMm3 { new_w_el_mm3: payload.new_w_el_mm3.clone() },
        En1999Mutation::ChangeAlloy(payload) => En1999MutationDsl::ChangeAlloy { new_alloy: payload.new_alloy.clone() },
        En1999Mutation::ChangeChi(payload) => En1999MutationDsl::ChangeChi { new_chi: payload.new_chi.clone() },
        En1999Mutation::ChangeITMm4(payload) => En1999MutationDsl::ChangeITMm4 { new_i_t_mm4: payload.new_i_t_mm4.clone() },
        En1999Mutation::ChangeLCrMm(payload) => En1999MutationDsl::ChangeLCrMm { new_l_cr_mm: payload.new_l_cr_mm.clone() },
        En1999Mutation::ChangeThetaC(payload) => En1999MutationDsl::ChangeThetaC { new_theta_c: payload.new_theta_c.clone() },
        En1999Mutation::ChangeDeltaSigmaEd(payload) => En1999MutationDsl::ChangeDeltaSigmaEd { new_delta_sigma_ed: payload.new_delta_sigma_ed.clone() },
        En1999Mutation::ChangeDeltaSigmaC(payload) => En1999MutationDsl::ChangeDeltaSigmaC { new_delta_sigma_c: payload.new_delta_sigma_c.clone() },
        En1999Mutation::ChangeFatigueM(payload) => En1999MutationDsl::ChangeFatigueM { new_fatigue_m: payload.new_fatigue_m.clone() },
        En1999Mutation::ChangeNCycles(payload) => En1999MutationDsl::ChangeNCycles { new_n_cycles: payload.new_n_cycles.clone() },
        En1999Mutation::ChangeVWeldEdKn(payload) => En1999MutationDsl::ChangeVWeldEdKn { new_v_weld_ed_kn: payload.new_v_weld_ed_kn.clone() },
        En1999Mutation::ChangeWeldThroatMm(payload) => En1999MutationDsl::ChangeWeldThroatMm { new_weld_throat_mm: payload.new_weld_throat_mm.clone() },
        En1999Mutation::ChangeWeldLengthMm(payload) => En1999MutationDsl::ChangeWeldLengthMm { new_weld_length_mm: payload.new_weld_length_mm.clone() },
        En1999Mutation::ChangeBetaW(payload) => En1999MutationDsl::ChangeBetaW { new_beta_w: payload.new_beta_w.clone() },
        En1999Mutation::ChangeSheetBMm(payload) => En1999MutationDsl::ChangeSheetBMm { new_sheet_b_mm: payload.new_sheet_b_mm.clone() },
        En1999Mutation::ChangeSheetTMm(payload) => En1999MutationDsl::ChangeSheetTMm { new_sheet_t_mm: payload.new_sheet_t_mm.clone() },
        En1999Mutation::ChangeSheetKSigma(payload) => En1999MutationDsl::ChangeSheetKSigma { new_sheet_k_sigma: payload.new_sheet_k_sigma.clone() },
        En1999Mutation::ChangeSheetWElMm3(payload) => En1999MutationDsl::ChangeSheetWElMm3 { new_sheet_w_el_mm3: payload.new_sheet_w_el_mm3.clone() },
        En1999Mutation::ChangeSheetMEdKnm(payload) => En1999MutationDsl::ChangeSheetMEdKnm { new_sheet_m_ed_knm: payload.new_sheet_m_ed_knm.clone() },
        En1999Mutation::ChangeShellTMm(payload) => En1999MutationDsl::ChangeShellTMm { new_shell_t_mm: payload.new_shell_t_mm.clone() },
        En1999Mutation::ChangeShellRMm(payload) => En1999MutationDsl::ChangeShellRMm { new_shell_r_mm: payload.new_shell_r_mm.clone() },
        En1999Mutation::ChangeSigmaEdShellMpa(payload) => En1999MutationDsl::ChangeSigmaEdShellMpa { new_sigma_ed_shell_mpa: payload.new_sigma_ed_shell_mpa.clone() },
        En1999Mutation::ChangeAnnex(payload) => En1999MutationDsl::ChangeAnnex { new_annex: payload.new_annex.clone() },
    }
}

fn en1999_mutation_from_dsl(mutation: En1999MutationDsl) -> En1999Mutation {
    match mutation {
        En1999MutationDsl::ChangeNEdKn { new_n_ed_kn } => En1999Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn }),
        En1999MutationDsl::ChangeMEdKnm { new_m_ed_knm } => En1999Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm }),
        En1999MutationDsl::ChangeAMm2 { new_a_mm2 } => En1999Mutation::ChangeAMm2(change_a_mm2::ChangeAMm2 { new_a_mm2 }),
        En1999MutationDsl::ChangeWElMm3 { new_w_el_mm3 } => En1999Mutation::ChangeWElMm3(change_w_el_mm3::ChangeWElMm3 { new_w_el_mm3 }),
        En1999MutationDsl::ChangeAlloy { new_alloy } => En1999Mutation::ChangeAlloy(change_alloy::ChangeAlloy { new_alloy }),
        En1999MutationDsl::ChangeChi { new_chi } => En1999Mutation::ChangeChi(change_chi::ChangeChi { new_chi }),
        En1999MutationDsl::ChangeITMm4 { new_i_t_mm4 } => En1999Mutation::ChangeITMm4(change_i_t_mm4::ChangeITMm4 { new_i_t_mm4 }),
        En1999MutationDsl::ChangeLCrMm { new_l_cr_mm } => En1999Mutation::ChangeLCrMm(change_l_cr_mm::ChangeLCrMm { new_l_cr_mm }),
        En1999MutationDsl::ChangeThetaC { new_theta_c } => En1999Mutation::ChangeThetaC(change_theta_c::ChangeThetaC { new_theta_c }),
        En1999MutationDsl::ChangeDeltaSigmaEd { new_delta_sigma_ed } => En1999Mutation::ChangeDeltaSigmaEd(change_delta_sigma_ed::ChangeDeltaSigmaEd { new_delta_sigma_ed }),
        En1999MutationDsl::ChangeDeltaSigmaC { new_delta_sigma_c } => En1999Mutation::ChangeDeltaSigmaC(change_delta_sigma_c::ChangeDeltaSigmaC { new_delta_sigma_c }),
        En1999MutationDsl::ChangeFatigueM { new_fatigue_m } => En1999Mutation::ChangeFatigueM(change_fatigue_m::ChangeFatigueM { new_fatigue_m }),
        En1999MutationDsl::ChangeNCycles { new_n_cycles } => En1999Mutation::ChangeNCycles(change_n_cycles::ChangeNCycles { new_n_cycles }),
        En1999MutationDsl::ChangeVWeldEdKn { new_v_weld_ed_kn } => En1999Mutation::ChangeVWeldEdKn(change_v_weld_ed_kn::ChangeVWeldEdKn { new_v_weld_ed_kn }),
        En1999MutationDsl::ChangeWeldThroatMm { new_weld_throat_mm } => En1999Mutation::ChangeWeldThroatMm(change_weld_throat_mm::ChangeWeldThroatMm { new_weld_throat_mm }),
        En1999MutationDsl::ChangeWeldLengthMm { new_weld_length_mm } => En1999Mutation::ChangeWeldLengthMm(change_weld_length_mm::ChangeWeldLengthMm { new_weld_length_mm }),
        En1999MutationDsl::ChangeBetaW { new_beta_w } => En1999Mutation::ChangeBetaW(change_beta_w::ChangeBetaW { new_beta_w }),
        En1999MutationDsl::ChangeSheetBMm { new_sheet_b_mm } => En1999Mutation::ChangeSheetBMm(change_sheet_b_mm::ChangeSheetBMm { new_sheet_b_mm }),
        En1999MutationDsl::ChangeSheetTMm { new_sheet_t_mm } => En1999Mutation::ChangeSheetTMm(change_sheet_t_mm::ChangeSheetTMm { new_sheet_t_mm }),
        En1999MutationDsl::ChangeSheetKSigma { new_sheet_k_sigma } => En1999Mutation::ChangeSheetKSigma(change_sheet_k_sigma::ChangeSheetKSigma { new_sheet_k_sigma }),
        En1999MutationDsl::ChangeSheetWElMm3 { new_sheet_w_el_mm3 } => En1999Mutation::ChangeSheetWElMm3(change_sheet_w_el_mm3::ChangeSheetWElMm3 { new_sheet_w_el_mm3 }),
        En1999MutationDsl::ChangeSheetMEdKnm { new_sheet_m_ed_knm } => En1999Mutation::ChangeSheetMEdKnm(change_sheet_m_ed_knm::ChangeSheetMEdKnm { new_sheet_m_ed_knm }),
        En1999MutationDsl::ChangeShellTMm { new_shell_t_mm } => En1999Mutation::ChangeShellTMm(change_shell_t_mm::ChangeShellTMm { new_shell_t_mm }),
        En1999MutationDsl::ChangeShellRMm { new_shell_r_mm } => En1999Mutation::ChangeShellRMm(change_shell_r_mm::ChangeShellRMm { new_shell_r_mm }),
        En1999MutationDsl::ChangeSigmaEdShellMpa { new_sigma_ed_shell_mpa } => En1999Mutation::ChangeSigmaEdShellMpa(change_sigma_ed_shell_mpa::ChangeSigmaEdShellMpa { new_sigma_ed_shell_mpa }),
        En1999MutationDsl::ChangeAnnex { new_annex } => En1999Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex }),
    }
}

impl OpText for En1999Mutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(en1999_mutation_from_dsl(<En1999MutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <En1999MutationDsl as OpText>::print_op(&en1999_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `En1999MutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for En1999Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        en1999_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(en1999_mutation_from_dsl(En1999MutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn op_text_round_trips_change_n_ed_kn() {
        store::os_store::test_support::assert_op_line_round_trip(&En1999Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: 95.0 }));
    }

    #[semio_framework_async_macros::async_test]
    fn op_text_round_trips_change_annex() {
        store::os_store::test_support::assert_op_line_round_trip(&En1999Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }));
    }

    #[semio_framework_async_macros::async_test]
    fn op_text_round_trips_change_alloy() {
        store::os_store::test_support::assert_op_line_round_trip(&En1999Mutation::ChangeAlloy(change_alloy::ChangeAlloy { new_alloy: "aw6082t6".to_string() }));
    }

    /// ⚖️ Every variant, not just the hand-picked ones above — full-coverage `OpText` round trip
    /// over the closed vocabulary, one sample value per field.
    #[semio_framework_async_macros::async_test]
    fn every_variant_op_text_round_trips() {
        for mutation in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&mutation);
        }
    }

    fn every_mutation() -> Vec<En1999Mutation> {
        vec![
            En1999Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: 95.0 }),
            En1999Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 5.0 }),
            En1999Mutation::ChangeAMm2(change_a_mm2::ChangeAMm2 { new_a_mm2: 1300.0 }),
            En1999Mutation::ChangeWElMm3(change_w_el_mm3::ChangeWElMm3 { new_w_el_mm3: 26_000.0 }),
            En1999Mutation::ChangeAlloy(change_alloy::ChangeAlloy { new_alloy: "aw6082t6".to_string() }),
            En1999Mutation::ChangeChi(change_chi::ChangeChi { new_chi: 0.8 }),
            En1999Mutation::ChangeITMm4(change_i_t_mm4::ChangeITMm4 { new_i_t_mm4: 5400.0 }),
            En1999Mutation::ChangeLCrMm(change_l_cr_mm::ChangeLCrMm { new_l_cr_mm: 3200.0 }),
            En1999Mutation::ChangeThetaC(change_theta_c::ChangeThetaC { new_theta_c: 180.0 }),
            En1999Mutation::ChangeDeltaSigmaEd(change_delta_sigma_ed::ChangeDeltaSigmaEd { new_delta_sigma_ed: 50.0 }),
            En1999Mutation::ChangeDeltaSigmaC(change_delta_sigma_c::ChangeDeltaSigmaC { new_delta_sigma_c: 80.0 }),
            En1999Mutation::ChangeFatigueM(change_fatigue_m::ChangeFatigueM { new_fatigue_m: 5.0 }),
            En1999Mutation::ChangeNCycles(change_n_cycles::ChangeNCycles { new_n_cycles: 600_000.0 }),
            En1999Mutation::ChangeVWeldEdKn(change_v_weld_ed_kn::ChangeVWeldEdKn { new_v_weld_ed_kn: 28.0 }),
            En1999Mutation::ChangeWeldThroatMm(change_weld_throat_mm::ChangeWeldThroatMm { new_weld_throat_mm: 5.0 }),
            En1999Mutation::ChangeWeldLengthMm(change_weld_length_mm::ChangeWeldLengthMm { new_weld_length_mm: 140.0 }),
            En1999Mutation::ChangeBetaW(change_beta_w::ChangeBetaW { new_beta_w: 0.7 }),
            En1999Mutation::ChangeSheetBMm(change_sheet_b_mm::ChangeSheetBMm { new_sheet_b_mm: 220.0 }),
            En1999Mutation::ChangeSheetTMm(change_sheet_t_mm::ChangeSheetTMm { new_sheet_t_mm: 2.5 }),
            En1999Mutation::ChangeSheetKSigma(change_sheet_k_sigma::ChangeSheetKSigma { new_sheet_k_sigma: 4.2 }),
            En1999Mutation::ChangeSheetWElMm3(change_sheet_w_el_mm3::ChangeSheetWElMm3 { new_sheet_w_el_mm3: 8500.0 }),
            En1999Mutation::ChangeSheetMEdKnm(change_sheet_m_ed_knm::ChangeSheetMEdKnm { new_sheet_m_ed_knm: 0.6 }),
            En1999Mutation::ChangeShellTMm(change_shell_t_mm::ChangeShellTMm { new_shell_t_mm: 4.5 }),
            En1999Mutation::ChangeShellRMm(change_shell_r_mm::ChangeShellRMm { new_shell_r_mm: 520.0 }),
            En1999Mutation::ChangeSigmaEdShellMpa(change_sigma_ed_shell_mpa::ChangeSigmaEdShellMpa { new_sigma_ed_shell_mpa: 160.0 }),
            En1999Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        ]
    }
}
//#endregion 🧪️Tests
