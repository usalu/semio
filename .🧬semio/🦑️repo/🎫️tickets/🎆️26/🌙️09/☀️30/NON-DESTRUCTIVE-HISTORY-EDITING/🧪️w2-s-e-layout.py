#!/usr/bin/env python3
"""📏️ W2-S-E layout parity: the `LayoutMutation` leaves wire camelCase, and every surface follows the Rust struct.

Idempotent steps, each re-reading its file right before writing it:

- `rust`      adds `#[value(rename_all = "camelCase")]` + the test-serde twin to every leaf `#[derive(MutationLeaf)]` struct;
- `fixtures`  renames the leaf payload keys of every committed `🦠️mutation/🔣️.json` (nested snapshot records are camelCase already);
- `schemas`   `Option` fields admit `null`, and the 14 stub leaves get full payload schemas with en/de `x-semio-ui`;
- `surfaces`  regenerates the aggregate GraphQL leaf types, the TS twin leaf interfaces, the text-facet JSON Schema, GraphQL
              inputs, proto messages and the positional grammar productions (semio / ANTLR / EBNF) from the Rust structs.

Usage: python3 🧪️w2-s-e-layout.py [--apply] rust|fixtures|schemas|surfaces…
"""
import importlib.util
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
SUBSET = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUTATIONS = f"{SUBSET}/🧬️schema/🧬️mutations"
ID_BASE = "https://json.schemas.assets.semio-tech.com/s/layout/layout/mutation"
APPLY = "--apply" in sys.argv
_probe = importlib.util.spec_from_file_location("probe", os.path.join(os.path.dirname(os.path.abspath(__file__)), "🧪️w2-s-e-leaf-parity.py"))
probe = importlib.util.module_from_spec(_probe)
_probe.loader.exec_module(probe)


