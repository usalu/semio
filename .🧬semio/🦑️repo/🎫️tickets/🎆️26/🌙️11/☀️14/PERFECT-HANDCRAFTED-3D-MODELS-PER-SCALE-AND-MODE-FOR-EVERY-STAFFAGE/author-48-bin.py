from author_48_binlib import *

WT = 0.003
YS = [0.025, 0.2, 0.4, 0.6, 0.8, 0.955]


def walls(prefix, ys, wt=WT, **kw):
    out = []
    for k, (a, b) in enumerate(zip(ys, ys[1:])):
        ha, hb = hw(a), hw(b)
        out.append(tube("%s-%d" % (prefix, k + 1), BIN, a, b, ha, hb, ha - wt * ha / hb, 0.15 * ha, **kw))
    return out


def rib(sid, y0, h, proud=0.002, sink=0.0015, **kw):
    ha, hb = hw(y0), hw(y0 + h)
    oa, ob = ha + proud, hb + proud
    s = ob / oa
    inner_b = (hb - sink) / s
    c = BIN(region="bin-ribs", layer="hdpe-wall")
    return tube(sid, c, y0, y0 + h, oa, ob, inner_b, 0.15 * ha + proud, **kw)


# ------------------------------------------------------------ 1:50
feet = [box("foot-%s%s" % (a, b), BIN(role="detail", tone=0.15), (sx * 0.175 - 0.025, 0.0, sz * 0.175 - 0.025), (sx * 0.175 + 0.025, 0.008, sz * 0.175 + 0.025), cr=0.002) for a, sx in (("front", 1), ("rear", -1)) for b, sz in (("-left", 1), ("-right", -1))]
body50 = feet + [
    box("pedestal", BIN, (-0.2, 0.008, -0.2), (0.2, 0.029, 0.2), cr=0.0085),
]
body50 += walls("wall", YS)
body50 += [tag(rib("rib-0", 0.15, 0.018), texture=True), tag(rib("rib-1", 0.45, 0.018), texture=True), tag(rib("rib-2", 0.72, 0.018), texture=True), tag(rib("liner-ring", 0.895, 0.012), texture=True)]
body50 += [
    ext("rim", BIN, rr(0.25, 0.255, 0.04), "zx", 0.94, 0.98, holes=[rr(0.235, 0.235, 0.035)]),
    box("hinge-bracket-l", BIN, (-0.268, 0.925, 0.16), (-0.242, 0.99, 0.22), cr=0.004),
    box("hinge-bracket-r", BIN, (-0.268, 0.925, -0.22), (-0.242, 0.99, -0.16), cr=0.004),
    cyl("hinge-pin", BIN(role="detail", tone=0.3), "w", (-0.255, 0.984, 0), 0.44, 0.006),
    ext("base-plate", Ctx("base-plate", HD, None, "body", 0.3), rr(0.2025, 0.2025, 0.03, cx=0.2), "zx", 0.0, 0.008),
]
dish, endp = dish_profile(), [list(p) for p in crown_profile()]
body50 += [
    ext("lid-cap", LID, dish, "xy", -0.242, 0.242),
    ext("lid-end-l", LID, endp, "xy", -0.25, -0.241),
    ext("lid-end-r", LID, endp, "xy", 0.241, 0.25),
    ext("lid-skirt", LID(tone=0.45), rr(0.25, 0.26, 0.012, cx=0.26), "zx", -0.028, 0.002, holes=[rr(0.246, 0.256, 0.008, cx=0.26)]),
    ext("lid-handle", LID(role="detail", tone=0.35), handle_profile(), "xy", -0.05, 0.05),
    cyl("hinge-barrel", LID(role="detail", tone=0.25), "w", (0.0, 0.004, 0.0), 0.26, 0.014),
]

FEET = {"foot-front-left": (0.175, 0.175), "foot-front-right": (0.175, -0.175), "foot-rear-left": (-0.175, 0.175), "foot-rear-right": (-0.175, -0.175)}


def pads(h):
    return [tag(box("support-" + k, ST(member="body", part="support", tone=0.1), (x - 0.025, 0.0, z - 0.025), (x + 0.025, h, z + 0.025)), tags=[k]) for k, (x, z) in FEET.items()]


