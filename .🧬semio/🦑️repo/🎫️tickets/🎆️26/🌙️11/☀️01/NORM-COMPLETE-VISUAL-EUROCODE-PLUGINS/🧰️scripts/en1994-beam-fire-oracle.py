"""Independent cross-check of the Rust catalogue module `beam_fire` (EN 1994-1-2 composite beam): scipy method-of-lines on a converged grid for a slab column away from the steel flange (pure 1D conduction from the soffit), a numpy replica of the two-dimensional explicit scheme, the lumped protected steel temperature (stdlib oracle RK4 against the Rust one-second stepping) and the reduced plastic moment by numeric strip integration of the stdlib oracle (`sagging_moment`). The values are hard-coded in the Rust tests. Property tables are the unverified ones of the Rust catalogue (`ndp.en1994.1-2.*`)."""
import importlib.util, json, math, pathlib, sys
import numpy as np
from scipy.integrate import solve_ivp

ROOT = pathlib.Path(__file__).resolve().parents[8]
SPEC = next((ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994").rglob("🔮️oracles/🧮️catalogue/🐍️.py"))
_s = importlib.util.spec_from_file_location("cat_oracle", SPEC)
cat = importlib.util.module_from_spec(_s)
_s.loader.exec_module(cat)

SIGMA, ALPHA_C, EPS, ALPHA_U, RHO = 5.67e-8, 25.0, 0.7, 9.0, 2300.0


def iso(t):
    return 20 + 345 * math.log10(8 * t / 60 + 1)


def lam(th):
    t = np.clip(th, 20, 1200) / 100
    return 1.36 - 0.136 * t + 0.0057 * t * t


def cp(th):
    return np.where(th <= 100, 900.0, np.where(th <= 200, 900 + (th - 100), np.where(th <= 400, 1000 + (th - 200) / 2, 1100.0)))


def flux(g, th):
    return ALPHA_C * (g - th) + EPS * SIGMA * ((g + 273) ** 4 - (th + 273) ** 4)


def surface(g, th, l, half):
    d = g - th
    return np.where(np.abs(d) < 1e-9, 0.0, d / (d / np.where(np.abs(d) < 1e-9, 1.0, flux(g, th)) + half / l))


KC = [(20, 1.0), (100, 1.0), (200, .95), (300, .85), (400, .75), (500, .60), (600, .45), (700, .30), (800, .15), (900, .08), (1000, .04), (1100, .01), (1200, 0.0)]


def kc(th):
    return float(np.interp(th, [a for a, _ in KC], [b for _, b in KC]))


def column_1d(ys, ends, n=75):
    d = ys / n

    def f(t, th):
        l, c = lam(th), RHO * cp(th)
        k = 2 * l[:-1] * l[1:] / (l[:-1] + l[1:])
        q = np.zeros(n)
        fl = k * (th[1:] - th[:-1]) / d
        q[:-1] += fl
        q[1:] -= fl
        q[0] += surface(iso(t), th[0], l[0], d / 2)
        q[-1] += (20 - th[-1]) / (1 / ALPHA_U + d / 2 / l[-1])
        return q / (c * d)

    sol = solve_ivp(f, (0, max(ends)), np.full(n, 20.0), method="BDF", t_eval=ends, rtol=1e-8, atol=1e-6)
    xs = (np.arange(n) + 0.5) * d
    return xs, sol.y.T


def replica(ys, half, flange_half, steel, ends):
    nx, ny = min(max(math.ceil(half / 0.025), 4), 80), min(max(math.ceil(ys / 0.01), 4), 40)
    dx, dy = half / nx, ys / ny
    onf = (np.arange(nx) + 0.5) * dx <= flange_half
    th = np.full((ny, nx), 20.0)
    out, t = [], 0.0
    for end in ends:
        while t < end - 1e-9:
            step = min(1.0, end - t)
            g, ts = iso(t + step / 2), steel[min(int(round(t + step / 2)), len(steel) - 1)]
            l, c = lam(th), RHO * cp(th)
            q = np.zeros_like(th)
            kx = 2 * l[:, :-1] * l[:, 1:] / (l[:, :-1] + l[:, 1:]) * (th[:, 1:] - th[:, :-1]) / dx * dy
            q[:, :-1] += kx
            q[:, 1:] -= kx
            ky = 2 * l[:-1] * l[1:] / (l[:-1] + l[1:]) * (th[1:] - th[:-1]) / dy * dx
            q[:-1] += ky
            q[1:] -= ky
            q[0] += np.where(onf, l[0] * (ts - th[0]) / (dy / 2) * dx, surface(g, th[0], l[0], dy / 2) * dx)
            q[-1] += (20 - th[-1]) / (1 / ALPHA_U + dy / 2 / l[-1]) * dx
            th = th + q * step / (c * dx * dy)
            t += step
        out.append(th.copy())
    return out, (nx, ny, dx, dy)


def steel_history(end, am_v, d_p, lam_p=0.12, c_p=1000.0, rho_p=800.0):
    def ca(th):
        if th < 600:
            return 425 + 0.773 * th - 1.69e-3 * th ** 2 + 2.22e-6 * th ** 3
        if th < 735:
            return 666 + 13002 / (738 - th)
        if th < 900:
            return 545 + 17820 / (th - 731)
        return 650
    phi = c_p * rho_p / (ca(20) * 7850) * d_p * am_v
    th, hist = 20.0, [20.0]
    for k in range(int(end)):
        g0, g1 = iso(k), iso(k + 1)
        dv = (lam_p / d_p) * am_v / (ca(th) * 7850) * (g0 - th) / (1 + phi / 3) - (math.exp(phi / 10) - 1) * (g1 - g0)
        if g0 - th > 0:
            dv = max(dv, 0.0)
        th += dv
        hist.append(th)
    return hist


KY = [1.0, 1.0, 1.0, 1.0, 1.0, 0.78, 0.47, 0.23, 0.11, 0.06, 0.04, 0.02, 0.0]


def ky(th):
    t = min(max(th, 20.0), 1200.0)
    if t <= 100:
        return 1.0
    i = min(int(t // 100), 11)
    return KY[i] + (t - 100 * i) / 100 * (KY[i + 1] - KY[i])


def main():
    sec = cat.ipe300()
    ys, hc, beff, fy, fcc, am_v, board, half_f = 0.15, 0.10, 2.4, 355e6, 0.85 * 30e6, 150.0, 0.02, 0.075
    ends = [1800.0, 3600.0, 5400.0, 7200.0]
    hist = steel_history(7200, am_v, board)
    fields, (nx, ny, dx, dy) = replica(ys, beff / 2, half_f, hist, ends)
    xs, fine = column_1d(ys, ends)
    res = {"grid": [nx, ny], "steel": [hist[int(e)] for e in ends]}
    far, rebar_row = nx - 1, int((ys - 0.04) / dy)
    res["soffit_far"] = [float(f[0, far]) for f in fields]
    res["mid_far"] = [float(f[ny // 2, far]) for f in fields]
    res["rebar_row_mean"] = [float(f[rebar_row].mean()) for f in fields]
    res["top_row_mean"] = [float(f[-1].mean()) for f in fields]
    res["under_flange_soffit"] = [float(f[0, 0]) for f in fields]
    res["fine_vs_replica_max_abs_k"] = max(abs(float(np.interp((j + 0.5) * dy, xs, fine[k])) - float(fields[k][j, far])) for k in range(4) for j in range(ny))
    res["fine_soffit_k"] = [float(np.interp(0.5 * dy, xs, fine[k])) for k in range(4)]
    res["rk4_steel_60"] = cat.protected_temperature(3600.0, am_v, board, 0.12, 1000.0, 800.0)
    rows = []
    for k, e in enumerate(ends):
        ta = hist[int(e)]
        fyk = fy * ky(ta)
        m0, nc, x = cat.sagging_moment(sec, fyk, ys, beff, hc, fcc, 1e30)
        kcm = float(np.mean([kc(float(v)) for j in range(ny - max(1, math.ceil(x / dy)), ny) for v in fields[k][j]]))
        m1, _, _ = cat.sagging_moment(sec, fyk, ys, beff, hc, fcc * kcm, 1e30)
        rows.append([ta, ky(ta), kcm, m0, m1])
    res["reduced"] = rows
    m20, _, _ = cat.sagging_moment(sec, fy, ys, beff, hc, fcc, 1e30)
    res["m_pl_cold"] = m20
    json.dump(res, sys.stdout, indent=1)


if __name__ == "__main__":
    main()
