"""🎸 The authoritative writer of `used-electric-bass-and-cabinet`: a used long-scale four-string electric bass in a folding A-frame stand beside a 300 W solid-state bass head on a 4×10 cabinet on castors, as a used-instrument dealer offers it. Run `python3 author-bass.py` to regenerate `🔣️.json` next to this file.

Every number is stated here or in `author-layout.py` / `author-parts.py`; the reference table and the reasons live in `../../reports/used-electric-bass-and-cabinet.md`.
"""
import importlib.util
import json
import math
import pathlib

HERE = pathlib.Path(__file__).parent
_spec = importlib.util.spec_from_file_location("parts", HERE / "author-parts.py")
P = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(P)
L, G, F = P.L, P.G, P.F
R3 = G.R3
SCALES = (50, 100, 200, 500)
VIEWS = ("elevation", "front", "plan")


def T(en, de):
    """🌍 A bilingual text."""
    return {"en": en, "de": de}


MATERIALS = [
    ("plywood-tolex", "Poplar plywood covered with black vinyl tolex, scuffed", "Pappelsperrholz mit schwarzem Vinyl-Tolex bezogen, abgestoßen", 0.92, 0.9, 620, 0.13, 1600, 9.0e9, 3.0e7, "authored-estimate", "iso-10456-2007"),
    ("grille-steel", "Perforated sheet steel grille, black powder coat", "Gelochtes Stahlblechgitter, schwarz pulverbeschichtet", 0.9, 0.9, 7850, 50, 470, 2.1e11, 2.35e8, "authored-estimate", "incropera-2018"),
    ("chrome-steel", "Chrome-plated steel, scuffed (ball corners, handles, jack plate)", "Verchromter Stahl, zerkratzt (Kugelecken, Griffe, Buchsenplatte)", 0.2, 0.4, 7850, 50, 470, 2.1e11, 2.35e8, "authored-estimate", "incropera-2018"),
    ("zinc-steel", "Zinc-plated pressed steel of the castor forks", "Verzinkter Stahl der Lenkrollengabeln", 0.25, 0.5, 7850, 50, 470, 2.1e11, 2.35e8, "authored-estimate", "incropera-2018"),
    ("pu-wheel", "Polyurethane castor tread, worn", "Polyurethan-Laufbelag der Rollen, abgefahren", 0.9, 0.9, 1200, 0.25, 1800, None, None, "authored-estimate", "iso-10456-2007"),
    ("rubber", "Rubber feet and caps", "Gummifüße und -kappen", 0.92, 0.9, 1200, 0.16, 1500, None, None, "authored-estimate", "iso-10456-2007"),
    ("aluminium", "Brushed aluminium front panel with printed legends", "Gebürstete Aluminium-Frontplatte mit Beschriftung", 0.3, 0.4, 2700, 160, 900, 7.0e10, 1.5e8, "authored-estimate", "incropera-2018"),
    ("chassis-steel", "Black painted steel chassis and rear panel", "Schwarz lackiertes Stahlchassis und Rückwand", 0.9, 0.9, 7850, 50, 470, 2.1e11, 2.35e8, "authored-estimate", "incropera-2018"),
    ("knob-abs", "Black ABS knobs and switch", "Schwarze ABS-Knöpfe und Schalter", 0.9, 0.9, 1050, 0.17, 1400, None, None, "authored-estimate", "iso-10456-2007"),
    ("handle-vinyl", "Vinyl-covered strap handle", "Vinylbezogener Tragegriff", 0.9, 0.9, 1300, 0.16, 1000, None, None, "authored-estimate", "iso-10456-2007"),
    ("alder-finish", "Alder body with polyurethane sunburst finish, worn through at the forearm", "Erlenkorpus mit Polyurethan-Sunburst-Lack, am Unterarm durchgescheuert", 0.9, 0.6, 420, 0.12, 1700, 9.5e9, 6.8e7, "literature", "fpl-wood-handbook-2021"),
    ("maple", "Hard maple neck and headstock, satin finish worn", "Ahornhals und Kopfplatte aus Hartahorn, Seidenmattlack abgegriffen", 0.9, 0.5, 705, 0.16, 1600, 1.26e10, 1.09e8, "literature", "fpl-wood-handbook-2021"),
    ("rosewood", "Rosewood fretboard with nickel-silver frets", "Palisandergriffbrett mit Neusilberbünden", 0.9, 0.8, 850, 0.17, 1600, 1.15e10, 1.1e8, "literature", "fpl-wood-handbook-2021"),
    ("chrome-hardware", "Chrome-plated steel hardware, tarnished (bridge, tuners, plate, knobs, strap buttons)", "Verchromte Stahlteile, angelaufen (Brücke, Mechaniken, Platte, Knöpfe, Gurtpins)", 0.15, 0.4, 7850, 50, 470, 2.1e11, 2.35e8, "authored-estimate", "incropera-2018"),
    ("nickel-string", "Nickel-wound steel bass strings", "Nickelumsponnene Stahl-Basssaiten", 0.12, 0.4, 7800, 50, 460, 2.0e11, 2.0e9, "authored-estimate", "daddario-tension-chart"),
    ("pickguard-pvc", "Three-ply PVC pickguard, scratched", "Dreilagiges PVC-Schlagbrett, verkratzt", 0.92, 0.85, 1350, 0.16, 1000, None, None, "authored-estimate", "iso-10456-2007"),
    ("pickup-abs", "Black ABS pickup covers", "Schwarze ABS-Tonabnehmerkappen", 0.9, 0.9, 1050, 0.17, 1400, None, None, "authored-estimate", "iso-10456-2007"),
    ("stand-steel", "Black powder-coated steel tube 16 x 1.2 mm", "Schwarz pulverbeschichtetes Stahlrohr 16 x 1,2 mm", 0.9, 0.9, 7850, 50, 470, 2.1e11, 2.35e8, "authored-estimate", "incropera-2018"),
    ("pe-foam", "Closed-cell foam sleeves of cradle and yoke", "Geschlossenzellige Schaumhüllen von Auflage und Halsgabel", 0.9, 0.9, 40, 0.04, 1400, None, None, "authored-estimate", "iso-10456-2007"),
]

DENSITY = {m[0]: m[5] for m in MATERIALS}

REFERENCES = [
    ("hornbostel-sachs-1914", "von Hornbostel, E. M., Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift für Ethnologie 46, 553-590 (class 321.322: necked box lutes, the family of the bass guitar)."),
    ("mimo-2011", "MIMO Consortium (2011). Revision of the Hornbostel-Sachs Classification of Musical Instruments (class 321.322-5 necked box lute played with the fingers, and 5 electrophones: the electric bass sounds by strings and is heard through an electromagnetic pickup, amplifier and loudspeakers)."),
    ("iec-62368-1-2018", "IEC 62368-1:2018 Audio/video, information and communication technology equipment - Part 1: Safety requirements (Annex E: normal operating condition of an audio amplifier is 1/8 of the non-clipped output power with pink noise)."),
    ("iec-60268-5-2003", "IEC 60268-5:2003 Sound system equipment - Part 5: Loudspeakers (rated noise power and nominal impedance of drive units and enclosures)."),
    ("en-12532-1999", "EN 12532:1999 Castors and wheels - Castors and wheels for applications up to 1.1 m/s (swivel castors with 75 mm wheels, load rating and rolling)."),
    ("din-33402-2-2020", "DIN 33402-2:2020 Ergonomics - Human body dimensions - Part 2: Values (standing hip and elbow heights of adults: the strap height of the played bass)."),
    ("fpl-wood-handbook-2021", "Forest Products Laboratory (2021). Wood Handbook - Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA (densities, moduli of elasticity and of rupture of red alder and hard maple, thermal properties of wood)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (plywood, polymers, rubber, steel)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities of painted, plated and polished metals, convection)."),
    ("daddario-tension-chart", "D'Addario & Co. (2019). A Complete Technical Reference for Fretted Instrument String Tensions (unit weights and tensions of nickel-wound bass strings at 34-inch scale: 170 to 190 N per string of a 45-105 set)."),
    ("bass-catalogue-typical", "Catalogue data of long-scale four-string solid-body electric basses of 1970-2000 production as music retailers list them (34-inch scale of 864 mm, overall length 1.15-1.17 m, body 0.33-0.34 m wide and 44-45 mm thick, neck 38-42 mm at the nut, 3.8-4.8 kg); a type, not one product."),
    ("cabinet-catalogue-typical", "Catalogue data of 4x10-inch bass loudspeaker cabinets as music retailers list them (0.60-0.66 m wide, 0.60-0.70 m high, 0.38-0.45 m deep, 30-40 kg, 400-800 W at 8 ohm, ball corners, recessed side handles, removable 3-inch swivel castors); a type, not one product."),
    ("head-catalogue-typical", "Catalogue data of solid-state bass amplifier heads of 250-350 W into 4 ohm as music retailers list them (0.48-0.60 m wide, 0.20-0.27 m high, 0.25-0.32 m deep, 9-16 kg, rated mains input 400-600 W, fan-cooled); a type, not one product."),
    ("stand-catalogue-typical", "Catalogue data of folding tubular A-frame guitar and bass stands as music retailers list them (16-19 mm steel tube, foam-covered cradle arms at about 0.2 m, neck yoke at 0.6-0.85 m, 1.2-2.0 kg); a type, not one product."),
]


def material_doc(m):
    """🧱 A material row."""
    d = {"id": m[0], "title": T(m[1], m[2]), "emissivity": m[3], "solarAbsorptivity": m[4], "densityKgM3": m[5], "conductivityWmK": m[6], "specificHeatJKgK": m[7]}
    if m[8]:
        d["elasticModulusPa"] = m[8]
        d["strengthPa"] = m[9]
    d["source"] = m[10]
    d["reference"] = m[11]
    return d


def volume_of(solid):
    """🧮 The volume of a body solid in cubic metres (boxes, revolves, capsules and extrudes)."""
    k = solid["kind"]
    if k == "box":
        s = solid["size"]
        return s[0] * s[1] * s[2]
    if k == "revolve":
        a, r = solid["profile"][1]
        return math.pi * r * r * abs(solid["profile"][2][0] - a)
    if k == "capsule":
        r = solid["r0"]
        ln = math.dist(solid["p0"], solid["p1"])
        return math.pi * r * r * ln + 4 / 3 * math.pi * r ** 3
    if k == "extrude":
        a = abs(G.area(solid["profile"])) - sum(abs(G.area(h)) for h in solid.get("holes", []))
        return a * abs(solid["to"] - solid["from"])
    return 0.0


def centre_of(solid):
    """🎯 The bone-local centre of a body solid."""
    k = solid["kind"]
    if k == "box":
        return solid["transform"]["translation"]
    if k == "revolve":
        return solid["origin"]
    if k == "capsule":
        return [(solid["p0"][i] + solid["p1"][i]) / 2 for i in range(3)]
    if k == "extrude":
        c = G.centroid(solid["profile"]) if abs(G.area(solid["profile"])) > 0 else solid["profile"][0]
        m = (solid["from"] + solid["to"]) / 2
        if solid["plane"] == "xy":
            return [c[0], c[1], m]
        if solid["plane"] == "yz":
            return [m, c[0], c[1]]
        return [c[1], m, c[0]]
    return [0, 0, 0]


PARTS = {s: P.all_parts(s) for s in SCALES}


def bone_mass(bone, extra=0.0):
    """⚖️ Mass and local centre of a bone from its 1:50 solids (volume x density), plus parts the model holds inside (drivers, transformer)."""
    m, mx, my = 0.0, 0.0, 0.0
    for part in PARTS[50]:
        if part.bone != bone:
            continue
        vol = volume_of(part.solid)
        dens = DENSITY[part.material]
        if part.material in ("grille-steel", "chrome-hardware"):
            dens *= 0.45
        if part.material == "stand-steel" and part.solid["kind"] == "capsule":
            r = part.solid["r0"]
            dens *= (r * r - (r - 0.0012) ** 2) / (r * r)
        c = centre_of(part.solid)
        m += vol * dens
        mx += vol * dens * c[0]
        my += vol * dens * c[1]
    return m, mx, my


