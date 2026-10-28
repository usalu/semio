"""[gram4-g] prints a thinned time series of the tick/probe lines of a harness log and the maxima. usage: ticks.py <log> [every]"""
import sys, json, re
rows = []
for line in open(sys.argv[1], encoding="utf8", errors="replace"):
    m = re.search(r'(\{"t":\d+,"tag":"[^"]+".*?\})(?:$|\s)', line)
    if m:
        try: rows.append(json.loads(m.group(1)))
        except Exception: pass
every = int(sys.argv[2]) if len(sys.argv) > 2 else max(1, len(rows) // 40)
for r in rows[::every]:
    print(r["t"], r["tag"], "rss", r["rss"], "heap", r["heap"], "extra", r["extra"], "sum", r["heap"] + r["extra"], "objs", r["objs"], "mi", r["mi"], "miReq", r.get("miReq", "-"), "miHuge", r.get("miHuge", "-"), "ext", r.get("ext", "-"), "ab", r.get("ab", "-"), "cap", r.get("cap", "-"), "miN", r.get("miN", "-"), "miCommit", r.get("miCommit", "-"))
if not rows: sys.exit("no ticks")
print("n", len(rows), "max heap+extra", max(r["heap"] + r["extra"] for r in rows), "max rss", max(r["rss"] for r in rows), "max mi", max(r["mi"] for r in rows))
