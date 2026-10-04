#!/usr/bin/env python3
"""🪆️ S4-TEXT (session 4, design §20.15, AUDIT-TOOLS F1 rewriting): rewriting's working-graph edits leave the parent lane. The six
parent leaves that read the composed `workingGraph` child (drag-/patch-/delete-working-nodes, connect-working-ports,
disconnect-working-edges, add-working-node) are deleted with every registry that names them; the editor publishes child-lane leaves
of the shared `s.stdio.semio@v1/graph` vocabulary (`drag-nodes`, `change-node-label`/`change-node-kind`, `delete-edge`+`delete-node`,
`create-edge`, `create-node`) and every reader composes the parent with the exact member-store child. The remaining parent leaves are
renumbered densely (drag-rule-nodes 7, set-rule-layout-points 8). Every rewrite asserts its anchor; files are staged and written
together, deletions last; `--check` is a dry run. Run from the repo root."""
import json
import re
import shutil
import sys
from pathlib import Path

R = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting")
S = R / "🏅️standards/🔖️1/🪆️subsets/✳️any"
M = S / "🧬️schema/🧬️mutations"
F = S / "🧫️fixtures/🧬️mutations"
E = S / "✏️editor"
GONE = ["✋️drag-working", "🩹️patch-working", "✂️delete-working", "🔌️connect-working", "🪚️disconnect-working", "➕️add-working"]
GONE_MODS = ["drag_working_nodes", "patch_working_nodes", "delete_working_nodes", "connect_working_ports", "disconnect_working_edges", "add_working_node"]
GONE_VARIANTS = ["DragWorkingNodes", "PatchWorkingNodes", "DeleteWorkingNodes", "ConnectWorkingPorts", "DisconnectWorkingEdges", "AddWorkingNode"]
GONE_KINDS = ["drag-working-nodes", "patch-working-nodes", "delete-working-nodes", "connect-working-ports", "disconnect-working-edges", "add-working-node"]
GONE_TAGS = ["dragWorkingNodes", "patchWorkingNodes", "deleteWorkingNodes", "connectWorkingPorts", "disconnectWorkingEdges", "addWorkingNode"]
FP = "semio_framework_plugin"
staged: dict = {}
deleted: list = []


def text(path: Path) -> str:
    return staged[path] if path in staged else path.read_text()


def replace(path: Path, old: str, new: str, count: int = 1) -> None:
    content = text(path)
    found = content.count(old)
    if found != count:
        sys.exit(f"{path}: expected {count} of {old[:100]!r}, found {found}")
    staged[path] = content.replace(old, new)


def mismatch(detail: str) -> str:
    return f'Fault::new({FP}::FaultOrigin::App, {FP}::FaultCode::new("app.command.tool-mismatch"), "{detail}")'


CAPACITY = f'Fault::new({FP}::FaultOrigin::App, {FP}::FaultCode::new("trinity.rewriting.retained-capacity"), "the rewriting command exceeds the capacity of one bounded edit")'

#region editor
ED = E / "🦀️.rs"
replace(ED, """    Ok(graph.to_snapshot())
}
/// 🧩️ One semantic rule-graph node""", """    Ok(graph.to_snapshot())
}
/// 🧾️ The working graph's resolved manifest (embedded, else named by `manifestId`); `None` when it resolves to none.
pub(crate) fn resolved_working_manifest(state: &RewritingSnapshot) -> Option<semio_s_artifact_trinity_jack::Manifest> {
    let mut graph = state.working_graph.clone();
    graph.resolve_manifest().ok().map(|()| graph.manifest.clone())
}
/// 🧩️ One semantic rule-graph node""")
replace(ED, """    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TrinityRewritingPlayApp>>>,
    operation: &semio_framework_plugin::AppOperationContext,
) -> Result<Emit<RewriteRuleMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    use crate::editor::rewriting::commands;
    Ok(match command {
        TrinityRewritingCommand::NodeGraphEdit { surface_id, operations_json } => commands::node_graph_edit(state, surface_id, operations_json, &operation.authoring_seed)?,""", f"""    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TrinityRewritingPlayApp>>>,
    operation: &semio_framework_plugin::AppOperationContext,
) -> Result<Emit<RewriteRuleMutation, NoConfigMutation, NoDraftMutation>, Fault> {{
    use crate::editor::rewriting::commands;
    let children = || context.map(|context| context.children.as_ref()).ok_or_else(|| Fault::new({FP}::FaultOrigin::App, {FP}::FaultCode::new("rewriting.child-refused"), "the retained document command carries no child view"));
    Ok(match command {{
        TrinityRewritingCommand::NodeGraphEdit {{ surface_id, operations_json }} => commands::node_graph_edit(state, children()?, surface_id, operations_json, &operation.authoring_seed)?,""")
replace(ED, """        TrinityRewritingCommand::PatchNodes { node_ids, field, value } => commands::patch_nodes(state, node_ids, interaction.selection.get("graph").map_or(&[][..], |selection| selection.ids.as_slice()), field, value)?,
        TrinityRewritingCommand::Reorganize => commands::reorganize(state),
        TrinityRewritingCommand::AddWorkingNode { kind, name, x, y } => commands::add_working_node_command(state, kind.as_deref(), name.as_deref(), *x, *y)?,
        _ => return Err(Fault::from("rewriting-document-command-route-mismatch")),""", f"""        TrinityRewritingCommand::PatchNodes {{ node_ids, field, value }} => commands::patch_nodes(state, children()?, node_ids, interaction.selection.get("graph").map_or(&[][..], |selection| selection.ids.as_slice()), field, value)?,
        TrinityRewritingCommand::Reorganize => commands::reorganize(state),
        TrinityRewritingCommand::AddWorkingNode {{ kind, name, x, y }} => commands::add_working_node_command(state, children()?, kind.as_deref(), name.as_deref(), *x, *y)?,
        _ => return Err({mismatch("the rewriting document reducer received a command outside its tool roster")}),""")
replace(ED, """        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchNodes", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },""", """        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchNodes", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Child] },""")
replace(ED, """        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addWorkingNode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },""", """        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addWorkingNode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },""")
replace(ED, """    if tool_id != request.tool_id || rewriting_document_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
        return Err(Fault::from("rewriting-document-command-mismatch-or-capacity"));
    }""", f"""    if tool_id != request.tool_id {{
        return Err({mismatch("Rewriting command does not match its exact registered tool")});
    }}
    if rewriting_document_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {{
        return Err({CAPACITY});
    }}""")
replace(ED, """impl ArtifactEditor for TrinityRewritingPlayApp {
    type Snapshot = RewritingSnapshot;""", """impl ArtifactEditor for TrinityRewritingPlayApp {
    /// 🧩️ The roster the composed `s.stdio.semio` `workingGraph` child opens through (design §20.15).
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = RewritingSnapshot;""")
replace(ED, """    fn initial_snapshot() -> RewritingSnapshot {
        default_rule_state()
    }
""", """    fn initial_snapshot() -> RewritingSnapshot {
        default_rule_state()
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
        crate::content::genesis_working_child_pack(snapshot, slot, child_id)
    }

    /// 📢️ The localized notices of the editor's own refusal codes (design §20.12).
    fn fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
        crate::content::rewriting_fault_notices()
    }
""")
replace(ED, """                let fixture = after_fixture(doc.snapshot).map_err(|error| MediaError::Payload(port.to_string(), error))?;""", """                let state = crate::content::composed(doc.snapshot, &doc.children).map_err(|fault| MediaError::Payload(port.to_string(), fault.message))?;
                let fixture = after_fixture(&state).map_err(|error| MediaError::Payload(port.to_string(), error))?;""")
