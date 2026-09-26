//! 🧬️ EN 1992 mutation unit tests — hierarchical RC structure vocabulary.

use crate::document::AnnexChoice;
use crate::part_1_2::FireRating;
use crate::{ExposureClass, En1992Snapshot};
use super::{
    apply_en1992_mutation, change_action_mk, change_action_n_ed, change_action_v_ed, change_anchor_a_s,
    change_anchor_h_ef, change_annex, change_bar_layer_count, change_bar_layer_diameter, change_cement_type,
    change_concrete_f_ck, change_delta_c_dev, change_design_working_life, change_member_axis_distance,
    change_member_cover, change_member_effective_depth, change_member_exposure, change_member_fire_rating,
    change_member_height, change_member_span, change_member_stirrup_spacing, change_member_width,
    change_reinforcement_f_yk, change_title, insert_anchor, insert_member, remove_anchor, remove_member,
    reorder_members, En1992Mutation,
};

fn base() -> En1992Snapshot {
    En1992Snapshot::compliant_office_frame()
}

#[test]
fn change_annex_switches_national_annex() {
    let before = base();
    let mutation = En1992Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    let (after, _) = apply_en1992_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.annex, AnnexChoice::En);
}

#[test]
fn change_title_updates_document_title() {
    let before = base();
    let mutation = En1992Mutation::ChangeTitle(change_title::ChangeTitle { new_title: "Wave C Frame".into() });
    let (after, _) = apply_en1992_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.title, "Wave C Frame");
}

#[test]
fn change_design_working_life_updates_years() {
    let before = base();
    let mutation = En1992Mutation::ChangeDesignWorkingLife(change_design_working_life::ChangeDesignWorkingLife { new_years: 100.0 });
    let (after, _) = apply_en1992_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.design_working_life_years, 100.0);
}

#[test]
fn change_delta_c_dev_updates_cover_allowance() {
    let before = base();
    let mutation = En1992Mutation::ChangeDeltaCDev(change_delta_c_dev::ChangeDeltaCDev { new_delta_c_dev: 0.015 });
    let (after, _) = apply_en1992_mutation(&before, &mutation).expect("apply");
    assert!((after.delta_c_dev - 0.015).abs() < 1e-12);
}

#[test]
fn change_cement_type_updates_binder() {
    let before = base();
    let mutation = En1992Mutation::ChangeCementType(change_cement_type::ChangeCementType { new_cement_type: "CEM III".into() });
    let (after, _) = apply_en1992_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.cement_type, "CEM III");
}

#[test]
fn change_concrete_f_ck_updates_grade() {
    let before = base();
    let grade_id = before.concrete_grades[0].id.clone();
    let mutation = En1992Mutation::ChangeConcreteFCk(change_concrete_f_ck::ChangeConcreteFCk { grade_id: grade_id.clone(), new_f_ck: 40.0e6 });
    let (after, _) = apply_en1992_mutation(&before, &mutation).expect("apply");
    assert!((after.concrete(&grade_id).unwrap().f_ck - 40.0e6).abs() < 1.0);
}

#[test]
fn change_reinforcement_f_yk_updates_grade() {
    let before = base();
    let grade_id = before.reinforcement_grades[0].id.clone();
    let mutation = En1992Mutation::ChangeReinforcementFYk(change_reinforcement_f_yk::ChangeReinforcementFYk { grade_id: grade_id.clone(), new_f_yk: 550.0e6 });
    let (after, _) = apply_en1992_mutation(&before, &mutation).expect("apply");
    assert!((after.reinforcement(&grade_id).unwrap().f_yk - 550.0e6).abs() < 1.0);
}

#[test]
fn insert_and_remove_member_round_trip() {
    let before = base();
    let mut member = before.members[0].clone();
    member.id = "beam-extra".into();
    let insert = En1992Mutation::InsertMember(insert_member::InsertMember { index: before.members.len(), member: member.clone() });
    let (mid, _) = apply_en1992_mutation(&before, &insert).expect("insert");
    assert!(mid.members.iter().any(|m| m.id == "beam-extra"));
    let remove = En1992Mutation::RemoveMember(remove_member::RemoveMember { member_id: "beam-extra".into() });
    let (after, _) = apply_en1992_mutation(&mid, &remove).expect("remove");
    assert!(!after.members.iter().any(|m| m.id == "beam-extra"));
}

