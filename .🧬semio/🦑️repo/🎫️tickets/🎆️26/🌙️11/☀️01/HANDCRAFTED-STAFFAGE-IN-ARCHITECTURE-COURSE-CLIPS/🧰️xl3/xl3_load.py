"""Time loading the largest stored order. Usage: xl3_load.py FILE"""
import importlib
import json
import os
import sys
import time

sys.path.insert(0, os.getcwd())
V = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
began = time.time()
data = json.loads(open(sys.argv[1], encoding="utf-8").read())
recipes = V._order_recipes(data["order"])
print(f"[load] rows={len(recipes)} fragments={sum(1 for r in recipes if not isinstance(r, int))} sources_match={data['sources'] == V._source_digest()} t={time.time() - began:.1f}s", flush=True)
