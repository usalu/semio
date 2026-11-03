from crowns import *
from decs import std_dec

AZ = {"L1": [180, 90, 270, 135, 225], "R1": [0, 260, 100, 45, 315], "L2": [140, 220, 55, 300, 5], "R2": [320, 40, 125, 200, 255], "L3": [250, 110, 190, 20, 340], "R3": [70, 290, 350, 160, 235]}
COPIES = {"50": 5, "100": 3, "200": 2}
CULM = {"1": (30, "culm-1a", "culm-1b", "plume-1"), "2": (100, "culm-2a", "culm-2b", "plume-2"), "3": (250, "culm-3a", "culm-3b", "plume-3"), "4": (330, "culm-4a", "culm-4b", "plume-4")}


def tip(b, bone):
    x, y, ang = b.w[bone]
    ln = next(q for q in b.asset["rig"]["bones"] if q["id"] == bone)["lengthM"]
    return x + ln * math.cos(math.radians(ang)), y + ln * math.sin(math.radians(ang))


def kf(az):
    return 1.0 - 0.13 * math.sin(math.radians(az)) ** 2


def ribbon(p0, p1, w0, w1):
    dx, dy = p1[0] - p0[0], p1[1] - p0[1]
    ln = math.hypot(dx, dy) or 1.0
    nx, ny = -dy / ln, dx / ln
    return [(p0[0] - nx * w0, p0[1] - ny * w0), (p0[0] + nx * w0, p0[1] + ny * w0), (p1[0] + nx * w1, p1[1] + ny * w1), (p1[0] - nx * w1, p1[1] - ny * w1)]


