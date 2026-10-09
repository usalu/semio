use super::*;
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::{compute_diagnostics, render, row_of, LOCALES};
use crate::ModelSnapshot;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const DEFECTS: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/⚠️diagnostics/💥️defects/📸️snapshot/🔣️.json");
const CLEAN: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/⚠️diagnostics/🏡️clean/📸️snapshot/🔣️.json");
const SOURCE: &str = include_str!("../../../🦀️.rs");

fn snapshot(text: &str) -> ModelSnapshot {
    from_json_str(text, JsonMemberPolicy::Reject).expect("fixture decodes")
}

fn finding(code: DiagnosticCode, elements: &[&str], storey: Option<&str>) -> Diagnostic {
    let finding = Diagnostic::new(code, elements);
    match storey {
        Some(storey) => finding.on(storey),
        None => finding,
    }
}

#[semio_framework_async_macros::async_test]
async fn the_index_of_no_findings_is_the_default() {
    assert_eq!(DiagnosticIndex::of(&[]), DiagnosticIndex::default());
    assert_eq!(DiagnosticIndex::of(&compute_diagnostics(&snapshot(CLEAN))), DiagnosticIndex::default());
}

#[semio_framework_async_macros::async_test]
async fn an_element_is_indexed_under_every_finding_that_names_it_with_its_worst_severity() {
    let found = vec![
        finding(DiagnosticCode::ClashWallColumn, &["c-1", "w-1"], Some("st-1")),
        finding(DiagnosticCode::ClashColumnColumn, &["c-1", "c-2"], Some("st-1")),
        finding(DiagnosticCode::RefWallType, &["w-1"], Some("st-2")),
        finding(DiagnosticCode::StoreyNoDatum, &["bldg"], None),
    ];
    let index = DiagnosticIndex::of(&found);
    assert_eq!(index.elements["c-1"], ElementFindings { severity: Severity::Error, count: 2, codes: vec![DiagnosticCode::ClashWallColumn, DiagnosticCode::ClashColumnColumn] });
    assert_eq!(index.elements["w-1"], ElementFindings { severity: Severity::Error, count: 2, codes: vec![DiagnosticCode::ClashWallColumn, DiagnosticCode::RefWallType] });
    assert_eq!(index.elements["c-2"].severity, Severity::Error);
    assert_eq!(index.elements["bldg"], ElementFindings { severity: Severity::Info, count: 1, codes: vec![DiagnosticCode::StoreyNoDatum] });
    assert_eq!(index.severity_of("w-1"), Some(Severity::Error));
    assert_eq!(index.severity_of("nobody"), None);
}

#[semio_framework_async_macros::async_test]
async fn the_counts_agree_with_the_findings_by_severity_category_code_and_storey() {
    let found = vec![
        finding(DiagnosticCode::ClashWallColumn, &["c-1", "w-1"], Some("st-1")),
        finding(DiagnosticCode::ClashWallColumn, &["c-2", "w-1"], Some("st-1")),
        finding(DiagnosticCode::RefWallType, &["w-2"], Some("st-2")),
        finding(DiagnosticCode::StoreyNoDatum, &["bldg"], None),
    ];
    let index = DiagnosticIndex::of(&found);
    assert_eq!(index.total, SeverityCounts { error: 1, warning: 2, info: 1 });
    assert_eq!(index.total.total(), found.len() as u32);
    assert_eq!(index.categories["clash"], SeverityCounts { error: 0, warning: 2, info: 0 });
    assert_eq!(index.categories["reference"], SeverityCounts { error: 1, warning: 0, info: 0 });
    assert_eq!(index.categories["storey"], SeverityCounts { error: 0, warning: 0, info: 1 });
    assert_eq!(index.codes["clash.wall-column"], 2);
    assert_eq!(index.on_storey("st-1"), SeverityCounts { error: 0, warning: 2, info: 0 });
    assert_eq!(index.on_storey("st-2").worst(), Some(Severity::Error));
    assert_eq!(index.on_storey("st-9"), SeverityCounts::default());
    assert_eq!(index.categories.values().map(SeverityCounts::total).sum::<u32>(), found.len() as u32);
    assert_eq!(index.codes.values().sum::<u32>(), found.len() as u32);
}

