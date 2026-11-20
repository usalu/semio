"""🧊 Round 3, machinery/expansion-valve: every solid of every authored scale (1:50, 1:100) and list (body, thermal, structure), stated by hand in world millimetres and written in the frame of its bone.

Usage: imported by r3-expansion-valve-build.py (`volumes()`).
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_expansion_valve_kit", os.path.join(HERE, "r3-expansion-valve-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
m = K.m

BRASS, COPPER, STEEL = "brass", "copper", "stainless-steel"
UP = (0.0, 1.0, 0.0)
AHEAD = (1.0, 0.0, 0.0)


def revolve(sid, bone, material, tone, origin, axis, profile, role="body", **extra):
    """🌀 A revolve about a world axis through a world point; profile [axial, radial] in mm."""
    return {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[m(a), m(r)] for a, r in profile], "axis": K.direction(bone, axis), "origin": K.local(bone, origin), **extra}


def along(sid, bone, material, tone, profile, origin=(0.0, 0.0, 0.0), role="body", **extra):
    """➡️ A revolve about the bone's own axis u through a bone-local point (mm); profile [u, radial] in mm."""
    return {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[m(a), m(r)] for a, r in profile], "axis": [1.0, 0.0, 0.0], "origin": [m(c) for c in origin], **extra}


def loft(sid, bone, material, tone, sections, role="body", **extra):
    """🪢 A loft through world sections (x, y, z, rx, ry, exponent)."""
    out = []
    for x, y, z, rx, ry, n in sections:
        s = {"at": K.local(bone, (x, y, z)), "rx": m(rx), "ry": m(ry)}
        if n != 2.0:
            s["exponent"] = n
        out.append(s)
    return {"id": sid, "kind": "loft", "bone": bone, "material": material, "role": role, "tone": tone, "sections": out, "capStart": True, "capEnd": True, **extra}


def tube(sid, bone, material, tone, pts, role="detail"):
    """🧵 A slender round tube of the capillary radius through world points."""
    return loft(sid, bone, material, tone, [(x, y, z, K.CAP_R, K.CAP_R, 2.0) for x, y, z in pts], role=role, slender=True)


def extrude(sid, bone, material, tone, plane, profile, lo, hi, holes=None, role="body", **extra):
    """🧱 An extrude of a bone-local profile (mm) on a coordinate plane between two bone-local coordinates (mm)."""
    solid = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[m(a), m(b)] for a, b in profile]}
    if holes:
        solid["holes"] = [[[m(a), m(b)] for a, b in hole] for hole in holes]
    return {**solid, "plane": plane, "from": m(lo), "to": m(hi), **extra}


def ellipsoid(sid, bone, material, tone, centre, radii, **extra):
    """🥚 An ellipsoid at a world point (mm) with radii (mm) along the bone axes."""
    return {"id": sid, "kind": "ellipsoid", "bone": bone, "material": material, "role": "body", "tone": tone, "radii": [m(r) for r in radii], "transform": {"translation": K.local(bone, centre)}, **extra}


def ref(sid, of, **extra):
    """🔗 A reference to a body solid of the same scale."""
    return {"id": sid, "of": of, **extra}


