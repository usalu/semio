//! 🧬️ En1990 closed semantic mutation vocabulary for the basis-of-design subject.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
extern crate semio_framework_os_kernel as vcs;

use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::EDIT_RULES;

//#region 🔖️Leaves
use semio_s_artifact_norm_en1990_mut_design_basis::change_annex;
use semio_s_artifact_norm_en1990_mut_design_basis::change_project_id;
use semio_s_artifact_norm_en1990_mut_design_basis::change_altitude_m;
use semio_s_artifact_norm_en1990_mut_reliability::change_consequence_class;
use semio_s_artifact_norm_en1990_mut_reliability::change_reliability_class;
use semio_s_artifact_norm_en1990_mut_design_basis::change_design_working_life_category;
use semio_s_artifact_norm_en1990_mut_design_basis::change_design_working_life_years;
use semio_s_artifact_norm_en1990_mut_design_basis::change_reference_period_years;
use semio_s_artifact_norm_en1990_mut_reliability::change_supervision_level;
use semio_s_artifact_norm_en1990_mut_reliability::change_inspection_level;
use semio_s_artifact_norm_en1990_mut_reliability::change_beta_computed;
use semio_s_artifact_norm_en1990_mut_actions::change_permanents;
use semio_s_artifact_norm_en1990_mut_actions::change_variables;
use semio_s_artifact_norm_en1990_mut_accidental::change_accidentals;
use semio_s_artifact_norm_en1990_mut_accidental::change_seismics;
use semio_s_artifact_norm_en1990_mut_members::change_members;
use semio_s_artifact_norm_en1990_mut_members::change_bridge_sls;
use semio_s_artifact_norm_en1990_mut_members::change_effects;
use semio_s_artifact_norm_en1990_mut_actions::insert_permanent;
use semio_s_artifact_norm_en1990_mut_actions::insert_variable;
use semio_s_artifact_norm_en1990_mut_accidental::insert_accidental;
use semio_s_artifact_norm_en1990_mut_accidental::insert_seismic;
use semio_s_artifact_norm_en1990_mut_members::insert_member;
use semio_s_artifact_norm_en1990_mut_members::insert_effect;
use semio_s_artifact_norm_en1990_mut_actions::remove_permanent;
use semio_s_artifact_norm_en1990_mut_actions::remove_variable;
use semio_s_artifact_norm_en1990_mut_accidental::remove_accidental;
use semio_s_artifact_norm_en1990_mut_accidental::remove_seismic;
use semio_s_artifact_norm_en1990_mut_members::remove_member;
use semio_s_artifact_norm_en1990_mut_members::remove_effect;
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
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff and inverse through the central applier.
pub mod mutation_bridge {
/// 🌉️ Applies one mutation to `base` through the central applier, returning the next snapshot and the raised `<Level>:<code>` messages.
pub fn apply_en1990_mutation(base: &semio_s_artifact_norm_en1990_model::En1990Snapshot, mutation: &crate::En1990Mutation) -> Result<(semio_s_artifact_norm_en1990_model::En1990Snapshot, Vec<String>), String> {
    let raised = <crate::En1990Mutation as protocol::Mutation<semio_s_artifact_norm_en1990_model::En1990Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = protocol::apply_diff(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ The mutation rows that undo `mutation` applied to `base`, in store order.
pub fn inverse_en1990_mutation(mutation: &crate::En1990Mutation, base: &semio_s_artifact_norm_en1990_model::En1990Snapshot) -> Result<Vec<crate::En1990Mutation>, semio_framework_value::ValueError> {
    <crate::En1990Mutation as protocol::Mutation<semio_s_artifact_norm_en1990_model::En1990Snapshot>>::inverse(mutation, base)
}
}
pub use crate::mutation_bridge::*;
