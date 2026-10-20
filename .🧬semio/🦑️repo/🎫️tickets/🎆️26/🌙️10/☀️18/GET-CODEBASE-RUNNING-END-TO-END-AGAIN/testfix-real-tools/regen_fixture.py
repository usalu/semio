#!/usr/bin/env python3
from uregen import *
d = load("📋️surface-fixture.json")
paths = [f["path"] for f in d["files"]]
allp = paths + ([d["metadataTarget"]] if d["metadataTarget"] not in paths else [])
res = fnlist([{"id": str(i), "path": p, "mode": "file"} for i, p in enumerate(allp)])
oldfns = {(fn["name"], fn["target"]): fn for f in d["files"] for fn in f["functions"]}
newfns = {}
for i, f in enumerate(d["files"]):
    r = res[str(i)]
    fns = [{"name": x["name"], "sha256": x["sha256"], "assertions": x["assertions"], "law": oldfns.get((x["name"], f["path"]), {}).get("law", False), "target": f["path"]} for x in r["fns"]]
    if i == 0:
        m = res[str(len(paths))] if d["metadataTarget"] not in paths else res[str(paths.index(d["metadataTarget"]))]
        fns += [{"name": x["name"], "sha256": x["sha256"], "assertions": x["assertions"], "law": oldfns.get((x["name"], d["metadataTarget"]), {}).get("law", False), "target": d["metadataTarget"]} for x in m["fns"]]
        f["assertions"] = r["assertions"] + m["assertions"]
    else: f["assertions"] = r["assertions"]
    f["sha256"] = r["sha256"]; f["bytes"] = r["bytes"]
    f["functions"] = fns
tot = sum(len(f["functions"]) for f in d["files"])
laws = sum(1 for f in d["files"] for fn in f["functions"] if fn["law"])
gained = [k for f in d["files"] for fn in f["functions"] for k in [(fn["name"], fn["target"])] if k not in oldfns]
lost = [k for k in oldfns if k not in {(fn["name"], fn["target"]) for f in d["files"] for fn in f["functions"]}]
print("functions", tot, "laws", laws, "gained", len(gained), "lost", lost[:6])
dec = [(k[0], oldfns[k]["assertions"], fn["assertions"]) for f in d["files"] for fn in f["functions"] for k in [(fn["name"], fn["target"])] if k in oldfns and fn["assertions"] < oldfns[k]["assertions"]]
print("assertion decreases", dec[:5])
save("📋️surface-fixture.json", d)
