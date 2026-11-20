"""🪨 Round 3 root fixes of components/solid-brick-wall-1960 outside `volumes`: centre of mass, bearing anchor, the structural assumption that names it, the mortar material, the 1:50 elevation and plan joints of the real NF block bond, and the 2D decisions of every scale.

Usage: python3 r3-solid-brick-wall-1960-roots.py [--write]   (default: candidate under 🗑️generated/r3-solid-brick-wall-1960/🔣️.json)
"""
import json
import os
import sys
import importlib.util

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-solid-brick-wall-1960-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

PLAN_LATERAL = 1.0
CAP = 0.0045


def t(en, de):
    return {"en": en, "de": de}


def stroke(id_, points, material="masonry-mortar"):
    return {"id": id_, "bone": "root", "role": "detail", "z": 2, "filled": False, "tone": 0.3, "region": "wall-field", "material": material, "kind": "poly", "points": [[K.r5(x), K.r5(y)] for x, y in points], "closed": False}


MATERIALS = [
    {"id": "masonry-mortar", "title": t("Lime-cement masonry mortar", "Kalkzement-Mauermörtel"), "emissivity": 0.9, "source": "literature", "solarAbsorptivity": 0.6, "densityKgM3": 1800, "conductivityWmK": 1.0, "specificHeatJKgK": 1000, "reference": "en-iso-10456"},
]


def elevation50(old):
    keep = [s for s in old if s["role"] == "body"]
    out = list(keep)
    for k in range(1, 12):
        y = k * K.COURSE + K.BED / 2
        out.append(stroke(f"joint-{k}", [(K.BRICK[0], y), (K.BRICK[1], y)]))
    for k in range(12):
        lay = K.LAYERS[K.course_kind(k)]
        x = (lay["collar"][0] + lay["collar"][1]) / 2
        v0, v1 = K.brick_v(k)
        out.append(stroke(f"longitudinal-joint-{k + 1}", [(x, v0), (x, min(v1, K.HEIGHT - CAP))]))
    return out


def plan50(old):
    out = [s for s in old if s["role"] == "body"]
    top = K.LAYERS[K.course_kind(11)]
    cx = (top["collar"][0] + top["collar"][1]) / 2
    out.append(stroke("plan-longitudinal-joint", [(cx, K.WALL_W[0] + CAP), (cx, K.WALL_W[1] - CAP)]))
    for i, c in enumerate(K.joints(K.HEADER_JOINT0, K.HEADER_PITCH)):
        out.append(stroke(f"plan-header-joint-{i + 1}", [(top["header"][0], PLAN_LATERAL * c), (top["header"][1], PLAN_LATERAL * c)]))
    for i, c in enumerate(K.joints(top["stretcherJoint"], K.STRETCHER_PITCH)):
        out.append(stroke(f"plan-stretcher-joint-{i + 1}", [(top["stretcher"][0], PLAN_LATERAL * c), (top["stretcher"][1], PLAN_LATERAL * c)]))
    return out


SILHOUETTE = t("The uninsulated 1960 wall in vertical section, a sample 1 m high and 1 m long: room side on the left, weather side on the right, 400 mm in total.", "Die ungedämmte Wand von 1960 im Vertikalschnitt, eine Probe 1 m hoch und 1 m lang: Raumseite links, Wetterseite rechts, insgesamt 400 mm.")
OVERLAYS = t("One thermal region wall-field over all layers; the surface temperatures come from the scenario.", "Eine thermische Region wall-field über alle Schichten; die Oberflächentemperaturen kommen aus dem Szenario.")
HATCHING = t("Cut solid brick at 45 degrees (DIN ISO 128-50); plaster and render stay plain.", "Geschnittener Vollziegel unter 45 Grad (DIN ISO 128-50); Innen- und Außenputz bleiben ohne Schraffur.")

