"""🧲️ Keeps, of the proposals of `suggest.ts` for an asset, the ops of the bones the gate log reports as hard at that scale, and only for the view the reported axis names (height: front; forward: plan; lateral: both): `python3 pick.py <id> <gate.log> <suggest.json> [--drop-review]` prints the ops (each with its bone) as JSON; the fixer edits the result into `ops/<id>.json`."""
import json, re, sys, glob
aid, log, sug = sys.argv[1:4]
a = json.load(open(glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{aid}/🔣️.json")[0]))["asset"]
hard, scale, on, axis = {}, None, False, None
for line in open(log):
    m = re.match(r"\S+ 1:(\d+) (HARD|reported|twin) (\S+)", line)
    if m: scale, on, axis = m[1], m[2] == "HARD" and m[3].startswith("sync-bone"), m[3].rsplit("-", 1)[-1]; continue
    if on and line.startswith("    "): hard.setdefault((scale, line.split()[0]), set()).update({"height": ["front"], "forward": ["plan"], "lateral": ["front", "plan"]}[axis])
    elif not line.startswith("    "): on = False
out = []
for op in json.load(open(sug)):
    bone = op.get("bone") or next(s["bone"] for s in a["representations"][op["scale"]]["geometry"]["elevation"] if s["id"] == op["id"])
    views = hard.get((op["scale"], bone), set())
    target = [v for v in op["to"] if v in views] if op["op"] == "copy" else [op.get("view") or ("plan" if op["op"] == "plan-revolve" else "front")]
    if op["op"] == "copy": op["to"] = target
    if target and target[0] in views and not ("--drop-review" in sys.argv and op.get("review")):
        op.pop("why", None); out.append({**op, "_bone": bone})
covered = {(o["scale"], o["_bone"]) for o in out}
print("[\n" + ",\n".join("  " + json.dumps(o) for o in out) + "\n]")
print("no proposal for:", sorted(set(hard) - covered), file=sys.stderr)
