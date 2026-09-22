use super::*;
use crate::editor::fem3d::terminology::fem3d_labels;
use semio_framework_plugin::plugin_app_close_prelude::Component;
use semio_framework_plugin::{ComponentTree, Locale, TreeWindowRequest, ViewModel, INTERACTION_SELECT_ACTION_ID};
use semio_framework_ui_contract::{TreeWindow, TREE_WINDOW_PATH_SEPARATOR, UI_BUILT_CHILDREN_MAX};

//#region 🔖️Fixtures
fn demo() -> Fem3dSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_boot_snapshot()
}

fn english() -> &'static Fem3dLabels {
    fem3d_labels(&ViewModel::default())
}

fn german() -> &'static Fem3dLabels {
    fem3d_labels(&ViewModel { locale: Locale::De, ..Default::default() })
}

/// 🪟️ A host viewport tall enough to hold the demo whole, with the three sections the author leaves
/// COLLAPSED opened explicitly — the label, nesting and keying laws read the tree itself, so they ask
/// for every row rather than one screenful, and a closed container is correctly empty.
///
/// 🧾️ The rows asked for per request are deliberately small. A host request RESERVES `1 + rows` off
/// the body-wide node ledger (`TreeWindows`, `TREE_WINDOW_BODY_NODE_BUDGET`) before any container is
/// built, so that a container the user scrolled to cannot be starved by the first-paint slices of the
/// containers in front of it. A fixture asking three times for 128 rows would reserve the whole
/// budget and starve the six sections the host has not addressed — which is exactly the cap the React
/// host applies to its own report (`Σ(1 + rows) ≤ TREE_WINDOW_BODY_NODE_BUDGET`). These three sections
/// hold one material, two sections and one analysis row between them, so two is exactly enough and
/// leaves the rest of the budget to the six the host has not addressed — which the bundled
/// concrete-forest document needs, at 97 of the 103 records a body may spend.
fn wide_view() -> ViewModel {
    let opened = ["materials", "sections", "analysis"].into_iter().map(|suffix| request(&format!("{TREE_NAMESPACE}.{suffix}"), Some(true), 0, 2)).collect();
    ViewModel { tree_windows: opened, tree_viewport_rows: Some(512), ..Default::default() }
}

fn build_for(document: &Fem3dSnapshot, labels: &Fem3dLabels, view: &ViewModel) -> BuiltNode {
    render(document, &Fem3dInteractionSnapshot::default(), labels, &TreeWindows::for_body(view, BODY_KEY)).expect("fem3d artifact tree assembly")
}

fn build(document: &Fem3dSnapshot, labels: &Fem3dLabels) -> BuiltNode {
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

/// 🔑️ A NESTED container is addressed by its window PATH — its parent section's key, the separator,
/// then its own key — never by its bare id. That is what lets a load case and a combination both
/// called `uls` keep independent windows while their node keys stay the raw pick target ids.
fn nested_request(section_suffix: &str, node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    request(&format!("{TREE_NAMESPACE}.{section_suffix}{TREE_WINDOW_PATH_SEPARATOR}{node_key}"), open, offset, rows)
}

fn viewing(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..Default::default() }
}

/// 🧱️ A document an order of magnitude past one viewport: 60 nodes and a 40-load wind case.
fn oversized() -> Fem3dSnapshot {
    let mut document = demo();
    document.nodes = (0..60).map(|index| FemNode { id: format!("g{index}"), x: index as f64, y: 0.0, z: 0.0 }).collect();
    document.load_cases.push(FemLoadCase {
        id: "wind".into(),
        name: "Wind".into(),
        loads: (0..40).map(|index| FemLoad::Nodal { id: format!("wl{index}"), node_id: "g0".into(), dof: FemDof::Tx, value: 1000.0 }).collect(),
        self_weight: false,
    });
    document
}

