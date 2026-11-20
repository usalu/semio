"""🛴 Round 4 review fixes of humans/child-on-kick-scooter, every row stated by hand: clean garment joints (knee, elbow, blouson over its hem band) and the anatomical pelvis and skull of the structure list at every scale (child: small face under a large braincase). Reads the catalog volumes, writes `volumes/humans/child-on-kick-scooter.json`."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("g11", os.path.join(T, "r4-g11-kit.py"))
G = importlib.util.module_from_spec(spec)
spec.loader.exec_module(G)

SLUG = "child-on-kick-scooter"
V = G.load(SLUG)

G.edit(V, "50", "thigh-g", 0.30, 1.0, [(0.302, 0.004, 0.0, 0.045, 0.044), (0.318, 0.003, 0.0, 0.044, 0.043), (0.329, 0.002, 0.0, 0.037, 0.036), (0.335, 0.000, 0.0, 0.024, 0.024)])
G.edit(V, "50", "shank-g", -1.0, 0.031, [(-0.030, 0.002, 0.0, 0.018, 0.018), (-0.016, 0.003, 0.0, 0.029, 0.028), (0.000, 0.004, 0.0, 0.035, 0.034), (0.030, 0.002, 0.0, 0.039, 0.038)])
G.edit(V, "50", "sleeve-up", 0.22, 1.0, [(0.2286, -0.002, 0.0, 0.035, 0.036), (0.2436, -0.002, 0.0, 0.0316, 0.0325), (0.2556, -0.002, 0.0, 0.0222, 0.023), (0.2616, -0.002, 0.0, 0.0115, 0.012)])
G.edit(V, "50", "sleeve-fore", -1.0, 0.001, [(-0.026, 0.000, 0.0, 0.014, 0.014), (-0.014, 0.000, 0.0, 0.023, 0.023), (0.000, -0.002, 0.0, 0.029, 0.030)])
G.edit(V, "50", "torso", -1.0, -0.02, [(-0.040, -0.005, 0.0, 0.128, 0.095, 2.35), (-0.024, -0.004, 0.0, 0.126, 0.093, 2.35)], sides=None)
G.edit(V, "50", "skirt", -1.0, 1.0, [(-0.016, 0.005, 0.0, 0.121, 0.087, 2.4), (-0.008, 0.005, 0.0, 0.124, 0.090, 2.4), (0.006, 0.004, 0.0, 0.121, 0.088, 2.4), (0.030, -0.001, 0.0, 0.114, 0.083, 2.4), (0.052, -0.004, 0.0, 0.108, 0.078, 2.4), (0.078, -0.004, 0.0, 0.106, 0.076, 2.4), (0.095, -0.004, 0.0, 0.088, 0.060, 2.35)], sides=None)
G.edit(V, "100", "torso", -1.0, -0.017, [(-0.040, -0.005, 0.0, 0.128, 0.095, 2.35), (-0.020, -0.004, 0.0, 0.126, 0.093, 2.35)], sides=None)
G.edit(V, "100", "skirt", -1.0, 1.0, [(-0.016, 0.005, 0.0, 0.122, 0.088, 2.4), (0.004, 0.004, 0.0, 0.121, 0.088, 2.4), (0.048, -0.004, 0.0, 0.108, 0.078, 2.4), (0.078, -0.004, 0.0, 0.106, 0.076, 2.4), (0.095, -0.004, 0.0, 0.088, 0.060, 2.35)], sides=None)

PELVIS = {"hip": 0.065, "asis": (0.058, -0.040, 0.085), "psis": (0.062, 0.048, 0.032), "tuber": (-0.052, 0.018, 0.045), "pubis": (-0.030, -0.042), "prom": (0.060, 0.008), "coccyx": (-0.008, 0.038), "k": 0.74}
HEAD = {"o": (-0.004, -0.006), "len": 0.084, "wid": 0.064, "hei": 0.060, "face": 0.68}
for scale in ("50", "100", "200", "500"):
    G.swap(V, scale, {"pelvis-bone", "ilium-l", "ilium-r", "pubis-l", "pubis-r", "pubis", "ischium-l", "ischium-r", "ischiopubic-ramus-l", "ischiopubic-ramus-r", "sacrum"}, G.pelvis(scale, "pelvis", "skin-child", 0.92, PELVIS))
    G.swap(V, scale, {"skull", "jaw", "mandible", "maxilla", "facial-skeleton", "mandible-ramus-l", "mandible-ramus-r"}, G.skull(scale, "head", "skin-child", 0.92, HEAD))
print(G.save(SLUG, V))
