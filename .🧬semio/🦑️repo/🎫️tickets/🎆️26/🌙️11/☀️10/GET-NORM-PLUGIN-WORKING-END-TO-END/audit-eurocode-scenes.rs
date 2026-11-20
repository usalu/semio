use pack::snapshot::ArtifactDsl;
use semio_s_plugin_norm_app_surface_panels::{toolbar::{lowered_viewport, ViewportView}, viewport3d::{config::Lang, window::world3d_scene_of}};
use semio_s_plugin_norm_app_surface_visual_chrome::diagrams2d::{diagram_scene, LoweringOptions};
#[cfg(not(feature = "structural-only"))]
use semio_s_plugin_norm_structural_scene::projection::ViewOptions;
use semio_s_plugin_norm_structural_scene::StructuralScene;
use semio_ui_contract::{UiFixedBytes, UI_FIXED_BYTES};
use semio_ui_locale::Locale;
use semio_ui_scene::SceneDoc;

#[cfg(any(feature = "window-admission", feature = "palette-window"))]
#[path = "eurocode-ui-arena.rs"]
mod ui_arena;

#[cfg(feature = "retained-json")]
fn portable(node: &semio_ui_contract::BuiltNode) -> serde_json::Value {
    serde_json::json!({
        "key": node.key,
        "component": node.component,
        "layout": node.layout,
        "bindings": node.bindings,
        "style": node.style,
        "accessibility": node.accessibility,
        "children": node.children.iter().map(portable).collect::<Vec<_>>(),
    })
}

#[cfg(feature = "retained-json")]
fn publish<S: SceneDoc>(id: &str, kind: semio_ui_contract::SurfaceKind, scene: &S) {
    let root = semio_plugin::scene_surface(id, kind, scene).expect("retained surface admission");
    println!("[DEBUG] retained {}", portable(&root));
}

fn transmitted<S: SceneDoc>(scene: &S) -> (usize, S) {
    let (spine, lanes) = scene.split_lanes();
    let bytes = spine.encode_pack().expect("valid fixed scene");
    assert!(UiFixedBytes::try_from_vec(bytes.clone()).is_ok());
    let mut restored = S::decode_pack(&bytes).expect("valid restored scene spine");
    for lane in lanes {
        assert!(restored.merge_lane(lane.key, lane.payload));
    }
    (bytes.len(), restored)
}

fn world(id: &str, scene: StructuralScene) {
    #[cfg(feature = "palette-window")]
    if id == "en1991" {
        palette(&scene);
    }
    let view = ViewportView::default().resolved(&scene);
    let lowered = lowered_viewport(&scene, &view, &[], Lang::En);
    assert!(!lowered.meshes.is_empty() && !lowered.instances.is_empty());
    let world = world3d_scene_of(&lowered, id, view.config.sun);
    let (fixed, restored) = transmitted(&world);
    assert_eq!(world.meshes_json, restored.meshes_json);
    assert_eq!(world.instances_json, restored.instances_json);
    assert_eq!(world.camera_json, restored.camera_json);
    println!("[DEBUG] {id} preserved {} meshes and {} instances with a {fixed}-byte fixed scene", lowered.meshes.len(), lowered.instances.len());
    #[cfg(feature = "retained-json")]
    publish(&format!("{id}.model"), semio_ui_contract::SurfaceKind::World3d, &world);
}

