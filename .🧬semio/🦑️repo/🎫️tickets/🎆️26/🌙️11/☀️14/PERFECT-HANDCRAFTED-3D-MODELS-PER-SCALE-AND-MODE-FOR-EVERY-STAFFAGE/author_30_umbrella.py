import sys
from author_30_lib import *

CAT, SLUG = "humans", "person-with-umbrella"
PAD500 = 0.0
PROPS = ("umbrella", "rib-front", "rib-back")
CANOPY = [(0.5405, 0.5232), (0.5539, 0.5232), (0.5715, 0.5105), (0.6215, 0.4793), (0.6733, 0.4297), (0.7163, 0.3695), (0.7498, 0.3013), (0.7735, 0.2276), (0.7868, 0.1511), (0.7891, 0.0744), (0.7895, 0.04)]


def L(x, y):
    return [r5(y - 1.2068), r5(-(x - 0.3)), 0.0]


def canopy_revolve(curve, bore, mat, sid="canopy"):
    prof = [[u, r] for u, r in curve[:-1]] + [[curve[-1][0], bore], [curve[0][0], bore]]
    return revolve(sid, "umbrella", mat, 0.3, "body", prof, sectionFaces=["-u"])


CURVES = {
    50: CANOPY,
    100: [(0.5405, 0.5232), (0.5539, 0.5232), (0.5715, 0.5105), (0.6733, 0.4297), (0.7498, 0.3013), (0.7868, 0.1511), (0.7895, 0.04)],
    200: [(0.5525, 0.5232), (0.5715, 0.5105), (0.6215, 0.4793), (0.7163, 0.3695), (0.7868, 0.1511), (0.7895, 0.045)],
    500: [(0.5525, 0.5232), (0.5715, 0.5105), (0.7, 0.4), (0.7716, 0.2), (0.7895, 0.06)],
}


def handle_chain(scale, r=0.0165):
    cx, cy, a, b = 0.3265, 1.1168, 0.0265, 0.0483
    step = {50: 20, 100: 28, 200: 55}[scale]
    pts = [(0.3, 1.2368), (0.3, cy)] + [(cx + a * math.cos(math.radians(t)), cy + b * math.sin(math.radians(t))) for t in range(180, 351, step)]
    return [capsule(f"handle-{i + 1}", "umbrella", "umbrella-handle", 0.18, "body", L(*pts[i]), L(*pts[i + 1]), r) for i in range(len(pts) - 1)]


def props(scale):
    if scale == 50:
        return props50()
    mm, mf = "umbrella-metal", "umbrella-fabric"
    out = [canopy_revolve(CURVES[scale], 0.0105, mf)]
    out.append(capsule("shaft", "umbrella", mm, 0.2, "body", [0.03, 0, 0], [0.83 if scale <= 100 else 0.87, 0, 0], {100: 0.009, 200: 0.006, 500: 0.0375}[scale], **({"slender": True} if scale <= 200 else {})))
    if scale <= 200:
        out += handle_chain(scale, 0.017 if scale == 100 else 0.0165)
    if scale <= 100:
        out.append(capsule("ferrule", "umbrella", mm, 0.1, "detail", [0.8, 0, 0], [0.884, 0, 0], 0.008, 0.00525))
        out.append(capsule("hub", "umbrella", mm, 0.3, "detail", [0.756, 0, 0], [0.806, 0, 0], 0.0225, lateralScale=0.55))
    for n in ("front", "back"):
        out.append(capsule(f"rib-{n}", f"rib-{n}", mm, 0.1, "detail", [0, 0, 0], [0.56, 0, 0], 0.0025 if scale <= 200 else 0.0125, slender=True))
    return out


