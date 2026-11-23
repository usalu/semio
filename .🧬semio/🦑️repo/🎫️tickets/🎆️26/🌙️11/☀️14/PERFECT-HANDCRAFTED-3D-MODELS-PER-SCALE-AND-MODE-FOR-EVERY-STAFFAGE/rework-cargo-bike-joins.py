"""🔩 vehicles/cargo-bike: the parts that hold the loose groups (measured by `volumes-connected`, never proven for this asset before).

1:50: the fork ends are 32 mm eyes that reach their blades (the blades ended 2 mm above 26 mm eyes; the blade tubes of the structure run 10 mm further into them); the two crates sit on the box floor and on each other (they hovered 2 and 3 mm); the structure no longer lists the sprocket as a wheel member (a 2 mm disc beside the hub, drivetrain, not load path).
1:100: the steerer cap reaches the head tube (4 mm short), the head lamp stands on a bracket from the steering drum and carries the front mudguard on a second bracket; the stand hinge of the structure is a 30 mm sleeve that meets the beam box.
1:200: a 30 mm crank axle joins both crank arms through the motor, a 40 x 50 x 110 mm bracket under the beam carries the stand legs.
One-shot on `volumes/vehicles/cargo-bike.json`, after `rework-cargo-bike-coarse-scales.py`; publish with `volumes-insert.py`.
"""
from rework_fittings_lib import by_id, load, save, say

ASSET = "vehicles/cargo-bike"


def like(volumes, new, pattern):
    """➕ Adds a body solid with material and tone of `pattern` and a thermal reference in the region of `pattern` when `pattern` is a thermal surface."""
    source = by_id(volumes["body"])[pattern]
    if new["id"] in by_id(volumes["body"]):
        raise SystemExit("already applied: " + new["id"])
    volumes["body"].append({**new, "material": source["material"], "role": "body", "tone": source["tone"]})
    twin = next((s for s in volumes["thermal"] if s.get("of") == pattern or pattern in s["id"]), None)
    if twin is not None:
        volumes["thermal"].append({"id": "th-" + new["id"], "of": new["id"], **{key: twin[key] for key in ("region", "layer") if key in twin}})


def sink(solid, drop):
    """⬇️ Lowers a solid along the bone normal."""
    transform = solid.setdefault("transform", {})
    at = transform.get("translation", [0, 0, 0])
    transform["translation"] = [at[0], round(at[1] - drop, 5), at[2]]


def main():
    """▶️ Adds the joins scale by scale."""
    document, indented = load(ASSET)
    v50, v100, v200 = (document["volumes"][s] for s in ("50", "100", "200"))
    for name in ("body", "thermal", "structure"):
        for solid in v50[name]:
            if "kind" in solid and solid.get("bone") == "crate":
                if "crate-lower" in solid["id"]:
                    sink(solid, 0.002)
                elif "crate-upper" in solid["id"]:
                    sink(solid, 0.005)
            if "kind" in solid and "fork-end" in solid["id"]:
                solid["profile"] = [[point[0], round(point[1] + 0.003, 5)] for point in solid["profile"]]
    for side in "lr":
        blade = by_id(v50["structure"])["fork-blade-" + side]
        blade["profile"] = [[0.285 if point[0] > 0 else 0, point[1]] for point in blade["profile"]]
    v50["structure"][:] = [s for s in v50["structure"] if s["id"] != "st-sprocket"]
    say(v50, "The fork ends are 32 mm eyes that the blades reach. The two crates stand on the box floor and on each other; the sprocket is drivetrain and no member of the wheel structure.",
        "Die Gabelenden sind 32-mm-Augen, die die Gabelscheiden erreichen. Die zwei Kisten stehen auf dem Kastenboden und aufeinander; das Ritzel gehört zum Antrieb und ist kein Stab der Radstruktur.")

    cap = by_id(v100["body"])["steerer-cap-f"]
    cap.update({"profile": [[0, 0.0165], [0.03, 0.0165]], "origin": [0.03, 0, 0]})
    like(v100, {"id": "lamp-bracket-f", "kind": "capsule", "bone": "fork", "p0": [0.19, 0.02, 0], "p1": [0.19, 0.06, 0], "r0": 0.0075, "r1": 0.0075}, "head-lamp")
    like(v100, {"id": "fender-bracket-f", "kind": "capsule", "bone": "fork", "p0": [0.191, 0.07, 0], "p1": [0.235, 0.06, 0], "r0": 0.0075, "r1": 0.0075}, "fender-f")
    hinge = by_id(v100["structure"])["stand-hinge"]
    hinge["profile"] = [[0, 0.015], [0.08, 0.015]]
    say(v100, "Joins: the steerer cap reaches the head tube, the head lamp stands on a bracket from the steering drum and carries the front mudguard on a second bracket; the stand hinge of the structure is a 30 mm sleeve at the beam box.",
        "Verbindungen: die Gabelschaftkappe erreicht das Steuerrohr, der Scheinwerfer steht auf einem Halter von der Lenktrommel und trägt das vordere Schutzblech über einen zweiten Halter; das Ständergelenk der Struktur ist eine 30-mm-Hülse am Trägerkasten.")

    like(v200, {"id": "crank-axle", "kind": "revolve", "bone": "crank", "profile": [[0, 0.015], [0.16, 0.015]], "axis": [0, 0, 1], "origin": [0, 0, -0.08]}, "crank-arm-f")
    like(v200, {"id": "stand-bracket", "kind": "box", "bone": "frame", "size": [0.04, 0.05, 0.11], "transform": {"translation": [1.55, 0.22, 0]}}, "beam")
    say(v200, "Joins: a 30 mm crank axle through the motor carries both crank arms, and a bracket block under the beam carries the two stand legs.",
        "Verbindungen: eine 30-mm-Tretlagerwelle durch den Motor trägt beide Kurbelarme, und ein Konsolenblock unter dem Träger trägt die zwei Ständerbeine.")
    save(ASSET, document, indented)
    print("applied")


if __name__ == "__main__":
    main()
