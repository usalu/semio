"""🪜 Stepladder, author pass 1 of the second round: the shoes carry the frames in the structure of every scale, and every folding state stands on the edges of its rubber soles instead of sinking them into the ground; the platform is lifted off its rest tube before the frames close.

Hand-stated numbers (metres; front-frame local u up the stile from the front foot point, v behind it; rear-frame local u from the head hinge to the rear foot point):
front sole ends (0.016455, 0.039727) and (-0.017221, -0.041575), rear sole ends (1.394624, -0.03845) and (1.373675, 0.034605): both soles are level in the open ladder and pass through the foot points;
stile 112.5 degrees, rear rail 141.5 degrees from the stile when open, closing by 0 to 31.5 degrees; head hinge at u 1.439582;
platform lift 62 degrees, three quarters of it within the first third of the closing (the hand lifts the hooked rear rim off the rest tube first).
Usage: python3 rework-stepladder-pass1.py [--apply]
"""
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
from rework_15_kit import Kit, SCALES, dump, member_range, text

fold = {"math": math}
source = open(os.path.join(T, "rework-15-stepladder-fold.py"), encoding="utf-8").read()
exec(source[source.index("A_HINGE, B_RAIL"):source.index("def rebase")], fold)
A_HINGE, CLOSE, LIFT, OPEN_REL = fold["A_HINGE"], fold["CLOSE"], fold["LIFT"], fold["OPEN_REL"]
FRONT_SOLE = [(0.016455, 0.039727), (-0.017221, -0.041575)]
REAR_SOLE = [(1.394624, -0.03845), (1.373675, 0.034605)]
STEPS = [(0.5, "folding-1"), (2.0, "folding-2"), (5.0, "folding-3"), (10.0, "folding-4"), (15.0, "folding-5"), (20.0, "folding-6"), (25.0, "folding-7"), (CLOSE, "folded")]


