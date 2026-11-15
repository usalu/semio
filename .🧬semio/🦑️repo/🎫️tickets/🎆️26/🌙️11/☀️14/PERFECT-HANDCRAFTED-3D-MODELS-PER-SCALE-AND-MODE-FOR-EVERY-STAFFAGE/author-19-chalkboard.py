import os, sys
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-19-lib.py")).read())
F, TR, L = "frame", "tray", "leaf"
AL, EN, MD, CH, SP = "aluminium-frame", "enamel-board", "mdf-core", "chalk", "sponge"
PT = 0.0172


def hollow_bar(id, bone, tone, axis, a0, a1, rect_ab, rect_cd, wall=0.002):
    (p0, p1), (q0, q1) = rect_ab, rect_cd
    prof = [[p0, q0], [p1, q0], [p1, q1], [p0, q1]]
    hole = [[p0 + wall, q0 + wall], [p1 - wall, q0 + wall], [p1 - wall, q1 - wall], [p0 + wall, q1 - wall]]
    plane = {"x": "yz", "y": "zx", "z": "xy"}[axis]
    return ext(id, bone, AL, tone, plane, prof, a0, a1, holes=[hole])


def tof(s, id, of, region, layer=None, faces=None):
    d = {"id": id, "of": of, "region": region}
    if layer:
        d["layer"] = layer
    if faces:
        d["sectionFaces"] = faces
    s.thermal.append(d)


sc = []
# ---- 1:50
s = Scale(50)
ring_holes = [rect(-0.99, 0.66, 1.95, 1.13), rect(0.99, 0.66, 1.95, 1.13)]
s.b(ext("frame-ring", F, AL, 0.75, "xy", rect(0, 0.66, 4, 1.2), -0.005, 0.04, holes=ring_holes))
for nm, sg in (("left", -1), ("right", 1)):
    d = s.b(box(f"panel-{nm}", F, EN, 0.3, [1.954, 1.134, PT], [sg * 0.99, 0.66, PT / 2]))
    pass
s.b(box("top-rail", F, AL, 0.5, [3.976, 0.048, 0.0076], [0, 1.224, -0.0012]))
s.b(ext("leaf-ring", L, AL, 0.85, "xy", rect(-1, 0.6, 2, 1.2), -0.04, -0.005, holes=[rect(-1, 0.6, 1.93, 1.13)]))
d = s.b(box("leaf-face", L, EN, 0.24, [1.934, 1.134, PT], [-1, 0.6, -0.0345 + PT / 2]))
pass
s.b(box("leaf-top-rail", L, AL, 0.55, [1.976, 0.043, 0.027], [-1, 1.1665, -0.0225]))
for nm, x in (("a", -1.54), ("b", -0.34)):
    s.b(box(f"leaf-grip-{nm}", L, AL, 0.4, [0.12, 0.02, 0.0075], [x, 0.058, -0.03625], 0.002, role="detail"))
for nm, x in (("a", -1.7), ("b", -0.3)):
    s.b(revolve(f"hanger-{nm}", L, AL, 0.2, [0, 0, 1], [x, 1.166, -0.04], [[0, 0], [0, 0.022], [0.01, 0.022], [0.01, 0], [0, 0]]) if False else revolve(f"hanger-{nm}", L, AL, 0.2, "z", [x, 1.166, -0.04], [[0, 0], [0, 0.022], [0.01, 0.022], [0.01, 0], [0, 0]]))
tray_prof = [[0, -0.04], [0, 0.04], [0.06, 0.04], [0.06, 0.032], [0.022, 0.032], [0.022, -0.032], [0.06, -0.032], [0.06, -0.04]]
s.b(ext("tray", TR, AL, 0.65, "yz", tray_prof, -2, 2))
for nm, (x0, x1, z) in (("a", (-1.293, -1.217, -0.012)), ("b", (-1.143, -1.087, 0.006)), ("c", (-1.043, -0.977, -0.003))):
    s.b(cyl(f"chalk-{nm}", TR, CH, 0.85, "x", [(x0 + x1) / 2, 0.029, z], x1 - x0 + 0.014, 0.007, role="detail"))
s.b(box("sponge", TR, SP, 0.6, [0.14, 0.036, 0.05], [0.97, 0.04, 0], 0.004, role="detail"))
s.b(box("ruler", TR, CH, 0.55, [0.9, 0.012, 0.04], [-0.25, 0.028, 0.004], role="detail"))

