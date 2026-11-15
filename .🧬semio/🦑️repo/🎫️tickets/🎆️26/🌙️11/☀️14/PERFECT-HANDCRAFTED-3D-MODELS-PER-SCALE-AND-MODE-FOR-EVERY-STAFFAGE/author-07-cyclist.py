import sys
from author_07_lib import *

SLUG = "cyclist"
SC = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]
HUMAN_MEMBERS = ["pelvis", "torso", "neck", "head", "arm-l", "forearm-l", "hand-l", "thigh-l", "shank-l", "foot-l", "arm-r", "forearm-r", "hand-r", "thigh-r", "shank-r", "foot-r"]
JOINTS = {50: ["thigh-l", "thigh-r", "shank-l", "shank-r", "foot-l", "foot-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "hand-l", "hand-r", "neck", "head", "torso"],
          100: ["thigh-l", "thigh-r", "shank-l", "shank-r", "foot-l", "foot-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "neck", "torso"],
          200: ["thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r"],
          500: ["thigh-l", "thigh-r", "arm-l", "arm-r"]}
MASSES = ["pelvis", "torso", "head", "thigh-l", "thigh-r"]
BM = dict(material="steel", role="body", tone=0.2)


def tube(B, id, p0, p1, r, w0=0.0, w1=None, region="bike-metal", **kw):
    B.add(capsule(id, "frame", [p0[0], p0[1], w0], [p1[0], p1[1], w0 if w1 is None else w1], r, **{**BM, **kw}), region)


def build50():
    B = Builder(SLUG, 50)
    load_seed(B, drop={"rear-spoke", "front-spoke", "glove-thumb-l", "glove-thumb-r", "chainring"})
    for sd in "lr":
        B.keep(f"glove-thumb-{sd}", region="gloves")
    for sid in ("jacket-zip-1", "jacket-zip-2"):
        s = B.get(sid)
        s["r0"] = s["r1"] = 0.0042
    B.add(revolve("chainring", "crank-r", [[-0.004, 0.0], [-0.004, 0.081], [0.004, 0.081], [0.004, 0.0]], axis=(0, 0, 1), material="steel", role="body", tone=0.55, transform={"translation": [0, 0, -0.053]}), "bike-metal")
    return B


def finish(B, structure_extra, decisions):
    print(snap(B, ["sleeve-l", "sleeve-r", "forearm-l", "forearm-r", "thigh-l", "thigh-r", "shank-l", "shank-r", "jacket", "helmet", "head"]).strip())
    B.structure = human_structure(B, HUMAN_MEMBERS, JOINTS[B.scale], MASSES) + structure_extra
    B.decisions = decisions
    extra = []
    for em, anc, dia, reg, flat in (("breath-nose", "nose", 0.0095, "face-skin", False), ("breath-mouth", "mouth", 0.0124, "face-skin", False), ("plume", "crown", 0.2, "jacket", True)):
        extra.append(emitter_solid(B, em, anc, dia, reg, flat))
    fit_thermal(B, extra=extra)
    write_source(B.slug, B.scale, B.body_out(), B.thermal, B.structure, B.decisions)
    print(B.slug, B.scale, "body", len(B.body), "thermal", len(B.thermal), "structure", len(B.structure), "kfit", {k: round(v, 2) for k, v in B.kfit.items()})


if __name__ == "__main__":
    for sc in SC:
        if sc == 50:
            B = build50()
            ext = []
            for sid in ("tube-seat", "tube-top", "tube-down", "tube-head", "tube-chainstay-l", "tube-chainstay-r", "tube-seatstay-l", "tube-seatstay-r", "fork-l", "fork-r", "stem", "seatpost", "handlebar-l", "handlebar-r"):
                ext.append({"id": f"member-{sid}", "of": sid, "member": "frame", "part": "member"})
            ext += [{"id": "member-rear-rim", "of": "rear-rim", "member": "rear-wheel", "part": "member"}, {"id": "member-front-rim", "of": "front-rim", "member": "front-wheel", "part": "member"},
                    {"id": "joint-rear-hub", "of": "rear-hub", "member": "rear-wheel", "part": "joint"}, {"id": "joint-front-hub", "of": "front-hub", "member": "front-wheel", "part": "joint"},
                    {"id": "member-crank-r", "of": "crank-arm-r", "member": "crank-r", "part": "member"}, {"id": "member-crank-l", "of": "crank-arm-l", "member": "crank-l", "part": "member"}]
            finish(B, ext, [d("placeholder", "Platzhalter")])
