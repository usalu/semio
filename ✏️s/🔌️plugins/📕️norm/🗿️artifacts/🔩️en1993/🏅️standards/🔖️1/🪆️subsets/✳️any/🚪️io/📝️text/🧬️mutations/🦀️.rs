//! ⚡️ En1993 mutations — OpText via JSON tokens (hierarchical steel subject).

use crate::artifact_schema::mutations::En1993Mutation;

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
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{En1993Diff, En1993Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::change_annex;
use crate::standards::v1::subsets::any::schema::mutations::update_bolt_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_bridge_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_cold_formed_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_crane_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_fatigue_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_fire_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_hss_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_member_properties;
use crate::standards::v1::subsets::any::schema::mutations::update_pile_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_plated_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_silo_shell_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_stainless_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_tension_component_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_through_thickness_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_tower_inputs;
use crate::standards::v1::subsets::any::schema::mutations::update_weld_inputs;
use crate::standards::v1::subsets::any::schema::mutations::insert_material;
use crate::standards::v1::subsets::any::schema::mutations::remove_material;
use crate::standards::v1::subsets::any::schema::mutations::insert_section;
use crate::standards::v1::subsets::any::schema::mutations::remove_section;
use crate::standards::v1::subsets::any::schema::mutations::insert_member;
use crate::standards::v1::subsets::any::schema::mutations::remove_member;
use crate::standards::v1::subsets::any::schema::mutations::insert_load_case;
use crate::standards::v1::subsets::any::schema::mutations::remove_load_case;
use crate::standards::v1::subsets::any::schema::mutations::insert_member_action;
use crate::standards::v1::subsets::any::schema::mutations::remove_member_action;
use crate::standards::v1::subsets::any::schema::mutations::insert_joint;
use crate::standards::v1::subsets::any::schema::mutations::remove_joint;
use crate::standards::v1::subsets::any::schema::mutations::insert_fatigue_detail;
use crate::standards::v1::subsets::any::schema::mutations::remove_fatigue_detail;
use crate::standards::v1::subsets::any::schema::mutations::insert_fire_exposure;
use crate::standards::v1::subsets::any::schema::mutations::remove_fire_exposure;
use crate::standards::v1::subsets::any::schema::mutations::insert_cold_formed_member;
use crate::standards::v1::subsets::any::schema::mutations::remove_cold_formed_member;
use crate::standards::v1::subsets::any::schema::mutations::insert_plated_panel;
use crate::standards::v1::subsets::any::schema::mutations::remove_plated_panel;
use crate::standards::v1::subsets::any::schema::mutations::insert_silo_shell;
use crate::standards::v1::subsets::any::schema::mutations::remove_silo_shell;
use crate::standards::v1::subsets::any::schema::mutations::insert_tension_component;
use crate::standards::v1::subsets::any::schema::mutations::remove_tension_component;
use crate::standards::v1::subsets::any::schema::mutations::insert_bridge_fatigue;
use crate::standards::v1::subsets::any::schema::mutations::remove_bridge_fatigue;
use crate::standards::v1::subsets::any::schema::mutations::insert_tower_leg;
use crate::standards::v1::subsets::any::schema::mutations::remove_tower_leg;
use crate::standards::v1::subsets::any::schema::mutations::insert_pile;
use crate::standards::v1::subsets::any::schema::mutations::remove_pile;
use crate::standards::v1::subsets::any::schema::mutations::insert_crane_runway;
use crate::standards::v1::subsets::any::schema::mutations::remove_crane_runway;

pub fn decode_en1993_mutation_json(text: &str) -> Result<En1993Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
