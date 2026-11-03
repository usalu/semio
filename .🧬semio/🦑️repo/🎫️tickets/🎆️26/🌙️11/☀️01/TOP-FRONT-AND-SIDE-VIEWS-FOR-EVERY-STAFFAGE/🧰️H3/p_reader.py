import sys, math
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from human import *

P = dict(solid_thigh=True, hl=.085, t0=.08, t1=.06, s0=.058, s1=.04, fw=.045, ww=.13, cw=.14, sw=.165, nw=.045, ac=.14, a0=.042, a1=.036, f0=.036, f1=.03, hr=.03, hw=.072, pd=.105, tl=.06, cd=.1, sd=.095)
KEY = {"thigh": "thigh-{s}", "shank": "shank-{s}", "shoe": "shoe-{s}", "toe": "shoe-toe-{s}", "shoe-toe": "shoe-toe-{s}", "hips": "pelvis-jeans", "pelvis": "pelvis-jeans", "torso": "sweater", "hem": "sweater-hem",
       "sleeve": "sleeve-{s}", "forearm": "forearm-{s}", "hand": "hand-{s}", "neck": "neck", "skin": "head", "hair": "hair", "skull": "head"}
HALF = .25


def build(a, sc, of, op):
    h = Human(a, coarse(P, sc), KEY); R = h.R; t = tier(sc)
    h.front(sc, of); h.plan(sc, op)
    s = lambda i: h.st.get(sc, i)
    bb = lambda i: h.st.bbox(sc, i)
    seat, lf, back = bb("bench-seat"), bb("bench-leg-front"), bb("bench-back")
    poly(of, "bench-seat", "seat", [(-HALF, seat[1]), (-HALF, seat[3]), (HALF, seat[3]), (HALF, seat[1])], 1.5, s("bench-seat"))
    for sd, sg in SIDES:
        poly(of, f"bench-leg-{sd}", "seat", [(sg * (HALF - .06), lf[1]), (sg * (HALF - .06), seat[1]), (sg * (HALF - .01), seat[1]), (sg * (HALF - .01), lf[1])], 1.4, s("bench-leg-front"))
    poly(of, "bench-back", "seat", [(-HALF, back[1]), (-HALF, back[3]), (HALF, back[3]), (HALF, back[1])], .8, s("bench-back"))
    rect(op, "bench-seat", "seat", seat[0], -HALF, seat[2] - seat[0], 2 * HALF, .7, s("bench-seat"))
    rect(op, "bench-back", "seat", back[0], -HALF, back[2] - back[0], 2 * HALF, .75, s("bench-back"))
    if t >= 3:
        for i, ly in enumerate((-.12, 0.0, .12)):
            line(op, f"bench-slat-{i}", "seat", [(seat[0] + .02, ly), (seat[2] - .02, ly)], .72, .3)
        line(of, "bench-slat-1", "seat", [(-HALF, seat[3] - .015), (HALF, seat[3] - .015)], 1.6, .3)
        line(of, "bench-back-slat", "seat", [(-HALF, (back[1] + back[3]) / 2), (HALF, (back[1] + back[3]) / 2)], .9, .3)
    pb, gb = bb("pad-cover"), bb("page-sheet")
    pw = R.p("pad", pb[0], 0), R.p("pad", pb[2], 0)
    lat = P["ac"]
    pwid, gwid = (.11, .105) if t == 0 else (.075, .07)
    th = .05 if t == 0 else .012
    limbsol(h, of, "pad-cover", "pad", th, th, lat, 6.4, s("pad-cover"), zr=pwid)
    cap(op, "pad-cover", "pad", pw[0][0] + .02, lat, pw[1][0] - .02, lat, pwid, pwid, 6.4, s("pad-cover"), follow=True)
    gw = R.p("page", gb[0], 0), R.p("page", gb[2], 0)
    limbsol(h, of, "page-sheet", "page", th, th, lat, 6.5, s("page-sheet"), zr=gwid)
    cap(op, "page-sheet", "page", gw[0][0] + .02, lat, gw[1][0] - .02, lat, gwid - .002, gwid - .002, 6.5, s("page-sheet"), follow=True)
    cb = bb("cup-body")
    cx, cy0, cy1 = R.o("cup")[0], R.p("cup", cb[0], 0)[1], R.p("cup", cb[2], 0)[1]
    cl = -.17
    poly(of, "cup-body", "cup", [(cl - .036, cy0), (cl - .04, cy1), (cl + .04, cy1), (cl + .036, cy0)], 1.7, s("cup-body"))
    ell(op, "cup-body", "cup", cx, cl, .04, .04, 1.7, s("cup-body"))
    if t >= 1:
        poly(of, "cup-lid", "cup", [(cl - .042, cy1), (cl - .042, cy1 + .012), (cl + .042, cy1 + .012), (cl + .042, cy1)], 1.75, s("cup-lid"))
        ell(op, "cup-lid", "cup", cx, cl, .03, .03, 1.75, s("cup-lid"))
    if t >= 2:
        poly(of, "cup-sleeve", "cup", [(cl - .039, cy0 + .03), (cl - .04, cy1 - .03), (cl + .04, cy1 - .03), (cl + .039, cy0 + .03)], 1.76, s("cup-sleeve"))
    if t >= 3:
        line(of, "pad-ruling", "pad", [(lat - .05, pw[0][1] + .0), (lat + .05, pw[0][1] + .0)], 6.46, .2, follow=True)
        line(of, "hair-bun-seam", "head", [(0.0, R.t("head")[1] - .02), (0.0, R.t("head")[1] + .03)], 5.65, .3)
    if True:
        bb_ = wbox(R, h.st, sc, "head", "hair-bun")
        lb = h.st.bbox(sc, "hair-bun")
        if t == 0: ellsol(of, "hair-bun", "head", lb[0], lb[2], lb[1], lb[3], max(.035, (lb[3] - lb[1]) / 2), 5.6, s("hair-bun"))
        else: ell(of, "hair-bun", "head", 0.0, (bb_[1] + bb_[3]) / 2, max(.035, (bb_[3] - bb_[1]) / 2), (bb_[3] - bb_[1]) / 2, 5.6, s("hair-bun"))
        ell(op, "hair-bun", "head", (bb_[0] + bb_[2]) / 2, 0.0, (bb_[2] - bb_[0]) / 2, (bb_[2] - bb_[0]) / 2, 6.2, s("hair-bun"))


NOTES = notes({"500": "The bench is a seat slab with two legs and a backrest panel; the cup and the pad are small blocks.", "200": "The bench shows seat, end legs and backrest; the pad and the cup stand as blocks; the hair bun tops the head.", "100": "Bench with seat, legs and backrest panel, notepad with sheet, cup with lid and sleeve, hair bun.", "50": "Bench slats, notepad ruling, hair bun seam and the cup with lid and sleeve."},
              {"500": "Die Bank ist eine Sitzplatte mit zwei Beinen und einer Rückenlehne; Becher und Block sind kleine Quader.", "200": "Die Bank zeigt Sitz, Stirnbeine und Rückenlehne; Block und Becher stehen als Quader; der Haarknoten sitzt auf dem Kopf.", "100": "Bank mit Sitz, Beinen und Lehne, Notizblock mit Blatt, Becher mit Deckel und Manschette, Haarknoten.", "50": "Bankleisten, Linierung des Blocks, Naht des Haarknotens sowie der Becher mit Deckel und Manschette."})
if __name__ == "__main__":
    print(write("seated-reader", build, NOTES))
