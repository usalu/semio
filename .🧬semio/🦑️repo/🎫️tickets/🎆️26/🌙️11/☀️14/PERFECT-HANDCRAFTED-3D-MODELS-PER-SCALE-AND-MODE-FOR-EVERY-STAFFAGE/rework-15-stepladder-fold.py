"""🪜 Rework 15 stepladder: the ladder folds. The rear frame turns on the head hinge, each spreader is two links on their frame pivots meeting in the knuckle (which rises as the ladder closes), and the platform turns up on its axle.

Hand-stated numbers (metres, front-frame local: u up the stile from the front shoe, v behind the stile):
head hinge on the stile axis at u 1.439582; rear rail 1.383598 long, 38.5 degrees from the stile when open, 7 degrees when closed (the rails stop behind the tread backs, v 34 mm);
spreader pivots at u 0.573668 on the stile and 0.83224 from the hinge on the rail, links 280.384 mm between holes with 12 mm ends, flat 20 x 8 mm on edge;
platform axle at u 1.128935, deck 244 mm deep, turned up by 62 degrees when closed.
Usage: python3 rework-15-stepladder-fold.py [--apply]
"""
import copy
import math
import sys

from rework_15_kit import Kit, SCALES, rnd, text

kit = Kit("objects/stepladder")
A_HINGE, B_RAIL, LINK, END = 1.439582, 1.383598, 0.280384, 0.012
PF_U, PR_U, AXLE_U, DECK = 0.573668, 0.83224, 1.128935, 0.244
OPEN_REL, CLOSE, LIFT = 141.5, 31.5, -62.0
BONES = {"platform": ("front-frame", AXLE_U, 0.0, 67.5, DECK), "spreader-front": ("front-frame", PF_U, 0.0, 67.5, LINK), "spreader-rear": ("rear-frame", PR_U, 0.0, 106.0, LINK)}


def norm(deg):
    return (deg + 180.0) % 360.0 - 180.0


def state(delta):
    """📐 Offsets of every moving bone and the root lean for a closing angle `delta` of the rear frame (0 open, 31.5 closed), all four shoes on the ground."""
    rel = math.radians(OPEN_REL + delta)
    rail = (math.cos(rel), math.sin(rel))
    foot = (A_HINGE + B_RAIL * rail[0], B_RAIL * rail[1])
    root = 180.0 - math.degrees(math.atan2(foot[1], foot[0])) - 112.5
    pf, pr = (PF_U, 0.0), (A_HINGE + PR_U * rail[0], PR_U * rail[1])
    half = math.dist(pf, pr) / 2
    beta = math.degrees(math.acos(min(1.0, half / LINK)))
    psi = math.degrees(math.atan2(pr[1] - pf[1], pr[0] - pf[0]))
    front = psi - beta
    knuckle = (pf[0] + LINK * math.cos(math.radians(front)), pf[1] + LINK * math.sin(math.radians(front)))
    rear = math.degrees(math.atan2(knuckle[1] - pr[1], knuckle[0] - pr[0]))
    return {"root": root, "bones": {
        "rear-frame": {"angleDeg": round(delta, 4)},
        "spreader-front": {"angleDeg": round(norm(front - 67.5), 4)},
        "spreader-rear": {"angleDeg": round(norm(rear - (OPEN_REL + delta) - 106.0), 4)},
        "platform": {"angleDeg": round(LIFT * delta / CLOSE, 4)}}}


def rebase(p, bone):
    _, bx, by, deg, _ = BONES[bone]
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    x, y = p[0] - bx, p[1] - by
    return [x * c + y * s, -x * s + y * c] + list(p[2:])


kit.bone("rear-frame")["joint"] = {"type": "revolute", "minDeg": 0, "maxDeg": CLOSE}
TITLES = {"platform": ("Platform on its axle", "Plattform auf ihrer Achse"), "spreader-front": ("Front spreader links (stile pivot to knuckle)", "Vordere Spreizglieder (Holmbolzen bis Kniegelenk)"),
          "spreader-rear": ("Rear spreader links (rail pivot to knuckle)", "Hintere Spreizglieder (Stützholmbolzen bis Kniegelenk)")}
LIMITS = {"platform": (LIFT, 0), "spreader-front": (-90, 0), "spreader-rear": (0, 90)}
for id, (parent, x, y, deg, length) in BONES.items():
    kit.add_bone("rear-frame", {"id": id, "parent": parent, "title": text(*TITLES[id]), "x": x, "y": y, "angleDeg": deg, "lengthM": length,
                                "joint": {"type": "revolute", "minDeg": LIMITS[id][0], "maxDeg": LIMITS[id][1]}})

