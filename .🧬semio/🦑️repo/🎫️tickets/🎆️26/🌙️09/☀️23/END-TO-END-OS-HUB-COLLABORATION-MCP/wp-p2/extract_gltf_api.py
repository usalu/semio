import os, re

LNK = "/Users/ueli/Documents/semio/.tmp-ticket/wp-p2/links"
path = os.readlink(f"{LNK}/stdio-gltf-snapshot.rs")
text = open(path, encoding="utf-8").read()
for name in [
    "GltfSnapshot",
    "GltfDocument",
    "GltfPrimitive",
    "GltfAccessor",
    "GltfBufferView",
    "GltfBuffer",
    "GltfMesh",
    "GltfNode",
    "GltfScene",
    "GltfAsset",
    "GltfSourceForm",
    "GltfComponentType",
    "GltfAccessorType",
]:
    idx = text.find(f"struct {name}")
    if idx < 0:
        idx = text.find(f"enum {name}")
    if idx >= 0:
        print("====", name)
        print(text[idx : idx + 700])
        print()
    else:
        print("MISSING", name)

io = os.readlink(f"{LNK}/stdio-gltf-io.rs")
text = open(io, encoding="utf-8").read()
for m in re.finditer(r"pub fn (encode_glb|decode_glb|parse_glb)[^\n]+", text):
    print("SIG", m.group(0))

lib = os.readlink(f"{LNK}/stdio-gltf-lib.rs")
print("LIB", open(lib, encoding="utf-8").read()[:3000])
