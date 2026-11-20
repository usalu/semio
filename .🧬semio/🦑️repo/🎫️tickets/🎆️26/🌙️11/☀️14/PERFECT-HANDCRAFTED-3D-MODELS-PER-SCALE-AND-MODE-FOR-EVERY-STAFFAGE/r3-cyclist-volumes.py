"""🧬 Round 3 volumes of humans/cyclist: per authored scale the body (from `-body50` / `-coarse`), the thermal list (region surfaces, nested layer shells at 1:50, breath and plume outlets on the surface) and the structure list (the rider's skeleton with spinal curvature, ribcage, scapulae, pelvis and long bones, joints at the rig joints, masses at the centres of mass; the bicycle's tubes, fork, bar, wheels and cranks), with bilingual decisions.

`build()` returns the `volumes` member.
"""
import copy, importlib.util, math, os

T = os.path.dirname(os.path.abspath(__file__))


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-cyclist-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


B = mod("body50")
C = mod("coarse")
R, K = B.R, B.K
SIDES = B.SIDES
LAYERS = {"jacket": [("jk-softshell", 0.002), ("jk-fleece", 0.004), ("jk-base", 0.001), ("jk-air", 0.004)], "trousers": [("tr-stretch", 0.002), ("tr-tights", 0.001), ("tr-air", 0.004)], "gloves": [("gl-leather", 0.001), ("gl-lining", 0.001), ("gl-air", 0.004)], "shoes": [("sh-canvas", 0.004), ("sh-air", 0.004), ("sh-sock", 0.002)], "helmet": [("hm-shell", 0.001), ("hm-eps", 0.025), ("hm-air", 0.012), ("hm-hair", 0.008)], "hair": [("hair-pelt", 0.01)]}
NO_SHELLS = ("jacket-zip", "jacket-reflective", "helmet-vent", "helmet-strap", "trouser-clip", "shoe-sole", "shoe-toe-sole")


def strip(solids):
    return [{k: v for k, v in s.items() if not k.startswith("_")} for s in solids]


def emitters(den):
    """💨 Outlets on the surface: nostrils under the nose tip, the parted lips, and the plume patch on the helmet crown (region jacket, the emitter's own)."""
    t = math.radians(B.HELMET_TILT)
    top = (B.HELMET_C0[0] + 0.131 * math.cos(t), B.HELMET_C0[1] + 0.131 * math.sin(t))
    thick = {50: 0.004, 100: 0.0075, 200: 0.015, 500: 0.0375}[den]
    nose_v = {50: -0.116, 100: -0.116, 200: -0.112, 500: -0.110}[den]
    out = [
        K.ellipsoid("th-emitter-breath-nose", "head", [thick, thick, max(0.012, thick)], [0.000, nose_v, 0.0], B.SKIN, 0.86, role="detail"),
        K.ellipsoid("th-emitter-breath-mouth", "head", [thick, thick, max(0.014, thick)], [-0.024, -0.112 + (0.004 if den >= 200 else 0.0), 0.0], B.SKIN, 0.6, role="detail"),
        K.ellipsoid("th-emitter-plume", "head", [thick, 0.050, 0.050], [top[0] + (0.06 if den == 500 else 0.0) - thick / 2, top[1], 0.0], B.HELMET, 0.82, role="detail"),
    ]
    for s, r, e in zip(out, ("face-skin", "face-skin", "jacket"), ("breath-nose", "breath-mouth", "plume")):
        s["region"] = r
        s["tags"] = [e]
    return out


def face_skin(den):
    """🙂 The face and the skin of the head below the helmet rim as an open skin (the scalp under the helmet belongs to the helmet's hair layer): the head's own sections up to the rim, no cap at the rim."""
    rows = [r for r in B.HEAD if r[0] <= 0.04]
    s = K.stack("th-face", "head", rows, B.SKIN, 0.86, caps=(True, False))
    s["region"] = "face-skin"
    return s


