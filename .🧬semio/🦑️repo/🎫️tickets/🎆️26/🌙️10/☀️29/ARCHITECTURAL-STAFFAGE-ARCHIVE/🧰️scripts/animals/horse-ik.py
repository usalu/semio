# [scratch] horse limb IK helpers on top of kin.py (importable); poses are emitted by horse-emit.py
import sys, math
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/animals")
import kin
OUT = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/animals/horse.json"
a = kin.load(OUT, "horse")
HX, FX, H0 = 0.043, 1.051, 1.312
def chains(s):
    return [f"thigh-h{s}", f"shank-h{s}", f"cannon-h{s}", f"pastern-h{s}", f"hoof-h{s}"], [f"scapula-f{s}", f"humerus-f{s}", f"radius-f{s}", f"cannon-f{s}", f"pastern-f{s}", f"hoof-f{s}"]
def hind(s, x, y, base, root, ab=(-80, -58, -50)):
    h, _ = chains(s)
    return a.solve_limb(base, h, f"contact-h{s}", (x, y), {h[0]:("solve",), h[1]:("solve",), h[2]:("abs",ab[0]), h[3]:("abs",ab[1]), h[4]:("abs",ab[2])}, root=root)
def fore(s, x, y, base, root, ab=(-90, -55, -52), sc=0):
    _, f = chains(s)
    return a.solve_limb(base, f, f"contact-f{s}", (x, y), {f[0]:("off",sc), f[1]:("solve",), f[2]:("solve",), f[3]:("abs",ab[0]), f[4]:("abs",ab[1]), f[5]:("abs",ab[2])}, root=root)
def bj(b): return {k: {"angleDeg": round(v, 1)} for k, v in b.items() if abs(v) > 0.05}
def toe_offset(hoofabs, L=0.09, anchor=(0.0514, -0.046)):
    r = math.radians(hoofabs); c, s = math.cos(r), math.sin(r)
    dx, dy = anchor[0] - L, anchor[1]
    return dx*c - dy*s, dx*s + dy*c
