#!/usr/bin/env python3
"""usage: nfs-attr-probe.py write DIR N INTERVAL_S LOG | watch stat|read|both DIR SECONDS LOG | join WRITE_LOG WATCH_LOG READER_MINUS_WRITER_MS
write (machine A): replace DIR/h.json atomically (temp file + rename, like head.json) N times; LOG: "i epoch_after_rename mtime_ns ino".
watch (machine B): poll DIR/h.json every 2 ms; stat = os.stat only, read = open+read only, both = stat then open+read; LOG: "S epoch mtime_ns ino size" when the stat signature changes, "R epoch i" when a new content index appears.
join: per edit the delay until the stat signature (matched by mtime_ns) and until the content became visible, in ms (median, p95, max)."""
import json, os, statistics, sys, time

def write(directory, n, interval, log):
    os.makedirs(directory, exist_ok=True)
    final = os.path.join(directory, "h.json"); temp = os.path.join(directory, ".h.tmp")
    with open(log, "w") as lf:
        for i in range(n):
            with open(temp, "w") as f:
                f.write("edit %d %s\n" % (i, os.urandom(8).hex()))
            os.rename(temp, final)
            t = time.time(); st = os.stat(final)
            lf.write("%d %.6f %d %d\n" % (i, t, st.st_mtime_ns, st.st_ino)); lf.flush()
            time.sleep(interval)

def watch(mode, directory, seconds, log):
    path = os.path.join(directory, "h.json"); sig = None; seen = set(); end = time.time() + seconds
    with open(log, "w") as lf:
        while time.time() < end:
            try:
                if mode in ("stat", "both"):
                    st = os.stat(path); now = time.time(); cur = (st.st_mtime_ns, st.st_ino, st.st_size)
                    if cur != sig:
                        sig = cur; lf.write("S %.6f %d %d %d\n" % (now, cur[0], cur[1], cur[2])); lf.flush()
                if mode in ("read", "both"):
                    with open(path, "rb") as f:
                        text = f.read()
                    now = time.time(); parts = text.split()
                    if len(parts) == 3 and parts[0] == b"edit" and int(parts[1]) not in seen:
                        seen.add(int(parts[1])); lf.write("R %.6f %d\n" % (now, int(parts[1]))); lf.flush()
            except (OSError, ValueError):
                pass
            time.sleep(0.002)

def stats(values):
    values = sorted(values)
    if not values: return None
    return {"n": len(values), "median_ms": round(statistics.median(values)), "p95_ms": round(values[min(len(values) - 1, int(0.95 * len(values)))]), "max_ms": round(values[-1])}

def join(write_log, watch_log, offset_ms):
    w = {}
    for line in open(write_log):
        i, t, m, ino = line.split(); w[int(i)] = (float(t), int(m))
    by_mtime = {m: i for i, (t, m) in w.items()}
    stat_seen = {}; read_seen = {}
    for line in open(watch_log):
        p = line.split()
        if p[0] == "S" and int(p[2]) in by_mtime: stat_seen.setdefault(by_mtime[int(p[2])], float(p[1]))
        elif p[0] == "R": read_seen.setdefault(int(p[2]), float(p[1]))
    off = offset_ms / 1000
    print(json.dumps({"edits": len(w), "stat_visible": stats([(stat_seen[i] - w[i][0] - off) * 1000 for i in w if i in stat_seen]), "read_visible": stats([(read_seen[i] - w[i][0] - off) * 1000 for i in w if i in read_seen])}))

if sys.argv[1] == "write": write(sys.argv[2], int(sys.argv[3]), float(sys.argv[4]), sys.argv[5])
elif sys.argv[1] == "watch": watch(sys.argv[2], sys.argv[3], float(sys.argv[4]), sys.argv[5])
else: join(sys.argv[2], sys.argv[3], float(sys.argv[4]))
