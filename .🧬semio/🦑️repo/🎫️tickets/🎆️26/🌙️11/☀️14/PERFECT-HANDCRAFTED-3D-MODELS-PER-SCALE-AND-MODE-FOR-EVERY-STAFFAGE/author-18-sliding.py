import sys, os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-18-lib.py")).read())
ST, GL, CV = "steel", "glass", "frame-cavity"
JS, TS_, LF, LG = "jamb-steel", "track-steel", "leaf-frame", "leaf-glass"
only = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]
YJ = -1.075
LAY = {"a": "skin-a", "c": "cavity", "b": "skin-b"}


def jamb_profile(sign):
    pts = [(-0.05, -0.05), (0.05, -0.05), (0.05, -0.04), (0.03, -0.04), (0.03, 0.04), (0.05, 0.04), (0.05, 0.05), (-0.05, 0.05)]
    return [[z, sign * x] for x, z in pts][::sign]


def jambs50(s):
    for sd, x, sg in (("west", 0.0, 1), ("east", 3.0, -1)):
        pr = [[z, x + sg * px] for px, z in [(-0.05, -0.05), (0.05, -0.05), (0.05, -0.04), (0.03, -0.04), (0.03, 0.04), (0.05, 0.04), (0.05, 0.05), (-0.05, 0.05)]]
        if sg < 0:
            pr = pr[::-1]
        s.b(ext(f"jamb-{sd}", "track", ST, 0.4, "zx", pr, -2.1, -0.05, role="body"))
    for sd, x in (("west", 0.035), ("east", 2.965)):
        d = box(f"jamb-{sd}-seal", "track", ST, 0.15, [0.009, 2.05, 0.08], [x, YJ, 0], role="detail")
        d["slender"] = True
        s.b(d)


def track50(s):
    pr = [[-0.05, -0.042], [-0.05, -0.027], [-0.01, -0.027], [-0.01, 0.027], [-0.05, 0.027], [-0.05, 0.042], [0.05, 0.042], [0.05, -0.042]]
    s.b(ext("track", "track", ST, 0.4, "yz", pr, -0.05, 3.05, role="body"))
    head = [[0, 0], [0, 0.012], [0.005, 0.012], [0.008, 0.0075], [0.008, 0]]
    for id_, x, c, t in (("a", 0.3, 2, 2.4), ("b", 1.0, 2, 1.0)):
        d = revolve(f"track-screw-{id_}", "track", ST, 0.2, [0, 0, 1], [x, 0, 0.042], head, role="detail")
        d["repeat"] = {"count": c, "translation": [t, 0, 0]}
        s.b(d)


def leaf50(s):
    for sd, v in (("w", -0.475), ("e", 0.475)):
        s.b(box(f"leaf-stile-{sd}", "leaf", ST, 0.4, [1.9, 0.05, 0.05], [1.05, v, 0]))
    s.b(box("leaf-rail-top", "leaf", ST, 0.4, [0.05, 0.9, 0.05], [0.125, 0, 0]))
    s.b(box("leaf-rail-bottom", "leaf", ST, 0.4, [0.05, 0.9, 0.05], [1.975, 0, 0]))
    s.b(box("leaf-glass", "leaf", GL, 0.8, [1.8, 0.9, 0.01], [1.05, 0, 0], role="glazing"))
    for sd, v in (("a", -0.16), ("b", 0.16)):
        s.b(box(f"hanger-{sd}", "leaf", ST, 0.3, [0.085, 0.04, 0.05], [0.0575, v, 0]))
        s.b(revolve(f"hanger-bolt-{sd}", "leaf", ST, 0.2, [0, 0, 1], [0.075, v, -0.028], [[0, 0], [0, 0.008], [0.056, 0.008], [0.056, 0]], role="detail"))
    s.b(box("handle-base", "leaf", ST, 0.3, [0.29, 0.06, 0.008], [1.025, 0.42, 0.009], repeat={"count": 2, "translation": [0, 0, -0.018]}))
    s.b(box("handle-grip", "leaf", ST, 0.25, [0.19, 0.03, 0.02], [1.025, 0.415, 0.023], repeat={"count": 2, "translation": [0, 0, -0.046]}))
    s.b(box("bottom-seal", "leaf", ST, 0.15, [0.02, 1.0, 0.05], [2.01, 0, 0], role="detail"))


def nest(s, region, id_, outer_of, inner_boxes):
    s.thof(f"th-{id_}-b", outer_of, region, "skin-b")
    for k, (sz, at) in zip(("c", "a"), inner_boxes):
        s.th(box(f"th-{id_}-{k}", "track" if region in (JS, TS_) else "leaf", CV if k == "c" else ST, 0.85 if k == "c" else 0.4, sz, at), region, LAY[k])


