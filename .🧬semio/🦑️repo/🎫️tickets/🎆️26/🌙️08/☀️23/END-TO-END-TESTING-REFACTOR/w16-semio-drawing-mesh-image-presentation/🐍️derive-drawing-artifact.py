"""🖍️ One-shot derivation of the real complex `stdio.semio.drawing` artifact and its per-kind
mutation payloads for `mutate-semio-drawing`.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 16.

PROVENANCE. Two real committed SVG documents, read by an INDEPENDENT reader written on Python's
standard library (`xml.etree` plus a path-data scanner written from the SVG 1.1 §8.3 command
grammar) — never through this repository's own svg bridge:

* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧫️fixtures/mouse.svg` — the introduction demonstration
  mouse: 8 real paths carrying `M`/`C`/`H`/`V`/`v`/`s`/`Z` commands, a `clipPath` group, and real
  `stroke-width`/`fill-opacity`/`stroke-opacity` presentation attributes.
* `…/🎨️svg/🧫️fixtures/qr-code.svg` — a real 1015x1015 Inkscape QR document: 329 rectangles, each
  inside its own `matrix(0.35,0,0,0.35,tx,ty)` group inside a fill group inside a layer group, plus a
  background layer carrying a real 5 476-byte `data:image/svg+xml;base64` raster reference.

THE MAPPING, stated so it can be checked:
* one semio LAYER per real SVG document, plus the QR file's own hidden background layer, whose
  `display:none` becomes `visible: false` and whose `opacity:0.5` becomes a real style opacity.
* `<g>` becomes a `Group`; its `transform="matrix(a,0,0,d,e,f)"` becomes
  `translation (e,f,0)` + `scale (a,d,1)`, which is exactly what that matrix is. Any other matrix
  form would be refused rather than approximated — none occurs.
* `<path d="…">` becomes a `Path` whose segments are the real commands, with relative commands
  resolved to absolute, `H`/`V` resolved to `lineTo`, and `S`/`T` expanded against the previous
  control point exactly as SVG 1.1 §8.3.6 defines.
* `<rect>` becomes the five-segment closed `Path` its geometry defines.
* `<image>` with a `data:` href becomes an `Image` carrying the decoded bytes verbatim.
* Presentation attributes and `style="…"` declarations become named styles, one per distinct
  combination, named after the document's own `class`/`id` where it has one. `currentColor` resolves
  to black, the initial value CSS gives the `color` property; that is a resolution, not data, and it
  is said here.

The DSL and pack files are written by the case's own independent Python implementation
(`🐍️component.py`), which was first checked to reproduce the committed `🖍️sketch` example artifact
byte for byte in both encodings and to reach all seventeen committed after-snapshots.
"""

import base64
import importlib.util
import json
import os
import re
import xml.etree.ElementTree as ET

spec = importlib.util.spec_from_file_location("loader", os.path.join(os.path.dirname(os.path.abspath(__file__)), "🐍️load.py"))
loader = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loader)
draw = loader.load("mutate-semio-drawing")

SVG = loader.REPO + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧫️fixtures"
CASE = loader.REPO + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-drawing/🧫️fixtures"

S = "{http://www.w3.org/2000/svg}"
XLINK = "{http://www.w3.org/1999/xlink}"
IDENTITY = {"translation": {"x": 0.0, "y": 0.0, "z": 0.0}, "rotation": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}, "scale": {"x": 1.0, "y": 1.0, "z": 1.0}}

styles = []


def style_named(name: str, declared: dict):
    """🎨️ Registers one distinct presentation combination under a name the source itself carries."""
    entry = {"name": name}
    for key in ("fill", "stroke", "strokeWidth", "opacity"):
        if declared.get(key) is not None:
            entry[key] = declared[key]
    for existing in styles:
        if {k: v for k, v in existing.items() if k != "name"} == {k: v for k, v in entry.items() if k != "name"}:
            return existing["name"]
    styles.append(entry)
    return name


def colour_of(text: str, alpha: float):
    """🎨️ `#rrggbb`, `none`, or `currentColor` — the last resolving to CSS's initial `color`, black."""
    if text is None or text == "none":
        return None
    if text == "currentColor":
        return {"r": 0.0, "g": 0.0, "b": 0.0, "a": draw.to_f32(alpha)}
    match = re.fullmatch(r"#([0-9a-fA-F]{6})", text.strip())
    if match is None:
        raise AssertionError("unresolvable paint %r" % text)
    value = match.group(1)
    channel = lambda pair: draw.to_f32(round(int(value[pair : pair + 2], 16) / 255, 6))
    return {"r": channel(0), "g": channel(2), "b": channel(4), "a": draw.to_f32(alpha)}


