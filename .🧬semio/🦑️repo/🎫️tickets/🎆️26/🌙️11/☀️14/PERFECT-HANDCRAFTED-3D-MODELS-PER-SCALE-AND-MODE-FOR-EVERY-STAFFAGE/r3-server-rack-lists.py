"""🗄 Round 3, appliances/server-rack: the thermal and structural lists and the volume decisions of every authored scale."""
import importlib.util
import os

_spec = importlib.util.spec_from_file_location("r3_server_rack_kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-server-rack-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

STEEL, MESH, CHASSIS, BLANK, RUBBER = "rack-steel", "mesh-steel", "chassis", "blank-steel", "caster-rubber"
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}


def th(solid, region, layer=None, faces=None):
    """🌡 Marks a solid as a thermal solid of a region (and layer)."""
    solid["region"] = region
    if layer:
        solid["layer"] = layer
    if faces:
        solid["sectionFaces"] = faces
    return solid


def ref(sid, of, region):
    """🔗 A thermal reference to a body solid."""
    return {"id": sid, "of": of, "region": region}


def st(solid, member, part, anchor=None):
    """🏗 Marks a solid as a structural solid."""
    solid["member"] = member
    solid["part"] = part
    if anchor:
        solid["anchor"] = anchor
    return solid


def sref(sid, of, member, part):
    """🔗 A structural reference to a body solid that is itself the member."""
    return {"id": sid, "of": of, "member": member, "part": part}


def fronts(scale):
    """🖥 The operating front faces of the units: the outer skin of each front plate, its back glued to the chassis."""
    out = []
    zf = -0.408 if scale == "50" else (-0.415 if scale == "100" else (-0.43 if scale == "200" else -0.49))
    t = {"servers": 0.002, "switch": 0.0015, "blanks": 0.0015}
    layer = {"servers": "front-bezel", "switch": "switch-front", "blanks": "blank-sheet"}
    half = 0.2 if scale == "500" else K.EAR
    groups = {
        "50": [("th-storage", "servers", [("storage", 1, 4)]), ("th-server", "servers", [("server-5", 5, 2)]), ("th-patch", "blanks", [("p", 40, 1)]), ("th-brush", "blanks", [("b", 41, 1)]), ("th-switch", "switch", [("s", 42, 1)])],
        "100": [("th-servers", "servers", [("storage", 1, 4), ("server-15", 15, 2)]), ("th-blanks", "blanks", [("p", 40, 1), ("b", 41, 1)]), ("th-switch", "switch", [("s", 42, 1)])],
    }
    groups["200"] = groups["100"]
    groups["500"] = [("th-servers", "servers", [("x", 1, 1), ("y", 15, 2)]), ("th-blanks", "blanks", [("p", 40, 1), ("b", 41, 1)]), ("th-switch", "switch", [("s", 42, 1)])]
    for sid, region, span in groups[scale]:
        y0 = K.unit_y(span[0][1], span[0][2])[0]
        y1 = K.unit_y(span[-1][1], span[-1][2])[1]
        if scale == "500" and sid == "th-servers":
            y0 = K.RAIL_Y0
        if scale == "500" and sid == "th-switch":
            y1 = K.RAIL_Y1
        solid = th(K.box(sid, "equipment", CHASSIS if region != "blanks" else BLANK, (-half, half), (y0, y1), (zf - t[region], zf), tone=0.4), region, layer[region], ["+w"])
        if sid == "th-server":
            K.repeat(solid, 6, dy=2 * K.U)
        out.append(solid)
    return out


