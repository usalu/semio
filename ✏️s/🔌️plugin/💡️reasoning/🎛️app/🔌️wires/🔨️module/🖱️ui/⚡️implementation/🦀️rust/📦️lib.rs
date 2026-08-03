//! 🗺️ Reasoning wires app — `DocumentApp` impl, render, manifest (constitutional: ui). B1: the
//! pure-trait conversion — `ReasoningWiresPlayApp` is a unit struct; every former
//! `WiresPlayRuntime` field (selection, in-flight drag) now lives in `reasoning_wires_engine::WiresConfig`,
//! written via `reasoning_wires_op::WiresConfigOperation`s (real `backwards`, no ad hoc runtime
//! `RefCell`); every action dispatches through the single typed
//! `reasoning_wires_protocol::WiresCommand` channel via `DocumentApp::handle`.

use reasoning_wires::MindmapWiresDocument;
use reasoning_wires_engine::{empty_mindmap_wires_document, find_board_node, metabolism_wires_example_document, DefaultWiresExtension, WiresConfig};
use reasoning_wires_op::{MindmapWiresOperation, WiresConfigOperation};
use reasoning_wires_protocol::WiresCommand;
use semio_framework_plugin::{
    app_labels, build_canvas_2d_scene, create_default_layout, localized_label_map,
    tree_item_with_action, ui_inspector_readonly_field, ui_stack_vertical, ui_text, Emit, ActionDescriptor, App,
    AppLabelsOverlay, AppLabelsOverlayExt, Canvas2dScene, ConfigView, DocumentApp, DocumentView, LocaleLabels, MediaClass, MediaForm, MediaType, OsMediaCapability,
    PanelGroup, PanelTreeBuilder, ArtifactKindSpec, SurfaceKind, UiNode, UiTreeItemNode,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::{json, Value};
use dsl::{from_dsl_value, to_dsl_value, DslValue};
use std::collections::BTreeMap;

//#region 🔖️Constants
const WIRES_PLAY_APP_ID: &str = "reasoning-wires-play";
const WIRES_PLAY_CONTROLLER_ID: &str = "reasoning-wires-play";
const WIRES_PLAY_SURFACE_ID: &str = "reasoning.wires.composite";
const WIRES_PLAY_BODY_COMPOSITE: &str = "reasoning.wires.composite";
const WIRES_PLAY_BODY_DOCUMENT: &str = "reasoning.wires.document";
const WIRES_PLAY_BODY_CATALOGUE: &str = "reasoning.wires.catalogue";
const WIRES_PLAY_BODY_PROPERTIES: &str = "reasoning.wires.properties";
/// 🏷️ Alias onto the canonical schema constant now that the entity crate is directly reachable —
/// keeps every call site in this file unchanged while dropping the pre-split duplicate literal.
const WIRES_FIXTURE_SCHEMA: &str = reasoning_wires::MINDMAP_WIRES_SCHEMA;
const WIRES_PLAY_EXAMPLE_METABOLISM_ID: &str = "metabolism";

const WIRES_PLAY_DOCUMENT_NAMESPACE: &str = "wires-play-document";
const WIRES_DOCUMENT_IDENTITY_PREFIX: &str = "wires-play-document.identity.";
const WIRES_DOCUMENT_RELATIONSHIP_PREFIX: &str = "wires-play-document.relationship.";
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — mirrors `shooting_ui`'s identical helpers.
fn is_de_locale(cfg: &WiresConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &WiresConfig) -> &'static L {
    if is_de_locale(cfg) { L::locale_labels_de() } else { L::locale_labels_en() }
}
//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn wires_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: WIRES_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args: semio_framework_plugin::optional_json_to_dsl(args),
    }
}

fn dsl_to_json(value: &DslValue) -> Value {
    from_dsl_value(value.clone()).unwrap_or(Value::Null)
}

fn fixture_json_string(fixture: &DslValue) -> String {
    serde_json::to_string(&dsl_to_json(fixture)).unwrap_or_else(|_| "{}".into())
}

fn fixture_camera(fixture: &DslValue) -> (f64, f64, f64) {
    let camera = fixture.get("camera");
    (
        camera.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()).unwrap_or(1.0),
    )
}

