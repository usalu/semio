"""🛺 Round 3 root fixes of vehicles/cargo-bike outside `volumes`: the rig becomes the real long-john (2.00 m wheelbase, 70 degree front head angle with 35 mm offset, the handlebar column on its own 80 degree steering axis, bottom bracket 0.47 m ahead of the rear axle, the box on the beam, the battery on the down tube, the two-leg stand under the box that lifts the front wheel), with anchors, poses, clip keys, texts, dimensions, supports, the thermal areas the honest model shows and the three drawings of every authored scale with their decisions.

Usage: python3 r3-cargo-bike-roots.py [--no-2d] [--apply]
Writes the candidate document to 🗑️generated/r3-cargo-bike/🔣️.json (with the volumes of the source); `--apply` re-reads the catalog file and writes it in the file's own indent-1 style (the `volumes` member keeps its text and is spliced by volumes-insert.py)."""
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/cargo-bike/🔣️.json"
OUT = os.path.join(T, "🗑️generated", "r3-cargo-bike")


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


pt = load("r3-cargo-bike-parts")
kit = pt.kit


def r4(x):
    return round(x, 4) + 0.0


def park_angle():
    """🅿️ The root angle at which the rear tyre patch (root) and the stand feet are level: the frame turns about the patch until the feet touch."""
    a = math.radians(kit.STAND_REST_DEG + kit.STAND_DOWN_DELTA)
    fx = kit.P[0] + kit.STAND_LENGTH * math.cos(a)
    fy = kit.P[1] + kit.STAND_LENGTH * math.sin(a)
    return round(math.degrees(math.atan2(-fy, fx)), 3)


def fork_local(x, y):
    (ox, oy), deg = kit.BONES["fork"]
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    dx, dy = x - ox, y - oy
    return r4(dx * c + dy * s), r4(-dx * s + dy * c)


def crate_ground(theta):
    """📦 The crate-x / crate-y offsets that put the crate stack on the ground 1.3 m behind the rear axle while the frame stands at theta."""
    dx = -2.48
    x = kit.CRATE_O[0] + dx
    y = -x * math.tan(math.radians(theta))
    return dx, r4(y - kit.CRATE_O[1])


def en_de(en, de):
    return {"en": en, "de": de}


BONES = {
    "frame": dict(x=0.0, y=0.0, angleDeg=0.0, lengthM=2.0, title=en_de("Frame with rear triangle, main beam and gooseneck", "Rahmen mit Hinterbau, Hauptträger und Schwanenhals")),
    "fork": dict(x=r4(kit.FORK_O[0]), y=r4(kit.FORK_O[1]), angleDeg=-kit.HA_F, lengthM=kit.FORK_T, title=en_de("Front steering head, fork and cable drum", "Vorderer Steuerkopf, Gabel und Seiltrommel")),
    "bar": dict(x=kit.BAR_O[0], y=kit.BAR_O[1], angleDeg=180.0 - kit.HA_R, lengthM=kit.BAR_L, title=en_de("Handlebar column, stem and handlebar", "Lenksäule, Vorbau und Lenker")),
    "box": dict(x=kit.BOX_O[0], y=kit.BOX_O[1], angleDeg=0.0, lengthM=0.84),
    "battery": dict(x=r4(kit.BATT_O[0]), y=r4(kit.BATT_O[1]), angleDeg=round(kit.DOWN_DEG, 3), lengthM=0.34),
    "motor": dict(x=kit.B[0], y=kit.B[1]),
    "crank": dict(x=kit.B[0], y=kit.B[1]),
    "stand": dict(x=kit.P[0], y=kit.P[1], angleDeg=kit.STAND_REST_DEG, lengthM=kit.STAND_LENGTH, title=en_de("Two-leg stand under the box", "Zweibeinständer unter der Box"), joint={"type": "revolute", "minDeg": 0, "maxDeg": 105}),
    "wheel-f": dict(x=kit.FORK_T, y=kit.OFFSET_F, angleDeg=kit.HA_F),
    "crate-x": dict(x=kit.CRATE_O[0], y=kit.CRATE_O[1], lengthM=pt.CRATE[0]),
    "crate-y": dict(lengthM=2 * pt.CRATE[2]),
    "crate": dict(lengthM=pt.CRATE[0]),
}

