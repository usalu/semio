"""✏️ Round 3 tram: the side elevation, front elevation and plan of every scale drawn as the true orthographic views of that scale's body (ruling E1), stated from the dimensions of `r3-tram-kit.py`. Every existing shape keeps id, bone, role, z, fill, tone, region and material; only its geometry is redrawn; new shapes are listed in NEW. Writes `r3-tram-2d/s<N>.<view>.json` (`python3 r3-tram-draw2d.py`)."""
import copy, glob, importlib.util, json, math, os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location('r3_tram_kit', os.path.join(T, 'r3-tram-kit.py'))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
globals().update({k: v for k, v in vars(K).items() if not k.startswith('_')})
CAT = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/tram/🔣️.json")[0]
OUT = os.path.join(T, 'r3-tram-2d')
SCALES = (50, 100, 200, 500)
HEADER = {50: [GLASS_TOP, 2.80, 2.95, 3.10, 3.22, 3.31, 3.37, 3.395], 100: [GLASS_TOP, 2.85, 3.10, 3.26, 3.36, 3.395], 200: [GLASS_TOP, 3.0, 3.25, 3.37, 3.395], 500: [GLASS_TOP, 3.15, 3.395]}
RIGHT = {50: ([('e1', 0.80, 3.30), ('e2', 5.00, 6.30), ('e3', 6.45, 7.47)], [('m1', -5.85, -4.25), ('m2', -2.55, -0.85)]),
         100: ([('e1', 0.80, 3.30), ('e2', 5.00, 7.47)], [('m1', -5.85, -4.25), ('m2', -2.55, -0.85)]),
         200: ([('e1', 0.80, 3.30), ('e2', 5.00, 6.30), ('e3', 6.45, 7.47)], [('m1', -5.85, -4.25), ('m2', -2.55, -0.85)])}
DOORS = {'a1': DOOR_A[0], 'a2': DOOR_A[0] + LEAF, 'b1': DOOR_B[0], 'b2': DOOR_B[0] + LEAF}
WHEELS = {'f1': BOGIE_F - WB, 'f2': BOGIE_F + WB, 'r1': BOGIE_R - WB, 'r2': BOGIE_R + WB}


def n(v):
    return round(v + 0.0, 4)


def P(pts):
    return {"kind": "poly", "points": [[n(x), n(y)] for x, y in pts], "closed": True}


def R(x0, y0, x1, y1, r=None):
    s = {"kind": "rect", "x": n(min(x0, x1)), "y": n(min(y0, y1)), "w": n(abs(x1 - x0)), "h": n(abs(y1 - y0))}
    if r:
        s["r"] = r
    return s


def E(cx, cy, rx, ry=None):
    return {"kind": "ellipse", "cx": n(cx), "cy": n(cy), "rx": n(rx), "ry": n(rx if ry is None else ry)}


def ring(pts):
    return "M " + " L ".join(f"{n(x)} {n(y)}" for x, y in pts) + " Z"


def path(rings):
    return {"kind": "path", "d": " ".join(ring(r) for r in rings)}


def box(x0, y0, x1, y1):
    return [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]


def circle(cx, cy, r, k=24):
    return [[cx + r * math.cos(2 * math.pi * i / k), cy + r * math.sin(2 * math.pi * i / k)] for i in range(k)]


def chamfer(x0, y0, x1, y1, c):
    return [[x0 + c, y0], [x1 - c, y0], [x1, y0 + c], [x1, y1 - c], [x1 - c, y1], [x0 + c, y1], [x0, y1 - c], [x0, y0 + c]]


def mid_local(pts):
    return [[-x, 0.3 - y] for x, y in pts]


def shift(pts, dx, dy):
    return [[x - dx, y - dy] for x, y in pts]


def plumb(y):
    return HW if y <= CANT else hw(y) * HW / hw(CANT)


def side_outline(scale, raked=True, x0=E0, bogies=(BOGIE_R, BOGIE_F)):
    """side view of the end module body from the skirt over the cab to the roof, bogie openings cut in"""
    bottom = [[x0, SKIRT]]
    for xb in bogies:
        bottom += cutout(xb, 8 if scale <= 100 else 5)
    bottom = [p for p in bottom if p[0] < XC + 0.001]
    nose = [[TIP, SKIRT], [TIP, 0.46], [TIP + BUMP, 0.46], [TIP + BUMP, 0.66], [TIP, 0.66], [TIP, BELT]] if scale < 500 else [[TIP, SKIRT], [TIP, BELT]]
    if scale == 200:
        nose = [[TIP, SKIRT], [TIP, 0.45], [TIP + BUMP, 0.45], [TIP + BUMP, 0.66], [TIP, 0.66], [TIP, BELT]]
    top = [[tip(y), y] for y in HEADER[scale]]
    sill = [[TIP + 0.004, BELT - 0.001], [TIP + 0.004, BELT + 0.06], [tip(BELT + 0.06), BELT + 0.06]] if scale <= 100 else []
    ws = [[tip(y), y] for y in (1.65, 2.15)] if scale == 50 else []
    return bottom + [[XC, SKIRT]] + nose + sill + ws + top + [[x0, ROOF]]


