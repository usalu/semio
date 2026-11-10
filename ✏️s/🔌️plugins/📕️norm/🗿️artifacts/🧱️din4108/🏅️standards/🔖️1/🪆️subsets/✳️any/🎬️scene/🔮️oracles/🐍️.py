"""🔮️ Independent oracle of the DIN 4108 scene diagrams: the steady-state temperature at the layer interfaces from a nodal solve of the series resistance network (numpy linear algebra, not the closed-form ratio of the code), the vapour pressure by linear interpolation over the equivalent air layer thickness between the boundaries and the condensation planes of the Glaser iteration (numpy), and the saturation pressure by the formula of DIN EN ISO 13788 (610.5 · exp(17.269 θ / (237.3 + θ)) for θ ≥ 0, 610.5 · exp(21.875 θ / (265.5 + θ)) below), which the code uses as well.
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
        g = 1.0 / max(resistance, 1e-9)
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
    sat = [saturation_iso_13788(t) for t in expected]
    last = len(sd) - 1
    planes = []

    def pressures():
        nodes = sorted([0, last] + planes)
        fixed = {0: p_int, last: p_ext, **{k: sat[k] for k in planes}}
        p = np.zeros(len(sd))
        for a, b in zip(nodes, nodes[1:]):
            idx = list(range(a, b + 1))
            p[idx] = np.interp([sd[k] for k in idx], [sd[a], sd[b]], [fixed[a], fixed[b]]) if sd[b] > sd[a] else fixed[a]
        return p

    while True:
        p = pressures()
        excess = [(p[k] - sat[k], k) for k in range(1, last) if k not in planes and p[k] > sat[k]]
        if not excess:
            break
        planes.append(max(excess)[1])
    for (x, y), s_k, want in zip(element["vapour"], sd, p):
        count += 1
        if abs(x - s_k) > 1e-9:
            failures.append(f"{element['id']}: vapour abscissa {x}, oracle {s_k}")
        if abs(y - want) > SATURATION_TOLERANCE * max(abs(want), 1.0):
            failures.append(f"{element['id']}: vapour pressure at s_d {s_k} is {y}, oracle {want}")
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
