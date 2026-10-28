"""🔎 Recompute every number printed on the Beat4_NormalizedIndex frames from the optimization model and compare the displayed text."""

import importlib.util
import math
from decimal import ROUND_HALF_UP, Decimal
from pathlib import Path

CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
MODEL = CLIP / "🧮️model" / "🐍️.py"
SPEC = importlib.util.spec_from_file_location("optimization_model", str(MODEL))
model = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(model)
study = model.load_study()
PARAMS = {section: {name: item["value"] for name, item in items.items()} for section, items in study["parameters"].items()}
BALANCED = study["weights"]["balanced"]
ORDER = ["electricity", "cost", "daylight", "carbon"]
FLOOR = 48
RESULTS = []


def rnd(value, digits):
    """🔢 Round half up the way a reader expects, starting from the decimal text of the value."""
    return Decimal(repr(float(value))).quantize(Decimal(1).scaleb(-digits), rounding=ROUND_HALF_UP)


def fmt(value, digits=0, lang="en"):
    """🌍 Format like the scene: en 1,250.5 and de 1.250,5."""
    text = f"{rnd(value, digits):,.{digits}f}"
    if lang == "de":
        text = text.replace(",", "#").replace(".", ",").replace("#", ".")
    return text


def check(label, computed, screen):
    """✅ Record one comparison and print it."""
    ok = computed == screen
    RESULTS.append(ok)
    print(f"{'OK      ' if ok else 'MISMATCH'} | {label} | model: {computed} | screen: {screen}")


def variant(identifier):
    """🔎 One evaluated variant (the sixth one included)."""
    return next(item for item in model.variants(extra=True) if item["id"] == identifier)


FIVE = ["punched", "ribbon", "glass_shaded", "punched_insulated", "glass_bare"]
SIX = FIVE + ["glass_premium"]

print("== 1 Cards: raw values (EN text) ==")
CARDS = {
    "punched": ("13.73", "19,428", "2.00", "2,520", "6.0"),
    "ribbon": ("10.92", "20,544", "3.00", "3,840", "12.0"),
    "glass_shaded": ("10.44", "21,475", "3.36", "5,136", "16.8"),
    "punched_insulated": ("13.41", "20,940", "2.00", "3,150", "6.0"),
    "glass_bare": ("15.11", "21,091", "5.60", "2,736", "16.8"),
}
for key, (intensity, cost, daylight, carbon, window) in CARDS.items():
    v = variant(key)
    check(f"{key} intensity kWh/(m²·a)", fmt(v["intensity"], 2), intensity)
    check(f"{key} cost CHF", fmt(v["cost"], 0), cost)
    check(f"{key} daylight %", fmt(v["daylight"], 2), daylight)
    check(f"{key} carbon kg CO2e", fmt(v["carbon"], 0), carbon)
    check(f"{key} window m² (WWR x 24)", fmt(v["window_area"], 1), window)

print("== 2 Cards and rulers: DE text ==")
for key, (intensity, cost, daylight, carbon, _) in CARDS.items():
    v = variant(key)
    check(f"{key} DE intensity", fmt(v["intensity"], 2, "de"), intensity.replace(".", ","))
    check(f"{key} DE cost", fmt(v["cost"], 0, "de"), cost.replace(",", "."))
    check(f"{key} DE daylight", fmt(v["daylight"], 2, "de"), daylight.replace(".", ","))
    check(f"{key} DE carbon", fmt(v["carbon"], 0, "de"), carbon.replace(",", "."))

print("== 3 Room bars and the printed arithmetic (kWh/a) ==")
ROOMS = {
    "glass_shaded": {"lighting": "181", "cooling": "281", "heating": "39", "total": "501", "intensity": "10.44"},
    "glass_bare": {"lighting": "121", "cooling": "567", "heating": "37", "total": "725", "intensity": "15.11"},
    "ribbon": {"lighting": "200", "cooling": "272", "heating": "52", "total": "524", "intensity": "10.92"},
}
for key, screen in ROOMS.items():
    v = variant(key)
    check(f"{key} lighting kWh/a", fmt(v["lighting"], 0), screen["lighting"])
    check(f"{key} cooling kWh/a", fmt(v["cooling"], 0), screen["cooling"])
    check(f"{key} heating kWh/a", fmt(v["heating"], 0), screen["heating"])
    check(f"{key} total kWh/a", fmt(v["electricity"], 0), screen["total"])
    check(f"{key} rounded parts add up to the printed total",
          str(int(fmt(v["lighting"], 0)) + int(fmt(v["cooling"], 0)) + int(fmt(v["heating"], 0))), screen["total"])
    check(f"{key} intensity = total / 48 m² (exact)", fmt(v["electricity"] / FLOOR, 2), screen["intensity"])
    check(f"{key} printed arithmetic: printed total / 48 m²", fmt(int(screen["total"]) / FLOOR, 2), screen["intensity"])
