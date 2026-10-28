#!/usr/bin/env python3
"""rebase-g: compare a bases.json (path -> sha256, or null for absent) with the live tree. Usage: drift.py bases.json [root]"""
import hashlib, json, os, sys
bases = json.load(open(sys.argv[1], encoding="utf-8")); root = sys.argv[2] if len(sys.argv) > 2 else os.getcwd()
bad = 0
for path, want in bases.items():
    full = os.path.join(root, path)
    have = hashlib.sha256(open(full, "rb").read()).hexdigest() if os.path.exists(full) else None
    if have != want:
        bad += 1; print("CHANGED", path.split("/")[-2] + "/" + path.split("/")[-1], want and want[:10], "->", have and have[:10])
print("drift:", bad, "of", len(bases))
