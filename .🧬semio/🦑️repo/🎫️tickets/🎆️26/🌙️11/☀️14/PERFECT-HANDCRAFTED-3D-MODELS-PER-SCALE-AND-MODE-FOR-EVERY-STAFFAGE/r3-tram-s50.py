"""🚃️ Round 3 tram: the 1:50 body, stated assembly by assembly (car bodies with real openings, cab, bellows, bogies, doors, pantograph, roof equipment)."""
import copy, importlib.util, math, os

_spec = importlib.util.spec_from_file_location('r3_tram_kit', os.path.join(os.path.dirname(os.path.abspath(__file__)), 'r3-tram-kit.py'))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
globals().update({k: v for k, v in vars(K).items() if not k.startswith('_')})

PAINT, GLASS, TRIM, DARK, STEEL, LAMP = 0.8, 0.28, 0.2, 0.12, 0.4, 0.93
S0, S1 = s_of(SILL), s_of(HEAD)
S_DOOR = s_of(DOOR_Y[1] - 0.01)
PANE_T, RECESS = 0.024, 0.006
END, MID = 'body-end', 'body-mid'


def dedupe(p):
    out = []
    for q in p:
        if not out or math.hypot(q[0] - out[-1][0], q[1] - out[-1][1]) > 0.004:
            out.append(q)
    return out


def lower_profiles(x0, x1, bogies, door):
    """side elevation polygons of the lower side panel between the skirt line and the belt: bogie openings cut in, split at the door"""
    bottom = [[x0, SKIRT]]
    for xb in bogies:
        bottom += cutout(xb)
    bottom += [[x1, SKIRT]]
    bottom = dedupe(bottom)
    if not door:
        return [bottom + [[x1, BELT], [x0, BELT]]]
    a, b = door[0] + 0.01, door[1] - 0.01
    aft = [q for q in bottom if q[0] < a] + [[a, SKIRT], [a, BELT], [x0, BELT]]
    fore = [[b, SKIRT]] + [q for q in bottom if q[0] > b] + [[x1, BELT], [b, BELT]]
    return [aft, fore]


def upper_profile(x0, x1, door, raked):
    top = x1 - (TANR * (CANT - BELT) if raked else 0.0)
    p = [[x0, 0]]
    if door:
        a, b = door[0] + 0.01, door[1] - 0.01
        p += [[a, 0], [a, S_DOOR], [b, S_DOOR], [b, 0]]
    return p + [[x1, 0], [top, S_TOP], [x0, S_TOP]]


def cab_pane():
    """the cab side pane behind the raked edge, as (x, y)"""
    return [[7.62, SILL], [edge(SILL) - 0.11, SILL], [edge(HEAD) - 0.11, HEAD], [7.62, HEAD]]


def car(tag, bone, x0, x1, bogies, door, panes_r, panes_l, raked):
    w = (lambda s: s) if bone == END else mid
    b = []
    for side, st, dr, panes in ((1, 'l', None, panes_l), (-1, 'r', door, panes_r)):
        z0, z1 = (HW - TW, HW) if side > 0 else (-HW, -HW + TW)
        for k, prof in enumerate(lower_profiles(x0, x1, bogies, dr)):
            b.append(w(ext(f'{tag}-side-lower-{st}' + ('' if not dr else '-ab'[k:k + 1] and f'-{"ab"[k]}'), END, 'paint', 'body', PAINT, 'xy', prof, z0, z1)))
        holes = [rect(a, S0, c, S1) for _, a, c in panes]
        if raked:
            holes.append([[x, s_of(y)] for x, y in cab_pane()])
        zc = (-TW, 0) if side > 0 else (0, TW)
        b.append(w(ext(f'{tag}-side-upper-{st}', END, 'paint', 'body', PAINT, 'xy', upper_profile(x0, x1, dr, raked), zc[0], zc[1], holes=holes, at=[0, BELT, side * HW], rot=qx(-side * PHI))))
        sm = (S0 + S1) / 2
        for n, a, c in panes:
            b.append(w(box(f'pane-{n}-{st}', END, 'glass', 'glazing', GLASS, [c - a + 0.02, S1 - S0 + 0.02, PANE_T], on_wall(side, (a + c) / 2, sm, -side * (RECESS + PANE_T / 2)), rot=qx(-side * PHI))))
        if raked:
            cp = cab_pane()
            grown = [[cp[0][0] - 0.01, s_of(SILL) - 0.01], [cp[1][0] + 0.012, s_of(SILL) - 0.01], [cp[2][0] + 0.012, s_of(HEAD) + 0.01], [cp[3][0] - 0.01, s_of(HEAD) + 0.01]]
            zp = (-RECESS - PANE_T, -RECESS) if side > 0 else (RECESS, RECESS + PANE_T)
            b.append(ext(f'pane-cab-{st}', END, 'glass', 'glazing', GLASS, 'xy', grown, zp[0], zp[1], at=[0, BELT, side * HW], rot=qx(-side * PHI)))
    xr = x1 - (TANR * (ROOF - BELT) - 0.06 if raked else 0.0)
    b.append(w(ext(f'{tag}-roof', END, 'paint', 'body', PAINT + 0.05, 'yz', roof_outline() + list(reversed(roof_outline(TW))), x0, xr)))
    hwf = HW - 0.002
    half = [[-hwf, x0]]
    for xb in bogies:
        half += [[-hwf, xb - 1.26], [-0.62, xb - 1.26], [-0.62, min(xb + 1.26, x1)]] + ([[-hwf, xb + 1.26]] if xb + 1.26 < x1 else [])
    half += [[-hwf, x1]] if half[-1][1] < x1 else []
    b.append(w(ext(f'{tag}-floor', END, 'paint', 'body', 0.5, 'zx', half + [[-z, x] for z, x in reversed(half)], SKIRT, FLOOR)))
    for xb, bt in zip(bogies, 'rf'):
        for side, st in ((1, 'l'), (-1, 'r')):
            za, zb = sorted((side * 0.60, side * (HW - TW + 0.004)))
            zi = sorted((side * 0.60, side * 0.64))
            b.append(slab(f'podest-top-{bt}-{st}', END, 'paint', 'body', 0.5, xb - 1.30, min(xb + 1.30, x1), 0.70, 0.74, za, zb))
            b.append(slab(f'podest-wall-{bt}-{st}', END, 'paint', 'body', 0.35, xb - 1.30, min(xb + 1.30, x1), SKIRT, 0.74, zi[0], zi[1]))
            b.append(slab(f'podest-end-{bt}-{st}-a', END, 'paint', 'body', 0.35, xb - 1.30, xb - 1.26, SKIRT, 0.74, za, zb))
            if xb + 1.30 < x1:
                b.append(slab(f'podest-end-{bt}-{st}-f', END, 'paint', 'body', 0.35, xb + 1.26, xb + 1.30, SKIRT, 0.74, za, zb))
    return b


