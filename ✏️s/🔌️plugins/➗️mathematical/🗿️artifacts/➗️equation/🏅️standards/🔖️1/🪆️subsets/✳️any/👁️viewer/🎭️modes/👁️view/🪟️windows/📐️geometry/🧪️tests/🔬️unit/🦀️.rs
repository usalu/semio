use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_table_kind() {
    let definition = definition();
    assert_eq!(definition.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_table_scene_with_one_row_per_point() {
    let document = EquationSnapshot::default();
    let points = equation_geometry(&document).points;
    assert!(!points.is_empty());
    let node = render(&document).expect("table surface");
    let semio_framework_plugin::plugin_app_close_prelude::Component::Surface(props) = node.component else { panic!("viewer must render a table surface") };
    let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(&props).expect("table payload");
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
