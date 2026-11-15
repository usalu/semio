"""seated-guitarist stool: areaM2 0.55 -> 0.645 (C16/C29: non-body surface region, within a factor 1.5; the 1:50 stool of four splayed round legs, a four-rung ring and a seat disc has an outer area of 0.646 m2, 1:100 0.633)."""
import sys
sys.path.insert(0, ".")
from author_69_lib import *
def fn(t):
    r = next(x for x in t["regions"] if x["id"] == "stool")
    assert r["areaM2"] == 0.55, r["areaM2"]
    r["areaM2"] = 0.645
edit_member("seated-guitarist", "thermal", fn)
