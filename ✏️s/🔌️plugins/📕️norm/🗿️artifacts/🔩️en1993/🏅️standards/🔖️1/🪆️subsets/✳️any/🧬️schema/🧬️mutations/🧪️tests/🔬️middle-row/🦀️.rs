//! 🧭️ Position-exact inverses: removing or inserting a MIDDLE row restores the document exactly, and the inverse names the original index.

use super::*;
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
use protocol::Mutation as _;

fn pad<T: Clone>(rows: &mut Vec<T>, rename: impl Fn(&mut T, String)) {
    while rows.len() < 3 {
        let mut row = rows[0].clone();
        rename(&mut row, format!("pad-{}", rows.len()));
        rows.push(row);
    }
}

fn padded() -> En1993Snapshot {
    let mut base = En1993Snapshot::default();
    pad(&mut base.materials, |row, id| row.id = id);
    pad(&mut base.sections, |row, id| row.id = id);
    pad(&mut base.members, |row, id| row.id = id);
    pad(&mut base.load_cases, |row, id| row.id = id);
    pad(&mut base.member_actions, |row, id| row.id = id);
    pad(&mut base.joints, |row, id| row.id = id);
    pad(&mut base.fatigue_details, |row, id| row.id = id);
    pad(&mut base.fire_exposures, |row, id| row.id = id);
    pad(&mut base.cold_formed_members, |row, id| row.id = id);
    pad(&mut base.plated_panels, |row, id| row.id = id);
    pad(&mut base.silo_shells, |row, id| row.id = id);
    pad(&mut base.tension_components, |row, id| row.id = id);
    pad(&mut base.bridge_fatigue, |row, id| row.id = id);
    pad(&mut base.tower_legs, |row, id| row.id = id);
    pad(&mut base.piles, |row, id| row.id = id);
    pad(&mut base.crane_runways, |row, id| row.id = id);
    base
}

