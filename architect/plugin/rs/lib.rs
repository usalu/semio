//! 🏛️ Architect plugin — architectural program DocumentApp bundled as a hot-swappable WASM component.

use architect_program::{
    adjacency_matrix, build_report, detect_adjacency_conflicts, empty_program, export_json, import_json,
    normalize_pair, run_analysis, sample_program, search_program, status_summary, undirected_edges,
    validate_program, Adjacency, AdjacencyKind, AnalysisKind, ConnectionKind, EntityHeader, EntityId,
    Program, ProgramElement, ProgramElementKind, ProgramOp, ReportKind, SearchQuery, ValidationStatus,
    ARCHITECT_PROGRAM_SCHEMA,
};
use semio_framework_plugin::{
    create_default_layout, ui_inspector_readonly_field, ui_stack_vertical, ui_text, ActionArgDef,
    ActionArgOption, ActionDefinition, ActionDescriptor, ActionEmit, ActionKind, App, AppLabelsOverlay,
    BlockListScene, DocumentApp, DocumentView, HostEffect, NodeGraphScene, PanelGroup, SurfaceKind,
    UiComponentSceneNode, UiNode, UiStackNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use vcs::CollectionOp;

//#region 🔖Constants
const ARCHITECT_APP_ID: &str = "architect";
const ARCHITECT_BODY_ADJACENCY: &str = "architect.adjacency";
const ARCHITECT_BODY_GRAPH: &str = "architect.graph";
const ARCHITECT_BODY_REGISTER: &str = "architect.register";
const ARCHITECT_BODY_REPORT: &str = "architect.report";
const ARCHITECT_BODY_DOCUMENT: &str = "architect.document";
const ARCHITECT_BODY_CATALOGUE: &str = "architect.catalogue";
const ARCHITECT_BODY_INSPECTION: &str = "architect.inspection";
const ARCHITECT_WINDOW_ADJACENCY: &str = "architect-adjacency";
const ARCHITECT_WINDOW_GRAPH: &str = "architect-graph";
const ARCHITECT_WINDOW_REGISTER: &str = "architect-register";
const ARCHITECT_WINDOW_REPORT: &str = "architect-report";
//#endregion 🔖Constants

//#region 🔖Runtime
/// @emoji 👁️ Ephemeral per-session view state — selection, active register, search, and cached reports.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ArchitectPlayRuntime {
    selected_ids: Vec<String>,
    active_register: String,
    search_query: String,
    last_report_json: String,
    adjacency_kind_filter: Option<AdjacencyKind>,
    graph_camera: GraphCamera,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphCamera {
    x: f64,
    y: f64,
    zoom: f64,
}

impl Default for GraphCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterBlockStep {
    id: String,
    title: String,
    blocks: Vec<RegisterBlockItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterBlockItem {
    id: String,
    label: String,
    kind: String,
}

const REGISTER_IDS: &[&str] = &[
    "elements", "stakeholders", "requirements", "adjacencies", "functions", "activities", "risks", "issues",
];
//#endregion 🔖Runtime

//#region 🔖Helpers
fn architect_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: ARCHITECT_APP_ID.into(),
        action: action.into(),
        args,
    }
}

fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        loading: None,
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
    }
}

fn tree_item_with_action(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    action: ActionDescriptor,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
        selected: None,
        loading: None,
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

fn tree_section(id: impl Into<String>, label: Option<String>, items: Vec<UiTreeItemNode>) -> UiTreeSectionNode {
    UiTreeSectionNode {
        id: id.into(),
        label,
        default_open: Some(true),
        loading: None,
        items,
    }
}

fn tree_node(sections: Vec<UiTreeSectionNode>, selected_ids: Option<Vec<String>>) -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections,
        loading: None,
        selected_ids,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn element_label(program: &Program, id: &EntityId) -> String {
    program
        .elements
        .iter()
        .find(|element| &element.header.id == id)
        .map(|element| element.header.name.clone())
        .unwrap_or_else(|| id.to_string())
}

fn adjacency_kind_label(kind: &AdjacencyKind) -> &'static str {
    match kind {
        AdjacencyKind::Required => "Required",
        AdjacencyKind::Preferred => "Preferred",
        AdjacencyKind::Optional => "Optional",
        AdjacencyKind::Prohibited => "Prohibited",
    }
}

fn next_adjacency_kind(current: Option<&AdjacencyKind>) -> Option<AdjacencyKind> {
    match current {
        None => Some(AdjacencyKind::Required),
        Some(AdjacencyKind::Required) => Some(AdjacencyKind::Preferred),
        Some(AdjacencyKind::Preferred) => Some(AdjacencyKind::Optional),
        Some(AdjacencyKind::Optional) => Some(AdjacencyKind::Prohibited),
        Some(AdjacencyKind::Prohibited) => None,
    }
}

fn find_adjacency<'a>(program: &'a Program, a: &EntityId, b: &EntityId) -> Option<&'a Adjacency> {
    let (left, right) = normalize_pair(a.clone(), b.clone());
    program
        .adjacencies
        .iter()
        .find(|row| row.element_a_id == left && row.element_b_id == right)
}

