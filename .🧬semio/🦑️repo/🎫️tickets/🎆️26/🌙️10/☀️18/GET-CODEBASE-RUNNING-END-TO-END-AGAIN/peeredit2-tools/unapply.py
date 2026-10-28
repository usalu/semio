#!/usr/bin/env python3
"""↩ Removes a landed change set from a root whose files peers edited after the landing: a file still holding the batch's new bytes gets its base bytes back, a new file is deleted, a drifted file gets the set's anchored edits reversed (new text → old text, in reverse order; each reversed anchor must be unique). Used to build the base mirror "live minus the batch" for failure-set comparisons; never run on live. usage: python3 -I unapply.py <set> <root>"""
import hashlib, json, os, runpy, sys
HERE = os.path.dirname(os.path.abspath(__file__))
name, root = sys.argv[1], os.path.abspath(sys.argv[2])
store, stage = os.path.join(HERE, name), os.path.join(HERE, "staged", name)
manifest = json.load(open(os.path.join(stage, "manifest.json"), encoding="utf-8"))
edits = []
for module in sorted(f for f in os.listdir(store) if f.startswith("edits_") and f.endswith(".py")): edits += runpy.run_path(os.path.join(store, module))["EDITS"]
bad = 0
for rel, row in manifest.items():
    path = os.path.join(root, rel); data = open(path, "rb").read() if os.path.exists(path) else None
    if data is not None and hashlib.sha256(data).hexdigest() == row["new"]:
        if row["base"] is None: os.remove(path)
        else: open(path, "wb").write(open(os.path.join(stage, "base", row["file"]), "rb").read())
        continue
    mine = [e for e in edits if e[0] == rel]
    if data is None or row["base"] is None or not mine: print("CANNOT UNAPPLY", rel[-70:]); bad += 1; continue
    text = data.decode("utf-8")
    for _rel, old, new, *mode in reversed(mine):
        count = text.count(new)
        if mode == ["all"] and count: text = text.replace(new, old); continue
        if count != 1: print("REVERSE ANCHOR %d× %s :: %s" % (count, rel[-50:], new[:70].replace("\n", "⏎"))); bad += 1; break
        text = text.replace(new, old, 1)
    open(path, "w", encoding="utf-8", newline="").write(text)
    print("reversed", rel[-60:])
sys.exit(2 if bad else 0)
