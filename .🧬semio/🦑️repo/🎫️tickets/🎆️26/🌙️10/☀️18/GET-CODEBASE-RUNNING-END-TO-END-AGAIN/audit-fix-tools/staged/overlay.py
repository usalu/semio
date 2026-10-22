#!/usr/bin/env python3
"""Builds /private/tmp/audit-fix/overlay: a symlink mirror of the repository whose project-assembly directory holds a real, replaceable copy of project-assembly. Dry by default; --write builds."""
import os, shutil, sys
REPO = os.environ.get("AUDIT_FIX_REPO", "/Users/ueli/Documents/semio")
OVERLAY = os.environ.get("AUDIT_FIX_OVERLAY", "/private/tmp/audit-fix/overlay")
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
INF = LIB + "/🏗️build/🕸️graph/📥️inference"
REAL_FILES = [INF + "/🧱️project-assembly/🟨️.mjs"]
write = "--write" in sys.argv
if not os.path.isdir(REPO) or not REAL_FILES or not all(os.path.isfile(os.path.join(REPO, f)) for f in REAL_FILES): sys.exit("refused: repository or source files missing")
if os.path.exists(OVERLAY) and write: shutil.rmtree(OVERLAY)
chain = [INF + "/🧱️project-assembly"]
real_dirs = set()
for f in REAL_FILES:
    parts = f.split("/")[:-1]
    for i in range(0, len(parts) + 1): real_dirs.add("/".join(parts[:i]))
def mirror(rel):
    src, dst = os.path.join(REPO, rel), os.path.join(OVERLAY, rel)
    if write: os.makedirs(dst, exist_ok=True)
    for name in sorted(os.listdir(src)) if rel else sorted(os.listdir(REPO)):
        child = (rel + "/" + name) if rel else name
        s, d = os.path.join(REPO, child), os.path.join(OVERLAY, child)
        if child in real_dirs: mirror(child)
        elif child in REAL_FILES:
            if write: shutil.copyfile(s, d)
        elif write: os.symlink(s, d)
mirror("")
print(("built " if write else "dry-run ") + OVERLAY, "real files:", len(REAL_FILES))