def body50():
    """🔩 1:50 body: every assembly of the installed valve."""
    oy = K.OUTLET_Y
    nut = [(z, x) for x, z in K.hexagon(K.NUT_AF)]
    return [
        revolve("liquid-line", "body", COPPER, 0.4, (0, 0, 0), UP, [(K.CUT + 3, 0), (K.CUT + 3, 3.96), (K.CUT, 3.96), (K.CUT, 4.76), (56, 4.76), (58, 6.3), (58, 0)], sectionFaces=["-v"]),
        revolve("nut", "body", BRASS, 0.5, (0, 0, 0), UP, [(50, 4.9), (50, 6.3), (51, 6.8), (55, 9.0), (55.6, 9.3), (67.4, 9.3), (68, 8.9), (68, 7.95), (59, 7.95), (57, 4.9), (50, 4.9)]),
        extrude("nut-hex", "body", BRASS, 0.5, "zx", nut, 55.6 - oy, 67.4 - oy, holes=[[(z, x) for x, z in K.ring(7.95, 20)]]),
        revolve("inlet-thread", "body", BRASS, 0.5, (0, 0, 0), UP, [(58, 0), (58, 5.5), (60, 7.95), (70.5, 7.95), (70.5, 0)]),
        loft("body", "body", BRASS, 0.5, [(0, y, 0, r, r, n) for y, r, n in K.BARREL]),
        revolve("outlet-boss", "body", BRASS, 0.5, (0, oy, 0), AHEAD, [(8, 0), (8, 8.6), (15.5, 8.6), (17, 7.4), (17, 0)]),
        revolve("outlet-stub", "body", COPPER, 0.4, (0, oy, 0), AHEAD, [(16, 0), (16, K.STUB_R), (42, K.STUB_R), (44.5, K.BELL_R), (54, K.BELL_R), (54, K.OUTLET_R), (46, K.OUTLET_R), (46, 0)]),
        revolve("outlet-pipe", "body", COPPER, 0.4, (0, oy, 0), AHEAD, [(46, 0), (46, K.OUTLET_R), (K.OUTLET_END, K.OUTLET_R), (K.OUTLET_END, 5.0), (K.OUTLET_END - 3, 5.0), (K.OUTLET_END - 3, 0)], sectionFaces=["+u"]),
        along("head", "head", STEEL, 0.55, [(-1.5, 0)] + K.HEAD + [(15.0, 0)]),
        tube("capillary-riser", "head", STEEL, 0.1, K.riser_path()),
        tube("capillary-coil", "head", STEEL, 0.1, K.coil_path()),
        tube("capillary", "head", STEEL, 0.1, K.lead_path()),
        along("stem-boss", "stem", BRASS, 0.5, [(-6, 0), (-6, 7), (19.5, 7), (19.5, 8.2), (20.8, 8.2), (22, 7), (22, 0)]),
        extrude("stem", "stem", BRASS, 0.3, "yz", K.hexagon(K.STEM_AF), 4.0, 19.5),
        along("bulb", "bulb", STEEL, 0.5, [(0, 0)] + K.BULB + [(K.BULB_TIP, 0)]),
        along("suction-line", "bulb", COPPER, 0.4, [(-4, 0), (-4, 7), (-7, 7), (-7, 8), (57, 8), (57, 7), (54, 7), (54, 0)], origin=(0, K.BULB_Y - K.SUCTION_Y, 0), sectionFaces=["-u", "+u"]),
        extrude("bulb-strap", "bulb", STEEL, 0.2, "yz", K.strap_outline(0.5), 20.0, 30.0, holes=[K.strap_outline(0.0)], role="detail", texture=True),
        {"id": "strap-lock", "kind": "box", "bone": "bulb", "material": STEEL, "role": "detail", "tone": 0.2, "size": [m(9), m(6), m(3.6)], "cornerRadius": m(0.6), "transform": {"translation": [m(25), m(7.6), m(-9.7)]}, "texture": True},
        {"id": "strap-screw", "kind": "capsule", "bone": "bulb", "material": STEEL, "role": "detail", "tone": 0.2, "p0": [m(25), m(2.5), m(-10.0)], "p1": [m(25), m(13.5), m(-10.0)], "r0": m(1.6), "r1": m(1.6), "texture": True},
    ]


REGION50 = {
    "liquid-line": "inlet", "nut": "inlet", "nut-hex": "inlet", "inlet-thread": "body", "body": "body", "outlet-boss": "body",
    "outlet-stub": "outlet", "outlet-pipe": "outlet", "head": "head", "capillary-riser": "capillary", "capillary-coil": "capillary", "capillary": "capillary",
    "stem-boss": "body", "stem": "body", "bulb": "bulb", "suction-line": "suction-line", "bulb-strap": "suction-line", "strap-lock": "suction-line", "strap-screw": "suction-line",
}


