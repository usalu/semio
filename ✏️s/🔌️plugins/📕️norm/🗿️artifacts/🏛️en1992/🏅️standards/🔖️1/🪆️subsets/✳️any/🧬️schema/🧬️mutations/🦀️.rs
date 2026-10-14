//! 🧬️ En1992 closed semantic mutation vocabulary — hierarchical RC structure subject.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1992_diff::En1992Diff;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES};

use semio_s_artifact_norm_en1992_mut_project::change_annex;
use semio_s_artifact_norm_en1992_mut_project::change_title;
use semio_s_artifact_norm_en1992_mut_project::change_design_working_life;
use semio_s_artifact_norm_en1992_mut_project::change_delta_c_dev;
use semio_s_artifact_norm_en1992_mut_project::change_cement_type;
use semio_s_artifact_norm_en1992_mut_project::change_concrete_f_ck;
use semio_s_artifact_norm_en1992_mut_project::change_reinforcement_f_yk;
use semio_s_artifact_norm_en1992_mut_members::insert_member;
use semio_s_artifact_norm_en1992_mut_members::remove_member;
use semio_s_artifact_norm_en1992_mut_members::reorder_members;
use semio_s_artifact_norm_en1992_mut_members::change_member_width;
use semio_s_artifact_norm_en1992_mut_members::change_member_height;
use semio_s_artifact_norm_en1992_mut_members::change_member_effective_depth;
use semio_s_artifact_norm_en1992_mut_members::change_member_cover;
use semio_s_artifact_norm_en1992_mut_members::change_member_exposure;
use semio_s_artifact_norm_en1992_mut_members::change_member_span;
use semio_s_artifact_norm_en1992_mut_resistance::change_member_stirrup_spacing;
use semio_s_artifact_norm_en1992_mut_resistance::change_member_axis_distance;
use semio_s_artifact_norm_en1992_mut_resistance::change_member_fire_rating;
use semio_s_artifact_norm_en1992_mut_resistance::change_bar_layer_count;
use semio_s_artifact_norm_en1992_mut_resistance::change_bar_layer_diameter;
use semio_s_artifact_norm_en1992_mut_resistance::change_action_mk;
use semio_s_artifact_norm_en1992_mut_resistance::change_action_nk;
use semio_s_artifact_norm_en1992_mut_resistance::change_action_vk;
use semio_s_artifact_norm_en1992_mut_anchors::insert_anchor;
use semio_s_artifact_norm_en1992_mut_anchors::remove_anchor;
use semio_s_artifact_norm_en1992_mut_anchors::change_anchor_h_ef;
use semio_s_artifact_norm_en1992_mut_anchors::change_anchor_a_s;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1992Snapshot, diff = En1992Diff, schema = "s.norm.en1992")]
pub enum En1992Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeTitle(change_title::ChangeTitle),
    ChangeDesignWorkingLife(change_design_working_life::ChangeDesignWorkingLife),
    ChangeDeltaCDev(change_delta_c_dev::ChangeDeltaCDev),
    ChangeCementType(change_cement_type::ChangeCementType),
    ChangeConcreteFCk(change_concrete_f_ck::ChangeConcreteFCk),
    ChangeReinforcementFYk(change_reinforcement_f_yk::ChangeReinforcementFYk),
    InsertMember(insert_member::InsertMember),
    RemoveMember(remove_member::RemoveMember),
    ReorderMembers(reorder_members::ReorderMembers),
    ChangeMemberWidth(change_member_width::ChangeMemberWidth),
    ChangeMemberHeight(change_member_height::ChangeMemberHeight),
    ChangeMemberEffectiveDepth(change_member_effective_depth::ChangeMemberEffectiveDepth),
    ChangeMemberCover(change_member_cover::ChangeMemberCover),
    ChangeMemberExposure(change_member_exposure::ChangeMemberExposure),
    ChangeMemberSpan(change_member_span::ChangeMemberSpan),
    ChangeMemberStirrupSpacing(change_member_stirrup_spacing::ChangeMemberStirrupSpacing),
    ChangeMemberAxisDistance(change_member_axis_distance::ChangeMemberAxisDistance),
    ChangeMemberFireRating(change_member_fire_rating::ChangeMemberFireRating),
    ChangeBarLayerCount(change_bar_layer_count::ChangeBarLayerCount),
    ChangeBarLayerDiameter(change_bar_layer_diameter::ChangeBarLayerDiameter),
    ChangeActionMk(change_action_mk::ChangeActionMk),
    ChangeActionNk(change_action_nk::ChangeActionNk),
    ChangeActionVk(change_action_vk::ChangeActionVk),
    InsertAnchor(insert_anchor::InsertAnchor),
    RemoveAnchor(remove_anchor::RemoveAnchor),
    ChangeAnchorHEf(change_anchor_h_ef::ChangeAnchorHEf),
    ChangeAnchorAs(change_anchor_a_s::ChangeAnchorAs),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-title",
    "change-design-working-life",
    "change-delta-c-dev",
    "change-cement-type",
    "change-concrete-f-ck",
    "change-reinforcement-f-yk",
    "insert-member",
    "remove-member",
    "reorder-members",
    "change-member-width",
    "change-member-height",
    "change-member-effective-depth",
    "change-member-cover",
    "change-member-exposure",
    "change-member-span",
    "change-member-stirrup-spacing",
    "change-member-axis-distance",
    "change-member-fire-rating",
    "change-bar-layer-count",
    "change-bar-layer-diameter",
    "change-action-mk",
    "change-action-nk",
    "change-action-vk",
    "insert-anchor",
    "remove-anchor",
    "change-anchor-h-ef",
    "change-anchor-as",
];

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

//#endregion 🧪️Tests

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::En1992Mutation;
    use semio_s_artifact_norm_en1992_model::En1992Snapshot;

    /// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
    pub fn apply_en1992_mutation(base: &En1992Snapshot, mutation: &En1992Mutation) -> Result<(En1992Snapshot, Vec<String>), String> {
        let raised = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(mutation, base);
        let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
        let applied = protocol::apply_diff(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
        Ok((applied, messages))
    }

    /// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
    pub fn inverse_en1992_mutation(mutation: &En1992Mutation, base: &En1992Snapshot) -> Result<Vec<En1992Mutation>, semio_framework_value::ValueError> {
        <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(mutation, base)
    }
}
pub use crate::mutation_bridge::*;
