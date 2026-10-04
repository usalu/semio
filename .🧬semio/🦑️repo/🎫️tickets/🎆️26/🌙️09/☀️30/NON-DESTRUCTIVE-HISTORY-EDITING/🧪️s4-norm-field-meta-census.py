#!/usr/bin/env python3
"""📊️ S4-NORM census: every norm `🏷️field-meta` table row vs the artifact's snapshot schema — does the schema already state the
row's label / unit / choices (`x-semio-ui` on the resolved node or along its `$ref` chain, `enum`)? Read-only."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ARTIFACTS = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
ROW = re.compile(r'\(\s*"(?P<path>[^"]+)"\s*,\s*(?:NormFieldMeta\s*\{\s*label_en:\s*"(?P<en>[^"]*)"\s*,\s*label_de:\s*"(?P<de>[^"]*)"\s*,\s*unit:\s*(?P<unit>None|Some\("[^"]*"\))\s*,\s*choices:\s*(?P<choices>None|Some\(\w+\))\s*\}|m\(\s*"(?P<en2>[^"]*)"\s*,\s*"(?P<de2>[^"]*)"\s*,\s*(?P<unit2>None|Some\("[^"]*"\))\s*,\s*(?P<choices2>None|Some\(\w+\))\s*\))\s*\)')


def resolve(schema: dict, node: dict, seen=0) -> list[dict]:
    chain = [node]
    while "$ref" in node and seen < 8:
        ref = node["$ref"]
        if not ref.startswith("#/"):
            break
        target = schema
        for step in ref[2:].split("/"):
            target = target[step]
        node = target
        chain.append(node)
        seen += 1
    return chain


def descend(schema: dict, path: str) -> list[dict] | None:
    chain = resolve(schema, schema)
    for raw in re.split(r"\.", path):
        name, *lists = raw.split("[]")
        node = next((c["properties"][name] for c in chain if name in c.get("properties", {})), None)
        if node is None:
            return None
        chain = resolve(schema, node)
        for _ in lists:
            items = next((c["items"] for c in chain if "items" in c), None)
            if items is None:
                return None
            chain = resolve(schema, items)
    return chain


def main() -> int:
    totals = {"rows": 0, "unresolved": 0, "label": 0, "choiceRows": 0, "choiceInSchema": 0}
    for meta in sorted(ARTIFACTS.glob("*/🏅️standards/🔖️1/🪆️subsets/✳️any/**/🏷️field-meta/🦀️.rs")):
        artifact = meta.relative_to(ARTIFACTS).parts[0]
        schema = json.loads((ARTIFACTS / artifact / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json").read_text())
        rows = [m.groupdict() for m in ROW.finditer(meta.read_text())]
        stats = {"rows": len(rows), "unresolved": 0, "label": 0, "choiceRows": 0, "choiceInSchema": 0}
        missing = []
        for row in rows:
            chain = descend(schema, row["path"])
            choices = row["choices"] or row["choices2"]
            if chain is None:
                stats["unresolved"] += 1
                missing.append(row["path"])
                continue
            ui = {}
            for node in chain:
                for key, value in node.get("x-semio-ui", {}).items():
                    ui.setdefault(key, value)
            if "label" in ui:
                stats["label"] += 1
            if choices != "None":
                stats["choiceRows"] += 1
                if any("enum" in node for node in chain) and "options" in ui:
                    stats["choiceInSchema"] += 1
        for key in totals:
            totals[key] += stats[key]
        print(f"{artifact}: {stats} unresolved={missing[:6]}")
    print("TOTAL", totals)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
