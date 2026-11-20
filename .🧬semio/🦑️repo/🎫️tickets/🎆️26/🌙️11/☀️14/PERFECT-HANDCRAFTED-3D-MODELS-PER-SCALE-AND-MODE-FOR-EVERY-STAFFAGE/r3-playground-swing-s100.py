"""🔧 Round 3 playground swing 1:100 (feature 15 mm): legs into the beam (sockets and sleeves merged), 15 mm base plates with 15 mm gussets, ties butting into the legs, the beam with domed ends, gussets, one block per swing hanger, chains as 18 mm slender rods, the seat as a five-section loft."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("swing_coarse", os.path.join(T, "r3-playground-swing-coarse.py"))
c = importlib.util.module_from_spec(_s)
_s.loader.exec_module(c)
k = c.k
ST, RB, AL, UP, WX, WZ = c.ST, c.RB, c.AL, c.UP, c.WX, c.WZ


def body():
    out = []
    for name, bone, xf in k.LEGS:
        for side, sg in k.SIDES:
            w = sg * k.ZA
            out.append(k.leg_tube("leg-%s-%s" % (name, side), bone, w, k.R_LEG, 0.03, ST, "body", 0.45, 0.015))
            out.append(c.plate_box("plate-%s-%s" % (name, side), bone, xf, w, 0.015))
            outer, inner = k.ribs_world(name)
            for rid, tri in (("out", outer), ("in", inner)):
                out.append(k.prism("rib-%s-%s-%s" % (rid, name, side), bone, ST, "detail", 0.35, [(x, max(y, 0.015)) for x, y in tri], k.local(bone, (0.0, 0.0, w)), k.ldir(bone, WX), k.ldir(bone, UP), -0.0075, 0.0075))
    for side, sg in k.SIDES:
        w = sg * k.ZA
        out.append(k.revolve("brace-%s" % side, "brace", ST, "body", 0.5, [[0.0, 0.0], [0.0, k.R_BRACE], [k.BRACE_LEN, k.R_BRACE], [k.BRACE_LEN, 0.0]], WX, [0.0, 0.0, w]))
        out.append(k.prism("gusset-%s" % side, "beam", ST, "body", 0.42, k.gusset_world(), [0.0, 0.0, w], WX, UP, -0.0075, 0.0075))
    out.append(k.revolve("beam", "beam", ST, "body", 0.45, [[0.0, 0.0], [0.0, 0.03], [0.002, 0.042], [0.005, k.R_BEAM], [1.895, k.R_BEAM], [1.898, 0.042], [1.9, 0.03], [1.9, 0.0]], WZ, [0.0, 0.0, -k.BEAM_HALF]))
    for name, bone, zc in k.CHAINS:
        out.append(k.box("hanger-%s" % name, "beam", ST, "detail", 0.35, (0.05, 0.07, 0.042), (0.0, -0.075, zc), radius=0.005))
        out.append(k.capsule("chain-%s" % name, bone, ST, "body", 0.4, [0.009, 0.0, zc], [1.925, 0.0, zc], 0.009, slender=True))
    cv = k.SEAT_TOP_V - k.SEAT[1] / 2
    rows = [(-0.215, 0.075, 0.0135), (-0.205, 0.0875, 0.0175), (0.0, 0.0875, 0.0175), (0.205, 0.0875, 0.0175), (0.215, 0.075, 0.0135)]
    out.append(k.loft("seat", "seat", RB, "body", 0.2, [k.section([0.0, cv, w], rx, ry, exponent=4) for w, rx, ry in rows]))
    return out


def structure(com):
    out = []
    u0, u1 = 0.015 / math.sin(c.A), k.LEG_LEN - 0.03
    for name, bone, xf in k.LEGS:
        for side, sg in k.SIDES:
            w = sg * k.ZA
            out.append(c.tube_member("member-leg-%s-%s" % (name, side), bone, 0.0603, 0.0545, u0, u1, w))
            out.append(c.glyph("mass-leg-%s-%s" % (name, side), bone, ST, (0.07, 0.036, 0.036), (com[bone][1], 0.0, w), bone, "mass"))
    for side, sg in k.SIDES:
        w = sg * k.ZA
        out.append(c.tube_member("member-brace-%s" % side, "brace", 0.0424, 0.0372, 0.0, k.BRACE_LEN, w))
        for j, u in enumerate((0.0, k.BRACE_LEN)):
            out.append(c.glyph("joint-clamp-%s-%d" % (side, j + 1), "brace", ST, (0.04, 0.04, 0.04), (u, 0.0, w), "brace", "joint"))
        out.append(c.glyph("mass-brace-%s" % side, "brace", ST, (0.06, 0.026, 0.026), (com["brace"][1], 0.0, w), "brace", "mass"))
        out.append(c.glyph("joint-apex-%s" % side, "beam", ST, (0.065, 0.065, 0.065), (0.0, 0.0, w), "beam", "joint"))
    out.append(c.beam_member("member-beam"))
    out.append(c.glyph("mass-beam", "beam", ST, (0.025, 0.026, 0.12), (0.0, com["beam"][2], 0.0), "beam", "mass"))
    for name, bone, zc in k.CHAINS:
        m = k.capsule("member-chain-%s" % name, bone, ST, "body", 0.5, [0.0, 0.0, zc], [k.CHAIN_LEN, 0.0, zc], 0.0075, slender=True)
        m.update({"member": bone, "part": "member"})
        out.append(m)
        out.append(c.glyph("joint-pivot-%s" % name, bone, ST, (0.02, 0.02, 0.02), (0.0, 0.0, zc), bone, "joint"))
        out.append(c.glyph("joint-hitch-%s" % name, "seat", ST, (0.02, 0.02, 0.02), (0.0, 0.0, zc), "seat", "joint"))
        out.append(c.glyph("mass-chain-%s" % name, bone, ST, (0.04, 0.016, 0.016), (com[bone][1], 0.0, zc), bone, "mass"))
    core = k.box("member-seat-core", "seat", AL, "body", 0.6, (0.16, 0.016, 0.40), (0.0, k.SEAT_TOP_V - k.SEAT[1] / 2, 0.0))
    core.update({"member": "seat", "part": "member"})
    out.append(core)
    out.append(c.glyph("mass-seat", "seat", RB, (0.045, 0.012, 0.07), (0.0, com["seat"][2], 0.0), "seat", "mass"))
    out += c.supports(0.05, 0.045, 0.02)
    return out


DECISIONS = [
    {"en": "Body at 1:100 (feature 15 mm), authored anew: the legs run as o60.3 lofts from their mitred feet into the beam (sockets and sleeves, 8 and 6 mm wider than leg and beam, merge into them); 15 mm base plates and 15 mm gussets (the 12 and 8 mm parts at the feature); the ties as o42.4 revolves butting into the legs (tee clamps below the feature); the beam with its domed ends; the apex gussets 15 mm thick; each swing hanger one rounded block of fork, band, bolt and pin; each chain one 18 mm slender rod (the links, 8 mm wire, are below the feature; 18 mm is the mean width of the alternating 27 and 8 mm links); the seat a loft of five rounded sections. Nuts, bolts, hooks and eye bolts are omitted.",
     "de": "Körper 1:100 (Merkmal 15 mm), neu erstellt: die Beine laufen als Lofts o60,3 von ihren Gehrungsfüßen in den Holm (Muffen und Hülsen, 8 und 6 mm breiter als Bein und Holm, gehen in ihnen auf); Fußplatten 15 mm und Rippen 15 mm (die 12- und 8-mm-Teile auf dem Merkmal); die Zugrohre als Rotation o42,4 stumpf an den Beinen (T-Klemmen unter dem Merkmal); der Holm mit gewölbten Enden; die Knotenbleche 15 mm dick; jedes Schaukelgelenk ein gerundeter Block aus Gabel, Band, Schraube und Bolzen; jede Kette ein schlanker 18-mm-Stab (die Glieder mit 8 mm Draht liegen unter dem Merkmal; 18 mm ist die mittlere Breite der abwechselnd 27 und 8 mm breiten Glieder); der Sitz eine Loft aus fünf gerundeten Schnitten. Muttern, Schrauben, Haken und Ringschrauben entfallen."},
    {"en": "Thermal: every body solid by reference in its region (frame-galv, chain-steel, seat-rubber); no layers to nest (single-layer regions), no emitters, no hidden regions.",
     "de": "Thermik: jeder Körper als Referenz in seiner Region (frame-galv, chain-steel, seat-rubber); keine Schichten zu schachteln (einschichtige Regionen), keine Emitter, keine verdeckten Regionen."},
    {"en": "Structure: hollow o60.3 x 2.9 legs and o42.4 x 2.6 ties on their axes, the o101.6 x 3.6 beam, chains as 15 mm slender rods (the generalised two-wire section), the seat core 16 mm (feature); joints as spheres at the apex sleeves, tee clamps, bearing pins and eye bolts; supports as wedge and roller under the plates; masses at the centres of legs, ties, beam, chains and seat.",
     "de": "Tragwerk: hohle Beine o60,3 x 2,9 und Zugrohre o42,4 x 2,6 auf ihren Achsen, der Holm o101,6 x 3,6, Ketten als schlanke 15-mm-Stäbe (verallgemeinerter Zwei-Draht-Querschnitt), der Sitzkern 16 mm (Merkmal); Gelenke als Kugeln an Scheitelhülsen, T-Klemmen, Lagerbolzen und Ringschrauben; Auflager als Keil und Rolle unter den Platten; Massen in den Schwerpunkten von Beinen, Zugrohren, Holm, Ketten und Sitz."},
]


def volume(com):
    b = body()
    return {"status": "authored", "denominator": 100, "body": b, "thermal": c.thermal(b), "structure": structure(com), "decisions": DECISIONS}
