use audited_diagrams::diagrams2d::{render_diagrams, DiagramsWindowSpec};
use pack::snapshot::ArtifactDsl;
use semio_plugin::{BuiltNode, TreeWindowRequest, ViewModel};
#[cfg(not(feature = "gallery-cohort"))]
use semio_plugin::{PanelTreeBuilder, TreeWindows};
#[cfg(feature = "gallery-cohort")]
use semio_plugin_window_kits_rich::tree_kit::{PanelTreeBuilder, TreeWindows};
use semio_s_plugin_norm_compliance::CheckStatus;
use semio_s_plugin_norm_structural_scene::Diagram2d;
use semio_ui_contract::{Component, SurfaceKind, UI_DOCUMENT_NODES};
use semio_ui_locale::{Locale, Terminology};

#[path = "eurocode-ui-arena.rs"]
mod arena;

#[cfg(feature = "en1994")]
extern crate audited_model as semio_s_artifact_norm_en1994_model;
#[cfg(feature = "en1994")]
extern crate audited_scene as semio_s_artifact_norm_en1994_scene;
#[cfg(feature = "en1994")]
#[path = "../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs"]
mod authored_viewport;
#[cfg(feature = "en1994")]
const VARIANT: &str = "en1994";
#[cfg(feature = "en1994")]
const CONTROLLER_ID: &str = CONTROLLER;

macro_rules! families {
    ($(($family:literal, $owner:literal, $model:ident)),+) => {
        $(
            #[cfg(feature = $family)]
            use audited_model::$model as Snapshot;
            #[cfg(feature = $family)]
            const BODY: &str = concat!("norm.", $family, ".play.diagrams");
            #[cfg(feature = $family)]
            const CONTROLLER: &str = concat!("s.", $owner, ".", $family, "@1/*#editor");
        )+
    };
}

families!(
    ("en1990", "norm-eurocode", En1990Snapshot),
    ("en1991", "norm-eurocode", En1991Snapshot),
    ("en1992", "norm-eurocode-structural", En1992Snapshot),
    ("en1993", "norm-eurocode-structural", En1993Snapshot),
    ("en1994", "norm-eurocode-structural", En1994Snapshot),
    ("en1995", "norm-eurocode-materials", En1995Snapshot),
    ("en1996", "norm-eurocode-materials", En1996Snapshot),
    ("en1997", "norm-eurocode", En1997Snapshot),
    ("en1998", "norm-eurocode", En1998Snapshot),
    ("en1999", "norm-eurocode-materials", En1999Snapshot)
);

fn diagrams(document: &Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    #[cfg(feature = "en1990")]
    return audited_scene::diagrams(document, report, focus, None);
    #[cfg(not(feature = "en1990"))]
    return audited_scene::diagrams(document, report, focus);
}

fn census(node: &BuiltNode, canvases: &mut Vec<String>) -> usize {
    if matches!(&node.component, Component::Surface(surface) if surface.kind == SurfaceKind::Canvas2d) {
        canvases.push(node.key.as_str().to_owned());
    }
    1 + node.children.iter().map(|child| census(child, canvases)).sum::<usize>()
}

fn portable(node: &BuiltNode) -> serde_json::Value {
    serde_json::json!({
        "key": node.key, "component": node.component, "layout": node.layout, "style": node.style,
        "accessibility": node.accessibility, "children": node.children.iter().map(portable).collect::<Vec<_>>(),
    })
}

