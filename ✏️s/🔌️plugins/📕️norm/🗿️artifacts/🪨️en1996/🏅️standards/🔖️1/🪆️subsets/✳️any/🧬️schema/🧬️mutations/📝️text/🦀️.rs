//! 🔧️ En1996 artifact — OpText/OpBinary codecs for `En1996Mutation`. Mutation apply/inverse
//! live in `🧬️mutations`; this facet only handcrafts the op wire forms (the shared
//! whole-document-replace macro, `impl_norm_set_snapshot_ops!`, no longer applies now that the
//! whole-document-replace variant is gone).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::document::AnnexChoice;
use crate::document::DesignSituation;
pub use crate::document_schema::mutations::En1996Mutation;
use crate::document_schema::mutations::{
    change_annex, change_area_mm2, change_bed_joint_thickness_mm, change_design_situation, change_exposure, change_f_k_mpa, change_f_vk_mpa, change_fire_resistance_min, change_h_ed_kn, change_h_ef_mm, change_m_ed_knm, change_masonry_class,
    change_mortar, change_mu, change_n_ed_kn, change_shear_area_mm2, change_storeys, change_t_ef_mm, change_unit, change_v_ed_kn, change_wall_thickness_mm, change_z_mm3,
};
use crate::part_2::ExposureClass;
use crate::part_2::MortarClass;
use crate::MasonryClass;
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

impl protocol::OpBinary for En1996MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn en1996_mutation_to_dsl(mutation: &En1996Mutation) -> En1996MutationDsl {
    match mutation {
        En1996Mutation::ChangeMEdKnm(payload) => En1996MutationDsl::ChangeMEdKnm { new_m_ed_knm: payload.new_m_ed_knm },
        En1996Mutation::ChangeNEdKn(payload) => En1996MutationDsl::ChangeNEdKn { new_n_ed_kn: payload.new_n_ed_kn },
        En1996Mutation::ChangeVEdKn(payload) => En1996MutationDsl::ChangeVEdKn { new_v_ed_kn: payload.new_v_ed_kn },
        En1996Mutation::ChangeHEdKn(payload) => En1996MutationDsl::ChangeHEdKn { new_h_ed_kn: payload.new_h_ed_kn },
        En1996Mutation::ChangeZMm3(payload) => En1996MutationDsl::ChangeZMm3 { new_z_mm3: payload.new_z_mm3 },
        En1996Mutation::ChangeAreaMm2(payload) => En1996MutationDsl::ChangeAreaMm2 { new_area_mm2: payload.new_area_mm2 },
        En1996Mutation::ChangeShearAreaMm2(payload) => En1996MutationDsl::ChangeShearAreaMm2 { new_shear_area_mm2: payload.new_shear_area_mm2 },
        En1996Mutation::ChangeFKMpa(payload) => En1996MutationDsl::ChangeFKMpa { new_f_k_mpa: payload.new_f_k_mpa },
        En1996Mutation::ChangeFVkMpa(payload) => En1996MutationDsl::ChangeFVkMpa { new_f_vk_mpa: payload.new_f_vk_mpa },
        En1996Mutation::ChangeAnnex(payload) => En1996MutationDsl::ChangeAnnex { new_annex: payload.new_annex },
        En1996Mutation::ChangeMasonryClass(payload) => En1996MutationDsl::ChangeMasonryClass { new_masonry_class: payload.new_masonry_class },
        En1996Mutation::ChangeDesignSituation(payload) => En1996MutationDsl::ChangeDesignSituation { new_design_situation: payload.new_design_situation },
        En1996Mutation::ChangeMu(payload) => En1996MutationDsl::ChangeMu { new_mu: payload.new_mu },
        En1996Mutation::ChangeWallThicknessMm(payload) => En1996MutationDsl::ChangeWallThicknessMm { new_wall_thickness_mm: payload.new_wall_thickness_mm },
        En1996Mutation::ChangeFireResistanceMin(payload) => En1996MutationDsl::ChangeFireResistanceMin { new_fire_resistance_min: payload.new_fire_resistance_min },
        En1996Mutation::ChangeUnit(payload) => En1996MutationDsl::ChangeUnit { new_unit: payload.new_unit.clone() },
        En1996Mutation::ChangeExposure(payload) => En1996MutationDsl::ChangeExposure { new_exposure: payload.new_exposure },
        En1996Mutation::ChangeMortar(payload) => En1996MutationDsl::ChangeMortar { new_mortar: payload.new_mortar },
        En1996Mutation::ChangeBedJointThicknessMm(payload) => En1996MutationDsl::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm: payload.new_bed_joint_thickness_mm },
        En1996Mutation::ChangeStoreys(payload) => En1996MutationDsl::ChangeStoreys { new_storeys: payload.new_storeys },
        En1996Mutation::ChangeHEfMm(payload) => En1996MutationDsl::ChangeHEfMm { new_h_ef_mm: payload.new_h_ef_mm },
        En1996Mutation::ChangeTEfMm(payload) => En1996MutationDsl::ChangeTEfMm { new_t_ef_mm: payload.new_t_ef_mm },
    }
}

