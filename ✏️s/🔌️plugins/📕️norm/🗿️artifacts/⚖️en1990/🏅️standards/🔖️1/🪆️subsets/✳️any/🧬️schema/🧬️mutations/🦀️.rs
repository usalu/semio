//! 🧬️ En1990 closed semantic mutation vocabulary for the basis-of-design subject.

use crate::diff::En1990Diff;
use crate::En1990Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::EDIT_RULES;
pub use crate::standards::v1::subsets::any::io::mutation_bridge::{apply_en1990_mutation, inverse_en1990_mutation};

//#region 🔖️Leaves
use super::change_annex;
use super::change_project_id;
use super::change_altitude_m;
use super::change_consequence_class;
use super::change_reliability_class;
use super::change_design_working_life_category;
use super::change_design_working_life_years;
use super::change_reference_period_years;
use super::change_supervision_level;
use super::change_inspection_level;
use super::change_beta_computed;
use super::change_permanents;
use super::change_variables;
use super::change_accidentals;
use super::change_seismics;
use super::change_members;
use super::change_bridge_sls;
use super::change_effects;
use super::insert_permanent;
use super::insert_variable;
use super::insert_accidental;
use super::insert_seismic;
use super::insert_member;
use super::insert_effect;
use super::remove_permanent;
use super::remove_variable;
use super::remove_accidental;
use super::remove_seismic;
use super::remove_member;
use super::remove_effect;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1990Snapshot, diff = En1990Diff, schema = "s.norm.en1990")]
pub enum En1990Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeProjectId(change_project_id::ChangeProjectId),
    ChangeAltitudeM(change_altitude_m::ChangeAltitudeM),
    ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass),
    ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass),
    ChangeDesignWorkingLifeCategory(change_design_working_life_category::ChangeDesignWorkingLifeCategory),
    ChangeDesignWorkingLifeYears(change_design_working_life_years::ChangeDesignWorkingLifeYears),
    ChangeReferencePeriodYears(change_reference_period_years::ChangeReferencePeriodYears),
    ChangeSupervisionLevel(change_supervision_level::ChangeSupervisionLevel),
    ChangeInspectionLevel(change_inspection_level::ChangeInspectionLevel),
    ChangeBetaComputed(change_beta_computed::ChangeBetaComputed),
    ChangePermanents(change_permanents::ChangePermanents),
    ChangeVariables(change_variables::ChangeVariables),
    ChangeAccidentals(change_accidentals::ChangeAccidentals),
    ChangeSeismics(change_seismics::ChangeSeismics),
    ChangeMembers(change_members::ChangeMembers),
    ChangeBridgeSls(change_bridge_sls::ChangeBridgeSls),
    ChangeEffects(change_effects::ChangeEffects),
    RemoveEffect(remove_effect::RemoveEffect),
    RemoveMember(remove_member::RemoveMember),
    RemoveSeismic(remove_seismic::RemoveSeismic),
    RemoveAccidental(remove_accidental::RemoveAccidental),
    RemoveVariable(remove_variable::RemoveVariable),
    RemovePermanent(remove_permanent::RemovePermanent),
    InsertEffect(insert_effect::InsertEffect),
    InsertMember(insert_member::InsertMember),
    InsertSeismic(insert_seismic::InsertSeismic),
    InsertAccidental(insert_accidental::InsertAccidental),
    InsertVariable(insert_variable::InsertVariable),
    InsertPermanent(insert_permanent::InsertPermanent),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-project-id",
    "change-altitude-m",
    "change-consequence-class",
    "change-reliability-class",
    "change-design-working-life-category",
    "change-design-working-life-years",
    "change-reference-period-years",
    "change-supervision-level",
    "change-inspection-level",
    "change-beta-computed",
    "change-permanents",
    "change-variables",
    "change-accidentals",
    "change-seismics",
    "change-members",
    "change-bridge-sls",
    "change-effects",
    "remove-effect",
    "remove-member",
    "remove-seismic",
    "remove-accidental",
    "remove-variable",
    "remove-permanent",
    "insert-effect",
    "insert-member",
    "insert-seismic",
    "insert-accidental",
    "insert-variable",
    "insert-permanent",
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod middle_row;
