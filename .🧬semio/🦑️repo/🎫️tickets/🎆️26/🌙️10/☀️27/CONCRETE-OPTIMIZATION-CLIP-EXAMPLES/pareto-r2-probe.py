"""🔬 Facts the Pareto round 2 scene narrates: steps, savings per year, path to the refinement, weights."""

import importlib.util
from pathlib import Path

CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
SPEC = importlib.util.spec_from_file_location("optimization_model", CLIP / "🧮️model" / "🐍️.py")
MODEL = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODEL)
DATA = MODEL.pareto()
BY = {d["id"]: d for d in DATA["designs"]}
print("front", DATA["front"])
for i, s in enumerate(DATA["steps"], 1):
    a, b = BY[s["from"]], BY[s["to"]]
    print(i, s["from"], "->", s["to"], f"+{s['cost']:.1f} CHF  -{s['saved']:.2f} kWh/a  ratio {s['ratio']:.2f}  payback {s['payback']:.1f}/{s['payback_hi']:.1f}  save/a {s['saved'] * DATA['tariff']:.2f} CHF  dL {b['lighting'] - a['lighting']:.2f} dC {b['cooling'] - a['cooling']:.2f} dH {b['heating'] - a['heating']:.2f}")
print("knee", DATA["knee"], "tariff", DATA["tariff"], DATA["tariff_hi"], DATA["service_life"])
ref = DATA["refinement"]
print("ref", ref)
member = BY[ref["member"]]
path = [member["design"]]
for key in ("overhang", "wwr", "insulation"):
    path.append({**path[-1], key: ref["design"][key]})
last = MODEL.evaluate(path[0])
for key, design in zip(("overhang", "wwr", "insulation"), path[1:]):
    now = MODEL.evaluate(design)
    print(key, design[key], f"dcost {now['cost'] - last['cost']:.2f} delec {now['electricity'] - last['electricity']:.2f}")
    last = now
print("net", last["cost"] - member["cost"], last["electricity"] - member["electricity"])
fronts = [BY[i] for i in DATA["front"]]
rc = max(f["cost"] for f in fronts) - min(f["cost"] for f in fronts)
re = max(f["electricity"] for f in fronts) - min(f["electricity"] for f in fronts)
print("ranges", rc, re, "worth at 0.5", rc / re)
for w in (0, .25, .5, .75, 1):
    print(w, MODEL.weighted_pick(w), "worth", (w * rc / ((1 - w) * re)) if w < 1 else None)
print("third", DATA["third_example"])
t = DATA["third_example"]
for k in ("dominated_2d", "dominator"):
    d = BY[k and t[k]]
    print(k, d["id"], d["cost"], d["electricity"], d["daylight"], MODEL.evaluate(d["design"])["autonomy"])
print("daylight range", min(d["daylight"] for d in DATA["designs"]), max(d["daylight"] for d in DATA["designs"]))
print("elec range", min(d["electricity"] for d in DATA["designs"]), max(d["electricity"] for d in DATA["designs"]))
for k in ("cooling", "daylight", "lighting"):
    print("max", k, max(DATA["designs"], key=lambda d: d[k])["id"])
