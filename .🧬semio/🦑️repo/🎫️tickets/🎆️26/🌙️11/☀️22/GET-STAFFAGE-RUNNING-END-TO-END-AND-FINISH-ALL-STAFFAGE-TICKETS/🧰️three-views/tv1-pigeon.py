"""🐦️ Pigeon, three-views sweep (clip take-off, toes hanging at 0.43 s): the front toes were end-on outlines carried as plates, so a toe that points down stayed 4 mm tall in the front elevation while the side elevation showed its 36 mm. The front toes of 1:50 and 1:100 become extruded solids: a hand-drawn profile in the toe bone's frame and the lateral span the front drawing had (the splayed toes of 1:50 run diagonally with `zTip`). `python3 tv1-pigeon.py [target json]`."""
import sys

import tv1_edit as e

doc = e.Doc(e.catalog_file("pigeon"))

TOES = {
    50: {
        "toes": ([[-0.0032, -0.0031], [0.0008, -0.0038], [0.0351, -0.0018], [0.0295, 0.0], [-0.0005, 0.0005]], (0.0179, 0.022), None),
        "toe-inner": ([[-0.0009, -0.0032], [0.0034, -0.0036], [0.0209, -0.0025], [0.0171, -0.0008], [-0.0002, 0.0002]], (0.0186, 0.0215), (0.0437, 0.0467)),
        "toe-outer": ([[-0.0009, -0.0032], [0.0035, -0.0036], [0.0231, -0.0024], [0.0192, -0.0007], [-0.0002, 0.0002]], (0.0186, 0.0215), (-0.0053, -0.0023)),
        "hallux": ([[-0.0158, -0.0044], [0.0002, -0.0034], [0.0, 0.0], [-0.0129, -0.0025]], (0.0183, 0.0234), None),
    },
    100: {
        "toes": ([[-0.0157, -0.0044], [0.0343, -0.0018], [0.0287, 0.0], [-0.0007, 0.0002], [-0.0122, -0.0022]], (0.0181, 0.0233), None),
        "toe-spread": ([[0.003, -0.0033], [0.023, -0.0023], [0.0192, -0.0008], [0.0037, 0.0002]], (0.0035, 0.0358), None),
    },
}

for scale, parts in TOES.items():
    for part, (profile, span, tip) in parts.items():
        for side, sign in (("far", 1), ("near", -1)):
            path, shape = doc.shape(scale, "front", f"{side}-{part}", f"toes-{side[0]}")
            flip = (lambda pair: [pair[0], pair[1]]) if sign > 0 else (lambda pair: [-pair[1], -pair[0]])
            solid = {"kind": "extruded", "sweep": "w", "z0": flip(span)[0], "z1": flip(span)[1]}
            if tip is not None:
                solid["zTip"] = flip(tip)
            doc.set(path, {**shape, "points": profile, "solid": solid})

print(doc.save(sys.argv[1] if len(sys.argv) > 1 else None), "edits")