def read(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return handle.read()


def write(path, before, after):
    if before == after:
        return False
    if APPLY:
        if read(path) != before:
            raise SystemExit(f"[w2-s-e] {path} changed while editing; rerun")
        with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
            handle.write(after)
    print(f"[w2-s-e] {'wrote' if APPLY else 'would write'} {path}")
    return True


def leaves():
    """(slug directory, kebab kind, variant, struct fields) of every layout leaf, in `LayoutMutation` declaration order."""
    aggregate = read(f"{MUTATIONS}/🦀️.rs")
    order = re.findall(r"^\s{4}(\w+)\((\w+)::\w+\),$", aggregate, re.M)
    by_variant = {}
    for entry in os.listdir(os.path.join(REPO, MUTATIONS)):
        descriptor = os.path.join(REPO, MUTATIONS, entry, "🔣️.json")
        if not os.path.isfile(descriptor):
            continue
        meta = json.load(open(descriptor, encoding="utf-8"))
        if isinstance(meta, dict) and isinstance(meta.get("aggregateVariant"), str):
            struct = probe.structs(read(f"{MUTATIONS}/{entry}/🦀️.rs"))[meta["aggregateVariant"]]
            by_variant[meta["aggregateVariant"]] = (entry, meta["semanticKind"], meta["aggregateVariant"], struct["fields"])
    return [by_variant[variant] for variant, _module in order]


#region 🦀️Rust
def rust():
    for entry, _kind, variant, _fields in leaves():
        path = f"{MUTATIONS}/{entry}/🦀️.rs"
        before = read(path)
        head = f"#[mutation_leaf(contract = ::protocol)]\npub struct {variant} {{"
        after = before
        if '#[cfg_attr(test, serde(rename_all = "camelCase"))]\n#[mutation_leaf' not in after:
            after = after.replace(f"#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n{head}", f'#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n#[cfg_attr(test, serde(rename_all = "camelCase"))]\n{head}')
        if f'#[mutation_leaf(contract = ::protocol)]\n#[value(rename_all = "camelCase")]\npub struct {variant} {{' not in after:
            after = after.replace(head, f'#[mutation_leaf(contract = ::protocol)]\n#[value(rename_all = "camelCase")]\npub struct {variant} {{')
        if after.count('rename_all = "camelCase"') != 2:
            raise SystemExit(f"[w2-s-e] {path}: the leaf struct attribute block has an unexpected shape")
        write(path, before, after)
#endregion 🦀️Rust


#region 🧫️Fixtures
def fixtures():
    root = os.path.join(REPO, SUBSET, "🧫️fixtures", "🧬️mutations")
    for directory, _dirs, files in os.walk(root):
        if not directory.endswith("🦠️mutation") or "🔣️.json" not in files:
            continue
        path = os.path.relpath(os.path.join(directory, "🔣️.json"), REPO)
        before = read(path)
        wire = json.loads(before)
        ((variant, payload),) = wire.items()
        renamed = {variant: {probe.camel(key): value for key, value in payload.items()}}
        write(path, before, json.dumps(renamed, indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else ""))
#endregion 🧫️Fixtures


#region 🧬️Schemas
def ui(widget, en, de, group, order, role="value", **extra):
    node = {"widget": widget, "role": role, "label": {"en": en, "de": de}} if widget else {"role": role, "label": {"en": en, "de": de}}
    description = extra.pop("description", None)
    if description is not None:
        node["description"] = {"en": description[0], "de": description[1]}
    node.update(extra)
    node["group"], node["order"] = group, order
    return node


def ref(kind, en, de, group, order, role="target", frame=False, description=None):
    node = ui("reference", en, de, group, order, role=role, **({} if description is None else {"description": description}))
    node["ref"] = {"kind": kind, "domain": "elements", "granularity": "element"} if frame else {"kind": kind}
    node["group"], node["order"] = node.pop("group"), node.pop("order")
    return node


def mm(en, de, group, order):
    return {"type": "number", "x-semio-ui": ui("stepper", en, de, group, order, unit="mm", step=1, precision=1)}


PAGE = {"type": "string", "x-semio-ui": ref("page", "Page", "Seite", "target", 10)}
FRAME = {"type": "string", "x-semio-ui": ref("frame", "Frame", "Rahmen", "target", 20, frame=True)}
ROTATION = {"type": "number", "x-semio-ui": {"widget": "dial", "role": "value", "label": {"en": "Rotation", "de": "Drehung"}, "unit": "rad", "displayUnit": "deg", "displayFactor": 57.29577951308232, "step": 0.017453292519943295, "softMin": -3.141592653589793, "softMax": 3.141592653589793, "snaps": [-3.141592653589793, -1.5707963267948966, 0, 1.5707963267948966, 3.141592653589793], "group": "geometry", "order": 50}}
EMPTY_INHERITS = ("Empty inherits the paragraph style.", "Leer übernimmt das Absatzformat.")
STUBS = {
    "create-character-style": ({
        "id": {"type": "string", "x-semio-ui": ui("text", "Character style ID", "Zeichenformat-ID", "identity", 10, description=("Id of the new character style.", "ID des neuen Zeichenformats."))},
        "name": {"type": ["string", "null"], "x-semio-ui": ui("text", "Style name", "Formatname", "identity", 20, description=("Empty leaves the style unnamed.", "Leer lässt das Format unbenannt."))},
    }, ["id"], None),
    "delete-character-style": ({
        "id": {"type": "string", "x-semio-ui": ref("characterStyle", "Character style", "Zeichenformat", "target", 10)},
    }, ["id"], None),
    "update-character-style": ({
        "id": {"type": "string", "x-semio-ui": ref("characterStyle", "Character style", "Zeichenformat", "target", 10)},
        "name": {"type": ["string", "null"], "x-semio-ui": ui("text", "Style name", "Formatname", "identity", 20, description=("Empty leaves the style unnamed.", "Leer lässt das Format unbenannt."))},
        "fontFamily": {"type": ["string", "null"], "x-semio-ui": ui("text", "Font family", "Schriftfamilie", "type", 30, description=EMPTY_INHERITS)},
        "fontSize": {"type": ["number", "null"], "x-semio-ui": ui("stepper", "Font size", "Schriftgrad", "type", 40, description=EMPTY_INHERITS, unit="pt", step=0.5, precision=1)},
        "fontWeight": {"type": ["integer", "null"], "minimum": 0, "x-semio-ui": ui("stepper", "Font weight", "Schriftstärke", "type", 50, description=("CSS weight, e.g. 400 regular, 700 bold; empty inherits the paragraph style.", "CSS-Stärke, z. B. 400 normal, 700 fett; leer übernimmt das Absatzformat."), step=100, precision=0)},
        "italic": {"type": ["boolean", "null"], "x-semio-ui": ui("toggle", "Italic", "Kursiv", "type", 60, description=EMPTY_INHERITS)},
        "color": {"type": ["array", "null"], "items": {"type": "number"}, "minItems": 4, "maxItems": 4, "x-semio-ui": ui("vector", "Text color", "Textfarbe", "appearance", 70, description=("RGBA, each channel 0 to 1; empty inherits the paragraph style.", "RGBA, jeder Kanal 0 bis 1; leer übernimmt das Absatzformat."))},
        "tracking": {"type": ["number", "null"], "x-semio-ui": ui("stepper", "Tracking", "Laufweite", "type", 80, description=EMPTY_INHERITS, step=1, precision=0)},
    }, ["id"], None),
    "update-parent-page": ({
        "id": {"type": "string", "x-semio-ui": ref("parentPage", "Parent page", "Mustervorlage", "target", 10)},
        "name": {"type": "string", "x-semio-ui": ui("text", "Name", "Name", "identity", 20)},
        "width": mm("Width", "Breite", "geometry", 30),
        "height": mm("Height", "Höhe", "geometry", 40),
    }, ["id", "name", "width", "height"], None),
    "update-spread": ({
        "id": {"type": "string", "x-semio-ui": ref("spread", "Spread", "Druckbogen", "target", 10)},
        "name": {"type": "string", "x-semio-ui": ui("text", "Name", "Name", "identity", 20)},
    }, ["id", "name"], None),
    "set-page-parent": ({
        "id": dict(PAGE),
        "parentPageId": {"type": ["string", "null"], "x-semio-ui": ref("parentPage", "Parent page", "Mustervorlage", "source", 20, role="value", description=("Empty detaches the page from its parent page.", "Leer löst die Seite von ihrer Mustervorlage."))},
    }, ["id"], None),
    "set-page-guides": ({
        "id": dict(PAGE),
        "guides": {"type": "array", "items": {"$ref": "#/$defs/LayoutRect"}, "x-semio-ui": ui(None, "Guides", "Hilfslinien", "record", 20, description=("Replaces every guide on the page.", "Ersetzt alle Hilfslinien der Seite."))},
    }, ["id", "guides"], {
        "LayoutRect": {"title": "LayoutRect", "type": "object", "additionalProperties": False, "required": ["x", "y", "w", "h"], "properties": {
            "x": mm("X", "X", "position", 10), "y": mm("Y", "Y", "position", 20), "w": mm("Width", "Breite", "geometry", 30), "h": mm("Height", "Höhe", "geometry", 40)}},
    }),
    "set-story-runs": ({
        "id": {"type": "string", "x-semio-ui": ref("story", "Story", "Textfluss", "target", 10)},
        "runs": {"type": "array", "items": {"$ref": "#/$defs/TextStyleRun"}, "x-semio-ui": ui(None, "Style runs", "Formatbereiche", "record", 20, description=("Replaces every style run of the story.", "Ersetzt alle Formatbereiche des Textflusses."))},
    }, ["id", "runs"], {
        "TextStyleRun": {"title": "TextStyleRun", "type": "object", "additionalProperties": False, "required": ["start", "end"], "properties": {
            "start": {"type": "integer", "minimum": 0, "x-semio-ui": ui("stepper", "Start", "Beginn", "range", 10, description=("First character offset of the run.", "Zeichenposition, an der der Bereich beginnt."), step=1, precision=0)},
            "end": {"type": "integer", "minimum": 0, "x-semio-ui": ui("stepper", "End", "Ende", "range", 20, description=("Character offset after the run.", "Zeichenposition nach dem Bereich."), step=1, precision=0)},
            "paragraphStyleId": {"type": ["string", "null"], "x-semio-ui": ref("paragraphStyle", "Paragraph style", "Absatzformat", "style", 30, role="value")},
            "characterStyleId": {"type": ["string", "null"], "x-semio-ui": ref("characterStyle", "Character style", "Zeichenformat", "style", 40, role="value")},
        }},
    }),
    "update-link": ({
        "id": {"type": "string", "x-semio-ui": ref("link", "Link", "Verknüpfung", "target", 10)},
        "width": {"type": "integer", "minimum": 0, "x-semio-ui": ui("stepper", "Width", "Breite", "print", 20, description=("Image width in pixels.", "Bildbreite in Pixeln."), unit="px", step=1, precision=0)},
        "height": {"type": "integer", "minimum": 0, "x-semio-ui": ui("stepper", "Height", "Höhe", "print", 30, description=("Image height in pixels.", "Bildhöhe in Pixeln."), unit="px", step=1, precision=0)},
        "dpi": {"type": "integer", "minimum": 0, "x-semio-ui": ui("stepper", "Resolution", "Auflösung", "print", 40, unit="dpi", step=1, precision=0, softMin=72, softMax=2400)},
        "colorProfile": {"type": ["string", "null"], "x-semio-ui": ui("text", "Color profile", "Farbprofil", "print", 50, description=("ICC profile name; empty uses the document profile.", "Name des ICC-Profils; leer verwendet das Dokumentprofil."))},
    }, ["id", "width", "height", "dpi"], None),
    "set-page-overrides": ({
        "id": dict(PAGE),
        "overrides": {"type": "array", "items": {"$ref": "#/$defs/PageOverride"}, "x-semio-ui": ui(None, "Overrides", "Abweichungen", "record", 20, description=("Replaces every override of a parent-page frame on this page.", "Ersetzt alle Abweichungen von Rahmen der Mustervorlage auf dieser Seite."))},
    }, ["id", "overrides"], {
        "PageOverride": {"title": "PageOverride", "type": "object", "additionalProperties": False, "required": ["objectId"], "properties": {
            "objectId": {"type": "string", "x-semio-ui": ref("frame", "Frame", "Rahmen", "target", 10, role="value", frame=True, description=("The parent-page frame this override changes.", "Der Rahmen der Mustervorlage, den diese Abweichung ändert."))},
            "bounds": {"anyOf": [{"$ref": "#/$defs/LayoutBounds"}, {"type": "null"}], "x-semio-ui": ui(None, "Bounds", "Begrenzung", "geometry", 20, description=("Empty keeps the parent-page bounds.", "Leer übernimmt die Begrenzung der Mustervorlage."))},
            "visible": {"type": ["boolean", "null"], "x-semio-ui": ui("toggle", "Visible", "Sichtbar", "state", 30)},
            "locked": {"type": ["boolean", "null"], "x-semio-ui": ui("toggle", "Locked", "Gesperrt", "state", 40)},
        }},
        "LayoutBounds": {"title": "LayoutBounds", "type": "object", "additionalProperties": False, "required": ["x", "y", "w", "h", "rotation"], "properties": {
            "x": mm("X", "X", "position", 10), "y": mm("Y", "Y", "position", 20), "w": mm("Width", "Breite", "geometry", 30), "h": mm("Height", "Höhe", "geometry", 40), "rotation": ROTATION}},
    }),
    "create-layer": ({
        "pageId": dict(PAGE),
        "id": {"type": "string", "x-semio-ui": ui("text", "Layer ID", "Ebenen-ID", "identity", 20, description=("Id of the new layer.", "ID der neuen Ebene."))},
        "name": {"type": "string", "x-semio-ui": ui("text", "Layer name", "Ebenenname", "identity", 30)},
        "remove": {"type": "boolean", "x-semio-ui": ui("toggle", "Remove layer", "Ebene entfernen", "state", 40, description=("On removes the named empty layer instead of creating it; this is the inverse step.", "Ein entfernt die genannte leere Ebene, statt sie anzulegen; das ist der Umkehrschritt."))},
    }, ["pageId", "id", "name"], None),
    "set-frame-layer": ({
        "pageId": dict(PAGE),
        "frameId": dict(FRAME),
        "layerId": {"type": "string", "x-semio-ui": ref("layer", "Layer", "Ebene", "target", 30, role="value", description=("The layer of the same page the frame moves to.", "Die Ebene derselben Seite, auf die der Rahmen wechselt."))},
    }, ["pageId", "frameId", "layerId"], None),
    "set-drawing-text": ({
        "index": {"type": "integer", "minimum": 0, "x-semio-ui": ui("stepper", "Label index", "Beschriftungsindex", "target", 10, description=("Position of the text label in the imported plan, counted from 0.", "Position der Beschriftung im importierten Plan, ab 0 gezählt."), step=1, precision=0, softMin=0, softMax=31)},
        "text": {"type": "string", "x-semio-ui": ui("text", "Text", "Text", "text", 20)},
    }, ["index", "text"], None),
    "reorder-frame": ({
        "pageId": dict(PAGE),
        "frameId": dict(FRAME),
        "forward": {"type": "boolean", "x-semio-ui": ui("toggle", "Bring forward", "Nach vorn holen", "order", 30, description=("On moves the frame one step forward in the paint order, off one step backward.", "Ein holt den Rahmen in der Zeichenreihenfolge einen Schritt nach vorn, aus schiebt ihn einen Schritt zurück."))},
    }, ["pageId", "frameId", "forward"], None),
}
NULLABLE = {
    "change-print-target": [("properties", "newPrintTarget")],
    "change-data-fields": [("properties", "newJson")],
    "create-page": [("properties", "index")],
    "create-story": [("properties", "index"), ("properties", "story", "properties", "styleRuns", "items", "properties", "paragraphStyleId"), ("properties", "story", "properties", "styleRuns", "items", "properties", "characterStyleId")],
    "create-link": [("properties", "index"), ("properties", "link", "properties", "colorProfile"), ("properties", "link", "properties", "state"), ("properties", "link", "properties", "proxyDataUrl")],
    "create-frame": [("properties", "index"), ("properties", "layerId")],
    "change-frame-fill": [("properties", "newFill")],
    "change-frame-stroke": [("properties", "newStroke")],
    "set-frame-flags": [("properties", "locked"), ("properties", "visible")],
    "update-text-frame": [("properties", "threadNext")],
}


def nullable(node):
    types = node["type"]
    node["type"] = types if isinstance(types, list) and "null" in types else ([types, "null"] if isinstance(types, str) else types + ["null"])


def schemas():
    for entry, kind, variant, _fields in leaves():
        path = f"{MUTATIONS}/{entry}/🧬️schema/🔣️.json"
        before = read(path)
        document = json.loads(before)
        if kind in STUBS:
            properties, required, defs = STUBS[kind]
            document = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"{ID_BASE}/{kind}/schema.json", "title": variant, "type": "object", "additionalProperties": False, "required": required, "properties": properties}
            if defs is not None:
                document["$defs"] = defs
        for pointer in NULLABLE.get(kind, []):
            node = document
            for segment in pointer:
                node = node[segment]
            nullable(node)
        rendered = json.dumps(document, indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else "")
        if document == json.loads(before):
            continue
        if json.dumps(json.loads(before), indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else "") != before and kind not in STUBS:
            rendered = before
            for pointer in NULLABLE.get(kind, []):
                anchor = rendered.index(f'"{pointer[-1]}": {{')
                typed = re.compile(r'"type": "(\w+)"').search(rendered, anchor)
                rendered = rendered[:typed.start()] + f'"type": ["{typed.group(1)}", "null"]' + rendered[typed.end():]
            if json.loads(rendered) != document:
                raise SystemExit(f"[w2-s-e] {path} is hand-formatted and the in-place edit diverges; edit it by hand")
        write(path, before, rendered)
