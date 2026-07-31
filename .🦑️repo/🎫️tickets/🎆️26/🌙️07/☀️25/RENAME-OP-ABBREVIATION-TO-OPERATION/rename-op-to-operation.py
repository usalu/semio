#!/usr/bin/env python3
"""Rename operation abbreviations (op/Op/Ops) to the long form across the repo."""

from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path

# .repo/🎫️/YY/MM/DD/TICKETSLUG/script.py → repo root is parents[6]
ROOT = Path(__file__).resolve().parents[6]
TICKET = Path(__file__).resolve().parent
assert (ROOT / "vcs").is_dir(), f"ROOT looks wrong: {ROOT}"

SKIP_DIR_NAMES = {
    ".git",
    ".repo",
    "node_modules",
    "target",
    "dist",
    "build",
    ".next",
    "coverage",
    "__pycache__",
    ".turbo",
    "out",
}
SKIP_FILE_SUFFIXES = {
    ".png",
    ".jpg",
    ".jpeg",
    ".gif",
    ".webp",
    ".ico",
    ".pdf",
    ".zip",
    ".wasm",
    ".bin",
    ".lock",
    ".woff",
    ".woff2",
    ".ttf",
    ".otf",
    ".mp4",
    ".webm",
    ".glb",
    ".gltf",
    ".hdr",
    ".exr",
}
TEXT_SUFFIXES = {
    ".rs",
    ".ts",
    ".tsx",
    ".js",
    ".jsx",
    ".mjs",
    ".cjs",
    ".json",
    ".md",
    ".mdx",
    ".toml",
    ".wit",
    ".cs",
    ".go",
    ".py",
    ".rb",
    ".graphql",
    ".gql",
    ".yml",
    ".yaml",
    ".css",
    ".scss",
    ".html",
    ".svg",
    ".txt",
    ".snap",
    ".wgsl",
    ".glsl",
    ".vert",
    ".frag",
    ".sql",
    ".sh",
    ".ps1",
    ".bat",
    ".cmd",
    ".editorconfig",
    ".gitignore",
    ".npmrc",
    ".nvmrc",
}

# External / operator names that must not become *Operation.
PRESERVE_EXACT = {
    "BooleanOps",  # geo crate trait
}

# Types where Op means Operator (not Operation).
OPERATOR_TYPES = {
    "BinOp": "BinaryOperator",
    "RelOp": "RelationalOperator",
    "BooleanOp": "BooleanOperator",
    "AssumeRelOp": "AssumeRelationalOperator",
}

# Document / VCS / domain operation types (Op → Operation).
OPERATION_TYPES = [
    "PresentationEditOp",
    "Puzzle2dLiveMirrorOps",
    "collectPuzzle2dLiveMirrorOps",
    "pushPuzzle2dLiveMirrorOps",
    "Gis3dTerrainOp",
    "ModulePayloadOp",
    "MindmapWiresOp",
    "StudioHistoryOp",
    "RewriteRuleOp",
    "TimestampedOp",
    "TrinityGraphOp",
    "Procedural2dOp",
    "Procedural3dOp",
    "SetDocumentOp",
    "GenerationOp",
    "CollectionOp",
    "ImperativeOp",
    "DrawBooleanOp",
    "Process3dOp",
    "Puzzle2dOp",
    "Puzzle3dOp",
    "Puzzle5dOp",
    "VcsDemoOp",
    "SourcingOp",
    "SequenceOp",
    "ProtocolOp",
    "ShootingOp",
    "PresentOp",
    "ProgramOp",
    "RemodelOp",
    "LowpolyOp",
    "LayoutOp",
    "RasterOp",
    "WriterOp",
    "DiffLineOp",
    "GisMapOp",
    "StudioOp",
    "SHomeOp",
    "Fem2dOp",
    "Fem3dOp",
    "DummyOp",
    "DemoOp",
    "TestOp",
    "MathOp",
    "FlowOp",
    "FormOp",
    "NoteOp",
    "CadOp",
    "DagOp",
    "KitOp",
    "OsOp",
    "ShellOp",
    "StoreOp",
    "LoadOp",
    "DocumentOp",
    "AppendedOp",
    "AppendOps",
    "RemoteOps",
    "LocalOps",
    "ReplayGoldenOps",
    "replayGoldenOps",
    "processPluginOps",
    "pendingOps",
    "remoteOps",
    "localOps",
    "FieldOps",
    "NodeOps",
    "EdgeOps",
    "EditOps",
    "GraphOps",
    "ContentOps",
    "CloudOps",
    "ShapeOps",
    "StyleOps",
    "PathOps",
    "TextOps",
    "XformOps",
    "GroupOps",
    "ClipOps",
    "BoolOps",
    "GradientOps",
    "AlgebraicOps",
    "CertifiedOps",
    "BitOps",
    "SetBooleanOp",
    "PortKindNotDeclaredOnOp",
    "OpEnvelope",
]