for scale in SCALES:
    body = kit.solids(scale)
    for s in body:
        if s["id"].startswith("platform") and s["id"] != "platform-axle" and s["bone"] == "front-frame":
            s["bone"] = "platform"
            s["profile"] = rnd([rebase(p, "platform") for p in s["profile"]], 6)
        if s["id"].startswith("spreader-knuckle") and s["bone"] == "front-frame":
            s["bone"] = "spreader-front"
            s["origin"] = [LINK, 0.0, s["origin"][2]]
    olds = [s for s in body if s["id"] in ("spreader-l", "spreader-r", "spreader-front-l", "spreader-front-r", "spreader-rear-l", "spreader-rear-r") and "transform" in s]
    for old in olds:
        side = old["id"][-1]
        z = old["transform"]["translation"][2]
        for part in (("front", "rear") if old["id"] in ("spreader-l", "spreader-r") else (old["id"].split("-")[1],)):
            link = {"id": "spreader-%s-%s" % (part, side), "kind": "extrude", "bone": "spreader-" + part, "material": old["material"], "role": old["role"], "tone": old["tone"],
                    "profile": [[p[0], z + p[1]] for p in old["profile"]], "plane": "yz", "from": -END, "to": LINK + END}
            if old.get("slender"):
                link["slender"] = True
            if old["id"] in ("spreader-l", "spreader-r"):
                kit.put(scale, link, region="spreaders", member="spreader-" + part, after=old["id"])
            else:
                kit.put(scale, link)
    kit.drop(scale, ["spreader-l", "spreader-r"])
    bones = {s["id"]: s["bone"] for s in kit.solids(scale)}
    for s in kit.solids(scale, "structure"):
        if bones.get(s.get("of")) in BONES:
            s["member"] = bones[s["of"]]

    for s in kit.shapes(scale, "elevation"):
        if s["id"] == "platform" and s["bone"] == "front-frame":
            s["bone"] = "platform"
            s["points"] = rnd([rebase(p, "platform") for p in s["points"]])
        elif s["id"] == "spreader-front" and s["bone"] == "front-frame":
            s["bone"] = "spreader-front"
            s["points"] = rnd([rebase(p, "spreader-front") for p in s["points"]])
        elif s["id"] == "spreader-rear" and s["bone"] == "rear-frame":
            s["bone"] = "spreader-rear"
            s["points"] = rnd([rebase(p, "spreader-rear") for p in s["points"]])
        elif s["id"] == "spreader-knuckle" and s["bone"] == "front-frame":
            s["bone"] = "spreader-front"
            s["cx"], s["cy"] = LINK, 0.0
    single = kit.shape(scale, "elevation", "spreader")
    if single is not None:
        for part in ("front", "rear"):
            stroke = copy.deepcopy(single)
            stroke.update({"id": "spreader-" + part, "bone": "spreader-" + part, "points": [[0.0, 0.0], [LINK, 0.0]]})
            kit.put_shape(scale, "elevation", stroke, after="spreader")
        kit.drop_shapes(scale, "elevation", ["spreader"])
    KNUCKLE_X = 0.4636 + PF_U * math.cos(math.radians(112.5)) - LINK
    for view in ("front", "plan"):
        for s in list(kit.shapes(scale, view)):
            if s["id"].startswith("platform") and not s["id"].startswith("platform-axle") and s["bone"] == "front-frame":
                s["bone"] = "platform"
            elif s["id"] in ("spreader-front-l", "spreader-front-r") and s["bone"] == "front-frame":
                s["bone"] = "spreader-front"
            elif s["id"] in ("spreader-rear-l", "spreader-rear-r") and s["bone"] == "rear-frame":
                s["bone"] = "spreader-rear"
            elif s["id"] in ("spreader-l", "spreader-r") and view == "front":
                s["bone"] = "spreader-front"
            elif s["id"] in ("spreader-l", "spreader-r") and view == "plan":
                (x0, z), (x1, _) = s["points"]
                for part, span in (("front", (KNUCKLE_X, x1)), ("rear", (x0, KNUCKLE_X))):
                    stroke = copy.deepcopy(s)
                    stroke.update({"id": "spreader-%s-%s" % (part, s["id"][-1]), "bone": "spreader-" + part, "points": [[span[0], z], [span[1], z]]})
                    kit.put_shape(scale, view, stroke, after=s["id"])
                kit.drop_shapes(scale, view, [s["id"]])

