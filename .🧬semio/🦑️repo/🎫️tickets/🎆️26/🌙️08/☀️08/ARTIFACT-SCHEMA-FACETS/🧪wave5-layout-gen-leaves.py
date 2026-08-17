#!/usr/bin/env python3
"""Generate layout facet leaves (non-Rust) with matching top-level fields."""
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout")

SNAPSHOT_FIELDS = [
    ("schema", "string", "persistent", True),
    ("name", "string", "persistent", True),
    ("grid", "GridSettings", "persistent", True),
    ("paragraphStyles", "ParagraphStyle[]", "persistent", True),
    ("characterStyles", "CharacterStyle[]", "persistent", True),
    ("stories", "TextStory[]", "persistent", True),
    ("links", "ImageLink[]", "persistent", True),
    ("parentPages", "ParentPage[]", "persistent", True),
    ("spreads", "Spread[]", "persistent", True),
    ("pages", "Page[]", "persistent", True),
    ("printTarget", "string", "persistent", False),
    ("dataFieldsJson", "string", "persistent", False),
]

ARTIFACT_FIELDS = SNAPSHOT_FIELDS + [
    ("selectedIds", "string[]", "shared-ui", True),
    ("activePageId", "string", "local-ui", True),
    ("engagementInput", "string", "local-ui", True),
    ("cameraX", "number", "local-ui", True),
    ("cameraY", "number", "local-ui", True),
    ("cameraZoom", "number", "local-ui", True),
    ("previewCameraX", "number", "local-ui", True),
    ("previewCameraY", "number", "local-ui", True),
    ("previewCameraZoom", "number", "local-ui", True),
    ("dropPreview", "LayoutDropPreviewState", "local-ui", True),
    ("locale", "string", "local-ui", True),
    ("hoveredId", "string", "preview", False),
]

DIFF_FIELDS = [
    ("artifact", "LayoutArtifact", "persistent", False),
    ("schema", "string", "persistent", False),
    ("name", "string", "persistent", False),
    ("grid", "GridSettings", "persistent", False),
    ("paragraphStyles", "LayoutParagraphStylesDelta", "persistent", False),
    ("characterStyles", "LayoutCharacterStylesDelta", "persistent", False),
    ("stories", "LayoutStoriesDelta", "persistent", False),
    ("links", "LayoutLinksDelta", "persistent", False),
    ("parentPages", "LayoutParentPagesDelta", "persistent", False),
    ("spreads", "LayoutSpreadsDelta", "persistent", False),
    ("pages", "LayoutPagesDelta", "persistent", False),
    ("printTarget", "string|null", "persistent", False),
    ("dataFieldsJson", "string|null", "persistent", False),
    ("selectedIds", "LayoutStringList", "shared-ui", False),
    ("activePageId", "string", "local-ui", False),
    ("engagementInput", "string", "local-ui", False),
    ("cameraX", "number", "local-ui", False),
    ("cameraY", "number", "local-ui", False),
    ("cameraZoom", "number", "local-ui", False),
    ("previewCameraX", "number", "local-ui", False),
    ("previewCameraY", "number", "local-ui", False),
    ("previewCameraZoom", "number", "local-ui", False),
    ("dropPreview", "LayoutDropPreviewState", "local-ui", False),
    ("locale", "string", "local-ui", False),
    ("hoveredId", "string|null", "preview", False),
]

DEFS = {
    "GridSettings": {"type": "object", "additionalProperties": False, "required": ["baselineGrid", "baselineOffset", "snapToBaseline"], "properties": {"baselineGrid": {"type": "number"}, "baselineOffset": {"type": "number"}, "snapToBaseline": {"type": "boolean"}}},
    "LayoutDropPreviewState": {"type": "object", "additionalProperties": False, "required": ["kind", "x", "y"], "properties": {"kind": {"type": "string"}, "x": {"type": "number"}, "y": {"type": "number"}}},
    "LayoutStringList": {"type": "object", "additionalProperties": False, "required": ["values"], "properties": {"values": {"type": "array", "items": {"type": "string"}}}},
    "ParagraphStyle": {"type": "object", "additionalProperties": True},
    "CharacterStyle": {"type": "object", "additionalProperties": True},
    "TextStory": {"type": "object", "additionalProperties": True},
    "ImageLink": {"type": "object", "additionalProperties": True},
    "ParentPage": {"type": "object", "additionalProperties": True},
    "Spread": {"type": "object", "additionalProperties": True},
    "Page": {"type": "object", "additionalProperties": True},
    "LayoutArtifact": {"type": "object", "additionalProperties": True},
    "LayoutParagraphStylesDelta": {"type": "object", "additionalProperties": True},
    "LayoutCharacterStylesDelta": {"type": "object", "additionalProperties": True},
    "LayoutStoriesDelta": {"type": "object", "additionalProperties": True},
    "LayoutLinksDelta": {"type": "object", "additionalProperties": True},
    "LayoutParentPagesDelta": {"type": "object", "additionalProperties": True},
    "LayoutSpreadsDelta": {"type": "object", "additionalProperties": True},
    "LayoutPagesDelta": {"type": "object", "additionalProperties": True},
}


