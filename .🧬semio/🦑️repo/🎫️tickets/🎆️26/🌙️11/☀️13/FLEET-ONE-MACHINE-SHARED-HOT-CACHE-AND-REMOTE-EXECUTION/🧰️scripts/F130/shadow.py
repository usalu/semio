#!/usr/bin/env python3
"""F130: builds a shadow overlay of the live tree for the staged files (symlinks to live siblings, real copies of staged files) so the staged tests run without touching the live tree. Usage: shadow.py <shadow-root>. The shadow root must be outside the checkout."""
import os, shutil, sys
LIVE = "/Users/ueli/Documents/semio"
HERE = LIVE + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/F130"
STAGE = HERE + "/stage"
root = sys.argv[1]
if root.startswith(LIVE):
    sys.exit("shadow root must be outside the checkout")
shutil.rmtree(root, ignore_errors=True)
staged = [os.path.relpath(os.path.join(d, n), STAGE) for d, _s, f in os.walk(STAGE) for n in f]
def build(rel_dir, names):
    dst = os.path.join(root, rel_dir)
    os.makedirs(dst, exist_ok=True)
    live = os.path.join(LIVE, rel_dir)
    heads = {n.split("/")[0] for n in names}
    if os.path.isdir(live):
        for entry in os.listdir(live):
            if entry not in heads:
                os.symlink(os.path.join(live, entry), os.path.join(dst, entry))
    for head in sorted(heads):
        sub = [n.split("/", 1)[1] for n in names if n.split("/")[0] == head and "/" in n]
        if sub:
            build(os.path.join(rel_dir, head), sub)
        else:
            shutil.copy(os.path.join(STAGE, rel_dir, head), os.path.join(dst, head))
build("", staged)
print("shadow ready:", root, len(staged), "files")
