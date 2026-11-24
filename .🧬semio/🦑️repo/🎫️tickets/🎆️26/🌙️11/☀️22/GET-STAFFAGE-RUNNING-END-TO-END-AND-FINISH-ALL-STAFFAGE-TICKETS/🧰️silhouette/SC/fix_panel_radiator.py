"""♨️ Repair of `panel-radiator`: (1) the plan of 1:50 and 1:100 filled the top grille as one rectangle, while the two middle rows of slots of the grille plate are open down to the floor between the panels (10 % of the plan; the fins below are texture): the grille is drawn with those sixteen slots open, as 1:200 already draws it; (2) the side elevation of 1:100 drew the upper wall bracket as its arm only, the 50 mm wall plate of the body stood 30.5 mm from the next drawn edge: the stroke follows the arm and the plate. Idempotent: python3 fix_panel_radiator.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "panel-radiator"
text, doc = sc.load(ID)
n = lambda v: format(sc.r6(v), "g")
OUTER = {50: (0.0335, -0.5015, 0.1365, 0.5015), 100: (0.035, -0.5, 0.135, 0.5)}
for scale in (50, 100):
    grille = sc.shape(doc, scale, "plan", "top-grille")
    plate = sc.solid(doc, scale, "grille-plate")
    x0, y0, x1, y1 = OUTER[scale]
    for key in ("x", "y", "w", "h"):
        grille.pop(key, None)
    grille["kind"] = "path"
    rings = [f"M {n(x0)} {n(y0)} L {n(x1)} {n(y0)} L {n(x1)} {n(y1)} L {n(x0)} {n(y1)} Z"]
    for hole in plate["holes"]:
        lo, hi = min(p[0] for p in hole), max(p[0] for p in hole)
        if not (-0.079 < lo and hi < -0.022):
            continue
        z0, z1 = min(p[1] for p in hole), max(p[1] for p in hole)
        a, b = 0.035 - hi, 0.035 - lo
        rings.append(f"M {n(b)} {n(z1)} L {n(b)} {n(z0)} L {n(a)} {n(z0)} L {n(a)} {n(z1)} Z")
    assert len(rings) == 17, len(rings)
    grille["d"] = " ".join(rings)
bracket = sc.shape(doc, 100, "elevation", "bracket-upper")
bracket["points"] = [[0.476, 0.0], [0.476, 0.0335], [0.445, 0.0335]]
sc.save(ID, text, doc)
