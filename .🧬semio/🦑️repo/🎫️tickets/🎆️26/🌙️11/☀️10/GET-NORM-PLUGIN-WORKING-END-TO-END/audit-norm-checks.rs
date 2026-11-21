use audited_inspection::{plan_checks, render_checks_with, ChecksActions, ChecksSort, ChecksViewState, ChecksWindowSpec, NORM_CHECKS_SECTION_ID, NORM_CHECKS_SUMMARY_ID, NORM_CHECKS_TREE_ID};
#[cfg(feature = "en1990")]
use audited_model::En1990Snapshot as Snapshot;
#[cfg(feature = "en1993")]
use audited_model::En1993Snapshot as Snapshot;
#[cfg(feature = "en1994")]
use audited_model::En1994Snapshot as Snapshot;
#[cfg(not(feature = "synthetic"))]
use pack::snapshot::ArtifactDsl;
use semio_plugin::{BuiltNode, TreeWindowRequest, TreeWindows, ViewModel};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, Remedy, SubjectRef};
use semio_ui_contract::{UI_RESIDENT_SURFACE_ITEMS, UI_DOCUMENT_NODES};
use semio_ui_locale::{Locale, Terminology};
use serde_json::{json, Value};

#[path = "eurocode-ui-arena.rs"]
mod arena;

fn portable(node: &BuiltNode) -> Value {
    json!({ "key": node.key, "component": node.component, "children": node.children.iter().map(portable).collect::<Vec<_>>() })
}

fn json_nodes(node: &Value) -> usize {
    1 + node["children"].as_array().unwrap().iter().map(json_nodes).sum::<usize>()
}

fn synthetic(distinct_parts: bool) -> CheckReport {
    let mut report = CheckReport::default();
    for index in 0..48 {
        let part = if distinct_parts { format!("Part {index:02}") } else { "ULS".into() };
        let subject = SubjectRef::new("B1", "members[B1].tM", LocalizedCopy::new("Member B1", "Stab B1"));
        report.push(CheckResult::assess(format!("test-{index}"), part, ClauseId::new("EN 1990", "1", "6.2.10"), subject.clone(), LocalizedCopy::new(format!("Check {index}"), format!("Nachweis {index}")))
            .explanation(LocalizedCopy::new("Failure explanation. ".repeat(33), "Begründung. ".repeat(60)))
            .annex(AnnexChoice::De)
            .remedy(Remedy::at_least(subject, Quantity::length_m(0.012), Quantity::length_m(0.022), LocalizedCopy::new("Increase thickness", "Dicke erhöhen")))
            .status(CheckStatus::Fail)
            .build());
    }
    report
}