def core_of(b, k=0.0006):
    sz = b["size"]
    c = dict(b)
    c["size"] = [sz[0] - 2 * k, sz[1] - 2 * k, sz[2] - 2 * k]
    c["material"] = MD
    c["tone"] = 0.5
    c.pop("sectionFaces", None)
    return c


for nm in ("left", "right"):
    pb = [b for b in s.body if b["id"] == f"panel-{nm}"][0]
    c = core_of(pb)
    c["id"] = f"core-{nm}"
    s.th(c, "board-face", "core")
    tof(s, f"enamel-{nm}", f"panel-{nm}", "board-face", "enamel", ["+w"])
lb = [b for b in s.body if b["id"] == "leaf-face"][0]
c = core_of(lb)
c["id"] = "leaf-core"
s.th(c, "leaf", "core")
tof(s, "leaf-enamel", "leaf-face", "leaf", "enamel", ["+w"])
tof(s, "th-frame-ring", "frame-ring", "frame", None, ["+w"])
tof(s, "th-top-rail", "top-rail", "frame")
for i in ("leaf-ring", "leaf-top-rail", "leaf-grip-a", "leaf-grip-b", "hanger-a", "hanger-b"):
    s.thof(f"th-{i}", i, "leaf")
for i in ("tray", "chalk-a", "chalk-b", "chalk-c", "sponge", "ruler"):
    s.thof(f"th-{i}", i, "tray")

s.st(hollow_bar("frame-top-bar", F, 0.75, "x", -2, 2, (1.225, 1.26), (-0.005, 0.04)), F, "member")
s.st(hollow_bar("frame-bottom-bar", F, 0.75, "x", -2, 2, (0.06, 0.095), (-0.005, 0.04)), F, "member")
for nm, x0, x1 in (("stile-l", -2, -1.965), ("stile-r", 1.965, 2), ("mullion", -0.015, 0.015)):
    s.st(hollow_bar(f"frame-{nm}", F, 0.75, "y", 0.095, 1.225, (-0.005, 0.04), (x0, x1)), F, "member")
s.stof("panel-left-member", "panel-left", F, "member")
s.stof("panel-right-member", "panel-right", F, "member")
s.stof("tray-member", "tray", TR, "member")
s.st(hollow_bar("leaf-top-bar", L, 0.85, "x", -2, 0, (1.165, 1.2), (-0.04, -0.005)), L, "member")
s.st(hollow_bar("leaf-bottom-bar", L, 0.85, "x", -2, 0, (0, 0.035), (-0.04, -0.005)), L, "member")
for nm, x0, x1 in (("stile-l", -2, -1.965), ("stile-r", -0.035, 0)):
    s.st(hollow_bar(f"leaf-{nm}", L, 0.85, "y", 0.035, 1.165, (-0.04, -0.005), (x0, x1)), L, "member")
s.stof("leaf-panel-member", "leaf-face", L, "member")
s.stof("hanger-a-joint", "hanger-a", L, "joint")
s.stof("hanger-b-joint", "hanger-b", L, "joint")
for nm, x in (("a", -1.5), ("b", 1.5)):
    s.st(box(f"tray-clip-{nm}", TR, AL, 0.4, [0.04, 0.053, 0.04], [x, 0.0485, 0]), TR, "joint")
hook = [[1.19, 0.0075], [1.21, 0.0075], [1.21, 0.055], [1.2, 0.062], [1.19, 0.055]]
s.st(ext("support-wall-hook-pin", F, AL, 0.15, "yz", hook, -0.15, 0.15), F, "support")
s.st(cyl("support-wall-rest-roller", F, AL, 0.15, "x", [0, 0.1, 0.0155], 0.3, 0.0085), F, "support")
sc.append(s)


def common_coarse(s, bw, hole_x, hole_y0, hole_y1, y0, depth_panel, ring_hole_f, leaf_hole):
    pass


# ---- 1:100
s = Scale(100)
s.b(ext("frame-ring", F, AL, 0.75, "xy", rect(0, 0.66, 4, 1.2), -0.005, 0.04, holes=[rect(-0.99, 0.66, 1.94, 1.12), rect(0.99, 0.66, 1.94, 1.12)]))
for nm, sg in (("left", -1), ("right", 1)):
    s.b(box(f"panel-{nm}", F, EN, 0.3, [1.944, 1.124, 0.03], [sg * 0.99, 0.66, 0.015]))
