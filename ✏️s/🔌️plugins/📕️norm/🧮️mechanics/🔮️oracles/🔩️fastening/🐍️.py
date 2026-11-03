"""🔩️ Oracle of the fastening kernel: closed forms of EN 1992-4 written independently of the Rust sources, the cone footprint union by `shapely` as third-party cross-check.

Reads `🧪️tests/🔩️fastening/🧾️cases.json` and prints `id.quantity value` lines in N, mm, MPa; exits 1 when a stated `expect_*` value differs from the closed form by more than 1e-9 relative.
"""
import json
import math
import pathlib
import sys

from shapely.geometry import box
from shapely.ops import unary_union

CASES = pathlib.Path(__file__).resolve().parents[2] / "🧪️tests" / "🔩️fastening" / "🧾️cases.json"
BIG = 1e9


def cone(c):
    hef, ccr = c["h_ef"], 1.5 * c["h_ef"]
    c1 = c["c1"]
    x_min = -c1 if c1 > 0 else -BIG
    footprints = [box(max(x - ccr, x_min), y - ccr, x + ccr, y + ccr) for x, y in c["anchors"]]
    ratio = unary_union(footprints).area / (2 * ccr) ** 2
    edge = min(min(x - x_min, BIG - x, y + BIG, BIG - y) for x, y in c["anchors"])
    psi_s = min(0.7 + 0.3 * edge / ccr, 1.0)
    psi_re = min(0.5 + hef / 200, 1.0)
    psi_ec = 1 / (1 + 2 * c["e_nx"] / (3 * hef)) * 1 / (1 + 2 * c["e_ny"] / (3 * hef))
    n0 = c["k1"] * math.sqrt(c["f_ck"]) * hef ** 1.5
    return {"n_rk_c": n0 * ratio * psi_s * psi_re * min(psi_ec, 1.0), "ratio": ratio, "psi_s": psi_s}


def pullout(c):
    return {"n_rk_p": c["k2"] * math.pi / 4 * (c["d_h"] ** 2 - c["d"] ** 2) * c["f_ck"]}


def steel(c):
    ok = c["f_uk"] <= 800 and c["f_yk"] / c["f_uk"] <= 0.8
    gn = max(1.2 * c["f_uk"] / c["f_yk"], 1.4) if ok else 1.5
    gv = max(c["f_uk"] / c["f_yk"], 1.25) if ok else 1.5
    return {"n_rk_s": c["a_s"] * c["f_uk"], "gamma_n": gn, "gamma_v": gv}


def edge(c):
    c1, dn = c["c1"], c["d_nom"]
    lf = min(c["h_ef"], 8 * dn)
    v0 = c["k9"] * dn ** (0.1 * math.sqrt(lf / c1)) * lf ** (0.1 * (dn / c1) ** 0.2) * math.sqrt(c["f_ck"]) * c1 ** 1.5
    ratio = 3 * c1 * min(c["h"], 1.5 * c1) / (4.5 * c1 * c1)
    psi_h = max(math.sqrt(1.5 * c1 / c["h"]), 1.0)
    return {"v_rk_c": v0 * ratio * psi_h}


def pryout(c):
    return {"v_rk_cp": (1.0 if c["h_ef"] < 60 else 2.0) * c["n_rk_c"]}


def interaction(c):
    return {"value": (c["n_ed"] / c["n_rd"]) ** c["exponent"] + (c["v_ed"] / c["v_rd"]) ** c["exponent"]}


def main():
    failed = 0
    for c in json.loads(CASES.read_text())["cases"]:
        for key, value in globals()[c["kind"]](c).items():
            print(f"{c['id']}.{key} {value!r}")
            want = c.get(f"expect_{key}")
            if want is not None and abs(want - value) > 1e-9 * max(abs(value), 1e-300):
                print(f"[mismatch] {c['id']}.{key}: stated {want!r}, closed form {value!r}", file=sys.stderr)
                failed = 1
    return failed


if __name__ == "__main__":
    sys.exit(main())
