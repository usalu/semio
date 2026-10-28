"""🔎 Recompute every number printed in Beat6_EvolutionarySearch from the model and compare it with the text seen on the 1280 x 720 keyframes."""

import importlib.util
import math
from pathlib import Path

MODEL = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py")
spec = importlib.util.spec_from_file_location("optimization_model", MODEL)
model = importlib.util.module_from_spec(spec)
spec.loader.exec_module(model)

FAILS = []


def check(label, shown, value, digits=0, comma=False):
    """🧮 Format the model value like the screen does and record a mismatch when the strings differ."""
    if isinstance(value, str):
        text = value
    else:
        text = f"{value:,.{digits}f}" if comma else f"{value:.{digits}f}"
    ok = text == shown
    if not ok:
        FAILS.append(label)
    print(f"{'OK      ' if ok else 'MISMATCH'} {label}: screen {shown!r} model {text!r} (raw {value!r})")


def sum_check(label, shown_total, parts, digits=0):
    """➕ Segments of the stacked bar must add up to the printed total within one display unit."""
    total = sum(parts)
    ok = abs(total - shown_total) <= 1.0 + 10 ** (-digits)
    if not ok:
        FAILS.append(label)
    print(f"{'OK      ' if ok else 'MISMATCH'} {label}: parts {[round(p, 2) for p in parts]} sum {total:.2f} screen {shown_total}")


def design_text(design):
    """🪪 Window share in %, overhang in m, as printed on the cards."""
    return f"{design['wwr'] * 100:.0f} %", f"{design['overhang']:.2f} m"


study = model.load_study()
baseline = study["baseline"]
start = model.gradient_path()[0]["design"]
found = model.evaluate(start)
print("== Start design (75 %, 0.10 m, 16 cm)")
check("start window share", "75", start["wwr"] * 100)
check("start overhang 1 dp (room label)", "0.1", start["overhang"], 1)
check("start electricity (bar, task frame)", "745", found["electricity"])
check("start heating", "35", found["heating"])
check("start cooling (room and bar)", "593", found["cooling"])
check("start lighting", "117", found["lighting"])
sum_check("start bar segments", 745, [found["heating"], found["cooling"], found["lighting"]])

print("== Sun")
sun = model.sun_state(180, "summer")
check("summer noon solar altitude (Sun 66 deg)", "66", sun["altitude"])
winter = model.sun_state(180, "winter")
print(f"INFO    winter noon altitude {winter['altitude']:.2f} deg (not printed on screen)")

print("== Map minimum (41 x 41 grid)")
surf = model.surface(41)
opt = surf["minimum"]
od = opt["design"]
best_found = model.evaluate(od)
check("map minimum window share", "66", od["wwr"] * 100)
check("map minimum overhang 2 dp (close chip)", "0.99", od["overhang"], 2)
check("map minimum overhang 1 dp (room label 'd 1.0 m')", "1.0", od["overhang"], 1)
check("map minimum electricity 1 dp", "494.8", opt["electricity"], 1)
check("map minimum electricity rounded (bar, colour bar)", "495", opt["electricity"])
check("map minimum heating", "40", best_found["heating"])
check("map minimum cooling", "273", best_found["cooling"])
check("map minimum lighting", "182", best_found["lighting"])
sum_check("map minimum bar segments", 495, [best_found["heating"], best_found["cooling"], best_found["lighting"]])
check("map minimum cost (close chip)", "21,260", best_found["cost"], 0, comma=True)
check("runs on the map", "1,681", surf["wwr"].__len__() * surf["overhang"].__len__(), 0, comma=True)

conv = model.convergence()
budget = conv["budget"]
print("== Convergence and reach")
check("budget", "50", budget)
limit = 1.01 * conv["map_minimum"]
reach = {}
for name, series in conv["series"].items():
    values = [v for _, v in series]
    reach[name] = next((i + 1 for i, v in enumerate(values) if v <= limit), None)
print(f"INFO    reach within 1 %: {reach}")
check("reach pattern (16 simulations)", "16", reach["pattern"])
check("reach gradient (41 simulations)", "41", reach["gradient"])
check("reach genetic (41 simulations)", "41", reach["genetic"])
print(f"INFO    'more than twice as many' holds: {reach['gradient'] > 2 * reach['pattern']} and {reach['genetic'] > 2 * reach['pattern']}")