def headlamp_side():
    z = -0.72
    yaw = math.atan2(nose_x(0.92, z + 0.01) - nose_x(0.92, z - 0.01), 0.02)
    rx = math.hypot(0.022 * math.cos(yaw), 0.12 * math.sin(yaw))
    return E(nose_x(0.92, z), 0.92, rx, 0.07)


def nose_plan(d, w, xc, m=24):
    return [[xc + dx, z] for z, dx in curve(d, w, NOSE_N, m)]


def front_section(scale):
    """the car body section seen from ahead: walls with the tumblehome (plumb at 1:200 and 1:500), cant curve and crown"""
    f = hw if scale <= 100 else plumb
    ys = [SKIRT, BELT, CANT] + [y for y, _ in CANT_TABLE[1:]]
    half = [[f(y), y] for y in ys]
    return [[-z, y] for z, y in half] + [[z, y] for z, y in reversed(half[:-1])]


def wheel_r(scale):
    return R_FLANGE if scale < 500 else 0.30


def wheel_span(scale):
    return {50: (-0.05, 0.105), 100: (-0.03, 0.10), 200: (0.0, 0.105), 500: (0.0, 0.12)}[scale]


def wheel_front(scale):
    """the lower half of the left wheel seen from ahead as (axial offset from the back face, height): hub boss, flange and tread of the scale's wheel section, mirrored to the upper half"""
    half = {50: [[-0.05, 0.2], [0.0, 0.2], [0.0, 0.0], [0.004, 0.0], [0.004, -0.022], [0.016, -0.022], [0.016, 0.0], [0.105, 0.0]],
            100: [[-0.03, 0.19], [0.0, 0.19], [0.0, -0.022], [0.02, -0.022], [0.02, 0.0], [0.10, 0.0]],
            200: [[0.0, -0.022], [0.03, -0.022], [0.03, 0.0], [0.105, 0.0]],
            500: [[0.0, 0.0], [0.12, 0.0]]}[scale]
    up = [[z, 0.6 - y] for z, y in reversed(half)]
    return [[Z_BACK + z, y] for z, y in half + up]


def wells(scale):
    """the wheelhouses seen through the bogie openings: the arch from foot to foot closed along the skirt line"""
    out = []
    for xb, bt in ((BOGIE_R, 'r'), (BOGIE_F, 'f')):
        arc = cutout(xb, 8 if scale <= 100 else 5)
        out.append((bt, [p for p in arc if p[0] <= XC + 0.001]))
    return out


