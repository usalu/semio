//! ⚡️ En1991 mutations — OpText as the aggregate's JSON wire, OpBinary as the protocol-tagged payload frame (design-load subject).

use crate::artifact_schema::mutations::En1991Mutation;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

impl protocol::OpText for En1991Mutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| semio_framework_diagnostic::TextError::from_value_error(e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}




#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1991Mutation> {
    use crate::artifact_schema::mutations::*;
    vec![
        En1991Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::De }),
        En1991Mutation::ChangeAltitude(change_altitude::ChangeAltitude { new_altitude: 1.0 }),
        En1991Mutation::ChangeAssumedBridgeTandem(change_assumed_bridge_tandem::ChangeAssumedBridgeTandem { new_assumed_bridge_tandem: 1.0 }),
        En1991Mutation::ChangeAssumedBridgeUdl(change_assumed_bridge_udl::ChangeAssumedBridgeUdl { new_assumed_bridge_udl: 1.0 }),
        En1991Mutation::ChangeAssumedCraneHorizontal(change_assumed_crane_horizontal::ChangeAssumedCraneHorizontal { new_assumed_crane_horizontal: 1.0 }),
        En1991Mutation::ChangeAssumedSiloPatch(change_assumed_silo_patch::ChangeAssumedSiloPatch { new_assumed_silo_patch: 1.0 }),
        En1991Mutation::ChangeStructureKind(change_structure_kind::ChangeStructureKind { new_structure_kind: crate::StructureKind::Bridge }),
    ]
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{En1991Diff, En1991Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::change_annex;
use crate::standards::v1::subsets::any::schema::mutations::change_snow_zone;
use crate::standards::v1::subsets::any::schema::mutations::change_altitude;
use crate::standards::v1::subsets::any::schema::mutations::change_en_sk;
use crate::standards::v1::subsets::any::schema::mutations::change_north_german_lowland_snow;
use crate::standards::v1::subsets::any::schema::mutations::change_wind_zone;
use crate::standards::v1::subsets::any::schema::mutations::change_en_vb;
use crate::standards::v1::subsets::any::schema::mutations::change_terrain_category;
use crate::standards::v1::subsets::any::schema::mutations::change_mixed_terrain_upwind;
use crate::standards::v1::subsets::any::schema::mutations::change_mixed_terrain_distance;
use crate::standards::v1::subsets::any::schema::mutations::change_orography_factor;
use crate::standards::v1::subsets::any::schema::mutations::change_coast_or_island;
use crate::standards::v1::subsets::any::schema::mutations::change_air_density;
use crate::standards::v1::subsets::any::schema::mutations::change_height;
use crate::standards::v1::subsets::any::schema::mutations::change_width;
use crate::standards::v1::subsets::any::schema::mutations::change_depth;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_delta_t;
use crate::standards::v1::subsets::any::schema::mutations::change_construction_activity;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_construction_qk;
use crate::standards::v1::subsets::any::schema::mutations::change_structure_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_bridge_lane;
use crate::standards::v1::subsets::any::schema::mutations::change_bridge_span;
use crate::standards::v1::subsets::any::schema::mutations::change_bridge_lane_width;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_bridge_tandem;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_bridge_udl;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_bridge_lm2;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_bridge_footway;
use crate::standards::v1::subsets::any::schema::mutations::change_storey_count;
use crate::standards::v1::subsets::any::schema::mutations::change_t_max;
use crate::standards::v1::subsets::any::schema::mutations::change_t_min;
use crate::standards::v1::subsets::any::schema::mutations::change_initial_temperature;
use crate::standards::v1::subsets::any::schema::mutations::change_thermal_element_type;
use crate::standards::v1::subsets::any::schema::mutations::change_thermal_bridge_type;
use crate::standards::v1::subsets::any::schema::mutations::change_linear_temperature_gradient;
use crate::standards::v1::subsets::any::schema::mutations::change_fire_mode;
use crate::standards::v1::subsets::any::schema::mutations::change_fire_curve;
use crate::standards::v1::subsets::any::schema::mutations::change_fire_duration;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_gas_temperature;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_h_net;
use crate::standards::v1::subsets::any::schema::mutations::change_fire_compartment_area;
use crate::standards::v1::subsets::any::schema::mutations::change_fire_compartment_height;
use crate::standards::v1::subsets::any::schema::mutations::change_fire_opening_factor;
use crate::standards::v1::subsets::any::schema::mutations::change_fire_thermal_inertia;
use crate::standards::v1::subsets::any::schema::mutations::change_fire_occupancy;
use crate::standards::v1::subsets::any::schema::mutations::change_fire_load_density_qf;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_qf_d;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_bridge_lm3;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_bridge_lm4;
use crate::standards::v1::subsets::any::schema::mutations::change_bridge_load_group;
use crate::standards::v1::subsets::any::schema::mutations::change_crane_claimed;
use crate::standards::v1::subsets::any::schema::mutations::change_crane_class;
use crate::standards::v1::subsets::any::schema::mutations::change_hoist_class;
use crate::standards::v1::subsets::any::schema::mutations::change_hoisting_speed;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_crane_wheel;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_crane_horizontal;
use crate::standards::v1::subsets::any::schema::mutations::change_silo_claimed;
use crate::standards::v1::subsets::any::schema::mutations::change_silo_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_silo_bulk_density;
use crate::standards::v1::subsets::any::schema::mutations::change_silo_height;
use crate::standards::v1::subsets::any::schema::mutations::change_silo_hydraulic_radius;
use crate::standards::v1::subsets::any::schema::mutations::change_silo_mu;
use crate::standards::v1::subsets::any::schema::mutations::change_silo_k;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_silo_pressure;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_silo_patch;
use crate::standards::v1::subsets::any::schema::mutations::change_assumed_silo_wall_friction;
use crate::standards::v1::subsets::any::schema::mutations::change_floor_assumed_qk;
use crate::standards::v1::subsets::any::schema::mutations::change_self_weight_assumed_gk;
use crate::standards::v1::subsets::any::schema::mutations::change_roof_assumed_sk;
use crate::standards::v1::subsets::any::schema::mutations::change_wind_face_assumed_wp;
use crate::standards::v1::subsets::any::schema::mutations::change_accidental_assumed_force;
use crate::standards::v1::subsets::any::schema::mutations::insert_floors;
use crate::standards::v1::subsets::any::schema::mutations::remove_floors;
use crate::standards::v1::subsets::any::schema::mutations::insert_self_weight_elements;
use crate::standards::v1::subsets::any::schema::mutations::remove_self_weight_elements;
use crate::standards::v1::subsets::any::schema::mutations::insert_roofs;
use crate::standards::v1::subsets::any::schema::mutations::remove_roofs;
use crate::standards::v1::subsets::any::schema::mutations::insert_wind_faces;
use crate::standards::v1::subsets::any::schema::mutations::remove_wind_faces;
use crate::standards::v1::subsets::any::schema::mutations::insert_accidental_cases;
use crate::standards::v1::subsets::any::schema::mutations::remove_accidental_cases;

/// 📥️ Decodes one committed mutation JSON document into [`En1991Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1991_mutation_json(text: &str) -> Result<En1991Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