def masses():
    """🧾 The masses of every bone: the hollow cabinet is its 18 mm plywood shell, four 3.0 kg drive units, grille and hardware; the head its 15 mm sleeve, panels and 6.4 kg of chassis, transformer and heat sink; the bass its wood and hardware; the stand its 16 x 1.2 mm tubes."""
    out = {}
    shell = 2 * (0.62 * 0.42) + 2 * (0.624 * 0.42) + 2 * (0.584 * 0.624) - 4 * math.pi * 0.115 ** 2
    ply = shell * 0.018 * 620
    drivers = 4 * 3.0
    grille = 0.62 * 0.604 * 0.0015 * 7850 * 0.5
    corners = 8 * 0.08
    handles = 2 * 0.25 + 0.15
    tolex = 1.9 * 0.35
    bracing = 0.8
    cab = ply + drivers + grille + corners + handles + tolex + bracing
    cab_com = ((ply + tolex + bracing + corners + handles) * 0.21 + drivers * 0.33 + grille * 0.414) / cab
    out["cabinet"] = (cab, cab_com, 0.33)
    sleeve = (2 * 0.30 * 0.55 + 2 * 0.30 * 0.24) * 0.015 * 620
    panels = 0.21 * 0.52 * 0.003 * 2700 + 0.21 * 0.52 * 0.0015 * 7850
    inside = 3.6 + 1.3 + 0.9 + 0.6
    head = sleeve + panels + inside + 0.25
    out["head"] = (head, 0.13, 0.115)
    m, mx, my = bone_mass("bass")
    extra = 0.30 + 0.12
    out["bass"] = (m + extra, mx / m, my / m)
    for b in ("stand", "stand-rear-leg"):
        m, mx, my = bone_mass(b)
        out[b] = (m + (0.15 if b == "stand" else 0.0), mx / m, my / m)
    for c in P.CASTORS:
        out[c] = (0.15, 0.03, -0.015)
        out[c.replace("castor", "wheel")] = (0.10, 0.0, 0.0)
    return out


MASS = masses()
TOTAL_MASS = sum(v[0] for v in MASS.values())


def bone_doc(bid, parent, title, x, y, z, deg, length, joint, mass=None, section=None):
    """🦴 A bone row."""
    d = {"id": bid, "parent": parent, "title": title, "x": R3(x), "y": R3(y)}
    if z:
        d["z"] = R3(z)
    d["angleDeg"] = R3(deg)
    d["lengthM"] = R3(length)
    d["joint"] = joint
    if mass:
        d["mass"] = {"kg": round(mass[0], 3), "comX": R3(mass[1]), "comY": R3(mass[2]), "distribution": "point", "source": "authored-estimate", "reference": mass[3]}
    if section:
        d["section"] = section
    return d


PLAY_ABD = 65.0


def played_offset():
    """🎯 The slide of the bass carrier from the stand to the strap position: the body centre ends 1.0 m above the floor, 0.20 m further ahead than the stand foot, the neck 24 degrees above horizontal towards the player's left."""
    fb = F["bass"]
    ring = L.body_outline()
    cs, cw = G.centroid(ring)
    u = fb.axis("u")
    w = (0.0, 0.0, 1.0)
    ca, sa = math.cos(math.radians(PLAY_ABD)), math.sin(math.radians(PLAY_ABD))
    un = tuple(ca * u[i] + sa * w[i] for i in range(3))
    wn = tuple(-sa * u[i] + ca * w[i] for i in range(3))
    centre = [cs * un[i] + cw * wn[i] for i in range(3)]
    target = (0.80, 1.00)
    dx = target[0] - centre[0] - fb.o[0]
    dy = target[1] - centre[1] - fb.o[1]
    return dx, dy


PLAY_DX, PLAY_DY = (round(x, 4) for x in played_offset())
ROLL_M = 0.60
WHEEL_TURN_DEG = -math.degrees(ROLL_M / L.WHEEL_R)


def rig():
    """🦴 The rig: a floor datum with the stand fixed to it, the cabinet on a floor slide with four swivel castors and spinning wheels, the head fixed on the cabinet, and the bass on a carrier slide with a revolute joint that lays its neck over for playing."""
    sp = P.SP
    fb = F["bass"]
    bones = [bone_doc("floor", None, T("Floor datum", "Bodenbezug"), 0, 0, 0, 0, 0.01, {"type": "fixed"})]
    bones.append(bone_doc("stand", "floor", T("Folding A-frame stand: frame tubes, cradle and yoke", "Klappbarer A-Ständer: Rahmenrohre, Auflage und Halsgabel"), L.STAND_X, 0, L.STAND_Z, 0, 0.40, {"type": "fixed"}, MASS["stand"] + ("stand-catalogue-typical",), {"areaM2": 5.58e-05, "secondMomentM4": 1.49e-09, "extremeFibreM": 0.008, "material": "stand-steel", "title": T("Steel tube 16 x 1.2 mm", "Stahlrohr 16 x 1,2 mm")}))
    hinge = sp["top"]
    fr = F["stand-rear-leg"]
    bones.append(bone_doc("stand-rear-leg", "stand", T("Folding rear leg", "Klappbares Hinterbein"), hinge[0], hinge[1], 0, fr.deg, P.REAR_LEG_LENGTH, {"type": "revolute", "minDeg": -60, "maxDeg": 0}, MASS["stand-rear-leg"] + ("stand-catalogue-typical",), {"areaM2": 5.58e-05, "secondMomentM4": 1.49e-09, "extremeFibreM": 0.008, "material": "stand-steel", "title": T("Steel tube 16 x 1.2 mm", "Stahlrohr 16 x 1,2 mm")}))
    bones.append(bone_doc("cabinet-roll", "floor", T("Floor slide of the rolled cabinet", "Bodenverschiebung des gerollten Lautsprechers"), 0, 0, 0, 0, 0.01, {"type": "prismatic", "axisDeg": 0, "minM": -1.5, "maxM": 1.5}))
    bones.append(bone_doc("cabinet", "cabinet-roll", T("4x10 cabinet: carcass, grille, corners, handles", "4x10-Box: Gehäuse, Gitter, Ecken, Griffe"), -L.CAB_D / 2, L.CASTOR_H, 0, 0, L.CAB_D, {"type": "fixed"}, MASS["cabinet"] + ("cabinet-catalogue-typical",)))
    for c in P.CASTORS:
        fc = F[c]
        lc = F["cabinet"].local(*fc.o)
        side = "left" if "-l" in c else "right"
        pos = "Front" if "front" in c else "Rear"
        pos_de = "Vordere" if "front" in c else "Hintere"
        bones.append(bone_doc(c, "cabinet", T(f"{pos} {side} swivel castor", f"{pos_de} {'linke' if side == 'left' else 'rechte'} Lenkrolle"), lc[0], lc[1], lc[2], -90, L.CASTOR_H - L.WHEEL_R, {"type": "revolute", "continuousTwist": True}, MASS[c] + ("en-12532-1999",)))
        wl = c.replace("castor", "wheel")
        lw = fc.local(*F[wl].o)
        bones.append(bone_doc(wl, c, T(f"{pos} {side} castor wheel 75 mm", f"{pos_de} {'linkes' if side == 'left' else 'rechtes'} Rollenrad 75 mm"), lw[0], lw[1], 0, 90, L.WHEEL_R, {"type": "revolute", "minDeg": -3600, "maxDeg": 3600}, MASS[wl] + ("en-12532-1999",)))
    lh = F["cabinet"].local(*F["head"].o)
    bones.append(bone_doc("head", "cabinet", T("300 W bass head on the cabinet", "300-W-Bassverstärker auf der Box"), lh[0], lh[1], 0, 0, L.HEAD_D, {"type": "fixed"}, MASS["head"] + ("head-catalogue-typical",)))
    bones.append(bone_doc("bass-slide", "floor", T("Forward slide of the bass from stand to player", "Vorwärtsschlitten des Basses vom Ständer zum Spieler"), fb.o[0], fb.o[1], fb.o[2], 0, 0.01, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.5}))
    bones.append(bone_doc("bass-lift", "bass-slide", T("Lift of the bass from the cradle to strap height", "Hub des Basses von der Auflage auf Gurthöhe"), 0, 0, 0, 0, 0.01, {"type": "prismatic", "axisDeg": 90, "minM": -0.05, "maxM": 1.0}))
    bones.append(bone_doc("bass", "bass-lift", T("Four-string electric bass, 34-inch scale", "Viersaitiger E-Bass, Mensur 34 Zoll"), 0, 0, 0, L.BASS_DEG, 1.165, {"type": "revolute", "minDeg": -30, "maxDeg": 30}, MASS["bass"] + ("bass-catalogue-typical",), {"areaM2": 0.00135, "secondMomentM4": 6.5e-08, "extremeFibreM": 0.013, "material": "maple", "title": T("Maple neck 52 x 26 mm with fretboard", "Ahornhals 52 x 26 mm mit Griffbrett")}))
    return {"kind": "mechanical", "root": "floor", "bones": bones}


def bass_world(pose):
    """🌍 The world frame (origin, u, v, w) of the bass bone in a pose: `stored` at rest or `played` slid and laid over."""
    fb = F["bass"]
    u, v, w = fb.axis("u"), fb.axis("v"), (0.0, 0.0, 1.0)
    o = list(fb.o)
    if pose == "played":
        ca, sa = math.cos(math.radians(PLAY_ABD)), math.sin(math.radians(PLAY_ABD))
        u, w = tuple(ca * u[i] + sa * w[i] for i in range(3)), tuple(-sa * u[i] + ca * w[i] for i in range(3))
        o = [o[0] + PLAY_DX, o[1] + PLAY_DY, o[2]]
    return o, u, v, w


def at(frame, s, vv, ww):
    """📍 A bass-local point in world metres for a frame of `bass_world`."""
    o, u, v, w = frame
    return [o[i] + s * u[i] + vv * v[i] + ww * w[i] for i in range(3)]


def dirw(frame, a, b, c):
    """🧭 A bass-local direction in world."""
    _, u, v, w = frame
    return [a * u[i] + b * v[i] + c * w[i] for i in range(3)]


STRING_T = 720.0
BASS_W = MASS["bass"][0] * 9.80665
TIP = L.horn_tip(L.body_outline())
FRET5 = L.NUT_S - L.SCALE_LENGTH * (1 - 2 ** (-5 / 12))

BASS_ANCHORS = {
    "bridge-saddles": (L.BRIDGE_S, -0.0372, 0.0),
    "nut": (L.NUT_S + 0.003, -0.0350, 0.0),
    "strap-tail": (-0.013, 0.0, 0.0),
    "strap-horn": (TIP[0] + 0.013, 0.0, TIP[1]),
    "fretting-hand": (FRET5, L.V_NECK_BACK, 0.0),
    "plucking-hand": (0.19, -0.0365, 0.0),
    "body-back": (0.22, 0.0225, -0.02),
    "rest-cradle-l": (L.BASS["s_contact"], 0.0225, L.ARM_W - L.BASS["delta"]),
    "rest-cradle-r": (L.BASS["s_contact"], 0.0225, -L.ARM_W - L.BASS["delta"]),
    "rest-yoke": (L.YOKE_S, L.V_NECK_BACK, 0.0),
}


def string_loads(frame):
    """🎼 The 0.72 kN of the four strings as an equal and opposite pair along the line from the saddles to the nut: it compresses and bends the neck and sums to nothing."""
    a = at(frame, *BASS_ANCHORS["bridge-saddles"])
    b = at(frame, *BASS_ANCHORS["nut"])
    d = [b[i] - a[i] for i in range(3)]
    n = math.sqrt(sum(x * x for x in d))
    f = [STRING_T * x / n for x in d]
    return [
        load("string-pull-bridge", "imposed", T("String tension on the bridge (0.72 kN)", "Saitenzug an der Brücke (0,72 kN)"), "bridge-saddles", f, "daddario-tension-chart"),
        load("string-pull-nut", "imposed", T("String tension at the nut and tuners (0.72 kN)", "Saitenzug an Sattel und Mechaniken (0,72 kN)"), "nut", [-x for x in f], "daddario-tension-chart"),
    ]


def load(lid, kind, title, anchor, f, ref, source="authored-estimate"):
    """🧲 An applied load at an anchor in world newtons."""
    d = {"id": lid, "kind": kind, "title": title, "anchor": anchor, "fxN": round(f[0], 2), "fyN": round(f[1], 2)}
    if abs(f[2]) > 1e-6:
        d["fzN"] = round(f[2], 2)
    d["source"] = source
    d["reference"] = ref
    return d


def com_world(frame):
    """🎯 The centre of mass of the bass in world metres."""
    m = MASS["bass"]
    return at(frame, m[1], m[2], 0.0)


