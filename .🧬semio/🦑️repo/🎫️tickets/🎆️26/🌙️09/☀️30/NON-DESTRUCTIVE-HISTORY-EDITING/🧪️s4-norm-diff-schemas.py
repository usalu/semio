#!/usr/bin/env python3
"""🔺️ S4-NORM: every norm artifact's diff JSON schema states exactly the Rust diff wire (D16 diff-schema defect).

The Rust `<X>Diff` struct is the truth: each member is `Option<T>` and serializes as `null` or as the value of the
like-named snapshot field, so its schema is `anyOf [<the snapshot field>, null]` referenced by JSON Pointer into the
snapshot schema; a `<X>List { values: Vec<T> }` member wraps that field as `{values: …}`; `artifact` references the artifact
schema; every `…List` member is such a `{values}` wrapper (declared by struct or by the diff module's list macro). Generated for all fifteen artifacts so one rule holds norm-wide. `--check` prints the pending count and writes nothing.
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ARTIFACTS = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
FIELD = re.compile(r"^\s*pub\s+(?P<name>[a-z_0-9]+)\s*:\s*(?P<type>.+?),\s*$")
#: 🎯️ Diff members that patch a NESTED snapshot field (their apply writes there), as the snapshot property path they carry.
NESTED = {"🏭️vdi3805": {"manufacturerFile": ["catalog", "file"]}}


def pointer(snapshot: dict, path: list[str]) -> str:
    """📍️ The JSON Pointer of a snapshot property path, following local `$ref`s so the pointer names the declaring node."""
    location, node = "#", snapshot
    for step in path:
        while "$ref" in node:
            location = node["$ref"]
            node = snapshot
            for part in location[2:].split("/"):
                node = node[part]
        location, node = f"{location}/properties/{step}", node["properties"][step]
    return location


def camel(name: str) -> str:
    head, *rest = name.split("_")
    return head + "".join(part[:1].upper() + part[1:] for part in rest)


def struct_fields(source: str, name: str) -> list[tuple[str, str, str | None]]:
    """🧱 `(field, type, explicit rename)` of `pub struct <name> { … }` in declaration order."""
    start = re.search(r"pub struct %s\s*\{" % re.escape(name), source)
    if start is None:
        raise SystemExit(f"struct {name} not found")
    body = source[start.end():source.index("\n}", start.end())]
    fields, rename = [], None
    for line in body.splitlines():
        attribute = re.search(r'#\[value\(rename\s*=\s*"([^"]+)"\)\]', line)
        if attribute:
            rename = attribute.group(1)
            continue
        match = FIELD.match(line)
        if match:
            fields.append((match.group("name"), match.group("type").strip(), rename))
            rename = None
    return fields


def diff_schema(subset: Path) -> dict:
    schema_dir = subset / "🧬️schema"
    source = (schema_dir / "🔺️diff/🦀️.rs").read_text()
    snapshot = json.loads((schema_dir / "📸️snapshot/🔣️.json").read_text())
    artifact = json.loads((schema_dir / "🔣️.json").read_text())
    current = json.loads((schema_dir / "🔺️diff/🔣️.json").read_text())
    diff_name = re.search(r"pub struct (\w+Diff)\s*\{", source).group(1)
    properties = {}
    for field, rust_type, rename in struct_fields(source, diff_name):
        key = rename or camel(field)
        inner = re.fullmatch(r"Option<(.+)>", rust_type)
        if inner is None:
            raise SystemExit(f"{subset.parts[-5]} {diff_name}.{field}: non-Option diff member {rust_type}")
        inner_type = inner.group(1)
        if re.fullmatch(r"Box<.*Artifact>", inner_type):
            value = {"$ref": artifact["$id"]}
        else:
            path = NESTED.get(subset.parts[-5], {}).get(key, [key])
            if path[0] not in snapshot.get("properties", {}):
                raise SystemExit(f"{subset.parts[-5]} {diff_name}.{field}: no snapshot property {key}")
            field_ref = {"$ref": snapshot["$id"] + "#" + pointer(snapshot, path)[1:]}
            list_name = inner_type.split("::")[-1]
            value = {"type": "object", "additionalProperties": False, "required": ["values"], "properties": {"values": field_ref}} if list_name.endswith("List") else field_ref
        state = current.get("properties", {}).get(key, {}).get("x-semio-state", "artifact")
        properties[key] = {"anyOf": [value, {"type": "null"}], "x-semio-state": state}
    return {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": current["$id"],
        "title": current.get("title", diff_name),
        "type": "object",
        "additionalProperties": False,
        "properties": properties,
    }


def main(argv: list[str]) -> int:
    check = "--check" in argv
    pending = []
    for subset in sorted(ARTIFACTS.glob("*/🏅️standards/🔖️1/🪆️subsets/✳️any")):
        path = subset / "🧬️schema/🔺️diff/🔣️.json"
        text = path.read_text()
        new = json.dumps(diff_schema(subset), indent=2, ensure_ascii=False) + "\n"
        if new != text:
            pending.append((path, new))
    if not check:
        for path, new in pending:
            path.write_text(new)
    print(("pending=" if check else "written=") + str(len(pending)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
