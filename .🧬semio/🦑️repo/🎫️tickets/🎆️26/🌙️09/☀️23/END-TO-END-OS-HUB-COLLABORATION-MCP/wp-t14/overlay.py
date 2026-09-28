#!/usr/bin/env python3
"""🪞️ T14 scratch overlay: an APFS copy-on-write clone (`clonefile(2)`, whole directories in one call, no data copied) of the
repo's working tree — every top-level entry except `.git`, `.🧬semio` and the ticket links — so gitignored build inputs
(`🤖️generated/`, `node_modules/`) come along. `--refresh <rel>…` re-clones single files/dirs from the live tree.
`sync` brings an existing overlay back to the live tree incrementally: every file newer than the overlay's stamp on EITHER
side (live edits, and files a patch wrote in the overlay) is re-cloned from the live tree, files deleted live are deleted,
then the stamp moves — no rmtree of whole trees, and every earlier patch is undone so patch scripts re-apply cleanly.
`sync <overlay> <base-commit>` also deletes every tracked file git removed since that commit (a deleted file older than the
stamp is invisible to the mtime walk).
usage: overlay.py create <overlay> | overlay.py sync <overlay> [<base-commit>] | overlay.py refresh <overlay> <rel>…"""
import ctypes
import os
import shutil
import sys
import time

ROOT = "/Users/ueli/Documents/semio"
SKIP = {".git", ".🧬semio", ".tmp-ticket", ".tmp-ticket-0918", ".tmp-wp-o3", ".%F0%9F%A7%ACsemio", ".DS_Store", ".nx", ".pytest_cache"}
libc = ctypes.CDLL("libc.dylib", use_errno=True)
libc.clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]


def clone(source, destination):
    if os.path.lexists(destination):
        if os.path.isdir(destination) and not os.path.islink(destination):
            trash = os.path.join(overlay, ".t14-trash")
            os.makedirs(trash, exist_ok=True)
            os.rename(destination, os.path.join(trash, f"{os.path.basename(destination)}-{time.time_ns()}"))
        else:
            os.unlink(destination)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    if libc.clonefile(source.encode(), destination.encode(), 1) != 0:
        raise OSError(ctypes.get_errno(), f"clonefile {source}")


mode, overlay = sys.argv[1], sys.argv[2]
started = time.time()
if mode == "create":
    os.makedirs(overlay, exist_ok=True)
    for entry in sorted(os.listdir(ROOT)):
        if entry in SKIP:
            continue
        clone(os.path.join(ROOT, entry), os.path.join(overlay, entry))
        print(f"cloned {entry} {time.time() - started:.1f}s", flush=True)
    open(os.path.join(overlay, ".t14-stamp"), "w").close()
elif mode == "sync":
    import subprocess
    stamp = os.path.join(overlay, ".t14-stamp")
    next_stamp = stamp + ".next"
    open(next_stamp, "w").close()
    prune = [arg for name in ("node_modules", "dist", "target", "🗑️generated", "generated", ".venv", ".t14-build", ".t14-target", ".t14-trash") for arg in ("-o", "-name", name)][1:]
    stale = set()
    for base in (ROOT, overlay):
        found = subprocess.run(["find", base, "(", *prune, ")", "-prune", "-o", "-newer", stamp, "-type", "f", "-print0"], capture_output=True).stdout.decode("utf-8", "replace").split("\0")
        for path in filter(None, found):
            rel = os.path.relpath(path, base)
            if rel.split(os.sep)[0] in SKIP or rel.startswith(".t14-"):
                continue
            stale.add(rel)
    base = sys.argv[3] if len(sys.argv) > 3 else None
    if base:
        for listing in (["git", "diff", "--no-renames", "--name-only", "--diff-filter=D", base, "HEAD"], ["git", "ls-files", "--deleted"]):
            stale.update(filter(None, subprocess.run(listing + ["-z"], cwd=ROOT, capture_output=True).stdout.decode("utf-8", "replace").split("\0")))
    removed = 0
    for rel in sorted(stale):
        source, destination = os.path.join(ROOT, rel), os.path.join(overlay, rel)
        if os.path.lexists(source):
            clone(source, destination)
        elif os.path.lexists(destination):
            os.unlink(destination)
            removed += 1
    os.replace(next_stamp, stamp)
    print(f"synced {len(stale)} files ({removed} removed)")
elif mode == "refresh":
    for rel in sys.argv[3:]:
        clone(os.path.join(ROOT, rel), os.path.join(overlay, rel))
        print(f"refreshed {rel}")
print(f"done {time.time() - started:.1f}s")
