"""🧷️ Joins op lists into `ops/<id>.json` in the order given: join.py <id> <ops.json>…"""
import json, sys, os
out = []
for path in sys.argv[2:]:
    out += json.load(open(path))
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "ops", sys.argv[1] + ".json"), "w"), ensure_ascii=False, indent=1)
print(sys.argv[1], len(out), "ops")
