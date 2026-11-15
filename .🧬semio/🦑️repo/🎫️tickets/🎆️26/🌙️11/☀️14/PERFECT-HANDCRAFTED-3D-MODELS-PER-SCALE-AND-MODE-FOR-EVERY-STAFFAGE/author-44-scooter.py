import os, sys
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-44-lib.py")).read())
SLUG = "electric-kick-scooter"
AN, RU, GT, PL, SD, MA = "anodised", "rubber", "griptape", "plastic", "steel-disc", "motor-alloy"
DK, FK, ST, WF, WR = "deck", "fork", "stem", "wheel-f", "wheel-r"
EL = lambda sc, i: shape(SLUG, sc, "elevation", i)


def corner(cx, cz, r, a0, n=3):
    return [(cz + r * math.sin(a0 + (math.pi / 2) * k / n), cx + r * math.cos(a0 + (math.pi / 2) * k / n)) for k in range(n + 1)]


def deck_outline(notch_x=-0.31, hw=0.085, x0=-0.41, x1=0.38, r=0.02, nw=0.03):
    pts = []
    pts += [(-hw + r, x1), (hw - r, x1)]
    pts += [(hw - r + r * math.sin(t), x1 - r + r * math.cos(t)) for t in (0.5236, 1.0472)] + [(hw, x1 - r)]
    pts += [(hw, x0 + r)]
    pts += [(hw - r + r * math.cos(t), x0 + r - r * math.sin(t)) for t in (0.5236, 1.0472)] + [(hw - r, x0)]
    pts += [(nw, x0), (nw, notch_x), (-nw, notch_x), (-nw, x0)]
    pts += [(-hw + r, x0)]
    pts += [(-hw + r - r * math.sin(t), x0 + r - r * math.cos(t)) for t in (0.5236, 1.0472)] + [(-hw, x0 + r)]
    pts += [(-hw, x1 - r)]
    pts += [(-hw + r - r * math.cos(t), x1 - r + r * math.sin(t)) for t in (0.5236, 1.0472)]
    return dedup([[r6(z), r6(x)] for z, x in pts])


def tyre_p(rin, rout, hw, ch=0.006):
    return cap_prof([[-hw + ch, rin], [-hw, rin + 0.008], [-hw, rout - 0.012], [-hw + 0.006, rout - 0.003], [-hw / 2, rout], [hw / 2, rout], [hw - 0.006, rout - 0.003], [hw, rout - 0.012], [hw, rin + 0.008], [hw - ch, rin]])


def wheels(sc):
    b = []
    for bone, tag in ((WF, "f"), (WR, "r")):
        if sc == 200:
            b.append(revolve(f"wheel-{tag}", bone, RU, 0.07, "z", [0, 0, 0], cap_prof([[-0.022, 0], [-0.022, 0.098], [-0.014, 0.108], [0.014, 0.108], [0.022, 0.098], [0.022, 0]])))
            continue
        fine = sc == 50
        b.append(revolve(f"tyre-{tag}", bone, RU, 0.07, "z", [0, 0, 0], tyre_p(0.073 if fine else 0.064, 0.108, 0.022, 0.006 if fine else 0.01)))
        rr = 0.075 if fine else 0.066
        if tag == "f":
            b.append(revolve("rim-f", bone, MA, 0.5, "z", [0, 0, 0], cap_prof([[-0.022, 0], [-0.022, rr], [0.022 if fine else 0.045, rr], [0.022 if fine else 0.045, 0]])))
            if fine:
                b.append(revolve("hub-f", bone, MA, 0.75, "z", [0, 0, 0.0225], cap_prof([[0, 0], [0, 0.035], [0.022, 0.035], [0.022, 0]]), role="detail"))
        else:
            b.append(revolve("rim-r", bone, AN, 0.55, "z", [0, 0, 0], cap_prof([[-0.022, 0], [-0.022, rr], [0.022, rr], [0.022, 0]])))
            if fine:
                b.append(revolve("disc-r", bone, SD, 0.6, "z", [0, 0, 0.0225], cap_prof([[0, 0], [0, 0.03], [0.006, 0.03], [0.006, 0]]), role="detail"))
                b.append(revolve("hub-r", bone, AN, 0.7, "z", [0, 0, 0.0285], cap_prof([[0, 0], [0, 0.03], [0.014, 0.03], [0.014, 0]]), role="detail"))
    return b


