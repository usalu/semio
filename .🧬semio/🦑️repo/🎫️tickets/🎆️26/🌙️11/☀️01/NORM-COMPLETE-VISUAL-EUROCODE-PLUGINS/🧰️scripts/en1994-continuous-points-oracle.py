"""PyNite cross-check of the EN 1994 action internals on continuous spans for concentrated forces at `xM` and UDLs, uncracked and with EI2 over the cracked fraction at internal supports.

Run: /home/deck/semio/.venv/bin/python -I en1994-continuous-points-oracle.py
Mirrors `catalogue::continuous::action_internals`: permanent actions load every span, variable actions the designed span (sagging) resp. the two spans next to the first internal support (hogging); neighbours carry the same load at the same relative position; result = (sagging, hogging, shear) of the designed span. Prints the reference values hard-coded in the Rust tests.
"""
import numpy as np
from Pynite import FEModel3D

E, I1 = 210e9, 1e-4


def layout(position, span, adjacent):
    adj = adjacent if adjacent > 1e-9 else span
    return {"end": ([span, adj], 0, False), "interior": ([adj, span, adj], 1, False), "cantilever": ([adj, span], 1, True)}[position]


def run(lengths, own, cantilever, loads, ratio, fraction):
    cuts = [0.0]
    stiff = []
    origin = 0.0
    k = len(lengths)
    for i, l in enumerate(lengths):
        zone = fraction * l if ratio < 1.0 else 0.0
        pts = {l}
        if zone > 1e-9:
            if i > 0:
                pts.add(zone)
            if i < k - 1 and not (cantilever and i == k - 1):
                pts.add(l - zone)
        load = loads[i]
        if load and load[0] == "P":
            pts.add(load[2] * l)
        for p in sorted(pts):
            if origin + p > cuts[-1] + 1e-9:
                cuts.append(origin + p)
        origin += l
    m = FEModel3D()
    m.add_material("S", 210e9, 80e9, 0.3, 7850)
    m.add_section("1", 1e-2, I1, I1, 2 * I1)
    m.add_section("2", 1e-2, I1 * ratio, I1 * ratio, 2 * I1)
    for j, x in enumerate(cuts):
        m.add_node(f"n{j}", x, 0, 0)
    bounds = np.cumsum([0.0] + lengths)

    def span_of(x):
        return min(int(np.searchsorted(bounds, x, side="right")) - 1, k - 1)

    for j in range(len(cuts) - 1):
        mid = 0.5 * (cuts[j] + cuts[j + 1])
        s = span_of(mid)
        l = lengths[s]
        local = mid - bounds[s]
        zone = fraction * l if ratio < 1.0 else 0.0
        cracked = (s > 0 and local < zone) or (s < k - 1 and not (cantilever and s == k - 1) and local > l - zone)
        m.add_member(f"m{j}", f"n{j}", f"n{j+1}", "S", "2" if cracked else "1")
        load = loads[s]
        if load and load[0] == "U":
            m.add_member_dist_load(f"m{j}", "Fy", -load[1], -load[1])
    supports = [0.0] + list(bounds[1:k] if cantilever is False else bounds[1:k])
    if not cantilever:
        supports = list(bounds)
    else:
        supports = list(bounds[:k])
    for idx, x in enumerate(supports):
        j = int(np.argmin([abs(c - x) for c in cuts]))
        m.def_support(f"n{j}", idx == 0, True, True, idx == 0, False, False)
    for i, load in enumerate(loads):
        if load and load[0] == "P":
            x = bounds[i] + load[2] * lengths[i]
            j = int(np.argmin([abs(c - x) for c in cuts]))
            m.add_node_load(f"n{j}", "FY", -load[1])
    m.analyze(check_statics=False)
    lo, hi = bounds[own], bounds[own + 1]
    sag = hog = shear = 0.0
    for j in range(len(cuts) - 1):
        if cuts[j] < lo - 1e-9 or cuts[j + 1] > hi + 1e-9:
            continue
        mem = m.members[f"m{j}"]
        arr = np.array(mem.moment_array("Mz", 401))
        mm = arr[1]
        sag = max(sag, float(-np.min(mm)))
        hog = max(hog, float(np.max(mm)))
        sh = np.array(mem.shear_array("Fy", 401))[1]
        shear = max(shear, float(np.max(np.abs(sh))))
    return sag, hog, shear


