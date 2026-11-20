"""🖊 The hand-drawn inner lines of the 2D drawings of the standing adult (seams, hems, laces, folds, face strokes), stated per scale and view at the landmarks of that scale's body in the world of the rest pose (x ahead, y up, z left).

A stroke is (id, bone, role, z order, tone, region, points); points are (x, y, z). `r3-standing-adult-2d.ts` turns them into shapes: bone-local in the side elevation, (z, y) in the front elevation, (x, z) in the plan.
"""
import r3_standing_adult_body50 as b50

HIP, SH = 0.085, 0.153
LEFT, RIGHT = ("l", 1, 1.0, 0.5), ("r", -1, 3.0, 6.0)


def sides(make):
    out = []
    for side, sign, leg, arm in (LEFT, RIGHT):
        out += make(side, sign, leg, arm)
    return out


def scale50():
    def elevation(side, sign, leg, arm):
        out = [
            ("jeans-side-seam-" + side, "thigh-" + side, "detail", leg + 0.03, 0.1, "jeans", [(-0.006, 0.955, sign * HIP), (-0.004, 0.800, sign * HIP), (0.002, 0.600, sign * HIP), (0.004, 0.470, sign * HIP)]),
            ("jeans-knee-" + side, "thigh-" + side, "texture", leg + 0.03, 0.1, "jeans", [(0.030, 0.538, sign * HIP), (0.050, 0.508, sign * HIP), (0.036, 0.470, sign * HIP)]),
            ("jeans-hem-" + side, "shank-" + side, "detail", leg + 0.03, 0.1, "jeans", [(-0.052, 0.127, sign * HIP), (0.044, 0.127, sign * HIP)]),
        ]
        for k, lx in enumerate((0.052, 0.074, 0.096)):
            top = 0.092 - (lx - 0.030) * 0.44
            out.append(("lace-%s-%d" % (side, k), "foot-" + side, "texture", leg + 0.2, 0.85, "sneakers", [(lx - 0.005, top + 0.003, sign * HIP), (lx + 0.006, top - 0.014, sign * HIP)]))
        for k in range(3):
            a, b = b50.finger_points(k), b50.finger_points(k + 1)
            mid = lambda p, q: ((p[0] + q[0]) / 2, (p[1] + q[1]) / 2, sign * (p[2] + q[2]) / 2)
            out.append(("finger-%s-%d" % (side, k), "hand-" + side, "texture", arm + 0.25, 0.1, "hand-skin", [mid(a[0], b[0]), mid(a[1], b[1]), mid((a[2][0], a[2][1] + 0.008, a[2][2]), (b[2][0], b[2][1] + 0.008, b[2][2]))]))
        return out

    trunk = [
        ("kangaroo-pocket", "torso", "detail", 4.1, 0.1, "hoodie", [(0.097, 1.080, 0), (0.088, 1.010, 0), (0.083, 0.945, 0)]),
        ("drawstring-0", "torso", "detail", 4.15, 0.1, "hoodie", [(0.090, 1.396, 0), (0.126, 1.262, 0)]),
        ("collar-rib", "torso", "detail", 4.1, 0.1, "hoodie", [(-0.050, 1.452, 0), (0.000, 1.452, 0), (0.060, 1.447, 0)]),
        ("hood-fold", "torso", "texture", 3.95, 0.1, "hoodie", [(-0.100, 1.468, 0), (-0.128, 1.430, 0), (-0.146, 1.375, 0), (-0.148, 1.330, 0)]),
        ("brow", "head", "texture", 5.3, 0.1, "face-skin", [(0.108, 1.613, 0), (0.094, 1.615, 0)]),
        ("mouth", "head", "texture", 5.3, 0.1, "face-skin", [(0.108, 1.529, 0), (0.092, 1.527, 0)]),
        ("hair-strand-0", "head", "texture", 5.6, 0.1, "hair", [(0.070, 1.690, 0), (0.024, 1.672, 0)]),
        ("hair-strand-1", "head", "texture", 5.6, 0.1, "hair", [(0.006, 1.704, 0), (-0.036, 1.668, 0)]),
        ("hair-strand-2", "head", "texture", 5.6, 0.1, "hair", [(-0.030, 1.640, 0), (-0.062, 1.598, 0)]),
    ]

    def front(side, sign, _leg, _arm):
        z = sign * HIP
        out = [
            ("jeans-hem-" + side, "shank-" + side, "detail", 1.03, 0.1, "jeans", [(0, 0.127, z - 0.044), (0, 0.127, z + 0.044)]),
            ("jeans-knee-%s-0" % side, "thigh-" + side, "texture", 1.03, 0.1, "jeans", [(0, 0.520, z - 0.030), (0, 0.514, z + 0.030)]),
            ("jeans-knee-%s-1" % side, "thigh-" + side, "texture", 1.03, 0.1, "jeans", [(0, 0.470, z - 0.028), (0, 0.474, z + 0.030)]),
            ("sole-line-" + side, "foot-" + side, "detail", 1.16, 0.1, "sneakers", [(0, 0.024, z - 0.046), (0, 0.024, z + 0.052)]),
            ("brow-" + side, "head", "texture", 5.3, 0.1, "face-skin", [(0, 1.612, sign * 0.013), (0, 1.615, sign * 0.032), (0, 1.611, sign * 0.050)]),
        ]
        for k, y in enumerate((0.082, 0.072, 0.062)):
            out.append(("lace-%s-%d" % (side, k), "foot-" + side, "texture", 1.2, 0.85, "sneakers", [(0, y, z - 0.015), (0, y, z + 0.017)]))
        return out

    face = [
        ("nose", "head", "texture", 5.3, 0.1, "face-skin", [(0, 1.604, 0.004), (0, 1.553, 0.014), (0, 1.549, 0.000), (0, 1.553, -0.014)]),
        ("mouth", "head", "texture", 5.3, 0.1, "face-skin", [(0, 1.530, -0.022), (0, 1.5275, 0), (0, 1.530, 0.022)]),
        ("hair-strand-0", "head", "texture", 5.6, 0.1, "hair", [(0, 1.712, -0.020), (0, 1.672, -0.032)]),
        ("hair-strand-1", "head", "texture", 5.6, 0.1, "hair", [(0, 1.716, 0.004), (0, 1.676, 0.008)]),
        ("hair-strand-2", "head", "texture", 5.6, 0.1, "hair", [(0, 1.710, 0.028), (0, 1.672, 0.040)]),
        ("drawstring-0", "torso", "detail", 4.15, 0.1, "hoodie", [(0, 1.396, -0.030), (0, 1.262, -0.036)]),
        ("drawstring-1", "torso", "detail", 4.15, 0.1, "hoodie", [(0, 1.396, 0.030), (0, 1.262, 0.036)]),
        ("collar-rib", "torso", "detail", 4.1, 0.1, "hoodie", [(0, 1.452, -0.088), (0, 1.447, -0.045), (0, 1.445, 0), (0, 1.447, 0.045), (0, 1.452, 0.088)]),
        ("kangaroo-pocket", "torso", "detail", 4.1, 0.45, "hoodie", [(0, 0.932, -0.132), (0, 0.948, -0.134), (0, 1.010, -0.122), (0, 1.074, -0.099), (0, 1.086, -0.092), (0, 1.086, 0.092), (0, 1.074, 0.099), (0, 1.010, 0.122), (0, 0.948, 0.134), (0, 0.932, 0.132)]),
    ]

    def plan(side, sign, _leg, _arm):
        return [
            ("shoulder-seam-" + side, "torso", "detail", 4.2, 0.55, "hoodie", [(-0.012, 1.46, sign * 0.088), (-0.016, 1.44, sign * 0.180)]),
            ("shoe-laces-" + side, "foot-" + side, "detail", 1.4, 0.6, "sneakers", [(0.046, 0.09, sign * HIP), (0.112, 0.06, sign * (HIP + 0.002))]),
        ]

    parting = [("hair-parting", "head", "detail", 6.3, 0.5, "hair", [(0.070, 1.70, 0.020), (0.010, 1.72, 0.026), (-0.050, 1.70, 0.022)])]
    return {"elevation": sides(elevation) + trunk, "front": sides(front) + face, "plan": sides(plan) + parting}


