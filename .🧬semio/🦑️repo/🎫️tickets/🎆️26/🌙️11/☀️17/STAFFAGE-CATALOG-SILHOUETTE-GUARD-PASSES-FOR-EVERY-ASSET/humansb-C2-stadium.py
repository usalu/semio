"""🏟️ Emits a stadium path (capsule outline) for a shape op. python3 humansb-C2-stadium.py x0 y0 x1 y1 r [n]"""
import math, sys
x0, y0, x1, y1, r = map(float, sys.argv[1:6]); n = int(sys.argv[6]) if len(sys.argv) > 6 else 8
a = math.atan2(y1 - y0, x1 - x0); pts = []
for i in range(n + 1): t = a - math.pi / 2 + math.pi * i / n; pts.append((x1 + r * math.cos(t), y1 + r * math.sin(t)))
for i in range(n + 1): t = a + math.pi / 2 + math.pi * i / n; pts.append((x0 + r * math.cos(t), y0 + r * math.sin(t)))
print("M " + " L ".join(f"{x:.4f} {y:.4f}" for x, y in pts) + " Z")
