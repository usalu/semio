"""🧱 Round 3, furniture/bar-stool: every body, thermal and structure solid of every authored scale, stated by hand from the construction in `r3-bar-stool-kit.py`.
"""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-bar-stool-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

FOUR = {"count": 4, "rotationAxis": [0.0, 1.0, 0.0], "rotationRad": 1.570796, "origin": [0.0, 0.0, 0.0]}
FOUR_SEAT = {"count": 4, "rotationAxis": [1.0, 0.0, 0.0], "rotationRad": 1.570796, "origin": [0.0, 0.0, 0.0]}


def rep(solid, repeat=FOUR):
    solid["repeat"] = dict(repeat)
    return solid


def cushion_profile(scale):
    """🛋 Half profile (world height, radius) of the upholstered cushion: cover over 40 mm foam on the plywood board, wrapped round the board edge and stapled underneath."""
    bottom = K.SEAT_JOINT_Y
    if scale == 50:
        pts = [(bottom, 0.0), (bottom, 0.175)] + K.arc(bottom + 0.005, 0.175, 0.005, -90, 0, 3)[1:] + [(0.730, K.SEAT_R)] + K.arc(0.730, 0.16, 0.02, 0, 90, 6)[1:] + [(0.7510, 0.10), (K.SEAT_TOP, 0.0)]
    elif scale == 100:
        pts = [(bottom, 0.0), (bottom, 0.177), (bottom + 0.003, K.SEAT_R), (0.730, K.SEAT_R)] + K.arc(0.730, 0.16, 0.02, 0, 90, 3)[1:] + [(K.SEAT_TOP, 0.0)]
    else:
        pts = [(bottom, 0.0), (bottom, K.SEAT_R), (0.732, K.SEAT_R), (0.746, 0.174), (K.SEAT_TOP - 0.001, 0.158), (K.SEAT_TOP, 0.0)]
    return pts


def foam_profile():
    """🧽 The foam inside the cover at 1:50: on the board top, 1.5 mm inside the cover everywhere else."""
    t = K.COVER_T
    board_top = K.SEAT_JOINT_Y + K.BOARD_T
    return [(board_top, 0.0), (board_top, K.SEAT_R - t), (0.730, K.SEAT_R - t)] + K.arc(0.730, 0.16, 0.02 - t, 0, 90, 6)[1:] + [(0.7495, 0.10), (K.SEAT_TOP - t, 0.0)]


def glide_profile():
    return [(0.0, 0.0), (0.0, 0.0135), (0.0015, K.GLIDE_R), (0.019, K.GLIDE_R), (K.GLIDE_H, 0.012), (K.GLIDE_H, 0.0)]


def pan_profile():
    """🥘 The pressed 2 mm steel seat pan with its 20 mm turned-down rim, open below, the legs welded inside the rim."""
    top, r, t, rim = K.PAN_TOP, K.PAN_R, K.PAN_T, K.PAN_RIM
    return [(top, 0.0), (top, r - 0.006)] + K.arc(top - 0.006, r - 0.006, 0.006, 90, 0, 3)[1:] + [(top - rim, r), (top - rim, r - t), (top - 0.006, r - t)] + K.arc(top - 0.006, r - 0.006, 0.006 - t, 0, 90, 2)[1:] + [(top - t, 0.0)]


def swivel_profile(scale):
    """🔄 Ball-bearing swivel plate (lazy-susan type): two pressed plates round a raised ball race, screwed under the board."""
    top = K.SEAT_JOINT_Y
    if scale == 50:
        return [(top - 0.012, 0.0), (top - 0.012, K.SWIVEL_R), (top - 0.009, K.SWIVEL_R), (top - 0.009, K.SWIVEL_RACE_R), (top - 0.003, K.SWIVEL_RACE_R), (top - 0.003, K.SWIVEL_R), (top, K.SWIVEL_R), (top, 0.0)]
    return [(top - 0.015, 0.0), (top - 0.015, K.SWIVEL_R), (top, K.SWIVEL_R), (top, 0.0)]