def skin(sid, y0, y1, n=6, e=8, region="bin-hdpe", layer="hdpe-wall"):
    ys = [y0 + (y1 - y0) * k / (n - 1) for k in range(n)]
    return lf(sid, BIN(region=region, layer=layer), "v", [(y, hw(y) + 0.0001, hw(y) + 0.0001, e) for y in ys], False, False)


def wallskin(prefix, ys, o=0.0001):
    out = []
    for k, (a, b) in enumerate(zip(ys, ys[1:])):
        ha, hb = hw(a), hw(b)
        out.append(tube("%s-%d" % (prefix, k + 1), BIN(layer="hdpe-wall"), a, b, ha + o, hb + o * hb / ha, ha - o, 0.15 * ha))
    return out


def film(sid, y0, y1, ha, hb, rr0, hz=None):
    hz = ha if hz is None else hz
    c = Ctx("body", "ldpe-film", "liner-film", "detail", 0.5)(layer="film")
    return ext(sid, c, rr(hz + 0.00002, ha + 0.00002, rr0), "zx", y0, y1, holes=[rr(hz - 0.00001, ha - 0.00001, rr0 - 0.00003)], taper=(1, hb / ha))


th50 = [th("th-" + f["id"], f["id"], "bin-hdpe", "hdpe-wall") for f in feet]
th50 += [th("th-pedestal", "pedestal", "bin-hdpe", "hdpe-wall")] + wallskin("th-wall", YS) + [th("th-rim", "rim", "bin-hdpe", "hdpe-wall"),
         th("th-hinge-bracket-l", "hinge-bracket-l", "bin-hdpe", "hdpe-wall"), th("th-hinge-bracket-r", "hinge-bracket-r", "bin-hdpe", "hdpe-wall")]
th50 += [th("th-" + r, r, "bin-ribs", "hdpe-wall") for r in ("rib-0", "rib-1", "rib-2", "liner-ring")]
th50 += [tag(th("th-lid-cap", "lid-cap", "lid-hdpe", "hdpe-wall"), sectionFaces=["-v", "+u", "-u"]), tag(th("th-lid-end-l", "lid-end-l", "lid-hdpe", "hdpe-wall"), sectionFaces=["+w"]), tag(th("th-lid-end-r", "lid-end-r", "lid-hdpe", "hdpe-wall"), sectionFaces=["-w"]),
         tag(ext("th-lid-skirt", LID(layer="hdpe-wall", tone=0.45), rr(0.2501, 0.2601, 0.0121, cx=0.26), "zx", -0.028, 0.002, holes=[rr(0.2499, 0.2599, 0.0119, cx=0.26)]), sectionFaces=["-v"]), th("th-lid-handle", "lid-handle", "lid-hdpe", "hdpe-wall"), th("th-hinge-barrel", "hinge-barrel", "lid-hdpe", "hdpe-wall"),
         film("th-film-rim", 0.94, 0.98, 0.255, 0.255, 0.04, hz=0.25), film("th-film-hem", 0.93, 0.94, 0.2475, 0.248, 0.0371)]
st50 = [stm("member-wall-%d" % k, "wall-%d" % k, "body", "member") for k in range(1, 6)] + [stm("member-pedestal", "pedestal", "body", "member"), stm("member-rim", "rim", "body", "member"),
        stm("member-base-plate", "base-plate", "base-plate", "member"), stm("member-lid", "lid-cap", "lid", "member"), stm("member-lid-skirt", "lid-skirt", "lid", "member"),
        stm("joint-hinge", "hinge-barrel", "lid", "joint"), stm("joint-bracket-l", "hinge-bracket-l", "body", "joint"), stm("joint-bracket-r", "hinge-bracket-r", "body", "joint"), stm("joint-pin", "hinge-pin", "body", "joint"),
        tag(ell("mass-body", ST(member="body", part="mass"), (0, 0.45, 0), (0.12, 0.3, 0.12))),
        tag(ell("mass-lid", ST(member="lid", part="mass", bone="lid"), (0.26, 0.019, 0), (0.2, 0.0095, 0.2)))] + pads(0.0075)