fn audit(name: &str, report: &CheckReport, locale: Locale, controls: bool) {
    let state = ChecksViewState { sort: ChecksSort::ReportOrder, ..ChecksViewState::default() };
    let spec = ChecksWindowSpec { controller_id: Some("s.norm-eurocode.en1990@1/*#editor"), actions: ChecksActions { view_controls: controls, ..ChecksActions::default() }, ..ChecksWindowSpec::default() };
    let groups = plan_checks(report, &state, locale);
    let logical = groups.iter().enumerate().flat_map(|(group_index, group)| {
        std::iter::once(format!("norm-checks-group-{group_index}")).chain(group.rows.iter().map(|index| format!("norm-checks-check-{index}")))
    }).collect::<Vec<_>>();
    assert!(!logical.is_empty(), "model fixture generated no checks");
    let mut offset = 0;
    let mut pages = 0;
    let mut max_nodes = 0;
    let mut max_items = 0;
    let mut project = |section: &str, offset: Option<usize>, total: usize| {
        let body = "norm-en1990-checks";
        let mut view = ViewModel::new(locale, Terminology::Native);
        if let Some(offset) = offset {
            view.tree_windows.push(TreeWindowRequest {
                body_key: body.into(), node_key: section.into(), open: Some(true),
                offset: offset as u32, rows: (total - offset).min(32) as u32,
            });
        }
        let node = render_checks_with(report, &state, &spec, &TreeWindows::for_body(&view, body), locale).expect("Checks window");
        let items = semio_ui_runtime::surface_subtree_items(&node).expect("native reconciler items");
        let json = serde_json::to_string(&portable(&node)).expect("serialize UI tree");
        drop(node);
        arena::drain();
        let document: Value = serde_json::from_str(&json).expect("independent JSON projection");
        let nodes = json_nodes(&document);
        println!("[DEBUG] {name} {locale:?} controls={controls} {section} offset={offset:?} nodes={nodes} items={items}");
        assert!(nodes <= UI_DOCUMENT_NODES && items <= UI_RESIDENT_SURFACE_ITEMS, "actual document budget");
        max_nodes = max_nodes.max(nodes);
        max_items = max_items.max(items);
        document["children"].as_array().unwrap().iter().find(|node| node["key"] == NORM_CHECKS_TREE_ID).unwrap()["children"].as_array().unwrap().iter()
            .find(|node| node["key"] == section).unwrap()["children"].as_array().unwrap().iter()
            .map(|node| {
                if name == "expanded-48" && node["key"].as_str().unwrap().contains("-check-") {
                    let row = node.to_string();
                    assert!(row.contains(if locale == Locale::En { "Failure explanation." } else { "Begründung." }));
                    assert!(row.contains(if locale == Locale::En { "Increase thickness" } else { "Dicke erhöhen" }));
                }
                node["key"].as_str().unwrap().to_owned()
            }).collect::<Vec<_>>()
    };
    let initial = project(NORM_CHECKS_SUMMARY_ID, None, 0);
    assert_eq!(initial.first().map(String::as_str), Some("norm-checks.summary.verdict"));
    while offset < logical.len() {
        let actual = project(NORM_CHECKS_SECTION_ID, Some(offset), logical.len());
        assert!(!actual.is_empty(), "no zero-width requested slice");
        assert_eq!(actual, logical[offset..offset + actual.len()]);
        offset += actual.len();
        pages += 1;
    }
    let controls_count = if controls { 11 } else { 0 };
    let summary_count = 1 + controls_count + report.summary.parts.len();
    let mut summary_offset = 0;
    let mut parts = Vec::new();
    while summary_offset < summary_count {
        let actual = project(NORM_CHECKS_SUMMARY_ID, Some(summary_offset), summary_count);
        assert!(!actual.is_empty(), "no zero-width summary slice");
        summary_offset += actual.len();
        parts.extend(actual.into_iter().filter(|key| key.starts_with("norm-checks.summary.part-")));
    }
    let expected_parts = (0..report.summary.parts.len()).map(|index| format!("norm-checks.summary.part-{index}")).collect::<Vec<_>>();
    assert_eq!(parts, expected_parts);
    println!("{name} {locale:?} controls={controls} {offset}/{} rows, {summary_offset}/{summary_count} summary rows ({} parts), {pages} pages, max {max_nodes} nodes and {max_items} items", logical.len(), parts.len());
}

fn main() {
    let owner = arena::register();
    let expanded = synthetic(false);
    let summary = synthetic(true);
    for locale in [Locale::En, Locale::De] {
        audit("expanded-48", &expanded, locale, true);
        audit("summary-48", &summary, locale, true);
    }
    #[cfg(not(feature = "synthetic"))]
    for path in std::env::args().skip(1) {
        let snapshot = Snapshot::parse_dsl(&std::fs::read_to_string(&path).expect("asset")).expect("snapshot");
        let report = audited_report::live::report(&snapshot);
        for locale in [Locale::En, Locale::De] {
            let example = std::path::Path::new(&path).parent().unwrap().file_name().unwrap().to_str().unwrap();
            for controls in [false, true] {
                audit(example, &report, locale, controls);
            }
        }
    }
    arena::close(owner);
}
