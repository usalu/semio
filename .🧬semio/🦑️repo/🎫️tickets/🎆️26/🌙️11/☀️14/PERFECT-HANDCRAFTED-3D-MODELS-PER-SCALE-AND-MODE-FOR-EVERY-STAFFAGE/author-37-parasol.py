from author_37_lib import *

CN = Ctx("base", "concrete", None, "body", 0.6)
AL = Ctx("mast", "aluminium-anodised", None, "body", 0.6)
CF = Ctx("crown", "polyester-canvas", None, "body", 0.8)
ST = Ctx("base", "concrete", None, "detail", 0.15)
RIBS = ["rib-right", "rib-b", "rib-c", "rib-d", "rib-left", "rib-f", "rib-g", "rib-h"]
D = math.radians(15.2803)
SD, CD = math.sin(D), math.cos(D)
RL = 1.237426


def arch(n):
    P = [(0, 0), (0.3465, 0.1150), (0.8909, 0.1035), (1.2374, 0)]
    pts = bezier(P[0], P[1], P[2], P[3], n)
    return [(u * CD + v * SD, -u * SD + v * CD) for u, v in pts]


def canopy(sid, n, t, skirt=True, ctx=CF, drop=0.0):
    a = arch(n)
    out = [[dy - drop, r] for r, dy in a]
    if skirt:
        out.append([a[-1][1] - drop, 1.2937])
    top = out
    bot = [[y - t, r] for y, r in top]
    return ring(sid, ctx, top + bot[::-1], "v")


def underside(n, t):
    s = canopy("th-canopy-underside", n, t, True, CF(region="canopy-underside", layer="cloth-under", bone="crown"), drop=t + 0.0006)
    s["sectionFaces"] = ["+v"]
    return s


def rib(rid, r0, r1, bone):
    return cap(rid + "-rod", AL(bone=bone), (0, 0, 0), (RL, 0, 0), r0, r1)


def hollow_mast(sid, t, inset, n=3):
    pts = [[0, 0.026 - inset], [2.15, 0.021 - inset]]
    o = [[u, r] for u, r in pts]
    i = [[u, r - t] for u, r in pts]
    return ring(sid, ST(bone="mast", member="mast", part="member"), o + i[::-1], "u")


def thermal_refs(frame_ids, under):
    t = [th("th-base-block", "base-block", "base-concrete", "concrete"),
         tag(th("th-canopy-top", "canopy", "canopy-top", "cloth-top"), sectionFaces=["-v"]),
         under]
    t += [th("th-" + i, i, "frame-alu", "tube") for i in frame_ids]
    return t


def mem(sid, of, member, part="member"):
    return stm(sid, of, member, part)


def basepad(h):
    return tag(box("support-socket-pad", ST(member="base", part="support", tone=0.1), (-0.2, 0.0, -0.15), (0.2, h, 0.15)), tags=["socket"])


def mass_base(r=1.0):
    return ell("mass-base", ST(member="base", part="mass", tone=0.2), (0, 0.1, 0), (0.15 * r, 0.08 * r, 0.1 * r))


# ------------------------------------------------------------ 1:50
body50 = [
    box("base-block", CN, (-0.2, 0, -0.15), (0.2, 0.2, 0.15), 0.01),
    box("base-plate", AL(bone="base"), (-0.14, 0.2, -0.14), (0.14, 0.215, 0.14), 0.002),
    rev("base-socket", AL(bone="base"), [[0.215, 0], [0.215, 0.045], [0.3, 0.035], [0.3, 0]], "v"),
    cap("mast", AL, (0, 0, 0), (2.15, 0, 0), 0.026, 0.021),
    ell("mast-knuckle", AL(role="detail"), (0, 0, 0), (0.04, 0.04, 0.04)),
    cyl("mast-tilt-collar", AL(role="detail"), "u", (0.875, 0, 0), 0.11, 0.034),
    cap("mast-crank-a", AL(role="detail"), (1.18, 0.021, 0), (1.18, 0.075, 0), 0.004),
    cap("mast-crank-b", AL(role="detail"), (1.18, 0.075, 0), (1.14, 0.095, 0), 0.004),
    ell("mast-crank-knob", AL(role="detail"), (1.135, 0.097, 0), (0.012, 0.012, 0.012)),
    rev("crown-hub", AL(bone="crown"), [[-0.06, 0], [-0.06, 0.04], [-0.04, 0.045], [0.06, 0.045], [0.08, 0.04], [0.08, 0]], "v"),
    rev("crown-finial", AL(bone="crown", role="detail"), [[0.08, 0], [0.08, 0.012], [0.1, 0.012], [0.1075, 0.009], [0.115, 0]], "v"),
    rev("runner-collar", AL(bone="runner"), [[-0.04, 0], [-0.04, 0.035], [0.04, 0.035], [0.04, 0]], "v"),
    tag(cap("stretch-right", AL(bone="stretch-right"), (0, 0, 0), (0.55, 0, 0), 0.006), slender=True),
    tag(cap("stretch-left", AL(bone="stretch-left"), (0, 0, 0), (0.55, 0, 0), 0.006), slender=True),
]
for rid in RIBS:
    body50.append(rib(rid, 0.011, 0.007, rid))
    body50.append(ell(rid + "-tip", AL(bone=rid, role="detail"), (RL, 0, 0), (0.013, 0.013, 0.013)))
