# [scratch] emits the pigeon pose and clip fragments from IK-solved joint angles and authored texts
import sys, json, math
A, F = sys.argv[1], sys.argv[2]
sys.path.insert(0, A)
import kin
a = kin.load(F, "pigeon")
R = lambda v: round(v, 1)
LEG = lambda s: [f"thigh-{s}", f"shank-{s}", f"tarsus-{s}", f"toes-{s}"]
def leg(s, x, y, base, tars=-80, toes=-3, root=(0, 0, 0), tol=0.003):
    c = LEG(s)
    o, e = a.solve_limb(base, c, f"contact-{s}", (x, y), {c[0]: ("solve",), c[1]: ("solve",), c[2]: ("abs", tars), c[3]: ("abs", toes)}, root=root)
    assert e < tol, (s, x, y, tars, toes, e)
    return o
def wrap(bid, v):
    lo, hi = a.limits(bid)
    for k in (-720, -360, 0, 360, 720):
        if lo - 1e-6 <= v + k <= hi + 1e-6: return v + k
    raise ValueError((bid, v))
def absang(base, chain, targets, root=(0, 0, 0)):
    w = a.fk(base, root); prev = kin.ang(w[a.bones[chain[0]]["parent"]]); o = {}
    for bid, t in zip(chain, targets):
        o[bid] = wrap(bid, t - prev - a.bones[bid]["angleDeg"]); prev = t
    return o
WING = lambda s: [f"humerus-{s}", f"forearm-{s}", f"hand-{s}"]
NECK = ["neck-1", "neck-2", "neck-3", "head"]
LEGS_ABS = lambda s: LEG(s)
FOLD = (192, 25, 188)
def wings(base, near, far=None, root=(0, 0, 0)):
    o = dict(base); far = far or tuple(v + d for v, d in zip(near, (4, 4, 6)))
    o.update(absang(base, WING("n"), near, root)); o.update(absang(base, WING("f"), far, root)); return o
def bj(b): return {k: {"angleDeg": R(v)} for k, v in b.items() if abs(v) > 0.05}
def pose(id, en, de, den, dde, bones, contacts, thermal, root=None, ground="auto", key=False):
    p = {"id": id, "title": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "bones": bj(bones)}
    if root is not None: p["root"] = {k: round(v, 4) for k, v in root.items()}
    if key: p["key"] = True
    p.update({"ground": ground, "contacts": contacts, "thermal": thermal, "loads": []}); return p
def th(met, bn=0.6, be=0.0, wing=0.45): return {"activityMet": met, "emitters": {"breath-nostril": bn, "breath-exertion": be}, "regions": {"plumage-wings": {"exposedFraction": wing}}}
BOTH = ["contact-n", "contact-f"]
def legs2(base, nx, fx, y, tars=-80, toes=-3, root=(0, 0, 0), sx=0.0):
    o = dict(base); o.update(leg("n", nx, y, base, tars, toes, root)); o.update(leg("f", fx, y, base, tars, toes, root)); return o
def ground_y(base, root=(0, 0, 0)):
    w = a.fk(base, root); return -min(a.anchor(w, "contact-n")[1], a.anchor(w, "contact-f")[1])
P = []; CL = []
w0 = a.fk({}); EYE0 = a.anchor(w0, "eye")
# standing
b = wings({}, FOLD, (196, 29, 194)); b = legs2(b, 0.03, 0.05, -0.095)
P.append(pose("standing-alert", "Standing alert", "Aufmerksam stehend", "Standing on both feet, chest tilted up, S-curved neck with the head up, wings folded.", "Stehen auf beiden Füßen, Brust angehoben, S-förmig gebogener Hals mit erhobenem Kopf, Flügel angelegt.", b, BOTH, th(1.0)))
# wings half open
b = wings({"neck-2": 6, "head": 5}, (135, 70, 150), (141, 76, 158)); b = legs2(b, 0.03, 0.05, -0.095)
P.append(pose("wings-half-open", "Wings half open", "Flügel halb geöffnet", "Standing with both wings lifted and the elbows spread, the wrists above the back and the primaries trailing, as when stretching or warming up.", "Stehend mit beiden angehobenen Flügeln und gespreizten Ellbogen, die Handgelenke über dem Rücken und die Handschwingen nach hinten, wie beim Strecken oder Aufwärmen.", b, BOTH, th(1.25, 0.9, 0, 1.0)))
# pecking
def neckpose(trunk, n, hd, extra=None):
    base = {"trunk": trunk}; base.update(absang(base, NECK, n + (hd,))); base.update(extra or {}); return base
