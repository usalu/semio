"""🧗️ Chooses, for a walker, the ops the silhouette guard accepts: `python3 greedy.py <id> > ops/<id>.json` (run with the environment of `../MR/env.sh`, BATCH=MH-2). The candidates are the ops of `walker-ops.py` (the plan of a hand as `extrude` instead of a body of revolution), in groups per scale: legs, shoes front, shoes plan, forearms front, hands front, sleeves front, sleeves plan, forearms plan, hands plan, and where that is refused the hands plan as the prism of the front (`copy` from the front). `--base <ops.json>` starts each scale from ops already chosen and `--only <group,…>` tries only those groups. Per scale the groups are added one after the other onto the backup; a group stays when the silhouette guard of that scale (`../MR/measure.ts`) still has no failing check and no shape fails to evaluate, else it is dropped and named on stderr with the values that refused it. One tool process runs at a time."""
import json, os, re, subprocess, sys
a = sys.argv[1]
B, S, T, W, M, GEN = (os.environ[k] for k in ("B", "S", "T", "W", "M", "GEN"))
backup = f"{GEN}/backup/{a}.json"
doc = json.load(open(backup))["asset"]
cand = json.loads(subprocess.run(["python3", f"{W}/walker-ops.py", backup], capture_output=True, text=True, check=True).stdout)
def bone(op):
    geo = doc["representations"][op["scale"]]["geometry"]
    return next(s["bone"] for s in geo["elevation"] if s["id"] == op["id"])
def group(op):
    b = re.sub(r"-[lr]$", "", bone(op)); view = "front" if op["op"] == "front-sweep" or op.get("view") == "front" else "plan"
    if b in ("thigh", "shank"): return "legs"
    if b in ("foot", "toe"): return "shoes " + view
    return {"arm": "sleeves", "upper-arm": "sleeves", "forearm": "forearms", "hand": "hands"}[b] + " " + view
for op in cand:
    if group(op) == "hands plan": op.clear() if False else op.update({"op": "extrude", "view": "plan"}) or [op.pop(k, None) for k in ("radius", "rebuild")]
ORDER = ["legs", "shoes front", "shoes plan", "forearms front", "hands front", "sleeves front", "sleeves plan", "forearms plan", "hands plan", "hands plan prism"]
for op in [dict(o) for o in cand if group(o) == "hands plan"]: cand.append({"op": "copy", "scale": op["scale"], "id": op["id"], "from": "front", "to": ["plan"], "alt": True})
_group = group
def group(op): return "hands plan prism" if op.get("alt") else _group(op)
base = json.load(open(sys.argv[sys.argv.index("--base") + 1])) if "--base" in sys.argv else []
only = sys.argv[sys.argv.index("--only") + 1].split(",") if "--only" in sys.argv else None
trial, ops_file = f"{GEN}/try/{a}.greedy.json", f"{GEN}/try/{a}.greedy.ops.json"
def measure(ops, scale):
    subprocess.run(["cp", backup, trial], check=True)
    json.dump(ops, open(ops_file, "w"))
    for _ in range(3):
        fit = subprocess.run([f"{W}/q.sh", "1000", "600", T, B, f"{W}/fit2.ts", trial, "--ops", ops_file, "--out", trial], cwd=T, capture_output=True, text=True)
        if " 0 changed something" in fit.stdout: break
    if "REFUSED" in fit.stdout: return "refused: " + next(l for l in fit.stdout.splitlines() if "REFUSED" in l)[:160]
    out = subprocess.run([f"{W}/q.sh", "2500", "900", f"{S}/📦️packages/🟦️typescript", B, f"{M}/measure.ts", trial, "--scales", scale], cwd=f"{S}/📦️packages/🟦️typescript", capture_output=True, text=True)
    line = next((l for l in out.stdout.splitlines() if "bbox" in l), None)
    if line is None: return "no measure: " + (out.stderr.strip().splitlines() or ["?"])[-3][:160]
    return line.split("  ", 1)[1] if "!" in line else None
kept = []
for scale in [k for k, r in doc["representations"].items() if r.get("status") == "authored"]:
    mine = [op for op in base if op.get("scale") == scale]
    done = set()
    for g in ORDER:
        add = [op for op in cand if op["scale"] == scale and group(op) == g]
        if not add or (only is not None and g not in only): continue
        if g == "hands plan prism" and ("hands plan" in done or not any(o["op"] == "front-sweep" and o["id"] == add[0]["id"] for o in mine)): continue
        why = measure(mine + add, scale)
        if why is None: mine += add; done.add(g); print(f"[DEBUG] 1:{scale} {g}: kept ({len(add)} ops)", file=sys.stderr, flush=True)
        else: print(f"1:{scale} {g}: dropped ({len(add)} ops), {why}", file=sys.stderr, flush=True)
    kept += mine
for op in kept: op.pop("alt", None)
print("[\n" + ",\n".join("  " + json.dumps(o) for o in kept) + "\n]")
