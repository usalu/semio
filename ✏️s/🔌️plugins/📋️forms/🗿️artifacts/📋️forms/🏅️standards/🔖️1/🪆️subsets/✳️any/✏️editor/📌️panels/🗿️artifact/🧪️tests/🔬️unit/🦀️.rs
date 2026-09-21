use super::*;
use crate::editor::forms::unit_tests::context::{forms_app, render as render_body};
use crate::editor::forms::FORMS_PLAY_BODY_ARTIFACT as BODY_ARTIFACT;

#[semio_framework_async_macros::async_test]
async fn document_tree_declares_drop_action() {
    let mut app = forms_app().await;
    let json = render_body(&mut app, BODY_ARTIFACT).await;
    assert!(json.contains(r#""trigger":"drop""#), "the tree carries a drop-trigger binding: {json}");
    assert!(json.contains("dropQuestionKind"));
    app.close();
}

#[semio_framework_async_macros::async_test]
async fn document_lists_steps() {
    let mut app = forms_app().await;
    let json = render_body(&mut app, BODY_ARTIFACT).await;
    assert!(json.contains("forms-play-document.steps"));
    app.close();
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(FORMS_PLAY_BODY_ARTIFACT));
}

//#region 🪟️WindowLaws
use crate::editor::forms::terminology::forms_play_labels;
use crate::materialize_forms_steps;
use semio_framework_plugin::plugin_app_close_prelude::Component;
use semio_framework_plugin::{TreeWindowRequest, ViewModel, INTERACTION_SELECT_ACTION_ID};
use semio_framework_ui_contract::{TreeWindow, UI_BUILT_CHILDREN_MAX};

fn question(step: usize, index: usize) -> FormQuestion {
    FormQuestion {
        id: format!("q{step:02}-{index:03}"),
        label: format!("Question {step}.{index}"),
        kind: "text".into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}

/// 🧱️ A form an order of magnitude past one viewport at BOTH levels: 120 steps, 120 questions in
/// the first one — the two nesting levels this tree windows independently.
fn oversized() -> FormsSnapshot {
    let steps: Vec<FormStep> = (0..120)
        .map(|step| FormStep {
            id: format!("s{step:02}"),
            title: format!("Step {step}"),
            description: None,
            blocks: (0..if step == 0 { 120 } else { 2 }).map(|index| question(step, index)).collect(),
        })
        .collect();
    let mut snapshot = FormsSnapshot::default();
    materialize_forms_steps(&mut snapshot.structure, steps);
    snapshot
}

fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: FORMS_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
}

/// 🪟️ A nested container's node key is its PATH from the body root — the SDK addresses a step row as
/// `<steps section>␟<step tree id>`, so a request naming the bare row id addresses nothing.
fn step_path(step_id: &str) -> String {
    format!("{FORMS_PLAY_DOCUMENT_STEPS}{}{step_id}", semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR)
}

/// 🪟️ A REALISTIC measured viewport. A large `tree_viewport_rows` is not a harmless over-request: a wide
/// section spends the whole first-paint budget on its own rows, exhausts the shared built-node page pool,
/// and then every nested container — including one the host explicitly asked for — silently builds empty
/// (see 📓️a8-remaining-plugins.md §6). The laws below test the contract, not that failure mode.
const MEASURED_VIEWPORT_ROWS: u32 = 8;

fn viewing(rows: u32, requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, tree_viewport_rows: Some(rows), ..Default::default() }
}

fn build(spec: &FormsSnapshot, view: &ViewModel) -> BuiltNode {
    render(spec, forms_play_labels(&ViewModel::default()), &TreeWindows::for_body(view, FORMS_PLAY_BODY_ARTIFACT)).expect("forms document tree assembly")
}

fn projection(node: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("forms document tree projection")
}

fn steps_section(tree: &BuiltNode) -> &BuiltNode {
    tree.children.iter().find(|child| child.key.as_str() == FORMS_PLAY_DOCUMENT_STEPS).expect("the steps section")
}

fn window_or_empty(node: &BuiltNode) -> Option<TreeWindow> {
    match &node.component {
        Component::TreeSection(props) => props.window,
        Component::TreeItem(props) => props.window,
        _ => panic!("a windowed container is a tree section or a nesting tree item"),
    }
}

fn row_keys(parent: &BuiltNode) -> Vec<&str> {
    parent.children.iter().map(|row| row.key.as_str()).collect()
}

/// 🪟️ Law (a): a form past one viewport streams at BOTH levels — the steps section and every
/// materialised step row stamp their FULL extent, build no more than their slice, and the body never
/// invents a `+N` row to stand in for the remainder.
#[semio_framework_async_macros::async_test]
async fn an_oversized_form_stamps_every_extent_and_materialises_one_viewport() {
    let spec = oversized();
    let tree = build(&spec, &viewing(48, Vec::new()));
    let steps = steps_section(&tree);
    assert_eq!(window_or_empty(steps).expect("the steps section stamps its window").total, 120, "120 steps are announced whole");
    assert!(steps.children.len() < 120, "a first paint materialises one viewport, not every step");
    assert!(steps.children.len() <= UI_BUILT_CHILDREN_MAX, "the section stays inside one built page");
    let first = steps.children.iter().next().expect("a step row");
    assert_eq!(first.key.as_str(), "step:s00", "a step row is keyed by its canonical tree id");
    assert_eq!(window_or_empty(first).expect("a step row stamps its own window").total, 120, "a step row announces every question it owns");
    assert!(first.children.len() < 120, "the nested window is one viewport too");
    let materialised: usize = steps.children.iter().map(|step| 1 + step.children.len()).sum();
    assert!(materialised <= 48, "one shared first-paint budget spans BOTH levels: {materialised}");
    let json = projection(tree);
    assert!(!json.contains(".more"), "a windowed container never mints a continuation key: {json}");
    assert!(!json.contains("\"+"), "a windowed container never mints a `+N` label: {json}");
}

