"""🌬️ Round-3 handcrafted volumes of appliances/ceiling-diffuser: every section stated by hand, written to volumes/appliances/ceiling-diffuser.json.

Construction (1:50, metres, y up, ceiling underside y = 0, square half-widths):
a 35.5 mm border frame (0.2975 -> 0.262) screwed to the ceiling, a 0.8 mm back plate closing the plenum under the ceiling with the
DN 250 spigot (0.7 mm, swaged bead for the clamp band) through it, and three nested square cones below: each annular cone is a
0.8 mm pressed sheet with a 45 degree rim that flares down and out to its lip and a shallow cone rising inwards; the centre cone is a
closed dish. Corner spacers carry the cones; the air leaves through the 10, 8 and 8 mm slots between the elements and turns along the ceiling.
Sources: Titus D-TMS-1.0 (https://www.titus-hvac.com/file/287/TMS.pdf), CAS AQN, Koolair 50-FR-4 (see r3-appliances-ceiling-diffuser.md).
"""
import json
import math
from pathlib import Path

T = Path(__file__).resolve().parent
OUT = T / "volumes" / "appliances" / "ceiling-diffuser.json"
SHEET = 0.0008
DOWN = [0.5, 0.5, 0.5, -0.5]


def r6(v):
    return round(v + 0.0, 6)


def rsq(a, rc, n=4):
    rc = min(rc, a * 0.5)
    pts = []
    for cx, cz, start in ((a - rc, a - rc, 0.0), (-(a - rc), a - rc, 90.0), (-(a - rc), -(a - rc), 180.0), (a - rc, -(a - rc), 270.0)):
        for k in range(n + 1):
            t = math.radians(start + 90.0 * k / n)
            pts.append([r6(cx + rc * math.cos(t)), r6(cz + rc * math.sin(t))])
    return pts


def circle(r, n=48):
    return [[r6(r * math.cos(2 * math.pi * k / n)), r6(r * math.sin(2 * math.pi * k / n))] for k in range(n)]


def base(id, material, role, tone):
    return {"id": id, "kind": None, "bone": "body", "material": material, "role": role, "tone": tone}


def plate(id, a, rc, hole, y0, y1, material="powder-coated-steel", role="body", tone=0.88, taper=None, faces=None):
    s = base(id, material, role, tone)
    s.update({"kind": "extrude", "profile": rsq(a, rc), "holes": [hole], "plane": "zx", "from": r6(y0), "to": r6(y1)})
    if taper:
        s["taperTo"] = [1, r6(taper)]
    if faces:
        s["sectionFaces"] = faces
    return s


def cone_ring(id, lip, ylip, rim, yin, rin, tone, rc=0.012):
    """⛰️ Annular pressed cone as two tapered 0.8 mm rings: the 45 degree rim from the lip up to its fold and the shallow cone inwards."""
    w_rim = SHEET * math.sqrt(2)
    top = lip - rim
    a = plate(id + "-rim", lip, rc, rsq(lip - w_rim, rc - w_rim), ylip, ylip + rim, tone=tone, taper=top / lip)
    rise = yin - (ylip + rim)
    w_cone = SHEET / math.sin(math.atan2(rise, top - rin)) * 2 / (1 + rin / top)
    rc2 = rc * top / lip
    b = plate(id + "-cone", top, rc2, rsq(top - w_cone, max(rc2 - w_cone, 0.0015)), ylip + rim, yin, tone=tone, taper=rin / top)
    return [a, b]


def cone_y(r, outer, yo, inner, yi):
    return yo + (outer - r) / (outer - inner) * (yi - yo)


def spacer(id, d, ylo, yhi, size=0.008, tone=0.6):
    s = base(id, "powder-coated-steel", "detail", tone)
    s.update({"kind": "box", "size": [size, r6(yhi - ylo), size], "transform": {"translation": [r6(d), r6((ylo + yhi) / 2), r6(d)]},
              "repeat": {"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": r6(math.pi / 2), "origin": [0, 0, 0]}})
    return s