def elevation(scale, sid):
    """geometry of the side elevation shape `sid` at `scale`, in the frame of its bone, or None when the shape is dropped"""
    if sid.startswith('wheel-'):
        return E(0, 0, wheel_r(scale))
    if sid.startswith('disc-'):
        return E(0, 0, 0.2)
    if sid.startswith('hub-'):
        return E(0, 0, 0.105)
    if sid == 'motor-f':
        if scale == 50:
            return path([chamfer(-0.45, -0.19, 0.45, 0.19, 0.04), box(-0.76, -0.085, -0.64, 0.085), box(0.64, -0.085, 0.76, 0.085), box(-0.64, -0.045, -0.455, 0.045), box(0.455, -0.045, 0.64, 0.045)])
        return P(chamfer(-0.45, -0.19, 0.45, 0.19, 0.05 if scale == 100 else 0.0)) if scale == 100 else P(box(-0.45, -0.19, 0.45, 0.19))
    if sid == 'motor-r':
        return P(chamfer(-0.32, -0.1, 0.32, 0.205 if scale == 50 else 0.19, 0.02))
    if sid.startswith('bogie-side-beam-'):
        xb = BOGIE_F if '-f-' in sid else BOGIE_R
        if scale == 50:
            return path([box(xb - 1.12, 0.50, xb + 1.12, 0.62), box(xb - 0.39, 0.62, xb - 0.21, 0.70), box(xb + 0.21, 0.62, xb + 0.39, 0.70), box(xb - WB - 0.09, 0.465, xb - WB + 0.09, 0.50), box(xb + WB - 0.09, 0.465, xb + WB + 0.09, 0.50)])
        if scale == 100:
            return path([box(xb - 1.12, 0.48, xb + 1.12, 0.62), box(xb - 0.39, 0.62, xb - 0.21, 0.70), box(xb + 0.21, 0.62, xb + 0.39, 0.70)])
        return R(xb - 1.12, 0.48, xb + 1.12, 0.62, 0.03)
    if sid == 'shell-end':
        return P(side_outline(scale))
    if sid == 'shell-mid':
        return P(mid_local(box(M0, SKIRT, M1, ROOF)))
    if sid == 'gangway':
        return P(box(-0.45, 0.0, 0.45, BELLOWS[2] - 0.3))
    if sid.startswith('pane-'):
        name = sid.split('-')[1]
        if name == 'cab':
            cp = [[7.62, SILL], [edge(SILL) - 0.11, SILL], [edge(HEAD) - 0.11, HEAD], [7.62, HEAD]]
            return P(cp)
        ends, mids = RIGHT[scale]
        for k, a, c in ends:
            if k == name:
                return P(box(a, SILL, c, HEAD))
        for k, a, c in mids:
            if k == name:
                return P(mid_local(box(a, SILL, c, HEAD)))
        return None
    if sid == 'window-band-end':
        return path([box(0.80, SILL, DOOR_A[0] - 0.08, HEAD), box(DOOR_A[1] + 0.08, SILL, 7.47, HEAD)])
    if sid == 'window-band-mid':
        return path([mid_local(box(-5.85, SILL, DOOR_B[0] - 0.08, HEAD)), mid_local(box(DOOR_B[1] + 0.08, SILL, -0.85, HEAD))])
    if sid.startswith('leaf-glass-'):
        g = {50: (0.11, 0.61, 2.31), 100: (0.11, 0.61, 2.31), 200: (0.12, 0.62, 2.30)}[scale]
        return P(box(g[0], g[1] - 0.3, LEAF - g[0], g[2] - 0.3))
    if sid.startswith('leaf-'):
        i = {50: 0.002, 100: 0.004, 200: 0.006, 500: 0.01}[scale]
        return P(box(i, 0.0, LEAF - i, DOOR_Y[1] - 0.3))
    if sid == 'pan-joint-base':
        return E(0, 0, 0.04)
    if sid == 'pan-joint-knee':
        return E(1.9, 0, 0.04)
    if sid == 'pan-joint-tip':
        return E(1.9, 0, 0.02)
    if sid == 'pan-head':
        c = {50: (-0.17, 0.17, 0.011, 0.025), 100: (-0.17, 0.17, 0.012, 0.025), 200: (-0.18, 0.18, 0.01, 0.0175), 500: (-0.0075, 0.0075, 0.02, 0.03)}[scale]
        return {"kind": "capsule", "x0": c[0], "y0": c[2], "x1": c[1], "y1": c[2], "r0": c[3], "r1": c[3]}
    if sid == 'pan-base':
        if scale == 50:
            rail = [[2.30, 3.488], [2.95, 3.488], [2.95, 3.47], [3.55, 3.47], [3.55, 3.488], [3.80, 3.488], [3.80, 3.548], [3.55, 3.548], [3.55, 3.57], [2.95, 3.57], [2.95, 3.548], [2.66, 3.548], [2.66, 3.595], [2.54, 3.595], [2.54, 3.548], [2.30, 3.548]]
            return path([rail, box(2.29, 3.388, 2.41, 3.488), box(3.69, 3.388, 3.81, 3.488)])
        if scale == 500:
            return R(2.28, ROOF - 0.01, 3.82, PAN[1], 0.03)
        return path([[[2.28, 3.48], [3.82, 3.48], [3.82, 3.55], [2.635, 3.55], [2.635, 3.59], [2.565, 3.59], [2.565, 3.55], [2.28, 3.55]], box(2.30, 3.39, 2.40, 3.48), box(3.70, 3.39, 3.80, 3.48)])
    if sid == 'resistor-box':
        x0, x1 = RESISTOR[0], RESISTOR[1]
        if scale <= 100:
            top = ROOF + RESISTOR[3]
            return P([[x0, ROOF + 0.04], [x0 + 0.10, ROOF + 0.04], [x0 + 0.10, ROOF - 0.025], [x0 + 0.20, ROOF - 0.025], [x0 + 0.20, ROOF + 0.04], [x1 - 0.20, ROOF + 0.04], [x1 - 0.20, ROOF - 0.025], [x1 - 0.10, ROOF - 0.025], [x1 - 0.10, ROOF + 0.04], [x1, ROOF + 0.04], [x1, top], [5.89, top], [5.89, top + 0.01], [5.11, top + 0.01], [5.11, top], [x0, top]])
        return R(x0, ROOF, x1, ROOF + RESISTOR[3], 0.02)
    if sid == 'hvac-box':
        return R(1.4, 0.3 - ROOF - HVAC[3], 5.2, 0.3 - ROOF, 0.04)
    if sid == 'headlamp':
        return headlamp_side()
    return 'keep'


