"""🧾 Round 3, appliances/desktop-monitor: the thermal and structural lists and the volume decisions of every authored scale.

Thermal: the three surface regions are the body surfaces by reference; the faces where the LCD module sits against the rear cover are the joint of one sealed casing (C25 b) and carry `sectionFaces`; the rear plume leaves through an outlet patch laid on the vent slots of the hump (D2). Structure: the real load path desk - rubber pads - steel-cored base plate - riser core - tilt hinge - quick-release bracket - VESA standoff - steel frame of the LCD module, with masses at the centres of mass of the rig.
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location("r3_desktop_monitor_" + name, os.path.join(HERE, f"r3-desktop-monitor-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit")
B = _load("body")

ROWS = {"50": B.HOUSING_50, "100": B.HOUSING_100, "200": B.HOUSING_200}
BAND = {"50": (-0.0225, -0.0130), "100": (-0.0230, -0.0120), "200": (-0.0220, -0.0110)}


def ref(sid, of, region, layer, faces=None):
    """🔗 A thermal reference to a body solid."""
    out = {"id": sid, "of": of, "region": region, "layer": layer}
    if faces:
        out["sectionFaces"] = faces
    return out


def plume(scale):
    """💨 The outlet patch of the rear plume: 0.30 m wide (the emitter diameter) along the vent band of the hump slope, 1.5 mm thick, its outer face 0.3 mm proud of the cover."""
    (um, vm), t, n, length = B.vent_band(ROWS[scale], *BAND[scale])
    u, v = um + n[0] * 0.00045, vm + n[1] * 0.00045
    angle = math.atan2(t[1], t[0])
    half = max(length, 0.012) / 2
    solid = K.box("th-rear-plume", "panel", "housing-plastic", (u - half, u + half), (v - 0.00075, v + 0.00075), (-0.15, 0.15), tone=0.55, rotation=K.about_w(angle))
    solid.update({"region": "housing", "layer": "housing-shell", "tags": ["rear-plume"]})
    return solid


STACK_V = -0.036
OPENING = {"50": (K.GLASS_U0, K.GLASS_U1, K.GLASS_W, K.GLASS_V, K.MODULE_V, 0.0015), "100": (-0.148, 0.140, 0.255, -0.0392, -0.0295, 0.002)}


def own(solid, region, layer):
    """🏷 Gives an own thermal solid its region and layer."""
    solid.update({"region": region, "layer": layer})
    return solid


def front_stack(scale):
    """🖼 The panel front as the 4 mm LCD stack of its layer (bezel face and recessed glass) and, behind it, the edge of the casing up to the rear cover, which belongs to the housing; the faces where they meet are the sealed joint of one casing."""
    u0, u1, w, gv, back, r = OPENING[scale]
    opening = K.rounded((-w, w), (u0, u1), r)
    outline = K.rounded((-K.PANEL_W, K.PANEL_W), (K.PANEL_U0, K.PANEL_U1), 0.005)
    return [
        own(K.face("th-front-frame", "panel", "bezel-plastic", outline, (K.FRONT_V, STACK_V), holes=[opening], sectionFaces=["+v"]), "panel-front", "front-stack"),
        own(K.face("th-front-glass", "panel", "display-glass", opening, (gv, STACK_V), role="glazing", sectionFaces=["+v"]), "panel-front", "front-stack"),
        own(K.face("th-casing-rim", "panel", "housing-plastic", outline, (STACK_V, back), sectionFaces=["-v"]), "housing", "housing-shell"),
    ]


def thermal(scale, body):
    """🌡 The thermal list of a scale."""
    ids = {s["id"] for s in body}
    stand = [ref("th-" + sid, sid, "stand", "stand-body") for sid in ("mount-plate", "hinge", "neck", "base-plate", "foot-front", "foot-rear") if sid in ids]
    if scale == "200":
        front = own(K.along_v("th-front-stack", "panel", "bezel-plastic", [(K.FRONT_V,) + B.SLAB_200[0][1:], (STACK_V,) + B.SLAB_200[0][1:]], sectionFaces=["+v"]), "panel-front", "front-stack")
        rim = own(K.along_v("th-casing-rim", "panel", "housing-plastic", [(STACK_V,) + B.SLAB_200[1][1:], B.SLAB_200[1]], sectionFaces=["-v"]), "housing", "housing-shell")
        hump = own(K.along_v("th-housing", "panel", "housing-plastic", B.HOUSING_200, sectionFaces=["-v"]), "housing", "housing-shell")
        return [front, rim, hump, plume(scale)] + stand
    out = front_stack(scale)
    if "power-led" in ids:
        out.append(ref("th-power-led", "power-led", "panel-front", "front-stack"))
    out.append(ref("th-housing", "housing", "housing", "housing-shell", ["-v"]))
    out.append(plume(scale))
    return out + stand


def part(solid, member, kind, **more):
    """🏗 Tags a solid as a structural part."""
    solid.update({"member": member, "part": kind})
    solid.update(more)
    return solid


def support(thick):
    """🟩 The contact of the four pads on the desk: one pad over the 0.19 x 0.20 m footprint of the anchor, on the desk plane."""
    return part(K.box("support-base-contact", "base", "rubber-foot", (0.010, 0.200), (0.0, thick), (-0.10, 0.10), tone=0.3, role="detail"), "base", "support", anchor="base-contact")


def structure(scale, body):
    """🏗 The structural list of a scale."""
    steel = dict(tone=0.3, role="detail")
    if scale == "50":
        return [
            part(K.box("member-base-core", "base", "stand-plastic", (0.020, 0.190), (0.004, 0.008), (-0.110, 0.110), radius=0.0015, **steel), "base", "member"),
            part(K.box("member-neck-core", "neck", "stand-plastic", (-0.004, 0.236), (0.003, 0.013), (-0.022, 0.022), radius=0.002, **steel), "neck", "member"),
            part(K.turned("joint-neck-foot", "neck", "stand-plastic", [[0, 0], [0, 0.012], [0.012, 0.012], [0.012, 0]], [1, 0, 0], (-0.006, 0.008, 0.0), tone=0.6, role="detail"), "neck", "joint"),
            {"id": "joint-hinge", "of": "hinge", "member": "neck", "part": "joint"},
            {"id": "member-mount", "of": "mount-plate", "member": "panel", "part": "member"},
            part(K.box("member-vesa", "panel", "stand-plastic", (-0.05, 0.05), (-0.0275, -0.0045), (-0.05, 0.05), radius=0.002, **steel), "panel", "member"),
            part(K.face("member-panel-frame", "panel", "stand-plastic", K.rounded((-0.266, 0.266), (-0.166, 0.149), 0.004), (-0.0385, -0.0305), holes=[K.rounded((-0.251, 0.251), (-0.151, 0.134), 0.002)], **steel), "panel", "member"),
            part(K.box("member-panel-rail", "panel", "stand-plastic", (-0.03, -0.01), (-0.0345, -0.0275), (-0.252, 0.252), **steel), "panel", "member"),
            part(K.box("member-panel-stile", "panel", "stand-plastic", (-0.152, 0.135), (-0.0345, -0.0275), (-0.012, 0.012), **steel), "panel", "member"),
            part(K.blob("mass-panel", "panel", "stand-plastic", (-0.01, -0.024, 0.0), (0.018, 0.0075, 0.018), tone=0.15, role="detail"), "panel", "mass"),
            part(K.blob("mass-neck", "neck", "stand-plastic", (0.12, 0.008, 0.0), (0.012, 0.0055, 0.012), tone=0.15, role="detail"), "neck", "mass"),
            part(K.blob("mass-base", "base", "stand-plastic", (0.105, 0.006, 0.0), (0.02, 0.002, 0.02), tone=0.15, role="detail"), "base", "mass"),
            support(0.003),
        ]
    if scale == "100":
        return [
            {"id": "member-base", "of": "base-plate", "member": "base", "part": "member"},
            part(K.box("member-neck-core", "neck", "stand-plastic", (-0.004, 0.232), (0.0005, 0.0155), (-0.024, 0.024), radius=0.003, **steel), "neck", "member"),
            part(K.turned("joint-hinge", "neck", "stand-plastic", [[0, 0], [0, 0.0105], [0.064, 0.0105], [0.064, 0]], [0, 0, 1], (K.NECK_LEN, 0.0, -0.032), tone=0.6, role="detail"), "neck", "joint"),
            part(K.box("member-vesa", "panel", "stand-plastic", (-0.05, 0.05), (-0.0275, 0.002), (-0.05, 0.05), radius=0.003, **steel), "panel", "member"),
            part(K.face("member-panel-frame", "panel", "stand-plastic", K.rounded((-0.262, 0.262), (-0.163, 0.146), 0.006), (-0.0385, -0.0275), holes=[K.rounded((-0.24, 0.24), (-0.141, 0.124), 0.003)], **steel), "panel", "member"),
            part(K.box("member-panel-rail", "panel", "stand-plastic", (-0.035, 0.005), (-0.0385, -0.0275), (-0.241, 0.241), **steel), "panel", "member"),
            part(K.blob("mass-panel", "panel", "stand-plastic", (-0.01, -0.019, 0.0), (0.02, 0.008, 0.02), tone=0.15, role="detail"), "panel", "mass"),
            part(K.blob("mass-base", "base", "stand-plastic", (0.105, 0.0061, 0.0), (0.03, 0.0061, 0.03), tone=0.15, role="detail"), "base", "mass"),
            support(0.0124),
        ]
    return [
        {"id": "member-base", "of": "base-plate", "member": "base", "part": "member"},
        {"id": "member-neck", "of": "neck", "member": "neck", "part": "member"},
        part(K.box("member-panel-rail", "panel", "stand-plastic", (-0.035, -0.005), (-0.034, -0.004), (-0.25, 0.25), **steel), "panel", "member"),
        part(K.box("member-panel-stile", "panel", "stand-plastic", (-0.12, 0.08), (-0.034, -0.004), (-0.015, 0.015), **steel), "panel", "member"),
        part(K.blob("mass-panel", "panel", "stand-plastic", (-0.01, -0.022, 0.0), (0.04, 0.0165, 0.04), tone=0.15, role="detail"), "panel", "mass"),
        support(0.0124),
    ]


def text(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


def decisions(scale):
    """📝 The volume decisions of a scale."""
    if scale == "50":
        return [
            text("Own 1:50 model of a 23.8-inch fixed-stand monitor (Dell SE2422H class). Panel: a black bezel frame 0.54 x 0.323 m and 9.5 mm deep with 6.5 mm sides and top and an 19 mm chin, the cover glass of the 527 x 296.5 mm active area set 0.7 mm back in its opening, the power light as a lens in the chin; the rear cover is one loft of nine sections from the full panel edge over the sloping upper back to the flat 0.35 x 0.17 m back of the electronics hump, 36 mm behind the screen; four vent slots lie across the upper slope; the quick-release bracket sits on the VESA field.",
                 "Eigenes 1:50-Modell eines 23,8-Zoll-Monitors mit festem Standfuß (Klasse Dell SE2422H). Panel: ein schwarzer Rahmen 0,54 x 0,323 m und 9,5 mm tief mit 6,5 mm Rand seitlich und oben und 19 mm Kinn, das Deckglas der aktiven Fläche 527 x 296,5 mm 0,7 mm tief in seiner Öffnung, die Betriebsleuchte als Linse im Kinn; die Rückwand ist ein Loft aus neun Schnitten vom vollen Panelrand über den schrägen oberen Rücken bis zur flachen Rückseite 0,35 x 0,17 m des Elektronikbuckels, 36 mm hinter dem Bildschirm; vier Lüftungsschlitze liegen quer auf dem oberen Hang; der Schnellwechselhalter sitzt auf dem VESA-Feld."),
            text("Stand: the riser is a rounded column 62 x 22 mm with a flared foot sunk into the plate and the 20 mm tilt-hinge barrel at its head inside the bracket; the base plate is a 0.25 x 0.21 m rounded plate with a bevelled top edge on four 22 mm rubber pads. Thermal: panel front, rear cover and stand by reference; the faces of the module against the rear cover are the sealed joint of one casing (sectionFaces); the rear plume leaves through a 0.30 m outlet patch on the vent slots. Structure: steel core of the plate, riser core, foot screw boss and hinge as joints, bracket, VESA standoff and the steel frame of the LCD module with its rail and stile as members, masses at the rig centres, one contact pad under the four feet.",
                 "Standfuß: der Hals ist eine gerundete Säule 62 x 22 mm mit verbreitertem Fuß, in die Platte eingelassen, und der 20-mm-Neigegelenkwalze am Kopf im Halter; die Fußplatte ist eine gerundete Platte 0,25 x 0,21 m mit abgeschrägter Oberkante auf vier 22-mm-Gummifüßen. Thermik: Panelfront, Rückwand und Standfuß als Verweis; die Flächen des Moduls an der Rückwand sind die dichte Fuge eines Gehäuses (sectionFaces); die Warmluftfahne tritt durch ein 0,30 m breites Austrittsfeld auf den Lüftungsschlitzen aus. Tragwerk: Stahlkern der Platte, Halskern, Schraubdom am Fuß und Gelenk als Knoten, Halter, VESA-Distanz und Stahlrahmen des LCD-Moduls mit Riegel und Ständer als Stäbe, Massen an den Schwerpunkten des Rigs, ein Kontaktpolster unter den vier Füßen."),
        ]
    if scale == "100":
        return [
            text("Own 1:100 model: the bezel frame stays 9.5 mm deep but its sides and top widen to 15 mm and the chin to 22 mm so that the glass reads at one feature; the rear cover is a loft of five sections; the hinge barrel and the bracket merge into the rounded head of the riser; the pads merge into the base plate, which stands on the desk; the power light and the vent slots are omitted (below 15 mm).",
                 "Eigenes 1:100-Modell: der Rahmen bleibt 9,5 mm tief, seine Seiten und die Oberkante werden auf 15 mm und das Kinn auf 22 mm verbreitert, damit das Glas mit einem Merkmal lesbar bleibt; die Rückwand ist ein Loft aus fünf Schnitten; Gelenkwalze und Halter gehen im gerundeten Kopf des Halses auf; die Füße gehen in der Fußplatte auf, die auf dem Tisch steht; Betriebsleuchte und Lüftungsschlitze entfallen (unter 15 mm)."),
            text("Thermal by reference with the sealed module joint as sectionFaces and the plume outlet on the slope of the hump; structure: the base plate itself as member, the riser core, a hinge joint cylinder, the VESA standoff with the bracket, the module frame with its rail, two masses and the contact pad.",
                 "Thermik als Verweis mit der dichten Modulfuge als sectionFaces und dem Fahnenaustritt auf dem Hang des Buckels; Tragwerk: die Fußplatte selbst als Stab, der Halskern, ein Gelenkzylinder, die VESA-Distanz mit dem Halter, der Modulrahmen mit Riegel, zwei Massen und das Kontaktpolster."),
        ]
    return [
        text("Own 1:200 model: the panel is one 12.5 mm slab (bezel, glass and module merged) with the rear hump as a loft of three sections; the riser is one feature (34 mm) deep with its hinge head; the base plate is a plain 12 mm rounded plate with a bevel. Glass, light, vents, bracket and pads are below 30 mm and merged or omitted.",
             "Eigenes 1:200-Modell: das Panel ist eine 12,5 mm dicke Platte (Rahmen, Glas und Modul zusammengefasst) mit dem Rückenbuckel als Loft aus drei Schnitten; der Hals ist ein Merkmal (34 mm) tief mit seinem Gelenkkopf; die Fußplatte ist eine schlichte gerundete 12-mm-Platte mit Fase. Glas, Leuchte, Schlitze, Halter und Füße liegen unter 30 mm und sind zusammengefasst oder entfallen."),
        text("Thermal: slab, hump, riser and plate by reference, the slab's back face as the sealed module joint, plus the plume outlet; structure: base plate and riser as members, the frame of the LCD module generalised to a rail and a stile crossing at the VESA field, one panel mass and the contact pad.",
             "Thermik: Platte, Buckel, Hals und Fußplatte als Verweis, die Rückseite der Platte als dichte Modulfuge, dazu der Fahnenaustritt; Tragwerk: Fußplatte und Hals als Stäbe, der Rahmen des LCD-Moduls verallgemeinert zu einem Riegel und einem Ständer, die sich am VESA-Feld kreuzen, eine Panelmasse und das Kontaktpolster."),
    ]
