"""🚍️ Round 3 city-bus: the 1:50 body, stated assembly by assembly (shell with real openings, glazing, doors, running gear, roof equipment)."""
import math
from r3_city_bus_kit import *
import r3_city_bus_kit as K

PAINT, GLASS, TRIM, RUBBER, RIM, LAMP = 0.82, 0.28, 0.18, 0.1, 0.68, 0.92
S0, S1 = SILL_GLASS - BELT, HEAD_GLASS - BELT
S_DOOR = (DOOR_TOP - BELT) / math.cos(PHI)


def centroid(p):
    a = cx = cy = 0.0
    for (x0, y0), (x1, y1) in zip(p, p[1:] + p[:1]):
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    return cx / (3 * a), cy / (3 * a)


GLASS_PROFILE = dband(GLASS_D, GLASS_W, NOSE_N, 0.02, 0.0, 1, 0.02, math.pi - 0.02)
K.GLASS_CX = centroid(GLASS_PROFILE)[1]


def shell():
    b = []
    for side, tag, doors, bays in ((1, 'l', False, BAYS_L), (-1, 'r', True, BAYS_R)):
        z0, z1 = (HW - TW, HW) if side > 0 else (-HW, -HW + TW)
        b.append(ext(f'side-lower-{tag}', 'body', 'paint', 'body', PAINT, 'xy', lower_profile(doors), z0, z1))
        holes = [rect(a, S0, c, S1) for a, c in bays]
        zc = (-TW, 0) if side > 0 else (0, TW)
        b.append(ext(f'side-upper-{tag}', 'body', 'paint', 'body', PAINT, 'xy', upper_profile(doors, S_DOOR), zc[0], zc[1], holes=holes, at=[0, BELT, side * HW], rot=qx(-side * PHI)))
        for k, (a, c) in enumerate(bays):
            sm = (S0 + S1) / 2
            b.append(box(f'pane-{tag}-{k + 1}', 'body', 'glass', 'glazing', GLASS, [c - a + 0.02, S1 - S0 + 0.02, 0.02], on_wall(side, (a + c) / 2, sm, -side * 0.016), rot=qx(-side * PHI)))
            if c - a > 0.9:
                b.append(box(f'transom-{tag}-{k + 1}', 'body', 'paint', 'detail', TRIM, [c - a + 0.01, 0.03, 0.014], on_wall(side, (a + c) / 2, 1.02, -side * 0.001), rot=qx(-side * PHI)))
    b.append(ext('roof', 'body', 'paint', 'body', PAINT + 0.04, 'yz', roof_outline() + list(reversed(roof_outline(TW))), X0, X1))
    w = HW - 0.002
    a = ARCH_HALF + 0.03
    zr, zf = Z_WELL_R - 0.03, Z_WELL_F - 0.03
    half = [[-w, X0], [-w, XR - a], [-zr, XR - a], [-zr, XR + a], [-w, XR + a], [-w, XF - a], [-zf, XF - a], [-zf, XF + a], [-w, XF + a], [-w, X1]]
    b.append(ext('floor', 'body', 'paint', 'body', 0.45, 'zx', half + [[-z, x] for z, x in reversed(half)], SKIRT, FLOOR))
    ao = math.asin((AXLE - SKIRT) / 0.61)
    for xa, zin, n in ((XR, Z_WELL_R, 'r'), (XF, Z_WELL_F, 'f')):
        for side, tag in ((1, 'l'), (-1, 'r')):
            tub = arc(xa, AXLE, 0.61, math.pi + ao, -ao, 24) + list(reversed(arch(xa, R_ARCH, 24)))
            b.append(ext(f'wheelhouse-{n}-{tag}', 'body', 'paint', 'body', 0.3, 'xy', tub, side * zin, side * (HW - TW + 0.005)))
            b.append(ext(f'wheelhouse-wall-{n}-{tag}', 'body', 'paint', 'body', 0.3, 'xy', arc(xa, AXLE, 0.61, math.pi + ao, -ao, 24), side * (zin - 0.03), side * zin))
            b.append(ext(f'arch-trim-{n}-{tag}', 'body', 'rubber', 'detail', TRIM, 'xy', arc(xa, AXLE, 0.635, math.pi + ARCH_A, -ARCH_A, 28) + list(reversed(arch(xa, R_ARCH - 0.004, 28))), side * (HW - 0.012), side * (HW + 0.008)))
            b.append(box(f'spring-seat-{n}-{tag}', 'body', 'steel-wheel', 'detail', 0.3, [0.26, 0.2, 0.24], [xa + (0.0 if n == 'f' else 0.34), 0.98 if n == 'f' else 0.72, side * (0.70 if n == 'f' else 0.47)]))
            if n == 'r':
                b.append(box(f'spring-seat-ra-{tag}', 'body', 'steel-wheel', 'detail', 0.3, [0.26, 0.2, 0.24], [xa - 0.34, 0.72, side * 0.47]))
    return b


