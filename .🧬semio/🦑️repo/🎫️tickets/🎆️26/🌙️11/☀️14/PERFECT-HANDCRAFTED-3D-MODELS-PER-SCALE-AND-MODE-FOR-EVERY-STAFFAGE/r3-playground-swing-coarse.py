"""🧩 Round 3 playground swing: shared hand-stated parts of the coarser scales (1:100, 1:200, 1:500), each sized for its scale by the caller: foot blocks, hanger blocks, joint and mass glyph solids, member tubes."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("swing_kit", os.path.join(T, "r3-playground-swing-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

ST, RB, AL = "steel-galvanised", "rubber", "aluminium"
FG, CS, SR = "frame-galv", "chain-steel", "seat-rubber"
UP, WX, WZ = (0, 1, 0), (1, 0, 0), (0, 0, 1)
A = math.radians(k.LEG_ANGLE)


def foot_block(sid, name, bone, xf, w, height):
    """🦶 Base plate and gussets merged into one block that tapers into the leg (the 2D foot of 1:200 and 1:500)."""
    hx = k.leg_half_x()
    xc = k.leg_centre_x(name, height)
    pts = [(xf - 0.09, 0.0), (xf + 0.09, 0.0), (xf + 0.09, height * 0.5), (xc + hx, height), (xc - hx, height), (xf - 0.09, height * 0.5)]
    return k.prism(sid, bone, ST, "body", 0.3, pts, k.local(bone, (0.0, 0.0, w)), k.ldir(bone, WX), k.ldir(bone, UP), -k.PLATE[2] / 2, k.PLATE[2] / 2)


def plate_box(sid, bone, xf, w, height):
    rot = k.quat_from_axes(k.ldir(bone, WX), k.ldir(bone, UP), [0, 0, 1])
    return k.box(sid, bone, ST, "body", 0.3, (k.PLATE[0], height, k.PLATE[2]), k.local(bone, (xf, height / 2, w)), rotation=rot, radius=0.002)


def glyph(sid, bone, material, radii, centre, member, part):
    s = k.ellipsoid(sid, bone, material, "detail", 0.2, radii, centre)
    s.update({"member": member, "part": part})
    return s


def tube_member(sid, bone, do, di, u0, u1, w):
    s = k.revolve(sid, bone, ST, "body", 0.45, [[u0, di / 2], [u1, di / 2], [u1, do / 2], [u0, do / 2]], WX, [0.0, 0.0, w])
    s.update({"member": bone, "part": "member"})
    return s


def beam_member(sid):
    s = k.revolve(sid, "beam", ST, "body", 0.45, [[0.0, k.R_BEAM - 0.0036], [2 * k.BEAM_TUBE_HALF, k.R_BEAM - 0.0036], [2 * k.BEAM_TUBE_HALF, k.R_BEAM], [0.0, k.R_BEAM]], WZ, [0.0, 0.0, -k.BEAM_TUBE_HALF])
    s.update({"member": "beam", "part": "member"})
    return s


def supports(half, depth, roller_r):
    out = []
    for name, bone, xf in k.LEGS:
        for side, sg in k.SIDES:
            w = sg * k.ZA
            anchor = ("pin-left-%s" if name == "left" else "roller-right-%s") % side
            if name == "left":
                s = k.prism("support-" + anchor, bone, ST, "body", 0.15, [[xf - half, -depth], [xf + half, -depth], [xf, 0.0]], k.local(bone, (0.0, 0.0, w)), k.ldir(bone, WX), k.ldir(bone, UP), -k.PLATE[2] / 2, k.PLATE[2] / 2)
            else:
                s = k.revolve("support-" + anchor, bone, ST, "body", 0.15, [[0.0, 0.0], [0.0, roller_r], [k.PLATE[2], roller_r], [k.PLATE[2], 0.0]], WZ, k.local(bone, (xf, -roller_r, w - k.PLATE[2] / 2)))
            s.update({"member": bone, "part": "support", "anchor": anchor})
            out.append(s)
    return out


def region(sid):
    if sid == "seat":
        return SR
    if sid.startswith("chain"):
        return CS
    return FG


def thermal(body):
    return [{"id": "th-" + s["id"], "of": s["id"], "region": region(s["id"])} for s in body]
