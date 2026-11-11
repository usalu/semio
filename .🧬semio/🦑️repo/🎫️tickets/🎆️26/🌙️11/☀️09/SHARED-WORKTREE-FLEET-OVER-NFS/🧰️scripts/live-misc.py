#!/usr/bin/env python3
"""usage: live-misc.py rename|exec|emoji WRITER TREE... -- (f) rename a directory holding 500 files, (g) flip an executable bit, (h) create a file whose relative path is 250 characters of 3-byte symbols; prints the time until every TREE shows the change."""
import json, os, stat, sys, time

mode, writer, trees = sys.argv[1], sys.argv[2], sys.argv[3:]

def wait(check, limit=120):
    t0 = time.monotonic(); done = {}
    while len(done) < len(trees) and time.monotonic() - t0 < limit:
        for t in trees:
            if t not in done and check(t):
                done[t] = round((time.monotonic() - t0) * 1000)
        time.sleep(0.05)
    return t0, {os.path.basename(k): v for k, v in done.items()}

if mode == "rename":
    src, dst, n = "rehearsal-rename/src", "rehearsal-rename/dst", 500
    os.makedirs(os.path.join(writer, src))
    for i in range(n):
        os.makedirs(os.path.join(writer, src, "d%d" % (i % 10)), exist_ok=True)
        with open(os.path.join(writer, src, "d%d" % (i % 10), "f%03d.txt" % i), "w") as f: f.write("rename %d\n" % i)
    _, created = wait(lambda t: os.path.isfile(os.path.join(t, src, "d9", "f%03d.txt" % (n - 1))))
    time.sleep(2)
    t0 = time.monotonic()
    os.rename(os.path.join(writer, src), os.path.join(writer, dst))
    def complete(t):
        return not os.path.exists(os.path.join(t, src)) and sum(len(fs) for _, _, fs in os.walk(os.path.join(t, dst))) == n
    _, done = wait(complete)
    print(json.dumps({"created_visible_ms": created, "renamed_complete_ms": done, "src_pruned_everywhere": all(not os.path.exists(os.path.join(t, src)) for t in [writer] + trees)}))
elif mode == "exec":
    rel = "rehearsal-live/tool.sh"
    p = os.path.join(writer, rel)
    with open(p, "w") as f: f.write("#!/bin/sh\necho hi\n")
    os.chmod(p, 0o644)
    wait(lambda t: os.path.isfile(os.path.join(t, rel)))
    time.sleep(2)
    os.chmod(p, 0o755)
    _, done = wait(lambda t: os.stat(os.path.join(t, rel)).st_mode & 0o111 == 0o111)
    os.chmod(p, 0o644)
    _, back = wait(lambda t: os.stat(os.path.join(t, rel)).st_mode & 0o111 == 0)
    print(json.dumps({"exec_on_ms": done, "exec_off_ms": back}))
else:
    syms = [chr(c) for c in range(0x2600, 0x2700) if chr(c).isprintable()][:200]
    parts = []
    for i, length in enumerate((62, 62, 62, 61)):
        parts.append("".join(syms[(i * 7 + j) % len(syms)] for j in range(length)))
    rel = "/".join(parts)
    assert len(rel) == 250, len(rel)
    p = os.path.join(writer, rel)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "w") as f: f.write("long emoji path\n")
    _, done = wait(lambda t: os.path.isfile(os.path.join(t, rel)))
    print(json.dumps({"relative_chars": len(rel), "relative_utf8_bytes": len(rel.encode()), "absolute_bytes": len(p.encode()), "visible_ms": done}))
