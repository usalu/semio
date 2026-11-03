from lib import *
import zlib

P = {
    "50": dict(n=28, k=9, amp=0.065, imp=3, arc="texture"),
    "100": dict(n=20, k=7, amp=0.07, imp=2, arc="detail"),
    "200": dict(n=14, k=5, amp=0.06, imp=1, arc=None),
    "500": dict(n=10, k=3, amp=0.05, imp=0, arc=None),
}


def seed(*a):
    return zlib.crc32("|".join(str(x) for x in a).encode()) / 4294967296.0


def lobe(name, bf, bp, x, y, z, rh, rv, like, imp=1, rhy=None, tone=None, hatch=None):
    return dict(name=name, bf=bf, bp=bp, x=x, y=y, z=z, rh=rh, rv=rv, like=like, imp=imp, rhy=rhy if rhy is not None else rh, tone=tone)


def ring(name, z, R, rh, rv, azs, bw, be, likefmt, imp=1, jit=0.0, rot=0.0, rhy=None, tone=None, imps=None):
    out = []
    for i, az in enumerate(azs):
        a = math.radians(az + rot)
        j = 1 + jit * (seed(name, az) - 0.5) * 2
        c = math.cos(a)
        east = c > 0.05 or (abs(c) <= 0.05 and i % 2 == 0)
        bone = be if east else bw
        out.append(lobe("%s-%d" % (name, int(round(az))), bone, bone, R * j * math.cos(a), R * j * math.sin(a), z + 0.04 * (seed(name, "z", az) - 0.5) * rv, rh * (0.92 + 0.16 * seed(name, "r", az)), rv, likefmt.format(bone), imps[i] if imps else imp, rhy=rhy, tone=tone))
    return out


def arc_pts(cx, cy, rx, ry, a0, a1, n=5):
    return [(cx + rx * math.cos(math.radians(a0 + (a1 - a0) * i / (n - 1))), cy + ry * math.sin(math.radians(a0 + (a1 - a0) * i / (n - 1)))) for i in range(n)]


def emit_lobes(b, sc, lobes, base_front=3.0, base_plan=3.0, arcs=True, shade_ids=(), front_prefix="f-", plan_prefix="p-"):
    p = P[sc]
    use = [L for L in lobes if L["imp"] <= p["imp"]]
    if not use:
        return
    xs = [L["x"] for L in use]
    zs = [L["z"] for L in use]
    xmin, xmax = min(xs), max(xs)
    zmin, zmax = min(zs), max(zs)
    for i, L in enumerate(use):
        s1 = seed(L["name"], sc, "f")
        s2 = seed(L["name"], sc, "p")
        zf = base_front + 2.0 * (L["x"] - xmin) / max(xmax - xmin, 1e-6) + 0.001 * i
        zp = base_plan + 2.0 * (L["z"] - zmin) / max(zmax - zmin, 1e-6) + 0.001 * i
        k = p["k"] + int(s1 * 3) - 1
        d = blob(L["y"], L["z"], L["rhy"], L["rv"], n=p["n"], k=k, amp=p["amp"], ph=s1 * 6.28, k2=k + 3, amp2=p["amp"] * 0.4, ph2=s2 * 6.28)
        b.add(sc, "front", front_prefix + L["name"], L["bf"], zf, d=d, like=L["like"], tone=L["tone"])
        k = p["k"] + int(s2 * 3) - 1
        d = blob(L["x"], L["y"], L["rh"], L["rhy"], n=p["n"], k=k, amp=p["amp"], ph=s2 * 6.28, k2=k + 3, amp2=p["amp"] * 0.4, ph2=s1 * 6.28)
        b.add(sc, "plan", plan_prefix + L["name"], L["bp"], zp, d=d, like=L["like"], tone=L["tone"])
        if arcs and p["arc"] and L["imp"] <= 1:
            a0 = 150 + 40 * s1
            pts = arc_pts(L["y"], L["z"] + 0.1 * L["rv"], 0.55 * L["rhy"], 0.5 * L["rv"], a0, a0 - 110)
            b.add(sc, "front", front_prefix + L["name"] + "-arc", L["bf"], zf + 0.0005, d=open_path(pts), role=p["arc"], tone=0.2, filled=False)
            a0 = 140 + 60 * s2
            pts = arc_pts(L["x"], L["y"], 0.55 * L["rh"], 0.55 * L["rhy"], a0, a0 - 120)
            b.add(sc, "plan", plan_prefix + L["name"] + "-arc", L["bp"], zp + 0.0005, d=open_path(pts), role=p["arc"], tone=0.2, filled=False)


def taper(b, sc, view, sid, bone, z, like, pts, role="body", tone=None, follow=True, **kw):
    geo = dict(points=[[round(x, 4), round(y, 4)] for x, y in pts], closed=True)
    if follow:
        geo["followTip"] = True
    return b.add(sc, view, sid, bone, z, like=like, role=role, tone=tone, kind="poly", **geo, **kw)


def stroke(b, sc, view, sid, bone, z, pts, role="texture", tone=0.2, like=None, region=None):
    return b.add(sc, view, sid, bone, z, d=open_path(pts), role=role, tone=tone, filled=False, like=like, region=region)


def disc(b, sc, view, sid, bone, z, cx, cy, rx, ry, like, tone=None):
    return b.add(sc, view, sid, bone, z, like=like, tone=tone, kind="ellipse", cx=round(cx, 4), cy=round(cy, 4), rx=round(rx, 4), ry=round(ry, 4))


def smooth(pts):
    return closed_path(pts)


def fit(pts, x0, x1, y0, y1):
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts]
    ax, ay = min(xs), min(ys)
    sx = (x1 - x0) / max(max(xs) - ax, 1e-9)
    sy = (y1 - y0) / max(max(ys) - ay, 1e-9)
    return [(x0 + (x - ax) * sx, y0 + (y - ay) * sy) for x, y in pts]


def limbs(b, sc, lobes, region, bone_of=None, front_att=None, lim=1, z0=1.0, width=None):
    p = P[sc]
    if sc not in ("50", "100"):
        return
    for L in lobes:
        if L["imp"] > lim:
            continue
        att = L.get("att", L["z"] - 0.35)
        s = seed(L["name"], sc, "limb")
        mid = (0.5 * L["y"] + 0.05 * (s - 0.5), 0.5 * (att + L["z"]) + 0.08)
        stroke(b, sc, "front", "fl-" + L["name"], L["bf"], z0 + 0.01 * s, [(0.0, att), mid, (0.9 * L["y"], L["z"] - 0.05 * L["rv"])], role=p["arc"], tone=0.2, region=region)
        bow = 0.12 * (s - 0.5)
        stroke(b, sc, "plan", "pl-" + L["name"], L["bp"], 7.0 + 0.01 * s, [(0.0, 0.0), (0.5 * L["x"] - bow * L["y"], 0.5 * L["y"] + bow * L["x"]), (0.9 * L["x"], 0.9 * L["y"])], role=p["arc"], tone=0.2, region=region)
