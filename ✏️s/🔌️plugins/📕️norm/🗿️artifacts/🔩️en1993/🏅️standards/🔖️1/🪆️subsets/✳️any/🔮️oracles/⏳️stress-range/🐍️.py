"""⏳️ Independent oracle of the fatigue stress range of EN 1993-1-9 5.1 to 5.4 and 7.1 (SI units): the fibre stress N/A + M_y z/I_y, the range with the compressive part counted with 0.6, the size effect (25/t)^0.2 and the damage n / N_R of the range on the S-N curve of the category (corner points N_C = 2e6, N_D = 5e6, N_L = 1e8 with slopes 3 and 5), the endurance evaluated a second time with `numpy.interp` on the log-log polyline. Run with `python -I`; it prints the JSON that the Rust unit tests embed."""

import json
import math

import numpy as np

N_C, N_D, N_L, M1, M2 = 2.0e6, 5.0e6, 1.0e8, 3.0, 5.0


def fibre_stress(n, my, mz, area, iy, iz, z, y):
    return n / area + my * z / iy - mz * y / iz


def reduced(sigma):
    return 0.6 * sigma if sigma < 0 else sigma


def stress_range(a, b, relieved):
    hi, lo = max(a, b), min(a, b)
    return reduced(hi) - reduced(lo) if relieved else hi - lo


def endurance_closed(delta, delta_c):
    delta_d = delta_c * (N_C / N_D) ** (1 / M1)
    delta_l = delta_d * (N_D / N_L) ** (1 / M2)
    if delta > delta_d:
        return N_C * (delta_c / delta) ** M1
    if delta > delta_l:
        return N_D * (delta_d / delta) ** M2
    return None


def endurance_interp(delta, delta_c):
    delta_d = delta_c * (N_C / N_D) ** (1 / M1)
    delta_l = delta_d * (N_D / N_L) ** (1 / M2)
    if delta <= delta_l:
        return None
    log_s = [math.log10(delta_c * 3), math.log10(delta_c), math.log10(delta_d), math.log10(delta_l)]
    log_n = [math.log10(N_C * (1 / 3) ** M1), math.log10(N_C), math.log10(N_D), math.log10(N_L)]
    return 10 ** float(np.interp(math.log10(delta), log_s[::-1], log_n[::-1]))


def main():
    area, iy, iz = 0.0106, 1.126e-4, 3.923e-5
    s_g = fibre_stress(100000.0, 40000.0, 0.0, area, iy, iz, 0.12, 0.0)
    s_q = fibre_stress(43333.0, 17333.0, 0.0, area, iy, iz, 0.12, 0.0)
    rng = stress_range(s_g, s_q, True)
    dc, cycles = 71e6, 1.0e7
    n_r = endurance_closed(rng, dc)
    out = {
        "showcase": {"sigma_g": s_g, "sigma_q": s_q, "range": rng, "n_r": n_r, "n_r_interp": endurance_interp(rng, dc), "damage": cycles / n_r if n_r else 0.0},
        "compression": {"range_relieved": stress_range(60e6, -20e6, True), "range_welded": stress_range(60e6, -20e6, False), "both_compressive": stress_range(-10e6, -50e6, True)},
        "size_effect_40mm": (25 / 40) ** 0.2,
        "endurance": {"above_limit": endurance_closed(100e6, 71e6), "m5_branch": endurance_closed(40e6, 71e6), "below_cutoff": endurance_closed(10e6, 71e6)},
        "endurance_interp": {"above_limit": endurance_interp(100e6, 71e6), "m5_branch": endurance_interp(40e6, 71e6)},
    }
    for key in ("above_limit", "m5_branch"):
        assert abs(out["endurance"][key] - out["endurance_interp"][key]) / out["endurance"][key] < 1e-9, key
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