fn fixture_nodes(fixture: &DslValue) -> &[DslValue] {
    fixture
        .get("nodes")
        .and_then(|value| value.as_array())
        .unwrap_or(&[])
}

fn fixture_edges(fixture: &DslValue) -> &[DslValue] {
    fixture
        .get("edges")
        .and_then(|value| value.as_array())
        .unwrap_or(&[])
}

fn wires_identities(wires: &DslValue) -> &[DslValue] {
    wires
        .get("identities")
        .and_then(|value| value.as_array())
        .unwrap_or(&[])
}

fn wires_relationships(wires: &DslValue) -> &[DslValue] {
    wires
        .get("relationships")
        .and_then(|value| value.as_array())
        .unwrap_or(&[])
}

/// 🔢️ `identityId`/`sourceIdentityId`/`targetIdentityId` read as a whole `u64` regardless of whether
/// the source JSON number is an integer or a float literal. `MindmapWiresDocument`'s `wires_fixture`
/// is opaque `serde_json::Value` at rest, but the `.wires` DSL's own `IdentityDsl`/`RelationshipDsl`
/// type these fields as plain `u64` (see `reasoning_wires`'s `🔖️DslMirror` region), so ids
/// round-tripped through the `.wires` DSL text arrive here as exact JSON integers (`Number(1)`); this
/// fallback stays for documents built or patched outside that DSL path, where nothing enforces the
/// integer representation.
fn dsl_id(value: Option<&DslValue>) -> Option<u64> {
    value.and_then(|value| value.as_f64().map(|float| float as u64))
}

