"""🌱 Round 3, furniture/three-seat-sofa: the root corrections outside `volumes`.

- Rig: the floor reference loses its length. The frame bone moves up into the plinth. The arm bones turn upright through the arms. The cushion bones sit on the deck, between the arms.
- Anchors: in the moved frames, with feet of 45 mm.
- Thermal: the new region `back-frame` (the upholstered outer back), its entry in every pose, and the areas from the hand sums of the real parts.
- Also: references, notes, structural and thermal assumptions, summary and the true-extent symbol.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_three_seat_sofa_kit", os.path.join(HERE, "r3-three-seat-sofa-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

REFERENCES = [
    {"id": "wayfair-sofa-dimensions", "citation": "Wayfair, 'Sofa dimensions: how to choose the right size sofa for your home' (standard three-seat sofas 71-86 in wide, seat depth 20-25 in, height 32-36 in), wayfair.ca guide T5610, read 2026-11-20."},
    {"id": "livingetc-sofa-dimensions", "citation": "Livingetc, 'Understanding sofa dimensions' (seat height 17-19 in, seat depth 21-24 in, arm heights of market sofas 25-30 in), livingetc.com/advice/sofa-dimensions, read 2026-11-20."},
]

NOTES = {
    "en": "An upholstered three-seat sofa, 2.00 x 0.90 x 0.85 m (front elevation 2.00 x 0.85 m, depth 0.90 m). Construction: a plinth of 18 mm birch plywood on beech base rails, 0.10 to 0.23 m, with a ply deck. Two box arms, 0.20 x 0.90 m to 0.62 m, built on beech posts and rails with an inner ply panel and padded with batting. A back frame between the arms (bottom rail, four uprights, top rail) with a vertical outer back, an inner face leaning 8 degrees and a rolled top at 0.85 m. Four turned beech feet, 100 mm high. Three loose seat cushions of 0.20 m HR foam (0.533 x 0.57 m, seat height 0.43-0.44 m, seat depth 0.57 m in front of the back cushions) with piped boxing, and three loose back cushions 0.15-0.17 m thick standing on the deck against the frame. Masses are the asset's authored estimates (foam 40 kg/m3). Cushions are rigid bodies on prismatic joints; sink depths follow the foam compression stress (ISO 3386-1) and are authored estimates. Occupants are loads, not drawn.",
    "de": "Ein gepolstertes Dreisitzer-Sofa, 2,00 x 0,90 x 0,85 m (Vorderansicht 2,00 x 0,85 m, Tiefe 0,90 m). Aufbau: ein Sockel aus 18-mm-Birkensperrholz auf Buchenschienen, 0,10 bis 0,23 m, mit einem Sperrholzdeck. Zwei Kastenarmlehnen, 0,20 x 0,90 m bis 0,62 m, auf Buchenpfosten und -holmen mit innerer Sperrholzplatte, mit Vlies gepolstert. Ein Rückenrahmen zwischen den Armlehnen (Unterholm, vier Stiele, Oberholm) mit senkrechter Rückseite, um 8 Grad geneigter Innenseite und gerollter Oberkante auf 0,85 m. Vier gedrechselte Buchenfüße, 100 mm hoch. Drei lose Sitzkissen aus 0,20 m HR-Schaum (0,533 x 0,57 m, Sitzhöhe 0,43-0,44 m, Sitztiefe 0,57 m vor den Rückenkissen) mit gekederter Box und drei lose Rückenkissen, 0,15-0,17 m dick, auf dem Deck an den Rahmen gelehnt. Die Massen sind die Schätzwerte des Objekts (Schaum 40 kg/m3). Kissen sind starre Körper auf Schiebegelenken; die Einsinktiefen folgen der Schaumdruckspannung (ISO 3386-1) und sind Schätzwerte. Personen sind Lasten, nicht gezeichnet.",
}

SUMMARY = {
    "en": "A 2.00 m upholstered three-seat sofa: box arms, a rolled back frame, three separately sinking seat cushions and three back cushions on a plywood plinth with turned beech feet; front elevation, section and plan symbol.",
    "de": "Ein 2,00 m breites gepolstertes Dreisitzer-Sofa: Kastenarmlehnen, ein Rückenrahmen mit gerollter Oberkante, drei einzeln einsinkende Sitzkissen und drei Rückenkissen auf einem Sperrholzsockel mit gedrechselten Buchenfüßen; Vorderansicht, Schnitt und Grundrisssymbol.",
}

ASSUMPTION = {
    "en": "The sofa is a rigid frame on four foot contacts (two feet under each arm, x = 0.12 m and 1.88 m, z = +-0.36 m): base rails through the arm frames, ply deck between them, back frame bolted to the arm back posts; each cushion is a rigid body on a prismatic joint that sinks into the frame, the foam deformation is hidden behind the plinth.",
    "de": "Das Sofa ist ein starrer Rahmen auf vier Fußkontakten (zwei Füße unter jeder Armlehne, x = 0,12 m und 1,88 m, z = +-0,36 m): Sockelschienen durch die Armrahmen, Sperrholzdeck dazwischen, Rückenrahmen an die hinteren Armpfosten geschraubt; jedes Kissen ist ein starrer Körper auf einem Schiebegelenk, der in den Rahmen einsinkt, die Schaumverformung verdeckt der Sockel.",
}

BACK_FRAME = {
    "id": "back-frame",
    "title": {"en": "Upholstered back frame", "de": "Gepolsterter Rückenrahmen"},
    "kind": "surface",
    "model": "passive",
    "material": "upholstery-fabric",
    "areaM2": 2.3,
    "characteristicLengthM": 0.62,
    "geometry": "vertical-plate",
    "skyViewFactor": 0.5,
    "layers": [
        {"id": "ply-back", "title": {"en": "Plywood back frame 18 mm", "de": "Sperrholz-Rückenrahmen 18 mm"}, "kind": "timber", "material": "birch-plywood", "thicknessM": 0.018, "resistanceM2KW": 0.105882, "heatCapacityJM2K": 19584, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "batting-back", "title": {"en": "Batting 0.02 m", "de": "Vlies 0,02 m"}, "kind": "insulation", "material": "polyester-batting", "thicknessM": 0.02, "resistanceM2KW": 0.5, "heatCapacityJM2K": 520, "source": "authored-estimate", "reference": "iso-10456-2007"},
        {"id": "fabric", "title": {"en": "Cover fabric 1 mm", "de": "Bezugsstoff 1 mm"}, "kind": "clothing", "material": "upholstery-fabric", "thicknessM": 0.001, "resistanceM2KW": 0.02, "heatCapacityJM2K": 520, "source": "authored-estimate", "reference": "iso-10456-2007"},
    ],
    "heatCapacityJM2K": 20624,
    "source": "authored-estimate",
    "reference": "iso-10456-2007",
}

AREAS = {"seat-cushions": 2.8, "back-cushions": 2.5, "arm-pads": None, "base-rail": None, "feet": None, "back-frame": None}


def rig(old):
    """🦴 The corrected rig."""
    m = K.masses()
    bones = []
    for bone in old["bones"]:
        bone = dict(bone)
        bid = bone["id"]
        if bid == "ground":
            bone["lengthM"] = 0
        elif bid == "frame":
            bone["y"] = K.FRAME_Y
            bone["title"] = {"en": "Timber frame: plinth, back frame and feet", "de": "Holzrahmen: Sockel, Rückenrahmen und Füße"}
            bone["mass"] = dict(bone["mass"], comX=1.0, comY=K.r6(m["frame"][2] - K.FRAME_Y))
        elif bid in ("arm-left", "arm-right"):
            ox, oy, _ = K.ORIGIN[bid]
            bone.update(x=ox, y=K.r6(oy - K.FRAME_Y), angleDeg=90, lengthM=K.r6(K.ARM_Y[1] - K.ARM_Y[0]))
            bone["mass"] = dict(bone["mass"], comX=K.r6(m["arm"][2] - K.ARM_Y[0]), comY=0)
        elif bid.startswith(("seat-", "back-")):
            ox, oy, _ = K.ORIGIN[bid]
            k = int(bid[-1]) - 1
            bone.update(x=ox, y=K.r6(oy - K.FRAME_Y), lengthM=K.r6(K.SEAT_X[k + 1] - K.SEAT_X[k]))
            cy = m["seat"][2] if bid.startswith("seat") else m["back"][2]
            bone["mass"] = dict(bone["mass"], comX=K.r6((K.SEAT_X[k + 1] - K.SEAT_X[k]) / 2), comY=K.r6(cy - K.DECK_Y))
        bones.append(bone)
    return dict(old, bones=bones)


def anchors(old):
    """⚓ Foot contacts on the floor in the moved frame bone (45 mm feet), sitting points at the crown of each seat cushion."""
    out = []
    for anchor in old:
        anchor = dict(anchor)
        if anchor.get("kind") == "contact":
            anchor["y"] = K.r6(-K.FRAME_Y)
            anchor["footprintLengthM"] = 0.045
            anchor["footprintWidthM"] = 0.045
        elif anchor["id"].startswith("seat-load"):
            anchor["x"] = K.r6((K.SEAT_X[1] - K.SEAT_X[0]) / 2)
            anchor["y"] = K.r6(K.SEAT_CROWN - K.DECK_Y)
            anchor["z"] = -0.2
        out.append(anchor)
    return out


def symbol(old):
    """🔣 The true-extent symbol redrawn to the construction (plinth, box arms, back frame, cushions, feet)."""
    sym = dict(old)
    g = lambda sid, role, z, filled, tone, geom: dict({"id": sid, "bone": "ground", "role": role, "z": z, "filled": filled, "tone": tone}, **geom)
    rect = lambda x0, y0, x1, y1: {"kind": "rect", "x": K.r6(x0), "y": K.r6(y0), "w": K.r6(x1 - x0), "h": K.r6(y1 - y0)}
    poly = lambda pts: {"kind": "poly", "points": [[K.r6(a), K.r6(b)] for a, b in pts], "closed": True}
    el, fr, pl = [], [], []
    for side, x in (("l", 0.0), ("r", 1.8)):
        el.append(g("arm-" + side, "body", 3, True, 0.85, rect(x, K.ARM_Y[0], x + K.ARM_W, K.ARM_Y[1])))
        pl.append(g("arm-" + side, "body", 3, True, 0.85, rect(x, K.ARM_Z[0], x + K.ARM_W, K.ARM_Z[1])))
    for side, x in (("l", K.FOOT_X[0]), ("r", K.FOOT_X[1])):
        el.append(g("foot-" + side, "body", 1, True, 0.85, poly([(x - 0.0225, 0), (x + 0.0225, 0), (x + 0.03, K.FOOT_H), (x - 0.03, K.FOOT_H)])))
    el.append(g("base", "body", 1, True, 0.85, rect(K.BASE_X[0], K.BASE_Y[0], K.BASE_X[1], K.BASE_Y[1])))
    el.append(g("back-frame", "body", 1, True, 0.85, rect(K.BASE_X[0], K.DECK_Y, K.BASE_X[1], K.FRAME_TOP)))
    for k in range(3):
        el.append(g(f"seat-{k + 1}", "body", 2.5, True, 0.85, rect(K.SEAT_X[k], K.DECK_Y, K.SEAT_X[k + 1], K.SEAT_CROWN)))
        el.append(g(f"back-{k + 1}", "body", 2, False, 0.5, rect(K.SEAT_X[k], K.SEAT_CROWN, K.SEAT_X[k + 1], 0.80)))
        pl.append(g(f"seat-{k + 1}", "body", 2, True, 0.85, rect(K.SEAT_X[k], K.SEAT_Z[0], K.SEAT_X[k + 1], K.SEAT_Z[1])))
        pl.append(g(f"back-{k + 1}", "body", 2, False, 0.5, rect(K.SEAT_X[k], 0.145, K.SEAT_X[k + 1], 0.37)))
    pl.append(g("back-frame", "body", 1, True, 0.85, rect(K.BASE_X[0], K.BACK_Z0, K.BASE_X[1], 0.45)))
    fr.append(g("arm", "body", 1, True, 0.85, rect(K.ARM_Z[0], K.ARM_Y[0], K.ARM_Z[1], K.ARM_Y[1])))
    for side, z in (("l", K.FOOT_Z), ("r", -K.FOOT_Z)):
        fr.append(g("foot-" + side, "body", 1, True, 0.85, poly([(z - 0.0225, 0), (z + 0.0225, 0), (z + 0.03, K.FOOT_H), (z - 0.03, K.FOOT_H)])))
    fr.append(g("base", "body", 2, True, 0.85, rect(K.BASE_Z[0], K.BASE_Y[0], K.BASE_Z[1], K.BASE_Y[1])))
    fr.append(g("seat", "body", 2.5, True, 0.85, rect(K.SEAT_Z[0], K.DECK_Y, K.SEAT_Z[1], K.SEAT_CROWN)))
    fr.append(g("back-frame", "body", 2.5, True, 0.85, poly([(K.BACK_Z0, K.DECK_Y), (0.45, K.DECK_Y), (0.45, K.FRAME_TOP), (0.37, K.FRAME_TOP), (0.37, 0.80)])))
    fr.append(g("back", "body", 3, False, 0.5, poly([(0.15, K.DECK_Y), (K.BACK_Z0, K.DECK_Y), (0.37, 0.80), (0.27, 0.80), (0.15, 0.50)])))
    sym["geometry"] = {"elevation": el, "front": fr, "plan": pl}
    sym["description"] = {
        "en": "The three-seat sofa in all three views: 2.00 x 0.90 x 0.85 m with two box arms, a back frame, three seat cushions and three back cushions on a plinth with four feet; the back stands on the left side.",
        "de": "Das Dreisitzer-Sofa in allen drei Ansichten: 2,00 x 0,90 x 0,85 m mit zwei Kastenarmlehnen, einem Rückenrahmen, drei Sitzkissen und drei Rückenkissen auf einem Sockel mit vier Füßen; die Rückenlehne steht auf der linken Seite.",
    }
    return sym


def apply(asset):
    """🌱 Writes the root corrections into the asset in memory."""
    asset["summary"] = SUMMARY
    asset["rig"] = rig(asset["rig"])
    asset["anchors"] = anchors(asset["anchors"])
    prov = asset["provenance"]
    known = {r["id"] for r in prov["references"]}
    prov["references"] = prov["references"] + [r for r in REFERENCES if r["id"] not in known]
    prov["notes"] = NOTES
    asset["structure"]["assumptions"][0] = ASSUMPTION
    regions = asset["thermal"]["regions"]
    if not any(r["id"] == "back-frame" for r in regions):
        at = next(i for i, r in enumerate(regions) if r["id"] == "base-rail")
        regions.insert(at, dict(BACK_FRAME))
    for r in regions:
        if r["id"] == "base-rail":
            r["title"] = {"en": "Upholstered plinth", "de": "Gepolsterter Sockel"}
        if AREAS.get(r["id"]) is not None:
            r["areaM2"] = AREAS[r["id"]]
    for pose in asset["poses"]:
        rs = pose["thermal"]["regions"]
        if "back-frame" not in rs:
            items = list(rs.items())
            at = [k for k, _ in items].index("base-rail")
            items.insert(at, ("back-frame", {"exposedFraction": 1, "sunlitFraction": 0}))
            pose["thermal"]["regions"] = dict(items)
    asset["symbols"] = [symbol(asset["symbols"][0])] + asset["symbols"][1:]