def tipy(b): return a.anchor(a.fk(b), "beak-tip")[1]
pk = {}
for pid, trunk, nk, hd, H, key, en, de, den, dde in (
    ("pecking-reach", -15, (20, 0, -20), -60, None, True, "Pecking, reaching", "Picken, Vorstrecken", "Body pitched forward and neck stretched down, the beak above the ground.", "Körper nach vorn geneigt und Hals nach unten gestreckt, der Schnabel über dem Boden."),
    ("pecking-strike", -32, (12, -15, -40), -75, 0.0, False, "Pecking, strike", "Picken, Zustoßen", "Beak tip on the ground: the trunk is pitched forward, the neck is folded down and the head points at the ground.", "Schnabelspitze am Boden: der Rumpf ist nach vorn geneigt, der Hals nach unten gefaltet und der Kopf zeigt zum Boden.")):
    b = neckpose(trunk, nk, hd); b = wings(b, FOLD, (196, 29, 194))
    H = 0.004 - tipy(b) if pid.endswith("strike") else 0.075 - tipy(b)
    b = legs2(b, 0.03, 0.04, -H, -55, -3)
    pk[pid] = H
    P.append(pose(pid, en, de, den, dde, b, BOTH, th(1.3, 0.8), key=key))
# walk
T, D, EXC, LIFT, N = 0.5, 0.62, 0.09, 0.016, 12
HW, FX = 0.09, 0.035
def foot(phase):
    if phase < D: return EXC / 2 - EXC * phase / D, 0.0, 0.0, max(0.0, (phase / D - 0.75) / 0.25), True
    s = (phase - D) / (1 - D); return -EXC / 2 + EXC * s, LIFT * math.sin(math.pi * s), math.sin(math.pi * s), 0.0, False
V = EXC / D / T
hold = 2 / 3; TS = T / 2; AMP = V * hold * TS / 2
def bob(k):
    ps = (k / N * 2) % 1
    d = AMP - 2 * AMP * ps / hold if ps < hold else -AMP + 2 * AMP * (lambda u: u * u * (3 - 2 * u))((ps - hold) / (1 - hold))
    return d
WALKING = []
for k in range(N):
    b = wings({}, FOLD, (196, 29, 194)); st = []
    for s, off in (("n", 0.0), ("f", 0.5)):
        dx, rise, sw, heel, stance = foot((k / N - off) % 1)
        b.update(leg(s, FX + dx, -HW + rise, b, -80 + 8 * heel + 18 * sw, -3 - 30 * sw, tol=0.003 if stance else 0.012))
        if stance: st.append(f"contact-{s}")
    d = bob(k) - AMP
    c = ["neck-1", "neck-2"]
    o, e = a.solve_limb(b, NECK, "eye", (EYE0[0] + d, EYE0[1] + 0.002 * math.cos(math.pi * 2 * k / N * 2)), {"neck-1": ("solve",), "neck-2": ("solve",), "neck-3": ("off", 0), "head": ("abs", -10)})
    assert e < 0.002, (k, e)
    b.update(o); b["tail"] = 3 * math.sin(2 * math.pi * k / N * 2)
    WALKING.append((b, st))
    P.append(pose(f"walk-phase-{k}", f"Walk, phase {k + 1} of {N}", f"Gehen, Phase {k + 1} von {N}", f"Walking at {k}/{N} of the stride; stance feet: {', '.join(x[8:] for x in st)}; the head is {'held back against the advancing body' if bob(k) < AMP and (k / N * 2) % 1 < hold else 'thrust forward'}.", f"Gehen bei {k}/{N} des Doppelschritts; Standfüße: {', '.join(x[8:] for x in st)}; der Kopf wird {'gegen den vorrückenden Körper zurückgehalten' if (k / N * 2) % 1 < hold else 'nach vorn gestoßen'}.", b, st, th(1.8, 0.8), key=True))
