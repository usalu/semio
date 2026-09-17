use super::*;
use crate::editor::cad::config::CadConfig;
use crate::editor::cad::forest_working_scene;
use crate::editor::cad::terminology::cad_labels;
use crate::editor::cad::unit_tests::context::*;
use crate::editor::cad::{make_object_for_typology, CadPlayApp, CadPlayRuntime};
use crate::standards::v1::subsets::any::io::geometry_import::CadPrimitiveSlot;
use crate::standards::v1::subsets::any::schema::inferences::{default_document, forest_play_scene, CAD_MODEL_DEFINITION_SHAPE};
use crate::{CadNode, CadPaneId};
use semio_framework_plugin::{ArtifactView, Locale, TreeWindowRequest, ViewModel, INTERACTION_SELECT_ACTION_ID};

//#region 🧪️Harness
/// 🪟️ A host view state whose first-paint budget is wide enough that every container materialises
/// its whole list — the laws below that are about content, not about windowing, read this.
fn wide_view_state(locale: Locale) -> ViewModel {
    ViewModel { locale, tree_viewport_rows: Some(1024), ..ViewModel::default() }
}

/// 🪟️ One host window request against this panel's body.
fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: CAD_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
}

fn windowed(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..ViewModel::default() }
}

fn child<'a>(parent: &'a BuiltNode, key: &str) -> &'a BuiltNode {
    parent.children.iter().find(|child| child.key.as_str() == key).unwrap_or_else(|| panic!("child {key} of {}", parent.key))
}

/// 🪟️ The `TreeWindow` a section or item stamped — every container the builder emits carries one
/// whenever it has entries, so the host always learns the full extent.
fn window_of(node: &BuiltNode) -> ui::TreeWindow {
    match &node.component {
        semio_framework_plugin::Component::TreeSection(props) => props.window.unwrap_or_else(|| panic!("section {} stamps a window", node.key)),
        semio_framework_plugin::Component::TreeItem(props) => props.window.unwrap_or_else(|| panic!("item {} stamps a window", node.key)),
        _ => panic!("{} is not a windowed container", node.key),
    }
}

fn row_keys(container: &BuiltNode) -> Vec<&str> {
    container.children.iter().map(|row| row.key.as_str()).collect()
}

fn tree_of(scene: CadSnapshot, view_state: &ViewModel) -> BuiltNode {
    build_document_tree(&view(scene, CadPlayRuntime::default()), cad_labels(view_state), &TreeWindows::for_body(view_state, CAD_PLAY_BODY_ARTIFACT)).expect("document tree")
}

fn fixture_json(node: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: node }).expect("fixture projection")
}

/// 🌲️ The forest document with `extra` synthetic nodes appended — an oversized container whose
/// entry count this crate owns outright.
fn oversized_scene(extra: usize) -> CadSnapshot {
    let mut scene = forest_play_scene();
    for index in 0..extra {
        scene.nodes.push(CadNode { id: format!("node-bulk-{index}"), label: format!("Bulk {index}"), kind: "group".into() });
    }
    scene
}
//#endregion 🧪️Harness

#[semio_framework_async_macros::async_test]
async fn document_lists_nodes() {
    let app = CadPlayApp::default();
    let scene = forest_play_scene();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let node = render_direct(&app, CAD_PLAY_BODY_ARTIFACT, &doc, &CadConfig::default(), &wide_view_state(Locale::En)).expect("CAD UI assembly");
    let json = fixture_json(node);
    assert!(json.contains("cad-node:node-root"), "{json}");
    assert!(json.contains("Concrete Forest Left"));
}

/// 🪆️ Every pane's objects come from its composed child's local owner, row ids are the raw object
/// ids the `"cad"` domain (which the tree is bound to) marks selection by, and each pane section
/// stamps the pane's FULL object count as its `window.total` — the host's scrollbar spans the whole
/// document even when only the visible slice was materialised.
#[semio_framework_async_macros::async_test]
async fn document_lists_every_pane_object_bound_to_the_cad_domain() {
    let working = forest_working_scene();
    let view_state = wide_view_state(Locale::En);
    let tree = tree_of(forest_play_scene(), &view_state);
    let semio_framework_plugin::Component::Tree(props) = &tree.component else { panic!("panel tree") };
    assert_eq!(props.interaction_domain.as_ref().map(|domain| domain.as_str()), Some(CAD_INTERACTION_DOMAIN));
    let panes: [(&str, &[CadObject]); 4] =
        [("shape", &working.objects), ("building", &working.building_objects), ("energy", &working.energy_objects), ("structure-classic", &working.structure_classic_objects)];
    let mut listed = 0;
    for (suffix, objects) in panes {
        let section = child(&tree, &format!("cad-play-document.{suffix}"));
        let window = window_of(section);
        assert_eq!(window.total as usize, objects.len(), "pane {suffix} reports its full object count");
        assert_eq!(window.offset, 0, "pane {suffix} starts at the top without a host request");
        let rows = row_keys(section);
        assert!(rows.len() <= objects.len(), "pane {suffix} never materialises more than it has: {rows:?}");
        assert!(rows.iter().all(|key| objects.iter().any(|object| object.id == **key)), "object rows use raw domain ids: {rows:?}");
        listed += rows.len();
    }
    assert!(listed > 0, "a wide viewport lists objects");
    let json = fixture_json(tree);
    assert!(json.contains("interactionSelect"), "rows pick through the framework domain");
    assert!(!json.contains(".more"), "no continuation row survives virtualisation: {json}");
}

