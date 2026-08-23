#!/usr/bin/env python3
"""One-off derivation step 1/2: extracts the real POSITION accessor out of the real committed
🧊️pattern-sphere.glb (679 KB) by hand-parsing the GLB container (12-byte header, JSON chunk, BIN
chunk) -- same technique as scratch-obj-3-0/derive_obj.py, no gltf crate/lib involved. Writes one
"x y z" line per real vertex to positions.txt, consumed by the Rust las-1-0-fixture-derive binary
(step 2/2) which builds the actual LAS 1.0 point cloud with the real `las` 0.11 reference crate.
"""
import struct
import json

SRC = "🧰️framework/🔨️modules/🖼️assets/🖼️images/🧊️pattern-sphere.glb"
OUT = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/las-1-0-fixture-derive/positions.txt"

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
print(f"vertices={len(positions)}")

with open(OUT, "w") as f:
    for p in positions:
        f.write(f"{p[0]:.9g} {p[1]:.9g} {p[2]:.9g}\n")
print("wrote", OUT, len(positions), "real vertex positions")
