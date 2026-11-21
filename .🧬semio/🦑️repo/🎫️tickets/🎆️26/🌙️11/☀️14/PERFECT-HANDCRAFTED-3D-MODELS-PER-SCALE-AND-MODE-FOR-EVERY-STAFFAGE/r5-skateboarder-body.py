"""🛹 Round 5 handcraft of humans/skateboarder, every row stated by hand on top of the round 4 source: relaxed hands with grouped, gently flexed fingers (no hooks), a knee where the shank's condyles carry the thigh end (no ball), a trouser seat that narrows into the crotch between the thighs (no brief panel), at 1:50; the same knee at 1:100; the skull of the structure list seated 4 mm higher and 5 mm further back with its axis 10 degrees from the head bone (was 16.4) and a 12 % smaller facial skeleton at every scale, so the mandible lies inside the chin instead of hanging below it. Reads `🗑️generated/r5-skateboarder/volumes.before.json` (the round 4 source), writes `volumes/humans/skateboarder.json`."""
import copy
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))


def mod(path, name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(T, path))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


G = mod("r4-g11-kit.py", "g11")
B = mod("r3-skateboarder-body50.py", "body50")
K = B.K
SRC = os.path.join(T, "🗑️generated", "r5-skateboarder", "volumes.before.json")
OUT = os.path.join(T, "volumes", "humans", "skateboarder.json")

V = json.load(open(SRC, encoding="utf-8"))["volumes"]


def put(scale, solid):
    """🔁 Replaces the body solid with the same id at a scale (keys stripped of the private `_` tags)."""
    lst = V[scale]["body"]
    k = next(i for i, e in enumerate(lst) if e["id"] == solid["id"])
    lst[k] = {a: b for a, b in solid.items() if not a.startswith("_")}


B.CURL = (8.0, 24.0, 38.0)
B.PHALANX = (0.047, 0.029, 0.023)
for s, n in B.SIDES:
    for k, (v0, sc, r) in enumerate(((0.0285, 0.95, 0.0098), (0.0095, 1.0, 0.0100), (-0.0095, 0.96, 0.0096), (-0.0275, 0.80, 0.0088))):
        put("50", B.finger(f"finger-{n}-{k}", f"hand-{n}", v0, sc, r, s))
    medial = -s
    put("50", K.tube(f"thumb-{n}", f"hand-{n}", [([0.006, 0.024, medial * 0.006], 0.0135, 0.0125), ([0.036, 0.036, medial * 0.012], 0.0118, 0.0110), ([0.064, 0.040, medial * 0.016], 0.0100, 0.0094), ([0.086, 0.038, medial * 0.019], 0.0088, 0.0082), ([0.100, 0.034, medial * 0.020], 0.0072, 0.0068)], B.SKIN, B.TONE["skin"], hint=K.W, role="detail"))

G.edit(V, "50", "thigh", 0.395, 1.0, [(0.400, 0.008, 0.0, 0.063, 0.062), (0.420, 0.006, 0.0, 0.061, 0.061), (0.446, 0.004, 0.0, 0.052, 0.052), (0.470, 0.002, 0.0, 0.034, 0.034), (0.482, 0.000, 0.0, 0.016, 0.016)])
G.edit(V, "50", "shank", -1.0, 0.06, [(-0.050, 0.006, 0.0, 0.020, 0.020), (-0.034, 0.005, 0.0, 0.042, 0.044), (-0.014, 0.003, 0.0, 0.056, 0.058), (0.004, 0.0, 0.0, 0.062, 0.063), (0.050, -0.004, 0.0, 0.062, 0.060)])
G.edit(V, "50", "trousers-seat", -1.0, 0.0, [(-0.116, 0.010, 0.0, 0.018, 0.026, 2.0), (-0.104, 0.016, 0.0, 0.052, 0.058, 2.1), (-0.088, 0.020, 0.0, 0.098, 0.080, 2.2), (-0.072, 0.025, 0.0, 0.140, 0.091, 2.3), (-0.040, 0.031, 0.0, 0.180, 0.103, 2.4), (0.000, 0.026, 0.0, 0.184, 0.106, 2.4)], sides=None)

G.edit(V, "100", "thigh", 0.395, 1.0, [(0.400, 0.010, 0.0, 0.064, 0.061), (0.448, 0.005, 0.0, 0.054, 0.052), (0.486, 0.000, 0.0, 0.030, 0.030)])
G.edit(V, "100", "shank", -1.0, 0.06, [(-0.050, 0.006, 0.0, 0.052, 0.054), (-0.018, 0.005, 0.0, 0.063, 0.063), (0.050, -0.004, 0.0, 0.063, 0.059)])

HEAD = {"o": (0.0880, -0.0570), "tilt": 10.0, "len": 0.092, "wid": 0.070, "hei": 0.068, "face": 0.88}
for scale in ("50", "100", "200", "500"):
    G.swap(V, scale, {"skull", "jaw", "mandible", "maxilla", "facial-skeleton", "mandible-ramus-l", "mandible-ramus-r"}, G.skull(scale, "head", "skin", 0.9, HEAD))

json.dump({"volumes": V}, open(OUT, "w", encoding="utf-8"), ensure_ascii=False, separators=(",", ":"))
print(OUT)