#[cfg(feature = "viewport-census")]
fn viewport_census(paths: impl Iterator<Item = String>) {
    use semio_s_plugin_norm_app_surface_panels::{lane_overlays::{LaneOverlays, PaletteEntry}, toolbar::render_viewport_panel};
    use semio_ui_contract::BuiltNode;
    fn count(node: &BuiltNode) -> usize {
        1 + node.children.iter().map(count).sum::<usize>()
    }
    let paint = |id: &str, path: &str, scene: StructuralScene, report, palette, controller: &'static str| {
        let body = format!("norm.{id}.play.viewport");
        let mut host = semio_plugin::ViewModel::new(Locale::En, semio_ui_locale::Terminology::Native);
        if id == "en1993" && std::env::var("SEMIO_VIEWPORT_BROWSER_FIXTURE").is_ok() {
            host.tree_windows = vec![semio_plugin::TreeWindowRequest { body_key: body.clone(), node_key: "norm-viewport-insert".into(), open: Some(true), offset: 0, rows: 12 }];
        }
        let view = ViewportView { lanes: LaneOverlays { palette, ..LaneOverlays::default() }, tree_context: Some((&host, &body)), ..ViewportView::default() };
        let paint = |view: &ViewportView<'_>| render_viewport_panel(&body, &scene, view, &[], &semio_s_plugin_norm_app_surface_visual_chrome::interaction_domain_id(id), controller, Locale::En).expect("authored viewport panel");
        if std::env::var("SEMIO_VIEWPORT_DIAGNOSE_GEOMETRY").is_ok() {
            use semio_s_plugin_norm_app_surface_panels::toolbar::render_toolbar;
            use semio_s_plugin_norm_app_surface_panels::viewport3d::window::render_viewport_page;
            let lowered = lowered_viewport(&scene, &view, &[], Lang::En);
            let mut meshes = lowered.meshes.iter().map(|m| {
                let data = format!("{m}");
                (m.get("id").and_then(|v| v.as_str()).unwrap(), data.len())
            }).collect::<Vec<_>>();
            meshes.sort_by_key(|mesh| std::cmp::Reverse(mesh.1));
            let (toolbar, toolbar_nodes) = render_toolbar(&scene, &view, Locale::En, controller).unwrap();
            let fixed = toolbar_nodes + 1 + 4 + 2 + 16;
            println!("[DEBUG] {id} chrome={fixed} toolbar={} toolbar reserve={toolbar_nodes} top meshes={:?}", count(&toolbar.unwrap()), &meshes[..5.min(meshes.len())]);
            for chrome in [0, 16, 32, 48, fixed] {
                let page = render_viewport_page(&body, &lowered, &semio_s_plugin_norm_app_surface_visual_chrome::interaction_domain_id(id), false, 0, chrome);
                println!("[DEBUG] {id} chrome={chrome} page {:?}", page.as_ref().map(|(node, _, total)| (count(node), *total)));
            }
            return;
        }
        let root = paint(&view);
        let children = root.children.iter().map(|node| (node.key.as_str(), count(node))).collect::<Vec<_>>();
        let scene_lanes = root.children.iter().last().expect("scene surface").children.iter().map(|node| (node.key.as_str(), count(node))).collect::<Vec<_>>();
        println!("[DEBUG] {id} {path} nodes={} children={children:?} scene_lanes={scene_lanes:?}", count(&root));
        #[cfg(feature = "paged-viewport")]
        {
            use semio_s_plugin_norm_app_surface_panels::{view_event, NormTransient};
            use semio_ui_contract::{Component, UI_DOCUMENT_NODES};
            let export = std::env::var("SEMIO_VIEWPORT_BROWSER_FIXTURE").is_ok();
            let toolbar = root.children.iter().find(|node| node.key.as_str() == "norm-viewport-toolbar").expect("toolbar");
            for section in toolbar.children.iter() {
                let Component::TreeSection(props) = &section.component else { panic!("toolbar section") };
                if let Some(window) = &props.window {
                    assert!(window.total as usize > semio_ui_contract::UI_FIXED_LIST_ITEMS);
                    let mut host = host.clone();
                    host.tree_windows = vec![semio_plugin::TreeWindowRequest { body_key: body.clone(), node_key: section.key.as_str().into(), open: Some(true), offset: window.total - 1, rows: 1 }];
                    let tail = paint(&ViewportView { tree_context: Some((&host, &body)), ..view.clone() });
                    assert!(count(&tail) <= UI_DOCUMENT_NODES);
                    let tail_toolbar = tail.children.iter().find(|node| node.key.as_str() == "norm-viewport-toolbar").unwrap();
                    let tail_section = tail_toolbar.children.iter().find(|node| node.key == section.key).unwrap();
                    let Component::TreeSection(tail_props) = &tail_section.component else { panic!("toolbar section") };
                    assert_eq!(tail_props.window.as_ref().unwrap().offset, window.total - 1);
                    assert_eq!(tail_section.children.len(), 1);
                    println!("[DEBUG] {id} {} reached final toolbar row {}/{}", section.key.as_str(), window.total, window.total);
                }
            }
            assert!(count(&root) <= UI_DOCUMENT_NODES);
            let help = root.children.iter().find(|node| node.key.as_str() == "norm-viewport-help").unwrap();
            let section = help.children.iter().next().unwrap();
            assert!(section.children.is_empty());
            let Component::TreeSection(help_props) = &section.component else { panic!("help section") };
            let help_total = help_props.window.as_ref().unwrap().total;
            assert!(help_total > 16);
            if export {
                println!("[DEBUG] viewport-initial {}", portable(&root));
            }
            if let Some(pager) = root.children.iter().find(|node| node.key.as_str() == "norm-viewport-scene-page") {
                let section = pager.children.iter().next().unwrap();
                let next = section.children.iter().find(|node| node.key.as_str().ends_with(".next")).expect("first page can advance");
                let component = serde_json::to_value(&next.component).expect("page row JSON");
                assert_eq!(component["target"]["activation"], "viewportAct");
                let args = &component["target"]["args"];
                let event = view_event(args["action"].as_str().unwrap(), None, args["number"].as_f64()).expect("authored page action");
                let lane = NormTransient::default().apply(event).view;
                assert_eq!(lane.scene_page, 1);
                let next_panel = paint(&ViewportView { lane: lane.clone(), ..view.clone() });
                assert!(count(&next_panel) <= UI_DOCUMENT_NODES);
                if export {
                    println!("[DEBUG] viewport-next {}", portable(&next_panel));
                    if let Some(insert) = host.tree_windows.first() {
                        let total = view.lanes.palette.len() as u32 + 1;
                        let mut insert_tail = host.clone();
                        insert_tail.tree_windows[0] = semio_plugin::TreeWindowRequest { offset: total - 1, rows: 1, ..insert.clone() };
                        let next_insert = paint(&ViewportView { lane: lane.clone(), tree_context: Some((&insert_tail, &body)), ..view.clone() });
                        assert!(count(&next_insert) <= UI_DOCUMENT_NODES);
                        assert_eq!(portable(next_insert.children.iter().last().unwrap()), portable(next_panel.children.iter().last().unwrap()));
                        println!("[DEBUG] viewport-next-insert-tail {}", portable(&next_insert));
                    }
                    let mut editor_host = host.clone();
                    editor_host.tree_windows = vec![semio_plugin::TreeWindowRequest { body_key: body.clone(), node_key: "norm-viewport-help.list".into(), open: Some(true), offset: 0, rows: help_total }];
                    let next_help = paint(&ViewportView { lane, tree_context: Some((&editor_host, &body)), ..view.clone() });
                    assert!(count(&next_help) <= UI_DOCUMENT_NODES);
                    println!("[DEBUG] viewport-next-help {}", portable(&next_help));
                    #[cfg(feature = "viewer-transfer")]
                    {
                        let viewer_body = format!("norm.{id}.view.viewport");
                        let controller = controller.strip_suffix("#editor").unwrap().to_owned() + "#viewer";
                        let controller = Box::leak(controller.into_boxed_str());
                        let domain = Box::leak(semio_s_plugin_norm_app_surface_visual_chrome::interaction_domain_id(id).into_boxed_str());
                        let report: semio_s_plugin_norm_app_surface::NormViewReport = report;
                        let viewer = semio_s_plugin_norm_app_surface::NormViewerFrame::new(report.clone(), Default::default(), &[]);
                        let initial = viewer.render_viewport(&viewer_body, &scene, domain, controller, &editor_host).expect("restored viewer scene");
                        assert!(count(&initial) <= UI_DOCUMENT_NODES);
                        let help = initial.children.iter().find(|node| node.key.as_str() == "norm-viewport-help").unwrap();
                        assert!(help.children.iter().next().unwrap().children.is_empty());
                        let viewport = initial.children.iter().last().unwrap();
                        assert!(viewport.bindings.iter().all(|binding| !matches!(binding.trigger, semio_ui_contract::Trigger::Delta | semio_ui_contract::Trigger::Commit | semio_ui_contract::Trigger::Abort | semio_ui_contract::Trigger::Drop)));
                        println!("[DEBUG] viewport-viewer {}", portable(&initial));
                        let event = view_event("viewportSetScenePage", None, Some(1.0)).unwrap();
                        let lane = NormTransient::default().apply(event).view;
                        let viewer = semio_s_plugin_norm_app_surface::NormViewerFrame::new(report.clone(), lane, &[]);
                        let next = viewer.render_viewport(&viewer_body, &scene, domain, controller, &editor_host).expect("viewer geometry page");
                        assert!(count(&next) <= UI_DOCUMENT_NODES);
                        println!("[DEBUG] viewport-viewer-next {}", portable(&next));
                    }
                }
                drop(next_panel);
                ui_arena::drain();
                let mut last_view = view.clone();
                last_view.lane.scene_page = u32::MAX;
                let last = paint(&last_view);
                assert!(count(&last) <= UI_DOCUMENT_NODES);
                let last_pager = last.children.iter().find(|node| node.key.as_str() == "norm-viewport-scene-page").expect("clamped pager");
                let row = last_pager.children.iter().next().unwrap().children.iter().next().unwrap();
                assert!(row.key.as_str().ends_with(".previous"));
                assert_eq!(last_pager.children.iter().next().unwrap().children.len(), 1);
                println!("[DEBUG] {id} page action advanced to 2 and clamped to last; nodes={}", count(&last));
            }
            if let Some(track) = scene.tracks.first() {
                let lane = NormTransient::default().apply(view_event("viewportSelectTrack", Some(&track.id), None).unwrap()).view;
                let active = paint(&ViewportView { lane: lane.clone(), ..view.clone() });
                assert!(count(&active) <= UI_DOCUMENT_NODES);
                let toolbar = active.children.iter().find(|node| node.key.as_str() == "norm-viewport-toolbar").unwrap();
                let playback = toolbar.children.iter().find(|node| node.key.as_str() == "norm-viewport-playback").unwrap();
                let Component::TreeSection(props) = &playback.component else { panic!("playback section") };
                if let Some(window) = &props.window {
                    let mut host = host.clone();
                    host.tree_windows = vec![semio_plugin::TreeWindowRequest { body_key: body.clone(), node_key: playback.key.as_str().into(), open: Some(true), offset: window.total - 1, rows: 1 }];
                    let tail = paint(&ViewportView { lane, tree_context: Some((&host, &body)), ..view.clone() });
                    assert!(count(&tail) <= UI_DOCUMENT_NODES);
                    let toolbar = tail.children.iter().find(|node| node.key.as_str() == "norm-viewport-toolbar").unwrap();
                    let playback = toolbar.children.iter().find(|node| node.key.as_str() == "norm-viewport-playback").unwrap();
                    assert_eq!(playback.children.len(), 1);
                    assert_eq!(playback.children.iter().next().unwrap().key.as_str(), "norm-viewport.speed.2");
                    println!("[DEBUG] {id} reached final playback row {}/{}", window.total, window.total);
                    if std::env::var("SEMIO_VIEWPORT_STRESS_WINDOWS").is_ok() {
                        host.tree_windows = ["norm-viewport-fields", "norm-viewport-playback", "norm-viewport-help.list"].into_iter().map(|key| semio_plugin::TreeWindowRequest { body_key: body.clone(), node_key: key.into(), open: Some(true), offset: 0, rows: 70 }).collect();
                        host.tree_viewport_rows = Some(128);
                        let stressed = paint(&ViewportView { lane: NormTransient::default().apply(view_event("viewportSelectTrack", Some(&track.id), None).unwrap()).view, tree_context: Some((&host, &body)), ..view.clone() });
                        assert!(count(&stressed) <= UI_DOCUMENT_NODES);
                        println!("[DEBUG] {id} simultaneous wide toolbar and help requests used {} records", count(&stressed));
                    }
                }
            }
            let mut help_host = host.clone();
            help_host.tree_windows = vec![semio_plugin::TreeWindowRequest { body_key: body.clone(), node_key: section.key.as_str().into(), open: Some(true), offset: 0, rows: help_total }];
            help_host.tree_viewport_rows = Some(128);
            let open_panel = paint(&ViewportView { tree_context: Some((&help_host, &body)), ..view.clone() });
            assert!(count(&open_panel) <= UI_DOCUMENT_NODES);
            let open_help = open_panel.children.iter().find(|node| node.key.as_str() == "norm-viewport-help").unwrap();
            assert_eq!(count(open_help), 18);
            println!("[DEBUG] {id} disclosed help on request: help={} panel={}", count(open_help), count(&open_panel));
            if export {
                println!("[DEBUG] viewport-help {}", portable(&open_panel));
                drop(open_panel);
                ui_arena::drain();
                help_host.tree_windows[0].offset = help_total - 1;
                help_host.tree_windows[0].rows = 1;
                let lane = NormTransient::default().apply(view_event("viewportSetScenePage", None, Some(1.0)).unwrap()).view;
                let tail = paint(&ViewportView { lane, tree_context: Some((&help_host, &body)), ..view.clone() });
                assert!(count(&tail) <= UI_DOCUMENT_NODES);
                println!("[DEBUG] viewport-help-tail {}", portable(&tail));
            }
        }
        drop(root);
        ui_arena::drain();
    };
    let palette_entry = |id: &str, en: &str, de: &str| PaletteEntry { id: id.to_owned(), label_en: en.to_owned(), label_de: de.to_owned() };
    for case in paths {
        let (id, path) = case.split_once('=').expect("family=absolute source path");
        let asset = std::fs::read_to_string(path).expect("viewport sample");
        match id {
            #[cfg(not(feature = "structural-only"))]
            "en1990" => {
                use semio_s_artifact_norm_en1990_scene::live_scene_of;
                use semio_s_artifact_norm_en1990_scene_kit::ENTITY_KINDS;
                let snapshot = semio_s_artifact_norm_en1990_model::En1990Snapshot::parse_dsl(&asset).unwrap();
                let report = semio_s_artifact_norm_en1990_inferences::live::report(&snapshot);
                let palette = ENTITY_KINDS.iter().filter(|kind| kind.id != "case").map(|kind| palette_entry(kind.id, kind.label_en, kind.label_de)).collect();
                paint(id, path, live_scene_of(&snapshot, &report, &ViewOptions::default()), report, palette, "s.norm-eurocode.en1990@1/*#editor");
            }
            #[cfg(not(feature = "structural-only"))]
            "en1991" => {
                use semio_s_artifact_norm_en1991_scene::{live_scene_of, ENTITY_KINDS};
                let snapshot = semio_s_artifact_norm_en1991_model::En1991Snapshot::parse_dsl(&asset).unwrap();
                let report = semio_s_artifact_norm_en1991_inferences::live::report(&snapshot);
                let palette = ENTITY_KINDS.iter().filter(|kind| kind.id != "case").map(|kind| palette_entry(kind.id, kind.label_en, kind.label_de)).collect();
                paint(id, path, live_scene_of(&snapshot, &report), report, palette, "s.norm-eurocode.en1991@1/*#editor");
            }
            #[cfg(feature = "structural")]
            "en1992" => {
                use semio_s_artifact_norm_en1992_scene::{live_scene_of, ENTITY_KINDS};
                let snapshot = semio_s_artifact_norm_en1992_model::En1992Snapshot::parse_dsl(&asset).unwrap();
                let report = semio_s_artifact_norm_en1992_inferences::live::report(&snapshot);
                let palette = ENTITY_KINDS.iter().filter(|kind| kind.id != "case").map(|kind| palette_entry(kind.id, kind.label_en, kind.label_de)).collect();
                paint(id, path, live_scene_of(&snapshot, &report), report, palette, "s.norm-eurocode-structural.en1992@1/*#editor");
            }
            #[cfg(feature = "structural")]
            "en1993" => {
                use semio_s_artifact_norm_en1993_scene::{live_scene_of, ENTITY_KINDS};
                let snapshot = semio_s_artifact_norm_en1993_model::En1993Snapshot::parse_dsl(&asset).unwrap();
                let report = semio_s_artifact_norm_en1993_inferences::live::report(&snapshot);
                let palette = ENTITY_KINDS.iter().map(|kind| palette_entry(kind.id, kind.label_en, kind.label_de)).collect();
                paint(id, path, live_scene_of(&snapshot, &report), report, palette, "s.norm-eurocode-structural.en1993@1/*#editor");
            }
            _ => panic!("unsupported viewport sample {id}"),
        }
    }
}

