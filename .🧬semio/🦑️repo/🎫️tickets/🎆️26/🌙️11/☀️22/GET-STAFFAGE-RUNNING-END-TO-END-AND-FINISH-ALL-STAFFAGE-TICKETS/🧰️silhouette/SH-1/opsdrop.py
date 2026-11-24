"""🧹️ Takes the ops of one scale and view (or of one whole scale) of a person out of ops/<slug>.json, so that the view starts again from the file of the session start: python3 opsdrop.py <slug> <scale> [view]"""
import json, os, sys

slug, scale, view = sys.argv[1], sys.argv[2], (sys.argv[3] if len(sys.argv) > 3 else None)
path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "ops", f"{slug}.json")
ops = json.load(open(path, encoding="utf-8"))
kept = [o for o in ops if not (o.get("scale") == scale and str(o["op"]).startswith("shape") and (view is None or o.get("view") == view))]
open(path, "w", encoding="utf-8").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in kept) + "\n]\n")
print(f"{slug}: {len(ops) - len(kept)} ops of 1:{scale} {view or 'all views'} dropped, {len(kept)} kept")