# take-off
XF = 0.0
def stage(id, trunk, necka, hd, wing, H, rx, tars, toes, en, de, den, dde, met, be, wexp, key=False):
    b = {"trunk": trunk}; b.update(absang(b, NECK, necka + (hd,))); b = wings(b, wing)
    b = legs2(b, XF - rx, XF - rx + 0.02, -H, tars, toes)
    P.append(pose(id, en, de, den, dde, b, BOTH, th(met, 0.3, be, wexp), root={"x": rx}, key=key)); return b
stage("takeoff-crouch", -10, (55, 95, 50), -10, (150, 40, 160), 0.07, -0.035, -60, -3, "Take-off crouch", "Abflug, Ducken", "Legs flexed, the body lowered and tilted forward, the wings lifting from the body; both feet planted.", "Gebeugte Beine, der Körper gesenkt und nach vorn geneigt, die Flügel heben sich vom Körper; beide Füße aufgesetzt.", 2.5, 0.5, 0.7)
stage("takeoff-wings-up", -8, (55, 95, 50), -10, (92, 88, 92), 0.075, -0.03, -62, -3, "Take-off, wings up", "Abflug, Flügel oben", "Top of the preparatory upstroke: both wings raised above the back while the legs stay flexed.", "Höhepunkt des vorbereitenden Aufschlags: beide Flügel über dem Rücken, die Beine bleiben gebeugt.", 4, 0.8, 1.0, True)
stage("takeoff-push", 0, (48, 80, 45), -8, (196, 194, 192), 0.105, 0.0, -85, -10, "Take-off, push-off", "Abflug, Abstoßen", "Mid downstroke: the wings sweep back and down while the legs extend and push the body up and forward.", "Mitte des Abschlags: die Flügel schwingen nach hinten und unten, während die Beine strecken und den Körper nach oben und vorn drücken.", 12, 1.0, 1.0, True)
bl = stage("takeoff-liftoff", 10, (44, 70, 35), -8, (268, 272, 276), 0.102, 0.025, -100, -40, "Take-off, lift-off", "Abflug, Abheben", "End of the first downstroke with fully extended legs on the toes at the instant the feet leave the ground.", "Ende des ersten Abschlags mit voll gestreckten Beinen auf den Zehen im Augenblick des Abhebens.", 12, 1.0, 1.0)
P[-1]["root"]["y"] = round(ground_y(bl), 4)
def air(id, y, x, wing, en, de, den, dde, key=False):
    root = (0, 0, 12)
    b = {}; b.update(absang(b, NECK, (40, 70, 25, -15), root)); b = wings(b, wing, root=root)
    for s in "nf": b.update(absang(b, LEG(s), (-40, -160, -135, -105), root))
    P.append(pose(id, en, de, den, dde, b, [], th(12, 0.1, 1.0, 1.0), root={"x": x, "y": y, "angleDeg": 12}, ground="authored", key=key))
air("airborne-wings-up", 0.17, 0.17, (92, 88, 92), "Airborne, wings up", "Flug, Flügel oben", "Climbing flight at the top of the upstroke, the legs trailing under the tail and the neck stretched forward.", "Steigflug am Ende des Aufschlags, die Beine unter dem Schwanz nachgezogen und der Hals nach vorn gestreckt.")
air("airborne-wings-down", 0.2, 0.33, (268, 272, 276), "Airborne, wings down", "Flug, Flügel unten", "Climbing flight at the end of the downstroke, the wings pointing down and slightly forward.", "Steigflug am Ende des Abschlags, die Flügel zeigen nach unten und leicht nach vorn.")
air("airborne-climb", 0.235, 0.52, (96, 92, 96), "Airborne, climbing", "Flug, Steigen", "Second upstroke, higher above the ground, the body pitched up.", "Zweiter Aufschlag, höher über dem Boden, der Körper nach oben geneigt.", True)
open(A + "/pigeon/40-poses.json", "w").write('      "poses": [\n' + ",\n".join("        " + json.dumps(p, ensure_ascii=False) for p in P) + "\n      ],\n")
print(len(P), "poses; peck H", pk, "liftoff y", P[[p["id"] for p in P].index("takeoff-liftoff")]["root"])
# clips
X = lambda en, de: {"en": en, "de": de}
def ev(t, kind, en, de, target=None):
    d = {"t": t, "kind": kind, "title": X(en, de)}
    if target: d["target"] = target
    return d