def rest_loads():
    """🪑 The bass resting in the stand: its weight shared by the two foam arms under the body and the foam yoke behind the neck (no friction at the yoke), equal and opposite on bass and stand."""
    fr = bass_world("stored")
    c = com_world(fr)
    cl, cr, y = (at(fr, *BASS_ANCHORS[k]) for k in ("rest-cradle-l", "rest-cradle-r", "rest-yoke"))
    cc = [(cl[i] + cr[i]) / 2 for i in range(3)]
    n = [-x for x in fr[2]]
    Wt = BASS_W
    mom_w = (c[0] - cc[0]) * (-Wt)
    lever = (y[0] - cc[0]) * n[1] - (y[1] - cc[1]) * n[0]
    Ry = -mom_w / lever
    Hc = -n[0] * Ry
    Nc = Wt - n[1] * Ry
    zl, zr = cl[2], cr[2]
    Nl = Nc * (c[2] - zr) / (zl - zr)
    Nr = Nc - Nl
    fy = [n[i] * Ry for i in range(3)]
    out = []
    for key, Nk, title_en, title_de in (("l", Nl, "left cradle arm", "linken Auflagearm"), ("r", Nr, "right cradle arm", "rechten Auflagearm")):
        f = [Hc * Nk / Nc, Nk, 0.0]
        out.append(load(f"cradle-{key}-on-bass", "carried", T(f"Stand carries the bass on the {title_en}", f"Der Ständer trägt den Bass am {title_de}"), f"rest-cradle-{key}", f, "stand-catalogue-typical"))
        out.append(load(f"bass-on-cradle-{key}", "carried", T(f"Bass on the {title_en} of the stand", f"Bass auf dem {title_de} des Ständers"), f"stand-cradle-{key}", [-x for x in f], "stand-catalogue-typical"))
    out.append(load("yoke-on-bass", "carried", T("Foam yoke holds the neck", "Die Schaumgabel hält den Hals"), "rest-yoke", fy, "stand-catalogue-typical"))
    out.append(load("bass-on-yoke", "carried", T("Neck leans on the yoke of the stand", "Der Hals lehnt an der Gabel des Ständers"), "stand-yoke", [-x for x in fy], "stand-catalogue-typical"))
    return out, {"Ry": Ry, "Hc": Hc, "Nl": Nl, "Nr": Nr}


def played_loads():
    """🤘 The bass on its strap: the fretting hand steadies the neck with 3 N, the two strap buttons carry the rest of the weight with no roll moment about its centre of mass; the plucking finger (3 N) and the fretting finger (5 N) press towards the body, the player's belly pushes back (8 N)."""
    fr = bass_world("played")
    c = com_world(fr)
    pts = [at(fr, *BASS_ANCHORS[k]) for k in ("strap-tail", "strap-horn", "fretting-hand")]
    hand = 3.0
    z1, z2, z3 = (p[2] - c[2] for p in pts)
    f2 = (-(BASS_W - hand) * z1 - hand * z3) / (z2 - z1)
    sol = [BASS_W - hand - f2, f2, hand]
    n = [-x for x in fr[2]]
    out = [
        load("strap-tail", "carried", T("Strap at the tail button", "Gurt am hinteren Gurtpin"), "strap-tail", [0, sol[0], 0], "din-33402-2-2020"),
        load("strap-horn", "carried", T("Strap at the horn button", "Gurt am Hornpin"), "strap-horn", [0, sol[1], 0], "din-33402-2-2020"),
        load("fretting-hand-carry", "carried", T("Fretting hand steadies the neck", "Greifhand stützt den Hals"), "fretting-hand", [0, sol[2], 0], "din-33402-2-2020"),
        load("pluck", "live", T("Plucking finger on the string (3 N)", "Zupffinger an der Saite (3 N)"), "plucking-hand", [-3 * x for x in n], "bass-catalogue-typical"),
        load("fret-press", "live", T("Fretting finger presses the string to the fret (5 N)", "Greiffinger drückt die Saite auf den Bund (5 N)"), "fretting-hand", [-5 * x for x in n], "bass-catalogue-typical"),
        load("belly", "live", T("Player's body against the back (8 N)", "Körper des Spielers am Rücken des Basses (8 N)"), "body-back", [8 * x for x in n], "bass-catalogue-typical"),
    ]
    return out, sol


def anchors():
    """📍 Contacts of the castors and stand feet, the load points of strings, strap, hands and stand contacts, the push handle and the fan outlet."""
    out = []
    for c in P.CASTORS:
        side = "left" if "-l" in c else "right"
        pos = "front" if "front" in c else "rear"
        out.append({"id": f"{c}-contact", "bone": c, "x": L.CASTOR_H, "y": -L.TRAIL, "kind": "contact", "title": T(f"Wheel of the {pos} {side} castor on the floor", f"Rad der {'vorderen' if pos == 'front' else 'hinteren'} {'linken' if side == 'left' else 'rechten'} Lenkrolle auf dem Boden"), "support": "contact", "footprintLengthM": 0.012, "footprintWidthM": 0.025, "frictionCoefficient": 0.5})
    sp = P.SP
    for side, sg in (("l", 1), ("r", -1)):
        out.append({"id": f"stand-foot-{side}", "bone": "stand", "x": R3(sp["foot"][0]), "y": 0.0, "z": R3(sg * L.ARM_W), "kind": "contact", "title": T(f"{'Left' if sg > 0 else 'Right'} front foot of the stand", f"{'Linker' if sg > 0 else 'Rechter'} Vorderfuß des Ständers"), "support": "contact", "footprintLengthM": 0.03, "footprintWidthM": 0.022, "frictionCoefficient": 0.7})
    fr = F["stand-rear-leg"]
    end = fr.world(P.REAR_LEG_LENGTH, 0, 0)
    foot = fr.local(end[0], 0.0, end[2])
    out.append({"id": "stand-foot-rear", "bone": "stand-rear-leg", "x": R3(foot[0]), "y": R3(foot[1]), "kind": "contact", "title": T("Foot of the rear leg", "Fuß des Hinterbeins"), "support": "contact", "footprintLengthM": 0.03, "footprintWidthM": 0.022, "frictionCoefficient": 0.7})
    titles = {
        "bridge-saddles": T("Saddles of the bridge", "Saitenreiter der Brücke"),
        "nut": T("Nut", "Sattel"),
        "strap-tail": T("Tail strap button", "Hinterer Gurtpin"),
        "strap-horn": T("Strap button on the horn", "Gurtpin am Horn"),
        "fretting-hand": T("Fretting hand at the fifth fret", "Greifhand am fünften Bund"),
        "plucking-hand": T("Plucking fingers over the pickup", "Zupffinger über dem Tonabnehmer"),
        "body-back": T("Back of the body against the player", "Korpusrücken am Spieler"),
        "rest-cradle-l": T("Body on the left cradle arm", "Korpus auf dem linken Auflagearm"),
        "rest-cradle-r": T("Body on the right cradle arm", "Korpus auf dem rechten Auflagearm"),
        "rest-yoke": T("Neck in the yoke", "Hals in der Gabel"),
    }
    for key, (s, v, w) in BASS_ANCHORS.items():
        a = {"id": key, "bone": "bass", "x": R3(s), "y": R3(v), "kind": "load", "title": titles[key]}
        if abs(w) > 1e-9:
            a["z"] = R3(w)
        out.append(a)
    fb = F["bass"]
    for key, src in (("stand-cradle-l", "rest-cradle-l"), ("stand-cradle-r", "rest-cradle-r"), ("stand-yoke", "rest-yoke")):
        p = fb.world(*BASS_ANCHORS[src])
        q = F["stand"].local(*p)
        a = {"id": key, "bone": "stand", "x": R3(q[0]), "y": R3(q[1]), "z": R3(q[2]), "kind": "load", "title": T({"stand-cradle-l": "Left cradle arm under the body", "stand-cradle-r": "Right cradle arm under the body", "stand-yoke": "Yoke behind the neck"}[key], {"stand-cradle-l": "Linker Auflagearm unter dem Korpus", "stand-cradle-r": "Rechter Auflagearm unter dem Korpus", "stand-yoke": "Gabel hinter dem Hals"}[key])}
        out.append(a)
    out.append({"id": "push-handle", "bone": "cabinet", "x": 0.21, "y": 0.44, "z": -0.318, "kind": "load", "title": T("Right side handle where the cabinet is pushed", "Rechter Seitengriff, an dem die Box geschoben wird")})
    out.append({"id": "fan-outlet", "bone": "head", "x": 0.0, "y": 0.13, "z": 0.15, "kind": "emitter", "title": T("Fan outlet in the rear panel of the head", "Lüfterauslass in der Rückwand des Verstärkers")})
    return out


REGIONS = [
    ("cabinet-carcass", "Cabinet carcass: tolex over plywood, black baffle", "Boxengehäuse: Tolex über Sperrholz, schwarze Schallwand", "surface", "operating", "plywood-tolex", 1.75, 0.66, "vertical-plate", 0.5, [], 17900, ("drive", 12.0), None),
    ("cabinet-grille", "Perforated steel grille in front of the four drive units", "Gelochtes Stahlgitter vor den vier Lautsprechern", "surface", "operating", "grille-steel", 0.75, 0.6, "vertical-plate", 0.5, [("perforated-steel", "Perforated steel 1.5 mm", "Lochblech 1,5 mm", "metal", "grille-steel", 0.0015, 3.0e-05, 2770)], 2770, ("drive", 25.5), ("passive", 0)),
    ("cabinet-hardware", "Ball corners, side handles and jack plate", "Kugelecken, Seitengriffe und Buchsenplatte", "surface", "passive", "chrome-steel", 0.20, 0.05, "vertical-plate", 0.5, [("chrome-shell", "Chrome-plated steel 1 mm", "Verchromter Stahl 1 mm", "metal", "chrome-steel", 0.001, 2.0e-05, 3690)], 3690, None, None),
    ("castors", "Four swivel castors with polyurethane wheels", "Vier Lenkrollen mit Polyurethanrädern", "surface", "passive", "pu-wheel", 0.12, 0.075, "cylinder", 0.2, [], 20000, None, None),
    ("head-casing", "Head sleeve: tolex over plywood, feet and strap handle", "Verstärkergehäuse: Tolex über Sperrholz, Füße und Tragegriff", "surface", "operating", "plywood-tolex", 0.60, 0.3, "vertical-plate", 0.5, [], 14900, ("amplifier", 25.0), None),
    ("head-front-panel", "Front panel with knobs, input and switch", "Frontplatte mit Knöpfen, Eingang und Schalter", "surface", "operating", "aluminium", 0.13, 0.21, "vertical-plate", 0.5, [("anodised-panel", "Aluminium panel 3 mm", "Aluminiumplatte 3 mm", "metal", "aluminium", 0.003, 1.9e-05, 7290)], 7290, ("amplifier", 10.0), None),
    ("head-rear-panel", "Rear panel with fan outlet, heat sink and speaker jacks", "Rückwand mit Lüfterauslass, Kühlkörper und Lautsprecherbuchsen", "surface", "operating", "chassis-steel", 0.11, 0.21, "vertical-plate", 0.5, [("steel-panel", "Steel panel 1.5 mm", "Stahlblech 1,5 mm", "metal", "chassis-steel", 0.0015, 3.0e-05, 5530)], 5530, ("amplifier", 15.0), ("electrical-input", 450)),
    ("bass-body", "Alder body with worn sunburst finish", "Erlenkorpus mit abgenutztem Sunburst-Lack", "surface", "passive", "alder-finish", 0.27, 0.33, "vertical-plate", 0.5, [], 16000, None, ("passive", 0)),
    ("bass-neck", "Maple neck and headstock with rosewood fretboard", "Ahornhals und Kopfplatte mit Palisandergriffbrett", "surface", "passive", "maple", 0.13, 0.05, "cylinder", 0.5, [], 11000, None, None),
    ("bass-hardware", "Bridge, tuners, control plate, knobs, strap buttons and strings", "Brücke, Mechaniken, Kontrollplatte, Knöpfe, Gurtpins und Saiten", "surface", "passive", "chrome-hardware", 0.05, 0.03, "vertical-plate", 0.5, [("chrome-plate", "Chrome-plated steel 1.5 mm", "Verchromter Stahl 1,5 mm", "metal", "chrome-hardware", 0.0015, 3.0e-05, 5530)], 5530, None, None),
    ("bass-pickguard", "Pickguard and pickup covers", "Schlagbrett und Tonabnehmerkappen", "surface", "passive", "pickguard-pvc", 0.06, 0.2, "vertical-plate", 0.5, [("pickguard-sheet", "PVC pickguard 2.3 mm", "PVC-Schlagbrett 2,3 mm", "polymer", "pickguard-pvc", 0.0023, 0.0144, 3100)], 3100, None, None),
    ("stand", "Stand tubes, foam sleeves and rubber caps", "Ständerrohre, Schaumhüllen und Gummikappen", "surface", "passive", "stand-steel", 0.09, 0.016, "cylinder", 0.5, [], 4400, None, None),
]

