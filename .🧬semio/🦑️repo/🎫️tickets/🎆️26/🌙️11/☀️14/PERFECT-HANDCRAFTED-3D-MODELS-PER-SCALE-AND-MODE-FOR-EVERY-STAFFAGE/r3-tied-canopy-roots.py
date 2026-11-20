"""🧰 Round 3 tied canopy: the root corrections outside `volumes` (2D drawings of all four scales redrawn as the true views of the corrected construction, the EPDM material, the strut angle in the structural assumptions, the duplicated clip events, region areas measured on the 1:50 model).

Usage: python3 r3-tied-canopy-roots.py            writes the candidate document to 🗑️generated/r3-tied-canopy/🔣️.json (volumes = the ticket source)
       python3 r3-tied-canopy-roots.py --apply    writes the changed members (never `volumes`) into the catalog file in its own formatting, re-reading it right before

See: r3-components-tied-canopy.md, round3-brief.md (ruling E1)
"""

import copy
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/tied-canopy/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-tied-canopy", "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "components", "tied-canopy.json")
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-tied-canopy-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])
r6, pt = k.r6, k.pt
H = 2.33
AREAS = {"rod-steel": 0.95, "canopy-glass": 5.73}


def text(en, de):
    """🗣️ A bilingual sentence."""
    return {"en": en, "de": de}