def declarations(element: ET.Element) -> dict:
    """🎨️ The element's own presentation attributes merged with its `style="…"` declarations."""
    merged = {key: value for key, value in element.attrib.items() if ":" not in key}
    for piece in element.attrib.get("style", "").split(";"):
        if ":" in piece:
            key, _, value = piece.partition(":")
            merged[key.strip()] = value.strip()
    return merged


def style_for(element: ET.Element, fallback: str):
    """🎨️ The named style an element's paint attributes describe, or nothing when it declares none."""
    declared = declarations(element)
    fill = colour_of(declared.get("fill"), float(declared.get("fill-opacity", 1)))
    stroke = colour_of(declared.get("stroke"), float(declared.get("stroke-opacity", 1)))
    width = float(declared["stroke-width"]) if "stroke-width" in declared else None
    opacity = draw.to_f32(float(declared["opacity"])) if "opacity" in declared else None
    if fill is None and stroke is None and width is None and opacity is None:
        return None
    name = declared.get("class", declared.get("id", fallback))
    return style_named(name, {"fill": fill, "stroke": stroke, "strokeWidth": width, "opacity": opacity})


NUMBER = re.compile(r"[-+]?(?:\d*\.\d+|\d+\.?)(?:[eE][-+]?\d+)?")
COMMAND = re.compile(r"[MmLlHhVvCcSsQqTtAaZz]")


def path_segments(data: str) -> list:
    """✏️ The SVG 1.1 §8.3 path grammar, resolved to this subset's six absolute segment kinds."""
    tokens = []
    at = 0
    while at < len(data):
        if data[at] in " ,\t\r\n":
            at += 1
            continue
        if COMMAND.fullmatch(data[at]):
            tokens.append(data[at])
            at += 1
            continue
        match = NUMBER.match(data, at)
        if match is None:
            raise AssertionError("unreadable path data at %r" % data[at : at + 20])
        tokens.append(float(match.group()))
        at = match.end()
    segments = []
    cursor = [0.0, 0.0]
    start = [0.0, 0.0]
    previous_cubic = None
    previous_quad = None
    index = 0
    command = None
    while index < len(tokens):
        if isinstance(tokens[index], str):
            command = tokens[index]
            index += 1
            if command in "Zz":
                segments.append({"kind": "close"})
                cursor = list(start)
                previous_cubic = previous_quad = None
                continue
        relative = command.islower()
        upper = command.upper()

        def take(count: int):
            nonlocal index
            values = tokens[index : index + count]
            index += count
            return [float(value) for value in values]

        def point(pair):
            return [pair[0] + (cursor[0] if relative else 0.0), pair[1] + (cursor[1] if relative else 0.0)]

        if upper == "M":
            to = point(take(2))
            segments.append({"kind": "moveTo", "to": {"x": to[0], "y": to[1]}})
            cursor = list(to)
            start = list(to)
            command = "l" if relative else "L"
            previous_cubic = previous_quad = None
            continue
        if upper == "L":
            to = point(take(2))
            segments.append({"kind": "lineTo", "to": {"x": to[0], "y": to[1]}})
            cursor = list(to)
            previous_cubic = previous_quad = None
            continue
        if upper == "H":
            value = take(1)[0]
            to = [value + (cursor[0] if relative else 0.0), cursor[1]]
            segments.append({"kind": "lineTo", "to": {"x": to[0], "y": to[1]}})
            cursor = list(to)
            previous_cubic = previous_quad = None
            continue
        if upper == "V":
            value = take(1)[0]
            to = [cursor[0], value + (cursor[1] if relative else 0.0)]
            segments.append({"kind": "lineTo", "to": {"x": to[0], "y": to[1]}})
            cursor = list(to)
            previous_cubic = previous_quad = None
            continue
        if upper == "C":
            values = take(6)
            c1 = point(values[0:2])
            c2 = point(values[2:4])
            to = point(values[4:6])
            segments.append({"kind": "cubicTo", "c1": {"x": c1[0], "y": c1[1]}, "c2": {"x": c2[0], "y": c2[1]}, "to": {"x": to[0], "y": to[1]}})
            cursor = list(to)
            previous_cubic = list(c2)
            previous_quad = None
            continue
        if upper == "S":
            values = take(4)
            mirror = [2 * cursor[0] - previous_cubic[0], 2 * cursor[1] - previous_cubic[1]] if previous_cubic is not None else list(cursor)
            c2 = point(values[0:2])
            to = point(values[2:4])
            segments.append({"kind": "cubicTo", "c1": {"x": mirror[0], "y": mirror[1]}, "c2": {"x": c2[0], "y": c2[1]}, "to": {"x": to[0], "y": to[1]}})
            cursor = list(to)
            previous_cubic = list(c2)
            previous_quad = None
            continue
        if upper == "Q":
            values = take(4)
            control = point(values[0:2])
            to = point(values[2:4])
            segments.append({"kind": "quadTo", "c": {"x": control[0], "y": control[1]}, "to": {"x": to[0], "y": to[1]}})
            cursor = list(to)
            previous_quad = list(control)
            previous_cubic = None
            continue
        if upper == "T":
            values = take(2)
            mirror = [2 * cursor[0] - previous_quad[0], 2 * cursor[1] - previous_quad[1]] if previous_quad is not None else list(cursor)
            to = point(values)
            segments.append({"kind": "quadTo", "c": {"x": mirror[0], "y": mirror[1]}, "to": {"x": to[0], "y": to[1]}})
            cursor = list(to)
            previous_quad = list(mirror)
            previous_cubic = None
            continue
        if upper == "A":
            values = take(7)
            to = point(values[5:7])
            segments.append({"kind": "arcTo", "rx": values[0], "ry": values[1], "x_rotation": values[2], "large_arc": values[3] != 0, "sweep": values[4] != 0, "to": {"x": to[0], "y": to[1]}})
            cursor = list(to)
            previous_cubic = previous_quad = None
            continue
        raise AssertionError("unsupported path command %r" % command)
    return segments