def thermal_of(body, regions):
    """🌡️ One reference per body solid: every region is a single metal skin, so the thermal surface is the body surface; cut pipe ends stay section faces."""
    out = []
    for solid in body:
        entry = ref("th-" + solid["id"], solid["id"], region=regions[solid["id"]])
        if "sectionFaces" in solid:
            entry["sectionFaces"] = solid["sectionFaces"]
        out.append(entry)
    return out


def structure50():
    """🏗️ 1:50 structure: the pipework that carries the valve, the pressure shells, the joints, the three pipe supports and the masses."""
    oy = K.OUTLET_Y
    return [
        revolve("member-liquid-line", "body", COPPER, 0.4, (0, 0, 0), UP, [(K.CUT, 3.96), (K.CUT, 4.76), (58, 4.76), (58, 3.96), (K.CUT, 3.96)], member="body", part="member"),
        revolve("member-valve-body", "body", BRASS, 0.5, (0, 0, 0), UP, [(58, 4.0), (58, 8.0), (108, 8.0), (108, 4.0), (58, 4.0)], member="body", part="member"),
        revolve("member-outlet", "body", COPPER, 0.4, (0, oy, 0), AHEAD, [(4, 5.0), (4, 6.0), (K.OUTLET_END, 6.0), (K.OUTLET_END, 5.0), (4, 5.0)], member="body", part="member"),
        along("member-element", "head", STEEL, 0.55, [(-1.5, 6.0), (-1.5, 7.0), (2.2, 7.0), (2.8, 8.0), (6.0, 19.6), (6.6, 21.0), (9.6, 21.0), (9.6, 20.0), (7.0, 20.0), (6.6, 18.8), (3.6, 8.0), (3.0, 6.0), (-1.5, 6.0)], member="head", part="member"),
        along("member-spindle-housing", "stem", BRASS, 0.3, [(-6, 3.0), (-6, 6.0), (19.5, 6.0), (19.5, 3.0), (-6, 3.0)], member="stem", part="member"),
        along("member-suction-line", "bulb", COPPER, 0.4, [(-7, 7), (-7, 8), (57, 8), (57, 7), (-7, 7)], origin=(0, K.BULB_Y - K.SUCTION_Y, 0), member="bulb", part="member"),
        ref("joint-flare-nut", "nut-hex", member="body", part="joint"),
        revolve("joint-solder-socket", "body", COPPER, 0.4, (0, oy, 0), AHEAD, [(46, K.OUTLET_R), (46, K.BELL_R), (54, K.BELL_R), (54, K.OUTLET_R), (46, K.OUTLET_R)], member="body", part="joint"),
        along("joint-element-seat", "head", BRASS, 0.55, [(-3.75, 6.0), (-3.75, 8.5), (3.75, 8.5), (3.75, 6.0), (-3.75, 6.0)], member="head", part="joint"),
        along("joint-spindle-thread", "stem", BRASS, 0.3, [(-4, 4.0), (-4, 7.0), (4, 7.0), (4, 4.0), (-4, 4.0)], member="stem", part="joint"),
        ref("joint-bulb-strap", "bulb-strap", member="bulb", part="joint"),
        ellipsoid("support-liquid-line", "body", COPPER, 0.3, (0, K.CUT + 4.7, 0), (4.7, 4.7, 4.7), member="body", part="support", anchor="liquid-line"),
        revolve("support-outlet-pipe", "body", COPPER, 0.3, (K.OUTLET_END - 4, oy, 0), (0, 0, 1), [(-6, 0), (-6, 4), (6, 4), (6, 0)], member="body", part="support", anchor="outlet-pipe"),
        revolve("support-suction-line", "bulb", COPPER, 0.3, (K.SUCTION_X[0] + 4, K.SUCTION_Y, K.BULB_Z), (0, 0, 1), [(-6, 0), (-6, 4), (6, 4), (6, 0)], member="bulb", part="support", anchor="suction-line"),
        ellipsoid("mass-body", "body", BRASS, 0.2, (4.0, 86.0, 0), (5, 5, 5), member="body", part="mass"),
        ellipsoid("mass-element", "head", STEEL, 0.2, (0, 116.5, 0), (3.75, 4.5, 4.5), member="head", part="mass"),
        ellipsoid("mass-bulb", "bulb", STEEL, 0.2, (-82.0, K.BULB_Y - 3.0, K.BULB_Z), (4.5, 4, 4.5), member="bulb", part="mass"),
    ]