replace(ED, """            TrinityRewritingCommand::NodeGraphEdit { surface_id, operations_json } => crate::editor::rewriting::commands::node_graph_edit(state, surface_id, operations_json, "")?,""", """            TrinityRewritingCommand::NodeGraphEdit { surface_id, operations_json } => crate::editor::rewriting::commands::node_graph_edit(state, &doc.children, surface_id, operations_json, "")?,""")
replace(ED, """            TrinityRewritingCommand::PatchNodes { node_ids, field, value } => crate::editor::rewriting::commands::patch_nodes(state, node_ids, &interaction.selection("graph").ids, field, value)?,""", """            TrinityRewritingCommand::PatchNodes { node_ids, field, value } => crate::editor::rewriting::commands::patch_nodes(state, &doc.children, node_ids, &interaction.selection("graph").ids, field, value)?,""")
replace(ED, """            TrinityRewritingCommand::AddWorkingNode { kind, name, x, y } => crate::editor::rewriting::commands::add_working_node_command(state, kind.as_deref(), name.as_deref(), *x, *y)?,
        })""", """            TrinityRewritingCommand::AddWorkingNode { kind, name, x, y } => crate::editor::rewriting::commands::add_working_node_command(state, &doc.children, kind.as_deref(), name.as_deref(), *x, *y)?,
        })""")
replace(ED, """        let labels = semio_framework_plugin::resolve_labels::<crate::editor::rewriting::terminology::TrinityRewritingLabels>(view_state);
        let root = match body_key {
            TRINITY_REWRITING_PLAY_BODY_BEFORE => edit::windows::before::render(state, &window_config),
            TRINITY_REWRITING_PLAY_BODY_AFTER => edit::windows::after::render(state, &window_config),""", """        let labels = semio_framework_plugin::resolve_labels::<crate::editor::rewriting::terminology::TrinityRewritingLabels>(view_state);
        let composed = || crate::content::composed(state, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("trinity.rewriting.working-graph", fault.message));
        let root = match body_key {
            TRINITY_REWRITING_PLAY_BODY_BEFORE => edit::windows::before::render(&composed()?, &window_config),
            TRINITY_REWRITING_PLAY_BODY_AFTER => edit::windows::after::render(&composed()?, &window_config),""")
replace(ED, """            TRINITY_REWRITING_PLAY_BODY_ARTIFACT => crate::editor::rewriting::panels::document::render(state, config, labels,""", """            TRINITY_REWRITING_PLAY_BODY_ARTIFACT => crate::editor::rewriting::panels::document::render(&composed()?, config, labels,""")
replace(ED, """ let state=doc.snapshot;
 semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::inferences::topology::compute_topology(&state.working_graph)?;""", """ let composed=crate::content::composed(doc.snapshot,&doc.children).map_err(|fault|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,fault.message))?;let state=&composed;
 semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::inferences::topology::compute_topology(&state.working_graph)?;""")
WJ = E / "🪟️window/🎚️config/🧵️job/🦀️.rs"
replace(WJ, """        _ => Err(Fault::from("rewriting-window-command-route-mismatch")),""", f"""        _ => Err({mismatch("the rewriting window reducer received a command outside its tool roster")}),""")
replace(WJ, """    if TrinityRewritingPlayApp::command_id(&request.command) != request.tool_id || extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
        return Err(Fault::from("rewriting-window-command-mismatch-or-capacity"));
    }""", f"""    if TrinityRewritingPlayApp::command_id(&request.command) != request.tool_id {{
        return Err({mismatch("Rewriting command does not match its exact registered tool")});
    }}
    if extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {{
        return Err({CAPACITY});
    }}""")
#endregion editor

#region aggregate
MR = M / "🦀️.rs"
for mod, variant in zip(GONE_MODS, GONE_VARIANTS):
    replace(MR, f"pub use super::{mod}::{{{mod}, {variant}}};\n", "")
    replace(MR, f"    {variant}({variant}),\n", "")
replace(MR, """/// 📏️ The byte bounds of a working-graph id or name and of a node or edge kind the working-graph leaves carry.
pub(crate) const WORKING_ID_MAXIMUM_BYTES: usize = 512;
pub(crate) const WORKING_KIND_MAXIMUM_BYTES: usize = 256;

#[path="🕸️working/🦀️.rs"]
mod working;
pub(crate) use working::{add_working_graph_node,edit_working_graph_nodes,remove_working_graph_nodes,connect_working_graph_ports,remove_working_graph_edges,working_graph_is_valid};

""", "")
replace(MR, "//#region 🕸️WorkingGraph\n", "//#region 🔢️Offsets\n")
replace(MR, "//#endregion 🕸️WorkingGraph\n", "//#endregion 🔢️Offsets\n")
replace(MR, """
#[cfg(test)]
#[path = "🧪️tests/🧪️node-drag-history/🦀️.rs"]
mod node_drag_history_tests;
""", "")
replace(MR, "//! ♻️ `trinity.rewrite.rule` semantic mutation aggregate.\n//!\n//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf.\n", "//! ♻️ `trinity.rewrite.rule` semantic mutation aggregate — the parent lane edits the rule's own members.\n//!\n//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf. The working graph lives in the composed\n//! `workingGraph` child (`s.stdio.semio@v1/graph`); every relative working-graph edit is a child-lane leaf of that shared\n//! vocabulary (design §20.15), so no parent leaf reads the child — `edit-before-fixture` replaces the whole child handle.\n")
deleted += [M / "🕸️working", M / "🧪️tests/🧪️node-drag-history"]

BR = M / "💾️binary/🦀️.rs"
for mod, variant in zip(GONE_MODS, GONE_VARIANTS):
    replace(BR, f'    ("{variant}", super::{mod}::binary::BINARY_TAG),\n', "")
PR = M / "💾️binary/📡️.protocol.semio"
for kind, tag in zip(GONE_KINDS, (7, 8, 11, 12, 13, 14)):
    replace(PR, f"record {kind} tag={tag}\nfield payload bytes\n", "")
replace(PR, "record drag-rule-nodes tag=9\n", "record drag-rule-nodes tag=7\n")
replace(PR, "record set-rule-layout-points tag=10\n", "record set-rule-layout-points tag=8\n")
TR = M / "📝️text/🦀️.rs"
for mod, variant in zip(GONE_MODS, GONE_VARIANTS):
    replace(TR, f'    ("{variant}", super::{mod}::text::TEXT_OPCODE),\n', "")
G4 = M / "📝️text/🅰️.g4"
replace(G4, "line: editBeforeFixture | editLhs | editRhs | changeParameterBinding | removeParameterBinding | changeRuleLayoutPoint | removeRuleLayoutPoint | dragWorkingNodes | patchWorkingNodes | dragRuleNodes | setRuleLayoutPoints | deleteWorkingNodes | connectWorkingPorts | disconnectWorkingEdges | addWorkingNode ;", "line: editBeforeFixture | editLhs | editRhs | changeParameterBinding | removeParameterBinding | changeRuleLayoutPoint | removeRuleLayoutPoint | dragRuleNodes | setRuleLayoutPoints ;")
for line in ["dragWorkingNodes: 'drag-working-nodes' SP targets SP number SP number ;\n", "patchWorkingNodes: 'patch-working-nodes' SP targets SP key SP value ;\n", "deleteWorkingNodes: 'delete-working-nodes' SP targets ;\n", "connectWorkingPorts: 'connect-working-ports' SP key SP key SP key ;\n", "disconnectWorkingEdges: 'disconnect-working-edges' SP targets ;\n", "addWorkingNode: 'add-working-node' SP key SP key SP value SP number SP number ;\n"]:
    replace(G4, line, "")
GS = M / "📝️text/📖️.grammar.semio"
replace(GS, "line = edit-before-fixture / edit-lhs / edit-rhs / change-parameter-binding / remove-parameter-binding / change-rule-layout-point / remove-rule-layout-point / drag-working-nodes / patch-working-nodes / drag-rule-nodes / set-rule-layout-points / delete-working-nodes / connect-working-ports / disconnect-working-edges / add-working-node", "line = edit-before-fixture / edit-lhs / edit-rhs / change-parameter-binding / remove-parameter-binding / change-rule-layout-point / remove-rule-layout-point / drag-rule-nodes / set-rule-layout-points")
for line in ['drag-working-nodes = "drag-working-nodes" SP targets SP number SP number\n', 'patch-working-nodes = "patch-working-nodes" SP targets SP key SP value\n', 'delete-working-nodes = "delete-working-nodes" SP targets\n', 'connect-working-ports = "connect-working-ports" SP key SP key SP key\n', 'disconnect-working-edges = "disconnect-working-edges" SP targets\n', 'add-working-node = "add-working-node" SP key SP key SP value SP number SP number\n']:
    replace(GS, line, "")
