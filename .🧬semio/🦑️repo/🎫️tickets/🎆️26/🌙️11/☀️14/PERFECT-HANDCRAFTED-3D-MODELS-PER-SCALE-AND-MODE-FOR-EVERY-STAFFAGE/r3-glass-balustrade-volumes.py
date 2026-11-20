"""🪟 Round 3 glass balustrade: writes the handcrafted `volumes` (body, thermal, structure, decisions at 1:50, 1:100, 1:200, 1:500)
to `volumes/components/glass-balustrade.json`.

Every solid is stated here by hand from the dimensions of `r3-glass-balustrade-kit.py`, which the 2D root script shares.

See: r3-components-glass-balustrade.md, round3-brief.md, design.md
"""

import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-glass-balustrade-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)
r6 = k.r6
L = k.HALF_LENGTH
GL = k.GLASS_HALF_LENGTH
STEEL, PLATE, FIXING, RUBBER, GLASS, FILM, RAIL = 0.38, 0.32, 0.22, 0.12, 0.92, 0.78, 0.45


def solid(id, kind, material, tone, role="body", **rest):
    """🧱 A solid of the bone `panel`."""
    return {"id": id, "kind": kind, "bone": "panel", "material": material, "role": role, "tone": tone, **rest}


def box(id, material, tone, size, centre, **rest):
    """📦 A box by size and centre (u, v, w)."""
    return solid(id, "box", material, tone, size=[r6(v) for v in size], transform={"translation": [r6(v) for v in centre]}, **rest)


def section(id, material, tone, profile, w0, w1, **rest):
    """📐 A cross-section (u, v) pushed along the panel from w0 to w1."""
    return solid(id, "extrude", material, tone, profile=profile, plane="xy", **{"from": r6(w0), "to": r6(w1)}, **rest)


def sheet(id, material, tone, w_half, u0, u1, v0, v1, holes=None, **rest):
    """🪟 A sheet in the panel plane (w, u) between v0 and v1, with round holes where given."""
    extra = {"holes": holes} if holes else {}
    return solid(id, "extrude", material, tone, profile=k.rect(-w_half, u0, w_half, u1), plane="zx", **extra, **{"from": r6(v0), "to": r6(v1)}, **rest)


def across(id, material, tone, profile, v0, v1, **rest):
    """⬡ A profile in the panel plane (w, u) pushed across the panel from v0 to v1."""
    return solid(id, "extrude", material, tone, profile=profile, plane="zx", **{"from": r6(v0), "to": r6(v1)}, **rest)


def on_plate(id, material, tone, profile, u0, u1, **rest):
    """🔩 A profile in the floor plane (v, w) pushed upwards from u0 to u1."""
    return solid(id, "extrude", material, tone, profile=profile, plane="yz", **{"from": r6(u0), "to": r6(u1)}, **rest)


def rod(id, material, tone, radius, start, axis, length, **rest):
    """🥢 A round rod from `start` along `axis`."""
    return solid(id, "revolve", material, tone, profile=[[0.0, r6(radius)], [r6(length), r6(radius)]], axis=axis, origin=[r6(v) for v in start], **rest)


def along(count, step):
    """🔁 Copies along the panel."""
    return {"count": count, "translation": [0, 0, step]}


def ref(id, of, **rest):
    """🔗 A reference to a body solid."""
    return {"id": id, "of": of, **rest}


def text(en, de):
    """🗣️ A bilingual sentence."""
    return {"en": en, "de": de}


def bolt_holes(radius, n=24):
    """🕳️ The seven bolt holes of the clamp line."""
    return [k.circle(w, k.BOLT_U, radius, n) for w in k.BOLT_W]


