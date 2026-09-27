#!/usr/bin/env python3
"""🪞️ ST2 overlay sync: makes `<overlay>` byte-equal to the live tree's git-indexed files (outside `.🧬semio/`) plus the
untracked generated sources a build needs (`🤖️generated/**`, untracked `🦀️.rs`, the ui token file), APFS-cloned.

Idempotent: unchanged files (same size + mtime) are skipped, changed ones re-cloned, overlay files the live tree no longer
has are removed (never under `node_modules`, `target`, `dist` or the private build dirs). Every `node_modules` directory of
the live tree becomes a symlink so Bun/Vitest resolve third-party packages. Patches applied to the overlay afterwards are
undone by the next sync (their new files are removed, their edits re-cloned).

usage: python3 st2-overlay.py <overlay-root>
"""
import ctypes
import os
import subprocess
import sys

REPO = "/Users/ueli/Documents/semio"
KEEP = {"node_modules", "target", "dist", ".st2-build", ".st2-target"}
libc = ctypes.CDLL("/usr/lib/libSystem.B.dylib", use_errno=True)
libc.clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]


def clone(source, destination):
    if os.path.lexists(destination):
        os.remove(destination)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    if os.path.islink(source):
        os.symlink(os.readlink(source), destination)
        return
    if libc.clonefile(source.encode(), destination.encode(), 0) != 0:
        raise OSError(ctypes.get_errno(), f"clonefile {source}")


def listing():
    tracked = subprocess.run(["git", "ls-files", "-z", "--", ":!:.🧬semio/**"], cwd=REPO, capture_output=True, check=True).stdout.decode().split("\0")
    others = subprocess.run(["git", "ls-files", "-z", "--others", "--", "*🤖️generated/*", "*/🦀️.rs", "*/🟦️.ts", ":!:.🧬semio/**", ":!:**/node_modules/**", ":!:**/target/**", ":!:**/dist/**"], cwd=REPO, capture_output=True, check=True).stdout.decode().split("\0")
    return sorted({rel for rel in tracked + others if rel and os.path.lexists(os.path.join(REPO, rel))})


def main():
    overlay = sys.argv[1]
    os.makedirs(overlay, exist_ok=True)
    wanted = listing()
    wanted_set = set(wanted)
    cloned = skipped = 0
    for rel in wanted:
        source = os.path.join(REPO, rel)
        destination = os.path.join(overlay, rel)
        if os.path.lexists(destination) and not os.path.islink(source):
            a, b = os.lstat(source), os.lstat(destination)
            if a.st_size == b.st_size and int(a.st_mtime) == int(b.st_mtime) and not os.path.islink(destination):
                skipped += 1
                continue
        clone(source, destination)
        cloned += 1
    removed = 0
    removed_log = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "generated", "overlay-removed.txt"), "w", encoding="utf-8")
    for dirpath, dirnames, filenames in os.walk(overlay):
        dirnames[:] = [name for name in dirnames if name not in KEEP and not os.path.islink(os.path.join(dirpath, name))]
        for name in filenames:
            path = os.path.join(dirpath, name)
            rel = os.path.relpath(path, overlay)
            if rel not in wanted_set and name != ".DS_Store":
                os.remove(path)
                removed_log.write(rel + "\n")
                removed += 1
    removed_log.close()
    for dirpath, dirnames, _files in os.walk(overlay, topdown=False):
        if os.path.basename(dirpath) in KEEP or any(part in KEEP for part in os.path.relpath(dirpath, overlay).split(os.sep)):
            continue
        if dirpath != overlay and not os.listdir(dirpath):
            os.rmdir(dirpath)
    links = 0
    found = []
    for dirpath, dirnames, _files in os.walk(REPO):
        rel_dir = os.path.relpath(dirpath, REPO)
        if "node_modules" in dirnames:
            found.append(os.path.normpath(os.path.join(rel_dir, "node_modules")))
        dirnames[:] = [name for name in dirnames if name not in ("node_modules", "target", "dist", ".git", ".🧬semio", ".tmp-ticket", ".tmp-ticket-0918") and not name.startswith(".") and os.path.isdir(os.path.join(overlay, rel_dir, name))]
    for rel in found:
        destination = os.path.join(overlay, rel)
        if os.path.islink(destination):
            continue
        if not os.path.isdir(os.path.dirname(destination)):
            continue
        os.symlink(os.path.join(REPO, rel), destination)
        links += 1
    print(f"st2-overlay: {len(wanted)} files wanted, {cloned} cloned, {skipped} unchanged, {removed} removed, {links} node_modules links → {overlay}")


if __name__ == "__main__":
    main()