# Longest-first identifier renames applied with word boundaries.
IDENT_RENAMES: list[tuple[str, str]] = []

for src, dst in OPERATOR_TYPES.items():
    IDENT_RENAMES.append((src, dst))

for name in sorted(OPERATION_TYPES, key=len, reverse=True):
    if name == "OpEnvelope":
        IDENT_RENAMES.append((name, "OperationEnvelope"))
    elif name.endswith("Ops"):
        IDENT_RENAMES.append((name, name[:-3] + "Operations"))
    elif name.endswith("Op"):
        IDENT_RENAMES.append((name, name[:-2] + "Operation"))
    else:
        raise SystemExit(f"unexpected operation token: {name}")

# Camel / snake helpers that contain Op/op as operation abbreviation.
EXTRA_RENAMES = [
    ("applyPresentationEditOp", "applyPresentationEditOperation"),
    ("backwardsPresentationEditOp", "backwardsPresentationEditOperation"),
    ("diffPresentationEditOp", "diffPresentationEditOperation"),
    ("applyOp", "applyOperation"),
    ("DRAW_BOOLEAN_OPS", "DRAW_BOOLEAN_OPERATIONS"),
    ("opEnvelope", "operationEnvelope"),
    ("OpId", "OperationId"),  # only if not already OperationId — careful
    ("opId", "operationId"),
    ("op_id", "operation_id"),
    ("op_envelope_from_edit", "operation_envelope_from_edit"),
    ("op_envelope_from_stored_edit", "operation_envelope_from_stored_edit"),
    ("edit_from_op_envelope", "edit_from_operation_envelope"),
    ("foreign_op_envelope", "foreign_operation_envelope"),
    ("sample_op_envelope", "sample_operation_envelope"),
    ("deliver_remote_ops", "deliver_remote_operations"),
    ("persist_ops", "persist_operations"),
    ("relay_ops_to_hub", "relay_operations_to_hub"),
    ("apply_ops", "apply_operations"),
    ("append_ops", "append_operations"),
    ("host_ops", "host_operations"),
    ("golden_ops", "golden_operations"),
    ("pending_ops", "pending_operations"),
    ("follow_up_ops", "follow_up_operations"),
    ("generation_ops", "generation_operations"),
    ("edit_ops", "edit_operations"),
    ("load_ops", "load_operations"),
    ("sub_ops", "sub_operations"),
    ("depth_ops", "depth_operations"),
    ("stencil_ops", "stencil_operations"),
    ("path_ops", "path_operations"),
    ("collection_op", "collection_operation"),
    ("invert_collection_op", "invert_collection_operation"),
    ("apply_collection_op", "apply_collection_operation"),
    ("collection_diff_from_op", "collection_diff_from_operation"),
    ("apply_remodel_op", "apply_remodel_operation"),
    ("apply_generation_op", "apply_generation_operation"),
    ("invert_generation_op", "invert_generation_operation"),
    ("apply_program_op", "apply_program_operation"),
    ("invert_program_op", "invert_program_operation"),
    ("apply_trinity_graph_ops", "apply_trinity_graph_operations"),
    ("dispatch_trinity_graph_ops", "dispatch_trinity_graph_operations"),
    ("validate_trinity_graph_op", "validate_trinity_graph_operation"),
    ("apply_protocol_edit_op", "apply_protocol_edit_operation"),
    ("apply_draw_edit_op", "apply_draw_edit_operation"),
    ("apply_form_edit_op", "apply_form_edit_operation"),
    ("apply_puzzle2d_op", "apply_puzzle2d_operation"),
    ("apply_puzzle3d_op", "apply_puzzle3d_operation"),
    ("apply_puzzle5d_op", "apply_puzzle5d_operation"),
    ("apply_lowpoly_op", "apply_lowpoly_operation"),
    ("puzzle2d_document_delta_ops", "puzzle2d_document_delta_operations"),
    ("puzzle3d_document_delta_ops", "puzzle3d_document_delta_operations"),
    ("puzzle5d_document_delta_ops", "puzzle5d_document_delta_operations"),
    ("procedural2d_fixture_ops", "procedural2d_fixture_operations"),
    ("procedural3d_fixture_ops", "procedural3d_fixture_operations"),
    ("flow_fixture_ops", "flow_fixture_operations"),
    ("sequence_fixture_ops", "sequence_fixture_operations"),
    ("vcs_demo_projection_diff_ops", "vcs_demo_projection_diff_operations"),
    ("try_commit_session_ops", "try_commit_session_operations"),
    ("replace_spec_ops", "replace_spec_operations"),
    ("remove_nodes_ops", "remove_nodes_operations"),
    ("insert_step_ops", "insert_step_operations"),
    ("remove_step_ops", "remove_step_operations"),
    ("patch_layer_ops", "patch_layer_operations"),
    ("engagement_submit_ops", "engagement_submit_operations"),
    ("emit_create_ops", "emit_create_operations"),
    ("reposition_ops", "reposition_operations"),
    ("removal_ops", "removal_operations"),
    ("segments_to_pdf_ops", "segments_to_pdf_operations"),
    ("hidden_op", "hidden_operation"),
    ("writer_hidden_op", "writer_hidden_operation"),
    ("update_block_op", "update_block_operation"),
    ("insert_op", "insert_operation"),
    ("append_op", "append_operation"),
    ("remove_op", "remove_operation"),
    ("document_op", "document_operation"),
    ("hub_document_op", "hub_document_operation"),
    ("graph_op", "graph_operation"),
    ("geo_op", "geo_operation"),
    ("num_op", "num_operation"),
    ("sup_op", "sup_operation"),
    ("text_op", "text_operation"),
    ("mesh_op", "mesh_operation"),
    ("paint_op", "paint_operation"),
    ("point_op", "point_operation"),
    ("vec_op", "vec_operation"),
    ("bool_op", "bool_operation"),
    ("boolean_op", "boolean_operation"),
    ("transform_op", "transform_operation"),
    ("binary_math_op", "binary_math_operation"),
    ("unary_math_op", "unary_math_operation"),
    ("add_step_op", "add_step_operation"),
    ("remove_step_op", "remove_step_operation"),
    ("move_step_op", "move_step_operation"),
    ("add_block_op", "add_block_operation"),
    ("remove_block_op", "remove_block_operation"),
    ("move_block_op", "move_block_operation"),
    ("update_protocol_title_op", "update_protocol_title_operation"),
    ("spawn_app_instance_op", "spawn_app_instance_operation"),
    ("patch_register_item_op", "patch_register_item_operation"),
    ("remove_register_item_op", "remove_register_item_operation"),
    ("patch_parameter_op", "patch_parameter_operation"),
    ("repoOp", "repoOperation"),
    ("booleanOp", "booleanOperation"),
    ("binOp", "binaryOperator"),
    # no-operation long form
    ("noop-restore", "no-operation-restore"),
    ("css-noop", "css-no-operation"),
    ("meshopt-noop", "meshopt-no-operation"),
    ("no-op", "no-operation"),
    ("No-op", "No-operation"),
    ("No Op", "No Operation"),
]