def end_walls():
    b = []
    portal = rect(PORTAL[1], -PORTAL[0], PORTAL[2], PORTAL[0])
    b.append(ext('end-rear-wall', END, 'paint', 'body', PAINT - 0.1, 'yz', section(), E0, E0 + 0.05, holes=[portal]))
    b.append(mid(ext('mid-front-wall', END, 'paint', 'body', PAINT - 0.1, 'yz', section(), M1 - 0.05, M1, holes=[portal])))
    b.append(mid(ext('mid-rear-wall', END, 'paint', 'body', PAINT - 0.1, 'yz', section(), M0, M0 + 0.05, holes=[portal])))
    b.append(mid(slab('mid-rear-blank', END, 'rubber', 'detail', DARK, M0 + 0.012, M0 + 0.045, PORTAL[1] - 0.01, PORTAL[2] + 0.01, -PORTAL[0] - 0.01, PORTAL[0] + 0.01)))
    return b


def cab():
    b = []
    b.append(ext('cab-nose', END, 'paint', 'body', PAINT, 'zx', dfill(NOSE_D, HW, XC), SKIRT, BELT))
    b.append(ext('bumper', END, 'rubber', 'detail', 0.3, 'zx', dband(NOSE_D + BUMP, HW, 0.07, XC), 0.46, 0.66))
    ys = [BELT, 1.65, 2.15, GLASS_TOP]
    b.append(loft('windscreen', END, 'glass', 'glazing', GLASS, [ring(y, edge(y), tip(y) - edge(y), hw(y) - 0.004) for y in ys]))
    yh = [GLASS_TOP, 2.80, 2.95, 3.10, 3.22, 3.31, 3.37, 3.395]
    b.append(loft('cab-header', END, 'paint', 'body', PAINT, [ring(y, edge(y), tip(y) - edge(y), hw(y) - 0.002) for y in yh]))
    b.append(ext('cab-sill', END, 'paint', 'body', PAINT, 'zx', dband(NOSE_D + 0.004, HW - 0.001, 0.07, XC), BELT - 0.001, BELT + 0.06))
    for side, st in ((1, 'l'), (-1, 'r')):
        f = 0.80
        b.append(loft(f'a-pillar-{st}', END, 'paint', 'body', PAINT, [sec([nose_x(y, side * f * hw(y)) - 0.022, y, side * (f * hw(y) - 0.02)], 0.05, 0.055, 4, QRING) for y in (BELT, 1.9, GLASS_TOP + 0.02)]))
        b.append(loft(f'b-pillar-{st}', END, 'paint', 'body', PAINT, [sec([edge(y) + 0.01, y, side * (hw(y) - 0.027)], 0.075, 0.03, 4, QRING) for y in (BELT, 1.9, GLASS_TOP + 0.02)]))
        z = side * 0.72
        dz = 0.01
        yaw = math.atan2(nose_x(0.92, z + dz) - nose_x(0.92, z - dz), 2 * dz)
        b.append(ell(f'headlamp-{st}', END, 'glass', 'glazing', LAMP, [0.022, 0.07, 0.12], [nose_x(0.92, z), 0.92, z], rot=qy(-yaw)))
        z = side * 0.98
        yaw = math.atan2(nose_x(0.92, z + dz) - nose_x(0.92, z - dz), 2 * dz)
        b.append(ell(f'indicator-{st}', END, 'glass', 'glazing', 0.7, [0.016, 0.05, 0.06], [nose_x(0.92, z), 0.92, z], rot=qy(-yaw)))
    rake = math.atan(TANR)
    b.append(box('destination-sign', END, 'glass', 'glazing', 0.08, [0.03, 0.2, 1.0], [nose_x(2.80, 0.3) + 0.002, 2.80, 0], rot=qz(rake), corner=0.008))
    b.append(box('coupler-hatch', END, 'paint', 'detail', PAINT - 0.12, [0.014, 0.2, 0.52], [TIP + 0.0, 0.82, 0], corner=0.005))
    p0 = [nose_x(1.26, -0.05) + 0.012, 1.26, -0.05]
    p1 = [nose_x(1.26, -0.72) + 0.012, 1.26, -0.72]
    b.append(cap('wiper-blade', END, 'rubber', 'detail', 0.08, [nose_x(1.3, -0.4) + 0.014 - TANR * 0.0, 1.3, -0.4], [nose_x(2.0, -0.33) + 0.014, 2.0, -0.33], 0.009))
    b.append(cap('wiper-arm', END, 'rubber', 'detail', 0.08, [nose_x(1.17, -0.1) + 0.012, 1.17, -0.1], [nose_x(1.65, -0.365) + 0.016, 1.65, -0.365], 0.008))
    return b


def bellows():
    b = []
    zo, zi, t = BELLOWS[0], BELLOWS[0] - 0.08, 0.008
    y0, y1 = BELLOWS[1], BELLOWS[2]
    xs = [-0.45 + 0.05 * k for k in range(19)]
    zig = lambda o, i: [(o if k % 2 == 0 else i) for k in range(19)]
    for side, st in ((1, 'l'), (-1, 'r')):
        outer = [[side * z, x] for z, x in zip(zig(zo, zi), xs)]
        inner = [[side * (z - t * 1.9), x] for z, x in zip(zig(zo, zi), xs)]
        b.append(ext(f'bellows-{st}', END, 'rubber', 'body', 0.16, 'zx', outer + list(reversed(inner)), y0, y1))
    top = [[x, y] for x, y in zip(xs, zig(y1, y1 - 0.08))]
    b.append(ext('bellows-top', END, 'rubber', 'body', 0.2, 'xy', top + [[x, y - t * 1.9] for x, y in reversed(top)], -zo, zo))
    bot = [[x, y] for x, y in zip(xs, zig(y0, y0 + 0.08))]
    b.append(ext('bellows-bottom', END, 'rubber', 'body', 0.12, 'xy', [[x, y + t * 1.9] for x, y in bot] + list(reversed(bot)), -zo, zo))
    b.append(cyl('gangway-turntable', END, 'paint', 'body', 0.45, [0, FLOOR - 0.025, 0], (0, 1, 0), 0.025, 0.72))
    b.append(slab('articulation-arm', END, 'steel-wheel', 'body', STEEL, -0.46, 0.46, 0.20, 0.27, -0.14, 0.14, corner=0.01))
    b.append(cyl('articulation-bearing', END, 'steel-wheel', 'body', STEEL - 0.1, [0, 0.15, 0], (0, 1, 0), 0.14, 0.17))
    return [on(s, 'gangway', (0, 0.3)) for s in b]


