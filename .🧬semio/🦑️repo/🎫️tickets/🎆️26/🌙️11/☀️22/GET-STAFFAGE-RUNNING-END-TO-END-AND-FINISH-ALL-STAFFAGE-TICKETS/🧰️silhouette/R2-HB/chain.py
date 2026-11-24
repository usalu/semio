"""⛓️ Hooks the ops of `worker-with-pallet-jack` into its generator chain: the ops of `ops/worker-with-pallet-jack.json` become the last ops of the repair file of the chain (`STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-18/repair/worker-with-pallet-jack.json`, one op per line as the file is written), replacing an earlier copy of the same op target, so that a re-run of the chain ends in this repair. `python3 chain.py`"""
import json
import os

import lib

SLUG = "worker-with-pallet-jack"
REPAIR = os.path.join(lib.ROOT, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️11", "☀️22", "STAFFAGE-HUMAN-COLLECTION-WAVE", "🧰️scripts", "HW-18", "repair", f"{SLUG}.json")


def target(op):
    """🎯️ What an op names: kind of thing, scale, view or list, id."""
    return (op["op"].split("-")[0], op.get("scale"), op.get("view", op.get("list")), op.get("id") or (op.get("shape") or op.get("solid") or {}).get("id"), json.dumps(op.get("path")))


before = open(REPAIR, encoding="utf8").read()
chain = json.loads(before)
mine = json.load(open(os.path.join(lib.HERE, "ops", f"{SLUG}.json"), encoding="utf8"))
names = {target(op) for op in mine if op["op"] == "shape-add"}
kept = [op for op in chain if not (op["op"] == "shape-add" and target(op) in names and op in mine)]
kept = [op for op in kept if op not in mine]
if open(REPAIR, encoding="utf8").read() != before:
    raise SystemExit("the repair file changed on disk since it was read: nothing written, run again")
with open(REPAIR, "w", encoding="utf8") as handle:
    handle.write("[\n" + ",\n".join(json.dumps(op, ensure_ascii=False) for op in kept + mine) + "\n]\n")
print(f"{REPAIR}: {len(kept)} ops of the chain, {len(mine)} of this round at the end")
