from quad import *
P = dict(
    foot="paw",
    hind=[(0.13, -62), (0.14, -110), (0.11, -80), (0.07, -48)],
    fore=[(0.11, -60), (0.11, -118), (0.12, -85), (0.08, -90), (0.06, -42)],
    hind_sole_x=0.06, fore_sole_x=0.47,
    sacrum_dx=0.05, thorax_x=0.25, neck_x=0.45,
    neck=(0.14, 38), head=(0.16, -32), jaw=((0.05, -0.03), 0.085, -35), ear=((0.03, 0.04), 0.09, 78),
    tail_root=(-0.03, 0.045), tail=[(0.15, -165), (0.14, -172), (0.12, -178)],
    lim=dict(pelvis=(-40, 60), lumbar=(-40, 45), thorax=(-25, 25), neck=(-100, 170), head=(-90, 80), jaw=(-40, 6), ear=(-110, 60), tail=(-90, 140),
             thigh=(-80, 75), shank=(-130, 75), metatarsus=(-110, 175), foot=(-100, 100), scapula=(-60, 60), humerus=(-110, 95), forearm=(-50, 140), metacarpus=(-185, 180)),
    titles={},
)