def action(position, span, adjacent, kind, magnitude, xi, permanent, ratio=1.0, fraction=0.0):
    lengths, own, cant = layout(position, span, adjacent)
    load = ("U", magnitude) if kind == "U" else ("P", magnitude, xi)
    n = len(lengths)
    sag_on = [True] * n if permanent else [i == own for i in range(n)]
    support = 1 if own == 0 else own
    hog_on = [True] * n if permanent else [i + 1 == support or i == support for i in range(n)]
    a = run(lengths, own, cant, [load if on else None for on in sag_on], ratio, fraction)
    b = a if permanent else run(lengths, own, cant, [load if on else None for on in hog_on], ratio, fraction)
    return a[0], b[1], max(a[2], b[2])


def sign_calibration():
    s, h, _ = run([8.0], 0, False, [("U", 10e3)], 1.0, 0.0)
    return s


if __name__ == "__main__":
    cal = sign_calibration()
    assert abs(cal - 10e3 * 64 / 8) < 1e-3 * cal, ("PyNite Mz sign: sagging must come out positive", cal)
    cases = [
        ("two-span udl permanent", ("end", 10.0, 10.0, "U", 1000.0, 0.5, True)),
        ("two-span mid force permanent", ("end", 10.0, 10.0, "P", 100e3, 0.5, True)),
        ("end span force 3 m of 8 m, neighbour 5 m, variable", ("end", 8.0, 5.0, "P", 100e3, 0.375, False)),
        ("end span force 3 m of 8 m, neighbour 5 m, permanent", ("end", 8.0, 5.0, "P", 100e3, 0.375, True)),
        ("interior span force 2 m of 8 m, neighbours 5 m, variable", ("interior", 8.0, 5.0, "P", 100e3, 0.25, False)),
        ("interior span force 2 m of 8 m, neighbours 5 m, permanent", ("interior", 8.0, 5.0, "P", 100e3, 0.25, True)),
        ("cantilever force 1 m of 2 m, back span 6 m, variable", ("cantilever", 2.0, 6.0, "P", 100e3, 0.5, False)),
        ("cantilever force 1.5 m of 2 m, back span 6 m, permanent", ("cantilever", 2.0, 6.0, "P", 100e3, 0.75, True)),
        ("end span force 6 m of 8 m, neighbour 8 m, variable", ("end", 8.0, 8.0, "P", 100e3, 0.75, False)),
    ]
    for name, (position, span, adjacent, kind, mag, xi, perm) in cases:
        s, h, v = action(position, span, adjacent, kind, mag, xi, perm)
        print(f"{name}: sagging {s:.2f} N m, hogging {h:.2f} N m, shear {v:.2f} N")
    ei1 = 210e9 * I1
    cracked_cases = [
        ("cracked end span force 3 m of 8 m, variable", ("end", 8.0, 6.0, "P", 100e3, 0.375, False)),
        ("cracked interior span force 6 m of 8 m, permanent", ("interior", 8.0, 6.0, "P", 100e3, 0.75, True)),
        ("cracked end span udl permanent", ("end", 8.0, 8.0, "U", 10e3, 0.5, True)),
    ]
    for name, (position, span, adjacent, kind, mag, xi, perm) in cracked_cases:
        s, h, v = action(position, span, adjacent, kind, mag, xi, perm, ratio=0.5, fraction=0.15)
        print(f"{name} (EI2 = 0.5 EI1 over 15 %): sagging {s:.2f} N m, hogging {h:.2f} N m, shear {v:.2f} N")
