"""Redraws the 1:500 representation of the two wall components (the validator rule `representation-copied`): the build-up of the etics wall becomes one wall outline per view (the 0.38 m and 0.32 m layers are 0.76 mm and 0.64 mm on paper, below a drawable layer), the solid brick wall is drawn as a closed polygon outline poché with its outer ring wound the other way and a darker tone."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from catalog_edit import S, edit
def rect_poly(s, tone):
    x, y, w, h = s["x"], s["y"], s["w"], s["h"]
    new = {k: v for k, v in s.items() if k not in ("kind", "x", "y", "w", "h", "r")}
    new["tone"] = tone
    new["kind"] = "poly"; new["points"] = [[x, y + h], [x + w, y + h], [x + w, y], [x, y]]; new["closed"] = True
    return new
def etics(rep):
    if rep.get("denominator") != 500 or rep.get("status") != "authored": return None
    g = rep["geometry"]
    if [s["id"] for s in g["elevation"]] != ["brick", "insulation"]: return None
    wall = {**g["elevation"][0], "id": "wall", "tone": 0.65, "w": 0.7}
    wallp = {**g["plan"][0], "id": "wall-plan", "tone": 0.65, "w": 0.7}
    g["elevation"] = [wall]; g["plan"] = [wallp]
    return rep
def solid(rep):
    if rep.get("denominator") != 500 or rep.get("status") != "authored": return None
    g = rep["geometry"]
    if g["elevation"][0].get("kind") != "rect": return None
    for vp in ("elevation", "front", "plan"): g[vp] = [rect_poly(s, 0.4) for s in g[vp]]
    return rep
print(edit(S + "🧱️components/etics-brick-wall-30/🔣️.json", lambda t: '"denominator": 500' in t and '"geometry"' in t and t.count('"geometry"') == 1, etics))
print(edit(S + "🧱️components/solid-brick-wall-1960/🔣️.json", lambda t: '"denominator": 500' in t and '"geometry"' in t and t.count('"geometry"') == 1, solid))
