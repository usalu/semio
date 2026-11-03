import sys, math
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from human import *

P = dict(hl=.045, t0=.065, t1=.05, s0=.05, s1=.04, fw=.042, drop=.09, ww=.12, cw=.13, sw=.135, nw=.05, ac=.11, a0=.04, a1=.036, f0=.036, f1=.032, hr=.03, hw=.075, pd=.1, tl=.05, cd=.1, sd=.09)
KEY = {"thigh": "thigh-{s}", "shank": "shank-{s}", "shoe": "shoe-{s}", "toe": "shoe-{s}", "hips": "thigh-l", "pelvis": "hoodie", "torso": "hoodie", "sleeve": "upper-arm-{s}", "forearm": "forearm-{s}",
       "hand": {"tone": .85, "region": "hands"}, "neck": "head", "skin": "head", "hair": "beanie", "skull": "head", "hem-leg": "x"}
BONES = {"neck": "head", "arm-l": "upper-arm-l", "arm-r": "upper-arm-r"}
HALF = .1


def build(a, sc, of, op):
    h = Human(a, coarse(P, sc), KEY, bones=BONES); R = h.R; t = tier(sc)
    sk = ("neck", "toe")
    h.front(sc, of, skip=sk); h.plan(sc, op, skip=sk)
    s = lambda i: h.st.get(sc, i)
    deck = s("deck"); wheel = s("wheel-f")
    oy = R.o("board")[1]
    prof = [(-.405, .103), (-.37, .03), (-.34, .018), (.34, .018), (.37, .03), (.405, .103), (.405, .085), (.35, -.008), (-.35, -.008), (-.405, .085)]
    for out in (of, op):
        poly(out, "deck", "board", [(x, y - .085 + .085) for x, y in prof], .5, deck); sol(out, -HALF, HALF)
    poly(of, "truck-front", "board", [(-.09, oy - .045), (-.09, oy - .008), (.09, oy - .008), (.09, oy - .045)], .4, wheel)
    for sd, sg in SIDES:
        rect(of, f"wheel-front-{sd}", "wheel-f", sg * .09 - .02, R.o("wheel-f")[1] - .027, .04, .054, .3, wheel, r=.012)
    for bone in ("wheel-f", "wheel-r"):
        for sd, sg in SIDES:
            rect(op, f"{bone}-{sd}", bone, R.o(bone)[0] - .027, sg * .105 - .02, .054, .04, .3, wheel, r=.01)
    if t >= 3:
        for bone in ("wheel-f", "wheel-r"):
            line(op, f"truck-{bone}", "board", [(R.o(bone)[0], -.09), (R.o(bone)[0], .09)], .55, .3)
        line(of, "deck-grip", "board", [(-HALF + .01, oy + .014), (HALF - .01, oy + .014)], .51, .3)
        for sd, sg in SIDES:
            line(of, f"cuff-{sd}", f"shank-{sd}", [(sg * P["hl"] - .045, R.t(f"shank-{sd}")[1] - .02), (sg * P["hl"] + .045, R.t(f"shank-{sd}")[1] - .02)], 1.03, .3)
        line(of, "hood-seam", "torso", [(-.06, R.t("torso")[1] - .02), (0, R.t("torso")[1] - .07), (.06, R.t("torso")[1] - .02)], 3.25, .3)
    if t >= 2:
        poly(of, "hood", "torso", [(-.07, R.t("torso")[1] - .03), (-.085, R.t("torso")[1] + .02), (.085, R.t("torso")[1] + .02), (.07, R.t("torso")[1] - .03)], 3.2, s("hood"), follow=True)
        ell(op, "hood", "torso", R.t("torso")[0] - .06, 0.0, .06, .09, 4.1, s("hood"))


NOTES = notes({"200": "", "50": "Deck with grip line and nose kick, trucks with four wheels, trouser cuffs and hood seam."},
              {"50": "Deck mit Grip-Linie und Nose-Kick, Achsen mit vier Rollen, Hosenaufschläge und Kapuzennaht."})
NOTES = {k: v for k, v in NOTES.items()}
for k in NOTES: NOTES[k]["en"] = NOTES[k]["en"].replace("flat masses on the ground line", "flat masses").rstrip(". ") + ". The board is a thin deck on its trucks; the rider stands with both feet on the centre line, so legs and board read as one stack in the front."
for k in NOTES: NOTES[k]["de"] = NOTES[k]["de"].rstrip(". ") + ". Das Brett ist ein dünnes Deck auf seinen Achsen; der Fahrer steht mit beiden Füßen auf der Mittellinie, daher lesen sich Beine und Brett vorn als ein Stapel."
if __name__ == "__main__":
    print(write("skateboarder", build, NOTES))
