import json, sys
from pathlib import Path
R = Path("C:/git/semio")
T = next((R / ".🧬semio/🦑️repo/🎫️tickets").glob("*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"))
last = {}
for l in (T / "🗑️generated/sm/results.jsonl").read_text(encoding="utf-8").splitlines():
    d = json.loads(l)
    k = (d["file"].replace("\\", "/").split("architecture/")[-1], d["scene"])
    if d["status"] == "start" and k in last and last[k]["status"] != "start":
        continue
    last[k] = d
only = sys.argv[1] if len(sys.argv) > 1 else None
from collections import Counter
c = Counter(d["status"] for d in last.values())
print(dict(c), "files", len({k[0] for k in last}))
for k, d in last.items():
    if d["status"] in ("fail", "timeout", "start", "noscene") or d["status"] == "skipped":
        print(d["status"], k[1], "|", k[0], "|", d.get("head", "")[:200])
