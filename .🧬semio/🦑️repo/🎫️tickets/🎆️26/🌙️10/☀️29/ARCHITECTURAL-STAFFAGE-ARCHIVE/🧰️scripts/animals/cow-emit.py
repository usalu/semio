# [scratch] emits the dairy-cow pose fragment from IK-solved joint angles and authored texts
import sys, json, math
F = sys.argv[1]; OUTF = sys.argv[2]
exec(open(__file__.replace("cow-emit.py", "cow-ik.py")).read().replace("F = sys.argv[1] if len(sys.argv) > 1 else None", "F = sys.argv[1]"))
G = 1.10
def err(tag, e, tol=0.004):
    assert e < tol, (tag, e)
def stand(base, root, hx, fx, g=G, hm=-86, hp=-52, fm=-86, fp=-52, sc=0):
    out = dict(base)
    for s, dh, df in (("n", hx[0], fx[0]), ("f", hx[1], fx[1])):
        o, e = hind(s, dh, -g, base, root, hm, hp); err(("h", s), e); out.update(o)
        o, e = fore(s, df, -g, base, root, fm, fp, sc=sc); err(("f", s), e); out.update(o)
    return out
def bj(b): return {k: {"angleDeg": round(v, 1)} for k, v in b.items() if abs(v) > 0.05}
def th(met, br): return {"activityMet": met, "emitters": {"breath-nose": br}}
HOOVES = ["contact-hn", "contact-hf", "contact-fn", "contact-ff"]
def pose(id, en, de, den, dde, bones, contacts, thermal, root=None, key=False):
    p = {"id": id, "title": {"en": en, "de": de}, "description": {"en": den, "de": dde}}
    if key: p["key"] = True
    p["bones"] = bj(bones)
    if root is not None: p["root"] = {"angleDeg": root}
    p.update({"ground": "auto", "contacts": contacts, "thermal": thermal, "loads": []}); return p
P = []
# standing, ruminating
base = {"jaw": -4, "head": 2, "tail-2": 4, "tail-3": 3, "ear-n": 6, "ear-f": 4, "udder": 0}
P.append(pose("standing-chewing", "Standing and chewing the cud", "Stehend beim Wiederkäuen",
  "Standing square with the head carried below the withers, the near hind leg under the hip, the far hind leg forward and the far fore leg slightly behind; the jaw is half open in the rhythm of rumination and the ears rest sideways.",
  "Ruhiges Stehen mit unter dem Widerrist getragenem Kopf, nahem Hinterbein unter der Hüfte, fernem Hinterbein vorn und fernem Vorderbein leicht zurück; der Kiefer ist im Rhythmus des Wiederkäuens halb geöffnet, die Ohren stehen seitlich.",
  stand(base, (0, 0, 0), (0.03, 0.17), (1.14, 1.08)), HOOVES, th(2.95, 0.5)))
# grazing
rootG = (0, 0, -8)
bg = {"neck": -72, "thorax": -3, "ear-n": 14, "ear-f": 12, "tail-2": 6, "jaw": -6}
best = None
for nk in range(-80, -39, 2):
  for h in range(0, 56):
    w = a.fk({**bg, "neck": nk, "head": h}, rootG); n = a.anchor(w, "nose")
    e = abs(n[1] - (-G + 0.04))
    if e < 0.01 and (best is None or n[0] > best[2][0]): best = (e, (nk, h), n)
print("graze head", best, file=sys.stderr)
bg["neck"], bg["head"] = best[1]; best = (0, best[1][1])
P.append(pose("grazing-head-down", "Grazing with the head down", "Weidend mit gesenktem Kopf",
  "Neck and head lowered until the muzzle touches the sward, the front lower than the hindquarters; the near fore leg steps forward, the ears point forward and the jaw is open for the bite.",
  "Hals und Kopf gesenkt, bis das Flotzmaul den Rasen berührt, die Vorhand tiefer als die Hinterhand; das nahe Vorderbein steht vor, die Ohren zeigen nach vorn und der Kiefer ist zum Biss geöffnet.",
  stand(bg, rootG, (0.03, 0.15), (1.3, 1.1), sc=0), HOOVES, th(3.4, 0.6), root=-8))