def loft(id, sections, tone, material="powder-coated-steel", role="body", cap=(True, True)):
    s = base(id, material, role, tone)
    s.update({"kind": "loft", "sections": [{"at": [0, r6(y), 0], "rx": r6(r), "ry": r6(r), "exponent": e, "rotation": DOWN} for y, r, e in sections],
              "capStart": cap[0], "capEnd": cap[1]})
    return s


def revolve(id, profile, material, role, tone, origin=(0, 0, 0), axis=(0, 1, 0)):
    s = base(id, material, role, tone)
    s.update({"kind": "revolve", "profile": [[r6(a), r6(b)] for a, b in profile], "axis": list(axis), "origin": list(origin)})
    return s


def ref(id, of, **extra):
    return {"id": id, "of": of, **extra}


def emitter(id, x, y, sx, sy, sz):
    s = base(id, "powder-coated-steel", "detail", 0.3)
    s.update({"kind": "box", "size": [sx, sy, sz], "transform": {"translation": [r6(x), r6(y), 0]}, "region": "plate", "tags": [id]})
    return s


def support(anchor, kind, x, size, ylo, yhi):
    s = base("support-" + anchor, "galvanised-steel", "detail", 0.25)
    r = size / 2
    if kind == "pin":
        s.update({"kind": "revolve", "profile": [[r6(ylo), 0], [r6(ylo), r6(r)], [r6(yhi), r6(r)], [r6(yhi), 0]], "axis": [0, 1, 0], "origin": [r6(x), 0, 0]})
    else:
        s.update({"kind": "revolve", "profile": [[0, r6(r)], [r6(2 * size), r6(r)]], "axis": [0, 0, 1], "origin": [r6(x), r6(r), r6(-size)]})
    s.update({"member": "body", "part": "support", "anchor": anchor, "tags": [anchor]})
    return s


def text(en, de):
    return {"en": en, "de": de}


def collar_tube(r_out, wall, bead):
    r_in = r_out - wall
    if bead:
        pts = [(-0.004, r_out), (0.021, r_out), (0.023, r_out + 0.0027), (0.029, r_out + 0.0027), (0.031, r_out), (0.05, r_out),
               (0.05, r_in), (0.031, r_in), (0.029, r_in + 0.0027), (0.023, r_in + 0.0027), (0.021, r_in), (-0.004, r_in)]
    else:
        pts = [(-0.002, r_out), (0.05, r_out), (0.05, r_in), (-0.002, r_in)]
    return pts + [pts[0]]