def props50():
    out = []
    mm, mh, mf = "umbrella-metal", "umbrella-handle", "umbrella-fabric"
    out.append(capsule("shaft", "umbrella", mm, 0.2, "body", [0.03, 0, 0], [0.83, 0, 0], 0.0088, slender=True))
    cx, cy, a, b = 0.3265, 1.1168, 0.0265, 0.0483
    pts = [(0.3, 1.2368), (0.3, cy)] + [(cx + a * math.cos(math.radians(t)), cy + b * math.sin(math.radians(t))) for t in range(180, 351, 20)]
    for i in range(len(pts) - 1):
        out.append(capsule(f"handle-{i + 1}", "umbrella", mh, 0.18, "body", L(*pts[i]), L(*pts[i + 1]), 0.0165))
    out.append(capsule("ferrule", "umbrella", mm, 0.1, "detail", [0.8, 0, 0], [0.884, 0, 0], 0.008, 0.00525))
    out.append(capsule("hub", "umbrella", mm, 0.3, "detail", [0.756, 0, 0], [0.806, 0, 0], 0.0225, lateralScale=0.55))
    out.append(canopy_revolve(CANOPY, 0.0105, mf))
    for n in ("front", "back"):
        out.append(capsule(f"rib-{n}", f"rib-{n}", mm, 0.1, "detail", [0, 0, 0], [0.56, 0, 0], 0.0025, slender=True))
    return out


DROP = {"thigh-l-coat", "thigh-r-coat", "foot-l", "foot-r", "toe-l", "toe-r", "thigh-l", "thigh-r", "thigh-l-leg-clothing", "thigh-r-leg-clothing", "coat-hem-l", "coat-hem-r", "trouser-hem-l", "trouser-hem-r", "cuff-l", "cuff-r", "fingers-l-0", "fingers-l-1", "fingers-l-2", "fingers-r-0", "fingers-r-1", "fingers-r-2"}
MAXN = {"thigh": {50: 8, 100: 5, 200: 4, 500: 3}, "coat": {50: 5, 100: 4, 200: 3, 500: 2}}


def thighs(P, scale):
    ft = FEATURE[str(scale)]
    out = []
    for s in ("l", "r"):
        b = f"thigh-{s}"
        if scale == 500:
            rows = rows_of(P, b, inc_el=[b], inc_w=[f"sil-leg-{s}"])
            out.append(loft_rows(b, b, "cotton-clothing", 0.25, "body", rows, ft * 0.5, 3, minr=0.55 * ft))
            continue
        rows = rows_of(P, b, inc_el=[f"thigh-skin-{s}"], inc_w=[b])
        out.append(loft_rows(b, b, "cotton-clothing", 0.25, "body", rows, ft * 0.5, MAXN["thigh"][scale], minr=0.55 * ft))
        rows = rows_of(P, b, inc_el=[b], inc_w=[b], wpad=0.0135 if scale <= 100 else 0.0, trim=0.0005)
        out.append(loft_rows(f"{b}-coat", b, "rain-shell", 0.62, "body", rows, ft * 0.5, MAXN["coat"][scale], minr=0.55 * ft))
    return out


def cuffs(body, scale):
    ft = FEATURE[str(scale)]
    ln = max(0.045, 1.3 * ft)
    out = []
    for s in ("l", "r"):
        f = next(x for x in body if x["id"] == f"forearm-{s}")
        L_ = f["sections"][-1]["at"][0]
        sc = f["sections"][-1]
        pad = 0.004 if scale <= 100 else 0.006
        mk = lambda u: {"at": [r5(u), sc["at"][1], sc["at"][2]], "rx": r5(sc["rx"] + pad), "ry": r5(sc["ry"] + pad), "exponent": 2.3, "rotation": Q_U}
        out.append(open_loft(f"cuff-{s}", f"forearm-{s}", "cotton-clothing", 0.8, "detail", [mk(L_ - ln - 0.005), mk(L_ - 0.005)], cap_start=True, cap_end=True))
    return out


def trunk500(P):
    ft = FEATURE["500"]
    rows = rows_of(P, "torso", inc_el=["figure"], inc_w=["sil-torso"], uspan=[-0.16, 0.4495], wclip=0.19)
    out = [loft_rows("torso", "torso", "rain-shell", 0.62, "body", rows, ft * 0.07, 8, minr=0.55 * ft)]
    for sd in "lr":
        for bone, mat, tone, nmax in ((f"upper-arm-{sd}", "rain-shell", 0.62, 3), (f"forearm-{sd}", "rain-shell", 0.62, 3), (f"shank-{sd}", "cotton-clothing", 0.25, 3)):
            rows = rows_of(P, bone, inc_w=[f"sil-shank-{sd}"] if bone.startswith("shank") else None, trim=0.002)
            out.append(loft_rows(bone, bone, mat, tone, "body", rows, ft * 0.15, nmax, minr=0.55 * ft))
        out.append(ellipsoid(f"shoulder-{sd}", f"upper-arm-{sd}", "rain-shell", 0.62, "body", [0.0, 0.0, 0.0], [0.06, 0.06, 0.06]))
    out.append(ellipsoid("pelvis", "pelvis", "cotton-clothing", 0.25, "body", [0.013, 0.0, 0.0], [0.085, 0.13, 0.2063]))
    rows = rows_of(P, "head", inc_el=["head"], inc_w=["head"], wview="front")
    out.append(loft_rows("head", "head", "skin", 0.8, "body", rows, ft * 0.15, 5, minr=0.55 * ft, e=2.0))
    return out


