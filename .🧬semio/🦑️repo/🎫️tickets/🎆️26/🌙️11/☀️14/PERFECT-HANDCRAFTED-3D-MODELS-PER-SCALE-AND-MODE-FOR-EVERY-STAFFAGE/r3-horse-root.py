"""🐴 Round 3 horse: root fixes of the catalog asset outside `volumes` (rig, anchors, poses, dimensions, 2D shapes), written member by member in the file's own indent-2 style.

Usage: python3 r3-horse-root.py rig            (thorax length, bones hanging on it, back anchor, default pose head and ears)
       python3 r3-horse-root.py members <patch.json>   (patch = { "dimensions": {...}, "representations": {...}, "anchors": [...], ... } whole members)
The asset file is re-read right before writing; only the named members change, every other byte stays.
"""
import json
import sys

FILE = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/horse/🔣️.json"


def block(key, value):
    text = json.dumps({key: value}, indent=2, ensure_ascii=False)[2:-2]
    return "\n".join("  " + line for line in text.split("\n"))


def write(change):
    text = open(FILE, encoding="utf-8").read()
    asset = json.loads(text)["asset"]
    log = []
    for key, new in change(asset, log).items():
        old = block(key, asset[key])
        if text.count(old) != 1:
            raise SystemExit("member %s not found exactly once" % key)
        text = text.replace(old, block(key, new))
    json.loads(text)
    open(FILE, "w", encoding="utf-8").write(text)
    for line in log:
        print(line)


def rig(asset, log):
    bones = json.loads(json.dumps(asset["rig"]))
    anchors = json.loads(json.dumps(asset["anchors"]))
    poses = json.loads(json.dumps(asset["poses"]))

    def put(holder, key, value, name):
        if holder.get(key) != value:
            log.append("%s: %s -> %s" % (name, holder.get(key), value))
            holder[key] = value

    by = {b["id"]: b for b in bones["bones"]}
    put(by["thorax"], "lengthM", 0.88, "rig thorax.lengthM")
    put(by["thorax"]["mass"], "comX", 0.44, "rig thorax.mass.comX")
    put(by["neck-1"], "x", 0.84, "rig neck-1.x")
    put(by["scapula-ff"], "x", 0.6, "rig scapula-ff.x")
    put(by["scapula-fn"], "x", 0.6, "rig scapula-fn.x")
    for a in anchors:
        if a["id"] == "back-plume":
            put(a, "x", 0.44, "anchor back-plume.x")
            put(a, "y", 0.2, "anchor back-plume.y")
    for p in poses:
        if p["id"] == "standing-alert":
            put(p["bones"]["head"], "angleDeg", -12, "pose standing-alert head.angleDeg")
            put(p["bones"]["ear-n"], "angleDeg", -6, "pose standing-alert ear-n.angleDeg")
            put(p["bones"]["ear-f"], "angleDeg", 0, "pose standing-alert ear-f.angleDeg")
    return {"rig": bones, "anchors": anchors, "poses": poses}


def members(path):
    patch = json.load(open(path, encoding="utf-8"))

    def change(asset, log):
        for key in patch:
            log.append("member %s replaced" % key)
        return patch

    return change


if __name__ == "__main__":
    if sys.argv[1] == "rig":
        write(rig)
    elif sys.argv[1] == "members":
        write(members(sys.argv[2]))
    else:
        raise SystemExit(__doc__)