fn en1996_mutation_from_dsl(mutation: En1996MutationDsl) -> En1996Mutation {
    match mutation {
        En1996MutationDsl::ChangeMEdKnm { new_m_ed_knm } => En1996Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm }),
        En1996MutationDsl::ChangeNEdKn { new_n_ed_kn } => En1996Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn }),
        En1996MutationDsl::ChangeVEdKn { new_v_ed_kn } => En1996Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn }),
        En1996MutationDsl::ChangeHEdKn { new_h_ed_kn } => En1996Mutation::ChangeHEdKn(change_h_ed_kn::ChangeHEdKn { new_h_ed_kn }),
        En1996MutationDsl::ChangeZMm3 { new_z_mm3 } => En1996Mutation::ChangeZMm3(change_z_mm3::ChangeZMm3 { new_z_mm3 }),
        En1996MutationDsl::ChangeAreaMm2 { new_area_mm2 } => En1996Mutation::ChangeAreaMm2(change_area_mm2::ChangeAreaMm2 { new_area_mm2 }),
        En1996MutationDsl::ChangeShearAreaMm2 { new_shear_area_mm2 } => En1996Mutation::ChangeShearAreaMm2(change_shear_area_mm2::ChangeShearAreaMm2 { new_shear_area_mm2 }),
        En1996MutationDsl::ChangeFKMpa { new_f_k_mpa } => En1996Mutation::ChangeFKMpa(change_f_k_mpa::ChangeFKMpa { new_f_k_mpa }),
        En1996MutationDsl::ChangeFVkMpa { new_f_vk_mpa } => En1996Mutation::ChangeFVkMpa(change_f_vk_mpa::ChangeFVkMpa { new_f_vk_mpa }),
        En1996MutationDsl::ChangeAnnex { new_annex } => En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex }),
        En1996MutationDsl::ChangeMasonryClass { new_masonry_class } => En1996Mutation::ChangeMasonryClass(change_masonry_class::ChangeMasonryClass { new_masonry_class }),
        En1996MutationDsl::ChangeDesignSituation { new_design_situation } => En1996Mutation::ChangeDesignSituation(change_design_situation::ChangeDesignSituation { new_design_situation }),
        En1996MutationDsl::ChangeMu { new_mu } => En1996Mutation::ChangeMu(change_mu::ChangeMu { new_mu }),
        En1996MutationDsl::ChangeWallThicknessMm { new_wall_thickness_mm } => En1996Mutation::ChangeWallThicknessMm(change_wall_thickness_mm::ChangeWallThicknessMm { new_wall_thickness_mm }),
        En1996MutationDsl::ChangeFireResistanceMin { new_fire_resistance_min } => En1996Mutation::ChangeFireResistanceMin(change_fire_resistance_min::ChangeFireResistanceMin { new_fire_resistance_min }),
        En1996MutationDsl::ChangeUnit { new_unit } => En1996Mutation::ChangeUnit(change_unit::ChangeUnit { new_unit }),
        En1996MutationDsl::ChangeExposure { new_exposure } => En1996Mutation::ChangeExposure(change_exposure::ChangeExposure { new_exposure }),
        En1996MutationDsl::ChangeMortar { new_mortar } => En1996Mutation::ChangeMortar(change_mortar::ChangeMortar { new_mortar }),
        En1996MutationDsl::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm } => En1996Mutation::ChangeBedJointThicknessMm(change_bed_joint_thickness_mm::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm }),
        En1996MutationDsl::ChangeStoreys { new_storeys } => En1996Mutation::ChangeStoreys(change_storeys::ChangeStoreys { new_storeys }),
        En1996MutationDsl::ChangeHEfMm { new_h_ef_mm } => En1996Mutation::ChangeHEfMm(change_h_ef_mm::ChangeHEfMm { new_h_ef_mm }),
        En1996MutationDsl::ChangeTEfMm { new_t_ef_mm } => En1996Mutation::ChangeTEfMm(change_t_ef_mm::ChangeTEfMm { new_t_ef_mm }),
    }
}

impl OpText for En1996Mutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(en1996_mutation_from_dsl(<En1996MutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <En1996MutationDsl as OpText>::print_op(&en1996_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `En1996MutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for En1996Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        en1996_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(en1996_mutation_from_dsl(En1996MutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
