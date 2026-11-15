import os, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())
R, V = "radiator", "valve-head"
PS, BRS = "painted-steel", "brass"
sc = []
ZL, ZR = -0.335, 0.395

def fin(v0, v1, t=0.0008):
    pts_up, pts_dn = [], []
    n = 12
    for k in range(n):
        u = 0.03 + k * 0.0504545
        v = v0 if k % 2 == 0 else v1
        pts_up.append([u, v + t / 2])
        pts_dn.append([u, v - t / 2])
    return pts_up + pts_dn[::-1]

def valve_parts(s, fine):
    s.b(cyl("valve-head", V, BRS, 0.3, "z", [0, -0.015, -0.41], 0.08, 0.027, role="detail"))
    s.b(box("valve-body", R, BRS, 0.3, [0.06, 0.05, 0.07], [-0.03, -0.055, ZL], role="detail", repeat={"count": 2, "translation": [0, 0, ZR - ZL]}))
    s.b(cyl("pipe", R, BRS, 0.3, "x", [-0.08, -0.055, ZL], 0.06, 0.011, role="detail", repeat={"count": 2, "translation": [0, 0, ZR - ZL]}))
    if fine:
        s.b(cyl("lockshield-cap", R, BRS, 0.35, "z", [-0.033, -0.055, 0.446], 0.032, 0.021, role="detail"))

def support(s, k):
    sz = {50: 0.02, 100: 0.025, 200: 0.03}[k]
    s.st(box("support-bracket-top-block", R, PS, 0.4, [sz, 0.035, 0.84], [0.47, 0.0175, 0]), "radiator", "support")

# ---- 1:50
s = Scale(50)
s.b(box("panel-rear", R, PS, 0.7, [0.585, 0.022, 0.94], [0.2925, -0.011, 0], r=0.008))
for nm, a in (("lower", 0), ("mid", 0.195), ("upper", 0.39)):
    s.b(box(f"panel-front-{nm}", R, PS, 0.7, [0.195, 0.022, 0.94], [a + 0.0975, -0.089, 0], r=0.005))
s.b(ext("convector-rear", R, PS, 0.5, "xy", fin(-0.022, -0.05), -0.46, 0.46, role="texture"))
s.b(ext("convector-front", R, PS, 0.5, "xy", fin(-0.05, -0.078), -0.46, 0.46, role="texture"))
s.b(box("end-cap", R, PS, 0.5, [0.6, 0.1, 0.03], [0.3, -0.05, 0.485], r=0.008, repeat={"count": 2, "translation": [0, 0, -0.97]}))
s.b(box("bottom-cap", R, PS, 0.5, [0.015, 0.1, 0.94], [0.0075, -0.05, 0]))
s.b(box("top-grille", R, PS, 0.35, [0.015, 0.1, 0.94], [0.5925, -0.05, 0], role="detail"))
s.b(box("bracket", R, PS, 0.4, [0.012, 0.035, 0.04], [0.056, 0.0175, 0.4], role="detail", repeat={"count": 2, "translation": [0.414, 0, 0]}))
s.b(box("bracket-far", R, PS, 0.4, [0.012, 0.035, 0.04], [0.056, 0.0175, -0.4], role="detail", repeat={"count": 2, "translation": [0.414, 0, 0]}))
valve_parts(s, True)
for nm in ("rear", "front-lower", "front-mid", "front-upper"):
    s.thof(f"th-panel-{nm}", f"panel-{nm}" if nm == "rear" else f"panel-{nm}", "panel-rear" if nm == "rear" else f"panel-{nm}")
s.thof("th-convector-rear", "convector-rear", "convector")
s.thof("th-convector-front", "convector-front", "convector")
s.thof("th-top-grille", "top-grille", "top-grille")
s.thof("th-bracket", "bracket", "bracket")
s.thof("th-bracket-far", "bracket-far", "bracket")
for nm in ("valve-head", "valve-body", "pipe", "lockshield-cap"):
    s.thof(f"th-valve-{nm}", nm, "valve")
