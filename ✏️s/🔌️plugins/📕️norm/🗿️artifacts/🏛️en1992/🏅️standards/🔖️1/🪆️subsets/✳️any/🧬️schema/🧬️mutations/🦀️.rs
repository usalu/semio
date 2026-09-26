//! 🧬️ En1992 closed semantic mutation vocabulary — hierarchical RC structure subject.

use crate::diff::En1992Diff;
use crate::En1992Snapshot;

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
use super::change_action_n_ed;
use super::change_action_v_ed;
use super::insert_anchor;
use super::remove_anchor;
use super::change_anchor_h_ef;
use super::change_anchor_a_s;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
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
    ChangeActionNEd(change_action_n_ed::ChangeActionNEd),
    ChangeActionVEd(change_action_v_ed::ChangeActionVEd),
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
    "change-action-n-ed",
    "change-action-v-ed",
    "insert-anchor",
    "remove-anchor",
    "change-anchor-h-ef",
    "change-anchor-as",
];

impl En1992Mutation {
    /// 📔️ Decompose whole-document replacement into semantic mutations (best-effort full replace via set fields).
    pub fn from_snapshot(base: &En1992Snapshot, target: &En1992Snapshot) -> Vec<Self> {
        let mut out = Vec::new();
        if base.annex != target.annex {
            out.push(Self::ChangeAnnex(change_annex::ChangeAnnex { new_annex: target.annex }));
        }
        if base.title != target.title {
            out.push(Self::ChangeTitle(change_title::ChangeTitle { new_title: target.title.clone() }));
        }
        if (base.design_working_life_years - target.design_working_life_years).abs() > f64::EPSILON {
            out.push(Self::ChangeDesignWorkingLife(change_design_working_life::ChangeDesignWorkingLife { new_years: target.design_working_life_years }));
        }
        if (base.delta_c_dev - target.delta_c_dev).abs() > f64::EPSILON {
            out.push(Self::ChangeDeltaCDev(change_delta_c_dev::ChangeDeltaCDev { new_delta_c_dev: target.delta_c_dev }));
        }
        if base.cement_type != target.cement_type {
            out.push(Self::ChangeCementType(change_cement_type::ChangeCementType { new_cement_type: target.cement_type.clone() }));
        }
        // Structural list replaces handled by removing extras then inserting missing — simplified: rebuild members via remove+insert.
        for m in &base.members {
            if !target.members.iter().any(|t| t.id == m.id) {
                out.push(Self::RemoveMember(remove_member::RemoveMember { member_id: m.id.clone() }));
            }
        }
        for (i, m) in target.members.iter().enumerate() {
            if !base.members.iter().any(|b| b.id == m.id) {
                out.push(Self::InsertMember(insert_member::InsertMember { index: i, member: m.clone() }));
            }
        }
        for a in &base.anchors {
            if !target.anchors.iter().any(|t| t.id == a.id) {
                out.push(Self::RemoveAnchor(remove_anchor::RemoveAnchor { anchor_id: a.id.clone() }));
            }
        }
        for (i, a) in target.anchors.iter().enumerate() {
            if !base.anchors.iter().any(|b| b.id == a.id) {
                out.push(Self::InsertAnchor(insert_anchor::InsertAnchor { index: i, anchor: a.clone() }));
            }
        }
        out
    }
}

//#region 🌉️ExternalCodecBridge
pub fn apply_en1992_mutation(base: &En1992Snapshot, mutation: &En1992Mutation) -> Result<(En1992Snapshot, Vec<String>), String> {
    let raised = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}
pub fn inverse_en1992_mutation(mutation: &En1992Mutation, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    <En1992Mutation as protocol::Mutation<En1992Snapshot>>::inverse(mutation, base)
}
/// 📥️ Decodes one committed mutation JSON document into [`En1992Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1992_mutation_json(text: &str) -> Result<En1992Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

//#endregion 🧪️Tests