D50 = [
    text("Own 1:50 model of the lidded HDPE bin (closed-empty pose): four 50 x 50 x 8 mm feet under a 0.4 m pedestal slab (rounded edges, 21 mm: slab and 4 mm tub floor), the tub as five hollow rounded-square wall segments of 3 mm HDPE that follow the drawn convex outline from 0.4 to 0.496 m, four moulded rib rings raised 2 mm (the three 18 mm bands of the drawing and the 12 mm liner-retaining ring; flagged texture because the 2D draws them as lines), the 40 mm rim collar as a hollow ring, two hinge brackets and the hinge pin, the raised false floor (8 mm plate at 0.1 m) on the base-plate bone, and the lid as an open-bottom dish of 4 mm skin with the drawn crowned section (extruded across the 0.5 m depth), two 8 mm end walls (a 4 mm end wall would fall below the print feature), the 30 mm skirt ring, the handle arch and the hinge barrel on the lid bone.",
         "Eigenes 1:50-Modell des HDPE-Abfallbehälters mit Deckel (Pose geschlossen-leer): vier Füsse 50 x 50 x 8 mm unter einer 0,4-m-Sockelplatte (gerundete Kanten, 21 mm: Platte und 4 mm Behälterboden), der Behälter aus fünf hohlen Wandsegmenten (abgerundetes Quadrat, 3 mm HDPE), die der gezeichneten konvexen Kontur von 0,4 auf 0,496 m folgen, vier angeformte Rippenringe, 2 mm erhaben (die drei 18-mm-Bänder der Zeichnung und der 12-mm-Ring für den Beutel; als texture gekennzeichnet, weil das 2D sie als Linien zeichnet), der 40-mm-Randkragen als Hohlring, zwei Scharnierböcke und der Scharnierstift, der erhöhte Zwischenboden (8-mm-Platte bei 0,1 m) am Knochen base-plate, und der Deckel als unten offene Schale aus 4 mm Haut mit dem gezeichneten gewölbten Schnitt (über die 0,5 m Tiefe gezogen), zwei 8-mm-Stirnwände (eine 4-mm-Stirnwand läge unter dem Druckmerkmal), dem 30-mm-Schürzenring, dem Griffbogen und der Scharniertrommel am Deckelknochen."),
    text("Thermal: the wall is five 0.2 mm skin rings that overlap the wall by 0.1 mm (only the outside face counts; the wall's inner faces enclose the bag cavity and are not an exchange surface), pedestal, feet, rim, brackets and ribs by reference (the rib rings sink 1.5 mm into the wall so their inner faces are buried), the lid by reference with the dish's inner faces declared section faces (the open underside, the cavity walls, the inner faces of the end walls) and a 0.2 mm skin ring for the skirt, the bag film as two 30 micrometre rings: one hugging the rim, one hanging as the hem on the shell below it. areaM2 corrected from this model within 1.5: bin-hdpe 1.4 to 1.67, lid-hdpe 0.26 to 0.38 (crown instead of plan area), liner-film 0.08 to 0.111.",
         "Thermik: die Wand besteht aus fünf 0,2-mm-Hautringen, die 0,1 mm in die Wand reichen (nur die Aussenfläche zählt; die Innenflächen der Wand umschliessen den Beutelraum und sind keine Austauschfläche), Sockel, Füsse, Rand, Böcke und Rippen als Verweis (die Rippenringe sinken 1,5 mm in die Wand, ihre Innenflächen sind verdeckt), der Deckel als Verweis mit den Innenflächen der Schale als Schnittflächen (offene Unterseite, Hohlraumwände, Innenseiten der Stirnwände) und einem 0,2-mm-Hautring für die Schürze, die Beutelfolie als zwei 30-Mikrometer-Ringe: einer am Rand anliegend, einer als Saum auf der Schale darunter. areaM2 aus diesem Modell innerhalb 1,5 korrigiert: bin-hdpe 1,4 auf 1,67, lid-hdpe 0,26 auf 0,38 (Wölbung statt Grundfläche), liner-film 0,08 auf 0,111."),
    text("Structure: the five wall segments, pedestal and rim are the members of the body bone, the false floor and the lid dish with its skirt the members of their bones, brackets, pin and barrel the hinge joints, a 4.5 kg body mass ellipsoid and a 1.2 kg lid mass ellipsoid, and four 50 x 50 mm contact pads under the feet as the supports of the anchors foot-front/rear-left/right.",
         "Tragwerk: die fünf Wandsegmente, der Sockel und der Rand sind die Stäbe des Körperknochens, der Zwischenboden und die Deckelschale mit ihrer Schürze die Stäbe ihrer Knochen, Böcke, Stift und Trommel die Scharniergelenke, ein Massenellipsoid von 4,5 kg für den Körper und eines von 1,2 kg für den Deckel, und vier Kontaktplatten 50 x 50 mm unter den Füssen als Auflager der Anker foot-front/rear-left/right."),
]