/// 🪟️ Law (a): an oversized container stamps `window.total == entries.len()` and materialises at
/// most the slice the window names — never a `+N` row, never a truncation the host cannot see past.
#[semio_framework_async_macros::async_test]
async fn an_oversized_document_stamps_the_full_total_and_materialises_only_its_slice() {
    let scene = oversized_scene(300);
    let nodes = scene.nodes.len();
    assert!(nodes > ui::UI_BUILT_CHILDREN_MAX, "this law needs a container past one built page, found {nodes}");
    let tree = tree_of(scene, &ViewModel::default());
    let section = child(&tree, "cad-play-document.nodes");
    let window = window_of(section);
    assert_eq!(window.total as usize, nodes, "the section reports every node it logically holds");
    assert!(section.children.len() <= ui::UI_BUILT_CHILDREN_MAX, "a built node fans out at most one host child list: {}", section.children.len());
    assert!(section.children.len() < nodes, "the first paint materialises a window, not the document");
    let json = fixture_json(tree);
    assert!(!json.contains(".more"), "no continuation key survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
}

/// 🪟️ Law (b): a container the host closed stamps its total and materialises zero children — the
/// row is expandable, its children simply were never built.
#[semio_framework_async_macros::async_test]
async fn a_closed_container_stamps_its_total_without_children() {
    let working = forest_working_scene();
    let view_state = windowed(vec![request("cad-play-document.shape", Some(false), 0, 32)]);
    let tree = tree_of(forest_play_scene(), &view_state);
    let shape = child(&tree, "cad-play-document.shape");
    assert_eq!(window_of(shape).total as usize, working.objects.len(), "a closed section still reports its extent");
    assert!(shape.children.is_empty(), "a closed section builds no rows: {:?}", row_keys(shape));
    // 🔽️ The reference sections are author-collapsed (`default_open: false`), the same law by the
    // other door: the host has never opened them, so nothing under them is built.
    let references = child(&tree, &format!("cad-play-document.references.{CAD_MODEL_DEFINITION_SHAPE}"));
    assert!(references.children.is_empty(), "an author-collapsed section builds no rows: {:?}", row_keys(references));
}

/// 🪟️ Law (c): a `TreeWindowRequest{offset, rows}` materialises exactly entries `[offset, offset +
/// rows)`, keyed by the raw domain id.
#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_slice_keyed_by_the_raw_id() {
    let working = forest_working_scene();
    let objects = &working.structure_classic_objects;
    assert!(objects.len() >= 4, "this law needs a pane with a few objects, found {}", objects.len());
    let offset = 2u32;
    let rows = 2u32;
    let view_state = windowed(vec![request("cad-play-document.structure-classic", Some(true), offset, rows)]);
    let tree = tree_of(forest_play_scene(), &view_state);
    let section = child(&tree, "cad-play-document.structure-classic");
    let window = window_of(section);
    assert_eq!(window.total as usize, objects.len());
    assert_eq!(window.offset, offset, "the stamped offset is the one the host asked for");
    let expected: Vec<&str> = objects[offset as usize..(offset + rows) as usize].iter().map(|object| object.id.as_str()).collect();
    assert_eq!(row_keys(section), expected, "exactly the requested slice, in document order");
}

