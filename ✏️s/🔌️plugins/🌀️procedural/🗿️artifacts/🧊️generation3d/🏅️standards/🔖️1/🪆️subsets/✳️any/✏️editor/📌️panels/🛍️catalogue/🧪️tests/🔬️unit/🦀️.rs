use super::*;
use crate::editor::generation3d::unit_tests::context::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn generation3d_labels_resolve_native_english_by_default() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("\"Widgets\""));
    assert!(!json.contains("Elemente"));
}

/// ⚖️ LAW: the panel accounts for EVERY registered operator — the rows it materialised plus the
/// count it publishes as omitted are the whole `flow_palette_catalogue_sections()` roster. Before
/// this, a section the row budget could not open at all vanished with no count anywhere, so the
/// panel understated the catalogue by an unbounded amount
/// (`📓️audit-user-journey-gaps-2026-09-13.md` §9 item 11).
#[test]
fn the_catalogue_panel_accounts_for_every_registered_operator() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let roster: usize = semio_framework_os_flow::flow_palette_catalogue_sections().iter().map(|section| section.items.len()).sum();
    let page = catalogue_page();
    assert!(roster > 0, "the flow palette registers at least one operator");
    assert_eq!(page.shown + page.omitted, roster, "every registered operator is either a row or part of the omitted count");
}

/// ⚖️ LAW: nothing the panel omits is unreachable. The unbounded browse surface is the canvas
/// spotlight, which reads the app-static `flow_app_catalogue` published once per app instance on the
/// reserved `framework.section.catalogue` surface — so that payload must carry the WHOLE roster the
/// panel pages over, not the page.
#[test]
fn the_spotlight_catalogue_offers_every_operator_the_panel_omits() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    let spotlight = semio_framework_os_flow::flow_app_catalogue();
    let spotlight_rows: usize = spotlight.sections.iter().map(|section| section.items.len()).sum();
    let roster: usize = sections.iter().map(|section| section.items.len()).sum();
    assert_eq!(spotlight_rows, roster, "the spotlight catalogue is the whole roster, never the panel's page");
    assert!(catalogue_page().omitted <= roster, "the omitted count cannot exceed the roster");
}

/// ⚖️ LAW: the panel states its omission. With the real operator sets installed the 31-row page
/// cannot hold the roster, and the reader has to be told so — the continuation row's label is the
/// omitted count itself (`+n`), which carries the same meaning on every locale × terminology axis.
#[semio_framework_async_macros::async_test]
async fn the_catalogue_panel_publishes_its_omitted_count() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let page = catalogue_page();
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_CATALOGUE).await;
    if page.omitted > 0 {
        assert!(json.contains(&format!("+{}", page.omitted)), "the catalogue panel must name its {} omitted operators: {json}", page.omitted);
    }
}
