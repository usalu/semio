#!/usr/bin/env python3
"""rebase-g: rewrites the CHANGED entries of a bases.json to the live sha256 (only after the batch's anchors were re-checked against the live files). Usage: rebase-bases.py bases.json [root]"""
import hashlib, json, os, sys
path = sys.argv[1]; root = sys.argv[2] if len(sys.argv) > 2 else os.getcwd()
bases = json.load(open(path, encoding="utf-8")); moved = 0
for rel, want in list(bases.items()):
    full = os.path.join(root, rel)
    if os.path.exists(full):
        have = hashlib.sha256(open(full, "rb").read()).hexdigest()
        if have != want: bases[rel] = have; moved += 1; print("rebased", rel.split("/")[-2] + "/" + rel.split("/")[-1], want[:10], "->", have[:10])
json.dump(bases, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1); open(path, "a").write("\n")
print("rebased:", moved, "of", len(bases))
