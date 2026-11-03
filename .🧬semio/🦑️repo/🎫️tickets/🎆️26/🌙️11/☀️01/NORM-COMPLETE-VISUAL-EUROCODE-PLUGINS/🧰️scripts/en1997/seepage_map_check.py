"""Numerical verification of the sheet-pile conformal map used by HYD-02 and the scene streamlines: h = H/2 - H/pi * Re arcsin(sqrt(z^2 + d^2)/d), z = x + iy, y < 0 (soil), pile at x = 0 for -d < y < 0."""
import cmath
import math

H, D = 3.0, 2.0


def potential(x, y):
    zeta = cmath.sqrt(complex(x, y) ** 2 + D * D)
    if (x >= 0) != (zeta.real >= 0) or (zeta.imag > 0) == (y < 0):
        zeta = -zeta
    return H / 2 - H / math.pi * cmath.asin(zeta / D).real


def laplace_residual(x, y, e=1e-3):
    return (potential(x + e, y) + potential(x - e, y) + potential(x, y + e) + potential(x, y - e) - 4 * potential(x, y)) / (e * e)


for x, y in [(1.0, -1.0), (2.0, -3.0), (-1.5, -2.5), (0.7, -0.4)]:
    print("laplace", x, y, round(laplace_residual(x, y), 4))
print("surface upstream h", round(potential(-2.0, -1e-9), 6), "downstream h", round(potential(2.0, -1e-9), 6))
for x in (0.05, 0.5, 1.0, 3.0):
    exit_gradient = abs(potential(x, -1e-4) - potential(x, -2e-4)) / 1e-4
    print("exit gradient x =", x, round(exit_gradient, 5), "closed form", round(H / (math.pi * math.hypot(x, D)), 5))
