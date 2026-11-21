"""🚐️ Round 3 city-bus: the 1:100, 1:200 and 1:500 lists, each authored for its own smallest feature (15, 30, 75 mm): thicker walls, merged glazing, simpler sections, fewer frames."""
import copy, math
from r3_city_bus_kit import *
import r3_city_bus_body50 as B
from r3_city_bus_lists50 import of, sref, node, STEEL

PAINT, GLASS, TRIM, RUBBER, RIM = 0.82, 0.28, 0.18, 0.1, 0.68
WALL = {100: 0.06, 200: 0.08, 500: 0.10}
ARC = {100: 20, 200: 14, 500: 8}
ENDS = {100: 32, 200: 20, 500: 12}
PANE = {100: 0.03, 200: 0.04, 500: 0.04}
CANT_COARSE = {100: CANT_TABLE, 200: [CANT_TABLE[i] for i in (0, 2, 3, 4, 5, 6, 8)], 500: [CANT_TABLE[i] for i in (0, 2, 4, 6, 8)]}
S0, S1 = B.S0, B.S1
RUNS_L = [(BAYS_L[0][0], BAYS_L[-1][1])]
RUNS_R = [(BAYS_R[0][0], BAYS_R[4][1]), (BAYS_R[5][0], BAYS_R[6][1])]
MULLIONS_L = [3.05, 4.35, 5.65, 6.95, 8.25, 9.55, 10.75]
MULLIONS_R = [1.75, 3.05, 4.35, 5.65, 8.865]


def roof_pts(d, t=0.0):
    half = [(w, y) for y, w in CANT_COARSE[d]]
    pts = [(-w, y) for w, y in half] + [(w, y) for w, y in reversed(half[:-1])]
    if t == 0.0:
        return [[y, z] for z, y in pts]
    out = []
    for i, (z, y) in enumerate(pts):
        a, b = pts[max(i - 1, 0)], pts[min(i + 1, len(pts) - 1)]
        dz, dy = b[0] - a[0], b[1] - a[1]
        m = math.hypot(dz, dy)
        out.append([y - dz / m * t, z + dy / m * t])
    return out


def shell(d):
    tw, n = WALL[d], ARC[d]
    b = []
    for side, tag, doors in ((1, 'l', False), (-1, 'r', True)):
        z0, z1 = (HW - tw, HW) if side > 0 else (-HW, -HW + tw)
        b.append(ext(f'side-lower-{tag}', 'body', 'paint', 'body', PAINT, 'xy', lower_profile(doors, n=n), z0, z1))
        if d == 100:
            holes = [rect(a, S0, c, S1) for a, c in (BAYS_L if side > 0 else BAYS_R)]
        else:
            holes = [rect(a, S0, c, S1) for a, c in (RUNS_L if side > 0 else RUNS_R)]
        zc = (-tw, 0) if side > 0 else (0, tw)
        b.append(ext(f'side-upper-{tag}', 'body', 'paint', 'body', PAINT, 'xy', upper_profile(doors, B.S_DOOR), zc[0], zc[1], holes=holes, at=[0, BELT, side * HW], rot=qx(-side * PHI)))
        inset = 0.008 if d == 100 else 0.0
        for k, h in enumerate(holes):
            a, c = h[0][0], h[1][0]
            b.append(box(f'glass-{tag}-{k + 1}', 'body', 'glass', 'glazing', GLASS, [c - a + 0.03, S1 - S0 + 0.03, PANE[d]], on_wall(side, (a + c) / 2, (S0 + S1) / 2, -side * (inset + PANE[d] / 2)), rot=qx(-side * PHI)))
            if d == 100 and c - a > 0.9:
                b.append(box(f'transom-{tag}-{k + 1}', 'body', 'paint', 'detail', TRIM, [c - a + 0.01, 0.03, 0.02], on_wall(side, (a + c) / 2, 1.02, -side * 0.002), rot=qx(-side * PHI)))
        if d == 200:
            for k, x in enumerate(MULLIONS_L if side > 0 else MULLIONS_R):
                b.append(box(f'mullion-{tag}-{k + 1}', 'body', 'paint', 'detail', PAINT, [0.09, S1 - S0 + 0.02, 0.03], on_wall(side, x, (S0 + S1) / 2, -side * 0.011), rot=qx(-side * PHI)))
            for k, (a, c) in enumerate(RUNS_L if side > 0 else RUNS_R):
                b.append(box(f'transom-{tag}-{k + 1}', 'body', 'paint', 'detail', TRIM, [c - a, 0.03, 0.03], on_wall(side, (a + c) / 2, 1.02, -side * 0.011), rot=qx(-side * PHI)))
    b.append(ext('roof', 'body', 'paint', 'body', PAINT + 0.04, 'yz', roof_pts(d) + list(reversed(roof_pts(d, tw))), X0, X1))
    w = HW - 0.002
    a = ARCH_HALF + 0.03
    zr, zf = Z_WELL_R - tw, Z_WELL_F - tw
    half = [[-w, X0], [-w, XR - a], [-zr, XR - a], [-zr, XR + a], [-w, XR + a], [-w, XF - a], [-zf, XF - a], [-zf, XF + a], [-w, XF + a], [-w, X1]]
    top = FLOOR if d < 500 else SKIRT + 0.08
    b.append(ext('floor', 'body', 'paint', 'body', 0.45, 'zx', half + [[-z, x] for z, x in reversed(half)], SKIRT, top))
    ro = R_ARCH + max(0.03, tw / 2)
    ao = math.asin((AXLE - SKIRT) / ro)
    for xa, zin, k in ((XR, Z_WELL_R, 'r'), (XF, Z_WELL_F, 'f')):
        for side, tag in ((1, 'l'), (-1, 'r')):
            tub = arc(xa, AXLE, ro, math.pi + ao, -ao, n) + list(reversed(arch(xa, R_ARCH, n)))
            b.append(ext(f'wheelhouse-{k}-{tag}', 'body', 'paint', 'body', 0.3, 'xy', tub, side * zin, side * (HW - tw + 0.005)))
            b.append(ext(f'wheelhouse-wall-{k}-{tag}', 'body', 'paint', 'body', 0.3, 'xy', arc(xa, AXLE, ro, math.pi + ao, -ao, n), side * (zin - tw), side * zin))
    return b


