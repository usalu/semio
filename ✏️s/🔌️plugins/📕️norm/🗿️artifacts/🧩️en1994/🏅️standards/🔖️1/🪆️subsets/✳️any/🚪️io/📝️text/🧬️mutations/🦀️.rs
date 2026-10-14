//! ⚡️ En1994 mutations — OpText/OpBinary via JSON tokens (hierarchical subject).

use semio_s_artifact_norm_en1994_mutations::En1994Mutation;

#[cfg(test)] use protocol::OpText;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

impl protocol::OpText for En1994Mutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| semio_framework_diagnostic::TextError::from_value_error(e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️op-round-trip/🦀️.rs"]
mod unit_tests;

#[allow(unused_imports)]
mod mutations_codec {
use crate::text::mutations::*;
use semio_s_artifact_norm_en1994_mutations::*;
use semio_s_artifact_norm_en1994_diff::En1994Diff;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_mut_project::change_annex;
use semio_s_artifact_norm_en1994_mut_project::change_structure_kind;
use semio_s_artifact_norm_en1994_mut_project::change_steel_fy_pa;
use semio_s_artifact_norm_en1994_mut_project::change_fire_rating;
use semio_s_artifact_norm_en1994_mut_project::change_insulation_thickness_m;
use semio_s_artifact_norm_en1994_mut_project::change_fatigue_detail;
use semio_s_artifact_norm_en1994_mut_beams::insert_beam;
use semio_s_artifact_norm_en1994_mut_beams::remove_beam;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_action_q_area_pa;
use semio_s_artifact_norm_en1994_mut_studs::change_beam_stud_spacing_m;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_span_m;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_slab_thickness_m;
use semio_s_artifact_norm_en1994_mut_studs::change_beam_stud_diameter_m;
use semio_s_artifact_norm_en1994_mut_studs::change_beam_stud_count;
use semio_s_artifact_norm_en1994_mut_studs::change_beam_stud_fu_pa;
use semio_s_artifact_norm_en1994_mut_studs::change_beam_transverse_as;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_construction;
use semio_s_artifact_norm_en1994_mut_columns_slabs::insert_column;
use semio_s_artifact_norm_en1994_mut_columns_slabs::remove_column;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_column_action_force_n;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_column_kind;
use semio_s_artifact_norm_en1994_mut_columns_slabs::insert_slab;
use semio_s_artifact_norm_en1994_mut_columns_slabs::remove_slab;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_slab_action_q_area_pa;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_slab_thickness_m;
#[cfg(test)]
use semio_s_artifact_norm_en1994_mutations::demo_mutation_cases;

pub fn decode_en1994_mutation_json(text: &str) -> Result<En1994Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}

pub fn encode_en1994_mutation_json(mutation: &En1994Mutation) -> String {
    semio_framework_pack_json::to_json_string(mutation)
}
}
pub use crate::text::mutations::mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