def bogie(xb, bt, powered):
    bone = f'bogie-{bt}'
    b = []
    for side, st in ((1, 'l'), (-1, 'r')):
        za, zb = sorted((side * 0.80, side * 1.0))
        b.append(slab(f'bogie-beam-{bt}-{st}', END, 'steel-wheel', 'body', STEEL, xb - 1.12, xb + 1.12, 0.50, 0.62, za, zb, corner=0.02))
        for k, dx in enumerate((-0.3, 0.3)):
            b.append(cyl(f'spring-{bt}-{st}-{k + 1}', END, 'rubber', 'detail', 0.15, [xb + dx, 0.62, side * 0.90], (0, 1, 0), 0.08, 0.09))
        for k, dx in enumerate((-WB, WB)):
            zp = sorted((side * 0.84, side * 0.99))
            b.append(slab(f'primary-{bt}-{st}-{k + 1}', END, 'rubber', 'detail', 0.12, xb + dx - 0.09, xb + dx + 0.09, 0.465, 0.505, zp[0], zp[1]))
            if powered:
                prof = [[0, 0.0], [0, 0.15], [0.02, 0.17], [0.15, 0.17], [0.17, 0.13], [0.17, 0.0]]
                b.append(rev(f'gearbox-{bt}-{st}-{k + 1}', END, 'motor-steel', 'body', 0.3, prof, [xb + dx, 0.3, side * 0.825], (0, 0, side)))
                sg = -1 if dx > 0 else 1
                b.append(cyl(f'pinion-{bt}-{st}-{k + 1}', END, 'motor-steel', 'detail', 0.3, [xb + dx + sg * 0.14, 0.3, side * 0.93], (sg, 0, 0), 0.12, 0.085))
                b.append(cyl(f'coupling-{bt}-{st}-{k + 1}', END, 'steel-wheel', 'detail', 0.45, [xb + dx + sg * 0.24, 0.3, side * 0.93], (sg, 0, 0), 0.23, 0.045))
            else:
                prof = [[0, 0.0], [0, 0.11], [0.015, 0.125], [0.11, 0.125], [0.135, 0.09], [0.135, 0.0]]
                b.append(rev(f'axle-box-{bt}-{st}-{k + 1}', END, 'steel-wheel', 'body', 0.35, prof, [xb + dx, 0.3, side * 0.825], (0, 0, side)))
            sg = -1 if dx > 0 else 1
            zc = sorted((side * 0.765, side * 0.822))
            b.append(slab(f'caliper-{bt}-{st}-{k + 1}', END, 'cast-iron', 'detail', 0.25, xb + dx + sg * 0.26 - 0.05, xb + dx + sg * 0.26 + 0.05, 0.24, 0.36, zc[0], zc[1], corner=0.01))
        if powered:
            prof = [[0, 0.0], [0, 0.15], [0.04, 0.19], [0.86, 0.19], [0.9, 0.15], [0.9, 0.0]]
            b.append(rev(f'motor-{bt}-{st}', END, 'motor-steel', 'body', 0.25, prof, [xb - 0.45, 0.3, side * 0.93], (1, 0, 0)))
            for k, dx in enumerate((-0.3, 0.3)):
                zm = sorted((side * 0.86, side * 1.0))
                b.append(slab(f'motor-bracket-{bt}-{st}-{k + 1}', END, 'steel-wheel', 'detail', STEEL, xb + dx - 0.04, xb + dx + 0.04, 0.44, 0.51, zm[0], zm[1]))
        else:
            zm = sorted((side * 0.82, side * 0.98))
            b.append(slab(f'brake-unit-{bt}-{st}', END, 'cast-iron', 'body', 0.3, xb - 0.32, xb + 0.32, 0.2, 0.505, zm[0], zm[1], corner=0.02))
        zt = sorted((side * 0.705, side * 0.795))
        b.append(slab(f'track-brake-{bt}-{st}', END, 'cast-iron', 'body', 0.2, xb - 0.46, xb + 0.46, 0.04, 0.14, zt[0], zt[1], corner=0.012))
        for k, dx in enumerate((-0.5, 0.5)):
            zu = sorted((side * 0.80, side * 0.875))
            b.append(slab(f'bogie-upright-{bt}-{st}-{k + 1}', END, 'steel-wheel', 'body', STEEL, xb + dx - 0.05, xb + dx + 0.05, 0.12, 0.51, zu[0], zu[1]))
    for k, dx in enumerate((-0.5, 0.5)):
        b.append(slab(f'bogie-cross-{bt}-{k + 1}', END, 'steel-wheel', 'body', STEEL, xb + dx - 0.05, xb + dx + 0.05, 0.12, 0.24, -0.875, 0.875, corner=0.01))
    b = [on(s, bone, (xb, 0.3)) for s in b]
    for k, dx in enumerate((-WB, WB)):
        wb = f'ws-{bt}{k + 1}'
        w = []
        for side, st in ((1, 'l'), (-1, 'r')):
            w.append(rev(f'wheel-{wb}-{st}', END, 'steel-wheel', 'body', 0.3, wheel_loop(), [xb + dx, 0.3, side * Z_BACK], (0, 0, side), bore=True))
            w.append(cyl(f'stub-axle-{wb}-{st}', END, 'steel-wheel', 'detail', STEEL, [xb + dx, 0.3, side * 0.56], (0, 0, side), 0.09, 0.055))
            w.append(rev(f'brake-disc-{wb}-{st}', END, 'cast-iron', 'detail', 0.55, [[0.096, 0.09], [0.096, 0.2], [0.114, 0.2], [0.114, 0.09]], [xb + dx, 0.3, side * Z_BACK], (0, 0, side), bore=True))
        b += [on(s, wb, (xb + dx, 0.3)) for s in w]
    return b


def doors():
    b = []
    for name, x0 in (('a1', DOOR_A[0]), ('a2', DOOR_A[0] + LEAF), ('b1', DOOR_B[0]), ('b2', DOOR_B[0] + LEAF)):
        first = name.endswith('1')
        y0, y1 = DOOR_Y
        g = rect(x0 + 0.12, 0.62, x0 + LEAF - 0.12, 2.30)
        w = [ext(f'leaf-{name}', END, 'paint', 'body', PAINT - 0.08, 'xy', rect(x0 + 0.002, y0, x0 + LEAF - 0.002, y1), -HW - 0.012, -HW + 0.028, holes=[g])]
        w.append(slab(f'leaf-glass-{name}', END, 'glass', 'glazing', GLASS, x0 + 0.11, x0 + LEAF - 0.11, 0.61, 2.31, -HW - 0.006, -HW + 0.018))
        xm = x0 + LEAF - 0.014 if first else x0 + 0.014
        w.append(slab(f'leaf-seal-{name}', END, 'rubber', 'detail', DARK, xm - 0.012, xm + 0.012, y0 + 0.01, y1 - 0.01, -HW - 0.016, -HW - 0.004))
        xbn = x0 + LEAF - 0.07 if first else x0 + 0.07
        w.append(cyl(f'leaf-button-{name}', END, 'glass', 'detail', LAMP, [xbn, 1.25, -HW - 0.012], (0, 0, -1), 0.012, 0.035))
        b += [on(lean(s), f'door-{name}', (x0, 0.3)) for s in w]
    return b