fn matrix_ledger(diagrams: &[Diagram2d], spec: &DiagramsWindowSpec, locale: Locale, offset: Option<u32>, nested: Option<(&str, u32)>) {
    use semio_ui_contract::{Buildable, HasBase, HasChildren, Label, column, tree_item};
    let namespace = format!("{BODY}.gallery");
    let section = format!("{BODY}.cards");
    let categories = Label::try_from(if locale == Locale::En { "Diagrams" } else { "Diagramme" }).expect("section label");
    let mut view = ViewModel::new(locale, Terminology::Native);
    view.tree_windows = offset.into_iter().map(|offset| TreeWindowRequest { body_key: BODY.into(), node_key: section.clone(), open: Some(true), offset, rows: 1 }).collect();
    if let Some((kind, nested_offset)) = nested {
        let diagram = &diagrams[0];
        let base = format!("{BODY}.{}", diagram.id.replace(':', "."));
        for section_kind in ["markers", "rows"] {
            view.tree_windows.push(TreeWindowRequest {
                body_key: BODY.into(),
                node_key: format!("{section}{}{base}.{section_kind}", semio_ui_contract::TREE_WINDOW_PATH_SEPARATOR),
                open: Some(section_kind == kind || kind == "both"),
                offset: nested_offset,
                rows: 1,
            });
        }
    }
    let windows = TreeWindows::for_body(&view, BODY);
    let scene = PanelTreeBuilder::indexed_body(&namespace, &windows, &section, categories, diagrams.len(), |index| {
        let diagram = &diagrams[index];
        let items = windows.items_remaining();
        let one = render_diagrams(BODY, std::slice::from_ref(diagram), spec, &windows, locale)?;
        let base = format!("{BODY}.{}", diagram.id.replace(':', "."));
        let costs: Vec<_> = one.children.iter().skip(1).map(|child| (child.key.as_str(), census(child, &mut Vec::new()), semio_ui_runtime::surface_subtree_items(child))).collect();
        let after_table = windows.items_remaining();
        println!("[DEBUG] nested {locale:?} {offset:?} {index} limit={nested:?} budget={items} after_table={after_table} children={costs:?}");
        if let Some(table) = one.children.iter().last() {
            let pages: Vec<_> = table.children.iter().map(|section| (section.key.as_str(), section.children.iter().map(|row| row.key.as_str()).collect::<Vec<_>>())).collect();
            println!("[DEBUG] pages {locale:?} {nested:?} {pages:?}");
        }
        let card = column().try_id(format!("{base}.card")).ok().expect("card id").try_children(one.children.into_iter().skip(1).filter(|child| !nested.is_some_and(|(kind, _)| kind == "both") || !child.key.as_str().ends_with(".basis"))).ok().expect("card children").try_build().expect("card");
        let label = Label::try_from(if locale == Locale::En { diagram.title.en.as_str() } else { diagram.title.de.as_str() }).expect("chart title");
        let row = tree_item(label).try_id(format!("{base}.item")).ok().expect("row id").try_child(card).ok().expect("row child").try_build().expect("row");
        println!("[DEBUG] row {locale:?} {offset:?} {index} limit={nested:?} nodes={} items={:?}", census(&row, &mut Vec::new()), semio_ui_runtime::surface_subtree_items(&row));
        Ok(row)
    }).expect("gallery header");
    println!("[DEBUG] body {locale:?} {offset:?} limit={nested:?} nodes={} items={:?} remaining={}", census(&scene, &mut Vec::new()), semio_ui_runtime::surface_subtree_items(&scene), windows.items_remaining());
}

fn matrix_pages(diagrams: &[Diagram2d], spec: &DiagramsWindowSpec, locale: Locale) -> bool {
    let gallery_key = format!("{BODY}.cards");
    let Some((index, chart)) = diagrams.iter().enumerate().find(|(_, diagram)| diagram.id == "matrix:document") else {
        println!("[DEBUG] FAIL matrix-pair {locale:?} missing authored matrix");
        return false;
    };
    let base = format!("{BODY}.{}", chart.id.replace(':', "."));
    let mut passed = true;
    for (name, offset, rows, markers, data) in [
        ("first", 0, 48, "m0", "1"),
        ("last", u32::MAX, 1, "m128", "72"),
    ] {
        let mut view = ViewModel::new(locale, Terminology::Native);
        view.tree_windows.push(TreeWindowRequest { body_key: BODY.into(), node_key: gallery_key.clone(), open: Some(true), offset: index as u32, rows: 1 });
        for section in ["markers", "rows"] {
            view.tree_windows.push(TreeWindowRequest {
                body_key: BODY.into(),
                node_key: format!("{gallery_key}{}{base}.{section}", semio_ui_contract::TREE_WINDOW_PATH_SEPARATOR),
                open: Some(true), offset, rows,
            });
        }
        let tree = TreeWindows::for_body(&view, BODY);
        let body = render_diagrams(BODY, diagrams, spec, &tree, locale).expect("render matrix pages");
        let mut canvases = Vec::new();
        let nodes = census(&body, &mut canvases);
        let items = semio_ui_runtime::surface_subtree_items(&body).expect("matrix item census");
        let mut keys = Vec::new();
        let mut pending = vec![&body];
        while let Some(node) = pending.pop() {
            keys.push(node.key.as_str());
            pending.extend(node.children.iter());
        }
        let marker_key = format!("{base}.marker.{markers}");
        let row_key = format!("{base}.row.{data}");
        let accepted = canvases.contains(&base) && keys.contains(&marker_key.as_str()) && keys.contains(&row_key.as_str()) && nodes <= UI_DOCUMENT_NODES && items < semio_ui_runtime::SURFACE_RECONCILE_MAX_ITEMS;
        let status = if accepted { "OK" } else { "FAIL" };
        println!("[DEBUG] {status} matrix-pair {locale:?} {name} nodes={nodes} items={items} canvas={} marker={} data={}", canvases.contains(&base), keys.contains(&marker_key.as_str()), keys.contains(&row_key.as_str()));
        passed &= accepted;
        drop(body);
        arena::drain();
    }
    passed
}