def build(scale):
    d = load(CAT, SLUG)["volumes"][str(scale)]
    P = Prof30(SLUG, scale)
    keep = [s for s in d["body"] if s["bone"] not in PROPS and s["id"] not in DROP]
    if scale == 500:
        keep = trunk500(P)
    ids = {s["id"] for s in keep}
    shoes = shoe_pair("leather-shoe", 0.22, scale)
    out = keep + shoes + thighs(P, scale) + cuffs(keep, scale)
    if scale == 500:
        out = [inflate(x, PAD500) if x["role"] == "body" and not x.get("slender") and x["id"] not in ("shaft",) else x for x in out]
    return out + props(scale)


def regmap(body, scale):
    ids = {b["id"] for b in body}
    N, O = "nest", "outer"
    m = {
        "head-skin": [(i, O) for i in ("head", "neck", "torso-head-skin", "eye-l", "eye-r", "ear-l", "ear-r", "nose", "mouth", "brow-l", "brow-r")],
        "hair": [("head-hair", O)],
        "coat": [(i, N if i in ("torso", "upper-arm-l", "upper-arm-r", "forearm-l", "forearm-r", "thigh-l-coat", "thigh-r-coat") else O) for i in ("torso", "shoulder-l", "shoulder-r", "upper-arm-l", "upper-arm-r", "forearm-l", "forearm-r", "thigh-l-coat", "thigh-r-coat", "head-coat", "coat-belt", "coat-front-seam")],
        "arm-clothing": [("cuff-l", O), ("cuff-r", O)],
        "hand-skin": [(i, O) for i in ("hand-l", "hand-r", "thumb-l", "thumb-r")],
        "leg-clothing": [(i, N) for i in ("thigh-l", "thigh-r", "shank-l", "shank-r")],
        "canopy": [("canopy", O)],
        "umbrella-frame": [(i, O) for i in sorted(ids) if i.startswith(("shaft", "handle", "hub", "ferrule", "rib-"))],
    }
    if scale > 50:
        m["shoes"] = [(i, O) for i in ("foot-l", "foot-r", "toe-l", "toe-r")]
        m["leg-clothing"].append(("pelvis", O))
    return {r: [(i, md) for i, md in lst if i in ids] for r, lst in m.items()}


def collar(scale):
    ft = FEATURE[str(scale)]
    r = max(0.057, ft * 0.6)
    L_ = max(0.058, ft * 1.3)
    return {"id": "th-shirt-collar", "bone": "head", "material": "cotton-clothing", "role": "body", "tone": 0.85, "kind": "loft", "sections": [{"at": [0.0, -0.002, 0.0], "rx": r5(r), "ry": r5(r + 0.003), "exponent": 2.3, "rotation": Q_U}, {"at": [r5(L_), -0.004, 0.0], "rx": r5(r + 0.001), "ry": r5(r + 0.005), "exponent": 2.3, "rotation": Q_U}], "capStart": True, "capEnd": True, "region": "torso-clothing", **({"layer": "collar"} if scale == 50 else {})}


def hair500():
    h = ellipsoid("th-head-hair", "head", "hair", 0.2, "body", [0.165, -0.002, 0.0], [0.105, 0.108, 0.093])
    h["region"] = "hair"
    return h


def pelvis_under(body):
    p = next(x for x in body if x["id"] == "pelvis")
    out = clamp_feature(shrink(p, 0.03), FEATURE["50"])
    out.update(id="th-pelvis-trousers", region="leg-clothing", layer="trousers", material="cotton-clothing")
    return out


