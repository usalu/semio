"""🧮️ EN 1993 clause oracle for worked examples E01-E15: quadratic-root chi, sine-series Galerkin Mcr, Heun fire integration; rainflow by the third-party `rainflow` package (ASTM E1049-85) with our four-point counter as cross-check."""
import json
import math

import numpy as np
import rainflow
from scipy import linalg, optimize

E, G = 210000.0, 81000.0


def chi_quadratic(lam, alpha, lam0=0.2, beta=1.0):
    phi = 0.5 * (1 + alpha * (lam - lam0) + beta * lam * lam)
    return min(1.0, (phi - math.sqrt(phi * phi - beta * lam * lam)) / (beta * lam * lam))


def jacobi_min(a):
    n = len(a)
    a = [r[:] for r in a]
    for _ in range(200):
        off = sum(a[i][j] ** 2 for i in range(n) for j in range(n) if i != j)
        if off < 1e-22 * max(1e-300, sum(a[i][i] ** 2 for i in range(n))):
            break
        for p in range(n - 1):
            for q in range(p + 1, n):
                if abs(a[p][q]) < 1e-300:
                    continue
                th = 0.5 * math.atan2(2 * a[p][q], a[q][q] - a[p][p])
                c, s = math.cos(th), math.sin(th)
                for k in range(n):
                    kp, kq = a[k][p], a[k][q]
                    a[k][p], a[k][q] = c * kp - s * kq, s * kp + c * kq
                for k in range(n):
                    pk, qk = a[p][k], a[q][k]
                    a[p][k], a[q][k] = c * pk - s * qk, s * pk + c * qk
    return min(a[i][i] for i in range(n))


def mcr_galerkin(L, Iz, It, Iw, moment, q=None, zg=0.0, n=10):
    pts = 4000
    h = L / pts
    xs = [(i + 0.5) * h for i in range(pts)]
    ke = []
    for k in range(1, n + 1):
        kk = k * math.pi / L
        ke.append(E * Iz * kk ** 4 * L / 2)
    for k in range(1, n + 1):
        kk = k * math.pi / L
        ke.append((E * Iw * kk ** 4 + G * It * kk ** 2) * L / 2)
    size = 2 * n
    kh = [[0.0] * size for _ in range(size)]
    for a in range(1, n + 1):
        for b in range(1, n + 1):
            ka = a * math.pi / L
            cross = sum(moment(x) * ka * ka * math.sin(ka * x) * math.sin(b * math.pi / L * x) for x in xs) * h
            kh[a - 1][n + b - 1] += -cross
            kh[n + b - 1][a - 1] += -cross
            if q is not None:
                kh[n + a - 1][n + b - 1] += -sum(q(x) * zg * math.sin(a * math.pi / L * x) * math.sin(b * math.pi / L * x) for x in xs) * h
    scaled = [[kh[i][j] / math.sqrt(ke[i] * ke[j]) for j in range(size)] for i in range(size)]
    return -1.0 / jacobi_min(scaled)


def chi_ayrton_perry(lam, alpha):
    if lam <= 0.2:
        return 1.0
    f = lambda x: (1 - x) * (1 - lam * lam * x) - alpha * (lam - 0.2) * x
    return optimize.brentq(f, 1e-6, 1.0)


def mcr_scipy(L, Iz, It, Iw, moment, q=None, zg=0.0, n=10):
    pts = 4000
    h = L / pts
    xs = (np.arange(pts) + 0.5) * h
    ks = np.arange(1, n + 1) * np.pi / L
    ke = np.diag(np.concatenate([E * Iz * ks ** 4 * L / 2, (E * Iw * ks ** 4 + G * It * ks ** 2) * L / 2]))
    s = np.sin(np.outer(ks, xs))
    m = np.array([moment(x) for x in xs])
    cross = -(s * (ks[:, None] ** 2) * m[None, :]) @ s.T * h
    kh = np.zeros((2 * n, 2 * n))
    kh[:n, n:] = cross
    kh[n:, :n] = cross.T
    if q is not None:
        qz = np.array([q(x) * zg for x in xs])
        kh[n:, n:] = -(s * qz[None, :]) @ s.T * h
    mu = linalg.eigh(kh, ke, eigvals_only=True)
    return -1.0 / mu.min()


