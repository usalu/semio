//! ⚡️ En1995 mutations — OpText/OpBinary via JSON tokens (hierarchical timber subject).

use semio_s_artifact_norm_en1995_mutations::En1995Mutation;

use protocol::{OpText};
#[cfg(test)] use protocol::{OpBinary};

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
use crate::text::mutations::*;
use semio_s_artifact_norm_en1995_mutations::*;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_mut_members::change_annex;
use semio_s_artifact_norm_en1995_mut_members::insert_member;
use semio_s_artifact_norm_en1995_mut_members::remove_member;
use semio_s_artifact_norm_en1995_mut_members::change_member_label_en;
use semio_s_artifact_norm_en1995_mut_members::change_member_label_de;
use semio_s_artifact_norm_en1995_mut_members::change_member_role;
use semio_s_artifact_norm_en1995_mut_members::change_member_strength_class;
use semio_s_artifact_norm_en1995_mut_members::change_member_service_class;
use semio_s_artifact_norm_en1995_mut_members::change_member_support;
use semio_s_artifact_norm_en1995_mut_geometry::change_member_b;
use semio_s_artifact_norm_en1995_mut_geometry::change_member_h;
use semio_s_artifact_norm_en1995_mut_geometry::change_member_span;
use semio_s_artifact_norm_en1995_mut_geometry::change_member_support_length;
use semio_s_artifact_norm_en1995_mut_geometry::change_member_bearing_length;
use semio_s_artifact_norm_en1995_mut_stability::change_member_buckling_length_y;
use semio_s_artifact_norm_en1995_mut_stability::change_member_buckling_length_z;
use semio_s_artifact_norm_en1995_mut_stability::change_member_restraint_spacing;
use semio_s_artifact_norm_en1995_mut_geometry::change_member_notch_depth;
use semio_s_artifact_norm_en1995_mut_geometry::change_member_notch_distance;
use semio_s_artifact_norm_en1995_mut_stability::change_member_m_crit;
use semio_s_artifact_norm_en1995_mut_footbridges::change_member_mass_kg_per_m;
use semio_s_artifact_norm_en1995_mut_footbridges::change_member_mass_kg_per_m2;
use semio_s_artifact_norm_en1995_mut_footbridges::change_member_damping_xi;
use semio_s_artifact_norm_en1995_mut_members::change_member_fire_duration;
use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_n_obs;
use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_tl_years;
use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_beta;
use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_a;
use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_b;
use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_crowd;
use semio_s_artifact_norm_en1995_mut_actions::insert_member_action;
use semio_s_artifact_norm_en1995_mut_actions::remove_member_action;
use semio_s_artifact_norm_en1995_mut_actions::change_member_action_kind;
use semio_s_artifact_norm_en1995_mut_actions::change_member_action_category;
use semio_s_artifact_norm_en1995_mut_actions::change_member_load_duration;
use semio_s_artifact_norm_en1995_mut_actions::change_member_action_q_line;
use semio_s_artifact_norm_en1995_mut_actions::change_member_action_f_point;
use semio_s_artifact_norm_en1995_mut_effects::change_member_action_mk;
use semio_s_artifact_norm_en1995_mut_effects::change_member_action_vk;
use semio_s_artifact_norm_en1995_mut_effects::change_member_action_nk;
use semio_s_artifact_norm_en1995_mut_effects::change_member_action_ntk;
use semio_s_artifact_norm_en1995_mut_effects::change_member_action_fc90_k;
use semio_s_artifact_norm_en1995_mut_connections::insert_connection;
use semio_s_artifact_norm_en1995_mut_connections::remove_connection;
use semio_s_artifact_norm_en1995_mut_connections::change_connection_label_en;
use semio_s_artifact_norm_en1995_mut_connections::change_connection_label_de;
use semio_s_artifact_norm_en1995_mut_connections::change_connection_fastener_type;
use semio_s_artifact_norm_en1995_mut_connections::change_connection_strength_class;
use semio_s_artifact_norm_en1995_mut_connections::change_connection_service_class;
use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_diameter;
use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_number;
use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_rows;
use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_spacing;
use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_edge_distance;
use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_end_distance;
use semio_s_artifact_norm_en1995_mut_plates::change_connection_t1;
use semio_s_artifact_norm_en1995_mut_plates::change_connection_t2;
use semio_s_artifact_norm_en1995_mut_plates::change_connection_steel_plate;
use semio_s_artifact_norm_en1995_mut_plates::change_connection_plate_thickness;
use semio_s_artifact_norm_en1995_mut_plates::change_connection_shear_planes;
use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_fuk;
use semio_s_artifact_norm_en1995_mut_joint_actions::insert_connection_action;
use semio_s_artifact_norm_en1995_mut_joint_actions::remove_connection_action;
use semio_s_artifact_norm_en1995_mut_joint_actions::change_connection_action_kind;
use semio_s_artifact_norm_en1995_mut_joint_actions::change_connection_load_duration;
use semio_s_artifact_norm_en1995_mut_joint_actions::change_connection_action_fk;

/// 🌉️ Decodes one mutation from the production JSON codec (the committed `🦠️mutation` vectors).
pub fn decode_en1995_mutation_json(text: &str) -> Result<En1995Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 🌉️ Encodes one mutation through the production JSON codec.
pub fn encode_en1995_mutation_json(mutation: &En1995Mutation) -> String {
    semio_framework_pack_json::to_json_string(mutation)
}
}
pub use crate::text::mutations::mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
