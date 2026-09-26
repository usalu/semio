use super::*;
use crate::editor::iso16757::unit_tests::context;
use crate::Iso16757Snapshot;
use semio_framework_plugin::{Locale, TreeWindowRequest, TreeWindows, ViewModel, TREE_WINDOW_PATH_SEPARATOR};

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_INPUTS);
    assert_eq!(definition().id, WINDOW_INPUTS);
}

#[semio_framework_async_macros::async_test]
async fn renders_structured_or_windowed_document_editor() {
    let mut app = context::app_with_registry().await;
    let body = context::render(&mut app, BODY_INPUTS).await;
    assert!(!body.is_empty(), "inputs body must render");
    assert!(!body.contains("Unknown body"), "inputs must resolve");
    context::close(&mut app);
}

#[test]
fn full_default_snapshot_inputs_expose_catalogue_sections_within_slots() {
    let document = Iso16757Snapshot::default();
    let root_id = crate::app_surface::inputs_section_id("");
    let catalogue_id = crate::app_surface::inputs_section_id("catalogue");
    let products_id = crate::app_surface::inputs_section_id("catalogue.products");
    let dictionary_id = crate::app_surface::inputs_section_id("dictionary");
    let subjects_id = crate::app_surface::inputs_section_id("dictionary.subjects");
    let geometry_id = crate::app_surface::inputs_section_id("geometry");
    let objects_id = crate::app_surface::inputs_section_id("geometry.objects");
    let catalogue_path = format!("{root_id}{TREE_WINDOW_PATH_SEPARATOR}{catalogue_id}");
    let products_path = format!("{catalogue_path}{TREE_WINDOW_PATH_SEPARATOR}{products_id}");
    let dictionary_path = format!("{root_id}{TREE_WINDOW_PATH_SEPARATOR}{dictionary_id}");
    let subjects_path = format!("{dictionary_path}{TREE_WINDOW_PATH_SEPARATOR}{subjects_id}");
    let geometry_path = format!("{root_id}{TREE_WINDOW_PATH_SEPARATOR}{geometry_id}");
    let objects_path = format!("{geometry_path}{TREE_WINDOW_PATH_SEPARATOR}{objects_id}");

    let collapsed_view = ViewModel {
        tree_windows: vec![TreeWindowRequest {
            body_key: BODY_INPUTS.into(),
            node_key: root_id.clone(),
            open: Some(true),
            offset: 0,
            rows: 16,
        }],
        tree_viewport_rows: Some(16),
        ..Default::default()
    };
    let collapsed = render(&document, Locale::En, "norm.iso16757", &TreeWindows::for_body(&collapsed_view, BODY_INPUTS)).expect("collapsed assemble");
    let collapsed_json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(collapsed)).expect("collapsed retire");
    assert!(collapsed_json.contains("norm-inputs-catalogue") || collapsed_json.contains("catalogue"), "{collapsed_json}");
    assert!(collapsed_json.contains("dictionary") || collapsed_json.contains("Dictionary"), "{collapsed_json}");
    assert!(collapsed_json.contains("geometry") || collapsed_json.contains("Geometry"), "{collapsed_json}");

    let product_path = format!("{products_path}{TREE_WINDOW_PATH_SEPARATOR}{}", crate::app_surface::inputs_section_id("catalogue.products[id=product-cv]"));
    let expanded_view = ViewModel {
        tree_windows: vec![
            TreeWindowRequest { body_key: BODY_INPUTS.into(), node_key: root_id, open: Some(true), offset: 0, rows: 16 },
            TreeWindowRequest { body_key: BODY_INPUTS.into(), node_key: catalogue_path, open: Some(true), offset: 0, rows: 16 },
            TreeWindowRequest { body_key: BODY_INPUTS.into(), node_key: products_path.clone(), open: Some(true), offset: 0, rows: 8 },
            TreeWindowRequest { body_key: BODY_INPUTS.into(), node_key: product_path, open: Some(true), offset: 0, rows: 16 },
            TreeWindowRequest { body_key: BODY_INPUTS.into(), node_key: dictionary_path, open: Some(true), offset: 0, rows: 8 },
            TreeWindowRequest { body_key: BODY_INPUTS.into(), node_key: subjects_path, open: Some(true), offset: 0, rows: 8 },
            TreeWindowRequest { body_key: BODY_INPUTS.into(), node_key: geometry_path, open: Some(true), offset: 0, rows: 8 },
            TreeWindowRequest { body_key: BODY_INPUTS.into(), node_key: objects_path, open: Some(true), offset: 0, rows: 8 },
        ],
        tree_viewport_rows: Some(16),
        ..Default::default()
    };
    let expanded = render(&document, Locale::En, "norm.iso16757", &TreeWindows::for_body(&expanded_view, BODY_INPUTS)).expect("expanded assemble");
    let expanded_json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(expanded)).expect("expanded retire");
    assert!(expanded_json.contains("norm-inputs-catalogue.products") || expanded_json.contains("products"), "{expanded_json}");
    assert!(expanded_json.contains("subjects") || expanded_json.contains("Subjects"), "{expanded_json}");
    assert!(expanded_json.contains("objects") || expanded_json.contains("Objects") || expanded_json.contains("geom-valve"), "{expanded_json}");
    assert!(expanded_json.contains("setField"), "expanded product leaf must bind setField: {expanded_json}");
    assert!(expanded_json.contains("catalogue.products") && (expanded_json.contains("seriesId") || expanded_json.contains("series_id") || expanded_json.contains("product-cv")), "{expanded_json}");

    let mut tree = dsl::ToValue::to_value(&document);
    crate::app_surface::set_value_at_path(&mut tree, "catalogue.products[id=product-cv].seriesId", dsl::DslValue::String("series-cv-edited".into())).expect("setField path");
    assert_eq!(
        crate::app_surface::get_value_at_path(&tree, "catalogue.products[id=product-cv].seriesId").ok(),
        Some(&dsl::DslValue::String("series-cv-edited".into()))
    );
}