s.b(box("top-rail", F, AL, 0.5, [3.96, 0.05, 0.02], [0, 1.215, 0.005]))
s.b(ext("leaf-ring", L, AL, 0.85, "xy", rect(-1, 0.6, 2, 1.2), -0.04, -0.005, holes=[rect(-1, 0.6, 1.92, 1.12)]))
s.b(box("leaf-face", L, EN, 0.24, [1.924, 1.124, 0.02], [-1, 0.6, -0.022]))
s.b(box("leaf-top-rail", L, AL, 0.55, [1.96, 0.045, 0.027], [-1, 1.1575, -0.0225]))
tray100 = [[0, -0.04], [0, 0.04], [0.06, 0.04], [0.06, 0.025], [0.022, 0.025], [0.022, -0.025], [0.06, -0.025], [0.06, -0.04]]
s.b(ext("tray", TR, AL, 0.65, "yz", tray100, -2, 2))
tof(s, "th-frame-ring", "frame-ring", "frame", None, ["+w"])
tof(s, "th-top-rail", "top-rail", "frame")
for nm in ("left", "right"):
    tof(s, f"th-panel-{nm}", f"panel-{nm}", "board-face", None, ["+w"])
tof(s, "th-leaf-ring", "leaf-ring", "leaf")
tof(s, "th-leaf-face", "leaf-face", "leaf", None, ["+w"])
tof(s, "th-leaf-top-rail", "leaf-top-rail", "leaf")
tof(s, "th-tray", "tray", "tray")
s.st(hollow_bar("frame-top-bar", F, 0.75, "x", -2, 2, (1.22, 1.26), (-0.005, 0.04), 0.0025), F, "member")
s.st(hollow_bar("frame-bottom-bar", F, 0.75, "x", -2, 2, (0.06, 0.1), (-0.005, 0.04), 0.0025), F, "member")
for nm, x0, x1 in (("stile-l", -2, -1.96), ("stile-r", 1.96, 2), ("mullion", -0.02, 0.02)):
    s.st(hollow_bar(f"frame-{nm}", F, 0.75, "y", 0.1, 1.22, (-0.005, 0.04), (x0, x1), 0.0025), F, "member")
for nm in ("left", "right"):
    s.stof(f"panel-{nm}-member", f"panel-{nm}", F, "member")
s.stof("tray-member", "tray", TR, "member")
s.st(hollow_bar("leaf-top-bar", L, 0.85, "x", -2, 0, (1.16, 1.2), (-0.04, -0.005), 0.0025), L, "member")
s.st(hollow_bar("leaf-bottom-bar", L, 0.85, "x", -2, 0, (0, 0.04), (-0.04, -0.005), 0.0025), L, "member")
for nm, x0, x1 in (("stile-l", -2, -1.96), ("stile-r", -0.04, 0)):
    s.st(hollow_bar(f"leaf-{nm}", L, 0.85, "y", 0.04, 1.16, (-0.04, -0.005), (x0, x1), 0.0025), L, "member")
s.stof("leaf-panel-member", "leaf-face", L, "member")
s.stof("leaf-runner-joint", "leaf-top-rail", L, "joint")
for nm, x in (("a", -1.5), ("b", 1.5)):
    s.st(box(f"tray-clip-{nm}", TR, AL, 0.4, [0.04, 0.053, 0.04], [x, 0.0485, 0]), TR, "joint")
s.st(ext("support-wall-hook-pin", F, AL, 0.15, "yz", [[1.19, 0.0075], [1.215, 0.0075], [1.215, 0.055], [1.2025, 0.062], [1.19, 0.055]], -0.15, 0.15), F, "support")
s.st(cyl("support-wall-rest-roller", F, AL, 0.15, "x", [0, 0.1, 0.0165], 0.3, 0.0095), F, "support")
sc.append(s)

# ---- 1:200
s = Scale(200)
s.b(ext("frame-ring", F, AL, 0.75, "xy", rect(0, 0.665, 4, 1.19), -0.005, 0.04, holes=[rect(-0.99, 0.665, 1.94, 1.11), rect(0.99, 0.665, 1.94, 1.11)]))
for nm, sg in (("left", -1), ("right", 1)):
    s.b(box(f"panel-{nm}", F, EN, 0.3, [1.944, 1.114, 0.032], [sg * 0.99, 0.665, 0.016]))
s.b(box("leaf-slab", L, AL, 0.35, [2, 1.19, 0.035], [-1, 0.605, -0.0225]))
s.b(box("tray", TR, AL, 0.65, [4, 0.07, 0.08], [0, 0.035, 0]))
tof(s, "th-frame-ring", "frame-ring", "frame", None, ["+w"])
for nm in ("left", "right"):
    tof(s, f"th-panel-{nm}", f"panel-{nm}", "board-face", None, ["+w"])