body50.append(canopy("canopy", 16, 0.0005))
alu50 = [b["id"] for b in body50 if b["id"] != "canopy" and b["id"] != "base-block"]
th50 = thermal_refs(alu50, underside(16, 0.0005))
st50 = [mem("member-base", "base-block", "base"), hollow_mast("member-mast", 0.002, 0.0015), mem("member-stretch-right", "stretch-right", "stretch-right"), mem("member-stretch-left", "stretch-left", "stretch-left")]
st50 += [mem("member-" + r, r + "-rod", r) for r in RIBS]
st50 += [mem("joint-knuckle", "mast-knuckle", "mast", "joint"), mem("joint-crown-hub", "crown-hub", "crown", "joint"), mem("joint-runner", "runner-collar", "runner", "joint"), mem("joint-collar", "mast-tilt-collar", "mast", "joint"), mass_base(), basepad(0.0075)]
D50 = [
    text("Own 1:50 model of the market parasol in the open-still pose: concrete block 0.4 x 0.3 x 0.2 m with 10 mm edge radius, aluminium plate and tapered socket cone, the 2.15 m tapering mast tube (52 to 42 mm) with the hinge ball, the tilt collar and the bent crank with its knob, the crown hub with finial, the sliding runner collar with two 12 mm stretchers, eight 1.24 m tapering ribs (22 to 14 mm) with ball tips that follow the bone poses (the rig lays them at the foreshortened azimuths the elevation needs), and the canopy as a 0.5 mm thin shell of revolution of the drawn dome (the 2D declares it revolved about the mast) with the flat 0.1 m hem skirt. The fabric lies above the straight ribs as drawn; sag between ribs is not modelled because the rig's rib azimuths are not equally spaced, so panel seams would not follow the ribs.",
         "Eigenes 1:50-Modell des Marktschirms in der Pose open-still: Betonblock 0,4 x 0,3 x 0,2 m mit 10 mm Kantenradius, Aluminiumplatte und verjüngter Hülsenkegel, das 2,15 m lange verjüngte Mastrohr (52 auf 42 mm) mit Gelenkkugel, Neigungskragen und gebogener Kurbel mit Knauf, die Schirmkrone mit Spitze, der Läuferring mit zwei 12-mm-Spreizen, acht 1,24 m lange verjüngte Speichen (22 auf 14 mm) mit Kugelspitzen, die den Knochenposen folgen (die Rig legt sie in die verkürzten Azimute, die der Aufriss braucht), und der Schirm als 0,5 mm dünne Rotationsschale der gezeichneten Wölbung (das 2D erklärt sie als um den Mast gedreht) mit dem flachen 0,1 m breiten Saumrand. Der Stoff liegt wie gezeichnet über den geraden Speichen; ein Durchhang zwischen den Speichen ist nicht modelliert, weil die Speichenazimute der Rig nicht gleich verteilt sind und Bahnnähte den Speichen nicht folgen würden."),
    text("Thermal by reference: the canopy-top region is the shell with its underside declared a section face, the hidden canopy-underside region the same shell with the top declared a section face (one fabric sheet, two sides), frame-alu all aluminium parts, base-concrete the block (its bottom is the footing contact and counts no area).",
         "Thermik als Verweis: die Region canopy-top ist die Schale mit als Schnittfläche erklärter Unterseite, die verdeckte Region canopy-underside dieselbe Schale mit erklärter Oberseite (ein Stoffblatt, zwei Seiten), frame-alu alle Aluminiumteile, base-concrete der Block (seine Unterseite ist der Fundamentkontakt und zählt keine Fläche)."),
    text("Structure: the concrete block (57.6 kg) as member with a compact mass ellipsoid, the mast as a real 2 mm hollow tube 1.5 mm inside the skin, the eight ribs and the two stretchers as members, hinge ball, crown hub, runner and tilt collar as joints, and the 0.4 x 0.3 m fixed block under the base as the support of the anchor socket.",
         "Tragwerk: der Betonblock (57,6 kg) als Stab mit kompaktem Massenellipsoid, der Mast als echtes 2-mm-Hohlrohr 1,5 mm innerhalb der Haut, die acht Speichen und die zwei Spreizen als Stäbe, Gelenkkugel, Krone, Läufer und Neigungskragen als Gelenke, und der Festblock 0,4 x 0,3 m unter dem Fuß als Auflager des Ankers socket."),
]

