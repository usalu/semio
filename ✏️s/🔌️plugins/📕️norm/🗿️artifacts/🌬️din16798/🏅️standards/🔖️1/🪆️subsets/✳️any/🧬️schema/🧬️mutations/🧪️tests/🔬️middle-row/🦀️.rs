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

fn padded() -> Din16798Snapshot {
    let mut base = Din16798Snapshot::default();
    pad(&mut base.zones, |row, id| row.id = id);
    pad(&mut base.vent_systems, |row, id| row.id = id);
    base
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_zones_row_is_undone_at_its_index() {
    let base = padded();
    let remove = Din16798Mutation::RemoveZone(remove_zone::RemoveZone { zone_id: base.zones[1].id.clone() });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din16798Mutation::InsertZone(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_zones_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = Din16798Mutation::InsertZone(insert_zone::InsertZone { index: Some(1), zone: { let mut row = base.zones[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_zones_row() {
    let base = padded();
    let insert = Din16798Mutation::InsertZone(insert_zone::InsertZone { index: None, zone: { let mut row = base.zones[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din16798Mutation::RemoveZone(remove)] if remove.zone_id == "fresh"), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_vent_systems_row_is_undone_at_its_index() {
    let base = padded();
    let remove = Din16798Mutation::RemoveVentSystem(remove_vent_system::RemoveVentSystem { vent_id: base.vent_systems[1].id.clone() });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din16798Mutation::InsertVentSystem(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_vent_systems_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = Din16798Mutation::InsertVentSystem(insert_vent_system::InsertVentSystem { index: Some(1), vent: { let mut row = base.vent_systems[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_vent_systems_row() {
    let base = padded();
    let insert = Din16798Mutation::InsertVentSystem(insert_vent_system::InsertVentSystem { index: None, vent: { let mut row = base.vent_systems[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din16798Mutation::RemoveVentSystem(remove)] if remove.vent_id == "fresh"), "the inverse must remove the appended row: {inverse:?}");
}

#[test]
fn removing_a_row_by_path_resolves_to_its_id() {
    use crate::app_surface::NormEdit;
    let base = padded();
    for (path, index) in [("zones", 1), ("zones[1]", 0), ("zones[id=pad-1]", 0)] {
        let resolved = resolve_edit(&base, &NormEdit::RemoveItem { path: path.to_string(), index }).expect("resolves");
        assert_eq!(resolved, vec![Din16798Mutation::RemoveZone(remove_zone::RemoveZone { zone_id: "pad-1".into() })], "{path}");
    }
    let resolved = resolve_edit(&base, &NormEdit::RemoveItem { path: "ventSystems".to_string(), index: 1 }).expect("resolves");
    assert_eq!(resolved, vec![Din16798Mutation::RemoveVentSystem(remove_vent_system::RemoveVentSystem { vent_id: base.vent_systems[1].id.clone() })]);
    assert!(resolve_edit(&base, &NormEdit::RemoveItem { path: "zones".to_string(), index: 9 }).is_err());
}
