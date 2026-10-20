#!/usr/bin/env python3
"""🧱️ Builds staged/plan.json from the live tree (read only): every group in order. Re-run after any drift."""
import importlib, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import blib
for name in sys.argv[1:] or ["s_plans", "s_leftovers", "s_code", "s_guard", "s_agent_client", "s_extension", "s_manifests"]:
    if os.path.isfile(os.path.join(blib.HERE, name + ".py")): importlib.import_module(name)
    else: print("skip", name)
out = os.path.join(blib.HERE, "staged", "plan.json")
json.dump(blib.PLAN, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({k: len(v) for k, v in blib.PLAN.items() if isinstance(v, list)}))
