"""🚊️ Round 3 tram: the 1:100, 1:200 and 1:500 lists, each stated anew for the smallest feature of its scale (15, 30, 75 mm): merged assemblies, simplified sections, the same real proportions as 1:50."""
import importlib.util, math, os

_here = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location(name.replace('-', '_'), os.path.join(_here, name + '.py'))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


K = _load('r3-tram-kit')
F = _load('r3-tram-s50')
globals().update({k: v for k, v in vars(K).items() if not k.startswith('_')})

PAINT, GLASS, DARK, STEEL = 0.8, 0.28, 0.12, 0.4
END, MID = 'body-end', 'body-mid'
DOOR_NAMES = (('a1', DOOR_A[0]), ('a2', DOOR_A[0] + LEAF), ('b1', DOOR_B[0]), ('b2', DOOR_B[0] + LEAF))
RIGHT_100 = {'end': [('e1', 0.80, 3.30), ('e2', 5.00, 7.47)], 'mid': [('m1', -5.85, -4.25), ('m2', -2.55, -0.85)]}
LEFT_100 = {'end': [('e1', 0.80, 3.30), ('e4', 3.45, 4.85), ('e2', 5.00, 7.47)], 'mid': [('m1', -5.85, -2.70), ('m2', -2.55, -0.85)]}
RIGHT_200 = {'end': [('e1', 0.80, 3.30), ('e2', 5.00, 6.30), ('e3', 6.45, 7.47)], 'mid': [('m1', -5.85, -4.25), ('m2', -2.55, -0.85)]}
LEFT_200 = {'end': [('e1', 0.80, 3.30), ('e2', 3.45, 7.47)], 'mid': [('m1', -5.85, -0.85)]}


def side(tag):
    return (lambda s: s) if tag == 'end' else F.mid


def roof_section(y0=CANT, lean=True):
    """the roof cap from the cant rail over the crown as (y, z)"""
    k = 1.0 if lean else HW / hw(CANT)
    half = [(hw(y0) * k, y0)] + [(w * k, y) for y, w in CANT_TABLE if y > y0]
    pts = [(-w, y) for w, y in half] + [(w, y) for w, y in reversed(half[:-1])]
    return [[y, z] for z, y in pts]


def plumb(y):
    """half width of the generalised section of 1:200 and 1:500: plumb walls up to the cant rail, the cant curve widened to meet them"""
    return HW if y <= CANT else hw(y) * HW / hw(CANT)


def cab_masses(scale, rings_w, rings_h):
    """the cab: lower nose block, raked windscreen and domed header, with the ring count and the section of the scale"""
    wf = hw if scale == 100 else plumb
    b = [ext('cab-nose', END, 'paint', 'body', PAINT, 'zx', dfill(NOSE_D, HW, XC, NOSE_N, 16 if scale > 100 else 24), SKIRT, BELT)]
    ys = [BELT + (GLASS_TOP - BELT) * k / (rings_w - 1) for k in range(rings_w)]
    b.append(loft('windscreen', END, 'glass', 'glazing', GLASS, [ring(y, edge(y), tip(y) - edge(y), wf(y) - 0.004) for y in ys]))
    b.append(loft('cab-header', END, 'paint', 'body', PAINT, [ring(y, edge(y), tip(y) - edge(y) - 0.002, wf(y) - 0.002) for y in rings_h]))
    return b


def bogie_masses(xb, bt, powered, scale):
    """bogie frame, wheels and drive at a coarse scale"""
    bone = f'bogie-{bt}'
    b = []
    for sd, st in ((1, 'l'), (-1, 'r')):
        za = sorted((sd * 0.80, sd * 1.0))
        if scale == 500:
            b.append(slab(f'bogie-frame-{bt}-{st}', END, 'steel-wheel', 'body', STEEL, xb - 1.12, xb + 1.12, 0.40, 0.62, za[0], za[1], corner=0.04))
        else:
            b.append(slab(f'bogie-beam-{bt}-{st}', END, 'steel-wheel', 'body', STEEL, xb - 1.12, xb + 1.12, 0.48, 0.62, za[0], za[1], corner=0.03))
            if scale == 100:
                for k, dx in enumerate((-0.3, 0.3)):
                    b.append(cyl(f'spring-{bt}-{st}-{k + 1}', END, 'rubber', 'detail', 0.15, [xb + dx, 0.62, sd * 0.90], (0, 1, 0), 0.08, 0.09))
            for k, dx in enumerate((-0.5, 0.5)):
                zu = sorted((sd * 0.80, sd * 0.90))
                b.append(slab(f'bogie-leg-{bt}-{st}-{k + 1}', END, 'steel-wheel', 'body', STEEL, xb + dx - 0.06, xb + dx + 0.06, 0.12, 0.49, zu[0], zu[1]))
            zt = sorted((sd * 0.70, sd * 0.80))
            b.append(slab(f'track-brake-{bt}-{st}', END, 'cast-iron', 'body', 0.2, xb - 0.46, xb + 0.46, 0.04, 0.16, zt[0], zt[1], corner=0.015))
            if powered:
                b.append(rev(f'motor-{bt}-{st}', END, 'motor-steel', 'body', 0.25, [[0, 0.0], [0, 0.15], [0.05, 0.19], [0.85, 0.19], [0.9, 0.15], [0.9, 0.0]] if scale == 100 else [[0, 0.0], [0, 0.19], [0.9, 0.19], [0.9, 0.0]], [xb - 0.45, 0.3, sd * 0.93], (1, 0, 0)))
                for k, dx in enumerate((-WB, WB)):
                    b.append(cyl(f'gearbox-{bt}-{st}-{k + 1}', END, 'motor-steel', 'body', 0.3, [xb + dx, 0.3, sd * 0.825], (0, 0, sd), 0.17, 0.16))
            else:
                zm = sorted((sd * 0.82, sd * 0.98))
                b.append(slab(f'brake-unit-{bt}-{st}', END, 'cast-iron', 'body', 0.3, xb - 0.32, xb + 0.32, 0.2, 0.49, zm[0], zm[1], corner=0.02))
    for k, dx in enumerate((-0.5, 0.5) if scale == 100 else (0.0,)):
        w_ = 0.05 if scale == 100 else 0.12
        b.append(slab(f'bogie-cross-{bt}' + (f'-{k + 1}' if scale == 100 else ''), END, 'steel-wheel', 'body', STEEL, xb + dx - w_, xb + dx + w_, 0.12, 0.24 if scale == 100 else 0.26, -0.90, 0.90, corner=0.01 if scale == 100 else 0.02))
    b = [on(s, bone, (xb, 0.3)) for s in b]
    for k, dx in enumerate((-WB, WB)):
        wb = f'ws-{bt}{k + 1}'
        w = []
        for sd, st in ((1, 'l'), (-1, 'r')):
            if scale == 100:
                loop = [[-0.03, 0.06], [-0.03, 0.11], [0, 0.13], [0, R_FLANGE], [0.02, R_FLANGE], [0.03, 0.302], [0.1, 0.298], [0.1, 0.24], [0.08, 0.22], [0.08, 0.06]]
            elif scale == 200:
                loop = [[0, 0.06], [0, R_FLANGE], [0.03, R_FLANGE], [0.03, 0.30], [0.105, 0.30], [0.105, 0.06]]
            else:
                loop = [[0, 0.0], [0, 0.30], [0.12, 0.30], [0.12, 0.0]]
            w.append(rev(f'wheel-{wb}-{st}', END, 'steel-wheel', 'body', 0.3, loop, [xb + dx, 0.3, sd * Z_BACK], (0, 0, sd)))
        if scale < 500:
            for sd, st in ((1, 'l'), (-1, 'r')):
                w.append(cyl(f'stub-axle-{wb}-{st}', END, 'steel-wheel', 'body', STEEL, [xb + dx, 0.3, sd * 0.56], (0, 0, sd), 0.14, 0.06 if scale == 100 else 0.075))
        if scale == 100:
            for sd, st in ((1, 'l'), (-1, 'r')):
                w.append(rev(f'brake-disc-{wb}-{st}', END, 'cast-iron', 'detail', 0.55, [[0.08, 0.09], [0.08, 0.2], [0.1, 0.2], [0.1, 0.09]], [xb + dx, 0.3, sd * Z_BACK], (0, 0, sd)))
        b += [on(s, wb, (xb + dx, 0.3)) for s in w]
    return b