def transform_of(element: ET.Element) -> dict:
    """📐️ `matrix(a,0,0,d,e,f)`, `scale(s)`/`scale(sx,sy)` and `translate(tx[,ty])` — the three forms
    the two sources use, each of which IS a `Transform` and needs no approximation. Anything that
    skews or rotates is refused rather than flattened; none occurs."""
    declared = element.attrib.get("transform")
    if declared is None:
        return json.loads(json.dumps(IDENTITY))
    match = re.fullmatch(r"(matrix|scale|translate)\(([^)]*)\)", declared.strip())
    if match is None:
        raise AssertionError("unsupported transform %r" % declared)
    form = match.group(1)
    values = [float(piece) for piece in re.split(r"[,\s]+", match.group(2).strip())]
    if form == "matrix":
        if values[1] != 0 or values[2] != 0:
            raise AssertionError("a skewing or rotating matrix has no faithful Transform: %r" % declared)
        scale = (values[0], values[3])
        translation = (values[4], values[5])
    elif form == "scale":
        scale = (values[0], values[1] if len(values) > 1 else values[0])
        translation = (0.0, 0.0)
    else:
        scale = (1.0, 1.0)
        translation = (values[0], values[1] if len(values) > 1 else 0.0)
    return {"translation": {"x": translation[0], "y": translation[1], "z": 0.0}, "rotation": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}, "scale": {"x": scale[0], "y": scale[1], "z": 1.0}}


def node_of(element: ET.Element, fallback: str):
    """🖍️ One SVG element as one `DrawNode`, recursing through groups."""
    tag = element.tag
    if tag == S + "g":
        children = []
        for at, child in enumerate(element):
            inner = node_of(child, "%s-%d" % (fallback, at))
            if inner is not None:
                children.append(inner)
        return {"kind": "group", "transform": transform_of(element), "children": children}
    if tag == S + "path":
        node = {"kind": "path", "segments": path_segments(element.attrib["d"])}
        style = style_for(element, fallback)
        if style is not None:
            node["style"] = style
        return node
    if tag == S + "rect":
        x = float(element.attrib.get("x", 0))
        y = float(element.attrib.get("y", 0))
        width = float(element.attrib["width"])
        height = float(element.attrib["height"])
        node = {
            "kind": "path",
            "segments": [
                {"kind": "moveTo", "to": {"x": x, "y": y}},
                {"kind": "lineTo", "to": {"x": x + width, "y": y}},
                {"kind": "lineTo", "to": {"x": x + width, "y": y + height}},
                {"kind": "lineTo", "to": {"x": x, "y": y + height}},
                {"kind": "close"},
            ],
        }
        style = style_for(element, fallback)
        if style is not None:
            node["style"] = style
        return node
    if tag == S + "image":
        href = element.attrib.get(XLINK + "href", "")
        head, _, payload = href.partition(",")
        if not head.startswith("data:"):
            return None
        mime = head[len("data:") :].split(";")[0]
        return {
            "kind": "image",
            "at": {"x": float(element.attrib.get("x", 0)), "y": float(element.attrib.get("y", 0))},
            "width": float(element.attrib["width"]),
            "height": float(element.attrib["height"]),
            "mime": mime,
            "bytes": list(base64.b64decode(payload)),
        }
    return None


