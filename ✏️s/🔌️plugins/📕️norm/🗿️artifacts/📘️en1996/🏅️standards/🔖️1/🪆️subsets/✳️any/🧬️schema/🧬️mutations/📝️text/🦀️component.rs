//! 🔧️ En1996 artifact — OpText/OpBinary codecs for `En1996Mutation`. Mutation apply/inverse
//! live in `🧬️mutations`; this facet only handcrafts the op wire forms (the shared
//! whole-document-replace macro, `impl_norm_set_snapshot_ops!`, no longer applies now that the
//! whole-document-replace variant is gone).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::en1996::part_2::ExposureClass;
use crate::artifacts::en1996::part_2::MortarClass;
pub use crate::artifacts::en1996::schema::mutations::En1996Mutation;
use crate::artifacts::en1996::schema::mutations::{
    change_annex, change_area_mm2, change_bed_joint_thickness_mm, change_design_situation, change_exposure, change_f_k_mpa, change_f_vk_mpa, change_fire_resistance_min, change_h_ed_kn, change_h_ef_mm, change_m_ed_knm, change_masonry_class,
    change_mortar, change_mu, change_n_ed_kn, change_shear_area_mm2, change_storeys, change_t_ef_mm, change_unit, change_v_ed_kn, change_wall_thickness_mm, change_z_mm3,
};
use crate::artifacts::en1996::MasonryClass;
use crate::document::AnnexChoice;
use crate::document::DesignSituation;
use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `En1996Mutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only; `En1996Mutation` itself,
/// and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum En1996MutationDsl {
    ChangeMEdKnm { new_m_ed_knm: f64 },
    ChangeNEdKn { new_n_ed_kn: f64 },
    ChangeVEdKn { new_v_ed_kn: f64 },
    ChangeHEdKn { new_h_ed_kn: f64 },
    ChangeZMm3 { new_z_mm3: f64 },
    ChangeAreaMm2 { new_area_mm2: f64 },
    ChangeShearAreaMm2 { new_shear_area_mm2: f64 },
    ChangeFKMpa { new_f_k_mpa: f64 },
    ChangeFVkMpa { new_f_vk_mpa: f64 },
    ChangeAnnex { new_annex: AnnexChoice },
    ChangeMasonryClass { new_masonry_class: MasonryClass },
    ChangeDesignSituation { new_design_situation: DesignSituation },
    ChangeMu { new_mu: f64 },
    ChangeWallThicknessMm { new_wall_thickness_mm: f64 },
    ChangeFireResistanceMin { new_fire_resistance_min: u32 },
    ChangeUnit { new_unit: String },
    ChangeExposure { new_exposure: ExposureClass },
    ChangeMortar { new_mortar: MortarClass },
    ChangeBedJointThicknessMm { new_bed_joint_thickness_mm: f64 },
    ChangeStoreys { new_storeys: u32 },
    ChangeHEfMm { new_h_ef_mm: f64 },
    ChangeTEfMm { new_t_ef_mm: f64 },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for En1996MutationDsl {
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

impl protocol::OpBinary for En1996MutationDsl {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

async fn en1996_mutation_to_dsl(mutation: &En1996Mutation) -> En1996MutationDsl {
    match mutation {
        En1996Mutation::ChangeMEdKnm(payload) => En1996MutationDsl::ChangeMEdKnm { new_m_ed_knm: payload.new_m_ed_knm.clone() },
        En1996Mutation::ChangeNEdKn(payload) => En1996MutationDsl::ChangeNEdKn { new_n_ed_kn: payload.new_n_ed_kn.clone() },
        En1996Mutation::ChangeVEdKn(payload) => En1996MutationDsl::ChangeVEdKn { new_v_ed_kn: payload.new_v_ed_kn.clone() },
        En1996Mutation::ChangeHEdKn(payload) => En1996MutationDsl::ChangeHEdKn { new_h_ed_kn: payload.new_h_ed_kn.clone() },
        En1996Mutation::ChangeZMm3(payload) => En1996MutationDsl::ChangeZMm3 { new_z_mm3: payload.new_z_mm3.clone() },
        En1996Mutation::ChangeAreaMm2(payload) => En1996MutationDsl::ChangeAreaMm2 { new_area_mm2: payload.new_area_mm2.clone() },
        En1996Mutation::ChangeShearAreaMm2(payload) => En1996MutationDsl::ChangeShearAreaMm2 { new_shear_area_mm2: payload.new_shear_area_mm2.clone() },
        En1996Mutation::ChangeFKMpa(payload) => En1996MutationDsl::ChangeFKMpa { new_f_k_mpa: payload.new_f_k_mpa.clone() },
        En1996Mutation::ChangeFVkMpa(payload) => En1996MutationDsl::ChangeFVkMpa { new_f_vk_mpa: payload.new_f_vk_mpa.clone() },
        En1996Mutation::ChangeAnnex(payload) => En1996MutationDsl::ChangeAnnex { new_annex: payload.new_annex.clone() },
        En1996Mutation::ChangeMasonryClass(payload) => En1996MutationDsl::ChangeMasonryClass { new_masonry_class: payload.new_masonry_class.clone() },
        En1996Mutation::ChangeDesignSituation(payload) => En1996MutationDsl::ChangeDesignSituation { new_design_situation: payload.new_design_situation.clone() },
        En1996Mutation::ChangeMu(payload) => En1996MutationDsl::ChangeMu { new_mu: payload.new_mu.clone() },
        En1996Mutation::ChangeWallThicknessMm(payload) => En1996MutationDsl::ChangeWallThicknessMm { new_wall_thickness_mm: payload.new_wall_thickness_mm.clone() },
        En1996Mutation::ChangeFireResistanceMin(payload) => En1996MutationDsl::ChangeFireResistanceMin { new_fire_resistance_min: payload.new_fire_resistance_min.clone() },
        En1996Mutation::ChangeUnit(payload) => En1996MutationDsl::ChangeUnit { new_unit: payload.new_unit.clone() },
        En1996Mutation::ChangeExposure(payload) => En1996MutationDsl::ChangeExposure { new_exposure: payload.new_exposure.clone() },
        En1996Mutation::ChangeMortar(payload) => En1996MutationDsl::ChangeMortar { new_mortar: payload.new_mortar.clone() },
        En1996Mutation::ChangeBedJointThicknessMm(payload) => En1996MutationDsl::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm: payload.new_bed_joint_thickness_mm.clone() },
        En1996Mutation::ChangeStoreys(payload) => En1996MutationDsl::ChangeStoreys { new_storeys: payload.new_storeys.clone() },
        En1996Mutation::ChangeHEfMm(payload) => En1996MutationDsl::ChangeHEfMm { new_h_ef_mm: payload.new_h_ef_mm.clone() },
        En1996Mutation::ChangeTEfMm(payload) => En1996MutationDsl::ChangeTEfMm { new_t_ef_mm: payload.new_t_ef_mm.clone() },
    }
}

async fn en1996_mutation_from_dsl(mutation: En1996MutationDsl) -> En1996Mutation {
    match mutation {
        En1996MutationDsl::ChangeMEdKnm { new_m_ed_knm } => En1996Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm }),
        En1996MutationDsl::ChangeNEdKn { new_n_ed_kn } => En1996Mutation::ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn { new_n_ed_kn }),
        En1996MutationDsl::ChangeVEdKn { new_v_ed_kn } => En1996Mutation::ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn { new_v_ed_kn }),
        En1996MutationDsl::ChangeHEdKn { new_h_ed_kn } => En1996Mutation::ChangeHEdKn(change_h_ed_kn::mutation::ChangeHEdKn { new_h_ed_kn }),
        En1996MutationDsl::ChangeZMm3 { new_z_mm3 } => En1996Mutation::ChangeZMm3(change_z_mm3::mutation::ChangeZMm3 { new_z_mm3 }),
        En1996MutationDsl::ChangeAreaMm2 { new_area_mm2 } => En1996Mutation::ChangeAreaMm2(change_area_mm2::mutation::ChangeAreaMm2 { new_area_mm2 }),
        En1996MutationDsl::ChangeShearAreaMm2 { new_shear_area_mm2 } => En1996Mutation::ChangeShearAreaMm2(change_shear_area_mm2::mutation::ChangeShearAreaMm2 { new_shear_area_mm2 }),
        En1996MutationDsl::ChangeFKMpa { new_f_k_mpa } => En1996Mutation::ChangeFKMpa(change_f_k_mpa::mutation::ChangeFKMpa { new_f_k_mpa }),
        En1996MutationDsl::ChangeFVkMpa { new_f_vk_mpa } => En1996Mutation::ChangeFVkMpa(change_f_vk_mpa::mutation::ChangeFVkMpa { new_f_vk_mpa }),
        En1996MutationDsl::ChangeAnnex { new_annex } => En1996Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex }),
        En1996MutationDsl::ChangeMasonryClass { new_masonry_class } => En1996Mutation::ChangeMasonryClass(change_masonry_class::mutation::ChangeMasonryClass { new_masonry_class }),
        En1996MutationDsl::ChangeDesignSituation { new_design_situation } => En1996Mutation::ChangeDesignSituation(change_design_situation::mutation::ChangeDesignSituation { new_design_situation }),
        En1996MutationDsl::ChangeMu { new_mu } => En1996Mutation::ChangeMu(change_mu::mutation::ChangeMu { new_mu }),
        En1996MutationDsl::ChangeWallThicknessMm { new_wall_thickness_mm } => En1996Mutation::ChangeWallThicknessMm(change_wall_thickness_mm::mutation::ChangeWallThicknessMm { new_wall_thickness_mm }),
        En1996MutationDsl::ChangeFireResistanceMin { new_fire_resistance_min } => En1996Mutation::ChangeFireResistanceMin(change_fire_resistance_min::mutation::ChangeFireResistanceMin { new_fire_resistance_min }),
        En1996MutationDsl::ChangeUnit { new_unit } => En1996Mutation::ChangeUnit(change_unit::mutation::ChangeUnit { new_unit }),
        En1996MutationDsl::ChangeExposure { new_exposure } => En1996Mutation::ChangeExposure(change_exposure::mutation::ChangeExposure { new_exposure }),
        En1996MutationDsl::ChangeMortar { new_mortar } => En1996Mutation::ChangeMortar(change_mortar::mutation::ChangeMortar { new_mortar }),
        En1996MutationDsl::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm } => En1996Mutation::ChangeBedJointThicknessMm(change_bed_joint_thickness_mm::mutation::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm }),
        En1996MutationDsl::ChangeStoreys { new_storeys } => En1996Mutation::ChangeStoreys(change_storeys::mutation::ChangeStoreys { new_storeys }),
        En1996MutationDsl::ChangeHEfMm { new_h_ef_mm } => En1996Mutation::ChangeHEfMm(change_h_ef_mm::mutation::ChangeHEfMm { new_h_ef_mm }),
        En1996MutationDsl::ChangeTEfMm { new_t_ef_mm } => En1996Mutation::ChangeTEfMm(change_t_ef_mm::mutation::ChangeTEfMm { new_t_ef_mm }),
    }
}

impl OpText for En1996Mutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(en1996_mutation_from_dsl(<En1996MutationDsl as OpText>::parse_op(line)?))
    }

    async fn print_op(&self) -> String {
        <En1996MutationDsl as OpText>::print_op(&en1996_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `En1996MutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for En1996Mutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        en1996_mutation_to_dsl(self).encode_op()
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(en1996_mutation_from_dsl(En1996MutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trips_change_annex() {
        store::os_store::test_support::assert_op_line_round_trip(&En1996Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trips_change_m_ed_knm() {
        store::os_store::test_support::assert_op_line_round_trip(&En1996Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: 12.5 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trips_change_unit() {
        store::os_store::test_support::assert_op_line_round_trip(&En1996Mutation::ChangeUnit(change_unit::mutation::ChangeUnit { new_unit: "calcium_silicate".into() }));
    }

    /// ⚖️ Every variant, not just the three hand-picked above — full-coverage `OpText` round trip
    /// over the closed vocabulary, one sample value per field.
    #[semio_framework_async_macros::async_test]
    async fn every_variant_op_text_round_trips() {
        for mutation in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&mutation);
        }
    }

    async fn every_mutation() -> Vec<En1996Mutation> {
        vec![
            En1996Mutation::ChangeMEdKnm(change_m_ed_knm::mutation::ChangeMEdKnm { new_m_ed_knm: 12.5 }),
            En1996Mutation::ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn { new_n_ed_kn: 250.0 }),
            En1996Mutation::ChangeVEdKn(change_v_ed_kn::mutation::ChangeVEdKn { new_v_ed_kn: 42.0 }),
            En1996Mutation::ChangeHEdKn(change_h_ed_kn::mutation::ChangeHEdKn { new_h_ed_kn: 28.0 }),
            En1996Mutation::ChangeZMm3(change_z_mm3::mutation::ChangeZMm3 { new_z_mm3: 9_500_000.0 }),
            En1996Mutation::ChangeAreaMm2(change_area_mm2::mutation::ChangeAreaMm2 { new_area_mm2: 540_000.0 }),
            En1996Mutation::ChangeShearAreaMm2(change_shear_area_mm2::mutation::ChangeShearAreaMm2 { new_shear_area_mm2: 320_000.0 }),
            En1996Mutation::ChangeFKMpa(change_f_k_mpa::mutation::ChangeFKMpa { new_f_k_mpa: 6.5 }),
            En1996Mutation::ChangeFVkMpa(change_f_vk_mpa::mutation::ChangeFVkMpa { new_f_vk_mpa: 0.18 }),
            En1996Mutation::ChangeAnnex(change_annex::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
            En1996Mutation::ChangeMasonryClass(change_masonry_class::mutation::ChangeMasonryClass { new_masonry_class: crate::artifacts::en1996::MasonryClass::Class4 }),
            En1996Mutation::ChangeDesignSituation(change_design_situation::mutation::ChangeDesignSituation { new_design_situation: crate::document::DesignSituation::Seismic }),
            En1996Mutation::ChangeMu(change_mu::mutation::ChangeMu { new_mu: 0.35 }),
            En1996Mutation::ChangeWallThicknessMm(change_wall_thickness_mm::mutation::ChangeWallThicknessMm { new_wall_thickness_mm: 300.0 }),
            En1996Mutation::ChangeFireResistanceMin(change_fire_resistance_min::mutation::ChangeFireResistanceMin { new_fire_resistance_min: 90 }),
            En1996Mutation::ChangeUnit(change_unit::mutation::ChangeUnit { new_unit: "calcium_silicate".to_string() }),
            En1996Mutation::ChangeExposure(change_exposure::mutation::ChangeExposure { new_exposure: crate::artifacts::en1996::part_2::ExposureClass::Mx3 }),
            En1996Mutation::ChangeMortar(change_mortar::mutation::ChangeMortar { new_mortar: crate::artifacts::en1996::part_2::MortarClass::M10 }),
            En1996Mutation::ChangeBedJointThicknessMm(change_bed_joint_thickness_mm::mutation::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm: 15.0 }),
            En1996Mutation::ChangeStoreys(change_storeys::mutation::ChangeStoreys { new_storeys: 4 }),
            En1996Mutation::ChangeHEfMm(change_h_ef_mm::mutation::ChangeHEfMm { new_h_ef_mm: 2800.0 }),
            En1996Mutation::ChangeTEfMm(change_t_ef_mm::mutation::ChangeTEfMm { new_t_ef_mm: 200.0 }),
        ]
    }
}
//#endregion 🧪️Tests
