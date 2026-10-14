//! ⚡️ En1996 mutations — OpText via JSON tokens, OpBinary via the norm-wide protocol-tagged payload frame.

use semio_s_artifact_norm_en1996_mutations::En1996Mutation;

use protocol::OpText;

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

impl OpText for En1996Mutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| semio_framework_diagnostic::TextError::from_value_error(e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

/// 💾️ The norm-wide payload op frame (`semio_s_artifact_norm_contract::payload_op_binary`), tagged by this subset's
/// `📡️.protocol.semio` records.

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod mutations_codec {
use crate::text::mutations::*;
use semio_s_artifact_norm_en1996_mutations::*;
use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_artifact_norm_en1996_mut_concentrated::change_concentrated_bearing_length;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_slab_span;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_length;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_height;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_thickness;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_eccentricity_bottom;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_eccentricity_top;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_phi_infinity;
use semio_s_artifact_norm_en1996_mut_loads::change_qk_snow;
use semio_s_artifact_norm_en1996_mut_concentrated::insert_concentrated;
use semio_s_artifact_norm_en1996_mut_loads::insert_load_case;
use semio_s_artifact_norm_en1996_mut_openings::insert_opening;
use semio_s_artifact_norm_en1996_mut_walls::insert_wall;
use semio_s_artifact_norm_en1996_mut_concentrated::remove_concentrated;
use semio_s_artifact_norm_en1996_mut_loads::remove_load_case;
use semio_s_artifact_norm_en1996_mut_openings::remove_opening;
use semio_s_artifact_norm_en1996_mut_walls::remove_wall;
use semio_s_artifact_norm_en1996_mut_project::change_annex;
use semio_s_artifact_norm_en1996_mut_loads::change_qp_wind;
use semio_s_artifact_norm_en1996_mut_loads::change_design_situation;
use semio_s_artifact_norm_en1996_mut_loads::change_load_case_situation;
use semio_s_artifact_norm_en1996_mut_concentrated::change_concentrated_force;
use semio_s_artifact_norm_en1996_mut_loads::change_gk_slab;
use semio_s_artifact_norm_en1996_mut_loads::change_qk_imposed;
use semio_s_artifact_norm_en1996_mut_project::change_is_basement;
use semio_s_artifact_norm_en1996_mut_project::change_storeys;
use semio_s_artifact_norm_en1996_mut_units::change_masonry_class;
use semio_s_artifact_norm_en1996_mut_project::change_imposed_category;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_label_de;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_label_en;
use semio_s_artifact_norm_en1996_mut_walls::change_exposure;
use semio_s_artifact_norm_en1996_mut_concentrated::change_concentrated_bearing_area;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_slab_bearing_depth;
use semio_s_artifact_norm_en1996_mut_eccentricity::change_tributary_area;
use semio_s_artifact_norm_en1996_mut_project::change_fire_rei;
use semio_s_artifact_norm_en1996_mut_reinforcement::change_as_horizontal;
use semio_s_artifact_norm_en1996_mut_reinforcement::change_as_vertical;
use semio_s_artifact_norm_en1996_mut_reinforcement::change_f_yd;
use semio_s_artifact_norm_en1996_mut_reinforcement::change_reinforced;
use semio_s_artifact_norm_en1996_mut_mortar::change_bed_joint_thickness;
use semio_s_artifact_norm_en1996_mut_mortar::change_fm;
use semio_s_artifact_norm_en1996_mut_mortar::change_mortar_class;
use semio_s_artifact_norm_en1996_mut_mortar::change_mortar_type;
use semio_s_artifact_norm_en1996_mut_loads::change_c_pe;
use semio_s_artifact_norm_en1996_mut_units::change_density;
use semio_s_artifact_norm_en1996_mut_walls::change_support_sides;
use semio_s_artifact_norm_en1996_mut_units::change_unit_fb;
use semio_s_artifact_norm_en1996_mut_units::change_unit_group;
use semio_s_artifact_norm_en1996_mut_units::change_unit_height;
use semio_s_artifact_norm_en1996_mut_units::change_unit_length;
use semio_s_artifact_norm_en1996_mut_units::change_unit_material;
use semio_s_artifact_norm_en1996_mut_units::change_unit_width;
use semio_s_artifact_norm_en1996_mut_walls::change_wall_type;
use semio_s_artifact_norm_en1996_mut_mortar::change_mu;
use semio_s_artifact_norm_en1996_mut_openings::change_opening_height;
use semio_s_artifact_norm_en1996_mut_openings::change_opening_sill;
use semio_s_artifact_norm_en1996_mut_openings::change_opening_width;
use semio_s_artifact_norm_en1996_mut_loads::change_hk_earth;

/// 📥️ Decodes one committed mutation JSON document into [`En1996Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1996_mutation_json(text: &str) -> Result<En1996Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use crate::text::mutations::mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