def body100():
    """🔩 1:100 body: eight masses and the capillary lead, authored for a 15 mm feature."""
    oy = K.OUTLET_Y
    return [
        revolve("inlet", "body", COPPER, 0.4, (0, 0, 0), UP, [(K.CUT, 0), (K.CUT, 7.5), (50, 7.5), (53, 11), (68, 11), (70, 9.5), (70, 0)], sectionFaces=["-v"]),
        revolve("body", "body", BRASS, 0.5, (0, 0, 0), UP, [(66, 0), (66, 11.5), (106, 11.5), (108.5, 8), (108.5, 0)]),
        revolve("outlet", "body", COPPER, 0.4, (0, oy, 0), AHEAD, [(8, 0), (8, 7.5), (K.OUTLET_END, 7.5), (K.OUTLET_END, 0)], sectionFaces=["+u"]),
        along("head", "head", STEEL, 0.55, [(-1.5, 0), (-1.5, 8), (2.5, 8), (6, 21), (10.5, 21), (13, 9), (17.5, 5), (17.5, 0)]),
        along("capillary-coil", "head", STEEL, 0.3, [(17, 0), (17, 29), (18, 30), (34, 30), (35, 29), (35, 0)]),
        tube("capillary", "head", STEEL, 0.1, K.lead_path(10), role="body"),
        along("stem", "stem", BRASS, 0.3, [(-6, 0), (-6, 8.5), (20.5, 8.5), (22, 7), (22, 0)]),
        along("bulb", "bulb", STEEL, 0.5, [(0, 0), (0, 5), (3, 7.5), (47, 7.5), (50, 5), (50, 0)]),
        along("suction-line", "bulb", COPPER, 0.4, [(-7, 0), (-7, 8), (57, 8), (57, 0)], origin=(0, K.BULB_Y - K.SUCTION_Y, 0), sectionFaces=["-u", "+u"]),
    ]


REGION100 = {"inlet": "inlet", "body": "body", "outlet": "outlet", "head": "head", "capillary-coil": "capillary", "capillary": "capillary", "stem": "body", "bulb": "bulb", "suction-line": "suction-line"}


def structure100():
    """🏗️ 1:100 structure: one column, the pipes and housings that are their own members, one joint, three supports, one mass."""
    oy = K.OUTLET_Y
    return [
        revolve("member-column", "body", BRASS, 0.5, (0, 0, 0), UP, [(K.CUT, 0), (K.CUT, 7.5), (108, 7.5), (108, 0)], member="body", part="member"),
        ref("member-outlet", "outlet", member="body", part="member"),
        ref("member-element", "head", member="head", part="member"),
        ref("member-spindle-housing", "stem", member="stem", part="member"),
        ref("member-suction-line", "suction-line", member="bulb", part="member"),
        ellipsoid("joint-body", "body", BRASS, 0.5, (0, oy, 0), (9, 9, 9), member="body", part="joint"),
        ellipsoid("support-liquid-line", "body", COPPER, 0.3, (0, K.CUT + 7.5, 0), (7.5, 7.5, 7.5), member="body", part="support", anchor="liquid-line"),
        ellipsoid("support-outlet-pipe", "body", COPPER, 0.3, (K.OUTLET_END - 7.5, oy, 0), (7.5, 7.5, 7.5), member="body", part="support", anchor="outlet-pipe"),
        ellipsoid("support-suction-line", "bulb", COPPER, 0.3, (K.SUCTION_X[0] + 7.5, K.SUCTION_Y, K.BULB_Z), (7.5, 7.5, 7.5), member="bulb", part="support", anchor="suction-line"),
        ellipsoid("mass-valve", "body", BRASS, 0.2, (0, 90.0, 0), (7.5, 7.5, 7.5), member="body", part="mass"),
    ]


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


