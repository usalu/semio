//! 🔧️ En1997 artifact — OpText/OpBinary codecs for `En1997Mutation`.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1997_mutations::En1997Mutation;
use semio_s_artifact_norm_en1997_mut_design::{change_annex, change_design_approach, change_design_situation, change_geotechnical_category, change_groundwater_level, change_investigation_depth};
use semio_s_artifact_norm_en1997_mut_foundations::{change_footing_embedment, change_footing_width, change_pile_count, change_pile_length, change_wall_base_width, insert_footing, insert_pile, remove_footing, remove_pile};
use semio_s_artifact_norm_en1997_mut_ground::{change_layer_oedometric_modulus, change_layer_phi_prime, change_slope_angle, insert_layer, remove_layer};
use semio_s_artifact_norm_en1997_model::{Pile, SoilLayer, SpreadFoundation};
use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `En1997Mutation`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum En1997MutationDsl {
    ChangeAnnex { new_annex: AnnexChoice },
    ChangeGeotechnicalCategory { new_geotechnical_category: u8 },
    ChangeDesignSituation { new_design_situation: String },
    ChangeDesignApproach { new_design_approach: String },
    ChangeGroundwaterLevel { new_groundwater_level: f64 },
    ChangeInvestigationDepth { new_investigation_depth: f64 },
    ChangeFootingWidth { id: String, new_width: f64 },
    ChangeFootingEmbedment { id: String, new_embedment: f64 },
    ChangePileLength { id: String, new_length: f64 },
    ChangePileCount { id: String, new_count: u32 },
    ChangeWallBaseWidth { id: String, new_base_width: f64 },
    ChangeSlopeAngle { id: String, new_angle_deg: f64 },
    ChangeLayerPhiPrime { id: String, new_phi_prime_deg: f64 },
    ChangeLayerOedometricModulus { id: String, new_oedometric_modulus: f64 },
    InsertLayer {
        index: usize,
        #[dsl(block)]
        layer: SoilLayer,
    },
    RemoveLayer { index: usize },
    InsertFooting {
        index: usize,
        #[dsl(block)]
        footing: SpreadFoundation,
    },
    RemoveFooting { index: usize },
    InsertPile {
        index: usize,
        #[dsl(block)]
        pile: Pile,
    },
    RemovePile { index: usize },
}

