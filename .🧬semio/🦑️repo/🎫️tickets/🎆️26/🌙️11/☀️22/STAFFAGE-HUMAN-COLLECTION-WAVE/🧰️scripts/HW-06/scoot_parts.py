"""🛵️ The four-wheel mobility scooter of `mobility-scooter-user`, part by part and scale by scale (world metres, rear axle at x 0,
y 0.125; the chassis bone frame is the world shifted down by the axle height). A mid-size class-B scooter after EN 12184:2022 and
catalogue values: wheelbase 0.92 m, 250 mm (10 inch) solid tyres, length 1.30 m, width 0.60 m, seat 0.56 m high, 74 kg with two
12 V 36 Ah batteries, a 24 V transaxle motor of 400 W rated output, 6 km/h.

Every part is stated by hand for each scale (`SCALE_PARTS`): what 1:50 shows as separate parts 1:500 merges or leaves out."""
import math
import hw06lib as L
import scoot_base as SB

A = SB.AXLE
R_WHEEL, W_WHEEL = 0.125, 0.08
REAR_Z, FRONT_Z, WHEELBASE = 0.235, 0.235, 0.92
BACK_DEG = 12.0


def qz(deg):
    h = math.radians(deg) / 2
    return [0, 0, round(math.sin(h), 6), round(math.cos(h), 6)]


def box(pid, c, size, r=0.0, rot=None, **kw):
    s = {"id": pid, "kind": "box", "bone": "chassis", "size": [round(x, 4) for x in size], "transform": {"translation": [round(c[0], 4), round(c[1] - A, 4), round(c[2], 4)]}}
    if r: s["cornerRadius"] = r
    if rot is not None: s["transform"]["rotation"] = qz(rot)
    s.update(kw); return s


def cap(pid, p0, p1, r0, r1=None, **kw):
    p0, p1 = tuple(p0) + (0.0,) * (3 - len(p0)), tuple(p1) + (0.0,) * (3 - len(p1))
    s = {"id": pid, "kind": "capsule", "bone": "chassis", "p0": [round(p0[0], 4), round(p0[1] - A, 4), round(p0[2], 4)],
         "p1": [round(p1[0], 4), round(p1[1] - A, 4), round(p1[2], 4)], "r0": r0, "r1": r0 if r1 is None else r1}
    s.update(kw); return s


def disc(pid, bone, z0, z1, r, hole=None, **kw):
    prof = [[round(r * math.cos(2 * math.pi * i / 32), 5), round(r * math.sin(2 * math.pi * i / 32), 5)] for i in range(32)]
    s = {"id": pid, "kind": "extrude", "bone": bone, "profile": prof, "plane": "xy", "from": round(z0, 4), "to": round(z1, 4)}
    if hole: s["holes"] = [[[round(hole * math.cos(-2 * math.pi * i / 24), 5), round(hole * math.sin(-2 * math.pi * i / 24), 5)] for i in range(24)]]
    s.update(kw); return s


MAT = {"shell": "abs-shell", "upholstery": "vinyl", "tyres": "pu-tyre", "battery-box": "battery-case", "drive-motor": "motor-alloy"}


def part(region, tone, role="body", material=None, **kw):
    return dict(region=region, material=material or MAT[region], tone=tone, role=role, **kw)


def wheels(scale):
    out = []
    for bone, zc in (("rear-wheel", REAR_Z), ("front-wheel", FRONT_Z)):
        tag = "rear" if bone == "rear-wheel" else "front"
        for side, sg in (("l", 1), ("r", -1)):
            z0, z1 = sg * zc - W_WHEEL / 2, sg * zc + W_WHEEL / 2
            if scale in ("50", "100"):
                out.append(disc(f"{tag}-tyre-{side}", bone, z0, z1, R_WHEEL, hole=0.076, **part("tyres", 0.12)))
                hz0, hz1 = (z0 + 0.006, z1 + 0.006) if sg > 0 else (z0 - 0.006, z1 - 0.006)
                out.append(disc(f"{tag}-hub-{side}", bone, hz0, hz1, 0.078, **part("shell", 0.7, material="aluminium-rim")))
            else:
                out.append(disc(f"{tag}-wheel-{side}", bone, z0, z1, R_WHEEL, **part("tyres", 0.14)))
    return out


def backrest_frame(asset_frames, asset):
    """Front face of the backrest: the plane leaning back 12 degrees that just touches the most backward point of buttocks and back."""
    c, s = math.cos(math.radians(BACK_DEG)), math.sin(math.radians(BACK_DEG))
    n = (-c, -s)
    best = None
    for so in asset["volumes"]["50"]["body"]:
        if so["bone"] not in ("pelvis", "lumbar", "torso") or so["kind"] not in ("loft", "ellipsoid", "capsule", "box"): continue
        for p in L.samples(so):
            q = L.to_world(asset_frames[so["bone"]], p)
            if q[1] < 0.6 or q[1] > 1.05: continue
            d = q[0] * n[0] + q[1] * n[1]
            if best is None or d > best: best = d
    return best + 0.002