def doors(scale):
    b = []
    for name, x0 in DOOR_NAMES:
        y0, y1 = DOOR_Y
        if scale == 100:
            g = rect(x0 + 0.12, 0.62, x0 + LEAF - 0.12, 2.30)
            w = [ext(f'leaf-{name}', END, 'paint', 'body', PAINT - 0.08, 'xy', rect(x0 + 0.004, y0, x0 + LEAF - 0.004, y1), -HW - 0.012, -HW + 0.028, holes=[g])]
            w.append(slab(f'leaf-glass-{name}', END, 'glass', 'glazing', GLASS, x0 + 0.11, x0 + LEAF - 0.11, 0.61, 2.31, -HW - 0.006, -HW + 0.018))
        elif scale == 200:
            w = [slab(f'leaf-{name}', END, 'paint', 'body', PAINT - 0.08, x0 + 0.006, x0 + LEAF - 0.006, y0, y1, -HW - 0.012, -HW + 0.028)]
            w.append(slab(f'leaf-glass-{name}', END, 'glass', 'glazing', GLASS, x0 + 0.12, x0 + LEAF - 0.12, 0.62, 2.30, -HW - 0.016, -HW - 0.008))
        else:
            w = [slab(f'leaf-{name}', END, 'paint', 'body', PAINT - 0.08, x0 + 0.01, x0 + LEAF - 0.01, y0, y1, -HW - 0.012, -HW + 0.063)]
        b += [on(lean(s) if scale == 100 else s, f'door-{name}', (x0, 0.3)) for s in w]
    return b


def bellows(scale):
    """the folded bellows: at 1:100 four folded skins with a flat back each (nine folds), at 1:200 one solid with four folds, at 1:500 a rounded block"""
    zo = BELLOWS[0]
    y0, y1 = BELLOWS[1], BELLOWS[2]
    b = []
    if scale == 100:
        t, depth = 0.015, 0.06
        xs = [-0.45 + 0.05 * k for k in range(19)]
        zig = lambda o, i: [(o if k % 2 == 0 else i) for k in range(19)]
        for sd, st in ((1, 'l'), (-1, 'r')):
            outer = [[sd * z, x] for z, x in zip(zig(zo, zo - depth), xs)]
            back = [[sd * (zo - depth - t), 0.45], [sd * (zo - depth - t), -0.45]]
            b.append(ext(f'bellows-{st}', END, 'rubber', 'body', 0.16, 'zx', outer + back, y0, y1))
        top = [[x, y] for x, y in zip(xs, zig(y1, y1 - depth))]
        b.append(ext('bellows-top', END, 'rubber', 'body', 0.2, 'xy', top + [[0.45, y1 - depth - t], [-0.45, y1 - depth - t]], -zo, zo))
        bot = [[x, y] for x, y in zip(xs, zig(y0, y0 + depth))]
        b.append(ext('bellows-bottom', END, 'rubber', 'body', 0.12, 'xy', [[-0.45, y0 + depth + t], [0.45, y0 + depth + t]] + list(reversed(bot)), -zo, zo))
    elif scale == 200:
        xs = [-0.45 + 0.9 * k / 8 for k in range(9)]
        zz = [zo if k % 2 == 0 else zo - 0.07 for k in range(9)]
        left = [[z, x] for z, x in zip(zz, xs)]
        b.append(ext('bellows', END, 'rubber', 'body', 0.16, 'zx', left + [[-z, x] for z, x in reversed(left)], y0, y1))
    else:
        b.append(ext('bellows', END, 'rubber', 'body', 0.16, 'yz', rrect(y0, -zo, y1, zo, 0.15, 3), -0.45, 0.45))
    if scale < 500:
        b.append(slab('articulation-arm', END, 'steel-wheel', 'body', STEEL, -0.46, 0.46, 0.20, 0.27, -0.14, 0.14, corner=0.01) if scale == 100 else slab('articulation-arm', END, 'steel-wheel', 'body', STEEL, -0.46, 0.46, 0.16, 0.28, -0.16, 0.16, corner=0.02))
        if scale == 100:
            b.append(cyl('articulation-bearing', END, 'steel-wheel', 'body', STEEL, [0, 0.15, 0], (0, 1, 0), 0.14, 0.17))
    return [on(s, 'gangway', (0, 0.3)) for s in b]


