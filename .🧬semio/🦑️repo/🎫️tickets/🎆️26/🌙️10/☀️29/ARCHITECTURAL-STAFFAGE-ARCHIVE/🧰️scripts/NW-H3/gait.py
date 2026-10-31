"""Procedural gait frames with exact foot planting (arithmetic only): stance foot pivots about a fixed heel then flat then about a fixed ball,
swing foot follows a smooth clearance arc, pelvis height follows a bob limited by leg reach. Used for the courier walk, the pregnant waddle and the runner."""
from h3lib import *


def geom_of(a):
    B = {b["id"]: b for b in a["rig"]["bones"]}
    l1, l2 = B["thigh-l"]["lengthM"], B["shank-l"]["lengthM"]
    Lf = B["foot-l"]["lengthM"]
    alpha = math.radians(90 - B["foot-l"]["angleDeg"])
    an = {x["id"]: x for x in a["anchors"]}
    H = (an["heel-l"]["x"], an["heel-l"]["y"])
    Bl = (an["ball-l"]["x"], an["ball-l"]["y"])
    return dict(l1=l1, l2=l2, alpha=alpha, H=H, Bl=Bl, hip_y=B["pelvis"]["y"], ankle=Lf * math.sin(alpha))


def ankle_from(point, theta, local):
    r = rot_pt(theta, local)
    return (point[0] - r[0], point[1] - r[1])


def leg_state(G, u, S, X):
    """ankle world position and foot delta (rad) of one leg at phase u (heel strike at u=0 for walking, ball landing for running).
    S: dict(h0,h1 (heel span or None), b0,b1 ball span, dHS, dTO, dMid, swing clearance); X: x of the heel (walk) or ball (run) at the strike of the current cycle"""
    al = G["alpha"]
    Hl, Bl = G["H"], G["Bl"]
    flat = -al
    XB = X + (rot_pt(flat, Bl)[0] - rot_pt(flat, Hl)[0])
    if S.get("heel") is None:
        XB = X
    def delta(u):
        ks = S["delta_keys"]
        return math.radians(interp(ks, u))
    if u < S["b1"]:
        d = delta(u)
        th = -al + d
        if S.get("heel") is not None and u < S["heel"][1]:
            return ankle_from((X, 0.0), th, Hl), d
        return ankle_from((XB, 0.0), th, Bl), d
    # swing: from toe-off to the next strike
    d0 = delta(S["b1"] - 1e-9)
    A0 = ankle_from((XB, 0.0), -al + d0, Bl)
    X1 = X + S["stride"]
    XB1 = X1 + (rot_pt(flat, Bl)[0] - rot_pt(flat, Hl)[0]) if S.get("heel") is not None else X1
    d1 = delta(1.0 - 1e-9) if False else math.radians(S["delta_keys"][0][1])
    th1 = -al + d1
    A1 = ankle_from((X1, 0.0), th1, Hl) if S.get("heel") is not None else ankle_from((X1, 0.0), th1, Bl)
    s = (u - S["b1"]) / (1.0 - S["b1"])
    sx = smooth(s) * 0.5 + s * 0.5 if S.get("lin", False) else smooth(s)
    x = A0[0] + (A1[0] - A0[0]) * sx
    y = A0[1] + (A1[1] - A0[1]) * smooth(s) + S["clear"] * math.sin(math.pi * s) ** 1.2
    d = math.radians(interp(S["delta_keys"], u))
    return (x, y), d


