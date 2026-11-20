"""🔩 Round 3 playground swing 1:200 (feature 30 mm): legs on 40 mm foot blocks, ties, a plain beam, 30 mm gussets, hanger blocks, chains as slender rods, the seat as a rounded slab."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("swing_coarse", os.path.join(T, "r3-playground-swing-coarse.py"))
c = importlib.util.module_from_spec(_s)
_s.loader.exec_module(c)
k = c.k
ST, RB, AL, UP, WX, WZ = c.ST, c.RB, c.AL, c.UP, c.WX, c.WZ
FOOT = 0.04


def body():
    out = []
    for name, bone, xf in k.LEGS:
        for side, sg in k.SIDES:
            w = sg * k.ZA
            out.append(k.leg_tube("leg-%s-%s" % (name, side), bone, w, k.R_LEG, 0.03, ST, "body", 0.45, FOOT * 0.75))
            out.append(c.foot_block("foot-%s-%s" % (name, side), name, bone, xf, w, FOOT))
    for side, sg in k.SIDES:
        w = sg * k.ZA
        out.append(k.revolve("brace-%s" % side, "brace", ST, "body", 0.5, [[0.0, 0.0], [0.0, k.R_BRACE], [k.BRACE_LEN, k.R_BRACE], [k.BRACE_LEN, 0.0]], WX, [0.0, 0.0, w]))
        out.append(k.prism("gusset-%s" % side, "beam", ST, "body", 0.42, k.gusset_world(), [0.0, 0.0, w], WX, UP, -0.015, 0.015))
    out.append(k.revolve("beam", "beam", ST, "body", 0.45, [[0.0, 0.0], [0.0, k.R_BEAM], [2 * k.BEAM_HALF, k.R_BEAM], [2 * k.BEAM_HALF, 0.0]], WZ, [0.0, 0.0, -k.BEAM_HALF]))
    for name, bone, zc in k.CHAINS:
        out.append(k.box("hanger-%s" % name, "beam", ST, "detail", 0.35, (0.05, 0.1685, 0.05), (0.0, -0.02575, zc)))
        out.append(k.capsule("chain-%s" % name, bone, ST, "body", 0.4, [0.009, 0.0, zc], [1.925, 0.0, zc], 0.009, slender=True))
    out.append(k.box("seat", "seat", RB, "body", 0.2, k.SEAT, (0.0, k.SEAT_TOP_V - k.SEAT[1] / 2, 0.0), radius=0.012))
    return out


def structure(com):
    out = []
    u0, u1 = FOOT * 0.75 / math.sin(c.A), k.LEG_LEN - 0.03
    for name, bone, xf in k.LEGS:
        for side, sg in k.SIDES:
            w = sg * k.ZA
            m = k.capsule("member-leg-%s-%s" % (name, side), bone, ST, "body", 0.45, [u0, 0.0, w], [u1, 0.0, w], k.R_LEG)
            m.update({"member": bone, "part": "member"})
            out.append(m)
            out.append(c.glyph("mass-leg-%s-%s" % (name, side), bone, ST, (0.09, 0.038, 0.038), (com[bone][1], 0.0, w), bone, "mass"))
    for side, sg in k.SIDES:
        w = sg * k.ZA
        m = k.capsule("member-brace-%s" % side, "brace", ST, "body", 0.5, [0.0, 0.0, w], [k.BRACE_LEN, 0.0, w], k.R_BRACE)
        m.update({"member": "brace", "part": "member"})
        out.append(m)
        out.append(c.glyph("mass-brace-%s" % side, "brace", ST, (0.07, 0.03, 0.03), (com["brace"][1], 0.0, w), "brace", "mass"))
        out.append(c.glyph("joint-apex-%s" % side, "beam", ST, (0.075, 0.075, 0.075), (0.0, 0.0, w), "beam", "joint"))
    m = k.capsule("member-beam", "beam", ST, "body", 0.45, [0.0, 0.0, -k.BEAM_TUBE_HALF + k.R_BEAM], [0.0, 0.0, k.BEAM_TUBE_HALF - k.R_BEAM], k.R_BEAM)
    m.update({"member": "beam", "part": "member"})
    out.append(m)
    out.append(c.glyph("mass-beam", "beam", ST, (0.03, 0.03, 0.15), (0.0, com["beam"][2], 0.0), "beam", "mass"))
    for name, bone, zc in k.CHAINS:
        m = k.capsule("member-chain-%s" % name, bone, ST, "body", 0.5, [0.0, 0.0, zc], [k.CHAIN_LEN, 0.0, zc], 0.008, slender=True)
        m.update({"member": bone, "part": "member"})
        out.append(m)
        out.append(c.glyph("joint-pivot-%s" % name, bone, ST, (0.03, 0.03, 0.03), (0.0, 0.0, zc), bone, "joint"))
    core = k.box("member-seat-core", "seat", AL, "body", 0.6, (0.16, 0.03, 0.40), (0.0, k.SEAT_TOP_V - k.SEAT[1] / 2, 0.0))
    core.update({"member": "seat", "part": "member"})
    out.append(core)
    out.append(c.glyph("mass-seat", "seat", RB, (0.05, 0.016, 0.08), (0.0, com["seat"][2], 0.0), "seat", "mass"))
    out += c.supports(0.06, 0.06, 0.03)
    return out


DECISIONS = [
    {"en": "Body at 1:200 (feature 30 mm), authored anew: each foot (plate and gussets) is one 40 mm block tapering into the leg, as drawn; o60.3 legs as lofts into the beam; o42.4 ties; the beam a plain o101.6 cylinder (the 5 mm domes are below the feature); 30 mm apex gussets; each hanger one block of the drawn front outline; chains as slender 18 mm rods drawn as 0.13 mm lines; the seat a rounded 430 x 175 x 35 mm slab.",
     "de": "Körper 1:200 (Merkmal 30 mm), neu erstellt: jeder Fuß (Platte und Rippen) ist ein 40-mm-Block, der in das Bein übergeht, wie gezeichnet; Beine o60,3 als Lofts in den Holm; Zugrohre o42,4; der Holm ein glatter Zylinder o101,6 (die 5-mm-Wölbungen liegen unter dem Merkmal); Knotenbleche 30 mm; jedes Gelenk ein Block des gezeichneten Vorderansichtsumrisses; Ketten als schlanke 18-mm-Stäbe, als 0,13-mm-Linien gezeichnet; der Sitz eine gerundete Platte 430 x 175 x 35 mm."},
    {"en": "Thermal: every body solid by reference in its region; single-layer regions, no emitters, no hidden regions.",
     "de": "Thermik: jeder Körper als Referenz in seiner Region; einschichtige Regionen, keine Emitter, keine verdeckten Regionen."},
    {"en": "Structure: legs, ties and beam as solid rods of their tube diameters (the 3 mm walls are below the feature), chains as slender 16 mm rods, the seat core 30 mm; joints at the apex sleeves and the bearing pins (the tee clamps and eye bolts merge into their members at this scale); wedge and roller supports; masses at the member centres.",
     "de": "Tragwerk: Beine, Zugrohre und Holm als volle Stäbe ihrer Rohrdurchmesser (die 3-mm-Wände liegen unter dem Merkmal), Ketten als schlanke 16-mm-Stäbe, der Sitzkern 30 mm; Gelenke an den Scheitelhülsen und Lagerbolzen (T-Klemmen und Ringschrauben gehen in diesem Maßstab in ihren Stäben auf); Keil- und Rollenlager; Massen in den Stabschwerpunkten."},
]


def volume(com):
    b = body()
    return {"status": "authored", "denominator": 200, "body": b, "thermal": c.thermal(b), "structure": structure(com), "decisions": DECISIONS}
