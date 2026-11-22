"""🖋️ The individually drawn 2D representations of the used close-coupled WC (side elevation, front elevation, plan) at 1:50, 1:100 and 1:200, drawn to the same numbers as the solids, and the unavailable 1:500.

@see ./author_geometry.py — the hand-chosen numbers
@see ./author_solids.py — the solids the drawings agree with
"""
from author_geometry import (BOWL_LOOP, BRACKET, BUFFER, BUFFERS, BUTTON, CAP, CARD, CIS, CLID, DECK, FOOT, HINGE, LID, LID_LUG_Z, LS, LUG, PAL_BOARD_W, PAL_ROW_X, PAL_ROW_Z, PAL_TOP_Z, PAL_X, PAL_Z, PALLET_H, PH, REAR, SEAT, SEAT_LUG_Z, SPIGOT, XB, Y_BLK1, Y_BOT1, Y_CROSS1, T, clipped_ellipse, ellipse_pts, envelope_outline, loc, r6, rounded_rect)
from author_solids import PAL200_BOT, stringer_comb, bowl_profile100, bowl_profile200, neck_profile, seat_hole, seat_outline, seat_outline200


def sh(id, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, hatch=None, pen=None):
    """✏️ One 2D shape."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    return s


def erect(bone, x0, y0, x1, y1):
    """⬛️ An elevation rectangle from world rest corners, in the frame of its bone."""
    u0, v0 = loc(bone, x0, y0)
    return {"kind": "rect", "x": u0, "y": v0, "w": r6(x1 - x0), "h": r6(y1 - y0)}


def epoly(bone, pts, closed=True):
    """🔷️ An elevation polygon from world rest points, in the frame of its bone."""
    return {"kind": "poly", "points": [list(loc(bone, x, y)) for x, y in pts], "closed": closed}


def epath(bone, rings):
    """〰️ An elevation path of several closed rings from world rest points."""
    return {"kind": "path", "d": " ".join("M " + " L ".join("%s %s" % loc(bone, x, y) for x, y in ring) + " Z" for ring in rings)}


def estroke(bone, lines):
    """➖️ Open elevation strokes from world rest points."""
    return {"kind": "path", "d": " ".join("M " + " L ".join("%s %s" % loc(bone, x, y) for x, y in line) for line in lines)}


def wrect(a0, b0, a1, b1):
    """⬜️ A front or plan rectangle in world rest coordinates (front: z, y; plan: x, z)."""
    return {"kind": "rect", "x": r6(a0), "y": r6(b0), "w": r6(a1 - a0), "h": r6(b1 - b0)}


def wpoly(pts, closed=True):
    """🔶️ A front or plan polygon in world rest coordinates."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}


def wpath(rings):
    """〰️ A front or plan path of several closed rings."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in ring) + " Z" for ring in rings)}


def wstroke(lines):
    """➖️ Open front or plan strokes."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in line) for line in lines)}


def box_ring(a0, a1, b0, b1):
    """▭️ The four corners of a rectangle as a ring."""
    return [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]


def lifted(pts, dy=PH):
    """⬆️ Profile points (a, y above the base) lifted onto the pallet."""
    return [(a, y + dy) for a, y in pts]


W = PAL_BOARD_W / 2
PAL = dict(region="pallet", material="spruce-pallet")
PAN = dict(region="pan", material="vitreous-china-pan")
CISR = dict(region="cistern", material="vitreous-china-cistern")
SEATR = dict(region="seat-set", material="duroplast-seat")
CAPR = dict(region="outlet-cap", material="pe-cap")
CARDR = dict(region="cardboard", material="corrugated-board")


def foot_side(view):
    """🦶️ The tapered foot in the side (x) or front (z) view."""
    if view == "elevation":
        c, r = FOOT["cx"], FOOT["rx"]
    else:
        c, r = 0.0, FOOT["rz"]
    rt = r * FOOT["top"]
    return [(c - r, PH), (c + r, PH), (c + rt, PH + FOOT["y1"]), (c - rt, PH + FOOT["y1"])]


