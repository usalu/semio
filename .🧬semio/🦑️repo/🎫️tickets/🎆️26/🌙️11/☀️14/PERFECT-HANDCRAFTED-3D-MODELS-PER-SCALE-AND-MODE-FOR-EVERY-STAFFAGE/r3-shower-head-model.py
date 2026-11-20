"""🛁 Round 3, appliances/shower-head: the handcrafted volumes of 1:50, 1:100, 1:200 and the digital 1:500 (body, thermal, structure, decisions), every profile and section stated here by hand; writes `volumes/appliances/shower-head.json`.

Usage: python3 r3-shower-head-model.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "appliances", "shower-head.json")
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-shower-head-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)

CH, ABS, HOSE, RUB, WATER = "chrome", "abs-chrome", "epdm-hose", "rubber-nozzle", "water"
UP, AHEAD, LEFT, RIGHT, DOWN, OUT = [1, 0, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1], [0, -1, 0], [1, 0, 0]
RAIL_O = (0.0, -k.RAIL_X, 0.0)
VALVE_O = (k.VALVE_X, 0.0, 0.0)

HANDLE_50 = [(-0.010, 0.0), (-0.010, 0.0122), (0.016, 0.0135), (0.030, 0.0125), (0.070, 0.0145), (0.105, 0.0135), (0.125, 0.0124), (0.125, 0.0)]
NECK_50 = [(0.110, 0.0, 0.0115, 0.0115), (0.126, -0.0003, 0.0131, 0.0124), (0.142, -0.0010, 0.0170, 0.0105), (0.156, -0.0005, 0.0230, 0.0085), (0.170, -0.0010, 0.0270, 0.0065), (0.184, -0.0030, 0.0260, 0.0045), (0.196, -0.0060, 0.0180, 0.0025)]
CLIP_OUT, CLIP_IN, CLIP_SLOT, CLIP_LOW, CLIP_TOP, CLIP_TAPER = 0.019, 0.014, 62.0, -0.014, 0.016, 0.9
SOCKET_50 = [(CLIP_LOW, 0.0), (CLIP_LOW, CLIP_OUT * CLIP_TAPER), (CLIP_TOP, CLIP_OUT), (CLIP_TOP, 0.0)]
YOKE_50 = [(-0.020, 0.014), (0.020, 0.014), (0.0275, 0.022), (0.0275, 0.050), (0.0255, 0.053), (0.020, 0.053), (0.020, 0.021), (-0.020, 0.021), (-0.020, 0.053), (-0.0255, 0.053), (-0.0275, 0.050), (-0.0275, 0.022)]
YOKE_HALF = 0.010
PIVOT_50 = [(0.016, 0.0), (0.016, 0.0065), (0.0275, 0.0065), (0.0275, 0.0085), (0.0305, 0.0085), (0.0315, 0.0075), (0.0315, 0.0)]
NUT_50 = [(k.NUT_END, 0.0), (k.NUT_END, 0.009), (-0.024, 0.0125), (-0.010, 0.0125), (-0.010, 0.0)]
HEAD_50 = [(-0.016, 0.0), (-0.0145, 0.024), (-0.011, 0.040), (-0.006, 0.052), (-0.002, 0.055), (0.004, 0.055), (0.008, 0.052), (0.010, 0.047), (0.010, 0.0)]
FACE_50 = [(0.0095, 0.0), (0.0095, 0.045), (0.0115, 0.045), (0.0115, 0.0)]
ROSETTE_50 = [(0.0, 0.0), (0.0, k.ROSETTE_R), (0.006, k.ROSETTE_R), (0.010, 0.019), (0.010, 0.0)]
ARM_50 = [(0.008, 0.0), (0.008, 0.011), (0.050, 0.010), (0.050, 0.0)]
SOCKET_LOW_50 = [(0.080, 0.0), (0.0815, 0.009), (0.085, 0.0135), (0.088, k.SOCKET_R), (0.120, k.SOCKET_R), (0.122, 0.013), (0.122, 0.0)]
SOCKET_TOP_50 = [(0.768, 0.0), (0.768, 0.013), (0.770, k.SOCKET_R), (0.802, k.SOCKET_R), (0.805, 0.0135), (0.8085, 0.009), (0.810, 0.0)]
ESCUTCHEON_50 = [(0.0, 0.0), (0.0, 0.030), (0.005, 0.030), (0.009, 0.027), (0.009, 0.0)]
VALVE_50 = [(0.007, 0.0), (0.007, 0.017), (0.044, 0.017), (0.052, 0.0145), (0.058, 0.008), (0.060, 0.0)]
STUB_50 = [(0.0, 0.0), (0.0, 0.0105), (0.020, 0.0105), (0.020, 0.0)]
NUT_ELBOW_50 = [(0.014, 0.0), (0.014, 0.0125), (0.030, 0.0125), (0.040, 0.009), (0.040, 0.0)]
BOSS_50 = [(0.015, 0.0), (0.015, 0.0095), (0.027, 0.0095), (0.028, 0.008), (0.028, 0.0)]
HUB_50 = [(0.027, 0.0), (0.027, 0.008), (0.035, 0.008), (0.036, 0.0065), (0.036, 0.0)]
SLEEVE_50 = [(-0.025, 0.0), (-0.025, 0.015), (-0.023, 0.017), (0.023, 0.017), (0.025, 0.015), (0.025, 0.0)]
KNOB_50 = [(0.015, 0.0), (0.015, 0.011), (0.030, 0.013), (0.034, 0.011), (0.034, 0.0)]

HANDLE_100 = [(k.NUT_END, 0.0), (k.NUT_END, 0.010), (-0.014, 0.0165), (0.014, 0.0185), (0.020, 0.0125), (0.070, 0.0145), (0.120, 0.0125), (0.150, 0.0115), (0.163, 0.008), (0.168, 0.0)]
HEAD_100 = [(-0.016, 0.0), (-0.012, 0.036), (-0.004, 0.055), (0.006, 0.055), (0.010, 0.048), (0.010, 0.0)]
BRACKET_100 = [(0.0, 0.0), (0.0, k.ROSETTE_R), (0.012, k.ROSETTE_R), (0.020, 0.012), (0.050, 0.011), (0.050, 0.0)]
SOCKET_LOW_100 = [(0.080, 0.0), (0.080, k.SOCKET_R), (0.122, k.SOCKET_R), (0.122, 0.0)]
SOCKET_TOP_100 = [(0.768, 0.0), (0.768, k.SOCKET_R), (0.810, k.SOCKET_R), (0.810, 0.0)]
ELBOW_100 = [(0.0, 0.0), (0.0, 0.030), (0.014, 0.030), (0.018, 0.018), (0.050, 0.018), (0.060, 0.010), (0.060, 0.0)]
OUTLET_100 = [(0.0, 0.0), (0.0, 0.0125), (0.032, 0.0125), (0.040, 0.009), (0.040, 0.0)]
SLEEVE_100 = [(-0.025, 0.0), (-0.025, 0.017), (0.025, 0.017), (0.025, 0.0)]
KNOB_100 = [(0.015, 0.0), (0.015, 0.012), (0.034, 0.012), (0.034, 0.0)]

POLE_200 = [(0.080, 0.0), (0.080, 0.015), (0.810, 0.015), (0.810, 0.0)]
STUB_200 = [(0.0, 0.0), (0.0, 0.015), (0.050, 0.015), (0.050, 0.0)]
ELBOW_200 = [(0.0, 0.0), (0.0, 0.030), (0.030, 0.030), (0.030, 0.020), (0.060, 0.020), (0.060, 0.0)]
HANDLE_200 = [(k.NUT_END, 0.0), (k.NUT_END, 0.015), (0.142, 0.015), (0.160, 0.009), (0.160, 0.0)]
HEAD_200 = [(-0.018, 0.0), (-0.018, 0.045), (-0.008, 0.055), (0.012, 0.055), (0.012, 0.0)]

HOSE_COUNTS = {"50": (6, 14, 14, 7), "100": (4, 9, 10, 4), "200": (3, 6, 8, 3), "500": (2, 4, 9, 2)}
HOSE_100, HOSE_200 = 0.0075, 0.015

STAPLE_500 = [(0.0, k.Y_LOW), (0.042, k.Y_LOW), (0.060, k.Y_LOW + 0.018), (0.060, k.Y_TOP - 0.018), (0.042, k.Y_TOP), (0.0, k.Y_TOP)]
ELBOW_500 = [(0.0, 0.0), (0.0, 0.030), (0.018, 0.030), (0.060, 0.019), (0.060, 0.0)]
HEAD_500 = [(-0.016, 0.0), (-0.016, 0.055), (0.014, 0.055), (0.014, 0.0)]


def text(en, de):
    return {"en": en, "de": de}


def rail_rev(i, profile, tone=0.8, **extra):
    return k.revolve(i, "rail", CH, "body", tone, profile, UP, RAIL_O, **extra)


def wall_rev(i, y, profile, tone=0.6, **extra):
    return k.revolve(i, "rail", CH, "body", tone, profile, AHEAD, (y, 0.0, 0.0), **extra)


def of(i, body_id, region, layer=None, **extra):
    d = {"id": i, "of": body_id, "region": region}
    if layer:
        d["layer"] = layer
    d.update(extra)
    return d


def st(solid, member, part, **extra):
    solid.update({"member": member, "part": part})
    solid.update(extra)
    return solid


def supports(low_r, top_r, depth):
    """📌 The pin (a cone into the wall face) under the lower bracket and the roller (a lateral cylinder on the wall face) behind the upper one."""
    pin = st(wall_rev("support-wall-low", k.Y_LOW, [(0.0, 0.0), (0.0, low_r), (depth, low_r * 0.4), (depth, 0.0)], 0.3), "rail", "support", anchor="wall-low")
    pin["role"] = "ground"
    roller = st(k.revolve("support-wall-top", "rail", CH, "ground", 0.3, [(-top_r * 2, 0.0), (-top_r * 2, top_r), (top_r * 2, top_r), (top_r * 2, 0.0)], LEFT, (k.Y_TOP, -top_r, 0.0)), "rail", "support", anchor="wall-top")
    return [pin, roller]


def scale_50():
    body = [
        rail_rev("tube", [(0.110, 0.0), (0.110, k.RAIL_R), (0.780, k.RAIL_R), (0.780, 0.0)]),
        wall_rev("rosette-low", k.Y_LOW, ROSETTE_50),
        wall_rev("arm-low", k.Y_LOW, ARM_50),
        rail_rev("socket-low", SOCKET_LOW_50, 0.6),
        wall_rev("rosette-top", k.Y_TOP, ROSETTE_50),
        wall_rev("arm-top", k.Y_TOP, ARM_50),
        rail_rev("socket-top", SOCKET_TOP_50, 0.6),
        k.revolve("escutcheon", "elbow", CH, "body", 0.7, ESCUTCHEON_50, OUT, (0, 0, 0)),
        k.revolve("valve", "elbow", CH, "body", 0.7, VALVE_50, OUT, (0, 0, 0)),
        k.revolve("outlet", "elbow", CH, "detail", 0.7, STUB_50, DOWN, VALVE_O),
        k.revolve("nut-elbow", "elbow", CH, "detail", 0.75, NUT_ELBOW_50, DOWN, VALVE_O),
        k.revolve("lever-boss", "elbow", CH, "detail", 0.7, BOSS_50, RIGHT, VALVE_O),
        k.revolve("lever-hub", "lever", CH, "detail", 0.85, HUB_50, RIGHT, (0, 0, 0)),
        k.loft("lever", "lever", CH, "detail", 0.85, [(0.002, 0, -0.0315, 0.004, 0.0052, 3), (0.028, 0, -0.0315, 0.004, 0.006, 3), (0.043, 0, -0.0315, 0.004, 0.007, 3), (0.050, 0, -0.0315, 0.004, 0.0055, 3)]),
        k.revolve("holder", "holder", CH, "body", 0.7, SLEEVE_50, [0, 1, 0], (0, 0, 0)),
        k.extrude("holder-arm", "holder", CH, "body", 0.7, "zx", -YOKE_HALF, YOKE_HALF, YOKE_50),
        k.revolve("pivot-left", "holder", CH, "detail", 0.75, PIVOT_50, LEFT, (k.PIVOT_X - k.RAIL_X, 0, 0)),
        k.revolve("pivot-right", "holder", CH, "detail", 0.75, PIVOT_50, RIGHT, (k.PIVOT_X - k.RAIL_X, 0, 0)),
        k.revolve("knob", "holder", CH, "detail", 0.75, KNOB_50, LEFT, (0, 0, 0)),
        k.extrude("socket", "shower", CH, "body", 0.72, "yz", CLIP_LOW, CLIP_TOP, k.clip_profile(CLIP_OUT, CLIP_IN, CLIP_SLOT, 12), taperTo=[CLIP_TAPER, 1]),
        k.revolve("nut", "shower", CH, "detail", 0.75, NUT_50, OUT, (0, 0, 0)),
        k.revolve("handle", "shower", ABS, "body", 0.82, HANDLE_50, OUT, (0, 0, 0)),
        k.loft("neck", "shower", ABS, "body", 0.86, [(u, v, 0, rx, ry) for u, v, rx, ry in NECK_50], rotation=k.ACROSS_U),
        k.revolve("head", "head", ABS, "body", 0.9, HEAD_50, OUT, (0, 0, 0)),
        k.revolve("face", "head", RUB, "detail", 0.4, FACE_50, OUT, (0, 0, 0), texture=True),
        k.hose_loft("hose", HOSE_COUNTS["50"], k.HOSE_R),
    ]
    thermal = [
        *[of("th-" + i, i, "rail", "tube") for i in ("tube", "rosette-low", "arm-low", "socket-low", "rosette-top", "arm-top", "socket-top", "holder", "holder-arm", "pivot-left", "pivot-right", "knob", "socket")],
        *[of("th-" + i, i, "valve", "body") for i in ("escutcheon", "valve", "outlet", "nut-elbow", "lever-boss", "lever-hub", "lever")],
        of("th-hose", "hose", "hose", "rubber"),
        of("th-nut", "nut", "handle", "shell"),
        of("th-handle", "handle", "handle", "shell"),
        of("th-neck", "neck", "head", "shell"),
        of("th-head", "head", "head", "shell"),
        of("th-face", "face", "head", "shell"),
        k.revolve("th-water-valve", "elbow", WATER, "detail", 0.5, [(0.0, 0.0), (0.0, 0.007), (0.046, 0.007), (0.046, 0.0)], OUT, (0, 0, 0), region="spray"),
        k.revolve("th-water-outlet", "elbow", WATER, "detail", 0.5, [(0.0, 0.0), (0.0, 0.006), (0.040, 0.006), (0.040, 0.0)], DOWN, VALVE_O, region="spray"),
        k.hose_loft("th-water-hose", HOSE_COUNTS["50"], k.HOSE_BORE, WATER, 0.5, region="spray"),
        k.revolve("th-water-handle", "shower", WATER, "detail", 0.5, [(k.NUT_END, 0.0), (k.NUT_END, 0.0045), (0.010, 0.006), (0.150, 0.006), (0.172, 0.0045), (0.172, 0.0)], OUT, (0, 0, 0), region="spray"),
        k.revolve("th-water-head", "head", WATER, "detail", 0.5, [(-0.009, 0.0), (-0.009, 0.030), (-0.004, 0.048), (0.007, 0.048), (0.007, 0.0)], OUT, (0, 0, 0), region="spray"),
        k.revolve("th-spray-mist", "head", WATER, "detail", 0.5, [(0.005, 0.0), (0.005, 0.045), (0.0125, 0.045), (0.0125, 0.0)], OUT, (0, 0, 0), region="head", tags=["emitter", "spray-mist"]),
        k.ellipsoid("th-steam-plume", "head", WATER, "detail", 0.5, (0.001, 0.0535, 0.0), (0.006, 0.004, 0.014), region="spray", tags=["emitter", "steam-plume"]),
    ]
    structure = [
        st(rail_rev("member-rail", k.tube_loop(k.Y_LOW, k.Y_TOP, k.RAIL_R, k.RAIL_R - k.RAIL_WALL)), "rail", "member"),
        st(wall_rev("member-bracket-low", k.Y_LOW, [(0.0, 0.0), (0.0, 0.009), (k.RAIL_X, 0.009), (k.RAIL_X, 0.0)]), "rail", "member"),
        st(wall_rev("member-bracket-top", k.Y_TOP, [(0.0, 0.0), (0.0, 0.009), (k.RAIL_X, 0.009), (k.RAIL_X, 0.0)]), "rail", "member"),
        st(k.revolve("member-elbow", "elbow", CH, "body", 0.7, k.tube_loop(0.0, 0.046, 0.0135, 0.007), OUT, (0, 0, 0)), "elbow", "member"),
        st(k.revolve("member-outlet", "elbow", CH, "body", 0.7, k.tube_loop(0.0, 0.026, 0.0105, 0.006), DOWN, VALVE_O), "elbow", "member"),
        {"id": "member-hose", "of": "hose", "member": "elbow", "part": "member"},
        st(k.revolve("member-holder", "holder", CH, "body", 0.7, k.tube_loop(-0.020, 0.020, 0.017, k.RAIL_R), [0, 1, 0], (0, 0, 0)), "holder", "member"),
        {"id": "member-holder-arm", "of": "holder-arm", "member": "holder", "part": "member"},
        st(k.revolve("member-shower", "shower", ABS, "body", 0.82, k.tube_loop(-0.010, 0.122, 0.0105, 0.0085), OUT, (0, 0, 0)), "shower", "member"),
        st(k.loft("member-neck", "shower", ABS, "body", 0.86, [(0.118, 0.0, 0, 0.0095, 0.0095), (0.142, -0.0030, 0, 0.0120, 0.0070), (0.166, -0.0060, 0, 0.0160, 0.0050), (0.186, -0.0100, 0, 0.0160, 0.0040)], rotation=k.ACROSS_U), "shower", "member"),
        st(k.revolve("member-head", "head", ABS, "body", 0.9, [(-0.004, 0.0), (-0.004, 0.048), (0.004, 0.048), (0.004, 0.0)], OUT, (0, 0, 0)), "head", "member"),
        st(k.revolve("joint-lever", "lever", CH, "detail", 0.7, [(0.012, 0.0), (0.012, 0.005), (0.036, 0.005), (0.036, 0.0)], RIGHT, (0, 0, 0)), "lever", "joint"),
        st(k.revolve("joint-holder", "holder", CH, "detail", 0.7, k.tube_loop(-0.004, 0.004, 0.0165, k.RAIL_R), [0, 1, 0], (0, 0, 0)), "holder", "joint"),
        st(k.revolve("joint-shower", "shower", CH, "detail", 0.7, [(-0.0305, 0.0), (-0.0305, 0.0055), (0.0305, 0.0055), (0.0305, 0.0)], LEFT, (0, 0, 0)), "shower", "joint"),
        *supports(0.016, 0.005, 0.010),
        st(k.ellipsoid("mass-rail", "rail", CH, "detail", 0.8, (0.445, -k.RAIL_X, 0.0), (0.020, 0.008, 0.008)), "rail", "mass"),
        st(k.ellipsoid("mass-elbow", "elbow", CH, "detail", 0.7, (0.036, 0.0, 0.0), (0.012, 0.011, 0.011)), "elbow", "mass"),
        st(k.ellipsoid("mass-shower", "shower", ABS, "detail", 0.82, (0.134, -0.008, 0.0), (0.010, 0.0045, 0.006)), "shower", "mass"),
    ]
    decisions = [
        text("Body: a 22 mm chrome rail between two end brackets, each a wall rosette of 44 mm, a round arm and a 30 mm end socket with a domed cap. The sliding holder is a 34 mm sleeve around the rail with a clamp button on the left and a forked arm of 55 by 20 mm; between its two cheeks the socket turns on two pivot pins: a conical clip of 34 to 38 mm with a slot of 14 mm at the front, through which the hose slips in. The hand shower sits with the cone of its handle in that clip, the conical hose nut hanging under it: a round handle of 25 to 29 mm, a neck that flattens and widens from 26 mm to a tongue of 54 mm lying on the back of the spray disc, and the disc of 110 mm, 26 mm thick, with a domed back, a rounded rim and the rubber nozzle field of 90 mm as a texture plate. The wall elbow is a 60 mm escutcheon, a 34 mm valve body with a domed end, the G 1/2 outlet pointing down with the conical hose nut, and a quarter-turn lever with its boss on the right side. The hose of 13 mm hangs as one bight of 1.2 m between the two nuts, lofted through 42 circles.", "Körper: eine verchromte Stange von 22 mm zwischen zwei Endhaltern, jeder mit Wandrosette von 44 mm, rundem Arm und Endhülse von 30 mm mit gewölbter Kappe. Der Gleithalter ist eine Hülse von 34 mm um die Stange mit Klemmknopf links und einem Gabelarm von 55 mal 20 mm; zwischen dessen zwei Wangen dreht die Aufnahme auf zwei Gelenkzapfen: eine konische Klammer von 34 bis 38 mm mit einem Schlitz von 14 mm an der Vorderseite, durch den der Schlauch eingelegt wird. Die Handbrause sitzt mit dem Konus ihres Griffs in dieser Klammer, die konische Schlauchmutter hängt darunter: ein runder Griff von 25 bis 29 mm, ein Hals, der sich von 26 mm zu einer Zunge von 54 mm abflacht und verbreitert und auf dem Rücken der Brausescheibe liegt, und die Scheibe von 110 mm, 26 mm dick, mit gewölbtem Rücken, gerundetem Rand und dem Gummidüsenfeld von 90 mm als Texturplatte. Der Wandanschluss ist eine Rosette von 60 mm, ein Ventilkörper von 34 mm mit gewölbtem Ende, der Abgang G 1/2 nach unten mit der konischen Schlauchmutter und ein Vierteldrehhebel mit Nabe auf der rechten Seite. Der Schlauch von 13 mm hängt als ein Bogen von 1,2 m zwischen den beiden Muttern, durch 42 Kreise geloftet."),
        text("Thermal: every region has one layer, so each surface is its body solid: the rail with brackets and holder, the valve with its outlet nut and lever, the hose, the handle with its hose nut, and the head with the neck that runs into its back. The rail tube is dry; its 1.2 mm wall is the hollow tube of the structure list. The water path of the hidden spray region runs as a core through the valve bore, the outlet, the hose (9 mm bore in the 13 mm hose), the handle and neck, and the chamber behind the nozzle plate. The spray-mist emitter is the nozzle field as a plate 1 mm proud of the face; the vapour plume leaves from a patch on the upper rim of the head.", "Thermik: jede Region hat eine Schicht, daher ist jede Oberfläche ihr Körpervolumen: die Stange mit Haltern und Gleithalter, das Ventil mit Abgangsmutter und Hebel, der Schlauch, der Griff mit seiner Schlauchmutter und der Kopf mit dem Hals, der in seinen Rücken läuft. Das Stangenrohr ist trocken; seine Wand von 1,2 mm ist das Hohlrohr der Tragwerksliste. Der Wasserweg der verdeckten Region Strahl läuft als Kern durch Ventilbohrung, Abgang, Schlauch (9 mm Bohrung im 13-mm-Schlauch), Griff und Hals und die Kammer hinter der Düsenplatte. Der Emitter spray-mist ist das Düsenfeld als Platte 1 mm vor der Fläche; die Dampffahne verlässt den Kopf an einem Fleck am oberen Rand."),
        text("Structure: the load path is the rail tube of 22 by 1.2 mm between the bracket axes, the two bracket studs into the wall with the pin at the lower and the roller at the upper bracket, the clamp ring and the forked arm of the holder, the pivot pin, the handle tube of 21 by 2 mm, the neck spine and the nozzle plate of the hand shower, the brass elbow with its outlet, and the hose hanging from both nuts as a tension member; joints at the lever axis, the sliding ring and the tilt axis; masses in the rail, in the valve body and in the neck of the hand shower.", "Tragwerk: der Lastpfad ist das Stangenrohr 22 mal 1,2 mm zwischen den Halterachsen, die zwei Halterbolzen in die Wand mit dem Festlager am unteren und dem Rollenlager am oberen Halter, Klemmring und Gabelarm des Gleithalters, der Gelenkzapfen, das Griffrohr 21 mal 2 mm, der Halsgrat und die Düsenplatte der Handbrause, der Messingbogen mit seinem Abgang und der Schlauch, der als Zugglied an beiden Muttern hängt; Gelenke an Hebelachse, Gleitring und Schwenkachse; Massen in der Stange, im Ventilkörper und im Hals der Handbrause."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_100():
    body = [
        rail_rev("tube", [(0.115, 0.0), (0.115, k.RAIL_R), (0.775, k.RAIL_R), (0.775, 0.0)]),
        wall_rev("bracket-low", k.Y_LOW, BRACKET_100),
        rail_rev("socket-low", SOCKET_LOW_100, 0.6),
        wall_rev("bracket-top", k.Y_TOP, BRACKET_100),
        rail_rev("socket-top", SOCKET_TOP_100, 0.6),
        k.revolve("elbow", "elbow", CH, "body", 0.7, ELBOW_100, OUT, (0, 0, 0)),
        k.revolve("outlet", "elbow", CH, "body", 0.7, OUTLET_100, DOWN, VALVE_O),
        k.revolve("holder", "holder", CH, "body", 0.7, SLEEVE_100, [0, 1, 0], (0, 0, 0)),
        k.loft("knuckle", "holder", CH, "body", 0.7, [(0.010, 0, 0, 0.013, 0.012, 4), (0.034, 0, 0, 0.019, 0.012, 4), (0.057, 0, 0, 0.019, 0.012, 4)]),
        k.revolve("knob", "holder", CH, "body", 0.75, KNOB_100, LEFT, (0, 0, 0)),
        k.revolve("handle", "shower", ABS, "body", 0.82, HANDLE_100, OUT, (0, 0, 0)),
        k.revolve("head", "head", ABS, "body", 0.9, HEAD_100, OUT, (0, 0, 0)),
        k.hose_loft("hose", HOSE_COUNTS["100"], HOSE_100),
    ]
    thermal = [
        *[of("th-" + i, i, "rail") for i in ("tube", "bracket-low", "socket-low", "bracket-top", "socket-top", "holder", "knuckle", "knob")],
        of("th-elbow", "elbow", "valve"),
        of("th-outlet", "outlet", "valve"),
        of("th-hose", "hose", "hose"),
        of("th-handle", "handle", "handle"),
        of("th-head", "head", "head"),
        k.hose_loft("th-water-hose", HOSE_COUNTS["100"], k.HOSE_BORE, WATER, 0.5, region="spray", slender=True),
        k.revolve("th-water-head", "head", WATER, "detail", 0.5, [(-0.006, 0.0), (-0.006, 0.046), (0.009, 0.046), (0.009, 0.0)], OUT, (0, 0, 0), region="spray"),
        k.revolve("th-spray-mist", "head", WATER, "detail", 0.5, [(-0.0035, 0.0), (-0.0035, 0.045), (0.0115, 0.045), (0.0115, 0.0)], OUT, (0, 0, 0), region="head", tags=["emitter", "spray-mist"]),
        k.ellipsoid("th-steam-plume", "head", WATER, "detail", 0.5, (0.001, 0.050, 0.0), (0.009, 0.0075, 0.016), region="spray", tags=["emitter", "steam-plume"]),
    ]
    structure = [
        st(rail_rev("member-rail", [(k.Y_LOW, 0.0), (k.Y_LOW, k.RAIL_R), (k.Y_TOP, k.RAIL_R), (k.Y_TOP, 0.0)]), "rail", "member"),
        st(wall_rev("member-bracket-low", k.Y_LOW, [(0.0, 0.0), (0.0, 0.009), (k.RAIL_X, 0.009), (k.RAIL_X, 0.0)]), "rail", "member"),
        st(wall_rev("member-bracket-top", k.Y_TOP, [(0.0, 0.0), (0.0, 0.009), (k.RAIL_X, 0.009), (k.RAIL_X, 0.0)]), "rail", "member"),
        st(k.revolve("member-elbow", "elbow", CH, "body", 0.7, [(0.0, 0.0), (0.0, 0.0135), (0.046, 0.0135), (0.046, 0.0)], OUT, (0, 0, 0)), "elbow", "member"),
        {"id": "member-hose", "of": "hose", "member": "elbow", "part": "member"},
        st(k.loft("member-holder", "holder", CH, "body", 0.7, [(-0.015, 0, 0, 0.009, 0.009, 4), (0.045, 0, 0, 0.009, 0.009, 4)]), "holder", "member"),
        st(k.revolve("member-shower", "shower", ABS, "body", 0.82, [(-0.010, 0.0), (-0.010, 0.008), (0.164, 0.008), (0.164, 0.0)], OUT, (0, 0, 0)), "shower", "member"),
        st(k.revolve("joint-holder", "holder", CH, "detail", 0.7, [(-0.0085, 0.0), (-0.0085, 0.0165), (0.0085, 0.0165), (0.0085, 0.0)], [0, 1, 0], (0, 0, 0)), "holder", "joint"),
        st(k.ellipsoid("joint-shower", "shower", CH, "detail", 0.7, (0, 0, 0), (0.009, 0.009, 0.016)), "shower", "joint"),
        *supports(0.018, 0.0075, 0.016),
        st(k.ellipsoid("mass-rail", "rail", CH, "detail", 0.8, (0.445, -k.RAIL_X, 0.0), (0.024, 0.009, 0.009)), "rail", "mass"),
        st(k.ellipsoid("mass-elbow", "elbow", CH, "detail", 0.7, (0.036, 0.0, 0.0), (0.014, 0.012, 0.012)), "elbow", "mass"),
        st(k.ellipsoid("mass-shower", "shower", ABS, "detail", 0.82, (0.134, -0.006, 0.0), (0.012, 0.0075, 0.008)), "shower", "mass"),
    ]
    decisions = [
        text("Body, authored for 15 mm: the rail tube stays at its 22 mm; each bracket is one turned piece (rosette running into the arm) and a plain end socket; the wall elbow is one turned body from the escutcheon to the rounded valve end with the outlet and its nut as one 25 mm cylinder; the holder is the sleeve, the clamp button and one block for arm and hinge; the hand shower is one turned handle that takes in the socket and the hose nut, and a plain lens for the spray disc. The handle ends as a dome in the lower back of the disc; the lever (10 mm), the neck tongue and the nozzle plate are below the feature and left out; the hose is drawn at the feature, 15 mm for its true 13 mm.", "Körper, für 15 mm entworfen: das Stangenrohr bleibt bei seinen 22 mm; jeder Halter ist ein Drehteil (Rosette läuft in den Arm) und eine glatte Endhülse; der Wandanschluss ist ein Drehkörper von der Rosette bis zum gerundeten Ventilende mit Abgang und Mutter als ein Zylinder von 25 mm; der Gleithalter besteht aus Hülse, Klemmknopf und einem Block für Arm und Gelenk; die Handbrause ist ein gedrehter Griff, der Aufnahme und Schlauchmutter einschließt, und eine schlichte Linse für die Brausescheibe. Der Griff endet als Kuppe im unteren Rücken der Scheibe; Hebel (10 mm), Halszunge und Düsenplatte liegen unter dem kleinsten Merkmal und entfallen; der Schlauch ist im kleinsten Merkmal gezeichnet, 15 mm für wahre 13 mm."),
        text("Thermal: the five surface regions are their body solids, the layers are merged into them; the hidden spray region is the water core of the hose and the chamber in the head; the nozzle field is the spray-mist outlet plate and the plume leaves from the upper rim.", "Thermik: die fünf Oberflächenregionen sind ihre Körpervolumen, die Schichten gehen darin auf; die verdeckte Region Strahl ist der Wasserkern des Schlauchs und die Kammer im Kopf; das Düsenfeld ist die Austrittsplatte von spray-mist, die Fahne verlässt den Kopf am oberen Rand."),
        text("Structure: rail as a 22 mm rod between the bracket studs, pin below and roller above, the holder as one bar from the rail to the tilt axis, the handle as a 16 mm rod, the elbow stub and the hanging hose; joints at the sliding ring and the tilt axis; three masses.", "Tragwerk: die Stange als Stab von 22 mm zwischen den Halterbolzen, Festlager unten und Rollenlager oben, der Gleithalter als ein Stab von der Stange zur Schwenkachse, der Griff als Stab von 16 mm, der Anschlussstutzen und der hängende Schlauch; Gelenke an Gleitring und Schwenkachse; drei Massen."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_200():
    body = [
        rail_rev("tube", POLE_200),
        wall_rev("bracket-low", k.Y_LOW - 0.007, STUB_200, 0.8),
        wall_rev("bracket-top", k.Y_TOP + 0.007, STUB_200, 0.8),
        k.revolve("elbow", "elbow", CH, "body", 0.7, ELBOW_200, OUT, (0, 0, 0)),
        k.loft("holder", "holder", CH, "body", 0.7, [(-0.019, 0, 0, 0.019, 0.019, 4), (0.020, 0, 0, 0.019, 0.019, 4), (0.057, 0, 0, 0.017, 0.016, 4)]),
        k.revolve("handle", "shower", ABS, "body", 0.82, HANDLE_200, OUT, (0, 0, 0)),
        k.revolve("head", "head", ABS, "body", 0.9, HEAD_200, OUT, (0, 0, 0)),
        k.hose_loft("hose", HOSE_COUNTS["200"], HOSE_200, into_valve=True),
    ]
    thermal = [
        *[of("th-" + i, i, "rail") for i in ("tube", "bracket-low", "bracket-top", "holder")],
        of("th-elbow", "elbow", "valve"),
        k.hose_loft("th-hose", HOSE_COUNTS["200"], k.HOSE_R, into_valve=True, region="hose", slender=True),
        of("th-handle", "handle", "handle"),
        of("th-head", "head", "head"),
        k.revolve("th-water-head", "head", WATER, "detail", 0.5, [(-0.012, 0.0), (-0.012, 0.040), (0.018, 0.040), (0.018, 0.0)], OUT, (-0.006, 0, 0), region="spray"),
        k.revolve("th-spray-mist", "head", WATER, "detail", 0.5, [(-0.016, 0.0), (-0.016, 0.045), (0.014, 0.045), (0.014, 0.0)], OUT, (0, 0, 0), region="head", tags=["emitter", "spray-mist"]),
        k.ellipsoid("th-steam-plume", "head", WATER, "detail", 0.5, (-0.003, 0.043, 0.0), (0.015, 0.015, 0.02), region="spray", tags=["emitter", "steam-plume"]),
    ]
    structure = [
        st(rail_rev("member-rail", [(k.Y_LOW - 0.007, 0.0), (k.Y_LOW - 0.007, k.RAIL_R), (k.Y_TOP + 0.007, k.RAIL_R), (k.Y_TOP + 0.007, 0.0)], slender=True), "rail", "member"),
        st(k.revolve("member-elbow", "elbow", CH, "body", 0.7, [(0.0, 0.0), (0.0, 0.015), (0.050, 0.015), (0.050, 0.0)], OUT, (0, 0, 0)), "elbow", "member"),
        st(k.loft("member-holder", "holder", CH, "body", 0.7, [(-0.015, 0, 0, 0.015, 0.015, 4), (0.045, 0, 0, 0.015, 0.015, 4)]), "holder", "member"),
        st(k.revolve("member-shower", "shower", ABS, "body", 0.82, [(-0.010, 0.0), (-0.010, 0.004), (0.158, 0.004), (0.158, 0.0)], OUT, (0, 0, 0), slender=True), "shower", "member"),
        st(wall_rev("member-bracket-low", k.Y_LOW - 0.007, [(0.0, 0.0), (0.0, 0.015), (k.RAIL_X, 0.015), (k.RAIL_X, 0.0)]), "rail", "member"),
        st(wall_rev("member-bracket-top", k.Y_TOP + 0.007, [(0.0, 0.0), (0.0, 0.015), (k.RAIL_X, 0.015), (k.RAIL_X, 0.0)]), "rail", "member"),
        *supports(0.015, 0.0075, 0.030),
        st(k.ellipsoid("mass-rail", "rail", CH, "detail", 0.8, (0.445, -k.RAIL_X, 0.0), (0.030, 0.015, 0.015)), "rail", "mass"),
        st(k.ellipsoid("mass-elbow", "elbow", CH, "detail", 0.7, (0.036, 0.0, 0.0), (0.016, 0.015, 0.015)), "elbow", "mass"),
        st(k.ellipsoid("mass-shower", "shower", ABS, "detail", 0.82, (0.140, -0.012, 0.0), (0.016, 0.015, 0.015)), "shower", "mass"),
    ]
    decisions = [
        text("Body, authored for 30 mm: rail, sockets and caps are one pole of 30 mm with two wall stubs; the wall elbow is one stepped turned block (escutcheon, valve and outlet); the holder is one block from the rail to the socket; the hand shower is a 30 mm handle rod with a domed end and a plain disc of 110 by 30 mm; lever, clamp button, nuts and nozzle plate are left out. The hose is drawn at the feature of 30 mm, as the rail is, and keeps its bight.", "Körper, für 30 mm entworfen: Stange, Hülsen und Kappen sind ein Stab von 30 mm mit zwei Wandstutzen; der Wandanschluss ist ein gestufter Drehblock (Rosette, Ventil und Abgang); der Gleithalter ist ein Block von der Stange bis zur Aufnahme; die Handbrause ist ein Griffstab von 30 mm mit gewölbtem Ende und eine schlichte Scheibe von 110 mal 30 mm; Hebel, Klemmknopf, Muttern und Düsenplatte entfallen. Der Schlauch ist wie die Stange im kleinsten Merkmal von 30 mm gezeichnet und behält seinen Bogen."),
        text("Thermal: each surface region is its body solid; the hose surface is the hose at its true 13 mm inside the 30 mm body hose; the spray region is the water chamber inside the head; the spray-mist outlet is the face plate and the plume patch sits on the upper rim.", "Thermik: jede Oberflächenregion ist ihr Körpervolumen; die Schlauchoberfläche ist der Schlauch in seinen wahren 13 mm im 30-mm-Körperschlauch; die Region Strahl ist die Wasserkammer im Kopf; der Austritt von spray-mist ist die Stirnplatte, der Fahnenfleck sitzt am oberen Rand."),
        text("Structure: the rail as a slender rod of its true 22 mm on its two wall stubs, pin below and roller above, the elbow stub, the holder bar and the handle as a slender rod, with three masses; joints are below the feature.", "Tragwerk: die Stange als schlanker Stab ihrer wahren 22 mm auf ihren zwei Wandstutzen, Festlager unten und Rollenlager oben, der Anschlussstutzen, der Halterstab und der Griff als schlanker Stab, mit drei Massen; Gelenke liegen unter dem kleinsten Merkmal."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_500():
    staple = k.bent_bar("rail", "rail", CH, "body", 0.8, [(y, -x) for x, y in STAPLE_500], k.RAIL_R, slender=True)
    body = [
        staple,
        k.revolve("elbow", "elbow", CH, "body", 0.7, ELBOW_500, OUT, (0, 0, 0)),
        k.loft("holder", "holder", CH, "body", 0.7, [(-0.017, 0, 0, 0.020, 0.018, 3), (0.060, 0, 0, 0.020, 0.015, 3)]),
        k.revolve("handle", "shower", ABS, "body", 0.82, [(k.NUT_END, 0.0), (k.NUT_END, 0.012), (0.060, 0.015), (0.160, 0.011), (0.160, 0.0)], OUT, (0, 0, 0)),
        k.revolve("head", "head", ABS, "body", 0.9, HEAD_500, OUT, (0, 0, 0)),
        k.hose_loft("hose", HOSE_COUNTS["500"], k.HOSE_R, into_valve=True, slender=True),
    ]
    thermal = [
        of("th-rail", "rail", "rail"),
        of("th-holder", "holder", "rail"),
        of("th-elbow", "elbow", "valve"),
        of("th-hose", "hose", "hose"),
        of("th-handle", "handle", "handle"),
        of("th-head", "head", "head"),
        k.revolve("th-water-head", "head", WATER, "detail", 0.5, [(-0.008, 0.0), (-0.008, 0.044), (0.008, 0.044), (0.008, 0.0)], OUT, (0, 0, 0), region="spray"),
        k.revolve("th-spray-mist", "head", WATER, "detail", 0.5, [(0.004, 0.0), (0.004, 0.046), (0.016, 0.046), (0.016, 0.0)], OUT, (0, 0, 0), region="head", tags=["emitter", "spray-mist"]),
        k.ellipsoid("th-steam-plume", "head", WATER, "detail", 0.5, (-0.001, 0.047, 0.0), (0.012, 0.011, 0.018), region="spray", tags=["emitter", "steam-plume"]),
    ]
    structure = [
        {"id": "member-rail", "of": "rail", "member": "rail", "part": "member"},
        {"id": "member-holder", "of": "holder", "member": "holder", "part": "member"},
        st(k.revolve("member-elbow", "elbow", CH, "body", 0.7, [(0.0, 0.0), (0.0, 0.012), (0.052, 0.012), (0.052, 0.0)], OUT, (0, 0, 0)), "elbow", "member"),
        st(k.revolve("member-shower", "shower", ABS, "body", 0.82, [(0.0, 0.0), (0.0, 0.004), (0.165, 0.004), (0.165, 0.0)], OUT, (0, 0, 0), slender=True), "shower", "member"),
        *supports(0.014, 0.008, 0.024),
        st(k.ellipsoid("mass-rail", "rail", CH, "detail", 0.8, (0.345, -k.RAIL_X, 0.0), (0.024, 0.016, 0.016)), "rail", "mass"),
        st(k.ellipsoid("mass-shower", "shower", ABS, "detail", 0.82, (0.120, -0.004, 0.0), (0.014, 0.012, 0.012)), "shower", "mass"),
    ]
    decisions = [
        text("Body, a digital miniature below the printable drawing (no part of the set reaches 75 mm across): six solids. Rail and both brackets are one bent bar of the true 22 mm that leaves the wall, runs up 0.69 m and returns to the wall; the wall elbow is one turned cone stump of 60 to 38 mm; the holder is one block from the rail to the socket; the hand shower is a plain handle of 30 mm and a flat disc of 110 by 30 mm on the spray axis; the hose keeps its bight as a slender line of its true 13 mm. End sockets, clamp button, lever, nuts, neck and nozzle plate are left out.", "Körper, eine digitale Miniatur unterhalb der druckbaren Zeichnung (kein Teil der Garnitur erreicht 75 mm): sechs Körper. Stange und beide Halter sind ein gebogener Stab der wahren 22 mm, der die Wand verlässt, 0,69 m hinaufläuft und zur Wand zurückkehrt; der Wandanschluss ist ein gedrehter Kegelstumpf von 60 auf 38 mm; der Gleithalter ist ein Block von der Stange bis zur Aufnahme; die Handbrause ist ein schlichter Griff von 30 mm und eine flache Scheibe von 110 mal 30 mm auf der Strahlachse; der Schlauch behält seinen Bogen als schlanke Linie seiner wahren 13 mm. Endhülsen, Klemmknopf, Hebel, Muttern, Hals und Düsenplatte entfallen."),
        text("Thermal: the five surface regions are these body solids (the holder counts to the rail); the spray region is the water chamber inside the disc; the spray-mist outlet is the face plate and the plume patch sits on the upper rim.", "Thermik: die fünf Oberflächenregionen sind diese Körper (der Gleithalter zählt zur Stange); die Region Strahl ist die Wasserkammer in der Scheibe; der Austritt von spray-mist ist die Stirnplatte, der Fahnenfleck sitzt am oberen Rand."),
        text("Structure: the bent rail bar itself between pin and roller, the holder block, the elbow stub, the hand shower as one slender rod from the tilt axis to the back of the disc, and two masses: rail with elbow, and hand shower.", "Tragwerk: der gebogene Stangenstab selbst zwischen Festlager und Rollenlager, der Halterblock, der Anschlussstutzen, die Handbrause als ein schlanker Stab von der Schwenkachse zum Rücken der Scheibe und zwei Massen: Stange mit Anschluss sowie Handbrause."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def main():
    volumes = {"50": scale_50(), "100": scale_100(), "200": scale_200(), "500": scale_500()}
    json.dump({"volumes": volumes}, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("written", SOURCE, {s: (len(v.get("body", [])), len(v.get("thermal", [])), len(v.get("structure", []))) for s, v in volumes.items()})


if __name__ == "__main__":
    main()