THICK = [[0.218, 0.012], [-0.032, 0.012], [-0.032, -0.044], [0.218, -0.044]]
kit.shape("200", "elevation", "platform")["points"] = THICK
kit.solid("200", "platform")["profile"] = THICK
kit.shape("200", "front", "platform")["points"] = [[-0.145, 1.006], [0.145, 1.006], [0.145, 1.062], [-0.145, 1.062]]
kit.decide("200", "The platform is drawn and modelled 56 mm deep (deck with its side cheeks) so that it prints over the whole band.", "Die Plattform ist 56 mm stark gezeichnet und modelliert (Deck mit Seitenwangen), damit sie im ganzen Band druckt.")

poses = kit.asset["poses"]
poses[:] = [p for p in poses if not p["id"].startswith("fold")]
base = next(p for p in poses if p["id"] == "open-empty")
STEPS = [(0.5, "folding-1"), (2.0, "folding-2"), (5.0, "folding-3"), (10.0, "folding-4"), (15.0, "folding-5"), (20.0, "folding-6"), (25.0, "folding-7"), (CLOSE, "folded")]
for delta, id in STEPS:
    pose = copy.deepcopy(base)
    now = state(delta)
    pose.update({"id": id, "bones": now["bones"], "root": {"angleDeg": round(now["root"], 4)}})
    if id == "folded":
        pose["title"] = text("Folded", "Zusammengeklappt")
        pose["description"] = text("Closed for carrying and storing: the rear frame lies 7 degrees off the stiles behind the treads, the spreader knuckles stand up, the platform is turned up; it still stands on its four shoes, 0.18 m apart.",
                                   "Zum Tragen und Lagern geschlossen: der Stützrahmen liegt 7 Grad neben den Holmen hinter den Stufen, die Kniegelenke der Spreizen stehen nach oben, die Plattform ist hochgeklappt; sie steht noch auf ihren vier Schuhen, 0,18 m auseinander.")
    else:
        pose["key"] = True
        pose["title"] = text("Folding, rear frame closed by %g degrees" % delta, "Beim Zusammenklappen, Stützrahmen um %s Grad geschlossen" % str(delta).replace(".", ","))
        pose["description"] = text("In-between key of the folding clip: knuckles lifted, platform rising, all four shoes on the ground.", "Zwischenstellung des Klapp-Clips: Kniegelenke angehoben, Plattform steigt, alle vier Schuhe am Boden.")
    poses.append(pose)

clips = kit.asset["clips"]
clips[:] = [c for c in clips if c["id"] != "fold-up"]
feet = base["contacts"]
clips.append({
    "id": "fold-up", "title": text("Fold up", "Zusammenklappen"),
    "description": text("The spreader knuckles are lifted, the rear frame swings in on the head hinge and the platform turns up; 4 s, then the ladder stands closed.",
                        "Die Kniegelenke der Spreizen werden angehoben, der Stützrahmen schwenkt am Kopfgelenk ein und die Plattform klappt hoch; 4 s, danach steht die Leiter geschlossen."),
    "durationS": 5, "fps": 15, "loop": False, "rootMotion": "none",
    "keys": [{"t": 0, "pose": "open-empty", "ease": "linear"}] + [{"t": round(0.5 + 3.5 * delta / CLOSE, 3), "pose": id, "ease": "linear"} for delta, id in STEPS] + [{"t": 5, "pose": "folded", "ease": "linear"}],
    "contacts": [{"anchor": a, "fromS": 0, "toS": 5} for a in feet],
    "phases": [{"id": "unlock", "fromS": 0, "toS": 0.7, "title": text("Lifting the knuckles", "Kniegelenke anheben")}, {"id": "close", "fromS": 0.7, "toS": 4, "title": text("Closing the frames", "Rahmen schließen")},
               {"id": "closed", "fromS": 4, "toS": 5, "title": text("Closed", "Geschlossen")}],
    "events": [], "tracks": []})

for scale in SCALES:
    kit.decide(scale, "The ladder folds: rear frame on the head hinge (0 to 31.5 degrees), each spreader as two links on their own bones meeting in the knuckle, the platform on its axle bone; pose `folded` and clip `fold-up` use them.",
               "Die Leiter klappt: Stützrahmen am Kopfgelenk (0 bis 31,5 Grad), jede Spreize als zwei Glieder an eigenen Knochen, die sich im Kniegelenk treffen, die Plattform an ihrem Achsknochen; Pose `folded` und Clip `fold-up` nutzen sie.")
kit.finish(sys.argv)
