# [scratch] emits the dog pose and clip fragments from IK-solved joint angles and authored texts
import sys, json, math
A, F = sys.argv[1], sys.argv[2]
sys.path.insert(0, A)
import kin
a = kin.load(F, "labrador-dog")
def chains(s):
    return [f"thigh-h{s}", f"shank-h{s}", f"metatarsus-h{s}", f"paw-h{s}"], [f"scapula-f{s}", f"humerus-f{s}", f"forearm-f{s}", f"metacarpus-f{s}", f"paw-f{s}"]
def limb(kind, s, x, y, base, root, m, p, sc=0):
    h, f = chains(s)
    if kind == "h":
        return a.solve_limb(base, h, f"contact-h{s}", (x, y), {h[0]:("solve",), h[1]:("solve",), h[2]:("abs",m), h[3]:("abs",p)}, root=root)
    return a.solve_limb(base, f, f"contact-f{s}", (x, y), {f[0]:("off",sc), f[1]:("solve",), f[2]:("solve",), f[3]:("abs",m), f[4]:("abs",p)}, root=root)
def static(base, root, hx, fx, gy, hm=(-85,-8), fm=(-85,-10), sc=0):
    out = dict(base)
    for s in "nf":
        out.update(limb("h", s, hx, gy, base, root, *hm)[0]); out.update(limb("f", s, fx, gy, base, root, *fm, sc=sc)[0])
    return out
def foot(phase, D, exc, lift):
    if phase < D: return exc/2 - exc*phase/D, 0.0, 0.0, True
    s = (phase - D)/(1-D); return -exc/2 + exc*s, lift*math.sin(math.pi*s), math.sin(math.pi*s), False
def gait(offs, D, exc, lift, jaw, bob, H=0.455, HX=0.02, FX=0.55):
    res = []
    for k in range(8):
        b = {}; st = []
        for ft, off in offs.items():
            dx, rise, sw, stance = foot((k/8 - off) % 1, D, exc, lift)
            if ft[0] == "h": o, e = limb("h", ft[1], HX+dx, -H+rise, {}, (0,0,0), -85+30*sw, -8-25*sw)
            else: o, e = limb("f", ft[1], FX+dx, -H+rise, {}, (0,0,0), -85+25*sw, -10-20*sw, sc=max(-20, min(20, dx*70)))
            assert e < (0.004 if stance else 0.02), (k, ft, e)
            b.update(o)
            if stance: st.append("contact-" + ft)
        b["jaw"] = jaw; b["head"] = bob*math.sin(2*math.pi*k/8)
        res.append((b, st))
    return res
def bj(b): return {k: {"angleDeg": round(v, 1)} for k, v in b.items() if abs(v) > 0.05}
def pose(id, en, de, den, dde, bones, contacts, thermal, root=None, loads=(), key=False):
    p = {"id": id, "title": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "bones": bj(bones)}
    if root is not None: p["root"] = {"angleDeg": root}
    if key: p["key"] = True
    p.update({"ground": "auto", "contacts": contacts, "thermal": thermal, "loads": list(loads)}); return p
def th(met, bn, sn, pm, tongue): return {"activityMet": met, "emitters": {"breath-nose": bn, "sniff": sn, "pant-mouth": pm}, "regions": {"tongue": {"exposedFraction": tongue}}}
ALL = ["contact-hn", "contact-hf", "contact-fn", "contact-ff"]
lead = {"id": "lead", "kind": "equipment", "title": {"en": "Lead tension", "de": "Leinenzug"}, "anchor": "collar", "fxN": 10, "fyN": 4, "source": "authored-estimate"}
P = []
P.append(pose("standing-alert", "Standing alert", "Aufmerksam stehend", "Standing square on four paws, ears raised, head up, tail carried level.", "Gerades Stehen auf vier Pfoten, Ohren aufgestellt, Kopf erhoben, Rute waagerecht getragen.", static({"ear-n":25,"ear-f":25,"tail-1":-12,"tail-2":-6,"head":3}, (0,0,0), 0.0, 0.55, -0.5), ALL, th(1.1, .5, 0, 0, .05)))
rootS = (0,0,43); bs = {"thorax":-10,"neck":-10,"head":-24,"ear-n":10,"ear-f":10,"tail-1":-50,"tail-2":-10}
rx, ry = a.anchor(a.fk(bs, rootS), "contact-rump"); sx = a.fk(bs, rootS)["scapula-fn"][4]
P.append(pose("sitting-waiting", "Sitting and waiting", "Sitzend wartend", "Sitting upright on the sitting bones with the hind paws flat in front of the haunches and the forelegs straight; the tail rests behind.", "Aufrechtes Sitzen auf den Sitzbeinhöckern, die Hinterpfoten flach vor der Hinterhand und die Vorderbeine gestreckt; die Rute ruht hinten.", static(bs, rootS, rx+0.22, sx+0.03, ry, hm=(-30,-4), fm=(-92,-10), sc=-18), ALL + ["contact-rump"], th(0.95, .4, 0, 0, .05), root=43))
rootN = (0,0,-8)
P.append(pose("sniffing-head-down", "Sniffing the ground", "Schnüffelnd am Boden", "Front lowered by flexing the forelegs, neck and head pointing down so the nose hovers a hand above the ground; ears pricked forward.", "Vorderkörper durch gebeugte Vorderbeine gesenkt, Hals und Kopf nach unten, die Nase schwebt eine Handbreit über dem Boden; Ohren nach vorn gerichtet.", static({"neck":-50,"head":-45,"ear-n":30,"ear-f":30,"tail-1":-8,"thorax":-6}, rootN, 0.0, 0.52, -0.5), ALL, th(1.3, .3, 1, 0, .05), root=-8))
walk = gait({"hn":0, "fn":.25, "hf":.5, "ff":.75}, .65, .34, .06, -6, 2)
trot = gait({"hn":0, "ff":0, "hf":.5, "fn":.5}, .55, .36, .08, -22, 1.5)
tw = lambda k: ["near hind just after touchdown", "near hind and far fore mid-stance, near fore swinging", "near fore touching down", "near fore early stance, far hind lifting", "far hind touching down", "far hind early stance, near hind lifting", "far fore touching down", "far fore early stance, far hind lifting"][k]
for k, (b, st) in enumerate(walk):
    P.append(pose(f"walk-phase-{k}", f"Walk, phase {k + 1} of 8", f"Schritt, Phase {k + 1} von 8", f"Lateral-sequence walk at {k}/8 of the cycle; stance paws: {', '.join(s[8:] for s in st)}.", f"Schritt in Seitenfolge bei {k}/8 des Zyklus; Standpfoten: {', '.join(s[8:] for s in st)}.", b, st, th(2.0, .8, 0, 0, .3), key=True))
for k, (b, st) in enumerate(trot):
    P.append(pose(f"trot-phase-{k}", f"Trot, phase {k + 1} of 8", f"Trab, Phase {k + 1} von 8", f"Trot at {k}/8 of the cycle with diagonal pairs moving together; stance paws: {', '.join(s[8:] for s in st)}.", f"Trab bei {k}/8 des Zyklus, diagonale Paare bewegen sich gemeinsam; Standpfoten: {', '.join(s[8:] for s in st)}.", b, st, th(3.6, .3, 0, 1, 1.0), loads=[lead], key=True))
open(A + "/labrador-dog/40-poses.json", "w").write('      "poses": [\n' + ",\n".join("        " + json.dumps(p, ensure_ascii=False) for p in P) + "\n      ],\n")
print(len(P), "poses")
