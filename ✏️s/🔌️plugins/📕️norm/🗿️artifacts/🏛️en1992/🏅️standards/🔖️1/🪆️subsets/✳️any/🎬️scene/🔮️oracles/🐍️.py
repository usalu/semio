"""🔮️ Stdlib-only Python oracle of the EN 1992 scene mechanics: the 1D implicit heat conduction under ISO 834 with the EN 1992-1-2 3.3 concrete properties (checked against the closed-form erfc solution for constant properties), the closed-form bending resistance of a singly reinforced rectangle, the crack spacing of EN 1992-1-1 7.3.4 and the elastic deflection.
Run `python3 -I 🐍️.py` to print the JSON whose numbers the Rust unit tests embed.
"""

import json
import math
import sys

SIGMA = 5.67e-8


def iso834(minutes):
    return 20.0 + 345.0 * math.log10(8.0 * minutes + 1.0)


def conductivity(t):
    x = min(max(t, 20.0), 1200.0) / 100.0
    return 2.0 - 0.2451 * x + 0.0107 * x * x


def specific_heat(t, peak=1470.0):
    if t < 100.0:
        return 900.0
    if t <= 115.0:
        return peak
    if t <= 200.0:
        return peak + (1000.0 - peak) * (t - 115.0) / 85.0
    if t <= 400.0:
        return 1000.0 + (t - 200.0) / 2.0
    return 1100.0


def density(t):
    if t <= 115.0:
        return 2300.0
    if t <= 200.0:
        return 2300.0 * (1.0 - 0.02 * (t - 115.0) / 85.0)
    if t <= 400.0:
        return 2300.0 * (0.98 - 0.03 * (t - 200.0) / 200.0)
    return 2300.0 * (0.95 - 0.07 * (min(t, 1200.0) - 400.0) / 800.0)


def tridiagonal(a, b, c, d):
    n = len(d)
    cc, dd = [0.0] * n, [0.0] * n
    cc[0], dd[0] = c[0] / b[0], d[0] / b[0]
    for i in range(1, n):
        m = b[i] - a[i] * cc[i - 1]
        cc[i] = c[i] / m
        dd[i] = (d[i] - a[i] * dd[i - 1]) / m
    x = [0.0] * n
    x[-1] = dd[-1]
    for i in range(n - 2, -1, -1):
        x[i] = dd[i] - cc[i] * x[i + 1]
    return x


def slab(minutes, depth=0.4, cells=400, dt=2.0, constant=None, gas=iso834, exposed_alpha=None):
    dx = depth / cells
    theta = [20.0] * cells
    t = 0.0
    while t < minutes * 60.0 - 1e-9:
        t += dt
        tg = gas(t / 60.0)
        lam = [constant[0] if constant else conductivity(x) for x in theta]
        cap = [constant[1] if constant else density(x) * specific_heat(x) for x in theta]
        a, b, c, d = [0.0] * cells, [0.0] * cells, [0.0] * cells, [0.0] * cells
        for i in range(cells):
            lw = 0.5 * (lam[i] + lam[i - 1]) if i > 0 else 0.0
            le = 0.5 * (lam[i] + lam[i + 1]) if i < cells - 1 else 0.0
            store = cap[i] * dx / dt
            a[i], c[i], b[i], d[i] = -lw / dx, -le / dx, store + (lw + le) / dx, store * theta[i]
        ks, kg = theta[0] + 273.15, tg + 273.15
        alpha = exposed_alpha if exposed_alpha else 25.0 + 0.7 * SIGMA * (kg * kg + ks * ks) * (kg + ks)
        r = 1.0 / (1.0 / alpha + 0.5 * dx / lam[0])
        b[0] += r
        d[0] += r * tg
        r = 1.0 / (1.0 / 4.0 + 0.5 * dx / lam[-1])
        b[-1] += r
        d[-1] += r * 20.0
        theta = tridiagonal(a, b, c, d)
    return theta, dx


