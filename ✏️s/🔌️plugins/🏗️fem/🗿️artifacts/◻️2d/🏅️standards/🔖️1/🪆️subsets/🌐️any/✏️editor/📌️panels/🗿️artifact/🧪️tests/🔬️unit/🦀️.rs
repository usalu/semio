use super::*;
use crate::editor::fem2d::terminology::fem2d_labels;
use semio_framework_plugin::plugin_app_close_prelude::Component;
use semio_framework_plugin::{ComponentTree, Locale, TreeWindowRequest, ViewModel, INTERACTION_SELECT_ACTION_ID};
use semio_framework_ui_contract::{TreeWindow, UI_BUILT_CHILDREN_MAX};

//#region 🔖️Fixtures
fn demo() -> Fem2dSnapshot {
    crate::standards::v1::subsets::any::schema::default_fem2d_snapshot()
}

fn english() -> &'static Fem2dLabels {
    fem2d_labels(&ViewModel::default())
}

fn german() -> &'static Fem2dLabels {
    fem2d_labels(&ViewModel { locale: Locale::De, ..Default::default() })
}

/// 🪟️ A host viewport tall enough to hold the demo whole — the label, nesting and keying laws read
/// the tree itself, so they ask for every row rather than one screenful.
fn wide_view() -> ViewModel {
    ViewModel { tree_viewport_rows: Some(512), ..Default::default() }
}

fn build_for(document: &Fem2dSnapshot, labels: &Fem2dLabels, view: &ViewModel) -> BuiltNode {
    render(document, &Fem2dInteractionSnapshot::default(), labels, &TreeWindows::for_body(view, BODY_KEY)).expect("fem2d artifact tree assembly")
}

fn build(document: &Fem2dSnapshot, labels: &Fem2dLabels) -> BuiltNode {
    build_for(document, labels, &wide_view())
}

fn projection(node: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(ComponentTree { root: node }).expect("fixture projection")
}

fn section_node<'a>(tree: &'a BuiltNode, suffix: &str) -> &'a BuiltNode {
    let key = format!("{TREE_NAMESPACE}.{suffix}");
    tree.children.iter().find(|child| child.key.as_str() == key).unwrap_or_else(|| panic!("section {key}"))
}

fn row_labels(parent: &BuiltNode) -> Vec<String> {
    parent
        .children
        .iter()
        .filter_map(|row| match &row.component {
            Component::TreeItem(props) => Some(props.label.0.as_str().to_string()),
            _ => None,
        })
        .collect()
}

fn row_keys(parent: &BuiltNode) -> Vec<&str> {
    parent.children.iter().map(|row| row.key.as_str()).collect()
}

/// 🪟️ The window a container stamps — the full logical extent plus the offset of the slice it built.
/// An empty list the host has never asked about stamps nothing, which is the same statement as an
/// extent of zero.
fn window_or_empty(node: &BuiltNode) -> Option<TreeWindow> {
    match &node.component {
        Component::TreeSection(props) => props.window,
        Component::TreeItem(props) => props.window,
        _ => panic!("a windowed container is a tree section or a nesting tree item"),
    }
}

fn window_of(node: &BuiltNode) -> TreeWindow {
    window_or_empty(node).expect("a non-empty container stamps its window")
}

fn extent_of(node: &BuiltNode) -> usize {
    window_or_empty(node).map_or(0, |window| window.total as usize)
}

/// 🪟️ One host window request for a container of this body.
fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: BODY_KEY.into(), node_key: node_key.into(), open, offset, rows }
}

fn viewing(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..Default::default() }
}

/// 🧱️ A document an order of magnitude past one viewport: 60 nodes and a 40-load wind case.
fn oversized() -> Fem2dSnapshot {
    let mut document = demo();
    document.nodes = (0..60).map(|index| FemNode { id: format!("g{index}"), x: index as f64, y: 0.0 }).collect();
    document.load_cases.push(FemLoadCase {
        id: "wind".into(),
        name: "Wind".into(),
        loads: (0..40).map(|index| FemLoad::Nodal { id: format!("wl{index}"), node_id: "g0".into(), dof: FemDof::Tx, value: 1000.0 }).collect(),
        self_weight: false,
    });
    document
}