# lying
GL = 0.53
rootL = (0, 0, 0)
bl = {"neck": 6, "head": -4, "jaw": -4, "ear-n": 6, "ear-f": 4, "tail-1": 20, "tail-2": 14, "udder": 85}
tv, tinfo = fit(bl, rootL, ["thorax"], [("contact-sternum", "y", -GL)], w=0)
bl.update(tv); print("thorax", tv, tinfo, file=sys.stderr)
L = dict(bl)
def hind_lying(s, hx):
    best = None
    for t in range(-20, 46):
        h, _ = chains(s)
        base = {**L, h[0]: t}
        try: o, e = a.solve_limb(base, h, f"contact-h{s}", (hx, -GL), {h[0]:("off", t), h[1]:("solve",), h[2]:("solve",), h[3]:("abs", -12)}, root=rootL)
        except Exception: continue
        w = a.fk({**base, **o}, rootL); hy = a.anchor(w, f"contact-hock-h{s}")[1]
        sc = e + abs(hy + GL)
        if best is None or sc < best[0]: best = (sc, e, hy, {**o, h[0]: t})
    return best
for s, hx in (("n", 0.5), ("f", 0.4)):
    b = hind_lying(s, hx); L.update(b[3]); print("hind", s, b[:3], file=sys.stderr)
bf = min((fore("n", x / 100, -GL, L, rootL, -172, 150, aid="contact-carpus-fn") + (x,) for x in range(110, 170, 5)), key=lambda r: r[1])
L.update(bf[0]); print("fore n", bf[1:], file=sys.stderr)
o, e = fore("f", 2.0, -GL, L, rootL, -15, -8); L.update(o); print("fore f", e, file=sys.stderr)
LYING = L
LCON = ["contact-sternum", "contact-carpus-fn", "contact-ff", "contact-hn", "contact-hf", "contact-hock-hn", "contact-hock-hf"]
P.append(pose("lying-resting", "Lying and resting", "Liegend ruhend",
  "Lying on the sternum with the near fore leg folded under the chest on its carpus, the far fore leg stretched forward on the hoof and both hind legs folded beneath the haunches; the weight rests on the brisket, the carpus, the hocks and the hooves while the head stays raised and the cow ruminates.",
  "Liegen auf dem Brustbein, das nahe Vorderbein unter der Brust auf der Vorderfußwurzel eingeschlagen, das ferne Vorderbein gestreckt vorn auf der Klaue und beide Hinterbeine unter der Hinterhand angezogen; das Gewicht ruht auf Brustspitze, Vorderfußwurzel, Sprunggelenken und Klauen, der Kopf bleibt erhoben und die Kuh käut wieder.",
  LYING, LCON, th(2.55, 0.45)))
# kneeling front
rootK = (0, 0, -12); GK = 1.0
bk = {"neck": -22, "head": 14, "jaw": -3, "ear-n": 10, "ear-f": 8, "tail-2": 8, "udder": 4}
K = dict(bk)
for s_, hx in (("n", 0.03), ("f", 0.17)):
    o, e = hind(s_, hx, -GK, K, rootK, -80, -50); err(("hk", s_), e, 0.006); K.update(o)
for s_ in "nf":
    bf = min((fore(s_, x / 100, -GK, K, rootK, -172, 150, aid=f"contact-carpus-f{s_}") + (x,) for x in range(100, 160, 5)), key=lambda r: r[1])
    print("kneel fore", s_, bf[1:], file=sys.stderr); K.update(bf[0])
KCON = ["contact-carpus-fn", "contact-carpus-ff", "contact-hn", "contact-hf"]
P.append(pose("kneeling-front", "Kneeling on the carpi", "Auf den Vorderfußwurzeln kniend",
  "Mid-way through lying down: both fore legs are folded and the cow rests on the front of the carpi, the cannons and hooves tucked back, the head lowered and the hindquarters still raised on the extended hind legs.",
  "Mitten im Abliegen: beide Vorderbeine sind eingeschlagen und die Kuh ruht auf der Vorderseite der Vorderfußwurzeln, Röhrbeine und Klauen nach hinten gelegt, der Kopf gesenkt und die Hinterhand noch auf den gestreckten Hinterbeinen erhoben.",
  K, KCON, th(3.3, 0.7), root=-12))
