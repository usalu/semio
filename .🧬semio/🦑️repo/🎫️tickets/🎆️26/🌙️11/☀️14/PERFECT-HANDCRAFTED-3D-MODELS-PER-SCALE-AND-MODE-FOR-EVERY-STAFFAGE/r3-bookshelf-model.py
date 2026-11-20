"""📖 Round 3, furniture/bookshelf: every solid of every scale and list, stated by hand from the measures of the kit.

1:50 is the whole flat-pack construction: grooved side panels down to the floor, grooved top and bottom between them, the 3 mm back in the grooves, the recessed kick board, ten shelf halves (the rig bends a shelf about its two supports), twenty 5 mm steel pins with their collars and the eight cam caps. 1:100 is the plain boards. 1:200 and 1:500 merge the carcass into one frame with boards of the smallest feature and one board per shelf.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_bookshelf_kit", os.path.join(HERE, "r3-bookshelf-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

SHELVES = range(1, 6)
ANCHORS = (("foot-left", K.T / 2), ("foot-right", K.W - K.T / 2))


def side(sid, mirror):
    """🪵 A side panel 18 x 300 x 1800 mm standing on the floor, with the 4 x 8 mm groove of the back panel in its inner face."""
    x = (lambda v: K.W - v) if mirror else (lambda v: v)
    inner, floor = K.T, K.T - K.GROOVE_DEPTH
    profile = [(K.ZF, x(0)), (K.ZF, x(inner)), (K.GROOVE_Z0, x(inner)), (K.GROOVE_Z0, x(floor)), (K.GROOVE_Z1, x(floor)), (K.GROOVE_Z1, x(inner)), (K.ZB, x(inner)), (K.ZB, x(0))]
    return K.upright(sid, profile, (0, K.H))


def lid(sid, y0, y1, groove_up):
    """🪵 The top or the bottom panel between the sides, grooved for the back panel on the face that looks into the carcass (`groove_up`: the upper face)."""
    face, floor = (y1, y1 - K.GROOVE_DEPTH) if groove_up else (y0, y0 + K.GROOVE_DEPTH)
    other = y0 if groove_up else y1
    profile = [(other, K.ZF), (face, K.ZF), (face, K.GROOVE_Z0), (floor, K.GROOVE_Z0), (floor, K.GROOVE_Z1), (face, K.GROOVE_Z1), (face, K.ZB), (other, K.ZB)]
    return K.across(sid, profile, (K.T, K.W - K.T))


def shelf_halves(n, thick=K.T, play=K.SHELF_PLAY, z1=K.SHELF_Z1, inner=K.T):
    """📏 The two halves of shelf `n`: each from its support to mid span, the underside on the pins."""
    y = K.SHELF_Y[n - 1]
    left = K.box(f"shelf-{n}-l", f"shelf-{n}-l", K.BOARD, (inner + play, K.W / 2), (y, y + thick), (K.SHELF_Z0, z1))
    right = K.box(f"shelf-{n}-r", f"shelf-{n}-r", K.BOARD, (K.W / 2, K.W - inner - play), (y, y + thick), (K.SHELF_Z0, z1))
    return [left, right]


def pins(n):
    """📍 The four shelf supports of shelf `n`: 5 mm steel pins, 8 mm in the hole and 9 mm under the board, with a 7 mm collar against the panel; front row 37 mm behind the front edge, rear row seven holes (224 mm) further."""
    y = K.SHELF_Y[n - 1] - K.PIN_R
    profile = [(-K.PIN_IN, 0), (-K.PIN_IN, K.PIN_R), (0, K.PIN_R), (0, K.COLLAR_R), (K.COLLAR_T, K.COLLAR_R), (K.COLLAR_T, K.PIN_R), (K.PIN_OUT, K.PIN_R), (K.PIN_OUT, 0)]
    left = K.turned(f"pin-{n}-l", f"shelf-{n}", K.STEEL, profile, (1, 0, 0), (K.T, y, K.PIN_Z0))
    right = K.turned(f"pin-{n}-r", f"shelf-{n}", K.STEEL, profile, (-1, 0, 0), (K.W - K.T, y, K.PIN_Z0))
    return [K.repeat(left, 2, K.PIN_ROWS), K.repeat(right, 2, K.PIN_ROWS)]


def cam_caps():
    """🔩 The eight cam fittings: 15 mm zinc drums let 12.5 mm into the underside of the top and of the bottom, 34 mm from each side panel, their slotted faces 0.5 mm proud."""
    out = []
    for name, y in (("top", K.TOP_Y0), ("bottom", K.BOTTOM_Y0)):
        for s, x in (("l", K.T + K.CAM_X), ("r", K.W - K.T - K.CAM_X)):
            cap = K.turned(f"cam-{name}-{s}", "frame", K.STEEL, [(-K.CAM_DEPTH, 0), (-K.CAM_DEPTH, K.CAM_R), (K.CAM_CAP, K.CAM_R), (K.CAM_CAP, 0)], (0, -1, 0), (x, y, K.CAM_Z0), tone=0.5)
            out.append(K.repeat(cap, 2, K.CAM_STEP))
    return out


def body50():
    """🧱 1:50: the whole construction."""
    out = [side("side-l", False), side("side-r", True), lid("top", K.TOP_Y0, K.H, False), lid("bottom", K.BOTTOM_Y0, K.BOTTOM_Y1, True)]
    out.append(K.box("back", "frame", K.HARDBOARD, (K.T - K.GROOVE_DEPTH + K.BACK_PLAY, K.W - K.T + K.GROOVE_DEPTH - K.BACK_PLAY), (K.BOTTOM_Y1 - K.GROOVE_DEPTH + K.BACK_PLAY, K.TOP_Y0 + K.GROOVE_DEPTH - K.BACK_PLAY), (K.BACK_Z0, K.BACK_Z0 + K.BACK_T)))
    out.append(K.box("plinth", "frame", K.BOARD, (K.T, K.W - K.T), (0, K.BOTTOM_Y0), (K.KICK_Z0, K.KICK_Z1), tone=0.4))
    for n in SHELVES:
        out += shelf_halves(n) + pins(n)
    return out + cam_caps()


def body100():
    """🧱 1:100: the plain boards, the back as a sheet between the sides, no fittings."""
    out = [K.box("side-l", "frame", K.BOARD, (0, K.T), (0, K.H), (K.ZF, K.ZB)), K.box("side-r", "frame", K.BOARD, (K.W - K.T, K.W), (0, K.H), (K.ZF, K.ZB))]
    out.append(K.box("top", "frame", K.BOARD, (K.T, K.W - K.T), (K.TOP_Y0, K.H), (K.ZF, K.ZB)))
    out.append(K.box("bottom", "frame", K.BOARD, (K.T, K.W - K.T), (K.BOTTOM_Y0, K.BOTTOM_Y1), (K.ZF, K.ZB)))
    out.append(K.box("back", "frame", K.HARDBOARD, (K.T, K.W - K.T), (K.BOTTOM_Y1, K.TOP_Y0), (K.BACK_Z0, K.BACK_Z0 + K.BACK_T)))
    out.append(K.box("plinth", "frame", K.BOARD, (K.T, K.W - K.T), (0, K.BOTTOM_Y0), (K.KICK_Z0, K.KICK_Z1), tone=0.4))
    for n in SHELVES:
        out += shelf_halves(n, play=0.0)
    return out


def carcass(t, kick_h):
    """🖼 The merged carcass of a coarse scale: sides, top and bottom as one frame of boards `t` thick, open between the feet for the kick board."""
    outline = [(0, 0), (t, 0), (t, kick_h), (K.W - t, kick_h), (K.W - t, 0), (K.W, 0), (K.W, K.H), (0, K.H)]
    hole = [(t, kick_h + t), (K.W - t, kick_h + t), (K.W - t, K.H - t), (t, K.H - t)]
    return K.facing("carcass", outline, [hole], (K.ZF, K.ZB))


def coarse(t, kick_h):
    """🧱 1:200 and 1:500: the carcass frame, the back as a slab of the smallest feature inside it, the kick board and one board per shelf on its carrier bone."""
    out = [carcass(t, kick_h)]
    out.append(K.box("back", "frame", K.HARDBOARD, (t, K.W - t), (kick_h + t, K.H - t), (K.ZB - t, K.ZB)))
    out.append(K.box("plinth", "frame", K.BOARD, (t, K.W - t), (0, kick_h), (K.KICK_Z0, K.KICK_Z0 + t), tone=0.4))
    for n in SHELVES:
        y = K.SHELF_Y[n - 1]
        out.append(K.box(f"shelf-{n}", f"shelf-{n}", K.BOARD, (t, K.W - t), (y, y + t), (K.SHELF_Z0, K.ZB - t)))
    return out


def body200():
    """🧱 1:200: boards of 30 mm."""
    return coarse(0.03, K.BOTTOM_Y0)


def body500():
    """🧱 1:500: boards of 75 mm."""
    return coarse(0.075, 0.075)


REGION = {"side": "carcass", "top": "carcass", "bottom": "carcass", "carcass": "carcass", "back": "back-panel", "plinth": "plinth", "shelf": "shelves"}


def thermal(scale, body):
    """🌡 The thermal list: every region is the surface of its own boards (one layer each), so each solid references its body board; the steel fittings belong to no region."""
    out = []
    for solid in body:
        key = solid["id"].split("-")[0]
        if key in REGION:
            out.append({"id": "th-" + solid["id"], "of": solid["id"], "region": REGION[key]})
    return out


def of(sid, member, part="member"):
    """🔗 A structure solid that is the body solid."""
    return {"id": ("m-" if part == "member" else "j-") + sid, "of": sid, "member": member, "part": part}


def own(solid, member, part):
    """🏗 A structure solid of its own."""
    return dict(solid, member=member, part=part)


def supports(grow, height):
    """🟢 The contact strip under each side panel: the full 18 x 300 mm floor edge, shown `grow` wider all round."""
    out = []
    for anchor, x in ANCHORS:
        pad = K.box("support-" + anchor, "frame", K.BOARD, (x - K.T / 2 - grow, x + K.T / 2 + grow), (0, height), (K.ZF - grow, K.ZB + grow), tone=0.2, role="ground")
        out.append(own(pad, "frame", "support") | {"anchor": anchor})
    return out


def lump(r):
    """⚫ The mass of the carcass at its centre: mid width, 0.891 m up, in the air of the third compartment."""
    return own(K.ellipsoid("mass-frame", "frame", K.BOARD, (K.W / 2, K.masses()["frame"][1], 0.0), (r, r, r), tone=0.3), "frame", "mass")


def cam_joints():
    """🔩 The eight cam-and-bolt joints of the carcass: a 7 mm steel bolt screwed 11 mm into the side panel and reaching 34 mm into the top or bottom, where the 15 mm cam drum of the body grips its head."""
    out = []
    for name, y0, y1 in (("top", K.TOP_Y0, K.H), ("bottom", K.BOTTOM_Y0, K.BOTTOM_Y1)):
        for s, x, d in (("l", K.T, 1), ("r", K.W - K.T, -1)):
            bolt = K.turned(f"bolt-{name}-{s}", "frame", K.STEEL, [(-K.BOLT_IN, 0), (-K.BOLT_IN, K.BOLT_R), (K.CAM_X, K.BOLT_R), (K.CAM_X, 0)], (d, 0, 0), (x, (y0 + y1) / 2, K.CAM_Z0), tone=0.25)
            out.append(own(K.repeat(bolt, 2, K.CAM_STEP), "frame", "joint"))
    return out


def corner_blocks(size):
    """🟧 The four carcass corners of a coarser scale: a block in the inner corner under the top and on the bottom, at each fitting."""
    out = []
    for name, y in (("top", (K.TOP_Y0 - size, K.TOP_Y0)), ("bottom", (K.BOTTOM_Y1, K.BOTTOM_Y1 + size))):
        for s, x in (("l", (K.T, K.T + size)), ("r", (K.W - K.T - size, K.W - K.T))):
            block = K.box(f"corner-{name}-{s}", "frame", K.STEEL, x, y, (K.CAM_Z0 - size / 2, K.CAM_Z0 + size / 2), tone=0.25, role="detail")
            out.append(own(K.repeat(block, 2, K.CAM_STEP), "frame", "joint"))
    return out


def bearings(size, inner=K.T, rows=True, z1=K.SHELF_Z1):
    """🟧 The shelf supports of a coarser scale: a block under each board end (per pin row, or one cleat through the depth): the member of the carrier bone, on which each shelf half hinges."""
    out = []
    for n in SHELVES:
        y = K.SHELF_Y[n - 1]
        for s, x in (("l", (inner, inner + size)), ("r", (K.W - inner - size, K.W - inner))):
            if rows:
                block = K.repeat(K.box(f"bearing-{n}-{s}", f"shelf-{n}", K.STEEL, x, (y - size, y), (K.PIN_Z0 - size / 2, K.PIN_Z0 + size / 2), tone=0.25, role="detail"), 2, K.PIN_ROWS)
            else:
                block = K.box(f"bearing-{n}-{s}", f"shelf-{n}", K.STEEL, x, (y - size, y), (K.SHELF_Z0, z1), tone=0.25, role="detail")
            out.append(own(block, f"shelf-{n}", "member"))
    return out


def half_members(t):
    """📏 The two beam halves of every shelf of a coarse scale, each on its own bone (the body draws one board)."""
    out = []
    for n in SHELVES:
        for solid in shelf_halves(n, thick=t, play=0.0, z1=K.ZB - t, inner=t):
            solid["id"] = "m-" + solid["id"]
            out.append(own(solid, solid["bone"], "member"))
    return out


def structure(scale, body):
    """🏗 The structural list of a scale: the carcass boards as the rigid frame, the back as its racking brace, the shelf halves as beams on their supports, the fittings as joints, the floor edges as supports and the carcass mass."""
    out = []
    for solid in body:
        sid = solid["id"]
        key = sid.split("-")[0]
        if key in ("side", "top", "bottom", "carcass", "back", "plinth"):
            out.append(of(sid, "frame"))
        elif key == "shelf" and sid.count("-") == 2:
            out.append(of(sid, solid["bone"]))
        elif key == "pin":
            out.append(of(sid, solid["bone"]))
        elif key == "cam":
            out.append(of(sid, "frame", "joint"))
    if scale == "50":
        out += cam_joints() + supports(0.006, 0.006) + [lump(0.03)]
    elif scale == "100":
        out += bearings(0.015) + corner_blocks(0.03) + supports(0.012, 0.012) + [lump(0.04)]
    elif scale == "200":
        out += half_members(0.03) + bearings(0.03, inner=0.03, rows=False, z1=K.ZB - 0.03) + supports(0.02, 0.02) + [lump(0.05)]
    else:
        out += half_members(0.075) + bearings(0.075, inner=0.075, rows=False, z1=K.ZB - 0.075) + supports(0.03, 0.04) + [lump(0.075)]
    return out


DECISIONS = {
    "50": [
        {"en": "Body 1:50 is the flat-pack bookcase as built: two 18 mm side panels standing on the floor, top and bottom between them, each with the 4 x 8 mm groove that holds the 3 mm hardboard back, a kick board set back 30 mm under the bottom, five shelves 762 x 280 mm lying with 1 mm play on four 5 mm steel pins with collars (System 32 rows 37 mm behind the front edge and 224 mm further back, shelf pitch 288 mm), and the drums of the eight cam fittings let into the undersides of the top and the bottom. Each shelf is two halves because the rig bends it about its supports; the 32 mm hole rows (5 mm holes, 0.1 mm on paper) and the edge banding are below the smallest feature and are left out.", "de": "Der Körper 1:50 ist das Regal, wie es gebaut ist: zwei 18-mm-Seiten, die auf dem Boden stehen, Deckel und Boden dazwischen, alle mit der 4 x 8 mm Nut für die 3-mm-Hartfaserrückwand, eine 30 mm zurückgesetzte Sockelblende unter dem Boden, fünf Fachböden 762 x 280 mm mit 1 mm Luft auf je vier 5-mm-Stahlbodenträgern mit Bund (Lochreihen System 32, 37 mm hinter der Vorderkante und 224 mm weiter hinten, Bodenabstand 288 mm) und die Trommeln der acht Exzenterverbinder, eingelassen in die Unterseiten von Deckel und Boden. Jeder Fachboden besteht aus zwei Hälften, weil das Rig ihn um seine Auflager biegt; die 32-mm-Lochreihen (5-mm-Löcher, 0,1 mm auf dem Papier) und die Kanten liegen unter dem Mindestmaß und fehlen."},
        {"en": "Thermal 1:50: carcass, shelves, back panel and kick board are single-layer regions, so each references its own boards; the steel fittings belong to no region. Structure 1:50: the sides, top, bottom and kick board are the rigid frame and the back its racking brace, the ten shelf halves are the beams, the twenty pins are the members of the carrier bones on which the halves hinge, the eight cam drums with their bolts are the carcass joints, the two floor edges of the sides are the supports and the carcass mass sits at its centre 0.89 m up.", "de": "Thermik 1:50: Korpus, Fachböden, Rückwand und Sockelblende sind einschichtige Regionen, jede verweist auf ihre eigenen Platten; die Stahlbeschläge gehören zu keiner Region. Tragwerk 1:50: Seiten, Deckel, Boden und Sockelblende sind der steife Rahmen, die Rückwand seine Aussteifung, die zehn Fachbodenhälften sind die Träger, die zwanzig Bodenträger die Glieder der Trägerknochen, auf denen die Hälften gelenkig liegen, die acht Exzentertrommeln mit ihren Bolzen die Korpusverbindungen, die zwei Bodenkanten der Seiten die Auflager, und die Korpusmasse sitzt in ihrem Schwerpunkt auf 0,89 m."},
    ],
    "100": [
        {"en": "Body 1:100 is drawn anew as plain boards: sides, top, bottom, kick board and the ten shelf halves as boxes without grooves or play, the back as a 3 mm sheet between the sides; pins and cam caps (5 and 15 mm) are below the 15 mm feature and are left out.", "de": "Der Körper 1:100 ist neu als schlichte Platten gezeichnet: Seiten, Deckel, Boden, Sockelblende und die zehn Fachbodenhälften als Quader ohne Nuten und Luft, die Rückwand als 3-mm-Platte zwischen den Seiten; Bodenträger und Exzenterkappen (5 und 15 mm) liegen unter dem Mindestmaß von 15 mm und fehlen."},
        {"en": "Thermal 1:100 references the boards of each region. Structure 1:100: the boards are the frame and the beams, a 15 mm block under each board end at both hole rows stands for the pins, a 30 mm block at each fitting for the cam joints, with the floor strips and the carcass mass.", "de": "Die Thermik 1:100 verweist auf die Platten jeder Region. Tragwerk 1:100: die Platten sind Rahmen und Träger, ein 15-mm-Klotz unter jedem Bodenende an beiden Lochreihen steht für die Bodenträger, ein 30-mm-Klotz an jedem Beschlag für die Exzenterverbindungen, dazu die Bodenstreifen und die Korpusmasse."},
    ],
    "200": [
        {"en": "Body 1:200 is a new abstraction: sides, top and bottom are one carcass frame of 30 mm boards (the smallest feature) with the opening for the kick board between its feet, the back is a 30 mm slab inside it, and every shelf is one 30 mm board on its carrier bone, because the 6 mm sag is 0.03 mm on paper.", "de": "Der Körper 1:200 ist eine neue Abstraktion: Seiten, Deckel und Boden sind ein Korpusrahmen aus 30-mm-Platten (das Mindestmaß) mit der Öffnung für die Sockelblende zwischen seinen Füßen, die Rückwand ist eine 30-mm-Scheibe darin, und jeder Fachboden ist eine 30-mm-Platte auf seinem Trägerknochen, weil die 6 mm Durchbiegung auf dem Papier 0,03 mm sind."},
        {"en": "Thermal 1:200 references the frame, the back, the kick board and the five boards. Structure 1:200: the frame, the back and the kick board, two beam halves per shelf on their own bones, one cleat under each board end for the pin rows, the floor strips and the carcass mass.", "de": "Die Thermik 1:200 verweist auf Rahmen, Rückwand, Sockelblende und die fünf Böden. Tragwerk 1:200: Rahmen, Rückwand und Sockelblende, zwei Trägerhälften je Fachboden auf ihren eigenen Knochen, eine Leiste unter jedem Bodenende für die Lochreihen, die Bodenstreifen und die Korpusmasse."},
    ],
    "500": [
        {"en": "Minimal: the body 1:500 is the carcass frame and the back with 75 mm boards (the smallest feature, 0.15 mm on paper), the kick board in the opening between the feet and five 75 mm shelf boards, so that the sign still reads as an open shelf unit with five shelves; the boards are therefore heavier than the real 18 mm.", "de": "Minimal: der Körper 1:500 ist der Korpusrahmen und die Rückwand mit 75-mm-Platten (das Mindestmaß, 0,15 mm auf dem Papier), die Sockelblende in der Öffnung zwischen den Füßen und fünf 75-mm-Fachböden, damit das Zeichen noch als offenes Regal mit fünf Böden lesbar ist; die Platten sind deshalb stärker als die echten 18 mm."},
        {"en": "Thermal 1:500 references the frame, the back, the kick board and the five boards. Structure 1:500: the frame with the back and the kick board, two beam halves per shelf on one cleat under each board end, the floor strips and the carcass mass.", "de": "Die Thermik 1:500 verweist auf Rahmen, Rückwand, Sockelblende und die fünf Böden. Tragwerk 1:500: der Rahmen mit Rückwand und Sockelblende, zwei Trägerhälften je Fachboden auf einer Leiste unter jedem Bodenende, die Bodenstreifen und die Korpusmasse."},
    ],
}


def volumes():
    """📚 The four authored scales."""
    out = {}
    for scale in ("50", "100", "200", "500"):
        body = globals()["body" + scale]()
        out[scale] = {"status": "authored", "denominator": int(scale), "body": body, "thermal": thermal(scale, body), "structure": structure(scale, body), "decisions": DECISIONS[scale]}
    return out


if __name__ == "__main__":
    print({k: [sum(s.get("repeat", {}).get("count", 1) for s in v[l]) for l in ("body", "thermal", "structure")] for k, v in volumes().items()})
