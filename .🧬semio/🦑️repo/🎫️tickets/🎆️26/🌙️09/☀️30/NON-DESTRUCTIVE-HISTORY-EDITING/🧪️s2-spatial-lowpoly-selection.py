#!/usr/bin/env python3
"""🧪️ S2-SPATIAL: authors lowpoly's three relative selection leaves — `move-selection`, `rotate-selection`,
`scale-selection` — with an INDEPENDENT half-edge mesh implementation (numpy float32 storage, the system libm for
cos/sin/hypot, written from the leaves' schemas and the motion law stated in their doc comments — no Rust read): the
payload schemas with full `x-semio-ui`, the leaf descriptors, every committed quintet, each scenario's Rust test, the
mounts, and the registration with every surface that enumerates the vocabulary (aggregate enum and KINDS, the
JSON/TS/GraphQL/proto/grammar/protocol twins, the `💠️mutate-lowpoly-1` harness and the oracle catalog). Case
directories follow the repo path budget (one emoji plus a short slug). Idempotent.
"""
import copy
import ctypes
import ctypes.util
import hashlib
import json
import math
import os
from collections import OrderedDict

import numpy as np

ARTIFACT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly"
ANY = ARTIFACT + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUTATIONS = ANY + "/🧬️schema/🧬️mutations"
FIXTURES = ANY + "/🧫️fixtures/🧬️mutations"
HARNESS = ANY + "/🧪️tests/💠️mutate-lowpoly-1"
SCRIPT = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s2-spatial-lowpoly-selection.py"
ID_BASE = "https://json.schemas.assets.semio-tech.com/s/lowpoly/lowpoly/mutation"

LIBM = ctypes.CDLL(ctypes.util.find_library("m"))
for _name in ("cos", "sin"):
    getattr(LIBM, _name).restype = ctypes.c_double
    getattr(LIBM, _name).argtypes = [ctypes.c_double]
LIBM.hypot.restype = ctypes.c_double
LIBM.hypot.argtypes = [ctypes.c_double, ctypes.c_double]


# region 🔢️Numbers
def f32(value):
    """🔢️ The float32 nearest `value`, held as a Python float."""
    return float(np.float32(value))


def number(value):
    """✍️ One float32 as the platform's JSON writer prints it: the shortest digits that round-trip the float32, fixed
    notation for a decimal exponent within -5..=15 (a whole number keeps `.0`), exponential otherwise."""
    v = np.float32(value)
    if v == 0:
        return "-0.0" if np.signbit(v) else "0.0"
    mantissa, exponent = np.format_float_scientific(np.abs(v), unique=True, trim="-").split("e")
    exponent = int(exponent)
    digits = mantissa.replace(".", "")
    sign = "-" if v < 0 else ""
    if -5 <= exponent <= 15:
        if exponent >= len(digits) - 1:
            return sign + digits + "0" * (exponent - (len(digits) - 1)) + ".0"
        if exponent >= 0:
            return sign + digits[:exponent + 1] + "." + digits[exponent + 1:]
        return sign + "0." + "0" * (-exponent - 1) + digits
    return sign + digits[0] + ("." + digits[1:] if len(digits) > 1 else "") + "e" + ("+" if exponent >= 0 else "") + str(exponent)


def compact(value):
    """✍️ The platform's compact JSON for a mesh value tree whose floats are float32s, in declaration order."""
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        return str(value)
    if isinstance(value, float):
        return number(value)
    if isinstance(value, str):
        return json.dumps(value, ensure_ascii=False)
    if isinstance(value, list):
        return "[" + ",".join(compact(item) for item in value) + "]"
    return "{" + ",".join(json.dumps(key, ensure_ascii=False) + ":" + compact(item) for key, item in value.items()) + "}"
# endregion 🔢️Numbers


# region 🕸️Mesh
def quad_mesh():
    """🕸️ One flat quad in the y = 0 plane: four corners (±1, 0, ±1), four boundary half-edges around one flat face."""
    corners = [(-1.0, 0.0, -1.0), (1.0, 0.0, -1.0), (1.0, 0.0, 1.0), (-1.0, 0.0, 1.0)]
    uvs = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
    mesh = OrderedDict([
        ("vertices", [OrderedDict([("position", [f32(c) for c in corner]), ("normal", None), ("halfedge", index)]) for index, corner in enumerate(corners)]),
        ("halfedges", [OrderedDict([("vertex", index), ("twin", None), ("next", (index + 1) % 4), ("face", 0), ("uv", [f32(u) for u in uv])]) for index, uv in enumerate(uvs)]),
        ("faces", [OrderedDict([("halfedge", 0), ("smooth", False), ("flipped", False)])]),
        ("uv_seams", []),
    ])
    recompute_normals(mesh)
    return mesh


def face_vertices(mesh, face):
    """🔁️ The vertex ids of one face, walking its half-edge loop."""
    out, start = [], mesh["faces"][face]["halfedge"]
    edge = start
    while True:
        out.append(mesh["halfedges"][edge]["vertex"])
        edge = mesh["halfedges"][edge]["next"]
        if edge == start:
            return out


def normalize(vector):
    """📐️ A float32 vector divided by its float64 length, back to float32; the zero vector stays zero."""
    length = LIBM.hypot(LIBM.hypot(float(vector[0]), float(vector[1])), float(vector[2]))
    return [0.0, 0.0, 0.0] if length == 0.0 else [f32(float(value) / length) for value in vector]


def newell(points):
    """📐️ Newell's face normal in float64 relative to the first point, normalised and stored as float32."""
    ox, oy, oz = (float(value) for value in points[0])
    nx = ny = nz = 0.0
    for index, a in enumerate(points):
        b = points[(index + 1) % len(points)]
        ax, ay, az = float(a[0]) - ox, float(a[1]) - oy, float(a[2]) - oz
        bx, by, bz = float(b[0]) - ox, float(b[1]) - oy, float(b[2]) - oz
        nx += (ay - by) * (az + bz)
        ny += (az - bz) * (ax + bx)
        nz += (ax - bx) * (ay + by)
    length = LIBM.hypot(LIBM.hypot(nx, ny), nz)
    return [0.0, 0.0, 0.0] if length == 0.0 else [f32(nx / length), f32(ny / length), f32(nz / length)]


