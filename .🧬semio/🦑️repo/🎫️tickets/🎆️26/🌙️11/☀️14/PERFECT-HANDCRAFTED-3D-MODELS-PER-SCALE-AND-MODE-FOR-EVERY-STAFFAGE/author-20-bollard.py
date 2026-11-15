import os, math, sys
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-20-lib.py")).read())

SH, SL, HY = "shaft", "sleeve", "hydraulic"
STG, RET, AMB, CIF, SSL, HYD = "steel-grey", "retro-film", "pc-amber", "cast-iron-frame", "steel-sleeve", "steel-hyd"
DOME = [[-0.07, 0.1], [-0.04745, 0.09606], [-0.02963, 0.08519], [-0.01625, 0.06875], [-0.00704, 0.04815], [-0.00171, 0.02477], [0.0, 0.0]]
sc = []


def circ(r, n=40, cx=0.0, cy=0.0):
    return [[r6(cx + r * math.cos(2 * math.pi * k / n)), r6(cy + r * math.sin(2 * math.pi * k / n))] for k in range(n)]


def chamfer_square(h, c):
    return [[-h + c, -h], [h - c, -h], [h, -h + c], [h, h - c], [h - c, h], [-h + c, h], [-h, h - c], [-h, -h + c]]


def rrect_pts(cx, cy, w, h, r, n=3):
    pts = []
    for (qx, qy, a0) in ((1, 1, 0), (-1, 1, 90), (-1, -1, 180), (1, -1, 270)):
        ox, oy = cx + qx * (w / 2 - r), cy + qy * (h / 2 - r)
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            pts.append([r6(ox + r * math.cos(a)), r6(oy + r * math.sin(a))])
    return pts


def solid_rev(id, bone, mat, tone, axis, origin, a, b, r, role="body", **kw):
    return revolve(id, bone, mat, tone, axis, origin, [[a, 0], [a, r], [b, r], [b, 0]], role=role, **kw)


def ring_rev(id, bone, mat, tone, axis, origin, a, b, r0, r1, role="body", **kw):
    return revolve(id, bone, mat, tone, axis, origin, [[a, r0], [a, r1], [b, r1], [b, r0], [a, r0]], role=role, **kw)


def skin(id, bone, mat, tone, y0, y1, r):
    return loft(id, bone, mat, tone, [sec([0, y0, 0], r, r), sec([0, y1, 0], r, r)], capStart=False, capEnd=False)


def led_repeat(n):
    return {"count": n, "rotationAxis": [0, 1, 0], "rotationRad": r6(2 * math.pi / n), "origin": [0, 0, 0]}


# ---------------------------------------------------------------- 1:50
s = Scale(50)
s.b(solid_rev("shaft-tube", SH, STG, 0.5, "y", [0, 0, 0], -1.15, -0.24, 0.1))
s.b(solid_rev("reflector", SH, RET, 0.08, "y", [0, 0, 0], -0.24, -0.13, 0.1))
s.b(solid_rev("beacon", SH, AMB, 0.7, "y", [0, 0, 0], -0.13, -0.07, 0.1))
s.b(revolve("cap", SH, STG, 0.5, "y", [0, 0, 0], DOME))
s.b(revolve("led", SH, STG, 0.1, "x", [0.0965, -0.1, 0], [[0, 0], [0, 0.01], [0.008, 0.01], [0.008, 0]], role="detail", repeat=led_repeat(10)))
s.b(ring_rev("sleeve-wall", SL, SSL, 0.3, "y", [0, 0, 0], -1.25, -0.07, 0.142, 0.15))
s.b(solid_rev("sleeve-floor", SL, SSL, 0.3, "y", [0, 0, 0], -1.25, -1.242, 0.15))
s.b(ext("cover-frame", SL, CIF, 0.6, "zx", chamfer_square(0.2, 0.02), -0.07, 0.015, holes=[circ(0.145)]))
s.b(solid_rev("frame-bolt", SL, CIF, 0.2, "y", [0.175, 0, 0.175], 0.015, 0.023, 0.01, role="detail", repeat={"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": r6(math.pi / 2), "origin": [0, 0, 0]}))
s.b(ext("brush-seal", SL, CIF, 0.3, "zx", circ(0.145), -0.02, -0.01, holes=[circ(0.1)], role="detail"))
s.b(ext("power-unit", HY, HYD, 0.5, "xy", rrect_pts(0.055, 0.05, 0.21, 0.4, 0.01), -0.01, 0.2, holes=[circ(0.035, 24, 0.055, 0.16)]))
s.b(solid_rev("motor", HY, HYD, 0.8, "z", [0.055, 0.16, 0.025], 0, 0.14, 0.035, role="detail"))
s.b(solid_rev("ram", HY, HYD, 0.7, "y", [-0.2, 0, 0], -0.442, 0.1, 0.06))
s.b(solid_rev("ram-rod", HY, HYD, 0.3, "y", [-0.2, 0, 0], 0.1, 0.3, 0.02, role="detail"))
s.b(capsule("hose", HY, HYD, 0.3, [-0.05, 0.08, 0.03], [-0.14, 0.08, 0.03], 0.008, role="detail"))
s.b(solid_rev("port-boss", HY, HYD, 0.3, "x", [-0.07, 0.08, 0.03], 0, 0.02, 0.02, role="detail"))

