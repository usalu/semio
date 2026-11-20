"""🧊 Round 3, appliances/laptop: every solid of every authored scale (body, thermal, structure) and the volume decisions, stated by hand in the bone frames of the kit.

1:50 is the real clamshell: bottom cover as a tub with a boat-shaped bilge on four rubber feet, keyboard deck with the rear hinge ledge, the keyboard well and the trackpad opening, 78 key caps in six rows, speaker fields, exhaust grille, ports, lid shell with cover glass and lit panel, hinge barrel. 1:100 and 1:200 are authored anew for their scale.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_laptop_kit", os.path.join(HERE, "r3-laptop-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

ALU, GLASS, RUBBER, KEYCAP, STEEL = "alu-anodised", "display-glass", "rubber-foot", "keycap-plastic", "hinge-steel"
W = K.HALF_W
D = K.DEPTH
C = K.CENTRE_U

TUB_50 = [(0.0020, 0.1500, 0.1025), (0.0026, 0.1535, 0.1060), (0.0036, 0.1555, 0.1080), (0.0050, 0.1563, 0.1088), (0.0062, W, C), (K.SEAM_V, W, C)]
DECK_RW, DECK_RU = W - 0.0003, C - 0.0003
LID_50 = [(K.LID_FACE_V, W, C), (0.0025, W, C), (0.0035, W - 0.0003, C - 0.0003), (0.0041, W - 0.0010, C - 0.0010), (K.LID_BACK_V, W - 0.0022, C - 0.0022)]
BASE_200 = [(0.0, W - 0.003, C - 0.003), (0.003, W, C), (K.DECK_V, W, C)]
LID_200 = [(-0.002, W, C), (K.LID_BACK_V, W, C)]
BASE_100 = [(0.0, 0.1500, 0.1025), (0.0012, 0.1545, 0.1070), (0.0030, 0.1562, 0.1087), (0.0050, W, C), (K.DECK_V, W, C)]
LID_100 = [(K.LID_FACE_V, W, C), (0.0030, W, C), (K.LID_BACK_V, W - 0.0015, C - 0.0015)]
KEYS = [
    ("fn", [(14, K.KEY_UNITS / 14)]),
    ("digits", [(13, 1), (1, 1.5)]),
    ("upper", [(1, 1.5), (13, 1)]),
    ("home", [(1, 1.75), (11, 1), (1, 1.75)]),
    ("lower", [(1, 2.25), (10, 1), (1, 2.25)]),
    ("space", [(3, 1), (1, 1.25), (1, 5), (1, 1.25), (1, 1)]),
]
PLUME_U, PLUME_R = 0.114, 0.1


def keys():
    """⌨ The 78 key caps: six rows from the function row at the hinge to the space row, each group of equal keys one box repeated along the row; the user's left hand is at -w."""
    out = []
    for (name, groups), (u0, u1) in zip(KEYS, K.KEY_ROWS):
        w = -K.KEY_UNITS * K.KEY_PITCH / 2
        for index, (count, units) in enumerate(groups):
            width = units * K.KEY_PITCH
            cap = K.box(f"keys-{name}-{index + 1}", "base", KEYCAP, (u0, u1), (K.BED_V, K.KEY_V), (w + K.KEY_GAP / 2, w + width - K.KEY_GAP / 2), role="texture", texture=True)
            out.append(K.repeated(cap, count, (0, 0, width)) if count > 1 else cap)
            w += count * width
    u0, u1 = K.KEY_ROWS[-1]
    mid, w = (u0 + u1) / 2, (11.5 - K.KEY_UNITS / 2) * K.KEY_PITCH
    lower = K.box("keys-arrow-lower", "base", KEYCAP, (mid + 0.00075, u1), (K.BED_V, K.KEY_V), (w + K.KEY_GAP / 2, w + K.KEY_PITCH - K.KEY_GAP / 2), role="texture", texture=True)
    upper = K.box("keys-arrow-up", "base", KEYCAP, (u0, mid - 0.00075), (K.BED_V, K.KEY_V), (w + K.KEY_PITCH + K.KEY_GAP / 2, w + 2 * K.KEY_PITCH - K.KEY_GAP / 2), role="texture", texture=True)
    return out + [K.repeated(lower, 3, (0, 0, K.KEY_PITCH)), upper]


