"""🌡️ Round 3 volumes of humans/child-on-kick-scooter: the body of every scale with its thermal model in space (nested layer shells at 1:50, emitter outlets on the surface) and its structure (the child's skeleton, the scooter's real frame, joints, supports, masses), plus the bilingual decisions.

`build()` returns the `volumes` member; `groups()` returns, per scale, which body solids each 2D shape group is drawn from.
"""
import copy, importlib.util, math, os

T = os.path.dirname(os.path.abspath(__file__))


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-child-on-kick-scooter-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


R = mod("ref")
K = mod("kit")
B50 = mod("body50")
C = mod("coarse")
BONE = "skin-child"
SIDES = ((1, "l"), (-1, "r"))
LAYERS = {"jacket": [("jacket-air", 0.005), ("jacket-tshirt", 0.004), ("jacket-jacket", 0.0)], "leggings": [("leggings-air", 0.002), ("leggings-knit", 0.0)], "sneakers": [("sneakers-air", 0.004), ("sneakers-sock", 0.002), ("sneakers-upper", 0.0)], "helmet": [("helmet-air", 0.025), ("helmet-eps", 0.0)], "hair": [("hair-pelt", 0.0)]}


def bodies(den):
    return B50.build() if den == 50 else getattr(C, f"body{den}")()


def clean(s):
    return {k: v for k, v in s.items() if not k.startswith("_")}


def emitters(den):
    """💨 The outlets (D2): nostril and mouth patches on the face, the plume patch on the crown of the helmet, never smaller than the smallest feature of the scale."""
    h = {50: 0.004, 100: 0.0076, 200: 0.0152, 500: 0.038}[den]
    nose = K.ellipsoid("th-emitter-breath-nose", "head", [h, h, max(h, 0.006)], [-0.021, -0.103 + h, 0], "skin-child", 0.5, role="detail", tags=["breath-nose"])
    mouth = K.ellipsoid("th-emitter-breath-mouth", "head", [h, h, max(h, 0.008)], [-0.036 - (h - 0.004), -0.097 + h, 0], "skin-child", 0.5, role="detail", tags=["breath-mouth"])
    top = {50: [(0.070, 0.102, 0.085, 2.0), (0.092, 0.072, 0.060, 2.0), (0.104, 0.040, 0.034, 2.0), (0.111, 0.010, 0.009, 2.0)], 100: [(0.066, 0.104, 0.086, 2.0), (0.094, 0.066, 0.055, 2.0), (0.112, 0.012, 0.011, 2.0)], 200: [(0.066, 0.104, 0.086, 2.0), (0.098, 0.060, 0.050, 2.0), (0.115, 0.014, 0.012, 2.0)], 500: [(0.036, 0.118, 0.094, 2.0), (0.085, 0.080, 0.066, 2.0), (0.113, 0.016, 0.014, 2.0)]}[den]
    plume = K.loft("th-emitter-plume", "head", C.helmet(top), "helmet-eps", 0.5, role="detail", tags=["plume"])
    nose["region"] = mouth["region"] = "face-skin"
    plume["region"] = "jacket"
    return [nose, mouth, plume]


def face_patch(head):
    """😶 The skin of the face as a one-sided patch: the head sections below the helmet rim, cut at the ear line; the scalp behind and above belongs to the hair and helmet regions, so the back and the top of the patch are section faces."""
    rows = []
    for q in head["sections"]:
        u, vc = q["at"][0], q["at"][1]
        if u > 0.036:
            continue
        front, back = q["ry"] - vc, q["ry"] + vc
        rows.append((u, front, min(back, 0.015), q["rx"], q.get("exponent", 2.0)))
    patch = K.stack("th-head", "head", rows, "skin-child", 0.85)
    patch["region"] = "face-skin"
    patch["sectionFaces"] = ["+v", "+u"]
    return patch


