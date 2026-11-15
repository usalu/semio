import sys, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_34_lib import *

RISE = 0.186667
RC = 0.0842
RT = 0.8
RB = 0.775
HR_R = 0.021
BAL_R = 0.006
RAIL_RISE = 2.613333 / (14 * math.radians(22.5))


def circle(r, n):
    return [[r6(r * math.cos(2 * math.pi * i / n)), r6(r * math.sin(2 * math.pi * i / n))] for i in range(n)]


def column(s, solid, wall, n):
    d = ext("column", "column", "steel", 0.5, "yz", circle(RC, n), 0, 2.8, holes=[circle(RC - wall, n)] if not solid else None)
    return s.b(d)


def plate(s, thick, bolts):
    holes = None
    if bolts:
        holes = [[[r6(0.14 * math.cos(a) + 0.011 * math.cos(b)), r6(0.14 * math.sin(a) + 0.011 * math.sin(b))] for b in [2 * math.pi * i / 16 for i in range(16)]] for a in [math.pi / 4 + k * math.pi / 2 for k in range(4)]]
    s.b(ext("base-plate", "column", "steel", 0.5, "yz", circle(0.2, 48), 0, thick, holes=holes))
    if bolts:
        for k in range(4):
            a = math.pi / 4 + k * math.pi / 2
            s.b(cyl("anchor-bolt-%d" % (k + 1), "column", "steel", 0.3, "x", [thick / 2 + 0.002, 0.14 * math.cos(a), 0.14 * math.sin(a)], thick - 0.004 + 0.0, 0.008, role="detail"))


def treads(s, tp, wl, n_arc, flange=True, depth=0.06):
    for k in range(1, 16):
        P = sector(k, n_arc)
        b = "tread-%d" % k
        if flange:
            s.b(ext("tread-top-%d" % k, b, "steel", 0.5, "zx", P, -tp, 0))
            s.b(ext("tread-lip-%d" % k, b, "steel", 0.5, "zx", P, -depth, -tp, holes=[inset_convex(P, wl)]))
        else:
            s.b(ext("tread-%d" % k, b, "steel", 0.5, "zx", P, -depth, 0))


def rail(s, n, r=HR_R, solid=True):
    pts = helix(RB, 1.12, RAIL_RISE, math.radians(11.25), n, math.radians(315) / (n - 1))
    h = tube_loft("handrail", "balustrade", "steel-bright", 0.3, pts, r, role="detail", slender=True)
    s.b(h)
    return h


def balusters(s, role="detail"):
    d = capsule("baluster", "balustrade", "steel-bright", 0.3, [0.76011, RISE, 0.15119], [0.76011, 1.12, 0.15119], BAL_R, role=role)
    d["repeat"] = {"count": 15, "rotationAxis": [0, 1, 0], "rotationRad": -0.3926991, "translation": [0, RISE, 0]}
    d["slender"] = True
    s.b(d)


def thermal_refs(s, tread_ids, col_ids, bal_ids):
    for i in tread_ids:
        s.thof("th-" + i, i, "treads")
    for i in col_ids:
        s.thof("th-" + i, i, "column-steel")
    for i in bal_ids:
        s.thof("th-" + i, i, "balustrade-steel")


