#!/usr/bin/env python3
"""🪞️ Completes the P9 overlay with every file of the live tree it lacks — the gitignored build inputs `p9-overlay.py` does
not list (generated sources such as `🎨️styling/🔤️tokens/🦀️.rs`) — by APFS clonefile, pruning build outputs and caches
(`target`, `node_modules`, `dist`, `pkg`, `.git`, `.🧬semio`, ticket `🗑️generated`, the `♻️mit-bestand` research archive). Never overwrites an overlay file.
Usage: p9-overlay-ignored.py <overlay-root> [--dry-run]"""
import ctypes
import os
import sys

REPO = "/Users/ueli/Documents/semio"
PRUNE = {"target", "node_modules", "dist", "pkg", ".git", ".🧬semio", "🗑️generated", ".venv", "__pycache__", ".nx", ".tmp-ticket", ".pytest_cache", "♻️mit-bestand"}
libc = ctypes.CDLL("libc.dylib", use_errno=True)
libc.clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]

target, dry = sys.argv[1], "--dry-run" in sys.argv
added = []
for root, dirs, names in os.walk(REPO):
    dirs[:] = [d for d in dirs if d not in PRUNE and not d.startswith(".t14-")]
    for name in names:
        source = os.path.join(root, name)
        rel = os.path.relpath(source, REPO)
        destination = os.path.join(target, rel)
        if os.path.lexists(destination) or not os.path.isfile(source) or os.path.islink(source):
            continue
        added.append(rel)
        if not dry:
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            if libc.clonefile(source.encode(), destination.encode(), 0) != 0:
                raise OSError(ctypes.get_errno(), f"clonefile {rel}")
for rel in added:
    print(("would add " if dry else "added ") + rel)
print(f"p9-overlay-ignored: {len(added)} file(s) {'missing' if dry else 'cloned'} into {target}")
