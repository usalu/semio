"""🦵 Charcoal grill, author pass 1, second cut: the load path closed to the ground, the 1:200 grate thinned for its feature, the 1:500 side handles left to the drawing's decision.

Usage: python3 rework-charcoal-grill-load-path.py   (edits T/volumes/objects/charcoal-grill.json in place, refuses to run twice)
Hand-chosen numbers: the lid hinge joint is a 15 mm pin on the rim line that reaches both rims; the front support pad is 12 mm high so the leg tube stands in it; each wheel is a member between its bearing and its support pad.
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/objects/charcoal-grill.json")
HINGE_R = {"50": 0.0075, "100": 0.0075}
PAD_H = 0.012
TEXT = {
    "50": {
        "en": "Load path closed: the lid stands on the bowl through a 15 mm hinge pin on the rim line that reaches both rims, the front leg tube stands 2.5 mm deep in its 12 mm support pad, and each wheel is a member between its axle bearing and its support pad.",
        "de": "Lastpfad geschlossen: Der Deckel steht über einen 15-mm-Gelenkbolzen auf der Randlinie, der beide Ränder erreicht, auf dem Kessel, das vordere Beinrohr steht 2,5 mm tief in seiner 12 mm hohen Auflagerplatte, und jedes Rad ist ein Bauteil zwischen Achslager und Auflagerplatte.",
    },
    "100": {
        "en": "Load path: each wheel is a member between its axle bearing and its support pad.",
        "de": "Lastpfad: Jedes Rad ist ein Bauteil zwischen Achslager und Auflagerplatte.",
    },
    "200": {
        "en": "Load path: each wheel is a member between its axle bearing and its support pad. Thermal: the hidden cooking grate keeps its ring and every second bar, because 19.5 mm pitch is below the 30 mm feature.",
        "de": "Lastpfad: Jedes Rad ist ein Bauteil zwischen Achslager und Auflagerplatte. Thermik: Der verdeckte Grillrost behält seinen Ring und jeden zweiten Stab, weil der Stababstand von 19,5 mm unter dem 30-mm-Merkmal liegt.",
    },
    "500": {
        "en": "Load path: each wheel is a member between its axle bearing and its support pad. Thermal: the hidden cooking grate keeps its ring and every second bar. The side handles (130 x 34 mm, 0.07 mm on paper) are left out as in the drawings of this scale.",
        "de": "Lastpfad: Jedes Rad ist ein Bauteil zwischen Achslager und Auflagerplatte. Thermik: Der verdeckte Grillrost behält Ring und jeden zweiten Stab. Die Seitengriffe (130 x 34 mm, 0,07 mm auf dem Papier) entfallen wie in den Zeichnungen dieses Maßstabs.",
    },
}

source = json.load(open(SOURCE, encoding="utf-8"))
for scale, v in source["volumes"].items():
    structure = v["structure"]
    if any(s["id"] == "member-wheel-left" for s in structure):
        raise SystemExit("already applied at 1:" + scale)
    body_ids = {s["id"] for s in v["body"]}
    at = next(i for i, s in enumerate(structure) if s["id"] == "member-axle") + 1
    structure[at:at] = [{"id": "member-wheel-" + side, "of": "wheel-" + side, "member": "bowl", "part": "member"} for side in ("left", "right") if "wheel-" + side in body_ids]
    for solid in structure:
        if solid["id"] == "joint-lid-hinge" and scale in HINGE_R and solid["kind"] == "capsule":
            r = HINGE_R[scale]
            solid["p0"], solid["p1"], solid["r0"], solid["r1"] = [0.004, 0.002, -0.02], [0.004, 0.002, 0.02], r, r
        if solid["id"] == "support-leg-front" and scale == "50":
            solid["size"][1] = PAD_H
            solid["transform"]["translation"][1] = PAD_H / 2
    if scale in ("200", "500"):
        bars = [s["id"] for s in v["thermal"] if s["id"].startswith("th-grate-bar-")]
        print("[DEBUG] thermal grate bars before", scale, len(bars))
        drop = {name for name in bars if int(name.rsplit("-", 1)[1]) % 2 == 0}
        v["thermal"] = [s for s in v["thermal"] if s["id"] not in drop]
    if scale == "500":
        drop = {"side-handle-left", "side-handle-right", "side-bracket-left-a", "side-bracket-right-a"}
        v["body"] = [s for s in v["body"] if s["id"] not in drop]
        for name in ("thermal", "structure"):
            v[name] = [s for s in v[name] if s.get("of") not in drop]
        for d in v["decisions"]:
            d["en"] = d["en"].replace("side handles and their blocks are 34 mm (the thinnest stroke drawn on the bowl is 33 mm), the pan hangs", "the pan hangs")
            d["de"] = d["de"].replace("Seitengriffe und ihre Blöcke sind 34 mm stark (der dünnste am Kessel gezeichnete Strich ist 33 mm), die Wanne hängt", "Die Wanne hängt")
    v["decisions"].append(TEXT[scale])
    print("[DEBUG]", scale, {name: len(v[name]) for name in ("body", "thermal", "structure")})
json.dump(source, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
