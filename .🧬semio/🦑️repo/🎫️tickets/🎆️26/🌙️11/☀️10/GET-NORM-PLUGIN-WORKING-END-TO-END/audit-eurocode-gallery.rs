use audited_diagrams::diagrams2d::{render_diagrams, DiagramsWindowSpec};
use pack::snapshot::ArtifactDsl;
use semio_plugin::{BuiltNode, TreeWindowRequest, TreeWindows, ViewModel};
use semio_s_plugin_norm_compliance::CheckStatus;
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
    ("en1994", "norm-eurocode-structural", En1994Snapshot),
    ("en1995", "norm-eurocode-materials", En1995Snapshot),
    ("en1996", "norm-eurocode-materials", En1996Snapshot),
    ("en1997", "norm-eurocode", En1997Snapshot),
    ("en1998", "norm-eurocode", En1998Snapshot),
    ("en1999", "norm-eurocode-materials", En1999Snapshot)
);

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
        let diagrams = audited_scene::diagrams(&document, &report, None);
        let focused = audited_scene::diagrams(&document, &report, worst);
        let focused = if focused.is_empty() { &diagrams } else { &focused };
        for locale in [Locale::En, Locale::De] {
            #[cfg(feature = "en1994")]
            { passed &= verify_viewport(&path, &document, &report, locale); }
            let mut view = ViewModel::new(locale, Terminology::Native);
            let mut cases = vec![("focus", focused, None), ("first", &diagrams, None)];
            if diagrams.len() > 1 {
                let offset = (diagrams.len() - 1) as u32;
                view.tree_windows = vec![TreeWindowRequest { body_key: BODY.into(), node_key: format!("{BODY}.cards"), open: Some(true), offset, rows: 1 }];
                cases.push(("last", &diagrams, Some(offset)));
            }
            for (variant, diagrams, offset) in cases {
                let tree = if offset.is_some() { TreeWindows::for_body(&view, BODY) } else { TreeWindows::unhosted() };
                match render_diagrams(BODY, diagrams, &spec, &tree, locale) {
                    Ok(node) => {
                        let mut canvases = Vec::new();
                        let nodes = census(&node, &mut canvases);
                        let correct_canvas = offset.map_or(diagrams.is_empty() == canvases.is_empty(), |index| {
                            canvases == [format!("{BODY}.{}", diagrams[index as usize].id.replace(':', "."))]
                        });
                        let accepted = nodes <= UI_DOCUMENT_NODES && correct_canvas;
                        let status = if accepted { "OK" } else { "FAIL" };
                        println!("[DEBUG] {status} {path} {locale:?} {variant} nodes={nodes} canvases={} total={}", canvases.len(), diagrams.len());
                        if emit && locale == Locale::En && variant != "focus" {
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
