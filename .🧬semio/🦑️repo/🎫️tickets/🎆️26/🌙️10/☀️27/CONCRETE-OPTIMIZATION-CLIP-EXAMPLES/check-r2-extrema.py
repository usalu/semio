"""🔢 Recompute every number the extrema clip shows on screen from the optimization model."""

import importlib
import json
import sys
from decimal import Decimal, ROUND_HALF_UP
from pathlib import Path

ROOT = Path("C:/git/semio")
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8")
CLIP = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
raw = json.loads((CLIP / "🧮️model/🔣️.json").read_text(encoding="utf-8"))
params = {section: {name: item["value"] for name, item in items.items()} for section, items in raw["parameters"].items()}
tariff = params["economics"]["tariff"]
study = model.orientation_study()
fixed = study["design"]
room = params["room"]
facade, floor = room["width"] * room["height"], room["width"] * room["depth"]
failures = []


def half_up(value, digits=0):
    """🧮 Round half up, the way a reader expects a printed value."""
    return Decimal(repr(float(value))).quantize(Decimal(1).scaleb(-digits), rounding=ROUND_HALF_UP)


def en(value, digits=0):
    """🇬🇧 English number text with comma thousands and dot decimals."""
    return f"{half_up(value, digits):,.{digits}f}"


def de(value, digits=0):
    """🇩🇪 German number text with dot thousands and comma decimals."""
    return en(value, digits).replace(",", "§").replace(".", ",").replace("§", ".")


def at(theta):
    """📍 Evaluated design at one façade bearing."""
    return model.evaluate({**fixed, "orientation": float(theta)})


def check(where, shown, value, digits=0, unit=""):
    """✅ Compare one number as printed in a frame with the recomputed value."""
    text = en(value, digits) if shown[0] not in "+-" else ("+" if value >= 0 else "-") + en(abs(value), digits)
    ok = shown == text
    if not ok:
        failures.append(f"{where}: screen {shown}{unit}, model {text}{unit} (raw {value:.4f})")
    print(f"[DEBUG] {'ok  ' if ok else 'FAIL'} {where}: screen {shown}{unit} | model {text}{unit} | raw {value:.4f} | de {de(value, digits)}")


def note(where, text):
    """📝 Print a derived fact that is not a pass/fail check."""
    print(f"[DEBUG] info {where}: {text}")


print("[DEBUG] --- room and window")
check("facade area 24 m2", "24", facade)
check("floor area 48 m2", "48", floor)
check("window 60 % of 24 m2 = 14.4 m2", "14.4", fixed["wwr"] * facade, 1)
check("tariff 0.25 CHF/kWh", "0.25", tariff, 2)
check("winter sun altitude 19 deg (noon)", "19", raw["sun"]["winter_altitude"])
check("summer sun altitude 66 deg (noon)", "66", raw["sun"]["summer_altitude"])

print("[DEBUG] --- stacked bars and chains by bearing")
bars = {
    180: ("43", "503", "135", "681", "170", "14"),
    90: ("181", "803", "147", "1,131", "283", "24"),
    270: ("181", "803", "147", "1,131", "283", "24"),
    450: ("181", "803", "147", "1,131", "283", "24"),
    315: ("289", "595", "158", "1,042", "261", "22"),
    360: ("311", "407", "164", "882", "221", "18"),
}
for theta, (h, c, l, e, cost, inten) in bars.items():
    r = at(theta)
    tag = f"θ {theta}"
    check(f"{tag} heating", h, r["heating"])
    check(f"{tag} cooling", c, r["cooling"])
    check(f"{tag} lighting", l, r["lighting"])
    check(f"{tag} electricity E", e, r["electricity"])
    check(f"{tag} cost CHF/a", cost, r["electricity"] * tariff)
    check(f"{tag} intensity kWh/(m2a)", inten, r["intensity"])
    shown_sum = int(h.replace(",", "")) + int(c.replace(",", "")) + int(l.replace(",", ""))
    note(f"{tag} shown parts sum", f"{shown_sum} vs shown total {e}: {'consistent' if shown_sum == int(e.replace(',', '')) else 'INCONSISTENT'}")
    if shown_sum != int(e.replace(",", "")):
        failures.append(f"{tag}: shown parts sum {shown_sum} != shown total {e}")

print("[DEBUG] --- sun hours in the section")
for theta, season, shown in ((180, "winter", 7), (90, "winter", 3), (180, "summer", 9), (270, "summer", 4), (360, "summer", 0), (315, "summer", 3), (450, "summer", 4)):
    count = len(model.sun_samples(theta, season))
    ok = count == shown
    if not ok:
        failures.append(f"direct sun {season} {theta}: screen {shown} h, model {count} h")
    print(f"[DEBUG] {'ok  ' if ok else 'FAIL'} direct sun {season} {theta} deg: screen {shown} h | model {count} h")

print("[DEBUG] --- neighbour lens")
e315, e310, e320 = at(315)["electricity"], at(310)["electricity"], at(320)["electricity"]
check("lens 310 delta", "+25", e310 - e315)
check("lens 320 delta", "-26", e320 - e315)
e355, e360, e365 = at(355)["electricity"], at(360)["electricity"], at(365)["electricity"]
check("lens 355 delta", "+3", e355 - e360)
check("lens 365 delta", "+3", e365 - e360)

