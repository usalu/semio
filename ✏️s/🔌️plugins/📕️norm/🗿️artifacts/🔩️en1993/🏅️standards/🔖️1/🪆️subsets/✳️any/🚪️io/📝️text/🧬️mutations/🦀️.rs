//! ⚡️ En1993 mutations — OpText via JSON tokens (hierarchical steel subject).

use semio_s_artifact_norm_en1993_mutations::En1993Mutation;

use protocol::OpText;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

impl OpText for En1993Mutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| semio_framework_diagnostic::TextError::from_value_error(e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

#[allow(unused_imports)]
mod mutations_codec {
use crate::text::mutations::*;
use semio_s_artifact_norm_en1993_mutations::*;
use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
use semio_s_artifact_norm_en1993_mut_general::change_annex;
use semio_s_artifact_norm_en1993_mut_connections::update_bolt_inputs;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::update_bridge_inputs;
use semio_s_artifact_norm_en1993_mut_thin_walled::update_cold_formed_inputs;
use semio_s_artifact_norm_en1993_mut_cranes::update_crane_inputs;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::update_fatigue_inputs;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::update_fire_inputs;
use semio_s_artifact_norm_en1993_mut_connections::update_hss_inputs;
use semio_s_artifact_norm_en1993_mut_general::update_member_properties;
use semio_s_artifact_norm_en1993_mut_towers::update_pile_inputs;
use semio_s_artifact_norm_en1993_mut_thin_walled::update_plated_inputs;
use semio_s_artifact_norm_en1993_mut_thin_walled::update_silo_shell_inputs;
use semio_s_artifact_norm_en1993_mut_general::update_stainless_inputs;
use semio_s_artifact_norm_en1993_mut_cranes::update_tension_component_inputs;
use semio_s_artifact_norm_en1993_mut_general::update_through_thickness_inputs;
use semio_s_artifact_norm_en1993_mut_towers::update_tower_inputs;
use semio_s_artifact_norm_en1993_mut_connections::update_weld_inputs;
use semio_s_artifact_norm_en1993_mut_general::insert_material;
use semio_s_artifact_norm_en1993_mut_general::remove_material;
use semio_s_artifact_norm_en1993_mut_general::insert_section;
use semio_s_artifact_norm_en1993_mut_general::remove_section;
use semio_s_artifact_norm_en1993_mut_general::insert_member;
use semio_s_artifact_norm_en1993_mut_general::remove_member;
use semio_s_artifact_norm_en1993_mut_connections::insert_load_case;
use semio_s_artifact_norm_en1993_mut_connections::remove_load_case;
use semio_s_artifact_norm_en1993_mut_connections::insert_member_action;
use semio_s_artifact_norm_en1993_mut_connections::remove_member_action;
use semio_s_artifact_norm_en1993_mut_connections::insert_joint;
use semio_s_artifact_norm_en1993_mut_connections::remove_joint;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::insert_fatigue_detail;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::remove_fatigue_detail;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::insert_fire_exposure;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::remove_fire_exposure;
use semio_s_artifact_norm_en1993_mut_thin_walled::insert_cold_formed_member;
use semio_s_artifact_norm_en1993_mut_thin_walled::remove_cold_formed_member;
use semio_s_artifact_norm_en1993_mut_thin_walled::insert_plated_panel;
use semio_s_artifact_norm_en1993_mut_thin_walled::remove_plated_panel;
use semio_s_artifact_norm_en1993_mut_thin_walled::insert_silo_shell;
use semio_s_artifact_norm_en1993_mut_thin_walled::remove_silo_shell;
use semio_s_artifact_norm_en1993_mut_cranes::insert_tension_component;
use semio_s_artifact_norm_en1993_mut_cranes::remove_tension_component;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::insert_bridge_fatigue;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::remove_bridge_fatigue;
use semio_s_artifact_norm_en1993_mut_towers::insert_tower_leg;
use semio_s_artifact_norm_en1993_mut_towers::remove_tower_leg;
use semio_s_artifact_norm_en1993_mut_towers::insert_pile;
use semio_s_artifact_norm_en1993_mut_towers::remove_pile;
use semio_s_artifact_norm_en1993_mut_cranes::insert_crane_runway;
use semio_s_artifact_norm_en1993_mut_cranes::remove_crane_runway;

pub fn decode_en1993_mutation_json(text: &str) -> Result<En1993Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use crate::text::mutations::mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
