"""🔩️ Independent oracle of the bolt group of EN 1993-1-8 3.12: (1) the elastic vector method with the polar moment, (2) the instantaneous centre of rotation with the Crawford-Kulak load-deformation law R = R_ult (1 - exp(-mu delta))^lambda solved as a three-unknown nonlinear system (centre x, centre y, load factor) with `scipy.optimize.fsolve` from several starting points, and (3) the bracketed search used by the Rust implementation (one-dimensional `scipy.optimize.brentq` on the symmetric centre, then a damped Newton polish), so that both agree. Run with `python -I`; it prints the JSON that the Rust unit tests embed."""

import json
import math

import numpy as np
from scipy.optimize import brentq, fsolve

MU, LAMBDA, DELTA_MAX = 10.0, 0.55, 8.64


def grid(rows, per_row, pitch, gauge):
    return [((c - (per_row - 1) / 2) * gauge, (r - (rows - 1) / 2) * pitch) for r in range(rows) for c in range(per_row)]


def elastic(points, fx, fy, mz):
    n = len(points)
    ip = sum(x * x + y * y for x, y in points)
    forces = []
    for x, y in points:
        forces.append((fx / n - mz * y / ip, fy / n + mz * x / ip))
    return forces


def law(rho, r_ult):
    return r_ult * (1 - math.exp(-MU * DELTA_MAX * rho)) ** LAMBDA


def bolt_vectors(points, centre, sense, r_ult):
    rs = [(x - centre[0], y - centre[1]) for x, y in points]
    norms = [math.hypot(*r) for r in rs]
    rmax = max(norms)
    out = []
    for (rx, ry), nr in zip(rs, norms):
        r = law(nr / rmax, r_ult) if nr > 1e-12 else 0.0
        out.append((sense * r * -ry / nr if nr > 1e-12 else 0.0, sense * r * rx / nr if nr > 1e-12 else 0.0, r, nr))
    return out


def moment(points, vectors):
    return sum(x * gy - y * gx for (x, y), (gx, gy, _, _) in zip(points, vectors))


def residual(unknown, points, f, mz, sense, r_ult):
    cx, cy, k = unknown
    v = bolt_vectors(points, (cx, cy), sense, r_ult)
    sx, sy = sum(g[0] for g in v), sum(g[1] for g in v)
    return [sx - k * f[0], sy - k * f[1], moment(points, v) - k * mz]


def ic_fsolve(points, f, mz, r_ult):
    sense = 1.0 if mz > 0 else -1.0
    norm = math.hypot(*f)
    nx, ny = -f[1] / norm, f[0] / norm
    scale = max(math.hypot(*p) for p in points)
    for t in (0.05, 0.2, 0.5, 1.0, 2.0, 5.0):
        guess = [sense * nx * t * scale, sense * ny * t * scale, len(points) * r_ult / norm * 0.5]
        sol, info, ok, _ = fsolve(residual, guess, args=(points, f, mz, sense, r_ult), full_output=True, xtol=1e-13)
        if ok == 1 and sol[2] > 0 and max(abs(np.array(residual(sol, points, f, mz, sense, r_ult)))) < 1e-6 * len(points) * r_ult:
            return sol, sense
    return None, sense


def ic_bracket(points, f, mz, r_ult):
    """The algorithm of the Rust implementation, written the same way."""
    norm = math.hypot(*f)
    ux, uy = f[0] / norm, f[1] / norm
    sense = 1.0 if mz > 0 else -1.0
    nx, ny = -uy, ux
    scale = max(math.hypot(*p) for p in points)

    def at(t):
        c = (sense * nx * t, sense * ny * t)
        v = bolt_vectors(points, c, sense, r_ult)
        sx, sy = sum(g[0] for g in v), sum(g[1] for g in v)
        k = (sx * f[0] + sy * f[1]) / (norm * norm)
        return c, v, k, moment(points, v) - k * mz

    lo, hi = 1e-9 * scale, 1e3 * scale
    glo, ghi = at(lo)[3], at(hi)[3]
    if glo * ghi > 0:
        return None
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        gm = at(mid)[3]
        if glo * gm <= 0:
            hi = mid
        else:
            lo, glo = mid, gm
    t = 0.5 * (lo + hi)
    c, v, k, _ = at(t)
    unknown = np.array([c[0], c[1], k])
    for _ in range(40):
        r = np.array(residual(unknown, points, f, mz, sense, r_ult))
        if np.max(np.abs(r)) < 1e-9 * len(points) * r_ult:
            break
        jac = np.zeros((3, 3))
        h = 1e-7 * scale
        for j in range(3):
            d = unknown.copy()
            step = h if j < 2 else 1e-7 * max(abs(unknown[2]), 1e-9)
            d[j] += step
            jac[:, j] = (np.array(residual(d, points, f, mz, sense, r_ult)) - r) / step
        try:
            delta = np.linalg.solve(jac, -r)
        except np.linalg.LinAlgError:
            break
        lam = 1.0
        while lam > 1e-4:
            trial = unknown + lam * delta
            if np.max(np.abs(np.array(residual(trial, points, f, mz, sense, r_ult)))) < np.max(np.abs(r)):
                break
            lam /= 2
        unknown = unknown + lam * delta
    return unknown, sense


def case(name, rows, per_row, pitch, gauge, fx, fy, e, r_ult):
    pts = grid(rows, per_row, pitch, gauge)
    mz = fy * e - fx * 0.0
    f = (fx, fy)
    el = elastic(pts, fx, fy, mz)
    el_max = max(math.hypot(*g) for g in el)
    sol, sense = ic_fsolve(pts, f, mz, r_ult)
    own = ic_bracket(pts, f, mz, r_ult)
    k_fsolve = float(sol[2])
    k_own = float(own[0][2])
    assert abs(k_fsolve - k_own) / k_fsolve < 1e-6, (name, k_fsolve, k_own)
    return {
        "name": name,
        "points": pts,
        "elastic_max": el_max,
        "elastic_factor": r_ult / el_max,
        "ic_factor": k_fsolve,
        "ic_centre": [float(sol[0]), float(sol[1])],
        "ic_factor_bracket": k_own,
        "gain": k_fsolve * el_max / r_ult,
    }


def main():
    r_ult = 100000.0
    out = {
        "bracket": case("3 rows x 2 bolts, e 150 mm", 3, 2, 75.0, 100.0, 0.0, 1000.0, 150.0, r_ult),
        "bracket4": case("4 rows x 2 bolts, e 100 mm", 4, 2, 80.0, 90.0, 0.0, 1000.0, 100.0, r_ult),
        "inclined": case("3 rows x 2 bolts, shear and axial", 3, 2, 75.0, 100.0, 300.0, 1000.0, 120.0, r_ult),
    }
    pure = grid(1, 2, 0.0, 100.0)
    out["two_bolt_moment"] = {"m_ult_expected": 2 * 50.0 * r_ult * (1 - math.exp(-MU * DELTA_MAX)) ** LAMBDA}
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
