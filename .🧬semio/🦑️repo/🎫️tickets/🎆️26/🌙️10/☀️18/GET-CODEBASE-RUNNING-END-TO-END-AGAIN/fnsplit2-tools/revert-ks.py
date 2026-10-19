"""↩️ Reverts a ksplit/psplit run: restores the snapshot over the root file and removes the child folders named in the spec. usage: revert-ks.py <spec.json> <snapshot>"""
import json, os, shutil, sys
spec = json.load(open(sys.argv[1], encoding="utf8"))
root = spec["file"]
base = spec.get("childRoot") or os.path.dirname(root)
dirs = []
if "kit" in spec: dirs.append(spec["kit"]["dir"])
for g in spec.get("groups", []):
    dirs.append(g["dir"])
    if "helpers" in g: dirs.append(g["helpers"]["dir"])
    for p in g.get("parts", []): dirs.append(p["dir"])
for f in spec.get("files", []): dirs.append(f["dir"])
for p in spec.get("parts", []): dirs.append(p["dir"])
if spec.get("shared"): dirs.append(spec["shared"]["dir"])
if spec.get("helpers"): dirs.append(spec["helpers"]["dir"])
shutil.copyfile(sys.argv[2], root)
for d in dirs: shutil.rmtree(os.path.join(base, d), ignore_errors=True)
print("reverted", len(dirs), "folders")