print("== Gradient descent")
g = model.gradient_path()
g0 = g[0]
for probe in g0["probes"]:
    axis = probe["direction"][:-1]
    shown = {("wwr", "-"): ("74.3", "741"), ("wwr", "+"): ("75.7", "748"), ("overhang", "-"): ("0.088", "746"), ("overhang", "+"): ("0.112", "743")}[(axis, probe["direction"][-1])]
    value = probe["design"]["wwr"] * 100 if axis == "wwr" else probe["design"]["overhang"]
    check(f"iterate 0 probe {probe['direction']} position", shown[0], value, 1 if axis == "wwr" else 3)
    check(f"iterate 0 probe {probe['direction']} electricity", shown[1], probe["electricity"])
check("iterate 0 slope windows per %", "5.0", g0["slope"]["wwr"], 1)
check("iterate 0 slope overhang per m", "-136", g0["slope"]["overhang"])
pw = {p["direction"]: p["electricity"] for p in g0["probes"]}
manual = (pw["overhang+"] - pw["overhang-"]) / (0.112 - 0.088)
print(f"INFO    iterate 0 overhang slope from the displayed probes: {manual:.1f} kWh/a per m; model gradient uses unrounded values")
g1 = g[1]
check("iterate 1 electricity (valley-side step)", "613", g1["electricity"])
check("iterate 1 change (Change chip)", "-132", g1["electricity"] - g0["electricity"])
check("iterate 1 slope windows per %", "2.9", g1["slope"]["wwr"], 1)
check("iterate 1 slope overhang per m", "-265", g1["slope"]["overhang"])
check("iterate 1 window share", "55", g1["design"]["wwr"] * 100)
check("iterate 1 overhang 2 dp (chip)", "0.26", g1["design"]["overhang"], 2)
glast = g[-1]
gprev = g[-2]
print(f"INFO    last iterate design {glast['design']} electricity {glast['electricity']:.2f} previous {gprev['electricity']:.2f}")
check("last iterate change (overshoot_2 chip 'Change 6')", "6", glast["electricity"] - gprev["electricity"])
check("last iterate slope windows per % ('-1.6')", "-1.6", glast["slope"]["wwr"], 1)
check("last iterate slope overhang per m ('59')", "59", glast["slope"]["overhang"])
check("last iterate electricity (bar 504)", "504", glast["electricity"])
grad_all = []
for item in g:
    grad_all.append(item)
    grad_all.extend(item["probes"])
gbest = min(grad_all, key=lambda item: item["electricity"])
check("gradient best after 50 runs (496.4)", "496.4", gbest["electricity"], 1)
check("gradient best window share ('61 %')", "61", gbest["design"]["wwr"] * 100)
check("gradient best overhang ('0.91 m')", "0.91", gbest["design"]["overhang"], 2)
check("gradient gap to map minimum ('1.6')", "1.6", round(gbest["electricity"], 1) - round(opt["electricity"], 1), 1)
print(f"INFO    gradient iterates {len(g)}, runs {5 * len(g)} (probes {sum(len(i['probes']) for i in g)} plus one base run per iterate)")

print("== Pattern search")
p = model.pattern_path()
p0 = p[0]
check("pattern start electricity", "745", p0["electricity"])
p1 = p[1]
probes1 = {q["direction"]: q for q in p1["probes"]}
check("pattern probe windows- 55 %", "55", probes1["wwr-"]["design"]["wwr"] * 100)
check("pattern probe windows- electricity", "659", probes1["wwr-"]["electricity"])
check("pattern probe windows+ clamped 80 %", "80", probes1["wwr+"]["design"]["wwr"] * 100)
check("pattern probe windows+ electricity", "770", probes1["wwr+"]["electricity"])
check("pattern probe overhang- clamped 0.00 m", "0.00", probes1["overhang-"]["design"]["overhang"], 2)
check("pattern probe overhang- electricity", "750", probes1["overhang-"]["electricity"])
check("pattern probe overhang+ 0.40 m", "0.40", probes1["overhang+"]["design"]["overhang"], 2)
check("pattern probe overhang+ electricity", "649", probes1["overhang+"]["electricity"])
check("pattern moved to best neighbour (flag)", "True", str(p1["moved"]))
check("pattern round-1 step window share ('20 %')", "20", p1["step"]["wwr"] * 100)
check("pattern round-1 step overhang ('0.30 m')", "0.30", p1["step"]["overhang"], 2)
plast = p[-1]
print(f"INFO    pattern last row step {plast['step']}, moved {plast['moved']}, design {plast['design']}")
check("pattern final electricity (496.7)", "496.7", plast["electricity"], 1)
check("pattern final window share", "75", plast["design"]["wwr"] * 100)
check("pattern final overhang 2 dp", "1.12", plast["design"]["overhang"], 2)
check("pattern final gap", "1.9", round(plast["electricity"], 1) - round(opt["electricity"], 1), 1)
check("pattern final step window share (printed '2 %')", "2", plast["step"]["wwr"] * 100)
check("pattern final step overhang 2 dp (printed '0.04 m')", "0.04", plast["step"]["overhang"], 2)
pl = {q["direction"]: q for q in plast["probes"]}
for key, shown in (("wwr-", "72"), ("wwr+", "78"), ("overhang-", "1.12"), ("overhang+", "1.20")):
    if key in pl:
        value = pl[key]["design"]["wwr"] * 100 if key.startswith("wwr") else pl[key]["design"]["overhang"]
        check(f"pattern final probe {key} position", shown, value, 0 if key.startswith("wwr") else 2)
