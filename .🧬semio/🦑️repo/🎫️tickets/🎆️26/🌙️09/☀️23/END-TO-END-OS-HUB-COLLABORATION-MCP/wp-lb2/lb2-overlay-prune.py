#!/usr/bin/env python3
"""🧹️ LB2 overlay prune: removes every overlay file (and emptied directory) whose path no longer exists in the live tree, so a
refreshed overlay (`lb2-overlay.py` + `lb2-overlay-missing.py`) carries no file a peer deleted or renamed since the clone and no
file a prepared patch added (patches are re-applied after a sync). Usage: lb2-overlay-prune.py <overlay-root> [--dry-run]"""
import os
import sys

REPO = "/Users/ueli/Documents/semio"
target = sys.argv[1]
dry = "--dry-run" in sys.argv
removed = []
for root, dirs, names in os.walk(target, topdown=False):
    rel_root = os.path.relpath(root, target)
    for name in names:
        rel = os.path.normpath(os.path.join(rel_root, name))
        if not os.path.lexists(os.path.join(REPO, rel)):
            removed.append(rel)
            if not dry:
                os.remove(os.path.join(root, name))
    if not dry and root != target and not os.listdir(root):
        os.rmdir(root)
for rel in removed:
    print(rel)
print(f"lb2-overlay-prune: {len(removed)} files {'would be ' if dry else ''}removed from {target}")
