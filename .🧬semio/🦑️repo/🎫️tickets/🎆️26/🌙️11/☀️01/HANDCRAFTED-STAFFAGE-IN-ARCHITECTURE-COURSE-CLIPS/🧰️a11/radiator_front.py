"""Hand-designed front elevation of panel-radiator, written into the asset and manifest by text edit.

python radiator_front.py <asset.json>

Splits the front water panel into three thermal bands (water enters at the top), makes the valve a warm operating region, authors geometry.front at
1:200, 1:100 and 1:50 (world metres: x across the 1.0 m length from the centre, y up from the floor) and lists the viewpoint.
"""
import copy
import json
import sys

BANDS = [("panel-front-upper", "upper third", "oberes Drittel", 0.38), ("panel-front-mid", "middle third", "mittleres Drittel", 0.33), ("panel-front-lower", "lower third", "unteres Drittel", 0.29)]
FRONT_W = 444.0
Y0, YP, YT = 0.11, 0.675, 0.71
HALF = 0.5
MAT = "painted-steel"


def r5(v):
    return round(v, 5)


def rect(id, bone, role, z, filled, tone, x, y, w, h, **extra):
    shape = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    for key in ("region", "regions", "material", "penMm"):
        if key in extra:
            shape[key] = extra[key]
    shape.update({"kind": "rect", "x": r5(x), "y": r5(y), "w": r5(w), "h": r5(h)})
    if "r" in extra:
        shape["r"] = extra["r"]
    return shape


def path(id, bone, role, z, tone, d, **extra):
    shape = {"id": id, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone}
    for key in ("region", "regions", "penMm"):
        if key in extra:
            shape[key] = extra[key]
    shape.update({"kind": "path", "d": d})
    return shape


def line(id, bone, role, z, tone, pts, **extra):
    shape = {"id": id, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone}
    for key in ("region", "penMm"):
        if key in extra:
            shape[key] = extra[key]
    shape.update({"kind": "poly", "points": [[r5(x), r5(y)] for x, y in pts], "closed": False})
    return shape


def bands(yp, role="detail"):
    ys = [Y0 + (yp - Y0) * i / 3 for i in range(4)]
    out = []
    for i, (rid, _, _, _) in enumerate(reversed(BANDS)):
        out.append(rect("front-" + rid.split("-")[-1], "radiator", role, 3, True, 0.7, -HALF, ys[i], 2 * HALF, ys[i + 1] - ys[i], region=rid, material=MAT))
    return out


def slits(count, y0, y1):
    pitch = 2 * HALF / count
    return " ".join(f"M {r5(-HALF + pitch * (i + 0.5))} {y0} L {r5(-HALF + pitch * (i + 0.5))} {y1}" for i in range(count))


def valves(scale):
    """Thermostatic head left, lockshield cap right, on the two bottom connections 0.4 m from the centre."""
    if scale == 200:
        return [
            rect("valve-left", "wall", "detail", 4, True, 0.3, -0.56, 0.035, 0.13, 0.075, region="valve", material="brass"),
            rect("valve-right", "wall", "detail", 4, True, 0.3, 0.4, 0.035, 0.11, 0.075, region="valve", material="brass"),
        ]
    w = 0.03 if scale == 100 else 0.022
    out = [
        rect("pipe-left", "wall", "detail", 3, True, 0.3, -0.435 - w / 2, 0.0, w, 0.06, region="valve", material="brass"),
        rect("valve-body-left", "wall", "detail", 4, True, 0.3, -0.47, 0.05, 0.07, 0.06, region="valve", material="brass"),
        rect("valve-head", "wall", "detail", 5, True, 0.3, -0.55, 0.043, 0.08, 0.054, region="valve", material="brass", r=0.012),
        rect("pipe-right", "wall", "detail", 3, True, 0.3, 0.435 - w / 2, 0.0, w, 0.06, region="valve", material="brass"),
        rect("valve-body-right", "wall", "detail", 4, True, 0.3, 0.4, 0.05, 0.07, 0.06, region="valve", material="brass"),
        rect("lockshield-cap", "wall", "detail", 5, True, 0.3, 0.47, 0.056, 0.032, 0.042, region="valve", material="brass", r=0.006),
    ]
    if scale == 50:
        out.append(line("valve-index", "wall", "texture", 6, 0.1, [(-0.52, 0.049), (-0.52, 0.091)], region="valve"))
    return out