def json_leaf(title: str, facet: str, fields):
    import json
    props = {}
    required = []
    for name, typ, state, req in fields:
        if typ.endswith("[]"):
            base = typ[:-2]
            prop = {"type": "array", "items": {"$ref": f"#/$defs/{base}"} if base in DEFS else {"type": "string"}, "x-semio-state": state}
        elif typ.endswith("|null"):
            base = typ[:-5]
            prop = {"oneOf": [{"type": "null"}, {"$ref": f"#/$defs/{base}"} if base in DEFS else {"type": "string"}], "x-semio-state": state}
        elif typ in DEFS:
            prop = {"$ref": f"#/$defs/{typ}", "x-semio-state": state}
        elif typ == "number":
            prop = {"type": "number", "format": "double", "x-semio-state": state}
        else:
            prop = {"type": "string", "x-semio-state": state}
        props[name] = prop
        if req:
            required.append(name)
    doc = {
        "$id": f"https://semio.tech/schema/s/layout/layout/{facet}.json",
        "title": title,
        "type": "object",
        "additionalProperties": False,
        "required": required,
        "properties": props,
        "$defs": DEFS,
    }
    return json.dumps(doc, indent=2) + "\n"


def ts_leaf(title: str, fields):
    lines = [f"/** 🧬️ Layout {title} schema. */", f"export interface {title} {{"]
    for name, typ, state, req in fields:
        opt = "" if req else "?"
        if typ.endswith("[]"):
            inner = typ[:-2]
            ts = f"{inner}[]"
        elif typ.endswith("|null"):
            ts = f"{typ.replace('|null', '')} | null"
        else:
            ts = typ
        lines.append(f"  /** @state {state} */")
        lines.append(f"  {name}{opt}: {ts};")
    lines.append("}")
    lines.append("")
    lines.append("export interface GridSettings { baselineGrid: number; baselineOffset: number; snapToBaseline: boolean; }")
    lines.append("export interface LayoutDropPreviewState { kind: string; x: number; y: number; }")
    lines.append("export interface LayoutStringList { values: string[]; }")
    return "\n".join(lines) + "\n"


def graphql_preamble():
    return '"""Layout facet schema."""\n'


def graphql_leaf(title: str, fields):
    lines = [graphql_preamble(), f"type {title} {{"]
    for name, typ, state, req in fields:
        gql = "String"
        if typ.endswith("[]"):
            base = typ[:-2]
            gql = f"[{base}!]!" if req else f"[{base}!]"
        elif typ == "number":
            gql = "Float!"
        elif typ in DEFS and typ not in ("GridSettings", "LayoutDropPreviewState", "LayoutStringList"):
            gql = f"{typ}!"
        elif typ in ("GridSettings", "LayoutDropPreviewState", "LayoutStringList"):
            gql = f"{typ}!"
        elif not req:
            gql = "String"
        else:
            gql = "String!"
        if not req and not typ.endswith("[]") and typ != "number":
            gql = gql.replace("!", "") if gql.endswith("!") else gql
        lines.append(f"  {name}: {gql} @state(class: {state})")
    lines.append("}")
    lines.append("")
    lines.append("type GridSettings { baselineGrid: Float! baselineOffset: Float! snapToBaseline: Boolean! }")
    lines.append("type LayoutDropPreviewState { kind: String! x: Float! y: Float! }")
    lines.append("type LayoutStringList { values: [String!]! }")
    return "\n".join(lines) + "\n"


def proto_leaf(title: str, fields):
    lines = ["syntax = \"proto3\";", "package semio.s.layout.layout;", "", f"message {title} {{"]
    n = 1
    for name, typ, state, _req in fields:
        snake = "".join("_" + c.lower() if c.isupper() else c for c in name).lstrip("_")
        if typ.endswith("[]"):
            lines.append(f"  repeated string {snake} = {n}; // @state {state}")
        elif typ == "number":
            lines.append(f"  optional double {snake} = {n}; // @state {state}")
        else:
            lines.append(f"  optional string {snake} = {n}; // @state {state}")
        n += 1
    lines.append("}")
    return "\n".join(lines) + "\n"


FACETS = [
    ("🧬️schema", "LayoutArtifact", "artifact", ARTIFACT_FIELDS),
    ("📸️snapshot/🧬️schema", "LayoutSnapshot", "snapshot", SNAPSHOT_FIELDS),
    ("🔺️diff/🧬️schema", "LayoutDiff", "diff", DIFF_FIELDS),
]

for rel, title, facet, fields in FACETS:
    d = ROOT / rel
    d.mkdir(parents=True, exist_ok=True)
    (d / "🔣️component.json").write_text(json_leaf(title, facet, fields))
    (d / "🟦️component.ts").write_text(ts_leaf(title, fields))
    (d / "🔗️component.graphql").write_text(graphql_leaf(title, fields))
    (d / "🛰️component.proto").write_text(proto_leaf(title, fields))
print("wrote 12 leaves")
