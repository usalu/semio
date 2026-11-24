"""🧩️ Joins op lists into the ops file of an asset: `python3 compose.py <out.json> <list.json|-> …` concatenates the JSON arrays (a `-` reads one from stdin), drops exact repeats and writes one op per line."""
import json, sys
ops, seen = [], set()
for source in sys.argv[2:]:
    for op in json.load(sys.stdin if source == "-" else open(source)):
        key = json.dumps(op, sort_keys=True)
        if key not in seen:
            seen.add(key)
            ops.append(op)
open(sys.argv[1], "w").write("[\n" + ",\n".join("  " + json.dumps(op) for op in ops) + "\n]\n")
print(sys.argv[1].split("/")[-1], len(ops), "ops")