def thermal(den, body):
    out = []
    for s in body:
        r = s["_region"]
        if den == 50 and r in LAYERS and not s["id"].startswith(NO_SHELLS):
            layers = LAYERS[r]
            depth = 0.0
            shells = []
            for k, (lid, th) in enumerate(layers):
                if k == 0:
                    shells.append({"id": f"th-{s['id']}-{lid}", "of": s["id"], "region": r, "layer": lid})
                else:
                    sh = K.shrink({q: v for q, v in s.items() if not q.startswith("_")}, depth, f"th-{s['id']}-{lid}")
                    sh["region"], sh["layer"] = r, lid
                    sh["role"] = "body"
                    sh.pop("texture", None)
                    shells.append(sh)
                depth += th
            out += shells[::-1]
        elif s["id"] == "head":
            out.append(face_skin(den))
        else:
            e = {"id": f"th-{s['id']}", "of": s["id"], "region": r}
            if den == 50 and r in LAYERS:
                e["layer"] = LAYERS[r][0][0]
            out.append(e)
    if den == 500:
        face = K.stack("th-face", "head", [(-0.078, 0.092, -0.050, 0.024, 2.0), (-0.030, 0.108, 0.030, 0.060, 2.1), (0.030, 0.108, 0.070, 0.068, 2.0)], B.SKIN, 0.86)
        face["region"] = "face-skin"
        hair = K.tube("th-hair-tail", "head", [([0.045, 0.092, 0], 0.038, 0.038), ([-0.010, 0.130, 0], 0.038, 0.038), ([-0.100, 0.120, 0], 0.038, 0.038)], B.HAIR, 0.25)
        hair["region"] = "hair"
        out += [face, hair]
    return out + emitters(den)


def member(s, member, part):
    s["member"] = member
    s["part"] = part
    s["material"] = s.get("material", "steel")
    return s


BONE_MAT = "skin"


