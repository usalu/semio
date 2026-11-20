"""🌱 Round 3 playground swing roots: rig, anchors, dimensions, materials, loads, thermal titles and areas, structural assumptions and provenance of the real single-bay A-frame swing (values from `r3-playground-swing-kit.py`)."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("swing_kit", os.path.join(T, "r3-playground-swing-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

REFERENCES = [
    {"id": "en-1176-2", "citation": "EN 1176-2:2017+AC:2019 Playground equipment and surfacing - Part 2: Additional specific safety requirements and test methods for swings (free space beside the seat at least 20 % of the chain length + 0.2 m, flat seats at least 0.35 m above the surface, as stated in the makers' installation instructions)."},
    {"id": "kompan-ksw921", "citation": "KOMPAN KSW921 Swing 1-seat steel frame, height 2.5 m, L x W x H 2.32 x 1.84 x 2.55 m, hot-dip galvanised steel posts (product data sheet).", "url": "https://kompan.com/en/us/p/ksw931"},
    {"id": "huck-4712", "citation": "Huck Seiltechnik article 4712: swing seat black with aluminium reinforcement, 48 x 17 cm overall (seat about 38 x 17 cm), 2.00 m short-link galvanised chains for a suspension height of 2.50 m, ring screws with finger-trap protection.", "url": "https://www.huck-seiltechnik.de/en/playground/fun-swings_13743/swing-seat-black-with-200-m-long-chains_4712"},
    {"id": "en-10219-2", "citation": "EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (o101.6 x 3.6: 8.70 kg/m; o114.3 x 4.0: 10.9 kg/m; o76.1 x 3.2: 5.75 kg/m; o60.3 x 2.9: 4.11 kg/m; o42.4 x 2.6: 2.55 kg/m)."},
    {"id": "din-766", "citation": "DIN 766:2016 Round steel link chains, short link (8 x 24: wire 8 mm, inner length 24 mm, inner width about 11 mm, outer width about 27 mm, about 1.4 kg/m)."},
]

NOTES = {
    "en": "Side elevation along the beam (end view of one A-frame). Each A-frame is two o60.3 legs splayed 22.2 degrees from the vertical (feet 2.0 m apart) into a welded apex bracket (o114.3 sleeve round the o101.6 beam, two o76.1 sockets, gusset), standing on 180 x 100 x 12 mm base plates with two gussets and two anchor bolts; a o42.4 tie at 0.6 m in tee clamps. The A-frames stand 1.75 m apart so that the seat edge keeps 0.62 m to the frame (EN 1176-2: 0.2 L + 0.2 m = 0.58 m). Pendulum length L = 1.9 m from the bearing pin of the swing hanger (85 mm below the beam axis) to the seat eye bolts, T = 2 pi sqrt(L/g) = 2.7656 s at g = 9.80665 m/s2. The seat is a child of the left chain with a hitch joint whose angle the pose counter-rotates (-theta), so the seat translates as in the real two-chain parallelogram; the right chain is kinematically coupled by the same angle. Amplitude 30 degrees (authored). Occupant: child of 25 kg (authored estimate). The seat stays horizontal; the swing is modelled as a one-degree-of-freedom pendulum with a rigid occupant.",
    "de": "Seitenansicht entlang des Holms (Stirnansicht eines A-Bocks). Jeder A-Bock besteht aus zwei Beinen o60,3, um 22,2 Grad gegen die Senkrechte gespreizt (Füße 2,0 m auseinander), in einem geschweißten Scheitelbeschlag (Hülse o114,3 um den Holm o101,6, zwei Muffen o76,1, Knotenblech), auf Fußplatten 180 x 100 x 12 mm mit zwei Rippen und zwei Ankerbolzen; ein Zugrohr o42,4 auf 0,6 m in T-Klemmen. Die A-Böcke stehen 1,75 m auseinander, damit die Sitzkante 0,62 m Abstand zum Gestell hat (EN 1176-2: 0,2 L + 0,2 m = 0,58 m). Pendellänge L = 1,9 m vom Lagerbolzen des Schaukelgelenks (85 mm unter der Holmachse) bis zu den Ringschrauben des Sitzes, T = 2 pi sqrt(L/g) = 2,7656 s bei g = 9,80665 m/s². Der Sitz ist Kind der linken Kette mit einem Hitch-Gelenk, dessen Winkel die Pose gegendreht (-theta) setzt, sodass der Sitz wie im realen Zwei-Ketten-Parallelogramm translatiert; die rechte Kette ist kinematisch über denselben Winkel gekoppelt. Amplitude 30 Grad (Festlegung). Insasse: Kind mit 25 kg (Schätzung). Der Sitz bleibt waagerecht; die Schaukel ist als Pendel mit einem Freiheitsgrad und starrem Insassen modelliert.",
}

SUMMARY = {
    "en": "A galvanised steel single-bay swing: two A-frames on base plates carry a 1.9 m beam with two swing hangers, two short-link chains and a flat rubber seat; the seat swings as a pendulum of 1.9 m length with a period of 2.77 s.",
    "de": "Einfeldschaukel aus verzinktem Stahl: zwei A-Böcke auf Fußplatten tragen einen 1,9 m langen Holm mit zwei Schaukelgelenken, zwei Rundstahlketten und einen flachen Gummisitz; der Sitz schwingt als Pendel von 1,9 m Länge mit einer Periode von 2,77 s.",
}

ALUMINIUM = {"id": "aluminium", "title": {"en": "Aluminium seat core", "de": "Aluminium-Sitzkern"}, "emissivity": 0.25, "solarAbsorptivity": 0.4, "densityKgM3": 2700, "conductivityWmK": 160, "specificHeatJKgK": 880, "elasticModulusPa": 70000000000, "source": "literature", "reference": "iso-10456-2007"}

STRUCTURE_ASSUMPTIONS = [
    {"en": "The two A-frames stand on four base plates on the ground (z = +-0.875 m): a pin under each left foot and a roller under each right foot (x = 0 and x = 2.0 m); the o42.4 tie at 0.6 m closes each A-frame; the seat swings as a one-DOF pendulum, so its inertia enters through the kinematics of the clip.",
     "de": "Die beiden A-Böcke stehen auf vier Fußplatten am Boden (z = +-0,875 m): unter jedem linken Fuß ein Festlager, unter jedem rechten ein Rollenlager (x = 0 und x = 2,0 m); das Zugrohr o42,4 auf 0,6 m schließt jeden A-Bock; der Sitz schwingt als Ein-Freiheitsgrad-Pendel, seine Trägheit wirkt über die Kinematik des Clips."},
    {"en": "The occupant is rigid and seated; the two chains coincide in this side elevation, the second is kinematically coupled to the first; the chains carry tension only, no member bending is checked.",
     "de": "Der Insasse ist starr und sitzend; die beiden Ketten fallen in dieser Seitenansicht zusammen, die zweite ist kinematisch an die erste gekoppelt; die Ketten tragen nur Zug, keine Biegenachweise der Glieder."},
]

THERMAL_ASSUMPTION_3 = {"en": "Heat capacities rho*cp*min(thickness, sqrt(alpha*3600 s)) with ISO 10456 values; the second chain is hidden in the side elevation but counts in the chain area.", "de": "Wärmekapazitäten rho*cp*min(Dicke, sqrt(alpha*3600 s)) mit ISO-10456-Werten; die zweite Kette ist in der Seitenansicht verdeckt, zählt aber zur Kettenfläche."}

REGION_TITLES = {
    "frame-galv": {"en": "Galvanised A-frames, beam and swing hangers", "de": "Verzinkte A-Böcke, Holm und Schaukelgelenke"},
    "chain-steel": {"en": "Steel chains, hooks and seat eye bolts", "de": "Stahlketten, Haken und Ringschrauben des Sitzes"},
    "seat-rubber": {"en": "Rubber seat", "de": "Gummisitz"},
}


def masses():
    return k.masses()


def apply(asset, areas=None):
    m = k.masses()
    sec = k.sections()
    asset["summary"] = SUMMARY
    prov = asset["provenance"]
    known = {r["id"] for r in prov["references"]}
    prov["references"] = prov["references"] + [r for r in REFERENCES if r["id"] not in known]
    prov["notes"] = NOTES
    asset["dimensions"] = {"heightM": 2.5, "widthM": round(k.FEET_X[1] - k.FEET_X[0] + k.PLATE[0], 3), "depthM": round(2 * k.BEAM_HALF, 3), "massKg": round(sum(v[0] for v in m.values()), 1)}
    if not any(x["id"] == "aluminium" for x in asset["materials"]):
        asset["materials"].append(ALUMINIUM)
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    place = {
        "leg-left": (k.FEET_X[0], 0.0, k.LEG_ANGLE, k.LEG_LEN),
        "leg-right": (k.FEET_X[1], 0.0, 180.0 - k.LEG_ANGLE, k.LEG_LEN),
        "brace": (k.BRACE_X0, k.BRACE_Y, 0.0, k.BRACE_LEN),
        "beam": (k.APEX_X, k.H_BEAM, 0.0, 0.1),
        "chain-left": (0.0, -k.PIVOT_DROP, -90.0, k.CHAIN_LEN),
        "chain-right": (0.0, -k.PIVOT_DROP, -90.0, k.CHAIN_LEN),
        "seat": (k.CHAIN_LEN, 0.0, 90.0, k.SEAT[0] / 2),
    }
    sections_of = {"leg-left": "leg", "leg-right": "leg", "brace": "brace", "beam": "beam", "chain-left": "chain", "chain-right": "chain", "seat": "seat"}
    titles = {
        "leg": {"en": "Leg tube o60.3 x 2.9 (EN 10219-2)", "de": "Beinrohr o60,3 x 2,9 (EN 10219-2)"},
        "brace": {"en": "Tie tube o42.4 x 2.6 (EN 10219-2)", "de": "Zugrohr o42,4 x 2,6 (EN 10219-2)"},
        "beam": {"en": "Beam tube o101.6 x 3.6 (EN 10219-2)", "de": "Holmrohr o101,6 x 3,6 (EN 10219-2)"},
        "chain": {"en": "Two link wires o8 in tension (DIN 766 8 x 24)", "de": "Zwei Gliedquerschnitte o8 auf Zug (DIN 766 8 x 24)"},
        "seat": {"en": "Aluminium core 160 x 8 mm of the seat", "de": "Aluminiumkern 160 x 8 mm des Sitzes"},
    }
    materials = {"leg": "steel-galvanised", "brace": "steel-galvanised", "beam": "steel-galvanised", "chain": "steel-galvanised", "seat": "aluminium"}
    refs = {"leg": "en-10219-2", "brace": "en-10219-2", "beam": "en-10219-2", "chain": "din-766", "seat": "huck-4712"}
    for bid, (x, y, ang, length) in place.items():
        b = bones[bid]
        b["x"], b["y"], b["angleDeg"], b["lengthM"] = round(x, 6), round(y, 6), round(ang, 4), round(length, 6)
        kg, cx, cy = m[bid]
        b["mass"] = {"kg": kg, "comX": cx, "comY": cy, "distribution": "uniform" if bid.startswith("chain") else "point", "source": "literature", "reference": refs[sections_of[bid]]}
        s = sec[sections_of[bid]]
        b["section"] = {"areaM2": s["areaM2"], "secondMomentM4": s["secondMomentM4"], "extremeFibreM": s["extremeFibreM"], "material": materials[sections_of[bid]], "title": titles[sections_of[bid]]}
    bones["seat"]["title"] = {"en": "Seat (hitch at the eye bolts)", "de": "Sitz (Gelenk an den Ringschrauben)"}
    for a in asset["anchors"]:
        if a["kind"] == "support":
            a["z"] = k.ZA if a["z"] > 0 else -k.ZA
            a["footprintLengthM"], a["footprintWidthM"] = k.PLATE[0], k.PLATE[2]
        if a["id"] == "seat-centre":
            a["x"], a["y"] = 0.0, k.SEAT_TOP_V
    for p in asset["poses"]:
        for ld in p.get("loads", []):
            if ld["bone"] == "seat":
                ld["x"], ld["y"] = 0.0, k.SEAT_TOP_V
    th = asset["thermal"]
    for r in th["regions"]:
        r["title"] = REGION_TITLES[r["id"]]
        if areas and r["id"] in areas:
            r["areaM2"] = areas[r["id"]]
    th["assumptions"][2] = THERMAL_ASSUMPTION_3
    asset["structure"]["assumptions"] = STRUCTURE_ASSUMPTIONS
    return asset