def thermal(a, body, scale):
    rm = regmap(body, scale)
    extra = [collar(scale)] + emitters(a, scale, region_default="head-skin")
    if scale == 500:
        rm["hair"] = []
        extra.append(hair500())
        for sd in "lr":
            h = ellipsoid(f"th-hand-{sd}", f"hand-{sd}", "skin", 0.8, "body", [0.09, 0.0, 0.0], [0.09, 0.0375, 0.0375])
            h["region"] = "hand-skin"
            extra.append(h)
    if scale == 50:
        rm["leg-clothing"] = [x for x in rm["leg-clothing"] if x[0] != "pelvis"]
        extra += [pelvis_under(body)] + nested_real(a, 50, "shoes", shoe_pair("leather-shoe", 0.22, 50, clip=0.1124, prefix="s-"))
    return thermal_from_map(a, scale, body, rm, extra)


def structure(a, body, scale):
    mem = member_bones(CAT, SLUG, scale)
    main = {f"thigh-{x}": [f"thigh-{x}"] for x in "lr"}
    jb = {50: [b for b in mem if b.startswith(("upper-arm", "forearm", "thigh", "shank"))] + ["torso", "head"], 100: [b for b in mem if b.startswith(("upper-arm", "forearm", "thigh", "shank"))]}.get(scale, [])
    ms = {50: ["torso", "pelvis", "thigh-l", "thigh-r"], 100: ["torso", "pelvis", "thigh-l", "thigh-r"], 200: ["torso", "thigh-l", "thigh-r"], 500: ["torso"]}[scale]
    out = person_structure(a, scale, body, mem, PROPS, main, joints=jb, masses=[m for m in ms if m in mem or True], kk={"torso": 0.5, "head": 0.5})
    for b in body:
        sid = b["id"]
        if sid == "shaft" or sid.startswith("handle-") or sid.startswith("rib-"):
            out.append({"id": "member-" + sid, "of": sid, "member": b["bone"], "part": "member"})
    return out


