#!/usr/bin/env python3
from uregen import *
d = load("📋️surface-fixture.json")
dirp = os.path.dirname(O + "/" + d["root"]) + "/📦️packages/🦀️rust"
out = subprocess.run(["bun", ".tfr-deps.ts", dirp + "/Cargo.toml"], cwd=O, capture_output=True, text=True).stdout
deps = json.loads(out)
old = {r["name"]: r for r in d["dependencies"]}
new = []
for name, spec in sorted(deps.items()):
    if name in ("serde", "serde_json"): continue
    tgt = os.path.normpath(os.path.join(dirp, spec["path"], "Cargo.toml"))
    new.append({"name": name, "path": os.path.relpath(tgt, O), "features": spec.get("features", [])})
print("deps", len(old), "->", len(new), "gained", sorted({r["name"] for r in new} - set(old)), "lost", sorted(set(old) - {r["name"] for r in new}))
for r in new:
    if r["name"] in old and (old[r["name"]]["path"] != r["path"] or old[r["name"]]["features"] != r["features"]): print("CHANGED", r["name"])
d["dependencies"] = new
save("📋️surface-fixture.json", d)
