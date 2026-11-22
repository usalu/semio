"""🐄 Dairy cow, author 07: the chest between the shoulders re-cut by hand.

The barrel used to taper to a point 0.25 m behind the point of the shoulder, so the shoulder blades stood off a hollow chest and the black chest patch hung free under the neck. A Holstein is 0.44 m wide and 0.62 m deep between the shoulders; the chest ends in a blunt dome at the manubrium of the sternum.
Rows are (u, v, w, a, b): a = vertical half extent, b = lateral half extent, in the frame of the lumbar bone.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib07", os.path.join(HERE, "rework-07-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
SOURCE = os.path.join(HERE, "volumes", "animals", "dairy-cow.json")

CHEST = {
    "50": [(1.13, -0.354, 0, 0.315, 0.228), (1.21, -0.35, 0, 0.305, 0.218), (1.27, -0.345, 0, 0.285, 0.2), (1.32, -0.335, 0, 0.24, 0.165), (1.36, -0.32, 0, 0.17, 0.115), (1.385, -0.31, 0, 0.09, 0.06), (1.395, -0.31, 0, 0.03, 0.02)],
    "100": [(1.16, -0.352, 0, 0.31, 0.222), (1.27, -0.345, 0, 0.285, 0.2), (1.33, -0.332, 0, 0.225, 0.155), (1.375, -0.315, 0, 0.12, 0.08), (1.395, -0.31, 0, 0.04, 0.03)],
}


def main():
    document = lib.load(SOURCE)
    for scale, rows in CHEST.items():
        barrel = lib.solid(document["volumes"][scale]["body"], "barrel")
        template = barrel["sections"][0]
        kept = [section for section in barrel["sections"] if section["at"][0] < rows[0][0] - 1e-6]
        fresh = lib.sections(rows, template["rotation"], template.get("exponent", 2.2))
        if "exponent" not in template:
            for section in fresh:
                del section["exponent"]
        barrel["sections"] = kept + fresh
    lib.save(SOURCE, document)
    print("[DEBUG] dairy cow chest re-cut")


main()