#[test]
fn reorder_members_swaps_indices() {
    let before = base();
    assert!(before.members.len() >= 2);
    let first = before.members[0].id.clone();
    let mutation = En1992Mutation::ReorderMembers(reorder_members::ReorderMembers { from_index: 0, to_index: 1 });
    let (after, _) = apply_en1992_mutation(&before, &mutation).expect("reorder");
    assert_eq!(after.members[1].id, first);
}

#[test]
fn change_member_geometry_fields() {
    let before = base();
    let id = before.members[0].id.clone();
    for (mutation, check) in [
        (En1992Mutation::ChangeMemberWidth(change_member_width::ChangeMemberWidth { member_id: id.clone(), new_value: 0.4 }), 0.4),
        (En1992Mutation::ChangeMemberHeight(change_member_height::ChangeMemberHeight { member_id: id.clone(), new_value: 0.7 }), 0.7),
        (En1992Mutation::ChangeMemberEffectiveDepth(change_member_effective_depth::ChangeMemberEffectiveDepth { member_id: id.clone(), new_value: 0.62 }), 0.62),
        (En1992Mutation::ChangeMemberCover(change_member_cover::ChangeMemberCover { member_id: id.clone(), new_value: 0.04 }), 0.04),
        (En1992Mutation::ChangeMemberSpan(change_member_span::ChangeMemberSpan { member_id: id.clone(), new_value: 8.0 }), 8.0),
    ] {
        let (after, _) = apply_en1992_mutation(&before, &mutation).expect("geom");
        let m = after.members.iter().find(|m| m.id == id).unwrap();
        let value = match &mutation {
            En1992Mutation::ChangeMemberWidth(_) => m.width,
            En1992Mutation::ChangeMemberHeight(_) => m.height,
            En1992Mutation::ChangeMemberEffectiveDepth(_) => m.effective_depth,
            En1992Mutation::ChangeMemberCover(_) => m.cover,
            En1992Mutation::ChangeMemberSpan(_) => m.span,
            _ => unreachable!(),
        };
        assert!((value - check).abs() < 1e-12);
    }
}

#[test]
fn change_member_exposure_and_fire() {
    let before = base();
    let id = before.members[0].id.clone();
    let (after, _) = apply_en1992_mutation(&before, &En1992Mutation::ChangeMemberExposure(change_member_exposure::ChangeMemberExposure {
        member_id: id.clone(),
        new_exposure: ExposureClass::Xc4,
    })).expect("exposure");
    assert_eq!(after.members.iter().find(|m| m.id == id).unwrap().exposure, ExposureClass::Xc4);
    let (after, _) = apply_en1992_mutation(&before, &En1992Mutation::ChangeMemberFireRating(change_member_fire_rating::ChangeMemberFireRating {
        member_id: id.clone(),
        new_rating: FireRating::R90,
    })).expect("fire");
    assert_eq!(after.members.iter().find(|m| m.id == id).unwrap().fire.as_ref().unwrap().rating, FireRating::R90);
}

#[test]
fn change_member_stirrup_spacing_and_axis_distance() {
    let before = base();
    let id = before.members[0].id.clone();
    let (after, _) = apply_en1992_mutation(&before, &En1992Mutation::ChangeMemberStirrupSpacing(change_member_stirrup_spacing::ChangeMemberStirrupSpacing {
        member_id: id.clone(),
        new_spacing: 0.120,
    })).expect("stirrup");
    assert!((after.members.iter().find(|m| m.id == id).unwrap().stirrups.as_ref().unwrap().spacing - 0.120).abs() < 1e-12);
    let (after, _) = apply_en1992_mutation(&before, &En1992Mutation::ChangeMemberAxisDistance(change_member_axis_distance::ChangeMemberAxisDistance {
        member_id: id.clone(),
        new_axis_distance: 0.045,
    })).expect("axis");
    assert!((after.members.iter().find(|m| m.id == id).unwrap().fire.as_ref().unwrap().axis_distance - 0.045).abs() < 1e-12);
}