#endregion 🧬️Schemas


#region 🗣️Surfaces
def base(ty):
    option = ty.startswith("Option<")
    return option, (ty[7:-1].strip() if option else ty)


def gql(ty, output):
    option, inner = base(ty)
    scalar = {"String": "String", "f64": "Float", "f32": "Float", "bool": "Boolean", "u32": "Int", "usize": "Int"}.get(inner)
    if scalar is None and inner.startswith("["):
        scalar = "[Float!]"
    elif scalar is None and inner.startswith("Vec<"):
        scalar = f"[{inner[4:-1]}!]" if output else "Bytes"
    elif scalar is None:
        scalar = inner if output else "Bytes"
    if not output and inner.startswith("Vec<"):
        scalar = "Bytes"
    if not output and inner in {"u32", "usize"}:
        scalar = "Float"
    return scalar if option else f"{scalar}!"


def proto(ty):
    option, inner = base(ty)
    scalar = {"String": "string", "f64": "double", "f32": "double", "bool": "bool", "u32": "uint32", "usize": "uint32"}.get(inner, "bytes")
    return f"optional {scalar}" if option else scalar


def ts(ty):
    option, inner = base(ty)
    scalar = {"String": "string", "f64": "number", "f32": "number", "bool": "boolean", "u32": "number", "usize": "number"}.get(inner)
    if scalar is None and inner.startswith("["):
        scalar = "[number, number, number, number]"
    elif scalar is None and inner.startswith("Vec<"):
        scalar = f"{inner[4:-1]}[]"
    elif scalar is None:
        scalar = inner
    return f"{scalar} | null" if option else scalar