def pallet_views(scale):
    """🪵️ The half pallet and the cardboard in the three views at 1:50 and 1:100 (boards, blocks or stringers), or 1:200 (deck and runners)."""
    p = "pallet"
    hatch = "grain" if scale == 50 else None
    sfx = {50: "", 100: "-100", 200: "-200"}[scale]
    if scale == 200:
        runner = [(-PAL_X, 0.0), (PAL_X, 0.0), (PAL_X, Y_BLK1), (PAL_ROW_X[2] - W, Y_BLK1), (PAL_ROW_X[2] - W, PAL200_BOT), (PAL_ROW_X[1] + W, PAL200_BOT), (PAL_ROW_X[1] + W, Y_BLK1), (PAL_ROW_X[1] - W, Y_BLK1), (PAL_ROW_X[1] - W, PAL200_BOT), (PAL_ROW_X[0] + W, PAL200_BOT), (PAL_ROW_X[0] + W, Y_BLK1), (-PAL_X, Y_BLK1)]
        el = [sh("pallet-runners-200", p, "body", 1, True, 0.6, epoly(p, runner), **PAL), sh("pallet-deck-200", p, "body", 1.2, True, 0.7, erect(p, -PAL_X, Y_BLK1, PAL_X, PALLET_H), **PAL), sh("cardboard-200", p, "body", 1.4, True, 0.5, erect(p, CARD["x0"], PALLET_H, CARD["x1"], PH), **CARDR)]
        fr = [sh("pallet-runners-200", p, "body", 1, True, 0.6, wpath([box_ring(z - W, z + W, 0.0, Y_BLK1) for z in PAL_ROW_Z]), **PAL), sh("pallet-deck-200", p, "body", 1.2, True, 0.7, wrect(-PAL_Z, Y_BLK1, PAL_Z, PALLET_H), **PAL), sh("cardboard-200", p, "body", 1.4, True, 0.5, wrect(-CARD["z"], PALLET_H, CARD["z"], PH), **CARDR)]
        pl = [sh("pallet-deck-200", p, "body", 1, True, 0.7, wrect(-PAL_X, -PAL_Z, PAL_X, PAL_Z), **PAL), sh("cardboard-200", p, "body", 1.4, True, 0.5, wrect(CARD["x0"], -CARD["z"], CARD["x1"], CARD["z"]), **CARDR)]
        return el, fr, pl
    el = [sh("pallet-bottom-boards" + sfx, p, "body", 1, True, 0.6, erect(p, -PAL_X, 0.0, PAL_X, Y_BOT1), **PAL)]
    if scale == 50:
        for name, x in zip(("rear", "middle", "front"), PAL_ROW_X):
            el.append(sh(f"pallet-blocks-{name}", p, "body", 1.1, True, 0.58, erect(p, x - W, Y_BOT1, x + W, Y_BLK1), **PAL))
        el.append(sh("pallet-cross-boards", p, "body", 1.2, True, 0.64, epath(p, [box_ring(x - W, x + W, Y_BLK1, Y_CROSS1) for x in PAL_ROW_X]), **PAL))
    else:
        el.append(sh("pallet-stringers-100", p, "body", 1.1, True, 0.62, epath(p, [box_ring(x - W, x + W, Y_BOT1, Y_CROSS1) for x in PAL_ROW_X]), **PAL))
    el.append(sh("pallet-top-boards" + sfx, p, "body", 1.3, True, 0.72, erect(p, -PAL_X, Y_CROSS1, PAL_X, PALLET_H), hatch=hatch, **PAL))
    el.append(sh("cardboard" + sfx, p, "body", 1.4, True, 0.5, erect(p, CARD["x0"], PALLET_H, CARD["x1"], PH), **CARDR))
    fr = [sh("pallet-bottom-boards" + sfx, p, "body", 1, True, 0.6, wpath([box_ring(z - W, z + W, 0.0, Y_BOT1) for z in PAL_ROW_Z]), **PAL)]
    if scale == 50:
        fr.append(sh("pallet-blocks-front", p, "body", 1.1, True, 0.58, wpath([box_ring(z - W, z + W, Y_BOT1, Y_BLK1) for z in PAL_ROW_Z]), **PAL))
        fr.append(sh("pallet-cross-boards", p, "body", 1.2, True, 0.64, wrect(-PAL_Z, Y_BLK1, PAL_Z, Y_CROSS1), **PAL))
    else:
        fr.append(sh("pallet-stringers-100", p, "body", 1.1, True, 0.62, wpoly(stringer_comb()), **PAL))
    fr.append(sh("pallet-top-boards" + sfx, p, "body", 1.3, True, 0.72, wpath([box_ring(z - W, z + W, Y_CROSS1, PALLET_H) for z in PAL_TOP_Z]), **PAL))
    fr.append(sh("cardboard" + sfx, p, "body", 1.4, True, 0.5, wrect(-CARD["z"], PALLET_H, CARD["z"], PH), **CARDR))
    cross = "pallet-cross-boards" if scale == 50 else "pallet-stringers-100"
    pl = [sh(cross, p, "body", 1, True, 0.5, wpath([box_ring(x - W, x + W, -PAL_Z, PAL_Z) for x in PAL_ROW_X]), **PAL),
          sh("pallet-top-boards" + sfx, p, "body", 1.2, True, 0.72, wpath([box_ring(-PAL_X, PAL_X, z - W, z + W) for z in PAL_TOP_Z]), hatch=hatch, **PAL),
          sh("cardboard" + sfx, p, "body", 1.4, True, 0.5, wrect(CARD["x0"], -CARD["z"], CARD["x1"], CARD["z"]), **CARDR)]
    return el, fr, pl


def seat_plan_ring(outline):
    """🪑️ The seat outline as a plan ring of (x, z) points."""
    return [(x, z) for x, z in outline]


