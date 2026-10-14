//! 🧬️ EN 1999 mutations — hierarchical aluminium-structure vocabulary.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1999_diff::En1999Diff;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES};

use semio_s_artifact_norm_en1999_mut_inputs::change_annex;
use semio_s_artifact_norm_en1999_mut_inputs::change_materials;
use semio_s_artifact_norm_en1999_mut_inputs::change_sections;
use semio_s_artifact_norm_en1999_mut_inputs::change_members;
use semio_s_artifact_norm_en1999_mut_inputs::change_connections;
use semio_s_artifact_norm_en1999_mut_inputs::change_fire_scenarios;
use semio_s_artifact_norm_en1999_mut_inputs::change_fatigue_details;
use semio_s_artifact_norm_en1999_mut_inputs::change_cold_formed;
use semio_s_artifact_norm_en1999_mut_inputs::change_shells;
use semio_s_artifact_norm_en1999_mut_members::add_member;
use semio_s_artifact_norm_en1999_mut_members::remove_member;
use semio_s_artifact_norm_en1999_mut_members::change_member_n_ed;
use semio_s_artifact_norm_en1999_mut_members::change_member_m_y_ed;
use semio_s_artifact_norm_en1999_mut_members::change_member_buckling_length;
use semio_s_artifact_norm_en1999_mut_members::change_material_designation;
use semio_s_artifact_norm_en1999_mut_members::change_plate_thickness;
use semio_s_artifact_norm_en1999_mut_members::change_weld_throat;
use semio_s_artifact_norm_en1999_mut_members::change_bolt_count;

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1999Snapshot, diff = En1999Diff, schema = "norm.en1999")]
pub enum En1999Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeMaterials(change_materials::ChangeMaterials),
    ChangeSections(change_sections::ChangeSections),
    ChangeMembers(change_members::ChangeMembers),
    ChangeConnections(change_connections::ChangeConnections),
    ChangeFireScenarios(change_fire_scenarios::ChangeFireScenarios),
    ChangeFatigueDetails(change_fatigue_details::ChangeFatigueDetails),
    ChangeColdFormed(change_cold_formed::ChangeColdFormed),
    ChangeShells(change_shells::ChangeShells),
    AddMember(add_member::AddMember),
    RemoveMember(remove_member::RemoveMember),
    ChangeMemberNEd(change_member_n_ed::ChangeMemberNEd),
    ChangeMemberMYEd(change_member_m_y_ed::ChangeMemberMYEd),
    ChangeMemberBucklingLength(change_member_buckling_length::ChangeMemberBucklingLength),
    ChangeMaterialDesignation(change_material_designation::ChangeMaterialDesignation),
    ChangePlateThickness(change_plate_thickness::ChangePlateThickness),
    ChangeWeldThroat(change_weld_throat::ChangeWeldThroat),
    ChangeBoltCount(change_bolt_count::ChangeBoltCount),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-materials",
    "change-sections",
    "change-members",
    "change-connections",
    "change-fire-scenarios",
    "change-fatigue-details",
    "change-cold-formed",
    "change-shells",
    "add-member",
    "remove-member",
    "change-member-n-ed",
    "change-member-my-ed",
    "change-member-buckling-length",
    "change-material-designation",
    "change-plate-thickness",
    "change-weld-throat",
    "change-bolt-count",
];
//#endregion 🔖️Mutations

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

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::En1999Mutation;
    use semio_s_artifact_norm_en1999_model::En1999Snapshot;

    /// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
    pub fn apply_en1999_mutation(base: &En1999Snapshot, mutation: &En1999Mutation) -> Result<(En1999Snapshot, Vec<String>), String> {
        let raised = <En1999Mutation as protocol::Mutation<En1999Snapshot>>::diff(mutation, base);
        let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
        let applied = protocol::apply_diff(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
        Ok((applied, messages))
    }

    /// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
    pub fn inverse_en1999_mutation(mutation: &En1999Mutation, base: &En1999Snapshot) -> Result<Vec<En1999Mutation>, semio_framework_value::ValueError> {
        <En1999Mutation as protocol::Mutation<En1999Snapshot>>::inverse(mutation, base)
    }
}
pub use crate::mutation_bridge::*;
