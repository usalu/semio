"""🔎 Red squirrel: 1:100, 1:200 and 1:500 lists without the 1:50 fingers, claws, pupils, lids, phalanges and caudal centra; mitten hand, whole sole and one bar per tail bone instead.

Usage: python3 rework-red-squirrel-coarse.py candidate|publish
"""
import json
import re
import sys

SOURCE = "volumes/animals/red-squirrel.json"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/red-squirrel/🔣️.json"
Q = [0, -0.7071068, 0, 0.7071068]
MITTEN = [(-2, 2.2, 1.6), (4, 3.5, 1.6), (10, 5.5, 2.1), (16, 6.0, 1.9), (22, 5.2, 1.6), (26, 3.6, 1.2), (27.6, 1.6, 0.7)]
MITTEN_DROOP = {22: -0.6, 26: -1.1, 27.6: -1.3}
SOLE = [(-6, 2.0, 1.0), (0, 4.4, 1.6), (15, 5.0, 1.8), (25, 6.8, 2.0), (32, 7.4, 1.9), (40, 6.8, 1.8), (48, 5.8, 1.6), (54, 4.2, 1.2), (57.5, 2.0, 0.7)]
TAIL_LENGTH = {"tail-1": 26, "tail-2": 32, "tail-3": 30, "tail-4": 34, "tail-5": 30, "tail-6": 23}
GONE = {
    "100": r"(hand|foot)-(digit|claw)-\d|pupil|supraorbital-lid",
    "200": r"(hand|foot)-(digit|claw)-\d|pupil|supraorbital-lid|muzzle-pad|nasal-recess",
    "500": r"(hand|foot)-(digit|claw)-\d|pupil|supraorbital-lid|muzzle-pad|nasal-recess",
}


def sections(rows, y_of):
    return [{"at": [round(x / 1000, 6), round(y_of(x) / 1000, 6), 0], "rx": rx / 1000, "ry": ry / 1000, "exponent": 2, "rotation": Q} for x, rx, ry in rows]


def coarse(volumes, originals):
    for scale in ("100", "200", "500"):
        v = volumes[scale]
        gone = re.compile(GONE[scale])
        removed = {s["id"] for s in v["body"] if gone.search(s["id"])}
        v["body"] = [s for s in v["body"] if s["id"] not in removed]
        v["thermal"] = [t for t in v["thermal"] if t.get("of") not in removed]
        for s in v["body"]:
            original = originals[scale].get(s["id"])
            if s["id"].endswith("-fingers"):
                s["sections"] = sections(MITTEN, lambda x: original["sections"][0]["at"][1] * 1000 + MITTEN_DROOP.get(x, 0))
            if s["id"].endswith("-sole"):
                s["sections"] = sections(SOLE, lambda x: original["sections"][2]["at"][1] * 1000)
        structure = []
        done = set()
        for s in v["structure"]:
            if s["id"].startswith("phalanx-"):
                continue
            if s["id"].startswith("caudal-centrum-") or s["id"].startswith("m-tail-"):
                bone = s["bone"]
                if bone not in done:
                    done.add(bone)
                    length = TAIL_LENGTH[bone]
                    structure.append({"id": "m-" + bone, "kind": "loft", "bone": bone, "material": "bone", "role": "body", "tone": 0.85, "sections": sections([(-0.6, 0.6, 0.6), (length / 2, 0.6, 0.6), (length + 0.6, 0.6, 0.6)], lambda x: 0), "capStart": True, "capEnd": True, "slender": True, "member": bone, "part": "member"})
                continue
            structure.append(s)
        v["structure"] = structure
    return volumes


def main():
    mode = sys.argv[1]
    volumes = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    originals = json.load(open("rework-red-squirrel-orig.json", encoding="utf-8"))["bodyBeforeCoarse"]
    volumes = coarse(volumes, originals)
    print({k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in volumes.items()})
    if mode == "candidate":
        document = json.load(open(CATALOG, encoding="utf-8"))
        document["asset"]["volumes"] = volumes
        json.dump(document, open("🗑️generated/r3-red-squirrel/🔣️.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    else:
        sys.path.insert(0, ".")
        girdles = open("rework-red-squirrel-girdles.py", encoding="utf-8").read().split("def scalar(value)")[1].split("def member_end")[0]
        scope = {"json": json}
        exec("def scalar(value)" + girdles, scope)
        open(SOURCE, "w", encoding="utf-8").write('{"volumes": ' + scope["dump"](volumes, 0) + "}\n")
        print("source written")


main()
