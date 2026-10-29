# [scratch] emits the cat pose fragment from IK-solved joint angles and authored texts
import sys, json, math
A, F = sys.argv[1], sys.argv[2]
sys.path.insert(0, A)
import importlib; kin = importlib.import_module('cat-kin')
a = kin.load(F, "domestic-cat")
def chains(s):
    return [f"thigh-h{s}", f"shank-h{s}", f"metatarsus-h{s}", f"paw-h{s}"], [f"scapula-f{s}", f"humerus-f{s}", f"forearm-f{s}", f"metacarpus-f{s}", f"paw-f{s}"]
BAD = []
def TOL(e, tol, *w):
    if e >= tol: BAD.append((w, round(e, 4)))
HB = (-65, 75); FB = (-45, 110)
def limb(*args, **kw):
    try: return limb0(*args, **kw)
    except TypeError:
        BAD.append((args[:4], kw.get('hb'), kw.get('fb'), 'infeasible')); return {}, 9.0
def limb0(kind, s, x, y, base, root, m, p, sc=0, hb=None, fb=None):
    h, f = chains(s)
    if kind == "h":
        return a.solve_limb(base, h, f"contact-h{s}", (x, y), {h[0]:("solve",)+(hb or HB), h[1]:("solve",), h[2]:("abs",m), h[3]:("abs",p)}, root=root)
    return a.solve_limb(base, f, f"contact-f{s}", (x, y), {f[0]:("off",sc), f[1]:("solve",)+(fb or FB), f[2]:("solve",), f[3]:("abs",m), f[4]:("abs",p)}, root=root)
def static(base, root, hx, fx, hgy, fgy, hm=(-82,-12), fm=(-85,-12), sc=0, tol=0.004, fdx=0.0, hdx=0.0, hb=None, fb=None):
    out = dict(base)
    for s in "nf":
        o, e = limb("h", s, hx + (hdx if s == "f" else 0), hgy, base, root, *hm, hb=hb); TOL(e, tol, "hind", s); out.update(o)
        o, e = limb("f", s, fx + (fdx if s == "f" else 0), fgy, base, root, *fm, sc=sc, fb=fb); TOL(e, tol, "fore", s); out.update(o)
    return out
def foot(phase, D, exc, lift):
    if phase < D: return exc/2 - exc*phase/D, 0.0, 0.0, True
    s = (phase - D)/(1-D); return -exc/2 + exc*s, lift*math.sin(math.pi*s), math.sin(math.pi*s), False
def gait(offs, D, exc, lift, base, bob, H, HX, FX, hm=-82, fm=-85, tail=0, lumb=2.5, scl=140):
    res = []
    for k in range(8):
        b = dict(base); st = []
        b["head"] = b.get("head", 0) + bob*math.sin(2*math.pi*k/8)
        b["lumbar"] = lumb*math.sin(2*math.pi*k/4)
        b["tail-3"] = b.get("tail-3", 0) + tail*math.sin(2*math.pi*k/8)
        b["tail-4"] = b.get("tail-4", 0) + tail*1.5*math.sin(2*math.pi*k/8 - 0.8)
        bk = dict(b)
        for ft, off in offs.items():
            dx, rise, sw, stance = foot((k/8 - off) % 1, D, exc, lift)
            if ft[0] == "h": o, e = limb("h", ft[1], HX+dx, -H+rise, bk, (0,0,0), hm+30*sw, -12-25*sw)
            else: o, e = limb("f", ft[1], FX+dx, -H+rise, bk, (0,0,0), fm+25*sw, -12-20*sw, sc=max(-28, min(28, dx*scl)))
            TOL(e, 0.004 if stance else 0.012, 'gait', k, ft, stance)
            b.update(o)
            if stance: st.append("contact-" + ft)
        res.append((b, st))
    return res
