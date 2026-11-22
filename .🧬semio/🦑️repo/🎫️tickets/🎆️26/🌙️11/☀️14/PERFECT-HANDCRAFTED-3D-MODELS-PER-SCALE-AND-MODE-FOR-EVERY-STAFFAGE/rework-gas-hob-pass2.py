"""🍳 Pass 2 chisel for appliances/gas-hob: a cast pan support that really carries a small pot.

Hand-chosen numbers only, set absolutely (running it twice changes nothing). The four fingers reach over the
flame crown to 36 mm from the burner axis and thin out from 12 mm at the frame to 8 mm at the tip; the four legs
are cast webs of 9 mm at the foot with a gusset under the side rail. Writes the volumes source and a candidate
asset document (🗑️generated/r3-gas-hob/🔣️.json); `--publish` writes the catalog root (2D, texts, areas) too.
"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-hob/🔣️.json"
SOURCE = os.path.join(T, "volumes/appliances/gas-hob.json")
CANDIDATE = os.path.join(T, "🗑️generated/r3-gas-hob/🔣️.json")

BURNER_Z = 0.03
PLATE_TOP = 0.066
TIP_R = 0.036
FINGER = [[0.126, 0.0482], [0.039, 0.0482], [0.0365, 0.0468], [0.036, 0.0445], [0.0365, 0.0412], [0.039, 0.0402], [0.12, 0.0362], [0.126, 0.0362]]
LEG_FRONT = [[0.0, -0.105], [0.0, -0.096], [0.022, -0.095], [0.037, -0.083], [0.037, -0.105]]
LEG_BACK = [[0.0, 0.165], [0.037, 0.165], [0.037, 0.143], [0.022, 0.155], [0.0, 0.156]]
TRIVET_AREA = float(os.environ.get("TRIVET_AREA", "0.066"))

document = json.load(open(CATALOG))
volumes = json.load(open(SOURCE))["volumes"]
asset = document["asset"]


def solid(scale, group, ident):
    return next(s for s in volumes[scale][group] if s["id"] == ident)


def shape(scale, view, ident):
    return next(s for s in asset["representations"][scale]["geometry"][view] if s["id"] == ident)


def leg(ident, profile):
    old = solid("50", "body", ident)
    old.clear()
    old.update({"id": ident, "kind": "extrude", "bone": "trivet", "material": "cast-iron", "role": "body", "tone": 0.45,
                "profile": profile, "plane": "yz", "from": -0.135, "to": -0.125, "repeat": {"count": 2, "translation": [0.26, 0, 0]}})


def poly(target, points):
    for key in ("x", "y", "w", "h", "points", "kind"):
        target.pop(key, None)
    target.update({"kind": "poly", "points": points, "closed": True})


solid("50", "body", "trivet-fingers")["profile"] = FINGER
leg("trivet-legs-front", LEG_FRONT)
leg("trivet-legs-back", LEG_BACK)
fingers100 = solid("100", "body", "trivet-fingers")
fingers100["size"] = [0.09, 0.01, 0.01]
fingers100["transform"] = {"translation": [0.081, 0.043, BURNER_Z]}
for scale, idents in (("100", ("trivet-legs-front", "trivet-legs-back")), ("200", ("trivet-posts-front", "trivet-posts-back")), ("500", ("trivet-posts-front", "trivet-posts-back"))):
    for ident in idents:
        solid(scale, "body", ident).pop("cornerRadius", None)

poly(shape("50", "elevation", "trivet-rail"), [[-0.135, 0.0482], [-0.135, 0.036], [0.135, 0.036], [0.135, 0.0482]])
poly(shape("50", "front", "trivet-rail"), [[-0.105, 0.1142], [-0.105, 0.102], [0.165, 0.102], [0.165, 0.1142]])
poly(shape("50", "front", "trivet-leg-right"), [[-0.105, 0.066], [-0.096, 0.066], [-0.095, 0.088], [-0.083, 0.103], [-0.105, 0.103]])
poly(shape("50", "front", "trivet-leg-left"), [[0.165, 0.066], [0.165, 0.103], [0.143, 0.103], [0.155, 0.088], [0.156, 0.066]])
shape("50", "plan", "plan-trivet-back")["points"] = [[-0.135, 0.161], [-0.135, 0.155], [-0.005, 0.155], [-0.005, 0.0665], [0.0, 0.066], [0.005, 0.0665], [0.005, 0.155], [0.135, 0.155], [0.135, 0.161], [0.131, 0.165], [-0.131, 0.165]]
shape("50", "plan", "plan-trivet-front")["points"] = [[-0.131, -0.105], [0.131, -0.105], [0.135, -0.101], [0.135, -0.095], [0.005, -0.095], [0.005, -0.0065], [0.0, -0.006], [-0.005, -0.0065], [-0.005, -0.095], [-0.135, -0.095], [-0.135, -0.101]]
shape("50", "plan", "plan-trivet-left")["points"] = [[-0.125, 0.025], [-0.0365, 0.025], [-0.036, 0.03], [-0.0365, 0.035], [-0.125, 0.035], [-0.125, 0.155], [-0.135, 0.155], [-0.135, -0.095], [-0.125, -0.095]]
shape("50", "plan", "plan-trivet-right")["points"] = [[0.125, -0.095], [0.135, -0.095], [0.135, 0.155], [0.125, 0.155], [0.125, 0.035], [0.0365, 0.035], [0.036, 0.03], [0.0365, 0.025], [0.125, 0.025]]
shape("100", "plan", "plan-trivet")["d"] = (
    "M 0.135 0.152 L 0.1333 0.1585 L 0.1285 0.1633 L 0.122 0.165 L -0.122 0.165 L -0.1285 0.1633 L -0.1333 0.1585 L -0.135 0.152 "
    "L -0.135 -0.092 L -0.1333 -0.0985 L -0.1285 -0.1033 L -0.122 -0.105 L 0.122 -0.105 L 0.1285 -0.1033 L 0.1333 -0.0985 L 0.135 -0.092 Z "
    "M -0.125 0.025 L -0.036 0.025 L -0.036 0.035 L -0.125 0.035 L -0.125 0.155 L -0.005 0.155 L -0.005 0.066 L 0.005 0.066 L 0.005 0.155 "
    "L 0.125 0.155 L 0.125 0.035 L 0.036 0.035 L 0.036 0.025 L 0.125 0.025 L 0.125 -0.095 L 0.005 -0.095 L 0.005 -0.006 L -0.005 -0.006 "
    "L -0.005 -0.095 L -0.125 -0.095 Z"
)

next(r for r in asset["thermal"]["regions"] if r["id"] == "trivet")["areaM2"] = TRIVET_AREA

TEXTS = [
    ("on four legs with four fingers that droop towards the burner",
     "on four cast legs, webs of 9 mm at the foot with a gusset under the side rail, with four fingers that reach over the flame crown to 36 mm from the burner axis, so that a 0.1 m pot stands on them, and thin out from 12 mm at the frame to 8 mm at the tip"),
    ("auf vier Beinen mit vier Fingern, die sich zum Brenner neigen",
     "auf vier gegossenen Beinen, Stegen von 9 mm am Fuß mit einer Konsole unter dem Seitenstab, mit vier Fingern, die über den Flammenkranz bis 36 mm an die Brennerachse reichen, sodass ein Topf von 0,1 m darauf steht, und sich von 12 mm am Rahmen auf 8 mm an der Spitze verjüngen"),
    ("the pan support keeps its frame, four legs and four fingers as 10 by 12 mm bars",
     "the pan support keeps its frame, four legs and four fingers, which reach to 36 mm from the burner axis as at 1:50, as 10 by 12 mm bars"),
    ("der Pfannenträger behält Rahmen, vier Beine und vier Finger als Stäbe von 10 mal 12 mm",
     "der Pfannenträger behält Rahmen, vier Beine und vier Finger, die wie in 1:50 bis 36 mm an die Brennerachse reichen, als Stäbe von 10 mal 12 mm"),
]
blob = json.dumps(volumes, ensure_ascii=False)
for old, new in TEXTS:
    if new not in blob:
        if blob.count(old) != 1:
            sys.exit("text not found exactly once: " + old)
        blob = blob.replace(old, new)
volumes = json.loads(blob)

json.dump({"volumes": volumes}, open(SOURCE, "w"), ensure_ascii=False, indent=1)
candidate = json.loads(json.dumps(document))
candidate["asset"]["volumes"] = volumes
os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
json.dump(candidate, open(CANDIDATE, "w"), ensure_ascii=False, indent=1)
print("source and candidate written")

if "--publish" in sys.argv:
    scope = {}
    exec(open(os.path.join(T, "volumes-insert.py")).read().split("def member_end")[0], scope)
    with open(CATALOG, "w") as out:
        out.write(scope["dump"](candidate, 0) + "\n")
    print("catalog written")
