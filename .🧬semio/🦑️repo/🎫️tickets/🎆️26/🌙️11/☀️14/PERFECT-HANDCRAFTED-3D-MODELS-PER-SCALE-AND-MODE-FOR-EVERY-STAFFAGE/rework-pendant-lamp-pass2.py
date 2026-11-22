"""🏮 Second-round chisel for `appliances/pendant-lamp`: the 1:200 lamp re-turned by hand (smooth dome, brass lampholder apart from the glass).

Usage: python3 rework-pendant-lamp-pass2.py [--root]
Rewrites the 1:200 lists of the volumes source `volumes/appliances/pendant-lamp.json`; every number is stated here.
`--root` redraws the 1:200 shade outline of the side and front elevation in the catalog file to the same nine stations (re-read immediately before the write, every other byte stays).
"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/appliances/pendant-lamp.json")

SHADE_200 = [
    [-0.03, 0.0], [-0.03, 0.0155], [0.0, 0.0155], [0.0, 0.017],
    [0.0011, 0.031], [0.0048, 0.046], [0.0116, 0.061], [0.0222, 0.076], [0.0366, 0.09], [0.0555, 0.103], [0.078, 0.114], [0.1038, 0.123],
    [0.1385, 0.133], [0.1385, 0.13],
    [0.1046, 0.1201], [0.0791, 0.1112], [0.057, 0.1004], [0.0385, 0.0877], [0.0245, 0.074], [0.0142, 0.0595], [0.0077, 0.045], [0.0041, 0.0305],
    [0.003, 0.017], [0.003, 0.0],
]
SHADE_2D_OLD = "M -0.03 0 L -0.03 0.0155 L 0 0.0155 L 0 0.017 L 0.00253 0.03825 L 0.01086 0.05965 L 0.02608 0.08028 L 0.04926 0.0992 L 0.08152 0.11545 L 0.12393 0.12812 L 0.1385 0.133 L 0.1385 -0.133 L 0.12393 -0.12812 L 0.08152 -0.11545 L 0.04926 -0.0992 L 0.02608 -0.08028 L 0.01086 -0.05965 L 0.00253 -0.03825 L 0 -0.017 L 0 -0.0155 L -0.03 -0.0155 L -0.03 0 Z"
SHADE_FRONT_OLD = "M 0 0.176 L 0.0155 0.176 L 0.0155 0.146 L 0.017 0.146 L 0.03825 0.14347 L 0.05965 0.13514 L 0.08028 0.11992 L 0.0992 0.09674 L 0.11545 0.06448 L 0.12812 0.02207 L 0.133 0.0075 L -0.133 0.0075 L -0.12812 0.02207 L -0.11545 0.06448 L -0.0992 0.09674 L -0.08028 0.11992 L -0.05965 0.13514 L -0.03825 0.14347 L -0.017 0.146 L -0.0155 0.146 L -0.0155 0.176 L 0 0.176 Z"
SHADE_APEX_HEIGHT = 0.146
SHADE_2D_OUTER = SHADE_200[1:13]
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/pendant-lamp/🔣️.json"
SOCKET_200 = [[0.033, 0.0], [0.033, 0.02], [0.0956, 0.02], [0.0956, 0.0]]
BULB_200 = [
    [0.0246, 0.0], [0.0246, 0.0135], [0.027, 0.0138], [0.0345, 0.0155], [0.042, 0.0185], [0.051, 0.0235], [0.0607, 0.028],
    [0.073, 0.03], [0.0825, 0.0285], [0.091, 0.0242], [0.098, 0.0177], [0.1025, 0.0106], [0.105, 0.0],
]
BODY_200 = {
    "en": "Body at 30 mm: the rose is a plain 100 mm disc, the cord stays at its true 6.6 mm (slender). The shade is the dome of the 1:50 curve turned at nine stations, so it reads as a smooth dome and not as a faceted cone, as a 3 mm shell of even thickness to the true rim at 0.266 m without the rolled bead; the 31 mm brass collar is its neck, because 31 mm cannot be printed apart from the dome at this scale. Under the dome the brass E27 lampholder is its own 40 by 63 mm sleeve on the holder bone, and the A60 lamp is the glass pear of eleven stations hanging from its mouth; cap, seam and rolled mouth are below the feature and left out.",
    "de": "Körper bei 30 mm: die Rosette ist eine schlichte Scheibe von 100 mm, das Kabel bleibt in seinen wahren 6,6 mm (schlank). Der Schirm ist die Kuppel der 1:50-Kurve, an neun Stationen gedreht, damit er als glatte Kuppel und nicht als facettierter Kegel gelesen wird, als gleichmäßig 3 mm starke Schale bis zum wahren Rand bei 0,266 m ohne den gerollten Wulst; die 31-mm-Messinghülse ist sein Hals, weil 31 mm in diesem Maßstab nicht getrennt von der Kuppel druckbar sind. Unter der Kuppel ist die Messing-E27-Fassung eine eigene Hülse von 40 mal 63 mm am Fassungsknochen, und die A60-Lampe ist die Glasbirne mit elf Stationen, die an ihrer Mündung hängt; Sockel, Naht und gerollte Mündung liegen unter der Merkmalsgröße und entfallen.",
}


def solid(items, identifier):
    return next(item for item in items if item["id"] == identifier)


def number(value):
    return ("%.5f" % value).rstrip("0").rstrip(".") if value else "0"


def shade_path():
    upper = [[x, r] for x, r in SHADE_2D_OUTER]
    lower = [[x, -r] for x, r in reversed(SHADE_2D_OUTER)]
    points = upper + lower
    return "M -0.03 0 L " + " L ".join(number(x) + " " + number(y) for x, y in points) + " L -0.03 0 Z"


def shade_front_path():
    right = [[r, SHADE_APEX_HEIGHT - x] for x, r in SHADE_2D_OUTER]
    left = [[-r, y] for r, y in reversed(right)]
    return "M 0 0.176 L " + " L ".join(number(x) + " " + number(y) for x, y in right + left) + " L 0 0.176 Z"


def edit_root():
    with open(CATALOG, encoding="utf-8") as handle:
        text = handle.read()
    for old, new in ((SHADE_2D_OLD, shade_path()), (SHADE_FRONT_OLD, shade_front_path())):
        if text.count(old) != 1:
            raise SystemExit("expected the old 1:200 shade outline once, found %d" % text.count(old))
        text = text.replace(old, new)
    with open(CATALOG, "w", encoding="utf-8") as handle:
        handle.write(text)


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    scale = document["volumes"]["200"]
    body = scale["body"]
    solid(body, "shade")["profile"] = SHADE_200
    bulb = solid(body, "bulb")
    bulb["profile"] = BULB_200
    socket = {
        "id": "socket", "kind": "revolve", "bone": "holder", "material": "brass-lacquered", "role": "body", "tone": 0.45,
        "profile": SOCKET_200, "axis": [1, 0, 0], "origin": [0, 0, 0],
    }
    scale["body"] = [item for item in body if item["id"] != "socket"]
    scale["body"].insert(scale["body"].index(bulb), socket)
    thermal = scale["thermal"]
    thermal[thermal.index(solid(thermal, "th-bulb"))] = {"id": "th-bulb", "of": "bulb", "region": "bulb-glass"}
    scale["decisions"][0] = BODY_200
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    if "--root" in sys.argv:
        edit_root()


if __name__ == "__main__":
    main()