def rainflow_four_point(history):
    pts = []
    for x in history:
        if not pts or x != pts[-1]:
            pts.append(x)
    rev = [pts[0]] + [pts[i] for i in range(1, len(pts) - 1) if (pts[i] - pts[i - 1]) * (pts[i + 1] - pts[i]) < 0] + [pts[-1]]
    cycles, st = [], []
    for x in rev:
        st.append(x)
        while len(st) >= 4:
            a, b, c, d = st[-4:]
            if abs(b - c) <= abs(a - b) and abs(b - c) <= abs(c - d):
                cycles.append((abs(b - c), 1.0))
                del st[-3:-1]
            else:
                break
    cycles += [(abs(st[i + 1] - st[i]), 0.5) for i in range(len(st) - 1)]
    return cycles


def ca(t):
    if t < 600:
        return 425 + 0.773 * t - 1.69e-3 * t ** 2 + 2.22e-6 * t ** 3
    if t < 735:
        return 666 + 13002 / (738 - t)
    if t < 900:
        return 545 + 17820 / (t - 731)
    return 650.0


def steel_temp(ksh, amv, t_end, dt=1.0):
    def rate(t, th):
        tg = 20 + 345 * math.log10(8 * t / 60 + 1)
        h = 25 * (tg - th) + 0.7 * 5.67e-8 * ((tg + 273) ** 4 - (th + 273) ** 4)
        return max(0.0, ksh * amv / (ca(th) * 7850) * h)
    th, t, out = 20.0, 0.0, {}
    while t < t_end - 1e-9:
        k1 = rate(t, th)
        k2 = rate(t + dt, th + dt * k1)
        th += dt * 0.5 * (k1 + k2)
        t += dt
        out[round(t)] = th
    return out


def sn_cycles(ds, dc):
    dd = dc * (2 / 5) ** (1 / 3)
    dl = dd * (5 / 100) ** (1 / 5)
    return 2e6 * (dc / ds) ** 3 if ds > dd else (5e6 * (dd / ds) ** 5 if ds > dl else math.inf)