ks = lambda *l: [{"t": t, "pose": p, "ease": "smooth"} for t, p in l]
tr = lambda target, *kv: {"target": target, "ease": "smooth", "keys": [{"t": t, "v": v} for t, v in kv]}
ADVANCE = round(EXC / D, 4)
dn, up = D * T, T
CL.append({"id": "walk-cycle", "title": X("Walking with head-bob", "Gehen mit Kopfnicken"), "description": X("Two-beat walk with alternating feet: each foot is on the ground 62 % of the stride so both feet share the load briefly twice per stride. The head is held still in space while the body passes under it and then thrust forward during each step (hold and thrust, Frost 1978).", "Zweitakt-Gang mit wechselnden Füßen: jeder Fuß steht 62 % des Doppelschritts am Boden, sodass beide Füße pro Doppelschritt zweimal kurz gemeinsam tragen. Der Kopf bleibt im Raum ruhig, während der Körper darunter vorbeizieht, und wird dann bei jedem Schritt nach vorn gestoßen (Halten und Stoßen, Frost 1978)."),
  "durationS": T, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": ADVANCE,
  "keys": ks(*[(round(k * T / N, 5), f"walk-phase-{k}") for k in range(N)]),
  "contacts": [{"anchor": "contact-n", "fromS": 0, "toS": round(dn, 4)}, {"anchor": "contact-f", "fromS": round(T / 2, 4), "toS": T}, {"anchor": "contact-f", "fromS": 0, "toS": round(dn - T / 2, 4)}],
  "phases": [{"id": "double-support-a", "fromS": 0, "toS": round(dn - T / 2, 4), "title": X("Both feet on the ground", "Beide Füße am Boden")}, {"id": "near-stance", "fromS": round(dn - T / 2, 4), "toS": round(T / 2, 4), "title": X("Near foot carries, far foot swings", "Naher Fuß trägt, ferner Fuß schwingt")}, {"id": "double-support-b", "fromS": round(T / 2, 4), "toS": round(dn, 4), "title": X("Both feet on the ground", "Beide Füße am Boden")}, {"id": "far-stance", "fromS": round(dn, 4), "toS": T, "title": X("Far foot carries, near foot swings", "Ferner Fuß trägt, naher Fuß schwingt")}],
  "events": [ev(0, "contact-on", "Near foot touches down", "Naher Fuß setzt auf", "contact-n"), ev(round(dn, 4), "contact-off", "Near foot lifts", "Naher Fuß hebt ab", "contact-n"), ev(round(T / 2, 4), "contact-on", "Far foot touches down", "Ferner Fuß setzt auf", "contact-f"), ev(round(dn - T / 2, 4), "contact-off", "Far foot lifts", "Ferner Fuß hebt ab", "contact-f"), ev(round(hold * TS, 4), "motion", "Head thrust forward", "Kopf wird nach vorn gestoßen"), ev(round(TS + hold * TS, 4), "motion", "Head thrust forward", "Kopf wird nach vorn gestoßen")],
  "tracks": [tr("emitter:breath-nostril", (0, 0.8), (T / 2, 0.9), (T, 0.8))]})
