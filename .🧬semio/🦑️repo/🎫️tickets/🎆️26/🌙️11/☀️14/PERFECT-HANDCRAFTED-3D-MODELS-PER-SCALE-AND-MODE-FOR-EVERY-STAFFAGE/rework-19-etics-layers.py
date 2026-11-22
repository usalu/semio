"""🧱 Rework 19, ETICS walls (12 and 30 cm): the system is built as it is applied. A 10 mm cementitious adhesive bed between brick and wool (the wall grows by 10 mm), the render as an 8 mm base coat that embeds the glass-fibre mesh plus a 12 mm finish coat, and the four plate anchors as bodies (1:50, 1:100). Each is its own thermal layer; rig, dimensions, mass, anchors of the structure and the 2D drawings move with it.

Usage: python3 rework-19-etics-layers.py <slug> <insulation thickness m> [--roots]
"""
import copy
import json
import sys

sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
import rework_19_lib as L

SLUG, W = sys.argv[1], float(sys.argv[2])
BRICK_OUT = 0.38
GLUE = 0.01
BASE, FINISH = 0.008, 0.012
GLUE_DENSITY, GLUE_LAMBDA = 1500, 0.87
FACES = ["-v", "+v", "-w", "+w"]
X_WOOL = BRICK_OUT + GLUE
X_RENDER = X_WOOL + W


def r6(x):
    return round(x + 0.0, 6)


def moved(x):
    return r6(x + GLUE) if x >= BRICK_OUT - 1e-9 else x


def shift_solid(s):
    if "of" in s:
        return
    if s["kind"] == "revolve":
        s["profile"] = [[moved(a) if a > BRICK_OUT + 1e-9 else a, b] for a, b in s["profile"]]
    elif "transform" in s and s["transform"]["translation"][0] > BRICK_OUT:
        s["transform"]["translation"][0] = r6(s["transform"]["translation"][0] + GLUE)


def layer_box(i, material, tone, x0, t):
    return {"id": i, "kind": "box", "bone": "root", "material": material, "role": "body", "tone": tone, "size": [t, 1.0, 1.0], "transform": {"translation": [r6(x0 + t / 2), 0.5, 0.0]}, "sectionFaces": FACES}


NOTES = [
    {"en": "The system is modelled in the order it is applied: a full bed of cementitious adhesive 10 mm on the brick, the mineral wool boards, the plate anchors through wool and adhesive into the brick, an 8 mm base coat that embeds the glass-fibre mesh and a 12 mm mineral finish coat. Adhesive, base coat and finish are layers of their own in the body and in the thermal list.", "de": "Das System ist in der Reihenfolge seines Auftrags modelliert: ein vollflächiges Bett aus zementärem Kleber 10 mm auf dem Ziegel, die Mineralwolleplatten, die Tellerdübel durch Wolle und Kleber in den Ziegel, ein 8 mm starker Unterputz, der das Glasfasergewebe einbettet, und 12 mm mineralischer Oberputz. Kleber, Unterputz und Oberputz sind eigene Schichten im Körper und in der thermischen Liste."},
]

doc = L.load_source(SLUG)
done = any(x["id"] == "adhesive" for x in doc["volumes"]["50"]["body"])
if not done:
    for scale, v in doc["volumes"].items():
        for key in ("body", "thermal", "structure"):
            for s in v[key]:
                shift_solid(s)
        k = next(n for n, x in enumerate(v["body"]) if x["id"] == "render")
        v["body"][k : k + 1] = [layer_box("render-base-coat", "mineral-render", 0.72, X_RENDER, BASE), layer_box("render-finish", "mineral-render", 0.8, X_RENDER + BASE, FINISH)]
        first_wool = next(n for n, x in enumerate(v["body"]) if x["material"] == "mineral-wool")
        v["body"].insert(first_wool, layer_box("adhesive", "etics-adhesive", 0.6, BRICK_OUT, GLUE))
        k = next(n for n, x in enumerate(v["thermal"]) if x["id"] == "th-face-render")
        faces = v["thermal"][k].get("sectionFaces", [])
        v["thermal"][k : k + 1] = [
            {"id": "th-render-base-coat", "of": "render-base-coat", "region": "wall-field", "layer": "base-coat", "sectionFaces": FACES},
            {"id": "th-face-render", "of": "render-finish", "region": "wall-field", "layer": "render", "sectionFaces": faces},
        ]
        first_wool = next(n for n, x in enumerate(v["thermal"]) if x.get("layer") == "insulation")
        v["thermal"].insert(first_wool, {"id": "th-adhesive", "of": "adhesive", "region": "wall-field", "layer": "adhesive", "sectionFaces": FACES})
        if scale in ("50", "100"):
            for s in v["structure"]:
                if s["id"].startswith("joint-anchor-"):
                    b = {key: copy.deepcopy(val) for key, val in s.items() if key not in ("member", "part")}
                    b["id"] = s["id"].replace("joint-anchor-", "plate-anchor-")
                    v["body"].append(b)
        for s in v["structure"]:
            if s["id"] == "mass-wall-strip":
                s["transform"]["translation"][0] = None
        v["decisions"] = NOTES + [d for d in v["decisions"] if d["en"] != NOTES[0]["en"]]