def s50():
    s = Scale(50)
    column(s, False, 0.0063, 48)
    plate(s, 0.02, True)
    treads(s, 0.0075, 0.0075, 8)
    balusters(s)
    rail(s, 71)
    thermal_refs(s, ["tread-%s-%d" % (p, k) for k in range(1, 16) for p in ("top", "lip")], ["column", "base-plate"], ["baluster", "handrail"])
    s.stof("st-column", "column", "column", "member")
    for k in range(1, 16):
        s.stof("st-tread-top-%d" % k, "tread-top-%d" % k, "tread-%d" % k, "member")
        s.stof("st-tread-lip-%d" % k, "tread-lip-%d" % k, "tread-%d" % k, "member")
        phi = math.radians(11.25 + 22.5 * (k - 1))
        s.st(box("joint-tread-%d" % k, "tread-%d" % k, "steel", 0.3, [0.03, 0.045, 0.02], at=[0.105 * math.cos(phi), -0.03, 0.105 * math.sin(phi)], rot=quat_y(-math.degrees(phi)), role="detail"), "tread-%d" % k, "joint")
    s.stof("st-handrail", "handrail", "balustrade", "member")
    s.stof("st-baluster", "baluster", "balustrade", "member")
    s.st(revolve("support-base-fix", "column", "steel", 0.15, "x", [0, 0, 0], [[0, 0.2], [0.0075, 0.2]], role="detail"), "column", "support")
    s.st(cyl("support-top-tie", "column", "steel", 0.15, "z", [2.7925, 0, 0], 0.1, 0.0075, role="detail"), "column", "support")
    s.st(ellipsoid("mass-column", "column", "steel", 0.1, [0.06, 0.06, 0.06], at=[1.4, 0, 0], role="detail"), "column", "mass")
    s.st(ellipsoid("mass-balustrade", "balustrade", "steel", 0.1, [0.05, 0.05, 0.05], at=[0, 2.43, 0], role="detail"), "balustrade", "mass")
    s.dec("The 1:50 model is the helical stair as built: a CHS 168.3 x 6.3 column tube on a 20 mm base plate with four anchor bolts, fifteen folded steel treads of 22.5 degrees each on their own bones (a 7.5 mm plate plus a 52.5 mm hollow flange ring, the drawn 60 mm bar), fifteen 12 mm balusters and one 42 mm handrail swept along the true helix at 0.93 m above the tread. The sleeve joints between the treads stay texture of the column tube, they are not modelled.", "Das 1:50-Modell ist die Wendeltreppe wie gebaut: ein Rohr CHS 168,3 x 6,3 auf einer 20-mm-Fußplatte mit vier Ankerschrauben, fünfzehn gekantete Stahlstufen zu je 22,5 Grad auf eigenen Knochen (7,5-mm-Platte plus 52,5 mm hoher hohler Randring, der gezeichnete 60-mm-Balken), fünfzehn 12-mm-Stäbe und ein 42-mm-Handlauf entlang der wahren Wendel 0,93 m über der Stufe. Die Hülsenfugen zwischen den Stufen bleiben Textur des Säulenrohrs und sind nicht modelliert.")
    s.dec("Thermal: every region is the outer surface of its body solids by reference: treads = plate and flange ring of all fifteen pans (top, underside and both sides of the flange are exposed), column-steel = tube and base plate, balustrade-steel = balusters and handrail.", "Thermik: Jede Region ist die Außenfläche ihrer Körper per Verweis: Stufen = Platte und Randring aller fünfzehn Wannen (Oberseite, Unterseite und beide Flanschseiten liegen frei), Säule = Rohr und Fußplatte, Geländer = Stäbe und Handlauf.")
    s.dec("Structure: the tube, each tread pan (plate and flange as one bending section, fixed at the column through a 30 x 45 x 20 mm weld hub), balusters and handrail are members; a fixed disc support at the base plate, a roller at the column head for the horizontal tie to the upper slab and mass points for column and balustrade.", "Tragwerk: Rohr, jede Stufenwanne (Platte und Flansch als ein Biegequerschnitt, über einen Schweißblock 30 x 45 x 20 mm an der Säule eingespannt), Stäbe und Handlauf sind Bauteile; eine feste Scheibe als Auflager an der Fußplatte, eine Rolle am Säulenkopf für die waagrechte Halterung an der oberen Decke sowie Massenpunkte für Säule und Geländer.")
    return s


def structure_common(s, scale, tread_refs, joints, joint_box, tie_r, disc_h, handrail=True, balusters_member=True):
    s.stof("st-column", "column", "column", "member")
    for k in range(1, 16):
        for p_ in tread_refs:
            s.stof("st-tread-%s-%d" % (p_, k) if p_ else "st-tread-%d" % k, ("tread-%s-%d" % (p_, k)) if p_ else "tread-%d" % k, "tread-%d" % k, "member")
        if joints:
            phi = math.radians(11.25 + 22.5 * (k - 1))
            s.st(box("joint-tread-%d" % k, "tread-%d" % k, "steel", 0.3, joint_box, at=[0.105 * math.cos(phi), -0.03, 0.105 * math.sin(phi)], rot=quat_y(-math.degrees(phi)), role="detail"), "tread-%d" % k, "joint")
    s.stof("st-handrail", "handrail", "balustrade", "member")
    if balusters_member:
        s.stof("st-baluster", "baluster", "balustrade", "member")
    s.st(revolve("support-base-fix", "column", "steel", 0.15, "x", [0, 0, 0], [[0, 0.2], [disc_h, 0.2]], role="detail"), "column", "support")
    s.st(cyl("support-top-tie", "column", "steel", 0.15, "z", [2.8 - tie_r, 0, 0], 0.1, tie_r, role="detail"), "column", "support")
    s.st(ellipsoid("mass-column", "column", "steel", 0.1, [0.06, 0.06, 0.06] if scale < 500 else [0.0842, 0.0842, 0.0842], at=[1.4, 0, 0], role="detail"), "column", "mass")
    s.st(ellipsoid("mass-balustrade", "balustrade", "steel", 0.1, [0.05, 0.05, 0.05] if scale < 500 else [0.0842, 0.0842, 0.0842], at=[0, 2.43, 0], role="detail"), "balustrade", "mass")


