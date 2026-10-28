import random, sys
sys.path.insert(0, str(__import__("pathlib").Path(__file__).parent))
from tune22 import *
base = {k: v[k.split("__")[0]][k.split("__")[1]] for k in ("climate__direct_share_summer", "climate__direct_share_winter", "climate__internal_gains_summer", "climate__internal_gains_winter", "climate__cooling_degree_hours", "climate__heating_degree_hours",
        "envelope__g_value", "envelope__u_window", "lighting__power_density", "daylight__sky_angle_loss", "daylight__overhang_half_depth", "daylight__reference_factor", "daylight__max_autonomy", "envelope__h_other", "climate__utilization_exponent_cooling")}
space = {"climate__direct_share_summer": (.5, 1.0), "climate__internal_gains_summer": (0, 500), "climate__internal_gains_winter": (200, 500), "climate__cooling_degree_hours": (6000, 20000), "climate__utilization_exponent_cooling": (1.5, 8),
         "envelope__g_value": (.35, .55), "envelope__u_window": (.8, 1.3), "lighting__power_density": (8, 12), "daylight__sky_angle_loss": (.2, .6), "daylight__overhang_half_depth": (.3, .9),
         "daylight__reference_factor": (1.5, 3), "envelope__h_other": (8, 20), "climate__heating_degree_hours": (60000, 90000)}
rnd = random.Random(7)
results = []
for n in range(3000):
    kw = {k: rnd.uniform(*r) for k, r in space.items()}
    setv(**kw)
    try:
        o = check()
    except Exception as e:
        continue
    pen = 0
    mins, maxs, pn = o["or"]
    if mins != [180, 360]: pen += 5
    if len(maxs) != 2 or not (262 <= maxs[0] <= 290): pen += 5
    pen += max(0, .2 - pn) * 20
    pen += max(0, o["curv"] + .09) * 60
    pen += 0 if o["sav"][1] >= 1.6 * o["sav"][0] else 3
    pen += 0 if o["share"] > .52 else 2
    w, d, e = o["opt"]
    pen += 0 if .35 <= w <= .65 else 5 * abs(w - .5)
    pen += 0 if .35 <= d <= .95 else 5 * abs(d - .65)
    pen += 0 if o["locs"] <= 5 else 1
    pen += 0 if o["min"] >= 480 else (480 - o["min"]) / 20
    pen += 0 if o["max"] <= 2000 else 3
    results.append((pen, kw, o))
results.sort(key=lambda r: r[0])
for pen, kw, o in results[:8]:
    print(round(pen, 2), {k.split("__")[1]: round(x, 3) for k, x in kw.items()}, o)
