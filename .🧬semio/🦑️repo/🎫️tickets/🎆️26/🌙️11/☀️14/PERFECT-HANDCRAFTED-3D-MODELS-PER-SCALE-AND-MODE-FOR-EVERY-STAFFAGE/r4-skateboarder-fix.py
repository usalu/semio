"""🛹 Round 4 review fixes of humans/skateboarder, every row stated by hand: sleeve caps lowered to the real shoulder line (no pads above the trapezius slope) and the anatomical pelvis and skull of the structure list at every scale. Reads the catalog volumes, writes `volumes/humans/skateboarder.json`."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("g11", os.path.join(T, "r4-g11-kit.py"))
G = importlib.util.module_from_spec(spec)
spec.loader.exec_module(G)

SLUG = "skateboarder"
V = G.load(SLUG)

G.edit(V, "50", "sleeve", -1.0, -0.029, [(-0.060, 0.0, -0.012, 0.012, 0.012), (-0.054, 0.0, -0.010, 0.030, 0.032), (-0.040, 0.0, -0.006, 0.044, 0.046), (-0.022, 0.0, -0.002, 0.053, 0.054)])
G.edit(V, "50", "hoodie", 0.44, 0.48, [(0.455, 0.002, 0.0, 0.184, 0.094, 2.6), (0.478, 0.004, 0.0, 0.150, 0.084, 2.3)], sides=None)
G.edit(V, "100", "sleeve", -1.0, -0.05, [(-0.060, 0.0, -0.012, 0.012, 0.012), (-0.040, 0.0, -0.006, 0.044, 0.046)])
G.edit(V, "200", "sleeve", -1.0, -0.02, [(-0.058, 0.0, -0.010, 0.016, 0.016), (-0.026, 0.0, -0.002, 0.052, 0.053)])
G.edit(V, "500", "sleeve", -1.0, -0.07, [(-0.045, 0.0, -0.008, 0.040, 0.040)])

PELVIS = {"hip": 0.087, "asis": (0.085, -0.040, 0.120), "psis": (0.095, 0.088, 0.045), "tuber": (-0.072, 0.045, 0.055), "pubis": (-0.045, -0.052), "prom": (0.092, 0.050), "coccyx": (-0.010, 0.082)}
HEAD = {"o": (0.0844, -0.0618), "tilt": 16.4, "len": 0.092, "wid": 0.070, "hei": 0.068, "face": 1.0}
for scale in ("50", "100", "200", "500"):
    G.swap(V, scale, {"pelvis-bone", "ilium-l", "ilium-r", "pubis-l", "pubis-r", "pubis", "ischium-l", "ischium-r", "ischiopubic-ramus-l", "ischiopubic-ramus-r", "sacrum"}, G.pelvis(scale, "pelvis", "skin", 0.9, PELVIS))
    G.swap(V, scale, {"skull", "jaw", "mandible", "maxilla", "facial-skeleton", "mandible-ramus-l", "mandible-ramus-r"}, G.skull(scale, "head", "skin", 0.9, HEAD))
print(G.save(SLUG, V))
