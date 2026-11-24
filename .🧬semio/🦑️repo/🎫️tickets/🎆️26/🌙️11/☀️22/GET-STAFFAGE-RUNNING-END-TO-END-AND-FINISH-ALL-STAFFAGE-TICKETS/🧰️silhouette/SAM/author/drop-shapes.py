"""✂️ Takes shape ops out of `ops/<id>.json` again whose shape id starts with one of the given prefixes (a part that patch.ts drew on a bone whose plane a pose turns, which the validator refuses as a flat shape): drop-shapes.py <id> <prefix>…"""
import json, os, sys

path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "ops", f"{sys.argv[1]}.json")
ops = json.load(open(path))
kept = [o for o in ops if not (o["op"].startswith("shape") and any(str(o.get("id") or o.get("shape", {}).get("id")).startswith(p) for p in sys.argv[2:]))]
open(path, "w").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in kept) + "\n]\n")
print(f"{sys.argv[1]}: {len(ops) - len(kept)} ops dropped")
