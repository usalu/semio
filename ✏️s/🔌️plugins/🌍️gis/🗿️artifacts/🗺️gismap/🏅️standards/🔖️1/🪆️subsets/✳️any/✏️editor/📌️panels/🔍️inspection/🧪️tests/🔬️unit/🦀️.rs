use super::*;
use crate::editor::gis2d::unit_tests::context::{app, close, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_inspector_always_summarises_the_schema_and_visible_count() {
    let mut app = app().await;
    let json = render_body(&mut app, GIS2D_PLAY_BODY_INSPECTION).await;
    assert!(json.contains(GIS_MAP_SCHEMA));
    assert!(json.contains(&format!("{}/{}", GIS_MAP_LAYER_IDS.len(), GIS_MAP_LAYER_IDS.len())));
    drop(json);
    close(&mut app);
}

/// 🕹️ The interaction-view threading law for this panel: a `"layer"` pick in the framework-owned
/// `"features"` domain adds that layer's own detail rows, a `"feature"` pick adds the picked
/// feature's id and its document kind, and an empty domain renders the summary alone.
#[semio_framework_async_macros::async_test]
async fn the_inspector_detail_section_follows_the_features_selection() {
    let document = crate::schema::default_document();
    let cfg = MapWindowConfig::default();
    let labels = crate::editor::gis2d::terminology::gis2d_labels(&semio_framework_plugin::ViewModel::default());
    // 🧩️ A `BuiltNode` carries retained page children, so it is projected through the fixture
    // transport (the same route `unit_tests::context::render` takes) rather than serialized directly.
    let json = |interaction: &Gis2dInteractionSnapshot| -> String {
        let tree = semio_framework_plugin::built_to_component_tree(render(&document, &cfg, interaction, labels).expect("inspector"));
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("inspector projection")
    };

    let empty = json(&Gis2dInteractionSnapshot::default());
    assert!(!empty.contains("gis2d-play-inspector.layer.id"), "an empty domain must render the summary alone: {empty}");

    let layer = Gis2dInteractionSnapshot { granularity: "layer".into(), ids: vec!["routes".into()], hovered_ids: Vec::new() };
    let layer_json = json(&layer);
    assert!(layer_json.contains("gis2d-play-inspector.layer.id"), "a layer pick must add the layer detail section: {layer_json}");
    assert!(layer_json.contains("routes"), "the layer detail section must name the picked layer: {layer_json}");

    let feature_id = document.positions.first().expect("the default fixture carries a position").id.clone();
    let feature = Gis2dInteractionSnapshot { granularity: GIS2D_FEATURE_GRANULARITY.into(), ids: vec![feature_id.clone()], hovered_ids: Vec::new() };
    let feature_json = json(&feature);
    assert!(feature_json.contains(&feature_id), "a feature pick must name the picked feature: {feature_json}");
    assert!(feature_json.contains("position"), "a feature pick must report the document kind it resolves to: {feature_json}");
}

/// 🪞️ B3a2: the "panel reflects state" half of the interaction bar. Before this, NO gis2d panel
/// projected the document — the artifact tree lists the eleven fixed layer ids and the inspector
/// summarised schema + layer visibility + selection size only, so `addFeature`/`deleteFeature` moved
/// the ledger and the edit count while every panel stayed byte-identical. The summary now carries
/// each collection's own extent, so a per-feature verb is visible in the UI it dispatched from.
#[semio_framework_async_macros::async_test]
async fn the_inspector_summary_projects_each_document_collection_extent() {
    let cfg = MapWindowConfig::default();
    let labels = crate::editor::gis2d::terminology::gis2d_labels(&semio_framework_plugin::ViewModel::default());
    let json = |document: &GisMapSnapshot| -> String {
        let tree = semio_framework_plugin::built_to_component_tree(render(document, &cfg, &Gis2dInteractionSnapshot::default(), labels).expect("inspector"));
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("inspector projection")
    };

    let mut document = crate::schema::default_document();
    let before = json(&document);
    for row in ["gis2d-play-inspector.positions-count", "gis2d-play-inspector.routes-count", "gis2d-play-inspector.regions-count"] {
        assert!(before.contains(row), "the summary must carry {row}: {before}");
    }
    assert!(before.contains(&document.positions.len().to_string()), "the positions row must print the document's own extent: {before}");

    document.positions.push(crate::MapFeature { id: "position-probe".into(), data: dsl::DslValue::Null });
    let after = json(&document);
    assert_ne!(before, after, "appending one position must move the inspector projection");
    assert!(after.contains(&document.positions.len().to_string()), "the positions row must follow the document: {after}");
}

#[semio_framework_async_macros::async_test]
async fn the_definition_binds_the_framework_inspection_tab_to_this_body() {
    let definition = definition();
    assert!(matches!(definition.group, PanelGroup::Details));
    assert_eq!(definition.body_key.as_deref(), Some(GIS2D_PLAY_BODY_INSPECTION));
}