fn identity_label(wires: &DslValue, identity_id: u64) -> Option<String> {
    wires_identities(wires)
        .iter()
        .find(|identity| dsl_id(identity.get("identityId")) == Some(identity_id))
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

fn wires_relationship_document_label(wires: &DslValue, edge_id: &str, labels: &WiresLabels) -> Option<String> {
    let relationship = wires_relationships(wires).iter().find(|row| {
        row.get("edgeId").and_then(|value| value.as_str()) == Some(edge_id)
    })?;
    let kind = relationship.get("kind")?.as_str()?;
    let source_id = dsl_id(relationship.get("sourceIdentityId"))?;
    let target_id = dsl_id(relationship.get("targetIdentityId"))?;
    let source = identity_label(wires, source_id)?;
    let target = identity_label(wires, target_id)?;
    Some(format!(
        "{}: {source} → {target}",
        relationship_kind_display_name(kind, labels)
    ))
}

fn wires_identity_kind_name(wires: &DslValue, identity_kind_id: &str) -> Option<String> {
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

fn wires_kind_catalog_entries(wires: &DslValue, key: &str) -> Vec<DslValue> {
    wires
        .get("kindCatalogs")
        .and_then(|value| value.get(key))
        .and_then(|value| value.as_array())
        .map(|values| values.to_vec())
        .or_else(|| {
            wires
                .get("board")
                .and_then(|value| value.get("meta"))
                .and_then(|value| value.get("kindCatalogs"))
                .and_then(|value| value.get(key))
                .and_then(|value| value.as_array())
                .map(|values| values.to_vec())
        })
        .unwrap_or_default()
}

fn document_tree_selected_ids(board: &DslValue, selected: &[String]) -> Vec<String> {
    let namespace = PanelTreeBuilder::new(WIRES_PLAY_DOCUMENT_NAMESPACE);
    selected
        .iter()
        .filter_map(|id| {
            if fixture_nodes(board)
                .iter()
                .any(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
            {
                return Some(namespace.item_id("identity", id));
            }
            if fixture_edges(board)
                .iter()
                .any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
            {
                return Some(namespace.item_id("relationship", id));
            }
            None
        })
        .collect()
}

fn relationship_edge_layers(wires: &DslValue, board: &DslValue) -> Vec<Value> {
    let mut layers = Vec::new();
    for relationship in wires_relationships(wires) {
        let edge_id = relationship.get("edgeId").and_then(|value| value.as_str()).unwrap_or("");
        if edge_id.is_empty() {
            continue;
        }
        let edge = fixture_edges(board).iter().find(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(edge_id));
        if let Some(edge) = edge {
            layers.push(dsl_to_json(edge));
        } else {
            layers.push(json!({
                "id": edge_id,
                "kind": "edge",
                "edgeKind": relationship.get("kind").map(dsl_to_json).unwrap_or_else(|| json!("relationship")),
                "source": relationship.get("sourceIdentityId").map(|value| value.as_f64().map(|n| n.to_string()).unwrap_or_default()).unwrap_or_default(),
                "target": relationship.get("targetIdentityId").map(|value| value.as_f64().map(|n| n.to_string()).unwrap_or_default()).unwrap_or_default(),
            }));
        }
    }
    layers
}

/// 🕸️ Re-lays out the board with the neutral `infinite_board_port_directed` force-graph solver —
/// the same shared mechanism `puzzle/2d`'s `forceLayout`/`reorganize` uses, depended on directly
/// rather than through puzzle's app program (mindmap's board schema is on its allowlist).
fn force_layout_board(board: &mut DslValue) {
    let Ok(layout_json) = infinite_board_port_directed::apply_force_graph_layout_to_fixture_v1_json(&fixture_json_string(board), r#"{"mode":"force-graph"}"#) else {
        return;
    };
    if let Ok(parsed) = serde_json::from_str::<Value>(&layout_json) {
        *board = to_dsl_value(&parsed).unwrap_or(DslValue::Null);
    }
}

/// 📐️ A JSON node's position, defaulting missing coordinates to the origin.
fn node_position(node: &DslValue) -> (f64, f64) {
    (
        node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0),
        node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0),
    )
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the mindmap wires app; one field per label makes every locale combination compile-checked.
    struct WiresLabels {
        identities: &'static str = en: "Identities", de: "Identitäten";
        relationships: &'static str = en: "Relationships", de: "Beziehungen";
        identity_kinds: &'static str = en: "Identity kinds", de: "Identitätsarten";
        relationship_kinds: &'static str = en: "Relationship kinds", de: "Beziehungsarten";
        relationship_kind_owns: &'static str = en: "Owns", de: "Besitzt";
        relationship_kind_is: &'static str = en: "Is", de: "Ist";
        relationship_kind_references: &'static str = en: "References", de: "Referenziert";
        relationship_kind_has: &'static str = en: "Has", de: "Hat";
        window_main: &'static str = en: "Canvas", de: "Leinwand";
        mode_edit: &'static str = en: "Edit", de: "Bearbeiten";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_wires_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn wires_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    localized_label_map(
        is_de,
        &[
            ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
            ("addNode", "Add Node", "Knoten hinzufügen"),
            ("addRelationship", "Add Relationship", "Beziehung hinzufügen"),
            ("deleteSelection", "Delete Selection", "Auswahl löschen"),
            ("forceLayout", "Force Layout", "Kraftbasiertes Layout"),
            ("reorganize", "Reorganize", "Neu anordnen"),
            ("canvasPointerMove", "Canvas Pointer Move", "Leinwand-Zeiger bewegt"),
            ("setSelection", "Set Selection", "Auswahl festlegen"),
            ("documentSelect", "Document Select", "Dokument auswählen"),
            ("canvasPointerDown", "Canvas Pointer Down", "Leinwand-Zeiger gedrückt"),
            ("canvasPointerUp", "Canvas Pointer Up", "Leinwand-Zeiger losgelassen"),
        ],
    )
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
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
    PanelTreeBuilder::new(WIRES_PLAY_DOCUMENT_NAMESPACE)
        .section_or_placeholder("wires-play-document.identities", Some(labels.identities.into()), true, identity_items, "(none)")
        .section_or_placeholder("wires-play-document.relationships", Some(labels.relationships.into()), false, relationship_items, "(none)")
        .selected(document_tree_selected_ids(board, selected))
        .selection_change(wires_action("setSelection", None))
        .build()
}

fn catalog_kind_label(entry: &DslValue) -> String {
    entry
        .get("name")
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
        .or_else(|| entry.get("id").and_then(|value| value.as_str()))
        .unwrap_or("kind")
        .into()
}

fn kind_catalog_items(namespace: &PanelTreeBuilder, kind: &str, entries: &[DslValue]) -> Vec<UiTreeItemNode> {
    entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
            let action = match kind {
                "identity-kinds" => wires_action("addNode", Some(json!({ "kind": kind_id }))),
                "relationship-kinds" => wires_action("addRelationship", Some(json!({ "kind": kind_id }))),
                _ => wires_action("addNode", Some(json!({ "kind": kind_id }))),
            };
            tree_item_with_action(
                namespace.item_id(kind, &format!("{index}.{kind_id}")),
                catalog_kind_label(entry),
                Some(kind_id.into()),
                action,
            )
        })
        .collect()
}

