"""🎬️ Independent oracle for the viewport dynamics: oscillation phase, ramps, track blends, inverse-distance surface displacement, grid node order and the half-sine mode frame.

Closed forms with the standard library; numpy cross-checks them when importable (`np.sin`, `np.interp`). Run `python3 -I 🐍️.py` to print the golden JSON
(`../../🧪️tests/🧫️fixtures/🎬️dynamics.json`) the Rust tests compare against.
"""
import json, math

VISUAL_HZ, RAMP_SECONDS = 0.5, 2.0

def phase(clock):
    return math.sin(2 * math.pi * VISUAL_HZ * clock)

def ramp(clock):
    return min(1.0, max(0.0, clock / RAMP_SECONDS))

def blend(keys, value):
    ats = [k[0] for k in keys]
    if value <= ats[0]:
        return [keys[0][1], None, 0.0]
    if value >= ats[-1]:
        return [keys[-1][1], None, 0.0]
    i = next(i for i, a in enumerate(ats) if a > value)
    (a0, f0), (a1, f1) = keys[i - 1], keys[i]
    return [f0, f1, (value - a0) / (a1 - a0)]

def idw(samples, at):
    num, den = [0.0, 0.0, 0.0], 0.0
    for (x, y), d in samples:
        d2 = (x - at[0]) ** 2 + (y - at[1]) ** 2
        if d2 < 1e-18:
            return list(d)
        num = [n + c / d2 for n, c in zip(num, d)]
        den += 1.0 / d2
    return [n / den for n in num]

def grid_nodes(rows, cols):
    out = []
    for r in range(rows - 1):
        for c in range(cols - 1):
            a, b, d, e = (r * cols + c), (r * cols + c + 1), ((r + 1) * cols + c), ((r + 1) * cols + c + 1)
            for tri in ((a, b, e), (a, e, d), (a, e, b), (a, d, e)):
                out.extend(tri)
    return out

def mode_depth(scale, clock, depth=0.01):
    return -depth * scale * phase(clock)

KEYS = [(0.0, "temp@0"), (1800.0, "temp@30"), (3600.0, "temp@60")]
SAMPLES = [((0.0, 0.0), (0.0, 0.0, 0.0)), ((2.0, 0.0), (0.0, 0.0, 0.02)), ((0.0, 2.0), (0.0, 0.0, -0.01))]

golden = {
    "phase": [[t, phase(t)] for t in (0.0, 0.5, 1.0, 1.5)],
    "ramp": [[t, ramp(t)] for t in (0.0, 1.0, 2.0, 3.0)],
    "blend": [[v, blend(KEYS, v)] for v in (0.0, 900.0, 1800.0, 2700.0, 4000.0)],
    "idw": [[list(p), idw(SAMPLES, p)] for p in ((0.0, 0.0), (1.0, 0.0), (0.5, 0.5), (1.0, 1.0))],
    "gridNodes3x3": grid_nodes(3, 3),
    "modeDepth": [[t, mode_depth(10.0, t)] for t in (0.0, 0.5, 1.0, 1.5)],
    "vertexBlend": {"from": [0.0, 1.0, 2.0, 3.0], "to": [10.0, 11.0, 12.0, 13.0], "weight": 0.25, "values": [a + (b - a) * 0.25 for a, b in zip([0.0, 1.0, 2.0, 3.0], [10.0, 11.0, 12.0, 13.0])]},
}

def numpy_check():
    try:
        import numpy as np
    except ImportError:
        return None
    assert all(abs(np.sin(2 * np.pi * VISUAL_HZ * t) - p) < 1e-12 for t, p in golden["phase"])
    assert abs(np.interp(900.0, [0.0, 1800.0], [0.0, 1.0]) - golden["blend"][1][1][2]) < 1e-12
    return True

if __name__ == "__main__":
    golden["numpy"] = numpy_check()
    print(json.dumps(golden, indent=2))
