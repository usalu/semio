"""🎺️ Authoritative generator of the staffage asset `used-trumpet`: every hand-chosen number of the used Bb student trumpet, its mouthpiece, its two mutes, the valve oil and the worn rectangular case lives here, and the asset document `🔣️.json` next to this file is written from it.

Run: `python3 author-used-trumpet.py [areas.json]` (writes `🔣️.json`; the optional JSON holds the 1:50 outer area per thermal region measured on this very model, see `author-measure-areas.sh`).

@see ../../brief.md
@see ./author_kit.py
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
import sys
from pathlib import Path

from author_kit import T, Part, TAU, arc, circle, path_length, r6, rect, torus, tube_path

HERE = Path(__file__).resolve().parent

# ---------------------------------------------------------------- reference numbers (report table)
RHO_BRASS = 8530.0
RHO_NS = 8700.0
RHO_ALU = 2700.0
RHO_CASE = 560.0
RHO_FOAM = 90.0
RHO_FIBRE = 700.0
RHO_PE = 950.0
RHO_OIL = 870.0
RHO_STEEL = 7850.0
WALL_TUBE = 0.00045
WALL_LEAD = 0.0005
WALL_BELL = 0.0005

L_CASE, W_CASE, H_BASE, H_LID, SHELL = 0.640, 0.330, 0.120, 0.055, 0.006
BED_TOP = 0.020
LINE = 0.003
GRIP = (-0.320, 0.100, -0.190)
HINGE = (-0.320, 0.120, 0.170)
LID_OPEN = 100.0
TRUMPET_AT = (-0.1765, 0.0415, -0.090)
PLAY_Y = 1.493
PITCH = -6.0
MP_OUT = 0.060
CARRY_Y = 0.660

U_VALVE = (0.200, 0.224, 0.248)
CASING_R = 0.01025
CAP_R = 0.0115
CASING_W = 0.020
BELL_W = 0.040
BELL_START = 0.300
BELL_PROFILE = [(0.300, 0.0078), (0.340, 0.0104), (0.370, 0.0138), (0.395, 0.0182), (0.415, 0.0236), (0.432, 0.0304), (0.446, 0.0383), (0.457, 0.0462), (0.465, 0.0530), (0.4705, 0.0571), (0.4745, 0.0593)]
BEAD = (0.4752, 0.0593, 0.0022)
STROKE = 0.017
PISTON_KG = 0.040
SPRING_KG = 0.006
CAP_KG = 0.015
FITTINGS_KG = 0.075


def up(u, vu, wl):
    """🔁️ Upright trumpet coordinates (u ahead, vu up, wl to the player's left) to the trumpet's lying rest frame (u, v, w)."""
    return (u, wl, -vu)


def cframe(x, y, z):
    """📦️ World point of the rest pose to the frame of the case bone (origin at the handle grip)."""
    return (x - GRIP[0], y - GRIP[1], z - GRIP[2])


def lframe(x, y, z):
    """🗃️ World point of the closed lid to the frame of the lid bone (origin on the hinge axis)."""
    return (x - HINGE[0], y - HINGE[1], z - HINGE[2])


def cbox(pid, lo, hi, **kw):
    """📦️ A box part of the case bone from world corners of the rest pose."""
    return Part(pid, "case", kw.pop("material", "case-ply-tolex"), "box", {"lo": cframe(*lo), "hi": cframe(*hi)}, zrest=GRIP[2], **kw)


def lbox(pid, lo, hi, **kw):
    """🗃️ A box part of the lid bone from world corners of the closed lid."""
    return Part(pid, "lid", kw.pop("material", "case-ply-tolex"), "box", {"lo": lframe(*lo), "hi": lframe(*hi)}, **kw)


def box_mass(lo, hi, rho):
    """⚖️ Mass and centre of a solid box."""
    v = abs((hi[0] - lo[0]) * (hi[1] - lo[1]) * (hi[2] - lo[2]))
    return v * rho, tuple((a + b) / 2 for a, b in zip(lo, hi))


def tube_mass(path, wall, rho=RHO_BRASS):
    """⚖️ Mass and centre of a thin-walled tube along a (a, b, r) path: section 2 pi r t times length times density."""
    m, ca, cb = 0.0, 0.0, 0.0
    for p, q in zip(path, path[1:]):
        ds = math.hypot(q[0] - p[0], q[1] - p[1])
        dm = TAU * (p[2] + q[2]) / 2 * wall * ds * rho
        m += dm
        ca += dm * (p[0] + q[0]) / 2
        cb += dm * (p[1] + q[1]) / 2
    return m, (ca / m, cb / m)


def revolve_volume(profile):
    """⚖️ Volume and axial centroid of an open revolve profile (closed through the axis) or of a closed loop (Pappus)."""
    loop = list(profile)
    if loop[0] != loop[-1]:
        loop = [(loop[0][0], 0.0)] + loop + [(loop[-1][0], 0.0), (loop[0][0], 0.0)]
    vol, mom = 0.0, 0.0
    for (a0, r0), (a1, r1) in zip(loop, loop[1:]):
        dv = math.pi * (a1 - a0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3
        vol += dv
        mom += dv * (a0 + a1) / 2
    return abs(vol), (mom / vol if vol else 0.0)


def revolve_area(profile):
    """📐️ Lateral surface of a revolve profile polyline (Pappus)."""
    return sum(TAU * (r0 + r1) / 2 * math.hypot(a1 - a0, r1 - r0) for (a0, r0), (a1, r1) in zip(profile, profile[1:]))


# ---------------------------------------------------------------- references and materials
REFS = [
    {"id": "hornbostel-sachs-1914", "citation": "Hornbostel, E. M. von, Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift fuer Ethnologie 46, 553-590; English: Baines, A., Wachsmann, K. P. (1961). Classification of Musical Instruments. Galpin Society Journal 14, 3-29 (trumpet 423.233: lip-vibrated aerophone, chromatic by valves, predominantly cylindrical bore; mutes are accessories, not instruments)."},
    {"id": "baines-1976", "citation": "Baines, A. (1976). Brass Instruments: Their History and Development. Faber and Faber, London, chapters 9 and 10 (the Perinet piston-valve trumpet in B flat: three valves, leadpipe to the main tuning slide, valve slides, bell branch and bow; the straight and cup mutes)."},
    {"id": "campbell-gilbert-myers-2021", "citation": "Campbell, M., Gilbert, J., Myers, A. (2021). The Science of Brass Instruments. ASA Press / Springer, Cham, chapters 2, 6 and 9 (B flat trumpet: about 1.48 m of tubing, bore about 11.7 mm, bell 120 to 125 mm; valve lengths; blowing pressures and air flows of the player; mute geometry)."},
    {"id": "fletcher-rossing-1998", "citation": "Fletcher, N. H., Rossing, T. D. (1998). The Physics of Musical Instruments, 2nd ed. Springer, New York, chapter 14 (lip-driven brass instruments, horn profiles, mouthpiece cup and throat, mutes)."},
    {"id": "barbenel-1988", "citation": "Barbenel, J. C., Kenny, P., Davies, J. B. (1988). Mouthpiece forces produced while playing the trumpet. Journal of Biomechanics 21(5), 417-424 (force of the mouthpiece on the lips rising with pitch and loudness, from a few newtons to several tens of newtons)."},
    {"id": "brass-catalogues", "citation": "Catalogue data of standard student B flat trumpets and their cases as music shops, school instrument depots and used-instrument dealers list them, including the student models of the Markneukirchen and Klingenthal workshops and the West German school trumpets as types, never as brands (overall length about 0.48 m without the mouthpiece, ML bore 11.68 mm, yellow-brass bell of 123 mm, nickel-silver caps and slides, 1.0 to 1.1 kg; 7C mouthpiece 88 mm, 26 mm rim, about 110 g; spun-aluminium straight mute 13 to 14 cm, fibre cup mute; 50 ml valve oil; rectangular wooden case with tolex cover, plush bed and accessory compartment, 0.60 to 0.65 m long, 2.8 to 3.5 kg) (authored-estimate)."},
    {"id": "cda-c26000", "citation": "Copper Development Association (2019). C26000 Cartridge Brass 70 %: physical and mechanical properties (density 8.53 g/cm3, thermal conductivity 120 W/(m K), specific heat 375 J/(kg K), modulus of elasticity 110 GPa); C75200 nickel silver 65-18 (density 8.73 g/cm3)."},
    {"id": "wood-handbook-2010", "citation": "Forest Products Laboratory (2010). Wood Handbook - Wood as an Engineering Material, General Technical Report FPL-GTR-190. USDA Forest Service, Madison WI (density, stiffness and thermal properties of plywood)."},
    {"id": "iso-10456-2007", "citation": "EN ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (thermal conductivity and specific heat of plywood, PVC, polyethylene foam, textiles and aluminium)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed. Wiley, table A.11 (emissivities: polished and oxidised brass, lacquered metal, polished and tarnished silver, aluminium, PVC, textiles)."},
    {"id": "din-33402-2", "citation": "DIN 33402-2:2020 Ergonomics - Human body dimensions - Part 2: Values (standing eye height about 1.51 to 1.63 m, mouth height about 1.50 m, knuckle height of the hanging arm about 0.76 m)."},
    {"id": "ashrae-2021", "citation": "ASHRAE (2021). Handbook - Fundamentals, chapter 9 Thermal Comfort (exhaled air near 34 degC and close to saturation; skin temperature of the hands about 33 degC)."},
]


def mat(mid, en, de, eps, alpha, rho, k, cp, e=None, f=None, src="literature", ref="incropera-2018"):
    """🧱️ One material."""
    m = {"id": mid, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp}
    if e:
        m["elasticModulusPa"] = e
    if f:
        m["strengthPa"] = f
    m.update({"source": src, "reference": ref})
    return m


MATERIALS = [
    mat("case-ply-tolex", "Poplar plywood 5 mm under black PVC tolex 1 mm, scuffed matt", "Pappelsperrholz 5 mm unter schwarzem PVC-Tolex 1 mm, matt verschrammt", 0.92, 0.9, RHO_CASE, 0.15, 1500, 6.0e9, 3.0e7, "authored-estimate", "wood-handbook-2010"),
    mat("tolex-torn", "Torn tolex with the plywood showing, aged", "Eingerissenes Tolex mit sichtbarem Sperrholz, gealtert", 0.9, 0.6, RHO_CASE, 0.15, 1500, None, None, "authored-estimate", "wood-handbook-2010"),
    mat("plush-foam", "Burgundy velour on polyethylene foam, the pile flattened where the trumpet lay", "Weinroter Velours auf Polyethylenschaum, der Flor plattgedrückt, wo die Trompete lag", 0.9, 0.7, RHO_FOAM, 0.04, 1400, None, None, "literature", "iso-10456-2007"),
    mat("nickel-steel", "Nickel-plated steel hardware, dulled and rust-spotted", "Vernickelte Stahlbeschläge, stumpf und rostfleckig", 0.2, 0.45, RHO_STEEL, 50, 460, 2.0e11, 3.5e8, "authored-estimate", "incropera-2018"),
    mat("handle-pvc", "PVC-sheathed steel handle, worn shiny on the grip", "PVC-ummantelter Stahlgriff, am Griff blankgegriffen", 0.9, 0.85, 1400, 0.2, 1200, 3.0e9, 4.0e7, "authored-estimate", "iso-10456-2007"),
    mat("brass-lacquer", "Yellow brass CuZn30 under clear lacquer, yellowed and hazed with age", "Messing CuZn30 unter Klarlack, mit dem Alter vergilbt und eingetrübt", 0.6, 0.4, RHO_BRASS, 120, 375, 1.1e11, 3.0e8, "authored-estimate", "cda-c26000"),
    mat("brass-worn", "Bare brass where the lacquer is worn off by the hands, tarnished brown", "Blankes Messing, wo die Hände den Lack abgerieben haben, braun angelaufen", 0.3, 0.55, RHO_BRASS, 120, 375, 1.1e11, 3.0e8, "authored-estimate", "incropera-2018"),
    mat("nickel-silver", "Nickel silver CuNi18Zn20 caps and slide tubes, tarnished", "Neusilber CuNi18Zn20, Kappen und Zugrohre, angelaufen", 0.15, 0.45, RHO_NS, 33, 380, 1.25e11, 4.0e8, "literature", "cda-c26000"),
    mat("button-pearl", "Mother-of-pearl valve-button inlays in nickel-silver rims", "Perlmutteinlagen der Ventildrücker in Neusilberfassungen", 0.9, 0.3, 2000, 0.5, 900, None, None, "authored-estimate", "incropera-2018"),
    mat("silver-plate", "Silver-plated brass mouthpiece, the plating worn through at the rim", "Versilbertes Messing-Mundstück, die Versilberung am Rand durchgerieben", 0.06, 0.25, RHO_BRASS, 120, 375, 1.1e11, 3.0e8, "authored-estimate", "incropera-2018"),
    mat("aluminium-spun", "Spun aluminium 0.7 mm of the straight mute, scratched and with a dent", "Gedrücktes Aluminium 0,7 mm des Straight-Dämpfers, verkratzt und mit einer Delle", 0.12, 0.3, RHO_ALU, 205, 900, 7.0e10, 1.5e8, "literature", "incropera-2018"),
    mat("cork", "Natural cork strips, compressed and darkened", "Naturkorkstreifen, zusammengedrückt und nachgedunkelt", 0.9, 0.6, 240, 0.045, 1900, None, None, "literature", "iso-10456-2007"),
    mat("fibre-painted", "Pressed fibre of the cup mute, painted black outside, chipped", "Gepresste Fasermasse des Cup-Dämpfers, außen schwarz lackiert, abgeplatzt", 0.9, 0.85, RHO_FIBRE, 0.15, 1300, 3.0e9, 2.0e7, "authored-estimate", "iso-10456-2007"),
    mat("pe-bottle", "Polyethylene valve-oil bottle, half full", "Polyethylen-Ventilölflasche, halb voll", 0.9, 0.5, RHO_PE, 0.4, 1900, 1.0e9, 2.0e7, "literature", "iso-10456-2007"),
    mat("microfibre", "Folded microfibre polishing cloth, grey with tarnish", "Gefaltetes Mikrofaser-Poliertuch, grau vom Anlaufen", 0.9, 0.6, 300, 0.05, 1300, None, None, "authored-estimate", "iso-10456-2007"),
]

# ---------------------------------------------------------------- trumpet layout (upright coordinates: u ahead from the receiver end, vu up from the receiver axis, wl to the left)
LEAD = tube_path([("line", (0.034, 0.0), 0.0050), ("line", (0.120, -0.0035), 0.0053), ("line", (0.240, -0.0090), 0.0057), ("line", (0.340, -0.0140), 0.0060)])
MAIN_LOWER = tube_path([("line", (0.340, -0.050), 0.0063), ("line", (0.248, -0.050), 0.0063)])
MAIN_CROOK = tube_path([("line", (0.315, -0.014), 0.0060), ("line", (0.400, -0.014), 0.0060), ("arc", (0.400, -0.032), 0.018, 90, -90, 8, 0.0063), ("line", (0.315, -0.050), 0.0060)])
STUB_1 = [tube_path([("line", (0.200, vu), 0.0058), ("line", (0.165, vu), 0.0058)]) for vu in (-0.026, -0.050)]
SLIDE_1 = tube_path([("line", (0.190, -0.026), 0.0053), ("line", (0.150, -0.026), 0.0053), ("arc", (0.150, -0.038), 0.012, 90, 270, 8, 0.0055), ("line", (0.190, -0.050), 0.0053)])
STUB_2 = [tube_path([("line", (0.224, vu), 0.0055), ("line", (0.252, vu), 0.0055)]) for vu in (-0.064, -0.080)]
SLIDE_2 = tube_path([("line", (0.235, -0.064), 0.0050), ("line", (0.272, -0.064), 0.0050), ("arc", (0.272, -0.072), 0.008, 90, -90, 6, 0.0052), ("line", (0.235, -0.080), 0.0050)])
STUB_3 = [tube_path([("line", (0.248, vu), 0.0055), ("line", (0.210, vu), 0.0055)]) for vu in (-0.054, -0.070)]
SLIDE_3 = tube_path([("line", (0.235, -0.054), 0.0050), ("line", (0.120, -0.054), 0.0050), ("arc", (0.120, -0.062), 0.008, 90, 270, 6, 0.0052), ("line", (0.235, -0.070), 0.0050)])
BOW = tube_path([("line", (0.200, -0.040), 0.0063), ("line", (0.045, -0.040), 0.0063), ("arc", (0.045, -0.020), 0.020, 270, 90, 10, 0.0063), ("line", (0.150, 0.0), 0.0066), ("line", (0.230, 0.0), 0.0070), ("line", (BELL_START, 0.0), 0.0078)])
KNUCKLES = [("k-main", 0.248, -0.050, 0.0, CASING_W, 0.0060), ("k-slide1-a", 0.200, -0.026, 0.0, CASING_W, 0.0056), ("k-slide1-b", 0.200, -0.050, 0.0, CASING_W, 0.0056), ("k-slide2-a", 0.224, -0.064, 0.0, CASING_W, 0.0053), ("k-slide2-b", 0.224, -0.080, 0.0, CASING_W, 0.0053), ("k-slide3-a", 0.248, -0.054, CASING_W, BELL_W, 0.0053), ("k-slide3-b", 0.248, -0.070, CASING_W, BELL_W, 0.0053), ("k-bow", 0.200, -0.040, CASING_W, BELL_W, 0.0062)]
RING_AT = (0.135, -0.0868)
RING_R, RING_WIRE = 0.0106, 0.0014
MOUTHPIECE = [(-0.0560, 0.0), (-0.0600, 0.0058), (-0.0640, 0.0081), (-0.0655, 0.0104), (-0.0648, 0.0129), (-0.0628, 0.0133), (-0.0595, 0.0126), (-0.0500, 0.0103), (-0.0380, 0.0079), (-0.0200, 0.0067), (0.0000, 0.0050), (0.0230, 0.0047)]
MOUTHPIECE_OUT = [(-0.0655, 0.0104), (-0.0648, 0.0129), (-0.0628, 0.0133), (-0.0595, 0.0126), (-0.0500, 0.0103), (-0.0380, 0.0079), (-0.0200, 0.0067), (0.0000, 0.0050), (0.0230, 0.0047)]


def lying_path(path):
    """🛏️ A tube path from upright (u, vu, r) to the lying (u, w, r) of the trumpet frame (w = -vu)."""
    return [(a, -b, r) for a, b, r in path]


def rel(path, du, dw):
    """↔️ A lying path relative to a bone origin at (du, dw)."""
    return [(a - du, b - dw, r) for a, b, r in path]


def coarse(path, keep):
    """🪶️ Every `keep`-th point of a path, both ends kept."""
    out = path[::keep]
    if out[-1] != path[-1]:
        out.append(path[-1])
    return out


def bell_loop(profile, wall):
    """🔔️ Closed loop of the bell shell: outer flare forward, inner flare back one wall thickness in (first point repeated)."""
    inner = [(a, max(r - wall, 0.0005)) for a, r in reversed(profile)]
    loop = list(profile) + inner
    return loop + [loop[0]]


def trumpet_parts(s):
    """🎺️ The trumpet, its slides, pistons and mouthpiece at scale 1:s, in the lying rest frame of each bone."""
    P = []
    lead = LEAD if s == 50 else coarse(LEAD, 2)
    if s < 200:
        rec = [(0.0, 0.0068), (0.026, 0.0068), (0.026, 0.0075), (0.034, 0.0075), (0.034, 0.0058)] if s == 50 else [(0.0, 0.0070), (0.034, 0.0070), (0.034, 0.0058)]
        m, c = revolve_volume(rec)
        P.append(Part("receiver", "trumpet", "nickel-silver", "ru", {"profile": rec}, region="trumpet-brass", tone=0.8, mass=m * 0.25 * RHO_NS + FITTINGS_KG, com=(0.22, 0, 0.02)))
        mp, cp = tube_mass(lying_path(lead), WALL_LEAD)
        P.append(Part("leadpipe", "trumpet", "brass-lacquer", "tube", {"path": lying_path(lead), "v": 0.0}, region="trumpet-brass", tone=0.72, mass=mp, com=(cp[0], 0, cp[1])))
        ml, cl = tube_mass(lying_path(MAIN_LOWER), WALL_TUBE * 2)
        P.append(Part("main-lower", "trumpet", "brass-lacquer", "tube", {"path": lying_path(MAIN_LOWER), "v": 0.0}, region="trumpet-brass", tone=0.72, mass=ml, com=(cl[0], 0, cl[1])))
    else:
        whole = tube_path([("line", (0.0, 0.0), 0.0068), ("line", (0.340, -0.014), 0.0060), ("line", (0.400, -0.014), 0.0060), ("arc", (0.400, -0.032), 0.018, 90, -90, 4, 0.0063), ("line", (0.248, -0.050), 0.0063)])
        mp, cp = tube_mass(lying_path(whole), WALL_LEAD * 1.3)
        P.append(Part("leadpipe-and-main-slide", "trumpet", "brass-lacquer", "tube", {"path": lying_path(whole), "v": 0.0}, region="trumpet-brass", tone=0.72, mass=mp + FITTINGS_KG, com=(cp[0], 0, cp[1])))
    if s == 50:
        wear = [p for p in lying_path(lead) if 0.11 <= p[0] <= 0.25]
        P.append(Part("leadpipe-wear", "trumpet", "brass-worn", "tube", {"path": [(a, b, r + 0.0003) for a, b, r in wear], "v": 0.0}, region="worn-brass", role="texture", tone=0.55, flags={"texture": True}))
    if s < 200:
        for i, path in enumerate(STUB_1):
            m, c = tube_mass(lying_path(path), WALL_TUBE)
            P.append(Part(f"stub-slide1-{'ab'[i]}", "trumpet", "nickel-silver", "tube", {"path": lying_path(path), "v": 0.0}, region="trumpet-brass", tone=0.8, mass=m, com=(c[0], 0, c[1])))
        for i, path in enumerate(STUB_2):
            m, c = tube_mass(lying_path(path), WALL_TUBE)
            P.append(Part(f"stub-slide2-{'ab'[i]}", "trumpet", "nickel-silver", "tube", {"path": lying_path(path), "v": 0.0}, region="trumpet-brass", tone=0.8, mass=m, com=(c[0], 0, c[1])))
        knuckles = KNUCKLES if s == 50 else [k for k in KNUCKLES if k[0] in ("k-main", "k-slide1-a", "k-slide2-a", "k-slide3-a", "k-bow")]
        for kid, u, vu, w0, w1, r in knuckles:
            m = math.pi * r * r * (w1 - w0) * 0.25 * RHO_BRASS
            P.append(Part(kid, "trumpet", "brass-lacquer", "cap_v", {"u": u, "w": -vu, "v0": w0, "v1": w1, "r": r}, region="trumpet-brass", role="detail", tone=0.7, mass=m, com=(u, (w0 + w1) / 2, -vu)))
        if s == 50:
            for i in range(2):
                P.append(Part(f"port-{i + 1}{i + 2}", "trumpet", "brass-lacquer", "cap_u", {"u0": U_VALVE[i], "u1": U_VALVE[i + 1], "v": CASING_W, "w": 0.030, "r": 0.0050}, region="trumpet-brass", role="detail", tone=0.7, mass=0.004, com=((U_VALVE[i] + U_VALVE[i + 1]) / 2, CASING_W, 0.030)))
        for i, u in enumerate(U_VALVE):
            bone = f"casing-{i + 1}"
            if s == 50:
                body = [(-0.072, CASING_R), (0.018, CASING_R)]
                P.append(Part(f"casing-{i + 1}", bone, "brass-lacquer", "ru", {"profile": body}, region="trumpet-brass", tone=0.74, mass=TAU * CASING_R * 0.090 * 0.0008 * RHO_BRASS + PISTON_KG + SPRING_KG, com=(-0.027, 0, 0)))
                P.append(Part(f"cap-top-{i + 1}", bone, "nickel-silver", "ru", {"profile": [(0.018, CAP_R), (0.026, CAP_R)]}, region="trumpet-brass", role="detail", tone=0.85, mass=CAP_KG, com=(0.022, 0, 0)))
                P.append(Part(f"cap-bottom-{i + 1}", bone, "nickel-silver", "ru", {"profile": [(-0.084, CAP_R - 0.001), (-0.081, CAP_R), (-0.072, CAP_R)]}, region="trumpet-brass", role="detail", tone=0.85, mass=CAP_KG, com=(-0.078, 0, 0)))
                sleeve = [(-0.050, CASING_R), (-0.050, CASING_R + 0.0003), (-0.008, CASING_R + 0.0003), (-0.008, CASING_R), (-0.050, CASING_R)]
                P.append(Part(f"casing-wear-{i + 1}", bone, "brass-worn", "ru", {"profile": sleeve, "outline": [(-0.050, 0.0), (-0.050, CASING_R + 0.0003), (-0.008, CASING_R + 0.0003), (-0.008, 0.0)]}, region="worn-brass", role="texture", tone=0.5, flags={"texture": True}))
            else:
                P.append(Part(f"casing-wear-{i + 1}", bone, "brass-worn", "ru", {"profile": [(-0.050, CASING_R + 0.0006), (-0.008, CASING_R + 0.0006)]}, region="worn-brass", role="detail", tone=0.5, solid3d=False))
                prof = [(-0.084, CAP_R), (-0.072, CAP_R), (-0.072, CASING_R), (0.018, CASING_R), (0.018, CAP_R), (0.026, CAP_R)]
                P.append(Part(f"casing-{i + 1}", bone, "brass-lacquer", "ru", {"profile": prof}, region="trumpet-brass", tone=0.74, mass=TAU * CASING_R * 0.090 * 0.0008 * RHO_BRASS + PISTON_KG + SPRING_KG + 2 * CAP_KG, com=(-0.029, 0, 0)))
            pb = f"piston-{i + 1}"
            if s == 50:
                P.append(Part(f"stem-{i + 1}", pb, "nickel-silver", "ru", {"profile": [(0.0, 0.0024), (0.021, 0.0024)]}, region="trumpet-brass", role="detail", tone=0.85, mass=0.003, com=(0.010, 0, 0)))
                P.append(Part(f"button-{i + 1}", pb, "button-pearl", "ru", {"profile": [(0.021, 0.0070), (0.023, 0.0080), (0.028, 0.0080), (0.029, 0.0072)]}, region="trumpet-brass", role="detail", tone=0.95, mass=0.005, com=(0.025, 0, 0)))
            else:
                P.append(Part(f"piston-top-{i + 1}", pb, "button-pearl", "ru", {"profile": [(0.0, 0.0030), (0.021, 0.0030), (0.021, 0.0080), (0.029, 0.0080)]}, region="trumpet-brass", role="detail", tone=0.95, mass=0.008, com=(0.020, 0, 0)))
    else:
        P.append(Part("valve-block", "trumpet", "brass-lacquer", "box", {"lo": (0.1895, CASING_W - CASING_R, -0.026), "hi": (0.2585, CASING_W + CASING_R, 0.084)}, region="trumpet-brass", tone=0.74, mass=3 * (TAU * CASING_R * 0.090 * 0.0008 * RHO_BRASS + PISTON_KG + SPRING_KG + 2 * CAP_KG) + 0.03, com=(0.224, CASING_W, 0.029)))
        P.append(Part("valve-block-wear", "trumpet", "brass-worn", "box", {"lo": (0.1883, CASING_W - CASING_R - 0.0012, 0.008), "hi": (0.2597, CASING_W + CASING_R + 0.0012, 0.050)}, region="worn-brass", role="body", tone=0.5, solid3d=False))
        P.append(Part("valve-tops", "trumpet", "button-pearl", "box", {"lo": (0.192, CASING_W - 0.008, -0.055), "hi": (0.256, CASING_W + 0.008, -0.026)}, region="trumpet-brass", tone=0.95, mass=0.024, com=(0.224, CASING_W, -0.040)))
        m, c = tube_mass(lying_path(SLIDE_1), WALL_TUBE * 2)
        P.append(Part("lower-slides", "trumpet", "nickel-silver", "tube", {"path": lying_path(tube_path([("line", (0.200, -0.026), 0.0055), ("line", (0.150, -0.026), 0.0055), ("arc", (0.150, -0.038), 0.012, 90, 270, 4, 0.0055), ("line", (0.224, -0.050), 0.0055), ("line", (0.240, -0.064), 0.0055), ("line", (0.272, -0.064), 0.0055), ("arc", (0.272, -0.072), 0.008, 90, -90, 2, 0.0055), ("line", (0.240, -0.080), 0.0055)])), "v": 0.0}, region="trumpet-brass", tone=0.8, mass=m * 2.2, com=(0.200, 0, 0.055)))
    mc, cc = tube_mass(lying_path(MAIN_CROOK), WALL_TUBE * 2)
    o = (0.400, 0.032)
    if s < 200:
        P.append(Part("main-crook", "main-slide", "brass-lacquer", "tube", {"path": rel(lying_path(MAIN_CROOK if s == 50 else coarse(MAIN_CROOK, 2)), *o), "v": 0.0}, region="trumpet-brass", tone=0.72, mass=mc, com=(cc[0] - o[0], 0, cc[1] - o[1])))
        m1, c1 = tube_mass(lying_path(SLIDE_1), WALL_TUBE * 2)
        o1 = (0.150, 0.038)
        P.append(Part("slide1-crook", "slide-1", "nickel-silver", "tube", {"path": rel(lying_path(SLIDE_1 if s == 50 else coarse(SLIDE_1, 2)), *o1), "v": 0.0}, region="trumpet-brass", tone=0.8, mass=m1, com=(c1[0] - o1[0], 0, c1[1] - o1[1])))
        m2, c2 = tube_mass(lying_path(SLIDE_2), WALL_TUBE * 2)
        o2 = (0.272, 0.072)
        P.append(Part("slide2-crook", "slide-2", "nickel-silver", "tube", {"path": rel(lying_path(SLIDE_2 if s == 50 else coarse(SLIDE_2, 2)), *o2), "v": 0.0}, region="trumpet-brass", tone=0.8, mass=m2, com=(c2[0] - o2[0], 0, c2[1] - o2[1])))
        m3, c3 = tube_mass(lying_path(SLIDE_3), WALL_TUBE * 2)
        o3 = (0.120, 0.062)
        P.append(Part("slide3-crook", "slide-3", "nickel-silver", "tube", {"path": rel(lying_path(SLIDE_3 if s == 50 else coarse(SLIDE_3, 2)), *o3), "v": 0.0}, region="trumpet-brass", tone=0.8, mass=m3 + 0.004, com=(c3[0] - o3[0], 0, c3[1] - o3[1])))
        for i, path in enumerate(STUB_3 if s == 50 else STUB_3[:1] + STUB_3[1:]):
            m, c = tube_mass(lying_path(path), WALL_TUBE)
            P.append(Part(f"stub-slide3-{'ab'[i]}", "bell", "nickel-silver", "tube", {"path": lying_path(path), "v": 0.0}, region="trumpet-brass", tone=0.8, mass=m, com=(c[0], 0, c[1])))
        if s == 50:
            P.append(Part("ring-3", "ring-3", "nickel-silver", "ru", {"profile": torus(0.0, RING_R, RING_WIRE, 10)}, region="trumpet-brass", role="detail", tone=0.85, flags={"slender": True}, mass=0.004, com=(0, 0, 0)))
    else:
        m3, c3 = tube_mass(lying_path(SLIDE_3), WALL_TUBE * 2)
        P.append(Part("third-slide", "bell", "nickel-silver", "tube", {"path": lying_path(tube_path([("line", (0.248, -0.054), 0.0052), ("line", (0.120, -0.054), 0.0052), ("arc", (0.120, -0.062), 0.008, 90, 270, 2, 0.0052), ("line", (0.248, -0.070), 0.0052)])), "v": 0.0}, region="trumpet-brass", tone=0.8, mass=m3 + 0.02, com=(0.180, 0, 0.062)))
    bow = BOW if s == 50 else coarse(BOW, 2 if s == 100 else 4)
    mb, cb = tube_mass(lying_path(bow), WALL_TUBE)
    P.append(Part("bow-and-bell-tail", "bell", "brass-lacquer", "tube", {"path": lying_path(bow), "v": 0.0}, region="trumpet-brass", tone=0.72, mass=mb, com=(cb[0], 0, cb[1])))
    prof = BELL_PROFILE if s == 50 else (BELL_PROFILE[::2] + ([BELL_PROFILE[-1]] if len(BELL_PROFILE) % 2 == 0 else []))
    if s == 50:
        loop = bell_loop(prof, WALL_BELL)
        P.append(Part("bell", "bell", "brass-lacquer", "ru", {"profile": loop, "outline": [(BELL_START, 0.0)] + prof + [(prof[-1][0], 0.0)]}, region="trumpet-brass", tone=0.78, mass=revolve_area(prof) * WALL_BELL * RHO_BRASS, com=(0.43, 0, 0)))
        P.append(Part("bell-bead", "bell", "brass-lacquer", "ru", {"profile": torus(BEAD[0], BEAD[1], BEAD[2], 10)}, region="trumpet-brass", role="detail", tone=0.8, flags={"slender": True}, mass=TAU * BEAD[1] * math.pi * 0.0012 ** 2 * RHO_BRASS + 0.012, com=(BEAD[0], 0, 0)))
    else:
        rim = prof + [(BEAD[0] + BEAD[2], BEAD[1] + BEAD[2])]
        loop = bell_loop(rim, WALL_BELL + (0.0015 if s == 200 else 0.0))
        P.append(Part("bell", "bell", "brass-lacquer", "ru", {"profile": loop, "outline": [(BELL_START, 0.0)] + rim + [(rim[-1][0], 0.0)]}, region="trumpet-brass", tone=0.78, mass=revolve_area(BELL_PROFILE) * WALL_BELL * RHO_BRASS + TAU * BEAD[1] * math.pi * 0.0012 ** 2 * RHO_BRASS + 0.012, com=(0.43, 0, 0)))
    if s == 50:
        mv, cv = revolve_volume(MOUTHPIECE)
        bore = math.pi * 0.0019 ** 2 * 0.079
        P.append(Part("mouthpiece", "mouthpiece", "silver-plate", "ru", {"profile": MOUTHPIECE, "outline": [(-0.0655, 0.0)] + MOUTHPIECE_OUT + [(0.023, 0.0)]}, region="mouthpiece", tone=0.9, mass=(mv - bore) * RHO_BRASS, com=(cv, 0, 0)))
        P.append(Part("mouthpiece-rim-wear", "mouthpiece", "brass-worn", "ru", {"profile": [(-0.0657, 0.0112), (-0.0657, 0.0126), (-0.0652, 0.0131), (-0.0645, 0.0126), (-0.0650, 0.0112), (-0.0657, 0.0112)], "outline": [(-0.0657, 0.0), (-0.0657, 0.0131), (-0.0645, 0.0131), (-0.0645, 0.0)]}, region="mouthpiece", role="texture", tone=0.6, flags={"texture": True}))
    else:
        outer = [(-0.0655, 0.0104), (-0.0650, 0.0133), (-0.0595, 0.0126), (-0.0380, 0.0079), (0.0000, 0.0050), (0.0230, 0.0047)] if s == 100 else [(-0.0655, 0.0133), (-0.0380, 0.0080), (0.0230, 0.0050)]
        mv, cv = revolve_volume(MOUTHPIECE)
        P.append(Part("mouthpiece", "mouthpiece", "silver-plate", "ru", {"profile": outer}, region="mouthpiece", tone=0.9, mass=(mv - math.pi * 0.0019 ** 2 * 0.079) * RHO_BRASS, com=(cv, 0, 0)))
    return P


# ---------------------------------------------------------------- case, lid and contents (world coordinates of the rest pose)
XI = L_CASE / 2 - SHELL
ZI = W_CASE / 2 - SHELL
Z_DIV = (0.000, 0.015)
Z_COMP = 0.090


def case_parts(s):
    """🧳️ The worn rectangular case, its lid, bed and hardware at scale 1:s."""
    P = []
    h, l = L_CASE / 2, W_CASE / 2
    shell = {"material": "case-ply-tolex", "region": "case-shell", "tone": 0.12}
    lining = {"material": "plush-foam", "region": "case-lining", "tone": 0.32}
    hw = {"material": "nickel-steel", "region": "case-hardware", "role": "detail", "tone": 0.7}
    if s == 50:
        for pid, lo, hi in [("bottom", (-h, 0.0, -l), (h, SHELL, l)), ("wall-near", (-h, SHELL, -l), (h, H_BASE, -ZI)), ("wall-far", (-h, SHELL, ZI), (h, H_BASE, l)),
                            ("wall-mouthpiece-end", (-h, SHELL, -ZI), (-XI, H_BASE, ZI)), ("wall-bell-end", (XI, SHELL, -ZI), (h, H_BASE, ZI))]:
            m, c = box_mass(lo, hi, RHO_CASE)
            P.append(cbox(pid, lo, hi, flags={"sheet": True} if pid == "bottom" else {}, mass=m, com=cframe(*c), **shell))
        m, c = box_mass((-XI, SHELL, -ZI), (XI, BED_TOP, ZI), RHO_FOAM)
        P.append(cbox("bed-floor", (-XI, SHELL, -ZI), (XI, BED_TOP, ZI), mass=m, com=cframe(*c), **lining))
        for pid, lo, hi in [("lining-near", (-XI, BED_TOP, -ZI), (XI, H_BASE - 0.004, -ZI + LINE)), ("lining-far", (-XI, BED_TOP, ZI - LINE), (XI, H_BASE - 0.004, ZI)),
                            ("lining-mouthpiece-end", (-XI, BED_TOP, -ZI + LINE), (-XI + LINE, H_BASE - 0.004, ZI - LINE)), ("lining-bell-end", (XI - LINE, BED_TOP, -ZI + LINE), (XI, H_BASE - 0.004, ZI - LINE))]:
            m, c = box_mass(lo, hi, 150)
            P.append(cbox(pid, lo, hi, flags={"sheet": True}, mass=m, com=cframe(*c), **lining))
        hi_div = 0.075
    else:
        outer = rect(-h, h, -l, l)
        inner = rect(-XI, XI, -ZI, ZI)
        m = (L_CASE * W_CASE - 2 * XI * 2 * ZI) * (H_BASE - SHELL) * RHO_CASE
        P.append(Part("walls", "case", "case-ply-tolex", "pv", {"profile": [cframe(x, 0, z)[0::2] for x, z in outer], "holes": [[cframe(x, 0, z)[0::2] for x, z in inner]], "v0": -GRIP[1], "v1": H_BASE - GRIP[1]}, region="case-shell", tone=0.12, mass=m, com=cframe(0, 0.063, 0), draw=False))
        for pid, lo, hi in [("wall-near", (-h, 0.0, -l), (h, H_BASE, -ZI)), ("wall-far", (-h, 0.0, ZI), (h, H_BASE, l)), ("wall-mouthpiece-end", (-h, 0.0, -ZI), (-XI, H_BASE, ZI)), ("wall-bell-end", (XI, 0.0, -ZI), (h, H_BASE, ZI))]:
            P.append(cbox(pid, lo, hi, solid3d=False, **shell))
        top = BED_TOP if s == 100 else 0.026
        m1, _ = box_mass((-h, 0, -l), (h, SHELL, l), RHO_CASE)
        m2, _ = box_mass((-XI, SHELL, -ZI), (XI, BED_TOP, ZI), RHO_FOAM)
        P.append(cbox("bottom-and-bed", (-XI, 0.0, -ZI), (XI, top, ZI), material="case-ply-tolex", region="case-shell", tone=0.12, mass=m1 + m2 + 0.05, com=cframe(0, 0.008, 0)))
        P.append(cbox("bed-lining", (-XI, top, -ZI), (XI, top + 0.003, ZI), flags={"sheet": True}, mass=0.0, com=cframe(0, top, 0), **lining))
        hi_div = 0.075
    m, c = box_mass((-XI + LINE, BED_TOP, Z_DIV[0]), (XI - LINE, hi_div, Z_DIV[1]), RHO_FOAM)
    P.append(cbox("divider", (-XI + (LINE if s == 50 else 0), BED_TOP if s < 200 else 0.026, Z_DIV[0]), (XI - (LINE if s == 50 else 0), hi_div, Z_DIV[1]), mass=m, com=cframe(*c), **lining))
    lead_top = TRUMPET_AT[1] - 0.0065
    if s < 200:
        m, c = box_mass((-0.185, BED_TOP, -0.100), (0.245, lead_top, 0.0), RHO_FOAM)
        P.append(cbox("bed-block", (-0.185, BED_TOP, -0.100), (0.245, lead_top, Z_DIV[0]), mass=m, com=cframe(*c), **lining))
        mp_bottom = TRUMPET_AT[1] - 0.0133
        P.append(cbox("mouthpiece-holder", (-0.305, BED_TOP, -0.106), (-0.212, mp_bottom, -0.074), mass=0.004, com=cframe(-0.26, 0.024, -0.09), **lining))
    else:
        m, c = box_mass((-0.31, 0.026, -0.105), (0.245, lead_top, 0.0), RHO_FOAM)
        P.append(cbox("bed-block", (-0.31, 0.026, -0.105), (0.245, lead_top, Z_DIV[0]), mass=m, com=cframe(*c), **lining))
    if s == 50:
        for i, x in enumerate((-0.20, 0.20)):
            P.append(cbox(f"latch-{i + 1}", (x - 0.020, 0.090, -l - 0.008), (x + 0.020, 0.126, -l), mass=0.030, com=cframe(x, 0.108, -l - 0.004), **dict(hw, role="body")))
            P.append(Part(f"hinge-{i + 1}", "case", "nickel-steel", "cap_u", {"u0": x - 0.030 - GRIP[0], "u1": x + 0.030 - GRIP[0], "v": HINGE[1] - GRIP[1], "w": HINGE[2] - GRIP[2], "r": 0.0050}, region="case-hardware", role="detail", tone=0.7, mass=0.020, com=cframe(x, HINGE[1], HINGE[2])))
        for i, (sx, sz) in enumerate([(-1, -1), (1, -1), (-1, 1), (1, 1)]):
            lo = (min(sx * (h + 0.001), sx * (h - 0.028)), 0.0, min(sz * (l + 0.001), sz * (l - 0.028)))
            hi = (max(sx * (h + 0.001), sx * (h - 0.028)), 0.028, max(sz * (l + 0.001), sz * (l - 0.028)))
            P.append(cbox(f"corner-{i + 1}", lo, hi, mass=0.020, com=cframe(sx * (h - 0.013), 0.014, sz * (l - 0.013)), **hw))
        for pid, lo, hi in [("valance-near", (-h, H_BASE - 0.012, -l - 0.001), (h, H_BASE, -l)), ("valance-far", (-h, H_BASE - 0.012, l), (h, H_BASE, l + 0.001))]:
            P.append(cbox(pid, lo, hi, mass=0.64 * 0.012 * 0.001 * RHO_ALU, com=cframe(0, H_BASE - 0.006, (lo[2] + hi[2]) / 2), material="nickel-steel", region="case-hardware", role="body", tone=0.75))
        P.append(Part("handle-grip", "case", "handle-pvc", "cap_u", {"u0": 0.255, "u1": 0.385, "v": 0.0, "w": 0.0, "r": 0.011}, region="case-hardware", tone=0.15, mass=0.080, com=(0.32, 0, 0)))
        for i, x in enumerate((-0.075, 0.055)):
            P.append(cbox(f"handle-post-{i + 1}", (x, GRIP[1] - 0.012, -0.190), (x + 0.020, GRIP[1] + 0.012, -l), material="handle-pvc", region="case-hardware", tone=0.18, mass=0.020, com=cframe(x + 0.01, GRIP[1], -0.178)))
    elif s == 100:
        hwb = dict(hw, role="body")
        for i, x in enumerate((-0.20, 0.20)):
            P.append(cbox(f"latch-{i + 1}", (x - 0.020, 0.090, -l - 0.008), (x + 0.020, 0.126, -l), mass=0.030, com=cframe(x, 0.108, -l - 0.004), **hwb))
        for i, (sx, sz) in enumerate([(-1, -1), (1, -1), (-1, 1), (1, 1)]):
            lo = (min(sx * (h + 0.001), sx * (h - 0.028)), 0.0, min(sz * (l + 0.001), sz * (l - 0.028)))
            hi = (max(sx * (h + 0.001), sx * (h - 0.028)), 0.028, max(sz * (l + 0.001), sz * (l - 0.028)))
            P.append(cbox(f"corner-{i + 1}", lo, hi, mass=0.020, com=cframe(sx * (h - 0.013), 0.014, sz * (l - 0.013)), **hwb))
        for pid, lo, hi in [("valance-near", (-h, H_BASE - 0.012, -l - 0.001), (h, H_BASE, -l)), ("valance-far", (-h, H_BASE - 0.012, l), (h, H_BASE, l + 0.001))]:
            P.append(cbox(pid, lo, hi, mass=0.64 * 0.012 * 0.001 * RHO_ALU, com=cframe(0, H_BASE - 0.006, (lo[2] + hi[2]) / 2), material="nickel-steel", region="case-hardware", role="body", tone=0.75))
        P.append(Part("hinge-line", "case", "nickel-steel", "cap_u", {"u0": 0.090, "u1": 0.550, "v": HINGE[1] - GRIP[1], "w": HINGE[2] - GRIP[2], "r": 0.0090}, region="case-hardware", tone=0.7, mass=0.040, com=cframe(0, HINGE[1], HINGE[2])))
        P.append(cbox("handle", (-0.075, GRIP[1] - 0.012, -0.201), (0.075, GRIP[1] + 0.012, -l), material="handle-pvc", region="case-hardware", tone=0.15, mass=0.120 + 0.06, com=cframe(0, GRIP[1], -0.185)))
    else:
        P.append(Part("hinge-line", "case", "nickel-steel", "cap_u", {"u0": 0.090, "u1": 0.550, "v": HINGE[1] - GRIP[1], "w": HINGE[2] - GRIP[2], "r": 0.0130}, region="case-hardware", tone=0.7, mass=0.040 + 0.08 + 0.02 + 0.06, com=cframe(0, HINGE[1], HINGE[2])))
        P.append(cbox("handle", (-0.075, GRIP[1] - 0.012, -0.201), (0.075, GRIP[1] + 0.012, -l), material="handle-pvc", region="case-hardware", tone=0.15, mass=0.120 + 0.06, com=cframe(0, GRIP[1], -0.185)))
    if s == 50:
        tex = {"role": "texture", "flags": {"texture": True}}
        P.append(cbox("tolex-tear", (0.228, 0.028, -l - 0.0004), (0.292, 0.066, -l), material="tolex-torn", region="case-shell", tone=0.55, **tex))
        P.append(cbox("scuffed-edge", (-0.250, 0.0, -l - 0.0004), (0.200, 0.010, -l), material="tolex-torn", region="case-shell", tone=0.4, **tex))
        P.append(cbox("cloth", (0.095, BED_TOP, 0.035), (0.235, BED_TOP + 0.010, 0.145), material="microfibre", region="case-lining", tone=0.6, mass=0.012, com=cframe(0.165, BED_TOP + 0.005, 0.09)))
    else:
        P.append(cbox("cloth", (0.095, BED_TOP + 0.003 if s == 100 else 0.029, 0.035), (0.235, (BED_TOP + 0.003 if s == 100 else 0.029) + 0.010, 0.145), material="microfibre", region="case-lining", tone=0.6, mass=0.012, com=cframe(0.165, BED_TOP + 0.005, 0.09)))
    lid = {"material": "case-ply-tolex", "region": "case-shell", "tone": 0.14}
    top0 = H_BASE + H_LID - SHELL
    if s == 50:
        for pid, lo, hi in [("lid-top", (-h, top0, -l), (h, H_BASE + H_LID, l)), ("lid-rim-near", (-h, H_BASE, -l), (h, top0, -ZI)), ("lid-rim-far", (-h, H_BASE, ZI), (h, top0, l)),
                            ("lid-rim-mouthpiece-end", (-h, H_BASE, -ZI), (-XI, top0, ZI)), ("lid-rim-bell-end", (XI, H_BASE, -ZI), (h, top0, ZI))]:
            m, c = box_mass(lo, hi, RHO_CASE)
            P.append(lbox(pid, lo, hi, flags={"sheet": True} if pid == "lid-top" else {}, mass=m, com=lframe(*c), **lid))
        P.append(lbox("lid-lining", (-XI, top0 - 0.004, -ZI), (XI, top0, ZI), flags={"sheet": True}, mass=2 * XI * 2 * ZI * 0.004 * 120, com=lframe(0, top0 - 0.002, 0), material="plush-foam", region="case-lining", tone=0.32))
        for i, (sx, sz) in enumerate([(-1, -1), (1, -1), (-1, 1), (1, 1)]):
            lo = (min(sx * (h + 0.001), sx * (h - 0.028)), H_BASE + H_LID - 0.028, min(sz * (l + 0.001), sz * (l - 0.028)))
            hi = (max(sx * (h + 0.001), sx * (h - 0.028)), H_BASE + H_LID + 0.001, max(sz * (l + 0.001), sz * (l - 0.028)))
            P.append(lbox(f"lid-corner-{i + 1}", lo, hi, mass=0.020, com=lframe(sx * (h - 0.013), H_BASE + H_LID - 0.014, sz * (l - 0.013)), **hw))
        P.append(lbox("inventory-sticker", (-0.120, H_BASE + H_LID, -0.070), (-0.030, H_BASE + H_LID + 0.0004, -0.010), material="tolex-torn", region="case-shell", role="texture", tone=0.95, flags={"texture": True}))
        P.append(lbox("lid-scuff", (0.150, H_BASE + H_LID, 0.060), (0.290, H_BASE + H_LID + 0.0004, 0.140), material="tolex-torn", region="case-shell", role="texture", tone=0.35, flags={"texture": True}))
    else:
        outer = [lframe(x, 0, z)[0::2] for x, z in rect(-h, h, -l, l)]
        inner = [lframe(x, 0, z)[0::2] for x, z in rect(-XI, XI, -ZI, ZI)]
        m = (L_CASE * W_CASE - 2 * XI * 2 * ZI) * (H_LID - SHELL) * RHO_CASE
        P.append(Part("lid-rims", "lid", "case-ply-tolex", "pv", {"profile": outer, "holes": [inner], "v0": 0.0, "v1": top0 - H_BASE}, region="case-shell", tone=0.14, mass=m, com=lframe(0, H_BASE + 0.025, 0), draw=False))
        for pid, lo, hi in [("lid-rim-near", (-h, H_BASE, -l), (h, top0, -ZI)), ("lid-rim-far", (-h, H_BASE, ZI), (h, top0, l)), ("lid-rim-mouthpiece-end", (-h, H_BASE, -ZI), (-XI, top0, ZI)), ("lid-rim-bell-end", (XI, H_BASE, -ZI), (h, top0, ZI))]:
            P.append(lbox(pid, lo, hi, solid3d=False, **lid))
        m1, _ = box_mass((-h, top0, -l), (h, H_BASE + H_LID, l), RHO_CASE)
        P.append(lbox("lid-top", (-h, top0 - (0.0 if s == 100 else 0.004), -l), (h, H_BASE + H_LID, l), flags={"sheet": True}, mass=m1 + 0.08 + 0.08, com=lframe(0, top0, 0), **lid))
        P.append(lbox("lid-lining", (-XI, top0 - (0.004 if s == 100 else 0.014), -ZI), (XI, top0 - (0.0 if s == 100 else 0.004), ZI), flags={"sheet": True}, mass=0.0, com=lframe(0, top0, 0), material="plush-foam", region="case-lining", tone=0.32))
    return P


STRAIGHT = [(0.0, 0.0480), (0.004, 0.0490), (0.010, 0.0482), (0.040, 0.0435), (0.070, 0.0375), (0.100, 0.0300), (0.135, 0.0215)]
CUP = [(0.0, 0.0165), (0.010, 0.0180), (0.050, 0.0280), (0.086, 0.0380), (0.088, 0.0525), (0.128, 0.0525), (0.130, 0.0505)]
BOTTLE = [(0.0, 0.0150), (0.002, 0.0160), (0.060, 0.0160), (0.066, 0.0110), (0.068, 0.0065)]
BOTTLE_CAP = [(0.068, 0.0068), (0.082, 0.0060), (0.088, 0.0030), (0.090, 0.0010)]


def lying_cone(profile, base_x, z, tip_r):
    """🛏️ Rest placement of a cone lying on the bed: bone origin at the wide end, tilted so that the wide rim and the narrow end both touch the bed."""
    big = max(r for _, r in profile)
    a_big = max(a for a, r in profile if r == big)
    length = profile[-1][0] - a_big
    tilt = math.degrees(math.atan2(big - tip_r, length))
    y0 = BED_TOP + big * math.cos(math.radians(tilt)) + a_big * math.sin(math.radians(tilt))
    return (base_x, y0, z, -tilt)


MUTE_AT = lying_cone(STRAIGHT, -0.305, Z_COMP, 0.0215 + 0.003)
CUP_PROFILE_FWD = [(0.130 - a, r) for a, r in reversed(CUP)]
CUP_AT = lying_cone(CUP_PROFILE_FWD, -0.160, Z_COMP, 0.0165)
BOTTLE_AT = (-0.010, BED_TOP + 0.016, Z_COMP + 0.020, 0.0)


def contents_parts(s):
    """🧰️ Straight mute, cup mute and valve oil in the accessory compartment at scale 1:s."""
    P = []
    prof = STRAIGHT if s == 50 else (STRAIGHT[::2] if s == 100 else [STRAIGHT[1], STRAIGHT[3], STRAIGHT[-1]])
    P.append(Part("straight-mute", "straight-mute", "aluminium-spun", "ru", {"profile": prof}, region="straight-mute", tone=0.82, mass=(revolve_area(STRAIGHT) + math.pi * 0.049 ** 2) * 0.0007 * RHO_ALU, com=(0.055, 0, 0)))
    if s < 200:
        cork = [(0.098, 0.0305), (0.098, 0.0335), (0.130, 0.0258), (0.130, 0.0228)] if s == 50 else [(0.095, 0.0312), (0.095, 0.0345), (0.130, 0.0260), (0.130, 0.0225)]
        P.append(Part("mute-corks", "straight-mute", "cork", "ru", {"profile": cork + [cork[0]], "outline": [(0.098, 0.0), (0.098, 0.0335), (0.130, 0.0258), (0.130, 0.0)]}, region="straight-mute", role="detail", tone=0.45, mass=0.003, com=(0.114, 0, 0)))
    cup = CUP_PROFILE_FWD if s == 50 else ([CUP_PROFILE_FWD[i] for i in (0, 1, 2, 3, 5, 6)] if s == 100 else [CUP_PROFILE_FWD[i] for i in (1, 2, 3, 6)])
    P.append(Part("cup-mute", "cup-mute", "fibre-painted", "ru", {"profile": cup}, region="cup-mute", tone=0.2, mass=0.038 * 0.002 * RHO_FIBRE + 0.004, com=(0.040, 0, 0)))
    if s < 200:
        P.append(Part("bottle", "oil-bottle", "pe-bottle", "ru", {"profile": BOTTLE if s == 50 else [BOTTLE[0], BOTTLE[2], BOTTLE[4]]}, region="accessories", tone=0.88, mass=0.012 + 0.040e-3 * RHO_OIL, com=(0.032, 0, 0)))
        P.append(Part("bottle-cap", "oil-bottle", "pe-bottle", "ru", {"profile": BOTTLE_CAP if s == 50 else [BOTTLE_CAP[0], BOTTLE_CAP[2]]}, region="accessories", role="detail", tone=0.3, mass=0.002, com=(0.078, 0, 0)))
    else:
        P.append(Part("bottle", "oil-bottle", "pe-bottle", "ru", {"profile": [(0.0, 0.0160), (0.090, 0.0160)]}, region="accessories", tone=0.88, mass=0.012 + 0.040e-3 * RHO_OIL + 0.002, com=(0.034, 0, 0)))
    return P


def parts(s):
    """🧩️ Every part of scale 1:s; the boxes and capsules of the unrotated case bone are drawn as lateral sweeps."""
    plist = case_parts(s) + contents_parts(s) + trumpet_parts(s)
    for p in plist:
        if p.bone == "case" and p.kind in ("box", "cap_u"):
            p.zrest = GRIP[2]
    return plist


# ---------------------------------------------------------------- rig
def bone(bid, parent, en, de, x, y, z, angle, length, joint, abduction=None, twist=None):
    """🦴️ One bone."""
    b = {"id": bid, "parent": parent, "title": T(en, de), "x": r6(x), "y": r6(y)}
    if z:
        b["z"] = r6(z)
    b["angleDeg"] = r6(angle)
    if abduction:
        b["abductionDeg"] = abduction
    if twist:
        b["twistDeg"] = twist
    b.update({"lengthM": r6(length), "joint": joint})
    return b


FIXED = {"type": "fixed"}
TL = cframe(*TRUMPET_AT)
RIG_BONES = [
    bone("floor", None, "Floor of the dealer's shop under the case", "Boden des Händlerladens unter dem Koffer", 0, 0, 0, 0, 0, FIXED),
    bone("case", "floor", "Case body along its length, from the mouthpiece end, on the axis of the handle grip; rolls under the hand when carried", "Kofferunterteil in Längsrichtung vom Mundstückende, auf der Achse des Tragegriffs; rollt beim Tragen unter der Hand", GRIP[0], GRIP[1], GRIP[2], 0, L_CASE, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -110, "twistMaxDeg": 0}),
    bone("lid", "case", "Lid on the hinge axis along the far edge; rests open at 100 degrees against its stops", "Deckel auf der Scharnierachse an der hinteren Kante; ruht 100 Grad offen an seinen Anschlägen", 0, HINGE[1] - GRIP[1], HINGE[2] - GRIP[2], 0, L_CASE, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": LID_OPEN}, twist=-LID_OPEN),
    bone("straight-mute", "case", "Straight mute lying in the compartment, from its closed wide end to its corked tip", "Straight-Dämpfer im Fach liegend, vom geschlossenen weiten Ende zur gekorkten Spitze", *[c for c in cframe(MUTE_AT[0], MUTE_AT[1], MUTE_AT[2])], MUTE_AT[3], 0, FIXED),
    bone("cup-mute", "case", "Cup mute lying in the compartment, from its cup to its corked tip", "Cup-Dämpfer im Fach liegend, vom Becher zur gekorkten Spitze", *[c for c in cframe(CUP_AT[0], CUP_AT[1], CUP_AT[2])], CUP_AT[3], 0, FIXED),
    bone("oil-bottle", "case", "Valve-oil bottle lying in the compartment", "Ventilölflasche im Fach liegend", *[c for c in cframe(BOTTLE_AT[0], BOTTLE_AT[1], BOTTLE_AT[2])], 0, 0, FIXED),
    bone("trumpet-lift", "case", "Lift of the trumpet out of its bed to the player's mouth", "Anheben der Trompete aus ihrem Bett an den Mund des Spielers", TL[0], TL[1], TL[2], 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 1.46}),
    bone("trumpet", "trumpet-lift", "Trumpet from the receiver end towards the bell, lying on its right side in the case; pitches and rolls upright when played", "Trompete vom Mundrohrende zum Schallstück, im Koffer auf der rechten Seite liegend; neigt und rollt beim Spielen aufrecht", 0, 0, 0, 0, BEAD[0], {"type": "revolute", "minDeg": -30, "maxDeg": 30, "twistMinDeg": -90, "twistMaxDeg": 0}),
    bone("bell", "trumpet", "Bell section: bow, bell branch and bell on the bell axis", "Schallstückteil: Bogen, Schallrohr und Schallstück auf der Schallstückachse", 0, BELL_W, 0, 0, BEAD[0], FIXED),
    bone("mouthpiece", "trumpet", "Mouthpiece, lying in its holder behind the receiver; pushed into the receiver to play", "Mundstück, in seinem Halter hinter dem Mundrohr liegend; zum Spielen ins Mundrohr gesteckt", -MP_OUT, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": MP_OUT}),
    bone("main-slide", "trumpet", "Main tuning slide crook", "Bogen des Hauptstimmzugs", 0.400, 0, 0.032, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.02}),
    bone("slide-1", "trumpet", "First valve slide, pushed out by the left thumb", "Zug des ersten Ventils, vom linken Daumen ausgezogen", 0.150, 0, 0.038, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": -0.02, "maxM": 0}),
    bone("slide-2", "trumpet", "Second valve slide", "Zug des zweiten Ventils", 0.272, 0, 0.072, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.01}),
    bone("slide-3", "trumpet", "Third valve slide, kicked out by the left ring finger", "Zug des dritten Ventils, vom linken Ringfinger ausgestoßen", 0.120, BELL_W, 0.062, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": -0.022, "maxM": 0}),
    bone("ring-3", "slide-3", "Finger ring of the third slide", "Fingerring des dritten Zugs", RING_AT[0] - 0.120, 0, -RING_AT[1] - 0.062, 90, 0, FIXED),
]
for i, u in enumerate(U_VALVE):
    RIG_BONES.append(bone(f"casing-{i + 1}", "trumpet", f"Valve casing {i + 1} along its axis towards the finger button", f"Ventilgehäuse {i + 1} entlang seiner Achse zum Drücker", u, CASING_W, 0, 0, 0, FIXED, abduction=-90))
    RIG_BONES.append(bone(f"piston-{i + 1}", f"casing-{i + 1}", f"Piston {i + 1} with stem and finger button, pressed 17 mm", f"Kolben {i + 1} mit Stange und Drücker, 17 mm gedrückt", 0.026, 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": -STROKE, "maxM": 0}))


def frames(pose):
    """📐️ World frames (origin, rotation columns u, v, w) of every bone for a pose given as {bone: offset}, enough for bounds and the hand placements here."""
    def mul(a, b):
        return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]

    def rw(d):
        c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
        return [[c, -s, 0], [s, c, 0], [0, 0, 1]]

    def rv(d):
        c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
        return [[c, 0, -s], [0, 1, 0], [s, 0, c]]

    def ru(d):
        c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
        return [[1, 0, 0], [0, c, s], [0, -s, c]]

    def rot(a, b, t):
        return mul(mul(rw(a), rv(b)), ru(t))

    out = {}
    root = pose.get("__root", {})
    for b in RIG_BONES:
        off = pose.get(b["id"], {})
        if b["parent"] is None:
            out[b["id"]] = ([0.0, root.get("y", 0.0), 0.0], rot(0, 0, 0))
            continue
        po, pr = out[b["parent"]]
        local = [b["x"] + off.get("dx", 0.0), b["y"] + off.get("dy", 0.0), b.get("z", 0.0)]
        o = [po[i] + sum(pr[i][k] * local[k] for k in range(3)) for i in range(3)]
        r = mul(mul(pr, rot(b["angleDeg"], b.get("abductionDeg", 0), b.get("twistDeg", 0))), rot(off.get("angleDeg", 0), off.get("abductionDeg", 0), off.get("twistDeg", 0)))
        out[b["id"]] = (o, r)
    return out


def world(fr, bid, p):
    """🌍️ A bone-local point in world coordinates."""
    o, r = fr[bid]
    return [o[i] + sum(r[i][k] * p[k] for k in range(3)) for i in range(3)]


def part_points(p):
    """📍️ Sample points of a part in its bone frame (corners, profile rings, path rings) for bounds."""
    g = p.geo
    if p.kind == "box":
        lo, hi = g["lo"], g["hi"]
        return [(a, b, c) for a in (lo[0], hi[0]) for b in (lo[1], hi[1]) for c in (lo[2], hi[2])]
    if p.kind == "pv":
        return [(a, v, b) for a, b in g["profile"] for v in (g["v0"], g["v1"])]
    if p.kind == "ru":
        return [(a, r * math.cos(t), r * math.sin(t)) for a, r in g["profile"] for t in [TAU * k / 16 for k in range(16)]]
    if p.kind == "tube":
        return [(a, g["v"] + r * math.cos(t), b + r * math.sin(t)) for a, b, r in g["path"] for t in [TAU * k / 8 for k in range(8)]]
    if p.kind == "cap_v":
        return [(g["u"] + g["r"] * math.cos(t), v, g["w"] + g["r"] * math.sin(t)) for v in (g["v0"], g["v1"]) for t in [TAU * k / 8 for k in range(8)]]
    if p.kind == "cap_u":
        return [(u, g["v"] + g["r"] * math.cos(t), g["w"] + g["r"] * math.sin(t)) for u in (g["u0"], g["u1"]) for t in [TAU * k / 8 for k in range(8)]]
    return []


def bounds(plist, pose):
    """📦️ World bounding box of the non-texture parts in a pose."""
    fr = frames(pose)
    xs, ys, zs = [], [], []
    for p in plist:
        if p.flags.get("texture"):
            continue
        for q in part_points(p):
            x, y, z = world(fr, p.bone, q)
            xs.append(x)
            ys.append(y)
            zs.append(z)
    return (min(xs), max(xs)), (min(ys), max(ys)), (min(zs), max(zs))


# ---------------------------------------------------------------- masses
def bone_masses(plist):
    """⚖️ Mass and centre (bone-local u, v) per bone from the parts' section-times-density masses."""
    acc = {}
    for p in plist:
        if p.mass <= 0:
            continue
        m, (u, v, w) = acc.get(p.bone, (0.0, (0.0, 0.0, 0.0)))
        c = p.com
        acc[p.bone] = (m + p.mass, (u + p.mass * c[0], v + p.mass * c[1], w + p.mass * c[2]))
    return {b: (m, (s[0] / m, s[1] / m, s[2] / m)) for b, (m, s) in acc.items()}


# ---------------------------------------------------------------- anchors
P50 = parts(50)
MASS = bone_masses(P50)
M_TOTAL = sum(m for m, _ in MASS.values())
M_TRUMPET = sum(MASS[b][0] for b in MASS if b in ("trumpet", "bell", "main-slide", "slide-1", "slide-2", "slide-3", "ring-3") or b.startswith("casing") or b.startswith("piston"))
M_CASE = MASS["case"][0] + MASS["lid"][0]


def case_com():
    """⚖️ Centre of mass of everything that hangs on the handle (case frame u, v, w)."""
    fr = frames({})
    o, r = fr["case"]
    m, acc = 0.0, [0.0, 0.0, 0.0]
    for b, (mb, c) in MASS.items():
        p = world(fr, b, c)
        loc = [sum(r[k][i] * (p[k] - o[k]) for k in range(3)) for i in range(3)]
        m += mb
        acc = [acc[i] + mb * loc[i] for i in range(3)]
    return [a / m for a in acc]


CCOM = case_com()
CARRY_TWIST = round(math.degrees(math.atan2(-CCOM[2], -CCOM[1])), 2)

ANCHORS = [
    {"id": "case-floor", "bone": "case", "x": r6(L_CASE / 2), "y": r6(-GRIP[1]), "z": r6(-GRIP[2]), "kind": "contact", "title": T("Bottom of the case on the shop floor", "Kofferboden auf dem Ladenboden"), "support": "contact", "normalDeg": 90, "footprintLengthM": L_CASE, "footprintWidthM": W_CASE, "frictionCoefficient": 0.5},
    {"id": "grip", "bone": "case", "x": r6(L_CASE / 2), "y": 0, "kind": "support", "title": T("Hand round the handle grip", "Hand um den Tragegriff"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.03},
    {"id": "left-hand", "bone": "trumpet", "x": U_VALVE[1], "y": CASING_W, "z": 0.030, "kind": "support", "title": T("Left hand round the three valve casings", "Linke Hand um die drei Ventilgehäuse"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.07, "footprintWidthM": 0.04},
    {"id": "right-hand", "bone": "trumpet", "x": 0.212, "y": 0.0, "z": 0.014, "kind": "support", "title": T("Right thumb under the leadpipe between the first and second valve", "Rechter Daumen unter dem Mundrohr zwischen erstem und zweitem Ventil"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.015},
    {"id": "lips", "bone": "mouthpiece", "x": -0.0655, "y": 0.0, "kind": "load", "title": T("Lips on the mouthpiece rim", "Lippen am Mundstückrand")},
    {"id": "bell-mouth", "bone": "bell", "x": r6(BELL_PROFILE[-1][0]), "y": 0.0, "kind": "emitter", "title": T("Mouth of the bell, where the breath leaves", "Mündung des Schallstücks, wo der Atem austritt")},
]
for i in range(3):
    ANCHORS.append({"id": f"valve-{i + 1}", "bone": f"piston-{i + 1}", "x": 0.029, "y": 0.0, "kind": "load", "title": T(f"Finger on valve button {i + 1}", f"Finger auf Ventildrücker {i + 1}")})

REGIONS = ["case-shell", "case-lining", "case-hardware", "trumpet-brass", "worn-brass", "mouthpiece", "straight-mute", "cup-mute", "accessories"]


def binding(exposed, contacts, breath):
    """🌬️ The thermal binding of every region and the breath emitter."""
    regions = {}
    for r in REGIONS:
        e = exposed.get(r, 1)
        c = contacts.get(r)
        regions[r] = {"exposedFraction": r6(e if not c else min(e, 1 - c[0])), "sunlitFraction": 0}
        if c:
            regions[r]["contact"] = {"fraction": c[0], "tempC": c[1]}
    return {"regions": regions, "emitters": {"breath-bell": breath}}


INSIDE = {r: 0.1 for r in ("case-lining", "trumpet-brass", "worn-brass", "mouthpiece", "straight-mute", "cup-mute", "accessories")}
LIP_N = 25.0
VALVE_N = 4.0


def valve_loads():
    """👆️ The finger forces on the three valve buttons (driven by the clip tracks)."""
    return [{"id": f"valve-press-{i + 1}", "kind": "live", "title": T(f"Finger force on valve button {i + 1}", f"Fingerkraft auf Ventildrücker {i + 1}"), "anchor": f"valve-{i + 1}", "fxN": 0, "fyN": round(-VALVE_N * math.cos(math.radians(PITCH)), 2), "source": "authored-estimate", "reference": "brass-catalogues"} for i in range(3)]


PLAYED = {"trumpet-lift": {"dy": r6(PLAY_Y - TRUMPET_AT[1])}, "trumpet": {"angleDeg": PITCH, "twistDeg": -90.0}, "mouthpiece": {"dx": MP_OUT}}
CLOSED = {"lid": {"twistDeg": LID_OPEN}}
CARRIED = {"lid": {"twistDeg": LID_OPEN}, "case": {"twistDeg": CARRY_TWIST}}

POSES = [
    {"id": "stored", "title": T("Stored open at the dealer", "Offen beim Händler gelagert"), "description": T("The case lies on the shop floor with its lid open against its stops; the trumpet lies on its right side in the plush bed with the bell towards the far end, the mouthpiece in its holder behind the receiver, the straight mute, the cup mute, the valve oil and a polishing cloth in the compartment.", "Der Koffer liegt auf dem Ladenboden, der Deckel offen an seinen Anschlägen; die Trompete liegt auf ihrer rechten Seite im Plüschbett, das Schallstück zum hinteren Ende, das Mundstück in seinem Halter hinter dem Mundrohr, Straight-Dämpfer, Cup-Dämpfer, Ventilöl und ein Poliertuch im Fach."),
     "bones": {}, "ground": "authored", "contacts": ["case-floor"], "thermal": binding({}, {"case-shell": (0.12, 19)}, 0), "loads": []},
    {"id": "closed", "title": T("Case closed", "Koffer geschlossen"), "description": T("The lid is shut over the trumpet and its accessories and the two latches are closed; the case lies on the floor as in the depot.", "Der Deckel ist über Trompete und Zubehör geschlossen, die zwei Schlösser sind zu; der Koffer liegt wie im Lager auf dem Boden."),
     "bones": CLOSED, "ground": "authored", "contacts": ["case-floor"], "thermal": binding(INSIDE, {"case-shell": (0.12, 19)}, 0), "loads": []},
    {"id": "carried", "title": T("Carried by the handle", "Am Griff getragen"), "description": T(f"The closed case hangs from the hand round its handle at 0.76 m, rolled {abs(CARRY_TWIST):.0f} degrees on to its side so that its centre of mass lies under the grip; the trumpet stays in its bed.", f"Der geschlossene Koffer hängt an der Hand um seinen Griff auf 0,76 m, um {abs(CARRY_TWIST):.0f} Grad auf die Seite gerollt, so dass sein Schwerpunkt unter dem Griff liegt; die Trompete bleibt in ihrem Bett."),
     "bones": CARRIED, "root": {"y": CARRY_Y}, "ground": "authored", "contacts": ["grip"], "thermal": binding(INSIDE, {"case-hardware": (0.06, 32)}, 0), "loads": []},
    {"id": "played", "title": T("Played standing", "Im Stehen gespielt"), "description": T("The trumpet is taken out of the open case and played standing: the mouthpiece in the receiver at the lips at 1.50 m, the bell lowered 6 degrees, the left hand round the valve casings, the right thumb under the leadpipe and three fingers on the buttons; the player's warm moist breath leaves at the bell.", "Die Trompete ist aus dem offenen Koffer genommen und wird im Stehen gespielt: das Mundstück im Mundrohr an den Lippen auf 1,50 m, das Schallstück 6 Grad gesenkt, die linke Hand um die Ventilgehäuse, der rechte Daumen unter dem Mundrohr und drei Finger auf den Drückern; der warme, feuchte Atem des Spielers tritt am Schallstück aus."),
     "bones": PLAYED, "ground": "authored", "contacts": ["case-floor", "left-hand", "right-hand"],
     "thermal": binding({}, {"case-shell": (0.12, 19), "worn-brass": (0.55, 33), "trumpet-brass": (0.06, 33), "mouthpiece": (0.12, 35)}, 1),
     "loads": [{"id": "lip-pressure", "kind": "live", "title": T("Mouthpiece pressure of the lips, 25 N along the leadpipe", "Mundstückdruck der Lippen, 25 N entlang des Mundrohrs"), "anchor": "lips", "fxN": round(LIP_N * math.cos(math.radians(PITCH)), 2), "fyN": round(LIP_N * math.sin(math.radians(PITCH)), 2), "source": "authored-estimate", "reference": "barbenel-1988"}] + valve_loads()},
]

NOTES = [("c4", ()), ("d4", (1, 3)), ("e4", (1, 2)), ("f4", (1,)), ("g4", ()), ("a4", (1, 2)), ("b4", (2,)), ("c5", ())]
NOTE_S = 0.5


def valve_state(pressed, kick):
    """🎹️ Piston and slide offsets of one fingering."""
    b = {f"piston-{i}": {"dx": -STROKE if i in pressed else 0.0} for i in (1, 2, 3)}
    b["slide-3"] = {"dx": -0.012 if kick else 0.0}
    return b


def valve_clip():
    """🎺️ A C major scale in 4 s: each fingering is reached in 50 ms and held; the third slide is kicked out on the D."""
    keys, tracks = [], {i: [] for i in (1, 2, 3)}
    prev = ()
    for n, (_, pressed) in enumerate(NOTES):
        t = n * NOTE_S
        keys.append({"t": r6(t), "pose": "played", "ease": "linear", "bones": valve_state(prev, prev == (1, 3))})
        keys.append({"t": r6(t + 0.05), "pose": "played", "ease": "step", "bones": valve_state(pressed, pressed == (1, 3))})
        for i in (1, 2, 3):
            tracks[i] += [{"t": r6(t), "v": 1.0 if i in prev else 0.0}, {"t": r6(t + 0.05), "v": 1.0 if i in pressed else 0.0}]
        prev = pressed
    keys.append({"t": 4.0, "pose": "played", "ease": "linear", "bones": valve_state((), False)})
    for i in (1, 2, 3):
        tracks[i].append({"t": 4.0, "v": 0.0})
    names = {"c4": ("C", "C"), "d4": ("D", "D"), "e4": ("E", "E"), "f4": ("F", "F"), "g4": ("G", "G"), "a4": ("A", "A"), "b4": ("B", "H"), "c5": ("high C", "hohes C")}
    return {"id": "valves", "title": T("Valve cycle: a C major scale", "Ventilspiel: eine C-Dur-Tonleiter"), "description": T("Eight notes of the written C major scale in 4 s, each fingering pressed in 50 ms (17 mm stroke) and held: open, 1+3 with the third slide kicked out 12 mm, 1+2, 1, open, 1+2, 2, open; the finger forces of 4 N follow the pistons.", "Acht Töne der notierten C-Dur-Tonleiter in 4 s, jeder Griff in 50 ms gedrückt (17 mm Hub) und gehalten: leer, 1+3 mit 12 mm ausgestoßenem dritten Zug, 1+2, 1, leer, 1+2, 2, leer; die Fingerkräfte von 4 N folgen den Kolben."),
            "durationS": 4.0, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys,
            "contacts": [{"anchor": a, "fromS": 0, "toS": 4.0} for a in ("case-floor", "left-hand", "right-hand")],
            "phases": [{"id": f"note-{nid}", "fromS": r6(n * NOTE_S), "toS": r6((n + 1) * NOTE_S), "title": T(f"Note {names[nid][0]}", f"Ton {names[nid][1]}")} for n, (nid, _) in enumerate(NOTES)],
            "events": [{"t": r6(n * NOTE_S), "kind": "load-on", "title": T(f"Fingering for {names[nid][0]}", f"Griff für {names[nid][1]}"), "target": f"valve-press-{(pressed or (1,))[0]}"} for n, (nid, pressed) in enumerate(NOTES) if pressed],
            "tracks": [{"target": f"load:valve-press-{i}", "ease": "step", "keys": tracks[i]} for i in (1, 2, 3)]}


CLIPS = [
    {"id": "carry", "title": T("Close and carry away", "Schließen und wegtragen"), "description": T(f"The lid is shut in 1.2 s, the hand takes the handle, lifts the case and lets it roll {abs(CARRY_TWIST):.0f} degrees on to its side until it hangs under the grip at 0.76 m.", f"Der Deckel wird in 1,2 s geschlossen, die Hand fasst den Griff, hebt den Koffer und lässt ihn um {abs(CARRY_TWIST):.0f} Grad auf die Seite rollen, bis er auf 0,76 m unter dem Griff hängt."),
     "durationS": 4.0, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.2, "pose": "closed", "ease": "smooth"}, {"t": 1.6, "pose": "closed", "ease": "smooth"}, {"t": 3.2, "pose": "carried", "ease": "smooth"}, {"t": 4.0, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": "case-floor", "fromS": 0, "toS": 1.7}, {"anchor": "grip", "fromS": 1.5, "toS": 4.0}],
     "phases": [{"id": "close", "fromS": 0, "toS": 1.2, "title": T("Closing the lid", "Deckel schließen")}, {"id": "grip", "fromS": 1.2, "toS": 1.6, "title": T("Taking the handle", "Griff fassen")}, {"id": "lift", "fromS": 1.6, "toS": 3.2, "title": T("Lifting", "Anheben")}, {"id": "carry", "fromS": 3.2, "toS": 4.0, "title": T("Carrying", "Tragen")}],
     "events": [{"t": 1.5, "kind": "contact-on", "title": T("Hand takes the handle", "Hand fasst den Griff"), "target": "grip"}, {"t": 1.7, "kind": "contact-off", "title": T("Case leaves the floor", "Koffer verlässt den Boden"), "target": "case-floor"}],
     "tracks": []},
    valve_clip(),
]

# ---------------------------------------------------------------- thermal
REGION_TITLES = {
    "case-shell": ("Case shell: tolex-covered plywood base and lid", "Koffer­schale: tolexbezogenes Sperrholz von Unterteil und Deckel", "case-ply-tolex", "vertical-plate", 0.33),
    "case-lining": ("Plush bed, linings, compartment divider and the polishing cloth lying on the bed", "Plüschbett, Auskleidung, Fachtrenner und das auf dem Bett liegende Poliertuch", "plush-foam", "horizontal-plate-up", 0.30),
    "case-hardware": ("Latches, hinges, corners, edge strips and handle", "Schlösser, Scharniere, Ecken, Kantenleisten und Griff", "nickel-steel", "vertical-plate", 0.03),
    "trumpet-brass": ("Lacquered brass of the trumpet: tubes, valve casings, slides and bell, inside and out", "Lackiertes Messing der Trompete: Rohre, Ventilgehäuse, Züge und Schallstück, innen und außen", "brass-lacquer", "cylinder", 0.012),
    "worn-brass": ("Bare worn brass of the valve casings and the leadpipe under the hands", "Blankgegriffenes Messing der Ventilgehäuse und des Mundrohrs unter den Händen", "brass-worn", "cylinder", 0.021),
    "mouthpiece": ("Silver-plated mouthpiece", "Versilbertes Mundstück", "silver-plate", "cylinder", 0.02),
    "straight-mute": ("Aluminium straight mute with its corks", "Straight-Dämpfer aus Aluminium mit seinen Korken", "aluminium-spun", "cylinder", 0.08),
    "cup-mute": ("Fibre cup mute", "Cup-Dämpfer aus Fasermasse", "fibre-painted", "cylinder", 0.09),
    "accessories": ("Valve-oil bottle", "Ventilölflasche", "pe-bottle", "cylinder", 0.03),
}
LAYER = {"case-shell": (SHELL, 0.15, RHO_CASE, 1500, "wood-handbook-2010"), "case-lining": (0.014, 0.04, RHO_FOAM, 1400, "iso-10456-2007"), "case-hardware": (0.001, 50, RHO_STEEL, 460, "incropera-2018"),
         "trumpet-brass": (WALL_TUBE, 120, RHO_BRASS, 375, "cda-c26000"), "worn-brass": (0.0008, 120, RHO_BRASS, 375, "cda-c26000"), "mouthpiece": (0.004, 120, RHO_BRASS, 375, "cda-c26000"),
         "straight-mute": (0.0007, 205, RHO_ALU, 900, "iso-10456-2007"), "cup-mute": (0.002, 0.15, RHO_FIBRE, 1300, "iso-10456-2007"), "accessories": (0.0015, 0.4, RHO_PE, 1900, "iso-10456-2007")}


def thermal_model(areas):
    """🌡️ The passive thermal model with the breath stream of the player."""
    regions = []
    for rid in REGIONS:
        en, de, material, geometry, lc = REGION_TITLES[rid]
        t, k, rho, cp, ref = LAYER[rid]
        reg = {"id": rid, "title": T(en, de), "kind": "surface", "model": "passive", "material": material, "areaM2": round(areas.get(rid, 1.0), 4), "characteristicLengthM": lc, "geometry": geometry, "skyViewFactor": 0.5,
               "layers": [], "heatCapacityJM2K": round(t * rho * cp), "source": "authored-estimate", "reference": ref}
        if rid in ("case-shell", "trumpet-brass"):
            reg.update({"ratedPowerW": 0, "ratedPowerBasis": "passive"})
        regions.append(reg)
    return {
        "environment": {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
        "passengers": [], "operating": [], "regions": regions,
        "emitters": [{"id": "breath-bell", "kind": "vent", "title": T("Warm moist breath of the player leaving at the bell", "Warmer, feuchter Atem des Spielers, der am Schallstück austritt"), "anchor": "bell-mouth", "directionDeg": 0, "diameterM": 0.118, "volumeFlowM3S": 0.0002, "temperatureRiseK": 7, "vapourKgPerKg": 0.022, "region": "trumpet-brass", "source": "authored-estimate", "reference": "campbell-gilbert-myers-2021"}],
        "assumptions": [
            T("The trumpet, its mouthpiece, mutes and case are acoustic and passive: the trumpet brass and the case shell each state their nameplate as 0 W passive. All regions are inert surfaces in a shop at 21 degC without sun; the bell counts its inside, which the room air and the breath reach, the tubes only their outside, the open case its plush bed and linings.", "Trompete, Mundstück, Dämpfer und Koffer sind akustisch und passiv: Trompetenmessing und Kofferschale nennen ihr Typenschild als 0 W passiv. Alle Regionen sind träge Flächen in einem Laden bei 21 °C ohne Sonne; das Schallstück zählt seine Innenseite, die Raumluft und Atem erreichen, die Rohre nur ihre Außenseite, der offene Koffer sein Plüschbett und seine Auskleidung."),
            T("Played: the breath of the player (about 0.2 l/s at a mezzo-forte, exhaled near 34 degC and saturated, cooled to about 28 degC and still near saturation after 1.48 m of brass) leaves at the bell as a stream 7 K above the room; the left hand covers about 55 % of the worn valve-casing brass at 33 degC, the right hand 6 % of the lacquered brass, the lips 12 % of the mouthpiece at 35 degC. Aged surfaces: yellowed clear lacquer on brass emits about 0.6, the bare tarnished brass under the hands 0.3 instead of 0.05 to 0.1 when polished, the worn silver plating 0.06, scuffed PVC tolex 0.92.", "Gespielt: der Atem des Spielers (etwa 0,2 l/s im Mezzoforte, nahe 34 °C und gesättigt ausgeatmet, nach 1,48 m Messing auf etwa 28 °C abgekühlt und noch nahe der Sättigung) tritt am Schallstück als Strom 7 K über der Raumluft aus; die linke Hand bedeckt etwa 55 % des abgegriffenen Ventilgehäuse-Messings bei 33 °C, die rechte Hand 6 % des lackierten Messings, die Lippen 12 % des Mundstücks bei 35 °C. Gealterte Oberflächen: vergilbter Klarlack auf Messing strahlt etwa 0,6, das blanke angelaufene Messing unter den Händen 0,3 statt 0,05 bis 0,1 poliert, die abgeriebene Versilberung 0,06, verschrammtes PVC-Tolex 0,92."),
        ],
    }


# ---------------------------------------------------------------- volumes
THERMAL_SKIP = {"cloth"}


def thermal_list(plist, s):
    """🌡️ The thermal list: references to the body solids of every region and the breath outlet at the bell mouth."""
    out = [{"id": f"th-{p.id}", "of": p.id, "region": p.region} for p in plist if p.solid3d and p.region and not (p.flags.get("texture") and p.region not in ("worn-brass",))]
    if s > 50:
        if s == 100:
            for i in range(3):
                sleeve = [(-0.050, CASING_R), (-0.050, CASING_R + 0.0006), (-0.008, CASING_R + 0.0006), (-0.008, CASING_R), (-0.050, CASING_R)]
                out.append({"id": f"th-casing-wear-{i + 1}", "kind": "revolve", "bone": f"casing-{i + 1}", "material": "brass-worn", "role": "detail", "tone": 0.5, "profile": [[r6(a), r6(r)] for a, r in sleeve], "axis": [1.0, 0.0, 0.0], "origin": [0.0, 0.0, 0.0], "region": "worn-brass"})
        else:
            ring = [(0.1883, CASING_W - CASING_R - 0.0012), (0.2597, CASING_W - CASING_R - 0.0012), (0.2597, CASING_W + CASING_R + 0.0012), (0.1883, CASING_W + CASING_R + 0.0012)]
            hole = [(0.1895, CASING_W - CASING_R), (0.1895, CASING_W + CASING_R), (0.2585, CASING_W + CASING_R), (0.2585, CASING_W - CASING_R)]
            out.append({"id": "th-valve-block-wear", "kind": "extrude", "bone": "trumpet", "material": "brass-worn", "role": "detail", "tone": 0.5, "profile": [[r6(a), r6(b)] for a, b in ring], "holes": [[[r6(a), r6(b)] for a, b in hole]], "plane": "xy", "from": 0.008, "to": 0.050, "region": "worn-brass"})
    rim = BELL_PROFILE[-1]
    depth = 0.005 if s == 50 else 0.012
    out.append({"id": "th-breath-bell", "kind": "revolve", "bone": "bell", "material": "brass-lacquer", "role": "detail", "tone": 0.2, "region": "trumpet-brass", "tags": ["emitter", "breath-bell"], "profile": [[r6(rim[0] - 0.001 - depth), r6(rim[1] - 0.004)], [r6(rim[0] - 0.001), r6(rim[1] - 0.004)]], "axis": [1.0, 0.0, 0.0], "origin": [0.0, 0.0, 0.0]})
    return out


STRUCT_MEMBERS = {"bottom": "case", "wall-near": "case", "wall-far": "case", "wall-mouthpiece-end": "case", "wall-bell-end": "case", "walls": "case", "bottom-and-bed": "case",
                  "lid-top": "lid", "lid-rim-near": "lid", "lid-rim-far": "lid", "lid-rim-mouthpiece-end": "lid", "lid-rim-bell-end": "lid", "lid-rims": "lid",
                  "bed-floor": "case", "bed-block": "case", "mouthpiece-holder": "case", "divider": "case",
                  "leadpipe": "trumpet", "main-lower": "trumpet", "leadpipe-and-main-slide": "trumpet", "valve-block": "trumpet", "k-main": "trumpet", "k-bow": "trumpet",
                  "bow-and-bell-tail": "bell", "bell": "bell", "bell-bead": "bell"}


def structure_list(plist, s):
    """🏗️ The structure list: the case shell and the lid as box members, the leadpipe, main slide tube, knuckles and bell section as the trumpet's tube frame, joints at the hinge, the handle and the valve block, mass marks at the centres of mass."""
    out = [{"id": f"st-{p.id}", "of": p.id, "member": STRUCT_MEMBERS[p.id], "part": "member"} for p in plist if p.id in STRUCT_MEMBERS and p.solid3d]
    f = {50: 0.0075, 100: 0.015, 200: 0.03}[s]
    jr = max(f, 0.008)
    hinge = cframe(0, HINGE[1], HINGE[2])
    out += [
        {"id": "joint-hinge", "kind": "capsule", "bone": "case", "material": "nickel-steel", "role": "detail", "tone": 0.25, "p0": [0.10, r6(hinge[1]), r6(hinge[2])], "p1": [0.54, r6(hinge[1]), r6(hinge[2])], "r0": r6(jr), "r1": r6(jr), "member": "case", "part": "joint"},
        {"id": "joint-grip", "kind": "capsule", "bone": "case", "material": "handle-pvc", "role": "detail", "tone": 0.25, "p0": [r6(L_CASE / 2), 0.0, 0.0], "p1": [r6(L_CASE / 2), 0.0, 0.026], "r0": r6(max(jr, 0.012)), "r1": r6(max(jr, 0.012)), "member": "case", "part": "joint"},
        {"id": "joint-valves", "kind": "ellipsoid", "bone": "trumpet", "material": "brass-lacquer", "role": "detail", "tone": 0.25, "radii": [r6(max(jr, 0.012)), r6(max(jr, 0.012)), r6(max(jr, 0.012))], "transform": {"translation": [r6(U_VALVE[0]), r6(CASING_W), 0.040]}, "member": "trumpet", "part": "joint"},
    ]
    for b, (m, c) in MASS.items():
        if m < 0.05 or b not in ("case", "lid", "trumpet", "bell", "straight-mute", "cup-mute", "mouthpiece"):
            continue
        rr = f * 0.6
        out.append({"id": f"mass-{b}", "kind": "ellipsoid", "bone": b, "material": "brass-lacquer" if b in ("trumpet", "bell", "mouthpiece") else "case-ply-tolex", "role": "detail", "tone": 0.2, "radii": [r6(rr)] * 3, "transform": {"translation": [r6(c[0]), r6(c[1]), r6(c[2])]}, "member": b if b in ("case", "lid", "trumpet", "bell") else ("trumpet" if b == "mouthpiece" else "case"), "part": "mass"})
    return out


VOLUME_DECISIONS = {
    50: [
        T("Body: the case built panel by panel from 6 mm tolex-covered plywood (declared sheets): bottom, four walls, lid top and four lid rims, the plush-covered foam bed of 14 mm, 3 mm plush linings, the compartment divider, the foam block under the right-hand tubes and the mouthpiece holder; nickel-plated latches, hinges, corners and edge strips, the PVC handle on two posts; torn tolex at the bell end, the scuffed bottom edge, an inventory sticker and a scuff on the lid as texture sheets 0.4 mm proud. The trumpet lies on its right side: receiver, tapered leadpipe, main tuning slide on its own bone, the three valve casings with knurled caps, stems and pearl buttons on the pistons, the 1st, 2nd and 3rd slides with their stockings, eight knuckles and two ports, the bow and bell branch as one loft, the bell as a 0.5 mm closed-loop shell of 123 mm with its rim bead, the finger ring; the bare worn brass of the valve casings and the leadpipe as texture sleeves 0.3 mm proud. The mouthpiece with its cup, its worn rim, the straight mute with three cork strips as one cork ring, the cup mute, the oil bottle and the cloth lie in the compartment.", "Körper: der Koffer Platte für Platte aus 6-mm-Sperrholz mit Tolexbezug (deklarierte Platten): Boden, vier Wände, Deckelplatte und vier Deckelränder, das plüschbezogene Schaumbett von 14 mm, 3-mm-Plüschauskleidung, der Fachtrenner, der Schaumblock unter den rechten Rohren und der Mundstückhalter; vernickelte Schlösser, Scharniere, Ecken und Kantenleisten, der PVC-Griff auf zwei Stützen; eingerissenes Tolex am Schallstückende, die verschrammte Bodenkante, ein Inventaraufkleber und eine Schramme auf dem Deckel als Texturfolien 0,4 mm erhaben. Die Trompete liegt auf ihrer rechten Seite: Mundrohrhülse, konisches Mundrohr, Hauptstimmzug an eigenem Knochen, die drei Ventilgehäuse mit gerändelten Kappen, Stangen und Perlmuttdrücker an den Kolben, der 1., 2. und 3. Zug mit ihren Hülsen, acht Ventilstutzen und zwei Durchgänge, Bogen und Schallrohr als ein Loft, das Schallstück als geschlossene 0,5-mm-Schale von 123 mm mit Randwulst, der Fingerring; das blankgegriffene Messing an Ventilgehäusen und Mundrohr als Texturhülsen 0,3 mm erhaben. Das Mundstück mit Kessel und abgeriebenem Rand, der Straight-Dämpfer mit drei Korkstreifen als ein Korkring, der Cup-Dämpfer, die Ölflasche und das Tuch liegen im Fach."),
        T("Thermal: references to the body solids of nine regions (case shell, lining, hardware, lacquered brass, worn brass, mouthpiece, both mutes, accessories) and the breath outlet as a disc across the bell mouth tagged for the emitter. Structure: the shell panels and lid panels as box members, the leadpipe, the main slide tube, three knuckles, the bow and the bell as the trumpet's tube frame; joints along the hinge, at the grip and at the valve block, mass marks at the centres of mass of case, lid, trumpet, bell, mutes and mouthpiece.", "Thermik: Verweise auf die Körper von neun Regionen (Kofferschale, Auskleidung, Beschläge, lackiertes Messing, abgegriffenes Messing, Mundstück, beide Dämpfer, Zubehör) und der Atemauslass als Scheibe über der Schallstückmündung, für den Emitter markiert. Tragwerk: Schalen- und Deckelplatten als Kastenstäbe, Mundrohr, Hauptzugrohr, drei Stutzen, Bogen und Schallstück als Rohrtragwerk der Trompete; Gelenke entlang des Scharniers, am Griff und am Ventilblock, Massemarken in den Schwerpunkten von Koffer, Deckel, Trompete, Schallstück, Dämpfern und Mundstück."),
    ],
    100: [
        T("Body for the 15 mm feature, drawn at true size where the drawing is (feature rule C13): the base walls are one extrusion with its hollow, bottom and bed one 20 mm block with a plush sheet, the lid rims one extrusion with its hollow and the lid top a sheet; one latch per side, the two hinges one hinge line, the handle one block, corners, edge strips and all texture are left out. The trumpet keeps its tubes, slides, three casings (caps merged into a stepped profile), the pistons as one part each, five of the eight knuckles, the bell with its bead merged into the rim, the mouthpiece as its outer profile; the cork strips stay one ring, the oil bottle three points. The worn brass is a thermal sleeve on each casing.", "Körper für das Merkmal von 15 mm, in wahrer Größe, wo die Zeichnung es ist (Merkmalsregel C13): die Wände des Unterteils sind eine Extrusion mit ihrem Hohlraum, Boden und Bett ein Block von 20 mm mit Plüschblatt, die Deckelränder eine Extrusion mit Hohlraum und die Deckelplatte ein Blatt; ein Schloss je Seite, die zwei Scharniere eine Scharnierlinie, der Griff ein Block, Ecken, Kantenleisten und alle Texturen entfallen. Die Trompete behält Rohre, Züge, drei Gehäuse (Kappen in ein gestuftes Profil verschmolzen), die Kolben als je ein Teil, fünf der acht Stutzen, das Schallstück mit in den Rand verschmolzenem Wulst, das Mundstück als Außenprofil; die Korkstreifen bleiben ein Ring, die Ölflasche drei Punkte. Das abgegriffene Messing ist eine thermische Hülse an jedem Gehäuse."),
        T("Thermal and structure follow the body; the joints and mass marks are 15 mm.", "Thermik und Tragwerk folgen dem Körper; Gelenke und Massemarken messen 15 mm."),
    ],
    200: [
        T("Body for the 30 mm feature: case walls and lid rims as hollow extrusions, bottom, bed and foam block as one 26 mm mass, the lid top with its lining one sheet, one hinge line, the handle one block, the divider and cloth kept at their drawn size. The trumpet becomes leadpipe and main slide as one tube, the three casings with their pistons as one valve block with a button strip on top, the lower slides as one tube loop, the third slide as one loop, the bow with the bell branch and the bell; the mouthpiece three points, the mutes four points each, the oil bottle one cylinder. The worn brass is a thermal band round the valve block.", "Körper für das Merkmal von 30 mm: Kofferwände und Deckelränder als hohle Extrusionen, Boden, Bett und Schaumblock als eine Masse von 26 mm, die Deckelplatte mit Auskleidung ein Blatt, eine Scharnierlinie, der Griff ein Block, Trenner und Tuch in ihrer gezeichneten Größe. Die Trompete wird zu Mundrohr und Hauptzug als ein Rohr, den drei Gehäusen mit Kolben als ein Ventilblock mit Drückerleiste, den unteren Zügen als eine Rohrschleife, dem dritten Zug als eine Schleife, dem Bogen mit Schallrohr und dem Schallstück; das Mundstück drei Punkte, die Dämpfer je vier Punkte, die Ölflasche ein Zylinder. Das abgegriffene Messing ist ein thermisches Band um den Ventilblock."),
        T("Thermal and structure follow the body; joints and mass marks are 30 mm.", "Thermik und Tragwerk folgen dem Körper; Gelenke und Massemarken messen 30 mm."),
    ],
}


def volume(s):
    """🧊️ The authored volume of scale 1:s."""
    plist = parts(s)
    body = [p.solid() for p in plist if p.solid3d]
    return {"status": "authored", "denominator": s, "body": body, "thermal": thermal_list(plist, s), "structure": structure_list(plist, s), "decisions": VOLUME_DECISIONS[s]}


# ---------------------------------------------------------------- representations
PENS = {50: {"body": 0.25, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.35}, 100: {"body": 0.2, "detail": 0.15, "texture": None, "glazing": None, "ground": 0.25}, 200: {"body": 0.2, "detail": 0.15, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}
REP_DECISIONS = {
    50: {
        "silhouette": T("The open case as the dealer shows it: the 0.64 x 0.33 m base on the floor with its lid standing open at 100 degrees behind it, the plush lining facing the viewer; inside, the trumpet lies on its right side with the bell to the far end, the mouthpiece behind the receiver, and the straight mute, cup mute, oil and cloth fill the compartment along the hinge side. Every part is drawn as the solid it is (extrusions and revolves placed with the bones), so the rolled trumpet, the opening lid and the rolled case read right in all three views.", "Der offene Koffer, wie ihn der Händler zeigt: das Unterteil 0,64 x 0,33 m auf dem Boden, dahinter der um 100 Grad offenstehende Deckel mit der Plüschauskleidung zum Betrachter; darin liegt die Trompete auf ihrer rechten Seite, das Schallstück zum hinteren Ende, das Mundstück hinter dem Mundrohr, und Straight-Dämpfer, Cup-Dämpfer, Öl und Tuch füllen das Fach an der Scharnierseite. Jedes Teil ist als der Körper gezeichnet, der es ist (Extrusionen und Rotationskörper mit den Knochen platziert), so dass die gerollte Trompete, der sich öffnende Deckel und der gerollte Koffer in allen drei Ansichten stimmen."),
        "retained": T("Base panels, linings, bed, divider, foam block, mouthpiece holder, latches, hinges, corners, edge strips, handle; lid panels, lid lining and corners; torn tolex, scuffed edge, sticker and lid scuff; the whole trumpet with receiver, leadpipe, slides, stockings, knuckles, ports, casings, caps, stems, buttons, bow, bell, bead and ring; the worn bands; mouthpiece with its worn rim; both mutes with corks, oil bottle with cap, cloth.", "Platten des Unterteils, Auskleidung, Bett, Trenner, Schaumblock, Mundstückhalter, Schlösser, Scharniere, Ecken, Kantenleisten, Griff; Deckelplatten, Deckelauskleidung und Ecken; eingerissenes Tolex, verschrammte Kante, Aufkleber und Deckelschramme; die ganze Trompete mit Hülse, Mundrohr, Zügen, Hülsen, Stutzen, Durchgängen, Gehäusen, Kappen, Stangen, Drückern, Bogen, Schallstück, Wulst und Ring; die abgegriffenen Bänder; Mundstück mit abgeriebenem Rand; beide Dämpfer mit Korken, Ölflasche mit Kappe, Tuch."),
        "merged": T("The three cork strips of the straight mute are one cork ring; the braces, the water key, the finger hook and the thumb saddle (all under 7.5 mm) are left to the tubes they sit on.", "Die drei Korkstreifen des Straight-Dämpfers sind ein Korkring; Stützen, Wasserklappe, Fingerhaken und Daumensattel (alle unter 7,5 mm) bleiben den Rohren überlassen, an denen sie sitzen."),
        "omitted": T("The pistons inside the casings, the valve springs and felts, the bore of the mouthpiece throat and the cavity of the cup mute, which no view sees.", "Die Kolben in den Gehäusen, Ventilfedern und Filze, die Bohrung der Mundstückkehle und der Hohlraum des Cup-Dämpfers, die keine Ansicht sieht."),
        "thinElements": T("The 6 mm shell panels, 3 mm linings and 1 mm edge strips are drawn at their true thickness; stems 4.8 mm, ring wire 2.8 mm and bell bead 4.4 mm at true size; wear and stickers are 0.1 mm texture outlines.", "Die 6-mm-Schalenplatten, 3-mm-Auskleidung und 1-mm-Kantenleisten sind in wahrer Dicke gezeichnet; Stangen 4,8 mm, Ringdraht 2,8 mm und Schallstückwulst 4,4 mm in wahrer Größe; Abrieb und Aufkleber sind Texturumrisse von 0,1 mm."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.25 mm, detail 0.18 mm, texture 0.1 mm, ground 0.35 mm; black tolex dark, the burgundy plush middle, lacquered brass light, nickel silver and pearl lightest.", "Körperstift 0,25 mm, Detail 0,18 mm, Textur 0,1 mm, Boden 0,35 mm; schwarzes Tolex dunkel, der weinrote Plüsch mittel, lackiertes Messing hell, Neusilber und Perlmutt am hellsten."),
        "overlays": T("Nine regions: case shell, lining, hardware, lacquered brass, worn brass, mouthpiece, straight mute, cup mute and accessories; each drawn shape names its region.", "Neun Regionen: Kofferschale, Auskleidung, Beschläge, lackiertes Messing, abgegriffenes Messing, Mundstück, Straight-Dämpfer, Cup-Dämpfer und Zubehör; jede gezeichnete Form nennt ihre Region."),
    },
    100: {
        "silhouette": T("The open case with its lid, the lying trumpet, mouthpiece, mutes and oil, generalised for 1:100.", "Der offene Koffer mit Deckel, die liegende Trompete, Mundstück, Dämpfer und Öl, für 1:100 verallgemeinert."),
        "retained": T("Base walls, bottom with bed, divider, foam block, holder, latches, hinge line, handle; lid rims, top and lining; trumpet tubes, slides, casings, pistons, five knuckles, bow, bell, mouthpiece; mutes with the cork ring, bottle, cloth.", "Wände, Boden mit Bett, Trenner, Schaumblock, Halter, Schlösser, Scharnierlinie, Griff; Deckelränder, Deckelplatte und Auskleidung; Trompetenrohre, Züge, Gehäuse, Kolben, fünf Stutzen, Bogen, Schallstück, Mundstück; Dämpfer mit Korkring, Flasche, Tuch."),
        "merged": T("Panels into hollow extrusions, caps into the casing profiles, stem and button into one piston, the bead into the bell rim.", "Platten zu hohlen Extrusionen, Kappen in die Gehäuseprofile, Stange und Drücker zu einem Kolben, der Wulst in den Schallstückrand."),
        "omitted": T("Corners, edge strips, ports, three knuckles, the finger ring and all texture.", "Ecken, Kantenleisten, Durchgänge, drei Stutzen, der Fingerring und alle Texturen."),
        "thinElements": T("Shell 6 mm, linings and tubes at true size through the 0.2 mm pen.", "Schale 6 mm, Auskleidung und Rohre in wahrer Größe über den 0,2-mm-Stift."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.2 mm, detail 0.15 mm, ground 0.25 mm.", "Körperstift 0,2 mm, Detail 0,15 mm, Boden 0,25 mm."),
        "overlays": T("The same nine regions as at 1:50.", "Dieselben neun Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": T("A box with its lid open and a trumpet lying in it: case walls, lid, the bell, the valve block, the leadpipe loop and the mutes as masses.", "Ein Kasten mit offenem Deckel und einer darin liegenden Trompete: Kofferwände, Deckel, Schallstück, Ventilblock, Mundrohrschleife und Dämpfer als Massen."),
        "retained": T("Walls, bottom mass, divider, hinge line, handle, lid rims and top; leadpipe with main slide, valve block, button strip, lower slides, third slide, bow, bell, mouthpiece, mutes, bottle, cloth.", "Wände, Bodenmasse, Trenner, Scharnierlinie, Griff, Deckelränder und Deckelplatte; Mundrohr mit Hauptzug, Ventilblock, Drückerleiste, untere Züge, dritter Zug, Bogen, Schallstück, Mundstück, Dämpfer, Flasche, Tuch."),
        "merged": T("Casings with pistons into one valve block, the slides into two loops, bed and block into the bottom mass.", "Gehäuse mit Kolben zu einem Ventilblock, die Züge zu zwei Schleifen, Bett und Block in die Bodenmasse."),
        "omitted": T("Latches, knuckles, the holder, corks and all detail and texture.", "Schlösser, Stutzen, der Halter, Korken und alle Details und Texturen."),
        "thinElements": T("Shell and tubes at true size through the 0.2 mm pen.", "Schale und Rohre in wahrer Größe über den 0,2-mm-Stift."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.2 mm, detail 0.15 mm, ground 0.25 mm.", "Körperstift 0,2 mm, Detail 0,15 mm, Boden 0,25 mm."),
        "overlays": T("The same nine regions as at 1:50.", "Dieselben neun Regionen wie bei 1:50."),
    },
}


INTERIOR = {"bottom", "bed-floor", "lining-near", "lining-far", "lining-mouthpiece-end", "lining-bell-end", "divider", "bed-block", "mouthpiece-holder", "cloth", "bottom-and-bed", "bed-lining"}


def paint_group(p, view):
    """🎨️ Paint group of a part in a view: the case shell never overlaps the contents at rest where it is painted over them, so it stays over them when the case is closed or rolled; the bed lies under the contents in plan, the open lid behind them in elevation."""
    if view == "elevation":
        if p.bone == "lid":
            return 0
        if p.bone != "case":
            return 1
        if p.id in INTERIOR - {"bottom", "bottom-and-bed"}:
            return 2
        if p.id in ("bottom", "bottom-and-bed"):
            return 5
        return 4 if p.region == "case-hardware" else 3
    if p.bone == "case":
        return 0 if view == "plan" and p.id in INTERIOR else 2
    if p.bone == "lid":
        return 2
    return 1


NEARNESS = {"elevation": lambda q: -q[2], "front": lambda q: q[0], "plan": lambda q: q[1]}


def representation(s):
    """🗺️ One authored representation: the same solid-placed shapes in elevation, front and plan, each view painted farthest first by the rest-pose nearness of the part's nearest point (the product flips a pair only where a pose reverses it)."""
    plist = [p for p in parts(s) if p.draw]
    fr = frames({})
    geometry = {}
    for view, near in NEARNESS.items():
        def key(k):
            q = [near(world(fr, plist[k].bone, x)) for x in part_points(plist[k])]
            return (paint_group(plist[k], view), sum(q) / len(q) + (0.0005 if plist[k].role == "texture" else 0.0), k)
        ranked = sorted(range(len(plist)), key=key)
        shapes = []
        for rank, k in enumerate(ranked):
            sh = plist[k].shape(2.0 + rank * 0.01)
            if plist[k].role == "texture":
                sh["filled"] = False
            shapes.append(sh)
        geometry[view] = shapes
    lo, hi = RANGES[s]
    return {"status": "authored", "denominator": s, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[s], "hatches": [], "geometry": geometry, "decisions": REP_DECISIONS[s]}


# ---------------------------------------------------------------- rig masses and sections
def finish_rig():
    """🦴️ Masses, sections and lengths into the rig."""
    for b in RIG_BONES:
        if b["id"] in MASS:
            m, c = MASS[b["id"]]
            b["mass"] = {"kg": round(m, 4), "comX": r6(c[0]), "comY": r6(c[1]), "distribution": "point", "source": "authored-estimate", "reference": "brass-catalogues" if b["id"] not in ("case", "lid") else "wood-handbook-2010"}
    by = {b["id"]: b for b in RIG_BONES}
    a_box = L_CASE * 0 + (W_CASE * H_BASE - (W_CASE - 2 * SHELL) * (H_BASE - SHELL))
    i_box = (W_CASE * H_BASE ** 3 - (W_CASE - 2 * SHELL) * (H_BASE - SHELL) ** 3) / 12
    by["case"]["section"] = {"areaM2": r6(a_box), "secondMomentM4": float(f"{i_box:.4e}"), "extremeFibreM": r6(H_BASE / 2), "material": "case-ply-tolex", "title": T("Open box section 330 x 120 mm of 6 mm plywood", "Offener Kastenquerschnitt 330 x 120 mm aus 6-mm-Sperrholz")}
    a_lid = W_CASE * H_LID - (W_CASE - 2 * SHELL) * (H_LID - SHELL)
    i_lid = (W_CASE * H_LID ** 3 - (W_CASE - 2 * SHELL) * (H_LID - SHELL) ** 3) / 12
    by["lid"]["section"] = {"areaM2": r6(a_lid), "secondMomentM4": float(f"{i_lid:.4e}"), "extremeFibreM": r6(H_LID / 2), "material": "case-ply-tolex", "title": T("Tray section 330 x 55 mm of 6 mm plywood", "Schalenquerschnitt 330 x 55 mm aus 6-mm-Sperrholz")}
    ro, ri = 0.0060, 0.0060 - WALL_LEAD
    by["trumpet"]["section"] = {"areaM2": float(f"{2 * math.pi * (ro ** 2 - ri ** 2):.4e}"), "secondMomentM4": float(f"{2 * (math.pi / 4 * (ro ** 4 - ri ** 4) + math.pi * (ro ** 2 - ri ** 2) * 0.025 ** 2):.4e}"), "extremeFibreM": 0.031, "material": "brass-lacquer", "title": T("Leadpipe and main slide tube, 12 mm brass tubes 0.5 mm wall, 50 mm apart", "Mundrohr und Hauptzugrohr, 12-mm-Messingrohre mit 0,5 mm Wand, 50 mm Abstand")}
    rb, rbi = 0.0063, 0.0063 - WALL_TUBE
    by["bell"]["section"] = {"areaM2": float(f"{math.pi * (rb ** 2 - rbi ** 2):.4e}"), "secondMomentM4": float(f"{math.pi / 4 * (rb ** 4 - rbi ** 4):.4e}"), "extremeFibreM": rb, "material": "brass-lacquer", "title": T("Bell branch, 12.6 mm brass tube 0.45 mm wall", "Schallrohr, 12,6-mm-Messingrohr mit 0,45 mm Wand")}


STRUCTURE = {
    "gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [],
    "assumptions": [
        T("Stored and closed, the whole asset rests with the case bottom on the floor (one contact of 0.64 x 0.33 m); carried, the hand round the handle is the only support (pin) and the case hangs rolled so that its centre of mass lies under it; played, the trumpet is held by the left hand round the valve casings and the right thumb under the leadpipe (pins) while the lips press 25 N against the mouthpiece rim along the leadpipe and up to three fingers press 4 N on the valve buttons; the case stays on the floor.", "Gelagert und geschlossen ruht das ganze Asset mit dem Kofferboden auf dem Boden (ein Kontakt von 0,64 x 0,33 m); getragen ist die Hand um den Griff das einzige Auflager (Gelenk), und der Koffer hängt so gerollt, dass sein Schwerpunkt darunter liegt; gespielt hält die linke Hand die Trompete um die Ventilgehäuse und der rechte Daumen unter dem Mundrohr (Gelenke), während die Lippen 25 N entlang des Mundrohrs gegen den Mundstückrand drücken und bis zu drei Finger mit 4 N auf die Drücker drücken; der Koffer bleibt am Boden."),
        T("The solver treats the asset as one rigid body, so in the played pose the split between the floor under the case and the hands follows its equal-stiffness assumption, not two separate bodies. Masses are thin-walled sections times length times density: tubes 0.45 mm and the leadpipe 0.5 mm of brass (8530 kg/m3), the bell 0.5 mm, casings 0.8 mm plus 32 g Monel pistons, nickel-silver caps 12 g; the case 6 mm plywood with tolex (560 kg/m3), foam 90 kg/m3, sheet-metal hardware.", "Der Löser behandelt das Asset als einen starren Körper, daher folgt in der gespielten Pose die Aufteilung zwischen dem Boden unter dem Koffer und den Händen seiner Annahme gleicher Steifigkeit, nicht zwei getrennten Körpern. Massen sind dünnwandige Querschnitte mal Länge mal Dichte: Rohre 0,45 mm und das Mundrohr 0,5 mm Messing (8530 kg/m3), das Schallstück 0,5 mm, Gehäuse 0,8 mm plus 32 g Monel-Kolben, Neusilberkappen 12 g; der Koffer 6-mm-Sperrholz mit Tolex (560 kg/m3), Schaum 90 kg/m3, Blechbeschläge."),
    ],
    "clinical": False,
}


def build(areas):
    """🏗️ Assembles and writes the document."""
    finish_rig()
    reps = {str(s): representation(s) for s in (50, 100, 200)}
    (bx, by_, bz) = bounds(P50, {})
    length = bx[1] - bx[0]
    reps["500"] = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
                   "reason": T(f"At 1:500 the case is only {L_CASE * 2:.2f} mm long and {H_BASE * 2 + H_LID * 2:.2f} mm high on paper, below the 3 mm under which an asset is not drawn; the trumpet inside is 1.2 mm long and its tubes 0.025 mm.", f"Bei 1:500 ist der Koffer auf dem Papier nur {L_CASE * 2:.2f} mm lang und {H_BASE * 2 + H_LID * 2:.2f} mm hoch, unter den 3 mm, unter denen ein Asset nicht gezeichnet wird; die Trompete darin ist 1,2 mm lang, ihre Rohre 0,025 mm."),
                   "limitingFeature": T("The 0.64 m length of the case", "Die Länge des Koffers von 0,64 m"), "limitingSizeM": round(length, 3), "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}
    vols = {str(s): volume(s) for s in (50, 100, 200)}
    vols["500"] = {"status": "unavailable", "denominator": 500}
    asset = {
        "id": "used-trumpet",
        "category": "objects",
        "title": T("Used Trumpet with Case", "Gebrauchte Trompete mit Koffer"),
        "summary": T(
            f"A second-hand B flat student trumpet as a used-instrument dealer or a music school depot offers it, cleaned, oiled and ready to play again: 0.48 m from the receiver to the 123 mm yellow-brass bell, ML bore, three Perinet piston valves with nickel-silver caps and pearl buttons, the clear lacquer yellowed and worn through to tarnished brass on the valve casings and the leadpipe where the hands held it; a silver-plated 7C mouthpiece with its rim worn to brass, an aluminium straight mute and a fibre cup mute, a half-full bottle of valve oil and a polishing cloth, all in a 0.64 x 0.33 x 0.175 m plywood case with black tolex, torn at one corner and scuffed along the bottom edge, with a school inventory sticker on the lid; {M_TOTAL:.2f} kg together, the trumpet {M_TRUMPET:.2f} kg. Poses: stored open, closed, carried by the handle, played standing with the bell 6 degrees down.",
            f"Eine gebrauchte B-Schülertrompete, wie sie ein Gebrauchtinstrumentenhandel oder das Lager einer Musikschule anbietet, gereinigt, geölt und wieder spielbereit: 0,48 m vom Mundrohr zum 123-mm-Schallstück aus Gelbmessing, ML-Bohrung, drei Perinet-Pumpventile mit Neusilberkappen und Perlmuttdrückern, der Klarlack vergilbt und an Ventilgehäusen und Mundrohr, wo die Hände hielten, bis aufs angelaufene Messing durchgegriffen; ein versilbertes 7C-Mundstück mit bis aufs Messing abgeriebenem Rand, ein Straight-Dämpfer aus Aluminium und ein Cup-Dämpfer aus Fasermasse, eine halbvolle Flasche Ventilöl und ein Poliertuch, alles in einem Sperrholzkoffer 0,64 x 0,33 x 0,175 m mit schwarzem Tolex, an einer Ecke eingerissen und an der Bodenkante verschrammt, mit einem Inventaraufkleber der Schule auf dem Deckel; zusammen {M_TOTAL:.2f} kg, die Trompete {M_TRUMPET:.2f} kg. Posen: offen gelagert, geschlossen, am Griff getragen, im Stehen gespielt mit 6 Grad gesenktem Schallstück."),
        "tags": ["used", "instrument", "music", "second-hand", "brass", "trumpet", "b-flat-trumpet", "piston-valves", "aerophone", "lip-reed", "mouthpiece", "mute", "straight-mute", "cup-mute", "valve-oil", "instrument-case", "hard-case", "tolex", "plywood", "lacquered-brass", "nickel-silver", "silver-plated", "aluminium", "student-instrument", "passive"],
        "provenance": {"author": "Semio staffage instrument authors (used instruments wave, author 21)", "method": "handcrafted", "created": "2026-11-22", "references": REFS,
                       "notes": T(
                           f"Trumpet (Hornbostel-Sachs 423.233): B flat, three Perinet valves, about 1.48 m of tubing; 0.478 m from the receiver end to the bell rim, bell 123 mm on its own axis 40 mm to the left of the leadpipe, valve casings 20.5 mm at 24 mm pitch, buttons 16 mm with a 17 mm stroke, leadpipe 10 to 12 mm, tubes 11 to 12.6 mm with 0.45 mm walls, bell wall 0.5 mm; {M_TRUMPET:.3f} kg from thin-walled sections. Mouthpiece 7C, 88 mm, rim 26.6 mm, {MASS['mouthpiece'][0] * 1000:.0f} g. Straight mute 135 mm, 98 mm base, {MASS['straight-mute'][0] * 1000:.0f} g; cup mute 130 mm, cup 105 mm, {MASS['cup-mute'][0] * 1000:.0f} g. Case 640 x 330 x 175 mm of 6 mm tolex-covered plywood with a 14 mm plush bed, {M_CASE:.2f} kg. Traces of use: the lacquer worn through on the casings and the leadpipe, yellowed lacquer, worn silver at the mouthpiece rim, a dented and scratched straight mute, chipped cup-mute paint, torn tolex, scuffed bottom edge, dull rust-spotted hardware, a handle worn shiny, flattened plush and an inventory sticker. Nothing that stops it from being played.",
                           f"Trompete (Hornbostel-Sachs 423.233): in B, drei Perinet-Ventile, etwa 1,48 m Rohrlänge; 0,478 m vom Mundrohrende zum Schallstückrand, Schallstück 123 mm auf eigener Achse 40 mm links des Mundrohrs, Ventilgehäuse 20,5 mm bei 24 mm Teilung, Drücker 16 mm mit 17 mm Hub, Mundrohr 10 bis 12 mm, Rohre 11 bis 12,6 mm mit 0,45 mm Wand, Schallstückwand 0,5 mm; {M_TRUMPET:.3f} kg aus dünnwandigen Querschnitten. Mundstück 7C, 88 mm, Rand 26,6 mm, {MASS['mouthpiece'][0] * 1000:.0f} g. Straight-Dämpfer 135 mm, Boden 98 mm, {MASS['straight-mute'][0] * 1000:.0f} g; Cup-Dämpfer 130 mm, Becher 105 mm, {MASS['cup-mute'][0] * 1000:.0f} g. Koffer 640 x 330 x 175 mm aus 6-mm-Sperrholz mit Tolexbezug und 14-mm-Plüschbett, {M_CASE:.2f} kg. Gebrauchsspuren: der Lack an Gehäusen und Mundrohr durchgegriffen, vergilbter Lack, abgeriebenes Silber am Mundstückrand, ein verbeulter und verkratzter Straight-Dämpfer, abgeplatzter Lack am Cup-Dämpfer, eingerissenes Tolex, verschrammte Bodenkante, stumpfe, rostfleckige Beschläge, ein blankgegriffener Griff, plattgedrückter Plüsch und ein Inventaraufkleber. Nichts, was das Spielen verhindert.")},
        "dimensions": {"heightM": round(by_[1], 3), "widthM": round(bx[1] - bx[0], 3), "depthM": round(bz[1] - bz[0], 3), "massKg": round(M_TOTAL, 3)},
        "materials": MATERIALS,
        "rig": {"kind": "mechanical", "root": "floor", "bones": RIG_BONES},
        "anchors": ANCHORS,
        "representations": reps,
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": thermal_model(areas),
        "structure": STRUCTURE,
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "valves", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"[DEBUG] masses { {b: round(m, 4) for b, (m, _) in MASS.items()} }")
    print(f"[DEBUG] total {M_TOTAL:.3f} trumpet {M_TRUMPET:.3f} case {M_CASE:.3f}; bounds x {bx} y {by_} z {bz}; carry twist {CARRY_TWIST} com {CCOM}")
    print(f"[DEBUG] played bounds {bounds(P50, PLAYED)}; carried bounds {bounds(P50, {**CARRIED, '__root': {'y': CARRY_Y}})}")
    print(f"[DEBUG] tubing length {sum(path_length(p) for p in [LEAD, MAIN_LOWER, MAIN_CROOK, BOW]) + 0.034 + (BELL_PROFILE[-1][0] - BELL_START):.3f} m open; slides {path_length(SLIDE_1):.3f} {path_length(SLIDE_2):.3f} {path_length(SLIDE_3):.3f}")


if __name__ == "__main__":
    build(json.loads(Path(sys.argv[1]).read_text()) if len(sys.argv) > 1 else {})
