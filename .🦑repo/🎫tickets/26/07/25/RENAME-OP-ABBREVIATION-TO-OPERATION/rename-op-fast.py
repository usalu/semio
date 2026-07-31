#!/usr/bin/env python3
"""Fast idempotent rename: operation abbreviations (op/Op/Ops) → long form."""

from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
TICKET = Path(__file__).resolve().parent
assert (ROOT / "vcs").is_dir(), ROOT

SKIP_DIRS = {
    ".git", ".repo", "node_modules", "target", "dist", "build", ".next",
    "coverage", "__pycache__", ".turbo", "out", ".vscode-test",
}
TEXT_SUFFIXES = {
    ".rs", ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".json", ".md", ".mdx",
    ".toml", ".wit", ".cs", ".go", ".py", ".rb", ".graphql", ".gql", ".yml",
    ".yaml", ".css", ".scss", ".html", ".svg", ".txt", ".snap", ".wgsl",
    ".glsl", ".sql", ".sh",
}

# (src, dst) longest-first. Op means Operation unless listed as Operator.
PAIRS: list[tuple[str, str]] = [
    # operators (Op → Operator)
    ("AssumeRelOp", "AssumeRelationalOperator"),
    ("BinaryOperator", "BinaryOperator"),  # idempotent anchor
    ("BinOp", "BinaryOperator"),
    ("RelationalOperator", "RelationalOperator"),
    ("RelOp", "RelationalOperator"),
    ("BooleanOperator", "BooleanOperator"),
    ("BooleanOp", "BooleanOperator"),
    # envelopes / wire
    ("OperationEnvelope", "OperationEnvelope"),
    ("OpEnvelope", "OperationEnvelope"),
    # long *Operation names first (idempotent), then *Op sources
    ("PresentationEditOperation", "PresentationEditOperation"),
    ("PresentationEditOp", "PresentationEditOperation"),
    ("applyPresentationEditOperation", "applyPresentationEditOperation"),
    ("applyPresentationEditOp", "applyPresentationEditOperation"),
    ("backwardsPresentationEditOperation", "backwardsPresentationEditOperation"),
    ("backwardsPresentationEditOp", "backwardsPresentationEditOperation"),
    ("diffPresentationEditOperation", "diffPresentationEditOperation"),
    ("diffPresentationEditOp", "diffPresentationEditOperation"),
    ("Puzzle2dLiveMirrorOperations", "Puzzle2dLiveMirrorOperations"),
    ("Puzzle2dLiveMirrorOps", "Puzzle2dLiveMirrorOperations"),
    ("collectPuzzle2dLiveMirrorOperations", "collectPuzzle2dLiveMirrorOperations"),
    ("collectPuzzle2dLiveMirrorOps", "collectPuzzle2dLiveMirrorOperations"),
    ("pushPuzzle2dLiveMirrorOperations", "pushPuzzle2dLiveMirrorOperations"),
    ("pushPuzzle2dLiveMirrorOps", "pushPuzzle2dLiveMirrorOperations"),
    ("Gis3dTerrainOperation", "Gis3dTerrainOperation"),
    ("Gis3dTerrainOp", "Gis3dTerrainOperation"),
    ("ModulePayloadOperation", "ModulePayloadOperation"),
    ("ModulePayloadOp", "ModulePayloadOperation"),
    ("MindmapWiresOperation", "MindmapWiresOperation"),
    ("MindmapWiresOp", "MindmapWiresOperation"),
    ("StudioHistoryOperation", "StudioHistoryOperation"),
    ("StudioHistoryOp", "StudioHistoryOperation"),
    ("RewriteRuleOperation", "RewriteRuleOperation"),
    ("RewriteRuleOp", "RewriteRuleOperation"),
    ("TimestampedOperation", "TimestampedOperation"),
    ("TimestampedOp", "TimestampedOperation"),
    ("TrinityGraphOperation", "TrinityGraphOperation"),
    ("TrinityGraphOp", "TrinityGraphOperation"),
    ("Procedural2dOperation", "Procedural2dOperation"),
    ("Procedural2dOp", "Procedural2dOperation"),
    ("Procedural3dOperation", "Procedural3dOperation"),
    ("Procedural3dOp", "Procedural3dOperation"),
    ("SetDocumentOperation", "SetDocumentOperation"),
    ("SetDocumentOp", "SetDocumentOperation"),
    ("GenerationOperation", "GenerationOperation"),
    ("GenerationOp", "GenerationOperation"),
    ("CollectionOperation", "CollectionOperation"),
    ("CollectionOp", "CollectionOperation"),
    ("ImperativeOperation", "ImperativeOperation"),
    ("ImperativeOp", "ImperativeOperation"),
    ("DrawBooleanOperation", "DrawBooleanOperation"),
    ("DrawBooleanOp", "DrawBooleanOperation"),
    ("DRAW_BOOLEAN_OPERATIONS", "DRAW_BOOLEAN_OPERATIONS"),
    ("DRAW_BOOLEAN_OPS", "DRAW_BOOLEAN_OPERATIONS"),
    ("Process3dOperation", "Process3dOperation"),
    ("Process3dOp", "Process3dOperation"),
    ("Puzzle2dOperation", "Puzzle2dOperation"),
    ("Puzzle2dOp", "Puzzle2dOperation"),
    ("Puzzle3dOperation", "Puzzle3dOperation"),
    ("Puzzle3dOp", "Puzzle3dOperation"),
    ("Puzzle5dOperation", "Puzzle5dOperation"),
    ("Puzzle5dOp", "Puzzle5dOperation"),
    ("VcsDemoOperation", "VcsDemoOperation"),
    ("VcsDemoOp", "VcsDemoOperation"),
    ("SourcingOperation", "SourcingOperation"),
    ("SourcingOp", "SourcingOperation"),
    ("SequenceOperation", "SequenceOperation"),
    ("SequenceOp", "SequenceOperation"),
    ("ProtocolOperation", "ProtocolOperation"),
    ("ProtocolOp", "ProtocolOperation"),
    ("ShootingOperation", "ShootingOperation"),
    ("ShootingOp", "ShootingOperation"),
    ("PresentOperation", "PresentOperation"),
    ("PresentOp", "PresentOperation"),
    ("ProgramOperation", "ProgramOperation"),
    ("ProgramOp", "ProgramOperation"),
    ("RemodelOperation", "RemodelOperation"),
    ("RemodelOp", "RemodelOperation"),
    ("LowpolyOperation", "LowpolyOperation"),
    ("LowpolyOp", "LowpolyOperation"),
    ("LayoutOperation", "LayoutOperation"),
    ("LayoutOp", "LayoutOperation"),
    ("RasterOperation", "RasterOperation"),
    ("RasterOp", "RasterOperation"),
    ("WriterOperation", "WriterOperation"),
    ("WriterOp", "WriterOperation"),
    ("DiffLineOperation", "DiffLineOperation"),
    ("DiffLineOp", "DiffLineOperation"),
    ("GisMapOperation", "GisMapOperation"),
    ("GisMapOp", "GisMapOperation"),
    ("StudioOperation", "StudioOperation"),
    ("StudioOp", "StudioOperation"),
    ("SHomeOperation", "SHomeOperation"),
    ("SHomeOp", "SHomeOperation"),
    ("Fem2dOperation", "Fem2dOperation"),
    ("Fem2dOp", "Fem2dOperation"),
    ("Fem3dOperation", "Fem3dOperation"),
    ("Fem3dOp", "Fem3dOperation"),
    ("DummyOperation", "DummyOperation"),
    ("DummyOp", "DummyOperation"),
    ("DemoOperation", "DemoOperation"),
    ("DemoOp", "DemoOperation"),
    ("TestOperation", "TestOperation"),
    ("TestOp", "TestOperation"),
    ("MathOperation", "MathOperation"),
    ("MathOp", "MathOperation"),
    ("FlowOperation", "FlowOperation"),
    ("FlowOp", "FlowOperation"),
    ("FormOperation", "FormOperation"),
    ("FormOp", "FormOperation"),
    ("NoteOperation", "NoteOperation"),
    ("NoteOp", "NoteOperation"),
    ("CadOperation", "CadOperation"),
    ("CadOp", "CadOperation"),
    ("DagOperation", "DagOperation"),
    ("DagOp", "DagOperation"),
    ("KitOperation", "KitOperation"),
    ("KitOp", "KitOperation"),
    ("OsOperation", "OsOperation"),
    ("OsOp", "OsOperation"),
    ("ShellOperation", "ShellOperation"),
    ("ShellOp", "ShellOperation"),
    ("StoreOperation", "StoreOperation"),
    ("StoreOp", "StoreOperation"),
    ("LoadOperation", "LoadOperation"),
    ("LoadOp", "LoadOperation"),
    ("DocumentOperation", "DocumentOperation"),
    ("DocumentOp", "DocumentOperation"),
    ("AppendedOperation", "AppendedOperation"),
    ("AppendedOp", "AppendedOperation"),
    ("AppendOperations", "AppendOperations"),
    ("AppendOps", "AppendOperations"),
    ("RemoteOperations", "RemoteOperations"),
    ("RemoteOps", "RemoteOperations"),
    ("LocalOperations", "LocalOperations"),
    ("LocalOps", "LocalOperations"),
    ("ReplayGoldenOperations", "ReplayGoldenOperations"),
    ("ReplayGoldenOps", "ReplayGoldenOperations"),
    ("replayGoldenOperations", "replayGoldenOperations"),
    ("replayGoldenOps", "replayGoldenOperations"),
    ("processPluginOperations", "processPluginOperations"),
    ("processPluginOps", "processPluginOperations"),
    ("pendingOperations", "pendingOperations"),
    ("pendingOps", "pendingOperations"),
    ("remoteOperations", "remoteOperations"),
    ("remoteOps", "remoteOperations"),
    ("localOperations", "localOperations"),
    ("localOps", "localOperations"),
    ("PortKindNotDeclaredOnOperation", "PortKindNotDeclaredOnOperation"),
    ("PortKindNotDeclaredOnOp", "PortKindNotDeclaredOnOperation"),
    ("SetBooleanOperation", "SetBooleanOperation"),
    ("SetBooleanOp", "SetBooleanOperation"),
    ("FieldOperations", "FieldOperations"),
    ("FieldOps", "FieldOperations"),
    ("NodeOperations", "NodeOperations"),
    ("NodeOps", "NodeOperations"),
    ("EdgeOperations", "EdgeOperations"),
    ("EdgeOps", "EdgeOperations"),
    ("EditOperations", "EditOperations"),
    ("EditOps", "EditOperations"),
    ("GraphOperations", "GraphOperations"),
    ("GraphOps", "GraphOperations"),
    ("ContentOperations", "ContentOperations"),
    ("ContentOps", "ContentOperations"),
    ("CloudOperations", "CloudOperations"),
    ("CloudOps", "CloudOperations"),
    ("ShapeOperations", "ShapeOperations"),
    ("ShapeOps", "ShapeOperations"),
    ("StyleOperations", "StyleOperations"),
    ("StyleOps", "StyleOperations"),
    ("PathOperations", "PathOperations"),
    ("PathOps", "PathOperations"),
    ("TextOperations", "TextOperations"),
    ("TextOps", "TextOperations"),
    ("XformOperations", "XformOperations"),
    ("XformOps", "XformOperations"),
    ("GroupOperations", "GroupOperations"),
    ("GroupOps", "GroupOperations"),
    ("ClipOperations", "ClipOperations"),
    ("ClipOps", "ClipOperations"),
    ("BoolOperations", "BoolOperations"),
    ("BoolOps", "BoolOperations"),
    ("GradientOperations", "GradientOperations"),
    ("GradientOps", "GradientOperations"),
    ("AlgebraicOperations", "AlgebraicOperations"),
    ("AlgebraicOps", "AlgebraicOperations"),
    ("CertifiedOperations", "CertifiedOperations"),
    ("CertifiedOps", "CertifiedOperations"),
    ("BitOperations", "BitOperations"),
    ("BitOps", "BitOperations"),
    ("applyOperation", "applyOperation"),
    ("applyOp", "applyOperation"),
    ("repoOperation", "repoOperation"),
    ("repoOp", "repoOperation"),
    ("booleanOperation", "booleanOperation"),
    ("booleanOp", "booleanOperation"),
    ("binaryOperator", "binaryOperator"),
    ("binOp", "binaryOperator"),
    ("operationEnvelope", "operationEnvelope"),
    ("opEnvelope", "operationEnvelope"),
    ("operationId", "operationId"),
    ("opId", "operationId"),
    ("operation_id", "operation_id"),
    ("op_id", "operation_id"),
    ("operation_envelope_from_stored_edit", "operation_envelope_from_stored_edit"),
    ("op_envelope_from_stored_edit", "operation_envelope_from_stored_edit"),
    ("operation_envelope_from_edit", "operation_envelope_from_edit"),
    ("op_envelope_from_edit", "operation_envelope_from_edit"),
    ("edit_from_operation_envelope", "edit_from_operation_envelope"),
    ("edit_from_op_envelope", "edit_from_operation_envelope"),
    ("foreign_operation_envelope", "foreign_operation_envelope"),
    ("foreign_op_envelope", "foreign_operation_envelope"),
    ("sample_operation_envelope", "sample_operation_envelope"),
    ("sample_op_envelope", "sample_operation_envelope"),
    ("deliver_remote_operations", "deliver_remote_operations"),
    ("deliver_remote_ops", "deliver_remote_operations"),
    ("persist_operations", "persist_operations"),
    ("persist_ops", "persist_operations"),
    ("relay_operations_to_hub", "relay_operations_to_hub"),
    ("relay_ops_to_hub", "relay_operations_to_hub"),
    ("apply_operations", "apply_operations"),
    ("apply_ops", "apply_operations"),
    ("append_operations", "append_operations"),
    ("append_ops", "append_operations"),
    ("host_operations", "host_operations"),
    ("host_ops", "host_operations"),
    ("golden_operations", "golden_operations"),
    ("golden_ops", "golden_operations"),
    ("pending_operations", "pending_operations"),
    ("pending_ops", "pending_operations"),
    ("follow_up_operations", "follow_up_operations"),
    ("follow_up_ops", "follow_up_operations"),
    ("generation_operations", "generation_operations"),
    ("generation_ops", "generation_operations"),
    ("edit_operations", "edit_operations"),
    ("edit_ops", "edit_operations"),
    ("load_operations", "load_operations"),
    ("load_ops", "load_operations"),
    ("sub_operations", "sub_operations"),
    ("sub_ops", "sub_operations"),
    ("depth_operations", "depth_operations"),
    ("depth_ops", "depth_operations"),
    ("stencil_operations", "stencil_operations"),
    ("stencil_ops", "stencil_operations"),
    ("path_operations", "path_operations"),
    ("path_ops", "path_operations"),
    ("collection_operation", "collection_operation"),
    ("collection_op", "collection_operation"),
    ("invert_collection_operation", "invert_collection_operation"),
    ("invert_collection_op", "invert_collection_operation"),
    ("apply_collection_operation", "apply_collection_operation"),
    ("apply_collection_op", "apply_collection_operation"),
    ("collection_diff_from_operation", "collection_diff_from_operation"),
    ("collection_diff_from_op", "collection_diff_from_operation"),
    ("apply_remodel_operation", "apply_remodel_operation"),
    ("apply_remodel_op", "apply_remodel_operation"),
    ("apply_generation_operation", "apply_generation_operation"),
    ("apply_generation_op", "apply_generation_operation"),
    ("invert_generation_operation", "invert_generation_operation"),
    ("invert_generation_op", "invert_generation_operation"),
    ("apply_program_operation", "apply_program_operation"),
    ("apply_program_op", "apply_program_operation"),
    ("invert_program_operation", "invert_program_operation"),
    ("invert_program_op", "invert_program_operation"),
    ("apply_trinity_graph_operations", "apply_trinity_graph_operations"),
    ("apply_trinity_graph_ops", "apply_trinity_graph_operations"),
    ("dispatch_trinity_graph_operations", "dispatch_trinity_graph_operations"),
    ("dispatch_trinity_graph_ops", "dispatch_trinity_graph_operations"),
    ("validate_trinity_graph_operation", "validate_trinity_graph_operation"),
    ("validate_trinity_graph_op", "validate_trinity_graph_operation"),
    ("apply_protocol_edit_operation", "apply_protocol_edit_operation"),
    ("apply_protocol_edit_op", "apply_protocol_edit_operation"),
    ("apply_draw_edit_operation", "apply_draw_edit_operation"),
    ("apply_draw_edit_op", "apply_draw_edit_operation"),
    ("apply_form_edit_operation", "apply_form_edit_operation"),
    ("apply_form_edit_op", "apply_form_edit_operation"),
    ("apply_puzzle2d_operation", "apply_puzzle2d_operation"),
    ("apply_puzzle2d_op", "apply_puzzle2d_operation"),
    ("apply_puzzle3d_operation", "apply_puzzle3d_operation"),
    ("apply_puzzle3d_op", "apply_puzzle3d_operation"),
    ("apply_puzzle5d_operation", "apply_puzzle5d_operation"),
    ("apply_puzzle5d_op", "apply_puzzle5d_operation"),
    ("apply_lowpoly_operation", "apply_lowpoly_operation"),
    ("apply_lowpoly_op", "apply_lowpoly_operation"),
    ("puzzle2d_document_delta_operations", "puzzle2d_document_delta_operations"),
    ("puzzle2d_document_delta_ops", "puzzle2d_document_delta_operations"),
    ("puzzle3d_document_delta_operations", "puzzle3d_document_delta_operations"),
    ("puzzle3d_document_delta_ops", "puzzle3d_document_delta_operations"),
    ("puzzle5d_document_delta_operations", "puzzle5d_document_delta_operations"),
    ("puzzle5d_document_delta_ops", "puzzle5d_document_delta_operations"),
    ("procedural2d_fixture_operations", "procedural2d_fixture_operations"),
    ("procedural2d_fixture_ops", "procedural2d_fixture_operations"),
    ("procedural3d_fixture_operations", "procedural3d_fixture_operations"),
    ("procedural3d_fixture_ops", "procedural3d_fixture_operations"),
    ("flow_fixture_operations", "flow_fixture_operations"),
    ("flow_fixture_ops", "flow_fixture_operations"),
    ("sequence_fixture_operations", "sequence_fixture_operations"),
    ("sequence_fixture_ops", "sequence_fixture_operations"),
    ("vcs_demo_projection_diff_operations", "vcs_demo_projection_diff_operations"),
    ("vcs_demo_projection_diff_ops", "vcs_demo_projection_diff_operations"),
    ("try_commit_session_operations", "try_commit_session_operations"),
    ("try_commit_session_ops", "try_commit_session_operations"),
    ("replace_spec_operations", "replace_spec_operations"),
    ("replace_spec_ops", "replace_spec_operations"),
    ("remove_nodes_operations", "remove_nodes_operations"),
    ("remove_nodes_ops", "remove_nodes_operations"),
    ("insert_step_operations", "insert_step_operations"),
    ("insert_step_ops", "insert_step_operations"),
    ("remove_step_operations", "remove_step_operations"),
    ("remove_step_ops", "remove_step_operations"),
    ("patch_layer_operations", "patch_layer_operations"),
    ("patch_layer_ops", "patch_layer_operations"),
    ("engagement_submit_operations", "engagement_submit_operations"),
    ("engagement_submit_ops", "engagement_submit_operations"),
    ("emit_create_operations", "emit_create_operations"),
    ("emit_create_ops", "emit_create_operations"),
    ("reposition_operations", "reposition_operations"),
    ("reposition_ops", "reposition_operations"),
    ("removal_operations", "removal_operations"),
    ("removal_ops", "removal_operations"),
    ("segments_to_pdf_operations", "segments_to_pdf_operations"),
    ("segments_to_pdf_ops", "segments_to_pdf_operations"),
    ("writer_hidden_operation", "writer_hidden_operation"),
    ("writer_hidden_op", "writer_hidden_operation"),
    ("hidden_operation", "hidden_operation"),
    ("hidden_op", "hidden_operation"),
    ("update_block_operation", "update_block_operation"),
    ("update_block_op", "update_block_operation"),
    ("insert_operation", "insert_operation"),
    ("insert_op", "insert_operation"),
    ("append_operation", "append_operation"),
    ("append_op", "append_operation"),
    ("remove_operation", "remove_operation"),
    ("remove_op", "remove_operation"),
    ("hub_document_operation", "hub_document_operation"),
    ("hub_document_op", "hub_document_operation"),
    ("document_operation", "document_operation"),
    ("document_op", "document_operation"),
    ("graph_operation", "graph_operation"),
    ("graph_op", "graph_operation"),
    ("geo_operation", "geo_operation"),
    ("geo_op", "geo_operation"),
    ("num_operation", "num_operation"),
    ("num_op", "num_operation"),
    ("sup_operation", "sup_operation"),
    ("sup_op", "sup_operation"),
    ("text_operation", "text_operation"),
    ("text_op", "text_operation"),
    ("mesh_operation", "mesh_operation"),
    ("mesh_op", "mesh_operation"),
    ("paint_operation", "paint_operation"),
    ("paint_op", "paint_operation"),
    ("point_operation", "point_operation"),
    ("point_op", "point_operation"),
    ("vec_operation", "vec_operation"),
    ("vec_op", "vec_operation"),
    ("bool_operation", "bool_operation"),
    ("bool_op", "bool_operation"),
    ("boolean_operation", "boolean_operation"),
    ("boolean_op", "boolean_operation"),
    ("transform_operation", "transform_operation"),
    ("transform_op", "transform_operation"),
    ("binary_math_operation", "binary_math_operation"),
    ("binary_math_op", "binary_math_operation"),
    ("unary_math_operation", "unary_math_operation"),
    ("unary_math_op", "unary_math_operation"),
    ("add_step_operation", "add_step_operation"),
    ("add_step_op", "add_step_operation"),
    ("remove_step_operation", "remove_step_operation"),
    ("remove_step_op", "remove_step_operation"),
    ("move_step_operation", "move_step_operation"),
    ("move_step_op", "move_step_operation"),
    ("add_block_operation", "add_block_operation"),
    ("add_block_op", "add_block_operation"),
    ("remove_block_operation", "remove_block_operation"),
    ("remove_block_op", "remove_block_operation"),
    ("move_block_operation", "move_block_operation"),
    ("move_block_op", "move_block_operation"),
    ("update_protocol_title_operation", "update_protocol_title_operation"),
    ("update_protocol_title_op", "update_protocol_title_operation"),
    ("spawn_app_instance_operation", "spawn_app_instance_operation"),
    ("spawn_app_instance_op", "spawn_app_instance_operation"),
    ("patch_register_item_operation", "patch_register_item_operation"),
    ("patch_register_item_op", "patch_register_item_operation"),
    ("remove_register_item_operation", "remove_register_item_operation"),
    ("remove_register_item_op", "remove_register_item_operation"),
    ("patch_parameter_operation", "patch_parameter_operation"),
    ("patch_parameter_op", "patch_parameter_operation"),
    ("no-operation-restore", "no-operation-restore"),
    ("noop-restore", "no-operation-restore"),
    ("css-no-operation", "css-no-operation"),
    ("css-noop", "css-no-operation"),
    ("meshopt-no-operation", "meshopt-no-operation"),
    ("meshopt-noop", "meshopt-no-operation"),
    ("no-operation", "no-operation"),
    ("no-op", "no-operation"),
    ("No-operation", "No-operation"),
    ("No-op", "No-operation"),
    ("No Operation", "No Operation"),
    ("No Op", "No Operation"),
    # type alias / associated type
    ("type Operation", "type Operation"),
    ("type Op", "type Operation"),
]

