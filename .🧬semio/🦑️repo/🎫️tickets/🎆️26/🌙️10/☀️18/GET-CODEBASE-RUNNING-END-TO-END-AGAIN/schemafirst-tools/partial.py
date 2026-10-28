#!/usr/bin/env python3
"""🧩️ Writes only some files of a staged batch into a MIRROR root (red-on-base proofs): python3 -I partial.py <batch> <mirror root> <substring>... (the root must not hold a .git: live refuses)."""
import json, os, sys
batch, root, *needles = sys.argv[1:]
if os.path.exists(os.path.join(root, ".git")): sys.exit("refusing a root with .git")
stage = os.path.join(os.path.dirname(os.path.abspath(__file__)), "staged", batch)
for rel, row in json.load(open(os.path.join(stage, "manifest.json"), encoding="utf8")).items():
    if any(needle in rel for needle in needles):
        target = os.path.join(root, rel)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "wb").write(open(os.path.join(stage, "files", row["file"]), "rb").read())
        print("wrote", rel[-70:])