def scale50():
    L1, Y1, L2, Y2, L3, Y3 = 0.265, -0.032, 0.2, -0.050, 0.135, -0.07
    R1, R2, R3 = 0.010, 0.010, 0.012
    I1, YI1, I2, YI2 = 0.075, -0.007, 0.06, -0.0324
    BACK = 0.0075
    body = [
        revolve("collar", collar_tube(0.1245, 0.0007, True), "galvanised-steel", "body", 0.5),
        plate("flange", 0.2975, 0.006, rsq(0.262, 0.002), -0.012, 0.0, faces=["+v"]),
        plate("backplate", 0.2625, 0.0025, circle(0.1245), 0.0, BACK, tone=0.84),
        *cone_ring("cone-1", L1, Y1, R1, YI1, I1, 0.8),
        *cone_ring("cone-2", L2, Y2, R2, YI2, I2, 0.72),
        loft("cone-3", [(Y3, L3, 12), (Y3 + R3, L3 - R3, 12), (-0.040, 0.006, 4)], 0.66),
    ]
    rc1 = 0.012 * (L1 - R1) / L1
    rc2 = 0.012 * (L2 - R2) / L2
    d1, d2, d3 = 0.13, 0.085, 0.068
    y1lo = cone_y(d1 + 0.293 * rc1, L1 - R1, Y1 + R1, I1, YI1)
    y2hi = cone_y(d2 + 0.293 * rc1, L1 - R1, Y1 + R1, I1, YI1) - SHEET * 1.02
    y2lo = cone_y(d2 + 0.293 * rc2, L2 - R2, Y2 + R2, I2, YI2)
    y3hi = cone_y(d3 + 0.293 * rc2, L2 - R2, Y2 + R2, I2, YI2) - SHEET * 1.02
    y3lo = Y3 + R3 + (L3 - R3 - (d3 + 0.293 * 0.008)) / (L3 - R3 - 0.006) * (-0.040 - Y3 - R3)
    body += [
        spacer("spacer-1", d1, y1lo - 0.0004, 0.0004),
        spacer("spacer-2", d2, y2lo - 0.0004, y2hi + 0.0004),
        spacer("spacer-3", d3, y3lo - 0.0006, y3hi + 0.0004),
    ]
    screw = base("screws", "powder-coated-steel", "detail", 0.82)
    screw.update({"kind": "ellipsoid", "radii": [0.006, 0.002, 0.006], "transform": {"translation": [0.28, -0.0115, 0.28]}, "texture": True,
                  "repeat": {"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": r6(math.pi / 2), "origin": [0, 0, 0]}})
    body.append(screw)
    plate_parts = ["flange", "backplate", "cone-1-rim", "cone-1-cone", "cone-2-rim", "cone-2-cone", "cone-3", "spacer-1", "spacer-2", "spacer-3", "screws"]
    thermal = [ref("th-" + p, p, region="plate", layer="sheet") for p in plate_parts]
    thermal.append(ref("th-collar", "collar", region="collar", layer="tube"))
    thermal += [emitter("jet-left", -0.2585, -0.017, 0.0075, 0.009, 0.46), emitter("jet-right", 0.2585, -0.017, 0.0075, 0.009, 0.46)]
    joint = base("joint-body", "galvanised-steel", "detail", 0.4)
    joint.update({"kind": "ellipsoid", "radii": [0.012, 0.0045, 0.012], "member": "body", "part": "joint"})
    mass = base("mass-body", "powder-coated-steel", "detail", 0.15)
    mass.update({"kind": "ellipsoid", "radii": [0.03, 0.004, 0.03], "transform": {"translation": [0, -0.02, 0]}, "member": "body", "part": "mass"})
    structure = [ref("member-" + p, p, member="body", part="member") for p in ["flange", "backplate", "spacer-1", "cone-1-cone", "spacer-2", "cone-2-cone", "spacer-3"]]
    structure += [joint, mass, support("ceiling-hanger-left", "pin", -0.28, 0.008, -0.012, 0.03), support("ceiling-hanger-right", "roller", 0.28, 0.008, 0, 0)]
    decisions = [
        text("Own model for 1:50 of a real surface-mounted four-way multi-cone diffuser (Titus TMS class, see the report): a 35.5 mm border frame (0.595 to 0.524 m, 12 mm deep) screwed to the ceiling with four flat lens-head screws (surface texture), the 0.8 mm back plate that closes the plenum, set into the 0.525 m ceiling cutout and modelled at the 7.5 mm smallest feature, the DN 250 galvanised spigot of 0.7 mm with its swaged bead for the clamp band, and three nested square cones below. Each annular cone is a 0.8 mm pressed sheet: a 10 mm rim at 45 degrees that flares down and out to its lip (0.265 and 0.2 m half width) and a shallow cone rising inwards to openings of 0.15 and 0.12 m; the centre cone is a closed dish 0.27 m square that rises to a point under the spigot. Four corner spacers carry each cone from the one above. The slots of 10, 8 and 8 mm between border and cones stay open, so the stepped face reads with its dark slot under every step.",
             "Eigenes Modell für 1:50 eines echten aufgesetzten Vierwege-Mehrkegelauslasses (Klasse Titus TMS, siehe Bericht): ein 35,5 mm breiter Randrahmen (0,595 auf 0,524 m, 12 mm tief), mit vier flachen Linsenkopfschrauben an der Decke verschraubt (Oberflächentextur), die 0,8-mm-Rückplatte, die den Plenumraum schließt, im Deckenausschnitt von 0,525 m sitzt und mit dem kleinsten Merkmal von 7,5 mm modelliert ist, der verzinkte DN-250-Stutzen von 0,7 mm mit eingedrückter Sicke für das Spannband, und darunter drei ineinanderliegende quadratische Kegel. Jeder Ringkegel ist ein gepresstes Blech von 0,8 mm: ein 10-mm-Rand unter 45 Grad, der sich nach unten und außen zur Lippe aufweitet (Halbbreite 0,265 und 0,2 m), und ein flacher Kegel, der nach innen bis zu Öffnungen von 0,15 und 0,12 m ansteigt; der Mittelkegel ist eine geschlossene Schale von 0,27 m im Quadrat, die unter dem Stutzen zu einer Spitze ansteigt. Vier Eckabstandshalter tragen jeden Kegel vom darüberliegenden. Die Schlitze von 10, 8 und 8 mm zwischen Rahmen und Kegeln bleiben offen, so liest sich die Stufenfront mit ihrem dunklen Schlitz unter jeder Stufe."),
        text("Thermal: the plate region is the honest sheet metal of border, back plate, cones, spacers and screw heads (references to the body solids); the plenum is open to the room through the slots, so both faces of every sheet count (C29, C31), the top of the border frame lies against the ceiling and is marked as a mounting face. The collar region is the spigot tube itself. The two jets are outlet patches in the mouth of the outer slot under the border, left and right, where the air leaves along the ceiling (D2). The single layers of 0.8 mm and 0.7 mm are the sheets themselves.",
             "Thermik: die Region Platte ist das ehrliche Blech von Rahmen, Rückplatte, Kegeln, Abstandshaltern und Schraubenköpfen (Verweise auf die Körper); der Plenumraum ist über die Schlitze zum Raum offen, deshalb zählen beide Seiten jedes Blechs (C29, C31), die Oberseite des Randrahmens liegt an der Decke und ist als Montagefläche markiert. Die Region Kragen ist das Stutzenrohr selbst. Die zwei Strahlen sind Austrittsflächen in der Mündung des äußeren Schlitzes unter dem Rahmen, links und rechts, wo die Luft an der Decke entlang austritt (D2). Die einzelnen Schichten von 0,8 mm und 0,7 mm sind die Bleche selbst."),
        text("Structure: the real load path. The two ceiling hangers (pin left as an 8 mm hanger stud through the border into the ceiling, roller right as an 8 mm roller on the frame against the ceiling) carry the border frame, the frame carries the back plate, and the cone pack hangs from the back plate through three tiers of corner spacers and the shallow cones of cone 1 and cone 2; the rims, the centre dish, the spigot and the screws are carried, not carrying. A fixed joint at the rig joint (centre of the spigot at the ceiling plane) and one compact mass at the centre of mass 20 mm below the ceiling.",
             "Tragwerk: der wirkliche Lastweg. Die zwei Deckenhalter (links ein Festlager als 8-mm-Gewindestift durch den Rahmen in die Decke, rechts ein Rollenlager als 8-mm-Rolle auf dem Rahmen an der Decke) tragen den Randrahmen, der Rahmen trägt die Rückplatte, und das Kegelpaket hängt über drei Lagen Eckabstandshalter und die flachen Kegel von Kegel 1 und Kegel 2 an der Rückplatte; Ränder, Mittelschale, Stutzen und Schrauben werden getragen und tragen nicht. Ein festes Gelenk am Riggelenk (Mitte des Stutzens in der Deckenebene) und eine kompakte Masse im Schwerpunkt 20 mm unter der Decke."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale100():
    body = [
        revolve("collar", collar_tube(0.1245, 0.0007, False), "galvanised-steel", "body", 0.5),
        plate("flange", 0.2975, 0.006, circle(0.1245), -0.015, 0.0, faces=["+v"]),
        loft("cone-1", [(-0.045, 0.262, 12), (-0.030, 0.247, 12), (-0.015, 0.19, 12)], 0.78),
        loft("cone-2", [(-0.070, 0.17, 12), (-0.060, 0.16, 12), (-0.045, 0.11, 12)], 0.68),
    ]
    thermal = [ref("th-flange", "flange", region="plate", layer="sheet"), ref("th-cone-1", "cone-1", region="plate", layer="sheet"),
               ref("th-cone-2", "cone-2", region="plate", layer="sheet"), ref("th-collar", "collar", region="collar", layer="tube"),
               emitter("jet-left", -0.235, -0.0225, 0.015, 0.015, 0.40), emitter("jet-right", 0.235, -0.0225, 0.015, 0.015, 0.40)]
    posts = base("member-spacers", "powder-coated-steel", "detail", 0.6)
    posts.update({"kind": "box", "size": [0.015, 0.045, 0.015], "transform": {"translation": [0.15, -0.0375, 0.15]},
                  "repeat": {"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": r6(math.pi / 2), "origin": [0, 0, 0]}, "member": "body", "part": "member"})
    joint = base("joint-body", "galvanised-steel", "detail", 0.4)
    joint.update({"kind": "ellipsoid", "radii": [0.0075, 0.0075, 0.0075], "transform": {"translation": [0, -0.0075, 0]}, "member": "body", "part": "joint"})
    mass = base("mass-body", "powder-coated-steel", "detail", 0.15)
    mass.update({"kind": "ellipsoid", "radii": [0.04, 0.0075, 0.04], "transform": {"translation": [0, -0.02, 0]}, "member": "body", "part": "mass"})
    structure = [ref("member-flange", "flange", member="body", part="member"), posts, joint, mass,
                 support("ceiling-hanger-left", "pin", -0.28, 0.015, -0.015, 0.03), support("ceiling-hanger-right", "roller", 0.28, 0.015, 0, 0)]
    decisions = [
        text("Own model for 1:100 against the 1:100 drawings: border frame and back plate merge into one 15 mm flange plate (12 mm frame drawn at the 15 mm feature) with the spigot hole; the three cones become two solid stepped cones, each with its 45 degree rim flaring down and out to the lip (0.262 and 0.17 m half width) and a 15 mm slot notch above it where the sheet rises inwards, so the stepped face with its two dark slots still reads. The spigot keeps its 0.7 mm wall (C30); bead, screws, spacers and the third slot fall under 15 mm.",
             "Eigenes Modell für 1:100 nach den 1:100-Zeichnungen: Randrahmen und Rückplatte werden zu einer Flanschplatte von 15 mm (der 12-mm-Rahmen auf das 15-mm-Merkmal gezeichnet) mit dem Stutzenloch; aus drei Kegeln werden zwei volle Kegelstufen, jede mit ihrem 45-Grad-Rand, der sich nach unten und außen zur Lippe aufweitet (Halbbreite 0,262 und 0,17 m), und darüber einer 15-mm-Schlitzkerbe, wo das Blech nach innen ansteigt, so dass die Stufenfront mit ihren zwei dunklen Schlitzen lesbar bleibt. Der Stutzen behält seine Wand von 0,7 mm (C30); Sicke, Schrauben, Abstandshalter und der dritte Schlitz fallen unter 15 mm."),
        text("Thermal: references to flange, both cones and the spigot; the cone pack is closed at this scale, so only its outer faces count. The jets are outlet patches in the outer slot notch, left and right.",
             "Thermik: Verweise auf Flansch, beide Kegel und den Stutzen; das Kegelpaket ist in diesem Maßstab geschlossen, deshalb zählen nur seine Außenflächen. Die Strahlen sind Austrittsflächen in der äußeren Schlitzkerbe, links und rechts."),
        text("Structure: the flange plate carries the load to the two hangers (pin stud and roller at the 15 mm feature); the three tiers of corner spacers merge into four 15 mm corner posts that carry the cone pack; a fixed joint at the rig joint and one mass at the centre of mass.",
             "Tragwerk: die Flanschplatte trägt die Last zu den zwei Haltern (Gewindestift und Rolle auf dem 15-mm-Merkmal); die drei Lagen Eckabstandshalter werden zu vier Eckstützen von 15 mm, die das Kegelpaket tragen; ein festes Gelenk am Riggelenk und eine Masse im Schwerpunkt."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    collar = base("collar", "galvanised-steel", "body", 0.5)
    collar.update({"kind": "revolve", "profile": [[-0.002, 0], [-0.002, 0.1245], [0.05, 0.1245], [0.05, 0]], "axis": [0, 1, 0], "origin": [0, 0, 0]})
    body = [
        collar,
        plate("flange", 0.2975, 0.006, circle(0.1245), -0.03, 0.0, faces=["+v"]),
        loft("cone", [(-0.07, 0.135, 12), (-0.03, 0.262, 12)], 0.72),
    ]
    thermal = [ref("th-flange", "flange", region="plate", layer="sheet"), ref("th-cone", "cone", region="plate", layer="sheet"),
               ref("th-collar", "collar", region="collar", layer="tube"),
               emitter("jet-left", -0.25, -0.045, 0.03, 0.03, 0.40), emitter("jet-right", 0.25, -0.045, 0.03, 0.03, 0.40)]
    joint = base("joint-body", "galvanised-steel", "detail", 0.4)
    joint.update({"kind": "ellipsoid", "radii": [0.015, 0.015, 0.015], "transform": {"translation": [0, -0.015, 0]}, "member": "body", "part": "joint"})
    mass = base("mass-body", "powder-coated-steel", "detail", 0.15)
    mass.update({"kind": "ellipsoid", "radii": [0.06, 0.015, 0.06], "transform": {"translation": [0, -0.02, 0]}, "member": "body", "part": "mass"})
    structure = [ref("member-flange", "flange", member="body", part="member"), joint, mass,
                 support("ceiling-hanger-left", "pin", -0.28, 0.03, -0.03, 0.03), support("ceiling-hanger-right", "roller", 0.28, 0.03, 0, 0)]
    decisions = [
        text("Own model for 1:200: a 30 mm flange plate (border, back plate and slots at the 30 mm feature), one cone mass that generalises the three steps to their envelope from 0.262 m half width under the flange to the 0.27 m square centre cone 70 mm below the ceiling, and the spigot as a full cylinder, as the plan draws it. A minimal model of three solids that keeps the 0.595 x 0.12 m outline and the stepped taper.",
             "Eigenes Modell für 1:200: eine Flanschplatte von 30 mm (Rahmen, Rückplatte und Schlitze auf dem 30-mm-Merkmal), eine Kegelmasse, die die drei Stufen zu ihrer Hülle verallgemeinert, von 0,262 m Halbbreite unter dem Flansch bis zum 0,27 m breiten Mittelkegel 70 mm unter der Decke, und der Stutzen als voller Zylinder, wie ihn der Grundriss zeichnet. Ein minimales Modell aus drei Körpern, das den Umriss von 0,595 x 0,12 m und die gestufte Verjüngung behält."),
        text("Thermal: references to flange, cone and spigot; the jets are 30 mm outlet patches where the cone leaves the flange underside. Structure: the flange plate carries the whole load to the pin and the roller (30 mm); spacers are merged into the carried cone mass, so one joint at the rig joint and one mass at the centre of mass remain.",
             "Thermik: Verweise auf Flansch, Kegel und Stutzen; die Strahlen sind 30-mm-Austrittsflächen, wo der Kegel die Flanschunterseite verlässt. Tragwerk: die Flanschplatte trägt die ganze Last zum Festlager und zur Rolle (30 mm); die Abstandshalter gehen in der getragenen Kegelmasse auf, so bleiben ein Gelenk am Riggelenk und eine Masse im Schwerpunkt."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


volumes = {"50": scale50(), "100": scale100(), "200": scale200(), "500": {"status": "unavailable", "denominator": 500}}
OUT.write_text(json.dumps({"volumes": volumes}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("[DEBUG] wrote", OUT)
