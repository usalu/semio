//! 🔧️ En1995 artifact — OpText/OpBinary codecs for `En1995Mutation`. Mutation apply/inverse
//! live in `🧬️mutations`; this facet only handcrafts the op wire forms (the shared
//! whole-document-replace macro no longer applies now that the whole-document-replace variant is
//! gone).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::en1995::schema::mutations::En1995Mutation;
use crate::artifacts::en1995::schema::mutations::{
    change_a_ef_mm2, change_a_mm2, change_a_vert_m_s2, change_b_mm, change_f_c_0_k, change_f_ed_kn, change_f_m_k, change_f_v_k, change_fire_duration_min, change_h_mm, change_load_duration, change_m_crit_knm, change_m_ed_knm, change_n_cycles_bridge,
    change_n_ed_kn, change_section_depth_mm, change_service_class, change_v_ed_kn, change_w_mm3, set_snapshot,
};
use crate::document::AnnexChoice;

use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `En1995Mutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only; `En1995Mutation` itself,
/// and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum En1995MutationDsl {
    ChangeAnnex { new_annex: AnnexChoice },
    ChangeMEdKnm { new_m_ed_knm: f64 },
    ChangeNEdKn { new_n_ed_kn: f64 },
    ChangeVEdKn { new_v_ed_kn: f64 },
    ChangeWMm3 { new_w_mm3: f64 },
    ChangeAMm2 { new_a_mm2: f64 },
    ChangeBMm { new_b_mm: f64 },
    ChangeHMm { new_h_mm: f64 },
    ChangeFMK { new_f_m_k: f64 },
    ChangeFC0K { new_f_c_0_k: f64 },
    ChangeServiceClass { new_service_class: String },
    ChangeLoadDuration { new_load_duration: String },
    ChangeMCritKnm { new_m_crit_knm: f64 },
    ChangeFEdKn { new_f_ed_kn: f64 },
    ChangeAEfMm2 { new_a_ef_mm2: f64 },
    ChangeFVK { new_f_v_k: f64 },
    ChangeFireDurationMin { new_fire_duration_min: f64 },
    ChangeSectionDepthMm { new_section_depth_mm: f64 },
    ChangeAVertMS2 { new_a_vert_m_s2: f64 },
    ChangeNCyclesBridge { new_n_cycles_bridge: f64 },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for En1995MutationDsl {
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

impl protocol::OpBinary for En1995MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn en1995_mutation_to_dsl(mutation: &En1995Mutation) -> En1995MutationDsl {
    match mutation {
        En1995Mutation::ChangeAnnex(payload) => En1995MutationDsl::ChangeAnnex { new_annex: payload.new_annex.clone() },
        En1995Mutation::ChangeMEdKnm(payload) => En1995MutationDsl::ChangeMEdKnm { new_m_ed_knm: payload.new_m_ed_knm.clone() },
        En1995Mutation::ChangeNEdKn(payload) => En1995MutationDsl::ChangeNEdKn { new_n_ed_kn: payload.new_n_ed_kn.clone() },
        En1995Mutation::ChangeVEdKn(payload) => En1995MutationDsl::ChangeVEdKn { new_v_ed_kn: payload.new_v_ed_kn.clone() },
        En1995Mutation::ChangeWMm3(payload) => En1995MutationDsl::ChangeWMm3 { new_w_mm3: payload.new_w_mm3.clone() },
        En1995Mutation::ChangeAMm2(payload) => En1995MutationDsl::ChangeAMm2 { new_a_mm2: payload.new_a_mm2.clone() },
        En1995Mutation::ChangeBMm(payload) => En1995MutationDsl::ChangeBMm { new_b_mm: payload.new_b_mm.clone() },
        En1995Mutation::ChangeHMm(payload) => En1995MutationDsl::ChangeHMm { new_h_mm: payload.new_h_mm.clone() },
        En1995Mutation::ChangeFMK(payload) => En1995MutationDsl::ChangeFMK { new_f_m_k: payload.new_f_m_k.clone() },
        En1995Mutation::ChangeFC0K(payload) => En1995MutationDsl::ChangeFC0K { new_f_c_0_k: payload.new_f_c_0_k.clone() },
        En1995Mutation::ChangeServiceClass(payload) => En1995MutationDsl::ChangeServiceClass { new_service_class: payload.new_service_class.clone() },
        En1995Mutation::ChangeLoadDuration(payload) => En1995MutationDsl::ChangeLoadDuration { new_load_duration: payload.new_load_duration.clone() },
        En1995Mutation::ChangeMCritKnm(payload) => En1995MutationDsl::ChangeMCritKnm { new_m_crit_knm: payload.new_m_crit_knm.clone() },
        En1995Mutation::ChangeFEdKn(payload) => En1995MutationDsl::ChangeFEdKn { new_f_ed_kn: payload.new_f_ed_kn.clone() },
        En1995Mutation::ChangeAEfMm2(payload) => En1995MutationDsl::ChangeAEfMm2 { new_a_ef_mm2: payload.new_a_ef_mm2.clone() },
        En1995Mutation::ChangeFVK(payload) => En1995MutationDsl::ChangeFVK { new_f_v_k: payload.new_f_v_k.clone() },
        En1995Mutation::ChangeFireDurationMin(payload) => En1995MutationDsl::ChangeFireDurationMin { new_fire_duration_min: payload.new_fire_duration_min.clone() },
        En1995Mutation::ChangeSectionDepthMm(payload) => En1995MutationDsl::ChangeSectionDepthMm { new_section_depth_mm: payload.new_section_depth_mm.clone() },
        En1995Mutation::ChangeAVertMS2(payload) => En1995MutationDsl::ChangeAVertMS2 { new_a_vert_m_s2: payload.new_a_vert_m_s2.clone() },
        En1995Mutation::ChangeNCyclesBridge(payload) => En1995MutationDsl::ChangeNCyclesBridge { new_n_cycles_bridge: payload.new_n_cycles_bridge.clone() },
    }
}

fn en1995_mutation_from_dsl(mutation: En1995MutationDsl) -> En1995Mutation {
    match mutation {
        En1995MutationDsl::ChangeAnnex { new_annex } => En1995Mutation::ChangeAnnex(set_snapshot::ChangeAnnex { new_annex }),
        En1995MutationDsl::ChangeMEdKnm { new_m_ed_knm } => En1995Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm }),
        En1995MutationDsl::ChangeNEdKn { new_n_ed_kn } => En1995Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn }),
        En1995MutationDsl::ChangeVEdKn { new_v_ed_kn } => En1995Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn }),
        En1995MutationDsl::ChangeWMm3 { new_w_mm3 } => En1995Mutation::ChangeWMm3(change_w_mm3::ChangeWMm3 { new_w_mm3 }),
        En1995MutationDsl::ChangeAMm2 { new_a_mm2 } => En1995Mutation::ChangeAMm2(change_a_mm2::ChangeAMm2 { new_a_mm2 }),
        En1995MutationDsl::ChangeBMm { new_b_mm } => En1995Mutation::ChangeBMm(change_b_mm::ChangeBMm { new_b_mm }),
        En1995MutationDsl::ChangeHMm { new_h_mm } => En1995Mutation::ChangeHMm(change_h_mm::ChangeHMm { new_h_mm }),
        En1995MutationDsl::ChangeFMK { new_f_m_k } => En1995Mutation::ChangeFMK(change_f_m_k::ChangeFMK { new_f_m_k }),
        En1995MutationDsl::ChangeFC0K { new_f_c_0_k } => En1995Mutation::ChangeFC0K(change_f_c_0_k::ChangeFC0K { new_f_c_0_k }),
        En1995MutationDsl::ChangeServiceClass { new_service_class } => En1995Mutation::ChangeServiceClass(change_service_class::ChangeServiceClass { new_service_class }),
        En1995MutationDsl::ChangeLoadDuration { new_load_duration } => En1995Mutation::ChangeLoadDuration(change_load_duration::ChangeLoadDuration { new_load_duration }),
        En1995MutationDsl::ChangeMCritKnm { new_m_crit_knm } => En1995Mutation::ChangeMCritKnm(change_m_crit_knm::ChangeMCritKnm { new_m_crit_knm }),
        En1995MutationDsl::ChangeFEdKn { new_f_ed_kn } => En1995Mutation::ChangeFEdKn(change_f_ed_kn::ChangeFEdKn { new_f_ed_kn }),
        En1995MutationDsl::ChangeAEfMm2 { new_a_ef_mm2 } => En1995Mutation::ChangeAEfMm2(change_a_ef_mm2::ChangeAEfMm2 { new_a_ef_mm2 }),
        En1995MutationDsl::ChangeFVK { new_f_v_k } => En1995Mutation::ChangeFVK(change_f_v_k::ChangeFVK { new_f_v_k }),
        En1995MutationDsl::ChangeFireDurationMin { new_fire_duration_min } => En1995Mutation::ChangeFireDurationMin(change_fire_duration_min::ChangeFireDurationMin { new_fire_duration_min }),
        En1995MutationDsl::ChangeSectionDepthMm { new_section_depth_mm } => En1995Mutation::ChangeSectionDepthMm(change_section_depth_mm::ChangeSectionDepthMm { new_section_depth_mm }),
        En1995MutationDsl::ChangeAVertMS2 { new_a_vert_m_s2 } => En1995Mutation::ChangeAVertMS2(change_a_vert_m_s2::ChangeAVertMS2 { new_a_vert_m_s2 }),
        En1995MutationDsl::ChangeNCyclesBridge { new_n_cycles_bridge } => En1995Mutation::ChangeNCyclesBridge(change_n_cycles_bridge::ChangeNCyclesBridge { new_n_cycles_bridge }),
    }
}

impl OpText for En1995Mutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(en1995_mutation_from_dsl(<En1995MutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <En1995MutationDsl as OpText>::print_op(&en1995_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `En1995MutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for En1995Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        en1995_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(en1995_mutation_from_dsl(En1995MutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// ⚖️ Every variant — full-coverage `OpText` round trip over the closed vocabulary, one sample
    /// value per field.
    #[semio_framework_async_macros::async_test]
    fn every_variant_op_text_round_trips() {
        for mutation in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&mutation);
        }
    }

    fn every_mutation() -> Vec<En1995Mutation> {
        vec![
            En1995Mutation::ChangeAnnex(set_snapshot::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
            En1995Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 25.0 }),
            En1995Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: 50.0 }),
            En1995Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 15.0 }),
            En1995Mutation::ChangeWMm3(change_w_mm3::ChangeWMm3 { new_w_mm3: 1_000_000.0 }),
            En1995Mutation::ChangeAMm2(change_a_mm2::ChangeAMm2 { new_a_mm2: 20_000.0 }),
            En1995Mutation::ChangeBMm(change_b_mm::ChangeBMm { new_b_mm: 200.0 }),
            En1995Mutation::ChangeHMm(change_h_mm::ChangeHMm { new_h_mm: 300.0 }),
            En1995Mutation::ChangeFMK(change_f_m_k::ChangeFMK { new_f_m_k: 24.0 }),
            En1995Mutation::ChangeFC0K(change_f_c_0_k::ChangeFC0K { new_f_c_0_k: 21.0 }),
            En1995Mutation::ChangeServiceClass(change_service_class::ChangeServiceClass { new_service_class: "sc1".into() }),
            En1995Mutation::ChangeLoadDuration(change_load_duration::ChangeLoadDuration { new_load_duration: "medium".into() }),
            En1995Mutation::ChangeMCritKnm(change_m_crit_knm::ChangeMCritKnm { new_m_crit_knm: 80.0 }),
            En1995Mutation::ChangeFEdKn(change_f_ed_kn::ChangeFEdKn { new_f_ed_kn: 18.0 }),
            En1995Mutation::ChangeAEfMm2(change_a_ef_mm2::ChangeAEfMm2 { new_a_ef_mm2: 12_000.0 }),
            En1995Mutation::ChangeFVK(change_f_v_k::ChangeFVK { new_f_v_k: 4.0 }),
            En1995Mutation::ChangeFireDurationMin(change_fire_duration_min::ChangeFireDurationMin { new_fire_duration_min: 30.0 }),
            En1995Mutation::ChangeSectionDepthMm(change_section_depth_mm::ChangeSectionDepthMm { new_section_depth_mm: 300.0 }),
            En1995Mutation::ChangeAVertMS2(change_a_vert_m_s2::ChangeAVertMS2 { new_a_vert_m_s2: 0.3 }),
            En1995Mutation::ChangeNCyclesBridge(change_n_cycles_bridge::ChangeNCyclesBridge { new_n_cycles_bridge: 500_000.0 }),
        ]
    }
}
//#endregion 🧪️Tests
