"""QA9: details of front and plan views get a higher z than the filled body of their bone and region (same-region merge rule). Atomic write."""
import json, os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧰️FW"))
from atomic_json import write_json_atomic

B = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🪑️objects"
for a in ("patio-heater", "bicycle-stand"):
    p = os.path.join(B, a, "🔣️.json")
    doc = json.load(open(p, encoding="utf-8"))
    changed = 0
    for rep in doc["asset"]["representations"].values():
        if rep["status"] != "authored":
            continue
        for vp in ("front", "plan"):
            shapes = rep["geometry"][vp]
            for u in shapes:
                if u.get("filled") or u.get("role") != "detail":
                    continue
                regs = set(u.get("regions") or ([u["region"]] if u.get("region") else []))
                tops = [f["z"] for f in shapes if f is not u and f.get("filled") and f["bone"] == u["bone"] and regs & set(f.get("regions") or ([f["region"]] if f.get("region") else [])) and f["z"] >= u["z"]]
                if tops:
                    u["z"] = round(max(tops) + 0.05, 3)
                    changed += 1
    write_json_atomic(p, doc)
    print(a, changed)
