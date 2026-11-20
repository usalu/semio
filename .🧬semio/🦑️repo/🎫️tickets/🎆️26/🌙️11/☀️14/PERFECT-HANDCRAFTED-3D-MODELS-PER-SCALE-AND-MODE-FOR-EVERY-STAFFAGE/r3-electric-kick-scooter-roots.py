"""🛴 Round 3 root fixes of vehicles/electric-kick-scooter outside `volumes`: the rig becomes the real folding scooter (the fork bone runs from the steering head down the steering axis to the front axle, the folding hinge sits 0.38 m high above the head tube with its pin behind the stem, the stem is 0.719 m long and folds by 95 degrees onto the rear mudguard), with the anchor, poses, masses, texts, the rear-rim thermal region, the thermal areas the honest model shows, the symbol and the three drawings of every authored scale with their decisions.

Usage: python3 r3-electric-kick-scooter-roots.py [--no-2d] [--apply]
Writes the candidate document to 🗑️generated/r3-electric-kick-scooter/🔣️.json (with the volumes of the source); `--apply` re-reads the catalog file and writes it in the file's own indent-1 style (the `volumes` member keeps its text and is spliced by volumes-insert.py)."""
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/electric-kick-scooter/🔣️.json"
OUT = os.path.join(T, "🗑️generated", "r3-electric-kick-scooter")


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


kit = load("r3-electric-kick-scooter-kit")


def r4(x):
    return round(x, 4) + 0.0


def en_de(en, de):
    return {"en": en, "de": de}


CONTACT_F = kit.loc("fork", (kit.AF[0], 0.0))

BONES = {
    "fork": dict(title=en_de("Steerer, fork crown and fork blades", "Gabelschaft, Gabelkrone und Gabelscheiden"), x=kit.H[0], y=kit.H[1], angleDeg=-90.0 + kit.RAKE, lengthM=r4(kit.FORK_LENGTH)),
    "stem": dict(title=en_de("Folding stem with cockpit and handlebar", "Klappbare Lenkstange mit Cockpit und Lenker"), x=0.0, y=-kit.PIN_BACK, angleDeg=180.0, lengthM=kit.STEM_LENGTH, joint={"type": "revolute", "minDeg": 0, "maxDeg": kit.FOLD_DEG}),
    "wheel-f": dict(x=r4(kit.FORK_LENGTH), y=r4(kit.FORK_OFFSET), angleDeg=90.0 - kit.RAKE),
}

MASS_COM = {"fork": (0.15, 0.0), "stem": (0.36, -kit.PIN_BACK)}

AREAS = {"deck-battery": 0.34, "deck-grip": 0.137, "hub-motor": 0.067, "stem-bars": 0.208, "fenders": 0.073, "tyres": 0.167, "brake-disc": 0.0126, "rear-rim": 0.08}

REFERENCE = {"id": "xiaomi-m365-2019", "citation": "Xiaomi (2019). Mi Electric Scooter (M365) specifications, mi.com: 1080 x 430 x 1140 mm unfolded, 1080 x 430 x 490 mm folded, 12.5 kg, 8.5 inch pneumatic tyres, 250 W front hub motor, front regenerative and rear disc brake."}

NOTES = en_de(
    "Typical adult folding e-scooter of the Xiaomi M365 class (1.09 x 0.43 x 1.11 m, wheelbase 0.86 m, 8 1/2 x 2 tyres of 0.216 m, 12.5 kg): an extruded aluminium deck 0.15 m wide with the battery inside, a gooseneck to the head tube, a two-blade fork around the front hub motor, the folding hinge 0.38 m high above the head tube and a stem that folds back by 95 degrees until the bell meets the catch on the rear mudguard; steering axis raked 8 degrees. Dimensions and power data are authored class values (authored-estimate) checked against the manufacturer's figures. Wheels roll with theta = -x / r (r 0.108 m). The rider is excluded.",
    "Typischer klappbarer Erwachsenen-E-Roller der Klasse Xiaomi M365 (1,09 x 0,43 x 1,11 m, Radstand 0,86 m, Reifen 8 1/2 x 2 mit 0,216 m, 12,5 kg): ein stranggepresstes Aluminium-Trittbrett von 0,15 m Breite mit dem Akku darin, ein Schwanenhals zum Steuerrohr, eine zweischeidige Gabel um den vorderen Nabenmotor, das Klappgelenk 0,38 m hoch über dem Steuerrohr und eine Lenkstange, die um 95 Grad nach hinten klappt, bis die Klingel den Haken am hinteren Schutzblech trifft; Lenkachse um 8 Grad geneigt. Abmessungen und Leistungsdaten sind selbst gesetzte Klassenwerte (authored-estimate), geprüft an den Herstellerangaben. Die Räder rollen mit theta = -x / r (r 0,108 m). Der Fahrer ist ausgeschlossen.",
)

ASSUMPTION = en_de(
    "The stem folds back about the hinge pin behind the steering head by 95 degrees until the handlebar lies over the rear mudguard; steering is a rotation of fork, front wheel and stem about the steering axis, which is raked 8 degrees.",
    "Die Lenkstange klappt um den Gelenkbolzen hinter dem Steuerkopf um 95 Grad nach hinten, bis der Lenker über dem hinteren Schutzblech liegt; Lenken ist eine Drehung von Gabel, Vorderrad und Lenkstange um die um 8 Grad geneigte Lenkachse.",
)

FOLDED = en_de(
    "Stem folded back by 95 degrees about the hinge pin until the handlebar lies over the rear mudguard and the bell meets the latch catch, charging at 63 W indoors.",
    "Lenkstange um 95 Grad um den Gelenkbolzen nach hinten geklappt, bis der Lenker über dem hinteren Schutzblech liegt und die Klingel den Haken trifft, Laden mit 63 W in Innenräumen.",
)

