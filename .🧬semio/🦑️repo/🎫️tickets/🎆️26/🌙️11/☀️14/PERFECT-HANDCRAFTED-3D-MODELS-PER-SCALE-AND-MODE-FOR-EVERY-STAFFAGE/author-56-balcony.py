import os, json, copy
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())
SL, CO, BM = "slab", "concrete", "break-module"
src = json.load(open(os.path.join(T, "volumes", "components", "cantilever-balcony.json")))["volumes"]


def load(den):
    v = src[str(den)]
    s = Scale(den)
    s.body, s.structure, s.decisions = v["body"], v["structure"], v["decisions"]
    for d in s.body:
        d.pop("sectionFaces", None)
    return s


def flag(s, ids_faces):
    for d in s.body:
        if d["id"] in ids_faces:
            d["sectionFaces"] = ids_faces[d["id"]]


def fl(s, faces):
    s.thermal[-1]["sectionFaces"] = faces


def skins(s, t, edge_h, edge_x, edge_t, edge_flags):
    s.th(box("th-slab-top", SL, CO, 0.7, [1.5, t, 1], [0.75, 0.1 - t / 2, 0]), "slab-top")
    fl(s, ["-v", "-u", "-w", "+w"])
    s.th(box("th-slab-soffit", SL, CO, 0.7, [1.5, t, 1], [0.75, -0.1 + t / 2, 0]), "slab-soffit")
    fl(s, ["+v", "-u", "-w", "+w"])
    s.th(box("th-slab-edge", SL, CO, 0.7, [edge_t, edge_h, 1], [edge_x, 0, 0]), "slab-edge")
    fl(s, edge_flags)


CUT = ["-w", "+w"]
EN_SKIN = " The 1 m strip stands for a balcony that continues sideways and a wall that continues up and down, so the strip ends and the wall cuts are section faces (hatched in the viewer); the skins of the slab overlap in the arris prisms, and the faces against the core or the neighbouring skin are section faces too, so each skin exposes its own outer face once."
DE_SKIN = " Der 1-m-Streifen steht für einen seitlich weiterlaufenden Balkon und eine nach oben und unten weiterlaufende Wand; Streifenenden und Wandschnitte sind daher Schnittflächen (im Betrachter schraffiert); die Plattenhäute überlappen in den Kantenprismen, und die Flächen am Kern oder an der Nachbarhaut sind ebenfalls Schnittflächen, damit jede Haut ihre Außenfläche genau einmal zeigt."
EN_BRK = " The thermal break module is not cut: its outer face lies against the slab in the wall plane and its top under wool and render, so its exposed faces are its 120 mm underside and its two strip ends (0.168 m2, areaM2 corrected from 0.2 to 0.17)."
DE_BRK = " Das Dämmelement ist nicht geschnitten: seine Außenfläche liegt in der Wandebene an der Platte und seine Oberseite unter Wolle und Putz, seine freien Flächen sind die 120 mm breite Unterseite und die zwei Streifenenden (0,168 m2, areaM2 von 0,2 auf 0,17 korrigiert)."


def addthermal(s, en, de):
    for d in s.decisions:
        if d["en"].startswith("Thermal") and "section faces" not in d["en"]:
            d["en"] += en
            d["de"] += de
            return


out = []
s = load(50)
flag(s, {"slab": CUT, "wall-core-upper": ["+v"] + CUT, "wall-insulation": ["+v"] + CUT, "wall-render": ["+v"] + CUT})
skins(s, 0.05, 0.2, 1.475, 0.05, ["-u", "+v", "-v", "-w", "+w"])
s.thermal[0]["sectionFaces"] = ["-v", "-u", "+u", "-w", "+w"]
s.thermal[1]["sectionFaces"] = ["+v", "-u", "+u", "-w", "+w"]
for i, r_ in (("post", "railing-post"), ("handrail", "railing-handrail"), ("mid-rail", "railing-mid-rail"), ("base-plate", "railing-base-plate")):
    s.thof(f"th-railing-{i}", r_, "railing-steel")
s.thof("th-break-core", "wall-core-band", "thermal-break", "break-concrete")
s.thof("th-break-module", "break-module", "thermal-break", "break-body")
s.thof("th-wall-concrete", "wall-core-upper", "wall-face", "wall-concrete")
s.thof("th-wall-wool", "wall-insulation", "wall-face", "wall-wool")
s.thof("th-wall-render", "wall-render", "wall-face", "wall-render")
addthermal(s, EN_SKIN + EN_BRK, DE_SKIN + DE_BRK)
out.append(s)

