"""🦵 Stage 3 of the seated reader: the 1:500 thigh above the knee.

In the plan the left thigh shows only as the strip between the forearm and the notebook; with a thigh of 134 mm above the knee that strip was 97 mm, 0.194 mm on paper, below the 0.2 mm an office laser prints, so the product refused to draw 1:500 at all. Hand-chosen radii (metres from the axis): 77 mm at 0.30 m from the hip joint (girth 0.48 m in jeans), 67 mm at the knee (0.43 m), 56 mm at the rounded end (0.47 m), in the solid and in the profile of the three drawings.

Run: `python3 rework-seated-reader-3-thigh500.py --apply` writes the volumes source and `representations` of the catalog.
"""
import copy
import importlib.util
import json
import os
import sys

from rework_03_lib import T, catalog_path, source_path

spec = importlib.util.spec_from_file_location("stage1", os.path.join(T, "rework-seated-reader-1-rig.py"))
stage1 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(stage1)

A = "humans/seated-reader"
AXIS = 0.0032
PROFILE = {0.3: 0.077, 0.43: 0.067, 0.47: 0.056}


def main():
    """🚀 Restates the thigh of 1:500 in the solid and in the drawings."""
    catalog = catalog_path(A)
    text = open(catalog, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"]["representations"])
    source = json.load(open(source_path(A), encoding="utf8"))
    for shapes in document["asset"]["representations"]["500"]["geometry"].values():
        for shape in shapes:
            if shape["id"] in ("thigh-l", "thigh-r"):
                for point in shape["points"]:
                    if point[0] in PROFILE and point[1] > 0.01:
                        point[1] = round(PROFILE[point[0]] + AXIS, 4)
    for solid in source["volumes"]["500"]["body"]:
        if solid["id"] in ("thigh-l", "thigh-r"):
            for section in solid["sections"]:
                if section["at"][0] in PROFILE:
                    section["rx"] = section["ry"] = PROFILE[section["at"][0]]
    if "--apply" in sys.argv:
        open(catalog, "w", encoding="utf8").write(stage1.spliced(text, "representations", before, document["asset"]["representations"]))
        json.dump(source, open(source_path(A), "w", encoding="utf8"), ensure_ascii=False, indent=1)
        open(source_path(A), "a", encoding="utf8").write("\n")
        print("applied")


if __name__ == "__main__":
    main()