/// 🏠️ The document `📓️w3-browser-verification.md` §6.2 caught faulting the artifact panel surface at
/// `nodes: 129 > max_nodes: 128`: 63 nodes, 63 supports, 8 solids and four load cases carrying twelve
/// loads each, every one of them open at the same time. Per-container windowing bounds each
/// CONTAINER; only the body-wide node ledger bounds the BODY.
fn house() -> Fem3dSnapshot {
    let mut document = demo();
    document.nodes = (0..63).map(|index| FemNode { id: format!("h{index}"), x: index as f64, y: 0.0, z: 0.0 }).collect();
    document.supports = (0..63).map(|index| crate::FemSupport { id: format!("hs{index}"), node_id: format!("h{index}"), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] }).collect();
    document.solids = (0..8)
        .map(|index| FemSolid {
            id: format!("hsol{index}"),
            name: format!("Slab {index}"),
            outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]],
            holes: Vec::new(),
            base_z: index as f64,
            height: 0.2,
            layers: 1,
            mesh_size: 1.0,
            material_id: "concrete".into(),
            axis: crate::FemAxis::Z,
        })
        .collect();
    document.load_cases = (0..4)
        .map(|case| FemLoadCase {
            id: format!("hc{case}"),
            name: format!("Case {case}"),
            loads: (0..12).map(|index| FemLoad::Nodal { id: format!("hl{case}_{index}"), node_id: "h0".into(), dof: FemDof::Tz, value: -1000.0 }).collect(),
            self_weight: false,
        })
        .collect();
    document
}

/// 🧾️ Records the reconciler charges for a presented body — every node counts once, which is exactly
/// `SurfaceReconcileLimits::max_nodes` (`semio_framework_ui_contract::UI_DOCUMENT_NODES`).
fn body_nodes(node: &BuiltNode) -> usize {
    1 + node.children.iter().map(body_nodes).sum::<usize>()
}

const SECTION_SUFFIXES: [&str; SECTIONS] = ["nodes", "elements", "solids", "supports", "load-cases", "combinations", "materials", "sections", "analysis"];

fn section_totals(document: &Fem3dSnapshot) -> [usize; SECTIONS] {
    [document.nodes.len(), document.elements.len(), document.solids.len(), document.supports.len(), document.load_cases.len(), document.combinations.len(), document.materials.len(), document.sections.len(), 1]
}
//#endregion 🔖️Fixtures