def ends():
    b = []
    b.append(ext('nose', 'body', 'paint', 'body', PAINT, 'zx', dband(NOSE_D, HW, NOSE_N, TW, X1), SKIRT, 1.19))
    b.append(ext('bumper-front', 'body', 'paint', 'detail', 0.5, 'zx', dband(0.55, HW, NOSE_N, 0.04, X1), FLOOR, 0.62))
    b.append(ext('floor-front', 'body', 'paint', 'body', 0.45, 'zx', dfill(NOSE_D - 0.005, HW - 0.005, NOSE_N, X1), SKIRT, FLOOR))
    b.append(ext('windscreen', 'body', 'glass', 'glazing', GLASS, 'zx', GLASS_PROFILE, 0, GLASS_H, at=[X1, SILL_GLASS, 0], rot=qz(RAKE), taper=[1, GLASS_K]))
    t0, t1 = 1.12, math.pi - 1.12
    sign = [[z, dx] for z, dx in dcurve(0.531, GLASS_W, NOSE_N, t0, t1, 14)] + [[z, dx] for z, dx in reversed(dcurve(0.522, GLASS_W - 0.009, NOSE_N, t0, t1, 14))]
    b.append(ext('destination-sign', 'body', 'glass', 'glazing', 0.08, 'zx', sign, 1.19, 1.47, at=[X1, SILL_GLASS, 0], rot=qz(RAKE)))
    b.append(loft('header', 'body', 'paint', 'body', PAINT, [ring(y, X1, rx, hw(y), NOSE_N) for y, rx in ((2.66, 0.365), (2.72, 0.365), (2.80, 0.34), (2.88, 0.295), (2.93, 0.235), (2.96, 0.155), (2.975, 0.08))]))
    for side, tag in ((1, 'l'), (-1, 'r')):
        b.append(loft(f'a-pillar-{tag}', 'body', 'paint', 'body', PAINT, [sec([11.51, 1.08, side * (HW - 0.04)], 0.08, 0.04, 4), sec([11.335, 2.72, side * (hw(2.72) - 0.04)], 0.08, 0.04, 4)]))
        za, zb = sorted((side * 0.70, side * 1.10))
        b.append(ext(f'headlamp-{tag}', 'body', 'glass', 'glazing', LAMP, 'zx', zband(NOSE_D + 0.008, NOSE_D - 0.01, NOSE_N, za, zb, X1), 0.70, 0.88))
        za, zb = sorted((side * 1.12, side * 1.22))
        b.append(ext(f'indicator-front-{tag}', 'body', 'glass', 'glazing', 0.7, 'zx', zband(NOSE_D + 0.008, NOSE_D - 0.01, NOSE_N, za, zb, X1, m=6), 0.70, 0.88))
        p0, p1 = glass_pt(0.12, side * 0.74, 0.016), glass_pt(0.12, side * 0.04, 0.016)
        b.append(cap(f'wiper-blade-{tag}', 'body', 'rubber', 'detail', 0.1, p0, p1, 0.009))
        mid = [(u + v) / 2 for u, v in zip(p0, p1)]
        b.append(cap(f'wiper-arm-{tag}', 'body', 'rubber', 'detail', 0.1, [X1 + nose_dx(side * 0.2) - 0.012, 1.185, side * 0.2], mid, 0.008))
    b.append(ext('grille-front', 'body', 'rubber', 'detail', TRIM, 'zx', zband(0.552, 0.54, NOSE_N, -0.62, 0.62, X1), 0.37, 0.43))
    b.append(box('plate-front', 'body', 'paint', 'detail', 0.95, [0.012, 0.11, 0.52], [11.992, 0.52, 0]))
    tail = dband(TAIL_D, HW, TAIL_N, TW, X0, -1)
    b.append(ext('tail', 'body', 'paint', 'body', PAINT, 'zx', tail, SKIRT, BELT))
    b.append(ext('tail-upper', 'body', 'paint', 'body', PAINT, 'zx', tail, BELT, 2.56, taper=[1, rr(hw(2.56) / HW)]))
    b.append(loft('tail-cap', 'body', 'paint', 'body', PAINT, [ring(y, X0, rx, hw(y), TAIL_N) for y, rx in ((2.55, TAIL_D), (2.75, 0.315), (2.86, 0.275), (2.93, 0.21), (2.965, 0.12))]))
    b.append(ext('floor-rear', 'body', 'paint', 'body', 0.45, 'zx', dfill(TAIL_D - 0.005, HW - 0.005, TAIL_N, X0, -1), SKIRT, FLOOR))
    b.append(ext('bumper-rear', 'body', 'paint', 'detail', 0.5, 'zx', dband(0.35, HW, TAIL_N, 0.04, X0, -1), FLOOR, 0.60))
    b.append(box('rear-window', 'body', 'glass', 'glazing', GLASS, [0.02, 0.80, 1.70], [0.017, 1.92, 0]))
    b.append(box('rear-hatch', 'body', 'paint', 'detail', PAINT - 0.07, [0.012, 0.74, 1.76], [0.017, 1.02, 0], corner=0.004))
    b.append(box('plate-rear', 'body', 'paint', 'detail', 0.95, [0.012, 0.11, 0.52], [0.008, 0.47, 0]))
    b.append(box('brake-lamp-high', 'body', 'glass', 'glazing', 0.6, [0.012, 0.04, 0.5], [0.022, 2.62, 0]))
    for side, tag in ((1, 'l'), (-1, 'r')):
        b.append(box(f'tail-lamp-{tag}', 'body', 'glass', 'glazing', 0.6, [0.03, 0.52, 0.11], [0.036, 1.18, side * 1.03], corner=0.012))
    b.append(box('mirror-r', 'body', 'paint', 'detail', TRIM, [0.07, 0.42, 0.2], [11.80, 2.05, MIRROR_R + 0.1], corner=0.03))
    b.append(cap('mirror-arm-r', 'body', 'paint', 'detail', TRIM, [11.47, 2.58, -1.245], [11.80, 2.27, MIRROR_R + 0.1], 0.017))
    b.append(box('mirror-l', 'body', 'paint', 'detail', TRIM, [0.07, 0.36, 0.18], [11.72, 2.0, MIRROR_L - 0.09], corner=0.03))
    b.append(cap('mirror-arm-l', 'body', 'paint', 'detail', TRIM, [11.5, 2.02, 1.255], [11.72, 2.02, MIRROR_L - 0.09], 0.017))
    return b


