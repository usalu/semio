from a_courier_rig import *
from gait import *
for s in "lr":
    bone(rig, f"foot-{s}")["joint"].update(maxDeg=40)
GEN_DIR = GEN
SA = 1.02
GLOVE_OFF = 0.09 * SA


def grip_world(off, root):
    W = Wof(a, off, root)
    return anchor_world(a, W, "handle-grip")


def arms_to_handle(off, root, hand_deg=0.0, spread=(0.0, 0.0), sides="rl"):
    gx, gy = grip_world(off, root)
    th = math.radians(hand_deg)
    tgt_r = (gx - GLOVE_OFF * math.cos(th), gy - GLOVE_OFF * math.sin(th))
    tgt_l = (tgt_r[0] + spread[0], tgt_r[1] + spread[1])
    for s, tg in (("r", tgt_r), ("l", tgt_l)):
        if s not in sides:
            continue
        clipped = solve_chain(a, off, root, f"arm-{s}", f"forearm-{s}", tg, -1)
        assert not clipped
        W = Wof(a, off, root)
        set_world(a, off, W, f"hand-{s}", th)


def static_pose(spec, tilt_off=0.0, arms=True, hand=0.0, sides="rl"):
    sp = dict(spec)
    off0, root, info = make_pose(a, dict(sp, arms={}))
    ypel = root["y"] + HIP
    ex = dict(sp.get("extra", {}))
    ex["truck-lift"] = {"dx": round(-root["y"], 5)}
    ex["hand-truck"] = {"angleDeg": tilt_off}
    sp["extra"] = ex
    off, root, info = make_pose(a, dict(sp), ypelvis=ypel)
    full = {k: dict(v) for k, v in off.items()}
    if arms:
        arms_to_handle(full, root, hand, sides=sides)
    return pose_clean(full), root


SPECS = {
    "waiting": (dict(torso=-3, neck=2, head=0, legs={"r": dict(x=-0.02), "l": dict(x=-0.02)}, support="r", reach=0.99), 0.0),
    "tilted-back": (dict(torso=-1, neck=3, head=2, legs={"r": dict(x=-0.03), "l": dict(x=-0.03)}, support="r", reach=0.985), 9.0),
    "parked-upright": (dict(torso=3, neck=-4, head=-4, arms={"l": dict(angles=(8, 28, -6))}, legs={"r": dict(x=-0.03), "l": dict(x=-0.03)}, support="r", reach=0.99), -10.0),
}
POSES = {k: static_pose(sp, t, arms=True, hand=(-22.0 if k == "tilted-back" else -8.0), sides=("r" if k == "parked-upright" else "rl")) for k, (sp, t) in SPECS.items()}
if __name__ == "__main__":
    for k, (off, root) in POSES.items():
        print(k, joint_issues(a, off), round(root["y"], 4), round(ground_y(a, off, CONTACTS), 4))
        skeleton_png(a, off, root, f"{GEN_DIR}/sk-c-{k}.png", den=40, view=(-0.6, -0.05, 1.5, 2.0))
