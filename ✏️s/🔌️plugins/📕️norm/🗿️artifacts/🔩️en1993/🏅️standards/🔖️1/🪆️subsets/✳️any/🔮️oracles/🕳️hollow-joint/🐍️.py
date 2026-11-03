"""🕳️ Oracle of the RHS K gap joint of the showcase row (EN 1993-1-8 Tables 7.8 and 7.11): formulas transcribed independently; values are UNVERIFIED against the standard text (no source in the repository). Run with `python3 -I`."""
import json
import math

b0, t0, fy0, b1, h1, t1, th1, b2, h2, t2, th2, gap, fyi = 120.0, 8.0, 355.0, 80.0, 80.0, 5.0, 45.0, 80.0, 80.0, 5.0, 45.0, 30.0, 355.0
beta = (b1 + b2 + h1 + h2) / (4 * b0)
gamma = b0 / (2 * t0)
kn = 1.0
n1 = 8.9 * kn * fy0 * t0**2 * math.sqrt(gamma) * (b1 + b2 + h1 + h2) / (4 * b0 * math.sin(math.radians(th1)))
n2 = n1 * math.sin(math.radians(th1)) / math.sin(math.radians(th2))


def brace(h, b, t):
    beff = min(10 / (b0 / t0) * fy0 * t0 / (fyi * t) * b, b)
    return fyi * t * (2 * h - 4 * t + b + beff)


force = 60000.0
print(json.dumps({"beta": beta, "n1_rd": n1, "n2_rd": n2, "brace1": brace(h1, b1, t1), "brace2": brace(h2, b2, t2), "utilisation": max(force / n1, force / n2, force / brace(h1, b1, t1), force / brace(h2, b2, t2))}))