for src, dst in EXTRA_RENAMES:
    IDENT_RENAMES.append((src, dst))

# Deduplicate while preserving order (first wins).
seen: set[str] = set()
ordered: list[tuple[str, str]] = []
for src, dst in IDENT_RENAMES:
    if src in PRESERVE_EXACT:
        continue
    if src in seen:
        continue
    # Don't clobber OperationId → OperationOperationId
    if src == "OpId":
        continue
    seen.add(src)
    ordered.append((src, dst))
IDENT_RENAMES = ordered

FILE_RENAMES = {
    "basic-remote-ops.json": "basic-remote-operations.json",
    "remote-ops-backlog.json": "remote-operations-backlog.json",
}


def should_skip_dir(name: str) -> bool:
    return name in SKIP_DIR_NAMES or name.startswith(".venv")


def is_text_file(path: Path) -> bool:
    if path.name == "AGENTS.md" or path.name.endswith(".agents.md"):
        return False
    if path.suffix.lower() in SKIP_FILE_SUFFIXES:
        return False
    if path.suffix.lower() in TEXT_SUFFIXES:
        return True
    if path.name in {"Dockerfile", "Makefile", "LICENSE", "README", "README.md"}:
        return True
    # extensionless scripts
    if path.suffix == "" and path.is_file() and path.stat().st_size < 200_000:
        return False
    return False