def deck(sc):
    b = []
    t = {50: 0.008, 100: 0.015, 200: 0.0}[sc]
    if sc == 200:
        out = deck_outline(r=0.02, nw=0.025)
        b.append(ext("deck-body", DK, AN, 0.6, "zx", out, 0.095, 0.145))
        return b
    out = deck_outline()
    b.append(ext("deck-top", DK, AN, 0.6, "zx", out, 0.145 - t, 0.145))
    b.append(ext("deck-bottom", DK, AN, 0.6, "zx", out, 0.095, 0.095 + t))
    ring = dedup(EL(sc, "deck"))
    ring = [[-0.4, 0.095], [0.37, 0.095], [0.38, 0.12], [0.37, 0.145], [-0.4, 0.145], [-0.41, 0.12]]
    wt = 0.008 if sc == 50 else 0.015
    for sd, z0, z1 in (("l", 0.085 - wt, 0.085), ("r", -0.085, -0.085 + wt)):
        b.append(ext(f"deck-side-{sd}", DK, AN, 0.6, "xy", ring, z0, z1))
    b.append(Bb("deck-end-front", DK, AN, 0.6, [wt, 0.05 - 2 * t, 0.17 - 2 * wt], [0.38 - wt / 2, 0.12, 0]))
    for sd, sg in (("l", 1), ("r", -1)):
        b.append(Bb(f"deck-notch-wall-{sd}", DK, AN, 0.6, [0.1, 0.05 - 2 * t, wt], [-0.36, 0.12, sg * (0.03 + wt / 2)]))
        b.append(Bb(f"deck-end-rear-{sd}", DK, AN, 0.6, [wt, 0.05 - 2 * t, 0.085 - wt - 0.03], [-0.41 + wt / 2, 0.12, sg * ((0.085 - wt + 0.03) / 2)]))
    return b


def Bb(id, bone, mat, tone, size, at, r=None, role="body"):
    return box(id, bone, mat, tone, size, at, r, role)


def deck_top_items(sc):
    b = []
    if sc == 50:
        b.append(Bb("grip-tape", DK, GT, 0.15, [0.66, 0.008, 0.14], [-0.03, 0.149, 0], 0.002))
    if sc < 200:
        b.append(ext("fender-r", DK, PL, 0.3, "xy", dedup(EL(sc, "fender-r")), -0.03, 0.03))
    return b


def steering(sc):
    b = []
    fine = sc == 50
    tube_r = {50: 0.019, 100: 0.0175, 200: 0.02}[sc]
    wall = {50: 0.002, 100: 0.003, 200: 0.004}[sc]
    tp = tube_r - (0.003 if sc == 50 else 0.0)
    stem = revolve("stem", ST, AN, 0.65, "x", [0, 0, 0], cap_prof([[0, tube_r - wall], [0, tube_r], [0.8, tp], [0.8, tp - wall]]) if sc < 200 else cap_prof([[0, 0], [0, tube_r], [0.8, tp], [0.8, 0]]))
    if sc == 200:
        stem["lateralScale"] = 1.25
    b.append(stem)
    br = 0.014 if sc < 200 else 0.03
    bw = br - wall
    if sc < 200:
        b.append(revolve("bar", ST, AN, 0.65, "z", [0.8, 0, -0.19], cap_prof([[0, bw], [0, br], [0.38, br], [0.38, bw]])))
        for sd, z0 in (("l", 0.12), ("r", -0.215)):
            b.append(revolve(f"grip-{sd}", ST, RU, 0.1, "z", [0.8, 0, z0], cap_prof([[0, 0], [0, 0.028], [0.095, 0.028], [0.095, 0]])))
    else:
        b.append(revolve("bars", ST, AN, 0.65, "z", [0.8, 0, -0.215], cap_prof([[0, 0], [0, 0.031], [0.43, 0.031], [0.43, 0]])))
    if sc < 200:
        hb = (0.088, 0.08, 0.07) if fine else (0.08, 0.07, 0.07)
        hpos = [-0.01, 0.0014, 0] if fine else [-0.01, 0.0014, 0]
        b.append(Bb("head-block", FK, AN, 0.6, list(hb), hpos, 0.01))
    zc = 0.0535 if sc < 200 else 0.03
    bl = capsule("fork-blade", FK, AN, 0.6, [0, 0, zc], [-0.1943, -0.003, zc], 0.014 if sc < 200 else 0.024, role="body")
    bl["lateralScale"] = r6((0.0085 if sc < 200 else 0.03) / (0.014 if sc < 200 else 0.024))
    b.append(bl)
    if fine:
        b.append(Bb("fork-crown", FK, AN, 0.6, [0.031, 0.03, 0.084], BP(0.4005, 0.27, 0.027, FK), 0.004, "detail"))
        lv = capsule("brake-lever", ST, PL, 0.2, [0.8036, -0.0106, 0.17], [0.7762, -0.0976, 0.17], 0.008, role="detail")
        lv["lateralScale"] = 1.25
        b.append(lv)
    return b


