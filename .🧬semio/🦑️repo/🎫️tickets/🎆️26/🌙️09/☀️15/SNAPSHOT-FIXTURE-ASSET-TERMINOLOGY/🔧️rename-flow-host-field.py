#!/usr/bin/env python3
"""🔧 Finish FlowHost host_document field + API naming (with_fixture, commit_fixture, …)."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]

RUST_REPLACEMENTS: list[tuple[str, str]] = [
    ("FlowHost::with_fixture", "FlowHost::with_host_document"),
    ("fn with_fixture<R>", "fn with_host_document<R>"),
    ("pub fn replace_fixture(", "pub fn replace_host_document("),
    ("pub fn resync_fixture_from_scene(", "pub fn resync_host_document_from_scene("),
    ("fn apply_fixture(", "fn apply_host_document("),
    ("self.apply_fixture(", "self.apply_host_document("),
    ("dedupe_fixture_widgets(", "dedupe_host_document_widgets("),
    ("fn dedupe_fixture_widgets(", "fn dedupe_host_document_widgets("),
    ("commit_fixture(", "commit_host_document("),
    ("pub fn commit_fixture(", "pub fn commit_host_document("),
    ("apply_fixture_helpers(", "apply_host_document_helpers("),
    ("pub fn apply_fixture_helpers(", "pub fn apply_host_document_helpers("),
    ("host.fixture", "host.host_document"),
    ("&host.fixture", "&host.host_document"),
    ("base.fixture", "base.host_document"),
    ("ReplaceFlowHostDocument { fixture }", "ReplaceFlowHostDocument { host_document: fixture }"),
    ("ReplaceFlowHostDocument { fixture:", "ReplaceFlowHostDocument { host_document:"),
    ("leaf.fixture.", "leaf.host_document."),
    ("FlowMutation::ReplaceFlowHostDocument(ReplaceFlowHostDocument { fixture,", "FlowMutation::ReplaceFlowHostDocument(ReplaceFlowHostDocument { host_document:"),
]

FLOW_HOST_SELF = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs"

FLOW_HOST_SELF_EXTRA: list[tuple[str, str]] = [
    ("self.fixture", "self.host_document"),
    ("            fixture,", "            host_document: fixture,"),
]

JSON_GLOBS = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/**/🔣️.json",
]

JSON_REPLACEMENTS: list[tuple[str, str]] = [
    ('"delta": "fixture"', '"delta": "hostDocument"'),
    ('"const": "fixture"', '"const": "hostDocument"'),
    ('"fixture",', '"hostDocument",'),
    ('"kind": "fixture"', '"kind": "hostDocument"'),
]


def rust_files() -> list[Path]:
    out = subprocess.check_output(
        [
            "rg",
            "-l",
            r"with_fixture|host\.fixture|self\.fixture|commit_fixture|apply_fixture_helpers|ReplaceFlowHostDocument \{ fixture|leaf\.fixture|resync_fixture|replace_fixture|dedupe_fixture",
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


def rewrite_rust(path: Path) -> bool:
    text = path.read_text()
    original = text
    for old, new in RUST_REPLACEMENTS:
        text = text.replace(old, new)
    if path == FLOW_HOST_SELF:
        for old, new in FLOW_HOST_SELF_EXTRA:
            text = text.replace(old, new)
    if text != original:
        path.write_text(text)
        return True
    return False


def rewrite_json(path: Path) -> bool:
    text = path.read_text()
    original = text
    for old, new in JSON_REPLACEMENTS:
        text = text.replace(old, new)
    if '"hostDocument"' in text and '"host_document"' not in text:
        pass
    if text != original:
        path.write_text(text)
        return True
    return False


def json_files() -> list[Path]:
    out = subprocess.check_output(
        [
            "rg",
            "-l",
            r'"delta": "fixture"|"const": "fixture"|"kind": "fixture"',
            "--glob",
            "**/🌊️flow/**",
            "-g",
            "*.json",
            "-g",
            "!.🧬semio/**",
        ],
        cwd=ROOT,
        text=True,
    )
    return [ROOT / line.strip() for line in out.splitlines() if line.strip()]


def main() -> None:
    rust_changed = sum(1 for p in rust_files() if rewrite_rust(p))
    json_changed = sum(1 for p in json_files() if rewrite_json(p))
    print(f"updated {rust_changed} Rust files, {json_changed} JSON files")


if __name__ == "__main__":
    main()