def rep50():
    """🖋️ The 1:50 drawing: the whole WC with rim, foot, rear body, deck, spigot and cap, the seat with its buffers and lugs, the lid, the cistern with its taped lid and replaced button, the half pallet board by board, and the traces of use."""
    el, fr, pl = pallet_views(50)
    p, c = "pan", "cistern"
    bowl_side = lifted(envelope_outline(BOWL_LOOP, XB))
    bowl_front = lifted(envelope_outline(BOWL_LOOP, 0.0, LS))
    el += [
        sh("foot", p, "body", 2, True, 0.86, epoly(p, foot_side("elevation")), **PAN),
        sh("spigot", p, "body", 2.1, True, 0.84, erect(p, SPIGOT["x0"], PH + SPIGOT["y"] - SPIGOT["ro"], SPIGOT["x1"], PH + SPIGOT["y"] + SPIGOT["ro"]), **PAN),
        sh("outlet-cap", p, "body", 2.2, True, 0.3, erect(p, CAP["x0"], PH + SPIGOT["y"] - CAP["ro"], CAP["x1"], PH + SPIGOT["y"] + CAP["ro"]), **CAPR),
        sh("rear-body", p, "body", 2.3, True, 0.88, erect(p, REAR["x0"], PH + REAR["y0"], REAR["x1"], PH + REAR["y1"]), **PAN),
        sh("neck", p, "body", 2.4, True, 0.86, epoly(p, neck_profile()), **PAN),
        sh("bowl", p, "body", 3, True, 0.92, epoly(p, bowl_side), **PAN),
        sh("deck", p, "body", 3.1, True, 0.9, erect(p, DECK["x0"], PH + DECK["y0"], DECK["x1"], PH + DECK["y1"]), **PAN),
        sh("rim-edge", p, "detail", 3.2, False, 0.4, estroke(p, [[(XB + 0.245, PH + 0.365), (XB + 0.16, PH + 0.365)]]), region="pan"),
        sh("foot-joint", p, "detail", 3.3, False, 0.4, estroke(p, [[(FOOT["cx"] + FOOT["rx"] * FOOT["top"], PH + FOOT["y1"]), (FOOT["cx"] - FOOT["rx"] * FOOT["top"], PH + FOOT["y1"])]]), region="pan"),
        sh("silicone-residue", p, "texture", 3.4, False, 0.3, estroke(p, [[(FOOT["cx"] - FOOT["rx"] + 0.01, PH + 0.004), (FOOT["cx"] + FOOT["rx"] - 0.01, PH + 0.004)]]), region="pan"),
        sh("glaze-chip", p, "texture", 3.5, False, 0.3, estroke(p, [[(0.196, PH + 0.03), (0.188, PH + 0.042), (0.2, PH + 0.05)]]), region="pan"),
        sh("hinge-brackets", p, "body", 3.6, True, 0.55, erect(p, BRACKET["x0"], PH + BRACKET["y0"], BRACKET["x1"], PH + BRACKET["y1"]), region="seat-set", material="chromed-zinc"),
        sh("cistern-tank", c, "body", 4, True, 0.9, erect(c, CIS["x0"], PH + CIS["y0"], CIS["x1"], PH + CIS["y1"]), **CISR),
        sh("cistern-sticker", c, "texture", 4.1, False, 0.3, erect(c, -0.31, PH + 0.52, -0.27, PH + 0.56), region="cistern"),
        sh("cistern-tape", c, "texture", 4.2, False, 0.3, erect(c, -0.279, PH + 0.70, -0.231, PH + CIS["y1"]), region="cistern"),
        sh("cistern-lid", "cistern-lid", "body", 4.3, True, 0.94, erect("cistern-lid", CLID["x0"], PH + CLID["y0"], CLID["x1"], PH + CLID["y1"]), **CISR),
        sh("button", "button", "body", 4.4, True, 0.98, erect("button", BUTTON["x"] - BUTTON["r"], PH + CLID["y1"], BUTTON["x"] + BUTTON["r"], PH + BUTTON["y1"]), region="cistern", material="pp-fittings", pen=0.25),
        sh("seat-buffers-front", "seat", "body", 5, True, 0.25, erect("seat", BUFFERS[0][0] - BUFFER["dx"] / 2, PH + BUFFER["y0"], BUFFERS[0][0] + BUFFER["dx"] / 2, PH + BUFFER["y1"]), region="seat-set", material="rubber-buffer", pen=0.25),
        sh("seat-buffers-rear", "seat", "body", 5, True, 0.25, erect("seat", BUFFERS[2][0] - BUFFER["dx"] / 2, PH + BUFFER["y0"], BUFFERS[2][0] + BUFFER["dx"] / 2, PH + BUFFER["y1"]), region="seat-set", material="rubber-buffer", pen=0.25),
        sh("seat-skirt", "seat", "body", 5.1, True, 0.78, erect("seat", SEAT["xcut"], PH + SEAT["y0"], SEAT["cx"] + SEAT["rx"], PH + SEAT["yb"]), **SEATR),
        sh("seat-ring", "seat", "body", 5.2, True, 0.82, erect("seat", SEAT["xcut"], PH + SEAT["yb"], SEAT["cx"] + SEAT["rx"], PH + SEAT["y1"]), **SEATR),
        sh("seat-lugs", "seat", "body", 5.3, True, 0.8, erect("seat", LUG["x0"], PH + LUG["y0"], LUG["x1"], PH + LUG["y1"]), **SEATR),
        sh("lid-skirt", "lid", "body", 6, True, 0.8, erect("lid", SEAT["xcut"], PH + LID["y0"], SEAT["cx"] + SEAT["rx"], PH + LID["ys"]), **SEATR),
        sh("lid", "lid", "body", 6.1, True, 0.86, erect("lid", SEAT["xcut"], PH + LID["ys"], SEAT["cx"] + SEAT["rx"], PH + LID["y1"]), **SEATR),
        sh("lid-lugs", "lid", "body", 6.2, True, 0.8, erect("lid", LUG["x0"], PH + LUG["y0"], LUG["x1"], PH + LUG["y1"]), **SEATR),
    ]
    fr += [
        sh("rear-body", p, "body", 2, True, 0.88, wrect(-REAR["z"], PH + REAR["y0"], REAR["z"], PH + REAR["y1"]), **PAN),
        sh("foot", p, "body", 2.1, True, 0.86, wpoly(foot_side("front")), **PAN),
        sh("deck", p, "body", 2.2, True, 0.9, wrect(-DECK["z"], PH + DECK["y0"], DECK["z"], PH + DECK["y1"]), **PAN),
        sh("cistern-tank", c, "body", 2.5, True, 0.9, wrect(-CIS["z"], PH + CIS["y0"], CIS["z"], PH + CIS["y1"]), **CISR),
        sh("cistern-tape", c, "texture", 2.6, False, 0.3, wpath([box_ring(z - 0.024, z + 0.024, PH + 0.70, PH + CIS["y1"]) for z in (-0.09, 0.09)]), region="cistern"),
        sh("cistern-lid", "cistern-lid", "body", 2.7, True, 0.94, wrect(-CLID["z"], PH + CLID["y0"], CLID["z"], PH + CLID["y1"]), **CISR),
        sh("button", "button", "body", 2.8, True, 0.98, wrect(-BUTTON["r"], PH + CLID["y1"], BUTTON["r"], PH + BUTTON["y1"]), region="cistern", material="pp-fittings", pen=0.25),
        sh("bowl", p, "body", 3, True, 0.92, wpoly(bowl_front), **PAN),
        sh("silicone-residue", p, "texture", 3.4, False, 0.3, wstroke([[(-FOOT["rz"] + 0.01, PH + 0.004), (FOOT["rz"] - 0.01, PH + 0.004)]]), region="pan"),
        sh("glaze-chip", p, "texture", 3.5, False, 0.3, wstroke([[(0.02, PH + 0.03), (0.032, PH + 0.042), (0.024, PH + 0.052)]]), region="pan"),
        sh("seat-buffers-front", "seat", "body", 4, True, 0.25, wpath([box_ring(z - BUFFER["dz"] / 2, z + BUFFER["dz"] / 2, PH + BUFFER["y0"], PH + BUFFER["y1"]) for z in (-0.10, 0.10)]), region="seat-set", material="rubber-buffer", pen=0.25),
        sh("seat-skirt", "seat", "body", 4.1, True, 0.78, wrect(-SEAT["rz"], PH + SEAT["y0"], SEAT["rz"], PH + SEAT["yb"]), **SEATR),
        sh("seat-ring", "seat", "body", 4.2, True, 0.82, wrect(-SEAT["rz"], PH + SEAT["yb"], SEAT["rz"], PH + SEAT["y1"]), **SEATR),
        sh("lid-skirt", "lid", "body", 5, True, 0.8, wrect(-SEAT["rz"], PH + LID["y0"], SEAT["rz"], PH + LID["ys"]), **SEATR),
        sh("lid", "lid", "body", 5.1, True, 0.86, wrect(-SEAT["rz"], PH + LID["ys"], SEAT["rz"], PH + LID["y1"]), **SEATR),
    ]
    seat_out = seat_plan_ring(seat_outline())
    pl += [
        sh("deck", p, "body", 2, True, 0.9, wpoly(rounded_rect(DECK["x0"], DECK["x1"], -DECK["z"], DECK["z"], DECK["r"])), **PAN),
        sh("bowl", p, "body", 2.1, True, 0.92, wpoly(ellipse_pts(XB, 0.0, 0.245, 0.245 * LS)), **PAN),
        sh("bowl-inside", p, "detail", 2.2, True, 0.6, wpoly(ellipse_pts(XB, 0.0, 0.226, 0.226 * LS)), **PAN),
        sh("limescale-ring", p, "texture", 2.3, False, 0.3, {"kind": "ellipse", "cx": r6(XB - 0.005), "cy": 0.0, "rx": 0.075, "ry": 0.058}, region="pan"),
        sh("hinge-brackets", p, "body", 2.4, True, 0.55, wpath([box_ring(BRACKET["x0"], BRACKET["x1"], z - BRACKET["w"] / 2, z + BRACKET["w"] / 2) for z in (-BRACKET["z"], BRACKET["z"])]), region="seat-set", material="chromed-zinc"),
        sh("seat-lugs", "seat", "body", 3, True, 0.8, wpath([box_ring(LUG["x0"], LUG["x1"], z - LUG["w"] / 2, z + LUG["w"] / 2) for z in (-SEAT_LUG_Z, SEAT_LUG_Z)]), **SEATR),
        sh("seat-ring", "seat", "body", 3.1, True, 0.82, wpoly(seat_out), **SEATR),
        sh("seat-opening", "seat", "detail", 3.2, False, 0.4, wpoly(seat_hole()), region="seat-set"),
        sh("lid-lugs", "lid", "body", 4, True, 0.8, wpath([box_ring(LUG["x0"], LUG["x1"], z - LUG["w"] / 2, z + LUG["w"] / 2) for z in (-LID_LUG_Z, LID_LUG_Z)]), **SEATR),
        sh("lid", "lid", "body", 4.1, True, 0.86, wpoly(seat_out), **SEATR),
        sh("lid-scratches", "lid", "texture", 4.2, False, 0.3, wstroke([[(0.12, 0.04), (0.2, 0.07)], [(0.15, -0.03), (0.24, -0.01)], [(0.02, 0.1), (0.07, 0.115)]]), region="seat-set"),
        sh("cistern-lid", "cistern-lid", "body", 5, True, 0.94, wpoly(rounded_rect(CLID["x0"], CLID["x1"], -CLID["z"], CLID["z"], CLID["r"])), **CISR),
        sh("cistern-tape", "cistern-lid", "texture", 5.1, False, 0.3, wpath([box_ring(CLID["x0"], CLID["x1"], z - 0.024, z + 0.024) for z in (-0.09, 0.09)]), region="cistern"),
        sh("button", "button", "body", 5.2, True, 0.98, {"kind": "ellipse", "cx": r6(BUTTON["x"]), "cy": 0.0, "rx": BUTTON["r"], "ry": BUTTON["r"]}, region="cistern", material="pp-fittings"),
        sh("button-split", "button", "detail", 5.3, False, 0.4, wstroke([[(BUTTON["x"] - 0.02, 0.0), (BUTTON["x"] + 0.02, 0.0)]]), region="cistern"),
    ]
    return {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7},
            "hatches": [{"id": "grain", "title": T("Spruce grain along the pallet boards", "Fichtenmaserung längs der Palettenbretter"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13}],
            "geometry": {"elevation": el, "front": fr, "plan": pl},
            "decisions": {
                "silhouette": T("WC 13.4 x 15.9 mm on a 16 x 2.9 mm half pallet: rim and bowl as their true turned silhouette, the tapered foot, the rear body under the cistern deck with the neck that fills behind the bowl, the horizontal outlet spigot at 0.18 m under the red cap, the seat ring on its 8 mm buffers, the lid, the hinge, the cistern with its lid and the button proud of it. Front view: the bowl narrowing to the foot, seat and lid, the cistern wider behind. Plan: lid, cistern lid with the two tape strips and the button, the deck and hinge behind the lid, the boards of the pallet.", "WC 13,4 x 15,9 mm auf einer 16 x 2,9 mm großen Halbpalette: Rand und Becken in ihrer wahren gedrehten Silhouette, der verjüngte Fuß, der Hinterkörper unter der Spülkastenauflage mit dem Hals hinter dem Becken, der waagerechte Abgangsstutzen auf 0,18 m unter der roten Kappe, der Sitzring auf seinen 8-mm-Puffern, der Deckel, das Scharnier, der Spülkasten mit Deckel und darüberstehender Drücker. Vorderansicht: das zum Fuß schmaler werdende Becken, Sitz und Deckel, dahinter der breitere Spülkasten. Grundriss: Deckel, Spülkastendeckel mit zwei Klebebandstreifen und Drücker, Auflage und Scharnier hinter dem Deckel, die Bretter der Palette."),
                "retained": T("The traces of use: the limescale ring in the bowl, the old silicone bead along the foot, the chipped glaze at the front of the foot, the scratched and yellowed lid, the depot sticker on the cistern, the packing tape that holds the cistern lid for transport and the white replacement button.", "Die Gebrauchsspuren: der Kalkrand im Becken, die alte Silikonfuge am Fuß, die abgeplatzte Glasur vorn am Fuß, der verkratzte und vergilbte Deckel, der Lageraufkleber am Spülkasten, das Packband, das den Spülkastendeckel für den Transport hält, und der weiße Ersatzdrücker."),
                "merged": T("Left and right lugs, brackets and buffers coincide in the side view; the bottom plate is drawn with the tank, the rear plate with the rear body.", "Linke und rechte Laschen, Halter und Puffer fallen in der Seitenansicht zusammen; der Boden wird mit dem Tank, die Hinterplatte mit dem Hinterkörper gezeichnet."),
                "omitted": T("The trapway, the foot plate, the flush and fill valves and the cistern bolts lie inside the ceramic and are not drawn; the 6 mm hinge pin lies inside the lugs.", "Der Siphon, die Fußplatte, Spül- und Füllventil und die Spülkastenschrauben liegen im Keramikkörper und werden nicht gezeichnet; der 6-mm-Scharnierstift liegt in den Laschen."),
                "thinElements": T("The 5 mm cardboard sheet (0.1 mm), the 8 mm buffers and the 5 mm button rim are inked by the 0.25 mm pen where they stand alone.", "Die 5-mm-Pappe (0,1 mm), die 8-mm-Puffer und der 5-mm-Drückerrand trägt der 0,25-mm-Stift, wo sie allein stehen."),
                "hatching": T("Spruce grain 0.6 mm on the pallet boards with the 0.13 mm pen; the glazed ceramic stays plain.", "Fichtenmaserung 0,6 mm auf den Palettenbrettern mit dem 0,13-mm-Stift; die glasierte Keramik bleibt glatt."),
                "styles": T("Button 0.98, cistern lid 0.94, bowl 0.92, deck and cistern 0.9, rear body 0.88, foot 0.86, lid 0.86, seat 0.82, pallet boards 0.58-0.72, hinge 0.55, cardboard 0.5, red cap 0.3, buffers 0.25.", "Drücker 0,98, Spülkastendeckel 0,94, Becken 0,92, Auflage und Spülkasten 0,9, Hinterkörper 0,88, Fuß 0,86, Deckel 0,86, Sitz 0,82, Palettenbretter 0,58-0,72, Scharnier 0,55, Pappe 0,5, rote Kappe 0,3, Puffer 0,25."),
                "overlays": T("One shape per part and region; the seat ring, the bowl and its limescale ring lie under the lid in plan and show when the lid is raised.", "Eine Form je Teil und Region; Sitzring, Becken und Kalkrand liegen im Grundriss unter dem Deckel und erscheinen bei geöffnetem Deckel."),
            }}


