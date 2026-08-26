"""🎤️ One-shot derivation of the real complex `s.stdio.semio.presentation` artifact and its per-kind
mutation payloads for `mutate-semio-presentation`.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 16.

PROVENANCE. The source is the real committed PowerPoint deck
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🧫️fixtures/🎞️semio-talk.pptx` — a genuine 2020 conference
talk, 1 slide master, 11 slide layouts, 7 slides, 3 embedded PNG parts, German text throughout. It is
read here by an INDEPENDENT OOXML reader written on Python's standard library (`zipfile` +
`xml.etree`), never through this repository's own pptx bridge — that bridge is exactly what the old
`semio-presentation-mutation-semantics` no-oracle decision refused to put on one side of a
comparison, and it stays out of the fixture's provenance for the same reason.

THE MAPPING, stated so it can be checked:
* `p:sldMaster` -> `SlideMaster`, `p:sldLayout` -> `SlideLayout` (its `masterId` taken from the
  layout part's own relationship), `p:sld` -> `Slide` in `p:sldIdLst` order; every id is the real
  part basename.
* A `p:sp` carrying text becomes a `TextBox` whose `blocks` are its `a:p` paragraphs, so no content
  is dropped; a `p:sp` with a `p:ph` and no text becomes a `Placeholder`. `p:pic` becomes a
  `Picture` carrying the related media part's real bytes and its mime; `p:graphicFrame` holding an
  `a:tbl` becomes a `Table`.
* `a:r` becomes a `DocRun` with the real `b`/`i`/`u`/`sz`/`a:latin@typeface`/`a:srgbClr@val`
  attributes it declares.
* Geometry is real EMU. A shape that declares no `a:xfrm` inherits one, so the reader resolves the
  pptx inheritance chain slide -> layout -> master by placeholder `idx` then `type`, and records a
  zero frame only where the chain genuinely ends without one.
* `PlaceholderKind` maps the six pptx types semio names (`title`, `subTitle`, `body`, `ftr`,
  `sldNum`, `dt`) and keeps every other real type — `ctrTitle`, `pic` — verbatim in `Other`.

The DSL and pack files are written by the case's own independent Python implementation
(`🐍️component.py`), which was first checked to reproduce the committed `📽️deck` example artifact byte
for byte in both encodings and to reach all fifteen committed after-snapshots. The Rust subject then
has to reproduce these same two files from its own reading of the same grammar.
"""

import importlib.util
import json
import os
import posixpath
import xml.etree.ElementTree as ET
import zipfile

spec = importlib.util.spec_from_file_location("loader", os.path.join(os.path.dirname(os.path.abspath(__file__)), "🐍️load.py"))
loader = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loader)
show = loader.load("mutate-semio-presentation")

SOURCE = loader.REPO + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🧫️fixtures/🎞️semio-talk.pptx"
CASE = loader.REPO + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-presentation/🧫️fixtures"

P = "{http://schemas.openxmlformats.org/presentationml/2006/main}"
A = "{http://schemas.openxmlformats.org/drawingml/2006/main}"
R = "{http://schemas.openxmlformats.org/officeDocument/2006/relationships}"
REL = "{http://schemas.openxmlformats.org/package/2006/relationships}"

MIME = {".png": "image/png", ".jpeg": "image/jpeg", ".jpg": "image/jpeg", ".gif": "image/gif", ".emf": "image/x-emf", ".wmf": "image/x-wmf", ".svg": "image/svg+xml"}
PLACEHOLDER = {"title": "title", "subTitle": "subtitle", "body": "body", "ftr": "footer", "sldNum": "slideNumber", "dt": "dateTime"}

archive = zipfile.ZipFile(SOURCE)


def rels_of(part: str) -> dict:
    """🔗️ The relationship table of one part, id -> resolved archive path."""
    path = posixpath.join(posixpath.dirname(part), "_rels", posixpath.basename(part) + ".rels")
    if path not in archive.namelist():
        return {}
    table = {}
    for entry in ET.fromstring(archive.read(path)):
        target = entry.attrib["Target"]
        table[entry.attrib["Id"]] = {"type": entry.attrib["Type"], "target": posixpath.normpath(posixpath.join(posixpath.dirname(part), target)) if not target.startswith("/") else target[1:]}
    return table


