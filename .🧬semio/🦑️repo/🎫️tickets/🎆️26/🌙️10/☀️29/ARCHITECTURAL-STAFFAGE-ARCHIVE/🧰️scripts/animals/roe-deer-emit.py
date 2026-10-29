# [scratch] emits the roe-deer pose fragment from IK-solved joint angles and authored texts
import sys, json, math
A, F = sys.argv[1], sys.argv[2]
sys.path.insert(0, A)
import kin
a = kin.load(F, "roe-deer")
def chains(s): return [f"thigh-h{s}", f"shank-h{s}", f"cannon-h{s}", f"hoof-h{s}"], [f"scapula-f{s}", f"humerus-f{s}", f"forearm-f{s}", f"cannon-f{s}", f"hoof-f{s}"]
def limb(kind, s, x, y, base, root, m, p, sc=0, tol=None):
    h, f = chains(s)
    if kind == "h": o, e = a.solve_limb(base, h, f"contact-h{s}", (x, y), {h[0]:("solve",), h[1]:("solve",), h[2]:("abs",m), h[3]:("abs",p)}, root=root)
    else: o, e = a.solve_limb(base, f, f"contact-f{s}", (x, y), {f[0]:("off",sc), f[1]:("solve",), f[2]:("solve",), f[3]:("abs",m), f[4]:("abs",p)}, root=root)
    if tol is not None: assert e < tol, (kind, s, x, y, round(e, 4), o)
    return o, e
H0, HX, FX = 0.68, 0.0, 0.72
D_W, EXC_W, D_B, EXC_B = .65, .48, .55, .4
def static(base, root, gy=-H0, hx=HX, fx=FX, hm=-85, hp=-52, fm=-88, fp=-52, sc=0, stag=(-0.05, 0.04)):
    out = dict(base)
    for s in "nf":
        d = 0 if s == "n" else 1
        out.update(limb("h", s, hx + d*stag[0], gy, base, root, hm, hp, tol=0.004)[0]); out.update(limb("f", s, fx + d*stag[1], gy, base, root, fm, fp, sc=sc, tol=0.004)[0])
    return out
def foot(phase, D, exc, lift):
    if phase < D: return exc/2 - exc*phase/D, 0.0, 0.0, True
    s = (phase - D)/(1-D); return -exc/2 + exc*s, lift*math.sin(math.pi*s), math.sin(math.pi*s), False
def gait(offs, D, exc, lift, bob, H, hx=HX, fx=0.74, n=8, tilt=80, hsw=(-85, 45, -52, -50), fsw=(-88, -40, -52, -50), arch=0.0):
    res = []
    for k in range(n):
        b = {}; st = []
        for ft, off in offs.items():
            dx, rise, sw, stance = foot((k/n - off) % 1, D, exc, lift)
            if ft[0] == "h": o, e = limb("h", ft[1], hx+dx, -H+rise, {}, (0,0,0), hsw[0]+tilt*dx+hsw[1]*sw, hsw[2]+(hsw[1]+hsw[3])*sw)
            else: o, e = limb("f", ft[1], fx+dx, -H+rise, {}, (0,0,0), fsw[0]+tilt*dx+fsw[1]*sw, fsw[2]+(fsw[1]+fsw[3])*sw, sc=max(-20, min(20, dx*70)))
            assert e < (0.004 if stance else 0.02), (k, ft, round(e, 4))
            b.update(o)
            if stance: st.append("contact-" + ft)
        b["neck"] = bob*math.sin(2*math.pi*k/n); b["head"] = -0.6*bob*math.sin(2*math.pi*k/n)
        b["lumbar"] = arch*math.sin(4*math.pi*k/n)
        res.append((b, st))
    return res
def bj(b): return {k: {"angleDeg": round(v, 1)} for k, v in b.items() if abs(v) > 0.05}
def pose(id, en, de, den, dde, bones, contacts, thermal, root=None, key=False):
    p = {"id": id, "title": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "bones": bj(bones)}
    if root is not None: p["root"] = {"angleDeg": root}
    if key: p["key"] = True
    p.update({"ground": "auto", "contacts": contacts, "thermal": thermal, "loads": []}); return p
