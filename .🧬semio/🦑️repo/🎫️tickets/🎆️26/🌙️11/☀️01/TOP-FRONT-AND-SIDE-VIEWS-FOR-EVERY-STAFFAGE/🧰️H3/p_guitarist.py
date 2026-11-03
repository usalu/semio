import sys, math
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️H3")
from human import *

P = dict(solid_thigh=False, hl=.105, t0=.10, t1=.075, s0=.07, s1=.05, fw=.055, drop=.077, ww=.17, cw=.19, sw=.21, nw=.06, ac=.218, a0=.054, a1=.048, f0=.05, f1=.04, hr=.04, hw=.082, pd=.125, tl=.08, cd=.125, sd=.12)
KEY = {"thigh": "thigh-{s}", "shank": "shank-{s}", "shoe": "boot-{s}", "toe": "boot-{s}", "hips": "thigh-l", "pelvis": "shirt", "torso": "shirt", "hem": "shirt",
       "sleeve": "upper-arm-{s}", "forearm": "forearm-{s}", "hand": "hand-{s}", "neck": "head", "skin": "head", "hair": "hair", "skull": "head"}
BONES = {"neck": "head", "arm-l": "upper-arm-l", "arm-r": "upper-arm-r"}


def build(a, sc, of, op):
    h = Human(a, coarse(P, sc), KEY, bones=BONES); R = h.R; t = tier(sc)
    sk = ("neck", "toe")
    h.front(sc, of, skip=sk); h.plan(sc, op, skip=sk)
    s = lambda i: h.st.get(sc, i)
    g = "guitar"
    gp = lambda lx, ly=0: R.p(g, lx, ly)
    gb, gn, gh = s("guitar-body"), s("guitar-neck"), s("guitar-head")
    c, n = gp(-.217), gp(.335)
    for view, out in (("f", of), ("p", op)):
        poly(out, "guitar-body", g, [(-.217, -.193), (.0, -.2), (.335, -.18), (.335, .18), (.0, .2), (-.217, .193)], 3.9, gb); sol(out, -.055, .055)
        poly(out, "guitar-neck", g, [(.31, -.024), (.64, -.024), (.64, .024), (.31, .024)], 3.85, gn); sol(out, -.024 if t else -.05, .024 if t else .05)
        poly(out, "guitar-head", g, [(.64, -.044), (.78, -.044), (.78, .044), (.64, .044)], 3.86, gh); sol(out, -.04, .04)
    if t >= 2:
        ell(of, "soundhole", g, 0.0, gp(.075)[1], .02, .02, 3.95, s("soundhole"), role="detail") if False else None
        line(op, "bridge", g, [(gp(-.1)[0], -.05), (gp(-.1)[0], .05)], 3.95, .2)
    if t >= 3:
        for i, lx in enumerate((.36, .41, .46, .505, .55, .59)):
            line(op, f"fret-{i}", g, [(gp(lx)[0], -.02), (gp(lx)[0], .02)], 3.96, .2, role="texture")
        line(op, "string", g, [(gp(-.1)[0], 0.0), (gp(.62)[0], 0.0)], 3.97, .2, role="texture")
        for i, ly in enumerate((.03, 0.0, -.03)):
            ell(op, f"tuner-{i}", g, gp(.76 - i * .035)[0], ly, .007, .007, 3.95, s("tuner-0"), role="detail")
    # stool: round pine seat on three legs
    st = s("stool")
    sx = R.o("stool")
    top = sx[1] + .56
    poly(of, "stool-seat", "stool", [(-.205, top - .04), (-.205, top), (.205, top), (.205, top - .04)], .6, st)
    poly(of, "stool-leg-l", "stool", [(.17, top - .04), (.21, 0.0 + R.o("stool")[1] - .0), (.17, 0.0 + R.o("stool")[1] - .0), (.12, top - .04)], .55, st)
    poly(of, "stool-leg-r", "stool", [(-.17, top - .04), (-.21, R.o("stool")[1]), (-.17, R.o("stool")[1]), (-.12, top - .04)], .55, st)
    if t >= 1:
        poly(of, "stool-rung", "stool", [(-.19, .25), (-.19, .27), (.19, .27), (.19, .25)], .56, st)
    ell(op, "stool-seat", "stool", 0.0, 0.0, .205, .205, .6, st)
    if t >= 2:
        ell(op, "stool-rim", "stool", 0.0, 0.0, .17, .17, .61, st, role="detail") if False else None
    if t >= 3:
        for i, an in enumerate((90, 210, 330)):
            line(op, f"stool-leg-{i}", "stool", [(.12 * math.cos(math.radians(an)), .12 * math.sin(math.radians(an))), (.19 * math.cos(math.radians(an)), .19 * math.sin(math.radians(an)))], .62, .2)
    if t >= 3:
        for sd, sg in SIDES:
            line(of, f"cuff-{sd}", f"forearm-{sd}", [(sg * .218 - .04, R.t(f"forearm-{sd}")[1] + .0), (sg * .218 + .04, R.t(f"forearm-{sd}")[1] + .0)], 4.7, .3)


NOTES = notes({"500": "The guitar is a narrow strip on the lap and the stool a seat on two legs.", "200": "The guitar shows its narrow edge: thin body, neck and headstock; the stool has a round seat on three legs.", "100": "The guitar shows body edge, neck and headstock; the stool has a seat, legs and a rung.", "50": "Guitar with bridge, six frets, string and three tuners; stool with its three legs seen from above."},
              {"500": "Die Gitarre ist ein schmaler Streifen auf dem Schoß, der Hocker eine Sitzfläche auf zwei Beinen.", "200": "Die Gitarre zeigt ihre schmale Kante: dünner Korpus, Hals und Kopfplatte; der Hocker hat eine runde Sitzfläche auf drei Beinen.", "100": "Die Gitarre zeigt Korpuskante, Hals und Kopfplatte; der Hocker hat Sitz, Beine und eine Sprosse.", "50": "Gitarre mit Steg, sechs Bünden, Saite und drei Mechaniken; Hocker mit seinen drei Beinen von oben."})
if __name__ == "__main__":
    print(write("seated-guitarist", build, NOTES))
