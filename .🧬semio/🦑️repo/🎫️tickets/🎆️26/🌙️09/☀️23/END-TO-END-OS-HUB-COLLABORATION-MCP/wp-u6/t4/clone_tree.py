#!/usr/bin/env python3
"""🧬️ Clones the live repo into a private scratch tree with APFS `clonefile` (no data copied), skipping VCS, hub, build and
package directories. Usage: clone_tree.py <destination>"""
import ctypes
import os
import sys

ROOT = "/Users/ueli/Documents/semio"
SKIP = {".git", ".🧬semio", "node_modules", "dist", "target", ".nx", "🗑️generated", "__pycache__", ".pytest_cache", ".tmp-ticket"}
libc = ctypes.CDLL("libc.dylib", use_errno=True)
destination = sys.argv[1]
files = 0
for directory, names, filenames in os.walk(ROOT):
    names[:] = [name for name in names if name not in SKIP and not os.path.islink(os.path.join(directory, name))]
    target = os.path.join(destination, os.path.relpath(directory, ROOT))
    os.makedirs(target, exist_ok=True)
    for name in filenames:
        source = os.path.join(directory, name)
        if os.path.islink(source):
            os.symlink(os.readlink(source), os.path.join(target, name))
        elif libc.clonefile(source.encode(), os.path.join(target, name).encode(), 0) != 0:
            raise OSError(ctypes.get_errno(), f"clonefile {source}")
        files += 1
print(f"cloned {files} files into {destination}")
