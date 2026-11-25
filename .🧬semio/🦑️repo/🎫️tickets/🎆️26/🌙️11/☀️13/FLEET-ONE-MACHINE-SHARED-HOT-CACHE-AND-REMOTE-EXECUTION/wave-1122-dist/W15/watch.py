"""👀️ W15 watch: one line per worker from its origin mirrors (ready.json, slice-memory.json, memory.json): ready, stage, slice phase and done, slice report rss/guard, cap cancellations of slice workers since argv[1] (epoch ms); appends the raw sample to argv[2]."""
import json, glob, os, sys, time
since, out, rows = int(sys.argv[1]), sys.argv[2], []
for o in sorted(glob.glob("/Users/Shared/semio-store/v1/origins/g1-m*")):
    w = os.path.basename(o)[3:]
    def load(n):
        try: return json.load(open(f"{o}/{n}"))
        except Exception: return {}
    r, s, m = load("ready.json"), load("slice-memory.json"), load("memory.json")
    al = [a for a in (m.get("lastAlarms") or []) if a.get("action") == "cancel" and a.get("atMs", 0) >= since]
    fresh = s.get("atMs", 0) >= since
    row = {"w": w, "t": int(time.time()), "ready": r.get("ready"), "stage": r.get("stage"), "phase": (s.get("phase") or {}).get("name", "")[:48] if fresh else "-", "done": (s.get("phase") or {}).get("done") if fresh else None,
           "rss": (s.get("processes") or {}).get("self", {}).get("rssMegabytes") if fresh else None, "guard": (s.get("guard") or {}).get("megabytes") if fresh else None, "up": s.get("uptimeSeconds") if fresh else None,
           "cancels": [(time.strftime("%H:%M", time.localtime(a["atMs"] / 1000)), a["kind"], a["reason"], a["megabytes"]) for a in al]}
    rows.append(row)
with open(out, "a") as f:
    for row in rows: f.write(json.dumps(row) + "\n")
print(time.strftime("%H:%M"), "ready:", [r["w"] for r in rows if r["ready"]])
for r in rows:
    if r["ready"] and r["phase"] == "-": continue
    print(f' {r["w"]} st={r["stage"]} up={r["up"]} rss={r["rss"]} guard={r["guard"]} {r["phase"]} {r["done"]} cancels={r["cancels"]}')