def thermal(den, body):
    out = []
    for s in body:
        region = s["_region"]
        layers = LAYERS.get(region)
        if s["id"] == "head":
            out.append(face_patch(s))
            continue
        if den != 50 or layers is None:
            out.append({"id": f"th-{s['id']}", "of": s["id"], "region": region})
            continue
        out.append({"id": f"th-{s['id']}", "of": s["id"], "region": region, "layer": layers[-1][0]})
        if s.get("texture") or s["role"] != "body" or s["kind"] not in ("loft", "ellipsoid", "capsule", "box"):
            continue
        for layer, inset in layers[:-1]:
            inner = K.shrink(clean(s), inset, f"th-{s['id']}-{layer}")
            inner["region"] = region
            inner["layer"] = layer
            out.append(inner)
    if den == 500:
        hair = K.stack("th-hair", "head", [(-0.050, -0.030, 0.062, 0.046), (-0.020, -0.024, 0.090, 0.060), (0.020, -0.020, 0.100, 0.066), (0.045, -0.020, 0.098, 0.066)], "hair-blond", 0.3)
        hair["region"] = "hair"
        out.append(hair)
    return out + emitters(den)


def foot_tube(i, bone, pts, side):
    a = math.radians(R.FOOT_DIR)
    q = K.quat(K.W, [math.sin(a), math.cos(a), 0.0])
    secs = []
    for x, y, rv, rw in pts:
        u, v = R.foot_local(x, y)
        secs.append(K.section([u, v, 0.0], rw, rv, 2.0, q))
    return K.loft(i, bone, secs, BONE, 0.92)