fn render_catalogue_panel(wires: &DslValue, labels: &WiresLabels) -> UiNode {
    let namespace = PanelTreeBuilder::new("wires-play-kinds");
    let identity_entries = wires_kind_catalog_entries(wires, "identityKinds");
    let relationship_entries = wires_kind_catalog_entries(wires, "relationshipKinds");
    let identity_items = kind_catalog_items(&namespace, "identity-kinds", &identity_entries);
    let relationship_items = kind_catalog_items(&namespace, "relationship-kinds", &relationship_entries);
    namespace
        .section_or_placeholder("wires-play-kinds.identity-kinds", Some(labels.identity_kinds.into()), true, identity_items, "(none)")
        .section_or_placeholder("wires-play-kinds.relationship-kinds", Some(labels.relationship_kinds.into()), true, relationship_items, "(none)")
        .build()
}

fn render_properties_panel(document: &MindmapWiresDocument, selected: &[String]) -> UiNode {
    let selected_nodes: Vec<&DslValue> = selected
        .iter()
        .filter_map(|id| {
            fixture_nodes(&document.board_fixture)
                .iter()
                .find(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
        })
        .collect();
    if selected_nodes.is_empty() {
        let extension = DefaultWiresExtension::from_fixture_json(&fixture_json_string(&document.wires_fixture)).ok();
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
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_canvas(board: &DslValue, wires: &DslValue) -> UiNode {
    let (camera_x, camera_y, zoom) = fixture_camera(board);
    let mut layers: Vec<Value> = fixture_nodes(board).iter().map(dsl_to_json).collect();
    layers.extend(fixture_edges(board).iter().map(dsl_to_json));
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
//#endregion 🔖️Render

//#region 🔖️ReasoningWiresPlayApp
/// 🧪️ B1: unit struct — every former `WiresPlayRuntime` field now lives in
/// `reasoning_wires_engine::WiresConfig` (see `DocumentApp::Config`), written through
/// `reasoning_wires_op::WiresConfigOperation`s.
#[derive(Default)]
pub struct ReasoningWiresPlayApp;

impl DocumentApp for ReasoningWiresPlayApp {
    type Projection = MindmapWiresDocument;
    type Operation = MindmapWiresOperation;
    type Config = WiresConfig;
    type ConfigOperation = WiresConfigOperation;
    type Command = WiresCommand;

    fn app_id(&self) -> &str {
        WIRES_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        WIRES_FIXTURE_SCHEMA
    }

    fn initial_projection(&self) -> MindmapWiresDocument {
        empty_mindmap_wires_document()
    }

    /// 🏷️ Maps each `WiresCommand` variant back to the action id it was declared under in
    /// `create_wires_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &WiresCommand) -> &str {
        match command {
            WiresCommand::SetActiveExample { .. } => "setActiveExample",
            WiresCommand::AddNode { .. } => "addNode",
            WiresCommand::AddRelationship { .. } => "addRelationship",
            WiresCommand::DeleteSelection => "deleteSelection",
            WiresCommand::ForceLayout => "forceLayout",
            WiresCommand::Reorganize => "reorganize",
            WiresCommand::CanvasPointerMove { .. } => "canvasPointerMove",
            WiresCommand::SetSelection { .. } => "setSelection",
            WiresCommand::DocumentSelect { .. } => "documentSelect",
            WiresCommand::CanvasPointerDown { .. } => "canvasPointerDown",
            WiresCommand::CanvasPointerUp => "canvasPointerUp",
            WiresCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(&self, command: &WiresCommand, doc: &DocumentView<'_, MindmapWiresDocument>, cfg: &ConfigView<'_, WiresConfig>) -> Emit<MindmapWiresOperation, WiresConfigOperation> {
        let document = doc.projection;
        let config = cfg.projection;
        match command {
            // 👁️ Config-only — mutate ephemeral selection/drag state, emit no document operations.
            WiresCommand::SetSelection { ids } | WiresCommand::DocumentSelect { ids } => {
                Emit::config(vec![WiresConfigOperation::SetSelection { ids: ids.clone() }])
            }
            WiresCommand::CanvasPointerDown { id, x, y } => match id.as_deref().filter(|id| find_board_node(document, id).is_some()) {
                Some(id) => Emit::config(vec![
                    WiresConfigOperation::SetSelection { ids: vec![id.to_string()] },
                    WiresConfigOperation::SetDrag { node_id: Some(id.to_string()), last_x: *x, last_y: *y },
                ]),
                None => Emit::default(),
            },
            WiresCommand::CanvasPointerUp => Emit::config(vec![WiresConfigOperation::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 }]),
            WiresCommand::SetLocale { value } => Emit::config(vec![WiresConfigOperation::SetLocale { value: value.clone() }]),
            // ✏️ Operations — dispatched as VCS operations with a true inverse.
            WiresCommand::SetActiveExample { example_id } => {
                let next = if example_id.as_str() == WIRES_PLAY_EXAMPLE_METABOLISM_ID {
                    metabolism_wires_example_document()
                } else {
                    empty_mindmap_wires_document()
                };
                Emit {
                    document_operations: vec![MindmapWiresOperation::ReplaceDocument { wires_fixture: next.wires_fixture, board_fixture: next.board_fixture }],
                    config_operations: vec![
                        WiresConfigOperation::SetSelection { ids: Vec::new() },
                        WiresConfigOperation::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 },
                    ],
                    ..Default::default()
                }
            }
            WiresCommand::AddNode { kind } => {
                let kind = if kind.is_empty() { "identity" } else { kind.as_str() };
                let id = format!("node-{}", fixture_nodes(&document.board_fixture).len() + 1);
                let node = to_dsl_value(&json!({
                    "id": id,
                    "nodeKind": kind,
                    "shape": "circle",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 24.0,
                    "text": id,
                    "handles": []
                }))
                .expect("node serializes");
                Emit {
                    document_operations: vec![MindmapWiresOperation::AddNode { node }],
                    config_operations: vec![WiresConfigOperation::SetSelection { ids: vec![id] }],
                    ..Default::default()
                }
            }
            WiresCommand::AddRelationship { kind } => {
                let kind = if kind.is_empty() { "owns" } else { kind.as_str() };
                let edge_id = format!("edge-{}", fixture_edges(&document.board_fixture).len() + 1);
                let edge = to_dsl_value(&json!({
                    "id": edge_id,
                    "edgeKind": format!("wires.{kind}"),
                    "source": "node-1",
                    "target": "node-2"
                }))
                .expect("edge serializes");
                let relationship = to_dsl_value(&json!({
                    "edgeId": edge_id,
                    "kind": kind,
                    "sourceIdentityId": 1,
                    "targetIdentityId": 2
                }))
                .expect("relationship serializes");
                Emit {
                    document_operations: vec![MindmapWiresOperation::AddRelationship { edge, relationship }],
                    config_operations: vec![WiresConfigOperation::SetSelection { ids: vec![edge_id] }],
                    ..Default::default()
                }
            }
            WiresCommand::DeleteSelection => {
                let mut operations = Vec::new();
                for id in &config.selected_ids {
                    if find_board_node(document, id).is_some() {
                        operations.push(MindmapWiresOperation::RemoveNode { node_id: id.clone() });
                    } else if fixture_edges(&document.board_fixture)
                        .iter()
                        .any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))
                    {
                        operations.push(MindmapWiresOperation::RemoveEdge { edge_id: id.clone() });
                    }
                }
                let config_operations = if operations.is_empty() { Vec::new() } else { vec![WiresConfigOperation::SetSelection { ids: Vec::new() }] };
                Emit { document_operations: operations, config_operations, ..Default::default() }
            }
            WiresCommand::ForceLayout | WiresCommand::Reorganize => {
                let mut board = document.board_fixture.clone();
                force_layout_board(&mut board);
                let operations: Vec<MindmapWiresOperation> = fixture_nodes(&board)
                    .iter()
                    .filter_map(|node| {
                        let id = node.get("id").and_then(|value| value.as_str())?;
                        let (nx, ny) = node_position(node);
                        let (ox, oy) = find_board_node(document, id).map(node_position).unwrap_or((nx, ny));
                        if nx == ox && ny == oy {
                            return None;
                        }
                        let mut patch = BTreeMap::new();
                        patch.insert("x".into(), to_dsl_value(&nx).unwrap_or(DslValue::Null));
                        patch.insert("y".into(), to_dsl_value(&ny).unwrap_or(DslValue::Null));
                        Some(MindmapWiresOperation::PatchNode { node_id: id.to_string(), patch })
                    })
                    .collect();
                Emit::operations(operations)
            }
            WiresCommand::CanvasPointerMove { x, y } => {
                let Some(drag_node_id) = config.drag_node_id.clone() else { return Emit::default() };
                let Some(node) = find_board_node(document, &drag_node_id) else { return Emit::default() };
                let zoom = fixture_camera(&document.board_fixture).2.max(1e-6);
                let (cur_x, cur_y) = node_position(node);
                let (dx, dy) = ((x - config.drag_last_x) / zoom, (y - config.drag_last_y) / zoom);
                let mut patch = BTreeMap::new();
                patch.insert("x".into(), to_dsl_value(&(cur_x + dx)).unwrap_or(DslValue::Null));
                patch.insert("y".into(), to_dsl_value(&(cur_y + dy)).unwrap_or(DslValue::Null));
                Emit {
                    document_operations: vec![MindmapWiresOperation::PatchNode { node_id: drag_node_id.clone(), patch }],
                    config_operations: vec![WiresConfigOperation::SetDrag { node_id: Some(drag_node_id.clone()), last_x: *x, last_y: *y }],
                    coalesce_key: Some(format!("drag:{drag_node_id}")),
                    ..Default::default()
                }
            }
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, MindmapWiresDocument>, cfg: &ConfigView<'_, WiresConfig>) -> UiNode {
        let document = doc.projection;
        let labels = resolve_labels::<WiresLabels>(cfg.projection);
        match body_key {
            WIRES_PLAY_BODY_COMPOSITE => render_canvas(&document.board_fixture, &document.wires_fixture),
            WIRES_PLAY_BODY_DOCUMENT => render_document_panel(document, &cfg.projection.selected_ids, labels),
            WIRES_PLAY_BODY_CATALOGUE => render_catalogue_panel(&document.wires_fixture, labels),
            WIRES_PLAY_BODY_PROPERTIES => render_properties_panel(document, &cfg.projection.selected_ids),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, cfg: &ConfigView<'_, WiresConfig>) -> AppLabelsOverlay {
        let labels = resolve_labels::<WiresLabels>(cfg.projection);
        let is_de = is_de_locale(cfg.projection);
        AppLabelsOverlay::with_framework_panel_tabs(
            ["framework.panel.document", "framework.panel.catalogue", "framework.panel.inspection"],
            is_de,
        )
        .window_kind_label("reasoning-wires-composite", labels.window_main)
        .mode_label("edit", labels.mode_edit)
        .action_labels(wires_action_labels(is_de))
        .example_labels(std::collections::HashMap::from([
            (WIRES_PLAY_EXAMPLE_METABOLISM_ID.to_string(), "Metabolism".to_string()),
        ]))
    }
}
//#endregion 🔖️ReasoningWiresPlayApp

//#region 🔖️Manifest
pub fn create_wires_app() -> App {
    App::from_builder(
        App::builder(WIRES_PLAY_APP_ID, "Mindmap Wires").document(["semio", "reasoning", "mindmap", "wires"])
            .artifact_kind(ArtifactKindSpec {
                id: "graph.wires".into(),
                name: "Wires Graph".into(),
                source_format: WIRES_FIXTURE_SCHEMA.into(),
                component_kind: "wires".into(),
                dimension: "graph".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Dag },
                schema: WIRES_FIXTURE_SCHEMA.into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("reasoning-wires")
            .mode("edit", "Edit", "square-pen")
            .default_mode_id("edit")
            .window_kind("reasoning-wires-composite", "Canvas", WIRES_PLAY_BODY_COMPOSITE, SurfaceKind::Canvas2d, "git-branch")
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
            .view_action("canvasPointerUp", "Canvas Pointer Up")
            // 🎯️ Typed channel surface (B1 pure-trait conversion) — `config_spec()`'s single source of
            // truth (the trait default `ConfigSpec::empty()`: none of `WiresConfig`'s fields are
            // user-visible settings, they're ephemeral view state) reused here rather than duplicated.
            .config(ReasoningWiresPlayApp::default().config_spec()),
    )
    .example(
        WIRES_PLAY_EXAMPLE_METABOLISM_ID,
        "Metabolism",
        serde_json::to_string(&metabolism_wires_example_document()).unwrap(),
        "network",
    )
    .workflow("reasoning-wires", "Mindmap Wires", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use reasoning_wires_engine::RelationshipKind;
    use semio_framework_plugin::testkit;
    use semio_framework_plugin::{PluginApp, VcsDocumentApp, ViewState};
    use store::MemoryBackbone;

    fn new_app() -> VcsDocumentApp<ReasoningWiresPlayApp> {
        testkit::new_app::<ReasoningWiresPlayApp>()
    }

    fn metabolism_app() -> VcsDocumentApp<ReasoningWiresPlayApp> {
        let mut app = new_app();
        let document = metabolism_wires_example_document();
        let envelope = store::create_document_envelope::<MindmapWiresDocument, MindmapWiresOperation>(WIRES_FIXTURE_SCHEMA, "reasoning-wires", document, None);
        let files = store::print_document_pack(&envelope).expect("print document pack");
        app.load_document_pack(&files).expect("load metabolism");
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

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more
    /// `ViewState.locale` threaded through `render` (the trait dropped `ViewState` entirely).
    #[test]
    fn wires_labels_resolve_native_in_german() {
        let mut app = metabolism_app();
        app.dispatch_typed(WiresCommand::SetLocale { value: "de-DE".into() }, &testkit::meta("local")).expect("set locale");
        let json = serde_json::to_string(&app.render(WIRES_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("Identitäten"));
        assert!(json.contains("Beziehungen"));
        let catalogue_json = serde_json::to_string(&app.render(WIRES_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render")).unwrap();
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
        let document = metabolism_wires_example_document();
        let ext = DefaultWiresExtension::from_fixture_json(&fixture_json_string(&document.wires_fixture)).expect("metabolism fixture");
        assert_eq!(ext.mindmap.topics.len(), 7);
        assert_eq!(ext.relationships.len(), 9);
    }

    #[test]
    fn relationship_kind_labels_match_fixture() {
        assert_eq!(RelationshipKind::Owns.label(), "owns");
        assert_eq!(relationship_kind_display_name("is", WiresLabels::locale_labels_en()), "Is");
    }

    #[test]
    fn add_node_appends_and_selects() {
        let mut app = new_app();
        app.dispatch_typed(WiresCommand::AddNode { kind: "identity".into() }, &testkit::meta("local")).expect("add");
        let projection = app.projection().expect("projection");
        assert_eq!(fixture_nodes(&projection.board_fixture).len(), 1);
        assert!(find_board_node(&projection, "node-1").is_some());
    }

    #[test]
    fn pointer_drag_translates_node_by_screen_delta() {
        let mut app = new_app();
        app.dispatch_typed(WiresCommand::AddNode { kind: "identity".into() }, &testkit::meta("local")).expect("add");
        app.dispatch_typed(WiresCommand::CanvasPointerDown { id: Some("node-1".into()), x: 100.0, y: 100.0 }, &testkit::meta("local")).expect("down");
        app.dispatch_typed(WiresCommand::CanvasPointerMove { x: 140.0, y: 130.0 }, &testkit::meta("local")).expect("move");
        let node = find_board_node(&app.projection().expect("projection"), "node-1").expect("node-1").clone();
        assert_eq!(node.get("x").and_then(|value| value.as_f64()), Some(40.0));
        assert_eq!(node.get("y").and_then(|value| value.as_f64()), Some(30.0));
        app.dispatch_typed(WiresCommand::CanvasPointerUp, &testkit::meta("local")).expect("up");
        // A coalesced drag collapses to a single undo step restoring the origin.
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        let node = find_board_node(&app.projection().expect("projection"), "node-1").expect("node-1").clone();
        assert_eq!(node.get("x").and_then(|value| value.as_f64()), Some(0.0));
    }

    #[test]
    fn delete_selection_removes_node() {
        let mut app = new_app();
        app.dispatch_typed(WiresCommand::AddNode { kind: "identity".into() }, &testkit::meta("local")).expect("add");
        app.dispatch_typed(WiresCommand::SetSelection { ids: vec!["node-1".into()] }, &testkit::meta("local")).expect("select");
        app.dispatch_typed(WiresCommand::DeleteSelection, &testkit::meta("local")).expect("delete");
        assert!(fixture_nodes(&app.projection().expect("projection").board_fixture).is_empty());
    }

    #[test]
    fn force_layout_action_repositions_metabolism_nodes() {
        let mut app = metabolism_app();
        let before: Vec<(f64, f64)> = fixture_nodes(&app.projection().expect("projection").board_fixture)
            .iter()
            .map(node_position)
            .collect();
        app.dispatch_typed(WiresCommand::ForceLayout, &testkit::meta("local")).expect("force layout");
        let after: Vec<(f64, f64)> =
            fixture_nodes(&app.projection().expect("projection").board_fixture).iter().map(node_position).collect();
        assert_eq!(before.len(), after.len());
        assert_ne!(before, after, "force layout should move at least one node");
    }

    #[test]
    fn metabolism_board_fixture_uses_mindmap_schema() {
        let document = metabolism_wires_example_document();
        assert_eq!(document.board_fixture.get("schema").and_then(|value| value.as_str()), Some(reasoning_wires::MINDMAP_BOARD_SCHEMA));
        assert_eq!(fixture_nodes(&document.board_fixture).len(), 7);
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            WiresCommand::AddNode { kind: "identity".into() },
            |app| fixture_nodes(&app.projection().expect("projection").board_fixture).len(),
            0,
            1,
        );
    }

    /// 🧪️ The definitional merge proof: A adds a node while B renames another node — disjoint edits
    /// on one backbone that must both survive on both instances (impossible under whole-document LWW).
    #[test]
    fn two_instances_converge_disjoint_graph_edits_via_backbone() {
        let mut instance_a = new_app();
        let mut instance_b = new_app();
        // Seed both from an identical base projection carrying node-1/node-2 (as initial state, not
        // as edits) so the only edits on the channel are A's and B's disjoint ones.
        let seed_node = |id: &str| {
            to_dsl_value(&json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": id, "handles": [] }))
                .expect("seed node")
        };
        let mut base = empty_mindmap_wires_document();
        base = store::apply_operation(&base, &MindmapWiresOperation::AddNode { node: seed_node("node-1") });
        base = store::apply_operation(&base, &MindmapWiresOperation::AddNode { node: seed_node("node-2") });
        let base_envelope = store::create_document_envelope::<MindmapWiresDocument, MindmapWiresOperation>(WIRES_FIXTURE_SCHEMA, "reasoning-wires", base, None);
        let base_files = store::print_document_pack(&base_envelope).expect("print document pack");
        instance_a.load_document_pack(&base_files).expect("load a");
        instance_b.load_document_pack(&base_files).expect("load b");
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://mindmap-convergence", "mem://mindmap-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        // A adds node-3 (a new node); B moves node-2 (a PatchNode) — disjoint edits on the graph.
        instance_a.dispatch_typed(WiresCommand::AddNode { kind: "identity".into() }, &testkit::meta("actor-a")).expect("a adds node");
        instance_b.dispatch_typed(WiresCommand::CanvasPointerDown { id: Some("node-2".into()), x: 0.0, y: 0.0 }, &testkit::meta("actor-b")).expect("b down");
        instance_b.dispatch_typed(WiresCommand::CanvasPointerMove { x: 50.0, y: 60.0 }, &testkit::meta("actor-b")).expect("b move");
        instance_b.dispatch_typed(WiresCommand::CanvasPointerUp, &testkit::meta("actor-b")).expect("b up");

        instance_a.handle_action("commitCheckpoint", None, &testkit::meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &testkit::meta("actor-b")).expect("pump b");

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
        testkit::assert_ingest_idempotent::<ReasoningWiresPlayApp, usize>(
            WiresCommand::AddNode { kind: "identity".into() },
            |app| fixture_nodes(&app.projection().expect("projection").board_fixture).len(),
        );
    }
}
//#endregion 🧪️Tests