def pantograph(scale):
    px, py = PAN
    L, U, H = 'pan-lower', 'pan-upper', 'pan-head'
    b = []
    if scale == 500:
        b.append(slab('pan-base', END, 'pan-alloy', 'body', 0.45, px - 0.32, px + 1.22, ROOF - 0.01, py, -0.47, 0.47, corner=0.03))
    else:
        for i, x in enumerate((px - 0.25, px + 1.15)):
            for sd, st in ((1, 'l'), (-1, 'r')):
                b.append(cyl(f'pan-insulator-{i + 1}-{st}', END, 'rubber', 'detail', 0.55, [x, ROOF - 0.01, sd * 0.42], (0, 1, 0), py - 0.06 - ROOF + 0.01, 0.05))
        for sd, st in ((1, 'l'), (-1, 'r')):
            za = sorted((sd * 0.38, sd * 0.46))
            b.append(slab(f'pan-frame-{st}', END, 'pan-alloy', 'body', 0.45, px - 0.32, px + 1.22, py - 0.07, py, za[0], za[1]))
        b.append(cyl('pan-shaft', END, 'pan-alloy', 'detail', 0.4, [px, py, -0.38], (0, 0, 1), 0.76, 0.035 if scale == 100 else 0.04))
    r = {100: (0.042, 0.034, 0.02), 200: (0.04, 0.035, 0.02), 500: (0.035, 0.035, 0.02)}[scale]
    b.append(cap('pan-lower-arm', L, 'pan-alloy', 'body', 0.45, [0, 0, 0], [1.9, 0, 0], r[0], r[1], slender=True))
    if scale == 500:
        b.append(cap('pan-upper-arm', U, 'pan-alloy', 'body', 0.45, [0, 0, 0], [1.9, 0, 0], r[2], slender=True))
        b.append(box('pan-head-bar', H, 'carbon', 'body', 0.1, [0.075, 0.06, 1.6], [0, 0.02, 0]))
        return b
    for sd, st in ((1, 'l'), (-1, 'r')):
        b.append(cap(f'pan-upper-arm-{st}', U, 'pan-alloy', 'body', 0.45, [0, 0, sd * 0.06], [1.9, 0, sd * 0.30], r[2], slender=True))
    if scale == 100:
        b.append(cyl('pan-knee', L, 'pan-alloy', 'detail', 0.4, [1.9, 0, -0.1], (0, 0, 1), 0.2, 0.04))
        b.append(cyl('pan-head-shaft', H, 'pan-alloy', 'detail', 0.4, [0, 0, -0.34], (0, 0, 1), 0.68, 0.016))
        b.append(box('pan-apex', H, 'pan-alloy', 'detail', 0.45, [0.38, 0.02, 0.62], [0, -0.004, 0]))
        b.append(cap('pan-upper-brace', U, 'pan-alloy', 'detail', 0.45, [0.95, 0, -0.18], [0.95, 0, 0.18], 0.012))
        for k, u in enumerate((-0.17, 0.17)):
            b.append(box(f'pan-strip-{k + 1}', H, 'carbon', 'body', 0.1, [0.05, 0.03, 1.1], [u, 0.012, 0], corner=0.006))
            for sd, st in ((1, 'l'), (-1, 'r')):
                b.append(cap(f'pan-horn-{k + 1}-{st}', H, 'pan-alloy', 'detail', 0.5, [u, 0.006, sd * 0.54], [u, -0.115, sd * 0.83], 0.011))
    else:
        b.append(box('pan-head-bar', H, 'carbon', 'body', 0.1, [0.4, 0.035, 1.1], [0, 0.01, 0], corner=0.008))
        for sd, st in ((1, 'l'), (-1, 'r')):
            b.append(cap(f'pan-horn-{st}', H, 'pan-alloy', 'detail', 0.5, [0, 0.0, sd * 0.54], [0, -0.115, sd * 0.83], 0.016))
    return b


def roof_units(scale):
    x0, x1, hz, h = HVAC
    c = {100: 0.04, 200: 0.04, 500: 0.06}[scale]
    hv = [slab('hvac-box', END, 'paint', 'body', PAINT - 0.12, x0, x1, ROOF - 0.02, ROOF + h, -hz, hz, corner=c)]
    if scale == 100:
        for k, x in enumerate((-4.5, -3.5)):
            hv.append(cyl(f'hvac-fan-{k + 1}', END, 'rubber', 'detail', DARK, [x, ROOF + h - 0.004, 0], (0, 1, 0), 0.02, 0.36))
    b = [F.mid(s) for s in hv]
    rx0, rx1, rhz, rh, _ = RESISTOR
    b.append(slab('resistor-box', END, 'resistor-steel', 'body', 0.55, rx0, rx1, ROOF - 0.02 if scale > 100 else ROOF + 0.04, ROOF + rh, -rhz, rhz, corner=c / 2))
    if scale == 100:
        for i, x in enumerate((rx0 + 0.15, rx1 - 0.15)):
            b.append(slab(f'resistor-foot-{i + 1}', END, 'resistor-steel', 'detail', 0.4, x - 0.05, x + 0.05, ROOF - 0.02, ROOF + 0.045, -0.69, 0.69))
    if scale == 100:
        b.append(cyl('resistor-fan', END, 'resistor-steel', 'detail', 0.3, [5.5, ROOF + rh - 0.004, 0], (0, 1, 0), 0.02, 0.39))
    return b


def car100(tag, x0, x1, bogies, door, raked):
    """1:100 module: 51.5 mm walls with the merged window openings, flush 20 mm panes, roof, floor with the bogie wells, end walls"""
    w = side(tag)
    b = []
    for sd, st, panes in ((1, 'l', LEFT_100[tag]), (-1, 'r', RIGHT_100[tag])):
        dr = door if sd < 0 else None
        z0, z1 = (HW - TW, HW) if sd > 0 else (-HW, -HW + TW)
        for k, prof in enumerate(F.lower_profiles(x0, x1, bogies, dr)):
            b.append(w(ext(f'{tag}-wall-lower-{st}' + (f'-{"ab"[k]}' if dr else ''), END, 'paint', 'body', PAINT, 'xy', prof, z0, z1)))
        holes = [rect(a, F.S0, c, F.S1) for _, a, c in panes]
        if raked:
            holes.append([[x, s_of(y)] for x, y in F.cab_pane()])
        zc = (-TW, 0) if sd > 0 else (0, TW)
        b.append(w(ext(f'{tag}-wall-upper-{st}', END, 'paint', 'body', PAINT, 'xy', F.upper_profile(x0, x1, dr, raked), zc[0], zc[1], holes=holes, at=[0, BELT, sd * HW], rot=qx(-sd * PHI))))
        sm = (F.S0 + F.S1) / 2
        for n, a, c in panes:
            b.append(w(box(f'pane-{n}-{st}', END, 'glass', 'glazing', GLASS, [c - a + 0.02, F.S1 - F.S0 + 0.02, 0.02], on_wall(sd, (a + c) / 2, sm, -sd * 0.014), rot=qx(-sd * PHI))))
        if raked:
            cp = F.cab_pane()
            grown = [[cp[0][0] - 0.01, s_of(SILL) - 0.01], [cp[1][0] + 0.012, s_of(SILL) - 0.01], [cp[2][0] + 0.012, s_of(HEAD) + 0.01], [cp[3][0] - 0.01, s_of(HEAD) + 0.01]]
            zp = (-0.024, -0.004) if sd > 0 else (0.004, 0.024)
            b.append(ext(f'pane-cab-{st}', END, 'glass', 'glazing', GLASS, 'xy', grown, zp[0], zp[1], at=[0, BELT, sd * HW], rot=qx(-sd * PHI)))
    xr = x1 - (TANR * (ROOF - BELT) - 0.06 if raked else 0.0)
    b.append(w(ext(f'{tag}-roof', END, 'paint', 'body', PAINT + 0.05, 'yz', roof_outline() + list(reversed(roof_outline(TW))), x0, xr)))
    hwf = HW - 0.002
    half = [[-hwf, x0]]
    for xb in bogies:
        half += [[-hwf, xb - 1.26], [-0.62, xb - 1.26], [-0.62, min(xb + 1.26, x1)]] + ([[-hwf, xb + 1.26]] if xb + 1.26 < x1 else [])
    half += [[-hwf, x1]] if half[-1][1] < x1 else []
    b.append(w(ext(f'{tag}-floor', END, 'paint', 'body', 0.5, 'zx', half + [[-z, x] for z, x in reversed(half)], SKIRT, FLOOR)))
    for xb, bt in zip(bogies, 'rf'):
        xe = min(xb + 1.30, x1)
        for sd, st in ((1, 'l'), (-1, 'r')):
            prof = [[sd * 0.60, SKIRT], [sd * 0.64, SKIRT], [sd * 0.64, 0.70], [sd * (HW - TW + 0.004), 0.70], [sd * (HW - TW + 0.004), 0.74], [sd * 0.60, 0.74]]
            b.append(ext(f'well-{bt}-{st}', END, 'paint', 'body', 0.45, 'yz', [[y, z] for z, y in prof], xb - 1.30, xe))
    return b


