"""💧 Assemble the humidifier with compact pad and casing mass markers and legible coarse-scale water levels.

Usage: python3 r5-evaporative-humidifier-build.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def load(name):
    """📥 Imports a sibling script by file name."""
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, name))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


MASS_RADIUS = {"50": {"mass-pad": 0.01, "mass-casing": 0.01}, "100": {"mass-casing": 0.02}, "200": {"mass-casing": 0.03}}


def main():
    r3 = load("r3-evaporative-humidifier-build.py")
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
