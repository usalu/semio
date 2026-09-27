"""🧬️ W4: bring layout's mutation mirror surfaces (text grammar mirrors, proto, JSON schemas, GraphQL, TS) in line with the
28-variant `LayoutMutation` enum: `rotate-frame` (never mirrored), `update-grid` and `set-frame-flags` (a peer's 14:21 leaves).
Idempotent; `--dry-run` prints the files it would change.
usage: python3 w4-layout-surfaces.py [--dry-run]"""
import json, sys

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
DRY = "--dry-run" in sys.argv
changed = []


def rewrite(rel, fn):
    path = ROOT + rel
    before = open(path, encoding="utf-8").read()
    after = fn(before)
    if after != before:
        changed.append(rel)
        if not DRY:
            open(path, "w", encoding="utf-8").write(after)


def once(text, old, new):
    if new in text:
        return text
    assert text.count(old) == 1, (old, text.count(old))
    return text.replace(old, new)


def grammar(text):
    text = once(text, "     / resize-frame\n", "     / resize-frame\n     / rotate-frame\n")
    return once(text, 'resize-frame = "resize-frame" SP id SP id SP number SP number\n', 'resize-frame = "resize-frame" SP id SP id SP number SP number\nrotate-frame = "rotate-frame" SP id SP id SP number\n')


def ebnf(text):
    text = once(text, "     | resize frame\n", "     | resize frame\n     | rotate frame\n")
    text = once(text, "     | change frame columns ;\n", "     | change frame columns\n     | update grid\n     | set frame flags ;\n")
    text = once(text, "resize frame = 'resize-frame', space, id, space, id, space, number, space, number ;\n", "resize frame = 'resize-frame', space, id, space, id, space, number, space, number ;\nrotate frame = 'rotate-frame', space, id, space, id, space, number ;\n")
    return once(text, "change frame columns = 'change-frame-columns', space, id, space, id, space, number ;\n", "change frame columns = 'change-frame-columns', space, id, space, id, space, number ;\nupdate grid = 'update-grid', space, number, space, number, space, boolean ;\nset frame flags = 'set-frame-flags', space, id, space, id, space, [ boolean ], space, [ boolean ] ;\n")


def g4(text):
    text = once(text, "resizeFrame | changeFrameFill", "resizeFrame | rotateFrame | changeFrameFill")
    text = once(text, "changeFrameWrapMode | changeFrameColumns ;", "changeFrameWrapMode | changeFrameColumns | updateGrid | setFrameFlags ;")
    text = once(text, "resizeFrame: 'resize-frame' SP id SP id SP number SP number ;\n", "resizeFrame: 'resize-frame' SP id SP id SP number SP number ;\nrotateFrame: 'rotate-frame' SP id SP id SP number ;\n")
    return once(text, "changeFrameColumns: 'change-frame-columns' SP id SP id SP number ;\n", "changeFrameColumns: 'change-frame-columns' SP id SP id SP number ;\nupdateGrid: 'update-grid' SP number SP number SP boolean ;\nsetFrameFlags: 'set-frame-flags' SP id SP id SP boolean? SP boolean? ;\n")


def proto(text):
    text = once(text, "message UpdateGrid {\n", "message RotateFrame {\n  string page_id = 1;\n  string frame_id = 2;\n  double new_rotation = 3;\n}\nmessage UpdateGrid {\n")
    return once(text, "    SetFrameFlags set_frame_flags = 27;\n", "    SetFrameFlags set_frame_flags = 27;\n    RotateFrame rotate_frame = 28;\n")


def text_graphql(text):
    text = once(text, "(tags 1..25 match protocol.semio order)", "(`📡️.protocol.semio` owns the wire tags 0..27)")
    text = once(text, "input ResizeFrame { pageId: String! frameId: String! newWidth: Float! newHeight: Float! }\n", "input ResizeFrame { pageId: String! frameId: String! newWidth: Float! newHeight: Float! }\ninput RotateFrame { pageId: String! frameId: String! newRotation: Float! }\n")
    text = once(text, "input ChangeFrameColumns { pageId: String! frameId: String! newColumns: Float! }\n", "input ChangeFrameColumns { pageId: String! frameId: String! newColumns: Float! }\ninput UpdateGrid { baselineGrid: Float! baselineOffset: Float! snapToBaseline: Boolean! }\ninput SetFrameFlags { pageId: String! frameId: String! locked: Boolean visible: Boolean }\n")
    text = once(text, "| ResizeFrame | ChangeFrameFill", "| ResizeFrame | RotateFrame | ChangeFrameFill")
    return once(text, "| ChangeFrameWrapMode | ChangeFrameColumns\n", "| ChangeFrameWrapMode | ChangeFrameColumns | UpdateGrid | SetFrameFlags\n")


