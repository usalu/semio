"""📐️ Declares dimensions the body has: keeps one `dims` op at the head of the repair ops `ops/<slug>.json` of a person of SH-2 (written one op per line like `ops.ts` writes them). `python3 dims.py <slug> widthM=0.874 heightM=1.783 …`"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
slug, pairs = sys.argv[1], dict(pair.split("=") for pair in sys.argv[2:])
path = os.path.join(HERE, "ops", f"{slug}.json")
ops = json.load(open(path, encoding="utf8")) if os.path.exists(path) else []
before = {}
for op in ops:
    if op["op"] == "dims":
        before.update(op["set"])
ops = [{"op": "dims", "set": {**before, **{key: float(value) for key, value in pairs.items()}}}] + [op for op in ops if op["op"] != "dims"]
with open(path, "w", encoding="utf8") as handle:
    handle.write("[\n" + ",\n".join(json.dumps(op, ensure_ascii=False, separators=(",", ":")) for op in ops) + "\n]\n")
print(f"{slug}: declared {ops[0]['set']}")
