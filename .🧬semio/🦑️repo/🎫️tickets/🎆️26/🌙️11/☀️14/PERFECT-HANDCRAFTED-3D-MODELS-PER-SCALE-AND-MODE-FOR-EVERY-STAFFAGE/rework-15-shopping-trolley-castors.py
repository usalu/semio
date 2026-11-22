"""🛒 Rework 15 shopping trolley: the four swivel castors become a real mechanism (swivel bone on the kingpin axis, wheel bone on the axle), the nylon hubs get their ribs, and the 1:500 basket gets its front struts back.

Hand-stated numbers (chassis-local metres; chassis origin = rear end of the flat rail at world (-0.295, 0.1675)):
kingpin axes x 0.035 (rear) and 0.595 (front), z +-0.25 and +-0.198, top of the swivel at y -0.0125 (underside of the o25 rail);
axle 35 mm behind the kingpin (trail) and 92.5 mm below the swivel top, wheel o125 x 32.
Usage: python3 rework-15-shopping-trolley-castors.py [--apply]
"""
import math
import sys

from rework_15_kit import Kit, SCALES, rnd, text

kit = Kit("objects/shopping-trolley")
KING_Y, DROP, TRAIL = -0.0125, 0.0925, 0.035
POS = {"rear": (0.035, 0.25, ("Rear", "Hintere", "hinteren")), "front": (0.595, 0.198, ("Front", "Vordere", "vorderen"))}
SIDE = {"l": (1, ("left", "linke", "links")), "r": (-1, ("right", "rechte", "rechts"))}


def to_castor(p, jx, jz):
    return [-(p[1] - KING_Y), p[0] - jx] + ([p[2] - jz] if len(p) > 2 else [])


def to_wheel(p, jx, jz):
    return [p[0] - (jx - TRAIL), p[1] - (KING_Y - DROP)] + ([p[2] - jz] if len(p) > 2 else [])


def rebind_solid(s, bone, move, jx, jz):
    s["bone"] = bone
    if s["kind"] == "revolve":
        s["origin"] = move(s["origin"], jx, jz)
    elif s["kind"] == "capsule":
        s["p0"], s["p1"] = move(s["p0"], jx, jz), move(s["p1"], jx, jz)
    elif s["kind"] == "extrude":
        s["profile"] = [move(p, jx, jz) for p in s["profile"]]
        s["from"], s["to"] = s["from"] - jz, s["to"] - jz
    elif s["kind"] == "box":
        s["transform"]["translation"] = move(s["transform"]["translation"], jx, jz)
        if move is to_castor:
            s["size"] = [s["size"][1], s["size"][0], s["size"][2]]
    else:
        raise SystemExit("unhandled kind " + s["kind"])


for pos, (jx, zs, (pe, pd, pdd)) in POS.items():
    for side, (sign, (se, sd, sdd)) in SIDE.items():
        jz = sign * zs
        castor, wheel = "castor-%s-%s" % (pos, side), "wheel-%s-%s" % (pos, side)
        kit.add_bone("chassis" if (pos, side) == ("rear", "l") else last, {
            "id": castor, "parent": "chassis",
            "title": text("%s %s castor: fork swivelling about the kingpin" % (pe, se), "%s %s Lenkrolle: Gabel schwenkt um den Königszapfen" % (pd, sd)),
            "x": jx, "y": KING_Y, "z": jz, "angleDeg": -90.0, "lengthM": DROP, "joint": {"type": "revolute", "continuousTwist": True}})
        kit.add_bone(castor, {
            "id": wheel, "parent": castor,
            "title": text("%s %s wheel on its axle" % (pe, se), "%s %s Rad auf seiner Achse" % (pd, sd)),
            "x": DROP, "y": -TRAIL, "angleDeg": 90.0, "lengthM": 0.0625, "joint": {"type": "revolute"}})
        last = wheel
        for scale in SCALES:
            for s in kit.solids(scale):
                tail = "-%s-%s" % (pos, side)
                if s["bone"] != "chassis":
                    continue
                if not s["id"].endswith(tail) and ("-%s-%s-" % (pos, side)) not in s["id"]:
                    continue
                head = s["id"].split("-" + pos)[0]
                if head in ("tyre", "hub", "wheel"):
                    rebind_solid(s, wheel, to_wheel, jx, jz)
                elif head in ("castor-crown", "castor-axle", "castor-fork") or (head == "castor" and scale == "200"):
                    rebind_solid(s, castor, to_castor, jx, jz)
            for i, s in enumerate(kit.solids(scale)):
                kit.solids(scale)[i] = rnd(s, 6)
        kit.put("50", {"id": "hub-ribs" + "-%s-%s" % (pos, side), "kind": "box", "bone": wheel, "material": "pu-tyre", "role": "detail", "tone": 0.7,
                       "size": [0.0245, 0.0075, 0.031], "transform": {"translation": [0.02825, 0.0, 0.0]},
                       "repeat": {"count": 5, "rotationRad": 2 * math.pi / 5, "rotationAxis": [0, 0, 1], "origin": [0, 0, 0]}},
                region="wheels", member="chassis", after="hub-%s-%s" % (pos, side))
        for anchor in kit.asset["anchors"]:
            if anchor["id"] == "wheel-%s-%s" % (pos, {"l": "left", "r": "right"}[side]):
                anchor.update({"bone": castor, "x": 0.155, "y": -TRAIL, "z": 0})
    near = "-%s-r" % pos
    for scale in SCALES:
        for s in kit.shapes(scale, "elevation"):
            if s["bone"] != "chassis":
                continue
            if s["id"] in ("wheel-" + pos, "hub-" + pos, "tread-" + pos):
                s["bone"] = "wheel" + near
                s["cx"], s["cy"] = rnd(to_wheel([s["cx"], s["cy"]], jx, 0))
            elif s["id"] == "axle-" + pos:
                s["bone"] = "castor" + near
                s["cx"], s["cy"] = rnd(to_castor([s["cx"], s["cy"]], jx, 0))
            elif s["id"] == "fork-" + pos:
                s["bone"] = "castor" + near
                s["points"] = rnd([to_castor(p, jx, 0) for p in s["points"]])
        for view in ("front", "plan"):
            for s in kit.shapes(scale, view):
                for side in SIDE:
                    if s["id"] == "wheel-%s-%s" % (pos, side):
                        s["bone"] = "wheel-%s-%s" % (pos, side)
                    elif s["id"] == "fork-%s-%s" % (pos, side):
                        s["bone"] = "castor-%s-%s" % (pos, side)