def cabinet(scale):
    """🧱 The cabinet sheets as the outer skins of the side panels, roof, rear door and base frame, their inner faces bounding the sealed airway of the closed cabinet."""
    zf = K.FRAME_Z
    out = []
    if scale == "50":
        for tag, (z0, z1) in (("f", (-zf, -0.0025)), ("r", (0.0025, zf))):
            out.append(th(K.box(f"th-side-a-{tag}", "frame", STEEL, (-K.HALF_W, -K.HALF_W + 0.0012), (0.070, 1.965), (z0, z1)), "cabinet", "cabinet-sheet", ["-v"]))
            out.append(th(K.box(f"th-side-b-{tag}", "frame", STEEL, (K.HALF_W - 0.0012, K.HALF_W), (0.070, 1.965), (z0, z1)), "cabinet", "cabinet-sheet", ["+v"]))
        for tag, (x0, x1) in (("a", (-K.DOOR_X, -0.001)), ("b", (0.001, K.DOOR_X))):
            out.append(th(K.box(f"th-rear-door-{tag}", "frame", STEEL, (x0, x1), (K.DOOR_Y0, K.DOOR_Y1), (zf, K.HALF_D)), "cabinet", "cabinet-sheet", ["-w"]))
    else:
        side = {"100": 0.0015, "200": 0.03, "500": 0.075}[scale]
        out.append(th(K.box("th-side-a", "frame", STEEL, (-K.HALF_W, -K.HALF_W + side), (K.BASE_Y0, 1.965 if scale == "100" else 1.94), (-zf, zf)), "cabinet", "cabinet-sheet", ["-v"]))
        out.append(th(K.box("th-side-b", "frame", STEEL, (K.HALF_W - side, K.HALF_W), (K.BASE_Y0, 1.965 if scale == "100" else 1.94), (-zf, zf)), "cabinet", "cabinet-sheet", ["+v"]))
        rear = {"100": 0.025, "200": 0.03, "500": 0.075}[scale]
        out.append(th(K.box("th-rear-door", "frame", STEEL, (-K.DOOR_X, K.DOOR_X), (K.DOOR_Y0, K.DOOR_Y1), (K.HALF_D - rear, K.HALF_D)), "cabinet", "cabinet-sheet", ["-w"]))
    roof0 = {"50": K.TOP_Y1, "100": K.TOP_Y1, "200": 1.94, "500": 1.925}[scale]
    out.append(th(K.box("th-roof", "frame", STEEL, (-K.HALF_W, K.HALF_W), (roof0, K.ROOF_Y1), (-zf, zf)), "cabinet", "cabinet-sheet", ["-u"]))
    if scale in ("50", "100"):
        out.append(th(K.box("th-base", "frame", STEEL, (-K.POST_X1, K.POST_X1), (K.BASE_Y0, K.BASE_Y1), (-zf, zf)), "cabinet", "cabinet-sheet", ["+u"]))
    elif scale == "200":
        out.append(th(K.box("th-base", "frame", STEEL, (-0.27, 0.27), (K.BASE_Y0, K.BASE_Y1 + 0.02), (-zf, zf)), "cabinet", "cabinet-sheet", ["+u"]))
    else:
        out.append(th(K.box("th-base", "frame", STEEL, (-K.HALF_W, K.HALF_W), (0.075, 0.15), (-zf, zf)), "cabinet", "cabinet-sheet", ["+u"]))
    return out


def front_door(scale):
    """🚪 The perforated door as its sheet: face and returns, the back open to the closed cabinet."""
    depth = {"50": K.DOOR_DEPTH, "100": K.DOOR_DEPTH, "200": 0.03, "500": 0.075}[scale]
    x = K.DOOR_X
    return [th(K.box("th-door", "door", MESH, (-x, x), (K.DOOR_Y0, K.DOOR_Y1), (-K.HALF_D, -K.HALF_D + depth)), "door", "door-sheet", ["+w"])]


def emitter(scale):
    """💨 The outlet of the warm air: the two roof fans, flush with the roof sheet in its openings (at 1:200 and 1:500, where the roof is drawn closed, one patch over the fan zone flush with the roof)."""
    t = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}[scale]
    if scale in ("200", "500"):
        solid = K.box("th-exhaust-air", "frame", STEEL, (-0.12, 0.12), (K.ROOF_Y1 - t, K.ROOF_Y1), (0.0, 0.42), tone=0.4, role="detail")
    else:
        solid = K.repeat(K.turned("th-exhaust-air", "frame", STEEL, [(0, K.FAN_R), (t, K.FAN_R)], "y", (0, K.ROOF_Y1 - t, K.FAN_Z[0]), tone=0.4, role="detail"), 2, dz=K.FAN_Z[1] - K.FAN_Z[0])
    solid["tags"] = ["exhaust-air"]
    return [th(solid, "cabinet")]


def thermal(scale, body):
    """🌡 The thermal list of a scale."""
    feet = [ref(f"th-{s['id']}", s["id"], "feet") for s in body if s["bone"] == "feet"]
    for solid in feet:
        if "plate" in solid["of"]:
            solid["sectionFaces"] = ["+u"]
    return cabinet(scale) + front_door(scale) + fronts(scale) + feet + emitter(scale)