//#region 🔖️Structure
/// 🌳️ The demo hall reaches every section, each header names the document's own count, and each
/// section stamps that same count as the extent of the list it is a window onto.
#[semio_framework_async_macros::async_test]
async fn demo_document_lists_every_section_with_its_own_count() {
    let document = demo();
    let tree = build(&document, english());
    let expected: [(&str, &str, usize); 8] = [
        ("nodes", "Nodes", document.nodes.len()),
        ("elements", "Elements", document.elements.len()),
        ("solids", "Solids", document.solids.len()),
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
    assert_eq!(document.nodes.len(), 20, "the bundled demo is the concrete forest");
    assert_eq!(document.elements.len(), 20);
}

/// 🌳️ Row ids ARE the raw entity ids: the framework marks domain selection by them, so a composite
/// key would never match a viewport pick.
#[semio_framework_async_macros::async_test]
async fn rows_are_keyed_by_the_raw_entity_id_and_bound_to_the_fem3d_domain() {
    let document = demo();
    let tree = build(&document, english());
    let Component::Tree(props) = &tree.component else { panic!("panel tree") };
    assert_eq!(props.interaction_domain.as_ref().map(|domain| domain.as_str()), Some(FEM3D_INTERACTION_DOMAIN));
    assert!(row_keys(section_node(&tree, "nodes")).contains(&"lc1b"));
    assert!(row_keys(section_node(&tree, "elements")).contains(&"l_col1"));
    assert!(row_keys(section_node(&tree, "supports")).contains(&"s_c1"));
    assert!(row_keys(section_node(&tree, "load-cases")).contains(&"dead"));
    assert!(row_keys(section_node(&tree, "combinations")).contains(&"uls"));
    assert!(row_keys(section_node(&tree, "materials")).contains(&"c30"));
    assert!(row_keys(section_node(&tree, "sections")).contains(&"hex30"));
}

/// 📋️ A load case row owns one child row per load, and a combination row one read-only child per
/// term; the case picks at `loadCase` granularity and each load at `load` granularity.
#[semio_framework_async_macros::async_test]
async fn load_case_rows_nest_their_loads_and_combination_rows_nest_their_terms() {
    let document = demo();
    let tree = build(&document, english());
    let cases = section_node(&tree, "load-cases");
    let dead = cases.children.iter().find(|row| row.key.as_str() == "dead").expect("dead case row");
    assert_eq!(window_of(dead).total, 16, "a case row stamps the extent of its own loads");
    assert!(row_keys(dead).contains(&"d_l_spine"), "{:?}", row_keys(dead));
    let live = cases.children.iter().find(|row| row.key.as_str() == "live").expect("live case row");
    assert_eq!(window_of(live).total, 16);
    assert!(row_keys(live).contains(&"q_l_spine"), "{:?}", row_keys(live));
    let uls = section_node(&tree, "combinations").children.iter().find(|row| row.key.as_str() == "uls").expect("uls row");
    assert_eq!(row_keys(uls), vec![format!("{TREE_NAMESPACE}.term.uls.dead").as_str(), format!("{TREE_NAMESPACE}.term.uls.live").as_str()]);
    assert_eq!(row_labels(uls), vec!["1.35 dead".to_string(), "1.5 live".to_string()]);
    assert_eq!(window_of(uls).total, 2, "a combination row stamps the extent of its own terms");
    let json = projection(build(&document, english()));
    assert!(json.contains(FEM3D_GRANULARITY_LOAD_CASE), "{json}");
    assert!(json.contains(FEM3D_GRANULARITY_LOAD));
    assert!(json.contains(FEM3D_GRANULARITY_COMBINATION));
}
//#endregion 🔖️Structure

//#region 🔖️Labels
/// 🏷️ Every row spells its entity the way an engineer reads it — id, then the one fact that tells
/// two rows of the same kind apart.
#[semio_framework_async_macros::async_test]
async fn rows_carry_the_human_label_of_their_entity() {
    let document = demo();
    let tree = build(&document, english());
    assert!(row_labels(section_node(&tree, "nodes")).contains(&"lc1b · (2.70, 2.34, 0.00)".to_string()));
    assert!(row_labels(section_node(&tree, "elements")).contains(&"l_col1 · Frame lc1b → lc1t".to_string()));
    assert!(row_labels(section_node(&tree, "supports")).contains(&"s_c1 · lc1b · Tx Ty Tz Rx Ry Rz".to_string()));
    assert!(row_labels(section_node(&tree, "load-cases")).contains(&"Dead Load · 16 Loads · Self Weight".to_string()));
    assert!(row_labels(section_node(&tree, "load-cases")).contains(&"Live Load · 16 Loads".to_string()));
    assert!(row_labels(section_node(&tree, "combinations")).contains(&"ULS · 1.35 dead + 1.5 live".to_string()));
    assert!(row_labels(section_node(&tree, "materials")).contains(&"C30/37 Concrete · E 33 GPa".to_string()));
    assert!(row_labels(section_node(&tree, "sections")).contains(&"Hexagonal Column R 0.30 · A 0.233827 m² · Iy 0.00438425 m⁴".to_string()));

    let live = section_node(&tree, "load-cases").children.iter().find(|row| row.key.as_str() == "live").expect("live case row");
    let loads = row_labels(live);
    assert!(loads.contains(&"wz −4952 N/m @ l_spine".to_string()), "{loads:?}");
}

/// 🔢️ A member UDL lists only its non-zero components, and a pathological name is clipped rather
/// than refusing the whole render.
#[semio_framework_async_macros::async_test]
async fn scalars_and_overlong_labels_stay_inside_the_ui_text_envelope() {
    let labels = english();
    let udl = FemLoad::MemberUdl { id: "l1".into(), element_id: "e8".into(), wx: 0.0, wy: 0.0, wz: -500.0 };
    assert_eq!(fem3d_load_label(&udl, labels), "wz −500 N/m @ e8");
    assert_eq!(fem3d_scalar(0.0), "0");
    assert_eq!(fem3d_coordinate(-4.0), "−4.00");

    let mut document = Fem3dSnapshot::default();
    document.solids.push(FemSolid { id: "s1".into(), name: "S".repeat(900), outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], holes: Vec::new(), base_z: 0.0, height: 0.2, layers: 1, mesh_size: 1.0, material_id: "concrete".into(), axis: crate::FemAxis::Z });
    let tree = build(&document, labels);
    let clipped = row_labels(section_node(&tree, "solids")).remove(0);
    assert!(clipped.len() <= semio_framework_ui_contract::UI_TEXT_MAX_BYTES, "clipped to {} bytes", clipped.len());
    assert!(clipped.ends_with('…'), "an overlong solid name is clipped, not refused");
}