fn default_element(name: impl Into<String>) -> ProgramElement {
    ProgramElement {
        header: EntityHeader::new(EntityId::new_serial("element"), name),
        code: String::new(),
        kind: ProgramElementKind::Room,
        parent_id: None,
        level: None,
        area: architect_program::QuantitySpec::default(),
        volume: architect_program::QuantitySpec::default(),
        height: architect_program::QuantitySpec::default(),
        occupancy: architect_program::QuantitySpec::default(),
        function_ids: Vec::new(),
        activity_ids: Vec::new(),
        user_profile_ids: Vec::new(),
        adjacency_ids: Vec::new(),
        quantity_ids: Vec::new(),
        requirement_ids: Vec::new(),
        location_hint: None,
        orientation: None,
        daylight_requirement: None,
        acoustic_class: None,
        security_zone: None,
        flexibility_notes: Vec::new(),
        growth_allocation: None,
    }
}

fn new_adjacency(program: &Program, a: EntityId, b: EntityId, kind: AdjacencyKind) -> Adjacency {
    let (left, right) = normalize_pair(a.clone(), b.clone());
    Adjacency {
        header: EntityHeader::new(
            EntityId::new_serial("adjacency"),
            format!("{} ↔ {}", element_label(program, &left), element_label(program, &right)),
        ),
        element_a_id: left,
        element_b_id: right,
        kind,
        connection: ConnectionKind::Direct,
        separations: Vec::new(),
        weight: 1.0,
        rationale: None,
        distance_max_m: None,
        distance_min_m: None,
        level_constraint: None,
        access_path: None,
        shared_wall: false,
        shared_entry: false,
        traffic_isolation: false,
        circulation_overlap: false,
        conflict_ids: Vec::new(),
        normalized: true,
        verification_status: ValidationStatus::Pending,
        source_relationship_id: None,
    }
}

fn store_runtime_json<T: Serialize>(runtime: &mut ArchitectPlayRuntime, value: &T) {
    runtime.last_report_json = serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into());
}

fn register_entities(program: &Program, register: &str) -> Vec<(EntityId, String)> {
    match register {
        "elements" => program.elements.iter().map(|e| (e.header.id.clone(), e.header.name.clone())).collect(),
        "stakeholders" => program.stakeholders.iter().map(|e| (e.header.id.clone(), e.header.name.clone())).collect(),
        "requirements" => program.requirements.iter().map(|e| (e.header.id.clone(), e.header.name.clone())).collect(),
        "adjacencies" => program.adjacencies.iter().map(|e| (e.header.id.clone(), e.header.name.clone())).collect(),
        "functions" => program.functions.iter().map(|e| (e.header.id.clone(), e.header.name.clone())).collect(),
        "activities" => program.activities.iter().map(|e| (e.header.id.clone(), e.header.name.clone())).collect(),
        "risks" => program.risks.iter().map(|e| (e.header.id.clone(), e.header.name.clone())).collect(),
        "issues" => program.issues.iter().map(|e| (e.header.id.clone(), e.header.name.clone())).collect(),
        _ => Vec::new(),
    }
}

fn empty_component_scene(surface_id: &str, component_kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: ARCHITECT_APP_ID.into(),
        component_kind,
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
    }
}

fn parse_entity_id(value: Option<&Value>, key: &str) -> Option<EntityId> {
    value.and_then(|args| args.get(key)).and_then(|v| v.as_str()).map(|s| EntityId(s.into()))
}

fn parse_adjacency_kind(value: Option<&Value>) -> Option<AdjacencyKind> {
    value
        .and_then(|args| args.get("kind"))
        .and_then(|v| v.as_str())
        .and_then(|kind| match kind {
            "required" => Some(AdjacencyKind::Required),
            "preferred" => Some(AdjacencyKind::Preferred),
            "optional" => Some(AdjacencyKind::Optional),
            "prohibited" => Some(AdjacencyKind::Prohibited),
            _ => None,
        })
}

fn analysis_kind_from_args(args: Option<&Value>) -> AnalysisKind {
    args.and_then(|value| value.get("analysisKind"))
        .and_then(|v| v.as_str())
        .map(|kind| match kind {
            "conflict" => AnalysisKind::Conflict,
            "dependency" => AnalysisKind::Dependency,
            "capacity" => AnalysisKind::Capacity,
            "workflow" => AnalysisKind::Workflow,
            "risk" => AnalysisKind::Risk,
            "scenario" => AnalysisKind::Scenario,
            _ => AnalysisKind::Gap,
        })
        .unwrap_or(AnalysisKind::Gap)
}