ribbon = variant("ribbon")
check("ribbon cooling demand kWh/a (heat)", fmt(ribbon["cooling_demand"], 0), "816")
check("ribbon cooling demand / EER 3.0 = electricity", fmt(ribbon["cooling_demand"] / PARAMS["systems"]["eer"], 0), "272")
check("ribbon heating = heating demand / COP 3.5", fmt(ribbon["heating_demand"] / PARAMS["systems"]["cop"], 0), "52")
check("glass_bare electricity exact, one decimal (arithmetic line prints 725)", fmt(variant("glass_bare")["electricity"], 1), "725.1")
check("ribbon lighting = 9 W/m² x 48 m² x 1500 h x (1 - autonomy)",
      fmt(PARAMS["lighting"]["power_density"] * FLOOR * PARAMS["lighting"]["hours"] / 1000 * (1 - ribbon["autonomy"]), 0), "200")
check("glass_bare autonomy: lamps off share of lighting hours", fmt(variant("glass_bare")["autonomy"] * 100, 0), "81")
check("glass_bare daylight share printed as lamps-off share", fmt(variant("glass_bare")["autonomy"] * 100, 0, "de"), "81")
check("sun summer noon altitude = 90 - |lat - decl|", fmt(90 - abs(study["sun"]["latitude"] - study["sun"]["declination"]), 0), "66")

print("== 4 Cost and carbon build-up of Punched windows (not on screen, basis for the fix) ==")
p = variant("punched")
wall, window = p["wall_area"], p["window_area"]
cost_parts = {
    "fixed": PARAMS["prices"]["fixed"],
    "wall": wall * PARAMS["prices"]["wall"],
    "insulation": wall * PARAMS["prices"]["insulation_per_cm"] * 100 * 0.16,
    "windows": window * PARAMS["prices"]["window"],
}
print("   cost parts CHF:", {k: round(v) for k, v in cost_parts.items()}, "sum", round(sum(cost_parts.values())))
check("punched cost = sum of parts", fmt(sum(cost_parts.values()), 0), "19,428")
carbon_parts = {
    "wall": wall * (PARAMS["carbon"]["wall"] + PARAMS["carbon"]["insulation_per_cm"] * 100 * 0.16),
    "windows": window * PARAMS["carbon"]["window"],
}
check("punched carbon = sum of parts", fmt(sum(carbon_parts.values()), 0), "2,520")
check("punched elevation: punched openings area 4 x 1.5 x 1.0 m", fmt(4 * 1.5 * 1.0, 1), "6.0")

print("== 5 Units beat: sums of the printed numbers ==")
SUMS_CHF_KG = {"punched": "21,964", "glass_bare": "23,848", "punched_insulated": "24,105", "ribbon": "24,398", "glass_shaded": "26,625"}
SUMS_KCHF_T = {"punched": "37.7", "ribbon": "38.3", "punched_insulated": "39.5", "glass_shaded": "40.4", "glass_bare": "44.5"}
for key, screen in SUMS_CHF_KG.items():
    v = variant(key)
    check(f"{key} sum CHF + kWh/(m²·a) + % + kg", fmt(v["cost"] + v["intensity"] + v["daylight"] + v["carbon"], 0), screen)
for key, screen in SUMS_KCHF_T.items():
    v = variant(key)
    check(f"{key} sum kCHF + kWh/(m²·a) + % + t", fmt(v["cost"] / 1000 + v["intensity"] + v["daylight"] + v["carbon"] / 1000, 1), screen)
print("   DE sums (units_2):", fmt(variant("punched")["cost"] + variant("punched")["intensity"] + variant("punched")["daylight"] + variant("punched")["carbon"], 0, "de"))

