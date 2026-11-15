"""Author 28: humans/child-on-kick-scooter, all four scales, on the kit of authors 07 and 22. python3 author-28-scooter.py [50 100 200 500]"""
import sys, copy
from author_07_lib import *
from author_22_lib import *
import author_07_lib as L

SLUG = "child-on-kick-scooter"
SC = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]
MEMBERS = ["pelvis", "torso", "neck", "head", "arm-l", "forearm-l", "hand-l", "thigh-l", "shank-l", "foot-l", "arm-r", "forearm-r", "hand-r", "thigh-r", "shank-r", "foot-r"]
JOINTS = {50: ["thigh-l", "thigh-r", "shank-l", "shank-r", "foot-l", "foot-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "hand-l", "hand-r", "neck", "head", "torso"],
          100: ["thigh-l", "thigh-r", "shank-l", "shank-r", "foot-l", "foot-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "neck", "torso"],
          200: ["thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r"],
          500: ["thigh-l", "thigh-r", "arm-l", "arm-r"]}
MASSES = ["pelvis", "torso", "head", "thigh-l", "thigh-r"]
FRAME_PARTS = ("deck", "stem", "handlebar", "fork", "head-tube", "strut", "strut-l", "strut-r", "bar-stem")
NOTES = {
 50: (d("1:50 body: the draft solids repaired against the 1:50 drawings and the real object. A child on a two-wheeled kick scooter: limbs, torso, neck and head are lofts with elliptic sections fitted laterally to the posed front and plan drawings, helmet, hair and backpack as separate shells, sneakers as lofts, the scooter as a deck plate with kick-up fender, steering column with head tube, fork, handlebar with grips and polyurethane wheels with rim, hub and three spokes. Zip below the 7.5 mm feature widened to 8.4 mm. Thermal: every region has solids, multi-layer clothing regions are nested shells (innermost first), breath and plume emitters at their anchors. Structure: bone members, joints at the rig joints, masses, every scooter part as a member, one support pad under each wheel contact.",
         "1:50-Körper: die Entwurfskörper wurden gegen die 1:50-Zeichnungen und das reale Objekt repariert. Ein Kind auf einem zweirädrigen Tretroller: Gliedmaßen, Rumpf, Hals und Kopf sind Lofts mit elliptischen Querschnitten, seitlich an Front- und Grundriss der Pose angepasst, Helm, Haar und Rucksack als eigene Schalen, Turnschuhe als Lofts, der Roller als Trittbrett mit hochgezogenem Schutzblech, Lenksäule mit Steuerrohr, Gabel, Lenker mit Griffen und Polyurethan-Räder mit Felge, Nabe und drei Speichen. Der Reißverschluss unter dem Merkmal von 7,5 mm wurde auf 8,4 mm verbreitert. Thermik: jede Region hat Körper, mehrschichtige Bekleidungsregionen sind verschachtelte Schalen (innerste zuerst), Atem- und Wärmefahne-Emitter an ihren Ankern. Tragwerk: Knochenglieder, Gelenke an den Rig-Gelenken, Massen, jedes Rollerteil als Glied, je Radkontakt eine Auflagerplatte.")),
 100: (d("1:100 body modelled anew from this scale's drawings: child as lofts with elliptic sections, helmet, hair and backpack shells, scooter as deck, kick-up fender, steering column, fork, handlebar with grips and two lens-shaped polyurethane wheels; rims, hubs, spokes, zips and thumbs lie below the 15 mm feature or are texture and are left out. Thermal: one skin solid per region and body solid, areas fitted to areaM2 (30 %). Structure: bone members, joints, masses, scooter members and a support pad under each wheel contact.",
         "Der Körper 1:100 ist aus den Zeichnungen dieses Maßstabs neu modelliert: Kind als Lofts mit elliptischen Querschnitten, Helm-, Haar- und Rucksackschalen, Roller aus Trittbrett, hochgezogenem Schutzblech, Lenksäule, Gabel, Lenker mit Griffen und zwei linsenförmigen Polyurethan-Rädern; Felgen, Naben, Speichen, Reißverschlüsse und Daumen liegen unter dem Merkmal von 15 mm oder sind Textur und entfallen. Thermik: eine Hautschale je Region und Körperteil, Flächen auf areaM2 abgestimmt (30 %). Tragwerk: Knochenglieder, Gelenke, Massen, Rollerglieder und je Radkontakt eine Auflagerplatte.")),
 200: (d("1:200 body modelled anew against this scale's drawings: merged forearm-and-hand and head masses, deck, column, fork and handlebar as single solids, wheels as discs; features below 30 mm are merged. Thermal skins fitted to areaM2 (30 %), structure with members, four joints, masses and the two support pads.",
         "Der Körper 1:200 ist neu gegen die Zeichnungen dieses Maßstabs modelliert: Unterarm und Hand sowie der Kopf sind zu Massen zusammengefasst, Trittbrett, Säule, Gabel und Lenker als einzelne Körper, Räder als Scheiben; Merkmale unter 30 mm sind verschmolzen. Thermische Hüllen auf areaM2 abgestimmt (30 %), Tragwerk mit Gliedern, vier Gelenken, Massen und den beiden Auflagerplatten.")),
 500: (d("1:500 body modelled anew as merged masses: head with helmet, torso with backpack, limbs as single lofts, scooter as deck, column and handlebar, two wheel discs. Everything below 75 mm is merged into its neighbour. Thermal skins fitted to areaM2 (30 %), structure keeps every load-bearing limb and scooter member with two joints per leg and arm.",
         "Der Körper 1:500 ist neu als zusammengefasste Massen modelliert: Kopf mit Helm, Rumpf mit Rucksack, Gliedmaßen als einzelne Lofts, Roller aus Trittbrett, Säule und Lenker, zwei Radscheiben. Alles unter 75 mm ist in den Nachbarn verschmolzen. Thermische Hüllen auf areaM2 abgestimmt (30 %), das Tragwerk behält jedes tragende Glied und Rollerteil mit zwei Gelenken je Bein und Arm.")),
}


FEATURE_FLOOR = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
CACHE = {}


def floor_solid(s, f):
    k = s["kind"]
    h = f * 0.5 * 1.03
    if k == "loft":
        for q in s["sections"]:
            q["rx"] = max(q["rx"], h)
            q["ry"] = max(q["ry"], h)
        us = [q["at"][0] for q in s["sections"]]
        if max(us) - min(us) < f * 1.06:
            k2 = f * 1.06 / max(max(us) - min(us), 1e-6)
            for q in s["sections"]:
                q["at"][0] = min(us) + (q["at"][0] - min(us)) * k2
    elif k == "capsule":
        s["r0"] = max(s["r0"], h)
        s["r1"] = max(s["r1"], h)
    elif k == "ellipsoid":
        s["radii"] = [max(r, h) for r in s["radii"]]
    elif k == "box":
        s["size"] = [max(r, f * 1.03) for r in s["size"]]
    elif k == "extrude":
        if s["to"] - s["from"] < f * 1.03:
            c = (s["to"] + s["from"]) / 2
            s["from"], s["to"] = c - f * 0.515, c + f * 0.515
        for a in (0, 1):
            vs = [p[a] for p in s["profile"]]
            lo, hi = min(vs), max(vs)
            if hi - lo < f * 1.03:
                c = (lo + hi) / 2
                k2 = f * 1.03 / max(hi - lo, 1e-6)
                for p in s["profile"]:
                    p[a] = c + (p[a] - c) * k2
    return s


def capsule_to_loft(s):
    p0, p1 = s["p0"], s["p1"]
    mid = [(a + b) / 2 for a, b in zip(p0, p1)]
    r = max(s["r0"], s["r1"])
    keep = {k: v for k, v in s.items() if k not in ("kind", "p0", "p1", "r0", "r1", "lateralScale")}
    return {**keep, "kind": "loft", "capStart": True, "capEnd": True, "sections": [{"at": list(p), "rx": r, "ry": r, "exponent": 2.0} for p in (p0, mid, p1)]}


def drawn_bones(B):
    return {s["bone"] for v in B.rep["geometry"].values() for s in v if s.get("role") == "body"}


def fill_missing(B, scale):
    have = {s["bone"] for s in B.body}
    for bone in sorted(drawn_bones(B) - have):
        if bone not in B.bones or not is_human_bone(bone):
            continue
        twin = bone[:-1] + ("r" if bone.endswith("l") else "l") if bone[-2:] in ("-l", "-r") else None
        src = [s for s in B.body if s["bone"] == twin] if twin else []
        if src:
            for s in src:
                m = mirror_w(s, s["id"].replace("-r", "-l").replace("-l", "-l") if False else f"{s['id']}-m", bone)
                B.add(m, B.reg.get(s["id"]))
            continue
        finer = {500: 200, 200: 100, 100: 50}[scale]
        FB = CACHE.get(finer) or CACHE.setdefault(finer, build(finer))
        for s in [x for x in FB.body if x["bone"] == bone]:
            c = copy.deepcopy(s)
            c["id"] = f"{s['id']}-{scale}"
            if c["kind"] == "loft" and len(c["sections"]) > 3:
                q = c["sections"]
                c["sections"] = [q[0], q[len(q) // 2], q[-1]]
            B.add(floor_solid(c, FEATURE_FLOOR[scale]), FB.reg.get(s["id"]))


def build(scale):
    B = Builder(SLUG, scale)
    if scale == 50:
        load_seed(B, drop={"zip"})
        B.add(capsule("zip", "torso", [0.06, -0.103, 0], [0.34, -0.052, 0], 0.0042, material="zip-jacket", role="detail", tone=0.5), "jacket")
        return B
    humanize(B, skip={"zip", "button-0", "button-1", "button-2", "thumb-l", "thumb-r", "fingers-l", "fingers-r", "eye-l", "eye-r", "ear", "mouth", "backpack-pocket"} if scale > 50 else ())
    for sid, s in B.raw.items():
        if is_human_bone(s["bone"]) or sid in B.reg:
            continue
        if s["bone"] == "frame" and s["role"] == "detail" and sid not in FRAME_PARTS:
            continue
        if sid.startswith(("rim", "hub", "spoke", "strut")):
            continue
        B.keep(sid)
    if scale == 100:
        B.get("fender")["profile"] = [[-0.07, 0.12], [-0.037, 0.14], [0.007, 0.161], [0.046, 0.167], [0.074, 0.165], [0.13, 0.153], [0.15, 0.125], [0.14, 0.134], [0.06, 0.14], [0.039, 0.138], [-0.02, 0.125]]
    if scale == 500:
        for sid in ("bar-stem", "handlebar"):
            if B.has(sid):
                B.drop(sid)
        B.add(capsule("handlebar", "frame", [0.5, 0.83, -0.175], [0.5, 0.83, 0.175], 0.0335, material="alu-stem", role="body", tone=0.5), ["grips", "stem"])
    fill_missing(B, scale)
    for s in B.body:
        if s["bone"] == "neck" and "face-skin" in B.reg.get(s["id"], []) and s["kind"] == "loft":
            rot = s["sections"][0]["rotation"]
            r = max(0.042, FEATURE_FLOOR[scale] * 0.52)
            s["sections"] = [{"at": [u, 0.001, 0.0], "rx": r, "ry": r, "exponent": 2.0, "rotation": rot} for u in (-0.015, 0.01, 0.04, 0.07)]
    if scale == 500:
        B.body = [capsule_to_loft(s) if s["kind"] == "capsule" and "repeat" not in s and "lateralScale" not in s and "transform" not in s else s for s in B.body]
    for s in B.body:
        floor_solid(s, FEATURE_FLOOR[scale] if s["bone"] != "frame" else min(FEATURE_FLOOR[scale], 0.066 if scale == 500 else FEATURE_FLOOR[scale]))
    return B


def post_snap(B, sc):
    for side in "lr":
        sid = f"shank-g-{side}"
        if B.has(sid):
            q = B.get(sid)["sections"]
            a, b = q[-2], q[-1]
            mk = lambda u, rx, ry: {**a, "at": [u, a["at"][1], a["at"][2]], "rx": rx, "ry": ry}
            q[-1:] = [mk(0.306, a["rx"], a["ry"]), mk(0.322, a["rx"] * 1.04, a["ry"] * 1.04), {**mk(0.331, a["rx"] * 0.97, 0.004), "ry": 0.004}]
    if sc == 50:
        h = B.get("handlebar")
        h["p0"][2], h["p1"][2] = -0.2075, 0.2075
        B.drop("grip")
        for sgn, side in ((-1, "r"), (1, "l")):
            B.add(capsule(f"grip-{side}", "frame", [0.6, 0.828, sgn * 0.12], [0.6, 0.828, sgn * 0.2075], 0.0195, material="rubber-grip", role="body", tone=0.12), "grips")


def cover_bones(B):
    covered = set()
    bodyby = {s["id"]: s for s in B.body}
    for s in B.structure:
        covered.add(s["bone"] if "bone" in s else bodyby[s["of"]]["bone"])
    out = []
    for bone in sorted(drawn_bones(B) - covered):
        if bone not in B.bones:
            continue
        src = next((s for s in B.body if s["bone"] == bone), None)
        if src:
            out.append({"id": f"member-{bone}", "of": src["id"], "member": bone, "part": "member"})
    return out


def frame_structure(B, sc):
    out = []
    for s in B.body:
        if s["bone"] == "frame" and s["id"].split("-")[0] in ("deck", "stem", "handlebar", "fork", "head", "strut", "bar"):
            out.append({"id": f"member-{s['id']}", "of": s["id"], "member": "frame", "part": "member"})
        if s["id"].startswith("tyre"):
            out.append({"id": f"member-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"})
    f = B.feature
    for aid, bone in (("wheel-rear-contact", "frame"), ("wheel-front-contact", "frame")):
        out.append(contact_pad(B, aid, "frame", max(f, 0.05), max(f, 0.03), material="alu-stem"))
    return out


if __name__ == "__main__":
    for sc in SC:
        B = build(sc)
        ids = [x["id"] for x in B.body if x["kind"] == "loft" and is_human_bone(x["bone"]) and not x["id"].startswith("hair") and x["bone"] != "neck" or x["id"] == "collar"]
        print(snap(B, ids, "grow=60 max=0.2" if sc != 50 else "").strip()[-300:])
        post_snap(B, sc)
        B.structure = human_structure(B, MEMBERS, JOINTS[sc], MASSES, extra=frame_structure(B, sc))
        B.structure += cover_bones(B)
        B.decisions = [NOTES[sc]]
        extra = [emitter_solid(B, em, anc, dia, reg, fl) for em, anc, dia, reg, fl in (("breath-nose", "nose", 0.0076, "face-skin", False), ("breath-mouth", "mouth", 0.0099, "face-skin", False), ("plume", "crown", 0.18, "jacket", True))]
        fails, dropped = finish_thermal(B, extra)
        print(sc, len(B.body), len(B.thermal), len(B.structure), "dropped", sorted(dropped), "area", fails)
