"""➕️ Appends ops to the ops file of an asset: `python3 addops.py <id> '<ops json>' [why]` drops `measure` ops, adds the reason as `why` to the first appended op and skips ops that are in the file already."""
import json, os, sys
path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "ops", sys.argv[1] + ".json")
ops = json.load(open(path)) if os.path.exists(path) else []
new = [op for op in json.loads(sys.argv[2]) if op["op"] != "measure"]
if len(sys.argv) > 3 and new: new[0] = {**new[0], "why": sys.argv[3]}
strip = lambda op: {k: v for k, v in op.items() if k != "why"}
have = [strip(op) for op in ops]
added = [op for op in new if strip(op) not in have]
json.dump(ops + added, open(path, "w"), indent=1, ensure_ascii=False)
print(f"{sys.argv[1]}: {len(ops)} ops + {len(added)}")
