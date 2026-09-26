//! 🧬️ EN 1993 mutation unit tests — hierarchical entity upserts.

use crate::document::AnnexChoice;
use super::{
    apply_en1993_mutation, change_annex, update_bolt_inputs, update_bridge_inputs, update_cold_formed_inputs, update_crane_inputs, update_fatigue_inputs,
    update_fire_inputs, update_hss_inputs, update_member_properties, update_pile_inputs, update_plated_inputs, update_silo_shell_inputs, update_stainless_inputs,
    update_tension_component_inputs, update_through_thickness_inputs, update_tower_inputs, update_weld_inputs, En1993Mutation,
};
use crate::{
    BridgeFatigue, SteelMember, ColdFormedMember, CraneRunway, En1993Snapshot, FatigueDetail, FireExposure, LoadCase, PlatedPanel, SiloShell, SteelJoint, SteelMaterial, SteelPile,
    TensionComponent, TowerLeg,
};

fn base() -> En1993Snapshot {
    En1993Snapshot::compliant_heb240_frame()
}

#[test]
fn change_annex_switches_national_annex() {
    let before = base();
    let mutation = En1993Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.annex, AnnexChoice::En);
}

#[test]
fn update_member_properties_upserts_member() {
    let before = base();
    let mut member = before.members[0].clone();
    member.length = 9.0;
    let mutation = En1993Mutation::UpdateMemberProperties(update_member_properties::UpdateMemberProperties { member: member.clone() });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.members.iter().find(|m| m.id == member.id).unwrap().length, 9.0);
}

#[test]
fn update_stainless_inputs_upserts_material() {
    let before = base();
    let material = SteelMaterial {
        id: "mat-s460".into(),
        grade: "S460".into(),
        fy: 460.0e6,
        fu: 540.0e6,
        e_modulus: 210.0e9,
        g_modulus: 81.0e9,
        subgrade: "J2".into(),
        kind: "carbon".into(),
    };
    let mutation = En1993Mutation::UpdateStainlessInputs(update_stainless_inputs::UpdateStainlessInputs { material });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert!(after.materials.iter().any(|m| m.id == "mat-s460" && m.fy == 460.0e6));
}

#[test]
fn update_through_thickness_inputs_upserts_section() {
    let before = base();
    let mut section = before.sections[0].clone();
    section.designation = "HEB 300".into();
    let mutation = En1993Mutation::UpdateThroughThicknessInputs(update_through_thickness_inputs::UpdateThroughThicknessInputs { section: section.clone() });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.sections.iter().find(|s| s.id == section.id).unwrap().designation, "HEB 300");
}

#[test]
fn update_hss_inputs_upserts_load_case() {
    let before = base();
    let load_case = LoadCase { id: "lc-acc".into(), name: "Accidental".into(), kind: "accidental".into(), category: "office".into() };
    let mutation = En1993Mutation::UpdateHssInputs(update_hss_inputs::UpdateHssInputs { load_case  });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert!(after.load_cases.iter().any(|l| l.id == "lc-acc"));
}

#[test]
fn update_weld_inputs_upserts_member_action() {
    let before = base();
    let mut action = before.member_actions[0].clone();
    action.action.n = 1.0e6;
    let mutation = En1993Mutation::UpdateWeldInputs(update_weld_inputs::UpdateWeldInputs { member_action: action.clone() });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.member_actions.iter().find(|a| a.id == action.id).unwrap().action.n, 1.0e6);
}

#[test]
fn update_bolt_inputs_upserts_joint() {
    let before = base();
    let mut joint = before.joints[0].clone();
    joint.bolt_diameter = 0.024;
    joint.bolt_class = "10.9".into();
    joint.bolt_rows = 2;
    joint.bolts_per_row = 2;
    let mutation = En1993Mutation::UpdateBoltInputs(update_bolt_inputs::UpdateBoltInputs { joint: joint.clone() });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.joints.iter().find(|j| j.id == joint.id).unwrap().bolt_diameter, 0.024);
}

