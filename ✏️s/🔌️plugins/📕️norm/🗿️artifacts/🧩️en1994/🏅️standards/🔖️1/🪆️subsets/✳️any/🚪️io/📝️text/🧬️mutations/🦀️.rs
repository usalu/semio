//! ⚡️ En1994 mutations — OpText/OpBinary via JSON tokens (hierarchical subject).

use crate::artifact_schema::mutations::En1994Mutation;

use protocol::OpText;

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
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{En1994Diff, En1994Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::change_annex;
use crate::standards::v1::subsets::any::schema::mutations::change_structure_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_steel_fy_pa;
use crate::standards::v1::subsets::any::schema::mutations::change_fire_rating;
use crate::standards::v1::subsets::any::schema::mutations::change_insulation_thickness_m;
use crate::standards::v1::subsets::any::schema::mutations::change_fatigue_detail;
use crate::standards::v1::subsets::any::schema::mutations::insert_beam;
use crate::standards::v1::subsets::any::schema::mutations::remove_beam;
use crate::standards::v1::subsets::any::schema::mutations::change_beam_action_q_area_pa;
use crate::standards::v1::subsets::any::schema::mutations::change_beam_stud_spacing_m;
use crate::standards::v1::subsets::any::schema::mutations::change_beam_span_m;
use crate::standards::v1::subsets::any::schema::mutations::change_beam_slab_thickness_m;
use crate::standards::v1::subsets::any::schema::mutations::change_beam_stud_diameter_m;
use crate::standards::v1::subsets::any::schema::mutations::change_beam_stud_count;
use crate::standards::v1::subsets::any::schema::mutations::change_beam_stud_fu_pa;
use crate::standards::v1::subsets::any::schema::mutations::change_beam_transverse_as;
use crate::standards::v1::subsets::any::schema::mutations::change_beam_construction;
use crate::standards::v1::subsets::any::schema::mutations::insert_column;
use crate::standards::v1::subsets::any::schema::mutations::remove_column;
use crate::standards::v1::subsets::any::schema::mutations::change_column_action_force_n;
use crate::standards::v1::subsets::any::schema::mutations::change_column_kind;
use crate::standards::v1::subsets::any::schema::mutations::insert_slab;
use crate::standards::v1::subsets::any::schema::mutations::remove_slab;
use crate::standards::v1::subsets::any::schema::mutations::change_slab_action_q_area_pa;
use crate::standards::v1::subsets::any::schema::mutations::change_slab_thickness_m;
#[cfg(test)]
use crate::artifact_schema::mutations::demo_mutation_cases;

pub fn decode_en1994_mutation_json(text: &str) -> Result<En1994Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}

pub fn encode_en1994_mutation_json(mutation: &En1994Mutation) -> String {
    semio_framework_pack_json::to_json_string(mutation)
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
