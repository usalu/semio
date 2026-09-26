#!/usr/bin/env python3
"""🧊️ Prepared patch (rule 20, apply after W2's `--packages all`): `🧊️mutate-semio-brep` is red in BOTH roles since the
brep v1 carrier grew (committed grammar `…/📸️snapshot/📝️text/📖️.grammar.semio`, 09-24): every vertex, edge and face
carries its native `tol`, and two lines follow `solids` — `coedges` (p-curve `~curve2`/`-`, parameter range, loop ring)
and `nextLabel`. The Rust codec, the grammar, the JSON schema, the committed specification vectors and the Rust-written
`✉️base` solid all moved; three things did not:

* the independent Python reference (7-line reader, no `tol`, no `coedges`/`nextLabel`, create verbs dropping `tol`):
  3 spec-vector rows + the identity round trip red;
* the feature's `create-vertex`/`create-edge`/`create-face` doc strings (no `tol`, which the Rust payloads require): the
  Rust subject cannot decode them;
* the concrete-forest carrier pair, written by the reference in the OLD 7-line layout: the Rust codec cannot read it.

The reference learns the grammar's new productions (and the pack twin's matching fields, pinned by re-encoding the
Rust-written solid byte for byte); the doc strings state `tol` (1e-7, the tolerance every committed brep document
carries); the forest pair is re-emitted in the new layout with `tol` 1e-7 on every vertex/edge/face, no coedges (the
grammar's own "reconstruct from loops" case) and `nextLabel` 0 (no label history).
Usage: brep-reference-carrier.py --dry-run | --write [--root <dir>]"""
import importlib.util
import json
import re
import sys
import types
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
root = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else REPO
BREP = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep"
REFERENCE = root / BREP / "🧪️tests/🧊️mutate-semio-brep/🐍️.py"
FEATURE = root / BREP / "🧪️tests/🧊️mutate-semio-brep/🥒️.feature"
FOREST_DSL = root / BREP / "🧫️fixtures/🧊️mutate-semio-brep/🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio"
FOREST_PACK = root / BREP / "🧫️fixtures/🧊️mutate-semio-brep/🎒️.pack.semio"
SOLID_DSL = REPO / BREP / "🖼️assets/🧊️solid/🗣️.dsl.semio"
SOLID_PACK = REPO / BREP / "🖼️assets/🧊️solid/🎒️.pack.semio"
VECTORS = REPO / BREP / "🧫️fixtures/🧬️mutations"
TOL = 1e-7