def text_json(text):
    doc = json.loads(text)
    titles = [entry["title"] for entry in doc["oneOf"]]
    def entry(title, kind, props, required):
        return {"type": "object", "title": title, "properties": {"kind": {"const": kind}, **props}, "required": ["kind", *required]}
    if "RotateFrame" not in titles:
        doc["oneOf"].insert(titles.index("ResizeFrame") + 1, entry("RotateFrame", "rotate-frame", {"pageId": {"type": "string"}, "frameId": {"type": "string"}, "newRotation": {"type": "number"}}, ["pageId", "frameId", "newRotation"]))
    if "UpdateGrid" not in titles:
        doc["oneOf"].append(entry("UpdateGrid", "update-grid", {"baselineGrid": {"type": "number"}, "baselineOffset": {"type": "number"}, "snapToBaseline": {"type": "boolean"}}, ["baselineGrid", "baselineOffset", "snapToBaseline"]))
    if "SetFrameFlags" not in titles:
        doc["oneOf"].append(entry("SetFrameFlags", "set-frame-flags", {"pageId": {"type": "string"}, "frameId": {"type": "string"}, "locked": {"type": "boolean"}, "visible": {"type": "boolean"}}, ["pageId", "frameId"]))
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


def aggregate_json(text):
    doc = json.loads(text)
    refs = {entry["$ref"] for entry in doc["oneOf"]}
    for kind in ("rotate-frame", "update-grid", "set-frame-flags"):
        ref = f"https://json.schemas.assets.semio-tech.com/s/layout/layout/mutation/{kind}/schema.json"
        if ref not in refs:
            doc["oneOf"].append({"$ref": ref})
    doc["oneOf"].sort(key=lambda entry: entry["$ref"])
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


def aggregate_graphql(text):
    text = text.replace("`LayoutMutation` enum and its 25 per-verb leaf structs", "`LayoutMutation` enum and its 28 per-verb leaf structs")
    text = once(text, "type ChangeFrameFill {\n", "type RotateFrame {\n  page_id: String!\n  frame_id: String!\n  new_rotation: Float!\n}\n\ntype ChangeFrameFill {\n")
    text = once(text, "type ChangeFrameColumns {\n  page_id: String!\n  frame_id: String!\n  new_columns: Int!\n}\n", "type ChangeFrameColumns {\n  page_id: String!\n  frame_id: String!\n  new_columns: Int!\n}\n\ntype UpdateGrid {\n  baseline_grid: Float!\n  baseline_offset: Float!\n  snap_to_baseline: Boolean!\n}\n\ntype SetFrameFlags {\n  page_id: String!\n  frame_id: String!\n  locked: Boolean\n  visible: Boolean\n}\n")
    text = once(text, "  | ResizeFrame\n  | ChangeFrameFill\n", "  | ResizeFrame\n  | RotateFrame\n  | ChangeFrameFill\n")
    return once(text, "  | ChangeFrameColumns\n", "  | ChangeFrameColumns\n  | UpdateGrid\n  | SetFrameFlags\n")


def aggregate_ts(text):
    text = text.replace("`LayoutMutation` enum and its 25 per-verb leaf structs", "`LayoutMutation` enum and its 28 per-verb leaf structs").replace("on every one of its 25 leaf structs", "on every one of its 28 leaf structs").replace("None of the 25 leaf structs", "None of the 28 leaf structs")
    text = once(text, "export interface ChangeFrameFill {\n", "export interface RotateFrame {\n  page_id: string;\n  frame_id: string;\n  new_rotation: number;\n}\n\nexport interface ChangeFrameFill {\n")
    text = once(text, "export interface ChangeFrameColumns {\n  page_id: string;\n  frame_id: string;\n  new_columns: number;\n}\n", "export interface ChangeFrameColumns {\n  page_id: string;\n  frame_id: string;\n  new_columns: number;\n}\n\nexport interface UpdateGrid {\n  baseline_grid: number;\n  baseline_offset: number;\n  snap_to_baseline: boolean;\n}\n\nexport interface SetFrameFlags {\n  page_id: string;\n  frame_id: string;\n  locked?: boolean;\n  visible?: boolean;\n}\n")
    text = once(text, "  | { ResizeFrame: ResizeFrame }\n", "  | { ResizeFrame: ResizeFrame }\n  | { RotateFrame: RotateFrame }\n")
    return once(text, "  | { ChangeFrameColumns: ChangeFrameColumns };\n", "  | { ChangeFrameColumns: ChangeFrameColumns }\n  | { UpdateGrid: UpdateGrid }\n  | { SetFrameFlags: SetFrameFlags };\n")


rewrite("📝️text/📖️.grammar.semio", grammar)
rewrite("📝️text/🔤️.ebnf", ebnf)
rewrite("📝️text/🅰️.g4", g4)
rewrite("📝️text/🛰️.proto", proto)
rewrite("📝️text/🔗️.graphql", text_graphql)
rewrite("📝️text/🔣️.json", text_json)
rewrite("🔣️.json", aggregate_json)
rewrite("🔗️.graphql", aggregate_graphql)
rewrite("🟦️.ts", aggregate_ts)
print(("would change: " if DRY else "changed: ") + (", ".join(changed) or "nothing"))
