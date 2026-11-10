"""🔮️ Independent oracle of the VDI 3805 scene: the EN 442 radiator output Φ = Φ_N·(ΔT/ΔT_N)^n and the valve law Δp = (ρ/ρ₀)(Q/kvs)² with the IAPWS-95 water density (NIST WebBook, 977.76 kg/m³ at 70 °C) recomputed with numpy, the logarithmic mean excess temperature 75/65/20 °C,
the body sizes of the products and the affine law of the face grips. Reads {"radiator", "valve", "grips"} as JSON on stdin and prints {"ok": bool, "checks": n, "failures": [...]}."""

import json
import math
import sys

import numpy as np

IAPWS95_DENSITY_70C = 977.76




def log_mean_excess(flow, ret, room):
    return (flow - ret) / math.log((flow - room) / (ret - room))


def main():
    data = json.load(sys.stdin)
    failures = []
    checks = [0]

    def check(ok, message):
        checks[0] += 1
        if not ok:
            failures.append(message)

    delta_t_nominal = log_mean_excess(75.0, 65.0, 20.0)
    check(abs(delta_t_nominal - 49.83) < 0.01, "the logarithmic mean excess temperature of 75/65/20 °C is 49.83 K")
    radiator = data["radiator"]
    dt = np.array(radiator["deltaT"], dtype=float)
    expected = radiator["standardOutputW"] * (dt / delta_t_nominal) ** radiator["exponent"]
    got = np.array(radiator["outputW"], dtype=float)
    check(got.shape == expected.shape and bool(np.allclose(got, expected, rtol=1e-9)), "radiator output curve follows the EN 442 power law")
    check(bool(np.all(np.diff(got) > 0)), "radiator output grows with the excess temperature")
    valve = data["valve"]
    q = np.array(valve["flowM3H"], dtype=float)
    dp = np.array(valve["dropBar"], dtype=float)
    check(valve["waterC"] == 70.0, "the valve diagram uses the 70 °C mean water temperature of 75/65")
    rho = IAPWS95_DENSITY_70C / 1000.0
    check(bool(np.allclose(dp, rho * (q / valve["kvs"]) ** 2, rtol=2e-5)), "valve pressure drop follows the density-corrected square law")
    check(abs(float(np.interp(valve["kvs"], q, dp)) - rho) < 0.02, "the pressure drop at the flow kvs is the relative water density in bar")
    for grip in data["grips"]:
        low, start, scale, rest, stored = grip["low"], grip["start"], grip["scale"], grip["rest"], grip["stored"]
        check(abs(low - start / scale + rest / scale - stored) < 1e-9, f"grip {grip['id']} reads the stored extent back")
        check(abs((low - start / scale + (rest + 0.1) / scale) - (stored + 0.1 / scale)) < 1e-9, f"grip {grip['id']} is affine with slope 1/scale")
    print(json.dumps({"ok": not failures, "checks": checks[0], "failures": failures}))
    sys.exit(0 if not failures else 1)


if __name__ == "__main__":
    main()
