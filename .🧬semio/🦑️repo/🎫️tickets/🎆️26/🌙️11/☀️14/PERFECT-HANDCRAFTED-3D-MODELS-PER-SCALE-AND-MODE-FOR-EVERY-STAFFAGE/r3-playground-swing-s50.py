"""🔩 Round 3 playground swing 1:50 (feature 7.5 mm): every part of the real swing stated by hand: legs with mitred feet, base plates with gussets and anchor nuts, tie tubes in tee clamps, the beam with domed ends, welded apex brackets (sleeve, sockets, gusset, bolts), swing hangers (band, fork, clamp bolt, bearing pin), forged top eyes, 2 x 76 DIN 766 links, eye bolts with acorn nuts and the moulded rubber seat."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("swing_kit", os.path.join(T, "r3-playground-swing-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

ST, RB, AL = "steel-galvanised", "rubber", "aluminium"
FG, CS, SR = "frame-galv", "chain-steel", "seat-rubber"
UP, WX, WZ = (0, 1, 0), (1, 0, 0), (0, 0, 1)


def legs():
    out = []
    for name, bone, xf in k.LEGS:
        rot_world = k.quat_from_axes(k.ldir(bone, WX), k.ldir(bone, UP), [0, 0, 1])
        for side, sg in k.SIDES:
            w = sg * k.ZA
            out.append(k.leg_tube("leg-%s-%s" % (name, side), bone, w, k.R_LEG, k.LEG_TOP_S, ST, "body", 0.45, k.PLATE[1]))
            out.append(k.box("plate-%s-%s" % (name, side), bone, ST, "body", 0.3, k.PLATE, k.local(bone, (xf, k.PLATE[1] / 2, w)), rotation=rot_world, radius=0.002))
            outer, inner = k.ribs_world(name)
            origin = k.local(bone, (0.0, 0.0, w))
            for rid, tri in (("out", outer), ("in", inner)):
                out.append(k.prism("rib-%s-%s-%s" % (rid, name, side), bone, ST, "detail", 0.35, tri, origin, k.ldir(bone, WX), k.ldir(bone, UP), -0.004, 0.004))
            for i, (x, dz) in enumerate(k.nut_spots(name)):
                o = k.local(bone, (x, k.PLATE[1], w + dz))
                out.append(k.prism("nut-%s-%s-%d" % (name, side, i + 1), bone, ST, "detail", 0.25, k.hexagon(k.NUT_AF), o, [0, 0, 1], k.ldir(bone, WX), 0.0, 0.0125))
                top = k.local(bone, (x, k.PLATE[1] + 0.0125, w + dz))
                up = k.ldir(bone, UP)
                out.append(k.capsule("stud-%s-%s-%d" % (name, side, i + 1), bone, ST, "detail", 0.3, [top[0] - up[0] * 0.003, top[1] - up[1] * 0.003, top[2]], [top[0] + up[0] * 0.003, top[1] + up[1] * 0.003, top[2]], 0.006))
    return out


def brace():
    out = []
    a = math.radians(k.LEG_ANGLE)
    ends = ((0.0, (math.cos(a), math.sin(a), 0.0), 1), (k.BRACE_LEN, (-math.cos(a), math.sin(a), 0.0), -1))
    for side, sg in k.SIDES:
        w = sg * k.ZA
        out.append(k.revolve("brace-%s" % side, "brace", ST, "body", 0.5, [[0.0, 0.0], [0.0, k.R_BRACE], [k.BRACE_LEN - 0.06, k.R_BRACE], [k.BRACE_LEN - 0.06, 0.0]], WX, [0.03, 0.0, w]))
        for j, (u, d, toward) in enumerate(ends):
            o = [u - d[0] * 0.035, -d[1] * 0.035, w]
            out.append(k.revolve("clamp-%s-%d" % (side, j + 1), "brace", ST, "detail", 0.35, [[0.0, 0.03], [0.0, 0.035], [0.003, 0.038], [0.067, 0.038], [0.07, 0.035], [0.07, 0.03]], d, o))
            out.append(k.revolve("clamp-stub-%s-%d" % (side, j + 1), "brace", ST, "detail", 0.35, [[0.02, 0.0], [0.02, 0.026], [0.065, 0.026], [0.068, 0.023], [0.068, 0.0]], (toward, 0, 0), [u, 0.0, w]))
    return out


def beam():
    out = [k.revolve("beam", "beam", ST, "body", 0.45, [[0.0, 0.0], [0.0, 0.03], [0.002, 0.042], [0.005, k.R_BEAM], [1.895, k.R_BEAM], [1.898, 0.042], [1.9, 0.03], [1.9, 0.0]], WZ, [0.0, 0.0, -k.BEAM_HALF])]
    for side, sg in k.SIDES:
        z = sg * k.ZA
        out.append(k.revolve("sleeve-%s" % side, "beam", ST, "body", 0.4, [[0.0, 0.05], [0.0, 0.0545], [0.0025, k.R_SLEEVE], [0.1175, k.R_SLEEVE], [0.12, 0.0545], [0.12, 0.05]], WZ, [0.0, 0.0, z - k.SLEEVE_HALF]))
        for name in ("left", "right"):
            lo, hi = k.apex_point(name, k.SOCKET_S[1]), k.apex_point(name, k.SOCKET_S[0])
            d = k.leg_dir(name)
            ch = k.apex_point(name, k.SOCKET_S[1] - 0.003)
            secs = [k.section([lo[0] - k.APEX_X, lo[1] - k.H_BEAM, z], k.R_SOCKET - 0.003, k.R_SOCKET - 0.003), k.section([ch[0] - k.APEX_X, ch[1] - k.H_BEAM, z], k.R_SOCKET, k.R_SOCKET), k.section([hi[0] - k.APEX_X, hi[1] - k.H_BEAM, z], k.R_SOCKET, k.R_SOCKET)]
            out.append(k.loft("socket-%s-%s" % (name, side), "beam", ST, "body", 0.4, secs))
            m = k.apex_point(name, 0.13)
            out.append(k.prism("socket-bolt-%s-%s" % (name, side), "beam", ST, "detail", 0.25, k.hexagon(k.NUT_AF, turn=math.degrees(math.atan2(d[1], d[0]))), [m[0] - k.APEX_X, m[1] - k.H_BEAM, 0.0], WX, UP, z + sg * (k.R_SOCKET - 0.0015), z + sg * (k.R_SOCKET + 0.0065)))
            out[-1]["from"], out[-1]["to"] = sorted([out[-1]["from"], out[-1]["to"]])
            out.append(k.prism("socket-nut-%s-%s" % (name, side), "beam", ST, "detail", 0.25, k.hexagon(k.NUT_AF, turn=math.degrees(math.atan2(d[1], d[0]))), [m[0] - k.APEX_X, m[1] - k.H_BEAM, 0.0], WX, UP, z - sg * (k.R_SOCKET - 0.0015), z - sg * (k.R_SOCKET + 0.0085)))
            out[-1]["from"], out[-1]["to"] = sorted([out[-1]["from"], out[-1]["to"]])
        out.append(k.prism("gusset-%s" % side, "beam", ST, "body", 0.42, k.gusset_world(), [0.0, 0.0, z], WX, UP, -0.005, 0.005))
        out.append(k.prism("sleeve-bolt-%s" % side, "beam", ST, "detail", 0.25, k.hexagon(k.NUT_AF), [0.0, 0.0555, z], WZ, WX, 0.0, 0.008))
        out.append(k.prism("sleeve-nut-%s" % side, "beam", ST, "detail", 0.25, k.hexagon(k.NUT_AF), [0.0, -0.0555, z], WZ, WX, -0.01, 0.0))
    for name, bone, zc in k.CHAINS:
        out.append(k.revolve("hanger-band-%s" % name, "beam", ST, "detail", 0.35, [[0.0, 0.05], [0.0, 0.0565], [0.002, 0.0585], [0.048, 0.0585], [0.05, 0.0565], [0.05, 0.05]], WZ, [0.0, 0.0, zc - 0.025]))
        cheek = [[-0.025, -0.048], [0.025, -0.048]] + [[0.025 * math.cos(math.radians(t)), -0.085 + 0.025 * math.sin(math.radians(t))] for t in (0, -30, -60, -90, -120, -150, 180)]
        for j, dz in enumerate((0.017, -0.017)):
            out.append(k.prism("hanger-cheek-%s-%d" % (name, j + 1), "beam", ST, "detail", 0.35, cheek, [0.0, 0.0, zc + dz], WX, UP, -0.004, 0.004))
        out.append(k.revolve("hanger-bolt-%s" % name, "beam", ST, "detail", 0.25, [[0.0, 0.0], [0.0, 0.0095], [0.009, 0.0095], [0.009, 0.006], [0.051, 0.006], [0.051, 0.0095], [0.06, 0.0095], [0.06, 0.0]], WZ, [0.0, -0.064, zc - 0.03]))
        out.append(k.revolve("hanger-pin-%s" % name, "beam", ST, "detail", 0.25, [[0.0, 0.0], [0.0, 0.012], [0.01, 0.012], [0.01, 0.008], [0.052, 0.008], [0.052, 0.012], [0.062, 0.012], [0.062, 0.0]], WZ, [0.0, -k.PIVOT_DROP, zc - 0.031]))
    return out


def chains():
    out = []
    for name, bone, zc in k.CHAINS:
        out.append(k.revolve("top-eye-%s" % name, bone, ST, "detail", 0.35, k.torus_profile(0.0155, 0.0055), WZ, [0.0, 0.0, zc]))
        out.append(k.capsule("hook-shank-%s" % name, bone, ST, "detail", 0.35, [0.019, 0.0, zc], [0.046, 0.0, zc], 0.0065))
        out.append(k.revolve("lower-eye-%s" % name, bone, ST, "detail", 0.35, k.torus_profile(0.013, 0.0055), UP, [0.058, 0.0, zc]))
        for parity, axis in ((0, 1), (1, 2)):
            path = k.link_path(k.link_u(parity), axis, zc)[1:-1]
            normal = [0, 0, 1] if axis == 1 else [0, 1, 0]
            sections = []
            for i, point in enumerate(path):
                tangent = k.unit(k.sub(path[(i + 1) % len(path)], path[i - 1]))
                rotation = k.quat_from_axes(normal, k.cross(tangent, normal), tangent)
                sections.append(k.section(point, k.WIRE_R, k.WIRE_R, rotation=rotation))
            tag = "a" if parity == 0 else "b"
            out.append(k.loft(f"links-{tag}-{name}", bone, ST, "body", 0.5,
                              sections + [sections[0]], caps=False,
                              repeat={"count": k.LINKS // 2, "translation": [2 * k.PITCH, 0, 0]}))
    return out


def seat():
    cv = k.SEAT_TOP_V - k.SEAT[1] / 2
    hz = k.SEAT[2] / 2
    prof = [(hz, 0.072, 0.012), (hz - 0.0015, 0.081, 0.0155), (hz - 0.0045, 0.0855, 0.0172), (hz - 0.01, 0.0875, 0.0175)]
    rows = [(-w, rx, ry) for w, rx, ry in prof] + [(0.0, 0.0875, 0.0175)] + [(w, rx, ry) for w, rx, ry in reversed(prof)]
    out = [k.loft("seat", "seat", RB, "body", 0.2, [k.section([0.0, cv, w], rx, ry, exponent=4) for w, rx, ry in rows])]
    for side, sg in k.SIDES:
        z = sg * k.SEAT_EYE_Z
        out.append(k.revolve("seat-eye-%s" % side, "seat", ST, "detail", 0.3, k.torus_profile(k.SEAT_EYE_RC, k.SEAT_EYE_R), WZ, [0.0, 0.0, z]))
        out.append(k.revolve("seat-collar-%s" % side, "seat", ST, "detail", 0.3, [[0.0, 0.0], [0.0, 0.013], [0.008, 0.013], [0.01, 0.005], [0.01, 0.0]], UP, [0.0, k.SEAT_TOP_V, z]))
        out.append(k.capsule("seat-bolt-%s" % side, "seat", ST, "detail", 0.3, [0.0, -0.019, z], [0.0, -0.068, z], 0.005))
        out.append(k.revolve("seat-nut-%s" % side, "seat", ST, "detail", 0.25, [[0.0, 0.0], [0.0, 0.0105], [-0.01, 0.0105], [-0.013, 0.008], [-0.0155, 0.004], [-0.016, 0.0]], UP, [0.0, k.SEAT_TOP_V - k.SEAT[1], z]))
    return out


def body():
    return legs() + brace() + beam() + chains() + seat()


def region(sid):
    if sid == "seat":
        return SR
    if sid.startswith(("top-eye", "hook-shank", "lower-eye", "links", "seat-")):
        return CS
    return FG


def thermal(b):
    return [{"id": "th-" + s["id"], "of": s["id"], "region": region(s["id"])} for s in b]


def structure(com):
    out = []
    a = math.radians(k.LEG_ANGLE)
    u0, u1 = k.PLATE[1] / math.sin(a), k.LEG_LEN - k.LEG_TOP_S
    for name, bone, xf in k.LEGS:
        for side, sg in k.SIDES:
            w = sg * k.ZA
            m = k.revolve("member-leg-%s-%s" % (name, side), bone, ST, "body", 0.45, [[u0, k.R_LEG_IN], [u1, k.R_LEG_IN], [u1, k.R_LEG], [u0, k.R_LEG]], WX, [0.0, 0.0, w])
            m.update({"member": bone, "part": "member"})
            out.append(m)
            out.append({"id": "joint-foot-%s-%s" % (name, side), "of": "plate-%s-%s" % (name, side), "member": bone, "part": "joint"})
            mass = k.ellipsoid("mass-leg-%s-%s" % (name, side), bone, ST, "detail", 0.2, [0.07, 0.033, 0.033], [com[bone][1], 0.0, w])
            mass.update({"member": bone, "part": "mass"})
            out.append(mass)
    for side, sg in k.SIDES:
        w = sg * k.ZA
        m = k.revolve("member-brace-%s" % side, "brace", ST, "body", 0.5, [[0.0, k.R_BRACE_IN], [k.BRACE_LEN, k.R_BRACE_IN], [k.BRACE_LEN, k.R_BRACE], [0.0, k.R_BRACE]], WX, [0.0, 0.0, w])
        m.update({"member": "brace", "part": "member"})
        out.append(m)
        for j in (1, 2):
            out.append({"id": "joint-clamp-%s-%d" % (side, j), "of": "clamp-%s-%d" % (side, j), "member": "brace", "part": "joint"})
        mass = k.ellipsoid("mass-brace-%s" % side, "brace", ST, "detail", 0.2, [0.06, 0.024, 0.024], [com["brace"][1], 0.0, w])
        mass.update({"member": "brace", "part": "mass"})
        out.append(mass)
        out.append({"id": "joint-apex-%s" % side, "of": "sleeve-%s" % side, "member": "beam", "part": "joint"})
    m = k.revolve("member-beam", "beam", ST, "body", 0.45, [[0.0, k.R_BEAM - 0.0036], [2 * k.BEAM_TUBE_HALF, k.R_BEAM - 0.0036], [2 * k.BEAM_TUBE_HALF, k.R_BEAM], [0.0, k.R_BEAM]], WZ, [0.0, 0.0, -k.BEAM_TUBE_HALF])
    m.update({"member": "beam", "part": "member"})
    out.append(m)
    mass = k.ellipsoid("mass-beam", "beam", ST, "detail", 0.2, [0.02, 0.023, 0.1], [0.0, com["beam"][2], 0.0])
    mass.update({"member": "beam", "part": "mass"})
    out.append(mass)
    for name, bone, zc in k.CHAINS:
        m = k.capsule("member-chain-%s" % name, bone, ST, "body", 0.5, [0.0, 0.0, zc], [k.CHAIN_LEN, 0.0, zc], k.WIRE_R * math.sqrt(2), slender=True)
        m.update({"member": bone, "part": "member"})
        out.append(m)
        out.append({"id": "joint-pivot-%s" % name, "of": "hanger-pin-%s" % name, "member": bone, "part": "joint"})
        mass = k.ellipsoid("mass-chain-%s" % name, bone, ST, "detail", 0.2, [0.03, 0.012, 0.012], [com[bone][1], 0.0, zc])
        mass.update({"member": bone, "part": "mass"})
        out.append(mass)
    core = k.box("member-seat-core", "seat", AL, "body", 0.6, [0.16, 0.008, 0.40], [0.0, k.SEAT_TOP_V - k.SEAT[1] / 2, 0.0])
    core.update({"member": "seat", "part": "member"})
    out.append(core)
    for side, sg in k.SIDES:
        out.append({"id": "joint-hitch-%s" % side, "of": "seat-eye-%s" % side, "member": "seat", "part": "joint"})
    mass = k.ellipsoid("mass-seat", "seat", RB, "detail", 0.2, [0.04, 0.01, 0.06], [0.0, com["seat"][2], 0.0])
    mass.update({"member": "seat", "part": "mass"})
    out.append(mass)
    out += supports(0.045, 0.04, 0.015)
    return out


def supports(half, depth, roller_r):
    """🟩 One support solid per anchor below its base plate: a wedge for a pin, a cylinder for a roller (B3)."""
    out = []
    for name, bone, xf in k.LEGS:
        for side, sg in k.SIDES:
            w = sg * k.ZA
            anchor = ("pin-left-%s" if name == "left" else "roller-right-%s") % side
            if name == "left":
                s = k.prism("support-" + anchor, bone, ST, "body", 0.15, [[xf - half, -depth], [xf + half, -depth], [xf, 0.0]], k.local(bone, (0.0, 0.0, w)), k.ldir(bone, WX), k.ldir(bone, UP), -k.PLATE[2] / 2, k.PLATE[2] / 2)
            else:
                c = k.local(bone, (xf, -roller_r, w - k.PLATE[2] / 2))
                s = k.revolve("support-" + anchor, bone, ST, "body", 0.15, [[0.0, 0.0], [0.0, roller_r], [k.PLATE[2], roller_r], [k.PLATE[2], 0.0]], WZ, c)
            s.update({"member": bone, "part": "support", "anchor": anchor})
            out.append(s)
    return out


DECISIONS = [
    {"en": "Body: the real single-bay swing part by part. Each A-frame: two o60.3 legs on their true centre lines, each a loft whose foot is cut by the plate plane (no wedge gap) and whose head ends inside an o76.1 socket; 180 x 100 x 12 mm base plates with two 8 mm gussets (outer and inner) and two anchor nuts with stud ends; the o42.4 tie at 0.6 m in cast tee clamps (collar round the leg, stub round the tie); the welded apex bracket (o114.3 sleeve round the beam, two sockets with M12 through-bolts, 10 mm gusset, vertical M12 bolt through sleeve and beam). The o101.6 beam as one revolve with domed ends. Two swing hangers: clamp band, forged fork of two 8 mm cheeks, clamp bolt, bearing pin 85 mm below the beam axis. Each chain: forged top eye on the pin, shank, lower eye, then 76 DIN 766 8 x 24 links as round-wire lofts on their stadium centre line, alternating in two planes and interlocking without gaps (two repeated lofts per chain). The last link hangs in the eye bolt of the seat; eye bolts with forged collar and acorn nut under the seat. Seat: moulded rubber, 430 x 175 x 35 mm loft of rounded-rectangle sections with rounded ends.",
     "de": "Körper: die reale Einfeldschaukel Teil für Teil. Jeder A-Bock: zwei Beine o60,3 auf ihren wahren Mittellinien, jedes eine Loft, deren Fuß von der Plattenebene geschnitten wird (keine Keillücke) und deren Kopf in einer Muffe o76,1 endet; Fußplatten 180 x 100 x 12 mm mit zwei 8-mm-Rippen (außen und innen) und zwei Ankermuttern mit Bolzenenden; das Zugrohr o42,4 auf 0,6 m in gegossenen T-Klemmen (Manschette um das Bein, Stutzen um das Rohr); der geschweißte Scheitelbeschlag (Hülse o114,3 um den Holm, zwei Muffen mit M12-Durchsteckschrauben, 10-mm-Knotenblech, senkrechte M12-Schraube durch Hülse und Holm). Der Holm o101,6 als eine Rotation mit gewölbten Enden. Zwei Schaukelgelenke: Klemmband, geschmiedete Gabel aus zwei 8-mm-Wangen, Klemmschraube, Lagerbolzen 85 mm unter der Holmachse. Jede Kette: geschmiedetes Kopfauge am Bolzen, Schaft, unteres Auge, dann 76 Glieder DIN 766 8 x 24 als Rundstahl-Lofts auf ihrer Stadionmittellinie, abwechselnd in zwei Ebenen und lückenlos ineinander (zwei wiederholte Lofts je Kette). Das letzte Glied hängt in der Ringschraube des Sitzes; Ringschrauben mit geschmiedetem Bund und Hutmutter unter dem Sitz. Sitz: Formgummi, Loft 430 x 175 x 35 mm aus Rundrechteck-Schnitten mit gerundeten Enden."},
    {"en": "Thermal: three single-layer regions (B2), every solid a reference to its body solid: frame-galv = legs, plates, gussets, anchor nuts, ties and clamps, beam, apex brackets and their bolts, swing hangers; chain-steel = top eyes, hooks, all links, seat eye bolts and acorn nuts; seat-rubber = the seat. No emitters, no hidden regions. The aluminium core of the seat is not a thermal region (it lies inside the rubber).",
     "de": "Thermik: drei einschichtige Regionen (B2), jeder Körper eine Referenz auf seinen Körperteil: frame-galv = Beine, Platten, Rippen, Ankermuttern, Zugrohre und Klemmen, Holm, Scheitelbeschläge mit Schrauben, Schaukelgelenke; chain-steel = Kopfaugen, Haken, alle Glieder, Ringschrauben und Hutmuttern des Sitzes; seat-rubber = der Sitz. Keine Emitter, keine verdeckten Regionen. Der Aluminiumkern des Sitzes ist keine thermische Region (er liegt im Gummi)."},
    {"en": "Structure: the load path of the real swing. Members: four o60.3 x 2.9 leg tubes and two o42.4 x 2.6 ties as hollow sections on their axes, the o101.6 x 3.6 beam, the chains as slender rods of the area of the two wires of a link (o11.3 equivalent) from pin to eye bolt, the 160 x 8 mm aluminium core of the seat. Joints: base plates (leg feet), tee clamps, apex sleeves, bearing pins (chain pivots), seat eye bolts (hitches). Supports: a wedge under each pin foot and a 30 mm roller under each roller foot at their anchors. Masses at the centres of the legs, ties, beam (32 mm below its axis: sockets and hangers hang under it), chains and seat.",
     "de": "Tragwerk: der Lastpfad der realen Schaukel. Stäbe: vier Beinrohre o60,3 x 2,9 und zwei Zugrohre o42,4 x 2,6 als Hohlprofile auf ihren Achsen, der Holm o101,6 x 3,6, die Ketten als schlanke Stäbe mit der Fläche der zwei Drähte eines Glieds (Ersatz o11,3) vom Bolzen zur Ringschraube, der Aluminiumkern 160 x 8 mm des Sitzes. Gelenke: Fußplatten (Beinfüße), T-Klemmen, Scheitelhülsen, Lagerbolzen (Kettendrehpunkte), Ringschrauben (Sitzgelenke). Auflager: ein Keil unter jedem Festlagerfuß und eine 30-mm-Rolle unter jedem Rollenlagerfuß an ihren Ankern. Massen in den Schwerpunkten von Beinen, Zugrohren, Holm (32 mm unter seiner Achse: Muffen und Gelenke hängen darunter), Ketten und Sitz."},
]


def volume(com):
    b = body()
    return {"status": "authored", "denominator": 50, "body": b, "thermal": thermal(b), "structure": structure(com), "decisions": DECISIONS}
