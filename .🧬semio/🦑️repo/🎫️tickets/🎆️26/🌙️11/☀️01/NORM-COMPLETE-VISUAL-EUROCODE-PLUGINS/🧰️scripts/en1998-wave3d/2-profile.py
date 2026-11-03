"""EN 1998 wave 3d: independent re-computation of the Annex B profile used by the liquefaction hazard tests (layer governing factors)."""
import math
def n_cs(n, fc):
    if fc <= 5: a, b = 0.0, 1.0
    elif fc < 35: a, b = math.exp(1.76 - 190 / fc ** 2), 0.99 + fc ** 1.5 / 1000
    else: a, b = 5.0, 1.2
    return a + b * n
def crr75(n):
    return math.inf if n >= 30 else 1 / (34 - n) + n / 135 + 50 / (10 * n + 45) ** 2 - 1 / 200
def msf(m): return 10 ** 2.24 / m ** 2.56
def profile(layers, water, mag, ags, c=0.65):
    bottoms = []; t = 0
    for l in layers: t += l[0]; bottoms.append(t)
    def stresses(z):
        top = 0; tot = 0
        for l in layers:
            tot += l[1] * max(0, min(l[0], z - top)); top += l[0]
        tot += layers[-1][1] * max(0, z - top)
        return tot, tot - 9810 * max(0, z - water)
    def factor(z):
        i = next((k for k, b in enumerate(bottoms) if z <= b + 1e-12), len(layers) - 1)
        n, fc = layers[i][3], layers[i][4]
        res = math.inf if n <= 0 else crr75(n_cs(n, fc)) * msf(mag)
        tot, eff = stresses(z)
        csr = c * ags / 9.81 * tot / eff
        return res / csr
    out = []
    for i in range(len(layers)):
        top = 0 if i == 0 else bottoms[i - 1]
        lo = min(max(top + 1e-6, 0.1), bottoms[i])
        pts = [lo + (bottoms[i] - lo) * k / 8 for k in range(9)]
        out.append(min((factor(z), z) for z in pts))
    return out
layer = lambda t, n: (t, 18000.0, 150.0, n, 5.0)
print(profile([layer(2, 0), layer(3, 6), layer(4, 25), layer(5, 8)], 1.0, 7.0, 3.0))
print(profile([layer(2, 0), layer(3, 6), layer(4, 40), layer(5, 8)], 1.0, 7.0, 3.0))
