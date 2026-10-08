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

fn padded() -> En1998Snapshot {
    let mut base = En1998Snapshot::default();
    pad(&mut base.buildings, |row, id| row.id = id);
    pad(&mut base.bridges, |row, id| row.id = id);
    pad(&mut base.assessments, |row, id| row.id = id);
    pad(&mut base.silos, |row, id| row.id = id);
    pad(&mut base.tanks, |row, id| row.id = id);
    pad(&mut base.foundations, |row, id| row.id = id);
    pad(&mut base.retaining_walls, |row, id| row.id = id);
    pad(&mut base.towers, |row, id| row.id = id);
    base
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_buildings_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1998Mutation::RemoveBuilding(remove_building::RemoveBuilding { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::InsertBuilding(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_buildings_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1998Mutation::InsertBuilding(insert_building::InsertBuilding { index: Some(1), building: { let mut row = base.buildings[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_buildings_row() {
    let base = padded();
    let insert = En1998Mutation::InsertBuilding(insert_building::InsertBuilding { index: None, building: { let mut row = base.buildings[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::RemoveBuilding(remove)] if remove.index == base.buildings.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_bridges_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1998Mutation::RemoveBridge(remove_bridge::RemoveBridge { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::InsertBridge(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_bridges_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1998Mutation::InsertBridge(insert_bridge::InsertBridge { index: Some(1), bridge: { let mut row = base.bridges[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_bridges_row() {
    let base = padded();
    let insert = En1998Mutation::InsertBridge(insert_bridge::InsertBridge { index: None, bridge: { let mut row = base.bridges[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::RemoveBridge(remove)] if remove.index == base.bridges.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_assessments_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1998Mutation::RemoveAssessment(remove_assessment::RemoveAssessment { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::InsertAssessment(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_assessments_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1998Mutation::InsertAssessment(insert_assessment::InsertAssessment { index: Some(1), assessment: { let mut row = base.assessments[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_assessments_row() {
    let base = padded();
    let insert = En1998Mutation::InsertAssessment(insert_assessment::InsertAssessment { index: None, assessment: { let mut row = base.assessments[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::RemoveAssessment(remove)] if remove.index == base.assessments.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_silos_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1998Mutation::RemoveSilo(remove_silo::RemoveSilo { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::InsertSilo(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_silos_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1998Mutation::InsertSilo(insert_silo::InsertSilo { index: Some(1), silo: { let mut row = base.silos[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_silos_row() {
    let base = padded();
    let insert = En1998Mutation::InsertSilo(insert_silo::InsertSilo { index: None, silo: { let mut row = base.silos[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::RemoveSilo(remove)] if remove.index == base.silos.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_tanks_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1998Mutation::RemoveTank(remove_tank::RemoveTank { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::InsertTank(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_tanks_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1998Mutation::InsertTank(insert_tank::InsertTank { index: Some(1), tank: { let mut row = base.tanks[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_tanks_row() {
    let base = padded();
    let insert = En1998Mutation::InsertTank(insert_tank::InsertTank { index: None, tank: { let mut row = base.tanks[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::RemoveTank(remove)] if remove.index == base.tanks.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_foundations_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1998Mutation::RemoveFoundation(remove_foundation::RemoveFoundation { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::InsertFoundation(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_foundations_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1998Mutation::InsertFoundation(insert_foundation::InsertFoundation { index: Some(1), foundation: { let mut row = base.foundations[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_foundations_row() {
    let base = padded();
    let insert = En1998Mutation::InsertFoundation(insert_foundation::InsertFoundation { index: None, foundation: { let mut row = base.foundations[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::RemoveFoundation(remove)] if remove.index == base.foundations.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_retaining_walls_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1998Mutation::RemoveRetainingWall(remove_retaining_wall::RemoveRetainingWall { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::InsertRetainingWall(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_retaining_walls_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1998Mutation::InsertRetainingWall(insert_retaining_wall::InsertRetainingWall { index: Some(1), wall: { let mut row = base.retaining_walls[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_retaining_walls_row() {
    let base = padded();
    let insert = En1998Mutation::InsertRetainingWall(insert_retaining_wall::InsertRetainingWall { index: None, wall: { let mut row = base.retaining_walls[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::RemoveRetainingWall(remove)] if remove.index == base.retaining_walls.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_towers_row_is_undone_at_its_index() {
    let base = padded();
    let remove = En1998Mutation::RemoveTower(remove_tower::RemoveTower { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::InsertTower(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_towers_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = En1998Mutation::InsertTower(insert_tower::InsertTower { index: Some(1), tower: { let mut row = base.towers[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_towers_row() {
    let base = padded();
    let insert = En1998Mutation::InsertTower(insert_tower::InsertTower { index: None, tower: { let mut row = base.towers[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [En1998Mutation::RemoveTower(remove)] if remove.index == base.towers.len()), "the inverse must remove the appended row: {inverse:?}");
}