s.th(skin("th-shaft-skin", SH, STG, 0.5, -0.65, -0.24, 0.1), "shaft-steel")
s.thof("th-cap", "cap", "shaft-steel")
s.thof("th-reflector", "reflector", "reflector-band")
s.thof("th-beacon", "beacon", "beacon-led")
s.th(mark([ext("th-frame-top", SL, CIF, 0.6, "zx", chamfer_square(0.2, 0.02), 0.0075, 0.015, holes=[circ(0.145)])], ["-v", "-u", "+u", "-w", "+w"])[0], "cover-frame")
s.th(skin("th-sleeve-skin", SL, SSL, 0.3, -1.25, -0.07, 0.15), "sleeve-steel")
s.thof("th-sleeve-floor", "sleeve-floor", "sleeve-steel")
s.thof("th-power-unit", "power-unit", "hydraulic-unit")

s.stof("member-sleeve", "sleeve-wall", "sleeve", "member")
s.st(ring_rev("member-shaft", SH, STG, 0.5, "y", [0, 0, 0], -1.15, -0.07, 0.095, 0.1), "shaft", "member")
s.stof("member-unit", "power-unit", "hydraulic", "member")
s.st(ring_rev("member-ram", HY, HYD, 0.7, "y", [-0.2, 0, 0], -0.442, 0.1, 0.052, 0.06), "hydraulic", "member")
s.st(solid_rev("member-ram-rod", HY, HYD, 0.3, "y", [-0.2, 0, 0], 0.1, 0.3, 0.02), "hydraulic", "member")
s.stof("joint-shaft-guide", "brush-seal", "shaft", "joint")
s.stof("joint-hydraulic-mount", "port-boss", "hydraulic", "joint")
s.st(ellipsoid("mass-shaft", SH, STG, 0.5, [0.03, 0.03, 0.03], [0, -0.55, 0], role="body"), "shaft", "mass")
s.st(ellipsoid("mass-sleeve", SL, SSL, 0.3, [0.025, 0.025, 0.025], [0, -0.6, 0.0]), "sleeve", "mass")
s.st(ellipsoid("mass-hydraulic", HY, HYD, 0.5, [0.03, 0.03, 0.03], [0, 0, 0.095]), "hydraulic", "mass")
s.st(ext("support-sleeve-fixed", SL, CIF, 0.6, "zx", [[-0.15, -0.15], [0.15, -0.15], [0.15, 0.15], [-0.15, 0.15]], -0.0075, 0.0075, holes=[circ(0.145)]), "sleeve", "support")
s.dec("Body: the grey steel shaft as a solid revolve of 0.20 m diameter in four stacked pieces that follow the drawing (tube, white reflector band, amber beacon band with ten 20 mm LED lenses on a 36 degree pitch, domed cap), the hollow 0.30 m sleeve with an 8 mm wall and floor, the cast-iron cover frame with its 0.145 m opening and four bolt heads, the 10 mm brush-seal ring between frame and shaft, the power unit as a rounded housing with a pocket that carries the motor, the 0.12 m ram barrel with its rod up to the shaft base, the hose through the sleeve wall and its port boss. The shaft is modelled filled, as the section drawing shows it; the wall of the real tube is the 5 mm thermal and structural layer.",
      "Körper: der graue Stahlschaft als voller Drehkörper von 0,20 m Durchmesser in vier gestapelten Teilen nach der Zeichnung (Rohr, weißes Reflexband, bernsteinfarbenes Leuchtband mit zehn LED-Linsen von 20 mm im Raster von 36 Grad, gewölbte Kappe), die hohle Hülse von 0,30 m mit 8 mm Wand und Boden, der Gusseisen-Abdeckrahmen mit der Öffnung von 0,145 m und vier Schraubenköpfen, der 10 mm dicke Bürstendichtungsring zwischen Rahmen und Schaft, die Antriebseinheit als abgerundetes Gehäuse mit einer Tasche für den Motor, der Zylinder von 0,12 m mit seiner Stange bis zum Schaftfuß, der Schlauch durch die Hülsenwand und sein Anschlussstutzen. Der Schaft ist gefüllt modelliert, wie ihn die Schnittzeichnung zeigt; die Wand des echten Rohrs ist die thermische und statische Schicht von 5 mm.")
