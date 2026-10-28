"""🔍 Recompute every number printed in the round-2 extrema frames from the optimization model and compare with the screen readings."""

import importlib
import sys
from decimal import Decimal, ROUND_HALF_UP
from pathlib import Path

ROOT = Path("C:/git/semio")
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8")
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
study = model.orientation_study()
fixed = dict(study["design"])
failures, notes = [], []


def half_up(value, digits=0):
    """🧮 Round half up as a reader expects."""
    return float(Decimal(repr(float(value))).quantize(Decimal(1).scaleb(-digits), rounding=ROUND_HALF_UP))


def at(theta, **design):
    """📍 Model result at one façade bearing and optional design overrides."""
    return model.evaluate({**fixed, **design, "orientation": float(theta)})


def check(where, screen, value, digits=0, tol=0.0):
    """✅ Compare a printed value with the model value rounded half up."""
    expect = half_up(value, digits)
    ok = abs(float(screen) - expect) <= tol + 1e-9
    line = f"{'ok  ' if ok else 'FAIL'} {where}: screen {screen} | model {expect} (raw {value:.4f})"
    print("[DEBUG] " + line)
    if not ok:
        failures.append(line)


def show(where, value):
    """📝 Print a raw model value for the record."""
    print(f"[DEBUG] info {where}: {value}")


print("[DEBUG] --- bars and parts by bearing (screen values from the frames)")
screen_bars = {
    90: (181, 803, 147, 1131, 283),
    180: (43, 503, 135, 681, 170),
    282: (218, 778, 150, 1146, 286),
    315: (289, 595, 158, 1042, 261),
    360: (311, 407, 164, 882, 221),
    450: (181, 803, 147, 1131, 283),
}
for theta, (h, c, l, e, cost) in screen_bars.items():
    r = at(theta)
    check(f"{theta} heating", h, r["heating"])
    check(f"{theta} cooling", c, r["cooling"])
    check(f"{theta} lighting", l, r["lighting"])
    check(f"{theta} electricity", e, r["electricity"])
    check(f"{theta} CHF/a", cost, r["electricity"] * study["tariff"])
    show(f"{theta} parts sum of unrounded", r["heating"] + r["cooling"] + r["lighting"])

print("[DEBUG] --- derivations on screen")
for theta, demand, cop_or_eer, shown in ((180, "heating", 3.5, 152), (180, "cooling", 3.0, 1510), (282, "cooling", 3.0, 2334)):
    r = at(theta)
    value = r[f"{demand}_demand"]
    check(f"{theta} {demand} demand kWh/a (printed {shown})", shown, value)
    show(f"{theta} {demand} demand / {cop_or_eer}", value / cop_or_eer)
for theta, shown_share, shown_kwh in ((180, 21, 135), (360, 25, 164), (282, 22, 150), (90, 19, 147)):
    r = at(theta)
    lamps = 1 - r["autonomy"]
    show(f"{theta} lamps-on share (1 - autonomy)", f"{lamps * 100:.2f} %")
    show(f"{theta} lighting from power x hours x share", f"0.43 x 1500 x {lamps:.4f} = {0.43 * 1500 * lamps:.2f}")
    check(f"{theta} lamps-on share printed {shown_share} %", shown_share, lamps * 100)
    naive = round(0.43 * 1500 * shown_share / 100)
    print(f"[DEBUG] {'ok  ' if naive == shown_kwh else 'FAIL'} {theta} printed arithmetic 0.43 x 1,500 x {shown_share} % = {naive} vs printed {shown_kwh}")
    if naive != shown_kwh:
        failures.append(f"{theta}: printed arithmetic 0.43 x 1,500 x {shown_share} % = {naive}, printed result {shown_kwh}")

print("[DEBUG] --- sun hours and strength on screen")
sun_screen = {
    90: ((4, 100), (3, 44)),
    180: ((9, 57), (7, 100)),
    282: ((4, 96), (3, 37)),
    315: ((3, 70), (0, 24)),
    360: ((0, 44), (0, 21)),
    450: ((4, 100), (3, 44)),
}
for theta, ((sh, ss), (wh, ws)) in sun_screen.items():
    r = at(theta)
    for season, hours, strength_shown, key in (("summer", sh, ss, "sun_summer"), ("winter", wh, ws, "sun_winter")):
        count = len(model.sun_samples(round(theta, 2), season))
        ok = count == hours
        line = f"{'ok  ' if ok else 'FAIL'} {theta} {season} direct sun hours: screen {hours} | model {count}"
        print("[DEBUG] " + line)
        if not ok:
            failures.append(line)
        check(f"{theta} {season} strength % (printed {strength_shown})", strength_shown, r[key] * 100, 0, tol=0.5)