def underside():
    """🔩 The underside of the bottom cover: two fields of fourteen intake slots under the fans and the eight screws that hold the cover, as sheets 0.3 mm proud of the flat bottom."""
    slot = lambda sid, w: K.repeated(K.box(sid, "base", KEYCAP, (0.040, 0.068), (0.0017, 0.002), (w - 0.0006, w + 0.0006), tone=0.08, role="texture", texture=True), 14, (0, 0, 0.003))
    head = [(0.0017, 0.0015), (0.002, 0.0015)]
    screw = lambda sid, u, w, count, step: K.repeated(K.stud(sid, "base", STEEL, head, (u, 0, w), role="texture", texture=True), count, (0, 0, step))
    return [slot("intake-l", -0.0945), slot("intake-r", 0.0555), screw("screw-rear", 0.012, -0.1, 3, 0.1), screw("screw-mid", C, -0.146, 2, 0.292), screw("screw-front", 0.206, -0.1, 3, 0.1)]


def deck_outline():
    """🗺 The plan outline (w, u) of the keyboard deck: the chassis outline drawn back by the 0.3 mm parting reveal and cut off straight at the hinge ledge."""
    return K.clipped(K.superellipse(C, DECK_RW, DECK_RU, K.PLAN_N, 64), K.LEDGE_U, C, DECK_RW, DECK_RU, K.PLAN_N)


def well():
    """🕳 The keyboard well opening (w, u)."""
    return K.chamfered(-K.WELL[2], K.WELL[2], K.WELL[0], K.WELL[1], 0.002)


def pad_opening():
    """🕳 The trackpad opening (w, u)."""
    return K.chamfered(-K.PAD[2], K.PAD[2], K.PAD[0], K.PAD[1], 0.002)


def body50():
    """🧱 1:50 body."""
    foot = [(0.0, 0.0052), (0.0006, K.FOOT_R), (0.0024, K.FOOT_R)]
    return [
        K.repeated(K.stud("foot-l", "base", RUBBER, foot, (K.FEET_U[0], 0, K.FEET_W), role="ground"), 2, (K.FEET_U[1] - K.FEET_U[0], 0, 0)),
        K.repeated(K.stud("foot-r", "base", RUBBER, foot, (K.FEET_U[0], 0, -K.FEET_W), role="ground"), 2, (K.FEET_U[1] - K.FEET_U[0], 0, 0)),
        K.slab("tub", "base", ALU, C, TUB_50, tone=0.5),
        K.plate("deck", "base", ALU, deck_outline(), (K.SEAM_V, K.DECK_V), holes=[well(), pad_opening()], tone=0.62),
        K.box("keyboard-bed", "base", KEYCAP, (K.WELL[0], K.WELL[1]), (0.008, K.BED_V), (-K.WELL[2], K.WELL[2]), tone=0.16, role="detail"),
        *keys(),
        K.box("trackpad", "base", GLASS, (K.PAD[0], K.PAD[1]), (0.008, K.PAD_V), (-K.PAD[2], K.PAD[2]), tone=0.52, role="detail"),
        *underside(),
        K.repeated(K.box("speaker", "base", ALU, (0.025, 0.113), (K.DECK_V, 0.01265), (-0.1525, -0.1425), tone=0.42, role="texture", texture=True), 2, (0, 0, 0.295)),
        K.box("vent-grille", "base", KEYCAP, (0.0015, 0.0085), (K.SEAM_V, 0.0088), (-0.12, 0.12), tone=0.1, role="texture", texture=True),
        K.box("port-usb-c-r", "base", KEYCAP, (0.02375, 0.03225), (0.0056, 0.0082), (-0.1566, -0.156), tone=0.08, role="texture", texture=True),
        K.box("port-audio-r", "base", KEYCAP, (0.0432, 0.0468), (0.0051, 0.0087), (-0.1566, -0.156), tone=0.08, role="texture", texture=True),
        K.repeated(K.box("port-usb-c-l", "base", KEYCAP, (0.02375, 0.03225), (0.0056, 0.0082), (0.156, 0.1566), tone=0.08, role="texture", texture=True), 2, (0.016, 0, 0)),
        K.slab("lid-shell", "lid", ALU, K.LID_CENTRE_U, LID_50, tone=0.55),
        K.plate("display", "lid", GLASS, K.superellipse((K.GLASS[0] + K.GLASS[1]) / 2, K.GLASS[2], (K.GLASS[1] - K.GLASS[0]) / 2, 60, 48), K.GLASS_V, tone=0.08, role="glazing"),
        K.box("screen", "lid", GLASS, (K.SCREEN[0], K.SCREEN[1]), K.SCREEN_V, (-K.SCREEN[2], K.SCREEN[2]), tone=0.3, role="texture", texture=True),
        K.stud("camera", "lid", GLASS, [(-0.0019, 0.0014), (-0.0017, 0.0014)], (0.2075, 0, 0), tone=0.02, role="texture", texture=True),
        K.axle("hinge", "lid", KEYCAP, [(-K.BARREL_W, 0.004), (-K.BARREL_W + 0.0005, K.BARREL_R), (K.BARREL_W - 0.0005, K.BARREL_R), (K.BARREL_W, 0.004)], tone=0.3),
    ]