def trim():
    b = []
    for tag, side, runs in (('l', 1, ((X0, 2.80), (4.10, 8.70), (10.0, X1))), ('r', -1, ((X0, 2.80), (4.10, DOOR_M[0]), (DOOR_M[1], 8.70), (10.0, DOOR_F[0])))):
        for k, (a, c) in enumerate(runs):
            b.append(box(f'rub-rail-{tag}-{k + 1}', 'body', 'rubber', 'detail', TRIM, [c - a, 0.05, 0.014], [(a + c) / 2, 0.62, side * (HW + 0.004)], corner=0.005))
    b.append(box('vent-grille', 'body', 'paint', 'detail', TRIM, [0.9, 0.7, 0.012], on_wall(1, 1.07, 0.75, 0.003), rot=qx(-PHI)))
    b.append(box('charge-flap', 'body', 'paint', 'detail', PAINT - 0.07, [0.34, 0.3, 0.012], [8.2, 0.86, -(HW + 0.003)], corner=0.004))
    return b


def doors():
    b = []
    for n in ('m-aft', 'm-fwd', 'f-aft', 'f-fwd'):
        bone = f'door-{n}'
        zo = -(HW + 0.003)
        b.append(ext(f'leaf-{n}', bone, 'door-leaf', 'body', 0.6, 'xy', rect(0.004, -0.01, LEAF - 0.004, 0.762), zo - 0.028, zo, holes=[rect(0.07, 0.16, LEAF - 0.07, 0.70)]))
        s_top = (2.39 - BELT) / math.cos(PHI)
        b.append(ext(f'leaf-{n}-upper', bone, 'door-leaf', 'body', 0.6, 'xy', rect(0.004, 0, LEAF - 0.004, s_top), -0.028, 0, holes=[rect(0.07, 0.06, LEAF - 0.07, 1.18)], at=[0, BELT - FLOOR, zo], rot=qx(PHI)))
        b.append(box(f'leaf-{n}-glass-lower', bone, 'glass', 'glazing', GLASS, [LEAF - 0.12, 0.56, 0.02], [LEAF / 2, 0.43, zo - 0.014]))
        s, zc = 0.62, -0.014
        b.append(box(f'leaf-{n}-glass', bone, 'glass', 'glazing', GLASS, [LEAF - 0.12, 1.14, 0.02], [LEAF / 2, BELT - FLOOR + s * math.cos(PHI) - zc * math.sin(PHI), zo + s * math.sin(PHI) + zc * math.cos(PHI)], rot=qx(PHI)))
        edge = LEAF - 0.012 if n.endswith('aft') else 0.012
        b.append(box(f'leaf-{n}-seal', bone, 'rubber', 'detail', 0.1, [0.024, 2.0, 0.034], [edge, 1.03, zo - 0.014 + 0.012], rot=qx(PHI * 0.5)))
    return b