REF_OF = {"cabinet-carcass": "cabinet-catalogue-typical", "cabinet-grille": "iec-60268-5-2003", "cabinet-hardware": "incropera-2018", "castors": "en-12532-1999", "head-casing": "iec-62368-1-2018", "head-front-panel": "iec-62368-1-2018", "head-rear-panel": "iec-62368-1-2018", "bass-body": "fpl-wood-handbook-2021", "bass-neck": "fpl-wood-handbook-2021", "bass-hardware": "incropera-2018", "bass-pickguard": "iso-10456-2007", "stand": "incropera-2018"}


def thermal_model(areas):
    """🌡️ The thermal model: a dealer's room at 20 degrees C; the head draws 450 W by its plate, at 1/8 power (IEC 62368-1) it releases 115 W (50 W through its casing, 65 W with the fan air), the cabinet 37.5 W of voice-coil heat; the bass, the stand and the castors are passive."""
    regions = []
    for r in REGIONS:
        rid, en, de, kind, model, mat, area, lc, geom, sky, layers, hc, op, rating = r
        d = {"id": rid, "title": T(en, de), "kind": kind, "model": model, "material": mat, "areaM2": round(areas.get(rid, area), 4), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": sky}
        d["layers"] = [{"id": l[0], "title": T(l[1], l[2]), "kind": l[3], "material": l[4], "thicknessM": l[5], "resistanceM2KW": l[6], "heatCapacityJM2K": l[7], "source": "authored-estimate", "reference": "iso-10456-2007"} for l in layers]
        d["heatCapacityJM2K"] = hc
        if op:
            d["operating"] = {"key": op[0], "heatW": op[1]}
        if rating:
            d["ratedPowerW"] = rating[1]
            d["ratedPowerBasis"] = rating[0]
        d["source"] = "authored-estimate"
        d["reference"] = REF_OF[rid]
        regions.append(d)
    return {
        "environment": {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
        "passengers": [],
        "operating": [
            {"id": "amplifier", "title": T("Bass head", "Bassverstärker"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 0.39, "title": T("On without signal (45 W)", "Eingeschaltet ohne Signal (45 W)")}, {"v": 1, "title": T("Played at 1/8 power (115 W heat)", "Gespielt bei 1/8 Leistung (115 W Wärme)")}]},
            {"id": "drive", "title": T("Drive into the cabinet", "Ansteuerung der Box"), "levels": [{"v": 0, "title": T("Silent", "Still")}, {"v": 1, "title": T("37.5 W into the voice coils", "37,5 W in die Schwingspulen")}]},
        ],
        "regions": regions,
        "emitters": [{"id": "head-fan", "kind": "fan", "title": T("Exhaust air of the head's fan", "Abluft des Verstärkerlüfters"), "anchor": "fan-outlet", "directionDeg": 180, "diameterM": 0.06, "operatingKey": "amplifier", "volumeFlowM3S": 0.006, "temperatureRiseK": 9.0, "source": "authored-estimate", "reference": "iec-62368-1-2018"}],
        "assumptions": [
            T("The head's nameplate states 450 W mains input for 300 W into 4 ohm. Played at the normal operating condition of IEC 62368-1 (pink noise at 1/8 of the non-clipped output power, 37.5 W) it draws about 152 W: 37.5 W go to the cabinet and end as voice-coil heat (25.5 W through the grille, 12 W through the carcass), 115 W stay in the head: 25 W through the sleeve, 10 W through the front panel, 15 W through the rear panel and 65 W with the fan air (0.006 m3/s x 1.2 kg/m3 x 1005 J/(kg K) x 9 K). Switched on without signal the head idles at 45 W (amplifier level 0.39).", "Das Typenschild des Verstärkers nennt 450 W Netzaufnahme für 300 W an 4 Ohm. Gespielt im Normalbetrieb nach IEC 62368-1 (rosa Rauschen bei 1/8 der unverzerrten Ausgangsleistung, 37,5 W) nimmt er etwa 152 W auf: 37,5 W gehen in die Box und werden Wärme der Schwingspulen (25,5 W durch das Gitter, 12 W durch das Gehäuse), 115 W bleiben im Verstärker: 25 W durch das Gehäuse, 10 W durch die Frontplatte, 15 W durch die Rückwand und 65 W mit der Lüfterluft (0,006 m3/s x 1,2 kg/m3 x 1005 J/(kg K) x 9 K). Eingeschaltet ohne Signal verbraucht er 45 W im Leerlauf (Verstärkerstufe 0,39)."),
            T("The cabinet is fed from the head and draws nothing itself (passive, 0 W); the bass has passive pickups and draws nothing (passive, 0 W); the total power of the asset is the head's 450 W.", "Die Box wird vom Verstärker gespeist und nimmt selbst nichts auf (passiv, 0 W); der Bass hat passive Tonabnehmer und nimmt nichts auf (passiv, 0 W); die Gesamtleistung des Assets sind die 450 W des Verstärkers."),
            T("Played, the back of the body touches the player over 12 % of its area and the neck the fretting hand over 8 %, both at 33 degrees C skin temperature; aged finishes and tarnished chrome are taken with the emissivities of worn polyurethane (0.9) and tarnished chrome (0.15).", "Gespielt liegt der Korpusrücken auf 12 % seiner Fläche am Spieler und der Hals auf 8 % an der Greifhand an, beide mit 33 Grad C Hauttemperatur; gealterte Lacke und angelaufenes Chrom sind mit den Emissionsgraden von abgenutztem Polyurethan (0,9) und angelaufenem Chrom (0,15) angesetzt."),
        ],
    }


REGION_IDS = [r[0] for r in REGIONS]


def pose_thermal(pose):
    """🌡️ The thermal binding of a pose: every region and the fan emitter, the operating levels and the contact of the played bass with the player."""
    regions = {}
    for rid in REGION_IDS:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if rid == "castors":
            b = {"exposedFraction": 0.95, "sunlitFraction": 0, "contact": {"fraction": 0.03, "tempC": 20}}
        if pose == "played" and rid == "bass-body":
            b = {"exposedFraction": 0.88, "sunlitFraction": 0, "contact": {"fraction": 0.12, "tempC": 33}}
        if pose == "played" and rid == "bass-neck":
            b = {"exposedFraction": 0.92, "sunlitFraction": 0, "contact": {"fraction": 0.08, "tempC": 33}}
        regions[rid] = b
    on = pose == "played"
    return {"operating": {"amplifier": 1 if on else 0, "drive": 1 if on else 0}, "regions": regions, "emitters": {"head-fan": 1 if on else 0}}


CONTACTS = [f"{c}-contact" for c in P.CASTORS] + ["stand-foot-l", "stand-foot-r", "stand-foot-rear"]


def poses():
    """🧍 Stored at the dealer, cabinet rolled, bass played standing."""
    rest, _ = rest_loads()
    played, _ = played_loads()
    stored = {
        "id": "stored", "title": T("Stored at the dealer", "Im Laden abgestellt"),
        "description": T("The stack stands switched off on its four castors with the head on the cabinet; the bass leans in its folding stand to the right, its body on the two foam arms and its neck in the yoke, strings tuned to pitch.", "Der Turm steht ausgeschaltet auf seinen vier Lenkrollen, der Verstärker auf der Box; rechts lehnt der Bass in seinem Klappständer, der Korpus auf den zwei Schaumarmen, der Hals in der Gabel, die Saiten gestimmt."),
        "bones": {}, "ground": "auto", "contacts": CONTACTS, "thermal": pose_thermal("stored"), "loads": string_loads(bass_world("stored")) + rest,
    }
    wheels = {c.replace("castor", "wheel"): {"angleDeg": R3(WHEEL_TURN_DEG)} for c in P.CASTORS}
    carried = {
        "id": "carried", "title": T("Cabinet rolled", "Box gerollt"),
        "description": T("The cabinet with the head on top is pushed 0.6 m forward on its castors by its right side handle (20 N against the rolling resistance); the 75 mm wheels turn 2.5 times; the bass stays in its stand.", "Die Box mit dem Verstärker darauf wird am rechten Seitengriff 0,6 m auf ihren Rollen nach vorn geschoben (20 N gegen den Rollwiderstand); die 75-mm-Räder drehen sich 2,5-mal; der Bass bleibt in seinem Ständer."),
        "bones": {"cabinet-roll": {"dx": ROLL_M}, **wheels}, "ground": "auto", "contacts": CONTACTS, "thermal": pose_thermal("carried"),
        "loads": string_loads(bass_world("stored")) + rest + [load("push", "live", T("Push on the side handle (20 N)", "Schub am Seitengriff (20 N)"), "push-handle", [20.0, 0, 0], "en-12532-1999")],
    }
    play = {
        "id": "played", "title": T("Played standing", "Im Stehen gespielt"),
        "description": T("The bass hangs on its strap in front of the stack with the body centre 1.0 m above the floor and the neck 24 degrees above horizontal towards the player's left; the head drives the cabinet at 1/8 power and its fan blows warm air out of the back; the player is not part of the asset, his strap, hands and belly are loads.", "Der Bass hängt am Gurt vor dem Turm, die Korpusmitte 1,0 m über dem Boden, der Hals 24 Grad über der Waagrechten zur linken Hand des Spielers; der Verstärker treibt die Box mit 1/8 Leistung, sein Lüfter bläst warme Luft nach hinten; der Spieler gehört nicht zum Asset, Gurt, Hände und Bauch sind Lasten."),
        "bones": {"bass-slide": {"dx": PLAY_DX}, "bass-lift": {"dy": PLAY_DY}, "bass": {"abductionDeg": PLAY_ABD}}, "ground": "auto", "contacts": CONTACTS, "thermal": pose_thermal("played"),
        "loads": string_loads(bass_world("played")) + played,
    }
    return [stored, carried, play]


def clips():
    """🎞️ Rolling the cabinet and a plucked groove on the strap."""
    spans = [{"anchor": a, "fromS": 0, "toS": 4} for a in CONTACTS]
    carry = {
        "id": "carry", "title": T("Roll the cabinet", "Box rollen"),
        "description": T("From the stored stack the cabinet with its head is pushed 0.6 m forward on its castors in four seconds; the wheels roll without slipping, the bass stays in its stand.", "Aus dem abgestellten Turm wird die Box mit dem Verstärker in vier Sekunden 0,6 m auf ihren Rollen nach vorn geschoben; die Räder rollen ohne zu gleiten, der Bass bleibt im Ständer."),
        "durationS": 4, "fps": 25, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 4, "pose": "carried", "ease": "smooth"}],
        "contacts": spans, "phases": [{"id": "push", "fromS": 0, "toS": 4, "title": T("Pushing", "Schieben")}],
        "events": [{"t": 0, "kind": "motion", "title": T("Push starts", "Schieben beginnt"), "target": "cabinet-roll"}, {"t": 4, "kind": "motion", "title": T("Cabinet stops", "Box steht"), "target": "cabinet-roll"}],
        "tracks": [],
    }
    dur = 2.4
    lift = lambda d: {"dy": R3(PLAY_DY + d)}
    keys = [
        {"t": 0, "pose": "played", "ease": "smooth"},
        {"t": 0.6, "pose": "played", "ease": "smooth", "bones": {"bass-lift": lift(0.006), "bass": {"abductionDeg": 63.5, "angleDeg": 1.2}}},
        {"t": 1.2, "pose": "played", "ease": "smooth"},
        {"t": 1.8, "pose": "played", "ease": "smooth", "bones": {"bass-lift": lift(-0.004), "bass": {"abductionDeg": 66.5, "angleDeg": -1.0}}},
        {"t": dur, "pose": "played", "ease": "smooth"},
    ]
    pluck = [{"t": round(0.3 * k + d, 3), "v": v} for k in range(8) for d, v in ((0.0, 1.0), (0.08, 0.2))] + [{"t": dur, "v": 1.0}]
    groove = {
        "id": "groove", "title": T("Plucked groove on the strap", "Gezupfter Groove am Gurt"),
        "description": T("Eighth notes at 100 beats per minute: the plucking finger strikes every 0.3 s, the bass sways a degree or two and bobs 6 mm on its strap with the player's body, the head runs at 1/8 power.", "Achtelnoten bei 100 Schlägen pro Minute: der Zupffinger schlägt alle 0,3 s an, der Bass schwingt mit dem Körper des Spielers ein bis zwei Grad und federt 6 mm am Gurt, der Verstärker läuft mit 1/8 Leistung."),
        "durationS": dur, "fps": 25, "loop": True, "rootMotion": "none", "keys": keys,
        "contacts": [{"anchor": a, "fromS": 0, "toS": dur} for a in CONTACTS],
        "phases": [{"id": "bar", "fromS": 0, "toS": dur, "title": T("One bar of eighth notes", "Ein Takt Achtelnoten")}],
        "events": [{"t": round(0.3 * k, 3), "kind": "load-on", "title": T("Note plucked", "Ton angezupft"), "target": "pluck"} for k in range(8)],
        "tracks": [{"target": "load:pluck", "ease": "linear", "keys": pluck}],
    }
    return [carry, groove]