# ------------------------------------------------------------ 1:100
def hws(y):
    return 0.2 + (y - 0.025) * 0.048 / 0.915


def rib2(sid, y0, h, hf, proud=0.002, sink=0.0015):
    ha, hb = hf(y0), hf(y0 + h)
    oa, ob = ha + proud, hb + proud
    s = ob / oa
    return tag(tube(sid, BIN(region="bin-ribs", layer="hdpe-wall"), y0, y0 + h, oa, ob, (hb - sink) / s, 0.15 * ha + proud), texture=True)


def lidparts(endw, bottom=0.0, handle=True, skirt=True):
    d = dish_profile(bottom=bottom)
    e = [list(p) for p in crown_profile()]
    out = [ext("lid-cap", LID, d, "xy", -0.25 + endw, 0.25 - endw),
           ext("lid-end-l", LID, e, "xy", -0.25, -0.25 + endw + 0.001), ext("lid-end-r", LID, e, "xy", 0.25 - endw - 0.001, 0.25)]
    if skirt:
        out.append(ext("lid-skirt", LID(tone=0.45), rr(0.25, 0.26, 0.012, cx=0.26), "zx", -0.028, 0.002, holes=[rr(0.246, 0.256, 0.008, cx=0.26)]))
    if handle:
        out.append(ext("lid-handle", LID(role="detail", tone=0.35), handle_profile(), "xy", -0.05, 0.05))
    return out


def lidth(lid_ids, handle=True):
    out = [tag(th("th-lid-cap", "lid-cap", "lid-hdpe", "hdpe-wall"), sectionFaces=["-v", "+u", "-u"]),
           tag(th("th-lid-end-l", "lid-end-l", "lid-hdpe", "hdpe-wall"), sectionFaces=["+w"]), tag(th("th-lid-end-r", "lid-end-r", "lid-hdpe", "hdpe-wall"), sectionFaces=["-w"])]
    if "lid-skirt" in lid_ids:
        out.append(ext("th-lid-skirt", LID(layer="hdpe-wall", tone=0.45), rr(0.2501, 0.2601, 0.0121, cx=0.26), "zx", -0.028, 0.002, holes=[rr(0.2499, 0.2599, 0.0119, cx=0.26)]))
    if handle:
        out.append(th("th-lid-handle", "lid-handle", "lid-hdpe", "hdpe-wall"))
    return out


body100 = [box("pedestal", BIN, (-0.2, 0.0, -0.2), (0.2, 0.025, 0.2), cr=0.0085),
           tube("wall", BIN, 0.025, 0.94, 0.2, 0.248, 0.2 - WT * 0.2 / 0.248, 0.03)]
body100 += [rib2("rib-0", 0.15, 0.018, hws), rib2("rib-1", 0.45, 0.018, hws), rib2("rib-2", 0.72, 0.018, hws)]
body100 += [ext("rim", BIN, rr(0.25, 0.255, 0.04), "zx", 0.94, 0.98, holes=[rr(0.235, 0.235, 0.035)]),
            box("hinge-bracket-l", BIN, (-0.268, 0.925, 0.16), (-0.242, 0.99, 0.22), cr=0.004), box("hinge-bracket-r", BIN, (-0.268, 0.925, -0.22), (-0.242, 0.99, -0.16), cr=0.004)]
