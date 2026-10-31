"""Shared anatomical drawing vocabulary of NW-H2 (head profile, mitten hand, foot stations). Pure coordinate functions in bone-local metres;
every asset script picks, scales, extends and clothes these forms by hand."""
from lib import *

# head + neck, bone-local (x up from the neck base, -y = face side); profile of an adult at stature 1.82 m
HEAD = [
    (0.000, 0.052), (0.045, 0.050), (0.095, 0.066), (0.150, 0.092), (0.215, 0.090), (0.258, 0.060), (0.279, 0.010),
    (0.268, -0.045), (0.240, -0.080), (0.210, -0.087), (0.186, -0.090), (0.168, -0.098), (0.152, -0.109),
    (0.139, -0.096), (0.128, -0.092), (0.118, -0.090), (0.108, -0.088), (0.084, -0.090), (0.068, -0.074),
    (0.060, -0.044), (0.036, -0.050), (0.000, -0.050),
]
HEAD_NECK_BACK = [(0.0, 0.052), (0.045, 0.050)]
HEAD_LM = {"nose": (0.152, -0.109), "chin": (0.070, -0.080), "eye": (0.186, -0.080), "brow": (0.202, -0.088), "ear": (0.150, 0.014), "mouth": (0.118, -0.090), "crown": (0.279, 0.0), "skull-back": (0.150, 0.092)}


def head(f=1.0, keep=None):
    return [(p[0] * f, p[1] * f) for p in HEAD]


def lm(name, f=1.0):
    p = HEAD_LM[name]
    return (p[0] * f, p[1] * f)


def mitten_st(f=1.0, grip=1.0):
    """Hand stations (s, +y width, -y width) for a hand bone of length 0.1861*f: wrist to fingertip."""
    return [(0.0 * f, 0.030 * f, 0.028 * f), (0.035 * f, 0.036 * f, 0.033 * f), (0.085 * f, 0.038 * f, 0.031 * f), (0.130 * f, 0.033 * f, 0.026 * f), (0.172 * f, 0.021 * f, 0.017 * f)]


def thumb(f=1.0, side=1):
    return capsule(0.040 * f, side * 0.040 * f, 0.105 * f, side * 0.052 * f, 0.012 * f, 0.009 * f)


def shoe_pts(f, h_collar=0.035, toe_h=0.060, heel_back=-0.058, toe_tip=0.236, sole=0.022, ankle=0.0808):
    """Shoe outline (foot-bone local, y up, sole at -ankle): returns (upper_pts, sole_pts)."""
    a = ankle * f
    up = [(heel_back * f, -a + sole * f, "c"), (heel_back * f - 0.002, -0.010 * f), (heel_back * f + 0.004, h_collar * f), (0.020 * f, h_collar * f + 0.004), (0.060 * f, 0.018 * f), (0.110 * f, -0.010 * f), (0.170 * f, -a + toe_h * f * 1.0), (toe_tip * f - 0.012, -a + 0.040 * f), (toe_tip * f, -a + sole * f, "c")]
    so = [(heel_back * f, -a, "c"), (heel_back * f, -a + sole * f, "c"), (toe_tip * f, -a + sole * f, "c"), (toe_tip * f + 0.004, -a + 0.012 * f), (toe_tip * f - 0.004, -a, "c")]
    return up, so
