from quad import *
P = dict(
    foot="hoof",
    hind=[(0.19, -62), (0.19, -108), (0.12, -82), (0.065, -58)],
    fore=[(0.2, -62), (0.16, -115), (0.15, -85), (0.11, -90), (0.065, -60)],
    hind_sole_x=0.07, fore_sole_x=0.66,
    sacrum_dx=0.08, thorax_x=0.38, neck_x=0.66,
    neck=(0.15, 8), head=(0.30, -48), jaw=((0.10, -0.05), 0.2, -50), ear=((0.04, 0.07), 0.17, -10),
    tail_root=(-0.04, 0.05), tail=[(0.08, -85), (0.07, -20)],
    lim=dict(pelvis=(-30, 40), lumbar=(-20, 25), thorax=(-15, 15), neck=(-60, 50), head=(-60, 50), jaw=(-35, 2), ear=(-60, 60), tail=(-70, 70),
             thigh=(-70, 70), shank=(-125, 70), metatarsus=(-100, 175), foot=(-70, 60), scapula=(-40, 40), humerus=(-100, 90), forearm=(-45, 135), metacarpus=(-160, 180)),
    titles={},
)
