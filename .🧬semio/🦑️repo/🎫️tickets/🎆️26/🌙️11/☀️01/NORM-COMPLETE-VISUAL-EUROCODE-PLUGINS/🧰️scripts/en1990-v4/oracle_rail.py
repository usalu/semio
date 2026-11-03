"""Third-party oracle (scipy) of the EN 1990 v4 rail resonance estimate: simply supported deck, analytic sine modes, HSLM-A1 train of the registry (unverified), modal ODEs integrated with scipy DOP853. Prints the reference peak accelerations the Rust test `the_modal_superposition_matches_the_scipy_oracle` compares against (3 % tolerance), the Fig. 6.10 limits and the E90.10 and override arithmetic. Usage: python -I oracle_rail.py"""
import json, math, numpy as np
from scipy.integrate import solve_ivp

L, MASS, F0, ZETA = 20.0, 5000.0, 4.0, 0.01
TRAIN = dict(coaches=18, D=18.0, d=2.0, P=170e3)
MODES = 3


def axles(t):
    out = []
    for bogie in range(t["coaches"] + 1):
        c = bogie * t["D"]
        out += [c - t["d"] / 2 + t["d"] / 2, c + t["d"] / 2 + t["d"] / 2]
    return out


def peak(speed, zeta, train=TRAIN):
    ax = axles(train)
    train_len = ax[-1]
    t_end = (L + train_len) / speed + 1.0
    mg = MASS * L / 2.0

    def rhs(t, y):
        dy = np.zeros_like(y)
        for n in range(1, MODES + 1):
            w = 2 * math.pi * F0 * n * n
            f = sum(train["P"] * math.sin(n * math.pi * (speed * t - a) / L) for a in ax if 0 <= speed * t - a <= L) / mg
            q, v = y[2 * (n - 1)], y[2 * (n - 1) + 1]
            dy[2 * (n - 1)] = v
            dy[2 * (n - 1) + 1] = f - 2 * zeta * w * v - w * w * q
        return dy

    ts = np.arange(0, t_end, 5e-4)
    sol = solve_ivp(rhs, (0, t_end), np.zeros(2 * MODES), t_eval=ts, method="DOP853", rtol=1e-8, atol=1e-10, max_step=2e-3)
    acc = np.zeros(len(sol.t))
    for n in range(1, MODES + 1):
        w = 2 * math.pi * F0 * n * n
        q, v = sol.y[2 * (n - 1)], sol.y[2 * (n - 1) + 1]
        f = np.array([sum(train["P"] * math.sin(n * math.pi * (speed * t - a) / L) for a in ax if 0 <= speed * t - a <= L) / mg for t in sol.t])
        acc += math.sin(n * math.pi / 2) * (f - 2 * zeta * w * v - w * w * q)
    return float(np.max(np.abs(acc)))


def limits(span):
    lower = 80.0 / span if span <= 20 else 23.58 * span ** -0.592
    return lower, 94.76 * span ** -0.748


if __name__ == "__main__":
    resonant = F0 * TRAIN["D"]
    ref = dict(
        resonant_speed=resonant,
        peak_on_resonance_z001=peak(resonant, 0.01),
        peak_on_resonance_z002=peak(resonant, 0.02),
        peak_off_resonance_z001=peak(0.75 * resonant, 0.01),
        limits_10m=limits(10.0),
        limits_40m=limits(40.0),
        xd_mpa=0.9 * 355.0 / 1.1,
    )
    print(json.dumps(ref, indent=1))