s.th(cyl("th-emitter-convection-outlet", R, PS, 0.5, "x", [0.5975, -0.05, 0], 0.0075, 0.047, tags=["convection-outlet"]), "top-grille")
s.th(cyl("th-emitter-front-plume", R, PS, 0.5, "y", [0.3, -0.09625, 0], 0.0075, 0.05, tags=["front-plume"]), "panel-front-mid")
s.st(box("member-panel", R, PS, 0.6, [0.585, 0.1, 0.94], [0.2925, -0.05, 0], r=0.008), "radiator", "member")
s.stof("member-bracket", "bracket", "radiator", "member")
s.stof("member-bracket-far", "bracket-far", "radiator", "member")
s.stof("joint-valve-head", "valve-head", "valve-head", "joint")
support(s, 50)
s.dec("Body: rear panel and three front zones as 22 mm pressed-steel boxes (6 and 5 mm rounded edges), the two convector fins as 0.8 mm zig-zag sheets (texture role, as drawn) in the 56 mm gap, end caps, bottom cap, the 15 mm top grille, four wall brackets, the brass thermostat valve (knob on its own revolute bone, 22 mm pipes, valve bodies) and the lockshield cap.",
      "Körper: Rückwand und drei Frontzonen als gepresste Stahlkästen von 22 mm (Kanten 6 und 5 mm gerundet), die beiden Konvektorbleche als 0,8-mm-Zickzackbleche (Textur-Rolle, wie gezeichnet) im Spalt von 56 mm, Seitenkappen, Bodenkappe, das Obergitter von 15 mm, vier Wandkonsolen, das Thermostatventil aus Messing (Kopf auf eigenem Drehknochen, Rohre 22 mm, Ventilkörper) und die Rücklaufkappe.")
s.dec("Thermal: all regions from the body; the two emitters are 7.5 mm outlet discs, the convection outlet on the grille and the front plume on the mid front zone.",
      "Thermisch: alle Regionen aus dem Körper; die zwei Emitter sind Auslassscheiben von 7,5 mm, der Konvektionsauslass am Gitter und die Frontfahne an der mittleren Frontzone.")
s.dec("Structure: the panel block as the member, the four brackets as load path to the wall, a joint at the valve-head knob, and the fixed support block at the upper bracket anchor.",
      "Tragwerk: der Plattenblock als Stab, die vier Konsolen als Lastpfad zur Wand, ein Gelenk am Ventilknopf und der feste Auflagerblock am oberen Konsolenanker.")
sc.append(s)

# ---- 1:100
s = Scale(100)
s.b(box("radiator-body", R, PS, 0.65, [0.57, 0.1, 1.0], [0.285, -0.05, 0]))
s.b(box("top-grille", R, PS, 0.35, [0.03, 0.1, 1.0], [0.585, -0.05, 0], role="detail"))
s.b(box("bracket", R, PS, 0.4, [0.015, 0.035, 0.04], [0.056, 0.0175, 0.4], role="detail", repeat={"count": 2, "translation": [0.42, 0, 0]}))
s.b(box("bracket-far", R, PS, 0.4, [0.015, 0.035, 0.04], [0.056, 0.0175, -0.4], role="detail", repeat={"count": 2, "translation": [0.42, 0, 0]}))
valve_parts(s, True)
for nm, reg in (("rear", "panel-rear"), ("front-lower", "panel-front-lower"), ("front-mid", "panel-front-mid"), ("front-upper", "panel-front-upper"), ("convector", "convector")):
    s.thof(f"th-{nm}", "radiator-body", reg)
s.thof("th-top-grille", "top-grille", "top-grille")
s.thof("th-bracket", "bracket", "bracket")
s.thof("th-bracket-far", "bracket-far", "bracket")
for nm in ("valve-head", "valve-body", "pipe"):
    s.thof(f"th-valve-{nm}", nm, "valve")
s.th(cyl("th-emitter-convection-outlet", R, PS, 0.5, "x", [0.585, -0.05, 0], 0.015, 0.047, tags=["convection-outlet"]), "top-grille")
s.th(cyl("th-emitter-front-plume", R, PS, 0.5, "y", [0.3, -0.0925, 0], 0.015, 0.05, tags=["front-plume"]), "panel-front-mid")
s.stof("member-panel", "radiator-body", "radiator", "member")
s.stof("member-bracket", "bracket", "radiator", "member")
s.stof("member-bracket-far", "bracket-far", "radiator", "member")
s.stof("joint-valve-head", "valve-head", "valve-head", "joint")
support(s, 100)
s.dec("Body: panels, fins and caps are merged into one 570 x 100 x 1000 mm block (the drawing shows one body with a split line), plus the 30 mm grille, brackets at 15 mm, valve with pipes; the lockshield cap merges into the right valve body.",
      "Körper: Platten, Bleche und Kappen sind zu einem Block 570 x 100 x 1000 mm zusammengefasst (die Zeichnung zeigt einen Körper mit Teilungslinie), dazu das Gitter von 30 mm, Konsolen mit 15 mm, Ventil mit Rohren; die Rücklaufkappe geht im rechten Ventilkörper auf.")
