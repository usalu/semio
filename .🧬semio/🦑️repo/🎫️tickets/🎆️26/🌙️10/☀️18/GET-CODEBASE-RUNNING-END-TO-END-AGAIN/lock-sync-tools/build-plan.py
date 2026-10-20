#!/usr/bin/env python3
"""🏗️ Builds staged/plan.json: the merged round-5 manifest batch (anchored edits + new files).
Sources: bzl-default plan (groups manifest, manifest2, closure, nx; rebased), parsers workspace row in all 6 roots (new, closes a gap of manifest2 which only edited the root).
Reads the live tree (read only). Usage: build-plan.py [--root DIR]"""
import json, os, sys, shutil
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
ROOT = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else os.path.abspath(os.path.join(C, *[".."] * 7))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
T0 = L + "/⚡️caching/🚀️bootstrap/🛠️tools"
PARSERS = L + "/🏗️build/🚀️bootstrap/📦️parsers"
ROOTS = [".", "♻️mit-bestand", "✏️s", "🌎️hub", "🎓️teaching", "🏢️semio-tech"]
src = json.load(open(os.path.join(C, "bzl-default-tools", "staged", "plan.json"), encoding="utf-8"))
read = lambda rel: open(os.path.join(ROOT, rel), encoding="utf-8", newline="").read()
files, order = {}, []
def add(path, edit):
    if path not in files: files[path] = []; order.append(path)
    files[path].append(edit)
REBASED = {("%s/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs" % L, "closure-2"): [2]}
for item in src["files"]:
    for e in item["edits"]:
        g = e["group"]
        keep = g in ("closure", "nx") or (g == "manifest" and item["path"].endswith("package.json")) or g == "manifest2"
        if not keep: continue
        counts = REBASED.get((item["path"], e["id"]), e["counts"])
        group = {"manifest": "manifests", "manifest2": "manifests"}.get(g, g)
        add(item["path"], {"id": "%s-%s" % (g, e["id"]), "group": group, "old": e["old"], "new": e["new"], "counts": counts, "source": "bzl-default " + g})
for r in ROOTS[1:]:
    t = read(r + "/package.json")
    row = '    "../%s",\n' % T0
    if t.count(row) != 1: sys.exit("anchor missing in " + r)
    new = row + '    "../%s",\n' % PARSERS
    add(r + "/package.json", {"id": "parsers-member", "group": "manifests", "old": row, "new": new, "counts": [1], "source": "lock-sync: parsers workspace row (bzl-default manifest2 only edited the root)"})
newfiles = [{"group": "manifests" if n["group"] == "manifest2" else n["group"], "path": n["path"]} for n in src["newFiles"] if n["group"] in ("closure", "manifest2")]
stage = os.path.join(HERE, "staged", "new")
shutil.rmtree(stage, ignore_errors=True)
for n in newfiles:
    srcp = os.path.join(C, "bzl-default-tools", "staged", "new", n["path"])
    dst = os.path.join(stage, n["path"]); os.makedirs(os.path.dirname(dst), exist_ok=True); shutil.copyfile(srcp, dst)
    if len(n["path"]) > 240: sys.exit("path too long " + n["path"])
plan = {"version": 1, "files": [{"path": p, "edits": files[p]} for p in order], "newFiles": newfiles}
json.dump(plan, open(os.path.join(HERE, "staged", "plan.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
tally = {}
for p in order:
    for e in files[p]: tally[e["group"]] = tally.get(e["group"], 0) + 1
print(json.dumps({"files": len(order), "edits": sum(tally.values()), "byGroup": tally, "newFiles": len(newfiles)}, ensure_ascii=False))