def ends100():
    portal = rect(PORTAL[1], -PORTAL[0], PORTAL[2], PORTAL[0])
    return [ext('end-rear-wall', END, 'paint', 'body', PAINT - 0.1, 'yz', section(), E0, E0 + 0.05, holes=[portal]),
            F.mid(ext('mid-front-wall', END, 'paint', 'body', PAINT - 0.1, 'yz', section(), M1 - 0.05, M1, holes=[portal])),
            F.mid(ext('mid-rear-wall', END, 'paint', 'body', PAINT - 0.1, 'yz', section(), M0, M0 + 0.05))]


def cab100():
    b = cab_masses(100, 3, [GLASS_TOP, 2.85, 3.10, 3.26, 3.36, 3.395])
    b.append(ext('cab-sill', END, 'paint', 'body', PAINT, 'zx', dband(NOSE_D + 0.004, HW - 0.001, 0.07, XC), BELT - 0.001, BELT + 0.06))
    b.append(ext('bumper', END, 'rubber', 'detail', 0.3, 'zx', dband(NOSE_D + BUMP, HW, 0.07, XC), 0.46, 0.66))
    for sd, st in ((1, 'l'), (-1, 'r')):
        f = 0.80
        b.append(loft(f'a-pillar-{st}', END, 'paint', 'body', PAINT, [sec([nose_x(y, sd * f * hw(y)) - 0.022, y, sd * (f * hw(y) - 0.02)], 0.05, 0.055, 4, QRING) for y in (BELT, GLASS_TOP + 0.02)]))
        z = sd * 0.72
        yaw = math.atan2(nose_x(0.92, z + 0.01) - nose_x(0.92, z - 0.01), 0.02)
        b.append(ell(f'headlamp-{st}', END, 'glass', 'glazing', 0.93, [0.024, 0.07, 0.13], [nose_x(0.92, z), 0.92, z], rot=qy(-yaw)))
    b.append(box('destination-sign', END, 'glass', 'glazing', 0.08, [0.03, 0.2, 1.0], [nose_x(2.80, 0.3) + 0.002, 2.80, 0], rot=qz(math.atan(TANR)), corner=0.008))
    return b


def body100():
    b = car100('end', E0, XC, (BOGIE_R, BOGIE_F), DOOR_A, True) + car100('mid', M0, M1, (), DOOR_B, False)
    b += ends100() + cab100() + bellows(100)
    b += bogie_masses(BOGIE_R, 'r', False, 100) + bogie_masses(BOGIE_F, 'f', True, 100)
    return b + doors(100) + pantograph(100) + roof_units(100)


def inset(pts, t):
    """an open (y, z) polyline that runs from the right foot over the crown to the left foot, moved inward by t with mitred corners; the two ends stay on their height"""
    out = []
    n = len(pts)
    for i in range(n):
        ns = []
        for j, k in ((i - 1, i), (i, i + 1)):
            if 0 <= j and k < n:
                dy, dz = pts[k][0] - pts[j][0], pts[k][1] - pts[j][1]
                m = math.hypot(dy, dz)
                ns.append((-dz / m, dy / m))
        ny, nz = sum(v[0] for v in ns), sum(v[1] for v in ns)
        m = math.hypot(ny, nz)
        ny, nz = ny / m, nz / m
        c = ns[0][0] * ny + ns[0][1] * nz
        d = t / max(c, 0.5)
        y, z = pts[i][0] + ny * d, pts[i][1] + nz * d
        if i in (0, n - 1):
            y = pts[i][0]
        out.append([y, z])
    return out


def roof_shell(t, lean):
    """the roof cap as a shell of the thickness t: outer section and its inset, as (y, z)"""
    o = roof_section(CANT, lean)
    return o, inset(o, t)


def plumb_section():
    """the generalised car body section of 1:200 and 1:500 from the skirt over the roof, as (y, z)"""
    r = roof_section(CANT, False)
    return [[SKIRT, -HW]] + r + [[SKIRT, HW]]


def module_shell(tag, x0, x1, bogies, door, raked, t, scale):
    """1:200 and 1:500 module as a closed shell of the thickness t: lower side panels with the bogie openings and the door gap, plumb upper panels with the raked cab edge, roof cap, floor with the bogie wells, podests, end walls"""
    w = side(tag)
    b = []
    xu = x1 - (TANR * (CANT - BELT) if raked else 0.0)
    for sd, st in ((1, 'l'), (-1, 'r')):
        dr = door if sd < 0 else None
        zz = sorted((sd * HW, sd * (HW - t)))
        for k, prof in enumerate(F.lower_profiles(x0, x1, bogies, dr)):
            b.append(w(ext(f'{tag}-wall-lower-{st}' + (f'-{"ab"[k]}' if dr else ''), END, 'paint', 'body', PAINT, 'xy', prof, *zz)))
        up = [[x0, BELT]]
        if dr:
            a, c = dr[0] + 0.01, dr[1] - 0.01
            up += [[a, BELT], [a, DOOR_Y[1] - 0.01], [c, DOOR_Y[1] - 0.01], [c, BELT]]
        up += [[x1, BELT], [xu, CANT], [x0, CANT]]
        b.append(w(ext(f'{tag}-wall-upper-{st}', END, 'paint', 'body', PAINT, 'xy', up, *zz)))
    xr = x1 - (TANR * (ROOF - BELT) - 0.04 if raked else 0.0)
    o, n_ = roof_shell(t, False)
    b.append(w(ext(f'{tag}-roof', END, 'paint', 'body', PAINT + 0.05, 'yz', o + list(reversed(n_)), x0, xr)))
    hwf = HW - t
    half = [[-hwf, x0]]
    for xb in bogies:
        half += [[-hwf, xb - 1.26], [-0.62, xb - 1.26], [-0.62, min(xb + 1.26, x1)]] + ([[-hwf, xb + 1.26]] if xb + 1.26 < x1 else [])
    half += [[-hwf, x1]] if half[-1][1] < x1 else []
    b.append(w(ext(f'{tag}-floor', END, 'paint', 'body', 0.5, 'zx', half + [[-z, x] for z, x in reversed(half)], SKIRT, SKIRT + t)))
    for xb, bt in zip(bogies, 'rf'):
        xe = min(xb + 1.30, x1)
        for sd, st in ((1, 'l'), (-1, 'r')):
            prof = [[sd * 0.62, SKIRT], [sd * (0.62 + t), SKIRT], [sd * (0.62 + t), 0.74 - t], [sd * hwf, 0.74 - t], [sd * hwf, 0.74], [sd * 0.62, 0.74]]
            b.append(ext(f'well-{bt}-{st}', END, 'paint', 'body', 0.45, 'yz', [[y, z] for z, y in prof], xb - 1.30, xe))
    sec_ = plumb_section()
    if tag == 'end':
        b.append(ext('end-rear-wall', END, 'paint', 'body', PAINT - 0.1, 'yz', sec_, x0, x0 + t))
    else:
        b.append(w(ext('mid-front-wall', END, 'paint', 'body', PAINT - 0.1, 'yz', sec_, x1 - t, x1)))
        b.append(w(ext('mid-rear-wall', END, 'paint', 'body', PAINT - 0.1, 'yz', sec_, x0, x0 + t)))
    return b


