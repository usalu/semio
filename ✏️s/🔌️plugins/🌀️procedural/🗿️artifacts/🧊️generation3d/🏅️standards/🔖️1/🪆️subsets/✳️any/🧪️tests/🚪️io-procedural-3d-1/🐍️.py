#!/usr/bin/env python3
"""🚪️ An INDEPENDENT second implementation of the three text mesh grammars `s.procedural.generation3d`
exports and imports — ASCII STL, Wavefront OBJ and ASCII PLY — serving as this case's differential
oracle, in a language that shares no code with the subject.

**What this case exists to catch.** Until ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, seven of this
artifact's nine format leaves were placeholders that could not fail: export returned the artifact's
own DSL text under the target format's name, and import discarded the bytes for an empty document.
Both directions reported success. A differential is the only shape of test that separates "the codec
works" from "our reader accepts our writer", because a reader and a writer that share a wrong
assumption agree with each other perfectly.

**Why a second implementation and not a third-party library.** The three grammars here are small,
fully specified plain-text formats, and the SUBJECT side of this comparison is not our Rust codec —
it is the repository's own dependency-free reader replaying committed bytes (see `🦀️.rs` beside this
file). A third-party reference IS separately registered and separately exercised for the same
geometry: the Rust round-trip lane (`[[test]] io-round-trip`, `🚪️io/🧪️tests/🔁️round-trip/🦀️.rs`)
recomputes every recovered mesh's enclosed volume and bounds with `parry3d`. This file adds what that
lane cannot: a reader written in another language, from the format specifications rather than from
our code.

**What it was written from.**

* the ASCII STL grammar — `solid <name>` / `facet normal i j k` / `outer loop` / three `vertex x y z`
  / `endloop` / `endfacet` / `endsolid` (https://en.wikipedia.org/wiki/STL_(file_format)).
* the Wavefront OBJ grammar — `v x y z`, `f a b c` with 1-based indices and optional `/vt/vn`
  references, `o <name>` object blocks.
* the ASCII PLY grammar — the `ply` magic, `format ascii 1.0`, `element <name> <count>`,
  `property <type> <name>`, `property list <count-type> <value-type> <name>`, `end_header`, then one
  whitespace-separated row per element instance.
* `🧫️fixtures/🚪️io/🧊️unit-cube/🔣️.json` — the one committed geometry, for the expected numbers.

**No Rust was read to write the grammars.** The committed
`🧫️fixtures/🚪️io/🧊️unit-cube/*second-implementation.*` files ARE this file's own output, written by
`write_stl`/`write_obj`/`write_ply` below; the subject half reads them back, and the Rust round-trip
lane feeds them through the artifact's real import leaves. So the comparison runs in both directions:
our reader against this writer, and this reader against these bytes.
"""

# region 🔖️Imports
import json
import math
import os

# endregion 🔖️Imports


# region 🔖️Geometry
TOLERANCE = 1e-3
"""🎚️ The comparison tolerance, matched to the Rust lane's: wide enough for `f32` coordinate storage,
narrow enough that a genuinely wrong coordinate fails."""

FIXTURE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🧫️fixtures", "🚪️io", "🧊️unit-cube")
"""🧊️ The committed geometry directory, shared with the Rust round-trip lane."""


def unit_cube():
    """🧊️ The committed cube as (vertices, triangles) — 8 shared vertices, 12 triangles."""
    with open(os.path.join(FIXTURE, "🔣️.json"), "r", encoding="utf-8") as handle:
        document = json.load(handle)
    flat = document["positions"]
    vertices = [tuple(flat[at : at + 3]) for at in range(0, len(flat), 3)]
    indices = document["indices"]
    triangles = [tuple(indices[at : at + 3]) for at in range(0, len(indices), 3)]
    if len(vertices) != 8 or len(triangles) != 12:
        raise AssertionError("the committed fixture is a unit cube: 8 vertices and 12 triangles, found %d and %d" % (len(vertices), len(triangles)))
    return vertices, triangles


def signed_volume(corners):
    """📦️ The enclosed volume of a closed triangle soup, by the divergence theorem: one sixth of the
    summed scalar triple product of every triangle's three corners. Positive for outward-wound
    faces, so a codec that reversed a winding shows up as a negative number rather than as a still
    plausible triangle count."""
    total = 0.0
    for a, b, c in corners:
        total += a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])
    return total / 6.0


def bounds(points):
    """📐️ The axis-aligned bounding box of a point set."""
    lows = [min(point[axis] for point in points) for axis in range(3)]
    highs = [max(point[axis] for point in points) for axis in range(3)]
    return lows, highs


def projection(fmt, vertices, triangles):
    """🔁️ The shape every producer and reader in this case is compared through — the subset all three
    grammars can carry, so one expectation serves all of them."""
    corners = [tuple(vertices[index] for index in triangle) for triangle in triangles]
    lows, highs = bounds(vertices)
    return {"format": fmt, "triangleCount": len(triangles), "vertexCount": len(vertices), "min": lows, "max": highs, "volume": signed_volume(corners)}


