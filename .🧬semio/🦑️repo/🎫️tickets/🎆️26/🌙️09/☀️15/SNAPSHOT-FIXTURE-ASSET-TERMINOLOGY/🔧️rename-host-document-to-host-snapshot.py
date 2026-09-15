#!/usr/bin/env python3
"""🔧 Replaces interim *HostDocument* vocabulary with *HostSnapshot* (no product-domain “document”)."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]

SKIP_SUBSTR = (
    "/.🧬semio/🎫️tickets/",
    "/♻️mit-bestand/",
    "/encode_document",
    "parent_document_id",
    "UiDocument",
    "import-document",
    "export-document",
    "set-fixture-json",
    "setFixtureJson",
    "syncDocumentJson",  # raster/map session API
    "NoteDocument",
    "WriterDocument",
)

# Longest-first symbol replacements (Rust / JSON / TS wire).
REPLACEMENTS: list[tuple[str, str]] = [
    ("ReplaceFlowHostDocument", "ReplaceFlowHostSnapshot"),
    ("ReplacedFlowHostDocument", "ReplacedFlowHostSnapshot"),
    ("FlowOwnedHostDocumentRetirementFactory", "FlowOwnedHostSnapshotRetirementFactory"),
    ("FlowHostDocumentRetirementFactory", "FlowHostSnapshotRetirementFactory"),
    ("FlowHostDocumentRetirement", "FlowHostSnapshotRetirement"),
    ("FlowHostDocumentCopy", "FlowHostSnapshotCopy"),
    ("FlowHostDocumentDsl", "FlowHostSnapshotDsl"),
    ("SequenceHostDocument", "SequenceHostSnapshot"),
    ("DagHostDocument", "DagHostSnapshot"),
    ("FlowHostDocument", "FlowHostSnapshot"),
    ("replace-flow-host-document", "replace-flow-host-snapshot"),
    ("replace_flow_host_document", "replace_flow_host_snapshot"),
    ("flow_vcs_host_document_scalar_digest", "flow_vcs_host_snapshot_scalar_digest"),
    ("flow_host_document_operations", "flow_host_snapshot_operations"),
    ("generation3d_host_document_operations", "generation3d_host_snapshot_operations"),
    ("generation2d_host_document_operations", "generation2d_host_snapshot_operations"),
    ("generation_host_document_for", "generation_host_snapshot_for"),
    ("flow_host_document_to_form_spec", "flow_host_snapshot_to_form_spec"),
    ("flow_host_document_to_dsl", "flow_host_snapshot_to_dsl"),
    ("os_workflow_to_flow_host_document_json", "os_workflow_to_flow_host_snapshot_json"),
    ("apply_flow_host_document_to_os_workflow", "apply_flow_host_snapshot_to_os_workflow"),
    ("dag_host_document_to_workflow", "dag_host_snapshot_to_workflow"),
    ("puzzle3d_operations_from_host_document_change", "puzzle3d_operations_from_host_snapshot_change"),
    ("set_host_document_preserving_history", "set_host_snapshot_preserving_history"),
    ("from_host_document_with_cache_and_infos", "from_host_snapshot_with_cache_and_infos"),
    ("from_host_document_with_cache", "from_host_snapshot_with_cache"),
    ("from_host_document_without_layout", "from_host_snapshot_without_layout"),
    ("replace_host_document_without_layout", "replace_host_snapshot_without_layout"),
    ("resync_host_document_from_scene", "resync_host_snapshot_from_scene"),
    ("replace_host_document", "replace_host_snapshot"),
    ("parse_host_document_json", "parse_host_snapshot_json"),
    ("host_document_json", "host_snapshot_json"),
    ("hostDocumentJson", "hostSnapshotJson"),
    ("hostDocumentChanged", "hostSnapshotChanged"),
    ('"hostDocument"', '"hostSnapshot"'),
    ("HostDocument(", "HostSnapshot("),
    ("FlowOwner::HostDocument", "FlowOwner::HostSnapshot"),
    ("FlowDelta::HostDocument", "FlowDelta::HostSnapshot"),
    ("to_host_document", "to_host_snapshot"),
    ("from_host_document", "from_host_snapshot"),
    ("with_host_document", "with_host_snapshot"),
    ("commit_host_document", "commit_host_snapshot"),
    ("tree_from_host_document", "tree_from_host_snapshot"),
    ("build_dag_host_document_v1", "build_dag_host_snapshot_v1"),
    ("apply_dag_layout_to_host_document_v1_value", "apply_dag_layout_to_host_snapshot_v1_value"),
    ("dag_document_from_host_document", "dag_document_from_host_snapshot"),
    ("dag_fixture_from_host_document", "dag_fixture_from_host_snapshot"),
    ('"flow.hostDocument"', '"flow.hostSnapshot"'),
    ("flow.hostDocument", "flow.hostSnapshot"),
    ('schema = "flow.host_document"', 'schema = "flow.host_snapshot"'),
    ("flow.host_document", "flow.host_snapshot"),
    ('schema = "dag.host_document"', 'schema = "dag.host_snapshot"'),
    ("dag.host_document", "dag.host_snapshot"),
    ("SetHostDocument", "SetHostSnapshot"),
    ("setHostDocument", "setHostSnapshot"),
    ("set-host-document", "set-host-snapshot"),
    ("host_document:", "host_snapshot:"),
    ("&host_document", "&host_snapshot"),
    (".host_document", ".host_snapshot"),
    (" fn host_document(", " fn host_snapshot("),
    ("synchronizeDocumentJson", "synchronizeSnapshotJson"),
    ("documentJson()", "snapshotJson()"),
    ("documentJson:", "snapshotJson:"),
    ('"documentJson"', '"snapshotJson"'),
    ("documentJson:commit", "snapshotJson:commit"),
    ("load_host_document_json", "load_host_snapshot_json"),
    ("host_document_json(", "host_snapshot_json("),
]


def list_files() -> list[Path]:
    out = subprocess.check_output(
        [
            "rg",
            "-l",
            "HostDocument|hostDocument|host_document|hostDocumentJson|setHostDocument|synchronizeDocumentJson",
            "--glob",
            "*.{rs,ts,tsx,json,js,md}",
            "-g",
            "!.🧬semio/**",
            "-g",
            "!**/♻️mit-bestand/**",
        ],
        cwd=ROOT,
        text=True,
    )
    paths = [ROOT / line.strip() for line in out.splitlines() if line.strip()]
    return [p for p in paths if not any(s in str(p) for s in SKIP_SUBSTR)]


def apply(path: Path) -> bool:
    text = path.read_text()
    original = text
    for old, new in REPLACEMENTS:
        text = text.replace(old, new)
    if text != original:
        path.write_text(text)
        return True
    return False


def rename_mutation_dir() -> None:
    old = ROOT / (
        "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/"
        "🌿️vcs/🧬️schema/🧬️mutations/♻️replace-flow-host-document"
    )
    new = old.parent / "♻️replace-flow-host-snapshot"
    if old.is_dir() and not new.exists():
        old.rename(new)


def main() -> None:
    changed = 0
    for path in list_files():
        if apply(path):
            changed += 1
    rename_mutation_dir()
    print(f"touched {changed} files")


if __name__ == "__main__":
    main()
