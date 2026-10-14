#!/usr/bin/env python3
"""📋️ Registers every crate a P8 cut created in the repository build owner registry (`📋️build-owners.json`):
name = package, root = package dir, manifest = its Cargo.toml, no extra packages. Existing rows are kept.

usage: 📋️build-owners.py [apply]
"""
import glob, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
path = os.path.join(REPO, "📋️build-owners.json")
data = json.load(open(path, encoding="utf-8"))
names = {o["name"] for o in data["owners"]}
roots = {o["root"] for o in data["owners"]}
added = []
for cfg_path in sorted(glob.glob(os.path.join(HERE, "*", "*-cut.json"))):
    config = json.load(open(cfg_path, encoding="utf-8"))
    for spec in config["crates"].values():
        root = os.path.relpath(os.path.join(REPO, config["artifact"], spec["dir"]), REPO)
        if not os.path.exists(os.path.join(REPO, root, "Cargo.toml")) or spec["package"] in names or root in roots:
            continue
        row = {"name": spec["package"], "root": root, "manifest": root + "/Cargo.toml", "packages": []}
        best = max(range(len(data["owners"])), key=lambda i: (len(os.path.commonprefix([data["owners"][i]["name"], spec["package"]])), i))
        data["owners"].insert(best + 1, row)
        names.add(spec["package"]); roots.add(root); added.append(spec["package"])
print(len(added), "added:", added)
if len(sys.argv) > 1 and sys.argv[1] == "apply":
    open(path, "w", encoding="utf-8").write(json.dumps(data, ensure_ascii=False, indent=2) + "\n")
