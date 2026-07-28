    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};
        use store::MemoryBackbone;

        fn meta(actor: &str) -> ActionMeta {
            ActionMeta { actor: actor.into(), instance_id: 1 }
        }

        fn new_app() -> VcsDocumentApp<Gis2dPlayApp> {
            VcsDocumentApp::new(Gis2dPlayApp::default())
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