def gait_frames(a, N, T, v, spec, ypelvis_nominal, bob, reach=0.985, offset=0.5, smooth_y=1):
    """frames for one cycle: returns list of dict(root=(x,y_abs), legs={bone:offset}, contacts=[ids]) and contact spans"""
    G = geom_of(a)
    stride = v * T
    S = dict(spec)
    S["stride"] = stride
    L = G["l1"] + G["l2"]
    X0 = spec["X0"] * stride
    frames, ys = [], []
    for i in range(N):
        u = i / N
        t = i * T / N
        xp = v * t
        st = {}
        for side, ph in (("l", 0.0), ("r", offset)):
            up = (u - ph) % 1.0
            Xs = X0 + v * T * ph if u >= ph else X0 + v * T * (ph - 1)
            (ax, ay), d = leg_state(G, up, S, Xs)
            st[side] = ((ax, ay), d)
        ymax = [st[s][0][1] + math.sqrt(max((reach * L) ** 2 - (st[s][0][0] - xp) ** 2, 0.0)) for s in "lr"]
        nominal = ypelvis_nominal - bob * (1 - math.cos(4 * math.pi * u)) / 2
        ys.append(min(nominal, *ymax))
        frames.append((xp, st))
    if smooth_y:
        ys2 = [(ys[i - 1] + 2 * ys[i] + ys[(i + 1) % N]) / 4 for i in range(N)]
        ys = [min(a_, b_) for a_, b_ in zip(ys, ys2)]
    out = []
    for i, (xp, st) in enumerate(frames):
        out.append(dict(xp=xp, yp=ys[i], st=st, u=i / N))
    return out, G


def legs_to_offsets(a, G, fr, flexfn=None):
    """IK for both legs of one frame -> bone offsets (dict) and clipped flags"""
    off = {}
    root = {"x": fr["xp"], "y": fr["yp"] - G["hip_y"]}
    flags = []
    for s in "lr":
        (ax, ay), d = fr["st"][s]
        flags.append(solve_chain(a, off, root, f"thigh-{s}", f"shank-{s}", (ax, ay), +1))
        W = Wof(a, off, root)
        set_world(a, off, W, f"foot-{s}", -G["alpha"] + d)
        flex = max(0.0, -math.degrees(d)) if flexfn is None else flexfn(s, d)
        off.setdefault(f"toe-{s}", {})["angleDeg"] = round(min(flex, 64), 4)
    return pose_clean(off), root, flags


def spans_of(S, offset, T, which):
    """contact spans (fromS,toS) in clip time for one leg ('heel' or 'ball') with phase offset (0 or 0.5), wrapping split"""
    a0, a1 = S[which] if which == "heel" else (S["b0"], S["b1"])
    ph = offset
    out = []
    f, t = (a0 + ph), (a1 + ph)
    if t <= 1:
        out.append((f, t))
    elif f >= 1:
        out.append((f - 1, t - 1))
    else:
        out += [(f, 1.0), (0.0, t - 1)]
    return [(round(x * T, 4), round(y * T, 4)) for x, y in out]


def cycle_offsets(a, G, frames, S, N):
    """bone offsets of the legs for every frame of one looping cycle; swing feet follow the shank (ankle offset blended between the
    toe-off and the strike frame), toes blended from push-off flexion to zero."""
    res = []
    for fr in frames:
        off, root, fl = legs_to_offsets(a, G, fr)
        assert not any(fl), ("leg reach exceeded", fr["u"])
        res.append([off, root])
    for side, ph in (("l", 0.0), ("r", S.get("offset", 0.5))):
        ups = [((fr["u"] - ph) % 1.0) for fr in frames]
        sw = [i for i, up in enumerate(ups) if up >= S["b1"]]
        if not sw:
            continue
        first, last = sw[0], sw[-1]
        prev = res[(first - 1) % N][0]
        nxt = res[(last + 1) % N][0]
        f0, f1 = prev[f"foot-{side}"]["angleDeg"], nxt[f"foot-{side}"]["angleDeg"]
        t0, t1 = prev[f"toe-{side}"]["angleDeg"], 0.0
        k = len(sw) + 1
        for j, i in enumerate(sw):
            sfr = smooth((j + 1) / k)
            res[i][0][f"foot-{side}"]["angleDeg"] = round(f0 + (f1 - f0) * sfr + S.get("swing_foot", 0) * math.sin(math.pi * sfr), 4)
            res[i][0][f"toe-{side}"]["angleDeg"] = round(t0 + (t1 - t0) * sfr, 4)
    return res
