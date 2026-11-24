"""🧹️ Takes one group of ops (the plates or prisms of one bone in one view: `plate|<view>|<bone>` or `prism|<view>|<bone>`) of one scale out of ops/<slug>.json, so that the bone is drawn again as the file of the session start draws it: python3 groupdrop.py <slug> <scale> <group>…"""
import json, os, sys

slug, scale, groups = sys.argv[1], sys.argv[2], set(sys.argv[3:])
path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "ops", f"{slug}.json")
ops = json.load(open(path, encoding="utf-8"))
kept = [o for o in ops if not (o.get("scale") == scale and o.get("group") in groups)]
open(path, "w", encoding="utf-8").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in kept) + "\n]\n")
print(f"{slug}: {len(ops) - len(kept)} ops of 1:{scale} {' '.join(sorted(groups))} dropped, {len(kept)} kept")