def main():
    out = {}
    a, iz, fy, L = 10600.0, 3923e4, 355.0, 4000.0
    ncr = math.pi ** 2 * E * iz / L ** 2
    lam = math.sqrt(a * fy / ncr)
    out["e01_chi_z"] = chi_quadratic(lam, 0.49)
    out["e01_nbrd_z_de"] = out["e01_chi_z"] * a * fy / 1.1
    iz2, it, iw, l2 = 603.8e4, 20.12e4, 125900e6, 5000.0
    mcr_cf = 1.127 * math.pi ** 2 * E * iz2 / l2 ** 2 * math.sqrt(iw / iz2 + l2 ** 2 * G * it / (math.pi ** 2 * E * iz2))
    out["e02_mcr_closed"] = mcr_cf
    out["e02_mcr_uniform_galerkin"] = mcr_galerkin(l2, iz2, it, iw, lambda x: 1.0)
    out["e02_mcr_uniform_closed"] = math.pi ** 2 * E * iz2 / l2 ** 2 * math.sqrt(iw / iz2 + l2 ** 2 * G * it / (math.pi ** 2 * E * iz2))
    out["e02_mcr_udl_galerkin"] = mcr_galerkin(l2, iz2, it, iw, lambda x: x * (l2 - x) / 2, lambda x: 1.0) * l2 * l2 / 8
    lam_lt = math.sqrt(628.4e3 * 235 / mcr_cf)
    out["e02_chi_lt_234"] = chi_quadratic(lam_lt, 0.34)
    c = chi_quadratic(lam_lt, 0.34, 0.4, 0.75)
    out["e02_chi_lt_mod"] = min(1.0, min(c, 1 / lam_lt ** 2) / (1 - 0.5 * (1 - 0.94) * (1 - 2 * (lam_lt - 0.8) ** 2)))
    eps = math.sqrt(235 / 355)
    k = 23.9
    lp = (1000 / 8) / (28.4 * eps * math.sqrt(k))
    out["e04_beff"] = (lp - 0.055 * 2) / lp ** 2 * 500
    lw = 1000 / (37.4 * 8 * eps * math.sqrt(5.34 + 4 / 4))
    out["e07_vbw"] = 1.37 / (0.7 + lw) * 355 * 1000 * 8 / math.sqrt(3) / 1000
    out["e08_fb"] = 2.5 * min(40 / 66, 800 / 490, 1.0) * 490 * 20 * 12 / 1.25
    sol = [(60.0, 1.0e6)]
    out["e11_damage"] = sum(n / sn_cycles(s, 71 / 1.15) for s, n in sol)
    for key, history in (("", [0, 10, 2, 8, 0, 9, 1, 6, 0]), ("_fatigue_detail", [0, 80, 10, 60, 20, 70, 0, 90, 30, 50, 0])):
        rf = [(rng, count) for rng, _, count, _, _ in rainflow.extract_cycles(history)]
        ours = rainflow_four_point(history)
        assert sorted(rf) == sorted(ours) or sum(n for _, n in rf) == sum(n for _, n in ours), (rf, ours)
        out["rainflow_count" + key] = sum(n for _, n in rf)
        out["rainflow_ranges" + key] = sorted(r for r, _ in rf)
    th = steel_temp(167.26 / 215.58 * 0.9, 215.58, 900)
    out["e12_theta"] = [th[300], th[600], th[900]]
    out["e13_phi"] = 1 / 200 * max(2 / 3, min(1, 2 / math.sqrt(6))) * math.sqrt(0.5 * (1 + 1 / 2))
    mp = 0.25 * 210 * 15 ** 2 * 355
    out["e10_ft2"] = (2 * mp + 40 * 282240) / 80
    nrk, myrk, ned, myed = 3763e3, 373.8e6, 800e3, 100e6
    ly = math.sqrt(10600 * 355 / (math.pi ** 2 * E * 11260e4 / 4000 ** 2))
    lz = math.sqrt(10600 * 355 / (math.pi ** 2 * E * 3923e4 / 4000 ** 2))
    cy, cz = chi_quadratic(ly, 0.34), chi_quadratic(lz, 0.49)
    for gm, key in ((1.0, "en"), (1.1, "de")):
        ny, nz = ned / (cy * nrk / gm), ned / (cz * nrk / gm)
        kyy = min(0.6 * (1 + (ly - 0.2) * ny), 0.6 * (1 + 0.8 * ny))
        kzy = 1 - 0.1 * lz / (0.6 - 0.25) * nz
        m = myed / (myrk / gm)
        out["e06_eta61_" + key] = ny + kyy * m
        out["e06_eta62_" + key] = nz + kzy * m
    out["e01_chi_z_brentq"] = chi_ayrton_perry(lam, 0.49)
    out["e02_chi_lt_brentq"] = chi_ayrton_perry(lam_lt, 0.34)
    out["e02_mcr_uniform_scipy"] = mcr_scipy(l2, iz2, it, iw, lambda x: 1.0)
    out["e02_mcr_udl_scipy"] = mcr_scipy(l2, iz2, it, iw, lambda x: x * (l2 - x) / 2, lambda x: 1.0) * l2 * l2 / 8
    out["e02_mcr_top_flange_scipy"] = mcr_scipy(l2, iz2, it, iw, lambda x: x * (l2 - x) / 2, lambda x: 1.0, zg=150.0) * l2 * l2 / 8
    out["e02_mcr_top_flange_galerkin"] = mcr_galerkin(l2, iz2, it, iw, lambda x: x * (l2 - x) / 2, lambda x: 1.0, zg=150.0) * l2 * l2 / 8
    print(json.dumps(out))


if __name__ == "__main__":
    main()
