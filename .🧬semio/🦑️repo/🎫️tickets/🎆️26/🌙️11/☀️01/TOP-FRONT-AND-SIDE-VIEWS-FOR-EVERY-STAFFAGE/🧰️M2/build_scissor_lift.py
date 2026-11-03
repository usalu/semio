import sys
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️M2")
from lib import *
from rest import fk

A = "scissor-lift"
doc = load(A)
rest = fk(doc)
SS, CP, RU, DS, RS = "scissor-steel", "chassis-paint", "rubber", "deck-steel", "rail-steel"
ARMS = [f"{k}{i}" for i in range(1, 6) for k in "AB"]
ZARM = {"A1": 3.05, "B1": 3.07, "A2": 3.1, "B2": 3.12, "A3": 3.15, "B3": 3.17, "A4": 3.2, "B4": 3.22, "A5": 3.25, "B5": 3.27}
TONE = lambda b: 0.4 if b[0] == "A" else 0.5


ARMP = [(-0.07, -0.06), (1.77, -0.06), (1.77, 0.06), (-0.07, 0.06)]


def arms_front(w, latA, latB):
    out = []
    for b in ARMS:
        lat = latA if b[0] == "A" else latB
        for side, sg in (("l", 1), ("r", -1)):
            out.append(P(f"arm-{b.lower()}-{side}", b, ZARM[b], TONE(b), ARMP, "arms", SS, solid={"kind": "extruded", "z0": n(sg * lat - w / 2), "z1": n(sg * lat + w / 2)}))
    return out


def arms_plan(r, latA, latB):
    out = []
    for b in ARMS:
        lat = latA if b[0] == "A" else latB
        ox, tx = rest[b][0][0], rest[b][2][0]
        for side, sg in (("l", 1), ("r", -1)):
            out.append(C(f"arm-{b.lower()}-{side}", b, ZARM[b], TONE(b), ox, sg * lat, tx, sg * lat, r, r, "arms", SS, followTip=True))
    return out


def tyres_front(z=4):
    out = []
    for side, sg in (("l", 1), ("r", -1)):
        out.append(R(f"tyre-f-{side}", "wheel-f", z, 0.12, 0.245 if sg > 0 else -0.405, 0, 0.16, 0.54, "tyres", RU, r=0.06))
    return out


def tyres_plan():
    out = []
    for w, x in (("f", 0.55), ("r", -0.55)):
        for side, sg in (("l", 1), ("r", -1)):
            out.append(R(f"tyre-{w}-{side}", f"wheel-{w}", 2.5, 0.12, x - 0.27, 0.245 if sg > 0 else -0.405, 0.54, 0.16, "tyres", RU, r=0.05))
    return out


def rails_front(rail_h, mids, posts_w, box):
    out = [R("deck", "platform", 3.6, 0.5, -0.37, 1.0, 0.74, 0.1, "deck", DS),
           R("toe-board", "platform", 3.7, 0.3, -0.37, 1.1, 0.74, 0.15, "rails", RS),
           R("top-rail", "platform", 3.7, 0.3, -0.37, 2.2 - rail_h, 0.74, rail_h, "rails", RS)]
    for k, (y0, h) in enumerate(mids):
        out.append(R("mid-rail" if k == 0 else f"mid-rail-{k + 1}", "platform", 3.7, 0.3, -0.37, y0, 0.74, h, "rails", RS))
    for side, sg in (("l", 1), ("r", -1)):
        out.append(R(f"post-{side}", "platform", 3.75, 0.3, 0.37 - posts_w if sg > 0 else -0.37, 1.1, posts_w, 1.1, "rails", RS))
    if box:
        out.append(R("control-box", "platform", 3.8, 0.15, -0.405, 1.75, 0.095, 0.25, "rails", CP))
    return out


