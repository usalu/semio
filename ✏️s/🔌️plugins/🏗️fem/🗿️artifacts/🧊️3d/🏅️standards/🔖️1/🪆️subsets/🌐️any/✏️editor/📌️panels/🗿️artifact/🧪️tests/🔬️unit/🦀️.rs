use super::*;
use crate::editor::fem3d::terminology::fem3d_labels;
use semio_framework_plugin::{ComponentTree, Locale, ViewModel};

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

fn build(document: &Fem3dSnapshot, labels: &Fem3dLabels) -> BuiltNode {
    render(document, &Fem3dInteractionSnapshot::default(), labels).expect("fem3d artifact tree assembly")
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

/// ➕️ The count a truncated section's `+N` continuation row names, or zero when nothing was omitted.
fn omitted(parent: &BuiltNode, suffix: &str) -> usize {
    let key = format!("{TREE_NAMESPACE}.{suffix}.more");
    parent.children.iter().find(|row| row.key.as_str() == key).map_or(0, |row| match &row.component {
        Component::TreeItem(props) => props.label.0.as_str().trim_start_matches('+').parse().expect("continuation count"),
        _ => panic!("continuation row is a tree item"),
    })
}

fn placed(parent: &BuiltNode, suffix: &str) -> usize {
    let continuation = format!("{TREE_NAMESPACE}.{suffix}.more");
    parent.children.iter().filter(|row| row.key.as_str() != continuation).count()
}

const SECTION_SUFFIXES: [&str; SECTIONS] = ["nodes", "elements", "solids", "supports", "load-cases", "combinations", "materials", "sections", "analysis"];
//#endregion 🔖️Fixtures

//#region 🔖️Structure
/// 🌳️ The demo hall reaches every section, each header names the document's own count, and each
/// section accounts for every entity it owns — placed rows plus what its `+N` row says it left out.
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
        assert_eq!(placed(node, suffix) + omitted(node, suffix), count, "section {suffix} accounts for every entity");
    }
    assert!(json.contains("Modal Count 3 · Buckling Count 3 · Deformation Scale 300"), "the analysis row carries the three settings");
    assert_eq!(document.nodes.len(), 16, "the bundled demo is the two-storey hall");
    assert_eq!(document.elements.len(), 16);
}

/// 🌳️ Row ids ARE the raw entity ids: the framework marks domain selection by them, so a composite
/// key would never match a viewport pick.
#[semio_framework_async_macros::async_test]
async fn rows_are_keyed_by_the_raw_entity_id_and_bound_to_the_fem3d_domain() {
    let document = demo();
    let tree = build(&document, english());
    let Component::Tree(props) = &tree.component else { panic!("panel tree") };
    assert_eq!(props.interaction_domain.as_ref().map(|domain| domain.as_str()), Some(FEM3D_INTERACTION_DOMAIN));
    assert!(row_keys(section_node(&tree, "nodes")).contains(&"n00_g"));
    assert!(row_keys(section_node(&tree, "elements")).contains(&"e1"));
    assert!(row_keys(section_node(&tree, "solids")).contains(&"sol1"));
    assert!(row_keys(section_node(&tree, "supports")).contains(&"s_00"));
    assert!(row_keys(section_node(&tree, "load-cases")).contains(&"dead"));
    assert!(row_keys(section_node(&tree, "combinations")).contains(&"uls"));
    assert!(row_keys(section_node(&tree, "materials")).contains(&"steel"));
    assert!(row_keys(section_node(&tree, "sections")).contains(&"hea200"));
}

