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

fn padded() -> Din4108Snapshot {
    let mut base = Din4108Snapshot::default();
    pad(&mut base.zones, |row, id| row.id = id);
    pad(&mut base.elements, |row, id| row.id = id);
    pad(&mut base.thermal_bridges, |row, id| row.id = id);
    pad(&mut base.zones[0].windows, |row, id| row.id = id);
    pad(&mut base.elements[0].layers, |row, id| row.id = id);
    base
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_zones_row_is_undone_at_its_index() {
    let base = padded();
    let remove = Din4108Mutation::RemoveZone(remove_zone::RemoveZone { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din4108Mutation::InsertZone(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_zones_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = Din4108Mutation::InsertZone(insert_zone::InsertZone { index: Some(1), zone: { let mut row = base.zones[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_zones_row() {
    let base = padded();
    let insert = Din4108Mutation::InsertZone(insert_zone::InsertZone { index: None, zone: { let mut row = base.zones[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din4108Mutation::RemoveZone(remove)] if remove.index == base.zones.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_elements_row_is_undone_at_its_index() {
    let base = padded();
    let remove = Din4108Mutation::RemoveElement(remove_element::RemoveElement { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din4108Mutation::InsertElement(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_elements_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = Din4108Mutation::InsertElement(insert_element::InsertElement { index: Some(1), element: { let mut row = base.elements[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_elements_row() {
    let base = padded();
    let insert = Din4108Mutation::InsertElement(insert_element::InsertElement { index: None, element: { let mut row = base.elements[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din4108Mutation::RemoveElement(remove)] if remove.index == base.elements.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_thermal_bridges_row_is_undone_at_its_index() {
    let base = padded();
    let remove = Din4108Mutation::RemoveThermalBridge(remove_thermal_bridge::RemoveThermalBridge { index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din4108Mutation::InsertThermalBridge(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_thermal_bridges_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = Din4108Mutation::InsertThermalBridge(insert_thermal_bridge::InsertThermalBridge { index: Some(1), bridge: { let mut row = base.thermal_bridges[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_thermal_bridges_row() {
    let base = padded();
    let insert = Din4108Mutation::InsertThermalBridge(insert_thermal_bridge::InsertThermalBridge { index: None, bridge: { let mut row = base.thermal_bridges[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din4108Mutation::RemoveThermalBridge(remove)] if remove.index == base.thermal_bridges.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_layers_row_is_undone_at_its_index() {
    let base = padded();
    let remove = Din4108Mutation::RemoveLayer(remove_layer::RemoveLayer { element_id: base.elements[0].id.clone(), index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din4108Mutation::InsertLayer(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_layers_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = Din4108Mutation::InsertLayer(insert_layer::InsertLayer { element_id: base.elements[0].id.clone(), index: Some(1), layer: { let mut row = base.elements[0].layers[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_layers_row() {
    let base = padded();
    let insert = Din4108Mutation::InsertLayer(insert_layer::InsertLayer { element_id: base.elements[0].id.clone(), index: None, layer: { let mut row = base.elements[0].layers[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din4108Mutation::RemoveLayer(remove)] if remove.index == base.elements[0].layers.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn removing_the_middle_zone_windows_row_is_undone_at_its_index() {
    let base = padded();
    let remove = Din4108Mutation::RemoveZoneWindow(remove_zone_window::RemoveZoneWindow { zone_id: base.zones[0].id.clone(), index: 1 });
    assert_mutation_inverse_sum_law(&remove, &base).await;
    let inverse = remove.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din4108Mutation::InsertZoneWindow(insert)] if insert.index == Some(1)), "the inverse must re-insert at the original index: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn inserting_a_middle_zone_windows_row_is_undone_by_removing_that_row() {
    let base = padded();
    let insert = Din4108Mutation::InsertZoneWindow(insert_zone_window::InsertZoneWindow { zone_id: base.zones[0].id.clone(), index: Some(1), window: { let mut row = base.zones[0].windows[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn inserting_without_a_position_appends_the_zone_windows_row() {
    let base = padded();
    let insert = Din4108Mutation::InsertZoneWindow(insert_zone_window::InsertZoneWindow { zone_id: base.zones[0].id.clone(), index: None, window: { let mut row = base.zones[0].windows[0].clone(); row.id = "fresh".into(); row } });
    assert_mutation_inverse_sum_law(&insert, &base).await;
    let inverse = insert.inverse(&base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [Din4108Mutation::RemoveZoneWindow(remove)] if remove.index == base.zones[0].windows.len()), "the inverse must remove the appended row: {inverse:?}");
}

#[semio_framework_async_macros::async_test]
async fn moving_a_middle_layer_is_undone_by_moving_it_back() {
    let base = padded();
    let reorder = Din4108Mutation::ReorderLayers(reorder_layers::ReorderLayers { element_id: base.elements[0].id.clone(), from: 1, to: 0 });
    assert_mutation_inverse_sum_law(&reorder, &base).await;
}