PENS = {
    50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5},
    100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
    200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25},
    500: {"body": 0.25, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = {
    50: [{"id": "grille-mesh", "title": T("Perforated grille", "Lochblechgitter"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}],
    100: [{"id": "grille-mesh", "title": T("Perforated grille", "Lochblechgitter"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.45, "penMm": 0.1}],
    200: [],
    500: [],
}


def nearness(view, corners):
    """📏 How near a part comes to the viewer of a view (larger is nearer): the elevation looks from the right (-z), the front from ahead (+x), the plan from above (+y)."""
    if not corners:
        return -9
    if view == "elevation":
        return max(-p[2] for p in corners)
    if view == "front":
        return max(p[0] for p in corners)
    return max(p[1] for p in corners)


def bass_face_details(scale):
    """🎨 The traces of use and the drawn detail on the face of the bass, placed on the bass bone with a `solid` sweep so they follow it when it is played: frets, inlay dots, the forearm wear through the finish, the worn fretboard at the first frets, pickguard scratches and the two screw holes of a removed finger rest."""
    out = []
    vt = L.V_BOARD_TOP
    sw = lambda v0, v1: {"kind": "extruded", "sweep": "v", "z0": R3(v0), "z1": R3(v1)}
    if scale == 50:
        frets = []
        for n in range(1, 21):
            s = L.NUT_S - L.SCALE_LENGTH * (1 - 2 ** (-n / 12))
            h = L.neck_half(s)
            frets.append([(s - 0.0006, -h), (s + 0.0006, -h), (s + 0.0006, h), (s - 0.0006, h)])
        out.append(("frets", "texture", 0.85, False, "bass-neck", G.path(frets), sw(vt - 0.0002, vt)))
        for n in (3, 5, 7, 9, 15, 17, 19):
            s = L.NUT_S - L.SCALE_LENGTH * (1 - 2 ** (-(n - 0.5) / 12))
            out.append((f"dot-{n}", "detail", 0.9, True, "bass-neck", G.poly(G.circle_points(s, 0.0, 0.0032, 10)), sw(vt - 0.0002, vt)))
        s12 = L.NUT_S - L.SCALE_LENGTH * (1 - 2 ** (-11.5 / 12))
        for k, w in enumerate((-0.012, 0.012)):
            out.append((f"dot-12-{k + 1}", "detail", 0.9, True, "bass-neck", G.poly(G.circle_points(s12, w, 0.0032, 10)), sw(vt - 0.0002, vt)))
        s1 = L.NUT_S - L.SCALE_LENGTH * (1 - 2 ** (-0.2 / 12))
        s5 = L.NUT_S - L.SCALE_LENGTH * (1 - 2 ** (-5 / 12))
        out.append(("fretboard-wear", "detail", 0.4, True, "bass-neck", G.poly([(s5, -0.016), (s1, -0.012), (s1, 0.010), (s5, 0.014)]), sw(vt - 0.0003, vt - 0.0001)))
        out.append(("forearm-wear", "detail", 0.66, True, "bass-body", G.poly([(0.035, -0.128), (0.080, -0.158), (0.150, -0.164), (0.205, -0.148), (0.190, -0.133), (0.140, -0.146), (0.085, -0.140), (0.050, -0.118)]), sw(-0.0228, -0.0226)))
        out.append(("pickguard-scratches", "texture", 0.6, False, "bass-pickguard", G.path([[(0.230, -0.090), (0.290, -0.070), (0.291, -0.069), (0.231, -0.089)], [(0.245, -0.050), (0.300, -0.055), (0.300, -0.054), (0.245, -0.049)], [(0.160, 0.040), (0.210, 0.062), (0.209, 0.063), (0.159, 0.041)]]), sw(-0.0250, -0.0249)))
        for k, s in enumerate((0.246, 0.292)):
            out.append((f"finger-rest-hole-{k + 1}", "detail", 0.05, True, "bass-pickguard", G.poly(G.circle_points(s, 0.050, 0.0016, 8)), sw(-0.0250, -0.0249)))
        out.append(("edge-dings", "texture", 0.7, False, "bass-body", G.path([[(0.300, 0.118), (0.312, 0.117), (0.312, 0.116), (0.300, 0.117)], [(0.420, -0.124), (0.432, -0.121), (0.432, -0.120), (0.420, -0.123)]]), sw(-0.0228, -0.0226)))
    elif scale == 100:
        out.append(("fretboard", "detail", 0.22, True, "bass-neck", G.poly(L.neck_outline(L.BOARD_END_S, L.NUT_S + 0.003, 3)), sw(vt - 0.0002, vt)))
        out.append(("forearm-wear", "detail", 0.66, True, "bass-body", G.poly([(0.035, -0.128), (0.090, -0.160), (0.180, -0.158), (0.170, -0.140), (0.090, -0.142), (0.050, -0.118)]), sw(-0.0228, -0.0226)))
    elif scale == 200:
        out.append(("fretboard", "detail", 0.22, True, "bass-neck", G.poly(L.neck_outline(L.BOARD_END_S, L.NUT_S, 2)), sw(-0.0377, -0.0375)))
    return out


def cabinet_details(scale):
    """🩹 The drawn traces on the cabinet and head: speaker cones behind the grille, the rental sticker, the torn tolex at the front top edge, the handle recesses, the printed legend band of the head panel."""
    out = []
    cab = F["cabinet"]
    hd = F["head"]
    if scale in (50, 100):
        for k, (w, v) in enumerate(((0.142, 0.183), (-0.142, 0.183), (0.142, 0.477), (-0.142, 0.477))):
            c = cab.world(0.42, v, w)
            out.append(("front", f"cone-{k + 1}", "cabinet", "detail", 0.1, False, "cabinet-grille", G.ellipse(c[2], c[1], 0.118, 0.118)))
        out.append(("elevation", "sticker", "cabinet", "detail", 0.85, True, "cabinet-carcass", G.rect(0.06, 0.16, 0.12, 0.19)))
        out.append(("elevation", "handle-recess-r", "cabinet", "detail", 0.05, True, "cabinet-hardware", G.rect(0.15, 0.27, 0.415, 0.465, 0.004)))
    if scale == 50:
        out.append(("elevation", "torn-tolex", "cabinet", "detail", 0.58, True, "cabinet-carcass", G.poly([(0.325, 0.66), (0.405, 0.66), (0.40, 0.645), (0.37, 0.636), (0.34, 0.648)])))
        out.append(("plan", "torn-tolex-top", "cabinet", "detail", 0.58, True, "cabinet-carcass", G.poly([(cab.world(0.325, 0, 0)[0], -0.31), (cab.world(0.405, 0, 0)[0], -0.31), (cab.world(0.40, 0, 0)[0], -0.27), (cab.world(0.36, 0, 0)[0], -0.255), (cab.world(0.335, 0, 0)[0], -0.285)])))
        out.append(("front", "grille-dent", "cabinet", "texture", 0.2, False, "cabinet-grille", G.poly([(0.20, 0.33), (0.222, 0.322), (0.245, 0.33)], closed=False)))
        legend = [hd.world(0.290, 0.19, -0.24), hd.world(0.290, 0.212, 0.24)]
        out.append(("front", "panel-legend", "head", "detail", 0.55, True, "head-front-panel", G.rect(legend[0][2], legend[1][2], legend[0][1], legend[1][1])))
        led = hd.world(0.290, 0.112, 0.235)
        out.append(("front", "power-led", "head", "detail", 0.95, True, "head-front-panel", G.ellipse(led[2], led[1], 0.003, 0.003)))
        out.append(("elevation", "head-scuff", "head", "detail", 0.5, True, "head-casing", G.poly([(0.262, 0.250), (0.300, 0.250), (0.300, 0.222), (0.285, 0.232)])))
    return out


def shapes(scale):
    """✏️ The three drawings of a scale: every part drawn by its own numbers, ordered by nearness to the viewer, with the traces of use on top of the part they mark."""
    geo = {}
    for view in VIEWS:
        items = []
        for part in PARTS[scale]:
            g = part.views.get(view)
            if g is None:
                continue
            items.append((nearness(view, part.corners), part, g))
        items.sort(key=lambda t: t[0])
        out = []
        for rank, (_, part, g) in enumerate(items):
            sh = {"id": part.id, "bone": part.bone, "role": part.role, "z": round(3 + 0.01 * rank, 3), "filled": part.flags.get("filled", True), "tone": part.tone}
            if part.extra.get("regions"):
                sh["regions"] = part.extra["regions"]
            else:
                sh["region"] = part.region
            sh["material"] = part.material
            if view == "front" and part.id == "grille":
                sh["hatch"] = "grille-mesh"
            sh.update(g)
            out.append(sh)
        top = 3 + 0.01 * len(items)
        k = 0
        for d in cabinet_details(scale):
            if d[0] != view:
                continue
            k += 1
            sh = {"id": d[1], "bone": d[2], "role": d[3], "z": round(top + 0.001 * k, 4), "filled": d[5], "tone": d[4], "region": d[6]}
            if d[1] == "cone-1" or d[1].startswith("cone"):
                pass
            sh.update(d[7])
            out.append(sh)
        if view == "front":
            for d in bass_face_details(scale):
                k += 1
                sh = {"id": d[0], "bone": "bass", "role": d[1], "z": round(top + 0.001 * k + 0.05, 4), "filled": d[3], "tone": d[2], "region": d[4]}
                sh.update(d[5])
                sh["solid"] = d[6]
                out.append(sh)
        if view == "front" and scale == 100:
            for sh in out:
                if sh["id"] == "carcass":
                    sh["hatch"] = "grille-mesh"
        geo[view] = out
    return geo


def solid_doc(part):
    """🧊 A body solid of a part."""
    d = {"id": part.id, "bone": part.bone, "material": part.material, "role": part.role, "tone": part.tone}
    for flag in ("slender", "sheet"):
        if part.flags.get(flag):
            d[flag] = True
    d.update(part.solid)
    return d


def box_solid(sid, bone, material, role, tone, lo, hi, **extra):
    """⬛ A bare box solid from local corners."""
    d = {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": [R3(hi[i] - lo[i]) for i in range(3)], "transform": {"translation": [R3((hi[i] + lo[i]) / 2) for i in range(3)]}}
    d.update(extra)
    return d


def prism_solid(sid, material, role, tone, profile, v0, v1, **extra):
    """🎸 A bare bass prism solid (profile in s, w; along v)."""
    d = {"id": sid, "kind": "extrude", "bone": "bass", "material": material, "role": role, "tone": tone, "profile": G.p2([(w, s) for s, w in profile]), "plane": "zx", "from": R3(v0), "to": R3(v1)}
    d.update(extra)
    return d


ALL_FACES = ["+u", "-u", "+v", "-v", "+w", "-w"]


def thermal_list(scale):
    """🌡️ The thermal model of a scale: every body part as the surface of its region, the thin regions a coarser scale merges as skins of their real layer, and the fan outlet as a patch in the rear panel."""
    out = [{"id": f"th-{p.id}", "of": p.id, "region": p.region} for p in PARTS[scale]]
    half_t = 0.0375 if scale == 500 else 0.0225
    if scale != 50:
        out.append(box_solid("th-grille-skin", "cabinet", "grille-steel", "detail", 0.3, (0.42, 0.028, -0.282), (0.4215, 0.632, 0.282), region="cabinet-grille", layer="perforated-steel"))
        out.append(box_solid("th-front-panel-skin", "head", "aluminium", "detail", 0.72, (0.30, 0.025, -0.26), (0.303, 0.235, 0.26), region="head-front-panel", layer="anodised-panel", sectionFaces=["-u"]))
        out.append(box_solid("th-rear-panel-skin", "head", "chassis-steel", "detail", 0.35, (-0.0015, 0.025, -0.26), (0.0, 0.235, 0.26), region="head-rear-panel", layer="steel-panel", sectionFaces=["+u"]))
    if scale in (200, 500):
        out.append(prism_solid("th-bridge-skin", "chrome-hardware", "detail", 0.75, P.rrect(0.055, 0.100, -0.037, 0.037, 0.004), -half_t - 0.0015, -half_t, region="bass-hardware", layer="chrome-plate"))
        back = -0.0075 if scale == 200 else 0.0245
        out.append(prism_solid("th-tuner-skin", "chrome-hardware", "detail", 0.75, P.rrect(0.972, 1.150, -0.095, -0.030, 0.006), back, back + 0.0015, region="bass-hardware", layer="chrome-plate"))
    if scale == 500:
        out.append(prism_solid("th-pickguard-skin", "pickguard-pvc", "detail", 0.28, L.PICKGUARD, -half_t - 0.0023, -half_t, region="bass-pickguard", layer="pickguard-sheet"))
        n = 0
        for cu in (0.0, L.CAB_D - 0.05):
            for vv in ((L.CAB_H, L.CAB_H + 0.001), (-0.001, 0.0)):
                for cw in (-L.CAB_W / 2, L.CAB_W / 2 - 0.05):
                    n += 1
                    out.append(box_solid(f"th-corner-skin-{n}", "cabinet", "chrome-steel", "detail", 0.68, (cu, vv[0], cw), (cu + 0.05, vv[1], cw + 0.05), region="cabinet-hardware", layer="chrome-shell"))
    u0 = 0.011 if scale == 50 else -0.0025
    out.append(box_solid("th-fan-outlet", "head", "chassis-steel", "texture", 0.3, (u0, 0.10, 0.12), (u0 + 0.001, 0.16, 0.18), tags=["head-fan"], region="head-rear-panel", sectionFaces=ALL_FACES))
    return out


def structure_list(scale):
    """🏗️ The load path of a scale: the stand tubes and foam that carry the bass to its three feet, the 18 mm plywood shell of the cabinet on its castors, the head's sleeve and chassis on its feet, the bass body and neck with the bolted neck joint and the strings, supports at every contact and the masses at their centres."""
    f = P.FEATURE[scale]
    out = []
    names = {p.id for p in PARTS[scale]}
    for p in PARTS[scale]:
        if p.bone in ("stand", "stand-rear-leg") and "cap" not in p.id:
            out.append({"id": f"st-{p.id}", "of": p.id, "member": p.bone, "part": "member"})
    rs = max(0.006, 0.55 * f)
    sp = P.SP
    for side, sg in (("l", 1), ("r", -1)):
        out.append({"id": f"support-stand-foot-{side}", "kind": "ellipsoid", "bone": "stand", "material": "rubber", "role": "ground", "tone": 0.1, "radii": [rs, rs, rs], "transform": {"translation": [R3(sp["foot"][0]), R3(rs), R3(sg * L.ARM_W)]}, "member": "stand", "part": "support", "anchor": f"stand-foot-{side}"})
    fr = F["stand-rear-leg"]
    end = fr.world(P.REAR_LEG_LENGTH, 0, 0)
    c = fr.local(end[0], rs, end[2])
    out.append({"id": "support-stand-foot-rear", "kind": "ellipsoid", "bone": "stand-rear-leg", "material": "rubber", "role": "ground", "tone": 0.1, "radii": [rs, rs, rs], "transform": {"translation": [R3(c[0]), R3(c[1]), 0.0]}, "member": "stand-rear-leg", "part": "support", "anchor": "stand-foot-rear"})
    t = 0.018
    hw, h, d = L.CAB_W / 2, L.CAB_H, L.CAB_D
    sheet = {"sheet": True} if scale in (200, 500) else {}
    panels = [
        ("bottom", (0, 0, -hw), (d, t, hw)), ("top", (0, h - t, -hw), (d, h, hw)),
        ("side-l", (0, t, hw - t), (d, h - t, hw)), ("side-r", (0, t, -hw), (d, h - t, -hw + t)),
        ("back", (0, t, -hw + t), (t, h - t, hw - t)), ("baffle", (0.385, t, -hw + t), (0.385 + t, h - t, hw - t)),
    ]
    for name, lo, hi in panels:
        out.append(box_solid(f"st-cabinet-{name}", "cabinet", "plywood-tolex", "body", 0.5, lo, hi, member="cabinet", part="member", **sheet))
    for cst in P.CASTORS:
        cid = cst.replace("castor-", "")
        wl = cst.replace("castor", "wheel")
        if scale == 50:
            out.append({"id": f"st-castor-plate-{cid}", "of": f"castor-plate-{cid}", "member": cst, "part": "joint"})
            out.append({"id": f"st-castor-fork-{cid}", "of": f"castor-fork-{cid}", "member": cst, "part": "member"})
            out.append({"id": f"st-wheel-{cid}", "of": f"wheel-{cid}", "member": wl, "part": "member"})
        elif scale == 100:
            out.append({"id": f"st-castor-fork-{cid}", "of": f"castor-fork-{cid}", "member": cst, "part": "member"})
            out.append({"id": f"st-wheel-{cid}", "of": f"wheel-{cid}", "member": wl, "part": "member"})
        else:
            out.append({"id": f"st-castor-{cid}", "of": f"castor-{cid}", "member": cst, "part": "member"})
        a = max(f, 0.012)
        out.append(box_solid(f"support-{cst}", cst, "pu-wheel", "ground", 0.1, (L.CASTOR_H - f, -L.TRAIL - a / 2, -a / 2), (L.CASTOR_H, -L.TRAIL + a / 2, a / 2), member=cst, part="support", anchor=f"{cst}-contact"))
    ft = L.HEAD_FEET if scale == 50 else 0.0
    ht = 0.015 if scale in (50, 100) else 0.014
    hsheet = {"sheet": True} if scale in (100, 200, 500) else {}
    out.append(box_solid("st-head-bottom", "head", "plywood-tolex", "body", 0.5, (0.0, ft, -L.HEAD_W / 2), (L.HEAD_D, ft + ht, L.HEAD_W / 2), member="head", part="member", **hsheet))
    out.append(box_solid("st-head-chassis", "head", "chassis-steel", "body", 0.5, (0.02, ft + ht, -0.255), (0.282, ft + ht + 0.0015, 0.255), member="head", part="member", sheet=True))
    if scale == 50:
        for i in range(4):
            out.append({"id": f"st-head-foot-{i + 1}", "of": f"head-foot-{i + 1}", "member": "head", "part": "joint"})
    for pid in ("bass-body", "neck", "fretboard", "headstock"):
        if pid in names:
            out.append({"id": f"st-{pid}", "of": pid, "member": "bass", "part": "member"})
    if scale == 50:
        for i in range(4):
            out.append({"id": f"st-string-{i + 1}", "of": f"string-{i + 1}", "member": "bass", "part": "member"})
    jt = {50: (-0.0225, -0.0075), 100: (-0.0225, -0.0075), 200: (-0.0375, -0.0075), 500: (-0.0375, 0.0375)}[scale]
    jw = 0.0375 if scale == 500 else 0.025
    out.append(prism_solid("st-neck-joint", "maple", "detail", 0.4, [(0.36, -jw), (0.44 if scale == 500 else 0.42, -jw), (0.44 if scale == 500 else 0.42, jw), (0.36, jw)], jt[0], jt[1], member="bass", part="joint"))
    ms = max(f, 0.03)
    for bone, (m, cx, cy) in (("cabinet", MASS["cabinet"][:3]), ("head", MASS["head"][:3]), ("bass", MASS["bass"][:3]), ("stand", MASS["stand"][:3])):
        out.append(box_solid(f"mass-{bone}", bone, "chassis-steel", "detail", 0.15, (cx - ms / 2, cy - ms / 2, -ms / 2), (cx + ms / 2, cy + ms / 2, ms / 2), member=bone, part="mass"))
    return out


REP_DECISIONS = {
    50: {
        "silhouette": T("The stack and the bass as built, 26 mm tall on paper: the cabinet 0.42 m deep with its recessed grille, ball corners and handles on four castors with forks and wheels, the head with sleeve, recessed panels, knobs, feet and strap handle, and beside it the bass leaning 12 degrees back in its A-frame stand, drawn by the same profiles as its solids so that it stays right when it is played.", "Turm und Bass wie gebaut, auf Papier 26 mm hoch: die 0,42 m tiefe Box mit eingelassenem Gitter, Kugelecken und Griffen auf vier Lenkrollen mit Gabeln und Rädern, der Verstärker mit Gehäuse, eingelassenen Platten, Knöpfen, Füßen und Tragegriff, daneben der Bass, 12 Grad zurückgelehnt im A-Ständer, mit denselben Profilen gezeichnet wie seine Körper, damit er auch gespielt stimmt."),
        "retained": T("The 18 mm front frame, the 12 mm grille recessed 4 mm, eight 50 mm ball corners, the 8 mm handle flanges, the jack plate, castor plates, forks and 75 mm wheels; nine knobs, input and switch of the head; body, pickguard, neck, fretboard, headstock, four tuners and posts, bridge, control plate, two knobs, split pickup and both strap buttons of the bass; frame tubes, cross tube, post, cradle arms with foam and lips, yoke with its arms, rear leg and rubber caps of the stand.", "Der 18-mm-Frontrahmen, das 12-mm-Gitter 4 mm zurückgesetzt, acht 50-mm-Kugelecken, die 8-mm-Griffschalen, die Buchsenplatte, Rollenplatten, Gabeln und 75-mm-Räder; neun Knöpfe, Eingang und Schalter des Verstärkers; Korpus, Schlagbrett, Hals, Griffbrett, Kopfplatte, vier Mechaniken und Stifte, Brücke, Kontrollplatte, zwei Knöpfe, geteilter Tonabnehmer und beide Gurtpins des Basses; Rahmenrohre, Querrohr, Stütze, Auflagearme mit Schaum und Lippen, Gabel mit Armen, Hinterbein und Gummikappen des Ständers."),
        "merged": T("Each castor fork with its 2.5 mm cheeks is one block around the wheel; tuner housing and key are one part; the saddles stand in the bridge block; the four drive units stay behind the grille as drawn cones.", "Jede Rollengabel mit ihren 2,5-mm-Wangen ist ein Block um das Rad; Gehäuse und Flügel jeder Mechanik sind ein Teil; die Reiter stehen im Brückenblock; die vier Lautsprecher bleiben als gezeichnete Membranen hinter dem Gitter."),
        "omitted": T("Screws, cables, the strap (it belongs to the player) and the speaker wiring.", "Schrauben, Kabel, der Gurt (er gehört dem Spieler) und die Lautsprecherverkabelung."),
        "thinElements": T("The four strings and twenty frets are texture strokes of 0.1 mm, the speaker cones detail circles of 0.18 mm, the 2.3 mm pickguard a strip at its true thickness.", "Die vier Saiten und zwanzig Bünde sind Texturstriche von 0,1 mm, die Lautsprechermembranen Detailkreise von 0,18 mm, das 2,3-mm-Schlagbrett ein Streifen in wahrer Dicke."),
        "hatching": T("The grille carries a crossed hatch of 0.5 mm for the perforated steel.", "Das Gitter trägt eine Kreuzschraffur von 0,5 mm für das Lochblech."),
        "styles": T("Tolex 0.18, grille 0.3, chrome 0.55 to 0.75, castors 0.28 and 0.6, panel 0.72, sunburst body 0.42, maple 0.78, rosewood 0.22, stand 0.15; body pen 0.35 mm, detail 0.18 mm, texture 0.1 mm.", "Tolex 0,18, Gitter 0,3, Chrom 0,55 bis 0,75, Rollen 0,28 und 0,6, Frontplatte 0,72, Sunburst-Korpus 0,42, Ahorn 0,78, Palisander 0,22, Ständer 0,15; Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,1 mm."),
        "overlays": T("Traces of use: the finish worn through at the forearm on the lower bout, the darkened fretboard at the first five frets, scratches and the two screw holes of a removed finger rest on the pickguard, dings on the body edge, torn tolex at the front top edge of the cabinet, a dented grille, an old rental sticker, scuffed head corners.", "Gebrauchsspuren: der Lack am Unterarm auf der unteren Zarge durchgescheuert, das dunkle Griffbrett an den ersten fünf Bünden, Kratzer und die zwei Schraublöcher einer entfernten Fingerstütze auf dem Schlagbrett, Macken an der Korpuskante, eingerissenes Tolex an der vorderen Oberkante der Box, ein eingedrücktes Gitter, ein alter Verleihaufkleber, abgestoßene Verstärkerecken."),
    },
    100: {
        "silhouette": T("The stack and the bass 13 mm tall on paper: cabinet, corners and castors, the head as one sleeve with its handle, the bass with body, pickguard, neck, headstock, tuners, bridge, knobs and pickup in its stand.", "Turm und Bass auf Papier 13 mm hoch: Box, Ecken und Rollen, der Verstärker als ein Gehäuse mit Griff, der Bass mit Korpus, Schlagbrett, Hals, Kopfplatte, Mechaniken, Brücke, Knöpfen und Tonabnehmer in seinem Ständer."),
        "retained": T("Height 1.32 m, the 0.66 m cabinet on 95 mm castors, the 0.25 m head with a 15 mm handle, the 45 mm body and 26 mm neck, the 16 mm stand tubes and the 24 mm foam.", "Höhe 1,32 m, die 0,66 m hohe Box auf 95-mm-Rollen, der 0,25 m hohe Verstärker mit 15-mm-Griff, der 45-mm-Korpus und der 26-mm-Hals, die 16-mm-Ständerrohre und der 24-mm-Schaum."),
        "merged": T("The grille and the front frame are the front of the carcass with the grille hatch; castor plate and fork are one block; fretboard and neck are one part with the fretboard drawn as a dark field; both pickup halves are one block; the head's panels are faces of its sleeve.", "Gitter und Frontrahmen sind die Front des Gehäuses mit der Gitterschraffur; Rollenplatte und Gabel sind ein Block; Griffbrett und Hals sind ein Teil, das Griffbrett als dunkles Feld gezeichnet; beide Tonabnehmerhälften sind ein Block; die Platten des Verstärkers sind Flächen seines Gehäuses."),
        "omitted": T("Knobs and feet of the head, handle flanges and jack plate, posts, control plate, strap buttons, strings and frets, the caps' inner detail.", "Knöpfe und Füße des Verstärkers, Griffschalen und Buchsenplatte, Stifte, Kontrollplatte, Gurtpins, Saiten und Bünde."),
        "thinElements": T("Speaker cones as detail circles of 0.13 mm; the pickguard at its true 2.3 mm.", "Lautsprechermembranen als Detailkreise von 0,13 mm; das Schlagbrett in seinen wahren 2,3 mm."),
        "hatching": T("The cabinet front carries the crossed grille hatch of 0.45 mm.", "Die Boxenfront trägt die Kreuzschraffur des Gitters von 0,45 mm."),
        "styles": T("As 1:50; body pen 0.25 mm, detail 0.13 mm.", "Wie 1:50; Körperstift 0,25 mm, Detail 0,13 mm."),
        "overlays": T("The forearm wear on the body, the rental sticker and the handle recesses.", "Die Abnutzung am Unterarm auf dem Korpus, der Verleihaufkleber und die Griffmulden."),
    },
    200: {
        "silhouette": T("A block 3.3 mm wide with a smaller block on top and four castor feet, beside it the bass 6.6 mm tall in three stand legs.", "Ein Block von 3,3 mm Breite mit einem kleineren Block darauf und vier Rollenfüßen, daneben der 6,6 mm hohe Bass in drei Ständerbeinen."),
        "retained": T("Cabinet, eight corners, four castors, the head with its handle block, the bass body with pickguard and one neck with headstock, the stand with its true 16 mm frame tubes and rear leg.", "Box, acht Ecken, vier Rollen, der Verstärker mit seinem Griffblock, der Basskorpus mit Schlagbrett und ein Hals mit Kopfplatte, der Ständer mit seinen wahren 16-mm-Rahmenrohren und dem Hinterbein."),
        "merged": T("Each castor is one block of its wheel and fork; neck, fretboard, headstock and tuner keys are one 30 mm part; the cradle arms, post and yoke are 30 mm bars; grille, panels and hardware are faces of their parts.", "Jede Rolle ist ein Block aus Rad und Gabel; Hals, Griffbrett, Kopfplatte und Mechanikflügel sind ein 30-mm-Teil; Auflagearme, Stütze und Gabel sind 30-mm-Stäbe; Gitter, Platten und Beschläge sind Flächen ihrer Teile."),
        "omitted": T("Bridge, knobs and pickup of the bass, lips and yoke arms of the stand, all traces of use smaller than 30 mm.", "Brücke, Knöpfe und Tonabnehmer des Basses, Lippen und Gabelarme des Ständers, alle Gebrauchsspuren unter 30 mm."),
        "thinElements": T("The frame tubes and the rear leg keep their true 16 mm as slender bars.", "Die Rahmenrohre und das Hinterbein behalten ihre wahren 16 mm als schlanke Stäbe."),
        "hatching": T("None.", "Keine."),
        "styles": T("As 1:50; body pen 0.25 mm, detail 0.13 mm for the fretboard field.", "Wie 1:50; Körperstift 0,25 mm, Detail 0,13 mm für das Griffbrettfeld."),
        "overlays": T("None.", "Keine."),
    },
    500: {
        "silhouette": T("The stack 2 mm tall as a block on four castor feet with the head on top, the bass 2.6 mm tall leaning on two stand legs.", "Der Turm 2 mm hoch als Block auf vier Rollenfüßen mit dem Verstärker darauf, der Bass 2,6 mm hoch an zwei Ständerbeine gelehnt."),
        "retained": T("The outline of cabinet and head, four castors as 75 mm feet, the bass as a 75 mm body and a 75 mm neck with its headstock, the frame tubes and rear leg at their true 16 mm and a 75 mm cross bar.", "Der Umriss von Box und Verstärker, vier Rollen als 75-mm-Füße, der Bass als 75-mm-Korpus und 75-mm-Hals mit Kopfplatte, Rahmenrohre und Hinterbein in ihren wahren 16 mm und ein 75-mm-Querstab."),
        "merged": T("Corners, handles and grille into the cabinet, handle and panels into the head, pickguard and hardware into the bass body, cradle, post and yoke into the cross bar and the bass, which leans directly on the frame tubes.", "Ecken, Griffe und Gitter in die Box, Griff und Platten in den Verstärker, Schlagbrett und Beschläge in den Basskorpus, Auflage, Stütze und Gabel in den Querstab und den Bass, der direkt an den Rahmenrohren lehnt."),
        "omitted": T("Every detail under 75 mm.", "Jedes Detail unter 75 mm."),
        "thinElements": T("Frame tubes and rear leg as slender bars of their true section.", "Rahmenrohre und Hinterbein als schlanke Stäbe in wahrem Querschnitt."),
        "hatching": T("None.", "Keine."),
        "styles": T("Tones as 1:50; one body pen of 0.25 mm.", "Töne wie 1:50; ein Körperstift von 0,25 mm."),
        "overlays": T("None.", "Keine."),
    },
}

VOL_DECISIONS = {
    50: [
        T("Body: the cabinet as built from its 18 mm front frame with the grille opening, the 12 mm grille recessed 4 mm, eight 50 mm ball corners standing 3 mm proud, two 8 mm handle flanges and the jack plate on four castors (14 mm plate block, fork block around the 2.5 mm cheeks, 75 mm wheel on its own spinning bone); the head as a 15 mm sleeve open front and back with the aluminium panel recessed 10 mm, nine knobs 17 mm proud, input, switch, rubber feet and the strap handle on two brackets; the bass as prisms of its true outlines along its thickness: 45 mm alder body, 2.3 mm pickguard, 18 mm maple neck under an 8 mm fretboard, 14 mm headstock, tuners, posts, bridge, 1.5 mm control plate, knobs, pickup halves, strap buttons and the four strings as slender rods; the stand as 16 mm tubes with 24 mm foam on cradle arms and yoke and 22 mm rubber caps.", "Körper: die Box wie gebaut aus ihrem 18-mm-Frontrahmen mit der Gitteröffnung, dem 12-mm-Gitter 4 mm zurückgesetzt, acht 50-mm-Kugelecken 3 mm vorstehend, zwei 8-mm-Griffschalen und der Buchsenplatte auf vier Lenkrollen (14-mm-Plattenblock, Gabelblock um die 2,5-mm-Wangen, 75-mm-Rad an seinem eigenen drehenden Knochen); der Verstärker als 15-mm-Gehäuse, vorn und hinten offen, mit der um 10 mm zurückgesetzten Aluminiumplatte, neun 17 mm vorstehenden Knöpfen, Eingang, Schalter, Gummifüßen und dem Tragegriff auf zwei Haltern; der Bass als Prismen seiner wahren Umrisse über seine Dicke: 45-mm-Erlenkorpus, 2,3-mm-Schlagbrett, 18-mm-Ahornhals unter einem 8-mm-Griffbrett, 14-mm-Kopfplatte, Mechaniken, Stifte, Brücke, 1,5-mm-Kontrollplatte, Knöpfe, Tonabnehmerhälften, Gurtpins und die vier Saiten als schlanke Stäbe; der Ständer aus 16-mm-Rohren mit 24-mm-Schaum auf Auflagearmen und Gabel und 22-mm-Gummikappen."),
        T("Thermal: every body part is the surface of its region; the fan outlet is a patch 1 mm in front of the rear panel; the cavity of the head between its panels is closed and counts no area.", "Thermisch: jedes Körperteil ist die Oberfläche seiner Region; der Lüfterauslass ist ein Feld 1 mm vor der Rückwand; der Hohlraum des Verstärkers zwischen seinen Platten ist geschlossen und zählt keine Fläche."),
        T("Structure: the stand's tubes and foam carry the bass from its two cradle contacts and the yoke to three feet; the cabinet is its shell of six 18 mm plywood panels on four castor plates, forks and wheels; the head is its sleeve bottom with the 1.5 mm steel chassis on four feet; the bass is body, neck, fretboard and headstock with the bolted neck joint and the strings in tension; supports at the seven floor contacts, masses at the centres of cabinet, head, bass and stand.", "Tragwerk: Rohre und Schaum des Ständers tragen den Bass von seinen zwei Auflagepunkten und der Gabel auf drei Füße; die Box ist ihre Schale aus sechs 18-mm-Sperrholzplatten auf vier Rollenplatten, Gabeln und Rädern; der Verstärker ist der Boden seines Gehäuses mit dem 1,5-mm-Stahlchassis auf vier Füßen; der Bass ist Korpus, Hals, Griffbrett und Kopfplatte mit der verschraubten Halsverbindung und den gespannten Saiten; Auflager an den sieben Bodenkontakten, Massen in den Schwerpunkten von Box, Verstärker, Bass und Ständer."),
    ],
    100: [
        T("Body for 15 mm: the carcass takes frame and grille in (the grille is its front face), corners stay as 50 mm blocks, castors are a fork block and a wheel; the head is one sleeve block with a 15 mm arched handle, its knobs and feet fall below the feature; the bass keeps body, pickguard sheet, a 26 mm neck with the fretboard, a 15 mm headstock, tuners thickened to 15 mm, bridge, knobs and one pickup block; the stand keeps its 16 mm tubes, 24 mm foam arms with lips, yoke and caps.", "Körper für 15 mm: das Gehäuse nimmt Rahmen und Gitter auf (das Gitter ist seine Frontfläche), die Ecken bleiben 50-mm-Blöcke, die Rollen sind Gabelblock und Rad; der Verstärker ist ein Gehäuseblock mit einem 15-mm-Bügelgriff, Knöpfe und Füße fallen unter das Merkmal; der Bass behält Korpus, Schlagbrett, einen 26-mm-Hals mit Griffbrett, eine 15-mm-Kopfplatte, auf 15 mm verdickte Mechaniken, Brücke, Knöpfe und einen Tonabnehmerblock; der Ständer behält seine 16-mm-Rohre, 24-mm-Schaumarme mit Lippen, Gabel und Kappen."),
        T("Thermal: the body parts are the surfaces of their regions; the grille and the two head panels are skins of their real 1.5, 3 and 1.5 mm layers on the faces that took them in; the fan outlet is a patch on the rear skin.", "Thermisch: die Körperteile sind die Oberflächen ihrer Regionen; das Gitter und die zwei Verstärkerplatten sind Häute ihrer wirklichen Schichten von 1,5, 3 und 1,5 mm auf den Flächen, die sie aufgenommen haben; der Lüfterauslass ist ein Feld auf der hinteren Haut."),
        T("Structure: as 1:50 without the castor plates, the head's feet and the strings; the head's 15 mm sleeve bottom stands on the cabinet's top panel.", "Tragwerk: wie 1:50 ohne Rollenplatten, Verstärkerfüße und Saiten; der 15-mm-Gehäuseboden des Verstärkers steht auf der Deckplatte der Box."),
    ],
    200: [
        T("Body for 30 mm: carcass with eight corner blocks, four castor blocks of wheel and fork, the head block with a 30 mm handle block, the bass body with its pickguard sheet and one 30 mm neck-and-headstock part, the stand with slender 16 mm frame tubes and rear leg and 30 mm cross tube, post, cradle arms and yoke.", "Körper für 30 mm: Gehäuse mit acht Eckblöcken, vier Rollenblöcke aus Rad und Gabel, der Verstärkerblock mit einem 30-mm-Griffblock, der Basskorpus mit seinem Schlagbrett und ein 30-mm-Teil aus Hals und Kopfplatte, der Ständer mit schlanken 16-mm-Rahmenrohren und Hinterbein und 30-mm-Querrohr, Stütze, Auflagearmen und Gabel."),
        T("Thermal: body parts as their regions; grille, head panels, bridge and tuners are skins of their real layers.", "Thermisch: Körperteile als ihre Regionen; Gitter, Verstärkerplatten, Brücke und Mechaniken sind Häute ihrer wirklichen Schichten."),
        T("Structure: the plywood panels of cabinet and head are declared sheets of their true 18 and 14 mm; castors are one member each; the bass is body and neck with the neck joint.", "Tragwerk: die Sperrholzplatten von Box und Verstärker sind erklärte Platten in ihren wahren 18 und 14 mm; die Rollen sind je ein Stab; der Bass ist Korpus und Hals mit der Halsverbindung."),
    ],
    500: [
        T("Body for 75 mm: the cabinet block on four 75 mm castor feet, the head block, the bass as a 75 mm body and a 75 mm neck with headstock leaning directly on the slender frame tubes, a 75 mm cross bar carrying the slender rear leg; cradle, post and yoke are below the feature and merged.", "Körper für 75 mm: der Boxenblock auf vier 75-mm-Rollenfüßen, der Verstärkerblock, der Bass als 75-mm-Korpus und 75-mm-Hals mit Kopfplatte direkt an die schlanken Rahmenrohre gelehnt, ein 75-mm-Querstab trägt das schlanke Hinterbein; Auflage, Stütze und Gabel liegen unter dem Merkmal und sind verschmolzen."),
        T("Thermal: body parts as their regions; grille, ball corners, head panels, pickguard, bridge and tuners are skins of their real layers.", "Thermisch: Körperteile als ihre Regionen; Gitter, Kugelecken, Verstärkerplatten, Schlagbrett, Brücke und Mechaniken sind Häute ihrer wirklichen Schichten."),
        T("Structure: the declared plywood sheets of cabinet and head, the castor feet, the bass body and neck with its joint, the slender stand tubes and the cross bar, 75 mm supports and masses.", "Tragwerk: die erklärten Sperrholzplatten von Box und Verstärker, die Rollenfüße, Basskorpus und Hals mit Verbindung, die schlanken Ständerrohre und der Querstab, 75-mm-Auflager und -Massen."),
    ],
}


def representation(scale):
    """🗺️ One authored representation."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES[scale], "geometry": shapes(scale), "decisions": REP_DECISIONS[scale]}


def volume(scale):
    """🧊 One authored volume."""
    return {"status": "authored", "denominator": scale, "body": [solid_doc(p) for p in PARTS[scale]], "thermal": thermal_list(scale), "structure": structure_list(scale), "decisions": VOL_DECISIONS[scale]}


def dimensions():
    """📏 The rest envelope of the 1:50 body (stored pose) and the total mass."""
    pts = [c for p in PARTS[50] for c in p.corners]
    xs, ys, zs = [p[0] for p in pts], [p[1] for p in pts], [p[2] for p in pts]
    return {"heightM": round(max(ys) - min(0.0, min(ys)), 3), "widthM": round(max(xs) - min(xs), 3), "depthM": round(max(zs) - min(zs), 3), "massKg": round(TOTAL_MASS, 2)}


def document(areas):
    """📄 The whole asset document."""
    asset = {
        "id": "used-electric-bass-and-cabinet",
        "category": "appliances",
        "title": T("Used Electric Bass with Amp Stack", "Gebrauchter E-Bass mit Verstärker"),
        "summary": T(
            "A second-hand long-scale four-string electric bass of the 1970s to 1990s type (864 mm scale, 1.165 m, 4.3 kg) in a folding A-frame stand beside a 300 W solid-state bass head on a 4x10 cabinet on castors (0.62 x 0.66 x 0.42 m, 35 kg), cleaned and set up to be played again; finish worn through at the forearm, darkened first frets, a removed finger rest, scuffed ball corners, torn tolex and a dented grille. The head's plate states 450 W; played at 1/8 power it releases 115 W, a third of it with the warm fan air out of the back.",
            "Ein gebrauchter viersaitiger E-Bass mit langer Mensur vom Typ der 1970er bis 1990er Jahre (Mensur 864 mm, 1,165 m, 4,3 kg) in einem klappbaren A-Ständer neben einem 300-W-Transistor-Bassverstärker auf einer 4x10-Box mit Rollen (0,62 x 0,66 x 0,42 m, 35 kg), gereinigt und spielbereit eingestellt; der Lack am Unterarm durchgescheuert, die ersten Bünde dunkel, eine Fingerstütze entfernt, Kugelecken abgestoßen, Tolex eingerissen und das Gitter eingedrückt. Das Typenschild des Verstärkers nennt 450 W; mit 1/8 Leistung gespielt gibt er 115 W ab, ein Drittel davon mit der warmen Lüfterluft nach hinten.",
        ),
        "tags": ["used", "instrument", "music", "second-hand", "strings", "amplification", "stage", "electric-bass", "bass-guitar", "bass-amplifier", "loudspeaker-cabinet", "castors", "instrument-stand", "electrical", "heat-source", "tolex", "plywood", "alder", "maple"],
        "provenance": {
            "author": "Semio staffage content (used instruments wave, row 10)",
            "method": "handcrafted",
            "created": "2026-11-22",
            "references": [{"id": r[0], "citation": r[1]} for r in REFERENCES],
            "notes": T(
                "A type, not a product: a bolt-on solid-body bass of Hornbostel-Sachs class 321.322-5 with an electromagnetic pickup (MIMO class 5, electrophones), 864 mm scale, 1.165 m long, body 0.335 m wide and 45 mm thick in alder, maple neck 41.5 mm at the nut and 63.5 mm at the twentieth fret, four in-line tuners, a split pickup and a bent-plate bridge, strung with a 45-105 nickel set at 0.72 kN; a 4x10 cabinet of 18 mm plywood in tolex with steel grille, ball corners, two recessed handles and four removable 75 mm swivel castors; a 300 W fan-cooled solid-state head of 0.55 x 0.24 x 0.30 m; a folding tubular A-frame stand of 16 mm steel with foam cradle and yoke. Ergonomics of the played pose after DIN 33402-2; heat after the normal operating condition of IEC 62368-1. Nothing is measured: dimensions and masses are catalogue-typical (authored estimates), wood data from the Wood Handbook (literature).",
                "Ein Typ, kein Produkt: ein geschraubter Solidbody-Bass der Hornbostel-Sachs-Klasse 321.322-5 mit elektromagnetischem Tonabnehmer (MIMO-Klasse 5, Elektrophone), Mensur 864 mm, 1,165 m lang, Korpus 0,335 m breit und 45 mm dick aus Erle, Ahornhals 41,5 mm am Sattel und 63,5 mm am zwanzigsten Bund, vier Mechaniken in Reihe, ein geteilter Tonabnehmer und eine Blechbrücke, mit einem Nickelsatz 45-105 auf 0,72 kN gespannt; eine 4x10-Box aus 18-mm-Sperrholz in Tolex mit Stahlgitter, Kugelecken, zwei eingelassenen Griffen und vier abnehmbaren 75-mm-Lenkrollen; ein lüftergekühlter 300-W-Transistorverstärker von 0,55 x 0,24 x 0,30 m; ein klappbarer A-Ständer aus 16-mm-Stahlrohr mit Schaumauflage und -gabel. Ergonomie der Spielpose nach DIN 33402-2; Wärme nach dem Normalbetrieb der IEC 62368-1. Nichts ist gemessen: Maße und Massen sind katalogtypisch (Schätzwerte), Holzdaten aus dem Wood Handbook (Literatur).",
            ),
        },
        "dimensions": dimensions(),
        "materials": [material_doc(m) for m in MATERIALS],
        "rig": rig(),
        "anchors": anchors(),
        "representations": {str(s): representation(s) for s in SCALES},
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal_model(areas),
        "structure": {
            "gravityMS2": 9.80665,
            "dynamics": "quasi-static",
            "supports": CONTACTS,
            "assumptions": [
                T("Seven floor contacts: four castor wheels under the cabinet and the two front feet and the rear foot of the stand; the solver treats the whole asset as one rigid body on elastic supports, so the share of the stand feet and of the castors is that rigid-body distribution, while the load paths inside cabinet, head, stand and bass follow their own members.", "Sieben Bodenkontakte: vier Rollenräder unter der Box und die zwei Vorderfüße und der Hinterfuß des Ständers; der Löser behandelt das ganze Asset als einen starren Körper auf elastischen Auflagern, sodass der Anteil der Ständerfüße und der Rollen diese Starrkörperverteilung ist, während die Lastpfade in Box, Verstärker, Ständer und Bass ihren eigenen Stäben folgen."),
                T("Stored, the bass rests on the two foam cradle arms and leans on the foam yoke without friction; these contact forces act equal and opposite on bass and stand, so the stand's tubes carry the bass to the floor. Played, the strap and the fretting hand carry the weight, the fingers and the player's belly press on the bass in equilibrium; the strings pull 0.72 kN between saddles and nut on every pose.", "Abgestellt liegt der Bass auf den zwei Schaumarmen der Auflage und lehnt reibungsfrei an der Schaumgabel; diese Kontaktkräfte wirken gleich und entgegengesetzt auf Bass und Ständer, sodass die Rohre des Ständers den Bass zum Boden tragen. Gespielt tragen Gurt und Greifhand das Gewicht, Finger und Bauch des Spielers drücken im Gleichgewicht auf den Bass; die Saiten ziehen in jeder Pose 0,72 kN zwischen Reitern und Sattel."),
                T("Rolled, a 20 N push at the right side handle overcomes the rolling resistance of the four 75 mm polyurethane castors (about 0.04 of the 48 kg it rolls, EN 12532 class).", "Gerollt überwindet ein Schub von 20 N am rechten Seitengriff den Rollwiderstand der vier 75-mm-Polyurethanrollen (etwa 0,04 der gerollten 48 kg, Klasse nach EN 12532)."),
            ],
            "clinical": False,
        },
        "volumes": {str(s): volume(s) for s in SCALES},
        "presentation": {"defaultPose": "stored", "defaultClip": "groove", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    areas_file = HERE / "author-areas.json"
    areas = json.loads(areas_file.read_text()) if areas_file.exists() else {}
    doc = document(areas)
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    a = doc["asset"]
    print("[DEBUG] written", a["dimensions"], "solids", {s: (len(a["volumes"][s]["body"]), len(a["volumes"][s]["thermal"]), len(a["volumes"][s]["structure"])) for s in a["volumes"]})
