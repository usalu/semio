"""🧊 Assemble the plate exchanger with compact core mass markers and legible damper, base rail and bypass opening.

Usage: python3 r5-plate-heat-exchanger-build.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
MASS_RADIUS = {"50": {"mass-plates": 0.015, "mass-frame": 0.015}, "100": {"mass-plates": 0.02, "mass-frame": 0.02}, "200": {"mass-plates": 0.03, "mass-frame": 0.03}}


def main():
    spec = importlib.util.spec_from_file_location("plate_build", os.path.join(HERE, "r3-plate-heat-exchanger-build.py"))
    r3 = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(r3)
    r3.main()
    cand = json.load(open(r3.OUT, encoding="utf-8"))
    vols = cand["asset"]["volumes"]
    for key, radii in MASS_RADIUS.items():
        for s in vols[key]["structure"]:
            if s["id"] in radii:
                s["radii"] = [radii[s["id"]]] * 3
    json.dump(cand, open(r3.OUT, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    json.dump({"volumes": vols}, open(r3.SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)


if __name__ == "__main__":
    main()
