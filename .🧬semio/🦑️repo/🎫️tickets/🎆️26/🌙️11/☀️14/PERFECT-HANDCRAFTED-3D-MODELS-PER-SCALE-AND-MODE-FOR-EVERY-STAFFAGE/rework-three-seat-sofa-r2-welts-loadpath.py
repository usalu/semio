"""🧵 Second round, pass 1 of `furniture/three-seat-sofa`: welts on the arm panels and knife-edge seams on the back cushions (1:50 body), seat foam as a fixed spring block under a sinking slab (structure, every scale), raked uprights and ply face at 1:100.

Usage: python3 rework-three-seat-sofa-r2-welts-loadpath.py
Every number is chosen by hand against the reference table of `rework/assets/furniture--three-seat-sofa.md`; the script only writes them into `volumes/furniture/three-seat-sofa.json`.
"""
import json

PATH = "volumes/furniture/three-seat-sofa.json"
FABRIC = {"material": "upholstery-fabric", "role": "detail", "tone": 0.35, "texture": True}
WELT_OUTER = [(0.0005, 0.064), (0.0025, 0.0795), (0.007, 0.0885), (0.015, 0.0918), (0.46, 0.0918), (0.474, 0.0895), (0.486, 0.0835), (0.494, 0.071), (0.4985, 0.06), (0.5005, 0.045)]
WELT_INNER = [(0.0085, 0.058), (0.011, 0.074), (0.016, 0.08), (0.022, 0.0818), (0.458, 0.0818), (0.472, 0.0795), (0.482, 0.0735), (0.4885, 0.064), (0.4905, 0.052), (0.491, 0.04)]
SEAM_PROUD, SEAM_HALF = 0.003, 0.004
SEATS = {"seat-1": 0.2, "seat-2": 0.7333, "seat-3": 1.2667}
DECK_TOP = {"50": 0.063, "100": 0.063, "200": 0.063, "500": 0.065}
SLAB_FROM, SLAB_THICK, FOAM_DEPTH, FOAM_Z = 0.095, 0.09, 0.45, -0.145
UPRIGHTS = [(0.2, 0.225), (0.7208, 0.7458), (1.2542, 1.2792), (1.775, 1.8)]
UPRIGHT = [[0.065, 0.325], [0.065, 0.44], [0.595, 0.44], [0.595, 0.39]]
FACE_100 = [[0.065, 0.31], [0.065, 0.325], [0.595, 0.39], [0.595, 0.375]]


def ring(half):
    return [[x, y] for x, y in half] + [[x, -y] for x, y in reversed(half)]


def welt(side, bone):
    return {"id": f"welt-{side}", "kind": "extrude", "bone": bone, **FABRIC, "profile": ring(WELT_OUTER), "holes": [list(reversed(ring(WELT_INNER)))], "plane": "xy", "from": 0.432, "to": 0.44, "repeat": {"count": 2, "translation": [0.0, 0.0, -0.872]}}


def seam(index, cushion):
    stations = [(s["at"][1], s["at"][2], s["rx"]) for s in cushion["sections"]]
    x = cushion["sections"][0]["at"][0]
    stations[0] = (0.004, stations[0][1], stations[0][2])
    top = stations[-1]
    stations.append((top[0] + 0.0035, top[1] + 0.0035, top[2] - 0.006))
    sections = [{"at": [x, round(y, 5), round(z, 5)], "rx": round(rx + SEAM_PROUD, 5), "ry": SEAM_HALF, "exponent": 5, "rotation": [0.707107, 0.0, 0.0, 0.707107]} for y, z, rx in stations]
    return {"id": f"seam-back-{index}", "kind": "loft", "bone": f"back-{index}", **FABRIC, "sections": sections, "capStart": True, "capEnd": True}


def after(items, anchor, new):
    at = next(i for i, e in enumerate(items) if e["id"] == anchor)
    items[at + 1:at + 1] = new


def loadpath(scale, structure):
    for bone, x0 in SEATS.items():
        slab = next(e for e in structure if e["id"] == f"member-{bone}")
        width, cx = slab["size"][0], slab["transform"]["translation"][0]
        look = {k: slab[k] for k in ("material", "role", "tone") if k in slab}
        slab["size"] = [width, SLAB_THICK, FOAM_DEPTH]
        slab["transform"] = {"translation": [cx, SLAB_FROM + SLAB_THICK / 2, FOAM_Z]}
        top = 0.065 + SLAB_FROM
        base = {"id": f"member-{bone}-spring", "kind": "box", "bone": "frame", **look, "size": [width, round(top - DECK_TOP[scale], 4), FOAM_DEPTH], "transform": {"translation": [round(x0 + cx, 5), round((top + DECK_TOP[scale]) / 2, 5), FOAM_Z]}, "member": bone, "part": "member"}
        after(structure, slab["id"], [base])
        pad = next((e for e in structure if e["id"] == f"joint-{bone}"), None)
        if pad:
            pad["bone"] = "frame"
            pad["transform"] = {"translation": [round(x0 + cx, 5), DECK_TOP[scale], FOAM_Z]}


def main():
    with open(PATH, encoding="utf-8") as handle:
        source = json.load(handle)
    volumes = source["volumes"]
    body = volumes["50"]["body"]
    body[:] = [e for e in body if not e["id"].startswith(("welt-", "seam-back-"))]
    after(body, "arm-l", [welt("arm-l", "arm-left")])
    after(body, "arm-r", [welt("arm-r", "arm-right")])
    for index in (1, 2, 3):
        after(body, f"back-{index}", [seam(index, next(e for e in body if e["id"] == f"back-{index}"))])
    for scale in ("50", "100", "200", "500"):
        structure = volumes[scale]["structure"]
        if not any(e["id"].endswith("-spring") for e in structure):
            loadpath(scale, structure)
    s100 = volumes["100"]["structure"]
    look = {k: v for k, v in next(e for e in volumes["50"]["structure"] if e["id"] == "member-upright-1").items() if k in ("material", "role", "tone")}
    first = next(i for i, e in enumerate(s100) if e["id"] == "member-upright-1")
    s100[:] = [e for e in s100 if not e["id"].startswith("member-upright-")]
    s100[first:first] = [{"id": f"member-upright-{n + 1}", "kind": "extrude", "bone": "frame", **look, "member": "frame", "part": "member", "profile": UPRIGHT, "plane": "yz", "from": a, "to": b} for n, (a, b) in enumerate(UPRIGHTS)]
    face = next(e for e in s100 if e["id"] == "member-back-face")
    keep = {k: face[k] for k in ("id", "bone", "material", "role", "tone", "member", "part") if k in face}
    face.clear()
    face.update({**keep, "kind": "extrude", "profile": FACE_100, "plane": "yz", "from": 0.2, "to": 1.8})
    with open(PATH, "w", encoding="utf-8") as handle:
        json.dump(source, handle, ensure_ascii=False, indent=1)
        handle.write("\n")


main()
