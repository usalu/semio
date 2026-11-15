from author_15_lib import *
MC, HB = "melamine-chipboard", "hardboard-back"
YS = [0.36, 0.66, 0.96, 1.26, 1.56]
def build(scale, t, bt, shelf_t, pins, plinth_y, shelf_depth=(-0.132, 0.145)):
    c = Ctx("bookshelf", scale); c.to("body")
    rc = {50: 0.002, 100: 0.003}.get(scale)
    c.bw("side-l", "frame", MC, "body", 0.7, (0, t), (0.06, 1.8), (-0.15, 0.15), corner=rc)
    c.bw("side-r", "frame", MC, "body", 0.7, (0.8 - t, 0.8), (0.06, 1.8), (-0.15, 0.15), corner=rc)
    c.bw("top", "frame", MC, "body", 0.7, (t, 0.8 - t), (1.8 - t, 1.8), (-0.15, 0.15), corner=rc)
    c.bw("bottom", "frame", MC, "body", 0.7, (t, 0.8 - t), (0.06, 0.06 + t), (-0.15, 0.15), corner=rc)
    c.bw("back", "frame", HB, "body", 0.9, (t, 0.8 - t), (0.07, 1.8 - t), (-0.15, -0.15 + bt))
    c.bw("plinth", "frame", MC, "body", 0.4, (0.014 if scale == 50 else 0.03 if scale >= 200 else 0.018, 0.786 if scale == 50 else 0.77 if scale >= 200 else 0.782), (0, plinth_y), (-0.13, 0.12))
    x0 = t - 0.004
    for n, y in enumerate(YS, 1):
        c.bw("shelf-%d-l" % n, "shelf-%d-l" % n, MC, "body", 0.7, (x0, 0.4 + 0.005 if False else x0 + 0.391 if scale < 200 else 0.4 + 0.015), (y, y + shelf_t), shelf_depth, corner=rc)
        c.bw("shelf-%d-r" % n, "shelf-%d-r" % n, MC, "body", 0.7, (0.8 - t + 0.004 - 0.391 if scale < 200 else 0.4 - 0.015, 0.8 - t + 0.004), (y, y + shelf_t), shelf_depth, corner=rc)
        if pins:
            for side, xa, d in (("l", t, 1), ("r", 0.8 - t, -1)):
                c.rev("pin-%d-%s" % (n, side), "shelf-%d" % n, MC, "body", 0.3, [[0, 0.0065], [0.02, 0.0065]], [d, 0, 0], [xa, y - 0.0055, 0.114], repeat={"count": 2, "translation": [0, 0, -0.22]})
    return c

def regions(c, scale, pins, t0=0.022, st=0.022, shelf_depth=(-0.132, 0.145)):
    c.to("thermal")
    for s in ("side-l", "side-r", "top", "bottom"): c.ref("th-" + s, s, region="carcass")
    c.ref("th-back", "back", region="back-panel"); c.ref("th-plinth", "plinth", region="plinth")
    for n in range(1, 6):
        for s in "lr":
            c.ref("th-shelf-%d-%s" % (n, s), "shelf-%d-%s" % (n, s), region="shelves")
            if pins: c.ref("th-pin-%d-%s" % (n, s), "pin-%d-%s" % (n, s), region="shelves")
    c.to("structure")
    for s in ("side-l", "side-r", "top", "bottom", "back"): c.ref("m-" + s, s, member="frame", part="member")
    c.ref("m-plinth", "plinth", member="ground", part="member")
    for n in range(1, 6):
        for s in "lr":
            c.ref("m-shelf-%d-%s" % (n, s), "shelf-%d-%s" % (n, s), member="shelf-%d-%s" % (n, s), part="member")
            if pins:
                c.ref("m-pin-%d-%s" % (n, s), "pin-%d-%s" % (n, s), member="shelf-%d" % n, part="member")
                c.ref("j-pin-%d-%s" % (n, s), "pin-%d-%s" % (n, s), member="shelf-%d-%s" % (n, s), part="joint")
        if not pins:
            y = YS[n - 1]
            c.bw("m-shelf-%d" % n, "frame", MC, "detail", 0.7, (t0, 0.8 - t0), (y, y + st), shelf_depth, member="shelf-%d" % n, part="member")

def finish(c, h, ph, pz):
    c.to("structure")
    c.bw("mass-frame", "frame", MC, "detail", 0.4, (0.4 - h, 0.4 + h), (0.9 - h, 0.9 + h), (-h, h), member="frame", part="mass")
    for sid, x in (("plinth-left", 0.06), ("plinth-right", 0.74)):
        c.bw("support-" + sid, "frame", MC, "ground", 0.2, (x - pz, x + pz), (0, ph), (-0.11, 0.1), member="frame", part="support", anchor=sid)