print("== 6 Normalized ruler values and the weighted index (balanced 40/30/20/10) ==")
rows = {row["id"]: row for row in model.index(BALANCED)}
SCREEN_NORM = {
    "punched": ("0.71", "0.00", "1.00", "0.00", "0.48", "2"),
    "ribbon": ("0.10", "0.55", "0.72", "0.50", "0.40", "1"),
    "glass_shaded": ("0.00", "1.00", "0.62", "1.00", "0.52", "3"),
    "punched_insulated": ("0.64", "0.74", "1.00", "0.24", "0.70", "5"),
    "glass_bare": ("1.00", "0.81", "0.00", "0.08", "0.65", "4"),
}
for key, screen in SCREEN_NORM.items():
    row = rows[key]
    for criterion, text in zip(ORDER, screen[:4]):
        check(f"{key} normalized {criterion}", fmt(row["normalized"][criterion], 2), text)
    check(f"{key} index", fmt(row["score"], 2), screen[4])
    check(f"{key} rank", str(row["rank"]), screen[5])
exact ={k: variant(k)["intensity"] for k in FIVE}
lo, hi = min(exact.values()), max(exact.values())
check("shift_2 span best to worst kWh/(m²·a)", fmt(hi - lo, 2), "4.67")
check("shift_2 Punched normalized from exact values", fmt((exact["punched"] - lo) / (hi - lo), 2), "0.71")
check("shift_2 Punched normalized from printed values", fmt((13.73 - 10.44) / (15.11 - 10.44), 2), "0.71")
check("shift_2 Ribbon normalized from printed values", fmt((10.92 - 10.44) / (15.11 - 10.44), 2), "0.10")

print("== 7 Sensitivity: cost-driven weights 23/60/11/6 ==")
heavy = model.reweigh(BALANCED, "cost", 0.6)
check("cost-heavy weights printed as 23/60/11/6", " ".join(fmt(heavy[c] * 100, 0) for c in ORDER), "23 60 11 6")
rows_heavy = {row["id"]: row for row in model.index(heavy)}
SCREEN_HEAVY = {"punched": "0.28", "ribbon": "0.46", "punched_insulated": "0.72", "glass_bare": "0.72", "glass_shaded": "0.73"}
for key, screen in SCREEN_HEAVY.items():
    check(f"cost-heavy index {key}", fmt(rows_heavy[key]["score"], 2), screen)
check("cost-heavy order (rank 1..5)", " > ".join(sorted(FIVE, key=lambda k: rows_heavy[k]["rank"])),
      "punched > ribbon > punched_insulated > glass_bare > glass_shaded")
check("weakest criterion of Ribbon = largest normalized value (daylight 0.72)",
      max(ORDER, key=lambda c: rows["ribbon"]["normalized"][c]) + " " + fmt(max(rows["ribbon"]["normalized"].values()), 2),
      "daylight 0.72")

print("== 8 Sweep over the cost weight: switch shares and close calls ==")
steps = [i / 1000 for i in range(0, 1001)]
winners, close, full, gaps = [], [], [], []
for share in steps:
    weights = model.reweigh(BALANCED, "cost", share) if share < 1 else {"electricity": 0, "cost": 1, "daylight": 0, "carbon": 0}
    board = sorted(model.index(weights), key=lambda r: r["score"])
    winners.append((share, board[0]["id"]))
    full.append((share, board[0]["id"], [row["id"] for row in board]))
    gap = board[1]["score"] - board[0]["score"]
    gaps.append((share, gap))
    if gap < 0.03:
        close.append(share)
switches = [(a[0], a[1], b[1]) for a, b in zip(winners, winners[1:]) if a[1] != b[1]]
print("   winner switches (cost weight, from, to):", switches)
print("   close-call share (gap < 0.03) from", min(close) if close else None, "to", max(close) if close else None)
check("winner at 0 % cost weight", winners[0][1], "glass_shaded")
check("winner at 100 % cost weight", winners[-1][1], "punched")
check("first switch cost weight (percent)", fmt(switches[0][0] * 100, 0), "4")
check("second switch cost weight (percent)", fmt(switches[1][0] * 100, 0), "39")
check("cost weight shown at 60 % winner", winners[600][1], "punched")
check("cost weight shown at 5 % winner", winners[50][1], "ribbon")

print("== 9 Range: six designs, rulers re-normalized ==")
rows_six = {row["id"]: row for row in model.index(BALANCED, SIX)}
SCREEN_SIX = {"ribbon": ("0.36", "1"), "glass_shaded": ("0.45", "2"), "punched": ("0.48", "3"), "glass_premium": ("0.52", "4"),
              "glass_bare": ("0.60", "5"), "punched_insulated": ("0.65", "6")}
for key, (score, rank) in SCREEN_SIX.items():
    check(f"six-design index {key}", fmt(rows_six[key]["score"], 2), score)
    check(f"six-design rank {key}", str(rows_six[key]["rank"]), rank)