def build50():
    s = Scale(50)
    jambs50(s)
    track50(s)
    leaf50(s)
    for sd, x in (("west", 0.0), ("east", 3.0)):
        nest(s, JS, f"jamb-{sd}", f"jamb-{sd}", [([0.092, 2.042, 0.092], [x, YJ, 0]), ([0.008, 1.958, 0.008], [x, YJ, 0])])
    nest(s, TS_, "track", "track", [([3.092, 0.052, 0.076], [1.5, 0.02, 0]), ([3.024, 0.024, 0.008], [1.5, 0.02, 0])])
    for sd, v in (("w", -0.475), ("e", 0.475)):
        nest(s, LF, f"leaf-stile-{sd}", f"leaf-stile-{sd}", [([1.892, 0.042, 0.042], [1.05, v, 0]), ([1.892, 0.016, 0.016], [1.05, v, 0])])
    for nm, u in (("top", 0.125), ("bottom", 1.975)):
        nest(s, LF, f"leaf-rail-{nm}", f"leaf-rail-{nm}", [([0.042, 0.9, 0.042], [u, 0, 0]), ([0.016, 0.9, 0.016], [u, 0, 0])])
    for i in ("hanger-a", "hanger-b", "handle-base", "handle-grip", "bottom-seal"):
        s.thof(f"th-{i}", i, LF, "skin-b")
    g = box("th-glass", "leaf", GL, 0.8, [1.8, 0.9, 0.01], [1.05, 0, 0])
    g["sectionFaces"] = ["-w"]
    s.th(g, LG, "glass-pane")
    for sd in ("west", "east"):
        pass
    tube = lambda w, h, t: (rect(0, 0, h, w), rect(0, 0, h - 2 * t, w - 2 * t))
    for sd, x in (("west", 0.0), ("east", 3.0)):
        o, i = tube(0.1, 0.1, 0.004)
        d = ext(f"m-jamb-{sd}", "track", ST, 0.5, "zx", [[p[0], p[1] + x] for p in o], -2.1, -0.05, holes=[[[p[0], p[1] + x] for p in i]])
        s.st(d, "track", "member")
    o, i = tube(0.05, 0.1, 0.004)
    s.st(ext("m-track", "track", ST, 0.5, "yz", rect(0, 0, 0.1, 0.05), -0.05, 3.05, holes=[rect(0, 0, 0.092, 0.042)]), "track", "member")
    for sd, v in (("w", -0.475), ("e", 0.475)):
        s.st(ext(f"m-leaf-stile-{sd}", "leaf", ST, 0.5, "yz", rect(v, 0, 0.05, 0.05), 0.1, 2.0, holes=[rect(v, 0, 0.042, 0.042)]), "leaf", "member")
    for nm, u in (("top", 0.125), ("bottom", 1.975)):
        s.st(ext(f"m-leaf-rail-{nm}", "leaf", ST, 0.5, "zx", rect(0, u, 0.05, 0.05), -0.45, 0.45, holes=[rect(0, u, 0.042, 0.042)]), "leaf", "member")
    s.stof("j-carriage-a", "hanger-a", "leaf", "joint")
    s.stof("j-carriage-b", "hanger-b", "leaf", "joint")
    s.st(cyl("support-west-jamb", "track", ST, 0.4, "z", [0, 0, 0], 0.12, 0.015), "track", "support")
    s.st(cyl("support-east-jamb", "track", ST, 0.4, "z", [3.0, 0, 0], 0.12, 0.02), "track", "support")
    s.dec("Body: the door as built. Two 100 x 100 mm jamb posts with the rebate for the brush seal (slender strip in it, as the 2D draws a line), the 100 mm track housing with its running slot (the housing is 84 mm deep so that the four 8 mm domed screw heads keep the door inside its 100 mm depth), the leaf as four 50 x 50 mm hollow-section members (stiles, top and bottom rail) round the 10 mm pane, two carriage hangers that reach 35 mm up into the track slot with their bolts, a pull handle on both faces (plate and stand-off grip) and the 20 mm bottom seal. The hollow sections are solid in the body (they are closed boxes seen from outside); their real wall is in the thermal and structure lists.",
          "Körper: das Tor, wie es gebaut ist. Zwei Pfosten 100 x 100 mm mit dem Falz für die Bürstendichtung (darin ein schlanker Streifen, wie ihn die 2D als Linie zeichnet), das 100-mm-Laufschienengehäuse mit seinem Laufschlitz (das Gehäuse ist 84 mm tief, damit die vier gewölbten Schraubenköpfe zu 8 mm das Tor in seiner Tiefe von 100 mm halten), der Flügel als vier Hohlprofile 50 x 50 mm (Pfosten, oberer und unterer Riegel) um die 10-mm-Scheibe, zwei Laufwagenhänger, die 35 mm in den Schlitz der Schiene reichen, mit ihren Bolzen, ein Griff auf beiden Seiten (Platte und abgesetzter Haltegriff) und die 20-mm-Bodendichtung. Die Hohlprofile sind im Körper massiv (von außen geschlossene Kästen); ihre wirkliche Wand steht in Thermik und Tragwerk.")
    s.dec("Thermal: every steel member is nested as its layers: outer skin (the body solid), cavity inset by the 4 mm skin, inner skin as the core that remains of the profile (the 50 mm cavity of the layer table does not fit a 100 mm or 50 mm profile, so the cavity is modelled at 42 / 34 mm and the inner skin as the 8 / 16 mm core). Glass: one 10 mm pane whose room face carries no area (`sectionFaces` -w), the declared 1.62 m2 is the pane area.",
          "Thermisch: jedes Stahlteil ist als seine Schichten geschachtelt: Außenhaut (der Körper), Hohlraum um die 4-mm-Haut eingerückt, Innenhaut als Kern, der vom Profil bleibt (der 50-mm-Hohlraum der Schichttabelle passt nicht in ein Profil von 100 oder 50 mm, daher ist der Hohlraum mit 42 / 34 mm und die Innenhaut als 8-/16-mm-Kern modelliert). Glas: eine Scheibe von 10 mm, deren Raumseite keine Fläche trägt (`sectionFaces` -w), die deklarierten 1,62 m2 sind die Scheibenfläche.")
    s.dec("Structure: real hollow sections (4 mm wall): the track RHS 100 x 50 x 4 over the full length (A 0.001136 m2 as the bone section), the two jamb posts 100 x 100 x 4, the leaf stiles and rails 50 x 50 x 4; the two hangers are the prismatic carriage joint; supports by type: a 30 mm pin through the west jamb head, a 40 mm roller at the east jamb head.",
          "Tragwerk: wirkliche Hohlprofile (Wand 4 mm): das Schienen-RHS 100 x 50 x 4 über die volle Länge (A 0,001136 m2 wie der Knochenquerschnitt), die zwei Pfosten 100 x 100 x 4, Pfosten und Riegel des Flügels 50 x 50 x 4; die beiden Hänger sind das Gleitgelenk des Laufwagens; Auflager nach Typ: ein Bolzen von 30 mm durch den Kopf des westlichen Pfostens, eine Rolle von 40 mm am Kopf des östlichen Pfostens.")
    return s