FORK = dump_of(SLUG)["frames"]["fork"]


def BP(x, y, z, bone):
    o, u, v = FORK["o"], FORK["u"], FORK["v"]
    dx, dy = x - o[0], y - o[1]
    return [r6(dx * u[0] + dy * u[1]), r6(dx * v[0] + dy * v[1]), z]


def body(sc):
    return deck(sc) + deck_top_items(sc) + wheels(sc) + steering(sc)


def by(lst):
    return {d["id"]: d for d in lst}


def thermal(s, sc):
    B = by(s.body)
    th = s.thermal
    if sc == 200:
        th.append(ref("th-deck-body", "deck-body", "deck-battery"))
        d = Bb("th-grip", DK, GT, 0.15, [0.66, 0.03, 0.14], [-0.03, 0.16, 0], 0.003)
        d["region"] = "deck-grip"
        th.append(d)
    else:
        fc = {"deck-top": ["-v"], "deck-bottom": ["+v"], "deck-side-l": ["-w"], "deck-side-r": ["+w"], "deck-end-front": ["-u"], "deck-notch-wall-l": ["+w"], "deck-notch-wall-r": ["-w"], "deck-end-rear-l": ["+u"], "deck-end-rear-r": ["+u"]}
        for i, f in fc.items():
            th.append(ref(f"th-{i}", i, "deck-battery", None, f))
        if sc == 50:
            th.append(ref("th-grip-tape", "grip-tape", "deck-grip"))
        else:
            d = Bb("th-grip", DK, GT, 0.15, [0.66, 0.015, 0.14], [-0.03, 0.1525, 0], 0.003)
            d["region"] = "deck-grip"
            th.append(d)
    d = Bb("th-bms", DK, PL, 0.2, [0.09, 0.02 if sc < 200 else 0.03, 0.1], [0.3, 0.12, 0], 0.004 if sc < 200 else None, "detail")
    d["region"] = "bms-controller"
    th.append(d)
    if sc < 200:
        th.append(ref("th-fender-r", "fender-r", "fenders"))
    else:
        d = ext("th-fender-r", DK, PL, 0.3, "xy", dedup(shape(SLUG, 100, "elevation", "fender-r")), -0.03, 0.03)
        d["region"] = "fenders"
        th.append(d)
    stem_ids = ["stem", "fork-blade"] + (["head-block"] if sc < 200 else []) + (["bar", "grip-l", "grip-r"] if sc < 200 else ["bars"]) + (["fork-crown", "brake-lever"] if sc == 50 else [])
    for i in stem_ids:
        th.append(ref(f"th-{i}", i, "stem-bars"))
    if sc == 200:
        for tg in ("f", "r"):
            th.append(ref(f"th-wheel-{tg}", f"wheel-{tg}", "tyres"))
        d = revolve("th-motor", WF, MA, 0.5, "z", [0, 0, -0.01], cap_prof([[0, 0], [0, 0.066], [0.03, 0.066], [0.03, 0]]), role="detail")
        d["region"] = "hub-motor"
        th.append(d)
        d = revolve("th-brake-disc", WR, SD, 0.6, "z", [0, 0, 0.0225], cap_prof([[0, 0], [0, 0.03], [0.03, 0.03], [0.03, 0]]), role="detail")
        d["region"] = "brake-disc"
        th.append(d)
    else:
        for tg in ("f", "r"):
            th.append(ref(f"th-tyre-{tg}", f"tyre-{tg}", "tyres"))
        th.append(ref("th-rim-f", "rim-f", "hub-motor"))
        if sc == 50:
            th.append(ref("th-hub-f", "hub-f", "hub-motor"))
            th.append(ref("th-disc-r", "disc-r", "brake-disc"))
            th.append(ref("th-hub-r", "hub-r", "brake-disc"))
        else:
            d = revolve("th-brake-disc", WR, SD, 0.6, "z", [0, 0, 0.0225], cap_prof([[0, 0], [0, 0.03], [0.03, 0.03], [0.03, 0]]), role="detail")
            d["region"] = "brake-disc"
            th.append(d)