#[test]
fn update_fatigue_inputs_upserts_detail() {
    let before = base();
    let detail = FatigueDetail {
        id: "fat-new".into(),
        member_id: before.members[0].id.clone(),
        category: 56,
        method: "safe_life".into(), spectrum: vec![crate::FatigueBand { id: "fb".into(), delta_sigma: 80.0e6, cycles: 2.0e6 }] };
    let mutation = En1993Mutation::UpdateFatigueInputs(update_fatigue_inputs::UpdateFatigueInputs { fatigue_detail: detail });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.fatigue_details.iter().find(|f| f.id == "fat-new").unwrap().category, 56);
}

#[test]
fn update_fire_inputs_upserts_exposure() {
    let before = base();
    let mut fire = before.fire_exposures[0].clone();
    fire.rating = "r90".into();
    fire.protection_thickness = 0.025;
    let mutation = En1993Mutation::UpdateFireInputs(update_fire_inputs::UpdateFireInputs { fire_exposure: fire.clone() });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.fire_exposures.iter().find(|f| f.id == fire.id).unwrap().rating, "r90");
}

#[test]
fn update_cold_formed_inputs_upserts_member() {
    let before = base();
    let cf = ColdFormedMember {
        id: "cf-1".into(),
        b_bar: 0.080,
        thickness: 0.003,
        k_sigma: 0.43,
        psi: -1.0,
        fy: 355.0e6,
        gross_resistance: 120.0e3, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 50.0e3 }] };
    let mutation = En1993Mutation::UpdateColdFormedInputs(update_cold_formed_inputs::UpdateColdFormedInputs { cold_formed_member: cf });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.cold_formed_members.iter().find(|c| c.id == "cf-1").unwrap().thickness, 0.003);
}

#[test]
fn update_plated_inputs_upserts_panel() {
    let before = base();
    let panel = PlatedPanel {
        id: "pl-1".into(),
        a: 1.0,
        b: 0.5,
        thickness: 0.012,
        fy: 355.0e6,
        k_sigma: 4.0, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 200.0e6 }] };
    let mutation = En1993Mutation::UpdatePlatedInputs(update_plated_inputs::UpdatePlatedInputs { plated_panel: panel });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.plated_panels.iter().find(|p| p.id == "pl-1").unwrap().thickness, 0.012);
}

#[test]
fn update_silo_shell_inputs_upserts_shell() {
    let before = base();
    let silo = SiloShell {
        id: "silo-1".into(),
        thickness: 0.008,
        radius: 3.0,
        depth: 12.0,
        k: 0.5,
        gamma: 9.0e3,
        fy: 355.0e6,
    };
    let mutation = En1993Mutation::UpdateSiloShellInputs(update_silo_shell_inputs::UpdateSiloShellInputs { silo_shell: silo });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.silo_shells.iter().find(|s| s.id == "silo-1").unwrap().radius, 3.0);
}

#[test]
fn update_tension_component_inputs_upserts_component() {
    let before = base();
    let tc = TensionComponent { id: "tc-1".into(), f_uk: 500.0e3, f_k: 400.0e3, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 300.0e3 }] };
    let mutation = En1993Mutation::UpdateTensionComponentInputs(update_tension_component_inputs::UpdateTensionComponentInputs { tension_component: tc });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.tension_components.iter().find(|t| t.id == "tc-1").unwrap().actions[0].force, 300.0e3);
}

#[test]
fn update_bridge_inputs_upserts_fatigue() {
    let before = base();
    let bf = BridgeFatigue {
        id: "br-1".into(),
        member_id: before.members[0].id.clone(),
        lambda: 1.2,
        phi2: 1.1,
        delta_sigma_p: 60.0e6,
        category: 71,
        method: "safe_life".into(),
    };
    let mutation = En1993Mutation::UpdateBridgeInputs(update_bridge_inputs::UpdateBridgeInputs { bridge_fatigue_item: bf });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.bridge_fatigue.iter().find(|b| b.id == "br-1").unwrap().lambda, 1.2);
}

#[test]
fn update_tower_inputs_upserts_leg() {
    let before = base();
    let tower = TowerLeg {
        id: "tw-1".into(),
        member_id: before.members[0].id.clone(),
        force_coefficient: 1.2,
        dynamic_factor: 1.5, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 400.0e3 }] };
    let mutation = En1993Mutation::UpdateTowerInputs(update_tower_inputs::UpdateTowerInputs { tower_leg: tower });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.tower_legs.iter().find(|t| t.id == "tw-1").unwrap().force_coefficient, 1.2);
    assert_eq!(after.tower_legs.iter().find(|t| t.id == "tw-1").unwrap().dynamic_factor, 1.5);
}