body100 += lidparts(0.015)
th100 = [th("th-pedestal", "pedestal", "bin-hdpe", "hdpe-wall"),
         tube("th-wall", BIN(layer="hdpe-wall"), 0.025, 0.94, 0.2001, 0.248 * 1.0001, 0.1999, 0.03),
         th("th-rim", "rim", "bin-hdpe", "hdpe-wall"), th("th-hinge-bracket-l", "hinge-bracket-l", "bin-hdpe", "hdpe-wall"), th("th-hinge-bracket-r", "hinge-bracket-r", "bin-hdpe", "hdpe-wall")]
th100 += [th("th-" + r, r, "bin-ribs", "hdpe-wall") for r in ("rib-0", "rib-1", "rib-2")]
th100 += lidth([b["id"] for b in body100]) + [film("th-film-rim", 0.94, 0.98, 0.255, 0.255, 0.04, hz=0.25), film("th-film-hem", 0.93, 0.94, 0.2475, 0.248, 0.0371)]
st100 = [stm("member-wall", "wall", "body", "member"), stm("member-pedestal", "pedestal", "body", "member"), stm("member-rim", "rim", "body", "member"),
         tag(ext("member-base-plate", Ctx("base-plate", HD, None, "detail", 0.3)(member="base-plate", part="member"), rr(0.2025, 0.2025, 0.03, cx=0.2), "zx", 0.0, 0.015)),
         stm("member-lid", "lid-cap", "lid", "member"), stm("member-lid-skirt", "lid-skirt", "lid", "member"),
         stm("joint-bracket-l", "hinge-bracket-l", "body", "joint"), stm("joint-bracket-r", "hinge-bracket-r", "body", "joint"),
         tag(ell("mass-body", ST(member="body", part="mass"), (0, 0.45, 0), (0.12, 0.3, 0.12))),
         tag(ell("mass-lid", ST(member="lid", part="mass", bone="lid"), (0.26, 0.019, 0), (0.2, 0.0095, 0.2)))] + pads(0.015)

# ------------------------------------------------------------ 1:200
def hws2(y):
    return 0.2 + (y - 0.025) * 0.048 / 0.915


body200 = [box("slab", BIN, (-0.2, 0.0, -0.2), (0.2, 0.03, 0.2), cr=0.0085),
           tube("wall", BIN, 0.03, 0.94, hws2(0.03), 0.248, hws2(0.03) - WT * hws2(0.03) / 0.248, 0.03),
           ext("rim", BIN, rr(0.25, 0.255, 0.04), "zx", 0.94, 0.98, holes=[rr(0.235, 0.235, 0.035)])]
body200 += lidparts(0.015, bottom=-0.02, handle=False, skirt=False)
th200 = [th("th-slab", "slab", "bin-hdpe", "hdpe-wall"), tube("th-wall", BIN(layer="hdpe-wall"), 0.03, 0.94, hws2(0.03) + 0.0001, 0.2481, hws2(0.03) - 0.0001, 0.03), th("th-rim", "rim", "bin-hdpe", "hdpe-wall")]
th200 += [tube("th-" + r, BIN(region="bin-ribs", layer="hdpe-wall"), y, y + 0.018, hws2(y) + 0.002, hws2(y + 0.018) + 0.002, hws2(y) - 0.0015, 0.03) for r, y in (("rib-0", 0.15), ("rib-1", 0.45), ("rib-2", 0.72))]
th200 += [tag(th("th-lid-cap", "lid-cap", "lid-hdpe", "hdpe-wall"), sectionFaces=["-v", "+u", "-u"]), tag(th("th-lid-end-l", "lid-end-l", "lid-hdpe", "hdpe-wall"), sectionFaces=["+w"]), tag(th("th-lid-end-r", "lid-end-r", "lid-hdpe", "hdpe-wall"), sectionFaces=["-w"]),
          film("th-film-rim", 0.94, 0.98, 0.255, 0.255, 0.04, hz=0.25)]
