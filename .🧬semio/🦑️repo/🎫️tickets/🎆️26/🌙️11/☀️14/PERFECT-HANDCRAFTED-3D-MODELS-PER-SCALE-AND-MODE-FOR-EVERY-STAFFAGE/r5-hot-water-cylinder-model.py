"""🛢️ Round 5 hot-water cylinder: keeps the round-3 hand-stated lists of 1:50 and 1:100 (`r3-hot-water-cylinder-model.py`, re-read and graded in round 5) and redraws what round 5 found wrong at the coarse scales: the plume outlet patch at 1:200 and 1:500 was a 32 / 76 mm terraced crown standing on the cover (now a thin outlet skin on the crown, D2), the 1:500 nipples, outlet and foot pads were 110 / 75 mm cubes (now round stubs and pucks); writes `volumes/appliances/hot-water-cylinder.json`.

Usage: python3 r5-hot-water-cylinder-model.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def _module(name, file):
    """📦 Imports a sibling script as a module."""
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


R3 = _module("r3cylinder", "r3-hot-water-cylinder-model.py")
K = R3.K
TEXTS = os.path.join(HERE, "r5-hot-water-cylinder-texts.json")


def plume(n, half):
    """🌫️ The plume outlet: a skin of 2 x half on the crown of the cover within r 0.2 m, flagged as a surface texture (it is the outlet patch, not a mass)."""
    return K.rev("th-top-plume", "cap", "cap-plastic", "detail", 0.2, R3.plume_patch(n, half), region="top-cap", tags=["top-plume"], texture=True)


def thermal200():
    """🌡️ 1:200 thermal: the round-3 list with the plume outlet as a 4 mm skin on the crown instead of a 32 mm terrace."""
    return [plume(6, 0.002) if s["id"] == "th-top-plume" else s for s in R3.thermal200()]


def body500():
    """🎨 1:500 body: base drum, lower and upper jacket, cover of four facets; the hot-water outlet and the two side nipples as round 110 mm stubs on their real axes (no longer cubes)."""
    out = [s for s in R3.body500() if s["id"] not in ("hot-out", "side-l", "side-r")]
    out.append(K.rev("hot-out", "connections", "brass", "body", 0.7, [(0, 0.055), (0.11, 0.055)], origin=(0.20278, 1.37, 0)))
    out.append(K.rev("side-l", "connections", "brass", "body", 0.7, [(0, 0.055), (0.11, 0.055)], origin=(-0.22, 1.19875, 0), axis=(-1, 0, 0)))
    out.append(K.rev("side-r", "connections", "brass", "body", 0.7, [(0, 0.055), (0.11, 0.055)], origin=(0.22, 0.92375, 0), axis=(1, 0, 0)))
    return out


def thermal500():
    """🌡️ 1:500 thermal: the round-3 list with the plume outlet as a 4 mm skin on the crown instead of a 76 mm terrace."""
    return [plume(4, 0.002) if s["id"] == "th-top-plume" else s for s in R3.thermal500()]


def structure500():
    """🏗️ 1:500 structure: the round-3 list with the levelling-foot contact pads as 75 mm round pucks under the leg ends (no longer cubes)."""
    out = []
    for s in R3.structure500():
        if s["id"] == "support-foot-left":
            fx, fz = K.FEET[0]
            s = K.rev("support-foot-left", "base", "powder-steel", "body", 0.3, [(0, 0.0375), (0.075, 0.0375)], origin=(fx, 0, fz), member="base", part="support", anchor="foot-left")
        elif s["id"] == "support-foot-right":
            fx, fz = K.FEET[1]
            s = K.rev("support-foot-right", "base", "powder-steel", "body", 0.3, [(0, 0.0375), (0.075, 0.0375)], origin=(fx, 0, fz), repeat={"count": 2, "translation": [0, 0, K.r5(-2 * fz)]}, member="base", part="support", anchor="foot-right")
        out.append(s)
    return out


def main():
    """💾 Writes the volumes source."""
    r3t = json.load(open(R3.TEXTS, encoding="utf-8"))["volumes"]
    r5t = json.load(open(TEXTS, encoding="utf-8"))
    vols = {
        "50": {"status": "authored", "denominator": 50, "body": R3.body50(), "thermal": R3.thermal50(), "structure": R3.structure50(), "decisions": r3t["50"]},
        "100": {"status": "authored", "denominator": 100, "body": R3.body100(), "thermal": R3.thermal100(), "structure": R3.structure100(), "decisions": r3t["100"]},
        "200": {"status": "authored", "denominator": 200, "body": R3.body200(), "thermal": thermal200(), "structure": R3.structure200(), "decisions": r5t["200"]},
        "500": {"status": "authored", "denominator": 500, "body": body500(), "thermal": thermal500(), "structure": structure500(), "decisions": r5t["500"]},
    }
    for s, v in vols.items():
        for lst in ("body", "thermal", "structure"):
            ids = [x["id"] for x in v[lst]]
            if len(ids) != len(set(ids)):
                raise SystemExit(f"duplicate ids in {s} {lst}")
    json.dump({"volumes": vols}, open(R3.SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("written", R3.SOURCE, {s: {k: len(v[k]) for k in ("body", "thermal", "structure")} for s, v in vols.items()})


if __name__ == "__main__":
    main()
