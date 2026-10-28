"""🔢 Recompute every table and chip of the extrema clip round 2 from the model and check that each printed sum closes."""

import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8")
CLIP = "🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip"
model = importlib.import_module(CLIP + ".🧮️model.🐍️")
room = importlib.import_module(CLIP + ".🎨️visuals.🏠️room.🐍️")
study = model.orientation_study()
fixed = study["design"]
tariff = study["tariff"]
failures = []
KEYS = ("heating", "cooling", "lighting")
BAL = {"heating": ("heating_losses", "heating_gains", "heating_eta", "heating_used"), "cooling": ("cooling_gains", "cooling_removable", "cooling_eta", "cooling_removed")}


def at(theta):
    """📍 Result of the study design at one bearing."""
    return model.evaluate({**fixed, "orientation": float(theta)})


def check(label, condition, detail=""):
    """✅ Print one check and remember failures."""
    if not condition:
        failures.append(label)
    print(f"[DEBUG] {'ok  ' if condition else 'FAIL'} {label} {detail}")


def wholes(found, digits=0):
    """🧮 Parts and total as printed (toolkit largest remainder)."""
    shown = room.apportion([found[key] for key in KEYS], digits, found["electricity"])
    return shown, round(sum(shown), digits)


for theta in (90, 180, 225, 282, 315, 360, 450):
    shown, total = wholes(at(theta))
    check(f"formula at {theta}", abs(sum(shown) - total) < 1e-9 and total == round(at(theta)["electricity"]), f"{shown} = {total}")
lighting = 9 * 8 * 6 * 1500 / 1000
for theta in (90, 180, 270, 315, 360):
    found = at(theta)
    check(f"lighting derivation at {theta}", abs(lighting * (1 - found["autonomy"]) - found["lighting"]) < 1e-9, f"{lighting:.0f} x {(1 - found['autonomy']) * 100:.1f} % = {found['lighting']:.1f}")
    check(f"heating derivation at {theta}", abs(found["heating_demand"] / 3.5 - found["heating"]) < 1e-9)
    check(f"cooling derivation at {theta}", abs(found["cooling_demand"] / 3.0 - found["cooling"]) < 1e-9)

for center in (study["start"], study["local_minima"][1]):
    here = at(center)
    for angle in (center - study["neighbour_step"], center + study["neighbour_step"]):
        there = at(angle)
        diff = {key: there[key] - here[key] for key in (*KEYS, "electricity")}
        shown, total = wholes(diff, 1)
        check(f"neighbour {angle} of {center}", abs(sum(shown) - total) < 1e-9, f"dE {total:+.1f} = " + " ".join(f"{value:+.1f}" for value in shown))

for before, after, saved in ((360, 180, False), (90, 180, True)):
    low, low_total = wholes(at(before))
    high, high_total = wholes(at(after))
    changes = [(b - a) * (-1 if saved else 1) for a, b in zip(low, high)]
    total = (high_total - low_total) * (-1 if saved else 1)
    check(f"table {before} to {after}", abs(sum(changes) - total) < 1e-9, f"{low} -> {high}: {changes} = {total}")
saving = at(90)["electricity"] - at(180)["electricity"]
check("saving 449.8 x 0.25 = 112.5", f"{saving:.1f}" == "449.8" and f"{saving * tariff:.1f}" == "112.5", f"{saving:.2f} -> {saving * tariff:.2f}")
gap = at(360)["electricity"] - at(180)["electricity"]
check("courtyard price", round(gap) == 201 and round(gap * tariff) == 50 and f"{gap / at(180)['electricity'] * 100:.1f}" == "29.5", f"{gap:.1f} kWh/a {gap * tariff:.1f} CHF/a")

for theta in (90, 180, 282, 315, 360):
    found = at(theta)
    for part, first, second, eta, removed in (("heating", "heating_losses", "heating_gains", "heating_eta", "heating_used"), ("cooling", "cooling_gains", "cooling_removable", "cooling_eta", "cooling_removed")):
        winter = part == "heating"
        demand = found[first] - found[removed] if winter else found[first] - found[removed]
        check(f"{part} demand balance at {theta} (model keys)", abs(found[eta] * found[second] - found[removed]) < 1e-9 and abs(demand - found[part + "_demand"]) < 1e-6,
              f"{found[first]:.0f} - {found[eta]:.2f} x {found[second]:.0f} = {demand:.1f}")
    check(f"conversion at {theta}", abs(found["heating_demand"] / found["cop"] - found["heating"]) < 1e-9 and abs(found["cooling_demand"] / found["eer"] - found["cooling"]) < 1e-9)
for theta in (90, 180, 360):
    found = at(theta)
    on = round(100 * found["lamps_on"], 1)
    check(f"lamps line at {theta}", round(found["lighting_power"] * found["lighting_hours"] * on / 100) == round(found["lighting"]) and abs(found["lamps_on"] - (1 - found["autonomy"])) < 1e-12,
          f"{found['lighting_power']:.3f} kW x {found['lighting_hours']:.0f} h/a x {on} % = {found['lighting_power'] * found['lighting_hours'] * on / 100:.1f}")

scene = importlib.import_module(CLIP + ".📉️extrema.🐍️")
widest = {}
for theta in sorted({90, 180, 225, 282, 315, 360, 450, *range(90, 451, 5)}):
    found = at(theta)
    for part in ("heating", "cooling"):
        line = scene.balance_lines(found, part)
        digits, base, percent, top, taken = line["digits"], line["base"], line["percent"], line["top"], line["taken"]
        widest[(part, digits, line["places"])] = widest.get((part, digits, line["places"]), 0) + 1
        product = percent / 100 * base
        demand = scene.half_up(found[part + "_demand"], digits)
        check(f"{part} share line at {theta}: {percent}% x printed base rounds to printed result", scene.half_up(product, digits) == taken and (line["exact"] == (abs(product - taken) < 1e-9)),
              f"{percent}% of {base} = {product:.4f} -> printed {taken} ({digits} dec)")
        check(f"{part} balance line at {theta}: printed numbers subtract exactly", abs(top - taken - demand) < 1e-9 and abs(top - found[BAL[part][0]]) <= 10 ** -digits and abs(taken - found[BAL[part][3]]) <= 10 ** -digits and abs(base - found[BAL[part][1]]) <= 10 ** -digits,
              f"{top} - {taken} = {demand}")
print("[DEBUG] printed precision (part, kWh/a decimals, percent decimals): count", widest)

print("[DEBUG] peak", max(study["samples"], key=lambda row: row[1]), study["local_maxima"], study["local_maxima_electricity"])
for design in ({}, {"overhang": .6}, {"wwr": .3}):
    energy = {theta: model.evaluate({**fixed, **design, "orientation": float(theta)})["electricity"] for theta in range(*study["constraint"], 1)}
    energy[450] = model.evaluate({**fixed, **design, "orientation": 450.0})["electricity"]
    best = min(energy, key=energy.get)
    print(f"[DEBUG] scope {design or 'study design'}: feasible optimum {best} deg {energy[best]:.1f} kWh/a")
sys.exit(1 if failures else 0)