impl OpText for En1997MutationDsl {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

//#endregion 🔖️HandcraftedOpCodecs

pub fn en1997_mutation_to_dsl(mutation: &En1997Mutation) -> En1997MutationDsl {
    match mutation {
        En1997Mutation::ChangeAnnex(payload) => En1997MutationDsl::ChangeAnnex { new_annex: payload.new_annex },
        En1997Mutation::ChangeGeotechnicalCategory(payload) => En1997MutationDsl::ChangeGeotechnicalCategory { new_geotechnical_category: payload.new_geotechnical_category },
        En1997Mutation::ChangeDesignSituation(payload) => En1997MutationDsl::ChangeDesignSituation { new_design_situation: payload.new_design_situation.clone() },
        En1997Mutation::ChangeDesignApproach(payload) => En1997MutationDsl::ChangeDesignApproach { new_design_approach: payload.new_design_approach.clone() },
        En1997Mutation::ChangeGroundwaterLevel(payload) => En1997MutationDsl::ChangeGroundwaterLevel { new_groundwater_level: payload.new_groundwater_level },
        En1997Mutation::ChangeInvestigationDepth(payload) => En1997MutationDsl::ChangeInvestigationDepth { new_investigation_depth: payload.new_investigation_depth },
        En1997Mutation::ChangeFootingWidth(payload) => En1997MutationDsl::ChangeFootingWidth { id: payload.id.clone(), new_width: payload.new_width },
        En1997Mutation::ChangeFootingEmbedment(payload) => En1997MutationDsl::ChangeFootingEmbedment { id: payload.id.clone(), new_embedment: payload.new_embedment },
        En1997Mutation::ChangePileLength(payload) => En1997MutationDsl::ChangePileLength { id: payload.id.clone(), new_length: payload.new_length },
        En1997Mutation::ChangePileCount(payload) => En1997MutationDsl::ChangePileCount { id: payload.id.clone(), new_count: payload.new_count },
        En1997Mutation::ChangeWallBaseWidth(payload) => En1997MutationDsl::ChangeWallBaseWidth { id: payload.id.clone(), new_base_width: payload.new_base_width },
        En1997Mutation::ChangeSlopeAngle(payload) => En1997MutationDsl::ChangeSlopeAngle { id: payload.id.clone(), new_angle_deg: payload.new_angle_deg },
        En1997Mutation::ChangeLayerPhiPrime(payload) => En1997MutationDsl::ChangeLayerPhiPrime { id: payload.id.clone(), new_phi_prime_deg: payload.new_phi_prime_deg },
        En1997Mutation::ChangeLayerOedometricModulus(payload) => En1997MutationDsl::ChangeLayerOedometricModulus { id: payload.id.clone(), new_oedometric_modulus: payload.new_oedometric_modulus },
        En1997Mutation::InsertLayer(payload) => En1997MutationDsl::InsertLayer { index: payload.index, layer: payload.layer.clone() },
        En1997Mutation::RemoveLayer(payload) => En1997MutationDsl::RemoveLayer { index: payload.index },
        En1997Mutation::InsertFooting(payload) => En1997MutationDsl::InsertFooting { index: payload.index, footing: payload.footing.clone() },
        En1997Mutation::RemoveFooting(payload) => En1997MutationDsl::RemoveFooting { index: payload.index },
        En1997Mutation::InsertPile(payload) => En1997MutationDsl::InsertPile { index: payload.index, pile: payload.pile.clone() },
        En1997Mutation::RemovePile(payload) => En1997MutationDsl::RemovePile { index: payload.index },
    }
}

pub fn en1997_mutation_from_dsl(mutation: En1997MutationDsl) -> En1997Mutation {
    match mutation {
        En1997MutationDsl::ChangeAnnex { new_annex } => En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex }),
        En1997MutationDsl::ChangeGeotechnicalCategory { new_geotechnical_category } => {
            En1997Mutation::ChangeGeotechnicalCategory(change_geotechnical_category::ChangeGeotechnicalCategory { new_geotechnical_category })
        }
        En1997MutationDsl::ChangeDesignSituation { new_design_situation } => En1997Mutation::ChangeDesignSituation(change_design_situation::ChangeDesignSituation { new_design_situation }),
        En1997MutationDsl::ChangeDesignApproach { new_design_approach } => En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach }),
        En1997MutationDsl::ChangeGroundwaterLevel { new_groundwater_level } => En1997Mutation::ChangeGroundwaterLevel(change_groundwater_level::ChangeGroundwaterLevel { new_groundwater_level }),
        En1997MutationDsl::ChangeInvestigationDepth { new_investigation_depth } => {
            En1997Mutation::ChangeInvestigationDepth(change_investigation_depth::ChangeInvestigationDepth { new_investigation_depth })
        }
        En1997MutationDsl::ChangeFootingWidth { id, new_width } => En1997Mutation::ChangeFootingWidth(change_footing_width::ChangeFootingWidth { id, new_width }),
        En1997MutationDsl::ChangeFootingEmbedment { id, new_embedment } => En1997Mutation::ChangeFootingEmbedment(change_footing_embedment::ChangeFootingEmbedment { id, new_embedment }),
        En1997MutationDsl::ChangePileLength { id, new_length } => En1997Mutation::ChangePileLength(change_pile_length::ChangePileLength { id, new_length }),
        En1997MutationDsl::ChangePileCount { id, new_count } => En1997Mutation::ChangePileCount(change_pile_count::ChangePileCount { id, new_count }),
        En1997MutationDsl::ChangeWallBaseWidth { id, new_base_width } => En1997Mutation::ChangeWallBaseWidth(change_wall_base_width::ChangeWallBaseWidth { id, new_base_width }),
        En1997MutationDsl::ChangeSlopeAngle { id, new_angle_deg } => En1997Mutation::ChangeSlopeAngle(change_slope_angle::ChangeSlopeAngle { id, new_angle_deg }),
        En1997MutationDsl::ChangeLayerPhiPrime { id, new_phi_prime_deg } => En1997Mutation::ChangeLayerPhiPrime(change_layer_phi_prime::ChangeLayerPhiPrime { id, new_phi_prime_deg }),
        En1997MutationDsl::ChangeLayerOedometricModulus { id, new_oedometric_modulus } => {
            En1997Mutation::ChangeLayerOedometricModulus(change_layer_oedometric_modulus::ChangeLayerOedometricModulus { id, new_oedometric_modulus })
        }
        En1997MutationDsl::InsertLayer { index, layer } => En1997Mutation::InsertLayer(insert_layer::InsertLayer { index, layer }),
        En1997MutationDsl::RemoveLayer { index } => En1997Mutation::RemoveLayer(remove_layer::RemoveLayer { index }),
        En1997MutationDsl::InsertFooting { index, footing } => En1997Mutation::InsertFooting(insert_footing::InsertFooting { index, footing }),
        En1997MutationDsl::RemoveFooting { index } => En1997Mutation::RemoveFooting(remove_footing::RemoveFooting { index }),
        En1997MutationDsl::InsertPile { index, pile } => En1997Mutation::InsertPile(insert_pile::InsertPile { index, pile }),
        En1997MutationDsl::RemovePile { index } => En1997Mutation::RemovePile(remove_pile::RemovePile { index }),
    }
}

