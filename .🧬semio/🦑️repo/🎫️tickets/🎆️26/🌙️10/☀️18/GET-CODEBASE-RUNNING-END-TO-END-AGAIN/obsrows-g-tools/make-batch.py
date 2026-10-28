#!/usr/bin/env python3
"""[obsrows-g] builds staged/batch.json (anchored edits) from `staged/<build-relative>` working copies against the live library files, plus the new files of the batch.
usage: make-batch.py            (re-runnable: reads the live tree, so run it before handing the batch over and again after the live files moved)
A staged file that is NOT in the live tree is a new file (tar). Hand-written alternatives (`MANUAL`) cover files another staged batch also rewrites."""
import difflib, hashlib, json, os, sys, tarfile
HERE = os.path.dirname(os.path.abspath(__file__))
BUILD = ""
ROOT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
sha = lambda s: hashlib.sha256(s.encode("utf8")).hexdigest()
MANUAL = json.load(open(HERE + "/manual-edits.json", encoding="utf8")) if os.path.exists(HERE + "/manual-edits.json") else {}

def hunks(a, b, ctx=2):
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    ops = [o for o in sm.get_opcodes() if o[0] != "equal"]
    groups = []
    for o in ops:
        if groups and o[1] - groups[-1][-1][2] <= 2 * ctx: groups[-1].append(o)
        else: groups.append([o])
    out = []
    for g in groups:
        i1, i2, j1, j2 = g[0][1], g[-1][2], g[0][3], g[-1][4]
        for c in range(ctx, 40):
            lo, hi = max(0, i1 - c), min(len(a), i2 + c)
            old = "".join(a[lo:hi])
            if "".join(a).count(old) == 1:
                new = "".join(a[lo:i1]) + "".join(b[j1:j2]) + "".join(a[i2:hi])
                out.append({"old": old, "new": new, "span": (lo, hi)})
                break
        else: sys.exit("no unique anchor for hunk at line %d" % i1)
    for x, y in zip(out, out[1:]):
        if x["span"][1] > y["span"][0]: sys.exit("overlapping anchors near line %d" % x["span"][1])
    return [{"old": h["old"], "new": h["new"]} for h in out]

files, new_files = [], []
for dirpath, _, names in os.walk(HERE + "/staged"):
    for name in sorted(names):
        full = os.path.join(dirpath, name); rel = os.path.relpath(full, HERE + "/staged")
        live = ROOT + BUILD + rel
        mine = open(full, encoding="utf8").read()
        if rel in MANUAL:
            files.append({"file": BUILD + rel, "edits": MANUAL[rel]}); continue
        if not os.path.exists(live): new_files.append(BUILD + rel); continue
        base = open(live, encoding="utf8").read()
        if base == mine: continue
        files.append({"file": BUILD + rel, "baseSha256": sha(base), "afterSha256": sha(mine), "edits": [{"variants": [h]} for h in hunks(base.splitlines(True), mine.splitlines(True))]})
for rel, edits in MANUAL.items():
    if not any(f["file"] == BUILD + rel for f in files): files.append({"file": BUILD + rel, "edits": edits})
with tarfile.open(HERE + "/batch/new-files.tgz", "w:gz") as tar:
    for n in new_files: tar.add(ROOT + BUILD + n if False else HERE + "/staged/" + n[len(BUILD):], arcname=n)
json.dump({"version": 1, "owner": "obsrows-g", "files": files}, open(HERE + "/batch/batch.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
json.dump(new_files, open(HERE + "/batch/new-files.json", "w", encoding="utf8"), ensure_ascii=False)
print("files with edits:", len(files), "new files:", len(new_files))
