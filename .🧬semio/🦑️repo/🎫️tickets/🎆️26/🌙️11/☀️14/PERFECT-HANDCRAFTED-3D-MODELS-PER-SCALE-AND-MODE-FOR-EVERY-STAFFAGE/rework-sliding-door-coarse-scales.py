"""🚪 components/sliding-door: 1:100, 1:200 and 1:500 modelled for their smallest feature instead of the 1:50 parts list (38 body solids at every scale before).

1:100 (15 mm, 10 mm on the leaf): jamb seals 15 mm, track end caps 15 mm (set into the track ends), slot brushes 15 mm, lock nut and adjusting nut one 16 mm nut, handle rosettes 10 mm.
1:200 (30 mm, 20 mm on the leaf): jamb seals, end caps and slot brushes belong to jambs and track; the glazing beads belong to the leaf frame; the two floor seals are one 20 mm strip; each wheel pair is one through roller of 30 mm; hanger bolt with its nuts is one 20 mm rod into the clevis (the pin is inside the clevis); the handle is a 20 mm bar on 20 mm legs without rosettes.
1:500 (3D only, 75 mm, 50 mm on the leaf): as 1:200, but each trolley with its hanger is one 50 mm block from the track down to the leaf, and floor guide, floor seal and handles are left out.
One-shot on `volumes/components/sliding-door.json`; publish with `volumes-insert.py`.
"""
from rework_fittings_lib import by_id, load, save, say

ASSET = "components/sliding-door"
ROLLERS = ["trolley-%s-%s-%s" % (t, part, side) for t in "ab" for part in ("wheels", "axles") for side in ("ext", "int")]
BEADS = ["bead-stiles-ext", "bead-rails-ext", "bead-stiles-int", "bead-rails-int"]
TRIM = ["jamb-west-seal", "jamb-east-seal", "track-end-caps", "track-slot-brush"]
HANDLE = ["handle-%s-%s" % (part, side) for part in ("rosette", "legs", "grip") for side in ("ext", "int")]


def remove(volumes, names):
    """✂️ Removes body solids with everything in thermal and structure that refers to them."""
    missing = set(names) - set(by_id(volumes["body"]))
    if missing:
        raise SystemExit("not in the body (already applied?): " + ", ".join(sorted(missing)))
    volumes["body"][:] = [s for s in volumes["body"] if s["id"] not in names]
    for name in ("thermal", "structure"):
        volumes[name][:] = [s for s in volumes[name] if s.get("of") not in names]


def fine(volumes):
    """🔧 1:100."""
    body = by_id(volumes["body"])
    for name in ("jamb-west-seal", "jamb-east-seal"):
        body[name]["size"][0] = 0.015
    body["track-end-caps"].update({"from": -0.05, "to": -0.035})
    body["track-end-caps"]["repeat"]["translation"] = [3.085, 0.0, 0.0]
    body["track-slot-brush"]["size"][2] = 0.015
    body["track-slot-brush"]["transform"]["translation"][2] = -0.0085
    body["track-slot-brush"]["repeat"]["translation"] = [0.0, 0.0, 0.017]
    body["hanger-nut-adjust"]["from"] = 0.055
    for side, axis in (("ext", 1), ("int", -1)):
        body["handle-rosette-" + side]["profile"] = [[0.0, 0.015], [0.008, 0.015], [0.01, 0.013], [0.01, 0.0]]
    remove(volumes, ["hanger-nut-lock"])


def coarse(volumes):
    """🧱 1:200, and the base of 1:500."""
    body = by_id(volumes["body"])
    seal = body["bottom-seal"]
    seal["size"] = [0.02, 1.0, 0.048]
    seal["transform"]["translation"] = [2.08, 0.0, 0.0]
    del seal["repeat"]
    for t in "ab":
        roller = body["trolley-%s-wheels-ext" % t]
        roller["profile"] = [[0.0, 0.015], [0.074, 0.015]]
        roller["origin"][2] = -0.037
    body["hanger-rod"]["profile"] = [[0.0, 0.01], [0.09, 0.01]]
    for side, sign in (("ext", 1), ("int", -1)):
        legs, grip = body["handle-legs-" + side], body["handle-grip-" + side]
        legs["p0"][2] = sign * 0.005
        legs.update({"r0": 0.01, "r1": 0.01})
        grip.update({"r0": 0.01, "r1": 0.01})
    gone = TRIM + BEADS + [name for name in ROLLERS if not name.endswith("wheels-ext")] + ["hanger-nut-lock", "hanger-nut-adjust", "hanger-pin", "handle-rosette-ext", "handle-rosette-int"]
    remove(volumes, gone)


def coarsest(volumes):
    """🧊 1:500 on top of the 1:200 model."""
    coarse(volumes)
    body = by_id(volumes["body"])
    body["trolley-chassis"]["profile"] = [[-0.025, -0.022], [0.025, -0.022], [0.025, 0.1], [-0.025, 0.1]]
    remove(volumes, ["floor-guide", "bottom-seal", "trolley-a-wheels-ext", "trolley-b-wheels-ext", "hanger-rod", "hanger-clevis"] + [name for name in HANDLE if "rosette" not in name])


def main():
    """▶️ Rewrites the three coarse scales."""
    document, indented = load(ASSET)
    fine(document["volumes"]["100"])
    coarse(document["volumes"]["200"])
    coarsest(document["volumes"]["500"])
    say(document["volumes"]["100"], "Trim and hardware for the 15 mm feature: jamb seals, track end caps and slot brushes are 15 mm, lock nut and adjusting nut of each hanger are one 16 mm nut, the handle rosettes are 10 mm.",
        "Abschlüsse und Beschläge für das 15-mm-Merkmal: Zargendichtungen, Schienenendkappen und Schlitzbürsten messen 15 mm, Kontermutter und Stellmutter jedes Gehänges sind eine 16-mm-Mutter, die Griffrosetten messen 10 mm.")
    say(document["volumes"]["200"], "Authored for the 30 mm feature (20 mm on the leaf): seals, end caps and brushes belong to jambs and track, the glazing beads to the leaf frame; one 20 mm floor seal; each wheel pair is one 30 mm through roller; each hanger is a 20 mm rod into its clevis; the handles are 20 mm bars on legs without rosettes.",
        "Für das 30-mm-Merkmal modelliert (20 mm am Flügel): Dichtungen, Endkappen und Bürsten gehören zu Zargen und Schiene, die Glasleisten zum Flügelrahmen; eine 20-mm-Bodendichtung; jedes Rollenpaar ist eine durchgehende 30-mm-Rolle; jedes Gehänge ist eine 20-mm-Stange in ihrer Gabel; die Griffe sind 20-mm-Stangen auf Stützen ohne Rosetten.")
    say(document["volumes"]["500"], "3D only, for the 75 mm feature (50 mm on the leaf): jambs, track, leaf frame and glass; each trolley with its hanger is one 50 mm block from the track down to the leaf; floor guide, floor seal, beads and handles are left out.",
        "Nur 3D, für das 75-mm-Merkmal (50 mm am Flügel): Zargen, Schiene, Flügelrahmen und Glas; jeder Laufwagen mit seinem Gehänge ist ein 50-mm-Block von der Schiene bis zum Flügel; Bodenführung, Bodendichtung, Glasleisten und Griffe entfallen.")
    save(ASSET, document, indented)
    print("applied")


if __name__ == "__main__":
    main()
