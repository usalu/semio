"""EN 1999 v6 references of the stiffener flexural rules, third-party: the Winkler strut critical load (scipy sparse eigenproblem of the finite-difference beam on elastic foundation) against sigma_cr,s = 2*sqrt(K*E*I)/A,; the ring line load 3*E*I/r^3 is the classical closed form (no third-party reference). Prints the numbers asserted by the Rust unit tests. Usage: /home/deck/semio/.venv/bin/python -I wave6_ref.py"""
import math
import numpy as np
from scipy.linalg import eigh

E, NU = 70_000e6, 0.3


def spring(t, b):
    return E * t**3 / (4 * (1 - NU**2) * b**3)


def strut_closed(k, i, a):
    return 2 * math.sqrt(k * E * i) / a


def strut_numeric(k, i, length=20.0, n=1200):
    h = length / n
    d4 = np.zeros((n - 1, n - 1))
    for r in range(n - 1):
        for c, w in ((r - 2, 1), (r - 1, -4), (r, 6), (r + 1, -4), (r + 2, 1)):
            if 0 <= c < n - 1:
                d4[r, c] = w
    d4[0, 0] = 7
    d4[-1, -1] = 7
    d2 = np.zeros((n - 1, n - 1))
    for r in range(n - 1):
        d2[r, r] = 2
        if r > 0: d2[r, r - 1] = -1
        if r < n - 2: d2[r, r + 1] = -1
    a = E * i * d4 / h**4 + k * np.eye(n - 1)
    b = d2 / h**2
    return eigh(a, b, eigvals_only=True, subset_by_index=[0, 0])[0]


if __name__ == "__main__":
    t, b, d, ts = 0.003, 0.05, 0.02, 0.004
    k = spring(t, b)
    a, i = d * ts, ts * d**3 / 12
    cl = strut_closed(k, i, a)
    nm = strut_numeric(k, i) / a
    print("spring", k, "area", a, "inertia", i)
    print("sigma_cr closed", cl, "numeric", nm)
    assert abs(cl - nm) / cl < 5e-3, (cl, nm)
    inertia, r = 2.5e-8, 0.5
    print("ring w_cr closed", 3 * E * inertia / r**3)