def board(solid_id, scale, thick=K.BOARD_T):
    """🪵 The plywood seat board under the foam: arrised edges at 1:50, a plain disc coarser, its radius generalised with the cover wrap of the scale."""
    bottom = K.SEAT_JOINT_Y
    top = bottom + thick
    if scale == 50:
        e = 0.002
        pts = [(bottom, 0.0), (bottom, K.BOARD_R - e), (bottom + e, K.BOARD_R), (top - e, K.BOARD_R), (top, K.BOARD_R - e), (top, 0.0)]
    else:
        r = {100: K.BOARD_R, 200: 0.174, 500: 0.17}[scale]
        pts = [(bottom, 0.0), (bottom, r), (top, r), (top, 0.0)]
    return K.revolve_y(solid_id, "seat", "birch-plywood", "body", 0.6, (0, 0, 0), pts)


def body(scale):
    """🪑 The body list of one scale."""
    out = []
    if scale in (50, 100):
        foot = K.LEG_FOOT if scale == 50 else (K.FOOT_R, K.TUBE_R)
        out.append(rep(K.capsule("leg", "frame", "chrome-steel", "body", 0.7, (K.LEG_TOP[0], K.LEG_TOP[1], 0), (foot[0], foot[1], 0), K.TUBE_R)))
    else:
        leg = rep(K.capsule("leg", "frame", "chrome-steel", "body", 0.7, (0.146, 0.676, 0), (K.FOOT_R, K.TUBE_R, 0), K.TUBE_R))
        if scale == 500:
            leg["slender"] = True
        out.append(leg)
    if scale == 50:
        out.append(rep(K.revolve_y("glide", "frame", "rubber", "body", 0.15, (K.FOOT_R, 0, 0), glide_profile())))
    ring_n = {50: 24, 100: 12, 200: 10, 500: 8}[scale]
    ring = K.revolve_y("ring", "frame", "chrome-steel", "body", 0.7, (0, K.RING_Y, 0), [(K.RING_Y + a, r) for a, r in K.loop(0.0, K.RING_R, K.TUBE_R / math.cos(math.pi / ring_n), ring_n)])
    out.append(ring)
    if scale == 50:
        out.append(K.revolve_y("pan", "frame", "chrome-steel", "body", 0.55, (0, 0, 0), pan_profile()))
        out.append(K.revolve_y("swivel", "seat", "chrome-steel", "body", 0.35, (0, 0, 0), swivel_profile(50)))
    elif scale == 100:
        top = K.SEAT_JOINT_Y - 0.015
        out.append(K.revolve_y("pan", "frame", "chrome-steel", "body", 0.55, (0, 0, 0), [(top - 0.017, 0.0), (top - 0.017, K.PAN_R), (top - 0.004, K.PAN_R), (top, K.PAN_R - 0.004), (top, 0.0)]))
        out.append(K.revolve_y("swivel", "seat", "chrome-steel", "body", 0.35, (0, 0, 0), swivel_profile(100)))
    elif scale == 200:
        out.append(K.revolve_y("mount", "frame", "chrome-steel", "body", 0.55, (0, 0, 0), [(0.666, 0.0), (0.666, K.PAN_R), (K.SEAT_JOINT_Y, K.PAN_R), (K.SEAT_JOINT_Y, 0.0)]))
    if scale in (50, 100, 200):
        out.append(K.revolve_y("cushion", "pad", "pu-leatherette", "body", 0.3, (0, 0, 0), cushion_profile(scale)))
    else:
        out.append(K.revolve_y("seat", "seat", "pu-leatherette", "body", 0.3, (0, 0, 0), [(0.666, 0.0), (0.666, K.PAN_R), (0.690, K.SEAT_R), (0.728, K.SEAT_R), (0.746, 0.17), (K.SEAT_TOP, 0.14), (K.SEAT_TOP, 0.0)]))
    if scale == 50:
        piping = K.revolve_y("piping", "pad", "pu-leatherette", "detail", 0.2, (0, 0, 0), [(K.PAD_JOINT_Y + a, r) for a, r in K.loop(0.7263 - K.PAD_JOINT_Y, 0.1763, 0.003, 10)])
        piping["slender"] = True
        out.append(piping)
    if scale == 50:
        out.append(K.extrude_xy("back-bar", "backrest", "chrome-steel", "body", 0.7, K.bar_outline(K.BAR_T), -K.BAR_W / 2, K.BAR_W / 2))
        out.append(K.arc_loft("back-pad", "backrest", "pu-leatherette", "body", 0.3, K.BACK_BEND_R, (K.BACK_Y0 + K.BACK_Y1) / 2, (K.BACK_Y1 - K.BACK_Y0) / 2, K.BACK_HALF_T, K.BACK_HALF_RAD, 9, 0.012, 3.5, K.BACK_CENTRE_X))
    elif scale == 100:
        out.append(K.extrude_xy("back-bar", "backrest", "chrome-steel", "body", 0.7, K.bar_outline(0.015, top_y=0.83, bend=0.0125), -K.BAR_W / 2, K.BAR_W / 2))
        out.append(K.arc_loft("back-pad", "backrest", "pu-leatherette", "body", 0.3, K.BACK_BEND_R, (K.BACK_Y0 + K.BACK_Y1) / 2, (K.BACK_Y1 - K.BACK_Y0) / 2, K.BACK_HALF_T, K.BACK_HALF_RAD, 5, 0.012, 3.5, K.BACK_CENTRE_X))
    elif scale == 200:
        out.append(K.extrude_xy("back-bar", "backrest", "chrome-steel", "body", 0.7, K.bar_outline(0.03, start_x=-0.11, top_y=0.82, bend=0.016), -K.BAR_W / 2, K.BAR_W / 2))
        out.append(K.arc_loft("back-pad", "backrest", "pu-leatherette", "body", 0.3, K.BACK_BEND_R, (K.BACK_Y0 + K.BACK_Y1) / 2, (K.BACK_Y1 - K.BACK_Y0) / 2, 0.015, K.BACK_HALF_RAD, 4, 0.012, 3.0, K.BACK_CENTRE_X))
    else:
        out.append(K.arc_loft("back", "backrest", "pu-leatherette", "body", 0.3, 0.1775, 0.79, 0.08, 0.0375, 0.78, 5, 0.03, 3.0))
    return out


