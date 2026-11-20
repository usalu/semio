"""🏗️ Round 3 tied canopy: writes the handcrafted `volumes` (body, thermal, structure, decisions at 1:50, 1:100, 1:200, 1:500) to `volumes/components/tied-canopy.json`.

Every solid is stated here; the node geometry (pins, gussets, fork heads, glass layout) comes from `r3-tied-canopy-kit.py`, which the 2D root script shares.

See: r3-components-tied-canopy.md, round3-brief.md, design.md
"""

import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-tied-canopy-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)
r6, pt = k.r6, k.pt
BONE = "canopy"


def solid(id, kind, material, role, tone, **rest):
    """🧱 A body solid of the canopy bone."""
    return {"id": id, "kind": kind, "bone": BONE, "material": material, "role": role, "tone": tone, **rest}


def box(id, material, role, tone, size, centre, **rest):
    """📦 A box by size and centre."""
    return solid(id, "box", material, role, tone, size=[r6(v) for v in size], transform={"translation": [r6(v) for v in centre]}, **rest)


def xy(id, material, role, tone, profile, z0, z1, **rest):
    """📐 A profile in the bone plane extruded across z."""
    return solid(id, "extrude", material, role, tone, profile=[pt(p) for p in profile], plane="xy", **{"from": r6(z0), "to": r6(z1)}, **rest)


def rev(id, material, role, tone, profile, axis, origin, **rest):
    """🌀 A revolve of (axial, radius) points."""
    return solid(id, "revolve", material, role, tone, profile=[[r6(a), r6(b)] for a, b in profile], axis=[r6(v) for v in axis], origin=[r6(v) for v in origin], **rest)


def ref(id, of, **rest):
    """🔗 A reference to a body solid."""
    return {"id": id, "of": of, **rest}


def text(en, de):
    """🗣️ A bilingual sentence."""
    return {"en": en, "de": de}


WALL_PROFILE = [[-2.33, -0.5], [-2.33, 0.5], [3.02, 0.5], [3.02, -0.5]]
WALL_HOLES = [[[-0.12, -0.1], [-0.12, 0.1], [0.12, 0.1], [0.12, -0.1]], [[2.76, -0.1], [2.76, 0.1], [3.0, 0.1], [3.0, -0.1]]]
IPE = [[0.08, 0.0], [0.08, 0.041], [0.0726, 0.041], [0.0726, 0.0115], [0.071915, 0.008056], [0.069964, 0.005136], [0.067044, 0.003185], [0.0636, 0.0025], [-0.0636, 0.0025], [-0.067044, 0.003185], [-0.069964, 0.005136], [-0.071915, 0.008056], [-0.0726, 0.0115], [-0.0726, 0.041], [-0.08, 0.041], [-0.08, 0.0], [-0.08, -0.041], [-0.0726, -0.041], [-0.0726, -0.0115], [-0.071915, -0.008056], [-0.069964, -0.005136], [-0.067044, -0.003185], [-0.0636, -0.0025], [0.0636, -0.0025], [0.067044, -0.003185], [0.069964, -0.005136], [0.071915, -0.008056], [0.0726, -0.0115], [0.0726, -0.041], [0.08, -0.041]]
IPE_SIMPLE = [[0.08, 0.041], [0.0726, 0.041], [0.0726, 0.0115], [0.0636, 0.0025], [-0.0636, 0.0025], [-0.0726, 0.0115], [-0.0726, 0.041], [-0.08, 0.041], [-0.08, -0.041], [-0.0726, -0.041], [-0.0726, -0.0115], [-0.0636, -0.0025], [0.0636, -0.0025], [0.0726, -0.0115], [0.0726, -0.041], [0.08, -0.041]]
SY = k.STRUT_PLATE_Y


def glass_profile(i, notch_half=k.NOTCH_HALF):
    """🪟 Plan profile (z, x) of pane i; pane 2 carries the notch open at its end for the gusset."""
    x0, x1 = k.PANES[i]
    if i != 2:
        return [[-0.5, r6(x0)], [0.5, r6(x0)], [0.5, r6(x1)], [-0.5, r6(x1)]]
    n0 = k.NOTCH[0]
    return [[-0.5, r6(x0)], [0.5, r6(x0)], [0.5, r6(x1)], [notch_half, r6(x1)], [notch_half, n0], [-notch_half, n0], [-notch_half, r6(x1)], [-0.5, r6(x1)]]


def glass(id, i, y0, y1):
    """🪟 One laminated pane between two heights."""
    return solid(id, "extrude", "glass", "glazing", 0.92, profile=glass_profile(i), plane="zx", **{"from": r6(y0), "to": r6(y1)})


def nuts(prefix, yc):
    """🔩 The four M16 nuts of an anchor plate, two rows of two (repeat across z)."""
    return [solid(f"{prefix}-{side}", "extrude", "steel", "detail", 0.25, profile=k.hexagon(yc + sy * k.BOLT_Y, -k.BOLT_Z), plane="yz", **{"from": 0.0, "to": k.NUT_H}, repeat={"count": 2, "translation": [0, 0, r6(2 * k.BOLT_Z)]}) for side, sy in (("top", 1), ("bottom", -1))]


def studs(prefix, yc, r):
    """🦯 The four anchor studs of a plate from the nut 0.45 m into the concrete (two rows, repeat across z)."""
    return [{**rev(f"{prefix}-{side}", "steel", "body", 0.35, [[0, 0], [0, r], [0.463, r], [0.463, 0]], [1, 0, 0], [-0.45, yc + sy * k.BOLT_Y, -k.BOLT_Z], repeat={"count": 2, "translation": [0, 0, r6(2 * k.BOLT_Z)]}), "member": BONE, "part": "joint"} for side, sy in (("top", 1), ("bottom", -1))]