def body50():
    """🔧 1:50: every part of the clamped balustrade."""
    w0 = k.BOLT_W[0]
    a0 = k.ANCHOR_W[0]
    top = k.PLATE_T
    return [
        box("base-plate", "steel", PLATE, [k.PLATE_T, 2 * k.PLATE_HALF, 2 * L], [k.PLATE_T / 2, 0, 0]),
        sheet("cheek-fixed", "steel", STEEL, L, top, k.SHOE_TOP, -k.CLAMP_HALF, -k.SLOT_HALF, bolt_holes(k.BOLT_HOLE_R)),
        sheet("cheek-clamp", "steel", STEEL, L, top, k.SHOE_TOP, k.SLOT_HALF, k.CLAMP_HALF, bolt_holes(k.BOLT_HOLE_R)),
        section("weld-outer", "steel", PLATE, [[top, -k.CLAMP_HALF], [top, r6(-k.CLAMP_HALF - k.WELD)], [r6(top + k.WELD), -k.CLAMP_HALF]], -L, L, role="detail"),
        section("weld-inner", "steel", PLATE, [[top, -k.SLOT_HALF], [r6(top + k.WELD), -k.SLOT_HALF], [top, r6(-k.SLOT_HALF + k.WELD)]], -L, L, role="detail"),
        sheet("pad-fixed", "elastomer", RUBBER, GL, k.GLASS_BOTTOM, k.SHOE_TOP, -k.SLOT_HALF, -k.GLASS_HALF, bolt_holes(k.BOLT_HOLE_R)),
        sheet("pad-clamp", "elastomer", RUBBER, GL, k.GLASS_BOTTOM, k.SHOE_TOP, k.GLASS_HALF, k.SLOT_HALF, bolt_holes(k.BOLT_HOLE_R)),
        box("setting-block", "elastomer", RUBBER, [k.GLASS_BOTTOM - top, 2 * k.GLASS_HALF, k.BLOCK_LENGTH], [(k.GLASS_BOTTOM + top) / 2, 0, k.BLOCK_W[0]], role="detail", repeat=along(2, k.BLOCK_W[1] - k.BLOCK_W[0])),
        sheet("pane-fixed", "glass", GLASS, GL, k.GLASS_BOTTOM, k.GLASS_TOP, -k.GLASS_HALF, -k.PVB_HALF, bolt_holes(k.SLEEVE_R), role="glazing"),
        sheet("interlayer", "pvb", FILM, GL, k.GLASS_BOTTOM, k.GLASS_TOP, -k.PVB_HALF, k.PVB_HALF, bolt_holes(k.SLEEVE_R), role="glazing"),
        sheet("pane-clamp", "glass", GLASS, GL, k.GLASS_BOTTOM, k.GLASS_TOP, k.PVB_HALF, k.GLASS_HALF, bolt_holes(k.SLEEVE_R), role="glazing"),
        solid(
            "bolt-sleeve",
            "revolve",
            "elastomer",
            RUBBER,
            role="detail",
            profile=[[-k.GLASS_HALF, k.BOLT_HOLE_R], [-k.GLASS_HALF, k.SLEEVE_R], [k.GLASS_HALF, k.SLEEVE_R], [k.GLASS_HALF, k.BOLT_HOLE_R]],
            axis=[0, 1, 0],
            origin=[k.BOLT_U, 0, w0],
            repeat=along(7, 0.3),
        ),
        rod("bolt-shank", "steel", FIXING, k.BOLT_R, [k.BOLT_U, -k.CLAMP_HALF, w0], [0, 1, 0], 2 * k.CLAMP_HALF + k.NUT_M + k.STUB, role="detail", repeat=along(7, 0.3)),
        across("bolt-head", "steel", FIXING, k.hexagon(w0, k.BOLT_U), -k.CLAMP_HALF - k.HEAD_K, -k.CLAMP_HALF, role="detail", repeat=along(7, 0.3)),
        solid(
            "bolt-nut",
            "extrude",
            "steel",
            FIXING,
            role="detail",
            profile=k.hexagon(w0, k.BOLT_U),
            holes=[k.circle(w0, k.BOLT_U, k.BOLT_R, 16)],
            plane="zx",
            **{"from": r6(k.CLAMP_HALF), "to": r6(k.CLAMP_HALF + k.NUT_M)},
            repeat=along(7, 0.3),
        ),
        solid(
            "anchor-nut-fixed",
            "extrude",
            "steel",
            FIXING,
            role="detail",
            profile=k.hexagon(-k.ANCHOR_V, a0),
            holes=[k.circle(-k.ANCHOR_V, a0, k.BOLT_R, 16)],
            plane="yz",
            **{"from": r6(top), "to": r6(top + k.NUT_M)},
            repeat=along(5, 0.4),
        ),
        solid(
            "anchor-nut-clamp",
            "extrude",
            "steel",
            FIXING,
            role="detail",
            profile=k.hexagon(k.ANCHOR_V, a0),
            holes=[k.circle(k.ANCHOR_V, a0, k.BOLT_R, 16)],
            plane="yz",
            **{"from": r6(top), "to": r6(top + k.NUT_M)},
            repeat=along(5, 0.4),
        ),
        rod("anchor-stud-fixed", "steel", FIXING, k.BOLT_R, [top, -k.ANCHOR_V, a0], [1, 0, 0], k.ANCHOR_STUD, role="detail", repeat=along(5, 0.4)),
        rod("anchor-stud-clamp", "steel", FIXING, k.BOLT_R, [top, k.ANCHOR_V, a0], [1, 0, 0], k.ANCHOR_STUD, role="detail", repeat=along(5, 0.4)),
        section("handrail-tube", "steel", RAIL, k.slotted_tube(k.RAIL_SLOT_HALF, 40), -k.RAIL_HALF_LENGTH, k.RAIL_HALF_LENGTH),
        section(
            "handrail-gasket",
            "elastomer",
            RUBBER,
            [
                [k.GASKET_BOTTOM, -k.RAIL_SLOT_HALF],
                [k.GASKET_TOP, -k.RAIL_SLOT_HALF],
                [k.GASKET_TOP, k.RAIL_SLOT_HALF],
                [k.GASKET_BOTTOM, k.RAIL_SLOT_HALF],
                [k.GASKET_BOTTOM, k.GLASS_HALF],
                [k.GLASS_TOP, k.GLASS_HALF],
                [k.GLASS_TOP, -k.GLASS_HALF],
                [k.GASKET_BOTTOM, -k.GLASS_HALF],
            ],
            -k.RAIL_HALF_LENGTH,
            k.RAIL_HALF_LENGTH,
            role="detail",
        ),
        cap("handrail-cap-left", 1),
        cap("handrail-cap-right", -1),
    ]