RIM_REGION = {
    "id": "rear-rim",
    "title": en_de("Rear wheel rim, spokes and hub", "Felge, Speichen und Nabe des Hinterrads"),
    "kind": "surface",
    "model": "passive",
    "material": "anodised",
    "areaM2": 0.05,
    "characteristicLengthM": 0.15,
    "geometry": "cylinder",
    "skyViewFactor": 0.3,
    "layers": [],
    "heatCapacityJM2K": 8000,
    "source": "authored-estimate",
    "reference": "ashrae-2021",
}

UNAVAILABLE = en_de(
    "At 1:500 the scooter is 2.2 mm long: the 0.216 m wheels are 0.43 mm discs, the 0.04 m stem 0.08 mm and the 0.055 m deck 0.11 mm, all below the printable feature size, and a composed silhouette would be an unreadable blob.",
    "Bei 1:500 ist der Roller 2,2 mm lang: die 0,216 m großen Räder sind 0,43-mm-Scheiben, die 0,04 m dicke Lenkstange 0,08 mm und das 0,055 m dicke Trittbrett 0,11 mm, alles unter der druckbaren Einzelheitsgröße; eine zusammengesetzte Silhouette wäre ein unlesbarer Fleck.",
)


def symbol(sym):
    """🔣 The true-extent symbol follows the corrected body: the deck ends between the wheels and the steering line runs through the hinge."""
    bar = kit.stem_pt(kit.STEM_LENGTH, -kit.PIN_BACK)
    for s in sym["geometry"]["elevation"]:
        if s["id"] == "elevation-deck":
            s.update(x=-0.316, y=0.095, w=0.584, h=0.055)
        if s["id"] == "elevation-fork":
            s["points"] = [[0.43, 0.108], [r4(kit.H[0]), r4(kit.H[1])], [r4(bar[0]), r4(bar[1])]]
        if s["id"] == "elevation-wheel-f" or s["id"] == "elevation-wheel-r":
            s.update(cy=0.108, rx=0.108, ry=0.108)
        if s["id"] == "elevation-grip":
            s.update(x=r4(bar[0] - 0.035), y=1.074, w=0.07, h=0.036)
    for s in sym["geometry"]["front"]:
        if s["id"] == "front-wheel":
            s.update(x=-0.025, w=0.05, h=0.216)
        if s["id"] == "front-deck":
            s.update(x=-0.075, y=0.095, w=0.15, h=0.055)
        if s["id"] == "front-stem":
            s.update(x=-0.022, y=0.23, w=0.044, h=0.862)
        if s["id"] == "front-bars":
            s.update(y=1.074, h=0.036)
    for s in sym["geometry"]["plan"]:
        if s["id"] == "plan-wheel-f":
            s.update(x=0.322, y=-0.025, w=0.216, h=0.05)
        if s["id"] == "plan-wheel-r":
            s.update(x=-0.538, y=-0.025, w=0.216, h=0.05)
        if s["id"] == "plan-deck":
            s.update(x=-0.316, y=-0.075, w=0.584, h=0.15)
        if s["id"] == "plan-stem":
            s["points"] = [[r4(bar[0]), 0], [0.43, 0]]
        if s["id"] == "plan-bars":
            s.update(x=r4(bar[0] - 0.018), w=0.036)


def roots(asset, drawings):
    order = list(asset.keys())
    prov = asset["provenance"]
    if REFERENCE["id"] not in [r["id"] for r in prov["references"]]:
        prov["references"].insert(0, REFERENCE)
    prov["notes"] = NOTES
    for bone in asset["rig"]["bones"]:
        for k, v in BONES.get(bone["id"], {}).items():
            bone[k] = v
        if bone["id"] in MASS_COM:
            bone["mass"]["comX"], bone["mass"]["comY"] = MASS_COM[bone["id"]]
    for a in asset["anchors"]:
        if a["id"] == "contact-f":
            a["x"], a["y"] = r4(CONTACT_F[0]), r4(CONTACT_F[1])
    for pose in asset["poses"]:
        if pose["id"] == "braking-stop":
            pose["bones"]["fork"] = {"twistDeg": -10}
        if pose["id"] == "folded-charging":
            pose["bones"]["stem"] = {"angleDeg": kit.FOLD_DEG}
            pose["description"] = FOLDED
        regions = pose["thermal"]["regions"]
        if "rear-rim" not in regions:
            regions["rear-rim"] = {"exposedFraction": 1, "sunlitFraction": regions["tyres"]["sunlitFraction"]}
    th = asset["thermal"]
    if "rear-rim" not in [r["id"] for r in th["regions"]]:
        th["regions"].append(json.loads(json.dumps(RIM_REGION)))
    for region in th["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    asset["structure"]["assumptions"][1] = ASSUMPTION
    for sym in asset["symbols"]:
        symbol(sym)
    rep = asset["representations"]["500"]
    rep["reason"] = UNAVAILABLE
    rep["limitingSizeM"] = 0.038
    if drawings is not None:
        for key, d in drawings.items():
            rep = asset["representations"][key]
            rep["geometry"] = d["geometry"]
            if d.get("decisions") is not None:
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
    src = os.path.join(T, "volumes", "vehicles", "electric-kick-scooter.json")
    cand = json.loads(json.dumps(doc))
    cand["asset"]["volumes"] = json.load(open(src, encoding="utf8"))["volumes"]
    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf8") as fh:
        json.dump(cand, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
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
