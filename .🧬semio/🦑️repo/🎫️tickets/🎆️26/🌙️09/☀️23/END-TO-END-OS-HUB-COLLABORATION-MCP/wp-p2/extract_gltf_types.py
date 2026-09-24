import os, re

root = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌊️gltf"
# find actual gltf dir
arts = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
root = next(os.path.join(arts, n) for n in os.listdir(arts) if "gltf" in n)
print("root", root)

for r, ds, fs in os.walk(root):
    for f in fs:
        if not f.endswith(".rs"):
            continue
        p = os.path.join(r, f)
        t = open(p, encoding="utf-8", errors="ignore").read()
        if "enum GltfComponentType" in t:
            idx = t.find("enum GltfComponentType")
            print("COMP", p)
            print(t[idx : idx + 500])
        if "enum GltfAccessorType" in t:
            idx = t.find("enum GltfAccessorType")
            print("ACC", p)
            print(t[idx : idx + 400])

# crate lib full reexports
lib = os.path.join(root, next(f for f in os.listdir(root) if f.endswith(".rs") and "lib" not in f.lower() or True))
# find root rs
for f in os.listdir(root):
    if f.endswith(".rs"):
        print("ROOT_RS", f)
        print(open(os.path.join(root, f), encoding="utf-8").read()[:4000])

# metabolism example that builds gltf
for r, ds, fs in os.walk(root):
    for f in fs:
        if f.endswith(".rs") and "metabolism" in r and "example" in r:
            p = os.path.join(r, f)
            t = open(p, encoding="utf-8").read()
            if "encode_glb" in t and "GltfAccessor" in t:
                print("EXAMPLE", p)
                # print imports and a chunk that builds accessors
                print(t[:2000])
                idx = t.find("GltfAccessor")
                print(t[idx : idx + 1500])
