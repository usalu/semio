"""📦 [nxdeadline] builds a flat staged batch: usage mkbatch.py <batch> <new-root> <rel>=<base> ...  where <base> is `live` (the file in the live checkout now), `none` (new file) or a path holding the base bytes (e.g. a peer lane's staged `files/fNNN`) -> C/nxdeadline-tools/staged/<batch>/{manifest.json,files/,base/,stage.py}; an existing verify.sh is kept."""
import hashlib, json, os, shutil, sys
T = os.path.dirname(os.path.abspath(__file__))
LIVE = T
while not os.path.isdir(os.path.join(LIVE, "🧰️framework")): LIVE = os.path.dirname(LIVE)
batch, new_root, specs = sys.argv[1], sys.argv[2], sys.argv[3:]
out = os.path.join(T, "staged", batch)
keep = open(out + "/verify.sh", encoding="utf8").read() if os.path.exists(out + "/verify.sh") else None
if os.path.exists(out): shutil.rmtree(out)
os.makedirs(out + "/files"); os.makedirs(out + "/base")
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest() if p and os.path.exists(p) else None
manifest = {}
for i, spec in enumerate(specs):
    rel, base = spec.rsplit("=", 1)
    name, n = f"f{i:03d}", os.path.join(new_root, rel)
    b = os.path.join(LIVE, rel) if base == "live" else None if base == "none" else base
    if not os.path.exists(n): sys.exit("missing new " + rel)
    if base != "none" and not os.path.exists(b): sys.exit("missing base " + rel)
    shutil.copyfile(n, f"{out}/files/{name}")
    if b: shutil.copyfile(b, f"{out}/base/{name}")
    manifest[rel] = {"file": name, "base": sha(b), "new": sha(n)}
    if manifest[rel]["base"] == manifest[rel]["new"]: sys.exit("unchanged " + rel)
json.dump(manifest, open(out + "/manifest.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
shutil.copyfile(os.path.join(T, "stage.template.py"), out + "/stage.py")
if keep is not None: open(out + "/verify.sh", "w", encoding="utf8").write(keep)
print(batch, len(manifest), "files")
