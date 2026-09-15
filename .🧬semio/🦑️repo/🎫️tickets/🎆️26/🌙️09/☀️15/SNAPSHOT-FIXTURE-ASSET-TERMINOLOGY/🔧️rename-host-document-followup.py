#!/usr/bin/env python3
"""🔧 Second pass: leftover host_document identifiers missed by path skips."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]

REPLACEMENTS = [
    ("flow_host_document_dsl_to_host_snapshot", "flow_host_snapshot_dsl_to_host_snapshot"),
    ("flow_host_document_dsl", "flow_host_snapshot_dsl"),
    ("flow_host_snapshot_to_dsl", "flow_host_snapshot_to_dsl"),
    ("generation3d_host_document_operations", "generation3d_host_snapshot_operations"),
    ("generation2d_host_document_operations", "generation2d_host_snapshot_operations"),
    ("apply_host_document_helpers", "apply_host_snapshot_helpers"),
    ("dedupe_host_document_widgets", "dedupe_host_snapshot_widgets"),
    ("apply_host_document(", "apply_host_snapshot("),
    ("fn apply_host_document", "fn apply_host_snapshot"),
    ("HostDocumentDiffing", "HostSnapshotDiffing"),
    ("HostDocumentOperations", "HostSnapshotOperations"),
    ("retire_owner!(FlowHostSnapshot, HostDocument)", "retire_owner!(FlowHostSnapshot, HostSnapshot)"),
    ("config_after_document_load", "config_after_snapshot_load"),
    ("host_document,", "host_snapshot,"),
    ("host_document }", "host_snapshot }"),
    ("host_document:", "host_snapshot:"),
    ("{ host_document", "{ host_snapshot"),
    (".host_document", ".host_snapshot"),
    ("&host_document", "&host_snapshot"),
    (" host_document ", " host_snapshot "),
    ("must be host_document", "must be host_snapshot"),
]


def paths() -> list[Path]:
    out = subprocess.check_output(
        ["rg", "-l", "host_document|HostDocument", "--glob", "*.{rs,ts,tsx,json}", "-g", "!.🧬semio/**"],
        cwd=ROOT,
        text=True,
    )
    return [ROOT / p.strip() for p in out.splitlines() if p.strip()]


def main() -> None:
    n = 0
    for path in paths():
        if "parent_document_id" in path.read_text() and False:
            pass
        text = path.read_text()
        orig = text
        for a, b in REPLACEMENTS:
            text = text.replace(a, b)
        if text != orig:
            path.write_text(text)
            n += 1
    print(f"followup touched {n} files")


if __name__ == "__main__":
    main()
