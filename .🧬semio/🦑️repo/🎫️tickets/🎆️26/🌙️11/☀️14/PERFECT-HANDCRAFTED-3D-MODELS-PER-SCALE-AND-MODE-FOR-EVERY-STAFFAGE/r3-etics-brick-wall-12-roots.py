"""🪨 Round 3 root fixes of components/etics-brick-wall-12 outside `volumes`: centre of mass, bearing anchor, the structural assumption that names it, mortar and anchor materials, the 1:50 elevation and plan joints of the real NF block bond and the board joint, and the 2D decisions of every scale.

Usage: python3 r3-etics-brick-wall-12-roots.py [--write]   (default: candidate under 🗑️generated/r3-etics-brick-wall-12/🔣️.json)
"""
import json
import os
import sys
import importlib.util

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-etics-brick-wall-12-kit.py"))
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
    {"id": "etics-anchor", "title": t("ETICS plate anchor, galvanised steel pin", "WDVS-Tellerdübel, verzinkter Stahlnagel"), "emissivity": 0.3, "source": "literature", "solarAbsorptivity": 0.6, "densityKgM3": 7800, "conductivityWmK": 50, "specificHeatJKgK": 450, "reference": "en-iso-10456"},
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
    out.append(stroke("board-joint", [(K.WOOL[0], K.BOARD_JOINT_V), (K.WOOL[1], K.BOARD_JOINT_V)], "mineral-wool"))
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
    out.append(stroke("plan-board-joint", [(K.WOOL[0], PLAN_LATERAL * K.BOARD_JOINT_HIGH_W), (K.WOOL[1], PLAN_LATERAL * K.BOARD_JOINT_HIGH_W)], "mineral-wool"))
    return out


SILHOUETTE = t("The 12 cm ETICS wall in vertical section, a sample 1 m high and 1 m long: room side on the left, outside on the right, 520 mm in total.", "Die Wand mit 12 cm WDVS im Vertikalschnitt, eine Probe 1 m hoch und 1 m lang: Raumseite links, außen rechts, insgesamt 520 mm.")
OVERLAYS = t("One thermal region wall-field over all layers; the surface temperatures come from the scenario.", "Eine thermische Region wall-field über alle Schichten; die Oberflächentemperaturen kommen aus dem Szenario.")
HATCHING = t("Brick at 45 degrees, mineral wool as a cross hatch (DIN ISO 128-50).", "Ziegel unter 45 Grad, Mineralwolle als Kreuzschraffur (DIN ISO 128-50).")

STRIP = t("The 1 m strip is one rigid body on a fixed base under the brick leaf; the self-weight of 719 kg acts at the centre of mass, 209 mm from the room face and so inside the leaf. No imposed load is applied.",
          "Der 1-m-Streifen ist ein starrer Körper auf eingespanntem Fußpunkt unter der Ziegelschale; das Eigengewicht von 719 kg wirkt im Schwerpunkt, 209 mm von der Raumseite und damit in der Schale. Es ist keine Nutzlast angesetzt.")

