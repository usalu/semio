//! 🔺 Trinity plugin — Jack and Rewrite apps in one hot-swappable WASM component.

pub mod app_jack {
    //! 🔱 Trinity Jack plugin — jack query play app bundled as a hot-swappable WASM component.

    use semio_framework_plugin::{SurfaceKind, PanelGroup,
        app_labels, build_node_graph_scene, build_table_scene, build_text_editor_scene,
        is_de_locale, localized_label_map, resolve_labels, text_identifier_occurrences_json, tree_item, tree_item_with_action,
        ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_text,
        ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, ActionDescriptor, AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp,
        DocumentView, MeasureSelectItem, NodeGraphScene, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelTreeBuilder, ResourceKindSpec,
        TableScene, TextEditorScene, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiTreeItemNode,
        ViewState, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild,
        WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
        FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
        FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
    };
    use serde::Serialize;
    use serde_json::{json, Value};
    use std::collections::{BTreeMap, HashMap};
    use trinity_jack::{complete, execute, format as jack_format, lint, parse, semantic_tokens, QueryResult, QueryResultKind};
    use trinity_ram::{Camera, Graph, GraphFixture, Node, PortDirection, PropertyValue, TrinityGraphOperation, TRINITY_GRAPH_SCHEMA};
    use store::DocumentDsl;

    //#region 🔖Constants
    const TRINITY_JACK_PLAY_APP_ID: &str = "trinity-jack-play";
    const TRINITY_JACK_PLAY_CONTROLLER_ID: &str = "trinity-jack-play";
    const TRINITY_JACK_PLAY_SURFACE_GRAPH: &str = "trinity.jack.play";
    const TRINITY_JACK_PLAY_SURFACE_EDITOR: &str = "trinity.jack.editor";
    const TRINITY_JACK_PLAY_SURFACE_RESULTS: &str = "trinity.jack.results";
    const TRINITY_JACK_PLAY_BODY_GRAPH: &str = "trinity.jack.play.main";
    const TRINITY_JACK_PLAY_BODY_EDITOR: &str = "trinity.jack.play.editor";
    const TRINITY_JACK_PLAY_BODY_RESULTS: &str = "trinity.jack.play.results";
    const TRINITY_JACK_PLAY_BODY_DOCUMENT: &str = "trinity.jack.play.document";
    const TRINITY_JACK_PLAY_BODY_CATALOGUE: &str = "trinity.jack.play.catalogue";
    const TRINITY_JACK_PLAY_BODY_INSPECTION: &str = "trinity.jack.play.inspection";
    const TRINITY_JACK_PLAY_WINDOW_GRAPH: &str = "trinity-jack-graph";
    const TRINITY_JACK_PLAY_WINDOW_EDITOR: &str = "trinity-jack-editor";
    const TRINITY_JACK_PLAY_WINDOW_RESULTS: &str = "trinity-jack-results";

    const NAKAGIN_FIXTURE_DSL: &str = include_str!("../../example/nakagin-capsule-tower.trinity");
    const BRANCH_FIXTURE_DSL: &str = include_str!("../../example/branch-chain.trinity");

    const TRINITY_JACK_DEFAULT_QUERY: &str =
        "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label";

    const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";
    //#endregion 🔖Constants

    //#region 🔖Types
    /// 🎯 Ephemeral editor selection range (offsets into the jack query text) — pure runtime.
    #[derive(Clone, Debug, Default, PartialEq)]
    struct TrinityEditorSelection {
        start: usize,
        end: usize,
    }

    /// 🎛️ Ephemeral view state (selection, query draft, results, LOD, engagement inputs) — lives on the
    /// app struct, never in the document, so it never pollutes undo history. The document projection is
    /// the bare {@link GraphFixture}.
    #[derive(Clone, Debug, Default, PartialEq)]
    struct TrinityJackRuntime {
        selected_node_ids: Vec<String>,
        active_fixture_id: String,
        jack_query: String,
        jack_result_json: String,
        editor_engagement_input: String,
        graph_engagement_input: String,
        results_engagement_input: String,
        reorganize_epoch: u64,
        editor_selection: Option<TrinityEditorSelection>,
        lod_mode_by_window: BTreeMap<String, String>,
        revision: u64,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct WorkflowDiagramPortRecord {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct WorkflowNodeRecord {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        inputs: Vec<WorkflowDiagramPortRecord>,
        outputs: Vec<WorkflowDiagramPortRecord>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct WorkflowEdgeRecord {
        id: String,
        source_node_id: String,
        source_port_id: String,
        target_node_id: String,
        target_port_id: String,
    }
    //#endregion 🔖Types

    //#region 🔖DocumentHelpers
    /// 📦 The default trinity graph fixture (Nakagin capsule tower) — the initial document projection.
    fn default_fixture() -> GraphFixture {
        GraphFixture::parse_dsl(NAKAGIN_FIXTURE_DSL).unwrap_or_else(|_| trinity_ram::empty_trinity_graph_fixture())
    }

    /// 🌱 Seeds the runtime with the default query and its result table so the Results window is populated on load.
    fn seeded_jack_runtime() -> TrinityJackRuntime {
        let (result_json, _) = run_jack_query(&default_fixture(), TRINITY_JACK_DEFAULT_QUERY);
        TrinityJackRuntime {
            active_fixture_id: "nakagin".into(),
            jack_query: TRINITY_JACK_DEFAULT_QUERY.into(),
            jack_result_json: result_json,
            ..Default::default()
        }
    }

    /// 🔎 Runs a jack query against the fixture, returning `(result_json, forward operations)`; a parse/execute
    /// failure yields an error result and no operations (no document mutation).
    fn run_jack_query(fixture: &GraphFixture, query: &str) -> (String, Vec<TrinityGraphOperation>) {
        let graph = match Graph::from_fixture(fixture.clone()) {
            Ok(graph) => graph,
            Err(error) => return (error_result_json(&error.to_string()), Vec::new()),
        };
        let parsed = match parse(query) {
            Ok(parsed) => parsed,
            Err(error) => return (error_result_json(&error), Vec::new()),
        };
        match execute(&graph, &parsed) {
            Ok((result, operations)) => (serde_json::to_string(&result).unwrap_or_default(), operations),
            Err(error) => (error_result_json(&error), Vec::new()),
        }
    }

    fn error_result_json(message: &str) -> String {
        json!({ "error": message }).to_string()
    }

    fn jack_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor {
            controller_id: TRINITY_JACK_PLAY_CONTROLLER_ID.into(),
            action: action.into(),
            args,
        }
    }

    fn graph_from_fixture_or_default(fixture: &GraphFixture) -> Graph {
        Graph::from_fixture(fixture.clone()).unwrap_or_else(|_| Graph::from_fixture(default_fixture()).expect("nakagin graph"))
    }

    /// 🧮 Clones the fixture and recomputes its derived (flat-position) node properties for the inspector.
    fn fixture_with_derived(fixture: &GraphFixture) -> Option<GraphFixture> {
        let mut graph = Graph::from_fixture(fixture.clone()).ok()?;
        graph.recompute_derived();
        Some(graph.to_fixture())
    }

    fn fixture_dsl_for_preset(preset_id: &str) -> Option<&'static str> {
        match preset_id {
            "nakagin" | "nakagin-capsule-tower" => Some(NAKAGIN_FIXTURE_DSL),
            "branch-chain" => Some(BRANCH_FIXTURE_DSL),
            _ => None,
        }
    }

    fn preset_query(preset_id: &str) -> &'static str {
        match preset_id {
            "branch-chain" => "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",
            _ => TRINITY_JACK_DEFAULT_QUERY,
        }
    }

    fn property_value_to_string(value: &PropertyValue) -> String {
        match value {
            PropertyValue::String(text) => text.clone(),
            PropertyValue::Number(number) => number.to_string(),
            PropertyValue::Bool(flag) => flag.to_string(),
            PropertyValue::Null => "null".into(),
            PropertyValue::Array(items) => serde_json::to_string(items).unwrap_or_else(|_| "[]".into()),
            PropertyValue::Object(map) => serde_json::to_string(map).unwrap_or_else(|_| "{}".into()),
        }
    }