def glazing_coarse(tag, panes_r, panes_l, raked, proud):
    """panes set flush on the plumb walls, standing out by `proud`"""
    w = side(tag)
    b = []
    for sd, st, panes in ((1, 'l', panes_l), (-1, 'r', panes_r)):
        zz = sorted((sd * (HW - 0.002), sd * (HW + proud)))
        for n, a, c in panes:
            b.append(w(slab(f'pane-{n}-{st}', END, 'glass', 'glazing', GLASS, a, c, SILL, HEAD, *zz)))
        if raked:
            b.append(ext(f'pane-cab-{st}', END, 'glass', 'glazing', GLASS, 'xy', F.cab_pane(), *zz))
    return b


def body200():
    b = []
    for tag, x0, x1, bogies, raked in (('end', E0, XC, (BOGIE_R, BOGIE_F), True), ('mid', M0, M1, (), False)):
        b += module_shell(tag, x0, x1, bogies, DOOR_A if tag == 'end' else DOOR_B, raked, 0.06, 200)
        b += glazing_coarse(tag, RIGHT_200[tag], LEFT_200[tag], raked, 0.008)
    b += cab_masses(200, 2, [GLASS_TOP, 3.0, 3.25, 3.37, 3.395])
    b.append(ext('bumper', END, 'rubber', 'detail', 0.3, 'zx', dband(NOSE_D + BUMP, HW, 0.08, XC), 0.45, 0.66))
    for sd, st in ((1, 'l'), (-1, 'r')):
        b.append(ell(f'headlamp-{st}', END, 'glass', 'glazing', 0.93, [0.03, 0.07, 0.13], [nose_x(0.92, sd * 0.72), 0.92, sd * 0.72]))
    b += bellows(200) + bogie_masses(BOGIE_R, 'r', False, 200) + bogie_masses(BOGIE_F, 'f', True, 200)
    return b + doors(200) + pantograph(200) + roof_units(200)


def body500():
    b = []
    for tag, x0, x1, bogies, raked in (('end', E0, XC, (BOGIE_R, BOGIE_F), True), ('mid', M0, M1, (), False)):
        b += module_shell(tag, x0, x1, bogies, DOOR_A if tag == 'end' else DOOR_B, raked, 0.10, 500)
        w = side(tag)
        for sd, st in ((1, 'l'), (-1, 'r')):
            a, c = (0.80, 7.47) if tag == 'end' else (-5.85, -0.85)
            if sd < 0:
                spans = [(a, DOOR_A[0] - 0.08), (DOOR_A[1] + 0.08, c)] if tag == 'end' else [(a, DOOR_B[0] - 0.08), (DOOR_B[1] + 0.08, c)]
            else:
                spans = [(a, c)]
            for k, (p, q) in enumerate(spans):
                b.append(w(slab(f'band-{tag}-{st}-{k + 1}', END, 'glass', 'glazing', GLASS, p, q, SILL, HEAD, *sorted((sd * HW, sd * (HW + 0.012))))))
    b += cab_masses(500, 2, [GLASS_TOP, 3.15, 3.395])
    b += bellows(500) + bogie_masses(BOGIE_R, 'r', False, 500) + bogie_masses(BOGIE_F, 'f', True, 500)
    return b + doors(500) + pantograph(500) + roof_units(500)


def thermal_coarse(scale, body):
    """thermal list of a coarse scale: references to the body solids by region, inner faces of the hollow 1:100 shell as section faces, the two outlet patches"""
    t = []
    of = F.of
    for s in body:
        i, bone = s["id"], s["bone"]
        mid = bone == MID
        left = i.endswith(('-l', '-l-a', '-l-b')) or '-l-' in i
        if '-wall-lower-' in i or '-wall-upper-' in i:
            t.append(of(f'th-{i}', i, 'body-panels', None, ['-w' if left else '+w']))
        elif i.endswith('-floor'):
            t.append(of(f'th-{i}', i, 'body-panels', None, ['-v' if mid else '+v']))
        elif i.startswith('well-'):
            t.append(of(f'th-{i}', i, 'body-panels', None, ['+v', '-w' if left else '+w']))
        elif i.endswith('-roof'):
            o, n_ = (roof_outline(), roof_outline(TW)) if scale == 100 else roof_shell({200: 0.06, 500: 0.10}[scale], False)
            inner_v = '+v' if mid else '-v'
            for part, a, c, face in (('r', 0, 4, '+w'), ('crown', 4, 10, inner_v), ('l', 10, 14, '-w')):
                t.append(F.th(s, f'th-{i}-{part}', 'roof', None, None, [face], profile=[[rr(p), rr(q)] for p, q in o[a:c + 1] + list(reversed(n_[a:c + 1]))]))
        elif i in ('end-rear-wall', 'mid-front-wall'):
            t.append(of(f'th-{i}', i, 'body-panels', None, ['+u']))
        elif i == 'mid-rear-wall':
            t.append(of(f'th-{i}', i, 'body-panels', None, ['-u', '+u']))
        elif i == 'cab-nose':
            t.append(of(f'th-{i}', i, 'body-panels', None, ['-u', '+v']))
        elif i == 'cab-header':
            t.append(of(f'th-{i}', i, 'body-panels', None, ['-u', '-v']))
        elif i in ('cab-sill', 'bumper') or i.startswith('a-pillar'):
            t.append(of(f'th-{i}', i, 'body-panels'))
        elif i == 'windscreen':
            t.append(of(f'th-{i}', i, 'glazing', None, ['-u', '-v', '+v']))
        elif i == 'destination-sign':
            t.append(of(f'th-{i}', i, 'glazing', None, ['-u']))
        elif i.startswith(('pane-', 'band-')):
            t.append(of(f'th-{i}', i, 'glazing', None, ['-w' if left else '+w']))
        elif i.startswith('leaf-glass-'):
            t.append(of(f'th-{i}', i, 'glazing', None, ['+w']))
        elif i.startswith('leaf-'):
            t.append(of(f'th-{i}', i, 'doors', None, ['+w']))
        elif i.startswith('bellows'):
            back = {'bellows-l': ['-w'], 'bellows-r': ['+w'], 'bellows-top': ['-v'], 'bellows-bottom': ['+v']}.get(i, ['-u', '+u'])
            t.append(of(f'th-{i}', i, 'gangway', None, back))
        elif i.startswith(('motor-', 'gearbox-')):
            t.append(of(f'th-{i}', i, 'traction-motors'))
        elif i.startswith(('wheel-', 'stub-axle-')):
            t.append(of(f'th-{i}', i, 'wheelsets'))
        elif i.startswith('brake-disc-'):
            t.append(of(f'th-{i}', i, 'brake-discs'))
        elif i.startswith('resistor-'):
            t.append(of(f'th-{i}', i, 'brake-resistor'))
        elif i == 'hvac-box':
            t.append(of(f'th-{i}', i, 'hvac-unit', None, ['+v']))
        elif i.startswith('hvac-fan'):
            t.append(of(f'th-{i}', i, 'hvac-unit'))
        elif i.startswith('pan-'):
            t.append(of(f'th-{i}', i, 'pantograph'))
    if scale > 100:
        for bt, xb in (('r', BOGIE_R), ('f', BOGIE_F)):
            if bt == 'f':
                for sd, st in ((1, 'l'), (-1, 'r')):
                    if scale == 500:
                        m = cyl(f'th-motor-f-{st}', END, 'motor-steel', 'body', 0.25, [xb - 0.45, 0.3, sd * 0.90], (1, 0, 0), 0.9, 0.17, region='traction-motors')
                        t.append(on(m, 'bogie-f', (xb, 0.3)))
            for k, dx in enumerate((-WB, WB)):
                wb = f'ws-{bt}{k + 1}'
                for sd, st in ((1, 'l'), (-1, 'r')):
                    d = rev(f'th-brake-disc-{wb}-{st}', END, 'cast-iron', 'detail', 0.55, [[0.0, 0.09], [0.0, 0.2], [0.03, 0.2], [0.03, 0.09]] if scale == 200 else [[0.0, 0.0], [0.0, 0.2], [0.075, 0.2], [0.075, 0.0]], [xb + dx, 0.3, sd * (Z_BACK - (0.035 if scale == 200 else 0.08))], (0, 0, sd), region='brake-discs')
                    t.append(on(d, wb, (xb + dx, 0.3)))
    x0, x1, hz, h, _ = RESISTOR
    t.append(cyl('resistor-fan-outlet', END, 'resistor-steel', 'detail', 0.9, [5.5, ROOF + h, 0], (0, 1, 0), {100: 0.015, 200: 0.03, 500: 0.075}[scale], 0.35, region='brake-resistor', tags=['emitter', 'resistor-fan']))
    v = F.mid(cyl('hvac-vent-outlet', END, 'paint', 'detail', 0.9, [-3.5, ROOF + HVAC[3], 0], (0, 1, 0), {100: 0.015, 200: 0.03, 500: 0.075}[scale], 0.45))
    v.update(region='hvac-unit', tags=['emitter', 'hvac-vent'])
    t.append(v)
    return t


