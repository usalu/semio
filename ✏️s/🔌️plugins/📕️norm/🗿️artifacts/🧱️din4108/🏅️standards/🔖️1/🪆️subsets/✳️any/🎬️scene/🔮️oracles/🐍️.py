"""🔮️ Independent oracle of the DIN 4108 scene diagrams: the steady-state temperature at the layer interfaces from a nodal solve of the series resistance network (numpy linear algebra, not the closed-form ratio of the code), the vapour pressure by linear interpolation over the equivalent air layer thickness (numpy), and the saturation pressure by the formula of DIN EN ISO 13788 (610.5 · exp(17.269 θ / (237.3 + θ)) for θ ≥ 0, 610.5 · exp(21.875 θ / (265.5 + θ)) below), which differs from the Magnus fit of the code by well under one percent.
Reads {"elements": [{"id", "r_si", "r_se", "t_int", "t_ext", "rh_int", "rh_ext", "layers": [{"d", "lambda", "mu"}], "temperature": [[x, y]], "vapour": [[x, y]], "saturation": [[x, y]]}]} as JSON on stdin and prints {"ok": bool, "checks": n, "failures": [...]}."""

import json
import math
import sys

import numpy as np

SATURATION_TOLERANCE = 0.01
TEMPERATURE_TOLERANCE = 1e-9


def saturation_iso_13788(theta):
    if theta >= 0.0:
        return 610.5 * math.exp(17.269 * theta / (237.3 + theta))
    return 610.5 * math.exp(21.875 * theta / (265.5 + theta))


def nodal_temperatures(element):
    resistances = [element["r_si"]] + [layer["d"] / layer["lambda"] for layer in element["layers"]] + [element["r_se"]]
    nodes = len(resistances) - 1
    conductance = np.zeros((nodes, nodes))
    source = np.zeros(nodes)
    for index, resistance in enumerate(resistances):
        g = 1.0 / resistance
        left, right = index - 1, index
        if left >= 0:
            conductance[left, left] += g
        if right < nodes:
            conductance[right, right] += g
        if left >= 0 and right < nodes:
            conductance[left, right] -= g
            conductance[right, left] -= g
        if left < 0:
            source[right] += g * element["t_int"]
        if right >= nodes:
            source[left] += g * element["t_ext"]
    return np.linalg.solve(conductance, source)


def check_element(element, failures):
    count = 0
    temperatures = nodal_temperatures(element)
    expected = [temperatures[0]] + [temperatures[i + 1] for i in range(len(element["layers"]))]
    for (x, y), t in zip(element["temperature"], expected):
        count += 1
        if abs(y - t) > 1e-6:
            failures.append(f"{element['id']}: temperature at {x} is {y}, oracle {t}")
    if len(element["temperature"]) != len(expected):
        failures.append(f"{element['id']}: {len(element['temperature'])} temperature points, oracle {len(expected)}")
    depth = 0.0
    depths = [0.0]
    for layer in element["layers"]:
        depth += layer["d"]
        depths.append(depth)
    for (x, _), want in zip(element["temperature"], depths):
        count += 1
        if abs(x - want) > 1e-12:
            failures.append(f"{element['id']}: temperature abscissa {x}, oracle {want}")
    sd = [0.0]
    for layer in element["layers"]:
        sd.append(sd[-1] + layer["d"] * layer["mu"])
    p_int = element["rh_int"] * saturation_iso_13788(element["t_int"])
    p_ext = element["rh_ext"] * saturation_iso_13788(element["t_ext"])
    for (x, y), s in zip(element["vapour"], sd):
        count += 1
        want = float(np.interp(s, [0.0, sd[-1]], [p_int, p_ext])) if sd[-1] > 0 else p_int
        if abs(x - s) > 1e-9:
            failures.append(f"{element['id']}: vapour abscissa {x}, oracle {s}")
        if abs(y - want) > SATURATION_TOLERANCE * max(abs(want), 1.0):
            failures.append(f"{element['id']}: vapour pressure at s_d {s} is {y}, oracle {want}")
    for (_, y), t in zip(element["saturation"], expected):
        count += 1
        want = saturation_iso_13788(t)
        if abs(y - want) > SATURATION_TOLERANCE * want:
            failures.append(f"{element['id']}: saturation pressure {y}, oracle {want}")
    return count


def main():
    document = json.load(sys.stdin)
    failures = []
    checks = sum(check_element(element, failures) for element in document["elements"])
    print(json.dumps({"ok": not failures and checks > 0, "checks": checks, "failures": failures}))
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())