def pantograph():
    b = []
    px, py = PAN
    ins = [[0, 0.0], [0, 0.045], [0.02, 0.06], [0.04, 0.04], [0.06, 0.06], [0.08, 0.04], [0.1, 0.05], [0.1, 0.0]]
    for i, x in enumerate((px - 0.25, px + 1.15)):
        for side, st in ((1, 'l'), (-1, 'r')):
            b.append(rev(f'pan-insulator-{i + 1}-{st}', END, 'rubber', 'detail', 0.55, ins, [x, ROOF - 0.012, side * 0.42], (0, 1, 0)))
        b.append(slab(f'pan-frame-cross-{i + 1}', END, 'pan-alloy', 'detail', 0.45, x - 0.03, x + 0.03, py - 0.062, py - 0.002, -0.47, 0.47))
    for side, st in ((1, 'l'), (-1, 'r')):
        za = sorted((side * 0.39, side * 0.45))
        b.append(slab(f'pan-frame-rail-{st}', END, 'pan-alloy', 'detail', 0.45, px - 0.30, px + 1.20, py - 0.062, py - 0.002, za[0], za[1]))
        zb = sorted((side * 0.30, side * 0.39))
        b.append(slab(f'pan-bearing-{st}', END, 'pan-alloy', 'detail', 0.4, px - 0.06, px + 0.06, py - 0.062, py + 0.045, zb[0], zb[1], corner=0.01))
    b.append(cyl('pan-shaft', END, 'pan-alloy', 'detail', 0.4, [px, py, -0.32], (0, 0, 1), 0.64, 0.03))
    b.append(cyl('pan-drive', END, 'pan-alloy', 'detail', 0.35, [px + 0.35, py - 0.03, 0.17], (1, 0, 0), 0.6, 0.05))
    L, U, H = 'pan-lower', 'pan-upper', 'pan-head'
    b.append(cap('pan-lower-arm', L, 'pan-alloy', 'body', 0.45, [0, 0, 0], [1.9, 0, 0], 0.042, 0.034, slender=True))
    b.append(cap('pan-lower-rod', L, 'pan-alloy', 'detail', 0.45, [0.22, -0.075, 0.1], [1.84, -0.06, 0.1], 0.011, slender=True))
    b.append(cyl('pan-knee', L, 'pan-alloy', 'detail', 0.4, [1.9, 0, -0.1], (0, 0, 1), 0.2, 0.04))
    for side, st in ((1, 'l'), (-1, 'r')):
        b.append(cap(f'pan-upper-arm-{st}', U, 'pan-alloy', 'body', 0.45, [0, 0, side * 0.07], [1.9, 0, side * 0.30], 0.02, slender=True))
    b.append(cap('pan-upper-brace', U, 'pan-alloy', 'detail', 0.45, [0.95, 0, -0.185], [0.95, 0, 0.185], 0.012))
    b.append(cap('pan-upper-rod', U, 'pan-alloy', 'detail', 0.45, [0.06, 0.06, 0], [1.86, 0.045, 0], 0.009, slender=True))
    b.append(cyl('pan-head-shaft', H, 'pan-alloy', 'detail', 0.4, [0, 0, -0.34], (0, 0, 1), 0.68, 0.016))
    for side, st in ((1, 'l'), (-1, 'r')):
        b.append(box(f'pan-apex-{st}', H, 'pan-alloy', 'detail', 0.45, [0.38, 0.02, 0.02], [0, -0.004, side * 0.30]))
    for k, u in enumerate((-0.17, 0.17)):
        b.append(box(f'pan-strip-{k + 1}', H, 'carbon', 'body', 0.1, [0.036, 0.022, 1.05], [u, 0.0154, 0], corner=0.004))
        b.append(box(f'pan-carrier-{k + 1}', H, 'pan-alloy', 'detail', 0.5, [0.044, 0.012, 1.1], [u, -0.001, 0]))
        for side, st in ((1, 'l'), (-1, 'r')):
            b.append(cap(f'pan-horn-{k + 1}-{st}', H, 'pan-alloy', 'detail', 0.5, [u, 0.006, side * 0.54], [u, -0.115, side * 0.83], 0.011))
    return b


def roof_equipment():
    b = []
    x0, x1, hz, h = HVAC
    y0, y1 = ROOF - 0.02, ROOF + h
    w = [slab('hvac-box', END, 'paint', 'body', PAINT - 0.12, x0, x1, y0, y1, -hz, hz, corner=0.04)]
    for k, x in enumerate((-4.5, -3.5)):
        w.append(cyl(f'hvac-fan-ring-{k + 1}', END, 'rubber', 'detail', DARK, [x, y1 - 0.004, 0], (0, 1, 0), 0.014, 0.36, 0.30))
        w.append(cyl(f'hvac-fan-hub-{k + 1}', END, 'rubber', 'detail', DARK, [x, y1 - 0.004, 0], (0, 1, 0), 0.012, 0.09))
    for k, x in enumerate((-4.8, -4.1, -3.4, -2.7, -2.0)):
        for side, st in ((1, 'l'), (-1, 'r')):
            za = sorted((side * hz, side * (hz + 0.005)))
            w.append(slab(f'hvac-louvre-{k}-{st}', END, 'paint', 'texture', TRIM, x - 0.006, x + 0.006, ROOF + 0.06, ROOF + 0.2, za[0], za[1], texture=True))
    b += [mid(s) for s in w]
    x0, x1, hz, h, foot = RESISTOR
    y0, y1 = ROOF + 0.04, ROOF + h
    b.append(slab('resistor-box', END, 'resistor-steel', 'body', 0.55, x0, x1, y0, y1, -hz, hz, corner=0.02))
    for i, x in enumerate((x0 + 0.15, x1 - 0.15)):
        for side, st in ((1, 'l'), (-1, 'r')):
            b.append(slab(f'resistor-foot-{i + 1}-{st}', END, 'resistor-steel', 'detail', 0.4, x - 0.05, x + 0.05, ROOF - 0.025, y0 + 0.003, side * 0.65 - 0.04, side * 0.65 + 0.04))
    b.append(cyl('resistor-fan-ring', END, 'resistor-steel', 'detail', 0.3, [5.5, y1 - 0.004, 0], (0, 1, 0), 0.014, 0.39, 0.35))
    b.append(cyl('resistor-fan-hub', END, 'resistor-steel', 'detail', 0.3, [5.5, y1 - 0.004, 0], (0, 1, 0), 0.012, 0.1))
    for k, x in enumerate((4.8, 5.2, 5.6, 6.0)):
        for side, st in ((1, 'l'), (-1, 'r')):
            za = sorted((side * hz, side * (hz + 0.005)))
            b.append(slab(f'resistor-louvre-{k}-{st}', END, 'resistor-steel', 'texture', TRIM, x - 0.006, x + 0.006, ROOF + 0.06, ROOF + 0.18, za[0], za[1], texture=True))
    return b


def body50():
    b = car('end', END, E0, XC, (BOGIE_R, BOGIE_F), DOOR_A, PANES_END_R, PANES_END_L, True)
    b += car('mid', MID, M0, M1, (), DOOR_B, PANES_MID_R, PANES_MID_L, False)
    b += end_walls() + cab() + bellows()
    b += bogie(BOGIE_R, 'r', False) + bogie(BOGIE_F, 'f', True)
    b += doors() + pantograph() + roof_equipment()
    return b


