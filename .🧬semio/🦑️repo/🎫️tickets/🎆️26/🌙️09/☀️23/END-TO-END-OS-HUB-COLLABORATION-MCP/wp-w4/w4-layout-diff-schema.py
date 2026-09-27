"""🔺️ W4: layout's `FramePatch` diff-schema mirrors (JSON Schema, proto, GraphQL, TS) gain the Rust fields they lack:
`rotation` (rotate-frame, 09-25) and `locked`/`visible` (set-frame-flags). Idempotent; `--dry-run` lists the files.
usage: python3 w4-layout-diff-schema.py [--dry-run]"""
import json, sys

DIFF = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/"
DRY = "--dry-run" in sys.argv
changed = []


def rewrite(name, fn):
    before = open(DIFF + name, encoding="utf-8").read()
    after = fn(before)
    if after != before:
        changed.append(name)
        if not DRY:
            open(DIFF + name, "w", encoding="utf-8").write(after)


def once(text, old, new):
    if new in text:
        return text
    assert text.count(old) == 1, (old, text.count(old))
    return text.replace(old, new)


def schema(text):
    doc = json.loads(text)
    patch = doc["$defs"]["FramePatch"]
    nullable = lambda kind: {"anyOf": [{"type": kind}, {"type": "null"}]}
    order = ["x", "y", "width", "height", "rotation", "fill", "stroke", "wrap_mode", "columns", "locked", "visible"]
    properties = {**patch["properties"], "rotation": nullable("number"), "locked": nullable("boolean"), "visible": nullable("boolean")}
    patch["required"] = order
    patch["properties"] = {key: properties[key] for key in order}
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


def proto(text):
    return once(text, "optional string wrap_mode = 7; optional uint32 columns = 8; }", "optional string wrap_mode = 7; optional uint32 columns = 8; optional double rotation = 9; optional bool locked = 10; optional bool visible = 11; }")


def graphql(text):
    return once(text, "type FramePatch { x: Float y: Float width: Float height: Float fill: [Float!] stroke: [Float!] wrap_mode: String columns: Int }", "type FramePatch { x: Float y: Float width: Float height: Float rotation: Float fill: [Float!] stroke: [Float!] wrap_mode: String columns: Int locked: Boolean visible: Boolean }")


def ts(text):
    return once(text, "export interface FramePatch { x: number | null; y: number | null; width: number | null; height: number | null; fill: [number, number, number, number] | null; stroke: [number, number, number, number] | null; wrap_mode: string | null; columns: number | null }", "export interface FramePatch { x: number | null; y: number | null; width: number | null; height: number | null; rotation: number | null; fill: [number, number, number, number] | null; stroke: [number, number, number, number] | null; wrap_mode: string | null; columns: number | null; locked: boolean | null; visible: boolean | null }")


rewrite("🔣️.json", schema)
rewrite("🛰️.proto", proto)
rewrite("🔗️.graphql", graphql)
rewrite("🟦️.ts", ts)
print(("would change: " if DRY else "changed: ") + (", ".join(changed) or "nothing"))