def isotherm_depth(minutes, level=500.0):
    theta, dx = slab(minutes)
    for i in range(len(theta) - 1):
        if theta[i] >= level > theta[i + 1]:
            return (i + 0.5 + (theta[i] - level) / (theta[i] - theta[i + 1])) * dx
    return 0.0


def erfc_check():
    k, c, minutes, surface = 2.0, 2.07e6, 30.0, 520.0
    theta, dx = slab(minutes, depth=0.5, cells=500, dt=1.0, constant=(k, c), gas=lambda _: surface, exposed_alpha=1e9)
    i = int(0.05 / dx)
    exact = 20.0 + (surface - 20.0) * math.erfc((i + 0.5) * dx / (2.0 * math.sqrt(k / c * minutes * 60.0)))
    return {"numeric": theta[i], "closed_form": exact}


def rectangle_resistance(b, h, depth, bar, count, fck_mpa, fyk_mpa, alpha_cc=0.85, hardening=1.08, eps_uk=0.05, strips=20000):
    """🧱️ Bending resistance at N = 0 by strain compatibility (SI): parabola-rectangle concrete, steel with the inclined top branch to k fyd at 0.9 eps_uk, bisection on the neutral axis depth."""
    area = count * math.pi * bar ** 2 / 4.0
    d = depth
    fcd, fyd, es = alpha_cc * fck_mpa / 1.5 * 1e6, fyk_mpa / 1.15 * 1e6, 200e9
    ec2, ecu2, eud = 2.0, 3.5, 0.9 * eps_uk * 1000.0

    def concrete(e):
        return 0.0 if e <= 0.0 else fcd * (1.0 - (1.0 - e / ec2) ** 2) if e < ec2 else fcd

    def steel(e):
        a, ey = abs(e) / 1000.0, fyd / es
        s = es * a if a <= ey else fyd + (hardening - 1.0) * fyd * (min(a, eud / 1000.0) - ey) / (eud / 1000.0 - ey)
        return math.copysign(s, e)

    def state(x):
        top = min(ecu2, eud * x / (d - x)) if x < d else ecu2
        du = min(x, h) / strips
        force = moment = 0.0
        for i in range(strips):
            u = (i + 0.5) * du
            f = b * du * concrete(top * (x - u) / x)
            force += f
            moment += f * (d - u)
        return force, moment, area * steel(top * (d - x) / x)

    lo, hi = 1e-4, d
    for _ in range(80):
        mid = 0.5 * (lo + hi)
        force, _, tension = state(mid)
        lo, hi = (mid, hi) if force < tension else (lo, mid)
    return state(0.5 * (lo + hi))[1], d, area


def crack_spacing(b, h, d, area, cover, bar, ecm, es=200e9):
    ae_rho = es / ecm * area / (b * d)
    x = d * ae_rho * (-1.0 + math.sqrt(1.0 + 2.0 / ae_rho))
    hc = min(2.5 * (h - d), (h - x) / 3.0, h / 2.0)
    rho = area / (b * hc)
    return 3.4 * cover + 0.8 * 0.5 * 0.425 * bar / rho, x


def simply_supported_deflection(q, span, ecm, b, h):
    return 5.0 * q * span ** 4 / (384.0 * ecm * b * h ** 3 / 12.0)


def expected():
    mrd, d, area = rectangle_resistance(0.30, 0.50, 0.450, 0.020, 4, 30.0, 500.0)
    ecm = 33.0e9
    sr, x = crack_spacing(0.30, 0.50, d, area, 0.035, 0.020, ecm)
    return {
        "erfc_check": erfc_check(),
        "isotherm_depth_m": {str(t): isotherm_depth(float(t)) for t in (30, 60, 90, 120)},
        "iso834": {str(t): iso834(float(t)) for t in (30, 60, 90, 120)},
        "beam_b1": {"mrd_nm": mrd, "d_m": d, "as_m2": area, "ecm_pa": ecm, "sr_max_m": sr, "x_cracked_m": x, "w_ss_m": simply_supported_deflection(20.0e3, 6.0, ecm, 0.30, 0.50)},
    }


if __name__ == "__main__":
    print(json.dumps(expected(), indent=2, sort_keys=True))
    sys.exit(0)
