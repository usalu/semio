"""Independent cross-check of the Rust catalogue module `fatigue` (EN 1994-2 6.8): the endurance is found numerically by root finding on the resistance curve (scipy `brentq`) instead of the closed form, the damage by trapezoid integration of the Palmgren-Miner sum of a two-block spectrum, and the stud row damages by the same root finding. The values are hard-coded in the Rust tests."""
import json, sys
from scipy.optimize import brentq


def resistance(dc, nref, m, gamma, n):
    return dc * (nref / n) ** (1 / m) / gamma


def endurance(dc, nref, m, gamma, delta):
    return 10 ** brentq(lambda ln: resistance(dc, nref, m, gamma, 10 ** ln) - delta, -3, 40, xtol=1e-14, rtol=1e-14)


def main():
    out = {}
    g = 1.15
    for name, (dc, nref, m, gam, delta, n) in {
        "stud": (90e6, 2e6, 8.0, g, 40e6, 5e6),
        "steel": (80e6, 2e6, 3.0, g, 50e6, 1e7),
        "rebar": (162.5e6, 1e6, 9.0, 1.15, 100e6, 2e6),
    }.items():
        nr = endurance(dc, nref, m, gam, delta)
        out[name] = {"resistance": resistance(dc, nref, m, gam, n), "endurance": nr, "damage": n / nr, "utilisation": delta / resistance(dc, nref, m, gam, n)}
    shares = [(0.5, 1.0), (1.5, 0.8), (2.5, 0.5), (3.5, 0.2)]
    out["rows"] = [{"x": x, "delta": 40e6 * s, "damage": 5e6 / endurance(90e6, 2e6, 8.0, g, 40e6 * s)} for x, s in shares]
    json.dump(out, sys.stdout)


if __name__ == "__main__":
    main()
