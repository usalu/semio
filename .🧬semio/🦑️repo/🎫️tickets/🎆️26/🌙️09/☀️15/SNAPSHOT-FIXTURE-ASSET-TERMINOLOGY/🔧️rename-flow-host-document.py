#!/usr/bin/env python3
"""🔧 Renames FlowFixture host document symbols (scoped, no Jack Graph bridge)."""
from __future__ import annotations

import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]

TYPE_REPLACEMENTS: list[tuple[str, str]] = [
    ("ReplaceFlowFixture", "ReplaceFlowHostDocument"),
    ("replace-flow-fixture", "replace-flow-host-document"),
    ("replace_flow_fixture", "replace_flow_host_document"),
    ("flow_fixture_operations", "flow_host_document_operations"),
    ("tree_from_fixture", "tree_from_host_document"),
    ("FlowOwnedFixtureRetirementFactory", "FlowOwnedHostDocumentRetirementFactory"),
    ("FlowFixtureRetirementFactory", "FlowHostDocumentRetirementFactory"),
    ("FlowFixtureRetirement", "FlowHostDocumentRetirement"),
    ("FlowFixtureDsl", "FlowHostDocumentDsl"),
    ("FlowFixture", "FlowHostDocument"),
    ('"flow.fixture"', '"flow.hostDocument"'),
    ("flow.fixture", "flow.hostDocument"),
    ("flow_vcs_fixture_scalar_digest", "flow_vcs_host_document_scalar_digest"),
    ("parse_fixture_json", "parse_host_document_json"),
    ("build_dag_fixture_v1", "build_dag_host_document_v1"),
    ("apply_generation_values_to_fixture", "apply_generation_values_to_host_document"),
    ("flow_fixture_to_form_spec", "flow_host_document_to_form_spec"),
]

FLOW_BRIDGE: list[tuple[str, str]] = [
    ("set_fixture_preserving_history", "set_host_document_preserving_history"),
    ("from_fixture_with_cache_and_infos", "from_host_document_with_cache_and_infos"),
    ("from_fixture_with_cache", "from_host_document_with_cache"),
    ("from_fixture_without_layout", "from_host_document_without_layout"),
    ("to_fixture", "to_host_document"),
    ("from_fixture", "from_host_document"),
]

DOCUMENT_API: list[tuple[str, str]] = [
    (".fixture()", ".host_document()"),
    ("fn fixture(&self)", "fn host_document(&self)"),
    ("fn fixture(&mut self", "fn host_document(&mut self"),
]

JACK_BRIDGE: list[tuple[str, str]] = [
    ("graph_from_fixture_or_default", "graph_from_snapshot_or_default"),
    ("fixture_to_workflow", "snapshot_to_workflow"),
    ("Graph::from_fixture", "Graph::from_snapshot"),
    ("pub fn from_fixture(mut fixture: JackSnapshot)", "pub fn from_snapshot(mut snapshot: JackSnapshot)"),
    ("pub fn to_fixture(&self)", "pub fn to_snapshot(&self)"),
    ("validate_trinity_fixture", "validate_trinity_snapshot"),
]


def all_paths(pattern: str, glob: str) -> list[Path]:
    out = subprocess.check_output(
        ["rg", "-l", pattern, "--glob", glob, "-g", "!.🧬semio/**", "-g", "!**/♻️mit-bestand/**"],
        cwd=ROOT,
        text=True,
    )
    return [ROOT / line.strip() for line in out.splitlines() if line.strip()]


def apply(path: Path, pairs: list[tuple[str, str]]) -> bool:
    text = path.read_text()
    for old, new in pairs:
        text = text.replace(old, new)
    if text != path.read_text():
        path.write_text(text)
        return True
    return False


def main() -> None:
    changed = 0
    rust_targets = all_paths("FlowFixture|flow\\.fixture|ReplaceFlowFixture", "*.rs")
    json_targets = all_paths("FlowFixture|flow\\.fixture|ReplaceFlowFixture", "*.json")
    for path in rust_targets + json_targets:
        if apply(path, TYPE_REPLACEMENTS):
            changed += 1

    flow_rust = list({p for p in all_paths("to_fixture|from_fixture", "*.rs") if "/🌊️flow/" in str(p)})

    for path in flow_rust:
        if apply(path, FLOW_BRIDGE):
            changed += 1

    vcs_rust = all_paths("\\.fixture\\(\\)|fn fixture\\(&", "*.rs")
    vcs_rust = [p for p in vcs_rust if "/🌊️flow/" in str(p)]
    for path in vcs_rust:
        if apply(path, DOCUMENT_API):
            changed += 1

    jack_rust = [p for p in all_paths("from_fixture|to_fixture|graph_from_fixture", "*.rs") if "/🔱️trinity/" in str(p) or "/🔌️jack/" in str(p)]
    for path in jack_rust:
        if apply(path, JACK_BRIDGE):
            changed += 1

    # fix jack helper body after param rename
    jack_editor = ROOT / "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
    if jack_editor.exists():
        text = jack_editor.read_text()
        text = text.replace(
            "crate::Graph::from_snapshot(fixture.clone())",
            "crate::Graph::from_snapshot(snapshot.clone())",
        )
        jack_editor.write_text(text)

    old = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🧬️mutations/♻️replace-flow-fixture"
    new = old.parent / "♻️replace-flow-host-document"
    if old.is_dir() and not new.exists():
        old.rename(new)

    print(f"touched {changed} files via replacements")


if __name__ == "__main__":
    main()