DECISIONS = {
    "50": {
        "silhouette": SILHOUETTE,
        "retained": t("All four layers at their real thickness: 15 mm lime plaster, the 365 mm solid brick leaf in its block bond (twelve NF courses of 83.3 mm, each with its bed joint and its longitudinal joint, which changes side from course to course), the 120 mm mineral wool boards with their horizontal board joint at 0.40 m and the 20 mm render. The plan shows the cut top course: headers on the room side, stretchers outside, their head joints and the vertical board joint of the wool.",
                      "Alle vier Schichten in wirklicher Dicke: 15 mm Kalkputz, die 365-mm-Vollziegelschale im Blockverband (zwölf NF-Schichten zu 83,3 mm, jede mit Lagerfuge und Längsfuge, die von Schicht zu Schicht die Seite wechselt), die 120-mm-Mineralwolleplatten mit ihrer waagrechten Plattenfuge bei 0,40 m und der 20-mm-Außenputz. Der Grundriss zeigt die geschnittene oberste Schicht: Binder raumseitig, Läufer außen, ihre Stoßfugen und die senkrechte Plattenfuge der Wolle."),
        "merged": t("None.", "Keine."),
        "omitted": t("The 12 mm mortar bed under the first course lies on the ground line and is drawn by it; the plate anchors and the reinforcing mesh lie inside the wool and the render and are not cut by this section; the board adhesive is not a declared layer of this build-up.",
                     "Das 12-mm-Mörtelbett unter der ersten Schicht liegt auf der Bodenlinie und wird von ihr gezeichnet; die Tellerdübel und das Armierungsgewebe liegen in Wolle und Putz und werden von diesem Schnitt nicht getroffen; der Plattenkleber ist keine erklärte Schicht dieses Aufbaus."),
        "thinElements": t("The 15 mm plaster is 0.3 mm and the 20 mm render 0.4 mm on paper; the 10 mm head and longitudinal joints and the 12.3 mm bed joints are single detail lines on their centre lines.",
                          "Der 15-mm-Innenputz ist auf dem Papier 0,3 mm, der 20-mm-Außenputz 0,4 mm; die 10-mm-Stoß- und Längsfugen und die 12,3-mm-Lagerfugen sind einzelne Detaillinien auf ihren Mittellinien."),
        "hatching": HATCHING,
        "styles": t("Body pen for the layer outlines, detail pen for the bed, longitudinal and head joints of the masonry and the joints of the wool boards.",
                    "Körperstift für die Schichtkonturen, Detailstift für die Lager-, Längs- und Stoßfugen des Mauerwerks und die Fugen der Wolleplatten."),
        "overlays": OVERLAYS,
    },
    "100": {
        "silhouette": SILHOUETTE,
        "retained": t("The masonry leaf, the 120 mm insulation and the render strip.", "Die Mauerwerksschale, die 120-mm-Dämmung und der Putzstreifen."),
        "merged": t("The 15 mm plaster lies under the body pen of the room face (25 mm at 1:100) and is drawn with the brick as one 380 mm leaf; the thermal region stays one and keeps all layers.",
                    "Der 15-mm-Innenputz liegt unter dem Körperstift der Raumseite (25 mm bei 1:100) und wird mit dem Ziegel als eine 380-mm-Schale gezeichnet; die thermische Region bleibt eine und behält alle Schichten."),
        "omitted": t("The masonry and board joints (10 to 12 mm, below the 15 mm smallest feature), the anchors, the mesh and the adhesive.",
                     "Die Mauerwerks- und Plattenfugen (10 bis 12 mm, unter der kleinsten Einzelheit von 15 mm), die Dübel, das Gewebe und der Kleber."),
        "thinElements": t("The 20 mm render is 0.2 mm on paper and stays a strip of its own: it is the finishing coat of the ETICS and of its own material.",
                          "Der 20-mm-Außenputz ist auf dem Papier 0,2 mm und bleibt ein eigener Streifen: er ist die Deckschicht des WDVS und aus eigenem Material."),
        "hatching": HATCHING,
        "styles": t("Body pen for the layer outlines.", "Körperstift für die Schichtkonturen."),
        "overlays": OVERLAYS,
    },
    "200": {
        "silhouette": SILHOUETTE,
        "retained": t("The masonry leaf (380 mm with its plaster) and the ETICS (140 mm of wool and render).", "Die Mauerwerksschale (380 mm mit ihrem Putz) und das WDVS (140 mm aus Wolle und Putz)."),
        "merged": t("The 20 mm render joins the wool and the 15 mm plaster joins the brick: both are below the 30 mm smallest feature; the thermal region stays one and keeps all layers.",
                    "Der 20-mm-Außenputz geht in der Wolle auf und der 15-mm-Innenputz im Ziegel: beide liegen unter der kleinsten Einzelheit von 30 mm; die thermische Region bleibt eine und behält alle Schichten."),
        "omitted": t("Joints, anchors, mesh and adhesive.", "Fugen, Dübel, Gewebe und Kleber."),
        "thinElements": t("None: the narrower field, the ETICS, is 0.7 mm on paper.", "Keine: das schmalere Feld, das WDVS, ist auf dem Papier 0,7 mm breit."),
        "hatching": t("None.", "Keine."),
        "styles": t("Body pen for the outlines of the two fields.", "Körperstift für die Konturen der zwei Felder."),
        "overlays": OVERLAYS,
    },
    "500": {
        "silhouette": t("The 520 mm wall as one cut field, 1 m high and 1 m long.", "Die 520 mm dicke Wand als ein geschnittenes Feld, 1 m hoch und 1 m lang."),
        "retained": t("The outline of the wall.", "Die Kontur der Wand."),
        "merged": t("All four layers form one field: the wall is 1.04 mm on paper and is drawn as one poché like every wall of a 1:500 plan; the thermal region keeps all layers.",
                    "Alle vier Schichten bilden ein Feld: die Wand ist auf dem Papier 1,04 mm dick und wird wie jede Wand eines Plans 1:500 als eine Füllung gezeichnet; die thermische Region behält alle Schichten."),
        "omitted": t("Layer boundaries, joints, anchors, mesh and adhesive.", "Schichtgrenzen, Fugen, Dübel, Gewebe und Kleber."),
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
