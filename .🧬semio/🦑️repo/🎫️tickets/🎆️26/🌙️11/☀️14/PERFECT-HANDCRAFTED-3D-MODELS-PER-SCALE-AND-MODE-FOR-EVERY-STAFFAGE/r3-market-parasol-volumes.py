"""🧊 Handcrafted volumes of the market parasol at 1:50, 1:100, 1:200 and 1:500: body, thermal and structure stated part by part from the real construction (ballast block, clevis foot, two-part mast, crank winch, crown hub with top cap, runner, eight ribs with tip caps and stretcher brackets, eight stretchers, eight flat gores); writes volumes/objects/market-parasol.json.

See ./r3-objects-market-parasol.md and ./design.md
"""
import importlib.util
import json
import math
from pathlib import Path

T = Path(__file__).resolve().parent


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), T / f"{name}.py")
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


k = module("r3-market-parasol-kit")
draw = module("r3-market-parasol-draw")
r6 = k.r6
ALU, CONC, CLOTH = "aluminium-anodised", "concrete", "polyester-canvas"


def solid(sid, kind, bone, material, role, tone, **geo):
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update(geo)
    return s


def tr(x, y, z):
    return {"translation": [r6(x), r6(y), r6(z)]}


def prof(points):
    return [[r6(a), r6(b)] for a, b in points]


def gore_uw(r):
    p, _ = draw.gore_profile(k, r)
    return p


def trimmed(p, a):
    """✂️ The gore polygon with its apex cut off at the distance a from the crown (the part under the hub)."""
    (o, f, n) = p
    s = a / math.hypot(*n)
    return [(a, 0.0), f, n, (n[0] * s, n[1] * s)]


def gores(v0, v1, prefix="panel", trim=None):
    return [solid(f"{prefix}-{r}", "extrude", f"rib-{r}", CLOTH, "body", 0.82, plane="zx", profile=prof([(w, u) for u, w in (gore_uw(r) if trim is None else trimmed(gore_uw(r), trim))]), **{"from": r6(v0), "to": r6(v1)}) for r in k.RIBS]


def underside(v0, v1, trim=None):
    out = []
    for g in gores(v0, v1, "th-underside", trim):
        g["region"] = "canopy-underside"
        g["layer"] = "cloth-under"
        g["sectionFaces"] = ["+v"]
        out.append(g)
    return out


def hub_profile(scale):
    ring = draw.hub_rings(0.055, 0.042, scale)[0]
    half = [(y, x) for x, y in ring if x >= -1e-9]
    half = sorted(set(half), key=lambda p: (p[0], -p[1]))
    return half


def crown_hub_50():
    pts = [(-0.05, 0.0), (-0.05, 0.042), (0.012, 0.042), (0.012, 0.055)]
    pts += [(0.012 + 0.055 * math.sin(math.radians(a)), 0.055 * math.cos(math.radians(a))) for a in (11.25, 22.5, 33.75, 45, 56.25, 67.5, 78.75)]
    pts += [(0.064, 0.0)]
    return prof(pts)


def rev(sid, bone, material, role, tone, profile, axis=(0, 1, 0), origin=(0, 0, 0)):
    return solid(sid, "revolve", bone, material, role, tone, profile=prof(profile), axis=list(axis), origin=[r6(x) for x in origin])


def clevis_profile():
    return [(-0.035, 0.215), (0.035, 0.215)] + draw.arc_pts(0, 0.26, 0.035, 0, 180, 12)[:-1] + [(-0.035, 0.26)]


def ribs_body(tip=True, slender=False, section=(0.016, 0.010)):
    out = []
    for r in k.RIBS:
        L = k.RIB_L - 0.06
        b = solid(f"rib-{r}-rod", "box", f"rib-{r}", ALU, "body", 0.35, size=[r6(L), section[0], section[1]], cornerRadius=0.002, transform=tr(0.03 + L / 2, 0, 0))
        if slender:
            b["slender"] = True
        out.append(b)
        if tip:
            out.append(solid(f"rib-{r}-tip", "capsule", f"rib-{r}", ALU, "detail", 0.2, p0=[r6(k.RIB_L - 0.032), 0, 0], p1=[r6(k.RIB_L - 0.009), 0, 0], r0=0.009, r1=0.009))
    return out


