"""🎛️ Chooses, per plan shape, the solid that keeps the silhouette guard: `python3 tune.py <id> [--only <scale>]` reads `ops/<id>.json`, and for every `plan-revolve` op tries in turn the body of revolution, the prism of the front twin (`front-sweep` then `copy` front to plan), the elevation outline swept laterally (`extrude`, view plan) and, for a slab swept along the bone, the full-length prism (`plan-along`), each on top of the ops already settled, with `fit.ts` on a copy and `../MR/measure.ts` at that scale. The first candidate without a failing silhouette check, without a new validator issue or sweep finding, with a plan IoU at most 0.004 below the backup (never below 0.922 unless the backup is) and a plan boundary distance at most max(backup, 90 % of the limit) is kept; if none qualifies the op is dropped and reported (the shape stays as it was). An entry `{ "choose": [[op…], [op…]], "scale", "label" }` states its own candidates (each a list of ops without the scale) and is settled the same way. Writes `ops/<id>.json` back and the log to $GEN/tune/<id>.log. Paths come from the environment of `../MR/env.sh` (BATCH=MH-1)."""
import json, os, re, subprocess, sys
aid = sys.argv[1]
only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else None
B, S, M, W, GEN, T = (os.environ[k] for k in ("B", "S", "M", "W", "GEN", "T"))
LIMIT = {"50": 15.0, "100": 30.0, "200": 60.0, "500": 150.0}
os.makedirs(f"{GEN}/tune", exist_ok=True)
log = open(f"{GEN}/tune/{aid}.log", "a")
def say(text):
    print(text); log.write(text + "\n"); log.flush()
def measure(path, scale):
    out = subprocess.run([B, f"{M}/measure.ts", path, "--scales", scale], cwd=f"{S}/📦️packages/🟦️typescript", capture_output=True, text=True, timeout=900).stdout
    m = re.search(r"1:%s bbox (.*?)  elevation ([\d.]+)(!?)/([\d.]+)mm(!?)  front ([\d.]+)(!?)/([\d.]+)mm(!?)  plan ([\d.]+)(!?)/([\d.]+)mm(!?)" % scale, out)
    if not m: return None
    return {"fails": out.split(f"1:{scale} bbox")[1].split("\n")[0].count("!"), "front": (float(m[6]), float(m[8])), "plan": (float(m[10]), float(m[12])), "elevation": (float(m[2]), float(m[4])),
            "validate": int(re.search(r"validate: (\d+)", out)[1]), "sweep": int(re.search(r"sweep: (\d+)", out)[1])}
def fit(ops):
    json.dump(ops, open(f"{GEN}/tune/{aid}.ops.json", "w"))
    run = subprocess.run([B, f"{W}/fit.ts", f"{GEN}/backup/{aid}.json", "--ops", f"{GEN}/tune/{aid}.ops.json", "--out", f"{GEN}/tune/{aid}.json"], cwd=T, capture_output=True, text=True, timeout=600)
    return run.returncode == 0, run.stdout
ops = json.load(open(f"ops/{aid}.json"))
base = {}
settled, dropped = [], []
for op in ops:
    tunable = op.get("op") == "plan-revolve" or "choose" in op
    if not tunable or (only and op["scale"] != only):
        settled.append(op); continue
    scale, sid = op["scale"], op.get("id") or op.get("label")
    if scale not in base: base[scale] = measure(f"{GEN}/backup/{aid}.json", scale)
    ref = base[scale]
    extra = {k: v for k, v in op.items() if k not in ("op", "scale", "id", "view", "from", "to")}
    sweep = {"op": "front-sweep", "scale": scale, "id": sid}
    candidates = [("revolve", [{"op": "plan-revolve", "scale": scale, "id": sid, **extra}]),
                  ("front-prism", ([] if sweep in settled else [sweep]) + [{"op": "copy", "scale": scale, "id": sid, "from": "front", "to": ["plan"]}]),
                  ("extrude", [{"op": "extrude", "scale": scale, "id": sid, "view": "plan", **extra}]),
                  ("along", [{"op": "plan-along", "scale": scale, "id": sid, "clip": True}])]
    if "choose" in op: candidates = [(f"choice {i + 1}", [{"scale": scale, **c} for c in cand]) for i, cand in enumerate(op["choose"])]
    chosen = None
    for name, cand in candidates:
        ok, out = fit(settled + cand)
        if not ok: say(f"  1:{scale} {sid} {name}: refused"); continue
        m = measure(f"{GEN}/tune/{aid}.json", scale)
        if m is None: say(f"  1:{scale} {sid} {name}: no measure"); continue
        good = m["fails"] <= ref["fails"] and m["validate"] <= ref["validate"] and m["sweep"] <= ref["sweep"] and m["plan"][0] >= min(ref["plan"][0], max(0.922, ref["plan"][0] - 0.004)) and m["plan"][1] <= max(ref["plan"][1], 0.9 * LIMIT[scale]) + 0.05 and m["front"][0] >= ref["front"][0] - 0.004 and m["front"][1] <= max(ref["front"][1], 0.9 * LIMIT[scale]) + 0.05
        say(f"  1:{scale} {sid} {name}: plan {m['plan'][0]:.4f}/{m['plan'][1]:.1f} (was {ref['plan'][0]:.4f}/{ref['plan'][1]:.1f}) front {m['front'][0]:.4f}/{m['front'][1]:.1f} fails {m['fails']} validate {m['validate']} sweep {m['sweep']} {'KEEP' if good else 'no'}")
        if good: chosen = cand; break
    if chosen is None: dropped.append((scale, sid)); say(f"  1:{scale} {sid}: NO candidate keeps the silhouette, op dropped")
    else: settled += chosen
open(f"ops/{aid}.json", "w").write("[\n" + ",\n".join("  " + json.dumps(o, ensure_ascii=False) for o in settled) + "\n]\n")
say(f"{aid}: {len(settled)} ops settled, dropped {dropped}")
