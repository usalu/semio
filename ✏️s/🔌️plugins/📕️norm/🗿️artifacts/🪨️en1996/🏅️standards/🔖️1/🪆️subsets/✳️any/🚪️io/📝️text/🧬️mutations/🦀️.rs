//! ⚡️ En1996 mutations — OpText via JSON tokens, OpBinary via the norm-wide protocol-tagged payload frame.

use crate::artifact_schema::mutations::En1996Mutation;

use protocol::{OpBinary, OpText};

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
impl OpBinary for En1996Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        semio_s_artifact_norm_contract::payload_op_binary::encode::<crate::En1996Snapshot, _>(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        semio_s_artifact_norm_contract::payload_op_binary::decode::<crate::En1996Snapshot, _>(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), bytes)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{En1996Diff, En1996Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::change_concentrated_bearing_length;
use crate::standards::v1::subsets::any::schema::mutations::change_slab_span;
use crate::standards::v1::subsets::any::schema::mutations::change_wall_length;
use crate::standards::v1::subsets::any::schema::mutations::change_wall_height;
use crate::standards::v1::subsets::any::schema::mutations::change_wall_thickness;
use crate::standards::v1::subsets::any::schema::mutations::change_eccentricity_bottom;
use crate::standards::v1::subsets::any::schema::mutations::change_eccentricity_top;
use crate::standards::v1::subsets::any::schema::mutations::change_phi_infinity;
use crate::standards::v1::subsets::any::schema::mutations::change_qk_snow;
use crate::standards::v1::subsets::any::schema::mutations::insert_concentrated;
use crate::standards::v1::subsets::any::schema::mutations::insert_load_case;
use crate::standards::v1::subsets::any::schema::mutations::insert_opening;
use crate::standards::v1::subsets::any::schema::mutations::insert_wall;
use crate::standards::v1::subsets::any::schema::mutations::remove_concentrated;
use crate::standards::v1::subsets::any::schema::mutations::remove_load_case;
use crate::standards::v1::subsets::any::schema::mutations::remove_opening;
use crate::standards::v1::subsets::any::schema::mutations::remove_wall;
use crate::standards::v1::subsets::any::schema::mutations::change_annex;
use crate::standards::v1::subsets::any::schema::mutations::change_qp_wind;
use crate::standards::v1::subsets::any::schema::mutations::change_design_situation;
use crate::standards::v1::subsets::any::schema::mutations::change_load_case_situation;
use crate::standards::v1::subsets::any::schema::mutations::change_concentrated_force;
use crate::standards::v1::subsets::any::schema::mutations::change_gk_slab;
use crate::standards::v1::subsets::any::schema::mutations::change_qk_imposed;
use crate::standards::v1::subsets::any::schema::mutations::change_is_basement;
use crate::standards::v1::subsets::any::schema::mutations::change_storeys;
use crate::standards::v1::subsets::any::schema::mutations::change_masonry_class;
use crate::standards::v1::subsets::any::schema::mutations::change_imposed_category;
use crate::standards::v1::subsets::any::schema::mutations::change_wall_label_de;
use crate::standards::v1::subsets::any::schema::mutations::change_wall_label_en;
use crate::standards::v1::subsets::any::schema::mutations::change_exposure;
use crate::standards::v1::subsets::any::schema::mutations::change_concentrated_bearing_area;
use crate::standards::v1::subsets::any::schema::mutations::change_slab_bearing_depth;
use crate::standards::v1::subsets::any::schema::mutations::change_tributary_area;
use crate::standards::v1::subsets::any::schema::mutations::change_fire_rei;
use crate::standards::v1::subsets::any::schema::mutations::change_as_horizontal;
use crate::standards::v1::subsets::any::schema::mutations::change_as_vertical;
use crate::standards::v1::subsets::any::schema::mutations::change_f_yd;
use crate::standards::v1::subsets::any::schema::mutations::change_reinforced;
use crate::standards::v1::subsets::any::schema::mutations::change_bed_joint_thickness;
use crate::standards::v1::subsets::any::schema::mutations::change_fm;
use crate::standards::v1::subsets::any::schema::mutations::change_mortar_class;
use crate::standards::v1::subsets::any::schema::mutations::change_mortar_type;
use crate::standards::v1::subsets::any::schema::mutations::change_c_pe;
use crate::standards::v1::subsets::any::schema::mutations::change_density;
use crate::standards::v1::subsets::any::schema::mutations::change_support_sides;
use crate::standards::v1::subsets::any::schema::mutations::change_unit_fb;
use crate::standards::v1::subsets::any::schema::mutations::change_unit_group;
use crate::standards::v1::subsets::any::schema::mutations::change_unit_height;
use crate::standards::v1::subsets::any::schema::mutations::change_unit_length;
use crate::standards::v1::subsets::any::schema::mutations::change_unit_material;
use crate::standards::v1::subsets::any::schema::mutations::change_unit_width;
use crate::standards::v1::subsets::any::schema::mutations::change_wall_type;
use crate::standards::v1::subsets::any::schema::mutations::change_mu;
use crate::standards::v1::subsets::any::schema::mutations::change_opening_height;
use crate::standards::v1::subsets::any::schema::mutations::change_opening_sill;
use crate::standards::v1::subsets::any::schema::mutations::change_opening_width;
use crate::standards::v1::subsets::any::schema::mutations::change_hk_earth;

/// 📥️ Decodes one committed mutation JSON document into [`En1996Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1996_mutation_json(text: &str) -> Result<En1996Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
