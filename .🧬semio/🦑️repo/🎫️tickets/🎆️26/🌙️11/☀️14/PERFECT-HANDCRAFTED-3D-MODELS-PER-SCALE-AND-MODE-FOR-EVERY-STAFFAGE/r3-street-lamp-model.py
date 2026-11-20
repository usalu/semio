"""🗼 Round-3 volumes of `objects/street-lamp`: every solid of every scale (1:50, 1:100, 1:200, 1:500) and list (body, thermal, structure) stated by hand from the dimensions of `r3-street-lamp-kit.py`; writes `volumes/objects/street-lamp.json`.

Usage: `python3 r3-street-lamp-model.py`
@see ./r3-street-lamp-kit.py
@see ./r3-objects-street-lamp.md
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-street-lamp-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
r6 = K.r6

STEEL, CONC, ALU, PC = "galv-steel", "concrete", "aluminium-anodised", "polycarbonate"
TONE = {STEEL: 0.4, CONC: 0.65, ALU: 0.5, PC: 0.85}


def base(sid, kind, bone, material, role="body", tone=None, **more):
    """🏷 Common members of a solid."""
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone}
    out.update(more)
    return out


def box(sid, bone, material, centre, size, radius=0.0, role="body", tone=None, **more):
    """📦 A box at a bone-local centre."""
    out = base(sid, "box", bone, material, role, tone)
    out["size"] = [r6(s) for s in size]
    if radius:
        out["cornerRadius"] = r6(radius)
    out["transform"] = {"translation": [r6(c) for c in centre]}
    out.update(more)
    return out


def revolve(sid, bone, material, profile, axis, origin=(0, 0, 0), role="body", tone=None, **more):
    """⭕ A revolve of an (axial, radial) profile."""
    out = base(sid, "revolve", bone, material, role, tone)
    out.update({"profile": [[r6(a), r6(r)] for a, r in profile], "axis": [r6(c) for c in axis], "origin": [r6(c) for c in origin]})
    out.update(more)
    return out


def loop(profile):
    """🔁 A closed revolve loop: the first point repeated (kernel closing rule)."""
    return list(profile) + [profile[0]]


def extrude(sid, bone, material, plane, profile, a, b, role="body", tone=None, **more):
    """🧱 An extrude of a plane profile between a and b."""
    out = base(sid, "extrude", bone, material, role, tone)
    out.update({"profile": [[r6(p), r6(q)] for p, q in profile], "plane": plane, "from": r6(a), "to": r6(b)})
    out.update(more)
    return out


def loft(sid, bone, material, sections, role="body", tone=None, **more):
    """🧵 A capped loft."""
    out = base(sid, "loft", bone, material, role, tone)
    out.update({"sections": sections, "capStart": True, "capEnd": True})
    out.update(more)
    return out


def ellipsoid(sid, bone, material, centre, radii, role="body", tone=None, **more):
    """🥚 An ellipsoid at a bone-local centre."""
    out = base(sid, "ellipsoid", bone, material, role, tone)
    out.update({"radii": [r6(r) for r in radii], "transform": {"translation": [r6(c) for c in centre]}})
    out.update(more)
    return out


def capsule(sid, bone, material, p0, p1, r, role="body", tone=None, **more):
    """💊 A capsule between two bone-local points."""
    out = base(sid, "capsule", bone, material, role, tone)
    out.update({"p0": [r6(c) for c in p0], "p1": [r6(c) for c in p1], "r0": r6(r), "r1": r6(r)})
    out.update(more)
    return out


def ref(sid, of, **more):
    """🔗 A reference to a body solid."""
    out = {"id": sid, "of": of}
    out.update(more)
    return out


U, V = (1, 0, 0), (0, 1, 0)
TT = K.TUBE_TOP
R = K.pole_r
COVER_PROFILE = [(0.12, 0.12), (0.13, 0.12), (0.155, 0.1105), (0.1775, 0.1005), (0.21, 0.092), (0.24, 0.0875), (0.27, 0.0852), (0.30, 0.0843)]
COVER_100 = [(0.12, 0.12), (0.13, 0.12), (0.17, 0.1015), (0.21, 0.092), (0.30, 0.0843)]
BRACE_AX = K.unit((K.BRACE_TOP[0] - K.BRACE_FOOT[0], K.BRACE_TOP[1] - K.BRACE_FOOT[1], 0.0))
BRACE_LEN = math.dist(K.BRACE_TOP, K.BRACE_FOOT)
BRACE_P0 = (K.BRACE_FOOT[0], K.BRACE_FOOT[1], 0.0)
BRACE_P1 = (K.BRACE_TOP[0], K.BRACE_TOP[1], 0.0)
LUG = [(0.555, -0.027), (0.635, -0.027), (0.62, -0.040), (0.595, -0.052), (0.570, -0.050)]
SUB_100 = [0, 1, 3, 5, 8, 9, 10, 11, 12]
SUB_200 = [0, 2, 4, 7, 9, 12]
HEAD_500 = [(0.000, 0.050, 0.034, -0.032), (0.080, 0.120, 0.049, -0.050), (0.430, 0.128, 0.036, -0.050), (0.505, 0.060, -0.002, -0.030)]
DOOR_AT = math.pi
PLUME_Y = K.fin_top(K.PLUME_U, 0.0)


def masses(r_found, r_pole, r_arm, r_head):
    """⚖ The four lumped masses at the declared centres of mass of the rig bones."""
    return [
        ellipsoid("mass-foundation", "foundation", STEEL, (0, 0.11, 0), (r_found,) * 3, tone=0.5, member="foundation", part="mass"),
        ellipsoid("mass-pole", "pole", STEEL, (2.94, 0, 0), (r_pole,) * 3, tone=0.5, member="pole", part="mass"),
        ellipsoid("mass-arm", "arm", ALU, (0.41, 0, 0), (r_arm,) * 3, tone=0.5, member="arm", part="mass"),
        ellipsoid("mass-head", "head", ALU, (0.24, -0.005, 0), (r_head,) * 3, tone=0.5, member="head", part="mass"),
    ]


def foundation(sid, h, top, radius=0.0):
    """🧱 The foundation member above the support pad."""
    return box(sid, "foundation", CONC, (0, (h + top) / 2, 0), (0.30, top - h, 0.30), radius, member="foundation", part="member")


def support(h, side=0.30):
    """🟩 The fixed support at `foundation-support`: the bearing pad in the ground joint under the plinth."""
    return box("support-foundation-support", "foundation", CONC, (0, h / 2, 0), (side, h, side), tone=0.1, member="foundation", part="support", anchor="foundation-support")


def plume(sec, n, rise, half_u, half_z, thick, steps=8):
    """♨ The plume outlet (D2): a curved plate lying on the hot top of the housing over the LED board (on the fin tops at 1:50), following the crown of section `sec` (centre, rx, ry) laterally, as thin as a surface patch (texture)."""
    secs = sec if isinstance(sec, list) else [sec]
    zs = [-half_z + 2 * half_z * k / steps for k in range(steps + 1)]
    top = [max(K.sp_top(cy, rx, ry, n, z) for cy, rx, ry in secs) + rise for z in zs]
    profile = [(y - 0.001, z) for y, z in zip(top, zs)] + [(y + thick, z) for y, z in reversed(list(zip(top, zs)))]
    return extrude("th-lamp-plume", "head", ALU, "yz", profile, K.PLUME_U - half_u, K.PLUME_U + half_u, role="detail", tone=0.9, region="lamp-housing", tags=["lamp-plume"], texture=True)


def s50():
    """🔍 1:50 (feature 7.5 mm): every assembly of the column, bracket and luminaire."""
    door_mid = R((K.DOOR_U0 + K.DOOR_U1) / 2)
    sweep = 2 * K.DOOR_HALF / door_mid
    phi = DOOR_AT - sweep / 2
    hexa = K.hexagon(K.NUT_AF)
    nut_rep = {"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": r6(math.pi / 2)}
    body = [
        box("plinth", "foundation", CONC, (0, 0.05, 0), (0.30, 0.10, 0.30), 0.008),
        box("base-plate", "foundation", STEEL, (0, 0.11, 0), (0.30, 0.02, 0.30), 0.004, tone=0.35),
        extrude("base-nut", "foundation", STEEL, "zx", hexa, K.NUT_Y0, K.NUT_Y1, role="detail", tone=0.2, transform={"translation": [K.BOLT_C, 0, K.BOLT_C]}, repeat=nut_rep),
        revolve("base-bolt", "foundation", STEEL, [(0.135, 0), (0.135, K.STUB_R), (K.STUB_Y1, K.STUB_R), (K.STUB_Y1, 0)], V, (K.BOLT_C, 0, K.BOLT_C), role="detail", tone=0.25, repeat=nut_rep),
        revolve("base-cover", "foundation", STEEL, loop(COVER_PROFILE + [(a, r - 0.003) for a, r in reversed(COVER_PROFILE)]), V, tone=0.33),
        revolve("pole", "pole", STEEL, loop([(0, K.POLE_R0), (TT, R(TT)), (TT, R(TT) - K.POLE_WALL), (0, K.POLE_R0 - K.POLE_WALL)]), U),
        revolve("pole-cap", "pole", STEEL, K.CAP, U, tone=0.3),
        revolve("pole-collar", "pole", STEEL, loop([(K.BAND_U0, R(K.BAND_U0) - 0.001), (K.BAND_U1, R(K.BAND_U1) - 0.001), (K.BAND_U1, K.BAND_R), (K.BAND_U0, K.BAND_R)]), U, role="detail", tone=0.3),
        revolve("pole-hatch", "pole", STEEL, loop([(K.DOOR_U0, R(K.DOOR_U0) - 0.001), (K.DOOR_U1, R(K.DOOR_U1) - 0.001), (K.DOOR_U1, R(K.DOOR_U1) + 0.003), (K.DOOR_U0, R(K.DOOR_U0) + 0.003)]), U, role="detail", tone=0.3, sweepRad=r6(sweep), transform={"rotation": [r6(math.sin(phi / 2)), 0, 0, r6(math.cos(phi / 2))]}, texture=True),
        revolve("arm-clamp", "arm", ALU, loop([(-K.CLAMP_H / 2, 0.0405), (K.CLAMP_H / 2, 0.0405), (K.CLAMP_H / 2, K.CLAMP_R), (-K.CLAMP_H / 2, K.CLAMP_R)]), V, role="detail", tone=0.3),
        box("arm-clamp-ear", "arm", ALU, (K.EAR_X, 0, 0), (0.023, 0.08, 0.028), 0.003, role="detail", tone=0.3),
        extrude("arm-clamp-bolt", "arm", STEEL, "xy", K.hexagon(K.EAR_BOLT_AF), -K.EAR_BOLT_Z, K.EAR_BOLT_Z, role="detail", tone=0.2, transform={"translation": [K.EAR_X, -K.EAR_BOLT_V, 0]}, repeat={"count": 2, "translation": [0, r6(2 * K.EAR_BOLT_V), 0]}),
        revolve("arm-socket", "arm", ALU, [(0.040, 0), (0.040, 0.038), (0.080, 0.038), (0.092, K.ARM_R + 0.002), (0.092, 0)], U, tone=0.45),
        revolve("arm", "arm", ALU, [(0.035, 0), (0.035, K.ARM_R), (K.ARM_L, K.ARM_R), (K.ARM_L, 0)], U),
        capsule("arm-brace", "arm", ALU, BRACE_P0, BRACE_P1, K.BRACE_R, tone=0.45),
        extrude("arm-brace-joint", "arm", ALU, "xy", LUG, -0.006, 0.006, role="detail", tone=0.3),
        extrude("arm-brace-foot", "arm", STEEL, "xy", K.FOOT_LUG, -K.LUG_HALF, K.LUG_HALF, role="detail", tone=0.3),
        revolve("head-spigot", "head", ALU, [(-0.03, 0), (-0.03, 0.038), (0.012, 0.038), (0.012, 0)], U, role="detail", tone=0.35),
        loft("head-housing", "head", ALU, K.loft(K.cover_rows(), K.COVER_N)),
        loft("head-tray", "head", ALU, K.loft(K.tray_rows(), K.TRAY_N), tone=0.42),
        extrude("head-fin", "head", ALU, "yz", K.fin_profile(K.PLUME_U), K.FIN_U[0] - K.FIN_T / 2, K.FIN_U[0] + K.FIN_T / 2, role="detail", tone=0.3, repeat={"count": len(K.FIN_U), "translation": [0.035, 0, 0]}, texture=True),
        extrude("head-hatch-joint", "head", ALU, "yz", K.joint_profile(), K.JOINT_U - K.JOINT_T / 2, K.JOINT_U + K.JOINT_T / 2, role="detail", tone=0.3, texture=True),
        extrude("lens", "head", PC, "zx", K.rounded_rect(-K.LENS_Z, K.LENS_Z, K.LENS_U0, K.LENS_U1, K.LENS_R), K.LENS_Y0, K.LENS_Y1, role="glazing", texture=True),
    ]
    thermal = [
        ref("th-plinth", "plinth", region="plinth-concrete"),
        ref("th-base-plate", "base-plate", region="pole-galv"),
        ref("th-base-nut", "base-nut", region="pole-galv"),
        ref("th-base-bolt", "base-bolt", region="pole-galv"),
        revolve("th-base-cover", "foundation", STEEL, [(0.12, 0)] + COVER_PROFILE + [(0.30, 0)], V, tone=0.33, region="pole-galv"),
        revolve("th-pole", "pole", STEEL, [(0, 0), (0, K.POLE_R0), (TT, R(TT)), (TT, 0)], U, region="pole-galv"),
        ref("th-pole-cap", "pole-cap", region="pole-galv"),
        ref("th-pole-collar", "pole-collar", region="pole-galv"),
        ref("th-arm", "arm", region="arm-alu"),
        ref("th-arm-socket", "arm-socket", region="arm-alu"),
        ref("th-arm-clamp", "arm-clamp", region="arm-alu"),
        ref("th-arm-clamp-ear", "arm-clamp-ear", region="arm-alu"),
        ref("th-arm-brace", "arm-brace", region="arm-alu"),
        ref("th-arm-brace-joint", "arm-brace-joint", region="arm-alu"),
        ref("th-arm-clamp-bolt", "arm-clamp-bolt", region="arm-alu"),
        ref("th-arm-brace-foot", "arm-brace-foot", region="pole-galv"),
        ref("th-head-hatch-joint", "head-hatch-joint", region="lamp-housing"),
        ref("th-head-housing", "head-housing", region="lamp-housing"),
        ref("th-head-tray", "head-tray", region="lamp-housing"),
        ref("th-head-spigot", "head-spigot", region="lamp-housing"),
        ref("th-head-fin", "head-fin", region="lamp-housing"),
        ref("th-lens", "lens", region="lamp-lens", sectionFaces=["+v"]),
        plume(K.cover_sec(*K.head_at(K.PLUME_U)), K.COVER_N, K.FIN_RISE, 0.0895, K.FIN_Z, 0.002),
    ]
    structure = [
        foundation("member-plinth", 0.015, 0.10, 0.008),
        ref("member-base-plate", "base-plate", member="foundation", part="member"),
        revolve("member-anchor-bolt", "foundation", STEEL, [(0.0, 0), (0.0, K.STUB_R), (K.STUB_Y1, K.STUB_R), (K.STUB_Y1, 0)], V, (K.BOLT_C, 0, K.BOLT_C), tone=0.25, repeat=nut_rep, member="foundation", part="member"),
        ref("member-pole", "pole", member="pole", part="member"),
        revolve("member-arm", "arm", ALU, loop([(0.035, K.ARM_R - K.ARM_WALL), (K.ARM_L, K.ARM_R - K.ARM_WALL), (K.ARM_L, K.ARM_R), (0.035, K.ARM_R)]), U, member="arm", part="member"),
        revolve("member-arm-brace", "arm", ALU, loop([(0, K.BRACE_R - K.BRACE_WALL), (BRACE_LEN, K.BRACE_R - K.BRACE_WALL), (BRACE_LEN, K.BRACE_R), (0, K.BRACE_R)]), BRACE_AX, BRACE_P0, tone=0.45, member="arm", part="member"),
        ref("member-head-chassis", "head-tray", member="head", part="member"),
        revolve("joint-base-weld", "pole", STEEL, loop([(0, K.POLE_R0 - 0.001), (0.012, K.POLE_R0 - 0.001), (0, K.POLE_R0 + 0.012)]), U, tone=0.3, member="pole", part="joint"),
        ref("joint-arm-clamp", "arm-clamp", member="arm", part="joint"),
        ref("joint-arm-clamp-ear", "arm-clamp-ear", member="arm", part="joint"),
        ref("joint-brace-foot", "pole-collar", member="arm", part="joint"),
        ref("joint-brace-lug", "arm-brace-joint", member="arm", part="joint"),
        ref("joint-brace-foot-lug", "arm-brace-foot", member="arm", part="joint"),
        ref("joint-arm-clamp-bolt", "arm-clamp-bolt", member="arm", part="joint"),
        revolve("joint-head-spigot", "head", ALU, loop([(-0.03, K.ARM_R - 0.001), (0.10, K.ARM_R - 0.001), (0.10, 0.036), (-0.03, 0.036)]), U, tone=0.35, member="head", part="joint"),
        support(0.015),
    ] + masses(0.03, 0.05, 0.016, 0.022)
    decisions = [
        {"en": "Body: a 6 m galvanised conical column as built: concrete plinth 0.30 x 0.30 x 0.10 m with rounded arrises, 300 x 300 x 20 mm base plate with four M20 anchor bolts (hex nuts AF 30 with washer, bolt ends), a 3 mm flared base shroud, the straight cone 170 to 80 mm (15 mm per metre, 4 mm wall) closed by a push-in dome cap, a flush inspection door (500 x 90 mm, texture), a brace band at 5.35 m; the bracket arm is a 60 mm aluminium tube in a cast clamp sleeve with its bolting ear, two M10 bolts and socket, braced by a 27 mm strut between a foot lug welded to the band and a gusset lug under the arm; the luminaire is a compact side-entry LED head 0.505 x 0.274 x 0.10 m: a domed die-cast cover over a flat-bottomed tray (parting line 12 mm below the arm axis), the spigot collar at the rear, the raised joint of the gear hatch (texture, 1.5 mm), six transverse cooling fins over the LED board (texture, 4 mm) and a flat polycarbonate lens 0.28 x 0.175 m proud of the tray (texture).", "de": "Körper: ein 6 m hoher verzinkter Kegelmast wie gebaut: Betonsockel 0,30 x 0,30 x 0,10 m mit gerundeten Kanten, Fußplatte 300 x 300 x 20 mm mit vier Ankerschrauben M20 (Sechskantmuttern SW 30 mit Scheibe, Schraubenenden), eine ausgestellte Fußmanschette von 3 mm, der gerade Kegel von 170 auf 80 mm (15 mm je Meter, 4 mm Wand) mit eingesteckter Kuppelkappe, eine bündige Revisionstür (500 x 90 mm, Textur), ein Strebenring auf 5,35 m; der Ausleger ist ein Aluminiumrohr von 60 mm in einer gegossenen Klemmhülse mit Schraublasche, zwei Schrauben M10 und Muffe, abgestützt durch eine Strebe von 27 mm zwischen einer am Ring angeschweißten Fußlasche und einem Knotenblech unter dem Arm; die Leuchte ist ein kompakter LED-Kopf mit seitlicher Einführung 0,505 x 0,274 x 0,10 m: eine gewölbte Druckgusshaube über einer flachen Wanne (Teilungsfuge 12 mm unter der Auslegerachse), der Zapfenkragen hinten, die erhabene Fuge der Geräteklappe (Textur, 1,5 mm), sechs quer liegende Kühlrippen über der LED-Platine (Textur, 4 mm) und eine flache Polycarbonat-Abdeckung 0,28 x 0,175 m unter der Wanne (Textur)."},
        {"en": "Thermal: the column and the base shroud are their closed outer envelopes (the sealed tube interior is not reachable by the air, C31), plate, nuts, bolt ends, cap and brace band reference the body; the arm region is the tube, socket, clamp sleeve with ear and bolts, strut and gusset lug (the steel foot lug belongs to the column region); the housing region is cover, tray, spigot collar, hatch joint and the fins; the lens is the body lens with its inner face against the tray as section face; the plume outlet is the fin field itself, a 2 mm curved plate 0.18 x 0.19 m lying on the fin tops (D2; the plume above it widens to the declared 0.30 m). The inspection door is flush in the tube envelope and not a separate thermal solid.", "de": "Thermik: Mast und Fußmanschette sind ihre geschlossenen Außenhüllen (das dichte Rohrinnere erreicht die Luft nicht, C31), Platte, Muttern, Schraubenenden, Kappe und Strebenring verweisen auf den Körper; die Region des Auslegers ist Rohr, Muffe, Klemmhülse mit Lasche und Schrauben, Strebe und Knotenblech (die stählerne Fußlasche gehört zur Mastregion); die Gehäuseregion ist Haube, Wanne, Zapfenkragen, Klappenfuge und die Rippen; die Abdeckung ist die Körperabdeckung mit ihrer Innenseite an der Wanne als Schnittfläche; der Fahnenaustritt ist das Rippenfeld selbst, eine gebogene Platte von 2 mm, 0,18 x 0,19 m, auf den Rippenoberkanten (D2; die Fahne darüber weitet sich auf die angegebenen 0,30 m). Die Revisionstür liegt bündig in der Rohrhülle und ist kein eigener Thermikkörper."},
        {"en": "Structure: the load path from the luminaire to the ground: the die-cast tray as head chassis held by the spigot clamp (joint), the 60 x 3 mm arm tube and the 26.9 x 2.6 mm strut (both hollow), the clamp sleeve with its two bolts, the brace band with the foot lug and the gusset lug as joints, the 4 mm conical tube with its fillet weld to the base plate, the plate, four M20 anchor bolts down into the concrete plinth, the fixed support as a 15 mm bearing pad in the ground joint under the plinth; masses lumped at the declared centres of the four bones.", "de": "Tragwerk: der Lastpfad von der Leuchte zum Boden: die Druckgusswanne als Kopfrahmen in der Zapfenklemme (Gelenk), das Auslegerrohr 60 x 3 mm und die Strebe 26,9 x 2,6 mm (beide hohl), Klemmhülse mit ihren zwei Schrauben, Strebenring mit Fußlasche und Knotenblech als Verbindungen, das 4 mm starke Kegelrohr mit seiner Kehlnaht an der Fußplatte, die Platte, vier Ankerschrauben M20 in den Betonsockel, das feste Auflager als Lagerplatte von 15 mm in der Bodenfuge unter dem Sockel; Massen an den angegebenen Schwerpunkten der vier Knochen."},
    ]
    return body, thermal, structure, decisions


def s100():
    """🔎 1:100 (feature 15 mm): base plate and nuts merged into the plinth, column solid with its dome, clamp as one sleeve, housing as cover and tray on seven stations."""
    rows = [K.HEAD[i] for i in SUB_100]
    body = [
        box("plinth", "foundation", CONC, (0, 0.06, 0), (0.30, 0.12, 0.30), 0.006),
        revolve("base-cover", "foundation", STEEL, [(0.12, 0)] + COVER_100 + [(0.30, 0)], V, tone=0.33),
        revolve("pole", "pole", STEEL, [(0, 0), (0, K.POLE_R0), (5.858, R(5.858)), (5.872, 0.034), (5.878, 0.022), (5.88, 0)], U),
        revolve("arm-clamp", "arm", ALU, [(-K.CLAMP_H / 2, 0), (-K.CLAMP_H / 2, K.CLAMP_R), (K.CLAMP_H / 2, K.CLAMP_R), (K.CLAMP_H / 2, 0)], V, role="detail", tone=0.3),
        revolve("arm", "arm", ALU, [(0.04, 0), (0.04, K.ARM_R), (K.ARM_L, K.ARM_R), (K.ARM_L, 0)], U),
        capsule("arm-brace", "arm", ALU, BRACE_P0, BRACE_P1, K.BRACE_R, tone=0.45, slender=True),
        loft("head-housing", "head", ALU, K.loft(K.cover_rows(rows), 2.6)),
        loft("head-tray", "head", ALU, K.loft(K.tray_rows(rows), K.TRAY_N), tone=0.42),
        extrude("lens", "head", PC, "zx", K.rounded_rect(-K.LENS_Z, K.LENS_Z, K.LENS_U0, K.LENS_U1, 0.03, 2), K.LENS_Y0, K.LENS_Y1, role="glazing", texture=True),
    ]
    thermal = [
        ref("th-plinth", "plinth", region="plinth-concrete"),
        ref("th-base-cover", "base-cover", region="pole-galv"),
        ref("th-pole", "pole", region="pole-galv"),
        ref("th-arm-clamp", "arm-clamp", region="arm-alu"),
        ref("th-arm", "arm", region="arm-alu"),
        ref("th-arm-brace", "arm-brace", region="arm-alu"),
        ref("th-head-housing", "head-housing", region="lamp-housing"),
        ref("th-head-tray", "head-tray", region="lamp-housing"),
        ref("th-lens", "lens", region="lamp-lens", sectionFaces=["+v"]),
        plume([K.cover_sec(*K.head_at(K.PLUME_U + d, rows)) for d in (-0.10, -0.05, 0.0, 0.05, 0.10)], 2.6, 0.0, 0.10, 0.11, 0.003),
    ]
    structure = [
        foundation("member-foundation", 0.015, 0.12, 0.006),
        revolve("member-pole", "pole", STEEL, loop([(0, K.POLE_R0 - K.POLE_WALL), (5.858, R(5.858) - K.POLE_WALL), (5.858, R(5.858)), (0, K.POLE_R0)]), U, member="pole", part="member"),
        revolve("member-arm", "arm", ALU, loop([(0.04, K.ARM_R - K.ARM_WALL), (K.ARM_L, K.ARM_R - K.ARM_WALL), (K.ARM_L, K.ARM_R), (0.04, K.ARM_R)]), U, member="arm", part="member"),
        ref("member-arm-brace", "arm-brace", member="arm", part="member"),
        ref("member-head-chassis", "head-tray", member="head", part="member"),
        revolve("joint-base-weld", "pole", STEEL, loop([(0, K.POLE_R0 - 0.001), (0.016, K.POLE_R0 - 0.001), (0, K.POLE_R0 + 0.016)]), U, tone=0.3, member="pole", part="joint"),
        ref("joint-arm-clamp", "arm-clamp", member="arm", part="joint"),
        revolve("joint-head-spigot", "head", ALU, loop([(-0.03, K.ARM_R - 0.001), (0.10, K.ARM_R - 0.001), (0.10, 0.038), (-0.03, 0.038)]), U, tone=0.35, member="head", part="joint"),
        support(0.015),
    ] + masses(0.03, 0.05, 0.016, 0.022)
    decisions = [
        {"en": "Body at 1:100 (feature 15 mm): plinth and base plate are one 0.12 m block (nuts and bolt ends under the feature are omitted), the shroud is a solid flare on four stations, the column a solid cone with its dome merged (the 4 mm cap step and the flush door are below the feature), the clamp one sleeve without its ear, socket merged into the 60 mm arm, the strut slender as drawn (stroke), brace band and gusset lug omitted (8 and 12 mm), the luminaire as cover and tray lofts on nine stations without fins and spigot collar, the lens a texture plate.", "de": "Körper 1:100 (Merkmal 15 mm): Sockel und Fußplatte sind ein Block von 0,12 m (Muttern und Schraubenenden unter dem Merkmal entfallen), die Manschette ein voller Kragen auf vier Stationen, der Mast ein voller Kegel mit eingeschmolzener Kuppel (Kappenstufe 4 mm und bündige Tür unter dem Merkmal), die Klemme eine Hülse ohne Lasche, die Muffe im Rohr von 60 mm aufgegangen, die Strebe schlank wie gezeichnet (Strich), Strebenring und Knotenblech entfallen (8 und 12 mm), die Leuchte als Haube und Wanne auf neun Stationen ohne Rippen und Zapfenkragen, die Abdeckung eine Texturplatte."},
        {"en": "Thermal at 1:100: every region references its body solids (the base plate's galvanised faces are merged into the plinth block); the lens with its inner face as section face; the plume outlet a curved plate 0.20 x 0.22 m on the crown of the cover (no fins at this scale).", "de": "Thermik 1:100: jede Region verweist auf ihre Körper (die verzinkten Flächen der Fußplatte gehen im Sockelblock auf); die Abdeckung mit ihrer Innenseite als Schnittfläche; der Fahnenaustritt eine gebogene Platte 0,20 x 0,22 m auf dem Scheitel der Haube (keine Rippen in diesem Maßstab)."},
        {"en": "Structure at 1:100: foundation block, the 4 mm conical tube and the 60 x 3 mm arm tube kept hollow (their walls are real, C30), the strut slender, the tray as head chassis; joints at the base weld, the clamp sleeve and the spigot clamp; anchor bolts and brace lug are merged (below the feature); fixed support and four masses.", "de": "Tragwerk 1:100: Fundamentblock, das 4 mm starke Kegelrohr und das Auslegerrohr 60 x 3 mm bleiben hohl (echte Wände, C30), die Strebe schlank, die Wanne als Kopfrahmen; Verbindungen an Fußnaht, Klemmhülse und Zapfenklemme; Ankerschrauben und Knotenblech gehen auf (unter dem Merkmal); festes Auflager und vier Massen."},
    ]
    return body, thermal, structure, decisions


def s200():
    """🔭 1:200 (feature 30 mm): plinth, column, arm, strut, one housing loft, lens texture."""
    rows = [K.HEAD[i] for i in SUB_200]
    body = [
        box("plinth", "foundation", CONC, (0, 0.06, 0), (0.30, 0.12, 0.30)),
        revolve("pole", "pole", STEEL, [(0, 0), (0, K.POLE_R0), (5.865, R(5.865)), (5.88, 0.026), (5.88, 0)], U),
        revolve("arm", "arm", ALU, [(0.0, 0), (0.0, K.ARM_R), (K.ARM_L, K.ARM_R), (K.ARM_L, 0)], U),
        capsule("arm-brace", "arm", ALU, BRACE_P0, BRACE_P1, K.BRACE_R, tone=0.45, slender=True),
        loft("head-housing", "head", ALU, K.loft(K.single_rows(rows), 3.4)),
        extrude("lens", "head", PC, "zx", [(-K.LENS_Z, K.LENS_U0), (K.LENS_Z, K.LENS_U0), (K.LENS_Z, K.LENS_U1), (-K.LENS_Z, K.LENS_U1)], -0.054, -0.047, role="glazing", texture=True),
    ]
    thermal = [
        ref("th-plinth", "plinth", region="plinth-concrete"),
        ref("th-pole", "pole", region="pole-galv"),
        ref("th-arm", "arm", region="arm-alu"),
        ref("th-arm-brace", "arm-brace", region="arm-alu"),
        ref("th-head-housing", "head-housing", region="lamp-housing"),
        ref("th-lens", "lens", region="lamp-lens", sectionFaces=["+v"]),
        plume([K.single_rows([(0, *K.head_at(K.PLUME_U + d, rows))])[0][1:] for d in (-0.10, -0.05, 0.0, 0.05, 0.10)], 3.4, 0.0, 0.10, 0.11, 0.004),
    ]
    structure = [
        foundation("member-foundation", 0.03, 0.12),
        revolve("member-pole", "pole", STEEL, loop([(0, K.POLE_R0 - K.POLE_WALL), (5.865, R(5.865) - K.POLE_WALL), (5.865, R(5.865)), (0, K.POLE_R0)]), U, member="pole", part="member"),
        revolve("member-arm", "arm", ALU, loop([(0.04, K.ARM_R - K.ARM_WALL), (K.ARM_L, K.ARM_R - K.ARM_WALL), (K.ARM_L, K.ARM_R), (0.04, K.ARM_R)]), U, member="arm", part="member"),
        ref("member-arm-brace", "arm-brace", member="arm", part="member"),
        loft("member-head-chassis", "head", ALU, K.loft([(u, -0.031, w, 0.019) for u, w, t, b in [K.HEAD[i] for i in (1, 3, 6, 9)]], 6.0), member="head", part="member"),
        revolve("joint-base-weld", "pole", STEEL, [(0, 0), (0, 0.11), (0.03, 0.11), (0.03, 0)], U, tone=0.3, member="pole", part="joint"),
        revolve("joint-arm-clamp", "arm", ALU, [(-0.05, 0), (-0.05, K.CLAMP_R), (0.05, K.CLAMP_R), (0.05, 0)], V, tone=0.3, member="arm", part="joint"),
        revolve("joint-head-spigot", "head", ALU, [(-0.03, 0), (-0.03, 0.038), (0.10, 0.038), (0.10, 0)], U, tone=0.35, member="head", part="joint"),
        support(0.03),
    ] + masses(0.03, 0.05, 0.02, 0.025)
    decisions = [
        {"en": "Body at 1:200 (feature 30 mm): the plinth block with the plate, the column one solid cone 170 to 80 mm with a rounded top (shroud 35 mm and clamp sleeve 9 mm proud are merged into it), the 60 mm arm from the column axis, the strut slender as drawn, the luminaire one superelliptic loft on six stations (cover and tray merged, flat underside kept by the exponent), the lens a texture plate.", "de": "Körper 1:200 (Merkmal 30 mm): der Sockelblock mit der Platte, der Mast ein voller Kegel 170 auf 80 mm mit gerundetem Kopf (Manschette 35 mm und Klemmhülse 9 mm Überstand gehen in ihm auf), der Ausleger 60 mm ab Mastachse, die Strebe schlank wie gezeichnet, die Leuchte ein superelliptischer Loft auf sechs Stationen (Haube und Wanne vereint, die flache Unterseite durch den Exponenten), die Abdeckung eine Texturplatte."},
        {"en": "Thermal at 1:200: every region references its body solid; only the total area is compared at this scale (C15).", "de": "Thermik 1:200: jede Region verweist auf ihren Körper; in diesem Maßstab wird nur die Gesamtfläche verglichen (C15)."},
        {"en": "Structure at 1:200: foundation block, conical tube and arm tube hollow (cavities 72 to 162 mm and 54 mm exceed the feature), strut slender, the head chassis a flat ring loft inside the housing; joints as solid blocks at the base, the clamp and the spigot; fixed support and four masses.", "de": "Tragwerk 1:200: Fundamentblock, Kegelrohr und Auslegerrohr hohl (Hohlräume 72 bis 162 mm und 54 mm über dem Merkmal), Strebe schlank, der Kopfrahmen ein flacher Loft im Gehäuse; Verbindungen als volle Körper an Fuß, Klemme und Zapfen; festes Auflager und vier Massen."},
    ]
    return body, thermal, structure, decisions


def s500():
    """🛰 1:500 (feature 75 mm): plinth, cone, arm bar, luminaire on four stations."""
    body = [
        box("plinth", "foundation", CONC, (0, 0.06, 0), (0.30, 0.12, 0.30)),
        revolve("pole", "pole", STEEL, [(0, 0), (0, K.POLE_R0), (5.88, K.POLE_R1), (5.88, 0)], U),
        revolve("arm", "arm", ALU, [(0.0, 0), (0.0, 0.04), (0.80, 0.04), (0.80, 0)], U),
        loft("head", "head", ALU, K.loft(K.single_rows(HEAD_500, 0.004), 3.2)),
    ]
    thermal = [
        ref("th-plinth", "plinth", region="plinth-concrete"),
        ref("th-pole", "pole", region="pole-galv"),
        ref("th-arm", "arm", region="arm-alu"),
        ref("th-head", "head", region="lamp-housing"),
        extrude("th-lens", "head", PC, "zx", [(-K.LENS_Z, K.LENS_U0), (K.LENS_Z, K.LENS_U0), (K.LENS_Z, K.LENS_U1), (-K.LENS_Z, K.LENS_U1)], -0.054, -0.046, role="glazing", region="lamp-lens", sectionFaces=["+v"], texture=True),
        plume([K.single_rows([(0, *K.head_at(K.PLUME_U + d, HEAD_500))], 0.004)[0][1:] for d in (-0.10, 0.0, 0.10)], 3.2, 0.0, 0.10, 0.11, 0.006),
    ]
    structure = [
        ref("member-foundation", "plinth", member="foundation", part="member"),
        ref("member-pole", "pole", member="pole", part="member"),
        revolve("member-arm", "arm", ALU, [(0.0, 0), (0.0, 0.0375), (0.82, 0.0375), (0.82, 0)], U, member="arm", part="member"),
        loft("member-head-chassis", "head", ALU, K.loft([(0.03, -0.012, 0.08, 0.038), (0.25, -0.012, 0.12, 0.038), (0.45, -0.012, 0.09, 0.038)], 4.0), member="head", part="member"),
        revolve("joint-base-weld", "pole", STEEL, [(0, 0), (0, 0.11), (0.075, 0.10), (0.075, 0)], U, tone=0.3, member="pole", part="joint"),
        revolve("joint-arm-clamp", "arm", ALU, [(-0.0375, 0), (-0.0375, 0.055), (0.0375, 0.055), (0.0375, 0)], V, tone=0.3, member="arm", part="joint"),
        support(0.075, 0.33),
    ] + masses(0.0375, 0.0375, 0.0375, 0.0375)
    decisions = [
        {"en": "Body at 1:500 (feature 75 mm): the plinth block, the column one straight cone 170 to 80 mm, the bracket a 80 mm bar (the 60 mm tube widened to the feature, the strut and the clamp below what the scale shows), the luminaire one loft on four stations 0.51 x 0.26 x 0.10 m so that it still reads as a flat side-entry head and not as a box.", "de": "Körper 1:500 (Merkmal 75 mm): der Sockelblock, der Mast ein gerader Kegel 170 auf 80 mm, der Ausleger ein Stab von 80 mm (das Rohr 60 mm auf das Merkmal verbreitert, Strebe und Klemme unter dem, was der Maßstab zeigt), die Leuchte ein Loft auf vier Stationen 0,51 x 0,28 x 0,08 m, damit sie noch als flacher Kopf mit seitlicher Einführung und nicht als Kiste liest."},
        {"en": "Thermal at 1:500: the four body solids carry their regions; the lens is a thin plate under the head (texture, inner face as section face) so that the region keeps its solid; only the total is compared (C15, C21).", "de": "Thermik 1:500: die vier Körper tragen ihre Regionen; die Abdeckung ist eine dünne Platte unter dem Kopf (Textur, Innenseite als Schnittfläche), damit die Region ihren Körper behält; verglichen wird nur die Summe (C15, C21)."},
        {"en": "Structure at 1:500: the foundation block and the column are the members themselves, the arm a 75 mm bar, the head chassis a flat loft inside the housing; joints at the base and at the clamp, the strut is merged into the arm member (below the feature); the fixed support is a bearing pad of one feature around the foot of the block; four masses of one feature.", "de": "Tragwerk 1:500: Fundamentblock und Mast sind selbst die Stäbe, der Ausleger ein Stab von 75 mm, der Kopfrahmen ein flacher Loft im Gehäuse; Verbindungen am Fuß und an der Klemme, die Strebe geht im Auslegerstab auf (unter dem Merkmal); das feste Auflager ist eine Lagerplatte von einem Merkmal um den Fuß des Blocks; vier Massen von einem Merkmal."},
    ]
    return body, thermal, structure, decisions


def main():
    """🗼 Writes the volumes source."""
    out = {}
    for den, fn in (("50", s50), ("100", s100), ("200", s200), ("500", s500)):
        body, thermal, structure, decisions = fn()
        out[den] = {"status": "authored", "denominator": int(den), "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}
    path = os.path.join(HERE, "volumes", "objects", "street-lamp.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump({"volumes": out}, f, ensure_ascii=False, indent=2)
        f.write("\n")
    print(path, {k: {l: len(v[l]) for l in ("body", "thermal", "structure")} for k, v in out.items()})


if __name__ == "__main__":
    main()
