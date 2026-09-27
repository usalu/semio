#!/usr/bin/env python3
"""🪞️ P9 overlay: APFS-clones (clonefile(2), copy-on-write, no data copied) every git-tracked and untracked-unignored file
outside `.🧬semio/`, plus the gitignored `🤖️generated/` source dirs the crates `include!`, into the overlay root.
Re-running refreshes files whose size or mtime differ. Usage: p9-overlay.py <overlay-root>"""
import ctypes
import os
import subprocess
import sys

REPO = "/Users/ueli/Documents/semio"
libc = ctypes.CDLL("libc.dylib", use_errno=True)
libc.clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]


def listing(*args):
    out = subprocess.run(["git", *args], cwd=REPO, capture_output=True, check=True).stdout.decode()
    return [path for path in out.split("\0") if path]


def clone(rel, target):
    source = os.path.join(REPO, rel)
    if not os.path.lexists(source) or os.path.isdir(source) and not os.path.islink(source):
        return 0
    destination = os.path.join(target, rel)
    if os.path.lexists(destination):
        a, b = os.lstat(source), os.lstat(destination)
        if a.st_size == b.st_size and int(a.st_mtime) == int(b.st_mtime):
            return 0
        os.remove(destination)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    if os.path.islink(source):
        os.symlink(os.readlink(source), destination)
        return 1
    if libc.clonefile(source.encode(), destination.encode(), 0) != 0:
        raise OSError(ctypes.get_errno(), f"clonefile {rel}")
    return 1


def generated_dirs():
    found = []
    for top in ("🧰️framework", "✏️s"):
        for root, dirs, _ in os.walk(os.path.join(REPO, top)):
            dirs[:] = [d for d in dirs if d not in ("dist", "target", "node_modules", ".git")]
            if os.path.basename(root) == "🤖️generated":
                found.append(os.path.relpath(root, REPO))
                dirs[:] = []
    return found


target = sys.argv[1]
exclude = ":!:.🧬semio/**"
files = listing("ls-files", "-z", "--", exclude) + listing("ls-files", "-z", "--others", "--exclude-standard", "--", exclude)
copied = sum(clone(rel, target) for rel in files)
extra = 0
for directory in generated_dirs():
    for root, _, names in os.walk(os.path.join(REPO, directory)):
        for name in names:
            extra += clone(os.path.relpath(os.path.join(root, name), REPO), target)
print(f"p9-overlay: {len(files)} listed, {copied} cloned/refreshed, {extra} generated files into {target}")