asset = L.load_asset(SLUG)
bone = asset["rig"]["bones"][0]
old_kg, old_com = bone["mass"]["kg"], bone["mass"]["comX"]
if abs(bone["lengthM"] - (BRICK_OUT + W + 0.02)) < 1e-6:
    glue_kg = GLUE_DENSITY * GLUE
    outer_kg = asset_outer = None
    wool = next(m for m in asset["materials"] if m["id"] == "mineral-wool")["densityKgM3"] * W
    render = next(m for m in asset["materials"] if m["id"] == "mineral-render")["densityKgM3"] * 0.02
    new_kg = round(old_kg + glue_kg, 1)
    new_com = round((old_kg * old_com + (wool + render) * GLUE + glue_kg * (BRICK_OUT + GLUE / 2)) / (old_kg + glue_kg), 5)
else:
    new_kg, new_com = old_kg, old_com
for v in doc["volumes"].values():
    for s in v["structure"]:
        if s["id"] == "mass-wall-strip":
            s["transform"]["translation"][0] = new_com
L.save_source(SLUG, doc)

if "--roots" in sys.argv and abs(bone["lengthM"] - (BRICK_OUT + W + 0.02)) < 1e-6:
    bone["lengthM"] = r6(bone["lengthM"] + GLUE)
    bone["mass"]["kg"], bone["mass"]["comX"] = new_kg, new_com
    asset["dimensions"]["widthM"] = r6(asset["dimensions"]["widthM"] + GLUE)
    asset["dimensions"]["massKg"] = new_kg
    asset["materials"].append({"id": "etics-adhesive", "title": {"en": "Cementitious ETICS adhesive mortar", "de": "Zementärer WDVS-Klebemörtel"}, "emissivity": 0.9, "source": "literature", "solarAbsorptivity": 0.6, "densityKgM3": GLUE_DENSITY, "conductivityWmK": GLUE_LAMBDA, "specificHeatJKgK": 1000, "reference": "en-iso-10456"})
    region = asset["thermal"]["regions"][0]
    layers = region["layers"]
    render = next(x for x in layers if x["id"] == "render")
    k = layers.index(render)
    base = dict(copy.deepcopy(render), id="base-coat", title={"en": "Base coat with glass-fibre mesh 8 mm", "de": "Unterputz mit Glasfasergewebe 8 mm"}, thicknessM=BASE, resistanceM2KW=round(render["resistanceM2KW"] * 0.4, 5), heatCapacityJM2K=round(render["heatCapacityJM2K"] * 0.4))
    render.update(title={"en": "Mineral finish coat 12 mm", "de": "Mineralischer Oberputz 12 mm"}, thicknessM=FINISH, resistanceM2KW=round(render["resistanceM2KW"] - base["resistanceM2KW"], 5), heatCapacityJM2K=render["heatCapacityJM2K"] - base["heatCapacityJM2K"])
    layers.insert(k, base)
    wool_layer = next(x for x in layers if x["id"] == "insulation")
    glue_cap = round(GLUE_DENSITY * 1000 * GLUE)
    layers.insert(layers.index(wool_layer), dict(copy.deepcopy(render), id="adhesive", title={"en": "Cementitious adhesive bed 10 mm", "de": "Zementäres Kleberbett 10 mm"}, material="etics-adhesive", thicknessM=GLUE, resistanceM2KW=round(GLUE / GLUE_LAMBDA, 5), heatCapacityJM2K=glue_cap))
    region["heatCapacityJM2K"] = region["heatCapacityJM2K"] + glue_cap
    for rep in asset["representations"].values():
        for view, shapes in rep.get("geometry", {}).items():
            for s in shapes:
                if s["kind"] == "rect" and view != "front":
                    if s["id"] in ("insulation", "insulation-plan"):
                        s["w"] = r6(s["w"] + GLUE)
                    elif s["x"] >= BRICK_OUT + 1e-9:
                        s["x"] = r6(s["x"] + GLUE)
                    elif s["x"] + s["w"] > BRICK_OUT + 1e-9:
                        s["w"] = r6(s["w"] + GLUE)
                elif s["kind"] == "poly" and view != "front":
                    edge = -1e-9 if s.get("material") == "mineral-wool" else 1e-9
                    s["points"] = [[moved(x) if x > BRICK_OUT + edge else x, y] for x, y in s["points"]]
        text = json.dumps(rep["decisions"], ensure_ascii=False)
        total = round((BRICK_OUT + W + 0.02) * 1000)
        text = text.replace("%d mm" % total, "%d mm" % (total + 10)).replace("All four layers at their real thickness", "The layers at their real thickness").replace("Alle vier Schichten in wirklicher Dicke", "Die Schichten in wirklicher Dicke")
        text = text.replace("; the board adhesive is not a declared layer of this build-up", "; the 10 mm adhesive bed is drawn with the wool, the 8 mm base coat with the render").replace("; der Plattenkleber ist keine erklärte Schicht dieses Aufbaus", "; das 10-mm-Kleberbett ist mit der Wolle gezeichnet, der 8-mm-Unterputz mit dem Außenputz")
        rep["decisions"] = json.loads(text)
    for a in asset["structure"]["assumptions"]:
        for lang in a:
            a[lang] = a[lang].replace("%d kg" % round(old_kg), "%d kg" % round(new_kg)).replace("%d mm" % round(old_com * 1000), "%d mm" % round(new_com * 1000))
    L.write_members(SLUG, {"rig": asset["rig"], "dimensions": asset["dimensions"], "materials": asset["materials"], "thermal": asset["thermal"], "representations": asset["representations"], "structure": asset["structure"]})
print(SLUG, "mass", old_kg, "->", new_kg, "com", old_com, "->", new_com)
