#!/usr/bin/env python3
"""One-off derivation: real pattern-sphere.glb geometry -> real multi-thousand-element ASCII PLY 1.0.

Source: 🧰️framework/🔨️modules/🖼️assets/🖼️images/🧊️pattern-sphere.glb (679 KB, real committed art asset;
the same source the OBJ 3.0 wave-7 case derived its own real fixture from). GLB container is parsed
by hand (12-byte header, JSON chunk, BIN chunk) since no gltf crate/lib is linked in this repo.
Single mesh, single primitive: POSITION (VEC3 f32, 8448), NORMAL (VEC3 f32, 8448), TEXCOORD_0 (VEC2
f32, 8448), indices (SCALAR u32, 48384 -> 16128 real triangles).

Emits:
  - element vertex 8449: x y z nx ny nz s t (float) -- the real 8448 accessor rows plus one real
    trailing orphan duplicate of vertex 0 (unreferenced by any face), giving `remove-row`/
    `insert-row` on "vertex" a real target that needs no cascading face-index repair.
  - element face 16128: property list uchar int vertex_indices -- the real 16128 triangles, 0-based
    (PLY, unlike OBJ, is natively 0-based).
  - element edge <N>: property int v1, property int v2 -- real unique undirected edges extracted
    from the mesh's own first 24 real triangles (a genuine sub-structure of the real topology, not
    fabricated indices), sized small on purpose so `remove-element` has a real, cheaply-described
    target distinct from the two primary elements.
"""
import struct
import json

SRC = "🧰️framework/🔨️modules/🖼️assets/🖼️images/🧊️pattern-sphere.glb"
OUT = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/scratch-ply-1-0/pattern-sphere.ply"

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

print(f"vertices={len(positions)} normals={len(normals)} texcoords={len(texcoords)} triangles={len(triangles)}")

# 🧷 One extra real vertex row, a duplicate of index 0's real value, appended unreferenced by any
# face -- gives "vertex" a real remove-row/insert-row target with no cascading face-index repair.
vertex_count = len(positions) + 1

# 🕸️ Real undirected edges extracted from the first 24 real triangles' own topology, deduplicated.
edge_set = []
seen = set()
for tri in triangles[:24]:
    for a, b in ((tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])):
        key = (min(a, b), max(a, b))
        if key not in seen:
            seen.add(key)
            edge_set.append(key)

lines = []
lines.append("ply")
lines.append("format ascii 1.0")
lines.append("comment stdio.ply 1.0 real-world fixture, derived once from real committed geometry.")
lines.append(f"comment source: shared-glb {SRC}")
lines.append("comment derivation: hand-parsed GLB container (12-byte header, JSON chunk, BIN chunk); POSITION/NORMAL/TEXCOORD_0")
lines.append("comment accessors and the index accessor read directly with plain struct decoding (this script), no gltf crate.")
lines.append("comment Vertex/normal/texcoord/face data below is the real mesh; the edge element is real topology extracted")
lines.append("comment from the mesh's own first 24 real triangles. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR.")
lines.append("comment trailing orphan vertex row below (last vertex index): a duplicate of index 0, unreferenced by any face")
lines.append(f"element vertex {vertex_count}")
lines.append("property float x")
lines.append("property float y")
lines.append("property float z")
lines.append("property float nx")
lines.append("property float ny")
lines.append("property float nz")
lines.append("property float s")
lines.append("property float t")
lines.append(f"element face {len(triangles)}")
lines.append("property list uchar int vertex_indices")
lines.append(f"element edge {len(edge_set)}")
lines.append("property int v1")
lines.append("property int v2")
lines.append("end_header")

for i in range(len(positions)):
    p = positions[i]
    n = normals[i]
    t = texcoords[i]
    lines.append(f"{p[0]:.6g} {p[1]:.6g} {p[2]:.6g} {n[0]:.6g} {n[1]:.6g} {n[2]:.6g} {t[0]:.6g} {t[1]:.6g}")
# trailing orphan duplicate of vertex 0
p, n, t = positions[0], normals[0], texcoords[0]
lines.append(f"{p[0]:.6g} {p[1]:.6g} {p[2]:.6g} {n[0]:.6g} {n[1]:.6g} {n[2]:.6g} {t[0]:.6g} {t[1]:.6g}")

for tri in triangles:
    lines.append(f"3 {tri[0]} {tri[1]} {tri[2]}")

for a, b in edge_set:
    lines.append(f"{a} {b}")

text = "\n".join(lines) + "\n"
with open(OUT, "w") as f:
    f.write(text)
print("wrote", OUT, len(text), "bytes,", len(lines), "lines, edges=", len(edge_set), "vertex_count=", vertex_count)