def cap(id, side):
    """🧢 A dished end cap of the handrail tube, 3 mm rim and 5 mm crown, on the +w (1) or -w (-1) end."""
    return solid(
        id,
        "revolve",
        "steel",
        RAIL,
        role="detail",
        profile=[[0.0, k.RAIL_R], [0.003, k.RAIL_R], [0.0055, 0.0185], [0.0072, 0.0125], [k.CAP, 0.005]],
        axis=[0, 0, side],
        origin=[k.RAIL_U, 0, side * k.RAIL_HALF_LENGTH],
    )


def mass(radius, proud):
    """⚖️ The mass of the glass strip as a disc at its centre of mass (0.55 m), standing `proud` of both glass faces so it shows."""
    return solid(
        "mass-panel",
        "revolve",
        "glass",
        0.5,
        profile=[[r6(-k.GLASS_HALF - proud), radius], [r6(k.GLASS_HALF + proud), radius]],
        axis=[0, 1, 0],
        origin=[0.55, 0, 0],
        member="panel",
        part="mass",
    )


def thermal50():
    """🌡️ 1:50: every body surface in its region."""
    clamp = ["base-plate", "cheek-fixed", "cheek-clamp", "weld-outer", "weld-inner", "pad-fixed", "pad-clamp", "setting-block", "bolt-sleeve", "bolt-shank", "bolt-head", "bolt-nut", "anchor-nut-fixed", "anchor-nut-clamp", "anchor-stud-fixed", "anchor-stud-clamp"]
    glass = ["pane-fixed", "interlayer", "pane-clamp"]
    rail = ["handrail-cap-left", "handrail-cap-right"]
    skin = rod("th-handrail-skin", "steel", RAIL, k.RAIL_R, [k.RAIL_U, 0, -k.RAIL_HALF_LENGTH], [0, 0, 1], 2 * k.RAIL_HALF_LENGTH, region="handrail-steel")
    return [ref(f"th-{s}", s, region="clamp-steel") for s in clamp] + [ref(f"th-{s}", s, region="panel-glass") for s in glass] + [skin] + [ref(f"th-{s}", s, region="handrail-steel") for s in rail]


