from pig import *
import pig_shapes as PS


def standing_offsets(sk):
    offs = {}
    place_leg(sk, offs, "hf", hoof_x(sk, "hn") + 0.2, 0.0, -80, -58)
    place_leg(sk, offs, "ff", hoof_x(sk, "fn") + 0.2, 0.0, -82, -60)
    return offs


def rooting_offsets(sk, drop=0.08, tilt=-10.0):
    offs = {"pelvis": tilt}
    set_world_angles(sk, offs, {"neck": -35.0, "head": -75.0})
    root = (sk.hip[0], sk.hip[1] - drop)
    for key, x, ac, ah in (("hn", hoof_x(sk, "hn"), -88, -58), ("hf", hoof_x(sk, "hn") + 0.2, -80, -58), ("fn", hoof_x(sk, "fn") + 0.04, -90, -60), ("ff", hoof_x(sk, "fn") - 0.2, -100, -60)):
        place_leg(sk, offs, key, x, 0.0, ac, ah, root=root)
    return offs, root


def lying_offsets(sk, drop=0.205):
    offs = {"neck": 14.0, "head": 14.0}
    root = (sk.hip[0], sk.hip[1] - drop)
    place_leg(sk, offs, "hn", 0.12, 0.12, 2, -4, root=root)
    place_leg(sk, offs, "hf", 0.12 + 0.2, 0.12, 2, -4, root=root)
    place_leg(sk, offs, "fn", 0.46, 0.15, 178, 170, scap=-62, root=root)
    place_leg(sk, offs, "ff", 0.46 + 0.2, 0.15, 178, 170, scap=-62, root=root)
    return offs, root


if __name__ == "__main__":
    sk = make_skel()
    OUT = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-A1/"
    sh = PS.shapes50(sk)
    for name, fn in (("root", rooting_offsets), ("lie", lying_offsets)):
        offs, root = fn(sk)
        for b in sk.bones:
            v = offs.get(b["id"], 0); lo, hi = sk.lim[b["id"]]
            if not lo - 1e-6 <= v <= hi + 1e-6: print("LIMIT", name, b["id"], round(v, 1), lo, hi)
        fr = sk.frames(offs, root)
        low = sorted(((round(min(p[1] for p in r), 3), s["id"]) for s, r in world_rings(sk, sh.items, fr)))[:4]
        print(name, low, sk.anchor_w(fr, "nose"), [round(v, 3) for v in bounds(world_rings(sk, sh.items, fr))])
        svg_pose(sk, sh.items, fr, OUT + "pig_%s.svg" % name, px_per_m=700, show_anchors=True)
        png(OUT + "pig_%s.svg" % name, OUT + "pig_%s.png" % name)
