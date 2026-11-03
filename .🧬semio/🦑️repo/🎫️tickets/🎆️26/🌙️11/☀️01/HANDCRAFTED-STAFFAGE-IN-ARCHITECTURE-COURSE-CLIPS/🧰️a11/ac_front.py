"""Hand-designed front elevation of split-ac-indoor-unit (0.85 m wide, 0.29 m high, origin at the lower edge, x from the centre).

python ac_front.py <asset.json>
"""
import json
import sys


def r5(v):
    return round(v, 5)


def rect(id, role, z, filled, tone, x, y, w, h, region=None, material=None, r=None):
    shape = {"id": id, "bone": "unit", "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        shape["region"] = region
    if material:
        shape["material"] = material
    shape.update({"kind": "rect", "x": r5(x), "y": r5(y), "w": r5(w), "h": r5(h)})
    if r:
        shape["r"] = r
    return shape


def path(id, role, z, tone, d):
    return {"id": id, "bone": "unit", "role": role, "z": z, "filled": False, "tone": tone, "kind": "path", "d": d}


def slats(count, y0, y1, half):
    pitch = 2 * half / count
    return " ".join(f"M {r5(-half + pitch * (i + 0.5))} {y0} L {r5(-half + pitch * (i + 0.5))} {y1}" for i in range(count))


def front(scale):
    shapes = [rect("casing", "body", 1, True, 0.88, -0.425, 0, 0.85, 0.29, "casing", "abs-white", 0.045)]
    if scale == 200:
        shapes.append(rect("intake-grille", "body", 2, True, 0.3, -0.38, 0.235, 0.76, 0.04, "intake-grille", "abs-grey"))
        shapes.append(rect("discharge-vane", "body", 2, True, 0.6, -0.39, 0.02, 0.78, 0.05, "discharge-vane", "abs-grey", 0.012))
        return shapes
    shapes.append(rect("intake-grille", "detail", 2, True, 0.3, -0.39, 0.232, 0.78, 0.045, "intake-grille", "abs-grey"))
    shapes.append(path("grille-slats", "detail", 2.2, 0.7, slats(15 if scale == 50 else 8, 0.238, 0.271, 0.37)))
    shapes.append(path("front-split", "detail", 1.5, 0.5, "M -0.41,0.215 L 0.41,0.215"))
    shapes.append(rect("discharge-vane", "detail", 2, True, 0.6, -0.39, 0.014, 0.78, 0.04, "discharge-vane", "abs-grey", 0.012))
    shapes.append(rect("display-window", "detail", 1.6, True, 0.15, 0.32, 0.12, 0.06, 0.035, None, "display", 0.006))
    return shapes


def tx(en, de):
    return {"en": en, "de": de}


ADD = {
    "50": ("silhouette", tx(" Front elevation: the 0.85 m casing with rounded ends, the slatted intake grille strip along the top, the panel split, the long discharge vane along the bottom edge and the display window at the right end.", " Vorderansicht: das 0,85 m lange Gehäuse mit gerundeten Enden, der Lamellen-Ansaugstreifen oben, die Blendenteilung, die lange Ausblaslamelle an der Unterkante und das Anzeigefenster am rechten Ende.")),
    "100": ("silhouette", tx(" Front elevation: casing, intake strip with eight slats, panel split, discharge vane and display window.", " Vorderansicht: Gehäuse, Ansaugstreifen mit acht Lamellen, Blendenteilung, Ausblaslamelle und Anzeigefenster.")),
    "200": ("silhouette", tx(" Front elevation: casing, intake strip and discharge vane as three filled forms.", " Vorderansicht: Gehäuse, Ansaugstreifen und Ausblaslamelle als drei gefüllte Formen.")),
}


def main():
    path_ = sys.argv[1]
    doc = json.load(open(path_, encoding="utf-8"))
    asset = doc["asset"]
    for key in ("200", "100", "50"):
        rep = asset["representations"][key]
        rep["geometry"]["front"] = front(int(key))
        field, add = ADD[key]
        rep["decisions"][field] = {lang: rep["decisions"][field][lang] + add[lang] for lang in ("en", "de")}
    asset["presentation"]["viewpoints"] = ["elevation", "front", "plan"]
    json.dump(doc, open(path_, "w", encoding="utf-8", newline="\n"), ensure_ascii=False, indent=1)


main()