def skeleton(den):
    """🦴 The rider's skeleton at one scale, authored per scale: shafts, joints and masses with the real proportions (femur head and neck, tibia and fibula, ulna and radius, spine with lordosis and kyphosis, ribcage, scapulae, clavicles, pelvis, skull with jaw)."""
    f = {50: 1.0, 100: 1.0, 200: 2.0, 500: 5.0}[den]
    fl = {50: 0.0075, 100: 0.015, 200: 0.030, 500: 0.075}[den]
    out = []
    m = lambda s, b, part="member": out.append(member(s, b, part))

    def rr(r):
        return max(r, fl / 2 + 0.0002)

    spine = [(-0.010, 0.000), (0.050, -0.022), (0.110, -0.030), (0.170, -0.020), (0.250, 0.005), (0.330, 0.018), (0.400, 0.012), (0.452, -0.004)]
    if den >= 200:
        spine = [(-0.010, 0.0), (0.110, -0.028), (0.300, 0.014), (0.452, -0.004)]
    m(K.tube("spine", "torso", [([u, v, 0.0], rr(0.017 - 0.006 * u / 0.45), rr(0.019 - 0.007 * u / 0.45)) for u, v in spine], BONE_MAT, 0.9), "torso")
    if den <= 200:
        rib = [(0.150, 0.125, 0.015, 0.118), (0.200, 0.150, 0.030, 0.130), (0.270, 0.156, 0.036, 0.128), (0.330, 0.140, 0.036, 0.114), (0.380, 0.110, 0.032, 0.088), (0.410, 0.082, 0.026, 0.058)]
        if den == 200:
            rib = [rib[0], rib[2], rib[4], rib[5]]
        m(K.stack("ribcage", "torso", [(u, fr, bk, hw, 2.2) for u, fr, bk, hw in rib], BONE_MAT, 0.9), "torso")
    else:
        m(K.stack("ribcage", "torso", [(0.160, 0.120, 0.020, 0.110, 2.2), (0.300, 0.140, 0.040, 0.118, 2.2), (0.400, 0.090, 0.030, 0.070, 2.2)], BONE_MAT, 0.9), "torso")
    if den <= 100:
        for s, n in SIDES:
            m(K.tube(f"clavicle-{n}", "torso", [([0.400, -0.085, s * 0.022], rr(0.0065), rr(0.0065)), ([0.412, -0.070, s * 0.100], rr(0.0065), rr(0.0065)), ([0.425, -0.020, s * 0.180], rr(0.0065), rr(0.0065))], BONE_MAT, 0.9), "torso")
            m(K.ellipsoid(f"scapula-{n}", "torso", [0.070, rr(0.0075), 0.048], [0.335, 0.050, s * 0.098], BONE_MAT, 0.9), "torso")
    pel = [(-0.045, 0.072, -0.030, 0.050), (-0.005, 0.064, 0.050, 0.110), (0.050, 0.034, 0.076, 0.128), (0.095, 0.020, 0.072, 0.132), (0.118, 0.004, 0.052, 0.104)]
    if den >= 200:
        pel = [pel[0], pel[2], pel[4]]
    m(K.stack("pelvis-bone", "pelvis", [(u, a, b, c, 2.2) for u, a, b, c in pel], BONE_MAT, 0.9), "pelvis")
    m(K.tube("cervical", "neck", [([-0.010, -0.004, 0.0], rr(0.011), rr(0.012)), ([0.055, -0.012, 0.0], rr(0.010), rr(0.011)), ([0.115, -0.004, 0.0], rr(0.010), rr(0.011))], BONE_MAT, 0.9), "neck")
    sk = [(-0.010, 0.064, 0.064, 0.050), (0.030, 0.096, 0.080, 0.064), (0.080, 0.094, 0.086, 0.066), (0.120, 0.070, 0.070, 0.054), (0.148, 0.022, 0.024, 0.020)]
    if den >= 200:
        sk = [sk[0], sk[2], sk[4]]
    m(K.stack("skull", "head", [(u, a, b, c, 2.0) for u, a, b, c in sk], BONE_MAT, 0.9), "head")
    if den <= 100:
        m(K.stack("mandible", "head", [(-0.074, 0.092, -0.064, 0.012, 2.0), (-0.055, 0.098, -0.020, 0.040, 2.0), (-0.025, 0.090, 0.010, 0.050, 2.0)], BONE_MAT, 0.9), "head")
    for s, n in SIDES:
        m(K.tube(f"femur-{n}", f"thigh-{n}", [([0.0, 0.0, 0.0], rr(0.016), rr(0.016)), ([0.030, 0.004, s * 0.040], rr(0.014), rr(0.014)), ([0.080, 0.006, s * 0.034], rr(0.0135), rr(0.0135)), ([0.300, 0.004, s * 0.008], rr(0.0125), rr(0.0125)), ([0.395, 0.0, 0.0], rr(0.022), rr(0.032))], BONE_MAT, 0.9), f"thigh-{n}")
        if den <= 100:
            m(K.ellipsoid(f"patella-{n}", f"thigh-{n}", [0.022, rr(0.0095), 0.020], [0.400, 0.040, 0.0], BONE_MAT, 0.9), f"thigh-{n}")
        m(K.tube(f"tibia-{n}", f"shank-{n}", [([0.000, 0.004, 0.0], rr(0.020), rr(0.032)), ([0.040, 0.010, 0.0], rr(0.014), rr(0.016)), ([0.380, 0.004, -s * 0.004], rr(0.011), rr(0.012)), ([0.420, 0.002, -s * 0.006], rr(0.013), rr(0.018))], BONE_MAT, 0.9), f"shank-{n}")
        if den <= 100:
            m(K.tube(f"fibula-{n}", f"shank-{n}", [([0.035, -0.012, s * 0.024], rr(0.0055), rr(0.0055)), ([0.250, -0.010, s * 0.022], rr(0.0045), rr(0.0045)), ([0.425, -0.006, s * 0.026], rr(0.0065), rr(0.0065))], BONE_MAT, 0.9), f"shank-{n}")
        heel = B.foot_local(-0.050, -0.070)
        mid = B.foot_local(0.020, -0.040)
        ball = B.foot_local(R.ANKLE_TO_BALL_X, -R.ANKLE_SHOE + R.BALL_SHOE)
        m(K.tube(f"tarsus-{n}", f"foot-{n}", [([heel[0], heel[1], 0.0], rr(0.016), rr(0.016)), ([0.0, -0.012, 0.0], rr(0.020), rr(0.017)), ([mid[0], mid[1], 0.0], rr(0.028), rr(0.013)), ([ball[0], ball[1], 0.0], rr(0.032), rr(0.009))], BONE_MAT, 0.9, hint=K.W), f"foot-{n}")
        if den <= 100:
            m(K.tube(f"phalanges-{n}", f"toe-{n}", [([0.0, 0.0, 0.0], rr(0.030), rr(0.008)), ([0.045, -0.004, 0.0], rr(0.024), rr(0.006))], BONE_MAT, 0.9, hint=K.W), f"foot-{n}")
        m(K.tube(f"humerus-{n}", f"arm-{n}", [([0.0, 0.0, 0.0], rr(0.020), rr(0.020)), ([0.035, 0.0, s * 0.004], rr(0.012), rr(0.012)), ([0.280, 0.0, 0.0], rr(0.0105), rr(0.0105)), ([0.312, 0.0, 0.0], rr(0.015), rr(0.028))], BONE_MAT, 0.9, hint=K.W), f"arm-{n}")
        if den <= 100:
            m(K.tube(f"ulna-{n}", f"forearm-{n}", [([-0.020, 0.0, -s * 0.010], rr(0.010), rr(0.010)), ([0.240, 0.0, -s * 0.014], rr(0.0055), rr(0.0055))], BONE_MAT, 0.9), f"forearm-{n}")
            m(K.tube(f"radius-{n}", f"forearm-{n}", [([0.005, 0.0, s * 0.012], rr(0.0060), rr(0.0060)), ([0.245, 0.0, s * 0.016], rr(0.0095), rr(0.0095))], BONE_MAT, 0.9), f"forearm-{n}")
        else:
            m(K.tube(f"forearm-bones-{n}", f"forearm-{n}", [([-0.020, 0.0, 0.0], rr(0.016), rr(0.010)), ([0.245, 0.0, 0.0], rr(0.018), rr(0.009))], BONE_MAT, 0.9, hint=K.W), f"forearm-{n}")
        m(K.tube(f"metacarpus-{n}", f"hand-{n}", [([0.000, 0.002, 0.0], rr(0.020), rr(0.010)), ([0.030, 0.002, 0.0], rr(0.030), rr(0.007)), ([0.085, 0.0, 0.0], rr(0.032), rr(0.007))], BONE_MAT, 0.9, hint=K.W), f"hand-{n}")
    return out