def turn(p, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return (p[0] * c - p[1] * s, p[0] * s + p[1] * c)


def lows(lean, delta):
    """📏 Lowest sole point of the front and of the rear shoes above the front foot point for a root lean and a closing angle."""
    front = 112.5 + lean
    hinge = turn((A_HINGE, 0.0), front)
    rear = [tuple(a + b for a, b in zip(hinge, turn(p, front + OPEN_REL + delta))) for p in REAR_SOLE]
    return min(turn(p, front)[1] for p in FRONT_SOLE), min(p[1] for p in rear)


def stand(delta, guess):
    """⚖️ Root lean and lift at which the lowest sole edges of both shoe pairs touch the ground."""
    lo, hi = guess - 12.0, guess + 4.0
    for _ in range(60):
        mid = (lo + hi) / 2
        front, rear = lows(mid, delta)
        if rear > front:
            lo = mid
        else:
            hi = mid
    front, rear = lows((lo + hi) / 2, delta)
    return (lo + hi) / 2, -front


def lift(delta):
    return LIFT * (1.0 - (1.0 - delta / CLOSE) ** float(os.environ.get("LAW", "4")))


kit = Kit("objects/stepladder")
SHOES = [("shoe-front-l", "front-frame"), ("shoe-front-r", "front-frame"), ("shoe-rear-l", "rear-frame"), ("shoe-rear-r", "rear-frame")]
for scale in SCALES:
    structure = kit.solids(scale, "structure")
    for id, bone in SHOES:
        if kit.solid(scale, id) and not any(s.get("of") == id for s in structure):
            first = next(i for i, s in enumerate(structure) if s.get("part") == "support")
            structure.insert(first, {"id": "st-" + id, "of": id, "member": bone, "part": "member"})
    kit.decide(scale, "Structure: the four rubber shoes are members of their frames, so the load path runs stile or rail end, shoe, contact pad, ground without a gap.",
               "Tragwerk: Die vier Gummischuhe sind Stäbe ihrer Rahmen, der Lastpfad läuft also lückenlos Holmende, Schuh, Aufstandsfläche, Boden.")

for delta, id in STEPS:
    pose = next(p for p in kit.asset["poses"] if p["id"] == id)
    lean, up = stand(delta, fold["state"](delta)["root"])
    front, rear = lows(lean, delta)
    assert abs(front - rear) < 1e-6
    pose["root"] = {"angleDeg": round(lean, 4), "y": round(up, 5)}
    pose["bones"]["platform"] = {"angleDeg": round(lift(delta), 4)}
    print("[DEBUG] %s lean %.4f lift %.2f mm platform %.2f" % (id, lean, up * 1000, lift(delta)))
    if id == "folded":
        pose["description"] = text("Closed for carrying and storing: the rear frame lies 7 degrees off the stiles behind the treads, the spreader knuckles stand up, the platform is turned up; it stands on the edges of its four rubber soles, 0.18 m apart.",
                                   "Zum Tragen und Lagern geschlossen: der Stützrahmen liegt 7 Grad neben den Holmen hinter den Stufen, die Kniegelenke der Spreizen stehen nach oben, die Plattform ist hochgeklappt; sie steht auf den Kanten ihrer vier Gummisohlen, 0,18 m auseinander.")
    else:
        pose["description"] = text("In-between key of the folding clip: knuckles lifted, the platform already lifted off its rest tube, the ladder standing on the edges of its four soles.",
                                   "Zwischenstellung des Klapp-Clips: Kniegelenke angehoben, die Plattform bereits vom Auflagerohr gehoben, die Leiter steht auf den Kanten ihrer vier Sohlen.")
THICK = [[0.218, 0.053], [-0.032, 0.053], [-0.032, -0.017], [0.218, -0.017]]
kit.shape("200", "elevation", "platform")["points"] = THICK
kit.solid("200", "platform")["profile"] = THICK
kit.shape("200", "front", "platform")["points"] = [[-0.145, 0.99], [0.145, 0.99], [0.145, 1.06], [-0.145, 1.06]]
kit.volumes["200"]["decisions"][:] = [d for d in kit.volumes["200"]["decisions"] if "56 mm deep" not in d["en"] and "70 mm deep" not in d["en"]]
kit.decide("200", "The platform is drawn and modelled 70 mm deep (0.2 mm on paper at 1:350, the end of the band) downwards from its true standing surface at 1.06 m, so that it prints in every folding state too.", "Die Plattform ist von ihrer wahren Standfläche auf 1,06 m nach unten 70 mm stark gezeichnet und modelliert (0,2 mm auf dem Papier bei 1:350, dem Bandende), damit sie auch in jeder Klappstellung druckt.")
clip = next(c for c in kit.asset["clips"] if c["id"] == "fold-up")
clip["description"] = text("The platform is lifted off its rest tube and the spreader knuckles are lifted, the rear frame swings in on the head hinge; 4 s, then the ladder stands closed on its sole edges.",
                           "Die Plattform wird vom Auflagerohr gehoben, die Kniegelenke der Spreizen werden angehoben, der Stützrahmen schwenkt am Kopfgelenk ein; 4 s, danach steht die Leiter geschlossen auf ihren Sohlenkanten.")

out = os.path.join(T, "🗑️generated", "rework-stepladder")
os.makedirs(out, exist_ok=True)
with open(os.path.join(out, "🔣️.json"), "w", encoding="utf-8") as f:
    f.write(dump(kit.document, 0) + "\n")
print("candidate", os.path.join(out, "🔣️.json"))
if "--apply" in sys.argv:
    with open(kit.source, "w", encoding="utf-8") as f:
        f.write(dump({"volumes": kit.volumes}, 0) + "\n")
    fresh = open(kit.catalog, encoding="utf-8").read()
    if fresh != kit.text:
        raise SystemExit("catalog changed while building; rerun")
    result = fresh
    changed = [k for k in kit.asset if k != "volumes" and kit.asset[k] != kit.before.get(k)]
    for key in changed:
        a, b = member_range(result, key)
        result = result[:a] + dump(kit.asset[key], 2) + result[b:]
    assert all(json.loads(result)["asset"][k] == kit.asset[k] for k in kit.asset if k != "volumes")
    open(kit.catalog, "w", encoding="utf-8").write(result)
    print("source written; catalog members written:", ", ".join(changed) or "none")
