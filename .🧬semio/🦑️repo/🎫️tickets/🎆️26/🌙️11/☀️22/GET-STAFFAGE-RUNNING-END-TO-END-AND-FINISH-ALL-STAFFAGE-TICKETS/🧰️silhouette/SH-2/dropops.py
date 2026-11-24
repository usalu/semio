"""✂️ Takes repair ops back out of `ops/<slug>.json`: every op whose index (as listed, zero-based) is named, singly or as a range `a-b`. The catalog file is then rebuilt with `rebuild.sh`. `python3 dropops.py <slug> <index|a-b>…`"""
import json
import os
import sys

slug = sys.argv[1]
path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "ops", f"{slug}.json")
ops = json.load(open(path, encoding="utf8"))
gone = set()
for spec in sys.argv[2:]:
    low, _, high = spec.partition("-")
    gone.update(range(int(low), int(high or low) + 1))
kept = [op for index, op in enumerate(ops) if index not in gone]
with open(path, "w", encoding="utf8") as handle:
    handle.write("[\n" + ",\n".join(json.dumps(op, ensure_ascii=False, separators=(",", ":")) for op in kept) + "\n]\n")
print(f"{slug}: {len(ops) - len(kept)} ops taken back, {len(kept)} left")
