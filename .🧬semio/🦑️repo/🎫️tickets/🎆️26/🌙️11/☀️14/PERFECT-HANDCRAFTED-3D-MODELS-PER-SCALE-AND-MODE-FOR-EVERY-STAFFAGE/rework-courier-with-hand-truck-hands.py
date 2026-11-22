"""🖐 Courier with hand truck: finger bones for both hands (rest placement = the grip around the bow handle, so every pose keeps its grip and a free hand opens by offsets), the phalanges of the skeleton on them, the rib cage and the handle of 1:500 at its feature size, and the decisions texts.

Usage: python3 rework-courier-with-hand-truck-hands.py            writes the candidate document 🗑️generated/rework-courier-with-hand-truck/cand/🔣️.json
       python3 rework-courier-with-hand-truck-hands.py --apply    re-reads and writes the volumes source and `rig` and `poses` of the catalog file
"""
import os

T = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(T, "rework-courier-with-hand-truck-girdles.py")).read().split("\ndef main")[0].split('"""', 2)[2])

FINGERS = {"index": {"en": "index finger", "de": "Zeigefinger"}, "middle": {"en": "middle finger", "de": "Mittelfinger"}, "ring": {"en": "ring finger", "de": "Ringfinger"}, "little": {"en": "little finger", "de": "kleiner Finger"}, "thumb": {"en": "thumb", "de": "Daumen"}}
PHALANX = {"proximal": {"en": "proximal phalanx", "de": "Grundglied"}, "middle": {"en": "middle phalanx", "de": "Mittelglied"}, "distal": {"en": "distal phalanx", "de": "Endglied"}}
SIDE = {"l": {"en": "Left", "de": "Links"}, "r": {"en": "Right", "de": "Rechts"}}
LIMITS = {"proximal": (-35, 70), "middle": (-45, 95), "distal": (-50, 45)}
RELAXED = {"proximal": 38, "middle": 42, "distal": 18}
THUMB = {"0": "proximal", "1": "distal"}


def chain(finger, s):
    if finger == "thumb":
        return [(f"glove-thumb-{k}-{s}", f"thumb-{part}-{s}", part) for k, part in THUMB.items()]
    return [(f"glove-{finger}-{part}-{s}", f"{finger}-{part}-{s}", part) for part in ("proximal", "middle", "distal")]


def fingers(asset, volumes):
    rig = asset["rig"]["bones"]
    by_id = {b["id"]: b for b in rig}
    if "index-proximal-l" in by_id:
        return
    body, st = volumes["50"]["body"], volumes["50"]["structure"]
    st[:] = [o for o in st if not re.match(r"^(phalanx-\d|thumb-bones)-", o["id"])]
    for s in "lr":
        hand = frames(rig)[f"hand-{s}"]
        at = rig.index(by_id[f"hand-{s}"]) + 1
        bones_of_hand = []
        for finger in FINGERS:
            parent = f"hand-{s}"
            for glove, bone, part in chain(finger, s):
                solid = find(body, glove)
                p0 = [hand[0][i] + apply(hand[1], solid["p0"])[i] for i in range(3)]
                p1 = [hand[0][i] + apply(hand[1], solid["p1"])[i] for i in range(3)]
                u = [p1[i] - p0[i] for i in range(3)]
                length = round(math.sqrt(sum(c * c for c in u)), 5)
                now = frames(rig)
                lo, hi = LIMITS[part]
                new = {"id": bone, "parent": parent, "title": {k: f"{SIDE[s][k]}, {FINGERS[finger][k]}, {PHALANX[part][k]}" for k in ("en", "de")}, **placed(now[parent], p0, u, [hand[1][i][2] for i in range(3)]), "lengthM": length, "joint": {"type": "revolute", "minDeg": lo, "maxDeg": hi}}
                new = {k: v for k, v in new.items() if k not in ("abductionDeg", "twistDeg") or abs(v) > 1e-9}
                rig.insert(at, new)
                at += 1
                frame = frames(rig)[bone]
                solid.update({"bone": bone, "p0": [0.0, 0.0, 0.0], "p1": [length, 0.0, 0.0]})
                bones_of_hand.append({"id": f"{bone[:-2]}-phalanx-{s}", "kind": "capsule", "bone": bone, **BONE, "p0": [-0.002, 0.0, 0.0], "p1": [round(length - 0.001, 5), 0.0, 0.0], "r0": 0.0045, "r1": 0.004, "member": f"hand-{s}", "part": "member"})
                parent = bone
        anchor = next(i for i, o in enumerate(st) if o["id"] == f"metacarpals-{s}") + 1
        st[anchor:anchor] = bones_of_hand
    for pose in asset["poses"]:
        offsets = pose.get("bones", {})
        if pose["id"] != "parked-upright":
            continue
        free = min("lr", key=lambda s: abs(offsets.get(f"forearm-{s}", {}).get("angleDeg", 0)))
        for finger in FINGERS:
            for _, bone, part in chain(finger, free):
                offsets[bone] = {"angleDeg": RELAXED[part] if finger != "thumb" else 15}