def bj(b): return {k: {"angleDeg": round(v, 1)} for k, v in b.items() if abs(v) > 0.05}
def pose(id, en, de, den, dde, bones, contacts, thermal, root=None, loads=()):
    p = {"id": id, "title": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "bones": bj(bones)}
    if root is not None: p["root"] = root
    p.update({"ground": "auto", "contacts": contacts, "thermal": thermal, "loads": list(loads)}); return p
def th(met, bn, ym=0, tongue=0.02, pads=1.0): return {"activityMet": met, "emitters": {"breath-nose": bn, "yawn-mouth": ym}, "regions": {"tongue": {"exposedFraction": tongue}, "pads": {"exposedFraction": pads}}}
ALL = ["contact-hn", "contact-hf", "contact-fn", "contact-ff"]
P = []
H0 = 0.225
tailAlert = {"tail-1": -20, "tail-2": -12, "tail-3": -8, "tail-4": -2, "tail-5": 20}
P.append(pose("standing-alert", "Standing alert", "Aufmerksam stehend", "Standing square on four paws, ears pricked forward, neck raised, the tail hanging low with the tip curved upward.", "Gerades Stehen auf vier Pfoten, Ohren nach vorn gespitzt, Hals erhoben, der Schwanz hängt tief mit nach oben gebogener Spitze.", static({**tailAlert, "neck": -2, "head": 4, "ear-n": -10, "ear-f": -10}, (0,0,0), 0.0, 0.28, -H0, -H0), ALL, th(0.9, 0.5)))
rootS = (0,0,50); bs = {"lumbar": 4, "thorax-a": -14, "thorax-b": -14, "neck": 24, "head": -50, "ear-n": -4, "ear-f": -4, "tail-1": -12, "tail-2": 122, "tail-3": 10, "tail-4": 15, "tail-5": 25, "scapula-fn": -20, "scapula-ff": -20}
w = a.fk(bs, rootS); rx, ry = a.anchor(w, "contact-rump"); sx = w["humerus-fn"][4]
P.append(pose("sitting-tail-wrapped", "Sitting with the tail wrapped", "Sitzend mit eingeschlagenem Schwanz", "Sitting upright on the sitting bones with the hind feet flat in front of the haunches, straight forelegs, head level, and the tail laid along the ground around the hind paws.", "Aufrechtes Sitzen auf den Sitzbeinhöckern, die Hinterfüße flach vor der Hinterhand, gerade Vorderbeine, Kopf waagerecht und der Schwanz um die Hinterpfoten auf den Boden gelegt.", static(bs, rootS, rx+0.14, sx, ry, ry, hm=(-12,-4), fm=(-80,-15), sc=-20, tol=0.006, hb=(-15, 40), fb=(-10, 60)), ALL + ["contact-rump"], th(0.75, 0.45, pads=0.7), root={"angleDeg": 50}))
SEP = 0.4
def stretch(id, en, de, den, dde, ang, dxs, h, base, met, ym=0, tongue=0.02, hm=(-82,-12), fm=(-20,-10), htol=0.006):
    sh = a.fk(base, (0,0,ang))["humerus-fn"]; fx0 = sh[4] + dxs; gy = sh[5] - h; rx = SEP - fx0
    bones = static(base, (0,0,ang), fx0 - SEP, fx0, gy, gy, hm=hm, fm=fm, tol=htol, hb=(-45, 40), fb=(-10, 140))
    print(id, "fx0", round(fx0, 3), "hx0", round(fx0 - SEP, 3), "rx", round(rx, 3), "gy", round(gy, 3), "hip height", round(a.fk(base, (0,0,ang))["thigh-hn"][5] - gy, 3), file=sys.stderr)
    return pose(id, en, de, den, dde, bones, ALL, th(met, 0.55, ym, tongue), root={"x": round(rx, 4), "angleDeg": ang})