#[cfg(feature = "palette-window")]
fn palette(scene: &StructuralScene) {
    use semio_plugin::ViewModel;
    use semio_s_plugin_norm_app_surface_panels::{lane_overlays::{LaneOverlays, PaletteEntry}, toolbar::render_toolbar};
    use semio_ui_contract::{Component, UI_FIXED_LIST_ITEMS};
    let request = ViewModel {
        tree_windows: vec![semio_framework::TreeWindowRequest {
            body_key: "norm.en1991.play.viewport".into(), node_key: "norm-viewport-insert".into(),
            open: Some(true), offset: (UI_FIXED_LIST_ITEMS + 2) as u32, rows: 1,
        }],
        ..ViewModel::new(Locale::En, semio_ui_locale::Terminology::Native)
    };
    let paint = |count, tree_context| {
        let palette = (0..count).map(|index| PaletteEntry { id: format!("tool-{index}"), label_en: format!("Tool {index}"), label_de: format!("Werkzeug {index}") }).collect();
        let view = ViewportView { lanes: LaneOverlays { palette, ..LaneOverlays::default() }, tree_context, ..ViewportView::default() };
        render_toolbar(scene, &view, Locale::En, "s.norm-eurocode.en1991@1/*#editor").expect("toolbar admission").0.expect("toolbar")
    };
    let compact = paint(UI_FIXED_LIST_ITEMS - 1, None);
    let insert = compact.children.iter().find(|node| node.key.as_str() == "norm-viewport-insert").expect("Insert");
    let Component::TreeSection(section) = &insert.component else { panic!("Insert") };
    assert!(section.window.is_none());
    assert_eq!(insert.children.len(), UI_FIXED_LIST_ITEMS);
    assert_eq!(insert.children[UI_FIXED_LIST_ITEMS - 1].key.as_str(), "norm-viewport.insert.off");
    let paged = paint(UI_FIXED_LIST_ITEMS + 2, Some((&request, "norm.en1991.play.viewport")));
    let insert = paged.children.iter().find(|node| node.key.as_str() == "norm-viewport-insert").expect("Insert");
    let Component::TreeSection(section) = &insert.component else { panic!("Insert") };
    let window = section.window.as_ref().expect("paged Insert");
    assert_eq!((window.total as usize, window.offset as usize), (UI_FIXED_LIST_ITEMS + 3, UI_FIXED_LIST_ITEMS + 2));
    assert_eq!(insert.children.len(), 1);
    assert_eq!(insert.children[0].key.as_str(), format!("norm-viewport.insert.tool-{}", UI_FIXED_LIST_ITEMS + 1));
    println!("[DEBUG] EN 1991 admitted all 32 fixed palette rows and rendered the final requested row of a 35-row host-paged palette");
}