def pin(id, p, tone=0.1, role="detail", r=k.PIN_R, head=k.PIN_HEAD_R, z0=k.PIN_Z0, length=k.PIN_LEN, heads=0.008):
    """📌 A pin across z with a head and a retaining collar."""
    prof = [[0, 0], [0, head], [heads, head], [heads, r], [length - heads, r], [length - heads, head], [length, head], [length, 0]] if heads else [[0, 0], [0, r], [length, r], [length, 0]]
    return rev(id, "steel", role, tone, prof, [0, 0, 1], [p[0], p[1], z0])


def tube(id, origin, axis, t0, t1, r, hollow=None, **rest):
    """🧪 A round tube along an axis in the bone plane, solid or hollow."""
    prof = [[t0, 0], [t0, r], [t1, r], [t1, 0]] if hollow is None else [[t0, r - hollow], [t0, r], [t1, r], [t1, r - hollow]]
    return rev(id, "steel", "body", 0.45, prof, [axis[0], axis[1], 0], [origin[0], origin[1], 0], **rest)


def scale50():
    """🔍 1:50: the canopy as built, every assembly at 7.5 mm."""
    body = [
        box("wall-core", "concrete", "body", 0.6, [0.3, 5.35, 1.0], [-0.35, 0.345, 0.0]),
        solid("wall-insulation", "extrude", "mineral-wool", "body", 0.9, profile=WALL_PROFILE, holes=WALL_HOLES, plane="yz", **{"from": -0.2, "to": -0.02}),
        solid("wall-render", "extrude", "render", "body", 0.55, profile=WALL_PROFILE, holes=WALL_HOLES, plane="yz", **{"from": -0.02, "to": 0.0}),
        rev("rebar-v-inner", "steel", "detail", 0.2, [[0, 0.006], [4.91, 0.006]], [0, 1, 0], [-0.45, -2.03, -0.45], repeat={"count": 7, "translation": [0, 0, 0.15]}),
        rev("rebar-h-inner", "steel", "detail", 0.2, [[0, 0.006], [0.9, 0.006]], [0, 0, 1], [-0.438, -2.03, -0.45], repeat={"count": 33, "translation": [0, 0.15, 0]}),
        rev("rebar-v-outer", "steel", "detail", 0.2, [[0, 0.006], [4.91, 0.006]], [0, 1, 0], [-0.25, -2.03, -0.45], repeat={"count": 7, "translation": [0, 0, 0.15]}),
        rev("rebar-h-outer", "steel", "detail", 0.2, [[0, 0.006], [0.9, 0.006]], [0, 0, 1], [-0.262, -2.03, -0.45], repeat={"count": 33, "translation": [0, 0.15, 0]}),
        box("pad-beam", "anchor-pad", "body", 0.7, [0.18, 0.24, 0.2], [-0.11, 0.0, 0.0]),
        box("plate-beam", "steel", "body", 0.3, k.PLATE, [-0.01, 0.0, 0.0]),
        *nuts("nut-beam", 0.0),
        box("pad-strut", "anchor-pad", "body", 0.7, [0.18, 0.24, 0.2], [-0.11, SY, 0.0]),
        box("plate-strut", "steel", "body", 0.3, k.PLATE, [-0.01, SY, 0.0]),
        *nuts("nut-strut", SY),
        solid("beam", "extrude", "steel", "body", 0.4, profile=IPE, plane="yz", **{"from": 0.0, "to": r6(k.BEAM_END - k.END_PLATE)}),
        box("beam-end-plate", "steel", "body", 0.4, [k.END_PLATE, 0.16, 0.082], [k.BEAM_END - k.END_PLATE / 2, 0.0, 0.0]),
        box("bearing-wall", "epdm", "detail", 0.15, [k.NOTCH[0] - 0.006 - k.GLASS_X0, k.BEARING, 0.06], [(k.GLASS_X0 + k.NOTCH[0] - 0.006) / 2, k.FLANGE_TOP + k.BEARING / 2, 0.0]),
        box("bearing-tip", "epdm", "detail", 0.15, [k.BEAM_END - 0.045 - 4.05, k.BEARING, 0.06], [(4.05 + k.BEAM_END - 0.045) / 2, k.FLANGE_TOP + k.BEARING / 2, 0.0]),
        *[glass(f"glass-{i}", i, k.GLASS_Y0, k.GLASS_Y1) for i in range(4)],
        box("joint-wall", "epdm", "detail", 0.15, [k.GLASS_X0, k.GLASS_T, 1.0], [k.GLASS_X0 / 2, k.GLASS_Y0 + k.GLASS_T / 2, 0.0]),
        solid("notch-seal", "extrude", "epdm", "detail", 0.15, profile=[[-k.NOTCH_HALF, k.NOTCH[0]], [k.NOTCH_HALF, k.NOTCH[0]], [k.NOTCH_HALF, r6(k.NOTCH[1])], [-k.NOTCH_HALF, r6(k.NOTCH[1])]], holes=[[[-k.GUSSET_HALF, r6(k.GUSSET_AT_GLASS[0])], [k.GUSSET_HALF, r6(k.GUSSET_AT_GLASS[0])], [k.GUSSET_HALF, r6(k.GUSSET_AT_GLASS[1])], [-k.GUSSET_HALF, r6(k.GUSSET_AT_GLASS[1])]]], plane="zx", **{"from": r6(k.GLASS_Y0), "to": r6(k.GLASS_Y1)}),
        *[box(f"joint-{i}", "epdm", "detail", 0.15, [k.JOINT, k.GLASS_T, 1.0], [x + k.JOINT / 2, k.GLASS_Y0 + k.GLASS_T / 2, 0.0]) for i, x in enumerate(k.JOINTS)],
        *[box(f"clamp-{i}", "steel", "detail", 0.25, [k.CLAMP_L, k.CLAMP_T, k.CLAMP_W], [x, k.GLASS_Y1 + k.CLAMP_T / 2, 0.0]) for i, x in enumerate(k.CLAMPS)],
        xy("gusset-beam", "steel", "body", 0.3, k.gusset_beam(), -k.GUSSET_HALF, k.GUSSET_HALF),
        xy("clevis-beam", "steel", "body", 0.3, k.cheek(k.PB, k.AXIS_B), k.CHEEK_IN, k.CHEEK_OUT, repeat={"count": 2, "translation": [0, 0, r6(-(k.CHEEK_IN + k.CHEEK_OUT))]}),
        xy("clevis-bridge-beam", "steel", "body", 0.3, k.bridge(k.PB, k.AXIS_B), -k.CHEEK_IN, k.CHEEK_IN),
        rev("clevis-body-beam", "steel", "body", 0.35, k.FORK_BODY, [k.AXIS_B[0], k.AXIS_B[1], 0], [k.PB[0], k.PB[1], 0]),
        pin("pin-beam", k.PB),
        xy("gusset-wall", "steel", "body", 0.3, k.gusset_wall(), -k.GUSSET_HALF, k.GUSSET_HALF),
        xy("clevis-wall", "steel", "body", 0.3, k.cheek(k.PW, k.AXIS_W), k.CHEEK_IN, k.CHEEK_OUT, repeat={"count": 2, "translation": [0, 0, r6(-(k.CHEEK_IN + k.CHEEK_OUT))]}),
        xy("clevis-bridge-wall", "steel", "body", 0.3, k.bridge(k.PW, k.AXIS_W), -k.CHEEK_IN, k.CHEEK_IN),
        rev("clevis-body-wall", "steel", "body", 0.35, k.FORK_BODY, [k.AXIS_W[0], k.AXIS_W[1], 0], [k.PW[0], k.PW[1], 0]),
        pin("pin-wall", k.PW),
        tube("strut-tube", k.PB, k.AXIS_B, k.TUBE_FROM, k.L - k.TUBE_FROM, k.TUBE_R),
    ]
    rod = ["gusset-beam", "clevis-beam", "clevis-bridge-beam", "clevis-body-beam", "pin-beam", "gusset-wall", "clevis-wall", "clevis-bridge-wall", "clevis-body-wall", "pin-wall", "strut-tube"]
    thermal = [
        ref("th-wall-concrete", "wall-core", region="wall-face", layer="wall-concrete"),
        ref("th-wall-wool", "wall-insulation", region="wall-face", layer="wall-wool"),
        ref("th-wall-render", "wall-render", region="wall-face", layer="wall-render", sectionFaces=["-u", "-v", "+v", "-w", "+w"]),
        {**box("th-concrete-beam", "concrete", "body", 0.6, [0.3, 0.24, 0.2], [-0.35, 0.0, 0.0]), "sectionFaces": ["-u", "+u", "-v", "+v", "-w", "+w"], "region": "wall-plate", "layer": "plate-concrete"},
        ref("th-pad-beam", "pad-beam", region="wall-plate", layer="plate-pad"),
        ref("th-plate-beam", "plate-beam", region="wall-plate", layer="plate-steel", sectionFaces=["-u"]),
        {**box("th-concrete-strut", "concrete", "body", 0.6, [0.3, 0.24, 0.2], [-0.35, SY, 0.0]), "sectionFaces": ["-u", "+u", "-v", "+v", "-w", "+w"], "region": "wall-plate", "layer": "plate-concrete"},
        ref("th-pad-strut", "pad-strut", region="wall-plate", layer="plate-pad"),
        ref("th-plate-strut", "plate-strut", region="wall-plate", layer="plate-steel", sectionFaces=["-u"]),
        ref("th-beam", "beam", region="canopy-steel"),
        ref("th-beam-end-plate", "beam-end-plate", region="canopy-steel"),
        *[ref(f"th-clamp-{i}", f"clamp-{i}", region="canopy-steel") for i in range(5)],
        *[ref(f"th-glass-{i}", f"glass-{i}", region="canopy-glass", sectionFaces=["-v"]) for i in range(4)],
        *[ref(f"th-{s}", s, region="rod-steel") for s in rod],
    ]
    structure = [
        ref("m-beam", "beam", member=BONE, part="member"),
        {**tube("m-strut", k.PB, k.AXIS_B, k.TUBE_FROM, k.L - k.TUBE_FROM, k.TUBE_R, hollow=k.TUBE_WALL), "tone": 0.5, "member": BONE, "part": "member"},
        *[ref(f"j-{s}", s, member=BONE, part="joint") for s in ["gusset-beam", "clevis-beam", "clevis-bridge-beam", "clevis-body-beam", "pin-beam", "gusset-wall", "clevis-wall", "clevis-bridge-wall", "clevis-body-wall", "pin-wall", "beam-end-plate", "plate-beam", "plate-strut"]],
        *studs("j-stud-beam", 0.0, 0.008),
        *studs("j-stud-strut", SY, 0.008),
        {**pin("support-wall-pin", (0.0, 0.0), tone=0.3, role="body", r=0.02, z0=-0.1, length=0.2, heads=0), "member": BONE, "part": "support", "anchor": "wall-pin"},
        {**pin("support-tie-rod", k.NODE, tone=0.3, role="body", r=0.02, z0=-0.06, length=0.12, heads=0), "member": BONE, "part": "support", "anchor": "tie-rod"},
        {**solid("mass-beam", "ellipsoid", "steel", "body", 0.5, radii=[0.035, 0.035, 0.035], transform={"translation": [2.75, 0.0, 0.0]}), "member": BONE, "part": "mass"},
    ]
    decisions = [
        text("Body: the tied canopy as built. Wall stub of concrete 300 mm with two hidden reinforcement mats (12 mm bars, 150 mm grid), mineral wool 180 mm and render 20 mm, both cut out round the two anchor elements (separation pad 180 x 240 x 200 mm with a 20 mm end plate 200 x 160 mm and four M16 nuts at 146 x 116 mm, edge distances 27 and 22 mm). The IPE 160 of its true section (root radius 9 mm) runs from its end plate at the wall to an 8 mm end plate at the tip.",
             "Körper: das Vordach wie gebaut. Wandstück aus Beton 300 mm mit zwei verdeckten Bewehrungsmatten (Stäbe 12 mm, Raster 150 mm), Mineralwolle 180 mm und Putz 20 mm, beide um die zwei Ankerelemente ausgespart (Trennlage 180 x 240 x 200 mm mit 20-mm-Stirnplatte 200 x 160 mm und vier Muttern M16 im Raster 146 x 116 mm, Randabstände 27 und 22 mm). Der IPE 160 in wahrem Querschnitt (Ausrundung 9 mm) läuft von seiner Stirnplatte an der Wand bis zu einer 8-mm-Kopfplatte an der Spitze."),
        text("Body: the glass roof is four laminated panes 2 x 8 mm (16.8 mm) of 1.35 m with 20 mm joints sealed by EPDM gaskets, laid on an 8 mm EPDM bearing strip on the top flange and closed against the render by a 30 mm EPDM wall joint, held by five 60 x 80 mm clamp plates at the wall, the three joints and the tip; the third pane is notched at its end round the gusset (25 mm slot, sealed with EPDM round the 15 mm plate). The strut is a CHS 60.3 tube with a welded end piece and a fork head (two 10 mm cheeks closed by a bridge behind the gusset lug, with a 36 mm shank) at each end, pinned with a 24 mm pin to a 15 mm gusset: on the top flange the pin sits on the system line 90 mm above the beam axis, clear of the glass, at the wall on the anchor plate.",
             "Körper: das Glasdach sind vier Verbundglasscheiben 2 x 8 mm (16,8 mm) zu 1,35 m mit 20-mm-Fugen, mit EPDM-Dichtungen geschlossen, auf einem 8-mm-EPDM-Auflagerband auf dem Obergurt, zum Putz mit einer 30-mm-EPDM-Wandfuge geschlossen, gehalten von fünf Klemmplatten 60 x 80 mm an der Wand, an den drei Fugen und an der Spitze; die dritte Scheibe ist an ihrem Ende um das Knotenblech ausgeklinkt (25-mm-Schlitz, um das 15-mm-Blech mit EPDM gedichtet). Der Stab ist ein Rohr 60,3 mit eingeschweißtem Endstück und Gabelkopf (zwei 10-mm-Laschen, hinter der Knotenblechlasche durch einen Steg geschlossen, mit 36-mm-Schaft) an jedem Ende, mit einem 24-mm-Bolzen an ein 15-mm-Knotenblech angeschlossen: auf dem Obergurt liegt der Bolzen auf der Systemlinie 90 mm über der Trägerachse, frei über dem Glas, an der Wand auf der Ankerplatte."),
        text("Thermal: the wall region is three nested layer slabs of their real thickness (concrete, wool, render; the render carries the cut edges as section faces); each anchor is stacked the same way (concrete column, pad, steel plate whose face bonded to the pad is a section face, C25 b). Beam, end plate and clamps are the canopy steel, the four panes the glass (their bedded underside a section face), strut tube, fork heads, gussets and pins the rod steel. The EPDM strip and joint gaskets belong to no region.",
             "Thermik: die Wandregion sind drei geschachtelte Schichtscheiben in wahrer Dicke (Beton, Wolle, Putz; der Putz trägt die Schnittkanten als Schnittflächen); jeder Anker ist ebenso geschichtet (Betonsäule, Trennlage, Stahlplatte, deren auf die Trennlage geklebte Fläche Schnittfläche ist, C25 b). Träger, Kopfplatte und Klemmen sind der Vordachstahl, die vier Scheiben das Glas (die aufgelagerte Unterseite Schnittfläche), Rohr, Gabelköpfe, Knotenbleche und Bolzen der Stabstahl. EPDM-Band und Fugendichtungen gehören zu keiner Region."),
        text("Structure: the IPE 160 is the beam member, the strut a hollow tube 60.3 x 3.2 mm between its fork heads; gussets, fork heads, pins, end plates and four M16 studs per anchor reaching 450 mm into the concrete are the joints; the wall pin is a hinge barrel on the beam axis at the wall face, the tie node a roller pin through the beam axis at x 4.0 where the strut system line meets it; the beam mass sits at its centre 2.75 m out.",
             "Tragwerk: der IPE 160 ist das Trägerglied, der Stab ein Hohlrohr 60,3 x 3,2 mm zwischen seinen Gabelköpfen; Knotenbleche, Gabelköpfe, Bolzen, Stirnplatten und je Anker vier Gewindebolzen M16, 450 mm in den Beton reichend, sind die Gelenke; das Wandgelenk ist ein Gelenkzylinder auf der Trägerachse an der Wandfläche, der Stabknoten ein Rollenbolzen durch die Trägerachse bei x 4,0, wo die Systemlinie des Stabs sie trifft; die Trägermasse liegt in ihrer Mitte 2,75 m vor der Wand."),
    ]
    return body, thermal, structure, decisions


