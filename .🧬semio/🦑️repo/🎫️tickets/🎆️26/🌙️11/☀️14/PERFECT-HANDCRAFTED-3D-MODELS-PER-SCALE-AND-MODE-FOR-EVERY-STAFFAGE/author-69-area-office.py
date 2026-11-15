"""seated-office-worker desk: areaM2 1.1 -> 1.35 (C16/C29: non-body surface region, factor 1.23 of the committed value, within 1.5). Hand calculation: board 0.78 x 0.6 x 0.025 both faces and edges 1.01, apron 0.1, two posts 0.23, foot plates 0.04 = 1.38; the 1:50 model measures 1.346."""
import sys
sys.path.insert(0, ".")
from author_69_lib import *
def fn(t):
    r = next(x for x in t["regions"] if x["id"] == "desk")
    assert r["areaM2"] == 1.1, r["areaM2"]
    r["areaM2"] = 1.35
edit_member("seated-office-worker", "thermal", fn)
