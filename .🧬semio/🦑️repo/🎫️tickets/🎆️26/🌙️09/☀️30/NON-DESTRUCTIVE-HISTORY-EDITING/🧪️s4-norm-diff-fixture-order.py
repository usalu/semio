#!/usr/bin/env python3
"""🔺️ S4-NORM: committed norm `🔺️diff` fixtures in the canonical Rust diff wire — every diff member present (`null` when
`None`), object members in schema declaration order (the `ToValue` order), two-space JSON with Python float repr. The diff
schema and the snapshot schema it points into decide the order; values never change. `--check` writes nothing."""
from __future__ import annotations

import json
import sys
from pathlib import Path

ARTIFACTS = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")


class Schemas:
    """🗂️ The subset's schemas by `$id`, resolving local and cross-document references (JSON Pointer fragments)."""

    def __init__(self, documents: list[dict]):
        self.by_id = {document["$id"]: document for document in documents if "$id" in document}

    def resolve(self, node: dict, base: dict) -> tuple[dict, dict]:
        while isinstance(node, dict) and "$ref" in node:
            target_id, _, pointer = node["$ref"].partition("#")
            document = base if target_id == "" else self.by_id[target_id]
            node = document
            for step in [part for part in pointer.split("/") if part]:
                node = node[step.replace("~1", "/").replace("~0", "~")]
            base = document
        return node, base


def ordered(schemas: Schemas, value, node: dict, base: dict, top: bool = False):
    node, base = schemas.resolve(node, base)
    branches = node.get("anyOf") or node.get("oneOf")
    if branches:
        if value is None:
            return None
        candidates = [schemas.resolve(branch, base) for branch in branches if branch.get("type") != "null"]
        objects = [candidate for candidate in candidates if isinstance(value, dict) and "properties" in candidate[0] and set(value) <= set(candidate[0]["properties"])]
        node, base = (objects or candidates)[0]
    if isinstance(value, list):
        return [ordered(schemas, item, node.get("items", {}), base) for item in value]
    if isinstance(value, dict):
        properties = node.get("properties", {})
        keys = [key for key in properties if key in value or top] + [key for key in value if key not in properties]
        return {key: ordered(schemas, value.get(key), properties.get(key, {}), base) for key in keys}
    return value


def main(argv: list[str]) -> int:
    check = "--check" in argv
    pending = []
    for subset in sorted(ARTIFACTS.glob("*/🏅️standards/🔖️1/🪆️subsets/✳️any")):
        schema_dir = subset / "🧬️schema"
        diff = json.loads((schema_dir / "🔺️diff/🔣️.json").read_text())
        schemas = Schemas([diff, json.loads((schema_dir / "📸️snapshot/🔣️.json").read_text()), json.loads((schema_dir / "🔣️.json").read_text())])
        for path in sorted(subset.glob("🧫️fixtures/🧬️mutations/*/*/🔺️diff/🔣️.json")):
            text = path.read_text()
            new = json.dumps(ordered(schemas, json.loads(text), diff, diff, top=True), indent=2, ensure_ascii=False) + "\n"
            if new != text:
                pending.append((path, new))
    if not check:
        for path, new in pending:
            path.write_text(new)
    print(("pending=" if check else "written=") + str(len(pending)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
