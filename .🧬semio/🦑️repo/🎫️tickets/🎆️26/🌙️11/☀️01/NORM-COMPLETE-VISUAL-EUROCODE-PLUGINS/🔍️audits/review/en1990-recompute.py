import math
from statistics import NormalDist

PSI = {"office": (0.7, 0.5, 0.3), "wind": (0.6, 0.2, 0.0), "snow": (0.5, 0.2, 0.0), "storage": (1.0, 0.9, 0.8)}

def combos(G_sup, G_inf, Q, eq, ksf=1.0, gG=1.35, gGi=1.0, gQ=1.5, xi=0.85, lead_psi1=True):
    out = []
    names = list(Q)
    leads = names if eq not in ("QP", "E612b") else [None]
    for lead in leads:
        tot = 0.0
        pg = {"610": gG, "610a": gG, "610b": gG * xi}.get(eq, 1.0)
        ultimate = eq in ("610", "610a", "610b")
        tot += sum(G_sup) * (ksf * pg if ultimate else 1.0) + sum(G_inf) * (gGi if ultimate else 1.0)
        for n, v in Q.items():
            p0, p1, p2 = PSI[n]
            lead_ = n == lead
            if eq in ("610", "610b"):
                f = ksf * gQ * (1.0 if lead_ else p0)
            elif eq == "610a":
                f = ksf * gQ * p0
            elif eq == "611b":
                f = (p1 if lead_ else p2)
            elif eq == "char":
                f = 1.0 if lead_ else p0
            elif eq == "freq":
                f = p1 if lead_ else p2
            else:
                f = p2
            tot += f * v
        out.append((lead, tot))
    return out

G = 50.0
Q = {"office": 20.0, "wind": 10.0, "snow": 8.0}
print("E90-01..03 max over leads, 6.10:", max(t for _, t in combos([G], [], Q, "610")))
print("6.10a:", max(t for _, t in combos([G], [], Q, "610a")), "6.10b:", max(t for _, t in combos([G], [], Q, "610b")))
for eq in ("char", "freq"):
    print(eq, max(t for _, t in combos([G], [], Q, eq)))
print("QP", combos([G], [], Q, "QP")[0][1])
print("accidental Ad=100:", 100.0 + max(t for _, t in combos([G], [], Q, "611b")))
print("seismic AEd=40:", 40.0 + combos([G], [], Q, "E612b")[0][1])
print("EQU stb/dst:", 0.9 * 400, 1.1 * 120 + 1.5 * 60)

print("--- defect demo A: g_sup with negative influence (relieving action flagged g_sup)")
# e.g. continuous beam support moment, permanent load in an unloaded span gives -40, loaded span +100
unsafe = 1.35 * 100 + 1.35 * (-40) + 1.5 * 20
correct = 1.35 * 100 + 1.00 * (-40) + 1.5 * 20
print("code (gamma_sup on the negative-effect action):", unsafe, "correct (gamma_inf on the relieving part):", correct, "unsafe by", correct - unsafe)

print("--- defect demo B: variable with negative effect dropped, reverse direction never checked")
G_eff, W_up = 30.0, -80.0
print("code E_d (+direction) =", 1.0 * G_eff, " ; uplift direction E_d,min = 1.0*30 + 1.5*(-80) =", 1.0 * G_eff + 1.5 * W_up, "(never evaluated)")

print("--- reliability checks")
N = NormalDist()
for b in (3.3, 3.8, 4.3, 4.2, 4.7, 5.2):
    print("beta", b, "pf", N.cdf(-b))
for rc, b1, b50 in ((1, 4.2, 3.3), (2, 4.7, 3.8), (3, 5.2, 4.3)):
    p1 = N.cdf(-b1)
    p50 = 1 - (1 - p1) ** 50
    print("RC", rc, "beta50 from beta1 =", -N.inv_cdf(p50), "table", b50)

print("--- erfc series/continued fraction (port of reliability-math) vs math.erfc")
def erf_series(x):
    x2, term, s, n = x * x, x, x, 0.0
    while n < 200:
        n += 1
        term *= -x2 / n
        add = term / (2 * n + 1)
        s += add
        if abs(add) < 1e-17 * abs(s):
            break
    return 2 / math.sqrt(math.pi) * s
def erfc_tail(x):
    t = x
    for k in range(120, 0, -1):
        t = x + (k / 2) / t
    return math.exp(-x * x) / (math.sqrt(math.pi) * t)
def erfc(x):
    if x < 0:
        return 2 - erfc(-x)
    if x < 2:
        return 1 - erf_series(x)
    return erfc_tail(x)
worst = 0
for x in [0.1 * i for i in range(0, 80)]:
    a, b = erfc(x), math.erfc(x)
    worst = max(worst, abs(a - b) / b)
    if x in (1.9, 2.0, 2.1, 3.0, 5.0, 7.0):
        print("x", x, "rel err", abs(a - b) / b)
print("max rel err 0..7.9:", worst)

print("--- S-N curve (EN 1993-1-9 style) check")
dc = 71.0
dd = (2 / 5) ** (1 / 3) * dc
dl = (5 / 100) ** (1 / 5) * dd
print("deltaC 71 -> deltaD", dd, "deltaL", dl, "(EN 1993-1-9 gives 52.4 and 28.8 for 71)")

print("--- Annex C alphas and R_d/E_d fixed alphas")
mu_r, sd_r, mu_e, sd_e, bt = 200.0, 20.0, 100.0, 20.0, 3.8
ar, ae = 0.8, -0.7
print("R_d =", mu_r - ar * bt * sd_r, "E_d =", mu_e - ae * bt * sd_e)

print("--- Annex D k_n interpolation (linear in 1/n), n=7, V unknown")
t_n = [6, 8]
kn = [2.18, 2.00]
inv = 1 / 7
t = (inv - 1 / 6) / (1 / 8 - 1 / 6)
print("k_n(7) =", kn[0] + t * (kn[1] - kn[0]))
print("normal: Xk = m(1-kn V) n=6 V=0.08 m=100:", 100 * (1 - 2.18 * 0.08))
V = 0.1
sy = math.sqrt(math.log(1 + V * V))
print("lognormal Xk n=6, V=0.1, m=100:", math.exp(math.log(100) - 0.5 * sy * sy - 2.18 * sy))
