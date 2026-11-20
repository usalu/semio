use audited_editor::{modes::edit::windows, Host, CONTROLLER_ID};
use pack::snapshot::ArtifactDsl;
use semio_plugin::{BuiltNode, TreeWindows, UiAssemblyResult};
use semio_s_plugin_norm_app_surface::{viewport3d::toolbar::ViewportView, NormSelection};
use semio_s_plugin_norm_compliance::CheckStatus;
use semio_ui_contract::{Component, SurfaceKind};
use semio_ui_contract_values::ui_value_headroom;
use semio_ui_locale::Locale;

#[path = "eurocode-ui-arena.rs"]
mod arena;

fn surface_count(node: &BuiltNode, kind: SurfaceKind) -> usize {
    usize::from(matches!(&node.component, Component::Surface(surface) if surface.kind == kind))
        + node.children.iter().map(|child| surface_count(child, kind)).sum::<usize>()
}

fn node_count(node: &BuiltNode) -> usize {
    1 + node.children.iter().map(node_count).sum::<usize>()
}

#[cfg(feature = "updated-diagrams")]
fn render_diagrams(host: &Host, focus: Option<&str>, locale: Locale, offset: Option<u32>) -> UiAssemblyResult<BuiltNode> {
    use audited_diagrams::diagrams2d::{render_diagrams, DiagramsWindowSpec};
    use semio_plugin::{TreeWindowRequest, ViewModel};
    let body = windows::diagrams::BODY_DIAGRAMS;
    let focused = audited_scene::diagrams(host.document(), host.report(), focus);
    let diagrams = if focused.is_empty() && focus.is_some() {
        audited_scene::diagrams(host.document(), host.report(), None)
    } else {
        focused
    };
    let spec = DiagramsWindowSpec { controller_id: Some(CONTROLLER_ID), ..DiagramsWindowSpec::default() };
    if let Some(offset) = offset {
        let view = ViewModel {
            tree_windows: vec![TreeWindowRequest { body_key: body.into(), node_key: format!("{body}.cards"), open: Some(true), offset, rows: 1 }],
            ..ViewModel::new(locale, semio_ui_locale::Terminology::Native)
        };
        let node = render_diagrams(body, &diagrams, &spec, &TreeWindows::for_body(&view, body), locale)?;
        let requested_card = node.children.iter().any(|child| {
            let id = format!("{body}.{}.item", diagrams[offset as usize].id.replace(':', "."));
            child.children.iter().any(|row| row.key.as_str() == id)
        });
        if surface_count(&node, SurfaceKind::Canvas2d) != 1 || !requested_card {
            return Err(semio_plugin::PluginAssemblyError::new("audit.requested-diagram", "The requested diagram card did not render"));
        }
        Ok(node)
    } else {
        let node = render_diagrams(body, &diagrams, &spec, &TreeWindows::unhosted(), locale)?;
        if !diagrams.is_empty() && surface_count(&node, SurfaceKind::Canvas2d) == 0 {
            return Err(semio_plugin::PluginAssemblyError::new("audit.empty-diagrams", "The diagram window did not render a first card"));
        }
        Ok(node)
    }
}

#[cfg(not(feature = "updated-diagrams"))]
fn render_diagrams(host: &Host, focus: Option<&str>, locale: Locale, _: Option<u32>) -> UiAssemblyResult<BuiltNode> {
    windows::diagrams::render(host.document(), host.report(), focus, &TreeWindows::unhosted(), locale)
}

fn portable(node: &BuiltNode) -> serde_json::Value {
    serde_json::json!({
        "key": node.key, "component": node.component, "layout": node.layout, "style": node.style,
        "accessibility": node.accessibility, "children": node.children.iter().map(portable).collect::<Vec<_>>(),
    })
}

