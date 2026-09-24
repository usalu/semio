import os, re

arts = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"

def find_art(needle):
    return next(os.path.join(arts, n) for n in os.listdir(arts) if needle in n)

# GltfBuffer struct
snap = "/Users/ueli/Documents/semio/.tmp-ticket/wp-p2/links/stdio-gltf-snapshot.rs"
text = open(os.readlink(snap), encoding="utf-8").read()
for name in ["GltfBuffer ", "GltfAccessor ", "GltfPrimitive ", "GltfAsset "]:
    idx = text.find(f"struct {name.strip()}")
    print("====", name)
    print(text[idx:idx+900])
    print("HAS_DEFAULT", "Default" in text[max(0,idx-200):idx+50])

# las
las = find_art("las")
print("LAS", las)
for r, ds, fs in os.walk(las):
    for f in fs:
        if not f.endswith(".rs"):
            continue
        p = os.path.join(r, f)
        t = open(p, encoding="utf-8", errors="ignore").read()
        for needle in ["pub fn encode_las", "pub fn decode_las", "struct LasSnapshot", "struct LasPoint", "struct LasHeader", "pub mod engine", "pub mod io", "pub mod schema"]:
            if needle in t:
                print(needle, "->", p.split("artifacts/")[-1][:120])

# dwg
dwg = find_art("dwg")
print("DWG", dwg)
for r, ds, fs in os.walk(dwg):
    for f in fs:
        if not f.endswith(".rs"):
            continue
        p = os.path.join(r, f)
        t = open(p, encoding="utf-8", errors="ignore").read()
        for needle in ["pub fn mesh_to_dwg", "pub fn dwg_to_bytes", "pub fn bytes_to_dwg", "fn encode", "fn decode", "struct DwgSnapshot", "pub mod engine", "pub mod io"]:
            if needle in t and ("fn mesh" in needle or "dwg_to" in needle or "bytes_to" in needle or "struct Dwg" in needle or "pub mod" in needle or "pub fn encode" in needle or "pub fn decode" in needle):
                # print matching lines
                for line in t.splitlines():
                    if any(x in line for x in ["mesh_to_dwg", "dwg_to_bytes", "bytes_to_dwg", "struct DwgSnapshot", "pub fn encode", "pub fn decode", "pub mod "]):
                        if "pub " in line or "struct" in line or "mesh_to" in line:
                            print("DWG_LINE", line.strip()[:140], "||", p.split("artifacts/")[-1][:100])
                            break