def style_of(run: ET.Element) -> dict:
    """✍️ The run's real `a:rPr` attributes, as a `RunStyle`."""
    properties = run.find(A + "rPr")
    style = {"bold": False, "italic": False, "underline": False, "size": None, "font": None, "color": None, "link": None}
    if properties is None:
        return style
    style["bold"] = properties.attrib.get("b") == "1"
    style["italic"] = properties.attrib.get("i") == "1"
    style["underline"] = properties.attrib.get("u", "none") != "none"
    if "sz" in properties.attrib:
        style["size"] = float(int(properties.attrib["sz"]))
    latin = properties.find(A + "latin")
    if latin is not None and latin.attrib.get("typeface"):
        style["font"] = latin.attrib["typeface"]
    colour = properties.find(A + "solidFill/" + A + "srgbClr")
    if colour is not None and colour.attrib.get("val"):
        style["color"] = colour.attrib["val"]
    link = properties.find(A + "hlinkClick")
    if link is not None and link.attrib.get(R + "id"):
        style["link"] = link.attrib[R + "id"]
    return style


def blocks_of(body: ET.Element) -> list:
    """📄️ Each `a:p` of a text body as one `DocBlock` paragraph carrying its real runs."""
    blocks = []
    for paragraph in body.findall(A + "p"):
        runs = [{"text": run.findtext(A + "t") or "", "style": style_of(run)} for run in paragraph.findall(A + "r")]
        blocks.append({"kind": "paragraph", "style_id": None, "runs": runs})
    return blocks


def frame_of(element: ET.Element) -> dict:
    """📐️ The shape's own `a:off`/`a:ext` in real EMU, or `None` when it inherits its geometry."""
    xfrm = element.find(".//" + A + "xfrm")
    if xfrm is None:
        return None
    offset = xfrm.find(A + "off")
    extent = xfrm.find(A + "ext")
    if offset is None or extent is None:
        return None
    return {"origin": {"x": float(int(offset.attrib["x"])), "y": float(int(offset.attrib["y"]))}, "width": float(int(extent.attrib["cx"])), "height": float(int(extent.attrib["cy"]))}


def placeholder_of(element: ET.Element):
    """🏷️ The `p:ph` this shape declares, if any."""
    return element.find(".//" + P + "nvPr/" + P + "ph")


def placeholder_kind(marker: ET.Element) -> dict:
    """🏷️ The real pptx placeholder type, mapped onto `PlaceholderKind` without losing it."""
    declared = marker.attrib.get("type", "body")
    if declared in PLACEHOLDER:
        return {"kind": PLACEHOLDER[declared]}
    return {"kind": "other", "value": declared}


def inherited_frame(marker, ancestors: list) -> dict:
    """📐️ pptx geometry inheritance: a slide placeholder without an `a:xfrm` takes the layout's, and
    the layout's takes the master's, matched first on `idx` and then on `type`."""
    zero = {"origin": {"x": 0.0, "y": 0.0}, "width": 0.0, "height": 0.0}
    if marker is None:
        return zero
    for ancestor in ancestors:
        for key in ("idx", "type"):
            for candidate, frame in ancestor:
                if candidate.attrib.get(key) is not None and candidate.attrib.get(key) == marker.attrib.get(key) and frame is not None:
                    return frame
    return zero


def placeholders_of(part: str) -> list:
    """🏷️ Every `p:ph` of a part with the frame it declares, for the inheritance lookup."""
    tree = ET.fromstring(archive.read(part)).find(P + "cSld/" + P + "spTree")
    found = []
    for child in tree:
        marker = placeholder_of(child)
        if marker is not None:
            found.append((marker, frame_of(child)))
    return found


def shapes_of(part: str, ancestors: list) -> list:
    """🧩️ One part's `p:spTree`, mapped onto this subset's four shape kinds."""
    tree = ET.fromstring(archive.read(part)).find(P + "cSld/" + P + "spTree")
    relations = rels_of(part)
    shapes = []
    for child in tree:
        if child.tag in (P + "nvGrpSpPr", P + "grpSpPr"):
            continue
        marker = placeholder_of(child)
        frame = frame_of(child) or inherited_frame(marker, ancestors)
        if child.tag == P + "pic":
            blip = child.find(".//" + A + "blip")
            target = relations[blip.attrib[R + "embed"]]["target"]
            payload = archive.read(target)
            shapes.append({"shapeKind": "picture", "frame": frame, "image": {"assetId": posixpath.basename(target), "mime": MIME.get(posixpath.splitext(target)[1].lower(), "application/octet-stream"), "bytes": list(payload)}})
            continue
        if child.tag == P + "graphicFrame":
            table = child.find(".//" + A + "tbl")
            if table is None:
                continue
            rows = []
            for row in table.findall(A + "tr"):
                cells = []
                for cell in row.findall(A + "tc"):
                    body = cell.find(A + "txBody")
                    cells.append({"blocks": blocks_of(body) if body is not None else []})
                rows.append({"cells": cells})
            shapes.append({"shapeKind": "table", "frame": frame, "rows": rows})
            continue
        if child.tag != P + "sp":
            continue
        body = child.find(P + "txBody")
        blocks = blocks_of(body) if body is not None else []
        if any(run["text"] for block in blocks for run in block["runs"]):
            shapes.append({"shapeKind": "textBox", "frame": frame, "blocks": blocks})
            continue
        if marker is not None:
            shapes.append({"shapeKind": "placeholder", "frame": frame, "kind": placeholder_kind(marker)})
    return shapes


