//! 🧬️ En1992 closed semantic mutation vocabulary — hierarchical RC structure subject.

use crate::diff::En1992Diff;
use crate::En1992Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::{resolve_edit, EDIT_RULES};

use super::change_annex;
use super::change_title;
use super::change_design_working_life;
use super::change_delta_c_dev;
use super::change_cement_type;
use super::change_concrete_f_ck;
use super::change_reinforcement_f_yk;
use super::insert_member;
use super::remove_member;
use super::reorder_members;
use super::change_member_width;
use super::change_member_height;
use super::change_member_effective_depth;
use super::change_member_cover;
use super::change_member_exposure;
use super::change_member_span;
use super::change_member_stirrup_spacing;
use super::change_member_axis_distance;
use super::change_member_fire_rating;
use super::change_bar_layer_count;
use super::change_bar_layer_diameter;
use super::change_action_mk;
use super::change_action_nk;
use super::change_action_vk;
use super::insert_anchor;
use super::remove_anchor;
use super::change_anchor_h_ef;
use super::change_anchor_a_s;

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


//#region 🧫️Vectors
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧫️Vectors

#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod middle_row;