print(f"INFO    pattern run count {1 + sum(len(item['probes']) for item in p)} (rows {len(p)})")

print("== Genetic algorithm")
ge = model.genetic()
gen0 = ge["generations"][0]["population"]
expected_gen0 = {"10": "810", "26": "654", "78": "649", "23": "643", "30": "598", "12": "765", "27": "587", "29": "618"}
for item in gen0:
    key = f"{item['design']['wwr'] * 100:.0f}"
    if key in expected_gen0:
        check(f"gen-0 card {key} % electricity", expected_gen0[key], item["electricity"])
ranks = sorted(gen0, key=lambda item: item["electricity"])
print("INFO    gen-0 ranks (best first):", [round(item["electricity"], 1) for item in ranks])
check("best parent 587 (elite 1)", "587", ranks[0]["electricity"])
check("second elite 598", "598", ranks[1]["electricity"])
bests = [round(gen["best"]["electricity"]) for gen in ge["generations"]]
check("best per generation string", "587 548 524 523 514 506 498 498", " ".join(str(b) for b in bests), 0)
children1 = ge["generations"][0]["children"]
expected_children = {"31": "563", "36": "548", "32": "557", "23": "624", "78": "581", "29": "572"}
for child in children1:
    key = f"{child['design']['wwr'] * 100:.0f}"
    check(f"population-1 child {key} % electricity", expected_children[key], child["electricity"])
check("children frame counter after six children (14 of 50)", "14", 8 + len(children1))
print(f"INFO    pattern final step window share raw {plast['step']['wwr'] * 100} printed as '2 %' (half-even rounding of 2.5)")
print(f"INFO    pattern final probes raw {pl['wwr-']['design']['wwr'] * 100} and {pl['wwr+']['design']['wwr'] * 100} printed as '72 %' and '78 %' (+/-2.5 around 75, shown as -3 and +3)")
gbest_ge = ge["generations"][-1]["best"]
check("genetic best (498.1)", "498.1", gbest_ge["electricity"], 1)
check("genetic best window share ('57 %')", "57", gbest_ge["design"]["wwr"] * 100)
check("genetic best overhang ('0.87 m')", "0.87", gbest_ge["design"]["overhang"], 2)
check("genetic gap ('3.3')", "3.3", round(gbest_ge["electricity"], 1) - round(opt["electricity"], 1), 1)
print(f"INFO    genetic evaluations {ge['generations'][-1]['evaluations']}")

feature = ge["generations"][0]["children"][0]
lookup = {item["id"]: item for item in gen0}
id_a, id_b = feature["from"]
pa, pb = lookup[id_a], lookup[id_b]
donors = {key: lookup[feature["genes"][key]]["design"][key] for key in ("wwr", "overhang")}
crossed = {**pa["design"], **donors}
crossed_e = model.evaluate(crossed)["electricity"]
check("crossover only (583)", "583", crossed_e)
check("child with mutation (563)", "563", feature["electricity"])
shift = feature["mutation"]
check("nudge windows +0.9 %", "0.9", shift["wwr"] * 100, 1)
check("nudge overhang -0.13 m", "-0.13", shift["overhang"], 2)
check("child window share after mutation ('31.1 %')", "31.1", feature["design"]["wwr"] * 100, 1)
check("child overhang after mutation ('0.56 m')", "0.56", feature["design"]["overhang"], 2)
check("parent A electricity (643)", "643", pa["electricity"])
check("parent B electricity (598)", "598", pb["electricity"])