#[cfg(feature = "en1994")]
fn measure_viewport_lanes(document: &Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) {
    use semio_s_plugin_norm_app_surface::viewport3d::{config::Lang, toolbar::{lowered_viewport, ViewportView}, window::world3d_scene_of};
    use semio_ui_scene::SceneDoc;
    let structural = authored_viewport::scene_of(document, report);
    let viewport = lowered_viewport(&structural, &ViewportView::default(), &[], Lang::En);
    let scene = world3d_scene_of(&viewport, "norm.en1994", false);
    println!("[DEBUG] viewport stats {:?}", viewport.stats);
    for lane in scene.split_lanes().1 {
        let deflated = semio_framework_deflate::deflate(lane.payload.as_bytes());
        assert_eq!(semio_framework_deflate::inflate(&deflated, lane.payload.len()).expect("inflate scene"), lane.payload.as_bytes());
        println!("[DEBUG] lane {} raw={} deflate={}", lane.key, lane.payload.len(), deflated.len());
    }
}

#[cfg(feature = "en1994")]
fn verify_viewport(path: &str, document: &Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport, locale: Locale) -> bool {
    use semio_s_plugin_norm_app_surface::{viewport3d::toolbar::ViewportView, NormSelection};
    let result = authored_viewport::render_with(document, report, &NormSelection::default(), &ViewportView::default(), locale);
    let accepted = match result {
        Ok(node) => {
            fn models(node: &BuiltNode) -> usize {
                usize::from(matches!(&node.component, Component::Surface(surface) if surface.kind == SurfaceKind::World3d))
                    + node.children.iter().map(models).sum::<usize>()
            }
            let nodes = census(&node, &mut Vec::new());
            if nodes > UI_DOCUMENT_NODES && locale == Locale::En {
                for child in &node.children {
                    let sections = child.children.iter().map(|section| (section.key.as_str(), census(section, &mut Vec::new()))).collect::<Vec<_>>();
                    println!("[DEBUG] viewport section {} {} {sections:?}", child.key.as_str(), census(child, &mut Vec::new()));
                }
            }
            let models = models(&node);
            let accepted = nodes <= UI_DOCUMENT_NODES && models == 1;
            let status = if accepted { "OK" } else { "FAIL" };
            println!("[DEBUG] {status} {path} {locale:?} viewport nodes={nodes} models={models}");
            accepted
        }
        Err(error) => {
            println!("[DEBUG] FAIL {path} {locale:?} viewport {error:?}");
            false
        }
    };
    arena::drain();
    accepted
}