def equals(where, actual, expected):
    """✅️ Compares two projections within tolerance, naming the first field that moved."""
    if actual["format"] != expected["format"]:
        raise AssertionError("%s: format %r is not %r" % (where, actual["format"], expected["format"]))
    for field in ("triangleCount", "vertexCount"):
        if actual[field] != expected[field]:
            raise AssertionError("%s: %s is %r, expected %r" % (where, field, actual[field], expected[field]))
    for field in ("min", "max"):
        for axis in range(3):
            if not math.isclose(actual[field][axis], expected[field][axis], abs_tol=TOLERANCE):
                raise AssertionError("%s: %s[%d] is %r, expected %r" % (where, field, axis, actual[field][axis], expected[field][axis]))
    if not math.isclose(actual["volume"], expected["volume"], abs_tol=TOLERANCE):
        raise AssertionError("%s: volume is %r, expected %r" % (where, actual["volume"], expected["volume"]))
# endregion 🔖️Geometry


# region 🔖️Stl
def write_stl(vertices, triangles):
    """🔺️ ASCII STL, per the grammar. STL shares no vertex pool: every facet writes its own three
    corners, and its own outward normal computed from the winding."""
    lines = ["solid generation3d-second-implementation"]
    for triangle in triangles:
        a, b, c = (vertices[index] for index in triangle)
        u = (b[0] - a[0], b[1] - a[1], b[2] - a[2])
        v = (c[0] - a[0], c[1] - a[1], c[2] - a[2])
        n = (u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0])
        length = math.sqrt(n[0] ** 2 + n[1] ** 2 + n[2] ** 2) or 1.0
        lines.append("  facet normal %.6f %.6f %.6f" % (n[0] / length, n[1] / length, n[2] / length))
        lines.append("    outer loop")
        for corner in (a, b, c):
            lines.append("      vertex %.6f %.6f %.6f" % corner)
        lines.append("    endloop")
        lines.append("  endfacet")
    lines.append("endsolid generation3d-second-implementation")
    return ("\n".join(lines) + "\n").encode("utf-8")


def read_stl(payload):
    """🔺️ Reads ASCII STL back into (vertices, triangles). The corners are kept in file order, which
    is what STL actually carries — no vertex welding, because the format states no shared pool."""
    vertices = []
    triangles = []
    current = []
    for line in payload.decode("utf-8").splitlines():
        token = line.strip().split()
        if not token:
            continue
        if token[0] == "vertex":
            if len(token) != 4:
                raise AssertionError("stl: a `vertex` record carries three coordinates, found %r" % (line,))
            current.append(tuple(float(value) for value in token[1:4]))
        elif token[0] == "endfacet":
            if len(current) != 3:
                raise AssertionError("stl: a facet carries exactly three vertices, found %d" % len(current))
            base = len(vertices)
            vertices.extend(current)
            triangles.append((base, base + 1, base + 2))
            current = []
    if not triangles:
        raise AssertionError("stl: the file declares no facets")
    return vertices, triangles
# endregion 🔖️Stl


# region 🔖️Obj
def write_obj(vertices, triangles):
    """🗿️ Wavefront OBJ, per the grammar. Indices are 1-based, and the shared vertex pool IS the
    format's own model, so this writer keeps the cube's 8 vertices."""
    lines = ["o generation3d-second-implementation"]
    for vertex in vertices:
        lines.append("v %.6f %.6f %.6f" % vertex)
    for triangle in triangles:
        lines.append("f %d %d %d" % tuple(index + 1 for index in triangle))
    return ("\n".join(lines) + "\n").encode("utf-8")


def read_obj(payload):
    """🗿️ Reads Wavefront OBJ back. `f` references may carry `v/vt/vn`, and a negative index is
    relative to the end of the pool — both are real spellings this reader accepts."""
    vertices = []
    triangles = []
    for line in payload.decode("utf-8").splitlines():
        token = line.strip().split()
        if not token:
            continue
        if token[0] == "v":
            vertices.append(tuple(float(value) for value in token[1:4]))
        elif token[0] == "f":
            corners = []
            for reference in token[1:]:
                index = int(reference.split("/")[0])
                corners.append(index - 1 if index > 0 else len(vertices) + index)
            for at in range(1, len(corners) - 1):
                triangles.append((corners[0], corners[at], corners[at + 1]))
    if not triangles:
        raise AssertionError("obj: the file declares no faces")
    return vertices, triangles
# endregion 🔖️Obj


# region 🔖️Ply
def write_ply(vertices, triangles):
    """🧱️ ASCII PLY, per the grammar: a `vertex` element with x/y/z float columns and a `face`
    element with one `vertex_indices` list column."""
    lines = [
        "ply",
        "format ascii 1.0",
        "comment generation3d second implementation",
        "element vertex %d" % len(vertices),
        "property float x",
        "property float y",
        "property float z",
        "element face %d" % len(triangles),
        "property list uchar int vertex_indices",
        "end_header",
    ]
    for vertex in vertices:
        lines.append("%.6f %.6f %.6f" % vertex)
    for triangle in triangles:
        lines.append("3 %d %d %d" % triangle)
    return ("\n".join(lines) + "\n").encode("utf-8")