def frame_coarse(tag, scale, x0, x1, bogies, door, raked):
    """the car body frame of a coarse scale: side sills stepped over the bogie wells, cant rails, door and end posts, a few roof bows, the end rings"""
    w = side(tag)
    bone = END if tag == 'end' else MID
    m = lambda s: dict(w(s), member=bone, part='member')
    sec = {100: 0.07, 200: 0.08, 500: 0.10}[scale]
    W = hw if scale == 100 else plumb
    out = []
    for sd, st in ((1, 'l'), (-1, 'r')):
        za = sorted((sd * (HW - 0.06), sd * (HW - 0.06 - sec)))
        h = sec + 0.04
        line, top = [[x0, SKIRT]], [[x0, SKIRT + h]]
        for xb in bogies:
            a, c = xb - 1.3, min(xb + 1.3, x1)
            line += [[a, SKIRT], [a, 0.70]]
            top += [[a - h, SKIRT + h], [a - h, 0.70 + h]]
            if c < x1:
                line += [[c, 0.70], [c, SKIRT]]
                top += [[c + h, 0.70 + h], [c + h, SKIRT + h]]
        yl = line[-1][1]
        line.append([x1, yl])
        top.append([x1, yl + h])
        out.append(m(ext(f'{tag}-sill-{st}', END, 'paint', 'body', 0.55, 'xy', F.dedupe(line) + list(reversed(F.dedupe(top))), za[0], za[1])))
        xc = x1 - (TANR * (CANT - 0.05 - BELT) if raked else 0.0)
        zc = sorted((sd * (W(CANT - 0.05) - 0.03), sd * (W(CANT - 0.05) - 0.03 - sec)))
        out.append(m(slab(f'{tag}-cant-rail-{st}', END, 'paint', 'body', 0.55, x0, xc, CANT - 0.05 - sec, CANT - 0.05, zc[0], zc[1])))
        posts = [x0 + 0.05] + ([door[0] - 0.05, door[1] + 0.05] if door and sd < 0 else []) + ([] if raked else [x1 - 0.05])
        if scale < 500:
            posts += [(F.POSTS[(tag, sd)][k]) for k in (1, 2) if not (door and sd < 0 and any(abs(F.POSTS[(tag, sd)][k] - d) < 0.3 for d in door))]
        for k, x in enumerate(sorted(set(round(p, 3) for p in posts))):
            base = 0.70 + sec + 0.04 if any(abs(x - xb) < 1.3 for xb in bogies) else SKIRT + sec
            out.append(m(bar(f'{tag}-post-{st}-{k + 1}', END, 'paint', 'body', 0.55, [x, base, sd * (HW - 0.06 - sec / 2)], [x, CANT - 0.05, sd * (W(CANT - 0.05) - 0.03 - sec / 2)], sec, sec)))
    bows = {100: 4, 200: 3, 500: 2}[scale]
    if scale == 100:
        bow = roof_outline(TW) + list(reversed(roof_outline(TW + sec)))
    else:
        o = roof_section(CANT, False)
        t = {200: 0.06, 500: 0.10}[scale]
        o = [o[k] for k in (0, 1, 3, 5, 7, 9, 11, 13, 14)]
        bow = inset(o, t) + list(reversed(inset(o, t + sec)))
    xr = x1 - (TANR * (ROOF - BELT) if raked else 0.0)
    for k in range(bows):
        x = x0 + 0.1 + (xr - x0 - 0.2) * k / (bows - 1)
        out.append(m(ext(f'{tag}-roof-bow-{k + 1}', END, 'paint', 'body', 0.55, 'yz', bow, x - sec / 2, x + sec / 2)))
    xs = [x0 + 0.05 + (x1 - x0 - 0.1) * k / 3 for k in range(4)] if scale < 500 else [(x0 + x1) / 2]
    for k, x in enumerate(xs):
        out.append(m(slab(f'{tag}-cross-bearer-{k + 1}', END, 'paint', 'body', 0.55, x - sec / 2, x + sec / 2, SKIRT + 0.002, FLOOR - 0.002 if scale < 500 else SKIRT + sec, -(HW - 0.06), HW - 0.06)))
    ends = [x0] if raked else [x0, x1 - sec]
    for k, x in enumerate(ends):
        out.append(m(ext(f'{tag}-end-ring-{k + 1}', END, 'paint', 'body', 0.55, 'yz', rect(SKIRT, -(HW - 0.06), CANT, HW - 0.06), x, x + sec, holes=[rect(SKIRT + sec, -(HW - 0.06 - sec), CANT - sec, HW - 0.06 - sec)])))
    return out