print("[DEBUG] --- global versus local gap and maximize")
e_global, e_local = study["local_minima_electricity"][0], study["local_minima_electricity"][1]
gap = e_local - e_global
check("global chip gap (882 - 681)", "201", gap)
check("global chip 882 minus 681 on screen", "201", float("882") - float("681"))
check("global chip percent", "29.5", gap / e_global * 100, 1, " %")
check("global chip CHF/a", "50", gap * tariff)
e_east = study["reference_electricity"]
saving = e_east - e_global
check("maximize saving E90 - E180", "450", saving)
check("maximize saving cost (chip)", "112", saving * tariff)
note("maximize chip arithmetic", f"printed 450 x 0.25 = {450 * 0.25}; the unrounded saving gives {saving * tariff:.3f} CHF/a")

print("[DEBUG] --- peaks and extremes (claims in captions)")
grid = {t: at(t)["electricity"] for t in range(90, 451)}
local_peak = max(range(262, 301), key=grid.get)
note("E around the west sweep, 262..300", f"maximum {local_peak} deg, {grid[local_peak]:.2f} kWh/a; E(270) = {grid[270]:.2f}; E(275) = {grid[275]:.2f}")
if local_peak != 282:
    failures.append(f"sweep_3 caption names the maximum at 282 deg (the scene reads it from the model), but E peaks at {local_peak} deg ({grid[local_peak]:.1f} kWh/a)")
peak_all = max(grid, key=grid.get)
note("maximum of E on 90..450", f"{peak_all} deg, {grid[peak_all]:.2f} kWh/a (ties allowed; study local maxima 282 and 438)")
cool = {t: at(t)["cooling"] for t in range(90, 451)}
note("cooling maximum on 90..450", f"{max(cool, key=cool.get)} deg, {max(cool.values()):.1f} kWh/a; cooling(270) = {cool[270]:.1f}")
note("cooling minimum on 90..450", f"{min(cool, key=cool.get)} deg, {min(cool.values()):.1f} kWh/a; cooling(360) = {cool[360]:.1f}")
if min(cool, key=cool.get) != 360:
    failures.append(f"sweep_4 'cooling is least' at 360 deg, but cooling minimum is at {min(cool, key=cool.get)} deg")
note("cooling minimum on 90..180 (sweep_2 'lowest so far')", f"{min(range(90, 181), key=cool.get)} deg, {min(cool[t] for t in range(90, 181)):.1f} kWh/a")
heat = {t: at(t)["heating"] for t in range(90, 451)}
note("heating minimum on 90..450", f"{min(heat, key=heat.get)} deg, {min(heat.values()):.1f} kWh/a")
s_curve = {t: study["reference_electricity"] - grid[t] for t in grid}
dip = min(s_curve, key=s_curve.get)
note("saving minimum (negative part)", f"{dip} deg, S = {s_curve[dip]:.2f} kWh/a")
note("saving at 270", f"S(270) = {s_curve[270]:.2f} kWh/a")
note("E(90) equals E(450)", f"{grid[90]:.3f} vs {grid[450]:.3f}")
note("study local maxima", f"{study['local_maxima']} with {[round(v, 1) for v in study['local_maxima_electricity']]}")
note("study minima", f"{study['local_minima']} with {[round(v, 1) for v in study['local_minima_electricity']]}")
note("global minimum", f"{study['global_minimum']} feasible {study['feasible_minimum']} constraint {study['constraint']}")

print("[DEBUG] --- scope and the three window shares")
lo, hi = study["constraint"]
shown_scope = {0.3: ("832", "360"), 0.6: ("882", "360"), 0.8: ("966", "360")}
for wwr, (value_text, theta_text) in shown_scope.items():
    curve = {t: model.evaluate({**fixed, "wwr": wwr, "orientation": float(t)})["electricity"] for t in range(lo, hi + 1)}
    best = min(curve, key=curve.get)
    best_even = min(range(lo, hi + 1, 2), key=curve.get)
    check(f"scope window {int(wwr * 100)} % minimum value", value_text, curve[best])
    ok = str(best) == theta_text
    if not ok:
        failures.append(f"scope window {int(wwr * 100)} %: argmin {best} deg, screen {theta_text}")
    print(f"[DEBUG] {'ok  ' if ok else 'FAIL'} scope window {int(wwr * 100)} % argmin over 1 deg grid {best} deg, over 2 deg grid {best_even} deg")
direction_changes = []
for index in range(0, 31):
    wwr = round(0.10 + index * 0.025, 4)
    if wwr > 0.8:
        break
    curve = {t: model.evaluate({**fixed, "wwr": wwr, "orientation": float(t)})["electricity"] for t in range(lo, hi + 1)}
    best = min(curve, key=curve.get)
    if best != 360:
        direction_changes.append((round(wwr * 100), best))
note("window shares 10 to 80 % whose minimum is not 360 deg on 270..450", str(direction_changes) if direction_changes else "none: the optimum stays at 360 deg for every window share, so 'perhaps its direction' is not shown")
check("scope_2 bill for 60 % window", "221", at(360)["electricity"] * tariff)

print("[DEBUG] --- German rendering of selected numbers")
for label, value, digits in (("E(90) 1.131", at(90)["electricity"], 0), ("gap 201 CHF", gap * tariff, 0), ("gap percent", gap / e_global * 100, 1)):
    note(label, f"en {en(value, digits)} / de {de(value, digits)}")

print()
print("[DEBUG] mismatches:", len(failures))
for line in failures:
    print("[DEBUG] MISMATCH", line)