s.dec("Thermal: every region is a skin where the surface, not the volume, exchanges heat: the shaft-steel skin is the exposed 0.41 m of tube above ground in the raised pose plus the cap, the two bands and the sleeve skin are the bodies' own surfaces, the cover frame is a 7.5 mm top plate whose edges are section faces in the paving, the hydraulic unit is the housing. areaM2 was corrected to the modelled exposed areas (shaft 0.41 to 0.307, beacon 0.025 to 0.0375, cover frame 0.12 to 0.093, hydraulic unit 0.35 to 0.45).",
      "Thermisch: jede Region ist eine Haut, wo die Oberfläche und nicht das Volumen Wärme austauscht: die Haut shaft-steel ist das freiliegende Rohr von 0,41 m über dem Boden in der ausgefahrenen Lage plus die Kappe, die beiden Bänder und die Hülsenhaut sind die eigenen Oberflächen der Körper, der Abdeckrahmen ist eine Deckplatte von 7,5 mm, deren Kanten Schnittflächen im Belag sind, die Hydraulikeinheit ist das Gehäuse. areaM2 wurde auf die modellierten freiliegenden Flächen korrigiert (Schaft 0,41 auf 0,307, Leuchtring 0,025 auf 0,0375, Abdeckrahmen 0,12 auf 0,093, Hydraulikeinheit 0,35 auf 0,45).")
s.dec("Structure: the three bones carry real sections: the buried sleeve as the hollow 8 mm tube, the shaft as a 5 mm steel tube, the hydraulic bone as the unit housing plus the 0.12 m ram barrel (tube) and its 40 mm rod; joints are the brush-seal guide of the prismatic shaft joint and the port boss that mounts the unit; masses are lumped at the declared centres; the fixed support is a square collar of 15 mm under the frame at the sleeve anchor.",
      "Tragwerk: die drei Knochen tragen echte Querschnitte: die eingegrabene Hülse als hohles Rohr mit 8 mm Wand, der Schaft als Stahlrohr mit 5 mm, der Hydraulikknochen als Einheitsgehäuse mit dem Zylinderrohr von 0,12 m und seiner Stange von 40 mm; Gelenke sind die Bürstendichtungsführung des Schubgelenks und der Anschlussstutzen, der die Einheit hält; die Massen sitzen punktförmig an den angegebenen Schwerpunkten; das feste Auflager ist ein quadratischer Kragen von 15 mm unter dem Rahmen am Hülsenanker.")
sc.append(s)



def bez(p0, c1, c2, p3, n=6):
    out = []
    for k in range(1, n + 1):
        u = k / n
        a, b, c, d = (1 - u) ** 3, 3 * u * (1 - u) ** 2, 3 * u * u * (1 - u), u ** 3
        out.append([r6(a * p0[0] + b * c1[0] + c * c2[0] + d * p3[0]), r6(a * p0[1] + b * c1[1] + c * c2[1] + d * p3[1])])
    return out


DOME2 = [[-0.05, 0.1]] + bez((-0.05, 0.1), (-0.01, 0.1), (0.0, 0.05), (0.0, 0.0))


def unit_box(s, id, x0, x1, z0, z1, r=None):
    s.b(box(id, HY, HYD, 0.5, [x1 - x0, 0.4, z1 - z0], [(x0 + x1) / 2 - 0.2, 0.05, (z0 + z1) / 2], r=r))


def support(s, w, hole, y0, y1):
    s.st(ext("support-sleeve-fixed", SL, CIF, 0.6, "zx", [[-w, -w], [w, -w], [w, w], [-w, w]], y0, y1, holes=[circ(hole)]), "sleeve", "support")