fn diagrams(id: &str, diagrams: &[semio_s_plugin_norm_structural_scene::Diagram2d]) {
    #[cfg(feature = "window-admission")]
    {
        use semio_s_plugin_norm_app_surface_visual_chrome::diagrams2d::{render_diagrams, DiagramsWindowSpec};
        use semio_ui_contract::{BuiltNode, Component, SurfaceKind};
        fn canvas_count(node: &BuiltNode) -> usize {
            usize::from(matches!(&node.component, Component::Surface(props) if props.kind == SurfaceKind::Canvas2d))
                + node.children.iter().map(canvas_count).sum::<usize>()
        }
        fn node_count(node: &BuiltNode) -> usize {
            1 + node.children.iter().map(node_count).sum::<usize>()
        }
        let controller_id = if id == "en1992" { "s.norm-eurocode-structural.en1992@1/*#editor" } else { "s.norm-eurocode.en1990@1/*#editor" };
        let spec = DiagramsWindowSpec { controller_id: Some(controller_id), ..DiagramsWindowSpec::default() };
        let key = format!("norm.{id}.play.diagrams");
        let cards_key = format!("{key}.cards");
        let root = render_diagrams(&key, diagrams, &spec, &semio_plugin::TreeWindows::unhosted(), Locale::En).expect("complete diagram window admission");
        let initial_nodes = node_count(&root);
        assert!(initial_nodes <= semio_ui_contract::UI_DOCUMENT_NODES, "{id} initial diagram has {initial_nodes} retained nodes");
        let count = canvas_count(&root);
        let cards = root.children.iter().find(|node| node.key.as_str() == cards_key);
        if let Some(cards) = cards {
            let Component::TreeSection(section) = &cards.component else { panic!("diagram cards section") };
            let window = section.window.as_ref().expect("diagram cards window");
            assert_eq!(window.total as usize, diagrams.len());
            assert!(count > 0 && count <= diagrams.len());
            let view = semio_plugin::ViewModel {
                tree_windows: vec![semio_framework::TreeWindowRequest { body_key: key.clone(), node_key: cards_key.clone(), open: Some(true), offset: (diagrams.len() - 1) as u32, rows: 1 }],
                ..semio_plugin::ViewModel::new(Locale::En, semio_ui_locale::Terminology::Native)
            };
            let tail = render_diagrams(&key, diagrams, &spec, &semio_plugin::TreeWindows::for_body(&view, &key), Locale::En).expect("final diagram card admission");
            assert_eq!(canvas_count(&tail), 1);
            let final_nodes = node_count(&tail);
            assert!(final_nodes <= semio_ui_contract::UI_DOCUMENT_NODES, "{id} final diagram has {final_nodes} retained nodes");
            let section = tail.children.iter().find(|node| node.key.as_str() == cards_key).expect("diagram cards");
            let last = diagrams.last().expect("diagram").id.replace(':', ".");
            assert_eq!(section.children[0].key.as_str(), format!("{key}.{last}.item"));
            #[cfg(feature = "gallery-json")]
            println!("[DEBUG] gallery {}", serde_json::json!({
                "body": key, "head": portable(&root), "tail": portable(&tail),
            }));
            println!("[DEBUG] {id} admitted {count}/{} diagram cards in {initial_nodes} retained nodes and rendered the last requested card in {final_nodes} nodes", diagrams.len());
        } else {
            assert_eq!(count, diagrams.len());
            println!("[DEBUG] {id} admitted all {} diagrams to the complete window in {initial_nodes} retained nodes", diagrams.len());
        }
        drop(root);
        ui_arena::drain();
        if diagrams.len() > 1 {
            let mut maximum = 0;
            for (offset, diagram) in diagrams.iter().enumerate() {
                let view = semio_plugin::ViewModel {
                    tree_windows: vec![semio_framework::TreeWindowRequest { body_key: key.clone(), node_key: cards_key.clone(), open: Some(true), offset: offset as u32, rows: 1 }],
                    ..semio_plugin::ViewModel::new(Locale::En, semio_ui_locale::Terminology::Native)
                };
                let page = render_diagrams(&key, diagrams, &spec, &semio_plugin::TreeWindows::for_body(&view, &key), Locale::En).expect("one requested diagram card");
                let nodes = node_count(&page);
                let section = page.children.iter().find(|node| node.key.as_str() == cards_key).expect("diagram cards");
                let expected = diagram.id.replace(':', ".");
                assert!(nodes <= semio_ui_contract::UI_DOCUMENT_NODES, "{id} diagram {expected} has {nodes} retained nodes");
                assert_eq!(section.children.len(), 1, "{id} diagram {expected} must materialise");
                assert_eq!(section.children[0].key.as_str(), format!("{key}.{expected}.item"));
                assert_eq!(canvas_count(&page), 1);
                maximum = maximum.max(nodes);
                drop(page);
                ui_arena::drain();
            }
            println!("[DEBUG] {id} reached all {} individual diagram cards with at most {maximum} retained nodes", diagrams.len());
        }
    }
    let options = LoweringOptions { width: 560.0, height: 360.0, locale: Locale::En, dark: false, selected: None, hovered: None, min_target_px: 24 };
    let mut largest = (0, 0);
    #[cfg(feature = "retained-json")]
    let mut retained = None;
    for diagram in diagrams {
        let (canvas, _) = diagram_scene(diagram, &options);
        assert_ne!(canvas.layers_json, "[]");
        let unsplit = canvas.encode_pack().expect("complete canvas scene").len();
        let (fixed, restored) = transmitted(&canvas);
        assert_eq!(canvas.layers_json, restored.layers_json);
        #[cfg(feature = "retained-json")]
        if unsplit > largest.0 {
            retained = Some(canvas);
        }
        largest = largest.max((unsplit, fixed));
    }
    assert!(largest.0 > UI_FIXED_BYTES);
    println!("[DEBUG] {id} preserved {} real diagrams, reducing the largest fixed payload from {} to {} bytes", diagrams.len(), largest.0, largest.1);
    #[cfg(feature = "retained-json")]
    publish(&format!("{id}.diagram"), semio_ui_contract::SurfaceKind::Canvas2d, &retained.expect("largest diagram"));
}

