"""Gait key frames by inverse kinematics (scratch aid): planted feet stay fixed, the hip follows a prescribed path.
Computes leg offsets only; the asset script adds trunk, head and arms. All parameters are authored values passed in."""
from lib import *

def smooth(u): u = max(0.0, min(1.0, u)); return u * u * (3 - 2 * u)

def leg_ik(hip, ankle, l1, l2):
    """World directions of thigh and shank with the knee in front (+x side)."""
    best = None
    for bend in (1, -1):
        th1, th2 = two_link(hip, ankle, l1, l2, bend)
        knee = (hip[0] + l1 * math.cos(math.radians(th1)), hip[1] + l1 * math.sin(math.radians(th1)))
        if best is None or knee[0] > best[0][0]: best = (knee, th1, th2)
    return best[1], best[2]

def gait_keys(h, mode, A, sigma, y_ss, bob, alpha_to, clearance, ho_frac=0.5, n=10, flight_rise=0.0, stance_dip=0.0):
    s = h.s
    l1, l2 = h.L["thigh"], h.L["shank"]
    ab = (0.13 * s, -0.08 * s)
    Lfull = l1 + l2
    keys = []
    def ankle_for(ball, a):
        c, sn = math.cos(math.radians(a)), math.sin(math.radians(a))
        return (ball[0] - (c * ab[0] - sn * ab[1]), ball[1] - (sn * ab[0] + c * ab[1]))
    def foot_state(phi_f, P):
        """ankle world, foot world angle for one foot with local phase phi_f in [0,1); P = planted ball x of the current/last stance."""
        ho = ho_frac * sigma if mode == "walk" else 0.35 * sigma
        if phi_f < sigma:
            a = 0.0 if phi_f < ho else -alpha_to * smooth((phi_f - ho) / (sigma - ho))
            return ankle_for((P, 0.0), a), a
        u = (phi_f - sigma) / (1 - sigma)
        a0 = ankle_for((P, 0.0), -alpha_to)
        a1 = ankle_for((P + A, 0.0), 0.0)
        e = smooth(u)
        x = a0[0] + (a1[0] - a0[0]) * e
        y = a0[1] + (a1[1] - a0[1]) * e + clearance * math.sin(math.pi * u) ** 1.2
        ang = -alpha_to * (1 - smooth(u / 0.75))
        return (x, y), ang
    def rel_at(hip_pt, ankle, a_world):
        th1, th2 = leg_ik(hip_pt, ankle, l1, l2)
        return norm(a_world - th2 - 90)
    for k in range(n):
        phi = k / n
        if mode == "walk":
            y_ds = y_ss - bob
            yh = y_ds + (y_ss - y_ds) * (1 - math.cos(4 * math.pi * phi)) / 2
        else:
            yh = (y_ss - 0.5 * stance_dip) - stance_dip * math.cos(4 * math.pi * (phi - sigma / 2))
        hip = (A * phi, yh)
        D = sigma * A / 2
        out = {"phi": phi, "hipY": yh}
        for side, phi0 in (("l", 0.0), ("r", 0.5)):
            pf = (phi - phi0) % 1.0
            P = A * (phi0 + math.floor(phi - phi0 + 1e-9)) + D
            ankle, a = foot_state(pf, P)
            if pf >= sigma:
                u = (pf - sigma) / (1 - sigma)
                tp0 = (phi - pf + sigma) % 1e9
                hip_to = (A * (phi - pf + sigma), (y_ss - stance_dip) if mode == "run" else y_ss - bob * 0.5)
                a_to = foot_state(sigma, P)
                hip_td = (A * (phi - pf + 1.0), y_ss if mode == "run" else y_ss - bob)
                a_td = foot_state(1.0 - 1e-9, P)
                rel0 = rel_at(hip_to, a_to[0], a_to[1])
                rel1 = rel_at(hip_td, ankle_for((P + A, 0.0), 0.0), 0.0)
                rel = rel0 + (rel1 - rel0) * smooth(u / 0.8)
            th1, th2 = leg_ik(hip, ankle, l1, l2)
            dist = math.hypot(ankle[0] - hip[0], ankle[1] - hip[1])
            if dist > l1 + l2 + 1e-6: raise ValueError(f"unreachable at phi={phi} {side}: {dist:.3f} > {l1 + l2:.3f}")
            if pf < sigma: rel = norm(a - th2 - 90)
            out[f"thigh-{side}"] = {"angleDeg": r(norm(th1 + 90), 2)}
            out[f"shank-{side}"] = {"angleDeg": r(norm(th2 - th1), 2)}
            out[f"foot-{side}"] = {"angleDeg": r(rel, 2)}
        keys.append(out)
    return keys