def masses(s, k):
    s.st(ellipsoid("mass-shaft", SH, STG, 0.5, [k, k, k], [0, -0.55, 0]), "shaft", "mass")
    s.st(ellipsoid("mass-sleeve", SL, SSL, 0.3, [k, k, k], [0, -0.6, 0.0]), "sleeve", "mass")
    s.st(ellipsoid("mass-hydraulic", HY, HYD, 0.5, [k, k, k], [0, 0, 0.095]), "hydraulic", "mass")


# ---------------------------------------------------------------- 1:100
s = Scale(100)
s.b(solid_rev("shaft-tube", SH, STG, 0.5, "y", [0, 0, 0], -1.15, -0.24, 0.1))
s.b(solid_rev("reflector", SH, RET, 0.08, "y", [0, 0, 0], -0.24, -0.13, 0.1))
s.b(solid_rev("beacon", SH, AMB, 0.7, "y", [0, 0, 0], -0.13, -0.07, 0.1))
s.b(revolve("cap", SH, STG, 0.5, "y", [0, 0, 0], DOME))
s.b(ring_rev("sleeve-wall", SL, SSL, 0.3, "y", [0, 0, 0], -1.25, -0.07, 0.135, 0.15))
s.b(solid_rev("sleeve-floor", SL, SSL, 0.3, "y", [0, 0, 0], -1.25, -1.234, 0.15))
s.b(ext("cover-frame", SL, CIF, 0.6, "zx", rrect_pts(0, 0, 0.4, 0.4, 0.02), -0.07, 0.015, holes=[circ(0.145)]))
unit_box(s, "power-unit", 0.15, 0.36, -0.01, 0.2)
s.b(solid_rev("ram", HY, HYD, 0.7, "y", [-0.2, 0, 0], -0.435, 0.1, 0.06))
s.b(solid_rev("ram-rod", HY, HYD, 0.3, "y", [-0.2, 0, 0], 0.1, 0.3, 0.02, role="detail"))
s.b(capsule("hose", HY, HYD, 0.3, [-0.05, 0.08, 0.03], [-0.14, 0.08, 0.03], 0.008, role="detail"))
s.b(solid_rev("port-boss", HY, HYD, 0.3, "x", [-0.07, 0.08, 0.03], 0, 0.02, 0.02, role="detail"))
s.th(skin("th-shaft-skin", SH, STG, 0.5, -0.65, -0.24, 0.1), "shaft-steel")
s.thof("th-cap", "cap", "shaft-steel")
s.thof("th-reflector", "reflector", "reflector-band")
s.thof("th-beacon", "beacon", "beacon-led")
s.th(mark([ext("th-frame-top", SL, CIF, 0.6, "zx", rrect_pts(0, 0, 0.4, 0.4, 0.02), -0.001, 0.015, holes=[circ(0.145)])], ["-v", "-u", "+u", "-w", "+w"])[0], "cover-frame")
s.th(skin("th-sleeve-skin", SL, SSL, 0.3, -1.25, -0.07, 0.15), "sleeve-steel")
s.thof("th-sleeve-floor", "sleeve-floor", "sleeve-steel")
s.thof("th-power-unit", "power-unit", "hydraulic-unit")
s.stof("member-sleeve", "sleeve-wall", "sleeve", "member")
s.st(ring_rev("member-shaft", SH, STG, 0.5, "y", [0, 0, 0], -1.15, -0.07, 0.095, 0.1), "shaft", "member")
s.stof("member-unit", "power-unit", "hydraulic", "member")
s.st(ring_rev("member-ram", HY, HYD, 0.7, "y", [-0.2, 0, 0], -0.435, 0.1, 0.05, 0.06), "hydraulic", "member")
s.stof("member-ram-rod", "ram-rod", "hydraulic", "member")
s.st(ring_rev("joint-shaft-guide", SL, STG, 0.3, "y", [0, 0, 0], -0.026, -0.01, 0.1, 0.145), "shaft", "joint")
s.stof("joint-hydraulic-mount", "port-boss", "hydraulic", "joint")
masses(s, 0.03)
support(s, 0.15, 0.145, -0.008, 0.008)
s.dec("Body: the same bones and proportions as 1:50, modelled anew against the 1:100 drawing: the shaft keeps its four pieces (tube, reflector band, beacon band, cap), the sleeve wall is thickened inward to 15 mm (the scale's smallest feature) so its outer 0.30 m stays true, the frame is one rounded square ring, the LED lenses, bolts and brush seal fall below the scale and are left out, the power unit is a plain block (the motor sits inside it), ram barrel, rod, hose and port boss are kept as drawn.",
      "Körper: dieselben Knochen und Proportionen wie 1:50, neu gegen die Zeichnung 1:100 modelliert: der Schaft behält seine vier Teile (Rohr, Reflexband, Leuchtband, Kappe), die Hülsenwand ist nach innen auf 15 mm (kleinste Einzelheit des Maßstabs) verdickt, damit die Außenseite von 0,30 m stimmt, der Rahmen ist ein einziger abgerundeter Quadratring, LED-Linsen, Schrauben und Bürstendichtung liegen unter dem Maßstab und entfallen, die Antriebseinheit ist ein einfacher Block (der Motor sitzt darin), Zylinder, Stange, Schlauch und Anschlussstutzen bleiben wie gezeichnet.")
