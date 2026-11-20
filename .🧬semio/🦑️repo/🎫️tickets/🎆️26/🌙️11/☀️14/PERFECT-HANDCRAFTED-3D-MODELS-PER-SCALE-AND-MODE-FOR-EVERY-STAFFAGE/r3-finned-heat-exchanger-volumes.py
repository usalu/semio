"""🧊 Round 3, machinery/finned-heat-exchanger: every solid of every scale and list, stated by hand in world coordinates (x ahead, y up, z left) and carried into the vertical bone frames by `place`.

The coil: 134 aluminium plate fins of 0.2 mm at 3 mm pitch (69.3 mm by 480 mm) on 24 copper tubes of 5/8 in (15.88 mm) in two staggered rows (40 mm tube pitch, 34.64 mm row pitch), finned length 0.4 m; 12 hairpins of 20 mm radius join the rows on the right end; on the left end every tube runs into its row's vertical header of 28 mm (return header behind with the stub at the top, supply header ahead with the stub at the bottom); two flanged tube sheets and two channels of 1.5 mm galvanised sheet form the casing that stands on the floor.

Usage: imported by `r3-finned-heat-exchanger-build.py` (`volumes()`).
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("fhx_geo", os.path.join(HERE, "r3-finned-heat-exchanger-geo.py"))
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)

STEEL, ALU, CU = "galvanised-steel", "aluminium-fin", "copper"
TONE = {"frame": 0.5, "fins": 0.78, "tubes": 0.4}
MAT = {"frame": STEEL, "fins": ALU, "tubes": CU}
MASS = {"fins": 2.1, "tubes": 2.7, "frame": 3.1}


def solid(id, bone, kind, role="body", **geometry):
    """🧱 A solid in world axes carried into its bone frame; `repeat_step` is a world vector."""
    step = geometry.pop("repeat_step", None)
    count = geometry.pop("repeat_count", None)
    extra = {k: geometry.pop(k) for k in ("sectionFaces", "texture", "slender") if k in geometry}
    out = {"id": id, "kind": kind, "bone": bone, "material": MAT[bone], "role": role, "tone": geometry.pop("tone", TONE[bone])}
    out.update(geometry)
    out["transform"] = G.place(bone)
    if count:
        out["repeat"] = {"count": count, "translation": G.vec(*step)}
    out.update(extra)
    return out


def tr(bone, p):
    """📍 Translation of a solid authored about its own centre p (world) into the bone frame."""
    ox, oy = G.RIG[bone]
    return [round(p[1] - oy, 6), round(ox - p[0], 6), round(p[2], 6)]


def extrude(id, bone, plane, profile, lo, hi, holes=None, **kw):
    """📐 Extrusion of a world-plane profile (xy along z, yz along x, zx along y)."""
    g = {"plane": plane, "profile": [[round(a, 6), round(b, 6)] for a, b in profile], "from": round(lo, 6), "to": round(hi, 6)}
    if holes:
        g["holes"] = [[[round(a, 6), round(b, 6)] for a, b in h] for h in holes]
    return solid(id, bone, "extrude", **g, **kw)


def rod(id, bone, origin, axis, profile, **kw):
    """🥢 Revolve of an [axial, radial] profile about a world axis through a world origin."""
    return solid(id, bone, "revolve", profile=[[round(a, 6), round(b, 6)] for a, b in profile], axis=axis, origin=[round(v, 6) for v in origin], **kw)


def pipe(id, bone, path, r, **kw):
    """🪈 A bent pipe: circular loft sections along a world centre line, capped."""
    return solid(id, bone, "loft", sections=[{"at": p, "rx": r, "ry": r} for p in path], capStart=True, capEnd=True, **kw)


def ell(id, bone, radii, centre, **kw):
    """🥚 An ellipsoid about a world centre (radii along world x, y, z)."""
    s = solid(id, bone, "ellipsoid", radii=[round(v, 6) for v in radii], **kw)
    s["transform"] = {**G.place(bone), "translation": tr(bone, centre)}
    return s


def cube(id, bone, lo, hi, **kw):
    """▭ An axis-aligned world box between two world corners."""
    s = solid(id, bone, "box", size=[round(b - a, 6) for a, b in zip(lo, hi)], **kw)
    s["transform"] = {**G.place(bone), "translation": tr(bone, [(a + b) / 2 for a, b in zip(lo, hi)])}
    return s


def rect(a0, b0, a1, b1):
    """⬜ Counter-clockwise rectangle profile."""
    return [[a0, b0], [a1, b0], [a1, b1], [a0, b1]]


def channel(id, up):
    """🛤️ A casing channel of 1.5 mm sheet between the tube sheets: the web lies on the fin pack, the two legs stand in the air faces, turned away from the pack (down to the floor at the bottom, up at the top)."""
    x, t = G.CASE_X, G.SHEET
    if up:
        y0, y1 = G.FIN_Y[1], G.TOP
        prof = [[-x, y1], [-x, y0], [x, y0], [x, y1], [x - t, y1], [x - t, y0 + t], [-x + t, y0 + t], [-x + t, y1]]
    else:
        y0, y1 = 0.0, G.FIN_Y[0]
        prof = [[-x, y0], [-x + t, y0], [-x + t, y1 - t], [x - t, y1 - t], [x - t, y0], [x, y0], [x, y1], [-x, y1]]
    return extrude(id, "frame", "xy", prof, -G.FIN_Z, G.FIN_Z)


def tube_sheet_profile(id, side, top=G.TOP, lip=G.FLANGE + G.SHEET):
    """🗜️ A tube sheet as one C profile of 1.5 mm sheet: the web the tubes pass through and the two 25 mm flanges turned away from the pack."""
    x, t, z0, z1 = G.CASE_X, G.SHEET, G.FIN_Z, G.FIN_Z + lip
    prof = [[z0, -x], [z1, -x], [z1, -x + t], [z0 + t, -x + t], [z0 + t, x - t], [z1, x - t], [z1, x], [z0, x]]
    if side < 0:
        prof = [[-a, b] for a, b in prof][::-1]
    return extrude(id, "frame", "zx", prof, 0.0, top)


def tubes():
    """🧪 The two tube rows: each tube runs from the root of its hairpin through both tube sheets into the axis of its header."""
    L = G.HEADER_Z - G.BEND_Z0
    prof = [[0, 0], [0, G.TUBE_R], [L, G.TUBE_R], [L, 0]]
    return [
        rod("tube-row-entering", "tubes", [G.ROW_X[0], G.tube_y(0, 0), G.BEND_Z0], [0, 0, 1], prof, repeat_count=G.TUBES_PER_ROW, repeat_step=(0, G.TUBE_PITCH, 0)),
        rod("tube-row-leaving", "tubes", [G.ROW_X[1], G.tube_y(1, 0), G.BEND_Z0], [0, 0, 1], prof, repeat_count=G.TUBES_PER_ROW, repeat_step=(0, G.TUBE_PITCH, 0)),
    ]


def bends(n):
    """🔁 The 12 hairpins on the right end, each joining tube k of the entering row with tube k of the leaving row."""
    return [pipe("hairpins", "tubes", G.bend_path(0, n), G.TUBE_R, repeat_count=G.TUBES_PER_ROW, repeat_step=(0, G.TUBE_PITCH, 0))]


def headers(domed):
    """🚿 The return header (entering row, behind) and the supply header (leaving row, ahead): vertical 28 mm pipes with end caps."""
    out = []
    for name, row in (("header-return", 0), ("header-supply", 1)):
        y0, y1 = G.HEADER_Y[row]
        L = y1 - y0
        prof = G.dome(L, G.HEADER_R, 0.007, 4) if domed else [[0, 0], [0, G.HEADER_R], [L, G.HEADER_R], [L, 0]]
        out.append(rod(name, "tubes", [G.ROW_X[row], y0, G.HEADER_Z], [0, 1, 0], prof))
    return out


def stubs(detail):
    """🚰 The two connection stubs leaving the headers to the left: outlet (return) at the top, inlet (supply) at the bottom; at 1:50 with the threaded end sleeve and the open bore."""
    out = []
    a = G.STUB_END - G.HEADER_Z
    for name, row in (("outlet-stub", 0), ("inlet-stub", 1)):
        if detail:
            b = a - G.SLEEVE_L
            c = a - G.BORE_L
            prof = [[0, 0], [0, G.HEADER_R], [b, G.HEADER_R], [b, G.SLEEVE_R], [a, G.SLEEVE_R], [a, G.BORE_R], [c, G.BORE_R], [c, 0]]
        else:
            prof = [[0, 0], [0, G.HEADER_R], [a, G.HEADER_R], [a, 0]]
        out.append(rod(name, "tubes", [G.ROW_X[row], G.STUB_Y[row], G.HEADER_Z], [0, 0, 1], prof))
    return out


def plugs():
    """🔩 The vent plug on top of the return header and the drain plug under the supply header."""
    prof = [[0, 0], [0, G.PLUG_R], [G.PLUG_L + 0.002, G.PLUG_R], [G.PLUG_L + 0.002, 0]]
    return [
        rod("vent-plug", "tubes", [G.ROW_X[0], G.HEADER_Y[0][1] - 0.002, G.HEADER_Z], [0, 1, 0], prof, tone=0.3),
        rod("drain-plug", "tubes", [G.ROW_X[1], G.HEADER_Y[1][0] + 0.002, G.HEADER_Z], [0, -1, 0], prof, tone=0.3),
    ]


def fin_edges():
    """〰️ The fin edges on both air faces: 134 fins of 0.2 mm at 3 mm pitch standing 2 mm proud of the pack core (texture)."""
    z0 = -(G.FIN_COUNT - 1) * G.FIN_PITCH / 2
    out = []
    for name, x0, x1 in (("fin-edges-ahead", G.FIN_X - 0.0025, G.FIN_X), ("fin-edges-behind", -G.FIN_X, -G.FIN_X + 0.0025)):
        out.append(cube(name, "fins", [x0, G.FIN_Y[0], z0 - G.FIN_T / 2], [x1, G.FIN_Y[1], z0 + G.FIN_T / 2], role="texture", texture=True, repeat_count=G.FIN_COUNT, repeat_step=(0, 0, G.FIN_PITCH)))
    return out


def ref(id, of, **kw):
    """🔗 A reference to a body solid."""
    out = {"id": id, "of": of}
    out.update(kw)
    return out


def part(s, id, member, kind, **kw):
    """🏗️ Gives a structural solid its id, member and part."""
    s["id"] = id
    s["member"] = member
    s["part"] = kind
    s.update(kw)
    return s


def collars(id, row, side):
    """⭕ The expanded tube-to-sheet joints of one row in one tube sheet: a collar around every tube where it passes the sheet."""
    z = side * (G.FIN_Z + G.SHEET / 2)
    r0, r1, h = G.TUBE_R, G.TUBE_R + 0.004, 0.00375
    prof = [[-h, r0], [h, r0], [h, r1], [-h, r1], [-h, r0]]
    return part(rod("x", "tubes", [G.ROW_X[row], G.tube_y(row, 0), z], [0, 0, 1], prof, repeat_count=G.TUBES_PER_ROW, repeat_step=(0, G.TUBE_PITCH, 0)), id, "tubes", "joint")


def header_joints(id, row):
    """🔗 The brazed joints of one row's tubes in its header: a saddle ring around every tube at the header wall."""
    z = G.HEADER_Z - G.HEADER_R
    r0, r1, h = G.TUBE_R, G.TUBE_R + 0.003, 0.00375
    prof = [[-h, r0], [h, r0], [h, r1], [-h, r1], [-h, r0]]
    return part(rod("x", "tubes", [G.ROW_X[row], G.tube_y(row, 0), z], [0, 0, 1], prof, repeat_count=G.TUBES_PER_ROW, repeat_step=(0, G.TUBE_PITCH, 0)), id, "tubes", "joint")


