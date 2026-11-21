"""🧤 R01 repair of humans/parent-carrying-toddler 1:500 (band 1:300…1:1000): the toddler's mittens are redrawn and remodelled 1.2 times larger (about 90 mm, a mitten mass at the 1:500 floor) in every view so the visible part prints 0.2 mm at 1:1000; band restored to 1:1000 (the moved 1:200/1:500 boundary at 1:300 stays, it is contiguous).

@see ./r5-range-gaps-part-h1.md
"""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r5-range-gaps-h1-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
A = "humans/parent-carrying-toddler"

if __name__ == "__main__":
    print(K.LIB.edit_shapes(A, [(500, view, f"toddler-mitten-{side}", K.grow(1.2) if view == "elevation" else K.widen(0 if view == "front" else 1, 0.09)) for view in ("elevation", "front", "plan") for side in "lr"]))
    print(K.LIB.edit_solids(A, 500, [("body", "toddler-mitten-l", K.radii(1.2)), ("body", "toddler-mitten-r", K.radii(1.2))]))
    print("range was", K.LIB.set_range(A, 500, 300, 1000))