def ends(d):
    tw, m = WALL[d], ENDS[d]
    b = []
    nd = NOSE_D if d == 100 else 0.545
    b.append(ext('nose', 'body', 'paint', 'body', PAINT, 'zx', dband(nd, HW, NOSE_N, tw, X1, m=m), SKIRT, 1.19))
    b.append(ext('floor-front', 'body', 'paint', 'body', 0.45, 'zx', dfill(nd - 0.005, HW - 0.005, NOSE_N, X1, m=m), SKIRT, FLOOR if d < 500 else SKIRT + 0.08))
    glass = dband(GLASS_D, GLASS_W, NOSE_N, PANE[d], 0.0, 1, 0.02, math.pi - 0.02, m=m)
    b.append(ext('windscreen', 'body', 'glass', 'glazing', GLASS, 'zx', glass, 0, GLASS_H, at=[X1, SILL_GLASS, 0], rot=qz(RAKE), taper=[1, GLASS_K]))
    head = {100: ((2.66, 0.365), (2.72, 0.365), (2.80, 0.34), (2.88, 0.295), (2.93, 0.235), (2.965, 0.14)), 200: ((2.66, 0.365), (2.74, 0.36), (2.82, 0.33), (2.91, 0.26), (2.965, 0.14)), 500: ((2.66, 0.365), (2.76, 0.35), (2.86, 0.31), (2.96, 0.16))}[d]
    b.append(loft('header', 'body', 'paint', 'body', PAINT, [ring(y, X1, rx, hw(y), NOSE_N) for y, rx in head]))
    tail = dband(TAIL_D, HW, TAIL_N, tw, X0, -1, m=m)
    b.append(ext('tail', 'body', 'paint', 'body', PAINT, 'zx', tail, SKIRT, BELT))
    b.append(ext('tail-upper', 'body', 'paint', 'body', PAINT, 'zx', tail, BELT, 2.56, taper=[1, rr(hw(2.56) / HW)]))
    cap_ = {100: ((2.55, TAIL_D), (2.75, 0.315), (2.86, 0.275), (2.93, 0.21), (2.965, 0.12)), 200: ((2.55, TAIL_D), (2.78, 0.31), (2.90, 0.24), (2.965, 0.12)), 500: ((2.55, TAIL_D), (2.84, 0.28), (2.96, 0.14))}[d]
    b.append(loft('tail-cap', 'body', 'paint', 'body', PAINT, [ring(y, X0, rx, hw(y), TAIL_N) for y, rx in cap_]))
    b.append(ext('floor-rear', 'body', 'paint', 'body', 0.45, 'zx', dfill(TAIL_D - 0.005, HW - 0.005, TAIL_N, X0, -1, m=m), SKIRT, FLOOR if d < 500 else SKIRT + 0.08))
    if d <= 200:
        for side, tag in ((1, 'l'), (-1, 'r')):
            b.append(loft(f'a-pillar-{tag}', 'body', 'paint', 'body', PAINT, [sec([11.51, 1.08, side * (HW - 0.04)], 0.08, 0.04, 4), sec([11.335, 2.72, side * (hw(2.72) - 0.04)], 0.08, 0.04, 4)]))
        b.append(box('rear-window', 'body', 'glass', 'glazing', GLASS, [0.03, 0.80, 1.70], [0.02, 1.92, 0]))
    else:
        b.append(box('rear-window', 'body', 'glass', 'glazing', GLASS, [0.04, 0.80, 1.70], [0.035, 1.92, 0]))
    if d == 100:
        b.append(ext('bumper-front', 'body', 'paint', 'detail', 0.5, 'zx', dband(0.55, HW, NOSE_N, 0.05, X1, m=m), FLOOR, 0.62))
        b.append(ext('bumper-rear', 'body', 'paint', 'detail', 0.5, 'zx', dband(0.35, HW, TAIL_N, 0.05, X0, -1, m=m), FLOOR, 0.60))
        t0, t1 = 1.12, math.pi - 1.12
        sign = dcurve(0.534, GLASS_W, NOSE_N, t0, t1, 10) + list(reversed(dcurve(0.519, GLASS_W - 0.015, NOSE_N, t0, t1, 10)))
        b.append(ext('destination-sign', 'body', 'glass', 'glazing', 0.08, 'zx', sign, 1.19, 1.47, at=[X1, SILL_GLASS, 0], rot=qz(RAKE)))
        for side, tag in ((1, 'l'), (-1, 'r')):
            za, zb = sorted((side * 0.70, side * 1.10))
            b.append(ext(f'headlamp-{tag}', 'body', 'glass', 'glazing', 0.92, 'zx', zband(NOSE_D + 0.008, NOSE_D - 0.012, NOSE_N, za, zb, X1, m=6), 0.70, 0.88))
            b.append(box(f'tail-lamp-{tag}', 'body', 'glass', 'glazing', 0.6, [0.03, 0.52, 0.11], [0.036, 1.18, side * 1.03]))
    if d <= 200:
        r = 0.022 if d == 100 else 0.025
        b.append(box('mirror-r', 'body', 'paint', 'detail', TRIM, [0.07, 0.42, 0.2], [11.80, 2.05, MIRROR_R + 0.1], corner=0.03 if d == 100 else 0.0))
        b.append(cap('mirror-arm-r', 'body', 'paint', 'detail', TRIM, [11.47, 2.58, -1.245], [11.80, 2.27, MIRROR_R + 0.1], r))
        b.append(box('mirror-l', 'body', 'paint', 'detail', TRIM, [0.07, 0.36, 0.14], [11.72, 2.0, MIRROR_L - 0.07], corner=0.03 if d == 100 else 0.0))
        b.append(cap('mirror-arm-l', 'body', 'paint', 'detail', TRIM, [11.5, 2.02, 1.255], [11.72, 2.02, MIRROR_L - 0.09], r))
    else:
        b.append(box('mirror-r', 'body', 'paint', 'detail', TRIM, [0.08, 0.42, 0.26], [11.78, 2.05, MIRROR_R + 0.13]))
        b.append(box('mirror-l', 'body', 'paint', 'detail', TRIM, [0.08, 0.36, 0.21], [11.70, 2.0, MIRROR_L - 0.105]))
    return b