const SECTION_SUFFIXES: [&str; SECTIONS] = ["nodes", "elements", "regions", "supports", "load-cases", "combinations", "materials", "sections", "analysis"];

fn section_totals(document: &Fem2dSnapshot) -> [usize; SECTIONS] {
    [document.nodes.len(), document.elements.len(), document.regions.len(), document.supports.len(), document.load_cases.len(), document.combinations.len(), document.materials.len(), document.sections.len(), 1]
}
//#endregion 🔖️Fixtures

//#region 🔖️Structure
/// 🌳️ The demo document reaches every section, each header names the document's own count, and each
/// section stamps that same count as the extent of the list it is a window onto.
#[semio_framework_async_macros::async_test]
async fn demo_document_lists_every_section_with_its_own_count() {
    let document = demo();
    let tree = build(&document, english());
    let expected: [(&str, &str, usize); 8] = [
        ("nodes", "Nodes", document.nodes.len()),
        ("elements", "Elements", document.elements.len()),
        ("regions", "Regions", document.regions.len()),
        ("supports", "Supports", document.supports.len()),
        ("load-cases", "Load Cases", document.load_cases.len()),
        ("combinations", "Combinations", document.combinations.len()),
        ("materials", "Materials", document.materials.len()),
        ("sections", "Sections", document.sections.len()),
    ];
    assert_eq!(tree.children.len(), SECTIONS, "nine sections, analysis included");
    let json = projection(build(&document, english()));
    for (suffix, noun, count) in expected {
        let node = section_node(&tree, suffix);
        assert!(json.contains(&format!("{noun} ({count})")), "section {suffix} header names its own count: {json}");
        assert_eq!(extent_of(node), count, "section {suffix} stamps the extent of its own list");
        assert_eq!(node.children.len(), count.max(1), "a viewport this tall materialises section {suffix} whole");
    }
    assert!(json.contains("Modal Count 3 · Buckling Count 3 · Deformation Scale 300"), "the analysis row carries the three settings");
    assert_eq!(document.nodes.len(), 12, "the bundled demo is the portal frame");
    assert_eq!(document.elements.len(), 9);
}

/// 🌳️ Row ids ARE the raw entity ids: the framework marks domain selection by them, so a composite
/// `fem2d-node:n1` key would never match a viewport pick.
#[semio_framework_async_macros::async_test]
async fn rows_are_keyed_by_the_raw_entity_id_and_bound_to_the_fem2d_domain() {
    let document = demo();
    let tree = build(&document, english());
    let Component::Tree(props) = &tree.component else { panic!("panel tree") };
    assert_eq!(props.interaction_domain.as_ref().map(|domain| domain.as_str()), Some(FEM2D_INTERACTION_DOMAIN));
    assert!(row_keys(section_node(&tree, "nodes")).contains(&"n1"));
    assert!(row_keys(section_node(&tree, "elements")).contains(&"e3"));
    assert!(row_keys(section_node(&tree, "regions")).contains(&"r1"));
    assert!(row_keys(section_node(&tree, "supports")).contains(&"s1"));
    assert!(row_keys(section_node(&tree, "load-cases")).contains(&"dead"));
    assert!(row_keys(section_node(&tree, "combinations")).contains(&"uls"));
    assert!(row_keys(section_node(&tree, "materials")).contains(&"steel"));
    assert!(row_keys(section_node(&tree, "sections")).contains(&"chs76"));
}