def front(scale):
    if scale == 200:
        yp = 0.66
        shapes = bands(yp)
        shapes.append(rect("grille-strip", "radiator", "detail", 4, True, 0.35, -HALF, yp, 2 * HALF, YT - yp, regions=["top-grille", "convector"], material=MAT))
    else:
        yp = YP
        shapes = bands(yp)
        shapes.append(rect("grille-strip", "radiator", "detail", 4, True, 0.35, -HALF, yp, 2 * HALF, YT - yp, region="top-grille", material=MAT))
        shapes.append(path("grille-fins", "radiator", "texture", 5, 0.2, slits(20 if scale == 50 else 10, r5(yp + 0.0055), r5(YT - 0.0055)), region="convector"))
    shapes.append(rect("radiator-outline", "radiator", "body", 6, False, 0.5, -HALF, Y0, 2 * HALF, YT - Y0))
    if scale == 50:
        for side, x in (("left", -0.47), ("right", 0.47)):
            shapes.append(line("end-seam-" + side, "radiator", "detail", 5, 0.5, [(x, Y0 + 0.02), (x, yp - 0.02)]))
    shapes.extend(valves(scale))
    return shapes


def tx(en, de):
    return {"en": en, "de": de}


DECISIONS = {
    "50": {
        "silhouette": tx(" Front elevation (1.0 m long, 0.60 m high, 0.11 m above the floor): the front plate in three bands (water enters at the top, the plate cools towards the return) over its outline, a slotted grille strip at the top edge with the convector fin tips, the two end seams, the thermostatic head on the left and the lockshield cap on the right bottom connection, each with pipe and valve body.",
                         " Vorderansicht (1,0 m lang, 0,60 m hoch, 0,11 m über dem Boden): die Frontplatte in drei Bändern (das Wasser tritt oben ein, die Platte kühlt zum Rücklauf hin ab) über ihrer Kontur, ein Schlitzgitterstreifen an der Oberkante mit den Konvektorlamellenspitzen, die beiden Stirnnähte, links der Thermostatkopf und rechts die Rücklaufverschraubung am unteren Anschluss, je mit Rohr und Ventilkörper."),
        "merged": tx(" Front: the two pressed borders between the bands are the band edges; the top grille lip and the convector fin tips share one strip.", " Vorderansicht: die beiden Sicken zwischen den Bändern sind die Bandkanten; Gitterlippe und Konvektorlamellenspitzen teilen sich einen Streifen."),
        "omitted": tx(" Front: rear panel, brackets, wall and the fins behind the plate (not visible from the front).", " Vorderansicht: Rückplatte, Konsolen, Wand und die Lamellen hinter der Platte (von vorn nicht sichtbar)."),
    },
    "100": {
        "silhouette": tx(" Front elevation: front plate in three bands over its outline, slotted grille strip with ten fin slits, thermostatic head and lockshield cap with pipes and valve bodies.",
                         " Vorderansicht: Frontplatte in drei Bändern über ihrer Kontur, Schlitzgitterstreifen mit zehn Lamellenschlitzen, Thermostatkopf und Rücklaufverschraubung mit Rohren und Ventilkörpern."),
        "merged": tx(" Front: the head index mark and the end seams (0.03 m from the edge, 0.3 mm) are merged into the plate edge.", " Vorderansicht: Kopfmarke und Stirnnähte (0,03 m vom Rand, 0,3 mm) sind in die Plattenkante aufgenommen."),
        "omitted": tx(" Front: rear panel, brackets, wall.", " Vorderansicht: Rückplatte, Konsolen, Wand."),
    },
    "200": {
        "silhouette": tx(" Front elevation: three plate bands, one grille strip (fin tips merged in) and a block each for the thermostatic head and the lockshield.",
                         " Vorderansicht: drei Plattenbänder, ein Gitterstreifen (Lamellenspitzen darin zusammengefasst) und je ein Block für Thermostatkopf und Rücklaufverschraubung."),
        "merged": tx(" Front: the grille lip and the convector fins are one strip listing both regions; pipe, valve body and head or cap are one block per side.", " Vorderansicht: Gitterlippe und Konvektorlamellen sind ein Streifen mit beiden Regionen; Rohr, Ventilkörper und Kopf bzw. Kappe sind je Seite ein Block."),
        "omitted": tx(" Front: fin slits, end seams, pipes (0.1 mm), rear panel, brackets, wall.", " Vorderansicht: Lamellenschlitze, Stirnnähte, Rohre (0,1 mm), Rückplatte, Konsolen, Wand."),
    },
}


