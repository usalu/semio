# [scratch] standing probe for the cat rig
import sys; sys.path.insert(0, sys.argv[1]); import kin
a = kin.load(sys.argv[2], "domestic-cat")
def chains(s):
    return [f"thigh-h{s}", f"shank-h{s}", f"metatarsus-h{s}", f"paw-h{s}"], [f"scapula-f{s}", f"humerus-f{s}", f"forearm-f{s}", f"metacarpus-f{s}", f"paw-f{s}"]
h, f = chains("n")
for gy in (-0.225, -0.2, -0.17):
  for hm in ((-80,-10),(-85,-10)):
    o, e = a.solve_limb({}, h, "contact-hn", (0.01, gy), {h[0]:("solve",), h[1]:("solve",), h[2]:("abs",hm[0]), h[3]:("abs",hm[1])})
    print("hind", gy, hm, {k: round(v,1) for k,v in o.items()}, round(e,4))
for gy in (-0.225, -0.2, -0.17):
    o, e = a.solve_limb({}, f, "contact-fn", (0.27, gy), {f[0]:("off",0), f[1]:("solve",), f[2]:("solve",), f[3]:("abs",-85), f[4]:("abs",-10)})
    print("fore", gy, {k: round(v,1) for k,v in o.items()}, round(e,4))
w = a.fk({}); print(a.anchor(w,"contact-hn"), a.anchor(w,"contact-fn"))