#[semio_framework_async_macros::async_test]
async fn the_index_of_the_defect_model_names_every_element_a_finding_names() {
    let found = compute_diagnostics(&snapshot(DEFECTS));
    let index = DiagnosticIndex::of(&found);
    let named: BTreeSet<&String> = found.iter().flat_map(|row| row.elements.iter()).collect();
    assert_eq!(index.elements.keys().collect::<BTreeSet<_>>(), named);
    assert_eq!(index.total.total() as usize, found.len());
    assert_eq!(index.elements["w-into-column"].codes, vec![DiagnosticCode::ClashWallColumn, DiagnosticCode::ClashStairWall]);
    assert_eq!(index.elements["w-missing-type"].codes, vec![DiagnosticCode::RefWallType]);
    assert!(index.elements.values().all(|entry| entry.count as usize >= entry.codes.len() && !entry.codes.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn the_index_is_deterministic_and_independent_of_the_order_of_findings() {
    let found = compute_diagnostics(&snapshot(DEFECTS));
    let mut reversed = found.clone();
    reversed.reverse();
    assert_eq!(DiagnosticIndex::of(&found), DiagnosticIndex::of(&reversed));
    assert_eq!(DiagnosticIndex::of(&found), DiagnosticIndex::of(&found));
}

#[semio_framework_async_macros::async_test]
async fn the_category_is_the_domain_of_the_slug_and_every_category_has_codes() {
    assert_eq!(category_of(DiagnosticCode::ClashWallWall), "clash");
    assert_eq!(category_of(DiagnosticCode::DuplicateId), "reference");
    assert_eq!(category_of(DiagnosticCode::RoofOverhangCollapsed), "roof");
    let categories = categories();
    assert_eq!(categories.iter().collect::<BTreeSet<_>>().len(), categories.len(), "each category once");
    assert!(DiagnosticCode::ALL.iter().all(|code| categories.contains(&category_of(*code)) && row_of(*code).slug.contains('.')));
}

fn variants_in_source() -> Vec<String> {
    let body = SOURCE.split("pub enum DiagnosticCode {").nth(1).and_then(|rest| rest.split("\n}").next()).expect("enum body");
    body.lines().map(|line| line.trim().trim_end_matches(',').to_string()).filter(|line| !line.is_empty()).collect()
}

#[semio_framework_async_macros::async_test]
async fn every_code_is_listed_once_in_all_and_has_a_row_in_both_languages() {
    let declared = variants_in_source();
    let listed: Vec<String> = DiagnosticCode::ALL.iter().map(|code| format!("{code:?}")).collect();
    assert_eq!(listed, declared, "ALL lists every variant in declaration order");
    assert!(DiagnosticCode::ALL.windows(2).all(|pair| pair[0] < pair[1]));
    let mut slugs = BTreeSet::new();
    for code in DiagnosticCode::ALL {
        let row = row_of(*code);
        assert!(slugs.insert(row.slug), "unique slug {}", row.slug);
        for locale in LOCALES {
            let finding = Diagnostic::new(*code, &["e-1"]).lacking("m-1");
            let text = render(&finding, locale).expect("both locales render");
            assert!(!text.trim().is_empty(), "{code:?} {locale}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn a_rendered_text_has_no_open_placeholder_once_its_numbers_are_given() {
    for code in DiagnosticCode::ALL {
        let row = row_of(*code);
        let mut finding = Diagnostic::new(*code, &["e-1", "e-2"]).lacking("m-1");
        for text in [row.en, row.de] {
            for name in text.split('{').skip(1).filter_map(|rest| rest.split('}').next()) {
                if name != "elements" && name != "missing" {
                    finding = finding.with(name, 1.5);
                }
            }
        }
        for locale in LOCALES {
            let text = render(&finding, locale).expect("renders");
            assert!(!text.contains('{') && !text.contains('}'), "{code:?} {locale}: {text}");
        }
    }
}

mod graph {
    use super::*;
    use crate::standards::v1::subsets::any::schema::inferences::model_graph::compute::dependency;
    use crate::standards::v1::subsets::any::schema::inferences::model_graph::{infer_selected, kinds, plan, ModelInferenceSession, ModelNode, NodeKind};
    use crate::{Entry, ModelDiff, ModelInference, WallPatch};
    use protocol::Inference;

    #[semio_framework_async_macros::async_test]
    async fn the_graph_projects_the_index_of_the_ordered_findings() {
        for text in [CLEAN, DEFECTS] {
            let model = snapshot(text);
            let inferred = ModelInference::infer(&model).expect("infers");
            assert_eq!(inferred.diagnostic_index, DiagnosticIndex::of(&inferred.diagnostics));
            assert_eq!(inferred.diagnostic_index.total.total() as usize, inferred.diagnostics.len());
        }
        assert!(!ModelInference::infer(&snapshot(DEFECTS)).expect("infers").diagnostic_index.elements.is_empty());
        assert_eq!(ModelInference::infer(&ModelSnapshot::default()).expect("infers").diagnostic_index, DiagnosticIndex::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn the_index_node_has_every_findings_node_as_parent_and_reads_nothing_else() {
        let model = snapshot(DEFECTS);
        let steps = plan::build(&model, kinds::closure(kinds::DIAGNOSTIC_INDEX));
        let index = steps.iter().find(|step| step.key == ModelNode::DiagnosticIndex).expect("the index node");
        let findings: Vec<&ModelNode> = steps.iter().filter(|step| step.key.kind() == NodeKind::Diagnostics).map(|step| &step.key).collect();
        assert_eq!(index.parents.iter().collect::<Vec<_>>(), findings, "the parents are exactly the findings nodes");
        assert!(steps.iter().position(|step| step.key == ModelNode::DiagnosticIndex) > steps.iter().rposition(|step| step.key.kind() == NodeKind::Diagnostics), "the findings come first");
        assert!(steps.iter().all(|step| !matches!(step.key.kind(), NodeKind::Plan | NodeKind::Quantity | NodeKind::Schedule)), "a selection plans the findings and their ancestors only");
        assert_eq!(dependency(&model, &ModelNode::DiagnosticIndex), dependency(&snapshot(CLEAN), &ModelNode::DiagnosticIndex), "the node reads no snapshot collection of its own");
    }

    #[semio_framework_async_macros::async_test]
    async fn the_selection_of_the_index_projects_it_and_the_selection_of_the_findings_alone_does_not() {
        let model = snapshot(DEFECTS);
        let with_index = infer_selected::<{ kinds::DIAGNOSTIC_INDEX }>(&model);
        assert_eq!(with_index.diagnostic_index, DiagnosticIndex::of(&compute_diagnostics(&model)));
        assert_eq!(infer_selected::<{ kinds::DIAGNOSTICS }>(&model).diagnostic_index, DiagnosticIndex::default(), "an unwanted kind stays empty");
    }

    #[semio_framework_async_macros::async_test]
    async fn the_index_is_cache_transparent_and_a_diff_that_touches_nothing_the_graph_reads_serves_it_gated() {
        let model = snapshot(DEFECTS);
        let uncached = ModelInference::infer(&model).expect("infers").diagnostic_index;
        let mut session = ModelInferenceSession::new();
        let cold = session.update(&model, &ModelDiff::default()).diagnostic_index.clone();
        assert_eq!(session.report().computed_by_kind.get("diagnostic-index"), Some(&1));
        let gated = session.update(&model, &ModelDiff::default()).diagnostic_index.clone();
        assert!(session.report().gated && session.report().computed == 0, "{:?}", session.report());
        let warm = session.refresh(&model).diagnostic_index.clone();
        assert_eq!(session.report().computed, 0, "a refresh of the same model is all cache hits");
        assert!(cold == uncached && gated == uncached && warm == uncached);
    }

    #[semio_framework_async_macros::async_test]
    async fn fixing_a_defect_updates_the_index_to_a_fresh_inference_and_recomputes_it_at_most_once() {
        let model = snapshot(DEFECTS);
        let mut session = ModelInferenceSession::new();
        session.refresh(&model);
        assert!(session.inference().diagnostic_index.elements.contains_key("w-missing-type"));
        let known = model.wall_types.keys().next().expect("a wall type").clone();
        let fix = ModelDiff::walls("w-missing-type", Entry::Patched(WallPatch { wall_type: Some(known), ..Default::default() }));
        let fixed = protocol::apply_diff(&fix, &model).expect("the fix applies");
        let updated = session.update(&fixed, &fix).clone();
        assert!(session.report().computed_by_kind.get("diagnostic-index").copied().unwrap_or(0) <= 1, "{:?}", session.report());
        assert!(updated.diagnostic_index.elements.get("w-missing-type").is_none_or(|entry| !entry.codes.contains(&DiagnosticCode::RefWallType)), "the reference finding is gone from the index");
        assert_eq!(updated.diagnostic_index, DiagnosticIndex::of(&updated.diagnostics));
        assert_eq!(updated, ModelInference::infer(&fixed).expect("infers"));
    }
}