GAIT_NAMES = [("heel-strike-l", "Heel strike, left", "Fersenkontakt links", "Left heel strike; the right foot is still on its toes behind (double support).", "Linker Fersenkontakt; der rechte Fuß steht noch hinten auf dem Ballen (Doppelstütz)."),
              ("toe-off-r", "Toe-off, right", "Abdruck rechts", "Right toe-off ends double support; the right leg starts to swing.", "Rechter Abdruck beendet die Doppelstützphase; das rechte Bein beginnt zu schwingen."),
              ("midstance-l", "Midstance, left", "Mittlere Standphase links", "Body vertically over the left foot, highest hip position; right leg swings through.", "Körper senkrecht über dem linken Fuß, höchste Hüftlage; das rechte Bein schwingt durch."),
              ("heel-off-l", "Heel-off, left", "Fersenabhebung links", "Left heel leaves the ground while the right leg reaches forward.", "Linke Ferse löst sich vom Boden, während das rechte Bein nach vorn greift."),
              ("late-swing-r", "Late swing, right", "Späte Schwungphase rechts", "Right foot descends to the ground ahead of the body.", "Rechter Fuß senkt sich vor dem Körper zum Boden."),
              ("heel-strike-r", "Heel strike, right", "Fersenkontakt rechts", "Right heel strike; the left foot is still on its toes behind (double support).", "Rechter Fersenkontakt; der linke Fuß steht noch hinten auf dem Ballen (Doppelstütz)."),
              ("toe-off-l", "Toe-off, left", "Abdruck links", "Left toe-off ends double support; the left leg starts to swing.", "Linker Abdruck beendet die Doppelstützphase; das linke Bein beginnt zu schwingen."),
              ("midstance-r", "Midstance, right", "Mittlere Standphase rechts", "Body vertically over the right foot; left leg swings through.", "Körper senkrecht über dem rechten Fuß; das linke Bein schwingt durch."),
              ("heel-off-r", "Heel-off, right", "Fersenabhebung rechts", "Right heel leaves the ground while the left leg reaches forward.", "Rechte Ferse löst sich vom Boden, während das linke Bein nach vorn greift."),
              ("late-swing-l", "Late swing, left", "Späte Schwungphase links", "Left foot descends to the ground ahead of the body.", "Linker Fuß senkt sich vor dem Körper zum Boden.")]
WALK_CONTACTS = [["heel-l", "contact-l", "contact-r"], ["heel-l", "contact-l"], ["heel-l", "contact-l"], ["heel-l", "contact-l"], ["contact-l"],
                 ["heel-r", "contact-r", "contact-l"], ["heel-r", "contact-r"], ["heel-r", "contact-r"], ["heel-r", "contact-r"], ["contact-r"]]

def walk_spans(T, double=0.1, stance=0.6):
    """Contact spans of a walking cycle of duration T; the right foot's wrap-around span comes first so the driver does not switch at the seam."""
    return [span("contact-r", 0, double * T), span("heel-l", 0, 0.3 * T), span("contact-l", 0, stance * T), span("heel-r", 0.5 * T, 0.8 * T), span("contact-r", 0.5 * T, T)]

def leg_only(k): return {n: v for n, v in k.items() if n.startswith(("thigh", "shank", "foot"))}