EB = M / "📝️text/🔤️.ebnf"
replace(EB, """     | remove rule layout point
     | drag working nodes
     | patch working nodes
     | drag rule nodes
     | set rule layout points
     | delete working nodes
     | connect working ports
     | disconnect working edges
     | add working node ;""", """     | remove rule layout point
     | drag rule nodes
     | set rule layout points ;""")
for line in ["drag working nodes = 'drag-working-nodes', space, targets, space, number, space, number ;\n", "patch working nodes = 'patch-working-nodes', space, targets, space, key, space, value ;\n", "delete working nodes = 'delete-working-nodes', space, targets ;\n", "connect working ports = 'connect-working-ports', space, key, space, key, space, key ;\n", "disconnect working edges = 'disconnect-working-edges', space, targets ;\n", "add working node = 'add-working-node', space, key, space, key, space, value, space, number, space, number ;\n"]:
    replace(EB, line, "")
GQ = M / "🔗️.graphql"
for line in ["input DragWorkingNodesInput { targets: [String!]! dx: Binary64WordInput! dy: Binary64WordInput! }\n", "input PatchWorkingNodesInput { targets: [String!]! field: String! value: String! }\n", "input DeleteWorkingNodesInput { targets: [String!]! }\n", "input ConnectWorkingPortsInput { source: String! target: String! kind: String! }\n", "input DisconnectWorkingEdgesInput { targets: [String!]! }\n", "input AddWorkingNodeInput { id: String! kind: String! name: String! x: Float! y: Float! }\n"]:
    replace(GQ, line, "")
for tag, variant in zip(GONE_TAGS, GONE_VARIANTS):
    replace(GQ, f" {tag}: {variant}Input\n", "")
PT = M / "🛰️.proto"
for line in ["message DragWorkingNodes { repeated string targets = 1; semio.framework.value.Binary64Word dx = 2; semio.framework.value.Binary64Word dy = 3; }\n", "message PatchWorkingNodes { repeated string targets = 1; string field = 2; string value = 3; }\n", "message DeleteWorkingNodes { repeated string targets = 1; }\n", "message ConnectWorkingPorts { string source = 1; string target = 2; string kind = 3; }\n", "message DisconnectWorkingEdges { repeated string targets = 1; }\n", "message AddWorkingNode { string id = 1; string kind = 2; string name = 3; double x = 4; double y = 5; }\n"]:
    replace(PT, line, "")
for variant, mod, field in zip(GONE_VARIANTS, GONE_MODS, (8, 9, 12, 13, 14, 15)):
    replace(PT, f"  {variant} {mod} = {field};\n", "")
replace(PT, "  DragRuleNodes drag_rule_nodes = 10;\n", "  DragRuleNodes drag_rule_nodes = 8;\n")
replace(PT, "  SetRuleLayoutPoints set_rule_layout_points = 11;\n", "  SetRuleLayoutPoints set_rule_layout_points = 9;\n")
AJ = M / "🔣️.json"
for kind in GONE_KINDS:
    replace(AJ, f"""    {{
      "$ref": "https://json.schemas.assets.semio-tech.com/s/trinity/rewriting/mutation/{kind}/schema.json"
    }},
""" if kind != "add-working-node" else f""",
    {{
      "$ref": "https://json.schemas.assets.semio-tech.com/s/trinity/rewriting/mutation/{kind}/schema.json"
    }}
""", "" if kind != "add-working-node" else "\n")
AT = M / "🟦️.ts"
for directory, variant in zip(GONE, GONE_VARIANTS):
    replace(AT, f'import type {{ {variant} }} from "./{directory}/🟦️.ts";\n', "")
for tag, variant in zip(GONE_TAGS, GONE_VARIANTS):
    old = f'  | ({{ mutation: "{tag}" }} & {variant})\n' if variant != "AddWorkingNode" else f'\n  | ({{ mutation: "{tag}" }} & {variant});'
    replace(AT, old, "" if variant != "AddWorkingNode" else ";")
for directory, tag in (("🫳️drag-rule", (9, 7)), ("📍️set-rule-layout", (10, 8))):
    replace(M / directory / "🔣️.json", f'  "binaryTag": {tag[0]},\n', f'  "binaryTag": {tag[1]},\n')
SC = M / "🧪️tests/🔬️structural-correspondence/🦀️.rs"
for kind in GONE_KINDS:
    content = text(SC)
    start = content.index(f'    {{\n        let kind = "{kind}";\n')
    end = content.index("\n    }\n", start) + len("\n    }\n")
    staged[SC] = content[:start] + content[end:]
replace(SC, '        let kind = "drag-rule-nodes";\n        let aggregate_variant = "DragRuleNodes";\n        let directory = "🫳️drag-rule";\n        let binary_tag = 9;', '        let kind = "drag-rule-nodes";\n        let aggregate_variant = "DragRuleNodes";\n        let directory = "🫳️drag-rule";\n        let binary_tag = 7;')
replace(SC, '        let kind = "set-rule-layout-points";\n        let aggregate_variant = "SetRuleLayoutPoints";\n        let directory = "📍️set-rule-layout";\n        let binary_tag = 10;', '        let kind = "set-rule-layout-points";\n        let aggregate_variant = "SetRuleLayoutPoints";\n        let directory = "📍️set-rule-layout";\n        let binary_tag = 8;')
B64 = M / "🧪️tests/🔢️binary64-transport/🟦️.ts"
replace(B64, " * @see ../../✋️drag-working/🧬️schema/🔣️.json\n", " * @see ../../🫳️drag-rule/🧬️schema/🔣️.json\n")
for line in ['    "add-working-node /properties/x",\n', '    "add-working-node /properties/y",\n', '    "drag-working-nodes /properties/dx",\n', '    "drag-working-nodes /properties/dy",\n']:
    replace(B64, line, "")
replace(B64, "  expect(edited.length).toBe(5);\n  expect(checked).toBe(10);", "  expect(edited.length).toBe(3);\n  expect(checked).toBe(6);")
deleted += [M / directory for directory in GONE] + [F / directory for directory in GONE]
#endregion aggregate

#region retirement
RT = S / "🧬️schema/♻️retirement/🦀️.rs"
for line in ["            Self::DragWorkingNodes(value) => (value.targets, (value.dx, value.dy)).retirement(),\n", "            Self::PatchWorkingNodes(value) => (value.targets, (value.field, value.value)).retirement(),\n", "            Self::DeleteWorkingNodes(value) => value.targets.retirement(),\n", "            Self::ConnectWorkingPorts(value) => (value.source, (value.target, value.kind)).retirement(),\n", "            Self::DisconnectWorkingEdges(value) => value.targets.retirement(),\n", "            Self::AddWorkingNode(value) => (value.id, (value.kind, (value.name, (value.x, value.y)))).retirement(),\n"]:
    replace(RT, line, "")
RF = S / "🧬️schema/♻️retirement/🧫️fixtures/🔣️.json"
retirement = json.loads(text(RF))
kept = [row for row in retirement["mutations"] if row["value"]["mutation"] not in GONE_TAGS]
if len(retirement["mutations"]) - len(kept) != 6:
    sys.exit("retirement fixture: expected six working rows")
retirement["mutations"] = kept
staged[RF] = json.dumps(retirement, indent=2, ensure_ascii=False) + "\n"
RTS = S / "🧬️schema/♻️retirement/🧪️tests/🔬️document-retirement/🟦️.ts"
replace(RTS, '["editBeforeFixture","editLhs","editRhs","changeParameterBinding","removeParameterBinding","changeRuleLayoutPoint","removeRuleLayoutPoint","dragWorkingNodes","patchWorkingNodes","dragRuleNodes","setRuleLayoutPoints","deleteWorkingNodes","connectWorkingPorts","disconnectWorkingEdges","addWorkingNode"]', '["editBeforeFixture","editLhs","editRhs","changeParameterBinding","removeParameterBinding","changeRuleLayoutPoint","removeRuleLayoutPoint","dragRuleNodes","setRuleLayoutPoints"]')
#endregion retirement

