"""🌡 Round 3, furniture/office-swivel-chair: the thermal and structural lists and the decisions of every authored scale, written against that scale's own body.

Thermal: every body solid is the surface of the region of its material; the upholstery is two nested shells at 1:50 (foam core inside the fabric). Structure: the load path from the seat and the backrest through mechanism, gas spring, hub and the five arms into the castor stems, axles and floor contacts; upholstery, covers, hoods and levers carry nothing and are absent.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def module(name):
    spec = importlib.util.spec_from_file_location("chair_" + name, os.path.join(HERE, f"r3-office-swivel-chair-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


B = module("body")
K = B.K
REGION = {"pa-plastic": ("chair-plastic", "pa-shell"), "steel-painted": ("chair-steel", "tube"), "rubber": ("castor-rubber", "tyre"), "fabric-upholstery": ("chair-fabric", "fabric")}
MEMBER_OF = {"hub": "base", "column-sleeve": None, "column": None}


def thermal(body, scale):
    out = []
    for solid in body:
        region, layer = REGION[solid["material"]]
        row = {"id": "th-" + solid["id"], "of": solid["id"], "region": region}
        if scale == 50:
            row["layer"] = layer
        out.append(row)
    if scale == 50:
        for foam in B.foam50():
            out.append({**foam, "region": "chair-fabric", "layer": "foam"})
    if scale in (200, 500):
        out.append({**B.pad_coarse(scale), "region": "chair-fabric"})
    return out


def structure(body, scale):
    ids = {s["id"] for s in body}
    f = 0.15 * scale / 1000
    out = []

    def ref(solid_id, member, part="member"):
        if solid_id in ids:
            out.append({"id": f"{part}-{solid_id}", "of": solid_id, "member": member, "part": part})

    ref("hub", "base")
    for arm in B.ARMS:
        ref(f"arm-{arm}", f"arm-{arm}")
        ref(f"arm-{arm}-boss", f"arm-{arm}")
    out.append({**B.tube("member-standpipe", "column-sleeve", 0.045, 0.279, 0.0215, 0.025), "member": "column-sleeve", "part": "member"})
    out.append({**B.tube("member-piston", "column", 0.160, 0.343, 0.0105, 0.014), "member": "column", "part": "member"})
    for solid_id in ("seat-mechanism", "seat-plate", "seat-hinge", "seat-shell", "armrest-strap-left", "armrest-strap-right", "armrest-post-left", "armrest-post-right", "armrest-stem-left", "armrest-stem-right"):
        ref(solid_id, "seat")
    if scale >= 200:
        out.append({**B.seat_loft("member-seat-pan", "pa-plastic", "body", 0.3, [(0.215, 0.397, 0.210, 0.013 if scale == 200 else 0.015, 4), (0.000, 0.397, 0.230, 0.013 if scale == 200 else 0.015, 4), (-0.205, 0.399, 0.205, 0.013 if scale == 200 else 0.015, 4)]), "member": "seat", "part": "member"})
    for solid_id in ("backrest-strut-foot", "backrest-strut", "backrest-bracket", "backrest-shell", "backrest-shell-left", "backrest-shell-right"):
        ref(solid_id, "backrest")
    if scale <= 100:
        r_pin, r_axle = (0.0055, 0.004) if scale == 50 else (0.0075, 0.0075)
        for arm in B.ARMS:
            px, pz, ax = B.castor_at(arm)
            out.append({**K.revolve(f"joint-castor-{arm}-stem", f"castor-{arm}", "steel-painted", "detail", 0.15, (px, 0, pz), B.UP, [(0.046, 0), (0.046, r_pin), (0.082, r_pin), (0.082, 0)]), "member": f"castor-{arm}", "part": "joint"})
            out.append({**K.revolve(f"joint-castor-{arm}-axle", f"castor-{arm}", "steel-painted", "detail", 0.15, (ax, K.AXLE_Y, pz), B.LEFT, [(-0.023, 0), (-0.023, r_axle), (0.023, r_axle), (0.023, 0)]), "member": f"castor-{arm}", "part": "joint"})
        out.append({**B.tube("joint-column-bush", "column-sleeve", 0.262, 0.287, 0.014, 0.0265, 0.15), "member": "column", "part": "joint"})
        out.append({**K.revolve("joint-seat-taper", "column", "steel-painted", "detail", 0.15, (0, 0, 0), B.UP, [(0.318, 0), (0.318, 0.0165), (0.343, 0.0205), (0.343, 0)]), "member": "seat", "part": "joint"})
        r_hinge = 0.009 if scale == 50 else 0.011
        out.append({**K.revolve("joint-backrest-hinge", "seat", "steel-painted", "detail", 0.15, (K.PIVOT[0], K.PIVOT[1], 0), B.LEFT, [(-0.040, 0), (-0.040, r_hinge), (0.040, r_hinge), (0.040, 0)]), "member": "backrest", "part": "joint"})
    pad = {50: (0.030, 0.0075), 100: (0.030, 0.015), 200: (0.030, 0.030), 500: (0.050, 0.050)}[scale]
    for arm in B.ARMS:
        px, pz, ax = B.castor_at(arm)
        out.append({**K.box(f"support-contact-castor-{arm}", f"castor-{arm}", "rubber", "detail", 0.15, (ax, pad[1] / 2, pz), (pad[0], pad[1], 0.050)), "member": f"castor-{arm}", "part": "support", "anchor": f"contact-castor-{arm}"})
    half = max(0.02, f / 2)
    for bone, material, centre, radii in (("base", "pa-plastic", (0, 0.092, 0), (0.030, 0.020, 0.030)), ("column", "steel-painted", (0, 0.215, 0), (0.020, 0.045, 0.020)), ("seat", "steel-painted", (-0.007, 0.426, 0), (0.070, 0.020, 0.070)), ("backrest", "pa-plastic", (-0.280, 0.710, 0), (0.020, 0.060, 0.070))):
        out.append({**K.ellipsoid(f"mass-{bone}", bone, material, "detail", 0.15, centre, [max(r, half) for r in radii]), "member": bone, "part": "mass"})
    return out


DECISIONS = {
    50: [
        K.text(
            "Body, modelled from the real chair: hub with its conical bore, five lofted polyamide arms that fall from the hub to round stem bosses, hooded twin-wheel castors of 50 mm (swivel head, neck, centre web, hood, two rubber tyres on plastic rims each) trailing 20 mm behind their stems, the 50 mm standpipe with guide bush, the 28 mm piston tube with the taper socket, the pressed-steel mechanism with seat plate, hinge and pin, height and lock levers with paddles and the tension knob, the plastic seat pan under the cushion with its waterfall front, the T-armrests (strap under the seat, bent post, inner stem, cap and pad) and the backrest, doubly curved as one surface of revolution about an axis in front of the sitter (lumbar S-curve as the profile, wrap as the sweep) with lofted side bands that round the outline, as a shell and an upholstered pad, on the bent flat-steel carrier with its cover.",
            "Körper, nach dem wirklichen Stuhl modelliert: Nabe mit Konusbohrung, fünf geloftete Polyamidarme, die von der Nabe zu runden Stiftaugen abfallen, Doppelrollen mit Haube von 50 mm (Drehkopf, Hals, Mittelsteg, Haube, je zwei Gummireifen auf Kunststofffelgen), 20 mm hinter ihren Stiften nachlaufend, das Standrohr von 50 mm mit Führungsbuchse, das Kolbenrohr von 28 mm mit Konusaufnahme, die Stahlblechmechanik mit Sitzplatte, Gelenk und Bolzen, Höhen- und Arretierhebel mit Griffen und das Spannrad, die Kunststoffsitzschale unter dem Polster mit abgerundeter Vorderkante, die T-Armlehnen (Lasche unter dem Sitz, gebogene Stütze, Innenrohr, Auflage und Polster) und die Rückenlehne, doppelt gekrümmt als eine Drehfläche um eine Achse vor der sitzenden Person (Lordosekurve als Profil, Umgriff als Drehwinkel) mit gelofteten Seitenbändern, die den Umriss runden, als Schale und Polster auf dem gebogenen Flachstahlträger mit seiner Abdeckung.",
        ),
        K.text(
            "Thermal: every body solid is the surface of the region of its material; the upholstery is two nested shells, the foam core (cushion, pad and its side bands inset by 1 mm) inside the fabric, innermost first; the tyre region is the rubber treads alone, the rims belong to the plastic. Structure: its own load path, hub and arms (the mouldings are the members), standpipe and piston as tubes with their real walls, mechanism, plate, hinge and seat pan, armrest straps and stems, carrier, bracket and shell of the backrest; joints are the castor stems and axles, the guide bush, the taper and the hinge pin; one contact pad under each castor; masses at the centres of hub, gas spring, seat and backrest. Cushion, pad, hoods, levers and knob carry nothing and are absent.",
            "Thermik: jeder Körper ist die Oberfläche der Region seines Materials; das Polster besteht aus zwei geschachtelten Schalen, dem Schaumkern (Sitzpolster, Lehnenpolster und dessen Seitenbänder um 1 mm eingerückt) im Stoff, innerste zuerst; die Reifenregion sind allein die Gummilaufflächen, die Felgen gehören zum Kunststoff. Tragwerk: eigener Lastpfad, Nabe und Arme (die Formteile sind die Stäbe), Standrohr und Kolben als Rohre mit ihren wirklichen Wänden, Mechanik, Platte, Gelenk und Sitzschale, Armlehnenlaschen und -rohre, Träger, Halter und Schale der Lehne; Gelenke sind die Rollenstifte und Achsen, die Führungsbuchse, der Konus und der Gelenkbolzen; je ein Kontaktpolster unter jeder Rolle; Massen in den Schwerpunkten von Nabe, Gasfeder, Sitz und Lehne. Polster, Hauben, Hebel und Spannrad tragen nichts und fehlen.",
        ),
    ],
    100: [
        K.text(
            "Modelled anew for 15 mm: arms with three sections, plain stem bosses, each castor as swivel head, one centre body and two plain wheels (tyre and rim merged, neck and thin hood dropped), standpipe with its bush and piston with its socket as one revolve each, mechanism as one tapered pan with the hinge block and the knob, levers dropped (8 mm), seat pan and cushion with fewer sections, armrests as one bent post and a pad, backrest shell and pad with six profile stations each and their side bands, carrier and cover.",
            "Neu für 15 mm modelliert: Arme mit drei Schnitten, schlichte Stiftaugen, jede Rolle als Drehkopf, ein Mittelkörper und zwei schlichte Räder (Reifen und Felge vereint, Hals und dünne Haube entfallen), Standrohr mit Buchse und Kolben mit Aufnahme je als ein Drehkörper, Mechanik als eine verjüngte Wanne mit Gelenkblock und Spannrad, Hebel entfallen (8 mm), Sitzschale und Polster mit weniger Schnitten, Armlehnen als eine gebogene Stütze mit Polster, Lehnenschale und -polster mit je sechs Profilstationen und ihren Seitenbändern, Träger und Abdeckung.",
        ),
        K.text(
            "Thermal: references by material, the upholstery as one surface. Structure: the same load path with thicker stems, axles and hinge pin, tubes for standpipe and piston, pads and masses sized for the scale.",
            "Thermik: Verweise nach Material, das Polster als eine Oberfläche. Tragwerk: derselbe Lastpfad mit dickeren Stiften, Achsen und Gelenkbolzen, Rohre für Standrohr und Kolben, Auflager und Massen für den Maßstab bemessen.",
        ),
    ],
    200: [
        K.text(
            "Modelled anew for 30 mm: arms run through to rounded tips, the stem boss reaches down as the castor head, each castor is one wheel body, hub solid, standpipe and piston plain, mechanism one pan, seat pan and cushion one mass, armrests a post and a pad, shell and pad of the backrest one curved mass with two side bands; knob, hinge, levers and cover merged away. The 10 mm carrier stays at its true thickness because the back would float without it.",
            "Neu für 30 mm modelliert: Arme laufen bis zu gerundeten Spitzen durch, das Stiftauge reicht als Rollenkopf hinab, jede Rolle ist ein Radkörper, Nabe voll, Standrohr und Kolben schlicht, Mechanik eine Wanne, Sitzschale und Polster eine Masse, Armlehnen Stütze und Polster, Schale und Polster der Lehne eine gekrümmte Masse mit zwei Seitenbändern; Spannrad, Gelenk, Hebel und Abdeckung sind aufgegangen. Der 10 mm starke Träger behält seine wahre Dicke, weil die Lehne ohne ihn schwebte.",
        ),
        K.text(
            "Thermal: references by material; the pad of the backrest, merged into the shell in the body, is its own thin surface on the shell so the fabric region keeps its area. Structure: hub, arms and bosses, the two tubes, mechanism and a seat pan of its own, armrest posts, carrier and shell; supports and masses, no joint solids at this scale.",
            "Thermik: Verweise nach Material; das Lehnenpolster, im Körper in der Schale aufgegangen, ist eine eigene dünne Fläche auf der Schale, damit die Stoffregion ihre Fläche behält. Tragwerk: Nabe, Arme und Augen, die zwei Rohre, Mechanik und eine eigene Sitzschale, Armlehnenstützen, Träger und Schale; Auflager und Massen, keine Gelenkkörper in diesem Maßstab.",
        ),
    ],
    500: [
        K.text(
            "Modelled anew for 75 mm as the smallest chair that still reads: hub, five two-section arms with a boss over each wheel body, one stepped column, mechanism wedge, one seat mass, armrest post and pad, the backrest as one curved panel and its carrier. Minimal: nothing more can be merged without losing the five-star base, the armrests or the free-standing back.",
            "Neu für 75 mm modelliert als kleinster noch lesbarer Stuhl: Nabe, fünf Arme mit zwei Schnitten und einem Auge über jedem Radkörper, eine gestufte Säule, Mechanikkeil, eine Sitzmasse, Armlehnenstütze und -polster, die Lehne als eine gekrümmte Fläche und ihr Träger. Minimal: mehr lässt sich nicht vereinen, ohne Fünfstern, Armlehnen oder die frei stehende Lehne zu verlieren.",
        ),
        K.text(
            "Thermal: references by material plus the pad surface of the backrest. Structure: hub, arms, the column tubes, mechanism, seat pan, posts, carrier and back panel, one support per castor and four masses.",
            "Thermik: Verweise nach Material und die Polsterfläche der Lehne. Tragwerk: Nabe, Arme, die Säulenrohre, Mechanik, Sitzschale, Stützen, Träger und Lehnenfläche, ein Auflager je Rolle und vier Massen.",
        ),
    ],
}


def fill(asset, volume, scale):
    volume["thermal"] = thermal(volume["body"], scale)
    volume["structure"] = structure(volume["body"], scale)
    volume["decisions"] = DECISIONS[scale]