def chassis(scale, back_d):
    c, s = math.cos(math.radians(BACK_DEG)), math.sin(math.radians(BACK_DEG))
    t_back = 0.08 if scale != "500" else 0.12
    yb = 0.80
    xf = (back_d - (0.03 if scale == "500" else 0.0) + yb * s) / (-c)
    centre = (xf - (t_back / 2) * c, yb - (t_back / 2) * s, 0.0)
    parts = []
    sh, up, bt, mo = part("shell", 0.45), part("upholstery", 0.22), part("battery-box", 0.34), part("drive-motor", 0.58)
    dark = part("shell", 0.3, material="steel-tube")
    parts += [box("rear-shroud", (0.04, 0.315, 0), (0.50, 0.09, 0.60), 0.025 if scale in ("50", "100") else 0.0, **sh),
              box("battery-box", (0.07, 0.185, 0), (0.38, 0.17, 0.34), 0.012 if scale == "50" else 0.0, **bt),
              cap("drive-motor", (0.0, A, -0.205), (0.0, A, 0.205), 0.065, **mo),
              box("deck", (0.53, 0.13, 0), (0.66, 0.05, 0.32), 0.01 if scale == "50" else 0.0, **sh) if scale != "500" else box("deck", (0.53, 0.1175, 0), (0.66, 0.075, 0.32), **sh),
              box("front-nose", (0.94, 0.24, 0), (0.28, 0.28, 0.34), 0.03 if scale in ("50", "100") else 0.0, **sh),
              cap("front-axle", (WHEELBASE, A, -0.205), (WHEELBASE, A, 0.205), 0.02 if scale != "500" else 0.04, **dark),
              cap("bumper", (1.07, 0.16, -0.19), (1.07, 0.16, 0.19), 0.025 if scale != "500" else 0.0375, **part("shell", 0.25, material="pu-tyre"))]
    if scale in ("50", "100"):
        parts.append(box("heel-rest", (0.43, 0.185, 0), (0.09, 0.07, 0.30), 0.01, **sh))
    if scale in ("50", "100", "200"):
        parts += [box("front-fender", (0.92, 0.33, 0), (0.30, 0.06, 0.57), 0.02 if scale != "200" else 0.0, **sh),
                  cap("seat-post", (0.10, 0.33, 0), (0.10, 0.47, 0), 0.03, **dark),
                  box("seat-base", (0.13, 0.47, 0), (0.46, 0.04, 0.44), **dark)]
    else:
        parts += [box("front-fender", (0.92, 0.3375, 0), (0.30, 0.075, 0.57), **sh), box("seat-base", (0.13, 0.43, 0), (0.46, 0.16, 0.44), **dark)]
    cushion_y = (0.486, 0.557) if scale != "500" else (0.48, 0.557)
    parts += [box("seat-cushion", (0.11, sum(cushion_y) / 2, 0), (0.52, cushion_y[1] - cushion_y[0], 0.48), 0.02 if scale in ("50", "100") else 0.0, **up),
              box("backrest", centre, (t_back, 0.56, 0.46 if scale != "500" else 0.36), 0.025 if scale in ("50", "100") else 0.0, rot=BACK_DEG, **up)]
    if scale != "500":
        rw = 0.06 if scale != "200" else 0.06
        for side, sg in (("l", 1), ("r", -1)):
            parts += [box(f"armrest-{side}", (0.10, 0.668, sg * 0.268), (0.36, 0.05, rw), 0.012 if scale in ("50", "100") else 0.0, **up),
                      cap(f"armrest-post-{side}", (0.02, 0.47, sg * 0.212), (0.06, 0.655, sg * 0.262), 0.012 if scale == "50" else 0.016, **dark)]
    top, base = (0.62, 0.875), (0.90, 0.30)
    if scale != "500":
        parts += [cap("tiller", base, top, 0.025, **dark),
                  cap("tiller-cover", (0.90, 0.30, 0), (0.85, 0.47, 0), 0.045, **sh),
                  cap("handlebar", (0.60, 0.865, -0.235), (0.60, 0.865, 0.235), 0.014 if scale == "50" else 0.016, **dark),
                  box("console", (0.645, 0.9, 0), (0.11, 0.06, 0.24), 0.01 if scale == "50" else 0.0, rot=-30, **part("shell", 0.3)),
                  box("basket", (0.84, 0.62, 0), (0.24, 0.26, 0.36), 0.01 if scale == "50" else 0.0, **part("shell", 0.62, material="steel-wire"))]
        if scale == "50":
            for side, sg in (("l", 1), ("r", -1)):
                parts.append(cap(f"grip-{side}", (0.60, 0.865, sg * 0.16), (0.60, 0.865, sg * 0.235), 0.019, **part("upholstery", 0.12, material="rubber-grip")))
            parts.append({"id": "headlight", "kind": "ellipsoid", "bone": "chassis", "radii": [0.02, 0.03, 0.045], "transform": {"translation": [1.075, 0.33 - A, 0]}, **part("shell", 0.9, material="lamp-lens")})
    else:
        parts += [cap("tiller", (0.90, 0.30, 0), (0.62, 0.875, 0), 0.0375, **dark),
                  box("handlebar", (0.62, 0.875, 0), (0.10, 0.08, 0.48), **dark),
                  box("basket", (0.84, 0.62, 0), (0.24, 0.26, 0.36), **part("shell", 0.62, material="steel-wire"))]
    return parts


def all_parts(scale, back_d):
    return chassis(scale, back_d) + wheels(scale)


def seat_load(scale):
    """The structural seat contact: a pad from the cushion up to the ischial tuberosities that carries the rider into the chassis."""
    return box("st-seat-contact", (0.04, 0.59, 0), (0.18, 0.08, 0.2), part="member", member="chassis", material="vinyl", role="body", tone=0.3)