def equipment_masses(scale):
    """⚖ The equipment mass as slabs inside the chassis that the rails carry: the storage array, each server, the top group."""
    out = []
    if scale in ("50", "100"):
        s0, s1 = K.unit_y(1, 4)
        out.append(st(K.box("mass-storage", "equipment", CHASSIS, (-0.2425, 0.2425), (s0 + 0.04, s1 - 0.04), (-0.358, 0.318), tone=0.15, role="detail"), "equipment", "mass"))
        v0, v1 = K.unit_y(5, 2)
        out.append(st(K.repeat(K.box("mass-server", "equipment", CHASSIS, (-0.2425, 0.2425), (v0 + 0.02, v1 - 0.02), (-0.358, 0.318), tone=0.15, role="detail"), 6, dy=2 * K.U), "equipment", "mass"))
        t0, t1 = K.unit_y(42, 1)
        out.append(st(K.box("mass-switch", "equipment", CHASSIS, (-0.2425, 0.2425), (t0 + 0.0125, t1 - 0.0125), (-0.358, -0.04), tone=0.15, role="detail"), "equipment", "mass"))
    else:
        f = FEATURE[scale]
        h = 0.2145 if scale == "200" else 0.125
        s0, s1 = K.RAIL_Y0, K.unit_y(15, 2)[1]
        out.append(st(K.box("mass-server-block", "equipment", CHASSIS, (-h, h), (s0 + f, s1 - f), (-0.30, 0.15), tone=0.15, role="detail"), "equipment", "mass"))
        t0, t1 = K.unit_y(40, 1)[0], K.RAIL_Y1
        out.append(st(K.box("mass-top-group", "equipment", CHASSIS, (-h, h), (t0 + 0.01, max(t1 - 0.01, t0 + 0.01 + f)), (-0.30, -0.05), tone=0.15, role="detail"), "equipment", "mass"))
    return out


def support(scale):
    """🛞 The contact of the four casters: a pad ring through the four wheel contacts on the floor."""
    t = max(FEATURE[scale], 0.008)
    outline, hole = K.rect(-0.27, 0.27, -0.47, 0.47), [list(reversed(K.rect(-0.23, 0.23, -0.43, 0.43)))]
    return [st(K.up("support-base-contact", "feet", RUBBER, outline, (-t, 0.0), holes=hole, tone=0.1, role="ground"), "feet", "support", "base-contact")]