def ref(sid, of, **more):
    """🔗 A reference to a body solid."""
    return dict({"id": sid, "of": of}, **more)


def plume(top):
    """♨ The outlet patch of the deck plume: a 0.2 m disc lying on the deck around its anchor, 0.4 mm proud."""
    out = K.plate("th-deck-plume", "base", ALU, K.disc(0, PLUME_U, PLUME_R, 32), (top, top + 0.0004), tone=0.6, role="texture", texture=True)
    out.update({"region": "deck", "tags": ["deck-plume"]})
    return out


def thermal50(body):
    """🌡 1:50 thermal: every body surface under its region; the faces where the two covers and the inlays meet are the joints of one casing."""
    own = {"tub": ("underside", ["+v"]), "deck": ("deck", ["-v"]), "keyboard-bed": ("deck", ["-v"]), "trackpad": ("deck", ["-v"])}
    out = []
    for solid in body:
        sid = solid["id"]
        if sid == "speaker" or sid.startswith(("intake", "screw", "camera")):
            continue
        if sid in own:
            out.append(ref("th-" + sid, sid, region=own[sid][0], sectionFaces=own[sid][1]))
        elif solid["bone"] == "lid":
            out.append(ref("th-" + sid, sid, region="lid"))
        elif sid.startswith(("foot", "port", "vent")):
            out.append(ref("th-" + sid, sid, region="underside"))
        else:
            out.append(ref("th-" + sid, sid, region="deck"))
    return out + [plume(K.DECK_V)]


def member(solid, bone, part="member", **more):
    """🏗 Marks a solid as a part of the structure of a bone."""
    solid.update({"member": bone, "part": part})
    solid.update(more)
    return solid


def support(thickness=0.0005):
    """🟩 The contact pad of the four feet on the desk."""
    return member(K.box("support-feet", "base", RUBBER, (0.014, 0.204), (0, thickness), (-0.14, 0.14), role="ground", tone=0.1), "base", "support", anchor="feet-contact")


def structure50():
    """🏗 1:50 structure: the chassis as a tray (floor, perimeter walls, ribs of the battery bay, keyboard deck plate with its openings) on the four feet, the hinge brackets, the two friction hinges on the axis, the lid as back plate with rim and hinge arms, the two masses."""
    shell = K.chamfered(-0.1555, 0.1555, K.LEDGE_U, 0.2165, 0.006)
    return [
        support(),
        ref("post-foot-l", "foot-l", member="base", part="member"),
        ref("post-foot-r", "foot-r", member="base", part="member"),
        member(K.plate("tray-floor", "base", ALU, K.chamfered(-0.152, 0.152, 0.005, 0.213, 0.006), (0.0024, 0.0044)), "base"),
        member(K.plate("frame-walls", "base", ALU, shell, (0.0040, 0.0105), holes=[K.chamfered(-0.1515, 0.1515, 0.0135, 0.2125, 0.004)]), "base"),
        member(K.box("rib-palm-rest", "base", ALU, (0.1225, 0.1255), (0.0044, 0.0105), (-0.1515, 0.1515)), "base"),
        member(K.repeated(K.box("rib-battery", "base", ALU, (0.1255, 0.2125), (0.0044, 0.0105), (0.0675, 0.0705)), 2, (0, 0, -0.138)), "base"),
        member(K.plate("deck-plate", "base", ALU, shell, (0.0105, K.DECK_V), holes=[well(), pad_opening()]), "base"),
        member(K.repeated(K.box("hinge-bracket", "base", STEEL, (0.003, 0.006), (0.0044, K.HINGE[1]), (0.108, 0.122)), 2, (0, 0, -0.23)), "base"),
        member(K.base("mass-base", "ellipsoid", "base", ALU, role="detail") | {"radii": [0.012, 0.0028, 0.06], "transform": {"translation": [C, 0.0074, 0.0]}}, "base", "mass"),
        member(K.box("back-plate", "lid", ALU, (-0.002, 0.211), (0.0024, K.LID_BACK_V), (-0.154, 0.154)), "lid"),
        member(K.plate("lid-rim", "lid", ALU, K.chamfered(-0.154, 0.154, -0.002, 0.211, 0.006), (-0.001, 0.0024), holes=[K.chamfered(-0.150, 0.150, 0.006, 0.207, 0.004)]), "lid"),
        member(K.repeated(K.box("hinge-arm", "lid", STEEL, (0.0, 0.07), (-0.0006, 0.0024), (0.11, 0.12)), 2, (0, 0, -0.23)), "lid"),
        member(K.repeated(K.axle("joint-hinge", "lid", STEEL, [(0.103, 0.003), (0.127, 0.003)]), 2, (0, 0, -0.23)), "lid", "joint"),
        member(K.base("mass-lid", "ellipsoid", "lid", ALU, role="detail") | {"radii": [0.032, 0.0016, 0.055], "transform": {"translation": [0.1, 0.0008, 0.0]}}, "lid", "mass"),
    ]


