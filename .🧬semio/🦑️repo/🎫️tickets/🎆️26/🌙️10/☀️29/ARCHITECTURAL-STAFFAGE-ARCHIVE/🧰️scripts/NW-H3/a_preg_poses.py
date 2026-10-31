from a_preg_rig import *
from gait import *
GEN_DIR = GEN


def torso_pt(off, root, s, f):
    """world point on the torso at height s along the bone and f forward (negative = back)"""
    W = Wof(a, off, root)
    return apply(W["torso"], (s, -f))


def hands_to(off, root, targets, hand_world):
    """targets {side: ('torso', s, f) | ('world', x, y)}"""
    for sd, tg in targets.items():
        if tg[0] == "torso":
            p = torso_pt(off, root, tg[1], tg[2])
        else:
            p = (tg[1], tg[2])
        th = math.radians(hand_world[sd])
        wrist = (p[0] - 0.08 * SA * math.cos(th) * 0, p[1] - 0.0)
        clipped = solve_chain(a, off, root, f"arm-{sd}", f"forearm-{sd}", wrist, -1)
        assert not clipped, (sd, tg)
        W = Wof(a, off, root)
        set_world(a, off, W, f"hand-{sd}", th)


def spec_pose(spec, targets, hand_world):
    off, root, info = make_pose(a, dict(spec, arms={}))
    full = {k: dict(v) for k, v in off.items()}
    hands_to(full, root, targets, hand_world)
    return pose_clean(full), root


SP = {
    "hand-on-back": (dict(pelvis=-3, torso=9, neck=-7, head=4, legs={"r": dict(x=-0.02), "l": dict(x=-0.02)}, support="r", reach=0.985),
                     {"r": ("torso", 0.07, -0.085), "l": ("world", 0.215, 1.02)}, {"r": -78, "l": -30}),
    "cradling-belly": (dict(pelvis=-2, torso=6, neck=-10, head=-2, legs={"r": dict(x=-0.02), "l": dict(x=-0.02)}, support="r", reach=0.99),
                       {"r": ("world", 0.225, 1.02), "l": ("world", 0.225, 1.02)}, {"r": -50, "l": -50}),
    "stretching-back": (dict(pelvis=-4, torso=12, neck=-2, head=8, legs={"r": dict(x=-0.02), "l": dict(x=-0.02)}, support="r", reach=0.975),
                        {"r": ("torso", 0.085, -0.06), "l": ("torso", 0.085, -0.06)}, {"r": -80, "l": -80}),
}
POSES = {k: spec_pose(*v) for k, v in SP.items()}
if __name__ == "__main__":
    for k, (off, root) in POSES.items():
        print(k, joint_issues(a, off), round(root["y"], 4), round(ground_y(a, off, CONTACTS), 4))
        skeleton_png(a, off, root, f"{GEN_DIR}/sk-p-{k}.png", den=40, view=(-0.6, -0.05, 0.8, 1.9))