def front(scale, sid):
    lo, hi = wheel_span(scale)
    if sid in ('wheel-l', 'wheel-r'):
        sd = 1 if sid.endswith('-l') else -1
        return P([[sd * z, y] for z, y in wheel_front(scale)])
    if sid == 'shell-end':
        return P(front_section(scale))
    if sid.startswith('brake-disc-ws-r1-'):
        a, b = (0.786, 0.804) if scale == 50 else (0.77, 0.79)
        return R(a, 0.1, b, 0.5) if sid.endswith('-l') else R(-b, 0.1, -a, 0.5)
    if sid == 'axle-ws-r1':
        rings = [box(-0.875, 0.12, 0.875, 0.24)] if scale == 50 else [box(-0.90, 0.12, 0.90, 0.24 if scale == 100 else 0.26)]
        if scale < 500:
            rings += [box(0.705, 0.04, 0.795, 0.12), box(-0.795, 0.04, -0.705, 0.12)] if scale == 50 else [box(0.70, 0.04, 0.80, 0.12), box(-0.80, 0.04, -0.70, 0.12)]
        return path(rings)
    if sid == 'hvac-box':
        return R(-HVAC[2], ROOF - 0.02, HVAC[2], ROOF + HVAC[3], 0.04)
    if sid == 'resistor-box':
        return R(-RESISTOR[2], ROOF, RESISTOR[2], ROOF + RESISTOR[3], 0.02)
    if sid == 'roof-boxes':
        return R(-HVAC[2], ROOF - 0.02, HVAC[2], ROOF + HVAC[3], 0.04)
    if sid == 'windscreen':
        f = hw if scale <= 100 else plumb
        return P([[-f(BELT) + 0.004, BELT], [f(BELT) - 0.004, BELT], [f(GLASS_TOP) - 0.004, GLASS_TOP], [-f(GLASS_TOP) + 0.004, GLASS_TOP]])
    if sid == 'destination-sign':
        return R(-0.5, 2.70, 0.5, 2.90, 0.02)
    if sid in ('headlamp-l', 'headlamp-r'):
        return E(0.72 if sid.endswith('-l') else -0.72, 0.92, 0.11, 0.07)
    if sid == 'headlamps':
        return path([[[0.72 + 0.11 * math.cos(t), 0.92 + 0.07 * math.sin(t)] for t in (2 * math.pi * i / 16 for i in range(16))], [[-0.72 + 0.11 * math.cos(t), 0.92 + 0.07 * math.sin(t)] for t in (2 * math.pi * i / 16 for i in range(16))]])
    if sid == 'pan-base':
        if scale == 50:
            return path([box(-0.47, 3.488, 0.47, 3.548), box(0.36, 3.388, 0.48, 3.488), box(-0.48, 3.388, -0.36, 3.488)])
        if scale == 500:
            return R(-0.47, ROOF - 0.01, 0.47, PAN[1], 0.03)
        return path([box(-0.46, 3.48, 0.46, 3.55), box(0.37, 3.39, 0.47, 3.48), box(-0.47, 3.39, -0.37, 3.48)])
    return 'keep'


def plan(scale, sid):
    if sid.startswith('door-opening-'):
        x0 = DOOR_A[0] if sid.endswith('-a') else DOOR_B[0]
        return R(x0, -HW, x0 + 2 * LEAF, -HW + TW)
    if sid.startswith('leaf-'):
        x0 = DOOR_A[0] if sid.startswith('leaf-a') else DOOR_B[0]
        x0 += LEAF if sid.endswith('2') else 0.0
        i = {50: 0.002, 100: 0.004, 200: 0.006, 500: 0.01}[scale]
        z = {50: (-0.012, 0.028), 100: (-0.012, 0.028), 200: (-0.012, 0.028), 500: (-0.012, 0.063)}[scale]
        return R(x0 + i, -HW + z[0], x0 + LEAF - i, -HW + z[1])
    if sid == 'shell-end':
        return P([[E0, -HW], [XC, -HW]] + list(reversed(nose_plan(NOSE_D + BUMP if scale < 500 else NOSE_D, HW, XC)))[1:-1] + [[XC, HW], [E0, HW]])
    if sid == 'shell-mid':
        return P(box(M0, -HW, M1, HW))
    if sid == 'gangway':
        return R(-0.45, -BELLOWS[0], 0.45, BELLOWS[0])
    if sid in ('roof-end', 'roof'):
        return R(E0, -1.0, tip(3.37), 1.0, 0.15)
    if sid == 'roof-mid':
        return R(M0, -1.0, M1, 1.0, 0.15)
    if sid == 'hvac-box':
        return R(HVAC[0], -HVAC[2], HVAC[1], HVAC[2], 0.04)
    if sid == 'resistor-box':
        return R(RESISTOR[0], -RESISTOR[2], RESISTOR[1], RESISTOR[2], 0.02)
    if sid == 'windscreen':
        f = hw if scale <= 100 else plumb
        lo = nose_plan(tip(BELT) - edge(BELT), f(BELT) - 0.004, edge(BELT), 20)
        hi = nose_plan(tip(GLASS_TOP) - edge(GLASS_TOP), f(GLASS_TOP) - 0.004, edge(GLASS_TOP), 20)
        return P(lo + list(reversed(hi)))
    if sid in ('headlamp-l', 'headlamp-r'):
        z = 0.72 if sid.endswith('-l') else -0.72
        return E(nose_x(0.92, z), z, 0.03, 0.11)
    if sid == 'pan-base':
        if scale == 50:
            return path([box(2.30, -0.47, 3.80, 0.47), box(2.38, -0.39, 3.72, 0.39)])
        if scale == 500:
            return R(2.28, -0.47, 3.82, 0.47, 0.03)
        return path([box(2.28, 0.38, 3.82, 0.46), box(2.28, -0.46, 3.82, -0.38)])
    return 'keep'


