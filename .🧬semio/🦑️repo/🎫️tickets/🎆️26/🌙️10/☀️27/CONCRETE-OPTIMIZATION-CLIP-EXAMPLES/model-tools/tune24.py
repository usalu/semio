import random, sys
sys.path.insert(0, str(__import__("pathlib").Path(__file__).parent))
from tune22 import *
space = {"climate__direct_share_summer": (.6, .9), "climate__internal_gains_summer": (0, 500), "climate__internal_gains_winter": (200, 500), "climate__cooling_degree_hours": (6000, 24000), "climate__utilization_exponent_cooling": (1.0, 8),
         "envelope__g_value": (.35, .55), "envelope__u_window": (.8, 1.3), "lighting__power_density": (8, 12), "daylight__sky_angle_loss": (.2, .6), "daylight__overhang_half_depth": (.3, .9),
         "daylight__reference_factor": (.6, 3), "daylight__max_autonomy": (.75, .95), "envelope__h_other": (8, 20), "climate__heating_degree_hours": (60000, 90000)}
def penalty(kw):
    setv(**kw)
    try: o = check()
    except Exception: return 1e9, None
    pen = 0
    mins, maxs, pn = o["or"]
    if mins != [180, 360]: pen += 20
    if len(maxs) != 2 or not (262 <= maxs[0] <= 290): pen += 20
    pen += max(0, .2 - pn) * 40
    pen += max(0, o["curv"] + .10) * 100
    pen += 0 if o["sav"][1] >= 1.6 * o["sav"][0] else 5
    pen += 0 if o["share"] > .53 else 5
    w, d, e = o["opt"]
    pen += 0 if .4 <= w <= .62 else 20 * abs(w - .5)
    pen += 0 if .4 <= d <= .9 else 20 * abs(d - .65)
    pen += 0 if o["locs"] <= 4 else 1
    pen += 0 if o["rmin"] >= 150 else (150 - o["rmin"]) / 10
    pen += 0 if o["min"] >= 485 else (485 - o["min"]) / 10
    pen += 0 if o["max"] <= 1500 else 3
    return pen, o
rnd = random.Random(int(sys.argv[1]) if len(sys.argv) > 1 else 1)
best = None
for restart in range(4):
    kw = {k: rnd.uniform(*r) for k, r in space.items()}
    pen, o = penalty(kw)
    for step in range(500):
        k = rnd.choice(list(space)); lo, hi = space[k]
        cand = dict(kw); cand[k] = min(hi, max(lo, kw[k] + rnd.gauss(0, (hi - lo) * .08)))
        p2, o2 = penalty(cand)
        if p2 <= pen: kw, pen, o = cand, p2, o2
    print(round(pen, 3), {k.split("__")[1]: round(x, 3) for k, x in kw.items()}, o, flush=True)
