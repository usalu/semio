# [scratch] NW-A1 gait key generator: foot trajectories + leg IK per phase
import math
from lib import *


def ss(u):
    u = max(0.0, min(1.0, u))
    return u * u * (3 - 2 * u)


class Leg:
    def __init__(self, sk, key, foot, td, x_ref):
        self.key, self.td, self.x_ref = key, td, x_ref
        hind = key.startswith("h")
        sfx = key
        self.hind = hind
        if hind:
            self.chain = ["thigh-" + sfx, "shank-" + sfx, "metatarsus-" + sfx, foot + "-" + sfx]
            self.pair = (self.chain[0], self.chain[1])
            self.sign = 1
        else:
            self.chain = ["scapula-" + sfx, "humerus-" + sfx, "forearm-" + sfx, "metacarpus-" + sfx, foot + "-" + sfx]
            self.pair = (self.chain[1], self.chain[2])
            self.sign = -1
        self.anchor = "contact-" + sfx
        self.scap = self.chain[0] if not hind else None


def leg_state(leg, phase, duty, S, G):
    """returns (x, y, ac, ah, scap_w, stance, u) for a leg at cycle phase; G: dict ac0, ks, flex, ah0, ahflex, ahroll, lift, scap0, scapamp"""
    psi = (phase - leg.td) % 1.0
    if psi < duty:
        u = psi / duty
        x = leg.x_ref + S / 2 - S * u
        y = 0.0
        ac = G["ac0"] + G["ks"] * (1 - 2 * u)
        ah = G["ah0"] - G["ahroll"] * ss((u - 0.85) / 0.15)
        stance = True
    else:
        u = (psi - duty) / (1 - duty)
        x = leg.x_ref - S / 2 + S * (u - math.sin(2 * math.pi * u) / (2 * math.pi))
        y = G["lift"] * math.sin(math.pi * u)
        ac = (G["ac0"] - G["ks"]) * (1 - ss(u)) + (G["ac0"] + G["ks"]) * ss(u) - G["flex"] * math.sin(math.pi * u)
        ah = (G["ah0"] - G["ahroll"]) * (1 - ss(u)) + G["ah0"] * ss(u) - G["ahflex"] * math.sin(math.pi * u)
        stance = False
    return x, y, ac, ah, stance, u


def solve_phase(sk, base_offs, legs, phase, duty, S, Gh, Gf, root=None, extra_y=0.0):
    offs = dict(base_offs)
    res = {}
    for leg in legs:
        G = Gh if leg.hind else Gf
        x, y, ac, ah, stance, u = leg_state(leg, phase, duty, S, G)
        fixed = {leg.chain[-1]: ah, leg.chain[-2]: ac}
        if not leg.hind:
            # shoulder swings with the leg: protracted at touchdown, retracted at lift-off
            psi = (phase - leg.td) % 1.0
            if psi < duty:
                fw = G["scap0"] + G["scapamp"] * (1 - 2 * psi / duty)
            else:
                v = (psi - duty) / (1 - duty)
                fw = (G["scap0"] - G["scapamp"]) * (1 - ss(v)) + (G["scap0"] + G["scapamp"]) * ss(v)
            fixed[leg.chain[0]] = fw
        p = solve_leg(sk, offs, leg.chain, leg.pair, fixed, leg.anchor, (x, y + extra_y), leg.sign, root)
        res[leg.key] = (stance, (x, y))
    return offs, res
