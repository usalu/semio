"""🌬️ Round 5 heat pump outdoor unit: extends the round 3 kit (`r3-heat-pump-outdoor-unit-kit.py`) with the coarse-scale fixes: casing top under the cover at 1:200 and 1:500 (no cover sunk into the casing), the side coil guard of 1:200 as louvre bars over the recessed fin pack (drawn with a 90 mm louvre hatch), and the 1:500 band ending at 1:900 (the office-laser feature of feet and cover holds there). `--apply` writes the catalog root members and the volumes source like round 3."""
import importlib.util
import json
import pathlib
import sys

T = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("hp3", T / "r3-heat-pump-outdoor-unit-kit.py")
k = importlib.util.module_from_spec(spec)
spec.loader.exec_module(k)

Y_CAP = k.LID_Y - 0.005
LOUVRE_PITCH = 0.09


def s200():
    """🧰 1:200, smallest feature 30 mm: fan chamber and compressor column under the cover, the side coil guard as 30 mm louvre bars in front of the recessed fin pack, a dark grille disc with its hub, the cover one slab resting on the casing."""
    t = 0.03
    sc = k.Scale(200)
    m = sc.add
    m(k.box("feet", (-0.17, 0.17), (0, k.Y_TRAY), (k.FEET_Z - 0.09, k.FEET_Z + 0.09), "rubber", 0.2, repeat={"count": 2, "translation": [0, 0, -2 * k.FEET_Z]}), "feet")
    m(k.box("base-tray", (k.XB, k.XF), (k.Y_TRAY, k.Y_BASE), (k.ZR, k.ZL), "galvanised-steel", 0.3), "base-tray")
    m(k.ext("front-panel", "yz", [(k.Y_BASE, k.ZR), (Y_CAP, k.ZR), (Y_CAP, k.ZP0 + 0.005), (k.Y_BASE, k.ZP0 + 0.005)], k.XF - t, k.XF, "powder-coat", 0.55, holes=[k.circle(k.FY, k.FZ, k.HOLE_R, 24)]), "casing")
    m(k.ext("column-shell", "zx", [(k.ZP0 + 0.005, k.XF), (k.ZL, k.XF), (k.ZL, k.XB), (k.ZP0, k.XB), (k.ZP0, k.XB + t), (k.ZL - t, k.XB + t), (k.ZL - t, k.XF - t), (k.ZP0 + 0.005, k.XF - t)], k.Y_BASE, Y_CAP, "powder-coat", 0.5), "compressor-casing")
    m(k.box("side-strip", (0.12, k.XF - t), (k.Y_BASE, Y_CAP), (k.ZR, k.ZR + t), "powder-coat", 0.55), "casing")
    m(k.box("top-panel", (k.XB, k.XF), (Y_CAP - t, Y_CAP), (k.ZR, k.ZL - t), "powder-coat", 0.55), "casing")
    m(k.box("partition", (k.XB + t, k.XF - t), (k.Y_BASE, Y_CAP - t), (k.ZP0, k.ZP0 + t), "galvanised-steel", 0.45), "casing")
    m(k.ext("coil", "zx", [(k.ZP0, k.XB), (k.ZR + t, k.XB), (k.ZR + t, 0.12), (k.ZR + t + 0.04, 0.12), (k.ZR + t + 0.04, k.XB + 0.04), (k.ZP0, k.XB + 0.04)], k.Y_BASE, Y_CAP - t, "aluminium", 0.22, "detail"), "coil-guard")
    m(k.box("side-louvres", (k.XB, 0.12), (k.Y_BASE + 0.045, k.Y_BASE + 0.075), (k.ZR, k.ZR + t), "powder-coat", 0.4, "detail", repeat={"count": 13, "translation": [0, LOUVRE_PITCH, 0]}), "coil-guard")
    m(k.box("guard-frame", (k.XB, 0.12), (k.Y_BASE, k.Y_BASE + 0.03), (k.ZR, k.ZR + t), "powder-coat", 0.4, "detail", repeat={"count": 2, "translation": [0, Y_CAP - k.Y_BASE - 0.06, 0]}), "coil-guard")
    m(k.rev("fan-guard", [(0.185, 0), (0.185, k.HOLE_R), (k.XF, k.HOLE_R), (k.XF, k.RIM_R + 0.005), (0.225, k.RIM_R + 0.005), (0.225, 0)], [1, 0, 0], [0, k.FY, k.FZ], "galvanised-steel", 0.25), "fan-guard")
    m(k.rev("fan-hub", [(0.20, 0), (0.20, 0.075), (0.235, 0.06), (0.235, 0)], [1, 0, 0], [0, k.FY, k.FZ], "galvanised-steel", 0.35, "detail"), "fan-guard")
    m(k.box("lid", (0, k.LID_L), (-0.005, k.LID_TOP), (-k.LID_W, k.LID_W), "powder-coat", 0.35, bone="lid"), "top-lid")
    k.emitters(sc, t)
    for id, x, z in (("post-front-right", (k.XF - t, k.XF), (k.ZR, k.ZR + t)), ("post-front-left", (k.XF - t, k.XF), (k.ZL - t, k.ZL)), ("post-rear-right", (k.XB, k.XB + t), (k.ZR, k.ZR + t)), ("post-rear-left", (k.XB, k.XB + t), (k.ZL - t, k.ZL))):
        sc.member(k.box(id, x, (k.Y_BASE, Y_CAP), z, "galvanised-steel", 0.45), "unit", "member")
    sc.member(k.ext("top-frame", "zx", [(k.ZR, k.XB), (k.ZL, k.XB), (k.ZL, k.XF), (k.ZR, k.XF)], Y_CAP - t, Y_CAP, "galvanised-steel", 0.45, holes=[[(k.ZR + t, k.XB + t), (k.ZL - t, k.XB + t), (k.ZL - t, k.XF - t), (k.ZR + t, k.XF - t)]]), "unit", "member")
    sc.ref("base-pan", "base-tray", "unit", "member")
    sc.ref("feet-blocks", "feet", "unit", "member")
    sc.ref("partition-plate", "partition", "unit", "member")
    sc.ref("lid-member", "lid", "lid", "member")
    sc.member(k.rev("hinge-lid", [(0, 0.015), (0.8, 0.015)], [0, 0, 1], [0.015, 0.0025, -0.4], "powder-coat", 0.3, bone="lid"), "lid", "joint")
    sc.member(k.rev("compressor", [(0.17, 0), (0.17, 0.085), (0.55, 0.085), (0.55, 0)], [0, 1, 0], [-0.02, 0, 0.40], "galvanised-steel", 0.2), "unit", "mass")
    k.supports(sc, t)
    k.masses(sc, 0.04)
    e = lambda s: sc.shape("elevation", s)
    e(k.rect("feet", -0.17, 0, 0.17, 0.09, "body", 1.5, 0.2, region="feet", material="rubber"))
    e(k.rect("casing", k.XB, k.Y_BASE, k.XF, Y_CAP, "body", 2, 0.55, regions=["casing", "compressor-casing", "fan-guard"], material="powder-coat"))
    e(k.rect("base-tray", k.XB, k.Y_TRAY, k.XF, k.Y_BASE, "body", 2.1, 0.3, region="base-tray", material="galvanised-steel"))
    e(k.rect("coil-guard", k.XB, k.Y_BASE, 0.12, Y_CAP, "detail", 2.3, 0.25, region="coil-guard", material="aluminium", hatch="louvres"))
    e(k.lid_shape([(0, k.LID_TOP), (k.LID_L, k.LID_TOP), (k.LID_L, -0.005), (0, -0.005)], 2.5))
    f = lambda s: sc.shape("front", s)
    for id, sign in (("foot-left", 1), ("foot-right", -1)):
        f(k.rect(id, sign * k.FEET_Z - 0.09, 0, sign * k.FEET_Z + 0.09, k.Y_TRAY, "body", 1.5, 0.2, region="feet", material="rubber"))
    f(k.rect("base-tray", k.ZR, k.Y_TRAY, k.ZL, k.Y_BASE, "body", 2.1, 0.3, region="base-tray", material="galvanised-steel"))
    f(k.rect("casing", k.ZR, k.Y_BASE, k.ZL, Y_CAP, "body", 2, 0.55, region="casing", material="powder-coat"))
    f(k.rect("compressor-panel", k.ZP1, k.Y_BASE, k.ZL, Y_CAP, "body", 2.05, 0.5, region="compressor-casing", material="powder-coat"))
    f(k.ellipse("fan-guard", k.FZ, k.FY, k.RIM_R + 0.005, "body", 2.2, 0.25, region="fan-guard", material="galvanised-steel"))
    f(k.ellipse("fan-hub", k.FZ, k.FY, 0.075, "detail", 2.3, 0.35, material="galvanised-steel"))
    f(k.lid_shape([(0, k.LID_TOP), (k.LID_L, k.LID_TOP), (k.LID_L, -0.005), (0, -0.005)], 2.5))
    sc.shape("plan", k.rect("casing", k.XB, k.ZR, k.XF, k.ZL, "body", 2, 0.55, region="casing", material="powder-coat"))
    sc.shape("plan", k.lid_shape([(0, k.LID_TOP), (k.LID_L, k.LID_TOP), (k.LID_L, -0.005), (0, -0.005)], 2.5))
    return sc


