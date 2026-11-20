use pack::snapshot::ArtifactDsl;
use semio_s_plugin_norm_app_surface::{protocol_locale, render_inspection};
use semio_ui_contract::{BuiltNode, Component, UI_DOCUMENT_NODES, UI_TEXT_MAX_BYTES};
use semio_ui_locale::Locale;

#[path = "eurocode-ui-arena.rs"]
mod arena;

#[cfg(feature = "en1992")]
use audited_model::En1992Snapshot as Snapshot;
#[cfg(feature = "en1993")]
use audited_model::En1993Snapshot as Snapshot;
#[cfg(feature = "en1994")]
use audited_model::En1994Snapshot as Snapshot;

fn node_count(node: &BuiltNode) -> usize {
    1 + node.children.iter().map(node_count).sum::<usize>()
}

fn find<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
    if node.key.as_str() == key { return Some(node); }
    node.children.iter().find_map(|child| find(child, key))
}

fn text(node: &BuiltNode, result: &mut String) {
    if let Component::Text(props) = &node.component {
        assert!(props.value.0.as_str().len() <= UI_TEXT_MAX_BYTES);
        result.push_str(props.value.0.as_str());
    }
    for child in node.children.iter() {
        text(child, result);
    }
}

fn portable(node: &BuiltNode) -> serde_json::Value {
    serde_json::json!({
        "key": node.key, "component": node.component, "layout": node.layout, "style": node.style,
        "accessibility": node.accessibility, "children": node.children.iter().map(portable).collect::<Vec<_>>(),
    })
}

fn main() {
    let owner = arena::register();
    let emit = std::env::var_os("SEMIO_WINDOW_EXPLANATION_FIXTURE").is_some();
    let mut emitted = false;
    for path in std::env::args().skip(1) {
        let document = Snapshot::parse_dsl(&std::fs::read_to_string(&path).unwrap()).unwrap();
        #[cfg(feature = "authored-host")]
        let host = audited_editor::Host::live(document);
        #[cfg(feature = "authored-host")]
        let report = host.report();
        #[cfg(not(feature = "authored-host"))]
        let report = audited_report::live::report(&document);
        for locale in [Locale::En, Locale::De] {
            let protocol = protocol_locale(locale);
            let mut longest = 0;
            let mut expanded = 0;
            for (index, check) in report.checks.iter().enumerate() {
                let input = check.explanation.resolve(&protocol);
                let input: &str = input.as_ref();
                let panel = render_inspection(&report, Some(index as u32), locale, None)
                    .unwrap_or_else(|error| panic!("{path} {locale:?} {} {error:?}", check.id));
                assert!(node_count(&panel) <= UI_DOCUMENT_NODES);
                let row = find(&panel, "norm-inspection.check.explanation").expect("explanation row");
                let Component::TreeItem(props) = &row.component else { panic!("explanation item") };
                longest = longest.max(input.len());
                if input.len() > UI_TEXT_MAX_BYTES {
                    expanded += 1;
                    assert_eq!(props.default_open, Some(true));
                    let mut actual = String::new();
                    text(row, &mut actual);
                    assert_eq!(actual, input, "the retained row must preserve every byte of the localized explanation");
                    if emit && !emitted {
                        emitted = true;
                        println!("[DEBUG] explanation {}", serde_json::json!({ "panel": portable(&panel), "text": input }));
                    }
                } else {
                    assert_eq!(props.description.as_ref().map(|text| text.as_str()), Some(input));
                }
                drop(panel);
                arena::drain();
            }
            println!("[DEBUG] {path} {locale:?} verified {} checks, {expanded} full-width explanations, max {longest} bytes", report.checks.len());
        }
    }
    arena::close(owner);
}