def fin_plates(count, t):
    """🍃 Representative fin plates of the pack: each fin is a plate that hangs on the 24 tubes by its collars."""
    step = 2 * (G.FIN_Z - 0.04) / (count - 1) if count > 1 else 0
    z0 = -(G.FIN_Z - 0.04) if count > 1 else 0.0
    s = cube("x", "fins", [-G.FIN_X, G.FIN_Y[0], z0 - t / 2], [G.FIN_X, G.FIN_Y[1], z0 + t / 2])
    if count > 1:
        s["repeat"] = {"count": count, "translation": G.vec(0, 0, step)}
    return part(s, "member-fin-plates", "fins", "member")


def corner_joints(id, y0, y1, t=0.0075):
    """🔩 The screwed lap joints of a channel in the two tube sheets."""
    s = cube("x", "frame", [-G.CASE_X, y0, -G.FIN_Z], [G.CASE_X, y1, -G.FIN_Z + t], repeat_count=2, repeat_step=(0, 0, 2 * G.FIN_Z - t))
    return part(s, id, "frame", "joint")


def posts(lip):
    """🏛️ The two tube sheets as the posts of the casing: the web that carries the 24 tubes down to the floor, with the root of its two flanges as stiffening lips."""
    return [part(tube_sheet_profile("x", side, G.TOP, lip), f"member-tube-sheet-{name}", "frame", "member") for name, side in (("left", 1), ("right", -1))]