def structure50():
    """🏗️ 1:50: the load path from the handrail through the two panes into the bolted clamp and the anchored base plate."""
    return [
        ref("member-pane-fixed", "pane-fixed", member="panel", part="member"),
        ref("member-pane-clamp", "pane-clamp", member="panel", part="member"),
        ref("member-handrail", "handrail-tube", member="panel", part="member"),
        ref("joint-pad-fixed", "pad-fixed", member="panel", part="joint"),
        ref("joint-pad-clamp", "pad-clamp", member="panel", part="joint"),
        ref("joint-setting-block", "setting-block", member="panel", part="joint"),
        ref("joint-cheek-fixed", "cheek-fixed", member="panel", part="joint"),
        ref("joint-cheek-clamp", "cheek-clamp", member="panel", part="joint"),
        ref("joint-weld-outer", "weld-outer", member="panel", part="joint"),
        ref("joint-weld-inner", "weld-inner", member="panel", part="joint"),
        ref("joint-bolt", "bolt-shank", member="panel", part="joint"),
        ref("joint-bolt-head", "bolt-head", member="panel", part="joint"),
        ref("joint-bolt-nut", "bolt-nut", member="panel", part="joint"),
        ref("joint-anchor-fixed", "anchor-stud-fixed", member="panel", part="joint"),
        ref("joint-anchor-clamp", "anchor-stud-clamp", member="panel", part="joint"),
        ref("joint-anchor-nut-fixed", "anchor-nut-fixed", member="panel", part="joint"),
        ref("joint-anchor-nut-clamp", "anchor-nut-clamp", member="panel", part="joint"),
        ref("support-base-plate", "base-plate", member="panel", part="support", anchor="base-clamp"),
        mass(0.06, 0.005),
    ]


def body100():
    """🔩 1:100: plate, two clamp bars (plate and strip merged), the laminate as one body, plain bolts and studs, the slotted tube."""
    top = k.PLATE_T
    r = 0.01
    return [
        box("base-plate", "steel", PLATE, [k.PLATE_T, 2 * k.PLATE_HALF, 2 * L], [k.PLATE_T / 2, 0, 0]),
        sheet("clamp-bar-fixed", "steel", STEEL, L, top, k.SHOE_TOP, -k.CLAMP_HALF, -k.GLASS_HALF, bolt_holes(r, 12)),
        sheet("clamp-bar-clamp", "steel", STEEL, L, top, k.SHOE_TOP, k.GLASS_HALF, k.CLAMP_HALF, bolt_holes(r, 12)),
        sheet("glass", "glass", GLASS, GL, top, k.GLASS_TOP, -k.GLASS_HALF, k.GLASS_HALF, bolt_holes(r, 12), role="glazing"),
        rod("clamp-bolt", "steel", FIXING, r, [k.BOLT_U, -k.CLAMP_HALF - 0.01, k.BOLT_W[0]], [0, 1, 0], 2 * k.CLAMP_HALF + 0.02, role="detail", repeat=along(7, 0.3)),
        rod("anchor-fixed", "steel", FIXING, r, [top, -k.ANCHOR_V, k.ANCHOR_W[0]], [1, 0, 0], k.ANCHOR_STUD, role="detail", repeat=along(5, 0.4)),
        rod("anchor-clamp", "steel", FIXING, r, [top, k.ANCHOR_V, k.ANCHOR_W[0]], [1, 0, 0], k.ANCHOR_STUD, role="detail", repeat=along(5, 0.4)),
        section("handrail-tube", "steel", RAIL, k.slotted_tube(k.GLASS_HALF, 20), -0.985, 0.985),
        rod("handrail-plug-left", "steel", RAIL, k.RAIL_R, [k.RAIL_U, 0, 0.985], [0, 0, 1], 0.015, role="detail"),
        rod("handrail-plug-right", "steel", RAIL, k.RAIL_R, [k.RAIL_U, 0, -0.985], [0, 0, -1], 0.015, role="detail"),
    ]