def scale100():
    """🔎 1:100: the same canopy at 15 mm; small parts merged."""
    body = [
        box("wall-core", "concrete", "body", 0.6, [0.3, 5.35, 1.0], [-0.35, 0.345, 0.0]),
        solid("wall-insulation", "extrude", "mineral-wool", "body", 0.9, profile=WALL_PROFILE, holes=WALL_HOLES, plane="yz", **{"from": -0.2, "to": 0.0}),
        box("plate-beam", "anchor-pad", "body", 0.7, [0.2, 0.24, 0.2], [-0.1, 0.0, 0.0]),
        box("plate-strut", "anchor-pad", "body", 0.7, [0.2, 0.24, 0.2], [-0.1, SY, 0.0]),
        solid("beam", "extrude", "steel", "body", 0.4, profile=IPE_SIMPLE, plane="yz", **{"from": 0.0, "to": k.BEAM_END}),
        *[glass(f"glass-{i}", i, k.FLANGE_TOP, k.GLASS_Y1) for i in range(4)],
        box("joint-wall", "epdm", "detail", 0.15, [k.GLASS_X0, k.GLASS_Y1 - k.FLANGE_TOP, 1.0], [k.GLASS_X0 / 2, (k.FLANGE_TOP + k.GLASS_Y1) / 2, 0.0]),
        box("notch-seal", "epdm", "detail", 0.15, [k.NOTCH[1] - k.GUSSET_AT_GLASS[1], k.GLASS_Y1 - k.FLANGE_TOP, 2 * k.NOTCH_HALF], [(k.NOTCH[1] + k.GUSSET_AT_GLASS[1]) / 2, (k.FLANGE_TOP + k.GLASS_Y1) / 2, 0.0]),
        *[box(f"joint-{i}", "epdm", "detail", 0.15, [k.JOINT, k.GLASS_Y1 - k.FLANGE_TOP, 1.0], [x + k.JOINT / 2, (k.FLANGE_TOP + k.GLASS_Y1) / 2, 0.0]) for i, x in enumerate(k.JOINTS)],
        xy("gusset-beam", "steel", "body", 0.3, k.gusset_beam(), -k.GUSSET_HALF, k.GUSSET_HALF),
        xy("clevis-beam", "steel", "body", 0.3, k.cheek(k.PB, k.AXIS_B), -k.CHEEK_OUT, k.CHEEK_OUT),
        pin("pin-beam", k.PB, r=0.0125, heads=0),
        xy("gusset-wall", "steel", "body", 0.3, k.gusset_wall(), -k.GUSSET_HALF, k.GUSSET_HALF),
        xy("clevis-wall", "steel", "body", 0.3, k.cheek(k.PW, k.AXIS_W), -k.CHEEK_OUT, k.CHEEK_OUT),
        pin("pin-wall", k.PW, r=0.0125, heads=0),
        rev("strut-tube", "steel", "body", 0.45, [[0.095, 0], [0.095, k.SHANK_R], [0.115, k.SHANK_R], [0.145, k.TUBE_R], [k.L - 0.145, k.TUBE_R], [k.L - 0.115, k.SHANK_R], [k.L - 0.095, k.SHANK_R], [k.L - 0.095, 0]], [k.AXIS_B[0], k.AXIS_B[1], 0], [k.PB[0], k.PB[1], 0]),
    ]
    rod = ["gusset-beam", "clevis-beam", "pin-beam", "gusset-wall", "clevis-wall", "pin-wall", "strut-tube"]
    thermal = [
        ref("th-wall-core", "wall-core", region="wall-face", sectionFaces=["-u", "+u", "-v", "+v", "-w", "+w"]),
        ref("th-wall-insulation", "wall-insulation", region="wall-face", sectionFaces=["-u", "-v", "+v", "-w", "+w"]),
        ref("th-plate-beam", "plate-beam", region="wall-plate", sectionFaces=["-u", "-v", "+v", "-w", "+w"]),
        ref("th-plate-strut", "plate-strut", region="wall-plate", sectionFaces=["-u", "-v", "+v", "-w", "+w"]),
        ref("th-beam", "beam", region="canopy-steel"),
        *[ref(f"th-glass-{i}", f"glass-{i}", region="canopy-glass", sectionFaces=["-v"]) for i in range(4)],
        *[ref(f"th-{s}", s, region="rod-steel") for s in rod],
    ]
    structure = [
        ref("m-beam", "beam", member=BONE, part="member"),
        {**tube("m-strut", k.PB, k.AXIS_B, 0.145, k.L - 0.145, k.TUBE_R, hollow=k.TUBE_WALL), "tone": 0.5, "member": BONE, "part": "member"},
        *[ref(f"j-{s}", s, member=BONE, part="joint") for s in ["gusset-beam", "clevis-beam", "pin-beam", "gusset-wall", "clevis-wall", "pin-wall", "plate-beam", "plate-strut"]],
        {**rev("j-stud-beam", "steel", "body", 0.35, [[0, 0], [0, 0.008], [0.45, 0.008], [0.45, 0]], [1, 0, 0], [-0.45, 0.0, -k.BOLT_Z], repeat={"count": 2, "translation": [0, 0, r6(2 * k.BOLT_Z)]}), "member": BONE, "part": "joint"},
        {**rev("j-stud-strut", "steel", "body", 0.35, [[0, 0], [0, 0.008], [0.45, 0.008], [0.45, 0]], [1, 0, 0], [-0.45, SY, -k.BOLT_Z], repeat={"count": 2, "translation": [0, 0, r6(2 * k.BOLT_Z)]}), "member": BONE, "part": "joint"},
        {**pin("support-wall-pin", (0.0, 0.0), tone=0.3, role="body", r=0.02, z0=-0.1, length=0.2, heads=0), "member": BONE, "part": "support", "anchor": "wall-pin"},
        {**pin("support-tie-rod", k.NODE, tone=0.3, role="body", r=0.02, z0=-0.06, length=0.12, heads=0), "member": BONE, "part": "support", "anchor": "tie-rod"},
        {**solid("mass-beam", "ellipsoid", "steel", "body", 0.5, radii=[0.035, 0.035, 0.035], transform={"translation": [2.75, 0.0, 0.0]}), "member": BONE, "part": "mass"},
    ]
    decisions = [
        text("Body: the same canopy at 1:100. The wall is the concrete core and one 200 mm outer slab (wool and render merged, as the 2D draws them) with the two anchor blocks in it (pad and end plate merged, nuts below 15 mm left out). The IPE 160 keeps its true section with a chamfer for the root radius; its tip plate is merged into it. Each pane is one 25 mm slab of glass and its EPDM bearing (the 8 mm strip and the 8 mm clamps are below 15 mm); the three 20 mm joint gaskets and the 30 mm wall joint stay.",
             "Körper: dasselbe Vordach in 1:100. Die Wand ist der Betonkern und eine 200-mm-Außenschicht (Wolle und Putz zusammengefasst, wie die 2D sie zeichnet) mit den zwei Ankerblöcken darin (Trennlage und Stirnplatte zusammengefasst, Muttern unter 15 mm weggelassen). Der IPE 160 behält seinen wahren Querschnitt mit einer Fase für die Ausrundung; seine Kopfplatte ist eingeschlossen. Jede Scheibe ist eine 25-mm-Platte aus Glas und EPDM-Auflager (Band und Klemmen mit 8 mm liegen unter 15 mm); die drei 20-mm-Fugendichtungen und die 30-mm-Wandfuge bleiben."),
        text("Body: each fork head is one 37 mm block of both cheeks over the 15 mm gusset, its shank and end cones merged with the tube into one revolve from fork to fork; the pins stay as 25 mm bolts without heads.",
             "Körper: jeder Gabelkopf ist ein 37-mm-Block aus beiden Laschen über dem 15-mm-Knotenblech, Schaft und Endkegel sind mit dem Rohr zu einem Drehkörper von Gabel zu Gabel zusammengefasst; die Bolzen bleiben als 25-mm-Bolzen ohne Köpfe."),
        text("Thermal: references to the body; the concrete core carries all faces as section faces and the outer slab its cut edges; each anchor block exposes only its front face (C25 b: its other faces lie in the wall); the glass undersides are section faces. Wall and anchor layers are merged at this scale (B2).",
             "Thermik: Verweise auf den Körper; der Betonkern trägt alle Flächen als Schnittflächen, die Außenschicht ihre Schnittkanten; jeder Ankerblock zeigt nur seine Stirnfläche (C25 b: die übrigen liegen in der Wand); die Glasunterseiten sind Schnittflächen. Wand- und Ankerschichten sind in diesem Maßstab zusammengefasst (B2)."),
        text("Structure: beam as member (the IPE body), the strut as a hollow tube, gussets, fork heads, pins and the anchor blocks as joints with one pair of studs per row, supports as a hinge barrel at the wall and a roller pin at the tie node, the beam mass at its centre.",
             "Tragwerk: Träger als Glied (der IPE-Körper), der Stab als Hohlrohr, Knotenbleche, Gabelköpfe, Bolzen und Ankerblöcke als Gelenke mit einem Bolzenpaar je Reihe, Lager als Gelenkzylinder an der Wand und Rollenbolzen am Stabknoten, die Trägermasse in ihrer Mitte."),
    ]
    return body, thermal, structure, decisions