def thermal(scale):
    """🌡 The thermal list: the regions on the body surfaces, the hidden plywood board as its own solid, nested foam shells at 1:50."""
    out = [
        {"id": "th-leg", "of": "leg", "region": "legs", "layer": "leg-wall"},
        {"id": "th-ring", "of": "ring", "region": "footrest-ring", "layer": "ring-wall"},
    ]
    if scale == 50:
        out.append({"id": "th-glide", "of": "glide", "region": "legs", "layer": "leg-wall"})
    if scale in (50, 100, 200):
        out.append({"id": "th-back-bar", "of": "back-bar", "region": "legs", "layer": "leg-wall"})
    if scale in (50, 100):
        out += [{"id": "th-pan", "of": "pan", "region": "seat-mount", "layer": "mount-steel"}, {"id": "th-swivel", "of": "swivel", "region": "seat-mount", "layer": "mount-steel"}]
    elif scale == 200:
        out.append({"id": "th-mount", "of": "mount", "region": "seat-mount", "layer": "mount-steel"})
    else:
        out.append(K.revolve_y("th-mount", "seat", "chrome-steel", "body", 0.55, (0, 0, 0), [(0.666, 0.0), (0.666, K.PAN_R), (K.SEAT_JOINT_Y, K.PAN_R), (K.SEAT_JOINT_Y, 0.0)]) | {"region": "seat-mount", "layer": "mount-steel"})
    plate = board("th-board", scale) | {"region": "seat-plate", "layer": "plate-ply"}
    out.append(plate)
    if scale == 50:
        out.append({"id": "th-cushion-cover", "of": "cushion", "region": "seat-pad", "layer": "leatherette"})
        out.append(K.revolve_y("th-cushion-foam", "pad", "hr-foam", "body", 0.8, (0, 0, 0), foam_profile()) | {"region": "seat-pad", "layer": "pad-foam"})
        out.append({"id": "th-back-cover", "of": "back-pad", "region": "backrest", "layer": "leatherette"})
        foam = K.arc_loft("th-back-foam", "backrest", "hr-foam", "body", 0.8, K.BACK_BEND_R, (K.BACK_Y0 + K.BACK_Y1) / 2, (K.BACK_Y1 - K.BACK_Y0) / 2 - K.COVER_T, K.BACK_HALF_T - K.COVER_T, K.BACK_HALF_RAD - K.COVER_T / K.BACK_BEND_R, 9, 0.0105, 3.5, K.BACK_CENTRE_X)
        out.append(foam | {"region": "backrest", "layer": "back-foam"})
    elif scale in (100, 200):
        out += [{"id": "th-cushion", "of": "cushion", "region": "seat-pad"}, {"id": "th-back-pad", "of": "back-pad", "region": "backrest"}]
    else:
        out += [{"id": "th-seat", "of": "seat", "region": "seat-pad"}, {"id": "th-back", "of": "back", "region": "backrest"}]
    return out


