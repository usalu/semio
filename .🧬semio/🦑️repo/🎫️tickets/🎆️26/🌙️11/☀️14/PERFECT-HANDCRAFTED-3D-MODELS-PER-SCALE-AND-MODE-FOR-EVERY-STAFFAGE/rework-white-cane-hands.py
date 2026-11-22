"""🖐️ Finger bones of `humans/person-with-white-cane`: fifteen bones per hand (thumb metacarpal and two phalanges, three phalanges per finger) whose rest placement is the modelled hand (right: grasp of the long cane with the index along the grip; left: relaxed), the finger solids of body and skeleton bound to them, the right thumb given its missing phalanx, and the fist closed round the upright cane in `cane-parked`.

Every point is stated by hand in the frame of the hand bone (u along the hand, v towards the thumb, w to the left; metres). Writes the candidate document; `--publish` writes the catalog asset and the volumes source.

See ./rework-white-cane-lib.py, ./rig-lateral-rest.md (1.1) and ./rework/assets/humans--person-with-white-cane.md.
"""
import importlib.util
import json
import math
import os
import sys

_spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-white-cane-lib.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

HAND = K.Frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
FINGERS = {"1": "index", "2": "middle", "3": "ring", "4": "little"}
SEGMENTS = {"": "proximal", "-mid": "middle", "-tip": "distal"}
THUMB = {"": "metacarpal", "-mid": "proximal", "-tip": "distal"}
TITLE = {
    "index": ("index finger", "Zeigefinger"), "middle": ("middle finger", "Mittelfinger"), "ring": ("ring finger", "Ringfinger"), "little": ("little finger", "Kleinfinger"), "thumb": ("thumb", "Daumen"),
    "proximal": ("proximal phalanx", "Grundglied"), "middle-seg": ("middle phalanx", "Mittelglied"), "distal": ("distal phalanx", "Endglied"), "metacarpal": ("metacarpal", "Mittelhandknochen"),
}
LIMITS = {"proximal": (-100, 100), "middle": (-110, 110), "distal": (-80, 80), "metacarpal": (-60, 60)}
THUMB_R = {"mid": [0.0755, 0.0265, 0.0275], "tip": [0.0965, 0.0295, 0.0255]}
PARKED = {
    "index": [[0.085, 0.03, 0.0515], [0.059, 0.03, 0.049], [0.041, 0.03, 0.04]],
    "thumb": [[0.027, 0.027, 0.042], [0.054, 0.02, 0.052], [0.073, 0.012, 0.05]],
}


def axis(side, finger):
    return [0.0, 0.0, -1.0] if finger == "thumb" else [0.0, -1.0 if side == "r" else 1.0, 0.0]


def relative(parent, child):
    return [[K.dot(a, b) for a in (parent.u, parent.v, parent.w)] for b in (child.u, child.v, child.w)]


def carried(posed, rel, origin):
    ax = [[sum(r[i] * (posed.u, posed.v, posed.w)[i][k] for i in range(3)) for k in range(3)] for r in rel]
    return K.Frame(list(origin), ax[0], ax[1], ax[2])


def right_thumb(volumes):
    """👍 The right thumb had one solid for both phalanges: proximal and distal phalanx lie along the top of the grip."""
    for name, lst, r in (("hand-r-thumb", "body", ((0.01, 0.009), (0.009, 0.0078))), ("bones-r-thumb", "structure", ((0.004, 0.0038), (0.004, 0.0038)))):
        solids = volumes["50"][lst]
        i = next(k for k, s in enumerate(solids) if s["id"] == name + "-tip")
        tip = solids[i]
        mid = {**tip, "id": name + "-mid", "p0": list(tip["p0"]), "p1": list(THUMB_R["mid"]), "r0": r[0][0], "r1": r[0][1]}
        tip["p0"], tip["p1"], tip["r0"], tip["r1"] = list(THUMB_R["mid"]), list(THUMB_R["tip"]), r[1][0], r[1][1]
        solids.insert(i, mid)
    th = volumes["50"]["thermal"]
    i = next(k for k, s in enumerate(th) if s["id"] == "th-hand-r-thumb-tip")
    th.insert(i, {"id": "th-hand-r-thumb-mid", "of": "hand-r-thumb-mid", "region": "hands"})


