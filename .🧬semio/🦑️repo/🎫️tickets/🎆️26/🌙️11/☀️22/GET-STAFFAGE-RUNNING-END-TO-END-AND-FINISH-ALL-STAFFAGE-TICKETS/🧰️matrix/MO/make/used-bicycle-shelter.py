"""🚲️ Writes the repair ops of `used-bicycle-shelter`: the bike rests on two tyres and its side stand, and the band 1:500 drew and modelled no side stand, so the contact anchor `bike-kickstand` lay 105 to 143 mm off every drawn shape of the bike. The band gets the side stand as it generalises every tube: a 75 mm band from the bottom bracket to the ground, in the three views and as a body solid with its thermal and structural reference, at the place the band 1:200 draws it; the decision texts say so."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write, GEOMETRY
a = asset("used-bicycle-shelter")
R = 0.0375
ORIGIN = (-0.55, 0.351, -0.38)
TOP, FOOT = (0.47, -0.05, 0.03), (0.29, round(-0.351 + R, 6), 0.18)
r6 = lambda v: round(v, 6)
ends = {
    "elevation": ((TOP[0], TOP[1]), (FOOT[0], FOOT[1])),
    "front": ((r6(ORIGIN[2] + TOP[2]), r6(ORIGIN[1] + TOP[1])), (r6(ORIGIN[2] + FOOT[2]), r6(ORIGIN[1] + FOOT[1]))),
    "plan": ((r6(ORIGIN[0] + TOP[0]), r6(ORIGIN[2] + TOP[2])), (r6(ORIGIN[0] + FOOT[0]), r6(ORIGIN[2] + FOOT[2]))),
}
ops = []
for view, ((x0, y0), (x1, y1)) in ends.items():
    fine = shape(a, 200, view, "kickstand")
    ops.append({"op": "shape-add", "scale": "500", "view": view, "after": "bar", "shape": like(fine, drop=GEOMETRY, kind="capsule", x0=x0, y0=y0, x1=x1, y1=y1, r0=R, r1=R)})
ops.append({"op": "solid-add", "scale": "500", "after": "bar", "solid": {"id": "kickstand", "bone": "bike", "material": "bike-steel", "role": "body", "tone": 0.35, "kind": "capsule", "p0": list(TOP), "p1": list(FOOT), "r0": R, "r1": R}})
ops.append({"op": "solid-add", "scale": "500", "list": "thermal", "after": "th-bar", "solid": {"id": "th-kickstand", "of": "kickstand", "region": "bike-frame"}})
ops.append({"op": "solid-add", "scale": "500", "list": "structure", "solid": {"id": "m-kickstand", "of": "kickstand", "member": "bike", "part": "member"}})
text = dict(a["volumes"]["500"]["decisions"][0])
text["en"] = text["en"].replace("saddle and bars (side stand, rack and chainring left out)", "saddle, bars and the side stand as a 75 mm tube, because the bike rests on it (rack and chainring left out)")
text["de"] = text["de"].replace("Sattel und Lenker (Seitenständer, Gepäckträger und Kettenblatt entfallen)", "Sattel, Lenker und der Seitenständer als 75-mm-Rohr, weil das Rad darauf steht (Gepäckträger und Kettenblatt entfallen)")
ops.append({"op": "set", "path": ["volumes", "500", "decisions", 0], "value": text})
ops.append({"op": "note", "scale": "500", "decision": "silhouette", "en": "The side stand of the bike is a 75 mm band: it carries the third contact of the bike.", "de": "Der Seitenständer des Rads ist ein 75-mm-Band: er trägt den dritten Aufstandspunkt des Rads."})
write("used-bicycle-shelter", ops)
