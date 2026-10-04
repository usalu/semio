#!/usr/bin/env python3
"""🧹️ One-off disk recovery, run ONLY while no cargo runs (it refuses otherwise): every package keeps its newest build unit
(two for third-party packages) per profile, older units whose `.lock` can be taken exclusively are removed; nx cache entries
older than 2 h go too. `--emergency` runs even while a cargo runs (locked units are still skipped). Prints the bytes recovered."""
import fcntl, os, shutil, subprocess, sys, time
ROOT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build"
NX = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/nx"
if "--emergency" not in sys.argv and subprocess.run(["pgrep", "-x", "cargo"], capture_output=True).stdout.strip():
    sys.exit("a cargo is running; refusing (pass --emergency below a few GiB free)")
def newest(path):
    m = os.stat(path).st_mtime
    for base, _, files in os.walk(path):
        for f in files:
            try: m = max(m, os.lstat(os.path.join(base, f)).st_mtime)
            except OSError: pass
    return m
before = shutil.disk_usage(ROOT).free
removed = 0
profiles = [os.path.join(ROOT, "debug", "build"), os.path.join(ROOT, "release", "build")]
for target in ("wasm32-wasip2", "wasm32-unknown-unknown"):
    base = os.path.join(ROOT, target)
    if os.path.isdir(base): profiles += [os.path.join(base, p, "build") for p in os.listdir(base)]
for build in profiles:
    if not os.path.isdir(build): continue
    for pkg in os.listdir(build):
        pdir = os.path.join(build, pkg)
        units = [os.path.join(pdir, u) for u in os.listdir(pdir) if os.path.isdir(os.path.join(pdir, u))]
        keep = 1 if pkg.startswith("semio-") else 2
        if len(units) <= keep: continue
        for _, unit in sorted(((newest(u), u) for u in units), reverse=True)[keep:]:
            try:
                fd = os.open(os.path.join(unit, ".lock"), os.O_RDWR | os.O_CREAT)
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except OSError:
                continue
            try: shutil.rmtree(unit, ignore_errors=True); removed += 1
            finally: os.close(fd)
bound = time.time() - 2 * 3600
for entry in os.listdir(NX) if os.path.isdir(NX) else []:
    path = os.path.join(NX, entry)
    if entry[:1].isdigit() and os.stat(path).st_mtime < bound: shutil.rmtree(path, ignore_errors=True) if os.path.isdir(path) else os.remove(path)
print(f"units removed {removed}; free {before / 2**30:.1f} GiB -> {shutil.disk_usage(ROOT).free / 2**30:.1f} GiB")
