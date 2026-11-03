"""EN 1998-1 5.4.3 / 5.5.3 critical region worked example with numpy and scipy (stand-alone, no repository code): the confinement alpha omega_wd of Eq. (5.15), the detailing limits of a DCH column, and the largest hoop spacing that still satisfies Eq. (5.15) found with scipy.optimize.brentq (the kernel bisects). Run: /home/deck/semio/.venv/bin/python -I 1-critical-region-oracle.py"""
import numpy as np
from scipy.optimize import brentq

bc, hc, lcl = 0.4, 0.5, 3.0
b0, h0 = 0.32, 0.42
n_ed, fcd, fyd, fydw = 800e3, 20.0, 434.8, 434.8
dbl, dblmax, dbw, s = 0.016, 0.02, 0.010, 0.08
legs_x, legs_y, n_bars, b_i = 3, 3, 8, 0.11
q0, t1, tc, es = 4.5, 0.6, 0.5, 200000.0


def alpha(spacing):
    an = 1 - np.sum(np.full(n_bars, b_i) ** 2) / (6 * b0 * h0)
    return an * (1 - spacing / (2 * b0)) * (1 - spacing / (2 * h0))


def omega(spacing):
    volume_per_hoop = np.pi * dbw ** 2 / 4 * (legs_x * b0 + legs_y * h0)
    return volume_per_hoop / (b0 * h0 * spacing) * fydw / fcd


mu_phi = 2 * q0 - 1 if t1 >= tc else 1 + 2 * (q0 - 1) * tc / t1
nu_d = n_ed / (bc * hc * fcd * 1e6)
required = 30 * mu_phi * nu_d * fyd / es * bc / b0 - 0.035
print("alpha", alpha(s), "omega_wd", omega(s), "alpha*omega", alpha(s) * omega(s))
print("nu_d", nu_d, "mu_phi", mu_phi, "required", required)
print("l_cr", max(max(bc, hc), lcl / 6, 0.45), "s_max DCH", min(min(b0, h0) / 3, 0.125, 6 * dbl), "d_bw min DCH", max(0.006, 0.4 * dblmax * np.sqrt(fyd / fydw)))
print("rho_tot", 0.0048 / (bc * hc))
print("largest spacing with alpha*omega >= 0.40", brentq(lambda x: alpha(x) * omega(x) - 0.40, 0.005, 0.5))