/// 🪟️ Law (b): a container the user closed is announced, not built — at either level.
#[semio_framework_async_macros::async_test]
async fn a_closed_container_stamps_its_extent_and_builds_no_child() {
    let spec = oversized();
    let closed_step = build(&spec, &viewing(512, vec![request(&step_path("step:s00"), Some(false), 0, 48)]));
    let first = steps_section(&closed_step).children.iter().next().expect("a step row");
    assert_eq!(window_or_empty(first).expect("window").total, 120, "a closed step row still announces its questions");
    assert!(first.children.is_empty(), "a closed step row builds no question");

    let closed_section = build(&spec, &viewing(512, vec![request(FORMS_PLAY_DOCUMENT_STEPS, Some(false), 0, 48)]));
    let steps = steps_section(&closed_section);
    assert_eq!(window_or_empty(steps).expect("window").total, 120, "a closed section still announces its extent");
    assert!(steps.children.is_empty(), "a closed section builds no row");
}

/// 🪟️ Law (c): the host names `{offset, rows}` and the guest materialises exactly that half-open
/// range — the same law one level down, keyed by the raw question id.
#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_own_range() {
    let spec = oversized();
    let tree = build(&spec, &viewing(MEASURED_VIEWPORT_ROWS, vec![request(FORMS_PLAY_DOCUMENT_STEPS, None, 10, 4), request(&step_path("step:s10"), None, 0, 0)]));
    let steps = steps_section(&tree);
    assert_eq!(window_or_empty(steps), Some(TreeWindow { total: 120, offset: 10 }));
    let expected_steps: Vec<String> = (10..14).map(|index| format!("step:s{index:02}")).collect();
    assert_eq!(row_keys(steps), expected_steps.iter().map(String::as_str).collect::<Vec<_>>(), "exactly steps [10, 14) keyed by their canonical tree ids");

    let nested = build(&spec, &viewing(MEASURED_VIEWPORT_ROWS, vec![request(&step_path("step:s00"), None, 50, 6)]));
    let first = steps_section(&nested).children.iter().next().expect("a step row");
    assert_eq!(window_or_empty(first), Some(TreeWindow { total: 120, offset: 50 }));
    let expected_questions: Vec<String> = (50..56).map(|index| format!("q00-{index:03}")).collect();
    assert_eq!(row_keys(first), expected_questions.iter().map(String::as_str).collect::<Vec<_>>(), "a nested window is the same law one level down, keyed by the raw question id");
}

/// 🪟️ Law (d): a domain-bound tree declares its picks on the rows and binds exactly once at the
/// root — steps at `"section"`, questions at `"field"`, both still draggable, neither binding an
/// action of its own. The tree's `dropAction` is not a row binding and survives.
#[semio_framework_async_macros::async_test]
async fn rows_declare_their_granularity_while_the_tree_binds_the_one_interaction_select() {
    let spec = oversized();
    let tree = build(&spec, &viewing(MEASURED_VIEWPORT_ROWS, vec![request(FORMS_PLAY_DOCUMENT_STEPS, None, 0, 2), request(&step_path("step:s00"), None, 0, 2)]));
    let Component::Tree(props) = &tree.component else { panic!("panel tree") };
    assert_eq!(props.interaction_domain.as_ref().map(|domain| domain.as_str()), Some(FORMS_INTERACTION_FIELDS));
    let binding = tree.bindings.iter().next().expect("the tree binds the domain select");
    assert_eq!(binding.action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    assert_eq!(binding.action.scope.as_str(), FORMS_PLAY_APP_ID);
    assert_eq!(tree.bindings.iter().filter(|binding| binding.action.name.as_str() == INTERACTION_SELECT_ACTION_ID).count(), 1, "exactly one tree-level select, never one per row");

    let step = steps_section(&tree).children.iter().next().expect("a step row");
    assert!(step.bindings.iter().next().is_none(), "a step row binds no action of its own");
    let Component::TreeItem(step_props) = &step.component else { panic!("tree item") };
    assert_eq!(step_props.granularity.as_ref().map(|text| text.as_str()), Some(FORMS_INTERACTION_GRANULARITY_SECTION));
    assert_eq!(step_props.draggable, Some(true), "a step stays draggable");
    assert_eq!(step_props.icon.as_ref().map(|icon| icon.as_str()), Some("list-tree"));

    let question = step.children.iter().next().expect("a question row");
    assert!(question.bindings.iter().next().is_none(), "a question row binds no action of its own");
    let Component::TreeItem(question_props) = &question.component else { panic!("tree item") };
    assert_eq!(question_props.granularity.as_ref().map(|text| text.as_str()), Some(FORMS_INTERACTION_GRANULARITY_FIELD));
    assert_eq!(question_props.draggable, Some(true), "a question stays draggable");
    assert_eq!(question_props.icon.as_ref().map(|icon| icon.as_str()), Some("help-circle"));

    let json = projection(tree);
    assert_eq!(json.matches(INTERACTION_SELECT_ACTION_ID).count(), 1, "the select is authored once for the whole tree: {json}");
    assert!(json.contains("dropQuestionKind"), "the tree keeps its drop action: {json}");
}
//#endregion 🪟️WindowLaws
