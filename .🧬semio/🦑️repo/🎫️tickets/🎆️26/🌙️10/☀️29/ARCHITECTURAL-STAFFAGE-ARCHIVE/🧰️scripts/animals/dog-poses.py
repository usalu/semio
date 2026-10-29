import sys, json, math; sys.path.insert(0, sys.argv[1])
import kin
a = kin.load(sys.argv[2], "labrador-dog")
H = 0.455
def hind(s, x, y, base, mabs=-85, pabs=-8):
    ch = [f"thigh-h{s}", f"shank-h{s}", f"metatarsus-h{s}", f"paw-h{s}"]
    return a.solve_limb(base, ch, f"contact-h{s}", (x, y), {ch[0]:("solve",), ch[1]:("solve",), ch[2]:("abs",mabs), ch[3]:("abs",pabs)}, root=base.get("_root",(0,0,0)))
def fore(s, x, y, base, mabs=-85, pabs=-10, sc=0):
    ch = [f"scapula-f{s}", f"humerus-f{s}", f"forearm-f{s}", f"metacarpus-f{s}", f"paw-f{s}"]
    return a.solve_limb(base, ch, f"contact-f{s}", (x, y), {ch[0]:("off",sc), ch[1]:("solve",), ch[2]:("solve",), ch[3]:("abs",mabs), ch[4]:("abs",pabs)}, root=base.get("_root",(0,0,0)))
HX, FX = 0.02, 0.55
def foot_pos(phase, D, exc, lift):
    # returns (dx, rise, swingfrac)
    if phase < D: return (exc/2 - exc*phase/D, 0.0, None)
    s = (phase - D)/(1-D)
    return (-exc/2 + exc*s, lift*math.sin(math.pi*s), s)
def gait(name, offs, D, exc, lift, jaw, nbob):
    out = []
    for k in range(8):
        ph = k/8
        base = {"jaw-ph": 0}
        b = {}
        stance = []
        for foot, off in offs.items():
            p = (ph - off) % 1
            dx, rise, s = foot_pos(p, D, exc, lift)
            sw = 0 if s is None else math.sin(math.pi*s)
            if foot[0] == "h":
                o, e = hind(foot[1], HX+dx, -H+rise, {}, -85+30*sw, -8-25*sw)
            else:
                o, e = fore(foot[1], FX+dx, -H+rise, {}, -85+25*sw, -10-20*sw, sc=max(-20,min(20,dx*70)))
            assert e < (0.004 if s is None else 0.02), (name, k, foot, e)
            b.update(o)
            if s is None: stance.append("contact-"+foot)
        b["jaw"] = jaw
        b["head"] = nbob*math.sin(2*math.pi*ph) if nbob else 0
        out.append((k, b, stance))
    return out
def fmt(b):
    return "{ " + ", ".join(f'"{k}": {{ "angleDeg": {round(v,1)} }}' for k, v in b.items() if abs(v) > 0.05) + " }"
if __name__ == "__main__":
    which = sys.argv[3]
    if which == "walk":
        for k, b, st in gait("walk", {"hn":0, "fn":.25, "hf":.5, "ff":.75}, .65, .34, .06, -6, 2): print(k, fmt(b), st)
    if which == "trot":
        for k, b, st in gait("trot", {"hn":0, "ff":0, "hf":.5, "fn":.5}, .55, .36, .08, -22, 1.5): print(k, fmt(b), st)
