//! 🧬️ En1990 closed semantic mutation vocabulary for the basis-of-design subject.

use crate::diff::En1990Diff;
use crate::En1990Snapshot;

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

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
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

impl En1990Mutation {
    pub fn from_snapshot(base: &En1990Snapshot, target: &En1990Snapshot) -> Vec<En1990Mutation> {
        let mut mutations = Vec::new();
        if base.annex != target.annex {
            mutations.push(En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: target.annex }));
        }
        if base.project_id != target.project_id {
            mutations.push(En1990Mutation::ChangeProjectId(change_project_id::ChangeProjectId { new_project_id: target.project_id.clone() }));
        }
        if base.altitude_m != target.altitude_m {
            mutations.push(En1990Mutation::ChangeAltitudeM(change_altitude_m::ChangeAltitudeM { new_altitude_m: target.altitude_m }));
        }
        if base.consequence_class != target.consequence_class {
            mutations.push(En1990Mutation::ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass { new_consequence_class: target.consequence_class }));
        }
        if base.reliability_class != target.reliability_class {
            mutations.push(En1990Mutation::ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass { new_reliability_class: target.reliability_class }));
        }
        if base.design_working_life_category != target.design_working_life_category {
            mutations.push(En1990Mutation::ChangeDesignWorkingLifeCategory(change_design_working_life_category::ChangeDesignWorkingLifeCategory { new_design_working_life_category: target.design_working_life_category }));
        }
        if base.design_working_life_years != target.design_working_life_years {
            mutations.push(En1990Mutation::ChangeDesignWorkingLifeYears(change_design_working_life_years::ChangeDesignWorkingLifeYears { new_design_working_life_years: target.design_working_life_years }));
        }
        if base.reference_period_years != target.reference_period_years {
            mutations.push(En1990Mutation::ChangeReferencePeriodYears(change_reference_period_years::ChangeReferencePeriodYears { new_reference_period_years: target.reference_period_years }));
        }
        if base.supervision_level != target.supervision_level {
            mutations.push(En1990Mutation::ChangeSupervisionLevel(change_supervision_level::ChangeSupervisionLevel { new_supervision_level: target.supervision_level.clone() }));
        }
        if base.inspection_level != target.inspection_level {
            mutations.push(En1990Mutation::ChangeInspectionLevel(change_inspection_level::ChangeInspectionLevel { new_inspection_level: target.inspection_level.clone() }));
        }
        if base.beta_computed != target.beta_computed {
            mutations.push(En1990Mutation::ChangeBetaComputed(change_beta_computed::ChangeBetaComputed { new_beta_computed: target.beta_computed }));
        }
        if base.permanents != target.permanents {
            mutations.push(En1990Mutation::ChangePermanents(change_permanents::ChangePermanents { new_permanents: target.permanents.clone() }));
        }
        if base.variables != target.variables {
            mutations.push(En1990Mutation::ChangeVariables(change_variables::ChangeVariables { new_variables: target.variables.clone() }));
        }
        if base.accidentals != target.accidentals {
            mutations.push(En1990Mutation::ChangeAccidentals(change_accidentals::ChangeAccidentals { new_accidentals: target.accidentals.clone() }));
        }
        if base.seismics != target.seismics {
            mutations.push(En1990Mutation::ChangeSeismics(change_seismics::ChangeSeismics { new_seismics: target.seismics.clone() }));
        }
        if base.members != target.members {
            mutations.push(En1990Mutation::ChangeMembers(change_members::ChangeMembers { new_members: target.members.clone() }));
        }
        if base.bridge_sls != target.bridge_sls {
            mutations.push(En1990Mutation::ChangeBridgeSls(change_bridge_sls::ChangeBridgeSls { new_bridge_sls: target.bridge_sls.clone() }));
        }
        if base.effects != target.effects {
            mutations.push(En1990Mutation::ChangeEffects(change_effects::ChangeEffects { new_effects: target.effects.clone() }));
        }
        mutations
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
