"""🦯 R01 repair of humans/elderly-with-cane 1:500 (band 1:350…1:1000): the cane strip and the left hand are redrawn 75–80 mm across (0.2 mm with the 0.13 mm pen at 1:1000), the hand mass remodelled to match; the slender cane solid keeps its true 22 mm (C24); band restored to 1:1000.

@see ./r5-range-gaps-part-h1.md
"""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r5-range-gaps-h1-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
A = "humans/elderly-with-cane"

if __name__ == "__main__":
    print(K.LIB.edit_shapes(A, [
        (500, "elevation", "cane", K.widen(1, 0.076, 0.0)),
        (500, "front", "cane", K.widen(0, 0.076)),
        (500, "plan", "cane", K.widen(1, 0.076)),
        (500, "front", "hand-l", K.widen(0, 0.08)),
        (500, "plan", "hand-l", K.widen(1, 0.08)),
    ]))
    print(K.LIB.edit_solids(A, 500, [("body", "hand-l", K.sections(floor_rx=0.04))]))
    print("range was", K.LIB.set_range(A, 500, 350, 1000))
