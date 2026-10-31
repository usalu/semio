from a_phone_rig import *
GEN_DIR = GEN


def hipx():
    return rig["bones"][0]["x"]


def mk(spec, **kw):
    off, root, info = make_pose(a, spec, **kw)
    assert not any(info["clipped"]), info
    return off, root


def ear_target(spec):
    s2 = dict(spec); s2["arms"] = {}
    off, root, _ = make_pose(a, s2)
    W = Wof(a, off, root)
    e = apply(W["head"], (0.045, 0.0))
    return e[0] - hipx() - root["x"], e[1]


SPEC = {}
SPEC["texting"] = dict(torso=-4, neck=-16, head=-24, legs={"r": dict(x=-0.01), "l": dict(x=-0.01)}, support="r", reach=0.985,
                       arms={"l": dict(wrist=(0.275, 1.125), hand=12), "r": dict(wrist=(0.275, 1.125), hand=12)})
SPEC["call"] = dict(torso=0, neck=-2, head=4, legs={"r": dict(x=0.0), "l": dict(x=0.0)}, support="r", reach=0.975,
                    arms={"l": dict(wrist=(0.17, 1.02), hand=-10)})
ex, ey = ear_target(SPEC["call"])
SPEC["call"]["arms"]["r"] = dict(wrist=(ex + 0.15, ey - 0.19), hand=88)
SPEC["photo"] = dict(torso=1, neck=0, head=2, legs={"r": dict(x=-0.02), "l": dict(x=-0.02)}, support="r", reach=0.985,
                     arms={"l": dict(wrist=(0.46, 1.49), hand=66), "r": dict(wrist=(0.46, 1.49), hand=66)})
POSES = {}
for k, sp in SPEC.items():
    POSES[k] = mk(sp)
if __name__ == "__main__":
    for k, (off, root) in POSES.items():
        print(k, joint_issues(a, off), round(root["y"], 4), round(ground_y(a, off, CONTACTS), 4))
        skeleton_png(a, off, root, f"{GEN_DIR}/sk-{k}.png", den=40, view=(-0.6, -0.05, 0.9, 1.9))
