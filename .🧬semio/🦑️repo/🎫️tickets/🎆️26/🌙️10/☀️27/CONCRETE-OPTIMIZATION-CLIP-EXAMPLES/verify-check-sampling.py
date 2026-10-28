"""🔢 Recompute the numbers printed on the current Beat2_GridRandomSampling frames (round 2 verifier)."""

import importlib.util
from decimal import Decimal, ROUND_HALF_UP
from pathlib import Path

CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
SPEC = importlib.util.spec_from_file_location("optimization_model", CLIP / "🧮️model" / "🐍️.py")
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)
TARIFF = M.pareto()["tariff"]
SAMPLES = {kind: M.samples(kind) for kind in ("grid", "random", "lhs")}
SURFACE = M.surface(41)
OPT = SURFACE["minimum"]


def r(value, digits=0):
    """🔢 Round half up from the decimal repr."""
    return str(Decimal(repr(float(value))).quantize(Decimal(1).scaleb(-digits), rounding=ROUND_HALF_UP))


def ev(wwr, d):
    """⚙️ Evaluate one design (south, 16 cm)."""
    return M.evaluate({"wwr": wwr, "overhang": d})


def line(tag, text, value):
    """🧾 Print one checked value."""
    print(f"{tag:<12} {text:<70} {value}")


base = ev(0.45, 0.6)
line("run_2", "heating demand / COP 3.5 (204 shown)", f"{base['heating_demand']:.2f} / 3.5 = {base['heating_demand'] / 3.5:.2f}")
line("run_2", "cooling demand / EER 3 (736 shown)", f"{base['cooling_demand']:.2f} / 3 = {base['cooling_demand'] / 3:.2f}")
line("run_2", "lighting 9 W/m2 x 48 m2 x 1500 h x on-share (219 shown)", f"{9 * 48 * 1500 * (1 - base['autonomy']) / 1000:.2f}")
line("run_2", "bill 522.1 x 0.25", f"{base['electricity'] * TARIFF:.3f} -> {r(base['electricity'] * TARIFF)}; printed arithmetic 522 x 0.25 = {522 * TARIFF}")
line("run", "shading winter / summer, autonomy", f"{base['shading_winter']:.4f} / {base['shading_summer']:.4f} / {base['autonomy']:.4f}")
line("parameters", "window area 45 %", f"{base['window_area']:.3f} m2")

opt = OPT["design"]
o = ev(opt["wwr"], opt["overhang"])
line("reveal_2", "optimum design / electricity", f"WWR {opt['wwr']*100:.3f} d {opt['overhang']:.4f} -> {o['electricity']:.4f}")
line("reveal_2", "heating demand (140 shown) / COP", f"{o['heating_demand']:.3f} / 3.5 = {o['heating_demand'] / 3.5:.3f}")
line("reveal_2", "cooling demand (820 shown) / EER", f"{o['cooling_demand']:.3f} / 3 = {o['cooling_demand'] / 3:.3f}")
line("reveal_2", "parts heating / cooling / lighting", f"{o['heating']:.3f} / {o['cooling']:.3f} / {o['lighting']:.3f}")
line("reveal_2", "lamps on share (28 % shown), autonomy", f"{1 - o['autonomy']:.4f}  lighting 0.432*1500*on = {0.432 * 1500 * (1 - o['autonomy']):.2f}; 0.43*1500*0.28 = {0.43 * 1500 * 0.28:.2f}")
line("reveal_2", "shading summer", f"{o['shading_summer']:.4f}")

lhs_best = min(SAMPLES["lhs"], key=lambda item: item["electricity"])
lb = ev(lhs_best["wwr"], lhs_best["overhang"])
line("limits_4", "LHS best design", f"WWR {lhs_best['wwr']*100:.3f} d {lhs_best['overhang']:.4f} -> {lb['electricity']:.4f}")
line("limits_4", "heating demand (135 shown) / COP 3.5", f"{lb['heating_demand']:.3f} / 3.5 = {lb['heating_demand'] / 3.5:.3f}  (printed 135 / 3.5 = {135 / 3.5:.2f})")
line("limits_4", "cooling demand (842 shown) / EER 3", f"{lb['cooling_demand']:.3f} / 3 = {lb['cooling_demand'] / 3:.3f}")
line("limits_4", "parts heating / cooling / lighting (exact)", f"{lb['heating']:.3f} / {lb['cooling']:.3f} / {lb['lighting']:.3f} sum {lb['electricity']:.3f}")
line("limits_4", "autonomy, on-share (27 shown), lamps 0.43*1500*0.27", f"{lb['autonomy']:.4f} on {1 - lb['autonomy']:.4f}; 0.43*1500*0.27 = {0.43 * 1500 * 0.27:.1f}; 0.432*1500*(1-a) = {0.432 * 1500 * (1 - lb['autonomy']):.2f}")
line("limits_4", "bill 496 x 0.25 / unrounded", f"{lb['electricity'] * TARIFF:.3f}")
line("limits_4", "shading summer / winter", f"{lb['shading_summer']:.4f} / {lb['shading_winter']:.4f}")