def protect_spans(text: str) -> tuple[str, list[str]]:
    """Protect geo::BooleanOps and already-correct OperationId from rewrites."""
    saved: list[str] = []

    def stash(match: re.Match[str]) -> str:
        saved.append(match.group(0))
        return f"__RENAME_PROTECT_{len(saved) - 1}__"

    # Keep external geo trait import/use paths intact.
    text = re.sub(r"\bBooleanOps\b", stash, text)
    # Keep OperationId (already long) from OpId rules if any slip in.
    text = re.sub(r"\bOperationId\b", stash, text)
    text = re.sub(r"\boperationId\b", stash, text)
    text = re.sub(r"\boperation_id\b", stash, text)
    return text, saved


def restore_spans(text: str, saved: list[str]) -> str:
    for i, value in enumerate(saved):
        text = text.replace(f"__RENAME_PROTECT_{i}__", value)
    return text


def rename_identifiers(text: str) -> tuple[str, int]:
    count = 0
    text, saved = protect_spans(text)
    for src, dst in IDENT_RENAMES:
        pattern = re.compile(rf"\b{re.escape(src)}\b")
        text, n = pattern.subn(dst, text)
        count += n
    text = restore_spans(text, saved)
    return text, count


def rename_serde_and_json_keys(text: str, path: Path) -> tuple[str, int]:
    count = 0
    # serde tag = "op" on operation enums → operation
    text2, n = re.subn(r'(tag\s*=\s*)"op"', r'\1"operation"', text)
    count += n
    text = text2

    rel = path.as_posix()
    # Expression schemas / expression-like assets use operator.
    if rel.endswith("expression.json") or "/expression/" in rel:
        text2, n = re.subn(r'"op"', '"operator"', text)
        count += n
        text = text2
        return text, count

    # Action / interaction / effect schemas use operation.
    if rel.endswith(("action.json", "interaction.json")) or "/action/" in rel or "/interaction/" in rel:
        text2, n = re.subn(r'"op"', '"operation"', text)
        count += n
        text = text2
        return text, count

    # Generic JSON: classify by value shape.
    if path.suffix == ".json":

        def repl_key(match: re.Match[str]) -> str:
            nonlocal count
            full = match.group(0)
            value = match.group(1)
            # comparison / arithmetic / n-ary expression operators
            if value in {
                ">",
                "<",
                ">=",
                "<=",
                "==",
                "!=",
                "+",
                "-",
                "*",
                "/",
                "%",
                "&&",
                "||",
                "min",
                "max",
                "and",
                "or",
                "not",
                "union",
                "difference",
                "intersection",
                "xor",
            }:
                count += 1
                return full.replace('"op"', '"operator"', 1)
            count += 1
            return full.replace('"op"', '"operation"', 1)

        text = re.sub(r'"op"\s*:\s*"([^"]*)"', repl_key, text)
        # remaining "op" keys without string values (rare) → operation
        text2, n = re.subn(r'"op"', '"operation"', text)
        count += n
        text = text2
        return text, count

    # In code: JSON-looking "op" keys for frames / kinds often mean operation.
    # Prefer operation for remaining quoted op keys in non-expression files.
    # Leave operator field renames to dedicated pass below.
    return text, count