def joints(den):
    fl = {50: 0.0075, 100: 0.015, 200: 0.030, 500: 0.075}[den]
    r = lambda x: max(x, fl / 2 + 0.0002)
    js = [("torso", 0.020), ("neck", 0.016), ("head", 0.016), ("pelvis", 0.022)]
    for s, n in SIDES:
        js += [(f"thigh-{n}", 0.024), (f"shank-{n}", 0.028), (f"foot-{n}", 0.020), (f"arm-{n}", 0.024), (f"forearm-{n}", 0.020), (f"hand-{n}", 0.016)]
        if den <= 100:
            js.append((f"toe-{n}", 0.012))
    js += [("crank-r", 0.024), ("rear-wheel", 0.022), ("front-wheel", 0.022)]
    if den <= 200:
        js += [("pedal-r", 0.010), ("pedal-l", 0.010)]
    out = []
    for b, rad in js:
        s = K.ellipsoid(f"joint-{b}", b, [r(rad), r(rad), r(rad)], [0.0, 0.0, 0.0], "steel" if b.startswith(("crank", "pedal", "rear", "front")) else BONE_MAT, 0.5)
        out.append(member(s, b, "joint"))
    steer = B.steer(R.FORK_LEN + 0.012 + R.HT_LEN / 2)
    out.append(member(K.ellipsoid("joint-steering", "frame", [r(0.020), r(0.020), r(0.020)], steer, "steel", 0.5), "frame", "joint"))
    return out


def masses(den, rig):
    fl = {50: 0.0075, 100: 0.015, 200: 0.030, 500: 0.075}[den]
    out = []
    for b in rig["bones"]:
        mm = b.get("mass")
        if mm is None or mm["kg"] < 4.0:
            continue
        rad = max(fl / 2 + 0.0002, 0.006 * mm["kg"] ** (1 / 3))
        out.append(member(K.ellipsoid(f"mass-{b['id']}", b["id"], [rad, rad, rad], [mm["comX"], mm["comY"], 0.0], "steel" if b["id"] == "frame" else BONE_MAT, 0.3), b["id"], "mass"))
    return out


BIKE_MEMBERS = ("tube-", "fork-", "stem", "steerer", "handlebar", "seatpost", "bottom-bracket", "crank-arm", "pedal-", "rim", "hub", "spokes", "tyre", "dropout", "chainring", "bb-spindle", "fork")


def bike_structure(den, body):
    out = []
    for s in body:
        if s["_region"] not in ("bike-metal", "tyres") or s["_g"] in ("carrier", "chain-guard", "headlamp", "kickstand"):
            continue
        if not any(s["id"].startswith(p) or p in s["id"] for p in BIKE_MEMBERS):
            continue
        if s["id"].startswith(("pedal-spindle",)):
            continue
        out.append({"id": f"st-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"})
    return out