def notches():
    return [solid(f"rib-{r}-notch", "box", f"rib-{r}", ALU, "detail", 0.3, size=[0.03, 0.012, 0.016], cornerRadius=0.002, transform=tr(k.S0, -0.013, 0)) for r in k.RIBS]


def stretchers_body(radius):
    return [solid(f"stretch-{r}", "capsule", f"stretch-{r}", ALU, "body", 0.4, p0=[0.04, 0, 0], p1=[r6(k.STRETCH_L - 0.012), 0, 0], r0=radius, r1=radius, slender=True) for r in k.RIBS]


def body50():
    b = [
        solid("base-block", "box", "base", CONC, "body", 0.62, size=[0.4, 0.2, 0.3], cornerRadius=0.012, transform=tr(0, 0.1, 0)),
        solid("base-plate", "box", "base", ALU, "body", 0.35, size=[0.28, 0.015, 0.28], cornerRadius=0.003, transform=tr(0, 0.2075, 0)),
        solid("base-bolts", "revolve", "base", ALU, "detail", 0.3, profile=prof([(0, 0), (0, 0.0095), (0.008, 0.0095), (0.008, 0)]), axis=[0, 1, 0], origin=[0.11, 0.215, 0.11], repeat={"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": r6(math.pi / 2), "origin": [0, 0, 0]}),
        solid("clevis-flange", "box", "base", ALU, "detail", 0.3, size=[0.07, 0.008, 0.07], cornerRadius=0.002, transform=tr(0, 0.219, 0)),
        solid("clevis-cheek-near", "extrude", "base", ALU, "body", 0.3, plane="xy", profile=prof(clevis_profile()), **{"from": -0.036, "to": -0.028}),
        solid("clevis-cheek-far", "extrude", "base", ALU, "body", 0.3, plane="xy", profile=prof(clevis_profile()), **{"from": 0.028, "to": 0.036}),
        solid("mast-pin", "capsule", "mast", ALU, "detail", 0.3, p0=[0, 0, -0.04], p1=[0, 0, 0.04], r0=0.006, r1=0.006),
        rev("mast-pin-nut-near", "mast", ALU, "detail", 0.2, [(-0.042, 0), (-0.042, 0.011), (-0.034, 0.011), (-0.034, 0)], axis=(0, 0, 1)),
        rev("mast-pin-nut-far", "mast", ALU, "detail", 0.2, [(0.034, 0), (0.034, 0.011), (0.042, 0.011), (0.042, 0)], axis=(0, 0, 1)),
        solid("mast-lug", "extrude", "mast", ALU, "body", 0.45, plane="xy", profile=prof(draw.arc_pts(0, 0, 0.023, 90, 270, 12) + [(0.07, -0.023), (0.07, 0.023)]), **{"from": -0.023, "to": 0.023}),
        rev("mast-lower", "mast", ALU, "body", 0.45, [(0.05, 0), (0.05, 0.024), (0.79, 0.024), (0.79, 0)], axis=(1, 0, 0)),
        rev("mast-tilt-collar", "mast", ALU, "detail", 0.25, [(0.755, 0), (0.755, 0.03), (0.761, 0.034), (0.859, 0.034), (0.865, 0.03), (0.865, 0)], axis=(1, 0, 0)),
        solid("mast-collar-knob", "capsule", "mast", ALU, "detail", 0.2, p0=[0.81, 0, -0.03], p1=[0.81, 0, -0.052], r0=0.009, r1=0.009),
        rev("mast-upper", "mast", ALU, "body", 0.45, [(0.78, 0), (0.78, 0.02), (2.07, 0.02), (2.07, 0)], axis=(1, 0, 0)),
        solid("mast-winch", "box", "mast", ALU, "detail", 0.3, size=[0.09, 0.034, 0.04], cornerRadius=0.006, transform=tr(1.12, 0.033, 0)),
        solid("mast-crank", "extrude", "mast", ALU, "detail", 0.25, plane="xy", profile=prof([(1.116, 0.05), (1.124, 0.05), (1.124, 0.083), (1.078, 0.083), (1.078, 0.074), (1.116, 0.074)]), **{"from": -0.004, "to": 0.004}),
        solid("mast-crank-knob", "capsule", "mast", ALU, "detail", 0.2, p0=[1.07, 0.078, 0], p1=[1.07, 0.107, 0], r0=0.008, r1=0.008),
        rev("runner-collar", "runner", ALU, "body", 0.35, [(-0.045, 0), (-0.045, 0.032), (0.03, 0.032), (0.036, 0.036), (0.045, 0.036), (0.045, 0)]),
        rev("crown-hub", "crown", ALU, "body", 0.35, crown_hub_50()),
        rev("crown-finial", "crown", ALU, "body", 0.3, [(0.058, 0), (0.058, 0.008), (0.074, 0.009), (0.08, 0.016), (0.09, 0.0185), (0.1, 0.016), (0.108, 0.011), (0.1125, 0)]),
    ]
    b += ribs_body() + notches() + stretchers_body(0.0055) + gores(0.008, 0.016)
    return b