def wheel(pre, bone, zc, side, d, grooves=(-0.05, 0.0, 0.05)):
    """one wheel about the axle line: tyre, rim, the ten hand holes; d = +1 mounting face outboard"""
    b = [rev(f'{pre}-tyre', bone, 'rubber', 'body', RUBBER, tyre_loop(grooves=grooves), [0, 0, zc]),
         rev(f'{pre}-rim', bone, 'steel-wheel', 'detail', RIM, rim_loop(d * side), [0, 0, zc])]
    a = math.radians(18)
    b.append(ell(f'{pre}-hand-hole', bone, 'cast-iron', 'detail', 0.2, [0.027, 0.02, 0.009], [0.237 * math.cos(a), 0.237 * math.sin(a), zc + d * side * 0.135], rot=qz(a), repeat={"count": 10, "rotationAxis": [0, 0, 1], "rotationRad": rr(math.pi / 5)}))
    return b


def running_gear():
    b = []
    for side, tag in ((1, 'l'), (-1, 'r')):
        zc = side * Z_FRONT
        b += wheel(f'f{tag}', 'wheel-f', zc, side, 1)
        b.append(rev(f'f{tag}-hub', 'wheel-f', 'steel-wheel', 'detail', 0.5, [[side * a, r] for a, r in ((0.215, 0), (0.212, 0.045), (0.195, 0.072), (0.165, 0.08), (0.14, 0.08))], [0, 0, zc]))
        b.append(cap(f'f{tag}-nut', 'wheel-f', 'steel-wheel', 'detail', 0.3, [0.1675, 0, zc + side * 0.158], [0.1675, 0, zc + side * 0.18], 0.013, repeat={"count": 10, "rotationAxis": [0, 0, 1], "rotationRad": rr(math.pi / 5)}))
        b.append(rev(f'f{tag}-brake-disc', 'wheel-f', 'cast-iron', 'detail', 0.35, [[side * a, r] for a, r in ((-0.07, 0.11), (-0.025, 0.11), (-0.025, 0.215), (-0.07, 0.215), (-0.07, 0.11))], [0, 0, zc]))
        zo, zi = side * Z_REAR_OUT, side * Z_REAR_IN
        b += wheel(f'r{tag}', 'wheel-r', zo, side, -1)
        b += wheel(f'r{tag}-inner', 'wheel-r', zi, side, 1, grooves=())
        zm = side * (Z_REAR_OUT + Z_REAR_IN) / 2
        b.append(rev(f'r{tag}-hub', 'wheel-r', 'steel-wheel', 'detail', 0.5, [[side * a, r] for a, r in ((0.105, 0), (0.105, 0.06), (0.085, 0.088), (0.0, 0.088))], [0, 0, zm]))
        b.append(cap(f'r{tag}-nut', 'wheel-r', 'steel-wheel', 'detail', 0.3, [0.1675, 0, zm + side * 0.01], [0.1675, 0, zm + side * 0.034], 0.013, repeat={"count": 10, "rotationAxis": [0, 0, 1], "rotationRad": rr(math.pi / 5)}))
        b.append(rev(f'r{tag}-brake-disc', 'wheel-r', 'cast-iron', 'detail', 0.35, [[side * a, r] for a, r in ((-0.07, 0.11), (-0.025, 0.11), (-0.025, 0.215), (-0.07, 0.215), (-0.07, 0.11))], [0, 0, zi]))
    b.append(loft('axle-beam-f', 'axle-f', 'steel-wheel', 'detail', 0.3, [sec([0, 0, -0.82], 0.06, 0.055, 4), sec([0, -0.31, -0.62], 0.06, 0.055, 4), sec([0, -0.31, 0.62], 0.06, 0.055, 4), sec([0, 0, 0.82], 0.06, 0.055, 4)]))
    for side, tag in ((1, 'l'), (-1, 'r')):
        b.append(cap(f'stub-axle-f-{tag}', 'axle-f', 'steel-wheel', 'detail', 0.3, [0, 0, side * 0.80], [0, 0, side * 1.0], 0.05))
        b.append(rev(f'air-spring-f-{tag}', 'axle-f', 'rubber', 'detail', 0.15, [[0, 0.07], [0.02, 0.105], [0.09, 0.125], [0.16, 0.105], [0.23, 0.125], [0.30, 0.105], [0.32, 0.0]], [0, 0.08, side * 0.70], axis=(0, 1, 0)))
        b.append(box(f'spring-carrier-f-{tag}', 'axle-f', 'steel-wheel', 'detail', 0.3, [0.2, 0.3, 0.1], [0, -0.05, side * 0.70]))
    b.append(loft('axle-beam-r', 'axle-r', 'e-axle', 'detail', 0.3, [sec([0, -0.29, -0.42], 0.13, 0.085, 5), sec([0, -0.29, 0.42], 0.13, 0.085, 5)]))
    for side, tag in ((1, 'l'), (-1, 'r')):
        b.append(rev(f'e-axle-motor-{tag}', 'axle-r', 'e-axle', 'detail', 0.32, [[side * a, r] for a, r in ((0.36, 0), (0.36, 0.18), (0.39, 0.22), (0.60, 0.22), (0.63, 0.17), (0.66, 0.10), (0.66, 0))], [0, 0, 0]))
        b.append(rev(f'e-axle-spindle-{tag}', 'axle-r', 'e-axle', 'detail', 0.3, [[side * a, r] for a, r in ((0.66, 0), (0.66, 0.1), (0.915, 0.1), (0.915, 0))], [0, 0, 0]))
        b.append(box(f'e-axle-drop-{tag}', 'axle-r', 'e-axle', 'detail', 0.3, [0.24, 0.3, 0.14], [0, -0.16, side * 0.42]))
        b.append(box(f'spring-carrier-r-{tag}', 'axle-r', 'e-axle', 'detail', 0.3, [0.8, 0.07, 0.12], [0, -0.215, side * 0.47], corner=0.01))
        for e, x in (('a', -0.34), ('b', 0.34)):
            b.append(rev(f'air-spring-r-{tag}-{e}', 'axle-r', 'rubber', 'detail', 0.15, [[0, 0.07], [0.02, 0.105], [0.09, 0.125], [0.16, 0.105], [0.23, 0.125], [0.30, 0.105], [0.32, 0.0]], [x, -0.18, side * 0.47], axis=(0, 1, 0)))
    return b


