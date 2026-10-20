#!/usr/bin/env python3
# usage: apply.py [--dry-run] [--allow-drift]
# Applies the staged rcache set to the live repository: every edit replaces exactly one anchored region, new files are copied from staged/.
# All edits are computed in memory first and nothing is written unless every one applies. A target whose content differs from the hash recorded
# at staging time is a drift: refused unless --allow-drift is given and every anchor still matches exactly once. Re-running after a successful
# apply reports "already applied" and writes nothing.
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from engine import *

dry = "--dry-run" in sys.argv
allow = "--allow-drift" in sys.argv
base_file = os.path.join(TOOLS, "base-hashes.json")
base = json.load(open(base_file, encoding="utf8")) if os.path.exists(base_file) else {}
edits = load_edits()
plan, problems, notes = [], [], []

for relative in edited_files(edits):
    path = os.path.join(ROOT, relative)
    if not os.path.isfile(path):
        problems.append(f"missing target: {relative}")
        continue
    text = read(path)
    mine = [e for e in edits if e["file"] == relative]
    fresh = sha(text)
    try:
        updated = text
        for edit in mine:
            updated = apply_one(updated, edit)
    except ValueError as error:
        applied = all(("new" in e and (e["new"] == "" or e["new"] in text)) for e in mine)
        if applied and any(e["new"] for e in mine):
            notes.append(f"already applied: {relative}")
            continue
        problems.append(f"anchor: {error}")
        continue
    drift = relative in base and base[relative] != fresh
    if drift and not allow:
        problems.append(f"drift (content differs from staging time): {relative}")
        continue
    if drift:
        notes.append(f"drift accepted, anchors unique: {relative}")
    plan.append((path, relative, updated))

for relative in staged_files():
    target = os.path.join(ROOT, relative)
    content = read(os.path.join(STAGED, relative))
    if os.path.exists(target):
        if read(target) == content:
            notes.append(f"already present: {relative}")
            continue
        problems.append(f"new file exists with other content: {relative}")
        continue
    if len(os.path.join(ROOT, relative).encode("utf-16-le")) // 2 > 256:
        problems.append(f"path exceeds 256 UTF-16 units: {relative}")
        continue
    plan.append((target, relative, content))

for line in notes:
    print("note:", line)
if problems:
    for line in problems:
        print("REFUSED:", line)
    print(f"refused: {len(problems)} problem(s), nothing written")
    sys.exit(1)
if dry:
    for _path, relative, _text in plan:
        print("would write:", relative)
    print(f"dry run ok: {len(plan)} file(s)")
    sys.exit(0)
for path, relative, text in plan:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".rcache.tmp"
    with open(temporary, "w", encoding="utf8", newline="") as handle:
        handle.write(text)
    os.replace(temporary, path)
    print("wrote:", relative)
print(f"applied: {len(plan)} file(s)")
