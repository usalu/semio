from quad import *
P = dict(
    foot="hoof",
    hind=[(0.22, -62), (0.24, -108), (0.17, -82), (0.06, -52)],
    fore=[(0.18, -62), (0.18, -115), (0.22, -86), (0.17, -90), (0.06, -53)],
    hind_sole_x=0.07, fore_sole_x=0.70,
    sacrum_dx=0.08, thorax_x=0.38, neck_x=0.72,
    neck=(0.30, 30), head=(0.22, -42), jaw=((0.07, -0.04), 0.115, -44), ear=((0.04, 0.05), 0.10, 38),
    tail_root=(-0.03, 0.07), tail=[(0.07, 70), (0.06, 60)],
    lim=dict(pelvis=(-30, 75), lumbar=(-20, 25), thorax=(-15, 15), neck=(-85, 75), head=(-70, 70), jaw=(-30, 2), ear=(-60, 60), tail=(-60, 60),
             thigh=(-70, 70), shank=(-120, 70), metatarsus=(-100, 130), foot=(-70, 60), scapula=(-40, 40), humerus=(-100, 90), forearm=(-45, 135), metacarpus=(-160, 70)),
    titles={},
)
