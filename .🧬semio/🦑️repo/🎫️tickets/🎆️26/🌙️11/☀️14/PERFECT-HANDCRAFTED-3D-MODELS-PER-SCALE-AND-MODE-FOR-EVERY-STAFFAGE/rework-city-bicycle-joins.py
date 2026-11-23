"""🔩 vehicles/city-bicycle: the parts that hold the loose groups of every scale (measured by `volumes-connected`, never proven for this asset before).

1:50: the kickstand mounting plate under both chainstays (the bracket ended 2.5 mm below them), the cross bolt of the saddle clamp through both saddle rails (the clamp ended 4.3 mm beside them), the two bearing cones with locknuts that carry the front hub on its axle between the fork ends (the hub shell had no bearing).
1:100: the seatstay bridge with the tab that carries the rear mudguard, the mudguard boss under the fork crown, the pedal spindles as body parts (they were structure solids only).
1:200: the mudguard boss, the kickstand bracket as a 30 mm block under the chainstays.
1:500: the kickstand bracket as a 75 mm block, the crank axle in the body (it was thermal and structure only), and wheels for the scale: one hub drum from dropout to dropout instead of hub, axle and nuts, a rim band as wide as the tyre, no nipples.
Structure of every scale: the kickstand bracket is a member of the frame, so the stand has a load path.
One-shot on `volumes/vehicles/city-bicycle.json`, after `rework-city-bicycle-coarse-wheels.py`; publish with `volumes-insert.py`.
"""
from rework_fittings_lib import by_id, drop, load, save, say

ASSET = "vehicles/city-bicycle"
BRACKET = [[-0.23, 0.197], [-0.2, 0.197], [-0.185, 0.3], [-0.225, 0.3]]


def solid(ident, kind, bone, material, tone, **members):
    """🧱 A body solid."""
    return {"id": ident, "kind": kind, "bone": bone, "material": material, "role": "body", "tone": tone, **members}


def add(volumes, part, region, member=None, layer=None):
    """➕ Adds a body solid with its thermal reference and, when `member` names a bone, its structure member."""
    if part["id"] in by_id(volumes["body"]):
        raise SystemExit("already applied: " + part["id"])
    volumes["body"].append(part)
    thermal = {"id": "th-" + part["id"], "of": part["id"], "region": region}
    if layer:
        thermal["layer"] = layer
    volumes["thermal"].append(thermal)
    if member:
        volumes["structure"].append({"id": "st-" + part["id"], "of": part["id"], "member": member, "part": "member"})


def boss(radius):
    """🔘 The mudguard boss under the fork crown, along the steering axis."""
    return solid("fender-boss-f", "capsule", "steer", "paint", 0.3, p0=[0.258, 0, 0], p1=[0.288, 0, 0], r0=radius, r1=radius)