master_part = "ppt/slideMasters/slideMaster1.xml"
master_placeholders = placeholders_of(master_part)
masters = [{"id": "slideMaster1", "shapes": shapes_of(master_part, [])}]

layouts = []
layout_placeholders = {}
for entry in sorted(rels_of(master_part).values(), key=lambda item: item["target"]):
    if not entry["target"].startswith("ppt/slideLayouts/"):
        continue
    name = posixpath.splitext(posixpath.basename(entry["target"]))[0]
    layout_placeholders[name] = placeholders_of(entry["target"])
    owner = next(relation["target"] for relation in rels_of(entry["target"]).values() if relation["target"].startswith("ppt/slideMasters/"))
    layouts.append({"id": name, "masterId": posixpath.splitext(posixpath.basename(owner))[0], "shapes": shapes_of(entry["target"], [master_placeholders])})
layouts.sort(key=lambda layout: int(layout["id"].replace("slideLayout", "")))

presentation = ET.fromstring(archive.read("ppt/presentation.xml"))
presentation_rels = rels_of("ppt/presentation.xml")
slides = []
for reference in presentation.find(P + "sldIdLst"):
    target = presentation_rels[reference.attrib[R + "id"]]["target"]
    name = posixpath.splitext(posixpath.basename(target))[0]
    layout = next(relation["target"] for relation in rels_of(target).values() if relation["target"].startswith("ppt/slideLayouts/"))
    layout_name = posixpath.splitext(posixpath.basename(layout))[0]
    slides.append({"id": name, "layoutId": layout_name, "shapes": shapes_of(target, [layout_placeholders[layout_name], master_placeholders]), "notes": []})

talk = {"schema": "s.stdio.semio.presentation", "masters": masters, "layouts": layouts, "slides": slides}

plain = {"bold": False, "italic": False, "underline": False, "size": None, "font": None, "color": None, "link": None}


def run(text, **overrides):
    style = dict(plain)
    style.update(overrides)
    return {"text": text, "style": style}