PAN_SPANS = {'pan-lower': 0.042, 'pan-upper': 0.04, 'pan-head': 0.55}
PAN_R = {50: (0.042, 0.02, 0.025), 100: (0.042, 0.02, 0.025), 200: (0.04, 0.02, 0.0175), 500: (0.035, 0.02, 0.03)}


def pan_capsule(scale, s, view):
    r = dict(zip(('pan-lower', 'pan-upper', 'pan-head'), PAN_R[scale]))[s["id"]]
    g = copy.deepcopy(s)
    if s["id"] == 'pan-head':
        c = elevation(scale, 'pan-head')
        g.update({k: c[k] for k in ('x0', 'y0', 'x1', 'y1', 'r0', 'r1')})
    else:
        g.update({"x0": 0, "y0": 0, "x1": 1.9, "y1": 0, "r0": r, "r1": r})
    span = 0.8 if s["id"] == 'pan-head' and scale == 500 else 0.845 if s["id"] == 'pan-head' and view == 'plan' else PAN_SPANS[s["id"]]
    if s["id"] == 'pan-upper' and scale == 500:
        span = 0.02
    if "solid" in g:
        g["solid"] = dict(g["solid"], z0=-n(span), z1=n(span))
    return g


def meta(base, **kw):
    s = {k: v for k, v in base.items()}
    s.update(kw)
    return s


