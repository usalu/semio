"""🪟 Every solid of components/tilt-window at 1:50, 1:100 and 1:200, stated part by part; writes volumes/components/tilt-window.json."""

import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-tilt-window-kit.py"))
k = importlib.util.module_from_spec(spec)
spec.loader.exec_module(k)
r = k.r
OUT = os.path.join(T, "volumes", "components", "tilt-window.json")


def tx(en, de):
    return {"en": en, "de": de}


def base(id, kind, bone, material, role, tone):
    return {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def box(id, bone, material, role, tone, lo, hi, repeat=None, **extra):
    s = base(id, "box", bone, material, role, tone)
    s["size"] = [r(hi[i] - lo[i]) for i in range(3)]
    s["transform"] = {"translation": [r((hi[i] + lo[i]) / 2) for i in range(3)]}
    if repeat is not None:
        s["repeat"] = {"count": 2, "translation": [0, 0, r(repeat)]}
    s.update(extra)
    return s


def extrude(id, bone, material, role, tone, plane, profile, a, b, holes=None, repeat=None, **extra):
    s = base(id, "extrude", bone, material, role, tone)
    s["profile"] = [[r(p), r(q)] for p, q in profile]
    if holes:
        s["holes"] = [[[r(p), r(q)] for p, q in hole] for hole in holes]
    s["plane"] = plane
    s["from"], s["to"] = r(min(a, b)), r(max(a, b))
    if repeat is not None:
        s["repeat"] = {"count": 2, "translation": [0, 0, r(repeat)]}
    s.update(extra)
    return s


def revolve_z(id, bone, material, role, tone, centre, radius, z0, length, repeat=None, **extra):
    s = base(id, "revolve", bone, material, role, tone)
    s["profile"] = [[0, r(radius)], [r(length), r(radius)]]
    s["axis"] = [0, 0, 1]
    s["origin"] = [r(centre[0]), r(centre[1]), r(z0)]
    if repeat is not None:
        s["repeat"] = {"count": 2, "translation": [0, 0, r(repeat)]}
    s.update(extra)
    return s


def capsule(id, bone, material, role, tone, p0, p1, radius, **extra):
    s = base(id, "capsule", bone, material, role, tone)
    s.update({"p0": [r(c) for c in p0], "p1": [r(c) for c in p1], "r0": r(radius), "r1": r(radius)})
    s.update(extra)
    return s


def ellipsoid(id, bone, material, role, tone, centre, radii, repeat=None, **extra):
    s = base(id, "ellipsoid", bone, material, role, tone)
    s["radii"] = [r(x) for x in radii]
    s["transform"] = {"translation": [r(c) for c in centre]}
    if repeat is not None:
        s["repeat"] = {"count": 2, "translation": [0, 0, r(repeat)]}
    s.update(extra)
    return s


def sash_xy(points):
    return [(k.su(Y), k.sv(X)) for X, Y in points]


def sill_xy(points):
    return [(k.lu(X), k.lv(Y)) for X, Y in points]


def sash_box(id, material, role, tone, X0, X1, Y0, Y1, Z0, Z1, **extra):
    lo = (k.su(Y0), min(k.sv(X0), k.sv(X1)), Z0)
    hi = (k.su(Y1), max(k.sv(X0), k.sv(X1)), Z1)
    return box(id, "sash", material, role, tone, lo, hi, **extra)


def sill_box(id, material, role, tone, X0, X1, Y0, Y1, Z0, Z1, **extra):
    return box(id, "sill", material, role, tone, (k.lu(X0), k.lv(Y0), Z0), (k.lu(X1), k.lv(Y1), Z1), **extra)


def ring_zx(id, material, role, tone, zo, yo0, yo1, zi, yi0, yi1, X0, X1, **extra):
    outer = [(-zo, k.su(yo0)), (zo, k.su(yo0)), (zo, k.su(yo1)), (-zo, k.su(yo1))]
    hole = [(-zi, k.su(yi0)), (-zi, k.su(yi1)), (zi, k.su(yi1)), (zi, k.su(yi0))]
    return extrude(id, "sash", material, role, tone, "zx", outer, k.sv(X0), k.sv(X1), holes=[hole], **extra)


def mirror_w(points):
    return [(v, -w) for v, w in reversed(points)]


SZ, SIGHT_Z, STOP_Z = k.SASH_Z, k.SIGHT_Z, k.Z_IN - k.STOP_W
RIM = ["-u", "+u", "-w", "+w"]
SEAL_X0, SEAL_X1 = k.GLASS_X0 + k.PANE, k.LIP_X - k.PANE


def lining50():
    return [
        extrude("sill", "sill", "timber", "body", 0.5, "xy", sill_xy(k.sill_section()), -k.Z_IN, k.Z_IN),
        sill_box("jamb-left", "timber", "body", 0.55, k.X0, k.X1, 0.0, k.H, k.Z_IN, k.Z_OUT),
        sill_box("jamb-right", "timber", "body", 0.55, k.X0, k.X1, 0.0, k.H, -k.Z_OUT, -k.Z_IN),
        sill_box("head", "timber", "body", 0.55, k.X0, k.X1, k.HEAD_Y0, k.H, -k.Z_IN, k.Z_IN),
        sill_box("stop-left", "timber", "body", 0.5, k.STOP_X0, k.STOP_X1, 0.05, k.HEAD_Y0, STOP_Z, k.Z_IN),
        sill_box("stop-right", "timber", "body", 0.5, k.STOP_X0, k.STOP_X1, 0.05, k.HEAD_Y0, -k.Z_IN, -STOP_Z),
        sill_box("stop-sill", "timber", "body", 0.5, k.STOP_X0, k.STOP_X1, 0.05, k.STOP_SILL_Y1, -STOP_Z, STOP_Z),
        sill_box("stop-head", "timber", "body", 0.5, k.STOP_X0, k.STOP_X1, k.STOP_HEAD_Y0, k.HEAD_Y0, -STOP_Z, STOP_Z),
        sill_box("hinge-leaf-frame", "steel", "detail", 0.25, k.HX - 0.06, k.HX, 0.05, 0.05 + k.LEAF, k.HINGE_Z - k.HINGE_L / 2, k.HINGE_Z + k.HINGE_L / 2, repeat=-2 * k.HINGE_Z),
        extrude("stay-arc", "sill", "steel", "detail", 0.3, "xy", sill_xy(k.arc_bar()), k.ARC_Z0, k.ARC_Z1, holes=[sill_xy(k.arc_slot())], repeat=-(k.ARC_Z0 + k.ARC_Z1)),
    ]


def sash50():
    stile = [(k.sv(X), Z) for X, Z in k.stile_section()]
    bead = [(k.sv(X), Z) for X, Z in [(k.SASH_X0, SIGHT_Z + k.REBATE), (k.SASH_X0, SIGHT_Z + k.CHAMFER), (k.SASH_X0 + k.CHAMFER, SIGHT_Z), (k.BEAD_X1, SIGHT_Z), (k.BEAD_X1, SIGHT_Z + k.REBATE)]]
    rail_w = SIGHT_Z + k.REBATE
    gy0, gy1 = k.GLASS_Y0, k.GLASS_Y1
    return [
        extrude("stile-left", "sash", "timber", "body", 0.6, "yz", stile, k.su(k.SASH_Y0), k.su(k.SASH_Y1)),
        extrude("stile-right", "sash", "timber", "body", 0.6, "yz", mirror_w(stile), k.su(k.SASH_Y0), k.su(k.SASH_Y1)),
        extrude("rail-bottom", "sash", "timber", "body", 0.6, "xy", sash_xy(k.rail_section(k.SASH_Y0, k.SIGHT_Y0, True)), -rail_w, rail_w),
        extrude("rail-top", "sash", "timber", "body", 0.6, "xy", sash_xy(k.rail_section(k.SIGHT_Y1, k.SASH_Y1, False)), -rail_w, rail_w),
        extrude("bead-bottom", "sash", "timber", "body", 0.62, "xy", sash_xy(k.bead_section(False)), -rail_w, rail_w),
        extrude("bead-top", "sash", "timber", "body", 0.62, "xy", sash_xy(k.bead_section(True)), -rail_w, rail_w),
        extrude("bead-left", "sash", "timber", "body", 0.62, "yz", bead, k.su(k.SIGHT_Y0), k.su(k.SIGHT_Y1)),
        extrude("bead-right", "sash", "timber", "body", 0.62, "yz", mirror_w(bead), k.su(k.SIGHT_Y0), k.su(k.SIGHT_Y1)),
        extrude("weather-bar", "sash", "timber", "body", 0.55, "xy", sash_xy(k.WEATHER), -k.WEATHER_Z, k.WEATHER_Z),
        sash_box("pane-inner", "glass", "glazing", 0.85, k.GLASS_X0, k.GLASS_X0 + k.PANE, gy0, gy1, -k.GLASS_Z, k.GLASS_Z),
        sash_box("pane-outer", "glass", "glazing", 0.85, k.LIP_X - k.PANE, k.LIP_X, gy0, gy1, -k.GLASS_Z, k.GLASS_Z),
        ring_zx("edge-seal", "edge-zone", "body", 0.25, k.GLASS_Z, gy0, gy1, k.GLASS_Z - k.SEAL, gy0 + k.SEAL, gy1 - k.SEAL, SEAL_X0, SEAL_X1),
        revolve_z("hinge-barrel", "sash", "steel", "detail", 0.25, (0, 0), k.HINGE_R, k.HINGE_Z - k.HINGE_L / 2, k.HINGE_L, repeat=-2 * k.HINGE_Z),
        sash_box("hinge-leaf-sash", "steel", "detail", 0.25, k.HX, k.SASH_X0, k.SASH_Y0, k.SASH_Y0 + 0.06, k.HINGE_Z - k.HINGE_L / 2, k.HINGE_Z + k.HINGE_L / 2, repeat=-2 * k.HINGE_Z),
        capsule("grip-post-left", "sash", "steel", "detail", 0.3, (k.su(k.GRIP_Y), k.sv(k.SASH_X0), k.GRIP_HALF), (k.su(k.GRIP_Y), k.sv(k.SASH_X0 - k.GRIP_REACH), k.GRIP_HALF), k.GRIP_R),
        capsule("grip-post-right", "sash", "steel", "detail", 0.3, (k.su(k.GRIP_Y), k.sv(k.SASH_X0), -k.GRIP_HALF), (k.su(k.GRIP_Y), k.sv(k.SASH_X0 - k.GRIP_REACH), -k.GRIP_HALF), k.GRIP_R),
        capsule("grip-bar", "sash", "steel", "detail", 0.3, (k.su(k.GRIP_Y), k.sv(k.SASH_X0 - k.GRIP_REACH), -k.GRIP_HALF), (k.su(k.GRIP_Y), k.sv(k.SASH_X0 - k.GRIP_REACH), k.GRIP_HALF), k.GRIP_R),
        revolve_z("stay-pin", "sash", "steel", "detail", 0.2, (k.KEEPER_U, k.KEEPER_V), k.KEEPER_R, SZ, k.ARC_Z1 - SZ - 0.001, repeat=-(2 * SZ + k.ARC_Z1 - SZ - 0.001)),
    ]


def glass_layers(prefix, lo_z, lo_y0, lo_y1, hole, region, layers):
    out = []
    for layer, X0, X1, faces in layers:
        if hole is None:
            out.append(sash_box("%s-%s" % (prefix, layer), "glass" if layer.startswith("pane") else "glass-unit-cavity", "body", 0.85, X0, X1, lo_y0, lo_y1, -lo_z, lo_z, sectionFaces=faces, region=region, layer=layer))
        else:
            zi, yi0, yi1 = hole
            out.append(ring_zx("%s-%s" % (prefix, layer), "glass" if layer.startswith("pane") else "edge-zone", "body", 0.85 if layer.startswith("pane") else 0.4, lo_z, lo_y0, lo_y1, zi, yi0, yi1, X0, X1, sectionFaces=faces, region=region, layer=layer))
    return out


def glass_thermal(layered):
    centre_layers = [("pane-inner", k.GLASS_X0, k.GLASS_X0 + k.PANE, RIM + ["+v"]), ("cavity", SEAL_X0, SEAL_X1, RIM + ["+v"]), ("pane-outer", k.LIP_X - k.PANE, k.LIP_X, RIM + ["+v"])]
    edge_layers = [("pane-inner", k.GLASS_X0, k.GLASS_X0 + k.PANE, RIM + ["+v"]), ("edge-body", SEAL_X0, SEAL_X1, RIM + ["+v"]), ("pane-outer", k.LIP_X - k.PANE, k.LIP_X, RIM + ["+v"])]
    cz, cy0, cy1 = k.CENTRE_Z, k.CENTRE_Y0, k.CENTRE_Y1
    if layered:
        return glass_layers("th-centre", cz, cy0, cy1, None, "pane-glass", centre_layers) + glass_layers("th-edge", SIGHT_Z, k.SIGHT_Y0, k.SIGHT_Y1, (cz, cy0, cy1), "pane-edge", edge_layers)
    if layered is None:
        return [ref("th-pane", "pane", "pane-glass", RIM + ["+v"]), ref("th-pane-edge", "pane", "pane-edge", RIM + ["-v", "+v"])]
    return [
        sash_box("th-centre", "glass", "body", 0.85, k.GLASS_X0, k.LIP_X, cy0, cy1, -cz, cz, sectionFaces=RIM + ["+v"], region="pane-glass"),
        ring_zx("th-edge", "edge-zone", "body", 0.4, SIGHT_Z, k.SIGHT_Y0, k.SIGHT_Y1, cz, cy0, cy1, k.GLASS_X0, k.LIP_X, sectionFaces=RIM + ["+v"], region="pane-edge"),
    ]


def lining_refs():
    """🧱 Lining boards: the room-side end face is conditioned interior (C25 c); jamb backs and the head top lie against the masonry reveal (C25 b); the sill bottom stays counted as the area reviewers decided (R-C, area-review.md)."""
    return [ref("th-jamb-left", "jamb-left", "frame-timber", ["-u", "+w"]), ref("th-jamb-right", "jamb-right", "frame-timber", ["-u", "-w"]), ref("th-head", "head", "frame-timber", ["-u", "+v"]), ref("th-sill", "sill", "frame-sill", ["-u"])]


def ref(id, of, region, faces=None):
    s = {"id": id, "of": of, "region": region}
    if faces:
        s["sectionFaces"] = faces
    return s


def member(id, of, bone, part="member", anchor=None):
    s = {"id": id, "of": of, "member": bone, "part": part}
    if anchor is not None:
        s["anchor"] = anchor
    return s


def structural(solid, bone, part, anchor=None):
    solid["member"] = bone
    solid["part"] = part
    if anchor is not None:
        solid["anchor"] = anchor
    return solid


def mass_solid(radius):
    kg, cu, cv, _ = k.sash_mass()
    return structural(ellipsoid("mass-sash", "sash", "steel", "detail", 0.2, (cu, cv, 0), (radius, radius, radius)), "sash", "mass")


def support_pin(radius, overhang):
    return structural(revolve_z("support-hinge", "sash", "steel", "detail", 0.2, (0, 0), radius, k.HINGE_Z - k.HINGE_L / 2 - overhang, k.HINGE_L + 2 * overhang, repeat=-2 * k.HINGE_Z), "sash", "support", "hinge")


def scale50():
    body = lining50() + sash50()
    timber_sash = ["stile-left", "stile-right", "rail-bottom", "rail-top", "bead-bottom", "bead-top", "bead-left", "bead-right"]
    thermal = glass_thermal(True)
    thermal += [ref("th-" + i, i, "frame-timber", ["+v"]) for i in timber_sash]
    thermal += [ref("th-weather-bar", "weather-bar", "frame-timber")]
    thermal += lining_refs()
    thermal += [ref("th-" + i, i, "frame-timber") for i in ("stop-left", "stop-right", "stop-sill", "stop-head")]
    structure = [member("m-" + i, i, "sash") for i in ("stile-left", "stile-right", "rail-bottom", "rail-top")]
    structure += [member("m-" + i, i, "sill") for i in ("sill", "jamb-left", "jamb-right", "head", "stay-arc")]
    structure += [member("j-hinge", "hinge-barrel", "sash", "joint"), member("j-stay-pin", "stay-pin", "sash", "joint"), support_pin(0.006, 0.015), mass_solid(0.03)]
    decisions = [
        tx("Body: the window as built in its closed default pose. Lining frame of 50 mm softwood boards 300 mm deep: the sill board with a 10 mm weathered fall outside the stop and an 8 x 10 mm drip groove, two jambs running to the floor line, the head board, and four 25 x 30 mm stop strips on the exterior side of the sash. Sash 776 x 1200 x 90 mm with 12 mm rebate clearance all round: stiles 70 mm with the room-side glass rebate (18 mm) and the 12 x 10 mm hardware groove on their outer faces, top rail 70 mm and bottom rail 80 mm with their rebates, four glazing beads 26 x 18 mm with an 8 mm chamfer, the weather bar 38 mm proud with a sloped top and drip nose.", "Körper: das Fenster wie gebaut in seiner geschlossenen Standardpose. Futterrahmen aus 50-mm-Nadelholzbrettern, 300 mm tief: das Sohlbrett mit 10 mm Wetterschräge außerhalb des Anschlags und einer Tropfnut 8 x 10 mm, zwei Seitenbretter bis zur Bodenlinie, das Kopfbrett und vier Anschlagleisten 25 x 30 mm auf der Außenseite des Flügels. Flügel 776 x 1200 x 90 mm mit 12 mm Falzluft rundum: Flügelhölzer 70 mm mit raumseitigem Glasfalz (18 mm) und der Beschlagnut 12 x 10 mm an der Außenkante, oberer Riegel 70 mm und unterer Riegel 80 mm mit ihren Falzen, vier Glasleisten 26 x 18 mm mit 8 mm Fase, der Wetterschenkel 38 mm vorstehend mit schräger Oberseite und Tropfnase."),
        tx("Body, glass and hardware: the insulating unit as two real 4 mm panes 664 x 1078 mm let 14 mm into the rebates, with the 16 x 12 mm edge seal between them (the gas cavity is air in the body); two tilt hinges at the room-side bottom corner (16 mm barrel on the hinge axis, 8 mm sash and frame leaves); the pull grip on the top rail (12 mm bar on two posts); two quadrant stays screwed to the jambs, 24 x 8 mm steel arcs concentric with the hinge axis with a 10 mm slot, in which an 8 mm pin on each stile runs to the stop at 6 degrees. The spring catch is mortised in the top rail and not visible.", "Körper, Glas und Beschläge: die Isolierverglasung als zwei echte 4-mm-Scheiben 664 x 1078 mm, 14 mm in die Falze eingelassen, mit dem Randverbund 16 x 12 mm dazwischen (der Gasraum ist im Körper Luft); zwei Kippbänder an der raumseitigen unteren Ecke (16-mm-Rolle auf der Bandachse, 8-mm-Lappen an Flügel und Futter); der Ziehgriff am oberen Riegel (12-mm-Stange auf zwei Stützen); zwei an die Seitenbretter geschraubte Kippbögen, 24 x 8 mm Stahlbögen konzentrisch zur Bandachse mit 10 mm Schlitz, in dem ein 8-mm-Zapfen an jedem Flügelholz bis zum Anschlag bei 6 Grad läuft. Der Schnäpper ist im oberen Riegel eingelassen und nicht sichtbar."),
        tx("Thermal: glass centre and the 65 mm edge zone of the visible glass as three real shells each (inner pane, gas or edge body, outer pane; rims are section faces, the room-side face of the inner pane is the conditioned interior face); sash timber, beads, weather bar, lining boards and stops reference the body with their room-side faces marked as conditioned interior faces; the sill board references the body with its room-side end face marked.", "Thermik: Glasmitte und die 65-mm-Randzone des sichtbaren Glases als je drei echte Schalen (innere Scheibe, Gas oder Randkörper, äußere Scheibe; Ränder sind Schnittflächen, die raumseitige Fläche der inneren Scheibe ist die konditionierte Innenfläche); Flügelholz, Glasleisten, Wetterschenkel, Futterbretter und Anschläge verweisen auf den Körper mit markierten raumseitigen Flächen; das Sohlbrett verweist auf den Körper mit markierter raumseitiger Stirnfläche."),
        tx("Structure: the two stiles and two rails are the sash members, sill, jambs, head and the quadrant stays the members of the lining frame; joints are the hinge barrels on the tilt axis and the stay pins; the support of the hinge anchor is the 12 mm steel pin through each hinge barrel, standing 15 mm out at both ends; the sash mass sits at its centre of mass.", "Tragwerk: die zwei Flügelhölzer und zwei Riegel sind die Stäbe des Flügels, Sohl-, Seiten- und Kopfbrett und die Kippbögen die Stäbe des Futterrahmens; Gelenke sind die Bandrollen auf der Kippachse und die Zapfen der Kippbögen; das Auflager des Bandankers ist der 12-mm-Stahlstift durch jede Bandrolle, an beiden Enden 15 mm vorstehend; die Flügelmasse sitzt in ihrem Schwerpunkt."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale100():
    ring_outer = [(0.0, k.Z_IN), (k.lv(k.HEAD_Y0), k.Z_IN), (k.lv(k.HEAD_Y0), -k.Z_IN), (0.0, -k.Z_IN)]
    ring_hole = [(k.lv(k.STOP_SILL_Y1), STOP_Z), (k.lv(k.STOP_SILL_Y1), -STOP_Z), (k.lv(k.STOP_HEAD_Y0), -STOP_Z), (k.lv(k.STOP_HEAD_Y0), STOP_Z)]
    body = [
        sill_box("sill", "timber", "body", 0.5, k.X0, k.X1, 0.0, 0.05, -k.Z_IN, k.Z_IN),
        sill_box("jamb-left", "timber", "body", 0.55, k.X0, k.X1, 0.0, k.H, k.Z_IN, k.Z_OUT),
        sill_box("jamb-right", "timber", "body", 0.55, k.X0, k.X1, 0.0, k.H, -k.Z_OUT, -k.Z_IN),
        sill_box("head", "timber", "body", 0.55, k.X0, k.X1, k.HEAD_Y0, k.H, -k.Z_IN, k.Z_IN),
        extrude("stop-ring", "sill", "timber", "body", 0.5, "yz", ring_outer, k.lu(k.STOP_X0), k.lu(k.STOP_X1), holes=[ring_hole]),
        sash_box("stile-left", "timber", "body", 0.6, k.SASH_X0, k.SASH_X1, k.SASH_Y0, k.SASH_Y1, SIGHT_Z, SZ),
        sash_box("stile-right", "timber", "body", 0.6, k.SASH_X0, k.SASH_X1, k.SASH_Y0, k.SASH_Y1, -SZ, -SIGHT_Z),
        sash_box("rail-bottom", "timber", "body", 0.6, k.SASH_X0, k.SASH_X1, k.SASH_Y0, k.SIGHT_Y0, -SIGHT_Z, SIGHT_Z),
        sash_box("rail-top", "timber", "body", 0.6, k.SASH_X0, k.SASH_X1, k.SIGHT_Y1, k.SASH_Y1, -SIGHT_Z, SIGHT_Z),
        sash_box("weather-bar", "timber", "body", 0.55, k.SASH_X1, 0.083, k.STOP_SILL_Y1, k.SIGHT_Y0, -k.WEATHER_Z, k.WEATHER_Z),
        sash_box("pane", "glass", "glazing", 0.85, k.GLASS_X0, k.LIP_X, k.SIGHT_Y0, k.SIGHT_Y1, -SIGHT_Z, SIGHT_Z),
        revolve_z("hinge-barrel", "sash", "steel", "detail", 0.25, (0, 0), k.HINGE_R, k.HINGE_Z - k.HINGE_L / 2, k.HINGE_L, repeat=-2 * k.HINGE_Z),
    ]
    thermal = glass_thermal(False)
    thermal += [ref("th-" + i, i, "frame-timber", ["+v"]) for i in ("stile-left", "stile-right", "rail-bottom", "rail-top")]
    thermal += [ref("th-weather-bar", "weather-bar", "frame-timber"), ref("th-stop-ring", "stop-ring", "frame-timber")]
    thermal += lining_refs()
    structure = [member("m-" + i, i, "sash") for i in ("stile-left", "stile-right", "rail-bottom", "rail-top")]
    structure += [member("m-" + i, i, "sill") for i in ("sill", "jamb-left", "jamb-right", "head")]
    structure += [member("j-hinge", "hinge-barrel", "sash", "joint")]
    structure += [structural(revolve_z("j-stay", "sash", "steel", "detail", 0.2, (k.KEEPER_U, k.KEEPER_V), 0.0075, k.Z_IN - 0.015, 0.015, repeat=-(2 * k.Z_IN - 0.015)), "sash", "joint")]
    structure += [support_pin(0.0075, 0.01), mass_solid(0.03)]
    decisions = [
        tx("Body: the same window simplified for 1:100. Lining as four plain boards (the 10 mm sill fall and the drip groove are below 15 mm and dropped) with the four stop strips merged into one stop ring; the sash as two plain 70 x 90 mm stiles and the 80 and 70 mm rails with rebates and beads merged into them, the weather bar as a 38 x 62 mm block; the glass as one 24 mm slab in the sight size; the two 16 mm hinge barrels stay. Pull grip, stays, pins, leaves and the edge seal are below the 15 mm feature and dropped.", "Körper: dasselbe Fenster vereinfacht für 1:100. Futter als vier einfache Bretter (die 10-mm-Wetterschräge und die Tropfnut liegen unter 15 mm und entfallen), die vier Anschlagleisten zu einem Anschlagrahmen zusammengefasst; der Flügel als zwei einfache Flügelhölzer 70 x 90 mm und die Riegel 80 und 70 mm mit eingeschlossenen Falzen und Glasleisten, der Wetterschenkel als Block 38 x 62 mm; das Glas als eine 24-mm-Platte im Lichtmaß; die zwei 16-mm-Bandrollen bleiben. Ziehgriff, Kippbögen, Zapfen, Lappen und Randverbund liegen unter dem Merkmal von 15 mm und entfallen."),
        tx("Thermal: the three layers of the unit are merged (B2): the glass centre is one 24 mm slab and the 65 mm edge zone one 24 mm ring, rims and room-side faces marked; timber parts reference the simplified body with their room-side faces marked, the lining boards also with their faces against the masonry marked; the sill board likewise.", "Thermik: die drei Schichten der Einheit sind zusammengefasst (B2): die Glasmitte ist eine 24-mm-Platte und die 65-mm-Randzone ein 24-mm-Ring, Ränder und raumseitige Flächen markiert; die Holzteile verweisen auf den vereinfachten Körper mit markierten raumseitigen Flächen, die Futterbretter auch mit markierten Flächen am Mauerwerk; das Sohlbrett ebenso."),
        tx("Structure: stiles and rails as sash members, sill, jambs and head as lining members, the hinge barrels as joints, the stay pins as 15 mm joint marks where the stays hold the sash, the hinge support as a 15 mm pin through the barrels and the sash mass at its centre.", "Tragwerk: Flügelhölzer und Riegel als Stäbe des Flügels, Sohl-, Seiten- und Kopfbrett als Stäbe des Futters, die Bandrollen als Gelenke, die Zapfen der Kippbögen als 15-mm-Gelenkmarken, wo die Bögen den Flügel halten, das Auflager als 15-mm-Stift durch die Rollen und die Flügelmasse in ihrem Schwerpunkt."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    arch = [(k.lv(0.0), k.Z_OUT), (k.lv(k.H), k.Z_OUT), (k.lv(k.H), -k.Z_OUT), (k.lv(0.0), -k.Z_OUT), (k.lv(0.0), -k.Z_IN), (k.lv(k.HEAD_Y0), -k.Z_IN), (k.lv(k.HEAD_Y0), k.Z_IN), (k.lv(0.0), k.Z_IN)]
    body = [
        sill_box("sill", "timber", "body", 0.5, k.X0, k.X1, 0.0, 0.05, -k.Z_IN, k.Z_IN),
        extrude("lining", "sill", "timber", "body", 0.55, "yz", arch, k.lu(k.X0), k.lu(k.X1)),
        ring_zx("sash-frame", "timber", "body", 0.6, SZ, k.SASH_Y0, k.SASH_Y1, SIGHT_Z, k.SIGHT_Y0, k.SIGHT_Y1, k.SASH_X0, k.SASH_X1),
        sash_box("pane", "glass", "glazing", 0.85, k.GLASS_X0, k.LIP_X, k.SIGHT_Y0, k.SIGHT_Y1, -SIGHT_Z, SIGHT_Z),
    ]
    thermal = glass_thermal(None) + [ref("th-sash-frame", "sash-frame", "frame-timber", ["+v"]), ref("th-lining", "lining", "frame-timber", ["-u", "+v"]), ref("th-sill", "sill", "frame-sill", ["-u"])]
    structure = [member("m-sash-frame", "sash-frame", "sash"), member("m-sill", "sill", "sill"), member("m-lining", "lining", "sill")]
    structure += [structural(revolve_z("j-hinge", "sash", "steel", "detail", 0.25, (0, 0), 0.016, k.HINGE_Z - k.HINGE_L / 2, k.HINGE_L, repeat=-2 * k.HINGE_Z), "sash", "joint")]
    structure += [structural(revolve_z("j-stay", "sash", "steel", "detail", 0.2, (k.KEEPER_U, k.KEEPER_V), 0.015, SZ - 0.018, 0.03, repeat=-(2 * SZ - 0.006)), "sash", "joint")]
    structure += [support_pin(0.015, 0.01), mass_solid(0.03)]
    decisions = [
        tx("Body: the window as four masses for 1:200: the sill board, the lining arch of jambs and head in one piece (stop strips merged), the sash as one 70 mm frame ring 90 mm deep (rail and stile differences, weather bar, beads and hinges below the 30 mm feature merged or dropped) and the glass as one slab in the sight size.", "Körper: das Fenster als vier Massen für 1:200: das Sohlbrett, der Futterbogen aus Seiten- und Kopfbrettern in einem Stück (Anschlagleisten eingeschlossen), der Flügel als ein 70-mm-Rahmenring 90 mm tief (Unterschiede von Riegeln und Flügelhölzern, Wetterschenkel, Glasleisten und Bänder unter dem Merkmal von 30 mm zusammengefasst oder entfallen) und das Glas als eine Platte im Lichtmaß."),
        tx("Thermal: the glass centre and the edge zone share the one glass slab of the body as the 2D glass line does (pane-glass takes its exterior face, pane-edge references the same slab with every face marked, so only the total is compared at this scale); sash ring and lining arch reference the body with their room-side faces and the faces against the masonry marked; the sill board as before.", "Thermik: Glasmitte und Randzone teilen sich die eine Glasplatte des Körpers wie die 2D-Glaslinie (pane-glass nimmt ihre Außenfläche, pane-edge verweist auf dieselbe Platte mit allen Flächen markiert, so dass in diesem Maßstab nur die Summe verglichen wird); Flügelring und Futterbogen verweisen auf den Körper mit markierten raumseitigen Flächen und Flächen am Mauerwerk; das Sohlbrett wie zuvor."),
        tx("Structure: the sash ring, the sill and the lining arch as members, the hinge as a 32 mm joint on the tilt axis, the stays as 30 mm joint marks at the stay pins, the support as a 30 mm pin through the hinge and the sash mass at its centre.", "Tragwerk: der Flügelring, das Sohlbrett und der Futterbogen als Stäbe, das Band als 32-mm-Gelenk auf der Kippachse, die Kippbögen als 30-mm-Gelenkmarken an den Zapfen, das Auflager als 30-mm-Stift durch das Band und die Flügelmasse in ihrem Schwerpunkt."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


if __name__ == "__main__":
    volumes = {"50": scale50(), "100": scale100(), "200": scale200(), "500": {"status": "unavailable", "denominator": 500}}
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, ensure_ascii=False, indent=2)
        f.write("\n")
    print("written", OUT, {s: (len(v.get("body", [])), len(v.get("thermal", [])), len(v.get("structure", []))) for s, v in volumes.items()})
