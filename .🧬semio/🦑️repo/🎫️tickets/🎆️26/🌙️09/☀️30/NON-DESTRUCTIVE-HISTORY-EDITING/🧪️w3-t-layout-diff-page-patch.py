#!/usr/bin/env python3
"""🩹️ W3-T-LAYOUT: brings the layout diff schema's `PagePatch`/`FramePatch` definitions (`🧬️schema/🔺️diff/🔣️.json`) up to
the Rust structs field for field — `frames_patched` (every field-patched frame of the page, in page order) replaces
`frame_patched`, and the members the definitions had drifted from (`layer_patched`, `parent_page_id`, `guides`, `overrides`,
`layer_added`, `layer_removed`, `frame_layer`, `frame_order`; frame `story_id`, `thread_next`, `inset_*`) are declared with
the Rust decode rule (a `#[value(default)]` member is optional); the story, link, paragraph-style, character-style,
parent-page and spread patches follow their Rust structs the same way — then validates EVERY page patch of every committed
`🔺️diff/🔣️.json` against the definition with the third-party `jsonschema` validator.

Usage: .venv/bin/python 🧪️w3-t-layout-diff-page-patch.py [--apply]
"""
import json
import os
import sys

import jsonschema

SUBSET = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCHEMA = f"{SUBSET}/🧬️schema/🔺️diff/🔣️.json"


def nullable(node):
    return {"anyOf": [node, {"type": "null"}]}


NUMBER, STRING, BOOLEAN = {"type": "number"}, {"type": "string"}, {"type": "boolean"}
RGBA = {"type": "array", "minItems": 4, "maxItems": 4, "items": {"type": "number"}}
COUNT = {"type": "integer", "minimum": 0}
FRAME_PATCH = [("x", NUMBER, True), ("y", NUMBER, True), ("width", NUMBER, True), ("height", NUMBER, True), ("rotation", NUMBER, False), ("fill", RGBA, True), ("stroke", RGBA, True), ("wrap_mode", STRING, True), ("columns", {"type": "integer", "minimum": 0}, True), ("locked", BOOLEAN, False), ("visible", BOOLEAN, False), ("story_id", STRING, False), ("thread_next", STRING, False), ("inset_x", NUMBER, False), ("inset_y", NUMBER, False), ("inset_width", NUMBER, False), ("inset_height", NUMBER, False)]
PAGE_PATCH = [
    ("name", nullable(STRING), True), ("width", nullable(NUMBER), True), ("height", nullable(NUMBER), True), ("margin_top", nullable(NUMBER), True), ("margin_right", nullable(NUMBER), True), ("margin_bottom", nullable(NUMBER), True), ("margin_left", nullable(NUMBER), True),
    ("columns_count", nullable({"type": "integer", "minimum": 0}), True), ("columns_gutter", nullable(NUMBER), True), ("frame_added", nullable({"$ref": "#/$defs/PageFrameAdded"}), True), ("frame_removed", nullable(STRING), True),
    ("frames_patched", {"type": "array", "items": {"$ref": "#/$defs/PageFramePatched"}}, True), ("layer_patched", nullable({"$ref": "#/$defs/PageLayerPatched"}), False), ("parent_page_id", nullable(STRING), False),
    ("guides", nullable({"type": "array", "items": {"$ref": "#/$defs/LayoutRect"}}), False), ("overrides", nullable({"type": "array", "items": {"$ref": "#/$defs/PageOverride"}}), False), ("layer_added", nullable({"$ref": "#/$defs/Layer"}), False),
    ("layer_removed", nullable(STRING), False), ("frame_layer", nullable({"$ref": "#/$defs/PageFrameLayer"}), False), ("frame_order", nullable({"type": "array", "items": STRING}), False),
]


def record(members, nullable_leaves):
    properties = {name: (nullable(node) if nullable_leaves else node) for name, node, _required in members}
    return {"type": "object", "additionalProperties": False, "required": [name for name, _node, required in members if required], "properties": properties}


def main():
    apply = "--apply" in sys.argv
    before = open(SCHEMA, encoding="utf-8").read()
    schema = json.loads(before)
    defs = schema["$defs"]
    defs["FramePatch"] = record(FRAME_PATCH, True)
    defs["PagePatch"] = record(PAGE_PATCH, False)
    defs["PageLayerPatched"] = record([("layer_id", STRING, True), ("name", nullable(STRING), True), ("visible", nullable(BOOLEAN), True), ("locked", nullable(BOOLEAN), True)], False)
    defs["PageFrameLayer"] = record([("frame_id", STRING, True), ("layer_id", STRING, True)], False)
    defs["TextStoryPatch"] = record([("content", nullable(STRING), True), ("style_runs", nullable({"type": "array", "items": {"$ref": "#/$defs/TextStyleRun"}}), False)], False)
    defs["ImageLinkPatch"] = record([("path", STRING, True), ("width", COUNT, False), ("height", COUNT, False), ("dpi", COUNT, False), ("color_profile", STRING, False)], True)
    defs["ParagraphStylePatch"] = record([("name", STRING, False), ("fontFamily", STRING, False), ("fontSize", NUMBER, False), ("fontWeight", COUNT, False), ("leading", NUMBER, False), ("tracking", NUMBER, False), ("alignment", STRING, False)], True)
    defs["CharacterStylePatch"] = record([("name", STRING, False), ("fontFamily", STRING, False), ("fontSize", NUMBER, False), ("fontWeight", COUNT, False), ("italic", BOOLEAN, False), ("color", RGBA, False), ("tracking", NUMBER, False)], True)
    defs["ParentPagePatch"] = record([("name", STRING, False), ("width", NUMBER, False), ("height", NUMBER, False)], True)
    defs["SpreadPatch"] = record([("name", STRING, False)], True)
    after = json.dumps(schema, indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else "")
    page_patch = jsonschema.Draft7Validator({"$ref": "#/$defs/PagePatch", "$defs": schema["$defs"]})
    checked, failures = 0, 0
    for directory, _dirs, files in os.walk(f"{SUBSET}/🧫️fixtures/🧬️mutations"):
        if not directory.endswith("🔺️diff") or "🔣️.json" not in files:
            continue
        diff = json.load(open(os.path.join(directory, "🔣️.json"), encoding="utf-8"))
        for entry in (diff.get("pages") or {}).get("patched", []):
            checked += 1
            for error in page_patch.iter_errors(entry["patch"]):
                failures += 1
                print(f"[w3-t-layout] {os.path.relpath(directory, SUBSET)}: {error.message}")
    print(f"[w3-t-layout] {checked} committed page patches checked, {failures} failures")
    if failures:
        raise SystemExit(1)
    if after != before:
        if apply:
            open(SCHEMA, "w", encoding="utf-8").write(after)
        print(f"[w3-t-layout] {'wrote' if apply else 'would write'} {os.path.relpath(SCHEMA, SUBSET)}")


if __name__ == "__main__":
    main()