def portal_member_structure(s, jb, track_h):
    pass


def leaf_window(s, u0, hole, depth, hid="leaf-frame"):
    outer = [[u0, -0.5], [2.0, -0.5], [2.0, 0.5], [u0, 0.5]]
    s.b(ext(hid, "leaf", ST, 0.4, "xy", outer, -depth / 2, depth / 2, holes=[[[hole[0], -hole[2]], [hole[1], -hole[2]], [hole[1], hole[2]], [hole[0], hole[2]]]]))


def common_struct(s, hanger_ids, leaf_of=None, hollow=True):
    for sd, x in (("west", 0.0), ("east", 3.0)):
        s.stof(f"m-jamb-{sd}", f"jamb-{sd}", "track", "member")
    s.stof("m-track", "track", "track", "member")
    s.stof("m-leaf-frame", "leaf-frame", "leaf", "member")
    for i, h in enumerate(hanger_ids):
        s.stof(f"j-carriage-{i}", h, "leaf", "joint")


def build100():
    s = Scale(100)
    for sd, x in (("west", 0.0), ("east", 3.0)):
        s.b(box(f"jamb-{sd}", "track", ST, 0.4, [0.1, 2.05, 0.1], [x, YJ, 0]))
    s.b(box("track", "track", ST, 0.4, [3.1, 0.1, 0.1], [1.5, 0, 0]))
    leaf_window(s, 0.1, (0.15, 1.95, 0.45), 0.05)
    s.b(box("leaf-glass", "leaf", GL, 0.8, [1.8, 0.9, 0.01], [1.05, 0, 0], role="glazing"))
    for sd, v in (("a", -0.16), ("b", 0.16)):
        s.b(box(f"hanger-{sd}", "leaf", ST, 0.3, [0.05, 0.04, 0.05], [0.075, v, 0]))
    s.b(box("handle", "leaf", ST, 0.3, [0.25, 0.04, 0.025], [1.025, 0.42, 0.0175]))
    for id_, reg, ly in (("jamb-west", JS, "skin-b"), ("jamb-east", JS, "skin-b"), ("track", TS_, "skin-b"), ("leaf-frame", LF, "skin-b"), ("hanger-a", LF, "skin-b"), ("hanger-b", LF, "skin-b"), ("handle", LF, "skin-b")):
        s.thof(f"th-{id_}", id_, reg, ly)
    g = box("th-glass", "leaf", GL, 0.8, [1.8, 0.9, 0.015], [1.05, 0, 0.0025], role="glazing")
    g["sectionFaces"] = ["-w"]
    s.th(g, LG, "glass-pane")
    common_struct(s, ["hanger-a", "hanger-b"])
    s.st(cyl("support-west-jamb", "track", ST, 0.4, "z", [0, 0, 0], 0.12, 0.02), "track", "support")
    s.st(cyl("support-east-jamb", "track", ST, 0.4, "z", [3.0, 0, 0], 0.12, 0.025), "track", "support")
    s.dec("Body: the door at 15 mm detail. Posts and track housing as plain 100 mm boxes (rebate, seal, running slot and screws merged), the leaf frame as one extrusion with the window opening (four hollow members, bottom seal and glazing channel merged), the 10 mm pane, two 40 x 50 mm hangers and one 25 mm handle block (plate and grip merged, one face).",
          "Körper: das Tor in 15-mm-Auflösung. Pfosten und Schienengehäuse als einfache 100-mm-Quader (Falz, Dichtung, Laufschlitz und Schrauben zusammengefasst), der Flügelrahmen als eine Extrusion mit der Fensteröffnung (vier Hohlprofile, Bodendichtung und Glasfalz zusammengefasst), die 10-mm-Scheibe, zwei Hänger 40 x 50 mm und ein Griffblock von 25 mm (Platte und Griff zusammengefasst, eine Seite).")
    s.dec("Thermal: one solid per region part (the nested layer shells of 1:50 are merged to the outer skin, the layer stack is carried by the declared layers), pane as before.",
          "Thermisch: ein Körper je Regionsteil (die geschachtelten Schichtschalen von 1:50 sind zur Außenhaut zusammengefasst, der Schichtaufbau steckt in den deklarierten Schichten), Scheibe wie zuvor.")
    s.dec("Structure: posts, track housing and leaf frame are the members (the hollow sections of 1:50 merge into the outer boxes), the hangers the carriage joints, supports as pin and roller at 20 / 25 mm.",
          "Tragwerk: Pfosten, Schienengehäuse und Flügelrahmen sind die Glieder (die Hohlprofile von 1:50 gehen in die äußeren Kästen auf), die Hänger die Wagengelenke, Auflager als Bolzen und Rolle zu 20 / 25 mm.")
    return s