fn fresh<T: Clone>(rows: &[T], rename: impl Fn(&mut T, String)) -> T {
    let mut row = rows[0].clone();
    rename(&mut row, "fresh".into());
    row
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_materials_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveMaterial(remove_material::RemoveMaterial { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertMaterial(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_materials_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertMaterial(insert_material::InsertMaterial { index: Some(1), material: fresh(&base.materials, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_materials_row() {
    let base = padded();
    let insert = En1993Mutation::InsertMaterial(insert_material::InsertMaterial { index: None, material: fresh(&base.materials, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveMaterial(remove)] if remove.index == base.materials.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_sections_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveSection(remove_section::RemoveSection { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertSection(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_sections_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertSection(insert_section::InsertSection { index: Some(1), section: fresh(&base.sections, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_sections_row() {
    let base = padded();
    let insert = En1993Mutation::InsertSection(insert_section::InsertSection { index: None, section: fresh(&base.sections, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveSection(remove)] if remove.index == base.sections.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_members_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveMember(remove_member::RemoveMember { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertMember(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_members_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertMember(insert_member::InsertMember { index: Some(1), member: fresh(&base.members, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_members_row() {
    let base = padded();
    let insert = En1993Mutation::InsertMember(insert_member::InsertMember { index: None, member: fresh(&base.members, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveMember(remove)] if remove.index == base.members.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_load_cases_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveLoadCase(remove_load_case::RemoveLoadCase { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertLoadCase(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_load_cases_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertLoadCase(insert_load_case::InsertLoadCase { index: Some(1), load_case: fresh(&base.load_cases, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_load_cases_row() {
    let base = padded();
    let insert = En1993Mutation::InsertLoadCase(insert_load_case::InsertLoadCase { index: None, load_case: fresh(&base.load_cases, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveLoadCase(remove)] if remove.index == base.load_cases.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_member_actions_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveMemberAction(remove_member_action::RemoveMemberAction { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertMemberAction(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_member_actions_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertMemberAction(insert_member_action::InsertMemberAction { index: Some(1), member_action: fresh(&base.member_actions, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_member_actions_row() {
    let base = padded();
    let insert = En1993Mutation::InsertMemberAction(insert_member_action::InsertMemberAction { index: None, member_action: fresh(&base.member_actions, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveMemberAction(remove)] if remove.index == base.member_actions.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_joints_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveJoint(remove_joint::RemoveJoint { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertJoint(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_joints_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertJoint(insert_joint::InsertJoint { index: Some(1), joint: fresh(&base.joints, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_joints_row() {
    let base = padded();
    let insert = En1993Mutation::InsertJoint(insert_joint::InsertJoint { index: None, joint: fresh(&base.joints, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveJoint(remove)] if remove.index == base.joints.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_fatigue_details_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveFatigueDetail(remove_fatigue_detail::RemoveFatigueDetail { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertFatigueDetail(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_fatigue_details_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertFatigueDetail(insert_fatigue_detail::InsertFatigueDetail { index: Some(1), fatigue_detail: fresh(&base.fatigue_details, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_fatigue_details_row() {
    let base = padded();
    let insert = En1993Mutation::InsertFatigueDetail(insert_fatigue_detail::InsertFatigueDetail { index: None, fatigue_detail: fresh(&base.fatigue_details, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveFatigueDetail(remove)] if remove.index == base.fatigue_details.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_fire_exposures_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveFireExposure(remove_fire_exposure::RemoveFireExposure { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertFireExposure(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_fire_exposures_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertFireExposure(insert_fire_exposure::InsertFireExposure { index: Some(1), fire_exposure: fresh(&base.fire_exposures, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_fire_exposures_row() {
    let base = padded();
    let insert = En1993Mutation::InsertFireExposure(insert_fire_exposure::InsertFireExposure { index: None, fire_exposure: fresh(&base.fire_exposures, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveFireExposure(remove)] if remove.index == base.fire_exposures.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_cold_formed_members_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveColdFormedMember(remove_cold_formed_member::RemoveColdFormedMember { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertColdFormedMember(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_cold_formed_members_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertColdFormedMember(insert_cold_formed_member::InsertColdFormedMember { index: Some(1), cold_formed_member: fresh(&base.cold_formed_members, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_cold_formed_members_row() {
    let base = padded();
    let insert = En1993Mutation::InsertColdFormedMember(insert_cold_formed_member::InsertColdFormedMember { index: None, cold_formed_member: fresh(&base.cold_formed_members, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveColdFormedMember(remove)] if remove.index == base.cold_formed_members.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_plated_panels_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemovePlatedPanel(remove_plated_panel::RemovePlatedPanel { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertPlatedPanel(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_plated_panels_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertPlatedPanel(insert_plated_panel::InsertPlatedPanel { index: Some(1), plated_panel: fresh(&base.plated_panels, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_plated_panels_row() {
    let base = padded();
    let insert = En1993Mutation::InsertPlatedPanel(insert_plated_panel::InsertPlatedPanel { index: None, plated_panel: fresh(&base.plated_panels, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemovePlatedPanel(remove)] if remove.index == base.plated_panels.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_silo_shells_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveSiloShell(remove_silo_shell::RemoveSiloShell { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertSiloShell(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_silo_shells_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertSiloShell(insert_silo_shell::InsertSiloShell { index: Some(1), silo_shell: fresh(&base.silo_shells, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_silo_shells_row() {
    let base = padded();
    let insert = En1993Mutation::InsertSiloShell(insert_silo_shell::InsertSiloShell { index: None, silo_shell: fresh(&base.silo_shells, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveSiloShell(remove)] if remove.index == base.silo_shells.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_tension_components_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveTensionComponent(remove_tension_component::RemoveTensionComponent { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertTensionComponent(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_tension_components_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertTensionComponent(insert_tension_component::InsertTensionComponent { index: Some(1), tension_component: fresh(&base.tension_components, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_tension_components_row() {
    let base = padded();
    let insert = En1993Mutation::InsertTensionComponent(insert_tension_component::InsertTensionComponent { index: None, tension_component: fresh(&base.tension_components, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveTensionComponent(remove)] if remove.index == base.tension_components.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_bridge_fatigue_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveBridgeFatigue(remove_bridge_fatigue::RemoveBridgeFatigue { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertBridgeFatigue(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_bridge_fatigue_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertBridgeFatigue(insert_bridge_fatigue::InsertBridgeFatigue { index: Some(1), bridge_fatigue_item: fresh(&base.bridge_fatigue, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_bridge_fatigue_row() {
    let base = padded();
    let insert = En1993Mutation::InsertBridgeFatigue(insert_bridge_fatigue::InsertBridgeFatigue { index: None, bridge_fatigue_item: fresh(&base.bridge_fatigue, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveBridgeFatigue(remove)] if remove.index == base.bridge_fatigue.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_tower_legs_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveTowerLeg(remove_tower_leg::RemoveTowerLeg { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertTowerLeg(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_tower_legs_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertTowerLeg(insert_tower_leg::InsertTowerLeg { index: Some(1), tower_leg: fresh(&base.tower_legs, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_tower_legs_row() {
    let base = padded();
    let insert = En1993Mutation::InsertTowerLeg(insert_tower_leg::InsertTowerLeg { index: None, tower_leg: fresh(&base.tower_legs, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveTowerLeg(remove)] if remove.index == base.tower_legs.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_piles_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemovePile(remove_pile::RemovePile { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertPile(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_piles_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertPile(insert_pile::InsertPile { index: Some(1), pile: fresh(&base.piles, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_piles_row() {
    let base = padded();
    let insert = En1993Mutation::InsertPile(insert_pile::InsertPile { index: None, pile: fresh(&base.piles, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemovePile(remove)] if remove.index == base.piles.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_crane_runways_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1993Mutation::RemoveCraneRunway(remove_crane_runway::RemoveCraneRunway { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::InsertCraneRunway(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_crane_runways_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1993Mutation::InsertCraneRunway(insert_crane_runway::InsertCraneRunway { index: Some(1), crane_runway: fresh(&base.crane_runways, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_crane_runways_row() {
    let base = padded();
    let insert = En1993Mutation::InsertCraneRunway(insert_crane_runway::InsertCraneRunway { index: None, crane_runway: fresh(&base.crane_runways, |row, id| row.id = id) });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1993Mutation::RemoveCraneRunway(remove)] if remove.index == base.crane_runways.len()), "the inverse must remove the appended row: {inverse:?}");
}