def rename_operator_fields(text: str) -> tuple[str, int]:
    """Rename field/param names that mean operator (BinOp / RelOp / Boolean CSG)."""
    count = 0
    patterns = [
        # Rust struct fields / patterns
        (r"\bop:\s*BinaryOperator\b", "operator: BinaryOperator"),
        (r"\bop:\s*RelationalOperator\b", "operator: RelationalOperator"),
        (r"\bop:\s*BooleanOperator\b", "operator: BooleanOperator"),
        (r"\bop:\s*DrawBooleanOperation\b", "operator: DrawBooleanOperation"),
        (r"\{ op:", "{ operator:"),
        (r"\(op,", "(operator,"),
        (r", op,", ", operator,"),
        (r", op\)", ", operator)"),
        (r"\bop ==\b", "operator =="),
        (r"\bmatch op\b", "match operator"),
        (r"\blet bop = match op\b", "let binary_operator = match operator"),
        (r"\bbop\b", "binary_operator"),
    ]
    for src, dst in patterns:
        text, n = re.subn(src, dst, text)
        count += n
    return text, count


def rename_type_params_and_bare(text: str, path: Path) -> tuple[str, int]:
    count = 0
    # Associated / alias: type Op = → type Operation =
    text, n = re.subn(r"\btype Op\b", "type Operation", text)
    count += n
    # Trait associated type declarations: type Op:
    text, n = re.subn(r"\btype Op:", "type Operation:", text)
    count += n

    # Generic type parameters named Op — rename to Operation.
    # Angle-bracket forms.
    def repl_generics(match: re.Match[str]) -> str:
        nonlocal count
        inner = match.group(1)
        parts = []
        for part in inner.split(","):
            raw = part
            token = part.strip()
            # Keep lifetimes / const generics / already Operation
            if re.fullmatch(r"Op", token):
                count += 1
                parts.append(part.replace("Op", "Operation", 1))
            elif re.fullmatch(r"Op\s*:\s*.+", token):
                count += 1
                parts.append(re.sub(r"\bOp\b", "Operation", part, count=1))
            else:
                parts.append(raw)
        return "<" + ",".join(parts) + ">"

    text, _ = re.subn(r"<([^<>{}]+)>", repl_generics, text)

    # impl<P, Op> / struct Foo<P, Op> already handled by angle brackets.
    # where Op: → where Operation: with trait path fix later
    text, n = re.subn(r"\bwhere Op:", "where Operation:", text)
    count += n
    text, n = re.subn(r"\bOp:", "Operation:", text)
    count += n

    # Fix Operation: Operation< bounds → path-qualified trait.
    # Prefer crate::Operation when inside vcs; else Operation trait via self:: or leave and fix per-file.
    if path.as_posix().endswith("vcs/rs/lib.rs") or "/vcs/" in path.as_posix():
        text, n = re.subn(r"\bOperation:\s*Operation<", "Operation: crate::Operation<", text)
        count += n
    else:
        # Common imports use Operation from vcs / prelude.
        text, n = re.subn(
            r"\bOperation:\s*Operation<",
            "Operation: ::vcs::Operation<",
            text,
        )
        # If file doesn't use vcs path, try unqualified via alias pattern OperationTrait — fallback:
        # many files `use vcs::Operation` — shadowing breaks. Insert path.
        count += n

    # PascalCase bare Op / Ops (regions, type params missed, docs).
    text, n = re.subn(r"\bOps\b", "Operations", text)
    count += n
    text, n = re.subn(r"\bOp\b", "Operation", text)
    count += n

    # Bare identifiers op / ops (word boundary), after longer renames.
    def repl_ops(match: re.Match[str]) -> str:
        nonlocal count
        count += 1
        return "operations"

    def repl_op(match: re.Match[str]) -> str:
        nonlocal count
        count += 1
        return "operation"

    text, _ = re.subn(r"(?<![A-Za-z0-9_])ops(?![A-Za-z0-9_])", repl_ops, text)
    text, _ = re.subn(r"(?<![A-Za-z0-9_])op(?![A-Za-z0-9_])", repl_op, text)

    # noop → no_operation; action-id strings prefer camelCase
    text, n = re.subn(r"\bnoop\b", "no_operation", text)
    count += n
    text, n = re.subn(r"\bNoop\b", "NoOperation", text)
    count += n
    text, n = re.subn(r'"no_operation"', '"noOperation"', text)
    count += n
    text, n = re.subn(r"'no_operation'", "'noOperation'", text)
    count += n

    return text, count