tof(s, "th-leaf-slab", "leaf-slab", "leaf", None, ["+w"])
tof(s, "th-tray", "tray", "tray")
s.stof("frame-member", "frame-ring", F, "member")
for nm in ("left", "right"):
    s.stof(f"panel-{nm}-member", f"panel-{nm}", F, "member")
s.stof("tray-member", "tray", TR, "member")
s.stof("leaf-member", "leaf-slab", L, "member")
s.st(box("leaf-runner-joint", L, AL, 0.4, [1.9, 0.032, 0.032], [-1, 1.17, -0.0225]), L, "joint")
s.st(box("support-wall-hook-pin", F, AL, 0.15, [0.3, 0.032, 0.0545], [0, 1.2, 0.03475]), F, "support")
s.st(cyl("support-wall-rest-roller", F, AL, 0.15, "x", [0, 0.1, 0.0165], 0.3, 0.0165), F, "support")
sc.append(s)

# ---- 1:500
s = Scale(500)
s.b(box("frame", F, AL, 0.5, [4, 1.19, 0.08], [0, 0.665, 0]))
s.b(box("leaf", L, AL, 0.35, [2, 1.19, 0.04], [-1, 0.605, -0.02]))
s.b(box("tray", TR, AL, 0.65, [4, 0.075, 0.08], [0, 0.0375, 0]))
tof(s, "th-frame-rim", "frame", "frame", None, ["+w", "-w"])
tof(s, "th-panel-face", "frame", "board-face", None, ["+w"])
tof(s, "th-leaf", "leaf", "leaf", None, ["+w"])
tof(s, "th-tray", "tray", "tray")
s.stof("frame-member", "frame", F, "member")
s.stof("tray-member", "tray", TR, "member")
s.stof("leaf-member", "leaf", L, "member")
s.st(box("support-wall-hook-pin", F, AL, 0.15, [0.3, 0.076, 0.076], [0, 1.2, 0.0455]), F, "support")
s.st(box("support-wall-rest-roller", F, AL, 0.15, [0.3, 0.076, 0.076], [0, 0.1, 0.0455]), F, "support")
sc.append(s)

