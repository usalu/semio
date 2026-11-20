"""🚲 Round 3 root fixes of vehicles/city-bicycle outside `volumes`: the stand becomes the real two-leg centre stand behind the bottom bracket (rig bone, anchor, poses, clip keys, titles), the wheelbase note and the frame centre of mass are corrected, the three drawings of every authored scale are replaced by the true views of the handcrafted bodies and their decisions are rewritten.

Usage: python3 r3-city-bicycle-roots.py [--park DEG] [--no-2d] [--apply]
Writes the candidate document to 🗑️generated/r3-city-bicycle/🔣️.json (with the volumes of the source); `--apply` re-reads the catalog file and writes it in the file's own indent-1 style (the `volumes` member keeps its text and is spliced by volumes-insert.py)."""
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/city-bicycle/🔣️.json"
OUT = os.path.join(T, "🗑️generated", "r3-city-bicycle")


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


kit = load("r3-city-bicycle-kit")

TEXT = [
    ("dynamo lamps and a rear stand", "dynamo lamps and a two-leg centre stand"),
    ("Dynamobeleuchtung und Hinterbauständer", "Dynamobeleuchtung und Zweibeinständer"),
    ("wheelbase 1.12 m, 37-622 tyres, rear stand,", "wheelbase 1.14 m, 37-622 tyres, two-leg centre stand,"),
    ("Radstand 1,12 m, Reifen 37-622, Hinterbauständer,", "Radstand 1,14 m, Reifen 37-622, Zweibeinständer,"),
    ("Parked on the rear stand", "Parked on the centre stand"),
    ("Auf dem Hinterbauständer geparkt", "Auf dem Zweibeinständer geparkt"),
    ("The rear stand is down and lifts", "The two-leg centre stand is down and lifts"),
    ("Der Hinterbauständer ist ausgeklappt", "Der Zweibeinständer ist ausgeklappt"),
    ("Technical key: rear stand down 3 m on", "Technical key: centre stand down 3 m on"),
    ("Technische Schlüsselstellung: Hinterbauständer 3 m weiter ausgeklappt", "Technische Schlüsselstellung: Zweibeinständer 3 m weiter ausgeklappt"),
]


def retext(v):
    if isinstance(v, dict):
        return {k: retext(x) for k, x in v.items()}
    if isinstance(v, list):
        return [retext(x) for x in v]
    if isinstance(v, str):
        for a, b in TEXT:
            v = v.replace(a, b)
    return v


def roots(asset, park, drawings):
    order = list(asset.keys())
    asset = {k: (v if k in ("volumes", "representations") else retext(v)) for k, v in asset.items()}
    for bone in asset["rig"]["bones"]:
        if bone["id"] == "stand":
            bone["title"] = {"en": "Two-leg centre stand", "de": "Zweibeinständer"}
            bone["x"], bone["y"] = kit.P
            bone["angleDeg"] = kit.STAND_REST_DEG
            bone["lengthM"] = kit.STAND_LENGTH
            bone["joint"] = {"type": "revolute", "minDeg": 0, "maxDeg": 110}
            bone["mass"]["comX"] = 0.11
        if bone["id"] == "frame":
            bone["mass"]["comX"] = FRAME_COM[0]
            bone["mass"]["comY"] = FRAME_COM[1]
    for anchor in asset["anchors"]:
        if anchor["id"] == "stand-foot":
            anchor["x"], anchor["y"] = kit.STAND_LENGTH, 0
            anchor["title"] = {"en": "Stand feet", "de": "Ständerfüße"}
            anchor["footprintLengthM"] = 0.03
            anchor["footprintWidthM"] = 0.2
    for pose in asset["poses"]:
        if "stand" in pose["bones"]:
            pose["bones"]["stand"] = {"angleDeg": kit.STAND_DOWN_DEG}
            pose["root"]["angleDeg"] = park
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


FRAME_COM = (-0.251, 0.593)
AREAS = {"mudguards": 0.27, "lamps": 0.028, "drivetrain": 0.3}

if __name__ == "__main__":
    args = sys.argv[1:]
    park = float(args[args.index("--park") + 1]) if "--park" in args else -1.9
    doc = json.load(open(ASSET, encoding="utf8"))
    drawings = None
    path2d = os.path.join(OUT, "drawings.json")
    if "--no-2d" not in args and os.path.exists(path2d):
        drawings = json.load(open(path2d, encoding="utf8"))
    doc["asset"] = roots(doc["asset"], park, drawings)
    src = os.path.join(T, "volumes", "vehicles", "city-bicycle.json")
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
        now["asset"] = roots(now["asset"], park, drawings)
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