def build(asset):
    volumes = asset["volumes"]
    if not any(s["id"] == "hand-r-thumb-mid" for s in volumes["50"]["body"]):
        right_thumb(volumes)
    body = {s["id"]: s for s in volumes["50"]["body"]}
    structure = {s["id"]: s for s in volumes["50"]["structure"]}
    bones = [b for b in asset["rig"]["bones"] if not any(b["id"].startswith(f + "-") for f in list(FINGERS.values()) + ["thumb"])]
    out, frames, chains = [], {}, {}
    for b in bones:
        out.append(b)
        if b["id"] not in ("hand-l", "hand-r"):
            continue
        side = b["id"][-1]
        for key, finger in [("thumb", "thumb")] + [("finger-" + n, f) for n, f in FINGERS.items()]:
            parent, pid = HAND, b["id"]
            for seg, part in (THUMB if finger == "thumb" else SEGMENTS).items():
                solid = body[f"hand-{side}-{key}{seg}"]
                frame = K.lateral(solid["p0"], solid["p1"], axis(side, finger))
                bid = f"{finger}-{part}-{side}"
                t = TITLE[part if part != "middle" else "middle-seg"]
                lo, hi = LIMITS[part]
                bone = {"id": bid, "parent": pid, "title": {"en": ("Left " if side == "l" else "Right ") + TITLE[finger][0] + ", " + t[0], "de": TITLE[finger][1] + (" links, " if side == "l" else " rechts, ") + t[1]},
                        **K.placement(parent, frame), "lengthM": round(math.dist(solid["p0"], solid["p1"]), 5), "joint": {"type": "revolute", "minDeg": lo, "maxDeg": hi, "twistMinDeg": -45, "twistMaxDeg": 45}}
                out.append(bone)
                for s in (solid, structure[f"bones-{side}-{key}{seg}"]):
                    s["p0"], s["p1"] = [K.r5(c) for c in frame.L(s["p0"])], [K.r5(c) for c in frame.L(s["p1"])]
                    s["bone"] = bid
                frames[bid] = frame
                chains.setdefault((side, finger), []).append((bid, parent, frame))
                parent, pid = frame, bid
    asset["rig"]["bones"] = out
    f100 = frames["index-proximal-r"]
    for lst, sid in (("body", "hand-r-index"), ("structure", "index-bones-r")):
        for s in volumes["100"][lst]:
            if s["id"] == sid and s["bone"] == "hand-r":
                s["p0"], s["p1"], s["bone"] = [K.r5(c) for c in f100.L(s["p0"])], [K.r5(c) for c in f100.L(s["p1"])], "index-proximal-r"
    ids = [b["id"] for b in out if b["id"] in frames]
    for pose in asset["poses"]:
        for bid in ids:
            pose["bones"][bid] = {"angleDeg": 0}
    parked = next(p for p in asset["poses"] if p["id"] == "cane-parked")
    for finger, tips in PARKED.items():
        posed, origin = HAND, None
        for (bid, parent, frame), tip in zip(chains[("r", finger)], tips):
            origin = frame.o if origin is None else origin
            free = carried(posed, relative(parent, frame), origin)
            target = K.lateral(origin, tip, free.w)
            p = K.placement(free, target)
            parked["bones"][bid] = {k: p[k] for k in ("angleDeg", "abductionDeg", "twistDeg") if k == "angleDeg" or abs(p[k]) > 1e-3}
            posed, origin = target, K.add(origin, target.u, next(b for b in out if b["id"] == bid)["lengthM"])
    note50 = {"en": "Hands at 1:50: each finger is three phalanges and the thumb a metacarpal and two phalanges, every one a solid on its own bone of the rig. The rest placement of these bones is the modelled hand: the right hand holds the long cane in the grasp of the constant-contact technique (index finger stretched along the grip, thumb on top, three fingers closed round it), the left hand hangs relaxed. In the pose with the cane parked upright the index finger and the thumb close round the grip.", "de": "Hände bei 1:50: jeder Finger besteht aus drei Gliedern, der Daumen aus Mittelhandknochen und zwei Gliedern, jedes ein Körper an seinem eigenen Knochen des Rigs. Die Ruhelage dieser Knochen ist die modellierte Hand: Die rechte Hand hält den Langstock im Griff der Dauerkontakttechnik (Zeigefinger gestreckt am Griff, Daumen obenauf, drei Finger um den Griff geschlossen), die linke hängt entspannt. In der Haltung mit senkrecht abgestelltem Stock schließen sich Zeigefinger und Daumen um den Griff."}
    note100 = {"en": "Right hand at 1:100: the index finger along the grip is one solid on the bone of its proximal phalanx, so that it closes round the grip when the cane is parked upright; the other fingers are one mass with the palm.", "de": "Rechte Hand bei 1:100: Der Zeigefinger am Griff ist ein Körper am Knochen seines Grundglieds, so dass er sich um den Griff schließt, wenn der Stock senkrecht abgestellt ist; die übrigen Finger bilden eine Masse mit der Handfläche."}
    for scale, note in (("50", note50), ("100", note100)):
        d = volumes[scale]["decisions"]
        if note not in d:
            d[:] = [x for x in d if not x["en"].startswith(note["en"][:14])] + [note]


if __name__ == "__main__":
    doc = K.load()
    if any(b["id"] == "index-proximal-r" for b in doc["asset"]["rig"]["bones"]):
        sys.exit("[DEBUG] the catalog asset already has finger bones: the finger solids are in their bone frames, nothing to do")
    build(doc["asset"])
    K.save(K.CANDIDATE, doc, like=K.CATALOG)
    print("[DEBUG] candidate", K.CANDIDATE)
    for b in doc["asset"]["rig"]["bones"]:
        if b["id"].endswith("-r") and b["parent"] != "pelvis" and ("index" in b["id"] or "thumb" in b["id"]):
            print("[DEBUG]", {k: v for k, v in b.items() if k not in ("title", "joint")})
    print("[DEBUG] parked", {k: v for k, v in next(p for p in doc["asset"]["poses"] if p["id"] == "cane-parked")["bones"].items() if "index" in k or "thumb" in k})
    if "--publish" in sys.argv:
        live = K.load()
        for k in ("rig", "poses", "volumes"):
            live["asset"][k] = doc["asset"][k]
        K.save(K.CATALOG, live, like=K.CATALOG)
        with open(K.SOURCE, encoding="utf8") as h:
            src = json.load(h)
        src["volumes"] = doc["asset"]["volumes"]
        K.save(K.SOURCE, src, like=K.SOURCE)
        print("[DEBUG] published")