s.dec("Thermal: skins and references as at 1:50 (shaft skin above ground, cap, bands, sleeve skin and floor, frame top plate, power-unit block); the exposed areas follow from the 1:100 bodies and stay within 30 % per region.",
      "Thermisch: Häute und Verweise wie bei 1:50 (Schaftrohr über dem Boden, Kappe, Bänder, Hülsenhaut und Boden, Rahmendeckplatte, Einheitsblock); die freiliegenden Flächen folgen aus den Körpern 1:100 und bleiben je Region innerhalb von 30 %.")
s.dec("Structure: sleeve tube, 5 mm shaft tube, unit block, ram barrel tube and rod are the members; the brush seal is gone, so the guide of the prismatic joint is a 15 mm ring between shaft and frame bore; support collar 15 mm; masses lumped.",
      "Tragwerk: Hülsenrohr, Schaftrohr von 5 mm, Einheitsblock, Zylinderrohr und Stange sind die Stäbe; die Bürstendichtung entfällt, die Führung des Schubgelenks ist daher ein Ring von 15 mm zwischen Schaft und Rahmenöffnung; Auflagerkragen 15 mm; Massen punktförmig.")
sc.append(s)

# ---------------------------------------------------------------- 1:200
s = Scale(200)
s.b(solid_rev("shaft-tube", SH, STG, 0.5, "y", [0, 0, 0], -1.15, -0.25, 0.1))
s.b(solid_rev("reflector", SH, RET, 0.08, "y", [0, 0, 0], -0.25, -0.13, 0.1))
s.b(revolve("cap-head", SH, STG, 0.5, "y", [0, 0, 0], [[-0.13, 0], [-0.13, 0.1], *DOME2]))
s.b(ring_rev("sleeve-wall", SL, SSL, 0.3, "y", [0, 0, 0], -1.25, -0.07, 0.1, 0.115))
s.b(solid_rev("sleeve-floor", SL, SSL, 0.3, "y", [0, 0, 0], -1.25, -1.219, 0.115))
s.b(ext("cover-frame", SL, CIF, 0.6, "zx", rrect_pts(0, 0, 0.4, 0.42, 0.02), -0.08, 0.015, holes=[circ(0.1)]))
unit_box(s, "power-unit", 0.115, 0.36, -0.05, 0.2)
s.b(solid_rev("ram", HY, HYD, 0.7, "y", [-0.2, 0, 0], -0.42, 0.1, 0.06))
s.b(solid_rev("ram-rod", HY, HYD, 0.3, "y", [-0.2, 0, 0], 0.1, 0.3, 0.02, role="detail"))
s.th(skin("th-shaft-skin", SH, STG, 0.5, -0.65, -0.25, 0.1), "shaft-steel")
s.th(revolve("th-dome", SH, STG, 0.5, "y", [0, 0, 0], DOME2), "shaft-steel")
s.thof("th-reflector", "reflector", "reflector-band")
s.th(skin("th-beacon", SH, AMB, 0.7, -0.13, -0.05, 0.1), "beacon-led")
s.th(mark([ext("th-frame-top", SL, CIF, 0.6, "zx", rrect_pts(0, 0, 0.4, 0.42, 0.02), -0.016, 0.015, holes=[circ(0.1)])], ["-v", "-u", "+u", "-w", "+w"])[0], "cover-frame")
s.th(skin("th-sleeve-skin", SL, SSL, 0.3, -1.25, -0.07, 0.115), "sleeve-steel")
s.thof("th-sleeve-floor", "sleeve-floor", "sleeve-steel")
s.thof("th-power-unit", "power-unit", "hydraulic-unit")
s.stof("member-sleeve", "sleeve-wall", "sleeve", "member")
s.st(ring_rev("member-shaft", SH, STG, 0.5, "y", [0, 0, 0], -1.15, -0.13, 0.095, 0.1), "shaft", "member")
s.stof("member-unit", "power-unit", "hydraulic", "member")
s.stof("member-ram", "ram", "hydraulic", "member")
s.stof("member-ram-rod", "ram-rod", "hydraulic", "member")
s.st(ring_rev("joint-shaft-guide", SL, STG, 0.3, "y", [0, 0, 0], -0.046, -0.015, 0.1, 0.115), "shaft", "joint")
masses(s, 0.03)
support(s, 0.15, 0.115, -0.016, 0.016)
s.dec("Body: modelled against the 1:200 drawing, which hugs the sleeve onto the shaft edge: tube and reflector band as two cylinders, beacon band and dome merged into one head (the beacon ring is below 30 mm), the sleeve a 15 mm tube from the shaft surface outward (the drawn stroke) with a 31 mm floor, the frame one 0.42 x 0.40 m ring whose opening closes onto the shaft (the 10 mm brush seal merges into it), the power unit one 0.23 x 0.25 x 0.40 m block welded to the sleeve, ram and rod as 120 mm and 40 mm columns. Bolts, LEDs, hose, boss and motor are omitted at this scale.",
      "Körper: gegen die Zeichnung 1:200 modelliert, die die Hülse an die Schaftkante legt: Rohr und Reflexband als zwei Zylinder, Leuchtband und Kappe zu einem Kopf vereint (der Leuchtring liegt unter 30 mm), die Hülse ein Rohr von 15 mm vom Schaft nach außen (der gezeichnete Strich) mit 31 mm Boden, der Rahmen ein Ring 0,42 x 0,40 m, dessen Öffnung sich am Schaft schließt (die Bürstendichtung von 10 mm geht darin auf), die Antriebseinheit ein Block 0,23 x 0,25 x 0,40 m, an die Hülse geschweißt, Zylinder und Stange als Säulen von 120 und 40 mm. Schrauben, LEDs, Schlauch, Stutzen und Motor entfallen in diesem Maßstab.")