def recompute_normals(mesh):
    """📐️ Every vertex takes the normalised Newell normal of the first face that visits it (all faces here are flat)."""
    flat = [None] * len(mesh["vertices"])
    for face in range(len(mesh["faces"])):
        ids = face_vertices(mesh, face)
        normal = normalize(newell([mesh["vertices"][vertex]["position"] for vertex in ids]))
        for vertex in ids:
            if flat[vertex] is None:
                flat[vertex] = normal
    for vertex, normal in zip(mesh["vertices"], flat):
        vertex["normal"] = normal


def moved(mesh, vertex_ids, motion):
    """🧲️ The mesh with every named vertex carried by the motion — float64 arithmetic on the float32 positions, stored
    back as float32 — and the normals recomputed. `vertex_ids` are sorted and unique."""
    out = copy.deepcopy(mesh)
    kind = motion[0]
    if kind == "offset":
        delta = [float(value) for value in motion[1]]
        transform = lambda point: [point[axis] + delta[axis] for axis in range(3)]
    elif kind == "turn":
        pivot = [float(value) for value in motion[1]]
        axis = [float(value) for value in motion[2]]
        length = LIBM.hypot(LIBM.hypot(axis[0], axis[1]), axis[2])
        x, y, z = (value / length for value in axis)
        c, s = LIBM.cos(float(motion[3])), LIBM.sin(float(motion[3]))
        t = 1.0 - c

        def transform(point):
            px, py, pz = (point[index] - pivot[index] for index in range(3))
            rx = (t * x * x + c) * px + (t * x * y - s * z) * py + (t * x * z + s * y) * pz
            ry = (t * x * y + s * z) * px + (t * y * y + c) * py + (t * y * z - s * x) * pz
            rz = (t * x * z - s * y) * px + (t * y * z + s * x) * py + (t * z * z + c) * pz
            return [pivot[0] + rx, pivot[1] + ry, pivot[2] + rz]
    else:
        pivot = [float(value) for value in motion[1]]
        factor = [float(value) for value in motion[2]]
        transform = lambda point: [pivot[axis] + (point[axis] - pivot[axis]) * factor[axis] for axis in range(3)]
    for vertex in vertex_ids:
        out["vertices"][vertex]["position"] = [f32(value) for value in transform([float(value) for value in out["vertices"][vertex]["position"]])]
    recompute_normals(out)
    return out


def handle(object_id, content):
    """🆔️ The content-addressed mesh child handle: `mesh-` + the first 16 lowercase hex digits of SHA-256 over the
    content, aimed at the object's own `<id>-mesh` child in the `s.stdio.semio` v1 mesh subset."""
    return OrderedDict([("childId", "mesh-" + hashlib.sha256(content.encode("utf-8")).hexdigest()[:16]), ("target", OrderedDict([("artifactId", object_id + "-mesh"), ("dialect", OrderedDict([("artifactKind", "s.stdio.semio"), ("standard", "v1"), ("subset", "mesh")]))]))])
# endregion 🕸️Mesh


# region 🧫️Scenarios
def document():
    """🧫️ A two-object document: `obj-plane` carries the flat quad (persisted content and the handle it hashes to) and a
    two-pixel opaque-white base paint layer; `obj-fin` carries no mesh and no paint stack."""
    content = compact(quad_mesh())
    transform = OrderedDict([("position", [0.0, 0.0, 0.0]), ("rotation", [0.0, 0.0, 0.0]), ("scale", [1.0, 1.0, 1.0])])
    plane = OrderedDict([("id", "obj-plane"), ("mesh", handle("obj-plane", content)), ("meshContent", content), ("name", "Plane"), ("paintLayers", [OrderedDict([("blendMode", "normal"), ("name", "Base"), ("opacity", 1.0), ("pixels", "//////////8="), ("visible", True)])]), ("smoothShading", False), ("transform", copy.deepcopy(transform))])
    fin = OrderedDict([("id", "obj-fin"), ("mesh", None), ("meshContent", ""), ("name", "Fin"), ("paintLayers", []), ("smoothShading", True), ("transform", copy.deepcopy(transform))])
    return OrderedDict([("objects", [plane, fin]), ("schema", "lowpoly.document")])


QUARTER = f32(math.pi / 2.0)

LEAVES = {
    "move-selection": dict(emoji="🚚️", variant="MoveSelection", module="move_selection", tag=19),
    "rotate-selection": dict(emoji="🌀️", variant="RotateSelection", module="rotate_selection", tag=20),
    "scale-selection": dict(emoji="🔍️", variant="ScaleSelection", module="scale_selection", tag=21),
}


def move(vertex_ids, offset, target="obj-plane"):
    return {"MoveSelection": OrderedDict([("objectId", target), ("offset", [f32(v) for v in offset]), ("vertexIds", list(vertex_ids))])}


def rotate(vertex_ids, pivot, axis, angle, target="obj-plane"):
    return {"RotateSelection": OrderedDict([("angle", f32(angle)), ("axis", [f32(v) for v in axis]), ("objectId", target), ("pivot", [f32(v) for v in pivot]), ("vertexIds", list(vertex_ids))])}


def scale(vertex_ids, pivot, factor, target="obj-plane"):
    return {"ScaleSelection": OrderedDict([("factor", [f32(v) for v in factor]), ("objectId", target), ("pivot", [f32(v) for v in pivot]), ("vertexIds", list(vertex_ids))])}


