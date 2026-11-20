"""🪢 Vacuum cleaner hose routes: the ten 0.15 m hose bones (1.5 m hose) of every pose laid as a hanging hose (bending stiffness plus weight, resting on the floor, clear of wand and canister) from the canister socket to the grip, which stays where the pose had it, so wand and nozzle do not move.

The hose end leaves the handle's swivel 40 degrees off the wand axis on the operator side (running; 70 degrees in the pulled stroke, where the hose has less room) or on the canister side (parked, handle turned about the wand). Prints the pose bone angles (relative, as the rig wants them) and the clearances.
"""
import json
import math
import sys

SEG = 0.15
N = 10
A = (0.27, 0.125)
HOSE_R = 0.018
WAND_R = 0.022
WAND_L = 0.95
EXIT = 40.0

ROUTES = {
    "running": {"grip": (0.5617, 0.9653), "gripDeg": -62.0, "side": -1, "nozzleDeg": 62, "init": [-30, -20, -5, 0, 20, 60, 85, 90, 85, 78]},
    "running-pulled": {"grip": (0.4238, 1.0243), "gripDeg": -70.0, "side": -1, "exit": 70.0, "nozzleDeg": 70, "init": [-30, -10, 0, 20, 60, 90, 100, 90, 70, 50]},
    "parked": {"grip": (-0.2336, 1.0643), "gripDeg": -102.0, "side": 1, "nozzleDeg": 102, "init": [70, 80, 90, 100, 110, 120, 130, 125, 120, 118]},
}


def pts_of(h):
    """🧮 Joint points of the chain for world headings h."""
    p = [A]
    for x in h:
        p.append((p[-1][0] + SEG * math.cos(math.radians(x)), p[-1][1] + SEG * math.sin(math.radians(x))))
    return p


def seg_dist(p, q, a, b, n=20):
    """📏 Shortest distance between segments pq and ab (sampled along pq)."""
    best = 9.0
    dx, dy = b[0] - a[0], b[1] - a[1]
    for i in range(n + 1):
        t = i / n
        x = p[0] + (q[0] - p[0]) * t
        y = p[1] + (q[1] - p[1]) * t
        u = max(0.0, min(1.0, ((x - a[0]) * dx + (y - a[1]) * dy) / (dx * dx + dy * dy)))
        best = min(best, math.hypot(x - a[0] - u * dx, y - a[1] - u * dy))
    return best


def canister_gap(x, y):
    """📦 Signed clearance of a point from the canister side outline with handle (body frame, metres; negative inside)."""
    gx = max(-0.255 - x, x - 0.255, 0.035 - y, y - 0.25)
    gh = max(-0.075 - x, x - 0.085, y - 0.275)
    return min(gx, gh)


def target(route):
    """🎯 World heading of the last hose bone: the swivel exit 40 degrees off the wand axis on the chosen side."""
    return route["gripDeg"] + 180 + route["side"] * route.get("exit", EXIT)


def energy(h, route, mu):
    """⚖️ Bending energy, weight, socket alignment and penalties for tip, exit, floor, wand and canister."""
    G = route["grip"]
    g = math.radians(route["gripDeg"])
    w0 = (G[0] + 0.05 * math.cos(g), G[1] + 0.05 * math.sin(g))
    w1 = (G[0] + WAND_L * math.cos(g), G[1] + WAND_L * math.sin(g))
    p = pts_of(h)
    e = 0.003 * sum((h[i] - h[i - 1]) ** 2 for i in range(1, N)) + 0.001 * h[0] ** 2
    e += sum((p[i][1] + p[i + 1][1]) / 2 for i in range(N))
    e += mu * ((p[-1][0] - G[0]) ** 2 + (p[-1][1] - G[1]) ** 2)
    e += mu * 1e-5 * (h[-1] - target(route)) ** 2
    for q in p[1:]:
        if q[1] < HOSE_R + 0.001:
            e += mu * (HOSE_R + 0.001 - q[1]) ** 2
    for i in range(N - 1):
        d = seg_dist(p[i], p[i + 1], w0, w1, 10) - HOSE_R - WAND_R - 0.045
        if d < 0:
            e += mu * d * d
    for q in p[1:N]:
        c = route["side"] * (math.cos(g) * (q[1] - G[1]) - math.sin(g) * (q[0] - G[0]))
        if c < HOSE_R + WAND_R + 0.045:
            e += mu * (HOSE_R + WAND_R + 0.045 - c) ** 2
    for i in range(1, N):
        for k in range(6):
            t = k / 5
            d = canister_gap(p[i][0] + (p[i + 1][0] - p[i][0]) * t, p[i][1] + (p[i + 1][1] - p[i][1]) * t) - HOSE_R - 0.01
            if d < 0:
                e += mu * d * d
    return e


def optimise(route):
    """🔧 Normalised gradient descent with an increasing penalty weight."""
    h = list(route["init"])
    for mu in (1e2, 1e3, 1e4, 1e5, 1e6, 1e7):
        step = 2.0
        for _ in range(2500):
            e0 = energy(h, route, mu)
            gr = []
            for i in range(N):
                h[i] += 1e-3
                gr.append((energy(h, route, mu) - e0) / 1e-3)
                h[i] -= 1e-3
            n = math.sqrt(sum(x * x for x in gr))
            if n < 1e-12:
                break
            nh = [h[i] - step * gr[i] / n for i in range(N)]
            if energy(nh, route, mu) < e0:
                h = nh
                step *= 1.2
            else:
                step *= 0.5
            if step < 1e-6:
                break
    return h


def wrap(a):
    return (a + 540) % 360 - 180


def main():
    out = {}
    for name, route in ROUTES.items():
        h = optimise(route)
        p = pts_of(h)
        G = route["grip"]
        g = math.radians(route["gripDeg"])
        w0 = (G[0] + 0.05 * math.cos(g), G[1] + 0.05 * math.sin(g))
        w1 = (G[0] + WAND_L * math.cos(g), G[1] + WAND_L * math.sin(g))
        wand = min(seg_dist(p[i], p[i + 1], w0, w1) for i in range(N - 1)) - HOSE_R - WAND_R
        can = min(canister_gap(p[i][0] + (p[i + 1][0] - p[i][0]) * k / 10, p[i][1] + (p[i + 1][1] - p[i][1]) * k / 10) for i in range(1, N) for k in range(11)) - HOSE_R
        rel = [h[0]] + [h[i] - h[i - 1] for i in range(1, N)]
        grip = wrap(route["gripDeg"] - h[-1])
        print(f"[DEBUG] {name}: tip error {math.hypot(p[-1][0] - G[0], p[-1][1] - G[1]) * 1000:.2f} mm, lowest hose underside {(min(q[1] for q in p) - HOSE_R) * 1000:.1f} mm, wand {wand * 1000:.1f} mm, canister {can * 1000:.1f} mm, top {max(q[1] for q in p) + HOSE_R:.3f} m, max joint turn {max(abs(r) for r in rel[1:]):.1f}, exit {h[-1]:.1f} (target {target(route):.1f}), grip {grip:.1f}", file=sys.stderr)
        bones = {f"hose-{i + 1}": {"angleDeg": round(rel[i], 4)} for i in range(N)}
        bones["grip"] = {"angleDeg": round(grip, 4)}
        bones["nozzle"] = {"angleDeg": route["nozzleDeg"]}
        out[name] = bones
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
