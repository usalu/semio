"""🧹️ Takes the ops of single drawn shapes (one scale, one view, the ids) out of ops/<slug>.json, so that these shapes are drawn again as the file of the session start draws them; an added shape disappears: python3 shapedrop.py <slug> <scale> <view> <id>…"""
import json, os, sys

slug, scale, view, ids = sys.argv[1], sys.argv[2], sys.argv[3], set(sys.argv[4:])
path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "ops", f"{slug}.json")
ops = json.load(open(path, encoding="utf-8"))
kept = [o for o in ops if not (o.get("scale") == scale and o.get("view") == view and str(o["op"]).startswith("shape") and (o.get("id") or (o.get("shape") or {}).get("id")) in ids)]
open(path, "w", encoding="utf-8").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in kept) + "\n]\n")
print(f"{slug}: {len(ops) - len(kept)} ops of 1:{scale} {view} {' '.join(sorted(ids))} dropped, {len(kept)} kept")
