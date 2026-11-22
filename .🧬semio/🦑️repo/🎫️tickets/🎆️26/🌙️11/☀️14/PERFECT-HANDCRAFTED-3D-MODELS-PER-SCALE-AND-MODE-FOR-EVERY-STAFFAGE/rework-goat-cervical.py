"""🦒 Chisel of the goat author: the cervical column of `animals/goat` as the S-curve it is, at every scale.

PATH is the line of the vertebral bodies in the frame of the neck bone (u from the thoracic inlet to the occiput,
v towards the crest): low and forward out of the chest, then rising to the skull. At 1:50 the seven vertebrae read
one by one (articular ends, waisted bodies, the wings of the atlas, the crest of the axis); coarser scales keep the curve.
"""
import json
import math
import sys

SOURCE = "volumes/animals/goat.json"
PATH = [(-0.00005, 0.0), (0.030, -0.022), (0.070, -0.040), (0.120, -0.050), (0.170, -0.050), (0.220, -0.042), (0.270, -0.028), (0.315, -0.012), (0.350, 0.0), (0.36896, 0.0004)]
VERTEBRAE = [("c7", 0.046), ("c6", 0.050), ("c5", 0.054), ("c4", 0.056), ("c3", 0.056), ("axis", 0.070), ("atlas", 0.040)]
END, WAIST = (0.019, 0.021), (0.013, 0.016)
COARSE = {"100": ([0.0, 0.12, 0.26, 0.40, 0.54, 0.68, 0.82, 0.93, 1.0], 0.019, 0.022), "200": ([0.0, 0.2, 0.4, 0.6, 0.8, 1.0], 0.020, 0.024), "500": ([0.0, 0.3, 0.7, 1.0], 0.0378, 0.0378)}


def lengths():
    out = [0.0]
    for a, b in zip(PATH, PATH[1:]):
        out.append(out[-1] + math.hypot(b[0] - a[0], b[1] - a[1]))
    return out


def at(fraction):
    run = lengths()
    s = fraction * run[-1]
    for i in range(len(PATH) - 1):
        if s <= run[i + 1] or i == len(PATH) - 2:
            t = (s - run[i]) / (run[i + 1] - run[i])
            a, b = PATH[i], PATH[i + 1]
            before, after = PATH[max(i - 1, 0)], PATH[min(i + 2, len(PATH) - 1)]
            ta = math.atan2(b[1] - before[1], b[0] - before[0])
            tb = math.atan2(after[1] - a[1], after[0] - a[0])
            return a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, ta + (tb - ta) * t
    raise ValueError(fraction)


def section(fraction, rx, ry):
    u, v, angle = at(fraction)
    s, c, h = math.sin(angle / 2), math.cos(angle / 2), math.sqrt(0.5)
    return {"at": [round(u, 5), round(v, 5), 0], "rx": rx, "ry": ry, "exponent": 2, "rotation": [round(-s * h, 8), round(c * h, 8), round(s * h, 8), round(c * h, 8)]}


def fine():
    total = sum(length for _, length in VERTEBRAE)
    out, run = [section(0.0, 0.021, 0.024)], 0.0
    for name, length in VERTEBRAE:
        a, b = run / total, (run + length) / total
        if name == "atlas":
            out.append(section(a + (b - a) * 0.30, 0.031, 0.015))
            out.append(section(a + (b - a) * 0.70, 0.031, 0.015))
            out.append(section(1.0, 0.016, 0.020))
        elif name == "axis":
            out.append(section(a + (b - a) * 0.25, 0.015, 0.026))
            out.append(section(a + (b - a) * 0.65, 0.014, 0.027))
            out.append(section(b, 0.020, 0.018))
        else:
            out.append(section(a + (b - a) * 0.15, *END))
            out.append(section(a + (b - a) * 0.50, *WAIST))
            out.append(section(a + (b - a) * 0.85, *END))
            out.append(section(b, 0.016, 0.018))
        run += length
    return out


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    for scale in ("50", "100", "200", "500"):
        entry = next(e for e in document["volumes"][scale]["structure"] if e.get("id") == "cervical")
        if scale == "50":
            entry["sections"] = fine()
        else:
            fractions, rx, ry = COARSE[scale]
            entry["sections"] = [section(f, rx, ry) for f in fractions]
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("[DEBUG] cervical column written", file=sys.stderr)


main()
