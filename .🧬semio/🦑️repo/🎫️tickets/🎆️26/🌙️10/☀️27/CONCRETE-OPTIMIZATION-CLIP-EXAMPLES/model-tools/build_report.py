import importlib.util
import sys
from pathlib import Path

base = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES")
clip = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
spec = importlib.util.spec_from_file_location("model", clip / "🧮️model/🐍️.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
tests = (base / "model-tests.txt").read_text(encoding="utf-8").strip() if (base / "model-tests.txt").exists() else "not run"
study = m.load_study()
f = lambda value, digits=1: f"{value:,.{digits}f}"
out = []
add = out.append
add("# Concrete Optimization Study: Model Report\n")
add("Simplified annual teaching model of one 8 m × 6 m × 3 m seminar room (48 m²), one glazed façade of 24 m², Zurich-like climate. Model: `🧮️model/🐍️.py` (stdlib only); inputs: `🧮️model/🔣️.json`; schema: `🧮️model/🧬️schema/🔣️.schema.json`; expectations: `🧪️cases/🔣️.json`. Generated from the model by `build_report.py` in this folder.\n")
b = m.evaluate()
add("## Baseline design (orientation 180°, WWR 45 %, overhang 0.6 m, insulation 16 cm)\n")
add("| Quantity | Value |\n| --- | --- |")
for key, unit in (("window_area", "m²"), ("wall_area", "m²"), ("u_wall", "W/(m²·K)"), ("heating_demand", "kWh/a"), ("cooling_demand", "kWh/a"), ("heating", "kWh/a"), ("cooling", "kWh/a"), ("lighting", "kWh/a"),
                  ("electricity", "kWh/a"), ("intensity", "kWh/(m²·a)"), ("solar_winter", "kWh/a"), ("solar_summer", "kWh/a"), ("shading_winter", "1"), ("shading_summer", "1"), ("sun_winter", "1"), ("sun_summer", "1"),
                  ("daylight", "%"), ("autonomy", "1"), ("cost", "CHF"), ("carbon", "kg CO₂e")):
    add(f"| {key} | {f(b[key], 3 if b[key] < 10 else 1)} {unit} |")
o = m.orientation_study()
at = lambda angle: o["samples"][angle - 90][1]
add("\n## 1. Orientation extrema (WWR 60 %, overhang 0 m, insulation 16 cm; 1° grid; electricity kWh/a)\n")
add("| Bearing | Electricity |\n| --- | --- |")
add(f"| East 90° = 450° (reference) | {f(at(90))} |")
for angle in o["local_maxima"]:
    add(f"| Local maximum {angle}° | {f(at(angle))} |")
for angle in o["local_minima"]:
    add(f"| Local minimum {angle}° ({'global, south' if angle == o['global_minimum'] else 'north, feasible'}) | {f(at(angle))} |")
add(f"\nNorth is {f(100 * (at(360) / at(180) - 1))} % worse than south. Constraint {o['constraint']}: feasible minimum {o['feasible_minimum']}°. Tariff {o['tariff']} CHF/kWh: saving at south vs east {f(o['savings'][90][1])} kWh/a = {f(o['savings'][90][2])} CHF/a; at north {f(o['savings'][270][1])} kWh/a = {f(o['savings'][270][2])} CHF/a.")
add(f"\nGreedy ±{o['neighbour_step']}° descent from {o['start']}°: " + " → ".join(f"{item['theta']}° ({f(item['electricity'], 0)})" for item in o["descent"]) + ".")
add(f"\nSun: noon altitude winter {o['sun']['winter_altitude']}°, summer {o['sun']['summer_altitude']}°, low summer sun {o['sun']['low_summer_altitude']}°; azimuth winter {o['sun']['winter_azimuth']}, summer {o['sun']['summer_azimuth']}.")
s = m.surface()
add("\n## 2. Surface and samples (south, 16 cm insulation; WWR 10–80 %, overhang 0–1.2 m)\n")
add(f"Dense 41 × 41 minimum: WWR {f(100 * s['minimum']['design']['wwr'])} %, overhang {f(s['minimum']['design']['overhang'], 2)} m, **{f(s['minimum']['electricity'])} kWh/a** (interior; valley of best overhang per WWR runs {f(min(s['overhang'][min(range(41), key=lambda j: row[j])] for row in s['electricity']), 2)}–{f(max(s['overhang'][min(range(41), key=lambda j: row[j])] for row in s['electricity']), 2)} m). Corners: 10 % / 0 m {f(s['electricity'][0][0])}, 80 % / 0 m {f(s['electricity'][-1][0])}, 10 % / 1.2 m {f(s['electricity'][0][-1])}, 80 % / 1.2 m {f(s['electricity'][-1][-1])}.\n")
add("| Plan | Points (WWR %, d m → kWh/a) | Best |\n| --- | --- | --- |")
for kind in ("grid", "random", "lhs"):
    items = m.samples(kind)
    add(f"| {kind} | " + "; ".join(f"{f(100 * i['wwr'], 0)} %, {f(i['overhang'], 2)} m → {f(i['electricity'], 0)}" for i in items) + f" | {f(min(i['electricity'] for i in items))} |")
d = m.sampling_diagnostics()
add(f"\nRandom set: largest empty box WWR {f(100 * d['largest_empty_box']['wwr'][0], 0)}–{f(100 * d['largest_empty_box']['wwr'][1], 0)} % × overhang {f(d['largest_empty_box']['overhang'][0], 2)}–{f(d['largest_empty_box']['overhang'][1], 2)} m ({f(100 * d['area'], 0)} % of the domain); closest pair #{d['closest_pair'][0]}/#{d['closest_pair'][1]} (ΔWWR {f(100 * d['closest_delta']['wwr'])} pp, Δd {f(d['closest_delta']['overhang'], 2)} m; normalized distance {f(d['closest_pair'][2], 3)}). LHS strata index of the points: {[i['strata_index'] for i in m.samples('lhs')]}.")
fa = m.factorial()
add("\n## 3. Factorial experiment (response: cooling electricity, kWh/a; south, 16 cm)\n")
add("| Run | WWR | Overhang | Cooling | Solar summer | Shading summer |\n| --- | --- | --- | --- | --- | --- |")
for run in fa["runs"]:
    add(f"| {run['id']} {run['coded']} | {f(100 * run['design']['wwr'], 0)} % | {f(run['design']['overhang'], 1)} m | {f(run['response'])} | {f(run['solar_summer'], 0)} | {f(run['shading_summer'], 2)} |")
e, c = fa["effects"], fa["coefficients"]
add(f"\nEffects: A (WWR) {f(e['a'])}, B (overhang) {f(e['b'])}, AB {f(e['ab'])} kWh/a. Coefficients b0 {f(c[0])}, b1 {f(c[1])}, b2 {f(c[2])}, b12 {f(c[3])}. Overhang saves {f(fa['runs'][0]['response'] - fa['runs'][1]['response'])} kWh/a at 30 % and {f(fa['runs'][2]['response'] - fa['runs'][3]['response'])} kWh/a at 70 %. Centre (50 %, 0.5 m): predicted {f(fa['centre']['predicted'])}, actual {f(fa['centre']['actual'])} ({f(100 * fa['centre']['curvature'])} %). At 50 % the 0.5 m overhang delivers {f(100 * fa['centre_savings']['share'], 0)} % of the 1.0 m saving.")
fr = m.fractional()
add(f"\nHalf fraction C = AB (C = insulation 10 / 30 cm): runs {[f(r['response']) for r in fr['runs']]}; confounded contrast {f(fr['alias_contrast'])}; true AB {f(fr['true']['ab'])}, true C {f(fr['true']['c'])}, sum {f(fr['true']['sum'])} kWh/a.")
add("\n## 4. Variants, criteria and index (south façade)\n")
add("| Variant | WWR | d | t | kWh/(m²·a) | CHF | Daylight % | kg CO₂e |\n| --- | --- | --- | --- | --- | --- | --- | --- |")
for v in m.variants(extra=True):
    dsg = v["design"]
    add(f"| {v['id']}{' (extra)' if v.get('extra') else ''} — {v['title']['en']} / {v['title']['de']} | {f(100 * dsg['wwr'], 0)} % | {f(dsg['overhang'], 1)} m | {f(100 * dsg['insulation'], 0)} cm | {f(v['intensity'])} | {f(v['cost'], 0)} | {f(v['daylight'], 2)} | {f(v['carbon'], 0)} |")
for name, weights in study["weights"].items():
    rows = sorted(m.index(weights), key=lambda r: r["rank"])
    add(f"\nWeights {name} ({', '.join(f'{k} {f(100 * v, 0)} %' for k, v in weights.items())}); normalization 0 best … 1 worst, score lower is better: " + "; ".join(f"{r['rank']}. {r['id']} {f(r['score'], 3)}" for r in rows) + ".")
ids = [v["id"] for v in m.variants(extra=True)]
add("\nWith the sixth variant (balanced): " + "; ".join(f"{r['rank']}. {r['id']} {f(r['score'], 3)}" for r in sorted(m.index(study["weights"]["balanced"], ids=ids), key=lambda r: r["rank"])) + ".")
p = m.pareto()
lk = {x["id"]: x for x in p["designs"]}
add("\n## 5. Pareto (south; WWR {" + ", ".join(f(100 * x, 0) for x in study["pareto"]["wwr"]) + "} % × insulation {" + ", ".join(f(100 * x, 0) for x in study["pareto"]["insulation"]) + "} cm × overhang {" + ", ".join(f(x, 1) for x in study["pareto"]["overhang"]) + "} m)\n")
add(f"Front ({len(p['front'])} of {len(p['designs'])}): " + ", ".join(p["front"]) + ".\n")
add("| Front member | Cost CHF | Electricity kWh/a | Daylight % |\n| --- | --- | --- | --- |")
for i in p["front"]:
    add(f"| {i} | {f(lk[i]['cost'], 0)} | {f(lk[i]['electricity'])} | {f(lk[i]['daylight'], 2)} |")
add("\n| Step | + CHF | − kWh/a | CHF per kWh/a | Payback 0.25 CHF/kWh (a) | Payback 0.40 CHF/kWh (a) |\n| --- | --- | --- | --- | --- | --- |")
for st in p["steps"]:
    add(f"| {st['from']} → {st['to']} | {f(st['cost'], 0)} | {f(st['saved'])} | {f(st['ratio'])} | {f(st['payback'])} | {f(st['payback_hi'])} |")
add(f"\nService life {p['service_life']} a. Knee: tariff {p['knee']['tariff']}, tariff_hi {p['knee']['tariff_hi']}. Dominated pair: {p['dominated_example']['winner']} dominates {p['dominated_example']['loser']} ({f(lk[p['dominated_example']['winner']]['cost'], 0)} CHF / {f(lk[p['dominated_example']['winner']]['electricity'])} vs {f(lk[p['dominated_example']['loser']]['cost'], 0)} CHF / {f(lk[p['dominated_example']['loser']]['electricity'])}). Step example: {p['step']['from']} → {p['step']['to']}: +{f(p['step']['cost'], 0)} CHF → {f(p['step']['electricity'])} kWh/a.")
r = p["refinement"]
add(f"\nRefinement: the dense 41 × 41 surface optimum (south, 16 cm), an off-grid design, WWR {f(100 * r['design']['wwr'], 0)} %, d {f(r['design']['overhang'], 2)} m, t {f(100 * r['design']['insulation'], 0)} cm ({f(r['cost'], 0)} CHF, {f(r['electricity'])} kWh/a) dominates {r['dominates']}. Third criterion: {p['third_example']['dominated_2d']} is dominated in cost × electricity by {p['third_example']['dominator']} but nondominated with daylight; front3 has {len(p['front3'])} members. Weighted picks (weight on electricity → front member): " + "; ".join(f"{x['weight']} → {x['id']}" for x in p["picks"]) + ".")
g, pa, ge, cv = m.gradient_path(), m.pattern_path(), m.genetic(), m.convergence()
add(f"\n## 6. Algorithms (south surface; start WWR {f(100 * g[0]['design']['wwr'], 0)} %, overhang {f(g[0]['design']['overhang'], 1)} m = {f(g[0]['electricity'], 0)} kWh/a)\n")
add(f"Gradient descent ({len(g)} iterates, 5 runs each, {g[-1]['evaluations']} runs): " + " → ".join(f"({f(100 * x['design']['wwr'], 0)} %, {f(x['design']['overhang'], 2)} m, {f(x['electricity'], 0)})" for x in g) + f". First slope {f(g[0]['slope']['wwr'], 2)} kWh/a per %-point, {f(g[0]['slope']['overhang'], 0)} kWh/a per m; the descent overshoots: electricity rises at iterate(s) {[k + 1 for k in range(1, len(g)) if g[k]['electricity'] > g[k - 1]['electricity']]} before it recovers.")
add(f"\nPattern search ({len(pa)} iterates, {pa[-1]['evaluations']} runs, {sum(not x['moved'] for x in pa[1:])} halvings): " + " → ".join(f"({f(100 * x['design']['wwr'], 0)} %, {f(x['design']['overhang'], 2)} m, {f(x['electricity'], 0)}{'' if x['moved'] else ' halve'})" for x in pa) + ".")
add(f"\nGenetic algorithm (population 8, 2 elites, 6 children, tournament of 2, Park-Miller LCG seed {ge['lcg']['seed']}, 7 generations, {ge['generations'][-1]['evaluations']} runs): best per generation " + ", ".join(f(x["best"]["electricity"]) for x in ge["generations"]) + ".")
add(f"\nConvergence (budget {cv['budget']}, map minimum {f(cv['map_minimum'])} kWh/a over {cv['runs_map']} runs): final " + ", ".join(f"{k} {f(v[-1][1])} ({len(v)} runs)" for k, v in cv["series"].items()) + "; best after 15 runs: " + ", ".join(f"{k} {f(v[14][1])}" for k, v in cv["series"].items()) + ".")
add("\n## Coefficients (all in `🔣️.json`, each with a bilingual `note`)\n")
add("| Section.name | Value | Unit | Meaning |\n| --- | --- | --- | --- |")
for section, items in study["parameters"].items():
    for name, item in items.items():
        add(f"| {section}.{name} | {item['value']} | {item['unit']} | {item['note']['en']} |")
add("\n## Notes for scene agents\n")
add("- `index`: normalized 0 = best … 1 = worst per criterion (daylight reversed), score = weighted mean (lower is better), rank 1 = lowest score. `tournaments` rows are `[candidate, candidate, winner]`.\n- `gradient` is per unit (WWR ratio 1.0, metre); `slope` is per percentage point and per metre.\n- Pattern `step` is the step used for the polls of that round (the start iterate carries the initial step); a stalled round (`moved` false) halves the step of the next round.\n- Genetic `evaluations` of a generation is cumulative including that generation's population (8, 14, … 50); elites keep their ids across generations.\n- Sample/surface/pareto designs all use the south façade with 16 cm insulation unless stated.\n")
import math
add("## Geometric shading\n")
add("The overhang slab sits on the wall top (3.0 m) and the window head at 2.7 m (`room.head`), so 0.3 m of wall stay above the window: the same numbers as `🎨️visuals/🏠️room/🐍️.py` (`HEAD = 2.7`, `HEIGHT = 3.0`, window height = min(area / 8 m, HEAD − 0.05), overhang tip at depth d from the glass plane). They agree.\n")
add("Shaded share of the window height at one sun position = clamp((d · tan(profile angle) − 0.3 m) / window height, 0, 1) with tan(profile) = tan(altitude) / cos(angle across the façade normal); sun behind the façade (across ≥ 89°) or below 3° altitude gives no direct sun. `shading_winter` / `shading_summer` are the beam-weighted averages (weight cos(altitude) · cos(across)) over the hourly solstice sun positions noon ± 4 h that shine on the façade. Solar gain = window · g · irradiance · (1 − direct_share · shading).\n")
sk = m.sun_samples(180, "summer")
add("South, summer solstice: " + "; ".join(f"{x['hour']:+d} h alt {f(x['altitude'])}° across {f(x['across'])}° profile {f(x['profile'])}° weight {f(x['weight'], 2)}" for x in sk) + ".\n")
add(f"Baseline (d = 0.6 m, WWR 45 %): shading_winter {f(b['shading_winter'], 2)} (the 19° winter sun casts only {f(0.6 * math.tan(math.radians(m.sun_state(180, 'winter')['profile'])), 2)} m of shadow, less than the 0.3 m gap), shading_summer {f(b['shading_summer'], 2)}.\n")
add("The toolkit's `sun_state` takes the single sun-path point nearest to the façade bearing (noon for south); the model offers the same function as `sun_state(orientation, season)` but averages the hourly positions, so south summer shading is a little higher than the noon ray alone (the toolkit already bends the drawn ray so that round(7 · shading) rays are blocked).\n")
add("## Test results\n")
add("```\n" + tests + "\n```\n")
(base / "model.md").write_text("\n".join(out) + "\n", encoding="utf-8")