def s500():
    """🧱 1:500, smallest feature 75 mm: casing and compressor column blocks under the cover slab, the merged base pan, the L coil as one dark slab, a dark fan disc and two feet."""
    t = 0.075
    sc = k.Scale(500)
    m = sc.add
    m(k.box("feet", (-0.17, 0.17), (0, k.Y_TRAY), (k.FEET_Z - 0.09, k.FEET_Z + 0.09), "rubber", 0.2, repeat={"count": 2, "translation": [0, 0, -2 * k.FEET_Z]}), "feet")
    m(k.box("base-tray", (k.XB, k.XF), (k.Y_TRAY, k.Y_TRAY + t), (k.ZR, k.ZL), "galvanised-steel", 0.3), "base-tray")
    m(k.ext("front-panel", "yz", [(k.Y_TRAY + t, k.ZR), (Y_CAP, k.ZR), (Y_CAP, k.ZL), (k.Y_TRAY + t, k.ZL)], k.XF - t, k.XF, "powder-coat", 0.55, holes=[k.circle(k.FY, k.FZ, k.HOLE_R, 16)]), "casing")
    m(k.ext("column-shell", "zx", [(k.ZP1, k.XB), (k.ZL, k.XB), (k.ZL, k.XF - t), (k.ZL - t, k.XF - t), (k.ZL - t, k.XB + t), (k.ZP1, k.XB + t)], k.Y_TRAY + t, Y_CAP, "powder-coat", 0.5), "compressor-casing")
    m(k.box("top-panel", (k.XB + t, k.XF - t), (Y_CAP - t, Y_CAP), (k.ZR + t, k.ZL - t), "powder-coat", 0.55), "casing")
    m(k.ext("coil", "zx", [(k.ZP1, k.XB), (k.ZR, k.XB), (k.ZR, k.XF - t), (k.ZR + t, k.XF - t), (k.ZR + t, k.XB + t), (k.ZP1, k.XB + t)], k.Y_TRAY + t, Y_CAP, "aluminium", 0.22, "detail"), "coil-guard")
    m(k.rev("fan-disc", [(k.XF - t, 0), (k.XF - t, k.HOLE_R), (0.235, k.HOLE_R), (0.235, 0)], [1, 0, 0], [0, k.FY, k.FZ], "galvanised-steel", 0.25), "fan-guard")
    m(k.box("lid", (0, k.LID_L), (-0.005, k.LID_TOP), (-k.LID_W, k.LID_W), "powder-coat", 0.35, bone="lid"), "top-lid")
    k.emitters(sc, t)
    for id, x, z in (("post-front-right", (k.XF - t, k.XF), (k.ZR, k.ZR + t)), ("post-front-left", (k.XF - t, k.XF), (k.ZL - t, k.ZL)), ("post-rear-right", (k.XB, k.XB + t), (k.ZR, k.ZR + t)), ("post-rear-left", (k.XB, k.XB + t), (k.ZL - t, k.ZL))):
        sc.member(k.box(id, x, (k.Y_TRAY + t, Y_CAP - t), z, "galvanised-steel", 0.45), "unit", "member")
    for id, y in (("base-frame", (k.Y_TRAY, k.Y_TRAY + t)), ("top-frame", (Y_CAP - t, Y_CAP))):
        sc.member(k.ext(id, "zx", [(k.ZR, k.XB), (k.ZL, k.XB), (k.ZL, k.XF), (k.ZR, k.XF)], y[0], y[1], "galvanised-steel", 0.45, holes=[[(k.ZR + t, k.XB + t), (k.ZL - t, k.XB + t), (k.ZL - t, k.XF - t), (k.ZR + t, k.XF - t)]]), "unit", "member")
    sc.ref("feet-blocks", "feet", "unit", "member")
    sc.ref("lid-member", "lid", "lid", "member")
    sc.member(k.rev("hinge-lid", [(0, 0.0375), (0.8, 0.0375)], [0, 0, 1], [0.0375, 0.0175, -0.4], "powder-coat", 0.3, bone="lid"), "lid", "joint")
    k.supports(sc, t)
    k.masses(sc, 0.075)
    e = lambda s: sc.shape("elevation", s)
    e(k.rect("feet", -0.17, 0, 0.17, 0.09, "body", 1.5, 0.2, region="feet", material="rubber"))
    e(k.rect("casing", k.XB, k.Y_TRAY, k.XF, Y_CAP, "body", 2, 0.55, regions=["casing", "compressor-casing", "fan-guard", "base-tray", "coil-guard"], material="powder-coat"))
    e(k.lid_shape([(0, k.LID_TOP), (k.LID_L, k.LID_TOP), (k.LID_L, -0.005), (0, -0.005)], 2.5))
    f = lambda s: sc.shape("front", s)
    for id, sign in (("foot-left", 1), ("foot-right", -1)):
        f(k.rect(id, sign * k.FEET_Z - 0.09, 0, sign * k.FEET_Z + 0.09, k.Y_TRAY, "body", 1.5, 0.2, region="feet", material="rubber"))
    f(k.rect("casing", k.ZR, k.Y_TRAY, k.ZL, Y_CAP, "body", 2, 0.55, regions=["casing", "compressor-casing", "base-tray"], material="powder-coat"))
    f(k.ellipse("fan-disc", k.FZ, k.FY, k.RIM_R, "body", 2.2, 0.25, region="fan-guard", material="galvanised-steel"))
    f(k.lid_shape([(0, k.LID_TOP), (k.LID_L, k.LID_TOP), (k.LID_L, -0.005), (0, -0.005)], 2.5))
    sc.shape("plan", k.rect("casing", k.XB, k.ZR, k.XF, k.ZL, "body", 2, 0.55, region="casing", material="powder-coat"))
    sc.shape("plan", k.lid_shape([(0, k.LID_TOP), (k.LID_L, k.LID_TOP), (k.LID_L, -0.005), (0, -0.005)], 2.5))
    return sc


