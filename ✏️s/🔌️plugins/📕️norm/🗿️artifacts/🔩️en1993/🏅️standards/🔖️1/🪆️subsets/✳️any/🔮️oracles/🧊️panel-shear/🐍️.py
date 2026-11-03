"""🧊️ Independent oracle of the web panel shear buckling of EN 1993-1-5 chapter 5: (1) the elastic shear buckling coefficient of the simply supported panel by the Rayleigh-Ritz method on the sine basis solved with `scipy.linalg.eigh` (generalised problem), compared with the classical Southwell-Skan values and the closed form 5.34 + 4/alpha^2; (2) the Annex A.3 k_tau with a longitudinal stiffener, chi_w of Table 5.1 (non-rigid end post) and V_b,Rd = V_bw,Rd + V_bf,Rd by closed forms, with `structuralcodes` absent for chapter 5 so the closed forms are the independent second implementation. Run with `python -I`; it prints the JSON that the Rust unit tests embed."""

import json
import math

import numpy as np
from scipy.linalg import eigh

TERMS = 8
CLASSICAL = {1.0: 9.34, 1.5: 7.1, 2.0: 6.6, 3.0: 5.9}


def ritz(alpha, terms=TERMS):
    a, b = alpha, 1.0
    idx = [(m, n) for m in range(1, terms + 1) for n in range(1, terms + 1)]
    size = len(idx)
    k = np.zeros((size, size))
    g = np.zeros((size, size))
    for i, (m, n) in enumerate(idx):
        k[i, i] = math.pi**4 * a * b / 4 * ((m / a) ** 2 + (n / b) ** 2) ** 2
        for j, (p, q) in enumerate(idx):
            if (m + p) % 2 == 1 and (n + q) % 2 == 1:
                g[i, j] = 4 * m * q * p * n / ((p * p - m * m) * (n * n - q * q))
    values = eigh(g + g.T, k, eigvals_only=True)
    return b * b / (math.pi**2 * values.max())


def k_tau(a, hw, t, isl):
    base = 5.34 + 4 * (hw / a) ** 2 if a / hw >= 1 else 4.0 + 5.34 * (hw / a) ** 2
    return base + max(9 * (hw / a) ** 2 * (isl / (t**3 * hw)) ** 0.75, 2.1 / t * (isl / hw) ** (1 / 3))


def panel(a, hw, t, fy, isl, flange=None, gm1=1.0):
    eps, eta = math.sqrt(235 / fy), 1.2 if fy <= 460 else 1.0
    k = k_tau(a, hw, t, isl)
    lam = hw / (37.4 * t * eps * math.sqrt(k))
    chi = eta if lam < 0.83 / eta else (0.83 / lam if lam < 1.08 else 0.83 / lam)
    vbw = chi * fy * hw * t / (math.sqrt(3) * gm1)
    vbf = 0.0
    if flange:
        bf, tf, fyf, mf, med = flange
        c = a * (0.25 + 1.6 * bf * tf * tf * fyf / (t * hw * hw * fy))
        vbf = bf * tf * tf * fyf / (c * gm1) * max(1 - (med / mf) ** 2, 0.0)
    cap = eta * fy * hw * t / (math.sqrt(3) * gm1)
    return {"k_tau": k, "lambda_w": lam, "chi_w": chi, "v_bw": vbw, "v_bf": vbf, "cap": cap, "v_b": min(vbw + vbf, cap)}


def main():
    mf = 300 * 20 * 1020 * 355
    out = {
        "ritz": {str(al): {"k": ritz(al), "classical": c, "closed": 5.34 + 4 / al**2} for al, c in CLASSICAL.items()},
        "panel": panel(2000, 1000, 8, 355, 0.0),
        "panel_flange": panel(2000, 1000, 8, 355, 0.0, (300, 20, 355, mf, 0.5 * mf)),
        "panel_stiffened_k": k_tau(2000, 1000, 8, 2.0e7),
    }
    for al, row in out["ritz"].items():
        assert abs(row["k"] - row["classical"]) / row["classical"] < 0.02, (al, row)
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
