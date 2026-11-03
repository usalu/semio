import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from human import *

P = dict(hl=.075, t0=.075, t1=.057, s0=.055, s1=.038, fw=.045, ww=.115, cw=.125, sw=.135, nw=.04, ac=.11, a0=.04, a1=.034, f0=.034, f1=.028, hr=.027, hw=.07, pd=.1, tl=.045, cd=.095, sd=.085)
KEY = {"thigh": "thigh-{s}", "thigh-over": "shorts-{s}", "shank": "shank-{s}", "shoe": "shoe-{s}", "toe": "shoe-toe-{s}", "shoe-toe": "shoe-toe-{s}", "hips": "pelvis-shorts", "pelvis": "pelvis-shorts", "torso": "singlet",
       "sleeve": "arm-{s}", "forearm": "forearm-{s}", "hand": "hand-{s}", "neck": "neck", "skin": "head", "hair": "hair", "skull": "head"}


def build(a, sc, of, op):
    h = Human(a, coarse(P, sc), KEY, bones={"foot-r": "shank-r", "toe-r": "shank-r"}); R = h.R; t = tier(sc); hl = P["hl"]
    sk = ("lower-r",)
    h.front(sc, of, skip=sk); h.plan(sc, op, skip=sk)
    over(h, sc, of, op, 0.2)
    s = lambda i: h.st.get(sc, i)
    bw = .09 if t == 0 else .03
    sy0, sy1 = R.o("shank-r")[1], R.t("shank-r")[1]
    limbsol(h, of, "socket", "shank-r", .052, .042, -hl, 3.02, s("socket"))
    cap(op, "socket", "shank-r", R.o("shank-r")[0], -hl, R.t("shank-r")[0], -hl, .052, .042, 3.02, s("socket"), follow=True)
    by0, by1 = R.o("blade-r")[1], R.t("blade-r")[1]
    limbsol(h, of, "blade-upper", "blade-r", .012, .012, -hl, 2.5, s("blade-upper"), zr=bw)
    cap(op, "blade-upper", "blade-r", R.o("blade-r")[0], -hl, R.t("blade-r")[0], -hl, bw, bw, 2.5, s("blade-upper"), follow=True)
    cy0, cy1 = R.o("blade-tip-r")[1], R.t("blade-tip-r")[1]
    limbsol(h, of, "blade-curl", "blade-tip-r", .012, .012, -hl, 2.5, s("blade-curl"), zr=bw)
    cap(op, "blade-curl", "blade-tip-r", R.o("blade-tip-r")[0], -hl, R.t("blade-tip-r")[0], -hl, bw, bw, 2.5, s("blade-curl"), follow=True)
    ell(of, "blade-toe", "blade-tip-r", -hl, cy1 + 0.014, .036 if t else .08, .016 if t else .06, 2.6, s("blade-toe"))
    cap(op, "blade-toe", "blade-tip-r", R.t("blade-tip-r")[0] - 0.05, -hl, R.t("blade-tip-r")[0] + 0.02, -hl, .034, .034, 2.6, s("blade-toe"))
    if t >= 3:
        line(of, "socket-trim", "shank-r", [(-hl - .05, sy0 - 0.005), (-hl + .05, sy0 - 0.005)], 3.1, .3)
        line(of, "socket-seam", "shank-r", [(-hl, sy0), (-hl, sy1 + 0.02)], 3.12, .1, role="texture")
        line(of, "blade-lamination", "blade-r", [(-hl + .01, by0 - .02), (-hl + .01, by1 + .02)], 2.7, .9, role="texture")
        line(of, "blade-bolt", "blade-r", [(-hl - .02, by0 - 0.006), (-hl + .02, by0 - 0.006)], 2.7, .1)
        line(of, "toe-tread", "blade-tip-r", [(-hl - .03, cy1 + 0.02), (-hl + .03, cy1 + 0.02)], 2.7, .1, role="texture")
    if t >= 2:
        line(of, "singlet-neck", "torso", [(-.035, R.t("torso")[1] - 0.004), (0, R.t("torso")[1] - .035), (.035, R.t("torso")[1] - .004)], 4.1, .1)
    if t >= 3:
        line(of, "bib", "torso", [(-.07, R.o("torso")[1] + .12), (.07, R.o("torso")[1] + .12), (.07, R.o("torso")[1] + .26), (-.07, R.o("torso")[1] + .26), (-.07, R.o("torso")[1] + .12)], 4.15, .9)


NOTES = notes({"500": "The prosthetic leg is a socket and a blade on the right.", "200": "The right leg is a socket with a blade; the shorts cover the upper thighs.", "100": "The right leg is a socket with the carbon blade and its toe pad; shorts on the upper thighs.", "50": "The right leg has socket trim, seam, blade lamination, bolt and toe tread; the singlet shows neck and bib."},
              {"500": "Das Prothesenbein rechts ist Schaft und Blade.", "200": "Das rechte Bein ist ein Schaft mit Blade; die Shorts bedecken die Oberschenkel.", "100": "Das rechte Bein ist ein Schaft mit dem Carbon-Blade und seinem Zehenpad; Shorts auf den Oberschenkeln.", "50": "Das rechte Bein zeigt Schaftrand, Naht, Blade-Lamellen, Schraube und Zehenprofil; das Trikot zeigt Ausschnitt und Startnummer."})
if __name__ == "__main__":
    print(write("runner-with-blade-prosthesis", build, NOTES))