#region snapshot-json
SJ = S / "🧬️schema/📸️snapshot/🔣️json/🦀️.rs"
replace(SJ, '"dragWorkingNodes"|"dragRuleNodes"=>&["targets","dx","dy"],"patchWorkingNodes"=>&["targets","field","value"],"setRuleLayoutPoints"=>&["points","cleared"],"deleteWorkingNodes"|"disconnectWorkingEdges"=>&["targets"],"connectWorkingPorts"=>&["source","target","kind"],"addWorkingNode"=>&["id","kind","name","x","y"],', '"dragRuleNodes"=>&["targets","dx","dy"],"setRuleLayoutPoints"=>&["points","cleared"],')
replace(SJ, '"dragWorkingNodes"|"dragRuleNodes"=>{for key in["dx","dy"]', '"dragRuleNodes"=>{for key in["dx","dy"]')
#endregion snapshot-json

#region oracles
OR = S / "🔮️oracles/🔣️.json"


def prune(node):
    if isinstance(node, list):
        return [prune(item) for item in node if not (item in GONE_KINDS or isinstance(item, dict) and (item.get("mutationId") in GONE_KINDS or item.get("id") in GONE_KINDS))]
    if isinstance(node, dict):
        return {key: prune(value) for key, value in node.items()}
    return node


oracles = json.loads(text(OR))
pruned = prune(oracles)
before, after = json.dumps(oracles), json.dumps(pruned)
if sum(before.count(f'"{kind}"') for kind in GONE_KINDS) == 0 or any(f'"{kind}"' in after for kind in GONE_KINDS):
    sys.exit("oracles: working kinds not fully pruned")
for host in pruned["oracleHostPackages"]:
    if host["package"] == "semio-s-artifact-trinity-jack":
        host["rationale"] = "The Rewriting scenario subject binds the complete declared child to its actual Jack handle and retains the computed owner."
staged[OR] = json.dumps(pruned, indent=2, ensure_ascii=False) + "\n"
#endregion oracles

#region root
RR = R / "🦀️.rs"
for mod in GONE_MODS:
    content = text(RR)
    start = content.index(f"                        #[path = \".\"]\n                        pub mod {mod} {{\n")
    end = content.index("\n                        }\n", start) + len("\n                        }\n")
    staged[RR] = content[:start] + content[end:]
replace(RR, "pub use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;\n", """pub use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🪆️content/🦀️.rs"]
pub mod content;
pub use content::rewriting_fault_notices;
""")
#endregion root

#region editor-docs
replace(ED, """/// shell. Each verb publishes granular `RewriteRuleMutation`s on the artifact lane except `resetRule`, a
/// host-applied `Effect::LoadDocument`.""", """/// shell. Each verb publishes granular `RewriteRuleMutation`s on the artifact lane — the working-graph verbs (`patchNodes`,
/// `addWorkingNode`, a working-canvas `nodeGraphEdit`) ONE edit of the composed `workingGraph` child (design §20.15) — except
/// `resetRule`, a host-applied `Effect::LoadDocument`.""")
READ = S / "🪆️content/👁️read/🦀️.rs"
replace(READ, "    let owner = children.typed_read::<SemioGraphSnapshot>(WORKING_CHILD_SLOT, &handle.target.artifact_id)?;", "    let owner = children.typed_read::<SemioGraphSnapshot>(WORKING_CHILD_SLOT, &handle.child_id)?;")
#endregion editor-docs

#region mutate-rewrite-1
MT = S / "🧪️tests/♻️mutate-rewrite-1"
MRS = MT / "🦀️.rs"
replace(MRS, '"remove-rule-layout-point", "drag-working-nodes", "patch-working-nodes", "drag-rule-nodes", "set-rule-layout-points", "delete-working-nodes", "connect-working-ports", "disconnect-working-edges", "add-working-node"];', '"remove-rule-layout-point", "drag-rule-nodes", "set-rule-layout-points"];')
replace(MRS, '            "edit-before-fixture" | "drag-working-nodes" | "patch-working-nodes" | "delete-working-nodes" | "connect-working-ports" | "disconnect-working-edges" | "add-working-node" => "workingGraph",', '            "edit-before-fixture" => "workingGraph",')
replace(MRS, """    fn relative_working(kind: &str) -> bool {
        matches!(kind, "drag-working-nodes" | "patch-working-nodes" | "delete-working-nodes" | "connect-working-ports" | "disconnect-working-edges" | "add-working-node")
    }

""", "")
replace(MRS, """            "drag-working-nodes" => "dragWorkingNodes", "patch-working-nodes" => "patchWorkingNodes",
            "drag-rule-nodes" => "dragRuleNodes", "set-rule-layout-points" => "setRuleLayoutPoints",
            "delete-working-nodes" => "deleteWorkingNodes", "connect-working-ports" => "connectWorkingPorts",
            "disconnect-working-edges" => "disconnectWorkingEdges", "add-working-node" => "addWorkingNode",
""", """            "drag-rule-nodes" => "dragRuleNodes", "set-rule-layout-points" => "setRuleLayoutPoints",
""")
replace(MRS, "            let publication = if relative_working(kind) { Some(owner::declared_publication(ctx, before.get())?) } else { None };\n", "")
replace(MRS, "            if let Some(input) = publication { input.publish(current.get_mut()); }\n", "")
replace(MRS, "//! ♻️ Full typed Rewriting scenario subject pairs explicit retained child and publication inputs.\n", "//! ♻️ Full typed Rewriting scenario subject pairs every declared parent with its explicit retained child input.\n")
OW = MT / "🪆️owner/🦀️.rs"
content = text(OW)
start = content.index("/// 🪆️ Explicit scenario execution authority binds a retained source and a distinct publication target.\n")
staged[OW] = content[:start].rstrip("\n") + "\n"
replace(OW, "use semio_repo_test_host::{parse_json, Context, Json};\n", "use semio_repo_test_host::{Context, Json};\n")
PY = MT / "🐍️.py"
replace(PY, '"remove-rule-layout-point", "drag-working-nodes", "patch-working-nodes", "drag-rule-nodes", "set-rule-layout-points", "delete-working-nodes", "connect-working-ports", "disconnect-working-edges", "add-working-node")', '"remove-rule-layout-point", "drag-rule-nodes", "set-rule-layout-points")')
replace(PY, '"removeRuleLayoutPoint", "dragWorkingNodes", "patchWorkingNodes", "dragRuleNodes", "setRuleLayoutPoints", "deleteWorkingNodes", "connectWorkingPorts", "disconnectWorkingEdges", "addWorkingNode")))', '"removeRuleLayoutPoint", "dragRuleNodes", "setRuleLayoutPoints")))')
replace(PY, 'WORKING = ("drag-working-nodes", "patch-working-nodes", "delete-working-nodes", "connect-working-ports", "disconnect-working-edges", "add-working-node")\n', "")
for name in ("endpoint", "publication_input"):
    content = text(PY)
    start = content.index(f"def {name}(")
    end = content.index("\ndef ", start + 1) + 1
    staged[PY] = content[:start] + content[end:]
content = text(PY)
start = content.index('    elif kind == "delete-working-nodes":\n')
end = content.index('    elif kind == "drag-rule-nodes":\n')
staged[PY] = content[:start] + content[end:]
replace(PY, "def apply_mutation(owner, mutation, replacement=None, publication=None):", "def apply_mutation(owner, mutation, replacement=None):")
replace(PY, """    if publication is not None:
        if kind not in WORKING or publication["source"] != owner[0]["workingGraph"]["content"]:
            raise AssertionError("publication authority belongs to this relative working mutation's source")
        parent["workingGraph"]["content"] = copy.deepcopy(publication["target"])
""", "")
replace(PY, '    if kind == "edit-before-fixture" or kind in WORKING:\n', '    if kind == "edit-before-fixture":\n')
replace(PY, 'expected = DOCUMENTS[kind][0] if kind in DOCUMENTS else "workingGraph" if kind in WORKING else "parameterBindings"', 'expected = DOCUMENTS[kind][0] if kind in DOCUMENTS else "parameterBindings"')
replace(PY, "        publication = publication_input(ctx, before) if kind in WORKING else None\n", "")
replace(PY, "        applied = apply_mutation(before, mutation, replacement, publication)\n", "        applied = apply_mutation(before, mutation, replacement)\n")
FE = MT / "🥒️.feature"
lines = text(FE).split("\n")
kept_lines = [line for line in lines if not any(line.startswith(f"      | {kind} ") for kind in GONE_KINDS)]
if len(lines) - len(kept_lines) != 18:
    sys.exit(f"feature: expected 18 working example rows, found {len(lines) - len(kept_lines)}")
