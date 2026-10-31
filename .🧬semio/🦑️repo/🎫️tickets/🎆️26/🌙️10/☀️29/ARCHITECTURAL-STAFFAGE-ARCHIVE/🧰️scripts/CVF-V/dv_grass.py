from wl import *
BASE = (0.0, 0.12)
B = {"1": [(-0.43, 0.37), (-1.01, 0.21)], "2": [(-0.32, 0.57), (-0.94, 0.74)], "3": [(-0.16, 0.70), (-0.69, 1.15)]}
CULM = {1: [(0.30, 0.86), (0.55, 1.23), (0.72, 1.48)], 2: [(0.10, 0.91), (0.20, 1.35), (0.27, 1.64)], 3: [(-0.10, 0.91), (-0.20, 1.35), (-0.27, 1.64)], 4: [(-0.30, 0.86), (-0.55, 1.23), (-0.72, 1.48)]}

def mir(p, s): return (-p[0] if s == "R" else p[0], p[1])
def rotb(p, deg): return rot(p, deg, BASE)

def bundle_pts(k, s, deg=0.0, sc=1.0):
    P = [BASE] + [mir(p, "L" if s == "L" else "R") for p in B[k]]
    c = BASE
    P = [(c[0] + (p[0]-c[0])*sc, c[1] + (p[1]-c[1])*sc) for p in P]
    return [rotb(p, deg * (1 if s == "L" else -1)) for p in P]

def culm_pts(i):
    return [BASE] + CULM[i]

def spray(R, s, k, tag, deg, w, z, region="blades", tone=0.55, sc=1.0):
    P = bundle_pts(k, s, deg, sc)
    ids = [f"bl-{s}{k}a{tag}", f"bl-{s}{k}b{tag}"]
    q = P[-1]; p0 = P[-2]; d = math.hypot(q[0]-p0[0], q[1]-p0[1]); P[-1] = (q[0] + (q[0]-p0[0])/d*0.07, q[1] + (q[1]-p0[1])/d*0.07)
    return pieces(R, P, [w*0.30, w*0.50, 0.006], [f"bl-{s}{k}a", f"bl-{s}{k}b"], ids, region, z=z, tone=tone, ext=0.4, n=5)

def plume(R, i, w, z, tone=0.8):
    P = [CULM[i][1], CULM[i][2]]
    d = None
    c0, c1 = P[0], P[1]
    ang = math.atan2(c1[1]-c0[1], c1[0]-c0[0]); L = math.hypot(c1[0]-c0[0], c1[1]-c0[1])
    ux, uy = math.cos(ang), math.sin(ang); nx, ny = -uy, ux
    pts = [(c0[0]-ux*0.03, c0[1]-uy*0.03), (c0[0] + ux*L*0.35 + nx*w*0.8, c0[1] + uy*L*0.35 + ny*w*0.8), (c0[0] + ux*L*0.75 + nx*w, c0[1] + uy*L*0.75 + ny*w), (c1[0] + ux*0.11, c1[1] + uy*0.11), (c0[0] + ux*L*0.75 - nx*w, c0[1] + uy*L*0.75 - ny*w), (c0[0] + ux*L*0.35 - nx*w*0.8, c0[1] + uy*L*0.35 - ny*w*0.8)]
    return shape(R, f"plume-{i}", f"plume-{i}", "body", smooth(pts, closed=True), region="plumes", tone=tone, z=z)

def culm_stroke(R, i, z, role="detail"):
    P = culm_pts(i)
    o = []
    o.append(shape(R, f"culm-{i}a", f"culm-{i}a", role, stroke([P[0], P[1]], False), region="culms", filled=False, tone=0.2, z=z))
    o.append(shape(R, f"culm-{i}b", f"culm-{i}b", role, stroke([P[1], P[2]], False), region="culms", filled=False, tone=0.2, z=z))
    return o

def strokes_blade(R, s, k, tag, deg, sc, z, pen=None):
    P = bundle_pts(k, s, deg, sc)
    o = []
    for j, bone in enumerate((f"bl-{s}{k}a", f"bl-{s}{k}b")):
        seg = [P[j], P[j+1]]
        if j == 1:
            mid = ((seg[0][0]+seg[1][0])/2, (seg[0][1]+seg[1][1])/2 + 0.06)
            seg = [seg[0], mid, seg[1]]
        sh = shape(R, f"bl-{s}{k}{'ab'[j]}{tag}", bone, "body", stroke(seg, len(seg) > 2), region="blades", filled=False, tone=0.4, z=z)
        o.append(sh)
    return o

def strand_pts(i):
    c0, c1 = CULM[i][1], CULM[i][2]; L = math.hypot(c1[0]-c0[0], c1[1]-c0[1]); ux, uy = (c1[0]-c0[0])/L, (c1[1]-c0[1])/L
    return stroke([(c0[0]-ux*0.06, c0[1]-uy*0.06), (c1[0]+ux*0.14, c1[1]+uy*0.14)], False)

def design(R, A):
    res = {}
    r2 = []
    for s in "LR":
        for k in "123":
            r2 += spray(R, s, k, "", 0, {"1": 0.14, "2": 0.14, "3": 0.28}[k], 3 + 0.1*int(k))
    for i in (1, 2, 3, 4):
        r2 += culm_stroke(R, i, 4)
        r2.append(plume(R, i, 0.05, 4.2))
    r2.append(shape(R, "crown", "root-plate", "body", poly([(-0.20, 0.0), (0.20, 0.0), (0.16, 0.20), (-0.16, 0.20)]), region="blades", tone=0.35, z=1))
    res["200"] = r2
    r1 = []
    for s in "LR":
        for k in "123":
            r1 += spray(R, s, k, "-o", 7, 0.10, 3 + 0.1*int(k), tone=0.5)
            r1 += spray(R, s, k, "-i", -7, 0.10, 3.05 + 0.1*int(k), tone=0.6, sc=0.93)
    for i in (1, 2, 3, 4):
        r1 += culm_stroke(R, i, 4)
        r1.append(plume(R, i, 0.045, 4.2))
        r1.append(shape(R, f"plume-{i}-strand", f"plume-{i}", "texture", strand_pts(i), filled=False, region="plumes", tone=0.2, z=4.3))
    r1.append(shape(R, "crown", "root-plate", "body", poly([(-0.20, 0.0), (0.20, 0.0), (0.16, 0.20), (-0.16, 0.20)]), region="blades", tone=0.35, z=1))
    res["100"] = r1
    return res