def th(met, bn, bh, mouth=None): return {"activityMet": met, "emitters": {"breath-nose": bn, "breath-heavy": bh}}
ALL = ["contact-hn", "contact-hf", "contact-fn", "contact-ff"]
P = []
def build():
    P.clear()
    alert = {"neck": -8, "head": -4, "ear-n": -24, "ear-f": -30}
    P.append(pose("alert", "Alert", "Wachsam", "Standing square on four hooves, neck raised, head carried level and both large ears turned forward to the source of a sound.", "Gerades Stehen auf vier Hufen, Hals aufgerichtet, Kopf waagerecht getragen und beide großen Lauscher nach vorn zur Geräuschquelle gedreht.", static(alert, (0,0,0)), ALL, th(1.2, 0.5, 0)))
    rootG = (0, 0, -7); gr = {"neck": -108, "head": 38, "jaw": -3, "ear-n": 14, "ear-f": 10, "lumbar": -4}
    P.append(pose("grazing", "Grazing", "Äsend", "Front end lowered by flexed forelegs and a rotated trunk, neck stretched forward and down so the muzzle hovers just above the ground vegetation; ears relaxed sideways.", "Vorderkörper durch gebeugte Vorderläufe und gedrehten Rumpf gesenkt, Hals nach vorn unten gestreckt, sodass der Äser dicht über der Bodenvegetation schwebt; Lauscher entspannt zur Seite.", static(gr, rootG), ALL, th(1.3, 0.5, 0), root=-7))
    ra = {"neck": -8, "head": -4, "ear-n": -24, "ear-f": -30}
    base = static(ra, (0,0,0))
    o, e = limb("f", "n", FX+0.13, -H0+0.2, ra, (0,0,0), -150, -95, sc=0); assert e < 0.01, e
    base.update(o)
    P.append(pose("alert-foreleg-raised", "Alert with raised foreleg", "Wachsam mit angehobenem Vorderlauf", "Frozen in alarm on three hooves: the near foreleg is lifted with the elbow bent and the knee folded so the hoof hangs clear of the ground, head high, ears forward.", "In Schreckstarre auf drei Hufen: der nahe Vorderlauf ist mit gebeugtem Ellbogen und eingeklappter Vorderfußwurzel angehoben, der Huf hängt frei über dem Boden, Kopf hoch, Lauscher nach vorn.", base, ["contact-hn", "contact-hf", "contact-ff"], th(1.4, 0.6, 0)))

    rootB = (0, 0, -8); bb = {"neck": -6, "head": 6, "ear-n": 12, "ear-f": 8, "lumbar": 14, "thorax": 4}
    ba = dict(bb)
    for sd in "nf":
        d = 0.0 if sd == "n" else 0.04
        ba.update(limb("f", sd, 0.78 + d, -0.64, bb, rootB, -92, -52, sc=-5, tol=0.01)[0])
        ba.update(limb("h", sd, 0.40 + d, -0.40, bb, rootB, -10, -60, tol=0.03)[0])
    P.append(pose("bounding-gathered", "Bounding, gathered", "Sprung, gesammelt", "Landing of a bound: the forelegs reach forward and take the weight, the back is arched and both hind legs are drawn forward under the belly with hock and stifle fully flexed, ready for the next push.", "Landung eines Sprungs: die Vorderläufe greifen nach vorn und tragen das Gewicht, der Rücken ist aufgewölbt und beide Hinterläufe sind mit stark gebeugtem Sprung- und Kniegelenk unter den Bauch gezogen, bereit zum nächsten Abdruck.", ba, ["contact-fn", "contact-ff"], th(6.0, 0.2, 1), root=-8))
    rootP = (0, 0, 8); pb = {"neck": -4, "head": 4, "ear-n": 22, "ear-f": 26, "lumbar": -10, "thorax": -3}
    pp = dict(pb)
    for sd in "nf":
        d = 0.0 if sd == "n" else 0.04
        pp.update(limb("h", sd, -0.05 + d, -0.62, pb, rootP, -80, -35, tol=0.02)[0])
        pp.update(limb("f", sd, 0.85 + d, -0.45, pb, rootP, -120, -95, sc=10, tol=0.03)[0])
    P.append(pose("bounding-pushoff", "Bounding, push-off", "Sprung, Abdruck", "Push-off of a bound: the hind legs extend and drive from the hooves, the trunk is pitched up, the back is stretched and the forelegs are folded forward with bent knees.", "Abdruck eines Sprungs: die Hinterläufe strecken sich und drücken von den Hufen ab, der Rumpf ist aufgerichtet, der Rücken gestreckt und die Vorderläufe sind mit gebeugten Vorderfußwurzeln nach vorn eingeklappt.", pp, ["contact-hn", "contact-hf"], th(6.0, 0.2, 1), root=8))
    D, EXC, HG = D_W, EXC_W, .65
    walk = gait({"hn":0, "fn":.25, "hf":.5, "ff":.75}, D, EXC, .08, 2, HG, arch=1.5)
    wd = ["near hind just touched down, far fore finishing its stance", "near hind and far hind-side pair loading, near fore swinging forward", "near fore touching down", "near fore loading, far hind swinging", "far hind touching down", "far hind loading, near hind lifting", "far fore touching down", "far fore loading, near hind swinging forward"]
    for k, (b, st) in enumerate(walk):
        P.append(pose(f"walk-phase-{k}", f"Walk, phase {k + 1} of 8", f"Schritt, Phase {k + 1} von 8", f"Lateral-sequence walk at {k}/8 of the cycle; stance hooves: {', '.join(x[8:] for x in st)}.", f"Schritt in Seitenfolge bei {k}/8 des Zyklus; Standhufe: {', '.join(x[8:] for x in st)}.", b, st, th(2.6, .8, 0), key=True))
    BD, BEXC, BH = D_B, EXC_B, .66
    bnd = gait({"hn":0, "hf":.03, "fn":.5, "ff":.53}, BD, BEXC, .2, 3, BH, hsw=(-85, 75, -52, -70), fsw=(-88, -50, -52, -60), arch=6)
    for k, (b, st) in enumerate(bnd):
        P.append(pose(f"bound-phase-{k}", f"Bound, phase {k + 1} of 8", f"Sprung, Phase {k + 1} von 8", f"Slow bounding with the hind pair and the fore pair striking alternately and overlapping; stance hooves: {', '.join(x[8:] for x in st)}.", f"Langsamer Sprung, Hinter- und Vorderlaufpaar setzen abwechselnd und überlappend auf; Standhufe: {', '.join(x[8:] for x in st)}.", b, st, th(6.0, .2, 1), key=True))

