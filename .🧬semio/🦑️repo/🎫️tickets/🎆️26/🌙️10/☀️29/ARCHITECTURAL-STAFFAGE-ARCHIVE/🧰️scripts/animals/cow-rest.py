# [scratch] derives dairy-cow rest bone frames from world joint targets (ground frame, x forward from the hip joint, y up)
import math, json, sys
H = 1.10
def vec(a, b): return (b[0]-a[0], b[1]-a[1])
def L(v): return math.hypot(*v)
def A(v): return math.degrees(math.atan2(v[1], v[0]))
pts = {
 "hip": (0, H),
 "stifle": (0.26, 0.80), "hock": (-0.04, 0.42), "hind-fet": (-0.01, 0.13),
 "sh": (1.37, 0.97), "elbow": (1.07, 0.78), "carpus": (1.10, 0.39), "fore-fet": (1.12, 0.13),
}
hind = [("thigh", "hip", "stifle"), ("shank", "stifle", "hock"), ("metatarsus", "hock", "hind-fet")]
for n, a, b in hind: v = vec(pts[a], pts[b]); print("hind", n, round(L(v), 3), round(A(v), 1))
fore = [("humerus", "sh", "elbow"), ("forearm", "elbow", "carpus"), ("cannon", "carpus", "fore-fet")]
for n, a, b in fore: v = vec(pts[a], pts[b]); print("fore", n, round(L(v), 3), round(A(v), 1))
