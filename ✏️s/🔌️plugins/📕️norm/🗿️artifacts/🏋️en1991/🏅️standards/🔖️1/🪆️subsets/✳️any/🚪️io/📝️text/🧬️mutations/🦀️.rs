//! ⚡️ En1991 mutations — OpText as the aggregate's JSON wire, OpBinary as the protocol-tagged payload frame (design-load subject).

use crate::En1991Mutation;

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
    use crate::*;
    vec![
        En1991Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: semio_s_plugin_norm_compliance::AnnexChoice::De }),
        En1991Mutation::ChangeAltitude(change_altitude::ChangeAltitude { new_altitude: 1.0 }),
        En1991Mutation::ChangeAssumedBridgeTandem(change_assumed_bridge_tandem::ChangeAssumedBridgeTandem { new_assumed_bridge_tandem: 1.0 }),
        En1991Mutation::ChangeAssumedBridgeUdl(change_assumed_bridge_udl::ChangeAssumedBridgeUdl { new_assumed_bridge_udl: 1.0 }),
        En1991Mutation::ChangeAssumedCraneHorizontal(change_assumed_crane_horizontal::ChangeAssumedCraneHorizontal { new_assumed_crane_horizontal: 1.0 }),
        En1991Mutation::ChangeAssumedSiloPatch(change_assumed_silo_patch::ChangeAssumedSiloPatch { new_assumed_silo_patch: 1.0 }),
        En1991Mutation::ChangeStructureKind(change_structure_kind::ChangeStructureKind { new_structure_kind: semio_s_artifact_norm_en1991_model::StructureKind::Bridge }),
        En1991Mutation::InsertCase(semio_s_artifact_norm_en1991_mut_design_cases::insert_case::InsertCase { index: 0, case: semio_s_artifact_norm_en1991_model::En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De).cases[0].clone() }),
        En1991Mutation::RemoveCase(semio_s_artifact_norm_en1991_mut_design_cases::remove_case::RemoveCase { id: "case-1".into() }),
        En1991Mutation::ChangeCaseNumber(semio_s_artifact_norm_en1991_mut_design_cases::change_case_number::ChangeCaseNumber { case_id: "case-e11-04".into(), key: "area".into(), value: Some(60.0) }),
        En1991Mutation::ChangeCaseNumber(semio_s_artifact_norm_en1991_mut_design_cases::change_case_number::ChangeCaseNumber { case_id: "case-e11-04".into(), key: "area".into(), value: None }),
        En1991Mutation::ChangeCaseText(semio_s_artifact_norm_en1991_mut_design_cases::change_case_text::ChangeCaseText { case_id: "case-e11-04".into(), key: "category".into(), value: Some("B".into()) }),
        En1991Mutation::ChangeCaseLabelEn(semio_s_artifact_norm_en1991_mut_design_cases::change_case_label_en::ChangeCaseLabelEn { case_id: "case-e11-04".into(), new_value: "Reduction".into() }),
        En1991Mutation::ChangeCaseLabelDe(semio_s_artifact_norm_en1991_mut_design_cases::change_case_label_de::ChangeCaseLabelDe { case_id: "case-e11-04".into(), new_value: "Abminderung".into() }),
        En1991Mutation::ChangeCaseEntryId(semio_s_artifact_norm_en1991_mut_design_cases::change_case_entry_id::ChangeCaseEntryId { case_id: "case-e11-04".into(), new_value: "E11.03".into() }),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod mutations_codec {
use crate::text::*;
use crate::*;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use semio_s_artifact_norm_en1991_mut_general::change_annex;
use semio_s_artifact_norm_en1991_mut_snow::change_snow_zone;
use semio_s_artifact_norm_en1991_mut_snow::change_altitude;
use semio_s_artifact_norm_en1991_mut_snow::change_en_sk;
use semio_s_artifact_norm_en1991_mut_snow::change_north_german_lowland_snow;
use semio_s_artifact_norm_en1991_mut_wind::change_wind_zone;
use semio_s_artifact_norm_en1991_mut_wind::change_en_vb;
use semio_s_artifact_norm_en1991_mut_wind::change_terrain_category;
use semio_s_artifact_norm_en1991_mut_wind::change_mixed_terrain_upwind;
use semio_s_artifact_norm_en1991_mut_wind::change_mixed_terrain_distance;
use semio_s_artifact_norm_en1991_mut_wind::change_orography_factor;
use semio_s_artifact_norm_en1991_mut_wind::change_coast_or_island;
use semio_s_artifact_norm_en1991_mut_wind::change_air_density;
use semio_s_artifact_norm_en1991_mut_general::change_height;
use semio_s_artifact_norm_en1991_mut_general::change_width;
use semio_s_artifact_norm_en1991_mut_general::change_depth;
use semio_s_artifact_norm_en1991_mut_thermal::change_assumed_delta_t;
use semio_s_artifact_norm_en1991_mut_gravity::change_construction_activity;
use semio_s_artifact_norm_en1991_mut_gravity::change_assumed_construction_qk;
use semio_s_artifact_norm_en1991_mut_general::change_structure_kind;
use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_lane;
use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_span;
use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_tandem_x;
use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_curve_radius;
use semio_s_artifact_norm_en1991_mut_traffic::change_rail_alpha;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_rail_lm71;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_rail_lm71_udl;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_rail_sw0;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_rail_sw2;
use semio_s_artifact_norm_en1991_mut_cranes::change_crane_x;
use semio_s_artifact_norm_en1991_mut_cranes::change_crane_runway_length;
use semio_s_artifact_norm_en1991_mut_cranes::change_crane_wheel_base;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_diameter;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_eccentricity;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_hopper_angle_deg;
use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_lane_width;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_tandem;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_udl;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_lm2;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_footway;
use semio_s_artifact_norm_en1991_mut_general::change_storey_count;
use semio_s_artifact_norm_en1991_mut_thermal::change_t_max;
use semio_s_artifact_norm_en1991_mut_thermal::change_t_min;
use semio_s_artifact_norm_en1991_mut_thermal::change_initial_temperature;
use semio_s_artifact_norm_en1991_mut_thermal::change_thermal_element_type;
use semio_s_artifact_norm_en1991_mut_thermal::change_thermal_bridge_type;
use semio_s_artifact_norm_en1991_mut_thermal::change_linear_temperature_gradient;
use semio_s_artifact_norm_en1991_mut_fire::change_fire_mode;
use semio_s_artifact_norm_en1991_mut_fire::change_fire_load_basis;
use semio_s_artifact_norm_en1991_mut_fire::change_fire_curve;
use semio_s_artifact_norm_en1991_mut_fire::change_fire_duration;
use semio_s_artifact_norm_en1991_mut_fire::change_assumed_gas_temperature;
use semio_s_artifact_norm_en1991_mut_fire::change_assumed_h_net;
use semio_s_artifact_norm_en1991_mut_compartment::change_fire_compartment_area;
use semio_s_artifact_norm_en1991_mut_compartment::change_fire_compartment_height;
use semio_s_artifact_norm_en1991_mut_compartment::change_fire_opening_factor;
use semio_s_artifact_norm_en1991_mut_compartment::change_fire_thermal_inertia;
use semio_s_artifact_norm_en1991_mut_fire::change_fire_occupancy;
use semio_s_artifact_norm_en1991_mut_fire::change_fire_load_density_qf;
use semio_s_artifact_norm_en1991_mut_fire::change_assumed_qf_d;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_lm3;
use semio_s_artifact_norm_en1991_mut_traffic::change_assumed_bridge_lm4;
use semio_s_artifact_norm_en1991_mut_traffic::change_bridge_load_group;
use semio_s_artifact_norm_en1991_mut_cranes::change_crane_claimed;
use semio_s_artifact_norm_en1991_mut_cranes::change_crane_class;
use semio_s_artifact_norm_en1991_mut_cranes::change_hoist_class;
use semio_s_artifact_norm_en1991_mut_cranes::change_hoisting_speed;
use semio_s_artifact_norm_en1991_mut_cranes::change_assumed_crane_wheel;
use semio_s_artifact_norm_en1991_mut_cranes::change_assumed_crane_horizontal;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_claimed;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_kind;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_bulk_density;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_height;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_hydraulic_radius;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_mu;
use semio_s_artifact_norm_en1991_mut_silos::change_silo_k;
use semio_s_artifact_norm_en1991_mut_silos::change_assumed_silo_pressure;
use semio_s_artifact_norm_en1991_mut_silos::change_assumed_silo_patch;
use semio_s_artifact_norm_en1991_mut_silos::change_assumed_silo_wall_friction;
use semio_s_artifact_norm_en1991_mut_gravity::change_floor_assumed_qk;
use semio_s_artifact_norm_en1991_mut_gravity::change_self_weight_assumed_gk;
use semio_s_artifact_norm_en1991_mut_snow::change_roof_assumed_sk;
use semio_s_artifact_norm_en1991_mut_wind_faces::change_wind_face_assumed_wp;
use semio_s_artifact_norm_en1991_mut_general::change_accidental_assumed_force;
use semio_s_artifact_norm_en1991_mut_gravity::insert_floor;
use semio_s_artifact_norm_en1991_mut_gravity::remove_floor;
use semio_s_artifact_norm_en1991_mut_gravity::insert_self_weight_element;
use semio_s_artifact_norm_en1991_mut_gravity::remove_self_weight_element;
use semio_s_artifact_norm_en1991_mut_snow::insert_roof;
use semio_s_artifact_norm_en1991_mut_snow::remove_roof;
use semio_s_artifact_norm_en1991_mut_wind_faces::insert_wind_face;
use semio_s_artifact_norm_en1991_mut_wind_faces::remove_wind_face;
use semio_s_artifact_norm_en1991_mut_general::insert_accidental_case;
use semio_s_artifact_norm_en1991_mut_general::remove_accidental_case;
use semio_s_artifact_norm_en1991_mut_snow::change_roof_pitch_deg;
use semio_s_artifact_norm_en1991_mut_snow::change_roof_parapet_height;
use semio_s_artifact_norm_en1991_mut_snow::change_roof_drift_obstruction_height;
use semio_s_artifact_norm_en1991_mut_gravity::change_floor_area;
use semio_s_artifact_norm_en1991_mut_gravity::change_self_weight_thickness;
use semio_s_artifact_norm_en1991_mut_wind_faces::change_wind_face_z;
use semio_s_artifact_norm_en1991_mut_wind_faces::change_wind_face_loaded_area;

/// 📥️ Decodes one committed mutation JSON document into [`En1991Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1991_mutation_json(text: &str) -> Result<En1991Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use crate::text::mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