def rep100():
    """🖋️ The 1:100 drawing: rim and bowl, foot, rear body, deck, outlet with cap, seat on the rim, lid, cistern, button, pallet with stringers."""
    el, fr, pl = pallet_views(100)
    p, c = "pan", "cistern"
    prof = bowl_profile100()
    el += [
        sh("foot-100", p, "body", 2, True, 0.86, epoly(p, foot_side("elevation")), **PAN),
        sh("spigot-100", p, "body", 2.05, True, 0.84, erect(p, CAP["x1"], PH + SPIGOT["y"] - SPIGOT["ro"], SPIGOT["x1"], PH + SPIGOT["y"] + SPIGOT["ro"]), **PAN),
        sh("outlet-cap-100", p, "body", 2.1, True, 0.3, erect(p, CAP["x0"], PH + SPIGOT["y"] - CAP["ro"], CAP["x1"], PH + SPIGOT["y"] + CAP["ro"]), **CAPR),
        sh("rear-body-100", p, "body", 2.3, True, 0.88, erect(p, REAR["x0"], PH + REAR["y0"], REAR["x1"], PH + REAR["y1"]), **PAN),
        sh("neck-100", p, "body", 2.4, True, 0.86, epoly(p, neck_profile()), **PAN),
        sh("bowl-100", p, "body", 3, True, 0.92, epoly(p, lifted(envelope_outline(prof, XB))), **PAN),
        sh("deck-100", p, "body", 3.1, True, 0.9, erect(p, DECK["x0"], PH + DECK["y0"], DECK["x1"], PH + DECK["y1"]), **PAN),
        sh("hinge-brackets-100", p, "body", 3.6, True, 0.55, erect(p, BRACKET["x0"], PH + BRACKET["y0"], BRACKET["x1"], PH + BRACKET["y1"]), region="seat-set", material="chromed-zinc"),
        sh("cistern-100", c, "body", 4, True, 0.9, erect(c, CIS["x0"], PH + CIS["y0"], CIS["x1"], PH + CIS["y1"]), **CISR),
        sh("cistern-lid-100", "cistern-lid", "body", 4.3, True, 0.94, erect("cistern-lid", CLID["x0"], PH + CLID["y0"], CLID["x1"], PH + CLID["y1"]), **CISR),
        sh("button-100", "button", "body", 4.4, True, 0.98, erect("button", BUTTON["x"] - BUTTON["r"], PH + CLID["y1"], BUTTON["x"] + BUTTON["r"], PH + BUTTON["y1"]), region="cistern", material="pp-fittings", pen=0.18),
        sh("seat-ring-100", "seat", "body", 5, True, 0.82, erect("seat", SEAT["xcut"], PH + BUFFER["y0"], SEAT["cx"] + SEAT["rx"], PH + SEAT["y1"]), **SEATR),
        sh("seat-lugs-100", "seat", "body", 5.3, True, 0.8, erect("seat", LUG["x0"], PH + LUG["y0"], LUG["x1"], PH + LUG["y1"]), **SEATR),
        sh("lid-100", "lid", "body", 6, True, 0.86, erect("lid", SEAT["xcut"], PH + LID["y0"], SEAT["cx"] + SEAT["rx"], PH + LID["y1"]), **SEATR),
        sh("lid-lugs-100", "lid", "body", 6.2, True, 0.8, erect("lid", LUG["x0"], PH + LUG["y0"], LUG["x1"], PH + LUG["y1"]), **SEATR),
    ]
    fr += [
        sh("rear-body-100", p, "body", 2, True, 0.88, wrect(-REAR["z"], PH + REAR["y0"], REAR["z"], PH + REAR["y1"]), **PAN),
        sh("foot-100", p, "body", 2.1, True, 0.86, wpoly(foot_side("front")), **PAN),
        sh("cistern-100", c, "body", 2.5, True, 0.9, wrect(-CIS["z"], PH + CIS["y0"], CIS["z"], PH + CIS["y1"]), **CISR),
        sh("cistern-lid-100", "cistern-lid", "body", 2.7, True, 0.94, wrect(-CLID["z"], PH + CLID["y0"], CLID["z"], PH + CLID["y1"]), **CISR),
        sh("button-100", "button", "body", 2.8, True, 0.98, wrect(-BUTTON["r"], PH + CLID["y1"], BUTTON["r"], PH + BUTTON["y1"]), region="cistern", material="pp-fittings", pen=0.18),
        sh("bowl-100", p, "body", 3, True, 0.92, wpoly(lifted(envelope_outline(prof, 0.0, LS))), **PAN),
        sh("seat-ring-100", "seat", "body", 4.1, True, 0.82, wrect(-SEAT["rz"], PH + BUFFER["y0"], SEAT["rz"], PH + SEAT["y1"]), **SEATR),
        sh("lid-100", "lid", "body", 5, True, 0.86, wrect(-SEAT["rz"], PH + LID["y0"], SEAT["rz"], PH + LID["y1"]), **SEATR),
    ]
    seat_out = seat_plan_ring(seat_outline())
    pl += [
        sh("deck-100", p, "body", 2, True, 0.9, wpoly(rounded_rect(DECK["x0"], DECK["x1"], -DECK["z"], DECK["z"], DECK["r"])), **PAN),
        sh("bowl-100", p, "body", 2.1, True, 0.92, wpoly(ellipse_pts(XB, 0.0, 0.245, 0.245 * LS)), **PAN),
        sh("bowl-inside-100", p, "detail", 2.2, True, 0.6, wpoly(ellipse_pts(XB, 0.0, 0.224, 0.224 * LS)), **PAN),
        sh("hinge-brackets-100", p, "body", 2.4, True, 0.55, wpath([box_ring(BRACKET["x0"], BRACKET["x1"], z - 0.0075, z + 0.0075) for z in (-0.1025, 0.1025)]), region="seat-set", material="chromed-zinc"),
        sh("seat-lugs-100", "seat", "body", 3, True, 0.8, wpath([box_ring(LUG["x0"], LUG["x1"], z - LUG["w"] / 2, z + LUG["w"] / 2) for z in (-SEAT_LUG_Z, SEAT_LUG_Z)]), **SEATR),
        sh("seat-ring-100", "seat", "body", 3.1, True, 0.82, wpoly(seat_out), **SEATR),
        sh("lid-lugs-100", "lid", "body", 4, True, 0.8, wpath([box_ring(LUG["x0"], LUG["x1"], z - LUG["w"] / 2, z + LUG["w"] / 2) for z in (-LID_LUG_Z, LID_LUG_Z)]), **SEATR),
        sh("lid-100", "lid", "body", 4.1, True, 0.86, wpoly(seat_out), **SEATR),
        sh("cistern-lid-100", "cistern-lid", "body", 5, True, 0.94, wpoly(rounded_rect(CLID["x0"], CLID["x1"], -CLID["z"], CLID["z"], CLID["r"])), **CISR),
        sh("button-100", "button", "body", 5.2, True, 0.98, {"kind": "ellipse", "cx": r6(BUTTON["x"]), "cy": 0.0, "rx": BUTTON["r"], "ry": BUTTON["r"]}, region="cistern", material="pp-fittings"),
    ]
    return {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5},
            "hatches": [], "geometry": {"elevation": el, "front": fr, "plan": pl},
            "decisions": {
                "silhouette": T("WC 6.7 x 8.0 mm on an 8 mm pallet: rim and bowl, foot, rear body with neck and deck, the outlet with its cap as one turned part, the seat down on the rim, the lid, the hinge, the cistern with lid and button; the pallet with its top boards, three stringers and bottom boards.", "WC 6,7 x 8,0 mm auf einer 8-mm-Palette: Rand und Becken, Fuß, Hinterkörper mit Hals und Auflage, der Abgang mit Kappe als ein gedrehtes Teil, der Sitz auf dem Rand, der Deckel, das Scharnier, der Spülkasten mit Deckel und Drücker; die Palette mit Deckbrettern, drei Querträgern und Bodenbrettern."),
                "retained": T("The red outlet cap, the dark bowl opening under a raised lid, the button and the gaps between the pallet boards.", "Die rote Abgangskappe, die dunkle Beckenöffnung unter geöffnetem Deckel, der Drücker und die Fugen zwischen den Palettenbrettern."),
                "merged": T("Seat ring, skirt and buffers in one band down to the rim (the 8 mm buffer gap is under the 15 mm feature); spigot and cap in one outline; blocks and cross boards of the pallet in three stringers.", "Sitzring, Schürze und Puffer in einem Band bis auf den Rand (der 8-mm-Pufferspalt liegt unter dem 15-mm-Merkmal); Stutzen und Kappe in einem Umriss; Klötze und Querbretter der Palette in drei Querträgern."),
                "omitted": T("Silicone bead, glaze chip, limescale ring, sticker, tape, lid scratches and the grain (all under 0.15 mm); the hinge pin.", "Silikonfuge, Glasurabplatzung, Kalkrand, Aufkleber, Klebeband, Deckelkratzer und Maserung (alle unter 0,15 mm); der Scharnierstift."),
                "thinElements": T("Cardboard 5 mm and button rim 5 mm are carried by the pens; the 20 mm lid by the 0.35 mm pen.", "Pappe 5 mm und Drückerrand 5 mm tragen die Stifte; den 20-mm-Deckel der 0,35-mm-Stift."),
                "hatching": T("None.", "Keine."),
                "styles": T("As at 1:50: button 0.98, cistern lid 0.94, bowl 0.92, cistern 0.9, foot 0.86, lid 0.86, seat 0.82, pallet 0.6-0.72, hinge 0.55, cap 0.3.", "Wie bei 1:50: Drücker 0,98, Spülkastendeckel 0,94, Becken 0,92, Spülkasten 0,9, Fuß 0,86, Deckel 0,86, Sitz 0,82, Palette 0,6-0,72, Scharnier 0,55, Kappe 0,3."),
                "overlays": T("Seat and bowl lie under the lid in plan and show when it is raised.", "Sitz und Becken liegen im Grundriss unter dem Deckel und erscheinen, wenn er geöffnet ist."),
            }}