def thermal_refs(body, region_of):
    out = []
    for s in body:
        reg = region_of(s)
        if reg is None:
            continue
        region, layer, faces = reg
        t = {"id": f"th-{s['id']}", "of": s["id"], "region": region, "layer": layer}
        if faces:
            t["sectionFaces"] = faces
        out.append(t)
    return out


def region_of(s):
    if s["material"] == CONC:
        return ("base-concrete", "concrete", None)
    if s["material"] == CLOTH:
        return ("canopy-top", "cloth-top", ["-v"])
    return ("frame-alu", "tube", None)


def tube_ring(sid, bone, a0, a1, ro, wall):
    return rev(sid, bone, ALU, "detail", 0.15, [(a0, ro - wall), (a1, ro - wall), (a1, ro), (a0, ro), (a0, ro - wall)], axis=(1, 0, 0))


def mass(sid, bone, member, radii, at):
    s = solid(sid, "ellipsoid", bone, ALU if bone != "base" else CONC, "detail", 0.2, radii=[r6(x) for x in radii], transform=tr(*at))
    s["member"] = member
    s["part"] = "mass"
    return s


def support(thick):
    s = solid("support-socket", "box", "base", CONC, "detail", 0.1, size=[0.4, r6(thick), 0.3], transform=tr(0, thick / 2, 0))
    s.update({"member": "base", "part": "support", "anchor": "socket"})
    return s


def ref(sid, of, member, part):
    return {"id": sid, "of": of, "member": member, "part": part}


def own(s, member, part):
    s = dict(s)
    s.update({"member": member, "part": part})
    return s


def structure50():
    st = [
        ref("member-base", "base-block", "base", "member"),
        mass("mass-base", "base", "base", (0.06, 0.04, 0.05), (0, 0.1, 0)),
        support(0.0075),
        ref("joint-pivot", "mast-pin", "mast", "joint"),
        own(tube_ring("member-mast-lower", "mast", 0.05, 0.79, 0.024, 0.002), "mast", "member"),
        own(tube_ring("member-mast-upper", "mast", 0.78, 2.07, 0.020, 0.0015), "mast", "member"),
        ref("joint-coupling", "mast-tilt-collar", "mast", "joint"),
        mass("mass-mast", "mast", "mast", (0.03, 0.016, 0.016), (k.MAST_L / 2, 0, 0)),
        ref("joint-crown", "crown-hub", "crown", "joint"),
        ref("joint-runner", "runner-collar", "runner", "joint"),
        mass("mass-runner", "runner", "runner", (0.02, 0.02, 0.02), (0, 0, 0)),
    ]
    for r in k.RIBS:
        st.append(ref(f"member-rib-{r}", f"rib-{r}-rod", f"rib-{r}", "member"))
        st.append(ref(f"member-rib-{r}-tip", f"rib-{r}-tip", f"rib-{r}", "member"))
        st.append(ref(f"joint-rib-{r}-stretcher", f"rib-{r}-notch", f"rib-{r}", "joint"))
        st.append(mass(f"mass-rib-{r}", f"rib-{r}", f"rib-{r}", (0.02, 0.007, 0.006), (k.RIB_L / 2, 0, 0)))
        st.append(ref(f"member-stretch-{r}", f"stretch-{r}", f"stretch-{r}", "member"))
    return st


