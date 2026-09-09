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
    assert_eq!(serde_json::from_str::<Vec<String>>(&scene.columns_json).unwrap(), ["#", "x", "y"]);
    let expected: Vec<Vec<String>> = points.iter().enumerate().map(|(index, point)| vec![index.to_string(), point.x.to_string(), point.y.to_string()]).collect();
    assert_eq!(serde_json::from_str::<Vec<Vec<String>>>(&scene.rows_json).unwrap(), expected);
}