s.dec("Thermal: panel, convector and zone regions all reference the merged block; emitters are 15 mm discs. Structure: block member, brackets, valve joint and the support at 15 mm; this structure is already minimal at this scale.",
      "Thermisch: Platten-, Konvektor- und Zonenregionen verweisen alle auf den zusammengefassten Block; Emitter sind Scheiben von 15 mm. Tragwerk: Blockstab, Konsolen, Ventilgelenk und das Auflager mit 15 mm; dieses Tragwerk ist bei diesem Maßstab bereits minimal.")
sc.append(s)

# ---- 1:200
s = Scale(200)
s.b(box("radiator-body", R, PS, 0.65, [0.6, 0.1, 1.0], [0.3, -0.05, 0]))
s.b(cyl("valve-head", V, BRS, 0.3, "z", [0, -0.015, -0.41], 0.08, 0.03, role="detail"))
s.b(box("valve-body", R, BRS, 0.3, [0.075, 0.06, 0.12], [-0.0375, -0.055, -0.38], role="detail"))
s.b(box("valve-body-right", R, BRS, 0.3, [0.075, 0.06, 0.1], [-0.0375, -0.055, 0.43], role="detail"))
s.b(cyl("pipe", R, BRS, 0.3, "x", [-0.08, -0.055, ZL], 0.06, 0.015, role="detail", repeat={"count": 2, "translation": [0, 0, ZR - ZL]}))
s.b(box("bracket", R, PS, 0.4, [0.03, 0.035, 0.04], [0.47, 0.0175, 0.4], role="detail", repeat={"count": 2, "translation": [0, 0, -0.8]}))
s.thof("th-bracket", "bracket", "bracket")
for nm, reg in (("rear", "panel-rear"), ("front-lower", "panel-front-lower"), ("front-mid", "panel-front-mid"), ("front-upper", "panel-front-upper"), ("convector", "convector"), ("top-grille", "top-grille")):
    s.thof(f"th-{nm}", "radiator-body", reg)
for nm in ("valve-head", "valve-body", "valve-body-right"):
    s.thof(f"th-valve-{nm}", nm, "valve")
s.th(cyl("th-emitter-convection-outlet", R, PS, 0.5, "x", [0.585, -0.05, 0], 0.03, 0.047, tags=["convection-outlet"]), "top-grille")
s.th(cyl("th-emitter-front-plume", R, PS, 0.5, "y", [0.3, -0.085, 0], 0.03, 0.05, tags=["front-plume"]), "panel-front-mid")
s.stof("member-panel", "radiator-body", "radiator", "member")
s.stof("joint-valve-head", "valve-head", "valve-head", "joint")
support(s, 200)
s.dec("Body: the radiator is one 600 x 100 x 1000 mm block including grille and brackets (the 1:200 drawing has one outline), the valve as knob disc, two valve blocks of 120 and 100 mm width and 30 mm pipes (pipes thickened to the feature); this model is already minimal.",
      "Körper: der Heizkörper ist ein Block 600 x 100 x 1000 mm einschließlich Gitter und Konsolen (die Zeichnung 1:200 hat eine Kontur), das Ventil als Knopfscheibe, zwei Ventilblöcke von 120 und 100 mm Breite und Rohre von 30 mm (auf die Einzelheit verdickt); dieses Modell ist bereits minimal.")
s.dec("Thermal: every region references the block (the zones are not separable at 30 mm); emitters 30 mm discs. Structure: block member and valve joint, brackets merged into the block, support at 30 mm.",
      "Thermisch: jede Region verweist auf den Block (die Zonen sind bei 30 mm nicht trennbar); Emitter Scheiben von 30 mm. Tragwerk: Blockstab und Ventilgelenk, Konsolen im Block aufgegangen, Auflager mit 30 mm.")
sc.append(s)
save("components", "panel-radiator", sc)
