#!/usr/bin/env python3
"""🪞️ LB2 overlay top-up: walks the source trees (pruning build outputs) and APFS-clones every source-like file
(.rs .json .wit .toml .ts .md .sql .graphql .proto .semio) that is missing from the overlay — the gitignored generated
sources crates `include!`/`#[path]` outside `🤖️generated/` (e.g. `🎨️styling/🔤️tokens/🦀️.rs`). Usage: lb2-overlay-missing.py <overlay-root>"""
import ctypes
import os
import sys

REPO = "/Users/ueli/Documents/semio"
PRUNE = {"dist", "target", "node_modules", ".git", "🗑️generated", ".vite", ".turbo", ".nx", "__pycache__"}
EXTENSIONS = (".rs", ".json", ".wit", ".toml", ".ts", ".md", ".sql", ".graphql", ".proto", ".semio", ".wgsl", ".bin")
libc = ctypes.CDLL("libc.dylib", use_errno=True)
libc.clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]
target = sys.argv[1]
added = []
for top in ("🧰️framework", "✏️s", "🌎️hub"):
    for root, dirs, names in os.walk(os.path.join(REPO, top)):
        dirs[:] = [d for d in dirs if d not in PRUNE]
        for name in names:
            if not name.endswith(EXTENSIONS):
                continue
            source = os.path.join(root, name)
            rel = os.path.relpath(source, REPO)
            destination = os.path.join(target, rel)
            if os.path.lexists(destination) or os.path.islink(source):
                continue
            os.makedirs(os.path.dirname(destination), exist_ok=True)
            if libc.clonefile(source.encode(), destination.encode(), 0) != 0:
                raise OSError(ctypes.get_errno(), f"clonefile {rel}")
            added.append(rel)
for rel in added:
    print(rel)
print(f"lb2-overlay-missing: {len(added)} files added into {target}")