def T(en, de): return {"en": en, "de": de}
def cyc(T_, stance, offs):
    sp, ev = [], []
    for ft, off in offs.items():
        a0 = off*T_; a1 = a0 + stance*T_; c = "contact-" + ft
        if a1 <= T_ + 1e-9: sp.append({"anchor": c, "fromS": round(a0, 4), "toS": round(a1, 4)})
        else: sp += [{"anchor": c, "fromS": round(a0, 4), "toS": round(T_, 4)}, {"anchor": c, "fromS": 0, "toS": round(a1 - T_, 4)}]
        ev.append({"t": round(a0, 4), "kind": "contact-on", "title": T(f"{ft} hoof touches down", f"Huf {ft} setzt auf"), "target": c})
        ev.append({"t": round(a1 % T_, 4), "kind": "contact-off", "title": T(f"{ft} hoof lifts", f"Huf {ft} hebt ab"), "target": c})
    return sp, sorted(ev, key=lambda e: e["t"])
HOOF = {"hn": ("Near hind", "Nahe Hinter"), "hf": ("Far hind", "Ferne Hinter"), "fn": ("Near fore", "Nahe Vorder"), "ff": ("Far fore", "Ferne Vorder")}
def evs(ev):
    for e in ev:
        ft = e["target"][8:]; en, de = HOOF[ft]; on = e["kind"] == "contact-on"
        e["title"] = T(f"{en} hoof {'touches down' if on else 'lifts'}", f"{de}huf {'setzt auf' if on else 'hebt ab'}")
    return ev