def rails_plan(w, box):
    out = [R("deck", "platform", 3.6, 0.5, -0.925, -0.37, 1.85, 0.74, "deck", DS),
           R("deck-ext", "deck-ext", 2.9, 0.55, -0.85, -0.35, 0.9, 0.7, "deck", DS),
           R("rail-l", "platform", 3.7, 0.3, -0.925, 0.37 - w, 1.85, w, "rails", RS),
           R("rail-r", "platform", 3.7, 0.3, -0.925, -0.37, 1.85, w, "rails", RS),
           R("rail-front", "platform", 3.7, 0.3, 0.925 - w, -0.37, w, 0.74, "rails", RS),
           R("rail-rear", "platform", 3.7, 0.3, -0.925, -0.37, w, 0.74, "rails", RS)]
    if box:
        out.append(R("control-box", "platform", 3.8, 0.15, 0.55, -0.405, 0.22, 0.095, "rails", CP))
    return out


chassis_f = R("chassis-body", "chassis", 2, 0.45, -0.37, 0.1, 0.74, 0.35, "chassis", CP)
chassis_p = R("chassis-body", "chassis", 2, 0.45, -0.925, -0.405, 1.85, 0.81, "chassis", CP)

front50 = [chassis_f, *tyres_front(), *arms_front(0.06, 0.34, 0.25), *rails_front(0.05, [(1.62, 0.05), (1.9, 0.05)], 0.05, True)]
plan50 = [chassis_p, *tyres_plan(), *arms_plan(0.03, 0.34, 0.25), *rails_plan(0.05, True)]
put(doc, 50, front50, plan50,
    "the chassis block, the two front tyres and the platform with its rail frame, toe board and control box are seen from the front; every arm is a pair of rect strips, one per scissor stack, that stretches with its bone so the stack grows as the lift rises. From above the deck and the rail ring cover the arms and chassis, the tyres peek out at both sides and each arm hides below the deck as a stretching capsule.",
    "gesehen werden von vorn der Fahrgestellblock, die beiden vorderen Reifen und die Bühne mit Geländerrahmen, Fußleiste und Steuerkasten; jeder Arm ist ein Paar Leisten, je eine pro Scherenpaket, die mit ihrem Knochen mitwächst, sodass das Paket beim Heben hochwächst. Von oben decken Boden und Geländerring Arme und Fahrgestell ab, die Reifen ragen an beiden Seiten hervor und jeder Arm liegt als sich streckende Kapsel unter dem Boden.")

front100 = [chassis_f, *tyres_front(), *arms_front(0.08, 0.34, 0.22), *rails_front(0.05, [(1.62, 0.05), (1.9, 0.05)], 0.05, False)]
plan100 = [chassis_p, *tyres_plan(), *arms_plan(0.04, 0.34, 0.22), *rails_plan(0.05, False)]
put(doc, 100, front100, plan100,
    "as at 1:50 without the control box, with arm strips 80 mm wide; the arms stay individually stretching strips in front and capsules under the deck in the top view.",
    "wie bei 1:50 ohne Steuerkasten und mit 80 mm breiten Armleisten; die Arme bleiben einzeln mitwachsende Leisten vorn und Kapseln unter dem Boden in der Draufsicht.")

front200 = [chassis_f, *tyres_front(), *arms_front(0.12, 0.33, 0.15), *rails_front(0.07, [(1.62, 0.07)], 0.07, False)]
plan200 = [chassis_p, *tyres_plan(), *rails_plan(0.07, False)]
put(doc, 200, front200, plan200,
    "front: ten arms as 120 mm strips in two stacks that stretch with their bones, chassis, tyres and rail frame; top: chassis, tyres, deck and rail ring (the arms lie hidden below the deck and are not drawn).",
    "vorn: zehn Arme als 120-mm-Leisten in zwei Paketen, die mit ihren Knochen mitwachsen, Fahrgestell, Reifen und Geländerrahmen; oben: Fahrgestell, Reifen, Boden und Geländerring (die Arme liegen verdeckt unter dem Boden und sind nicht gezeichnet).")
save(A, doc)
print("ok")