def base(id, role, z, tone, region=None, material=None, filled=True, **rest):
    """🏷️ The members every shape of the canopy bone carries."""
    out = {"id": id, "bone": "canopy", "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        out["region"] = region
    if material:
        out["material"] = material
    out.update(rest)
    return out


def rect(x, y, w, h):
    """▭ Rect geometry."""
    return {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}


def poly(points, closed=True):
    """⬠ Poly geometry."""
    return {"kind": "poly", "points": [pt(p) for p in points], "closed": closed}


def bounds(points):
    """📏 Bounding box of points."""
    xs, ys = [p[0] for p in points], [p[1] for p in points]
    return min(xs), min(ys), max(xs), max(ys)


def keep(old, id, **geometry):
    """♻️ An existing shape with new geometry: its id, bone, role, z, fill, tone, region, material, hatch and pen stay."""
    s = {key: value for key, value in old[id].items() if key in ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm", "title")}
    s.update(geometry)
    return s


def tube_outline(t0, t1, cones):
    """🧪 Elevation outline of the strut tube between t0 and t1 (from the beam pin), with the shank and end cones when `cones`."""
    if cones:
        prof = [(t0, k.SHANK_R), (0.115, k.SHANK_R), (0.145, k.TUBE_R), (k.L - 0.145, k.TUBE_R), (k.L - 0.115, k.SHANK_R), (t1, k.SHANK_R)]
    else:
        prof = [(t0, k.TUBE_R), (t1, k.TUBE_R)]
    return [k.along(k.PB, k.AXIS_B, s, -n) for s, n in prof] + [k.along(k.PB, k.AXIS_B, s, n) for s, n in reversed(prof)]


def lateral_fork(pin, axis, coord, cheek_only=False):
    """🔱 Front or plan outline of a fork head: the cheeks 37 mm wide, then the shank and cone to the tube (coord 1 = height for the front, 0 = x for the plan)."""
    pts = k.cheek(pin, axis)
    c = [p[coord] for p in pts]
    sgn = 1 if axis[coord] > 0 else -1
    start = min(c) if sgn > 0 else max(c)
    at = lambda s: pin[coord] + s * axis[coord]
    if cheek_only:
        end = max(c) if sgn > 0 else min(c)
        return [(-k.CHEEK_OUT, start), (k.CHEEK_OUT, start), (k.CHEEK_OUT, end), (-k.CHEEK_OUT, end)]
    return [(-k.CHEEK_OUT, start), (k.CHEEK_OUT, start), (k.CHEEK_OUT, at(0.125)), (k.TUBE_R, at(0.145)), (k.TUBE_R, at(0.16)), (-k.TUBE_R, at(0.16)), (-k.TUBE_R, at(0.145)), (-k.CHEEK_OUT, at(0.125))]


def pin_side(c, r, head):
    """📌 A pin seen from the side: shank with a head and a collar 8 mm deep (lateral -0.0265..0.0265)."""
    a, b = 0.0265, 0.0185
    return [(-a, -head), (-b, -head), (-b, -r), (b, -r), (b, -head), (a, -head), (a, head), (b, head), (b, r), (-b, r), (-b, head), (-a, head)]


def shift(points, dx, dy, swap=False):
    """↗️ Moves (and optionally swaps) 2D points."""
    return [(dx + (p[1] if swap else p[0]), dy + (p[0] if swap else p[1])) for p in points]


def front_plan_pair(points, coord):
    """🔁 Lateral outline (u = lateral, v = along coord) as front (x = lateral, y = height) or plan (x = ahead, y = lateral) points."""
    return [(u, v + H) for u, v in points] if coord == 1 else [(v, u) for u, v in points]


def glass_plan(i, notch_half=k.NOTCH_HALF):
    """🪟 Plan outline of pane i (pane 2 with its notch)."""
    x0, x1 = k.PANES[i]
    if i != 2:
        return rect(x0, -0.5, x1 - x0, 1.0)
    n0 = k.NOTCH[0]
    return poly([(x0, -0.5), (x0, 0.5), (x1, 0.5), (x1, notch_half), (n0, notch_half), (n0, -notch_half), (x1, -notch_half), (x1, -0.5)])


def redraw50(rep):
    """✏️ 1:50: elevation, front and plan of the corrected construction."""
    g = rep["geometry"]
    E, F, P = ({s["id"]: s for s in g[v]} for v in ("elevation", "front", "plan"))
    gb, gw = k.gusset_beam(), k.gusset_wall()
    pins = [(k.PB, "pin-beam"), (k.PW, "pin-wall")]
    elevation = [
        E["wall-core"], E["wall-rebar-inner"], E["wall-rebar-outer"], E["wall-insulation"], E["wall-insulation-zigzag"], E["wall-render"],
        keep(E, "plate-beam", **rect(-0.02, -0.1, 0.02, 0.2)), E["pad-beam"],
        keep(E, "plate-strut", **rect(-0.02, k.STRUT_PLATE_Y - 0.1, 0.02, 0.2)), E["pad-strut"],
        *[base(f"nut-{a}-{b}", "detail", 1.55, 0.25, "wall-plate", "steel", **rect(0.0, yc + sy * k.BOLT_Y - k.NUT_R, k.NUT_H, 2 * k.NUT_R)) for a, yc in (("beam", 0.0), ("strut", k.STRUT_PLATE_Y)) for b, sy in (("top", 1), ("bottom", -1))],
        E["beam"],
        keep(E, "beam-flange-top", **poly([(0.0, 0.0726), (k.BEAM_END - k.END_PLATE, 0.0726)], False)),
        keep(E, "beam-flange-bottom", **poly([(0.0, -0.0726), (k.BEAM_END - k.END_PLATE, -0.0726)], False)),
        base("beam-end-plate", "detail", 3.1, 0.3, "canopy-steel", filled=False, **poly([(k.BEAM_END - k.END_PLATE, -0.08), (k.BEAM_END - k.END_PLATE, 0.08)], False)),
        base("bearing-wall", "detail", 3.2, 0.15, None, "epdm", **rect(k.GLASS_X0, k.FLANGE_TOP, k.NOTCH[0] - 0.006 - k.GLASS_X0, k.BEARING)),
        base("bearing-tip", "detail", 3.2, 0.15, None, "epdm", **rect(4.05, k.FLANGE_TOP, k.BEAM_END - 0.045 - 4.05, k.BEARING)),
        base("gusset-beam", "body", 3.3, 0.3, "rod-steel", "steel", **poly(gb)),
        *[keep(E, f"glass-{i}", **rect(x0, k.GLASS_Y0, x1 - x0, k.GLASS_T)) for i, (x0, x1) in enumerate(k.PANES)],
        *[base(f"joint-{i}", "detail", 3.55, 0.15, None, "epdm", **rect(x, k.GLASS_Y0, k.JOINT, k.GLASS_T)) for i, x in enumerate(k.JOINTS)],
        base("joint-wall", "detail", 3.55, 0.15, None, "epdm", **rect(0.0, k.GLASS_Y0, k.GLASS_X0, k.GLASS_T)),
        *[base(f"clamp-{i}", "detail", 3.6, 0.25, "canopy-steel", "steel", **rect(x - k.CLAMP_L / 2, k.GLASS_Y1, k.CLAMP_L, k.CLAMP_T)) for i, x in enumerate(k.CLAMPS)],
        keep(E, "strut-tube", **poly(tube_outline(0.155, k.L - 0.155, False))),
        keep(E, "strut-axis", **poly([k.PB, k.PW], False)),
        base("gusset-wall", "body", 5.15, 0.3, "rod-steel", "steel", **poly(gw)),
        keep(E, "clevis-beam", **poly(k.fork_outline(k.PB, k.AXIS_B))),
        keep(E, "clevis-wall", **poly(k.fork_outline(k.PW, k.AXIS_W))),
        *[keep(E, id, **{"kind": "ellipse", "cx": r6(p[0]), "cy": r6(p[1]), "rx": k.PIN_HEAD_R, "ry": k.PIN_HEAD_R}) for p, id in pins],
    ]
    gbx, gby = bounds(gb)[0::2], bounds(gb)[1::2]
    gwx, gwy = bounds(gw)[0::2], bounds(gw)[1::2]
    t0, t1 = k.along(k.PB, k.AXIS_B, 0.155, 0), k.along(k.PB, k.AXIS_B, k.L - 0.155, 0)
    front = [
        F["wall-face"], F["pad-beam"], F["pad-strut"], F["plate-beam"], F["plate-strut"], F["beam"],
        base("bearing-tip", "detail", 3.2, 0.15, None, "epdm", **rect(-0.03, H + k.FLANGE_TOP, 0.06, k.BEARING)),
        keep(F, "glass-0", **rect(-0.5, H + k.GLASS_Y0, 1.0, k.GLASS_T)),
        base("clamp-4", "detail", 3.6, 0.25, "canopy-steel", "steel", **rect(-k.CLAMP_W / 2, H + k.GLASS_Y1, k.CLAMP_W, k.CLAMP_T)),
        base("gusset-beam", "body", 4.4, 0.3, "rod-steel", "steel", **rect(-k.GUSSET_HALF, H + gby[0], 2 * k.GUSSET_HALF, gby[1] - gby[0])),
        base("gusset-wall", "body", 4.4, 0.3, "rod-steel", "steel", **rect(-k.GUSSET_HALF, H + gwy[0], 2 * k.GUSSET_HALF, gwy[1] - gwy[0])),
        keep(F, "strut-tube", **rect(-k.TUBE_R, H + t0[1], 2 * k.TUBE_R, t1[1] - t0[1])),
        keep(F, "clevis-beam", **poly(front_plan_pair(lateral_fork(k.PB, k.AXIS_B, 1), 1))),
        keep(F, "clevis-wall", **poly(front_plan_pair(lateral_fork(k.PW, k.AXIS_W, 1), 1))),
        F["beam-flange-top"], F["beam-flange-bottom"],
        *[base(id, "detail", 5.3, 0.1, "rod-steel", "steel", **poly(shift(pin_side(None, k.PIN_R, k.PIN_HEAD_R), 0.0, H + p[1]))) for p, id in pins],
        keep(F, "strut-axis", **poly([(0.0, H + k.PB[1]), (0.0, H + k.PW[1])], False)),
    ]
    plan = [
        P["wall-core"], P["wall-insulation"], P["wall-render"],
        *[keep(P, f"glass-{i}", **glass_plan(i)) for i in range(4)],
        *[base(f"joint-{i}", "detail", 2.1, 0.15, None, "epdm", **rect(x, -0.5, k.JOINT, 1.0)) for i, x in enumerate(k.JOINTS)],
        base("joint-wall", "detail", 2.1, 0.15, None, "epdm", **rect(0.0, -0.5, k.GLASS_X0, 1.0)),
        base("notch-seal", "detail", 2.1, 0.15, None, "epdm", **rect(k.NOTCH[0], -k.NOTCH_HALF, k.NOTCH[1] - k.NOTCH[0], 2 * k.NOTCH_HALF)),
        P["beam"], keep(P, "beam-web", **poly([(0.0, 0.0), (k.BEAM_END - k.END_PLATE, 0.0)], False)),
        *[base(f"clamp-{i}", "detail", 3.6, 0.25, "canopy-steel", "steel", **rect(x - k.CLAMP_L / 2, -k.CLAMP_W / 2, k.CLAMP_L, k.CLAMP_W)) for i, x in enumerate(k.CLAMPS)],
        base("gusset-beam", "body", 4.4, 0.3, "rod-steel", "steel", **rect(gbx[0], -k.GUSSET_HALF, gbx[1] - gbx[0], 2 * k.GUSSET_HALF)),
        base("gusset-wall", "body", 4.4, 0.3, "rod-steel", "steel", **rect(gwx[0], -k.GUSSET_HALF, gwx[1] - gwx[0], 2 * k.GUSSET_HALF)),
        keep(P, "strut-tube", **rect(t1[0], -k.TUBE_R, t0[0] - t1[0], 2 * k.TUBE_R)),
        keep(P, "strut-axis", **poly([(k.PW[0], 0.0), (k.PB[0], 0.0)], False)),
        keep(P, "clevis-beam", **poly(front_plan_pair(lateral_fork(k.PB, k.AXIS_B, 0), 0))),
        base("clevis-wall", "body", 5.2, 0.3, "rod-steel", "steel", **poly(front_plan_pair(lateral_fork(k.PW, k.AXIS_W, 0), 0))),
        *[base(id, "detail", 5.3, 0.1, "rod-steel", "steel", **poly([(p[0] + v, u) for u, v in pin_side(None, k.PIN_R, k.PIN_HEAD_R)])) for p, id in pins],
    ]
    g["elevation"], g["front"], g["plan"] = elevation, front, plan
    d = rep["decisions"]
    d["silhouette"] = text(
        "The full detail of the tied canopy: the insulated concrete wall with reinforcement, two anchor elements (separation pad, 20 mm end plate, four M16 nuts), the IPE 160 beam with flange lines and its tip plate, the EPDM bearing strip, four laminated glass panes with 20 mm EPDM-sealed joints and a 30 mm EPDM wall joint, five clamp plates, and the tube strut with a fork head on a gusset at each end, pinned with 24 mm pins. Front and plan: the front shows the render face, the pads with their plates, the beam end, bearing, glass edge and tip clamp, the 60 mm tube with axis line, both fork heads, gussets and pins; the plan shows the wall top (concrete stipple, insulation, render), the four panes with their dark joint gaskets (the third notched round the gusset), the clamps, the beam with its web line seen through the glass, the strut with axis, fork heads, gussets and pins. The canopy hangs at door-head height, beam underside 2.25 m above the ground; the wall rises to 5.35 m and the strut anchor is 5.21 m above the ground.",
        "Das volle Detail des abgespannten Vordachs: die gedämmte Betonwand mit Bewehrung, zwei Ankerelemente (Trennlage, 20-mm-Stirnplatte, vier Muttern M16), der Träger IPE 160 mit Flanschlinien und Kopfplatte, das EPDM-Auflagerband, vier Verbundglasscheiben mit 20-mm-Fugen mit EPDM-Dichtung und einer 30-mm-EPDM-Wandfuge, fünf Klemmplatten und der Rohrstab mit je einem Gabelkopf auf einem Knotenblech an beiden Enden, mit 24-mm-Bolzen angeschlossen. Vorder- und Draufsicht: Die Vorderansicht zeigt die Putzfläche, die Trennlagen mit ihren Platten, das Trägerende, Auflager, Glaskante und Endklemme, das Rohr 60 mm mit Achslinie, beide Gabelköpfe, Knotenbleche und Bolzen; die Draufsicht zeigt die Wandkrone (Betonpunktraster, Dämmung, Putz), die vier Scheiben mit ihren dunklen Fugendichtungen (die dritte um das Knotenblech ausgeklinkt), die Klemmen, den Träger mit seiner durch das Glas sichtbaren Steglinie, den Stab mit Achse, Gabelköpfen, Knotenblechen und Bolzen. Das Vordach hängt in Türsturzhöhe, Trägerunterkante 2,25 m über dem Boden; die Wand steigt auf 5,35 m, der Stabanschluss liegt 5,21 m über dem Boden.")
    d["retained"] = text(
        "Wall core (6 mm) with two bars, 180 mm insulation with zigzag, render, end plates (0.4 x 4 mm) and pads, nuts (0.26 x 0.55 mm), beam (3.2 mm tall) with flange lines and tip plate line, bearing strip (0.16 mm), four panes (27 x 0.34 mm, joints 0.4 mm), clamps (1.2 x 0.16 mm), gussets, fork heads, tube, pins (0.72 mm heads).",
        "Wandkern (6 mm) mit zwei Stäben, 180 mm Dämmung mit Zickzack, Putz, Stirnplatten (0,4 x 4 mm) und Trennlagen, Muttern (0,26 x 0,55 mm), Träger (3,2 mm hoch) mit Flanschlinien und Kopfplattenlinie, Auflagerband (0,16 mm), vier Scheiben (27 x 0,34 mm, Fugen 0,4 mm), Klemmen (1,2 x 0,16 mm), Knotenbleche, Gabelköpfe, Rohr, Bolzen (Köpfe 0,72 mm).")
    d["merged"] = text(
        "Each fork head is one outline of cheeks, shank and end cone; the render below the beam plate is drawn together with the wool.",
        "Jeder Gabelkopf ist ein Umriss aus Laschen, Schaft und Endkegel; der Putz unter der Trägerplatte ist mit der Wolle zusammen gezeichnet.")
    d["omitted"] = text(
        "Weld seams, the clamp bolts in the joints, the stud threads and the hollow-section wall of the strut.",
        "Schweißnähte, die Klemmschrauben in den Fugen, die Gewinde der Ankerbolzen und die Wand des Rohrprofils.")
    d["thinElements"] = text(
        "The tube wall (3.2 mm, 0.06 mm) is not resolved; the 16.8 mm glass is a 0.34 mm filled strip with a 0.13 mm outline, the 8 mm EPDM strip and clamps 0.16 mm filled strips, the 20 mm render a 0.4 mm strip.",
        "Die Rohrwand (3,2 mm, 0,06 mm) wird nicht aufgelöst; das 16,8-mm-Glas ist ein 0,34 mm hoher gefüllter Streifen mit 0,13 mm Kontur, EPDM-Band und Klemmen mit 8 mm sind 0,16-mm-Streifen, der 20-mm-Putz ein 0,4 mm breiter Streifen.")
    d["overlays"] = text(
        "Wall layers, plates with pads, beam, each glass pane and the strut with its fork heads and gussets are separate overlay cells, so the cold plate bridges read against the insulated wall.",
        "Wandschichten, Platten mit Trennlagen, Träger, jede Glasscheibe und der Stab mit Gabelköpfen und Knotenblechen sind getrennte Überlagerungszellen, so heben sich die kalten Plattenbrücken von der gedämmten Wand ab.")


def redraw100(rep):
    """✏️ 1:100: the same construction with merged fork heads and glass slabs."""
    g = rep["geometry"]
    E, F, P = ({s["id"]: s for s in g[v]} for v in ("elevation", "front", "plan"))
    gb, gw = k.gusset_beam(), k.gusset_wall()
    gbx, gby = bounds(gb)[0::2], bounds(gb)[1::2]
    gwx, gwy = bounds(gw)[0::2], bounds(gw)[1::2]
    ym = (k.FLANGE_TOP + k.GLASS_Y1) / 2
    elevation = [
        E["wall-core"], E["wall-insulation"], E["wall-insulation-zigzag"], E["plate-beam"], E["plate-strut"], E["beam"],
        base("gusset-beam", "body", 3.3, 0.3, "rod-steel", "steel", **poly(gb)),
        *[keep(E, f"glass-sheet-{i}", penMm=0.25, **poly([(x0, ym), (x1, ym)], False)) for i, (x0, x1) in enumerate(k.PANES)],
        *[base(f"joint-{i}", "detail", 3.55, 0.15, None, "epdm", **rect(x, k.FLANGE_TOP, k.JOINT, k.GLASS_Y1 - k.FLANGE_TOP)) for i, x in enumerate(k.JOINTS)],
        base("joint-wall", "detail", 3.55, 0.15, None, "epdm", **rect(0.0, k.FLANGE_TOP, k.GLASS_X0, k.GLASS_Y1 - k.FLANGE_TOP)),
        keep(E, "strut", **poly(tube_outline(0.095, k.L - 0.095, True))),
        base("gusset-wall", "body", 5.15, 0.3, "rod-steel", "steel", **poly(gw)),
        keep(E, "clevis-beam", **poly(k.cheek(k.PB, k.AXIS_B))),
        keep(E, "clevis-wall", **poly(k.cheek(k.PW, k.AXIS_W))),
        *[keep(E, id, **{"kind": "ellipse", "cx": r6(p[0]), "cy": r6(p[1]), "rx": 0.0125, "ry": 0.0125}) for p, id in ((k.PB, "pin-beam"), (k.PW, "pin-wall"))],
    ]
    a, b = k.along(k.PB, k.AXIS_B, 0.095, 0), k.along(k.PB, k.AXIS_B, k.L - 0.095, 0)
    c0, c1 = k.along(k.PB, k.AXIS_B, 0.115, 0), k.along(k.PB, k.AXIS_B, 0.145, 0)
    d0, d1 = k.along(k.PB, k.AXIS_B, k.L - 0.145, 0), k.along(k.PB, k.AXIS_B, k.L - 0.115, 0)
    lat = lambda c: [(-k.SHANK_R, a[c]), (k.SHANK_R, a[c]), (k.SHANK_R, c0[c]), (k.TUBE_R, c1[c]), (k.TUBE_R, d0[c]), (k.SHANK_R, d1[c]), (k.SHANK_R, b[c]), (-k.SHANK_R, b[c]), (-k.SHANK_R, d1[c]), (-k.TUBE_R, d0[c]), (-k.TUBE_R, c1[c]), (-k.SHANK_R, c0[c])]
    front = [
        F["wall-face"], F["pad-beam"], F["pad-strut"], F["beam"],
        keep(F, "glass-0", **rect(-0.5, H + k.FLANGE_TOP, 1.0, k.GLASS_Y1 - k.FLANGE_TOP)),
        base("gusset-beam", "body", 4.4, 0.3, "rod-steel", "steel", **rect(-k.GUSSET_HALF, H + gby[0], 2 * k.GUSSET_HALF, gby[1] - gby[0])),
        base("gusset-wall", "body", 4.4, 0.3, "rod-steel", "steel", **rect(-k.GUSSET_HALF, H + gwy[0], 2 * k.GUSSET_HALF, gwy[1] - gwy[0])),
        keep(F, "strut-tube", **poly(front_plan_pair(lat(1), 1))),
        keep(F, "clevis-beam", **poly(front_plan_pair(lateral_fork(k.PB, k.AXIS_B, 1, True), 1))),
        keep(F, "clevis-wall", **poly(front_plan_pair(lateral_fork(k.PW, k.AXIS_W, 1, True), 1))),
    ]
    plan = [
        P["wall-core"], P["wall-insulation"],
        *[keep(P, f"glass-{i}", **glass_plan(i)) for i in range(4)],
        *[base(f"joint-{i}", "detail", 2.1, 0.15, None, "epdm", **rect(x, -0.5, k.JOINT, 1.0)) for i, x in enumerate(k.JOINTS)],
        base("joint-wall", "detail", 2.1, 0.15, None, "epdm", **rect(0.0, -0.5, k.GLASS_X0, 1.0)),
        base("notch-seal", "detail", 2.1, 0.15, None, "epdm", **rect(k.GUSSET_AT_GLASS[1], -k.NOTCH_HALF, k.NOTCH[1] - k.GUSSET_AT_GLASS[1], 2 * k.NOTCH_HALF)),
        P["beam"],
        base("gusset-beam", "body", 4.4, 0.3, "rod-steel", "steel", **rect(gbx[0], -k.GUSSET_HALF, gbx[1] - gbx[0], 2 * k.GUSSET_HALF)),
        base("gusset-wall", "body", 4.4, 0.3, "rod-steel", "steel", **rect(gwx[0], -k.GUSSET_HALF, gwx[1] - gwx[0], 2 * k.GUSSET_HALF)),
        keep(P, "strut-tube", **poly(front_plan_pair(lat(0), 0))),
        keep(P, "clevis-beam", **poly(front_plan_pair(lateral_fork(k.PB, k.AXIS_B, 0, True), 0))),
        base("clevis-wall", "body", 5.2, 0.3, "rod-steel", "steel", **poly(front_plan_pair(lateral_fork(k.PW, k.AXIS_W, 0, True), 0))),
    ]
    g["elevation"], g["front"], g["plan"] = elevation, front, plan
    d = rep["decisions"]
    d["silhouette"] = text(
        "A hatched concrete wall with insulation, two anchor blocks, a steel beam with four glass panes and a strut with a fork head on a gusset at each end. Front and plan: the front adds the wall pads, the end of the beam, the 60 mm tube with its end cones, the fork heads and gussets; the plan draws the wall top with concrete stipple, the four panes with their joints and the notch of the third, the beam under them and the strut above them with both fork heads and gussets. The canopy hangs at door-head height, beam underside 2.25 m above the ground; the wall rises to 5.35 m and the strut anchor is 5.21 m above the ground.",
        "Eine schraffierte Betonwand mit Dämmung, zwei Ankerblöcke, ein Stahlträger mit vier Glasscheiben und ein Stab mit je einem Gabelkopf auf einem Knotenblech an beiden Enden. Vorder- und Draufsicht: Die Vorderansicht ergänzt die Wandplatten, das Trägerende, das Rohr 60 mm mit seinen Endkegeln, die Gabelköpfe und Knotenbleche; die Draufsicht zeichnet die Wandkrone mit Betonpunktraster, die vier Scheiben mit ihren Fugen und der Ausklinkung der dritten, den Träger darunter und den Stab darüber mit beiden Gabelköpfen und Knotenblechen. Das Vordach hängt in Türsturzhöhe, Trägerunterkante 2,25 m über dem Boden; die Wand steigt auf 5,35 m, der Stabanschluss liegt 5,21 m über dem Boden.")
    d["retained"] = text(
        "Wall core (3 mm, stipple), insulation with zigzag (2 mm), two anchor blocks (2 x 2.4 mm), beam (1.6 mm tall), four panes (13.5 mm each) with three 0.2 mm joint gaskets, gussets, fork heads, strut with end cones (0.6 mm), two pin dots.",
        "Wandkern (3 mm, Punktierung), Dämmung mit Zickzack (2 mm), zwei Ankerblöcke (2 x 2,4 mm), Träger (1,6 mm hoch), vier Scheiben (je 13,5 mm) mit drei 0,2-mm-Fugendichtungen, Knotenbleche, Gabelköpfe, Stab mit Endkegeln (0,6 mm), zwei Bolzenpunkte.")
    d["merged"] = text(
        "The 20 mm render is merged into the insulation shape; pad and end plate are one anchor block; each glass pane and its EPDM bearing are one 25 mm line; each fork head is one outline of its two cheeks.",
        "Der 20-mm-Putz ist in der Dämmungsform zusammengefasst; Trennlage und Stirnplatte sind ein Ankerblock; jede Glasscheibe mit ihrem EPDM-Auflager ist eine 25-mm-Linie; jeder Gabelkopf ist ein Umriss seiner beiden Laschen.")
    d["omitted"] = text(
        "Flange lines, nuts, clamp plates and tip plate (8 mm, below the 15 mm feature) and the welds.",
        "Flanschlinien, Muttern, Klemmplatten und Kopfplatte (8 mm, unter dem 15-mm-Merkmal) und die Schweißnähte.")
    d["thinElements"] = text(
        "Each 25 mm glass pane with its bearing is a 0.25 mm line, its true thickness; the strut is drawn at its true 0.6 mm width.",
        "Jede 25-mm-Glasscheibe mit Auflager ist eine 0,25-mm-Linie, ihre wahre Dicke; der Stab hat seine wahre Breite von 0,6 mm.")
    d["overlays"] = text(
        "Wall layers, the two anchor blocks, beam, each glass pane and the strut with fork heads and gussets are separate overlay cells.",
        "Wandschichten, die beiden Ankerblöcke, Träger, jede Glasscheibe und der Stab mit Gabelköpfen und Knotenblechen sind getrennte Überlagerungszellen.")


def redraw200(rep):
    """✏️ 1:200: wall bar, beam band, glass line, strut line with two node lugs."""
    g = rep["geometry"]
    E, F, P = ({s["id"]: s for s in g[v]} for v in ("elevation", "front", "plan"))
    nb, nw = k.node_beam(), k.node_wall()
    ym = (k.FLANGE_TOP + k.GLASS_Y1) / 2
    node = lambda id, z, geo: base(id, "body", z, 0.3, "rod-steel", "steel", **geo)
    bx, by = bounds(nb)[0::2], bounds(nb)[1::2]
    wx, wy = bounds(nw)[0::2], bounds(nw)[1::2]
    g["elevation"] = [
        E["wall-core"], E["wall-outer"], E["beam"],
        keep(E, "glass-roof", penMm=0.125, **poly([(0.0, ym), (k.BEAM_END, ym)], False)),
        keep(E, "strut", **poly([k.PB, k.PW], False)),
        node("node-beam", 5.2, poly(nb)), node("node-wall", 5.2, poly(nw)),
    ]
    g["front"] = [
        F["wall-core"], F["beam"],
        keep(F, "glass-roof", **rect(-0.5, H + k.FLANGE_TOP, 1.0, k.GLASS_Y1 - k.FLANGE_TOP)),
        keep(F, "strut", **poly([(0.0, H + k.PB[1]), (0.0, H + k.PW[1])], False)),
        node("node-beam", 5.2, rect(-0.02, H + by[0], 0.04, by[1] - by[0])), node("node-wall", 5.2, rect(-0.02, H + wy[0], 0.04, wy[1] - wy[0])),
    ]
    g["plan"] = [
        P["wall-core"], P["wall-outer"],
        keep(P, "glass-roof", **rect(0.0, -0.5, k.BEAM_END, 1.0)),
        P["beam"],
        keep(P, "strut", **poly([(k.PW[0], 0.0), (k.PB[0], 0.0)], False)),
        node("node-beam", 5.2, rect(bx[0], -0.02, bx[1] - bx[0], 0.04)), node("node-wall", 5.2, rect(wx[0], -0.02, wx[1] - wx[0], 0.04)),
    ]
    d = rep["decisions"]
    d["silhouette"] = text(
        "A two-layer wall bar, a beam band with a glass line on top and a thick strut line between two node lugs, one on the beam, one at the wall. Front and plan: the front shows the wall face, the beam end (IPE 160), the roof as an edge-on glass strip, the strut as a line and both nodes; the plan shows the wall (concrete and insulation), the glass roof, the beam seen through it, the strut line and the nodes. The canopy hangs at door-head height, beam underside 2.25 m above the ground; the wall rises to 5.35 m and the strut anchor is 5.21 m above the ground.",
        "Ein zweischichtiger Wandbalken, ein Trägerband mit einer Glaslinie darauf und eine kräftige Stablinie zwischen zwei Knotenlaschen, eine auf dem Träger, eine an der Wand. Vorder- und Draufsicht: Die Vorderansicht zeigt die Wandfläche, das Trägerende (IPE 160), das Dach als Glasstreifen in Kantenansicht, den Stab als Linie und beide Knoten; die Draufsicht zeigt die Wand (Beton und Dämmung), das Glasdach, den hindurch sichtbaren Träger, die Stablinie und die Knoten. Das Vordach hängt in Türsturzhöhe, Trägerunterkante 2,25 m über dem Boden; die Wand steigt auf 5,35 m, der Stabanschluss liegt 5,21 m über dem Boden.")
    d["retained"] = text(
        "Concrete core (1.5 mm), insulation and render with the anchor plates (1 mm), beam band (0.8 mm tall), glass line, strut line, two node lugs (about 1.5 x 1 mm).",
        "Betonkern (1,5 mm), Dämmung und Putz mit den Ankerplatten (1 mm), Trägerband (0,8 mm hoch), Glaslinie, Stablinie, zwei Knotenlaschen (etwa 1,5 x 1 mm).")
    d["merged"] = text(
        "Mineral wool, render and anchor plates share the outer wall shape; the glass panes with their bearing, joint gaskets and wall joint are one line from the wall; gusset, fork head and pin are one node lug at each strut end.",
        "Mineralwolle, Putz und Ankerplatten teilen sich die äußere Wandform; die Glasscheiben mit Auflager, Fugendichtungen und Wandfuge sind eine Linie ab der Wand; Knotenblech, Gabelkopf und Bolzen sind an jedem Stabende eine Knotenlasche.")
    d["omitted"] = text(
        "Flange lines, glass joints and gaskets, clamps, the tube's end cones and the anchor plate outlines.",
        "Flanschlinien, Glasfugen und Dichtungen, Klemmen, die Endkegel des Rohrs und die Ankerplattenumrisse.")
    d["thinElements"] = text(
        "The 25 mm glass with its bearing is a 0.125 mm pen line (25 mm exact); the 60 mm strut is a 0.3 mm pen line (60 mm exact).",
        "Das 25-mm-Glas mit Auflager ist eine Linie mit 0,125 mm Feder (25 mm genau); der 60-mm-Stab ist eine Linie mit 0,3 mm Feder (60 mm genau).")
    d["overlays"] = text(
        "Wall core, outer wall with plates, beam, glass line, strut and the two nodes are separate overlay cells; the outer wall is a merged layer and plate cell.",
        "Wandkern, Außenwand mit Platten, Träger, Glaslinie, Stab und die zwei Knoten sind getrennte Überlagerungszellen; die Außenwand ist eine zusammengefasste Schicht-und-Platten-Zelle.")


def redraw500(rep):
    """✏️ 1:500: wall bar, beam bar with the roof, strut line to the roof."""
    g = rep["geometry"]
    E, F, P = ({s["id"]: s for s in g[v]} for v in ("elevation", "front", "plan"))
    top = (k.NODE[0] + k.GLASS_Y1 / k.DIR[1] * k.DIR[0], k.GLASS_Y1)
    g["elevation"] = [E["wall"], keep(E, "beam", **rect(0.0, -0.08, k.BEAM_END, 0.08 + k.GLASS_Y1)), keep(E, "strut", **poly([k.WALL_NODE, top], False))]
    g["front"] = [
        F["wall"],
        keep(F, "beam", **poly([(-0.5, H + k.FLANGE_TOP), (-0.041, H + k.FLANGE_TOP), (-0.041, H - 0.08), (0.041, H - 0.08), (0.041, H + k.FLANGE_TOP), (0.5, H + k.FLANGE_TOP), (0.5, H + k.GLASS_Y1), (-0.5, H + k.GLASS_Y1)])),
        keep(F, "strut", **poly([(0.0, H + 0.1), (0.0, H + k.WALL_NODE[1])], False)),
    ]
    g["plan"] = [P["wall"], keep(P, "roof", **rect(0.0, -0.5, k.BEAM_END, 1.0)), P["beam"], keep(P, "strut", **poly([(0.0, 0.0), (top[0], 0.0)], False))]
    d = rep["decisions"]
    d["silhouette"] = text(
        "A tall wall bar on the left, a horizontal canopy bar projecting 5.5 m from it and a single strut line rising from the roof over the beam to the wall; a classic tied canopy diagram. Front and plan: the front is the wall seen from the open side with the canopy as a T (beam under the roof strip) and the strut as a line; the plan is the wall top, the glass roof, the beam seen through it and the strut line from the wall to its node on the roof. The canopy hangs at door-head height, beam underside 2.25 m above the ground; the wall rises to 5.35 m and the strut anchor is 5.21 m above the ground.",
        "Ein hoher Wandbalken links, ein waagerechter Vordachbalken, der 5,5 m daraus auskragt, und eine einzelne Stablinie, die vom Dach über dem Träger zur Wand ansteigt; das klassische Bild eines abgespannten Vordachs. Vorder- und Draufsicht: Die Vorderansicht ist die Wand von der offenen Seite mit dem Vordach als T (Träger unter dem Dachstreifen) und dem Stab als Linie; die Draufsicht ist die Wandkrone, das Glasdach, der hindurch sichtbare Träger und die Stablinie von der Wand zu ihrem Knoten auf dem Dach. Das Vordach hängt in Türsturzhöhe, Trägerunterkante 2,25 m über dem Boden; die Wand steigt auf 5,35 m, der Stabanschluss liegt 5,21 m über dem Boden.")
    d["thinElements"] = text(
        "The 60 mm tube strut is a 0.13 mm pen line (65 mm real); the 0.37 mm beam bar is drawn at its true height of 0.185 m (beam and roof).",
        "Der 60-mm-Rohrstab ist eine Linie mit 0,13 mm Feder (65 mm real); der 0,37 mm hohe Trägerbalken ist in seiner wahren Höhe von 0,185 m gezeichnet (Träger und Dach).")
    d["retained"] = text(
        "Wall (1 x 6.3 mm), beam with glass cover (11 x 0.37 mm) and the strut angle (36 degrees to the beam).",
        "Wand (1 x 6,3 mm), Träger mit Glasdeckung (11 x 0,37 mm) und der Stabwinkel (36 Grad zum Träger).")


def candidate():
    """🧪 The catalog asset with every root correction and the ticket source's volumes."""
    doc = json.load(open(CATALOG, encoding="utf-8"))
    a = doc["asset"]
    if not any(m["id"] == "epdm" for m in a["materials"]):
        a["materials"].append({"id": "epdm", "title": text("EPDM glazing bearing strip", "EPDM-Glasauflagerband"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1150, "conductivityWmK": 0.25, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"})
    for rep, fn in ((a["representations"]["50"], redraw50), (a["representations"]["100"], redraw100), (a["representations"]["200"], redraw200), (a["representations"]["500"], redraw500)):
        fn(rep)
    st = a["structure"]["assumptions"][0]
    st["en"] = st["en"].replace("144.04 deg", "144.25 deg")
    st["de"] = st["de"].replace("144,04 Grad", "144,25 Grad")
    for clip in a["clips"]:
        seen, events = set(), []
        for e in clip["events"]:
            key = json.dumps(e, sort_keys=True)
            if key not in seen:
                seen.add(key)
                events.append(e)
        clip["events"] = events
    for region in a["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    a["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    return doc


def apply():
    """💾 Writes the changed members of the candidate into the catalog file in its own formatting."""
    want = json.load(open(CANDIDATE, encoding="utf-8"))["asset"]
    original = open(CATALOG, encoding="utf-8").read()
    text_ = original
    have = json.loads(text_)["asset"]
    changed = [key for key in have if key != "volumes" and have[key] != want[key]]
    for key in changed:
        start, end = member_range(text_, "    ", key)
        text_ = text_[:start] + "    " + json.dumps(key) + ": " + json.dumps(want[key], indent=2, ensure_ascii=False).replace("\n", "\n    ") + text_[end:]
    got = json.loads(text_)["asset"]
    if list(got) != list(have) or any(got[key] != want[key] for key in want if key != "volumes") or got["volumes"] != have["volumes"]:
        raise SystemExit("the edited text does not match the candidate")
    if open(CATALOG, encoding="utf-8").read() != original:
        raise SystemExit("catalog changed while writing; retry")
    open(CATALOG, "w", encoding="utf-8").write(text_)
    print("written", CATALOG, "members:", ", ".join(changed) or "none")


if __name__ == "__main__":
    if "--apply" in sys.argv:
        apply()
    else:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        doc = candidate()
        with open(CANDIDATE, "w", encoding="utf-8") as f:
            json.dump(doc, f, ensure_ascii=False, indent=2)
            f.write("\n")
        print(CANDIDATE)