#[test]
fn update_pile_inputs_upserts_pile() {
    let before = base();
    let pile = SteelPile {
        id: "pile-1".into(),
        section_id: before.sections[0].id.clone(),
        material_id: before.materials[0].id.clone(),
        driving_stress: 200.0e6,
        embedded_length: 8.0,
        shaft_perimeter: 1.2, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 1500.0e3 }] };
    let mutation = En1993Mutation::UpdatePileInputs(update_pile_inputs::UpdatePileInputs { pile });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert!((after.piles.iter().find(|p| p.id == "pile-1").unwrap().driving_stress - 200.0e6).abs() < 1.0);
}

#[test]
fn update_crane_inputs_upserts_runway() {
    let before = base();
    let crane = CraneRunway {
        id: "cr-1".into(),
        member_id: before.members[0].id.clone(),
        wheel_contact_length: 0.050,
        dispersion: 0.015,
        web_thickness: 0.012,
        phi: 1.1, fy: 355.0e6, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "q-imposed".into(), force: 200.0e3 }] };
    let mutation = En1993Mutation::UpdateCraneInputs(update_crane_inputs::UpdateCraneInputs { crane_runway: crane });
    let (after, _) = apply_en1993_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.crane_runways.iter().find(|c| c.id == "cr-1").unwrap().actions[0].force, 200.0e3);
}

#[test]
fn from_snapshot_emits_annex_and_member_list_rewrite() {
    let before = base();
    let mut after = before.clone();
    after.annex = AnnexChoice::En;
    after.members[0].length = 12.0;
    let mutations = En1993Mutation::from_snapshot(&before, &after);
    assert!(mutations.iter().any(|m| matches!(m, En1993Mutation::ChangeAnnex(_))));
    assert!(mutations.iter().any(|m| matches!(m, En1993Mutation::RemoveMember(_))));
    assert!(mutations.iter().any(|m| matches!(m, En1993Mutation::InsertMember(_))));
}


#[test]
fn insert_and_remove_member_round_trip() {
    let before = base();
    let member = SteelMember {
        id: "member-x".into(),
        label: "X".into(),
        member_type: "beam".into(),
        section_id: "sec-heb240".into(),
        material_id: "mat-s355".into(),
        length: 3.0,
        buckling_length_y: 3.0,
        buckling_length_z: 3.0,
        ltb_length: 3.0,
        ltb_restraint_spacing: 1.5,
        load_application: "shearCenter".into(),
        end_moment_ratio_psi: -1.0,
        moment_diagram: "linear".into(),
        deflection_limit_ratio: 300.0,
        analysis: "elastic".into(),
    };
    let insert = En1993Mutation::InsertMember(super::insert_member::InsertMember { index: before.members.len(), member: member.clone() });
    let (mid, _) = apply_en1993_mutation(&before, &insert).expect("insert");
    assert_eq!(mid.members.len(), before.members.len() + 1);
    let remove = En1993Mutation::RemoveMember(super::remove_member::RemoveMember { index: mid.members.len() - 1 });
    let (after, _) = apply_en1993_mutation(&mid, &remove).expect("remove");
    assert_eq!(after.members.len(), before.members.len());
}

#[test]
fn from_snapshot_emits_insert_remove_for_member_list_rewrite() {
    let before = base();
    let mut target = before.clone();
    target.members.push(SteelMember {
        id: "member-y".into(),
        label: "Y".into(),
        member_type: "column".into(),
        section_id: "sec-heb240".into(),
        material_id: "mat-s355".into(),
        length: 5.0,
        buckling_length_y: 5.0,
        buckling_length_z: 5.0,
        ltb_length: 5.0,
        ltb_restraint_spacing: 2.5,
        load_application: "shearCenter".into(),
        end_moment_ratio_psi: 0.0,
        moment_diagram: "linear".into(),
        deflection_limit_ratio: 300.0,
        analysis: "elastic".into(),
    });
    let ops = En1993Mutation::from_snapshot(&before, &target);
    assert!(ops.iter().any(|m| matches!(m, En1993Mutation::RemoveMember(_))));
    assert!(ops.iter().any(|m| matches!(m, En1993Mutation::InsertMember(_))));
}