rootH = (0, 0, -5); GH = 0.8
bh = {"neck": -12, "head": 10, "jaw": -3, "ear-n": 8, "ear-f": 6, "tail-1": 10, "tail-2": 10, "udder": 25}
H_ = dict(bh)
for s_, hx in (("n", 0.2), ("f", 0.24)):
    o, e = hind(s_, hx, -GH, H_, rootH, -62, -45); err(("hh", s_), e, 0.006); H_.update(o)
for s_ in "nf":
    bf = min((fore(s_, x / 100, -GH, H_, rootH, -172, 150, aid=f"contact-carpus-f{s_}") + (x,) for x in range(100, 160, 5)), key=lambda r: r[1])
    print("lower fore", s_, bf[1:], file=sys.stderr); H_.update(bf[0])
P.append(pose("lowering-hindquarters", "Lowering the hindquarters", "Hinterhand absenkend",
  "Second half of lying down: the hind legs fold at stifle and hock and the haunches sink while the cow stays on its carpi.",
  "Zweite Hälfte des Abliegens: die Hinterbeine beugen sich in Knie und Sprunggelenk und die Hinterhand sinkt, die Kuh bleibt auf den Vorderfußwurzeln.",
  H_, KCON, th(3.3, 0.7), root=-5, key=True))
# walk
T_W = 1.4; D = 0.68; EXC = 0.7; HW = 1.06; LIFT = 0.1
def foot(phase):
    if phase < D: return EXC/2 - EXC*phase/D, 0.0, 0.0, True
    s_ = (phase - D)/(1-D); return -EXC/2 + EXC*s_, LIFT*math.sin(math.pi*s_), math.sin(math.pi*s_), False
offs = {"hn": 0, "fn": .25, "hf": .5, "ff": .75}
vW = EXC / D / T_W
metW = round((870 + 1220*vW) / (58.15*5.3), 2)
WALK = []
for k in range(8):
    ph = 2*math.pi*k/8
    b = {"jaw": -3, "ear-n": 6, "ear-f": 4, "neck": 3*math.sin(ph), "head": -3*math.sin(ph) + 2, "tail-1": 8*math.sin(ph), "tail-2": 10*math.sin(ph - 0.8), "tail-3": 12*math.sin(ph - 1.6), "udder": 4*math.sin(ph), "thorax": 1.0*math.sin(ph), "lumbar": -1.0*math.sin(ph)}
    base_k = dict(b); st = []
    for ft, off in offs.items():
        dx, rise, sw, stance = foot((k/8 - off) % 1)
        if ft[0] == "h": o, e = hind(ft[1], 0.03 + dx, -HW + rise, base_k, (0, 0, 0), -86 + 25*sw, -52 - 15*sw)
        else: o, e = fore(ft[1], 1.27 + dx, -HW + rise, base_k, (0, 0, 0), -86 - 35*sw, -52 - 30*sw, sc=max(-20, min(12, dx*55)))
        err((k, ft), e, 0.004 if stance else 0.02); b.update(o)
        if stance: st.append("contact-" + ft)
    WALK.append((b, st))
names = {"hn": "near hind", "fn": "near fore", "hf": "far hind", "ff": "far fore"}; dn = {"hn": "nahes Hinterbein", "fn": "nahes Vorderbein", "hf": "fernes Hinterbein", "ff": "fernes Vorderbein"}
for k, (b, st) in enumerate(WALK):
    sl = [s_[8:] for s_ in st]
    P.append(pose(f"walk-phase-{k}", f"Walk, phase {k + 1} of 8", f"Schritt, Phase {k + 1} von 8",
      f"Four-beat lateral-sequence walk at {k}/8 of the cycle; hooves on the ground: {', '.join(names[x] for x in sl)}.",
      f"Viertakt-Schritt in Seitenfolge bei {k}/8 des Zyklus; Klauen am Boden: {', '.join(dn[x] for x in sl)}.", b, st, th(metW, 0.8), key=True))
o, e = fore("f", 1.9, -GL + 0.2, LYING, rootL, -15, -8); err("lift", e, 0.01)
json.dump({k: {"angleDeg": round(v, 1)} for k, v in o.items()}, open(OUTF.replace("40-poses.json", "far-fore-lift.json"), "w"))
open(OUTF, "w").write('      "poses": [\n' + ",\n".join("        " + json.dumps(p, ensure_ascii=False) for p in P) + "\n      ],\n")
print(len(P), "poses")