DEC = {
50: [
("Body: the 4 m x 1.2 m board as the 1:50 drawing builds it: the 35 mm aluminium frame ring (45 mm deep, 30 mm centre mullion) extruded from the drawn outline with its two panel openings, two 17.2 mm ceramic-steel panels (0.6 mm skins around a 16 mm MDF core) seated 5 mm behind the front rim and overlapping the frame by 2 mm, the 48 mm guide rail on the top bar, the sliding leaf in front (own frame ring 35 mm deep, 17.2 mm panel, top rail, two flush finger pulls of 7.5 mm, two 44 mm hanger rollers) shown in the leaf-left pose, the chalk tray as the U trough of the drawing (22 mm floor, 8 mm walls) with three chalk cylinders, the sponge and the ruler. The grips and rollers are now 7.5 mm thick (before: 2.5 and 3.5 mm, below the feature) and sit flush with the leaf front instead of protruding. The aluminium profile is modelled as its closed envelope: its 2 mm wall is below the feature and no drawing shows the cavity.",
 "Körper: die Tafel 4 m x 1,2 m, wie die Zeichnung 1:50 sie baut: der Alurahmen 35 mm (45 mm tief, 30 mm Mittelsteg) als Ring aus dem gezeichneten Umriss mit den beiden Plattenöffnungen extrudiert, zwei Keramikstahlplatten von 17,2 mm (Deckbleche 0,6 mm um einen MDF-Kern von 16 mm), 5 mm hinter dem Vorderrand eingesetzt und 2 mm in den Rahmen überlappend, die Führungsschiene 48 mm auf dem oberen Holm, davor der Schiebeflügel (eigener Rahmenring 35 mm tief, Platte 17,2 mm, Oberschiene, zwei bündige Griffmulden von 7,5 mm, zwei Laufrollen von 44 mm) in der Pose Flügel links, die Kreideablage als U-Trog der Zeichnung (Boden 22 mm, Wände 8 mm) mit drei Kreidezylindern, Schwamm und Lineal. Griffe und Rollen sind jetzt 7,5 mm dick (vorher 2,5 und 3,5 mm, unter der Einzelheit) und liegen bündig mit der Flügelfront statt vorzustehen. Das Aluprofil ist als geschlossene Hülle modelliert: seine Wand von 2 mm liegt unter der Einzelheit, und keine Zeichnung zeigt den Hohlraum."),
("Thermal: board-face and leaf are nested shells per layer (innermost first, after the layer order of both regions was corrected in the asset to core, then enamel): the 16 mm MDF core inside the full 17.2 mm panel whose skin is the 0.6 mm enamel; the enamel shell is the body panel, only its front face counts (the hidden back is a section face). frame, the leaf's aluminium parts and the tray with its chalk, sponge and ruler reference the body. The frame area 0.8 m2 was a mis-estimate against the modelled ring and is corrected to 1.1 m2 (1:50 exposes 1.15 m2, ratio 1.4); the tray 0.32 m2 is the plan footprint, the trough exposes 1.34 m2 (ratio 4.2): not applied, listed in area-review.md, the tray area stays outside 15 %.",
 "Thermisch: board-face und leaf sind verschachtelte Schalen je Schicht (innen zuerst, nachdem die Schichtreihenfolge beider Regionen im Asset auf core, dann enamel berichtigt wurde): der MDF-Kern von 16 mm in der vollen Platte von 17,2 mm, deren Haut das Emaille von 0,6 mm ist; die Emailleschale ist die Körperplatte, nur ihre Vorderseite zählt (die verdeckte Rückseite ist eine Schnittfläche). Rahmen, die Aluteile des Flügels und die Ablage mit Kreide, Schwamm und Lineal verweisen auf den Körper. Die Rahmenfläche 0,8 m2 war eine Fehlschätzung gegenüber dem modellierten Ring und ist auf 1,1 m2 berichtigt (1:50 zeigt 1,15 m2, Verhältnis 1,4); die Ablage mit 0,32 m2 ist die Grundrissfläche, der Trog zeigt 1,34 m2 (Verhältnis 4,2): nicht angewandt, in area-review.md aufgeführt, die Ablagefläche bleibt außerhalb von 15 %."),
("Structure: the load path of the 72 kg board: the frame carries the panels, so the member solids are the five hollow aluminium bars of the frame (top, bottom, two stiles, mullion; 35 x 45 mm, 2 mm wall, real extruded sections) and the two panels (referenced, they are the shear skins), the leaf has four hollow bars and its panel, the tray is its own trough section (referenced). Joints are the two hanger rollers (the prismatic joint of the leaf) and two tray clips between tray and frame; the supports are the pin (hanging rail, a wedge-ended rail 0.3 m wide on the upper bar) and the roller (a 17 mm buffer cylinder 0.3 m wide at the lower bar); both reach the elevation plane where the anchors lie. The masses are carried by the panels, so there are no extra mass solids.",
 "Tragwerk: der Lastpfad der Tafel von 72 kg: der Rahmen trägt die Platten, daher sind die Stabkörper die fünf hohlen Alustäbe des Rahmens (oben, unten, zwei Pfosten, Mittelsteg; 35 x 45 mm, Wand 2 mm, echte extrudierte Profile) und die beiden Platten (referenziert, sie sind die Schubhäute), der Flügel hat vier Hohlstäbe und seine Platte, die Ablage ist ihr eigenes Trogprofil (referenziert). Gelenke sind die zwei Laufrollen (das Schubgelenk des Flügels) und zwei Ablagehalter zwischen Ablage und Rahmen; die Auflager sind der Bolzen (Hängeschiene, eine Schiene mit keilförmigem Ende, 0,3 m breit am oberen Holm) und die Rolle (ein Pufferzylinder von 17 mm, 0,3 m breit am unteren Holm); beide reichen bis zur Aufrissebene, in der die Anker liegen. Die Massen tragen die Platten, daher gibt es keine zusätzlichen Massenkörper."),
],
100: [
("Body: the same parts merged to what 1:100 draws: frame ring with 40 mm borders, two 30 mm panels (the drawn depth), the guide rail 20 mm deep (the 5 mm rail is below the 15 mm feature), leaf ring, leaf panel 20 mm and leaf rail, the tray as a U trough with 15 mm walls (the feature). Finger pulls, hanger rollers, chalk, sponge and ruler are gone (not drawn, below 15 mm or merged).",
 "Körper: dieselben Teile zusammengefasst auf das, was 1:100 zeichnet: Rahmenring mit Rändern von 40 mm, zwei Platten von 30 mm (die gezeichnete Tiefe), die Führungsschiene 20 mm tief (die 5 mm der Schiene liegen unter der Einzelheit 15 mm), Flügelring, Flügelplatte 20 mm und Flügelschiene, die Ablage als U-Trog mit Wänden von 15 mm (die Einzelheit). Griffmulden, Laufrollen, Kreide, Schwamm und Lineal entfallen (nicht gezeichnet, unter 15 mm oder zusammengefasst)."),
("Thermal: all four regions reference the body solids (single skin per region at this scale); the panel and leaf backs are section faces. The tray still measures 3 times its declared area (area-review.md).",
 "Thermisch: alle vier Regionen verweisen auf Körper (je eine Haut pro Region bei diesem Maßstab); die Rückseiten von Platte und Flügel sind Schnittflächen. Die Ablage misst weiter das Dreifache ihrer erklärten Fläche (area-review.md)."),
("Structure: as at 1:50 with 40 mm hollow bars (2.5 mm wall), the two hanger rollers merged into the leaf rail as the runner joint, both tray clips kept, supports 20 mm and 19 mm thick.",
 "Tragwerk: wie bei 1:50 mit Hohlstäben von 40 mm (Wand 2,5 mm), die zwei Laufrollen in der Flügelschiene als Laufgelenk zusammengefasst, beide Ablagehalter behalten, Auflager 20 mm und 19 mm dick."),
],
200: [
("Body: the 30 mm feature takes away the borders' detail: frame ring (40 mm borders) with two 32 mm panels, the leaf as one 35 mm slab, the tray as one 70 x 80 mm block (the drawing shows it as a filled rectangle), no rail, no tray contents.",
 "Körper: die Einzelheit 30 mm nimmt den Rändern das Detail: Rahmenring (Ränder 40 mm) mit zwei Platten von 32 mm, der Flügel als eine Platte von 35 mm, die Ablage als ein Block 70 x 80 mm (die Zeichnung zeigt ein gefülltes Rechteck), keine Schiene, kein Ablageinhalt."),
("Thermal: the four regions reference the ring, the panels, the leaf slab and the tray block; only the total over all regions is compared at this scale.",
 "Thermisch: die vier Regionen verweisen auf Ring, Platten, Flügelplatte und Ablageblock; bei diesem Maßstab wird nur die Summe über alle Regionen verglichen."),
("Structure: hollow bars are merged: the frame ring, panels, leaf slab and tray block are the members (referenced), one 32 mm runner joint along the leaf top, supports 32 mm and 33 mm.",
 "Tragwerk: die Hohlstäbe sind zusammengefasst: Rahmenring, Platten, Flügelplatte und Ablageblock sind die Stäbe (referenziert), ein Laufgelenk von 32 mm am Flügelkopf, Auflager 32 mm und 33 mm."),
],
500: [
("Body: three masses, the 80 mm deep frame block 4.0 x 1.19 m, the 40 mm leaf block in front of it (the drawing gives the leaf 40 mm, below the 75 mm feature; its thinnest drawn extent is its limit, C13) and the tray as one 75 mm block (the 1:500 drawing is a line at the tray height; the block fills the front view and the pen band of that line; 75 mm is the feature).",
 "Körper: drei Massen, der 80 mm tiefe Rahmenblock 4,0 x 1,19 m, der 40 mm tiefe Flügelblock davor (die Zeichnung gibt dem Flügel 40 mm, unter der Einzelheit 75 mm; seine dünnste gezeichnete Ausdehnung ist seine Grenze, C13) und die Ablage als ein Block von 75 mm (die Zeichnung 1:500 ist eine Linie in Ablagehöhe; der Block füllt die Ansicht und das Stiftband dieser Linie; 75 mm ist die Einzelheit)."),
("Thermal: board-face references the frame block (front face only), frame the same block's rim (front and back are section faces, so the 4 edges count), leaf and tray reference the leaf block and the tray block; only the total area is compared.",
 "Thermisch: board-face verweist auf den Rahmenblock (nur die Vorderseite), frame auf dessen Rand (Vorder- und Rückseite sind Schnittflächen, es zählen die vier Kanten), leaf und tray auf Flügelblock und Ablageblock; verglichen wird nur die Gesamtfläche."),
("Structure: the three bodies are the members (referenced), supports are two 76 mm blocks at the anchors.",
 "Tragwerk: die drei Körper sind die Stäbe (referenziert), Auflager sind zwei Blöcke von 76 mm an den Ankern."),
],
}
for _s in sc:
    for en, de in DEC[_s.den]:
        _s.dec(en, de)
save_asset("chalkboard", sc)
