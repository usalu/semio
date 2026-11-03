"""🌀️ Independent oracle of the EN 1993 scene modes: the Euler load of a pinned strut, the elastic lateral-torsional critical moment of a fork-supported I under constant moment and the midspan deflection of a simply supported beam under constant moment.
Arguments: E G Iy Iz It Iw L M. Prints `key value` lines. Uses numpy (linear algebra of an assembled Hermite finite element model) when installed and the closed forms otherwise."""

import math
import sys


def euler(e, i, length):
    return math.pi**2 * e * i / length**2


def mcr(e, g, iz, it, iw, length):
    return math.pi**2 * e * iz / length**2 * math.sqrt(iw / iz + length**2 * g * it / (math.pi**2 * e * iz))


def deflection(e, i, length, m):
    return m * length**2 / (8.0 * e * i)


def euler_numpy(e, i, length, elements=64):
    import numpy as np

    h = length / elements
    n = 2 * (elements + 1)
    k = np.zeros((n, n))
    g = np.zeros((n, n))
    ke = e * i / h**3 * np.array([[12, 6 * h, -12, 6 * h], [6 * h, 4 * h * h, -6 * h, 2 * h * h], [-12, -6 * h, 12, -6 * h], [6 * h, 2 * h * h, -6 * h, 4 * h * h]])
    ge = 1.0 / (30.0 * h) * np.array([[36, 3 * h, -36, 3 * h], [3 * h, 4 * h * h, -3 * h, -h * h], [-36, -3 * h, 36, -3 * h], [3 * h, -h * h, -3 * h, 4 * h * h]])
    for el in range(elements):
        s = slice(2 * el, 2 * el + 4)
        k[s, s] += ke
        g[s, s] += ge
    free = [d for d in range(n) if d not in (0, n - 2)]
    values = np.linalg.eigvals(np.linalg.solve(g[np.ix_(free, free)], k[np.ix_(free, free)]))
    return float(min(v.real for v in values if v.real > 0))


if __name__ == "__main__":
    e, g, iy, iz, it, iw, length, m = (float(v) for v in sys.argv[1:9])
    try:
        euler_value = euler_numpy(e, min(iy, iz), length)
    except ImportError:
        euler_value = euler(e, min(iy, iz), length)
    print("euler", euler_value)
    print("mcr", mcr(e, g, iz, it, iw, length))
    print("deflection", deflection(e, iy, length, m))
