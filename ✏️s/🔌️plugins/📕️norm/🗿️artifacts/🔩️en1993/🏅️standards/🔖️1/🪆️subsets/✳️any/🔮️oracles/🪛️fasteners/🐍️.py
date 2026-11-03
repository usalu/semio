"""🪛️ Independent oracle of the screw and blind-rivet resistances of EN 1993-1-3 8.3 (SI units): bearing with linear interpolation between the equal-thickness and the thick-support formula (coefficients as in the registry rows ndp.en1993.1-3.fastener.*, written from memory and unverified), net section, pull-over, pull-out and the minimum spacings of the showcase joint; the interpolation is evaluated a second time with `numpy.interp`. Run with `python -I`; it prints the JSON that the Rust unit tests embed."""

import json
import math

import numpy as np

COEF = {"screw": {"equal": 3.2, "thick": 3.2, "pull_over": 1.5, "pull_out": 0.45, "min": (3.0, 1.5, 3.0, 3.0)}, "rivet": {"equal": 3.6, "thick": 3.6, "min": (1.5, 1.5, 3.0, 3.0)}}
RATIO_TOP = 2.5
GM2 = 1.25


def bearing(kind, t1, t2, d, fu1, gm2=GM2):
    equal = COEF[kind]["equal"] * math.sqrt(t1**3 * d) * fu1 / gm2
    thick = COEF[kind]["thick"] * t1 * d * fu1 / gm2
    return float(np.interp(t2 / t1, [1.0, RATIO_TOP], [equal, thick]))


def joint():
    d, dw, n, per_row = 0.0055, 0.016, 4, 2
    t1, t2, fu1, fu2 = 0.00075, 0.0015, 360e6, 360e6
    shear, tension, width = 4000.0, 1200.0, 0.1
    b = bearing("screw", t1, t2, d, fu1)
    over = COEF["screw"]["pull_over"] * dw * t1 * fu1 / GM2
    out = COEF["screw"]["pull_out"] * d * t2 * fu2 / GM2
    net = (width - per_row * d) * t1 * fu1 / GM2
    u_v, u_t = shear / n / b, tension / n / min(over, out)
    spacing = [m * d for m in COEF["screw"]["min"]]
    return {"bearing": b, "pull_over": over, "pull_out": out, "net": net, "u_shear": u_v, "u_tension": u_t, "u_interaction": u_v + u_t, "u_net": shear / net, "spacing_min": spacing, "count_for_u1": math.ceil(n * (u_v + u_t))}


def main():
    out = {"showcase": joint()}
    out["rivet_equal"] = bearing("rivet", 0.001, 0.001, 0.0048, 400e6)
    out["screw_thick"] = bearing("screw", 0.001, 0.0025, 0.0055, 360e6)
    out["screw_ratio_2"] = bearing("screw", 0.00075, 0.0015, 0.0055, 360e6)
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
