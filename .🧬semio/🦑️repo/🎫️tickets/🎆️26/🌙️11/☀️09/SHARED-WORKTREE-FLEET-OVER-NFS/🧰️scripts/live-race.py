#!/usr/bin/env python3
"""usage: live-race.py REL TREE_A TREE_B OTHER_TREE... -- write different bytes to REL in TREE_A and TREE_B ~5 ms apart, wait until every tree holds identical bytes, report winner, convergence time and where the loser's bytes are (lost dirs, store blobs)."""
import hashlib, json, os, sys, time

rel, a, b, others = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4:]
trees = [a, b] + others
store_for = os.environ.get("STORES", "").split(":")
os.makedirs(os.path.join(a, os.path.dirname(rel)), exist_ok=True)
os.makedirs(os.path.join(b, os.path.dirname(rel)), exist_ok=True)
da, db = b"writer A (first)\n", b"writer B (5 ms later)\n"
t0 = time.monotonic()
with open(os.path.join(a, rel), "wb") as f: f.write(da)
time.sleep(0.005)
with open(os.path.join(b, rel), "wb") as f: f.write(db)
gap = (time.monotonic() - t0) * 1000
final = None
t1 = time.monotonic()
stable_since = None
while time.monotonic() - t0 < 60:
    vals = []
    for t in trees:
        try: vals.append(open(os.path.join(t, rel), "rb").read())
        except OSError: vals.append(None)
    if len(set(vals)) == 1 and vals[0] in (da, db):
        if stable_since is None: stable_since = time.monotonic()
        if time.monotonic() - stable_since > 3: final = vals[0]; break
    else:
        stable_since = None
    time.sleep(0.05)
conv = (stable_since - t0) * 1000 if stable_since else None
loser = db if final == da else da
lh = hashlib.sha256(loser).hexdigest()
where = []
for t in trees:
    lost = os.path.join(t, ".cache", "f", "lost")
    if os.path.isdir(lost):
        for base, _, names in os.walk(lost):
            for n in names:
                p = os.path.join(base, n)
                try:
                    if open(p, "rb").read() == loser: where.append(p)
                except OSError: pass
for s in [x for x in store_for if x]:
    p = os.path.join(s, "v1", "blobs", lh[:2], lh)
    if os.path.exists(p): where.append(p)
print(json.dumps({"write_gap_ms": round(gap, 1), "winner": "A" if final == da else "B" if final == db else None, "converged_ms": conv and round(conv), "loser_sha256": lh, "loser_bytes_found_at": [w[-110:] for w in where]}))