st200 = [stm("member-wall", "wall", "body", "member"), stm("member-slab", "slab", "body", "member"), stm("member-rim", "rim", "body", "member"),
         tag(ext("member-base-plate", Ctx("base-plate", HD, None, "detail", 0.3)(member="base-plate", part="member"), rr(0.2025, 0.2025, 0.03, cx=0.2), "zx", 0.0, 0.03)),
         stm("member-lid", "lid-cap", "lid", "member"),
         tag(ell("mass-body", ST(member="body", part="mass"), (0, 0.45, 0), (0.12, 0.3, 0.12))),
         tag(ell("mass-lid", ST(member="lid", part="mass", bone="lid"), (0.26, 0.005, 0), (0.2, 0.03, 0.2)))] + pads(0.03)

# ------------------------------------------------------------ 1:500
body500 = [box("slab", BIN, (-0.2, 0.0, -0.2), (0.2, 0.075, 0.2), cr=0.012),
           tube("wall", BIN(regions=["bin-hdpe", "bin-ribs", "liner-film"]), 0.075, 0.98, 0.203, 0.255, 0.203 - WT * 0.203 / 0.255, 0.03),
           box("lid", LID, (0.0, 0.0, -0.25), (0.52, 0.04, 0.25))]
th500 = [th("th-slab", "slab", "bin-hdpe", "hdpe-wall"), tube("th-wall", BIN(layer="hdpe-wall"), 0.075, 0.98, 0.2031, 0.2551, 0.2029, 0.03), th("th-lid", "lid", "lid-hdpe", "hdpe-wall")]
th500 += [tube("th-" + r, BIN(region="bin-ribs", layer="hdpe-wall"), y, y + 0.018, 0.2 + (0.055 * (y - 0.075) / 0.865) + 0.002, 0.2 + (0.055 * (y + 0.018 - 0.075) / 0.865) + 0.002, 0.2 + (0.055 * (y - 0.075) / 0.865) - 0.0015, 0.03) for r, y in (("rib-0", 0.15), ("rib-1", 0.45), ("rib-2", 0.72))]
th500 += [film("th-film-rim", 0.94, 0.98, 0.255, 0.255, 0.04, hz=0.25)]
st500 = [stm("member-wall", "wall", "body", "member"), stm("member-slab", "slab", "body", "member"), stm("member-lid", "lid", "lid", "member"),
         tag(ext("member-base-plate", Ctx("base-plate", HD, None, "detail", 0.3)(member="base-plate", part="member"), rr(0.2025, 0.2025, 0.03, cx=0.2), "zx", 0.0, 0.075)),
         tag(ell("mass-body", ST(member="body", part="mass"), (0, 0.45, 0), (0.12, 0.3, 0.12))),
         tag(ell("mass-lid", ST(member="lid", part="mass", bone="lid"), (0.26, 0.0, 0), (0.2, 0.0375, 0.2)))]
