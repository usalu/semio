import sys; sys.path.insert(0, sys.argv[1])
import kin
a = kin.load(sys.argv[2], "labrador-dog")
H = 0.50
def hind(s, x, y=-H, base={}):
    ch = [f"thigh-h{s}", f"shank-h{s}", f"metatarsus-h{s}", f"paw-h{s}"]
    return a.solve_limb(base, ch, f"contact-h{s}", (x, y), {ch[0]:("solve",), ch[1]:("solve",), ch[2]:("abs",-85), ch[3]:("abs",-8)})
def fore(s, x, y=-H, base={}):
    ch = [f"scapula-f{s}", f"humerus-f{s}", f"forearm-f{s}", f"metacarpus-f{s}", f"paw-f{s}"]
    return a.solve_limb(base, ch, f"contact-f{s}", (x, y), {ch[0]:("off",0), ch[1]:("solve",), ch[2]:("solve",), ch[3]:("abs",-82), ch[4]:("abs",-10)})
for f, x in ((hind, 0.03), (fore, 0.25)):
    o, e = f("n", x); print(f.__name__, {k: round(v,1) for k,v in o.items()}, "err", round(e,4))
