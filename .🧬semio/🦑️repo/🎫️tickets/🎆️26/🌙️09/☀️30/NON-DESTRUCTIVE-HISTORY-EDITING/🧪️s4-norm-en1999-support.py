#!/usr/bin/env python3
"""📍️ S4-NORM: EN 1999 member `support` becomes the closed enum `SupportCondition` (schema-first): the snapshot schema
gains `$defs/SupportCondition` (enum + en/de option labels) referenced by `AluminiumMember.support`, and the artifact schema
states the same enum. Wire values are unchanged (`simplySupported` | `continuous` | `cantilever`). The snapshot `annex` property
references its own `$defs/AnnexChoice` (the Rust `AnnexChoice`, wire `En`/`De`) instead of a free string, the artifact schema states
that enum. `--check` writes nothing."""
from __future__ import annotations

import json
import sys
from pathlib import Path

SCHEMA = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema")
VALUES = ["simplySupported", "continuous", "cantilever"]
OPTIONS = {
    "simplySupported": {"en": "Simply supported", "de": "Einfeldträger"},
    "continuous": {"en": "Continuous", "de": "Durchlaufträger"},
    "cantilever": {"en": "Cantilever", "de": "Kragarm"},
}


def snapshot(schema: dict) -> dict:
    member = schema["$defs"]["AluminiumMember"]["properties"]
    label = member["support"].get("x-semio-ui", {}).get("label", {"en": "Support conditions", "de": "Lagerungsbedingungen"})
    member["support"] = {"$ref": "#/$defs/SupportCondition", "x-semio-ui": {"label": label}}
    schema["properties"]["annex"] = {"$ref": "#/$defs/AnnexChoice", "x-semio-state": "artifact"}
    schema["$defs"]["SupportCondition"] = {"title": "SupportCondition", "type": "string", "enum": VALUES, "x-semio-ui": {"options": OPTIONS}, "x-semio-formats": ["🔣️jsonschema", "🟦️typescript"]}
    return schema


def artifact(schema: dict) -> dict:
    schema["properties"]["members"]["items"]["properties"]["support"] = {"type": "string", "enum": VALUES}
    schema["properties"]["annex"] = {"type": "string", "enum": ["En", "De"], "x-semio-state": "artifact"}
    return schema


def main(argv: list[str]) -> int:
    pending = []
    for path, rewrite in ((SCHEMA / "📸️snapshot/🔣️.json", snapshot), (SCHEMA / "🔣️.json", artifact)):
        text = path.read_text()
        new = json.dumps(rewrite(json.loads(text)), indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else "")
        if new != text:
            pending.append((path, new))
    if "--check" not in argv:
        for path, new in pending:
            path.write_text(new)
    print(("pending=" if "--check" in argv else "written=") + str(len(pending)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