def bellows_thermal():
    """the bellows as the surface the outside air meets: the folded outer face of each wall over a flat back that lies in the gangway and is no outer surface"""
    b = []
    zo, zi = BELLOWS[0], BELLOWS[0] - 0.08
    y0, y1 = BELLOWS[1], BELLOWS[2]
    xs = [-0.45 + 0.05 * k for k in range(19)]
    zig = lambda o, i: [(o if k % 2 == 0 else i) for k in range(19)]
    for side, st, face in ((1, 'l', '-w'), (-1, 'r', '+w')):
        outer = [[side * z, x] for z, x in zip(zig(zo, zi), xs)]
        b.append(ext(f'th-bellows-{st}', END, 'rubber', 'body', 0.16, 'zx', outer + [[side * (zi - 0.016), 0.45], [side * (zi - 0.016), -0.45]], y0 + 0.096, y1 - 0.096, region='gangway', sectionFaces=[face]))
    top = [[x, y] for x, y in zip(xs, zig(y1, y1 - 0.08))]
    b.append(ext('th-bellows-top', END, 'rubber', 'body', 0.2, 'xy', top + [[0.45, y1 - 0.096], [-0.45, y1 - 0.096]], -zo, zo, region='gangway', sectionFaces=['-v']))
    bot = [[x, y] for x, y in zip(xs, zig(y0, y0 + 0.08))]
    b.append(ext('th-bellows-bottom', END, 'rubber', 'body', 0.12, 'xy', [[-0.45, y0 + 0.096], [0.45, y0 + 0.096]] + list(reversed(bot)), -zo, zo, region='gangway', sectionFaces=['+v']))
    return [on(s, 'gangway', (0, 0.3)) for s in b]


def of(id, body, region, layer=None, faces=None, tags=None):
    s = {"id": id, "of": body, "region": region}
    if layer:
        s["layer"] = layer
    if faces:
        s["sectionFaces"] = faces
    if tags:
        s["tags"] = tags
    return s


def th(solid, id, region, layer, tone=None, faces=None, **change):
    s = copy.deepcopy(solid)
    s.update(change)
    s["id"], s["region"] = id, region
    if layer:
        s["layer"] = layer
    if tone is not None:
        s["tone"] = tone
    if faces:
        s["sectionFaces"] = faces
    return s


def shrunk(solid, t, id, region, layer, tone=0.5):
    s = th(solid, id, region, layer, tone)
    for q in s["sections"]:
        q["rx"], q["ry"] = rr(q["rx"] - t), rr(q["ry"] - t)
    return s


def split_box(solid, id, region, layers, tones, faces=None, layers_out='pane-out'):
    """a box cut into plates across its thinnest edge (the local w): layers = [(layer id, thickness)] from -w to +w"""
    out = []
    size = solid["size"]
    tr = solid["transform"]["translation"]
    q = solid["transform"].get("rotation", [0, 0, 0, 1])
    w = -size[2] / 2
    for (layer, t), tone in zip(layers, tones):
        s = th(solid, f'{id}-{layer}', region, layer, tone, faces if layer == layers_out else None)
        s["size"] = [size[0], size[1], rr(t)]
        d = qrot(q, [0, 0, w + t / 2])
        s["transform"] = dict(solid["transform"], translation=[rr(a + b) for a, b in zip(tr, d)])
        s.pop("cornerRadius", None)
        out.append(s)
        w += t
    return out


GLASS_LAYERS = (('pane-in', 0.006), ('gap', 0.012), ('pane-out', 0.006))


def thermal50(body):
    by = {s["id"]: s for s in body}
    t = []
    INS = TW - SKIN
    for s in body:
        i = s["id"]
        if '-side-lower-' in i or '-side-upper-' in i:
            left = i.split('-side-')[1].split('-')[1] == 'l'
            lo, hi = s["from"], s["to"]
            a, b = (hi - SKIN, hi) if left else (lo, lo + SKIN)
            c, d = (lo, hi - SKIN) if left else (lo + SKIN, hi)
            t.append(th(s, f'th-{i}-skin', 'body-panels', 'skin', None, ['-w' if left else '+w'], **{"from": rr(a), "to": rr(b)}))
            t.append(th(s, f'th-{i}-insulation', 'body-panels', 'insulation', 0.5, **{"from": rr(c), "to": rr(d)}))
        elif i.endswith('-roof'):
            o, n_ = roof_outline(), roof_outline(SKIN)
            inner_v = '-v' if s["bone"] == END else '+v'
            for part, a, b, face in (('r', 0, 4, '+w'), ('crown', 4, 10, inner_v), ('l', 10, 14, '-w')):
                t.append(th(s, f'th-{i}-skin-{part}', 'roof', 'skin', None, [face], profile=[[rr(p), rr(q)] for p, q in o[a:b + 1] + list(reversed(n_[a:b + 1]))]))
            t.append(th(s, f'th-{i}-insulation', 'roof', 'insulation', 0.5, profile=[[rr(p), rr(q)] for p, q in roof_outline(SKIN) + list(reversed(roof_outline(TW)))]))
        elif i.endswith('-floor'):
            t.append(th(s, f'th-{i}-skin', 'body-panels', 'skin', None, ['+v' if s["bone"] == END else '-v'], **{"from": SKIRT, "to": rr(SKIRT + SKIN)}))
            t.append(th(s, f'th-{i}-insulation', 'body-panels', 'insulation', 0.5, **{"from": rr(SKIRT + SKIN), "to": FLOOR}))
        elif i in ('end-rear-wall', 'mid-rear-wall', 'mid-front-wall'):
            lo, hi = s["from"], s["to"]
            a, b = (hi - SKIN, hi) if i == 'mid-front-wall' else (lo, lo + SKIN)
            c, d = (lo, hi - SKIN) if i == 'mid-front-wall' else (lo + SKIN, hi)
            t.append(th(s, f'th-{i}-skin', 'body-panels', 'skin', None, ['+u'] if i != 'mid-rear-wall' else ['-u', '+u'], **{"from": rr(a), "to": rr(b)}))
            t.append(th(s, f'th-{i}-insulation', 'body-panels', 'insulation', 0.5, **{"from": rr(c), "to": rr(d)}))
        elif i.startswith('podest-'):
            left = i.split('-')[3] == 'l'
            face = '+v' if '-top-' in i else ('-w' if left else '+w') if '-wall-' in i else '-u' if i.endswith('-a') else '+u'
            t.append(of(f'th-{i}', i, 'body-panels', 'skin', [face]))
        elif i == 'mid-rear-blank':
            t.append(of(f'th-{i}', i, 'body-panels', 'skin', ['-u', '+u']))
        elif i in ('bumper', 'cab-sill', 'coupler-hatch') or i.startswith(('a-pillar', 'b-pillar')):
            t.append(of(f'th-{i}', i, 'body-panels', 'skin'))
        elif i.startswith('pane-') and s["kind"] == 'box':
            left = i.endswith('-l')
            layers = GLASS_LAYERS if left else tuple(reversed(GLASS_LAYERS))
            t += split_box(s, f'th-{i}', 'glazing', layers, (0.3, 0.6, 0.3), ['-w' if left else '+w'])
        elif i.startswith('pane-cab-'):
            left = i.endswith('-l')
            lo = s["from"]
            order = GLASS_LAYERS if left else tuple(reversed(GLASS_LAYERS))
            for layer, d in order:
                t.append(th(s, f'th-{i}-{layer}', 'glazing', layer, 0.6 if layer == 'gap' else 0.3, ['-w' if left else '+w'] if layer == 'pane-out' else None, **{"from": rr(lo), "to": rr(lo + d)}))
                lo += d
        elif i.startswith('leaf-glass-'):
            for l2 in split_box(s, f'th-{i}', 'glazing', tuple(reversed(GLASS_LAYERS)), (0.3, 0.6, 0.3), ['+w']):
                t.append(l2)
        elif i.startswith('leaf-') and s["kind"] == 'extrude':
            lo, hi = s["from"], s["to"]
            t.append(of(f'th-{i}-skin', i, 'doors', 'skin', ['+w']))
            x0, y0 = s["profile"][0]
            x1, y1 = s["profile"][2]
            core = rect(x0 + SKIN, y0 + SKIN, x1 - SKIN, y1 - SKIN)
            hole = [[p + (SKIN if p < (x0 + x1) / 2 else -SKIN) * -1, q + (SKIN if q < (y0 + y1) / 2 else -SKIN) * -1] for p, q in s["holes"][0]]
            t.append(th(s, f'th-{i}-insulation', 'doors', 'insulation', 0.5, profile=[[rr(p), rr(q)] for p, q in core], holes=[[[rr(p), rr(q)] for p, q in hole]], **{"from": rr(lo + SKIN), "to": rr(hi - SKIN)}))
        elif i.startswith(('motor-f', 'motor-r', 'gearbox-', 'pinion-')):
            t.append(of(f'th-{i}', i, 'traction-motors'))
        elif i.startswith(('wheel-', 'stub-axle-')):
            t.append(of(f'th-{i}', i, 'wheelsets'))
        elif i.startswith('brake-disc-'):
            t.append(of(f'th-{i}', i, 'brake-discs'))
        elif i in ('resistor-box', 'resistor-fan-ring', 'resistor-fan-hub') or i.startswith('resistor-foot'):
            t.append(of(f'th-{i}', i, 'brake-resistor'))
        elif i == 'hvac-box':
            t.append(of(f'th-{i}', i, 'hvac-unit', None, ['+v']))
        elif i.startswith('hvac-fan'):
            t.append(of(f'th-{i}', i, 'hvac-unit'))
        elif i.startswith('pan-'):
            t.append(of(f'th-{i}', i, 'pantograph'))
    n = by['cab-nose']
    t.append(of('th-cab-nose-skin', 'cab-nose', 'body-panels', 'skin', ['-u', '+v']))
    t.append(th(n, 'th-cab-nose-insulation', 'body-panels', 'insulation', 0.5, profile=[[rr(p), rr(q)] for p, q in dfill(NOSE_D - SKIN, HW - SKIN, XC)], **{"from": rr(SKIRT + SKIN)}))
    t.append(of('th-cab-header-skin', 'cab-header', 'body-panels', 'skin', ['-u', '-v']))
    t.append(shrunk(by['cab-header'], SKIN, 'th-cab-header-insulation', 'body-panels', 'insulation'))
    t.append(of('th-windscreen-pane-out', 'windscreen', 'glazing', 'pane-out', ['-u', '-v', '+v']))
    t.append(shrunk(by['windscreen'], 0.006, 'th-windscreen-gap', 'glazing', 'gap', 0.6))
    t.append(shrunk(by['windscreen'], 0.018, 'th-windscreen-pane-in', 'glazing', 'pane-in', 0.3))
    d = by['destination-sign']
    t += split_box(dict(d, size=[d["size"][1], d["size"][2], d["size"][0]], transform=dict(d["transform"], rotation=qmul(d["transform"]["rotation"], [0.5, 0.5, 0.5, 0.5]))), 'th-destination-sign', 'glazing', GLASS_LAYERS, (0.2, 0.5, 0.1), ['-u'])
    t += bellows_thermal()
    x0, x1, hz, h, foot = RESISTOR
    t.append(cyl('resistor-fan-outlet', END, 'resistor-steel', 'detail', 0.9, [5.5, ROOF + h + 0.01, 0], (0, 1, 0), 0.008, 0.35, region='brake-resistor', tags=['emitter', 'resistor-fan']))
    v = mid(cyl('hvac-vent-outlet', END, 'paint', 'detail', 0.9, [-3.5, ROOF + HVAC[3] + 0.01, 0], (0, 1, 0), 0.008, 0.30))
    v.update(region='hvac-unit', tags=['emitter', 'hvac-vent'])
    t.append(v)
    return t


