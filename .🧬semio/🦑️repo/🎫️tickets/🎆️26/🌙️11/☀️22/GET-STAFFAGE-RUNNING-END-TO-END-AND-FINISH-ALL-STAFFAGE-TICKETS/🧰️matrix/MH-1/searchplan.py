"""🧭️ Writes the second plan of an asset for `solve.ts`: `python3 searchplan.py <id> <scale>:<shape id>[:<view>+<view>,<view>…]…` keeps the ops already settled in `ops/<id>.json` as fixed entries (applied without a measurement) and adds one `search` entry per argument (the solid of the volume in every subset of the views, or in the subsets stated, kept only with fewer hard findings of the matrix); the plan it replaces is kept once as `plan/<id>.auto.json`."""
import json, os, shutil, sys
asset, *entries = sys.argv[1:]
here = os.path.dirname(os.path.abspath(__file__))
plan, first = f"{here}/plan/{asset}.json", f"{here}/plan/{asset}.auto.json"
if os.path.exists(plan) and not os.path.exists(first): shutil.copy(plan, first)
out = [{**op, "fixed": True} for op in json.load(open(f"{here}/ops/{asset}.json"))]
for entry in entries:
    scale, shape, *rest = entry.split(":")
    out.append({"search": True, "scale": scale, "id": shape, **({"subsets": [subset.split("+") for subset in rest[0].split(",")]} if rest else {})})
open(plan, "w").write("[\n" + ",\n".join("  " + json.dumps(o) for o in out) + "\n]\n")
print(f"{asset}: {len(out)} entries, {len(entries)} searches")