def text_json(ty):
    option, inner = base(ty)
    scalar = {"String": "string", "f64": "number", "f32": "number", "bool": "boolean", "u32": "integer", "usize": "integer"}.get(inner)
    if scalar is None:
        scalar = "array" if inner.startswith("[") or inner.startswith("Vec<") else "object"
    return {"type": [scalar, "null"] if option else scalar}


def grammar_token(field):
    option, inner = base(field["type"])
    token = "number" if inner in {"f64", "f32", "u32", "usize"} else "boolean" if inner == "bool" else "text" if inner == "String" and not (field["ident"] == "id" or field["ident"].endswith("_id") or field["ident"] == "thread_next") else "id" if inner == "String" else "block"
    return token, option or field["default"]


def surfaces():
    table = leaves()
    stubs = {kind for _entry, kind, _variant, _fields in table if kind in STUBS}

    graphql_path = f"{MUTATIONS}/🔗️.graphql"
    before = read(graphql_path)
    start, end = before.index("type RenameLayout {"), before.index("union LayoutMutation =")
    types = "".join(f"type {variant} {{\n" + "".join(f"  {field['wire']}: {gql(field['type'], True)}\n" for field in fields) + "}\n\n" for _e, _k, variant, fields in table)
    write(graphql_path, before, before[:start] + types + before[end:])

    ts_path = f"{MUTATIONS}/🟦️.ts"
    before = read(ts_path)
    start, end = before.index("//#region 🔖️Leaves\n") + len("//#region 🔖️Leaves\n"), before.index("//#endregion 🔖️Leaves")
    interfaces = "\n".join(f"export interface {variant} {{\n" + "".join(f"  {field['wire']}{'?' if field['default'] else ''}: {ts(field['type'])};\n" for field in fields) + "}\n" for _e, _k, variant, fields in table)
    after = before[:start] + interfaces + before[end:]
    header_end = after.index("*/") + 2
    header = ("/** 🧬️ LayoutMutation — closed semantic mutation vocabulary for the layout document, mirrors\n"
              " *  `🧬️mutations/🦀️.rs`'s `LayoutMutation` enum and its per-verb leaf structs field-for-field\n"
              " *  (`.../🧬️mutations/<verb-folder>/🦀️.rs`). The enum carries no `#[value(tag)]`, so it wires\n"
              " *  EXTERNALLY TAGGED: `{ \"<PascalCaseVariantName>\": { ...leaf fields } }`. Every leaf struct carries\n"
              " *  `#[value(rename_all = \"camelCase\")]`, so its fields wire camelCase (`{\"ChangePageWidth\":\n"
              " *  {\"id\":\"page-1\",\"newWidth\":240.0}}`); an `Option` field wires `null` when absent. */")
    write(ts_path, before, header + after[header_end:])

    text_json_path = f"{MUTATIONS}/📝️text/🔣️.json"
    before = read(text_json_path)
    document = json.loads(before)
    document["oneOf"] = [{"type": "object", "title": variant, "properties": {"kind": {"const": kind}, **{field["wire"]: text_json(field["type"]) for field in fields}}, "required": ["kind"] + [field["wire"] for field in fields if not base(field["type"])[0] and not field["default"]]} for _e, kind, variant, fields in table]
    write(text_json_path, before, json.dumps(document, indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else ""))

    text_graphql_path = f"{MUTATIONS}/📝️text/🔗️.graphql"
    before = read(text_graphql_path)
    start, end = before.index("input RenameLayout"), before.index("union LayoutMutation =")
    inputs = "".join(f"input {variant} {{ " + " ".join(f"{field['wire']}: {gql(field['type'], False)}" for field in fields) + " }\n" for _e, _k, variant, fields in table)
    write(text_graphql_path, before, before[:start] + inputs + "\n" + before[end:])

    proto_path = f"{MUTATIONS}/📝️text/🛰️.proto"
    before = read(proto_path)
    start, end = before.index("message RenameLayout {"), before.index("message LayoutMutation {")
    messages = "".join(f"message {variant} {{\n" + "".join(f"  {proto(field['type'])} {field['ident']} = {index};\n" for index, field in enumerate(fields, 1)) + "}\n" for _e, _k, variant, fields in table)
    write(proto_path, before, before[:start] + messages + "\n" + before[end:])

    grammars = (
        (f"{MUTATIONS}/📝️text/📖️.grammar.semio", lambda kind: f'{kind} = "{kind}"', lambda tokens: "".join(f" SP {token}{'?' if option else ''}" for token, option in tokens)),
        (f"{MUTATIONS}/📝️text/🅰️.g4", lambda kind: f"{probe.camel(kind.replace('-', '_'))}: '{kind}'", lambda tokens: "".join(f" SP {token}{'?' if option else ''}" for token, option in tokens) + " ;"),
        (f"{MUTATIONS}/📝️text/🔤️.ebnf", lambda kind: f"{kind.replace('-', ' ')} = '{kind}'", lambda tokens: "".join(f", space, {'[ ' + token + ' ]' if option else token}" for token, option in tokens) + " ;"),
    )
    for path, head, tail in grammars:
        before = read(path)
        after = before
        for _e, kind, _variant, fields in table:
            if kind not in stubs:
                continue
            prefix = head(kind)
            line = prefix + tail([grammar_token(field) for field in fields])
            matches = [existing for existing in after.splitlines() if existing.startswith(prefix + " ") or existing.startswith(prefix + ",")]
            if len(matches) != 1:
                raise SystemExit(f"[w2-s-e] {path}: {len(matches)} productions for {kind}")
            after = after.replace(matches[0], line)
        write(path, before, after)
#endregion 🗣️Surfaces


if __name__ == "__main__":
    for step in [arg for arg in sys.argv[1:] if arg != "--apply"]:
        {"rust": rust, "fixtures": fixtures, "schemas": schemas, "surfaces": surfaces}[step]()