DECISIONS50 = [
    text("Body: the valve is modelled part by part as it is built. The liquid line is a 9.5 mm copper tube with its 45 degree flare, held by a 3/8 in flare nut (tapered skirt and chamfer collar turned in one piece, 19 mm hexagon around it) on the threaded inlet of the forged brass body; the body is one loft from the round inlet neck into the rounded square forging and back to the round seat of the element; the outlet is a boss, a 12.7 mm copper stub with its swaged 12 mm socket and the pipe soldered into it; the setting spindle sits under a 17 mm hexagon cap on a boss inclined 30 degrees downwards; the thermostatic element is one turned profile with seat, lower housing, crimped rim, cap and capillary nipple.",
         "Körper: Das Ventil ist Teil für Teil so modelliert, wie es gebaut ist. Die Flüssigkeitsleitung ist ein 9,5-mm-Kupferrohr mit 45-Grad-Bördel, gehalten von einer 3/8-Zoll-Bördelmutter (konischer Schaft und Fasenbund aus einem Stück gedreht, 19-mm-Sechskant darum) auf dem Gewindeeingang des geschmiedeten Messingkörpers; der Körper ist ein Loft vom runden Eingangshals in das gerundete Vierkant-Schmiedeteil und zurück zum runden Sitz des Elements; der Ausgang besteht aus einem Auge, einem 12,7-mm-Kupferstutzen mit aufgeweiteter 12-mm-Muffe und dem eingelöteten Rohr; die Einstellspindel sitzt unter einer 17-mm-Sechskantkappe auf einem um 30 Grad abwärts geneigten Auge; das thermostatische Element ist ein gedrehtes Profil mit Sitz, Unterschale, Bördelrand, Deckel und Kapillarnippel."),
    text("Body: the 1.6 mm capillary keeps its real size and length (about 1.5 m) as three slender tubes: the riser from the nipple with a flat spiral, seven turns of 58 mm at a pitch of 2 mm for the surplus length, and the lead that leaves the lowest turn and enters the bulb along its axis. The 14.5 mm by 50 mm bulb with its charging nib lies on top of a 64 mm piece of the 16 mm suction line and is held by a 10 mm band with lock and screw; band, lock and screw are texture parts below the 7.5 mm printing limit.",
         "Körper: Die 1,6-mm-Kapillare behält ihre echte Größe und Länge (rund 1,5 m) als drei schlanke Rohre: das Steigstück vom Nippel mit flacher Spirale, sieben Windungen von 58 mm mit 2 mm Steigung für die Überlänge und die Zuleitung, die die unterste Windung verlässt und axial in den Fühler mündet. Der Fühler von 14,5 mm mal 50 mm mit Füllnippel liegt auf einem 64 mm langen Stück der 16-mm-Saugleitung und wird von einem 10-mm-Band mit Schloss und Schraube gehalten; Band, Schloss und Schraube sind Texturteile unter der Druckgrenze von 7,5 mm."),
    text("Thermal: every region is a single metal skin, so each thermal solid references the body solid that carries it: liquid line and flare nut are the inlet, stub and soldered pipe the outlet, forging, bosses and spindle cap the body, the element the head, the three capillary tubes the capillary, the bulb the bulb, the suction line with its band the suction line. The cut ends of the three pipes are section faces; there are no hidden regions and no emitters.",
         "Thermik: Jede Region ist eine einzelne Metallhaut, daher verweist jeder Thermikkörper auf den Körper, der sie trägt: Flüssigkeitsleitung und Bördelmutter sind der Eingang, Stutzen und eingelötetes Rohr der Ausgang, Schmiedeteil, Augen und Spindelkappe der Körper, das Element der Kopf, die drei Kapillarrohre die Kapillare, der Fühler der Fühler, die Saugleitung mit ihrem Band die Saugleitung. Die Schnittenden der drei Rohre sind Schnittflächen; verdeckte Regionen und Emitter gibt es nicht."),
    text("Structure: the valve hangs in its pipework. Members are the liquid line tube (9.5 mm by 0.8 mm), the pressure shell of the valve body, the outlet tube (12 mm by 1 mm), the lower housing shell of the element, the spindle housing and the suction line tube that carries the bulb; joints are the flare nut, the solder socket, the element seat, the spindle thread and the bulb band; a pin sits in the cut end of the liquid line, rollers in the cut ends of the outlet pipe and the suction line; masses sit at the centres of body, element and bulb.",
         "Tragwerk: Das Ventil hängt in seinen Rohren. Stäbe sind das Rohr der Flüssigkeitsleitung (9,5 mm mal 0,8 mm), die Druckschale des Ventilkörpers, das Ausgangsrohr (12 mm mal 1 mm), die Unterschale des Elements, das Spindelgehäuse und das Saugleitungsrohr, das den Fühler trägt; Gelenke sind die Bördelmutter, die Lötmuffe, der Elementsitz, das Spindelgewinde und das Fühlerband; ein Festlager sitzt im Schnittende der Flüssigkeitsleitung, Rollenlager in den Schnittenden von Ausgangsrohr und Saugleitung; Massen liegen in den Schwerpunkten von Körper, Element und Fühler."),
]

