//! 🗺️ Mindmap plugin — WIRES app in a hot-swappable WASM component.

pub mod app_wires {
    //! 🧠 Mindmap Wires plugin — declarative WIRES play app bundled as a hot-swappable WASM component.

    use puzzle_2d::Puzzle2dExtension;
    use reasoning_mindmap::{
        empty_mindmap_wires_document, find_board_node, MindmapWiresDocument, MindmapWiresOp,
    };
    use reasoning_mindmap_wires::{DefaultWiresExtension, RelationshipKind};
    use semio_framework_plugin::{SurfaceKind,
        build_canvas_2d_scene, create_default_layout, ui_inspector_readonly_field, ui_stack_vertical, ui_text,
        ActionEmit, App, Canvas2dScene, ActionDescriptor, DocumentApp, DocumentView, PanelGroup, UiNode, UiTreeItemNode,
        UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
        FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Map, Value};

    //#region 🔖Constants
    const WIRES_PLAY_APP_ID: &str = "reasoning-wires-play";
    const WIRES_PLAY_CONTROLLER_ID: &str = "reasoning-wires-play";
    const WIRES_PLAY_SURFACE_ID: &str = "reasoning.wires.composite";
    const WIRES_PLAY_BODY_COMPOSITE: &str = "reasoning.wires.composite";
    const WIRES_PLAY_BODY_DOCUMENT: &str = "reasoning.wires.document";
    const WIRES_PLAY_BODY_CATALOGUE: &str = "reasoning.wires.catalogue";
    const WIRES_PLAY_BODY_PROPERTIES: &str = "reasoning.wires.properties";
    const WIRES_FIXTURE_SCHEMA: &str = "reasoning.wires.fixture";
    const PUZZLE2D_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
    const WIRES_PLAY_EXAMPLE_METABOLISM_ID: &str = "metabolism";
    const METABOLISM_WIRES_EXAMPLE_JSON: &str = include_str!("../../wires/example/metabolism.wires.json");

    const WIRES_DOCUMENT_IDENTITY_PREFIX: &str = "wires-play-document.identity.";
    const WIRES_DOCUMENT_RELATIONSHIP_PREFIX: &str = "wires-play-document.relationship.";
    //#endregion 🔖Constants

    //#region 🔖Terminology
    /// 🗣️ Complete UI label set for the mindmap wires app; one field per label makes every locale combination compile-checked.
    struct WiresLabels {
        identities: &'static str,
        relationships: &'static str,
        identity_kinds: &'static str,
        relationship_kinds: &'static str,
        relationship_kind_owns: &'static str,
        relationship_kind_is: &'static str,
        relationship_kind_references: &'static str,
        relationship_kind_has: &'static str,
        window_main: &'static str,
        mode_edit: &'static str,
    }

    const WIRES_LABELS_NATIVE_EN: WiresLabels = WiresLabels {
        identities: "Identities",
        relationships: "Relationships",
        identity_kinds: "Identity kinds",
        relationship_kinds: "Relationship kinds",
        relationship_kind_owns: "Owns",
        relationship_kind_is: "Is",
        relationship_kind_references: "References",
        relationship_kind_has: "Has",
        window_main: "Canvas",
        mode_edit: "Edit",
    };

    const WIRES_LABELS_NATIVE_DE: WiresLabels = WiresLabels {
        identities: "Identitäten",
        relationships: "Beziehungen",
        identity_kinds: "Identitätsarten",
        relationship_kinds: "Beziehungsarten",
        relationship_kind_owns: "Besitzt",
        relationship_kind_is: "Ist",
        relationship_kind_references: "Referenziert",
        relationship_kind_has: "Hat",
        window_main: "Leinwand",
        mode_edit: "Bearbeiten",
    };