MASS_COM = {"fork": (0.30, 0.02), "bar": (0.70, 0.02), "box": (0.38, 0.13), "battery": (0.16, 0.0665), "motor": (0.02, -0.01), "stand": (0.12, 0.0)}

AREAS = {"frame": 1.35, "box-wood": 3.61, "battery-pack": 0.125, "tyres": 0.51}

SUMMARY = en_de(
    "A riderless e-assist long-john cargo bike of the Bakfiets.nl Cargobike Long class: 26 inch rear and 20 inch front wheel on a 2.00 m wheelbase, a low aluminium main beam that bends up around the front wheel, a plywood cargo box with a child bench between the handlebar column and the front wheel, steering by cables, mid-drive motor, frame battery, crank, a two-leg stand under the box and two stacked crates; parked on the stand the front wheel is lifted and the load sits just behind the stand feet.",
    "Ein fahrerloses Long-John-Lastenrad mit Elektrounterstützung der Klasse Bakfiets.nl Cargobike Long: 26-Zoll-Hinterrad und 20-Zoll-Vorderrad auf 2,00 m Radstand, ein tiefer Aluminium-Hauptträger, der sich um das Vorderrad hochbiegt, eine Sperrholzbox mit Kinderbank zwischen Lenksäule und Vorderrad, Seilzuglenkung, Mittelmotor, Rahmenakku, Kurbel, ein Zweibeinständer unter der Box und zwei gestapelte Kisten; auf dem Ständer ist das Vorderrad angehoben und die Last liegt knapp hinter den Ständerfüßen.",
)

REFERENCES = [
    {"id": "bakfiets-pds-2026", "citation": "Azor Bike BV (2026). Bakfiets.nl Product Data Sheet: Cargo Bike / E-Cargo Bike Long, steel or aluminium frame, Shimano EP6 Cargo mid drive with 504 Wh battery, wheelbase 200 cm, 260 x 64 x 120 cm (L x W x H), cargo box Long 148 l."},
    {"id": "urban-arrow-2025", "citation": "Urban Arrow service centre (2025). Dimensions and use specifications of the Urban Arrow Family: length 263 cm, maximum width 68.5 cm, 50 kg, maximum total load 250 kg."},
]

NOTES = en_de(
    "Long-john cargo bike of the Bakfiets.nl Cargobike Long class, riderless: the assist, rolling and braking keys describe the machine alone (no rider heat). Geometry from the manufacturer data sheet (wheelbase 2.00 m, 2.60 x 0.64 m) and real long-john construction: 26 x 2.0 rear and 20 x 2.0 front tyre (ETRTO 50-559 and 50-406), front head angle 70 degrees with 35 mm fork offset (trail 56 mm), handlebar column at 80 degrees, bottom bracket 0.47 m ahead of the rear axle and 0.30 m high, box floor 0.30 m, box sides 0.33 m high. Motor and battery losses are small (30 W and 9 W at level 1: 12 percent of a 250 W motor and a few percent of the battery draw), so the model shows a warm motor housing rather than a plume. Masses are authored typical values (authored-estimate).",
    "Long-John-Lastenrad der Klasse Bakfiets.nl Cargobike Long ohne Fahrer: die Schlüssel Unterstützung, Rollen und Bremsen beschreiben nur die Maschine (keine Fahrerwärme). Geometrie nach dem Datenblatt des Herstellers (Radstand 2,00 m, 2,60 x 0,64 m) und wirklicher Long-John-Bauweise: Reifen 26 x 2,0 hinten und 20 x 2,0 vorn (ETRTO 50-559 und 50-406), vorderer Lenkkopfwinkel 70 Grad mit 35 mm Gabelversatz (Nachlauf 56 mm), Lenksäule unter 80 Grad, Tretlager 0,47 m vor der Hinterachse und 0,30 m hoch, Boxboden 0,30 m, Boxseiten 0,33 m hoch. Motor- und Akkuverluste sind klein (30 W und 9 W bei Stufe 1: 12 Prozent eines 250-W-Motors und wenige Prozent der Akkuentnahme), daher zeigt das Modell ein warmes Motorgehäuse statt einer Abgasfahne. Massen sind selbst gesetzte typische Werte (authored-estimate).",
)

