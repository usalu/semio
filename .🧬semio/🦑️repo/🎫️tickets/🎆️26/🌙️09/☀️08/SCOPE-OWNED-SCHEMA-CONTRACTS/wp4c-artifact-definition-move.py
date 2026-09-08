#!/usr/bin/env python3
"""🗿️ Moves `📜️artifact-definition.json` out of the artifact-level `🧬️schema/` slot.

`✏️s/🔌️plugins/*/🗿️artifacts/<artifact>` is not a declared `schemaScopeOwnerLevels` level — only the
standard subset below it and the plugin root above it are — so a `🧬️schema/` directory there is a
`schema-owner-ineligible` finding for every artifact that has one (51 in this partition). The
directory holds no schema at all: `📜️artifact-definition.json` is the artifact's *definition data*
(id, standards, profiles), and execution contract §B admits only the canonical five files and facet
directories inside a `🧬️schema/` module. Same repair as WP4b's `📜️native-codec-factories.json`: the
data file moves one directory up and every reader follows it.

Usage: python3 wp4c-artifact-definition-move.py report|apply
"""
from __future__ import annotations
import json, os, subprocess, sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
NAME = "📜️artifact-definition.json"
OLD = f"🧬️schema/{NAME}"


def artifacts() -> list[str]:
    found = []
    for plugin in sorted(os.listdir(os.path.join(REPO, "✏️s/🔌️plugins"))):
        container = os.path.join(REPO, "✏️s/🔌️plugins", plugin, "🗿️artifacts")
        if not os.path.isdir(container):
            continue
        for artifact in sorted(os.listdir(container)):
            module = os.path.join(container, artifact, "🧬️schema")
            if os.path.isfile(os.path.join(module, NAME)):
                found.append(os.path.relpath(os.path.join(container, artifact), REPO))
    return found


def readers() -> list[str]:
    # 🧱️`✏️s` only: `💻️os` carries the same literal for its own artifacts, which this partition never moves.
    out = subprocess.run(["git", "grep", "-lF", OLD, "--", "✏️s"], cwd=REPO, capture_output=True).stdout.decode()
    return [line for line in out.split("\n") if line and "🎫️tickets" not in line]


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "report"
    moved = []
    for artifact in artifacts():
        source = os.path.join(REPO, artifact, "🧬️schema", NAME)
        target = os.path.join(REPO, artifact, NAME)
        moved.append(artifact)
        if mode != "apply":
            continue
        os.replace(source, target)
        os.rmdir(os.path.join(REPO, artifact, "🧬️schema"))
    rewritten = 0
    for path in readers():
        absolute = os.path.join(REPO, path)
        if not os.path.exists(absolute):
            continue
        with open(absolute, encoding="utf-8") as handle:
            text = handle.read()
        if OLD not in text:
            continue
        rewritten += 1
        if mode != "apply":
            continue
        with open(absolute, "w", encoding="utf-8") as handle:
            handle.write(text.replace(OLD, NAME))
    print(json.dumps({"mode": mode, "artifactsMoved": len(moved), "readersRewritten": rewritten}, ensure_ascii=False))


if __name__ == "__main__":
    main()