CL.append({"id": "peck-cycle", "title": X("Pecking food", "Futter picken"), "description": X("Standing on both feet the pigeon lowers the body and stretches the neck down twice, strikes the ground with the beak and straightens up again; the feet do not move.", "Auf beiden Füßen stehend senkt die Taube den Körper und streckt den Hals zweimal nach unten, stößt mit dem Schnabel auf den Boden und richtet sich wieder auf; die Füße bewegen sich nicht."),
  "durationS": 2.0, "fps": 30, "loop": True, "rootMotion": "none",
  "keys": ks((0, "standing-alert"), (0.4, "pecking-reach"), (0.5, "pecking-strike"), (0.62, "pecking-reach"), (0.72, "pecking-strike"), (0.84, "pecking-reach"), (1.3, "standing-alert")),
  "contacts": [{"anchor": "contact-n", "fromS": 0, "toS": 2.0}, {"anchor": "contact-f", "fromS": 0, "toS": 2.0}],
  "phases": [{"id": "look", "fromS": 0, "toS": 0.4, "title": X("Lowering the head", "Kopf senken")}, {"id": "pecks", "fromS": 0.4, "toS": 0.84, "title": X("Two pecks", "Zwei Picks")}, {"id": "rise", "fromS": 0.84, "toS": 2.0, "title": X("Straightening up and scanning", "Aufrichten und Umschauen")}],
  "events": [ev(0, "contact-on", "Near foot on the ground", "Naher Fuß am Boden", "contact-n"), ev(0, "contact-on", "Far foot on the ground", "Ferner Fuß am Boden", "contact-f"), ev(0.5, "motion", "Beak strikes the ground", "Schnabel trifft den Boden"), ev(0.72, "motion", "Beak strikes the ground", "Schnabel trifft den Boden"), ev(0, "inhale", "Inhalation", "Einatmen"), ev(1.1, "exhale", "Exhalation", "Ausatmen")],
  "tracks": [tr("activityMet", (0, 1.0), (0.4, 1.3), (0.84, 1.3), (1.3, 1.0), (2.0, 1.0)), tr("emitter:breath-nostril", (0, 0.6), (0.5, 0.9), (1.3, 0.6), (2.0, 0.6))]})
TL = 0.36
CL.append({"id": "take-off", "title": X("Take-off into flight", "Abflug in den Flug"), "description": X("From the crouch the wings rise and beat down while the legs extend and push; the feet leave the ground at 0.36 s and three wing positions follow in climbing flight (about 0.36 s without ground contact). Wing beats are slowed to about 4 Hz for legibility; real take-off beats are faster.", "Aus dem Ducken heben sich die Flügel und schlagen nach unten, während die Beine strecken und abstoßen; die Füße verlassen den Boden bei 0,36 s, danach folgen drei Flügelstellungen im Steigflug (etwa 0,36 s ohne Bodenkontakt). Die Flügelschläge sind zur besseren Lesbarkeit auf etwa 4 Hz verlangsamt; echte Abflugschläge sind schneller."),
  "durationS": 0.72, "fps": 30, "loop": False, "rootMotion": "none",
  "keys": ks((0, "takeoff-crouch"), (0.22, "takeoff-wings-up"), (0.3, "takeoff-push"), (TL, "takeoff-liftoff"), (0.48, "airborne-wings-up"), (0.6, "airborne-wings-down"), (0.72, "airborne-climb")),
  "contacts": [{"anchor": "contact-n", "fromS": 0, "toS": TL}, {"anchor": "contact-f", "fromS": 0, "toS": TL}],
  "phases": [{"id": "crouch", "fromS": 0, "toS": 0.22, "title": X("Crouch and wings up", "Ducken und Flügel heben")}, {"id": "push-off", "fromS": 0.22, "toS": TL, "title": X("Downstroke and push-off", "Abschlag und Abstoßen")}, {"id": "flight", "fromS": TL, "toS": 0.72, "ballistic": True, "title": X("Climbing flight", "Steigflug")}],
  "events": [ev(0, "contact-on", "Near foot on the ground", "Naher Fuß am Boden", "contact-n"), ev(0, "contact-on", "Far foot on the ground", "Ferner Fuß am Boden", "contact-f"), ev(TL, "contact-off", "Near foot leaves the ground", "Naher Fuß verlässt den Boden", "contact-n"), ev(TL, "contact-off", "Far foot leaves the ground", "Ferner Fuß verlässt den Boden", "contact-f"), ev(0.22, "motion", "Downstroke begins", "Abschlag beginnt"), ev(0.3, "thermal", "Flight metabolism starts", "Flugstoffwechsel beginnt")],
  "tracks": [tr("activityMet", (0, 2.5), (0.22, 4), (0.3, 12), (0.72, 12)), tr("emitter:breath-exertion", (0, 0.5), (0.22, 0.8), (0.3, 1), (0.72, 1)), tr("emitter:breath-nostril", (0, 0.3), (0.3, 0.1), (0.72, 0.1)), tr("region:plumage-wings:exposedFraction", (0, 0.7), (0.22, 1), (0.72, 1))]})
open(A + "/pigeon/50-clips.json", "w").write('      "clips": [\n' + ",\n".join("        " + json.dumps(c, ensure_ascii=False) for c in CL) + "\n      ],\n")
