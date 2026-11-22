"""🚶️ HW-05 sagittal gait generator: smooth foot paths with heel, foot-flat and forefoot rockers, two-link inverse kinematics of the
hip-knee-ankle chain of the asset's own rig and opposed arm swing, sampled as clip keys whose planted contacts never slide.

Units: metres, seconds, degrees. World side elevation: x ahead, y up. Bone offsets follow the rig convention (positive turns u towards v,
so a positive thigh offset is hip flexion, a negative shank offset knee flexion, a positive foot offset dorsiflexion).

@see https://en.wikipedia.org/wiki/Gait_(human) — rockers and phases
@see ./lib.py — rest frames of the rig
"""
import math

from lib import frames, sub, col, r


def _deg(x):
    return math.degrees(x)


def _angle(v):
    return math.atan2(v[1], v[0])


def _rot(p, a):
    c, s = math.cos(a), math.sin(a)
    return (p[0] * c - p[1] * s, p[0] * s + p[1] * c)


def _minjerk(s):
    return s * s * s * (10 - 15 * s + 6 * s * s)


def leg_geometry(asset, side="l"):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    fr = frames(asset["rig"]["bones"])
    root = next(b for b in asset["rig"]["bones"] if b["parent"] is None)
    hip = fr[f"thigh-{side}"][0]
    anchors = {a["id"]: a for a in asset["anchors"]}
    heel = anchors[f"heel-{side}"]
    foot = bones[f"foot-{side}"]
    toe = bones[f"toe-{side}"]
    foot_rest = _angle(col(fr[f"foot-{side}"][1], 0))
    return {
        "hip": (hip[0] - root["x"], hip[1] - root["y"]),
        "L1": bones[f"thigh-{side}"]["lengthM"],
        "L2": bones[f"shank-{side}"]["lengthM"],
        "Lf": foot["lengthM"],
        "Lt": toe["lengthM"],
        "footRel": foot["angleDeg"],
        "toeRel": toe["angleDeg"],
        "footRest": foot_rest,
        "heel": (heel["x"], heel["y"]),
        "rootY": root["y"],
    }


def foot_points(g, ankle, psi):
    """World heel, ball and toe tip of a foot whose bone points at absolute angle psi (rad) with its ankle at `ankle`."""
    hx, hy = _rot(g["heel"], psi)
    bx, by = _rot((g["Lf"], 0), psi)
    return (ankle[0] + hx, ankle[1] + hy), (ankle[0] + bx, ankle[1] + by)


def ik(g, hip, ankle):
    d = (ankle[0] - hip[0], ankle[1] - hip[1])
    n = math.hypot(*d)
    reach = math.sqrt(g["L1"] ** 2 + g["L2"] ** 2 + 2 * g["L1"] * g["L2"] * math.cos(math.radians(g.get("kneeMin", 0.0))))
    if n > reach:
        raise ValueError(f"leg overstretched {n:.4f} > {reach:.4f}")
    ck = (n * n - g["L1"] ** 2 - g["L2"] ** 2) / (2 * g["L1"] * g["L2"])
    kappa = math.acos(max(-1, min(1, ck)))
    psi_d = _angle(d)
    psi1 = psi_d + math.atan2(g["L2"] * math.sin(kappa), g["L1"] + g["L2"] * math.cos(kappa))
    psi2 = psi1 - kappa
    return psi1, psi2