def structure(s, sc):
    B = by(s.body)
    t = {50: 0.003, 100: 0.004, 200: 0.0}[sc]
    if sc == 200:
        s.st(Bb("deck-girder", DK, AN, 0.5, [0.76, 0.04, 0.15], [-0.01, 0.12, 0]), DK, "member")
    else:
        out = [[0.099, -0.08], [0.141, -0.08], [0.141, 0.08], [0.099, 0.08]]
        hole = [[0.099 + t, -0.08 + t], [0.141 - t, -0.08 + t], [0.141 - t, 0.08 - t], [0.099 + t, 0.08 - t]]
        s.st(ext("deck-tube", DK, AN, 0.5, "yz", out, -0.39, 0.36, holes=[hole]), DK, "member")
    s.stof("stem-member", "stem", ST, "member")
    s.stof("fork-member", "fork-blade", FK, "member")
    if sc == 200:
        for tg, wb in (("f", WF), ("r", WR)):
            s.st(revolve(f"axle-{tg}", wb, SD, 0.4, "z", [0, 0, -0.03], cap_prof([[0, 0], [0, 0.02], [0.08, 0.02], [0.08, 0]]), role="detail"), wb, "member")
    else:
        s.stof("wheel-f-member", "rim-f", WF, "member")
        s.stof("wheel-r-member", "rim-r", WR, "member")
    if sc <= 100:
        s.st(revolve("stem-hinge", ST, SD, 0.3, "z", [0, 0, -0.03], cap_prof([[0, 0], [0, 0.016], [0.06, 0.016], [0.06, 0]]), role="detail"), ST, "joint")
        if sc == 50:
            s.st(revolve("head-bearing", FK, SD, 0.3, "x", [-0.02, 0, 0], cap_prof([[0, 0], [0, 0.018], [0.03, 0.018], [0.03, 0]]), role="detail"), FK, "joint")
        s.st(revolve("axle-f", WF, SD, 0.3, "z", [0, 0, -0.03], cap_prof([[0, 0], [0, 0.012], [0.07, 0.012], [0.07, 0]]), role="detail"), WF, "joint")
        s.st(revolve("axle-r", WR, SD, 0.3, "z", [0, 0, -0.03], cap_prof([[0, 0], [0, 0.012], [0.07, 0.012], [0.07, 0]]), role="detail"), WR, "joint")
    s.st(ellipsoid("mass-battery", DK, SD, 0.2, [0.25, 0.012 if sc < 200 else 0.02, 0.06], [-0.02, 0.12, 0], role="detail"), DK, "mass")
    s.st(ellipsoid("mass-stem", ST, SD, 0.2, [0.1, 0.014 if sc < 200 else 0.016, 0.014 if sc < 200 else 0.016], [0.4, 0, 0], role="detail"), ST, "mass")
    s.st(ellipsoid("mass-motor", WF, SD, 0.2, [0.05, 0.05, 0.02], [0, 0, 0], role="detail"), WF, "mass")