#[cfg(not(feature = "structural-only"))]
fn eurocode(args: &mut impl Iterator<Item = String>) {
    let en1990_text = std::fs::read_to_string(args.next().expect("EN 1990 source")).unwrap();
    let en1991_text = std::fs::read_to_string(args.next().expect("EN 1991 source")).unwrap();
    let en1990 = semio_s_artifact_norm_en1990_model::En1990Snapshot::parse_dsl(&en1990_text).unwrap();
    let en1991 = semio_s_artifact_norm_en1991_model::En1991Snapshot::parse_dsl(&en1991_text).unwrap();
    let en1990_report = semio_s_artifact_norm_en1990_inferences::live::report(&en1990);
    let en1991_report = semio_s_artifact_norm_en1991_inferences::live::report(&en1991);
    world("en1990", semio_s_artifact_norm_en1990_scene::live_scene_of(&en1990, &en1990_report, &ViewOptions::default()));
    world("en1991", semio_s_artifact_norm_en1991_scene::live_scene_of(&en1991, &en1991_report));
    diagrams("en1990", &semio_s_artifact_norm_en1990_scene_diagrams::diagrams(&en1990, &en1990_report, None, None));
}

fn main() {
    #[cfg(any(feature = "window-admission", feature = "palette-window"))]
    let arena = ui_arena::register();
    #[cfg(feature = "viewport-census")]
    {
        viewport_census(std::env::args().skip(1));
        ui_arena::close(arena);
        return;
    }
    #[cfg(not(feature = "viewport-census"))]
    {
    let mut args = std::env::args().skip(1);
    #[cfg(not(feature = "structural-only"))]
    eurocode(&mut args);
    #[cfg(feature = "structural")]
    structural(args);
    #[cfg(any(feature = "window-admission", feature = "palette-window"))]
    ui_arena::close(arena);
    }
}

#[cfg(feature = "structural")]
fn structural(mut args: impl Iterator<Item = String>) {
    let en1992_text = std::fs::read_to_string(args.next().expect("EN 1992 source")).unwrap();
    let en1993_text = std::fs::read_to_string(args.next().expect("EN 1993 source")).unwrap();
    let en1992 = semio_s_artifact_norm_en1992_model::En1992Snapshot::parse_dsl(&en1992_text).unwrap();
    let en1993 = semio_s_artifact_norm_en1993_model::En1993Snapshot::parse_dsl(&en1993_text).unwrap();
    let en1992_report = semio_s_artifact_norm_en1992_inferences::live::report(&en1992);
    let en1993_report = semio_s_artifact_norm_en1993_inferences::live::report(&en1993);
    world("en1992", semio_s_artifact_norm_en1992_scene::live_scene_of(&en1992, &en1992_report));
    world("en1993", semio_s_artifact_norm_en1993_scene::live_scene_of(&en1993, &en1993_report));
    diagrams("en1992", &semio_s_artifact_norm_en1992_scene::diagrams(&en1992, &en1992_report, None));
}
