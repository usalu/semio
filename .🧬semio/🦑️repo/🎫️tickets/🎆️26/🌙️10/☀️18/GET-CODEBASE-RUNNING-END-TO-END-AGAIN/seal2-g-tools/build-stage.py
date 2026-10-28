#!/usr/bin/env python3
"""🏗️ Builds the staged batch `staged/seal2/` from the anchored edits (edits.py) computed against the LIVE tree and the new files under src/new/ (repo-relative paths).
usage: python3 -I build-stage.py [--root DIR]   (DIR = the tree the edits are computed against, default the live repository)"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import edits as E
root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.path.abspath(os.path.join(HERE, *([".."] * 8)))
out = os.path.join(HERE, "staged", "seal2")
shutil.rmtree(out, ignore_errors=True)
os.makedirs(os.path.join(out, "edits")); os.makedirs(os.path.join(out, "files"))
manifest = {"edits": [], "files": []}
bases = {}
sha = lambda data: hashlib.sha256(data).hexdigest()
for index, edit in enumerate(E.edits(root), 1):
    text = open(os.path.join(root, edit["file"]), encoding="utf-8").read()
    assert text.count(edit["old"]) == 1, (edit["name"], text.count(edit["old"]))
    for kind in ("old", "new"):
        open(os.path.join(out, "edits", "%02d-%s.%s" % (index, edit["name"], kind)), "w", encoding="utf-8", newline="").write(edit[kind])
    manifest["edits"].append({"file": edit["file"], "name": edit["name"], "old": "edits/%02d-%s.old" % (index, edit["name"]), "new": "edits/%02d-%s.new" % (index, edit["name"])})
    bases[edit["file"]] = sha(text.encode("utf-8"))
new_root = os.path.join(HERE, "src", "new")
for current, _, names in os.walk(new_root):
    for name in sorted(names):
        source = os.path.join(current, name); rel = os.path.relpath(source, new_root)
        target = os.path.join(out, "files", rel); os.makedirs(os.path.dirname(target), exist_ok=True); shutil.copyfile(source, target)
        manifest["files"].append({"path": rel, "source": "files/" + rel})
        bases[rel] = "absent"
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
json.dump(bases, open(os.path.join(out, "bases.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1, sort_keys=True)
shutil.copyfile(os.path.join(HERE, "apply.py"), os.path.join(out, "apply.py"))
open(os.path.join(out, "depth.txt"), "w").write("10")
for name, command in (("apply", "apply"), ("check", "check"), ("revert", "revert")):
    open(os.path.join(out, name + ".sh"), "w", encoding="utf-8").write('#!/bin/sh\n# usage: sh %s.sh [--root DIR]  (anchored edits + new files; see apply.py)\nexec python3 -I "$(dirname "$0")/apply.py" %s "$@"\n' % (name, command))
print("staged", out, len(manifest["edits"]), "edits", len(manifest["files"]), "new files")
