"""📦 [td-graph] builds a flat staged batch: usage mkbatch.py <batch> <base-root> <new-root> <rel>...  -> C/td-graph-tools/staged/<batch>/{manifest.json,files/,base/,stage.py}; base = the file at <base-root> (absent = new file), new = the file at <new-root>."""
import hashlib, json, os, shutil, sys
T = os.path.dirname(os.path.abspath(__file__))
batch, base_root, new_root, rels = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4:]
out = os.path.join(T, "staged", batch)
keep = open(out + "/verify.sh", encoding="utf8").read() if os.path.exists(out + "/verify.sh") else None
if os.path.exists(out): shutil.rmtree(out)
os.makedirs(out + "/files"); os.makedirs(out + "/base")
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest() if os.path.exists(p) else None
manifest = {}
for i, rel in enumerate(rels):
    name = f"f{i:03d}"; b, n = os.path.join(base_root, rel), os.path.join(new_root, rel)
    if not os.path.exists(n): sys.exit("missing new " + rel)
    shutil.copyfile(n, f"{out}/files/{name}")
    if os.path.exists(b): shutil.copyfile(b, f"{out}/base/{name}")
    manifest[rel] = {"file": name, "base": sha(b), "new": sha(n)}
    if manifest[rel]["base"] == manifest[rel]["new"]: sys.exit("unchanged " + rel)
json.dump(manifest, open(out + "/manifest.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
shutil.copyfile(os.path.join(T, "stage.template.py"), out + "/stage.py")
if keep is not None: open(out + "/verify.sh", "w", encoding="utf8").write(keep)
print(batch, len(manifest), "files")