fn main() -> std::process::ExitCode {
    let owner = arena::register();
    let spec = DiagramsWindowSpec { controller_id: Some(CONTROLLER), ..DiagramsWindowSpec::default() };
    let mut passed = true;
    let mut args = std::env::args().skip(1).peekable();
    let emit = args.next_if(|argument| argument == "--emit").is_some();
    let ledger = args.next_if(|argument| argument == "--matrix-ledger").is_some();
    let every = args.next_if(|argument| argument == "--every").is_some();
    let verify_matrix_pages = args.next_if(|argument| argument == "--matrix-pages").is_some();
    #[cfg(feature = "en1994")]
    let measure = args.next_if(|argument| argument == "--measure-viewport").is_some();
    for path in args {
        let text = std::fs::read_to_string(&path).expect("read example asset");
        let document = Snapshot::parse_dsl(&text).expect("decode example asset");
        let report = audited_report::live::report(&document);
        #[cfg(feature = "en1994")]
        if measure { measure_viewport_lanes(&document, &report); }
        let worst = report.checks.iter()
            .filter(|check| check.status != CheckStatus::NotApplicable && !check.subject.entity_id.is_empty())
            .max_by(|a, b| a.utilization.total_cmp(&b.utilization))
            .map(|check| check.subject.entity_id.as_str());
        let focused = diagrams(&document, &report, worst);
        let diagrams = diagrams(&document, &report, None);
        let focused = if focused.is_empty() { &diagrams } else { &focused };
        for locale in [Locale::En, Locale::De] {
            if verify_matrix_pages && std::path::Path::new(&path).components().any(|part| part.as_os_str().to_string_lossy().ends_with("road-bridge-showcase")) {
                passed &= matrix_pages(&diagrams, &spec, locale);
            }
            if ledger {
                for offset in [None, Some(0)] {
                    matrix_ledger(&diagrams, &spec, locale, offset, None);
                    arena::drain();
                }
                for kind in ["markers", "rows", "both"] {
                    for nested_offset in [0, u32::MAX] {
                        matrix_ledger(&diagrams, &spec, locale, Some(0), Some((kind, nested_offset)));
                        arena::drain();
                    }
                }
            }
            #[cfg(feature = "en1994")]
            { passed &= verify_viewport(&path, &document, &report, locale); }
            let mut view = ViewModel::new(locale, Terminology::Native);
            let mut cases = vec![("focus", focused, None), ("first", &diagrams, None)];
            if diagrams.len() > 1 {
                let offset = (diagrams.len() - 1) as u32;
                if every {
                    cases.extend((0..offset).map(|index| ("requested", &diagrams, Some(index))));
                }
                cases.push(("last", &diagrams, Some(offset)));
            }
            for (variant, diagrams, offset) in cases {
                view.tree_windows = offset.into_iter().map(|offset| TreeWindowRequest { body_key: BODY.into(), node_key: format!("{BODY}.cards"), open: Some(true), offset, rows: 1 }).collect();
                let tree = if offset.is_some() { TreeWindows::for_body(&view, BODY) } else { TreeWindows::unhosted() };
                match render_diagrams(BODY, diagrams, &spec, &tree, locale) {
                    Ok(node) => {
                        let mut canvases = Vec::new();
                        let nodes = census(&node, &mut canvases);
                        let start = offset.unwrap_or_default() as usize;
                        let correct_canvas = diagrams.is_empty() == canvases.is_empty()
                            && (offset.is_none() || canvases.len() == 1)
                            && canvases.len() <= diagrams.len().saturating_sub(start)
                            && canvases.iter().zip(diagrams.iter().skip(start)).all(|(key, diagram)| *key == format!("{BODY}.{}", diagram.id.replace(':', ".")));
                        let items = semio_ui_runtime::surface_subtree_items(&node).expect("price retained body");
                        let accepted = nodes <= UI_DOCUMENT_NODES && items < semio_ui_runtime::SURFACE_RECONCILE_MAX_ITEMS && correct_canvas;
                        let status = if accepted { "OK" } else { "FAIL" };
                        println!("[DEBUG] {status} {path} {locale:?} {variant} offset={offset:?} nodes={nodes} items={items} canvases={} total={}", canvases.len(), diagrams.len());
                        if let (false, None, Some(first)) = (accepted, offset, diagrams.first()) {
                            let id = &first.id;
                            let single = render_diagrams(BODY, std::slice::from_ref(first), &spec, &TreeWindows::unhosted(), locale);
                            let census = single.map(|node| {
                                let child_items: Vec<_> = node.children.iter().map(|child| (child.key.as_str().to_owned(), semio_ui_runtime::surface_subtree_items(child))).collect();
                                (census(&node, &mut Vec::new()), semio_ui_runtime::surface_subtree_items(&node), child_items)
                            });
                            println!("[DEBUG] first diagram {id} payload ({}, {}, {}) nodes/items={census:?}", first.series.len(), first.markers.len(), first.annotations.len());
                        }
                        if emit && locale == Locale::En && matches!(variant, "first" | "last") {
                            let variant = if variant == "first" { "all-diagrams" } else { "last-diagram" };
                            println!("[DEBUG] diagram-window {}", serde_json::json!({"variant": variant, "root": portable(&node)}));
                        }
                        passed &= accepted;
                    }
                    Err(error) => {
                        println!("[DEBUG] FAIL {path} {locale:?} {variant} {error:?}");
                        passed = false;
                    }
                }
                arena::drain();
            }
        }
    }
    arena::close(owner);
    if passed { std::process::ExitCode::SUCCESS } else { std::process::ExitCode::FAILURE }
}