/// 📋️ A load case row owns one child row per load, and a combination row one read-only child per
/// term; the case picks at `loadCase` granularity and each load at `load` granularity.
#[semio_framework_async_macros::async_test]
async fn load_case_rows_nest_their_loads_and_combination_rows_nest_their_terms() {
    let document = demo();
    let tree = build(&document, english());
    let cases = section_node(&tree, "load-cases");
    let dead = cases.children.iter().find(|row| row.key.as_str() == "dead").expect("dead case row");
    assert_eq!(row_keys(dead), vec!["l1"]);
    let live = cases.children.iter().find(|row| row.key.as_str() == "live").expect("live case row");
    assert_eq!(row_keys(live), vec!["l2", "l3"]);
    let uls = section_node(&tree, "combinations").children.iter().find(|row| row.key.as_str() == "uls").expect("uls row");
    assert_eq!(row_keys(uls), vec![format!("{TREE_NAMESPACE}.term.uls.dead").as_str(), format!("{TREE_NAMESPACE}.term.uls.live").as_str()]);
    assert_eq!(row_labels(uls), vec!["1.35 dead".to_string(), "1.5 live".to_string()]);
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
    assert!(row_labels(section_node(&tree, "nodes")).contains(&"n20_l1 · (8.00, 0.00, 2.80)".to_string()));
    assert!(row_labels(section_node(&tree, "elements")).contains(&"e1 · Frame n00_g → n00_l1".to_string()));
    assert!(row_labels(section_node(&tree, "solids")).contains(&"sol1 · First Floor Slab · 4 pts · Axis Z".to_string()));
    assert!(row_labels(section_node(&tree, "supports")).contains(&"ss_0 · sc0 · Tx Ty Tz".to_string()));
    assert!(row_labels(section_node(&tree, "load-cases")).contains(&"Dead Load · 1 Load · Self Weight".to_string()));
    assert!(row_labels(section_node(&tree, "load-cases")).contains(&"Live Load · 2 Loads".to_string()));
    assert!(row_labels(section_node(&tree, "combinations")).contains(&"ULS · 1.35 dead + 1.5 live".to_string()));
    assert!(row_labels(section_node(&tree, "materials")).contains(&"Steel S235 · E 210 GPa".to_string()));
    assert!(row_labels(section_node(&tree, "sections")).contains(&"HEA 200 · A 0.00538 m² · Iy 3.69e-5 m⁴".to_string()));

    let live = section_node(&tree, "load-cases").children.iter().find(|row| row.key.as_str() == "live").expect("live case row");
    let loads = row_labels(live);
    assert!(loads.contains(&"Tz −5000 N @ n20_l1".to_string()), "{loads:?}");
    assert!(loads.contains(&"1500 Pa @ sol1".to_string()), "{loads:?}");
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
    assert!(json.contains("Knoten (16)"), "{json}");
    assert!(json.contains("Elemente (16)"));
    assert!(json.contains("Volumenkörper (1)"));
    assert!(json.contains("Lastfälle (2)"));
    assert!(json.contains("Rahmenstab n00_g → n00_l1"));
    assert!(json.contains("Eigengewicht"));
    assert!(json.contains("Anzahl Eigenformen"));
    assert!(!json.contains("Nodes ("));
}
//#endregion 🔖️Labels