/// 📋️ A load case row owns one child row per load, and a combination row one read-only child per
/// term; the case picks at `loadCase` granularity and each load at `load` granularity.
#[semio_framework_async_macros::async_test]
async fn load_case_rows_nest_their_loads_and_combination_rows_nest_their_terms() {
    let document = demo();
    let tree = build(&document, english());
    let cases = section_node(&tree, "load-cases");
    let dead = cases.children.iter().find(|row| row.key.as_str() == "dead").expect("dead case row");
    assert_eq!(row_keys(dead), vec!["l5"]);
    assert_eq!(window_of(dead).total, 1, "a case row stamps the extent of its own loads");
    let live = cases.children.iter().find(|row| row.key.as_str() == "live").expect("live case row");
    assert_eq!(row_keys(live), vec!["l6", "l7"]);
    let uls = section_node(&tree, "combinations").children.iter().find(|row| row.key.as_str() == "uls").expect("uls row");
    assert_eq!(row_keys(uls), vec![format!("{TREE_NAMESPACE}.term.uls.dead").as_str(), format!("{TREE_NAMESPACE}.term.uls.live").as_str()]);
    assert_eq!(row_labels(uls), vec!["1.35 dead".to_string(), "1.5 live".to_string()]);
    assert_eq!(window_of(uls).total, 2, "a combination row stamps the extent of its own terms");
    let json = projection(build(&document, english()));
    assert!(json.contains(FEM2D_GRANULARITY_LOAD_CASE), "{json}");
    assert!(json.contains(FEM2D_GRANULARITY_LOAD));
    assert!(json.contains(FEM2D_GRANULARITY_COMBINATION));
}
//#endregion 🔖️Structure

//#region 🔖️Labels
/// 🏷️ Every row spells its entity the way an engineer reads it — id, then the one fact that tells
/// two rows of the same kind apart.
#[semio_framework_async_macros::async_test]
async fn rows_carry_the_human_label_of_their_entity() {
    let document = demo();
    let tree = build(&document, english());
    assert!(row_labels(section_node(&tree, "nodes")).contains(&"n1 · (0.00, −4.00)".to_string()));
    assert!(row_labels(section_node(&tree, "nodes")).contains(&"p0_l1 · (0.00, 2.80)".to_string()));
    assert!(row_labels(section_node(&tree, "elements")).contains(&"e3 · Beam n1 → n2".to_string()));
    assert!(row_labels(section_node(&tree, "regions")).contains(&"r1 · First Floor Slab · 4 pts".to_string()));
    assert!(row_labels(section_node(&tree, "supports")).contains(&"s1 · n1 · Tx Ty".to_string()));
    assert!(row_labels(section_node(&tree, "load-cases")).contains(&"Dead Load · 1 Load · Self Weight".to_string()));
    assert!(row_labels(section_node(&tree, "load-cases")).contains(&"Live Load · 2 Loads".to_string()));
    assert!(row_labels(section_node(&tree, "combinations")).contains(&"ULS · 1.35 dead + 1.5 live".to_string()));
    assert!(row_labels(section_node(&tree, "materials")).contains(&"Steel S235 · E 210 GPa".to_string()));
    assert!(row_labels(section_node(&tree, "sections")).contains(&"CHS 76 Foundation Column · A 0.001 m² · Iy 1.00e-6 m⁴".to_string()));

    let live = section_node(&tree, "load-cases").children.iter().find(|row| row.key.as_str() == "live").expect("live case row");
    let loads = row_labels(live);
    assert!(loads.contains(&"Ty −12000 N @ p8_l1".to_string()), "{loads:?}");
    assert!(loads.contains(&"1500 Pa @ r1".to_string()), "{loads:?}");
}

