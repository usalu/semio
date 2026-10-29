# [scratch] emits the horse pose fragment from IK-solved joint angles and authored texts; prints clip timing numbers
import sys, json, math
exec(open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/animals/horse-ik.py").read())
R0 = (0, 0, 0)
def stand(base, root, hx, fx, gy, sc=0):
    out = dict(base)
    for s in "nf":
        for fn, x in ((hind, hx), (fore, fx)):
            o, e = fn(s, x, gy, base, root); assert e < 0.004, (s, fn.__name__, e); out.update(o)
    return out
def foot(phase, D, exc, lift):
    if phase < D: return exc/2 - exc*phase/D, 0.0, 0.0, True
    s = (phase - D)/(1-D); return -exc/2 + exc*s, lift*math.sin(math.pi*s), math.sin(math.pi*s), False
D, EXC, LIFT, HG, T, N = 0.64, 0.80, 0.09, 1.28, 0.95, 8
HXG, FXG = 0.043, 1.08
OFFS = {"hn": 0.0, "fn": 0.25, "hf": 0.5, "ff": 0.75}
def gait(nod, ears):
    res = []
    for k in range(N):
        b = {}; st = []
        for ft, off in OFFS.items():
            dx, rise, sw, stance = foot((k/N - off) % 1, D, EXC, LIFT)
            if ft[0] == "h":
                o, e = hind(ft[1], HXG+dx, -HG+rise, {}, R0, (-80+16*dx+35*sw, -58-25*sw, -50-25*sw))
            else:
                o, e = fore(ft[1], FXG+dx, -HG+rise, {}, R0, (-90+15*dx-60*sw, -55-30*sw, -52-30*sw), sc=-9+32*dx)
            assert e < (0.004 if stance else 0.02), (k, ft, round(e, 4))
            b.update(o)
            if stance: st.append("contact-" + ft)
        ph = 2*math.pi*k/N
        b["neck-1"] = nod*math.sin(ph - 0.6); b["neck-2"] = -0.5*nod*math.sin(ph - 0.6); b["head"] = 0.6*nod*math.sin(ph + 1.0)
        b["ear-n"] = ears*math.sin(ph); b["ear-f"] = -ears*math.sin(ph)
        b["tail-1"] = 4*math.sin(ph + 1.5); b["tail-2"] = 6*math.sin(ph + 1.0); b["tail-3"] = 7*math.sin(ph + 0.6); b["tail-4"] = 7*math.sin(ph)
        b["jaw"] = -1
        res.append((b, st))
    return res
def pose(id, en, de, den, dde, bones, contacts, thermal, root=None, loads=(), key=False):
    p = {"id": id, "key": key, "title": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "bones": bj(bones)}
    if root is not None: p["root"] = {"angleDeg": root}
    if not key: del p["key"]
    p.update({"ground": "auto", "contacts": contacts, "thermal": thermal, "loads": list(loads)}); return p
def th(met, w, bn, sn, eyes=1.0):
    return {"activityMet": met, "emitters": {"breath-nose": bn, "snort": sn}, "regions": {"coat-trunk": {"wettedness": w}, "coat-neck": {"wettedness": w}, "coat-limbs": {"wettedness": w}, "eyes": {"exposedFraction": eyes}}}
ALL = ["contact-hn", "contact-hf", "contact-fn", "contact-ff"]
lead = {"id": "lead", "kind": "equipment", "title": {"en": "Lead rope tension", "de": "Strickzug"}, "anchor": "halter-ring", "fxN": 15, "fyN": -6, "source": "authored-estimate"}
P = []
alert = {"neck-1": 6, "neck-2": 4, "neck-3": 6, "head": 16, "ear-n": -16, "ear-f": -10, "tail-1": -6, "tail-2": -4, "tail-3": 2, "tail-4": 2, "jaw": 0}
P.append(pose("standing-alert", "Standing alert", "Aufmerksam stehend", "Standing square on four hooves, neck raised, head high and ears pricked forward, tail carried away from the hindquarters.", "Gerades Stehen auf vier Hufen, Hals aufgerichtet, Kopf hoch und Ohren nach vorn gestellt, der Schweif wird vom Hinterteil abgehoben getragen.", stand(alert, R0, HX, FX, -H0), ALL, th(1.7, 0, 0.7, 0.4)))
rest = {"neck-1": -32, "neck-2": -4, "neck-3": -6, "head": -18, "ear-n": 38, "ear-f": 32, "tail-1": 4, "tail-2": 2, "jaw": -2}
rb = stand(rest, R0, HX, FX, -H0); del rb["thigh-hn"], rb["shank-hn"], rb["cannon-hn"], rb["pastern-hn"], rb["hoof-hn"]
ox, oy = toe_offset(-72)
tx = HX - 0.12
o, e = hind("n", tx + ox, -H0 + 0.012 + oy, rest, R0, (-93, -78, -72)); print("rest leg err", round(e, 4), file=sys.stderr); rb.update(o)
P.append(pose("standing-rest", "Resting on three legs", "Dösend auf drei Beinen", "Relaxed stand with the head carried low, ears turned sideways and the near hind leg rested: stifle and hock flexed, the hoof tipped on its toe just behind the standing leg and carrying no weight.", "Entspanntes Stehen mit tief getragenem Kopf, seitlich gedrehten Ohren und entlastetem nahen Hinterbein: Knie und Sprunggelenk gebeugt, der Huf ruht auf der Zehenspitze knapp hinter dem Standbein und trägt kein Gewicht.", rb, ["contact-hf", "contact-fn", "contact-ff"], th(1.5, 0, 0.5, 0, 0.55)))
gz = {"neck-1": -105, "neck-2": -5, "neck-3": 5, "head": 65, "ear-n": 12, "ear-f": 18, "tail-1": 6, "tail-2": 3, "jaw": -3}
RG = (0, 0, -3)
gb = dict(gz)
for s, fx in (("n", FX + 0.24), ("f", FX - 0.06)):
    o, e = fore(s, fx, -H0, gz, RG); assert e < 0.004, e; gb.update(o)
for s in "nf":
    o, e = hind(s, HX, -H0, gz, RG); assert e < 0.004, e; gb.update(o)
P.append(pose("grazing-head-down", "Grazing, head down", "Grasen mit gesenktem Kopf", "Grazing with the neck stretched steeply down and the muzzle just above the sward; the near foreleg steps forward of the far one and the body pitches slightly forward.", "Grasen mit steil abgesenktem Hals und dem Maul knapp über der Grasnarbe; das nahe Vorderbein steht vor dem fernen und der Rumpf neigt sich leicht nach vorn.", gb, ALL, th(1.9, 0, 0.6, 0), root=-3))
walk = gait(3.5, 6)
tw = ["near hind just after touchdown", "near hind mid-stance, near fore swinging", "near fore touching down", "near fore early stance, far hind lifting", "far hind touching down", "far hind early stance, near hind lifting", "far fore touching down", "far fore early stance, far hind lifting"]
for k, (b, st) in enumerate(walk):
    nm = ", ".join(s[8:] for s in st)
    P.append(pose(f"walk-phase-{k}", f"Walk, phase {k + 1} of {N}", f"Schritt, Phase {k + 1} von {N}", f"Four-beat lateral-sequence walk at {k}/{N} of the cycle; stance hooves: {nm}.", f"Viertakt-Schritt in Seitenfolge bei {k}/{N} des Zyklus; Standhufe: {nm}.", b, st, th(4.2, 0.08, 1.0, 0), loads=[lead], key=True))
open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/animals/horse/40-poses.json", "w").write('      "poses": [\n' + ",\n".join("        " + json.dumps(p, ensure_ascii=False) for p in P) + "\n      ],\n")
print(len(P), "poses")
for ft, off in OFFS.items():
    on, offt = off*T, off*T + D*T
    print(ft, "stance", round(on, 4), round(offt, 4), "wrap" if offt > T else "")
print("exc", EXC, "advance expected", EXC/D)