def structure(scale, body):
    """🏗 The load path of a scale: frame, rails, door leaf, casters, joints, support and masses."""
    ids = {s["id"] for s in body}
    out = []
    if scale in ("50", "100"):
        for sid in ("post-a", "post-b", "base-frame", "top-frame", "depth-rail-a", "depth-rail-b", "side-mullion"):
            if sid in ids:
                out.append(sref(f"member-{sid}", sid, "frame", "member"))
        for sid in ("rail-front-a", "rail-front-b", "rail-rear-a", "rail-rear-b"):
            out.append(sref(f"member-{sid}", sid, "equipment", "member"))
        out.append(sref("member-door", "door-return", "door", "member"))
        for s in body:
            if s["bone"] == "feet" and "wheel" not in s["id"]:
                out.append(sref(f"member-{s['id']}", s["id"], "feet", "member"))
        n = 0.044 if scale == "50" else 0.05
        for side, x in (("a", -0.2625), ("b", 0.2625)):
            for level, y in (("low", 0.085), ("high", 1.952)):
                out.append(st(K.repeat(K.box(f"joint-corner-{side}-{level}", "frame", STEEL, (x - n / 2, x + n / 2), (y - n / 2, y + n / 2), (-0.455 - n / 2, -0.455 + n / 2), tone=0.6, role="detail"), 2, dz=0.91), "frame", "joint"))
        if scale == "50":
            for tag, z in (("front", K.FRONT_FLANGE_Z + 0.021), ("rear", K.REAR_FLANGE_Z - 0.021)):
                bolt = K.turned(f"joint-rail-{tag}", "equipment", STEEL, [(0, 0.006), (0.024, 0.006)], "x", (-0.256, 0.115, z), tone=0.6, role="detail")
                out.append(st(K.repeat(bolt, 2, dy=1.807), "equipment", "joint"))
                bolt = K.turned(f"joint-rail-{tag}-b", "equipment", STEEL, [(0, 0.006), (0.024, 0.006)], "x", (0.232, 0.115, z), tone=0.6, role="detail")
                out.append(st(K.repeat(bolt, 2, dy=1.807), "equipment", "joint"))
            out.append(sref("joint-door-hinge", "door-hinge", "door", "joint"))
        else:
            out.append(st(K.repeat(K.box("joint-door-hinge", "door", MESH, (-0.285, -0.27), (0.27, 0.33), (-0.495, -0.48), tone=0.6, role="detail"), 3, dy=0.72), "door", "joint"))
    elif scale == "200":
        outline, hole = K.rect(-0.27, 0.27, -0.475, 0.475), [list(reversed(K.rect(-0.23, 0.23, -0.435, 0.435)))]
        out.append(sref("member-base-frame", "base-frame", "frame", "member"))
        out.append(st(K.up("member-top-frame", "frame", STEEL, outline, (1.91, 1.94), holes=hole, tone=0.5), "frame", "member"))
        for side, (xa, xb) in (("a", (-0.27, -0.24)), ("b", (0.24, 0.27))):
            out.append(st(K.repeat(K.box(f"member-post-{side}", "frame", STEEL, (xa, xb), (0.12, 1.91), (-0.475, -0.445), tone=0.5), 2, dz=0.92), "frame", "member"))
        for tag, z in (("front", (-0.43, -0.40)), ("rear", (0.33, 0.36))):
            for side, (xa, xb) in (("a", (-0.2445, -0.2145)), ("b", (0.2145, 0.2445))):
                out.append(st(K.box(f"member-rail-{tag}-{side}", "equipment", STEEL, (xa, xb), (K.RAIL_Y0 + 0.02, K.RAIL_Y1 - 0.03), z, tone=0.5), "equipment", "member"))
        out.append(sref("member-door", "door-return", "door", "member"))
        for s in body:
            if s["bone"] == "feet":
                out.append(sref(f"member-{s['id']}", s["id"], "feet", "member"))
    else:
        f = 0.075
        outline, hole = K.rect(-0.3, 0.3, -0.5, 0.5), [list(reversed(K.rect(-0.225, 0.225, -0.425, 0.425)))]
        out.append(st(K.up("member-base-frame", "frame", STEEL, outline, (0.075, 0.15), holes=hole, tone=0.5), "frame", "member"))
        out.append(st(K.up("member-top-frame", "frame", STEEL, outline, (1.925, K.HEIGHT), holes=hole, tone=0.5), "frame", "member"))
        for side, (xa, xb) in (("a", (-0.3, -0.225)), ("b", (0.225, 0.3))):
            out.append(st(K.repeat(K.box(f"member-post-{side}", "frame", STEEL, (xa, xb), (0.15, 1.925), (0.425, 0.5), tone=0.5), 2, dz=-0.925), "frame", "member"))
        for side, (xa, xb) in (("a", (-0.2, -0.125)), ("b", (0.125, 0.2))):
            out.append(st(K.box(f"member-rail-{side}", "equipment", STEEL, (xa, xb), (0.15, 1.92), (-0.40, -0.325), tone=0.5), "equipment", "member"))
        out.append(st(K.deep("member-door", "door", MESH, K.rect(-0.27, 0.27, 0.15, 1.925), (-0.5, -0.425), holes=[list(reversed(K.rect(-0.195, 0.195, 0.225, 1.85)))], tone=0.5), "door", "member"))
        out.append(sref("member-casters", "casters", "feet", "member"))
    r = {"50": 0.04, "100": 0.05, "200": 0.06, "500": 0.08}[scale]
    out.append(st(K.blob("mass-frame", "frame", STEEL, (0, 1.0, 0), (r, r, r), tone=0.15, role="detail"), "frame", "mass"))
    out += equipment_masses(scale)
    if scale in ("50", "100"):
        out.append(st(K.box("mass-door", "door", MESH, (-0.15, 0.15), (0.88, 1.18), (-0.4925, -0.4775), tone=0.15, role="detail"), "door", "mass"))
    return out + support(scale)


