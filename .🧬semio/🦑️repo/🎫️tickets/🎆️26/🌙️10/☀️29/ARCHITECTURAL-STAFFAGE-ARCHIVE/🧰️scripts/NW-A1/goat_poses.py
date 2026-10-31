from goat import *
import goat_shapes as GS


def standing_offsets(sk):
    offs = {}
    return offs


def upright_offsets(sk, tilt=62.0, drop=0.10, foot_x=None):
    for it in range(4):
        offs = {"pelvis": tilt}
        set_world_angles(sk, offs, {"neck": 78.0, "head": 25.0, "tail-1": 100.0})
        root = (sk.hip[0], sk.hip[1] - drop)
        fx = hoof_x(sk, "hn") if foot_x is None else foot_x
        place_leg(sk, offs, "hn", fx, 0.0, -80, -52, root=root)
        place_leg(sk, offs, "hf", fx - 0.2, 0.0, -88, -52, root=root)
        set_world_angles(sk, offs, {"scapula-fn": 25.0, "humerus-fn": -60.0, "forearm-fn": -20.0, "metacarpus-fn": -70.0, "hoof-fn": -95.0, "scapula-ff": 30.0, "humerus-ff": -55.0, "forearm-ff": -25.0, "metacarpus-ff": -75.0, "hoof-ff": -100.0})
        fr = sk.frames(offs, root)
        cx, cy, m = com_of(sk, fr, None)
        a = sk.anchor_w(fr, "contact-hn")
        foot_x = fx + (cx - a[0] - 0.0)
    return offs, root, (cx, a[0])


def ledge_offsets(sk, ledge=0.30, tilt=22.0):
    offs = {"pelvis": tilt}
    set_world_angles(sk, offs, {"neck": 55.0, "head": -10.0})
    root = (sk.hip[0], sk.hip[1] - 0.02)
    place_leg(sk, offs, "hn", hoof_x(sk, "hn"), 0.0, -86, -52, root=root)
    place_leg(sk, offs, "hf", hoof_x(sk, "hn") - 0.2, 0.0, -96, -52, root=root)
    place_leg(sk, offs, "fn", 0.95, ledge, -80, -20, root=root)
    place_leg(sk, offs, "ff", 0.75, ledge, -60, -20, root=root)
    return offs, root


if __name__ == "__main__":
    sk = make_skel()
    OUT = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-A1/"
    sh = GS.shapes50(sk)
    for name, fn in (("up", upright_offsets), ("ledge", ledge_offsets)):
        r = fn(sk)
        offs, root = r[0], r[1]
        for b in sk.bones:
            v = offs.get(b["id"], 0); lo, hi = sk.lim[b["id"]]
            if not lo - 1e-6 <= v <= hi + 1e-6: print("LIMIT", name, b["id"], round(v, 1), lo, hi)
        fr = sk.frames(offs, root)
        print(name, r[2:] if len(r) > 2 else "", [round(v, 3) for v in bounds(world_rings(sk, sh.items, fr))])
        svg_pose(sk, sh.items, fr, OUT + "goat_%s.svg" % name, px_per_m=500, show_anchors=True)
        png(OUT + "goat_%s.svg" % name, OUT + "goat_%s.png" % name)
