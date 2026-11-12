"""Writes the language-agnostic vectors of the portable math: expected values from 80-digit decimal series (stdlib only), cross-checked against the platform libm."""
import json, math, sys
from decimal import Decimal as D, getcontext
getcontext().prec = 90
OUT = sys.argv[1]

def atan_inv(n):
    x = D(1) / n; s = x; t = x; k = 1; n2 = n * n
    while abs(t) > D(10) ** -88:
        t = -t / n2; k += 2; s += t / k
    return s
PI = 4 * (4 * atan_inv(5) - atan_inv(239))

def sin_cos(x):
    x = D(x)
    n = int((x / (PI / 2)).to_integral_value())
    r = x - n * (PI / 2)
    def series(r, start):
        term = r if start == 1 else D(1); s = term; k = start
        r2 = r * r
        while abs(term) > D(10) ** -88:
            term = -term * r2 / ((k + 1) * (k + 2)); k += 2; s += term
        return s
    s, c = series(r, 1), series(r, 0)
    q = n % 4
    return [(s, c), (c, -s), (-s, -c), (-c, s)][q]

def atan(x):
    x = D(x); halvings = 0
    while abs(x) > D("0.1"):
        x = x / (1 + (1 + x * x).sqrt()); halvings += 1
    s = x; t = x; k = 1; x2 = x * x
    while abs(t) > D(10) ** -88:
        t = -t * x2; k += 2; s += t / k
    return s * (2 ** halvings)

def atan2(y, x):
    y, x = D(y), D(x)
    if x > 0: return atan(y / x)
    if x < 0: return atan(y / x) + (PI if y >= 0 else -PI)
    return PI / 2 if y > 0 else -PI / 2

ref = {
    "sin": lambda a: sin_cos(a[0])[0], "cos": lambda a: sin_cos(a[0])[1],
    "tan": lambda a: sin_cos(a[0])[0] / sin_cos(a[0])[1],
    "atan2": lambda a: atan2(a[0], a[1]), "atan": lambda a: atan(a[0]),
    "asin": lambda a: atan2(D(a[0]), ((1 - D(a[0])) * (1 + D(a[0]))).sqrt()),
    "acos": lambda a: atan2(((1 - D(a[0])) * (1 + D(a[0]))).sqrt(), D(a[0])),
    "exp": lambda a: D(a[0]).exp(),
    "pow": lambda a: (-1 if a[0] < 0 and a[1] % 2 == 1 else 1) * (D(a[1]) * abs(D(a[0])).ln()).exp(),
    "log": lambda a: D(a[0]).ln(), "log2": lambda a: D(a[0]).ln() / D(2).ln(), "log10": lambda a: D(a[0]).log10(),
    "cbrt": lambda a: (D(a[0]).ln() / 3).exp() if a[0] > 0 else -((-D(a[0])).ln() / 3).exp(),
    "hypot": lambda a: sum((D(v) * D(v) for v in a), D(0)).sqrt(),
}
libm = {"sin": math.sin, "cos": math.cos, "tan": math.tan, "atan2": math.atan2, "atan": math.atan, "asin": math.asin, "acos": math.acos, "exp": math.exp, "log": math.log, "log2": math.log2, "log10": math.log10, "cbrt": lambda v: math.copysign(abs(v) ** (1 / 3), v), "pow": math.pow, "hypot": math.hypot}

state = [20260611]
def rnd():
    state[0] = (state[0] * 1103515245 + 12345) % 2147483648
    return state[0] / 2147483648
def spread(lo, hi): return lo + (hi - lo) * rnd()

cases = []
def add(fn, *args):
    expected = float(ref[fn](args))
    got = libm[fn](*args)
    assert abs(expected - got) <= (4 if fn == "cbrt" else 2) * math.ulp(expected) + 1e-300, (fn, args, expected, got)
    cases.append({"fn": fn, "args": list(args), "expected": expected})

fixed = [0.1, 0.5, 1.0, 2.0, 3.0, 0.7853981633974483, 1.5707963267948966, 3.141592653589793, 1.2981137639120113, 2.6987558475348785, 0.5235987755982988, 2.0943951023931953, 3.665191429188092, 6.283185307179586, 100.0, 12345.678, 1e-3, 1e-9]
for a in fixed:
    for fn in ("sin", "cos", "tan"):
        add(fn, a); add(fn, -a)
for _ in range(120):
    a = spread(-8, 8)
    for fn in ("sin", "cos", "tan"): add(fn, a)
for a in [0.0, 0.1, 0.5, 0.7071067811865476, 0.9238795325112867, 0.9999999797076282, 0.38268343236508984, -0.3, -0.9]:
    for fn in ("asin", "acos"): add(fn, a)
for _ in range(80):
    a = spread(-1, 1)
    for fn in ("asin", "acos"): add(fn, a)
for y, x in [(1, 1), (-1, 1), (1, -1), (-1, -1), (0.5, 2), (-0.5204136140522797, 0.8539143225810449), (-0.9998476951563913, -0.017452406437283755), (0.784622850615449, 0.6199733722443947), (3, 4), (1e-8, 1), (1, 1e-8), (123.0, -0.001)]:
    add("atan2", float(y), float(x))
for _ in range(120): add("atan2", spread(-3, 3), spread(-3, 3))
for a in [0.0, 1e-4, 0.3, 1.0, -1.0, 2.5, 10.0, -10.0, 1.8680326530176248, 18.678, 50.0, 300.0, -300.0, 700.0]: add("exp", a)
for _ in range(80): add("exp", spread(-30, 30))
for x, y in [(2.0, 10.0), (2.0, 0.5), (10.0, -2.0), (0.5, 3.0), (1.5, 2.4), (0.04045, 2.4), (0.31, 1 / 2.4), (-2.0, 3.0), (7.0, 0.3333333333333333)]: add("pow", x, y)
for _ in range(80): add("pow", spread(0.01, 20), spread(-6, 6))
for a in [0.5, 1.0000001, 2.0, 3.0, 10.0, 100.0, 1000.0, 1e-5, 0.1, 7.5, 12345.678, 1e15, 8.0, 0.001]:
    for fn in ("log", "log2", "log10"): add(fn, a)
for _ in range(100):
    a = spread(0.001, 1000)
    for fn in ("log", "log2", "log10"): add(fn, a)
for a in [8.0, 27.0, 2.0, 3.0, 5.0, 7.0, 11.0, 13.0, 29.0, 0.001, -8.0, 1e6]: add("cbrt", a)
for _ in range(60): add("cbrt", spread(-50, 50))
for args in [(3.0, 4.0), (0.0, 0.0, 0.784622850615449, 0.6199733722443947), (-0.11000000000000001, 0.01), (1.0, 1.0, 1.0), (1e200, 1e200), (1e-200, 1e-200), (0.3, 0.4, 0.5)]: add("hypot", *args)
for _ in range(150): add("hypot", spread(-5, 5), spread(-5, 5), spread(-5, 5))
json.dump({"schema": "semio.model.portable-math-vector/v1", "cases": cases}, open(OUT, "w"), indent=0, ensure_ascii=False)
open(OUT, "a").write("\n")
print(len(cases))