span5 = (variant("glass_shaded")["cost"] - variant("punched")["cost"])
check("range: Premium cost on the five-design ruler", fmt((variant("glass_premium")["cost"] - variant("punched")["cost"]) / span5, 2), "1.30")
span6 = variant("glass_premium")["cost"] - variant("punched")["cost"]
for key, text in {"ribbon": "0.42", "punched_insulated": "0.57", "glass_bare": "0.63", "glass_shaded": "0.77", "glass_premium": "1.00"}.items():
    check(f"six-design cost ruler {key}", fmt((variant(key)["cost"] - variant("punched")["cost"]) / span6, 2), text)

print("== 10 Limits: cap rule of the scene and the constrained board ==")
winner_cost = variant("ribbon")["cost"]
cap = math.ceil(winner_cost / 500) * 500
check("limit line CHF (scene rule ceil(winner cost / 500) x 500)", fmt(cap, 0), "21,000")
for key in FIVE:
    v = variant(key)
    verdict = "allowed" if v["cost"] <= cap else "not allowed"
    print(f"   {key:18s} cost {fmt(v['cost'], 0):>7s} -> {verdict}")
feasible = [k for k in FIVE if variant(k)["cost"] <= cap]
check("feasible set under the cap", " ".join(sorted(feasible)), "punched punched_insulated ribbon")
check("constrained board index uses the five-design normalization (as printed)", fmt(rows["ribbon"]["score"], 2), "0.40")
sub = [variant(k) for k in feasible]
sub_rows = {}
for criterion in ORDER:
    column = {v["id"]: v[{"electricity": "intensity", "cost": "cost", "daylight": "daylight", "carbon": "carbon"}[criterion]] for v in sub}
    low, high = min(column.values()), max(column.values())
    for key, value in column.items():
        norm = (value - low) / (high - low) if high != low else 0.0
        if criterion == "daylight":
            norm = 1 - norm
        sub_rows.setdefault(key, 0.0)
        sub_rows[key] += BALANCED[criterion] * norm
print("   index if re-normalized over the three allowed designs:", {k: fmt(v, 2) for k, v in sub_rows.items()})
check("Record: electricity range best ... worst", f"{fmt(variant('glass_shaded')['intensity'], 2)} ... {fmt(variant('glass_bare')['intensity'], 2)}", "10.44 ... 15.11")
check("Record: cost range", f"{fmt(variant('punched')['cost'], 0)} ... {fmt(variant('glass_shaded')['cost'], 0)}", "19,428 ... 21,475")
check("Record: daylight range best ... worst", f"{fmt(variant('glass_bare')['daylight'], 2)} ... {fmt(variant('punched')['daylight'], 2)}", "5.60 ... 2.00")
check("Record: carbon range", f"{fmt(variant('punched')['carbon'], 0)} ... {fmt(variant('glass_shaded')['carbon'], 0)}", "2,520 ... 5,136")

print("== 11 Pareto front of the five designs (cost, electricity) ==")
pts = {k: (variant(k)["cost"], variant(k)["intensity"]) for k in FIVE}
front = [k for k in FIVE if not any(all(pts[o][i] <= pts[k][i] for i in (0, 1)) and any(pts[o][i] < pts[k][i] for i in (0, 1)) for o in FIVE if o != k)]
check("Pareto front members", " ".join(sorted(front)), "glass_shaded punched ribbon")
check("Pointer axis point Bare (cost, kWh/(m²·a))", f"{fmt(pts['glass_bare'][0], 0)} {fmt(pts['glass_bare'][1], 2)}", "21,091 15.11")

print()
print("== 12 Sweep detail: full ranking changes and close-call intervals (gap rank 1 to 2 below 0.03) ==")
ranking_changes = [(a[0], tuple(a[2]), tuple(b[2])) for a, b in zip(full, full[1:]) if a[2] != b[2]]
for share, before, after in ranking_changes:
    print(f"   at cost weight {fmt(share * 100, 1)} %: {' > '.join(before)}  ->  {' > '.join(after)}")
bands, start = [], None
for share, in_band in ((s, g < 0.03) for s, g in gaps):
    if in_band and start is None:
        start = share
    if not in_band and start is not None:
        bands.append((start, prev))
        start = None
    prev = share
if start is not None:
    bands.append((start, prev))
print("   close-call bands (cost weight %):", [(fmt(a * 100, 1), fmt(b * 100, 1)) for a, b in bands])
print("   screen shows bands near 0-11 % and 36-44 %")

print()
print(f"SUMMARY: {sum(RESULTS)} OK, {len(RESULTS) - sum(RESULTS)} MISMATCH of {len(RESULTS)} checks")