POSE_TEXT = {
    "parked-on-stand": (en_de("Parked on the stand, box empty", "Auf dem Ständer, Box leer"), en_de("Standing on the rear tyre and the two-leg stand under the box with the front wheel about 3 cm off the ground; the crates wait on the ground behind the bike.", "Steht auf dem Hinterreifen und dem Zweibeinständer unter der Box, das Vorderrad etwa 3 cm über dem Boden; die Kisten warten hinter dem Rad auf dem Boden.")),
    "loaded-on-stand": (en_de("Parked on the stand, 60 kg in the box", "Auf dem Ständer, 60 kg in der Box"), en_de("Same stand position with two crates of 60 kg in the box just behind the stand feet: the stand carries most of the weight, the rear tyre the rest.", "Gleiche Ständerstellung mit zwei Kisten von 60 kg in der Box knapp hinter den Ständerfüßen: der Ständer trägt den größten Teil des Gewichts, der Hinterreifen den Rest.")),
    "rolling-loaded": (None, en_de("Stand folded up under the box, both tyres on the ground, 60 kg in the box, pushed along at walking pace; the freewheel keeps the crank still.", "Ständer unter der Box hochgeklappt, beide Reifen am Boden, 60 kg in der Box, im Schritttempo geschoben; der Freilauf hält die Kurbel still.")),
    "turning-left": (None, en_de("Stand folded up under the box, both tyres on the ground, 60 kg in the box, handlebar and front wheel turned 14 degrees to the left through the steering cables.", "Ständer unter der Box hochgeklappt, beide Reifen am Boden, 60 kg in der Box, Lenker und Vorderrad über die Lenkseile um 14 Grad nach links eingeschlagen.")),
}

LOAD_BOX = en_de(
    "On the stand the two crates are lifted from the ground behind the bike, carried high over the saddle, the handlebar and the box wall and lowered into the box; the 60 kg act on the bike from the moment they rest on the box floor.",
    "Auf dem Ständer werden die zwei Kisten vom Boden hinter dem Rad angehoben, hoch über Sattel, Lenker und Boxwand getragen und in die Box abgesenkt; die 60 kg wirken auf das Rad, sobald sie auf dem Boxboden stehen.",
)

ASSUMPTION = en_de(
    "The bike is a rigid body on equally stiff patches; parked on the two-leg stand under the box the front wheel is lifted and its patch carries nothing, so the stand feet and the rear tyre share the weight. The load in the box sits just behind the stand feet, so the stand carries most of it.",
    "Das Rad ist ein starrer Körper auf gleich steifen Standflächen; auf dem Zweibeinständer unter der Box ist das Vorderrad abgehoben und seine Fläche trägt nichts, Ständerfüße und Hinterreifen teilen das Gewicht. Die Last in der Box liegt knapp hinter den Ständerfüßen, daher trägt der Ständer den größten Teil.",
)