DECISIONS100 = [
    text("Body: at 1:100 nothing under 15 mm survives as its own part. Liquid line and flare nut are one stepped column drawn 15 mm thick, the forging is a round barrel of 23 mm, stub, socket and pipe are one outlet bar of 15 mm, the spindle boss and cap one inclined stub, the element a plain disc with a central boss, the seven capillary turns one disc of 60 mm resting on that boss, bulb and suction line two plain bars; band, lock, screw, nib, flare and chamfers are gone. Only the lead of the capillary keeps its true 1.6 mm as a slender tube, because it is what ties the bulb to the valve.",
         "Körper: Bei 1:100 bleibt nichts unter 15 mm als eigenes Teil. Flüssigkeitsleitung und Bördelmutter sind eine gestufte, 15 mm dick gezeichnete Säule, das Schmiedeteil ist ein runder Zylinder von 23 mm, Stutzen, Muffe und Rohr sind ein Ausgangsstab von 15 mm, Spindelauge und Kappe ein geneigter Stumpf, das Element eine einfache Scheibe mit Mittelnabe, die sieben Kapillarwindungen eine Scheibe von 60 mm auf dieser Nabe, Fühler und Saugleitung zwei einfache Stäbe; Band, Schloss, Schraube, Füllnippel, Bördel und Fasen entfallen. Nur die Zuleitung der Kapillare behält als schlankes Rohr ihre echten 1,6 mm, weil sie den Fühler mit dem Ventil verbindet."),
    text("Thermal: each region references its merged body mass; the capillary region is the coil disc and the lead.",
         "Thermik: Jede Region verweist auf ihre zusammengefasste Körpermasse; die Region Kapillare besteht aus der Windungsscheibe und der Zuleitung."),
    text("Structure: liquid line and valve body are one column of 15 mm; outlet bar, element disc, spindle stub and suction line bar are their own members; one joint marks the valve centre where the outlet and the spindle branch off; the pin and the two rollers sit in the three cut pipe ends; one mass stands for the whole 0.4 kg.",
         "Tragwerk: Flüssigkeitsleitung und Ventilkörper sind eine Säule von 15 mm; Ausgangsstab, Elementscheibe, Spindelstumpf und Saugleitungsstab sind ihre eigenen Stäbe; ein Gelenk markiert die Ventilmitte, an der Ausgang und Spindel abzweigen; das Festlager und die beiden Rollenlager sitzen in den drei Rohrschnittenden; eine Masse steht für die gesamten 0,4 kg."),
]


def volumes():
    """📦 The whole `volumes` member."""
    b50, b100 = body50(), body100()
    return {
        "50": {"status": "authored", "denominator": 50, "body": b50, "thermal": thermal_of(b50, REGION50), "structure": structure50(), "decisions": DECISIONS50},
        "100": {"status": "authored", "denominator": 100, "body": b100, "thermal": thermal_of(b100, REGION100), "structure": structure100(), "decisions": DECISIONS100},
        "200": {"status": "unavailable", "denominator": 200},
        "500": {"status": "unavailable", "denominator": 500},
    }