spec = importlib.util.spec_from_file_location("semio_repo_test", REPO / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
host = importlib.util.module_from_spec(spec)
sys.modules["semio_repo_test"] = host
spec.loader.exec_module(host)


def load(source: str, name: str) -> types.ModuleType:
    """🐍️ A reference module built from source text, so the patched reference is exercised before it is written."""
    module = types.ModuleType(name)
    module.__file__ = str(REFERENCE)
    exec(compile(source, str(REFERENCE), "exec"), module.__dict__)
    return module


EDITS = [
    ("  (`document = artifact-mark schema-line vertices-line edges-line loops-line faces-line shells-line\n  solids-line`, the tagged `curve = L|C|E|N` and `surface = P|C|O|S|T|N` value productions with\n  their exact field lists, and `bool = \"0\" | \"1\"`);",
     "  (`document = artifact-mark schema-line vertices-line edges-line loops-line faces-line shells-line\n  solids-line coedges-line next-label-line`, every vertex/edge/face closing on its `tol` number, the\n  tagged `curve = L|C|E|N`, `curve2 = L|C|E|N` (a coedge's `~`-marked p-curve, or `-`) and\n  `surface = P|C|O|S|T|N` value productions with their exact field lists, and `bool = \"0\" | \"1\"`);"),
    ('LETTER_SURFACE = {letter: kind for kind, letter in SURFACE_LETTER.items()}\n',
     'LETTER_SURFACE = {letter: kind for kind, letter in SURFACE_LETTER.items()}\n'
     '#: ➰️ `curve2 = "L" … | "C" … | "E" … | "N" …` — a coedge\'s p-curve, the same tags one dimension down.\n'
     'CURVE2_ORDER = CURVE_ORDER\n'
     'LETTER_CURVE2 = LETTER_CURVE\n'),
    ('    "nurbs": (("controlPoints", "P"), ("weights", "N"), ("degree", "i"), ("knots", "N")),\n}\nSURFACE_FIELDS = {',
     '    "nurbs": (("controlPoints", "P"), ("weights", "N"), ("degree", "i"), ("knots", "N")),\n}\n'
     '#: 🗺️ `curve2`\'s arms, with `q` a `point2` and `Q` a `point2-list`.\n'
     'CURVE2_FIELDS = {\n'
     '    "line": (("origin", "q"), ("direction", "q")),\n'
     '    "circle": (("center", "q"), ("radius", "n")),\n'
     '    "ellipse": (("center", "q"), ("xAxis", "q"), ("radiusMajor", "n"), ("radiusMinor", "n")),\n'
     '    "nurbs": (("controlPoints", "Q"), ("weights", "N"), ("degree", "i"), ("knots", "N")),\n'
     '}\nSURFACE_FIELDS = {'),
    ('def print_point(point: dict) -> str:\n    return "[%s,%s,%s]" % (print_number(point["x"]), print_number(point["y"]), print_number(point["z"]))\n',
     'def print_point(point: dict) -> str:\n    return "[%s,%s,%s]" % (print_number(point["x"]), print_number(point["y"]), print_number(point["z"]))\n\n\n'
     'def read_point2(reader: Reader) -> dict:\n    """📌️ `point2 = "[" number "," number "]"`."""\n    reader.take("[")\n    x = reader.number()\n    reader.take(",")\n    y = reader.number()\n    reader.take("]")\n    return {"x": x, "y": y}\n\n\n'
     'def print_point2(point: dict) -> str:\n    return "[%s,%s]" % (print_number(point["x"]), print_number(point["y"]))\n'),
    ('        if shape == "p":\n            value[name] = read_point(reader)\n        elif shape == "n":',
     '        if shape == "p":\n            value[name] = read_point(reader)\n        elif shape == "q":\n            value[name] = read_point2(reader)\n        elif shape == "Q":\n            value[name] = read_items(reader, read_point2)\n        elif shape == "n":'),
    ('        if shape == "p":\n            parts.append(print_point(member))\n',
     '        if shape == "p":\n            parts.append(print_point(member))\n        elif shape == "q":\n            parts.append(print_point2(member))\n        elif shape == "Q":\n            parts.append("[%s]" % ",".join(print_point2(point) for point in member))\n'),
    ('def read_vertex(reader: Reader) -> dict:\n    """📍️ `vertex = "[" hex "," point3 "]"`."""\n    reader.take("[")\n    vertex_id = reader.hex()\n    reader.take(",")\n    point = read_point(reader)\n    reader.take("]")\n    return {"id": vertex_id, "point": point}\n',
     'def read_vertex(reader: Reader) -> dict:\n    """📍️ `vertex = "[" hex "," point3 "," number "]"` — the number is the vertex\'s own `tol`."""\n    reader.take("[")\n    vertex_id = reader.hex()\n    reader.take(",")\n    point = read_point(reader)\n    reader.take(",")\n    tol = reader.number()\n    reader.take("]")\n    return {"id": vertex_id, "point": point, "tol": tol}\n'),
    ('    """➰ `edge = "[" hex "," hex "," hex "," curve "]"`."""',
     '    """➰ `edge = "[" hex "," hex "," hex "," curve "," number "]"` — the number is the edge\'s own `tol`."""'),
    ('    curve = read_curve(reader)\n    reader.take("]")\n    return {"id": edge_id, "startVertex": start, "endVertex": end, "curve": curve}\n',
     '    curve = read_curve(reader)\n    reader.take(",")\n    tol = reader.number()\n    reader.take("]")\n    return {"id": edge_id, "startVertex": start, "endVertex": end, "curve": curve, "tol": tol}\n'),
    ('    """🔷️ `face = "[" hex "," hex "," hex-list "," surface "," bool "]"`."""',
     '    """🔷️ `face = "[" hex "," hex "," hex-list "," surface "," bool "," number "]"` — the number is the face\'s own `tol`."""'),
    ('    orientation = reader.bit()\n    reader.take("]")\n    return {"id": face_id, "outerLoop": outer, "innerLoops": inner, "surface": surface, "orientation": orientation}\n',
     '    orientation = reader.bit()\n    reader.take(",")\n    tol = reader.number()\n    reader.take("]")\n    return {"id": face_id, "outerLoop": outer, "innerLoops": inner, "surface": surface, "orientation": orientation, "tol": tol}\n'),
    ('COLLECTIONS = (("vertices", read_vertex), ("edges", read_edge), ("loops", read_loop), ("faces", read_face), ("shells", read_shell), ("solids", read_solid))',
     'def read_coedge(reader: Reader) -> dict:\n'
     '    """🧱️ `coedge = "[" hex "," hex "," bool "," opt-curve2 "," prange "," hex "," hex "," hex "]"` with\n'
     '    `opt-curve2 = "-" | "~" curve2` and `prange = "[" number "," number "]"`."""\n'
     '    reader.take("[")\n    coedge_id = reader.hex()\n    reader.take(",")\n    edge = reader.hex()\n    reader.take(",")\n    forward = reader.bit()\n    reader.take(",")\n'
     '    if reader.peek() == "-":\n        reader.take("-")\n        pcurve = None\n    else:\n        reader.take("~")\n        pcurve = read_geometry(reader, LETTER_CURVE2, CURVE2_FIELDS, "curve2")\n'
     '    reader.take(",")\n    reader.take("[")\n    start = reader.number()\n    reader.take(",")\n    end = reader.number()\n    reader.take("]")\n'
     '    reader.take(",")\n    loop_id = reader.hex()\n    reader.take(",")\n    following = reader.hex()\n    reader.take(",")\n    preceding = reader.hex()\n    reader.take("]")\n'
     '    return {"id": coedge_id, "edge": edge, "forward": forward, "pcurve": pcurve, "prange": [start, end], "loopId": loop_id, "next": following, "prev": preceding}\n\n\n'
     'def print_coedge(coedge: dict) -> str:\n'
     '    """✍️ The writing direction of `read_coedge`."""\n'
     '    pcurve = "-" if coedge["pcurve"] is None else "~" + print_geometry(coedge["pcurve"], CURVE_LETTER, CURVE2_FIELDS, "curve2")\n'
     '    return "[%s,%s,%s,%s,[%s,%s],%s,%s,%s]" % (hex_of(coedge["id"]), hex_of(coedge["edge"]), "1" if coedge["forward"] else "0", pcurve, print_number(coedge["prange"][0]), print_number(coedge["prange"][1]), hex_of(coedge["loopId"]), hex_of(coedge["next"]), hex_of(coedge["prev"]))\n\n\n'
     'COLLECTIONS = (("vertices", read_vertex), ("edges", read_edge), ("loops", read_loop), ("faces", read_face), ("shells", read_shell), ("solids", read_solid), ("coedges", read_coedge))'),
    ('    """📖️ The seven body lines of a brep document, under the text envelope."""',
     '    """📖️ The nine body lines of a brep document, under the text envelope: the schema, the seven collections and\n    `nextLabel`, the native label high-water mark."""'),
    ('    keys = ["schema"] + [name for name, _ in COLLECTIONS]\n',
     '    keys = ["schema"] + [name for name, _ in COLLECTIONS] + ["nextLabel"]\n'),
    ('    for (name, reader_of), raw in zip(COLLECTIONS, values[1:]):\n        reader = Reader(raw)\n        document[name] = read_items(reader, reader_of)\n        reader.done()\n    return document\n',
     '    for (name, reader_of), raw in zip(COLLECTIONS, values[1:-1]):\n        reader = Reader(raw)\n        document[name] = read_items(reader, reader_of)\n        reader.done()\n'
     '    if not values[-1].isdigit():\n        raise AssertionError("nextLabel is an unsigned integer, found %r" % values[-1])\n    document["nextLabel"] = int(values[-1])\n    return document\n'),
    ('    vertices = ",".join("[%s,%s]" % (hex_of(vertex["id"]), print_point(vertex["point"])) for vertex in document["vertices"])\n'
     '    edges = ",".join("[%s,%s,%s,%s]" % (hex_of(edge["id"]), hex_of(edge["startVertex"]), hex_of(edge["endVertex"]), print_curve(edge["curve"])) for edge in document["edges"])\n',
     '    vertices = ",".join("[%s,%s,%s]" % (hex_of(vertex["id"]), print_point(vertex["point"]), print_number(vertex["tol"])) for vertex in document["vertices"])\n'
     '    edges = ",".join("[%s,%s,%s,%s,%s]" % (hex_of(edge["id"]), hex_of(edge["startVertex"]), hex_of(edge["endVertex"]), print_curve(edge["curve"]), print_number(edge["tol"])) for edge in document["edges"])\n'),
    ('        "[%s,%s,[%s],%s,%s]" % (hex_of(face["id"]), hex_of(face["outerLoop"]), ",".join(hex_of(name) for name in face["innerLoops"]), print_surface(face["surface"]), "1" if face["orientation"] else "0")\n',
     '        "[%s,%s,[%s],%s,%s,%s]" % (hex_of(face["id"]), hex_of(face["outerLoop"]), ",".join(hex_of(name) for name in face["innerLoops"]), print_surface(face["surface"]), "1" if face["orientation"] else "0", print_number(face["tol"]))\n'),
    ('            "solids=[%s]" % solids,\n        ]\n',
     '            "solids=[%s]" % solids,\n            "coedges=[%s]" % ",".join(print_coedge(coedge) for coedge in document["coedges"]),\n            "nextLabel=%d" % document["nextLabel"],\n        ]\n'),
    ('def write_point_bytes(point: dict) -> bytes:\n    return struct.pack("<3d", point["x"], point["y"], point["z"])\n',
     'def write_point_bytes(point: dict) -> bytes:\n    return struct.pack("<3d", point["x"], point["y"], point["z"])\n\n\n'
     'def read_point2_bytes(data: bytes, at: int) -> tuple:\n    """📌️ Two little-endian f64 — a p-curve\'s parameter-plane point."""\n    x, y = struct.unpack_from("<2d", data, at)\n    return {"x": x, "y": y}, at + 16\n\n\n'
     'def write_point2_bytes(point: dict) -> bytes:\n    return struct.pack("<2d", point["x"], point["y"])\n\n\n'
     'def read_f64(data: bytes, at: int) -> tuple:\n    """🔢️ One little-endian f64 — every `tol` and every prange bound."""\n    return struct.unpack_from("<d", data, at)[0], at + 8\n'),
    ('        if shape == "p":\n            value[name], at = read_point_bytes(data, at)\n',
     '        if shape == "p":\n            value[name], at = read_point_bytes(data, at)\n        elif shape == "q":\n            value[name], at = read_point2_bytes(data, at)\n        elif shape == "Q":\n            count, at = read_varint(data, at)\n            points = []\n            for _ in range(count):\n                point, at = read_point2_bytes(data, at)\n                points.append(point)\n            value[name] = points\n'),
    ('        if shape == "p":\n            out += write_point_bytes(member)\n',
     '        if shape == "p":\n            out += write_point_bytes(member)\n        elif shape == "q":\n            out += write_point2_bytes(member)\n        elif shape == "Q":\n            out += write_varint(len(member))\n            for point in member:\n                out += write_point2_bytes(point)\n'),
    ('    """📦️ Binary envelope, then `format u8`, the schema, and the six collections in grammar order."""',
     '    """📦️ Binary envelope, then `format u8`, the schema, the seven collections in grammar order (each vertex, edge and\n    face closing on its `tol` f64; a coedge\'s p-curve behind a presence byte) and the varint `nextLabel`."""'),
    ('        point, at = read_point_bytes(body, at)\n        vertices.append({"id": vertex_id, "point": point})\n',
     '        point, at = read_point_bytes(body, at)\n        tol, at = read_f64(body, at)\n        vertices.append({"id": vertex_id, "point": point, "tol": tol})\n'),
    ('        curve, at = read_pack_geometry(body, at, CURVE_ORDER, CURVE_FIELDS, "curve")\n        edges.append({"id": edge_id, "startVertex": start, "endVertex": end, "curve": curve})\n',
     '        curve, at = read_pack_geometry(body, at, CURVE_ORDER, CURVE_FIELDS, "curve")\n        tol, at = read_f64(body, at)\n        edges.append({"id": edge_id, "startVertex": start, "endVertex": end, "curve": curve, "tol": tol})\n'),
    ('        faces.append({"id": face_id, "outerLoop": outer, "innerLoops": inner, "surface": surface, "orientation": orientation == 1})\n',
     '        tol, at = read_f64(body, at)\n        faces.append({"id": face_id, "outerLoop": outer, "innerLoops": inner, "surface": surface, "orientation": orientation == 1, "tol": tol})\n'),
    ('        document[key] = records\n    if at != len(body):\n        raise AssertionError("%d trailing byte(s) after the last solid record" % (len(body) - at))\n',
     '        document[key] = records\n    count, at = read_varint(body, at)\n    coedges = []\n    for _ in range(count):\n'
     '        coedge_id, at = read_string(body, at)\n        edge, at = read_string(body, at)\n        forward, at = body[at] == 1, at + 1\n        present, at = body[at] == 1, at + 1\n'
     '        pcurve = None\n        if present:\n            pcurve, at = read_pack_geometry(body, at, CURVE2_ORDER, CURVE2_FIELDS, "curve2")\n'
     '        start, at = read_f64(body, at)\n        end, at = read_f64(body, at)\n        loop_id, at = read_string(body, at)\n        following, at = read_string(body, at)\n        preceding, at = read_string(body, at)\n'
     '        coedges.append({"id": coedge_id, "edge": edge, "forward": forward, "pcurve": pcurve, "prange": [start, end], "loopId": loop_id, "next": following, "prev": preceding})\n'
     '    document["coedges"] = coedges\n    document["nextLabel"], at = read_varint(body, at)\n'
     '    if at != len(body):\n        raise AssertionError("%d trailing byte(s) after nextLabel" % (len(body) - at))\n'),
    ('        body += write_string(vertex["id"]) + write_point_bytes(vertex["point"])\n',
     '        body += write_string(vertex["id"]) + write_point_bytes(vertex["point"]) + struct.pack("<d", vertex["tol"])\n'),
    ('        body += write_pack_geometry(edge["curve"], CURVE_ORDER, CURVE_FIELDS)\n',
     '        body += write_pack_geometry(edge["curve"], CURVE_ORDER, CURVE_FIELDS) + struct.pack("<d", edge["tol"])\n'),
    ('        body.append(1 if face["orientation"] else 0)\n',
     '        body.append(1 if face["orientation"] else 0)\n        body += struct.pack("<d", face["tol"])\n'),
    ('            for item in record[member]:\n                body += write_flagged(item, field)\n    token = PACK_TOKEN.encode("utf-8")\n',
     '            for item in record[member]:\n                body += write_flagged(item, field)\n    body += write_varint(len(document["coedges"]))\n    for coedge in document["coedges"]:\n'
     '        body += write_string(coedge["id"]) + write_string(coedge["edge"]) + bytes([1 if coedge["forward"] else 0, 0 if coedge["pcurve"] is None else 1])\n'
     '        if coedge["pcurve"] is not None:\n            body += write_pack_geometry(coedge["pcurve"], CURVE2_ORDER, CURVE2_FIELDS)\n'
     '        body += struct.pack("<2d", coedge["prange"][0], coedge["prange"][1]) + write_string(coedge["loopId"]) + write_string(coedge["next"]) + write_string(coedge["prev"])\n'
     '    body += write_varint(document["nextLabel"])\n    token = PACK_TOKEN.encode("utf-8")\n'),
    ('        result["vertices"].append({"id": args["id"], "point": clone(args["point"])})\n',
     '        result["vertices"].append({"id": args["id"], "point": clone(args["point"]), "tol": args["tol"]})\n'),
    ('        result["edges"].append({"id": args["id"], "startVertex": args["start_vertex"], "endVertex": args["end_vertex"], "curve": clone(args["curve"])})\n',
     '        result["edges"].append({"id": args["id"], "startVertex": args["start_vertex"], "endVertex": args["end_vertex"], "curve": clone(args["curve"]), "tol": args["tol"]})\n'),
    ('"surface": clone(args["surface"]), "orientation": args["orientation"]})\n',
     '"surface": clone(args["surface"]), "orientation": args["orientation"], "tol": args["tol"]})\n'),
    ('    return {"CreateEdge": {"id": edge["id"], "start_vertex": edge["startVertex"], "end_vertex": edge["endVertex"], "curve": clone(edge["curve"])}}\n',
     '    return {"CreateEdge": {"id": edge["id"], "start_vertex": edge["startVertex"], "end_vertex": edge["endVertex"], "curve": clone(edge["curve"]), "tol": edge["tol"]}}\n'),
    ('        steps = [{"CreateVertex": {"id": vertex["id"], "point": clone(vertex["point"])}}]\n',
     '        steps = [{"CreateVertex": {"id": vertex["id"], "point": clone(vertex["point"]), "tol": vertex["tol"]}}]\n'),
    ('"surface": clone(face["surface"]), "orientation": face["orientation"]}}]\n',
     '"surface": clone(face["surface"]), "orientation": face["orientation"], "tol": face["tol"]}}]\n'),
]
CREATES = re.compile(r'(\{"Create(?:Vertex|Edge|Face)":\{"id":"[^"]*")')


def realign(lines: list) -> list:
    """📏️ Re-pads every Examples table so each column is as wide as its widest cell."""
    at = 0
    while at < len(lines):
        if not lines[at].strip().startswith("|"):
            at += 1
            continue
        end = at
        while end < len(lines) and lines[end].strip().startswith("|"):
            end += 1
        indent = lines[at][: len(lines[at]) - len(lines[at].lstrip())]
        rows = [[cell.strip() for cell in line.strip()[1:-1].split(" | ")] for line in lines[at:end]]
        widths = [max(len(row[column]) for row in rows) for column in range(len(rows[0]))]
        lines[at:end] = [indent + "| " + " | ".join(cell.ljust(width) for cell, width in zip(row, widths)) + " |" for row in rows]
        at = end
    return lines


write = "--write" in sys.argv
problems = []
original = REFERENCE.read_text(encoding="utf-8")
patched = original
if "def read_coedge(" in original:
    problems.append("already applied")
for old, new in EDITS:
    if patched.count(old) != 1:
        problems.append(f"reference: {patched.count(old)} × {old[:70]!r}")
        continue
    patched = patched.replace(old, new)
feature = FEATURE.read_text(encoding="utf-8")
creates = len(CREATES.findall(feature))
if creates != 9 or '"tol"' in feature:
    problems.append("feature: %d create payloads, tol already present %s" % (creates, '"tol"' in feature))
feature = "\n".join(realign(CREATES.sub(r'\1,"tol":1e-7', feature).split("\n")))
forest_dsl = forest_pack = None
if not problems:
    before = load(original, "brep_before")
    after = load(patched, "brep_after")
    old_forest = before.parse_dsl(FOREST_DSL.read_text(encoding="utf-8"))
    if before.parse_pack(FOREST_PACK.read_bytes()) != old_forest:
        problems.append("the committed forest pair disagrees with itself before the patch")
    forest = {key: value for key, value in old_forest.items()}
    for key in ("vertices", "edges", "faces"):
        forest[key] = [dict(item, tol=TOL) for item in old_forest[key]]
    forest["coedges"] = []
    forest["nextLabel"] = 0
    forest_dsl = after.print_dsl(forest)
    forest_pack = after.pack_bytes(forest)
    if after.parse_dsl(forest_dsl) != forest or after.parse_pack(forest_pack) != forest:
        problems.append("the re-emitted forest pair does not decode back to the forest")
    solid_text = SOLID_DSL.read_text(encoding="utf-8")
    solid = after.parse_dsl(solid_text)
    if after.print_dsl(solid) != solid_text or after.pack_bytes(solid) != SOLID_PACK.read_bytes() or after.parse_pack(SOLID_PACK.read_bytes()) != solid:
        problems.append("the patched reference does not reproduce the Rust-written solid pair byte for byte")
    agreed = refused = 0
    for vector in sorted(VECTORS.glob("*/*")):
        read = lambda part: json.loads((vector / part / "🔣️.json").read_text(encoding="utf-8"))
        status = read("🎯️outcome")["status"]
        try:
            produced = after.apply_mutation(read("📸️snapshot/⬅️before"), read("🦠️mutation"))
        except AssertionError:
            produced = None
        expected = read("📸️snapshot/➡️after") if status == "applied" else read("📸️snapshot/⬅️before")
        if (status == "rejected" and produced is None) or produced == expected:
            agreed += 1
        else:
            refused += 1
            problems.append(f"vector {vector.parent.name}/{vector.name} ({status}) disagrees")
    inverse_from = feature.index("@id-inverse")
    rows = [(match.start() > inverse_from, json.loads(match.group(1))) for match in re.finditer(r"\| (\{\"prepare\".*?\}) +\|", feature)]
    for inverted, row in rows:
        try:
            document = after.apply_all(forest, row["prepare"])
            mutated = after.apply_mutation(document, row["mutation"])
            if inverted and after.apply_all(mutated, after.inverse_mutation(document, row["mutation"])) != document:
                problems.append(f"feature row {list(row['mutation'])[0]}: the inverse does not restore the forest")
            after.parse_dsl(after.print_dsl(mutated))
        except AssertionError as error:
            problems.append(f"feature row {list(row['mutation'])[0]}: {error}")
    print(f"vectors {agreed}/{agreed + refused} agree; feature rows {len(rows)} apply + invert on the re-emitted forest")
print(f"files=4 problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    REFERENCE.write_text(patched, encoding="utf-8")
    FEATURE.write_text(feature, encoding="utf-8")
    FOREST_DSL.write_text(forest_dsl, encoding="utf-8")
    FOREST_PACK.write_bytes(forest_pack)