def leg_state(g, p, t, hip_local=None):
    """Ankle, absolute foot angle, toe offset and contacts of one leg at cycle time t (its own heel strike at t = 0, heel at x = 0)."""
    T, S = p["T"], p["stride"]
    t = t % T
    t1, t2, t3 = p["footFlat"] * T, p["heelOff"] * T, p["toeOff"] * T
    flat = g["footRest"]
    heel_flat = _rot(g["heel"], flat)
    ankle_flat = (-heel_flat[0], -heel_flat[1])
    ball_flat = (ankle_flat[0] + g["Lf"] * math.cos(flat), ankle_flat[1] + g["Lf"] * math.sin(flat))

    def about_heel(pitch, heel_x):
        psi = flat + math.radians(pitch)
        h = _rot(g["heel"], psi)
        return (heel_x - h[0], -h[1]), psi

    def about_ball(pitch, ball_x):
        psi = flat + math.radians(pitch)
        b = _rot((g["Lf"], 0), psi)
        return (ball_x - b[0], ball_flat[1] - b[1]), psi

    if t < t1:
        s = t / t1
        pitch = p["pitchStrike"] * (1 - s * s * (3 - 2 * s))
        ankle, psi = about_heel(pitch, 0.0)
        return ankle, psi, 0.0, ["heel"]
    if t < t2:
        return ankle_flat, flat, 0.0, ["heel", "ball"]
    if t < t3:
        s = (t - t2) / (t3 - t2)
        pitch = p["pitchOff"] * s * s * (3 - 2 * s)
        ankle, psi = about_ball(pitch, ball_flat[0])
        return ankle, psi, -pitch, ["ball"]
    a0, psi0 = about_ball(p["pitchOff"], ball_flat[0])
    a1, psi1 = about_heel(p["pitchStrike"], S)
    s = (t - t3) / (T - t3)
    m = _minjerk(s)
    h0, h1, h = hip_local(t3), hip_local(T), hip_local(t)
    r0 = (a0[0] - h0[0], a0[1] - h0[1])
    r1 = (a1[0] - h1[0], a1[1] - h1[1])
    peak = p["clearancePeak"]
    w = 0.5 * s / peak if s < peak else 0.5 + 0.5 * (s - peak) / (1 - peak)
    x = h[0] + r0[0] + (r1[0] - r0[0]) * m
    y = h[1] + r0[1] + (r1[1] - r0[1]) * m + p["clearance"] * math.sin(math.pi * w)
    pitch_mid = p["pitchSwing"]
    if s < 0.4:
        q = s / 0.4
        pitch = p["pitchOff"] + (pitch_mid - p["pitchOff"]) * q * q * (3 - 2 * q)
    else:
        q = (s - 0.4) / 0.6
        pitch = pitch_mid + (p["pitchStrike"] - pitch_mid) * q * q * (3 - 2 * q)
    toe = -p["pitchOff"] * max(0.0, 1 - s / 0.35) ** 2
    return (x, y), flat + math.radians(pitch), toe, []


def hip_at(g, p, t, h0):
    """Hip joint: forward at constant speed, highest at single-support midstance (h0), lowest by `drop` in double support."""
    T = p["T"]
    x = p["speed"] * (t - p["midStance"] * T) + p["hipAhead"]
    y = h0 - p["drop"] * (1 - math.cos(4 * math.pi * (t - p["midStance"] * T) / T)) / 2
    return (x, y)


def solve_cycle(asset, p, n):
    """Keys of one gait cycle (left heel strike at t = 0): bone offsets, contacts and the lowest swing clearance."""
    gl, gr = leg_geometry(asset, "l"), leg_geometry(asset, "r")
    gl["kneeMin"] = gr["kneeMin"] = p["kneeMin"]
    T = p["T"]

    def legs(t, h0):
        out = {}
        for side, g, shift, xs in (("l", gl, 0.0, 0.0), ("r", gr, T / 2, p["stride"] / 2)):
            ankle, psi_f, toe, contacts = leg_state(g, p, t - shift, lambda tau, sh=shift, x0=xs: (hip_at(g, p, tau + sh, h0)[0] - x0, hip_at(g, p, tau + sh, h0)[1]))
            k = math.floor((t - shift) / T)
            ankle = (ankle[0] + xs + k * p["stride"], ankle[1])
            hip = hip_at(g, p, t, h0)
            psi1, psi2 = ik(g, hip, ankle)
            th = _deg(psi1) + 90
            sh = _deg(psi2 - psi1)
            ft = _deg(psi_f - psi2) - g["footRel"]
            out[side] = {"thigh": th, "shank": sh, "foot": ft, "toe": toe, "contacts": contacts, "ankle": ankle, "psi": psi_f, "g": g}
        return out

    flat = gl["footRest"]
    hf = _rot(gl["heel"], flat)
    ankle_flat = (-hf[0], -hf[1])
    reach_mid = math.sqrt(gl["L1"] ** 2 + gl["L2"] ** 2 + 2 * gl["L1"] * gl["L2"] * math.cos(math.radians(p["kneeMid"])))
    dx = p["hipAhead"] - ankle_flat[0]
    h0 = ankle_flat[1] + math.sqrt(reach_mid ** 2 - dx ** 2)
    lo, hi = 0.0, 0.3
    for _ in range(50):
        mid = (lo + hi) / 2
        p["drop"] = mid
        try:
            for i in range(240):
                legs(T * i / 240, h0)
            hi = mid
        except ValueError:
            lo = mid
    p["drop"] = hi + p.get("dropSlack", 0.0)
    keys = []
    clearance = 1.0
    for i in range(400):
        t = T * i / 400
        st = legs(t, h0)
        for side in ("l", "r"):
            if not st[side]["contacts"]:
                heel, ball = foot_points(st[side]["g"], st[side]["ankle"], st[side]["psi"])
                tip_psi = st[side]["psi"] + math.radians(st[side]["g"]["toeRel"] + st[side]["toe"])
                tip = (ball[0] + st[side]["g"]["Lt"] * math.cos(tip_psi), ball[1] + st[side]["g"]["Lt"] * math.sin(tip_psi))
                clearance = min(clearance, heel[1], ball[1], tip[1])
    for i in range(n):
        t = T * i / n
        st = legs(t, h0)
        ph = 2 * math.pi * t / T
        arm_l = p["armBias"] - p["armSwing"] * math.cos(ph)
        arm_r = p["armBias"] + p["armSwing"] * math.cos(ph)
        fa_l = p["elbowBias"] + p["elbowSwing"] * (1 - math.cos(ph + math.pi)) / 2
        fa_r = p["elbowBias"] + p["elbowSwing"] * (1 - math.cos(ph)) / 2
        bones = {}
        for side in ("l", "r"):
            s = st[side]
            bones[f"thigh-{side}"] = {"angleDeg": r(s["thigh"], 3)}
            bones[f"shank-{side}"] = {"angleDeg": r(s["shank"], 3)}
            bones[f"foot-{side}"] = {"angleDeg": r(s["foot"], 3)}
            bones[f"toe-{side}"] = {"angleDeg": r(max(0.0, s["toe"]), 3)}
        bones["arm-l"] = {"angleDeg": r(arm_l, 3)}
        bones["arm-r"] = {"angleDeg": r(arm_r, 3)}
        bones["forearm-l"] = {"angleDeg": r(fa_l, 3)}
        bones["forearm-r"] = {"angleDeg": r(fa_r, 3)}
        bones["torso"] = {"angleDeg": r(p["lean"] + p["leanSwing"] * math.cos(4 * math.pi * (t - p["midStance"] * T) / T), 3)}
        keys.append({"t": r(t, 4), "bones": bones, "contacts": {side: st[side]["contacts"] for side in ("l", "r")}})
    return {"keys": keys, "h0": h0, "drop": p["drop"], "clearance": clearance}