def skeleton(den):
    """🦴 The child's skeleton as members: skull and jaw, cervical spine, thoracolumbar spine with its curvature, ribcage, scapulae, clavicles (one shoulder-girdle bar from 1:200), pelvis, the long bones with head, shaft and condyles, hands and feet simplified by scale; no shaft is thinner than the smallest feature of its scale."""
    fine = den <= 100
    mid = den <= 200
    k = {50: 1.0, 100: 1.0, 200: 1.25, 500: 1.6}[den]
    fl = {50: 0.0, 100: 0.0076, 200: 0.0152, 500: 0.038}[den]
    o = []

    def t(x):
        return max(x * k, fl)

    def e(x):
        return max(x, fl)

    def m(s, member, part="member"):
        s["member"] = member
        s["part"] = part
        o.append(s)

    m(K.ellipsoid("skull", "head", [0.066, 0.089, 0.067], [0.056, -0.004, 0], BONE, 0.92), "head")
    if mid:
        m(K.stack("jaw", "head", [(-0.070, 0.070, -0.048, 0.014), (-0.052, 0.086, -0.012, 0.038), (-0.026, 0.090, 0.010, 0.046), (0.000, 0.086, 0.020, 0.050)], BONE, 0.92), "head")
    m(K.tube("spine-cervical", "neck", [([-0.012, 0.004, 0], t(0.012), t(0.012)), ([0.045, -0.010 if mid else 0.0, 0], t(0.011), t(0.011)), ([0.098, -0.002, 0], t(0.012), t(0.012))], BONE, 0.92), "neck")
    spine = [(-0.012, 0.034), (0.050, 0.020), (0.120, 0.032), (0.200, 0.050), (0.285, 0.046), (0.352, 0.018)] if mid else [(-0.012, 0.026), (0.120, 0.030), (0.240, 0.042), (0.352, 0.018)]
    m(K.tube("spine", "torso", [([u, v, 0], t(0.015), t(0.014)) for u, v in spine], BONE, 0.92), "torso")
    m(K.stack("ribcage", "torso", [(0.085, 0.050, 0.052, 0.070), (0.130, 0.066, 0.064, 0.090), (0.200, 0.068, 0.070, 0.098), (0.265, 0.056, 0.070, 0.092), (0.305, 0.036, 0.056, 0.066), (0.328, 0.018, 0.038, 0.034)] if mid else [(0.090, 0.050, 0.052, 0.072), (0.200, 0.068, 0.070, 0.098), (0.290, 0.046, 0.064, 0.080), (0.328, 0.026, 0.044, 0.044)], BONE, 0.92), "torso")
    m(K.stack("pelvis-bone", "pelvis", [(-0.045, 0.028, 0.040, 0.046), (-0.010, 0.046, 0.056, 0.086), (0.030, 0.044, 0.062, 0.100), (0.060, 0.022, 0.058, 0.084)] if mid else [(-0.045, 0.036, 0.042, 0.050), (0.010, 0.046, 0.060, 0.096), (0.060, 0.030, 0.058, 0.084)], BONE, 0.92), "pelvis")
    if den == 200:
        m(K.capsule("shoulder-girdle", "torso", [0.318, 0.010, -0.118], [0.318, 0.010, 0.118], fl, fl, BONE, 0.92), "torso")
    for s, n in SIDES:
        if fine:
            m(K.ellipsoid(f"scapula-{n}", "torso", [0.046, e(0.0065), 0.034], [0.262, 0.074, s * 0.072], BONE, 0.92, rotation=K.rot_w(8)), "torso")
            m(K.capsule(f"clavicle-{n}", "torso", [0.316, -0.048, s * 0.014], [0.322, 0.004, s * 0.122], e(0.006), e(0.006), BONE, 0.92), "torso")
        m(K.tube(f"humerus-{n}", f"arm-{n}", [([0.0, 0, 0], e(0.016), e(0.016)), ([0.030, 0, 0], t(0.010), t(0.010)), ([0.190, 0, 0], t(0.009), t(0.009)), ([0.2286, 0, 0], e(0.018), t(0.012))], BONE, 0.92), f"arm-{n}")
        if fine:
            m(K.capsule(f"radius-{n}", f"forearm-{n}", [0.004, 0.002, -s * 0.010], [0.1794, 0.0, -s * 0.011], e(0.0058), e(0.0075), BONE, 0.92), f"forearm-{n}")
            m(K.capsule(f"ulna-{n}", f"forearm-{n}", [-0.012, -0.006, s * 0.008], [0.176, 0.0, s * 0.012], e(0.008), e(0.0052), BONE, 0.92), f"forearm-{n}")
        else:
            m(K.capsule(f"radius-ulna-{n}", f"forearm-{n}", [0.0, 0, 0], [0.1794, 0, 0], t(0.011), t(0.010), BONE, 0.92), f"forearm-{n}")
        if mid:
            m(K.limb(f"hand-bones-{n}", f"hand-{n}", [(-0.004, 0.001, 0.0, t(0.0065), 0.019), (0.030, 0.001, 0.0, t(0.006), 0.026), (0.066, 0.0, 0.0, t(0.0055), 0.027)], BONE, 0.92, s), f"hand-{n}")
        else:
            m(K.capsule(f"hand-bones-{n}", f"hand-{n}", [0.0, -0.002, 0.0], [0.062, -0.016, 0.0], fl, fl, BONE, 0.92), f"hand-{n}")
        if den == 50:
            for j, (w, r, end) in enumerate(B50.FINGERS):
                f = B50.finger(f"phalanges-{n}{j}", f"hand-{n}", -s * w, 0.0042, end, BONE, 0.92)
                cu, cv = R.GRIP_LOCAL
                for q in f["sections"]:
                    a = math.atan2(q["at"][0] - cu, q["at"][1] - cv)
                    rr = R.GRIP_R + r + 0.0004
                    q["at"] = K.vec([cu + rr * math.sin(a), cv + rr * math.cos(a), q["at"][2]])
                f["role"] = "body"
                m(f, f"hand-{n}")
        elif den == 100:
            m(C.grip_roll(f"phalanges-{n}", f"hand-{n}", -0.027, 0.025, 0.0076, 165, BONE, 0.92), f"hand-{n}")
        m(K.tube(f"femur-{n}", f"thigh-{n}", [([0.0, 0, 0], e(0.017), e(0.017)), ([0.020, -0.002, s * 0.030], t(0.012), t(0.012)), ([0.050, 0, s * 0.036], t(0.014), t(0.014)), ([0.150, 0.004, s * 0.018], t(0.0115), t(0.0115)), ([0.270, 0.003, s * 0.003], t(0.013), t(0.013)), ([0.302, 0, 0], e(0.026), e(0.020))] if mid else [([0.0, 0, 0], fl, fl), ([0.302, 0, 0], fl, fl)], BONE, 0.92), f"thigh-{n}")
        if fine:
            m(K.ellipsoid(f"patella-{n}", f"thigh-{n}", [0.014, e(0.0075), 0.013], [0.300, 0.033, 0], BONE, 0.92), f"thigh-{n}")
        m(K.limb(f"tibia-{n}", f"shank-{n}", [(0.006, 0.0, 0.0, e(0.020), e(0.026)), (0.045, 0.004, 0.0, t(0.013), t(0.014)), (0.200, 0.002, 0.0, t(0.0105), t(0.0105)), (0.270, 0.0, 0.0, t(0.012), t(0.014)), (0.2934, 0.0, -0.004, e(0.014), e(0.018))] if mid else [(0.0, 0.0, 0.0, fl, fl), (0.2934, 0.0, 0.0, fl, fl)], BONE, 0.92, s), f"shank-{n}")
        if fine:
            m(K.capsule(f"fibula-{n}", f"shank-{n}", [0.022, -0.012, s * 0.022], [0.298, -0.008, s * 0.021], e(0.0055), e(0.0065), BONE, 0.92), f"shank-{n}")
        if mid:
            m(foot_tube(f"tarsus-{n}", f"foot-{n}", [(-0.048, -0.036, e(0.013), e(0.013)), (-0.030, -0.030, e(0.018), e(0.016)), (0.0, -0.016, e(0.017), 0.019), (0.030, -0.026, e(0.012), 0.022), (0.060, -0.036, t(0.008), 0.027), (0.092, -0.046, t(0.0065), 0.030)], s), f"foot-{n}")
        else:
            m(foot_tube(f"tarsus-{n}", f"foot-{n}", [(-0.020, -0.024, fl, fl), (0.095, -0.024, fl, fl)], s), f"foot-{n}")
        if fine:
            m(K.limb(f"toes-{n}", f"toe-{n}", [(-0.004, 0.017, 0.0, e(0.006), 0.030), (0.030, 0.014, 0.0, e(0.0055), 0.028), (0.050, 0.012, 0.0, e(0.0045), 0.022)], BONE, 0.92, s), f"toe-{n}")
    return o