#[test]
fn change_bar_layer_and_actions() {
    let before = base();
    let id = before.members[0].id.clone();
    let layer_id = before.members[0].longitudinal[0].id.clone();
    let action_id = before.members[0].actions[0].id.clone();
    let (after, _) = apply_en1992_mutation(&before, &En1992Mutation::ChangeBarLayerCount(change_bar_layer_count::ChangeBarLayerCount {
        member_id: id.clone(), layer_id: layer_id.clone(), new_count: 6,
    })).expect("count");
    assert_eq!(after.members.iter().find(|m| m.id == id).unwrap().longitudinal.iter().find(|l| l.id == layer_id).unwrap().count, 6);
    let (after, _) = apply_en1992_mutation(&before, &En1992Mutation::ChangeBarLayerDiameter(change_bar_layer_diameter::ChangeBarLayerDiameter {
        member_id: id.clone(), layer_id: layer_id.clone(), new_diameter: 0.02,
    })).expect("dia");
    assert!((after.members.iter().find(|m| m.id == id).unwrap().longitudinal.iter().find(|l| l.id == layer_id).unwrap().diameter - 0.02).abs() < 1e-12);
    let (after, _) = apply_en1992_mutation(&before, &En1992Mutation::ChangeActionMk(change_action_mk::ChangeActionMk {
        member_id: id.clone(), action_id: action_id.clone(), new_value: 200.0e3,
    })).expect("m");
    assert!((after.members.iter().find(|m| m.id == id).unwrap().actions.iter().find(|a| a.id == action_id).unwrap().m_k - 200.0e3).abs() < 1.0);
    let (after, _) = apply_en1992_mutation(&before, &En1992Mutation::ChangeActionNEd(change_action_n_ed::ChangeActionNEd {
        member_id: id.clone(), action_id: action_id.clone(), new_value: -50.0e3,
    })).expect("n");
    assert!((after.members.iter().find(|m| m.id == id).unwrap().actions.iter().find(|a| a.id == action_id).unwrap().n_k + 50.0e3).abs() < 1.0);
    let (after, _) = apply_en1992_mutation(&before, &En1992Mutation::ChangeActionVEd(change_action_v_ed::ChangeActionVEd {
        member_id: id.clone(), action_id: action_id.clone(), new_value: 90.0e3,
    })).expect("v");
    assert!((after.members.iter().find(|m| m.id == id).unwrap().actions.iter().find(|a| a.id == action_id).unwrap().v_k - 90.0e3).abs() < 1.0);
}

#[test]
fn insert_remove_and_change_anchor() {
    let before = base();
    let mut anchor = before.anchors[0].clone();
    anchor.id = "anchor-extra".into();
    let (mid, _) = apply_en1992_mutation(&before, &En1992Mutation::InsertAnchor(insert_anchor::InsertAnchor {
        index: before.anchors.len(), anchor: anchor.clone(),
    })).expect("insert");
    assert!(mid.anchors.iter().any(|a| a.id == "anchor-extra"));
    let (after, _) = apply_en1992_mutation(&mid, &En1992Mutation::ChangeAnchorHEf(change_anchor_h_ef::ChangeAnchorHEf {
        anchor_id: "anchor-extra".into(), new_value: 0.12,
    })).expect("hef");
    assert!((after.anchors.iter().find(|a| a.id == "anchor-extra").unwrap().h_ef - 0.12).abs() < 1e-12);
    let (after, _) = apply_en1992_mutation(&mid, &En1992Mutation::ChangeAnchorAs(change_anchor_a_s::ChangeAnchorAs {
        anchor_id: "anchor-extra".into(), new_value: 2.0e-4,
    })).expect("as");
    assert!((after.anchors.iter().find(|a| a.id == "anchor-extra").unwrap().a_s - 2.0e-4).abs() < 1e-12);
    let (after, _) = apply_en1992_mutation(&mid, &En1992Mutation::RemoveAnchor(remove_anchor::RemoveAnchor {
        anchor_id: "anchor-extra".into(),
    })).expect("remove");
    assert!(!after.anchors.iter().any(|a| a.id == "anchor-extra"));
}
