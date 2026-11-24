"""➕️ Appends hand-written ops (JSON objects, one per argument) to ops/<slug>.json; an op with the same op, scale, view and id replaces the earlier one, so the call is idempotent: python3 opsadd.py <slug> '<op json>'…"""
import json, os, sys

slug = sys.argv[1]
path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "ops", f"{slug}.json")
ops = json.load(open(path, encoding="utf-8")) if os.path.exists(path) else []
key = lambda o: (o["op"], o.get("scale"), o.get("view"), o.get("list"), o.get("id") or (o.get("shape") or o.get("solid") or {}).get("id"), o.get("hand"))
for text in sys.argv[2:]:
    new = {**json.loads(text), "hand": True}
    ops = [o for o in ops if key(o) != key(new)] + [new]
open(path, "w", encoding="utf-8").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops) + "\n]\n")
print(f"{slug}: {len(sys.argv) - 2} ops added, {len(ops)} in all")