FR, FRS = 0.55, 0.35
POSTS = {('end', 1): [0.5, 3.375, 4.925, 6.375, 7.545], ('end', -1): [0.5, 3.4, 4.9, 6.375, 7.545], ('mid', 1): [-5.94, -4.175, -2.625, -0.51], ('mid', -1): [-5.94, -4.125, -2.625, -0.51]}
BEARERS = {'end': [0.52, 3.4, 4.15, 4.9, 5.6], 'mid': [-5.94, -5.1, -4.3, -3.5, -2.7, -1.9, -1.1, -0.52]}


def frame(tag, bone, x0, x1, bogies, door, raked):
    """the welded aluminium car body frame of one module: sole bars, bolsters over the bogies, cross-bearers, posts, waist and cant rails, roof bows, end rings"""
    w = (lambda s: s) if bone == END else mid
    m = lambda s: dict(w(s), member=bone, part='member')
    out = []
    for side, st in ((1, 'l'), (-1, 'r')):
        cuts = [x0] + [v for xb in bogies for v in (xb - 1.3, min(xb + 1.3, x1))] + [x1]
        for k in range(0, len(cuts), 2):
            if cuts[k + 1] - cuts[k] > 0.05:
                za = sorted((side * 1.06, side * 1.145))
                out.append(m(slab(f'{tag}-sole-bar-{st}-{k // 2 + 1}', END, 'paint', 'body', FR, cuts[k], cuts[k + 1], SKIRT, 0.44, za[0], za[1])))
        for xb, bt in zip(bogies, 'rf'):
            za = sorted((side * 0.80, side * 1.145))
            out.append(m(slab(f'{tag}-bolster-{bt}-{st}', END, 'paint', 'body', FR, xb - 1.3, min(xb + 1.3, x1), 0.70, 0.80, za[0], za[1])))
            for k, xx in enumerate((xb - 1.3, xb + 1.3)):
                if xx < x1 - 0.01:
                    zb = sorted((side * 1.06, side * 1.145))
                    out.append(m(slab(f'{tag}-bolster-post-{bt}-{st}-{k + 1}', END, 'paint', 'body', FR, xx - 0.04, xx + 0.04, SKIRT, 0.80, zb[0], zb[1])))
        top = x1 - (TANR * (CANT - 0.04 - BELT) if raked else 0.0)
        zc = sorted((side * (hw(CANT - 0.04) - 0.09), side * (hw(CANT - 0.04) - 0.01)))
        out.append(m(slab(f'{tag}-cant-rail-{st}', END, 'paint', 'body', FR, x0, top, CANT - 0.08, CANT, zc[0], zc[1])))
        zw = sorted((side * 1.085, side * 1.145))
        spans = [(x0, x1)] if side > 0 or not door else [(x0, door[0]), (door[1], x1)]
        for k, (a, c) in enumerate(spans):
            out.append(m(slab(f'{tag}-waist-rail-{st}-{k + 1}', END, 'paint', 'body', FR, a, c, BELT - 0.06, BELT, zw[0], zw[1])))
        for k, x in enumerate(POSTS[(tag, side)]):
            base = 0.80 if any(abs(x - xb) < 1.3 for xb in bogies) else SKIRT
            out.append(m(slab(f'{tag}-post-lower-{st}-{k + 1}', END, 'paint', 'body', FR, x - 0.03, x + 0.03, base, BELT, zw[0], zw[1])))
            out.append(m(bar(f'{tag}-post-upper-{st}-{k + 1}', END, 'paint', 'body', FR, on_wall(side, x, 0, -side * 0.03), on_wall(side, x, S_TOP, -side * 0.03), 0.06, 0.05)))
        if door and side < 0:
            for k, x in enumerate((door[0] - 0.04, door[1] + 0.04)):
                out.append(m(slab(f'{tag}-door-post-{k + 1}', END, 'paint', 'body', FR, x - 0.035, x + 0.035, SKIRT, DOOR_Y[1] + 0.1, zw[0], zw[1])))
            out.append(m(slab(f'{tag}-door-lintel', END, 'paint', 'body', FR, door[0] - 0.075, door[1] + 0.075, DOOR_Y[1] + 0.03, DOOR_Y[1] + 0.11, -1.14, -1.07)))
    bow = roof_outline(TW) + list(reversed(roof_outline(TW + 0.06)))
    xs = sorted(set(POSTS[(tag, 1)]))
    for k, x in enumerate(xs):
        out.append(m(ext(f'{tag}-roof-bow-{k + 1}', END, 'paint', 'body', FR, 'yz', bow, x - 0.03, x + 0.03)))
    for k, x in enumerate(BEARERS[tag]):
        out.append(m(slab(f'{tag}-cross-bearer-{k + 1}', END, 'paint', 'body', FR, x - 0.04, x + 0.04, SKIRT + 0.005, FLOOR - 0.005, -1.06, 1.06)))
    ringp = section(TW)
    portal = rect(PORTAL[1] + 0.0, -PORTAL[0], PORTAL[2], PORTAL[0])
    ends = [x0] if raked else [x0, x1 - 0.07]
    for k, x in enumerate(ends):
        out.append(m(ext(f'{tag}-end-ring-{k + 1}', END, 'paint', 'body', FR, 'yz', ringp, x + 0.0, x + 0.07, holes=[rect(SKIRT + 0.08, -1.02, CANT + 0.1, 1.02)])))
        for j, z in enumerate((-PORTAL[0] - 0.04, PORTAL[0] + 0.04)):
            out.append(m(slab(f'{tag}-portal-post-{k + 1}-{j + 1}', END, 'paint', 'body', FR, x, x + 0.07, SKIRT + 0.05, CANT + 0.15, z - 0.04, z + 0.04)))
        out.append(m(slab(f'{tag}-portal-lintel-{k + 1}', END, 'paint', 'body', FR, x, x + 0.07, PORTAL[2], PORTAL[2] + 0.08, -PORTAL[0] - 0.08, PORTAL[0] + 0.08)))
    return out


