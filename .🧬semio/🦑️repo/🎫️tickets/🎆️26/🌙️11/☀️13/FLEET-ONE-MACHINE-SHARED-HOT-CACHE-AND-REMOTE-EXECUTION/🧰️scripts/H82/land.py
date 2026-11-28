#!/usr/bin/env python3
"""H82 landing of staged files from the scratch mirror: per area lock, base check (3-way merge with git merge-file when a peer
changed the live file since the stage, never when conflicts), temp + fsync + rename under an odd generation, lock released at once.
usage: land.py <scratch> <group>   group = f4 (remote-execution) | f9 (caching-cargo) | f5 (graph, post-seal only)
       land.py <scratch> <group> --check   verifies bases and merges without writing"""
import hashlib, os, shutil, subprocess, sys, tempfile, time
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
REPO = "/Users/ueli/Documents/semio"
scratch, group = sys.argv[1], sys.argv[2]
check = "--check" in sys.argv
area = {"f4": "remote-execution", "f9": "caching-cargo", "f5": "graph"}[group]
rows = [line.split(None, 1) for line in open(os.path.join(scratch, f"bases-{group}.txt")).read().splitlines() if line.strip()]
sha = lambda path: hashlib.sha256(open(path, "rb").read()).hexdigest()
plan = []
for base_hash, rel in rows:
    live, staged, base = os.path.join(REPO, rel), os.path.join(scratch, "tree", rel), os.path.join(scratch, "base", rel)
    new = open(staged, "rb").read()
    if sha(live) == base_hash:
        plan.append((live, new, "direct"))
        continue
    merged = tempfile.mktemp(prefix="h82-merge-")
    shutil.copy(live, merged)
    run = subprocess.run(["git", "merge-file", merged, base, staged], capture_output=True)
    if run.returncode != 0:
        print("CONFLICT", rel, run.returncode); sys.exit(2)
    plan.append((live, open(merged, "rb").read(), "merged")); os.remove(merged)
for live, _, how in plan: print(how, live[-80:])
if check: sys.exit(0)
lock, generation = os.path.join(T, "landing.lock." + area), os.path.join(T, "landing.generation." + area)
os.mkdir(lock)
try:
    open(os.path.join(lock, "owner"), "w").write(f"H82 {time.strftime('%H:%M:%S')} {group}\n")
    try: gen = int(open(generation).read().strip() or "0")
    except Exception: gen = 0
    gen += gen % 2 == 0
    open(generation, "w").write(str(gen))
    temps = []
    for live, data, _ in plan:
        fd, tmp = tempfile.mkstemp(dir=os.path.dirname(live), prefix=".h82-")
        os.write(fd, data); os.fsync(fd); os.close(fd)
        os.chmod(tmp, os.stat(live).st_mode & 0o7777); temps.append((tmp, live))
    for tmp, live in temps: os.replace(tmp, live)
    open(generation, "w").write(str(gen + 1))
finally:
    shutil.rmtree(lock, ignore_errors=True)
print("landed", group, len(plan), "files, lock", area, "released")
