#!/usr/bin/env python3
"""Samples the process tree under a root pid every N seconds into a CSV (ticket tool, stdlib only, streams, own RSS stays small).

usage: sample.py <root-pid | command-substring> <out.csv> [interval-seconds]   (a substring selects the PARENT of the first process whose command contains it, e.g. the gate that spawned `playwright/cli.js test`)
Columns: time, load1, tree RSS MB, then RSS MB / %CPU / process count per class (chromium, playwright node, vite+bun site scripts, proctor, other).
Ends when the root pid is gone. summarize: sample.py --summary <out.csv>
"""
import csv, os, resource, subprocess, sys, time

CLASSES = ("chromium", "playwright", "site", "proctor", "other")

def classify(command: str) -> str:
    low = command.lower()
    if "chrom" in low or "headless_shell" in low:
        return "chromium"
    if "playwright" in low:
        return "playwright"
    if "dev-site" in low or "vite" in low or "📜️script.ts" in command:
        return "site"
    if "proctor" in low:
        return "proctor"
    return "other"

def cpu_seconds(text):
    """ps TIME [[dd-]hh:]mm:ss.xx -> seconds"""
    days, _, rest = text.rpartition("-")
    fields = [float(x) for x in rest.split(":")]
    total = 0.0
    for x in fields:
        total = total * 60 + x
    return total + (float(days) * 86400 if days else 0.0)

def table():
    out = subprocess.run(["ps", "-axo", "pid=,ppid=,rss=,pcpu=,time=,lstart=,command="], capture_output=True, text=True).stdout
    rows = {}
    for line in out.splitlines():
        parts = line.split(None, 10)
        if len(parts) == 11:
            rows[int(parts[0])] = (int(parts[1]), int(parts[2]), float(parts[3]), parts[10], cpu_seconds(parts[4]), " ".join(parts[5:10]))
    return rows

def tree(root, rows):
    kids = {}
    for pid, (ppid, *_rest) in rows.items():
        kids.setdefault(ppid, []).append(pid)
    seen, i = [root], 0
    while i < len(seen):
        seen.extend(kids.get(seen[i], []))
        i += 1
    return [pid for pid in seen if pid in rows]

def summary(path):
    peak = {"rss": 0.0}
    last = {}
    total_cpu = 0.0
    n = 0
    with open(path) as handle:
        reader = csv.DictReader(handle)
        for row in reader:
            n += 1
            for key, value in row.items():
                if key != "time":
                    peak[key] = max(peak.get(key, 0.0), float(value))
            total_cpu += sum(float(row[f"{c}_cpu"]) for c in CLASSES)
            if f"{CLASSES[0]}_cpus" in row:
                last = {c: float(row[f"{c}_cpus"]) for c in CLASSES}
    print("CPU-seconds by class (ps TIME, summed over every process seen):", {k: round(v) for k, v in last.items()}, "total", round(sum(last.values())))
    print({k: round(v, 1) for k, v in peak.items() if k != "rss"}, "samples", n, "mean tree cpu % (ps, decayed)", round(total_cpu / max(1, n), 1))

def main():
    if sys.argv[1] == "--summary":
        return summary(sys.argv[2])
    target, out = sys.argv[1], sys.argv[2]
    interval = float(sys.argv[3]) if len(sys.argv) > 3 else 5.0
    root = int(target) if target.isdigit() else 0
    deadline = time.time() + 3 * 3600
    while root == 0 and time.time() < deadline:
        rows = table()
        hit = next((pid for pid, (_p, _r, _c, command, _t, _s) in rows.items() if target in command and "sample.py" not in command and "detach.py" not in command), 0)
        root = rows[hit][0] if hit else 0
        if root == 0:
            time.sleep(interval)
    seen, done = {}, {c: 0.0 for c in CLASSES}
    with open(out, "w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(["time", "load1", "tree_mb"] + [f"{c}_{m}" for c in CLASSES for m in ("mb", "cpu", "n")] + [f"{c}_cpus" for c in CLASSES])
        while True:
            rows = table()
            if root not in rows:
                break
            members = tree(root, rows)
            agg = {c: [0.0, 0.0, 0] for c in CLASSES}
            for pid in members:
                _ppid, rss, cpu, command, used, started = rows[pid]
                a = agg[classify(command)]
                a[0] += rss / 1024
                a[1] += cpu
                a[2] += 1
                seen[(pid, started)] = (classify(command), used)
            for c in CLASSES:
                done[c] = sum(used for (kind, used) in seen.values() if kind == c)
            writer.writerow([time.strftime("%H:%M:%S"), round(os.getloadavg()[0], 1), round(sum(a[0] for a in agg.values()))] + [x for c in CLASSES for x in (round(agg[c][0]), round(agg[c][1]), agg[c][2])] + [round(done[c], 1) for c in CLASSES])
            handle.flush()
            if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 200 * 1048576:
                break
            time.sleep(interval)

main()
