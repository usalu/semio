"""🛴 R01 repair of humans/child-on-kick-scooter 1:500 (band 1:330…1:1000): the scooter tyres are redrawn and remodelled 72 mm wide (0.2 mm with the 0.13 mm pen at 1:1000; the real 35 mm tread is below the 1:500 floor), front and plan views and both tyre revolves; band restored to 1:1000 (the moved 1:200/1:500 boundary at 1:330 stays, it is contiguous).

@see ./r5-range-gaps-part-h1.md
"""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r5-range-gaps-h1-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
A = "humans/child-on-kick-scooter"


def tyre(solid):
    solid["profile"] = [[-0.036, 0], [-0.036, 0.0725], [0.036, 0.0725], [0.036, 0]]
    return solid


if __name__ == "__main__":
    print(K.LIB.edit_shapes(A, [
        (500, "front", "wheel-front", K.widen(0, 0.072, 0.0)),
        (500, "front", "wheel-rear", K.widen(0, 0.072, 0.0)),
        (500, "plan", "scooter-front-wheel", K.widen(1, 0.072, 0.0)),
    ]))
    print(K.LIB.edit_solids(A, 500, [("body", "tyre-front", tyre), ("body", "tyre-rear", tyre)]))
    print("range was", K.LIB.set_range(A, 500, 330, 1000))