/// 🔢️ A section's `iy` reads as an exponent, a member UDL keeps its typographic minus, and a
/// pathological name is clipped rather than refusing the whole render.
#[semio_framework_async_macros::async_test]
async fn scalars_and_overlong_labels_stay_inside_the_ui_text_envelope() {
    let labels = english();
    let hea200 = FemSection { id: "hea200".into(), name: "HEA200".into(), area: 0.00538, iy: 0.0000369 };
    assert_eq!(fem2d_section_label(&hea200), "HEA200 · A 0.00538 m² · Iy 3.69e-5 m⁴");
    let udl = FemLoad::MemberUdl { id: "l1".into(), element_id: "e8".into(), wx: 0.0, wy: -500.0 };
    assert_eq!(fem2d_load_label(&udl, labels), "wy −500 N/m @ e8");
    assert_eq!(fem2d_scalar(0.0), "0");
    assert_eq!(fem2d_coordinate(-4.0), "−4.00");

    let mut document = Fem2dSnapshot::default();
    document.regions.push(FemRegion { id: "r1".into(), name: "R".repeat(900), outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], holes: Vec::new(), thickness: 0.2, material_id: "concrete".into(), mesh_size: 1.0 });
    let tree = build(&document, labels);
    let clipped = row_labels(section_node(&tree, "regions")).remove(0);
    assert!(clipped.len() <= semio_framework_ui_contract::UI_TEXT_MAX_BYTES, "clipped to {} bytes", clipped.len());
    assert!(clipped.ends_with('…'), "an overlong region name is clipped, not refused");
}

/// 🇩🇪️ The whole tree localises: section headers, element kinds, the load-case summary and the
/// entity noun each row carries as its description.
#[semio_framework_async_macros::async_test]
async fn german_labels_resolve_across_the_whole_tree() {
    let document = demo();
    let json = projection(build(&document, german()));
    assert!(json.contains("Knoten (12)"), "{json}");
    assert!(json.contains("Elemente (9)"));
    assert!(json.contains("Lastfälle (2)"));
    assert!(json.contains("Querschnitte (4)"));
    assert!(json.contains("Balken n1 → n2"));
    assert!(json.contains("Eigengewicht"));
    assert!(json.contains("Anzahl Eigenformen"));
    assert!(!json.contains("Nodes ("));
    assert!(!json.contains("Beam n1"));
}
//#endregion 🔖️Labels

