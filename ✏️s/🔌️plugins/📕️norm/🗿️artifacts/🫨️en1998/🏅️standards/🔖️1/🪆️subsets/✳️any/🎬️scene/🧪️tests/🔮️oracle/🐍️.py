import json
import sys

import numpy as np


def elastic(p, ts):
    ag, s, tb, tc, td = (p[k] for k in ("ag", "s", "tb", "tc", "td"))
    ts = np.asarray(ts, dtype=float)
    plateau = ag * s * 2.5
    return np.select(
        [ts <= tb, ts <= tc, ts <= td],
        [ag * s * (1 + ts / tb * 1.5), plateau, plateau * tc / np.maximum(ts, 1e-12)],
        plateau * tc * td / np.maximum(ts, 1e-12) ** 2,
    )


def design(p, ts):
    ag, s, tb, tc, td, q, beta = (p[k] for k in ("ag", "s", "tb", "tc", "td", "q", "beta"))
    ts = np.asarray(ts, dtype=float)
    plateau = ag * s * 2.5 / q
    floor = beta * ag
    return np.select(
        [ts <= tb, ts <= tc, ts <= td],
        [ag * s * (2 / 3 + ts / tb * (2.5 / q - 2 / 3)), plateau, np.maximum(plateau * tc / np.maximum(ts, 1e-12), floor)],
        np.maximum(plateau * tc * td / np.maximum(ts, 1e-12) ** 2, floor),
    )


def modes(masses, stiffness, site):
    n = len(masses)
    k = np.zeros((n, n))
    for i in range(n):
        k[i, i] += stiffness[i]
        if i + 1 < n:
            k[i, i] += stiffness[i + 1]
            k[i, i + 1] -= stiffness[i + 1]
            k[i + 1, i] -= stiffness[i + 1]
    mass = np.asarray(masses, dtype=float)
    m = np.sqrt(mass)
    w2, v = np.linalg.eigh(k / np.outer(m, m))
    out = []
    for index in range(n):
        phi = v[:, index] / m
        phi = phi / phi[np.argmax(np.abs(phi))]
        period = float(2 * np.pi / np.sqrt(w2[index]))
        gamma = float(np.sum(mass * phi) / np.sum(mass * phi**2))
        meff = float(np.sum(mass * phi) ** 2 / np.sum(mass * phi**2) / mass.sum())
        se = float(elastic({**site, "ag": site["a_gr"]}, [period])[0])
        amplitude = gamma * site["gamma_i"] * se * (period / (2 * np.pi)) ** 2
        out.append({"period": period, "shape": phi.tolist(), "gamma": gamma, "meff": meff, "amplitude": amplitude})
    return out


def design_level(case, t1):
    p = case["site"]
    se = float(elastic({**p, "ag": p["a_gr"]}, [t1])[0])
    return max(se * p["gamma_i"] / case["q"], 0.2 * p["a_gr"])


def forces(case):
    m = np.asarray(case["masses"], dtype=float)
    z = np.asarray(case["elevations"], dtype=float)
    lam = 0.85 if case["t1"] <= 2 * case["site"]["tc"] and len(m) > 2 else 1.0
    fb = design_level(case, case["t1"]) * m.sum() * lam
    return (fb * m * z / np.sum(m * z)).tolist(), fb


def main():
    case = json.load(sys.stdin)
    p = case["spectrum"]
    f, fb = forces(case)
    found = modes(case["masses"], case["stiffness"], case["site"]) if case["stiffness"] else []
    period, mode = (found[0]["period"], found[0]["shape"]) if found else (0.0, [])
    json.dump({"elastic": elastic(p, case["periods"]).tolist(), "design": design(p, case["periods"]).tolist(), "forces": f, "baseShear": fb, "modePeriod": period, "mode": mode, "modes": found}, sys.stdout)


main()