def clips():
    C = []
    sp, ev = cyc(1.0, D_W, {"hn": 0, "fn": .25, "hf": .5, "ff": .75}); ev = evs(ev)
    ev += [{"t": 0, "kind": "inhale", "title": T("Inhalation", "Einatmen")}, {"t": 0.5, "kind": "exhale", "title": T("Exhalation", "Ausatmen")}]
    C.append({"id": "walk-cycle", "title": T("Walking, four-beat", "Schritt im Viertakt"), "description": T("Lateral-sequence walk after Hildebrand (1976): near hind, near fore, far hind, far fore touch down at quarter-cycle intervals; each hoof is on the ground 65 % of the cycle, so three or four hooves always carry. The head nods slightly with the stride.", "Schritt in Seitenfolge nach Hildebrand (1976): nahes Hinterbein, nahes Vorderbein, fernes Hinterbein, fernes Vorderbein setzen im Viertelzyklus auf; jeder Huf steht 65 % des Zyklus am Boden, sodass stets drei oder vier Hufe tragen. Der Kopf nickt leicht im Schritt."), "durationS": 1.0, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": round(EXC_W/D_W, 4),
        "keys": [{"t": round(k/8, 4), "pose": f"walk-phase-{k}", "ease": "smooth"} for k in range(8)], "contacts": sp,
        "phases": [{"id": f"{n}-strike", "fromS": i*0.25, "toS": (i+1)*0.25, "title": T(f"{en} strikes", f"{de} setzt auf")} for i, (n, en, de) in enumerate([("near-hind", "Near hind", "Nahes Hinterbein"), ("near-fore", "Near fore", "Nahes Vorderbein"), ("far-hind", "Far hind", "Fernes Hinterbein"), ("far-fore", "Far fore", "Fernes Vorderbein")])],
        "events": ev, "tracks": [{"target": "emitter:breath-nose", "ease": "smooth", "keys": [{"t": 0, "v": 0.7}, {"t": 0.5, "v": 0.9}, {"t": 1.0, "v": 0.7}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.6}, {"t": 0.5, "v": 2.7}, {"t": 1.0, "v": 2.6}]}]})
    TB = 0.9
    sp, ev = cyc(TB, D_B, {"hn": 0, "hf": .03, "fn": .5, "ff": .53}); ev = evs(ev)
    C.append({"id": "bound-cycle", "title": T("Slow bounding", "Langsamer Sprunglauf"), "description": T("A slow, stotting-like bound: the hind pair pushes and the fore pair lands in overlapping stances (55 % of the cycle each), so the body is always supported; the back arches and stretches and the head bobs. The flight phase of a fast roe bound is deliberately omitted.", "Ein langsamer Sprunglauf: das Hinterlaufpaar drückt ab und das Vorderlaufpaar landet in sich überlappenden Standphasen (je 55 % des Zyklus), der Körper ist also stets abgestützt; der Rücken wölbt und streckt sich und der Kopf wippt. Die Flugphase eines schnellen Rehsprungs ist bewusst ausgelassen."), "durationS": TB, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": 0.6775,
        "keys": [{"t": round(k*TB/8, 4), "pose": f"bound-phase-{k}", "ease": "smooth"} for k in range(8)], "contacts": sp,
        "phases": [{"id": "hind-drive", "fromS": 0, "toS": 0.45, "title": T("Hind pair drives", "Hinterlaufpaar drückt ab")}, {"id": "fore-landing", "fromS": 0.45, "toS": 0.9, "title": T("Fore pair lands and carries", "Vorderlaufpaar landet und trägt")}],
        "events": ev, "tracks": [{"target": "emitter:breath-heavy", "ease": "smooth", "keys": [{"t": 0, "v": 0.8}, {"t": 0.45, "v": 1}, {"t": 0.9, "v": 0.8}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 5.5}, {"t": 0.45, "v": 6.5}, {"t": 0.9, "v": 5.5}]}]})
    ALLC = [{"anchor": c, "fromS": 0, "toS": 7} for c in ("contact-hn", "contact-hf", "contact-ff")] + [{"anchor": "contact-fn", "fromS": 0, "toS": 3.7}]
    C.append({"id": "alarm-freeze", "title": T("Alarm freeze", "Schreckstarre"), "description": T("The roe grazes, then the head snaps up, the ears flick forward and back, the animal freezes and finally lifts the near foreleg and holds it motionless. The three other hooves stay planted throughout.", "Das Reh äst, dann ruckt der Kopf hoch, die Lauscher schnellen nach vorn und zurück, das Tier erstarrt und hebt schließlich den nahen Vorderlauf an und hält ihn reglos. Die anderen drei Hufe bleiben die ganze Zeit am Boden."), "durationS": 7, "fps": 30, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "grazing", "ease": "smooth"}, {"t": 1.5, "pose": "grazing", "ease": "smooth", "bones": {"jaw": {"angleDeg": -12}}}, {"t": 1.9, "pose": "grazing", "ease": "smooth"}, {"t": 2.25, "pose": "alert", "ease": "smooth", "bones": {"ear-n": {"angleDeg": -42}, "ear-f": {"angleDeg": -46}}},
                 {"t": 2.7, "pose": "alert", "ease": "smooth"}, {"t": 3.1, "pose": "alert", "ease": "smooth", "bones": {"ear-n": {"angleDeg": -4}, "ear-f": {"angleDeg": -30}}}, {"t": 3.5, "pose": "alert", "ease": "smooth"}, {"t": 3.6, "pose": "alert", "ease": "smooth"}, {"t": 4.4, "pose": "alert-foreleg-raised", "ease": "smooth"}, {"t": 7, "pose": "alert-foreleg-raised", "ease": "smooth"}],
        "contacts": ALLC,
        "phases": [{"id": "graze", "fromS": 0, "toS": 1.9, "title": T("Grazing", "Äsen")}, {"id": "head-snap", "fromS": 1.9, "toS": 2.7, "title": T("Head snaps up, ears forward", "Kopf ruckt hoch, Lauscher nach vorn")}, {"id": "freeze", "fromS": 2.7, "toS": 3.6, "title": T("Freezing, ear flick", "Erstarren, Lauscherzucken")}, {"id": "raise", "fromS": 3.6, "toS": 4.4, "title": T("Near foreleg lifts", "Naher Vorderlauf hebt an")}, {"id": "hold", "fromS": 4.4, "toS": 7, "title": T("Holding still", "Reglos verharren")}],
        "events": [{"t": 2.1, "kind": "motion", "title": T("Head snaps up", "Kopf ruckt hoch")}, {"t": 3.1, "kind": "motion", "title": T("Ear flick", "Lauscherzucken")}, {"t": 3.7, "kind": "contact-off", "title": T("Near fore hoof lifts", "Nahfuß-Vorderhuf hebt ab"), "target": "contact-fn"}, {"t": 2.25, "kind": "thermal", "title": T("Breath held, heart rate rises", "Atem angehalten, Herzfrequenz steigt")}],
        "tracks": [{"target": "emitter:breath-nose", "ease": "smooth", "keys": [{"t": 0, "v": 0.5}, {"t": 1.9, "v": 0.5}, {"t": 2.4, "v": 0.1}, {"t": 3.6, "v": 0.2}, {"t": 4.4, "v": 0.6}, {"t": 7, "v": 0.6}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.3}, {"t": 2.1, "v": 1.3}, {"t": 2.6, "v": 1.8}, {"t": 7, "v": 1.4}]}]})
    return C
if __name__ == "__main__" or True:
    build()
    open(A + "/roe-deer/40-poses.json", "w").write('      "poses": [\n' + ",\n".join("        " + json.dumps(p, ensure_ascii=False) for p in P) + "\n      ],\n")
    open(A + "/roe-deer/50-clips.json", "w").write('      "clips": [\n' + ",\n".join("        " + json.dumps(c, ensure_ascii=False) for c in clips()) + "\n      ],\n")
    print(len(P), "poses")