def structure50(body):
    s = frame('end', END, E0, XC, (BOGIE_R, BOGIE_F), DOOR_A, True)
    s += frame('mid', MID, M0, M1, (), DOOR_B, False)
    e = lambda q, bone=END, part='member': dict(q, member=bone, part=part)
    s.append(e(ext('cab-anticlimber', END, 'paint', 'body', FR, 'zx', dband(NOSE_D - 0.02, HW - 0.02, 0.1, XC), 0.46, 0.62)))
    s.append(e(ext('cab-floor-frame', END, 'paint', 'body', FR, 'zx', dband(NOSE_D - 0.02, HW - 0.02, 0.07, XC), SKIRT + 0.005, FLOOR)))
    s.append(e(ext('cab-waist-ring', END, 'paint', 'body', FR, 'zx', dband(NOSE_D - 0.02, HW - 0.02, 0.06, XC), BELT - 0.07, BELT - 0.005)))
    for side, st in ((1, 'l'), (-1, 'r')):
        z = side * 0.45
        s.append(e(bar(f'cab-collision-post-{st}', END, 'paint', 'body', FR, [nose_x(0.4, z) - 0.09, 0.35, z], [nose_x(BELT, z) - 0.09, BELT, z], 0.08, 0.08)))
        f = 0.80
        pts = [[nose_x(y, side * f * hw(y)) - 0.05, y, side * (f * hw(y) - 0.04)] for y in (BELT, GLASS_TOP)]
        s.append(e(bar(f'cab-a-post-{st}', END, 'paint', 'body', FR, pts[0], pts[1], 0.06, 0.06)))
        s.append(e(bar(f'cab-b-post-{st}', END, 'paint', 'body', FR, [edge(BELT) - 0.03, BELT, side * (HW - 0.06)], [edge(CANT - 0.05) - 0.03, CANT - 0.05, side * (hw(CANT - 0.05) - 0.06)], 0.07, 0.05)))
    s.append(e(ext('cab-header-ring', END, 'paint', 'body', FR, 'zx', dband(tip(GLASS_TOP + 0.09) - edge(GLASS_TOP + 0.09) - 0.03, hw(GLASS_TOP + 0.09) - 0.03, 0.08, edge(GLASS_TOP + 0.09)), GLASS_TOP + 0.01, GLASS_TOP + 0.09)))
    g = lambda q, part='member': dict(on(q, 'gangway', (0, 0.3)), member='gangway', part=part)
    s.append({"id": 'articulation-lower', "of": 'articulation-arm', "member": 'gangway', "part": 'member'})
    s.append(g(slab('articulation-upper', END, 'steel-wheel', 'body', FRS, -0.52, 0.52, 3.08, 3.15, -0.07, 0.07)))
    s.append({"id": 'articulation-joint', "of": 'articulation-bearing', "member": 'body-mid', "part": 'joint'})
    s.append(g(cyl('articulation-upper-pin', END, 'steel-wheel', 'detail', 0.2, [0, 3.05, 0], (0, 1, 0), 0.13, 0.06), 'joint'))
    for bt in 'rf':
        bone = f'bogie-{bt}'
        for b in body:
            i = b["id"]
            if i.startswith((f'bogie-beam-{bt}', f'bogie-cross-{bt}', f'bogie-upright-{bt}')):
                s.append({"id": f'st-{i}', "of": i, "member": bone, "part": 'member'})
            elif i.startswith(f'spring-{bt}'):
                s.append({"id": f'st-{i}', "of": i, "member": bone, "part": 'joint'})
        xb = BOGIE_R if bt == 'r' else BOGIE_F
        for k, dx in enumerate((-WB, WB)):
            wb = f'ws-{bt}{k + 1}'
            for side, st in ((1, 'l'), (-1, 'r')):
                s.append({"id": f'st-wheel-{wb}-{st}', "of": f'wheel-{wb}-{st}', "member": wb, "part": 'member'})
                ax = cyl(f'axle-journal-{wb}-{st}', END, 'steel-wheel', 'body', FRS, [xb + dx, 0.3, side * 0.56], (0, 0, side), 0.39, 0.055)
                s.append(dict(on(ax, wb, (xb + dx, 0.3)), member=wb, part='member'))
                br = cyl(f'bearing-{wb}-{st}', END, 'cast-iron', 'detail', 0.2, [xb + dx, 0.3, side * 0.84], (0, 0, side), 0.1, 0.09)
                s.append(dict(on(br, wb, (xb + dx, 0.3)), member=wb, part='joint'))
            s.append(dict(ell(f'mass-{wb}', wb, 'steel-wheel', 'detail', 0.6, [0.1, 0.1, 0.16], [0, 0, 0]), member=wb, part='mass'))
        s.append(dict(ell(f'mass-{bone}', bone, 'steel-wheel', 'detail', 0.6, [0.24, 0.03, 0.3] if bt == 'f' else [0.2, 0.03, 0.26], [0, 0, 0]), member=bone, part='mass'))
    for i, bone, part in (('pan-lower-arm', 'pan-lower', 'member'), ('pan-upper-arm-l', 'pan-upper', 'member'), ('pan-upper-arm-r', 'pan-upper', 'member'), ('pan-upper-brace', 'pan-upper', 'member'), ('pan-apex-l', 'pan-head', 'member'), ('pan-apex-r', 'pan-head', 'member'), ('pan-shaft', 'pan-lower', 'joint'), ('pan-knee', 'pan-upper', 'joint'), ('pan-head-shaft', 'pan-head', 'joint'), ('pan-frame-rail-l', 'body-end', 'member'), ('pan-frame-rail-r', 'body-end', 'member'), ('pan-frame-cross-1', 'body-end', 'member'), ('pan-frame-cross-2', 'body-end', 'member')):
        s.append({"id": f'st-{i}', "of": i, "member": bone, "part": part})
    for name, x0 in (('a1', DOOR_A[0]), ('a2', DOOR_A[0] + LEAF), ('b1', DOOR_B[0]), ('b2', DOOR_B[0] + LEAF)):
        bone = f'door-{name}'
        fr = ext(f'leaf-frame-{name}', END, 'paint', 'body', FR, 'xy', rect(x0 + 0.01, DOOR_Y[0] + 0.01, x0 + LEAF - 0.01, DOOR_Y[1] - 0.01), -HW - 0.004, -HW + 0.024, holes=[rect(x0 + 0.05, DOOR_Y[0] + 0.06, x0 + LEAF - 0.05, DOOR_Y[1] - 0.05)])
        s.append(dict(on(lean(fr), bone, (x0, 0.3)), member=bone, part='member'))
        hg = slab(f'leaf-carriage-{name}', END, 'cast-iron', 'detail', 0.2, x0 + LEAF / 2 - 0.1, x0 + LEAF / 2 + 0.1, DOOR_Y[1] - 0.02, DOOR_Y[1] + 0.05, -HW + 0.035, -HW + 0.075)
        s.append(dict(on(lean(hg), bone, (x0, 0.3)), member=bone, part='joint'))
        s.append(dict(ell(f'mass-{bone}', bone, 'steel-wheel', 'detail', 0.6, [0.07, 0.07, 0.02], [LEAF / 2, 1.08, -HW + 0.01]), member=bone, part='mass'))
    s.append(dict(ell('mass-body-end', END, 'steel-wheel', 'detail', 0.6, [0.4, 0.3, 0.4], [5.0, 1.5, 0]), member=END, part='mass'))
    s.append(dict(ell('mass-body-mid', MID, 'steel-wheel', 'detail', 0.6, [0.32, 0.24, 0.32], [2.8, -1.2, 0]), member=MID, part='mass'))
    s.append(dict(ell('mass-gangway', 'gangway', 'steel-wheel', 'detail', 0.6, [0.12, 0.12, 0.12], [0, 1.4, 0]), member='gangway', part='mass'))
    s.append(dict(ell('mass-pan-lower', 'pan-lower', 'steel-wheel', 'detail', 0.6, [0.07, 0.035, 0.035], [0.95, 0, 0]), member='pan-lower', part='mass'))
    s.append(dict(ell('mass-pan-upper', 'pan-upper', 'steel-wheel', 'detail', 0.6, [0.07, 0.019, 0.019], [0.95, 0, 0]), member='pan-upper', part='mass'))
    s.append(dict(ell('mass-pan-head', 'pan-head', 'steel-wheel', 'detail', 0.6, [0.04, 0.02, 0.06], [0, 0, 0]), member='pan-head', part='mass'))
    return s