def fix_over_renames(text: str) -> tuple[str, int]:
    """Repair known false positives from bare op/ops replacement."""
    count = 0
    repairs = [
        # open / option / etc. should never have been matched due to word boundary
        # But operationeration if Operation + op glued — check
        ("operationeration", "operation"),
        ("Operationserations", "Operations"),
        ("operationserations", "operations"),
        # ActionEmit::operations is correct; ensure fn name ops→operations done via bare
        # geo protect restore already handled
        # Kind strings that must stay camelCase for wire protocol
        ('"remote_operations"', '"remoteOperations"'),
        ('"local_operations"', '"localOperations"'),
        ("kind: \"remote_operations\"", 'kind: "remoteOperations"'),
        ("kind: \"local_operations\"", 'kind: "localOperations"'),
        # Rust enum variants already renamed RemoteOperations
        # Fix ::vcs::Operation when file uses semio_vcs / vcs crate name differently — left to compile
        # noOperation action string preferences
        ("no_operation", "no_operation"),  # keep snake in Rust locals
    ]
    for src, dst in repairs:
        if src == dst:
            continue
        text, n = text.replace(src, dst), text.count(src)
        # use subn for count
        text, n = re.subn(re.escape(src), dst, text)
        count += n
    # Wire kind strings in JS often camelCase
    text, n = re.subn(r"\bremote_operations\b", "remoteOperations", text)
    count += n
    text, n = re.subn(r"\blocal_operations\b", "localOperations", text)
    count += n
    text, n = re.subn(r"\bpending_operations\b", "pendingOperations", text)
    count += n
    return text, count


def process_file(path: Path) -> int:
    try:
        original = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return 0
    text = original
    total = 0
    text, n = rename_identifiers(text)
    total += n
    text, n = rename_serde_and_json_keys(text, path)
    total += n
    text, n = rename_operator_fields(text)
    total += n
    text, n = rename_type_params_and_bare(text, path)
    total += n
    text, n = fix_over_renames(text)
    total += n
    if text != original:
        path.write_text(text, encoding="utf-8")
    return total if text != original else 0


def iter_files() -> list[Path]:
    files: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if not should_skip_dir(d)]
        for name in filenames:
            path = Path(dirpath) / name
            if not is_text_file(path):
                continue
            # skip this script and inventory outputs inside ticket (still under .repo — skipped)
            files.append(path)
    return files


def rename_files() -> list[tuple[str, str]]:
    renamed: list[tuple[str, str]] = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if not should_skip_dir(d)]
        for name in filenames:
            if name in FILE_RENAMES:
                src = Path(dirpath) / name
                dst = Path(dirpath) / FILE_RENAMES[name]
                if src.exists() and not dst.exists():
                    src.rename(dst)
                    renamed.append((str(src.relative_to(ROOT)), str(dst.relative_to(ROOT))))
    return renamed


def main() -> int:
    files = iter_files()
    per_file: dict[str, int] = {}
    total = 0
    for path in files:
        n = process_file(path)
        if n:
            rel = str(path.relative_to(ROOT))
            per_file[rel] = n
            total += n
    renamed = rename_files()
    report = {
        "files_touched": len(per_file),
        "replacement_events": total,
        "per_file": per_file,
        "renamed_files": renamed,
    }
    (TICKET / "rename-report.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(f"touched={len(per_file)} replacements~={total} renamed_files={len(renamed)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