def scale200():
    """🗺️ 1:200: wall in two slabs, beam, one glass slab, strut with two node lugs."""
    base_b = [(3.715, k.FLANGE_TOP - 0.0025), (4.035, k.FLANGE_TOP - 0.0025)]
    base_w = [(-0.002, SY - 0.095), (-0.002, SY + 0.095)]
    n0, n1 = k.x_range_at(k.node_beam(), k.FLANGE_TOP, k.GLASS_Y1)
    slot = [[-0.02, r6(n0)], [-0.02, r6(n1)], [0.02, r6(n1)], [0.02, r6(n0)]]
    body = [
        box("wall-core", "concrete", "body", 0.6, [0.3, 5.35, 1.0], [-0.35, 0.345, 0.0]),
        box("wall-outer", "mineral-wool", "body", 0.8, [0.2, 5.35, 1.0], [-0.1, 0.345, 0.0]),
        solid("beam", "extrude", "steel", "body", 0.4, profile=[[0.08, 0.041], [0.0726, 0.041], [0.0726, 0.0025], [-0.0726, 0.0025], [-0.0726, 0.041], [-0.08, 0.041], [-0.08, -0.041], [-0.0726, -0.041], [-0.0726, -0.0025], [0.0726, -0.0025], [0.0726, -0.041], [0.08, -0.041]], plane="yz", **{"from": 0.0, "to": k.BEAM_END}),
        solid("glass-roof", "extrude", "glass", "glazing", 0.92, profile=[[-0.5, 0.0], [0.5, 0.0], [0.5, k.BEAM_END], [-0.5, k.BEAM_END]], holes=[slot], plane="zx", **{"from": k.FLANGE_TOP, "to": r6(k.GLASS_Y1)}),
        xy("node-beam", "steel", "body", 0.3, k.node_beam(), -0.02, 0.02),
        xy("node-wall", "steel", "body", 0.3, k.node_wall(), -0.02, 0.02),
        tube("strut-tube", k.PB, k.AXIS_B, 0.08, k.L - 0.08, k.TUBE_R, slender=True),
    ]
    thermal = [
        ref("th-wall-core", "wall-core", region="wall-face", sectionFaces=["-u", "+u", "-v", "+v", "-w", "+w"]),
        ref("th-wall-outer", "wall-outer", region="wall-face", sectionFaces=["-u", "-v", "+v", "-w", "+w"]),
        {**box("th-anchor-beam", "anchor-pad", "body", 0.7, [0.2, 0.24, 0.2], [-0.1, 0.0, 0.0]), "region": "wall-plate", "sectionFaces": ["-u", "-v", "+v", "-w", "+w"]},
        {**box("th-anchor-strut", "anchor-pad", "body", 0.7, [0.2, 0.24, 0.2], [-0.1, SY, 0.0]), "region": "wall-plate", "sectionFaces": ["-u", "-v", "+v", "-w", "+w"]},
        ref("th-beam", "beam", region="canopy-steel"),
        ref("th-glass-roof", "glass-roof", region="canopy-glass", sectionFaces=["-v"]),
        ref("th-strut-tube", "strut-tube", region="rod-steel"),
        ref("th-node-beam", "node-beam", region="rod-steel"),
        ref("th-node-wall", "node-wall", region="rod-steel"),
    ]
    structure = [
        ref("m-beam", "beam", member=BONE, part="member"),
        {**tube("m-strut", k.PB, k.AXIS_B, 0.08, k.L - 0.08, k.TUBE_R, hollow=k.TUBE_WALL, slender=True), "tone": 0.5, "member": BONE, "part": "member"},
        ref("j-node-beam", "node-beam", member=BONE, part="joint"),
        ref("j-node-wall", "node-wall", member=BONE, part="joint"),
        {**box("j-anchor-beam", "steel", "body", 0.35, [0.2, 0.2, 0.16], [-0.1, 0.0, 0.0]), "member": BONE, "part": "joint"},
        {**box("j-anchor-strut", "steel", "body", 0.35, [0.2, 0.2, 0.16], [-0.1, SY, 0.0]), "member": BONE, "part": "joint"},
        {**pin("support-wall-pin", (0.0, 0.0), tone=0.3, role="body", r=0.03, z0=-0.1, length=0.2, heads=0), "member": BONE, "part": "support", "anchor": "wall-pin"},
        {**pin("support-tie-rod", k.NODE, tone=0.3, role="body", r=0.03, z0=-0.06, length=0.12, heads=0), "member": BONE, "part": "support", "anchor": "tie-rod"},
        {**solid("mass-beam", "ellipsoid", "steel", "body", 0.5, radii=[0.04, 0.04, 0.04], transform={"translation": [2.75, 0.0, 0.0]}), "member": BONE, "part": "mass"},
    ]
    decisions = [
        text("Body: the wall is the concrete core and one 200 mm outer slab (the anchor elements lie inside it, not drawn at this scale); the IPE 160 keeps its true outline (82 x 160 mm, web and flanges without the root radius); the roof is one 25 mm slab of glass, bearing and gaskets, 5.5 m long from the wall (the 30 mm wall joint merged), with the slot the node lug passes through; each strut node is one 40 mm lug (gusset and fork head merged) and the strut between them is the true 60.3 mm tube (slender, the 2D draws it as a 0.3 mm line).",
             "Körper: die Wand ist der Betonkern und eine 200-mm-Außenschicht (die Ankerelemente liegen darin und sind in diesem Maßstab nicht gezeichnet); der IPE 160 behält seinen wahren Umriss (82 x 160 mm, Steg und Gurte ohne Ausrundung); das Dach ist eine 25-mm-Platte aus Glas, Auflager und Dichtungen, 5,5 m lang ab der Wand (die 30-mm-Wandfuge eingeschlossen), mit dem Schlitz, durch den die Knotenlasche geht; jeder Stabknoten ist eine 40-mm-Lasche (Knotenblech und Gabelkopf zusammengefasst), der Stab dazwischen das wahre Rohr 60,3 mm (schlank, die 2D zeichnet ihn als 0,3-mm-Linie)."),
        text("Thermal: references to the body with section faces on the concrete core, the cut edges of the outer slab and the glass underside; the anchor region is two 200 x 240 x 200 mm blocks inside the outer slab that expose only their front face; the node lugs belong to the rod steel.",
             "Thermik: Verweise auf den Körper mit Schnittflächen am Betonkern, an den Schnittkanten der Außenschicht und an der Glasunterseite; die Ankerregion sind zwei Blöcke 200 x 240 x 200 mm in der Außenschicht, die nur ihre Stirnfläche zeigen; die Knotenlaschen gehören zum Stabstahl."),
        text("Structure: beam and hollow strut as members, the two node lugs as the pinned joints at both strut ends, the anchor plates with their studs as one block each inside the wall, supports as 60 mm hinge and roller pins, the beam mass at its centre.",
             "Tragwerk: Träger und Hohlstab als Glieder, die zwei Knotenlaschen als gelenkige Anschlüsse an beiden Stabenden, die Ankerplatten mit ihren Bolzen als je ein Block in der Wand, Lager als 60-mm-Gelenk- und Rollenbolzen, die Trägermasse in ihrer Mitte."),
    ]
    return body, thermal, structure, decisions


