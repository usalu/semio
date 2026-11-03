import sys, math
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from human import *

P = dict(solid_thigh=True, hl=.09, t0=.085, t1=.062, s0=.06, s1=.042, fw=.048, ww=.14, cw=.15, sw=.175, nw=.047, ac=.145, a0=.045, a1=.038, f0=.038, f1=.032, hr=.03, hw=.074, pd=.11, tl=.06, cd=.105, sd=.1)
KEY = {"thigh": "thigh-{s}", "shank": "shank-{s}", "shoe": "shoe-{s}", "toe": "shoe-toe-{s}", "shoe-toe": "shoe-toe-{s}", "hips": "pelvis-trousers", "pelvis": "pelvis-trousers", "torso": "shirt", "hem": "shirt",
       "sleeve": "sleeve-{s}", "forearm": "forearm-{s}", "hand": "hand-{s}", "neck": "neck", "skin": "head", "hair": "hair", "skull": "head"}
HALF = .3
SIDE_LAT = {"side-a": .2853, "side-b": -.2853, "back-a": .1763, "back-b": -.1763, "front": 0.0}


def build(a, sc, of, op):
    h = Human(a, coarse(P, sc), KEY); R = h.R; t = tier(sc)
    h.front(sc, of); h.plan(sc, op)
    s = lambda i: h.st.get(sc, i)
    for old in a["representations"][sc]["geometry"]["plan"]:
        i = old["id"]
        if i.startswith(("chair", "desk", "laptop")):
            old = dict(old)
            if i == "desk-top-plan": old.update(y=-HALF, h=2 * HALF)
            if i == "desk-edge-plan": old["points"] = [[0.292, -HALF + .012], [1.048, -HALF + .012], [1.048, HALF - .012], [0.292, HALF - .012], [0.292, -HALF + .012]]
            op.add(old)
    wb = lambda bone, i: wbox(R, h.st, sc, bone, i)
    # desk: top slab, two posts and an apron rail; the laptop shows the back of its lid
    dy0 = R.o("desk-top")[1]
    poly(of, "desk-top", "desk-top", [(-HALF, dy0), (-HALF, dy0 + .025), (HALF, dy0 + .025), (HALF, dy0)], 5.0, s("desk-top"))
    for sd, sg in SIDES:
        poly(of, f"desk-post-{sd}", "desk-leg", [(sg * (HALF - .045), 0.0), (sg * (HALF - .045), R.t("desk-leg")[1]), (sg * (HALF - .005), R.t("desk-leg")[1]), (sg * (HALF - .005), 0.0)], 4.9, s("desk-end-panel"), follow=True)
    if t >= 1:
        poly(of, "desk-apron", "desk-top", [(-HALF + .045, dy0 - .1), (-HALF + .045, dy0), (HALF - .045, dy0), (HALF - .045, dy0 - .1)], 4.95, s("desk-end-panel"))
    ly0, ly1 = R.o("screen")[1], R.t("screen")[1]
    poly(of, "laptop-lid", "screen", [(-.17, ly0), (-.17, ly1), (.17, ly1), (.17, ly0)], 5.6, s("laptop-lid"), follow=True)
    if t >= 1:
        by0 = R.o("laptop")[1]
        poly(of, "laptop-base", "laptop", [(-.17, by0), (-.17, by0 + .02), (.17, by0 + .02), (.17, by0)], 5.5, s("laptop-base"))
    # chair: seat, backrest, armrests, gas column and the star base with its castors
    c = wb("chair-seat", "chair-seat-cushion")
    poly(of, "chair-seat", "chair-seat", [(-.235, c[1]), (-.235, c[3]), (.235, c[3]), (.235, c[1])], .41, s("chair-seat-cushion"))
    k0, k1 = R.o("chair-backrest")[1], R.t("chair-backrest")[1]
    poly(of, "chair-backrest", "chair-backrest", [(-.245, k0 + .05), (-.245, k1), (.245, k1), (.245, k0 + .05)], .41, s("chair-backrest-shell"), follow=True)
    g0, g1 = R.o("chair-column")[1], R.t("chair-column")[1]
    poly(of, "chair-column", "chair-column", [(-.0125, g0), (-.0125, g1), (.0125, g1), (.0125, g0)], .32, s("chair-column-rod"), follow=True)
    star_y = R.o("chair-base")[1]
    poly(of, "chair-base", "chair-base", [(-.285, star_y), (-.04, star_y + .045), (.04, star_y + .045), (.285, star_y), (.04, star_y + .015), (-.04, star_y + .015)], .275, s("chair-base-star"))
    for key, lat in SIDE_LAT.items():
        bone = f"chair-castor-{key}"
        if key.startswith("back") and t == 0: continue
        if key == "front" or t >= 1:
            ell(of, bone, bone, lat, R.o(bone)[1], .025, .025, .14 if key != "back-a" and key != "back-b" else .13, s("chair-castor-front"))
    if t >= 1:
        for sd, sg in SIDES:
            pad = wb("chair-seat", "chair-armrest-pad")
            rect(of, f"chair-armrest-pad-{sd}", "chair-seat", sg * .268 - .022, pad[1] if pad[1] < pad[3] else pad[3], .044, abs(pad[3] - pad[1]), .5, s("chair-armrest-pad"), r=.01)
            poly(of, f"chair-armrest-post-{sd}", "chair-seat", [(sg * .268 - .01, c[3]), (sg * .268 - .01, pad[1]), (sg * .268 + .01, pad[1]), (sg * .268 + .01, c[3])], .5, s("chair-armrest-post") | {"tone": .3}) if False else None
    if t >= 3:
        line(of, "desk-edge-line", "desk-top", [(-HALF + .01, dy0 + .0155), (HALF - .01, dy0 + .0155)], 5.1, .3)
        line(of, "laptop-display", "screen", [(-.15, ly0 + .02), (-.15, ly1 - .02), (.15, ly1 - .02), (.15, ly0 + .02), (-.15, ly0 + .02)], 5.65, .2, follow=True)


NOTES = notes({"500": "The desk is a top slab on two posts and the chair a seat, backrest and star base. The plan shows the desk 0.6 m wide, the chair star and the person.", "200": "Desk with two posts and an apron rail, laptop lid, chair with seat, backrest, column and star base with castors. The plan shows the desk 0.6 m wide.", "100": "Desk top with posts and apron, laptop lid and base, chair with armrests, column and castors.", "50": "Desk edge line, laptop display frame and the chair details; in the plan the desk, laptop and chair keep their authored detail with the desk 0.6 m wide."},
              {"500": "Der Schreibtisch ist eine Platte auf zwei Pfosten, der Stuhl Sitz, Lehne und Fußkreuz. Der Grundriss zeigt den 0,6 m breiten Tisch, das Fußkreuz und die Person.", "200": "Schreibtisch mit zwei Pfosten und Zarge, Laptop-Deckel, Stuhl mit Sitz, Lehne, Säule und Fußkreuz mit Rollen. Der Grundriss zeigt den 0,6 m breiten Tisch.", "100": "Tischplatte mit Pfosten und Zarge, Laptop-Deckel und -Basis, Stuhl mit Armlehnen, Säule und Rollen.", "50": "Tischkantenlinie, Displayrahmen des Laptops und die Stuhldetails; im Grundriss behalten Tisch, Laptop und Stuhl ihre Detailzeichnung, der Tisch ist 0,6 m breit."})
if __name__ == "__main__":
    print(write("seated-office-worker", build, NOTES))
