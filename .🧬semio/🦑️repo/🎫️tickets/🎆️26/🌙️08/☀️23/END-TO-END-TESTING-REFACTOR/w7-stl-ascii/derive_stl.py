import struct, json, math, sys

SRC = "♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/🧊️hexagonal-cut-concrete-forest-left.glb"
OUT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🧫️fixtures/🧊️hexagonal-cut-concrete-forest-left.stl"
SOLID_NAME = "hexagonal-cut-concrete-forest-left"

def read_glb(path):
    data = open(path, "rb").read()
    magic, version, length = struct.unpack_from("<4sII", data, 0)
    assert magic == b"glTF", magic
    off = 12
    chunks = []
    while off < length:
        clen, ctype = struct.unpack_from("<II", data, off)
        coff = off + 8
        chunks.append((ctype, data[coff:coff+clen]))
        off = coff + clen
    json_bytes = next(c[1] for c in chunks if c[0] == 0x4E4F534A)
    bin_bytes = next(c[1] for c in chunks if c[0] == 0x004E4942)
    gltf = json.loads(json_bytes)
    return gltf, bin_bytes

COMPONENT_FMT = {5120: ("b", 1), 5121: ("B", 1), 5122: ("h", 2), 5123: ("H", 2), 5125: ("I", 4), 5126: ("f", 4)}
TYPE_COUNT = {"SCALAR": 1, "VEC2": 2, "VEC3": 3, "VEC4": 4, "MAT4": 16}

def read_accessor(gltf, bin_bytes, index):
    acc = gltf["accessors"][index]
    bv = gltf["bufferViews"][acc["bufferView"]]
    fmt_char, comp_size = COMPONENT_FMT[acc["componentType"]]
    n_comp = TYPE_COUNT[acc["type"]]
    stride = bv.get("byteStride", n_comp * comp_size)
    base = bv.get("byteOffset", 0) + acc.get("byteOffset", 0)
    out = []
    for i in range(acc["count"]):
        rec_off = base + i * stride
        values = struct.unpack_from("<" + fmt_char * n_comp, bin_bytes, rec_off)
        out.append(values)
    return out

def main():
    gltf, bin_bytes = read_glb(SRC)
    assert len(gltf["nodes"]) == 1 and "matrix" not in gltf["nodes"][0] and "translation" not in gltf["nodes"][0], "unexpected node transform"
    triangles = []
    mesh = gltf["meshes"][0]
    for prim in mesh["primitives"]:
        mode = prim.get("mode", 4)
        assert mode == 4, f"non-triangle primitive mode {mode}"
        positions = read_accessor(gltf, bin_bytes, prim["attributes"]["POSITION"])
        indices = [v[0] for v in read_accessor(gltf, bin_bytes, prim["indices"])]
        assert len(indices) % 3 == 0
        for i in range(0, len(indices), 3):
            a, b, c = (positions[indices[i]], positions[indices[i + 1]], positions[indices[i + 2]])
            triangles.append((a, b, c))

    def normal(a, b, c):
        u = (b[0]-a[0], b[1]-a[1], b[2]-a[2])
        v = (c[0]-a[0], c[1]-a[1], c[2]-a[2])
        n = (u[1]*v[2]-u[2]*v[1], u[2]*v[0]-u[0]*v[2], u[0]*v[1]-u[1]*v[0])
        length = math.sqrt(n[0]**2 + n[1]**2 + n[2]**2)
        if length == 0.0:
            return (0.0, 0.0, 0.0)
        return (n[0]/length, n[1]/length, n[2]/length)

    lines = [f"solid {SOLID_NAME}"]
    for a, b, c in triangles:
        nx, ny, nz = normal(a, b, c)
        lines.append(f"  facet normal {nx} {ny} {nz}")
        lines.append("    outer loop")
        for v in (a, b, c):
            lines.append(f"      vertex {v[0]} {v[1]} {v[2]}")
        lines.append("    endloop")
        lines.append("  endfacet")
    lines.append(f"endsolid {SOLID_NAME}")
    text = "\n".join(lines) + "\n"

    with open(OUT, "w") as f:
        f.write(text)

    print("triangles:", len(triangles))
    print("bytes written:", len(text.encode("utf-8")))
    print("output:", OUT)

if __name__ == "__main__":
    main()
