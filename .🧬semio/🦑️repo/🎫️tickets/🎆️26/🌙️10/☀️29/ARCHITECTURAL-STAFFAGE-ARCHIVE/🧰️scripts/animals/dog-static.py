import sys, math; sys.path.insert(0, sys.argv[1])
sys.argv_saved = sys.argv
import importlib.util
spec = importlib.util.spec_from_file_location("dp", sys.argv[1] + "/dog-poses.py")
import kin
a = kin.load(sys.argv[2], "labrador-dog")
def chains(s):
    return [f"thigh-h{s}", f"shank-h{s}", f"metatarsus-h{s}", f"paw-h{s}"], [f"scapula-f{s}", f"humerus-f{s}", f"forearm-f{s}", f"metacarpus-f{s}", f"paw-f{s}"]
def solve(base, root, hx, fx, gy, hm=(-85,-8), fm=(-85,-10), sc=0):
    out = dict(base)
    for s, dx in (("n", 0), ("f", 0)):
        h, f = chains(s)
        o, e = a.solve_limb(base, h, f"contact-h{s}", (hx+dx, gy), {h[0]:("solve",), h[1]:("solve",), h[2]:("abs",hm[0]), h[3]:("abs",hm[1])}, root=root); out.update(o); print("hind", s, round(e,4), file=sys.stderr)
        o, e = a.solve_limb(base, f, f"contact-f{s}", (fx+dx, gy), {f[0]:("off",sc), f[1]:("solve",), f[2]:("solve",), f[3]:("abs",fm[0]), f[4]:("abs",fm[1])}, root=root); out.update(o); print("fore", s, round(e,4), file=sys.stderr)
    return out
def fmt(b):
    return "{ " + ", ".join(f'"{k}": {{ "angleDeg": {round(v,1)} }}' for k, v in b.items() if abs(v) > 0.05) + " }"
mode = sys.argv[3]
if mode == "stand":
    b = solve({"ear-n":25,"ear-f":25,"tail-1":-12,"tail-2":-6,"head":3}, (0,0,0), -0.0, 0.55, -0.5)
    print(fmt(b))
if mode == "sniff":
    base = {"neck":-50,"head":-45,"ear-n":30,"ear-f":30,"tail-1":-8,"thorax":-6}
    b = solve(base, (0,0,-8), 0.0, 0.52, -0.5)
    w = a.fk(b, (0,0,-8)); print(fmt(b)); print("nose", a.anchor(w, "nose"), file=sys.stderr)
if mode == "sit":
    root = (0,0,43)
    base = {"thorax":-10,"neck":-10,"head":-24,"ear-n":10,"ear-f":10,"tail-1":-50,"tail-2":-10}
    w = a.fk(base, root); rx, ry = a.anchor(w, "contact-rump"); print("rump", rx, ry, file=sys.stderr)
    sh = a.fk(base, root)["scapula-fn"]; print("shoulder", sh[4], sh[5], file=sys.stderr)
    b = solve(base, root, rx+0.22, sh[4]+0.03, ry, hm=(-30,-4), fm=(-92,-10), sc=-18)
    w = a.fk(b, root)
    print(fmt(b)); print({k: tuple(round(v,3) for v in a.anchor(w,k)) for k in ("contact-hn","contact-hf","contact-fn","contact-ff","contact-rump","nose")}, file=sys.stderr)