SCENARIOS = [
    ("move-selection", "🚚️moves", "The whole plane slides by (0.5, 0, -0.25): every corner moves, the normals stay up.", move([], (0.5, 0.0, -0.25)), None),
    ("move-selection", "📌️pins", "Two corners of the plane slide one unit along x; the other two stay pinned.", move([1, 2], (1.0, 0.0, 0.0)), None),
    ("move-selection", "🧩️part", "Vertex 0 and the absent vertex 9 are named: vertex 0 lifts, vertex 9 is skipped as a partial application.", move([0, 9], (0.0, 0.5, 0.0)), None),
    ("move-selection", "⏸️still", "A zero offset moves no vertex: the drag is a declared no-op.", move([], (0.0, 0.0, 0.0)), None),
    ("move-selection", "⛔️ghost", "A drag of object obj-ghost, which the document does not hold, is refused.", move([], (1.0, 0.0, 0.0), target="obj-ghost"), None),
    ("move-selection", "🕳️bare", "A drag of obj-fin, which carries no mesh, is refused.", move([], (1.0, 0.0, 0.0), target="obj-fin"), None),
    ("move-selection", "🔁️twice", "Vertex 1 named twice is an invariant violation the schema already refuses.", move([1, 1], (1.0, 0.0, 0.0)), "schema"),
    ("rotate-selection", "🌀️turns", "The whole plane turns a quarter about +y through the origin.", rotate([], (0.0, 0.0, 0.0), (0.0, 1.0, 0.0), QUARTER), None),
    ("rotate-selection", "📐️lifts", "Corner 2 alone turns half a radian about +z through the origin, so the quad bends and its normals tilt.", rotate([2], (0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 0.5), None),
    ("rotate-selection", "⏸️still", "A zero angle turns no vertex: the turn is a declared no-op.", rotate([], (0.0, 0.0, 0.0), (0.0, 1.0, 0.0), 0.0), None),
    ("rotate-selection", "🫥️void", "A turn about the zero axis is the declared invariant axis-nonzero.", rotate([], (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), 0.5), "axis-nonzero"),
    ("rotate-selection", "⛔️ghost", "A turn of object obj-ghost, which the document does not hold, is refused.", rotate([], (0.0, 0.0, 0.0), (0.0, 1.0, 0.0), 0.5, target="obj-ghost"), None),
    ("scale-selection", "🔍️wides", "The whole plane doubles along x about the origin.", scale([], (0.0, 0.0, 0.0), (2.0, 1.0, 1.0)), None),
    ("scale-selection", "🧩️part", "Vertices 3 and the absent 7 halve along z: vertex 3 moves, vertex 7 is skipped.", scale([3, 7], (0.0, 0.0, 0.0), (1.0, 1.0, 0.5)), None),
    ("scale-selection", "⏸️still", "Unit factors scale nothing: a declared no-op.", scale([], (0.0, 0.0, 0.0), (1.0, 1.0, 1.0)), None),
    ("scale-selection", "🫓️flat", "A zero factor would flatten the plane and is an invariant violation the schema already refuses.", scale([], (0.0, 0.0, 0.0), (0.0, 1.0, 1.0)), "schema"),
]


SCENARIO_IDS = {
    ("move-selection", "🚚️moves"): "slides-the-whole-plane",
    ("move-selection", "📌️pins"): "slides-two-corners-along-x",
    ("move-selection", "🧩️part"): "lifts-one-vertex-and-skips-an-absent-one",
    ("move-selection", "⏸️still"): "a-zero-offset-moves-nothing",
    ("move-selection", "⛔️ghost"): "refuses-a-missing-object",
    ("move-selection", "🕳️bare"): "refuses-an-object-without-mesh",
    ("move-selection", "🔁️twice"): "refuses-a-vertex-named-twice",
    ("rotate-selection", "🌀️turns"): "quarter-turns-the-plane-about-y",
    ("rotate-selection", "📐️lifts"): "bends-one-corner-about-z",
    ("rotate-selection", "⏸️still"): "a-zero-angle-turns-nothing",
    ("rotate-selection", "🫥️void"): "refuses-the-zero-axis",
    ("rotate-selection", "⛔️ghost"): "refuses-a-missing-object",
    ("scale-selection", "🔍️wides"): "doubles-the-plane-along-x",
    ("scale-selection", "🧩️part"): "halves-one-vertex-and-skips-an-absent-one",
    ("scale-selection", "⏸️still"): "unit-factors-scale-nothing",
    ("scale-selection", "🫓️flat"): "refuses-a-zero-factor",
}
"""🏷️ Every scenario's descriptive id in the oracle catalog, beside its budget-short directory name."""

def motion_of(kind, payload):
    if kind == "move-selection":
        return ("offset", payload["offset"])
    if kind == "rotate-selection":
        return ("turn", payload["pivot"], payload["axis"], payload["angle"])
    return ("stretch", payload["pivot"], payload["factor"])


def violation(kind, payload):
    ids = payload["vertexIds"]
    if len(set(ids)) != len(ids):
        return True
    if kind == "rotate-selection" and all(value == 0.0 for value in payload["axis"]):
        return True
    if kind == "scale-selection" and not all(value > 0.0 for value in payload["factor"]):
        return True
    return False


def identity(kind, payload):
    if kind == "move-selection":
        return all(value == 0.0 for value in payload["offset"])
    if kind == "rotate-selection":
        return payload["angle"] == 0.0
    return all(value == 1.0 for value in payload["factor"])


def empty_diff():
    return OrderedDict([("artifact", None), ("objects", None), ("schema", None)])


def mesh_diff(object_id, content):
    patch = OrderedDict([("mesh", handle(object_id, content)), ("meshContent", content), ("name", None), ("smoothShading", None), ("transform", None)])
    return OrderedDict([("artifact", None), ("objects", OrderedDict([("added", []), ("patched", [OrderedDict([("id", object_id), ("paintLayers", None), ("patch", patch)])]), ("removed", []), ("reordered", None)])), ("schema", None)])


def rejected(code, object_id, invariant=None):
    outcome = OrderedDict([("status", "rejected"), ("code", code)])
    if invariant:
        outcome["invariant"] = invariant
    outcome["path"] = [object_id]
    return outcome


def applied(*codes):
    return OrderedDict([("status", "applied")] + ([("messages", [OrderedDict([("level", "warning"), ("code", code)]) for code in codes])] if codes else []))


def apply_selection(before, kind, mutation):
    """🧮️ `(after, diff, outcome)` of one selection leaf on `before`."""
    payload = mutation[LEAVES[kind]["variant"]]
    object_id = payload["objectId"]
    if violation(kind, payload):
        return before, None, rejected("mutation.invariant", object_id, invariant="axis-nonzero" if kind == "rotate-selection" and all(v == 0.0 for v in payload["axis"]) else None)
    record = next((entry for entry in before["objects"] if entry["id"] == object_id), None)
    if record is None or record["mesh"] is None or not record["meshContent"]:
        return before, None, rejected("mutation.target-missing", object_id)
    mesh = json.loads(record["meshContent"], object_pairs_hook=OrderedDict)
    count = len(mesh["vertices"])
    ids = list(range(count)) if not payload["vertexIds"] else payload["vertexIds"]
    present = sorted({vertex for vertex in ids if vertex < count})
    skipped = [vertex for vertex in ids if vertex >= count]
    if not present:
        return before, None, rejected("mutation.target-missing", object_id)
    partial = ["mutation.partial"] if skipped else []
    if identity(kind, payload):
        return before, empty_diff(), applied(*partial, "mutation.no-op")
    content = compact(moved(mesh, present, motion_of(kind, payload)))
    if content == record["meshContent"]:
        return before, empty_diff(), applied(*partial, "mutation.no-op")
    after = copy.deepcopy(before)
    target = next(entry for entry in after["objects"] if entry["id"] == object_id)
    target["mesh"], target["meshContent"] = handle(object_id, content), content
    return after, mesh_diff(object_id, content), applied(*partial)
# endregion 🧫️Scenarios


# region 🧬️Schemas
def label(en, de):
    return OrderedDict([("en", en), ("de", de)])


def object_input(order):
    return OrderedDict([("type", "string"), ("x-semio-ui", OrderedDict([("widget", "reference"), ("role", "target"), ("label", label("Object", "Objekt")), ("description", label("The object whose mesh the motion moves.", "Das Objekt, dessen Netz die Bewegung bewegt.")), ("ref", OrderedDict([("kind", "object"), ("domain", "mesh"), ("granularity", "object")])), ("group", "target"), ("order", order)]))])


def vertices_input(order):
    return OrderedDict([("type", "array"), ("items", OrderedDict([("type", "integer"), ("minimum", 0)])), ("uniqueItems", True), ("x-semio-ui", OrderedDict([("widget", "reference"), ("role", "target"), ("label", label("Vertices", "Eckpunkte")), ("description", label("The vertices the motion carries; empty carries every vertex of the object, absent ones are skipped.", "Die Eckpunkte, die die Bewegung mitnimmt; leer nimmt jeden Eckpunkt des Objekts mit, fehlende werden übersprungen.")), ("ref", OrderedDict([("kind", "vertex"), ("domain", "mesh"), ("granularity", "vertex")])), ("group", "target"), ("order", order)]))])


def vector_input(en, de, description_en, description_de, group, order, step=0.1, positive=False):
    items = OrderedDict([("type", "number")] + ([("exclusiveMinimum", 0)] if positive else []))
    return OrderedDict([("type", "array"), ("items", items), ("minItems", 3), ("maxItems", 3), ("x-semio-ui", OrderedDict([("widget", "vector"), ("role", "value"), ("label", label(en, de)), ("description", label(description_en, description_de)), ("step", step), ("precision", 3), ("group", group), ("order", order)]))])


def leaf_schema(kind):
    variant = LEAVES[kind]["variant"]
    properties = OrderedDict([("objectId", object_input(10)), ("vertexIds", vertices_input(20))])
    invariants = []
    if kind == "move-selection":
        description = "The gumball's drag of a selection stated as its intent: the object, the vertices it names (every vertex when empty) and one offset. The geometry is derived from the object's persisted mesh on every application. Its hard bounds (vertices named once) state the payload-intrinsic breach the diff refuses as `mutation.invariant`; refusals that depend on the base (an absent object, mesh or vertex) belong to the diff alone."
        properties["offset"] = vector_input("Offset", "Versatz", "How far every named vertex moves along x, y and z.", "Wie weit sich jeder genannte Eckpunkt entlang x, y und z bewegt.", "motion", 30)
        required = ["objectId", "offset", "vertexIds"]
    elif kind == "rotate-selection":
        description = "The gumball's turn of a selection stated as its intent: the object, the vertices it names (every vertex when empty), the pivot, the axis and the angle in radians. The geometry is derived from the object's persisted mesh on every application. Its hard bounds (vertices named once) and its `x-semio-invariant` (a non-zero axis) state the payload-intrinsic breaches the diff refuses as `mutation.invariant`; refusals that depend on the base belong to the diff alone."
        properties["pivot"] = vector_input("Pivot", "Drehpunkt", "The point the axis passes through.", "Der Punkt, durch den die Achse verläuft.", "pivot", 40)
        properties["axis"] = vector_input("Axis", "Achse", "The direction the vertices turn about; its length does not matter.", "Die Richtung, um die sich die Eckpunkte drehen; ihre Länge spielt keine Rolle.", "motion", 30)
        properties["angle"] = OrderedDict([("type", "number"), ("x-semio-ui", OrderedDict([("widget", "dial"), ("role", "value"), ("label", label("Angle", "Winkel")), ("description", label("Right-handed turn about the axis.", "Rechtshändige Drehung um die Achse.")), ("unit", "rad"), ("displayUnit", "deg"), ("displayFactor", 57.29577951308232), ("step", 0.017453292519943295), ("softMin", -3.141592653589793), ("softMax", 3.141592653589793), ("snaps", [-3.141592653589793, -1.5707963267948966, 0, 1.5707963267948966, 3.141592653589793]), ("group", "motion"), ("order", 35)]))])
        required = ["angle", "axis", "objectId", "pivot", "vertexIds"]
        invariants = [OrderedDict([("id", "axis-nonzero"), ("description", label("The rotation axis is not the zero vector.", "Die Drehachse ist nicht der Nullvektor."))])]
    else:
        description = "The gumball's scaling of a selection stated as its intent: the object, the vertices it names (every vertex when empty), the pivot and one factor per axis. The geometry is derived from the object's persisted mesh on every application. Its hard bounds (vertices named once, positive factors) state the payload-intrinsic breaches the diff refuses as `mutation.invariant`; refusals that depend on the base belong to the diff alone."
        properties["pivot"] = vector_input("Pivot", "Bezugspunkt", "The point that stays where it is.", "Der Punkt, der an seinem Ort bleibt.", "pivot", 40)
        properties["factor"] = vector_input("Factor", "Faktor", "The factor along x, y and z; 1 keeps the size.", "Der Faktor entlang x, y und z; 1 behält die Größe.", "motion", 30, step=0.01, positive=True)
        required = ["factor", "objectId", "pivot", "vertexIds"]
    schema = OrderedDict([("$schema", "http://json-schema.org/draft-07/schema#"), ("$id", f"{ID_BASE}/{kind}/schema.json"), ("title", variant), ("description", description), ("type", "object"), ("additionalProperties", False), ("required", required)])
    if invariants:
        schema["x-semio-invariant"] = invariants
    schema["properties"] = properties
    return schema


def descriptor(kind):
    leaf = LEAVES[kind]
    return OrderedDict([("schemaVersion", 1), ("owner", f"✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{leaf['emoji']}{kind}"), ("semanticKind", kind), ("displayName", " ".join(word.capitalize() for word in kind.split("-"))), ("emoji", leaf["emoji"]), ("aggregateVariant", leaf["variant"]), ("payloadSchema", "🧬️schema/🔣️.json"), ("textOpcode", None), ("binaryTag", leaf["tag"]), ("invertibility", "explicit-mutation"), ("diffParticipation", "detect"), ("outcomeClasses", ["applied", "no-op", "rejected"]), ("composition", "atomic"), ("requiredLanguageSurfaces", ["rust", "json-schema", "text", "binary"])])
# endregion 🧬️Schemas


# region 📝️Files
def dump(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle_:
        handle_.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def read(path):
    with open(path, encoding="utf-8") as handle_:
        return handle_.read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle_:
        handle_.write(text)


def once(text, old, new, where):
    if new in text:
        return text
    assert text.count(old) == 1, f"{where}: anchor not unique/absent: {old[:120]!r}"
    return text.replace(old, new, 1)


def test_file(kind, case, story, forward):
    leaf = LEAVES[kind]
    rel = f"../../../../../🧫️fixtures/🧬️mutations/{leaf['emoji']}{kind}/{case}"
    constants = [f'const BEFORE: &str = include_str!("{rel}/📸️snapshot/⬅️before/🔣️.json");', f'const AFTER: &str = include_str!("{rel}/📸️snapshot/➡️after/🔣️.json");', f'const MUTATION: &str = include_str!("{rel}/🦠️mutation/🔣️.json");']
    if forward:
        constants.append(f'const DIFF: &str = include_str!("{rel}/🔺️diff/🔣️.json");')
    constants.append(f'const OUTCOME: &str = include_str!("{rel}/🎯️outcome/🔣️.json");')
    header = f"""//! 🧪️ `{kind}` fixture — `{case}`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 half-edge mesh in
//! `{SCRIPT}`.
//!
//! {story}

use super::laws;

{chr(10).join(constants)}
"""
    if forward:
        return header + """
/// ▶️ The motion carries `before` to exactly the committed `after` and produces exactly the committed delta.
#[test]
fn applies_to_committed_after_with_the_committed_diff() {
    laws::forward(BEFORE, MUTATION, AFTER, DIFF);
}

/// ↩️ The computed inverse — the prior mesh handle and content written back as one edit — restores `before` exactly.
#[test]
fn inverse_restores_before() {
    laws::inverse_restores(BEFORE, MUTATION);
}

/// 🎯️ The declared outcome — status and ordered diagnostics — is what the leaf emits.
#[test]
fn declared_outcome_holds() {
    laws::declared_outcome(BEFORE, MUTATION, OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE, AFTER, MUTATION, Some(DIFF));
}
"""
    return header + """
/// ⛔️ The refused or no-op motion leaves the document byte-identical and emits the declared diagnostic.
#[test]
fn refusal_leaves_the_document_untouched() {
    laws::refusal(BEFORE, MUTATION, AFTER, OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE, AFTER, MUTATION, None);
}
"""


def module_name(case):
    return "tests_" + "".join(char for char in case if char.isascii())
# endregion 📝️Files


# region 🧬️Register
def mount(cases):
    path = ARTIFACT + "/🦀️.rs"
    source = read(path)
    for kind, leaf in LEAVES.items():
        directory = f"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{leaf['emoji']}{kind}"
        tests = []
        for case in cases[kind]:
            tests += ["                            #[cfg(test)]", f'                            #[path = "{directory}/🧪️tests/{case}/🦀️.rs"]', f"                            mod {module_name(case)};"]
        block = "\n".join(["                        #[path = \".\"]", f"                        pub mod {leaf['module']} {{",
                           f'                            #[path = "{directory}/🦀️.rs"]', "                            mod component;",
                           f'                            #[path = "{directory}/🔺️diff/🦀️.rs"]', "                            pub mod diff;",
                           f'                            #[path = "{directory}/↩️inverse/🦀️.rs"]', "                            pub mod inverse;",
                           "                            pub use component::*;", *tests, "                        }"]) + "\n"
        start = source.find(f"                        #[path = \".\"]\n                        pub mod {leaf['module']} {{")
        if start >= 0:
            end = source.index("                        }\n", source.index(f"pub mod {leaf['module']} {{", start)) + len("                        }\n")
            source = source[:start] + block + source[end:]
        else:
            anchor_start = source.index("                        pub mod apply_paint_stroke {")
            anchor = source.index("                        }\n", anchor_start) + len("                        }\n")
            for previous in ("move_selection", "rotate_selection"):
                if f"pub mod {previous} {{" in source and kind != "move-selection":
                    previous_start = source.index(f"                        pub mod {previous} {{")
                    anchor = max(anchor, source.index("                        }\n", previous_start) + len("                        }\n"))
            source = source[:anchor] + block + source[anchor:]
    write(path, source)


def register_aggregate():
    path = MUTATIONS + "/🦀️.rs"
    text = read(path)
    text = once(text, "    ApplyPaintStroke(super::apply_paint_stroke::ApplyPaintStroke),\n}", "    ApplyPaintStroke(super::apply_paint_stroke::ApplyPaintStroke),\n    MoveSelection(super::move_selection::MoveSelection),\n    RotateSelection(super::rotate_selection::RotateSelection),\n    ScaleSelection(super::scale_selection::ScaleSelection),\n}", "enum")
    text = once(text, '    "apply-paint-stroke",\n];', '    "apply-paint-stroke",\n    "move-selection",\n    "rotate-selection",\n    "scale-selection",\n];', "KINDS")
    write(path, text)

    path = MUTATIONS + "/🔣️.json"
    data = json.loads(read(path), object_pairs_hook=OrderedDict)
    for kind, leaf in LEAVES.items():
        if not any(leaf["variant"] in arm.get("properties", {}) for arm in data["oneOf"]):
            data["oneOf"].append(OrderedDict([("type", "object"), ("additionalProperties", False), ("required", [leaf["variant"]]), ("properties", OrderedDict([(leaf["variant"], OrderedDict([("$ref", f"{ID_BASE}/{kind}/schema.json")]))]))]))
    data["description"] = data["description"].replace("Eighteen-variant", "Twenty-one-variant")
    write(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")

    path = MUTATIONS + "/🟦️.ts"
    text = read(path)
    text = once(text, "  | { ApplyPaintStroke: { objectId: string; layerIndex: number; eraser: boolean; color: [number, number, number, number]; radius: number; hardness: number; opacity: number; points: [number, number][] } };",
                "  | { ApplyPaintStroke: { objectId: string; layerIndex: number; eraser: boolean; color: [number, number, number, number]; radius: number; hardness: number; opacity: number; points: [number, number][] } }\n  | { MoveSelection: { objectId: string; vertexIds: number[]; offset: [number, number, number] } }\n  | { RotateSelection: { objectId: string; vertexIds: number[]; pivot: [number, number, number]; axis: [number, number, number]; angle: number } }\n  | { ScaleSelection: { objectId: string; vertexIds: number[]; pivot: [number, number, number]; factor: [number, number, number] } };", "ts union")
    text = once(text, '  "ApplyPaintStroke",\n] as const;', '  "ApplyPaintStroke",\n  "MoveSelection",\n  "RotateSelection",\n  "ScaleSelection",\n] as const;', "ts tags")
    text = text.replace("six paint-layer verbs, one pixel edit and one paint stroke)", "six paint-layer verbs, one pixel edit, one paint stroke and three selection motions)")
    text = text.replace("eighteen variants:", "twenty-one variants:")
    write(path, text)

    path = MUTATIONS + "/🔗️.graphql"
    text = read(path)
    inputs = "\ninput MoveSelectionInput {\n  objectId: String!\n  vertexIds: [Int!]!\n  offset: [Float!]!\n}\n\ninput RotateSelectionInput {\n  objectId: String!\n  vertexIds: [Int!]!\n  pivot: [Float!]!\n  axis: [Float!]!\n  angle: Float!\n}\n\ninput ScaleSelectionInput {\n  objectId: String!\n  vertexIds: [Int!]!\n  pivot: [Float!]!\n  factor: [Float!]!\n}\n"
    text = once(text, "input ApplyPaintStrokeInput {\n  objectId: String!\n  layerIndex: Int!\n  eraser: Boolean!\n  color: [Int!]!\n  radius: Float!\n  hardness: Float!\n  opacity: Float!\n  points: [[Float!]!]!\n}\n", "input ApplyPaintStrokeInput {\n  objectId: String!\n  layerIndex: Int!\n  eraser: Boolean!\n  color: [Int!]!\n  radius: Float!\n  hardness: Float!\n  opacity: Float!\n  points: [[Float!]!]!\n}\n" + inputs, "graphql input")
    text = once(text, "  APPLY_PAINT_STROKE\n}", "  APPLY_PAINT_STROKE\n  MOVE_SELECTION\n  ROTATE_SELECTION\n  SCALE_SELECTION\n}", "graphql enum")
    text = once(text, "  applyPaintStroke: ApplyPaintStrokeInput\n}", "  applyPaintStroke: ApplyPaintStrokeInput\n  moveSelection: MoveSelectionInput\n  rotateSelection: RotateSelectionInput\n  scaleSelection: ScaleSelectionInput\n}", "graphql envelope")
    text = once(text, "  | ApplyPaintStrokeMutation\n", "  | ApplyPaintStrokeMutation\n  | MoveSelectionMutation\n  | RotateSelectionMutation\n  | ScaleSelectionMutation\n", "graphql union")
    types = "\ntype MoveSelectionMutation {\n  mutation: LowpolyMutationKind!\n  payload: MoveSelectionPayload!\n}\ntype MoveSelectionPayload {\n  objectId: String!\n  vertexIds: [Int!]!\n  offset: [Float!]!\n}\n\ntype RotateSelectionMutation {\n  mutation: LowpolyMutationKind!\n  payload: RotateSelectionPayload!\n}\ntype RotateSelectionPayload {\n  objectId: String!\n  vertexIds: [Int!]!\n  pivot: [Float!]!\n  axis: [Float!]!\n  angle: Float!\n}\n\ntype ScaleSelectionMutation {\n  mutation: LowpolyMutationKind!\n  payload: ScaleSelectionPayload!\n}\ntype ScaleSelectionPayload {\n  objectId: String!\n  vertexIds: [Int!]!\n  pivot: [Float!]!\n  factor: [Float!]!\n}\n"
    text = once(text, "type ApplyPaintStrokePayload {\n  objectId: String!\n  layerIndex: Int!\n  eraser: Boolean!\n  color: [Int!]!\n  radius: Float!\n  hardness: Float!\n  opacity: Float!\n  points: [[Float!]!]!\n}\n", "type ApplyPaintStrokePayload {\n  objectId: String!\n  layerIndex: Int!\n  eraser: Boolean!\n  color: [Int!]!\n  radius: Float!\n  hardness: Float!\n  opacity: Float!\n  points: [[Float!]!]!\n}\n" + types, "graphql type")
    write(path, text)

    path = MUTATIONS + "/🛰️.proto"
    text = read(path)
    messages = "\nmessage MoveSelection {\n  string object_id = 1;\n  repeated uint32 vertex_ids = 2;\n  repeated float offset = 3;\n}\n\nmessage RotateSelection {\n  string object_id = 1;\n  repeated uint32 vertex_ids = 2;\n  repeated float pivot = 3;\n  repeated float axis = 4;\n  float angle = 5;\n}\n\nmessage ScaleSelection {\n  string object_id = 1;\n  repeated uint32 vertex_ids = 2;\n  repeated float pivot = 3;\n  repeated float factor = 4;\n}\n"
    text = once(text, "message ApplyPaintStroke {\n  string object_id = 1;\n  uint64 layer_index = 2;\n  bool eraser = 3;\n  repeated uint32 color = 4;\n  float radius = 5;\n  float hardness = 6;\n  float opacity = 7;\n  repeated StrokePoint points = 8;\n}\n", "message ApplyPaintStroke {\n  string object_id = 1;\n  uint64 layer_index = 2;\n  bool eraser = 3;\n  repeated uint32 color = 4;\n  float radius = 5;\n  float hardness = 6;\n  float opacity = 7;\n  repeated StrokePoint points = 8;\n}\n" + messages, "proto message")
    text = once(text, "    ApplyPaintStroke apply_paint_stroke = 18;\n", "    ApplyPaintStroke apply_paint_stroke = 18;\n    MoveSelection move_selection = 19;\n    RotateSelection rotate_selection = 20;\n    ScaleSelection scale_selection = 21;\n", "proto oneof")
    write(path, text)

    path = MUTATIONS + "/📖️.grammar.semio"
    text = read(path)
    text = once(text, "         | edit-paint-layer | apply-paint-stroke\n", "         | edit-paint-layer | apply-paint-stroke\n         | move-selection | rotate-selection | scale-selection\n", "grammar alternative")
    rules = 'move-selection = "move-selection" "object-id" "=" IDENT "vertex-ids" "=" "[" INT* "]" "offset" "=" vector\nrotate-selection = "rotate-selection" "object-id" "=" IDENT "vertex-ids" "=" "[" INT* "]" "pivot" "=" vector "axis" "=" vector "angle" "=" FLOAT\nscale-selection = "scale-selection" "object-id" "=" IDENT "vertex-ids" "=" "[" INT* "]" "pivot" "=" vector "factor" "=" vector\nvector = "[" FLOAT FLOAT FLOAT "]"\n'
    text = once(text, "stroke-point = \"[\" FLOAT FLOAT \"]\"\n", "stroke-point = \"[\" FLOAT FLOAT \"]\"\n" + rules, "grammar rule")
    write(path, text)

    path = MUTATIONS + "/💾️binary/📡️.protocol.semio"
    text = read(path)
    records = "record move-selection tag=19\nfield object-id utf8\nfield vertex-ids array varint\nfield offset array f32\nrecord rotate-selection tag=20\nfield object-id utf8\nfield vertex-ids array varint\nfield pivot array f32\nfield axis array f32\nfield angle f32\nrecord scale-selection tag=21\nfield object-id utf8\nfield vertex-ids array varint\nfield pivot array f32\nfield factor array f32\n"
    text = once(text, "field points array f32\n", "field points array f32\n" + records, "protocol record")
    write(path, text)


PY_KINDS = '    "apply-paint-stroke",\n)'
PY_SELECTION = '''def selection_number(value):
    """✍️ One float32 as the platform's JSON writer prints it inside a mesh: shortest round-trip digits, fixed
    notation for a decimal exponent within -5..=15 (a whole number keeps `.0`), exponential otherwise."""
    v = numpy.float32(value)
    if v == 0:
        return "-0.0" if numpy.signbit(v) else "0.0"
    mantissa, exponent = numpy.format_float_scientific(numpy.abs(v), unique=True, trim="-").split("e")
    exponent, digits, sign = int(exponent), mantissa.replace(".", ""), "-" if v < 0 else ""
    if -5 <= exponent <= 15:
        if exponent >= len(digits) - 1:
            return sign + digits + "0" * (exponent - (len(digits) - 1)) + ".0"
        if exponent >= 0:
            return sign + digits[:exponent + 1] + "." + digits[exponent + 1:]
        return sign + "0." + "0" * (-exponent - 1) + digits
    return sign + digits[0] + ("." + digits[1:] if len(digits) > 1 else "") + "e" + ("+" if exponent >= 0 else "") + str(exponent)


def selection_json(value):
    """✍️ The platform's compact JSON of a half-edge mesh value tree, members in declaration order."""
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        return str(value)
    if isinstance(value, float):
        return selection_number(value)
    if isinstance(value, list):
        return "[" + ",".join(selection_json(item) for item in value) + "]"
    return "{" + ",".join(json.dumps(key) + ":" + selection_json(item) for key, item in value.items()) + "}"


def selection_moved(content, payload, kind):
    """🧲️ A selection leaf's mesh: every named vertex (every vertex when none is named) carried by the motion in float64
    off its float32 position and stored back as float32 — an offset, a right-handed turn about the normalised axis
    through the pivot, or per-axis factors about the pivot — then every vertex normal recomputed as the normalised
    Newell normal of the first face visiting it. Libm's `hypot`, `cos` and `sin`, the platform's own."""
    f32 = lambda value: float(numpy.float32(value))
    libm = ctypes.CDLL(ctypes.util.find_library("m"))
    for name, arity in (("hypot", 2), ("cos", 1), ("sin", 1)):
        getattr(libm, name).restype = ctypes.c_double
        getattr(libm, name).argtypes = [ctypes.c_double] * arity
    mesh = json.loads(content, object_pairs_hook=collections.OrderedDict)
    count = len(mesh["vertices"])
    present = sorted({vertex for vertex in (payload["vertexIds"] or range(count)) if vertex < count})
    if kind == "move-selection":
        transform = lambda point: [point[axis] + float(payload["offset"][axis]) for axis in range(3)]
    elif kind == "rotate-selection":
        pivot, axis = [float(value) for value in payload["pivot"]], [float(value) for value in payload["axis"]]
        length = libm.hypot(libm.hypot(axis[0], axis[1]), axis[2])
        x, y, z = (value / length for value in axis)
        c, s = libm.cos(float(payload["angle"])), libm.sin(float(payload["angle"]))
        t = 1.0 - c

        def transform(point):
            px, py, pz = (point[index] - pivot[index] for index in range(3))
            return [pivot[0] + ((t * x * x + c) * px + (t * x * y - s * z) * py + (t * x * z + s * y) * pz), pivot[1] + ((t * x * y + s * z) * px + (t * y * y + c) * py + (t * y * z - s * x) * pz), pivot[2] + ((t * x * z - s * y) * px + (t * y * z + s * x) * py + (t * z * z + c) * pz)]
    else:
        pivot, factor = [float(value) for value in payload["pivot"]], [float(value) for value in payload["factor"]]
        transform = lambda point: [pivot[axis] + (point[axis] - pivot[axis]) * factor[axis] for axis in range(3)]
    for vertex in present:
        mesh["vertices"][vertex]["position"] = [f32(value) for value in transform([float(value) for value in mesh["vertices"][vertex]["position"]])]
    normalised = lambda vector, length: [0.0, 0.0, 0.0] if length == 0.0 else [f32(float(value) / length) for value in vector]
    flat = [None] * count
    for face in mesh["faces"]:
        ids, edge = [], face["halfedge"]
        while True:
            ids.append(mesh["halfedges"][edge]["vertex"])
            edge = mesh["halfedges"][edge]["next"]
            if edge == face["halfedge"]:
                break
        points = [mesh["vertices"][vertex]["position"] for vertex in ids]
        origin = points[0]
        nx = ny = nz = 0.0
        for index, a in enumerate(points):
            b = points[(index + 1) % len(points)]
            ax, ay, az = (float(a[k]) - float(origin[k]) for k in range(3))
            bx, by, bz = (float(b[k]) - float(origin[k]) for k in range(3))
            nx += (ay - by) * (az + bz)
            ny += (az - bz) * (ax + bx)
            nz += (ax - bx) * (ay + by)
        newell = normalised([nx, ny, nz], libm.hypot(libm.hypot(nx, ny), nz))
        normal = normalised(newell, libm.hypot(libm.hypot(newell[0], newell[1]), newell[2]))
        for vertex in ids:
            if flat[vertex] is None:
                flat[vertex] = normal
    for vertex, normal in zip(mesh["vertices"], flat):
        vertex["normal"] = normal
    return selection_json(mesh)


def selection_handle(object_id, content):
    """🆔️ The content-addressed mesh child handle the persisted content hashes to: `mesh-` + the first 16 lowercase hex
    digits of SHA-256 over the content, aimed at the object's `<id>-mesh` child in the `s.stdio.semio` v1 mesh subset."""
    return {"childId": "mesh-" + hashlib.sha256(content.encode("utf-8")).hexdigest()[:16], "target": {"artifactId": object_id + "-mesh", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "mesh"}}}


'''
PY_APPLY = '''    elif kind in ("move-selection", "rotate-selection", "scale-selection"):
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        if record["mesh"] is None or not record["meshContent"]:
            raise AssertionError("mutate-%s: object %r carries no mesh" % (kind, payload["objectId"]))
        record["meshContent"] = selection_moved(record["meshContent"], payload, kind)
        record["mesh"] = selection_handle(payload["objectId"], record["meshContent"])
'''
PY_INVERSE = '''    if kind in ("move-selection", "rotate-selection", "scale-selection"):
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        return [("create-mesh", {"id": payload["objectId"], "childId": record["mesh"]["childId"], "target": copy.deepcopy(record["mesh"]["target"]), "meshWorkspace": record["meshContent"]})]
'''


def register_harness(rows):
    path = HARNESS + "/🦀️.rs"
    text = read(path)
    text = once(text, '    "apply-paint-stroke",\n];', '    "apply-paint-stroke",\n    "move-selection",\n    "rotate-selection",\n    "scale-selection",\n];', "rust KINDS")
    write(path, text)

    path = HARNESS + "/🐍️.py"
    text = read(path)
    text = once(text, PY_KINDS, '    "apply-paint-stroke",\n    "move-selection",\n    "rotate-selection",\n    "scale-selection",\n)', "py KINDS")
    text = once(text, "import base64\nimport copy\nimport json\nimport math\n", "import base64\nimport collections\nimport copy\nimport ctypes\nimport ctypes.util\nimport hashlib\nimport json\nimport math\n", "py imports")
    text = once(text, '    else:\n        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)', PY_APPLY + '    else:\n        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)', "py apply")
    text = once(text, '    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)', PY_INVERSE + '    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)', "py inverse")
    text = once(text, "def object_at(document, identity, kind, where):", PY_SELECTION + "def object_at(document, identity, kind, where):", "py selection")
    write(path, text)

    path = HARNESS + "/🥒️.feature"
    text = read(path)
    anchor = "      | apply-paint-stroke            | 🖌️apply-paint-stroke/🖌️dabs |\n"
    assert text.count(anchor) == 2, "feature anchors"
    lines = "".join(f"      | {kind:<29} | {LEAVES[kind]['emoji']}{kind}/{case} |\n" for kind, case in rows)
    if lines not in text:
        text = text.replace(anchor, anchor + lines)
    write(path, text)

    path = ANY + "/🔮️oracles/🔣️.json"
    data = json.loads(read(path), object_pairs_hook=OrderedDict)
    catalog = data["mutationCatalogs"][0]
    manifests = data["mutationManifests"][0]["mutations"]
    for kind, leaf in LEAVES.items():
        if kind not in catalog["kinds"]:
            catalog["kinds"].append(kind)
        catalog["vectors"] = [vector for vector in catalog["vectors"] if vector["mutationId"] != kind]
        catalog["vectors"].append(OrderedDict([("mutationId", kind), ("sourceMutationDirectoryName", leaf["emoji"] + kind), ("mutationDirectoryName", leaf["emoji"] + kind), ("scenarios", [OrderedDict([("id", SCENARIO_IDS[(kind, case)]), ("directoryName", case)]) for scenario_kind, case, *_ in SCENARIOS if scenario_kind == kind])]))
        if not any(manifest["id"] == kind for manifest in manifests):
            manifests.append(OrderedDict([("id", kind), ("capability", "lowpoly-1-mutate"), ("payloadSchema", "🧬️.schema.json"), ("outcomes", ["applied", "no-op", "rejected"]), ("productionDispatch", OrderedDict([("operation", kind), ("bridgeVersion", 1), ("variant", leaf["variant"])])), ("oracleRequirements", [OrderedDict([("capability", "lowpoly-1-mutate"), ("qualifyingKind", "verified-native-second-implementation")])])]))
    write(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")
# endregion 🧬️Register


def main():
    for kind, leaf in LEAVES.items():
        root = f"{MUTATIONS}/{leaf['emoji']}{kind}"
        dump(f"{root}/🧬️schema/🔣️.json", leaf_schema(kind))
        dump(f"{root}/🔣️.json", descriptor(kind))
    cases = {kind: [] for kind in LEAVES}
    rows = []
    for kind, case, story, mutation, invariant in SCENARIOS:
        leaf = LEAVES[kind]
        before = document()
        after, diff, outcome = apply_selection(before, kind, mutation)
        root = f"{FIXTURES}/{leaf['emoji']}{kind}/{case}"
        dump(f"{root}/📸️snapshot/⬅️before/🔣️.json", before)
        dump(f"{root}/📸️snapshot/➡️after/🔣️.json", after)
        dump(f"{root}/🦠️mutation/🔣️.json", mutation)
        dump(f"{root}/🎯️outcome/🔣️.json", outcome)
        absent = f"{root}/🔺️diff/🚫️.absent"
        if diff is None:
            write(absent, "")
            if os.path.exists(f"{root}/🔺️diff/🔣️.json"):
                os.remove(f"{root}/🔺️diff/🔣️.json")
        else:
            dump(f"{root}/🔺️diff/🔣️.json", diff)
            if os.path.exists(absent):
                os.remove(absent)
        forward = diff is not None and diff["objects"] is not None
        if forward:
            rows.append((kind, case))
        cases[kind].append(case)
        write(f"{MUTATIONS}/{leaf['emoji']}{kind}/🧪️tests/{case}/🦀️.rs", test_file(kind, case, story, forward))
        print(kind, case, outcome["status"], outcome.get("code", [m["code"] for m in outcome.get("messages", [])]), len(case.encode("utf-8")))
    mount(cases)
    register_aggregate()
    register_harness(rows)


if __name__ == "__main__":
    main()