def doors(d):
    b = []
    for n in ('m-aft', 'm-fwd', 'f-aft', 'f-fwd'):
        bone = f'door-{n}'
        if d == 100:
            t, zo = 0.03, -(HW + 0.003)
            b.append(box(f'leaf-{n}', bone, 'door-leaf', 'body', 0.6, [LEAF - 0.008, 0.772, t], [LEAF / 2, 0.376, zo - t / 2]))
            s = (2.39 - BELT) / math.cos(PHI)
            c = [LEAF / 2, BELT - FLOOR + s / 2 * math.cos(PHI) + t / 2 * math.sin(PHI), zo + s / 2 * math.sin(PHI) - t / 2 * math.cos(PHI)]
            b.append(box(f'leaf-{n}-upper', bone, 'door-leaf', 'body', 0.6, [LEAF - 0.008, s, t], c, rot=qx(PHI)))
            b.append(box(f'leaf-{n}-glass-lower', bone, 'glass', 'glazing', GLASS, [LEAF - 0.14, 0.54, 0.012], [LEAF / 2, 0.43, zo - t - 0.002]))
            sg, zc = 0.62, -(t + 0.002)
            b.append(box(f'leaf-{n}-glass', bone, 'glass', 'glazing', GLASS, [LEAF - 0.14, 1.12, 0.012], [LEAF / 2, BELT - FLOOR + sg * math.cos(PHI) - zc * math.sin(PHI), zo + sg * math.sin(PHI) + zc * math.cos(PHI)], rot=qx(PHI)))
        else:
            t = 0.04 if d == 200 else 0.075
            psi = math.atan((HW - hw(2.39)) / 2.06)
            zmid = -((HW + hw(2.39)) / 2 + 0.016 + t / 2)
            b.append(box(f'leaf-{n}', bone, 'door-leaf', 'body', 0.6, [LEAF - 0.008, 2.06, t], [LEAF / 2, 1.02, zmid], rot=qx(psi)))
            if d == 200:
                b.append(box(f'leaf-{n}-glass', bone, 'glass', 'glazing', GLASS, [LEAF - 0.14, 1.72, 0.012], [LEAF / 2, 1.05, zmid - t / 2 - 0.003], rot=qx(psi)))
    return b


def disc_profile(side, h, r):
    return [[side * a, q] for a, q in ((-h, 0), (-h, r - 0.04), (-h + 0.04, r), (h - 0.04, r), (h, r - 0.04), (h, 0))]