/// 🕹️ Law (d): a domain pick row carries `granularity` and NO binding of its own; the one
/// `interactionSelect` lives on the tree root, so a pane of objects costs a single argument map.
#[semio_framework_async_macros::async_test]
async fn object_rows_are_domain_picks_without_a_row_binding() {
    let view_state = wide_view_state(Locale::En);
    let tree = tree_of(forest_play_scene(), &view_state);
    let activates = tree.bindings.iter().filter(|binding| binding.action.name.as_str() == INTERACTION_SELECT_ACTION_ID).count();
    assert_eq!(activates, 1, "the tree root carries exactly one interactionSelect binding");
    let section = child(&tree, "cad-play-document.shape");
    let row = section.children.iter().next().expect("the shape pane lists objects");
    assert!(row.bindings.is_empty(), "a pick row binds nothing of its own, found {}", row.bindings.len());
    let semio_framework_plugin::Component::TreeItem(props) = &row.component else { panic!("object row is a tree item") };
    assert_eq!(props.granularity.as_ref().map(|granularity| granularity.as_str()), Some(edit::CAD_WORLD_PICK_GRANULARITY), "the row names the domain granularity it picks at");
}

/// 🧾️ Every node record one body spends — the tree root, every section, every object group and every
/// leaf row counts once, exactly as `SurfaceReconcileLimits::max_nodes` counts them.
fn node_records(node: &BuiltNode) -> usize {
    1 + node.children.iter().map(node_records).sum::<usize>()
}

/// 🔑️ Every node key in one body. A windowed container is addressed by its authored key, so two
/// containers (or two rows) sharing one key inside a body make the host's `TreeWindowRequest`
/// ambiguous — the SDK refuses it.
fn node_keys(node: &BuiltNode, keys: &mut Vec<String>) {
    keys.push(node.key.as_str().to_string());
    for child in node.children.iter() {
        node_keys(child, keys);
    }
}

/// 🧾️ The whole-document law: the nodes container alone holds more entries than a whole
/// `UI_DOCUMENT_NODES` arena, AND all nine sections plus every object group are open at once, each
/// asking for far more rows than the arena could hold. Per-container clamps do not save this body —
/// only the SDK's body-wide node ledger does. Every non-empty container must still stamp its FULL
/// total, the whole body must reconcile inside `UI_DOCUMENT_NODES` records, and no container may be
/// closed off with a `+N`.
#[semio_framework_async_macros::async_test]
async fn a_whole_open_document_stamps_every_total_and_stays_inside_the_body_node_ceiling() {
    let working = forest_working_scene();
    let scene = oversized_scene(300);
    let nodes = scene.nodes.len();
    assert!(nodes > ui::UI_DOCUMENT_NODES, "one container must hold more than the whole node arena, found {nodes}");
    let panes: [(&str, &[CadObject]); 4] =
        [("shape", &working.objects), ("building", &working.building_objects), ("energy", &working.energy_objects), ("structure-classic", &working.structure_classic_objects)];

    let mut requests = vec![request("cad-play-document.nodes", Some(true), 0, 512)];
    for (suffix, objects) in panes {
        requests.push(request(&format!("cad-play-document.{suffix}"), Some(true), 0, 512));
        // 🔑️ A nested container is addressed by its window PATH — the enclosing section's key, the
        // separator, then its own key — so the same object id under two pane sections stays distinct.
        requests.extend(objects.iter().map(|object| request(&format!("cad-play-document.{suffix}{}{}", ui::TREE_WINDOW_PATH_SEPARATOR, object.id), Some(true), 0, 512)));
    }
    for model_definition_id in [CAD_MODEL_DEFINITION_SHAPE, CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC] {
        requests.push(request(&format!("cad-play-document.references.{model_definition_id}"), Some(true), 0, 512));
    }
    let tree = tree_of(scene, &windowed(requests));

    assert_eq!(window_of(child(&tree, "cad-play-document.nodes")).total as usize, nodes, "the nodes section stamps its FULL total however few rows it could afford");
    for (suffix, objects) in panes {
        let section = child(&tree, &format!("cad-play-document.{suffix}"));
        assert_eq!(window_of(section).total as usize, objects.len(), "pane {suffix} stamps its FULL total");
    }

    let records = node_records(&tree);

    // 🔑️ One body, one key per node: a duplicate would make a host window request ambiguous.
    let mut keys = Vec::new();
    node_keys(&tree, &mut keys);
    let mut unique = keys.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), keys.len(), "every node key in this body is unique ({} of {} distinct)", unique.len(), keys.len());
    assert!(records <= ui::UI_DOCUMENT_NODES, "the whole open document must reconcile inside one surface arena, spent {records} of {}", ui::UI_DOCUMENT_NODES);
    let json = fixture_json(tree);
    assert!(!json.contains(".more"), "no continuation row closes an exhausted container: {json}");
    assert!(!json.contains("\"+"), "and no `+N` label either: {json}");
}

