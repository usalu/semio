"""Shared body conversion: side-outline extrudes of the human bones become lofts."""
from lib import *

HUMAN = ("pelvis", "torso", "neck", "head", "arm", "upper-arm", "forearm", "hand", "thigh", "shank", "foot", "toe")
EXP = {"head": 2.0, "hair": 2.0, "hat": 2.0, "helmet": 2.0, "torso": 2.5, "hip": 2.5, "neck": 2.0, "collar": 2.2, "backpack": 3.2}


def is_human_bone(b):
    return re.sub(r"-(l|r)$", "", b) in HUMAN


def view_only(A):
    d = json.load(open(f"{T}/🗑️generated/drafts/humans/{A.slug}.json"))
    return {x["solid"] for x in d["report"]["provenance"] if x["route"].startswith("view-only")}


def convertible(s):
    if s["kind"] != "extrude" or s["plane"] != "xy" or "holes" in s or "transform" in s or "repeat" in s:
        return False
    if s["role"] == "body":
        return True
    ring = s["profile"]
    us = [p[0] for p in ring]
    return (max(us) - min(us)) > 0.012 and (s["to"] - s["from"]) > 0.03


def humanize(A, skip=(), use=(), aspects=None, exps=None, lats=None, ramps=None, loft_kw=None, bones=None, front=None):
    aspects, exps, lats, ramps, loft_kw, front = aspects or {}, exps or {}, lats or {}, ramps or {}, loft_kw or {}, front or {}
    vo = view_only(A)
    for sid, s in A.draft.items():
        if sid in skip or (sid in vo and sid not in use):
            continue
        if bones is None and not is_human_bone(s["bone"]):
            continue
        if bones is not None and s["bone"] not in bones:
            continue
        if convertible(s):
            lo = loft_from(s, lat=lats.get(sid), aspect=aspects.get(sid), exp=exps.get(sid, EXP.get(stem(sid), 2.2)), ramp=ramps.get(sid, "auto"), **({"rxfn": front_halfwidth(A, s["bone"], front[sid][0], front[sid][1] if len(front[sid]) > 1 else "front")} if sid in front else {}), **loft_kw.get(sid, {}))
            A.add(lo, like=sid)
        else:
            A.keep(sid)


def get(A, sid):
    return next(s for s in A.out if s["id"] == sid)


def eyes(A, head_id="head", u=None, w=0.03, rx=0.008, ry=0.006, rz=0.007, material=None, tone=0.2, like="eye", region="face-skin"):
    h = get(A, head_id)
    for side, sgn in (("l", 1), ("r", -1)):
        v = surface_v(h, u, sgn * w) + ry * 0.4
        s = ellipsoid(f"eye-{side}", h["bone"], [rx, ry, rz], [u, v, sgn * w], material=material or h["material"], role="detail", tone=tone, region=region)
        A.add(s, like=like, maxden=50)


def deltoids(A, arm_ids, scale=1.0):
    for aid in arm_ids:
        a = get(A, aid)
        s0 = a["sections"][0]
        s1 = a["sections"][1]
        r = max(s0["ry"], s1["ry"]) * 1.04 * scale
        side = "l" if aid.endswith("l") or a["bone"].endswith("-l") else "r"
        d = ellipsoid(f"shoulder-{side}", a["bone"], [r, r, r], [s1["at"][0] * 0.55, s1["at"][1], s1["at"][2]], material=a["material"], role=a["role"], tone=a["tone"], region=a.get("region"))
        A.add({k: v for k, v in d.items() if v is not None}, like=aid)


def report(A):
    for s in A.draft.values():
        if s["kind"] == "extrude":
            ring = s["profile"]
            us = [p[0] for p in ring]; vs = [p[1] for p in ring]
            print(f"{s['id']:18} {s['bone']:12} {s['plane']} a[{min(us):.3f},{max(us):.3f}] b[{min(vs):.3f},{max(vs):.3f}] c[{s['from']:.3f},{s['to']:.3f}] {s['role']}")


