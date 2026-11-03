import math
from PIL import Image, ImageDraw
from lib import *


def capsule_poly(s):
    x0, y0, x1, y1, r0, r1 = s["x0"], s["y0"], s["x1"], s["y1"], s["r0"], s["r1"]
    pts = []
    ang = math.atan2(y1 - y0, x1 - x0)
    for i in range(13):
        t = ang + math.pi / 2 + math.pi * i / 12
        pts.append((x1 + r1 * math.cos(t - math.pi), y1 + r1 * math.sin(t - math.pi)))
    for i in range(13):
        t = ang - math.pi / 2 + math.pi * i / 12
        pts.append((x0 + r0 * math.cos(t - math.pi), y0 + r0 * math.sin(t - math.pi)))
    # simple hull-like ring: end caps
    pts = []
    for i in range(13):
        t = ang - math.pi / 2 + math.pi * i / 12
        pts.append((x1 + r1 * math.cos(t), y1 + r1 * math.sin(t)))
    for i in range(13):
        t = ang + math.pi / 2 + math.pi * i / 12
        pts.append((x0 + r0 * math.cos(t), y0 + r0 * math.sin(t)))
    return pts


def draw_shapes(dr, shapes, ox, oy, ppm, tf=None):
    for s in sorted(shapes, key=lambda s: s["z"]):
        if s["kind"] == "capsule":
            pts = capsule_poly(s)
        elif s["kind"] == "path":
            pts = sample_path(s["d"], 6)
        else:
            pts = shape_pts(s)
        if tf is not None:
            pts = [tf(s, p) for p in pts]
        px = [(ox + x * ppm, oy - y * ppm) for x, y in pts]
        tone = s.get("tone", 0.5)
        g = int(70 + 170 * tone)
        if s["filled"] and len(px) > 2:
            dr.polygon(px, fill=(g, g, g), outline=(0, 0, 0))
        else:
            dr.line(px, fill=(60, 60, 60), width=1)


def elevation_shapes(asset, sc, w):
    out = []
    for s in asset["representations"][sc]["geometry"]["elevation"]:
        ox, oy, ang = w[s["bone"]]
        c, sn = math.cos(math.radians(ang)), math.sin(math.radians(ang))
        pts = [(ox + px * c - py * sn, oy + px * sn + py * c) for px, py in shape_pts(s, 6)]
        t = dict(s)
        t["kind"] = "poly"
        t["points"] = pts
        t["closed"] = True
        out.append(t)
    return out


def render(asset, views, sc, out, ppm=None, ext=None):
    w = rest_world(asset)
    dm = asset["dimensions"]
    H, W, D = dm["heightM"], dm["widthM"], dm["depthM"]
    pad = 0.6
    if ppm is None:
        ppm = 620 / max(H + 2 * pad, 4)
        ppm = min(ppm, 900 / (W + D + D + 6 * pad + 0.1))
    elev = elevation_shapes(asset, sc, w)
    pw = int((W + 2 * pad) * ppm)
    dw = int((D + 2 * pad) * ppm)
    img = Image.new("RGB", (pw + 2 * dw + 40, int((H + 2 * pad) * ppm) + 20), (255, 255, 255))
    dr = ImageDraw.Draw(img)
    gy = int((H + pad) * ppm) + 10
    # elevation
    draw_shapes(dr, elev, pw / 2, gy, ppm)
    dr.line([(0, gy), (img.width, gy)], fill=(200, 0, 0), width=1)
    # front
    draw_shapes(dr, views["front"], pw + 20 + dw / 2, gy, ppm)
    # plan: x forward (to the right), y lateral up; centred vertically around the ground line mid
    pl_h = int((D + 2 * pad) * ppm)
    cy = gy - (H * ppm) / 2 if False else gy
    draw_shapes(dr, views["plan"], pw + dw + 40 + dw / 2 - 0, 0, ppm) if False else None
    # plan drawn on its own band above: shift so plan fits: render separately
    plan_img = Image.new("RGB", (pw, pl_h), (255, 255, 255))
    pd = ImageDraw.Draw(plan_img)
    draw_shapes(pd, views["plan"], pw / 2, pl_h / 2, ppm)
    pd.line([(0, pl_h / 2), (pw, pl_h / 2)], fill=(200, 200, 255), width=1)
    pd.line([(pw / 2, 0), (pw / 2, pl_h)], fill=(200, 200, 255), width=1)
    # combine: top row elevation|front, bottom row plan
    canvas = Image.new("RGB", (img.width, img.height + pl_h + 10), (255, 255, 255))
    canvas.paste(img, (0, 0))
    canvas.paste(plan_img, (0, img.height + 10))
    canvas.save(out)
