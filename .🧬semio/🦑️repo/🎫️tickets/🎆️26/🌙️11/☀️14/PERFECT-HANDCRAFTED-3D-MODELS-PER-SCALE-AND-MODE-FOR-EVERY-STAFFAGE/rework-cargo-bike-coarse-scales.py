"""🚲 vehicles/cargo-bike: 1:100, 1:200 and 1:500 modelled for their smallest feature instead of the 1:50 parts.

1:100 (15 mm): axle and nuts of each wheel are one 15 mm stud; the five carrier cross bars are 15 mm.
1:200 (30 mm): each hub is one turned drum with a 30 mm stud to the dropouts (axle and nuts in it); chainstays, their bridge, the carrier struts and cross bars are 30 mm; seatstays, carrier rails and legs are true rods (`slender`); the steering drum is 30 mm deep; the carrier tail plate is left out; bearing sleeves of the structure 30 mm.
1:500 (75 mm): hubs are plain drums from dropout to dropout (front 75 mm); the rims are true hoops (`slender`); beam, head tubes, down tube, seat tube, chainstays, riser, stem, bar clamp and steering drum are 75 mm masses as the seat post already was; chainstay bridge, carrier struts, cross bars and tail plate are left out; bearing sleeves and wheel mass markers of the structure 75 mm.
One-shot on `volumes/vehicles/cargo-bike.json`; publish with `volumes-insert.py`.
"""
import math

from rework_fittings_lib import by_id, load, save, say

ASSET = "vehicles/cargo-bike"
BARS = ["carrier-bar-%d" % n for n in (1, 2, 3, 4, 5)]
NUTS = ["nut-%s-%s" % (w, side) for w in "rf" for side in "lr"]
RODS = ["seatstay-l", "seatstay-r", "carrier-rail-l", "carrier-rail-r"] + ["carrier-leg-%s-%s" % (leg, side) for leg in "ab" for side in "lr"]


def remove(volumes, names):
    """✂️ Removes body solids with everything in thermal and structure that refers to them."""
    missing = set(names) - set(by_id(volumes["body"]))
    if missing:
        raise SystemExit("not in the body (already applied?): " + ", ".join(sorted(missing)))
    volumes["body"][:] = [s for s in volumes["body"] if s["id"] not in names]
    for name in ("thermal", "structure"):
        volumes[name][:] = [s for s in volumes[name] if s.get("of") not in names]


def length(solid):
    """📏 Length of a rod."""
    return math.dist(solid["p0"], solid["p1"]) if solid["kind"] == "capsule" else max(p[0] for p in solid["profile"]) - min(p[0] for p in solid["profile"])


def thick(solid, radius):
    """🎚️ Sets the radius of a rod (capsule, two-point revolve) or the half extents of a loft."""
    if solid["kind"] == "capsule":
        solid.update({"r0": radius, "r1": radius})
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section.update({"rx": max(section["rx"], radius), "ry": max(section["ry"], radius)})
    else:
        solid["profile"] = [[p[0], radius if p[1] > 0 else 0.0] for p in solid["profile"]]


def rods(body, feature):
    """🪡 True rods where 20:1 holds, feature-thick bars elsewhere."""
    for name in RODS:
        rod = body[name]
        radius = rod["r0"] if rod["kind"] == "capsule" else max(p[1] for p in rod["profile"])
        if length(rod) >= 40 * radius:
            rod["slender"] = True
        else:
            thick(rod, feature / 2)


def drum(reach, radius, stud=None):
    """🥁 Hub profile from dropout to dropout, with studs when the drum is shorter."""
    if stud is None:
        return [[-reach, 0.0], [-reach, radius], [reach, radius], [reach, 0.0]]
    half, r = stud
    return [[-reach, 0.0], [-reach, r], [-half, r], [-half, radius], [half, radius], [half, r], [reach, r], [reach, 0.0]]


