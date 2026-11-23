"""🚲 vehicles/city-bicycle: the wheel centres and rims of 1:100 and 1:200 modelled for the smallest feature of their scale instead of the 1:50 parts.

1:100 (15 mm): the 10 mm axle and its two 18 mm nuts are one axle stud of 15 mm over the outer faces of the nuts (rear 152 mm, front 132 mm); the nuts are no solids of their own.
1:200 (30 mm): hub, brake unit, axle and nuts are one turned hub body (the shell without the 3 mm spoke flanges, a 30 mm stud to the dropouts); the rim is a closed 30 mm band under the tyre instead of the 25 mm well section; the 4 mm nipples are left out; the bearing sleeve of the structure is 30 mm.
One-shot on `volumes/vehicles/city-bicycle.json`; publish with `volumes-insert.py`.
"""
from rework_fittings_lib import by_id, drop, load, save, say

ASSET = "vehicles/city-bicycle"
REACH = {"r": 0.076, "f": 0.066}
HUB = {
    "r": [[-0.076, 0.0], [-0.076, 0.015], [-0.034, 0.015], [-0.034, 0.03], [0.036, 0.03], [0.036, 0.045], [0.056, 0.045], [0.056, 0.015], [0.076, 0.015], [0.076, 0.0]],
    "f": [[-0.066, 0.0], [-0.066, 0.015], [-0.038, 0.015], [-0.038, 0.03], [-0.0305, 0.03], [-0.0305, 0.045], [0.002, 0.045], [0.01, 0.03], [0.026, 0.03], [0.026, 0.042], [0.042, 0.042], [0.042, 0.015], [0.066, 0.015], [0.066, 0.0]],
}
BAND = [[-0.015, 0.3175], [-0.015, 0.302], [-0.01, 0.297], [0.01, 0.297], [0.015, 0.302], [0.015, 0.3175], [-0.015, 0.3175]]
NIPPLES = ["nipple-%s-%d" % (w, n) for w in "rf" for n in (1, 2, 3, 4)]


def main():
    """▶️ Rewrites the wheel centres of both scales in body, thermal and structure."""
    document, indented = load(ASSET)
    fine, coarse = document["volumes"]["100"], document["volumes"]["200"]
    for w, reach in REACH.items():
        axle = by_id(fine["body"])["axle-" + w]
        if axle["profile"][0][1] != 0.005:
            raise SystemExit("already applied")
        axle["profile"] = [[0, 0.0075], [2 * reach, 0.0075]]
        axle["origin"] = [0, 0, -reach]
        hub = by_id(coarse["body"])["hub-" + w]
        hub["profile"] = HUB[w]
        by_id(coarse["body"])["rim-" + w]["profile"] = BAND
        bearing = by_id(coarse["structure"])["bearing-" + w]
        bearing["profile"] = [[0, 0.015], [bearing["profile"][1][0], 0.015]]
    nuts = ["nut-%s-%s" % (w, side) for w in "rf" for side in "lr"]
    drop(fine["body"], set(nuts))
    drop(fine["thermal"], {"th-" + n for n in nuts})
    drop(coarse["body"], set(nuts) | {"axle-r", "axle-f"} | set(NIPPLES))
    drop(coarse["thermal"], {"th-" + n for n in nuts + ["axle-r", "axle-f"] + NIPPLES})
    drop(coarse["structure"], {"st-axle-r", "st-axle-f"} | {"st-" + n for n in NIPPLES})
    say(fine, "Axle and axle nuts of each wheel are one 15 mm stud from nut face to nut face (rear 152 mm, front 132 mm): the 8 mm nuts are below the smallest feature of 1:100.",
        "Achse und Achsmuttern jedes Rades sind ein 15-mm-Bolzen von Mutterfläche zu Mutterfläche (hinten 152 mm, vorn 132 mm): die 8 mm starken Muttern liegen unter dem kleinsten Merkmal von 1:100.")
    say(coarse, "Wheel centres for the 30 mm feature: hub shell, brake unit, axle and nuts are one turned body with a 30 mm stud to the dropouts, the rim is a closed 30 mm band under the tyre, the spoke nipples are left out and the bearing sleeve of the structure is 30 mm; the spokes stay true rods.",
        "Radmitten für das 30-mm-Merkmal: Nabenhülse, Bremse, Achse und Muttern sind ein Drehkörper mit einem 30-mm-Bolzen bis zu den Ausfallenden, die Felge ist ein geschlossenes 30-mm-Band unter dem Reifen, die Speichennippel entfallen und die Lagerhülse der Struktur misst 30 mm; die Speichen bleiben wahre Stäbe.")
    save(ASSET, document, indented)
    print("applied")


if __name__ == "__main__":
    main()
