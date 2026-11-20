"""🔥 Round 3 panel radiator: the handcrafted `volumes` of every authored scale (1:50, 1:100, 1:200; 1:500 unavailable) written to `volumes/components/panel-radiator.json`.

Every solid is stated by hand from the type 22 valve-compact construction in `r3-panel-radiator-kit.py`: two pressed water panels with 30 vertical channels, two trapezoidal convector fin packs, side covers, slotted top grille, four wall brackets, the thermostat head on the left end (seen from the room) and the bottom connection block with its two pipes.

Usage: python3 r3-panel-radiator-build.py
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-panel-radiator-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
box, span_box, extrude, revolve, ellipsoid = K.box, K.span_box, K.extrude, K.revolve, K.ellipsoid
H, W = K.HALF, K.WALL
SOURCE = os.path.join(T, "volumes", "components", "panel-radiator.json")
PLASTIC = "abs-plastic"


def ref(sid, of, **extra):
    """🔗 A `{ of }` reference to a body solid of the same scale."""
    out = {"id": sid, "of": of}
    out.update(extra)
    return out


def text(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def head50(bone="valve-head"):
    """🌡️ The thermostat head at 1:50: plastic dial body 52 mm across and 80 mm long with a rounded cap, on the joint axis (w)."""
    return revolve("valve-head", [(-0.513, 0), (-0.513, 0.0215), (-0.516, 0.026), (-0.583, 0.026), (-0.589, 0.0235), (-0.593, 0.018), (-0.595, 0.009), (-0.595, 0)], [0, 0, 1], [0, 0, 0], bone=bone, material=PLASTIC, tone=0.85)


def ports(sid, solid_fn):
    """🔁 A connection part at the outer port, copied to the inner port 50 mm further in."""
    s = solid_fn(sid)
    s["repeat"] = {"count": 2, "translation": [0, 0, K.r5(K.PORTS_W[1] - K.PORTS_W[0])]}
    return s


def body50():
    """🏗️ 1:50 body: every assembly a type 22 valve-compact radiator has at the 7.5 mm feature."""
    rear_lo, rear_hi = K.REAR
    front_lo, front_hi = K.FRONT
    out = [
        extrude("panel-rear", "yz", K.channel_profile(rear_lo, rear_hi), 0.035, 0.55),
        span_box("header-rear", (0, 0.04), K.REAR, (-H, H), 0.006, repeat={"count": 2, "translation": [0.545, 0, 0]}),
        extrude("panel-front", "yz", K.channel_profile(front_lo, front_hi), 0.035, 0.55),
        span_box("header-front", (0, 0.04), K.FRONT, (-H, H), 0.006, repeat={"count": 2, "translation": [0.545, 0, 0]}),
        extrude("fins-rear", "yz", K.fin_profile(-0.0215, -0.0495, -0.4875, 0.4875), 0.025, 0.565, role="texture", tone=0.5, texture=True),
        extrude("fins-front", "yz", K.fin_profile(-0.0785, -0.0505, -0.4875, 0.4875), 0.025, 0.565, role="texture", tone=0.5, texture=True),
    ]
    for name, sgn in (("left", 1), ("right", -1)):
        prof = [(-0.022, 0.5), (-0.078, 0.5), (-0.078, 0.49), (-0.0765, 0.49), (-0.0765, 0.4985), (-0.0235, 0.4985), (-0.0235, 0.49), (-0.022, 0.49)]
        out.append(extrude("side-cover-" + name, "yz", [(v, sgn * w) for v, w in prof], 0.0, K.PANEL_H, tone=0.6))
    out += [
        extrude("grille-rim", "yz", [(-0.1015, -0.5015), (0.0015, -0.5015), (0.0015, 0.5015), (-0.1015, 0.5015)], 0.575, K.TOP, holes=[[(-0.1, -H), (-0.1, H), (0.0, H), (0.0, -H)]], tone=0.6),
        extrude("grille-plate", "yz", [(-0.1, -H), (0.0, -H), (0.0, H), (-0.1, H)], K.TOP - 0.0075, K.TOP, holes=K.slot_holes(), role="detail", tone=0.6),
    ]
    up = [(0.44, 0.035), (0.44, 0.032), (0.47, 0.032), (0.47, 0.0), (0.494, 0.0), (0.494, 0.004), (0.482, 0.004), (0.482, 0.032), (0.49, 0.032), (0.49, 0.035)]
    low = [(0.025, 0.035), (0.025, 0.032), (0.05, 0.032), (0.05, 0.0), (0.062, 0.0), (0.062, 0.032), (0.075, 0.032), (0.075, 0.035)]
    for sid, prof in (("bracket-upper", up), ("bracket-lower", low)):
        out.append(extrude(sid, "xy", prof, K.BRACKET_W[0], K.BRACKET_W[1], role="detail", tone=0.4, repeat={"count": 2, "translation": [0, 0, -0.8]}))
    hu, hv = K.HEAD_U, K.HEAD_V
    out += [
        head50(),
        revolve("valve-collar", [(-0.497, 0), (-0.497, 0.018), (-0.514, 0.018), (-0.514, 0)], [0, 0, 1], [hu, hv, 0]),
        span_box("connection-block", K.BLOCK["u"], K.BLOCK["v"], K.BLOCK["w"], 0.004, material="brass", role="detail", tone=0.3),
        ports("connection-nut-upper", lambda sid: extrude(sid, "yz", K.hexagon(K.PORT_V, K.PORTS_W[0], 0.03), -0.03, -0.012, material="brass", role="detail", tone=0.3)),
        ports("connection-nipple", lambda sid: revolve(sid, [(-0.013, 0), (-0.013, 0.0105), (0.003, 0.0105), (0.003, 0)], [1, 0, 0], [0, K.PORT_V, K.PORTS_W[0]])),
        ports("spindle-cap", lambda sid: revolve(sid, [(0.001, 0), (0.001, 0.01), (-0.012, 0.01), (-0.012, 0)], [0, 1, 0], [-0.0475, -0.068, K.PORTS_W[0]])),
        ports("connection-nut-lower", lambda sid: extrude(sid, "yz", K.hexagon(K.PORT_V, K.PORTS_W[0], 0.027), -0.08, -0.064, material="brass", role="detail", tone=0.3)),
        ports("pipe", lambda sid: revolve(sid, [(K.FLOOR_U, 0), (K.FLOOR_U, 0.0075), (-0.079, 0.0075), (-0.079, 0)], [1, 0, 0], [0, K.PORT_V, K.PORTS_W[0]])),
    ]
    for end, sgn in (("left", 1), ("right", -1)):
        prof = [(0.497, 0), (0.497, K.PLUG_R), (0.5 + K.PLUG_OUT, K.PLUG_R), (0.5 + K.PLUG_OUT, 0)]
        prof = [(sgn * a, r) for a, r in prof] if sgn > 0 else [(-a, r) for a, r in reversed(prof)]
        out.append(revolve("plug-rear-" + end, prof, [0, 0, 1], [K.HUB[0], -0.011, 0], repeat={"count": 2, "translation": [K.HUB[1] - K.HUB[0], 0, 0]}))
        out.append(revolve("plug-front-lower-" + end, prof, [0, 0, 1], [K.HUB[0], -0.089, 0]))
        out.append(revolve("plug-front-upper-" + end, prof, [0, 0, 1], [K.HUB[1], -0.089, 0]))
    return out


def band(sid, region, u0, u1, header=None):
    """🌡️ One thermal band of the front water panel between u0 and u1: the channelled section, with the horizontal header where the band holds one."""
    lo, hi = K.FRONT
    out = [dict(extrude(sid, "yz", K.channel_profile(lo, hi), u0, u1), region=region)]
    if header is not None:
        out.append(dict(span_box(sid + "-header", header, K.FRONT, (-H, H), 0.006), region=region))
    return out


def thermal50():
    """🌡️ 1:50 thermal: every region on the real part, the front panel in its three flow bands, emitters as outlet patches."""
    out = [
        ref("th-panel-rear", "panel-rear", region="panel-rear"),
        ref("th-header-rear", "header-rear", region="panel-rear"),
        ref("th-side-cover-left", "side-cover-left", region="panel-rear"),
        ref("th-side-cover-right", "side-cover-right", region="panel-rear"),
        ref("th-plug-rear-left", "plug-rear-left", region="panel-rear"),
        ref("th-plug-rear-right", "plug-rear-right", region="panel-rear"),
    ]
    out += band("th-band-lower", "panel-front-lower", 0.035, 0.195, (0, 0.04))
    out += band("th-band-mid", "panel-front-mid", 0.195, 0.39)
    out += band("th-band-upper", "panel-front-upper", 0.39, 0.55, (0.545, 0.585))
    out += [
        ref("th-plug-front-lower-left", "plug-front-lower-left", region="panel-front-lower"),
        ref("th-plug-front-lower-right", "plug-front-lower-right", region="panel-front-lower"),
        ref("th-plug-front-upper-left", "plug-front-upper-left", region="panel-front-upper"),
        ref("th-plug-front-upper-right", "plug-front-upper-right", region="panel-front-upper"),
        ref("th-fins-rear", "fins-rear", region="convector"),
        ref("th-fins-front", "fins-front", region="convector"),
        ref("th-grille-rim", "grille-rim", region="top-grille"),
        ref("th-grille-plate", "grille-plate", region="top-grille"),
        ref("th-bracket-upper", "bracket-upper", region="bracket", sectionFaces=["+v"]),
        ref("th-bracket-lower", "bracket-lower", region="bracket", sectionFaces=["+v"]),
    ]
    for sid in ("valve-head", "valve-collar", "connection-block", "connection-nut-upper", "connection-nipple", "spindle-cap", "connection-nut-lower", "pipe"):
        out.append(ref("th-" + sid, sid, region="valve"))
    out += emitters(0.0075)
    return out


def emitters(t):
    """💨 The two emitters as outlet patches `t` proud of the surface the air leaves: the convection outlet over the grille slots (0.0284 m2, the area of the 0.19 m nozzle), the plume patch on the middle of the front panel (the area of the 0.1 m nozzle)."""
    side = 0.0886
    return [
        dict(span_box("th-emitter-convection-outlet", (K.TOP, K.TOP + t), (-0.095, -0.005), (-0.1575, 0.1575), tone=0.5), region="top-grille", tags=["convection-outlet"]),
        dict(span_box("th-emitter-front-plume", (0.2925 - side / 2, 0.2925 + side / 2), (-0.1 - t, -0.1), (-side / 2, side / 2), tone=0.5), region="panel-front-mid", tags=["front-plume"]),
    ]


def sheet_shell(sid, v, u1, wall=0.00125, **kw):
    """🧊 A pressed water panel as the structural member: the two 1.25 mm steel sheets around the water space, a hollow section 22 x 1000 mm."""
    lo, hi = v
    hole = [(lo + wall, -H + wall), (hi - wall, -H + wall), (hi - wall, H - wall), (lo + wall, H - wall)]
    return extrude(sid, "yz", [(lo, -H), (hi, -H), (hi, H), (lo, H)], 0.0, u1, holes=[hole], tone=0.6, member="radiator", part="member", **kw)


def nipples(r, u_step, kind="revolve"):
    """🔩 The pressed connection nipples that join the two panels at the ends of both headers: the joints of the panel pair."""
    out = []
    for name, w in (("left", 0.47), ("right", -0.47)):
        if kind == "revolve":
            s = revolve("joint-nipple-" + name, [(-0.078, 0), (-0.078, r), (-0.022, r), (-0.022, 0)], [0, 1, 0], [K.HUB[0], 0, w], material="painted-steel", role="body", tone=0.5)
            s["origin"] = [K.HUB[0], 0, w]
        else:
            s = span_box("joint-nipple-" + name, (K.HUB[0] - r, K.HUB[0] + r), (-0.078, -0.022), (w - r, w + r), tone=0.5)
        s.update(member="radiator", part="joint", repeat={"count": 2, "translation": [u_step, 0, 0]})
        out.append(s)
    return out


def supports(screw_r, head_r, pad):
    """🧱 The four wall fixings: the screws of the two upper brackets (pins, the radiator hangs on them) and the flanges of the two lower holders against the wall (rollers, horizontal reaction)."""
    out = []
    for name, w in (("left", 0.4), ("right", -0.4)):
        s = revolve("support-hook-" + name, [(-0.006, 0), (-0.006, head_r), (-0.003, head_r), (-0.003, screw_r), (0.025, screw_r), (0.025, 0)], [0, 1, 0], [0.465, W, w], material="painted-steel", role="body", tone=0.4)
        s.update(member="radiator", part="support", anchor="hook-" + name)
        out.append(s)
        p = span_box("support-rest-" + name, (0.025, 0.075), (W - pad / 2, W + pad / 2), (w - 0.02, w + 0.02), tone=0.4)
        p.update(member="radiator", part="support", anchor="rest-" + name)
        out.append(p)
    return out


def hooks(t):
    """🪝 The joints where the welded lugs of the rear panel rest on the hook noses of the two upper brackets (`t` = the feature of the scale)."""
    j = span_box("joint-hook", (0.482, 0.482 + t), (-0.003, -0.003 + t), (0.38, 0.42), tone=0.5, repeat={"count": 2, "translation": [0, 0, -0.8]})
    j.update(member="radiator", part="joint")
    return [j]


def structure50():
    """🦴 1:50 structure: the two pressed panels as hollow sheet members, the four nipples, the brackets, the wall fixings and the masses."""
    out = [sheet_shell("member-panel-rear", K.REAR, K.PANEL_H), sheet_shell("member-panel-front", K.FRONT, K.PANEL_H)]
    out += nipples(0.012, K.HUB[1] - K.HUB[0]) + hooks(0.0095)
    out += [ref("member-bracket-upper", "bracket-upper", member="radiator", part="member"), ref("member-bracket-lower", "bracket-lower", member="radiator", part="member")]
    j = revolve("joint-valve-head", [(-0.48, 0), (-0.48, 0.006), (-0.514, 0.006), (-0.514, 0)], [0, 0, 1], [0, 0, 0], bone="valve-head", material="brass", role="body", tone=0.5)
    j.update(member="valve-head", part="joint")
    out.append(j)
    out += supports(0.0035, 0.0065, 0.0075)
    m = ellipsoid("mass-radiator", (0.3, -0.05, 0), (0.03, 0.02, 0.03), tone=0.5)
    m.update(member="radiator", part="mass")
    hm = ellipsoid("mass-valve-head", (0, 0, -0.555), (0.009, 0.009, 0.009), bone="valve-head", material=PLASTIC, tone=0.5)
    hm.update(member="valve-head", part="mass")
    return out + [m, hm]


DECISIONS50 = [
    text("Body: a type 22 valve-compact radiator as built. Each water panel is two pressed 1.25 mm sheets: 30 vertical channels at 33.3 mm pitch with 4 mm seam grooves on both faces (extruded section) between the bottom and top headers (40 mm, full 22 mm thickness, rounded). Between the panels two trapezoidal convector fin sheets of 0.6 mm at 25 mm pitch (texture, as the section draws them), closed at the ends by folded side covers; on top the grille with 4 x 8 slots of 8 x 100 mm, its 1.2 mm sheet generalised to the 7.5 mm feature, in a folded 1.5 mm rim with 25 mm lips. Four wall brackets as folded L sections (12 mm arm, 3 mm wall flange, hook nose at the upper ones). On the left end (seen from the room) the plastic thermostat head (52 x 82 mm) on its brass union collar, eight blind and vent plugs in the side tappings, and below that end the brass H-block with hex unions, isolating spindle caps and the two 15 mm pipes to the floor.", "Körper: ein Ventilkompaktheizkörper Typ 22 wie gebaut. Jede Wasserplatte besteht aus zwei gepressten 1,25-mm-Blechen: 30 senkrechte Kanäle mit 33,3 mm Teilung und 4 mm tiefen Nahtrillen auf beiden Seiten (extrudierter Querschnitt) zwischen dem unteren und oberen Sammelkanal (40 mm, volle Dicke 22 mm, gerundet). Zwischen den Platten zwei trapezförmige Konvektorbleche von 0,6 mm mit 25 mm Teilung (Textur, wie der Schnitt sie zeichnet), an den Enden von gekanteten Seitenblenden geschlossen; oben das Abdeckgitter mit 4 x 8 Schlitzen von 8 x 100 mm, sein 1,2-mm-Blech auf das Merkmal von 7,5 mm verallgemeinert, in einem gekanteten 1,5-mm-Rahmen mit 25-mm-Lippen. Vier Wandkonsolen als gekantete L-Profile (12-mm-Arm, 3-mm-Wandflansch, Hakennase an den oberen). Am linken Ende (vom Raum aus) der Thermostatkopf aus Kunststoff (52 x 82 mm) auf seinem Überwurf aus Messing, acht Blind- und Entlüftungsstopfen in den seitlichen Muffen, und unter diesem Ende der Hahnblock aus Messing mit Sechskantverschraubungen, Absperrkappen und den zwei 15-mm-Rohren zum Boden."),
    text("Thermal: the rear panel region holds the rear panel with its headers, the side covers and its plugs; the front panel is modelled in its three flow bands (lower band with the bottom header, upper band with the top header, each with its plugs) on the same channelled section as the body; convector = both fin sheets (exchange area); top grille = rim and slotted plate; brackets with their wall face marked as mounting face; valve = head, collar, H-block, unions, caps and pipes. The two emitters are outlet patches 7.5 mm proud: over the grille slots (0.0284 m2, the 0.19 m nozzle) and on the middle of the front panel (the 0.1 m nozzle). The panels' declared areas count one face only and stay as declared (area review).", "Thermisch: die Region der Rückplatte umfasst die Rückplatte mit ihren Sammelkanälen, die Seitenblenden und ihre Stopfen; die Frontplatte ist in ihren drei Strömungsbändern modelliert (unteres Band mit dem unteren, oberes Band mit dem oberen Sammelkanal, je mit ihren Stopfen) auf demselben Kanalquerschnitt wie der Körper; Konvektor = beide Lamellenbleche (Austauschfläche); Abdeckgitter = Rahmen und Schlitzplatte; Konsolen mit ihrer Wandseite als Montagefläche markiert; Ventil = Kopf, Überwurf, Hahnblock, Verschraubungen, Kappen und Rohre. Die beiden Emitter sind Auslassflächen 7,5 mm über der Oberfläche: über den Gitterschlitzen (0,0284 m2, die Düse von 0,19 m) und auf der Mitte der Frontplatte (die Düse von 0,1 m). Die deklarierten Flächen der Platten zählen nur eine Seite und bleiben wie deklariert (Flächenprüfung)."),
    text("Structure: the load path of a hung radiator. The two water panels are hollow sheet members (1.25 mm walls around the water space), joined by the four pressed connection nipples at the header ends; the radiator hangs with its rear-panel lugs (joints) on the hook noses of the two upper brackets (pins: the wall screws) and is held off the wall by the two lower holders (rollers: the wall flanges); the thermostat head turns on its valve spindle (joint on the revolute bone). Masses: radiator with water at its centre, thermostat head.", "Tragwerk: der Lastpfad eines gehängten Heizkörpers. Die beiden Wasserplatten sind hohle Blechstäbe (1,25-mm-Wände um den Wasserraum), verbunden durch die vier gepressten Verbindungsnippel an den Enden der Sammelkanäle; der Heizkörper hängt mit den Laschen der Rückplatte (Gelenke) an den Hakennasen der beiden oberen Konsolen (Festlager: die Wandschrauben) und wird von den beiden unteren Haltern auf Abstand gehalten (Rollenlager: die Wandflansche); der Thermostatkopf dreht auf seiner Ventilspindel (Gelenk am Drehknochen). Massen: Heizkörper mit Wasser in seinem Schwerpunkt, Thermostatkopf."),
]


def body100():
    """🏗️ 1:100 body at the 15 mm feature: flat panels (the 4 mm channel grooves are 0.04 mm on paper), the fin packs as one block, side covers as 15 mm end closures, the 30 mm grille drawn at this scale, arm brackets, head, block and pipes."""
    return [
        span_box("panel-rear", (0, 0.57), K.REAR, (-H, H), 0.004),
        span_box("panel-front", (0, 0.57), K.FRONT, (-H, H), 0.004),
        span_box("convector", (0.025, 0.55), (-0.0775, -0.0225), (-0.485, 0.485), role="detail", tone=0.45),
        span_box("side-cover", (0, 0.57), (-0.078, -0.022), (0.485, 0.5), tone=0.6, repeat={"count": 2, "translation": [0, 0, -0.985]}),
        span_box("top-grille", (0.57, K.TOP), (-0.1, 0.0), (-H, H), 0.004, role="detail", tone=0.55),
        span_box("bracket-upper", (0.468, 0.483), (0.0, W), (0.38, 0.42), role="detail", tone=0.4, repeat={"count": 2, "translation": [0, 0, -0.8]}),
        span_box("bracket-lower", (0.048, 0.063), (0.0, W), (0.38, 0.42), role="detail", tone=0.4, repeat={"count": 2, "translation": [0, 0, -0.8]}),
        revolve("valve-head", [(-0.497, 0), (-0.497, 0.018), (-0.514, 0.018), (-0.514, 0.026), (-0.595, 0.026), (-0.595, 0)], [0, 0, 1], [0, 0, 0], bone="valve-head", material=PLASTIC, tone=0.85),
        span_box("connection-block", (-0.06, 0.0), K.BLOCK["v"], K.BLOCK["w"], 0.004, material="brass", role="detail", tone=0.3),
        ports("pipe", lambda sid: revolve(sid, [(K.FLOOR_U, 0), (K.FLOOR_U, 0.0075), (-0.059, 0.0075), (-0.059, 0)], [1, 0, 0], [0, K.PORT_V, K.PORTS_W[0]])),
    ]


def thermal100():
    """🌡️ 1:100 thermal: rear panel with the side covers, the front panel as three flat bands (thirds of the 0.57 m panel drawn at this scale), convector block, grille, brackets, valve, outlet patches 15 mm proud."""
    third = 0.57 / 3
    out = [ref("th-panel-rear", "panel-rear", region="panel-rear"), ref("th-side-cover", "side-cover", region="panel-rear")]
    for k, (name, region) in enumerate((("lower", "panel-front-lower"), ("mid", "panel-front-mid"), ("upper", "panel-front-upper"))):
        out.append(dict(span_box("th-band-" + name, (k * third, (k + 1) * third), K.FRONT, (-H, H), 0.004 if k != 1 else 0), region=region))
    out += [
        ref("th-convector", "convector", region="convector"),
        ref("th-top-grille", "top-grille", region="top-grille"),
        ref("th-bracket-upper", "bracket-upper", region="bracket", sectionFaces=["+v"]),
        ref("th-bracket-lower", "bracket-lower", region="bracket", sectionFaces=["+v"]),
        ref("th-valve-head", "valve-head", region="valve"),
        ref("th-connection-block", "connection-block", region="valve"),
        ref("th-pipe", "pipe", region="valve"),
    ]
    return out + emitters(0.015)


def structure100():
    """🦴 1:100 structure: hollow panel members to the 0.57 m panel height of this scale, four nipples, brackets, fixings, the radiator mass."""
    out = [sheet_shell("member-panel-rear", K.REAR, 0.57), sheet_shell("member-panel-front", K.FRONT, 0.57)]
    out += nipples(0.0125, K.HUB[1] - K.HUB[0] - 0.01) + hooks(0.015)
    out += [ref("member-bracket-upper", "bracket-upper", member="radiator", part="member"), ref("member-bracket-lower", "bracket-lower", member="radiator", part="member")]
    j = revolve("joint-valve-head", [(-0.48, 0), (-0.48, 0.0075), (-0.514, 0.0075), (-0.514, 0)], [0, 0, 1], [0, 0, 0], bone="valve-head", material="brass", role="body", tone=0.5)
    j.update(member="valve-head", part="joint")
    out.append(j)
    out += supports(0.0075, 0.009, 0.015)
    m = ellipsoid("mass-radiator", (0.3, -0.05, 0), (0.03, 0.02, 0.03), tone=0.5)
    m.update(member="radiator", part="mass")
    return out + [m]


DECISIONS100 = [
    text("Body: the panels are flat 22 mm plates with 4 mm rounded edges (the channel grooves are 0.04 mm on paper and the headers 0.4 mm), the two fin sheets are one fin-pack block between the panels, the folded side covers are 15 mm end closures, the grille is the 30 mm bar this scale draws, the four brackets are 15 x 35 x 40 mm arms. The thermostat head is one turned body with its collar, the H-block takes in its unions and caps, the two pipes stay. Plugs omitted (11 mm on a 15 mm feature).", "Körper: die Platten sind ebene 22-mm-Platten mit 4 mm gerundeten Kanten (die Kanalrillen messen auf Papier 0,04 mm, die Sammelkanäle 0,4 mm), die beiden Lamellenbleche sind ein Lamellenblock zwischen den Platten, die gekanteten Seitenblenden sind 15-mm-Endabschlüsse, das Gitter ist die 30-mm-Leiste dieses Maßstabs, die vier Konsolen sind Arme von 15 x 35 x 40 mm. Der Thermostatkopf ist ein Drehkörper mit Überwurf, der Hahnblock schließt Verschraubungen und Kappen ein, die beiden Rohre bleiben. Stopfen entfallen (11 mm bei 15 mm Merkmal)."),
    text("Thermal: rear panel and side covers, the front panel as three flat bands of 0.19 m, the fin-pack block as convector (exchange area), grille, brackets with their wall face as mounting face, valve as head, block and pipes; outlet patches 15 mm proud.", "Thermisch: Rückplatte und Seitenblenden, die Frontplatte als drei ebene Bänder von 0,19 m, der Lamellenblock als Konvektor (Austauschfläche), Gitter, Konsolen mit ihrer Wandseite als Montagefläche, Ventil als Kopf, Block und Rohre; Auslassflächen 15 mm über der Oberfläche."),
    text("Structure: the two panels as hollow sheet members (the 19.5 mm water space is above the 15 mm feature), four connection nipples, the two hook joints, the brackets, two wall screws and two wall flanges as supports, the valve spindle joint and the radiator mass; the head mass is merged into the radiator.", "Tragwerk: die beiden Platten als hohle Blechstäbe (der Wasserraum von 19,5 mm liegt über dem Merkmal von 15 mm), vier Verbindungsnippel, die zwei Hakengelenke, die Konsolen, zwei Wandschrauben und zwei Wandflansche als Auflager, das Gelenk der Ventilspindel und die Heizkörpermasse; die Kopfmasse ist im Heizkörper enthalten."),
]


def body200():
    """🏗️ 1:200 body at the 30 mm feature: the radiator as one rounded mass of panels, fins and grille, four bracket blocks, the head, the H-block and the two pipes as one column."""
    return [
        span_box("radiator", (0, K.TOP), (-0.1, 0.0), (-H, H), 0.008, tone=0.65),
        span_box("bracket", (0.461, 0.491), (0.0, W), (0.38, 0.42), role="detail", tone=0.4, repeat={"count": 2, "translation": [-0.42, 0, 0]}),
        span_box("bracket-far", (0.461, 0.491), (0.0, W), (-0.42, -0.38), role="detail", tone=0.4, repeat={"count": 2, "translation": [-0.42, 0, 0]}),
        revolve("valve-head", [(-0.5, 0), (-0.5, 0.026), (-0.595, 0.026), (-0.595, 0)], [0, 0, 1], [0, 0, 0], bone="valve-head", material=PLASTIC, tone=0.85),
        span_box("connection-block", (-0.06, 0.0), K.BLOCK["v"], K.BLOCK["w"], material="brass", role="detail", tone=0.3),
        span_box("pipes", (K.FLOOR_U, -0.06), (-0.065, -0.035), (-0.4725, -0.4075), material="brass", role="detail", tone=0.3),
    ]


def thermal200():
    """🌡️ 1:200 thermal: the merged mass split into rear panel (30 mm), convector core (40 mm) and three front bands (30 mm), the grille as the top 30 mm of the core, brackets, valve, patches 30 mm proud."""
    out = [dict(span_box("th-panel-rear", (0, K.TOP), (-0.03, 0.0), (-H, H)), region="panel-rear")]
    for k, (name, region) in enumerate((("lower", "panel-front-lower"), ("mid", "panel-front-mid"), ("upper", "panel-front-upper"))):
        out.append(dict(span_box("th-band-" + name, (k * 0.2, (k + 1) * 0.2), (-0.1, -0.07), (-H, H)), region=region))
    out += [
        dict(span_box("th-convector", (0, 0.57), (-0.07, -0.03), (-H, H)), region="convector"),
        dict(span_box("th-top-grille", (0.57, K.TOP), (-0.07, -0.03), (-H, H)), region="top-grille"),
        ref("th-bracket", "bracket", region="bracket", sectionFaces=["+v"]),
        ref("th-bracket-far", "bracket-far", region="bracket", sectionFaces=["+v"]),
        ref("th-valve-head", "valve-head", region="valve"),
        ref("th-connection-block", "connection-block", region="valve"),
        ref("th-pipes", "pipes", region="valve"),
    ]
    return out + emitters(0.03)


def structure200():
    """🦴 1:200 structure: the two panels as solid 30 mm plates (their water space is below the feature), four nipples as blocks, the bracket blocks, the four fixings, the valve joint and the mass."""
    out = []
    for sid, v in (("member-panel-rear", (-0.03, 0.0)), ("member-panel-front", (-0.1, -0.07))):
        s = span_box(sid, (0, K.TOP), v, (-H, H), tone=0.6)
        s.update(member="radiator", part="member")
        out.append(s)
    out += nipples(0.015, K.HUB[1] - K.HUB[0], kind="box")
    out += [ref("member-bracket", "bracket", member="radiator", part="member"), ref("member-bracket-far", "bracket-far", member="radiator", part="member")]
    j = span_box("joint-valve-head", (-0.015, 0.015), (-0.015, 0.015), (-0.53, -0.5), bone="valve-head", material="brass", tone=0.5)
    j.update(member="valve-head", part="joint")
    out.append(j)
    for name, w in (("left", 0.4), ("right", -0.4)):
        s = span_box("support-hook-" + name, (0.45, 0.48), (W - 0.015, W + 0.015), (w - 0.015, w + 0.015), tone=0.4)
        s.update(member="radiator", part="support", anchor="hook-" + name)
        p = span_box("support-rest-" + name, (0.035, 0.065), (W - 0.015, W + 0.015), (w - 0.02, w + 0.02), tone=0.4)
        p.update(member="radiator", part="support", anchor="rest-" + name)
        out += [s, p]
    m = ellipsoid("mass-radiator", (0.3, -0.05, 0), (0.03, 0.02, 0.03), tone=0.5)
    m.update(member="radiator", part="mass")
    return out + [m]


DECISIONS200 = [
    text("Body: the panels, fin packs and grille are one rounded mass of 0.6 x 0.1 x 1.0 m, as the drawing of this scale fills it (the 56 mm gap between the panels is 0.28 mm on paper); the upper and lower brackets of each side are 30 mm blocks in the wall gap; the head stays at its real 52 mm; the H-block takes in its unions; the two pipes are one 65 mm column.", "Körper: Platten, Lamellenpakete und Gitter sind eine gerundete Masse von 0,6 x 0,1 x 1,0 m, wie die Zeichnung dieses Maßstabs sie füllt (der Spalt von 56 mm zwischen den Platten misst auf Papier 0,28 mm); die oberen und unteren Konsolen jeder Seite sind 30-mm-Blöcke im Wandabstand; der Kopf bleibt bei seinen echten 52 mm; der Hahnblock schließt die Verschraubungen ein; die beiden Rohre sind eine Säule von 65 mm."),
    text("Thermal: the merged mass is split into the regions it holds: rear panel 30 mm, convector core 40 mm, three front bands of 0.2 m and 30 mm, the grille as the top 30 mm of the core; brackets, valve as head, block and pipe column; outlet patches 30 mm proud.", "Thermisch: die zusammengefasste Masse ist in ihre Regionen geteilt: Rückplatte 30 mm, Konvektorkern 40 mm, drei Frontbänder von 0,2 m und 30 mm, das Gitter als die oberen 30 mm des Kerns; Konsolen, Ventil als Kopf, Block und Rohrsäule; Auslassflächen 30 mm über der Oberfläche."),
    text("Structure: the two panels as solid 30 mm plates (their water space is below the feature), the four nipples as 30 mm blocks, the bracket blocks, wall screws and wall flanges as 30 mm supports, the valve joint and the radiator mass.", "Tragwerk: die beiden Platten als volle 30-mm-Platten (ihr Wasserraum liegt unter dem Merkmal), die vier Nippel als 30-mm-Blöcke, die Konsolblöcke, Wandschrauben und Wandflansche als 30-mm-Auflager, das Ventilgelenk und die Heizkörpermasse."),
]


def volumes():
    """📚 The `volumes` of every scale."""
    return {
        "50": {"status": "authored", "denominator": 50, "body": body50(), "thermal": thermal50(), "structure": structure50(), "decisions": DECISIONS50},
        "100": {"status": "authored", "denominator": 100, "body": body100(), "thermal": thermal100(), "structure": structure100(), "decisions": DECISIONS100},
        "200": {"status": "authored", "denominator": 200, "body": body200(), "thermal": thermal200(), "structure": structure200(), "decisions": DECISIONS200},
        "500": {"status": "unavailable", "denominator": 500},
    }


if __name__ == "__main__":
    with open(SOURCE, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes()}, f, ensure_ascii=False, indent=1)
        f.write("\n")
    v = volumes()
    print("written", SOURCE, {s: {k: len(v[s].get(k, [])) for k in ("body", "thermal", "structure")} for s in ("50", "100", "200")})