def support(anchor, x, z, r, h):
    s = K.revolve_y(f"support-{anchor}", "frame", "rubber", "ground", 0.2, (x, 0, z), [(0.0, 0.0), (0.0, r), (h, r), (h, 0.0)])
    return s | {"member": "frame", "part": "support", "anchor": anchor}


def structure(scale):
    """🏗 The load path: tube legs and ring welded to the seat pan, the swivel bearing, the plywood board, the foam cushion, the bent back bar; welds and bearings as joints, a contact pad under every glide, masses at the centres of mass."""
    feature = 0.15 * scale / 1000
    out = [{"id": "m-leg", "of": "leg", "member": "frame", "part": "member"}, {"id": "m-ring", "of": "ring", "member": "frame", "part": "member"}]
    if scale == 50:
        out.append({"id": "m-glide", "of": "glide", "member": "ground", "part": "member"})
    else:
        g = max(feature, 0.02)
        out.append(rep(K.revolve_y("m-foot", "frame", "rubber", "detail", 0.2, (K.FOOT_R, 0, 0), [(0.0, 0.0), (0.0, max(K.GLIDE_R, g / 2)), (g, max(K.GLIDE_R, g / 2)), (g, 0.0)])) | {"member": "ground", "part": "member"})
    if scale in (50, 100):
        out += [{"id": "m-pan", "of": "pan", "member": "frame", "part": "member"}, {"id": "j-swivel", "of": "swivel", "member": "seat", "part": "joint"}]
    elif scale == 200:
        out.append({"id": "m-mount", "of": "mount", "member": "frame", "part": "member"})
    m_board = board("m-board", scale, max(K.BOARD_T, feature)) | {"member": "seat", "part": "member"}
    if scale == 500:
        m_board = K.revolve_y("m-board", "seat", "birch-plywood", "body", 0.6, (0, 0, 0), [(0.666, 0.0), (0.666, 0.15), (0.666 + feature, 0.15), (0.666 + feature, 0.0)]) | {"member": "seat", "part": "member"}
    out.append(m_board)
    if scale in (50, 100, 200):
        out.append({"id": "m-cushion", "of": "cushion", "member": "pad", "part": "member"})
        out.append({"id": "m-back-bar", "of": "back-bar", "member": "backrest", "part": "member"})
    else:
        out.append(K.revolve_y("m-cushion", "pad", "hr-foam", "body", 0.8, (0, 0, 0), [(K.SEAT_TOP - feature - 0.002, 0.0), (K.SEAT_TOP - feature - 0.002, 0.16), (K.SEAT_TOP - 0.002, 0.16), (K.SEAT_TOP - 0.002, 0.0)]) | {"member": "pad", "part": "member"})
        out.append({"id": "m-back", "of": "back", "member": "backrest", "part": "member"})
    if scale in (50, 100):
        wr = 0.016 if scale == 50 else feature
        out.append(rep(K.ellipsoid("j-weld-top", "frame", "chrome-steel", "detail", 0.4, (K.LEG_TOP[0], K.PAN_TOP - K.PAN_T - 0.006, 0), (wr, 0.012 if scale == 50 else feature, wr))) | {"member": "frame", "part": "joint"})
        ry = K.leg_radius_at(K.RING_Y)
        out.append(rep(K.ellipsoid("j-weld-ring", "frame", "chrome-steel", "detail", 0.4, ((ry + K.RING_R) / 2, K.RING_Y, 0), (wr, wr, wr))) | {"member": "frame", "part": "joint"})
        out.append(K.ellipsoid("j-back-bolts", "backrest", "chrome-steel", "detail", 0.4, (K.BACK_JOINT[0], K.BACK_JOINT[1], 0), (0.03, 0.006 if scale == 50 else feature / 2, 0.02)) | {"member": "backrest", "part": "joint"})
    if scale == 50:
        out.append(K.revolve_y("j-pad-foam", "pad", "hr-foam", "detail", 0.4, (0, 0, 0), [(K.PAD_JOINT_Y - 0.004, 0.0), (K.PAD_JOINT_Y - 0.004, 0.12), (K.PAD_JOINT_Y + 0.004, 0.12), (K.PAD_JOINT_Y + 0.004, 0.0)]) | {"member": "pad", "part": "joint"})
    pad_r = K.GLIDE_R if scale <= 100 else max(K.GLIDE_R, feature / 2)
    pad_h = max(0.004, feature / 2) if scale > 50 else 0.0075
    for anchor, x, z in (("foot-xp", K.FOOT_R, 0.0), ("foot-xn", -K.FOOT_R, 0.0), ("foot-zp", 0.0, K.FOOT_R), ("foot-zn", 0.0, -K.FOOT_R)):
        out.append(support(anchor, x, z, pad_r, pad_h))
    _, frame_y = K.frame_mass_centre()
    mr = {50: 0.02, 100: 0.025, 200: 0.035, 500: 0.0375}[scale]
    out.append(K.ellipsoid("mass-frame", "frame", "chrome-steel", "detail", 0.4, (0, frame_y, 0), (mr, mr, mr)) | {"member": "frame", "part": "mass"})
    if scale <= 200:
        out.append(K.ellipsoid("mass-seat", "seat", "birch-plywood", "detail", 0.4, (0, K.SEAT_JOINT_Y + 0.004, 0), (max(mr / 2, feature / 2 if scale > 50 else 0.008), mr, mr)) | {"member": "seat", "part": "mass"})
        out.append(K.ellipsoid("mass-pad", "pad", "hr-foam", "detail", 0.4, (0, 0.735, 0), (mr, max(mr / 2, feature / 2 if scale > 50 else 0.008), mr)) | {"member": "pad", "part": "mass"})
        out.append(K.ellipsoid("mass-backrest", "backrest", "pu-leatherette", "detail", 0.4, (-0.199, 0.825, 0), (mr, max(0.01, feature / 2), mr)) | {"member": "backrest", "part": "mass"})
    else:
        out.append(K.ellipsoid("mass-seat", "seat", "birch-plywood", "detail", 0.4, (0, 0.715, 0), (mr, mr, mr)) | {"member": "seat", "part": "mass"})
    return out