def rep200():
    """🖋️ The 1:200 drawing: one bowl outline, foot, rear body with deck, outlet, seat and lid as two bands, cistern with lid and button, pallet deck and runners."""
    el, fr, pl = pallet_views(200)
    p, c = "pan", "cistern"
    prof = bowl_profile200()
    seat_out = seat_plan_ring(seat_outline200())
    out_pts = [(CAP["x0"], PH + SPIGOT["y"] - CAP["ro"]), (CAP["x1"], PH + SPIGOT["y"] - CAP["ro"]), (CAP["x1"], PH + SPIGOT["y"] - SPIGOT["ro"]), (SPIGOT["x1"], PH + SPIGOT["y"] - SPIGOT["ro"]), (SPIGOT["x1"], PH + SPIGOT["y"] + SPIGOT["ro"]), (CAP["x1"], PH + SPIGOT["y"] + SPIGOT["ro"]), (CAP["x1"], PH + SPIGOT["y"] + CAP["ro"]), (CAP["x0"], PH + SPIGOT["y"] + CAP["ro"])]
    el += [
        sh("foot-200", p, "body", 2, True, 0.86, epoly(p, foot_side("elevation")), **PAN),
        sh("outlet-200", p, "body", 2.1, True, 0.3, epoly(p, out_pts), **CAPR),
        sh("rear-body-200", p, "body", 2.3, True, 0.88, erect(p, REAR["x0"], PH + REAR["y0"], REAR["x1"], PH + 0.37), **PAN),
        sh("neck-200", p, "body", 2.4, True, 0.86, epoly(p, neck_profile()), **PAN),
        sh("bowl-200", p, "body", 3, True, 0.92, epoly(p, lifted(envelope_outline(prof, XB))), **PAN),
        sh("deck-200", p, "body", 3.1, True, 0.9, erect(p, DECK["x0"], PH + 0.37, DECK["x1"], PH + DECK["y1"]), **PAN),
        sh("cistern-200", c, "body", 4, True, 0.9, erect(c, CIS["x0"], PH + CIS["y0"], CIS["x1"], PH + 0.76), **CISR),
        sh("cistern-lid-200", "cistern-lid", "body", 4.3, True, 0.94, erect("cistern-lid", CLID["x0"], PH + 0.76, CLID["x1"], PH + CLID["y1"]), **CISR),
        sh("seat-ring-200", "seat", "body", 5, True, 0.82, erect("seat", HINGE[0], PH + 0.40, SEAT["cx"] + SEAT["rx"], PH + 0.43), **SEATR),
        sh("lid-200", "lid", "body", 6, True, 0.86, erect("lid", HINGE[0], PH + 0.43, SEAT["cx"] + SEAT["rx"], PH + 0.46), **SEATR),
    ]
    fr += [
        sh("rear-body-200", p, "body", 2, True, 0.88, wrect(-REAR["z"], PH + REAR["y0"], REAR["z"], PH + 0.37), **PAN),
        sh("foot-200", p, "body", 2.1, True, 0.86, wpoly(foot_side("front")), **PAN),
        sh("cistern-200", c, "body", 2.5, True, 0.9, wrect(-CIS["z"], PH + CIS["y0"], CIS["z"], PH + 0.76), **CISR),
        sh("cistern-lid-200", "cistern-lid", "body", 2.7, True, 0.94, wrect(-CLID["z"], PH + 0.76, CLID["z"], PH + CLID["y1"]), **CISR),
        sh("bowl-200", p, "body", 3, True, 0.92, wpoly(lifted(envelope_outline(prof, 0.0, LS))), **PAN),
        sh("seat-ring-200", "seat", "body", 4.1, True, 0.82, wrect(-SEAT["rz"], PH + 0.40, SEAT["rz"], PH + 0.43), **SEATR),
        sh("lid-200", "lid", "body", 5, True, 0.86, wrect(-SEAT["rz"], PH + 0.43, SEAT["rz"], PH + 0.46), **SEATR),
    ]
    pl += [
        sh("deck-200", p, "body", 2, True, 0.9, wpoly(rounded_rect(DECK["x0"], DECK["x1"], -DECK["z"], DECK["z"], DECK["r"])), **PAN),
        sh("bowl-200", p, "body", 2.1, True, 0.92, wpoly(ellipse_pts(XB, 0.0, 0.245, 0.245 * LS)), **PAN),
        sh("seat-ring-200", "seat", "body", 3.1, True, 0.82, wpoly(seat_out), **SEATR),
        sh("lid-200", "lid", "body", 4.1, True, 0.86, wpoly(seat_out), **SEATR),
        sh("cistern-lid-200", "cistern-lid", "body", 5, True, 0.94, wpoly(rounded_rect(CLID["x0"], CLID["x1"], -CLID["z"], CLID["z"], CLID["r"])), **CISR),
    ]
    return {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350}, "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            "hatches": [], "geometry": {"elevation": el, "front": fr, "plan": pl},
            "decisions": {
                "silhouette": T("WC 3.4 x 4.0 mm on a 4 mm pallet: the bowl as one outline over the foot, the rear body with deck and neck, the red outlet, seat and lid as two 30 mm bands, the cistern with its lid and button; the pallet as deck and three runners.", "WC 3,4 x 4,0 mm auf einer 4-mm-Palette: das Becken als ein Umriss über dem Fuß, der Hinterkörper mit Auflage und Hals, der rote Abgang, Sitz und Deckel als zwei 30-mm-Bänder, der Spülkasten mit Deckel und Drücker; die Palette als Deck und drei Kufen."),
                "retained": T("Rim height 0.40 m over the pallet, the cistern rising to 0.79 m, the outlet at the back and the openings of the pallet.", "Randhöhe 0,40 m über der Palette, der bis 0,79 m reichende Spülkasten, der Abgang hinten und die Öffnungen der Palette."),
                "merged": T("Seat and lid reach back to the hinge axis (lugs and brackets under the 30 mm feature); the five top boards and the cross boards as one deck; blocks and bottom board as a runner.", "Sitz und Deckel reichen bis zur Scharnierachse (Laschen und Halter unter dem 30-mm-Merkmal); die fünf Deckbretter und die Querbretter als ein Deck; Klötze und Bodenbrett als Kufe."),
                "omitted": T("Hinge, buffers, the bowl opening in plan and every trace of use (under 0.15 mm at 1:200).", "Scharnier, Puffer, die Beckenöffnung im Grundriss und alle Gebrauchsspuren (unter 0,15 mm bei 1:200)."),
                "thinElements": T("The 30 mm seat, lid and cistern lid bands and the 5 mm cardboard are carried by the 0.25 mm pen.", "Die 30-mm-Bänder von Sitz, Deckel und Spülkastendeckel und die 5-mm-Pappe trägt der 0,25-mm-Stift."),
                "hatching": T("None.", "Keine."),
                "styles": T("Button 0.98, cistern lid 0.94, bowl 0.92, cistern 0.9, foot 0.86, lid 0.86, seat 0.82, pallet 0.6-0.7, cap 0.3.", "Drücker 0,98, Spülkastendeckel 0,94, Becken 0,92, Spülkasten 0,9, Fuß 0,86, Deckel 0,86, Sitz 0,82, Palette 0,6-0,7, Kappe 0,3."),
                "overlays": T("The seat lies under the lid in plan.", "Der Sitz liegt im Grundriss unter dem Deckel."),
            }}


