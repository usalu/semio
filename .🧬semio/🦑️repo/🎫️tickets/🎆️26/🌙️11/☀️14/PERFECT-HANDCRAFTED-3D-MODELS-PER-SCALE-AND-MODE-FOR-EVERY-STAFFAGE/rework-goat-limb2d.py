"""📏 Chisel of the goat author: the elevation profiles of shoulder and arm (extruded spans) at 1:50 and 1:100.

The ruler (`r3-project2d.ts`, output kept under the generated folder) gives the outline of the re-cut body in the
bone frame; the profile keeps its `solid` and gets the lateral span of the new muscle sheet beside the plane.
"""
import json
import re
import sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/goat/🔣️.json"
SPAN = {"shoulder": (0.035, 0.107), "arm": (0.044, 0.108)}


def ruler(scale):
    shapes = {}
    for line in open("🗑️generated/rework-goat/ruler-%s.txt" % scale, encoding="utf-8"):
        if line.startswith('{"id":'):
            shape = json.loads(line)
            shapes[shape["id"]] = shape
    return shapes


def close(text, start):
    depth = 0
    for index in range(start, len(text)):
        if text[index] == "[":
            depth += 1
        elif text[index] == "]":
            depth -= 1
            if depth == 0:
                return index + 1
    raise ValueError("unclosed")


def main():
    with open(CATALOG, encoding="utf-8") as handle:
        text = handle.read()
    for scale in ("50", "100"):
        shapes = ruler(scale)
        for part, (inner, outer) in SPAN.items():
            for side, sign in (("n", -1), ("f", 1)):
                ident = part + "-" + side
                base = text.index('"%s": {' % scale, text.index('\n    "representations"'))
                view = text.index('"elevation"', base)
                at = text.index('"id": "%s"' % ident, view)
                assert at < text.index('"front"', view), ident
                points = text.index('"points": ', at) + len('"points": ')
                text = text[:points] + json.dumps(shapes[ident]["points"]) + text[close(text, points):]
                low, high = sorted((sign * inner, sign * outer))
                for key, value in (("z0", low), ("z1", high)):
                    found = re.compile(r'"%s": -?[0-9.]+' % key).search(text, at)
                    assert found.start() < text.index('"id": ', at + 5), key
                    text = text[:found.start()] + '"%s": %s' % (key, value) + text[found.end():]
    json.loads(text)
    with open(CATALOG, "w", encoding="utf-8") as handle:
        handle.write(text)
    print("[DEBUG] limb profiles written", file=sys.stderr)


main()
