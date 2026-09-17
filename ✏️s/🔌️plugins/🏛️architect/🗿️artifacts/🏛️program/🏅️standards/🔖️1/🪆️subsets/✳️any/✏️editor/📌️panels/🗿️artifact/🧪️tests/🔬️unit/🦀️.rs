use super::*;
use crate::{empty_plugin, sample_plugin};
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

fn unhosted(program: &ProgramSnapshot) -> String {
    crate::editor::architect::unit_tests::context::project_render(render(program, &TreeWindows::unhosted()))
}

#[semio_framework_async_macros::async_test]
async fn the_tab_is_the_framework_document_tab_bound_to_this_apps_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key.as_deref(), Some(ARCHITECT_BODY_ARTIFACT));
    assert!(matches!(definition.group, PanelGroup::Workbench));
}

#[semio_framework_async_macros::async_test]
async fn the_tree_lists_program_meta_and_the_elements() {
    let program = sample_plugin();
    let json = unhosted(&program);
    assert!(json.contains("Sample Clinic"));
    assert!(json.contains(&program.elements[0].header.id.to_string()));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the document panel binds the
/// "program" interaction domain — the framework owns selection over its granularity-marked element
/// rows (see `render`'s own doc comment).
#[semio_framework_async_macros::async_test]
async fn the_tree_binds_the_program_interaction_domain() {
    let json = unhosted(&sample_plugin());
    assert!(json.contains("\"interactionDomain\":\"program\""));
}

#[semio_framework_async_macros::async_test]
async fn an_empty_program_renders_the_none_placeholder_row() {
    let json = unhosted(&empty_plugin());
    assert!(json.contains("architect-document.elements.empty"));
}

//#region 🪟️WindowLaws
/// 🪟️ A program an order of magnitude past one viewport — the subject of every window law below. The
/// elements register is the unbounded one (`elements: Vec<ProgramElement>`), so the sample element is
/// re-keyed `count` times rather than hand-built: `ProgramElement` has no `Default`.
fn oversized_program(count: usize) -> ProgramSnapshot {
    let mut program = sample_plugin();
    let seed = program.elements.first().cloned().expect("the sample program carries an element to scale");
    program.elements = (0..count)
        .map(|index| {
            let mut element = seed.clone();
            element.header.id = crate::kernel::EntityId(format!("element-{index}"));
            element.header.name = format!("Element {index}");
            element.parent_id = None;
            element
        })
        .collect();
    program
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(program: &ProgramSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    crate::editor::architect::unit_tests::context::project_render(render(program, &TreeWindows::for_body(&view, ARCHITECT_BODY_ARTIFACT)))
}

fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: ARCHITECT_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
}

/// 🪟️ Law (a): every container stamps its FULL extent and materialises at most one viewport — no `+N`,
/// and no `.chunks(UI_FIXED_LIST_ITEMS)` "Registers 1–32"/"33–64" section splitting either.
#[test]
fn an_oversized_program_stamps_totals_and_never_a_continuation_row() {
    let program = oversized_program(300);
    let json = window_body(&program, Vec::new());
    assert!(json.contains("\"total\":300"), "the elements section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(!json.contains("architect-document.registers.0"), "the chunked-into-pages register sections are gone: {json}");
    assert!(json.contains("architect-document.registers\""), "one register section spans the whole roster: {json}");
    assert!(json.matches("\"key\":\"element-").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a container the host closed stamps its total and materialises nothing.
#[test]
fn a_closed_section_stamps_its_total_and_materialises_no_children() {
    let program = oversized_program(300);
    let json = window_body(&program, vec![request("architect-document.elements", Some(false), 0, 0)]);
    assert!(json.contains("\"total\":300"), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("\"key\":\"element-"), "a closed section materialises no rows: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw entity id.
#[test]
fn a_host_window_materialises_exactly_its_slice() {
    let program = oversized_program(300);
    let json = window_body(&program, vec![request("architect-document.elements", Some(true), 120, 10)]);
    assert!(json.contains("\"offset\":120"), "the section reports its offset: {json}");
    for index in 120..130 {
        assert!(json.contains(&format!("\"key\":\"element-{index}\"")), "row {index} is inside the window: {json}");
    }
    assert!(!json.contains("\"key\":\"element-119\""), "the row before the window stays out: {json}");
    assert!(!json.contains("\"key\":\"element-130\""), "the row after the window stays out: {json}");
}

/// 🪟️ Law (d): element rows are pick rows — they carry the domain's granularity and NO binding of
/// their own, while the tree root carries exactly one `interactionSelect`. Register rows are NOT pick
/// rows: switching the active register is unrelated to entity selection, so they keep `selectRegister`.
#[test]
fn pick_rows_carry_granularity_while_the_tree_carries_the_one_interaction_select() {
    let json = window_body(&oversized_program(4), vec![request("architect-document.registers", Some(true), 0, 4)]);
    assert_eq!(json.matches("interactionSelect").count(), 1, "exactly one tree-level interactionSelect binding: {json}");
    assert!(json.contains("\"granularity\":\"entity\""), "element rows are marked as pick targets: {json}");
    assert!(json.contains("selectRegister"), "register rows keep their own action: {json}");
}

/// 🪟️ The 66-row register summary is authored CLOSED so it cannot swallow the shared first-paint
/// budget: it stamps its extent, materialises nothing, and the elements section — this panel's subject
/// — gets the viewport.
#[test]
fn the_register_summary_is_closed_on_first_paint_so_elements_get_the_viewport() {
    let json = window_body(&oversized_program(300), Vec::new());
    assert!(!json.contains("architect-document.register."), "no register row materialises on a cold paint: {json}");
    assert!(json.contains("\"key\":\"element-0\""), "the elements section gets the first-paint budget: {json}");
}
//#endregion 🪟️WindowLaws
