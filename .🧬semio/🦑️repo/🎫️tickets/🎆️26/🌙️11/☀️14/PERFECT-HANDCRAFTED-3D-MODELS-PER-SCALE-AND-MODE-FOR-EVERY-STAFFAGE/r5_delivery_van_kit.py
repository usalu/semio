"""🧰 Round 5 writers of the delivery van shared by its scales: the tall commercial door mirror as a lofted housing with its glass on the rear face, and the flared wheel-arch lips on the flat body side, laid on the round 3 master lines of `r3_delivery_van_kit`.

Reference: Ford Transit Custom / VW T6 door mirrors (housing about 0.18 m wide, 0.27 m tall, 0.12 m deep, glass split in a main and a wide-angle pane), arch radius 0.42 m around the 0.356 m tyre.
"""
import math

from r3_delivery_van_kit import RA, YA, box, loft, sec
from r5_city_car_kit import framed


def mirror(id, sign, x0, y0, root, tip, rows, **fields):
    """🪞 A door-mirror housing lofted out from the arm: rows (share of the reach, half depth, half height)."""
    return loft(id, [sec([x0, y0, sign * (root + (tip - root) * share)], depth, height, 2.6) for share, depth, height in rows], **fields)


def mirror_glass(id, sign, x, y, z, half_h, half_w, t, **fields):
    """🔲 The glass of a door mirror on the rear face of its housing."""
    return box(id, [t, 2 * half_h, 2 * half_w], [x, y, sign * z], radius=round(t / 2.2, 4), **fields)


def arch_lip(id, sign, xc, z_side, radius=0.012, a0=-8, a1=188, steps=16, **fields):
    """🪶 The flared lip round a wheel arch on the flat side: a round bead on the arch edge standing proud of the side."""
    out = []
    for k in range(steps + 1):
        a = math.radians(a0 + (a1 - a0) * k / steps)
        point = [xc + (RA + 0.004) * math.cos(a), YA + (RA + 0.004) * math.sin(a), sign * (z_side + radius * 0.4)]
        out.append(framed(point, [math.cos(a), math.sin(a), 0.0], [math.sin(a), -math.cos(a), 0.0], radius, radius, 2.5))
    return loft(id, out, **fields)
