"""🛏️ Root fix of the pose `lying-resting` of the dairy cow: the near hind hoof hung 0.25 m below the ground the cow lies on, the point of the near hock 4 cm above it and, with the sternum anchor on the real chest floor, the brisket 4 cm above it. The ground of the pose is the plane the sternum and the belly rest on; all four legs are folded onto it, the near hind leg beside the far one. The udder was turned by 85 degrees about its attachment, which drove its rear half 0.15 m into the ground; it now keeps its place between the hind legs (a rigid udder cannot flatten).

Usage: python3 r3-dairy-cow-lying.py [--apply]
"""
import copy
import math
import sys

import r3_dairy_cow_io as io


def frames(bones, pose, root=None):
    by = {b["id"]: b for b in bones}
    out = {}

    def solve(i):
        if i not in out:
            b, d = by[i], pose.get(i, {})
            if b["parent"] is None:
                out[i] = (b["x"], b["y"], b["angleDeg"] + d.get("angleDeg", 0))
            else:
                px, py, pa = solve(b["parent"])
                c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
                out[i] = (px + b["x"] * c - b["y"] * s, py + b["x"] * s + b["y"] * c, pa + b["angleDeg"] + d.get("angleDeg", 0))
        return out[i]

    for b in bones:
        solve(b["id"])
    return out


def heights(asset, pose):
    f = frames(asset["rig"]["bones"], pose)
    out = {}
    for a in asset["anchors"]:
        x, y, deg = f[a["bone"]]
        out[a["id"]] = y + a["x"] * math.sin(math.radians(deg)) + a["y"] * math.cos(math.radians(deg))
    return out


def solve1(asset, pose, bone, anchor, target, lo, hi):
    """📐 The angle of `bone` between lo and hi at which `anchor` stands at the height `target`."""
    def gap(v):
        pose[bone] = {"angleDeg": v}
        return heights(asset, pose)[anchor] - target

    a, b = lo, hi
    if gap(a) * gap(b) > 0:
        raise SystemExit(f"no root for {bone} in {lo}..{hi}: {gap(lo):.3f} {gap(hi):.3f}")
    for _ in range(60):
        mid = (a + b) / 2
        if gap(a) * gap(mid) <= 0:
            b = mid
        else:
            a = mid
    pose[bone] = {"angleDeg": round((a + b) / 2, 1)}


def fixed(asset):
    poses = copy.deepcopy(asset["poses"])
    pose = next(p for p in poses if p["id"] == "lying-resting")["bones"]
    ground = heights(asset, pose)["contact-sternum"]
    solve1(asset, pose, "forearm-fn", "contact-carpus-fn", ground, 40, 110)
    solve1(asset, pose, "forearm-ff", "contact-ff", ground, 30, 100)
    pose["thigh-hn"] = {"angleDeg": 31}
    pose["udder"] = {"angleDeg": 0}
    for side in ("hn", "hf"):
        for _ in range(12):
            solve1(asset, pose, f"shank-{side}", f"contact-hock-{side}", ground + 0.006, -80, -10)
            solve1(asset, pose, f"metatarsus-{side}", f"contact-{side}", ground, 40, 130)
    return poses


def main():
    text, asset = io.read()
    poses = fixed(asset)
    old = next(p for p in asset["poses"] if p["id"] == "lying-resting")
    new = next(p for p in poses if p["id"] == "lying-resting")
    for bone in new["bones"]:
        if old["bones"].get(bone) != new["bones"][bone]:
            print(bone, old["bones"].get(bone), "->", new["bones"][bone])
    for label, p in (("old", old), ("new", new)):
        h = heights(asset, p["bones"])
        print(label, {k: round(h[k], 3) for k in p["contacts"]})
    if "--apply" in sys.argv:
        print("written", io.write(lambda a: {"poses": fixed(a)}))


main()
