"""📐 Dairy cow 2D carry: re-cuts the limb shapes of every drawing from the rebuilt body with `r3-project2d.ts` as the ruler and adds the pastern shapes of the new pastern bones.

Usage: python3 rework-dairy-cow-2d.py [--scales 50,100] [--views elevation,front,plan] [--bones neck,head,jaw]
Elevation is written first; front and plan take their depth from the elevation twin. The catalog file is re-read right before each write and only `representations` changes.
"""
import glob
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"
GEOMETRY = ("kind", "points", "closed", "d", "x", "y", "width", "height", "rx", "ry", "cx", "cy", "r", "x1", "y1", "x2", "y2", "radius", "followTip", "cornerRadius", "rotationDeg")
LIMBS = [name + side for name in ("scapula-f", "humerus-f", "forearm-f", "cannon-f", "pastern-f", "hoof-f", "thigh-h", "shank-h", "metatarsus-h", "pastern-h", "hoof-h") for side in ("n", "f")]
sys.path.insert(0, HERE)
publish = {"__file__": os.path.join(HERE, "rework-dairy-cow-publish.py")}
exec(open(os.path.join(HERE, "rework-dairy-cow-publish.py"), encoding="utf-8").read().replace("\nmain()\n", "\n"), publish)


def ruler(scale, view):
    bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
    run = subprocess.run([bun, "r3-project2d.ts", "animals/dairy-cow", "--scale", str(scale), "--view", view, "--quiet"], cwd=HERE, capture_output=True, text=True)
    if run.returncode != 0:
        raise SystemExit("[DEBUG] ruler failed: " + run.stdout[-400:] + run.stderr[-400:])
    return json.load(open(os.path.join(HERE, "🗑️generated", "r3-project2d", "animals", "dairy-cow.s%s.%s.json" % (scale, view)), encoding="utf-8"))


def main():
    args = sys.argv[1:]

    def option(name, default):
        return args[args.index(name) + 1] if name in args else default

    limbs = option("--bones", ",".join(LIMBS)).split(",")
    for scale in option("--scales", "50,100,200,500").split(","):
        for view in option("--views", "elevation,front,plan").split(","):
            measured = ruler(scale, view)
            text = open(CATALOG, encoding="utf-8").read()
            document = json.loads(text)
            shapes = document["asset"]["representations"][scale]["geometry"][view]
            log = []
            if view == "front" and "--front-drop" in args:
                for shape in shapes:
                    if shape.get("bone") in LIMBS or "solid" in shape:
                        continue
                    if "points" in shape:
                        shape["points"] = [[x, round(y - 0.06, 4)] for x, y in shape["points"]]
                    else:
                        moved = [key for key in ("cy", "y", "y1", "y2") if key in shape]
                        for key in moved:
                            shape[key] = round(shape[key] - 0.06, 4)
                        log.append("%s %s %s" % (shape["id"], shape.get("kind"), moved or "NOT MOVED"))
            for entry in measured["bones"]:
                bone, outlines = entry["bone"], entry["shapes"]
                if bone not in limbs or not outlines:
                    continue
                side = "near" if bone.endswith("n") else "far"
                if entry.get("replaces"):
                    target = next((s for s in shapes if s["id"] == entry["replaces"] and s.get("bone") == bone), None)
                    if target is None or "solid" in target or "solid" in outlines[0]:
                        log.append("skip " + bone)
                        continue
                    for key in GEOMETRY:
                        target.pop(key, None)
                    for key in GEOMETRY:
                        if key in outlines[0]:
                            target[key] = outlines[0][key]
                    log.append(bone)
                elif bone.startswith(("pastern-", "hoof-", "cannon-", "metatarsus-")):
                    part, tag = bone.split("-")
                    limb = "fore" if tag[0] == "f" else "hind"
                    chain = [name + "-" + tag for name in (("hoof", "pastern", "cannon", "forearm", "humerus") if limb == "fore" else ("hoof", "pastern", "metatarsus", "shank", "thigh"))]
                    hoof = next((s for name in chain for s in shapes if s.get("bone") == name and s.get("filled")), None)
                    part = "cannon" if part in ("cannon", "metatarsus") else part
                    if hoof is None:
                        log.append("no anchor shape for " + bone)
                        continue
                    name = "%s-%s-%s" % (side, limb, part)
                    for old in [s for s in shapes if s["id"] == name]:
                        shapes.remove(old)
                    fresh = {"id": name, "bone": bone, "role": "body", "z": round(hoof["z"] - (0.02 if part == "pastern" else 0.01), 3), "filled": True, "tone": 0.9 if part == "pastern" else 0.32, "region": "lower-legs" if part == "pastern" else "hooves"}
                    for key in GEOMETRY:
                        if key in outlines[0]:
                            fresh[key] = outlines[0][key]
                    shapes.insert(shapes.index(hoof), fresh)
                    log.append("new " + name)
            if open(CATALOG, encoding="utf-8").read() != text:
                raise SystemExit("asset changed while drawing; retry")
            open(CATALOG, "w", encoding="utf-8").write(publish["dump"](document, 0) + "\n")
            print("[DEBUG] 1:%s %s: %s" % (scale, view, ", ".join(log)))


main()