tailUp = {"tail-1": -50, "tail-2": -20, "tail-3": -12, "tail-4": -6, "tail-5": 20}
P.append(stretch("stretch-stance", "Long stance before stretching", "Lange Standposition vor dem Strecken", "All four paws planted wide apart, the forepaws far in front: the back dips slightly and the head rises as the cat prepares to stretch.", "Alle vier Pfoten weit auseinander aufgesetzt, die Vorderpfoten weit vorn: der Rücken senkt sich leicht und der Kopf hebt sich, die Katze bereitet das Strecken vor.", -4, 0.12, 0.18, {**tailAlert, "lumbar": 4, "neck": 16, "ear-n": -4, "ear-f": -4}, 1.0))
P.append(stretch("play-bow", "Play bow", "Spielverbeugung", "Front down, hips up: the chest sinks between the extended forelegs while the hind legs stay straight and the tail is raised; the head looks forward.", "Vorderkörper unten, Hinterteil oben: die Brust sinkt zwischen die gestreckten Vorderbeine, die Hinterbeine bleiben gestreckt und der Schwanz ist erhoben; der Blick geht nach vorn.", -17, 0.15, 0.13, {**tailUp, "lumbar": 6, "thorax-a": 4, "neck": 26, "head": 6, "ear-n": -6, "ear-f": -6}, 1.5, fm=(-5,-10)))
P.append(stretch("stretch-deep", "Deep stretch", "Tiefes Strecken", "The full stretch: forelegs reaching forward along the ground, chest almost on the floor, the back swayed down and the rump high, head stretched out, the mouth open in a yawn.", "Das volle Strecken: Vorderbeine nach vorn über den Boden gestreckt, Brust fast am Boden, Rücken durchgebogen und Hinterteil hoch, Kopf vorgestreckt, das Maul zum Gähnen geöffnet.", -26, 0.17, 0.10, {**tailUp, "tail-1": -62, "lumbar": 10, "thorax-a": 6, "thorax-b": 2, "neck": 22, "head": -10, "jaw": -34, "ear-n": 30, "ear-f": 30}, 1.2, ym=1.0, tongue=0.6, hm=(-62,-14), fm=(0,-3)))
OFFS = {"hn":0, "fn":.25, "hf":.5, "ff":.75}
tailWalk = {"tail-1": -45, "tail-2": -18, "tail-3": -10, "tail-4": -4, "tail-5": 22}
walk = gait(OFFS, .625, .22, .035, {**tailWalk, "neck": 4, "head": 0, "ear-n": -6, "ear-f": -6, "jaw": 0}, 1.5, 0.215, 0.03, 0.31, tail=5)
tailStalk = {"tail-1": 6, "tail-2": 4, "tail-3": 0, "tail-4": 6, "tail-5": 12}
stalk = gait(OFFS, .75, .11, .028, {**tailStalk, "neck": -14, "head": -6, "ear-n": 24, "ear-f": 24, "thorax-b": 4}, 1.0, 0.17, 0.03, 0.3, hm=-70, fm=-70, tail=8, lumb=3, scl=120)
def tw(k, off=0): return k
names = {"hn": "near hind", "fn": "near fore", "hf": "far hind", "ff": "far fore"}
for kind, G, met, bn, en, de in (("walk", walk, 2.0, 0.8, "Walk", "Schritt"), ("stalk", stalk, 1.6, 0.4, "Stalk", "Pirsch")):
    for k, (b, st) in enumerate(G):
        sl = ", ".join(s[8:] for s in st)
        p = pose(f"{kind}-phase-{k}", f"{en}, phase {k + 1} of 8", f"{de}, Phase {k + 1} von 8", f"{en} at {k}/8 of the cycle; stance paws: {sl}.", f"{de} bei {k}/8 des Zyklus; Standpfoten: {sl}.", b, st, th(met, bn))
        p["key"] = True
        P.append(p)
for p in P:
    if p["id"].startswith(("walk-phase", "stalk-phase")): p["key"] = True
print(BAD)
open(A + "/domestic-cat/40-poses.json", "w").write('      "poses": [\n' + ",\n".join("        " + json.dumps(p, ensure_ascii=False) for p in P) + "\n      ],\n")
print(len(P), "poses")
