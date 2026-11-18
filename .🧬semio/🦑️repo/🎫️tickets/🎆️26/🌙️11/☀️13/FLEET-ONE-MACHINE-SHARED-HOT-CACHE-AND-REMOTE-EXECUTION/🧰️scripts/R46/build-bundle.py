#!/usr/bin/env python3
"""R46 bundle: unified diffs (live now -> tested overlay) of every file of the shared-worktree module that differs between the live tree and the overlay, new files in full, with the baseline and result sha256 of each. usage: build-bundle.py <overlay root>"""
import os, sys, json, hashlib, difflib
HERE = os.path.dirname(os.path.abspath(__file__))
LIVE = "/Users/ueli/Documents/semio"
SW = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def rd(p):
    with open(p, "rb") as h:
        return h.read()


overlay = os.path.abspath(sys.argv[1])
bundle = {"patches": {}, "baseline": {}, "result": {}, "full": {}, "new": {}}
for directory, names, files in os.walk(overlay + "/" + SW):
    names[:] = [n for n in names if n != "node_modules"]
    for name in sorted(files):
        path = os.path.join(directory, name)
        rel = os.path.relpath(path, overlay)
        live = LIVE + "/" + rel
        new = rd(path)
        if not os.path.exists(live):
            bundle["new"][rel] = new.decode("utf8")
            bundle["result"][rel] = sha(new)
            continue
        old = rd(live)
        if old == new:
            continue
        bundle["baseline"][rel], bundle["result"][rel] = sha(old), sha(new)
        bundle["patches"][rel] = "".join(difflib.unified_diff(old.decode("utf8").splitlines(True), new.decode("utf8").splitlines(True), "a/f", "b/f", n=3))
json.dump(bundle, open(HERE + "/R46-bundle.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(bundle["patches"]), "patches;", len(bundle["new"]), "new files;", sum(d.count("\n") for d in bundle["patches"].values()), "diff lines")
for rel, d in bundle["patches"].items():
    print("  ", rel[len(SW) + 1:], "+%d -%d" % (sum(1 for l in d.splitlines() if l.startswith("+") and not l.startswith("+++")), sum(1 for l in d.splitlines() if l.startswith("-") and not l.startswith("---"))))
for rel in bundle["new"]:
    print("   new", rel[len(SW) + 1:])
