//! 🧬️ EN 1999 mutations — hierarchical aluminium-structure vocabulary.

use crate::diff::En1999Diff;
use crate::En1999Snapshot;

use super::change_annex;
use super::change_materials;
use super::change_sections;
use super::change_members;
use super::change_connections;
use super::change_fire_scenarios;
use super::change_fatigue_details;
use super::change_cold_formed;
use super::change_shells;
use super::add_member;
use super::remove_member;
use super::change_member_n_ed;
use super::change_member_m_y_ed;
use super::change_member_buckling_length;
use super::change_material_designation;
use super::change_plate_thickness;
use super::change_weld_throat;
use super::change_bolt_count;

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
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

impl En1999Mutation {
    /// 🔀 Raise the mutation sequence that turns `base` into `target` (B2 app-surface setField/insert/remove/remedy).
    pub fn from_snapshot_replace(target: &En1999Snapshot) -> Vec<En1999Mutation> {
        Self::from_snapshot(&En1999Snapshot::empty(), target)
    }

    pub fn from_snapshot(base: &En1999Snapshot, target: &En1999Snapshot) -> Vec<En1999Mutation> {
        let mut out = Vec::new();
        if base.annex != target.annex {
            out.push(En1999Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: target.annex }));
        }
        if base.materials != target.materials {
            out.push(En1999Mutation::ChangeMaterials(change_materials::ChangeMaterials { materials: target.materials.clone() }));
        }
        if base.sections != target.sections {
            out.push(En1999Mutation::ChangeSections(change_sections::ChangeSections { sections: target.sections.clone() }));
        }
        if base.members != target.members {
            out.push(En1999Mutation::ChangeMembers(change_members::ChangeMembers { members: target.members.clone() }));
        }
        if base.connections != target.connections {
            out.push(En1999Mutation::ChangeConnections(change_connections::ChangeConnections { connections: target.connections.clone() }));
        }
        if base.fire_scenarios != target.fire_scenarios {
            out.push(En1999Mutation::ChangeFireScenarios(change_fire_scenarios::ChangeFireScenarios { fire_scenarios: target.fire_scenarios.clone() }));
        }
        if base.fatigue_details != target.fatigue_details {
            out.push(En1999Mutation::ChangeFatigueDetails(change_fatigue_details::ChangeFatigueDetails { fatigue_details: target.fatigue_details.clone() }));
        }
        out
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;

pub fn decode_en1999_mutation_json(text: &str) -> Result<En1999Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

pub fn apply_en1999_mutation(base: &En1999Snapshot, mutation: &En1999Mutation) -> Result<(En1999Snapshot, Vec<String>), String> {
    let raised = <En1999Mutation as protocol::Mutation<En1999Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <En1999Diff as protocol::MutationDiff<En1999Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

pub fn inverse_en1999_mutation(mutation: &En1999Mutation, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    <En1999Mutation as protocol::Mutation<En1999Snapshot>>::inverse(mutation, base)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