DECISIONS = {
    50: [
        {"en": "1:50 body: a 1.72 m woman on a low-step city bicycle, every section stated by hand: trousers seat and thighs with quadriceps and hamstring mass, knees with the patella bulge, calves, ankles, canvas sneakers with separate soles and toe boxes; a softshell jacket with waist, bust, shoulder slope and a flared hem over the seated hips, high collar, zip and reflective band; sleeves with deltoid and elbow; gloved hands closed round the swept-back grips with four fingers and a thumb; neck, head with jaw, nose, lips, eyes and ears, a vented helmet with straps and a ponytail. The bicycle has a wave down tube, seat tube with collar and post, head tube with headset, stem and clamp, swept-back bar, grips, brake levers and bell, a crowned fork with curved blades, chainstays, seatstays with bridge, dropouts, revolved sheet fenders with stays, a tube carrier with tail lamp, chain guard, chainring, cog and chain, cranks and platform pedals, a sprung saddle on rails, head lamp, side stand, brakes, and wheels with 40 mm tyres, box rims, 36 crossed spokes and hubs.", "de": "1:50-Körper: eine 1,72 m große Frau auf einem Citybike mit tiefem Einstieg, jeder Querschnitt von Hand gesetzt: Gesäß und Oberschenkel der Hose mit Quadrizeps- und Beugermasse, Knie mit der Wölbung der Kniescheibe, Waden, Knöchel, Canvas-Sneaker mit eigener Sohle und Zehenkappe; eine Softshelljacke mit Taille, Brust, Schulterneigung und über der sitzenden Hüfte ausgestelltem Saum, hohem Kragen, Reißverschluss und Reflexband; Ärmel mit Deltamuskel und Ellbogen; behandschuhte Hände mit vier Fingern und Daumen um die nach hinten geschwungenen Griffe; Hals, Kopf mit Kiefer, Nase, Lippen, Augen und Ohren, ein belüfteter Helm mit Riemen und ein Pferdeschwanz. Das Fahrrad hat ein Wellen-Unterrohr, ein Sitzrohr mit Klemme und Stütze, ein Steuerrohr mit Steuersatz, Vorbau und Klemmung, einen geschwungenen Lenker mit Griffen, Bremshebeln und Klingel, eine Gabel mit Krone und gebogenen Scheiden, Kettenstreben, Sitzstreben mit Steg, Ausfallenden, gedrehte Blechschutzbleche mit Streben, einen Rohrgepäckträger mit Rücklicht, Kettenschutz, Kettenblatt, Ritzel und Kette, Kurbeln und Plattformpedale, einen gefederten Sattel auf Streben, Scheinwerfer, Seitenständer, Bremsen und Räder mit 40-mm-Reifen, Kastenfelgen, 36 gekreuzten Speichen und Naben."},
        {"en": "1:50 thermal: every region has solids; jacket, trousers, gloves, shoes, helmet and hair are nested shells, one per layer with its real thickness, innermost first, the outer layer being the body surface itself; face and neck skin, tyres, saddle and the steel parts are the body surfaces; the breath leaves through patches on the nostrils and between the lips, the plume through a patch on the helmet crown.", "de": "1:50-Thermik: jede Region hat Körper; Jacke, Hose, Handschuhe, Schuhe, Helm und Haar sind verschachtelte Schalen, je Schicht eine mit ihrer wahren Dicke, die innerste zuerst, die äußere Schicht ist die Körperoberfläche selbst; Gesichts- und Halshaut, Reifen, Sattel und die Stahlteile sind die Körperoberflächen; der Atem tritt durch Flächen an den Nasenlöchern und zwischen den Lippen aus, die Fahne durch eine Fläche auf dem Helmscheitel."},
        {"en": "1:50 structure: the rider's skeleton with lumbar lordosis and thoracic kyphosis, ribcage, clavicles, scapulae, pelvis, skull with mandible, cervical spine, femur with head and neck, patella, tibia, fibula, tarsus, toes, humerus, ulna, radius and metacarpus; joints at every rig joint, the steering head, bottom bracket, hubs and pedal spindles; masses at the centres of mass of frame, pelvis, torso, head and thighs; the bicycle's tubes, fork, stem, bar, seat post, cranks, pedals, rims, hubs, spokes and tyres are its members.", "de": "1:50-Tragwerk: das Skelett der Fahrerin mit Lendenlordose und Brustkyphose, Brustkorb, Schlüsselbeinen, Schulterblättern, Becken, Schädel mit Unterkiefer, Halswirbelsäule, Oberschenkelknochen mit Kopf und Hals, Kniescheibe, Schien- und Wadenbein, Fußwurzel, Zehen, Oberarmknochen, Elle, Speiche und Mittelhand; Gelenke an jedem Rig-Gelenk, am Steuerkopf, Tretlager, an den Naben und Pedalachsen; Massen in den Schwerpunkten von Rahmen, Becken, Rumpf, Kopf und Oberschenkeln; Rohre, Gabel, Vorbau, Lenker, Sattelstütze, Kurbeln, Pedale, Felgen, Naben, Speichen und Reifen sind die Glieder des Fahrrads."},
    ],
    100: [
        {"en": "1:100 body: fewer sections per limb and trunk; the four fingers become one roll round the grip beside the thumb, eyes and lips are below the 15 mm feature and dropped, sole and upper are one shoe form; the bicycle loses brake levers, bell, headset cups, collar, bridge, dropouts, fender stays, saddle rails and springs, the side-stand foot and the brakes, the carrier is two side frames with three bars, the wheels have twelve spokes.", "de": "1:100-Körper: weniger Querschnitte je Glied und Rumpf; die vier Finger werden eine Rolle um den Griff neben dem Daumen, Augen und Lippen liegen unter dem 15-mm-Merkmal und entfallen, Sohle und Schaft sind eine Schuhform; das Fahrrad verliert Bremshebel, Klingel, Steuersatzschalen, Sattelklemme, Steg, Ausfallenden, Schutzblechstreben, Sattelstreben und -federn, den Ständerfuß und die Bremsen, der Gepäckträger besteht aus zwei Seitenrahmen mit drei Stäben, die Räder haben zwölf Speichen."},
        {"en": "1:100 thermal: one surface solid per region part, the clothing layers merged into the outer surface (their thickness is below the scale), plus the breath and plume outlets.", "de": "1:100-Thermik: ein Oberflächenkörper je Regionsteil, die Kleidungsschichten in der äußeren Oberfläche zusammengefasst (ihre Dicke liegt unter dem Maßstab), dazu die Atem- und Fahnenauslässe."},
        {"en": "1:100 structure: the same skeleton with every shaft at least 15 mm, scapulae and clavicles kept, the bicycle members as at 1:50 without the bridge and dropouts.", "de": "1:100-Tragwerk: dasselbe Skelett mit jedem Schaft mindestens 15 mm, Schulterblätter und Schlüsselbeine bleiben, die Glieder des Fahrrads wie bei 1:50 ohne Steg und Ausfallenden."},
    ],
    200: [
        {"en": "1:200 body: four to six sections per mass, hands as fists closed round the bar, collar merged into the jacket, ears and nose into the head; the bicycle is head tube with stem, wave tube, seat tube with post, one fork plate, one plate for both chainstays and one for both seatstays, a carrier box on two legs, a solid chainring with its guard, crank bars, pedal blocks, fenders, saddle, lamp and wheels whose tyre and rim are one ring with six spokes.", "de": "1:200-Körper: vier bis sechs Querschnitte je Masse, Hände als um den Lenker geschlossene Fäuste, Kragen in die Jacke, Ohren und Nase in den Kopf übernommen; das Fahrrad besteht aus Steuerrohr mit Vorbau, Wellenrohr, Sitzrohr mit Stütze, einer Gabelplatte, je einer Platte für beide Ketten- und beide Sitzstreben, einem Gepäckträgerkasten auf zwei Beinen, einem vollen Kettenblatt mit Schutz, Kurbelstäben, Pedalblöcken, Schutzblechen, Sattel, Lampe und Rädern, deren Reifen und Felge ein Ring mit sechs Speichen sind."},
        {"en": "1:200 thermal: one surface solid per region part, layers merged; only the total exposed area is compared at this scale.", "de": "1:200-Thermik: ein Oberflächenkörper je Regionsteil, Schichten zusammengefasst; bei diesem Maßstab wird nur die gesamte Oberfläche verglichen."},
        {"en": "1:200 structure: one bone per segment with shafts of at least 30 mm (ulna and radius as one forearm bar, no fibula, patella, toes, clavicles or scapulae), spine with two curves, ribcage, pelvis and skull; the bicycle members are the merged tubes, plates and wheel rings.", "de": "1:200-Tragwerk: ein Knochen je Segment mit Schäften von mindestens 30 mm (Elle und Speiche als ein Unterarmstab, ohne Wadenbein, Kniescheibe, Zehen, Schlüsselbeine oder Schulterblätter), Wirbelsäule mit zwei Krümmungen, Brustkorb, Becken und Schädel; die Glieder des Fahrrads sind die zusammengefassten Rohre, Platten und Radringe."},
    ],
    500: [
        {"en": "1:500 body: two or three sections per mass at the 75 mm floor: head and helmet one form, jacket, trousers seat, one form per arm and leg segment, gloves as blunt fists, the toe box merged into the shoe; the bicycle is drawn by slender members of true diameter: two tyre rings, one tube from the front axle through fork, head tube and stem, one bar from grip to grip, the wave and seat tubes, one bent stay from the bottom bracket over the rear axle to the seat tube, and a saddle block.", "de": "1:500-Körper: zwei oder drei Querschnitte je Masse an der 75-mm-Grenze: Kopf und Helm eine Form, Jacke, Hosengesäß, eine Form je Arm- und Beinsegment, Handschuhe als stumpfe Fäuste, die Zehenkappe im Schuh; das Fahrrad besteht aus schlanken Gliedern wahren Durchmessers: zwei Reifenringe, ein Rohr von der Vorderachse durch Gabel, Steuerrohr und Vorbau, ein Lenker von Griff zu Griff, Wellen- und Sitzrohr, eine gebogene Strebe vom Tretlager über die Hinterachse zum Sitzrohr und ein Sattelblock."},
        {"en": "1:500 thermal: the surfaces of the body masses, with the face and the hair as own solids inside the head-and-helmet form, plus the outlets; only the total exposed area is compared.", "de": "1:500-Thermik: die Oberflächen der Körpermassen, mit Gesicht und Haar als eigenen Körpern in der Form aus Kopf und Helm, dazu die Auslässe; verglichen wird nur die gesamte Oberfläche."},
        {"en": "1:500 structure: a stick skeleton at the 75 mm floor (spine, ribcage, pelvis, skull, one bar per limb segment), joints and masses, and the slender bicycle members.", "de": "1:500-Tragwerk: ein Stabskelett an der 75-mm-Grenze (Wirbelsäule, Brustkorb, Becken, Schädel, ein Stab je Gliedsegment), Gelenke und Massen sowie die schlanken Glieder des Fahrrads."},
    ],
}


