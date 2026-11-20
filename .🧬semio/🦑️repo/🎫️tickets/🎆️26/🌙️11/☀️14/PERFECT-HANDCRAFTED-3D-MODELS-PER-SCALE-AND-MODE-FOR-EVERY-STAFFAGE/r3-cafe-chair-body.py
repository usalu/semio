"""🪵 Round 3, furniture/cafe-chair: every body solid of every authored scale, stated by hand from the measures of `r3-cafe-chair-kit.py` (1:50 full construction, 1:100 without glides and caps, 1:100 and 1:200 with slender true-size tubes drawn as strokes, 1:200 with 30 mm boards, 1:500 the chair sign of 75 mm masses on slender tubes).
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_cafe_chair_kit", os.path.join(HERE, "r3-cafe-chair-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

UP = (0.0, 1.0, 0.0)
GLIDE = [(0.0, 0.0), (0.0, 0.0105), (0.0015, K.GLIDE_R), (0.008, K.GLIDE_R), (K.GLIDE_H, 0.011), (K.GLIDE_H, 0.0)]
CAP = [(0.0, 0.0), (0.0, K.R), (0.002, K.R), (0.005, 0.0095), (0.007, 0.006), (K.CAP_H, 0.0)]
TONE_SEAT, TONE_RAIL, TONE_BACK = 0.62, 0.45, 0.55


def floor_station(stations, direction):
    """⬇ Replaces the first station by one on the floor with a horizontal section (the tube end stands on the floor where no glide covers it)."""
    (p, t) = stations[0]
    k = -p[1] / direction[1]
    q = (p[0] + k * direction[0], 0.0, p[2])
    return [((q[0], 0.0005, q[2]), (0.0, 1.0, 0.0))] + stations[1:]


def rear_tubes(fine, glide, slender=False):
    """🦿 The two rear tubes: the leg part (bone leg-rear) up to the middle of the bend and the post part (bone back) from there to the top; slender at true size where the scale draws them as strokes."""
    leg, post = K.rear_stations(fine)
    extra = {"slender": True} if slender else {}
    out = []
    for p, side in K.SIDES:
        a = K.with_z(leg, side, K.rear_z)
        if not glide:
            a = floor_station(a, K.REAR_DIR)
        out.append(K.tube(f"rear-tube-{p}", "leg-rear", a, K.R, **extra))
    for p, side in K.SIDES:
        out.append(K.tube(f"post-{p}", "back", K.with_z(post, side, K.rear_z), K.R, **extra))
    return out


def rear_kinked():
    """🦿 The rear tubes of 1:500: a straight slender leg from the floor to the corner and a straight slender post from the corner to the top (the 80 mm bend is below the feature)."""
    out = []
    for p, side in K.SIDES:
        c = (K.CORNER[0], K.CORNER[1], side * K.POST_Z)
        foot = (K.REAR_FOOT[0], 0.0005, side * K.REAR_FOOT_Z)
        d = K.unit((c[0] - foot[0], c[1] - foot[1], c[2] - foot[2]))
        out.append(K.tube(f"rear-tube-{p}", "leg-rear", [(foot, (0.0, 1.0, 0.0)), (c, d)], K.R, slender=True))
    for p, side in K.SIDES:
        c = (K.CORNER[0], K.CORNER[1], side * K.POST_Z)
        top = (K.POST_TOP[0], K.POST_TOP[1], side * K.POST_Z)
        d = (K.POST_DIR[0], K.POST_DIR[1], 0.0)
        out.append(K.tube(f"post-{p}", "back", [(c, d), (top, d)], K.R, slender=True))
    return out


def front_tubes(glide, slender=False):
    """🦵 The two front tubes from the glide (or the floor) to 2 mm inside the seat board."""
    foot = K.add(K.FRONT_FOOT, K.FRONT_DIR, 0.005 / K.FRONT_DIR[1])
    stations = [(foot, K.FRONT_DIR), (K.FRONT_TOP, K.FRONT_DIR)]
    out = []
    for p, side in K.SIDES:
        a = K.with_z(stations, side, K.front_z)
        if not glide:
            a = floor_station(a, K.FRONT_DIR)
        extra = {"slender": True} if slender else {}
        out.append(K.tube(f"front-tube-{p}", "leg-front", a, K.R, **extra))
    return out


def glides():
    """🔘 Nylon glides under the four feet, standing on the floor about the vertical through the foot."""
    out = []
    for p, side in K.SIDES:
        out.append(K.revolve(f"glide-rear-{p}", "leg-rear", K.NYLON, GLIDE, UP, (K.REAR_FOOT[0], 0.0, side * K.REAR_FOOT_Z), role="detail"))
    for p, side in K.SIDES:
        out.append(K.revolve(f"glide-front-{p}", "leg-front", K.NYLON, GLIDE, UP, (K.FRONT_FOOT[0], 0.0, side * K.FRONT_FOOT_Z), role="detail"))
    return out


def caps():
    """🎩 Domed plastic end caps on the post tops."""
    d = (K.POST_DIR[0], K.POST_DIR[1], 0.0)
    return [K.revolve(f"cap-{p}", "back", K.NYLON, CAP, d, (K.POST_TOP[0], K.POST_TOP[1], side * K.POST_Z), role="detail") for p, side in K.SIDES]


def stretchers(slender=False):
    """➖ The two Ø16 side stretchers, welded between the leg axes at 0.15 m."""
    out = []
    for p, side in K.SIDES:
        a, b = K.STRETCH_REAR, K.STRETCH_FRONT
        extra = {"slender": True} if slender else {}
        out.append(K.capsule(f"stretcher-{p}", "stretcher", (a[0], a[1], side * a[2]), (b[0], b[1], side * b[2]), K.RS, **extra))
    return out


def rail_span():
    """↔ The u range of the side rails: from 4 mm in front of the post axis to 4 mm behind the front leg axis."""
    return K.U_POST + 0.004, K.U_FRONT - 0.004


def seat_rails():
    """🟫 The beech apron frame: two side rails between the tubes and a front and a back rail between the side rails, 18 x 45 mm under the board."""
    u0, u1 = rail_span()
    vm, h = (K.APRON_V0 + K.APRON_V1) / 2, K.APRON_V1 - K.APRON_V0
    out = [K.box(f"rail-side-{p}", "seat", K.BEECH, ((u0 + u1) / 2, vm, side * (K.APRON_W0 + K.APRON_W1) / 2), (u1 - u0, h, K.APRON_T), tone=TONE_RAIL) for p, side in K.SIDES]
    out.append(K.box("rail-front", "seat", K.BEECH, (u1 - K.APRON_T / 2, vm, 0.0), (K.APRON_T, h, 2 * K.APRON_W0), tone=TONE_RAIL))
    out.append(K.box("rail-back", "seat", K.BEECH, (u0 + K.APRON_T / 2, vm, 0.0), (K.APRON_T, h, 2 * K.APRON_W0), tone=TONE_RAIL))
    return out


def apron_ring(wall, v1):
    """🟫 The apron frame merged into one ring of the given wall thickness."""
    u0, u1 = rail_span()
    w = K.APRON_W1
    outer = [(-w, u0), (w, u0), (w, u1), (-w, u1)]
    hole = [(-w + wall, u0 + wall), (w - wall, u0 + wall), (w - wall, u1 - wall), (-w + wall, u1 - wall)]
    return K.plate("apron", "seat", K.BEECH, outer, K.APRON_V0, v1, tone=TONE_RAIL, holes=[hole])


def body50():
    """🔍 1:50: the full construction."""
    out = rear_tubes(True, True) + caps() + front_tubes(True) + glides() + stretchers()
    out.append(K.plate("seat-board", "seat", K.BEECH, K.seat_outline(4), -K.BOARD_T, 0.0, tone=TONE_SEAT))
    out += seat_rails()
    out.append(K.curved("back-board", K.BOARD_U0, K.BOARD_U1, K.BACK_T, K.BACK_W, 11, tone=TONE_BACK))
    out.append(K.curved("back-rail", K.RAIL_U0, K.RAIL_U1, K.BACK_T, K.BACK_W, 11, tone=TONE_BACK))
    return out


def body100():
    """🔍 1:100: glides and caps merged into the tubes, the apron frame one ring, boards with fewer sections."""
    out = rear_tubes(True, False, slender=True) + front_tubes(False, slender=True) + stretchers(slender=True)
    out.append(K.plate("seat-board", "seat", K.BEECH, K.seat_outline(2), -K.BOARD_T, 0.0, tone=TONE_SEAT))
    out.append(apron_ring(K.APRON_T, K.APRON_V1))
    out.append(K.curved("back-board", K.BOARD_U0, K.BOARD_U1, K.BACK_T, K.BACK_W, 5, exponent=4, tone=TONE_BACK))
    out.append(K.curved("back-rail", K.RAIL_U0, K.RAIL_U1, K.BACK_T, K.BACK_W, 5, exponent=4, tone=TONE_BACK))
    return out


def flat_back(sid, u0, u1, thick, v_front):
    """▭ A flat back board of the coarse scales: its front face at v_front, `thick` deep behind it."""
    return K.box(sid, "back", K.BEECH, ((u0 + u1) / 2, v_front + thick / 2, 0.0), (u1 - u0, thick, 2 * K.BACK_W), tone=TONE_BACK)


def body200():
    """🔍 1:200: slender true-size tubes (one continuous rear tube per side), a 30 mm seat board on a 30 mm apron ring, flat 30 mm back board and rail in front of the posts."""
    out = rear_tubes(False, False, slender=True) + front_tubes(False, slender=True) + stretchers(slender=True)
    out.append(K.plate("seat-board", "seat", K.BEECH, K.seat_outline(0, notch=False), -0.030, 0.0, tone=TONE_SEAT))
    out.append(apron_ring(0.030, -0.030))
    front = K.back_vc(K.POST_Z) - K.BACK_T / 2
    out.append(flat_back("back-board", K.BOARD_U0, K.BOARD_U1, 0.030, front))
    out.append(flat_back("back-rail", K.RAIL_U0, K.RAIL_U1, 0.030, front))
    return out


def body500():
    """🔍 1:500: a 75 mm seat slab and a 75 mm back slab (board and posts merged) on slender true-size tubes, the rear ones kinked at the corner."""
    out = rear_kinked() + front_tubes(False, slender=True) + stretchers(slender=True)
    out.append(K.plate("seat-slab", "seat", K.BEECH, K.seat_outline(0, notch=False), -0.075, 0.0, tone=TONE_SEAT))
    out.append(K.box("back-slab", "back", K.BEECH, ((K.BOARD_U0 + K.BOARD_U1) / 2, (K.SLAB500_V0 + K.SLAB500_V1) / 2, 0.0), (K.BOARD_U1 - K.BOARD_U0, K.SLAB500_V1 - K.SLAB500_V0, 2 * K.BACK_W), tone=TONE_BACK))
    return out


if __name__ == "__main__":
    for s in ("50", "100", "200", "500"):
        print(s, [b["id"] for b in globals()["body" + s]()])
