"""🎻️ Writes the repair ops of `used-double-bass`: the stool stands on four glides at the ends of a star base, 0.262 m from the column on the diagonals. The band 1:500 merged the star into a disc of 0.197 m radius, the half width the star shows in the side elevation and the front, so the four contact anchors `stool-foot-0` … `stool-foot-3` lay 66 mm outside it in the plan. The merged base becomes the square plate that has the glides at its corners: the same 0.393 m in the side elevation and the front, a square instead of a circle in the plan and a box instead of a disc in the body; the decision texts say so."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write, GEOMETRY
a = asset("used-double-bass")
R, X = 0.196576, -1.15
r6 = lambda v: round(v, 6)
ops = [{"op": "shape", "scale": "500", "view": "plan", "id": "stool-base", "shape": like(shape(a, 500, "plan", "stool-base"), drop=GEOMETRY, kind="poly", points=[[r6(X - R), -R], [r6(X + R), -R], [r6(X + R), R], [r6(X - R), R]], closed=True)}]
old = next(s for s in a["volumes"]["500"]["body"] if s["id"] == "stool-base")
ops.append({"op": "solid", "scale": "500", "id": "stool-base", "solid": {**{k: old[k] for k in ("id", "bone", "material", "role", "tone")}, "kind": "box", "size": [r6(2 * R), 0.075, r6(2 * R)], "transform": {"translation": [0, 0.0375, 0]}}})
SWAPS = {
    ("representations", "500", "decisions", "silhouette"): (("the stool as base disc", "the stool as square base plate"), ("der Hocker als Fußscheibe", "der Hocker als quadratische Fußplatte")),
    ("representations", "500", "decisions", "merged"): (("the star base with glides and hub into one disc", "the star base with glides and hub into one square plate with the four glides at its corners"), ("Sternfuß mit Gleitern und Nabe in eine Scheibe", "Sternfuß mit Gleitern und Nabe in eine quadratische Platte mit den vier Gleitern an den Ecken")),
    ("volumes", "500", "decisions", 1): (("as one disc 75 mm high", "as one square plate 75 mm high with the glides at its corners"), ("als eine 75 mm hohe Scheibe", "als eine 75 mm hohe quadratische Platte mit den Gleitern an den Ecken")),
}
for path, ((en0, en1), (de0, de1)) in SWAPS.items():
    node = a
    for step in path:
        node = node[step]
    text = {"en": node["en"].replace(en0, en1), "de": node["de"].replace(de0, de1)}
    assert en1 in text["en"] and de1 in text["de"], path
    ops.append({"op": "set", "path": list(path), "value": text})
write("used-double-bass", ops)