# ------------------------------------------------------------ 1:100
body100 = [
    box("base-block", CN, (-0.2, 0, -0.15), (0.2, 0.2, 0.15), 0.01),
    box("base-plate", AL(bone="base"), (-0.14, 0.2, -0.14), (0.14, 0.215, 0.14)),
    rev("base-socket", AL(bone="base"), [[0.215, 0], [0.215, 0.045], [0.3, 0.035], [0.3, 0]], "v"),
    cap("mast", AL, (0, 0, 0), (2.15, 0, 0), 0.026, 0.021),
    rev("crown-hub", AL(bone="crown"), [[-0.045, 0], [-0.045, 0.07], [0.05, 0.07], [0.05, 0.0125], [0.111, 0.0125], [0.111, 0]], "v"),
    rev("runner-collar", AL(bone="runner"), [[-0.04, 0], [-0.04, 0.035], [0.04, 0.035], [0.04, 0]], "v"),
    tag(cap("stretch-right", AL(bone="stretch-right"), (0, 0, 0), (0.55, 0, 0), 0.0075), slender=True),
    tag(cap("stretch-left", AL(bone="stretch-left"), (0, 0, 0), (0.55, 0, 0), 0.0075), slender=True),
]
for rid in RIBS:
    body100.append(rib(rid, 0.009, 0.009, rid))
body100.append(canopy("canopy", 10, 0.001))
alu100 = [b["id"] for b in body100 if b["id"] not in ("canopy", "base-block")]
th100 = thermal_refs(alu100, underside(10, 0.001))
st100 = [mem("member-base", "base-block", "base"), mem("member-mast", "mast", "mast"), mem("member-stretch-right", "stretch-right", "stretch-right"), mem("member-stretch-left", "stretch-left", "stretch-left")]
st100 += [mem("member-" + r, r + "-rod", r) for r in RIBS]
st100 += [mem("joint-crown-hub", "crown-hub", "crown", "joint"), mem("joint-runner", "runner-collar", "runner", "joint"), mass_base(), basepad(0.015)]
D100 = [
    text("Own 1:100 model against the 1:100 drawings: block, one socket cone, the mast as a 50 to 42 mm tapering capsule, a 0.095 m crown hub with finial, the runner collar and two 15 mm stretchers, eight 18 mm ribs, the plate again (15 mm) and the canopy as a 1 mm shell of revolution (ten arch segments) with its hem skirt; plate, hinge ball, tilt collar, crank, finial and the rib ball tips are merged into the mast and ribs (all below 15 mm or 2 x 15 mm).",
         "Eigenes 1:100-Modell gegen die 1:100-Zeichnungen: Block, ein Hülsenkegel, der Mast als 50 auf 42 mm verjüngte Kapsel, eine Krone von 0,095 m mit Spitze, der Läuferring und zwei 15-mm-Spreizen, acht 18-mm-Speichen, die Platte (15 mm) und der Schirm als 1-mm-Rotationsschale (zehn Bogensegmente) mit Saumrand; Platte, Gelenkkugel, Neigungskragen, Kurbel, Spitze und die Kugelspitzen der Speichen sind in Mast und Speichen aufgegangen (alle unter 15 mm oder 2 x 15 mm)."),
    text("Thermal by reference as at 1:50; structure: block with mass, mast, ribs and stretchers as members, crown hub and runner as joints, and a 15 mm fixed block as support.",
         "Thermik als Verweis wie bei 1:50; Tragwerk: Block mit Masse, Mast, Speichen und Spreizen als Stäbe, Krone und Läufer als Gelenke und ein 15-mm-Festblock als Auflager."),
]

# ------------------------------------------------------------ 1:200
body200 = [
    box("base-block", CN, (-0.2, 0, -0.15), (0.2, 0.2, 0.15)),
    cap("mast", AL, (0, 0, 0), (2.15, 0, 0), 0.03, 0.025),
    rev("crown-hub", AL(bone="crown"), [[-0.045, 0], [-0.045, 0.07], [0.045, 0.07], [0.045, 0.015], [0.111, 0.015], [0.111, 0]], "v"),
    cap("stretch-right", AL(bone="stretch-right"), (0, 0, 0), (0.55, 0, 0), 0.015),
    cap("stretch-left", AL(bone="stretch-left"), (0, 0, 0), (0.55, 0, 0), 0.015),
]
for rid in RIBS:
    body200.append(rib(rid, 0.015, 0.015, rid))
