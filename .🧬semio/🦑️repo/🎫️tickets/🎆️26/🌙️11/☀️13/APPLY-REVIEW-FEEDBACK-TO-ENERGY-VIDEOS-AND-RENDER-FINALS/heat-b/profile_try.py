import numpy as np
import sys
AMP, WID = float(sys.argv[1]), float(sys.argv[2])

SEASON_DAYS, GT, THETA_I, THETA_HG = 212, 3500.0, 20.0, 15.0
DAYS = np.arange(365)
PHASE = -np.cos(2 * np.pi * (DAYS - 15) / 365)
NOISE = np.convolve(np.random.default_rng(18599).normal(0.0, 2.2, 365), np.ones(5) / 5, mode="same")
GAP = np.abs((DAYS - 15 + 182) % 365 - 182)
BUMP = np.exp(-(GAP / WID) ** 2)


def curve(mean, cold):
    return mean + AMP * PHASE - cold * BUMP + NOISE


def fit_mean(cold):
    lo, hi = -30.0, 40.0
    for _ in range(100):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if (curve(mid, cold) < THETA_HG).sum() > SEASON_DAYS else (lo, mid)
    return hi


def total(cold):
    theta = curve(fit_mean(cold), cold)
    return float((THETA_I - theta)[theta < THETA_HG].sum())


lo, hi = 0.0, 30.0
for _ in range(60):
    mid = (lo + hi) / 2
    lo, hi = (mid, hi) if total(mid) < GT else (lo, mid)
cold = (lo + hi) / 2
theta = curve(fit_mean(cold), cold)
mask = theta < THETA_HG
print("cold", cold, "count", mask.sum(), "sum", (THETA_I - theta)[mask].sum())
for m in range(12):
    s = slice(int(m * 30.4), int((m + 1) * 30.4))
    print(m + 1, round(theta[s].mean(), 1))
print("min", theta.min(), "max", theta.max())
print("first/last heating day", np.where(mask)[0][[0, -1]], "runs", int((np.diff(mask.astype(int)) != 0).sum()))
