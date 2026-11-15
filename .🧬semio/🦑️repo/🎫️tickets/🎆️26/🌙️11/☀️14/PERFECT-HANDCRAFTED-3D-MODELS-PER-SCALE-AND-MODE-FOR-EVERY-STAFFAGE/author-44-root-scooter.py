import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from author_14_root import add_shape

cx, cy, R, r = -0.43, 0.108, 0.116, 0.100
n = 14
a0, a1 = math.radians(10), math.radians(170)
outer = [[round(cx + R * math.cos(a0 + (a1 - a0) * k / n), 4), round(cy + R * math.sin(a0 + (a1 - a0) * k / n), 4)] for k in range(n + 1)]
inner = [[round(cx + r * math.cos(a1 - (a1 - a0) * k / n), 4), round(cy + r * math.sin(a1 - (a1 - a0) * k / n), 4)] for k in range(n + 1)]
shape = {"kind": "poly", "points": outer + inner, "closed": True, "id": "fender-r", "bone": "deck", "role": "body", "z": 3.5, "filled": True, "tone": 0.55, "region": "fenders", "material": "plastic"}
add_shape("electric-kick-scooter", "100", "elevation", "deck", shape)
print("added elevation fender-r at 1:100")
