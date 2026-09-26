"""⏱️ F2 — per-10 s timeline of one origin from a Chromium NetLog: requests started, queue time p50/max of the requests sent in
the bucket, and the number of long-lived streams (SSE / on the wire ≥ 10 s) holding a connection at the bucket start.
usage: python3 f2-netlog-timeline.py <netlog.json> <origin> [bucket_s]"""
import json, sys, urllib.parse, collections
text = open(sys.argv[1], encoding="utf-8").read().rstrip()
if not text.endswith("}"):
    text = text.rstrip(",\n ") + "]}"
log = json.loads(text)
origin = sys.argv[2]
bucket = int(sys.argv[3]) * 1000 if len(sys.argv) > 3 else 10_000
types = {v: k for k, v in log["constants"]["logEventTypes"].items()}
phases = {v: k for k, v in log["constants"]["logEventPhase"].items()}
stypes = {v: k for k, v in log["constants"]["logSourceType"].items()}
rows = {}
t_last = 0
for e in log["events"]:
    t = int(e["time"]); t_last = max(t_last, t)
    if stypes.get(e["source"]["type"]) != "URL_REQUEST": continue
    k = types.get(e["type"]); ph = phases.get(e.get("phase"), "")
    r = rows.setdefault(e["source"]["id"], {"url": None, "start": None, "sent": None, "end": None})
    p = e.get("params", {})
    if k == "URL_REQUEST_START_JOB" and "url" in p and r["url"] is None: r["url"] = p["url"]; r["start"] = r["start"] or t
    if k == "REQUEST_ALIVE" and ph == "PHASE_BEGIN": r["start"] = t
    if k == "HTTP_TRANSACTION_SEND_REQUEST" and ph == "PHASE_BEGIN" and r["sent"] is None: r["sent"] = t
    if k == "REQUEST_ALIVE" and ph == "PHASE_END": r["end"] = t
mine = [r for r in rows.values() if r["url"] and r["url"].startswith(origin) and r["start"] is not None]
t0 = min(r["start"] for r in mine)
longs = [r for r in mine if r["sent"] is not None and ((r["end"] or t_last) - r["sent"]) >= 10_000]
buckets = collections.defaultdict(list)
for r in mine:
    if r["sent"] is not None: buckets[(r["sent"] - t0) // bucket].append(r["sent"] - r["start"])
print(f"origin {origin}: {len(mine)} requests over {(t_last - t0) / 1000:.0f} s; long-lived {len(longs)}")
for b in sorted(buckets):
    q = sorted(buckets[b]); at = t0 + b * bucket
    held = sum(1 for r in longs if r["sent"] <= at < (r["end"] or t_last))
    print(f"{b * bucket // 1000:5d}s sent={len(q):4d} queue p50={q[len(q) // 2]:6d} max={q[-1]:6d} ms  long-lived holding={held}")