    /// 🗣️ Resolves the active label set from the shell-provided locale; unknown locales fall back to native English.
    fn wires_labels(view_state: &ViewState) -> &'static WiresLabels {
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        if is_de {
            &WIRES_LABELS_NATIVE_DE
        } else {
            &WIRES_LABELS_NATIVE_EN
        }
    }
    //#endregion 🔖Terminology

    //#region 🔖Runtime
    /// 🖱️ In-flight pointer drag of one board node, tracked by screen delta so no viewport size is needed.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct WiresDragState {
        node_id: String,
        last_x: f64,
        last_y: f64,
    }

    /// 🎛️ Ephemeral view state (selection + in-flight drag) held in the app struct, never in the
    /// document — so it stays out of undo history and off the op channel.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct WiresPlayRuntime {
        selected_ids: Vec<String>,
        drag: Option<WiresDragState>,
    }

    fn default_empty_wires_fixture() -> Value {
        json!({
            "schema": WIRES_FIXTURE_SCHEMA,
            "identities": [],
            "relationships": [],
            "board": default_empty_board_fixture()
        })
    }

    fn default_empty_board_fixture() -> Value {
        json!({
            "schema": PUZZLE2D_FIXTURE_SCHEMA,
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [],
            "edges": [],
            "wires": []
        })
    }

    fn wires_action(action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor {
            controller_id: WIRES_PLAY_CONTROLLER_ID.into(),
            action: action.into(),
            args,
        }
    }

    fn selection_ids(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("ids"))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
            .or_else(|| {
                args.and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(|id| vec![id.to_string()])
            })
            .unwrap_or_default()
    }

    fn fixture_camera(fixture: &Value) -> (f64, f64, f64) {
        let camera = fixture.get("camera");
        (
            camera.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0),
            camera.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0),
            camera.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()).unwrap_or(1.0),
        )
    }

    fn fixture_nodes(fixture: &Value) -> &[Value] {
        fixture
            .get("nodes")
            .and_then(|value| value.as_array())
            .map(|values| values.as_slice())
            .unwrap_or(&[])
    }

    fn fixture_edges(fixture: &Value) -> &[Value] {
        fixture
            .get("edges")
            .and_then(|value| value.as_array())
            .map(|values| values.as_slice())
            .unwrap_or(&[])
    }

    fn wires_identities(wires: &Value) -> &[Value] {
        wires
            .get("identities")
            .and_then(|value| value.as_array())
            .map(|values| values.as_slice())
            .unwrap_or(&[])
    }

    fn wires_relationships(wires: &Value) -> &[Value] {
        wires
            .get("relationships")
            .and_then(|value| value.as_array())
            .map(|values| values.as_slice())
            .unwrap_or(&[])
    }

    fn wires_fixture_board(wires: &Value) -> Value {
        let mut board = wires.get("board").cloned().unwrap_or_else(default_empty_board_fixture);
        if let Some(obj) = board.as_object_mut() {
            obj.insert("schema".into(), json!(PUZZLE2D_FIXTURE_SCHEMA));
            if !obj.contains_key("wires") {
                obj.insert("wires".into(), json!([]));
            }
            if let Some(nodes) = obj.get_mut("nodes").and_then(|value| value.as_array_mut()) {
                for node in nodes {
                    if let Some(node_obj) = node.as_object_mut() {
                        if !node_obj.contains_key("handles") {
                            node_obj.insert("handles".to_string(), json!([]));
                        }
                    }
                }
            }
        }
        board
    }

    fn document_from_wires_fixture(wires: Value) -> MindmapWiresDocument {
        MindmapWiresDocument {
            board_fixture: wires_fixture_board(&wires),
            wires_fixture: wires,
        }
    }

    fn identity_label(wires: &Value, identity_id: u64) -> Option<String> {
        wires_identities(wires)
            .iter()
            .find(|identity| identity.get("identityId").and_then(|value| value.as_u64()) == Some(identity_id))
            .and_then(|identity| identity.get("label").and_then(|value| value.as_str()))
            .map(str::to_string)
    }

    fn relationship_kind_display_name<'a>(kind: &'a str, labels: &WiresLabels) -> &'a str {
        match kind {
            "owns" => labels.relationship_kind_owns,
            "is" => labels.relationship_kind_is,
            "references" => labels.relationship_kind_references,
            "has" => labels.relationship_kind_has,
            _ => kind,
        }
    }

    fn wires_relationship_document_label(wires: &Value, edge_id: &str, labels: &WiresLabels) -> Option<String> {
        let relationship = wires_relationships(wires).iter().find(|row| {
            row.get("edgeId").and_then(|value| value.as_str()) == Some(edge_id)
        })?;
        let kind = relationship.get("kind")?.as_str()?;
        let source_id = relationship.get("sourceIdentityId")?.as_u64()?;
        let target_id = relationship.get("targetIdentityId")?.as_u64()?;
        let source = identity_label(wires, source_id)?;
        let target = identity_label(wires, target_id)?;
        Some(format!(
            "{}: {source} → {target}",
            relationship_kind_display_name(kind, labels)
        ))
    }

    fn wires_identity_kind_name(wires: &Value, identity_kind_id: &str) -> Option<String> {
        wires
            .get("kindCatalogs")
            .and_then(|value| value.get("identityKinds"))
            .and_then(|value| value.as_array())
            .into_iter()
            .flatten()
            .chain(
                wires
                    .get("board")
                    .and_then(|value| value.get("meta"))
                    .and_then(|value| value.get("kindCatalogs"))
                    .and_then(|value| value.get("identityKinds"))
                    .and_then(|value| value.as_array())
                    .into_iter()
                    .flatten(),
            )
            .find(|row| row.get("id").and_then(|value| value.as_str()) == Some(identity_kind_id))
            .and_then(|row| row.get("name").and_then(|value| value.as_str()))
            .map(str::to_string)
    }

    fn wires_kind_catalog_entries(wires: &Value, key: &str) -> Vec<Value> {
        wires
            .get("kindCatalogs")
            .and_then(|value| value.get(key))
            .and_then(|value| value.as_array())
            .cloned()
            .or_else(|| {
                wires
                    .get("board")
                    .and_then(|value| value.get("meta"))
                    .and_then(|value| value.get("kindCatalogs"))
                    .and_then(|value| value.get(key))
                    .and_then(|value| value.as_array())
                    .cloned()
            })
            .unwrap_or_default()
    }

    fn document_tree_selected_ids(board: &Value, selected: &[String]) -> Vec<String> {
        selected
            .iter()
            .filter_map(|id| {
                if fixture_nodes(board)
                    .iter()
                    .any(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
                {
                    return Some(format!("{WIRES_DOCUMENT_IDENTITY_PREFIX}{id}"));
                }
                if fixture_edges(board)
                    .iter()
                    .any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
                {
                    return Some(format!("{WIRES_DOCUMENT_RELATIONSHIP_PREFIX}{id}"));
                }
                None
            })
            .collect()
    }
    //#endregion 🔖Runtime

    //#region 🔖Canvas
    fn relationship_edge_layers(wires: &Value, board: &Value) -> Vec<Value> {
        let mut layers = Vec::new();
        for relationship in wires_relationships(wires) {
            let edge_id = relationship.get("edgeId").and_then(|value| value.as_str()).unwrap_or("");
            if edge_id.is_empty() {
                continue;
            }
            let edge = fixture_edges(board).iter().find(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(edge_id));
            if let Some(edge) = edge {
                layers.push(edge.clone());
            } else {
                layers.push(json!({
                    "id": edge_id,
                    "kind": "edge",
                    "edgeKind": relationship.get("kind").cloned().unwrap_or_else(|| json!("relationship")),
                    "source": relationship.get("sourceIdentityId").map(|value| value.to_string()).unwrap_or_default(),
                    "target": relationship.get("targetIdentityId").map(|value| value.to_string()).unwrap_or_default(),
                }));
            }
        }
        layers
    }

    /// 🕸️ Re-lays out the board with `puzzle_2d`'s force-graph solver, same as `puzzle/2d`'s `forceLayout`/`reorganize`.
    fn force_layout_board(board: &mut Value) {
        let Ok(layout_json) = puzzle_2d::apply_force_graph_layout_to_fixture_v1_json(&board.to_string(), r#"{"mode":"force-graph"}"#) else {
            return;
        };
        if let Ok(parsed) = serde_json::from_str(&layout_json) {
            *board = parsed;
        }
    }

    fn render_canvas(board: &Value, wires: &Value) -> UiNode {
        let (camera_x, camera_y, zoom) = fixture_camera(board);
        let mut layers: Vec<Value> = fixture_nodes(board).iter().cloned().collect();
        layers.extend(fixture_edges(board).iter().cloned());
        layers.extend(relationship_edge_layers(wires, board));
        build_canvas_2d_scene(
            WIRES_PLAY_SURFACE_ID,
            WIRES_PLAY_CONTROLLER_ID,
            Canvas2dScene {
                camera_x,
                camera_y,
                zoom,
                layers_json: serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into()),
            },
        )
    }
    //#endregion 🔖Canvas

    //#region 🔖DocumentPanel
    fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
        UiTreeItemNode {
            id: id.into(),
            label: label.into(),
            description,
            icon_id: None,
            selected: None,
            default_open: None,
            action: Some(action),
            hover_action: None,
            unhover_action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        }
    }

    fn render_document_panel(document: &MindmapWiresDocument, selected: &[String], labels: &WiresLabels) -> UiNode {
        let wires = &document.wires_fixture;
        let board = &document.board_fixture;
        let identity_items: Vec<UiTreeItemNode> = wires_identities(wires)
            .iter()
            .filter_map(|identity| {
                let node_id = identity.get("nodeId")?.as_str()?;
                let label = identity.get("label")?.as_str()?;
                let identity_kind = identity.get("identityKind").and_then(|value| value.as_str());
                let description = identity_kind
                    .and_then(|kind| wires_identity_kind_name(wires, kind))
                    .filter(|kind_name| kind_name != label);
                Some(tree_item_with_action(
                    format!("{WIRES_DOCUMENT_IDENTITY_PREFIX}{node_id}"),
                    label,
                    description,
                    wires_action("setSelection", Some(json!({ "ids": [node_id] }))),
                ))
            })
            .collect();
        let relationship_items: Vec<UiTreeItemNode> = fixture_edges(board)
            .iter()
            .filter_map(|edge| {
                let edge_id = edge.get("id")?.as_str()?;
                Some(tree_item_with_action(
                    format!("{WIRES_DOCUMENT_RELATIONSHIP_PREFIX}{edge_id}"),
                    wires_relationship_document_label(wires, edge_id, labels).unwrap_or_else(|| edge_id.into()),
                    None,
                    wires_action("setSelection", Some(json!({ "ids": [edge_id] }))),
                ))
            })
            .collect();
        UiNode::Tree(UiTreeNode {
            sections: vec![
                UiTreeSectionNode {
                    id: "wires-play-document.identities".into(),
                    label: Some(labels.identities.into()),
                    default_open: Some(true),
                    items: if identity_items.is_empty() {
                        vec![UiTreeItemNode {
                            id: "wires-play-document.identities.empty".into(),
                            label: "(none)".into(),
                            description: None,
                            icon_id: None,
                            selected: None,
                            default_open: None,
                            action: None,
                            hover_action: None,
                            unhover_action: None,
                            actions: None,
                            draggable: None,
                            drag_data: None,
                            items: None,
                            control: None,
                            is_hidden: None,
                        }]
                    } else {
                        identity_items
                    },
                },
                UiTreeSectionNode {
                    id: "wires-play-document.relationships".into(),
                    label: Some(labels.relationships.into()),
                    default_open: Some(false),
                    items: if relationship_items.is_empty() {
                        vec![UiTreeItemNode {
                            id: "wires-play-document.relationships.empty".into(),
                            label: "(none)".into(),
                            description: None,
                            icon_id: None,
                            selected: None,
                            default_open: None,
                            action: None,
            hover_action: None,
            unhover_action: None,
            actions: None,
                            draggable: None,
                            drag_data: None,
                            items: None,
                            control: None,
                            is_hidden: None,
                        }]
                    } else {
                        relationship_items
                    },
                },
            ],
            selected_ids: Some(document_tree_selected_ids(board, selected)),
            highlighted_ids: None,
            selection_change: Some(wires_action("setSelection", None)),
            drop_action: None,
        })
    }
    //#endregion 🔖DocumentPanel

    //#region 🔖CataloguePanel
    fn catalog_kind_label(entry: &Value) -> String {
        entry
            .get("name")
            .and_then(|value| value.as_str())
            .filter(|value| !value.is_empty())
            .or_else(|| entry.get("id").and_then(|value| value.as_str()))
            .unwrap_or("kind")
            .into()
    }

    fn kind_catalog_section(section_id: &str, label: &str, entries: &[Value]) -> UiTreeSectionNode {
        let items: Vec<UiTreeItemNode> = entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
                let action = match section_id {
                    "wires-play-kinds.identity-kinds" => wires_action("addNode", Some(json!({ "kind": kind_id }))),
                    "wires-play-kinds.relationship-kinds" => {
                        wires_action("addRelationship", Some(json!({ "kind": kind_id })))
                    }
                    _ => wires_action("addNode", Some(json!({ "kind": kind_id }))),
                };
                UiTreeItemNode {
                    id: format!("{section_id}.{index}.{kind_id}"),
                    label: catalog_kind_label(entry),
                    description: Some(kind_id.into()),
                    icon_id: None,
                    selected: None,
                    default_open: None,
                    action: Some(action),
                    hover_action: None,
                    unhover_action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: None,
                }
            })
            .collect();
        UiTreeSectionNode {
            id: section_id.into(),
            label: Some(label.into()),
            default_open: Some(true),
            items: if items.is_empty() {
                vec![UiTreeItemNode {
                    id: format!("{section_id}.empty"),
                    label: "(none)".into(),
                    description: None,
                    icon_id: None,
                    selected: None,
                    default_open: None,
                    action: None,
            hover_action: None,
            unhover_action: None,
            actions: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: None,
                }]
            } else {
                items
            },
        }
    }

    fn render_catalogue_panel(wires: &Value, labels: &WiresLabels) -> UiNode {
        let identity_entries = wires_kind_catalog_entries(wires, "identityKinds");
        let relationship_entries = wires_kind_catalog_entries(wires, "relationshipKinds");
        UiNode::Tree(UiTreeNode {
            sections: vec![
                kind_catalog_section("wires-play-kinds.identity-kinds", labels.identity_kinds, &identity_entries),
                kind_catalog_section("wires-play-kinds.relationship-kinds", labels.relationship_kinds, &relationship_entries),
            ],
            selected_ids: None,
            highlighted_ids: None,
            selection_change: None,
            drop_action: None,
        })
    }
    //#endregion 🔖CataloguePanel

    //#region 🔖InspectorPanel
    fn render_properties_panel(document: &MindmapWiresDocument, selected: &[String]) -> UiNode {
        let selected_nodes: Vec<&Value> = selected
            .iter()
            .filter_map(|id| {
                fixture_nodes(&document.board_fixture)
                    .iter()
                    .find(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
            })
            .collect();
        if selected_nodes.is_empty() {
            let extension = DefaultWiresExtension::from_fixture_json(&document.wires_fixture.to_string()).ok();
            return ui_stack_vertical(vec![
                ui_text(format!("Schema: {WIRES_FIXTURE_SCHEMA}")),
                ui_text(format!(
                    "Identities: {}",
                    extension.as_ref().map(|ext| ext.mindmap.topics.len()).unwrap_or(0)
                )),
                ui_text(format!(
                    "Relationships: {}",
                    extension.as_ref().map(|ext| ext.relationships.len()).unwrap_or(0)
                )),
                ui_text(format!("Board nodes: {}", fixture_nodes(&document.board_fixture).len())),
            ]);
        }
        let node = selected_nodes[0];
        let identity = wires_identities(&document.wires_fixture)
            .iter()
            .find(|identity| identity.get("nodeId").and_then(|value| value.as_str()) == node.get("id").and_then(|value| value.as_str()));
        ui_stack_vertical(vec![
            ui_inspector_readonly_field(
                "wires-play-inspector.id",
                "Id",
                node.get("id")
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .to_string(),
            ),
            ui_inspector_readonly_field(
                "wires-play-inspector.identity-label",
                "Identity",
                identity
                    .and_then(|row| row.get("label"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("—")
                    .to_string(),
            ),
            ui_inspector_readonly_field(
                "wires-play-inspector.node-kind",
                "Identity Kind",
                node.get("nodeKind")
                    .and_then(|value| value.as_str())
                    .unwrap_or("—")
                    .to_string(),
            ),
            ui_inspector_readonly_field(
                "wires-play-inspector.x",
                "X",
                node.get("x")
                    .and_then(|value| value.as_f64())
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "—".into()),
            ),
            ui_inspector_readonly_field(
                "wires-play-inspector.y",
                "Y",
                node.get("y")
                    .and_then(|value| value.as_f64())
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "—".into()),
            ),
        ])
    }
    //#endregion 🔖InspectorPanel

    //#region 🔖WiresPlayApp
    #[derive(Default)]
    pub struct WiresPlayApp {
        runtime: WiresPlayRuntime,
    }

    /// 📐 A JSON node's position, defaulting missing coordinates to the origin.
    fn node_position(node: &Value) -> (f64, f64) {
        (
            node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0),
            node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0),
        )
    }

    impl DocumentApp for WiresPlayApp {
        type Projection = MindmapWiresDocument;
        type Op = MindmapWiresOp;

        fn app_id(&self) -> &str {
            WIRES_PLAY_APP_ID
        }

        fn document_schema(&self) -> &str {
            WIRES_FIXTURE_SCHEMA
        }

        fn initial_projection(&self) -> MindmapWiresDocument {
            empty_mindmap_wires_document()
        }

        fn handle_action(
            &mut self,
            action: &str,
            args: Option<&Value>,
            doc: &DocumentView<'_, MindmapWiresDocument>,
            _view_state: &ViewState,
        ) -> ActionEmit<MindmapWiresOp> {
            let document = doc.projection;
            match action {
                // 👁️ View actions — mutate ephemeral runtime, emit no ops.
                "setSelection" | "documentSelect" => {
                    self.runtime.selected_ids = selection_ids(args);
                    ActionEmit::default()
                }
                "canvasPointerDown" => {
                    let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    if let Some(id) = id.filter(|id| find_board_node(document, id).is_some()) {
                        self.runtime.selected_ids = vec![id.to_string()];
                        self.runtime.drag = Some(WiresDragState { node_id: id.to_string(), last_x: x, last_y: y });
                    }
                    ActionEmit::default()
                }
                "canvasPointerUp" => {
                    self.runtime.drag = None;
                    ActionEmit::default()
                }
                // ✏️ Operations — dispatched as VCS operations with a true inverse.
                "setActiveExample" => {
                    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                    let next = if example_id == WIRES_PLAY_EXAMPLE_METABOLISM_ID {
                        document_from_wires_fixture(
                            serde_json::from_str(METABOLISM_WIRES_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_wires_fixture()),
                        )
                    } else {
                        empty_mindmap_wires_document()
                    };
                    self.runtime.selected_ids.clear();
                    self.runtime.drag = None;
                    ActionEmit::ops(vec![MindmapWiresOp::ReplaceDocument {
                        wires_fixture: next.wires_fixture,
                        board_fixture: next.board_fixture,
                    }])
                }
                "addNode" => {
                    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("identity");
                    let id = format!("node-{}", fixture_nodes(&document.board_fixture).len() + 1);
                    let node = json!({
                        "id": id,
                        "nodeKind": kind,
                        "shape": "circle",
                        "x": 0.0,
                        "y": 0.0,
                        "radius": 24.0,
                        "text": id,
                        "handles": []
                    });
                    self.runtime.selected_ids = vec![id];
                    ActionEmit::ops(vec![MindmapWiresOp::AddNode { node }])
                }
                "addRelationship" => {
                    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("owns");
                    let edge_id = format!("edge-{}", fixture_edges(&document.board_fixture).len() + 1);
                    let edge = json!({
                        "id": edge_id,
                        "edgeKind": format!("wires.{kind}"),
                        "source": "node-1",
                        "target": "node-2"
                    });
                    let relationship = json!({
                        "edgeId": edge_id,
                        "kind": kind,
                        "sourceIdentityId": 1,
                        "targetIdentityId": 2
                    });
                    self.runtime.selected_ids = vec![edge_id];
                    ActionEmit::ops(vec![MindmapWiresOp::AddRelationship { edge, relationship }])
                }
                "deleteSelection" => {
                    let mut ops = Vec::new();
                    for id in &self.runtime.selected_ids {
                        if find_board_node(document, id).is_some() {
                            ops.push(MindmapWiresOp::RemoveNode { node_id: id.clone() });
                        } else if fixture_edges(&document.board_fixture)
                            .iter()
                            .any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
                        {
                            ops.push(MindmapWiresOp::RemoveEdge { edge_id: id.clone() });
                        }
                    }
                    if !ops.is_empty() {
                        self.runtime.selected_ids.clear();
                    }
                    ActionEmit::ops(ops)
                }
                "forceLayout" | "reorganize" => {
                    let mut board = document.board_fixture.clone();
                    force_layout_board(&mut board);
                    let ops: Vec<MindmapWiresOp> = fixture_nodes(&board)
                        .iter()
                        .filter_map(|node| {
                            let id = node.get("id").and_then(|value| value.as_str())?;
                            let (nx, ny) = node_position(node);
                            let (ox, oy) = find_board_node(document, id).map(node_position).unwrap_or((nx, ny));
                            if nx == ox && ny == oy {
                                return None;
                            }
                            let mut patch = Map::new();
                            patch.insert("x".into(), json!(nx));
                            patch.insert("y".into(), json!(ny));
                            Some(MindmapWiresOp::PatchNode { node_id: id.to_string(), patch })
                        })
                        .collect();
                    ActionEmit::ops(ops)
                }
                "canvasPointerMove" => {
                    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let Some(drag) = self.runtime.drag.clone() else { return ActionEmit::default() };
                    let Some(node) = find_board_node(document, &drag.node_id) else { return ActionEmit::default() };
                    let zoom = fixture_camera(&document.board_fixture).2.max(1e-6);
                    let (cur_x, cur_y) = node_position(node);
                    let (dx, dy) = ((x - drag.last_x) / zoom, (y - drag.last_y) / zoom);
                    let mut patch = Map::new();
                    patch.insert("x".into(), json!(cur_x + dx));
                    patch.insert("y".into(), json!(cur_y + dy));
                    self.runtime.drag = Some(WiresDragState { node_id: drag.node_id.clone(), last_x: x, last_y: y });
                    ActionEmit::amend(
                        vec![MindmapWiresOp::PatchNode { node_id: drag.node_id.clone(), patch }],
                        format!("drag:{}", drag.node_id),
                    )
                }
                _ => ActionEmit::default(),
            }
        }

        fn render(&self, body_key: &str, doc: &DocumentView<'_, MindmapWiresDocument>, view_state: &ViewState) -> UiNode {
            let document = doc.projection;
            let labels = wires_labels(view_state);
            match body_key {
                WIRES_PLAY_BODY_COMPOSITE => render_canvas(&document.board_fixture, &document.wires_fixture),
                WIRES_PLAY_BODY_DOCUMENT => render_document_panel(document, &self.runtime.selected_ids, labels),
                WIRES_PLAY_BODY_CATALOGUE => render_catalogue_panel(&document.wires_fixture, labels),
                WIRES_PLAY_BODY_PROPERTIES => render_properties_panel(document, &self.runtime.selected_ids),
                _ => ui_text(format!("Unknown body: {body_key}")),
            }
        }

        fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
            let labels = wires_labels(view_state);
            let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
            let mut overlay = semio_framework_plugin::AppLabelsOverlay::with_framework_panel_tabs(
                ["framework.panel.document", "framework.panel.catalogue", "framework.panel.inspection"],
                is_de,
            );
            overlay.window_kind_labels = std::collections::HashMap::from([("reasoning-wires-composite".to_string(), labels.window_main.to_string())]);
            overlay.mode_labels = std::collections::HashMap::from([("edit".to_string(), labels.mode_edit.to_string())]);
            overlay.action_labels = wires_action_labels(is_de);
            overlay.utility_labels = std::collections::HashMap::new();
            overlay.example_labels = std::collections::HashMap::from([
                ("empty".to_string(), (if is_de { "Leer" } else { "Empty" }).to_string()),
                (WIRES_PLAY_EXAMPLE_METABOLISM_ID.to_string(), "Metabolism".to_string()),
            ]);
            overlay
        }
    }
    //#endregion 🔖WiresPlayApp

    //#region 🔖CommandLabels
    /// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_wires_app`'s
    /// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
    /// palette and Actions rail get a translated label without threading locale through the whole builder chain.
    fn wires_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
        const ENTRIES: &[(&str, &str, &str)] = &[
            ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
            ("addNode", "Add Node", "Knoten hinzufuegen"),
            ("addRelationship", "Add Relationship", "Beziehung hinzufuegen"),
            ("deleteSelection", "Delete Selection", "Auswahl loeschen"),
            ("forceLayout", "Force Layout", "Kraftbasiertes Layout"),
            ("reorganize", "Reorganize", "Neu anordnen"),
            ("canvasPointerMove", "Canvas Pointer Move", "Leinwand-Zeiger bewegt"),
            ("setSelection", "Set Selection", "Auswahl festlegen"),
            ("documentSelect", "Document Select", "Dokument auswaehlen"),
            ("canvasPointerDown", "Canvas Pointer Down", "Leinwand-Zeiger gedrueckt"),
            ("canvasPointerUp", "Canvas Pointer Up", "Leinwand-Zeiger losgelassen"),
        ];
        ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
    }
    //#endregion 🔖CommandLabels

    //#region 🔖AppFactory
    pub fn create_wires_app() -> App {
        App::from_builder(
            App::builder(WIRES_PLAY_APP_ID, "Mindmap Wires").document(["semio", "reasoning", "mindmap", "wires"])
                .icon_id("reasoning-wires")
                .mode("edit", "Edit")
                .default_mode_id("edit")
                .window_kind("reasoning-wires-composite", "Canvas", WIRES_PLAY_BODY_COMPOSITE, SurfaceKind::Canvas2d)
                .panel_tab("framework.panel.document", FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, WIRES_PLAY_BODY_DOCUMENT)
                .panel_tab("framework.panel.catalogue", FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, WIRES_PLAY_BODY_CATALOGUE)
                .panel_tab("framework.panel.inspection", FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, WIRES_PLAY_BODY_PROPERTIES)
                .default_layout(create_default_layout(
                    &["reasoning-wires-composite".into()],
                    "row",
                    Some(&[100.0]),
                    Some(&["Canvas".into()]),
                ))
                // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
                .operation("setActiveExample", "Set Active Example")
                .operation("addNode", "Add Node")
                .operation("addRelationship", "Add Relationship")
                .operation("deleteSelection", "Delete Selection")
                .operation("forceLayout", "Force Layout")
                .operation("reorganize", "Reorganize")
                .operation("canvasPointerMove", "Canvas Pointer Move")
                // 👁️ Ephemeral view state — selection and in-flight drag.
                .view_action("setSelection", "Set Selection")
                .view_action("documentSelect", "Document Select")
                .view_action("canvasPointerDown", "Canvas Pointer Down")
                .view_action("canvasPointerUp", "Canvas Pointer Up"),
        )
        .example("empty", "Empty", serde_json::to_string(&empty_mindmap_wires_document()).unwrap())
        .example(
            WIRES_PLAY_EXAMPLE_METABOLISM_ID,
            "Metabolism",
            serde_json::to_string(&document_from_wires_fixture(
                serde_json::from_str(METABOLISM_WIRES_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_wires_fixture()),
            ))
            .unwrap(),
        )
        .program("reasoning-wires", "Mindmap Wires", "graph")
    }
    //#endregion 🔖AppFactory

    //#region 🧪Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};
        use vcs::{Backbone, BackboneMessage, MemoryBackbone};

        fn meta(actor: &str) -> ActionMeta {
            ActionMeta { actor: actor.into(), instance_id: 1 }
        }

        fn new_app() -> VcsDocumentApp<WiresPlayApp> {
            VcsDocumentApp::new(WiresPlayApp::default())
        }

        fn metabolism_app() -> VcsDocumentApp<WiresPlayApp> {
            let mut app = new_app();
            let document =
                document_from_wires_fixture(serde_json::from_str(METABOLISM_WIRES_EXAMPLE_JSON).expect("metabolism fixture"));
            app.load_document(
                &serde_json::to_string(&vcs::create_document_vcs_envelope::<MindmapWiresDocument, MindmapWiresOp>(
                    WIRES_FIXTURE_SCHEMA,
                    "reasoning-wires",
                    document,
                    None,
                ))
                .unwrap(),
            )
            .expect("load metabolism");
            app
        }

        #[test]
        fn renders_canvas_scene() {
            let mut app = new_app();
            let node = app.render(WIRES_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
            assert!(serde_json::to_string(&node).unwrap().contains("canvas-2d"));
        }

        #[test]
        fn wires_labels_resolve_native_by_default() {
            let mut app = metabolism_app();
            let json = serde_json::to_string(&app.render(WIRES_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
            assert!(json.contains("Identities"));
            assert!(json.contains("Relationships"));
            let catalogue_json = serde_json::to_string(&app.render(WIRES_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render")).unwrap();
            assert!(catalogue_json.contains("Identity kinds"));
            assert!(catalogue_json.contains("Relationship kinds"));
        }

        #[test]
        fn wires_labels_resolve_native_in_german() {
            let mut app = metabolism_app();
            let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
            let json = serde_json::to_string(&app.render(WIRES_PLAY_BODY_DOCUMENT, None, &view_state).expect("render")).unwrap();
            assert!(json.contains("Identitäten"));
            assert!(json.contains("Beziehungen"));
            let catalogue_json = serde_json::to_string(&app.render(WIRES_PLAY_BODY_CATALOGUE, None, &view_state).expect("render")).unwrap();
            assert!(catalogue_json.contains("Identitätsarten"));
            assert!(catalogue_json.contains("Beziehungsarten"));
        }

        #[test]
        fn document_has_identities_section() {
            let mut app = metabolism_app();
            let json = serde_json::to_string(&app.render(WIRES_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
            assert!(json.contains("wires-play-document.identities"));
            assert!(json.contains("Metabolism"));
        }

        #[test]
        fn metabolism_fixture_hydrates_extension() {
            let ext = DefaultWiresExtension::from_fixture_json(METABOLISM_WIRES_EXAMPLE_JSON).expect("metabolism fixture");
            assert_eq!(ext.mindmap.topics.len(), 7);
            assert_eq!(ext.relationships.len(), 9);
        }

        #[test]
        fn relationship_kind_labels_match_fixture() {
            assert_eq!(RelationshipKind::Owns.label(), "owns");
            assert_eq!(relationship_kind_display_name("is", &WIRES_LABELS_NATIVE_EN), "Is");
        }

        #[test]
        fn add_node_appends_and_selects() {
            let mut app = new_app();
            app.handle_action("addNode", Some(&json!({ "kind": "identity" })), &ViewState::default(), &meta("local")).expect("add");
            let projection = app.projection().expect("projection");
            assert_eq!(fixture_nodes(&projection.board_fixture).len(), 1);
            assert!(find_board_node(&projection, "node-1").is_some());
        }

        #[test]
        fn pointer_drag_translates_node_by_screen_delta() {
            let mut app = new_app();
            app.handle_action("addNode", Some(&json!({ "kind": "identity" })), &ViewState::default(), &meta("local")).expect("add");
            app.handle_action("canvasPointerDown", Some(&json!({ "id": "node-1", "x": 100.0, "y": 100.0 })), &ViewState::default(), &meta("local")).expect("down");
            app.handle_action("canvasPointerMove", Some(&json!({ "x": 140.0, "y": 130.0 })), &ViewState::default(), &meta("local")).expect("move");
            let node = find_board_node(&app.projection().expect("projection"), "node-1").expect("node-1").clone();
            assert_eq!(node.get("x").and_then(|value| value.as_f64()), Some(40.0));
            assert_eq!(node.get("y").and_then(|value| value.as_f64()), Some(30.0));
            app.handle_action("canvasPointerUp", Some(&json!({ "x": 140.0, "y": 130.0 })), &ViewState::default(), &meta("local")).expect("up");
            // A coalesced drag collapses to a single undo step restoring the origin.
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            let node = find_board_node(&app.projection().expect("projection"), "node-1").expect("node-1").clone();
            assert_eq!(node.get("x").and_then(|value| value.as_f64()), Some(0.0));
        }

        #[test]
        fn delete_selection_removes_node() {
            let mut app = new_app();
            app.handle_action("addNode", Some(&json!({ "kind": "identity" })), &ViewState::default(), &meta("local")).expect("add");
            app.handle_action("setSelection", Some(&json!({ "ids": ["node-1"] })), &ViewState::default(), &meta("local")).expect("select");
            app.handle_action("deleteSelection", None, &ViewState::default(), &meta("local")).expect("delete");
            assert!(fixture_nodes(&app.projection().expect("projection").board_fixture).is_empty());
        }

        #[test]
        fn force_layout_action_repositions_metabolism_nodes() {
            let mut app = metabolism_app();
            let before: Vec<(f64, f64)> = fixture_nodes(&app.projection().expect("projection").board_fixture)
                .iter()
                .map(node_position)
                .collect();
            app.handle_action("forceLayout", None, &ViewState::default(), &meta("local")).expect("force layout");
            let after: Vec<(f64, f64)> =
                fixture_nodes(&app.projection().expect("projection").board_fixture).iter().map(node_position).collect();
            assert_eq!(before.len(), after.len());
            assert_ne!(before, after, "force layout should move at least one node");
        }

        #[test]
        fn wires_fixture_board_uses_puzzle_schema() {
            let _extension = Puzzle2dExtension;
            let wires: Value = serde_json::from_str(METABOLISM_WIRES_EXAMPLE_JSON).unwrap();
            let board = wires_fixture_board(&wires);
            assert_eq!(board.get("schema").and_then(|value| value.as_str()), Some(PUZZLE2D_FIXTURE_SCHEMA));
            assert_eq!(fixture_nodes(&board).len(), 7);
        }

        #[test]
        fn undo_redo_round_trip_through_the_wrapper() {
            let mut app = new_app();
            app.handle_action("addNode", Some(&json!({ "kind": "identity" })), &ViewState::default(), &meta("local")).expect("add");
            assert_eq!(fixture_nodes(&app.projection().expect("projection").board_fixture).len(), 1);
            app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
            assert!(fixture_nodes(&app.projection().expect("projection").board_fixture).is_empty());
            app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
            assert_eq!(fixture_nodes(&app.projection().expect("projection").board_fixture).len(), 1);
        }

        /// 🧪 The definitional merge proof: A adds a node while B renames another node — disjoint edits
        /// on one backbone that must both survive on both instances (impossible under whole-document LWW).
        #[test]
        fn two_instances_converge_disjoint_graph_edits_via_backbone() {
            let mut instance_a = new_app();
            let mut instance_b = new_app();
            // Seed both from an identical base projection carrying node-1/node-2 (as initial state, not
            // as edits) so the only edits on the channel are A's and B's disjoint ones.
            let seed_node = |id: &str| json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": id, "handles": [] });
            let mut base = empty_mindmap_wires_document();
            base = vcs::apply_operation(&base, &MindmapWiresOp::AddNode { node: seed_node("node-1") });
            base = vcs::apply_operation(&base, &MindmapWiresOp::AddNode { node: seed_node("node-2") });
            let base_envelope = serde_json::to_string(&vcs::create_document_vcs_envelope::<MindmapWiresDocument, MindmapWiresOp>(
                WIRES_FIXTURE_SCHEMA,
                "reasoning-wires",
                base,
                None,
            ))
            .unwrap();
            instance_a.load_document(&base_envelope).expect("load a");
            instance_b.load_document(&base_envelope).expect("load b");
            let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://mindmap-convergence", "mem://mindmap-convergence");
            instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
            instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

            // A adds node-3 (a new node); B moves node-2 (a PatchNode) — disjoint edits on the graph.
            instance_a.handle_action("addNode", Some(&json!({ "kind": "identity" })), &ViewState::default(), &meta("actor-a")).expect("a adds node");
            instance_b.handle_action("canvasPointerDown", Some(&json!({ "id": "node-2", "x": 0.0, "y": 0.0 })), &ViewState::default(), &meta("actor-b")).expect("b down");
            instance_b.handle_action("canvasPointerMove", Some(&json!({ "x": 50.0, "y": 60.0 })), &ViewState::default(), &meta("actor-b")).expect("b move");
            instance_b.handle_action("canvasPointerUp", Some(&json!({ "x": 50.0, "y": 60.0 })), &ViewState::default(), &meta("actor-b")).expect("b up");

            instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
            instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

            let projection_a = instance_a.projection().expect("projection a");
            let projection_b = instance_b.projection().expect("projection b");
            // A's added node-3 survives on both.
            assert!(find_board_node(&projection_a, "node-3").is_some(), "A keeps its own node");
            assert!(find_board_node(&projection_b, "node-3").is_some(), "B converges on A's node");
            // B's move of node-2 survives on both.
            let x_of = |document: &MindmapWiresDocument| find_board_node(document, "node-2").map(node_position).unwrap().0;
            assert_eq!(x_of(&projection_a), 50.0, "A converges on B's move");
            assert_eq!(x_of(&projection_b), 50.0, "B keeps its own move");
        }

        #[test]
        fn ingest_operations_is_idempotent() {
            let mut sender = new_app();
            let (near, mut far) = MemoryBackbone::pair("mem://mindmap-doc", "mem://mindmap-doc");
            sender.attach_backbone(Box::new(near)).expect("attach");
            sender.handle_action("addNode", Some(&json!({ "kind": "identity" })), &ViewState::default(), &meta("local")).expect("add");
            let mut envelopes = Vec::new();
            for message in far.receive().expect("receive") {
                if let BackboneMessage::Ops { envelopes: ops } = message {
                    envelopes.extend(ops);
                }
            }
            assert!(!envelopes.is_empty(), "expected the applied op on the channel");
            let operations_json = serde_json::to_string(&envelopes).expect("serialize");
            let mut receiver = new_app();
            receiver.ingest_operations(&operations_json).expect("ingest once");
            receiver.ingest_operations(&operations_json).expect("ingest twice");
            assert_eq!(fixture_nodes(&receiver.projection().expect("projection").board_fixture).len(), 1, "no double-apply");
        }
    }
    //#endregion 🧪Tests
}

//#region 🔖Bundle
fn register_reasoning_mindmap_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "reasoning-mindmap", label: "Mindmap", version: "0.1.0",
    setup: register_reasoning_mindmap_exports,
    apps: [ app_wires::create_wires_app => app_wires::WiresPlayApp ],
}
//#endregion 🔖Bundle
