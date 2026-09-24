import os, re

arts = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
gltf = next(os.path.join(arts, n) for n in os.listdir(arts) if "gltf" in n)
snap_path = None
for r, ds, fs in os.walk(gltf):
    for f in fs:
        if f.endswith(".rs") and "snapshot" in r and "diff" not in r:
            p = os.path.join(r, f)
            if "struct GltfDocument" in open(p, encoding="utf-8", errors="ignore").read(500):
                snap_path = p
                break

text = open(snap_path, encoding="utf-8").read()

def full_struct(name):
    idx = text.find(f"struct {name} ")
    if idx < 0:
        idx = text.find(f"struct {name}\n")
    # find matching braces
    start = text.find("{", idx)
    depth = 0
    i = start
    while i < len(text):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return text[idx : i + 1]
        i += 1
    return text[idx : idx + 800]

for name in ["GltfMesh", "GltfNode", "GltfScene", "GltfPrimitive", "GltfBuffer", "GltfBufferView", "GltfAccessor", "GltfAsset"]:
    print("=" * 60, name)
    print(full_struct(name))
    print()

# How other code constructs GltfMesh without Default
for r, ds, fs in os.walk(gltf):
    for f in fs:
        if not f.endswith(".rs"):
            continue
        p = os.path.join(r, f)
        t = open(p, encoding="utf-8", errors="ignore").read()
        if "GltfMesh {" in t and "primitives:" in t and "encode_glb" in t:
            print("BUILDER", p.split("gltf/")[-1][:100])
            idx = t.find("GltfMesh {")
            print(t[idx : idx + 600])

# DWG: drawing <-> snapshot
dwg = next(os.path.join(arts, n) for n in os.listdir(arts) if "dwg" in n)
for r, ds, fs in os.walk(dwg):
    for f in fs:
        if not f.endswith(".rs"):
            continue
        p = os.path.join(r, f)
        t = open(p, encoding="utf-8", errors="ignore").read()
        if ("DwgSnapshot" in t and "DwgDrawing" in t and ("from" in t.lower() or "into" in t or "to_snapshot" in t or "from_drawing" in t)):
            for line in t.splitlines():
                if "DwgDrawing" in line and ("Snapshot" in line or "fn " in line):
                    if any(x in line for x in ["fn ", "impl", "From", "Into", "to_", "from_"]):
                        print("CONV", line.strip()[:160], "||", p.split("dwg/")[-1][:80])

# search drawing field in snapshot
for r, ds, fs in os.walk(dwg):
    for f in fs:
        if f.endswith(".rs") and "snapshot" in r:
            p = os.path.join(r, f)
            t = open(p, encoding="utf-8", errors="ignore").read()
            if "struct DwgSnapshot" in t:
                idx = t.find("struct DwgSnapshot")
                # print more fields
                print(t[idx : idx + 2000])
                break

# encode_las - does it fill header from points?
las = next(os.path.join(arts, n) for n in os.listdir(arts) if "las" in n)
for r, ds, fs in os.walk(las):
    for f in fs:
        if f.endswith(".rs"):
            p = os.path.join(r, f)
            t = open(p, encoding="utf-8", errors="ignore").read()
            if "pub fn encode_las" in t:
                idx = t.find("pub fn encode_las")
                print("ENCODE_LAS", t[idx : idx + 1200])