body200.append(canopy("canopy", 6, 0.003))
alu200 = [b["id"] for b in body200 if b["id"] not in ("canopy", "base-block")]
th200 = thermal_refs(alu200, underside(6, 0.003))
st200 = [mem("member-base", "base-block", "base"), mem("member-mast", "mast", "mast"), mem("member-stretch-right", "stretch-right", "stretch-right"), mem("member-stretch-left", "stretch-left", "stretch-left")]
st200 += [mem("member-" + r, r + "-rod", r) for r in RIBS]
st200 += [mass_base(), basepad(0.03)]
D200 = [
    text("Own 1:200 model: block, a 60 to 50 mm mast, a 70 mm crown hub with finial, 30 mm stretchers, eight 30 mm ribs (the feature; the drawn member line is 18 mm) and the canopy as a 3 mm shell of revolution on six arch segments with the hem skirt; socket, runner, plate and all fittings are merged.",
         "Eigenes 1:200-Modell: Block, ein Mast von 60 auf 50 mm, eine 70-mm-Krone mit Spitze, 30-mm-Spreizen, acht 30-mm-Speichen (das Merkmal; die gezeichnete Linie ist 18 mm) und der Schirm als 3-mm-Rotationsschale auf sechs Bogensegmenten mit Saumrand; Hülse, Läufer, Platte und alle Beschläge sind zusammengefasst."),
    text("Thermal by reference; structure: block with mass, mast, ribs, stretchers and a 30 mm fixed block.",
         "Thermik als Verweis; Tragwerk: Block mit Masse, Mast, Speichen, Spreizen und ein 30-mm-Festblock."),
]

# ------------------------------------------------------------ 1:500
body500 = [
    box("base-block", CN, (-0.2, 0, -0.15), (0.2, 0.2, 0.15)),
    cap("mast", AL, (0, 0, 0), (2.2, 0, 0), 0.0375, 0.0375),
    cap("rib-right-rod", AL(bone="rib-right"), (0.3, 0, 0), (1.0, 0, 0), 0.0375),
    cap("rib-left-rod", AL(bone="rib-left"), (0.3, 0, 0), (1.0, 0, 0), 0.0375),
    canopy("canopy", 4, 0.01),
]
th500 = thermal_refs(["mast", "rib-right-rod", "rib-left-rod"], underside(4, 0.01))
st500 = [mem("member-base", "base-block", "base"), mem("member-mast", "mast", "mast"), mem("member-rib-right", "rib-right-rod", "rib-right"), mem("member-rib-left", "rib-left-rod", "rib-left")]
for r in RIBS[1:4] + RIBS[5:]:
    st500.append(tag(cap("member-" + r, AL(bone=r, member=r, part="member"), (0, 0, 0), (RL, 0, 0), 0.0375), slender=False))
st500 += [tag(cap("member-stretch-right", ST(bone="stretch-right", member="stretch-right", part="member"), (0, 0, 0), (0.55, 0, 0), 0.0375)), tag(cap("member-stretch-left", ST(bone="stretch-left", member="stretch-left", part="member"), (0, 0, 0), (0.55, 0, 0), 0.0375)), mass_base(), basepad(0.075)]
D500 = [
    text("Own 1:500 model: block, a 75 mm mast and the canopy as a 10 mm shell of revolution on four arch segments with the hem skirt; the two ribs the drawing names are 75 mm solids (kept 0.3 to 1.0 m so they stay under the 10 mm shell), the other ribs, hub, stretchers and fittings are below the 75 mm feature and live only in the structure list.",
         "Eigenes 1:500-Modell: Block, ein 75-mm-Mast und der Schirm als 10-mm-Rotationsschale auf vier Bogensegmenten mit Saumrand; die zwei in der Zeichnung genannten Speichen sind 75-mm-Körper (von 0,3 bis 1,0 m, damit sie unter der 10-mm-Schale bleiben), die übrigen Speichen, Krone, Spreizen und Beschläge liegen unter dem 75-mm-Merkmal und stehen nur in der Tragwerksliste."),
    text("Thermal: block, mast and the shell (top and hidden underside as section-face split); structure: block with mass, mast, eight 75 mm ribs and two stretchers as members, and a 75 mm fixed block.",
         "Thermik: Block, Mast und die Schale (Oberseite und verdeckte Unterseite als Schnittflächenteilung); Tragwerk: Block mit Masse, Mast, acht 75-mm-Speichen und zwei Spreizen als Stäbe und ein 75-mm-Festblock."),
]
write("market-parasol", {
    50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
    100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
    200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
    500: dict(body=body500, thermal=th500, structure=st500, decisions=D500),
}, category="objects")
