#!/usr/bin/env python3
"""🧰️ CD1 set generator: turns (base file, target file) pairs into ONE standalone, idempotent set script — every edited text file
becomes anchored `(old, new)` hunks (difflib context widened until each anchor is unique in the base), every new file is embedded
whole; the generator proves itself by applying the hunks to the bases and comparing with the targets byte for byte.
Usage: cd1-make-set.py <spec.json> <out.py> (writes <name>.payload.json beside the script)
spec = {"name", "doc", "backups", "edits": [[rel, base_abs, target_abs]], "creates": [[rel, target_abs]]}"""
import difflib
import json
import os
import sys

spec = json.load(open(sys.argv[1], encoding="utf-8"))


def hunks(old, new):
    a, b = old.splitlines(keepends=True), new.splitlines(keepends=True)
    for context in (3, 5, 8, 13, 21, 34, 55):
        pairs, ok = [], True
        for group in difflib.SequenceMatcher(None, a, b, autojunk=False).get_grouped_opcodes(context):
            i1, i2, j1, j2 = group[0][1], group[-1][2], group[0][3], group[-1][4]
            before, after = "".join(a[i1:i2]), "".join(b[j1:j2])
            if old.count(before) != 1:
                ok = False
                break
            pairs.append((before, after))
        if ok:
            return pairs
    raise SystemExit("no unique anchors")


def apply(text, pairs):
    for before, after in pairs:
        if text.count(before) != 1:
            return None
        text = text.replace(before, after)
    return text


edits = []
for rel, base, target in spec["edits"]:
    old, new = open(base, encoding="utf-8", newline="").read(), open(target, encoding="utf-8", newline="").read()
    pairs = hunks(old, new)
    assert apply(old, pairs) == new, rel
    edits.append((rel, pairs))
creates = [(rel, open(target, encoding="utf-8", newline="").read()) for rel, target in spec["creates"]]
body = f'''#!/usr/bin/env python3
"""{spec["doc"]}
Usage: {spec["name"]}.py [--dry-run (default) | --write | --revert] [--root <tree>] — backups `{spec["backups"]}`."""
import json, os, shutil, sys

PAYLOAD = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "{spec["name"]}.payload.json"), encoding="utf-8"))
EDITS = [(rel, [tuple(pair) for pair in pairs]) for rel, pairs in PAYLOAD["edits"]]
CREATES = [tuple(row) for row in PAYLOAD["creates"]]
BACKUPS = PAYLOAD["backups"]
ROOT = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
MODE = "--write" if "--write" in sys.argv else "--revert" if "--revert" in sys.argv else "--dry-run"


def read(rel):
    return open(os.path.join(ROOT, rel), encoding="utf-8", newline="").read()


def plan():
    out, faults = {{}}, []
    for rel, pairs in EDITS:
        text = read(rel)
        for before, after in pairs:
            if text.count(before) != 1:
                faults.append(f"{{rel}}: anchor matches {{text.count(before)}}× — {{before[:120]!r}}")
                break
            text = text.replace(before, after)
        out[rel] = text
    for rel, text in CREATES:
        if os.path.exists(os.path.join(ROOT, rel)):
            faults.append(f"{{rel}}: exists already")
        out[rel] = text
    return out, faults


if MODE == "--revert":
    for rel, _ in EDITS + CREATES:
        saved = os.path.join(BACKUPS, rel)
        target = os.path.join(ROOT, rel)
        if os.path.exists(saved):
            shutil.copy2(saved, target)
        elif os.path.exists(target) and rel in dict(CREATES):
            os.remove(target)
    print("reverted")
    sys.exit(0)
applied = all(dict(CREATES).get(rel) == read(rel) for rel, _ in CREATES if os.path.exists(os.path.join(ROOT, rel))) and CREATES and all(os.path.exists(os.path.join(ROOT, rel)) for rel, _ in CREATES) and all(all(read(rel).count(after) >= 1 for _, after in pairs) for rel, pairs in EDITS)
if applied:
    print("already applied")
    sys.exit(0)
out, faults = plan()
print(f"{{len(EDITS)}} edited + {{len(CREATES)}} created, faults {{len(faults)}}")
for fault in faults:
    print("FAULT", fault)
if faults:
    sys.exit(1)
if MODE == "--write":
    for rel, text in out.items():
        target = os.path.join(ROOT, rel)
        if os.path.exists(target):
            os.makedirs(os.path.dirname(os.path.join(BACKUPS, rel)), exist_ok=True)
            shutil.copy2(target, os.path.join(BACKUPS, rel))
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8", newline="").write(text)
    print("written")
'''
open(sys.argv[2], "w", encoding="utf-8").write(body)
json.dump({"edits": edits, "creates": creates, "backups": spec["backups"]}, open(os.path.join(os.path.dirname(sys.argv[2]), f"{spec['name']}.payload.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(f"{spec['name']}: {len(edits)} edits ({sum(len(p) for _, p in edits)} hunks) + {len(creates)} creates → {sys.argv[2]}")
