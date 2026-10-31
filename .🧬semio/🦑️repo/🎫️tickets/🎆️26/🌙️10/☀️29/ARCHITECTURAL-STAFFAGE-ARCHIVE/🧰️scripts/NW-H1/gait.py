"""NW-H1: periodic plantar-foot gait generator on the 2-D rig. Feet roll heel -> flat -> heel-off about the planted anchors; the pelvis advances with constant mean speed.
All numbers (angles, fractions) are authored estimates guided by Perry & Burnfield 2010 / Winter 2009 (stance about 60-65 % of the cycle in slow walking)."""
import math
from nwlib import *


def smooth(s):
    s = max(0.0, min(1.0, s))
    return s * s * (3 - 2 * s)


class Gait:
    def __init__(self, h, T, S, stance=0.64, u1=0.12, u2=0.40, hs_deg=14.0, to_deg=32.0, clear=0.045, flat_k=0.30, bob=0.010, y0=None, y_sag=0.03):
        self.h, self.T, self.S = h, T, S
        self.sf, self.u1, self.u2 = stance, u1, u2
        self.hs, self.to, self.clear = hs_deg, to_deg, clear
        self.flat_k = flat_k
        self.bob = bob
        self.y0 = (h.hh - y_sag) if y0 is None else y0
        self.v = S / T

    def px(self, t):
        return self.v * t

    def py(self, t):
        ph = (t / self.T) % 1.0
        return self.y0 + self.bob * math.cos(4 * math.pi * (ph - 0.2))

    def _flat_ankle(self, hs_time):
        h = self.h
        return self.px(hs_time) + self.flat_k * self.S + 0.0

    def foot(self, t, offset):
        """state of one foot at time t: dict(ankle=(x,y), foot, toe, heel_planted, ball_planted)"""
        h, T = self.h, self.T
        n = math.floor(t / T - offset + 1e-12)
        t_hs = (n + offset) * T
        u = (t - t_hs) / T
        xa = self.px(t_hs) + self.flat_k * self.S
        xh = xa - h.heel_back
        xb = xa + h.Lball
        rest = -h.theta
        sf, u1, u2 = self.sf, self.u1, self.u2
        if u < u1:
            s = u / u1
            ang = rest + self.hs * (1 - smooth(s))
            ank = h.foot_pose((xh, 0.0), "heel", ang)
            return dict(ankle=ank, foot=ang, toe=ang + h.theta, heel=True, ball=False, u=u, xa=xa)
        if u < u2:
            ank = h.foot_pose((xh, 0.0), "heel", rest)
            return dict(ankle=ank, foot=rest, toe=0.0, heel=True, ball=True, u=u, xa=xa)
        if u < sf:
            s = (u - u2) / (sf - u2)
            ang = rest - self.to * smooth(s)
            ank = h.foot_pose((xb, 0.0), "ball", ang)
            return dict(ankle=ank, foot=ang, toe=0.0, heel=False, ball=True, u=u, xa=xa)
        s = (u - sf) / (1 - sf)
        a_to = rest - self.to
        ank0 = h.foot_pose((xb, 0.0), "ball", a_to)
        xa_n = xa + self.S
        xh_n = xa_n - h.heel_back
        ang_hs = rest + self.hs
        ank1 = h.foot_pose((xh_n, 0.0), "heel", ang_hs)
        sm = smooth(s)
        x = ank0[0] + (ank1[0] - ank0[0]) * sm
        y = ank0[1] + (ank1[1] - ank0[1]) * sm + self.clear * math.sin(math.pi * s) ** 1.2
        ang = a_to + (ang_hs - a_to) * smooth(min(1.0, s * 1.15))
        toe_w = ang + h.theta + self.to * (1 - smooth(min(1.0, s * 1.4)))
        return dict(ankle=(x, y), foot=ang, toe=toe_w, heel=False, ball=False, u=u, xa=xa)

    def spans(self, offset_l=0.0, offset_r=0.5):
        """contact spans over one cycle [0,T] for anchors of both feet (clip-local times; wraps handled by splitting)"""
        T = self.T
        out = []
        for s, off in (("l", offset_l), ("r", offset_r)):
            for kind, a, b in (("heel", 0.0, self.u2), ("ball", self.u1, self.sf)):
                for k in (-1, 0, 1):
                    f = (k + off + a) * T
                    e = (k + off + b) * T
                    lo, hi = max(0.0, f), min(T, e)
                    if hi - lo > 1e-6:
                        out.append({"anchor": kind + "-" + s, "fromS": r4(lo), "toS": r4(hi)})
        return out