def s50():
    c = build(50, 0.018, 0.0075, 0.018, True, 0.075)
    regions(c, 50, True); finish(c, 0.05, 0.008, 0.03)
    c.dec("1:50 is the full construction: 18 mm melamine sides, top, bottom and ten shelf halves with 2 mm edge rounding, a recessed 75 mm plinth, a hardboard back (modelled at the 7.5 mm smallest feature instead of 3 mm) flush with the rear edge, and the twenty shelf pins as 13 mm round studs at the drawn positions; the shelf halves overlap 10 mm at mid-span (the hinge of the sag motion) as drawn.", "1:50 ist die volle Konstruktion: 18 mm Melaminseiten, Deckel, Boden und zehn Fachbodenhälften mit 2 mm Kantenrundung, ein zurückgesetzter 75 mm Sockel, eine bündig hinten liegende Hartfaserrückwand (im 7,5 mm Mindestmass statt 3 mm) und die zwanzig Bodenträger als runde 13 mm Zapfen an den gezeichneten Stellen; die Fachbodenhälften überlappen sich wie gezeichnet 10 mm in der Mitte (das Gelenk der Durchbiegung).")
    c.dec("Thermal: carcass, shelves, back panel and plinth reference the body boards (single layers). Structure: sides, top, bottom and back are the frame member, the plinth the ground member, the shelf halves their own members, the studs are the shelf members and the sag hinges, one compact mass at the carcass centre, two contact pads under the plinth.", "Thermik: Korpus, Fachböden, Rückwand und Sockel verweisen auf die Körperplatten (einschichtig). Tragwerk: Seiten, Deckel, Boden und Rückwand sind das Rahmenglied, der Sockel das Bodenglied, die Fachbodenhälften eigene Glieder, die Zapfen die Fachbodenglieder und die Durchbiegungsgelenke, eine kompakte Masse in der Korpusmitte, zwei Auflager unter dem Sockel.")
    return c.out()

def s100():
    c = build(100, 0.022, 0.015, 0.022, False, 0.075)
    regions(c, 100, False, 0.022, 0.022); finish(c, 0.05, 0.015, 0.03)
    c.dec("1:100 thickens the boards to the drawn 22 mm and the back to the 15 mm smallest feature, drops the shelf studs and the plinth recess detail, and keeps the five shelf pairs.", "1:100 verdickt die Platten auf die gezeichneten 22 mm und die Rückwand auf das 15 mm Mindestmass, lässt die Zapfen und das Sockeldetail weg und behält die fünf Fachbodenpaare.")
    c.dec("Thermal references the body boards; structure adds one plain full-width board per shelf pair as the shelf member (no rounded edges, lighter than the finer scale), the studs are merged into the carcass joints.", "Thermik verweist auf die Körperplatten; das Tragwerk ergänzt je Fachbodenpaar eine einfache durchgehende Platte als Fachbodenglied (ohne Kantenrundung, leichter als der feinere Massstab), die Zapfen sind in die Korpusgelenke eingegangen.")
    return c.out()

def s200():
    c = build(200, 0.03, 0.03, 0.03, False, 0.08)
    regions(c, 200, False, 0.03, 0.03); finish(c, 0.05, 0.03, 0.04)
    c.dec("1:200 builds the carcass from 30 mm boards (sides, top, bottom, back and ten shelf halves; the 30 mm smallest feature is above the real 18 mm) so the front reads as five open bays instead of the drawn filled block; the plinth is 80 mm high.", "1:200 baut den Korpus aus 30 mm Platten (Seiten, Deckel, Boden, Rückwand und zehn Fachbodenhälften; das 30 mm Mindestmass liegt über den echten 18 mm), sodass die Front als fünf offene Fächer statt als gezeichneter gefüllter Block liest; der Sockel ist 80 mm hoch.")
    return c.out()

def s500():
    c = build(500, 0.075, 0.075, 0.075, False, 0.08)
    regions(c, 500, False, 0.075, 0.075); finish(c, 0.075, 0.075, 0.075)
    c.dec("Minimal: 1:500 uses 75 mm boards throughout (the smallest feature), five shelf pairs between 75 mm sides, top, bottom and back; the carcass is therefore heavier than the real 18 mm construction.", "Minimal: 1:500 verwendet durchgehend 75 mm Platten (das Mindestmass), fünf Fachbodenpaare zwischen 75 mm Seiten, Deckel, Boden und Rückwand; der Korpus ist daher schwerer als die echte 18 mm Konstruktion.")
    return c.out()

if __name__ == "__main__":
    save("bookshelf", {50: s50(), 100: s100(), 200: s200(), 500: s500()})