staged[FE] = "\n".join(kept_lines)
replace(FE, "    And the child publication input <publication>\n", "")
replace(FE, "| fixture                                       | publication |\n", "| fixture                                       |\n")
replace(FE, " | not-required |\n", " |\n", 9)
replace(FE, "  The document and all fifteen mutation payloads use their closed schemas.", "  The document and all nine mutation payloads use their closed schemas.")
replace(FE, "  `🔌️jack` sibling, all fifteen of them are ACCEPTING,", "  `🔌️jack` sibling, all nine of them are ACCEPTING,")
replace(FE, "  committed evidence; what the real-document rows add is the same fifteen verbs against a rule whose", "  committed evidence; what the real-document rows add is the same nine verbs against a rule whose")
replace(FE, """  declares its replacement child. No handler discovers a sibling input, reconstructs a reduced
  node/edge carrier or parses structured rule meaning from JSON strings. Relative graph edits
  retain every unrelated node, port, edge, intrinsic property and raw geometry word.""", """  declares its replacement child. No handler discovers a sibling input, reconstructs a reduced
  node/edge carrier or parses structured rule meaning from JSON strings. Relative working-graph
  edits are child-lane leaves of the shared Semio graph vocabulary (design §20.15), owned by its
  own cases, never a rewriting parent leaf.""")
replace(FE, "  Both implementations assert that each verb writes exactly one of the five semantic members.\n  Working-graph mutations include the retained full child in that comparison.", "  Both implementations assert that each verb writes exactly one of the five semantic members.\n  The working-graph replacement includes the retained full child in that comparison.")
#endregion mutate-rewrite-1

#region sqlite-ts
SQ = S / "🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"
replace(SQ, "expect(files.length).toBe(30);expect(files).toContain('➕️add-working/➕️adds/📸️snapshot/⬅️before/🔣️.json');expect(files).toContain('➕️add-working/➕️adds/📸️snapshot/➡️after/🔣️.json');", "expect(files.length).toBe(18);expect(files).toContain('🫳️drag-rule/🫳️moves/📸️snapshot/⬅️before/🔣️.json');expect(files).toContain('🫳️drag-rule/🫳️moves/📸️snapshot/➡️after/🔣️.json');")
replace(SQ, 'test("Rewriting committed working mutations retain exact typed child graph facts",()=>{', 'test("Rewriting committed mutation children are exact typed graphs whose edges join held nodes",()=>{')
replace(SQ, " expect(files.length).toBe(30);\n const ajv=semioSchemaAjvV1({allErrors:true}),validate=ajv.compile(JSON.parse(readFileSync(new URL(\"../../🌳️typed/🪆️child/🔣️.json\",import.meta.url),\"utf8\")));", " expect(files.length).toBe(18);\n const ajv=semioSchemaAjvV1({allErrors:true}),validate=ajv.compile(JSON.parse(readFileSync(new URL(\"../../🌳️typed/🪆️child/🔣️.json\",import.meta.url),\"utf8\")));")
content = text(SQ)
start = content.index(' const read=(slug:string,direction:string)=>JSON.parse(readFileSync(new URL(slug+"/📸️snapshot/"+direction+"/🪆️child/🔣️.json",root),"utf8"));\n')
end = content.index("\n});\n", start) + 1
staged[SQ] = content[:start] + content[end:]
for title in ('test("Rewriting working inverses restore the full typed Jack owner through independent SQL",async()=>{\n', 'test("Rewriting six relative specification executions declare closed child publication authority",async()=>{\n'):
    content = text(SQ)
    start = content.index(title)
    end = content.index("\n});\n", start) + len("\n});\n")
    tail = content[end:]
    staged[SQ] = content[:start] + (tail[1:] if tail.startswith("\n") else tail)
replace(SQ, 'test("Rewriting exact forty-six scenario roster owns all thirty closed typed inline payloads",async()=>{', 'test("Rewriting exact twenty-eight scenario roster owns all eighteen closed typed inline payloads",async()=>{')
replace(SQ, ' const kinds=["edit-before-fixture","edit-lhs","edit-rhs","change-parameter-binding","remove-parameter-binding","change-rule-layout-point","remove-rule-layout-point","drag-working-nodes","patch-working-nodes","drag-rule-nodes","set-rule-layout-points","delete-working-nodes","connect-working-ports","disconnect-working-edges","add-working-node"];', ' const kinds=["edit-before-fixture","edit-lhs","edit-rhs","change-parameter-binding","remove-parameter-binding","change-rule-layout-point","remove-rule-layout-point","drag-rule-nodes","set-rule-layout-points"];')
replace(SQ, " expect(schemas.length).toBe(15);", " expect(schemas.length).toBe(9);", 2)
replace(SQ, ' expect(files.length).toBe(15);\n const db=new Database(":memory:");', ' expect(files.length).toBe(9);\n const db=new Database(":memory:");')
replace(SQ, 'test("Rewriting published mutation union accepts all fifteen typed committed inputs",()=>{', 'test("Rewriting published mutation union accepts all nine typed committed inputs",()=>{')
replace(SQ, ' expect(files.length).toBe(15);\n for(const path of files){const input=', ' expect(files.length).toBe(9);\n for(const path of files){const input=')
replace(SQ, 'expect(mutations.length).toBe(15);', 'expect(mutations.length).toBe(9);')
replace(SQ, '.getFields()).length).toBe(15);', '.getFields()).length).toBe(9);')
replace(SQ, " expect(inline.length).toBe(30);", " expect(inline.length).toBe(18);")
replace(SQ, 'test("Rewriting all forty-six scenario owners declare complete before after and replacement child inputs",async()=>{', 'test("Rewriting all twenty-eight scenario owners declare complete before after and replacement child inputs",async()=>{')
replace(SQ, " expect(feature.errors).toEqual([]);expect(feature.scenarios.length).toBe(46);", " expect(feature.errors).toEqual([]);expect(feature.scenarios.length).toBe(28);")
deleted.append(S / "🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🪆️publication")
#endregion sqlite-ts

#region viewer
VW = S / "👁️viewer/🦀️.rs"
replace(VW, """impl ArtifactViewer for TrinityRewritingViewer {
    type Snapshot = RewritingSnapshot;""", """impl ArtifactViewer for TrinityRewritingViewer {
    /// 🧩️ The same member roster the editor declares — see `TrinityRewritingPlayApp`'s `Members`.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = RewritingSnapshot;""")
replace(VW, """    const DOCUMENT_SCHEMA: &'static str = REWRITE_RULE_SCHEMA;

    /// 🔐️ The artifact's own document-store owner catalogue""", """    const DOCUMENT_SCHEMA: &'static str = REWRITE_RULE_SCHEMA;

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
        crate::content::genesis_working_child_pack(snapshot, slot, child_id)
    }

    /// 🔐️ The artifact's own document-store owner catalogue""")
#endregion viewer

#region editor-tests
UT = E / "🧪️tests/🔬️unit/🦀️.rs"


def swap_fn(path: Path, name: str, new: str) -> None:
    content = text(path)
    lines = content.split("\n")
    at = [index for index, line in enumerate(lines) if re.match(rf"^(pub(\(crate\))? )?(async )?fn {re.escape(name)}\(", line)]
    if len(at) != 1:
        sys.exit(f"{path}: expected one fn {name}, found {len(at)}")
    start = at[0]
    while start > 0 and (lines[start - 1].startswith("///") or lines[start - 1].startswith("#[")):
        start -= 1
    end = at[0]
    while lines[end] != "}":
        end += 1
    staged[path] = "\n".join(lines[:start] + new.rstrip("\n").split("\n") + lines[end + 1:])