worst = ev(0.10, 1.2)
best = ev(0.80, 1.2)
line("grid_4", "worst 10 % / 1.20 m parts H/C/L (183/145/518 shown)", f"{worst['heating']:.2f} / {worst['cooling']:.2f} / {worst['lighting']:.2f} total {worst['electricity']:.2f} autonomy {worst['autonomy']:.4f} window {worst['window_area']:.2f}")
line("grid_4", "best 80 % / 1.20 m parts H/C/L (34/304/161 shown)", f"{best['heating']:.2f} / {best['cooling']:.2f} / {best['lighting']:.2f} total {best['electricity']:.2f} autonomy {best['autonomy']:.4f} window {best['window_area']:.2f}")
line("grid_4", "shading summer worst / best", f"{worst['shading_summer']:.4f} / {best['shading_summer']:.4f}")

for label, wwr, d, shown in (("gaps_2 draw3", 0.33, 0.72, "79/198/291 = 568"), ("gaps_2 draw4", 0.35, 0.78, "75/202/285 = 562")):
    item = ev(wwr, d)
    line(label, f"parts H/C/L total (shown {shown})", f"{item['heating']:.2f} / {item['cooling']:.2f} / {item['lighting']:.2f} total {item['electricity']:.2f} autonomy {item['autonomy']:.4f}")

best_random = min(SAMPLES["random"], key=lambda item: item["electricity"])
br = ev(best_random["wwr"], best_random["overhang"])
line("gaps_3", "best random electricity exact / cost (shown bill 141)", f"{br['electricity']:.3f} -> bill {br['electricity'] * TARIFF:.3f} -> {r(br['electricity'] * TARIFF)}; from printed 562 x 0.25 = {562 * TARIFF}")

ev_last = ev(SAMPLES["lhs"][8]["wwr"], SAMPLES["lhs"][8]["overhang"])
line("evaluate_2", "LHS last draw 76 % / 0.60 m exact parts", f"{ev_last['heating']:.3f} / {ev_last['cooling']:.3f} / {ev_last['lighting']:.3f} total {ev_last['electricity']:.3f} autonomy {ev_last['autonomy']:.4f} bill {ev_last['electricity'] * TARIFF:.3f}")

probe = ev(0.80, 0.0)
line("reveal", "80 % / 0 m exact parts (33/630/112 total 775)", f"{probe['heating']:.3f} / {probe['cooling']:.3f} / {probe['lighting']:.3f} total {probe['electricity']:.3f} autonomy {probe['autonomy']:.4f} bill {probe['electricity'] * TARIFF:.3f}")

m68 = ev(0.68, 1.0)
line("limits_4", "68 % / 1.00 m summer / winter shade (99 / 1 shown)", f"{m68['shading_summer']:.4f} / {m68['shading_winter']:.4f}")

for label, wwr, d in (("run_3 grid mid", 0.45, 0.6), ("random 9", 0.18, 0.08)):
    item = ev(wwr, d)
    line(label, "parts / autonomy", f"{item['heating']:.2f} {item['cooling']:.2f} {item['lighting']:.2f} total {item['electricity']:.2f} autonomy {item['autonomy']:.4f}")

pos_gap = (r(best_random["electricity"], 1), r(OPT["electricity"], 1))
line("compare_2", "deltas exact: grid / random / LHS", f"{min(SAMPLES['grid'], key=lambda i: i['electricity'])['electricity'] - OPT['electricity']:.3f} / {best_random['electricity'] - OPT['electricity']:.3f} / {lhs_best['electricity'] - OPT['electricity']:.3f}")
line("limits", "threshold 1.05 x optimum exact / 1.05 x 494.8", f"{OPT['electricity'] * 1.05:.4f} / {494.8 * 1.05:.4f}")
line("cells", "LHS largest empty box", f"{M.sampling_diagnostics('lhs')['largest_empty_box']}")
line("gaps", "random largest empty box", f"{M.sampling_diagnostics('random')['largest_empty_box']}")


# Exact sample designs (second part of the round 2 numeric verification)
"""🔢 Exact sample designs behind the gaps_2, gaps_3, grid_2 and grid_4 frames."""

import importlib.util
from pathlib import Path

CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
SPEC = importlib.util.spec_from_file_location("optimization_model", CLIP / "🧮️model" / "🐍️.py")
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)
TARIFF = M.pareto()["tariff"]
R = M.samples("random")
L = M.samples("lhs")


def run(wwr, overhang):
    """⚙️ Evaluate one south design at the baseline insulation."""
    return M.evaluate({"wwr": wwr, "overhang": overhang})


def show(tag, wwr, overhang):
    """🧾 Print one design with its exact parts, bill and daylight autonomy."""
    item = run(wwr, overhang)
    print(f"{tag:<16} wwr {wwr*100:.4f} d {overhang:.4f} | H {item['heating']:.3f} C {item['cooling']:.3f} L {item['lighting']:.3f} | total {item['electricity']:.3f} | bill {item['electricity'] * TARIFF:.3f} | autonomy {item['autonomy']*100:.2f} | window {item['window_area']:.2f} m2 | shade S {item['shading_summer']*100:.2f} W {item['shading_winter']*100:.2f}")


show("draw 3 (gaps_2)", R[2]["wwr"], R[2]["overhang"])
show("draw 4 (gaps_2)", R[3]["wwr"], R[3]["overhang"])
best_r = min(R, key=lambda i: i["electricity"])
show("best random", best_r["wwr"], best_r["overhang"])
show("grid 45/0.0", 0.45, 0.0)
show("grid 80/1.2 best", 0.8, 1.2)
show("grid 10/1.2 worst", 0.1, 1.2)
best_l = min(L, key=lambda i: i["electricity"])
show("lhs best", best_l["wwr"], best_l["overhang"])
print("random draws", [(round(i["wwr"] * 100, 4), round(i["overhang"], 4)) for i in R])
