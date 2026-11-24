"""➕️ Adds hand-written repair ops to the repair ops of a person of SH-2 (`ops/<slug>.json` here, or the repair file of its generator chain as `people.ts` names it): a `dims` op is merged into the one at the head, an op that names the same target as an earlier one (same op, scale, view, list, id and, for the `-set` ops, the same keys) replaces it in place, every other op is appended. Written one op per line like `ops.ts` writes them. `python3 addops.py <slug> <new-ops.json>`"""
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
WAVE = os.path.normpath(os.path.join(HERE, "..", "..", "..", "STAFFAGE-HUMAN-COLLECTION-WAVE", "🧰️scripts"))


def ops_file(slug):
    """📁️ The repair ops file of a person, as `people.ts` resolves it."""
    text = open(os.path.join(HERE, "people.ts"), encoding="utf8").read()
    worker = re.search(rf'"{re.escape(slug)}": "(HW-1\d)"', text)
    if slug == "tennis-player":
        return os.path.join(WAVE, "REP-A", "ops", "tennis-player.80-silhouette.json")
    return os.path.join(WAVE, worker.group(1), "repair", f"{slug}.json") if worker else os.path.join(HERE, "ops", f"{slug}.json")


def signature(op):
    """🔖️ What an op targets."""
    ident = op.get("id") or (op.get("shape") or {}).get("id") or (op.get("solid") or {}).get("id")
    keys = sorted(op["set"]) if op["op"].endswith("-set") else None
    return json.dumps([op["op"].replace("-add", ""), op.get("scale"), op.get("view"), op.get("list"), ident, op.get("path"), keys])


slug, new_file = sys.argv[1:3]
path = ops_file(slug)
ops = json.load(open(path, encoding="utf8")) if os.path.exists(path) else []
added = replaced = 0
for op in json.load(open(new_file, encoding="utf8")):
    if op["op"] == "dims":
        head = next((o for o in ops if o["op"] == "dims"), None)
        if head is None:
            ops.insert(0, op)
        else:
            head["set"].update(op["set"])
        replaced += 1
        continue
    at = next((i for i, o in enumerate(ops) if signature(o) == signature(op)), -1)
    if at < 0:
        ops.append(op)
        added += 1
    else:
        ops[at] = op
        replaced += 1
with open(path, "w", encoding="utf8") as handle:
    handle.write("[\n" + ",\n".join(json.dumps(op, ensure_ascii=False, separators=(",", ":")) for op in ops) + "\n]\n")
print(f"{slug}: {added} ops added, {replaced} merged or replaced, {len(ops)} in {os.path.relpath(path, HERE)}")
