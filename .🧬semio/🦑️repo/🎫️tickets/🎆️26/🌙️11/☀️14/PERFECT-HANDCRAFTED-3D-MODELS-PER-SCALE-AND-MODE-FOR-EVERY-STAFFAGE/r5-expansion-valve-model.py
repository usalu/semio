"""🧯 Round 5, machinery/expansion-valve: rebuilds 1:100, 1:200 and 1:500 by hand (1:50 kept), redraws the 1:100 plan coil as a ring and sets the inlet area from the 1:50 model.

Writes the volumes source `volumes/machinery/expansion-valve.json` and the candidate `🗑️generated/r5-expansion-valve/🔣️.json`; with `--apply` it writes the catalog roots (thermal inlet area, 1:100 plan coil) after re-reading the file.
Usage: python3 r5-expansion-valve-model.py [--apply]
See: r3-machinery-expansion-valve.md (reference table), r5-brief.md.
"""
import copy
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/expansion-valve/🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "machinery", "expansion-valve.json")
OUT = os.path.join(HERE, "🗑️generated", "r5-expansion-valve", "🔣️.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])

COPPER, BRASS, STEEL = "copper", "brass", "stainless-steel"
UP, AHEAD = [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]
INLET_AREA = 0.00215


def mm(points):
    """📏 Millimetre pairs to metres."""
    return [[round(a / 1000, 6), round(b / 1000, 6)] for a, b in points]


def rev(sid, bone, material, tone, profile, axis=AHEAD, origin=(0.0, 0.0, 0.0), **extra):
    """🌀 A revolve of a millimetre profile [axial, radial] about a bone-local axis."""
    return {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": tone, "profile": mm(profile), "axis": axis, "origin": [o / 1000 for o in origin], **extra}


def ball(sid, bone, material, centre, r, member, part, **extra):
    """⚪ A small sphere for joints, supports and masses (mm)."""
    return {"id": sid, "kind": "ellipsoid", "bone": bone, "material": material, "role": "body", "tone": 0.3, "radii": [r / 1000] * 3, "transform": {"translation": [c / 1000 for c in centre]}, "member": member, "part": part, **extra}


def lead(sid, points):
    """🧵 The 1.6 mm capillary as a slender loft through head-local points (mm)."""
    return {"id": sid, "kind": "loft", "bone": "head", "material": STEEL, "role": "body", "tone": 0.1, "sections": [{"at": [round(c / 1000, 6) for c in p], "rx": 0.0008, "ry": 0.0008} for p in points], "capStart": True, "capEnd": True, "slender": True}


LEAD = [(20.0, 0.0, 29.0), (19.027, 10.452, 28.86), (16.386, 18.216, 28.48), (12.494, 23.904, 27.92), (7.768, 28.128, 27.24), (2.625, 31.5, 26.5), (-2.518, 34.632, 25.76), (-7.244, 38.136, 25.08), (-11.136, 42.624, 24.52), (-13.777, 48.708, 24.14), (-14.75, 57.0, 24.0)]
RISER = [(17.2, 0.0, 0.0), (18.2, 0.0, 6.0), (19.4, 0.0, 12.0), (20.0, 0.0, 18.0), (20.0, 0.0, 24.0)]
INLET_ORIGIN = (0.0, -93.0, 0.0)
SUCTION_ORIGIN = (0.0, 15.25, 0.0)


def refs(body, regions, faces):
    """🔗 Thermal references to the body solids, one region each."""
    return [{"id": "th-" + b["id"], "of": b["id"], "region": regions[b["id"]], **({"sectionFaces": faces[b["id"]]} if b["id"] in faces else {})} for b in body]


REGIONS = {"inlet": "inlet", "body": "body", "outlet": "outlet", "head": "head", "capillary-coil": "capillary", "capillary": "capillary", "stem": "body", "bulb": "bulb", "suction-line": "suction-line"}
FACES = {"inlet": ["-v"], "outlet": ["+u"], "suction-line": ["-u", "+u"]}


def supports(r):
    """🟢 Pin and rollers in the three cut pipe ends."""
    return [
        ball("support-liquid-line", "body", COPPER, (0.0, -71.0 + r, 0.0), r, "body", "support", anchor="liquid-line"),
        ball("support-outlet-pipe", "body", COPPER, (82.0 - r, 0.0, 0.0), r, "body", "support", anchor="outlet-pipe"),
        ball("support-suction-line", "bulb", COPPER, (57.0 - r, 15.25, 0.0), r, "bulb", "support", anchor="suction-line"),
    ]


def body100():
    """🔩 1:100: the 1:50 parts merged to 15 mm masses; the coil is a 60 mm ring band carried on the capillary, so the element shows through it."""
    return [
        rev("inlet", "body", COPPER, 0.4, [(22, 0), (22, 7.5), (50, 7.5), (53, 11), (68, 11), (70, 9.5), (70, 0)], UP, INLET_ORIGIN, sectionFaces=["-v"]),
        rev("body", "body", BRASS, 0.5, [(66, 0), (66, 11.5), (106, 11.5), (108.5, 8), (108.5, 0)], UP, INLET_ORIGIN),
        rev("outlet", "body", COPPER, 0.4, [(8, 0), (8, 7.5), (82, 7.5), (82, 0)], sectionFaces=["+u"]),
        rev("head", "head", STEEL, 0.55, [(-1.5, 0), (-1.5, 8), (2.5, 8), (6, 21), (10.5, 21), (13, 9), (17.5, 5), (17.5, 0)]),
        rev("capillary-coil", "head", STEEL, 0.3, [(17, 15), (17, 29), (18, 30), (34, 30), (35, 29), (35, 15), (17, 15)]),
        lead("capillary", RISER + LEAD),
        rev("stem", "stem", BRASS, 0.3, [(-6, 0), (-6, 8.5), (20.5, 8.5), (22, 7), (22, 0)]),
        rev("bulb", "bulb", STEEL, 0.5, [(0, 0), (0, 5), (3, 7.5), (47, 7.5), (50, 5), (50, 0)]),
        rev("suction-line", "bulb", COPPER, 0.4, [(-7, 0), (-7, 8), (57, 8), (57, 0)], origin=SUCTION_ORIGIN, sectionFaces=["-u", "+u"]),
    ]


def structure100():
    """🏗️ 1:100: column, the pipes and housings that are their own members, the branch joint, three supports, one mass."""
    return [
        rev("member-column", "body", BRASS, 0.5, [(22, 0), (22, 7.5), (108, 7.5), (108, 0)], UP, INLET_ORIGIN, member="body", part="member"),
        {"id": "member-outlet", "of": "outlet", "member": "body", "part": "member"},
        {"id": "member-element", "of": "head", "member": "head", "part": "member"},
        {"id": "member-spindle-housing", "of": "stem", "member": "stem", "part": "member"},
        {"id": "member-suction-line", "of": "suction-line", "member": "bulb", "part": "member"},
        ball("joint-body", "body", BRASS, (0, 0, 0), 9.0, "body", "joint"),
        *supports(7.5),
        ball("mass-valve", "body", BRASS, (0, -3, 0), 7.5, "body", "mass"),
    ]


def body200():
    """🔩 1:200 (30 mm feature, model only): flare nut merged into a chamfered collar of the straight liquid line, element lens with the coil as one closed drum seated on its cap, a plain spindle stub, bulb as a round-ended rod on the suction line."""
    return [
        rev("inlet", "body", COPPER, 0.4, [(22, 0), (22, 7.5), (48, 7.5), (52, 11), (66, 11), (70, 9), (70, 0)], UP, INLET_ORIGIN, sectionFaces=["-v"]),
        rev("body", "body", BRASS, 0.5, [(66, 0), (66, 11.5), (105, 11.5), (108.5, 7.5), (108.5, 0)], UP, INLET_ORIGIN),
        rev("outlet", "body", COPPER, 0.4, [(8, 0), (8, 7.5), (82, 7.5), (82, 0)], sectionFaces=["+u"]),
        rev("head", "head", STEEL, 0.55, [(-1.5, 0), (-1.5, 8), (5, 21), (11, 21), (17.5, 5), (17.5, 0)]),
        rev("capillary-coil", "head", STEEL, 0.3, [(15, 0), (15, 29), (16, 30), (34, 30), (35, 29), (35, 0)]),
        lead("capillary", [LEAD[0], LEAD[3], LEAD[6], LEAD[9], LEAD[10]]),
        rev("stem", "stem", BRASS, 0.3, [(-6, 0), (-6, 8.5), (22, 8.5), (22, 0)]),
        rev("bulb", "bulb", STEEL, 0.5, [(0, 0), (0, 4), (3.5, 7.5), (46.5, 7.5), (50, 4), (50, 0)]),
        rev("suction-line", "bulb", COPPER, 0.4, [(-7, 0), (-7, 8), (57, 8), (57, 0)], origin=SUCTION_ORIGIN, sectionFaces=["-u", "+u"]),
    ]


def structure200():
    """🏗️ 1:200: one column, the members that are their own parts, the branch joint, three supports, one mass."""
    return [
        rev("member-column", "body", BRASS, 0.5, [(22, 0), (22, 7.5), (108, 7.5), (108, 0)], UP, INLET_ORIGIN, member="body", part="member"),
        {"id": "member-outlet", "of": "outlet", "member": "body", "part": "member"},
        {"id": "member-element", "of": "head", "member": "head", "part": "member"},
        {"id": "member-spindle-housing", "of": "stem", "member": "stem", "part": "member"},
        {"id": "member-suction-line", "of": "suction-line", "member": "bulb", "part": "member"},
        ball("joint-body", "body", BRASS, (0, 0, 0), 10.0, "body", "joint"),
        *supports(7.5),
        ball("mass-valve", "body", BRASS, (0, -3, 0), 8.0, "body", "mass"),
    ]


def body500():
    """🔩 1:500 (75 mm feature, model only): the valve as a tee of straight pipe, barrel and outlet; element and coil one turned head (lens under a drum); spindle stub kept for the angle-valve silhouette; bulb and suction line plain rods."""
    return [
        rev("inlet", "body", COPPER, 0.4, [(22, 0), (22, 8), (68, 8), (68, 0)], UP, INLET_ORIGIN, sectionFaces=["-v"]),
        rev("body", "body", BRASS, 0.5, [(66, 0), (66, 11.5), (108.5, 11.5), (108.5, 0)], UP, INLET_ORIGIN),
        rev("outlet", "body", COPPER, 0.4, [(8, 0), (8, 7.5), (82, 7.5), (82, 0)], sectionFaces=["+u"]),
        rev("head", "head", STEEL, 0.55, [(-1.5, 0), (-1.5, 8), (5, 21), (11, 21), (15, 12), (15, 30), (35, 30), (35, 0)]),
        lead("capillary", [LEAD[0], LEAD[6], LEAD[10]]),
        rev("stem", "stem", BRASS, 0.3, [(-6, 0), (-6, 8.5), (22, 8.5), (22, 0)]),
        rev("bulb", "bulb", STEEL, 0.5, [(0, 0), (0, 7.5), (50, 7.5), (50, 0)]),
        rev("suction-line", "bulb", COPPER, 0.4, [(-7, 0), (-7, 8), (57, 8), (57, 0)], origin=SUCTION_ORIGIN, sectionFaces=["-u", "+u"]),
    ]


def structure500():
    """🏗️ 1:500: column, outlet, element, spindle and suction members, three supports, one mass."""
    return [
        rev("member-column", "body", BRASS, 0.5, [(22, 0), (22, 8), (66, 8), (66, 11.5), (108, 11.5), (108, 0)], UP, INLET_ORIGIN, member="body", part="member"),
        {"id": "member-outlet", "of": "outlet", "member": "body", "part": "member"},
        {"id": "member-element", "of": "head", "member": "head", "part": "member"},
        {"id": "member-spindle-housing", "of": "stem", "member": "stem", "part": "member"},
        {"id": "member-suction-line", "of": "suction-line", "member": "bulb", "part": "member"},
        *supports(8.0),
        ball("mass-valve", "body", BRASS, (0, -3, 0), 10.0, "body", "mass"),
    ]


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


DECISIONS100_COIL = text(
    "Body: the seven capillary turns are one ring band of 60 mm by 18 mm with a 30 mm opening, so the element and its nipple show through it as on the real valve; the ring hangs on the capillary, which rises from the nipple into the band (a slender 1.6 mm tube, as in 1:50) and leaves it as the lead to the bulb. The plan draws the same ring.",
    "Körper: Die sieben Kapillarwindungen sind ein Ringband von 60 mm mal 18 mm mit 30 mm Öffnung, sodass Element und Nippel wie am echten Ventil hindurch sichtbar sind; der Ring hängt an der Kapillare, die vom Nippel in das Band steigt (ein schlankes 1,6-mm-Rohr wie bei 1:50) und es als Zuleitung zum Fühler verlässt. Der Grundriss zeichnet denselben Ring.")

DECISIONS200 = [
    text("Body, 1:200 (model only, 30 mm feature; the drawing is unavailable at this scale): the valve keeps the angle-valve silhouette of the Danfoss T 2 at its real size: the straight 3/8 in liquid line with the flare nut merged into one chamfered collar, the 23 mm forged barrel, the 15 mm outlet bar ending 54 mm beside the axis, the spindle stub inclined 30 degrees downwards, the 42 mm element as a lens with the coiled capillary as one closed 60 mm drum seated on its cap, the 1.6 mm lead in five stations, the 14.5 by 50 mm bulb as a round-ended rod lying on the 16 mm suction line.",
         "Körper, 1:200 (nur Modell, 30-mm-Merkmal; die Zeichnung ist bei diesem Maßstab nicht verfügbar): Das Ventil behält die Eckventil-Silhouette des Danfoss T 2 in echter Größe: die gerade 3/8-Zoll-Flüssigkeitsleitung mit der Bördelmutter als ein gefaster Bund, der 23-mm-Schmiedezylinder, der 15-mm-Ausgangsstab 54 mm neben der Achse endend, der um 30 Grad abwärts geneigte Spindelstumpf, das 42-mm-Element als Linse mit der aufgewickelten Kapillare als geschlossene 60-mm-Trommel auf seinem Deckel, die 1,6-mm-Zuleitung in fünf Stationen, der Fühler von 14,5 mal 50 mm als Stab mit runden Enden auf der 16-mm-Saugleitung."),
    text("Thermal: every region references its body mass; the capillary region is the drum and the lead; the cut pipe ends are section faces.",
         "Thermik: Jede Region verweist auf ihre Körpermasse; die Region Kapillare ist die Trommel mit der Zuleitung; die Rohrschnittenden sind Schnittflächen."),
    text("Structure: the liquid line and the body are one 15 mm column; the outlet, the element, the spindle housing and the suction line are their own members; one joint at the valve centre where outlet and spindle branch; a pin and two rollers in the cut pipe ends; one mass for the 0.4 kg.",
         "Tragwerk: Flüssigkeitsleitung und Körper sind eine 15-mm-Säule; Ausgang, Element, Spindelgehäuse und Saugleitung sind eigene Stäbe; ein Gelenk in der Ventilmitte, wo Ausgang und Spindel abzweigen; ein Festlager und zwei Rollenlager in den Rohrschnittenden; eine Masse für die 0,4 kg."),
]

DECISIONS500 = [
    text("Body, 1:500 (model only, 75 mm feature; the drawing is unavailable at this scale): a tee of straight liquid line, forged barrel and outlet bar; element and coil are one turned head (the element lens under the coil drum); the inclined spindle stub stays because it makes the angle valve; bulb and suction line are plain rods joined to the head by the three-station capillary lead.",
         "Körper, 1:500 (nur Modell, 75-mm-Merkmal; die Zeichnung ist bei diesem Maßstab nicht verfügbar): ein T aus gerader Flüssigkeitsleitung, Schmiedezylinder und Ausgangsstab; Element und Wicklung sind ein gedrehter Kopf (die Elementlinse unter der Wicklungstrommel); der geneigte Spindelstumpf bleibt, weil er das Eckventil ausmacht; Fühler und Saugleitung sind einfache Stäbe, mit dem Kopf über die dreiteilige Kapillarzuleitung verbunden."),
    text("Thermal: every region references its body mass; the capillary region is the lead (the coil is merged into the head); the cut pipe ends are section faces.",
         "Thermik: Jede Region verweist auf ihre Körpermasse; die Region Kapillare ist die Zuleitung (die Wicklung ist im Kopf aufgegangen); die Rohrschnittenden sind Schnittflächen."),
    text("Structure: one column, the outlet, the element, the spindle housing and the suction line as members, a pin and two rollers in the cut pipe ends, one mass.",
         "Tragwerk: eine Säule, Ausgang, Element, Spindelgehäuse und Saugleitung als Stäbe, ein Festlager und zwei Rollenlager in den Rohrschnittenden, eine Masse."),
]


def ring_path(r_out, r_in, n=36):
    """⭕ An SVG path of an annulus (outer counter-clockwise, inner clockwise) about the origin, metres."""
    def loop(r, sign):
        pts = [(r * math.cos(sign * 2 * math.pi * i / n), r * math.sin(sign * 2 * math.pi * i / n)) for i in range(n)]
        return "M " + " L ".join(f"{round(x, 6)} {round(y, 6)}" for x, y in pts) + " Z"
    return loop(r_out, 1) + " " + loop(r_in, -1)


def volumes(old):
    """📦 The whole `volumes` member: 1:50 kept, 1:100 to 1:500 authored here."""
    v = copy.deepcopy(old)
    b100, b200, b500 = body100(), body200(), body500()
    dec100 = [d for d in v["100"]["decisions"] if not d["en"].startswith("Body: the seven capillary turns")]
    dec100[1] = text(dec100[1]["en"].replace("coil disc", "coil ring"), dec100[1]["de"].replace("Windungsscheibe", "Windungsring"))
    dec100[0] = text(dec100[0]["en"].replace(", the seven capillary turns one disc of 60 mm resting on that boss", ""), dec100[0]["de"].replace(", die sieben Kapillarwindungen eine Scheibe von 60 mm auf dieser Nabe", ""))
    v["100"] = {**v["100"], "body": b100, "thermal": refs(b100, REGIONS, FACES), "structure": structure100(), "decisions": [dec100[0], DECISIONS100_COIL, *dec100[1:]]}
    v["200"] = {"status": "authored", "denominator": 200, "body": b200, "thermal": refs(b200, REGIONS, FACES), "structure": structure200(), "decisions": DECISIONS200}
    v["500"] = {"status": "authored", "denominator": 500, "body": b500, "thermal": refs(b500, REGIONS, FACES), "structure": structure500(), "decisions": DECISIONS500}
    return v


def roots(asset):
    """🌱 Root edits: inlet area from the 1:50 model (C16, factor 1.49), 1:100 plan coil drawn as the ring."""
    for r in asset["thermal"]["regions"]:
        if r["id"] == "inlet":
            r["areaM2"] = INLET_AREA
    for s in asset["representations"]["100"]["geometry"]["plan"]:
        if s["id"] == "capillary-coil":
            for k in ("cx", "cy", "rx", "ry"):
                s.pop(k, None)
            s["kind"] = "path"
            s["d"] = ring_path(0.03, 0.015)


def main():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    old = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    cand = copy.deepcopy(doc)
    roots(cand["asset"])
    cand["asset"]["volumes"] = volumes(old if "--from-catalog" not in sys.argv else doc["asset"]["volumes"])
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    json.dump(cand, open(OUT, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("candidate", OUT)
    if "--apply" in sys.argv:
        text_ = open(CATALOG, encoding="utf-8").read()
        have = json.loads(text_)["asset"]
        for key in ("thermal", "representations"):
            if have[key] != cand["asset"][key]:
                start, end = member_range(text_, "    ", key)
                text_ = text_[:start] + "    " + json.dumps(key) + ": " + json.dumps(cand["asset"][key], indent=2, ensure_ascii=False).replace("\n", "\n    ") + text_[end:]
        open(CATALOG, "w", encoding="utf-8").write(text_)
        json.dump({"volumes": cand["asset"]["volumes"]}, open(SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
        print("applied roots and source")


if __name__ == "__main__":
    main()