def pod_loft(id, bone, mat, tone, length, half, top, ramp):
    r = (top + 0.05) / 2
    c = top - r
    st = [(0.0, 0.02, half - 0.09, 4), (ramp * 0.35, 0.6, half - 0.04, 6), (ramp, 1.0, half, 8)]
    secs = [xsec(x, -0.05, -0.05 + 2 * r * k, w, e) for x, k, w, e in st] + [xsec(length - x, -0.05, -0.05 + 2 * r * k, w, e) for x, k, w, e in reversed(st)]
    return loft(id, bone, mat, 'body', tone, secs)


def roof_equipment():
    b = [pod_loft('battery-pack', 'battery', 'battery-housing', 0.55, 6.3, 1.04, 0.312, 0.3)]
    b.append(box('battery-module', 'battery', 'battery-housing', 'detail', 0.62, [0.9, 0.016, 1.84], [0.75, 0.312, 0], corner=0.006, repeat={"count": 6, "translation": [0.96, 0, 0]}))
    b.append(pod_loft('hvac-unit', 'hvac', 'hvac-housing', 0.6, 2.5, 0.90, 0.242, 0.22))
    b.append(rev('hvac-fan', 'hvac', 'hvac-housing', 'detail', 0.25, [[0, 0.35], [0.01, 0.35], [0.01, 0.0]], [1.25, 0.24, 0], axis=(0, 1, 0)))
    b.append(rev('hvac-fan-hub', 'hvac', 'hvac-housing', 'detail', 0.5, [[0, 0.09], [0.018, 0.08], [0.018, 0.0]], [1.25, 0.24, 0], axis=(0, 1, 0)))
    for k, x in enumerate((0.34, 1.72)):
        b.append(box(f'hvac-louvre-{k + 1}', 'hvac', 'hvac-housing', 'texture', 0.3, [0.03, 0.012, 1.4], [x, 0.242, 0], texture=True, repeat={"count": 5, "translation": [0.11, 0, 0]}))
    return b