#[semio_framework_async_macros::async_test]
async fn object_tree_item_shows_name_with_kind_as_secondary_label() {
    let mut object = make_object_for_typology("building.building.beam", 0, CadPaneId::Shape);
    object.label = "U2".into();
    let windows = TreeWindows::unhosted();
    let labels = cad_labels(&ViewModel::default());
    let item = object_tree_item(&windows, "shape", &object, labels).expect("object tree item");
    let semio_framework_plugin::Component::TreeItem(props) = &item.component else {
        panic!("expected tree item");
    };
    assert_eq!(props.label.0.as_str(), "U2");
    assert_eq!(props.description.as_ref().map(|text| text.as_str()), Some("Beam"));

    // 🔑️ A second render is a second body: one `TreeWindows` may see a given node key only once
    // (`ui.tree-window.duplicate-key`), so the German paint gets its own ledger, as `render_body` does.
    let de_view = ViewModel { locale: Locale::De, ..ViewModel::default() };
    let de_labels = cad_labels(&de_view);
    let de_windows = TreeWindows::unhosted();
    let de_item = object_tree_item(&de_windows, "shape", &object, de_labels).expect("German object tree item");
    let semio_framework_plugin::Component::TreeItem(props) = &de_item.component else {
        panic!("expected German tree item");
    };
    assert_eq!(props.description.as_ref().map(|text| text.as_str()), Some("Träger"));
}

/// 🪟️ An object row is an author-collapsed container: it always publishes how many primitives it
/// holds, and it builds them only once the host opens it — law (b) at item level.
#[semio_framework_async_macros::async_test]
async fn object_tree_item_streams_its_primitive_children_on_expand() {
    let mut object = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    object.primitives = vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: "solid-1".into(), kind: "solid".into() }];
    let labels = cad_labels(&ViewModel::default());
    let collapsed = object_tree_item(&TreeWindows::unhosted(), "shape", &object, labels).expect("collapsed object row");
    assert_eq!(collapsed.key.as_str(), object.id.as_str(), "object rows are keyed by the raw domain id");
    assert_eq!(window_of(&collapsed).total, 1, "a collapsed object row still reports its primitive count");
    assert!(collapsed.children.is_empty(), "a collapsed object row builds no primitive rows: {:?}", row_keys(&collapsed));

    let view_state = windowed(vec![request(object.id.as_str(), Some(true), 0, 8)]);
    let opened = object_tree_item(&TreeWindows::for_body(&view_state, CAD_PLAY_BODY_ARTIFACT), "shape", &object, labels).expect("opened object row");
    assert_eq!(window_of(&opened).total, 1);
    let json = fixture_json(opened);
    assert!(json.contains("cad-primitive:"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn document_tree_selected_and_highlighted_ids_are_none_without_a_reference_selection() {
    let scene = default_document();
    let runtime = CadPlayRuntime::default();
    assert_eq!(document_tree_selected_ids(&scene, &runtime).expect("selection assembly"), None);
    assert_eq!(document_tree_highlighted_ids(&scene, &runtime).expect("highlight assembly"), None);
}

#[semio_framework_async_macros::async_test]
async fn document_tree_selected_ids_resolves_reference_selection() {
    let scene = forest_play_scene();
    let runtime = CadPlayRuntime { selected_reference_model_definition_id: Some(CAD_MODEL_DEFINITION_SHAPE.into()), selected_reference_id: Some("ref-concrete-forest".into()), ..CadPlayRuntime::default() };
    let selected = document_tree_selected_ids(&scene, &runtime).expect("selection assembly").expect("selected");
    assert!(selected.iter().any(|id| id == "cad-reference:spatial.shape:ref-concrete-forest"));
}

#[semio_framework_async_macros::async_test]
async fn cad_labels_translate_document_tree_panes_in_german() {
    let app = CadPlayApp::default();
    let scene = default_document();
    let history = empty_history();
    let doc = ArtifactView::new(&scene, &history);
    let config = CadConfig::default();
    let node = render_direct(&app, CAD_PLAY_BODY_ARTIFACT, &doc, &config, &wide_view_state(Locale::De)).expect("CAD UI assembly");
    let json = fixture_json(node);
    assert!(json.contains("\"Form\""));
    assert!(json.contains("Gebäude"));
    assert!(json.contains("Energie"));
    assert!(json.contains("Tragwerk Klassisch"));
    assert!(json.contains("Referenzen"));
    assert!(json.contains("\"Knoten\""));
    assert!(!json.contains("\"Shape\""));
    assert!(!json.contains("Struktur Klassisch"));
}