//#region 🔖️Interaction
/// 🕹️ A node row DECLARES its pick — `granularity` plus its own raw key — and binds nothing: the
/// whole tree carries exactly one `interactionSelect`, so a wide section costs no argument arena and
/// no row actions compete with the panels rendered beside it.
#[semio_framework_async_macros::async_test]
async fn rows_declare_their_granularity_while_the_tree_binds_the_one_interaction_select() {
    let document = demo();
    let tree = build(&document, english());
    let row = section_node(&tree, "nodes").children.iter().find(|row| row.key.as_str() == "n1").expect("n1 row");
    assert!(row.bindings.iter().next().is_none(), "a pick row binds no action of its own");
    let Component::TreeItem(props) = &row.component else { panic!("tree item") };
    assert_eq!(props.granularity.as_ref().map(|text| text.as_str()), Some(FEM2D_GRANULARITY_NODE));
    assert!(props.row_actions.is_empty(), "a row authors its pick only — focus and delete live in the inspector");
    assert_eq!(props.description.as_ref().map(|text| text.as_str()), Some("Node"));

    let binding = tree.bindings.iter().next().expect("the tree binds the domain select");
    assert_eq!(binding.action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    assert_eq!(binding.action.scope.as_str(), crate::editor::fem2d::FEM2D_PLAY_CONTROLLER_ID);
    assert_eq!(tree.bindings.iter().count(), 1, "exactly one tree-level select, never one per row");

    let json = projection(build(&document, english()));
    assert_eq!(json.matches(INTERACTION_SELECT_ACTION_ID).count(), 1, "the select is authored once for the whole tree: {json}");
    assert!(!json.contains("focusEntity") && !json.contains("removeSelection"), "the tree carries no row actions: {json}");
}

/// 🎯️ Selection and hover are recorded on the built tree from the live interaction snapshot, and a
/// selection wider than one fixed list marks its first page instead of refusing the render.
#[semio_framework_async_macros::async_test]
async fn selected_and_hovered_ids_are_marked_from_the_interaction_snapshot() {
    let document = demo();
    let view = wide_view();
    let windows = TreeWindows::for_body(&view, BODY_KEY);
    let interaction = Fem2dInteractionSnapshot { selected_ids: vec!["n1".into(), "e3".into()], hovered_ids: vec!["s1".into()] };
    let tree = render(&document, &interaction, english(), &windows).expect("marked tree");
    let Component::Tree(props) = &tree.component else { panic!("panel tree") };
    assert_eq!(props.interaction_domain.as_ref().map(|domain| domain.as_str()), Some(FEM2D_INTERACTION_DOMAIN));
    let wide: Vec<String> = (0..80).map(|index| format!("n{index}")).collect();
    assert_eq!(marked_ids(&wide).len(), MARKED_IDS_LIMIT, "a wider selection marks its first page rather than refusing the render");
    let interaction = Fem2dInteractionSnapshot { selected_ids: wide, hovered_ids: Vec::new() };
    assert!(render(&document, &interaction, english(), &windows).is_ok(), "an oversized selection never faults the panel");
}

/// 🪆️ The app's manifest declares this panel under the framework's artifact tab, with the body key
/// its render routing switches on — the seam between "the tree assembles" and "the shell shows it".
///
/// 🐛️ This asserts the manifest rather than driving `context::fem2d_app()` + `render(app, BODY_KEY)`:
/// as of 2026-09-16 EVERY `fem2d_app()`-based test in this crate — the pre-existing
/// `renders_fem2d_model_scene` and `an_unknown_body_key_renders_a_diagnostic_instead_of_panicking`
/// included — aborts in `ArtifactStore::drop` with `artifact store reached Drop without its exact
/// terminal-empty shallow-shell witness`. That is the shared harness owed a close step, not this
/// panel; the routed render belongs back here once the harness retires its store.
#[semio_framework_async_macros::async_test]
async fn the_app_declares_the_artifact_panel_under_its_body_key() {
    let definition = crate::editor::fem2d::create_fem2d_app();
    let panel = definition.panel_tabs.iter().find(|tab| tab.body_key.as_deref() == Some(BODY_KEY)).expect("the artifact panel is declared");
    assert!(matches!(panel.group, PanelGroup::Workbench));
    assert!(matches!(&panel.kind, PanelTabKind::App(id) if id == FRAMEWORK_PANEL_TAB_ARTIFACT_ID));
    assert_eq!(definition.panel_tabs.iter().filter(|tab| tab.body_key.as_deref() == Some(BODY_KEY)).count(), 1, "one artifact panel, declared once");
}
//#endregion 🔖️Interaction

//#region 🔖️Windows
/// 🪟️ A document an order of magnitude past one viewport streams instead of truncating: every
/// container stamps the FULL extent of its own list, materialises no more than the slice it was
/// given, and the body never invents a `+N` row to stand in for the remainder.
#[semio_framework_async_macros::async_test]
async fn an_oversized_document_stamps_every_extent_and_materialises_one_viewport() {
    let document = oversized();
    let tree = render(&document, &Fem2dInteractionSnapshot::default(), english(), &TreeWindows::unhosted()).expect("an oversized document still assembles");
    let totals = section_totals(&document);
    for (suffix, total) in SECTION_SUFFIXES.into_iter().zip(totals) {
        let node = section_node(&tree, suffix);
        assert_eq!(extent_of(node), total, "section {suffix} stamps the full extent of its list");
        assert!(node.children.len() <= total.max(1), "section {suffix} materialises no more than it has");
        assert!(node.children.len() <= UI_BUILT_CHILDREN_MAX, "section {suffix} stays inside one built page");
    }
    let nodes = section_node(&tree, "nodes");
    assert_eq!(window_of(nodes).total, 60, "60 nodes are announced whole");
    assert!(nodes.children.len() < 60, "a first paint materialises one viewport, not the whole list");
    let wind = section_node(&tree, "load-cases").children.iter().find(|row| row.key.as_str() == "wind");
    if let Some(wind) = wind {
        assert_eq!(window_of(wind).total, 40, "a case row announces every load it owns");
    }
    let total = 1 + tree.children.iter().map(|section| 1 + section.children.iter().map(|row| 1 + row.children.len()).sum::<usize>()).sum::<usize>();
    assert!(total < semio_framework_ui_contract::UI_DOCUMENT_NODES, "one first paint stays inside one UI document: {total}");
    let json = projection(tree);
    assert!(!json.contains(".more"), "a windowed container never mints a continuation key: {json}");
    assert!(!json.contains("\"+"), "a windowed container never mints a `+N` label: {json}");
}

/// 🪟️ A container the user closed is announced, not built: the extent is stamped so the host can
/// size the disclosure, and not one child row is materialised.
#[semio_framework_async_macros::async_test]
async fn a_closed_container_stamps_its_extent_and_builds_no_child() {
    let document = oversized();
    let view = viewing(vec![request(&format!("{TREE_NAMESPACE}.nodes"), Some(false), 0, 48), request("wind", Some(false), 0, 48)]);
    let tree = render(&document, &Fem2dInteractionSnapshot::default(), english(), &TreeWindows::for_body(&view, BODY_KEY)).expect("a closed container still assembles");
    let nodes = section_node(&tree, "nodes");
    assert_eq!(window_of(nodes).total, 60, "a closed section still announces its extent");
    assert!(nodes.children.is_empty(), "a closed section builds no row");
    let wind = section_node(&tree, "load-cases").children.iter().find(|row| row.key.as_str() == "wind").expect("wind case row");
    assert_eq!(window_of(wind).total, 40);
    assert!(wind.children.is_empty(), "a closed case row builds no load");
}

/// 🪟️ Scrolling is a request, not a page: the host names `{offset, rows}` and the guest materialises
/// exactly that half-open range, still keyed by the raw entity id.
#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_own_range() {
    let document = oversized();
    let view = viewing(vec![request(&format!("{TREE_NAMESPACE}.nodes"), None, 20, 8), request("wind", None, 5, 4)]);
    let tree = render(&document, &Fem2dInteractionSnapshot::default(), english(), &TreeWindows::for_body(&view, BODY_KEY)).expect("a windowed document assembles");
    let nodes = section_node(&tree, "nodes");
    assert_eq!(window_of(nodes), TreeWindow { total: 60, offset: 20 });
    assert_eq!(row_keys(nodes), (20..28).map(|index| format!("g{index}")).collect::<Vec<_>>(), "exactly entries [20, 28) keyed by the raw node id");
    let wind = section_node(&tree, "load-cases").children.iter().find(|row| row.key.as_str() == "wind").expect("wind case row");
    assert_eq!(window_of(wind), TreeWindow { total: 40, offset: 5 });
    assert_eq!(row_keys(wind), (5..9).map(|index| format!("wl{index}")).collect::<Vec<_>>(), "a nested window is the same law one level down");
}

/// 🌱️ An empty document is a readable tree of "(none)" placeholders, not a refusal.
#[semio_framework_async_macros::async_test]
async fn an_empty_document_renders_placeholders() {
    let document = Fem2dSnapshot::default();
    let tree = build(&document, english());
    for suffix in ["nodes", "elements", "regions", "supports", "load-cases", "combinations", "materials", "sections"] {
        assert_eq!(row_labels(section_node(&tree, suffix)), vec!["(none)".to_string()], "section {suffix}");
        assert_eq!(extent_of(section_node(&tree, suffix)), 0, "an empty section announces an empty list");
    }
    let json = projection(tree);
    assert!(json.contains("Nodes (0)"), "{json}");
}
//#endregion 🔖️Windows
