"""Samples the process table every 2 s and records the first and last time a cargo/rustc/scheduler/nested-repo command line is seen (rustedit-g). Streams, aborts above 200 MB RSS.
usage: procs.py <out.jsonl> <stopfile>
"""
import json, os, re, resource, subprocess, sys, time

out, stop = sys.argv[1], sys.argv[2]
seen = {}
t0 = time.time()
KEEP = re.compile(r"(^|/)(cargo|rustc|semio-build-scheduler|rustc-wrapper)\b|cargo (build|fetch|metadata|config|check)|ScriptsRepo|bazel\(|repo ./")
while not os.path.exists(stop):
    text = subprocess.run(["ps", "-A", "-o", "pid=,ppid=,time=,rss=,command="], capture_output=True, text=True).stdout
    now = time.time() - t0
    for line in text.splitlines():
        parts = line.split(None, 4)
        if len(parts) < 5 or not KEEP.search(parts[4]):
            continue
        cmd = re.sub(r"/Users/ueli/Documents/semio/", "", parts[4])
        key = (parts[0], cmd[:110])
        row = seen.setdefault(key, {"pid": parts[0], "ppid": parts[1], "first": now, "last": now, "rss": 0, "cpu": parts[2], "cmd": cmd[:160]})
        row["last"] = now
        row["cpu"] = parts[2]
        row["rss"] = max(row["rss"], int(parts[3]) if parts[3].isdigit() else 0)
    if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 200 * 1024 * 1024:
        break
    time.sleep(2)
with open(out, "w") as handle:
    for row in sorted(seen.values(), key=lambda r: r["first"]):
        row["first"], row["last"] = round(row["first"]), round(row["last"])
        handle.write(json.dumps(row, ensure_ascii=False) + "\n")