def body100():
    """🧱 1:100 body: the chassis as one hull standing on the desk with the key field and the trackpad as inlays, the lid with its glass and the hinge barrel."""
    return [
        K.slab("base", "base", ALU, C, BASE_100, tone=0.58),
        K.box("key-field", "base", KEYCAP, (0.019, 0.119), (K.DECK_V, 0.0127), (-0.138, 0.138), tone=0.25, role="texture", texture=True),
        K.box("trackpad", "base", GLASS, (K.PAD[0], K.PAD[1]), (K.DECK_V, 0.0127), (-K.PAD[2], K.PAD[2]), tone=0.5, role="texture", texture=True),
        K.slab("lid", "lid", ALU, K.LID_CENTRE_U, LID_100, tone=0.55),
        K.box("display", "lid", GLASS, (0.012, 0.209), K.GLASS_V, (-0.152, 0.152), tone=0.08, role="glazing"),
        K.axle("hinge", "lid", KEYCAP, [(-K.BARREL_W, K.BARREL_R), (K.BARREL_W, K.BARREL_R)], tone=0.3),
    ]


def thermal100():
    """🌡 1:100 thermal: the hull split at the parting line of the two covers into underside and deck, the inlays, the lid, the plume outlet."""
    lower = K.slab("th-underside", "base", ALU, C, [row for row in BASE_100 if row[0] < K.SEAM_V] + [(K.SEAM_V, W, C)], tone=0.5)
    upper = K.slab("th-deck", "base", ALU, C, [(K.SEAM_V, W, C), (K.DECK_V, W, C)], tone=0.62)
    return [
        dict(lower, region="underside", sectionFaces=["+v"]),
        dict(upper, region="deck", sectionFaces=["-v"]),
        ref("th-lid", "lid", region="lid"),
        ref("th-display", "display", region="lid"),
        ref("th-hinge", "hinge", region="lid"),
        plume(0.0127),
    ]


def structure100():
    """🏗 1:100 structure: floor, wall frame with one cross rib, deck plate with the keyboard opening; the lid as a frame on two hinges."""
    shell = K.chamfered(-0.1555, 0.1555, K.LEDGE_U, 0.2165, 0.006)
    return [
        support(),
        member(K.plate("tray-floor", "base", ALU, K.chamfered(-0.152, 0.152, 0.005, 0.213, 0.006), (0.0005, 0.0045)), "base"),
        member(K.plate("frame-walls", "base", ALU, shell, (0.0045, K.SEAM_V), holes=[K.chamfered(-0.1495, 0.1495, 0.0155, 0.2105, 0.004)]), "base"),
        member(K.box("rib-palm-rest", "base", ALU, (0.1215, 0.1275), (0.0045, K.SEAM_V), (-0.1495, 0.1495)), "base"),
        member(K.plate("deck-plate", "base", ALU, shell, (K.SEAM_V, K.DECK_V), holes=[well()]), "base"),
        member(K.base("mass-base", "ellipsoid", "base", ALU, role="detail") | {"radii": [0.011, 0.002, 0.06], "transform": {"translation": [C, 0.0065, 0.0]}}, "base", "mass"),
        member(K.plate("lid-frame", "lid", ALU, K.chamfered(-0.154, 0.154, -0.002, 0.211, 0.006), (K.LID_FACE_V, K.LID_BACK_V), holes=[K.chamfered(-0.146, 0.146, 0.008, 0.203, 0.004)]), "lid"),
        member(K.repeated(K.axle("joint-hinge", "lid", STEEL, [(0.1, 0.0035), (0.13, 0.0035)]), 2, (0, 0, -0.23)), "lid", "joint"),
        member(K.base("mass-lid", "ellipsoid", "lid", ALU, role="detail") | {"radii": [0.032, 0.0016, 0.055], "transform": {"translation": [0.1, 0.0008, 0.0]}}, "lid", "mass"),
    ]