//#region 🔖️Interaction
/// 🕹️ A node row carries the framework `interactionSelect` args naming the `"fem3d"` domain, the
/// `node` granularity and the raw id — the same payload a viewport pick sends — and no row actions.
#[semio_framework_async_macros::async_test]
async fn a_node_row_binds_the_interaction_select_args_for_its_own_id() {
    let document = demo();
    let tree = build(&document, english());
    let row = section_node(&tree, "nodes").children.iter().find(|row| row.key.as_str() == "n00_g").expect("n00_g row");
    let binding = row.bindings.iter().next().expect("row binds an action");
    assert_eq!(binding.action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    assert_eq!(binding.action.scope.as_str(), crate::editor::fem3d::FEM3D_PLAY_CONTROLLER_ID);
    let Component::TreeItem(props) = &row.component else { panic!("tree item") };
    assert!(props.row_actions.is_empty(), "a row authors its pick only — focus and delete live in the inspector");
    assert_eq!(props.description.as_ref().map(|text| text.as_str()), Some("Node"));
    let json = projection(build(&document, english()));
    assert!(json.contains("\\\"granularity\\\":\\\"node\\\",\\\"id\\\":\\\"n00_g\\\""), "the pick targets carry the raw id: {json}");
    assert!(!json.contains("focusEntity") && !json.contains("removeSelection"), "the tree carries no row actions: {json}");
}

/// 🎯️ Selection and hover are recorded on the built tree from the live interaction snapshot, and a
/// selection wider than one fixed list marks its first page instead of refusing the render.
#[semio_framework_async_macros::async_test]
async fn selected_and_hovered_ids_are_marked_from_the_interaction_snapshot() {
    let document = demo();
    let interaction = Fem3dInteractionSnapshot { selected_ids: vec!["n00_g".into(), "e1".into()], hovered_ids: vec!["s_00".into()] };
    let tree = render(&document, &interaction, english()).expect("marked tree");
    let Component::Tree(props) = &tree.component else { panic!("panel tree") };
    assert_eq!(props.interaction_domain.as_ref().map(|domain| domain.as_str()), Some(FEM3D_INTERACTION_DOMAIN));
    let wide: Vec<String> = (0..80).map(|index| format!("n{index}")).collect();
    assert_eq!(marked_ids(&wide).len(), MARKED_IDS_LIMIT, "a wider selection marks its first page rather than refusing the render");
    let interaction = Fem3dInteractionSnapshot { selected_ids: wide, hovered_ids: Vec::new() };
    assert!(render(&document, &interaction, english()).is_ok(), "an oversized selection never faults the panel");
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

//#region 🔖️Paging
/// 🪙️ The quota split is max-min fair: a section that fits under the page's ceiling keeps ALL its
/// rows and only the widest sections truncate.
#[semio_framework_async_macros::async_test]
async fn section_quotas_are_max_min_fair() {
    assert_eq!(section_quotas([1, 1, 1, 1, 1, 1, 1, 1, 0], 31), [1, 1, 1, 1, 1, 1, 1, 1, 0], "a document inside the page keeps every row");
    let quotas = section_quotas([60, 2, 2, 2, 2, 2, 2, 2, 0], 31);
    assert_eq!(quotas[1..8], [2, 2, 2, 2, 2, 2, 2], "the small sections stay whole");
    assert_eq!(quotas[0], 31 - 14, "the one wide section absorbs the whole deficit");
    let quotas = section_quotas([60, 60, 60, 60, 60, 60, 60, 60, 0], 31);
    assert!(quotas[..8].iter().all(|quota| *quota >= 3), "no section that wants rows is starved: {quotas:?}");
    assert!(quotas.iter().sum::<usize>() <= 31);
    assert_eq!(section_quotas([0; SECTIONS], 31), [0; SECTIONS]);
}

/// 🪙️ A document an order of magnitude past one argument-arena page closes each truncated section
/// with a `+N` row that accounts for every entity it left out — never an admission fault.
#[semio_framework_async_macros::async_test]
async fn an_oversized_document_pages_with_continuation_rows_instead_of_faulting() {
    let mut document = demo();
    document.nodes = (0..60).map(|index| FemNode { id: format!("g{index}"), x: index as f64, y: 0.0, z: 0.0 }).collect();
    document.load_cases.push(FemLoadCase {
        id: "wind".into(),
        name: "Wind".into(),
        loads: (0..40).map(|index| FemLoad::Nodal { id: format!("wl{index}"), node_id: "g0".into(), dof: FemDof::Tx, value: 1000.0 }).collect(),
        self_weight: false,
    });
    let tree = render(&document, &Fem3dInteractionSnapshot::default(), english()).expect("an oversized document still assembles");
    let nodes = section_node(&tree, "nodes");
    assert!(omitted(nodes, "nodes") > 0, "60 nodes do not fit one page");
    assert_eq!(placed(nodes, "nodes") + omitted(nodes, "nodes"), 60, "the continuation row accounts for every omitted node");
    let wind = section_node(&tree, "load-cases").children.iter().find(|row| row.key.as_str() == "wind").expect("wind case row");
    assert_eq!(placed(wind, "load-cases.wind") + omitted(wind, "load-cases.wind"), 40, "a case past the page pages its own loads");
    for suffix in SECTION_SUFFIXES {
        let node = section_node(&tree, suffix);
        assert!(node.children.len() <= semio_framework_ui_contract::UI_FIXED_LIST_ITEMS, "section {suffix} stays inside the fixed list");
        assert!(!node.children.is_empty(), "section {suffix} is never empty");
    }
    let total = 1 + tree.children.iter().map(|section| 1 + section.children.iter().map(|row| 1 + row.children.len()).sum::<usize>()).sum::<usize>();
    assert!(total < semio_framework_ui_contract::UI_DOCUMENT_NODES, "the whole tree stays inside one UI document: {total}");
    let _ = projection(tree);
}

/// 🌱️ An empty document is a readable tree of "(none)" placeholders, not a refusal.
#[semio_framework_async_macros::async_test]
async fn an_empty_document_renders_placeholders() {
    let tree = render(&Fem3dSnapshot::default(), &Fem3dInteractionSnapshot::default(), english()).expect("empty document tree");
    for suffix in ["nodes", "elements", "solids", "supports", "load-cases", "combinations", "materials", "sections"] {
        assert_eq!(row_labels(section_node(&tree, suffix)), vec!["(none)".to_string()], "section {suffix}");
    }
    let json = projection(tree);
    assert!(json.contains("Nodes (0)"), "{json}");
}
//#endregion 🔖️Paging
