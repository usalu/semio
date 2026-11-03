"""🔥️ Independent oracle of the utilisation over time in fire (EN 1993-1-2): the unprotected steel temperature of Eq. (4.25) integrated with scipy's `solve_ivp` (RK45, tight tolerances; the Rust history uses explicit Euler steps of 10 s), the standard fire of EN 1991-1-2 and k_y,θ of Table 3.1 by `numpy.interp`; mu(t) = mu_0 / k_y,theta(t). Run with `python -I`; it prints the JSON that the Rust test `w3_fire_utilisation_matches_the_scipy_oracle` embeds (tolerance 3 % on theta, because Euler with 10 s steps differs from RK45)."""

import json
import math

import numpy as np
from scipy.integrate import solve_ivp

SIGMA, ALPHA_C, RHO = 5.67e-8, 25.0, 7850.0
THETAS = [20, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200]
KY = [1.0, 1.0, 1.0, 1.0, 1.0, 0.78, 0.47, 0.23, 0.11, 0.06, 0.04, 0.02, 0.0]


def gas(t_min):
    return 20.0 + 345.0 * math.log10(8.0 * t_min + 1.0)


def c_a(theta):
    if theta < 600:
        return 425 + 0.773 * theta - 1.69e-3 * theta**2 + 2.22e-6 * theta**3
    if theta < 735:
        return 666 + 13002 / (738 - theta)
    if theta < 900:
        return 545 + 17820 / (theta - 731)
    return 650.0


def steel(am_v, minutes, eps=0.7):
    def rhs(t, y):
        tg, th = gas(t / 60.0), y[0]
        h = ALPHA_C * (tg - th) + eps * SIGMA * ((tg + 273.0) ** 4 - (th + 273.0) ** 4)
        return [max(am_v / (c_a(th) * RHO) * h, 0.0)]

    times = np.arange(0.0, minutes * 60.0 + 1e-9, 60.0)
    return solve_ivp(rhs, (0.0, times[-1]), [20.0], t_eval=times, rtol=1e-8, atol=1e-8, max_step=5.0)


def main():
    am_v, mu0 = 200.0, 0.5
    sol = steel(am_v, 30)
    rows = []
    for t, theta in zip(sol.t, sol.y[0]):
        ky = float(np.interp(theta, THETAS, KY))
        rows.append({"minutes": t / 60.0, "theta": float(theta), "ky": ky, "mu": mu0 / max(ky, 1e-3)})
    print(json.dumps({"am_v": am_v, "mu0": mu0, "rows": rows[::5]}, indent=1))


if __name__ == "__main__":
    main()
