import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

def circle(id, bone, r, **kw):
    import math
    return poly(id, bone, [(r * math.cos(2 * math.pi * k / 48), r * math.sin(2 * math.pi * k / 48)) for k in range(48)], **kw)


PW = dict(regions=["wall", "base"], material="stainless")
WL = dict(region="wall", material="stainless")
HD = dict(region="handle", material="stainless")
WT = dict(region="water", material="water")

views = {}

views[200] = (
    [rect("pot-end", "pot", -0.086, 0, 0.172, 0.1, z=3.5, tone=0.4, **PW),
     rect("handle-end", "handles", -0.012, 0.064, 0.024, 0.036, z=3.6, tone=0.2, **HD)],
    [circle("pot-top", "pot", 0.086, z=3.5, tone=0.4, **PW),
     ell("water-top", "water", 0, 0, 0.055, 0.055, z=3.6, tone=0.45, **WT),
     rect("handle-left", "handles", -0.125, -0.012, 0.045, 0.024, z=3.7, tone=0.2, **HD),
     rect("handle-right", "handles", 0.08, -0.012, 0.045, 0.024, z=3.7, tone=0.2, **HD)],
    {"silhouette": ("Front: the pot as a 0.172 m wide wall strip with the near handle end-on in front; plan: the pot disc with the water disc inside and the two handle stubs on the x axis.",
                    "Vorderansicht: der Topf als 0,172 m breiter Wandstreifen mit dem nahen Griff von vorn davor; Draufsicht: die Topfscheibe mit der Wasserscheibe darin und den beiden Grifflaschen auf der x-Achse."),
     "omitted": ("The water is hidden behind the wall in front; the thin rim is merged into the pot disc in plan.",
                 "Das Wasser liegt in der Vorderansicht hinter der Wand; der dünne Rand geht in der Draufsicht in der Topfscheibe auf.")})

views[100] = (
    [rect("pot-end", "pot", -0.086, 0, 0.172, 0.1, z=3.5, tone=0.4, **PW),
     rect("handle-end", "handles", -0.01, 0.062, 0.02, 0.016, z=3.6, tone=0.2, **HD)],
    [circle("pot-top", "pot", 0.086, z=3.5, tone=0.4, **PW),
     ell("water-top", "water", 0, 0, 0.07, 0.07, z=3.6, tone=0.45, **WT),
     rect("handle-left", "handles", -0.125, -0.01, 0.041, 0.02, z=3.7, tone=0.2, **HD),
     rect("handle-right", "handles", 0.084, -0.01, 0.041, 0.02, z=3.7, tone=0.2, **HD)],
    {"silhouette": ("Front: the wall strip with the base line and the near handle end-on; plan: the pot ring with the water disc and the two handle lugs.",
                    "Vorderansicht: der Wandstreifen mit Bodenlinie und dem nahen Griff von vorn; Draufsicht: der Topfring mit der Wasserscheibe und den beiden Grifflaschen."),
     "retained": ("The rim ring stays 0.018 m wide so it prints as a separate band around the water.",
                  "Der Randring bleibt 0,018 m breit und druckt so als eigenes Band um das Wasser.")})

views[50] = (
    [rect("wall-end", "pot", -0.086, 0.008, 0.172, 0.092, z=3.5, tone=0.4, region="wall", material="stainless"),
     rect("base-end", "pot", -0.086, 0, 0.172, 0.008, z=3.5, tone=0.4, region="base", material="stainless"),
     poly("rim-line", "pot", [(-0.086, 0.094), (0.086, 0.094)], z=3.6, tone=0.3, role="detail", closed=False, region="wall"),
     rect("handle-end", "handles", -0.01, 0.062, 0.02, 0.016, z=3.6, tone=0.2, **HD),
     rect("handle-rivet", "handles", -0.004, 0.066, 0.008, 0.008, z=3.7, tone=0.5, role="detail", **HD)],
    [ell("pot-top", "pot", 0, 0, 0.086, 0.086, z=3.5, tone=0.4, **WL),
     ell("rim-inner", "pot", 0, 0, 0.08, 0.08, z=3.55, tone=0.3, role="detail", filled=False, **WL),
     ell("water-top", "water", 0, 0, 0.072, 0.072, z=3.6, tone=0.45, **WT),
     ell("bubble-a", "surface", -0.03, 0.02, 0.006, 0.006, z=3.7, tone=0.9, role="detail", **WT),
     ell("bubble-b", "surface", 0.025, 0.03, 0.005, 0.005, z=3.7, tone=0.9, role="detail", **WT),
     ell("bubble-c", "surface", 0.0, -0.02, 0.007, 0.007, z=3.7, tone=0.9, role="detail", **WT),
     ell("bubble-d", "surface", 0.04, -0.015, 0.005, 0.005, z=3.7, tone=0.9, role="detail", **WT),
     ell("bubble-e", "surface", -0.04, -0.03, 0.005, 0.005, z=3.7, tone=0.9, role="detail", **WT),
     rect("handle-left", "handles", -0.125, -0.01, 0.041, 0.02, z=3.8, tone=0.2, **HD),
     rect("handle-right", "handles", 0.084, -0.01, 0.041, 0.02, z=3.8, tone=0.2, **HD),
     ell("rivet-left", "handles", -0.108, 0, 0.003, 0.003, z=3.9, tone=0.5, role="detail", **HD),
     ell("rivet-right", "handles", 0.108, 0, 0.003, 0.003, z=3.9, tone=0.5, role="detail", **HD)],
    {"silhouette": ("Front: wall, base and rim line with the near handle and its rivet end-on; plan: pot wall, rim ring, water disc with five boiling bubbles and the two riveted handle lugs.",
                    "Vorderansicht: Wand, Boden und Randlinie mit dem nahen Griff und seiner Niete von vorn; Draufsicht: Topfwand, Randring, Wasserscheibe mit fünf Kochblasen und die beiden vernieteten Grifflaschen."),
     "retained": ("The bubbles are bound to the surface bone and mark the boiling pose in plan.",
                  "Die Blasen sind an den Oberflächenknochen gebunden und kennzeichnen die kochende Pose in der Draufsicht.")})

apply("cooking-pot", views)