def build200():
    s = Scale(200)
    for sd, x in (("west", 0.0), ("east", 3.0)):
        s.b(box(f"jamb-{sd}", "track", ST, 0.4, [0.1, 2.05, 0.1], [x, YJ, 0]))
    s.b(box("track", "track", ST, 0.4, [3.1, 0.1, 0.1], [1.5, 0, 0]))
    leaf_window(s, 0.05, (0.125, 1.925, 0.425), 0.05)
    s.b(box("leaf-glass", "leaf", GL, 0.8, [1.8, 0.85, 0.02], [1.025, 0, 0], role="glazing"))
    for id_, reg in (("jamb-west", JS), ("jamb-east", JS), ("track", TS_), ("leaf-frame", LF)):
        s.thof(f"th-{id_}", id_, reg, "skin-b")
    g = box("th-glass", "leaf", GL, 0.8, [1.8, 0.85, 0.02], [1.025, 0, 0], role="glazing")
    g["sectionFaces"] = ["-w"]
    s.th(g, LG, "glass-pane")
    common_struct(s, [])
    s.st(box("j-carriage", "leaf", ST, 0.3, [0.06, 0.4, 0.05], [0.08, 0, 0]), "leaf", "joint")
    s.st(cyl("support-west-jamb", "track", ST, 0.4, "z", [0, 0, 0], 0.12, 0.03), "track", "support")
    s.st(cyl("support-east-jamb", "track", ST, 0.4, "z", [3.0, 0, 0], 0.12, 0.03), "track", "support")
    s.dec("Body: the door at 30 mm detail: posts and track housing as boxes, the leaf as a frame extrusion with its window (frame to u 0.05, hangers merged into it) and a 20 mm pane (glazing role). Handle, bottom seal and hangers are below 30 mm and merged into the frame.",
          "Körper: das Tor in 30-mm-Auflösung: Pfosten und Schienengehäuse als Quader, der Flügel als Rahmenextrusion mit seinem Fenster (Rahmen bis u 0,05, Hänger darin aufgegangen) und einer Scheibe von 20 mm (Rolle Verglasung). Griff, Bodendichtung und Hänger liegen unter 30 mm und sind im Rahmen aufgegangen.")
    s.dec("Thermal: one outer skin solid per region part, the pane with its room face left out. Structure: posts, track and leaf frame are the members, the carriage is one 60 x 400 mm block at the frame top, supports 30 mm.",
          "Thermisch: ein Außenhautkörper je Regionsteil, die Scheibe ohne ihre Raumseite. Tragwerk: Pfosten, Schiene und Flügelrahmen sind die Glieder, der Laufwagen ist ein Block 60 x 400 mm am Rahmenkopf, Auflager 30 mm.")
    return s