def support(t):
    """🦶 The contact under the feet of the two tube sheets: the foot of the C profile of each sheet, the second by a half turn about the vertical axis."""
    s = tube_sheet_profile("x", 1, t)
    s["repeat"] = {"count": 2, "rotationAxis": [1, 0, 0], "rotationRad": round(math.pi, 9), "origin": [0, 0, 0]}
    return part(s, "support-base-contact", "frame", "support", anchor="base-contact")


def support_pads(t, z0, z1):
    """🦶 The contact pads under the two end plates (1:200)."""
    s = cube("x", "frame", [-G.CASE_X, 0.0, -z1], [G.CASE_X, t, -z0], repeat_count=2, repeat_step=(0, 0, z0 + z1))
    return part(s, "support-base-contact", "frame", "support", anchor="base-contact")


def mass(id, bone, kg, centre):
    """⚖️ A concentrated mass at a centre of mass, its radius growing with the cube root of the mass."""
    r = round(0.016 * kg ** (1 / 3), 4)
    return part(ell("x", bone, [r, r, r], centre), id, bone, "mass")


def t(en, de):
    """🗣️ Bilingual text."""
    return {"en": en, "de": de}


def masses_full():
    """⚖️ Fin pack, tube circuit with headers (its centre shifted towards the header end) and the two tube sheets with their halves of the channels."""
    return [
        mass("mass-fins", "fins", MASS["fins"], [0, 0.26, 0.0]),
        mass("mass-tubes", "tubes", MASS["tubes"], [0, 0.26, 0.067]),
        mass("mass-tube-sheet-left", "frame", MASS["frame"] / 2, [0, 0.26, 0.162]),
        mass("mass-tube-sheet-right", "frame", MASS["frame"] / 2, [0, 0.26, -0.162]),
    ]