DECISIONS = {
    50: [
        K.text("1:50 is the full construction of a classic swivel bar stool: four 25 x 1.5 mm chromed tubes splayed 4.7 degrees from the 0.144 m pan radius to the 0.1975 m foot circle, each standing in a domed 30 mm rubber glide; the footrest ring (25 mm tube bent to 0.40 m) welded to the outside of the legs at 0.27 m; a pressed 2 mm steel seat pan with a 20 mm turned-down rim into which the legs are welded; a 12 mm ball-bearing swivel plate with its raised race; the upholstered seat (12 mm birch plywood board, 40 mm foam, leatherette wrapped round the board edge and stapled underneath) as a revolve with a crowned top, a 20 mm top round and piping at the top seam; a bent 40 x 8 mm flat back bar screwed under the board and rising 10 mm behind the cushion into a curved back pad (0.30 m wide, 90 mm high, 26 mm thick) bent on a 0.30 m radius with rounded ends.", "1:50 ist die volle Konstruktion eines klassischen Drehbarhockers: vier verchromte Rohre 25 x 1,5 mm, um 4,7 Grad gespreizt vom Pfannenradius 0,144 m zum Fußkreis 0,1975 m, jedes in einem gewölbten 30-mm-Gummigleiter; der Fußring (25-mm-Rohr, auf 0,40 m gebogen) außen an die Beine auf 0,27 m geschweißt; eine gepresste 2-mm-Stahlpfanne mit 20 mm abgekantetem Rand, in die die Beine geschweißt sind; ein 12-mm-Kugeldrehteller mit erhabener Laufbahn; der Polstersitz (12 mm Birkensperrholz, 40 mm Schaum, Kunstleder um die Plattenkante geschlagen und unten getackert) als Drehkörper mit gewölbter Oberseite, 20-mm-Rundung und Keder an der oberen Naht; ein gebogener Flachstahl 40 x 8 mm unter der Platte verschraubt, 10 mm hinter dem Polster hochgeführt in ein gebogenes Rückenpolster (0,30 m breit, 90 mm hoch, 26 mm dick) auf 0,30 m Radius mit gerundeten Enden."),
        K.text("Thermal: legs, glides and back bar are the chromed-frame region, the footrest ring its own, pan and swivel the new seat-mount region; cushion and back pad are nested shells (foam 1.5 mm inside the leatherette cover); the hidden plywood board is a 12 mm solid of its own inside the cushion. Structure: legs, ring and pan as welded members with weld joints at the leg tops and the ring, the swivel as the bearing joint of the seat, the board as seat member, the foam cushion as pad member over its compression joint, the back bar bolted under the board, a contact pad under every glide and masses at the centres of mass (the frame's lies between the legs at 0.43 m, where it really is).", "Thermisch: Beine, Gleiter und Rückenbügel bilden die Region des Chromgestells, der Fußring seine eigene, Pfanne und Drehteller die neue Region der Sitzaufnahme; Sitz- und Rückenpolster sind verschachtelte Schalen (Schaum 1,5 mm innerhalb des Kunstlederbezugs); die verdeckte Sperrholzplatte ist ein eigener 12-mm-Körper im Polster. Tragwerk: Beine, Ring und Pfanne als geschweißte Stäbe mit Schweißknoten an den Beinköpfen und am Ring, der Drehteller als Lagergelenk des Sitzes, die Platte als Sitzstab, das Schaumpolster als Polsterstab über seinem Druckgelenk, der Rückenbügel unter der Platte verschraubt, ein Kontaktfeld unter jedem Gleiter und Massen in den Schwerpunkten (der des Gestells liegt zwischen den Beinen auf 0,43 m, wo er tatsächlich ist)."),
    ],
    100: [
        K.text("1:100 keeps every assembly at 15 mm: legs as true 25 mm tubes running into the floor (the 22 mm glides merge into the leg ends), a 12-section ring, the pan as one 17 mm disc with a chamfered edge, the swivel as one 15 mm plate, the cushion with a three-segment top round and no piping or crown, the back bar thickened to 15 mm and the curved back pad with five sections.", "1:100 behält jede Baugruppe bei 15 mm: Beine als echte 25-mm-Rohre bis zum Boden (die 22-mm-Gleiter gehen in den Beinenden auf), ein Ring mit 12 Schnitten, die Pfanne als eine 17-mm-Scheibe mit gefaster Kante, der Drehteller als eine 15-mm-Platte, das Polster mit dreiteiliger Rundung ohne Keder und Wölbung, der Rückenbügel auf 15 mm verstärkt und das gebogene Rückenpolster mit fünf Schnitten."),
        K.text("Thermal by reference to the body with the board as its own hidden solid and single shells (layers merged, B2); structure as at 1:50 with feature-sized weld joints and no pad joint.", "Thermisch als Verweise auf den Body mit der Platte als eigenem verdecktem Körper und einfachen Schalen (Schichten zusammengefasst, B2); Tragwerk wie bei 1:50 mit Schweißknoten in Merkmalsgröße und ohne Polstergelenk."),
    ],
    200: [
        K.text("1:200 generalises to 30 mm: legs and footrest ring stay true 25 mm tubes, drawn at that width (C13); pan and swivel merge into one 32 mm mount disc on the frame; the cushion is a 54 mm disc with a chamfered top edge; back bar and back pad are 30 mm thick, the pad bent on its 0.30 m radius with four sections.", "1:200 verallgemeinert auf 30 mm: Beine und Fußring bleiben echte 25-mm-Rohre, in dieser Breite gezeichnet (C13); Pfanne und Drehteller verschmelzen zu einer 32-mm-Aufnahmescheibe am Gestell; das Polster ist eine 54-mm-Scheibe mit gefaster Oberkante; Rückenbügel und Rückenpolster sind 30 mm dick, das Polster mit vier Schnitten auf seinem 0,30-m-Radius gebogen."),
        K.text("Thermal by reference with the board as its own hidden solid; structure: legs, ring, mount, a 30 mm board member, cushion and back bar, one contact pad per foot, four masses.", "Thermisch als Verweise mit der Platte als eigenem verdecktem Körper; Tragwerk: Beine, Ring, Aufnahme, ein 30-mm-Plattenstab, Polster und Rückenbügel, ein Kontaktfeld je Fuß, vier Massen."),
    ],
    500: [
        K.text("1:500 is the sign of the stool at 75 mm: four true 25 mm legs as slender tubes (drawn as 90 mm pen strokes, C24) and the ring as a true 25 mm tube drawn at that width (C13), the seat as one 86 mm block on the swivel axis (cushion, board, swivel and pan merged, narrowing to the pan below) and the backrest as one 75 mm curved panel round the rear of the seat, bar and pad merged.", "1:500 ist das Zeichen des Hockers bei 75 mm: vier echte 25-mm-Beine als schlanke Rohre (als 90-mm-Stiftstriche gezeichnet, C24) und der Ring als echtes 25-mm-Rohr in dieser Breite gezeichnet (C13), der Sitz als ein 86-mm-Block auf der Drehachse (Polster, Platte, Drehteller und Pfanne verschmolzen, unten auf die Pfanne eingezogen) und die Lehne als eine 75 mm dicke, gebogene Platte um die Rückseite des Sitzes, Bügel und Polster verschmolzen."),
        K.text("Thermal: seat-pad and backrest by reference, the mount and the hidden board as solids inside the seat block; structure: legs, ring, a board member, a cushion member, the back panel, one contact pad per foot and two masses (frame and seat with backrest).", "Thermisch: Sitzpolster und Lehne als Verweise, Aufnahme und verdeckte Platte als Körper im Sitzblock; Tragwerk: Beine, Ring, ein Plattenstab, ein Polsterstab, die Rückenplatte, ein Kontaktfeld je Fuß und zwei Massen (Gestell sowie Sitz mit Lehne)."),
    ],
}


def volumes():
    """📦 The `volumes` member: four authored scales."""
    return {str(d): {"status": "authored", "denominator": d, "body": body(d), "thermal": thermal(d), "structure": structure(d), "decisions": DECISIONS[d]} for d in (50, 100, 200, 500)}
