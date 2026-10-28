#!/usr/bin/env python3
"""♻️ Copies the files a staged batch touches from the live tree over a mirror (so the batch can be applied again from its base). usage: python3 -I reset-mirror.py <mirror root> <batch dir>"""
import json, os, shutil, sys
mirror, batch = sys.argv[1], sys.argv[2]
live = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *([".."] * 8)))
manifest = json.load(open(os.path.join(batch, "manifest.json"), encoding="utf-8"))
files = {e["file"] for e in manifest["edits"]} | set(manifest.get("extra", []))
for rel in sorted(files):
    shutil.copyfile(os.path.join(live, rel), os.path.join(mirror, rel))
for item in manifest["files"]:
    path = os.path.join(mirror, item["path"])
    if os.path.exists(path): os.remove(path)
print("reset", len(files), "files")