def main():
    """▶️ Rewrites the three coarse scales."""
    document, indented = load(ASSET)
    v100, v200, v500 = (document["volumes"][s] for s in ("100", "200", "500"))
    body = by_id(v100["body"])
    if body["axle-r"]["profile"][0][1] != 0.005:
        raise SystemExit("already applied")
    for name in ("axle-r", "axle-f"):
        thick(body[name], 0.0075)
    for name in BARS:
        thick(body[name], 0.0075)
    remove(v100, NUTS)

    body = by_id(v200["body"])
    body["hub-r"]["profile"] = drum(0.087, 0.046, (0.0675, 0.015))
    body["hub-f"]["profile"] = drum(0.066, 0.031, (0.046, 0.015))
    for name in ["chainstay-l", "chainstay-r", "chainstay-bridge", "carrier-strut-l", "carrier-strut-r"] + BARS:
        thick(body[name], 0.015)
    rods(body, 0.03)
    body["drum-r"]["profile"] = [[round(p[0] * 1.2, 5), p[1]] for p in body["drum-r"]["profile"]]
    for name in ("bearing-r", "bearing-f"):
        thick(by_id(v200["structure"])[name], 0.015)
    remove(v200, NUTS + ["axle-r", "axle-f", "carrier-tail"])

    body = by_id(v500["body"])
    body["hub-r"]["profile"] = drum(0.087, 0.046)
    body["hub-f"]["profile"] = drum(0.066, 0.0375)
    for name in ("rim-r", "rim-f"):
        body[name]["slender"] = True
    for name in ("beam", "head-tube-f", "head-tube-r", "down-tube", "seat-tube", "chainstay-l", "chainstay-r", "riser", "stem"):
        thick(body[name], 0.0375)
    body["bar-clamp"].update({"profile": [[0, 0.0375], [0.075, 0.0375]], "origin": [0.82, -0.032, -0.0375]})
    body["drum-r"]["profile"] = [[round(p[0] * 3, 5), round(p[1] * 0.0375 / 0.034, 5)] for p in body["drum-r"]["profile"]]
    structure = by_id(v500["structure"])
    for name in ("bearing-r", "bearing-f"):
        thick(structure[name], 0.0375)
    for name in ("mass-wheel-r", "mass-wheel-f"):
        structure[name]["radii"] = [0.0375, 0.0375, 0.0375]
    remove(v500, NUTS + ["axle-r", "axle-f", "carrier-tail", "chainstay-bridge", "carrier-strut-l", "carrier-strut-r"] + BARS)

    say(v100, "Axle and nuts of each wheel are one 15 mm stud and the five carrier cross bars are 15 mm, the smallest feature of 1:100.",
        "Achse und Muttern jedes Rades sind ein 15-mm-Bolzen und die fünf Querstäbe des Gepäckträgers messen 15 mm, das kleinste Merkmal von 1:100.")
    say(v200, "Authored for the 30 mm feature: each hub is one drum with a 30 mm stud to the dropouts, chainstays, their bridge, the carrier struts and cross bars are 30 mm, seatstays, carrier rails and legs stay true rods, the steering drum is 30 mm deep and the carrier tail plate is left out.",
        "Für das 30-mm-Merkmal modelliert: jede Nabe ist eine Trommel mit einem 30-mm-Bolzen bis zu den Ausfallenden, Kettenstreben, ihr Steg, die Trägerstreben und Querstäbe messen 30 mm, Sitzstreben, Trägerholme und -beine bleiben wahre Stäbe, die Lenktrommel ist 30 mm tief und die Heckplatte des Trägers entfällt.")
    say(v500, "Authored for the 75 mm feature: hubs are plain drums from dropout to dropout, rims true hoops, and beam, head tubes, down tube, seat tube, chainstays, riser, stem, bar clamp and steering drum are 75 mm masses like the seat post; chainstay bridge, carrier struts, cross bars and tail plate are left out.",
        "Für das 75-mm-Merkmal modelliert: die Naben sind glatte Trommeln von Ausfallende zu Ausfallende, die Felgen wahre Reifen, und Träger, Steuerrohre, Unterrohr, Sitzrohr, Kettenstreben, Lenksäule, Vorbau, Lenkerklemme und Lenktrommel sind 75-mm-Massen wie die Sattelstütze; Kettenstrebensteg, Trägerstreben, Querstäbe und Heckplatte entfallen.")
    save(ASSET, document, indented)
    print("applied")


if __name__ == "__main__":
    main()
