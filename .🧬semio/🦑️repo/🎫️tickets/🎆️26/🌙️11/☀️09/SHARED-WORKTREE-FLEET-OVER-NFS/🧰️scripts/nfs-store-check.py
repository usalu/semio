#!/usr/bin/env python3
"""usage: nfs-store-check.py STORE [SINCE_EPOCH] -- independent integrity check of a store (Python standard library only): per origin head.json against its segments (sealed segments contiguous, every line up to head.bytes of the head segment is a complete JSON line, the head ends at a newline), every `put` event names a blob of the right size, blobs newer than SINCE_EPOCH hash to their name, leftovers in v1/tmp. Prints one JSON line; exit 1 on any problem."""
import hashlib, json, os, sys

store, since = sys.argv[1], float(sys.argv[2]) if len(sys.argv) > 2 else None
v1 = os.path.join(store, "v1"); problems = []; events = puts = 0; checked_blobs = 0; hashed = 0
for origin in sorted(os.listdir(os.path.join(v1, "origins"))):
    base = os.path.join(v1, "origins", origin)
    try: head = json.load(open(os.path.join(base, "head.json")))
    except FileNotFoundError: continue
    except ValueError: problems.append("head.json of %s is not JSON" % origin); continue
    segs = sorted(os.listdir(os.path.join(base, "segments")))
    want = ["%012d.jsonl" % i for i in range(head["segment"] + 1)]
    if segs[:len(want)] != want: problems.append("%s: segments %s, head says %d" % (origin, segs[-3:], head["segment"]))
    for name in want:
        data = open(os.path.join(base, "segments", name), "rb").read()
        if name == want[-1]:
            if len(data) < head["bytes"]: problems.append("%s: head.bytes %d beyond the segment (%d)" % (origin, head["bytes"], len(data))); continue
            data = data[:head["bytes"]]
        if data and not data.endswith(b"\n"): problems.append("%s/%s: published bytes end inside a line" % (origin, name))
        for line in data.split(b"\n")[:-1] if data.endswith(b"\n") else data.split(b"\n"):
            if not line: continue
            try: ev = json.loads(line)
            except ValueError: problems.append("%s/%s: unparseable line" % (origin, name)); break
            events += 1
            if ev.get("operation") == "put":
                puts += 1; sha = ev["sha256"]; p = os.path.join(v1, "blobs", sha[:2], sha)
                try: size = os.stat(p).st_size
                except OSError: problems.append("missing blob %s" % sha); continue
                checked_blobs += 1
                if size != ev["bytes"]: problems.append("blob %s has %d bytes, event says %d" % (sha, size, ev["bytes"]))
                elif since is not None and os.stat(p).st_mtime >= since:
                    hashed += 1
                    if hashlib.sha256(open(p, "rb").read()).hexdigest() != sha: problems.append("blob %s does not hash to its name" % sha)
tmp = os.path.join(v1, "tmp"); left = os.listdir(tmp) if os.path.isdir(tmp) else []
print(json.dumps({"origins": len(os.listdir(os.path.join(v1, "origins"))), "events": events, "puts": puts, "blobs_checked": checked_blobs, "blobs_hashed": hashed, "tmp_leftovers": len(left), "problems": problems[:10], "problem_count": len(problems)}))
sys.exit(1 if problems else 0)