def body100():
    b = [
        solid("base-block", "box", "base", CONC, "body", 0.62, size=[0.4, 0.2, 0.3], cornerRadius=0.01, transform=tr(0, 0.1, 0)),
        solid("base-plate", "box", "base", ALU, "body", 0.35, size=[0.28, 0.015, 0.28], transform=tr(0, 0.2075, 0)),
        solid("base-socket", "extrude", "base", ALU, "body", 0.3, plane="xy", profile=prof(clevis_profile()), **{"from": -0.042, "to": 0.042}),
        rev("mast", "mast", ALU, "body", 0.45, [(-0.024, 0), (-0.024, 0.022), (2.07, 0.022), (2.07, 0)], axis=(1, 0, 0)),
        rev("mast-tilt-collar", "mast", ALU, "detail", 0.25, [(0.755, 0), (0.755, 0.034), (0.865, 0.034), (0.865, 0)], axis=(1, 0, 0)),
        solid("mast-winch", "extrude", "mast", ALU, "detail", 0.3, plane="xy", profile=prof([(1.075, 0.016), (1.165, 0.016), (1.165, 0.05), (1.128, 0.05), (1.128, 0.086), (1.06, 0.086), (1.06, 0.115), (1.044, 0.115), (1.044, 0.07), (1.112, 0.07), (1.112, 0.05), (1.075, 0.05)]), **{"from": -0.012, "to": 0.012}),
        rev("runner-collar", "runner", ALU, "body", 0.35, [(-0.045, 0), (-0.045, 0.036), (0.045, 0.036), (0.045, 0)]),
        rev("crown-hub", "crown", ALU, "body", 0.35, [(-0.05, 0), (-0.05, 0.042), (0.012, 0.042), (0.012, 0.055), (0.05, 0.033), (0.064, 0.012), (0.1125, 0.012), (0.1125, 0)]),
    ]
    b += ribs_body(tip=False, slender=True) + stretchers_body(0.006) + gores(0.008, 0.023)
    return b


def structure100():
    st = [
        ref("member-base", "base-block", "base", "member"),
        mass("mass-base", "base", "base", (0.07, 0.05, 0.06), (0, 0.1, 0)),
        support(0.015),
        solid("joint-pivot", "capsule", "mast", ALU, "detail", 0.2, p0=[0, 0, -0.04], p1=[0, 0, 0.04], r0=0.008, r1=0.008, member="mast", part="joint"),
        own(tube_ring("member-mast", "mast", -0.024, 2.07, 0.022, 0.002), "mast", "member"),
        mass("mass-mast", "mast", "mast", (0.04, 0.016, 0.016), (k.MAST_L / 2, 0, 0)),
        ref("joint-crown", "crown-hub", "crown", "joint"),
        ref("joint-runner", "runner-collar", "runner", "joint"),
    ]
    for r in k.RIBS:
        st.append(ref(f"member-rib-{r}", f"rib-{r}-rod", f"rib-{r}", "member"))
        st.append(solid(f"joint-rib-{r}-stretcher", "ellipsoid", f"rib-{r}", ALU, "detail", 0.2, radii=[0.0125, 0.0125, 0.0125], transform=tr(k.S0, -0.004, 0), member=f"rib-{r}", part="joint"))
        st.append(ref(f"member-stretch-{r}", f"stretch-{r}", f"stretch-{r}", "member"))
    return st


def body200():
    b = [
        solid("base-block", "box", "base", CONC, "body", 0.62, size=[0.4, 0.2, 0.3], transform=tr(0, 0.1, 0)),
        solid("base-socket", "box", "base", ALU, "body", 0.3, size=[0.08, 0.1, 0.084], transform=tr(0, 0.25, 0)),
        rev("mast", "mast", ALU, "body", 0.45, [(0.03, 0), (0.03, 0.022), (2.07, 0.022), (2.07, 0)], axis=(1, 0, 0)),
        rev("runner-collar", "runner", ALU, "body", 0.35, [(-0.045, 0), (-0.045, 0.036), (0.045, 0.036), (0.045, 0)]),
        rev("crown-hub", "crown", ALU, "body", 0.35, [(-0.05, 0), (-0.05, 0.045), (0.012, 0.045), (0.06, 0.03), (0.1125, 0.015), (0.1125, 0)]),
    ]
    return b + gores(-0.008, 0.022)