def new_shapes(scale, view):
    """shapes the old drawings lacked: bogie legs and track brakes, gearboxes and axle boxes, the articulation arm, A-pillars, the destination sign, the drive below the skirt, the 1:500 bogie frames"""
    out = []
    det = {"role": "detail", "filled": True}
    if view == 'elevation':
        for bt, arc in wells(scale):
            out.append(dict(P(arc), id=f'well-{bt}', bone='body-end', role='body' if scale == 500 else 'detail', z=1.5, filled=True, tone=0.15, region='body-panels', material='paint'))
        for bt, xb in (('f', BOGIE_F), ('r', BOGIE_R)):
            bone = f'bogie-{bt}'
            if scale == 500:
                out.append(dict(R(-1.12, 0.10, 1.12, 0.32, 0.04), id=f'bogie-frame-{bt}', bone=bone, role='body', filled=True, z=3.5, tone=0.35, region='traction-motors', material='steel-wheel'))
                continue
            w = 0.05 if scale == 50 else 0.06
            top = 0.51 if scale == 50 else 0.49
            tb = (0.04, 0.14) if scale == 50 else (0.04, 0.16)
            out.append(dict(path([box(-0.5 - w, 0.12 - 0.3, -0.5 + w, top - 0.3), box(0.5 - w, 0.12 - 0.3, 0.5 + w, top - 0.3), box(-0.46, tb[0] - 0.3, 0.46, tb[1] - 0.3)]), id=f'bogie-legs-{bt}', bone=bone, z=3.2, tone=0.35, region=None, material='steel-wheel', **det))
            r = 0.17 if bt == 'f' and scale == 50 else 0.16 if bt == 'f' else 0.125
            name = 'gearbox' if bt == 'f' else 'axle-box'
            out.append(dict(path([circle(-WB, 0, r), circle(WB, 0, r)]), id=f'{name}-{bt}', bone=bone, z=4.6, tone=0.3, region='traction-motors' if bt == 'f' else None, material='motor-steel' if bt == 'f' else 'steel-wheel', **det))
        if scale < 500:
            arm = P([[-0.46, -0.10], [-0.17, -0.10], [-0.17, -0.15], [0.17, -0.15], [0.17, -0.10], [0.46, -0.10], [0.46, -0.03], [0.17, -0.03], [0.17, -0.01], [-0.17, -0.01], [-0.17, -0.03], [-0.46, -0.03]]) if scale <= 100 else R(-0.46, -0.14, 0.46, -0.02)
            out.append(dict(arm, id='articulation-arm', bone='gangway', z=2.1, tone=0.35, region=None, material='steel-wheel', **det))
        if scale <= 100:
            c = math.cos(math.atan(TANR))
            sgn = [[-0.015, -0.1], [0.015, -0.1], [0.015, 0.1], [-0.015, 0.1]]
            xc = nose_x(2.80, 0.3) + 0.002
            out.append(dict(P([[xc + u * c - v * math.sin(math.atan(TANR)), 2.80 + u * math.sin(math.atan(TANR)) + v * c] for u, v in sgn]), id='destination-sign', bone='body-end', role='glazing', z=6.2, filled=True, tone=0.08, region='glazing', material='glass'))
        if scale == 50:
            arm = quad((nose_x(1.17, -0.1) + 0.012, 1.17), (nose_x(1.65, -0.365) + 0.016, 1.65), 0.016)
            blade = quad((nose_x(1.3, -0.4) + 0.014, 1.3), (nose_x(2.0, -0.33) + 0.014, 2.0), 0.018)
            out.append(dict(path([arm, blade]), id='wiper', bone='body-end', role='detail', z=6.3, filled=True, tone=0.08, region='body-panels', material='rubber'))
        if scale <= 100:
            out.append({"kind": "poly", "points": [[0.0, 0.0], [0.0, n(BELLOWS[2] - 0.3)]], "closed": False, "penMm": 0.13, "id": 'bellows-folds', "bone": 'gangway', "role": 'texture', "z": 2.3, "filled": False, "tone": 0.3, "region": None, "material": None, "_multi": [round(-0.4 + 0.1 * k, 3) for k in range(9)]})
    if view == 'front':
        if scale <= 100:
            for sd, st in ((1, 'l'), (-1, 'r')):
                f = 0.80
                pts = [[sd * (f * hw(y) - 0.02 - 0.055), y] for y in (BELT, GLASS_TOP + 0.02)] + [[sd * (f * hw(y) - 0.02 + 0.055), y] for y in (GLASS_TOP + 0.02, BELT)]
                out.append(dict(P(pts), id=f'a-pillar-{st}', bone='body-end', role='detail', z=6.5, filled=True, tone=0.62, region='body-panels', material='paint'))
        if scale == 100:
            out.append(dict(R(-0.5, 2.70, 0.5, 2.90, 0.02), id='destination-sign', bone='body-end', role='glazing', z=6, filled=True, tone=0.6, region='glazing', material='glass'))
        if scale < 500:
            arm = P([[-0.14, 0.20], [-0.17, 0.20], [-0.17, 0.15], [0.17, 0.15], [0.17, 0.20], [0.14, 0.20], [0.14, 0.27], [0.17, 0.27], [0.17, 0.29], [-0.17, 0.29], [-0.17, 0.27], [-0.14, 0.27]]) if scale <= 100 else R(-0.16, 0.16, 0.16, 0.28)
            out.append(dict(arm, id='articulation-arm', bone='gangway', role='detail', z=1.3, filled=True, tone=0.35, material='steel-wheel'))
            out.append(dict(path([circle(0.93, 0.30, 0.19, 24), circle(-0.93, 0.30, 0.19, 24)]), id='drive-f', bone='bogie-f', role='detail', z=1.4, filled=True, tone=0.25, region='traction-motors', material='motor-steel'))
    if view == 'front' and scale == 500:
        out.append(dict(R(-HVAC[2], ROOF - 0.02, HVAC[2], ROOF + HVAC[3], 0.06), id='roof-boxes', bone='body-end', role='body', z=2.6, filled=True, tone=0.45, material='resistor-steel', regions=['hvac-unit', 'brake-resistor']))
        out.append(dict(R(-0.47, ROOF - 0.01, 0.47, PAN[1], 0.03), id='pan-base', bone='body-end', role='body', z=3.3, filled=True, tone=0.4, region='pantograph', material='pan-alloy'))
    out += pan_extra(scale, view)
    return out


PAN_HEAD = (2.6, 4.9736)
PAN_KNEE = (4.3617, 4.2618)


def quad(p0, p1, w):
    """a straight bar of the width w between two points"""
    dx, dy = p1[0] - p0[0], p1[1] - p0[1]
    m = math.hypot(dx, dy)
    nx, ny = -dy / m * w / 2, dx / m * w / 2
    return [[p0[0] + nx, p0[1] + ny], [p1[0] + nx, p1[1] + ny], [p1[0] - nx, p1[1] - ny], [p0[0] - nx, p0[1] - ny]]