def split_regions(asset):
    regions = asset["thermal"]["regions"]
    index = next(i for i, r in enumerate(regions) if r["id"] == "panel-front")
    original = regions[index]
    parts = []
    for rid, en, de, share in BANDS:
        part = copy.deepcopy(original)
        part["id"] = rid
        part["title"] = tx(f"Front water panel, {en}", f"Vordere Wasserplatte, {de}")
        part["areaM2"] = round(original["areaM2"] / 3, 6)
        part["operating"]["heatW"] = round(FRONT_W * share, 4)
        parts.append(part)
    regions[index : index + 1] = parts
    valve = next(r for r in regions if r["id"] == "valve")
    valve["model"] = "operating"
    valve["operating"] = {"key": "heating", "heatW": 3.0}
    for pose in asset["poses"]:
        bindings = pose["thermal"]["regions"]
        first = bindings.pop("panel-front")
        rebuilt = {}
        for key, value in bindings.items():
            rebuilt[key] = value
            if key == "panel-rear":
                for rid, *_ in BANDS:
                    rebuilt[rid] = copy.deepcopy(first)
        pose["thermal"]["regions"] = rebuilt
    names = [rid for rid, *_ in BANDS]
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        shapes = rep["geometry"]["elevation"]
        out = []
        for shape in shapes:
            if shape.get("regions") and "panel-front" in shape["regions"]:
                i = shape["regions"].index("panel-front")
                shape["regions"][i : i + 1] = names
            if shape.get("region") == "panel-front":
                points = shape["points"]
                top = max(p[0] for p in points)
                steps = [top * i / 3 for i in range(4)]
                for (rid, *_), lo, hi in zip(reversed(BANDS), steps, steps[1:]):
                    part = copy.deepcopy(shape)
                    part["id"] = "panel-" + rid.replace("panel-", "")
                    part["region"] = rid
                    part["points"] = [[r5(lo), points[0][1]], [r5(lo), points[1][1]], [r5(hi), points[2][1]], [r5(hi), points[3][1]]]
                    out.append(part)
                continue
            out.append(shape)
        rep["geometry"]["elevation"] = out


def main():
    asset_path = sys.argv[1]
    doc = json.load(open(asset_path, encoding="utf-8"))
    asset = doc["asset"]
    split_regions(asset)
    for key in ("200", "100", "50"):
        rep = asset["representations"][key]
        rep["geometry"]["front"] = front(int(key))
        for field, add in DECISIONS[key].items():
            rep["decisions"][field] = {lang: rep["decisions"][field][lang] + add[lang] for lang in ("en", "de")}
    asset["presentation"]["viewpoints"] = ["elevation", "front"]
    asset["thermal"]["assumptions"].append(tx(
        "The front panel is split into three thermal bands: the water enters at the top (38 % of the front panel heat), flows down and returns cooler at the bottom (29 %), an authored estimate of the 10 K spread; the thermostatic valve and its pipe carry 3 W of the supply water heat; the front elevation (viewpoint front) draws these bands, the top grille strip with the convector fin tips and the valve and lockshield, while the section (viewpoint elevation) draws the same regions in profile.",
        "Die Frontplatte ist in drei thermische Bänder geteilt: das Wasser tritt oben ein (38 % der Frontplattenwärme), strömt nach unten und kehrt unten kühler zurück (29 %), eine Schätzung der 10-K-Spreizung; das Thermostatventil und sein Rohr führen 3 W der Vorlaufwärme; die Vorderansicht (Ansicht front) zeichnet diese Bänder, den Gitterstreifen mit den Konvektorlamellenspitzen sowie Ventil und Rücklaufverschraubung, der Schnitt (Ansicht elevation) dieselben Regionen im Profil."))
    asset["summary"]["en"] = asset["summary"]["en"].replace("shown in vertical section on two wall brackets", "drawn in vertical section (viewpoint elevation) and in front elevation (viewpoint front) on two wall brackets")
    asset["summary"]["de"] = asset["summary"]["de"].replace("im Vertikalschnitt an zwei Wandkonsolen", "im Vertikalschnitt (Ansicht elevation) und in der Vorderansicht (Ansicht front) an zwei Wandkonsolen")
    json.dump(doc, open(asset_path, "w", encoding="utf-8", newline="\n"), ensure_ascii=False, indent=1)


main()
