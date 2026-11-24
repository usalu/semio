"""✍️ Merges hand-written repair ops into the ops file of a vehicle (an op that names the same target replaces the earlier one in place, others are appended): hand.py <id> < json list of ops on stdin."""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))


def signature(o):
    """🔑️ The target an op names."""
    return json.dumps([o["op"].replace("-add", ""), o.get("scale"), o.get("view"), o.get("list"), o.get("id") or (o.get("shape") or o.get("solid") or {}).get("id"), o.get("path"), sorted(o["set"]) if "set" in o else None])


slug = sys.argv[1]
path = f"{HERE}/ops/{slug}.json"
ops = json.load(open(path)) if os.path.exists(path) else []
for o in json.load(sys.stdin):
    hit = [i for i, x in enumerate(ops) if signature(x) == signature(o)]
    if hit:
        ops[hit[0]] = o
    else:
        ops.append(o)
open(path, "w").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops) + "\n]\n")
print(f"{slug}: {len(ops)} ops")
