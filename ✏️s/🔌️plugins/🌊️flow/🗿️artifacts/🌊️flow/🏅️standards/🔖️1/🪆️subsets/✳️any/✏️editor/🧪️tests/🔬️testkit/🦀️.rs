use super::*;
use semio_framework_plugin::testkit::meta;
use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
use semio_s_artifact_stdio_semio::{create_semio_member, SemioMembers};
use store::ArtifactPack;

pub type FlowApp = VcsArtifactApp<EditorApp<FlowPlayApp>, SemioMembers>;

/// 🧪️ Installs a hand-authored `flow.extension` manifest fixture (a "math" module contributing the
/// `math.add` operator) so tests exercising the catalogue/extension surfaces have something real
/// installed — deliberately NOT the production `flow-extension-*` crates: flow-core must not
/// dev-depend on its own extensions (audit finding C1, see ticket
/// `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`'s `w3-flow.md`). Each real extension crate already
/// exhaustively tests its own manifest/operator content in its own `#[cfg(test)] mod tests` (e.g.
/// `flow-extension-math`'s `manifest_lists_math_operators_and_schemas`); this fixture only covers
/// what flow-core's own tests assert on (`catalogue_lists_module_operators`).
fn install_first_party_light_flow_extensions_for_tests() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let manifest = flow::FlowExtensionManifest {
            schema: "flow.extension".into(),
            id: "math".into(),
            name: "Math".into(),
            version: "0.0.0-test-fixture".into(),
            activation_events: vec!["onStartup".into()],
            contributes: flow::FlowExtensionContributes {
                schemas: vec![],
                operators: vec![flow::neural::OperatorInfo { id: "math.add".into(), extension: "math".into(), name: "Add".into(), abbreviation: "Add".into(), ..Default::default() }],
                widgets: vec![],
                commands: vec![],
                settings: vec![],
            },
        };
        let manifest_json = flow::os_pack::json::to_json_string(&manifest);
        flow::install_flow_extension_manifest("flow-core-test-fixture", &manifest_json).expect("fixture extension admission");
    });
}

pub(crate) async fn register_content_child(app: &mut FlowApp) {
    let snapshot = app.snapshot().expect("Flow parent snapshot");
    let fixture = snapshot.to_fixture();
    let content = crate::flow_content_snapshot_from_working(&fixture.widgets, &fixture.synapses, &fixture.layout);
    let dialect = snapshot.content.target.dialect.clone();
    let member = create_semio_member(&snapshot.content.child_id, &dialect, &content.encode_pack()).await.expect("Flow child member");
    app.register_child("content", snapshot.content.child_id, dialect, member).await.expect("register Flow content child");
}

/// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
pub async fn flow_app() -> FlowApp {
    install_first_party_light_flow_extensions_for_tests();
    let mut app = VcsArtifactApp::<EditorApp<FlowPlayApp>, SemioMembers>::new(EditorApp::default()).await;
    register_content_child(&mut app).await;
    app
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
pub async fn flow_app_with_registry() -> FlowApp {
    install_first_party_light_flow_extensions_for_tests();
    let definition = create_flow_app();
    let registry = AppActionRegistry::from_definition(&definition);
    let mut app = VcsArtifactApp::<EditorApp<FlowPlayApp>, SemioMembers>::with_registry(EditorApp::default(), registry).await;
    register_content_child(&mut app).await;
    app
}

pub async fn dispatch(app: &mut FlowApp, command: FlowCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn dispatch_with_registry(app: &mut FlowApp, command: FlowCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut FlowApp, body_key: &str) -> String {
    let tree = app.render(body_key, None, &ViewModel::default()).await.expect("render");
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement")
}

fn projection_fixture_node(value: &Value) -> semio_framework_plugin::BuiltNode {
    let mut node = semio_framework_plugin::BuiltNode::try_new(value["key"].as_str().unwrap(), serde_json::from_value(value["component"].clone()).unwrap()).unwrap();
    for child in value["children"].as_array().unwrap() {
        node.children.try_push(projection_fixture_node(child)).unwrap();
    }
    if value["rejected"].as_bool() == Some(true) {
        node.rejected_children.try_push(semio_framework_plugin::BuiltNode::try_new("rejected", serde_json::from_value(serde_json::json!({ "type": "text", "value": "rejected" })).unwrap()).unwrap()).unwrap();
    }
    node
}

fn saturated_rejected_fixture_tree(pages: usize, capacity: usize) -> semio_framework_plugin::ComponentTree {
    use semio_framework_ui_contract::{BuiltNode, Component, Label, TextProps};
    let text = |key: &str| BuiltNode::try_new(key, Component::Text(TextProps { value: Label::try_from("fixture").unwrap(), emphasize: None, data_attributes: None })).unwrap();
    let mut root = text("root");
    let mut full_pages = pages.checked_sub(capacity + 2).unwrap();
    for group in 0..capacity {
        let mut branch = text(&format!("group-{group}"));
        for row in 0..capacity {
            let mut node = text(&format!("row-{row}"));
            if full_pages > 0 {
                for leaf in 0..capacity {
                    node.children.try_push(text(&format!("leaf-{leaf}"))).unwrap();
                }
                full_pages -= 1;
            }
            branch.children.try_push(node).unwrap();
        }
        root.children.try_push(branch).unwrap();
    }
    assert_eq!(full_pages, 0);
    root.rejected_children.try_push(text("rejected")).unwrap();
    semio_framework_plugin::ComponentTree { root }
}

#[test]
fn flow_render_fixture_projection_retires_populated_and_rejected_pages() {
    use semio_framework_plugin::testkit::{project_and_retire_fixture_tree, FIXTURE_TREE_MAX_DEPTH, FIXTURE_TREE_MAX_NODES, FIXTURE_TREE_RETIRE_STEPS};
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖼️tree-projection/🔣️.json")).unwrap();
    assert_eq!(fixture["contractId"], "semio.fixture.tree-projection/v1");
    assert_eq!(fixture["maximumDepth"], FIXTURE_TREE_MAX_DEPTH);
    assert_eq!(fixture["maximumNodes"], FIXTURE_TREE_MAX_NODES);
    assert_eq!(fixture["retirementSteps"], FIXTURE_TREE_RETIRE_STEPS);
    for row in fixture["cases"].as_array().unwrap() {
        let tree = semio_framework_plugin::ComponentTree { root: projection_fixture_node(&row["input"]) };
        match project_and_retire_fixture_tree(tree) {
            Ok(json) => assert_eq!(serde_json::from_str::<Value>(&json).unwrap(), row["expected"]),
            Err(error) => assert_eq!(Some(error), row["error"].as_str()),
        }
        assert!(semio_framework_ui_contract::close_built_node_page_one());
    }
    let probe = &fixture["retirementProbe"];
    let pages = probe["reservedPages"].as_u64().unwrap() as usize;
    let capacity = probe["childCapacity"].as_u64().unwrap() as usize;
    assert_eq!(pages, semio_framework_ui_contract::UI_BUILT_CHILD_RETIRE_SLOTS);
    assert_eq!(capacity, semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX);
    drop(saturated_rejected_fixture_tree(pages, capacity));
    let steps = (1..=FIXTURE_TREE_RETIRE_STEPS).find(|_| semio_framework_ui_contract::close_built_node_page_one()).expect("all reserved pages retire within the exact authority bound");
    assert_eq!(steps, probe["closeSteps"].as_u64().unwrap() as usize);
    assert_eq!(steps - pages, probe["ownedNodes"].as_u64().unwrap() as usize);
    assert!(steps > probe["supersededLimit"].as_u64().unwrap() as usize);
    let tree = saturated_rejected_fixture_tree(pages, capacity);
    assert_eq!(project_and_retire_fixture_tree(tree).unwrap_err(), probe["error"].as_str().unwrap());
    assert_eq!(semio_framework_ui_contract::close_built_node_page_one(), probe["terminalEmpty"].as_bool().unwrap());
    eprintln!("[DEBUG] retained Flow fixture tree observation: two positive trees, two structural denials and {pages} saturated pages retired in {steps} counted turns");
}

pub async fn main_window_measures(app: &mut FlowApp) -> Vec<WindowMeasure> {
    app.window_measures(&ViewModel::default()).await.get(main::FLOW_PLAY_WINDOW_MAIN).cloned().expect("main window measures")
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is the framework's injected
/// `interactionSelect` verb now, dispatched against the "graph" domain declared on this app —
/// requires `flow_app_with_registry()` (a bare `flow_app()` has no declared interaction domains to
/// select against). `node_ids`/`edge_ids` are raw widget/synapse ids, converted to the row-id-
/// prefixed `InteractionTarget` ids the document panel tree/`interaction_topology` both use (see
/// `flow_graph_node_target_id`/`flow_graph_edge_target_id`).
pub async fn select_graph(app: &mut FlowApp, node_ids: &[&str], edge_ids: &[&str]) {
    let mut targets: Vec<Value> = node_ids.iter().map(|id| serde_json::json!({ "granularity": "node", "id": flow_graph_node_target_id(id) })).collect();
    targets.extend(edge_ids.iter().map(|id| serde_json::json!({ "granularity": "edge", "id": flow_graph_edge_target_id(id) })));
    let targets_json = serde_json::to_string(&targets).expect("targets json");
    let args = dsl::DslValue::from(serde_json::json!({ "domainId": FLOW_INTERACTION_GRAPH, "targets": targets_json, "merge": "replace" }));
    app.handle_action("interactionSelect", Some(&args), &meta("test")).await.expect("interactionSelect");
}