mouse = ET.parse(os.path.join(SVG, "mouse.svg")).getroot()
qr = ET.parse(os.path.join(SVG, "qr-code.svg")).getroot()

mouse_children = [node for node in (node_of(child, "mouse-%d" % at) for at, child in enumerate(mouse) if child.tag != S + "defs") if node is not None]
qr_layers = [child for child in qr if child.tag == S + "g"]
background = next(layer for layer in qr_layers if layer.attrib.get("{http://www.inkscape.org/namespaces/inkscape}label") == "background")
foreground = next(layer for layer in qr_layers if layer.attrib.get("{http://www.inkscape.org/namespaces/inkscape}label") == "foreground")

background_style = style_for(background, "qr-background")
background_node = node_of(background, "qr-background")
if background_style is not None:
    background_node = {"kind": "group", "transform": background_node["transform"], "children": background_node["children"]}

drawing = {
    "schema": "stdio.semio.drawing",
    "canvas": {"width": 1015.0, "height": 1015.0, "background": {"r": 1.0, "g": 1.0, "b": 1.0, "a": 1.0}},
    "styles": styles,
    "layers": [
        {"id": "mouse", "name": "Introduction demonstration mouse", "visible": True, "root": {"kind": "group", "transform": json.loads(json.dumps(IDENTITY)), "children": mouse_children}},
        {"id": "qr-background", "name": "background", "visible": False, "root": background_node},
        {"id": "qr-foreground", "name": "foreground", "visible": True, "root": node_of(foreground, "qr-foreground")},
    ],
}

first_group = drawing["layers"][2]["root"]["children"][0]
nested = first_group["children"][0]

