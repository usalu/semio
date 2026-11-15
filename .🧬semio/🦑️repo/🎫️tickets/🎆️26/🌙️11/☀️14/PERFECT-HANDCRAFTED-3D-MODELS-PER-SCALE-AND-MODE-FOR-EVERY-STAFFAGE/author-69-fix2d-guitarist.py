"""Root repair seated-guitarist plan 1:50/1:100 (views-disagree z, 0.536 front against 0.518 plan): the rig puts the shoulders at z = +-0.218 and the upper-arm section reaches +-0.051 (hint z0/z1 0.167..0.269, 3D arms +-0.264); the front polygon starts at the joint (u = 0) while the plan path of the upper arm started at u = 0.05 (left) and 0.03 (right) and so missed the first centimetres of the arm that stick out sideways. The plan path now starts at the joint; front and elevation are untouched."""
import sys
sys.path.insert(0, ".")
from author_69_lib import *
def fn(r):
    for sc in ("50", "100"):
        for s in r[sc]["geometry"]["plan"]:
            if s["bone"] in ("upper-arm-l", "upper-arm-r") and "d" in s:
                st = "0.05" if s["bone"].endswith("l") else "0.03"
                tokens = s["d"].split(" ")
                s["d"] = " ".join("0.0" if t == st else t for t in tokens)
edit_member("seated-guitarist", "representations", fn)