SEAT_RUNS = {'l': ((0.95, 3), (4.55, 6)), 'r': ((0.95, 3), (4.55, 2), (7.95, 1))}
SEAT_PITCH = 0.75


def interior():
    b = []
    for tag, side in (('l', 1), ('r', -1)):
        for k, (x, n) in enumerate(SEAT_RUNS[tag]):
            rep = {"count": n, "translation": [SEAT_PITCH, 0, 0]}
            b.append(box(f'seat-{tag}-{k + 1}', 'body', 'interior-trim', 'detail', 0.35, [0.42, 0.09, 0.84], [x, 0.80, side * 0.78], corner=0.03, repeat=rep))
            b.append(box(f'seat-back-{tag}-{k + 1}', 'body', 'interior-trim', 'detail', 0.3, [0.06, 0.68, 0.84], [x - 0.24, 1.15, side * 0.78], rot=qz(0.14), corner=0.025, repeat=rep))
            b.append(box(f'seat-foot-{tag}-{k + 1}', 'body', 'interior-trim', 'detail', 0.3, [0.06, 0.42, 0.06], [x, 0.55, side * 0.48], repeat=rep))
        b.append(cap(f'grab-rail-{tag}', 'body', 'interior-trim', 'detail', 0.3, [0.7, 2.12, side * 0.5], [10.0, 2.12, side * 0.5], 0.016))
        for e, x in (('a', 6.2), ('b', 7.6)):
            b.append(cap(f'stanchion-{tag}-{e}', 'body', 'interior-trim', 'detail', 0.3, [x, FLOOR, side * 0.5], [x, 2.12, side * 0.5], 0.017))
    b.append(box('driver-partition', 'body', 'interior-trim', 'detail', 0.3, [0.04, 1.5, 0.95], [10.72, 1.09, 0.74]))
    b.append(box('dashboard', 'body', 'interior-trim', 'detail', 0.25, [0.36, 0.5, 2.0], [11.62, 0.95, 0], corner=0.08))
    b.append(box('driver-seat', 'body', 'interior-trim', 'detail', 0.3, [0.45, 0.1, 0.5], [11.02, 0.85, 0.72], corner=0.03))
    b.append(box('driver-seat-back', 'body', 'interior-trim', 'detail', 0.3, [0.08, 0.75, 0.5], [10.84, 1.25, 0.72], rot=qz(0.14), corner=0.03))
    b.append(rev('steering-wheel', 'body', 'interior-trim', 'detail', 0.15, [[-0.015, 0.205], [0.015, 0.205], [0.015, 0.235], [-0.015, 0.235], [-0.015, 0.205]], [11.36, 1.2, 0.72], axis=(-0.5, 0.866, 0)))
    b.append(cap('steering-column', 'body', 'interior-trim', 'detail', 0.15, [11.36, 1.2, 0.72], [11.52, 0.95, 0.72], 0.025))
    return b


def body50():
    return shell() + ends() + trim() + doors() + interior() + running_gear() + roof_equipment()
