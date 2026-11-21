"""🧷 Assembles and writes the round-5 root fixes of `humans/pregnant-person`.

Without arguments it builds the candidate document `🗑️generated/r5-pregnant-person/🔣️.json`: the catalog asset with `roots.json` laid over it (from `r5-person-with-rollator-arms.ts`: the shoulder joints moved out to the real biacromial breadth, the arms of every pose and clip key re-solved so the hands keep their places on the back and the abdomen), the redrawn outlines of the shapes listed in `PROJECTED` taken from `r3-project2d.ts` (ruling E1: the sweater outline now includes the knit that falls from the bust onto the abdomen, `belly-drape`) and the hand prisms (`extruded`, sweep `w`, lateral span in the rest pose) shifted 8 mm outwards with the shoulder joints (`HANDS`, absolute values), and the volumes source merged in; `dimensions.widthM` follows the wider shoulders (0.525 -> 0.531 m, right elbow out to the left sleeve in `hand-on-back`; the forward length from the right elbow to the abdomen and left thumb measures 0.520 to 0.541 m in the drawings of the four scales and 0.511 to 0.524 m in their bodies, and this is the value all of them agree with). With `--write` the members that differ are replaced textually in the catalog file (re-read right before, the file's own style, every other member byte-identical, member order checked; `volumes` is left to `volumes-insert.py`).

Usage: python3 r5-pregnant-person-candidate.py [--write]
@see ./r3-humans-pregnant-person.md
"""
import glob
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
ASSET = glob.glob(CATALOG + "/*/pregnant-person/🔣️.json")[0]
MANIFEST = CATALOG + "/🔣️.json"
OUT = os.path.join(HERE, "🗑️generated", "r5-pregnant-person")


def member_range(text, indent, key):
    found = re.search(r"\n" + indent + r'"' + key + r'"\s*:\s*', text)
    depth, index, quoted = 0, found.end(), False
    while index < len(text):
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
                return found.start() + 1, index + 1
        elif depth == 0 and char in ",\n":
            return found.start() + 1, index
        index += 1
    raise SystemExit("unbalanced asset file")


def load(name):
    path = os.path.join(OUT, name) if not name.startswith("r5-") else os.path.join(HERE, name)
    return json.load(open(path, encoding="utf-8")) if os.path.exists(path) else None


WIDTH = 0.531
HANDS = json.loads('{"50|elevation|hand-l": [0.1178, 0.1765], "50|elevation|hand-r": [-0.1765, -0.1277], "50|front|hand-r": [-0.1765, -0.1277], "50|front|hand-l": [0.1178, 0.1765], "50|plan|hand-r": [-0.1765, -0.1277], "50|plan|hand-l": [0.1178, 0.1765], "100|elevation|hand-l": [0.1217, 0.176], "100|elevation|hand-r": [-0.176, -0.1217], "100|front|hand-r": [-0.176, -0.1217], "100|front|hand-l": [0.1217, 0.176], "100|plan|hand-r": [-0.176, -0.1217], "100|plan|hand-l": [0.1217, 0.176], "200|elevation|hand-l": [0.1275, 0.176], "200|elevation|hand-r": [-0.176, -0.1347], "200|front|hand-r": [-0.176, -0.1347], "200|front|hand-l": [0.1275, 0.176], "200|plan|hand-r": [-0.176, -0.1347], "200|plan|hand-l": [0.1275, 0.176], "500|elevation|hand-l": [0.0998, 0.1982], "500|elevation|hand-r": [-0.1982, -0.107], "500|front|hand-r": [-0.1982, -0.107], "500|front|hand-l": [0.0998, 0.1982], "500|plan|hand-r": [-0.1982, -0.107], "500|plan|hand-l": [0.0998, 0.1982]}')
PROJECTED = [(scale, view, "torso", "sweater") for scale in ("50", "100", "200") for view in ("elevation", "front", "plan")]
PROJECT2D = os.path.join(HERE, "🗑️generated", "r3-project2d", "humans")


def patched(asset):
    roots = load("roots.json")
    if roots:
        asset.update({k: roots[k] for k in ("rig", "poses", "clips") if k in roots})
    asset["dimensions"]["widthM"] = WIDTH
    for key, (z0, z1) in HANDS.items():
        scale, view, shape_id = key.split("|")
        shape = next(s for s in asset["representations"][scale]["geometry"][view] if s["id"] == shape_id)
        shape["solid"]["z0"], shape["solid"]["z1"] = z0, z1
    for scale, view, bone, shape_id in PROJECTED:
        projected = json.load(open(os.path.join(PROJECT2D, "pregnant-person.s%s.%s.%s.json" % (scale, view, bone)), encoding="utf-8"))
        new = next(s for b in projected["bones"] for s in b["shapes"] if s["id"] == shape_id)
        shapes = asset["representations"][scale]["geometry"][view]
        old = next(s for s in shapes if s["id"] == shape_id and s.get("bone") == bone)
        for key in ("kind", "points", "closed", "d"):
            if key in new:
                old[key] = new[key]
            elif key in old and key in ("d", "points"):
                del old[key]
    return asset


text = open(ASSET, encoding="utf-8").read()
before = json.loads(text)
after = patched(json.loads(text)["asset"])
if "--write" not in sys.argv:
    document = json.loads(text)
    document["asset"] = dict(after)
    document["asset"]["volumes"] = json.load(open(os.path.join(HERE, "volumes", "humans", "pregnant-person.json"), encoding="utf-8"))["volumes"]
    os.makedirs(OUT, exist_ok=True)
    json.dump(document, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False)
    print("candidate", os.path.join(OUT, "🔣️.json"), "changed:", ", ".join(k for k in before["asset"] if before["asset"][k] != after[k]))
    raise SystemExit(0)
indent = re.search(r'\n( +)"id"\s*:', text).group(1)
depth = len(indent) // 2
changed = [k for k in before["asset"] if k != "volumes" and before["asset"][k] != after[k]]
for key in changed:
    start, end = member_range(text, indent, key)
    body = json.dumps(after[key], ensure_ascii=False, indent=2).replace("\n", "\n" + "  " * depth)
    text = text[:start] + indent + '"%s": ' % key + body + text[end:]
check = json.loads(text)
for key in before["asset"]:
    if key not in changed and check["asset"][key] != before["asset"][key]:
        raise SystemExit("member %s changed unexpectedly" % key)
    if key in changed and check["asset"][key] != after[key]:
        raise SystemExit("member %s not written as meant" % key)
if list(check["asset"].keys()) != list(before["asset"].keys()):
    raise SystemExit("member order changed")
print("changed members:", ", ".join(changed) or "none")
if changed:
    open(ASSET, "w", encoding="utf-8").write(text)
    print("written", ASSET)
manifest = open(MANIFEST, encoding="utf-8").read()
row = re.search(r'\{\s*"id": "pregnant-person",.*?"poses"', manifest, re.S)
new = row.group(0)
for key in ("heightM", "widthM"):
    new = re.sub(r'"%s": [0-9.]+' % key, '"%s": %s' % (key, json.dumps(after["dimensions"][key])), new)
if new != row.group(0):
    open(MANIFEST, "w", encoding="utf-8").write(manifest[: row.start()] + new + manifest[row.end():])
    print("manifest row updated")
