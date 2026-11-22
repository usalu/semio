"""📐 Author 07: carries re-cut 3D parts into the 2D drawings of one asset with `r3-project2d.ts` as the ruler.

Usage: python3 rework-07-2d.py <slug> --replace bone,bone,... [--under bone:id,bone:id] [--scales 50,100] [--views elevation,front,plan] [--root-member key=jsonfile ...]
`--replace`: the outline of the bone's body solids goes over the bone's shape that the ruler names (shapes with a `solid` are left alone).
`--under`: the outlines of that bone are added as extra shapes `<id>`, `<id>-2` just below the bone's main shape, so only what stands out of it shows.
Elevation is written first, front and plan are measured afterwards (they take their depth from the elevation twin). Only `representations` of the asset is spliced; the file is re-read right before the write.
"""
import glob
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals"
GEOMETRY = ("kind", "points", "closed", "d", "x", "y", "width", "height", "rx", "ry", "cx", "cy", "r", "x1", "y1", "x2", "y2", "radius", "followTip", "cornerRadius", "rotationDeg")


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def member_end(text, start):
    depth, index, quoted = 0, start, False
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
                return index + 1
        index += 1
    raise SystemExit("unbalanced asset file")


def splice(path, key, mutate):
    text = open(path, encoding="utf-8").read()
    document = json.loads(text)
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    found = re.search(r'\n' + indent + r'"' + key + r'"\s*:\s*', text)
    value = document["asset"][key]
    mutate(value, document["asset"])
    end = member_end(text, found.end())
    result = text[: found.end()] + dump(value, len(indent) // 2) + text[end:]
    check = json.loads(result)
    if check["asset"][key] != value or {k: v for k, v in check["asset"].items() if k != key} != {k: v for k, v in document["asset"].items() if k != key}:
        raise SystemExit("splice did not round-trip")
    if open(path, encoding="utf-8").read() != text:
        raise SystemExit("asset changed while splicing; retry")
    open(path, "w", encoding="utf-8").write(result)


def ruler(slug, scale, view):
    bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
    run = subprocess.run([bun, "r3-project2d.ts", "animals/" + slug, "--scale", str(scale), "--view", view, "--quiet"], cwd=HERE, capture_output=True, text=True)
    if run.returncode != 0:
        raise SystemExit("[DEBUG] ruler failed: " + run.stdout[-400:] + run.stderr[-400:])
    return json.load(open(os.path.join(HERE, "🗑️generated", "r3-project2d", "animals", "%s.s%s.%s.json" % (slug, scale, view)), encoding="utf-8"))


def main():
    args = sys.argv[1:]
    slug = args[0]

    def option(name, default=""):
        return args[args.index(name) + 1] if name in args else default

    replace = [b for b in option("--replace").split(",") if b]
    under = dict(item.split(":") for item in option("--under").split(",") if item)
    scales = option("--scales", "50,100,200,500").split(",")
    views = option("--views", "elevation,front,plan").split(",")
    path = os.path.join(CATALOG, slug, "🔣️.json")
    for scale in scales:
        for view in views:
            measured = ruler(slug, scale, view)
            log = []

            def mutate(representations, asset):
                shapes = representations[scale]["geometry"][view]
                for entry in measured["bones"]:
                    bone = entry["bone"]
                    outlines = entry["shapes"]
                    if not outlines:
                        continue
                    if bone in replace and entry["replaces"]:
                        target = next((s for s in shapes if s["id"] == entry["replaces"] and s["bone"] == bone), None)
                        if target is None or "solid" in target or "solid" in outlines[0]:
                            log.append("skip " + bone)
                            continue
                        for key in GEOMETRY:
                            target.pop(key, None)
                        for key in GEOMETRY:
                            if key in outlines[0]:
                                target[key] = outlines[0][key]
                        base = target["id"]
                        for old in [s for s in shapes if re.fullmatch(re.escape(base) + r"-part-\d+", s["id"])]:
                            shapes.remove(old)
                        at = shapes.index(target)
                        for extra in outlines[1:]:
                            at += 1
                            shapes.insert(at, extra)
                        log.append("%s->%s(%d)" % (bone, base, len(outlines)))
                    elif bone in under:
                        main_shape = next((s for s in shapes if s["id"] == entry["replaces"]), None)
                        if main_shape is None:
                            continue
                        name = under[bone]
                        for old in [s for s in shapes if s["id"] == name or re.fullmatch(re.escape(name) + r"-\d+", s["id"])]:
                            shapes.remove(old)
                        at = shapes.index(main_shape)
                        for number, outline in enumerate(outlines):
                            extra = dict(outline)
                            extra["id"] = name if number == 0 else "%s-%d" % (name, number + 1)
                            extra["z"] = round(main_shape["z"] - 0.05, 3)
                            shapes.insert(at, extra)
                            at += 1
                        log.append("%s under %s(%d)" % (bone, main_shape["id"], len(outlines)))

            splice(path, "representations", mutate)
            print("[DEBUG] 1:%s %s: %s" % (scale, view, ", ".join(log)))


if __name__ == "__main__":
    main()