STRIP = t("The 1 m strip is one rigid body on a fixed base under the brick leaf; the self-weight of 712 kg acts at the centre of mass, 201 mm from the room face and at half height. No imposed load is applied.",
          "Der 1-m-Streifen ist ein starrer Körper auf eingespanntem Fußpunkt unter der Ziegelschale; das Eigengewicht von 712 kg wirkt im Schwerpunkt, 201 mm von der Raumseite und auf halber Höhe. Es ist keine Nutzlast angesetzt.")

DECISIONS = {
    "50": {
        "silhouette": SILHOUETTE,
        "retained": t("All three layers at their real thickness: 15 mm lime-cement plaster, the 365 mm solid brick leaf in its block bond (twelve NF courses of 83.3 mm, each with its bed joint and its longitudinal joint, which changes side from course to course) and the 20 mm mineral render. The plan shows the cut top course: headers on the room side, stretchers outside, and their head joints offset by a quarter brick.",
                      "Alle drei Schichten in wirklicher Dicke: 15 mm Kalkzementputz, die 365-mm-Vollziegelschale im Blockverband (zwölf NF-Schichten zu 83,3 mm, jede mit Lagerfuge und Längsfuge, die von Schicht zu Schicht die Seite wechselt) und der 20-mm-Mineralputz. Der Grundriss zeigt die geschnittene oberste Schicht: Binder raumseitig, Läufer außen, ihre Stoßfugen um einen Viertelstein versetzt."),
        "merged": t("None.", "Keine."),
        "omitted": t("The 12 mm mortar bed under the first course lies on the ground line and is drawn by it; the two coats of the plaster and of the render are not told apart.",
                     "Das 12-mm-Mörtelbett unter der ersten Schicht liegt auf der Bodenlinie und wird von ihr gezeichnet; die zwei Lagen des Innen- und des Außenputzes werden nicht unterschieden."),
        "thinElements": t("The 15 mm plaster is 0.3 mm and the 20 mm render 0.4 mm on paper; the 10 mm head and longitudinal joints and the 12.3 mm bed joints are single detail lines on their centre lines.",
                          "Der 15-mm-Innenputz ist auf dem Papier 0,3 mm, der 20-mm-Außenputz 0,4 mm; die 10-mm-Stoß- und Längsfugen und die 12,3-mm-Lagerfugen sind einzelne Detaillinien auf ihren Mittellinien."),
        "hatching": HATCHING,
        "styles": t("Body pen for the layer outlines, detail pen for the bed, longitudinal and head joints of the masonry.",
                    "Körperstift für die Schichtkonturen, Detailstift für die Lager-, Längs- und Stoßfugen des Mauerwerks."),
        "overlays": OVERLAYS,
    },
    "100": {
        "silhouette": SILHOUETTE,
        "retained": t("The masonry leaf and the render strip.", "Die Mauerwerksschale und der Putzstreifen."),
        "merged": t("The 15 mm plaster lies under the body pen of the room face (25 mm at 1:100) and is drawn with the brick as one 380 mm leaf; the thermal region stays one and keeps all layers.",
                    "Der 15-mm-Innenputz liegt unter dem Körperstift der Raumseite (25 mm bei 1:100) und wird mit dem Ziegel als eine 380-mm-Schale gezeichnet; die thermische Region bleibt eine und behält alle Schichten."),
        "omitted": t("The masonry joints (10 to 12 mm, below the 15 mm smallest feature).", "Die Mauerwerksfugen (10 bis 12 mm, unter der kleinsten Einzelheit von 15 mm)."),
        "thinElements": t("The 20 mm render is 0.2 mm on paper and stays a strip of its own: it is the weather skin of the wall and of its own material.",
                          "Der 20-mm-Außenputz ist auf dem Papier 0,2 mm und bleibt ein eigener Streifen: er ist die Wetterhaut der Wand und aus eigenem Material."),
        "hatching": HATCHING,
        "styles": t("Body pen for the layer outlines.", "Körperstift für die Schichtkonturen."),
        "overlays": OVERLAYS,
    },
    "200": {
        "silhouette": SILHOUETTE,
        "retained": t("The wall as one 400 mm field in the brick tone.", "Die Wand als ein 400-mm-Feld im Ziegelton."),
        "merged": t("The 15 mm plaster and the 20 mm render join the brick: both are below the 30 mm smallest feature; the thermal region stays one and keeps all layers.",
                    "Der 15-mm-Innenputz und der 20-mm-Außenputz gehen im Ziegel auf: beide liegen unter der kleinsten Einzelheit von 30 mm; die thermische Region bleibt eine und behält alle Schichten."),
        "omitted": t("Layer boundaries and joints.", "Schichtgrenzen und Fugen."),
        "thinElements": t("None: the field is 2 mm on paper.", "Keine: das Feld ist auf dem Papier 2 mm breit."),
        "hatching": t("None: the hatch lines would close up in a 2 mm field.", "Keine: die Schraffurlinien würden in einem 2-mm-Feld zulaufen."),
        "styles": t("Body pen for the outline.", "Körperstift für die Kontur."),
        "overlays": OVERLAYS,
    },
    "500": {
        "silhouette": t("The 400 mm wall as one cut field, 1 m high and 1 m long.", "Die 400 mm dicke Wand als ein geschnittenes Feld, 1 m hoch und 1 m lang."),
        "retained": t("The outline of the wall.", "Die Kontur der Wand."),
        "merged": t("All three layers form one field: the wall is 0.8 mm on paper and is drawn as one dark poché like every wall of a 1:500 plan; the thermal region keeps all layers.",
                    "Alle drei Schichten bilden ein Feld: die Wand ist auf dem Papier 0,8 mm dick und wird wie jede Wand eines Plans 1:500 als eine dunkle Füllung gezeichnet; die thermische Region behält alle Schichten."),
        "omitted": t("Layer boundaries and joints.", "Schichtgrenzen und Fugen."),
        "thinElements": t("None.", "Keine."),
        "hatching": t("None.", "Keine."),
        "styles": t("Body pen for the outline.", "Körperstift für die Kontur."),
        "overlays": OVERLAYS,
    },
}


