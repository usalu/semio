"""🔁️ Replaces, in `ops/<id>.json`, every `plan-revolve` whose dry run (the output of fit.ts on stdin) shows a body of revolution that cannot hold both the side thickness and the front width of the part (radius scaled by more than 12 % or a footprint mismatch above 15 mm) by the prism of the front twin: `front-sweep` of the shape, then `copy` of it from the front to the plan. `python3 swap.py <id> < fit-output`; prints what it swapped."""
import json, re, sys
aid = sys.argv[1]
bad = set()
for line in sys.stdin:
    m = re.match(r"\S+ plan-revolve 1:(\d+) (\S+): fitted: .*radius × ([\d.]+), footprint mismatch side ([\d.]+) mm, front ([\d.]+) mm", line)
    if m and (abs(float(m[3]) - 1) > 0.12 or max(float(m[4]), float(m[5])) > 15): bad.add((m[1], m[2]))
path = f"ops/{aid}.json"
ops, out = json.load(open(path)), []
for op in ops:
    if op["op"] == "plan-revolve" and (op["scale"], op["id"]) in bad:
        sweep = {"op": "front-sweep", "scale": op["scale"], "id": op["id"]}
        if sweep not in ops and sweep not in out: out.append(sweep)
        out.append({"op": "copy", "scale": op["scale"], "id": op["id"], "from": "front", "to": ["plan"]})
    else: out.append(op)
open(path, "w").write("[\n" + ",\n".join("  " + json.dumps(o, ensure_ascii=False) for o in out) + "\n]\n")
print("swapped", sorted(bad))