def pan_extra(scale, view):
    """the pantograph parts the arm and head capsules cannot show: the V of the upper arms, the brace, the knee shaft and the horns"""
    meta_ = {"role": 'detail', "filled": True, "tone": 0.3, "region": 'pantograph', "material": 'pan-alloy'}
    out = []
    if scale == 500:
        return out
    z0 = 0.07 if scale == 50 else 0.06
    hw_ = 0.011 if scale <= 100 else 0.016
    if view == 'front':
        out.append(dict(path([quad((z0, PAN_KNEE[1]), (0.30, PAN_HEAD[1]), 0.04), quad((-z0, PAN_KNEE[1]), (-0.30, PAN_HEAD[1]), 0.04)]), id='pan-upper-arms', bone='pan-upper', z=3.15, **meta_))
        out.append(dict(path([quad((0.54, PAN_HEAD[1] + 0.006), (0.83, PAN_HEAD[1] - 0.115), 2 * hw_), quad((-0.54, PAN_HEAD[1] + 0.006), (-0.83, PAN_HEAD[1] - 0.115), 2 * hw_)]), id='pan-horns', bone='pan-head', z=3.25, **meta_))
        if scale <= 100:
            out.append(dict(R(-0.192 if scale == 100 else -0.197, 4.606, 0.192 if scale == 100 else 0.197, 4.630), id='pan-brace', bone='pan-upper', z=3.15, **meta_))
            out.append(dict(R(-0.10, PAN_KNEE[1] - 0.04, 0.10, PAN_KNEE[1] + 0.04), id='pan-knee', bone='pan-lower', z=3.05, **meta_))
    if view == 'plan':
        out.append(dict(path([quad((PAN_KNEE[0], z0), (PAN_HEAD[0], 0.30), 0.04), quad((PAN_KNEE[0], -z0), (PAN_HEAD[0], -0.30), 0.04)]), id='pan-upper-arms', bone='pan-upper', z=3.15, **meta_))
    if view == 'elevation':
        us = (-0.17, 0.17) if scale <= 100 else (0.0,)
        out.append(dict(path([box(u - hw_, -0.126, u + hw_, 0.017) for u in us]), id='pan-horns', bone='pan-head', z=3.25, **meta_))
        if scale == 50:
            for sid, bone, pts in (('pan-lower-rod', 'pan-lower', [[0.22, -0.075], [1.84, -0.06]]), ('pan-upper-rod', 'pan-upper', [[0.06, 0.06], [1.86, 0.045]])):
                out.append({"kind": "poly", "points": pts, "closed": False, "penMm": 0.35, "id": sid, "bone": bone, "role": 'detail', "z": 3.05, "filled": False, "tone": 0.3, "region": 'pantograph', "material": 'pan-alloy'})
    return out


def clean(s):
    if s.get("region") is None:
        s.pop("region", None)
    if s.get("material") is None:
        s.pop("material", None)
    return s


def expand(s):
    if "_multi" not in s:
        return [clean(s)]
    out = []
    for k, x in enumerate(s.pop("_multi")):
        t = copy.deepcopy(s)
        t["id"] = f'{s["id"]}-{k + 1}'
        t["points"] = [[x, p[1]] for p in s["points"]]
        out.append(clean(t))
    return out