def s100():
    s = Scale(100)
    column(s, False, 0.015, 32)
    plate(s, 0.02, False)
    treads(s, 0.015, 0.015, 4)
    balusters(s)
    rail(s, 36)
    thermal_refs(s, ["tread-%s-%d" % (p, k) for k in range(1, 16) for p in ("top", "lip")], ["column", "base-plate"], ["baluster", "handrail"])
    structure_common(s, 100, ["top", "lip"], True, [0.03, 0.045, 0.02], 0.008, 0.0155)
    s.dec("At 1:100 the column is a tube with a 15 mm wall (the real 6.3 mm is below the 15 mm feature), the base plate stays 20 mm without bolts, each tread is a 15 mm plate with a 15 mm flange ring in four arc facets instead of eight, the balusters keep their true 12 mm as slender rods and the handrail is swept through 36 stations of the helix.", "Bei 1:100 ist die Säule ein Rohr mit 15 mm Wand (die wirklichen 6,3 mm liegen unter dem 15-mm-Merkmal), die Fußplatte bleibt 20 mm stark ohne Schrauben, jede Stufe ist eine 15-mm-Platte mit 15-mm-Randring in vier statt acht Bogenfacetten, die Stäbe behalten ihre wahren 12 mm als schlanke Stäbe und der Handlauf wird über 36 Stationen der Wendel gezogen.")
    s.dec("Thermal: by reference to the body solids as at 1:50 (plate and flange of each tread, tube and plate, balusters and handrail); the wall and flange thicknesses are thicker than real, the exposed surfaces are not.", "Thermik: wie bei 1:50 per Verweis auf die Körper (Platte und Flansch jeder Stufe, Rohr und Platte, Stäbe und Handlauf); Wand- und Flanschdicken sind dicker als real, die freiliegenden Flächen nicht.")
    s.dec("Structure: members and weld hubs (20 x 30 x 45 mm, the smallest legal size here) as at 1:50, a 20 mm disc support at the base, a 16 mm roller at the column head and two mass points; no anchor bolts.", "Tragwerk: Bauteile und Schweißblöcke (20 x 30 x 45 mm, die kleinste zulässige Größe) wie bei 1:50, eine 20-mm-Scheibe als Auflager am Fuß, eine 16-mm-Rolle am Säulenkopf und zwei Massenpunkte; keine Ankerschrauben.")
    return s


