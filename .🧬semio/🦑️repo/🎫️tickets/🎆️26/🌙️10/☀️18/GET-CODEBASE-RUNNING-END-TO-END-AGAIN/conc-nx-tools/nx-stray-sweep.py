#!/usr/bin/env python3
"""Lists (default) or deletes (--apply) stray per-run Nx workspace-data directories left in ticket `🗑️generated` folders: a directory is stray when it holds project-graph.json and file-map.json (or nx_files.nxt) and no file in it is newer than --idle-hours (default 24). Logs, evidence and cargo targets are never touched; symbolic links are never followed.
usage: nx-stray-sweep.py [--root TICKETS_DIR] [--idle-hours 24] [--apply]"""
import os, shutil, sys, time

args = sys.argv[1:]
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8), ".🧬semio", "🦑️repo", "🎫️tickets"))
if "--root" in args:
    ROOT = os.path.abspath(args[args.index("--root") + 1])
idle = float(args[args.index("--idle-hours") + 1]) * 3600 if "--idle-hours" in args else 24 * 3600
now = time.time()
GENERATED = "🗑️generated"


def newest_and_size(path):
    newest, size = 0.0, 0
    for base, dirs, files in os.walk(path, followlinks=False):
        for name in files:
            try:
                status = os.lstat(os.path.join(base, name))
            except OSError:
                continue
            newest = max(newest, status.st_mtime)
            size += status.st_size
    return newest, size


found = []
for base, dirs, files in os.walk(ROOT, followlinks=False):
    inside = GENERATED in base.split(os.sep)
    if not inside:
        dirs[:] = [d for d in dirs if not d.startswith(".git")]
    if inside and "project-graph.json" in files and ("file-map.json" in files or "nx_files.nxt" in files):
        newest, size = newest_and_size(base)
        found.append((base, newest, size))
        dirs[:] = []
stray = [row for row in found if now - row[1] >= idle]
for base, newest, size in sorted(stray, key=lambda row: -row[2]):
    print("%8.1f MiB  idle %5.1f h  %s" % (size / 1048576, (now - newest) / 3600, base))
print("stray %d of %d nx workspace-data directories, %.1f MiB" % (len(stray), len(found), sum(row[2] for row in stray) / 1048576))
if "--apply" in args:
    for base, _, _ in stray:
        shutil.rmtree(base)
    print("deleted %d directories" % len(stray))
else:
    print("DRY RUN: nothing deleted (use --apply)")