DECISIONS = {
    "50": [
        {"en": "Body: the cabinet as built: welded base and top frames of 40 x 30 mm, four 35 x 40 mm corner posts, depth rails and the four 19-inch mounting angles (2 mm sheet), side panels as folded trays of 1.2 mm sheet with 20 mm returns and a vertical joint, louvres pressed into the rear of the right panel, a roof with its 33 mm skirt, two fan units with hubs and guards and a brush cable entry, two 0U power strips at the rear; a front door of folded sheet whose face carries 20 mm holes at 28 mm pitch (the real 5 mm perforation, 70 percent open, generalised to webs the scale can print) with swing handle, lock and three hinges, a split rear door perforated with the same holes offset by half a pitch; the storage array, six 2U servers, patch panel, cable brush and switch with their fronts, ears, handles, drive bays, grilles, ports and LEDs; four 50 mm swivel casters with plate, race, fork and wheel. Roof sheet and door faces are 7.5 mm (the smallest feature) instead of 1.5 mm.", "de": "Körper: der Schrank, wie er gebaut ist: geschweißter Sockel- und Kopfrahmen aus 40 x 30 mm, vier Eckpfosten 35 x 40 mm, Tiefenstreben und die vier 19-Zoll-Montagewinkel (2 mm Blech), Seitenwände als gekantete Wannen aus 1,2-mm-Blech mit 20 mm Kantung und senkrechter Fuge, Lüftungskiemen hinten in der rechten Wand, ein Dach mit 33-mm-Rand, zwei Lüftereinheiten mit Nabe und Gitter und eine Bürsten-Kabeleinführung, zwei 0-HE-Steckdosenleisten hinten; eine Fronttür aus gekantetem Blech, deren Fläche 20-mm-Löcher im Raster 28 mm trägt (die echte 5-mm-Lochung mit 70 Prozent Öffnung, auf druckbare Stege verallgemeinert), mit Schwenkgriff, Schloss und drei Scharnieren, eine geteilte Rücktür mit denselben Löchern um ein halbes Raster versetzt; Speicher, sechs 2-HE-Server, Patchfeld, Kabelbürste und Switch mit Fronten, Winkeln, Griffen, Laufwerksschächten, Gittern, Ports und LEDs; vier 50-mm-Lenkrollen mit Platte, Drehkranz, Gabel und Rad. Dachblech und Türflächen sind 7,5 mm (das kleinste Merkmal) statt 1,5 mm."},
        {"en": "Thermal: the cabinet sheets as the 1.2 mm outer skins of side panels, roof, rear door and base frame and the door as its leaf, their inner faces bounding the sealed airway of the closed cabinet; server, switch and blank fronts as 2 and 1.5 mm skins on the front plates; the casters as their own solids; the warm air leaves through the two roof fans. Structure: corner posts, base and top frames, depth rails, the four mounting angles, the door leaf and the caster forks as members, welded corners, rail bolts and door hinges as joints, the contact under the four wheels as support, the equipment mass as a slab in each chassis on the rails, the cabinet and door masses at their centres.", "de": "Thermik: die Schrankbleche als 1,2 mm starke Außenhäute von Seitenwänden, Dach, Rücktür und Sockelrahmen und die Tür als ihr Flügel, die Innenflächen begrenzen den geschlossenen Luftraum des Schranks; Server-, Switch- und Blendenfronten als 2 und 1,5 mm starke Häute auf den Frontplatten; die Rollen als eigene Körper; die warme Luft tritt durch die zwei Dachlüfter aus. Tragwerk: Eckpfosten, Sockel- und Kopfrahmen, Tiefenstreben, die vier Montagewinkel, der Türflügel und die Rollengabeln als Stäbe, geschweißte Ecken, Schienenschrauben und Türscharniere als Gelenke, der Kontakt unter den vier Rädern als Lager, die Gerätemasse als Platte in jedem Gehäuse auf den Schienen, die Massen von Schrank und Tür in ihren Schwerpunkten."},
    ],
    "100": [
        {"en": "Body: modelled anew for 15 mm: one folded tray per side panel without joint or louvres, rings, posts and depth rails as plain bars, the 19-inch angles as 22 mm bars, roof sheet 15 mm with the fan openings and cable slot over plain fan frames, the front door face with 30 mm holes at 45 mm pitch and one handle block, the rear door as two plain leaves, the storage array and each server as front and chassis, patch panel, brush and switch as one top group, the casters as plate, fork and wheel; bays, grilles, ports, LEDs, hinges, guards and power strips are below the scale.", "de": "Körper: neu für 15 mm gebaut: eine gekantete Wanne je Seitenwand ohne Fuge und Kiemen, Rahmen, Pfosten und Tiefenstreben als einfache Stäbe, die 19-Zoll-Winkel als 22-mm-Stäbe, Dachblech 15 mm mit Lüfteröffnungen und Kabelschlitz über einfachen Lüfterrahmen, die Fronttür mit 30-mm-Löchern im Raster 45 mm und einem Griffblock, die Rücktür als zwei glatte Flügel, Speicher und jeder Server als Front und Gehäuse, Patchfeld, Bürste und Switch als eine Obergruppe, die Rollen als Platte, Gabel und Rad; Schächte, Gitter, Ports, LEDs, Scharniere, Schutzgitter und Steckdosenleisten liegen unter dem Maßstab."},
        {"en": "Thermal: side, roof, rear and base skins, the door leaf, one front skin each for the servers, the blanks and the switch, the caster solids and the two fan outlets. Structure: posts, rings, depth rails, angles, door leaf and caster forks as members, the eight frame corners and three hinges as joints, the wheel contact ring, a mass slab per unit and the cabinet and door masses.", "de": "Thermik: Seiten-, Dach-, Rück- und Sockelhaut, der Türflügel, je eine Fronthaut für die Server, die Blenden und den Switch, die Rollenkörper und die zwei Lüfterauslässe. Tragwerk: Pfosten, Rahmen, Tiefenstreben, Winkel, Türflügel und Rollengabeln als Stäbe, die acht Rahmenecken und drei Scharniere als Gelenke, der Radkontaktring, eine Massenplatte je Gerät und die Massen von Schrank und Tür."},
    ],
    "200": [
        {"en": "Body: a carcass for 30 mm: side walls, roof and rear door as 30 mm plates on a base frame, the door as a 30 mm frame around a face with 30 mm holes at 60 mm pitch through which the equipment shows, the storage array and servers merged into one server block, patch panel, brush and switch into one top group, the casters as four blocks; posts, angles, fans and handles are inside or below the scale.", "de": "Körper: ein Korpus für 30 mm: Seitenwände, Dach und Rücktür als 30-mm-Platten auf einem Sockelrahmen, die Tür als 30-mm-Rahmen um eine Fläche mit 30-mm-Löchern im Raster 60 mm, durch die die Geräte sichtbar sind, Speicher und Server zu einem Serverblock, Patchfeld, Bürste und Switch zu einer Obergruppe zusammengefasst, die Rollen als vier Blöcke; Pfosten, Winkel, Lüfter und Griffe liegen innen oder unter dem Maßstab."},
        {"en": "Thermal: 30 mm skins of side walls, roof, rear door and base, the door plate, front skins of the server block, the blanks and the switch, the caster blocks, the fan outlets. Structure: four 30 mm posts between the base frame and a top ring, four mounting angles, the door frame and the casters as members, the contact ring, the masses of the cabinet, the server block and the top group.", "de": "Thermik: 30-mm-Häute von Seitenwänden, Dach, Rücktür und Sockel, die Türplatte, Fronthäute von Serverblock, Blenden und Switch, die Rollenblöcke, die Lüfterauslässe. Tragwerk: vier 30-mm-Pfosten zwischen Sockelrahmen und Kopfring, vier Montagewinkel, der Türrahmen und die Rollen als Stäbe, der Kontaktring, die Massen von Schrank, Serverblock und Obergruppe."},
    ],
    "500": [
        {"en": "Body: the cabinet as one block on a band of casters (70 mm, below the 75 mm feature), the server block and the top group seen in two windows of the front where the perforated door shows them, 10 mm behind the door plane.", "de": "Körper: der Schrank als ein Block auf einem Rollenband (70 mm, unter dem Merkmal von 75 mm), Serverblock und Obergruppe in zwei Fenstern der Front, wo die gelochte Tür sie zeigt, 10 mm hinter der Türebene."},
        {"en": "Thermal: 75 mm skins of the cabinet, the door plate in the front, front skins of the two equipment groups, the caster band, one fan outlet patch on the roof. Structure: four 75 mm posts between a base and a top ring, two mounting angles, the door frame, the caster band, the contact ring and the masses of cabinet and equipment.", "de": "Thermik: 75-mm-Häute des Schranks, die Türplatte in der Front, Fronthäute der beiden Gerätegruppen, das Rollenband, eine Lüfterauslassfläche auf dem Dach. Tragwerk: vier 75-mm-Pfosten zwischen einem Sockel- und einem Kopfring, zwei Montagewinkel, der Türrahmen, das Rollenband, der Kontaktring und die Massen von Schrank und Geräten."},
    ],
}


def decisions(scale):
    """📝 The decisions of a scale (en, de)."""
    return DECISIONS[scale]
