import os, re

LNK = "/Users/ueli/Documents/semio/.tmp-ticket/wp-p2/links"
arts = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
snap = open(os.readlink(f"{LNK}/stdio-gltf-snapshot.rs"), encoding="utf-8").read()

for name in ["GltfAsset", "GltfMesh", "GltfNode", "GltfScene", "GltfPrimitive", "GltfBuffer", "GltfBufferView", "GltfAccessor"]:
    # find impl Default for Name
    has_impl = f"impl Default for {name}" in snap
    # find derive before struct - look back 500 chars
    idx = snap.find(f"struct {name} ")
    if idx < 0:
        idx = snap.find(f"struct {name}\n")
    if idx < 0:
        idx = snap.find(f"struct {name}{{")
    window = snap[max(0, idx - 400) : idx]
    has_derive = "Default" in window
    print(name, "impl" if has_impl else "-", "derive" if has_derive else "-", "window:", window.replace("\n", " ")[-120:])

# schema module - does it expose snapshot submodule?
gltf = next(os.path.join(arts, n) for n in os.listdir(arts) if "gltf" in n)
# find schema/rs at subset any
for r, ds, fs in os.walk(gltf):
    if "schema" in os.path.basename(r) and any(f.endswith(".rs") for f in fs):
        for f in fs:
            if f.endswith(".rs") and "snapshot" not in r and "diff" not in r and "mutation" not in r and "test" not in r and "inference" not in r:
                p = os.path.join(r, f)
                t = open(p, encoding="utf-8", errors="ignore").read(2000)
                if "pub mod snapshot" in t or "pub use" in t:
                    print("SCHEMA_MOD", p.split("gltf/")[-1][:100])
                    for line in t.splitlines()[:60]:
                        if "mod " in line or "pub use" in line:
                            print(" ", line[:140])

# DWG snapshot and public reexports
dwg = next(os.path.join(arts, n) for n in os.listdir(arts) if "dwg" in n)
root_rs = next(os.path.join(dwg, f) for f in os.listdir(dwg) if f.endswith(".rs"))
t = open(root_rs, encoding="utf-8").read()
print("DWG_ROOT_LEN", len(t))
for line in t.splitlines():
    if "pub use" in line or line.startswith("pub mod") or "mesh_to_dwg" in line or "DwgSnapshot" in line or "DwgDrawing" in line:
        print("DWG", line[:160])

# find DwgSnapshot fields
for r, ds, fs in os.walk(dwg):
    for f in fs:
        if f.endswith(".rs"):
            p = os.path.join(r, f)
            txt = open(p, encoding="utf-8", errors="ignore").read()
            if "pub struct DwgSnapshot" in txt or "struct DwgSnapshot" in txt:
                idx = txt.find("struct DwgSnapshot")
                print("DWG_SNAP", p.split("dwg/")[-1][:100])
                print(txt[idx:idx+700])
                break

# Is mesh_to_dwg pub?
for r, ds, fs in os.walk(dwg):
    for f in fs:
        if f.endswith(".rs"):
            p = os.path.join(r, f)
            txt = open(p, encoding="utf-8", errors="ignore").read()
            if "mesh_to_dwg_drawing" in txt:
                for line in txt.splitlines():
                    if "mesh_to_dwg_drawing" in line and ("fn " in line or "use " in line or "pub use" in line):
                        print("VIS", line.strip()[:160], "||", p.split("dwg/")[-1][:80])

# Las header default and how encode fills
las = next(os.path.join(arts, n) for n in os.listdir(arts) if "las" in n)
root_rs = next(os.path.join(las, f) for f in os.listdir(las) if f.endswith(".rs"))
t = open(root_rs, encoding="utf-8").read()
for line in t.splitlines():
    if "pub use" in line or line.startswith("pub mod") or "LasSnapshot" in line or "encode_las" in line:
        print("LAS", line[:160])

# LasHeader Default
for r, ds, fs in os.walk(las):
    for f in fs:
        if f.endswith(".rs") and "snapshot" in r:
            p = os.path.join(r, f)
            t = open(p, encoding="utf-8", errors="ignore").read()
            if "impl Default for LasHeader" in t:
                idx = t.find("impl Default for LasHeader")
                print(t[idx:idx+800])
            if "impl Default for LasPoint" in t:
                idx = t.find("impl Default for LasPoint")
                print(t[idx:idx+400])