def apply(doc):
    a = doc["asset"]
    a["materials"] = [m for m in a["materials"] if m["id"] not in {x["id"] for x in MATERIALS}] + MATERIALS
    mass = a["rig"]["bones"][0]["mass"]
    mass["comX"] = K.r5(K.COM_U)
    mass["comY"] = K.COM_V
    mass["source"] = "literature"
    mass["reference"] = "en-iso-10456"
    base = next(x for x in a["anchors"] if x["id"] == "base")
    base["x"] = K.r5((K.BRICK[0] + K.BRICK[1]) / 2)
    base["footprintLengthM"] = K.r5(K.BRICK[1] - K.BRICK[0])
    base["footprintWidthM"] = 1.0
    a["structure"]["assumptions"][0] = STRIP
    r = a["representations"]
    r["50"]["geometry"]["elevation"] = elevation50(r["50"]["geometry"]["elevation"])
    r["50"]["geometry"]["plan"] = plan50(r["50"]["geometry"]["plan"])
    for s, d in DECISIONS.items():
        r[s]["decisions"] = {k: d[k] for k in r[s]["decisions"]}
        assert set(d) == set(r[s]["decisions"]), (s, set(d) ^ set(r[s]["decisions"]))
    return doc


def main():
    write = "--write" in sys.argv
    doc = K.load(K.CATALOG)
    if write:
        src = K.load(K.SOURCE)
        apply(doc)
        doc["asset"]["volumes"] = src["volumes"]
        text = json.dumps(doc, indent=2, ensure_ascii=False) + "\n"
        with open(K.CATALOG, "w", encoding="utf-8") as f:
            f.write(text)
        print("[DEBUG] wrote", K.CATALOG)
    else:
        os.makedirs(K.GEN, exist_ok=True)
        src = K.load(K.SOURCE)
        apply(doc)
        doc["asset"]["volumes"] = src["volumes"]
        path = os.path.join(K.GEN, "🔣️.json")
        with open(path, "w", encoding="utf-8") as f:
            f.write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
        print(path)


if __name__ == "__main__":
    main()