def thermal100():
    """🌡️ 1:100: the merged body surfaces in their regions."""
    clamp = ["base-plate", "clamp-bar-fixed", "clamp-bar-clamp", "clamp-bolt", "anchor-fixed", "anchor-clamp"]
    skin = rod("th-handrail-skin", "steel", RAIL, k.RAIL_R, [k.RAIL_U, 0, -L], [0, 0, 1], 2 * L, region="handrail-steel")
    return [ref(f"th-{s}", s, region="clamp-steel") for s in clamp] + [ref("th-glass", "glass", region="panel-glass"), skin]


def structure100():
    """🏗️ 1:100: laminate and handrail as members, the bolted clamp bars as the fixed joint, the plate as the support."""
    return [
        ref("member-glass", "glass", member="panel", part="member"),
        ref("member-handrail", "handrail-tube", member="panel", part="member"),
        ref("joint-clamp-fixed", "clamp-bar-fixed", member="panel", part="joint"),
        ref("joint-clamp-clamp", "clamp-bar-clamp", member="panel", part="joint"),
        ref("joint-bolt", "clamp-bolt", member="panel", part="joint"),
        ref("joint-anchor-fixed", "anchor-fixed", member="panel", part="joint"),
        ref("joint-anchor-clamp", "anchor-clamp", member="panel", part="joint"),
        ref("support-base-plate", "base-plate", member="panel", part="support", anchor="base-clamp"),
        mass(0.075, 0.01),
    ]


FLANGE = 0.03


def body200():
    """🧱 1:200: the shoe as one inverted T, the glass sheet, the handrail as a round with the glass notch."""
    c = k.CLAMP_HALF
    shoe = [[0, -k.PLATE_HALF], [0, k.PLATE_HALF], [FLANGE, k.PLATE_HALF], [FLANGE, c], [k.SHOE_TOP, c], [k.SHOE_TOP, -c], [FLANGE, -c], [FLANGE, -k.PLATE_HALF]]
    return [
        section("shoe", "steel", STEEL, shoe, -L, L),
        box("glass", "glass", GLASS, [k.RAIL_U - k.SHOE_TOP, 2 * k.GLASS_HALF, 2 * L], [(k.RAIL_U + k.SHOE_TOP) / 2, 0, 0], role="glazing"),
        section("handrail", "steel", RAIL, k.notched_disc(k.GLASS_HALF, 14), -L, L),
    ]


def structure200():
    """🏗️ 1:200: glass and handrail members, the clamp upstand as the fixed joint, the flange as the support."""
    c = k.CLAMP_HALF
    return [
        ref("member-glass", "glass", member="panel", part="member"),
        ref("member-handrail", "handrail", member="panel", part="member"),
        box("joint-clamp", "steel", STEEL, [k.SHOE_TOP - FLANGE, 2 * c, 2 * L], [(k.SHOE_TOP + FLANGE) / 2, 0, 0], member="panel", part="joint"),
        box("support-base-plate", "steel", PLATE, [FLANGE, 2 * k.PLATE_HALF, 2 * L], [FLANGE / 2, 0, 0], member="panel", part="support", anchor="base-clamp"),
        mass(0.09, 0.02),
    ]


RAIL_BAR = 0.0424