payloads = {
    "create-layer": {"CreateLayer": {"index": 1, "layer": {"id": "annotations", "name": "Anmerkungen", "visible": True, "root": {"kind": "group", "transform": json.loads(json.dumps(IDENTITY)), "children": [{"kind": "text", "value": "Maßstab 1:50 – Übersicht", "at": {"x": 12.0, "y": 24.0}, "style": styles[0]["name"]}]}}}},
    "delete-layer": {"DeleteLayer": {"id": "qr-background"}},
    "create-node": {
        "CreateNode": {
            "parent": {"layer": 0, "path": []},
            "index": 1,
            "node": {
                "kind": "path",
                "segments": [
                    {"kind": "moveTo", "to": {"x": 4.0, "y": 60.0}},
                    {"kind": "arcTo", "rx": 20.0, "ry": 12.0, "x_rotation": 0.0, "large_arc": True, "sweep": False, "to": {"x": 44.0, "y": 60.0}},
                    {"kind": "quadTo", "c": {"x": 24.0, "y": 70.0}, "to": {"x": 4.0, "y": 60.0}},
                    {"kind": "close"},
                ],
                "style": styles[0]["name"],
            },
        }
    },
    "delete-node": {"DeleteNode": {"at": {"layer": 0, "path": [1]}}},
    "move-node": {"MoveNode": {"at": {"layer": 2, "path": [0, 0]}, "new_origin": {"x": 512.5, "y": -128.25}}},
    "drag-nodes": {"DragNodes": {"ats": [{"layer": 2, "path": [0, 0]}, {"layer": 2, "path": [0, 1]}], "offset": {"x": 2.5, "y": -1.25}}},
    "rotate": {"Rotate": {"at": {"layer": 2, "path": [0, 0]}, "new_rotation": {"x": 0.0, "y": 0.0, "z": 1.0, "w": 0.0}}},
    "scale": {"Scale": {"at": {"layer": 2, "path": [0, 0]}, "new_scale": {"x": 2.0, "y": 0.5, "z": 1.0}}},
    "reorder-nodes": {"ReorderNodes": {"parent": {"layer": 2, "path": [0]}, "from": 0, "to": 40}},
    "group": {"Group": {"parent": {"layer": 2, "path": [0]}, "indices": [4, 5, 6], "transform": {"translation": {"x": 1.0, "y": 2.0, "z": 0.0}, "rotation": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}, "scale": {"x": 1.0, "y": 1.0, "z": 1.0}}}},
    "ungroup": {"Ungroup": {"at": {"layer": 2, "path": [0, 3]}}},
    # 🫓 The MOUSE layer root, whose only descendant group is the identity `clipPath` group — the one
    # branch of this real drawing that `flatten` can actually dissolve. Verified against the QR
    # foreground too: every one of its 329 descendant groups carries a `matrix(0.35,…)` transform, so
    # `flatten` there is a refusal, exactly as the production test
    # `flatten_refuses_a_non_identity_descendant_group` states.
    "flatten": {"Flatten": {"at": {"layer": 0, "path": []}}},
    "unflatten": {"Unflatten": {"at": {"layer": 0, "path": [0]}, "original": {"kind": "group", "transform": json.loads(json.dumps(IDENTITY)), "children": [{"kind": "text", "value": "ersetzt", "at": {"x": 1.0, "y": 1.0}, "style": styles[0]["name"]}]}}},
    "replace-path": {
        "ReplacePath": {
            "at": {"layer": 0, "path": [1]},
            "new_segments": [
                {"kind": "moveTo", "to": {"x": 0.0, "y": 0.0}},
                {"kind": "cubicTo", "c1": {"x": 8.0, "y": 0.0}, "c2": {"x": 16.0, "y": 8.0}, "to": {"x": 16.0, "y": 16.0}},
                {"kind": "arcTo", "rx": 4.0, "ry": 4.0, "x_rotation": 90.0, "large_arc": False, "sweep": True, "to": {"x": 24.0, "y": 16.0}},
                {"kind": "quadTo", "c": {"x": 28.0, "y": 20.0}, "to": {"x": 32.0, "y": 24.0}},
                {"kind": "lineTo", "to": {"x": 0.0, "y": 24.0}},
                {"kind": "close"},
            ],
        }
    },
    "replace-fill": {"ReplaceFill": {"style_name": styles[0]["name"], "new_fill": {"r": 0.25, "g": 0.5, "b": 0.75, "a": 0.5}}},
    "change-stroke-color": {"ChangeStrokeColor": {"style_name": styles[0]["name"], "new_color": {"r": 1.0, "g": 0.5, "b": 0.0, "a": 1.0}}},
    "change-stroke-width": {"ChangeStrokeWidth": {"style_name": styles[0]["name"], "new_width": 3.25}},
}

os.makedirs(CASE, exist_ok=True)
dsl = draw.print_dsl(drawing).encode("utf-8")
pack = draw.pack_bytes(drawing)
assert draw.parse_dsl(dsl.decode("utf-8")) == drawing, "the derived DSL does not read back as the drawing it was written from"
assert draw.parse_pack(pack) == drawing, "the derived pack does not read back as the drawing it was written from"
with open(os.path.join(CASE, "🗣️artifact.dsl.semio"), "wb") as handle:
    handle.write(dsl)
with open(os.path.join(CASE, "🎒️artifact.pack.semio"), "wb") as handle:
    handle.write(pack)
for kind, payload in payloads.items():
    with open(os.path.join(CASE, "🦠️%s.json" % kind), "w", encoding="utf-8") as handle:
        json.dump(payload, handle, ensure_ascii=False, separators=(",", ":"))
        handle.write("\n")

for kind, payload in payloads.items():
    applied = draw.apply_mutation(drawing, payload)
    undone = applied
    for step in draw.inverse_mutation(drawing, payload):
        undone = draw.apply_mutation(undone, step)
    assert undone == drawing, "%s: the independent inverse does not restore the derived drawing" % kind
    print("%-22s applied ok, inverse restores" % kind)

print("styles", [style["name"] for style in styles])
print("layers", [(layer["id"], layer["visible"]) for layer in drawing["layers"]])
print("census", json.dumps(draw.shape_report(drawing)))
print("dsl bytes", len(dsl), "pack bytes", len(pack))