replace(UT, """semio_framework_plugin::history_edit_acceptance_law!("trinity", TrinityRewritingPlayApp, trinity_rewriting_manifest_for_tests, "../..");
""", """semio_framework_plugin::history_edit_acceptance_law!("trinity", TrinityRewritingPlayApp, trinity_rewriting_manifest_for_tests, "../..");
semio_framework_plugin::composed_reload_law!("trinity", TrinityRewritingPlayApp, trinity_rewriting_manifest_for_tests, "../..");
semio_framework_plugin::composed_child_history_law!("trinity", TrinityRewritingPlayApp, trinity_rewriting_manifest_for_tests, [("patchNodes", r#"{"nodeIds":["7dc5b737-3b6b-4068-b315-b7bacc91c2e1"],"field":"name","value":"Renamed core"}"#)]);
""")
replace(UT, "    let mut app = artifact_app_laws::new_app_with_registry::<EditorApp<TrinityRewritingPlayApp>>(trinity_rewriting_manifest_for_tests).await;", "    let mut app = artifact_app_laws::new_app_with_registry_and_members::<EditorApp<TrinityRewritingPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(trinity_rewriting_manifest_for_tests).await;")
replace(UT, "pub(crate) struct RewritingTestApp(VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>>);", "pub(crate) struct RewritingTestApp(VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>);")
replace(UT, "    type Target = VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>>;", "    type Target = VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;")
swap_fn(UT, "export_media_graph_out_reflects_rule_applied_fixture", """#[semio_framework_async_macros::async_test]
async fn export_media_graph_out_reflects_rule_applied_fixture() {
    let mut app = new_app().await;
    let graph_out = app.export_media("graph:out").await.expect("graph:out export");
    let MediaPayload::Structured { json, .. } = graph_out.payload else { panic!("structured payload") };
    let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64");
    let fixture = <JackSnapshot as ArtifactPack>::decode_pack(&bytes).expect("decode pack");
    let expected = after_fixture(&composed_state(&app).await).expect("valid typed rule");
    assert_eq!(fixture.nodes().expect("valid retained Jack child").len(), expected.nodes().expect("valid retained Jack child").len());
}""")
swap_fn(UT, "working_graph_node_ids", """/// 🧸️ The live `workingGraph` member — the working graph's single truth (design §20.15), never the parent's genesis owner.
async fn live_working(app: &RewritingTestApp) -> semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
    use store::{ArtifactPack, SpaceMember};
    let snapshot = app.snapshot().expect("rewriting parent projection");
    let bytes = app.child_store(crate::content::WORKING_CHILD_SLOT, &snapshot.working_graph.content.child_id).await.expect("working child").document_pack_bytes().await.expect("working child pack");
    SemioGraphSnapshot::decode_pack(&bytes).expect("working child snapshot")
}

/// 🧩️ The parent projection composed with its live working child, as every reader sees the rule.
async fn composed_state(app: &RewritingTestApp) -> RewritingSnapshot {
    let mut state = app.snapshot().expect("rewriting parent projection");
    semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut state.working_graph.content, live_working(app).await);
    state
}

async fn working_node_ids(app: &RewritingTestApp) -> Vec<String> {
    live_working(app).await.nodes.iter().map(|node| node.id.value.clone()).collect()
}""")
swap_fn(UT, "working_graph_node_name", """async fn working_node_label(app: &RewritingTestApp, id: &str) -> String {
    live_working(app).await.nodes.iter().find(|node| node.id.value == id).map(|node| node.label.clone()).expect("node")
}

/// 🕹️ Dispatches one rail press and settles its retained job; a refusal surfaces as the fault.
async fn dispatch_rail(app: &mut RewritingTestApp, action: &str, args: &[(&str, &str)]) -> Result<artifact_app_laws::TypedOperationFixtureReceipt, Fault> {
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::Value::Object(args.iter().map(|(key, value)| ((*key).to_string(), semio_framework_pack_json::Value::String((*value).to_string()))).collect()));
    app.handle_action(action, Some(&args), &meta("local")).await?;
    artifact_app_laws::settle_registered_typed_operation(&mut app.0, REWRITING_TEST_INSTANCE).await
}

/// 🕸️ Dispatches one `nodeGraphEdit` batch on `surface` and settles it.
async fn graph_edit(app: &mut RewritingTestApp, surface: &str, rows: semio_framework_pack_json::Value) -> Result<artifact_app_laws::TypedOperationFixtureReceipt, Fault> {
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "surfaceId": surface, "operations": rows.to_string() }));
    app.handle_action("nodeGraphEdit", Some(&args), &meta("local")).await?;
    artifact_app_laws::settle_registered_typed_operation(&mut app.0, REWRITING_TEST_INSTANCE).await
}

/// 📜️ The edited history rows of the document.
async fn edit_rows(app: &mut RewritingTestApp) -> usize {
    use semio_framework_plugin::PluginApp;
    app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).count()
}

fn child_lane(receipt: &artifact_app_laws::TypedOperationFixtureReceipt) -> bool {
    receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Child)
}""")
swap_fn(UT, "patch_nodes_from_the_rail_patches_the_selection_or_the_listed_nodes", """/// ⚖️ LAW: `patchNodes` pressed from the rail with an EMPTY `nodeIds` patches the selected nodes of the
/// working graph, and a comma list in the text field names several nodes (S15: the verb "moved nothing") — each press ONE edit of
/// the composed `workingGraph` child (design §20.15).
#[semio_framework_async_macros::async_test]
async fn patch_nodes_from_the_rail_patches_the_selection_or_the_listed_nodes() {
    let mut app = new_app().await;
    let ids = working_node_ids(&app).await;
    select_graph(&mut app, &[&ids[0]]).await;
    let receipt = dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "S15 Selected")]).await.expect("an empty nodeIds patches the selection");
    assert!(child_lane(&receipt), "patchNodes edits the working child: lanes {:?}", receipt.lanes);
    assert_eq!(working_node_label(&app, &ids[0]).await, "S15 Selected");
    dispatch_rail(&mut app, "patchNodes", &[("nodeIds", &format!("{} {}", ids[0], ids[1])), ("field", "name"), ("value", "S15 Listed")]).await.expect("a listed nodeIds patches both nodes");
    assert_eq!((working_node_label(&app, &ids[0]).await, working_node_label(&app, &ids[1]).await), ("S15 Listed".to_string(), "S15 Listed".to_string()));
}""")
swap_fn(UT, "patch_nodes_refuses_what_it_cannot_apply", """/// ⚖️ LAW: a `patchNodes` that cannot move the document is refused by name — an unknown id is
/// `mutation.target-missing`, no id and no selection is `app.command.targets-required` (the precondition an agent,
/// which has no selection, meets by naming `nodeIds`), an unsupported field, an empty value or a kind the manifest does not
/// declare is `app.command.invalid-args` — and every refusal leaves the working child untouched.
#[semio_framework_async_macros::async_test]
async fn patch_nodes_refuses_what_it_cannot_apply() {
    let mut app = new_app().await;
    let first = working_node_ids(&app).await[0].clone();
    let before = live_working(&app).await;
    let refusal = |result: Result<artifact_app_laws::TypedOperationFixtureReceipt, Fault>| result.err().expect("refused").code.0;
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", "no-such-node"), ("field", "name"), ("value", "x")]).await), "mutation.target-missing");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("field", "name"), ("value", "x")]).await), "app.command.targets-required");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", first.as_str()), ("field", "colour"), ("value", "x")]).await), "app.command.invalid-args");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", first.as_str()), ("field", "kind"), ("value", " ")]).await), "app.command.invalid-args");
    assert_eq!(refusal(dispatch_rail(&mut app, "patchNodes", &[("nodeIds", first.as_str()), ("field", "kind"), ("value", "NoSuchKind")]).await), "app.command.invalid-args", "a kind the manifest does not declare is refused, not written");
    assert_eq!(live_working(&app).await, before, "every refusal leaves the working child untouched");
}""")
swap_fn(UT, "working_graph_positions", """async fn working_positions(app: &RewritingTestApp) -> Vec<(String, f64, f64)> {
    live_working(app).await.nodes.iter().map(|node| (node.id.value.clone(), node.position.x, node.position.y)).collect()
}""")
swap_fn(UT, "a_released_working_graph_drag_is_one_tool_transaction_of_one_relative_leaf", """/// ⚖️ LAW: a released working-graph drag (the hosts' `move` gesture record) is ONE edit of the composed `workingGraph` child holding
/// ONE relative `drag-nodes {targets, dx, dy}` (design §20.15) — every target moves by the offset, every other node stays — and ONE
/// undo restores the graph; two gestures are two rows, a release that moves nothing leaves zero trace, and a malformed record is
/// refused by name.
#[semio_framework_async_macros::async_test]
async fn a_released_working_graph_drag_is_one_tool_transaction_of_one_relative_leaf() {
    let mut app = new_app().await;
    let before = working_positions(&app).await;
    let ids = working_node_ids(&app).await;
    let (first, second) = (ids[0].as_str(), ids[1].as_str());
    let rows = edit_rows(&mut app).await;
    let receipt = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![drag_row("g-1", &[first, second], 24.0, -8.0)])).await.expect("a drag row is admitted");
    assert!(child_lane(&receipt), "the drag edits the working child: lanes {:?}", receipt.lanes);
    for ((id, x, y), (_, before_x, before_y)) in working_positions(&app).await.into_iter().zip(before.iter().cloned()) {
        let expected = if id == first || id == second { (before_x + 24.0, before_y - 8.0) } else { (before_x, before_y) };
        assert_eq!((x, y), expected, "node {id}");
    }
    assert_eq!(edit_rows(&mut app).await, rows + 1, "one drag, one row");
    graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![drag_row("g-2", &[first], 1.0, 0.0)])).await.expect("a second drag");
    assert_eq!(edit_rows(&mut app).await, rows + 2, "two gestures are two rows");
    graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![drag_row("g-3", &[first], 0.0, 0.0)])).await.expect("a zero drag");
    assert_eq!(edit_rows(&mut app).await, rows + 2, "a release that moved nothing leaves zero trace");
    history(&mut app, "undo").await;
    history(&mut app, "undo").await;
    assert_eq!(working_positions(&app).await, before, "the undo rows restore the working graph");
    let malformed = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::json!([{ "operation": "move", "gestureId": "g", "nodeIds": [], "dx": 1.0, "dy": 0.0 }])).await;
    assert!(malformed.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a malformed gesture record is refused by name");
}""")
replace(UT, '    let emit = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_LHS, &operations, "seed").expect("a rule-node drag");', '    let emit = crate::editor::rewriting::commands::node_graph_edit(&state, &semio_framework_plugin::app::ChildContentView::EMPTY, TRINITY_REWRITING_PLAY_SURFACE_LHS, &operations, "seed").expect("a rule-node drag");')
swap_fn(UT, "the_shared_node_graph_rows_map_to_intent_leaves", """/// ⚖️ LAW (fixture `🧫️fixtures/🧫️node-graph-edit-rows`): every accepted row of the shared node-graph record vocabulary maps to a
/// child-lane leaf on the working graph — `connect` draws ONE `create-edge` carrying the graph's edge kind and the `source->target`
/// id, `disconnect` cuts ONE `delete-edge`, each undone by ONE row — every refused row refuses the whole batch by name,
/// `setSlider`/`insertPort` are refused (the graph has neither), and a rule side draws or cuts no wire alone.
#[semio_framework_async_macros::async_test]
async fn the_shared_node_graph_rows_map_to_intent_leaves() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json")).expect("node-graph-edit-rows fixture");
    let mut app = new_app().await;
    let row = |value: &serde_json::Value| semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("corpus row");
    for refused in corpus["refused"].as_array().expect("refused rows") {
        let emit = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![row(&refused["row"])])).await;
        assert!(emit.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "refused row {} is refused by name", refused["id"]);
    }
    let before = live_working(&app).await;
    let (source_node, target_node) = (before.nodes[0].id.value.clone(), before.nodes[1].id.value.clone());
    let connect = semio_framework_pack_json::json!([{ "operation": "connect", "sourceNodeId": source_node, "sourcePortId": "out", "targetNodeId": target_node, "targetPortId": "in" }]);
    let drawn = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, connect.clone()).await.expect("a connect row");
    assert!(child_lane(&drawn), "the wire edits the working child: lanes {:?}", drawn.lanes);
    let wired = live_working(&app).await;
    let id = format!("{source_node}@out->{target_node}@in");
    assert_eq!(wired.edges.len(), before.edges.len() + 1, "one wire is drawn");
    assert!(wired.edges.iter().any(|edge| edge.id.value == id && edge.source.value == source_node && edge.target.value == target_node && edge.source_port.as_deref() == Some("out") && edge.target_port.as_deref() == Some("in") && edge.kind == before.edges[0].kind), "{:?}", wired.edges);
    history(&mut app, "undo").await;
    assert_eq!(live_working(&app).await, before, "one undo row restores the working graph");
    graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::json!([{ "operation": "disconnect", "synapseId": before.edges[0].id.value }])).await.expect("a disconnect row");
    let severed = live_working(&app).await;
    assert_eq!((severed.nodes.len(), severed.edges.len()), (before.nodes.len(), before.edges.len() - 1), "one wire is cut, every node stays");
    for rejected in [semio_framework_pack_json::json!({ "operation": "setSlider", "widgetId": "w", "value": 1.0 }), semio_framework_pack_json::json!({ "operation": "insertPort", "nodeId": source_node, "side": "input", "index": 0 })] {
        assert!(graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![rejected.clone()])).await.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "{rejected} is refused by name");
    }
    assert!(graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_LHS, connect).await.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a rule side draws no wire alone");
}""")
swap_fn(UT, "one_canvas_drag_is_one_history_row_labelled_from_its_leaf", """/// ⚖️ LAW: one canvas drag through the shell is ONE edit and ONE history row labelled from its `drag-nodes` leaf in every language,
/// and ONE undo moves the nodes back.
#[semio_framework_async_macros::async_test]
async fn one_canvas_drag_is_one_history_row_labelled_from_its_leaf() {
    use semio_framework_plugin::PluginApp;
    let mut app = new_app().await;
    let before = working_positions(&app).await;
    let ids = working_node_ids(&app).await;
    let rows_before = edit_rows(&mut app).await;
    graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::Value::Array(vec![drag_row("g-shell", &[ids[0].as_str(), ids[1].as_str()], 24.0, -8.0)])).await.expect("the drag is admitted");
    let rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).collect();
    assert_eq!(rows.len(), rows_before + 1, "one drag, one edit, one row");
    let row = rows.iter().max_by_key(|row| row.seq).expect("the drag's row");
    assert_eq!(row.mutations.len(), 1, "one relative leaf");
    assert_eq!(row.mutations[0].label.resolve(Terminology::Native, Locale::En), "Drag 2 nodes by (24, -8)");
    assert_eq!(row.mutations[0].label.resolve(Terminology::Native, Locale::De), "2 Knoten um (24; -8) ziehen");
    history(&mut app, "undo").await;
    assert_eq!(working_positions(&app).await, before, "one undo moves the nodes back");
}""")
swap_fn(UT, "a_deleted_working_graph_selection_is_relative_leaves", """/// ⚖️ LAW: a `delete` row on the working graph is ONE edit of the composed `workingGraph` child: the `delete-edge` of every wire
/// touching each named node, then its `delete-node`, then the `delete-edge` of every wire it names apart from those — a node the graph
/// does not hold is skipped, never a whole-graph write — and ONE undo restores the graph.
#[semio_framework_async_macros::async_test]
async fn a_deleted_working_graph_selection_is_relative_leaves() {
    let mut app = new_app().await;
    let before = live_working(&app).await;
    let touches = |edge: &semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphEdge, id: &str| edge.source.value == id || edge.target.value == id;
    let doomed = before.nodes.iter().map(|node| node.id.value.clone()).find(|id| before.edges.iter().any(|edge| touches(edge, id))).expect("a node with an edge");
    let touching = before.edges.iter().filter(|edge| touches(edge, &doomed)).count();
    let apart = before.edges.iter().find(|edge| !touches(edge, &doomed)).map(|edge| edge.id.value.clone());
    let synapses: Vec<String> = before.edges.iter().filter(|edge| touches(edge, &doomed)).take(1).map(|edge| edge.id.value.clone()).chain(apart.clone()).collect();
    let receipt = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, semio_framework_pack_json::json!([{ "operation": "delete", "nodeIds": [doomed, "absent"], "synapseIds": synapses }])).await.expect("a delete");
    assert!(child_lane(&receipt), "the delete edits the working child: lanes {:?}", receipt.lanes);
    let after = live_working(&app).await;
    assert!(after.nodes.iter().all(|node| node.id.value != doomed) && after.nodes.len() + 1 == before.nodes.len(), "only the named node the graph holds is gone");
    assert_eq!(after.edges.len() + touching + usize::from(apart.is_some()), before.edges.len(), "every edge touching it and the named wire apart are gone, every other edge stays");
    history(&mut app, "undo").await;
    assert_eq!(live_working(&app).await, before, "one undo row restores the working graph");
    let rule = graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_LHS, semio_framework_pack_json::json!([{ "operation": "delete", "nodeIds": [], "synapseIds": ["lhs-wire"] }])).await;
    assert!(rule.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a rule wire is cut with its clause, never alone");
}""")
swap_fn(UT, "the_add_node_verb_is_one_relative_leaf_and_canonical_graphs_undo_relatively", """/// ⚖️ LAW (audit T1/T2): the guest's own add-node verb is ONE `create-node` of the composed `workingGraph` child carrying the first
/// free `n<k>` id, the graph's node kind, the id as name and the requested position — it lands exactly that node, a second add
/// takes the next free id and its given name, a non-finite position is refused by name before any child is read — and ONE undo
/// removes the added node.
#[semio_framework_async_macros::async_test]
async fn the_add_node_verb_is_one_relative_leaf_and_canonical_graphs_undo_relatively() {
    let mut app = new_app().await;
    let before = live_working(&app).await;
    let add = |pairs: &[(&str, semio_framework_pack_json::Value)]| semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::Value::Object(pairs.iter().map(|(key, value)| ((*key).to_string(), value.clone())).collect()));
    app.handle_action("addWorkingNode", Some(&add(&[("x", semio_framework_pack_json::json!(40.0)), ("y", semio_framework_pack_json::json!(-20.0))])), &meta("local")).await.expect("an add");
    assert!(child_lane(&settle(&mut app).await), "the add edits the working child");
    let added = live_working(&app).await;
    assert_eq!(added.nodes.len(), before.nodes.len() + 1, "exactly one node lands");
    let first = added.nodes.iter().find(|node| before.nodes.iter().all(|held| held.id != node.id)).expect("the added node").clone();
    assert!(first.id.value.starts_with('n') && first.label == first.id.value && (first.position.x, first.position.y) == (40.0, -20.0), "{first:?}");
    app.handle_action("addWorkingNode", Some(&add(&[("kind", semio_framework_pack_json::json!(first.kind)), ("name", semio_framework_pack_json::json!("Second")), ("x", semio_framework_pack_json::json!(0.0)), ("y", semio_framework_pack_json::json!(0.0))])), &meta("local")).await.expect("a second add");
    settle(&mut app).await;
    let again = live_working(&app).await;
    let second = again.nodes.iter().find(|node| added.nodes.iter().all(|held| held.id != node.id)).expect("the second node");
    assert!(second.id != first.id && second.label == "Second", "{second:?}");
    let nan = crate::editor::rewriting::commands::add_working_node_command(&default_rule_state(), &semio_framework_plugin::app::ChildContentView::EMPTY, None, None, f64::NAN, 0.0);
    assert!(nan.is_err_and(|fault| fault.code.0 == "app.command.invalid-args"), "a non-finite position is refused by name");
    history(&mut app, "undo").await;
    assert_eq!(live_working(&app).await, added, "one undo removes the second node");
}""")
swap_fn(UT, "a_history_edit_of_a_binary64_offset_validates_and_replays", """/// ⚖️ LAW (Binary64 transport): a time-travel edit of a binary64 input — the `/dx` offset of a released rule-node drag's
/// `drag-rule-nodes` — validates against its leaf schema (`Binary64Transport`: the exact word or a plain number, the form
/// `payload_value` and the history editor carry) and replays: the overwrite moves the dragged clause by the edited offset.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_of_a_binary64_offset_validates_and_replays() {
    use semio_framework::kernel::HistoryTimeTravelStage;
    use semio_framework_plugin::PluginApp;
    let mut app = new_app().await;
    graph_edit(&mut app, TRINITY_REWRITING_PLAY_SURFACE_LHS, semio_framework_pack_json::Value::Array(vec![drag_row("g-history", &["lhs-match"], 24.0, -8.0)])).await.expect("the drag is admitted");
    assert_eq!(app.snapshot().expect("dragged").rule_layout.get("lhs-match"), Some(&LayoutPoint { x: 24.0, y: -8.0 }), "the match moves from its default slot");
    let rows = app.history_snapshot().await.expect("history").upserts;
    let mutation_id = rows.iter().filter(|row| row.edit_id.is_some()).max_by_key(|row| row.seq).and_then(|row| row.mutations.first()).map(|mutation| mutation.mutation_id.clone()).expect("the drag's leaf");
    async fn stage(app: &mut RewritingTestApp) -> Option<HistoryTimeTravelStage> {
        app.history_snapshot().await.expect("history").time_travel.map(|status| status.stage)
    }
    let verbs = [
        ("historyEditBegin", semio_framework_pack_json::json!({ "mutationId": mutation_id })),
        ("historyEditInput", semio_framework_pack_json::json!({ "path": "/dx", "value": 50.0 })),
        ("historyEditAccept", semio_framework_pack_json::json!({})),
    ];
    for (verb, args) in verbs {
        let result = app.handle_action(verb, Some(&semio_framework_pack_json::to_dsl_value(&args)), &meta("local")).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
        assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
    }
    for _ in 0..10_000 {
        if stage(&mut app).await != Some(HistoryTimeTravelStage::Replaying) {
            break;
        }
        app.0.advance_typed_operation_publication().await.expect("a driver turn");
        while app.0.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(stage(&mut app).await, Some(HistoryTimeTravelStage::Reviewing), "the edited offset replays to a clean review");
    for (verb, args) in [("historyEditFinalize", semio_framework_pack_json::json!({})), ("historyEditCommit", semio_framework_pack_json::json!({ "choice": "overwrite" }))] {
        let result = app.handle_action(verb, Some(&semio_framework_pack_json::to_dsl_value(&args)), &meta("local")).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
        assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
    }
    for _ in 0..10_000 {
        if stage(&mut app).await.is_none() {
            break;
        }
        app.0.advance_typed_operation_publication().await.expect("a driver turn");
        while app.0.take_typed_operation_ui_progress().is_some() {}
    }
    assert_eq!(app.snapshot().expect("edited head").rule_layout.get("lhs-match"), Some(&LayoutPoint { x: 50.0, y: -8.0 }), "the overwrite replays the edited offset");
}""")
CF = E / "🧪️tests/🪆️child-frame/🦀️.rs"
replace(CF, """ let current=app.snapshot().expect("published parent");
 let child=semio_s_artifact_trinity_jack::jack_content_for_handle(&current.working_graph.content).expect("actual published child authority");
 assert_eq!(child.snapshot().nodes[0].label,"edited !@/\\0引用😀");""", """ let view=app.test_child_content_view();
 let id=full_frame_contract()["childId"].as_str().expect("declared member key").to_owned();
 let child=view.typed_read::<SemioGraphSnapshot>("workingGraph",&id).expect("actual published child edit");
 assert_eq!(child.nodes[0].label,"edited !@/\\0引用😀");""")
for old, new in (("child.snapshot().nodes[0]", "child.nodes[0]"), ("child.snapshot().edges", "child.edges")):
    replace(CF, old, new, text(CF).count(old))
replace(CF, 'encode_semio_graph_snapshot_json(child.snapshot())', 'encode_semio_graph_snapshot_json(&child)')
replace(CF, " drop(child);artifact_app_laws::close_registered_fixture_app(&mut app);\n}", " drop(child);drop(view);artifact_app_laws::close_registered_fixture_app(&mut app);\n}")
#endregion editor-tests

if "--check" not in sys.argv:
    for path, content in staged.items():
        path.write_text(content)
    for path in deleted:
        shutil.rmtree(path)
print(f"rewriting child lane: {len(staged)} files staged, {len(deleted)} trees deleted ({'checked' if '--check' in sys.argv else 'written'})")
