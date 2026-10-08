use super::*;

fn demo() -> (ModelSnapshot, ModelInference) {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let inference = crate::editor::bim::inference::with_inference(None, &snapshot, Clone::clone);
    (snapshot, inference)
}

#[semio_framework_async_macros::async_test]
async fn the_schedule_lists_one_row_per_element_with_the_inferred_quantities_then_the_kind_totals() {
    let (snapshot, inference) = demo();
    let rows = rows(&snapshot, &inference, &BimLabels::NATIVE_EN);
    let elements: Vec<&Row> = rows.iter().filter(|row| !row.total).collect();
    assert_eq!(elements.iter().map(|row| row.id.as_str()).collect::<Vec<_>>(), snapshot.walls.keys().map(String::as_str).collect::<Vec<_>>());
    let south = elements.iter().find(|row| row.id == "w-south").expect("south wall row");
    assert_eq!((south.kind.as_str(), south.storey.as_str(), south.name.as_str(), south.type_name.as_str()), ("Wall", "Ground", "South", "Brick 300"));
    assert!((south.length - 8.0).abs() < 1e-9);
    assert!((south.volume - 8.0 * 0.3 * 3.0).abs() < 1e-6, "length times thickness times height, got {}", south.volume);
    let total = rows.iter().find(|row| row.total && row.id == "total:wall").expect("the wall total");
    assert_eq!(total.count, 4);
    assert!((total.volume - elements.iter().map(|row| row.volume).sum::<f64>()).abs() < 1e-6, "the total is the sum of its rows");
}

#[semio_framework_async_macros::async_test]
async fn the_kind_and_total_labels_follow_the_locale() {
    let (snapshot, inference) = demo();
    let rows = rows(&snapshot, &inference, &BimLabels::NATIVE_DE);
    assert!(rows.iter().any(|row| row.kind == "Wand" && row.name == "South"));
    assert!(rows.iter().any(|row| row.total && row.name.starts_with("Summe")));
}

#[semio_framework_async_macros::async_test]
async fn the_table_is_a_table_surface_in_the_elements_domain() {
    let (snapshot, inference) = demo();
    assert!(matches!(definition().surface_kind, SurfaceKind::Table));
    assert!(render(&snapshot, &inference, &BimLabels::NATIVE_DE).is_ok());
    assert!(render(&ModelSnapshot::default(), &ModelInference::default(), &BimLabels::NATIVE_EN).is_ok());
}
