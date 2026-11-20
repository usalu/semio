"""🔩 Round 3 playground swing 1:500 (feature 75 mm): slender legs and ties on 75 mm foot blocks, the beam, slender chains, the seat generalised to a 75 mm bar."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("swing_coarse", os.path.join(T, "r3-playground-swing-coarse.py"))
c = importlib.util.module_from_spec(_s)
_s.loader.exec_module(c)
k = c.k
ST, RB, AL, UP, WX, WZ = c.ST, c.RB, c.AL, c.UP, c.WX, c.WZ
FOOT = 0.075


def body():
    out = []
    for name, bone, xf in k.LEGS:
        for side, sg in k.SIDES:
            w = sg * k.ZA
            leg = k.leg_tube("leg-%s-%s" % (name, side), bone, w, k.R_LEG, 0.03, ST, "body", 0.45, FOOT * 0.75)
            leg["slender"] = True
            out.append(leg)
            out.append(c.foot_block("foot-%s-%s" % (name, side), name, bone, xf, w, FOOT))
    for side, sg in k.SIDES:
        w = sg * k.ZA
        out.append(k.revolve("brace-%s" % side, "brace", ST, "body", 0.5, [[0.0, 0.0], [0.0, k.R_BRACE], [k.BRACE_LEN, k.R_BRACE], [k.BRACE_LEN, 0.0]], WX, [0.0, 0.0, w], slender=True))
    out.append(k.revolve("beam", "beam", ST, "body", 0.45, [[0.0, 0.0], [0.0, k.R_BEAM], [2 * k.BEAM_HALF, k.R_BEAM], [2 * k.BEAM_HALF, 0.0]], WZ, [0.0, 0.0, -k.BEAM_HALF]))
    for name, bone, zc in k.CHAINS:
        out.append(k.capsule("chain-%s" % name, bone, ST, "body", 0.4, [-0.066, 0.0, zc], [1.925, 0.0, zc], 0.009, slender=True))
    out.append(k.box("seat", "seat", RB, "body", 0.2, (k.SEAT[0], 0.075, k.SEAT[2]), (0.0, k.SEAT_TOP_V - 0.0375, 0.0), radius=0.02))
    return out


def structure(com):
    out = []
    u0 = FOOT * 0.75 / math.sin(c.A)
    for name, bone, xf in k.LEGS:
        for side, sg in k.SIDES:
            w = sg * k.ZA
            m = k.capsule("member-leg-%s-%s" % (name, side), bone, ST, "body", 0.45, [u0, 0.0, w], [k.LEG_LEN, 0.0, w], 0.025, slender=True)
            m.update({"member": bone, "part": "member"})
            out.append(m)
            out.append(c.glyph("mass-leg-%s-%s" % (name, side), bone, ST, (0.12, 0.04, 0.04), (com[bone][1], 0.0, w), bone, "mass"))
    for side, sg in k.SIDES:
        w = sg * k.ZA
        m = k.capsule("member-brace-%s" % side, "brace", ST, "body", 0.5, [0.0, 0.0, w], [k.BRACE_LEN, 0.0, w], 0.018, slender=True)
        m.update({"member": "brace", "part": "member"})
        out.append(m)
        out.append(c.glyph("joint-apex-%s" % side, "beam", ST, (0.08, 0.08, 0.08), (0.0, 0.0, w), "beam", "joint"))
    m = k.capsule("member-beam", "beam", ST, "body", 0.45, [0.0, 0.0, -k.BEAM_TUBE_HALF + 0.045], [0.0, 0.0, k.BEAM_TUBE_HALF - 0.045], 0.045)
    m.update({"member": "beam", "part": "member"})
    out.append(m)
    out.append(c.glyph("mass-beam", "beam", ST, (0.04, 0.04, 0.2), (0.0, com["beam"][2], 0.0), "beam", "mass"))
    for name, bone, zc in k.CHAINS:
        m = k.capsule("member-chain-%s" % name, bone, ST, "body", 0.5, [0.0, 0.0, zc], [k.CHAIN_LEN, 0.0, zc], 0.008, slender=True)
        m.update({"member": bone, "part": "member"})
        out.append(m)
        out.append(c.glyph("joint-pivot-%s" % name, bone, ST, (0.04, 0.04, 0.04), (0.0, 0.0, zc), bone, "joint"))
    core = k.box("member-seat-core", "seat", AL, "body", 0.6, (0.16, 0.075, 0.40), (0.0, k.SEAT_TOP_V - 0.0375, 0.0))
    core.update({"member": "seat", "part": "member"})
    out.append(core)
    out += c.supports(0.08, 0.08, 0.04)
    return out


DECISIONS = [
    {"en": "Body at 1:500 (feature 75 mm), authored anew: legs and ties are slender rods at their true 60.3 and 42.4 mm (drawn as 0.15 mm lines, C24), each foot a 75 mm block, the beam a o101.6 cylinder, chains slender 18 mm rods, the seat generalised to a 430 x 175 x 75 mm bar (its 35 mm are below the feature). Gussets, hangers and all hardware are omitted.",
     "de": "Körper 1:500 (Merkmal 75 mm), neu erstellt: Beine und Zugrohre sind schlanke Stäbe in ihren wahren 60,3 und 42,4 mm (als 0,15-mm-Linien gezeichnet, C24), jeder Fuß ein 75-mm-Block, der Holm ein Zylinder o101,6, Ketten schlanke 18-mm-Stäbe, der Sitz zu einem Balken 430 x 175 x 75 mm verallgemeinert (seine 35 mm liegen unter dem Merkmal). Knotenbleche, Gelenke und alle Beschläge entfallen."},
    {"en": "Thermal: every body solid by reference in its region; single-layer regions, no emitters, no hidden regions.",
     "de": "Thermik: jeder Körper als Referenz in seiner Region; einschichtige Regionen, keine Emitter, keine verdeckten Regionen."},
    {"en": "Structure: legs and ties as slender rods of 50 and 36 mm (inside their body rods), the beam a 90 mm rod, chains slender 16 mm rods, the seat core a 75 mm bar inside the seat; joints only at the apexes and the bearing pins; wedge and roller supports; masses at the legs, the beam and the seat (chain and tie masses merge into their members).",
     "de": "Tragwerk: Beine und Zugrohre als schlanke Stäbe von 50 und 36 mm (in ihren Körperstäben), der Holm ein 90-mm-Stab, Ketten schlanke 16-mm-Stäbe, der Sitzkern ein 75-mm-Balken im Sitz; Gelenke nur an den Scheiteln und Lagerbolzen; Keil- und Rollenlager; Massen an Beinen, Holm und Sitz (Ketten- und Zugrohrmassen gehen in ihren Stäben auf)."},
]


def volume(com):
    b = body()
    s = structure(com)
    s.append(c.glyph("mass-seat", "seat", RB, (0.06, 0.04, 0.1), (0.0, k.SEAT_TOP_V - 0.0375, 0.0), "seat", "mass"))
    return {"status": "authored", "denominator": 500, "body": b, "thermal": c.thermal(b), "structure": s, "decisions": DECISIONS}