    /// 🧲 Re-runs force layout on the fixture, returning the repositioned fixture (or `None` if empty).
    fn force_layout_fixture(fixture: &GraphFixture) -> Option<GraphFixture> {
        let mut fixture = fixture.clone();
        if fixture.nodes.is_empty() {
            return None;
        }
        use mathematical_graph_drawing::force::{run_force_layout, ForceLayoutOptions};
        use mathematical_geometry::Vec2;
        let mut positions: Vec<Vec2> = fixture.nodes.iter().map(|node| Vec2::new(node.x, node.y)).collect();
        let radii: Vec<f64> = fixture.nodes.iter().map(|node| (node.width.max(48.0) + node.height.max(24.0)) * 0.25).collect();
        let id_to_index: std::collections::HashMap<String, usize> = fixture
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.id.clone(), index))
            .collect();
        let mut edge_pairs = Vec::new();
        for edge in &fixture.edges {
            let (source_node, _) = split_endpoint(&edge.source);
            let (target_node, _) = split_endpoint(&edge.target);
            if let (Some(a), Some(b)) = (id_to_index.get(&source_node), id_to_index.get(&target_node)) {
                edge_pairs.push((*a, *b));
            }
        }
        let pin = vec![None; positions.len()];
        run_force_layout(
            &mut positions,
            &radii,
            &edge_pairs,
            &pin,
            &ForceLayoutOptions {
                iterations: 120,
                ..ForceLayoutOptions::default()
            },
        );
        for (index, node) in fixture.nodes.iter_mut().enumerate() {
            node.x = positions[index].x;
            node.y = positions[index].y;
        }
        Some(fixture)
    }

    /// 🧭 Emits a `Reposition` operation for every node whose position differs between `before` and `after`.
    fn reposition_operations(before: &GraphFixture, after: &GraphFixture) -> Vec<TrinityGraphOperation> {
        after
            .nodes
            .iter()
            .filter_map(|node| {
                let prev = before.nodes.iter().find(|entry| entry.id == node.id)?;
                if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
                    Some(TrinityGraphOperation::Reposition { id: node.id.clone(), x: node.x, y: node.y })
                } else {
                    None
                }
            })
            .collect()
    }

    /// 🎯 Selection ids from an action's args — delegates to the SDK's shared `ids`-array reader, falling
    /// back to a singular `nodeIds`/`nodeId` key for actions dispatched from the node-graph scene surface.
    fn selection_ids(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("nodeIds"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .or_else(|| Some(semio_framework_plugin::selection_ids(args)).filter(|ids: &Vec<String>| !ids.is_empty()))
            .or_else(|| {
                args.and_then(|value| value.get("nodeId"))
                    .and_then(|value| value.as_str())
                    .map(|id| vec![id.to_string()])
            })
            .unwrap_or_default()
    }

    fn trinity_lod_tier_rows() -> Vec<Value> {
        serde_json::from_str(&trinity_rewrite::trinity_lod_scale_json()).unwrap_or_default()
    }

    fn trinity_lod_measure(window_id: &str, current_mode: &str) -> WindowMeasure {
        let mut items = vec![MeasureSelectItem {
            id: TRINITY_LOD_MODE_AUTOMATIC.into(),
            value: TRINITY_LOD_MODE_AUTOMATIC.into(),
            label: "Automatic".into(),
        }];
        items.extend(trinity_lod_tier_rows().into_iter().filter_map(|row| {
            let id = row.get("id")?.as_str()?.to_string();
            let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
            Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
        }));
        WindowMeasure::Select {
            id: format!("{window_id}-lod"),
            label: Some("LOD".into()),
            value: current_mode.into(),
            items,
            on_change: jack_action("setLodMode", Some(json!({ "windowId": window_id }))),
        }
    }

    fn trinity_lod_json_for_window(runtime: &TrinityJackRuntime, window_id: &str) -> Option<String> {
        let mode = runtime.lod_mode_by_window.get(window_id).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
        if mode == TRINITY_LOD_MODE_AUTOMATIC {
            Some(json!({ "automatic": true }).to_string())
        } else {
            Some(json!({ "automatic": false, "forcedLabel": mode }).to_string())
        }
    }
    /// 🩹 Delegates to `trinity_ram::parse_port_key` (the one place the `nodeId@portId` convention is
    /// owned) instead of hand-rolling a second splitter here.
    fn split_endpoint(endpoint: &str) -> (String, String) {
        trinity_ram::parse_port_key(endpoint).map_or_else(|| (endpoint.to_string(), "in".into()), |(n, p)| (n.to_string(), p.to_string()))
    }

    fn fixture_to_workflow(fixture: &GraphFixture) -> (String, String, String) {
        let nodes: Vec<WorkflowNodeRecord> = fixture
            .nodes
            .iter()
            .map(|node| node_to_workflow_record(node))
            .collect();
        let edges: Vec<WorkflowEdgeRecord> = fixture
            .edges
            .iter()
            .map(|edge| {
                let (source_node_id, source_port_id) = split_endpoint(&edge.source);
                let (target_node_id, target_port_id) = split_endpoint(&edge.target);
                WorkflowEdgeRecord {
                    id: edge.id.clone(),
                    source_node_id,
                    source_port_id,
                    target_node_id,
                    target_port_id,
                }
            })
            .collect();
        let viewport = serde_json::to_string(&fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
        (
            serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
            serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
            viewport,
        )
    }

    fn node_to_workflow_record(node: &Node) -> WorkflowNodeRecord {
        let width = if node.width > 0.0 { node.width } else { 96.0 };
        let height = if node.height > 0.0 { node.height } else { 48.0 };
        WorkflowNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width,
            height,
            inputs: node
                .ports
                .iter()
                .filter(|port| port.direction == PortDirection::In)
                .map(|port| WorkflowDiagramPortRecord {
                    id: trinity_ram::port_key(&node.id, &port.id),
                    label: Some(port.id.clone()),
                })
                .collect(),
            outputs: node
                .ports
                .iter()
                .filter(|port| port.direction == PortDirection::Out)
                .map(|port| WorkflowDiagramPortRecord {
                    id: trinity_ram::port_key(&node.id, &port.id),
                    label: Some(port.id.clone()),
                })
                .collect(),
        }
    }

    fn result_to_table(result_json: &str) -> (String, String) {
        let parsed: QueryResult = serde_json::from_str(result_json).unwrap_or(QueryResult::table(vec![], vec![]));
        let columns: Vec<Value> = parsed
            .columns
            .iter()
            .map(|column| json!({ "id": column, "label": column }))
            .collect();
        let rows: Vec<Value> = parsed
            .rows
            .iter()
            .enumerate()
            .map(|(index, row)| {
                let mut record = serde_json::Map::new();
                record.insert("index".into(), json!(index + 1));
                for (column, value) in parsed.columns.iter().zip(row.iter()) {
                    record.insert(column.clone(), json!(property_value_to_string(value)));
                }
                Value::Object(record)
            })
            .collect();
        (
            serde_json::to_string(&columns).unwrap_or_else(|_| "[]".into()),
            serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()),
        )
    }
    //#endregion 🔖DocumentHelpers

    //#region 🔖Terminology
    /// 🗣️ Complete UI label set for the Jack query app; one field per label makes every locale combination compile-checked.
    app_labels! {
        struct TrinityJackLabels {
            pieces: &'static str = en: "Pieces", de: "Stücke";
            connections: &'static str = en: "Connections", de: "Verbindungen";
            fixtures: &'static str = en: "Fixtures", de: "Fixturen";
            example_queries: &'static str = en: "Example queries", de: "Beispielabfragen";
            manifest_kinds: &'static str = en: "Manifest kinds", de: "Manifestarten";
            piece: &'static str = en: "Piece", de: "Stück";
            connection: &'static str = en: "Connection", de: "Verbindung";
            connector: &'static str = en: "Connector", de: "Verbinder";
            geometry: &'static str = en: "Geometry", de: "Geometrie";
            identity: &'static str = en: "Identity", de: "Identität";
            history: &'static str = en: "History", de: "Verlauf";
            query: &'static str = en: "Query", de: "Abfrage";
            window_graph: &'static str = en: "Nakagin Graph", de: "Nakagin-Graph";
            window_editor: &'static str = en: "Jack Query", de: "Jack-Abfrage";
            window_results: &'static str = en: "Results", de: "Ergebnisse";
        }
    }
    //#endregion 🔖Terminology

    //#region 🔖CommandLabels
    /// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_trinity_jack_app`'s static
    /// manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command palette
    /// and Actions rail get a translated label without threading locale through the whole builder chain.
    fn trinity_jack_action_labels(is_de: bool) -> HashMap<String, String> {
        localized_label_map(is_de, &[
            ("nodeGraphEdit", "Edit Graph", "Graph bearbeiten"),
            ("nodeGraphViewport", "Set Graph Viewport", "Graph-Ansicht festlegen"),
            ("patchTrinityNodes", "Patch Nodes", "Knoten aktualisieren"),
            ("reorganize", "Reorganize", "Neu anordnen"),
            ("runJackQuery", "Run Jack Query", "Jack-Abfrage ausführen"),
            ("submit", "Submit Jack Query", "Jack-Abfrage absenden"),
            ("loadExampleQuery", "Load Example Query", "Beispielabfrage laden"),
            ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
            ("setSelection", "Set Selection", "Auswahl festlegen"),
            ("selectNode", "Select Node", "Knoten auswählen"),
            ("nodeGraphSelect", "Select Graph Node", "Graph-Knoten auswählen"),
            ("nodeGraphHover", "Hover Graph Node", "Graph-Knoten hovern"),
            ("textEdit", "Edit Jack Query", "Jack-Abfrage bearbeiten"),
            ("textSelect", "Select Jack Query Text", "Jack-Abfragetext auswählen"),
            ("textHover", "Hover Jack Query Text", "Jack-Abfragetext hovern"),
            ("requestCompletions", "Request Completions", "Vervollständigungen anfordern"),
            ("formatDocument", "Format Jack Query", "Jack-Abfrage formatieren"),
            ("setLodMode", "Set LOD Mode", "LOD-Modus festlegen"),
            ("editorEngagementInput", "Editor Engagement Input", "Editor-Eingabe"),
            ("graphEngagementInput", "Graph Engagement Input", "Graph-Eingabe"),
            ("resultsEngagementInput", "Results Engagement Input", "Ergebnis-Eingabe"),
            ("graphPointerDown", "Graph Pointer Down", "Graph-Zeiger gedrückt"),
        ])
    }
    //#endregion 🔖CommandLabels

    //#region 🔖Panels
    fn flat_position_uv(node: &Node) -> (String, String) {
        let Some(flat) = node.properties.get("flatPosition").and_then(PropertyValue::as_object) else {
            return (String::new(), String::new());
        };
        let format_axis = |axis: &str| flat.get(axis).and_then(PropertyValue::as_f64).map(|value| format!("{value:.2}")).unwrap_or_default();
        (format_axis("u"), format_axis("v"))
    }

    fn build_document_tree(fixture: &GraphFixture, runtime: &TrinityJackRuntime, labels: &TrinityJackLabels) -> UiNode {
        let builder = PanelTreeBuilder::new("trinity-document");
        let node_items: Vec<UiTreeItemNode> = fixture
            .nodes
            .iter()
            .map(|node| {
                tree_item_with_action(
                    builder.item_id("node", &node.id),
                    if node.name.is_empty() { node.id.clone() } else { node.name.clone() },
                    Some(node.kind.clone()),
                    jack_action("setSelection", Some(json!({ "ids": [node.id] }))),
                )
            })
            .collect();
        let edge_items: Vec<UiTreeItemNode> = fixture
            .edges
            .iter()
            .map(|edge| tree_item(builder.item_id("edge", &edge.id), format!("{} → {}", edge.source, edge.target)))
            .collect();
        let selected = runtime.selected_node_ids.iter().map(|id| builder.item_id("node", id)).collect();
        builder
            .section("trinity-document.nodes", Some(labels.pieces.into()), true, node_items)
            .section("trinity-document.edges", Some(labels.connections.into()), false, edge_items)
            .selected(selected)
            .selection_change(jack_action("setSelection", Some(json!({ "ids": [] }))))
            .build()
    }

    fn build_catalogue_tree(runtime: &TrinityJackRuntime, labels: &TrinityJackLabels) -> UiNode {
        let fixtures = [("nakagin", "Nakagin — Table"), ("branch-chain", "Branch — Graph")];
        let examples = [
            ("where-or", "Where Or", "MATCH (a:Piece) WHERE a.name = 't_f0_b_c0' OR a.name = 't_f0_b_c1' RETURN a.name"),
            ("return-graph", "Return Graph", "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b"),
            ("set-label", "Set Label", "MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'demo-label'"),
            ("set-position", "Set Position", "MATCH (a:Piece) WHERE a.name = 'b' SET a.x = 300, a.y = 120"),
            ("create-node", "Create Node", "CREATE (n:Piece)"),
            ("create-edge", "Create Edge", "MATCH (a:Piece), (b:Piece) WHERE a.name = 'b' AND b.name != 'b' CREATE (a)-[:Connection]->(b)"),
            ("delete-leaf", "Delete Leaf", "MATCH (n:Piece) WHERE n.name = 'b' DELETE n"),
            ("merge-edge", "Merge Edge", "MERGE (x:Piece)-[:Connection]->(y:Piece)"),
        ];
        let builder = PanelTreeBuilder::new("trinity-jack-catalogue");
        let fixture_items: Vec<UiTreeItemNode> = fixtures
            .iter()
            .map(|(id, label)| {
                tree_item_with_action(
                    builder.item_id("fixture", id),
                    *label,
                    Some(preset_query(id).into()),
                    jack_action("setActiveExample", Some(json!({ "exampleId": id }))),
                )
            })
            .collect();
        let example_items: Vec<UiTreeItemNode> = examples
            .iter()
            .map(|(id, label, query)| {
                tree_item_with_action(
                    builder.item_id("example", id),
                    *label,
                    Some((*query).into()),
                    jack_action("loadExampleQuery", Some(json!({ "query": query }))),
                )
            })
            .collect();
        let selected = if runtime.active_fixture_id.is_empty() { vec![] } else { vec![builder.item_id("fixture", &runtime.active_fixture_id)] };
        builder
            .section("trinity-jack-catalogue.fixtures", Some(labels.fixtures.into()), true, fixture_items)
            .section("trinity-jack-catalogue.examples", Some(labels.example_queries.into()), true, example_items)
            .section(
                "trinity-jack-catalogue.kinds",
                Some(labels.manifest_kinds.into()),
                false,
                vec![
                    tree_item("trinity-jack-catalogue.piece", labels.piece),
                    tree_item("trinity-jack-catalogue.connection", labels.connection),
                    tree_item("trinity-jack-catalogue.connector", labels.connector),
                ],
            )
            .selected(selected)
            .build()
    }

    fn build_inspector_tree(fixture: &GraphFixture, runtime: &TrinityJackRuntime, term_labels: &TrinityJackLabels) -> UiNode {
        if runtime.selected_node_ids.is_empty() {
            return ui_declarative_sections_to_tree(&[UiSectionNode {
                id: "trinity-inspector.empty".into(),
                label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
                default_open: Some(true),
                presence: UiPresence::default(),
                children: vec![ui_text("Select one or more pieces")],
            }]);
        }
        let nodes: Vec<&Node> = runtime
            .selected_node_ids
            .iter()
            .filter_map(|id| fixture.nodes.iter().find(|node| &node.id == id))
            .collect();
        if nodes.is_empty() {
            return ui_declarative_sections_to_tree(&[UiSectionNode {
                id: "trinity-inspector.missing".into(),
                label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
                default_open: Some(true),
                presence: UiPresence::default(),
                children: vec![ui_text("Piece not found")],
            }]);
        }
        let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
        let name_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>());
        let kind_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.kind.clone()).collect::<Vec<_>>());
        let port_counts: Vec<String> = nodes.iter().map(|node| node.ports.len().to_string()).collect();
        let ports_mixed = ui_inspector_mixed_text(&port_counts);
        let derived_fixture = fixture_with_derived(fixture);
        let derived_uv = |id: &str| -> (String, String) {
            derived_fixture
                .as_ref()
                .and_then(|fixture| fixture.nodes.iter().find(|node| node.id == id))
                .map(flat_position_uv)
                .unwrap_or_default()
        };
        let u_values: Vec<String> = node_ids.iter().map(|id| derived_uv(id).0).collect();
        let v_values: Vec<String> = node_ids.iter().map(|id| derived_uv(id).1).collect();
        let u_mixed = ui_inspector_mixed_text(&u_values);
        let v_mixed = ui_inspector_mixed_text(&v_values);
        ui_inspector_groups_to_tree(&[
            UiInspectorFieldGroup { presence: UiPresence::default(),
                id: "trinity-inspector.geometry".into(),
                label: term_labels.geometry.into(),
                default_open: None,
                fields: vec![
                    ui_inspector_readonly_field(
                        "trinity-inspector.flat-u",
                        "Flat U",
                        if u_mixed.placeholder.is_none() {
                            u_values.first().cloned().unwrap_or_default()
                        } else {
                            u_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                        },
                    ),
                    ui_inspector_readonly_field(
                        "trinity-inspector.flat-v",
                        "Flat V",
                        if v_mixed.placeholder.is_none() {
                            v_values.first().cloned().unwrap_or_default()
                        } else {
                            v_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                        },
                    ),
                    ui_inspector_readonly_field(
                        "trinity-inspector.ports",
                        "Connectors",
                        if ports_mixed.placeholder.is_none() {
                            port_counts.first().cloned().unwrap_or_default()
                        } else {
                            ports_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                        },
                    ),
                ],
            },
            UiInspectorFieldGroup {
                presence: UiPresence::default(),
                id: "trinity-inspector.identity".into(),
                label: term_labels.identity.into(),
                default_open: None,
                fields: vec![
                    semio_framework_plugin::UiNode::Field(UiFieldNode {presence: UiPresence::default(), 
                        id: "trinity-inspector.name".into(),
                        label: "Name".into(),
                        child: Box::new(semio_framework_plugin::UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(), 
                            id: "trinity-inspector.name.input".into(),
                            input_kind: "text".into(),
                            value: name_mixed.value,
                            placeholder: name_mixed.placeholder,
                            commit: None,
                            on_change: jack_action(
                                "patchTrinityNodes",
                                Some(json!({ "nodeIds": node_ids, "field": "name" })),
                            ),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                    }),
                    ui_inspector_readonly_field(
                        "trinity-inspector.kind",
                        "Kind",
                        if kind_mixed.placeholder.is_none() {
                            nodes.first().map(|node| node.kind.clone()).unwrap_or_default()
                        } else {
                            kind_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                        },
                    ),
                    ui_inspector_readonly_field(
                        "trinity-inspector.id",
                        "Id",
                        if node_ids.len() == 1 {
                            node_ids.first().cloned().unwrap_or_default()
                        } else {
                            format!("{} selected", node_ids.len())
                        },
                    ),
                ],
            },
        ])
    }
    //#endregion 🔖Panels

    //#region 🔖Render
    fn render_graph(fixture: &GraphFixture, runtime: &TrinityJackRuntime) -> UiNode {
        let (nodes_json, edges_json, viewport_json) = fixture_to_workflow(fixture);
        let selection_json = if runtime.selected_node_ids.is_empty() {
            None
        } else {
            serde_json::to_string(&runtime.selected_node_ids).ok()
        };
        build_node_graph_scene(
            TRINITY_JACK_PLAY_SURFACE_GRAPH,
            TRINITY_JACK_PLAY_CONTROLLER_ID,
            NodeGraphScene {
                selection_json,
                context_menu_json: Some(
                    r#"[{"id":"delete-selection","label":"Delete selection","icon":"trash","action":"nodeGraphEdit","args":{"operations":[{"operation":"deleteSelection"}]},"destructive":true}]"#.into(),
                ),
                lod_json: trinity_lod_json_for_window(runtime, TRINITY_JACK_PLAY_WINDOW_GRAPH),
                ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
            },
        )
    }

    fn render_editor(fixture: &GraphFixture, runtime: &TrinityJackRuntime) -> UiNode {
        let query = &runtime.jack_query;
        let graph = graph_from_fixture_or_default(fixture);
        let cursor = runtime.editor_selection.as_ref().map(|selection| selection.end).unwrap_or(0);
        let selection_json = runtime
            .editor_selection
            .as_ref()
            .map(|selection| json!({ "start": selection.start, "end": selection.end }).to_string());
        build_text_editor_scene(
            TRINITY_JACK_PLAY_SURFACE_EDITOR,
            TRINITY_JACK_PLAY_CONTROLLER_ID,
            TextEditorScene {
                selection_json,
                tokens_json: serde_json::to_string(&semantic_tokens(query)).ok(),
                diagnostics_json: serde_json::to_string(&lint(&graph, query)).ok(),
                completions_json: serde_json::to_string(&complete(&graph, query, cursor)).ok(),
                occurrences_json: text_identifier_occurrences_json(query, cursor),
                ..TextEditorScene::base(query.clone(), Some("jack".into()), None)
            },
        )
    }

    fn render_results(runtime: &TrinityJackRuntime) -> UiNode {
        let result: QueryResult = serde_json::from_str(&runtime.jack_result_json).unwrap_or(QueryResult::table(vec![], vec![]));
        if result.kind == QueryResultKind::Graph {
            if let Some(fixture) = &result.graph_fixture {
                let (nodes_json, edges_json, viewport_json) = fixture_to_workflow(fixture);
                return build_node_graph_scene(
                    TRINITY_JACK_PLAY_SURFACE_RESULTS,
                    TRINITY_JACK_PLAY_CONTROLLER_ID,
                    NodeGraphScene::base(nodes_json, edges_json, viewport_json),
                );
            }
        }
        let (columns_json, rows_json) = result_to_table(&runtime.jack_result_json);
        build_table_scene(
            TRINITY_JACK_PLAY_SURFACE_RESULTS,
            TRINITY_JACK_PLAY_CONTROLLER_ID,
            TableScene::base(columns_json, rows_json),
        )
    }
    //#endregion 🔖Render

    //#region 🔖TrinityJackPlayApp
    /// 🔱 Trinity Jack play app — a jack-query editor over a live {@link GraphFixture} projection; all
    /// document mutation flows through {@link TrinityGraphOperation}, all editor/selection/LOD state is runtime.
    pub struct TrinityJackPlayApp {
        runtime: TrinityJackRuntime,
    }

    impl Default for TrinityJackPlayApp {
        fn default() -> Self {
            Self { runtime: seeded_jack_runtime() }
        }
    }

    impl DocumentApp for TrinityJackPlayApp {
        type Projection = GraphFixture;
        type Operation = TrinityGraphOperation;

        fn app_id(&self) -> &str {
            TRINITY_JACK_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            TRINITY_GRAPH_SCHEMA
        }

        fn initial_projection(&self) -> GraphFixture {
            default_fixture()
        }

        fn handle_action(
            &mut self,
            action: &str,
            args: Option<&Value>,
            doc: &DocumentView<'_, GraphFixture>,
            _view_state: &ViewState,
        ) -> ActionEmit<TrinityGraphOperation> {
            let fixture = doc.projection;
            match action {
                "setSelection" | "selectNode" | "nodeGraphSelect" => {
                    self.runtime.selected_node_ids = selection_ids(args);
                    ActionEmit::default()
                }
                "nodeGraphHover" | "textHover" => ActionEmit::default(),
                "nodeGraphViewport" => {
                    if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                        if let Ok(camera) = serde_json::from_str::<Camera>(viewport_json) {
                            return ActionEmit::amend(vec![TrinityGraphOperation::SetCamera { camera }], "viewport");
                        }
                    }
                    ActionEmit::default()
                }
                "nodeGraphEdit" => {
                    let operations = args
                        .and_then(|value| value.get("operations"))
                        .and_then(|value| value.as_array())
                        .cloned()
                        .unwrap_or_default();
                    let mut emitted: Vec<TrinityGraphOperation> = Vec::new();
                    let mut has_set_fixture = false;
                    for operation in operations {
                        match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                            "setFixture" => {
                                if let Some(next) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| GraphFixture::from_json(json).ok()) {
                                    emitted.push(TrinityGraphOperation::SetFixture { fixture: next });
                                    has_set_fixture = true;
                                }
                            }
                            "deleteSelection" => {
                                let deletes: Vec<TrinityGraphOperation> = self
                                    .runtime
                                    .selected_node_ids
                                    .iter()
                                    .filter(|id| fixture.nodes.iter().any(|node| &node.id == *id))
                                    .map(|id| TrinityGraphOperation::DeleteNode { id: id.clone() })
                                    .collect();
                                if !deletes.is_empty() {
                                    self.runtime.selected_node_ids.clear();
                                    emitted.extend(deletes);
                                }
                            }
                            _ => {}
                        }
                    }
                    if emitted.is_empty() {
                        ActionEmit::default()
                    } else if has_set_fixture {
                        ActionEmit::amend(emitted, "node-graph-edit")
                    } else {
                        ActionEmit::operations(emitted)
                    }
                }
                "textEdit" => {
                    if let Some(text) = args.and_then(|v| v.get("text")).and_then(|v| v.as_str()) {
                        self.runtime.jack_query = text.into();
                    }
                    ActionEmit::default()
                }
                "textSelect" => {
                    let start = args.and_then(|v| v.get("start")).and_then(|v| v.as_u64()).unwrap_or(0);
                    let end = args.and_then(|v| v.get("end")).and_then(|v| v.as_u64()).unwrap_or(start);
                    self.runtime.editor_selection = Some(TrinityEditorSelection { start: start as usize, end: end as usize });
                    ActionEmit::default()
                }
                "requestCompletions" => {
                    self.runtime.revision += 1;
                    ActionEmit::default()
                }
                "formatDocument" => {
                    if let Ok(formatted) = jack_format(&self.runtime.jack_query) {
                        self.runtime.jack_query = formatted;
                    }
                    ActionEmit::default()
                }
                "setLodMode" => {
                    if let (Some(window_id), Some(value)) = (
                        args.and_then(|v| v.get("windowId")).and_then(|v| v.as_str()),
                        args.and_then(|v| v.get("value")).and_then(|v| v.as_str()),
                    ) {
                        self.runtime.lod_mode_by_window.insert(window_id.into(), value.into());
                    }
                    ActionEmit::default()
                }
                "loadExampleQuery" => {
                    if let Some(query) = args.and_then(|v| v.get("query")).and_then(|v| v.as_str()) {
                        self.runtime.jack_query = query.into();
                        let (result_json, operations) = run_jack_query(fixture, query);
                        self.runtime.jack_result_json = result_json;
                        return ActionEmit::operations(operations);
                    }
                    ActionEmit::default()
                }
                "runJackQuery" | "submit" => {
                    let query = args
                        .and_then(|v| v.get("query"))
                        .and_then(|v| v.as_str())
                        .filter(|value| !value.trim().is_empty())
                        .map(str::to_string)
                        .unwrap_or_else(|| self.runtime.jack_query.clone());
                    self.runtime.jack_query = query.clone();
                    let (result_json, operations) = run_jack_query(fixture, &query);
                    self.runtime.jack_result_json = result_json;
                    self.runtime.results_engagement_input.clear();
                    ActionEmit::operations(operations)
                }
                "setActiveExample" => {
                    let example_id = args.and_then(|v| v.get("exampleId")).and_then(|v| v.as_str()).unwrap_or("");
                    if let Some(next) = fixture_dsl_for_preset(example_id).and_then(|dsl| GraphFixture::parse_dsl(dsl).ok()) {
                        self.runtime.active_fixture_id = example_id.into();
                        self.runtime.jack_query = preset_query(example_id).into();
                        let (result_json, _) = run_jack_query(&next, &self.runtime.jack_query);
                        self.runtime.jack_result_json = result_json;
                        return ActionEmit::operations(vec![TrinityGraphOperation::SetFixture { fixture: next }]);
                    }
                    ActionEmit::default()
                }
                "patchTrinityNodes" => {
                    let node_ids: Vec<String> = args
                        .and_then(|v| v.get("nodeIds"))
                        .and_then(|v| serde_json::from_value(v.clone()).ok())
                        .unwrap_or_default();
                    let field = args.and_then(|v| v.get("field")).and_then(|v| v.as_str()).unwrap_or("");
                    let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()).map(str::trim).unwrap_or("");
                    if field == "name" && !node_ids.is_empty() && !value.is_empty() {
                        let operations: Vec<TrinityGraphOperation> = node_ids
                            .iter()
                            .filter(|id| fixture.nodes.iter().any(|node| &node.id == *id))
                            .map(|id| TrinityGraphOperation::Rename { id: id.clone(), name: value.into() })
                            .collect();
                        return ActionEmit::operations(operations);
                    }
                    ActionEmit::default()
                }
                "reorganize" => {
                    self.runtime.reorganize_epoch += 1;
                    match force_layout_fixture(fixture) {
                        Some(after) => ActionEmit::operations(reposition_operations(fixture, &after)),
                        None => ActionEmit::default(),
                    }
                }
                "editorEngagementInput" => {
                    if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                        self.runtime.editor_engagement_input = value.into();
                    }
                    ActionEmit::default()
                }
                "graphEngagementInput" => {
                    if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                        self.runtime.graph_engagement_input = value.into();
                    }
                    ActionEmit::default()
                }
                "resultsEngagementInput" => {
                    if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                        self.runtime.results_engagement_input = value.into();
                    }
                    ActionEmit::default()
                }
                "graphPointerDown" => {
                    if let Some(node_id) = args.and_then(|v| v.get("nodeId")).and_then(|v| v.as_str()) {
                        self.runtime.selected_node_ids = vec![node_id.into()];
                    }
                    ActionEmit::default()
                }
                _ => ActionEmit::default(),
            }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, GraphFixture>, view_state: &ViewState) -> UiNode {
            let fixture = doc.projection;
            let labels = resolve_labels::<TrinityJackLabels>(view_state);
            match body_key {
                TRINITY_JACK_PLAY_BODY_GRAPH => render_graph(fixture, &self.runtime),
                TRINITY_JACK_PLAY_BODY_EDITOR => render_editor(fixture, &self.runtime),
                TRINITY_JACK_PLAY_BODY_RESULTS => render_results(&self.runtime),
                TRINITY_JACK_PLAY_BODY_DOCUMENT => build_document_tree(fixture, &self.runtime, labels),
                TRINITY_JACK_PLAY_BODY_CATALOGUE => build_catalogue_tree(&self.runtime, labels),
                TRINITY_JACK_PLAY_BODY_INSPECTION => build_inspector_tree(fixture, &self.runtime, labels),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn window_measures(&self, _doc: &DocumentView<'_, GraphFixture>, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
            let mode = self
                .runtime
                .lod_mode_by_window
                .get(TRINITY_JACK_PLAY_WINDOW_GRAPH)
                .map(String::as_str)
                .unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
            HashMap::from([(
                TRINITY_JACK_PLAY_WINDOW_GRAPH.to_string(),
                vec![trinity_lod_measure(TRINITY_JACK_PLAY_WINDOW_GRAPH, mode)],
            )])
        }

        fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
            let labels = resolve_labels::<TrinityJackLabels>(view_state);
            AppLabelsOverlay::default()
                .window_kind_label(TRINITY_JACK_PLAY_WINDOW_GRAPH, labels.window_graph)
                .window_kind_label(TRINITY_JACK_PLAY_WINDOW_EDITOR, labels.window_editor)
                .window_kind_label(TRINITY_JACK_PLAY_WINDOW_RESULTS, labels.window_results)
                .action_labels(trinity_jack_action_labels(is_de_locale(view_state)))
        }
    }
    //#endregion 🔖TrinityJackPlayApp

    //#region 🔖Manifest
    fn jack_window_stack(id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
        WindowLayoutChild::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode {
                kind: "window".into(),
                window_kind_id: id.into(),
                title: Some(title.into()),
                instance_id: None,
                template_id: None,
            }],
        })
    }

    fn jack_layout() -> WindowLayout {
        WindowLayout {
            root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
                kind: "row".into(),
                size: None,
                children: vec![
                    WindowLayoutChild::Stack(WindowLayoutStackNode {
                        kind: "stack".into(),
                        size: Some(0.6),
                        active_window_kind_id: None,
                        children: vec![WindowLayoutWindowNode {
                            kind: "window".into(),
                            window_kind_id: TRINITY_JACK_PLAY_WINDOW_GRAPH.into(),
                            title: Some("Nakagin Graph".into()),
                            instance_id: None,
                            template_id: None,
                        }],
                    }),
                    WindowLayoutChild::Axis(WindowLayoutAxisNode {
                        kind: "column".into(),
                        size: Some(0.4),
                        children: vec![
                            jack_window_stack(TRINITY_JACK_PLAY_WINDOW_EDITOR, "Jack Query", Some(0.55)),
                            jack_window_stack(TRINITY_JACK_PLAY_WINDOW_RESULTS, "Results", Some(0.45)),
                        ],
                    }),
                ],
            }),
        }
    }

    pub fn create_trinity_jack_app() -> App {
        App::from_builder(
            App::builder(TRINITY_JACK_PLAY_APP_ID, "Trinity Jack").document(["semio", "trinity", "jack"])
                .resource_kind(ResourceKindSpec {
                    id: "graph.trinity".into(),
                    name: "Trinity Graph".into(),
                    source_format: "trinity.graph".into(),
                    component_kind: "trinity".into(),
                    dimension: "graph".into(),
                    media_capability: OsMediaCapability::MeshOnly,
                    media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
                    schema: "trinity.graph".into(),
                    export_formats: vec![],
                    import_formats: vec![],
                })
                .icon_id("trinity")
                .mode("explore", "Explore")
                .default_mode_id("explore")
                .window_kind(TRINITY_JACK_PLAY_WINDOW_GRAPH, "Nakagin Graph", TRINITY_JACK_PLAY_BODY_GRAPH, SurfaceKind::NodeGraph, "graph-dag")
                .window_kind(TRINITY_JACK_PLAY_WINDOW_EDITOR, "Jack Query", TRINITY_JACK_PLAY_BODY_EDITOR, SurfaceKind::TextEditor, "document-jack")
                .window_kind(TRINITY_JACK_PLAY_WINDOW_RESULTS, "Results", TRINITY_JACK_PLAY_BODY_RESULTS, SurfaceKind::Table, "table-2")
                .default_layout(jack_layout())
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                    PanelGroup::Workbench,
                    TRINITY_JACK_PLAY_BODY_DOCUMENT,
                )
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                    PanelGroup::Workbench,
                    TRINITY_JACK_PLAY_BODY_CATALOGUE,
                )
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                    PanelGroup::Details,
                    TRINITY_JACK_PLAY_BODY_INSPECTION,
                )
                .operation("nodeGraphEdit", "Edit Graph")
                .operation("nodeGraphViewport", "Set Graph Viewport")
                .operation("patchTrinityNodes", "Patch Nodes")
                .operation("reorganize", "Reorganize")
                .operation("runJackQuery", "Run Jack Query")
                .operation("submit", "Submit Jack Query")
                .operation("loadExampleQuery", "Load Example Query")
                .operation("setActiveExample", "Set Active Example")
                .view_action("setSelection", "Set Selection")
                .view_action("selectNode", "Select Node")
                .view_action("nodeGraphSelect", "Select Graph Node")
                .view_action("nodeGraphHover", "Hover Graph Node")
                .view_action("textEdit", "Edit Jack Query")
                .view_action("textSelect", "Select Jack Query Text")
                .view_action("textHover", "Hover Jack Query Text")
                .view_action("requestCompletions", "Request Completions")
                .view_action("formatDocument", "Format Jack Query")
                .view_action("setLodMode", "Set LOD Mode")
                .view_action("editorEngagementInput", "Editor Engagement Input")
                .view_action("graphEngagementInput", "Graph Engagement Input")
                .view_action("resultsEngagementInput", "Results Engagement Input")
                .view_action("graphPointerDown", "Graph Pointer Down")
                // 📝 Staged argument forms for the panel-visible preset loaders.
                .action_args("setActiveExample", vec![
                    ActionArgDef::select("exampleId", "Fixture", vec![
                        ActionArgOption::new("nakagin", "Nakagin — Table"),
                        ActionArgOption::new("branch-chain", "Branch — Graph"),
                    ]).required(),
                ])
                .action_args("loadExampleQuery", vec![
                    ActionArgDef::select("query", "Example", vec![
                        ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 't_f0_b_c0' OR a.name = 't_f0_b_c1' RETURN a.name", "Where Or"),
                        ActionArgOption::new("MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b", "Return Graph"),
                        ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'demo-label'", "Set Label"),
                        ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 'b' SET a.x = 300, a.y = 120", "Set Position"),
                        ActionArgOption::new("CREATE (n:Piece)", "Create Node"),
                        ActionArgOption::new("MATCH (a:Piece), (b:Piece) WHERE a.name = 'b' AND b.name != 'b' CREATE (a)-[:Connection]->(b)", "Create Edge"),
                        ActionArgOption::new("MATCH (n:Piece) WHERE n.name = 'b' DELETE n", "Delete Leaf"),
                        ActionArgOption::new("MERGE (x:Piece)-[:Connection]->(y:Piece)", "Merge Edge"),
                    ]).required(),
                ])
                .keybinding("mod+z", "undo")
                .keybinding("mod+shift+z", "redo")
                .keybinding("mod+alt+s", "commitCheckpoint"),
        )
        .example("nakagin", "Nakagin", default_fixture().print_dsl())
        .program("trinity", "Trinity", "graph")
    }
    //#endregion 🔖Manifest

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{testkit, ActionMeta, PluginApp, VcsDocumentApp};

        fn meta(actor: &str) -> ActionMeta {
            testkit::meta(actor)
        }

        fn new_app() -> VcsDocumentApp<TrinityJackPlayApp> {
            testkit::new_app()
        }

        fn node_id_at(app: &VcsDocumentApp<TrinityJackPlayApp>, index: usize) -> String {
            app.projection().expect("projection").nodes[index].id.clone()
        }

        #[test]
        fn renders_node_graph_scene() {
            let mut app = new_app();
            let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
        }

        #[test]
        fn renders_jack_editor() {
            let mut app = new_app();
            let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("text-editor"));
            assert!(json.contains(TRINITY_JACK_DEFAULT_QUERY));
        }

        #[test]
        fn run_query_populates_results_and_a_set_query_mutates_projection() {
            let mut app = new_app();
            // The seeded default query (read-only) already populated the results runtime on construction.
            app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, &ViewState::default()).expect("render");
            let result = app
                .handle_action(
                    "runJackQuery",
                    Some(&json!({ "query": "MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'ran-label'" })),
                    &ViewState::default(),
                    &meta("local"),
                )
                .expect("run");
            assert!(!result.operations.is_empty(), "a SET query emits operations");
            let projection = app.projection().expect("projection");
            assert!(serde_json::to_string(&projection).unwrap().contains("ran-label"));
        }

        #[test]
        fn node_graph_select_updates_selection_and_document_tree() {
            let mut app = new_app();
            let node_id = node_id_at(&app, 0);
            let result = app
                .handle_action("nodeGraphSelect", Some(&json!({ "nodeIds": [node_id.clone()] })), &ViewState::default(), &meta("local"))
                .expect("select");
            assert!(result.operations.is_empty(), "selection is a view action, no operations");
            let tree = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&tree).unwrap().contains(&format!("trinity-document.node.{node_id}")));
        }

        #[test]
        fn nakagin_fixture_has_nodes() {
            assert!(!default_fixture().nodes.is_empty());
        }

        #[test]
        fn editor_scene_has_tokens_and_diagnostics() {
            let mut app = new_app();
            let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("tokensJson"));
            assert!(json.contains("diagnosticsJson"));
            assert!(json.contains("completionsJson"));
        }

        #[test]
        fn text_edit_updates_query_without_operations() {
            let mut app = new_app();
            let result = app
                .handle_action("textEdit", Some(&json!({ "text": "MATCH (a:Piece) RETURN a.name" })), &ViewState::default(), &meta("local"))
                .expect("edit");
            assert!(result.operations.is_empty());
            let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("MATCH (a:Piece) RETURN a.name"));
        }

        #[test]
        fn graph_scene_has_lod_json() {
            let mut app = new_app();
            let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewState::default()).expect("render");
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("lodJson"));
            assert!(json.contains("automatic"));
        }

        #[test]
        fn set_lod_mode_persists_per_window() {
            let mut app = new_app();
            app.handle_action(
                "setLodMode",
                Some(&json!({ "windowId": TRINITY_JACK_PLAY_WINDOW_GRAPH, "value": "minimap" })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("lod");
            let measures = app.window_measures(&ViewState::default());
            let graph_measures = serde_json::to_string(&measures[TRINITY_JACK_PLAY_WINDOW_GRAPH]).unwrap();
            assert!(graph_measures.contains("minimap"));
        }

        #[test]
        fn return_graph_example_renders_node_graph_in_results() {
            let mut app = new_app();
            app.handle_action(
                "loadExampleQuery",
                Some(&json!({ "query": "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b" })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("load example");
            let node = app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
        }

        #[test]
        fn catalogue_has_eight_example_queries() {
            let mut app = new_app();
            let node = app.render(TRINITY_JACK_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
            let json = serde_json::to_string(&node).unwrap();
            for id in ["where-or", "return-graph", "set-label", "set-position", "create-node", "create-edge", "delete-leaf", "merge-edge"] {
                assert!(json.contains(id), "missing example query {id}");
            }
        }

        #[test]
        fn inspector_has_flat_position_fields() {
            let mut app = new_app();
            let node_id = node_id_at(&app, 0);
            app.handle_action("nodeGraphSelect", Some(&json!({ "nodeIds": [node_id] })), &ViewState::default(), &meta("local")).expect("select");
            let node = app.render(TRINITY_JACK_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("Flat U"));
            assert!(json.contains("Flat V"));
        }

        // 🧰 `VcsDocumentApp::tools()` no longer exists — utility bars are now derived by the renderer
        // from the utility registry, which this app declares none of. `runJackQuery` is a plain
        // operation and `undo` is a framework-injected History action; both still live in the
        // static `AppDefinition.actions` list (undo/redo render via the History rail, not a
        // per-app utility bar) — assert on that surface instead.
        #[test]
        fn app_definition_declares_run_jack_query_and_history_actions() {
            let definition = create_trinity_jack_app().definition;
            let action_ids: Vec<&str> = definition.actions.iter().map(|action| action.id.as_str()).collect();
            assert!(action_ids.contains(&"runJackQuery"));
            assert!(action_ids.contains(&"undo"));
        }

        #[test]
        fn trinity_jack_labels_resolve_native_by_default() {
            let mut app = new_app();
            let node = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("\"Pieces\""));
            assert!(json.contains("\"Connections\""));
            assert!(!json.contains("Stücke"));
        }

        #[test]
        fn trinity_jack_labels_translate_panels_in_german() {
            let mut app = new_app();
            let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
            let document_json = serde_json::to_string(&app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &view_state).expect("render")).unwrap();
            assert!(document_json.contains("Stücke"));
            assert!(document_json.contains("Verbindungen"));
            assert!(!document_json.contains("\"Pieces\""));
            let catalogue_json = serde_json::to_string(&app.render(TRINITY_JACK_PLAY_BODY_CATALOGUE, None, &view_state).expect("render")).unwrap();
            assert!(catalogue_json.contains("Fixturen"));
            assert!(catalogue_json.contains("Beispielabfragen"));
            assert!(catalogue_json.contains("Manifestarten"));
            // 🧰 `VcsDocumentApp::tools()` no longer exists (see the removed utility-bar test above); the
            // "Verlauf" (History rail group) label had no per-app surface even before removal — only
            // the `runJackQuery` action label is this app's own to assert on.
            let action_labels = app.app_labels(&view_state).action_labels;
            assert_eq!(action_labels.get("runJackQuery").map(String::as_str), Some("Jack-Abfrage ausführen"));
        }

        #[test]
        fn patch_trinity_nodes_emits_rename_operation() {
            let mut app = new_app();
            let node_id = node_id_at(&app, 0);
            let result = app
                .handle_action(
                    "patchTrinityNodes",
                    Some(&json!({ "nodeIds": [node_id.clone()], "field": "name", "value": "renamed-node" })),
                    &ViewState::default(),
                    &meta("local"),
                )
                .expect("patch");
            assert_eq!(result.operations.len(), 1);
            let renamed = app.projection().expect("projection").nodes.iter().find(|node| node.id == node_id).unwrap().name.clone();
            assert_eq!(renamed, "renamed-node");
        }

        #[test]
        fn node_graph_viewport_emits_coalesced_set_camera() {
            let mut app = new_app();
            for zoom in [1.5, 2.0, 2.5] {
                app.handle_action(
                    "nodeGraphViewport",
                    Some(&json!({ "viewportJson": json!({ "x": 10.0, "y": 20.0, "zoom": zoom }).to_string() })),
                    &ViewState::default(),
                    &meta("local"),
                )
                .expect("viewport");
            }
            assert_eq!(app.projection().expect("projection").camera.zoom, 2.5);
            // Coalesced into a single undo step: undo restores the original camera, not an intermediate zoom.
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            assert_eq!(app.projection().expect("projection").camera.zoom, default_fixture().camera.zoom);
        }

        #[test]
        fn set_active_example_replaces_fixture_via_set_fixture_operation() {
            let mut app = new_app();
            let before = app.projection().expect("projection").name.clone();
            let result = app
                .handle_action("setActiveExample", Some(&json!({ "exampleId": "branch-chain" })), &ViewState::default(), &meta("local"))
                .expect("example");
            assert_eq!(result.operations.len(), 1);
            let after = app.projection().expect("projection").name.clone();
            assert_ne!(before, after, "loading a different preset replaces the fixture");
        }

        #[test]
        fn node_graph_edit_delete_selection_emits_delete_node_operations() {
            let mut app = new_app();
            let node_id = node_id_at(&app, 0);
            app.handle_action("nodeGraphSelect", Some(&json!({ "nodeIds": [node_id.clone()] })), &ViewState::default(), &meta("local")).expect("select");
            let before = app.projection().expect("projection").nodes.len();
            let result = app
                .handle_action("nodeGraphEdit", Some(&json!({ "operations": [{ "operation": "deleteSelection" }] })), &ViewState::default(), &meta("local"))
                .expect("delete");
            assert!(!result.operations.is_empty());
            assert_eq!(app.projection().expect("projection").nodes.len(), before - 1);
        }

        #[test]
        fn undo_redo_round_trip_through_the_wrapper() {
            let mut app = new_app();
            let node_id = node_id_at(&app, 0);
            let before_name = app.projection().unwrap().nodes.iter().find(|n| n.id == node_id).unwrap().name.clone();
            testkit::assert_undo_redo_round_trip(
                &mut app,
                "patchTrinityNodes",
                Some(&json!({ "nodeIds": [node_id.clone()], "field": "name", "value": "undo-me" })),
                |app| app.projection().unwrap().nodes.iter().find(|n| n.id == node_id).unwrap().name.clone(),
                before_name,
                "undo-me".to_string(),
            );
        }

        /// 🧪 The definitional merge proof: two instances start from the same fixture, rename DISJOINT
        /// nodes (A renames node 0, B renames node 1), and exchanging operations over a `MemoryBackbone`
        /// converges both to contain BOTH renames — impossible under whole-document `setDocument` LWW.
        #[test]
        fn two_instances_converge_disjoint_edits_via_backbone() {
            let seed = new_app();
            let node0 = node_id_at(&seed, 0);
            let node1 = node_id_at(&seed, 1);
            drop(seed);
            testkit::assert_two_instances_converge::<TrinityJackPlayApp, _>(
                "mem://trinity-jack-convergence",
                ("patchTrinityNodes", Some(&json!({ "nodeIds": [node0.clone()], "field": "name", "value": "Renamed By A" }))),
                ("patchTrinityNodes", Some(&json!({ "nodeIds": [node1.clone()], "field": "name", "value": "Renamed By B" }))),
                |app| {
                    let projection = app.projection().unwrap();
                    (
                        projection.nodes.iter().find(|n| n.id == node0).unwrap().name.clone(),
                        projection.nodes.iter().find(|n| n.id == node1).unwrap().name.clone(),
                    )
                },
            );
        }

        #[test]
        fn ingest_operations_is_idempotent() {
            let seed = new_app();
            let node_id = node_id_at(&seed, 0);
            drop(seed);
            testkit::assert_ingest_idempotent::<TrinityJackPlayApp, _>(
                "patchTrinityNodes",
                Some(&json!({ "nodeIds": [node_id.clone()], "field": "name", "value": "Hero" })),
                |app| app.projection().unwrap().nodes.iter().find(|n| n.id == node_id).unwrap().name.clone(),
            );
        }
    }
    //#endregion 🧪Tests
}
pub mod app_rewrite {
    //! ♻️ Trinity Rewrite plugin — parametric rewrite play app bundled as a hot-swappable WASM component.

    use semio_framework_plugin::{SurfaceKind, PanelGroup,
        app_labels, build_node_graph_scene, build_text_editor_scene, text_identifier_bounds_at,
        is_de_locale, localized_label_map, resolve_labels, tree_item, tree_item_with_action,
        ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
        ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, ActionDescriptor, AppLabelsOverlay, AppLabelsOverlayExt,
        DocumentApp, DocumentView, MeasureSelectItem, NodeGraphScene, PanelTreeBuilder,
        TextEditorScene, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiTreeItemNode,
        ViewState, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot,
        WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
        FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
        FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
    };
    use serde::Serialize;
    use serde_json::{json, Value};
    use std::collections::{BTreeMap, HashMap};
    use trinity_jack::semantic_tokens;
    use trinity_ram::{Camera, Graph, GraphFixture, Node, PortDirection, PropertyValue};
    use trinity_rewrite::{
        apply_rule, build_rule_query, rule_query_json, trinity_lod_scale_json,
        AssignmentJson, Lhs, LayoutPoint, ParameterKind, ParameterSpec, Rhs, Rule,
        PatternJson, RewriteRuleOperation, RewriteRuleState, REWRITE_RULE_SCHEMA,
    };
    use store::DocumentDsl;

    //#region 🔖Constants
    const TRINITY_REWRITE_PLAY_APP_ID: &str = "trinity-rewrite-play";
    const TRINITY_REWRITE_PLAY_CONTROLLER_ID: &str = "trinity-rewrite-play";
    const TRINITY_REWRITE_PLAY_SURFACE_BEFORE: &str = "trinity.rewrite.before";
    const TRINITY_REWRITE_PLAY_SURFACE_AFTER: &str = "trinity.rewrite.after";
    const TRINITY_REWRITE_PLAY_SURFACE_LHS: &str = "trinity.rewrite.lhs";
    const TRINITY_REWRITE_PLAY_SURFACE_RHS: &str = "trinity.rewrite.rhs";
    const TRINITY_REWRITE_PLAY_SURFACE_JACK: &str = "trinity.rewrite.jack";
    const TRINITY_REWRITE_PLAY_BODY_BEFORE: &str = "trinity.rewrite.play.before";
    const TRINITY_REWRITE_PLAY_BODY_AFTER: &str = "trinity.rewrite.play.after";
    const TRINITY_REWRITE_PLAY_BODY_LHS: &str = "trinity.rewrite.play.lhs";
    const TRINITY_REWRITE_PLAY_BODY_RHS: &str = "trinity.rewrite.play.rhs";
    const TRINITY_REWRITE_PLAY_BODY_JACK: &str = "trinity.rewrite.play.jack";
    const TRINITY_REWRITE_PLAY_BODY_PARAMETERS: &str = "trinity.rewrite.play.parameters";
    const TRINITY_REWRITE_PLAY_BODY_DOCUMENT: &str = "trinity.rewrite.play.document";
    const TRINITY_REWRITE_PLAY_BODY_CATALOGUE: &str = "trinity.rewrite.play.catalogue";
    const TRINITY_REWRITE_PLAY_BODY_INSPECTION: &str = "trinity.rewrite.play.inspection";
    const TRINITY_REWRITE_PLAY_WINDOW_BEFORE: &str = "trinity-rewrite-before";
    const TRINITY_REWRITE_PLAY_WINDOW_AFTER: &str = "trinity-rewrite-after";
    const TRINITY_REWRITE_PLAY_WINDOW_LHS: &str = "trinity-rewrite-lhs";
    const TRINITY_REWRITE_PLAY_WINDOW_RHS: &str = "trinity-rewrite-rhs";
    const TRINITY_REWRITE_PLAY_WINDOW_JACK: &str = "trinity-rewrite-jack";
    const TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS: &str = "trinity-rewrite-parameters";
    const TRINITY_REWRITE_PLAY_RULE_NAME: &str = "label-core";

    const NAKAGIN_FIXTURE_DSL: &str = include_str!("../../example/nakagin-capsule-tower.trinity");

    const DEFAULT_LHS_JSON: &str = r#"{
      "pattern": {
        "leftVar": "a",
        "leftKind": "Piece",
        "edgeVar": "r",
        "edgeKind": "Connection",
        "rightVar": "b",
        "rightKind": "Piece"
      },
      "whereClause": "a.name = 'b'"
    }"#;

    const DEFAULT_RHS_JSON: &str = r#"{
      "create": [],
      "delete": [],
      "set": [{ "var": "a", "prop": "label", "value": "$label" }],
      "merge": [],
      "parameters": [{ "name": "label", "kind": "string", "default": "nakagin-core" }]
    }"#;

    const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";
    //#endregion 🔖Constants

    //#region 🔖Types
    /// 🎛️ Ephemeral view state (selection, hover/select var focus, epochs, LOD) — lives on the app
    /// struct, never in the document. The document projection is the {@link RewriteRuleState}.
    #[derive(Clone, Debug, Default, PartialEq)]
    struct RewritePlayRuntime {
        selected_node_ids: Vec<String>,
        reorganize_epoch: u64,
        active_hover_var: String,
        hover_epoch: u64,
        active_select_var: String,
        select_epoch: u64,
        lod_mode_by_window: BTreeMap<String, String>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct WorkflowDiagramPortRecord {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct WorkflowNodeRecord {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        inputs: Vec<WorkflowDiagramPortRecord>,
        outputs: Vec<WorkflowDiagramPortRecord>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct WorkflowEdgeRecord {
        id: String,
        source_node_id: String,
        source_port_id: String,
        target_node_id: String,
        target_port_id: String,
    }
    //#endregion 🔖Types

    //#region 🔖DocumentHelpers
    /// 📦 JSON text of the bundled Nakagin fixture — `RewriteRuleState`'s own `_json` fields keep their
    /// JSON contract (see `patch_fixture_nodes`/`parse_fixture_json`), so the `.trinity` DSL source is
    /// parsed once and re-serialized here rather than propagating DSL text into those fields.
    fn nakagin_fixture_json() -> String {
        GraphFixture::parse_dsl(NAKAGIN_FIXTURE_DSL).expect("bundled nakagin fixture parses").to_json().expect("fixture serializes")
    }

    fn default_rule_state() -> RewriteRuleState {
        let mut state = RewriteRuleState {
            before_fixture_json: nakagin_fixture_json(),
            lhs_json: DEFAULT_LHS_JSON.into(),
            rhs_json: DEFAULT_RHS_JSON.into(),
            parameter_bindings: BTreeMap::new(),
            rule_layout: BTreeMap::new(),
        };
        state.parameter_bindings = default_parameter_bindings(&state.rhs_json);
        state
    }

    fn rewrite_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor {
            controller_id: TRINITY_REWRITE_PLAY_CONTROLLER_ID.into(),
            action: action.into(),
            args,
        }
    }

    fn parse_fixture_json(json: &str) -> Option<GraphFixture> {
        GraphFixture::from_json(json).ok()
    }

    fn default_parameter_bindings(rhs_json: &str) -> BTreeMap<String, PropertyValue> {
        let Ok(rhs) = serde_json::from_str::<Rhs>(rhs_json) else {
            return BTreeMap::new();
        };
        rhs.parameters
            .iter()
            .map(|param| (param.name.clone(), param.default.clone()))
            .collect()
    }

    /// 📤 Emits a `SetState` operation iff `next` differs from `current` (mirrors the store's LWW no-operation guard),
    /// so view-neutral re-computations don't record empty history entries.
    fn set_state_emit(current: &RewriteRuleState, next: RewriteRuleState) -> ActionEmit<RewriteRuleOperation> {
        if &next == current {
            ActionEmit::default()
        } else {
            ActionEmit::operations(vec![RewriteRuleOperation::SetState { state: next }])
        }
    }

    fn build_rule_from_state(state: &RewriteRuleState) -> Result<Rule, String> {
        let lhs: Lhs = serde_json::from_str(&state.lhs_json).map_err(|e| e.to_string())?;
        let rhs: Rhs = serde_json::from_str(&state.rhs_json).map_err(|e| e.to_string())?;
        Ok(Rule {
            name: TRINITY_REWRITE_PLAY_RULE_NAME.into(),
            lhs,
            rhs,
        })
    }

    fn compiled_jack_query(state: &RewriteRuleState) -> String {
        let rule_json = match build_rule_from_state(state) {
            Ok(rule) => serde_json::to_string(&rule).unwrap_or_default(),
            Err(_) => return String::new(),
        };
        let bindings_json = serde_json::to_string(&state.parameter_bindings).unwrap_or_else(|_| "{}".into());
        rule_query_json(&rule_json, &bindings_json)
            .ok()
            .and_then(|json| serde_json::from_str::<Value>(&json).ok())
            .and_then(|value| value.get("query").and_then(|query| query.as_str()).map(str::to_string))
            .unwrap_or_else(|| {
                build_rule_from_state(state)
                    .map(|rule| build_rule_query(&rule, &state.parameter_bindings))
                    .unwrap_or_default()
            })
    }

    fn apply_rewrite_to_fixture(before_json: &str, state: &RewriteRuleState) -> String {
        let Ok(mut graph) = Graph::load_json(before_json) else {
            return before_json.into();
        };
        let Ok(rule) = build_rule_from_state(state) else {
            return before_json.into();
        };
        if apply_rule(&mut graph, &rule, &state.parameter_bindings).is_ok() {
            graph.fixture_json().unwrap_or_else(|_| before_json.into())
        } else {
            before_json.into()
        }
    }

    fn after_fixture_json(state: &RewriteRuleState) -> String {
        apply_rewrite_to_fixture(&state.before_fixture_json, state)
    }

    /// 🎯 Selection ids from an action's args — delegates to the SDK's shared `ids`-array reader, falling
    /// back to a singular `nodeIds`/`nodeId` key for actions dispatched from the node-graph scene surface.
    fn selection_ids(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("nodeIds"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .or_else(|| Some(semio_framework_plugin::selection_ids(args)).filter(|ids: &Vec<String>| !ids.is_empty()))
            .or_else(|| {
                args.and_then(|value| value.get("nodeId"))
                    .and_then(|value| value.as_str())
                    .map(|id| vec![id.to_string()])
            })
            .unwrap_or_default()
    }

    fn sync_select_var_from_node(runtime: &mut RewritePlayRuntime, fixture_json: &str, node_id: &str) {
        if let Some(fixture) = parse_fixture_json(fixture_json) {
            if let Some(node) = fixture.nodes.iter().find(|node| node.id == node_id) {
                if let Some(var) = var_from_node_name(&node.name) {
                    runtime.active_select_var = var;
                }
            }
        }
    }

    fn sync_hover_var_from_node(runtime: &mut RewritePlayRuntime, fixture_json: &str, node_id: &str) {
        if let Some(fixture) = parse_fixture_json(fixture_json) {
            if let Some(node) = fixture.nodes.iter().find(|node| node.id == node_id) {
                if let Some(var) = var_from_node_name(&node.name) {
                    runtime.active_hover_var = var;
                }
            }
        }
        runtime.hover_epoch += 1;
    }

    /// 🧭 Resolves which fixture backs a given rewrite graph surface (Before/After/LHS/RHS), for hover/select var lookup.
    fn fixture_json_for_surface(surface_id: &str, state: &RewriteRuleState) -> String {
        if surface_id == TRINITY_REWRITE_PLAY_SURFACE_AFTER {
            after_fixture_json(state)
        } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_LHS {
            lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout)
        } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_RHS {
            rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout)
        } else {
            state.before_fixture_json.clone()
        }
    }

    fn apply_semantic_layout_edit(rule_layout: &mut BTreeMap<String, LayoutPoint>, current_fixture_json: &str, edited_fixture_json: &str) -> bool {
        let (Some(current), Some(edited)) = (parse_fixture_json(current_fixture_json), parse_fixture_json(edited_fixture_json)) else {
            return false;
        };
        let mut changed = false;
        for node in &edited.nodes {
            let Some(prev) = current.nodes.iter().find(|entry| entry.id == node.id) else {
                continue;
            };
            if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
                rule_layout.insert(node.id.clone(), LayoutPoint { x: node.x, y: node.y });
                changed = true;
            }
        }
        changed
    }

    enum RuleClauseRef {
        LhsWhere,
        RhsCreate(usize),
        RhsMerge(usize),
        RhsSet(usize),
        RhsDelete(usize),
        RhsParameter(usize),
    }

    fn parse_clause_ref(node_id: &str) -> Option<RuleClauseRef> {
        if node_id == "lhs-where" {
            return Some(RuleClauseRef::LhsWhere);
        }
        let (prefix, index) = node_id.rsplit_once('-')?;
        let index: usize = index.parse().ok()?;
        match prefix {
            "rhs-create" => Some(RuleClauseRef::RhsCreate(index)),
            "rhs-merge" => Some(RuleClauseRef::RhsMerge(index)),
            "rhs-set" => Some(RuleClauseRef::RhsSet(index)),
            "rhs-delete" => Some(RuleClauseRef::RhsDelete(index)),
            "rhs-parameter" => Some(RuleClauseRef::RhsParameter(index)),
            _ => None,
        }
    }

    fn remove_at<T>(items: &mut Vec<T>, index: usize) -> bool {
        if index < items.len() {
            items.remove(index);
            true
        } else {
            false
        }
    }

    fn delete_rule_clause(state: &mut RewriteRuleState, node_id: &str) -> bool {
        let Some(clause_ref) = parse_clause_ref(node_id) else {
            return false;
        };
        let Ok(mut lhs) = serde_json::from_str::<Lhs>(&state.lhs_json) else {
            return false;
        };
        let Ok(mut rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
            return false;
        };
        let changed = match clause_ref {
            RuleClauseRef::LhsWhere => {
                let had = lhs.where_clause.is_some();
                lhs.where_clause = None;
                had
            }
            RuleClauseRef::RhsCreate(index) => remove_at(&mut rhs.create, index),
            RuleClauseRef::RhsMerge(index) => remove_at(&mut rhs.merge, index),
            RuleClauseRef::RhsSet(index) => remove_at(&mut rhs.set, index),
            RuleClauseRef::RhsDelete(index) => remove_at(&mut rhs.delete, index),
            RuleClauseRef::RhsParameter(index) => {
                if index < rhs.parameters.len() {
                    let removed = rhs.parameters.remove(index);
                    state.parameter_bindings.remove(&removed.name);
                    true
                } else {
                    false
                }
            }
        };
        if changed {
            state.lhs_json = serde_json::to_string(&lhs).unwrap_or_default();
            state.rhs_json = serde_json::to_string(&rhs).unwrap_or_default();
            state.rule_layout.remove(node_id);
        }
        changed
    }

    /// ➕ Appends a default instance of `clause_kind` to the rule (rewrite.where/create/merge/set/delete/parameter).
    fn add_rule_clause(state: &mut RewriteRuleState, clause_kind: &str) -> bool {
        let Ok(mut lhs) = serde_json::from_str::<Lhs>(&state.lhs_json) else {
            return false;
        };
        let Ok(mut rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
            return false;
        };
        let left_var = lhs.pattern.left_var.clone();
        let changed = match clause_kind {
            "where" => {
                if lhs.where_clause.is_some() {
                    false
                } else {
                    lhs.where_clause = Some(format!("{left_var}.name = 'value'"));
                    true
                }
            }
            "create" => {
                rhs.create.push(PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
                true
            }
            "merge" => {
                rhs.merge.push(PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
                true
            }
            "set" => {
                rhs.set.push(AssignmentJson { var: left_var, prop: "label".into(), value: PropertyValue::String(String::new()) });
                true
            }
            "delete" => {
                rhs.delete.push(left_var);
                true
            }
            "parameter" => {
                let name = format!("param{}", rhs.parameters.len());
                state.parameter_bindings.insert(name.clone(), PropertyValue::String(String::new()));
                rhs.parameters.push(ParameterSpec { name, kind: ParameterKind::String, default: PropertyValue::String(String::new()) });
                true
            }
            _ => false,
        };
        if changed {
            state.lhs_json = serde_json::to_string(&lhs).unwrap_or_default();
            state.rhs_json = serde_json::to_string(&rhs).unwrap_or_default();
        }
        changed
    }

    /// 🖊️ Applies node-graph editor operations (drag layout / delete-selection) in place to `state`, returning
    /// whether anything changed; the caller wraps the result in a `SetState` operation.
    fn apply_rewrite_node_graph_edit_operations(state: &mut RewriteRuleState, runtime: &mut RewritePlayRuntime, surface_id: &str, operations: &[Value]) -> bool {
        let mut changed = false;
        for operation in operations {
            match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                "setFixture" => {
                    let Some(fixture_json) = operation.get("fixtureJson").and_then(|value| value.as_str()) else {
                        continue;
                    };
                    if parse_fixture_json(fixture_json).is_none() {
                        continue;
                    }
                    if surface_id == TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                        state.before_fixture_json = fixture_json.into();
                        changed = true;
                    } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_LHS {
                        let current = lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout);
                        changed |= apply_semantic_layout_edit(&mut state.rule_layout, &current, fixture_json);
                    } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_RHS {
                        let current = rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout);
                        changed |= apply_semantic_layout_edit(&mut state.rule_layout, &current, fixture_json);
                    }
                }
                "deleteSelection" => {
                    if runtime.selected_node_ids.is_empty() {
                        continue;
                    }
                    if surface_id == TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                        let ids = runtime.selected_node_ids.clone();
                        if let Some(mut fixture) = parse_fixture_json(&state.before_fixture_json) {
                            fixture.nodes.retain(|node| !ids.contains(&node.id));
                            fixture.edges.retain(|edge| {
                                let from = trinity_ram::port_node_id(&edge.source).unwrap_or(&edge.source);
                                let to = trinity_ram::port_node_id(&edge.target).unwrap_or(&edge.target);
                                !ids.iter().any(|id| id == from || id == to)
                            });
                            if let Ok(json) = Graph::from_fixture(fixture).and_then(|graph| graph.fixture_json()) {
                                state.before_fixture_json = json;
                                runtime.selected_node_ids.clear();
                                changed = true;
                            }
                        }
                    } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_LHS || surface_id == TRINITY_REWRITE_PLAY_SURFACE_RHS {
                        let ids = runtime.selected_node_ids.clone();
                        let mut deleted = false;
                        for id in &ids {
                            deleted |= delete_rule_clause(state, id);
                        }
                        if deleted {
                            runtime.selected_node_ids.clear();
                            changed = true;
                        }
                    }
                }
                _ => {}
            }
        }
        changed
    }

    fn patch_fixture_nodes(fixture_json: &str, node_ids: &[String], field: &str, value: &str) -> Option<String> {
        let mut fixture = GraphFixture::from_json(fixture_json).ok()?;
        for node in fixture.nodes.iter_mut() {
            if !node_ids.iter().any(|id| id == &node.id) {
                continue;
            }
            match field {
                "name" => node.name = value.into(),
                "kind" => node.kind = value.into(),
                _ => {}
            }
        }
        Graph::from_fixture(fixture).ok()?.fixture_json().ok()
    }

    fn semantic_rule_node(id: &str, kind: &str, name: &str, x: f64, y: f64, rule_layout: &BTreeMap<String, LayoutPoint>) -> Node {
        let (x, y) = rule_layout.get(id).map(|point| (point.x, point.y)).unwrap_or((x, y));
        Node {
            id: id.into(),
            name: name.into(),
            kind: kind.into(),
            x,
            y,
            width: 160.0,
            height: 56.0,
            ports: vec![],
            properties: Default::default(),
        }
    }

    fn lhs_semantic_graph_fixture(lhs: &Lhs, rule_layout: &BTreeMap<String, LayoutPoint>) -> GraphFixture {
        let mut nodes = vec![semantic_rule_node(
            "lhs-match",
            "rewrite.match",
            &format!("{}:{}", lhs.pattern.left_var, lhs.pattern.left_kind),
            0.0,
            0.0,
            rule_layout,
        )];
        let mut edges = Vec::new();
        if let Some(where_clause) = lhs.where_clause.as_deref().filter(|value| !value.trim().is_empty()) {
            nodes.push(semantic_rule_node("lhs-where", "rewrite.where", where_clause, 220.0, 80.0, rule_layout));
            edges.push(trinity_ram::Edge {
                id: "lhs-match-where".into(),
                kind: "rewrite.flow".into(),
                source: "lhs-match@out".into(),
                target: "lhs-where@in".into(),
                properties: Default::default(),
            });
        }
        GraphFixture {
            schema: GraphFixture::SCHEMA.into(),
            name: "lhs".into(),
            manifest_id: Some("nakagin".into()),
            manifest: trinity_ram::Manifest::nakagin_default(),
            camera: trinity_ram::Camera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes,
            edges,
            root_node_id: None,
        }
    }

    fn rhs_semantic_graph_fixture(rhs: &Rhs, rule_layout: &BTreeMap<String, LayoutPoint>) -> GraphFixture {
        let mut nodes = Vec::new();
        let edges = Vec::new();
        let mut y = 0.0;
        for (index, pattern) in rhs.create.iter().enumerate() {
            let id = format!("rhs-create-{index}");
            nodes.push(semantic_rule_node(
                &id,
                "rewrite.create",
                &format!("{}:{}", pattern.left_var, pattern.left_kind),
                (index as f64) * 220.0,
                y,
                rule_layout,
            ));
        }
        y += 80.0;
        for (index, pattern) in rhs.merge.iter().enumerate() {
            let id = format!("rhs-merge-{index}");
            nodes.push(semantic_rule_node(
                &id,
                "rewrite.merge",
                &format!("{}:{}", pattern.left_var, pattern.left_kind),
                (index as f64) * 220.0,
                y,
                rule_layout,
            ));
        }
        y += 80.0;
        for (index, assignment) in rhs.set.iter().enumerate() {
            let id = format!("rhs-set-{index}");
            nodes.push(semantic_rule_node(
                &id,
                "rewrite.set",
                &format!("{}.{} = {:?}", assignment.var, assignment.prop, assignment.value),
                (index as f64) * 220.0,
                y,
                rule_layout,
            ));
        }
        y += 80.0;
        for (index, name) in rhs.delete.iter().enumerate() {
            let id = format!("rhs-delete-{index}");
            nodes.push(semantic_rule_node(&id, "rewrite.delete", name, (index as f64) * 220.0, y, rule_layout));
        }
        y += 80.0;
        for (index, parameter) in rhs.parameters.iter().enumerate() {
            let id = format!("rhs-parameter-{index}");
            let kind = match parameter.kind {
                ParameterKind::String => "string",
                ParameterKind::Number => "number",
                ParameterKind::Boolean => "boolean",
            };
            nodes.push(semantic_rule_node(
                &id,
                "rewrite.parameter",
                &format!("{}:{kind}", parameter.name),
                (index as f64) * 220.0,
                y,
                rule_layout,
            ));
        }
        if nodes.is_empty() {
            nodes.push(semantic_rule_node("rhs-empty", "rewrite.create", "result:Piece", 0.0, 0.0, rule_layout));
        }
        GraphFixture {
            schema: GraphFixture::SCHEMA.into(),
            name: "rhs".into(),
            manifest_id: Some("nakagin".into()),
            manifest: trinity_ram::Manifest::nakagin_default(),
            camera: trinity_ram::Camera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes,
            edges,
            root_node_id: None,
        }
    }

    fn lhs_graph_fixture_json(lhs_json: &str, rule_layout: &BTreeMap<String, LayoutPoint>) -> String {
        let Ok(lhs) = serde_json::from_str::<Lhs>(lhs_json) else {
            return nakagin_fixture_json();
        };
        Graph::from_fixture(lhs_semantic_graph_fixture(&lhs, rule_layout))
            .ok()
            .and_then(|graph| graph.fixture_json().ok())
            .unwrap_or_else(nakagin_fixture_json)
    }

    fn rhs_graph_fixture_json(rhs_json: &str, rule_layout: &BTreeMap<String, LayoutPoint>) -> String {
        let Ok(rhs) = serde_json::from_str::<Rhs>(rhs_json) else {
            return nakagin_fixture_json();
        };
        Graph::from_fixture(rhs_semantic_graph_fixture(&rhs, rule_layout))
            .ok()
            .and_then(|graph| graph.fixture_json().ok())
            .unwrap_or_else(nakagin_fixture_json)
    }

    fn node_id_for_var(fixture_json: &str, var: &str) -> Option<String> {
        if var.is_empty() {
            return None;
        }
        let fixture = GraphFixture::from_json(fixture_json).ok()?;
        fixture
            .nodes
            .iter()
            .find(|node| {
                node.name.starts_with(&format!("{var}:"))
                    || node.name == var
                    || var_from_node_name(&node.name).as_deref() == Some(var)
            })
            .map(|node| node.id.clone())
    }

    fn graph_hover_json(fixture_json: &str, hover_var: &str, hover_node_id: &str) -> Option<String> {
        let node_id = if !hover_node_id.is_empty() {
            Some(hover_node_id.to_string())
        } else {
            node_id_for_var(fixture_json, hover_var)
        }?;
        Some(json!({ "nodeId": node_id }).to_string())
    }

    fn graph_selection_json(fixture_json: &str, select_var: &str, selected_ids: &[String]) -> Option<String> {
        if !selected_ids.is_empty() {
            return serde_json::to_string(selected_ids).ok();
        }
        node_id_for_var(fixture_json, select_var).map(|id| serde_json::to_string(&vec![id]).unwrap_or_else(|_| "[]".into()))
    }

    fn var_from_node_name(name: &str) -> Option<String> {
        let trimmed = name.trim();
        if let Some((var, _)) = trimmed.split_once(':') {
            return Some(var.trim().into());
        }
        if let Some((var, _)) = trimmed.split_once(" : ") {
            return Some(var.trim().into());
        }
        None
    }
    /// 🩹 Delegates to `trinity_ram::parse_port_key` (the one place the `nodeId@portId` convention is
    /// owned) instead of hand-rolling a second splitter here.
    fn split_endpoint(endpoint: &str) -> (String, String) {
        trinity_ram::parse_port_key(endpoint).map_or_else(|| (endpoint.to_string(), "in".into()), |(n, p)| (n.to_string(), p.to_string()))
    }

    fn fixture_to_workflow(fixture: &GraphFixture) -> (String, String, String) {
        let nodes: Vec<WorkflowNodeRecord> = fixture.nodes.iter().map(node_to_workflow_record).collect();
        let edges: Vec<WorkflowEdgeRecord> = fixture
            .edges
            .iter()
            .map(|edge| {
                let (source_node_id, source_port_id) = split_endpoint(&edge.source);
                let (target_node_id, target_port_id) = split_endpoint(&edge.target);
                WorkflowEdgeRecord {
                    id: edge.id.clone(),
                    source_node_id,
                    source_port_id,
                    target_node_id,
                    target_port_id,
                }
            })
            .collect();
        let viewport = serde_json::to_string(&fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
        (
            serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
            serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
            viewport,
        )
    }

    fn node_to_workflow_record(node: &Node) -> WorkflowNodeRecord {
        let width = if node.width > 0.0 { node.width } else { 96.0 };
        let height = if node.height > 0.0 { node.height } else { 48.0 };
        WorkflowNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width,
            height,
            inputs: node
                .ports
                .iter()
                .filter(|port| port.direction == PortDirection::In)
                .map(|port| WorkflowDiagramPortRecord {
                    id: trinity_ram::port_key(&node.id, &port.id),
                    label: Some(port.id.clone()),
                })
                .collect(),
            outputs: node
                .ports
                .iter()
                .filter(|port| port.direction == PortDirection::Out)
                .map(|port| WorkflowDiagramPortRecord {
                    id: trinity_ram::port_key(&node.id, &port.id),
                    label: Some(port.id.clone()),
                })
                .collect(),
        }
    }
    //#endregion 🔖DocumentHelpers

    //#region 🔖Terminology
    /// 🗣️ Complete UI label set for the Rewrite rule app; one field per label makes every locale combination compile-checked.
    app_labels! {
        struct TrinityRewriteLabels {
            pieces: &'static str = en: "Pieces", de: "Stücke";
            piece: &'static str = en: "Piece", de: "Stück";
            connection: &'static str = en: "Connection", de: "Verbindung";
            connector: &'static str = en: "Connector", de: "Verbinder";
            catalogue: &'static str = en: "Catalogue", de: "Katalog";
            add_to_lhs: &'static str = en: "Add to LHS", de: "Zu LHS hinzufügen";
            add_to_rhs: &'static str = en: "Add to RHS", de: "Zu RHS hinzufügen";
            parameters: &'static str = en: "Parameters", de: "Parameter";
            geometry: &'static str = en: "Geometry", de: "Geometrie";
            identity: &'static str = en: "Identity", de: "Identität";
            history: &'static str = en: "History", de: "Verlauf";
            rule: &'static str = en: "Rule", de: "Regel";
            window_before: &'static str = en: "Before", de: "Vorher";
            window_after: &'static str = en: "After", de: "Nachher";
            window_lhs: &'static str = en: "LHS", de: "LHS";
            window_rhs: &'static str = en: "RHS", de: "RHS";
            window_jack: &'static str = en: "Jack", de: "Jack";
            window_parameters: &'static str = en: "Parameters", de: "Parameter";
        }
    }
    //#endregion 🔖Terminology

    //#region 🔖CommandLabels
    /// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_rewrite_app`'s static
    /// manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command palette
    /// and Actions rail get a translated label without threading locale through the whole builder chain.
    fn trinity_rewrite_action_labels(is_de: bool) -> HashMap<String, String> {
        localized_label_map(is_de, &[
            ("addRuleClause", "Add Rule Clause", "Regelklausel hinzufügen"),
            ("resetRule", "Reset Rule", "Regel zurücksetzen"),
            ("setParameter", "Set Parameter", "Parameter festlegen"),
            ("patchTrinityNodes", "Patch Nodes", "Knoten aktualisieren"),
            ("nodeGraphEdit", "Edit Graph", "Graph bearbeiten"),
            ("nodeGraphViewport", "Set Graph Viewport", "Graph-Ansicht festlegen"),
            ("setLhsJson", "Set LHS Json", "LHS-JSON festlegen"),
            ("setRhsJson", "Set RHS Json", "RHS-JSON festlegen"),
            ("setSelection", "Set Selection", "Auswahl festlegen"),
            ("selectNode", "Select Node", "Knoten auswählen"),
            ("nodeGraphSelect", "Select Graph Node", "Graph-Knoten auswählen"),
            ("nodeGraphHover", "Hover Graph Node", "Graph-Knoten hovern"),
            ("graphPointerDown", "Graph Pointer Down", "Graph-Zeiger gedrückt"),
            ("textSelect", "Select Text", "Text auswählen"),
            ("textHover", "Hover Text", "Text hovern"),
            ("recomputeRewrite", "Recompute Rewrite", "Rewrite neu berechnen"),
            ("reorganize", "Reorganize", "Neu anordnen"),
            ("setLodMode", "Set LOD Mode", "LOD-Modus festlegen"),
        ])
    }
    //#endregion 🔖CommandLabels

    //#region 🔖Panels
    fn build_document_tree(state: &RewriteRuleState, runtime: &RewritePlayRuntime, labels: &TrinityRewriteLabels) -> UiNode {
        let Some(fixture) = parse_fixture_json(&state.before_fixture_json) else {
            return ui_text("Invalid trinity fixture");
        };
        let builder = PanelTreeBuilder::new("trinity-document");
        let node_items: Vec<UiTreeItemNode> = fixture
            .nodes
            .iter()
            .map(|node| {
                tree_item_with_action(
                    builder.item_id("node", &node.id),
                    if node.name.is_empty() { node.id.clone() } else { node.name.clone() },
                    Some(node.kind.clone()),
                    rewrite_action("setSelection", Some(json!({ "ids": [node.id] }))),
                )
            })
            .collect();
        let selected = runtime.selected_node_ids.iter().map(|id| builder.item_id("node", id)).collect();
        builder
            .section("trinity-document.nodes", Some(labels.pieces.into()), true, node_items)
            .selected(selected)
            .selection_change(rewrite_action("setSelection", Some(json!({ "ids": [] }))))
            .build()
    }

    fn catalogue_add_item(id: &str, label: &str, clause_kind: &str) -> UiTreeItemNode {
        tree_item_with_action(id, label, None, rewrite_action("addRuleClause", Some(json!({ "kind": clause_kind }))))
}

    fn build_catalogue_tree(labels: &TrinityRewriteLabels) -> UiNode {
        PanelTreeBuilder::new("trinity-catalogue")
            .section(
                "trinity-catalogue.kinds",
                Some(labels.catalogue.into()),
                true,
                vec![
                    tree_item("trinity-catalogue.piece", labels.piece),
                    tree_item("trinity-catalogue.connection", labels.connection),
                    tree_item("trinity-catalogue.connector", labels.connector),
                ],
            )
            .section(
                "trinity-catalogue.lhs",
                Some(labels.add_to_lhs.into()),
                true,
                vec![catalogue_add_item("trinity-catalogue.add-where", "Where clause", "where")],
            )
            .section(
                "trinity-catalogue.rhs",
                Some(labels.add_to_rhs.into()),
                true,
                vec![
                    catalogue_add_item("trinity-catalogue.add-create", "Create pattern", "create"),
                    catalogue_add_item("trinity-catalogue.add-merge", "Merge pattern", "merge"),
                    catalogue_add_item("trinity-catalogue.add-set", "Set assignment", "set"),
                    catalogue_add_item("trinity-catalogue.add-delete", "Delete pattern", "delete"),
                    catalogue_add_item("trinity-catalogue.add-parameter", "Parameter", "parameter"),
                ],
            )
            .selected(vec![])
            .build()
    }

    fn flat_position_uv(node: &Node) -> (String, String) {
        let Some(flat) = node.properties.get("flatPosition").and_then(PropertyValue::as_object) else {
            return (String::new(), String::new());
        };
        let format_axis = |axis: &str| flat.get(axis).and_then(PropertyValue::as_f64).map(|value| format!("{value:.2}")).unwrap_or_default();
        (format_axis("u"), format_axis("v"))
    }

    fn fixture_with_derived(fixture_json: &str) -> Option<GraphFixture> {
        let mut graph = Graph::load_json(fixture_json).ok()?;
        graph.recompute_derived();
        Some(graph.to_fixture())
    }

    fn build_inspector_tree(state: &RewriteRuleState, runtime: &RewritePlayRuntime, term_labels: &TrinityRewriteLabels) -> UiNode {
        let Some(fixture) = parse_fixture_json(&state.before_fixture_json) else {
            return ui_text("Invalid trinity fixture");
        };
        if runtime.selected_node_ids.is_empty() {
            return ui_declarative_sections_to_tree(&[UiSectionNode {
                id: "trinity-inspector.empty".into(),
                label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
                default_open: Some(true),
                presence: UiPresence::default(),
                children: vec![ui_text("Select one or more pieces")],
            }]);
        }
        let nodes: Vec<&Node> = runtime
            .selected_node_ids
            .iter()
            .filter_map(|id| fixture.nodes.iter().find(|node| &node.id == id))
            .collect();
        if nodes.is_empty() {
            return ui_text("Piece not found");
        }
        let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
        let name_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>());
        let kind_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.kind.clone()).collect::<Vec<_>>());
        let derived_fixture = fixture_with_derived(&state.before_fixture_json);
        let derived_uv = |id: &str| -> (String, String) {
            derived_fixture
                .as_ref()
                .and_then(|fixture| fixture.nodes.iter().find(|node| node.id == id))
                .map(flat_position_uv)
                .unwrap_or_default()
        };
        let u_values: Vec<String> = node_ids.iter().map(|id| derived_uv(id).0).collect();
        let v_values: Vec<String> = node_ids.iter().map(|id| derived_uv(id).1).collect();
        let u_mixed = ui_inspector_mixed_text(&u_values);
        let v_mixed = ui_inspector_mixed_text(&v_values);
        ui_inspector_groups_to_tree(&[
            UiInspectorFieldGroup { presence: UiPresence::default(),
                id: "trinity-inspector.geometry".into(),
                label: term_labels.geometry.into(),
                default_open: None,
                fields: vec![
                    ui_inspector_readonly_field(
                        "trinity-inspector.flat-u",
                        "Flat U",
                        if u_mixed.placeholder.is_none() { u_values.first().cloned().unwrap_or_default() } else { u_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                    ),
                    ui_inspector_readonly_field(
                        "trinity-inspector.flat-v",
                        "Flat V",
                        if v_mixed.placeholder.is_none() { v_values.first().cloned().unwrap_or_default() } else { v_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                    ),
                ],
            },
            UiInspectorFieldGroup {
                presence: UiPresence::default(),
                id: "trinity-inspector.identity".into(),
                label: term_labels.identity.into(),
                default_open: None,
                fields: vec![
                    semio_framework_plugin::UiNode::Field(UiFieldNode {presence: UiPresence::default(), 
                        id: "trinity-inspector.name".into(),
                        label: "Name".into(),
                        child: Box::new(semio_framework_plugin::UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(), 
                            id: "trinity-inspector.name.input".into(),
                            input_kind: "text".into(),
                            value: name_mixed.value,
                            placeholder: name_mixed.placeholder,
                            commit: None,
                            on_change: rewrite_action("patchTrinityNodes", Some(json!({ "nodeIds": node_ids, "field": "name" }))),
                            min: None,
                            max: None,
                            step: None,
                            accept: None,
                        })),
                        description: None,
                        required: None,
                        error: None,
                    }),
                    ui_inspector_readonly_field(
                        "trinity-inspector.kind",
                        "Kind",
                        if kind_mixed.placeholder.is_none() {
                            nodes.first().map(|node| node.kind.clone()).unwrap_or_default()
                        } else {
                            kind_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                        },
                    ),
                ],
            },
        ])
    }

    fn build_parameters_panel(state: &RewriteRuleState, labels: &TrinityRewriteLabels) -> UiNode {
        let Ok(rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
            return ui_text("Invalid RHS");
        };
        let mut children: Vec<UiNode> = Vec::new();
        for param in &rhs.parameters {
            let value = state
                .parameter_bindings
                .get(&param.name)
                .cloned()
                .unwrap_or_else(|| param.default.clone());
            let display = match value {
                PropertyValue::String(text) => text,
                PropertyValue::Number(number) => number.to_string(),
                PropertyValue::Bool(flag) => flag.to_string(),
                _ => String::new(),
            };
            children.push(semio_framework_plugin::UiNode::Field(UiFieldNode {presence: UiPresence::default(), 
                id: format!("trinity-rewrite.param.{}", param.name),
                label: param.name.clone(),
                child: Box::new(semio_framework_plugin::UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(), 
                    id: format!("trinity-rewrite.param.{}.input", param.name),
                    input_kind: match param.kind {
                        ParameterKind::Number => "number",
                        ParameterKind::Boolean => "text",
                        ParameterKind::String => "text",
                    }
                    .into(),
                    value: display,
                    placeholder: Some(param.kind_label()),
                    commit: Some("blur".into()),
                    on_change: rewrite_action("setParameter", Some(json!({ "name": param.name }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }));
        }
        if children.is_empty() {
            children.push(ui_text("No parameters declared on RHS."));
        }
        ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-rewrite.parameters".into(),
            label: Some(labels.parameters.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            children,
        }])
    }

    trait ParameterKindLabel {
        fn kind_label(&self) -> String;
    }

    impl ParameterKindLabel for ParameterSpec {
        fn kind_label(&self) -> String {
            match self.kind {
                ParameterKind::String => "string".into(),
                ParameterKind::Number => "number".into(),
                ParameterKind::Boolean => "boolean".into(),
            }
        }
    }
    //#endregion 🔖Panels

    //#region 🔖Render
    const DELETE_SELECTION_CONTEXT_MENU: &str =
        r#"[{"id":"delete-selection","label":"Delete selection","icon":"trash","action":"nodeGraphEdit","args":{"operations":[{"operation":"deleteSelection"}]},"destructive":true}]"#;

    fn rewrite_lod_json_for_window(runtime: &RewritePlayRuntime, window_id: &str) -> Option<String> {
        let mode = runtime.lod_mode_by_window.get(window_id).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
        if mode == TRINITY_LOD_MODE_AUTOMATIC {
            Some(json!({ "automatic": true }).to_string())
        } else {
            Some(json!({ "automatic": false, "forcedLabel": mode }).to_string())
        }
    }

    fn trinity_rewrite_lod_measure(window_id: &str, current_mode: &str) -> WindowMeasure {
        let mut items = vec![MeasureSelectItem { id: TRINITY_LOD_MODE_AUTOMATIC.into(), value: TRINITY_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() }];
        let rows: Vec<Value> = serde_json::from_str(&trinity_lod_scale_json()).unwrap_or_default();
        items.extend(rows.into_iter().filter_map(|row| {
            let id = row.get("id")?.as_str()?.to_string();
            let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
            Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
        }));
        WindowMeasure::Select {
            id: format!("{window_id}-lod"),
            label: Some("LOD".into()),
            value: current_mode.into(),
            items,
            on_change: rewrite_action("setLodMode", Some(json!({ "windowId": window_id }))),
        }
    }

    fn jack_token_at_offset(text: &str, offset: usize) -> Option<String> {
        if offset >= text.len() {
            return None;
        }
        let slice = &text[offset..];
        let token: String = slice.chars().take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_').collect();
        if token.is_empty() {
            None
        } else {
            Some(token)
        }
    }

    fn render_rule_graph(
        surface_id: &str,
        window_id: &str,
        fixture_json: &str,
        runtime: &RewritePlayRuntime,
        hover_node_id: &str,
        editable: bool,
    ) -> UiNode {
        let fixture = parse_fixture_json(fixture_json).unwrap_or_else(|| GraphFixture::parse_dsl(NAKAGIN_FIXTURE_DSL).unwrap());
        let (nodes_json, edges_json, viewport_json) = fixture_to_workflow(&fixture);
        let hover_json = graph_hover_json(fixture_json, &runtime.active_hover_var, hover_node_id);
        let selection_json = graph_selection_json(fixture_json, &runtime.active_select_var, &runtime.selected_node_ids);
        build_node_graph_scene(
            surface_id,
            TRINITY_REWRITE_PLAY_CONTROLLER_ID,
            NodeGraphScene {
                hover_json,
                selection_json,
                lod_json: rewrite_lod_json_for_window(runtime, window_id),
                editable: editable.then_some(true),
                context_menu_json: editable.then(|| DELETE_SELECTION_CONTEXT_MENU.into()),
                ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
            },
        )
    }

    fn render_fixture_graph(surface_id: &str, window_id: &str, fixture_json: &str, runtime: &RewritePlayRuntime, editable: bool) -> UiNode {
        render_rule_graph(surface_id, window_id, fixture_json, runtime, "", editable)
    }

    fn var_occurrences_json(text: &str, var: &str) -> Option<String> {
        if var.is_empty() {
            return None;
        }
        let mut ranges = Vec::new();
        let mut scan = 0usize;
        while let Some(found) = text[scan..].find(var) {
            let at = scan + found;
            let end = at + var.len();
            if text_identifier_bounds_at(text, at) == Some((at, end)) {
                ranges.push(json!({ "start": at, "end": end }));
            }
            scan = at + var.len();
        }
        if ranges.is_empty() {
            return None;
        }
        let ranges_json = serde_json::to_string(&ranges).unwrap_or_else(|_| "[]".into());
        Some(json!({ "selection": ranges_json, "hover": ranges_json }).to_string())
    }

    fn render_jack_editor(state: &RewriteRuleState, runtime: &RewritePlayRuntime) -> UiNode {
        let query = compiled_jack_query(state);
        let active_var = if !runtime.active_hover_var.is_empty() {
            runtime.active_hover_var.as_str()
        } else {
            runtime.active_select_var.as_str()
        };
        build_text_editor_scene(
            TRINITY_REWRITE_PLAY_SURFACE_JACK,
            TRINITY_REWRITE_PLAY_CONTROLLER_ID,
            TextEditorScene {
                tokens_json: serde_json::to_string(&semantic_tokens(&query)).ok(),
                occurrences_json: var_occurrences_json(&query, active_var),
                ..TextEditorScene::base(query, Some("jack".into()), None)
            },
        )
    }
    //#endregion 🔖Render

    //#region 🔖TrinityRewritePlayApp
    /// ♻️ Trinity Rewrite play app — a parametric-rewrite editor over a {@link RewriteRuleState}
    /// projection. Every rule/parameter/before-fixture mutation flows through the single LWW
    /// {@link RewriteRuleOperation::SetState}; hover/select var focus, epochs and LOD are runtime.
    #[derive(Default)]
    pub struct TrinityRewritePlayApp {
        runtime: RewritePlayRuntime,
    }

    impl DocumentApp for TrinityRewritePlayApp {
        type Projection = RewriteRuleState;
        type Operation = RewriteRuleOperation;

        fn app_id(&self) -> &str {
            TRINITY_REWRITE_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            REWRITE_RULE_SCHEMA
        }

        fn initial_projection(&self) -> RewriteRuleState {
            default_rule_state()
        }

        fn handle_action(
            &mut self,
            action: &str,
            args: Option<&Value>,
            doc: &DocumentView<'_, RewriteRuleState>,
            _view_state: &ViewState,
        ) -> ActionEmit<RewriteRuleOperation> {
            let state = doc.projection;
            match action {
                "setSelection" | "selectNode" | "nodeGraphSelect" => {
                    self.runtime.selected_node_ids = selection_ids(args);
                    let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                    if let Some(node_id) = self.runtime.selected_node_ids.first().cloned() {
                        let fixture_json = fixture_json_for_surface(surface_id, state);
                        sync_select_var_from_node(&mut self.runtime, &fixture_json, &node_id);
                        self.runtime.select_epoch += 1;
                    }
                    ActionEmit::default()
                }
                "nodeGraphHover" => {
                    let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                    let node_id = args
                        .and_then(|value| value.get("hoverJson"))
                        .and_then(|value| {
                            if value.is_null() {
                                None
                            } else if let Some(text) = value.as_str() {
                                serde_json::from_str::<Value>(text)
                                    .ok()
                                    .and_then(|parsed| parsed.get("nodeId").and_then(|id| id.as_str().map(str::to_string)))
                            } else {
                                value.get("nodeId").and_then(|id| id.as_str().map(str::to_string))
                            }
                        });
                    if let Some(node_id) = node_id {
                        let fixture_json = fixture_json_for_surface(surface_id, state);
                        sync_hover_var_from_node(&mut self.runtime, &fixture_json, &node_id);
                    }
                    ActionEmit::default()
                }
                "nodeGraphViewport" => {
                    let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                    if surface_id == TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                        if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                            if let Ok(camera) = serde_json::from_str::<Camera>(viewport_json) {
                                let mut next = state.clone();
                                if let Some(mut fixture) = parse_fixture_json(&next.before_fixture_json) {
                                    fixture.camera = camera;
                                    if let Ok(json) = Graph::from_fixture(fixture).and_then(|graph| graph.fixture_json()) {
                                        next.before_fixture_json = json;
                                        return ActionEmit::amend(vec![RewriteRuleOperation::SetState { state: next }], "viewport");
                                    }
                                }
                            }
                        }
                    }
                    ActionEmit::default()
                }
                "nodeGraphEdit" => {
                    let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                    let operations = args
                        .and_then(|value| value.get("operations"))
                        .and_then(|value| value.as_array())
                        .cloned()
                        .unwrap_or_default();
                    let mut next = state.clone();
                    if apply_rewrite_node_graph_edit_operations(&mut next, &mut self.runtime, surface_id, &operations) {
                        set_state_emit(state, next)
                    } else {
                        ActionEmit::default()
                    }
                }
                "setLhsJson" => {
                    if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                        let mut next = state.clone();
                        next.lhs_json = value.into();
                        return set_state_emit(state, next);
                    }
                    ActionEmit::default()
                }
                "setRhsJson" => {
                    if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                        let mut next = state.clone();
                        next.rhs_json = value.into();
                        next.parameter_bindings = default_parameter_bindings(&next.rhs_json);
                        return set_state_emit(state, next);
                    }
                    ActionEmit::default()
                }
                "setParameter" => {
                    let name = args.and_then(|v| v.get("name")).and_then(|v| v.as_str()).unwrap_or("");
                    let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()).unwrap_or("");
                    if !name.is_empty() {
                        let mut next = state.clone();
                        let Ok(rhs) = serde_json::from_str::<Rhs>(&next.rhs_json) else {
                            return ActionEmit::default();
                        };
                        let kind = rhs
                            .parameters
                            .iter()
                            .find(|param| param.name == name)
                            .map(|param| param.kind.clone());
                        let parsed = match kind {
                            Some(ParameterKind::Number) => value.parse::<f64>().ok().map(PropertyValue::Number),
                            Some(ParameterKind::Boolean) => Some(PropertyValue::Bool(value.eq_ignore_ascii_case("true"))),
                            Some(ParameterKind::String) | None => Some(PropertyValue::String(value.into())),
                        };
                        if let Some(parsed) = parsed {
                            next.parameter_bindings.insert(name.into(), parsed);
                            return set_state_emit(state, next);
                        }
                    }
                    ActionEmit::default()
                }
                "addRuleClause" => {
                    let kind = args.and_then(|v| v.get("kind")).and_then(|v| v.as_str()).unwrap_or("");
                    let mut next = state.clone();
                    if add_rule_clause(&mut next, kind) {
                        return set_state_emit(state, next);
                    }
                    ActionEmit::default()
                }
                "recomputeRewrite" | "reorganize" => {
                    self.runtime.reorganize_epoch += 1;
                    ActionEmit::default()
                }
                "resetRule" => set_state_emit(state, default_rule_state()),
                "graphPointerDown" => {
                    if let Some(node_id) = args.and_then(|v| v.get("nodeId")).and_then(|v| v.as_str()) {
                        self.runtime.selected_node_ids = vec![node_id.into()];
                    }
                    ActionEmit::default()
                }
                "patchTrinityNodes" => {
                    let node_ids: Vec<String> = args
                        .and_then(|v| v.get("nodeIds"))
                        .and_then(|v| serde_json::from_value(v.clone()).ok())
                        .unwrap_or_default();
                    let field = args.and_then(|v| v.get("field")).and_then(|v| v.as_str()).unwrap_or("");
                    let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()).map(str::trim).unwrap_or("");
                    if !node_ids.is_empty() && !field.is_empty() && !value.is_empty() {
                        let mut next = state.clone();
                        if let Some(patched) = patch_fixture_nodes(&next.before_fixture_json, &node_ids, field, value) {
                            next.before_fixture_json = patched;
                            return set_state_emit(state, next);
                        }
                    }
                    ActionEmit::default()
                }
                "textSelect" => {
                    if let Some(var) = args.and_then(|v| v.get("var")).and_then(|v| v.as_str()) {
                        self.runtime.active_select_var = var.into();
                    } else if let Some(start) = args.and_then(|v| v.get("start")).and_then(|v| v.as_u64()) {
                        if let Some(token) = jack_token_at_offset(&compiled_jack_query(state), start as usize) {
                            self.runtime.active_select_var = token;
                        }
                    }
                    self.runtime.select_epoch += 1;
                    ActionEmit::default()
                }
                "textHover" => {
                    if let Some(var) = args.and_then(|v| v.get("var")).and_then(|v| v.as_str()) {
                        self.runtime.active_hover_var = var.into();
                    } else if let Some(offset) = args.and_then(|v| v.get("offset")).and_then(|v| v.as_u64()) {
                        if let Some(token) = jack_token_at_offset(&compiled_jack_query(state), offset as usize) {
                            self.runtime.active_hover_var = token;
                        }
                    }
                    self.runtime.hover_epoch += 1;
                    ActionEmit::default()
                }
                "setLodMode" => {
                    if let (Some(window_id), Some(value)) = (
                        args.and_then(|v| v.get("windowId")).and_then(|v| v.as_str()),
                        args.and_then(|v| v.get("value")).and_then(|v| v.as_str()),
                    ) {
                        self.runtime.lod_mode_by_window.insert(window_id.into(), value.into());
                    }
                    ActionEmit::default()
                }
                _ => ActionEmit::default(),
            }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, RewriteRuleState>, view_state: &ViewState) -> UiNode {
            let state = doc.projection;
            let runtime = &self.runtime;
            let labels = resolve_labels::<TrinityRewriteLabels>(view_state);
            match body_key {
                TRINITY_REWRITE_PLAY_BODY_BEFORE => render_fixture_graph(
                    TRINITY_REWRITE_PLAY_SURFACE_BEFORE,
                    TRINITY_REWRITE_PLAY_WINDOW_BEFORE,
                    &state.before_fixture_json,
                    runtime,
                    true,
                ),
                TRINITY_REWRITE_PLAY_BODY_AFTER => render_fixture_graph(
                    TRINITY_REWRITE_PLAY_SURFACE_AFTER,
                    TRINITY_REWRITE_PLAY_WINDOW_AFTER,
                    &after_fixture_json(state),
                    runtime,
                    false,
                ),
                TRINITY_REWRITE_PLAY_BODY_LHS => render_fixture_graph(
                    TRINITY_REWRITE_PLAY_SURFACE_LHS,
                    TRINITY_REWRITE_PLAY_WINDOW_LHS,
                    &lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout),
                    runtime,
                    true,
                ),
                TRINITY_REWRITE_PLAY_BODY_RHS => render_fixture_graph(
                    TRINITY_REWRITE_PLAY_SURFACE_RHS,
                    TRINITY_REWRITE_PLAY_WINDOW_RHS,
                    &rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout),
                    runtime,
                    true,
                ),
                TRINITY_REWRITE_PLAY_BODY_JACK => render_jack_editor(state, runtime),
                TRINITY_REWRITE_PLAY_BODY_PARAMETERS => build_parameters_panel(state, labels),
                TRINITY_REWRITE_PLAY_BODY_DOCUMENT => build_document_tree(state, runtime, labels),
                TRINITY_REWRITE_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
                TRINITY_REWRITE_PLAY_BODY_INSPECTION => build_inspector_tree(state, runtime, labels),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn window_measures(&self, _doc: &DocumentView<'_, RewriteRuleState>, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
            let mode_for = |window_id: &str| self.runtime.lod_mode_by_window.get(window_id).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
            HashMap::from([
                (TRINITY_REWRITE_PLAY_WINDOW_BEFORE.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, mode_for(TRINITY_REWRITE_PLAY_WINDOW_BEFORE))]),
                (TRINITY_REWRITE_PLAY_WINDOW_AFTER.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_AFTER, mode_for(TRINITY_REWRITE_PLAY_WINDOW_AFTER))]),
                (TRINITY_REWRITE_PLAY_WINDOW_LHS.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_LHS, mode_for(TRINITY_REWRITE_PLAY_WINDOW_LHS))]),
                (TRINITY_REWRITE_PLAY_WINDOW_RHS.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_RHS, mode_for(TRINITY_REWRITE_PLAY_WINDOW_RHS))]),
            ])
        }

        fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
            let labels = resolve_labels::<TrinityRewriteLabels>(view_state);
            AppLabelsOverlay::default()
                .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, labels.window_before)
                .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_AFTER, labels.window_after)
                .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_LHS, labels.window_lhs)
                .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_RHS, labels.window_rhs)
                .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_JACK, labels.window_jack)
                .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS, labels.window_parameters)
                .action_labels(trinity_rewrite_action_labels(is_de_locale(view_state)))
        }
    }
    //#endregion 🔖TrinityRewritePlayApp

    //#region 🔖Manifest
    fn rewrite_window_stack(id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
        WindowLayoutChild::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode {
                kind: "window".into(),
                window_kind_id: id.into(),
                title: Some(title.into()),
                instance_id: None,
                template_id: None,
            }],
        })
    }

    fn rewrite_layout() -> WindowLayout {
        WindowLayout {
            root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
                kind: "column".into(),
                size: None,
                children: vec![
                    WindowLayoutChild::Axis(WindowLayoutAxisNode {
                        kind: "row".into(),
                        size: Some(0.5),
                        children: vec![
                            rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_LHS, "LHS", Some(0.34)),
                            rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_RHS, "RHS", Some(0.34)),
                            rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_JACK, "Jack", Some(0.32)),
                        ],
                    }),
                    WindowLayoutChild::Axis(WindowLayoutAxisNode {
                        kind: "row".into(),
                        size: Some(0.5),
                        children: vec![
                            rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS, "Parameters", Some(0.34)),
                            rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, "Before", Some(0.33)),
                            rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_AFTER, "After", Some(0.33)),
                        ],
                    }),
                ],
            }),
        }
    }

    pub fn create_rewrite_app() -> App {
        App::from_builder(
            App::builder(TRINITY_REWRITE_PLAY_APP_ID, "Trinity Rewrite").document(["semio", "trinity", "rewrite"])
                .icon_id("trinity-rewrite")
                .mode("explore", "Explore")
                .default_mode_id("explore")
                .window_kind(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, "Before", TRINITY_REWRITE_PLAY_BODY_BEFORE, SurfaceKind::NodeGraph, "git-branch")
                .window_kind(TRINITY_REWRITE_PLAY_WINDOW_AFTER, "After", TRINITY_REWRITE_PLAY_BODY_AFTER, SurfaceKind::NodeGraph, "arrow-right")
                .window_kind(TRINITY_REWRITE_PLAY_WINDOW_LHS, "LHS", TRINITY_REWRITE_PLAY_BODY_LHS, SurfaceKind::NodeGraph, "trinity-lhs")
                .window_kind(TRINITY_REWRITE_PLAY_WINDOW_RHS, "RHS", TRINITY_REWRITE_PLAY_BODY_RHS, SurfaceKind::NodeGraph, "trinity-rhs")
                .window_kind(TRINITY_REWRITE_PLAY_WINDOW_JACK, "Jack", TRINITY_REWRITE_PLAY_BODY_JACK, SurfaceKind::TextEditor, "document-jack")
                .window_kind(
                    TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS,
                    "Parameters",
                    TRINITY_REWRITE_PLAY_BODY_PARAMETERS,
                    SurfaceKind::Canvas2d,
                    "settings-2",
                )
                .default_layout(rewrite_layout())
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                    PanelGroup::Workbench,
                    TRINITY_REWRITE_PLAY_BODY_DOCUMENT,
                )
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                    PanelGroup::Workbench,
                    TRINITY_REWRITE_PLAY_BODY_CATALOGUE,
                )
                .panel_tab(
                    FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                    PanelGroup::Details,
                    TRINITY_REWRITE_PLAY_BODY_INSPECTION,
                )
                // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
                .operation("addRuleClause", "Add Rule Clause")
                .operation("resetRule", "Reset Rule")
                .operation("setParameter", "Set Parameter")
                .operation("patchTrinityNodes", "Patch Nodes")
                .operation("nodeGraphEdit", "Edit Graph")
                .operation("nodeGraphViewport", "Set Graph Viewport")
                // 🛠️ Dev-only raw rule editors — kept out of the command palette.
                .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setLhsJson", "Set LHS Json", ActionKind::Operation) })
                .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setRhsJson", "Set RHS Json", ActionKind::Operation) })
                // 👁️ Ephemeral view state — selection, hover, text cursor, recompute/layout, LOD.
                .view_action("setSelection", "Set Selection")
                .view_action("selectNode", "Select Node")
                .view_action("nodeGraphSelect", "Select Graph Node")
                .view_action("nodeGraphHover", "Hover Graph Node")
                .view_action("graphPointerDown", "Graph Pointer Down")
                .view_action("textSelect", "Select Text")
                .view_action("textHover", "Hover Text")
                .view_action("recomputeRewrite", "Recompute Rewrite")
                .view_action("reorganize", "Reorganize")
                .view_action("setLodMode", "Set LOD Mode")
                // 📝 Staged argument forms.
                .action_args("addRuleClause", vec![
                    ActionArgDef::select("kind", "Clause", vec![
                        ActionArgOption::new("where", "Where"),
                        ActionArgOption::new("create", "Create"),
                        ActionArgOption::new("merge", "Merge"),
                        ActionArgOption::new("set", "Set"),
                        ActionArgOption::new("delete", "Delete"),
                        ActionArgOption::new("parameter", "Parameter"),
                    ]).required(),
                ])
                .action_args("setLhsJson", vec![ActionArgDef::text("value", "LHS JSON").required()])
                .action_args("setRhsJson", vec![ActionArgDef::text("value", "RHS JSON").required()])
                .keybinding("mod+z", "undo")
                .keybinding("mod+shift+z", "redo")
                .keybinding("mod+alt+s", "commitCheckpoint"),
        )
        .example("label-core", "Label Core", default_rule_state().print_dsl())
        .program("trinity-rewrite", "Trinity Rewrite", "graph")
    }
    //#endregion 🔖Manifest

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{testkit, ActionMeta, PluginApp, VcsDocumentApp};

        fn meta(actor: &str) -> ActionMeta {
            testkit::meta(actor)
        }

        fn new_app() -> VcsDocumentApp<TrinityRewritePlayApp> {
            testkit::new_app()
        }

        fn dispatch(app: &mut VcsDocumentApp<TrinityRewritePlayApp>, action: &str, args: Option<&Value>) -> semio_framework_plugin::kernel::InvocationResult {
            app.handle_action(action, args, &ViewState::default(), &meta("local")).expect("dispatch")
        }

        #[test]
        fn renders_before_and_after_graphs() {
            let mut app = new_app();
            let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, None, &ViewState::default()).expect("render");
            let after = app.render(TRINITY_REWRITE_PLAY_BODY_AFTER, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&before).unwrap().contains("node-graph"));
            assert!(serde_json::to_string(&after).unwrap().contains("node-graph"));
        }

        #[test]
        fn compiles_jack_query_from_rule() {
            let query = compiled_jack_query(&default_rule_state());
            assert!(query.contains("MATCH"));
            assert!(query.contains("SET"));
        }

        #[test]
        fn apply_rewrite_changes_after_fixture() {
            let state = default_rule_state();
            assert_ne!(state.before_fixture_json, after_fixture_json(&state));
        }

        #[test]
        fn renders_lhs_rhs_graphs() {
            let mut app = new_app();
            let lhs_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_LHS, None, &ViewState::default()).expect("render")).unwrap();
            let rhs_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_RHS, None, &ViewState::default()).expect("render")).unwrap();
            assert!(lhs_json.contains("node-graph"));
            assert!(rhs_json.contains("node-graph"));
            assert!(lhs_json.contains("\"editable\":true"));
            assert!(rhs_json.contains("\"editable\":true"));
        }

        #[test]
        fn set_parameter_emits_one_op_and_is_undoable() {
            let mut app = new_app();
            let result = dispatch(&mut app, "setParameter", Some(&json!({ "name": "label", "value": "changed" })));
            assert_eq!(result.operations.len(), 1, "a parameter edit is a single SetState operation");
            assert_eq!(app.projection().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("changed".into())));
            dispatch(&mut app, "undo", None);
            assert_eq!(app.projection().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
        }

        #[test]
        fn commit_checkpoint_records_change_and_stays_undoable() {
            let mut app = new_app();
            dispatch(&mut app, "setParameter", Some(&json!({ "name": "label", "value": "changed" })));
            dispatch(&mut app, "commitCheckpoint", None);
            let envelope: Value = serde_json::from_str(&app.document_json().expect("document json")).unwrap();
            assert!(envelope["vcs"]["checkpoints"].as_array().map(|c| !c.is_empty()).unwrap_or(false), "checkpoint should be recorded");
            dispatch(&mut app, "undo", None);
            assert_eq!(app.projection().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
        }

        #[test]
        fn add_and_delete_rhs_set_clause() {
            let mut app = new_app();
            dispatch(&mut app, "addRuleClause", Some(&json!({ "kind": "set" })));
            let rhs: Rhs = serde_json::from_str(&app.projection().unwrap().rhs_json).unwrap();
            assert_eq!(rhs.set.len(), 2);
            // deleteSelection requires a prior selection; select the newly added clause first (runtime).
            dispatch(&mut app, "setSelection", Some(&json!({ "ids": ["rhs-set-1"], "surfaceId": TRINITY_REWRITE_PLAY_SURFACE_RHS })));
            let result = dispatch(&mut app, "nodeGraphEdit", Some(&json!({ "surfaceId": TRINITY_REWRITE_PLAY_SURFACE_RHS, "operations": [{ "operation": "deleteSelection" }] })));
            assert!(!result.operations.is_empty());
            let rhs: Rhs = serde_json::from_str(&app.projection().unwrap().rhs_json).unwrap();
            assert_eq!(rhs.set.len(), 1);
        }

        #[test]
        fn jack_view_has_occurrences_after_select() {
            let mut app = new_app();
            let result = dispatch(&mut app, "textSelect", Some(&json!({ "var": "a" })));
            assert!(result.operations.is_empty(), "text selection is a view action, no operations");
            let node = app.render(TRINITY_REWRITE_PLAY_BODY_JACK, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("occurrencesJson"));
        }

        #[test]
        fn graph_scenes_have_lod_json() {
            let mut app = new_app();
            let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&before).unwrap().contains("lodJson"));
        }

        // 🧰 `VcsDocumentApp::tools()` no longer exists — utility bars are now derived by the renderer
        // from the utility registry, which this app declares none of. `reorganize` is a plain view
        // action and `undo` is a framework-injected History action; both still live in the static
        // `AppDefinition.actions` list.
        #[test]
        fn app_definition_declares_reorganize_and_history_actions() {
            let definition = create_rewrite_app().definition;
            let action_ids: Vec<&str> = definition.actions.iter().map(|action| action.id.as_str()).collect();
            assert!(action_ids.contains(&"undo"));
            assert!(action_ids.contains(&"reorganize"));
        }

        #[test]
        fn trinity_rewrite_labels_resolve_native_by_default() {
            let mut app = new_app();
            let json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
            assert!(json.contains("\"Pieces\""));
            assert!(!json.contains("Stücke"));
        }

        #[test]
        fn trinity_rewrite_labels_translate_panels_in_german() {
            let mut app = new_app();
            let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
            let document_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_DOCUMENT, None, &view_state).expect("render")).unwrap();
            assert!(document_json.contains("Stücke"));
            assert!(!document_json.contains("\"Pieces\""));
            let catalogue_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_CATALOGUE, None, &view_state).expect("render")).unwrap();
            assert!(catalogue_json.contains("Katalog"));
            assert!(catalogue_json.contains("Zu LHS hinzufügen"));
            assert!(catalogue_json.contains("Zu RHS hinzufügen"));
            let parameters_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_PARAMETERS, None, &view_state).expect("render")).unwrap();
            assert!(parameters_json.contains("\"Parameter\""));
            // 🧰 `VcsDocumentApp::tools()` no longer exists (see the removed utility-bar test above); the
            // "Verlauf" (History rail group) label had no per-app surface even before removal — only
            // the `resetRule` action label is this app's own to assert on.
            let action_labels = app.app_labels(&view_state).action_labels;
            assert_eq!(action_labels.get("resetRule").map(String::as_str), Some("Regel zurücksetzen"));
        }

        #[test]
        fn set_lhs_json_undo_redo_round_trip() {
            let mut app = new_app();
            let original = app.projection().unwrap().lhs_json;
            let next_lhs = r#"{"pattern":{"leftVar":"x","leftKind":"Piece","edgeVar":"r","edgeKind":"Connection","rightVar":"y","rightKind":"Piece"}}"#;
            testkit::assert_undo_redo_round_trip(
                &mut app,
                "setLhsJson",
                Some(&json!({ "value": next_lhs })),
                |app| app.projection().unwrap().lhs_json,
                original,
                next_lhs.to_string(),
            );
        }
    }
    //#endregion 🧪Tests
}

//#region 🔖Bundle
/// 🗂️ Registers this crate's two document kinds' pack↔dsl codecs so `framework/sync`'s
/// `FolderEndpoint::Pack` (and any other schema-string-keyed caller) can print/parse them without
/// depending on `trinity_ram`/`trinity_rewrite`'s concrete `Projection`/`Operation` types.
fn register_trinity_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<app_jack::TrinityJackPlayApp>(trinity_ram::TRINITY_GRAPH_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<app_rewrite::TrinityRewritePlayApp>(trinity_rewrite::REWRITE_RULE_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "trinity",
    label: "Trinity",
    version: "0.1.0",
    setup: register_trinity_exports,
    apps: [
        app_jack::create_trinity_jack_app => app_jack::TrinityJackPlayApp,
        app_rewrite::create_rewrite_app => app_rewrite::TrinityRewritePlayApp,
    ]
}
//#endregion 🔖Bundle