def joints(den):
    r = {50: 0.011, 100: 0.012, 200: 0.016, 500: 0.0}[den]
    if r == 0:
        return []
    names = ["torso", "neck", "head"] + [f"{b}-{n}" for n in "lr" for b in (["arm", "forearm", "hand", "thigh", "shank", "foot", "toe"] if den <= 100 else ["arm", "forearm", "thigh", "shank", "foot"])]
    out = []
    for b in names:
        q = r * (0.75 if b.startswith(("hand", "toe")) else 1.0)
        s = K.ellipsoid(f"joint-{b}", b, [q, q, q], [0, 0, 0], BONE, 0.5)
        s["member"] = b
        s["part"] = "joint"
        out.append(s)
    return out


def masses(den):
    r = lambda kg: round(0.022 * kg ** (1 / 3) * {50: 1.0, 100: 1.0, 200: 1.1, 500: 1.3}[den], 4)
    spec = [("pelvis", "pelvis", 3.12, [0.032, 0.006, 0]), ("torso", "torso", 8.32, [round(0.45 * R.TORSO_L, 4), 0.004, 0]), ("head", "head", 2.34, [0.05, -0.006, 0]), ("thigh-l", "thigh-l", 3.38, [round(0.4095 * R.THIGH_L, 4), 0, 0]), ("thigh-r", "thigh-r", 3.38, [round(0.4095 * R.THIGH_L, 4), 0, 0]), ("frame", "frame", 3.3, [0.33, 0.071, 0])]
    out = []
    for i, b, kg, at in spec:
        q = r(kg)
        s = K.ellipsoid(f"mass-{i}", b, [0.07, 0.0135 if den <= 100 else 0.015, 0.05] if i == "frame" else [q, q, q], at, "alu-deck" if i == "frame" else BONE, 0.4)
        s["member"] = b
        s["part"] = "mass"
        out.append(s)
    return out


FRAME_MEMBERS = {50: ["deck", "strut-l", "strut-r", "axle-rear", "axle-front", "hinge-block", "neck-arm", "head-tube", "fork-crown", "fork-l", "fork-r", "stem-lower", "clamp", "stem-upper", "bar-tee", "handlebar"], 100: ["deck", "strut-l", "strut-r", "hinge-block", "neck-arm", "head-tube", "fork-l", "fork-r", "stem-lower", "clamp", "stem-upper", "handlebar"], 200: ["deck", "neck-arm", "stem", "fork-l", "fork-r", "handlebar"], 500: ["deck", "stem", "fork", "handlebar"]}
WHEEL_MEMBERS = {50: ["tyre", "rim", "spoke", "hub"], 100: ["tyre", "rim", "hub"], 200: ["tyre"], 500: ["tyre"]}