def body200():
    """🧱 1:200 body: two plates with the rounded plan corners, the chassis with a chamfered bottom edge, the lid standing with its foot in the rear edge of the chassis."""
    return [
        K.slab("base", "base", ALU, C, BASE_200, tone=0.58),
        K.slab("lid", "lid", ALU, K.LID_CENTRE_U, LID_200, tone=0.5),
        K.box("display", "lid", GLASS, (0.012, 0.209), (-0.0024, -0.002), (-0.152, 0.152), tone=0.08, role="glazing"),
        K.box("key-field", "base", KEYCAP, (0.019, 0.119), (K.DECK_V, 0.0129), (-0.138, 0.138), tone=0.25, role="texture", texture=True),
    ]


def thermal200():
    """🌡 1:200 thermal: the 1.2 mm bottom plate and the 1.2 mm deck plate of the chassis as the shells of their layers, the lid plate, the plume outlet."""
    return [
        dict(K.box("th-underside", "base", ALU, (0.004, D - 0.004), (0, 0.0012), (-W + 0.004, W - 0.004), tone=0.5), region="underside", layer="bottom-shell", sectionFaces=["+v"]),
        dict(K.box("th-deck", "base", ALU, (0.002, D - 0.002), (K.DECK_V - 0.0012, K.DECK_V), (-W + 0.002, W - 0.002), tone=0.62), region="deck", layer="deck-shell", sectionFaces=["-v"]),
        ref("th-lid", "lid", region="lid"),
        ref("th-display", "display", region="lid"),
        plume(0.0129),
    ]


def structure200():
    """🏗 1:200 structure: the chassis as a perimeter frame, the lid as a frame on the hinge axle, two masses, the contact pad."""
    return [
        support(),
        member(K.plate("chassis-frame", "base", ALU, K.chamfered(-W, W, 0, D, 0.008), (0, K.DECK_V), holes=[K.chamfered(-0.1365, 0.1365, 0.02, 0.198, 0.004)]), "base"),
        member(K.base("mass-base", "ellipsoid", "base", ALU, role="detail") | {"radii": [0.03, 0.00625, 0.05], "transform": {"translation": [C, 0.00625, 0.0]}}, "base", "mass"),
        member(K.plate("lid-frame", "lid", ALU, K.chamfered(-W, W, K.LID_U0, K.LID_U1, 0.008), (-0.002, K.LID_BACK_V), holes=[K.chamfered(-0.1415, 0.1415, 0.0105, 0.1985, 0.004)]), "lid"),
        member(K.axle("joint-hinge", "lid", STEEL, [(-0.12, 0.0035), (0.12, 0.0035)]), "lid", "joint"),
        member(K.base("mass-lid", "ellipsoid", "lid", ALU, role="detail") | {"radii": [0.03, 0.00325, 0.05], "transform": {"translation": [0.1, 0.00125, 0.0]}}, "lid", "mass"),
    ]


def body500():
    """🧱 1:500 body: two plain rectangular plates and the dark glass face of the lid."""
    return [
        K.box("base", "base", ALU, (0, D), (0, K.DECK_V), (-W, W), tone=0.58),
        K.box("lid", "lid", ALU, (K.LID_U0, K.LID_U1), (-0.002, K.LID_BACK_V), (-W, W), tone=0.5),
        K.box("display", "lid", GLASS, (0.015, 0.206), (-0.0024, -0.002), (-0.149, 0.149), tone=0.08, role="glazing"),
    ]


def thermal500():
    """🌡 1:500 thermal: the chassis plate halved at mid height into underside and deck, the lid with its glass, the plume outlet as an octagon on the deck."""
    half = K.DECK_V / 2
    outlet = K.plate("th-deck-plume", "base", ALU, K.disc(0, PLUME_U, PLUME_R, 8), (K.DECK_V, K.DECK_V + 0.0004), tone=0.6, role="texture", texture=True)
    outlet.update({"region": "deck", "tags": ["deck-plume"]})
    return [
        dict(K.box("th-underside", "base", ALU, (0, D), (0, half), (-W, W), tone=0.5), region="underside", sectionFaces=["+v"]),
        dict(K.box("th-deck", "base", ALU, (0, D), (half, K.DECK_V), (-W, W), tone=0.62), region="deck", sectionFaces=["-v"]),
        ref("th-lid", "lid", region="lid"),
        ref("th-display", "display", region="lid"),
        outlet,
    ]