def running_gear(d):
    b = []
    spring = [[0, 0.11], [0.32, 0.11], [0.32, 0.0]]
    for side, tag in ((1, 'l'), (-1, 'r')):
        zc, zo, zi, zm = side * Z_FRONT, side * Z_REAR_OUT, side * Z_REAR_IN, side * 0.915
        if d == 500:
            b.append(rev(f'wheel-f{tag}', 'wheel-f', 'rubber', 'body', RUBBER, disc_profile(1, 0.1375, R_TYRE), [0, 0, zc]))
            b.append(rev(f'wheel-r{tag}', 'wheel-r', 'rubber', 'body', RUBBER, disc_profile(1, 0.2975, R_TYRE), [0, 0, zm]))
            continue
        if d == 100:
            tyre = tyre_loop()
        else:
            tyre = [[-0.1375, 0.30], [-0.1375, 0.44], [-0.10, 0.48], [0.10, 0.48], [0.1375, 0.44], [0.1375, 0.30], [-0.1375, 0.30]]
        for pre, bone, z, dish in ((f'f{tag}', 'wheel-f', zc, 1), (f'r{tag}', 'wheel-r', zo, -1), (f'r{tag}-inner', 'wheel-r', zi, 1)):
            b.append(rev(f'{pre}-tyre', bone, 'rubber', 'body', RUBBER, tyre, [0, 0, z]))
            if d == 100:
                b.append(rev(f'{pre}-rim', bone, 'steel-wheel', 'detail', RIM, rim_loop(dish * side), [0, 0, z]))
            else:
                k = dish * side
                b.append(rev(f'{pre}-rim', bone, 'steel-wheel', 'detail', RIM, [[k * a, r] for a, r in ((-0.11, 0), (-0.11, 0.30), (0.11, 0.30), (0.16, 0.2), (0.16, 0))], [0, 0, z]))
        if d == 100:
            b.append(rev(f'f{tag}-hub', 'wheel-f', 'steel-wheel', 'detail', 0.5, [[side * a, r] for a, r in ((0.215, 0), (0.205, 0.06), (0.16, 0.08), (0.14, 0.08))], [0, 0, zc]))
            b.append(rev(f'r{tag}-hub', 'wheel-r', 'steel-wheel', 'detail', 0.5, [[side * a, r] for a, r in ((0.105, 0), (0.105, 0.07), (0.0, 0.088))], [0, 0, zm]))
        b.append(rev(f'f{tag}-brake-disc', 'wheel-f', 'cast-iron', 'detail', 0.35, [[side * a, r] for a, r in ((-0.07, 0.11), (-0.025, 0.11), (-0.025, 0.215), (-0.07, 0.215), (-0.07, 0.11))], [0, 0, zc]))
        b.append(rev(f'r{tag}-brake-disc', 'wheel-r', 'cast-iron', 'detail', 0.35, [[side * a, r] for a, r in ((-0.07, 0.11), (-0.025, 0.11), (-0.025, 0.215), (-0.07, 0.215), (-0.07, 0.11))], [0, 0, zi]))
        b.append(rev(f'e-axle-motor-{tag}', 'axle-r', 'e-axle', 'detail', 0.32, [[side * a, r] for a, r in ((0.36, 0), (0.36, 0.22), (0.61, 0.22), (0.66, 0.1), (0.66, 0))], [0, 0, 0]))
        b.append(rev(f'e-axle-spindle-{tag}', 'axle-r', 'e-axle', 'detail', 0.3, [[side * a, r] for a, r in ((0.66, 0), (0.66, 0.1), (0.915, 0.1), (0.915, 0))], [0, 0, 0]))
        if d == 100:
            b.append(box(f'spring-carrier-f-{tag}', 'axle-f', 'steel-wheel', 'detail', 0.3, [0.2, 0.3, 0.1], [0, -0.05, side * 0.70]))
            b.append(cap(f'stub-axle-f-{tag}', 'axle-f', 'steel-wheel', 'detail', 0.3, [0, 0, side * 0.80], [0, 0, side * 1.0], 0.05))
            b.append(rev(f'air-spring-f-{tag}', 'axle-f', 'rubber', 'detail', 0.15, spring, [0, 0.08, side * 0.70], axis=(0, 1, 0)))
            b.append(box(f'e-axle-drop-{tag}', 'axle-r', 'e-axle', 'detail', 0.3, [0.24, 0.3, 0.14], [0, -0.16, side * 0.42]))
            b.append(box(f'spring-carrier-r-{tag}', 'axle-r', 'e-axle', 'detail', 0.3, [0.8, 0.07, 0.12], [0, -0.215, side * 0.47]))
            for e, x in (('a', -0.34), ('b', 0.34)):
                b.append(rev(f'air-spring-r-{tag}-{e}', 'axle-r', 'rubber', 'detail', 0.15, spring, [x, -0.18, side * 0.47], axis=(0, 1, 0)))
    e = 4 if d < 500 else 2
    r = (0.06, 0.055) if d < 500 else (0.075, 0.075)
    b.append(loft('axle-beam-f', 'axle-f', 'steel-wheel', 'detail', 0.3, [sec([0, 0, -0.82], r[0], r[1], e), sec([0, -0.31, -0.62], r[0], r[1], e), sec([0, -0.31, 0.62], r[0], r[1], e), sec([0, 0, 0.82], r[0], r[1], e)]))
    rr_ = (0.13, 0.085) if d < 500 else (0.13, 0.0975)
    half = 0.42 if d < 500 else 0.60
    yb = -0.29 if d < 500 else -0.2775
    b.append(loft('axle-beam-r', 'axle-r', 'e-axle', 'detail', 0.3, [sec([0, yb, -half], rr_[0], rr_[1], 5), sec([0, yb, half], rr_[0], rr_[1], 5)]))
    return b


def roof_equipment(d):
    if d == 100:
        return [B.pod_loft('battery-pack', 'battery', 'battery-housing', 0.55, 6.3, 1.04, 0.32, 0.3), B.pod_loft('hvac-unit', 'hvac', 'hvac-housing', 0.6, 2.5, 0.90, 0.242, 0.22),
                rev('hvac-fan', 'hvac', 'hvac-housing', 'detail', 0.25, [[0, 0.35], [0.016, 0.35], [0.016, 0.0]], [1.25, 0.236, 0], axis=(0, 1, 0))]
    if d == 200:
        out = []
        for id, bone, mat, tone, length, half, top in (('battery-pack', 'battery', 'battery-housing', 0.55, 6.3, 1.04, 0.32), ('hvac-unit', 'hvac', 'hvac-housing', 0.6, 2.5, 0.9, 0.25)):
            out.append(loft(id, bone, mat, 'body', tone, [xsec(0.0, -0.05, 0.03, half - 0.08, 5), xsec(0.26, -0.05, top, half, 8), xsec(length - 0.26, -0.05, top, half, 8), xsec(length, -0.05, 0.03, half - 0.08, 5)]))
        return out
    return [loft('battery-pack', 'battery', 'battery-housing', 'body', 0.55, [xsec(0.05, -0.05, 0.32, 1.04, 6), xsec(6.25, -0.05, 0.32, 1.04, 6)]), loft('hvac-unit', 'hvac', 'hvac-housing', 'body', 0.6, [xsec(0.05, -0.05, 0.25, 0.9, 6), xsec(2.45, -0.05, 0.25, 0.9, 6)])]


