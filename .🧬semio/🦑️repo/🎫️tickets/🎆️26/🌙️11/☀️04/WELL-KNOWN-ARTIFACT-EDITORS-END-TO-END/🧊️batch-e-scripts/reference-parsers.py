"""Independent stdlib reference parsers and writers for the batch E natural-file fixtures.

Usage: reference-parsers.py <artifacts-root>
Parses each `sample.<ext>` with code that shares nothing with the Rust codecs, writes the language-agnostic `expected.json` next to it, and writes the
binary samples (`sample.las`, `sample.glb`) with `struct` so the Rust decoders are checked against an independent writer.
"""
import json, re, struct, sys, pathlib

ROOT = pathlib.Path(sys.argv[1])
EDITORS = {
    "obj": "🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/✏️editor",
    "stl": "🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/✏️editor",
    "ply": "🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/✏️editor",
    "gltf": "🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/✏️editor",
    "las": "☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/✏️editor",
    "step": "📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/✏️editor",
}


def fixture(kind, name):
    directory = ROOT / EDITORS[kind] / "🧪️tests" / "🧫️fixtures" / "📂️natural"
    directory.mkdir(parents=True, exist_ok=True)
    return directory / name


def emit(kind, data):
    fixture(kind, "expected.json").write_text(json.dumps(data, indent=2) + "\n")


def obj():
    vertices, normals, faces, objects, mtllib = [], [], [], [], None
    for line in fixture("obj", "sample.obj").read_text().splitlines():
        t = line.split()
        if not t or t[0].startswith("#"):
            continue
        if t[0] == "v": vertices.append([float(x) for x in t[1:4]])
        elif t[0] == "vn": normals.append([float(x) for x in t[1:4]])
        elif t[0] == "f": faces.append([int(c.split("/")[0]) - 1 for c in t[1:]])
        elif t[0] == "o": objects.append(t[1])
        elif t[0] == "mtllib": mtllib = t[1]
    emit("obj", {"vertices": vertices, "normals": normals, "faces": faces, "objects": objects, "mtllib": mtllib})


def stl():
    solid, triangles, cur = None, [], None
    for line in fixture("stl", "sample.stl").read_text().splitlines():
        t = line.split()
        if t[:1] == ["solid"]: solid = t[1]
        elif t[:2] == ["facet", "normal"]: cur = {"normal": [float(x) for x in t[2:5]], "vertices": []}
        elif t[:1] == ["vertex"]: cur["vertices"].append([float(x) for x in t[1:4]])
        elif t[:1] == ["endfacet"]: triangles.append(cur)
    emit("stl", {"solid": solid, "triangles": triangles})


def ply():
    lines = fixture("ply", "sample.ply").read_text().splitlines()
    end = lines.index("end_header")
    header = lines[:end]
    counts = {h.split()[1]: int(h.split()[2]) for h in header if h.startswith("element")}
    body = lines[end + 1:]
    vertex = [[float(x) for x in row.split()] for row in body[: counts["vertex"]]]
    faces = [[int(x) for x in row.split()[1:]] for row in body[counts["vertex"]: counts["vertex"] + counts["face"]]]
    emit("ply", {"format": header[1].split()[1], "comments": [h[8:] for h in header if h.startswith("comment ")], "vertex": vertex, "faces": faces})


def las():
    scale, offset = (0.01, 0.01, 0.01), (100.0, 200.0, 0.0)
    points = [(101.25, 202.5, 3.0, 9, 2), (110.0, 210.0, 12.5, 40, 5)]
    header = bytearray(227)
    header[0:4] = b"LASF"
    header[24], header[25] = 1, 2
    struct.pack_into("<H", header, 94, 227)
    struct.pack_into("<I", header, 96, 227)
    header[104] = 0
    struct.pack_into("<H", header, 105, 20)
    struct.pack_into("<I", header, 107, len(points))
    struct.pack_into("<3d", header, 131, *scale)
    struct.pack_into("<3d", header, 155, *offset)
    xs, ys, zs = zip(*[p[:3] for p in points])
    struct.pack_into("<6d", header, 179, max(xs), min(xs), max(ys), min(ys), max(zs), min(zs))
    body = b"".join(
        struct.pack("<iiiHBBbBH", round((x - offset[0]) / scale[0]), round((y - offset[1]) / scale[1]), round((z - offset[2]) / scale[2]), i, 0x11, c, 0, 0, 7)
        for x, y, z, i, c in points
    )
    path = fixture("las", "sample.las")
    path.write_bytes(bytes(header) + body)
    data = path.read_bytes()
    n = struct.unpack_from("<I", data, 107)[0]
    sx, sy, sz = struct.unpack_from("<3d", data, 131)
    ox, oy, oz = struct.unpack_from("<3d", data, 155)
    read = []
    for i in range(n):
        x, y, z, inten, _flags, cls, _angle, _user, source = struct.unpack_from("<iiiHBBbBH", data, 227 + 20 * i)
        read.append({"x": x * sx + ox, "y": y * sy + oy, "z": z * sz + oz, "intensity": inten, "classification": cls, "pointSourceId": source})
    emit("las", {"scale": [sx, sy, sz], "offset": [ox, oy, oz], "points": read})


def gltf():
    document = json.loads(fixture("gltf", "sample.gltf").read_text())
    blob = json.dumps(document, separators=(",", ":")).encode()
    blob += b" " * (-len(blob) % 4)
    glb = b"glTF" + struct.pack("<II", 2, 12 + 8 + len(blob)) + struct.pack("<I", len(blob)) + b"JSON" + blob
    fixture("gltf", "sample.glb").write_bytes(glb)
    chunk = struct.unpack_from("<I4s", glb, 12)
    assert chunk[1] == b"JSON" and struct.unpack_from("<I", glb, 8)[0] == len(glb)
    local = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)]
    blob_bytes = b"".join(struct.pack("<3f", *p) for p in local)
    import base64
    triangle = {
        "asset": {"version": "2.0"}, "scene": 0, "scenes": [{"nodes": [0]}],
        "nodes": [{"name": "moved", "mesh": 0, "translation": [1.0, 2.0, 3.0], "scale": [2.0, 2.0, 2.0]}],
        "meshes": [{"primitives": [{"attributes": {"POSITION": 0}}]}],
        "accessors": [{"bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3"}],
        "bufferViews": [{"buffer": 0, "byteOffset": 0, "byteLength": len(blob_bytes)}],
        "buffers": [{"byteLength": len(blob_bytes), "uri": "data:application/octet-stream;base64," + base64.b64encode(blob_bytes).decode()}],
    }
    fixture("gltf", "triangle.gltf").write_text(json.dumps(triangle) + "\n")
    world = [[p[0] * 2 + 1, p[1] * 2 + 2, p[2] * 2 + 3] for p in local]
    emit("gltf", {"triangleWorld": world, "nodes": [{"name": n.get("name"), "translation": n.get("translation")} for n in document["nodes"]], "scene": document["scene"]})


def step():
    text = fixture("step", "sample.step").read_text()
    points = [[float(x) for x in m.group(2).split(",")] for m in re.finditer(r"#(\d+)=CARTESIAN_POINT\('',\(([^)]*)\)\);", text)]
    directions = [[float(x) for x in m.group(2).split(",")] for m in re.finditer(r"#(\d+)=DIRECTION\('',\(([^)]*)\)\);", text)]
    emit("step", {"cartesianPoints": points, "directions": directions, "schema": re.search(r"FILE_SCHEMA\(\('([^']*)'\)\)", text).group(1)})


for fn in (obj, stl, ply, las, gltf, step):
    fn()
print("ok")