def coarse(volumes):
    st = volumes["500"]["structure"]
    if not find(st, "ribcage"):
        at = min(i for i, o in enumerate(st) if o["id"].startswith("costal-"))
        st[:] = [o for o in st if not o["id"].startswith("costal-")]
        rows = [((0.19, -0.012), 0.118, 0.088), ((0.27, -0.016), 0.140, 0.108), ((0.36, -0.012), 0.128, 0.098), ((0.435, 0.0), 0.085, 0.068), ((0.465, 0.006), 0.045, 0.040)]
        st.insert(at, {"id": "ribcage", "kind": "loft", "bone": "torso", **BONE, "sections": [{"at": [x, y, 0], "rx": rx, "ry": ry, "exponent": 2.2, "rotation": [0, -0.70711, 0, 0.70711]} for (x, y), rx, ry in rows], "capStart": True, "capEnd": True, "member": "torso", "part": "member"})
    find(volumes["500"]["body"], "handle").update({"r0": 0.0375, "r1": 0.0375})


NOTES = {
    "50": {"en": "Rework 2026-11-22: clavicles and hip bones are bones of the rig on their lateral rest placement, so the arms hang from the acromial end of the clavicle (0.34 m between the acromioclavicular joints, 0.37 m between the joint centres) and the thighs from the acetabulum of each hip bone (0.19 m apart); the skeleton has S-curved clavicles, scapulae with spine, acromion, neck and coracoid, hip bones with ala, auricular part, acetabulum, pubis and ischium around the obturator opening, a mandible of ramus and body plates hinged at the temporal bone, and every finger has its phalanges on its own bones, resting in the grip around the handle.", "de": "Überarbeitung 2026-11-22: Schlüsselbeine und Hüftbeine sind Knochen des Rigs in seitlicher Ruhelage, die Arme hängen am akromialen Ende des Schlüsselbeins (0,34 m zwischen den Schultereckgelenken, 0,37 m zwischen den Gelenkzentren) und die Oberschenkel an der Hüftpfanne jedes Hüftbeins (0,19 m Abstand); das Skelett hat S-förmig geschwungene Schlüsselbeine, Schulterblätter mit Gräte, Schulterhöhe, Hals und Rabenschnabelfortsatz, Hüftbeine mit Darmbeinschaufel, Gelenkfläche zum Kreuzbein, Hüftpfanne, Scham- und Sitzbein um das Hüftloch, einen Unterkiefer aus Ast- und Körperplatten am Schläfenbein, und jeder Finger hat seine Glieder auf eigenen Knochen, in Ruhe im Griff um den Bügel."},
    "100": {"en": "Rework 2026-11-22: clavicles, scapulae and hip bones sit on the lateral girdle bones of the rig; the scapula keeps blade, spine with acromion and neck, the hip bone keeps ala, acetabulum, pubis and ischium, each at 15 mm or more.", "de": "Überarbeitung 2026-11-22: Schlüsselbeine, Schulterblätter und Hüftbeine sitzen auf den seitlichen Gürtelknochen des Rigs; das Schulterblatt behält Blatt, Gräte mit Schulterhöhe und Hals, das Hüftbein Schaufel, Pfanne, Scham- und Sitzbein, jeweils ab 15 mm."},
    "200": {"en": "Rework 2026-11-22: the girdle is reduced to clavicle, scapula blade with acromion, and a hip bone of ala, auricular bar, ischium and pubis on the lateral girdle bones; humerus, forearm, femur and tibia keep their true shaft section as slender members.", "de": "Überarbeitung 2026-11-22: der Gürtel ist auf Schlüsselbein, Schulterblatt mit Schulterhöhe und ein Hüftbein aus Schaufel, Kreuzbeinsteg, Sitz- und Schambein auf den seitlichen Gürtelknochen reduziert; Oberarm, Unterarm, Oberschenkel und Schienbein behalten als schlanke Stäbe ihren wahren Schaftquerschnitt."},
    "500": {"en": "Rework 2026-11-22: the skeleton is skull, spine, one rib cage volume, sternum, the two clavicles and one bar per hip bone on the lateral girdle bones with the sacrum between them, and the long bones at their true shaft section as slender members; the hand truck keeps rails, handle and the wheel bracket between rail and axle.", "de": "Überarbeitung 2026-11-22: das Skelett besteht aus Schädel, Wirbelsäule, einem Brustkorbvolumen, Brustbein, den beiden Schlüsselbeinen und je einem Stab pro Hüftbein auf den seitlichen Gürtelknochen mit dem Kreuzbein dazwischen sowie den Röhrenknochen mit wahrem Schaftquerschnitt als schlanke Stäbe; die Sackkarre behält Holme, Griff und den Radträger zwischen Holm und Achse."},
}


def run():
    document = json.load(open(CATALOG))
    source = json.load(open(SOURCE))
    asset, volumes = document["asset"], source["volumes"]
    fingers(asset, volumes)
    coarse(volumes)
    for scale, note in NOTES.items():
        if note not in volumes[scale]["decisions"]:
            volumes[scale]["decisions"].append(note)
    if "--apply" in sys.argv:
        text = open(CATALOG).read()
        for key, following in (("poses", "clips"), ("rig", "anchors")):
            a, b = text.index(f'\n    "{key}": ') + len(key) + 9, text.index(f',\n    "{following}": ')
            text = text[:a] + json.dumps(asset[key], ensure_ascii=False, indent=2).replace("\n", "\n    ") + text[b:]
        open(CATALOG, "w").write(text)
        json.dump(source, open(SOURCE, "w"), ensure_ascii=False, separators=(",", ":"))
        print("applied")
        return
    document["asset"]["volumes"] = volumes
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(document, open(CANDIDATE, "w"), ensure_ascii=False, indent=1)
    for b in asset["rig"]["bones"]:
        if re.match(r"(index|thumb)-.*-l", b["id"]):
            print({k: v for k, v in b.items() if k not in ("title", "joint")})
    print(CANDIDATE)


run()