duels = [duel for duel in ge["generations"][0]["tournaments"] if duel[0] != duel[1]]
rank = {item["id"]: r for r, item in enumerate(ranks, 1)}
weak = max(duels, key=lambda duel: (rank[duel[2]], rank[next(i for i in duel[:2] if i != duel[2])]))
winner, loser = weak[2], next(i for i in weak[:2] if i != weak[2])
check("weak duel winner rank ('rank 5 of 8')", "5", rank[winner])
check("weak duel loser rank ('rank 8')", "8", rank[loser])
print(f"INFO    weak duel winner {lookup[winner]['electricity']:.1f} loser {lookup[loser]['electricity']:.1f}")

later = ge["generations"][1]
later_lookup = {item["id"]: item for item in later["population"]}
options = []
for item in later["children"]:
    if any(item["mutation"].values()):
        cross = {**later_lookup[item["from"][0]]["design"], **{key: later_lookup[item["genes"][key]]["design"][key] for key in ("wwr", "overhang")}}
        options.append((item, cross, item["electricity"] - model.evaluate(cross)["electricity"]))
worse = [o for o in options if o[2] > 0] or options
second, second_cross, _ = max(worse, key=lambda o: abs(o[2]))
cross_e = model.evaluate(second_cross)["electricity"]
check("later child crossover only (629)", "629", cross_e)
check("later child with mutation (648)", "648", second["electricity"])
check("later child nudge windows ('0.0 %')", "0.0", second["mutation"]["wwr"] * 100, 1)
check("later child nudge overhang ('-0.07 m')", "-0.07", second["mutation"]["overhang"], 2)
born = {item["id"]: index + 1 for index, item in enumerate([*gen0, *(child for gen in ge["generations"] for child in gen["children"])])}
print(f"INFO    later child (generation {second['id']}) is simulation #{born[second['id']]} of 50; the frame reads 18 of 50 at this point (round 3)")
if born[second["id"]] != 18:
    FAILS.append("later child counter differs from its simulation index")
    print(f"MISMATCH later child shown at run 18 but simulated at run {born[second['id']]}")

print("== Stopping rules (tolerance: gain below 1 kWh/a in 5 runs; stagnation: none in 10 runs)")
for name, series in conv["series"].items():
    values = [v for _, v in series]
    tol = next((n for n in range(6, budget + 1) if values[n - 6] - values[n - 1] < 1.0), None)
    stag = next((n for n in range(11, budget + 1) if values[n - 11] - values[n - 1] <= 0), None)
    print(f"INFO    {name}: tolerance fires at {tol}, stagnation at {stag}")
check("tolerance stop gradient (23)", "23", next((n for n in range(6, budget + 1) if [v for _, v in conv['series']['gradient']][n - 6] - [v for _, v in conv['series']['gradient']][n - 1] < 1.0), 0))
check("tolerance stop pattern (21)", "21", next((n for n in range(6, budget + 1) if [v for _, v in conv['series']['pattern']][n - 6] - [v for _, v in conv['series']['pattern']][n - 1] < 1.0), 0))
check("tolerance stop genetic (15)", "15", next((n for n in range(6, budget + 1) if [v for _, v in conv['series']['genetic']][n - 6] - [v for _, v in conv['series']['genetic']][n - 1] < 1.0), 0))

print("== Bar segment sums (legend values as printed)")
for label, design, shown_total, shown_parts in (
        ("gradient best (gradient_result, confirm)", gbest["design"], 496.4, (42.8, 263.8, 189.8)),
        ("crossover child (what-if, not drawn in the bar)", crossed, 583.0, (85.7, 191.1, 306.2)),
        ("mutated child (mutation)", feature["design"], 563.2, (83.6, 193.0, 286.6)),
        ("children frame (548)", ge["generations"][1]["best"]["design"], 548.3, (72.9, 228.3, 247.1)),
        ("genetic best (genetic_result)", gbest_ge["design"], 498.1, (46.1, 251.6, 200.4)),
        ("pattern best (pattern_result)", plast["design"], 496.7, (35.8, 292.9, 168.0))):
    found_parts = model.evaluate(design)
    parts = [found_parts["heating"], found_parts["cooling"], found_parts["lighting"]]
    printed = sum(shown_parts)
    exact = found_parts["electricity"]
    print(f"INFO    {label}: printed parts {shown_parts} sum {printed}, printed total {shown_total}, exact total {exact:.2f}, exact parts {[round(x, 2) for x in parts]}")
    if abs(printed - shown_total) > 1e-9:
        FAILS.append(f"sum of printed segments differs from total: {label}")
        print(f"MISMATCH printed segments sum {printed} but total {shown_total}: {label}")

print("== Summary")
print(f"{len(FAILS)} mismatches" + (": " + ", ".join(FAILS) if FAILS else ""))
