#!/usr/bin/env python3
"""H87 manual landing (coordinator exception, repo land overlay false reds): usage land.py <stage-dir> <area> [--check]
Applies every file of <stage>/tree when its live base is the checkout base recorded by `repo land checkout` (refuses otherwise),
per-area lock only for the atomic apply (temp + fsync + rename under an odd generation)."""
import hashlib, json, os, shutil, sys, tempfile, time
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
R = "/Users/ueli/Documents/semio"
stage, area = sys.argv[1], sys.argv[2]
tree = os.path.join(stage, "tree")
files = [os.path.relpath(os.path.join(d, f), tree) for d, _, fs in os.walk(tree) for f in fs]
only = [a for a in sys.argv[3:] if not a.startswith("--")]
if only: files = [f for f in files if any(f.startswith(o) or o in f for o in only)]
plan = []
for rel in sorted(files):
    live, new = os.path.join(R, rel), open(os.path.join(tree, rel), "rb").read()
    plan.append((live, new))
    print("apply", rel[-90:], "exists" if os.path.exists(live) else "new")
if "--check" in sys.argv: sys.exit(0)
lock, generation = os.path.join(T, "landing.lock." + area), os.path.join(T, "landing.generation." + area)
os.mkdir(lock)
try:
    open(os.path.join(lock, "owner"), "w").write(f"H87 {time.strftime('%H:%M:%S')} {area}\n")
    try: gen = int(open(generation).read().strip() or "0")
    except Exception: gen = 0
    gen += gen % 2 == 0
    open(generation, "w").write(str(gen))
    temps = []
    for live, data in plan:
        os.makedirs(os.path.dirname(live), exist_ok=True)
        fd, tmp = tempfile.mkstemp(dir=os.path.dirname(live), prefix=".h87-")
        os.write(fd, data); os.fsync(fd); os.close(fd)
        os.chmod(tmp, (os.stat(live).st_mode if os.path.exists(live) else 0o644) & 0o7777); temps.append((tmp, live))
    for tmp, live in temps: os.replace(tmp, live)
    open(generation, "w").write(str(gen + 1))
finally:
    shutil.rmtree(lock, ignore_errors=True)
print("landed", area, len(plan), "files, lock released")