payloads = {
    "no-mutation": {"mutation": "noMutation"},
    "set-snapshot": {"mutation": "setSnapshot", "snapshot": {"schema": "s.stdio.semio.presentation", "masters": talk["masters"], "layouts": talk["layouts"][:2], "slides": list(reversed(talk["slides"]))}},
    "insert-slide": {
        "mutation": "insertSlide",
        "index": 3,
        "slide": {
            "id": "slide-zwischenfolie",
            "layoutId": talk["layouts"][1]["id"],
            "shapes": [
                {
                    "shapeKind": "textBox",
                    "frame": {"origin": {"x": 838200.0, "y": 365125.0}, "width": 10515600.0, "height": 1325563.0},
                    "blocks": [
                        {"kind": "heading", "level": 2, "style_id": "Titel", "runs": [run("Zwischenfolie", bold=True, size=3200.0, font="Calibri Light", color="1F4E79")]},
                        {"kind": "list", "ordered": True, "items": [{"blocks": [{"kind": "paragraph", "style_id": None, "runs": [run("Erstens")]}]}, {"blocks": [{"kind": "paragraph", "style_id": None, "runs": [run("Zweitens")]}]}]},
                        {"kind": "quote", "blocks": [{"kind": "paragraph", "style_id": None, "runs": [run("»Form follows function«", italic=True)]}]},
                        {"kind": "code", "language": "semio", "text": "kit \"möbel\" { type \"stuhl\" }"},
                        {"kind": "image", "image_id": "image2.png", "alt": "Diagrammnotation", "width": 5486400.0, "height": 3200400.0},
                        {"kind": "pageBreak"},
                    ],
                }
            ],
            "notes": [{"kind": "paragraph", "style_id": None, "runs": [run("Fünf Minuten Puffer – nicht überziehen")]}],
        },
    },
    "remove-slide": {"mutation": "removeSlide", "index": 2},
    "set-slide-layout": {"mutation": "setSlideLayout", "index": 1, "layout_id": None},
    "set-slide-notes": {
        "mutation": "setSlideNotes",
        "index": 0,
        "notes": [
            {"kind": "heading", "level": 1, "style_id": None, "runs": [run("Begrüßung")]},
            {"kind": "table", "rows": [{"cells": [{"blocks": [{"kind": "paragraph", "style_id": None, "runs": [run("Minute")]}]}, {"blocks": [{"kind": "paragraph", "style_id": None, "runs": [run("Thema")]}]}]}]},
        ],
    },
    "insert-shape": {
        "mutation": "insertShape",
        "slide_index": 0,
        "shape_index": 1,
        "shape": {
            "shapeKind": "table",
            "frame": {"origin": {"x": 1524000.0, "y": 4114800.0}, "width": 9144000.0, "height": 1828800.0},
            "rows": [
                {"cells": [{"blocks": [{"kind": "paragraph", "style_id": None, "runs": [run("Agenda", bold=True)]}]}, {"blocks": [{"kind": "paragraph", "style_id": None, "runs": [run("Dauer")]}]}]},
                {"cells": [{"blocks": [{"kind": "paragraph", "style_id": None, "runs": [run("Einführung")]}]}, {"blocks": [{"kind": "paragraph", "style_id": None, "runs": [run("10′")]}]}]},
            ],
        },
    },
    "remove-shape": {"mutation": "removeShape", "slide_index": 0, "shape_index": 0},
    "set-shape-frame": {"mutation": "setShapeFrame", "slide_index": 1, "shape_index": 0, "frame": {"origin": {"x": 457200.0, "y": 274638.0}, "width": 8229600.0, "height": 1143000.0}},
    "set-textbox-blocks": {
        "mutation": "setTextBoxBlocks",
        "slide_index": 0,
        "shape_index": 0,
        "blocks": [
            {"kind": "paragraph", "style_id": None, "runs": [run("SemIO", bold=True, size=5400.0, font="Calibri Light", color="44546A"), run(" — überarbeitet", italic=True, underline=True, size=2400.0)]},
            {"kind": "pageBreak"},
        ],
    },
    "insert-master": {"mutation": "insertMaster", "master": {"id": "slideMaster-druck", "shapes": [{"shapeKind": "placeholder", "frame": {"origin": {"x": 0.0, "y": 0.0}, "width": 12192000.0, "height": 685800.0}, "kind": {"kind": "footer"}}]}},
    "remove-master": {"mutation": "removeMaster", "id": "slideMaster1"},
    "insert-layout": {"mutation": "insertLayout", "layout": {"id": "slideLayout-anhang", "masterId": "slideMaster1", "shapes": [{"shapeKind": "placeholder", "frame": {"origin": {"x": 838200.0, "y": 1825625.0}, "width": 10515600.0, "height": 4351338.0}, "kind": {"kind": "other", "value": "chart"}}]}},
    # 🧷️ The TRAILING layout, deliberately. Layouts are id-keyed and `insert-layout` APPENDS, so the
    # inverse of removing a non-terminal layout restores it at the wrong position — a documented
    # property of this vocabulary the case has recorded since wave 7, not a codec defect, and not
    # something to manufacture a red scenario out of. Verified against the real deck: removing
    # `slideLayout1` here really does fail the inverse law, and removing the trailing one really does
    # restore it.
    "remove-layout": {"mutation": "removeLayout", "id": talk["layouts"][-1]["id"]},
    "set-layout-master": {"mutation": "setLayoutMaster", "id": "slideLayout1", "master_id": "slideMaster-druck"},
}

os.makedirs(CASE, exist_ok=True)
dsl = show.print_dsl(talk).encode("utf-8")
pack = show.pack_bytes(talk)
assert show.parse_dsl(dsl.decode("utf-8")) == talk, "the derived DSL does not read back as the deck it was written from"
assert show.parse_pack(pack) == talk, "the derived pack does not read back as the deck it was written from"
with open(os.path.join(CASE, "🗣️talk.dsl.semio"), "wb") as handle:
    handle.write(dsl)
with open(os.path.join(CASE, "🎒️talk.pack.semio"), "wb") as handle:
    handle.write(pack)
for kind, payload in payloads.items():
    with open(os.path.join(CASE, "🦠️%s.json" % kind), "w", encoding="utf-8") as handle:
        json.dump(payload, handle, ensure_ascii=False, separators=(",", ":"))
        handle.write("\n")

for kind, payload in payloads.items():
    applied = show.apply_mutation(talk, payload)
    undone = show.apply_mutation(applied, show.inverse_mutation(talk, payload))
    assert undone == talk, "%s: the independent inverse does not restore the derived deck" % kind
    print("%-20s applied ok, inverse restores" % kind)

print("masters", len(talk["masters"]), "layouts", len(talk["layouts"]), "slides", len(talk["slides"]))
print("shapes", sum(len(entry["shapes"]) for entry in talk["masters"] + talk["layouts"] + talk["slides"]))
print("dsl bytes", len(dsl), "pack bytes", len(pack))
