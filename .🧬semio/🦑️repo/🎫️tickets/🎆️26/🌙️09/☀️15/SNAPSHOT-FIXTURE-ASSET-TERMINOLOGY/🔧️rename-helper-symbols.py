#!/usr/bin/env python3
"""🔧 Renames runtime helper symbols that used fixture for FlowHostDocument semantics."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]

REPLACEMENTS: list[tuple[str, str]] = [
    ("generation3d_fixture_operations", "generation3d_host_document_operations"),
    ("generation2d_fixture_operations", "generation2d_host_document_operations"),
    ("generation_fixture_for", "generation_host_document_for"),
    ("flow_fixture_to_form_spec", "flow_host_document_to_form_spec"),
    ("flow_fixture_to_dsl", "flow_host_document_to_dsl"),
    ("flow_fixture_dsl_to_host_document", "flow_host_document_dsl_to_host_document"),
    ("os_workflow_to_flow_fixture", "os_workflow_to_flow_host_document_json"),
    ("apply_flow_fixture_to_os_workflow", "apply_flow_host_document_to_os_workflow"),
    ("//#region 🔖️FixtureDiffing", "//#region 🔖️HostDocumentDiffing"),
    ("//#region 🔖️FixtureOperations", "//#region 🔖️HostDocumentOperations"),
]


def targets() -> list[Path]:
    out = subprocess.check_output(
        [
            "rg",
            "-l",
            "generation3d_fixture_operations|generation2d_fixture_operations|generation_fixture_for|"
            "flow_fixture_to_form_spec|flow_fixture_to_dsl|os_workflow_to_flow_fixture|"
            "apply_flow_fixture_to_os_workflow",
            "--glob",
            "*.rs",
            "-g",
            "!.🧬semio/**",
            "-g",
            "!.generation3d-*-link/**",
        ],
        cwd=ROOT,
        text=True,
    )
    return [ROOT / line.strip() for line in out.splitlines() if line.strip()]


def rewrite(path: Path) -> bool:
    text = path.read_text()
    original = text
    for old, new in REPLACEMENTS:
        text = text.replace(old, new)
    if text != original:
        path.write_text(text)
        return True
    return False


def main() -> None:
    changed = sum(1 for path in targets() if rewrite(path))
    print(f"updated {changed} Rust files for helper symbol renames")


if __name__ == "__main__":
    main()