def bike500_structure(body):
    out = [{"id": f"st-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"} for s in body if s["_region"] in ("bike-metal", "tyres")]
    for n in ("rear", "front"):
        s = K.capsule(f"st-{n}-spokes", f"{n}-wheel", [0.0, 0.0, 0.0], [R.WHEEL_R - 0.03, 0.0, 0.0], 0.006, 0.006, "steel", 0.6, slender=True)
        s["repeat"] = {"count": 4, "rotationAxis": [0, 0, 1], "rotationRad": round(math.pi / 2, 7)}
        out.append(member(s, f"{n}-wheel", "member"))
    for s, n in SIDES:
        out.append(member(K.capsule(f"st-crank-{n}", f"crank-{n}", [0.0, 0.0, s * R.CRANK_Z], [R.CRANK, 0.0, s * R.CRANK_Z], 0.0377, 0.0377, "steel", 0.4), f"crank-{n}", "member"))
    return out


def build(rig=None):
    import json
    if rig is None:
        rig = json.load(open(os.path.join(T, "🗑️generated", "r3-cyclist", "root.json")))["rig"]
    bodies = {50: B.build(), 100: C.body100(), 200: C.body200(), 500: C.body500()}
    vol = {}
    for den, body in bodies.items():
        st = [{k: v for k, v in s.items() if not k.startswith("_")} for s in skeleton(den)] + joints(den) + masses(den, rig)
        st = [{k: v for k, v in s.items() if not k.startswith("_")} for s in st]
        st += bike500_structure(body) if den == 500 else bike_structure(den, body)
        vol[str(den)] = {"status": "authored", "denominator": den, "body": strip(body), "thermal": strip(thermal(den, body)), "structure": st, "decisions": DECISIONS[den]}
    return vol
