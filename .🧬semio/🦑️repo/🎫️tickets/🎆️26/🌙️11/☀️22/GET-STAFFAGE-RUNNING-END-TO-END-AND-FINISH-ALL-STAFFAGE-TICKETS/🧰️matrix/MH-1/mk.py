"""🧱️ Writes `ops/<id>.json` from the picked proposals (pick.py on stdin) and hand-made ops appended from a JSON file: `python3 pick.py … | python3 mk.py <id> [<extra.json>]`; the `_bone` and `review` marks are dropped, an op already present is not repeated; an entry `{ "drop": {…} }` of the extra file removes the picked ops whose fields all match."""
import json, sys
ops = json.load(sys.stdin)
extra = json.load(open(sys.argv[2])) if len(sys.argv) > 2 else []
for rule in (entry["drop"] for entry in extra if "drop" in entry): ops = [op for op in ops if any(op.get(k) != v for k, v in rule.items())]
ops += [entry for entry in extra if "drop" not in entry]
seen, out = set(), []
for op in ops:
    op.pop("_bone", None); op.pop("review", None); op.pop("why", None)
    key = json.dumps(op, sort_keys=True)
    if key not in seen: seen.add(key); out.append(op)
open(f"ops/{sys.argv[1]}.json", "w").write("[\n" + ",\n".join("  " + json.dumps(o, ensure_ascii=False) for o in out) + "\n]\n")
print(len(out), "ops")
