"""🧊️ One-off derivation (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 22).

Reads the REAL committed advanced-B-rep file
`♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/📐️hexagonal-cut-concrete-forest-left-bim.stp` — the real
Rhino 8.31 / ST-Developer v19.2 BIM export of the "hexagonal cut concrete forest" structure, the
richest B-rep committed anywhere in this repository (12 solids, 127 faces, 270 B-spline edges) —
with a purpose-written ISO 10303-21 Part 21 reader, and maps its real
geometry/topology entity graph onto the `s.stdio.semio.brep` document model. Nothing is invented:
every vertex point, every B-spline control point/knot, every loop's edge order and orientation
flag, every face's plane and every shell/solid membership comes out of the source file's own
entities, and every semio id carries the source entity number it came from.

Writes `🗣️hexagonal-cut-concrete-forest-left.dsl.semio` and `🎒️hexagonal-cut-concrete-forest-left.pack.semio`
into `mutate-semio-brep/🧫️fixtures/` through the case's own INDEPENDENT Python implementation of the
carrier, so the fixture is a statement of the committed grammar and protocol, not of the Rust codec.
"""

import importlib.util
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
STEP = ROOT / "♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/📐️hexagonal-cut-concrete-forest-left-bim.stp"
CASE = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-brep"


# region 🔖️Part21
RECORD = re.compile(r"#(\d+)\s*=\s*([A-Z_0-9]+)\s*\((.*?)\)\s*;", re.S)


def split_args(text):
    """✂️ Top-level comma split honouring nested parentheses and quoted strings."""
    out, depth, quoted, current = [], 0, False, []
    for ch in text:
        if quoted:
            current.append(ch)
            if ch == "'":
                quoted = False
            continue
        if ch == "'":
            quoted = True
            current.append(ch)
        elif ch == "(":
            depth += 1
            current.append(ch)
        elif ch == ")":
            depth -= 1
            current.append(ch)
        elif ch == "," and depth == 0:
            out.append("".join(current).strip())
            current = []
        else:
            current.append(ch)
    out.append("".join(current).strip())
    return [a for a in out]


def read_step(path):
    text = path.read_text(encoding="utf-8", errors="replace")
    data = text.split("DATA;", 1)[1].split("ENDSEC;", 1)[0]
    data = re.sub(r"/\*.*?\*/", "", data, flags=re.S)
    entities = {}
    for match in RECORD.finditer(data):
        entities[int(match.group(1))] = (match.group(2), split_args(match.group(3)))
    return entities


def ref(token):
    return int(token.lstrip().lstrip("#"))


def numbers(token):
    return [float(x) for x in split_args(token.strip()[1:-1]) if x.strip()]


def refs(token):
    return [ref(x) for x in split_args(token.strip()[1:-1]) if x.strip()]


def flag(token):
    return token.strip() == ".T."


# endregion 🔖️Part21


# region 🔖️Mapping
def point_of(entities, at):
    kind, args = entities[at]
    assert kind == "CARTESIAN_POINT", kind
    x, y, z = numbers(args[1])
    return {"x": x, "y": y, "z": z}


def direction_of(entities, at):
    kind, args = entities[at]
    assert kind == "DIRECTION", kind
    x, y, z = numbers(args[1])
    return {"x": x, "y": y, "z": z}


def placement_of(entities, at):
    kind, args = entities[at]
    assert kind == "AXIS2_PLACEMENT_3D", kind
    location = point_of(entities, ref(args[1]))
    axis = direction_of(entities, ref(args[2])) if args[2].strip() != "$" else {"x": 0.0, "y": 0.0, "z": 1.0}
    return location, axis


def curve_of(entities, at):
    kind, args = entities[at]
    if kind == "B_SPLINE_CURVE_WITH_KNOTS":
        degree = int(float(args[1]))
        control = [point_of(entities, r) for r in refs(args[2])]
        multiplicities = [int(v) for v in numbers(args[6])]
        distinct = numbers(args[7])
        knots = []
        for value, count in zip(distinct, multiplicities):
            knots.extend([value] * count)
        return {"kind": "nurbs", "controlPoints": control, "weights": [1.0] * len(control), "degree": degree, "knots": knots}
    if kind == "LINE":
        origin = point_of(entities, ref(args[1]))
        vector_kind, vector_args = entities[ref(args[2])]
        assert vector_kind == "VECTOR", vector_kind
        direction = direction_of(entities, ref(vector_args[1]))
        magnitude = float(vector_args[2])
        return {"kind": "line", "origin": origin, "direction": {axis: value * magnitude for axis, value in direction.items()}}
    if kind == "CIRCLE":
        centre, axis = placement_of(entities, ref(args[1]))
        return {"kind": "circle", "center": centre, "axis": axis, "radius": float(args[2])}
    raise AssertionError("unmapped curve entity %s" % kind)