def thermal(d, body):
    by = {s['id']: s for s in body}
    t = []
    for tag, side in (('l', 1), ('r', -1)):
        inward = '-w' if side > 0 else '+w'
        t.append(of(f'th-side-lower-{tag}', f'side-lower-{tag}', 'body-panels', None, [inward]))
        t.append(of(f'th-side-upper-{tag}', f'side-upper-{tag}', 'body-panels', None, [inward]))
        if f'a-pillar-{tag}' in by:
            t.append(of(f'th-a-pillar-{tag}', f'a-pillar-{tag}', 'body-panels'))
    for s in body:
        i = s['id']
        if i.startswith('mullion-'):
            t.append(of(f'th-{i}', i, 'body-panels', None, ['-w' if '-l-' in i else '+w']))
        if i.startswith('glass-'):
            t.append(of(f'th-{i}', i, 'glazing', None, ['-w' if '-l-' in i else '+w']))
        if i.startswith('leaf-') and 'glass' in i:
            t.append(of(f'th-{i}', i, 'glazing', None, ['+w']))
        elif i.startswith('leaf-'):
            t.append(of(f'th-{i}', i, 'doors', None, ['+w']))
    t += [of('th-nose', 'nose', 'body-panels', None, ['-u']), of('th-tail', 'tail', 'body-panels', None, ['+u']), of('th-tail-upper', 'tail-upper', 'body-panels', None, ['+u'])]
    for i in ('floor', 'floor-front', 'floor-rear'):
        t.append(of(f'th-{i}', i, 'body-panels', None, ['+v']))
    for i, back in (('bumper-front', '-u'), ('bumper-rear', '+u')):
        if i in by:
            t.append(of(f'th-{i}', i, 'body-panels', None, [back]))
    if d == 100:
        o, inn = roof_pts(d), roof_pts(d, WALL[d])
        k = 4
        for tag, po, pi, faces in (('r', o[:k + 1], inn[:k + 1], ['+w', '-v']), ('c', o[k:len(o) - k], inn[k:len(o) - k], ['-v']), ('l', o[len(o) - k - 1:], inn[len(o) - k - 1:], ['-w', '-v'])):
            s = copy.deepcopy(by['roof'])
            s.update({"id": f'th-roof-{tag}', "region": 'roof', "sectionFaces": faces, "profile": [[rr(a), rr(b)] for a, b in po + list(reversed(pi))]})
            t.append(s)
    else:
        t.append(of('th-roof', 'roof', 'roof', None, ['-v']))
    t += [of('th-header', 'header', 'body-panels', None, ['-u', '-v']), of('th-tail-cap', 'tail-cap', 'body-panels', None, ['+u', '-v'])]
    t.append(of('th-windscreen', 'windscreen', 'glazing', None, ['-u']))
    if 'rear-window' in by:
        t.append(of('th-rear-window', 'rear-window', 'glazing', None, ['+u']))
    t += [of('th-battery-pack', 'battery-pack', 'battery-pack'), of('th-hvac-unit', 'hvac-unit', 'hvac-unit')]
    if 'hvac-fan' in by:
        t.append({**copy.deepcopy(by['hvac-fan']), 'id': 'th-hvac-vent', 'region': 'hvac-unit', 'tags': ['emitter', 'hvac-vent']})
    else:
        top = 0.25 if d == 200 else 0.25
        thick = 0.03 if d == 200 else 0.075
        t.append({**rev('th-hvac-vent', 'hvac', 'hvac-housing', 'detail', 0.25, [[0, 0.35], [thick, 0.35], [thick, 0.0]], [1.25, top - thick + 0.004, 0], axis=(0, 1, 0)), 'region': 'hvac-unit', 'tags': ['emitter', 'hvac-vent']})
    t.append(of('th-axle-beam-r', 'axle-beam-r', 'e-axle'))
    for tag, side in (('l', 1), ('r', -1)):
        if d < 500:
            t.append(of(f'th-e-axle-motor-{tag}', f'e-axle-motor-{tag}', 'e-axle'))
            t.append(of(f'th-e-axle-spindle-{tag}', f'e-axle-spindle-{tag}', 'e-axle'))
            if d == 100:
                t.append(of(f'th-e-axle-drop-{tag}', f'e-axle-drop-{tag}', 'e-axle'))
                t.append(of(f'th-spring-carrier-r-{tag}', f'spring-carrier-r-{tag}', 'e-axle'))
            for w in (f'f{tag}', f'r{tag}', f'r{tag}-inner'):
                if d == 100:
                    t.append(of(f'th-{w}-tyre', f'{w}-tyre', 'tyres'))
                else:
                    s = copy.deepcopy(by[f'{w}-tyre'])
                    s.update({"id": f'th-{w}-tyre', "region": 'tyres', "profile": [[-0.1375, 0.305], [-0.1375, 0.44], [-0.10, 0.48], [0.10, 0.48], [0.1375, 0.44], [0.1375, 0.305], [0.02, 0.29], [-0.02, 0.29], [-0.1375, 0.305]]})
                    t.append(s)
            for w in (f'f{tag}', f'r{tag}'):
                t.append(of(f'th-{w}-brake-disc', f'{w}-brake-disc', 'brakes'))
        else:
            t.append(of(f'th-wheel-f{tag}', f'wheel-f{tag}', 'tyres'))
            t.append(of(f'th-wheel-r{tag}', f'wheel-r{tag}', 'tyres'))
            for w, bone, z in ((f'f{tag}', 'wheel-f', side * (Z_FRONT - 0.2)), (f'r{tag}', 'wheel-r', side * 0.56)):
                k = rev(f'th-{w}-brake-disc', bone, 'cast-iron', 'detail', 0.35, [[-0.04, 0.11], [0.04, 0.11], [0.04, 0.215], [-0.04, 0.215], [-0.04, 0.11]], [0, 0, z])
                k['region'] = 'brakes'
                t.append(k)
    return t


