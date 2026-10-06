//! ⚡️ EN 1992 mutation text wire — minimal codec for hierarchical vocabulary.

use crate::artifact_schema::mutations::En1992Mutation;
use crate::artifact_schema::mutations::*;

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

fn enc_json<T: semio_framework_value::ToValue>(v: &T) -> String {
    semio_framework_pack_json::to_json_string(v)
}
fn dec_json<T: semio_framework_value::FromValue>(s: &str) -> Result<T, String> {
    semio_framework_pack_json::from_json_str(s, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}

/// 🖨️ Print mutation as `kind key=value…` line.
pub fn print_op(op: &En1992Mutation) -> String {
    match op {
        En1992Mutation::ChangeAnnex(p) => format!("change-annex new-annex={}", enc_json(&p.new_annex)),
        En1992Mutation::ChangeTitle(p) => format!("change-title new-title={}", enc_json(&p.new_title)),
        En1992Mutation::ChangeDesignWorkingLife(p) => format!("change-design-working-life new-years={}", p.new_years),
        En1992Mutation::ChangeDeltaCDev(p) => format!("change-delta-c-dev new-delta-c-dev={}", p.new_delta_c_dev),
        En1992Mutation::ChangeCementType(p) => format!("change-cement-type new-cement-type={}", enc_json(&p.new_cement_type)),
        En1992Mutation::ChangeConcreteFCk(p) => format!("change-concrete-f-ck grade-id={} new-f-ck={}", enc_json(&p.grade_id), p.new_f_ck),
        En1992Mutation::ChangeReinforcementFYk(p) => format!("change-reinforcement-f-yk grade-id={} new-f-yk={}", enc_json(&p.grade_id), p.new_f_yk),
        En1992Mutation::InsertMember(p) => format!("insert-member index={} member={}", p.index, enc_json(&p.member)),
        En1992Mutation::RemoveMember(p) => format!("remove-member member-id={}", enc_json(&p.member_id)),
        En1992Mutation::ReorderMembers(p) => format!("reorder-members from-index={} to-index={}", p.from_index, p.to_index),
        En1992Mutation::ChangeMemberWidth(p) => format!("change-member-width member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberHeight(p) => format!("change-member-height member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberEffectiveDepth(p) => format!("change-member-effective-depth member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberCover(p) => format!("change-member-cover member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberExposure(p) => format!("change-member-exposure member-id={} new-exposure={}", enc_json(&p.member_id), enc_json(&p.new_exposure)),
        En1992Mutation::ChangeMemberSpan(p) => format!("change-member-span member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberStirrupSpacing(p) => format!("change-member-stirrup-spacing member-id={} new-spacing={}", enc_json(&p.member_id), p.new_spacing),
        En1992Mutation::ChangeMemberAxisDistance(p) => format!("change-member-axis-distance member-id={} new-axis-distance={}", enc_json(&p.member_id), p.new_axis_distance),
        En1992Mutation::ChangeMemberFireRating(p) => format!("change-member-fire-rating member-id={} new-rating={}", enc_json(&p.member_id), enc_json(&p.new_rating)),
        En1992Mutation::ChangeBarLayerCount(p) => format!("change-bar-layer-count member-id={} layer-id={} new-count={}", enc_json(&p.member_id), enc_json(&p.layer_id), p.new_count),
        En1992Mutation::ChangeBarLayerDiameter(p) => format!("change-bar-layer-diameter member-id={} layer-id={} new-diameter={}", enc_json(&p.member_id), enc_json(&p.layer_id), p.new_diameter),
        En1992Mutation::ChangeActionMk(p) => format!("change-action-mk member-id={} action-id={} new-value={}", enc_json(&p.member_id), enc_json(&p.action_id), p.new_value),
        En1992Mutation::ChangeActionNk(p) => format!("change-action-nk member-id={} action-id={} new-value={}", enc_json(&p.member_id), enc_json(&p.action_id), p.new_value),
        En1992Mutation::ChangeActionVk(p) => format!("change-action-vk member-id={} action-id={} new-value={}", enc_json(&p.member_id), enc_json(&p.action_id), p.new_value),
        En1992Mutation::InsertAnchor(p) => format!("insert-anchor index={} anchor={}", p.index, enc_json(&p.anchor)),
        En1992Mutation::RemoveAnchor(p) => format!("remove-anchor anchor-id={}", enc_json(&p.anchor_id)),
        En1992Mutation::ChangeAnchorHEf(p) => format!("change-anchor-h-ef anchor-id={} new-value={}", enc_json(&p.anchor_id), p.new_value),
        En1992Mutation::ChangeAnchorAs(p) => format!("change-anchor-as anchor-id={} new-value={}", enc_json(&p.anchor_id), p.new_value),
    }
}

/// 📖️ Parse a single op line — JSON payload after first space for complex kinds.
pub fn parse_op(line: &str) -> Result<En1992Mutation, String> {
    let line = line.trim();
    let (kind, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::HashMap<String, String> = rest
        .split_whitespace()
        .filter_map(|tok| tok.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())))
        .collect();
    let get = |k: &str| args.get(k).cloned().ok_or_else(|| format!("missing {k}"));
    match kind {
        "change-annex" => Ok(En1992Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: dec_json(&get("new-annex")?)? })),
        "change-title" => Ok(En1992Mutation::ChangeTitle(change_title::ChangeTitle { new_title: dec_json(&get("new-title")?)? })),
        "change-design-working-life" => Ok(En1992Mutation::ChangeDesignWorkingLife(change_design_working_life::ChangeDesignWorkingLife { new_years: get("new-years")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-delta-c-dev" => Ok(En1992Mutation::ChangeDeltaCDev(change_delta_c_dev::ChangeDeltaCDev { new_delta_c_dev: get("new-delta-c-dev")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-cement-type" => Ok(En1992Mutation::ChangeCementType(change_cement_type::ChangeCementType { new_cement_type: dec_json(&get("new-cement-type")?)? })),
        "change-concrete-f-ck" => Ok(En1992Mutation::ChangeConcreteFCk(change_concrete_f_ck::ChangeConcreteFCk { grade_id: dec_json(&get("grade-id")?)?, new_f_ck: get("new-f-ck")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-reinforcement-f-yk" => Ok(En1992Mutation::ChangeReinforcementFYk(change_reinforcement_f_yk::ChangeReinforcementFYk { grade_id: dec_json(&get("grade-id")?)?, new_f_yk: get("new-f-yk")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "remove-member" => Ok(En1992Mutation::RemoveMember(remove_member::RemoveMember { member_id: dec_json(&get("member-id")?)? })),
        "reorder-members" => Ok(En1992Mutation::ReorderMembers(reorder_members::ReorderMembers { from_index: get("from-index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, to_index: get("to-index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-member-width" => Ok(En1992Mutation::ChangeMemberWidth(change_member_width::ChangeMemberWidth { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-height" => Ok(En1992Mutation::ChangeMemberHeight(change_member_height::ChangeMemberHeight { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-effective-depth" => Ok(En1992Mutation::ChangeMemberEffectiveDepth(change_member_effective_depth::ChangeMemberEffectiveDepth { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-cover" => Ok(En1992Mutation::ChangeMemberCover(change_member_cover::ChangeMemberCover { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-exposure" => Ok(En1992Mutation::ChangeMemberExposure(change_member_exposure::ChangeMemberExposure { member_id: dec_json(&get("member-id")?)?, new_exposure: dec_json(&get("new-exposure")?)? })),
        "change-member-span" => Ok(En1992Mutation::ChangeMemberSpan(change_member_span::ChangeMemberSpan { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-stirrup-spacing" => Ok(En1992Mutation::ChangeMemberStirrupSpacing(change_member_stirrup_spacing::ChangeMemberStirrupSpacing { member_id: dec_json(&get("member-id")?)?, new_spacing: get("new-spacing")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-axis-distance" => Ok(En1992Mutation::ChangeMemberAxisDistance(change_member_axis_distance::ChangeMemberAxisDistance { member_id: dec_json(&get("member-id")?)?, new_axis_distance: get("new-axis-distance")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-fire-rating" => Ok(En1992Mutation::ChangeMemberFireRating(change_member_fire_rating::ChangeMemberFireRating { member_id: dec_json(&get("member-id")?)?, new_rating: dec_json(&get("new-rating")?)? })),
        "change-bar-layer-count" => Ok(En1992Mutation::ChangeBarLayerCount(change_bar_layer_count::ChangeBarLayerCount { member_id: dec_json(&get("member-id")?)?, layer_id: dec_json(&get("layer-id")?)?, new_count: get("new-count")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-bar-layer-diameter" => Ok(En1992Mutation::ChangeBarLayerDiameter(change_bar_layer_diameter::ChangeBarLayerDiameter { member_id: dec_json(&get("member-id")?)?, layer_id: dec_json(&get("layer-id")?)?, new_diameter: get("new-diameter")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-action-mk" => Ok(En1992Mutation::ChangeActionMk(change_action_mk::ChangeActionMk { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-action-nk" => Ok(En1992Mutation::ChangeActionNk(change_action_nk::ChangeActionNk { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-action-vk" => Ok(En1992Mutation::ChangeActionVk(change_action_vk::ChangeActionVk { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "remove-anchor" => Ok(En1992Mutation::RemoveAnchor(remove_anchor::RemoveAnchor { anchor_id: dec_json(&get("anchor-id")?)? })),
        "change-anchor-h-ef" => Ok(En1992Mutation::ChangeAnchorHEf(change_anchor_h_ef::ChangeAnchorHEf { anchor_id: dec_json(&get("anchor-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-as" => Ok(En1992Mutation::ChangeAnchorAs(change_anchor_a_s::ChangeAnchorAs { anchor_id: dec_json(&get("anchor-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "insert-member" => Ok(En1992Mutation::InsertMember(insert_member::InsertMember { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, member: dec_json(&get("member")?)? })),
        "insert-anchor" => Ok(En1992Mutation::InsertAnchor(insert_anchor::InsertAnchor { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, anchor: dec_json(&get("anchor")?)? })),
        other => Err(format!("unknown mutation kind {other}")),
    }
}

impl protocol::OpText for En1992Mutation {
    fn print_op(&self) -> String { print_op(self) }
    fn parse_op(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_op(text).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::En1992Diff;
use crate::En1992Snapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_annex;
use crate::standards::v1::subsets::any::schema::mutations::change_title;
use crate::standards::v1::subsets::any::schema::mutations::change_design_working_life;
use crate::standards::v1::subsets::any::schema::mutations::change_delta_c_dev;
use crate::standards::v1::subsets::any::schema::mutations::change_cement_type;
use crate::standards::v1::subsets::any::schema::mutations::change_concrete_f_ck;
use crate::standards::v1::subsets::any::schema::mutations::change_reinforcement_f_yk;
use crate::standards::v1::subsets::any::schema::mutations::insert_member;
use crate::standards::v1::subsets::any::schema::mutations::remove_member;
use crate::standards::v1::subsets::any::schema::mutations::reorder_members;
use crate::standards::v1::subsets::any::schema::mutations::change_member_width;
use crate::standards::v1::subsets::any::schema::mutations::change_member_height;
use crate::standards::v1::subsets::any::schema::mutations::change_member_effective_depth;
use crate::standards::v1::subsets::any::schema::mutations::change_member_cover;
use crate::standards::v1::subsets::any::schema::mutations::change_member_exposure;
use crate::standards::v1::subsets::any::schema::mutations::change_member_span;
use crate::standards::v1::subsets::any::schema::mutations::change_member_stirrup_spacing;
use crate::standards::v1::subsets::any::schema::mutations::change_member_axis_distance;
use crate::standards::v1::subsets::any::schema::mutations::change_member_fire_rating;
use crate::standards::v1::subsets::any::schema::mutations::change_bar_layer_count;
use crate::standards::v1::subsets::any::schema::mutations::change_bar_layer_diameter;
use crate::standards::v1::subsets::any::schema::mutations::change_action_mk;
use crate::standards::v1::subsets::any::schema::mutations::change_action_nk;
use crate::standards::v1::subsets::any::schema::mutations::change_action_vk;
use crate::standards::v1::subsets::any::schema::mutations::insert_anchor;
use crate::standards::v1::subsets::any::schema::mutations::remove_anchor;
use crate::standards::v1::subsets::any::schema::mutations::change_anchor_h_ef;
use crate::standards::v1::subsets::any::schema::mutations::change_anchor_a_s;

/// 📥️ Decodes one committed mutation JSON document into [`En1992Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1992_mutation_json(text: &str) -> Result<En1992Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
