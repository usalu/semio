use super::*;
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};

fn unhosted() -> String {
    crate::editor::architect::unit_tests::context::project_render(render(&TreeWindows::unhosted()))
}

/// 🪟️ The catalogue body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, ..Default::default() };
    crate::editor::architect::unit_tests::context::project_render(render(&TreeWindows::for_body(&view, ARCHITECT_BODY_CATALOGUE)))
}

fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: ARCHITECT_BODY_CATALOGUE.into(), node_key: node_key.into(), open, offset, rows }
}

#[semio_framework_async_macros::async_test]
async fn the_tab_is_the_framework_catalogue_tab_bound_to_this_apps_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key.as_deref(), Some(ARCHITECT_BODY_CATALOGUE));
    assert!(matches!(definition.group, PanelGroup::Workbench));
}

#[semio_framework_async_macros::async_test]
async fn the_action_shortcuts_are_present() {
    let json = unhosted();
    for id in ["architect-catalogue.validate", "architect-catalogue.analysis", "architect-catalogue.report", "architect-catalogue.search"] {
        assert!(json.contains(id), "missing shortcut {id}");
    }
}

//#region 🪟️WindowLaws
/// 🪟️ Law (a): the 66-entry register roster is ONE section stamping its full extent — the
/// `.chunks(UI_FIXED_LIST_ITEMS)` "Registers 1–32"/"33–64"/"65–66" idiom is gone, and a first paint
/// materialises at most one viewport with no continuation row.
#[test]
fn the_register_roster_is_one_section_that_stamps_its_full_extent() {
    let json = unhosted();
    assert!(json.contains(&format!("\"total\":{}", REGISTER_IDS.len())), "the registers section stamps its full extent: {json}");
    assert!(json.contains(&format!("\"total\":{}", SHORTCUTS.len())), "the actions section stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(!json.contains("architect-catalogue.registers.0"), "the chunked-into-pages register sections are gone: {json}");
    assert!(json.contains("architect-catalogue.registers\""), "one register section spans the whole roster: {json}");
    assert!(json.matches("architect-catalogue.register.").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "first paint materialises about one viewport: {json}");
}

/// 🪟️ Law (b): a container the host closed stamps its total and materialises nothing.
#[test]
fn a_closed_register_section_stamps_its_total_and_materialises_no_children() {
    let json = window_body(vec![request("architect-catalogue.registers", Some(false), 0, 0)]);
    assert!(json.contains(&format!("\"total\":{}", REGISTER_IDS.len())), "a closed section still stamps its extent: {json}");
    assert!(!json.contains("architect-catalogue.register."), "a closed section materialises no rows: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)` of the roster — including
/// the tail the old 32-slot chunking could only reach through a "Registers 65–66" section.
#[test]
fn a_host_window_materialises_exactly_its_slice_of_the_roster() {
    let offset = REGISTER_IDS.len() - 6;
    let json = window_body(vec![request("architect-catalogue.registers", Some(true), offset as u32, 4)]);
    assert!(json.contains(&format!("\"offset\":{offset}")), "the section reports its offset: {json}");
    for register in &REGISTER_IDS[offset..offset + 4] {
        assert!(json.contains(&format!("architect-catalogue.register.{register}")), "row {register} is inside the window: {json}");
    }
    for register in &REGISTER_IDS[..offset] {
        assert!(!json.contains(&format!("architect-catalogue.register.{register}\"")), "row {register} stays out of the window: {json}");
    }
}

/// 🪟️ Law (d), unbound-tree variant: the catalogue is deliberately NOT a pick surface — it declares no
/// interaction domain, stamps no granularity, and every row keeps its own app action.
#[test]
fn the_catalogue_tree_binds_no_domain_and_every_row_keeps_its_own_action() {
    let json = unhosted();
    assert!(!json.contains("interactionDomain"), "the catalogue declares no interaction domain: {json}");
    assert!(!json.contains("interactionSelect"), "the catalogue stamps no tree-level pick binding: {json}");
    assert!(!json.contains("\"granularity\""), "catalogue rows are not pick targets: {json}");
    assert!(json.contains("selectRegister"), "register rows keep their own action: {json}");
    assert!(json.contains("runValidation"), "shortcut rows keep their own action: {json}");
}
//#endregion 🪟️WindowLaws
