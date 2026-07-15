//! 🌐 GIS plugin — 2D map app in a hot-swappable WASM component.

pub mod app_2d {
    //! 🗺️ GIS 2D plugin — GIS map play app bundled as a hot-swappable WASM component.

    use gis_2d::{
        clamp_map_layer_weight, empty_gis_map_projection, gis_map_descriptor_json, gis_map_document_from_descriptor_json,
        gis_map_layer_weight_slider_ids_json, gis_map_lod_scale_json,
        open_url, GisMapDocument, GisMapOp, MapFeature, MapFeaturePatch, MapHost, GIS_MAP_LOD_MODE_AUTOMATIC,
        GIS_MAP_SCHEMA,
    };
    use semio_framework_plugin::{SurfaceKind, PanelGroup,
        build_gis_map_scene, create_default_layout, MeasureSelectItem, ui_inspector_groups_to_tree, ui_inspector_mixed_toggle,
        ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionEmit, App, ActionDescriptor, DocumentApp, DocumentView, DwgDrawing, DwgGeometry, GisMapScene,
        UiControlNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiSelectItem, UiSelectNode, UiSliderNode,
        UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowMeasure,
        FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
        FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
        FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{HashMap, HashSet};
    use vcs::CollectionOp;

    //#region 🔖Constants
    const GIS2D_PLAY_APP_ID: &str = "gis2d-play";
    const GIS2D_PLAY_SURFACE: &str = "gis2d.play.composite";
    const GIS2D_PLAY_BODY_COMPOSITE: &str = "gis2d.play.composite";
    const GIS2D_PLAY_BODY_DOCUMENT: &str = "gis2d.play.document";
    const GIS2D_PLAY_BODY_CATALOGUE: &str = "gis2d.play.catalogue";
    const GIS2D_PLAY_BODY_INSPECTION: &str = "gis2d.play.inspection";
    const GIS2D_PLAY_WINDOW_MAIN: &str = "gis2d-main";

    const REUSE_MAP_EXAMPLE_JSON: &str = include_str!("../../2d/example/reuse.map.gis.json");

    const GIS_MAP_LAYER_IDS: &[(&str, &str, &str)] = &[
        ("raster", "Raster", "map"),
        ("water", "Water", "droplets"),
        ("land", "Land", "mountain"),
        ("roads", "Roads", "route"),
        ("buildings", "Buildings", "building"),
        ("borders", "Borders", "square-dashed"),
        ("labels", "Labels", "type"),
        ("positions", "Positions", "map-pin"),
        ("positionLabels", "Position Labels", "tag"),
        ("routes", "Routes", "git-branch"),
        ("regions", "Regions", "layers"),
    ];
    //#endregion 🔖Constants

    //#region 🔖Types
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Gis2dPlayRuntime {
        #[serde(default)]
        selected_ids: Vec<String>,
        #[serde(default)]
        layer_visibility: HashMap<String, bool>,
        #[serde(default = "default_map_camera_json")]
        camera_json: String,
        #[serde(default = "default_render_mode")]
        render_mode: String,
        #[serde(default = "default_vector_style")]
        vector_style: String,
        #[serde(default = "default_lod_mode")]
        lod_mode: String,
        #[serde(default = "default_feature_selection_json")]
        feature_selection_json: String,
        #[serde(default = "default_hover_json")]
        hover_json: String,
        #[serde(default = "default_selection_method")]
        selection_method: String,
        #[serde(default = "default_selection_mode")]
        selection_mode: String,
        #[serde(default)]
        layer_stroke_scale: HashMap<String, f64>,
    }

    fn default_hover_json() -> String {
        "null".into()
    }

    fn default_selection_method() -> String {
        "rectangle".into()
    }

    fn default_selection_mode() -> String {
        "default".into()
    }

    fn default_map_camera_json() -> String {
        r#"{"x":0,"y":0,"zoom":1}"#.into()
    }

    fn default_render_mode() -> String {
        "combined".into()
    }

    fn default_vector_style() -> String {
        "colored".into()
    }

    fn default_lod_mode() -> String {
        GIS_MAP_LOD_MODE_AUTOMATIC.into()
    }

    fn default_feature_selection_json() -> String {
        r#"{"positions":[],"routes":[]}"#.into()
    }

    impl Default for Gis2dPlayRuntime {
        fn default() -> Self {
            Self {
                selected_ids: Vec::new(),
                layer_visibility: HashMap::new(),
                camera_json: default_map_camera_json(),
                render_mode: default_render_mode(),
                vector_style: default_vector_style(),
                lod_mode: default_lod_mode(),
                feature_selection_json: default_feature_selection_json(),
                hover_json: default_hover_json(),
                selection_method: default_selection_method(),
                selection_mode: default_selection_mode(),
                layer_stroke_scale: HashMap::new(),
            }
        }
    }

    //#endregion 🔖Types

    //#region 🔖Terminology
    /// 🗣️ Complete UI label set for the GIS 2D app; one field per label makes every locale combination compile-checked.
    struct Gis2dPlayLabels {
        layer_raster: &'static str,
        layer_water: &'static str,
        layer_land: &'static str,
        layer_roads: &'static str,
        layer_buildings: &'static str,
        layer_borders: &'static str,
        layer_map_labels: &'static str,
        layer_positions: &'static str,
        layer_position_labels: &'static str,
        layer_routes: &'static str,
        layer_regions: &'static str,
        map_view: &'static str,
        render_mode: &'static str,
        render_mode_image: &'static str,
        render_mode_vector: &'static str,
        render_mode_combined: &'static str,
        vector_style: &'static str,
        vector_style_colored: &'static str,
        vector_style_figure_ground: &'static str,
        vector_style_inverted_figure: &'static str,
        lod_mode: &'static str,
        lod_automatic: &'static str,
        selection_method: &'static str,
        selection_method_rectangle: &'static str,
        selection_method_lasso: &'static str,
        layers_group: &'static str,
        layer_weights_group: &'static str,
        weight_suffix: &'static str,
        selected_features: &'static str,
        map_layer: &'static str,
        schema: &'static str,
        layers_visible: &'static str,
        field_id: &'static str,
        field_label: &'static str,
        field_visible: &'static str,
    }

    const GIS2D_LABELS_NATIVE_EN: Gis2dPlayLabels = Gis2dPlayLabels {
        layer_raster: "Raster",
        layer_water: "Water",
        layer_land: "Land",
        layer_roads: "Roads",
        layer_buildings: "Buildings",
        layer_borders: "Borders",
        layer_map_labels: "Labels",
        layer_positions: "Positions",
        layer_position_labels: "Position Labels",
        layer_routes: "Routes",
        layer_regions: "Regions",
        map_view: "Map View",
        render_mode: "Render Mode",
        render_mode_image: "Image",
        render_mode_vector: "Vector",
        render_mode_combined: "Combined",
        vector_style: "Vector Style",
        vector_style_colored: "Colored",
        vector_style_figure_ground: "Figure Ground",
        vector_style_inverted_figure: "Inverted Figure",
        lod_mode: "LOD Mode",
        lod_automatic: "Automatic",
        selection_method: "Selection Method",
        selection_method_rectangle: "Rectangle",
        selection_method_lasso: "Lasso",
        layers_group: "Layers",
        layer_weights_group: "Layer Weights",
        weight_suffix: "weight",
        selected_features: "Selected Features",
        map_layer: "Map Layer",
        schema: "Schema",
        layers_visible: "Layers visible",
        field_id: "Id",
        field_label: "Label",
        field_visible: "Visible",
    };

    const GIS2D_LABELS_NATIVE_DE: Gis2dPlayLabels = Gis2dPlayLabels {
        layer_raster: "Raster",
        layer_water: "Wasser",
        layer_land: "Land",
        layer_roads: "Straßen",
        layer_buildings: "Gebäude",
        layer_borders: "Grenzen",
        layer_map_labels: "Beschriftungen",
        layer_positions: "Positionen",
        layer_position_labels: "Positionsbeschriftungen",
        layer_routes: "Routen",
        layer_regions: "Regionen",
        map_view: "Kartenansicht",
        render_mode: "Darstellungsmodus",
        render_mode_image: "Bild",
        render_mode_vector: "Vektor",
        render_mode_combined: "Kombiniert",
        vector_style: "Vektorstil",
        vector_style_colored: "Farbig",
        vector_style_figure_ground: "Figur-Grund",
        vector_style_inverted_figure: "Invertierte Figur",
        lod_mode: "LOD-Modus",
        lod_automatic: "Automatisch",
        selection_method: "Auswahlmethode",
        selection_method_rectangle: "Rechteck",
        selection_method_lasso: "Lasso",
        layers_group: "Ebenen",
        layer_weights_group: "Ebenengewichte",
        weight_suffix: "Gewicht",
        selected_features: "Ausgewählte Objekte",
        map_layer: "Kartenebene",
        schema: "Schema",
        layers_visible: "Sichtbare Ebenen",
        field_id: "Id",
        field_label: "Bezeichnung",
        field_visible: "Sichtbar",
    };

    /// 🗣️ Resolves the active label set from the shell-provided locale; unknown locales fall back to native English.
    fn gis2d_labels(view_state: &ViewState) -> &'static Gis2dPlayLabels {
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        if is_de { &GIS2D_LABELS_NATIVE_DE } else { &GIS2D_LABELS_NATIVE_EN }
    }