def slender_ribs(radius):
    return [solid(f"rib-{r}-rod", "capsule", f"rib-{r}", ALU, "detail", 0.35, p0=[0.04, 0, 0], p1=[r6(k.RIB_L - 0.01), 0, 0], r0=radius, r1=radius, slender=True) for r in k.RIBS]


def box_bar(sid, bone, length, side, **extra):
    return solid(sid, "box", bone, ALU, "detail", 0.35, size=[r6(length), side, side], transform=tr(0.04 + length / 2, 0, 0), slender=True, **extra)


def structure_500():
    st = [
        ref("member-base", "base-block", "base", "member"),
        mass("mass-base", "base", "base", (0.075, 0.05, 0.06), (0, 0.1, 0)),
        support(0.075),
        ref("member-mast", "mast", "mast", "member"),
        ref("joint-crown", "crown-hub", "crown", "joint"),
    ]
    for r in k.RIBS:
        st.append(box_bar(f"member-rib-{r}", f"rib-{r}", k.RIB_L - 0.05, 0.016, member=f"rib-{r}", part="member"))
        st.append(box_bar(f"member-stretch-{r}", f"stretch-{r}", k.STRETCH_L - 0.052, 0.012, member=f"stretch-{r}", part="member"))
    return st


def structure_coarse(feature, body_ids):
    st = [
        ref("member-base", "base-block", "base", "member"),
        mass("mass-base", "base", "base", (feature, feature * 0.7, feature * 0.8), (0, 0.1, 0)),
        support(feature),
        ref("member-mast", "mast", "mast", "member"),
    ]
    if "crown-hub" in body_ids:
        st.append(ref("joint-crown", "crown-hub", "crown", "joint"))
    if "runner-collar" in body_ids:
        st.append(ref("joint-runner", "runner-collar", "runner", "joint"))
    for s in slender_ribs(0.008):
        st.append(own(dict(s, id=f"member-{s['bone']}"), s["bone"], "member"))
    for r in k.RIBS:
        st.append(solid(f"member-stretch-{r}", "capsule", f"stretch-{r}", ALU, "detail", 0.4, p0=[0.04, 0, 0], p1=[r6(k.STRETCH_L - 0.012), 0, 0], r0=0.006, r1=0.006, slender=True, member=f"stretch-{r}", part="member"))
    return st


def thermal_slender():
    out = []
    for s in slender_ribs(0.008):
        out.append(dict(s, id=f"th-{s['id']}", region="frame-alu", layer="tube"))
    for r in k.RIBS:
        out.append(solid(f"th-stretch-{r}", "capsule", f"stretch-{r}", ALU, "detail", 0.4, p0=[0.04, 0, 0], p1=[r6(k.STRETCH_L - 0.012), 0, 0], r0=0.006, r1=0.006, slender=True, region="frame-alu", layer="tube"))
    return out


def body500():
    return [
        solid("base-block", "box", "base", CONC, "body", 0.62, size=[0.4, 0.2, 0.3], transform=tr(0, 0.1, 0)),
        dict(rev("mast", "mast", ALU, "body", 0.45, [(-0.06, 0), (-0.06, 0.022), (k.MAST_L - 0.03, 0.022), (k.MAST_L - 0.03, 0)], axis=(1, 0, 0)), slender=True),
        rev("crown-hub", "crown", ALU, "body", 0.35, [(-0.05, 0), (-0.05, 0.045), (0.06, 0.045), (0.1125, 0.02), (0.1125, 0)]),
    ] + gores(-0.03, 0.045)


def t(en, de):
    return {"en": en, "de": de}