def decisions(s, sc):
    d = {50: ("The kick scooter is built as the real vehicle: a hollow aluminium deck (8 mm top and bottom plates cut from the drawn plan with a notch for the rear wheel, two 8 mm side rails from the drawn side outline, end walls and notch walls), a grip-tape plate on top, the rear mudguard as a 13.5 mm arch band extruded from the drawn arch, a hollow 38 mm stem tube tapering to 30 mm, a 28 mm handlebar tube with two rubber grips, a brake lever, a head block and a single-sided fork blade with crown, and two wheels: a pneumatic tyre ring with a hub-motor housing (front, with the 22 mm cover) and a rim disc with brake disc and hub boss (rear). The battery pack and the controller sit inside the closed deck as thermal and structure solids.", "Der Tretroller ist als wirkliches Fahrzeug gebaut: ein hohles Aluminiumdeck (8 mm Deck- und Bodenplatte aus dem gezeichneten Grundriss mit einer Aussparung für das Hinterrad, zwei 8-mm-Seitenschienen aus dem gezeichneten Seitenumriss, Stirn- und Aussparungswände), eine Griptape-Platte oben, das hintere Schutzblech als 13,5 mm dickes Bogenband aus dem gezeichneten Bogen, ein hohles 38-mm-Lenkrohr, das sich auf 30 mm verjüngt, ein 28-mm-Lenkerrohr mit zwei Gummigriffen, ein Bremshebel, ein Steuerkopf und eine einseitige Gabel mit Krone, und zwei Räder: ein Luftreifenring mit Nabenmotorgehäuse (vorn, mit der 22-mm-Kappe) und eine Felgenscheibe mit Bremsscheibe und Nabenansatz (hinten). Akkupack und Steuerung sitzen im geschlossenen Deck als Thermik- und Strukturkörper."),
         100: ("At 1:100 the deck plates, rails and walls are 15 mm; the grip tape is 15 mm; the stem (38 mm), bar (28 mm) and grips are plain tubes; the head block is the 70 mm block of the drawing; wheels are a tyre ring and a 15 mm-feature rim disc; hub cover, brake lever, crown and the rear hub and disc are merged away (the disc stays in thermal).", "Bei 1:100 sind Deck- und Bodenplatte, Schienen und Wände 15 mm dick; das Griptape ist 15 mm; Lenkrohr (38 mm), Lenker (28 mm) und Griffe sind glatte Rohre; der Steuerkopf ist der 70-mm-Block der Zeichnung; die Räder bestehen aus Reifenring und Felgenscheibe; Nabenkappe, Bremshebel, Krone sowie Hinternabe und -scheibe sind zusammengefasst (die Scheibe bleibt im Thermikmodell)."),
         200: ("At 1:200 the deck is one closed 50 mm block with the wheel notch, the stem a 50 mm tube, the bars one 62 mm tube, the fork blade 48 x 24 mm, each wheel one solid 44 mm tyre disc; the grip tape, battery, controller, motor and brake disc are not drawn and live in thermal and structure as real solids.", "Bei 1:200 ist das Deck ein geschlossener 50-mm-Block mit der Radaussparung, das Lenkrohr ein 50-mm-Rohr, der Lenker ein 62-mm-Rohr, das Gabelblatt 48 x 24 mm, jedes Rad eine massive 44-mm-Reifenscheibe; Griptape, Akku, Steuerung, Motor und Bremsscheibe werden nicht gezeichnet und stehen als echte Körper in Thermik und Struktur.")}[sc]
    s.dec(*d)
    s.dec("Thermal: deck plates, rails and walls are the single-layer 'wall' region by reference; the controller is a hidden real box inside the deck; motor housing, hub cover, tyres, brake disc and hub, fender and every tube are references; at 1:200 motor and disc are real hidden solids.", "Thermik: Deck-, Bodenplatte, Schienen und Wände sind die einschichtige Region 'wall' per Verweis; die Steuerung ist ein verdeckter echter Kasten im Deck; Motorgehäuse, Kappe, Reifen, Bremsscheibe und Nabe, Schutzblech und jedes Rohr sind Verweise; bei 1:200 sind Motor und Scheibe echte verdeckte Körper.")
    s.dec("Structure: the deck is a hollow aluminium box girder member, stem and fork blade are members, each wheel's rim is its member; the stem hinge, head bearing and the two axles are joints (1:50 and 1:100); compact masses at the battery, the stem middle and the front motor.", "Struktur: das Deck ist ein hohler Aluminium-Kastenträger als Stab, Lenkrohr und Gabelblatt sind Stäbe, die Felge jedes Rades ist sein Stab; Lenkrohrscharnier, Steuerkopflager und die beiden Achsen sind Gelenke (1:50 und 1:100); kompakte Massen im Akku, in der Lenkrohrmitte und im vorderen Motor.")


def chk(s):
    for lst in (s.body, s.thermal, s.structure):
        ids = [d["id"] for d in lst]
        assert len(ids) == len(set(ids)), [i for i in ids if ids.count(i) > 1]


if __name__ == "__main__":
    only = [int(a) for a in sys.argv[1:]] or [50, 100, 200]
    sc = []
    for k in (50, 100, 200):
        if k in only:
            s = Scale(k)
            s.body = body(k)
            thermal(s, k)
            structure(s, k)
            decisions(s, k)
            chk(s)
            sc.append(s)
    save_v(SLUG, sc)
