import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from human import *

P = dict(hl=.055, t0=.05, t1=.038, s0=.036, s1=.026, fw=.032, ww=.095, cw=.105, sw=.115, nw=.028, ac=.105, a0=.033, a1=.028, f0=.027, f1=.022, hr=.02, hw=.052, pd=.08, tl=.04, cd=.075, sd=.06, fl5=(.05, .055, .05))
KEY = {"thigh": "thigh-{s}", "thigh-over": "shorts-{s}", "shank": "shank-{s}", "shoe": "shoe-{s}", "toe": "shoe-toe-{s}", "shoe-toe": "shoe-toe-{s}", "hips": "pelvis-shorts", "pelvis": "pelvis-shorts", "torso": "tshirt",
       "sleeve": "arm-skin-{s}", "forearm": "forearm-{s}", "hand": "hand-{s}", "neck": "neck", "skin": "head", "hair": "hair", "skull": "head"}


def build(a, sc, of, op):
    h = Human(a, coarse(P, sc), KEY); R = h.R; t = tier(sc)
    h.front(sc, of); h.plan(sc, op)
    over(h, sc, of, op, 0.12)
    s = lambda i: h.st.get(sc, i)
    y0, y1 = R.o("pack")[1], R.t("pack")[1]
    poly(of, "pack-body", "pack", [(-.11, y0 - .05), (-.11, y1), (.11, y1), (.11, y0 - .05)], 3.5, s("pack-body"), follow=True)
    px = wbox(R, h.st, sc, "pack", "pack-body")
    rect(op, "pack-body", "pack", px[0], -.1, px[2] - px[0], .2, 4.05, s("pack-body"))
    if t >= 2:
        for sd, sg in SIDES:
            line(of, f"pack-strap-{sd}", "torso", [(sg * .07, R.t("torso")[1] - .01), (sg * .06, R.o("torso")[1] + .06)], 4.4, .2)
            line(op, f"pack-strap-{sd}", "torso", [(R.t("torso")[0] - .02, sg * .07), (R.t("torso")[0] + .02, sg * .075)], 4.4, .2)
    if t >= 3:
        rect(of, "tshirt-print", "torso", -.04, R.o("torso")[1] + .15, .08, .06, 4.2, s("tshirt-print"))
        line(op, "pack-pocket", "pack", [(px[0] + .02, -.05), (px[0] + .02, .05)], 4.1, .1)


NOTES = notes({"500": "The backpack shows as a small block behind the body.", "200": "The backpack sits behind the torso and shows beside it; shorts cover the upper thighs.", "100": "Backpack with straps, shorts on the thighs.", "50": "Backpack with straps and pocket line, T-shirt print on the front."},
              {"500": "Der Rucksack erscheint als kleiner Block hinter dem Körper.", "200": "Der Rucksack sitzt hinter dem Rumpf und ragt seitlich heraus; Shorts bedecken die Oberschenkel.", "100": "Rucksack mit Trägern, Shorts auf den Oberschenkeln.", "50": "Rucksack mit Trägern und Taschenlinie, Aufdruck auf dem T-Shirt vorn."})
if __name__ == "__main__":
    print(write("running-child", build, NOTES))
