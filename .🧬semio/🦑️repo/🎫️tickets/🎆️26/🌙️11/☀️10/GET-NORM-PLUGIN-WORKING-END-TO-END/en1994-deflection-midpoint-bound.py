"""Midpoint-cell deflection of a uniformly loaded simple span against the closed form: observed error at N and 2N cells."""
import sys
L = 10.0
def shape(n, u):
    h = L / n
    xs = [(k + 0.5) * h for k in range(n)]
    kap = [x * (L - x) / 2.0 for x in xs]
    theta = -sum((L - x) * k for x, k in zip(xs, kap)) * h / L
    def w(j):
        a = sum(kap[:j]) * h
        b = sum(k * x for k, x in zip(kap[:j], xs[:j])) * h
        x = j * h
        return -(theta * x + x * a - b)
    return w(round(u * n)) / w(n // 2)
line = lambda x: x * (1 - 2 * x * x + x ** 3)
for u in (0.1, 0.25, 0.75):
    exact = line(u) / line(0.5)
    errs = [abs(shape(n, u) - exact) for n in (400, 800, 1600)]
    print(u, errs, errs[0] / errs[1], errs[1] / errs[2])