DECISIONS = [
    {"en": "1:50 shows the tram as it is built: side walls 51.5 mm thick with real window and door openings and 40 mm of tumblehome above the waist, a roof with rolled cant rails, a cab with a superelliptic nose, a windscreen raked by 17 degrees and a domed header, V-folded bellows over the articulation bearing, podests over the bogies and plug-sliding door leaves, tilted with the tumblehome, with their glass, seals and buttons.", "de": "1:50 zeigt die Straßenbahn, wie sie gebaut ist: 51,5 mm starke Seitenwände mit echten Fenster- und Türöffnungen und 40 mm Einzug über der Brüstung, ein Dach mit gerundeten Obergurten, ein Fahrerstand mit superelliptischer Nase, um 17 Grad geneigter Frontscheibe und gewölbtem Stirndach, V-gefalteter Balg über dem Gelenklager, Podeste über den Drehgestellen und Schwenkschiebetürflügel, mit dem Einzug geneigt, mit Glas, Dichtungen und Tastern."},
    {"en": "The bogies are modelled part by part: wheels of 600 mm with flange and recessed web on stub axles (independent wheels, no axle under the low floor), wheel brake discs with calipers, side beams over rubber primary springs, cranked cross members under the aisle, secondary springs, magnetic track brakes, and on the powered bogie one longitudinal motor per side that drives both wheels through bevel gearboxes; the trailing bogie carries axle boxes instead.", "de": "Die Drehgestelle sind Teil für Teil modelliert: Räder mit 600 mm samt Spurkranz und eingezogener Radscheibe auf Achsstummeln (Losräder, keine Achse unter dem Niederflurboden), Radbremsscheiben mit Bremszangen, Langträger auf Gummi-Primärfedern, gekröpfte Querträger unter dem Gang, Sekundärfedern, Magnetschienenbremsen und am Triebdrehgestell je Seite ein längs liegender Motor, der beide Räder über Kegelradgetriebe antreibt; das Laufdrehgestell trägt stattdessen Achslager."},
    {"en": "The thermal list splits every wall, roof, floor and door plate into the 1.5 mm skin and the 50 mm lining and every pane into pane, gap and pane; the structure list is the welded car body frame (sole bars, bolsters, cross-bearers, posts, waist and cant rails, roof bows, end rings, cab posts), the articulation links, the bogie frames, stub axles and wheels, the pantograph arms and the door leaf frames, with the masses at the centres of mass.", "de": "Die thermische Liste teilt jede Wand-, Dach-, Boden- und Türplatte in 1,5 mm Haut und 50 mm Auskleidung und jede Scheibe in Scheibe, Zwischenraum und Scheibe; die Tragwerksliste ist das geschweißte Wagenkastengerippe (Langträger, Drehgestellträger, Querträger, Säulen, Brüstungs- und Obergurte, Dachspriegel, Endringe, Fahrerstandssäulen), die Gelenkverbindungen, die Drehgestellrahmen, Achsstummel und Räder, die Stromabnehmerarme und die Türflügelrahmen, mit den Massen in den Schwerpunkten."},
]