DECISIONS = {
    "50": [
        t("Own 1:50 model of the real market parasol in the open-still pose: concrete ballast block with 12 mm rounded edges, aluminium cover plate with four bolts, clevis bracket (flange, two cheeks) and the mast foot lug turning on a 12 mm pin with nuts 60 mm above the plate; lower mast tube 48 mm, coupling collar 68 mm with clamp knob, upper tube 40 mm, crank winch with housing, bent arm and handle at 1.38 m; runner sleeve with the stretcher notch ring; crown hub with domed top cap and finial; eight 16 x 10 mm ribs 1.37 m long at 45 degrees with tip caps and stretcher brackets; eight 11 mm stretchers; eight flat 8 mm gores, each on its rib between the rib and the next rib, so the canopy is the taut octagonal pyramid of a real market parasol with ridges over the ribs.", "Eigenes 1:50-Modell des realen Marktschirms in der Pose open-still: Betonballastblock mit 12 mm gerundeten Kanten, Aluminium-Deckplatte mit vier Schrauben, Gabelbock (Flansch, zwei Wangen) und die Mastfußlasche auf einem 12-mm-Bolzen mit Muttern 60 mm über der Platte; Unterrohr 48 mm, Kupplungsmuffe 68 mm mit Klemmknopf, Oberrohr 40 mm, Kurbelwinde mit Gehäuse, gekröpftem Arm und Griff auf 1,38 m; Läuferhülse mit Spreizenkranz; Krone mit gewölbter Kappe und Knauf; acht Speichen 16 x 10 mm, 1,37 m lang im 45-Grad-Abstand mit Endkappen und Spreizenlaschen; acht Spreizen 11 mm; acht ebene 8-mm-Bahnen, jede auf ihrer Speiche zwischen dieser und der nächsten Speiche, sodass der Schirm die gespannte achteckige Pyramide eines echten Marktschirms mit Graten über den Speichen ist."),
        t("Thermal: base-concrete is the block; frame-alu every aluminium part by reference; canopy-top the eight gores with their underside declared a section face; the hidden canopy-underside region is a 0.5 mm cloth shell under each gore with its top declared a section face (one sheet, two sides).", "Thermik: base-concrete ist der Block; frame-alu jedes Aluminiumteil als Verweis; canopy-top die acht Bahnen mit als Schnittfläche erklärter Unterseite; die verdeckte Region canopy-underside ist eine 0,5-mm-Stoffschale unter jeder Bahn mit als Schnittfläche erklärter Oberseite (ein Blatt, zwei Seiten)."),
        t("Structure: the load path of the real frame: the ballast block as member with its mass, the fixed support under it, the pivot pin, the lower and upper mast as hollow tubes (48 x 2 and 40 x 1.5 mm) with the coupling collar as splice joint, crown hub and runner as joints, the eight ribs and eight stretchers as members with the rib brackets as their pin joints, and the masses of mast, runner and every rib at their centres.", "Tragwerk: der Lastpfad des realen Gestells: der Ballastblock als Stab mit seiner Masse, das feste Auflager darunter, der Gelenkbolzen, Unter- und Oberrohr als Hohlrohre (48 x 2 und 40 x 1,5 mm) mit der Kupplungsmuffe als Stoßgelenk, Krone und Läufer als Gelenke, die acht Speichen und acht Spreizen als Stäbe mit den Speichenlaschen als ihren Bolzengelenken, und die Massen von Mast, Läufer und jeder Speiche in ihren Schwerpunkten."),
    ],
    "100": [
        t("Own 1:100 model: block, cover plate, the clevis as one 84 mm block with the pin, one 44 mm mast tube (the 4 mm step between 48 and 40 mm is below the 15 mm feature), coupling collar, winch housing with crank as one bent part, plain runner sleeve, crown hub merged with top cap and finial, eight ribs and eight stretchers as true-size slender bars under the canopy, eight flat 15 mm gores; bolts, clamp knob, tip caps and stretcher brackets merged away.", "Eigenes 1:100-Modell: Block, Deckplatte, der Gabelbock als ein 84-mm-Block mit Bolzen, ein 44-mm-Mastrohr (der 4-mm-Absatz zwischen 48 und 40 mm liegt unter dem Mindestmaß 15 mm), Kupplungsmuffe, Windengehäuse mit Kurbel als ein gekröpftes Teil, glatte Läuferhülse, Krone mit Kappe und Knauf verschmolzen, acht Speichen und acht Spreizen als maßhaltige schlanke Stäbe unter dem Schirm, acht ebene 15-mm-Bahnen; Schrauben, Klemmknopf, Endkappen und Spreizenlaschen entfallen."),
        t("Thermal by reference to the body (gores with section underside, block, aluminium parts) plus a 0.5 mm underside shell per gore; structure: block with mass and support, pivot pin, one 44 x 2 mm mast tube with its mass, crown and runner joints, ribs and stretchers as members with a pin joint at every rib bracket.", "Thermik als Verweis auf den Körper (Bahnen mit Schnitt-Unterseite, Block, Aluminiumteile) plus eine 0,5-mm-Unterseitenschale je Bahn; Tragwerk: Block mit Masse und Auflager, Gelenkbolzen, ein Mastrohr 44 x 2 mm mit seiner Masse, Kronen- und Läufergelenk, Speichen und Spreizen als Stäbe mit einem Bolzengelenk an jeder Speichenlasche."),
    ],
    "200": [
        t("Own 1:200 model: block, the clevis merged with the plate into one foot block, one 44 mm mast, runner sleeve, crown hub with cap and finial as one tapering revolve, eight flat 30 mm gores that also take in the ribs below them; crank, collar, stretchers and tip caps are below the 30 mm feature and merged away.", "Eigenes 1:200-Modell: Block, Gabelbock und Platte zu einem Fußblock verschmolzen, ein 44-mm-Mast, Läuferhülse, Krone mit Kappe und Knauf als ein sich verjüngender Drehkörper, acht ebene 30-mm-Bahnen, die auch die Speichen darunter aufnehmen; Kurbel, Muffe, Spreizen und Endkappen liegen unter dem Mindestmaß 30 mm und entfallen."),
        t("Thermal by reference plus underside shells, ribs and stretchers kept as slender aluminium bars in the thermal and structural lists only; structure: block with mass and support, mast, crown and runner joints, ribs and stretchers as slender members.", "Thermik als Verweis plus Unterseitenschalen, Speichen und Spreizen nur in Thermik und Tragwerk als schlanke Aluminiumstäbe; Tragwerk: Block mit Masse und Auflager, Mast, Kronen- und Läufergelenk, Speichen und Spreizen als schlanke Stäbe."),
    ],
    "500": [
        t("Own 1:500 model: the ballast block, the mast as a true-size slender bar, the crown hub as one block, and the canopy as eight flat 75 mm gores forming the octagonal pyramid; everything else is below the 75 mm feature.", "Eigenes 1:500-Modell: der Ballastblock, der Mast als maßhaltiger schlanker Stab, die Krone als ein Block und der Schirm als acht ebene 75-mm-Bahnen, die die achteckige Pyramide bilden; alles andere liegt unter dem Mindestmaß 75 mm."),
        t("Thermal by reference plus underside shells; structure: block with mass and support, mast, crown joint, ribs and stretchers as slender members.", "Thermik als Verweis plus Unterseitenschalen; Tragwerk: Block mit Masse und Auflager, Mast, Kronengelenk, Speichen und Spreizen als schlanke Stäbe."),
    ],
}


def scale(den, body, structure, extra_thermal=()):
    th = thermal_refs(body, region_of)
    v = (0.0075, 0.008) if den in (50, 100) else ((-0.0085, -0.008) if den == 200 else (-0.0305, -0.03))
    th += underside(*v, trim=0.045 if den == 100 else None)
    th += list(extra_thermal)
    return {"status": "authored", "denominator": den, "body": body, "thermal": th, "structure": structure, "decisions": DECISIONS[str(den)]}


def main():
    b50, b100, b200, b500 = body50(), body100(), body200(), body500()
    vols = {
        "50": scale(50, b50, structure50()),
        "100": scale(100, b100, structure100()),
        "200": scale(200, b200, structure_coarse(0.03, {s["id"] for s in b200}), thermal_slender()),
        "500": scale(500, b500, structure_500()),
    }
    k.SOURCE.write_text(json.dumps({"volumes": vols}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    for s, v in vols.items():
        print(s, len(v["body"]), len(v["thermal"]), len(v["structure"]))


if __name__ == "__main__":
    main()
