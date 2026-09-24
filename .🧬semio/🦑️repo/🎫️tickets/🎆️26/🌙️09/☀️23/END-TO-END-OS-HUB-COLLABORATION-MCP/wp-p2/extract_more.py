import os, re

LNK = "/Users/ueli/Documents/semio/.tmp-ticket/wp-p2/links"
arts = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"

snap = open(os.readlink(f"{LNK}/stdio-gltf-snapshot.rs"), encoding="utf-8").read()
idx = snap.find("struct GltfBuffer {")
print("BUFFER", snap[idx:idx+500])
# Default derives near GltfDocument, GltfAsset, GltfMesh, GltfNode, GltfScene, GltfPrimitive
for name in ["GltfDocument", "GltfAsset", "GltfMesh", "GltfNode", "GltfScene", "GltfPrimitive", "GltfAccessor", "GltfBufferView", "GltfBuffer"]:
    # look for #[derive(...Default...)] before struct
    m = re.search(rf"#\[derive\([^\]]*Default[^\]]*\)\]\s*(?:pub\s+)?struct {name}\b", snap)
    print(name, "Default" if m else "NO_DEFAULT")

# How is schema module wired?
gltf = next(os.path.join(arts, n) for n in os.listdir(arts) if "gltf" in n)
# find pub mod schema in root and standards
for rel in ["", ]:
    pass
# read root rs for mod declarations - already have lib
lib = open(os.path.join(gltf, next(f for f in os.listdir(gltf) if f.endswith(".rs"))), encoding="utf-8").read()
# find include of schema
for line in lib.splitlines():
    if "mod " in line or "pub use" in line[:40]:
        if any(x in line for x in ["schema", "engine", "io", "standards"]):
            print("LIBMOD", line)

# find standards/mod
for r, ds, fs in os.walk(gltf):
    depth = r.count(os.sep) - gltf.count(os.sep)
    if depth > 4:
        continue
    for f in fs:
        if f.endswith(".rs"):
            p = os.path.join(r, f)
            t = open(p, encoding="utf-8", errors="ignore").read(3000)
            if "pub mod schema" in t and ("pub mod io" in t or "pub mod engine" in t or "snapshot" in t):
                print("MODFILE", p.split("gltf/")[-1][:100])
                for line in t.splitlines()[:80]:
                    if line.startswith("pub mod") or line.startswith("mod ") or "pub use" in line:
                        print(" ", line[:120])

# LAS types
las = next(os.path.join(arts, n) for n in os.listdir(arts) if n.endswith("las") or "las" in n)
las_snap = None
for r, ds, fs in os.walk(las):
    for f in fs:
        if f.endswith(".rs") and "snapshot" in r:
            p = os.path.join(r, f)
            t = open(p, encoding="utf-8", errors="ignore").read()
            if "struct LasSnapshot" in t:
                las_snap = p
                print("LAS_SNAP", p.split("las/")[-1] if "las" in p else p)
                for name in ["LasSnapshot", "LasPoint", "LasHeader"]:
                    idx = t.find(f"struct {name}")
                    print("====", name)
                    print(t[idx:idx+600])
                break

# encode_las signature
for r, ds, fs in os.walk(las):
    for f in fs:
        if f.endswith(".rs"):
            p = os.path.join(r, f)
            t = open(p, encoding="utf-8", errors="ignore").read()
            for m in re.finditer(r"pub fn (encode_las|decode_las)[^\n]+", t):
                print("LAS_SIG", m.group(0), "||", p.split("las/")[-1][:80])

# DWG mesh_to_dwg
dwg = next(os.path.join(arts, n) for n in os.listdir(arts) if "dwg" in n)
for r, ds, fs in os.walk(dwg):
    for f in fs:
        if not f.endswith(".rs"):
            continue
        p = os.path.join(r, f)
        t = open(p, encoding="utf-8", errors="ignore").read()
        if "fn mesh_to_dwg_drawing" in t:
            idx = t.find("fn mesh_to_dwg_drawing")
            print("MESH_DWG", p.split("dwg/")[-1][:100])
            print(t[idx:idx+800])
        if "fn dwg_to_bytes" in t:
            idx = t.find("fn dwg_to_bytes")
            print("DWG_BYTES", t[idx:idx+400])
        if "fn dwg_from_bytes" in t:
            idx = t.find("fn dwg_from_bytes")
            print("DWG_FROM", t[idx:idx+400])
        if "fn dwg_drawing_to_mesh" in t:
            idx = t.find("fn dwg_drawing_to_mesh")
            print("DWG_MESH", t[idx:idx+500])
