use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_table_kind() {
    let definition = definition();
    assert_eq!(definition.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_table_scene_with_one_row_per_point() {
    let document = EquationSnapshot::default();
    let points = document.geometry.points.clone();
    assert!(!points.is_empty());
    let node = render(&document).expect("table surface");
    let scene = semio_framework_plugin::artifact_app_laws::built_surface_scene::<semio_framework_ui_scene::TableScene>(&node).expect("the assembled table scene: the spine with its column/row lanes merged back in");
    // 📊️ `TableWindowKit::render` emits the renderer table contract both hosts read: `columnsJson` as
    // `{id, label}` records and `rowsJson` as `{id, "<column index>": cell}` records — no longer bare
    // strings and bare cell arrays. What this law proves (one row per point, `#`/`x`/`y` columns,
    // index and coordinates as the cell text) is read back out of those records.
    let columns: Vec<String> = serde_json::from_str::<Vec<serde_json::Value>>(&scene.columns_json).unwrap().into_iter().map(|column| column["label"].as_str().expect("column label").to_string()).collect();
    assert_eq!(columns, ["#", "x", "y"]);
    let rows: Vec<Vec<String>> = serde_json::from_str::<Vec<serde_json::Value>>(&scene.rows_json)
        .unwrap()
        .into_iter()
        .map(|record| (0..columns.len()).map(|column| record[column.to_string().as_str()].as_str().expect("table cell").to_string()).collect())
        .collect();
    let expected: Vec<Vec<String>> = points.iter().enumerate().map(|(index, point)| vec![index.to_string(), point.x.to_string(), point.y.to_string()]).collect();
    assert_eq!(rows, expected);
}
