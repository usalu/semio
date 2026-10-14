#!/usr/bin/env python3
"""📋️ Admits every Cargo package of a cut config to `📋️build-owners.json` as a manifest-owned build owner (name, root,
`Cargo.toml` manifest, no npm packages), inserted after the last smaller name inside the registry's contiguous `semio-` block, and
drops rows whose manifest no longer exists under a cut crate's root.

usage: 📋️build-owners-rows.py <cut.json> plan|apply
"""
import json, os, re, sys

REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
config, mode = json.load(open(sys.argv[1], encoding="utf-8")), sys.argv[2]
path = os.path.join(REPO, "📋️build-owners.json")
registry = json.load(open(path, encoding="utf-8"))
owners = registry["owners"]
start = next(i for i, o in enumerate(owners) if o["name"].startswith("semio-"))
end = start
while end < len(owners) and owners[end]["name"].startswith("semio-"):
    end += 1
block = owners[start:end]
known = {o["name"] for o in owners}
added = []
for row in config["crates"]:
    root = os.path.relpath(os.path.join(os.path.dirname(os.path.join(REPO, row["root"])), "📦️packages", "🦀️rust"), REPO)
    manifest = os.path.join(root, "Cargo.toml")
    if not os.path.exists(os.path.join(REPO, manifest)):
        raise SystemExit("missing " + manifest)
    if row["package"] in known or any(o["root"] == root for o in owners):
        continue
    entry = {"name": row["package"], "root": root, "manifest": manifest, "packages": []}
    at = max((i + 1 for i, o in enumerate(block) if o["name"] < row["package"]), default=0)
    block.insert(at, entry)
    added.append(row["package"])
registry["owners"] = owners[:start] + block + owners[end:]
print(f"+{len(added)}: {' '.join(added)}")
if mode == "apply":
    open(path, "w", encoding="utf-8").write(json.dumps(registry, ensure_ascii=False, indent=2) + "\n")
