"""🏃 Red squirrel: the shoulder blade takes its share of the fore-limb swing in the poses whose hands are off the ground.

Usage: python3 rework-red-squirrel-poses.py candidate|publish
"""
import json
import re
import sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/red-squirrel/🔣️.json"
ORIGINAL = "rework-red-squirrel-orig.json"
FREE_HANDS = ("running", "bound-gather", "bound-push", "bound-flight", "bound-reach")
SHARE = 0.15
LIMIT = 20


def poses(original):
    out = json.loads(json.dumps(original))
    for pose in out:
        if pose["id"] not in FREE_HANDS:
            continue
        bones = {}
        for bone, offset in pose["bones"].items():
            if bone in ("arm-n", "arm-f"):
                share = round(max(-LIMIT, min(LIMIT, SHARE * offset["angleDeg"])), 1)
                if abs(share) >= 1:
                    bones["scapula-" + bone[-1]] = {"angleDeg": share}
                    offset = dict(offset, angleDeg=round(offset["angleDeg"] - share, 2))
            bones[bone] = offset
        pose["bones"] = bones
        print(pose["id"], {k: v["angleDeg"] for k, v in bones.items() if k[:3] in ("sca", "arm")})
    return out


def member_end(text, start):
    depth, index, quoted = 0, start, False
    while True:
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1


def main():
    new = poses(json.load(open(ORIGINAL, encoding="utf-8"))["poses"])
    text = open(CATALOG, encoding="utf-8").read()
    if sys.argv[1] == "candidate":
        document = json.loads(text)
        document["asset"]["poses"] = new
        source = json.load(open("volumes/animals/red-squirrel.json", encoding="utf-8"))["volumes"]
        if "--source" in sys.argv:
            document["asset"]["volumes"] = source
        json.dump(document, open("🗑️generated/r3-red-squirrel/🔣️.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        return
    found = re.search(r'\n    "poses"\s*:\s*', text)
    end = member_end(text, found.end())
    result = text[: found.end()] + json.dumps(new, ensure_ascii=False, indent=2).replace("\n", "\n    ") + text[end:]
    check, before = json.loads(result), json.loads(text)
    if check["asset"]["poses"] != new or {k: v for k, v in check["asset"].items() if k != "poses"} != {k: v for k, v in before["asset"].items() if k != "poses"}:
        raise SystemExit("poses splice did not round-trip")
    if open(CATALOG, encoding="utf-8").read() != text:
        raise SystemExit("catalog changed while writing; retry")
    open(CATALOG, "w", encoding="utf-8").write(result)
    print("poses written")


main()