def structure(den, body):
    ids = {s["id"] for s in body}
    out = skeleton(den) + joints(den) + masses(den)
    for i in FRAME_MEMBERS[den]:
        assert i in ids, i
        out.append({"id": f"st-{i}", "of": i, "member": "frame", "part": "member"})
    for n in ("rear", "front"):
        for i in WHEEL_MEMBERS[den]:
            assert f"{i}-{n}" in ids, i
            out.append({"id": f"st-{i}-{n}", "of": f"{i}-{n}", "member": f"wheel-{n}", "part": "member"})
    f = {50: 0.006, 100: 0.0085, 200: 0.016, 500: 0.038}[den]
    for a, x in (("wheel-rear-contact", 0.0), ("wheel-front-contact", R.WB)):
        s = K.revolve(f"support-{a}", "frame", [(-0.015, 0.0), (-0.015, f), (0.015, f), (0.015, 0.0)] if den < 500 else [(-0.04, 0.0), (-0.04, f), (0.04, f), (0.04, 0.0)], [x, f, 0], K.W, "alu-stem", 0.3)
        s.update(member="frame", part="support", anchor=a)
        out.append(s)
    return out


def d(en, de):
    return {"en": en, "de": de}


def decisions(den, counts):
    b, t, s = counts
    txt = {
        50: [d(f"1:50 body ({b} solids): an 8-year-old girl of 1.274 m (about 6.5 head heights) modelled by hand in the gliding stance. Lofts with anatomical sections for pelvis, trunk with shoulder slope, neck, skull with jaw, cheeks, nose, ears and eyes, upper arm with deltoid, forearm tapering to a flat wrist, a hand closed round the grip with four separate fingers and the thumb, thigh, knee, calf belly and ankle; the zip jacket with hem band, collar, placket and three snap buttons, ankle-length leggings, sneakers with a separate sole, toe box and laces, a round skate helmet with straps, a ponytail, a backpack with front pocket, shoulder straps and sternum strap. The scooter is a real 145 mm children's model: extruded deck with grip tape, rear dropout plates, spoon brake with pad, folding block with hinge pin and gooseneck, head tube with cups, fork crown and blades, two-part stem with clamp and lever, T-bar with grips, polyurethane wheels with core, six spoke arms, hub and axle bolts.", f"1:50-Körper ({b} Körper): ein achtjähriges Mädchen von 1,274 m (etwa 6,5 Kopfhöhen), von Hand in der Gleithaltung modelliert. Lofts mit anatomischen Querschnitten für Becken, Rumpf mit Schulterschräge, Hals, Schädel mit Kiefer, Wangen, Nase, Ohren und Augen, Oberarm mit Deltamuskel, Unterarm bis zum flachen Handgelenk, eine um den Griff geschlossene Hand mit vier einzelnen Fingern und Daumen, Oberschenkel, Knie, Wadenbauch und Knöchel; die Reißverschlussjacke mit Saumbund, Kragen, Leiste und drei Druckknöpfen, knöchellange Leggings, Sneaker mit eigener Sohle, Zehenkappe und Schnürung, ein runder Skatehelm mit Riemen, ein Pferdeschwanz, ein Rucksack mit Vortasche, Schultergurten und Brustgurt. Der Roller ist ein reales 145-mm-Kindermodell: stranggepresstes Trittbrett mit Griffband, hintere Ausfallbleche, Löffelbremse mit Belag, Klappblock mit Gelenkbolzen und Schwanenhals, Steuerrohr mit Lagerschalen, Gabelkrone und Gabelscheiden, zweiteilige Lenkstange mit Klemme und Hebel, T-Lenker mit Griffen, Polyurethanräder mit Kern, sechs Speichenarmen, Nabe und Achsschrauben."),
             d(f"1:50 thermal ({t} solids): every region is its body surface; jacket, leggings, sneakers and helmet are nested shells, one per layer with its real thickness (air, T-shirt, jacket; air, knit; air, sock, upper; air gap, EPS), the hair is its pelt layer. The breath leaves through outlet patches at the nostrils and the mouth, the body plume through a patch on the crown of the helmet.", f"1:50-Thermik ({t} Körper): jede Region ist ihre Körperoberfläche; Jacke, Leggings, Sneaker und Helm sind verschachtelte Schalen, eine je Schicht in echter Dicke (Luft, T-Shirt, Jacke; Luft, Strick; Luft, Socke, Obermaterial; Luftspalt, EPS), das Haar ist seine Haarschicht. Der Atem tritt durch Auslassflächen an Nasenlöchern und Mund aus, die Körperfahne durch eine Fläche auf dem Helmscheitel."),
             d(f"1:50 structure ({s} solids): the load path is the skeleton and the scooter frame, never the body: skull and jaw, cervical spine, thoracolumbar spine with lordosis and kyphosis, ribcage, scapulae, clavicles, pelvis, femur with head, neck and condyles, patella, tibia and fibula, tarsus with metatarsals, toes, humerus, radius and ulna, metacarpus and the phalanges round the grip; joints at the rig joints, masses at the centres of mass of pelvis, trunk, head, thighs and scooter; deck, dropouts, axles, folding block, gooseneck, head tube, fork, stem, clamp and handlebar as members, the wheels as members of their own bones and a roller support under each tread patch.", f"1:50-Tragwerk ({s} Körper): der Lastpfad ist das Skelett und der Rollerrahmen, nie der Körper: Schädel und Kiefer, Halswirbelsäule, Brust- und Lendenwirbelsäule mit Lordose und Kyphose, Brustkorb, Schulterblätter, Schlüsselbeine, Becken, Oberschenkelknochen mit Kopf, Hals und Kondylen, Kniescheibe, Schien- und Wadenbein, Fußwurzel mit Mittelfuß, Zehen, Oberarmknochen, Speiche und Elle, Mittelhand und die Fingerglieder um den Griff; Gelenke an den Rig-Gelenken, Massen in den Schwerpunkten von Becken, Rumpf, Kopf, Oberschenkeln und Roller; Trittbrett, Ausfallbleche, Achsen, Klappblock, Schwanenhals, Steuerrohr, Gabel, Lenkstange, Klemme und Lenker als Glieder, die Räder als Glieder ihrer eigenen Knochen und je ein Rollenlager unter jeder Aufstandsfläche.")],
        100: [d(f"1:100 body ({b} solids), authored anew for 15 mm: the same child with fewer sections; the fingers are one roll closed round the grip with a separate thumb, the shoe is one upper with its sole and a toe box, the nose a single wedge, the ears thickened to 15 mm; eyes, mouth, buttons, placket, laces, helmet straps and the sternum strap are left out, the front pocket is one plain bulge on the backpack, the shoulder straps stay as bands. Scooter: deck, dropout plates, brake spoon, folding block with gooseneck, plain head tube, fork blades, stem with clamp, handlebar, plain grips, tyre, core disc and hub.", f"1:100-Körper ({b} Körper), neu für 15 mm modelliert: dasselbe Kind mit weniger Querschnitten; die Finger sind eine um den Griff geschlossene Rolle mit eigenem Daumen, der Schuh ist ein Schaft samt Sohle mit Zehenkappe, die Nase ein einzelner Keil, die Ohren auf 15 mm verstärkt; Augen, Mund, Knöpfe, Leiste, Schnürung, Helmriemen und Brustgurt entfallen, die Vortasche ist eine glatte Wölbung auf dem Rucksack, die Schultergurte bleiben als Bänder. Roller: Trittbrett, Ausfallbleche, Bremslöffel, Klappblock mit Schwanenhals, glattes Steuerrohr, Gabelscheiden, Lenkstange mit Klemme, Lenker, glatte Griffe, Reifen, Kernscheibe und Nabe."),
              d(f"1:100 thermal ({t} solids): one surface solid per region part, the layers merged into the surface (B2); breath outlets at nose and mouth and the plume patch on the helmet crown.", f"1:100-Thermik ({t} Körper): ein Oberflächenkörper je Regionsteil, die Schichten in der Oberfläche zusammengefasst (B2); Atemauslässe an Nase und Mund und die Fahnenfläche auf dem Helmscheitel."),
              d(f"1:100 structure ({s} solids): the skeleton with skull, jaw, spine, ribcage, scapulae, clavicles, pelvis, femur, patella, tibia, fibula, tarsus, toes, humerus, radius, ulna, metacarpus and one phalanx roll, joints, masses, the frame parts and wheels as members and the two roller supports.", f"1:100-Tragwerk ({s} Körper): das Skelett mit Schädel, Kiefer, Wirbelsäule, Brustkorb, Schulterblättern, Schlüsselbeinen, Becken, Oberschenkelknochen, Kniescheibe, Schienbein, Wadenbein, Fußwurzel, Zehen, Oberarmknochen, Speiche, Elle, Mittelhand und einer Fingergliederrolle, Gelenke, Massen, die Rahmenteile und Räder als Glieder und die beiden Rollenlager.")],
        200: [d(f"1:200 body ({b} solids), authored anew for 30 mm: one loft per limb segment, the hand a closed fist over the grip, shoe and toe as two plain masses, hip and seat merged under the jacket hem, neck with collar, head without nose or ears, helmet, ponytail, one backpack mass. Scooter: deck slab, brake spoon, one stem tube with the head tube, gooseneck, two fork blades, handlebar with grips, wheel discs.", f"1:200-Körper ({b} Körper), neu für 30 mm modelliert: ein Loft je Gliedmaßensegment, die Hand eine geschlossene Faust über dem Griff, Schuh und Zehen als zwei glatte Massen, Hüfte und Gesäß unter dem Jackensaum zusammengefasst, Hals mit Kragen, Kopf ohne Nase und Ohren, Helm, Pferdeschwanz, eine Rucksackmasse. Roller: Trittbrettplatte, Bremslöffel, ein Lenkstangenrohr samt Steuerrohr, Schwanenhals, zwei Gabelscheiden, Lenker mit Griffen, Radscheiben."),
              d(f"1:200 thermal ({t} solids): every region keeps its surface solids, layers merged; outlets for breath and plume at the size this scale can show.", f"1:200-Thermik ({t} Körper): jede Region behält ihre Oberflächenkörper, Schichten zusammengefasst; Auslässe für Atem und Fahne in der Größe, die dieser Maßstab zeigen kann."),
              d(f"1:200 structure ({s} solids): skull, jaw, spine, ribcage, scapulae, clavicles, pelvis and one bone per limb segment at 1.25 times its shaft thickness, joints at the large joints, masses, deck, gooseneck, stem, fork, handlebar and wheels as members, two roller supports.", f"1:200-Tragwerk ({s} Körper): Schädel, Kiefer, Wirbelsäule, Brustkorb, Schulterblätter, Schlüsselbeine, Becken und ein Knochen je Gliedmaßensegment in 1,25-facher Schaftdicke, Gelenke an den großen Gelenken, Massen, Trittbrett, Schwanenhals, Lenkstange, Gabel, Lenker und Räder als Glieder, zwei Rollenlager.")],
        500: [d(f"1:500 body ({b} solids), authored anew for 75 mm: three or four sections per mass; thigh, shank, shoe with toe, upper arm, forearm and fist per side, hip, trunk, shoulders with neck, head, helmet and backpack. Scooter: deck slab, the stem as one slender tube at its true 34 mm, fork with gooseneck as one plate, handlebar, two wheel discs. This is the minimal model of the asset.", f"1:500-Körper ({b} Körper), neu für 75 mm modelliert: drei bis vier Querschnitte je Masse; Oberschenkel, Unterschenkel, Schuh mit Zehen, Oberarm, Unterarm und Faust je Seite, Hüfte, Rumpf, Schultern mit Hals, Kopf, Helm und Rucksack. Roller: Trittbrettplatte, die Lenkstange als ein schlankes Rohr in wahren 34 mm, Gabel mit Schwanenhals als eine Platte, Lenker, zwei Radscheiben. Dies ist das minimale Modell des Assets."),
              d(f"1:500 thermal ({t} solids): one surface solid per region part, layers merged, outlets for breath and plume at the size floor.", f"1:500-Thermik ({t} Körper): ein Oberflächenkörper je Regionsteil, Schichten zusammengefasst, Auslässe für Atem und Fahne an der Größenuntergrenze."),
              d(f"1:500 structure ({s} solids): skull, spine, ribcage, pelvis and one bone per limb segment at 1.6 times its shaft thickness, masses, deck, stem, fork, handlebar and wheels as members, two roller supports; joints are below what the scale shows.", f"1:500-Tragwerk ({s} Körper): Schädel, Wirbelsäule, Brustkorb, Becken und ein Knochen je Gliedmaßensegment in 1,6-facher Schaftdicke, Massen, Trittbrett, Lenkstange, Gabel, Lenker und Räder als Glieder, zwei Rollenlager; Gelenke liegen unter dem, was der Maßstab zeigt.")],
    }
    return txt[den]


def scale(den):
    body = bodies(den)
    th = thermal(den, body)
    st = structure(den, body)
    return {"status": "authored", "denominator": den, "body": [clean(s) for s in body], "thermal": th, "structure": st, "decisions": decisions(den, (len(body), len(th), len(st)))}


def build():
    return {str(den): scale(den) for den in (50, 100, 200, 500)}


def groups():
    out = {}
    for den in (50, 100, 200, 500):
        g = {}
        for s in bodies(den):
            g.setdefault(s["_g"], []).append(s["id"])
        out[str(den)] = g
    return out
