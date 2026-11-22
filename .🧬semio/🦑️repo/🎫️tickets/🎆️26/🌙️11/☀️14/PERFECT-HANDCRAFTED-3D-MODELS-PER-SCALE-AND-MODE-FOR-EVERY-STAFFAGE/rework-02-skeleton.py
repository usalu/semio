"""🦴️ The human skeleton of author 02 as one connected load path, stated bone by bone (rework 2026-11-22).

`human_skeleton(P, scale, masses)` writes the structure list of one scale from the measures `P` an asset script states (bone lengths,
joint offsets, skull points, foot points, ribcage breadths, contact anchors). The anatomy is fixed here by hand: skull with mandible,
cervical column, thoracolumbar column with its lordosis and kyphosis and the spinous processes, twelve rib pairs from the column to
the sternum (1 to 7), to the costal arch (8 to 10) or free (11, 12), sternum, clavicles, scapulae with spine, acromion and glenoid,
sacrum, ilia with their crests, pubic and ischial rami meeting in the symphysis, femur with head, neck, trochanter and condyles,
patella, tibia and fibula, talus, calcaneus, metatarsus and toes, humerus, ulna and radius (thumb side ahead), carpus, metacarpus,
phalanges. Every member overlaps its neighbour at the joint centre, so the path stays closed in every pose, and it ends under
each forefoot in the pad of sole and plantar tissue that stands on the contact anchor (a support solid where the asset declares the
anchor in `structure.supports`). Coarser scales keep the topology and merge what falls below the feature: 1:100 six rib pairs and one
finger blade, 1:200 and 1:500 a ribcage and a pelvis basin as one mass each and one member per limb bone.
Frames: trunk, neck, head u up, +v back; limbs u distal, +v front; +w the figure's left; `k` scales the adult numbers (1 = 1.80 m).
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(HERE, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

FLOOR = {50: 0.004, 100: 0.0078, 200: 0.0155, 500: 0.0155}
RIB_BREADTH = [0.055, 0.085, 0.105, 0.118, 0.126, 0.132, 0.136, 0.138, 0.136, 0.130, 0.112, 0.090]
RIB_DROP = [0.030, 0.045, 0.058, 0.068, 0.076, 0.084, 0.090, 0.090, 0.085, 0.075, 0.040, 0.030]


def human_skeleton(P, sc, masses):
    """🦴️ The structure solids of one scale: members, joints, supports, then the stored mass markers."""
    k, f = P["k"], max(FLOOR[sc], P.get("floor", {}).get(sc, 0))
    fine, mid, coarse = sc == 50, sc == 100, sc >= 200
    arm = P.get("arm", "upper-arm")
    out = []

    def tag(solid, bone, part="member", **extra):
        solid.update(member=bone, part=part, **extra)
        out.append(solid)
        return solid

    def cap(id, bone, p0, p1, r0, r1=None, **extra):
        return tag(lib.capsule(id, bone, "bone", 0.85, p0, p1, lib.r5(max(r0, f)), lib.r5(max(r0 if r1 is None else r1, f)), **extra), bone)

    def ell(id, bone, at, radii, part="member", tone=0.85, **extra):
        return tag(lib.ellipsoid(id, bone, "bone", tone, at, [lib.r5(max(r, f)) for r in radii]), bone, part, **extra)

    def lo(id, bone, rows, sub=2, **kw):
        return tag(lib.loft(id, bone, "bone", 0.85, [(u, cv, cw, max(rv, f), max(rw, f), 2) for u, cv, cw, rv, rw in rows], sub, **kw), bone)

    Lp, Lt, Ln, us, zs, zh = P["pelvis"], P["torso"], P["neck"], P["shoulder_u"], P["shoulder_z"], P["hip_z"]
    S = P["skull"]
    spine_v = lambda u: 0.036 * k * max(0.0, 1 - ((u - 0.70 * Lt) / (0.30 * Lt)) ** 2) - 0.012 * k * max(0.0, 1 - ((u - 0.18 * Lt) / (0.18 * Lt)) ** 2)

    # head
    ell("sk-cranium", "head", S["cranium"], S["cranium_radii"])
    if not coarse:
        ell("sk-face", "head", S["face"], S["face_radii"])
        for s, t in ((1, "l"), (-1, "r")):
            condyle, angle, chin = [(p[0], p[1], p[2] * s) for p in (S["condyle"], S["angle"], S["chin"])]
            cap(f"sk-mandible-ramus-{t}", "head", condyle, angle, 0.008 * k)
            cap(f"sk-mandible-body-{t}", "head", angle, chin, 0.008 * k, 0.009 * k)
    else:
        cap("sk-mandible", "head", (S["face"][0], S["face"][1], 0), (S["chin"][0] + f * 0.6, S["chin"][1] + f * 0.8, 0), 0.03 * k)
    # neck and trunk column
    lo("sk-cervical", "neck", [(-0.012 * k, 0.004 * k, 0, 0.017 * k, 0.020 * k), (Ln * 0.5, -0.004 * k, 0, 0.015 * k, 0.018 * k), (Ln + 0.014 * k, 0.0, 0, 0.015 * k, 0.019 * k)])
    lo("sk-spine", "torso", [(u, spine_v(u), 0, r * k, r * 1.15 * k) for u, r in ((-0.012 * k, 0.024), (0.18 * Lt, 0.024), (0.40 * Lt, 0.021), (0.70 * Lt, 0.018), (0.90 * Lt, 0.016), (Lt + 0.012 * k, 0.016))], 3)
    if fine:
        for i in range(17):
            u = Lt * (0.035 + 0.92 * i / 16)
            name = f"l{5 - i}" if i < 5 else f"t{17 - i}"
            cap(f"sk-process-{name}", "torso", (u, spine_v(u), 0), (u - 0.012 * k, spine_v(u) + (0.058 if i < 5 else 0.050) * k, 0), 0.006 * k, 0.005 * k)
    # ribcage
    top = Lt - 0.012 * k
    sternum_top, sternum_low = (Lt - 0.034 * k, -0.062 * k), (Lt - 0.235 * k, -0.104 * k)
    sternum_v = lambda u: sternum_top[1] + (sternum_low[1] - sternum_top[1]) * min(1.0, max(0.0, (sternum_top[0] - u) / (sternum_top[0] - sternum_low[0])))
    kw = P.get("chest", k)
    if not coarse:
        lo("sk-sternum", "torso", [(sternum_low[0] - 0.02 * k, sternum_low[1] + 0.004 * k, 0, 0.006 * k, 0.010 * k), (sternum_low[0], sternum_low[1], 0, 0.008 * k, 0.018 * k), (Lt - 0.09 * k, sternum_v(Lt - 0.09 * k), 0, 0.008 * k, 0.020 * k), (sternum_top[0] + 0.004 * k, sternum_top[1], 0, 0.009 * k, 0.030 * k)])
        for s, t in ((1, "l"), (-1, "r")):
            arch_a, arch_b = (sternum_low[0], sternum_low[1], 0.016 * k * s), (Lt - 0.345 * k, -0.066 * k, 0.118 * kw * s)
            cap(f"sk-costal-arch-{t}", "torso", arch_a, arch_b, 0.006 * k, 0.005 * k)
            for i in (range(12) if fine else (0, 2, 4, 6, 8, 10)):
                u0, w, drop = top - i * 0.0245 * k, RIB_BREADTH[i] * kw, RIB_DROP[i] * k
                v0 = spine_v(u0)
                points = [(u0, v0 + 0.008 * k, 0.016 * k * s), (u0 - 0.008 * k, v0 + 0.050 * k, 0.60 * w * s), (u0 - 0.45 * drop, 0.008 * k, w * s)]
                if i < 7:
                    end = (max(u0 - drop, sternum_low[0] - 0.012 * k), sternum_v(u0 - drop), 0.012 * k * s)
                    points += [(u0 - 0.8 * drop, 0.72 * end[1], 0.64 * w * s), end]
                elif i < 10:
                    a = (0.30, 0.62, 0.96)[i - 7]
                    end = tuple(arch_a[j] + (arch_b[j] - arch_a[j]) * a for j in range(3))
                    points += [(u0 - 0.85 * drop, 0.80 * end[1], max(abs(end[2]), 0.72 * w) * s), end]
                for j, (a, b) in enumerate(zip(points, points[1:])):
                    cap(f"sk-rib-{i + 1}-{t}-{j + 1}", "torso", a, b, 0.005 * k)
    else:
        rows = [(top + 0.004 * k, 0.050), (top - 0.07 * k, 0.108), (top - 0.17 * k, 0.136), (top - 0.26 * k, 0.126), (top - 0.31 * k, 0.090)]
        lo("sk-ribcage", "torso", [(u, spine_v(u) * 0.3 - 0.012 * k, 0, 0.62 * w * k + 0.012 * k, w * kw) for u, w in rows], 2)
    # shoulder girdle
    for s, t in ((1, "l"), (-1, "r")):
        acromion = (us + 0.030 * k, 0.010 * k, (zs - 0.024 * k) * s)
        cap(f"sk-clavicle-{t}", "torso", (sternum_top[0] - 0.004 * k, sternum_top[1] + 0.004 * k, 0.018 * k * s), acromion, 0.0065 * k, 0.006 * k)
        if not coarse:
            body = [(us - 0.155 * k, 0.078 * k, 0.075 * kw * s, 0.006 * k, 0.012 * k), (us - 0.08 * k, 0.082 * k, 0.090 * kw * s, 0.006 * k, 0.034 * k), (us - 0.01 * k, 0.072 * k, 0.098 * kw * s, 0.007 * k, 0.048 * k), (us + 0.02 * k, 0.062 * k, 0.104 * kw * s, 0.007 * k, 0.038 * k)]
            lo(f"sk-scapula-{t}", "torso", body)
            cap(f"sk-scapular-spine-{t}", "torso", (us + 0.004 * k, 0.074 * k, 0.062 * kw * s), acromion, 0.006 * k)
            cap(f"sk-scapula-neck-{t}", "torso", (us - 0.006 * k, 0.062 * k, 0.118 * kw * s), (us, 0.004 * k, (zs - 0.020 * k) * s), 0.008 * k)
            ell(f"sk-glenoid-{t}", "torso", (us, 0.002 * k, (zs - 0.018 * k) * s), (0.018 * k, 0.014 * k, 0.010 * k))
        else:
            cap(f"sk-scapula-{t}", "torso", (us - 0.12 * k, 0.060 * k, 0.085 * kw * s), acromion, 0.012 * k)
            cap(f"sk-glenoid-{t}", "torso", acromion, (us, 0, zs * s), 0.012 * k)
    # pelvis
    ta = P.get("torso_at", (Lp, 0.0))
    if not coarse:
        lo("sk-sacrum", "pelvis", [(ta[0] - 0.115 * k, ta[1] + 0.062 * k, 0, 0.008 * k, 0.010 * k), (ta[0] - 0.07 * k, ta[1] + 0.058 * k, 0, 0.014 * k, 0.030 * k), (ta[0] - 0.02 * k, ta[1] + 0.030 * k, 0, 0.020 * k, 0.050 * k), (ta[0] + 0.010 * k, ta[1] + 0.004 * k, 0, 0.022 * k, 0.046 * k)])
    for s, t in ((1, "l"), (-1, "r")):
        hip = (0.0, 0.0, zh * s)
        if not coarse:
            ell(f"sk-acetabulum-{t}", "pelvis", hip, (0.032 * k, 0.032 * k, 0.022 * k))
            crest = [(ta[0] - 0.030 * k, ta[1] + 0.060 * k, 0.045 * k * s), (ta[0] + 0.030 * k, ta[1] + 0.035 * k, 0.105 * k * s), (ta[0] + 0.048 * k, ta[1] - 0.010 * k, 0.138 * k * s), (ta[0] + 0.018 * k, ta[1] - 0.070 * k, 0.126 * k * s)]
            for j, (a, b) in enumerate(zip(crest, crest[1:])):
                cap(f"sk-iliac-crest-{t}-{j + 1}", "pelvis", a, b, 0.009 * k)
            lo(f"sk-ilium-{t}", "pelvis", [(0.010 * k, 0.004 * k, (zh + 0.004 * k) * s, 0.030 * k, 0.014 * k), (ta[0] * 0.5, ta[1] * 0.5 + 0.004 * k, (zh + 0.016 * k) * s, 0.046 * k, 0.010 * k), (ta[0] + 0.020 * k, ta[1] - 0.004 * k, 0.120 * k * s, 0.058 * k, 0.009 * k), (ta[0] + 0.044 * k, ta[1] - 0.010 * k, 0.134 * k * s, 0.040 * k, 0.008 * k)])
            cap(f"sk-sacroiliac-{t}", "pelvis", (ta[0] - 0.025 * k, ta[1] + 0.036 * k, 0.040 * k * s), (ta[0] + 0.004 * k, ta[1] + 0.030 * k, 0.100 * k * s), 0.013 * k)
            cap(f"sk-pubis-{t}", "pelvis", (-0.004 * k, -0.022 * k, (zh - 0.012 * k) * s), (-0.040 * k, -0.072 * k, 0.006 * k * s), 0.010 * k)
            cap(f"sk-ischium-{t}", "pelvis", (-0.012 * k, 0.012 * k, (zh - 0.010 * k) * s), (-0.082 * k, 0.030 * k, 0.058 * k * s), 0.012 * k, 0.015 * k)
            cap(f"sk-ischiopubic-ramus-{t}", "pelvis", (-0.082 * k, 0.030 * k, 0.058 * k * s), (-0.052 * k, -0.066 * k, 0.010 * k * s), 0.008 * k)
        else:
            cap(f"sk-hip-{t}", "pelvis", (ta[0] - 0.01 * k, ta[1] + 0.02 * k, 0.03 * k * s), hip, 0.03 * k, 0.026 * k)
    if coarse:
        lo("sk-pelvis", "pelvis", [(-0.07 * k, 0.0, 0, 0.040 * k, 0.070 * k), (0.0, 0.004 * k, 0, 0.060 * k, zh + 0.02 * k), (ta[0] + 0.03 * k, ta[1] * 0.6, 0, 0.070 * k, 0.130 * k)])
    # limbs
    Lth, Lsh, La, Lf, Lh = P["thigh"], P["shank"], P["arm_length"], P["forearm"], P["hand"]
    foot = P["foot"]
    for s, t in ((1, "l"), (-1, "r")):
        stump = P.get("stump") == t
        if not coarse:
            ell(f"sk-femur-head-{t}", f"thigh-{t}", (0, 0, 0), (0.024 * k, 0.024 * k, 0.024 * k))
            cap(f"sk-femur-neck-{t}", f"thigh-{t}", (0, 0, 0), (0.045 * k, 0.002 * k, 0.056 * k * s), 0.014 * k, 0.016 * k)
            ell(f"sk-trochanter-{t}", f"thigh-{t}", (0.030 * k, 0.000, 0.066 * k * s), (0.026 * k, 0.018 * k, 0.016 * k))
            lo(f"sk-femur-{t}", f"thigh-{t}", [(0.040 * k, 0.002 * k, 0.054 * k * s, 0.016 * k, 0.016 * k), (0.28 * Lth, 0.010 * k, 0.038 * k * s, 0.0145 * k, 0.014 * k), (0.62 * Lth, 0.012 * k, 0.016 * k * s, 0.0140 * k, 0.0135 * k), (Lth - 0.055 * k, 0.004 * k, 0.004 * k * s, 0.018 * k, 0.020 * k), (Lth - 0.016 * k, -0.002 * k, 0, 0.031 * k, 0.041 * k)], round1=0.022 * k)
            if not stump:
                ell(f"sk-patella-{t}", f"thigh-{t}", (Lth - 0.010 * k, 0.034 * k, 0), (0.022 * k, 0.010 * k, 0.021 * k))
        else:
            cap(f"sk-femur-{t}", f"thigh-{t}", (0, 0, 0), (Lth, 0, 0), 0.022 * k, 0.026 * k)
        if stump:
            cap(f"sk-tibia-{t}", f"shank-{t}", (-0.004 * k, 0.004 * k, 0), (P["stump_length"], 0.004 * k, 0), 0.026 * k if not coarse else 0.03 * k, 0.014 * k)
            if not coarse:
                cap(f"sk-fibula-{t}", f"shank-{t}", (0.030 * k, -0.012 * k, 0.034 * k * s), (P["stump_length"] - 0.02 * k, -0.008 * k, 0.026 * k * s), 0.007 * k)
            continue
        if not coarse:
            lo(f"sk-tibia-{t}", f"shank-{t}", [(0.004 * k, 0.002 * k, 0, 0.028 * k, 0.039 * k), (0.07 * k, 0.010 * k, -0.002 * k * s, 0.017 * k, 0.016 * k), (Lsh - 0.07 * k, 0.004 * k, -0.002 * k * s, 0.013 * k, 0.013 * k), (Lsh - 0.012 * k, 0.0, -0.002 * k * s, 0.018 * k, 0.023 * k)], round0=0.012 * k, round1=0.012 * k)
            cap(f"sk-fibula-{t}", f"shank-{t}", (0.030 * k, -0.012 * k, 0.034 * k * s), (Lsh + 0.004 * k, -0.006 * k, 0.027 * k * s), 0.007 * k, 0.009 * k)
        else:
            cap(f"sk-tibia-{t}", f"shank-{t}", (0, 0, 0), (Lsh, 0, 0), 0.024 * k, 0.018 * k)
        heel, ball, toe_tip, lift = foot["heel"], foot["ball"], foot["toe"], foot["lift"]
        up = foot["up"]
        raised = lambda p, h: (p[0] + up[0] * h, p[1] + up[1] * h)
        heel_b, ball_b = raised(heel, lift + 0.016 * k), raised(ball, lift + 0.010 * k)
        if not coarse:
            ell(f"sk-talus-{t}", f"foot-{t}", (0.004 * k, -0.004 * k, 0), (0.026 * k, 0.020 * k, 0.022 * k))
            cap(f"sk-calcaneus-{t}", f"foot-{t}", (0, 0, 0), (heel_b[0], heel_b[1], 0.003 * k * s), 0.018 * k, 0.017 * k)
            for j, w in enumerate((-0.030, -0.015, 0.0, 0.015, 0.030) if fine else (-0.02, 0.02)):
                cap(f"sk-metatarsal-{j + 1}-{t}", f"foot-{t}", (ball_b[0] * 0.18, ball_b[1] * 0.18 - 0.004 * k, w * 0.5 * k * s), (ball_b[0], ball_b[1], (w + 0.004) * k * s), 0.0075 * k if fine else 0.012 * k, 0.006 * k if fine else 0.011 * k)
                cap(f"sk-toe-{j + 1}-{t}", f"toe-{t}", (-0.004 * k, lift + 0.010 * k, (w + 0.004) * k * s), (toe_tip * (0.95 - 0.07 * abs(j - 1)), lift + 0.006 * k, (w * 1.05 + 0.002) * k * s), 0.006 * k if fine else 0.011 * k, 0.005 * k if fine else 0.010 * k)
        else:
            cap(f"sk-calcaneus-{t}", f"foot-{t}", (0, 0, 0), (heel_b[0], heel_b[1], 0), 0.018 * k)
            cap(f"sk-foot-{t}", f"foot-{t}", (0, 0, 0), (ball_b[0], ball_b[1], 0), 0.018 * k, 0.016 * k, lateralScale=1.6)
            cap(f"sk-phalanges-{t}", f"toe-{t}", (-0.004 * k, lift + 0.012 * k, 0), (toe_tip * 0.8, lift + 0.010 * k, 0), 0.014 * k, lateralScale=1.6)
        pad = max(0.008 * k, f)
        sole = tag(lib.capsule(f"st-sole-{t}", f"foot-{t}", P["sole_material"], 0.5, (ball[0] + up[0] * pad, ball[1] + up[1] * pad, 0), (ball_b[0], ball_b[1], 0), lib.r5(pad)), f"foot-{t}")
        if foot["anchor"].format(t) in P.get("supports", ()):
            sole.update(part="support", anchor=foot["anchor"].format(t))
    for s, t in ((1, "l"), (-1, "r")):
        if not coarse:
            ell(f"sk-humerus-head-{t}", f"{arm}-{t}", (0, 0, -0.002 * k * s), (0.023 * k, 0.023 * k, 0.023 * k))
            lo(f"sk-humerus-{t}", f"{arm}-{t}", [(0.012 * k, 0.0, 0, 0.014 * k, 0.014 * k), (0.35 * La, 0.002 * k, 0, 0.0115 * k, 0.0115 * k), (La - 0.05 * k, 0.0, 0, 0.012 * k, 0.014 * k), (La - 0.010 * k, 0.0, 0, 0.015 * k, 0.031 * k)], round1=0.014 * k)
            cap(f"sk-ulna-{t}", f"forearm-{t}", (-0.012 * k, -0.010 * k, 0), (Lf - 0.004 * k, -0.016 * k, 0), 0.011 * k, 0.006 * k)
            cap(f"sk-radius-{t}", f"forearm-{t}", (0.006 * k, 0.011 * k, 0), (Lf, 0.012 * k, 0), 0.008 * k, 0.011 * k)
        else:
            cap(f"sk-humerus-{t}", f"{arm}-{t}", (0, 0, 0), (La, 0, 0), 0.02 * k, 0.018 * k)
            cap(f"sk-forearm-{t}", f"forearm-{t}", (0, 0, 0), (Lf, 0, 0), 0.018 * k, 0.014 * k)
        hand = P["hand_points"]
        if P.get("no_hand", {}).get(sc):
            continue
        if not coarse:
            ell(f"sk-carpus-{t}", f"hand-{t}", (0.014 * k, 0.002 * k, 0), (0.017 * k, 0.025 * k, 0.010 * k))
            lo(f"sk-metacarpus-{t}", f"hand-{t}", [(0.026 * k, 0.003 * k, -0.002 * k * s, 0.026 * k, 0.007 * k), (hand["knuckle"] - 0.002 * k, hand["palm_v"], -0.005 * k * s, 0.036 * k, 0.006 * k)])
            a, b = hand["thumb"][0], hand["thumb"][1]
            cap(f"sk-thumb-{t}", f"hand-{t}", (a[0], a[1], a[2] * s), (b[0], b[1], b[2] * s), 0.005 * k, 0.0045 * k)
            if fine:
                c = hand["thumb"][2]
                cap(f"sk-thumb-tip-{t}", f"hand-{t}", (b[0], b[1], b[2] * s), (c[0], c[1], c[2] * s), 0.0045 * k, 0.004 * k)
                for i, chain in enumerate(hand["fingers"], 1):
                    for j, (p, q) in enumerate(zip(chain, chain[1:])):
                        cap(f"sk-phalanx-{i}-{j + 1}-{t}", f"hand-{t}", (p[0], p[1], p[2] * s), (q[0], q[1], q[2] * s), 0.0045 * k, 0.004 * k)
            else:
                first, last = hand["fingers"][1], hand["fingers"][2]
                mid_v = (first[0][1] + last[0][1]) / 2
                lo(f"sk-fingers-{t}", f"hand-{t}", [(p[0], mid_v, p[2] * s, 0.030 * k, 0.0078) for p in first[:3]], 1)
        else:
            tip = hand["mitten_tip"]
            cap(f"sk-hand-{t}", f"hand-{t}", (0, 0, 0), (tip[0], tip[1], tip[2] * s), 0.016 * k, 0.014 * k)
    # joints: one marker at the origin of every articulated bone
    joints = ["torso", "neck", "head"] + [f"{b}-{t}" for t in ("l", "r") for b in (arm, "forearm", "hand", "thigh", "shank", "foot", "toe")]
    for bone in joints:
        if P.get("stump") and bone in (f"foot-{P['stump']}", f"toe-{P['stump']}"):
            continue
        if bone.startswith("hand") and P.get("no_hand", {}).get(sc):
            continue
        r = (0.020 if bone in ("torso",) or bone.startswith(("thigh", "shank")) else 0.014) * k
        at = (0, P["foot"]["lift"] + 0.010 * k, 0) if bone.startswith("toe") else (0, 0, 0)
        ell(f"joint-{bone}", bone, at, (r * 0.7, r, r * (2.2 if bone.startswith("toe") else 1.0)), part="joint", tone=0.5)
    return out + [dict(m) for m in masses]
