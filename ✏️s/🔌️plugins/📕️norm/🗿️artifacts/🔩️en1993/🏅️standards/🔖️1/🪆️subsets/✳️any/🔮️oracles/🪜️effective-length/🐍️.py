"""🪜️ Independent oracle of the effective length factor of a column in a frame (EN 1993-1-1 6.3.1.3): the transcendental nomogram equations of Jackson and Moreland in their textbook form (with tangents, not multiplied by sin x) solved with `scipy.optimize.brentq` on a pole-free bracket. The closed-form approximations of the ENV 1993-1-1 Annex E (distribution factors eta_1, eta_2) are not used because their coefficients are not verified against the print. Run with `python -I`; it prints the JSON that the Rust unit tests embed."""

import json
import math

from scipy.optimize import brentq

BIG = 1.0e9


def non_sway(ga, gb):
    def f(k):
        x = math.pi / k
        return ga * gb / 4 * x * x + (ga + gb) / 2 * (1 - x / math.tan(x)) + 2 * math.tan(x / 2) / x - 1
    return f


def sway(ga, gb):
    def f(k):
        x = math.pi / k
        return (ga * gb * x * x - 36) / (6 * (ga + gb)) - x / math.tan(x)
    return f


def root(f, lo, hi):
    grid = [lo + (hi - lo) * i / 4000 for i in range(1, 4000)]
    values = [f(k) for k in grid]
    for a, b, fa, fb in zip(grid, grid[1:], values, values[1:]):
        if fa * fb < 0 and abs(fa) < 1e6 and abs(fb) < 1e6:
            return brentq(f, a, b, xtol=1e-14)
    return None


def k_non_sway(ga, gb):
    return root(non_sway(ga, gb), 0.5001, 0.9999)


def k_sway(ga, gb):
    return root(sway(ga, gb), 1.0001, 12.0)


def main():
    out = {"non_sway": {}, "sway": {}}
    for ga, gb in [(1.0, 1.0), (0.5, 2.0), (10.0, 10.0), (0.1, 0.1), (2.0, 5.0)]:
        k = k_non_sway(ga, gb)
        out["non_sway"][f"{ga}:{gb}"] = {"k": k}
    for ga, gb in [(1.0, 1.0), (0.5, 2.0), (10.0, 10.0), (0.1, 0.1), (2.0, 5.0)]:
        k = k_sway(ga, gb)
        out["sway"][f"{ga}:{gb}"] = {"k": k}
    out["limits"] = {"fixed_fixed_non_sway": 0.5, "fixed_fixed_sway": 1.0, "fixed_pinned_non_sway": 0.6991556596428412, "fixed_free_sway": 2.0, "k_non_sway_0.001": k_non_sway(0.001, 0.001), "k_sway_0.001": k_sway(0.001, 0.001), "k_non_sway_0_big": k_non_sway(0.001, 1e6), "k_sway_0_big": k_sway(0.001, 1e6)}
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