s = load(100)
flag(s, {"slab": CUT, "wall-core": ["-v", "+v"] + CUT, "wall-insulation-render": ["+v"] + CUT})
skins(s, 0.05, 0.2, 1.475, 0.05, ["-u", "+v", "-v", "-w", "+w"])
s.thermal[0]["sectionFaces"] = ["-v", "-u", "+u", "-w", "+w"]
s.thermal[1]["sectionFaces"] = ["+v", "-u", "+u", "-w", "+w"]
for i, r_ in (("post", "railing-post"), ("handrail", "railing-handrail"), ("mid-rail", "railing-mid-rail")):
    s.thof(f"th-railing-{i}", r_, "railing-steel")
s.th(box("th-break-core", SL, CO, 0.7, [0.18, 0.2, 1], [-0.21, 0, 0]), "thermal-break", "break-concrete")
fl(s, ["-u", "-v", "-w", "+w"])
s.thof("th-break-module", "break-module", "thermal-break", "break-body")
s.th(box("th-wall-concrete", SL, CO, 0.7, [0.18, 1.1, 1], [-0.21, 0.65, 0]), "wall-face", "wall-concrete")
fl(s, ["-u", "+v", "-w", "+w"])
s.th(box("th-wall-wool", SL, "mineral-wool", 0.9, [0.12, 1.1, 1], [-0.06, 0.65, 0]), "wall-face", "wall-wool")
fl(s, ["-u", "+v", "-w", "+w"])
EN100 = " The room-side faces of the wall core and of the break band are section faces (-u), as in the etics walls: a layered region counts one face, its outer one."
DE100 = " Die raumseitigen Flächen von Wandkern und Dämmelementband sind Schnittflächen (-u), wie bei den WDVS-Wänden: eine geschichtete Region zählt eine Fläche, ihre äußere."
addthermal(s, EN_SKIN + EN100 + EN_BRK, DE_SKIN + DE100 + DE_BRK)
out.append(s)

s = load(200)
flag(s, {"slab": CUT, "wall": ["-v", "+v"] + CUT})
skins(s, 0.06, 0.2, 1.47, 0.06, ["-u", "+v", "-v", "-w", "+w"])
s.thermal[0]["sectionFaces"] = ["-v", "-u", "+u", "-w", "+w"]
s.thermal[1]["sectionFaces"] = ["+v", "-u", "+u", "-w", "+w"]
s.thof("th-railing-post", "railing-post", "railing-steel")
s.thof("th-railing-handrail", "railing-handrail", "railing-steel")
s.thof("th-break-module", "break-module", "thermal-break", "break-body")
wall_prof = [[-0.3, -0.1], [-0.12, -0.1], [-0.12, 0.1], [0, 0.1], [0, 1.2], [-0.3, 1.2]]
s.th(ext("th-wall-mass", SL, CO, 0.7, "xy", wall_prof, -0.5, 0.5), "wall-face", "wall-concrete")
fl(s, ["-u", "+v", "-v", "-w", "+w"])
addthermal(s, EN_SKIN + EN_BRK, DE_SKIN + DE_BRK)
out.append(s)

s = load(500)
flag(s, {"balcony-solid": CUT})
s.th(box("th-balcony-top", SL, CO, 0.7, [1.5, 0.075, 1], [0.75, 0.0625, 0]), "slab-top")
fl(s, ["-v", "-u", "-w", "+w"])
s.th(box("th-balcony-soffit", SL, CO, 0.7, [1.5, 0.075, 1], [0.75, -0.0625, 0]), "slab-soffit")
fl(s, ["+v", "-u", "-w", "+w"])
s.th(box("th-balcony-edge", SL, CO, 0.7, [0.075, 0.2, 1], [1.4625, 0, 0]), "slab-edge")
fl(s, ["-u", "+v", "-v", "-w", "+w"])
s.th(box("th-balcony-break", SL, BM, 0.8, [0.12, 0.2, 1], [-0.06, 0, 0]), "thermal-break")
s.th(box("th-balcony-wall", SL, CO, 0.7, [0.3, 1.1, 1], [-0.15, 0.65, 0]), "wall-face")
fl(s, ["-u", "+v", "-v", "-w", "+w"])
s.thof("th-railing-post", "railing-post", "railing-steel")
s.thof("th-railing-handrail", "railing-handrail", "railing-steel")
addthermal(s, EN_SKIN, DE_SKIN)
out.append(s)
save("components", "cantilever-balcony", out)