fn report_kind_from_args(args: Option<&Value>) -> ReportKind {
    args.and_then(|value| value.get("reportKind"))
        .and_then(|v| v.as_str())
        .map(|kind| match kind {
            "programOverview" => ReportKind::ProgramOverview,
            "stakeholderSummary" => ReportKind::StakeholderSummary,
            "requirementsMatrix" => ReportKind::RequirementsMatrix,
            "adjacencyMatrix" => ReportKind::AdjacencyMatrix,
            "gapAnalysis" => ReportKind::GapAnalysis,
            "riskRegister" => ReportKind::RiskRegister,
            "validationSummary" => ReportKind::ValidationSummary,
            "recommendation" => ReportKind::Recommendation,
            _ => ReportKind::ExecutiveSummary,
        })
        .unwrap_or(ReportKind::ExecutiveSummary)
}
//#endregion 🔖Helpers

//#region 🔖AdjacencyRender
/// @emoji 🔺 Signature adjacency matrix — triangle glyph strip plus lower-triangle pair rows.
fn render_adjacency_body(program: &Program, runtime: &ArchitectPlayRuntime) -> UiNode {
    let matrix = adjacency_matrix(program);
    let n = matrix.element_ids.len();
    if n == 0 {
        return ui_text("Add program elements to edit adjacencies.");
    }

    let mut glyph_rows = Vec::new();
    let mut pair_sections = Vec::new();

    glyph_rows.push(ui_text(" "));
    pair_sections.push(tree_section(
        "architect-adjacency.headers",
        Some("Columns".into()),
        matrix
            .element_ids
            .iter()
            .enumerate()
            .map(|(index, id)| tree_item(format!("architect-adjacency.col.{index}"), element_label(program, id)))
            .collect(),
    ));

    for row in 1..n {
        let row_id = &matrix.element_ids[row];
        let glyph = "▲".repeat(row);
        glyph_rows.push(ui_text(glyph));

        let mut items = Vec::new();
        for col in 0..row {
            let col_id = &matrix.element_ids[col];
            let cell = &matrix.cells[row][col];
            if let Some(filter) = &runtime.adjacency_kind_filter {
                match cell {
                    Some(existing) if &existing.kind != filter => continue,
                    None => continue,
                    _ => {}
                }
            }
            let kind_label = cell
                .as_ref()
                .map(|existing| adjacency_kind_label(&existing.kind).to_string())
                .unwrap_or_else(|| "—".into());
            let label = format!(
                "{} ↔ {} [{kind_label}]",
                element_label(program, col_id),
                element_label(program, row_id)
            );
            items.push(tree_item_with_action(
                format!("architect-adjacency.pair.{}-{}", col_id, row_id),
                label,
                None,
                architect_action(
                    "setAdjacencyKind",
                    Some(json!({
                        "elementAId": col_id,
                        "elementBId": row_id,
                        "cycle": true
                    })),
                ),
            ));
        }

        pair_sections.push(tree_section(
            format!("architect-adjacency.row.{row}"),
            Some(element_label(program, row_id)),
            items,
        ));
    }

    let conflicts = detect_adjacency_conflicts(program);
    if !conflicts.is_empty() {
        pair_sections.push(tree_section(
            "architect-adjacency.conflicts",
            Some(format!("Conflicts ({})", conflicts.len())),
            conflicts
                .iter()
                .map(|conflict| tree_item(format!("architect-adjacency.conflict.{}", conflict.adjacency_a_id), &conflict.message))
                .collect(),
        ));
    }

    UiNode::Stack(UiStackNode {
        direction: "row".into(),
        gap: Some("0.5rem".into()),
        padding: None,
        id: Some("architect-adjacency.matrix".into()),
        selected: None,
        loading: None,
        activate: None,
        drop_action: None,
        children: vec![ui_stack_vertical(glyph_rows), tree_node(pair_sections, None)],
    })
}
//#endregion 🔖AdjacencyRender

//#region 🔖GraphRender
fn graph_media_json(program: &Program, _camera: &GraphCamera) -> (String, String) {
    let count = program.elements.len().max(1);
    let radius = 220.0;
    let center_x = 320.0;
    let center_y = 240.0;
    let nodes: Vec<Value> = program
        .elements
        .iter()
        .enumerate()
        .map(|(index, element)| {
            let angle = std::f64::consts::TAU * (index as f64) / (count as f64);
            json!({
                "id": element.header.id,
                "label": element.header.name,
                "x": center_x + radius * angle.cos(),
                "y": center_y + radius * angle.sin(),
                "width": 108.0,
                "height": 44.0,
                "inputs": [{"id": "in"}],
                "outputs": [{"id": "out"}],
            })
        })
        .collect();
    let edges: Vec<Value> = undirected_edges(program)
        .iter()
        .enumerate()
        .map(|(index, (source, target, weight))| {
            json!({
                "id": format!("edge-{index}"),
                "sourceNodeId": source,
                "sourcePortId": "out",
                "targetNodeId": target,
                "targetPortId": "in",
                "label": format!("{weight:.1}"),
            })
        })
        .collect();
    (
        serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
    )
}