def structure_coarse(scale, body):
    s = frame_coarse('end', scale, E0, XC, (BOGIE_R, BOGIE_F), DOOR_A, True) + frame_coarse('mid', scale, M0, M1, (), DOOR_B, False)
    e = lambda q, bone=END, part='member': dict(q, member=bone, part=part)
    sec = {100: 0.07, 200: 0.08, 500: 0.10}[scale]
    s.append(e(ext('cab-ring', END, 'paint', 'body', 0.55, 'zx', dband(NOSE_D - 0.03, HW - 0.03, sec, XC), BELT - sec - 0.01, BELT - 0.01)))
    for sd, st in ((1, 'l'), (-1, 'r')):
        z = sd * (0.8 * HW - 0.05)
        s.append(e(bar(f'cab-post-{st}', END, 'paint', 'body', 0.55, [nose_x(BELT, z) - 0.07, BELT, z], [nose_x(GLASS_TOP, z * hw(GLASS_TOP) / HW) - 0.07, GLASS_TOP, z * (hw(GLASS_TOP) / HW if scale == 100 else 1)], sec, sec)))
    names = {x["id"] for x in body}
    g = lambda q, part='member': dict(on(q, 'gangway', (0, 0.3)), member='gangway', part=part)
    if 'articulation-arm' in names:
        s.append({"id": 'st-articulation-arm', "of": 'articulation-arm', "member": 'gangway', "part": 'member'})
    else:
        s.append(g(slab('articulation-link', END, 'steel-wheel', 'body', 0.35, -0.44, 0.44, 0.32, 0.42, -0.15, 0.15)))
    s.append(g(slab('articulation-upper', END, 'steel-wheel', 'body', 0.35, -0.5, 0.5, 3.04, 3.04 + sec, -sec, sec)))
    s.append(g(cyl('articulation-pin', END, 'steel-wheel', 'detail', 0.2, [0, 0.165 if scale < 500 else 0.31, 0], (0, 1, 0), 0.12, 0.08), 'joint'))
    for bt, xb in (('r', BOGIE_R), ('f', BOGIE_F)):
        bone = f'bogie-{bt}'
        for b in body:
            i = b["id"]
            if i.startswith((f'bogie-beam-{bt}', f'bogie-frame-{bt}', f'bogie-cross-{bt}', f'bogie-leg-{bt}')):
                s.append({"id": f'st-{i}', "of": i, "member": bone, "part": 'member'})
        p = cyl(f'pivot-{bt}', END, 'steel-wheel', 'detail', 0.2, [xb, 0.24, 0], (0, 1, 0), 0.04 if scale < 500 else 0.08, 0.16)
        s.append(dict(on(p, bone, (xb, 0.3)), member=bone, part='joint'))
        for k, dx in enumerate((-WB, WB)):
            wb = f'ws-{bt}{k + 1}'
            for sd, st in ((1, 'l'), (-1, 'r')):
                if f'stub-axle-{wb}-{st}' in names:
                    s.append({"id": f'st-stub-axle-{wb}-{st}', "of": f'stub-axle-{wb}-{st}', "member": wb, "part": 'member'})
                else:
                    ax = cyl(f'stub-axle-{wb}-{st}', END, 'steel-wheel', 'body', 0.35, [xb + dx, 0.3, sd * 0.56], (0, 0, sd), 0.16, 0.0375)
                    s.append(dict(on(ax, wb, (xb + dx, 0.3)), member=wb, part='member'))
                s.append({"id": f'st-wheel-{wb}-{st}', "of": f'wheel-{wb}-{st}', "member": wb, "part": 'member'})
        s.append(dict(ell(f'mass-{bone}', bone, 'steel-wheel', 'detail', 0.6, [0.22, 0.04 if scale < 500 else 0.075, 0.28], [0, 0, 0]), member=bone, part='mass'))
    for i in sorted(names):
        if i.startswith('pan-') and ('arm' in i or i.startswith(('pan-head-bar', 'pan-strip', 'pan-frame', 'pan-base'))):
            bone = next(b["bone"] for b in body if b["id"] == i)
            s.append({"id": f'st-{i}', "of": i, "member": bone, "part": 'member'})
    for bone, r, half in (('pan-lower', 0.035 if scale < 500 else 0.0375, 0.38), ('pan-upper', 0.035 if scale < 500 else 0.0375, 0.08), ('pan-head', 0.016, 0.3)):
        if bone == 'pan-head' and scale == 500:
            continue
        s.append(dict(cyl(f'hinge-{bone}', bone, 'pan-alloy', 'detail', 0.2, [0, 0, -half], (0, 0, 1), 2 * half, r), member=bone, part='joint'))
    for name, _ in DOOR_NAMES:
        s.append({"id": f'st-leaf-{name}', "of": f'leaf-{name}', "member": f'door-{name}', "part": 'member'})
    s.append(dict(ell('mass-body-end', END, 'steel-wheel', 'detail', 0.6, [0.35, 0.25, 0.35], [5.0, 1.5, 0]), member=END, part='mass'))
    s.append(dict(ell('mass-body-mid', MID, 'steel-wheel', 'detail', 0.6, [0.3, 0.22, 0.3], [3.2, -1.2, 0]), member=MID, part='mass'))
    if scale < 500:
        s.append(dict(ell('mass-gangway', 'gangway', 'steel-wheel', 'detail', 0.6, [0.1, 0.1, 0.1], [0, 1.4, 0]), member='gangway', part='mass'))
    return s