def structure(d, body):
    by = {s['id']: s for s in body}
    h, w = {100: (0.06, 0.05), 200: (0.07, 0.06), 500: (0.09, 0.08)}[d]
    stations = {100: [0.4, 1.75, 4.35, 5.65, 6.95, 8.25, 10.75, 11.4], 200: [0.4, 1.75, 5.65, 8.25, 11.4], 500: [0.4, 5.65, 11.4]}[d]

    def m(id, bone, p0, p1, hh=h, ww=w):
        s = bar(id, bone, STEEL, 'body', 0.4, p0, p1, hh, ww)
        s.update({"member": bone, "part": "member"})
        return s
    s = []
    inset = 0.03 + w / 2
    for tag, side in (('l', 1), ('r', -1)):
        s.append(m(f'longeron-{tag}', 'body', [0.4, 0.36, side * 0.26], [11.4, 0.36, side * 0.26], 0.1, max(w, 0.06)))
        zc = side * (hw(CANT) - inset)
        s.append(m(f'cant-rail-{tag}', 'body', [0.4, CANT, zc], [11.4, CANT, zc]))
        if d <= 200:
            for k, (a, b) in enumerate(((0.4, 2.84), (4.06, 8.74), (9.96, 11.4))):
                s.append(m(f'sill-{tag}-{k + 1}', 'body', [a, 0.36, side * (HW - inset)], [b, 0.36, side * (HW - inset)]))
            runs = ((0.4, 11.4),) if side > 0 else ((0.4, 6.23), (7.57, 10.155))
            for k, (a, b) in enumerate(runs):
                s.append(m(f'waist-rail-{tag}-{k + 1}', 'body', [a, BELT, side * (HW - inset)], [b, BELT, side * (HW - inset)]))
        for k, x in enumerate(stations):
            s.append(m(f'post-{tag}-{k + 1}-lower', 'body', [x, 0.36, side * (HW - inset)], [x, BELT, side * (HW - inset)]))
            s.append(m(f'post-{tag}-{k + 1}', 'body', [x, BELT, side * (HW - inset)], [x, CANT, zc]))
    for k, x in enumerate(stations):
        s.append(m(f'cross-member-{k + 1}', 'body', [x, 0.36, -(HW - inset)], [x, 0.36, HW - inset]))
        zc = hw(CANT) - inset
        pts = [[x, CANT, -zc], [x, 2.87, -0.95], [x, 2.87, 0.95], [x, CANT, zc]] if d >= 200 else [[x, CANT, -zc], [x, 2.875, -1.0], [x, 2.92, 0.0], [x, 2.875, 1.0], [x, CANT, zc]]
        for j, (a, b) in enumerate(zip(pts, pts[1:])):
            s.append(m(f'roof-bow-{k + 1}-{j + 1}', 'body', a, b))
    s.append(node('mass-body', 'body', [5.8, 1.3, 0], [0.45, 0.3, 0.45], 'mass', 0.6))
    for tag, side in (('l', 1), ('r', -1)):
        s.append(m(f'battery-rail-{tag}', 'battery', [0.2, 0.0, side * 0.8], [6.1, 0.0, side * 0.8], max(h, 0.08), max(w, 0.08)))
    for tag, side in (('l', 1), ('r', -1)):
        s.append(m(f'hvac-rail-{tag}', 'hvac', [0.15, 0.0, side * 0.7], [2.35, 0.0, side * 0.7], max(h, 0.08), max(w, 0.08)))
    s.append(node('mass-battery', 'battery', [3.1, 0.15, 0], [0.5, 0.1, 0.4], 'mass', 0.6))
    s.append(node('mass-hvac', 'hvac', [1.25, 0.12, 0], [0.25, 0.08, 0.25], 'mass', 0.6))
    for n in ('m-aft', 'm-fwd', 'f-aft', 'f-fwd'):
        bone = f'door-{n}'
        leaf = by[f'leaf-{n}']
        z = leaf['transform']['translation'][2]
        if d == 100:
            for e, x in (('a', 0.04), ('b', LEAF - 0.04)):
                s.append(m(f'leaf-{n}-stile-{e}', bone, [x, 0.0, z], [x, 0.76, z], 0.05, 0.02))
                s.append(m(f'leaf-{n}-stile-{e}-upper', bone, [x, 0.76, z], [x, 2.03, z + 1.27 * math.tan(PHI)], 0.05, 0.02))
            s.append(node(f'leaf-{n}-carriage', bone, [LEAF / 2, 2.03, z + 1.27 * math.tan(PHI)], [0.06, 0.03, 0.012], 'joint'))
        else:
            psi = math.atan((HW - hw(2.39)) / 2.06)
            f = 0.04 if d == 200 else 0.075
            ct, st = math.cos(psi), math.sin(psi)

            def at(x, y):
                return [x, 1.02 + y * ct, z + y * st]
            top, low = 1.03 - f / 2, -1.03 + f / 2
            for e, x in (('a', f / 2 + 0.004), ('b', LEAF - f / 2 - 0.004)):
                s.append(m(f'leaf-{n}-stile-{e}', bone, at(x, -1.03), at(x, 1.03), f, f))
            for e, y in (('top', top), ('bottom', low)):
                s.append(m(f'leaf-{n}-rail-{e}', bone, at(f + 0.004, y), at(LEAF - f - 0.004, y), f, f))
            r = 0.04 if d == 200 else 0.06
            s.append(node(f'leaf-{n}-carriage', bone, [LEAF / 2, 2.0, z + 0.98 * math.tan(psi)], [r * 1.5, r, r / 2 if d == 200 else 0.0375], 'joint'))
    s.append(sref('axle-f-beam', 'axle-beam-f', 'axle-f', 'member'))
    s.append(sref('axle-r-beam', 'axle-beam-r', 'axle-r', 'member'))
    for tag, side in (('l', 1), ('r', -1)):
        if d == 100:
            s.append(sref(f'axle-f-spring-{tag}', f'air-spring-f-{tag}', 'axle-f', 'joint'))
            for e in ('a', 'b'):
                s.append(sref(f'axle-r-spring-{tag}-{e}', f'air-spring-r-{tag}-{e}', 'axle-r', 'joint'))
            s.append(sref(f'wheel-f-hub-{tag}', f'f{tag}-hub', 'wheel-f', 'joint'))
            s.append(sref(f'wheel-r-hub-{tag}', f'r{tag}-hub', 'wheel-r', 'joint'))
        else:
            r = 0.09 if d == 200 else 0.1
            s.append(pin(f'axle-f-spring-{tag}', 'axle-f', [0, 0.08 if d == 200 else 0.0, side * 0.70], (0, 1, 0), 0.3 if d == 200 else 0.2, r + 0.02))
            s.append(pin(f'axle-r-spring-{tag}', 'axle-r', [0, -0.2, side * 0.47] if d == 200 else [0, -0.25, side * 0.3], (0, 1, 0), 0.3 if d == 200 else 0.15, r + 0.02))
            s.append(pin(f'wheel-f-hub-{tag}', 'wheel-f', [0, 0, side * (Z_FRONT - 0.2 if d == 500 else Z_FRONT)], (0, 0, side), 0.16, r * 0.8))
            s.append(pin(f'wheel-r-hub-{tag}', 'wheel-r', [0, 0, side * 0.835], (0, 0, side), 0.16, r * 0.8))
        if d < 500:
            s.append(sref(f'wheel-f-rim-{tag}', f'f{tag}-rim', 'wheel-f', 'member'))
            s.append(sref(f'wheel-r-rim-{tag}', f'r{tag}-rim', 'wheel-r', 'member'))
            s.append(sref(f'wheel-r-rim-{tag}-inner', f'r{tag}-inner-rim', 'wheel-r', 'member'))
        else:
            for bone, z, hw_ in (('wheel-f', side * Z_FRONT, 0.11), ('wheel-r', side * 0.915, 0.27)):
                k = rev(f'{bone}-rim-{tag}', bone, STEEL, 'detail', 0.5, [[-hw_, 0], [-hw_, 0.29], [hw_, 0.29], [hw_, 0]], [0, 0, z])
                k.update({"member": bone, "part": "member"})
                s.append(k)
    s.append(node('mass-axle-f', 'axle-f', [0, -0.31, 0], [0.1, 0.05, 0.1] if d < 500 else [0.1, 0.06, 0.1], 'mass', 0.6))
    s.append(node('mass-axle-r', 'axle-r', [0, -0.29, 0], [0.12, 0.06, 0.12], 'mass', 0.6))
    s.append(node('mass-wheel-f', 'wheel-f', [0, 0, 0], [0.08, 0.08, 0.08], 'mass', 0.6))
    s.append(node('mass-wheel-r', 'wheel-r', [0, 0, 0], [0.1, 0.1, 0.1], 'mass', 0.6))
    return s