def roots(asset, drawings):
    order = list(asset.keys())
    theta = park_angle()
    asset["summary"] = SUMMARY
    prov = asset["provenance"]
    ids = [r["id"] for r in prov["references"]]
    for ref in REFERENCES:
        if ref["id"] not in ids:
            prov["references"].insert(0 if ref["id"] == "bakfiets-pds-2026" else 1, ref)
    prov["notes"] = NOTES
    asset["dimensions"]["widthM"] = 2.6
    asset["dimensions"]["depthM"] = 0.64
    fx, fy, m = pt.frame_com()
    for bone in asset["rig"]["bones"]:
        spec = BONES.get(bone["id"], {})
        for k, v in spec.items():
            bone[k] = v
        if bone["id"] == "frame":
            bone["mass"]["comX"], bone["mass"]["comY"] = fx, fy
        if bone["id"] in MASS_COM:
            bone["mass"]["comX"], bone["mass"]["comY"] = MASS_COM[bone["id"]]
    for a in asset["anchors"]:
        if a["id"] == "contact-f":
            a["x"], a["y"] = fork_local(kit.F[0], 0.0)
        if a["id"] == "stand-pad":
            a["x"], a["y"] = kit.STAND_LENGTH, 0
            a["title"] = en_de("Stand feet", "Ständerfüße")
            a["footprintLengthM"] = 0.05
            a["footprintWidthM"] = 0.46
        if a["id"] == "crate-cog":
            a["x"], a["y"] = r4(pt.CRATE[0] / 2), pt.CRATE[2]
    dx, dy = crate_ground(theta)
    for pose in asset["poses"]:
        pid = pose["id"]
        if pid in ("parked-on-stand", "loaded-on-stand"):
            pose["bones"]["stand"] = {"angleDeg": kit.STAND_DOWN_DELTA}
            pose["root"]["angleDeg"] = theta
            pose["contacts"] = ["contact-r", "stand-pad"]
        else:
            pose["bones"].pop("stand", None)
        if pid == "parked-on-stand":
            pose["bones"]["crate-x"] = {"dx": dx}
            pose["bones"]["crate-y"] = {"dy": dy}
        if pid in POSE_TEXT:
            title, desc = POSE_TEXT[pid]
            if title is not None:
                pose["title"] = title
            pose["description"] = desc
        pose["bones"] = {k: pose["bones"][k] for k in sorted(pose["bones"], key=lambda b: [x["id"] for x in asset["rig"]["bones"]].index(b))}
    for clip in asset["clips"]:
        if clip["id"] == "load-box":
            clip["description"] = LOAD_BOX
            clip["contacts"] = [{"anchor": "contact-r", "fromS": 0, "toS": clip["durationS"]}, {"anchor": "stand-pad", "fromS": 0, "toS": clip["durationS"]}]
            for key in clip["keys"]:
                b = key.get("bones")
                if b is None:
                    continue
                if key["t"] <= 1:
                    b["crate-x"] = {"dx": dx}
                    b["crate-y"] = {"dy": dy}
                else:
                    b["crate-y"] = {"dy": 0.9}
    st = asset["structure"]
    st["supports"] = ["contact-r"]
    st["assumptions"][0] = ASSUMPTION
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    if drawings is not None:
        for key, d in drawings.items():
            rep = asset["representations"][key]
            rep["geometry"] = d["geometry"]
            rep["decisions"] = d["decisions"]
            rep["pens"] = d["pens"]
    return {k: asset[k] for k in order}


if __name__ == "__main__":
    args = sys.argv[1:]
    doc = json.load(open(ASSET, encoding="utf8"))
    drawings = None
    path2d = os.path.join(OUT, "drawings.json")
    if "--no-2d" not in args and os.path.exists(path2d):
        drawings = json.load(open(path2d, encoding="utf8"))
    doc["asset"] = roots(doc["asset"], drawings)
    src = os.path.join(T, "volumes", "vehicles", "cargo-bike.json")
    cand = json.loads(json.dumps(doc))
    cand["asset"]["volumes"] = json.load(open(src, encoding="utf8"))["volumes"]
    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf8") as fh:
        json.dump(cand, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print("park angle", park_angle(), "crate on ground", crate_ground(park_angle()))
    print("candidate", os.path.join(OUT, "🔣️.json"))
    if "--apply" in args:
        text = open(ASSET, encoding="utf8").read()
        now = json.loads(text)
        now["asset"] = roots(now["asset"], drawings)
        start = text.index('\n  "volumes": {') + 1
        end = text.index('\n  "presentation": {') + 1
        block = text[start:end]
        marker = "@@VOLUMES@@"
        vol = now["asset"]["volumes"]
        now["asset"]["volumes"] = marker
        out = json.dumps(now, ensure_ascii=False, indent=1) + "\n"
        out = out.replace('  "volumes": "' + marker + '",\n', block)
        assert json.loads(out)["asset"]["volumes"] == vol
        with open(ASSET, "w", encoding="utf8") as fh:
            fh.write(out)
        print("applied", ASSET)