def thumb(A, draft_id, hand_bone, r=None, glove=None):
    s = A.draft[draft_id]
    ring = s["profile"]
    us = [p[0] for p in ring]
    vs = [p[1] for p in ring]
    L = max(us) - min(us)
    vm = (min(vs) + max(vs)) / 2
    rr = r or min(0.02, (max(vs) - min(vs)) / 2)
    wc = (s["from"] + s["to"]) / 2
    if glove:
        g = get(A, glove)
        wc = g["sections"][len(g["sections"]) // 2]["at"][2]
    A.add(capsule(draft_id, hand_bone, [min(us) + 0.15 * L, vm, wc], [max(us) - 0.1 * L, vm, wc], rr, rr * 0.8, material=s["material"], role=s["role"], tone=s["tone"], region=s.get("region")), like=draft_id)


def box_of(A, sid, corner=0.004, **over):
    s = A.draft[sid]
    ring = s["profile"]
    us = [p[0] for p in ring]
    vs = [p[1] for p in ring]
    m = meta(s, **over)
    m.pop("bone")
    b = box(sid, s["bone"], [max(us) - min(us), max(vs) - min(vs), s["to"] - s["from"]], [(max(us) + min(us)) / 2, (max(vs) + min(vs)) / 2, (s["from"] + s["to"]) / 2], corner=corner, **m)
    A.add(b, like=sid)
    return b


def patch(A, id, bone, u, v, w, size, like, material, tone, region, role="detail", corner=0.003):
    A.add(box(id, bone, size, [u, v, w], corner=corner, material=material, role=role, tone=tone, region=region), like=like, maxden=50)


def ring_wheel(A, id, bone, R, r, ls, n, material, tone, region, role="body", like=None, maxden="auto", z=0.0):
    """A torus-like ring (tyre, rim, chainring teeth) as n chord capsules about the w axis through the bone origin."""
    th = 2 * math.pi / n
    s = capsule(id, bone, [-R * math.sin(th / 2), R * math.cos(th / 2), z], [R * math.sin(th / 2), R * math.cos(th / 2), z], r, ls=ls, material=material, role=role, tone=tone, region=region)
    s["repeat"] = {"count": n, "rotationAxis": [0, 0, 1], "rotationRad": th, "origin": [0, 0, z]}
    A.add(s, like=like or id, maxden=maxden)


def spokes(A, id, bone, r0, r1, n, rad, material, tone, region, z0=0.0, z1=0.0, like=None):
    th = 2 * math.pi / n
    s = capsule(id, bone, [r0, 0.0, z0], [r1, 0.0, z1], rad, material=material, role="detail", tone=tone, region=region)
    s["repeat"] = {"count": n, "rotationAxis": [0, 0, 1], "rotationRad": th, "origin": [0, 0, 0]}
    A.add(s, like=like or id, maxden=50)


def interp(solid, u):
    secs = solid["sections"]
    for a, b in zip(secs, secs[1:]):
        if a["at"][0] <= u <= b["at"][0]:
            t = (u - a["at"][0]) / (b["at"][0] - a["at"][0])
            break
    else:
        a = b = secs[0] if u < secs[0]["at"][0] else secs[-1]
        t = 0
    L = lambda x, y: x + (y - x) * t
    return {"v": L(a["at"][1], b["at"][1]), "w": L(a["at"][2], b["at"][2]), "rx": L(a["rx"], b["rx"]), "ry": L(a["ry"], b["ry"]), "e": a.get("exponent", 2)}


def band(A, id, base_id, u0, u1, grow, material, tone, region, role="detail", like=None, maxden=50):
    """A flat-ended ring slightly larger than a loft (reflective band, trouser clip, cuff)."""
    base = get(A, base_id)
    secs = []
    for u in (u0, (u0 + u1) / 2, u1):
        i = interp(base, u)
        secs.append({"at": [u, i["v"], i["w"]], "rx": i["rx"] + grow, "ry": i["ry"] + grow, "exponent": i["e"], "rotation": ROT["u"]})
    A.add({"id": id, "kind": "loft", "bone": base["bone"], "material": material, "role": role, "tone": tone, "region": region, "sections": secs, "capStart": True, "capEnd": True}, like=like or id, maxden=maxden)


def refinger(A, ids, glove_id):
    g = get(A, glove_id)
    wc = g["sections"][len(g["sections"]) // 2]["at"][2]
    for sid in ids:
        s = json.loads(json.dumps(A.draft[sid]))
        s["p0"][2] = wc
        s["p1"][2] = wc
        A.add(s, like=sid, maxden=50)


def tube_path(A, id, bone, pts, r, material, tone, region, role="body", like=None, w=0.0, maxden="auto", r1=None):
    """A polyline tube: one capsule per segment (pts are (u, v) pairs at lateral w)."""
    for i, (p, q) in enumerate(zip(pts, pts[1:])):
        A.add(capsule(f"{id}-{i + 1}" if len(pts) > 2 else id, bone, [p[0], p[1], w], [q[0], q[1], w], r, material=material, role=role, tone=tone, region=region), like=like or id, maxden=maxden)


def pfn(points):
    """Piecewise linear function through (u, value) points."""
    pts = sorted(points)

    def f(u):
        if u <= pts[0][0]:
            return pts[0][1]
        for (a, x), (c, y) in zip(pts, pts[1:]):
            if u <= c:
                return x + (y - x) * (u - a) / (c - a)
        return pts[-1][1]

    return f


def mirror(A, sid, new_id, new_bone, like=None, maxden="auto"):
    """The solid with the lateral axis flipped, bound to the opposite bone (left to right limb)."""
    s = json.loads(json.dumps(get(A, sid)))
    s["id"], s["bone"] = new_id, new_bone
    s.pop("maxDenominator", None)
    if s["kind"] == "loft":
        for q in s["sections"]:
            q["at"][2] = -q["at"][2]
    elif s["kind"] == "capsule":
        s["p0"][2] = -s["p0"][2]
        s["p1"][2] = -s["p1"][2]
    elif s["kind"] in ("ellipsoid", "box"):
        s["transform"]["translation"][2] = -s["transform"]["translation"][2]
    else:
        raise ValueError(s["kind"])
    A.add(s, like=like or sid, maxden=maxden)
    return s


def shift_copy(A, sid, new_id, new_bone, dw, like=None, maxden="auto"):
    """A copy of a loft with its lateral centre moved by dw, bound to another bone (far limb of the same outline)."""
    s = json.loads(json.dumps(get(A, sid)))
    s["id"], s["bone"] = new_id, new_bone
    s.pop("maxDenominator", None)
    for q in s["sections"]:
        q["at"][2] += dw
    A.add(s, like=like or sid, maxden=maxden)
    return s


def shift_w(s, dw):
    """Move a solid along the bone-local w axis by dw (for bones whose origin sits off the body axis)."""
    k = s["kind"]
    if k == "loft":
        for q in s["sections"]:
            q["at"][2] += dw
    elif k == "capsule":
        s["p0"][2] += dw
        s["p1"][2] += dw
    elif k == "extrude":
        if s["plane"] == "xy":
            s["from"] += dw
            s["to"] += dw
        else:
            raise ValueError("extrude plane")
    else:
        t = s.setdefault("transform", {"translation": [0, 0, 0]})
        t.setdefault("translation", [0, 0, 0])
        t["translation"][2] += dw
    return s


def finger_caps(A, ids, glove_id, maxden=50):
    """Thin extruded finger outlines become round capsules along u on the glove centre line."""
    g = get(A, glove_id)
    wc = g["sections"][len(g["sections"]) // 2]["at"][2]
    for sid in ids:
        s = A.draft[sid]
        us = [p[0] for p in s["profile"]]
        vs = [p[1] for p in s["profile"]]
        r = max(0.004, (max(vs) - min(vs)) / 2)
        vm = (max(vs) + min(vs)) / 2
        A.add(capsule(sid, s["bone"], [min(us) + r, vm, wc], [max(us) - r, vm, wc], r, material=s["material"], role=s["role"], tone=s["tone"], region=s.get("region")), like=sid, maxden=maxden)


def clean_polyline(pts, tol=5e-4):
    """Drop retraced spikes and near-duplicate points of an open polyline."""
    pts = [list(p) for p in pts]
    changed = True
    while changed:
        changed = False
        for i in range(1, len(pts) - 1):
            a, b, c = pts[i - 1], pts[i], pts[i + 1]
            v1, v2 = (b[0] - a[0], b[1] - a[1]), (c[0] - b[0], c[1] - b[1])
            n1, n2 = math.hypot(*v1), math.hypot(*v2)
            if n1 < tol or n2 < tol or (v1[0] * v2[0] + v1[1] * v2[1]) / (n1 * n2) < -0.95:
                del pts[i]
                changed = True
                break
    return pts


def segs_cross(p, q, r, s):
    def o(a, b, c):
        return (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
    return o(p, q, r) * o(p, q, s) < 0 and o(r, s, p) * o(r, s, q) < 0


def simple(pts):
    n = len(pts) - 1
    return all(not segs_cross(pts[i], pts[i + 1], pts[j], pts[j + 1]) for i in range(n) for j in range(i + 2, n))


def dp(pts, tol):
    if len(pts) < 3:
        return pts
    a, b = pts[0], pts[-1]
    L = math.hypot(b[0] - a[0], b[1] - a[1]) or 1e-9
    dmax, k = 0, 0
    for i in range(1, len(pts) - 1):
        d = abs((b[0] - a[0]) * (pts[i][1] - a[1]) - (b[1] - a[1]) * (pts[i][0] - a[0])) / L
        if d > dmax:
            dmax, k = d, i
    if dmax <= tol:
        return [a, b]
    return dp(pts[: k + 1], tol)[:-1] + dp(pts[k:], tol)