def build(b):
    for sc in ("50", "100", "200"):
        wid = {"50": (0.03, 0.015), "100": (0.035, 0.02), "200": (0.07, 0.04)}[sc]
        taper(b, sc, "front", "tuft", "root-plate", 2.0, "tuft", [(-0.22, 0.0), (0.22, 0.0), (0.12, 0.2), (-0.12, 0.2)])
        disc(b, sc, "plan", "tuft", "root-plate", 2.0, 0.0, 0.0, 0.22, 0.22, "tuft")
        for key, azs in AZ.items():
            ba, bb = "bl-%sa" % key, "bl-%sb" % key
            (ax, ay), (bx, by) = tip(b, ba), tip(b, bb)
            ra, rb = abs(ax), abs(bx)
            ox, oy = b.w[ba][0], b.w[ba][1]
            for k, az in enumerate(azs[: COPIES[sc]]):
                c, s = math.cos(math.radians(az)), math.sin(math.radians(az))
                f = kf(az)
                pa = (ra * f * c * (1.06 if abs(c) > 0.9 else 1.0), ra * f * s)
                pb = (rb * f * c * (1.06 if abs(c) > 0.9 else 1.0), rb * f * s)
                la, lb = ("blade-%s-%da" % (key, min(k + 1, 4)), "blade-%s-%db" % (key, min(k + 1, 4))) if sc == "50" else ("bl-%sa-0" % key, "bl-%sb-0" % key)
                zz = 3.0 + 0.1 * k + 0.01 * "L1R1L2R2L3R3".index(key)
                b.add(sc, "plan", "%s-%d-a" % (key, k + 1), ba, zz, like=la, kind="capsule", x0=0.0, y0=0.0, x1=round(pa[0], 4), y1=round(pa[1], 4), r0=wid[0], r1=wid[1], followTip=True)
                b.add(sc, "plan", "%s-%d-b" % (key, k + 1), bb, zz + 0.001, like=lb, kind="capsule", x0=round(pa[0], 4), y0=round(pa[1], 4), x1=round(pb[0], 4), y1=round(pb[1], 4), r0=wid[1], r1=wid[1] * 0.7, followTip=True)
                depth = 3.0 + 0.4 * (1 + c) + 0.01 * k
                p0, p1, p2 = (0.0, 0.12), (pa[1], ay), (pb[1], by)
                taper(b, sc, "front", "%s-%d-a" % (key, k + 1), ba, depth, la, ribbon(p0, p1, wid[0], wid[1]))
                taper(b, sc, "front", "%s-%d-b" % (key, k + 1), bb, depth + 0.001, lb, ribbon(p1, p2, wid[1], wid[1] * 0.7))
        for n, (az, ca, cb, pl) in CULM.items():
            c, s = math.cos(math.radians(az)), math.sin(math.radians(az))
            (ax, ay), (bx, by) = tip(b, ca), tip(b, cb)
            pa, pb = (abs(ax) * c, abs(ax) * s), (abs(bx) * c, abs(bx) * s)
            ln = 0.4 if n in "23" else 0.19
            like_c = ("culm-%sa" % n, "culm-%sb" % n) if sc == "50" else ("culm-%sa-axis" % n, "culm-%sb-axis" % n)
            if sc != "200":
                taper(b, sc, "front", "culm-%sa" % n, ca, 5.0, like_c[0], ribbon((0.0, 0.12), (pa[1], ay), 0.014, 0.01))
                taper(b, sc, "front", "culm-%sb" % n, cb, 5.0, like_c[1], ribbon((pa[1], ay), (pb[1], by), 0.01, 0.008))
                b.add(sc, "plan", "culm-%sa" % n, ca, 5.0, like=like_c[0], kind="capsule", x0=0.0, y0=0.0, x1=round(pa[0], 4), y1=round(pa[1], 4), r0=0.016, r1=0.012, followTip=True)
                b.add(sc, "plan", "culm-%sb" % n, cb, 5.0, like=like_c[1], kind="capsule", x0=round(pa[0], 4), y0=round(pa[1], 4), x1=round(pb[0], 4), y1=round(pb[1], 4), r0=0.012, r1=0.01, followTip=True)
            dirx, diry = math.cos(math.radians(b.w[pl][2])), math.sin(math.radians(b.w[pl][2]))
            rad = abs(bx)
            lat = rad * s
            ang = math.degrees(math.atan2(diry, abs(dirx) * 0.0 + dirx * 0.0 + (dirx * s if False else dirx)))
            hx = b.w[pl][0]
            cxm = lat + 0.5 * ln * dirx * s
            cym = b.w[pl][1] + 0.5 * ln * diry
            rotf = math.degrees(math.atan2(diry, dirx * s if abs(dirx * s) > 0.02 else 0.02 * (1 if dirx * s >= 0 else -1)))
            b.add(sc, "front", "plume-" + n, pl, 6.0, like="plume-" + n, kind="ellipse", cx=round(cxm, 4), cy=round(cym, 4), rx=round(0.5 * ln, 4), ry=0.05, rotationDeg=round(rotf, 2))
            px = abs(b.w[pl][0]) * c + 0.5 * ln * dirx * 0.0
            b.add(sc, "plan", "plume-" + n, pl, 6.0, like="plume-" + n, kind="ellipse", cx=round(abs(b.w[pl][0]) * c + 0.5 * ln * abs(dirx) * c, 4), cy=round(abs(b.w[pl][0]) * s + 0.5 * ln * abs(dirx) * s, 4), rx=0.07 if ln < 0.3 else 0.06, ry=0.06)
    fr = fit([(-0.9, 0.05), (-0.95, 0.35), (-0.7, 0.7), (-0.5, 1.0), (-0.3, 1.3), (-0.25, 1.75), (-0.1, 1.5), (0.0, 1.78), (0.15, 1.5), (0.28, 1.75), (0.3, 1.3), (0.55, 0.95), (0.75, 0.65), (0.97, 0.35), (0.9, 0.05), (0.45, 0.2), (0.0, 0.0), (-0.45, 0.2)], -0.9, 0.9, 0.0, 1.75)
    b.add("500", "front", "tuft", "root-plate", 1.0, d=smooth(fr), like="tuft")
    pl = fit(outline(0.0, 0.0, 1.0, 0.85, n=16, k=9, amp=0.1, ph=0.4), -1.07, 1.07, -0.9, 0.9)
    b.add("500", "plan", "tuft", "root-plate", 1.0, d=smooth(pl), like="tuft")


def decisions():
    return std_dec(dict(
        front=("the fountain of arching blades seen from the side, the blade bundles around the clump ordered by depth and the four plumes above", "die Fontäne aus überhängenden Blättern von der Seite, die Blattbündel rund um den Horst nach der Tiefe geordnet und die vier Federn darüber"),
        trunk=("four culms, each in two segments", "vier Halmen, jeder in zwei Abschnitten"),
        plan=("the clump from above as a star of arching blade bundles with the four plume tops near the middle", "der Horst von oben als Stern aus überhängenden Blattbündeln mit den vier Federspitzen nahe der Mitte"),
        base=("the culm lines and the clump disc", "die Halmlinien und die Horstscheibe"),
        front100=("two blade copies per bundle, culms and plumes", "zwei Blattkopien je Bündel, Halmen und Federn"),
        plan100=("the star of blade bundles with culms and plumes", "dem Stern aus Blattbündeln mit Halmen und Federn"),
        main200=("one broad ribbon per blade bundle and the four plumes", "einem breiten Band je Blattbündel und den vier Federn"),
        trunk500=("the base of the clump", "dem Horstfuß")))
