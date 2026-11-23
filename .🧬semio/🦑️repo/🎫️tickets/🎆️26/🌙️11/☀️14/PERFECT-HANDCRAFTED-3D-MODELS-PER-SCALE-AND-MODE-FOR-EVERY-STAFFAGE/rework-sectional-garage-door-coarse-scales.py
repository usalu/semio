"""🚪 components/sectional-garage-door: hardware of 1:100 and 1:200 and the door leaf of 1:500 modelled for the smallest feature of their scale.

1:100 (15 mm): the hinge pins are 15 mm (were 12). 1:200: the cable studs are 20 mm (were 15, below the 18 mm the panel drawing allows).
1:500 (limit 40 mm, the thinnest drawing of a panel): each section is one 40 mm sandwich body over the full width that takes its two end stiles in; bottom rail and floor seal are one 45 mm shoe; the top cap is part of the top section; each hinge (two or three leaves and a pin) is one 80 x 40 x 80 mm block across the joint that also carries the roller stem; bottom and top roller brackets are 40 mm blocks; the cable studs are left out.
Load path (measured, open before this pass): 1:50 one M8 bolt joins each sway brace to its rear hanger (2.5 mm apart before); 1:200 the end bearing plates are structure members (the shaft group was 61 mm from the posts).
One-shot on `volumes/components/sectional-garage-door.json`; publish with `volumes-insert.py`.
"""
from rework_fittings_lib import by_id, load, save, say

ASSET = "components/sectional-garage-door"


def fatter(solid, factor):
    """🎚️ Scales the radial coordinate of a turned profile."""
    solid["profile"] = [[point[0], round(point[1] * factor, 5)] for point in solid["profile"]]


def remove(volumes, names):
    """✂️ Removes body solids with everything in thermal and structure that refers to them."""
    volumes["body"][:] = [s for s in volumes["body"] if s["id"] not in names]
    for name in ("thermal", "structure"):
        volumes[name][:] = [s for s in volumes[name] if s.get("of") not in names]


def main():
    """▶️ Rewrites the named hardware and the 1:500 leaf."""
    document, indented = load(ASSET)
    v100, v200, v500 = (document["volumes"][s] for s in ("100", "200", "500"))
    pin = by_id(v100["body"])["hinge-pin-1"]
    if max(point[1] for point in pin["profile"]) != 0.006:
        raise SystemExit("already applied")
    for k in (1, 2, 3):
        fatter(by_id(v100["body"])["hinge-pin-%d" % k], 1.25)
    for side in "lr":
        fatter(by_id(v200["body"])["cable-stud-" + side], 4 / 3)
    body = by_id(v500["body"])
    for k in (1, 2, 3, 4):
        body["panel-%d" % k].update({"from": -1.21, "to": 1.21})
    body["bottom-rail"]["profile"] = [[0.0, 0.0185], [0.045, 0.0185], [0.045, 0.0615], [0.0, 0.0615]]
    body["bottom-bracket"]["profile"] = [[0.046, -0.02], [0.15, -0.02], [0.15, 0.02], [0.046, 0.02]]
    body["top-bracket"]["profile"] = [[0.37, -0.02], [0.478, -0.02], [0.478, 0.02], [0.37, 0.02]]
    gone = {"bottom-seal", "top-cap", "cable-stud-l", "cable-stud-r"} | {"stile-%d-%s" % (k, side) for k in (1, 2, 3, 4) for side in "lr"}
    for k in (1, 2, 3):
        block = body["hinge-leaf-up-%d" % k]
        block.update({"profile": [[-0.04, -0.02], [0.04, -0.02], [0.04, 0.02], [-0.04, 0.02]], "from": -1.18, "to": -1.1})
        gone |= {"hinge-leaf-low-a-%d" % k, "hinge-leaf-low-b-%d" % k, "hinge-pin-%d" % k}
        joint = by_id(v500["structure"])["j-hinge-pin-%d" % k]
        joint["of"] = "hinge-leaf-up-%d" % k
        v500["structure"][:] = [s for s in v500["structure"] if s["id"] != "m-hinge-leaf-up-%d" % k]
    keep = [s for s in v500["structure"] if s["id"].startswith("j-hinge-pin-")]
    remove(v500, gone)
    v500["structure"] += [s for s in keep if s not in v500["structure"]]
    v50 = document["volumes"]["50"]
    brace = by_id(v50["body"])["brace-l"]
    twin = next(s for s in v50["thermal"] if s.get("of") == "brace-l")
    for side, z in (("l", 1.21), ("r", -1.2475)):
        bolt = {"id": "brace-bolt-" + side, "kind": "revolve", "bone": "track", "material": brace["material"], "role": "detail", "tone": brace["tone"], "profile": [[0, 0.004], [0.0375, 0.004]], "axis": [0, 0, 1], "origin": [2.495, 2.59, z]}
        v50["body"].append(bolt)
        v50["thermal"].append({**{key: value for key, value in twin.items() if key not in ("id", "of")}, "id": "th-" + bolt["id"], "of": bolt["id"]})
        v50["structure"].append({"id": "j-" + bolt["id"], "of": bolt["id"], "member": "track", "part": "joint"})
    for side in "lr":
        v200["structure"].append({"id": "m-bearing-plate-" + side, "of": "bearing-plate-" + side, "member": "track", "part": "member"})
    say(v50, "Each sway brace is bolted to its rear hanger with one M8 bolt through both angle legs (the legs lay 2.5 mm apart without a fastener).", "Jede Windverstrebung ist mit einer M8-Schraube durch beide Winkelschenkel an ihren hinteren Abhänger geschraubt (die Schenkel lagen 2,5 mm auseinander, ohne Verbindungsmittel).")
    say(v100, "The hinge pins are 15 mm, the smallest feature of 1:100.", "Die Scharnierstifte messen 15 mm, das kleinste Merkmal von 1:100.")
    say(v200, "The cable studs on the bottom brackets are 20 mm; the end bearing plates are members of the structure, so the spring shaft has its load path into the wall angles.", "Die Seilbolzen an den Bodenkonsolen messen 20 mm; die Endlagerplatten sind Stäbe der Struktur, so hat die Federwelle ihren Lastpfad in die Wandwinkel.")
    say(v500, "The leaf for the 40 mm the panel drawing allows: each section is one sandwich body over the full width with its end stiles in it, bottom rail and floor seal are one 45 mm shoe, the top cap belongs to the top section, each hinge is one 80 x 40 x 80 mm block across the joint that carries the roller stem, the roller brackets are 40 mm blocks and the cable studs are left out.",
        "Das Torblatt für die 40 mm, die die Paneelzeichnung zulässt: jede Sektion ist ein Sandwichkörper über die volle Breite mit ihren Endkappen, Bodenschiene und Bodendichtung sind ein 45-mm-Schuh, die obere Abschlusskappe gehört zur obersten Sektion, jedes Scharnier ist ein Block 80 x 40 x 80 mm über der Fuge, der den Rollenbolzen trägt, die Rollenkonsolen sind 40-mm-Blöcke und die Seilbolzen entfallen.")
    save(ASSET, document, indented)
    print("applied")


if __name__ == "__main__":
    main()