def scale500():
    """🛰️ 1:500: wall block, beam, roof slab, strut."""
    top = (k.NODE[0] - k.GLASS_Y1 / k.DIR[1] * -k.DIR[0], k.GLASS_Y1)
    length = ((top[0] - k.WALL_NODE[0]) ** 2 + (top[1] - k.WALL_NODE[1]) ** 2) ** 0.5
    body = [
        box("wall", "concrete", "body", 0.7, [0.5, 5.35, 1.0], [-0.25, 0.345, 0.0]),
        solid("beam", "extrude", "steel", "body", 0.4, profile=[[0.08, 0.041], [-0.08, 0.041], [-0.08, -0.041], [0.08, -0.041]], plane="yz", **{"from": 0.0, "to": k.BEAM_END}),
        solid("roof", "extrude", "glass", "glazing", 0.92, profile=[[-0.5, 0.0], [0.5, 0.0], [0.5, k.BEAM_END], [-0.5, k.BEAM_END]], plane="zx", **{"from": k.FLANGE_TOP, "to": r6(k.GLASS_Y1)}),
        tube("strut-tube", k.WALL_NODE, k.AXIS_W, 0.0, length, k.TUBE_R, slender=True),
    ]
    thermal = [
        ref("th-wall", "wall", region="wall-face", sectionFaces=["-u", "-v", "+v", "-w", "+w"]),
        ref("th-wall-anchors", "wall", region="wall-plate", sectionFaces=["-u", "+u", "-v", "+v", "-w", "+w"]),
        ref("th-beam", "beam", region="canopy-steel"),
        ref("th-roof", "roof", region="canopy-glass", sectionFaces=["-v"]),
        ref("th-strut-tube", "strut-tube", region="rod-steel"),
    ]
    structure = [
        ref("m-beam", "beam", member=BONE, part="member"),
        {**tube("m-strut", k.WALL_NODE, k.AXIS_W, 0.0, length, k.TUBE_R, hollow=k.TUBE_WALL, slender=True), "tone": 0.5, "member": BONE, "part": "member"},
        {**box("j-node-beam", "steel", "body", 0.35, [0.075, 0.075, 0.075], [k.PB[0], k.PB[1], 0.0]), "member": BONE, "part": "joint"},
        {**box("j-node-wall", "steel", "body", 0.35, [0.075, 0.075, 0.075], [0.0375, k.WALL_NODE[1], 0.0]), "member": BONE, "part": "joint"},
        {**box("support-wall-pin", "steel", "body", 0.3, [0.075, 0.075, 0.1], [0.0, 0.0, 0.0]), "member": BONE, "part": "support", "anchor": "wall-pin"},
        {**box("support-tie-rod", "steel", "body", 0.3, [0.075, 0.075, 0.1], [k.NODE[0], 0.0, 0.0]), "member": BONE, "part": "support", "anchor": "tie-rod"},
        {**box("mass-beam", "steel", "body", 0.5, [0.075, 0.075, 0.075], [2.75, 0.0, 0.0]), "member": BONE, "part": "mass"},
    ]
    decisions = [
        text("Body: four masses. The 500 mm wall block (concrete and insulation merged), the IPE 160 as its 82 x 160 mm envelope (flanges and web below 75 mm), the 25 mm roof slab of glass, bearing and gaskets from the wall to the beam tip, and the strut as the true 60.3 mm tube (slender, the 2D draws a line) from its wall anchor to the roof where its fork node stands; nodes, clamps and anchors are below the scale.",
             "Körper: vier Massen. Der 500-mm-Wandblock (Beton und Dämmung zusammengefasst), der IPE 160 als seine Hülle 82 x 160 mm (Gurte und Steg unter 75 mm), die 25-mm-Dachplatte aus Glas, Auflager und Dichtungen von der Wand bis zur Trägerspitze und der Stab als wahres Rohr 60,3 mm (schlank, die 2D zeichnet eine Linie) vom Wandanker bis zum Dach, wo sein Gabelknoten steht; Knoten, Klemmen und Anker liegen unter dem Maßstab."),
        text("Thermal: references to the body; the anchor region references the wall block with all faces as section faces (the anchors lie inside it and are not drawn at this scale; the region only stays covered).",
             "Thermik: Verweise auf den Körper; die Ankerregion verweist auf den Wandblock mit allen Flächen als Schnittflächen (die Anker liegen darin und sind in diesem Maßstab nicht gezeichnet; die Region bleibt nur abgedeckt)."),
        text("Structure: beam and strut as members, 75 mm joint blocks at the beam pin and at the wall anchor of the strut, supports as 75 mm blocks (hinge and roller), the beam mass as a 75 mm block at its centre.",
             "Tragwerk: Träger und Stab als Glieder, 75-mm-Gelenkblöcke am Trägerbolzen und am Wandanker des Stabs, Lager als 75-mm-Blöcke (Gelenk und Rolle), die Trägermasse als 75-mm-Block in ihrer Mitte."),
    ]
    return body, thermal, structure, decisions


def main():
    """💾 Writes the source file."""
    volumes = {}
    for scale, build in (("50", scale50), ("100", scale100), ("200", scale200), ("500", scale500)):
        body, thermal, structure, decisions = build()
        volumes[scale] = {"status": "authored", "denominator": int(scale), "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}
    path = os.path.join(HERE, "volumes", "components", "tied-canopy.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, ensure_ascii=False, indent=2)
        f.write("\n")
    print(path, {s: [len(v[n]) for n in ("body", "thermal", "structure")] for s, v in volumes.items()})


if __name__ == "__main__":
    main()