ORIGINAL = {(50, 'elevation'): ('wheel-f1', 'disc-f1', 'hub-f1', 'wheel-f2', 'disc-f2', 'hub-f2', 'wheel-r1', 'disc-r1', 'hub-r1', 'wheel-r2', 'disc-r2', 'hub-r2', 'motor-f', 'motor-r', 'shell-end', 'bogie-side-beam-f-l', 'bogie-side-beam-r-l', 'shell-mid', 'gangway', 'pane-e1', 'pane-e2', 'pane-e3', 'pane-cab', 'pane-m1', 'pane-m2', 'leaf-a1', 'leaf-glass-a1', 'leaf-a2', 'leaf-glass-a2', 'leaf-b1', 'leaf-glass-b1', 'leaf-b2', 'leaf-glass-b2', 'pan-lower', 'pan-upper', 'pan-joint-base', 'pan-joint-knee', 'pan-joint-tip', 'pan-head', 'pan-base', 'resistor-box', 'hvac-box', 'louvre-r0', 'louvre-r1', 'louvre-r2', 'louvre-r3', 'louvre-h0', 'louvre-h1', 'louvre-h2', 'louvre-h3', 'louvre-h4', 'headlamp'), (50, 'front'): ('wheel-l', 'wheel-r', 'shell-end', 'brake-disc-ws-r1-l', 'brake-disc-ws-r1-r', 'axle-ws-r1', 'hvac-box', 'resistor-box', 'windscreen', 'destination-sign', 'headlamp-l', 'headlamp-r', 'pan-lower', 'pan-upper', 'pan-head', 'pan-base'), (50, 'plan'): ('door-opening-a', 'door-opening-b', 'leaf-a1', 'leaf-a2', 'leaf-b1', 'leaf-b2', 'shell-end', 'shell-mid', 'gangway', 'roof-end', 'roof-mid', 'hvac-box', 'resistor-box', 'windscreen', 'headlamp-l', 'headlamp-r', 'pan-base', 'pan-lower', 'pan-upper', 'pan-head'), (100, 'elevation'): ('wheel-f1', 'disc-f1', 'wheel-f2', 'disc-f2', 'wheel-r1', 'disc-r1', 'wheel-r2', 'disc-r2', 'motor-f', 'motor-r', 'shell-end', 'bogie-side-beam-f-l', 'bogie-side-beam-r-l', 'shell-mid', 'gangway', 'pane-e1', 'pane-e2', 'pane-cab', 'pane-m1', 'pane-m2', 'leaf-a1', 'leaf-glass-a1', 'leaf-a2', 'leaf-glass-a2', 'leaf-b1', 'leaf-glass-b1', 'leaf-b2', 'leaf-glass-b2', 'pan-lower', 'pan-upper', 'pan-joint-base', 'pan-joint-knee', 'pan-joint-tip', 'pan-head', 'pan-base', 'resistor-box', 'hvac-box', 'headlamp'), (100, 'front'): ('wheel-l', 'wheel-r', 'shell-end', 'brake-disc-ws-r1-l', 'brake-disc-ws-r1-r', 'axle-ws-r1', 'hvac-box', 'resistor-box', 'windscreen', 'headlamps', 'pan-lower', 'pan-upper', 'pan-head', 'pan-base'), (100, 'plan'): ('door-opening-a', 'door-opening-b', 'leaf-a1', 'leaf-a2', 'leaf-b1', 'leaf-b2', 'shell-end', 'shell-mid', 'gangway', 'roof', 'hvac-box', 'resistor-box', 'windscreen', 'pan-base', 'pan-lower', 'pan-upper', 'pan-head'), (200, 'elevation'): ('wheel-f1', 'disc-f1', 'wheel-f2', 'disc-f2', 'wheel-r1', 'disc-r1', 'wheel-r2', 'disc-r2', 'shell-end', 'shell-mid', 'gangway', 'pane-e1', 'pane-e2', 'pane-e3', 'pane-cab', 'pane-m1', 'pane-m2', 'leaf-a1', 'leaf-a2', 'leaf-b1', 'leaf-b2', 'pan-lower', 'pan-upper', 'pan-head', 'pan-base', 'resistor-box', 'hvac-box'), (200, 'front'): ('wheel-l', 'wheel-r', 'shell-end', 'axle-ws-r1', 'roof-boxes', 'windscreen', 'pan-lower', 'pan-upper', 'pan-head'), (200, 'plan'): ('door-opening-a', 'door-opening-b', 'leaf-a1', 'leaf-a2', 'leaf-b1', 'leaf-b2', 'shell-end', 'shell-mid', 'gangway', 'hvac-box', 'resistor-box', 'pan-lower', 'pan-upper', 'pan-head'), (500, 'elevation'): ('wheel-f1', 'wheel-f2', 'wheel-r1', 'wheel-r2', 'shell-end', 'shell-mid', 'gangway', 'window-band-end', 'window-band-mid', 'leaf-a1', 'leaf-a2', 'leaf-b1', 'leaf-b2', 'pan-lower', 'pan-upper', 'pan-head', 'pan-base', 'resistor-box', 'hvac-box'), (500, 'front'): ('wheel-l', 'wheel-r', 'shell-end', 'axle-ws-r1', 'windscreen', 'pan-lower', 'pan-upper', 'pan-head'), (500, 'plan'): ('shell-end', 'shell-mid', 'gangway', 'door-opening-a', 'door-opening-b', 'pan-lower', 'pan-upper', 'pan-head', 'leaf-a1', 'leaf-a2', 'leaf-b1', 'leaf-b2')}
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "d", "x0", "y0", "x1", "y1", "r0", "r1", "rotationDeg")


def redraw(asset):
    """the drawings of every scale and view: existing shapes redrawn in place, shapes of this round (NEW) replaced by their current statement or dropped when no longer stated"""
    fresh = {(sc, v): [e for s in new_shapes(sc, v) for e in expand(s)] for sc in SCALES for v in ('elevation', 'front', 'plan')}
    own = {e["id"] for lst in fresh.values() for e in lst} | {'wiper', 'pan-knee', 'pan-brace'}
    out = {}
    for scale in SCALES:
        for view, fn in (('elevation', elevation), ('front', front), ('plan', plan)):
            mine = {e["id"]: e for e in fresh[(scale, view)]}
            shapes, done = [], set()
            for s in asset["representations"][str(scale)]["geometry"][view]:
                if s["id"] in mine:
                    shapes.append(mine[s["id"]])
                    done.add(s["id"])
                    continue
                if s["id"] in own and s["id"] not in ORIGINAL.get((scale, view), ()):
                    continue
                if s["id"] in ('pan-lower', 'pan-upper', 'pan-head') and s["kind"] == 'capsule':
                    shapes.append(pan_capsule(scale, s, view))
                    continue
                g = fn(scale, s["id"])
                if g == 'keep' or (s["kind"] == 'poly' and "penMm" in s and not s.get("filled")):
                    shapes.append(s)
                    continue
                if g is None:
                    continue
                shapes.append({**g, **{k: v for k, v in s.items() if k not in GEOM}})
            shapes += [e for e in fresh[(scale, view)] if e["id"] not in done]
            out[(scale, view)] = shapes
    return out


if __name__ == '__main__':
    a = json.load(open(CAT))["asset"]
    os.makedirs(OUT, exist_ok=True)
    for (scale, view), shapes in redraw(a).items():
        json.dump(shapes, open(os.path.join(OUT, f's{scale}.{view}.json'), 'w'), ensure_ascii=False, indent=1)
        print(scale, view, len(shapes))