def contact_spans(p):
    """Heel and ball spans of both feet over one cycle (left heel strike at 0), wrapped into [0, T]."""
    T = p["T"]
    out = []
    for side, shift in (("l", 0.0), ("r", T / 2)):
        for anchor, a, b in (("heel", 0.0, p["heelOff"] * T), ("ball", p["footFlat"] * T, p["toeOff"] * T)):
            a, b = a + shift, b + shift
            if b <= T:
                out.append({"anchor": f"{anchor}-{side}", "fromS": r(a, 4), "toS": r(b, 4)})
            elif a >= T:
                out.append({"anchor": f"{anchor}-{side}", "fromS": r(a - T, 4), "toS": r(b - T, 4)})
            else:
                out.append({"anchor": f"{anchor}-{side}", "fromS": r(a, 4), "toS": r(T, 4)})
                out.append({"anchor": f"{anchor}-{side}", "fromS": 0.0, "toS": r(b - T, 4)})
    return out


def walk_clip(cyc, p, clip_id, title, description, pose, met, breathing, extra=lambda t: {}):
    """A looping planted walk clip from a solved cycle: keys, contact spans, the four gait phases and the contact events."""
    from lib import T
    Tc = p["T"]
    keys = [{"t": k["t"], "pose": pose, "ease": "smooth", "bones": dict(k["bones"], **extra(k["t"]))} for k in cyc["keys"]]
    ds = p["toeOff"] - 0.5
    events = []
    for side, sh, en, de in (("l", 0.0, "Left", "Linke"), ("r", Tc / 2, "Right", "Rechte")):
        events.append({"t": r(sh % Tc, 4), "kind": "contact-on", "title": T(f"{en} foot lands", f"{de}r Fuß setzt auf"), "target": f"heel-{side}"})
        events.append({"t": r((p["toeOff"] * Tc + sh) % Tc, 4), "kind": "contact-off", "title": T(f"{en} toe-off", f"{de}r Abdruck"), "target": f"ball-{side}"})
    events.sort(key=lambda e: e["t"])
    phases = [
        {"id": "double-support-left-lead", "fromS": 0.0, "toS": r(ds * Tc, 4), "title": T("Double support, left foot leading", "Doppelstandphase, linker Fuß vorn")},
        {"id": "left-single-support", "fromS": r(ds * Tc, 4), "toS": r(0.5 * Tc, 4), "title": T("Left single support, right leg swinging", "Linke Einzelstandphase, rechtes Bein schwingt")},
        {"id": "double-support-right-lead", "fromS": r(0.5 * Tc, 4), "toS": r((0.5 + ds) * Tc, 4), "title": T("Double support, right foot leading", "Doppelstandphase, rechter Fuß vorn")},
        {"id": "right-single-support", "fromS": r((0.5 + ds) * Tc, 4), "toS": r(Tc, 4), "title": T("Right single support, left leg swinging", "Rechte Einzelstandphase, linkes Bein schwingt")},
    ]
    return {"id": clip_id, "title": title, "description": description, "durationS": Tc, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": p["stride"],
            "keys": keys, "contacts": contact_spans(p), "phases": phases, "events": events,
            "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": met}, {"t": Tc, "v": met}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": breathing}, {"t": Tc, "v": breathing}]}]}