def rep500():
    """🚫️ 1:500 is not drawn: the whole WC is about one and a half millimetres of paper."""
    return {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
            "reason": T("The WC is 0.67 x 0.37 x 0.80 m = 1.3 x 0.7 x 1.6 mm on paper at 1:500, under 3 mm: seat, lid and rim (20-45 mm = 0.04-0.09 mm), the outlet (0.12 m = 0.23 mm) and the cistern lid would merge into one anonymous speck; the pallet under it (1.6 x 0.3 mm) with it.", "Das WC misst 0,67 x 0,37 x 0,80 m = 1,3 x 0,7 x 1,6 mm auf dem Papier bei 1:500, unter 3 mm: Sitz, Deckel und Rand (20-45 mm = 0,04-0,09 mm), der Abgang (0,12 m = 0,23 mm) und der Spülkastendeckel würden zu einem namenlosen Fleck verschmelzen; die Palette darunter (1,6 x 0,3 mm) mit ihm."),
            "limitingFeature": T("Seat ring and lid, 20-31 mm (0.04-0.06 mm on paper)", "Sitzring und Deckel, 20-31 mm (0,04-0,06 mm auf dem Papier)"),
            "limitingSizeM": 0.02, "alternatives": [{"kind": "scale", "denominator": 200}]}
