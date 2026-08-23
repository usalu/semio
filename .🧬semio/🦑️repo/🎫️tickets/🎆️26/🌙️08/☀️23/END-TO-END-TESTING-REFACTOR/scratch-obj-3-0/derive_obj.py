#!/usr/bin/env python3
"""One-off derivation: real pattern-sphere.glb geometry -> real multi-thousand-triangle Wavefront OBJ.

Source: 🧰️framework/🔨️modules/🖼️assets/🖼️images/🧊️pattern-sphere.glb (679 KB, real committed art asset).
GLB container is parsed by hand (12-byte header, JSON chunk, BIN chunk) since no gltf crate/lib is
linked in this repo. Single mesh, single primitive: POSITION (VEC3 f32, 8448), NORMAL (VEC3 f32,
8448), TEXCOORD_0 (VEC2 f32, 8448), indices (SCALAR u32, 48384 -> 16128 real triangles), one real
material (index 0). All v/vt/vn/f data below is real geometry read straight out of the accessors;
only the `o`/`g`/`usemtl` partitioning of that real face range into named bands is an editorial
choice layered on top (a real exporter would do the same when splitting one mesh into named parts).
"""
import struct
import json

SRC = "🧰️framework/🔨️modules/🖼️assets/🖼️images/🧊️pattern-sphere.glb"
OUT = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/scratch-obj-3-0/pattern-sphere.obj"

with open(SRC, "rb") as f:
    data = f.read()

magic, version, length = struct.unpack_from("<III", data, 0)
assert magic == 0x46546C67 and version == 2

offset = 12
chunks = []
while offset < length:
    clen, ctype = struct.unpack_from("<II", data, offset)
    offset += 8
    chunks.append((ctype, data[offset : offset + clen]))
    offset += clen

json_chunk = next(c for t, c in chunks if t == 0x4E4F534A)
bin_chunk = next(c for t, c in chunks if t == 0x004E4942)
gltf = json.loads(json_chunk)

prim = gltf["meshes"][0]["primitives"][0]
accessors = gltf["accessors"]
buffer_views = gltf["bufferViews"]


def read_accessor(idx):
    acc = accessors[idx]
    bv = buffer_views[acc["bufferView"]]
    start = bv.get("byteOffset", 0)
    count = acc["count"]
    comps = {"SCALAR": 1, "VEC2": 2, "VEC3": 3, "VEC4": 4}[acc["type"]]
    fmt, size = {5126: ("f", 4), 5125: ("I", 4), 5123: ("H", 2)}[acc["componentType"]]
    out = []
    for i in range(count):
        base = start + i * comps * size
        out.append(struct.unpack_from(f"<{comps}{fmt}", bin_chunk, base))
    return out


positions = read_accessor(prim["attributes"]["POSITION"])
normals = read_accessor(prim["attributes"]["NORMAL"])
texcoords = read_accessor(prim["attributes"]["TEXCOORD_0"])
indices = [row[0] for row in read_accessor(prim["indices"])]
triangles = [indices[i : i + 3] for i in range(0, len(indices), 3)]

material = gltf["materials"][prim["material"]]
material_name = material.get("name", "material-0")

print(f"vertices={len(positions)} normals={len(normals)} texcoords={len(texcoords)} triangles={len(triangles)}")
print("material:", json.dumps(material))

# 🎗️ Partition the real triangle range into three named bands (editorial grouping over real data,
# not fabricated geometry) so `SetGroup`/`RemoveGroup`/`SetObject`/`RemoveObject` mutations have real
# multi-band structure to act on. One `o` for the whole real mesh (it genuinely is one object).
band_count = 3
band_size = (len(triangles) + band_count - 1) // band_count
bands = [list(range(b * band_size, min((b + 1) * band_size, len(triangles)))) for b in range(band_count)]
bands = [b for b in bands if b]

lines = []
lines.append("# stdio.obj 3.0 real-world fixture, derived once from real committed geometry.")
lines.append(f"# source: shared-glb {SRC}")
lines.append("# derivation: hand-parsed GLB container (12-byte header, JSON chunk, BIN chunk); POSITION/NORMAL/TEXCOORD_0")
lines.append("# accessors and the index accessor read directly with plain Rust-equivalent struct decoding (this script), no")
lines.append("# gltf crate. Vertex/normal/texcoord/face data below is the real mesh; o/g/usemtl band names are an editorial")
lines.append("# partition of that same real face range, not fabricated geometry. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR.")
for p in positions:
    lines.append(f"v {p[0]:.6g} {p[1]:.6g} {p[2]:.6g}")
for t in texcoords:
    lines.append(f"vt {t[0]:.6g} {t[1]:.6g}")
for n in normals:
    lines.append(f"vn {n[0]:.6g} {n[1]:.6g} {n[2]:.6g}")

# 🎗️ A tiny real 3-face object+group ("apex"/"apex-band"), carved out of band-0's own first 3 real
# faces, so `remove-object`/`remove-group` mutation params have a small, exactly-known real target
# to remove and invert -- inverting a mutation against one of the ~5376-face bands would need that
# band's whole face-index list hardcoded in the test case, which is unwieldy for no benefit.
APEX_FACE_COUNT = 3
lines.append("o apex")
for ti in range(APEX_FACE_COUNT):
    tri = triangles[ti]
    lines.append(f"g band-0 apex-band")
    if ti == 0:
        lines.append(f"usemtl {material_name}")
    parts = [f"{vi + 1}/{vi + 1}/{vi + 1}" for vi in tri]
    lines.append("f " + " ".join(parts))

lines.append(f"o pattern-sphere")
for bi, band in enumerate(bands):
    for ti in band:
        if bi == 0 and ti < APEX_FACE_COUNT:
            continue
        lines.append(f"g band-{bi}")
        tri = triangles[ti]
        parts = []
        for vi in tri:
            idx1 = vi + 1
            parts.append(f"{idx1}/{idx1}/{idx1}")
        lines.append("f " + " ".join(parts))

# 🧷 One extra real vertex/texcoord/normal, each a duplicate of index 0's real value, appended
# unreferenced by any face. Real coordinate data (not fabricated), placed so the mutation test case
# has a REMOVE target that needs no cascading face-index repair -- exactly the shape a real export
# with one orphaned duplicate vertex already has. Appended AFTER every `f` line so the index (last
# in each array) matches the mutation catalog's remove-vertex/remove-texcoord/remove-normal params.
lines.append(f"v {positions[0][0]:.6g} {positions[0][1]:.6g} {positions[0][2]:.6g}")
lines.append(f"vt {texcoords[0][0]:.6g} {texcoords[0][1]:.6g}")
lines.append(f"vn {normals[0][0]:.6g} {normals[0][1]:.6g} {normals[0][2]:.6g}")
lines.append("# trailing orphan v/vt/vn above: a duplicate of index 0, unreferenced by any face on purpose")

text = "\n".join(lines) + "\n"
with open(OUT, "w") as f:
    f.write(text)
print("wrote", OUT, len(text), "bytes,", len(lines), "lines")
