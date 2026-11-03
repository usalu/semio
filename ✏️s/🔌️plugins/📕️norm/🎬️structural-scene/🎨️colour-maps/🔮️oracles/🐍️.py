#!/usr/bin/env python3
"""🎨️ Independent oracle of the colour maps: ramp interpolation, WCAG contrast and CVD distinguishability from the published CIE/WCAG/Machado formulas; cross-checked against colorspacious when it is installed. Reads one JSON case from stdin, writes one JSON answer. Usage: python3 -I 🐍️.py < case.json"""
import json
import math
import sys

MACHADO = {
    "prot": [[0.152286, 1.052583, -0.204868], [0.114503, 0.786281, 0.099216], [-0.003882, -0.048116, 1.051998]],
    "deut": [[0.367322, 0.860646, -0.227968], [0.280085, 0.672501, 0.047413], [-0.011820, 0.042940, 0.968881]],
    "trit": [[1.255528, -0.076749, -0.178779], [-0.078411, 0.930809, 0.147602], [0.004733, 0.691367, 0.303900]],
}


def lin(c):
    c /= 255
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def enc(v):
    return 255 * (12.92 * v if v <= 0.0031308 else 1.055 * v ** (1 / 2.4) - 0.055)


def lum(c):
    r, g, b = (lin(x) for x in c)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def contrast(a, b):
    hi, lo = sorted((lum(a) + 0.05, lum(b) + 0.05), reverse=True)
    return hi / lo


def lab(c):
    r, g, b = (lin(x) for x in c)
    x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b
    y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b
    z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b
    f = lambda t: t ** (1 / 3) if t > 216 / 24389 else (24389 / 27 * t + 16) / 116
    fx, fy, fz = f(x / 0.95047), f(y), f(z / 1.08883)
    return 116 * fy - 16, 500 * (fx - fy), 200 * (fy - fz)


def delta_e(a, b):
    return math.dist(lab(a), lab(b))


def simulate(kind, c):
    l = [lin(x) for x in c]
    m = MACHADO[kind]
    return [enc(min(1.0, max(0.0, sum(m[i][j] * l[j] for j in range(3))))) for i in range(3)]


def sample(stops, t):
    t = min(1.0, max(0.0, t))
    for (t0, c0), (t1, c1) in zip(stops, stops[1:]):
        if t <= t1:
            u = min(1.0, max(0.0, (t - t0) / (t1 - t0)))
            return [math.floor(a + (b - a) * u + 0.5) for a, b in zip(c0, c1)]
    return list(stops[-1][1])


def cross_check(a, b, kind):
    try:
        import numpy as np
        from colorspacious import cspace_convert, deltaE
    except ImportError:
        return None
    seen = lambda c: np.clip(cspace_convert(c, {"name": "sRGB1+CVD", "cvd_type": {"prot": "protanomaly", "deut": "deuteranomaly", "trit": "tritanomaly"}[kind], "severity": 100}, "sRGB255"), 0, 255)
    return float(deltaE(seen(a), seen(b), input_space="sRGB255"))


# CIEDE2000 after Sharma, Wu and Dalal (2005), written independently of the Rust implementation.
def delta_e2000(l1, l2):
    (L1, a1, b1), (L2, a2, b2) = l1, l2
    cb = (math.hypot(a1, b1) + math.hypot(a2, b2)) / 2
    g = 0.5 * (1 - math.sqrt(cb**7 / (cb**7 + 25**7)))
    ap1, ap2 = (1 + g) * a1, (1 + g) * a2
    cp1, cp2 = math.hypot(ap1, b1), math.hypot(ap2, b2)
    hue = lambda b, a: 0.0 if a == 0 and b == 0 else math.degrees(math.atan2(b, a)) % 360
    hp1, hp2 = hue(b1, ap1), hue(b2, ap2)
    dl, dc = L2 - L1, cp2 - cp1
    dh = 0.0 if cp1 * cp2 == 0 else (hp2 - hp1 - 360 if hp2 - hp1 > 180 else hp2 - hp1 + 360 if hp2 - hp1 < -180 else hp2 - hp1)
    dhh = 2 * math.sqrt(cp1 * cp2) * math.sin(math.radians(dh / 2))
    lb, cpb = (L1 + L2) / 2, (cp1 + cp2) / 2
    if cp1 * cp2 == 0:
        hb = hp1 + hp2
    elif abs(hp1 - hp2) <= 180:
        hb = (hp1 + hp2) / 2
    else:
        hb = (hp1 + hp2 + 360) / 2 if hp1 + hp2 < 360 else (hp1 + hp2 - 360) / 2
    rad = math.radians
    t = 1 - 0.17 * math.cos(rad(hb - 30)) + 0.24 * math.cos(rad(2 * hb)) + 0.32 * math.cos(rad(3 * hb + 6)) - 0.20 * math.cos(rad(4 * hb - 63))
    rc = 2 * math.sqrt(cpb**7 / (cpb**7 + 25**7))
    rt = -math.sin(rad(60 * math.exp(-(((hb - 275) / 25) ** 2)))) * rc
    sl = 1 + 0.015 * (lb - 50) ** 2 / math.sqrt(20 + (lb - 50) ** 2)
    sc, sh = 1 + 0.045 * cpb, 1 + 0.015 * cpb * t
    return math.sqrt((dl / sl) ** 2 + (dc / sc) ** 2 + (dhh / sh) ** 2 + rt * (dc / sc) * (dhh / sh))


def min_pairwise_2000(kind, colours):
    seen = [lab(simulate(kind, c)) for c in colours]
    return min(delta_e2000(a, b) for i, a in enumerate(seen) for b in seen[i + 1:])


def main():
    case = json.load(sys.stdin)
    out = {"ramps": [], "pairs": [], "white_black": contrast([255, 255, 255], [0, 0, 0])}
    for r in case["ramps"]:
        pts = [sample(r["stops"], i / (r["n"] - 1)) for i in range(r["n"])]
        entry = {"samples": pts, "contrast": [contrast(p, r["bg"]) for p in pts], "adjacent": {}, "ends": {}}
        for kind in MACHADO:
            seen = [simulate(kind, p) for p in pts]
            entry["adjacent"][kind] = min(delta_e(a, b) for a, b in zip(seen, seen[1:]))
            entry["ends"][kind] = delta_e(seen[0], seen[-1])
        out["ramps"].append(entry)
    for a, b in case["pairs"]:
        out["pairs"].append({k: delta_e(simulate(k, a), simulate(k, b)) for k in MACHADO})
    out["ciede2000"] = [delta_e2000(a, b) for a, b in case.get("lab_pairs", [])]
    out["palettes"] = [{k: min_pairwise_2000(k, p) for k in MACHADO} for p in case.get("palettes", [])]
    out["colorspacious"] = cross_check(case["pairs"][0][0], case["pairs"][0][1], "deut") if case["pairs"] else None
    json.dump(out, sys.stdout)


if __name__ == "__main__":
    main()