DEC = {
    50: [
        ("Body at 1:50: every limb, the torso, the head and the shoes are lofts whose sections follow this scale's own elevation, front and plan; the thigh is split into the trouser loft and a wider raincoat skirt loft that ends at the drawn hem, the shoes are stepped lofts with the ankle shaft, the cuffs are short rings that stand 4 mm proud of the sleeve. The umbrella is built by hand: a revolved canopy with a bored axis, a rim hem band and the fabric dome of the drawn profile, a slender steel shaft, a J-shaped handle of ten capsules following the drawn hook, hub, ferrule and the two drawn ribs as slender rods.", "Körper bei 1:50: Gliedmassen, Rumpf, Kopf und Schuhe sind Lofts, deren Schnitte den eigenen Seiten-, Vorder- und Draufsichten dieses Massstabs folgen; der Oberschenkel besteht aus dem Hosen-Loft und einem weiteren Regenmantel-Rockloft, der am gezeichneten Saum endet, die Schuhe sind abgestufte Lofts mit Knöchelschaft, die Manschetten sind kurze Ringe, die 4 mm über dem Ärmel stehen. Der Schirm ist von Hand gebaut: ein Rotationskörper mit durchbohrter Achse, einem Saumband am Rand und der Stoffkuppel des gezeichneten Profils, ein schlanker Stahlschaft, ein J-Griff aus zehn Kapseln nach dem gezeichneten Haken, Nabe, Zwinge und die beiden gezeichneten Rippen als schlanke Stäbe."),
        ("Thermal at 1:50: the multi-layer regions (coat, trousers, shoes) are nested shells, one per layer, shrunk inwards by the thickness of the layers outside; the outer layer references the body solid. The raincoat skirt buries the trouser thigh down to the drawn hem, so trousers expose only the lower thigh and the shank; the hip trousers sit 30 mm inside the coat; the shoes are the foot and toe lofts cut at the trouser hem. The canopy dome carries a section face on its flat underside (the back of a one-sided patch). The shirt collar and the cuffs are real thin rings. Emitters are small solids tagged with their id at their anchors.", "Thermik bei 1:50: die mehrschichtigen Regionen (Mantel, Hose, Schuhe) sind ineinander liegende Schalen, eine je Schicht, um die Dicke der aussenliegenden Schichten nach innen verkleinert; die äusserste Schicht verweist auf den Körper. Der Mantelrock begräbt den Hosen-Oberschenkel bis zum gezeichneten Saum, die Hose zeigt also nur den unteren Oberschenkel und den Unterschenkel; die Hüfthose liegt 30 mm im Mantel; die Schuhe sind Fuss- und Zehen-Loft, am Hosensaum abgeschnitten. Die Schirmkuppel trägt an der flachen Unterseite eine Schnittfläche (Rückseite eines einseitigen Flecks). Hemdkragen und Manschetten sind echte dünne Ringe. Emitter sind kleine Körper mit ihrer Id an ihren Ankern."),
        ("Structure at 1:50: a member core (about half the limb section) along every load-bearing bone, joint solids at the shoulders, elbows, hips, knees, neck and waist, mass solids at the trunk, pelvis and thighs, and the umbrella shaft, handle chain and ribs as members by reference; the load path of the wind on the canopy runs through the ribs, the shaft and the right arm into the trunk and the feet.", "Tragwerk bei 1:50: ein Stabkern (etwa halber Gliedquerschnitt) entlang jedes tragenden Knochens, Gelenkkörper an Schultern, Ellbogen, Hüften, Knien, Hals und Taille, Massenkörper an Rumpf, Becken und Oberschenkeln, dazu Schirmschaft, Griffkette und Rippen als Stäbe per Verweis; der Lastpfad des Winds auf dem Schirm läuft über die Rippen, den Schaft und den rechten Arm in Rumpf und Füsse."),
    ],
    100: [
        ("Body at 1:100: the person is regenerated from this scale's own drawings with fewer sections (at most eight per limb), the shoes are four-station lofts, the thigh keeps the trouser and the raincoat skirt loft, the cuffs are short rings. The umbrella is a revolved canopy with seven profile points and a rim band, a slender shaft, the J handle in eight capsules, hub, ferrule and the two ribs as slender rods; the drawn panel seams and rim lines are texture and not modelled.", "Körper bei 1:100: die Person wird aus den eigenen Zeichnungen dieses Massstabs mit weniger Schnitten neu erzeugt (höchstens acht je Glied), die Schuhe sind Lofts aus vier Stationen, der Oberschenkel behält Hosen- und Mantelrock-Loft, die Manschetten sind kurze Ringe. Der Schirm ist ein Rotationskörper mit sieben Profilpunkten und einem Randband, ein schlanker Schaft, der J-Griff aus acht Kapseln, Nabe, Zwinge und die beiden Rippen als schlanke Stäbe; die gezeichneten Bahnnähte und Randlinien sind Textur und nicht modelliert."),
        ("Thermal at 1:100: one surface solid per part referencing the body (no layers below 1:50); the skirt still buries the trouser thigh so the per-region exposed areas follow the drawing; hip trousers, shirt collar and shoes as at 1:50 without nested shells.", "Thermik bei 1:100: ein Flächenkörper je Teil mit Verweis auf den Körper (keine Schichten unterhalb 1:50); der Rock begräbt weiter den Hosen-Oberschenkel, damit die freien Flächen je Region der Zeichnung folgen; Hüfthose, Hemdkragen und Schuhe wie bei 1:50 ohne geschachtelte Schalen."),
        ("Structure at 1:100: members along every load-bearing bone, joints at the limb joints, masses at trunk, pelvis and thighs; umbrella shaft, handle and ribs by reference.", "Tragwerk bei 1:100: Stäbe entlang jedes tragenden Knochens, Gelenke an den Gliedergelenken, Massen an Rumpf, Becken und Oberschenkeln; Schirmschaft, Griff und Rippen per Verweis."),
    ],
    200: [
        ("Body at 1:200: merged masses from this scale's drawings: torso, pelvis and head as lofts of three to six sections, the thigh as trouser loft plus a skirt loft that is wider only sagittally, shoes as three-station lofts; hips are added to the front and the plan drawing because the 1:200 drawings omitted them. The umbrella is a six-point revolved canopy, a slender 12 mm shaft drawn as a stroke, a five-capsule handle and the two ribs.", "Körper bei 1:200: zusammengefasste Massen aus den Zeichnungen dieses Massstabs: Rumpf, Becken und Kopf als Lofts aus drei bis sechs Schnitten, der Oberschenkel als Hosen-Loft plus Rockloft, der nur sagittal breiter ist, Schuhe als Lofts aus drei Stationen; die Hüfte wurde in Vorder- und Draufsicht ergänzt, weil die 1:200-Zeichnungen sie ausliessen. Der Schirm ist eine Kuppel aus sechs Profilpunkten, ein schlanker 12-mm-Schaft als Strich gezeichnet, ein Griff aus fünf Kapseln und die beiden Rippen."),
        ("Thermal at 1:200: every region has a surface solid referencing the body or a small real ring (collar, cuffs); only the sum of the exposed areas is compared at this scale.", "Thermik bei 1:200: jede Region hat einen Flächenkörper mit Verweis auf den Körper oder einen kleinen echten Ring (Kragen, Manschetten); bei diesem Massstab wird nur die Summe der freien Flächen verglichen."),
        ("Structure at 1:200: a member per load-bearing bone, joints dropped, masses at trunk and thighs, umbrella shaft, handle and ribs by reference.", "Tragwerk bei 1:200: ein Stab je tragenden Knochen, Gelenke entfallen, Massen an Rumpf und Oberschenkeln, Schirmschaft, Griff und Rippen per Verweis."),
    ],
    500: [
        ("Body at 1:500: the torso (with the pelvis merged in it), the head, the arms, the thighs and the shanks are lofts of two to eight sections regenerated from this scale's drawings, the pelvis is an ellipsoid, the shoes are two-station lofts whose toe is 76 mm high (the smallest feature). The 1:500 drawings were repaired: the torso outline lost its arm blocks and overlapping subpaths, the head and the hips were added. The umbrella is a four-point revolved canopy and a 75 mm shaft, the ribs are 25 mm rods.", "Körper bei 1:500: Rumpf (mit eingeschmolzenem Becken), Kopf, Arme, Oberschenkel und Unterschenkel sind Lofts aus zwei bis acht Schnitten, aus den Zeichnungen dieses Massstabs neu erzeugt, das Becken ist ein Ellipsoid, die Schuhe sind Lofts aus zwei Stationen, deren Zehe 76 mm hoch ist (das kleinste Merkmal). Die 1:500-Zeichnungen wurden repariert: die Rumpfkontur verlor ihre Armblöcke und überlappenden Teilpfade, Kopf und Hüfte wurden ergänzt. Der Schirm ist eine Kuppel aus vier Profilpunkten und ein 75-mm-Schaft, die Rippen sind 25-mm-Stäbe."),
        ("Thermal at 1:500: every region has a solid: surface solids referencing the body, and real solids where the scale draws no body part (hair cap, hands, shirt collar, cuffs); only the total exposed area is compared.", "Thermik bei 1:500: jede Region hat einen Körper: Flächenkörper mit Verweis auf den Körper und echte Körper, wo der Massstab kein Körperteil zeichnet (Haarkappe, Hände, Hemdkragen, Manschetten); verglichen wird nur die Gesamtfläche."),
        ("Structure at 1:500: members along every load-bearing bone, a mass at the trunk, umbrella shaft and ribs by reference.", "Tragwerk bei 1:500: Stäbe entlang jedes tragenden Knochens, eine Masse am Rumpf, Schirmschaft und Rippen per Verweis."),
    ],
}


def decisions(sc):
    return [{"en": e, "de": d} for e, d in DEC[sc]]


def cmd_list(kind, sc):
    a = load_asset(CAT, SLUG)
    d = load(CAT, SLUG)["volumes"][str(sc)]
    if kind == "thermal":
        save_lists(CAT, SLUG, sc, thermal=thermal(a, d["body"], sc))
    else:
        save_lists(CAT, SLUG, sc, structure=structure(a, d["body"], sc))


if __name__ == "__main__":
    cmd, sc = sys.argv[1], int(sys.argv[2])
    if cmd == "body":
        save_lists(CAT, SLUG, sc, body=build(sc))
    elif cmd in ("thermal", "structure"):
        cmd_list(cmd, sc)
    elif cmd == "decisions":
        save_lists(CAT, SLUG, sc, decisions=decisions(sc))
    elif cmd == "all":
        save_lists(CAT, SLUG, sc, body=build(sc), decisions=decisions(sc))
        cmd_list("thermal", sc)
        cmd_list("structure", sc)
    print(fails(CAT, SLUG, sc))
