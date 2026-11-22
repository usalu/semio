"""♦️ Rework 11: the cross-flow plate core becomes what it is, a stack of thin square plates with alternately sealed edges, instead of one solid prism.

Usage: python3 rework-11-plate-heat-exchanger-build.py [--publish]
Frame of bone `plates`: origin on the left corner of the diamond, u along the horizontal diagonal (0.45 m), v up, w along the stack (0.47 m). The plates are 0.318 m squares of 0.3 mm aluminium standing on one corner, parallel to the side elevation; channel k is closed on its north-east and south-west edges, channel k + 1 on its north-west and south-east edges, so the two air streams cross without mixing.
"""
import json
import sys

from rework_11_lib import *

SLUG = "plate-heat-exchanger"
AL = "aluminium-plate"
OLD = pristine(SLUG)
DIAMOND = [[0.225, -0.225], [0.45, 0.0], [0.225, 0.225], [0.0, 0.0]]
EDGE = {
    "north-east": [[0.225, 0.225], [0.45, 0.0], [0.439, 0.0], [0.225, 0.214]],
    "south-west": [[0.225, -0.225], [0.0, 0.0], [0.011, 0.0], [0.225, -0.214]],
    "north-west": [[0.225, 0.225], [0.0, 0.0], [0.011, 0.0], [0.225, 0.214]],
    "south-east": [[0.225, -0.225], [0.45, 0.0], [0.439, 0.0], [0.225, -0.214]],
}


def core(pitch, count):
    """`count` plates of 0.3 mm every `pitch` along the stack and the folded edge seals of the channels between them."""
    gap = pitch - 0.0003
    out = [ext("plates", "plates", AL, 0.72, DIAMOND, "xy", -0.235, -0.2347, slender=True, repeat=rep(count, [0, 0, pitch]))]
    pairs = (count - 1) // 2
    for k, (name, profile) in enumerate(EDGE.items()):
        start = -0.2347 + (pitch if k >= 2 else 0)
        out.append(ext("edge-seal-" + name, "plates", AL, 0.6, profile, "xy", start, start + gap, role="detail", slender=True, repeat=rep(pairs + (0 if k >= 2 and count % 2 == 0 else (1 if k < 2 and count % 2 == 0 else 0)), [0, 0, 2 * pitch])))
    return out


NOTE = {
    "50": text("The core is modelled as it is built: 95 square plates of 0.3 mm aluminium, 0.318 m on a side, every 5 mm along the 0.47 m stack, with the folded edge seals of the 94 channels: every second channel is closed on its north-east and south-west edges and open to the outdoor-air stream, the channels between are closed on the north-west and south-east edges and open to the extract-air stream.", "Der Kern ist gebaut wie in Wirklichkeit: 95 quadratische Platten aus 0,3 mm Aluminium mit 0,318 m Kantenlänge, alle 5 mm entlang des 0,47 m langen Stapels, mit den gefalzten Randdichtungen der 94 Kanäle: jeder zweite Kanal ist an der Nordost- und Südwestkante geschlossen und zum Außenluftstrom offen, die Kanäle dazwischen sind an der Nordwest- und Südostkante geschlossen und zum Abluftstrom offen."),
    "100": text("The core keeps real plates of 0.3 mm, 48 of them every 10 mm, with the alternating edge seals of the 47 channels.", "Der Kern behält echte Platten von 0,3 mm, 48 Stück alle 10 mm, mit den wechselnden Randdichtungen der 47 Kanäle."),
}


def main():
    document, before = load_catalog(SLUG)
    volumes = json.loads(json.dumps(OLD["volumes"]))
    for scale, pitch, count in (("50", 0.005, 95), ("100", 0.01, 48)):
        v = volumes[scale]
        parts = core(pitch, count)
        v["body"] = [s for s in v["body"] if s["id"] != "plate-pack"] + parts
        v["thermal"] = [s for s in v["thermal"] if s.get("of") != "plate-pack"] + [th(p["id"], "plates") for p in parts]
        v["structure"] = v["structure"] + [st("st-plates", "plates", "member", of="plates")]
        v["decisions"] = v["decisions"] + [NOTE[scale]]
    save(SLUG, document, volumes, before, "--publish" in sys.argv)


main()
