#!/usr/bin/env python3
"""📦️ usage: python3 -I stage.py <check|apply|revert> <root> -> lands (or removes) the staged dashplan batch (slot order: a slot holds the plan record of its last selection only).
staged/manifest.json maps every repository path to {file, base sha256, new sha256}; staged/files/<file> is the new version, staged/base/<file> the bytes the edit started from.
check: per file DONE (already new) / APPLY (still the base) / MERGE-CLEAN / MERGE-CONFLICT (a peer edited the file since staging: `git merge-file` on temporary copies). apply: refuses everything on one conflict, otherwise writes all atomically per file. revert: the mirror image (new -> base, three-way when a peer edited on top). Exit 0 ok, 1 refused."""
import hashlib, json, os, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
S = HERE
mode, root = sys.argv[1], sys.argv[2]
manifest = json.load(open(S + "/manifest.json", encoding="utf8"))


def sha(data):
    return hashlib.sha256(data).hexdigest()


def merge(live, ours, theirs):
    with tempfile.TemporaryDirectory() as directory:
        paths = []
        for name, data in (("live", live), ("from", ours), ("to", theirs)):
            paths.append(directory + "/" + name)
            open(paths[-1], "wb").write(data)
        run = subprocess.run(["git", "merge-file", "-p", "-L", "live", "-L", "from", "-L", "to"] + paths, capture_output=True)
        return run.returncode, run.stdout


rows = []
for rel, row in sorted(manifest.items()):
    path = os.path.join(root, rel)
    base, new = open(S + "/base/" + row["file"], "rb").read(), open(S + "/files/" + row["file"], "rb").read()
    want, start, label = (new, base, "APPLY") if mode in ("check", "apply") else (base, new, "RESTORE")
    if not os.path.exists(path):
        rows.append((rel, "MISSING", None))
        continue
    current = open(path, "rb").read()
    if current == want:
        rows.append((rel, "DONE", None))
    elif current == start:
        rows.append((rel, label, want))
    else:
        code, out = merge(current, start, want)
        rows.append((rel, "MERGE-CLEAN" if code == 0 else "MERGE-CONFLICT(%d)" % code, out))
bad = [r for r in rows if r[1] == "MISSING" or r[1].startswith("MERGE-CONFLICT")]
for rel, state, _ in rows:
    print("%-16s %s" % (state, rel))
if bad:
    print("REFUSED: no file touched")
    sys.exit(1)
if mode in ("apply", "revert"):
    for rel, state, data in rows:
        if state == "DONE":
            continue
        path = os.path.join(root, rel)
        open(path + ".dashplan.tmp", "wb").write(data)
        os.replace(path + ".dashplan.tmp", path)
    print("applied" if mode == "apply" else "reverted", len([r for r in rows if r[1] != "DONE"]), "files")