print("[DEBUG] --- neighbour decomposition (local, local_2)")
rows = {315: {310: (-7.1, 33.3, -1.1, 25.1), 320: (5.8, -33.0, 1.0, -26.2)}, 360: {355: (-0.2, 2.8, 0.0, 2.6), 365: (-0.2, 2.8, 0.0, 2.6)}}
for centre, neighbours in rows.items():
    here = at(centre)
    for angle, (sh, sc, sl, se) in neighbours.items():
        there = at(angle)
        dh, dc, dl = there["heating"] - here["heating"], there["cooling"] - here["cooling"], there["lighting"] - here["lighting"]
        de = there["electricity"] - here["electricity"]
        for name, shown, value in (("heating", sh, dh), ("cooling", sc, dc), ("lighting", sl, dl), ("dE", se, de)):
            ok = abs(half_up(value, 1) - shown) < 1e-9
            line = f"{'ok  ' if ok else 'FAIL'} neighbour {angle} vs {centre} {name}: screen {shown:+.1f} | model {half_up(value, 1):+.1f} (raw {value:+.3f})"
            print("[DEBUG] " + line)
            if not ok:
                failures.append(line)
        show(f"neighbour {angle} sum of printed parts", round(sh + sc + sl, 1))

print("[DEBUG] --- global and east tables")
south, north, east = at(180), at(360), at(90)
for name, a, b, shown in (("heating", north, south, -268), ("cooling", north, south, 96), ("lighting", north, south, -29), ("electricity", north, south, -201)):
    check(f"north to south {name} change (printed {shown:+d})", abs(shown), abs(b[name if name != "electricity" else "electricity"] - a[name if name != "electricity" else "electricity"]))
for name, shown in (("heating", 138), ("cooling", 300), ("lighting", 12), ("electricity", 450)):
    check(f"east to south saving {name} (printed {shown:+d})", shown, east[name] - south[name])
show("saving 449.8 x 0.25", round(449.8 * 0.25, 4))
show("model saving x tariff", round((east["electricity"] - south["electricity"]) * study["tariff"], 4))
show("north minus south in CHF", round((north["electricity"] - south["electricity"]) * study["tariff"], 4))

print("[DEBUG] --- maximum and minimum of the west sweep, S dip")
grid = {t: at(t)["electricity"] for t in range(90, 451)}
argmax = max(range(262, 301), key=grid.get)
check("maximum around the west sweep is 282 deg (screen 282)", 282, argmax)
show("E(282), E(270), E(275)", (round(grid[282], 2), round(grid[270], 2), round(grid[275], 2)))
show("S(282) = E(90) - E(282)", round(grid[90] - grid[282], 2))
show("study local maxima", study["local_maxima"])

print("[DEBUG] --- scope: overhang 0.6 m curve on the courtyard domain (screen 354, 743 kWh/a)")
lo, hi = study["constraint"]
curve = {t: at(t, wwr=0.6, overhang=0.6)["electricity"] for t in range(lo, hi + 1)}
best = min(curve, key=curve.get)
check("overhang 0.6 minimum value (screen 743)", 743, curve[best])
print(f"[DEBUG] {'ok  ' if best == 354 else 'FAIL'} overhang 0.6 argmin on 1 deg grid: screen 354 | model {best}")
if best != 354:
    failures.append(f"overhang 0.6 argmin {best} != 354")
jumps = sorted(((abs(curve[t + 1] - curve[t]), t, t + 1) for t in range(lo, hi)), reverse=True)[:8]
for size, a, b in jumps:
    print(f"[DEBUG] info one-degree jump {a}->{b}: {size:.1f} kWh/a ({curve[a]:.1f} -> {curve[b]:.1f})")
print("[DEBUG] info E around the overhang minimum:", [(t, round(curve[t], 1)) for t in range(350, 358)])
check("window 30 % minimum (screen 832)", 832, min(at(t, wwr=0.3)["electricity"] for t in range(lo, hi + 1)))
check("window 60 % minimum (screen 882)", 882, min(at(t)["electricity"] for t in range(lo, hi + 1)))
check("window 80 % minimum (screen 966)", 966, min(at(t, wwr=0.8)["electricity"] for t in range(lo, hi + 1)))

print()
print("[DEBUG] mismatches:", len(failures))
for line in failures:
    print("[DEBUG] MISMATCH " + line)