def structure500():
    """🏗 1:500 structure: the floor plate of the chassis on the contact pad, the back plate of the lid on one hinge axle, the two masses."""
    return [
        support(),
        member(K.box("floor-plate", "base", ALU, (0.005, D - 0.005), (0.002, 0.006), (-W + 0.005, W - 0.005)), "base"),
        member(K.base("mass-base", "ellipsoid", "base", ALU, role="detail") | {"radii": [0.025, 0.005, 0.045], "transform": {"translation": [C, 0.0074, 0.0]}}, "base", "mass"),
        member(K.box("back-plate", "lid", ALU, (0.0, 0.2105), (0.0015, K.LID_BACK_V), (-W + 0.003, W - 0.003)), "lid"),
        member(K.box("joint-hinge", "lid", STEEL, (-0.0035, 0.0035), (-0.0035, 0.0035), (-0.12, 0.12)), "lid", "joint"),
        member(K.base("mass-lid", "ellipsoid", "lid", ALU, role="detail") | {"radii": [0.025, 0.003, 0.045], "transform": {"translation": [0.1, 0.0008, 0.0]}}, "lid", "mass"),
    ]


def text(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    "50": [
        text(
            "Body: the real clamshell at 1:50. The bottom cover is a tub with a boat-shaped bilge (6.5 mm high, flat bottom 300 x 205 mm) on four round rubber feet of 12 mm; the keyboard deck is a 4 mm plate over it that ends 9.5 mm before the rear edge, so the hinge barrel of 9 mm sits on a ledge with the 1.5 mm exhaust gap under it; the deck has the keyboard well (279 x 104 mm, 1.5 mm deep) with 78 key caps in six rows below the deck plane, the glass trackpad (130 x 78 mm) 0.2 mm deep in its opening, two speaker fields and the exhaust grille on the ledge; the space row ends in the inverted T of the arrow keys; two USB-C ports on the left, one USB-C and the audio jack on the right; the flat bottom carries two fields of fourteen intake slots under the fans and the eight screws of the cover; the camera lens sits in the top rim of the glass; the lid is a 5.9 mm shell with a rounded back edge, the cover glass with a 3 mm rim and the lit 14-inch 16:10 panel (301.6 x 188.5 mm). Closed, nothing touches: 0.3 mm between glass and deck.",
            "Körper: das reale Klappgehäuse bei 1:50. Die Bodenschale ist eine Wanne mit bootsförmiger Kimm (6,5 mm hoch, flacher Boden 300 x 205 mm) auf vier runden Gummifüßen von 12 mm; das Tastaturdeck ist eine 4-mm-Platte darüber, die 9,5 mm vor der Hinterkante endet, sodass die Scharnierwalze von 9 mm auf einem Absatz mit dem 1,5 mm hohen Abluftspalt darunter sitzt; das Deck hat die Tastaturmulde (279 x 104 mm, 1,5 mm tief) mit 78 Tastenkappen in sechs Reihen unter der Deckebene, das Glas-Touchpad (130 x 78 mm) 0,2 mm tief in seiner Öffnung, zwei Lautsprecherfelder und das Abluftgitter auf dem Absatz; die Leertastenreihe endet im umgekehrten T der Pfeiltasten; zwei USB-C-Buchsen links, eine USB-C- und die Klinkenbuchse rechts; der flache Boden trägt zwei Felder mit je vierzehn Ansaugschlitzen unter den Lüftern und die acht Schrauben der Schale; die Kameralinse sitzt im oberen Rand des Glases; der Deckel ist eine 5,9-mm-Schale mit gerundeter Rückkante, das Deckglas mit 3 mm Rand und das leuchtende 14-Zoll-Panel im Format 16:10 (301,6 x 188,5 mm). Geschlossen berührt sich nichts: 0,3 mm zwischen Glas und Deck.",
        ),
        text(
            "Thermal: every body solid is the surface of its region. Underside: the tub, the feet, the ports and the grille; deck: the deck plate, the keyboard bed, the keys and the trackpad; lid: shell, glass, panel and hinge barrel. The faces where tub, deck plate and the two inlays meet are joints of one sealed casing and count for no area. Each region has one aluminium layer, so there are no nested shells; the asset declares no hidden region, so no internals are modelled. The deck plume leaves from a 0.2 m outlet patch lying on the deck.",
            "Thermik: jeder Körper des Bodys ist die Oberfläche seiner Region. Unterseite: Wanne, Füße, Buchsen und Gitter; Deck: Deckplatte, Tastaturbett, Tasten und Touchpad; Deckel: Schale, Glas, Panel und Scharnierwalze. Die Flächen, an denen Wanne, Deckplatte und die zwei Einlagen aufeinandertreffen, sind Fugen eines geschlossenen Gehäuses und zählen nicht zur Fläche. Jede Region hat eine Aluminiumschicht, daher gibt es keine verschachtelten Schalen; das Asset erklärt keine verdeckte Region, daher sind keine Innenteile modelliert. Die Deckfahne geht von einem 0,2 m großen Auslassfeld auf dem Deck aus.",
        ),
        text(
            "Structure: the chassis is a tray: the floor plate on the four feet, the perimeter walls, the cross rib between keyboard bay and battery bay with the two ribs beside the trackpad, and the deck plate with its two openings as the upper flange; two steel brackets rise from the floor to the hinge axis, the two friction hinges are the joints, and the lid is a back plate with a rim and two steel hinge arms. Plates are modelled 2 mm thick (real 1 to 1.2 mm). The masses of 0.82 and 0.48 kg sit at the centres of mass, the contact pad of 190 x 280 mm lies under the feet.",
            "Tragwerk: das Chassis ist eine Wanne: die Bodenplatte auf den vier Füßen, die umlaufenden Wände, die Querrippe zwischen Tastatur- und Akkufach mit den zwei Rippen neben dem Touchpad und die Deckplatte mit ihren zwei Öffnungen als Obergurt; zwei Stahlwinkel steigen vom Boden zur Scharnierachse, die zwei Reibscharniere sind die Gelenke, und der Deckel ist eine Rückplatte mit Rand und zwei stählernen Scharnierarmen. Platten sind 2 mm dick modelliert (real 1 bis 1,2 mm). Die Massen von 0,82 und 0,48 kg sitzen in den Schwerpunkten, das Kontaktpolster von 190 x 280 mm liegt unter den Füßen.",
        ),
    ],
    "100": [
        text(
            "Body: at 1:100 the chassis is one hull that stands on the desk (feet and the 2 mm air gap merged, bilge kept), the keys are one dark key field and the trackpad one inlay, both as sheets on the deck; the lid is one shell with its glass (the lit panel merged into it), and the hinge barrel stays because it joins lid and chassis in every pose. Ports, speaker fields, grille, the hinge ledge and the keyboard well are left out.",
            "Körper: bei 1:100 ist das Chassis ein Rumpf, der auf dem Tisch steht (Füße und 2 mm Luftspalt aufgegangen, Kimm erhalten), die Tasten sind ein dunkles Tastenfeld und das Touchpad eine Einlage, beide als Blätter auf dem Deck; der Deckel ist eine Schale mit ihrem Glas (das leuchtende Panel darin aufgegangen), und die Scharnierwalze bleibt, weil sie Deckel und Chassis in jeder Pose verbindet. Buchsen, Lautsprecherfelder, Gitter, Scharnierabsatz und Tastaturmulde entfallen.",
        ),
        text(
            "Thermal: the hull is split at the parting line of the two covers (8.5 mm) into the underside and the deck, whose meeting faces are a joint; the key field and the trackpad are part of the deck surface; lid, glass and barrel are the lid; the plume outlet lies on the deck.",
            "Thermik: der Rumpf ist an der Trennfuge der zwei Schalen (8,5 mm) in Unterseite und Deck geteilt, deren Berührflächen eine Fuge sind; Tastenfeld und Touchpad gehören zum Deck; Deckel, Glas und Walze sind der Deckel; der Fahnenauslass liegt auf dem Deck.",
        ),
        text(
            "Structure: floor plate, wall frame with the one cross rib and the deck plate with the keyboard opening; the lid is a frame on two hinge joints; back plate, brackets, hinge arms, battery ribs and the feet are merged; two masses and the contact pad.",
            "Tragwerk: Bodenplatte, Wandrahmen mit der einen Querrippe und die Deckplatte mit der Tastaturöffnung; der Deckel ist ein Rahmen an zwei Scharniergelenken; Rückplatte, Winkel, Scharnierarme, Akkurippen und Füße sind aufgegangen; zwei Massen und das Kontaktpolster.",
        ),
    ],
    "200": [
        text(
            "Body: at 1:200 the laptop is two plates with rounded plan corners: the chassis 218 x 12.5 x 313 mm with a chamfered bottom edge and the lid 218 x 6.5 x 313 mm, which stands with its foot in the rear edge of the chassis so that the joint is closed in every pose; the dark glass face of the lid and the dark key field on the deck stay as sheets, because both are several features large; trackpad, barrel and bilge are merged.",
            "Körper: bei 1:200 ist der Laptop zwei Platten mit gerundeten Grundrissecken: das Chassis 218 x 12,5 x 313 mm mit gefaster Unterkante und der Deckel 218 x 6,5 x 313 mm, der mit seinem Fuß in der Hinterkante des Chassis steht, sodass das Gelenk in jeder Pose geschlossen ist; die dunkle Glasfläche des Deckels und das dunkle Tastenfeld auf dem Deck bleiben als Blätter, weil beide mehrere Merkmalsgrößen groß sind; Touchpad, Walze und Kimm sind aufgegangen.",
        ),
        text(
            "Thermal: the 1.2 mm bottom plate and the 1.2 mm deck plate of the chassis as the shells of their layers (their inner faces are joints of the casing), the lid plate with its glass and the plume outlet on the deck.",
            "Thermik: die 1,2 mm dicke Bodenplatte und die 1,2 mm dicke Deckplatte des Chassis als Schalen ihrer Schichten (ihre Innenflächen sind Fugen des Gehäuses), die Deckelplatte mit ihrem Glas und der Fahnenauslass auf dem Deck.",
        ),
        text(
            "Structure: the chassis as a perimeter frame, the lid as a frame on one hinge axle, the two masses and the contact pad.",
            "Tragwerk: das Chassis als umlaufender Rahmen, der Deckel als Rahmen an einer Scharnierachse, die zwei Massen und das Kontaktpolster.",
        ),
    ],
}
DECISIONS["500"] = [
    text(
        "Minimal: at 1:500 the laptop is smaller than three features (313 x 218 mm against 75 mm), so it is two plain rectangular plates, the chassis 218 x 12.5 x 313 mm and the lid 218 x 6.5 x 313 mm standing in the rear edge of the chassis, and the dark glass face of the lid, which alone tells the screen side from the back; corner radii, key field and every other part are merged. The scale has no printed drawing (below the printable feature); the model serves the digital views.",
        "Minimal: bei 1:500 ist der Laptop kleiner als drei Merkmalsgrößen (313 x 218 mm gegenüber 75 mm), daher ist er zwei schlichte rechteckige Platten, das Chassis 218 x 12,5 x 313 mm und der Deckel 218 x 6,5 x 313 mm, der in der Hinterkante des Chassis steht, sowie die dunkle Glasfläche des Deckels, die allein die Bildschirmseite von der Rückseite unterscheidet; Eckradien, Tastenfeld und alle übrigen Teile sind aufgegangen. Der Maßstab hat keine gedruckte Zeichnung (unter der druckbaren Merkmalsgröße); das Modell dient den digitalen Ansichten.",
    ),
    text(
        "Thermal: the chassis plate is halved at mid height into the underside and the deck (the meeting faces are a joint of the casing), the lid with its glass is the lid, and the plume leaves from an octagonal outlet patch on the deck.",
        "Thermik: die Chassisplatte ist auf halber Höhe in Unterseite und Deck geteilt (die Berührflächen sind eine Fuge des Gehäuses), der Deckel mit seinem Glas ist der Deckel, und die Fahne geht von einem achteckigen Auslassfeld auf dem Deck aus.",
    ),
    text(
        "Structure: the floor plate of the chassis on the contact pad, the back plate of the lid on one square hinge axle, and the two masses at the centres of mass.",
        "Tragwerk: die Bodenplatte des Chassis auf dem Kontaktpolster, die Rückplatte des Deckels an einer quadratischen Scharnierachse und die zwei Massen in den Schwerpunkten.",
    ),
]


def volumes():
    """📦 The four scale volumes."""
    b50 = body50()
    return {
        "50": {"status": "authored", "denominator": 50, "body": b50, "thermal": thermal50(b50), "structure": structure50(), "decisions": DECISIONS["50"]},
        "100": {"status": "authored", "denominator": 100, "body": body100(), "thermal": thermal100(), "structure": structure100(), "decisions": DECISIONS["100"]},
        "200": {"status": "authored", "denominator": 200, "body": body200(), "thermal": thermal200(), "structure": structure200(), "decisions": DECISIONS["200"]},
        "500": {"status": "authored", "denominator": 500, "body": body500(), "thermal": thermal500(), "structure": structure500(), "decisions": DECISIONS["500"]},
    }