def body500():
    """▫️ 1:500: the foot as a coarse inverted T, the glass sheet, the handrail as a square bar."""
    c, f = 0.0375, 0.0375
    foot = [[0, -k.PLATE_HALF], [0, k.PLATE_HALF], [f, k.PLATE_HALF], [f, c], [k.SHOE_TOP, c], [k.SHOE_TOP, -c], [f, -c], [f, -k.PLATE_HALF]]
    rail_bottom = 1.1 - RAIL_BAR
    return [
        section("foot", "steel", STEEL, foot, -L, L),
        box("glass", "glass", GLASS, [rail_bottom - k.SHOE_TOP, 2 * k.GLASS_HALF, 2 * L], [(rail_bottom + k.SHOE_TOP) / 2, 0, 0], role="glazing"),
        box("handrail", "steel", RAIL, [RAIL_BAR, RAIL_BAR, 2 * L], [1.1 - RAIL_BAR / 2, 0, 0]),
    ]


def structure500():
    """🏗️ 1:500: glass and handrail members on the foot as the fixed support."""
    return [
        ref("member-glass", "glass", member="panel", part="member"),
        ref("member-handrail", "handrail", member="panel", part="member"),
        ref("support-foot", "foot", member="panel", part="support", anchor="base-clamp"),
    ]