# Drop identity pairs (anchors only used above for clarity — filter them out).
PAIRS = [(a, b) for a, b in PAIRS if a != b]
# Unique by src, longest first for alternation.
by_src: dict[str, str] = {}
for a, b in PAIRS:
    by_src.setdefault(a, b)
SOURCES = sorted(by_src.keys(), key=len, reverse=True)
COMBINED = re.compile(r"\b(" + "|".join(re.escape(s) for s in SOURCES) + r")\b")

# Protect geo::BooleanOps and OperationId family during bare renames.
PROTECT = re.compile(r"\b(?:BooleanOps|OperationId|operationId|operation_id)\b")

FILE_RENAMES = {
    "basic-remote-ops.json": "basic-remote-operations.json",
    "remote-ops-backlog.json": "remote-operations-backlog.json",
}


def iter_files() -> list[Path]:
    out: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS and not d.startswith(".venv")]
        for name in filenames:
            if name == "AGENTS.md":
                continue
            path = Path(dirpath) / name
            if path.suffix.lower() not in TEXT_SUFFIXES:
                continue
            out.append(path)
    return out


def rewrite(text: str, path: Path) -> str:
    saved: list[str] = []

    def stash(m: re.Match[str]) -> str:
        saved.append(m.group(0))
        return f"\0P{len(saved)-1}\0"

    text = PROTECT.sub(stash, text)
    text = COMBINED.sub(lambda m: by_src[m.group(1)], text)

    # serde tag
    text = re.sub(r'(tag\s*=\s*)"op"', r'\1"operation"', text)
    text = re.sub(r'(tag\s*=\s*)"operation"', r'\1"operation"', text)  # idempotent

    rel = path.as_posix()
    if rel.endswith("expression.json") or "/expression/" in rel:
        text = text.replace('"op"', '"operator"')
    elif rel.endswith(("action.json", "interaction.json")) or "/action/" in rel or "/interaction/" in rel:
        text = text.replace('"op"', '"operation"')
    elif path.suffix == ".json":
        def json_op(m: re.Match[str]) -> str:
            value = m.group(1)
            key = "operator" if value in {
                ">", "<", ">=", "<=", "==", "!=", "+", "-", "*", "/", "%",
                "&&", "||", "min", "max", "and", "or", "not",
                "union", "difference", "intersection", "xor",
            } else "operation"
            return f'"{key}": "{value}"'
        text = re.sub(r'"op"\s*:\s*"([^"]*)"', json_op, text)
        text = text.replace('"op"', '"operation"')

    # Operator field names after type renames
    text = re.sub(r"\bop:\s*BinaryOperator\b", "operator: BinaryOperator", text)
    text = re.sub(r"\bop:\s*RelationalOperator\b", "operator: RelationalOperator", text)
    text = re.sub(r"\bop:\s*BooleanOperator\b", "operator: BooleanOperator", text)
    text = re.sub(r"\bop:\s*DrawBooleanOperation\b", "operator: DrawBooleanOperation", text)

    # Bare Pascal / snake tokens
    text = re.sub(r"\bOps\b", "Operations", text)
    text = re.sub(r"\bOp\b", "Operation", text)
    text = re.sub(r"(?<![A-Za-z0-9_])ops(?![A-Za-z0-9_])", "operations", text)
    text = re.sub(r"(?<![A-Za-z0-9_])op(?![A-Za-z0-9_])", "operation", text)
    text = re.sub(r"\bnoop\b", "no_operation", text)
    text = re.sub(r"\bNoop\b", "NoOperation", text)
    text = text.replace('"no_operation"', '"noOperation"')
    text = text.replace("'no_operation'", "'noOperation'")

    # Path-qualify shadowed Operation trait bounds
    if "Operation: Operation<" in text:
        if "/vcs/" in rel or rel.endswith("vcs/rs/lib.rs"):
            text = text.replace("Operation: Operation<", "Operation: crate::Operation<")
        else:
            text = text.replace("Operation: Operation<", "Operation: vcs::Operation<")
    # Fix double path if re-run
    text = text.replace("Operation: vcs::vcs::Operation<", "Operation: vcs::Operation<")
    text = text.replace("Operation: crate::crate::Operation<", "Operation: crate::Operation<")

    # Wire kind camelCase repairs if snake leaked
    text = text.replace('"remote_operations"', '"remoteOperations"')
    text = text.replace('"local_operations"', '"localOperations"')
    text = text.replace('"pending_operations"', '"pendingOperations"')

    for i, value in enumerate(saved):
        text = text.replace(f"\0P{i}\0", value)
    return text


def main() -> int:
    files = iter_files()
    touched: dict[str, int] = {}
    for i, path in enumerate(files):
        if i % 200 == 0:
            print(f"… {i}/{len(files)}", flush=True)
        try:
            original = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        updated = rewrite(original, path)
        if updated != original:
            try:
                path.write_text(updated, encoding="utf-8")
            except OSError as exc:
                print(f"skip write {path}: {exc}", flush=True)
                continue
            touched[str(path.relative_to(ROOT))] = abs(len(updated) - len(original))

    renamed = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for name in filenames:
            if name in FILE_RENAMES:
                src = Path(dirpath) / name
                dst = Path(dirpath) / FILE_RENAMES[name]
                if src.exists() and not dst.exists():
                    src.rename(dst)
                    renamed.append([str(src.relative_to(ROOT)), str(dst.relative_to(ROOT))])

    report = {"files_touched": len(touched), "renamed_files": renamed, "files": sorted(touched)}
    (TICKET / "rename-report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(f"done touched={len(touched)} renamed={len(renamed)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
