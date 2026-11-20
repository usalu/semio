"""🌱 Round 3 electric jug kettle: builds the candidate asset document with every root correction (rig, anchor, 2D drawings of every scale and view, representation decisions) from the catalog file and the drawing module, writes it to `🗑️generated/r3-electric-kettle/🔣️.json` with the source volumes merged in, and with `--apply` writes the changed members into the catalog file in its own formatting (re-read right before, member order kept, `volumes` untouched).

Usage: python3 r3-electric-kettle-roots.py [--apply]
"""
import copy
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/electric-kettle/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-electric-kettle", "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "appliances", "electric-kettle.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])
_spec = importlib.util.spec_from_file_location("draw", os.path.join(HERE, "r3-electric-kettle-draw.py"))
D = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(D)


AREAS = {"shell": 0.104, "lid": 0.019}
REFERENCE = {"id": "retail-jug-kettles-2026", "citation": "Currys UK and Bosch product data of 1.7 L, 2 to 2.2 kW stainless jug kettles (Russell Hobbs Classics, Ariete Vintage 2869 and 2877, Bosch TWK7203), read 2026-11-20: overall 205 to 305 mm high, 220 to 230 mm wide, 160 to 220 mm deep; 360-degree power base, covered spout with limescale filter, C handle with the rocker switch at its foot, water window on the side."}


def bone(asset, bid):
    return next(b for b in asset["rig"]["bones"] if b["id"] == bid)


def roots(asset):
    """🌱 Every root correction in one place, applied to a copy of the catalog asset."""
    a = copy.deepcopy(asset)
    h = bone(a, "handle")
    h["x"] = D.K.HANDLE_X
    h["lengthM"] = 0.057
    h["mass"]["comX"] = 0.035
    w = bone(a, "water")
    w["mass"]["comY"] = 0.03
    tip = next(x for x in a["anchors"] if x["id"] == "spout-tip")
    tip["x"], tip["y"] = D.K.SPOUT_TIP
    for region in a["thermal"]["regions"]:
        region["areaM2"] = AREAS.get(region["id"], region["areaM2"])
    for x in a["thermal"]["assumptions"]:
        x["en"] = x["en"].replace("so the boil still takes about 184 seconds", "so the boil takes about 179 seconds")
        x["de"] = x["de"].replace("sodass das Aufkochen weiterhin etwa 184 Sekunden dauert", "sodass das Aufkochen etwa 179 Sekunden dauert")
    boil = next(c for c in a["clips"] if c["id"] == "boil")
    shift = {171: 166, 184: 179, 141: 136}
    for k in boil["keys"]:
        k["t"] = shift.get(k["t"], k["t"])
    for ph in boil["phases"]:
        ph["fromS"], ph["toS"] = shift.get(ph["fromS"], ph["fromS"]), shift.get(ph["toS"], ph["toS"])
    for tr in boil["tracks"]:
        for k in tr["keys"]:
            k["t"] = shift.get(k["t"], k["t"])
    refs = a["provenance"]["references"]
    if not any(r["id"] == REFERENCE["id"] for r in refs):
        refs.append(REFERENCE)
    a["provenance"]["notes"] = json.load(open(os.path.join(HERE, "r3-electric-kettle-texts.json"), encoding="utf-8"))["provenanceNotes"]
    for den, rep in a["representations"].items():
        if rep["status"] != "authored":
            continue
        rep["geometry"] = D.geometry(den, rep["geometry"])
        rep["decisions"] = D.decisions(den, rep["decisions"])
    return a


def inline(value, indent=0):
    """🖨️ JSON in the catalog's style: two-space indent, arrays of numbers and of number pairs on one line."""
    pad = "  " * indent
    if isinstance(value, list) and value and all(isinstance(x, (int, float)) and not isinstance(x, bool) for x in value):
        return json.dumps(value)
    if isinstance(value, list) and value and all(isinstance(x, list) and all(isinstance(y, (int, float)) for y in x) for x in value):
        return "[" + ", ".join(json.dumps(x) for x in value) + "]"
    if isinstance(value, list):
        if not value:
            return "[]"
        return "[\n" + ",\n".join(pad + "  " + inline(x, indent + 1) for x in value) + "\n" + pad + "]"
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(pad + "  " + json.dumps(k, ensure_ascii=False) + ": " + inline(v, indent + 1) for k, v in value.items()) + "\n" + pad + "}"
    return json.dumps(value, ensure_ascii=False)


def apply(want):
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    changed = [key for key in have if key != "volumes" and have[key] != want[key]]
    for key in changed:
        start, end = member_range(text, "    ", key)
        text = text[:start] + "    " + json.dumps(key) + ": " + inline(want[key], 2) + text[end:]
    got = json.loads(text)["asset"]
    if list(got) != list(have) or any(got[key] != want[key] for key in want if key != "volumes") or got["volumes"] != have["volumes"]:
        raise SystemExit("the edited text does not match the candidate")
    open(CATALOG, "w", encoding="utf-8").write(text)
    return changed


def main():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    want = roots(doc["asset"])
    cand = copy.deepcopy(doc)
    cand["asset"] = copy.deepcopy(want)
    cand["asset"]["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    with open(CANDIDATE, "w", encoding="utf-8") as f:
        json.dump(cand, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("candidate", CANDIDATE)
    if "--apply" in sys.argv:
        print("written", CATALOG, "members:", ", ".join(apply(want)) or "none")


if __name__ == "__main__":
    main()
