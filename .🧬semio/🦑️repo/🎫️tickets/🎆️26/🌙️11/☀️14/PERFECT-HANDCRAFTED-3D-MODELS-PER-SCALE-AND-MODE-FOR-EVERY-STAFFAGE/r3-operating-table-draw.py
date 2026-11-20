"""✏️ Round 3, furniture/operating-table: the 2D drawings of every scale and view (ruling E1). Every shape is the true orthographic outline of one hand-stated part of that scale's model: elevation shapes in the frame of their bone, front shapes as (lateral, rest height), plan shapes as (ahead, lateral); the shapes of the tilting top are extruded solids in front and plan so they follow the tilt.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_operating_table_kit", os.path.join(HERE, "r3-operating-table-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

ORDER = {"e": 0, "f": 1, "p": 2}
BASE_Z = {"e": 2.0, "f": 2.0, "p": 1.0}


def outline(geometry, to):
    """🖊 A world outline as shape geometry; `to` maps a world point of the view to shape coordinates."""
    kind = geometry[0]
    if kind == "rect":
        _, a0, a1, b0, b1, radius = geometry
        (x0, y0), (x1, y1) = to(a0, b0), to(a1, b1)
        shape = {"kind": "rect", "x": K.r6(min(x0, x1)), "y": K.r6(min(y0, y1)), "w": K.r6(abs(x1 - x0)), "h": K.r6(abs(y1 - y0))}
        if radius:
            shape["r"] = radius
        return shape
    if kind == "ellipse":
        _, cx, cy, rx, ry = geometry
        x, y = to(cx, cy)
        return {"kind": "ellipse", "cx": K.r6(x), "cy": K.r6(y), "rx": K.r6(rx), "ry": K.r6(ry)}
    if kind == "capsule":
        _, x0, y0, x1, y1, radius = geometry
        (a0, b0), (a1, b1) = to(x0, y0), to(x1, y1)
        return {"kind": "capsule", "x0": K.r6(a0), "y0": K.r6(b0), "x1": K.r6(a1), "y1": K.r6(b1), "r0": radius, "r1": radius}
    return {"kind": "poly", "points": [list(map(K.r6, to(x, y))) for x, y in geometry[1]], "closed": True}


def shape(part, view, z, fine):
    """🔷 One shape of one part in one view."""
    bone = part["bone"]
    out = {"id": part["id"], "bone": bone, "role": part["role"] if fine else "body", "z": z, "filled": True, "tone": part["tone"]}
    if "regions" in part:
        out["regions"] = part["regions"]
    else:
        out["region"] = part["region"]
    out["material"] = part["material"]
    if view == "e" or bone == "top":
        out.update(outline(part["e"], lambda x, y: K.local(bone, x, y)))
        if view != "e":
            out["solid"] = {"kind": "extruded", "sweep": "w", "z0": K.r6(part["zr"][0]), "z1": K.r6(part["zr"][1])}
    else:
        out.update(outline(part[view], lambda a, b: (a, b)))
    return out


def geometry(scale, plist):
    """📐 The three views of one scale, each painted from the far part to the near one."""
    views = {}
    for name, view in (("elevation", "e"), ("front", "f"), ("plan", "p")):
        seen = sorted((p for p in plist if view in p["views"]), key=lambda p: p["depth"][ORDER[view]])
        views[name] = [shape(p, view, round(BASE_Z[view] + 0.05 * k, 2), scale == "50") for k, p in enumerate(seen)]
    return views


def texts(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """📝 The eight decision texts of one representation."""
    keys = ("silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays")
    return {key: {"en": en, "de": de} for key, (en, de) in zip(keys, (silhouette, retained, merged, omitted, thin, hatching, styles, overlays))}


STYLES = ("Pads 0.3, stainless parts 0.4 to 0.72 (inner shrouds lighter), powder-coated chassis and stand 0.4 to 0.5, tyres 0.2, lamp housing 0.35, lens 0.9.", "Polster 0,3, Edelstahlteile 0,4 bis 0,72 (innere Hüllen heller), pulverbeschichtetes Fahrgestell und Ständer 0,4 bis 0,5, Reifen 0,2, Leuchtengehäuse 0,35, Linse 0,9.")
OVERLAYS = ("One shape per part and region: pads, frame, column, chassis, stand, lamp housing and lens; in the thermal view only the lamp head warms.", "Eine Form je Teil und Region: Polster, Rahmen, Säule, Fahrgestell, Ständer, Leuchtengehäuse und Linse; in der Thermalansicht erwärmt sich nur der Leuchtenkopf.")
NO_HATCH = ("None; the lens is a light fill.", "Keine; die Linse ist hell gefüllt.")

DECISIONS = {
    "50": texts(
        ("True orthographic views of the 1:50 model, 49 x 39.2 mm in the side elevation: the chassis with its chamfered spine, rounded cross beams and twin castors, the three telescopic shrouds with flange and cap, the bearing block and the saddle cheek on the tilt pin, the section trays with hinge barrels and accessory rails, the head plate on its rod, four pads in a row, and the light stand beyond the foot end with legs, castors, ballast box, socket, tube, driver box, bearing head, spring arm, half yoke and the lamp head with lens and handle over the leg plates. Front view from the foot end 11.2 mm wide: stand in front of the leg plates, column and chassis. Plan: head pad, back, seat and the two leg pads between the rails, the chassis ends, the stand and the lamp head with arm and yoke above the table.", "Wahre orthogonale Ansichten des Modells 1:50, in der Seitenansicht 49 x 39,2 mm: das Fahrgestell mit gefastem Mittelträger, gerundeten Querträgern und Doppelrollen, die drei Teleskophüllen mit Flansch und Deckel, Lagerbock und Sattelwange am Kippbolzen, die Segmentwannen mit Gelenkhülsen und Zubehörschienen, die Kopfplatte auf ihrem Stab, vier Polster in einer Reihe und hinter dem Fußende der Leuchtenständer mit Holmen, Rollen, Ballastkasten, Fußhülse, Rohr, Treiberkasten, Lagerkopf, Federarm, Halbbügel und dem Leuchtenkopf mit Linse und Griff über den Beinplatten. Vorderansicht vom Fußende, 11,2 mm breit: Ständer vor Beinplatten, Säule und Fahrgestell. Draufsicht: Kopfpolster, Rücken, Sitz und die zwei Beinpolster zwischen den Schienen, die Enden des Fahrgestells, der Ständer und der Leuchtenkopf mit Arm und Bügel über dem Tisch."),
        ("Top 2.0 m long at 0.90 m, pads 80 mm, section frames 40 mm, rails 25 x 10 mm, column 0.30 m wide stepping to 0.24 m, tilt pin at 0.74 m, chassis 0.94 m with 100 mm castors, stand 1.96 m with a 50 mm tube, 0.8 m arm, lamp head 0.5 m with the lens at 1.66 m.", "Platte 2,0 m lang auf 0,90 m, Polster 80 mm, Segmentrahmen 40 mm, Schienen 25 x 10 mm, Säule 0,30 m breit, gestuft bis 0,24 m, Kippbolzen auf 0,74 m, Fahrgestell 0,94 m mit 100-mm-Rollen, Ständer 1,96 m mit 50-mm-Rohr, Arm 0,8 m, Leuchtenkopf 0,5 m mit der Linse auf 1,66 m."),
        ("Left and right parts coincide in the side elevation (the right ones are drawn); the far wheel of each twin castor lies behind the near one.", "Linke und rechte Teile fallen in der Seitenansicht zusammen (die rechten sind gezeichnet); das hintere Rad jeder Doppelrolle liegt hinter dem vorderen."),
        ("Cables, hand control, hydraulic and spindle drives inside column and sections, the LED field behind the lens, screws.", "Kabel, Handbedienung, Hydraulik- und Spindelantriebe in Säule und Segmenten, das LED-Feld hinter der Linse, Schrauben."),
        ("Nothing is thickened: the 10 mm rails (0.2 mm on paper), the 8 mm lens, the 16 mm rods and posts and the 25 mm yoke tube are filled shapes at their true size.", "Nichts ist verdickt: die 10-mm-Schienen (0,2 mm auf Papier), die 8-mm-Linse, die 16-mm-Stäbe und Stützen und das 25-mm-Bügelrohr sind gefüllte Formen in wahrer Größe."),
        NO_HATCH, STYLES, OVERLAYS),
    "100": texts(
        ("True orthographic views of the 1:100 model, 24.5 x 19.6 mm: chassis bars on single wheels, two column shrouds under the bearing block, the section slabs with their rail bars, the head plate, four pads, and the stand with legs, wheels, ballast bar, tube, driver box, bearing head, arm, yoke and the lamp head with lens and handle. Front view from the foot end; plan with the pads between the rail bars and the lamp head above the leg plates.", "Wahre orthogonale Ansichten des Modells 1:100, 24,5 x 19,6 mm: Fahrgestellholme auf Einzelrädern, zwei Säulenhüllen unter dem Lagerbock, die Segmentplatten mit ihren Schienenstäben, die Kopfplatte, vier Polster und der Ständer mit Holmen, Rädern, Ballastholm, Rohr, Treiberkasten, Lagerkopf, Arm, Bügel und dem Leuchtenkopf mit Linse und Griff. Vorderansicht vom Fußende; Draufsicht mit den Polstern zwischen den Schienenstäben und dem Leuchtenkopf über den Beinplatten."),
        ("The five-part top, the stepped column, the H chassis, the saddle on the pin, the arm, the yoke and the lens.", "Die fünfteilige Platte, die gestufte Säule, das H-Fahrgestell, der Sattel am Bolzen, der Arm, der Bügel und die Linse."),
        ("Rails and posts are one 30 mm bar, the twin wheels one wheel, the middle and upper shroud one, spine and ballast box of the stand one bar.", "Schienen und Stützen sind ein 30-mm-Stab, die Doppelräder ein Rad, mittlere und obere Hülle eine, Mittelholm und Ballastkasten des Ständers ein Holm."),
        ("Hinge barrels, head rods and clamp, pedals, castor forks, socket, the cooling step of the housing.", "Gelenkhülsen, Kopfstäbe und Klemmstück, Pedale, Rollengabeln, Fußhülse, die Kühlstufe des Gehäuses."),
        ("The lens is drawn 15 mm thick (0.15 mm), the yoke tube at its true 25 mm.", "Die Linse ist 15 mm stark gezeichnet (0,15 mm), das Bügelrohr in wahren 25 mm."),
        NO_HATCH, STYLES, OVERLAYS),
    "200": texts(
        ("True orthographic views of the 1:200 model, 12.3 x 9.8 mm: an H chassis on rollers, a two-step column with head block, the frame slab under a head pad, a body pad and a leg pad, and the stand with legs, rollers, ballast box, tube, bearing head, arm, L yoke and the lamp head with its handle. Front view from the foot end; plan with pads on the frame slab and the lamp head above.", "Wahre orthogonale Ansichten des Modells 1:200, 12,3 x 9,8 mm: ein H-Fahrgestell auf Walzen, eine zweistufige Säule mit Kopfblock, die Rahmenplatte unter Kopfpolster, Rumpfpolster und Beinpolster und der Ständer mit Holmen, Walzen, Ballastkasten, Rohr, Lagerkopf, Arm, L-Bügel und dem Leuchtenkopf mit Griff. Vorderansicht vom Fußende; Draufsicht mit den Polstern auf der Rahmenplatte und dem Leuchtenkopf darüber."),
        ("The narrower head piece, the pad joint at the knee hinge, the step of the column, the open space under the top, the yoke.", "Das schmalere Kopfteil, die Polsterfuge am Kniegelenk, die Stufe der Säule, der freie Raum unter der Platte, der Bügel."),
        ("Rails into the edge of the frame slab (0.56 m wide), back and seat pad into one, the two leg pads into one, lens into the head body.", "Schienen in die Kante der Rahmenplatte (0,56 m breit), Rücken- und Sitzpolster zu einem, die zwei Beinpolster zu einem, die Linse in den Kopfkörper."),
        ("Saddle cheeks, driver box, wheels as single wheels, every rounding under 30 mm.", "Sattelwangen, Treiberkasten, Räder als Einzelräder, jede Rundung unter 30 mm."),
        ("The yoke is drawn as 30 mm bars (0.15 mm) instead of the 25 mm tube; the 50 mm tube is true.", "Der Bügel ist als 30-mm-Stäbe (0,15 mm) statt des 25-mm-Rohrs gezeichnet; das 50-mm-Rohr ist wahr."),
        NO_HATCH, STYLES, OVERLAYS),
    "500": texts(
        ("True orthographic views of the 1:500 model, 4.9 x 3.9 mm: base slab, column block and the top slab with its narrower head piece; beside it the stand as foot slab, column, arm, hanger and lamp disc. Front view: the top slab on the column with the stand column in front; plan: the top, the stand foot, the arm and the round lamp head.", "Wahre orthogonale Ansichten des Modells 1:500, 4,9 x 3,9 mm: Fußplatte, Säulenblock und der Plattenquader mit dem schmaleren Kopfteil; daneben der Ständer als Fußplatte, Säule, Arm, Hänger und Leuchtenscheibe. Vorderansicht: der Plattenquader auf der Säule mit der Ständersäule davor; Draufsicht: die Platte, der Ständerfuß, der Arm und der runde Leuchtenkopf."),
        ("Top at 0.90 m and 2.0 m long, the open space under it, the stand height of 1.96 m, the lamp head over the table.", "Platte auf 0,90 m und 2,0 m lang, der freie Raum darunter, die Ständerhöhe 1,96 m, der Leuchtenkopf über dem Tisch."),
        ("Pads, frame and rails are one slab; chassis and castors one base slab; legs, castors and ballast one foot slab; yoke one hanger; housing and lens one disc.", "Polster, Rahmen und Schienen sind ein Quader; Fahrgestell und Rollen eine Fußplatte; Holme, Rollen und Ballast eine Fußplatte; der Bügel ein Hänger; Gehäuse und Linse eine Scheibe."),
        ("Everything below 75 mm: rails, hinges, saddle, handle, driver box.", "Alles unter 75 mm: Schienen, Gelenke, Sattel, Griff, Treiberkasten."),
        ("The stand column, arm and hanger are drawn 60 mm wide (0.12 mm) and read as their pen line.", "Ständersäule, Arm und Hänger sind 60 mm breit gezeichnet (0,12 mm) und lesen sich als ihre Stiftlinie."),
        NO_HATCH, STYLES, OVERLAYS),
}


def apply(asset, parts_by_scale):
    """🧰 Writes the redrawn geometry and decisions of every scale into the asset."""
    for scale, plist in parts_by_scale.items():
        representation = asset["representations"][scale]
        representation["geometry"] = geometry(scale, plist)
        representation["decisions"] = DECISIONS[scale]