DECISIONS = {
    100: [
        {"en": "Body at 1:100: the same shell construction with 60 mm walls. One opening and one flush pane per bay with its hopper transom, the windscreen as one curved 30 mm pane between the A-pillars, nose with bumper, lamp clusters and destination display, tail walls with cap, bumper, rear window and lamp strips, mirrors on their arms. Door leaves are solid kinked panels with two applied panes; rub rails, arch trims, wipers, plates, seals and the whole interior except floor and wheelhouses are left out. Wheels keep tyre, disc wheel, hub and brake disc; hand holes and nuts are dropped; air springs are plain cylinders; the roof units are plain fairings, the heat pump with its fan disc.", "de": "Körper bei 1:100: dieselbe Schalenkonstruktion mit 60 mm starken Wänden. Je Feld eine Öffnung und eine bündige Scheibe mit Kämpfer, die Windschutzscheibe als eine gebogene, 30 mm starke Scheibe zwischen den A-Säulen, Bug mit Stoßfänger, Leuchten und Zielanzeige, Heckwände mit Kappe, Stoßfänger, Heckscheibe und Leuchtenbändern, Spiegel an ihren Armen. Türflügel sind massive, geknickte Tafeln mit zwei aufgesetzten Scheiben; Scheuerleisten, Radlaufblenden, Wischer, Kennzeichen, Dichtungen und der ganze Innenausbau außer Boden und Radkästen entfallen. Räder behalten Reifen, Scheibenrad, Nabe und Bremsscheibe; Handlöcher und Muttern entfallen; Luftfedern sind einfache Zylinder; die Dachgeräte sind glatte Verkleidungen, die Wärmepumpe mit ihrer Lüfterscheibe."},
        {"en": "Thermal at 1:100: one solid per part and region without layer plates. Panels, frames, nose, tail and floor are the body walls with their cabin faces marked as built-in; the roof is cut into two cant strips and the crown; panes are the glazing, whole leaves the door region, their panes also glazing; fairings, fan outlet, drive axle, six tyres and four brake discs as at 1:50.", "de": "Thermik bei 1:100: je Teil und Region ein Körper ohne Schichtplatten. Bleche, Rahmen, Bug, Heck und Boden sind die Wände des Körpers, ihre Innenraumflächen als eingebaut markiert; das Dach ist in zwei Kantenstreifen und die Kuppe geteilt; Scheiben sind die Verglasung, ganze Flügel die Türregion, ihre Scheiben zusätzlich Verglasung; Verkleidungen, Lüfteraustritt, Antriebsachse, sechs Reifen und vier Bremsscheiben wie bei 1:50."},
        {"en": "Structure at 1:100: the frame reduced to eight ring frames (posts, cross member, roof bow in four pieces) with longerons, sills, waist and cant rails in 60 x 50 mm tube; leaves as two stiles with their carriage; axle beams, air springs, disc wheels and hubs are the running-gear members and joints; one mass per bone.", "de": "Tragwerk bei 1:100: das Gerippe auf acht Ringspante (Pfosten, Querträger, Dachspriegel in vier Stücken) mit Längsträgern, Schwellern, Gürtel- und Dachlängsholmen aus Rohr 60 x 50 mm reduziert; Flügel als zwei Holme mit Laufwagen; Achsträger, Luftfedern, Scheibenräder und Naben sind die Stäbe und Gelenke des Fahrwerks; je Knochen eine Masse."},
    ],
    200: [
        {"en": "Body at 1:200: 80 mm walls with one glazing band per window run (one on the driver's side, two between the doors) instead of single panes; the pillars are applied 90 mm strips and each run has one transom strip. Nose and tail are plain curved walls without bumpers and lamps, the windscreen one 40 mm pane, the header with its lip over the screen in five rings, the tail cap in four, the rear window flush in the tail. Each door leaf is one tilted panel with one pane. Wheels are a tyre and a dished disc, twins still separate; axles are their beams, the drive axle with the two motor housings and the stub spindles into the hubs; the roof units are four-section fairings; mirrors stay on thicker arms because they set the overall width.", "de": "Körper bei 1:200: 80 mm starke Wände mit einem Glasband je Fensterreihe (eines auf der Fahrerseite, zwei zwischen den Türen) statt einzelner Scheiben; die Pfosten sind aufgesetzte 90 mm breite Streifen, jede Reihe hat einen Kämpferstreifen. Bug und Heck sind glatte gebogene Wände ohne Stoßfänger und Leuchten, die Windschutzscheibe eine 40 mm starke Scheibe, der Frontdom mit seiner Lippe über der Scheibe in fünf Ringen, die Heckkappe in vier, die Heckscheibe bündig im Heck. Jeder Türflügel ist eine geneigte Tafel mit einer Scheibe. Räder sind Reifen und geschüsselte Scheibe, die Zwillinge noch getrennt; Achsen sind ihre Träger, die Antriebsachse mit den beiden Motorgehäusen und den Achsstummeln in die Naben; die Dachgeräte sind Verkleidungen aus vier Querschnitten; die Spiegel bleiben an dickeren Armen, weil sie die Gesamtbreite bestimmen."},
        {"en": "Thermal at 1:200: the body solids by region (walls with built-in cabin faces, pillar strips, roof, glazing bands, leaves, fairings, fan outlet, drive axle, tyres down to the rim flange, brake discs); only the total area is compared at this scale.", "de": "Thermik bei 1:200: die Körper des Aufbaus nach Regionen (Wände mit eingebauten Innenraumflächen, Pfostenstreifen, Dach, Glasbänder, Flügel, Verkleidungen, Lüfteraustritt, Antriebsachse, Reifen bis zum Felgenhorn, Bremsscheiben); bei diesem Maßstab wird nur die Gesamtfläche verglichen."},
        {"en": "Structure at 1:200: five ring frames with longerons, sills, waist and cant rails in 70 x 60 mm tube; each leaf is a frame of two stiles and two rails in 40 mm section on a carriage; axle beams and disc wheels with one spring and one hub joint each side; one mass per bone.", "de": "Tragwerk bei 1:200: fünf Ringspante mit Längsträgern, Schwellern, Gürtel- und Dachlängsholmen aus Rohr 70 x 60 mm; jeder Flügel ist ein Rahmen aus zwei Holmen und zwei Riegeln mit 40 mm Querschnitt an einem Laufwagen; Achsträger und Scheibenräder mit je einem Feder- und einem Nabengelenk je Seite; je Knochen eine Masse."},
    ],
    500: [
        {"en": "Body at 1:500: the long box with its real cross-section in 100 mm walls, cant radius in four straight pieces, both wheel arches, three glazing bands, the curved windscreen, the header with its lip over the screen in four rings, the tail cap in three, the rear window; four plain door panels of 75 mm, two roof units with their rounded section, four wheels as discs (the rear twins merged into one wide disc), two axle beams and the mirrors as ears on the body side, kept because they set the overall width.", "de": "Körper bei 1:500: der lange Kasten mit seinem wirklichen Querschnitt in 100 mm starken Wänden, der Kantenradius in vier geraden Stücken, beide Radausschnitte, drei Glasbänder, die gebogene Windschutzscheibe, der Frontdom mit seiner Lippe über der Scheibe in vier Ringen, die Heckkappe in drei, die Heckscheibe; vier glatte, 75 mm starke Türtafeln, zwei Dachgeräte mit ihrem gerundeten Querschnitt, vier Räder als Scheiben (die hinteren Zwillinge zu einer breiten Scheibe verschmolzen), zwei Achsträger und die Spiegel als Ohren an der Wand, beibehalten, weil sie die Gesamtbreite bestimmen."},
        {"en": "Thermal at 1:500: walls, roof, glazing bands, leaves, roof units with the fan outlet patch, the drive axle beam, the four wheel discs as tyres and four brake discs inside them; only the total area is compared.", "de": "Thermik bei 1:500: Wände, Dach, Glasbänder, Flügel, Dachgeräte mit der Austrittsfläche des Lüfters, der Antriebsachsträger, die vier Radscheiben als Reifen und vier Bremsscheiben darin; verglichen wird nur die Gesamtfläche."},
        {"en": "Structure at 1:500: two longerons and two cant rails tied by three ring frames in 90 x 80 mm tube, each door leaf a frame of two stiles and two rails in 75 mm section on its carriage, the two axle beams, a rim cylinder in each wheel, one spring and one hub joint each side, one mass per bone; the model is minimal for this scale.", "de": "Tragwerk bei 1:500: zwei Längsträger und zwei Dachlängsholme, verbunden durch drei Ringspante aus Rohr 90 x 80 mm, jeder Türflügel ein Rahmen aus zwei Holmen und zwei Riegeln mit 75 mm Querschnitt an seinem Laufwagen, die beiden Achsträger, ein Felgenzylinder in jedem Rad, je Seite ein Feder- und ein Nabengelenk, je Knochen eine Masse; das Modell ist für diesen Maßstab minimal."},
    ],
}


def pin(id, bone, origin, axis, length, r):
    """a joint as a short cylinder from the origin along the axis"""
    s = rev(id, bone, STEEL, 'detail', 0.5, [[0, 0], [0, r], [length, r], [length, 0]], origin, axis)
    s.update({"member": bone, "part": "joint"})
    return s


def scale(d):
    body = shell(d) + ends(d) + doors(d) + running_gear(d) + roof_equipment(d)
    return {"body": body, "thermal": thermal(d, body), "structure": structure(d, body), "decisions": DECISIONS[d]}