def main():
    """▶️ Adds the joining parts scale by scale."""
    document, indented = load(ASSET)
    v50, v100, v200, v500 = (document["volumes"][s] for s in ("50", "100", "200", "500"))

    add(v50, solid("stand-plate", "box", "frame", "paint", 0.3, size=[0.045, 0.008, 0.09], transform={"translation": [-0.205, 0.3, 0]}), "frame-tubes", "frame", "paint")
    add(v50, solid("saddle-clamp-bolt", "revolve", "frame", "chrome", 0.7, profile=[[0, 0.005], [0.09, 0.005]], axis=[0, 0, 1], origin=[-0.349, 0.9487, -0.045]), "saddle")
    add(v50, solid("cone-f-l", "revolve", "wheel-f", "chrome", 0.7, profile=[[0.036, 0], [0.036, 0.012], [0.042, 0.012], [0.042, 0.008], [0.046, 0.008], [0.046, 0]], axis=[0, 0, 1], origin=[0, 0, 0]), "hub-brakes")
    add(v50, solid("cone-f-r", "revolve", "wheel-f", "chrome", 0.7, profile=[[-0.046, 0], [-0.046, 0.008], [-0.038, 0.008], [-0.038, 0.012], [-0.032, 0.012], [-0.032, 0]], axis=[0, 0, 1], origin=[0, 0, 0]), "hub-brakes")
    v50["structure"].append({"id": "st-stand-bracket", "of": "stand-bracket", "member": "frame", "part": "member"})
    say(v50, "Joins: the kickstand has its mounting plate under both chainstays, the saddle clamp its cross bolt through both rails, and the front hub runs on two bearing cones with locknuts between the fork ends.",
        "Verbindungen: der Ständer hat seine Montageplatte unter beiden Kettenstreben, die Sattelklemme ihren Querbolzen durch beide Sattelstreben, und die Vorderradnabe läuft auf zwei Lagerkonen mit Kontermuttern zwischen den Gabelenden.")

    add(v100, solid("seatstay-bridge", "revolve", "frame", "paint", 0.3, profile=[[0, 0.0075], [0.06416, 0.0075]], axis=[0, 0, 1], origin=[-0.36513, 0.68763, -0.03208]), "frame-tubes", "frame")
    add(v100, solid("fender-tab-r", "capsule", "frame", "paint", 0.3, p0=[-0.36513, 0.68763, 0], p1=[-0.3889, 0.6499, 0], r0=0.0075, r1=0.0075), "mudguards")
    add(v100, boss(0.0075), "mudguards")
    for side in "nf":
        structure = by_id(v100["structure"])["pedal-spindle-" + side]
        part = {key: value for key, value in structure.items() if key not in ("member", "part")}
        for key in [key for key in structure if key not in ("id", "member", "part")]:
            del structure[key]
        structure["of"] = part["id"]
        part["role"] = "body"
        volumes_body = v100["body"]
        volumes_body.append(part)
        v100["thermal"].append({"id": "th-" + part["id"], "of": part["id"], "region": "drivetrain"})
    v100["structure"].append({"id": "st-stand-bracket", "of": "stand-bracket", "member": "frame", "part": "member"})
    say(v100, "Joins: a 15 mm seatstay bridge with its tab carries the rear mudguard, a boss under the fork crown the front one, the pedals sit on their spindles, and the kickstand bracket is a member of the frame.",
        "Verbindungen: ein 15-mm-Steg zwischen den Sitzstreben trägt mit seiner Lasche das hintere Schutzblech, ein Auge unter der Gabelkrone das vordere, die Pedale sitzen auf ihren Achsen, und die Ständerkonsole ist ein Stab des Rahmens.")

    add(v200, boss(0.015), "mudguards")
    add(v200, solid("stand-bracket", "extrude", "frame", "paint", 0.3, profile=BRACKET, plane="xy", **{"from": -0.036, "to": 0.036}), "frame-tubes", "frame")
    say(v200, "Joins: a 30 mm boss under the fork crown carries the front mudguard and a 30 mm bracket block under the chainstays carries the two kickstand legs.",
        "Verbindungen: ein 30-mm-Auge unter der Gabelkrone trägt das vordere Schutzblech und ein 30-mm-Konsolenblock unter den Kettenstreben die beiden Ständerbeine.")

    add(v500, solid("stand-bracket", "extrude", "frame", "paint", 0.3, profile=[[-0.2525, 0.197], [-0.1775, 0.197], [-0.1775, 0.3], [-0.2525, 0.3]], plane="xy", **{"from": -0.0375, "to": 0.0375}), "frame-tubes", "frame")
    axle = by_id(v500["structure"])["crank-axle"]
    part = {key: value for key, value in axle.items() if key not in ("member", "part")}
    part.update({"role": "body", "tone": 0.3})
    v500["body"].append(part)
    for holder in (axle, by_id(v500["thermal"])["th-crank-axle"]):
        for key in ("kind", "bone", "material", "profile", "axis", "origin", "role", "tone"):
            holder.pop(key, None)
        holder["of"] = "crank-axle"
    nuts = ["nut-%s-%s" % (w, side) for w in "rf" for side in "lr"]
    nipples = ["nipple-%s-%d" % (w, n) for w in "rf" for n in (1, 2, 3, 4)]
    drop(v500["body"], set(nuts) | {"axle-r", "axle-f"} | set(nipples))
    drop(v500["thermal"], {"th-" + n for n in nuts + ["axle-r", "axle-f"] + nipples})
    drop(v500["structure"], {"st-axle-r", "st-axle-f"} | {"st-" + n for n in nipples})
    for w, reach in (("r", 0.076), ("f", 0.066)):
        by_id(v500["body"])["hub-" + w]["profile"] = [[-reach, 0.0], [-reach, 0.045], [reach, 0.045], [reach, 0.0]]
        by_id(v500["body"])["rim-" + w]["profile"] = [[-0.0185, 0.3175], [-0.0185, 0.302], [-0.0125, 0.297], [0.0125, 0.297], [0.0185, 0.302], [0.0185, 0.3175], [-0.0185, 0.3175]]
    say(v500, "Wheels and joins for the 75 mm feature: each hub is one 90 mm drum from dropout to dropout that takes in brake, axle and nuts, the rim is a band as wide as the tyre, the nipples are left out; the crank axle is a body part, and one 75 mm bracket block under the chainstays carries the kickstand legs.",
        "Räder und Verbindungen für das 75-mm-Merkmal: jede Nabe ist eine 90-mm-Trommel von Ausfallende zu Ausfallende, die Bremse, Achse und Muttern aufnimmt, die Felge ist ein Band in Reifenbreite, die Nippel entfallen; die Tretlagerwelle ist ein Körperteil, und ein 75-mm-Konsolenblock unter den Kettenstreben trägt die Ständerbeine.")
    save(ASSET, document, indented)
    print("applied")


if __name__ == "__main__":
    main()
