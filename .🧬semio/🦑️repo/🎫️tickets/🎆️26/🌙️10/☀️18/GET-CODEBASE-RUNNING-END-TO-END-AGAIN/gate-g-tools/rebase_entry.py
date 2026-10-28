"""🔀️ Rebases one whole-file entry of a flat staged batch onto the live file by a 3-way merge (ours = staged new, base = staged base, theirs = live) and re-records base/new hashes; keeps `<file>.pre-rebase` copies.
Usage: python3 -I rebase_entry.py <staged-dir> <repo-relative path suffix> [root]"""
import hashlib, json, os, shutil, subprocess, sys
stage, suffix = sys.argv[1], sys.argv[2]
root = sys.argv[3] if len(sys.argv) > 3 else "/Users/ueli/Documents/semio"
manifest = os.path.join(stage, "manifest.json")
rows = json.load(open(manifest, encoding="utf-8"))
rel = next(k for k in rows if k.endswith(suffix))
row = rows[rel]
new, base, live = os.path.join(stage, "files", row["file"]), os.path.join(stage, "base", row["file"]), os.path.join(root, rel)
for p in (new, base): shutil.copyfile(p, p + ".pre-rebase")
shutil.copyfile(manifest, manifest + ".pre-rebase")
merged = subprocess.run(["git", "merge-file", "-p", new, base, live], capture_output=True)
if merged.returncode != 0: sys.exit("CONFLICT %d in %s" % (merged.returncode, rel))
open(new, "wb").write(merged.stdout); shutil.copyfile(live, base)
digest = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
row["base"], row["new"] = digest(base), digest(new)
open(manifest, "w", encoding="utf-8").write(json.dumps(rows, ensure_ascii=False, indent=1))
print("rebased", rel[-60:])
