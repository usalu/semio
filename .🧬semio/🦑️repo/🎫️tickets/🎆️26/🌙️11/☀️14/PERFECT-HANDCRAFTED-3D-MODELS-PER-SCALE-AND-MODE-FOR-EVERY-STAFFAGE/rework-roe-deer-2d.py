"""🦌 roe-deer 2D: at 1:500 the head shape stands for the nose leather and the eyes and the hind body for the rump patch (each under 0.25 mm on paper), as the 1:200 head already does.

Usage: python3 rework-roe-deer-2d.py   (re-reads the catalog file, writes `representations`)
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("roe_rig", os.path.join(T, "rework-roe-deer-rig.py"))
R = importlib.util.module_from_spec(spec)
spec.loader.exec_module(R)


def main():
    text = open(R.CATALOG, encoding="utf8").read()
    asset = json.loads(text)["asset"]
    for view in ("elevation", "front", "plan"):
        for shape in asset["representations"]["500"]["geometry"][view]:
            if shape["id"] == "head":
                shape.pop("region", None)
                shape["regions"] = ["coat-head", "nose-skin", "eyes"]
            if shape["id"] == "hindbody":
                shape.pop("region", None)
                shape["regions"] = ["coat-trunk", "rump-patch"]
    open(R.CATALOG, "w", encoding="utf8").write(R.splice(text, "representations", asset["representations"]))


if __name__ == "__main__":
    main()
