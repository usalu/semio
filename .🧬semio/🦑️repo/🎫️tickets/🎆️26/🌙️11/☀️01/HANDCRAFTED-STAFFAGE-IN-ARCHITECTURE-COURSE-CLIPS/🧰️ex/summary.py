import json
from pathlib import Path

T = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS")
r = json.loads((T / "🧰️ex/results.json").read_text(encoding="utf8"))
ok = [k for k, v in r.items() if v["ok"]]
bad = [k for k, v in r.items() if not v["ok"]]
print("total", len(r), "ok", len(ok), "failed", bad)
print("frames total", sum(r[k]["frames"] for k in ok), "wall sum", round(sum(r[k]["wall"] for k in ok), 1))
print("batch peaks", sorted({v["batchPeakMB"] for v in r.values()}, key=int)[-3:])
print("slow >60s:", [(k, r[k]["wall"]) for k in ok if r[k]["wall"] > 60])
print("top wall:", sorted(((r[k]["wall"], k) for k in ok), reverse=True)[:8])
print("600 cap:", [(k, r[k]["frames"], r[k]["fps"]) for k in ok if r[k]["frames"] >= 600])
print("fractional fps:", [(k, r[k]["frames"], r[k]["fps"]) for k in ok if r[k]["fps"] != int(r[k]["fps"])])