def s200():
    s = Scale(200)
    s.b(cyl("column", "column", "steel", 0.5, "x", [1.4, 0, 0], 2.8, RC))
    s.b(cyl("base-plate", "column", "steel", 0.5, "x", [0.015, 0, 0], 0.03, 0.2))
    treads(s, 0, 0, 3, flange=False)
    rail(s, 19)
    thermal_refs(s, ["tread-%d" % k for k in range(1, 16)], ["column", "base-plate"], ["handrail"])
    b = capsule("th-baluster", "balustrade", "steel-bright", 0.3, [0.76011, RISE, 0.15119], [0.76011, 1.12, 0.15119], BAL_R)
    b["repeat"] = {"count": 15, "rotationAxis": [0, 1, 0], "rotationRad": -0.3926991, "translation": [0, RISE, 0]}
    b["slender"] = True
    s.th(b, "balustrade-steel")
    structure_common(s, 200, [""], False, None, 0.0155, 0.031, balusters_member=False)
    s.dec("At 1:200 the column is a solid 168 mm cylinder on a 30 mm disc (the smallest feature; drawn 20 mm), each tread one 60 mm solid wedge of 22.5 degrees (plate and flange pan merged), and the handrail is a 42 mm tube through 19 helix stations; the balusters are not drawn at this scale and stay out of the body.", "Bei 1:200 ist die Säule ein massiver Zylinder von 168 mm auf einer 30-mm-Scheibe (kleinstes Merkmal; gezeichnet 20 mm), jede Stufe ein 60 mm hoher Vollkeil von 22,5 Grad (Wanne aus Platte und Flansch zusammengefasst) und der Handlauf ein 42-mm-Rohr durch 19 Wendelstationen; die Stäbe sind in diesem Maßstab nicht gezeichnet und bleiben außerhalb des Körpers.")
    s.dec("Thermal: treads, column and handrail by reference; the 15 balusters remain as slender 12 mm rods in the thermal list only, so the balustrade region keeps its surface.", "Thermik: Stufen, Säule und Handlauf per Verweis; die 15 Stäbe bleiben als schlanke 12-mm-Stäbe nur in der Thermikliste, damit die Geländerregion ihre Fläche behält.")
    s.dec("Structure: column, fifteen tread wedges and handrail as members, disc and roller supports, two mass points; weld hubs are below the 30 mm feature and left out.", "Tragwerk: Säule, fünfzehn Stufenkeile und Handlauf als Bauteile, Scheiben- und Rollenauflager, zwei Massenpunkte; die Schweißblöcke liegen unter dem 30-mm-Merkmal und entfallen.")
    return s


def s500():
    s = Scale(500)
    s.b(cyl("column", "column", "steel", 0.5, "x", [1.4, 0, 0], 2.8, RC))
    s.b(cyl("base-plate", "column", "steel", 0.5, "x", [0.0375, 0, 0], 0.075, 0.2))
    treads(s, 0, 0, 2, flange=False, depth=0.075)
    rail(s, 15, r=0.0325)
    thermal_refs(s, ["tread-%d" % k for k in range(1, 16)], ["column", "base-plate"], ["handrail"])
    structure_common(s, 500, [""], False, None, 0.04, 0.076, balusters_member=False)
    s.dec("At 1:500 the stair is the saw-tooth outline of the 2D: a 168 mm solid column on a 75 mm disc foot, fifteen solid wedges of 22.5 degrees and 75 mm (the smallest feature, the real pan is 60 mm) and the handrail as a 65 mm tube through 15 helix stations, as thick as the 0.13 mm line draws it. The balusters are not drawn and not modelled.", "Bei 1:500 ist die Treppe die Sägezahnkontur der 2D-Zeichnung: eine massive 168-mm-Säule auf einem 75-mm-Scheibenfuß, fünfzehn Vollkeile von 22,5 Grad und 75 mm (kleinstes Merkmal, die wirkliche Wanne ist 60 mm) und der Handlauf als 65-mm-Rohr durch 15 Wendelstationen, so dick, wie ihn die 0,13-mm-Linie zeichnet. Die Stäbe sind nicht gezeichnet und nicht modelliert.")
    s.dec("Thermal: treads, column with foot and handrail by reference; the 12 mm balusters are far below the 75 mm feature and carry no thermal solid here, the total area still lies within the 30 % band.", "Thermik: Stufen, Säule mit Fuß und Handlauf per Verweis; die 12-mm-Stäbe liegen weit unter dem 75-mm-Merkmal und tragen hier keinen Thermikkörper, die Gesamtfläche liegt dennoch im 30-%-Band.")
    s.dec("Structure: column, treads and handrail as members, a 75 mm disc and a 80 mm roller as supports and two mass points sized to the feature; no joints at this scale.", "Tragwerk: Säule, Stufen und Handlauf als Bauteile, eine 75-mm-Scheibe und eine 80-mm-Rolle als Auflager sowie zwei Massenpunkte in Merkmalsgröße; keine Knoten in diesem Maßstab.")
    return s


SCALES = {"500": s500, "50": s50, "100": s100, "200": s200}
if __name__ == "__main__":
    want = sys.argv[1:] or list(SCALES)
    save("components", "spiral-stair", [SCALES[w]() for w in want])
