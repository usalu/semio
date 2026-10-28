"""Streams the unit-cache event logs of a time window: per lane and kind counts and milliseconds, the slowest crates, misses by crate (rustedit-g).

usage: events.py <startEpochMs> <endEpochMs> [--lane ID]   (streams, never keeps a log in memory)
"""
import collections, glob, json, os, sys

X = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/x"
start, end = int(sys.argv[1]), int(sys.argv[2])
only = sys.argv[sys.argv.index("--lane") + 1] if "--lane" in sys.argv else None
count, ms, wait = collections.Counter(), collections.Counter(), collections.Counter()
rows = []
for path in sorted(glob.glob(os.path.join(X, "*.jsonl"))):
    lane = os.path.basename(path)[:-6]
    if only and lane != only:
        continue
    if os.path.getmtime(path) * 1000 < start:
        continue
    with open(path, encoding="utf8") as handle:
        for line in handle:
            try:
                row = json.loads(line)
            except ValueError:
                continue
            if not start <= row["atMs"] <= end:
                continue
            key = (lane, row["kind"])
            count[key] += 1
            ms[key] += row["milliseconds"]
            wait[key] += row.get("waitedMs", 0)
            if row["kind"] in ("miss", "incremental", "bypass", "script-miss", "failed", "slot-wait", "lock-wait", "hit", "script-hit"):
                rows.append((row["milliseconds"], lane, row["kind"], row["crate"], row["atMs"], row.get("waitedMs", 0), row.get("reason", "")))
for (lane, kind), n in sorted(count.items()):
    print("%-8s %-14s n=%-5d sum=%8.1fs waited=%7.1fs" % (lane, kind, n, ms[(lane, kind)] / 1000, wait[(lane, kind)] / 1000))
print("-- non-trivial events (miss, incremental, bypass, failed, waits), slowest first")
for row in sorted((r for r in rows if r[2] not in ("hit", "script-hit")), reverse=True)[:25]:
    print("%7.1fs %-8s %-12s %-50s at+%s waited=%.1fs %s" % (row[0] / 1000, row[1], row[2], row[3], row[4] - start, row[5] / 1000, row[6]))
print("-- hits: %d, hit crates sum %.1fs" % (sum(1 for r in rows if r[2] in ("hit", "script-hit")), sum(r[0] for r in rows if r[2] in ("hit", "script-hit")) / 1000))
print("-- misses by crate:", ", ".join(sorted({r[3] for r in rows if r[2] == "miss"}))[:1500])