    /// 🗣️ Resolves a standard map layer's display label from its stable id; unknown ids fall back to the catalog's native English text.
    fn gis2d_layer_label(layer_id: &str, labels: &Gis2dPlayLabels) -> &'static str {
        match layer_id {
            "raster" => labels.layer_raster,
            "water" => labels.layer_water,
            "land" => labels.layer_land,
            "roads" => labels.layer_roads,
            "buildings" => labels.layer_buildings,
            "borders" => labels.layer_borders,
            "labels" => labels.layer_map_labels,
            "positions" => labels.layer_positions,
            "positionLabels" => labels.layer_position_labels,
            "routes" => labels.layer_routes,
            "regions" => labels.layer_regions,
            // 🗣️ unreachable in practice — the arms above already cover every id in GIS_MAP_LAYER_IDS.
            _ => "",
        }
    }
    //#endregion 🔖Terminology

    //#region 🔖DocumentHelpers
    fn default_layer_visibility() -> HashMap<String, bool> {
        GIS_MAP_LAYER_IDS.iter().map(|(id, _, _)| ((*id).into(), true)).collect()
    }

    /// 🗺️ The default map document, seeded from the bundled reuse example.
    fn default_document() -> GisMapDocument {
        gis_map_document_from_descriptor_json(REUSE_MAP_EXAMPLE_JSON)
    }

    /// 🎛️ The default runtime — every layer visible, camera framed to the seed document's world extent.
    fn default_runtime() -> Gis2dPlayRuntime {
        let mut runtime = Gis2dPlayRuntime::default();
        runtime.layer_visibility = default_layer_visibility();
        let mut host = map_host_from(&default_document(), &runtime);
        host.fit_world_camera();
        runtime.camera_json = host.camera_json();
        runtime
    }

    /// 🗺️ Builds a `MapHost` from the document content (derived descriptor JSON) plus the runtime's
    /// camera/render/style/LOD/selection view state.
    fn map_host_from(document: &GisMapDocument, runtime: &Gis2dPlayRuntime) -> MapHost {
        let mut host = MapHost::new();
        let descriptor = gis_map_descriptor_json(document);
        let _ = host.sync_map_json(&descriptor);
        if let Ok(camera) = serde_json::from_str::<Value>(&runtime.camera_json) {
            let x = camera.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
            let y = camera.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
            let zoom = camera.get("zoom").and_then(|value| value.as_f64()).unwrap_or(1.0);
            host.set_camera(x, y, zoom);
        }
        host.set_render_mode(&runtime.render_mode);
        host.set_vector_style(&runtime.vector_style);
        host.set_lod_mode(&runtime.lod_mode);
        let _ = host.set_selection_json(&runtime.feature_selection_json);
        host
    }

    fn layer_visibility_json(runtime: &Gis2dPlayRuntime) -> String {
        let mut map = default_layer_visibility();
        for (id, visible) in &runtime.layer_visibility {
            map.insert(id.clone(), *visible);
        }
        serde_json::to_string(&map).unwrap_or_else(|_| "{}".into())
    }

    fn layer_stroke_scale_json(runtime: &Gis2dPlayRuntime) -> String {
        let mut map: HashMap<String, f64> = GIS_MAP_LAYER_IDS
            .iter()
            .map(|(id, _, _)| ((*id).into(), 1.0))
            .collect();
        for (id, weight) in &runtime.layer_stroke_scale {
            map.insert(id.clone(), clamp_map_layer_weight(*weight));
        }
        serde_json::to_string(&map).unwrap_or_else(|_| "{}".into())
    }

    fn merge_feature_selection(
        current_json: &str,
        positions: Vec<String>,
        routes: Vec<String>,
        mode: &str,
    ) -> Value {
        let current: Value = serde_json::from_str(current_json).unwrap_or(json!({"positions":[],"routes":[]}));
        let current_positions: Vec<String> = current
            .get("positions")
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .unwrap_or_default();
        let current_routes: Vec<String> = current
            .get("routes")
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .unwrap_or_default();
        let mut next_positions: HashSet<String> = current_positions.iter().cloned().collect();
        let mut next_routes: HashSet<String> = current_routes.iter().cloned().collect();
        let incoming_positions: HashSet<String> = positions.into_iter().collect();
        let incoming_routes: HashSet<String> = routes.into_iter().collect();
        match mode {
            "additive" => {
                next_positions.extend(incoming_positions);
                next_routes.extend(incoming_routes);
            }
            "subtractive" => {
                next_positions.retain(|id| !incoming_positions.contains(id));
                next_routes.retain(|id| !incoming_routes.contains(id));
            }
            "invertive" => {
                for id in incoming_positions {
                    if !next_positions.insert(id.clone()) {
                        next_positions.remove(&id);
                    }
                }
                for id in incoming_routes {
                    if !next_routes.insert(id.clone()) {
                        next_routes.remove(&id);
                    }
                }
            }
            _ => {
                next_positions = incoming_positions;
                next_routes = incoming_routes;
            }
        }
        json!({
            "positions": next_positions.into_iter().collect::<Vec<_>>(),
            "routes": next_routes.into_iter().collect::<Vec<_>>(),
        })
    }

    fn gis2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor {
            controller_id: GIS2D_PLAY_APP_ID.into(),
            action: action.into(),
            args,
        }
    }

    /// 🌉 Diffs a positions collection before/after an in-place edit into granular `GisMapOp::Positions`
    /// ops (add/remove/patch by id), so whole-array replacements still converge per-feature.
    fn positions_ops(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapOp> {
        let mut ops = Vec::new();
        let after_ids: HashSet<&str> = after.iter().map(|feature| feature.id.as_str()).collect();
        for feature in before {
            if !after_ids.contains(feature.id.as_str()) {
                ops.push(GisMapOp::Positions(CollectionOp::Remove { id: feature.id.clone() }));
            }
        }
        for (index, feature) in after.iter().enumerate() {
            match before.iter().find(|entry| entry.id == feature.id) {
                None => ops.push(GisMapOp::Positions(CollectionOp::Add { index, item: feature.clone() })),
                Some(prev) if prev.data != feature.data => ops.push(GisMapOp::Positions(CollectionOp::Patch {
                    id: feature.id.clone(),
                    patch: MapFeaturePatch { data: Some(feature.data.clone()) },
                })),
                Some(_) => {}
            }
        }
        ops
    }

    fn selection_ids(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("ids"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .unwrap_or_default()
    }

    fn layer_visible(runtime: &Gis2dPlayRuntime, layer_id: &str) -> bool {
        runtime.layer_visibility.get(layer_id).copied().unwrap_or(true)
    }

    fn layer_weight_slider_fields(runtime: &Gis2dPlayRuntime, labels: &Gis2dPlayLabels) -> Vec<UiNode> {
        layer_weight_entries(runtime, labels)
            .into_iter()
            .map(|(layer_id, label, value)| {
                UiNode::Field(UiFieldNode {
                    id: format!("gis2d-play-inspector.weight.{layer_id}"),
                    label: format!("{label} {}", labels.weight_suffix),
                    child: Box::new(UiNode::Slider(UiSliderNode {
                        id: format!("gis2d-play-inspector.weight.{layer_id}.slider"),
                        value,
                        min: 0.25,
                        max: 3.0,
                        step: 0.05,
                        on_change: gis2d_action(
                            "setLayerStrokeScale",
                            Some(json!({ "layerId": layer_id })),
                        ),
                        unit: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                })
            })
            .collect()
    }

    fn lod_select_entries(labels: &Gis2dPlayLabels) -> Vec<(String, String)> {
        std::iter::once((GIS_MAP_LOD_MODE_AUTOMATIC.into(), labels.lod_automatic.into()))
            .chain(
                serde_json::from_str::<Vec<Value>>(&gis_map_lod_scale_json())
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|lod| {
                        let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
                        let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
                        Some((id, name))
                    }),
            )
            .collect()
    }

    fn layer_weight_entries(runtime: &Gis2dPlayRuntime, labels: &Gis2dPlayLabels) -> Vec<(String, String, f64)> {
        let ids: Vec<String> = serde_json::from_str(&gis_map_layer_weight_slider_ids_json(
            &runtime.lod_mode,
            &runtime.render_mode,
        ))
        .unwrap_or_default();
        ids.into_iter()
            .map(|layer_id| {
                let value = runtime
                    .layer_stroke_scale
                    .get(&layer_id)
                    .copied()
                    .map(clamp_map_layer_weight)
                    .unwrap_or(1.0);
                let label = gis2d_layer_label(&layer_id, labels).to_string();
                (layer_id, label, value)
            })
            .collect()
    }

    fn gis2d_window_measures(runtime: &Gis2dPlayRuntime, labels: &Gis2dPlayLabels) -> Vec<WindowMeasure> {
        let layer_toggles: Vec<WindowMeasure> = GIS_MAP_LAYER_IDS
            .iter()
            .map(|(id, _, icon)| WindowMeasure::Toggle {
                id: format!("gis2d-play-window.layer.{id}"),
                icon_id: (*icon).into(),
                label: Some(gis2d_layer_label(id, labels).into()),
                pressed: layer_visible(runtime, id),
                text: None,
                on_change: gis2d_action("toggleLayerVisibility", Some(json!({ "layerId": id }))),
            })
            .collect();
        let layer_weight_sliders: Vec<WindowMeasure> = layer_weight_entries(runtime, labels)
            .into_iter()
            .map(|(layer_id, label, value)| WindowMeasure::Slider {
                id: format!("gis2d-play-window.weight.{layer_id}"),
                label: Some(format!("{label} {}", labels.weight_suffix)),
                value,
                min: 0.25,
                max: 3.0,
                step: Some(0.05),
                on_change: gis2d_action("setLayerStrokeScale", Some(json!({ "layerId": layer_id }))),
            })
            .collect();
        vec![
            WindowMeasure::Select {
                id: "gis2d-play-window.render-mode".into(),
                label: Some(labels.render_mode.into()),
                value: runtime.render_mode.clone(),
                items: vec![
                    MeasureSelectItem { id: "image".into(), value: "image".into(), label: labels.render_mode_image.into() },
                    MeasureSelectItem { id: "vector".into(), value: "vector".into(), label: labels.render_mode_vector.into() },
                    MeasureSelectItem { id: "combined".into(), value: "combined".into(), label: labels.render_mode_combined.into() },
                ],
                on_change: gis2d_action("setRenderMode", None),
            },
            WindowMeasure::Select {
                id: "gis2d-play-window.vector-style".into(),
                label: Some(labels.vector_style.into()),
                value: runtime.vector_style.clone(),
                items: vec![
                    MeasureSelectItem { id: "colored".into(), value: "colored".into(), label: labels.vector_style_colored.into() },
                    MeasureSelectItem { id: "figureGround".into(), value: "figureGround".into(), label: labels.vector_style_figure_ground.into() },
                    MeasureSelectItem { id: "invertedFigure".into(), value: "invertedFigure".into(), label: labels.vector_style_inverted_figure.into() },
                ],
                on_change: gis2d_action("setVectorStyle", None),
            },
            WindowMeasure::Select {
                id: "gis2d-play-window.lod-mode".into(),
                label: Some(labels.lod_mode.into()),
                value: runtime.lod_mode.clone(),
                items: lod_select_entries(labels)
                    .into_iter()
                    .map(|(value, label)| MeasureSelectItem { id: value.clone(), value, label })
                    .collect(),
                on_change: gis2d_action("setLodMode", None),
            },
            WindowMeasure::Select {
                id: "gis2d-play-window.selection-method".into(),
                label: Some(labels.selection_method.into()),
                value: runtime.selection_method.clone(),
                items: vec![
                    MeasureSelectItem { id: "rectangle".into(), value: "rectangle".into(), label: labels.selection_method_rectangle.into() },
                    MeasureSelectItem { id: "lasso".into(), value: "lasso".into(), label: labels.selection_method_lasso.into() },
                ],
                on_change: gis2d_action("setSelectionMethod", None),
            },
            WindowMeasure::Group {
                id: "gis2d-play-window.layers".into(),
                label: labels.layers_group.into(),
                default_open: Some(true),
                active_utility_id: None,
                children: layer_toggles,
            },
            WindowMeasure::Group {
                id: "gis2d-play-window.layer-weights".into(),
                label: labels.layer_weights_group.into(),
                default_open: Some(false),
                children: layer_weight_sliders,
                active_utility_id: None,
            },
        ]
    }
    //#endregion 🔖DocumentHelpers

    //#region 🔖Panels
    fn tree_item(
        id: impl Into<String>,
        label: impl Into<String>,
        description: Option<String>,
        icon_id: Option<String>,
        action: Option<ActionDescriptor>,
    ) -> UiTreeItemNode {
        UiTreeItemNode {
            id: id.into(),
            label: label.into(),
            description,
            icon_id,
            loading: None,
            selected: None,
            default_open: None,
            hover_action: None,
            unhover_action: None,
            actions: None,
            action,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        }
    }

    fn build_document_tree(runtime: &Gis2dPlayRuntime, labels: &Gis2dPlayLabels) -> UiNode {
        let layer_items: Vec<UiTreeItemNode> = GIS_MAP_LAYER_IDS
            .iter()
            .map(|(id, _, icon)| {
                tree_item(
                    format!("gis2d-play-document.layer.{id}"),
                    gis2d_layer_label(id, labels),
                    Some((*id).into()),
                    Some((*icon).into()),
                    Some(gis2d_action("setSelection", Some(json!({ "ids": [id] })))),
                )
            })
            .collect();
        UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode {
                id: "gis2d-play-document.layers".into(),
                label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
                default_open: Some(true),
                loading: None,
                items: layer_items,
            }],
            selected_ids: Some(
                runtime
                    .selected_ids
                    .iter()
                    .map(|id| format!("gis2d-play-document.layer.{id}"))
                    .collect(),
            ),
            highlighted_ids: None,
            selection_change: Some(gis2d_action("setSelection", None)),
            drop_action: None,
            loading: None,
        })
    }

    fn build_catalogue_tree(runtime: &Gis2dPlayRuntime, labels: &Gis2dPlayLabels) -> UiNode {
        let items: Vec<UiTreeItemNode> = GIS_MAP_LAYER_IDS
            .iter()
            .map(|(id, _, icon)| {
                tree_item(
                    format!("gis2d-play-catalogue.layer.{id}"),
                    gis2d_layer_label(id, labels),
                    None,
                    Some((*icon).into()),
                    Some(gis2d_action("toggleLayerVisibility", Some(json!({ "layerId": id })))),
                )
            })
            .collect();
        let _ = runtime;
        UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode {
                id: "gis2d-play-catalogue.layers".into(),
                label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
                default_open: Some(true),
                loading: None,
                items,
            }],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
            loading: None,
        })
    }

    fn map_view_field_group(runtime: &Gis2dPlayRuntime, labels: &Gis2dPlayLabels) -> UiInspectorFieldGroup {
        let lod_items: Vec<UiSelectItem> = lod_select_entries(labels)
            .into_iter()
            .map(|(value, label)| UiSelectItem { value, label })
            .collect();
        let selection: Value = serde_json::from_str(&runtime.feature_selection_json).unwrap_or(json!({"positions":[],"routes":[]}));
        let selected_count = selection.get("positions").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
            + selection.get("routes").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0);
        let mut fields = vec![
                UiNode::Field(UiFieldNode {
                    id: "gis2d-play-inspector.render-mode".into(),
                    label: labels.render_mode.into(),
                    child: Box::new(UiNode::Select(UiSelectNode {
                        id: "gis2d-play-inspector.render-mode.select".into(),
                        value: runtime.render_mode.clone(),
                        items: vec![
                            UiSelectItem { value: "image".into(), label: labels.render_mode_image.into() },
                            UiSelectItem { value: "vector".into(), label: labels.render_mode_vector.into() },
                            UiSelectItem { value: "combined".into(), label: labels.render_mode_combined.into() },
                        ],
                        placeholder: None,
                        on_change: gis2d_action("setRenderMode", None),
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
                UiNode::Field(UiFieldNode {
                    id: "gis2d-play-inspector.vector-style".into(),
                    label: labels.vector_style.into(),
                    child: Box::new(UiNode::Select(UiSelectNode {
                        id: "gis2d-play-inspector.vector-style.select".into(),
                        value: runtime.vector_style.clone(),
                        items: vec![
                            UiSelectItem { value: "colored".into(), label: labels.vector_style_colored.into() },
                            UiSelectItem { value: "figureGround".into(), label: labels.vector_style_figure_ground.into() },
                            UiSelectItem { value: "invertedFigure".into(), label: labels.vector_style_inverted_figure.into() },
                        ],
                        placeholder: None,
                        on_change: gis2d_action("setVectorStyle", None),
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
                UiNode::Field(UiFieldNode {
                    id: "gis2d-play-inspector.lod-mode".into(),
                    label: labels.lod_mode.into(),
                    child: Box::new(UiNode::Select(UiSelectNode {
                        id: "gis2d-play-inspector.lod-mode.select".into(),
                        value: runtime.lod_mode.clone(),
                        items: lod_items,
                        placeholder: None,
                        on_change: gis2d_action("setLodMode", None),
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
                UiNode::Field(UiFieldNode {
                    id: "gis2d-play-inspector.selection-method".into(),
                    label: labels.selection_method.into(),
                    child: Box::new(UiNode::Select(UiSelectNode {
                        id: "gis2d-play-inspector.selection-method.select".into(),
                        value: runtime.selection_method.clone(),
                        items: vec![
                            UiSelectItem { value: "rectangle".into(), label: labels.selection_method_rectangle.into() },
                            UiSelectItem { value: "lasso".into(), label: labels.selection_method_lasso.into() },
                        ],
                        placeholder: None,
                        on_change: gis2d_action("setSelectionMethod", None),
                    })),
                    description: None,
                    required: None,
                    error: None,
                }),
                ui_inspector_readonly_field("gis2d-play-inspector.feature-selection", labels.selected_features, selected_count.to_string()),
        ];
        fields.extend(layer_weight_slider_fields(runtime, labels));
        UiInspectorFieldGroup {
            id: "gis2d-play-inspector.map-view".into(),
            label: labels.map_view.into(),
            default_open: Some(true),
            fields,
        }
    }

    fn build_inspector_tree(runtime: &Gis2dPlayRuntime, labels: &Gis2dPlayLabels) -> UiNode {
        let map_view_group = map_view_field_group(runtime, labels);
        if runtime.selected_ids.is_empty() {
            let visible_count = GIS_MAP_LAYER_IDS
                .iter()
                .filter(|(id, _, _)| layer_visible(runtime, id))
                .count();
            return ui_inspector_groups_to_tree(&[
                map_view_group,
                UiInspectorFieldGroup {
                    id: "gis2d-play-inspector.summary".into(),
                    label: labels.map_layer.into(),
                    default_open: Some(true),
                    fields: vec![
                        ui_inspector_readonly_field("gis2d-play-inspector.schema", labels.schema, GIS_MAP_SCHEMA.to_string()),
                        ui_inspector_readonly_field(
                            "gis2d-play-inspector.visible-count",
                            labels.layers_visible,
                            format!("{visible_count}/{}", GIS_MAP_LAYER_IDS.len()),
                        ),
                    ],
                },
            ]);
        }
        let layer_id = &runtime.selected_ids[0];
        let label = gis2d_layer_label(layer_id, labels);
        let visible = layer_visible(runtime, layer_id);
        let mixed = ui_inspector_mixed_toggle(&[visible]);
        ui_inspector_groups_to_tree(&[
            map_view_group,
            UiInspectorFieldGroup {
                id: "gis2d-play-inspector.layer".into(),
                label: labels.map_layer.into(),
                default_open: Some(true),
                fields: vec![
                    ui_inspector_readonly_field("gis2d-play-inspector.id", labels.field_id, layer_id.clone()),
                    ui_inspector_readonly_field("gis2d-play-inspector.label", labels.field_label, label.to_string()),
                    UiNode::Field(UiFieldNode {
                        id: "gis2d-play-inspector.visible".into(),
                        label: labels.field_visible.into(),
                        child: Box::new(UiNode::Toggle(UiToggleNode {
                            id: "gis2d-play-inspector.visible.toggle".into(),
                            icon_id: "eye".into(),
                            pressed: mixed.uniform && mixed.pressed,
                            text: None,
                            on_change: gis2d_action("toggleLayerVisibility", Some(json!({ "layerId": layer_id }))),
                        })),
                        description: None,
                        required: None,
                        error: None,
                    }),
                ],
            },
        ])
    }
    //#endregion 🔖Panels

    //#region 🔖Render
    fn apply_gis_map_tile_base_url(scene: &mut GisMapScene) {
        let Ok(base) = std::env::var("SEMIO_GIS_MAP_TILE_BASE_URL") else {
            return;
        };
        let base = base.trim_end_matches('/');
        scene.tile_url_template = format!("{base}/osm/{{z}}/{{x}}/{{y}}.png");
        scene.vector_tile_url_template = format!("{base}/vt/{{z}}/{{x}}/{{y}}.pbf");
    }

    fn render_canvas(document: &GisMapDocument, runtime: &Gis2dPlayRuntime) -> UiNode {
        let mut scene = GisMapScene::base(gis_map_descriptor_json(document), runtime.camera_json.clone());
        scene.render_mode = runtime.render_mode.clone();
        scene.vector_style = runtime.vector_style.clone();
        scene.lod_mode = runtime.lod_mode.clone();
        scene.layer_visibility_json = layer_visibility_json(runtime);
        scene.layer_stroke_scale_json = layer_stroke_scale_json(runtime);
        scene.selection_json = runtime.feature_selection_json.clone();
        scene.hover_json = runtime.hover_json.clone();
        scene.selection_method = runtime.selection_method.clone();
        scene.selection_mode = runtime.selection_mode.clone();
        apply_gis_map_tile_base_url(&mut scene);
        build_gis_map_scene(GIS2D_PLAY_SURFACE, GIS2D_PLAY_APP_ID, scene)
    }
    //#endregion 🔖Render

    //#region 🔖Gis2dPlayApp
    /// 🗺️ GIS 2D map play app. The document holds positions/routes/regions; everything else (camera,
    /// render mode, style, LOD, selection, hover, layer visibility, stroke weights) is runtime view state.
    pub struct Gis2dPlayApp {
        runtime: Gis2dPlayRuntime,
    }

    impl Default for Gis2dPlayApp {
        fn default() -> Self {
            Self { runtime: default_runtime() }
        }
    }

    impl DocumentApp for Gis2dPlayApp {
        type Projection = GisMapDocument;
        type Op = GisMapOp;

        fn app_id(&self) -> &str {
            GIS2D_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            GIS_MAP_SCHEMA
        }

        fn initial_projection(&self) -> GisMapDocument {
            default_document()
        }

        fn handle_action(
            &mut self,
            action: &str,
            args: Option<&Value>,
            doc: &DocumentView<'_, GisMapDocument>,
            _view_state: &ViewState,
        ) -> ActionEmit<GisMapOp> {
            let document = doc.projection;
            match action {
                // 👁️ View/config actions — mutate runtime, emit no ops.
                "setSelection" => {
                    self.runtime.selected_ids = selection_ids(args);
                    ActionEmit::default()
                }
                "toggleLayerVisibility" => {
                    if let Some(layer_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) {
                        let visible = !layer_visible(&self.runtime, layer_id);
                        self.runtime.layer_visibility.insert(layer_id.into(), visible);
                    }
                    ActionEmit::default()
                }
                "setCamera" => {
                    let camera = args.and_then(|value| value.get("camera")).or_else(|| args.and_then(|value| value.get("cameraJson")));
                    if let Some(camera) = camera {
                        self.runtime.camera_json = camera.to_string();
                    }
                    ActionEmit::default()
                }
                "fitWorld" => {
                    let mut host = map_host_from(document, &self.runtime);
                    host.fit_world_camera();
                    self.runtime.camera_json = host.camera_json();
                    ActionEmit::default()
                }
                "setRenderMode" => {
                    if let Some(mode) = args.and_then(|value| value.get("mode").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                        self.runtime.render_mode = mode.into();
                    }
                    ActionEmit::default()
                }
                "setVectorStyle" => {
                    if let Some(style) = args.and_then(|value| value.get("style").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                        self.runtime.vector_style = style.into();
                    }
                    ActionEmit::default()
                }
                "setLodMode" => {
                    if let Some(mode) = args.and_then(|value| value.get("mode").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                        self.runtime.lod_mode = mode.into();
                    }
                    ActionEmit::default()
                }
                "setFeatureSelection" => {
                    let positions: Vec<String> = args.and_then(|value| value.get("positions")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                    let routes: Vec<String> = args.and_then(|value| value.get("routes")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                    let mode = args.and_then(|value| value.get("mode")).and_then(|value| value.as_str()).unwrap_or("default");
                    let selection = merge_feature_selection(&self.runtime.feature_selection_json, positions, routes, mode);
                    let mut host = map_host_from(document, &self.runtime);
                    if host.set_selection_json(&selection.to_string()).is_ok() {
                        self.runtime.feature_selection_json = selection.to_string();
                    }
                    ActionEmit::default()
                }
                "setHover" => {
                    let hover = args.and_then(|value| value.get("hover")).cloned().unwrap_or(Value::Null);
                    self.runtime.hover_json = hover.to_string();
                    ActionEmit::default()
                }
                "setSelectionMethod" => {
                    if let Some(method) = args.and_then(|value| value.get("method").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                        self.runtime.selection_method = method.into();
                    }
                    ActionEmit::default()
                }
                "setSelectionMode" => {
                    if let Some(mode) = args.and_then(|value| value.get("mode").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                        self.runtime.selection_mode = mode.into();
                    }
                    ActionEmit::default()
                }
                "clearSelection" => {
                    self.runtime.feature_selection_json = default_feature_selection_json();
                    ActionEmit::default()
                }
                "selectAll" => {
                    let host = map_host_from(document, &self.runtime);
                    let selection = json!({
                        "positions": host.positions.keys().cloned().collect::<Vec<_>>(),
                        "routes": host.routes.keys().cloned().collect::<Vec<_>>(),
                    });
                    self.runtime.feature_selection_json = selection.to_string();
                    ActionEmit::default()
                }
                "deselect" => {
                    let (Some(kind), Some(id)) = (
                        args.and_then(|value| value.get("featureKind")).and_then(|value| value.as_str()),
                        args.and_then(|value| value.get("featureId")).and_then(|value| value.as_str()),
                    ) else {
                        return ActionEmit::default();
                    };
                    let mut selection: Value = serde_json::from_str(&self.runtime.feature_selection_json).unwrap_or(json!({"positions":[],"routes":[]}));
                    let bucket = if kind == "position" { "positions" } else { "routes" };
                    if let Some(rows) = selection.get_mut(bucket).and_then(|value| value.as_array_mut()) {
                        rows.retain(|row| row.as_str() != Some(id));
                    }
                    self.runtime.feature_selection_json = selection.to_string();
                    ActionEmit::default()
                }
                "focusFeature" => {
                    let (Some(kind), Some(id)) = (
                        args.and_then(|value| value.get("featureKind")).and_then(|value| value.as_str()),
                        args.and_then(|value| value.get("featureId")).and_then(|value| value.as_str()),
                    ) else {
                        return ActionEmit::default();
                    };
                    let mut host = map_host_from(document, &self.runtime);
                    if host.focus_feature(kind, id) {
                        self.runtime.camera_json = host.camera_json();
                    }
                    ActionEmit::default()
                }
                "openSource" => {
                    if let Some(feature_id) = args.and_then(|value| value.get("featureId")).and_then(|value| value.as_str()) {
                        let host = map_host_from(document, &self.runtime);
                        if let Some(url) = host.positions.get(feature_id).and_then(|row| row.source_url.as_deref()) {
                            let _ = open_url(url);
                        }
                    }
                    ActionEmit::default()
                }
                "setLayerStrokeScale" => {
                    let layer_id = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str());
                    let value = args
                        .and_then(|value| value.get("value"))
                        .and_then(|value| value.as_f64())
                        .or_else(|| args.and_then(|value| value.get("weight")).and_then(|value| value.as_f64()));
                    if let (Some(layer_id), Some(value)) = (layer_id, value) {
                        self.runtime.layer_stroke_scale.insert(layer_id.into(), clamp_map_layer_weight(value));
                    }
                    ActionEmit::default()
                }
                // ✏️ Operation actions — flow through the document store with true inverses.
                "setActiveExample" => {
                    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                    let next = if example_id.is_empty() || example_id == "empty" { GisMapDocument::default() } else { default_document() };
                    self.runtime.selected_ids.clear();
                    if !example_id.is_empty() && example_id != "empty" {
                        let mut host = map_host_from(&next, &self.runtime);
                        host.fit_world_camera();
                        self.runtime.camera_json = host.camera_json();
                    }
                    ActionEmit::ops(vec![GisMapOp::SetDocument { document: next }])
                }
                "patchPositions" => {
                    let Some(positions) = args.and_then(|value| value.get("positions")) else {
                        return ActionEmit::default();
                    };
                    let next = gis_map_document_from_descriptor_json(&json!({ "positions": positions }).to_string()).positions;
                    let ops = positions_ops(&document.positions, &next);
                    ActionEmit::ops(ops)
                }
                "patchRoutes" | "patchRoute" => {
                    let route_ids: Vec<String> = if action == "patchRoute" {
                        args.and_then(|value| value.get("routeId")).and_then(|value| value.as_str()).map(|id| vec![id.to_string()]).unwrap_or_default()
                    } else {
                        args.and_then(|value| value.get("routeIds")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default()
                    };
                    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str());
                    let value = args.and_then(|value| value.get("value"));
                    let (false, Some(field), Some(value)) = (route_ids.is_empty(), field, value) else {
                        return ActionEmit::default();
                    };
                    let ops: Vec<GisMapOp> = document
                        .routes
                        .iter()
                        .filter(|route| route_ids.iter().any(|id| id == &route.id))
                        .filter_map(|route| {
                            let mut data = route.data.clone();
                            let object = data.as_object_mut()?;
                            object.insert(field.into(), value.clone());
                            Some(GisMapOp::Routes(CollectionOp::Patch { id: route.id.clone(), patch: MapFeaturePatch { data: Some(data) } }))
                        })
                        .collect();
                    ActionEmit::ops(ops)
                }
                _ => ActionEmit::default(),
            }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, GisMapDocument>, view_state: &ViewState) -> UiNode {
            let document = doc.projection;
            let labels = gis2d_labels(view_state);
            match body_key {
                GIS2D_PLAY_BODY_COMPOSITE => render_canvas(document, &self.runtime),
                GIS2D_PLAY_BODY_DOCUMENT => build_document_tree(&self.runtime, labels),
                GIS2D_PLAY_BODY_CATALOGUE => build_catalogue_tree(&self.runtime, labels),
                GIS2D_PLAY_BODY_INSPECTION => build_inspector_tree(&self.runtime, labels),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn window_measures(
            &self,
            _doc: &DocumentView<'_, GisMapDocument>,
            view_state: &ViewState,
        ) -> HashMap<String, Vec<WindowMeasure>> {
            let labels = gis2d_labels(view_state);
            HashMap::from([(GIS2D_PLAY_WINDOW_MAIN.into(), gis2d_window_measures(&self.runtime, labels))])
        }
    }
    //#endregion 🔖Gis2dPlayApp

    //#region 🔖AppFactory
    /// 🔽 The static LOD-mode choices for the palette arg schema: the automatic mode plus each LOD scale
    /// tier from the map descriptor, labelled in the app's base locale (localization is applied by overlay).
    fn lod_arg_options() -> Vec<ActionArgOption> {
        std::iter::once(ActionArgOption::new(GIS_MAP_LOD_MODE_AUTOMATIC, GIS2D_LABELS_NATIVE_EN.lod_automatic))
            .chain(
                serde_json::from_str::<Vec<Value>>(&gis_map_lod_scale_json())
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|lod| {
                        let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
                        let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
                        Some(ActionArgOption::new(id, name))
                    }),
            )
            .collect()
    }

    pub fn create_gis2d_app() -> App {
        App::from_builder(
            App::builder(GIS2D_PLAY_APP_ID, "GIS 2D").document(["semio", "gis", "2d"])
                .icon_id("gis2d")
                .mode("edit", "Edit")
                .default_mode_id("edit")
                .window_kind(GIS2D_PLAY_WINDOW_MAIN, "Map", GIS2D_PLAY_BODY_COMPOSITE, SurfaceKind::GisMap)
                .default_layout(create_default_layout(
                    &[GIS2D_PLAY_WINDOW_MAIN.into()],
                    "row",
                    Some(&[100.0]),
                    Some(&["Map".into()]),
                ))
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                    PanelGroup::Workbench,
                    GIS2D_PLAY_BODY_DOCUMENT,
                )
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                    PanelGroup::Workbench,
                    GIS2D_PLAY_BODY_CATALOGUE,
                )
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                    PanelGroup::Details,
                    GIS2D_PLAY_BODY_INSPECTION,
                )
                // ✏️ Operation actions — flow through the document store with true inverses. `setActiveExample`
                // replaces document content via `SetDocument` ops, so it is an Operation, not a View action.
                .operation("setActiveExample", "Set Active Example")
                .operation("patchPositions", "Patch Positions")
                .operation("patchRoutes", "Patch Routes")
                .operation("patchRoute", "Patch Route")
                // 👁️ View actions — mutate ephemeral runtime view state (selection, camera, render config,
                // hover, layer visibility, stroke weights), never the document.
                .view_action("setSelection", "Set Selection")
                .view_action("toggleLayerVisibility", "Toggle Layer Visibility")
                .view_action("fitWorld", "Fit World")
                .view_action("setCamera", "Set Camera")
                .view_action("setRenderMode", "Set Render Mode")
                .view_action("setVectorStyle", "Set Vector Style")
                .view_action("setLodMode", "Set LOD Mode")
                .view_action("setFeatureSelection", "Set Feature Selection")
                .view_action("setHover", "Set Hover")
                .view_action("setSelectionMethod", "Set Selection Method")
                .view_action("setSelectionMode", "Set Selection Mode")
                .view_action("clearSelection", "Clear Selection")
                .view_action("selectAll", "Select All")
                .view_action("deselect", "Deselect")
                .view_action("focusFeature", "Focus Feature")
                .view_action("setLayerStrokeScale", "Set Layer Stroke Scale")
                // 🌐 Shell action — opens the picked feature's source URL through the host.
                .shell_action("openSource", "Open Source")
                // 📝 Argument schemas for the discrete-choice actions so the command palette can stage them
                // and the registry validates the vocabulary. The arg id matches the key each handler reads.
                .action_args("setActiveExample", vec![
                    ActionArgDef::select("exampleId", "Example", vec![
                        ActionArgOption::new("empty", "Empty"),
                        ActionArgOption::new("reuse-map", "Reuse Map"),
                    ]).default_value("reuse-map"),
                ])
                .action_args("setRenderMode", vec![
                    ActionArgDef::select("value", "Render Mode", vec![
                        ActionArgOption::new("image", "Image"),
                        ActionArgOption::new("vector", "Vector"),
                        ActionArgOption::new("combined", "Combined"),
                    ]).default_value("combined"),
                ])
                .action_args("setVectorStyle", vec![
                    ActionArgDef::select("value", "Vector Style", vec![
                        ActionArgOption::new("colored", "Colored"),
                        ActionArgOption::new("figureGround", "Figure Ground"),
                        ActionArgOption::new("invertedFigure", "Inverted Figure"),
                    ]).default_value("colored"),
                ])
                .action_args("setLodMode", vec![
                    ActionArgDef::select("value", "LOD Mode", lod_arg_options()).default_value(GIS_MAP_LOD_MODE_AUTOMATIC),
                ])
                .action_args("setSelectionMethod", vec![
                    ActionArgDef::select("value", "Selection Method", vec![
                        ActionArgOption::new("rectangle", "Rectangle"),
                        ActionArgOption::new("lasso", "Lasso"),
                    ]).default_value("rectangle"),
                ])
                .keybinding("mod+z", "undo")
                .keybinding("mod+shift+z", "redo"),
        )
        .example("reuse-map", "Reuse Map", serde_json::to_string(&default_document()).unwrap())
        .program("gis2d", "GIS 2D", "map")
    }

    fn gis2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
        semio_framework_os::map_points_svg(value, "GIS 2D")
    }

    /// 🗺️ Imports a DWG drawing into a bare gis map document: entity vertices become position features.
    /// Falls back to the default reuse-map document when the DWG carries no point-like geometry.
    fn gis2d_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
        let points: Vec<[f64; 2]> = drawing
            .entities
            .iter()
            .flat_map(|entity| match &entity.geometry {
                DwgGeometry::Point { at } => vec![[at[0], at[1]]],
                DwgGeometry::Line { start, end } => vec![[start[0], start[1]], [end[0], end[1]]],
                DwgGeometry::LwPolyline { vertices, .. } => vertices.clone(),
                DwgGeometry::Polyline3d { vertices, .. } => vertices.iter().map(|v| [v[0], v[1]]).collect(),
                _ => Vec::new(),
            })
            .collect();
        if points.is_empty() {
            return serde_json::to_value(default_document()).map_err(|error| error.to_string());
        }
        let positions: Vec<MapFeature> = points
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let id = format!("dwg-{index}");
                MapFeature { id: id.clone(), data: json!({ "id": id, "lon": point[0], "lat": point[1] }) }
            })
            .collect();
        serde_json::to_value(GisMapDocument { positions, routes: Vec::new(), regions: Vec::new() }).map_err(|error| error.to_string())
    }

    pub fn register_gis2d_exports() {
        semio_framework_os::register_2d_export_handlers("2d.map", "gis2d", gis2d_document_json_to_svg);
        semio_framework_os::register_dwg_import_handler("2d.map", gis2d_document_json_from_dwg);
    }
    //#endregion 🔖AppFactory

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{ActionKind, ActionMeta, PluginApp, VcsDocumentApp};
        use semio_framework_plugin::app::AppActionRegistry;
        use vcs::MemoryBackbone;

        fn meta(actor: &str) -> ActionMeta {
            ActionMeta { actor: actor.into(), instance_id: 1 }
        }

        fn new_app() -> VcsDocumentApp<Gis2dPlayApp> {
            VcsDocumentApp::new(Gis2dPlayApp::default())
        }

        /// 🧬 A wrapper carrying the real registry so kind discipline (View/Shell-emits-ops rejection) runs.
        fn new_app_with_registry() -> VcsDocumentApp<Gis2dPlayApp> {
            let definition = create_gis2d_app().definition;
            VcsDocumentApp::with_registry(Gis2dPlayApp::default(), AppActionRegistry::from_definition(&definition))
        }

        fn render(app: &mut VcsDocumentApp<Gis2dPlayApp>, body_key: &str, view_state: &ViewState) -> String {
            serde_json::to_string(&app.render(body_key, None, view_state).expect("render")).unwrap()
        }

        #[test]
        fn dwg_import_collects_point_and_line_vertices() {
            let mut drawing = DwgDrawing::default();
            let layer = drawing.ensure_layer("0");
            drawing.entities.push(semio_framework_os::DwgEntity { layer, color: semio_framework_os::DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 0.0] } });
            drawing.entities.push(semio_framework_os::DwgEntity { layer, color: semio_framework_os::DwgColor::ByLayer, geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [3.0, 4.0, 0.0] } });
            let value = gis2d_document_json_from_dwg(&drawing).expect("import dwg");
            let positions = value.get("positions").and_then(|v| v.as_array()).expect("positions array");
            assert_eq!(positions.len(), 3);
        }

        #[test]
        fn dwg_import_falls_back_to_default_document_when_empty() {
            let drawing = DwgDrawing::default();
            let value = gis2d_document_json_from_dwg(&drawing).expect("import empty dwg");
            let document: GisMapDocument = serde_json::from_value(value).expect("document");
            assert!(!document.positions.is_empty(), "fallback seeds the reuse-map document");
        }

        #[test]
        fn renders_gis_map_scene() {
            let mut app = new_app();
            assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE, &ViewState::default()).contains("gis2d-map"));
        }

        #[test]
        fn render_canvas_uses_absolute_tile_urls_when_env_set() {
            unsafe { std::env::set_var("SEMIO_GIS_MAP_TILE_BASE_URL", "http://127.0.0.1:6141") };
            let mut app = new_app();
            let json = render(&mut app, GIS2D_PLAY_BODY_COMPOSITE, &ViewState::default());
            assert!(json.contains("http://127.0.0.1:6141/osm/{z}/{x}/{y}.png"));
            assert!(json.contains("http://127.0.0.1:6141/vt/{z}/{x}/{y}.pbf"));
            unsafe { std::env::remove_var("SEMIO_GIS_MAP_TILE_BASE_URL") };
        }

        #[test]
        fn document_lists_map_layers() {
            let mut app = new_app();
            assert!(render(&mut app, GIS2D_PLAY_BODY_DOCUMENT, &ViewState::default()).contains("gis2d-play-document.layer.raster"));
        }

        #[test]
        fn catalogue_lists_layer_toggles() {
            let mut app = new_app();
            assert!(render(&mut app, GIS2D_PLAY_BODY_CATALOGUE, &ViewState::default()).contains("gis2d-play-catalogue.layer.water"));
        }

        #[test]
        fn gis2d_labels_resolve_native_by_default() {
            let mut app = new_app();
            let json = render(&mut app, GIS2D_PLAY_BODY_INSPECTION, &ViewState::default());
            assert!(json.contains("\"Map View\""));
            assert!(json.contains("\"Render Mode\""));
            assert!(json.contains("\"Selected Features\""));
            assert!(json.contains("\"Map Layer\""));
            assert!(!json.contains("Kartenansicht"));
        }

        #[test]
        fn gis2d_labels_translate_inspector_and_layers_in_german() {
            let mut app = new_app();
            let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
            let inspector_json = render(&mut app, GIS2D_PLAY_BODY_INSPECTION, &view_state);
            assert!(inspector_json.contains("Kartenansicht"));
            assert!(inspector_json.contains("Darstellungsmodus"));
            assert!(inspector_json.contains("Ausgewählte Objekte"));
            assert!(inspector_json.contains("Kartenebene"));
            assert!(!inspector_json.contains("\"Map View\""));

            let document_json = render(&mut app, GIS2D_PLAY_BODY_DOCUMENT, &view_state);
            assert!(document_json.contains("Wasser"));
            assert!(!document_json.contains("\"Water\""));

            let window = app.window_measures(&view_state);
            let window_json = serde_json::to_string(window.get(GIS2D_PLAY_WINDOW_MAIN).unwrap()).unwrap();
            assert!(window_json.contains("Ebenen"));
            assert!(window_json.contains("Ebenengewichte"));
        }

        #[test]
        fn set_selection_is_view_state_and_emits_no_ops() {
            let mut app = new_app();
            let result = app.handle_action("setSelection", Some(&json!({ "ids": ["roads"] })), &ViewState::default(), &meta("local")).expect("setSelection");
            assert!(result.operations.is_empty(), "selection must not produce document ops");
        }

        #[test]
        fn set_render_mode_is_view_state() {
            let mut app = new_app();
            let result = app.handle_action("setRenderMode", Some(&json!({ "mode": "vector" })), &ViewState::default(), &meta("local")).expect("setRenderMode");
            assert!(result.operations.is_empty());
            assert!(render(&mut app, GIS2D_PLAY_BODY_COMPOSITE, &ViewState::default()).contains("\"renderMode\":\"vector\""));
        }

        #[test]
        fn set_active_example_empty_then_reuse_round_trips_document() {
            let mut app = new_app();
            assert!(!app.projection().expect("projection").positions.is_empty());
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": "empty" })), &ViewState::default(), &meta("local")).expect("empty");
            assert!(app.projection().expect("projection").positions.is_empty());
            app.handle_action("setActiveExample", Some(&json!({ "exampleId": "reuse-map" })), &ViewState::default(), &meta("local")).expect("reuse");
            assert!(!app.projection().expect("projection").positions.is_empty());
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            assert!(app.projection().expect("projection").positions.is_empty(), "undo returns to the empty document");
        }

        /// 🧬 `setActiveExample` replaces document content with `SetDocument` ops, so it MUST be declared as
        /// an Operation. Under the real registry the View/Shell → emits-ops guard rejects a mis-declaration;
        /// this proves the corrected declaration lets the document-replacing edit flow through without erroring.
        #[test]
        fn set_active_example_is_operation_under_registry_kind_discipline() {
            let definition = create_gis2d_app().definition;
            let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
            assert!(matches!(action.kind, ActionKind::Operation), "loading an example emits SetDocument ops, so it is an Operation");
            assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");

            let mut app = new_app_with_registry();
            let result = app
                .handle_action("setActiveExample", Some(&json!({ "exampleId": "empty" })), &ViewState::default(), &meta("local"))
                .expect("operation emits ops without tripping the kind-discipline guard");
            assert_eq!(result.operations.len(), 1, "loading an example is one document-replacing edit");
            assert!(app.projection().expect("projection").positions.is_empty(), "the empty example clears every position feature");
        }

        /// 👁️ A representative View action mutates only runtime view state, so under the real registry it
        /// emits no ops and never trips the View → emits-ops guard.
        #[test]
        fn view_actions_emit_no_ops_under_registry_kind_discipline() {
            let mut app = new_app_with_registry();
            let render_mode = app.handle_action("setRenderMode", Some(&json!({ "value": "vector" })), &ViewState::default(), &meta("local")).expect("setRenderMode");
            assert!(render_mode.operations.is_empty(), "render mode is ephemeral view state");
            let fit = app.handle_action("fitWorld", None, &ViewState::default(), &meta("local")).expect("fitWorld");
            assert!(fit.operations.is_empty(), "framing the world only moves the runtime camera");
        }

        #[test]
        fn patch_routes_emits_route_patch_ops_and_updates_document() {
            let mut app = new_app();
            let route_id = "bg_holz_fassade_botanique:bw_institut_botanique_ulg:0";
            let result = app
                .handle_action("patchRoute", Some(&json!({ "routeId": route_id, "field": "label", "value": "Renamed Route" })), &ViewState::default(), &meta("local"))
                .expect("patchRoute");
            assert_eq!(result.operations.len(), 1, "one matching route → one patch op");
            let document = app.projection().expect("projection");
            let route = document.routes.iter().find(|route| route.id == route_id).expect("route");
            assert_eq!(route.data.get("label").and_then(|value| value.as_str()), Some("Renamed Route"));
        }

        /// 🤝 Definitional merge proof: two instances on one backbone patch DIFFERENT routes; after
        /// exchanging ops both converge and keep both edits — impossible under whole-map LWW snapshots.
        #[test]
        fn two_instances_converge_on_disjoint_route_edits() {
            let mut instance_a = new_app();
            let mut instance_b = new_app();
            let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://gis2d-convergence", "mem://gis2d-convergence");
            instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
            instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

            let routes: Vec<String> = instance_a.projection().expect("projection").routes.iter().map(|route| route.id.clone()).collect();
            let (route_a, route_b) = (routes[0].clone(), routes[1].clone());

            instance_a.handle_action("patchRoute", Some(&json!({ "routeId": route_a, "field": "label", "value": "A" })), &ViewState::default(), &meta("actor-a")).expect("a patch");
            instance_b.handle_action("patchRoute", Some(&json!({ "routeId": route_b, "field": "label", "value": "B" })), &ViewState::default(), &meta("actor-b")).expect("b patch");

            instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
            instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

            let projection_a = instance_a.projection().expect("projection a");
            let label = |document: &GisMapDocument, id: &str| document.routes.iter().find(|route| route.id == id).and_then(|route| route.data.get("label").and_then(|value| value.as_str().map(str::to_string)));
            assert_eq!(label(&projection_a, &route_a).as_deref(), Some("A"), "A keeps its own edit");
            assert_eq!(label(&projection_a, &route_b).as_deref(), Some("B"), "A absorbs B's disjoint edit");
        }
    }
    //#endregion 🧪Tests
}

pub mod app_3d {
    //! ⛰️ GIS 3D plugin — terrain viewer app bundled as a hot-swappable WASM component. Reuses the
    //! existing `World3d` viewport/renderer rather than a bespoke one (see `gis/3d/rs` for why);
    //! deliberately read-mostly for this first pass — the only editable/undoable property is
    //! vertical exaggeration.

    use gis_3d::{
        build_terrain_scene_json, projection, Gis3dTerrainDocument, Gis3dTerrainOp,
        TerrainDescriptorJson, TerrainProjectOrigin, GIS_3D_TERRAIN_SCHEMA,
    };
    use semio_framework_plugin::{
        build_world_3d_scene, create_default_layout, ui_text, world3d_default_camera, world3d_scene_extended, world3d_selection_json,
        ActionEmit, App, DocumentApp, DocumentView, SurfaceKind, UiNode, ViewState, WindowMeasure,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::HashMap;

    //#region 🔖Constants
    const GIS3D_PLAY_APP_ID: &str = "gis3d-play";
    const GIS3D_PLAY_SURFACE: &str = "gis3d.play.composite";
    const GIS3D_PLAY_BODY_COMPOSITE: &str = "gis3d.play.composite";
    const GIS3D_PLAY_WINDOW_MAIN: &str = "gis3d-main";

    const REUSE_TERRAIN_EXAMPLE_JSON: &str = include_str!("../../3d/example/reuse.terrain.gis.json");
    //#endregion 🔖Constants

    //#region 🔖Types
    /// 🎛️ Ephemeral view state — the read-only terrain fixture, the camera, and the current selection —
    /// lives in the app struct; only the vertical exaggeration is document (undoable) state.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Gis3dPlayRuntime {
        #[serde(default)]
        terrain_fixture_json: String,
        #[serde(default = "world3d_default_camera")]
        camera_json: String,
        #[serde(default)]
        selected_ids: Vec<String>,
    }

    impl Default for Gis3dPlayRuntime {
        fn default() -> Self {
            Self {
                terrain_fixture_json: REUSE_TERRAIN_EXAMPLE_JSON.into(),
                camera_json: initial_camera_json(),
                selected_ids: Vec::new(),
            }
        }
    }
    //#endregion 🔖Types

    //#region 🔖Document
    fn empty_terrain_descriptor() -> TerrainDescriptorJson {
        TerrainDescriptorJson {
            schema: GIS_3D_TERRAIN_SCHEMA.into(),
            project_origin: TerrainProjectOrigin { lon: 0.0, lat: 0.0 },
            positions: Vec::new(),
            exaggeration: 1.0,
        }
    }

    fn parse_descriptor(runtime: &Gis3dPlayRuntime) -> TerrainDescriptorJson {
        serde_json::from_str(&runtime.terrain_fixture_json).unwrap_or_else(|_| empty_terrain_descriptor())
    }

    /// 🎥 A default overview camera scaled for a real-world DEM tile patch (hundreds of meters to a
    /// few kilometers wide) — the generic `world3d_default_camera()` (position `[4,-4,3]`) assumes
    /// an object-scale scene and would sit inside the ground here.
    fn initial_camera_json() -> String {
        json!({ "position": [800.0, -800.0, 600.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string()
    }
    //#endregion 🔖Document

    //#region 🔖Render
    /// 📍 GIS pins are emitted as plain `World3d` instances with no matching `meshesJson` entry —
    /// `WorldInstancesLayer`'s existing missing-mesh fallback renders a small colored box, so
    /// selection/hover/context-menu all work for free without any new scene-schema surface.
    fn instances_json(descriptor: &TerrainDescriptorJson) -> String {
        let instances: Vec<Value> = descriptor
            .positions
            .iter()
            .map(|position| {
                let (x, y) = projection::lonlat_to_local_meters(position.lon, position.lat, descriptor.project_origin.lon, descriptor.project_origin.lat);
                json!({
                    "id": position.id,
                    "meshId": "pin",
                    "position": [x, y, 50.0],
                    "color": "#ff3355",
                    "label": position.label,
                })
            })
            .collect();
        serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
    }

    fn render_canvas(document: &Gis3dTerrainDocument, runtime: &Gis3dPlayRuntime) -> UiNode {
        let mut descriptor = parse_descriptor(runtime);
        descriptor.exaggeration = document.exaggeration;
        let mut scene = world3d_scene_extended(
            runtime.camera_json.clone(),
            "[]".into(),
            instances_json(&descriptor),
            world3d_selection_json("rectangle", &runtime.selected_ids, None),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        scene.terrain_json = Some(build_terrain_scene_json(&descriptor));
        build_world_3d_scene(GIS3D_PLAY_SURFACE, GIS3D_PLAY_APP_ID, scene)
    }
    //#endregion 🔖Render

    //#region 🔖Gis3dPlayApp
    #[derive(Default)]
    pub struct Gis3dPlayApp {
        runtime: Gis3dPlayRuntime,
    }

    impl DocumentApp for Gis3dPlayApp {
        type Projection = Gis3dTerrainDocument;
        type Op = Gis3dTerrainOp;

        fn app_id(&self) -> &str {
            GIS3D_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            GIS_3D_TERRAIN_SCHEMA
        }

        fn initial_projection(&self) -> Gis3dTerrainDocument {
            Gis3dTerrainDocument { exaggeration: parse_descriptor(&Gis3dPlayRuntime::default()).exaggeration }
        }

        fn handle_action(
            &mut self,
            action: &str,
            args: Option<&Value>,
            _doc: &DocumentView<'_, Gis3dTerrainDocument>,
            _view_state: &ViewState,
        ) -> ActionEmit<Gis3dTerrainOp> {
            match action {
                "setCamera" => {
                    let camera = args.and_then(|value| value.get("camera")).or_else(|| args.and_then(|value| value.get("cameraJson")));
                    if let Some(camera) = camera {
                        self.runtime.camera_json = camera.to_string();
                    }
                    ActionEmit::default()
                }
                "setSelection" | "worldSelect" => {
                    if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                        self.runtime.selected_ids = ids;
                    }
                    ActionEmit::default()
                }
                "setExaggeration" => {
                    if let Some(exaggeration) = args.and_then(|value| value.get("exaggeration")).and_then(|value| value.as_f64()) {
                        return ActionEmit::amend(vec![Gis3dTerrainOp::SetExaggeration { exaggeration }], "gis3d-exaggeration");
                    }
                    ActionEmit::default()
                }
                _ => ActionEmit::default(),
            }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, Gis3dTerrainDocument>, _view_state: &ViewState) -> UiNode {
            match body_key {
                GIS3D_PLAY_BODY_COMPOSITE => render_canvas(doc.projection, &self.runtime),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }
    }
    //#endregion 🔖Gis3dPlayApp

    //#region 🔖AppFactory
    pub fn create_gis3d_app() -> App {
        App::from_builder(
            App::builder(GIS3D_PLAY_APP_ID, "GIS 3D")
                .document(["semio", "gis", "3d"])
                .icon_id("gis3d")
                .mode("view", "View")
                .default_mode_id("view")
                .window_kind(GIS3D_PLAY_WINDOW_MAIN, "Terrain", GIS3D_PLAY_BODY_COMPOSITE, SurfaceKind::World3d)
                .default_layout(create_default_layout(&[GIS3D_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Terrain".into()])))
                .view_action("setCamera", "Set Camera")
                .view_action("setSelection", "Set Selection")
                .view_action("worldSelect", "Select")
                .operation("setExaggeration", "Set Exaggeration")
                .keybinding("mod+z", "undo")
                .keybinding("mod+shift+z", "redo"),
        )
        .example(
            "reuse-terrain",
            "Reuse Terrain",
            serde_json::to_string(&Gis3dTerrainDocument { exaggeration: parse_descriptor(&Gis3dPlayRuntime::default()).exaggeration }).unwrap(),
        )
        .program("gis3d", "GIS 3D", "terrain")
    }
    //#endregion 🔖AppFactory

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};

        fn meta(actor: &str) -> ActionMeta {
            ActionMeta { actor: actor.into(), instance_id: 1 }
        }

        fn new_app() -> VcsDocumentApp<Gis3dPlayApp> {
            VcsDocumentApp::new(Gis3dPlayApp::default())
        }

        #[test]
        fn seeds_exaggeration_from_the_terrain_fixture() {
            let app = new_app();
            assert_eq!(app.projection().expect("projection").exaggeration, 1.5);
        }

        #[test]
        fn camera_and_selection_are_view_state_and_emit_no_ops() {
            let mut app = new_app();
            let camera = app
                .handle_action("setCamera", Some(&json!({ "camera": { "position": [1.0, 1.0, 1.0] } })), &ViewState::default(), &meta("local"))
                .expect("setCamera");
            assert!(camera.operations.is_empty(), "camera is ephemeral view state");
            let selection = app
                .handle_action("worldSelect", Some(&json!({ "ids": ["p_institut_de_botanique_ulg_liege"] })), &ViewState::default(), &meta("local"))
                .expect("worldSelect");
            assert!(selection.operations.is_empty(), "selection is ephemeral view state");
        }

        /// 🧪 A slider drag is many `setExaggeration` ticks sharing one coalesce key: they fold into ONE
        /// undoable edit, so a single undo restores the fixture's exaggeration rather than a mid-drag value.
        #[test]
        fn exaggeration_drag_coalesces_into_one_undo_step() {
            let mut app = new_app();
            for value in [2.0, 2.5, 3.0] {
                app.handle_action("setExaggeration", Some(&json!({ "exaggeration": value })), &ViewState::default(), &meta("local")).expect("drag tick");
            }
            assert_eq!(app.projection().expect("projection").exaggeration, 3.0);
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            assert_eq!(app.projection().expect("projection").exaggeration, 1.5, "one coalesced edit: undo restores the fixture exaggeration");
        }
    }
    //#endregion 🧪Tests
}

//#region 🔖Bundle
semio_framework_plugin::semio_plugin! {
    id: "gis",
    label: "GIS",
    version: "0.1.0",
    setup: app_2d::register_gis2d_exports,
    apps: [
        app_2d::create_gis2d_app => app_2d::Gis2dPlayApp,
        app_3d::create_gis3d_app => app_3d::Gis3dPlayApp,
    ],
}
//#endregion 🔖Bundle
