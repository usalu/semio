"""♿️ Repair of `wheelchair-ramp` at 1:200: the two handrails (42.4 mm tube, bent over the landing, with a return at each end) were declared `slender`; the kernel finds no centre line for the bent tube and the comparison then takes the straight chord between its ends (105 mm from the drawn rail, IoU 0.921). At 1:200 the tube is thicker than the smallest feature (30 mm), a body of its own: it is compared as the tube it is, which the 36 mm stroke of the drawing follows over the landing and down the returns. Idempotent: python3 fix_wheelchair_ramp.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "wheelchair-ramp"
text, doc = sc.load(ID)
for name in ("rail-a", "rail-b"):
    sc.solid(doc, 200, name).pop("slender", None)
sc.save(ID, text, doc)