st500 += [tag(box("support-" + k, ST(member="body", part="support", tone=0.1), (x - 0.0375, 0.0, z - 0.0375), (x + 0.0375, 0.075, z + 0.0375)), tags=[k]) for k, (x, z) in FEET.items()]
D100 = [
    text("Own 1:100 model: pedestal and floor are one 25 mm slab, the tub is one straight hollow wall of 3 mm (the drawn straight outline 0.4 to 0.496 m) with three texture rib rings, the rim ring, two hinge brackets, and the lid as the same open dish with 15 mm end walls (the print feature) and the 30 mm skirt ring and the handle; hinge pin, barrel and feet are merged away (not drawn).",
         "Eigenes 1:100-Modell: Sockel und Boden sind eine 25-mm-Platte, der Behälter eine gerade hohle Wand von 3 mm (die gezeichnete gerade Kontur 0,4 bis 0,496 m) mit drei Rippenringen (texture), der Randring, zwei Scharnierböcke, und der Deckel als dieselbe unten offene Schale mit 15-mm-Stirnwänden (das Druckmerkmal), dem 30-mm-Schürzenring und dem Griff; Scharnierstift, Trommel und Füsse sind zusammengefasst (nicht gezeichnet)."),
    text("Thermal: as at 1:50 with one wall skin ring; the false floor is a structure-only plate (the bone is not drawn at this scale).",
         "Thermik: wie bei 1:50 mit einem Wandhautring; der Zwischenboden ist eine reine Tragwerksplatte (der Knochen ist in diesem Maßstab nicht gezeichnet)."),
    text("Structure: wall, pedestal, rim, lid dish and skirt as members, bracket joints, the two masses, a 15 mm false-floor plate and four 15 mm contact pads.",
         "Tragwerk: Wand, Sockel, Rand, Deckelschale und Schürze als Stäbe, Bockgelenke, die beiden Massen, eine 15-mm-Zwischenbodenplatte und vier 15-mm-Kontaktplatten."),
]
D200 = [
    text("Own 1:200 model: a 30 mm slab (pedestal and floor merged), one hollow wall with the 3 mm skin, the 40 mm rim ring, and the lid dish whose skirt is part of the drawn lid outline (down to -20 mm) with 15 mm end walls; ribs (below the 30 mm feature), liner, handle, hinge and brackets are not drawn at this scale and exist only in the thermal list (rib rings, liner film).",
         "Eigenes 1:200-Modell: eine 30-mm-Platte (Sockel und Boden zusammengefasst), eine hohle Wand mit der 3-mm-Haut, der 40-mm-Randring und die Deckelschale, deren Schürze Teil der gezeichneten Deckelkontur ist (bis -20 mm), mit 15-mm-Stirnwänden; Rippen (unter dem Merkmal von 30 mm), Beutel, Griff, Scharnier und Böcke sind in diesem Maßstab nicht gezeichnet und stehen nur in der Thermikliste (Rippenringe, Beutelfolie)."),
    text("Thermal: wall skin ring, slab, rim and lid by reference or skin, three thermal-only rib rings and the film ring hugging the rim; only the total is compared here.",
         "Thermik: Wandhautring, Platte, Rand und Deckel als Verweis oder Haut, drei Rippenringe und der am Rand anliegende Folienring nur in der Thermik; in diesem Maßstab wird nur die Gesamtfläche verglichen."),
    text("Structure: wall, slab, rim, lid and a 30 mm false-floor plate as members, the two masses (the lid mass sits at the hinge level so that it does not bury the lid top) and four 30 mm pads.",
         "Tragwerk: Wand, Platte, Rand, Deckel und eine 30-mm-Zwischenbodenplatte als Stäbe, die beiden Massen (die Deckelmasse sitzt auf Scharnierhöhe, damit sie die Deckeloberseite nicht verdeckt) und vier 30-mm-Platten."),
]
D500 = [
    text("Own 1:500 model: a 75 mm slab (the print feature, thicker than the 29 mm pedestal and floor it merges), one hollow tapered wall from 0.075 to 0.98 m that includes the rim (0.203 to 0.255 m, the drawn outline), and the lid as a solid 0.52 x 0.04 x 0.5 m block (the drawn rectangle).",
         "Eigenes 1:500-Modell: eine 75-mm-Platte (das Druckmerkmal, dicker als die 29 mm Sockel und Boden, die sie zusammenfasst), eine hohle konische Wand von 0,075 bis 0,98 m mit dem Rand (0,203 bis 0,255 m, die gezeichnete Kontur) und der Deckel als massiver Block 0,52 x 0,04 x 0,5 m (das gezeichnete Rechteck)."),
    text("Thermal: wall skin ring, slab and lid by reference or skin, thermal-only rib rings and film ring (regions bin-ribs and liner-film have no body solid here); total compared.",
         "Thermik: Wandhautring, Platte und Deckel als Verweis oder Haut, Rippenringe und Folienring nur in der Thermik (die Regionen bin-ribs und liner-film haben hier keinen Körper); Gesamtfläche verglichen."),
    text("Structure: wall, slab, lid and a 75 mm false-floor plate as members, two masses and four 75 mm contact blocks.",
         "Tragwerk: Wand, Platte, Deckel und eine 75-mm-Zwischenbodenplatte als Stäbe, zwei Massen und vier 75-mm-Kontaktblöcke."),
]
if __name__ == "__main__":
    write("waste-bin", {50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
                        100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
                        200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
                        500: dict(body=body500, thermal=th500, structure=st500, decisions=D500)}, category="objects")
