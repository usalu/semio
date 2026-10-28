#!/usr/bin/env python3
"""🪞 usage: overlay.py [<overlay-dir>]  -> mirrors the repository with symlinks, except that every file under staged/files is a REAL copy (its ancestor directories are real, filled with symlinks to the real siblings), so graph-free tests of staged edits run without touching the tree. Prints the overlay path."""
import json, os, sys, shutil
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
STAGED = C + "/jobsrv-g-tools/staged/files"
OV = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/jg-ov"
with open(STAGED + "/manifest.json", encoding="utf8") as handle: KEY = {rel: e["file"] for rel, e in json.load(handle).items()}
files = list(KEY)
EXTRA = [line.strip() for line in open(C + "/jobsrv-g-tools/staged/overlay-extra.txt", encoding="utf8") if line.strip()] if os.path.exists(C + "/jobsrv-g-tools/staged/overlay-extra.txt") else []
only = [part for part in os.environ.get("JG_ONLY", "").split(",") if part]
if only: files = [f for f in files if any(part in f for part in only)]
files += EXTRA
if os.path.lexists(OV): shutil.rmtree(OV)
real_dirs = {""}
for f in files:
    parts = f.split("/")
    for i in range(1, len(parts)): real_dirs.add("/".join(parts[:i]))
fileset = set(files)
def staged_children(rel):
    prefix = rel + "/" if rel else ""
    return {p[len(prefix):].split("/")[0] for p in files if p.startswith(prefix)}
def build(rel):
    src = os.path.join(ROOT, rel); dst = os.path.join(OV, rel)
    os.makedirs(dst, exist_ok=True)
    try: names = set(os.listdir(src)) if os.path.isdir(src) else set()
    except OSError: names = set()
    for name in names | staged_children(rel):
        r = (rel + "/" + name) if rel else name
        if r in real_dirs: build(r)
        elif r in fileset: pass
        else: os.symlink(os.path.join(ROOT, r), os.path.join(OV, r))
build("")
for f in files:
    dst = os.path.join(OV, f); os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.copy2(os.path.join(ROOT, f) if f in EXTRA else os.path.join(STAGED, KEY[f]), dst)
print(OV)
