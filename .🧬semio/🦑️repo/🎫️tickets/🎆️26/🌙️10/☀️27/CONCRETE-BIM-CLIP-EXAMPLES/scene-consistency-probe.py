"""🔬 Probe the model for the consistency scene: roof slab, links, events, tributary."""
import importlib
import json
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
mod = m.load_model()
print(list(mod.keys()))
for e in mod["elements"]:
    if e["class"] == "IfcSlab":
        print(json.dumps(e, ensure_ascii=False)[:1800])
print("LINKS", json.dumps(mod["links"], ensure_ascii=False))
print("EVENTS", json.dumps(mod.get("demo_events"), ensure_ascii=False))
print("MODELS", json.dumps(mod["models"], ensure_ascii=False)[:1500])
print("LOADS", json.dumps(mod["loads"], ensure_ascii=False)[:1500])
print(mod["revision"], mod["schema_id"])
print("trib", hasattr(m, "tributary"), "translate", hasattr(m, "translate_structure"), "ids", hasattr(m, "ids_check"))
if hasattr(m, "tributary"):
    t = m.tributary(mod)
    print(json.dumps({k: v for k, v in t.items() if k != "pilotis"}, ensure_ascii=False)[:600])
    print(json.dumps(t["pilotis"][0], ensure_ascii=False)[:600])
event = m.demo_event("roof_soil", mod)
new = m.apply_event(event, mod)
print("EVENT", event, new["revision"])
print("ISSUES", m.consistency_issues(new))
print("ISSUES0", m.consistency_issues(mod))