def scale(n, body, thermal, structure, decisions):
    """🗂️ One authored scale."""
    return {"status": "authored", "denominator": n, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def build():
    """🧮 All four scales."""
    three = lambda shoe: [ref(f"th-{shoe}", shoe, region="clamp-steel"), ref("th-glass", "glass", region="panel-glass"), ref("th-handrail", "handrail", region="handrail-steel")]
    v50 = scale(
        50,
        body50(),
        thermal50(),
        structure50(),
        [
            text(
                "Body: the clamped balustrade of DIN 18008-4 category B as it is built: a 300 x 15 mm steel base plate, the fixed clamp plate 15 x 110 mm welded to it with two 6 mm fillet welds, the loose clamp plate opposite, two 10 mm elastomer strips over the 100 mm clamping height, the laminate as two 10 mm panes with the 1.5 mm PVB film between them standing on two 100 mm setting blocks, seven M12 through-bolts at 300 mm (hexagon head on the fixed plate, nut on the loose plate) in 26 mm glass holes with plastic sleeves, ten M12 anchor studs with nuts at 400 mm on both sides of the clamp, and the slotted 42.4 x 2 mm tube handrail with its U gasket over the glass edge and two dished end caps.",
                "Körper: die eingespannte Brüstung nach DIN 18008-4 Kategorie B, wie sie gebaut wird: eine Stahlfußplatte 300 x 15 mm, das feste Klemmblech 15 x 110 mm mit zwei Kehlnähten a6 daran geschweißt, gegenüber das lose Klemmblech, zwei Elastomerstreifen von 10 mm über die Einspannhöhe von 100 mm, das Verbundglas als zwei 10-mm-Scheiben mit der 1,5-mm-PVB-Folie dazwischen auf zwei Klotzungen von 100 mm, sieben Durchgangsschrauben M12 im Abstand von 300 mm (Sechskantkopf am festen Blech, Mutter am losen Blech) in Glasbohrungen von 26 mm mit Kunststoffhülsen, zehn Ankerbolzen M12 mit Muttern im Abstand von 400 mm beidseits der Klemmung und der geschlitzte Rohrhandlauf 42,4 x 2 mm mit U-Dichtung über der Glaskante und zwei gewölbten Endkappen.",
            ),
            text(
                "Omitted below the 7.5 mm feature: washers (2.5 mm), the arrised glass edges (1.5 mm), the 1 mm play of the bolt holes in the steel plates beyond their 14 mm bore, and the part of the anchors inside the slab.",
                "Unter der Einzelheit von 7,5 mm entfallen: Unterlegscheiben (2,5 mm), die gesäumten Glaskanten (1,5 mm), das Lochspiel der Stahlbleche über die 14-mm-Bohrung hinaus und der Teil der Anker in der Decke.",
            ),
            text(
                "Thermal: every region is its body surfaces: the clamp region is base plate, clamp plates, welds, elastomer strips, setting blocks, sleeves, bolts and anchors (the strips and sleeves between glass and steel show in the cut), the glass region is both panes with the film, the handrail region is the outer skin of the closed hollow tube, modelled as the round that the tube and its gasket fill, with the two caps: the air inside the tube exchanges no heat with the surroundings.",
                "Thermisch: jede Region besteht aus ihren Körperflächen: die Klemmregion aus Fußplatte, Klemmblechen, Nähten, Elastomerstreifen, Klotzungen, Hülsen, Schrauben und Ankern (Streifen und Hülsen zwischen Glas und Stahl zeigen sich im Schnitt), die Glasregion aus beiden Scheiben mit der Folie, die Handlaufregion ist die Außenhaut des geschlossenen Hohlrohrs, modelliert als der Rundkörper, den Rohr und Dichtung füllen, mit den beiden Kappen: die Luft im Rohr tauscht keine Wärme mit der Umgebung.",
            ),
            text(
                "Structure: the load path of the cantilever: the handrail tube spreads the line load, the two glass panes are the member, the elastomer strips, the setting blocks, the two clamp plates, their welds and the seven bolts with heads and nuts are the fixed joint, the anchor studs with their nuts tie the base plate, which is the fixed support at the clamp anchor, to the slab; the mass of the one-metre glass strip is a disc in the laminate at 0.55 m. Film, sleeves, gasket and caps carry nothing and are absent.",
                "Tragwerk: der Lastpfad des Kragarms: das Handlaufrohr verteilt die Linienlast, die beiden Glasscheiben sind der Stab, Elastomerstreifen, Klotzungen, die beiden Klemmbleche, ihre Nähte und die sieben Schrauben mit Köpfen und Muttern sind die Einspannung, die Ankerbolzen mit ihren Muttern binden die Fußplatte, das feste Auflager am Klemmanker, an die Decke; die Masse des ein Meter breiten Glasstreifens ist eine Scheibe im Verbund bei 0,55 m. Folie, Hülsen, Dichtung und Kappen tragen nichts und fehlen.",
            ),
        ],
    )
    v100 = scale(
        100,
        body100(),
        thermal100(),
        structure100(),
        [
            text(
                "Body: authored for the 15 mm feature: the 15 mm base plate, each clamp plate merged with its elastomer strip into one 25 mm clamp bar, the laminate as one 21.5 mm glass body standing on the plate (film and setting blocks merged), each through-bolt with head and nut as one plain 20 mm rod standing 10 mm proud of both bars, each anchor stud with its nut as a 20 mm stub, the handrail as the slotted tube gripping the glass directly (gasket merged) with two flat 15 mm end plugs; welds, sleeves and the hexagon forms are below the feature.",
                "Körper: für die Einzelheit von 15 mm gezeichnet: die Fußplatte 15 mm, jedes Klemmblech mit seinem Elastomerstreifen zu einer Klemmleiste von 25 mm zusammengefasst, das Verbundglas als ein Glaskörper von 21,5 mm, der auf der Platte steht (Folie und Klotzungen zusammengefasst), jede Durchgangsschraube mit Kopf und Mutter als ein glatter Stab von 20 mm, der beidseits 10 mm vorsteht, jeder Ankerbolzen mit Mutter als Stumpf von 20 mm, der Handlauf als geschlitztes Rohr, das das Glas direkt fasst (Dichtung zusammengefasst), mit zwei flachen Endstopfen von 15 mm; Nähte, Hülsen und die Sechskantformen liegen unter der Einzelheit.",
            ),
            text(
                "Thermal: the merged body surfaces in their three regions; the handrail is the outer skin of the closed tube as one round over the full 2 m. Structure: the glass body and the handrail tube are the members, the two clamp bars with the seven bolts the fixed joint, the base plate with its ten anchor studs the support, the mass disc sits in the glass at 0.55 m.",
                "Thermisch: die zusammengefassten Körperflächen in ihren drei Regionen; der Handlauf ist die Außenhaut des geschlossenen Rohrs als ein Rundkörper über die vollen 2 m. Tragwerk: Glaskörper und Handlaufrohr sind die Stäbe, die beiden Klemmleisten mit den sieben Schrauben die Einspannung, die Fußplatte mit ihren zehn Ankerbolzen das Auflager, die Massenscheibe sitzt im Glas bei 0,55 m.",
            ),
        ],
    )
    v200 = scale(
        200,
        body200(),
        three("shoe"),
        structure200(),
        [
            text(
                "Body: authored for the 30 mm feature: base plate and clamp are one inverted T (flange 300 mm, drawn 30 mm thick instead of 15 mm, upstand 71.5 mm wide to 125 mm), the glass is one sheet over the full 2 m from the clamp into the handrail, the handrail a round of 42.4 mm with the notch that holds the glass edge; bolts, anchors and end caps are below the feature.",
                "Körper: für die Einzelheit von 30 mm gezeichnet: Fußplatte und Klemmung sind ein umgekehrtes T (Flansch 300 mm, 30 statt 15 mm dick gezeichnet, Steg 71,5 mm breit bis 125 mm), das Glas ist eine Scheibe über die vollen 2 m von der Klemmung bis in den Handlauf, der Handlauf ein Rundstab von 42,4 mm mit der Kerbe, die die Glaskante hält; Schrauben, Anker und Endkappen liegen unter der Einzelheit.",
            ),
            text(
                "Thermal: shoe, glass and handrail are the three regions. Structure: glass and handrail are the members, the clamp upstand is the fixed joint and the flange the support, each a solid of its own; the mass disc sits in the glass at 0.55 m.",
                "Thermisch: Schuh, Glas und Handlauf sind die drei Regionen. Tragwerk: Glas und Handlauf sind die Stäbe, der Klemmsteg ist die Einspannung und der Flansch das Auflager, je ein eigener Körper; die Massenscheibe sitzt im Glas bei 0,55 m.",
            ),
        ],
    )
    v500 = scale(
        500,
        body500(),
        three("foot"),
        structure500(),
        [
            text(
                "Minimal: a digital-only scale (the print is unavailable at 1:500) of three bodies for the 75 mm feature: the foot is a coarse inverted T (plate 300 mm wide, drawn 37.5 mm thick, clamp 75 mm wide to 125 mm), the glass is one sheet at its true 21.5 mm, the handrail a square bar of 42.4 mm resting on the glass edge.",
                "Minimal: ein rein digitaler Maßstab (der Druck ist bei 1:500 nicht verfügbar) aus drei Körpern für die Einzelheit von 75 mm: der Fuß ist ein grobes umgekehrtes T (Platte 300 mm breit, 37,5 mm dick gezeichnet, Klemmung 75 mm breit bis 125 mm), das Glas ist eine Scheibe in wahrer Dicke 21,5 mm, der Handlauf ein Vierkantstab von 42,4 mm auf der Glaskante.",
            ),
            text(
                "Thermal: foot, glass and handrail are the three regions. Structure: glass and handrail are the members and the foot is the fixed support at the clamp anchor.",
                "Thermisch: Fuß, Glas und Handlauf sind die drei Regionen. Tragwerk: Glas und Handlauf sind die Stäbe, der Fuß ist das feste Auflager am Klemmanker.",
            ),
        ],
    )
    return {"volumes": {"50": v50, "100": v100, "200": v200, "500": v500}}


def main():
    """🚀 Write the volumes source."""
    path = os.path.join(HERE, "volumes", "components", "glass-balustrade.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump(build(), f, ensure_ascii=False, indent=2)
        f.write("\n")
    print(path)


if __name__ == "__main__":
    main()
