use super::*;
use crate::editor::layout::unit_tests::context::{layout_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_sample_pages() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_ARTIFACT).await;
    assert!(json.contains("layout-document.page.page-1"));
    assert!(json.contains("Page 1"));
}

#[semio_framework_async_macros::async_test]
async fn document_tree_has_nine_sections() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_ARTIFACT).await;
    for section_id in
        ["layout-document.document", "layout-document.spreads", "layout-document.pages", "layout-document.frames", "layout-document.parentPages", "layout-document.layers", "layout-document.stories", "layout-document.links", "layout-document.styles"]
    {
        assert!(json.contains(section_id), "missing section {section_id}");
    }
}

#[semio_framework_async_macros::async_test]
async fn layout_labels_resolve_native_english_by_default() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_ARTIFACT).await;
    assert!(json.contains("\"Frames\""));
    assert!(json.contains("\"Layers\""));
    assert!(!json.contains("Rahmen"));
}

#[semio_framework_async_macros::async_test]
async fn layout_labels_translate_document_tree_in_german() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_ARTIFACT).await;
    assert!(json.contains("\"Rahmen\""));
    assert!(json.contains("\"Ebenen\""));
    assert!(!json.contains("\"Frames\""));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_ARTIFACT));
}

//#region 🪟️WindowLaws
use crate::editor::layout::terminology::layout_labels;
use crate::{ImageLink, Layer, ParagraphStyle, TextStory};
use semio_framework_plugin::plugin_app_close_prelude::{BuiltNode, Component};
use semio_framework_plugin::{TreeWindowRequest, ViewModel, INTERACTION_SELECT_ACTION_ID};
use semio_framework_ui_contract::{TreeWindow, UI_BUILT_CHILDREN_MAX};

/// 🪟️ Re-keys a cloned frame — `Frame` carries its id inside every variant, and an oversized
/// fixture needs hundreds of distinct ones.
fn frame_with_id(frame: &Frame, id: String) -> Frame {
    let mut clone = frame.clone();
    match &mut clone {
        Frame::Rect { id: slot, .. } | Frame::Text { id: slot, .. } | Frame::Image { id: slot, .. } => *slot = id,
    }
    clone
}

/// 🧱️ A real print document an order of magnitude past one viewport: 300 frames on the first page,
/// 120 links, 150 paragraph styles, 90 stories and 60 layers — the five genuinely open-ended
/// collections of this tree.
fn oversized() -> LayoutSnapshot {
    let mut document = crate::standards::v1::subsets::any::schema::default_document();
    let page = document.pages.first_mut().expect("the demo document has a page");
    let seed_frame = page.frames.first().cloned().expect("the demo page has a frame");
    page.frames = (0..300).map(|index| frame_with_id(&seed_frame, format!("frame-{index:03}"))).collect();
    let seed_layer = page.layers.first().cloned().expect("the demo page has a layer");
    page.layers = (0..60).map(|index| Layer { id: format!("layer-{index:03}"), ..seed_layer.clone() }).collect();
    let seed_link = document.links.first().cloned().expect("the demo document has a link");
    document.links = (0..120).map(|index| ImageLink { id: format!("link-{index:03}"), ..seed_link.clone() }).collect();
    let seed_story = document.stories.first().cloned().expect("the demo document has a story");
    document.stories = (0..90).map(|index| TextStory { id: format!("story-{index:03}"), ..seed_story.clone() }).collect();
    let seed_style = document.paragraph_styles.first().cloned().expect("the demo document has a paragraph style");
    document.paragraph_styles = (0..150).map(|index| ParagraphStyle { id: format!("style-{index:03}"), ..seed_style.clone() }).collect();
    document
}

/// 🪟️ The extent every one of the nine sections must stamp, in authored order.
fn section_totals(document: &LayoutSnapshot) -> [usize; 9] {
    [
        1,
        document.spreads.len(),
        document.pages.len(),
        document.pages.iter().map(|page| page.frames.len()).sum(),
        document.parent_pages.len(),
        document.pages.iter().map(|page| page.layers.len()).sum(),
        document.stories.len(),
        document.links.len(),
        document.paragraph_styles.len() + document.character_styles.len(),
    ]
}

fn frame_ids(document: &LayoutSnapshot) -> Vec<String> {
    document.pages.iter().flat_map(|page| page.frames.iter().map(|frame| frame.id().to_string())).collect()
}

fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: LAYOUT_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
}

fn viewing(rows: u32, requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, tree_viewport_rows: Some(rows), ..Default::default() }
}

fn build(document: &LayoutSnapshot, view: &ViewModel) -> BuiltNode {
    render(document, &LayoutWindowConfig::default(), layout_labels(&ViewModel::default()), &TreeWindows::for_body(view, LAYOUT_PLAY_BODY_ARTIFACT)).expect("layout document tree assembly")
}

fn projection(node: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("layout document tree projection")
}

fn section_node<'a>(tree: &'a BuiltNode, id: &str) -> &'a BuiltNode {
    tree.children.iter().find(|child| child.key.as_str() == id).unwrap_or_else(|| panic!("section {id}"))
}

fn window_or_empty(node: &BuiltNode) -> Option<TreeWindow> {
    match &node.component {
        Component::TreeSection(props) => props.window,
        Component::TreeItem(props) => props.window,
        _ => panic!("a windowed container is a tree section or a nesting tree item"),
    }
}

fn extent_of(node: &BuiltNode) -> usize {
    window_or_empty(node).map_or(0, |window| window.total as usize)
}

fn row_keys(parent: &BuiltNode) -> Vec<&str> {
    parent.children.iter().map(|row| row.key.as_str()).collect()
}