def read_ply(payload):
    """🧱️ Reads ASCII PLY back by walking its declared header: element order and per-element property
    counts are what say how many rows to consume and how wide each one is."""
    text = payload.decode("utf-8").splitlines()
    if not text or text[0].strip() != "ply":
        raise AssertionError("ply: the file does not open with the `ply` magic")
    elements = []
    at = 1
    while at < len(text):
        token = text[at].strip().split()
        at += 1
        if not token:
            continue
        if token[0] == "format" and token[1] != "ascii":
            raise AssertionError("ply: this reader implements the ascii form, the file declares %r" % token[1])
        if token[0] == "element":
            elements.append({"name": token[1], "count": int(token[2]), "properties": []})
        elif token[0] == "property":
            if not elements:
                raise AssertionError("ply: a `property` appears before any `element`")
            elements[-1]["properties"].append("list" if token[1] == "list" else token[-1])
        elif token[0] == "end_header":
            break
    rows = [line for line in text[at:] if line.strip()]
    vertices = []
    triangles = []
    cursor = 0
    for element in elements:
        for _ in range(element["count"]):
            if cursor >= len(rows):
                raise AssertionError("ply: the header declares more rows than the body carries")
            fields = rows[cursor].split()
            cursor += 1
            if element["name"] == "vertex":
                names = element["properties"]
                vertices.append(tuple(float(fields[names.index(axis)]) for axis in ("x", "y", "z")))
            elif element["name"] == "face":
                count = int(fields[0])
                corners = [int(value) for value in fields[1 : 1 + count]]
                for hop in range(1, len(corners) - 1):
                    triangles.append((corners[0], corners[hop], corners[hop + 1]))
    if not triangles:
        raise AssertionError("ply: the file declares no faces")
    return vertices, triangles
# endregion 🔖️Ply


# region 🔖️Formats
FORMATS = {
    "stl": ("🔺️second-implementation.stl", write_stl, read_stl, 36),
    "obj": ("🗿️second-implementation.obj", write_obj, read_obj, 8),
    "ply": ("🧱️second-implementation.ply", write_ply, read_ply, 8),
}
"""🗂️ Per format: the committed file, this implementation's writer and reader, and the vertex count
the format's own model implies — 36 for STL, which shares no vertex pool, 8 for the two that do."""


def expected(fmt):
    """🧊️ The committed cube's numbers, as the named format is able to carry them."""
    vertices, triangles = unit_cube()
    shape = projection(fmt, vertices, triangles)
    shape["vertexCount"] = FORMATS[fmt][3]
    return shape


def committed_bytes(fmt):
    """📄️ The committed second-implementation file for a format."""
    with open(os.path.join(FIXTURE, FORMATS[fmt][0]), "rb") as handle:
        return handle.read()
# endregion 🔖️Formats


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison. The harness module is imported here rather than at file scope so this file
    stays runnable on its own (`python3 🐍️.py`) for the self-check below."""
    from semio_repo_test import Adapter, Outcome

    def read_handler(fmt):
        def handler(ctx):
            del ctx
            vertices, triangles = FORMATS[fmt][2](committed_bytes(fmt))
            shape = projection(fmt, vertices, triangles)
            equals("read-%s" % fmt, shape, expected(fmt))
            return Outcome(shape)

        return handler

    def round_trip_handler(fmt):
        def handler(ctx):
            del ctx
            vertices, triangles = unit_cube()
            written = FORMATS[fmt][1](vertices, triangles)
            back = projection(fmt, *FORMATS[fmt][2](written))
            equals("round-trip-%s" % fmt, back, expected(fmt))
            return Outcome(back, raw=written)

        return handler

    built = Adapter("python")
    for fmt in FORMATS:
        built = built.oracle("read-%s" % fmt, read_handler(fmt))
        built = built.oracle("round-trip-%s" % fmt, round_trip_handler(fmt))
    return built
# endregion 🔖️Registration


# region 🔖️SelfCheck
def self_check():
    """🧪️ Runs both laws for all three formats without the harness, so this reference can be executed
    and trusted on its own before any coordinator is involved. It also REWRITES nothing: the
    committed files must already parse to the same projection this implementation writes."""
    for fmt in FORMATS:
        vertices, triangles = unit_cube()
        written = FORMATS[fmt][1](vertices, triangles)
        equals("round-trip-%s" % fmt, projection(fmt, *FORMATS[fmt][2](written)), expected(fmt))
        equals("read-%s" % fmt, projection(fmt, *FORMATS[fmt][2](committed_bytes(fmt))), expected(fmt))
        print("ok read-%s round-trip-%s" % (fmt, fmt))


if __name__ == "__main__":
    self_check()
# endregion 🔖️SelfCheck