/// 🇩🇪️ The whole tree localises: section headers, element kinds, the load-case summary and the
/// entity noun each row carries as its description.
#[semio_framework_async_macros::async_test]
async fn german_labels_resolve_across_the_whole_tree() {
    let document = demo();
    let json = projection(build(&document, german()));
    assert!(json.contains("Knoten (20)"), "{json}");
    assert!(json.contains("Elemente (20)"));
    assert!(json.contains("Volumenkörper (0)"));
    assert!(json.contains("Lastfälle (2)"));
    assert!(json.contains("Rahmenstab lc1b → lc1t"));
    assert!(json.contains("Eigengewicht"));
    assert!(json.contains("Anzahl Eigenformen"));
    assert!(!json.contains("Nodes ("));
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
    let row = section_node(&tree, "nodes").children.iter().find(|row| row.key.as_str() == "lc1b").expect("lc1b row");
    assert!(row.bindings.iter().next().is_none(), "a pick row binds no action of its own");
    let Component::TreeItem(props) = &row.component else { panic!("tree item") };
    assert_eq!(props.granularity.as_ref().map(|text| text.as_str()), Some(FEM3D_GRANULARITY_NODE));
    assert!(props.row_actions.is_empty(), "a row authors its pick only — focus and delete live in the inspector");
    assert_eq!(props.description.as_ref().map(|text| text.as_str()), Some("Node"));

    let binding = tree.bindings.iter().next().expect("the tree binds the domain select");
    assert_eq!(binding.action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    assert_eq!(binding.action.scope.as_str(), FEM3D_PLAY_CONTROLLER_ID);
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
    let interaction = Fem3dInteractionSnapshot { selected_ids: vec!["lc1b".into(), "l_col1".into()], hovered_ids: vec!["s_c1".into()] };
    let tree = render(&document, &interaction, english(), &windows).expect("marked tree");
    let Component::Tree(props) = &tree.component else { panic!("panel tree") };
    assert_eq!(props.interaction_domain.as_ref().map(|domain| domain.as_str()), Some(FEM3D_INTERACTION_DOMAIN));
    let wide: Vec<String> = (0..80).map(|index| format!("n{index}")).collect();
    assert_eq!(marked_ids(&wide).len(), MARKED_IDS_LIMIT, "a wider selection marks its first page rather than refusing the render");
    let interaction = Fem3dInteractionSnapshot { selected_ids: wide, hovered_ids: Vec::new() };
    // 🪟️ A fresh `TreeWindows` per render: the value carries the render's first-paint budget and its
    // body-wide node ledger in `Cell`s, so reusing the one the render above already spent would ask
    // this panel to build a whole tree out of an exhausted ledger — which is not what the host does
    // (`render_body` builds one per body per render).
    let windows = TreeWindows::for_body(&view, BODY_KEY);
    render(&document, &interaction, english(), &windows).expect("an oversized selection never faults the panel");
}

/// 🪆️ The app's manifest declares this panel under the framework's artifact tab, with the body key
/// its render routing switches on.
#[semio_framework_async_macros::async_test]
async fn the_app_declares_the_artifact_panel_under_its_body_key() {
    let definition = crate::editor::fem3d::create_fem3d_app();
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
    let tree = render(&document, &Fem3dInteractionSnapshot::default(), english(), &TreeWindows::unhosted()).expect("an oversized document still assembles");
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
    let view = viewing(vec![request(&format!("{TREE_NAMESPACE}.nodes"), Some(false), 0, 48), nested_request("load-cases", "wind", Some(false), 0, 48)]);
    let tree = render(&document, &Fem3dInteractionSnapshot::default(), english(), &TreeWindows::for_body(&view, BODY_KEY)).expect("a closed container still assembles");
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
    let view = viewing(vec![request(&format!("{TREE_NAMESPACE}.nodes"), None, 20, 8), nested_request("load-cases", "wind", None, 5, 4)]);
    let tree = render(&document, &Fem3dInteractionSnapshot::default(), english(), &TreeWindows::for_body(&view, BODY_KEY)).expect("a windowed document assembles");
    let nodes = section_node(&tree, "nodes");
    assert_eq!(window_of(nodes), TreeWindow { row_extent: Default::default(), total: 60, offset: 20 });
    assert_eq!(row_keys(nodes), (20..28).map(|index| format!("g{index}")).collect::<Vec<_>>(), "exactly entries [20, 28) keyed by the raw node id");
    let wind = section_node(&tree, "load-cases").children.iter().find(|row| row.key.as_str() == "wind").expect("wind case row");
    assert_eq!(window_of(wind), TreeWindow { row_extent: Default::default(), total: 40, offset: 5 });
    assert_eq!(row_keys(wind), (5..9).map(|index| format!("wl{index}")).collect::<Vec<_>>(), "a nested window is the same law one level down");
}

/// 🏠️ The House regression: the whole BODY reconciles, not just each container. A first paint of a
/// 63-node, 63-support, 8-solid document with four open load cases stays inside the reconciler's
/// record arena — the fault `📓️w3-browser-verification.md` §6.2 recorded as
/// `1:framework.panel.artifact — nodes: 129 vs max_nodes: 128`.
#[semio_framework_async_macros::async_test]
async fn a_house_sized_first_paint_keeps_the_whole_body_inside_the_record_arena() {
    let document = house();
    let tree = render(&document, &Fem3dInteractionSnapshot::default(), english(), &TreeWindows::unhosted()).expect("a House-sized document still assembles");
    let nodes = body_nodes(&tree);
    println!("[DEBUG] fem3d-house first-paint body_nodes={nodes}");
    assert!(nodes <= semio_framework_ui_contract::UI_DOCUMENT_NODES, "the House body reconciles: {nodes} > {}", semio_framework_ui_contract::UI_DOCUMENT_NODES);
    for (suffix, total) in SECTION_SUFFIXES.into_iter().zip(section_totals(&document)) {
        assert_eq!(extent_of(section_node(&tree, suffix)), total, "section {suffix} still stamps its full extent, seated or starved");
    }
    let json = projection(tree);
    assert!(!json.contains(".more") && !json.contains("\"+"), "no continuation row survives a starved body: {json}");
}

/// 🏠️ The same document with the host driving it: every open container addressed, `Σ(1 + rows)`
/// inside `TREE_WINDOW_BODY_NODE_BUDGET` exactly as the React host caps its own report. Every
/// requested window is honoured in full — a request the host is allowed to file is one the guest can
/// always serve — and the body still fits the record arena.
#[semio_framework_async_macros::async_test]
async fn a_house_sized_body_honours_every_capped_host_window() {
    let document = house();
    let mut requests = vec![
        request(&format!("{TREE_NAMESPACE}.nodes"), Some(true), 0, 20),
        request(&format!("{TREE_NAMESPACE}.solids"), Some(true), 0, 8),
        request(&format!("{TREE_NAMESPACE}.supports"), Some(true), 0, 20),
        request(&format!("{TREE_NAMESPACE}.load-cases"), Some(true), 0, 4),
    ];
    requests.extend((0..4).map(|case| nested_request("load-cases", &format!("hc{case}"), Some(true), 0, 8)));
    let cost: u32 = requests.iter().map(|entry| 1 + entry.rows).sum();
    assert!(cost as usize <= semio_framework_plugin::TREE_WINDOW_BODY_NODE_BUDGET, "the fixture asks what the host is allowed to ask: {cost}");
    let view = viewing(requests);
    let tree = render(&document, &Fem3dInteractionSnapshot::default(), english(), &TreeWindows::for_body(&view, BODY_KEY)).expect("a House-sized windowed document assembles");
    assert_eq!(section_node(&tree, "nodes").children.len(), 20, "a capped request is honoured in full, wherever it sits in the body");
    assert_eq!(section_node(&tree, "supports").children.len(), 20);
    assert_eq!(section_node(&tree, "solids").children.len(), 8);
    let cases = section_node(&tree, "load-cases");
    assert_eq!(cases.children.len(), 4, "every load case row is materialised");
    for case in cases.children.iter() {
        assert_eq!(case.children.len(), 8, "and every open case serves its own capped window");
        assert_eq!(window_of(case).total, 12, "while still announcing all twelve loads");
    }
    assert_eq!(window_of(section_node(&tree, "nodes")), TreeWindow { row_extent: Default::default(), total: 63, offset: 0 });
    let nodes = body_nodes(&tree);
    println!("[DEBUG] fem3d-house hosted body_nodes={nodes}");
    assert!(nodes <= semio_framework_ui_contract::UI_DOCUMENT_NODES, "the House body reconciles under a full host request set: {nodes} > {}", semio_framework_ui_contract::UI_DOCUMENT_NODES);
}

/// 🌱️ An empty document is a readable tree of "(none)" placeholders, not a refusal.
#[semio_framework_async_macros::async_test]
async fn an_empty_document_renders_placeholders() {
    let document = Fem3dSnapshot::default();
    let tree = build(&document, english());
    for suffix in ["nodes", "elements", "solids", "supports", "load-cases", "combinations", "materials", "sections"] {
        assert_eq!(row_labels(section_node(&tree, suffix)), vec!["(none)".to_string()], "section {suffix}");
        assert_eq!(extent_of(section_node(&tree, suffix)), 0, "an empty section announces an empty list");
    }
    let json = projection(tree);
    assert!(json.contains("Nodes (0)"), "{json}");
}
//#endregion 🔖️Windows