/// 🪟️ Law (a): a print document past one viewport streams instead of truncating — every one of the
/// nine containers stamps the FULL extent of its own list, materialises no more than the slice it
/// was given, and the body never invents a `+N` row to stand in for the remainder.
#[semio_framework_async_macros::async_test]
async fn an_oversized_document_stamps_every_extent_and_materialises_one_viewport() {
    let document = oversized();
    let tree = build(&document, &viewing(48, Vec::new()));
    assert_eq!(tree.children.len(), LAYOUT_DOCUMENT_SECTIONS.len(), "nine sections, every one of them windowed");
    for (id, total) in LAYOUT_DOCUMENT_SECTIONS.into_iter().zip(section_totals(&document)) {
        let node = section_node(&tree, id);
        assert_eq!(extent_of(node), total, "section {id} stamps the full extent of its list");
        assert!(node.children.len() <= total.max(1), "section {id} materialises no more than it has");
        assert!(node.children.len() <= UI_BUILT_CHILDREN_MAX, "section {id} stays inside one built page");
    }
    let frames = section_node(&tree, LAYOUT_DOCUMENT_SECTIONS[3]);
    assert_eq!(extent_of(frames), 300, "300 frames are announced whole");
    assert!(frames.children.len() < 300, "a first paint materialises one viewport, not the whole list");
    let materialised: usize = tree.children.iter().map(|section| section.children.len()).sum();
    assert!(materialised <= 48, "the measured viewport is the whole first-paint budget: {materialised}");
    let json = projection(tree);
    assert!(!json.contains(".more"), "a windowed container never mints a continuation key: {json}");
    assert!(!json.contains("\"+"), "a windowed container never mints a `+N` label: {json}");
}

/// 🪟️ Law (b): a container the user closed is announced, not built — the extent is stamped so the
/// host can size the disclosure, and not one child row is materialised.
#[semio_framework_async_macros::async_test]
async fn a_closed_section_stamps_its_extent_and_builds_no_child() {
    let document = oversized();
    let tree = build(&document, &viewing(512, vec![request(LAYOUT_DOCUMENT_SECTIONS[3], Some(false), 0, 48)]));
    let frames = section_node(&tree, LAYOUT_DOCUMENT_SECTIONS[3]);
    assert_eq!(extent_of(frames), 300, "a closed section still announces its extent");
    assert!(frames.children.is_empty(), "a closed section builds no row");
}

/// 🪟️ Law (c): scrolling is a request, not a page — the host names `{offset, rows}` and the guest
/// materialises exactly that half-open range, still keyed by the BARE frame id.
#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_own_range() {
    let document = oversized();
    let tree = build(&document, &viewing(48, vec![request(LAYOUT_DOCUMENT_SECTIONS[3], None, 100, 12), request(LAYOUT_DOCUMENT_SECTIONS[8], Some(true), 40, 6)]));
    let frames = section_node(&tree, LAYOUT_DOCUMENT_SECTIONS[3]);
    assert_eq!(window_or_empty(frames), Some(TreeWindow { total: 300, offset: 100 }));
    let expected: Vec<String> = frame_ids(&document)[100..112].to_vec();
    assert_eq!(row_keys(frames), expected.iter().map(String::as_str).collect::<Vec<_>>(), "exactly frames [100, 112) keyed by their raw ids");
    let styles = section_node(&tree, LAYOUT_DOCUMENT_SECTIONS[8]);
    assert_eq!(window_or_empty(styles), Some(TreeWindow { total: section_totals(&document)[8] as u32, offset: 40 }));
    assert_eq!(styles.children.len(), 6, "an authored-closed section opens on the host's word and honours its window");
}

/// 🪟️ Law (d): a domain-bound tree declares its picks on the rows and binds exactly once at the
/// root — a frame row carries `granularity` and nothing else, while rows with a real app action of
/// their own (a page's `setActivePage`) keep it.
#[semio_framework_async_macros::async_test]
async fn frame_rows_declare_their_granularity_while_the_tree_binds_the_one_interaction_select() {
    let document = oversized();
    let tree = build(&document, &viewing(512, Vec::new()));
    let Component::Tree(props) = &tree.component else { panic!("panel tree") };
    assert_eq!(props.interaction_domain.as_ref().map(|domain| domain.as_str()), Some(LAYOUT_INTERACTION_ELEMENTS));
    let binding = tree.bindings.iter().next().expect("the tree binds the domain select");
    assert_eq!(binding.action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    assert_eq!(binding.action.scope.as_str(), LAYOUT_PLAY_APP_ID);
    assert_eq!(tree.bindings.iter().count(), 1, "exactly one tree-level select, never one per row");

    let frames = section_node(&tree, LAYOUT_DOCUMENT_SECTIONS[3]);
    assert!(!frames.children.is_empty(), "a viewport this tall materialises frame rows");
    for row in &frames.children {
        assert!(row.bindings.iter().next().is_none(), "a pick row binds no action of its own: {}", row.key.as_str());
        let Component::TreeItem(props) = &row.component else { panic!("tree item") };
        assert_eq!(props.granularity.as_ref().map(|text| text.as_str()), Some(LAYOUT_GRANULARITY_ELEMENT));
    }

    let page = section_node(&tree, LAYOUT_DOCUMENT_SECTIONS[2]).children.get(0).expect("a page row");
    assert_eq!(page.bindings.iter().next().expect("a page row keeps its own action").action.name.as_str(), "setActivePage");
    let Component::TreeItem(props) = &page.component else { panic!("tree item") };
    assert!(props.granularity.is_none(), "a page is not a target of the elements domain");
}
//#endregion 🪟️WindowLaws
