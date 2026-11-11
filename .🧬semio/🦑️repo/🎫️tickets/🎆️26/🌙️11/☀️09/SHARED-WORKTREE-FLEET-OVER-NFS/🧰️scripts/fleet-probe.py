#!/usr/bin/env python3
"""usage: fleet-probe.py PROBES.jsonl ORIGINS.json RESULTS.jsonl -- follows the probe log written by fleet-load.py; for each probe picks one reader in the writer's group and one in another group and polls (20 ms) until the reader's tree holds the same bytes; appends {w,g,scope,r,ms,big,n} (ms null after 180 s)."""
import hashlib, json, os, random, sys, time
import fleet

probes, origins_file, results = sys.argv[1:4]
rnd = random.Random(5); pending = []; pos = 0; known = []; loaded = 0
out = open(results, "a", buffering=1)

def match(r, p):
    path = f"{fleet.tree(r)}/{p['p']}"
    try:
        if os.stat(path).st_size != p["n"]: return False
        with open(path, "rb") as f: data = f.read()
        return hashlib.sha256(data).hexdigest() == p["sha"]
    except OSError: return False

while True:
    now = time.time()
    if now - loaded > 5:
        try: known = json.load(open(origins_file)); loaded = now
        except (OSError, ValueError): pass
    try:
        with open(probes) as f:
            f.seek(pos); chunk = f.read()
            end = chunk.rfind("\n") + 1
            for line in chunk[:end].splitlines():
                p = json.loads(line); w = p["w"]; g = fleet.group_of(w)
                same = [r for r in known if fleet.group_of(r) == g and r != w]; other = [r for r in known if fleet.group_of(r) != g]
                if same: pending.append((p, "in", rnd.choice(same)))
                if other: pending.append((p, "cross", rnd.choice(other)))
            pos += end
    except OSError: pass
    keep = []
    for item in pending:
        p, scope, r = item; ms = None
        if match(r, p): ms = (time.time() - p["t"]) * 1000
        elif time.time() - p["t"] > 180: ms = -1
        if ms is None: keep.append(item)
        else: out.write(json.dumps({"w": p["w"], "g": fleet.group_of(p["w"]), "scope": scope, "r": r, "ms": None if ms < 0 else round(ms), "big": bool(p.get("big")), "n": p["n"], "t": p["t"]}) + "\n")
    pending = keep
    time.sleep(0.02)
