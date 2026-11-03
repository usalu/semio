"""🚗 Lists shapes without region per representation and the thermal region ids of the vehicles."""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[8] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles"
for name in sys.argv[1:]:
    doc = json.loads((ROOT / name / "🔣️.json").read_text(encoding="utf-8"))
    a = doc["asset"]
    print("==", name, [(r["id"], r["model"], r.get("kind")) for r in a["thermal"]["regions"]])
    for key, rep in a["representations"].items():
        if rep.get("status") != "authored":
            continue
        for lst in ("elevation", "plan", "section"):
            for sh in rep["geometry"].get(lst) or []:
                if not (sh.get("region") or sh.get("regions")):
                    print(key, lst, sh["id"], sh.get("bone"), sh.get("role"), sh.get("filled"), sh.get("material"))
