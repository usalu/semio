"""🪟 components/tilt-window: 1:100, 1:200 and 1:500 modelled for their smallest feature instead of the 1:50 parts list.

1:100 (15 mm): each tilt hinge is its 16 mm knuckle alone (the two 8 mm leaves are let in flush and gone), the quadrant stays are 15 mm plates with 15 mm pins, the grip is a 15 mm bar on 15 mm posts.
1:200 (30 mm): stops 30 x 30 mm, hinge knuckles 30 mm, stays plain 30 x 30 mm bars without the slot (the 27 mm quadrant band is below the feature) whose pin is a 30 mm joint block of the structure only, stiles and rails as plain 70 x 90 mm sections that take the glazing beads in, the grip one 30 mm bar on the top rail.
1:500 (3D only): as 1:200, without stops (part of the reveal lining), without grip and without the pin joint; the stay member of the structure is the stay bar of the body.
One-shot on `volumes/components/tilt-window.json`; publish with `volumes-insert.py`.
"""
from rework_fittings_lib import by_id, drop, load, save, say

ASSET = "components/tilt-window"
BEADS = ["bead-bottom", "bead-top", "bead-left", "bead-right"]
STOPS = {"stop-left": ([0.03, 1.224, 0.03], [0.205, 0.612, 0.385]), "stop-right": ([0.03, 1.224, 0.03], [0.205, 0.612, -0.385]),
         "stop-sill": ([0.03, 0.03, 0.74], [0.205, 0.015, 0.0]), "stop-head": ([0.03, 0.03, 0.74], [0.205, 1.209, 0.0])}
SECTIONS = {"stile-left": [[-0.008, 0.318], [-0.008, 0.388], [-0.098, 0.388], [-0.098, 0.318]], "stile-right": [[-0.008, -0.388], [-0.008, -0.318], [-0.098, -0.318], [-0.098, -0.388]],
            "rail-bottom": [[0.004, -0.008], [0.004, -0.098], [0.084, -0.098], [0.084, -0.008]], "rail-top": [[1.204, -0.008], [1.204, -0.098], [1.134, -0.098], [1.134, -0.008]]}


def stay(body, thickness):
    """📐 Quadrant stay plates of the given thickness against the jambs, mirrored."""
    arc = body["stay-arc"]
    arc["from"] = round(0.4 - thickness, 4)
    arc["repeat"]["translation"] = [0, 0, round(-(0.8 - thickness), 4)]


def coarse(volumes, stops, grip):
    """🧱 The merged sash and hardware of 1:200 and 1:500."""
    body = by_id(volumes["body"])
    if "bead-top" not in body:
        raise SystemExit("already applied")
    for name, profile in SECTIONS.items():
        body[name]["profile"] = profile
    body["hinge-barrel"]["profile"] = [[0, 0.015], [0.06, 0.015]]
    stay(body, 0.03)
    body["stay-arc"]["profile"] = [[0.0086, 1.1412], [0.1759, 1.1412], [0.1759, 1.1712], [0.0086, 1.1712]]
    body["stay-arc"].pop("holes", None)
    pin = body["stay-pin"]
    joint = by_id(volumes["structure"])["j-stay-pin"]
    del joint["of"]
    joint.update({"kind": "box", "bone": "sash", "material": pin["material"], "role": "body", "tone": pin["tone"], "size": [0.03, 0.03, 0.03], "transform": {"translation": [1.15, -0.053, 0.385]}, "repeat": {"count": 2, "translation": [0, 0, -0.77]}})
    gone = BEADS + ["hinge-leaf-frame", "hinge-leaf-sash", "stay-pin", "grip-post-left", "grip-post-right"]
    if stops:
        for name, (size, at) in STOPS.items():
            body[name].update({"size": size, "transform": {"translation": at}})
    else:
        gone += list(STOPS)
    if grip:
        body["grip-bar"].update({"p0": [1.169, 0.007, -0.05], "p1": [1.169, 0.007, 0.05], "r0": 0.015, "r1": 0.015})
    else:
        gone.append("grip-bar")
    drop(volumes["body"], set(gone))
    drop(volumes["thermal"], {"th-" + name for name in BEADS + ([] if stops else list(STOPS))})


def main():
    """▶️ Rewrites the hardware of 1:100 and the merged lists of 1:200 and 1:500."""
    document, indented = load(ASSET)
    fine = document["volumes"]["100"]
    body = by_id(fine["body"])
    stay(body, 0.015)
    body["stay-pin"]["profile"] = [[0, 0.0075], [0.015, 0.0075]]
    body["stay-pin"]["origin"] = [1.15, -0.053, 0.385]
    body["stay-pin"]["repeat"]["translation"] = [0, 0, -0.785]
    for name in ("grip-post-left", "grip-post-right", "grip-bar"):
        body[name].update({"r0": 0.0075, "r1": 0.0075})
    drop(fine["body"], {"hinge-leaf-frame", "hinge-leaf-sash"})
    coarse(document["volumes"]["200"], True, True)
    coarse(document["volumes"]["500"], False, False)
    coarsest = document["volumes"]["500"]["structure"]
    member = by_id(coarsest)["m-stay-arc"]
    for key in [key for key in member if key not in ("id", "member", "part")]:
        del member[key]
    member["of"] = "stay-arc"
    drop(coarsest, {"j-stay-pin"})
    by_id(coarsest)["mass-sash"]["radii"] = [0.0375, 0.0375, 0.0375]
    say(fine, "Hardware for the 15 mm feature: each tilt hinge is its 16 mm knuckle on the sill (the 8 mm leaves are let in flush), the quadrant stays are 15 mm plates with 15 mm pins and the grip is a 15 mm bar on two posts.",
        "Beschläge für das 15-mm-Merkmal: jedes Kippband ist seine 16-mm-Rolle auf der Sohlbank (die 8 mm starken Lappen sind bündig eingelassen), die Scherenbögen sind 15-mm-Platten mit 15-mm-Zapfen und der Griff ist eine 15-mm-Stange auf zwei Stützen.")
    say(document["volumes"]["200"], "Authored for the 30 mm feature: stops of 30 x 30 mm, hinge knuckles of 30 mm, stays as plain 30 x 30 mm bars without slot, stiles and rails as plain 70 x 90 mm sections that take the glazing beads in, and one 30 mm grip bar on the top rail; the stay pin is a joint block of the structure only.",
        "Für das 30-mm-Merkmal modelliert: Anschläge 30 x 30 mm, Bandrollen 30 mm, Scheren als glatte Stäbe 30 x 30 mm ohne Schlitz, Flügelfriese als glatte Querschnitte 70 x 90 mm, die die Glasleisten einschließen, und eine 30-mm-Griffstange am oberen Fries; der Scherenzapfen ist nur ein Gelenkblock der Struktur.")
    say(document["volumes"]["500"], "3D only, a massing of the 1:200 parts: plain sash sections, 30 mm hinge knuckles and stay bars; the stops belong to the reveal lining and the grip is left out.",
        "Nur 3D, eine Massierung der Teile von 1:200: glatte Flügelquerschnitte, 30-mm-Bandrollen und Scherenstäbe; die Anschläge gehören zur Laibungszarge und der Griff entfällt.")
    save(ASSET, document, indented)
    print("applied")


if __name__ == "__main__":
    main()
