#!/usr/bin/env python3
"""🧹️ Disk-emergency prune of STALE cargo build units in one `<target>/<profile>/build` directory: every package keeps its two newest
units (`--keep <n>` changes that); an older unit goes only when its newest file is older than `<min-idle-hours>` and its `.lock` (when present) can be taken
exclusively without blocking. Build units are regenerable cache. Usage: `<build-dir> <min-idle-hours> [--keep <n>] [--apply]` (dry run by default);
prints units and bytes, and with `--apply` appends the removed unit paths to `🗑️generated/coord/stale-units-removed.txt`."""
import fcntl, os, shutil, sys, time

build, hours = sys.argv[1], float(sys.argv[2])
apply = "--apply" in sys.argv
keep = int(sys.argv[sys.argv.index("--keep") + 1]) if "--keep" in sys.argv else 2
if not os.path.isdir(build) or os.path.basename(os.path.normpath(build)) != "build":
    sys.exit("refusing: not a cargo `build` directory")
log = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "coord", "stale-units-removed.txt")
bound = time.time() - hours * 3600
units = freed = skipped = 0
for package in sorted(os.listdir(build)):
    directory = os.path.join(build, package)
    if not os.path.isdir(directory):
        continue
    rows = []
    for unit in os.listdir(directory):
        path = os.path.join(directory, unit)
        if not os.path.isdir(path):
            continue
        size = newest = 0
        for base, _, files in os.walk(path):
            for name in files:
                try:
                    stat = os.lstat(os.path.join(base, name))
                except OSError:
                    continue
                size += stat.st_blocks * 512
                newest = max(newest, stat.st_mtime)
        rows.append((newest, size, path))
    for newest, size, path in sorted(rows, reverse=True)[keep:]:
        if newest > bound:
            continue
        lock = os.path.join(path, ".lock")
        if os.path.exists(lock):
            try:
                handle = os.open(lock, os.O_RDWR)
                fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except OSError:
                skipped += 1
                continue
            if os.fstat(handle).st_mtime > bound:
                skipped += 1
                continue
        units += 1
        freed += size
        if apply:
            shutil.rmtree(path, ignore_errors=True)
            with open(log, "a", encoding="utf-8") as out:
                out.write(f"{time.strftime('%F %T')} {path}\n")
print(f"{'removed' if apply else 'would remove'} {units} stale units, {freed / 2**30:.1f} GiB; {skipped} locked units skipped")
