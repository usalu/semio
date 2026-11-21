"""🪟 Round 5 root fix of components/tied-canopy, 1:100 plan: the four panes meet on the joint centre lines under the drawn EPDM joint strips, so no 0.1 mm white sliver remains between pane outlines (the product refused 1:100 for it).

Usage: python3 r5-tied-canopy-roots.py [--apply]

@see ./r3-components-tied-canopy.md
@see ./r5_c03_lib.py"""
import sys

import r5_c03_lib as L

HALF_JOINT = 0.01


def fix(asset):
    """🧩 Extend each 1:100 plan pane by half a joint towards its neighbours."""
    plan = asset["representations"]["100"]["geometry"]["plan"]
    panes = {s["id"]: s for s in plan if s["id"] in ("glass-0", "glass-1", "glass-2", "glass-3")}
    joints = sorted(s["x"] + s["w"] / 2 for s in plan if s["id"] in ("joint-0", "joint-1", "joint-2"))
    edges = [None, *joints, None]
    for i in range(4):
        s = panes[f"glass-{i}"]
        lo, hi = edges[i], edges[i + 1]
        if s["kind"] == "rect":
            x0, x1 = s["x"], s["x"] + s["w"]
            x0, x1 = (lo if lo is not None else x0), (hi if hi is not None else x1)
            s["x"], s["w"] = round(x0, 6), round(x1 - x0, 6)
        else:
            xs = sorted({p[0] for p in s["points"]})
            left, right = xs[0], xs[-1]
            s["points"] = [[round(lo if p[0] == left and lo is not None else hi if p[0] == right and hi is not None else p[0], 6), p[1]] for p in s["points"]]


def main():
    """🚀 Candidate, validate, splice."""
    doc = L.load("tied-canopy")
    fix(doc["asset"])
    ok, _ = L.candidate("tied-canopy", doc)
    if "--apply" in sys.argv:
        if not ok:
            sys.exit("candidate invalid, not written")
        fresh = L.load("tied-canopy")
        fix(fresh["asset"])
        L.splice("tied-canopy", {"representations": fresh["asset"]["representations"]})


if __name__ == "__main__":
    main()
