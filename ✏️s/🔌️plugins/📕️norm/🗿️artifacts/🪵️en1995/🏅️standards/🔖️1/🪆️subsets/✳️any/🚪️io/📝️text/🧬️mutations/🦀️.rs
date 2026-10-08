//! ⚡️ En1995 mutations — OpText/OpBinary via JSON tokens (hierarchical timber subject).

use crate::artifact_schema::mutations::En1995Mutation;

use protocol::{OpBinary, OpText};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

impl OpText for En1995Mutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| semio_framework_diagnostic::TextError::from_value_error(e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}



#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️op-round-trip/🦀️.rs"]
mod unit_tests;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{En1995Diff, En1995Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::change_annex;
use crate::standards::v1::subsets::any::schema::mutations::insert_member;
use crate::standards::v1::subsets::any::schema::mutations::remove_member;
use crate::standards::v1::subsets::any::schema::mutations::change_member_label_en;
use crate::standards::v1::subsets::any::schema::mutations::change_member_label_de;
use crate::standards::v1::subsets::any::schema::mutations::change_member_role;
use crate::standards::v1::subsets::any::schema::mutations::change_member_strength_class;
use crate::standards::v1::subsets::any::schema::mutations::change_member_service_class;
use crate::standards::v1::subsets::any::schema::mutations::change_member_support;
use crate::standards::v1::subsets::any::schema::mutations::change_member_b;
use crate::standards::v1::subsets::any::schema::mutations::change_member_h;
use crate::standards::v1::subsets::any::schema::mutations::change_member_span;
use crate::standards::v1::subsets::any::schema::mutations::change_member_support_length;
use crate::standards::v1::subsets::any::schema::mutations::change_member_bearing_length;
use crate::standards::v1::subsets::any::schema::mutations::change_member_buckling_length_y;
use crate::standards::v1::subsets::any::schema::mutations::change_member_buckling_length_z;
use crate::standards::v1::subsets::any::schema::mutations::change_member_restraint_spacing;
use crate::standards::v1::subsets::any::schema::mutations::change_member_notch_depth;
use crate::standards::v1::subsets::any::schema::mutations::change_member_notch_distance;
use crate::standards::v1::subsets::any::schema::mutations::change_member_m_crit;
use crate::standards::v1::subsets::any::schema::mutations::change_member_mass_kg_per_m;
use crate::standards::v1::subsets::any::schema::mutations::change_member_mass_kg_per_m2;
use crate::standards::v1::subsets::any::schema::mutations::change_member_damping_xi;
use crate::standards::v1::subsets::any::schema::mutations::change_member_fire_duration;
use crate::standards::v1::subsets::any::schema::mutations::change_member_bridge_n_obs;
use crate::standards::v1::subsets::any::schema::mutations::change_member_bridge_tl_years;
use crate::standards::v1::subsets::any::schema::mutations::change_member_bridge_beta;
use crate::standards::v1::subsets::any::schema::mutations::change_member_bridge_a;
use crate::standards::v1::subsets::any::schema::mutations::change_member_bridge_b;
use crate::standards::v1::subsets::any::schema::mutations::change_member_bridge_crowd;
use crate::standards::v1::subsets::any::schema::mutations::insert_member_action;
use crate::standards::v1::subsets::any::schema::mutations::remove_member_action;
use crate::standards::v1::subsets::any::schema::mutations::change_member_action_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_member_action_category;
use crate::standards::v1::subsets::any::schema::mutations::change_member_load_duration;
use crate::standards::v1::subsets::any::schema::mutations::change_member_action_q_line;
use crate::standards::v1::subsets::any::schema::mutations::change_member_action_f_point;
use crate::standards::v1::subsets::any::schema::mutations::change_member_action_mk;
use crate::standards::v1::subsets::any::schema::mutations::change_member_action_vk;
use crate::standards::v1::subsets::any::schema::mutations::change_member_action_nk;
use crate::standards::v1::subsets::any::schema::mutations::change_member_action_ntk;
use crate::standards::v1::subsets::any::schema::mutations::change_member_action_fc90_k;
use crate::standards::v1::subsets::any::schema::mutations::insert_connection;
use crate::standards::v1::subsets::any::schema::mutations::remove_connection;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_label_en;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_label_de;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_fastener_type;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_strength_class;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_service_class;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_diameter;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_number;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_rows;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_spacing;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_edge_distance;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_end_distance;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_t1;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_t2;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_steel_plate;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_plate_thickness;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_shear_planes;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_fuk;
use crate::standards::v1::subsets::any::schema::mutations::insert_connection_action;
use crate::standards::v1::subsets::any::schema::mutations::remove_connection_action;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_action_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_load_duration;
use crate::standards::v1::subsets::any::schema::mutations::change_connection_action_fk;

/// 🌉️ Decodes one mutation from the production JSON codec (the committed `🦠️mutation` vectors).
pub fn decode_en1995_mutation_json(text: &str) -> Result<En1995Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 🌉️ Encodes one mutation through the production JSON codec.
pub fn encode_en1995_mutation_json(mutation: &En1995Mutation) -> String {
    semio_framework_pack_json::to_json_string(mutation)
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
