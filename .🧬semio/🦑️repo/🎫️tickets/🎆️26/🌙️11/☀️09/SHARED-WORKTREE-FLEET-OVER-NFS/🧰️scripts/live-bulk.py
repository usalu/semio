#!/usr/bin/env python3
"""usage: live-bulk.py create WRITER DIR N TREE... | delete WRITER DIR TREE... -- (b) create N small files in WRITER/DIR and time until all exist with identical bytes in every TREE; (c) remove WRITER/DIR and time until the directory is gone from every TREE."""
import json, os, shutil, sys, time

mode = sys.argv[1]
if mode == "create":
    writer, d, n, trees = sys.argv[2], sys.argv[3], int(sys.argv[4]), sys.argv[5:]
    os.makedirs(os.path.join(writer, d), exist_ok=True)
    t0 = time.monotonic()
    for i in range(n):
        with open(os.path.join(writer, d, "f%04d.txt" % i), "w") as f:
            f.write("file %d %s\n" % (i, "x" * (i % 97)))
    created = time.monotonic() - t0
    done = {}
    while len(done) < len(trees) and time.monotonic() - t0 < 300:
        for t in trees:
            if t in done:
                continue
            ok = True
            for i in range(n):
                p = os.path.join(t, d, "f%04d.txt" % i)
                try:
                    if open(p).read() != "file %d %s\n" % (i, "x" * (i % 97)):
                        ok = False; break
                except OSError:
                    ok = False; break
            if ok:
                done[t] = round((time.monotonic() - t0) * 1000)
        time.sleep(0.05)
    print(json.dumps({"files": n, "create_ms": round(created * 1000), "present_ms": {os.path.basename(k): v for k, v in done.items()}}))
else:
    writer, d, trees = sys.argv[2], sys.argv[3], sys.argv[4:]
    t0 = time.monotonic()
    shutil.rmtree(os.path.join(writer, d))
    done = {}
    while len(done) < len(trees) and time.monotonic() - t0 < 300:
        for t in trees:
            if t not in done and not os.path.exists(os.path.join(t, d)):
                done[t] = round((time.monotonic() - t0) * 1000)
        time.sleep(0.05)
    print(json.dumps({"gone_ms": {os.path.basename(k): v for k, v in done.items()}}))