impl OpText for En1997Mutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Ok(en1997_mutation_from_dsl(<En1997MutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <En1997MutationDsl as OpText>::print_op(&en1997_mutation_to_dsl(self))
    }
}

//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod mutations_codec {
use crate::mutations::*;
use semio_s_artifact_norm_en1997_mutations::*;
use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_mut_design::change_annex;
use semio_s_artifact_norm_en1997_mut_design::change_geotechnical_category;
use semio_s_artifact_norm_en1997_mut_design::change_design_situation;
use semio_s_artifact_norm_en1997_mut_design::change_design_approach;
use semio_s_artifact_norm_en1997_mut_design::change_groundwater_level;
use semio_s_artifact_norm_en1997_mut_design::change_investigation_depth;
use semio_s_artifact_norm_en1997_mut_foundations::change_footing_width;
use semio_s_artifact_norm_en1997_mut_foundations::change_footing_embedment;
use semio_s_artifact_norm_en1997_mut_foundations::change_pile_length;
use semio_s_artifact_norm_en1997_mut_foundations::change_pile_count;
use semio_s_artifact_norm_en1997_mut_foundations::change_wall_base_width;
use semio_s_artifact_norm_en1997_mut_ground::change_slope_angle;
use semio_s_artifact_norm_en1997_mut_ground::change_layer_phi_prime;
use semio_s_artifact_norm_en1997_mut_ground::change_layer_oedometric_modulus;
use semio_s_artifact_norm_en1997_mut_ground::insert_layer;
use semio_s_artifact_norm_en1997_mut_ground::remove_layer;
use semio_s_artifact_norm_en1997_mut_foundations::insert_footing;
use semio_s_artifact_norm_en1997_mut_foundations::remove_footing;
use semio_s_artifact_norm_en1997_mut_foundations::insert_pile;
use semio_s_artifact_norm_en1997_mut_foundations::remove_pile;

/// 📥️ Decodes one committed mutation JSON document into [`En1997Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1997_mutation_json(text: &str) -> Result<En1997Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use crate::mutations::mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