fn check(path: &str, locale: Locale, window: &str, result: UiAssemblyResult<BuiltNode>, emit: bool) -> bool {
    let accepted = match result {
        Ok(node) => {
            let models = surface_count(&node, SurfaceKind::World3d);
            let diagrams = surface_count(&node, SurfaceKind::Canvas2d);
            let nodes = node_count(&node);
            if nodes > semio_ui_contract::UI_DOCUMENT_NODES {
                println!("[DEBUG] FAIL {path} {locale:?} {window} retained node capacity {nodes}");
                drop(node);
                arena::drain();
                return false;
            }
            println!("[DEBUG] OK {path} {locale:?} {window} models={models} diagrams={diagrams} nodes={nodes}");
            if emit && locale == Locale::En && matches!(window, "all-diagrams" | "last-diagram") {
                println!("[DEBUG] diagram-window {}", serde_json::json!({"variant": window, "root": portable(&node)}));
            }
            true
        }
        Err(error) => {
            println!("[DEBUG] FAIL {path} {locale:?} {window} {error:?}");
            false
        }
    };
    arena::drain();
    accepted
}

fn main() -> std::process::ExitCode {
    let owner = arena::register();
    let initial_headroom = ui_value_headroom();
    let mut passed = true;
    let mut args = std::env::args().skip(1).peekable();
    let emit = args.next_if(|argument| argument == "--emit").is_some();
    for path in args {
        let text = std::fs::read_to_string(&path).expect("read example asset");
        let host = match ArtifactDsl::parse_dsl(&text) {
            Ok(document) => Host::live(document),
            Err(error) => {
                println!("[DEBUG] FAIL {path} decode {error}");
                passed = false;
                continue;
            }
        };
        let selection = NormSelection::default();
        let worst = host.report().checks.iter()
            .filter(|check| check.status != CheckStatus::NotApplicable && !check.subject.entity_id.is_empty())
            .max_by(|a, b| a.utilization.total_cmp(&b.utilization))
            .map(|check| check.subject.entity_id.as_str());
        let count = windows::diagrams::all(host.document(), host.report()).len();
        println!("[DEBUG] PLAN {path} focus={worst:?} diagrams={count}");
        for locale in [Locale::En, Locale::De] {
            if std::env::var_os("SEMIO_AUDIT_DESCRIPTIONS").is_some() {
                let protocol = semio_s_plugin_norm_app_surface::protocol_locale(locale);
                for (index, report) in host.report().checks.iter().enumerate() {
                    let description = report.explanation.resolve(&protocol);
                    if description.len() > semio_ui_contract::UI_TEXT_MAX_BYTES {
                        println!("[DEBUG] LONG {path} {locale:?} check={index} id={} explanation bytes={}", report.id, description.len());
                    }
                    let inspection = semio_s_plugin_norm_app_surface::render_inspection(host.report(), Some(index as u32), locale, Some(CONTROLLER_ID));
                    if let Err(error) = inspection {
                        println!("[DEBUG] FAIL {path} {locale:?} inspection check={index} id={} {error:?}", report.id);
                        passed = false;
                    }
                    arena::drain();
                }
                continue;
            }
            passed &= check(&path, locale, "outliner", windows::outliner::render(&host, &selection, locale, CONTROLLER_ID, &TreeWindows::unhosted()), emit);
            passed &= check(&path, locale, "properties", windows::properties::render(&host, &selection, locale, CONTROLLER_ID, &TreeWindows::unhosted()), emit);
            passed &= check(&path, locale, "viewport", windows::viewport::render_with(host.document(), host.report(), &selection, &ViewportView::default(), locale), emit);
            passed &= check(&path, locale, "checks", windows::checks::render(host.report(), None, &TreeWindows::unhosted(), locale, Some(CONTROLLER_ID)), emit);
            passed &= check(&path, locale, "diagrams", render_diagrams(&host, worst, locale, None), emit);
            passed &= check(&path, locale, "all-diagrams", render_diagrams(&host, None, locale, None), emit);
            #[cfg(feature = "updated-diagrams")]
            if count > 1 {
                passed &= check(&path, locale, "last-diagram", render_diagrams(&host, None, locale, Some((count - 1) as u32)), emit);
            }
        }
    }
    arena::drain();
    let remaining = ui_value_headroom();
    if remaining == initial_headroom {
        arena::close(owner);
    } else {
        println!("[DEBUG] arena stayed open through process exit with {} retained collections and {} retained pages",
            initial_headroom.collections - remaining.collections, initial_headroom.items - remaining.items);
        std::mem::forget(owner);
    }
    if passed { std::process::ExitCode::SUCCESS } else { std::process::ExitCode::FAILURE }
}