DECISION_200 = k.txt(
    "Authored for 30 mm: the casing becomes the fan chamber block and the compressor column block over the base pan on two feet, both ending under the cover; the fin pack is a 40 mm dark slab 30 mm behind the side face, seen between thirteen 30 mm louvre bars at 90 mm pitch in a top and bottom frame, and as a dark face on the back; grille, blades and bell mouth are one dark disc 10 mm proud with the hub on it; the cover is one 45 mm slab resting on the casing.",
    "Für 30 mm erstellt: das Gehäuse wird zum Block der Lüfterkammer und zum Block der Verdichtersäule über der Bodenwanne auf zwei Füßen, beide enden unter der Haube; das Lamellenpaket ist eine dunkle 40-mm-Platte 30 mm hinter der Seitenfläche, zwischen dreizehn 30-mm-Lamellen im Raster von 90 mm in einem oberen und unteren Rahmen sichtbar, und hinten eine dunkle Fläche; Gitter, Flügel und Einströmdüse sind eine dunkle Scheibe 10 mm vor der Front mit der Nabe; die Haube ist eine 45-mm-Platte, die auf dem Gehäuse aufliegt.",
)
HATCHING_200 = k.txt(
    "Coil guard of the side elevation: one line per louvre bar, 90 mm apart (0.45 mm on paper), as the 1:200 model has them.",
    "Tauscherschutz der Seitenansicht: eine Linie je Lamelle, 90 mm Abstand (0,45 mm auf dem Papier), wie das Modell 1:200 sie hat.",
)
RETAINED_200 = k.txt(
    "Side: feet, base pan, casing, the louvred coil guard of the right side, cover. Front: feet, base pan, casing, compressor column panel, the dark fan disc with its hub, cover. Plan: casing under the cover.",
    "Seite: Füße, Bodenwanne, Gehäuse, der Lamellenschutz des Tauschers an der rechten Seite, Haube. Front: Füße, Bodenwanne, Gehäuse, Blech der Verdichtersäule, die dunkle Lüfterscheibe mit Nabe, Haube. Draufsicht: Gehäuse unter der Haube.",
)
RANGE_500 = 900
AREAS_C31 = {"casing": 3.75, "base-tray": 1.0}


def build():
    """🏗️ Round 3 candidate with the round 5 coarse scales, decisions, hatch and 1:500 band."""
    k.s200, k.s500 = s200, s500
    k.AREAS.update(AREAS_C31)
    k.TEXTS["volumes"]["200"] = [DECISION_200] + [d for d in k.TEXTS["volumes"]["200"][1:]]
    doc, volumes = k.build()
    reps = doc["asset"]["representations"]
    reps["200"]["hatches"] = [{"id": "louvres", "title": k.txt("Coil guard louvres", "Lamellen des Tauscherschutzes"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.45, "penMm": 0.1}]
    reps["200"]["decisions"]["hatching"] = HATCHING_200
    reps["200"]["decisions"]["retained"] = RETAINED_200
    reps["500"]["validRange"]["maxDenominator"] = RANGE_500
    return doc, volumes


if __name__ == "__main__":
    doc, volumes = build()
    k.OUT.mkdir(parents=True, exist_ok=True)
    (k.OUT / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
    print("candidate", k.OUT / "🔣️.json")
    if "--apply" in sys.argv:
        k.apply(doc, volumes)
        print("applied")