fn render_graph_body(program: &Program, runtime: &ArchitectPlayRuntime) -> UiNode {
    let (nodes_json, edges_json) = graph_media_json(program, &runtime.graph_camera);
    let viewport_json = serde_json::to_string(&runtime.graph_camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let mut scene = empty_component_scene(ARCHITECT_BODY_GRAPH, SurfaceKind::NodeGraph);
    scene.node_graph = Some(NodeGraphScene {
        editable: Some(true),
        selection_json: if runtime.selected_ids.is_empty() {
            None
        } else {
            Some(serde_json::to_string(&runtime.selected_ids).unwrap_or_else(|_| "[]".into()))
        },
        ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
    });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖GraphRender

//#region 🔖RegisterRender
fn render_register_body(program: &Program, runtime: &ArchitectPlayRuntime) -> UiNode {
    let register = if runtime.active_register.is_empty() {
        "elements"
    } else {
        runtime.active_register.as_str()
    };
    let entities = register_entities(program, register);
    if entities.is_empty() {
        return ui_text(format!("No entities in register '{register}'."));
    }

    let steps: Vec<RegisterBlockStep> = entities
        .iter()
        .map(|(id, name)| RegisterBlockStep {
            id: id.to_string(),
            title: name.clone(),
            blocks: vec![RegisterBlockItem {
                id: format!("{id}-block"),
                label: name.clone(),
                kind: register.into(),
            }],
        })
        .collect();
    let steps_json = serde_json::to_string(&steps).unwrap_or_else(|_| "[]".into());
    let palette_json = serde_json::to_string(&[json!({
        "blockKind": register,
        "label": register,
        "iconId": "square",
    })])
    .unwrap_or_else(|_| "[]".into());
    let selected_id = runtime.selected_ids.first().cloned();
    let mut scene = empty_component_scene(ARCHITECT_BODY_REGISTER, SurfaceKind::BlockList);
    scene.block_list = Some(BlockListScene {
        steps_json,
        palette_json,
        selected_id,
        dragging_id: None,
    });
    UiNode::ComponentScene(scene)
}

//#endregion 🔖RegisterRender

//#region 🔖Panels
fn build_document_tree(program: &Program, runtime: &ArchitectPlayRuntime) -> UiNode {
    let summary = status_summary(program);
    let element_items: Vec<UiTreeItemNode> = program
        .elements
        .iter()
        .map(|element| {
            tree_item_with_action(
                format!("architect-document.element.{}", element.header.id),
                format!("{} ({:?})", element.header.name, element.kind),
                Some(element.header.id.to_string()),
                architect_action("setSelection", Some(json!({ "ids": [element.header.id] }))),
            )
        })
        .collect();
    let register_items: Vec<UiTreeItemNode> = summary
        .by_register
        .iter()
        .map(|row| {
            tree_item_with_action(
                format!("architect-document.register.{}", row.register),
                format!("{} ({})", row.register, row.count),
                None,
                architect_action("selectRegister", Some(json!({ "register": row.register }))),
            )
        })
        .collect();
    tree_node(
        vec![
            tree_section(
                "architect-document.meta",
                Some("Program".into()),
                vec![
                    tree_item("architect-document.meta.title", format!("Title: {}", program.meta.title)),
                    tree_item(
                        "architect-document.meta.project",
                        format!("Project: {} ({})", program.project.client_name, program.project.code),
                    ),
                    tree_item(
                        "architect-document.meta.entities",
                        format!("Entities tracked: {}", summary.total_entities),
                    ),
                ],
            ),
            tree_section("architect-document.registers", Some("Registers".into()), register_items),
            tree_section(
                "architect-document.elements",
                Some("Elements".into()),
                if element_items.is_empty() {
                    vec![tree_item("architect-document.elements.empty", "(none)")]
                } else {
                    element_items
                },
            ),
        ],
        Some(
            runtime
                .selected_ids
                .iter()
                .map(|id| format!("architect-document.element.{id}"))
                .collect(),
        ),
    )
}

fn build_catalogue_tree() -> UiNode {
    let register_items: Vec<UiTreeItemNode> = REGISTER_IDS
        .iter()
        .map(|register| {
            tree_item_with_action(
                format!("architect-catalogue.register.{register}"),
                *register,
                None,
                architect_action("selectRegister", Some(json!({ "register": register }))),
            )
        })
        .collect();
    tree_node(
        vec![
            tree_section(
                "architect-catalogue.actions",
                Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
                vec![
                    tree_item_with_action(
                        "architect-catalogue.add-element",
                        "Add Element",
                        None,
                        architect_action("addElement", Some(json!({ "name": "New Room" }))),
                    ),
                    tree_item_with_action(
                        "architect-catalogue.validate",
                        "Run Validation",
                        None,
                        architect_action("runValidation", None),
                    ),
                    tree_item_with_action(
                        "architect-catalogue.analysis-gap",
                        "Gap Analysis",
                        None,
                        architect_action("runAnalysis", Some(json!({ "analysisKind": "gap" }))),
                    ),
                    tree_item_with_action(
                        "architect-catalogue.report-executive",
                        "Executive Report",
                        None,
                        architect_action("runReport", Some(json!({ "reportKind": "executiveSummary" }))),
                    ),
                    tree_item_with_action(
                        "architect-catalogue.export",
                        "Export Program",
                        None,
                        architect_action("exportProgram", None),
                    ),
                ],
            ),
            tree_section("architect-catalogue.registers", Some("Registers".into()), register_items),
        ],
        None,
    )
}

fn build_inspection_tree(program: &Program, runtime: &ArchitectPlayRuntime) -> UiNode {
    if runtime.selected_ids.is_empty() {
        return ui_stack_vertical(vec![ui_text("Select an entity in the document or register view.")]);
    }
    let id = EntityId(runtime.selected_ids[0].clone());
    if let Some(element) = program.elements.iter().find(|row| row.header.id == id) {
        return ui_stack_vertical(vec![
            ui_inspector_readonly_field("architect-inspection.element.id", "Id", element.header.id.to_string()),
            ui_inspector_readonly_field("architect-inspection.element.name", "Name", element.header.name.clone()),
            ui_inspector_readonly_field("architect-inspection.element.kind", "Kind", format!("{:?}", element.kind)),
            ui_inspector_readonly_field("architect-inspection.element.code", "Code", element.code.clone()),
            ui_inspector_readonly_field(
                "architect-inspection.element.level",
                "Level",
                element.level.clone().unwrap_or_else(|| "—".into()),
            ),
        ]);
    }
    if let Some(stakeholder) = program.stakeholders.iter().find(|row| row.header.id == id) {
        return ui_stack_vertical(vec![
            ui_inspector_readonly_field("architect-inspection.stakeholder.id", "Id", stakeholder.header.id.to_string()),
            ui_inspector_readonly_field("architect-inspection.stakeholder.name", "Name", stakeholder.header.name.clone()),
            ui_inspector_readonly_field("architect-inspection.stakeholder.role", "Role", stakeholder.role.clone()),
            ui_inspector_readonly_field(
                "architect-inspection.stakeholder.organization",
                "Organization",
                stakeholder.organization.clone(),
            ),
        ]);
    }
    if let Some(adjacency) = program.adjacencies.iter().find(|row| row.header.id == id) {
        return ui_stack_vertical(vec![
            ui_inspector_readonly_field("architect-inspection.adjacency.id", "Id", adjacency.header.id.to_string()),
            ui_inspector_readonly_field(
                "architect-inspection.adjacency.pair",
                "Pair",
                format!(
                    "{} ↔ {}",
                    element_label(program, &adjacency.element_a_id),
                    element_label(program, &adjacency.element_b_id)
                ),
            ),
            ui_inspector_readonly_field(
                "architect-inspection.adjacency.kind",
                "Kind",
                adjacency_kind_label(&adjacency.kind),
            ),
            ui_inspector_readonly_field(
                "architect-inspection.adjacency.weight",
                "Weight",
                format!("{:.2}", adjacency.weight),
            ),
        ]);
    }
    ui_stack_vertical(vec![ui_text(format!("Entity {id} not found in active registers."))])
}
//#endregion 🔖Panels

//#region 🔖ArchitectPlayApp
#[derive(Default)]
struct ArchitectPlayApp {
    runtime: ArchitectPlayRuntime,
}

impl ArchitectPlayApp {
    fn ensure_default_register(&mut self) {
        if self.runtime.active_register.is_empty() {
            self.runtime.active_register = "elements".into();
        }
    }
}

impl DocumentApp for ArchitectPlayApp {
    type Projection = Program;
    type Op = ProgramOp;

    fn app_id(&self) -> &str {
        ARCHITECT_APP_ID
    }

    fn document_schema(&self) -> &str {
        ARCHITECT_PROGRAM_SCHEMA
    }

    fn initial_projection(&self) -> Program {
        sample_program()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, Program>,
        _view_state: &ViewState,
    ) -> ActionEmit<ProgramOp> {
        self.ensure_default_register();
        let program = doc.projection;
        match action {
            "setSelection" => {
                if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| value.as_array()) {
                    self.runtime.selected_ids = ids
                        .iter()
                        .filter_map(|value| value.as_str().map(str::to_string))
                        .collect();
                }
                ActionEmit::default()
            }
            "selectRegister" => {
                if let Some(register) = args.and_then(|value| value.get("register")).and_then(|value| value.as_str()) {
                    self.runtime.active_register = register.into();
                    self.runtime.selected_ids.clear();
                }
                ActionEmit::default()
            }
            "search" => {
                if let Some(query) = args.and_then(|value| value.get("query")).and_then(|value| value.as_str()) {
                    self.runtime.search_query = query.into();
                    let hits = search_program(
                        program,
                        &SearchQuery {
                            keywords: query.split_whitespace().map(str::to_string).collect(),
                            ..SearchQuery::default()
                        },
                        None,
                    );
                    self.runtime.selected_ids = hits.iter().take(8).map(|hit| hit.entity_id.to_string()).collect();
                    store_runtime_json(&mut self.runtime, &hits);
                }
                ActionEmit::default()
            }
            "setAdjacencyKind" => {
                let a = parse_entity_id(args, "elementAId");
                let b = parse_entity_id(args, "elementBId");
                let (Some(a), Some(b)) = (a, b) else {
                    return ActionEmit::default();
                };
                let cycle = args.and_then(|value| value.get("cycle")).and_then(|value| value.as_bool()).unwrap_or(false);
                let explicit = parse_adjacency_kind(args);
                let existing = find_adjacency(program, &a, &b);
                let next = if cycle {
                    next_adjacency_kind(existing.map(|row| &row.kind))
                } else {
                    explicit.or_else(|| next_adjacency_kind(existing.map(|row| &row.kind)))
                };
                match next {
                    Some(kind) => {
                        let adjacency = if let Some(row) = existing {
                            let mut updated = row.clone();
                            updated.kind = kind;
                            updated
                        } else {
                            new_adjacency(program, a, b, kind)
                        };
                        ActionEmit::ops(vec![ProgramOp::SetAdjacency { adjacency }])
                    }
                    None => {
                        if let Some(row) = existing {
                            ActionEmit::ops(vec![ProgramOp::ClearAdjacency { id: row.header.id.clone() }])
                        } else {
                            ActionEmit::default()
                        }
                    }
                }
            }
            "addElement" => {
                let name = args
                    .and_then(|value| value.get("name"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("New Room");
                let element = default_element(name);
                let id = element.header.id.to_string();
                self.runtime.selected_ids = vec![id];
                self.runtime.active_register = "elements".into();
                ActionEmit::ops(vec![ProgramOp::Elements(CollectionOp::Add {
                    index: program.elements.len(),
                    item: element,
                })])
            }
            "removeElement" => {
                let id = args
                    .and_then(|value| value.get("elementId"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str());
                let Some(id) = id else {
                    return ActionEmit::default();
                };
                self.runtime.selected_ids.retain(|selected| selected != id);
                let mut ops = vec![ProgramOp::Elements(CollectionOp::Remove {
                    id: EntityId(id.into()),
                })];
                for adjacency in program
                    .adjacencies
                    .iter()
                    .filter(|row| row.element_a_id.0 == id || row.element_b_id.0 == id)
                {
                    ops.push(ProgramOp::ClearAdjacency {
                        id: adjacency.header.id.clone(),
                    });
                }
                ActionEmit::ops(ops)
            }
            "runValidation" => {
                let diagnostics = validate_program(program);
                store_runtime_json(&mut self.runtime, &diagnostics);
                ActionEmit::default()
            }
            "runAnalysis" => {
                let result = run_analysis(program, analysis_kind_from_args(args));
                store_runtime_json(&mut self.runtime, &result);
                ActionEmit::default()
            }
            "runReport" => {
                let report = build_report(program, report_kind_from_args(args));
                store_runtime_json(&mut self.runtime, &report);
                ActionEmit::default()
            }
            "exportProgram" => {
                let json_text = export_json(program).unwrap_or_else(|error| json!({ "error": error.to_string() }).to_string());
                ActionEmit::effect(HostEffect::DownloadMediaExport {
                    filename: format!("{}.architect.json", program.meta.document_id),
                    mime_type: "application/json".into(),
                    data: json_text,
                    encoding: None,
                })
            }
            "importProgram" => {
                let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                let Ok(next) = import_json(json_text) else {
                    return ActionEmit::default();
                };
                self.runtime.selected_ids.clear();
                ActionEmit::ops(vec![ProgramOp::SetProgram { program: next }])
            }
            "nodeGraphEdit" => {
                let edit_ops = args
                    .and_then(|value| value.get("ops"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut emitted = Vec::new();
                for op in edit_ops {
                    match op.get("op").and_then(Value::as_str).unwrap_or("") {
                        "connect" => {
                            let source = op.get("sourceNodeId").and_then(Value::as_str);
                            let target = op.get("targetNodeId").and_then(Value::as_str);
                            if let (Some(source), Some(target)) = (source, target) {
                                let a = EntityId(source.into());
                                let b = EntityId(target.into());
                                let kind = find_adjacency(program, &a, &b)
                                    .map(|row| row.kind.clone())
                                    .unwrap_or(AdjacencyKind::Preferred);
                                emitted.push(ProgramOp::SetAdjacency {
                                    adjacency: new_adjacency(program, a, b, kind),
                                });
                            }
                        }
                        "deleteSelection" => {
                            if let Some(ids) = op.get("nodeIds").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                                for id in ids {
                                    emitted.push(ProgramOp::Elements(CollectionOp::Remove {
                                        id: EntityId(id.clone()),
                                    }));
                                    for adjacency in program
                                        .adjacencies
                                        .iter()
                                        .filter(|row| row.element_a_id.0 == id || row.element_b_id.0 == id)
                                    {
                                        emitted.push(ProgramOp::ClearAdjacency {
                                            id: adjacency.header.id.clone(),
                                        });
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if emitted.is_empty() {
                    ActionEmit::default()
                } else {
                    ActionEmit::ops(emitted)
                }
            }
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(Value::as_str) {
                    if let Ok(camera) = serde_json::from_str::<GraphCamera>(viewport_json) {
                        self.runtime.graph_camera = camera;
                    }
                }
                ActionEmit::default()
            }
            "setAdjacencyFilter" => {
                self.runtime.adjacency_kind_filter = parse_adjacency_kind(args);
                ActionEmit::default()
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Program>, _view_state: &ViewState) -> UiNode {
        let program = doc.projection;
        match body_key {
            ARCHITECT_BODY_ADJACENCY => render_adjacency_body(program, &self.runtime),
            ARCHITECT_BODY_GRAPH => render_graph_body(program, &self.runtime),
            ARCHITECT_BODY_REGISTER => render_register_body(program, &self.runtime),
            ARCHITECT_BODY_REPORT => {
                if self.runtime.last_report_json.is_empty() {
                    ui_text("Run validation, analysis, or report to populate this panel.")
                } else {
                    ui_text(&self.runtime.last_report_json)
                }
            }
            ARCHITECT_BODY_DOCUMENT => build_document_tree(program, &self.runtime),
            ARCHITECT_BODY_CATALOGUE => build_catalogue_tree(),
            ARCHITECT_BODY_INSPECTION => build_inspection_tree(program, &self.runtime),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        let mut overlay = AppLabelsOverlay::with_framework_panel_tabs(
            [
                "framework.panel.document",
                "framework.panel.catalogue",
                "framework.panel.inspection",
            ],
            is_de,
        );
        overlay.window_kind_labels = HashMap::from([
            (ARCHITECT_WINDOW_ADJACENCY.to_string(), "Adjacency".into()),
            (ARCHITECT_WINDOW_GRAPH.to_string(), "Graph".into()),
            (ARCHITECT_WINDOW_REGISTER.to_string(), "Register".into()),
            (ARCHITECT_WINDOW_REPORT.to_string(), "Report".into()),
        ]);
        overlay.mode_labels = HashMap::from([
            ("edit".into(), "Edit".into()),
            ("review".into(), "Review".into()),
            ("report".into(), "Report".into()),
        ]);
        overlay.action_labels = architect_action_labels(is_de);
        overlay.example_labels = HashMap::from([
            ("sample".into(), "Sample Clinic".into()),
            ("empty".into(), "Empty Program".into()),
        ]);
        overlay
    }
}
//#endregion 🔖ArchitectPlayApp

//#region 🔖CommandLabels
fn architect_action_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("setAdjacencyKind", "Set Adjacency Kind", "Adjazenzart festlegen"),
        ("selectRegister", "Select Register", "Register waehlen"),
        ("addElement", "Add Element", "Element hinzufuegen"),
        ("removeElement", "Remove Element", "Element entfernen"),
        ("runValidation", "Run Validation", "Validierung ausfuehren"),
        ("runAnalysis", "Run Analysis", "Analyse ausfuehren"),
        ("runReport", "Run Report", "Bericht erzeugen"),
        ("exportProgram", "Export Program", "Programm exportieren"),
        ("importProgram", "Import Program", "Programm importieren"),
        ("nodeGraphEdit", "Node Graph Edit", "Knotengraph bearbeiten"),
        ("nodeGraphViewport", "Node Graph Viewport", "Knotengraph-Ansicht"),
        ("search", "Search", "Suchen"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
    ];
    ENTRIES
        .iter()
        .map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string()))
        .collect()
}
//#endregion 🔖CommandLabels

//#region 🔖Manifest
fn create_architect_app() -> App {
    App::from_builder(
        App::builder(ARCHITECT_APP_ID, "Architect")
            .document(["semio", "architect"])
            .icon_id("architect")
            .mode("edit", "Edit")
            .mode("review", "Review")
            .mode("report", "Report")
            .default_mode_id("edit")
            .window_kind(
                ARCHITECT_WINDOW_ADJACENCY,
                "Adjacency",
                ARCHITECT_BODY_ADJACENCY,
                SurfaceKind::Canvas2d,
            )
            .window_kind(ARCHITECT_WINDOW_GRAPH, "Graph", ARCHITECT_BODY_GRAPH, SurfaceKind::NodeGraph)
            .window_kind(
                ARCHITECT_WINDOW_REGISTER,
                "Register",
                ARCHITECT_BODY_REGISTER,
                SurfaceKind::BlockList,
            )
            .window_kind(ARCHITECT_WINDOW_REPORT, "Report", ARCHITECT_BODY_REPORT, SurfaceKind::TextEditor)
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                ARCHITECT_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                ARCHITECT_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                ARCHITECT_BODY_INSPECTION,
            )
            .operation("setAdjacencyKind", "Set Adjacency Kind")
            .operation("addElement", "Add Element")
            .operation("removeElement", "Remove Element")
            .operation("importProgram", "Import Program")
            .operation("nodeGraphEdit", "Node Graph Edit")
            .operation("nodeGraphViewport", "Node Graph Viewport")
            .view_action("selectRegister", "Select Register")
            .view_action("runValidation", "Run Validation")
            .view_action("runAnalysis", "Run Analysis")
            .view_action("runReport", "Run Report")
            .view_action("search", "Search")
            .view_action("setSelection", "Set Selection")
            .shell_action("exportProgram", "Export Program")
            .action_with(ActionDefinition {
                in_palette: false,
                ..ActionDefinition::new("setAdjacencyFilter", "Set Adjacency Filter", ActionKind::View)
            })
            .action_args(
                "setAdjacencyKind",
                vec![ActionArgDef::select(
                    "kind",
                    "Kind",
                    vec![
                        ActionArgOption::new("required", "Required"),
                        ActionArgOption::new("preferred", "Preferred"),
                        ActionArgOption::new("optional", "Optional"),
                        ActionArgOption::new("prohibited", "Prohibited"),
                    ],
                )],
            )
            .action_args(
                "runAnalysis",
                vec![ActionArgDef::select(
                    "analysisKind",
                    "Analysis",
                    vec![
                        ActionArgOption::new("gap", "Gap"),
                        ActionArgOption::new("conflict", "Conflict"),
                        ActionArgOption::new("dependency", "Dependency"),
                        ActionArgOption::new("workflow", "Workflow"),
                        ActionArgOption::new("risk", "Risk"),
                    ],
                )],
            )
            .action_args(
                "runReport",
                vec![ActionArgDef::select(
                    "reportKind",
                    "Report",
                    vec![
                        ActionArgOption::new("executiveSummary", "Executive Summary"),
                        ActionArgOption::new("programOverview", "Program Overview"),
                        ActionArgOption::new("adjacencyMatrix", "Adjacency Matrix"),
                        ActionArgOption::new("validationSummary", "Validation Summary"),
                    ],
                )],
            )
            .action_args("search", vec![ActionArgDef::text("query", "Query")])
            .action_args("importProgram", vec![ActionArgDef::text("json", "Program JSON")])
            .default_layout(create_default_layout(
                &[
                    ARCHITECT_WINDOW_ADJACENCY.into(),
                    ARCHITECT_WINDOW_GRAPH.into(),
                    ARCHITECT_WINDOW_REGISTER.into(),
                    ARCHITECT_WINDOW_REPORT.into(),
                ],
                "row",
                Some(&[30.0, 30.0, 20.0, 20.0]),
                Some(&["Adjacency".into(), "Graph".into(), "Register".into(), "Report".into()]),
            )),
    )
    .example("sample", "Sample Clinic", serde_json::to_string(&sample_program()).unwrap())
    .example("empty", "Empty Program", serde_json::to_string(&empty_program()).unwrap())
    .program("architect", "Architect", "data")
}

fn register_architect_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "architect",
    label: "Architect",
    version: "0.1.0",
    setup: register_architect_exports,
    apps: [ create_architect_app => ArchitectPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    fn with_doc_view<R>(program: &Program, run: impl FnOnce(DocumentView<'_, Program>) -> R) -> R {
        let history = HistoryView {
            columns: Vec::new(),
            can_undo: false,
            can_redo: false,
            active_alternative_id: None,
            current_checkpoint_id: None,
        };
        run(DocumentView { projection: program, history: &history })
    }

    #[test]
    fn adjacency_matrix_renders_triangle_strip() {
        let app = ArchitectPlayApp::default();
        let program = sample_program();
        with_doc_view(&program, |doc| {
            let node = app.render(ARCHITECT_BODY_ADJACENCY, &doc, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains('▲'));
            assert!(json.contains("Reception"));
        });
    }

    #[test]
    fn graph_body_emits_node_graph_scene() {
        let app = ArchitectPlayApp::default();
        let program = sample_program();
        with_doc_view(&program, |doc| {
            let node = app.render(ARCHITECT_BODY_GRAPH, &doc, &ViewState::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("node-graph"));
        });
    }

    #[test]
    fn set_adjacency_kind_cycles_required_to_preferred() {
        let mut app = ArchitectPlayApp::default();
        let program = sample_program();
        let adjacency = program.adjacencies.first().expect("adjacency");
        with_doc_view(&program, |doc| {
            let emit = app.handle_action(
                "setAdjacencyKind",
                Some(&json!({
                    "elementAId": adjacency.element_a_id,
                    "elementBId": adjacency.element_b_id,
                    "cycle": true
                })),
                &doc,
                &ViewState::default(),
            );
            assert!(matches!(
                emit.ops.first(),
                Some(ProgramOp::SetAdjacency { adjacency: updated }) if updated.kind == AdjacencyKind::Preferred
            ));
        });
    }

    #[test]
    fn run_validation_populates_report_json() {
        let mut app = ArchitectPlayApp::default();
        let program = sample_program();
        with_doc_view(&program, |doc| {
            app.handle_action("runValidation", None, &doc, &ViewState::default());
        });
        assert!(!app.runtime.last_report_json.is_empty());
    }

    #[test]
    fn search_finds_sample_elements() {
        let mut app = ArchitectPlayApp::default();
        let program = sample_program();
        with_doc_view(&program, |doc| {
            app.handle_action(
                "search",
                Some(&json!({ "query": "Reception" })),
                &doc,
                &ViewState::default(),
            );
        });
        assert!(!app.runtime.selected_ids.is_empty());
    }
}
//#endregion 🧪Tests
