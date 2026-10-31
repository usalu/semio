from fox import *
import fox_shapes as FS

PX = lambda sk, k: hoof_x(sk, k)


def standing_offsets(sk):
    offs = {}
    place_leg(sk, offs, "hf", PX(sk, "hn") + 0.12, 0.0, -80, -48)
    place_leg(sk, offs, "ff", PX(sk, "fn") + 0.12, 0.0, -86, -42)
    return offs


def crouch_offsets(sk, drop=0.07, tilt=-14.0):
    offs = {"pelvis": tilt}
    set_world_angles(sk, offs, {"neck": -5.0, "head": -45.0, "ear-n": 30.0, "ear-f": 30.0, "tail-1": -175.0, "tail-2": -178.0, "tail-3": 180.0})
    root = (sk.hip[0], sk.hip[1] - drop)
    place_leg(sk, offs, "hn", PX(sk, "hn"), 0.0, -75, -45, root=root)
    place_leg(sk, offs, "hf", PX(sk, "hn") + 0.12, 0.0, -70, -45, root=root)
    place_leg(sk, offs, "fn", PX(sk, "fn") + 0.04, 0.0, -80, -40, root=root)
    place_leg(sk, offs, "ff", PX(sk, "fn") + 0.16, 0.0, -75, -40, root=root)
    return offs, root


def curled_offsets(sk, drop=0.239):
    for _ in range(4):
        offs = {"lumbar": 10.0}
        set_world_angles(sk, offs, {"neck": -150.0, "head": -172.0, "ear-n": 200.0, "ear-f": 200.0, "tail-1": -150.0, "tail-2": -25.0, "tail-3": 35.0, "jaw": -172.0})
        root = (sk.hip[0], sk.hip[1] - drop)
        place_leg(sk, offs, "hn", 0.08, 0.0, 2, -48, root=root)
        place_leg(sk, offs, "hf", 0.20, 0.0, 2, -48, root=root)
        place_leg(sk, offs, "fn", 0.40, 0.0, 178, -42, scap=-60, root=root)
        place_leg(sk, offs, "ff", 0.52, 0.0, 178, -42, scap=-60, root=root)
        fr = sk.frames(offs, root)
        sh = FS.shapes50(sk)
        low = min(min(p[1] for p in r) for s_, r in world_rings(sk, sh.items, fr))
        drop += low
    return offs, root


def flight_offsets(sk, y=0.6, x=0.0):
    offs = {"pelvis": -30.0, "lumbar": 6.0}
    set_world_angles(sk, offs, {"neck": -40.0, "head": -62.0, "ear-n": 20.0, "ear-f": 20.0, "tail-1": -190.0, "tail-2": -195.0, "tail-3": -200.0,
                               "scapula-fn": -35.0, "humerus-fn": -70.0, "forearm-fn": -75.0, "metacarpus-fn": -80.0, "paw-fn": -60.0,
                               "scapula-ff": -30.0, "humerus-ff": -62.0, "forearm-ff": -70.0, "metacarpus-ff": -75.0, "paw-ff": -55.0,
                               "thigh-hn": -125.0, "shank-hn": -150.0, "metatarsus-hn": -170.0, "paw-hn": -150.0,
                               "thigh-hf": -120.0, "shank-hf": -145.0, "metatarsus-hf": -165.0, "paw-hf": -145.0})
    return offs, (x, y)


def land_offsets(sk, drop=0.07, tilt=-14.0):
    offs = {"pelvis": tilt}
    set_world_angles(sk, offs, {"neck": -15.0, "head": -45.0, "ear-n": 25.0, "ear-f": 25.0, "tail-1": -185.0, "tail-2": -190.0, "tail-3": -195.0})
    root = (sk.hip[0], sk.hip[1] - drop)
    place_leg(sk, offs, "hn", PX(sk, "hn") - 0.05, 0.0, -80, -45, root=root)
    place_leg(sk, offs, "hf", PX(sk, "hn") + 0.07, 0.0, -75, -45, root=root)
    place_leg(sk, offs, "fn", PX(sk, "fn") + 0.09, 0.0, -75, -40, root=root)
    place_leg(sk, offs, "ff", PX(sk, "fn") + 0.21, 0.0, -70, -40, root=root)
    return offs, root


if __name__ == "__main__":
    sk = make_skel()
    OUT = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-A1/"
    sh = FS.shapes50(sk)
    for name, fn in (("crouch", crouch_offsets), ("curl", curled_offsets), ("flight", flight_offsets), ("land", land_offsets)):
        offs, root = fn(sk)
        for b in sk.bones:
            v = offs.get(b["id"], 0); lo, hi = sk.lim[b["id"]]
            if not lo - 1e-6 <= v <= hi + 1e-6: print("LIMIT", name, b["id"], round(v, 1), lo, hi)
        fr = sk.frames(offs, root)
        low = sorted(((round(min(p[1] for p in r), 3), s["id"]) for s, r in world_rings(sk, sh.items, fr)))[:3]
        print(name, low, [round(v, 3) for v in bounds(world_rings(sk, sh.items, fr))])
        svg_pose(sk, sh.items, fr, OUT + "fox_%s.svg" % name, px_per_m=800, show_anchors=True)
        png(OUT + "fox_%s.svg" % name, OUT + "fox_%s.png" % name)
