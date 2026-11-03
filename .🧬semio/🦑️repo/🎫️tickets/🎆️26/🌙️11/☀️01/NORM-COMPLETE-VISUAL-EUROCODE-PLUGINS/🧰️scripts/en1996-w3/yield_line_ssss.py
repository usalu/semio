#!/usr/bin/env python3
"""Yield-line oracle (virtual work, derived, not normative): isotropic rectangle a<=b, four simply supported edges, uniform q.
m(x) = q(ab/2 - a x/3) / (4b/a + 2a/x), x = distance of ridge end from short edge, maximised over 0<x<=b/2."""
import sys

def m(a, b, x, q=1.0):
    return q * (a * b / 2 - a * x / 3) / (4 * b / a + 2 * a / x)

def best(a, b, q=1.0, n=200000):
    return max((m(a, b, b / 2 * i / n, q), b / 2 * i / n) for i in range(1, n + 1))

if __name__ == "__main__":
    assert abs(best(1, 1)[0] - 1 / 24) < 1e-9, "square must give q a^2/24"
    for a, b in [(1, 1), (1, 1.5), (1, 2), (3, 4.5)]:
        mm, x = best(a, b)
        print(f"a={a} b={b} m/(q a^2)={mm / a**2:.6f} x={x:.4f}")
