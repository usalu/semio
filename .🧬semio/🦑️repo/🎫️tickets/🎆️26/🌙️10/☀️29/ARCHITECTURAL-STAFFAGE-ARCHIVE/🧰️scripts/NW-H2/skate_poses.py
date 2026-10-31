"""Skateboarder poses (positions relative to the unrotated board origin = deck underside centre)."""
from skate_rig import *

bones, L = build_rig()
AY = DECK_TOP + L["ankle"]
GROUND = -BOARD_Y


def kick(x):
    d = max(0.0, (abs(x) - 0.28) / 0.125)
    return 0.075 * d ** 1.6


def rotp(p, th):
    c, s = rot(th)
    return (c * p[0] - s * p[1], s * p[0] + c * p[1])


def lowest_y(f, ids, anchors):
    ys = []
    for a in anchors:
        if a["id"] in ids:
            ys.append(f.at(a["bone"], (a["x"], a["y"]))[1])
    return min(ys)


def poses():
    P = {}
    P["cruising"] = make_pose(bones, L, pelvis=(-0.01, 0.84), torso_deg=80, head_deg=88, front=(0.12, AY), rear=(-0.15, AY), arm_near=(0.52, 0.98), arm_far=(-0.48, 1.00), hand=(0, 0))
    P["pushing"] = make_pose(bones, L, pelvis=(0.04, 0.80), torso_deg=64, head_deg=78, front=(0.15, AY), rear=ankle_for_ball((-0.38, GROUND), -42, L), arm_near=(0.48, 0.70), arm_far=(-0.20, 0.66), rear_foot=-42, hand=(0, 0))
    P["push-plant"] = make_pose(bones, L, pelvis=(0.0, 0.82), torso_deg=70, head_deg=82, front=(0.15, AY), rear=ankle_for_ball((-0.14, GROUND), -14, L), arm_near=(0.44, 0.76), arm_far=(-0.28, 0.76), rear_foot=-14)
    P["push-lift"] = make_pose(bones, L, pelvis=(-0.02, 0.76), torso_deg=62, head_deg=76, front=(0.15, AY), rear=(-0.44, 0.06), arm_near=(0.46, 0.68), arm_far=(-0.24, 0.62), rear_foot=-55)
    P["push-swing"] = make_pose(bones, L, pelvis=(-0.02, 0.80), torso_deg=72, head_deg=84, front=(0.15, AY), rear=(-0.22, 0.22), arm_near=(0.40, 0.78), arm_far=(-0.30, 0.78), rear_foot=-25)
    P["push-reach"] = make_pose(bones, L, pelvis=(-0.08, 0.84), torso_deg=74, head_deg=86, front=(0.15, AY), rear=(-0.15, 0.26), arm_near=(0.40, 0.78), arm_far=(-0.30, 0.80), rear_foot=-10)
    P["crouch"] = make_pose(bones, L, pelvis=(-0.13, 0.73), torso_deg=60, head_deg=76, front=(0.13, AY), rear=(-0.17, AY), arm_near=(0.34, 0.52), arm_far=(-0.12, 0.46), hand=(0, 0))
    th = 38
    rear_b = (-0.30, 0.012 + L["ankle"] + kick(-0.30))
    front_b = (0.12, AY)
    P["pop"] = make_pose(bones, L, pelvis=(-0.03, 0.74), torso_deg=82, head_deg=90, front=rotp(front_b, th), rear=rotp(rear_b, th), arm_near=(0.30, 0.94), arm_far=(-0.22, 0.84), theta=th, front_foot=th - 24, rear_foot=th)
    P["ollie-air"] = make_pose(bones, L, pelvis=(-0.10, 0.74), torso_deg=82, head_deg=88, front=rotp((0.22, AY), 6), rear=rotp((-0.24, AY), 6), arm_near=(0.46, 0.90), arm_far=(-0.26, 0.84), theta=6, front_foot=6, rear_foot=6)
    P["landing"] = make_pose(bones, L, pelvis=(-0.11, 0.76), torso_deg=66, head_deg=80, front=(0.14, AY), rear=(-0.17, AY), arm_near=(0.40, 0.66), arm_far=(-0.30, 0.64))
    return P


if __name__ == "__main__":
    P = poses()
    names = list(P)
    for n in names:
        print(n, joint_report(P[n]))
    render(skeleton_svg([P[n] for n in names], names, 25, x_range=(-1, 1), y_range=(-0.15, 1.95)), "skate-skel", 2400)
