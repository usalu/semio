"""Throwaway estimate of the Pareto grid with rough teaching coefficients, only to choose a believable storyboard."""
import itertools

AREA, FACADE = 48.0, 24.0
GRID = list(itertools.product((.2, .35, .5, .65, .8), (.10, .20, .30), (0.0, .8)))


def evaluate(wwr, t, d):
    aw, ao = wwr * FACADE, (1 - wwr) * FACADE
    uw = 1 / (.17 + .12 + .9 + t / .035 * .5 + t / .035 * .0 + .0)
    uw = 1 / (.17 + .12 + .5 + t / .035)
    uwin = .9
    h = uw * ao + uwin * aw + 14.0
    hdh = 72.0
    solar_w = 150 * aw * .55 * (1 - .15 * d)
    heat = max(0.0, h * hdh - solar_w - 1500 - 0.0) / 3.5
    solar_s = 330 * aw * .55 * (1 - .65 * d / (d + .5) * 1.0) * (1 - .0)
    cool = max(0.0, solar_s + 1800 - 2600) / 3.2
    cool = max(0.0, solar_s - 1200) / 3.2
    daylight = 2.0 + 5.2 * wwr * (1 - .08 * d)
    auto = min(.85, .22 + 1.0 * wwr * (1 - .18 * d))
    light = 1450 * (1 - auto)
    el = heat + cool + light
    cost = ao * (380 + 7.5 * t * 100) + aw * 1050 + 900 * d * 8
    return {"cost": cost, "electricity": el, "daylight": daylight, "heat": heat, "cool": cool, "light": light}


rows = [(w, t, d, evaluate(w, t, d)) for w, t, d in GRID]


def dom(a, b, keys):
    sa = [a[k] * (1 if k != "daylight" else -1) for k in keys]
    sb = [b[k] * (1 if k != "daylight" else -1) for k in keys]
    return all(x <= y for x, y in zip(sa, sb)) and any(x < y for x, y in zip(sa, sb))


def front(keys):
    return [i for i, r in enumerate(rows) if not any(dom(o[3], r[3], keys) for j, o in enumerate(rows) if j != i)]


f2 = front(("cost", "electricity"))
f3 = front(("cost", "electricity", "daylight"))
for i in sorted(range(len(rows)), key=lambda i: rows[i][3]["cost"]):
    w, t, d, r = rows[i]
    print(f"{i:2d} wwr={w:.2f} t={t:.2f} d={d:.1f} cost={r['cost']:7.0f} el={r['electricity']:6.0f} dl={r['daylight']:4.1f}"
          f" h={r['heat']:5.0f} c={r['cool']:5.0f} l={r['light']:5.0f} {'F2' if i in f2 else '  '} {'F3' if i in f3 else '  '}")
print("front2", len(f2), "front3", len(f3), "only3", [i for i in f3 if i not in f2])