DECISIONS = {
    100: [
        {"en": "1:100 keeps the built shell at 15 mm: 51.5 mm walls with 40 mm of tumblehome, the window openings merged where the 1:100 drawing merges them (the second and third cab-module windows on the right, the first two middle-module windows on the left), flush 20 mm panes, roof with rolled cant rails, floor with the bogie wells and one podest profile per well; the cab keeps its superelliptic nose, raked windscreen, domed header, sill band, bumper, A-pillars, headlamps and destination sign; wipers, indicators, coupler hatch, seals and buttons are left out.", "de": "1:100 behält den gebauten Kasten bei 15 mm: 51,5 mm starke Wände mit 40 mm Einzug, Fensteröffnungen zusammengelegt, wo die 1:100-Zeichnung sie zusammenlegt (zweites und drittes Fenster des Endmoduls rechts, die ersten beiden Fenster des Mittelmoduls links), bündige 20-mm-Scheiben, Dach mit gerundeten Obergurten, Boden mit den Drehgestellnischen und je einem Podestprofil; der Fahrerstand behält superelliptische Nase, geneigte Frontscheibe, gewölbtes Stirndach, Brüstungsband, Stoßfänger, A-Säulen, Scheinwerfer und Zielanzeige; Wischer, Blinker, Kupplungsklappe, Dichtungen und Taster entfallen."},
        {"en": "Bogies at 1:100: side beams on two legs, one cross member, magnetic track brakes, wheels with flange, tread and recessed web, axles and brake discs, on the powered bogie a longitudinal motor and two gearboxes per side, on the trailing bogie a brake unit per side; the secondary springs stay on the beams, calipers, primary springs and brackets merge into them; wheels sit on stub axles as in a real independent-wheel low-floor bogie. The bellows keep nine folds; the pantograph keeps insulators, frame, arms with knee shaft and brace and a two-strip head with apex and horns.", "de": "Drehgestelle in 1:100: Langträger auf zwei Beinen, ein Querträger, Magnetschienenbremsen, Räder mit Spurkranz, Lauffläche und eingezogener Scheibe, Achsen und Bremsscheiben, am Triebdrehgestell je Seite ein Längsmotor und zwei Getriebe, am Laufdrehgestell je Seite ein Bremsgerät; die Sekundärfedern bleiben auf den Trägern, Zangen, Primärfedern und Konsolen gehen in ihnen auf; die Räder sitzen wie in einem echten Losradfahrwerk auf Achsstummeln. Der Balg behält neun Falten, der Stromabnehmer Isolatoren, Rahmen, Arme mit Kniewelle und Strebe und eine Wippe mit zwei Schleifleisten, Scheitelrahmen und Hörnern."},
        {"en": "Thermal: one reference per part and region, the cabin faces of walls, floor, roof, doors and panes as section faces, the bellows as one folded skin, the two fan outlets as patches on the roof units; layers are merged into their plate (B2). Structure: stepped side sills, cant rails, door and end posts, four roof bows and four cross-bearers per module, end rings, the cab ring and posts, articulation links and pin, bogie frames with pivots, axles and wheels, pantograph frame, arms and hinges, door leaves, masses.", "de": "Thermik: eine Referenz je Teil und Bereich, die Innenraumseiten von Wänden, Boden, Dach, Türen und Scheiben als Schnittflächen, der Balg als eine gefaltete Haut, die beiden Lüfteraustritte als Flächen auf den Dachgeräten; Schichten gehen in ihrer Platte auf (B2). Tragwerk: gestufte Seitenschweller, Obergurte, Tür- und Endsäulen, vier Dachspriegel und vier Querträger je Modul, Endrahmen, Fahrerstandsring und -säulen, Gelenkverbindungen und -zapfen, Drehgestellrahmen mit Drehzapfen, Achsen und Räder, Stromabnehmerrahmen, -arme und -gelenke, Türflügel, Massen."},
    ],
    200: [
        {"en": "1:200 models each module as masses at 30 mm: a lower block from the skirt to the belt with the bogie openings, plumb walls up to the cant (the 40 mm tumblehome is under 1.5 features and dropped), a roof cap with the cant curve, end walls; windows as 8 mm proud panes (the left windows of the middle module merge into one band as drawn), the cab as nose block, raked windscreen and domed header with bumper and headlamps.", "de": "1:200 bildet jedes Modul bei 30 mm als Massen: ein Unterkasten von der Schürze bis zur Brüstung mit den Drehgestellöffnungen, lotrechte Wände bis zum Obergurt (der 40-mm-Einzug liegt unter 1,5 Merkmalen und entfällt), eine Dachkappe mit der Obergurtrundung, Stirnwände; Fenster als 8 mm vorstehende Scheiben (die linken Fenster des Mittelmoduls zu einem Band zusammengelegt), der Fahrerstand als Nasenblock, geneigte Frontscheibe und gewölbtes Stirndach mit Stoßfänger und Scheinwerfern."},
        {"en": "Bogies keep side beams, legs, cross member, track brakes, motors with gearboxes or brake units, wheels with flange and tread and the axles; the bellows have four folds; door leaves are solid panels with an applied pane; the pantograph keeps insulators, frame, arms and a one-piece head with horns; roof units are plain boxes.", "de": "Drehgestelle behalten Langträger, Beine, Querträger, Schienenbremsen, Motoren mit Getrieben oder Bremsgeräte, Räder mit Spurkranz und Lauffläche und die Achsen; der Balg hat vier Falten; Türflügel sind volle Tafeln mit aufgesetzter Scheibe; der Stromabnehmer behält Isolatoren, Rahmen, Arme und eine einteilige Wippe mit Hörnern; Dachgeräte sind einfache Kästen."},
        {"en": "Thermal: references by region, the brake discs as 30 mm discs inside the wheels (they are thinner than the feature in the body), the outlet patches on the roof units. Structure: stepped sills, cant rails, posts at the doors and ends plus two window posts per side, three roof bows and four cross-bearers per module, end rings, cab ring and posts, articulation, bogie frames, axles, wheels, pantograph, door leaves, masses.", "de": "Thermik: Referenzen je Bereich, die Bremsscheiben als 30-mm-Scheiben in den Rädern (im Körper sind sie dünner als das Merkmal), die Austrittsflächen auf den Dachgeräten. Tragwerk: gestufte Schweller, Obergurte, Säulen an Türen und Enden und zwei Fenstersäulen je Seite, drei Dachspriegel und vier Querträger je Modul, Endrahmen, Fahrerstandsring und -säulen, Gelenk, Drehgestellrahmen, Achsen, Räder, Stromabnehmer, Türflügel, Massen."},
    ],
    500: [
        {"en": "1:500 keeps what reads at 75 mm: the two modules as lower block with the bogie openings, plumb walls and a roof cap with the cant curve, the cab nose with raked windscreen and domed header, the bellows as one rounded block, window bands split at the doors, four door panels, the two roof units and the pantograph with its base, two arms and the collector bar.", "de": "1:500 behält, was bei 75 mm lesbar ist: die beiden Module als Unterkasten mit den Drehgestellöffnungen, lotrechte Wände und eine Dachkappe mit Obergurtrundung, die Fahrerstandsnase mit geneigter Frontscheibe und gewölbtem Stirndach, den Balg als einen gerundeten Block, Fensterbänder an den Türen geteilt, vier Türtafeln, die beiden Dachgeräte und den Stromabnehmer mit Sockel, zwei Armen und Schleifleiste."},
        {"en": "Each bogie becomes one frame block per side with the four 600 mm wheels as plain discs; motors, gearboxes, brakes and axles merge into the frame, the brake discs and motors are kept only in the thermal list as compact solids inside the bogie.", "de": "Jedes Drehgestell wird ein Rahmenblock je Seite mit den vier 600-mm-Rädern als einfache Scheiben; Motoren, Getriebe, Bremsen und Achsen gehen im Rahmen auf, Bremsscheiben und Motoren bleiben nur in der Thermikliste als kompakte Körper im Drehgestell."},
        {"en": "Structure at 1:500: one stepped sill and one cant rail per side, door and end posts, two roof bows and one cross-bearer per module, end rings, cab ring and posts, articulation link and pin, bogie frame blocks with pivots, axles and wheels, pantograph base and arms, door panels and the masses.", "de": "Tragwerk in 1:500: ein gestufter Schweller und ein Obergurt je Seite, Tür- und Endsäulen, zwei Dachspriegel und ein Querträger je Modul, Endrahmen, Fahrerstandsring und -säulen, Gelenklenker und -zapfen, Drehgestellrahmenblöcke mit Drehzapfen, Achsen und Räder, Stromabnehmersockel und -arme, Türtafeln und die Massen."},
    ],
}


def scale(n):
    body = {100: body100, 200: body200, 500: body500}[n]()
    return {'body': body, 'thermal': thermal_coarse(n, body), 'structure': structure_coarse(n, body), 'decisions': DECISIONS[n]}
