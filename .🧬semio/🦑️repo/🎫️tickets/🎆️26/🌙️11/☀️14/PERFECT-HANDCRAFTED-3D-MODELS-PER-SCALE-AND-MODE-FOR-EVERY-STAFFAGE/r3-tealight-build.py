"""🕯️ Round 3 tealight: every solid, every 2D shape and every root value stated by hand (millimetres, written as metres).

Usage: python3 r3-tealight-build.py volumes | candidate | catalog
  volumes    writes volumes/objects/tealight.json
  candidate  writes 🗑️generated/r3-tealight/🔣️.json (catalog document + root fixes + volumes)
  catalog    re-reads the catalog file and writes the root fixes into it (volumes are spliced by volumes-insert.py)
"""
import copy
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/tealight/🔣️.json"))
SOURCE = os.path.join(T, "volumes/objects/tealight.json")
CANDIDATE = os.path.join(T, "🗑️generated/r3-tealight/🔣️.json")
AXIS = [1, 0, 0]
ORIGIN = [0, 0, 0]


def m(mm):
    return round(mm / 1000, 6)


def pts(profile):
    return [[m(a), m(r)] for a, r in profile]


def revolve(id, bone, material, role, tone, profile, **extra):
    return {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": pts(profile), "axis": AXIS, "origin": ORIGIN, **extra}


def loop(profile):
    return profile + [profile[0]]


def ball(id, bone, material, tone, radius, along=0.0, **extra):
    solid = {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": tone, "radii": [m(radius)] * 3, **extra}
    if along:
        solid["transform"] = {"translation": [m(along), 0, 0]}
    return solid


def rod(id, bone, material, tone, a, b, radius, **extra):
    return {"id": id, "kind": "capsule", "bone": bone, "material": material, "role": "detail", "tone": tone, "p0": [m(a), 0, 0], "p1": [m(b), 0, 0], "r0": m(radius), "r1": m(radius), **extra}


CUP_50 = [
    (0.35, 0), (0.35, 13.0), (0.0, 14.0), (0.0, 17.06), (0.076, 17.44), (0.293, 17.77), (0.617, 17.98), (1.0, 18.06), (16.0, 19.0),
    (16.0, 18.9), (1.0, 17.96), (0.68, 17.89), (0.37, 17.70), (0.17, 17.40), (0.1, 17.06), (0.1, 14.04), (0.45, 13.04), (0.45, 0),
]
WAX_50 = loop([
    (0, 6.3), (0, 16.9), (1.0, 17.82), (13.2, 18.60), (14.0, 17.8), (14.0, 1.05), (3.4, 1.05), (3.4, 1.9), (0.6, 1.9), (0.6, 6.3),
])
WICK_50 = [(-14.0, 0), (-14.0, 6.0), (-13.7, 6.0), (-13.7, 2.3), (-13.3, 1.7), (-11.0, 1.7), (-11.0, 1.0), (6.0, 1.0), (6.6, 0.85), (6.9, 0.5), (7.0, 0)]
FLAME_50 = [
    (0, 0), (0.3, 1.2), (1.0, 2.2), (2.0, 3.0), (3.5, 3.6), (5.0, 3.9), (7.0, 4.0), (9.0, 3.85), (11.0, 3.5), (12.0, 3.25),
    (12.6, 2.9), (13.1, 2.2), (13.4, 1.2), (13.5, 0),
]
TIP_50 = [
    (-2.5, 0), (-2.35, 1.6), (-2.0, 2.6), (-1.5, 3.1), (-1.0, 3.25), (0.0, 3.3), (2.0, 3.3), (4.0, 3.25), (5.0, 3.0), (7.0, 2.4), (9.0, 1.75),
    (11.0, 1.1), (12.5, 0.6), (13.5, 0.25), (14.0, 0),
]
PLUME_50 = [(14.2, 0), (13.5, 0.45), (12.5, 0.8), (11.0, 1.3), (9.0, 1.95), (8.0, 2.28), (8.0, 2.08), (9.0, 1.75), (11.0, 1.1), (12.5, 0.6), (13.5, 0.25), (14.0, 0)]
SUPPORT_RING = loop([(-0.3, 14.0), (-0.3, 17.06), (0, 17.06), (0, 14.0)])
WAX_FACES = ["-u", "+v", "-v", "+w", "-w"]

CUP_100 = [(0, 0), (0, 18.0), (16.0, 19.0), (16.0, 18.9), (0.1, 17.9), (0.1, 0)]
PLUG_100 = [(-14.35, 0), (-14.35, 17.75), (0, 18.6), (0, 1.0), (6.5, 1.0), (7.0, 0)]
FLAME_100 = [(0, 0), (0.5, 1.5), (1.5, 2.7), (3.5, 3.6), (7.0, 4.0), (11.0, 3.5), (12.6, 2.9), (13.5, 0)]
TIP_100 = [(-2.5, 0), (-2.0, 2.6), (-1.0, 3.25), (0.0, 3.3), (4.0, 3.25), (7.0, 2.4), (11.0, 1.1), (14.0, 0)]
PLUME_100 = [(14.3, 0), (11.0, 1.4), (8.0, 2.35), (8.0, 2.08), (11.0, 1.1), (14.0, 0)]

CUP_200 = [(0, 0), (0, 18.0), (16.0, 19.0), (16.0, 18.8), (14.5, 18.5), (14.5, 0)]
SHELL_200 = [(0, 0), (0, 18.0), (16.0, 19.0), (16.0, 18.8), (0.2, 17.8), (0.2, 0)]
SKIN_200 = [(0, 0), (0, 18.3), (0.3, 18.3), (0.3, 1.0), (7.0, 0)]
WICK_200 = [(0, 0), (0, 1.05), (6.5, 1.05), (7.2, 0)]
FLAME_200 = [(0, 0), (2.0, 3.0), (7.0, 4.0), (13.0, 3.0), (19.0, 1.1), (22.0, 0)]
PLUME_200 = [(22.3, 0), (16.0, 2.3), (16.0, 2.05), (22.0, 0)]

CUP_500 = [(0, 0), (0, 18.0), (16.0, 19.0), (14.5, 18.3), (14.5, 0)]
FLAME_500 = [(0, 0), (1.5, 2.8), (7.0, 4.0), (22.0, 0)]
SKIN_500 = [(0, 0), (0, 18.2), (0.2, 18.2), (0.2, 1.0), (7.0, 0)]
SHELL_500 = [(0, 0), (0, 18.0), (16.0, 19.0), (16.0, 18.7), (0.3, 17.7), (0.3, 0)]
WICK_500 = [(0.1, 0), (0.1, 1.1), (7.1, 0)]
PLUME_500 = [(22.3, 0), (14.0, 2.4), (14.0, 2.13), (22.0, 0)]


def text(en, de):
    return {"en": en, "de": de}


def volume_50():
    body = [
        revolve("cup", "cup", "aluminium-cup", "body", 0.5, CUP_50),
        revolve("wax", "wax", "paraffin", "body", 0.85, WAX_50),
        revolve("wick", "wick", "wick-cotton", "detail", 0.1, WICK_50),
        revolve("flame", "flame-body", "flame", "body", 0.9, FLAME_50),
        revolve("flame-tongue", "flame-tip", "flame", "body", 0.97, TIP_50),
    ]
    thermal = [
        {"id": "th-cup", "of": "cup", "region": "cup", "layer": "foil"},
        {"id": "th-wax", "of": "wax", "region": "wax", "layer": "wax", "sectionFaces": WAX_FACES},
        revolve("th-wick", "wick", "wick-cotton", "detail", 0.1, [(0, 0), (0, 1.0), (6.0, 1.0), (6.6, 0.85), (6.9, 0.5), (7.0, 0)], region="wick", layer="braid"),
        revolve("th-wick-root", "wick", "wick-cotton", "detail", 0.25, [(-14.0, 0), (-14.0, 6.0), (-13.7, 6.0), (-13.7, 2.3), (-13.3, 1.7), (-11.0, 1.7), (-11.0, 1.0), (-0.05, 1.0), (-0.05, 0)], region="wick", layer="braid", sectionFaces=["-u", "+u", "-v", "+v", "-w", "+w"]),
        {"id": "th-flame", "of": "flame", "region": "flame", "layer": "envelope"},
        {"id": "th-flame-tongue", "of": "flame-tongue", "region": "flame", "layer": "envelope"},
        revolve("th-flame-plume", "flame-tip", "flame", "detail", 0.97, PLUME_50, region="flame", layer="envelope", tags=["flame-plume"]),
    ]
    structure = [
        {"id": "member-cup", "of": "cup", "member": "cup", "part": "member"},
        revolve("member-wax", "wax", "paraffin", "body", 0.7, [(0, 0), (0, 13.0), (14.0, 13.0), (14.0, 0)], member="wax", part="member"),
        {"id": "member-wick", "of": "wick", "member": "wick", "part": "member"},
        rod("member-flame-body", "flame-body", "flame", 0.9, 0.3, 13.2, 0.28, slender=True, member="flame-body", part="member"),
        rod("member-flame-tip", "flame-tip", "flame", 0.97, 0.0, 13.6, 0.28, slender=True, member="flame-tip", part="member"),
        revolve("joint-wax", "wax", "paraffin", "ground", 0.3, [(-0.15, 0), (-0.15, 13.0), (0.15, 13.0), (0.15, 0)], member="wax", part="joint"),
        ball("joint-wick", "wick", "wick-cotton", 0.3, 3.75, member="wick", part="joint"),
        {"id": "mass-wax", "kind": "ellipsoid", "bone": "wax", "material": "paraffin", "role": "detail", "tone": 0.15, "radii": [m(3.75), m(5.5), m(5.5)], "transform": {"translation": [m(7), 0, 0]}, "member": "wax", "part": "mass"},
        revolve("support-base-contact", "cup", "aluminium-cup", "ground", 0.1, SUPPORT_RING, member="cup", part="support", anchor="base-contact", tags=["base-contact"]),
    ]
    decisions = [
        text(
            "Body 1:50, the real tealight part by part: the deep-drawn cup is one shell of 0.1 mm foil (C30) with the embossed floor (centre panel raised 0.35 mm inside a standing ring of 28 to 34 mm), the 1 mm base radius and the wall with 3.6 degrees of draft from 36 mm at the base to 38 mm at the cut rim; the pressed paraffin puck of 14 mm lies loose in it (0.15 mm clearance, chamfered edges, the recess for the sustainer and the wick bore); the wick is one solid of the 2 mm braid with its sustainer (12 mm disc, 3 mm crimp neck) standing on the floor, running through the puck and standing 7 mm above the wax; the flame is the luminous teardrop of 8 mm by 22 mm in two parts that slide into each other (lower bulb on flame-body, upper tongue on flame-tip), so that the stretched flame of the flicker poses stays closed. In the unlit pose both flame parts sink completely into the puck and the wick.",
            "Körper 1:50, das echte Teelicht Teil für Teil: das tiefgezogene Schälchen ist eine Schale aus 0,1 mm Folie (C30) mit geprägtem Boden (Mittelfeld 0,35 mm angehoben in einem Standring von 28 bis 34 mm), dem Bodenradius von 1 mm und der Wand mit 3,6 Grad Auszugsschräge von 36 mm am Boden auf 38 mm am Schnittrand; der gepresste Paraffinkörper von 14 mm liegt lose darin (0,15 mm Spiel, gefaste Kanten, Aussparung für den Dochthalter und Dochtbohrung); der Docht ist ein Körper aus dem 2-mm-Geflecht mit seinem Dochthalter (Scheibe 12 mm, Quetschhals 3 mm), der auf dem Boden steht, durch den Wachskörper läuft und 7 mm über das Wachs ragt; die Flamme ist der leuchtende Tropfen von 8 mm mal 22 mm in zwei ineinander gleitenden Teilen (untere Zwiebel am Knochen flame-body, obere Zunge am Knochen flame-tip), damit die gestreckte Flamme der Flackerposen geschlossen bleibt. In der gelöschten Pose versinken beide Flammenteile vollständig im Wachskörper und im Docht.",
        ),
        text(
            "Thermal 1:50: the cup is its real foil shell (layer foil, inside and outside count); the wax region is the puck with only its top face as surface (the faces against cup, wick and sustainer are section faces); the wick region is the 7 mm of braid in the air, and its root (the braid inside the puck with the sustainer) is a real internal whose faces lie against the wax and count for no area; the flame region is both flame parts; the emitter flame-plume is an outlet cap of 0.2 mm on the upper 6 mm of the tongue, where the plume leaves the flame (D2).",
            "Thermik 1:50: das Schälchen ist seine echte Folienschale (Schicht foil, innen und aussen zählen); die Wachsregion ist der Wachskörper mit nur seiner Oberseite als Oberfläche (die Flächen gegen Schälchen, Docht und Dochthalter sind Schnittflächen); die Dochtregion sind die 7 mm Geflecht an der Luft, und ihre Wurzel (das Geflecht im Wachskörper mit dem Dochthalter) ist echtes Innenleben, dessen Flächen am Wachs anliegen und keine Fläche zählen; die Flammenregion sind beide Flammenteile; der Emitter flame-plume ist eine Auslasskappe von 0,2 mm auf den oberen 6 mm der Zunge, wo die Fahne die Flamme verlässt (D2).",
        ),
        text(
            "Structure 1:50, the real load path: the puck (12.8 g, mass at its centre) stands as a pressure column of 26 mm on the raised floor panel (bearing joint), the foil shell carries it through the embossed floor into the standing ring, and the annular pad under the ring is the support base-contact; the wick is a strand in its sustainer foot with a joint marker of one feature where it leaves the wax; the flame bones are massless axes (slender rods of 0.56 mm) and carry no load, so they have no joint markers.",
            "Tragwerk 1:50, der echte Lastpfad: der Wachskörper (12,8 g, Masse im Schwerpunkt) steht als Drucksäule von 26 mm auf dem angehobenen Bodenfeld (Lagerfuge), die Folienschale trägt ihn über den geprägten Boden in den Standring, und das Ringpolster unter dem Standring ist das Auflager base-contact; der Docht ist ein Strang in seinem Halterfuss mit einer Gelenkmarke von einem Merkmal am Austritt aus dem Wachs; die Flammenknochen sind masselose Achsen (schlanke Stäbe von 0,56 mm) und tragen keine Last, deshalb haben sie keine Gelenkmarken.",
        ),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def volume_100():
    body = [
        revolve("cup", "cup", "aluminium-cup", "body", 0.5, CUP_100),
        revolve("wax-top", "wick", "paraffin", "body", 0.85, PLUG_100),
        revolve("flame", "flame-body", "flame", "body", 0.9, FLAME_100),
        revolve("flame-tongue", "flame-tip", "flame", "body", 0.97, TIP_100),
    ]
    thermal = [
        {"id": "th-cup", "of": "cup", "region": "cup", "layer": "foil"},
        {"id": "th-wax", "of": "wax-top", "region": "wax", "layer": "wax", "sectionFaces": WAX_FACES},
        revolve("th-wick", "wick", "wick-cotton", "detail", 0.1, [(0, 0), (0, 1.05), (6.5, 1.05), (7.1, 0)], region="wick", layer="braid"),
        {"id": "th-flame", "of": "flame", "region": "flame", "layer": "envelope"},
        {"id": "th-flame-tongue", "of": "flame-tongue", "region": "flame", "layer": "envelope"},
        revolve("th-flame-plume", "flame-tip", "flame", "detail", 0.97, PLUME_100, region="flame", layer="envelope", tags=["flame-plume"]),
    ]
    structure = [
        {"id": "member-cup", "of": "cup", "member": "cup", "part": "member"},
        {"id": "member-wax", "of": "wax-top", "member": "wax", "part": "member"},
        rod("member-wick", "wick", "wick-cotton", 0.1, -13.8, 6.6, 0.45, slender=True, member="wick", part="member"),
        rod("member-flame-body", "flame-body", "flame", 0.9, 0.4, 13.0, 0.3, slender=True, member="flame-body", part="member"),
        rod("member-flame-tip", "flame-tip", "flame", 0.97, 0.2, 13.4, 0.3, slender=True, member="flame-tip", part="member"),
        revolve("support-base-contact", "cup", "aluminium-cup", "ground", 0.1, loop([(-0.4, 14.0), (-0.4, 17.5), (0, 17.5), (0, 14.0)]), member="cup", part="support", anchor="base-contact", tags=["base-contact"]),
    ]
    decisions = [
        text(
            "Body 1:100, drawn anew for this scale: the cup is a plain foil cone (flat floor, straight wall from 36 to 38 mm, no embossing, no base radius); the wax fill and the wick are one plug on the wick bone, a plain truncated cone with a 2 mm pin of 7 mm on top (the sustainer, the bore and the clearance are gone); the flame is the same two sliding parts with fewer profile stations. Nothing is thickened: the candle keeps its true size.",
            "Körper 1:100, für diesen Maßstab neu gezeichnet: das Schälchen ist ein schlichter Folienkegel (flacher Boden, gerade Wand von 36 auf 38 mm, keine Prägung, kein Bodenradius); Wachsfüllung und Docht sind ein Pfropfen am Dochtknochen, ein schlichter Kegelstumpf mit einem 2-mm-Stift von 7 mm obenauf (Dochthalter, Bohrung und Spiel entfallen); die Flamme sind dieselben zwei gleitenden Teile mit weniger Profilstationen. Nichts ist verdickt: die Kerze behält ihre wahre Grösse.",
        ),
        text(
            "Thermal 1:100: cup shell and wax plug by reference (wax: top face only), the wick as its own pin over the pin of the plug, both flame parts, and a plain cone cap on the upper 6 mm of the tongue as the emitter outlet.",
            "Thermik 1:100: Schälchenschale und Wachspfropfen als Verweis (Wachs: nur die Oberseite), der Docht als eigener Stift über dem Stift des Pfropfens, beide Flammenteile und eine schlichte Kegelkappe auf den oberen 6 mm der Zunge als Emitterauslass.",
        ),
        text(
            "Structure 1:100: the foil shell, the wax plug as the pressure block it is, the wick as a slender strand from the floor to its tip, the two massless flame axes and the annular support under the base. Joint markers and the mass marker are left out: one feature (15 mm) is larger than the 14 mm puck that would have to hold them.",
            "Tragwerk 1:100: die Folienschale, der Wachspfropfen als der Druckblock, der er ist, der Docht als schlanker Strang vom Boden bis zur Spitze, die zwei masselosen Flammenachsen und das Ringauflager unter dem Boden. Gelenkmarken und die Massenmarke entfallen: ein Merkmal (15 mm) ist grösser als der 14 mm hohe Wachskörper, der sie aufnehmen müsste.",
        ),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def volume_200():
    body = [
        revolve("cup", "cup", "aluminium-cup", "body", 0.5, CUP_200),
        revolve("wax-top", "wick", "paraffin", "detail", 0.85, SKIN_200, texture=True),
        revolve("flame", "flame-body", "flame", "body", 0.9, FLAME_200),
    ]
    thermal = [
        {"id": "th-cup", "of": "cup", "region": "cup", "layer": "foil"},
        {"id": "th-wax", "of": "wax-top", "region": "wax", "layer": "wax", "sectionFaces": WAX_FACES},
        revolve("th-wick", "wick", "wick-cotton", "detail", 0.1, WICK_200, region="wick", layer="braid"),
        {"id": "th-flame", "of": "flame", "region": "flame", "layer": "envelope"},
        revolve("th-flame-plume", "flame-body", "flame", "detail", 0.97, PLUME_200, region="flame", layer="envelope", tags=["flame-plume"]),
    ]
    structure = [
        revolve("member-cup", "cup", "aluminium-cup", "body", 0.5, SHELL_200, member="cup", part="member"),
        {"id": "member-wax", "of": "cup", "member": "wax", "part": "member"},
        rod("member-wick", "wick", "wick-cotton", 0.1, -13.6, 6.4, 0.45, slender=True, member="wick", part="member"),
        rod("member-flame-body", "flame-body", "flame", 0.9, 0.5, 12.9, 0.3, slender=True, member="flame-body", part="member"),
        rod("member-flame-tip", "flame-tip", "flame", 0.97, 0.4, 12.4, 0.27, slender=True, member="flame-tip", part="member"),
        revolve("support-base-contact", "cup", "aluminium-cup", "ground", 0.1, [(-0.5, 0), (-0.5, 18.0), (0, 18.0), (0, 0)], member="cup", part="support", anchor="base-contact", tags=["base-contact"]),
    ]
    decisions = [
        text(
            "Body 1:200, three masses: the cup with its wax fill as one solid cone of 36 to 38 mm with the wax level sunk 1.5 mm inside the rim (C30: the cavity is below the feature), the wax surface with the wick as a cone of 7 mm on the wick bone, as the plan draws it (a 0.3 mm sheet, flagged texture), and the flame as one drop of 8 mm by 22 mm on flame-body; the separate tongue, the sustainer, embossing and radii are gone. The model keeps the true size of the candle: it is 0.19 mm on paper and is not thickened.",
            "Körper 1:200, drei Massen: das Schälchen mit seiner Wachsfüllung als ein voller Kegel von 36 auf 38 mm mit dem 1,5 mm im Rand abgesenkten Wachsspiegel (C30: der Hohlraum liegt unter dem Merkmal), die Wachsoberfläche mit dem Docht als Kegel von 7 mm am Dochtknochen, wie ihn die Draufsicht zeichnet (eine Schicht von 0,3 mm, Kennzeichen texture), und die Flamme als ein Tropfen von 8 mm mal 22 mm am Knochen flame-body; die eigene Zunge, der Dochthalter, Prägung und Radien entfallen. Das Modell behält die wahre Grösse der Kerze: sie misst 0,19 mm auf dem Papier und wird nicht verdickt.",
        ),
        text(
            "Thermal 1:200: the cup cone and the wax surface by reference (wax: top face only), the wick as its own 2 mm pin over the cone, the flame drop, and a cone cap on its upper 6 mm as the emitter outlet; only the total area is compared at this scale.",
            "Thermik 1:200: der Schälchenkegel und die Wachsoberfläche als Verweis (Wachs: nur die Oberseite), der Docht als eigener 2-mm-Stift über dem Kegel, der Flammentropfen und eine Kegelkappe auf seinen oberen 6 mm als Emitterauslass; in diesem Maßstab wird nur die Gesamtfläche verglichen.",
        ),
        text(
            "Structure 1:200: a cone shell with a 0.2 mm wall as the cup member, the solid cone inside it as the wax block, the wick as a slender strand, the two massless flame axes and one full contact disc under the base as the support; no joint or mass markers (one feature, 30 mm, is as large as the candle).",
            "Tragwerk 1:200: eine Kegelschale mit 0,2 mm Wand als Schälchenstab, der volle Kegel darin als Wachsblock, der Docht als schlanker Strang, die zwei masselosen Flammenachsen und eine volle Kontaktscheibe unter dem Boden als Auflager; keine Gelenk- und Massenmarken (ein Merkmal, 30 mm, ist so gross wie die Kerze).",
        ),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def volume_500():
    body = [
        revolve("cup", "cup", "aluminium-cup", "body", 0.5, CUP_500),
        revolve("wax-top", "wick", "paraffin", "detail", 0.85, SKIN_500),
        revolve("flame", "flame-body", "flame", "body", 0.9, FLAME_500),
    ]
    thermal = [
        {"id": "th-cup", "of": "cup", "region": "cup", "layer": "foil"},
        {"id": "th-wax", "of": "wax-top", "region": "wax", "layer": "wax", "sectionFaces": WAX_FACES},
        revolve("th-wick", "wick", "wick-cotton", "detail", 0.1, WICK_500, region="wick", layer="braid"),
        {"id": "th-flame", "of": "flame", "region": "flame", "layer": "envelope"},
        revolve("th-flame-plume", "flame-body", "flame", "detail", 0.97, PLUME_500, region="flame", layer="envelope", tags=["flame-plume"]),
    ]
    structure = [
        revolve("member-cup", "cup", "aluminium-cup", "body", 0.5, SHELL_500, member="cup", part="member"),
        revolve("member-wax", "wax", "paraffin", "body", 0.7, [(0, 0), (0, 13.0), (13.5, 13.0), (13.5, 0)], member="wax", part="member"),
        revolve("member-wick", "wick", "wick-cotton", "detail", 0.1, [(-13.5, 0), (-13.5, 0.45), (6.0, 0.45), (6.0, 0)], slender=True, member="wick", part="member"),
        revolve("member-flame-body", "flame-body", "flame", "detail", 0.9, [(0.5, 0), (0.5, 0.27), (12.5, 0.27), (12.5, 0)], slender=True, member="flame-body", part="member"),
        revolve("member-flame-tip", "flame-tip", "flame", "detail", 0.97, [(0.5, 0), (0.5, 0.25), (11.0, 0.25), (11.0, 0)], slender=True, member="flame-tip", part="member"),
        revolve("support-base-contact", "cup", "aluminium-cup", "ground", 0.1, [(-0.6, 0), (-0.6, 18.0), (0, 18.0), (0, 0)], member="cup", part="support", anchor="base-contact", tags=["base-contact"]),
    ]
    decisions = [
        text(
            "Body 1:500, digital only (the candle is 0.08 mm on paper, the 2D representation is unavailable): the cup with its wax as one solid cone of 36 to 38 mm with a sunk top, the wax surface with a wick cone that reaches the flame, and the flame as one four-station drop of 8 mm by 22 mm, all in true size. The model is already minimal.",
            "Körper 1:500, nur digital (die Kerze misst 0,08 mm auf dem Papier, die 2D-Darstellung ist nicht verfügbar): das Schälchen mit seinem Wachs als ein voller Kegel von 36 auf 38 mm mit abgesenkter Oberseite, die Wachsoberfläche mit einem Dochtkegel, der die Flamme erreicht, und die Flamme als ein Tropfen von 8 mm mal 22 mm mit vier Stationen, alles in wahrer Grösse. Das Modell ist bereits minimal.",
        ),
        text(
            "Thermal 1:500: the cup cone and the wax surface by reference (wax: top face only), the wick as a cone, the flame drop and its outlet cap; only the total area is compared at this scale.",
            "Thermik 1:500: der Schälchenkegel und die Wachsoberfläche als Verweis (Wachs: nur die Oberseite), der Docht als Kegel, der Flammentropfen und seine Auslasskappe; in diesem Maßstab wird nur die Gesamtfläche verglichen.",
        ),
        text(
            "Structure 1:500: not the body block but the load path in its plainest form: a cone shell with a 0.3 mm wall, the wax pressure column, plain slender cylinders for the wick strand and the two massless flame axes, and the contact disc as the support. The model is already minimal.",
            "Tragwerk 1:500: nicht der Körperblock, sondern der Lastpfad in seiner schlichtesten Form: eine Kegelschale mit 0,3 mm Wand, die Wachs-Drucksäule, schlichte schlanke Zylinder für den Dochtstrang und die zwei masselosen Flammenachsen und die Kontaktscheibe als Auflager. Das Modell ist bereits minimal.",
        ),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def volumes():
    return {"50": volume_50(), "100": volume_100(), "200": volume_200(), "500": volume_500()}


def mirror(profile):
    return [[m(a), m(r)] for a, r in profile] + [[m(a), m(-r)] for a, r in reversed(profile) if r != 0]


def world(profile, base):
    return [[m(r), m(base + a)] for a, r in profile] + [[m(-r), m(base + a)] for a, r in reversed(profile) if r != 0]


FLAME_2D_50 = [(0, 0), (1.0, 2.2), (3.5, 3.6), (7.0, 4.0), (11.0, 3.5), (12.6, 2.9), (13.5, 0)]
TIP_2D_50 = [(-2.5, 0), (-2.0, 2.6), (-1.0, 3.25), (0.0, 3.3), (4.0, 3.25), (7.0, 2.4), (11.0, 1.1), (14.0, 0)]
FLAME_2D_100 = [(0, 0), (2.0, 3.0), (7.0, 4.0), (11.0, 3.5), (13.5, 0)]
TIP_2D_100 = [(-2.5, 0), (-1.5, 3.1), (0.0, 3.3), (4.0, 3.25), (9.0, 1.75), (14.0, 0)]
FLAME_2D_200 = [(0, 0), (2.0, 3.0), (7.0, 4.0), (13.0, 3.0), (22.0, 0)]
CUP_2D = [(0, 18.0), (16.0, 19.0)]
FLAME_BASE = 18.5
TIP_BASE = 26.5


def shape(old, **geometry):
    kept = {key: value for key, value in old.items() if key in ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm")}
    return {**kept, **geometry}


def poly(points, closed=True):
    return {"kind": "poly", "points": points, "closed": closed}


def ellipse(radius):
    return {"kind": "ellipse", "cx": 0.0, "cy": 0.0, "rx": m(radius), "ry": m(radius)}


def redraw(representation, flame, tip):
    geometry = representation["geometry"]
    for view in ("elevation", "front", "plan"):
        shapes = []
        for old in geometry[view]:
            id = old["id"]
            if view == "plan":
                radius = {"cup": 19.0, "wax-surface": 17.8, "wax-top": 17.8, "flame": 4.0}[id]
                shapes.append(shape(old, **ellipse(radius)))
            elif id == "cup":
                shapes.append(shape(old, **poly(mirror(CUP_2D) if view == "elevation" else world(CUP_2D, 0))))
            elif id == "flame":
                shapes.append(shape(old, **poly(mirror(flame) if view == "elevation" else world(flame, FLAME_BASE))))
            elif id == "tongue":
                shapes.append(shape(old, **poly(mirror(tip) if view == "elevation" else world(tip, TIP_BASE))))
            elif id == "wick":
                shapes.append(shape(old, **poly([[0, 0], [m(7), 0]] if view == "elevation" else [[0, m(14.5)], [0, m(21.5)]], closed=False)))
            elif id == "wax-surface":
                shapes.append(shape(old, **poly([[m(14), m(14)], [m(14), m(-14)]] if view == "elevation" else [[m(-14), m(14.5)], [m(14), m(14.5)]], closed=False)))
            else:
                raise SystemExit("unknown 2D shape " + id)
        geometry[view] = shapes


DECISIONS_2D = {
    "50": {
        "silhouette": text(
            "A 38 mm by 40.5 mm candle, 0.8 mm by 0.8 mm on paper, drawn as the true view of the 1:50 body: the cup with 3.6 degrees of draft (36 mm at the base, 38 mm at the rim) and the flame of 8 mm by 22 mm standing on the wick. Side and front view: the cup, the wax level as a line inside it, the wick, the flame bulb and its tongue; bulb and tongue lift, lean and stretch with their bones. Plan view: the rim disc, the wax disc and the flame disc in the middle; in the unlit pose the flame parts sink into the wax and the wick and are hidden.",
            "Eine Kerze von 38 mm mal 40,5 mm, auf Papier 0,8 mm mal 0,8 mm, gezeichnet als wahre Ansicht des 1:50-Körpers: das Schälchen mit 3,6 Grad Auszugsschräge (36 mm am Boden, 38 mm am Rand) und die Flamme von 8 mm mal 22 mm auf dem Docht. Seiten- und Vorderansicht: das Schälchen, der Wachsspiegel als Linie darin, der Docht, die Flammenzwiebel und ihre Zunge; Zwiebel und Zunge heben, neigen und strecken sich mit ihren Knochen. Draufsicht: die Randscheibe, die Wachsscheibe und die Flammenscheibe in der Mitte; in der gelöschten Pose versinken die Flammenteile in Wachs und Docht und sind verdeckt.",
        ),
        "retained": text(
            "The 16 mm cup widening from 36 to 38 mm, the wax level at 14.5 mm as a straight line, the wick standing 7 mm above the wax, the flame bulb and its tongue. Front view: the same. Plan view: rim, wax disc, flame disc.",
            "Das 16 mm hohe Schälchen, das sich von 36 auf 38 mm weitet, der Wachsspiegel bei 14,5 mm als gerade Linie, der 7 mm über das Wachs ragende Docht, die Flammenzwiebel und ihre Zunge. Vorderansicht: dasselbe. Draufsicht: Rand, Wachsscheibe, Flammenscheibe.",
        ),
        "omitted": text(
            "The embossed floor and the base radius of the cup, the sustainer, the foil thickness, the blue flame foot and the melt pool. Plan view: the wick and the tongue are hidden by the flame; the sustainer is hidden below the wax.",
            "Der geprägte Boden und der Bodenradius des Schälchens, der Dochthalter, die Folienstärke, der blaue Flammenfuss und das Schmelzbad. Draufsicht: Docht und Zunge sind von der Flamme verdeckt; der Dochthalter liegt verdeckt unter dem Wachs.",
        ),
        "thinElements": text(
            "The flame keeps its true 8 mm (0.16 mm on paper, above the 0.15 mm feature); the wick and the wax level are 0.13 mm lines (the wax line stops 4 mm short of the wall so that its pen stays inside the cup); the tongue runs out into a sharp tip.",
            "Die Flamme behält ihre wahren 8 mm (0,16 mm auf Papier, über dem Merkmal von 0,15 mm); Docht und Wachsspiegel sind 0,13-mm-Linien (die Wachslinie endet 4 mm vor der Wand, damit ihr Stift im Schälchen bleibt); die Zunge läuft spitz aus.",
        ),
    },
    "100": {
        "silhouette": text(
            "A 0.4 mm by 0.4 mm candle, the true view of the 1:100 body: the cup cone of 36 to 38 mm with the flame of 8 mm by 22 mm above it. Side and front view: the cup trapezoid, the flame bulb and its tongue as two five-station drops; the tongue rides on its own bone and stretches independently. Plan view: the cup disc, the wax disc and the flame disc inside it (the sunk flame of the unlit pose is hidden).",
            "Eine Kerze von 0,4 mm mal 0,4 mm, die wahre Ansicht des 1:100-Körpers: der Schälchenkegel von 36 auf 38 mm mit der Flamme von 8 mm mal 22 mm darüber. Seiten- und Vorderansicht: das Schälchentrapez, die Flammenzwiebel und ihre Zunge als zwei Tropfen mit fünf Stationen; die Zunge hängt an einem eigenen Knochen und streckt sich unabhängig. Draufsicht: die Schälchenscheibe, die Wachsscheibe und darin die Flammenscheibe (die versenkte Flamme der gelöschten Pose ist verdeckt).",
        ),
        "retained": text(
            "The cup cone of 16 mm height and 38 mm rim, the flame with its tongue and its lean. Front and plan view: cup (merged), flame; front view: tongue.",
            "Der Schälchenkegel von 16 mm Höhe und 38 mm Rand, die Flamme mit ihrer Zunge und ihrer Neigung. Vorder- und Draufsicht: Schälchen (verschmolzen), Flamme; Vorderansicht: Zunge.",
        ),
        "thinElements": text(
            "The flame keeps its true 8 mm: 0.08 mm on paper, which the 0.18 mm body pen carries to 0.26 mm; nothing is thickened, so the drawing and the 1:100 body are one form.",
            "Die Flamme behält ihre wahren 8 mm: 0,08 mm auf Papier, die der Körperstift von 0,18 mm auf 0,26 mm trägt; nichts ist verdickt, Zeichnung und 1:100-Körper sind eine Form.",
        ),
    },
    "200": {
        "silhouette": text(
            "A 0.2 mm dot with a flame, the true view of the 1:200 body: the cup cone of 36 to 38 mm and one flame drop of 8 mm by 22 mm. Side and front view: the cup trapezoid and the drop above it; the flame bone lifts it out of the cup. Plan view: the cup disc and the flame disc in its middle; the wax disc sits on the wick bone, so the sunk flame of the unlit pose is painted behind it and disappears.",
            "Ein Punkt von 0,2 mm mit einer Flamme, die wahre Ansicht des 1:200-Körpers: der Schälchenkegel von 36 auf 38 mm und ein Flammentropfen von 8 mm mal 22 mm. Seiten- und Vorderansicht: das Schälchentrapez und der Tropfen darüber; der Flammenhub hebt ihn aus dem Schälchen. Draufsicht: die Schälchenscheibe und die Flammenscheibe in ihrer Mitte; die Wachsscheibe hängt am Dochtknochen, die versenkte Flamme der gelöschten Pose wird deshalb dahinter gemalt und verschwindet.",
        ),
        "thinElements": text(
            "Nothing is thickened: the cup is 0.19 mm and the flame 0.04 mm on paper, and the body pen of 0.18 mm (raised from 0.13 mm) carries both above the 0.2 mm print feature over the whole band from 1:150 to 1:350; drawing and 1:200 body are one form in true size.",
            "Nichts ist verdickt: das Schälchen misst 0,19 mm und die Flamme 0,04 mm auf Papier, und der Körperstift von 0,18 mm (von 0,13 mm angehoben) trägt beide über das Druckmerkmal von 0,2 mm im ganzen Band von 1:150 bis 1:350; Zeichnung und 1:200-Körper sind eine Form in wahrer Grösse.",
        ),
        "styles": text("Cup 0.5, flame 0.9; one body pen of 0.18 mm.", "Schälchen 0,5, Flamme 0,9; ein Körperstift von 0,18 mm."),
    },
}


def bone(asset, id):
    return next(item for item in asset["rig"]["bones"] if item["id"] == id)


def roots(asset):
    asset["provenance"]["references"] = [item for item in asset["provenance"]["references"] if item["id"] != "tealight-trade-sizes"] + [
        {
            "id": "tealight-trade-sizes",
            "citation": "Trade sizes of standard tealights and their deep-drawn aluminium cups (38 mm diameter by 16 mm height; cups of 36 to 39 mm by 15 to 18 mm; separate wick with a tin-plate sustainer), en.wikipedia.org/wiki/Tealight and candle-making suppliers, read 2026-11-20.",
        }
    ]
    asset["provenance"]["notes"] = text(
        "A standard tealight: a deep-drawn cup of 0.1 mm aluminium foil, 38 mm across the cut rim, 36 mm across the base (3.6 degrees of draft, 1 mm base radius) and 16 mm high, with an embossed floor that stands on a ring of 28 to 34 mm; a pressed puck of 12.8 g of paraffin, 14 mm high, lies loose in it up to 14.5 mm; a braided cotton wick of 2 mm in a tin-plate sustainer of 12 mm runs through the puck and stands about 7 mm above the wax. The flame is 22 mm high and 8 mm wide at the widest point, a third of the way up, and begins 4 mm above the wax. A tealight burns about 4 hours, which is 3.2 g of wax per hour or 0.9 mg per second; at the heat of combustion of paraffin of 42 to 46 MJ/kg this is 38 to 41 W, of which roughly 85 percent leave the flame as convective plume and radiation; the pose lit takes 35 W and the flicker peak 40 W (authored estimate from this range). The unlit pose hides the flame inside the wax and the wick by lowering the flame bones, so one asset shows all states. The origin is the centre of the base on the surface; every bone runs up the axis of the candle.",
        "Ein handelsübliches Teelicht: ein tiefgezogenes Schälchen aus 0,1 mm Aluminiumfolie, 38 mm am Schnittrand und 36 mm am Boden weit (3,6 Grad Auszugsschräge, 1 mm Bodenradius) und 16 mm hoch, mit geprägtem Boden, der auf einem Ring von 28 bis 34 mm steht; darin liegt lose ein gepresster Körper aus 12,8 g Paraffin, 14 mm hoch, bis auf 14,5 mm; ein geflochtener Baumwolldocht von 2 mm in einem Dochthalter aus Weissblech von 12 mm läuft durch den Wachskörper und ragt etwa 7 mm über das Wachs. Die Flamme ist 22 mm hoch und an der breitesten Stelle, auf einem Drittel der Höhe, 8 mm breit und beginnt 4 mm über dem Wachs. Ein Teelicht brennt etwa 4 Stunden, das sind 3,2 g Wachs je Stunde oder 0,9 mg je Sekunde; bei einem Heizwert von Paraffin von 42 bis 46 MJ/kg sind das 38 bis 41 W, von denen etwa 85 Prozent als konvektive Fahne und Strahlung die Flamme verlassen; die Pose brennend nimmt 35 W und das Flackermaximum 40 W (Schätzwert aus diesem Bereich). Die Pose unbrennend verbirgt die Flamme in Wachs und Docht, indem sie die Flammenknochen absenkt, sodass ein Asset alle Zustände zeigt. Der Ursprung ist die Mitte des Bodens auf der Unterlage; jeder Knochen läuft die Achse der Kerze hinauf.",
    )
    cup, wax, wick, flame, body, tip = (bone(asset, id) for id in ("cup", "wax", "wick", "flame", "flame-body", "flame-tip"))
    cup.update({"x": 0, "y": 0, "angleDeg": 90, "lengthM": 0.016})
    cup["mass"].update({"comX": 0.007, "comY": 0})
    wax.update({"x": 0.0005, "y": 0, "angleDeg": 0, "lengthM": 0.014})
    wax["mass"].update({"comX": 0.007, "comY": 0})
    wick.update({"x": 0.014, "y": 0, "angleDeg": 0, "lengthM": 0.007})
    wick["mass"].update({"comX": -0.011, "comY": 0})
    flame.update({"x": 0.004, "y": 0, "angleDeg": 0, "lengthM": 0})
    flame["title"] = text("Flame lift (hides the flame in the wax)", "Flammenhub (verbirgt die Flamme im Wachs)")
    flame["joint"] = {"type": "prismatic", "axisDeg": 0, "minM": -0.01775, "maxM": 0.003}
    body.update({"x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.0135})
    tip.update({"x": 0.008, "y": 0, "angleDeg": 0, "lengthM": 0.014})
    tip["joint"] = {"type": "prismatic", "axisDeg": 0, "minM": -0.006, "maxM": 0.006}
    contact, top = (next(item for item in asset["anchors"] if item["id"] == id) for id in ("base-contact", "flame-top"))
    contact.update({"footprintLengthM": 0.034, "footprintWidthM": 0.034})
    contact["title"] = text("Standing ring of the cup on the surface", "Standring des Schälchens auf der Unterlage")
    top.update({"x": 0.014, "y": 0})
    for pose in asset["poses"]:
        if pose["id"] == "unlit":
            pose["bones"] = {"flame": {"dx": -0.01775}, "flame-tip": {"dx": -0.00575}}
        if pose["id"] == "flicker-peak":
            pose["bones"] = {"flame": {"dx": 0.002}, "flame-body": {"angleDeg": -10}, "flame-tip": {"dx": 0.004}}
    for clip in asset["clips"]:
        for key in clip["keys"]:
            offset = key.get("bones", {}).get("flame-tip")
            if offset and "dy" in offset:
                offset["dx"] = offset.pop("dy")
    representations = asset["representations"]
    redraw(representations["50"], FLAME_2D_50, TIP_2D_50)
    redraw(representations["100"], FLAME_2D_100, TIP_2D_100)
    redraw(representations["200"], FLAME_2D_200, None)
    representations["200"]["pens"]["body"] = 0.18
    for scale, decisions in DECISIONS_2D.items():
        representations[scale]["decisions"].update(decisions)
    regions = {item["id"]: item for item in asset["thermal"]["regions"]}
    regions["cup"]["areaM2"] = 0.0056
    regions["wick"].update({"areaM2": 4.7e-05, "characteristicLengthM": 0.007})
    asset["structure"]["assumptions"] = [
        text(
            "The 14 g candle stands on the embossed standing ring of its cup: a unilateral annular contact of 28 to 34 mm carries the weight.",
            "Die 14 g schwere Kerze steht auf dem geprägten Standring ihres Schälchens: ein einseitiger Ringkontakt von 28 bis 34 mm trägt das Gewicht.",
        )
    ]
    return asset


def read(path):
    with open(path, encoding="utf-8") as file:
        return json.load(file)


def write(path, document):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as file:
        file.write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else ""
    if mode == "volumes":
        write(SOURCE, {"volumes": volumes()})
    elif mode == "candidate":
        document = read(CATALOG)
        roots(document["asset"])
        document["asset"]["volumes"] = volumes()
        write(CANDIDATE, document)
    elif mode == "catalog":
        document = read(CATALOG)
        before = copy.deepcopy(document["asset"]["volumes"])
        roots(document["asset"])
        assert document["asset"]["volumes"] == before
        write(CATALOG, document)
    else:
        raise SystemExit(__doc__)


main()