def build500():
    s = Scale(500)
    s.b(box("jamb-west", "track", ST, 0.4, [0.15, 2.15, 0.1], [0.025, -1.025, 0]))
    s.b(box("jamb-east", "track", ST, 0.4, [0.15, 2.15, 0.1], [2.975, -1.025, 0]))
    s.b(box("track", "track", ST, 0.4, [2.8, 0.15, 0.1], [1.5, -0.025, 0]))
    s.b(box("leaf", "leaf", GL, 0.7, [1.95, 1.0, 0.075], [1.025, 0, 0]))
    for id_, reg in (("jamb-west", JS), ("jamb-east", JS), ("track", TS_)):
        s.thof(f"th-{id_}", id_, reg, "skin-b")
    g = box("th-glass", "leaf", GL, 0.8, [1.8, 0.85, 0.075], [1.025, 0, 0], role="glazing")
    g["sectionFaces"] = ["-w"]
    s.th(g, LG, "glass-pane")
    s.th(box("th-leaf-frame", "leaf", ST, 0.4, [0.075, 1.0, 0.075], [0.0875, 0, 0]), LF, "skin-b")
    for sd, x in (("west", 0.0), ("east", 3.0)):
        s.stof(f"m-jamb-{sd}", f"jamb-{sd}", "track", "member")
    s.stof("m-track", "track", "track", "member")
    s.stof("m-leaf", "leaf", "leaf", "member")
    s.st(box("j-carriage", "leaf", ST, 0.3, [0.075, 0.4, 0.075], [0.0875, 0, 0]), "leaf", "joint")
    s.st(box("support-west-jamb", "track", ST, 0.4, [0.075, 0.075, 0.1], [0, 0, 0]), "track", "support")
    s.st(box("support-east-jamb", "track", ST, 0.4, [0.075, 0.075, 0.1], [3.0, 0, 0]), "track", "support")
    s.dec("Body: the door at 75 mm detail as the drawing shows it: the portal (two 150 mm posts and the 150 mm track housing as three boxes, 100 mm deep) and the leaf as one 75 mm deep block over the opening width of 1 m (the 2D draws it as one outline); pane, frame, hangers, handle, seals and screws are merged into it.",
          "Körper: das Tor in 75-mm-Auflösung, wie es die Zeichnung zeigt: das Portal (zwei Pfosten 150 mm und das Schienengehäuse 150 mm als drei Quader, 100 mm tief) und der Flügel als ein 75 mm tiefer Block über die Öffnungsbreite von 1 m (die 2D zeichnet ihn als einen Umriss); Scheibe, Rahmen, Hänger, Griff, Dichtungen und Schrauben sind darin aufgegangen.")
    s.dec("Thermal: the portal parts are referenced from the body, the pane is a 75 mm block inside the leaf block (room face left out) and the leaf frame is the 75 mm head bar of the leaf. Structure: portal parts and leaf block are the members, one 75 mm block is the carriage, supports are 75 mm blocks. Minimal: the least that carries the loads.",
          "Thermisch: die Portalteile werden aus dem Körper referenziert, die Scheibe ist ein 75-mm-Block im Flügelblock (Raumseite ausgelassen) und der Flügelrahmen ist der 75-mm-Kopfstab des Flügels. Tragwerk: Portalteile und Flügelblock sind die Glieder, ein 75-mm-Block ist der Laufwagen, Auflager sind 75-mm-Blöcke. Minimal: das Wenigste, das die Lasten trägt.")
    return s


SC = {}
for n_, f_ in ((50, build50), (100, build100), (200, build200), (500, build500)):
    if n_ in only:
        SC[n_] = f_()
if __name__ == "__main__":
    save_asset("sliding-door", [SC[k] for k in sorted(SC)])