def scale100():
    def elevation(side, sign, leg, arm):
        return [
            ("cuff-" + side, "forearm-" + side, "detail", arm + 0.02, 0.1, "hoodie", [(-0.040, 0.878, sign * SH), (0.040, 0.878, sign * SH)]),
            ("jeans-seam-" + side, "thigh-" + side, "detail", leg + 0.03, 0.1, "jeans", [(-0.006, 0.950, sign * HIP), (-0.002, 0.700, sign * HIP), (0.004, 0.470, sign * HIP)]),
            ("sole-" + side, "foot-" + side, "detail", leg + 0.15, 0.1, "sneakers", [(-0.070, 0.024, sign * HIP), (0.150, 0.024, sign * HIP)]),
        ]

    trunk = [
        ("hem-rib", "torso", "detail", 4.1, 0.1, "hoodie", [(-0.136, 0.902, 0), (0.104, 0.902, 0)]),
        ("pocket", "torso", "detail", 4.1, 0.1, "hoodie", [(0.096, 1.080, 0), (0.087, 1.010, 0), (0.082, 0.945, 0)]),
        ("collar", "torso", "detail", 4.1, 0.1, "hoodie", [(-0.048, 1.452, 0), (0.004, 1.452, 0), (0.056, 1.448, 0)]),
    ]

    def front(side, sign, _leg, _arm):
        z = sign * HIP
        return [
            ("jeans-hem-" + side, "shank-" + side, "detail", 1.03, 0.1, "jeans", [(0, 0.127, z - 0.043), (0, 0.127, z + 0.043)]),
            ("sole-" + side, "foot-" + side, "detail", 1.16, 0.1, "sneakers", [(0, 0.024, z - 0.046), (0, 0.024, z + 0.052)]),
        ]

    middle = [
        ("hem-rib", "torso", "detail", 4.1, 0.1, "hoodie", [(0, 0.902, -0.180), (0, 0.902, 0.180)]),
        ("collar-rib", "torso", "detail", 4.1, 0.1, "hoodie", [(0, 1.452, -0.086), (0, 1.446, 0), (0, 1.452, 0.086)]),
        ("kangaroo-pocket", "torso", "detail", 4.1, 0.1, "hoodie", [(0, 0.905, -0.132), (0, 1.010, -0.122), (0, 1.086, -0.094), (0, 1.086, 0.094), (0, 1.010, 0.122), (0, 0.905, 0.132)]),
    ]
    return {"elevation": sides(elevation) + trunk, "front": sides(front) + middle, "plan": []}


def scale200():
    return {"elevation": [("pocket", "torso", "detail", 4.1, 0.1, "hoodie", [(0.094, 1.075, 0), (0.084, 0.950, 0)])], "front": [], "plan": []}


def strokes():
    return {"50": scale50(), "100": scale100(), "200": scale200(), "500": {"elevation": [], "front": [], "plan": []}}
