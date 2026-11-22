"""🔍 Probe of the roll-up: lowest point of the cover and the bass along the tilt for a pivot, printed per instant."""
import importlib.util, math, sys
spec = importlib.util.spec_from_file_location("bass", "author-double-bass.py")
m = importlib.util.module_from_spec(spec); sys.argv = ["x"]; spec.loader.exec_module(m)
m.rest_geometry(); m.H_WORLD = (0, 0, -0.325)
phi = m.CARRY_DEG + m.DELTA
pts = [(u, v) for u, v in m.bag_loop(50)] + [(u, s * rx) for u, rx, _ in m.SLEEVE for s in (-1, 1)]
W = [m.cover_to_world((u, v, 0)) for u, v in pts]
axle = m.cover_to_world((m.AXLE_U, 0, 0))
c, s = math.cos(math.radians(phi)), math.sin(math.radians(phi))
for qx in (-0.9, -0.5, -0.2, 0.0, 0.2, 0.4):
    qy = (m.WHEEL_R - s * (axle[0] - qx) - c * axle[1]) / (1 - c)
    row = []
    for t in (0.25, 0.5, 0.75, 1.0):
        a = math.radians(phi * t); ca, sa = math.cos(a), math.sin(a)
        ys = [(qy + sa * (x - qx) + ca * (y - qy), i) for i, (x, y, _) in enumerate(W)]
        lo = min(ys)
        row.append(f"t{t}: {lo[0]:+.3f} @u{pts[lo[1]][0]:.2f},v{pts[lo[1]][1]:+.2f}")
    print(f"[DEBUG] q=({qx:+.2f},{qy:+.2f})", " | ".join(row))