s.dec("Thermal: one solid per region: shaft skin above ground and dome, reflector band, an open skin for the beacon ring (the head body merges beacon and dome), the sleeve skin and floor, the frame top plate, the power-unit block; at this scale only the total exposed area over all regions is compared with the declared total.",
      "Thermisch: ein Körper je Region: Schaftrohr über dem Boden und Kalotte, Reflexband, eine offene Haut für den Leuchtring (der Kopfkörper vereint Leuchtband und Kalotte), Hülsenhaut und Boden, die Rahmendeckplatte, der Einheitsblock; in diesem Maßstab wird nur die Summe der freiliegenden Flächen mit der angegebenen Summe verglichen.")
s.dec("Structure: sleeve tube, shaft tube, unit block, ram and rod are kept as members (no load-bearing member is dropped); the shaft guide is one 30 mm ring; the hose boss joint is merged into the unit block; support collar 30 mm.",
      "Tragwerk: Hülsenrohr, Schaftrohr, Einheitsblock, Zylinder und Stange bleiben Stäbe (kein tragender Stab entfällt); die Schaftführung ist ein Ring von 30 mm; das Gelenk des Schlauchstutzens geht im Einheitsblock auf; Auflagerkragen 30 mm.")
sc.append(s)

# ---------------------------------------------------------------- 1:500
s = Scale(500)
s.b(solid_rev("shaft", SH, STG, 0.5, "y", [0, 0, 0], -1.15, -0.13, 0.1))
s.b(revolve("cap-head", SH, STG, 0.5, "y", [0, 0, 0], [[-0.13, 0], [-0.13, 0.1], *DOME2]))
s.b(ring_rev("sleeve-wall", SL, SSL, 0.3, "y", [0, 0, 0], -1.25, -0.07, 0.1, 0.115))
s.b(solid_rev("sleeve-floor", SL, SSL, 0.3, "y", [0, 0, 0], -1.25, -1.1745, 0.115))
s.b(ext("cover-frame", SL, CIF, 0.6, "zx", rrect_pts(0, 0, 0.4, 0.42, 0.02), -0.11, 0.02, holes=[circ(0.1)]))
unit_box(s, "power-unit", 0.115, 0.36, -0.05, 0.2)
s.th(skin("th-shaft-skin", SH, STG, 0.5, -0.65, -0.13, 0.1), "shaft-steel")
s.thof("th-head", "cap-head", "shaft-steel")
s.th(skin("th-reflector", SH, RET, 0.08, -0.25, -0.13, 0.1), "reflector-band")
s.th(skin("th-beacon", SH, AMB, 0.7, -0.13, -0.05, 0.1), "beacon-led")
s.th(mark([ext("th-frame-top", SL, CIF, 0.6, "zx", rrect_pts(0, 0, 0.4, 0.42, 0.02), -0.056, 0.02, holes=[circ(0.1)])], ["-v", "-u", "+u", "-w", "+w"])[0], "cover-frame")
s.th(skin("th-sleeve-skin", SL, SSL, 0.3, -1.25, -0.07, 0.115), "sleeve-steel")
s.thof("th-power-unit", "power-unit", "hydraulic-unit")
s.stof("member-sleeve", "sleeve-wall", "sleeve", "member")
s.st(ring_rev("member-shaft", SH, STG, 0.5, "y", [0, 0, 0], -1.15, -0.13, 0.095, 0.1), "shaft", "member")
s.stof("member-unit", "power-unit", "hydraulic", "member")
s.st(solid_rev("member-ram", HY, HYD, 0.7, "y", [-0.2, 0, 0], -0.4175, 0.3, 0.06), "hydraulic", "member")
s.st(ring_rev("joint-shaft-guide", SL, STG, 0.3, "y", [0, 0, 0], -0.1, -0.0235, 0.1, 0.115), "shaft", "joint")
masses(s, 0.04)
support(s, 0.15, 0.115, -0.0385, 0.0385)
s.dec("Body: modelled against the 1:500 drawing: the shaft is one cylinder from the base to the cap (tube and reflector band are one piece), the cap one head with the beacon ring and dome, the sleeve a 15 mm tube hugging the shaft (the drawn stroke) with a 75 mm floor, the frame one 0.42 x 0.40 m ring 130 mm high, the power unit one block; ram, rod, hose, bolts, LEDs and motor are below the scale or omitted by the drawing.",
      "Körper: gegen die Zeichnung 1:500 modelliert: der Schaft ist ein Zylinder vom Fuß bis zur Kappe (Rohr und Reflexband sind ein Stück), die Kappe ein Kopf mit Leuchtring und Kalotte, die Hülse ein Rohr von 15 mm, das sich an den Schaft legt (der gezeichnete Strich), mit 75 mm Boden, der Rahmen ein Ring 0,42 x 0,40 m und 130 mm hoch, die Antriebseinheit ein Block; Zylinder, Stange, Schlauch, Schrauben, LEDs und Motor liegen unter dem Maßstab oder entfallen laut Zeichnung.")
s.dec("Thermal: one skin or block per region (shaft skin above ground and the head, reflector and beacon as open skins, sleeve skin without a separate floor disc, frame top plate, power unit); only the total exposed area is compared at this scale.",
      "Thermisch: eine Haut oder ein Block je Region (Schaftrohr über dem Boden und Kopf, Reflexband und Leuchtring als offene Häute, Hülsenhaut ohne eigene Bodenscheibe, Rahmendeckplatte, Einheit); in diesem Maßstab wird nur die Summe der freiliegenden Flächen verglichen.")
s.dec("Structure: sleeve tube, shaft tube, unit block and one merged ram-and-rod column (ram and rod are not separable at 75 mm) remain as members, the shaft guide is merged into one 75 mm ring, support collar 75 mm; the ram column lies inside the sleeve outline.",
      "Tragwerk: Hülsenrohr, Schaftrohr, Einheitsblock und eine vereinte Säule aus Zylinder und Stange (bei 75 mm nicht trennbar) bleiben Stäbe, die Schaftführung ist zu einem Ring von 75 mm vereint, Auflagerkragen 75 mm; die Zylindersäule liegt innerhalb der Hülsenkontur.")
sc.append(s)

save("objects", "retractable-bollard", sc)