def surface_of(entities, at):
    kind, args = entities[at]
    if kind == "PLANE":
        origin, normal = placement_of(entities, ref(args[1]))
        return {"kind": "plane", "origin": origin, "normal": normal}
    if kind == "CYLINDRICAL_SURFACE":
        origin, axis = placement_of(entities, ref(args[1]))
        return {"kind": "cylinder", "origin": origin, "axis": axis, "radius": float(args[2])}
    raise AssertionError("unmapped surface entity %s" % kind)


def build(entities):
    vertices, edges, loops, faces, shells, solids = [], [], [], [], [], []
    vertex_name, edge_name, loop_name, face_name, shell_name = {}, {}, {}, {}, {}
    for at in sorted(entities):
        kind, args = entities[at]
        if kind == "VERTEX_POINT":
            vertex_name[at] = "v%d" % at
            vertices.append({"id": vertex_name[at], "point": point_of(entities, ref(args[1]))})
    for at in sorted(entities):
        kind, args = entities[at]
        if kind == "EDGE_CURVE":
            edge_name[at] = "e%d" % at
            edges.append({"id": edge_name[at], "startVertex": vertex_name[ref(args[1])], "endVertex": vertex_name[ref(args[2])], "curve": curve_of(entities, ref(args[3]))})
    for at in sorted(entities):
        kind, args = entities[at]
        if kind == "EDGE_LOOP":
            loop_name[at] = "l%d" % at
            items = []
            for oriented in refs(args[1]):
                oriented_kind, oriented_args = entities[oriented]
                assert oriented_kind == "ORIENTED_EDGE", oriented_kind
                items.append({"edge": edge_name[ref(oriented_args[3])], "orientation": flag(oriented_args[4])})
            loops.append({"id": loop_name[at], "edges": items})
    for at in sorted(entities):
        kind, args = entities[at]
        if kind == "ADVANCED_FACE":
            face_name[at] = "f%d" % at
            outer, inner = None, []
            for bound in refs(args[1]):
                bound_kind, bound_args = entities[bound]
                target = loop_name[ref(bound_args[1])]
                if bound_kind == "FACE_OUTER_BOUND":
                    outer = target
                else:
                    inner.append(target)
            faces.append({"id": face_name[at], "outerLoop": outer, "innerLoops": inner, "surface": surface_of(entities, ref(args[2])), "orientation": flag(args[3])})
    for at in sorted(entities):
        kind, args = entities[at]
        if kind in ("CLOSED_SHELL", "OPEN_SHELL"):
            shell_name[at] = "s%d" % at
            shells.append({"id": shell_name[at], "faces": [{"face": face_name[f], "orientation": True} for f in refs(args[1])]})
    for at in sorted(entities):
        kind, args = entities[at]
        if kind == "MANIFOLD_SOLID_BREP":
            solids.append({"id": "so%d" % at, "shells": [{"shell": shell_name[ref(args[1])], "isVoid": False}]})
    return {"schema": "stdio.semio.brep", "vertices": vertices, "edges": edges, "loops": loops, "faces": faces, "shells": shells, "solids": solids}


# endregion 🔖️Mapping


def load_oracle():
    """🐍️ Imports the case's own independent implementation, stubbing only the host handles it
    imports for registration — none of which the carrier functions touch."""
    stub = type(sys)("semio_repo_test")
    stub.Adapter = type("Adapter", (), {"__init__": lambda self, name: None, "oracle": lambda self, *a: self})
    stub.Context = object
    stub.Outcome = object
    stub.digest = lambda data: ""
    sys.modules["semio_repo_test"] = stub
    spec = importlib.util.spec_from_file_location("brep_oracle", CASE / "🐍️component.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main():
    entities = read_step(STEP)
    document = build(entities)
    oracle = load_oracle()
    dsl = oracle.print_dsl(document)
    assert oracle.parse_dsl(dsl) == document, "the printed DSL does not re-parse to the same document"
    pack = oracle.pack_bytes(document)
    assert oracle.parse_pack(pack) == document, "the encoded pack does not re-decode to the same document"
    out = CASE / "🧫️fixtures"
    out.mkdir(exist_ok=True)
    (out / "🗣️hexagonal-cut-concrete-forest-left.dsl.semio").write_text(dsl, encoding="utf-8")
    (out / "🎒️hexagonal-cut-concrete-forest-left.pack.semio").write_bytes(pack)
    print("vertices=%d edges=%d loops=%d faces=%d shells=%d solids=%d" % (len(document["vertices"]), len(document["edges"]), len(document["loops"]), len(document["faces"]), len(document["shells"]), len(document["solids"])))
    print("curve kinds:", sorted({e["curve"]["kind"] for e in document["edges"]}))
    print("surface kinds:", sorted({f["surface"]["kind"] for f in document["faces"]}))
    print("dsl bytes=%d pack bytes=%d" % (len(dsl.encode("utf-8")), len(pack)))
    print("last vertex=%s last edge=%s last face=%s last shell=%s last solid=%s" % (document["vertices"][-1]["id"], document["edges"][-1]["id"], document["faces"][-1]["id"], document["shells"][-1]["id"], document["solids"][-1]["id"]))


if __name__ == "__main__":
    main()