for pos, (jx, zs, _) in POS.items():
    for side, (sign, _) in SIDE.items():
        tail, castor, wheel = "-%s-%s" % (pos, side), "castor-%s-%s" % (pos, side), "wheel-%s-%s" % (pos, side)
        for leg, (a, b) in (("o", (0.018, 0.033)), ("i", (-0.033, -0.018))):
            fork = kit.solid("100", "castor-fork%s-%s" % (tail, leg if sign > 0 else {"o": "i", "i": "o"}[leg]))
            fork["from"], fork["to"] = a, b
        kit.drop("500", ["castor-fork%s-o" % tail, "castor-fork%s-i" % tail, "castor-crown" + tail, "castor-axle" + tail])
        kit.put("500", {"id": "castor-housing" + tail, "kind": "box", "bone": castor, "material": "zinc-tube", "role": "body", "tone": 0.6,
                        "size": [0.075, 0.075, 0.085], "transform": {"translation": [0.0585, -0.0175, 0.0]}},
                region="frame-tube", member=castor, after="wheel" + tail)
        kit.solid("500", "wheel" + tail)["profile"] = [[-0.0375, 0.054], [-0.029, 0.0625], [0.029, 0.0625], [0.0375, 0.054]]
        pad = kit.solid("500", "support-wheel" + tail, "structure")
        pad.pop("of", None)
        pad.update({"kind": "box", "bone": "chassis", "material": "pu-tyre", "role": "ground", "tone": 0.3, "size": [0.03, 0.03, 0.03],
                    "transform": {"translation": [jx - TRAIL, -0.1525, sign * zs]}, "member": "chassis"})
        if kit.solid("500", "st-wheel" + tail, "structure") is None:
            kit.volumes["500"]["structure"].append({"id": "st-wheel" + tail, "of": "wheel" + tail, "member": wheel, "part": "member"})

for scale in SCALES:
    bones = {s["id"]: s["bone"] for s in kit.solids(scale)}
    for s in kit.solids(scale, "structure"):
        if bones.get(s.get("of"), "chassis") != "chassis" and s["member"] == "chassis":
            s["member"] = bones[s["of"]]

for sign, side in ((1, "l"), (-1, "r")):
    kit.put("500", {"id": "strut-" + side, "kind": "capsule", "bone": "chassis", "material": "zinc-tube", "role": "body", "tone": 0.62,
                    "p0": [0.595, 0.0125, sign * 0.1987], "p1": [0.557, 0.3945, sign * 0.186], "r0": 0.0095, "r1": 0.0095, "slender": True},
            region="frame-tube", member="chassis", after="frame-%s-2" % side)

kit.decide("50", "The castors work: fork, crown and axle hang on a swivel bone on the kingpin axis (free axial twist), tyre and ribbed nylon hub turn on a wheel bone 35 mm behind it; top plate and ball-race head stay on the chassis.",
           "Die Lenkrollen funktionieren: Gabel, Gabelkopf und Achse hängen an einem Schwenkknochen auf der Königszapfenachse (freie Drehung um die Längsachse), Reifen und gerippte Nylonnabe drehen auf einem Radknochen 35 mm dahinter; Anschraubplatte und Kugellagerkopf bleiben am Fahrgestell.")
for scale in ("100", "200", "500"):
    kit.decide(scale, "Castor forks swivel on their kingpin bones and the wheels turn on their axle bones, as at 1:50.", "Die Rollengabeln schwenken an ihren Königszapfenknochen und die Räder drehen an ihren Achsknochen wie in 1:50.")
kit.decide("500", "The two front struts under the basket floor are kept as slender o25 rods so the basket is carried at the front and does not float.", "Die beiden vorderen Streben unter dem Korbboden bleiben als schlanke Stäbe o25 erhalten, damit der Korb vorn getragen wird und nicht schwebt.")
kit.finish(sys.argv)