def scale50():
    """🔬 1:50: every assembly at its real size."""
    body = [
        channel("casing-bottom", False),
        channel("casing-top", True),
        tube_sheet_profile("tube-sheet-left", 1),
        tube_sheet_profile("tube-sheet-right", -1),
        cube("fin-pack", "fins", [-G.FIN_X + 0.002, G.FIN_Y[0], -G.FIN_Z], [G.FIN_X - 0.002, G.FIN_Y[1], G.FIN_Z]),
        *fin_edges(),
        *tubes(),
        *bends(10),
        *headers(True),
        *stubs(True),
        *plugs(),
    ]
    casing = ["casing-bottom", "casing-top", "tube-sheet-left", "tube-sheet-right"]
    thermal = [ref("th-fin-pack", "fin-pack", region="fins")]
    thermal += [ref(f"th-{i}", i, region="tubes") for i in ("tube-row-entering", "tube-row-leaving", "hairpins")]
    thermal += [ref(f"th-{i}", i, region="headers") for i in ("header-return", "header-supply", "outlet-stub", "inlet-stub", "vent-plug", "drain-plug")]
    thermal += [ref(f"th-{i}", i, region="casing") for i in casing]
    structure = [ref(f"member-{i}", i, member="frame", part="member") for i in ("casing-bottom", "casing-top")] + posts(0.0075)
    structure += [ref(f"member-{i}", i, member="tubes", part="member") for i in ("tube-row-entering", "tube-row-leaving")]
    structure.append(fin_plates(5, 0.0075))
    structure += [collars(f"joint-{row}-{name}", k, side) for row, k in (("entering", 0), ("leaving", 1)) for name, side in (("left", 1), ("right", -1))]
    structure += [header_joints("joint-header-return", 0), header_joints("joint-header-supply", 1)]
    structure += [corner_joints("joint-channel-bottom", 0.0, G.CASE_H), corner_joints("joint-channel-top", G.FIN_Y[1], G.TOP)]
    structure.append(support(0.0075))
    structure += masses_full()
    decisions = [
        t("The coil as it is built, a two-row coil of the P40 geometry: 134 aluminium plate fins of 0.2 mm at 3 mm pitch, 69.3 mm deep and 0.48 m high over a finned length of 0.4 m, are far below the 7.5 mm feature, so the pack is a core block 2 mm inside each air face with the real fin edges standing out of it as texture; 24 copper tubes of 5/8 in (15.88 mm) in two staggered rows (40 mm tube pitch, 34.64 mm row pitch) pass through the pack and both tube sheets; 12 hairpins of 20 mm bend radius join tube k of the entering row with tube k of the leaving row beyond the right tube sheet.", "Das Register, wie es gebaut ist, ein zweireihiges Register der Geometrie P40: 134 Aluminiumlamellen von 0,2 mm bei 3 mm Teilung, 69,3 mm tief und 0,48 m hoch über 0,4 m berippte Länge, liegen weit unter dem 7,5-mm-Merkmal, daher ist das Paket ein Kernblock 2 mm hinter jeder Luftseite, aus dem die echten Lamellenkanten als Textur herausstehen; 24 Kupferrohre von 5/8 Zoll (15,88 mm) in zwei versetzten Reihen (40 mm Rohrteilung, 34,64 mm Reihenteilung) laufen durch das Paket und beide Rohrböden; 12 Haarnadelbögen mit 20 mm Biegeradius verbinden hinter dem rechten Rohrboden Rohr k der Eintrittsreihe mit Rohr k der Austrittsreihe."),
        t("On the left end every tube runs into the vertical 28 mm header of its row: the return header behind (entering row) with the outlet stub at the top and a vent plug on its cap, the supply header ahead (leaving row, counterflow) with the inlet stub at the bottom and a drain plug under its cap; headers have domed caps, the stubs end in a 34 mm threaded sleeve with the open bore. Tubes are modelled full, since their bore is closed by hairpins and headers.", "Am linken Ende mündet jedes Rohr in den senkrechten 28-mm-Sammler seiner Reihe: hinten der Rücklaufsammler (Eintrittsreihe) mit dem Austrittsstutzen oben und einem Entlüftungsstopfen auf der Kappe, vorn der Vorlaufsammler (Austrittsreihe, Gegenstrom) mit dem Eintrittsstutzen unten und einem Entleerungsstopfen unter der Kappe; die Sammler haben gewölbte Kappen, die Stutzen enden in einer 34-mm-Gewindemuffe mit offener Bohrung. Die Rohre sind voll modelliert, da ihre Bohrung durch Bögen und Sammler verschlossen ist."),
        t("The casing is 1.5 mm galvanised sheet at its real thickness: two tube sheets (each one C profile: the web the tubes pass through and two 25 mm flanges turned away from the pack; tube and mounting holes are not cut, because a flat sheet with holes falls under the 7.5 mm feature rule) and two channels of 100 mm by 20 mm between them, whose web lies on the pack and whose legs stand in the air faces; the coil stands on the feet of the tube sheets and the legs of the bottom channel. The 2D section A-A cuts the coil across its tubes at mid length.", "Das Gehäuse ist 1,5 mm verzinktes Blech in echter Dicke: zwei Rohrböden (je ein C-Profil: der Steg, durch den die Rohre laufen, und zwei vom Paket weg gekantete 25-mm-Flansche; Rohr- und Befestigungslöcher sind nicht ausgeschnitten, weil ein flaches Blech mit Löchern unter die 7,5-mm-Merkmalregel fällt) und dazwischen zwei Profile von 100 mm mal 20 mm, deren Steg am Paket anliegt und deren Schenkel in den Luftseiten stehen; das Register steht auf den Füßen der Rohrböden und den Schenkeln des unteren Profils. Der 2D-Schnitt A-A schneidet das Register in halber Länge quer zu den Rohren."),
        t("Thermal: the fin region (exchange area) is the pack core; the tube region is the 24 tubes and 12 hairpins; the header region is both headers, both stubs and the two plugs; the casing region is the tube sheets, flanges and channels. Every region has one layer, so there are no nested shells.", "Thermisch: die Lamellenregion (Austauschfläche) ist der Paketkern; die Rohrregion sind die 24 Rohre und 12 Bögen; die Sammlerregion sind beide Sammler, beide Stutzen und die zwei Stopfen; die Gehäuseregion sind Rohrböden, Flansche und Profile. Jede Region hat eine Schicht, daher gibt es keine verschachtelten Schalen."),
        t("Structure: every fin is a plate that hangs on the tubes by its collars (five representative fin plates, drawn 7.5 mm thick); the 24 tubes span between the tube sheets (members) with an expanded collar joint at every pass through a sheet and a brazed joint at every header entry; the two tube sheets (web with the roots of its flanges as stiffening lips) are the posts and the two channels the rails of the casing frame (lap joints at the four channel ends), which stands on the feet of the two tube sheets (support); masses of the fin pack, of the tube circuit with its headers (shifted towards the header end) and of the two casing halves.", "Tragwerk: jede Lamelle ist eine Platte, die mit ihren Kragen an den Rohren hängt (fünf stellvertretende Lamellenplatten, 7,5 mm dick gezeichnet); die 24 Rohre spannen zwischen den Rohrböden (Stäbe) mit einer aufgeweiteten Kragenverbindung an jedem Durchgang durch einen Rohrboden und einer Lötverbindung an jedem Sammlereintritt; die zwei Rohrböden (Steg mit den Wurzeln seiner Flansche als Versteifungslippen) sind die Pfosten und die zwei Profile die Riegel des Gehäuserahmens (Überlappstöße an den vier Profilenden), der auf den Füßen der zwei Rohrböden steht (Auflager); Massen des Lamellenpakets, des Rohrkreises mit seinen Sammlern (zum Sammlerende verschoben) und der zwei Gehäusehälften."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale100():
    """🔭 1:100: plain pack, tube sheets as one C profile, plain headers and stubs, coarser hairpins."""
    body = [
        channel("casing-bottom", False),
        channel("casing-top", True),
        tube_sheet_profile("tube-sheet-left", 1),
        tube_sheet_profile("tube-sheet-right", -1),
        cube("fin-pack", "fins", [-G.FIN_X, G.FIN_Y[0], -G.FIN_Z], [G.FIN_X, G.FIN_Y[1], G.FIN_Z]),
        *tubes(),
        *bends(5),
        *headers(False),
        *stubs(False),
    ]
    casing = ["casing-bottom", "casing-top", "tube-sheet-left", "tube-sheet-right"]
    thermal = [ref("th-fin-pack", "fin-pack", region="fins")]
    thermal += [ref(f"th-{i}", i, region="tubes") for i in ("tube-row-entering", "tube-row-leaving", "hairpins")]
    thermal += [ref(f"th-{i}", i, region="headers") for i in ("header-return", "header-supply", "outlet-stub", "inlet-stub")]
    thermal += [ref(f"th-{i}", i, region="casing") for i in casing]
    structure = [ref(f"member-{i}", i, member="frame", part="member") for i in ("casing-bottom", "casing-top")] + posts(0.015)
    structure += [ref(f"member-{i}", i, member="tubes", part="member") for i in ("tube-row-entering", "tube-row-leaving")]
    structure.append(fin_plates(3, 0.015))
    for name, side in (("left", 1), ("right", -1)):
        z = side * (G.FIN_Z + G.SHEET / 2)
        structure.append(part(cube("x", "tubes", [-0.03, 0.02, z - 0.0075], [0.03, 0.5, z + 0.0075]), f"joint-tube-sheet-{name}", "tubes", "joint"))
    structure.append(support(0.015))
    structure += masses_full()
    decisions = [
        t("At 1:100 the fins are below every feature and leave no texture: the pack is a plain block of 69.3 mm by 0.48 m by 0.4 m between the tube sheets; the 24 tubes keep their real 15.88 mm (above the 15 mm feature) and their staggered pattern; the 12 hairpins are coarser lofts; the headers are plain 28 mm pipes with flat ends and the stubs plain 28 mm pipes without sleeve and bore; the plugs are gone.", "Bei 1:100 liegen die Lamellen unter jedem Merkmal und hinterlassen keine Textur: das Paket ist ein glatter Block von 69,3 mm mal 0,48 m mal 0,4 m zwischen den Rohrböden; die 24 Rohre behalten ihre echten 15,88 mm (über dem 15-mm-Merkmal) und ihr versetztes Muster; die 12 Haarnadelbögen sind gröbere Lofts; die Sammler sind glatte 28-mm-Rohre mit flachen Enden und die Stutzen glatte 28-mm-Rohre ohne Muffe und Bohrung; die Stopfen entfallen."),
        t("The casing keeps its real 1.5 mm sheet (wall thickness of open profiles is kept at 1:100): tube sheets and channels are the C and U profiles of the real sections.", "Das Gehäuse behält sein echtes 1,5-mm-Blech (die Wanddicke offener Profile bleibt bei 1:100 erhalten): Rohrböden und Profile sind die C- und U-Profile der echten Querschnitte."),
        t("Thermal: the same four regions as at 1:50 on the coarser solids. Structure: tube-sheet posts, channels and tubes as members, three representative fin plates, one joint band per tube sheet for the 24 collar joints, the tube-sheet feet as support, and the same four masses.", "Thermisch: dieselben vier Regionen wie bei 1:50 auf den gröberen Körpern. Tragwerk: Rohrbodenpfosten, Profile und Rohre als Stäbe, drei stellvertretende Lamellenplatten, je Rohrboden ein Verbindungsband für die 24 Kragenverbindungen, die Rohrbodenfüße als Auflager und dieselben vier Massen."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    """🗺️ 1:200: pack and tubes one block, the casing four bars and plates, hairpins merged into the right end plate, one header block and two stubs."""
    body = [
        cube("casing-bottom", "frame", [-G.CASE_X, 0.0, -G.FIN_Z], [G.CASE_X, 0.03, G.FIN_Z]),
        cube("casing-top", "frame", [-G.CASE_X, 0.49, -G.FIN_Z], [G.CASE_X, G.TOP, G.FIN_Z]),
        cube("end-plate-left", "frame", [-G.CASE_X, 0.0, G.FIN_Z], [G.CASE_X, G.TOP, 0.23]),
        cube("end-plate-right", "frame", [-G.CASE_X, 0.0, G.RIGHT], [G.CASE_X, G.TOP, -G.FIN_Z]),
        cube("coil-block", "fins", [-G.FIN_X, 0.03, -G.FIN_Z], [G.FIN_X, 0.49, G.FIN_Z]),
        cube("header-block", "tubes", [-0.035, 0.012, 0.23], [0.035, 0.508, 0.26]),
    ]
    for name, row in (("outlet-stub", 0), ("inlet-stub", 1)):
        a = G.STUB_END - 0.26
        body.append(rod(name, "tubes", [G.ROW_X[row], G.STUB_Y[row], 0.26], [0, 0, 1], [[0, 0], [0, 0.015], [a, 0.015], [a, 0]]))
    bank = cube("th-tube-bank", "tubes", [-0.0253, 0.03, -G.FIN_Z], [0.0253, 0.49, G.FIN_Z])
    bank["region"] = "tubes"
    thermal = [ref("th-coil-block", "coil-block", region="fins"), bank]
    thermal += [ref(f"th-{i}", i, region="headers") for i in ("header-block", "outlet-stub", "inlet-stub")]
    casing = ["casing-bottom", "casing-top", "end-plate-left", "end-plate-right"]
    thermal += [ref(f"th-{i}", i, region="casing") for i in casing]
    structure = [ref(f"member-{i}", i, member="frame", part="member") for i in casing]
    L = 2 * G.FIN_Z
    structure.append(part(rod("x", "tubes", [0, 0.08, -G.FIN_Z], [0, 0, 1], [[0, 0], [0, 0.015], [L, 0.015], [L, 0]], repeat_count=4, repeat_step=(0, 0.12, 0)), "member-tube-bars", "tubes", "member"))
    structure.append(part(cube("x", "fins", [-G.FIN_X, 0.03, -0.015], [G.FIN_X, 0.49, 0.015]), "member-fin-plates", "fins", "member"))
    structure.append(support_pads(0.03, G.FIN_Z, 0.23))
    structure.append(mass("mass-coil", "fins", MASS["fins"] + MASS["tubes"], [0, 0.26, 0.0]))
    decisions = [
        t("At 1:200 (30 mm feature) the coil is a few masses: the fin pack with the tubes inside it is one block of 69.3 mm by 0.46 m by 0.4 m; the casing is two 30 mm bars (the 20 mm channels thickened into the pack) and two end plates of the full 0.1 m by 0.52 m, the left one 30 mm (tube sheet and flange), the right one 39 mm because it takes in the hairpins; the two headers are one block of 70 mm by 30 mm and the stubs 30 mm pipes leaving to the left at the top behind and the bottom ahead.", "Bei 1:200 (30-mm-Merkmal) besteht das Register aus wenigen Massen: das Lamellenpaket mit den Rohren darin ist ein Block von 69,3 mm mal 0,46 m mal 0,4 m; das Gehäuse sind zwei 30-mm-Riegel (die 20-mm-Profile ins Paket verdickt) und zwei Endplatten von vollen 0,1 m mal 0,52 m, die linke 30 mm (Rohrboden und Flansch), die rechte 39 mm, weil sie die Haarnadelbögen aufnimmt; die zwei Sammler sind ein Block von 70 mm mal 30 mm und die Stutzen 30-mm-Rohre, die oben hinten und unten vorn nach links austreten."),
        t("Thermal: the block carries the fins, one slab inside it stands for the two tube rows, the header block and the stubs are the header region, bars and end plates the casing. Structure: bars and end plates as the casing frame, four 30 mm bars between the end plates, each standing for six tubes, one fin plate hanging on them, the pads under the end plates as support and one mass for the coil.", "Thermisch: der Block trägt die Lamellen, eine Scheibe darin steht für die zwei Rohrreihen, Sammlerblock und Stutzen sind die Sammlerregion, Riegel und Endplatten das Gehäuse. Tragwerk: Riegel und Endplatten als Gehäuserahmen, vier 30-mm-Stäbe zwischen den Endplatten, jeder für sechs Rohre, eine daran hängende Lamellenplatte, die Flächen unter den Endplatten als Auflager und eine Masse für das Register."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def volumes():
    """🗂️ All four scales (1:500 stays unavailable, as its representation)."""
    return {"50": scale50(), "100": scale100(), "200": scale200(), "500": {"status": "unavailable", "denominator": 500}}
