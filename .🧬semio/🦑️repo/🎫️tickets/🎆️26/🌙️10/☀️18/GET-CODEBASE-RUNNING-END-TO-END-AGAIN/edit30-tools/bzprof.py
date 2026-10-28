"""⏲️ Bazel trace profile summary: `bzprof.py <command-<id>.profile.gz> [min-ms]` prints the absolute start, every complete event of at least min-ms (default 200) relative to the invocation start, and the action spans."""
import datetime, gzip, json, sys

data = json.load(gzip.open(sys.argv[1]))
floor = float(sys.argv[2]) if len(sys.argv) > 2 else 200.0
other = data.get("otherData", {})
start_ms = int(other.get("build_start_ms") or other.get("start_ms") or 0)
names = {}
for e in data["traceEvents"]:
    if e.get("ph") == "M" and e.get("name") == "thread_name":
        names[e["tid"]] = e["args"]["name"]
print("start", datetime.datetime.fromtimestamp(start_ms / 1000).strftime("%H:%M:%S.%f")[:-3] if start_ms else "?", {k: v for k, v in other.items() if k in ("bazel_version", "output_base", "build_id")})
rows = [e for e in data["traceEvents"] if e.get("ph") == "X" and e.get("dur", 0) / 1000 >= floor]
for e in sorted(rows, key=lambda e: e["ts"]):
    print("%9.3f +%8.3f  %-28s %-40s %s" % (e["ts"] / 1e6, e["dur"] / 1e6, (e.get("cat") or "")[:28], e["name"][:40], names.get(e["tid"], e["tid"])[:30]))
