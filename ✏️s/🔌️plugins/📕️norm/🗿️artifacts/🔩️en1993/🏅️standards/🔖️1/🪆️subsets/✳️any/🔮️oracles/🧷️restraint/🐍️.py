"""🧷️ Independent oracle (resisting modulus W_pl,y for the class 1 HEB 240, EN 1993-1-1 6.3.2.1(3)) of the EN 1993-1-1 restraint checks: sine-series Galerkin M_cr with scipy.linalg.eigh (fork supports, optional intermediate fork support by Rayleigh-Ritz with a piecewise-sine basis is not needed here), chi_LT by the Ayrton-Perry root, the closed-form C_1 estimate, L_m of Eq. 6.67. Run with `python3 -I`; prints one JSON object read from the heb240 asset."""
import json
import math
import pathlib

import numpy as np
from scipy import linalg, optimize

ASSET = pathlib.Path(__file__).resolve().parents[2] / "🖼️assets/✅️heb240-compliant/snapshot.json"


def mcr_fork_fork(length, e, g, iz, it, iw, moment, q=None, zg=0.0, n=12):
    pts = 6000
    h = length / pts
    xs = (np.arange(pts) + 0.5) * h
    ks = np.arange(1, n + 1) * np.pi / length
    ke = np.diag(np.concatenate([e * iz * ks**4 * length / 2, (e * iw * ks**4 + g * it * ks**2) * length / 2]))
    s = np.sin(np.outer(ks, xs))
    m = np.array([moment(x) for x in xs])
    cross = -(s * (ks[:, None] ** 2) * m[None, :]) @ s.T * h
    kh = np.zeros((2 * n, 2 * n))
    kh[:n, n:] = cross
    kh[n:, :n] = cross.T
    if q is not None:
        kh[n:, n:] = -(s * np.array([q(x) * zg for x in xs])[None, :]) @ s.T * h
    return float(-1.0 / linalg.eigh(kh, ke, eigvals_only=True).min())


def chi_ayrton_perry(lam, alpha):
    if lam <= 0.2:
        return 1.0
    return float(optimize.brentq(lambda x: (1 - x) * (1 - lam * lam * x) - alpha * (lam - 0.2) * x, 1e-9, 1.0))


def l_m(iz, n_ed, a, c1, wpl, it, fy):
    return 38.0 * iz / math.sqrt(n_ed / a / 57.4 + wpl * wpl / (756.0 * c1 * c1 * a * it) * (fy / 235.0) ** 2)


def main():
    doc = json.loads(ASSET.read_text(encoding="utf8"))
    s, m, mat = doc["sections"][0], doc["members"][0], doc["materials"][0]
    e, g, fy = mat["eModulus"] / 1e6, mat["gModulus"] / 1e6, mat["fy"] / 1e6
    iz, it, iw, wel, wpl, area = s["iz"] * 1e12, s["it"] * 1e12, s["iw"] * 1e18, s["wElY"] * 1e9, s["wPlY"] * 1e9, s["area"] * 1e6
    length = m["length"] * 1e3
    m_ed, gm1 = 100.0e6, 1.1
    parabola = lambda x: x * (length - x) / 2.0
    mcr_udl = mcr_fork_fork(length, e, g, iz, it, iw, parabola) * length**2 / 8.0
    mcr_uniform = mcr_fork_fork(length, e, g, iz, it, iw, lambda x: 1.0)
    mcr_top = mcr_fork_fork(length, e, g, iz, it, iw, parabola, q=lambda x: 1.0, zg=120.0) * length**2 / 8.0
    w_res = wpl
    lam = math.sqrt(w_res * fy / mcr_udl)
    chi = chi_ayrton_perry(lam, 0.21)
    rd = chi * w_res * fy / gm1
    lam_top = math.sqrt(w_res * fy / mcr_top)
    rd_top = chi_ayrton_perry(lam_top, 0.21) * w_res * fy / gm1
    lm = l_m(math.sqrt(iz / area), 0.0, area, 2.752, wpl, it, fy)
    print(json.dumps({"mcr_udl_nmm": mcr_udl, "mcr_uniform_nmm": mcr_uniform, "mcr_udl_top_flange_120_nmm": mcr_top, "lambda_lt": lam, "chi_lt": chi, "m_b_rd_nmm": rd, "util_100knm": m_ed / rd, "util_top_300knm": 300.0e6 / rd_top, "m_b_rd_top_nmm": rd_top, "l_m_mm_c1_2752": lm}))


main()
