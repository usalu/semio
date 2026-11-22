"""🐈 Trunk joints of the domestic cat: every trunk loft ends in a rounded nose that starts at its spinal joint, so a bent spine never shows a flat cap."""
import json, math, sys

SOURCE = "volumes/animals/domestic-cat.json"
NOSE_T = (0.2, 0.4, 0.58, 0.74, 0.87, 0.95, 0.99)
NOSES = {
    "50": {"rump": [("end", 0.0474, 0.058)], "hindbody": [("start", 0.0, 0.055), ("end", 0.1331, 0.060)], "midbody": [("start", 0.0, 0.060), ("end", 0.0677, 0.058)], "chest": [("start", 0.0, 0.056)]},
    "100": {"rump": [("end", 0.0474, 0.058)], "hindbody": [("start", 0.0, 0.055), ("end", 0.1331, 0.060)], "midbody": [("start", 0.0, 0.060), ("end", 0.0677, 0.058)], "chest": [("start", 0.0025, 0.058)]},
    "200": {"hindbody": [("end", 0.147, 0.060)], "chest": [("start", -0.030, 0.062)]},
    "500": {"hindbody": [("end", 0.147, 0.060)], "chest": [("start", -0.031, 0.062)]},
}


def lerp(a, b, u):
    return a + (b - a) * u


def section_at(sections, x):
    """📍 Section of the loft at bone station x, interpolated between its two neighbours."""
    for a, b in zip(sections, sections[1:]):
        if a["at"][0] <= x <= b["at"][0]:
            u = (x - a["at"][0]) / (b["at"][0] - a["at"][0])
            near = a if u < 0.5 else b
            return {"at": [round(lerp(a["at"][i], b["at"][i], u), 5) for i in range(3)], "rx": round(lerp(a["rx"], b["rx"], u), 5), "ry": round(lerp(a["ry"], b["ry"], u), 5), "exponent": round(lerp(a["exponent"], b["exponent"], u), 5), "rotation": near["rotation"]}
    raise SystemExit(f"station {x} outside loft")


def slope(sections, x):
    """📐 Rise of the centre line per metre along the bone around station x."""
    a, b = section_at(sections, max(sections[0]["at"][0], x - 0.02)), section_at(sections, min(sections[-1]["at"][0], x + 0.02))
    return (b["at"][1] - a["at"][1]) / (b["at"][0] - a["at"][0])


def nose(sections, side, x0, length, count):
    """👃 Replace everything beyond the joint station by a blunt half ellipsoid of the given length."""
    base, rise, sign = section_at(sections, x0), slope(sections, x0), 1 if side == "end" else -1
    kept = [s for s in sections if (s["at"][0] < x0 - 0.002 if side == "end" else s["at"][0] > x0 + 0.002)]
    tip = []
    for t in NOSE_T[:: 1 if count >= 7 else 2]:
        f = math.sqrt(1 - t * t) * (1 - 0.12 * t)
        dx = sign * length * t
        tip.append({"at": [round(x0 + dx, 5), round(base["at"][1] + rise * dx * (1 - 0.5 * t), 5), 0], "rx": round(base["rx"] * f, 5), "ry": round(base["ry"] * f, 5), "exponent": round(lerp(base["exponent"], 2, t), 5), "rotation": base["rotation"]})
    return kept + [base] + tip if side == "end" else tip[::-1] + [base] + kept


document = json.load(open(SOURCE))
for scale, lofts in NOSES.items():
    body = {solid["id"]: solid for solid in document["volumes"][scale]["body"]}
    for identifier, ends in lofts.items():
        for side, x0, length in ends:
            body[identifier]["sections"] = nose(body[identifier]["sections"], side, x0, length, 7 if scale in ("50", "100") else 4)
        print(scale, identifier, len(body[identifier]["sections"]), "sections")
json.dump(document, open(SOURCE, "w"), ensure_ascii=False, indent=1)
