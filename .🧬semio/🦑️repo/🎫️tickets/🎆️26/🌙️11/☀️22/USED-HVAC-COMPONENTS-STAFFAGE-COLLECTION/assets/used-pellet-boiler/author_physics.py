"""⚗️ Materials, anchors, poses, clips, thermal and structural model of the used pellet boiler; imported by `author-used-pellet-boiler.py`.

@see ../../brief.md
"""
import math

PT = 0.144
TOPC = 1.194
TOPL = 1.214


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def references():
    """📚 The standards, datasheets and literature the asset is modelled to."""
    return [
        {"id": "en-303-5", "citation": "EN 303-5:2021 Heating boilers for solid fuels, manually and automatically stoked, nominal heat output up to 500 kW: classes, efficiency (class 5 boilers about 90 to 93 % at nominal output), flue gas temperatures and surface temperature limits."},
        {"id": "en-iso-17225-2", "citation": "EN ISO 17225-2:2021 Solid biofuels, graded wood pellets: 6 mm diameter, net calorific value at least 16.5 MJ/kg (4.6 kWh/kg), bulk density 600 to 750 kg/m3, moisture at most 10 %."},
        {"id": "pellet-boiler-datasheets", "citation": "Manufacturer data sheets of 15 kW class pellet boilers with integrated day hopper and vacuum feed (Froeling PE1 Pellet, OekoFEN Pellematic, Windhager BioWIN 2, read 2026-11-22): casing about 0.6 to 0.8 m wide, 0.7 to 0.8 m deep and 1.1 to 1.3 m high, side hopper 100 to 200 l, empty weight 200 to 300 kg, water content 50 to 70 l, flue spigot 130 mm at the top rear, electrical demand 40 to 80 W in operation and 250 to 400 W while the igniter runs, suction turbine 1.0 to 1.3 kW, flow temperature up to 80 to 90 degC with a minimum return of about 55 degC. Dimensions of this asset are a typical member of that class, not one product."},
        {"id": "din-580", "citation": "DIN 580:2010-08 Lifting eye bolts: M12, working load limit 340 kg in the axial direction, ring outer diameter about 54 mm, ring section about 11 mm, collar 30 mm."},
        {"id": "en-818-4", "citation": "EN 818-4:1996+A1:2008 Short link chain for lifting purposes, chain slings grade 8: 8 mm chain about 1.4 kg/m, working load limit of a two-leg sling at 0 to 45 degrees 2.8 t."},
        {"id": "en-12195-2", "citation": "EN 12195-2:2000 Load restraint assemblies on road vehicles, web lashing made from man-made fibres: 50 mm polyester webbing about 1.5 to 2 mm thick, ratchet tensioner, LC 2500 daN."},
        {"id": "en-13698-1", "citation": "EN 13698-1:2003 Pallet production specification, construction of 800 x 1200 mm flat wooden pallets (EPAL 1): five deck boards 22 mm (145 / 100 / 145 / 100 / 145 mm), three stringer boards 22 x 145 mm, nine blocks 78 mm high, three bottom boards 22 mm, overall height 144 mm, mass about 25 kg."},
        {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products, hygrothermal properties, tabulated design values: spruce 450 to 500 kg/m3, 0.13 W/mK, 1600 J/kgK; steel 7800 kg/m3, 50 W/mK, 450 J/kgK; mineral wool 0.035 to 0.04 W/mK."},
        {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley: emissivities of painted, oxidised and galvanised steel, polymers and timber; natural convection from vertical plates and cylinders."},
        {"id": "obernberger-2010", "citation": "Obernberger, I., Thek, G. (2010). The Pellet Handbook: The Production and Thermal Utilisation of Biomass Pellets, Earthscan: combustion air ratio 1.4 to 1.8 in small pellet boilers, flue gas mass about 9 to 11 kg per kg of pellets, flue gas temperature 110 to 160 degC at nominal load, water vapour in the flue gas from fuel hydrogen and moisture."},
        {"id": "vdi-2700", "citation": "VDI 2700 Blatt 3.1 (2006) Securing of loads on road vehicles, lashing: tie-down lashing over the load with ratchet straps hooked to the load carrier."},
    ]


def M(id, en, de, e, a, rho, k, c, src, ref, E=None):
    """🧱 A material."""
    m = {"id": id, "title": T(en, de), "emissivity": e, "solarAbsorptivity": a, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": c, "source": src, "reference": ref}
    if E:
        m["elasticModulusPa"] = E
    return m


def materials():
    """🎨 Every material of the asset; worn and painted surfaces are tagged with their source."""
    return [
        M("powder-coat-faded", "Faded light grey powder-coated steel sheet 1.0 mm (casing, hopper, cyclone)", "Verblasstes hellgrau pulverbeschichtetes Stahlblech 1,0 mm (Gehäuse, Behälter, Zyklon)", 0.9, 0.45, 7850, 50, 460, "authored-estimate", "incropera-2018", 210e9),
        M("enamel-black", "Black stove enamel on the insulated combustion door, dulled by heat", "Schwarzer Ofenlack auf der gedämmten Brennraumtür, durch Hitze stumpf", 0.93, 0.92, 7850, 50, 460, "authored-estimate", "incropera-2018"),
        M("mineral-wool", "Mineral wool insulation 40 mm behind the casing", "Mineralwolldämmung 40 mm hinter dem Gehäuse", 0.9, 0.5, 60, 0.04, 1030, "literature", "iso-10456-2007"),
        M("borosilicate", "Borosilicate sight glass, sooted at the edge", "Borosilikat-Schauglas, am Rand verrußt", 0.9, 0.2, 2230, 1.1, 830, "literature", "incropera-2018"),
        M("galvanised-steel", "Weathered galvanised steel (feet, base frame, handles, ratchets)", "Bewitterter verzinkter Stahl (Füße, Grundrahmen, Griffe, Ratschen)", 0.28, 0.65, 7850, 50, 460, "literature", "incropera-2018", 210e9),
        M("polycarbonate", "Polycarbonate control panel, yellowed", "Polycarbonat-Bedienfeld, vergilbt", 0.9, 0.4, 1200, 0.2, 1200, "authored-estimate", "incropera-2018"),
        M("aluminium-plate", "Anodised aluminium type plate", "Eloxiertes Aluminium-Typenschild", 0.6, 0.4, 2700, 200, 900, "authored-estimate", "incropera-2018"),
        M("steel-sooted", "Sooted and tarnished steel flue spigot", "Verrußter, angelaufener Stahl-Abgasstutzen", 0.85, 0.85, 7850, 50, 460, "authored-estimate", "incropera-2018"),
        M("cast-aluminium", "Die-cast aluminium flue gas fan, dust-grey", "Druckguss-Aluminium des Saugzuggebläses, staubgrau", 0.55, 0.5, 2700, 150, 900, "authored-estimate", "incropera-2018"),
        M("steel-rusty", "Black steel pipe nipple with flash rust", "Schwarzer Stahl-Rohrnippel mit Flugrost", 0.8, 0.8, 7850, 50, 460, "authored-estimate", "incropera-2018"),
        M("brass-tarnished", "Tarnished brass cap with limescale", "Angelaufene Messingkappe mit Kalkbelag", 0.5, 0.6, 8500, 110, 380, "authored-estimate", "incropera-2018"),
        M("polymer-housing", "Grey polymer housing of the suction turbine", "Graues Kunststoffgehäuse der Saugturbine", 0.9, 0.6, 1100, 0.2, 1500, "authored-estimate", "incropera-2018"),
        M("pvc-spiral", "Grey PVC spiral hose, cut", "Grauer PVC-Spiralschlauch, abgeschnitten", 0.92, 0.6, 1300, 0.16, 1000, "authored-estimate", "incropera-2018"),
        M("spruce-weathered", "Weathered grey spruce of the pallet boards", "Vergrautes Fichtenholz der Palettenbretter", 0.9, 0.7, 470, 0.13, 1600, "literature", "iso-10456-2007", 11e9),
        M("chipboard-block", "Compressed wood-chip pallet block", "Palettenklotz aus Holzspanpressholz", 0.9, 0.65, 550, 0.14, 1700, "authored-estimate", "iso-10456-2007", 3e9),
        M("polyester-webbing", "Faded orange polyester webbing 50 mm", "Verblasstes orangefarbenes Polyestergurtband 50 mm", 0.9, 0.55, 1380, 0.2, 1200, "authored-estimate", "en-12195-2"),
        M("alloy-steel", "Grade 8 alloy steel chain and eye bolts, oxidised", "Kette und Ringschrauben aus legiertem Stahl Güteklasse 8, oxidiert", 0.8, 0.75, 7850, 45, 460, "literature", "en-818-4", 210e9),
    ]


def anchors(W):
    """📍 Supports of the pallet runners, contacts of the boiler feet, the crane hook, the flue outlet and the two service loads."""
    ob = W("boiler")
    op = W("pallet")
    A = []
    for n, x, l in (("back", -0.35, 0.1), ("middle", 0.0, 0.145), ("front", 0.35, 0.1)):
        A.append({"id": f"pallet-runner-{n}", "bone": "pallet", "x": round(x - op[0], 5), "y": 0, "kind": "support", "title": T(f"{n.capitalize()} bottom board of the pallet on the yard floor", {"back": "Hinteres", "middle": "Mittleres", "front": "Vorderes"}[n] + " Bodenbrett der Palette auf dem Lagerboden"), "support": "contact", "footprintLengthM": l, "footprintWidthM": 1.2, "frictionCoefficient": 0.5})
    for n, x, z in (("front-right", 0.33, -0.45), ("front-left", 0.33, 0.45), ("back-right", -0.33, -0.45), ("back-left", -0.33, 0.45)):
        en = n.replace("-", " ")
        de = {"front-right": "vorne rechts", "front-left": "vorne links", "back-right": "hinten rechts", "back-left": "hinten links"}[n]
        A.append({"id": f"foot-{n}", "bone": "boiler", "x": round(x - ob[0], 5), "y": 0, "z": z, "kind": "contact", "title": T(f"Levelling foot {en} on the boiler room floor", f"Stellfuß {de} auf dem Heizraumboden"), "support": "contact", "footprintLengthM": 0.04, "footprintWidthM": 0.04, "frictionCoefficient": 0.4})
    A.append({"id": "crane-hook", "bone": "sling-front", "x": -0.45, "y": -0.022, "kind": "support", "title": T("Crane hook in the master link of the chain sling", "Kranhaken im Aufhängering des Kettengehänges"), "support": "pin"})
    A.append({"id": "flue-outlet", "bone": "boiler", "x": round(-0.25 - ob[0], 5), "y": round(TOPC + 0.12 - ob[1], 5), "z": -0.2, "kind": "emitter", "title": T("Flue gas outlet at the top of the spigot", "Abgasaustritt oben am Stutzen")})
    A.append({"id": "water-centre", "bone": "boiler", "x": round(0.0 - ob[0], 5), "y": round(0.62 - ob[1], 5), "z": -0.2, "kind": "load", "title": T("Centre of the water content of the heat exchanger", "Schwerpunkt des Wasserinhalts im Wärmeübertrager")})
    A.append({"id": "hopper-centre", "bone": "boiler", "x": round(0.0 - ob[0], 5), "y": round(0.60 - ob[1], 5), "z": 0.3, "kind": "load", "title": T("Centre of the pellet fill of the day hopper", "Schwerpunkt der Pelletfüllung im Tagesbehälter")})
    return A


def sling_angles():
    """📐 Leg angles of the lifted sling: the free chain ends meet above the centre of the casing (bisection on the leg geometry)."""
    a, b, target = -0.45, -0.022, 0.011 - 0.25
    lo, hi = -89.0, 0.0
    for _ in range(80):
        mid = (lo + hi) / 2
        f = math.radians(mid)
        x = a * math.cos(f) - b * math.sin(f)
        if x < target:
            hi = mid
        else:
            lo = mid
    return round((lo + hi) / 2, 3)


def strap_angles():
    """🎗 Fold angles of the released straps: each leg swings down onto the yard floor from its hook, its top half continues flat along the floor."""
    out = {}
    for s, top in (("r", TOPC), ("l", TOPL)):
        L = top + 0.002 - PT
        th = math.degrees(math.asin(PT / L))
        out[s] = (round(90 + th, 3), round(90 - th, 3))
    return out


REGIONS = ["casing", "hopper", "combustion-door", "flue-spigot", "flue-fan", "connections", "suction-unit", "suction-hose", "pallet", "straps", "sling"]


def bind(levels, emit, contact_floor=False, sling_free=False):
    """🌬 A thermal binding of every region and emitter."""
    reg = {}
    for r in REGIONS:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if r == "pallet":
            b = {"exposedFraction": 0.9, "sunlitFraction": 0, "contact": {"fraction": 0.1, "tempC": 14}}
        if r == "casing" and contact_floor:
            b = {"exposedFraction": 0.98, "sunlitFraction": 0, "contact": {"fraction": 0.02, "tempC": 14}}
        reg[r] = b
    return {"operating": levels, "regions": reg, "emitters": {"flue-gas": emit}}


def offsets(state):
    """🦴 Bone offsets of the three configurations."""
    st = strap_angles()
    phi = sling_angles()
    B = {}
    if state in ("released", "taut", "lifted", "reinstalled"):
        for s, (leg, top) in st.items():
            B[f"strap-{s}-front"] = {"angleDeg": -leg}
            B[f"strap-{s}-front-top"] = {"angleDeg": -top}
            B[f"strap-{s}-ratchet"] = {"angleDeg": 180}
            B[f"strap-{s}-back"] = {"angleDeg": leg}
            B[f"strap-{s}-back-top"] = {"angleDeg": top}
    if state in ("taut", "lifted"):
        B["sling-front"] = {"angleDeg": phi}
        B["sling-back"] = {"angleDeg": -phi}
    if state == "lifted":
        B["boiler"] = {"dy": 0.3}
        B["sling"] = {"dy": 0.3}
    if state == "reinstalled":
        B["boiler"] = {"dy": -PT}
        B["pallet"] = {"dx": 2.4}
        B["sling"] = {"dy": -(TOPC - PT)}
    return B


OFF = {"burner": 0, "controls": 0, "suction": 0, "heating": 0}
FEET = ["foot-front-right", "foot-front-left", "foot-back-right", "foot-back-left"]


def service_loads():
    """⚖️ The service loads of the reinstalled boiler: 62 l of heating water and a full day hopper of 100 kg pellets."""
    return [
        {"id": "water-content", "kind": "carried", "title": T("Heating water 62 l in the heat exchanger, 0.61 kN", "Heizwasser 62 l im Wärmeübertrager, 0,61 kN"), "anchor": "water-centre", "fxN": 0, "fyN": -608, "source": "authored-estimate", "reference": "pellet-boiler-datasheets"},
        {"id": "pellet-fill", "kind": "carried", "title": T("Full day hopper, 155 l of pellets at 650 kg/m3 (100 kg), 0.98 kN", "Voller Tagesbehälter, 155 l Pellets mit 650 kg/m3 (100 kg), 0,98 kN"), "anchor": "hopper-centre", "fxN": 0, "fyN": -981, "source": "literature", "reference": "en-iso-17225-2"},
    ]


def poses():
    """🧍 Stored (default), lifted, reinstalled firing, reinstalled cold, and the two key poses of the lift."""
    P = []
    P.append({"id": "stored", "title": T("Stored on the pallet", "Auf der Palette gelagert"),
              "description": T("As the component exchange keeps it: drained, flue and connections capped, strapped to an EPAL pallet with two ratchet straps over the top, the chain sling hooked into its eye bolts and laid on the casing for the next lift.", "Wie die Bauteilbörse ihn lagert: entleert, Abgas und Anschlüsse verschlossen, mit zwei Ratschengurten über die Oberseite auf einer EPAL-Palette verzurrt, das Kettengehänge in seine Ringschrauben eingehängt und für das nächste Anheben auf das Gehäuse gelegt."),
              "bones": {}, "ground": "authored", "contacts": [], "thermal": bind(dict(OFF), 0), "loads": []})
    P.append({"id": "straps-released", "key": True, "title": T("Straps released", "Gurte gelöst"), "description": T("The ratchets are opened and both straps are laid down off the pallet onto the floor.", "Die Ratschen sind geöffnet und beide Gurte sind von der Palette auf den Boden gelegt."),
              "bones": offsets("released"), "ground": "authored", "contacts": [], "thermal": bind(dict(OFF), 0), "loads": []})
    P.append({"id": "sling-taut", "key": True, "title": T("Sling taut", "Gehänge gespannt"), "description": T("The crane hook has taken the master link; both chain legs stand taut at about 57 degrees from the eye bolts, the boiler still rests on the pallet.", "Der Kranhaken hat den Aufhängering übernommen; beide Kettenstränge stehen unter etwa 57 Grad straff von den Ringschrauben, der Kessel steht noch auf der Palette."),
              "bones": offsets("taut"), "ground": "authored", "contacts": ["crane-hook"], "thermal": bind(dict(OFF), 0), "loads": []})
    P.append({"id": "lifted", "title": T("Lifted off the pallet", "Von der Palette gehoben"),
              "description": T("Hanging 0.3 m above the pallet from the crane hook on its two-leg chain sling and the two M12 eye bolts on the casing top; the pallet and the released straps stay on the floor.", "Hängt 0,3 m über der Palette am Kranhaken, am zweisträngigen Kettengehänge und den zwei M12-Ringschrauben auf dem Gehäusedeckel; Palette und gelöste Gurte bleiben auf dem Boden."),
              "bones": offsets("lifted"), "ground": "authored", "contacts": ["crane-hook"], "thermal": bind(dict(OFF), 0), "loads": []})
    P.append({"id": "reinstalled", "title": T("Reinstalled, firing at 15 kW", "Wieder eingebaut, Feuerung mit 15 kW"),
              "description": T("Back in a boiler room on its four levelling feet, filled with 62 l of water and a full day hopper, burning 3.4 kg of pellets an hour for its nominal 15 kW: flow 70 degC, return 55 degC, flue gas at 140 degC up the spigot; the pallet with straps and sling is set aside.", "Wieder in einem Heizraum auf seinen vier Stellfüßen, mit 62 l Wasser und vollem Tagesbehälter, verbrennt 3,4 kg Pellets je Stunde für seine Nennleistung von 15 kW: Vorlauf 70 °C, Rücklauf 55 °C, Abgas mit 140 °C über den Stutzen; die Palette mit Gurten und Gehänge steht beiseite."),
              "bones": offsets("reinstalled"), "ground": "authored", "contacts": FEET, "thermal": bind({"burner": 1, "controls": 1, "suction": 0, "heating": 1}, 1, True), "loads": service_loads()})
    P.append({"id": "reinstalled-off", "title": T("Reinstalled, cold", "Wieder eingebaut, kalt"),
              "description": T("Reinstalled and filled but switched off: burner, fan and controls are off and the water stands at room temperature.", "Wieder eingebaut und gefüllt, aber ausgeschaltet: Brenner, Gebläse und Regelung sind aus und das Wasser hat Raumtemperatur."),
              "bones": offsets("reinstalled"), "ground": "authored", "contacts": FEET, "thermal": bind(dict(OFF), 0, True), "loads": service_loads()})
    return P


def clips():
    """🎞 The lift from the pallet and the cold start of the reinstalled boiler."""
    lift = {"id": "lift", "title": T("Lift off the pallet", "Von der Palette heben"),
            "description": T("The two ratchet straps are opened and laid down, the crane takes the chain sling taut and lifts the boiler 0.3 m off the pallet, which stays on the floor.", "Die zwei Ratschengurte werden geöffnet und abgelegt, der Kran strafft das Kettengehänge und hebt den Kessel 0,3 m von der Palette, die auf dem Boden bleibt."),
            "durationS": 14, "fps": 10, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 4, "pose": "straps-released", "ease": "smooth"}, {"t": 8, "pose": "sling-taut", "ease": "smooth"}, {"t": 14, "pose": "lifted", "ease": "smooth"}],
            "contacts": [{"anchor": "crane-hook", "fromS": 8, "toS": 14}],
            "phases": [{"id": "release", "fromS": 0, "toS": 4, "title": T("Release the straps", "Gurte lösen")}, {"id": "rig", "fromS": 4, "toS": 8, "title": T("Take the sling taut", "Gehänge straffen")}, {"id": "hoist", "fromS": 8, "toS": 14, "title": T("Hoist", "Heben")}],
            "events": [{"t": 0, "kind": "motion", "title": T("Ratchets opened", "Ratschen geöffnet")}, {"t": 8, "kind": "contact-on", "title": T("Crane hook takes the load", "Kranhaken übernimmt die Last"), "target": "crane-hook"}, {"t": 14, "kind": "motion", "title": T("Boiler hangs 0.3 m above the pallet", "Kessel hängt 0,3 m über der Palette")}],
            "tracks": []}
    fire = {"id": "cold-start", "title": T("Cold start and heat-up after reinstallation", "Kaltstart und Aufheizen nach dem Wiedereinbau"),
            "description": T("The reinstalled boiler is switched on: the igniter lights the pellet bed after one minute, the burner modulates up to its nominal 15 kW within ten minutes, the flow climbs to 70 degC and the flue gas rises; after twenty minutes the suction turbine refills the day hopper for one and a half minutes.", "Der wieder eingebaute Kessel wird eingeschaltet: der Zünder entzündet das Pelletbett nach einer Minute, der Brenner moduliert in zehn Minuten auf seine Nennleistung von 15 kW, der Vorlauf steigt auf 70 °C und das Abgas steigt auf; nach zwanzig Minuten füllt die Saugturbine den Tagesbehälter eineinhalb Minuten lang nach."),
            "durationS": 1500, "fps": 1, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "step"}, {"t": 1500, "pose": "reinstalled", "ease": "step"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 1500} for a in FEET],
            "phases": [{"id": "ignition", "fromS": 0, "toS": 60, "title": T("Ignition", "Zündung")}, {"id": "heat-up", "fromS": 60, "toS": 660, "title": T("Modulating up", "Hochmodulieren")}, {"id": "nominal", "fromS": 660, "toS": 1500, "title": T("Nominal output", "Nennleistung")}],
            "events": [{"t": 60, "kind": "ignition", "title": T("Pellet bed lights", "Pelletbett zündet"), "target": "casing"}, {"t": 1200, "kind": "operating", "title": T("Suction turbine refills the hopper", "Saugturbine füllt den Behälter nach"), "target": "suction-unit"}, {"t": 1290, "kind": "operating", "title": T("Hopper full, turbine off", "Behälter voll, Turbine aus"), "target": "suction-unit"}],
            "tracks": [
                {"target": "operating:controls", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 1500, "v": 1}]},
                {"target": "operating:burner", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 0.3}, {"t": 660, "v": 1}, {"t": 1500, "v": 1}]},
                {"target": "emitter:flue-gas", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 0.3}, {"t": 660, "v": 1}, {"t": 1500, "v": 1}]},
                {"target": "operating:heating", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 900, "v": 1}, {"t": 1500, "v": 1}]},
                {"target": "operating:suction", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 1200, "v": 1}, {"t": 1290, "v": 0}, {"t": 1500, "v": 0}]},
            ]}
    return [lift, fire]


def R(id, en, de, kind, model, mat, area, L, geom, svf, layers, hc, src, ref, **k):
    """🔥 A thermal region."""
    r = {"id": id, "title": T(en, de), "kind": kind, "model": model, "material": mat, "areaM2": area, "characteristicLengthM": L, "geometry": geom, "skyViewFactor": svf, "layers": layers, "heatCapacityJM2K": hc, "source": src, "reference": ref}
    r.update(k)
    return r


def Lay(id, en, de, kind, mat, t, res, hc, src="authored-estimate", ref="iso-10456-2007"):
    """🧶 A thermal layer."""
    return {"id": id, "title": T(en, de), "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": res, "heatCapacityJM2K": hc, "source": src, "reference": ref}


def thermal(areas):
    """🌡 Hall environment, operating keys, eleven regions with their real areas, the flue gas emitter."""
    A = lambda k, d: round(areas.get(k, d), 3)
    sheet = Lay("sheet", "Powder-coated steel sheet 1.0 mm, faded", "Pulverbeschichtetes Stahlblech 1,0 mm, verblasst", "metal", "powder-coat-faded", 0.001, 2e-05, 3600, "authored-estimate", "incropera-2018")
    circuit = {"key": "heating", "rows": [{"v": 0.5, "supplyTempC": 55, "returnTempC": 45}, {"v": 1, "supplyTempC": 70, "returnTempC": 55}], "flowPosition": 0.5, "filmM2KW": 0.002}
    regions = [
        R("casing", "Boiler casing with base frame, ash box and control panel", "Kesselgehäuse mit Grundrahmen, Aschebox und Bedienfeld", "surface", "operating", "powder-coat-faded", A("casing", 3.3), 1.0, "vertical-plate", 0.4,
          [Lay("insulation", "Mineral wool 40 mm around the heat exchanger", "Mineralwolle 40 mm um den Wärmeübertrager", "insulation", "mineral-wool", 0.04, 1.0, 2500, "literature", "iso-10456-2007"), sheet], 6100, "authored-estimate", "pellet-boiler-datasheets",
          operating={"key": "burner", "heatW": 220}, ratedPowerW=16300, ratedPowerBasis="fuel-input"),
        R("hopper", "Integrated day hopper with lid", "Integrierter Tagesbehälter mit Deckel", "surface", "passive", "powder-coat-faded", A("hopper", 2.0), 1.0, "vertical-plate", 0.4, [sheet], 3600, "authored-estimate", "pellet-boiler-datasheets"),
        R("combustion-door", "Insulated combustion chamber door with sight glass", "Gedämmte Brennraumtür mit Schauglas", "surface", "operating", "enamel-black", A("combustion-door", 0.27), 0.42, "vertical-plate", 0.4,
          [Lay("door-plate", "Enamelled steel door plate 3 mm over vermiculite", "Lackiertes Stahl-Türblatt 3 mm über Vermiculit", "metal", "enamel-black", 0.003, 0.05, 15000)], 15000, "authored-estimate", "en-303-5", operating={"key": "burner", "heatW": 40}),
        R("flue-spigot", "Flue spigot 130 mm at the top rear, sooted inside", "Abgasstutzen 130 mm oben hinten, innen verrußt", "exhaust", "operating", "steel-sooted", A("flue-spigot", 0.11), 0.13, "cylinder", 0.6,
          [Lay("pipe", "Steel tube 2.5 mm", "Stahlrohr 2,5 mm", "metal", "steel-sooted", 0.0025, 5e-05, 9000)], 9000, "authored-estimate", "obernberger-2010", operating={"key": "burner", "heatW": 55}),
        R("flue-fan", "Flue gas fan with the drives of auger, igniter and controls", "Saugzuggebläse mit den Antrieben von Schnecke, Zünder und Regelung", "engine", "operating", "cast-aluminium", A("flue-fan", 0.16), 0.2, "vertical-plate", 0.4,
          [Lay("housing", "Die-cast aluminium housing 4 mm", "Druckguss-Aluminiumgehäuse 4 mm", "metal", "cast-aluminium", 0.004, 3e-05, 9700)], 9700, "authored-estimate", "pellet-boiler-datasheets", operating={"key": "controls", "heatW": 30}, ratedPowerW=360, ratedPowerBasis="electrical-input"),
        R("connections", "Capped flow and return stubs 1 inch at the rear", "Verschlossene Vor- und Rücklaufstutzen 1 Zoll hinten", "surface", "hydronic", "steel-rusty", A("connections", 0.026), 0.034, "cylinder", 0.5,
          [Lay("pipe", "Steel pipe 3.2 mm", "Stahlrohr 3,2 mm", "metal", "steel-rusty", 0.0032, 6e-05, 11500)], 11500, "authored-estimate", "pellet-boiler-datasheets", hydronic=circuit, ratedPowerW=0, ratedPowerBasis="passive"),
        R("suction-unit", "Cyclone separator with suction turbine on the hopper", "Zyklonabscheider mit Saugturbine auf dem Behälter", "surface", "operating", "powder-coat-faded", A("suction-unit", 0.33), 0.24, "cylinder", 0.6, [sheet], 3600, "authored-estimate", "pellet-boiler-datasheets",
          operating={"key": "suction", "heatW": 60}, ratedPowerW=1150, ratedPowerBasis="electrical-input"),
        R("suction-hose", "Remnant of the pellet suction hose, cut", "Rest des Pellet-Saugschlauchs, abgeschnitten", "surface", "passive", "pvc-spiral", A("suction-hose", 0.07), 0.06, "cylinder", 0.5,
          [Lay("wall", "PVC spiral hose wall 3 mm", "PVC-Spiralschlauchwand 3 mm", "polymer", "pvc-spiral", 0.003, 0.019, 3900)], 3900, "authored-estimate", "incropera-2018"),
        R("pallet", "EPAL euro pallet, weathered", "EPAL-Europalette, vergraut", "surface", "passive", "spruce-weathered", A("pallet", 6.0), 0.8, "horizontal-plate-up", 0.5,
          [Lay("boards", "Spruce boards 22 mm", "Fichtenbretter 22 mm", "timber", "spruce-weathered", 0.022, 0.17, 16500, "literature", "iso-10456-2007")], 16500, "literature", "en-13698-1"),
        R("straps", "Two ratchet straps 50 mm", "Zwei Ratschengurte 50 mm", "surface", "passive", "polyester-webbing", A("straps", 0.35), 0.05, "vertical-plate", 0.4,
          [Lay("webbing", "Polyester webbing 2 mm", "Polyestergurtband 2 mm", "polymer", "polyester-webbing", 0.002, 0.01, 3300, "authored-estimate", "en-12195-2")], 3300, "authored-estimate", "en-12195-2"),
        R("sling", "Two-leg chain sling with eye bolts", "Zweisträngiges Kettengehänge mit Ringschrauben", "surface", "passive", "alloy-steel", A("sling", 0.04), 0.022, "cylinder", 0.5,
          [Lay("chain", "Chain steel, 8 mm bar", "Kettenstahl, 8 mm Rundstahl", "metal", "alloy-steel", 0.008, 0.0002, 7200, "literature", "en-818-4")], 7200, "literature", "en-818-4"),
    ]
    return {
        "environment": {"airTempC": 15, "meanRadiantTempC": 15, "skyTempC": 14, "groundTempC": 14, "relativeHumidity": 0.6, "windMS": 0.2, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
        "passengers": [],
        "operating": [
            {"id": "burner", "title": T("Pellet burner", "Pelletbrenner"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 0.3, "title": T("Minimum load 4.5 kW", "Kleinlast 4,5 kW")}, {"v": 1, "title": T("Nominal 15 kW output (16.3 kW fuel)", "Nennleistung 15 kW (16,3 kW Brennstoff)")}]},
            {"id": "controls", "title": T("Controls, flue gas fan and auger", "Regelung, Saugzuggebläse und Schnecke"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("Running", "In Betrieb")}]},
            {"id": "suction", "title": T("Suction turbine refilling the hopper", "Saugturbine füllt den Behälter"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("Refilling (1.15 kW)", "Nachfüllen (1,15 kW)")}]},
            {"id": "heating", "title": T("Heating water through the boiler", "Heizwasser durch den Kessel"), "levels": [{"v": 0, "title": T("Drained or cold", "Entleert oder kalt")}, {"v": 0.5, "title": T("Warming up, 55/45 degC", "Aufwärmen, 55/45 °C")}, {"v": 1, "title": T("Design 70/55 degC", "Auslegung 70/55 °C")}]},
        ],
        "regions": regions,
        "emitters": [{"id": "flue-gas", "kind": "flue", "title": T("Flue gas leaving the spigot", "Abgas aus dem Stutzen"), "anchor": "flue-outlet", "directionDeg": 90, "diameterM": 0.123, "operatingKey": "burner", "volumeFlowM3S": 0.0118, "temperatureRiseK": 125, "vapourKgPerKg": 0.055, "source": "authored-estimate", "reference": "obernberger-2010"}],
        "assumptions": [
            T("Nameplate: 15 kW nominal heat output at about 92 % efficiency is 16.3 kW of pellet fuel input (3.5 kg/h at 4.7 kWh/kg), stated on the casing region; controls, flue gas fan, auger and igniter 360 W and the suction turbine 1.15 kW are stated as electrical input on their regions. The capped stubs carry water from elsewhere and are passive.", "Typenschild: 15 kW Nennwärmeleistung bei etwa 92 % Wirkungsgrad sind 16,3 kW Pellet-Brennstoffzufuhr (3,5 kg/h bei 4,7 kWh/kg), angegeben an der Gehäuseregion; Regelung, Saugzuggebläse, Schnecke und Zünder mit 360 W und die Saugturbine mit 1,15 kW stehen als elektrische Aufnahme an ihren Regionen. Die verschlossenen Stutzen führen Wasser von anderswo und sind passiv."),
            T("Only the surface losses heat the room: about 220 W from the casing, 40 W from the door, 55 W from the spigot and 30 W from the fan at nominal output; the useful 15 kW go into the heating water and the flue gas carries about 1.6 kW (about 42 kg/h at 140 degC, 125 K above the hall) up the flue. The flue gas is shown leaving the spigot although in service a connector pipe leads it into the chimney.", "Nur die Oberflächenverluste heizen den Raum: etwa 220 W vom Gehäuse, 40 W von der Tür, 55 W vom Stutzen und 30 W vom Gebläse bei Nennleistung; die nutzbaren 15 kW gehen ins Heizwasser, und das Abgas führt etwa 1,6 kW (etwa 42 kg/h mit 140 °C, 125 K über der Halle) über den Abzug. Das Abgas ist am Stutzen austretend gezeigt, obwohl es im Betrieb über ein Verbindungsstück in den Schornstein geht."),
            T("Stored, lifted and cold the boiler is drained (no water in the stubs) and every surface is at the hall temperature of 15 degC; the faded powder coat (absorptance 0.45 instead of about 0.3 when new), the dulled stove enamel, the sooted spigot, the rusty stubs and the weathered grey pallet timber are authored estimates of used surfaces.", "Gelagert, angehoben und kalt ist der Kessel entleert (kein Wasser in den Stutzen) und jede Oberfläche hat die Hallentemperatur von 15 °C; die verblasste Pulverbeschichtung (Absorptionsgrad 0,45 statt etwa 0,3 im Neuzustand), der stumpfe Ofenlack, der verrußte Stutzen, die rostigen Stutzen und das vergraute Palettenholz sind Schätzwerte gebrauchter Oberflächen."),
        ],
    }


def structure():
    """🏗 Quasi-static rigid-body statics on the pallet runners, the boiler feet or the crane hook."""
    return {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": ["pallet-runner-back", "pallet-runner-middle", "pallet-runner-front"],
            "assumptions": [
                T("Stored: the 284 kg assembly (boiler 255 kg, pallet 25 kg with straps 1.8 kg, sling 2.4 kg) stands on the three bottom boards of the pallet as unilateral contacts with friction 0.5 on the yard floor; the boiler sits on its four levelling feet on the outer deck boards, which carry it through the stringer boards and the nine blocks into the bottom boards.", "Gelagert: die 284 kg schwere Einheit (Kessel 255 kg, Palette 25 kg mit Gurten 1,8 kg, Gehänge 2,4 kg) steht auf den drei Bodenbrettern der Palette als einseitige Kontakte mit Reibung 0,5 auf dem Lagerboden; der Kessel steht mit seinen vier Stellfüßen auf den äußeren Deckbrettern, die ihn über die Querbretter und die neun Klötze in die Bodenbretter abtragen."),
                T("Lifted: the boiler hangs from the crane hook (pin) in the master link of a two-leg grade 8 chain sling with 8 mm chain, its legs at about 57 degrees to the casing top, hooked into two M12 eye bolts DIN 580 screwed into the casing above the centre of mass; each leg carries about 1.5 kN. The solver treats the asset as one rigid body, so the pallet runners stay active supports in this pose and share the load with the hook; read the hook reaction as the weight of the boiler and sling, about 2.53 kN.", "Angehoben: der Kessel hängt am Kranhaken (Gelenk) im Aufhängering eines zweisträngigen Kettengehänges der Güteklasse 8 mit 8-mm-Kette, die Stränge unter etwa 57 Grad zum Gehäusedeckel, eingehängt in zwei M12-Ringschrauben DIN 580 über dem Schwerpunkt; jeder Strang trägt etwa 1,5 kN. Der Löser behandelt das Asset als einen starren Körper, daher bleiben die Palettenkufen in dieser Pose aktive Auflager und teilen die Last mit dem Haken; die Hakenreaktion ist als Gewicht von Kessel und Gehänge zu lesen, etwa 2,53 kN."),
                T("Reinstalled: the boiler stands on its four levelling feet (contacts, friction 0.4) and carries its service loads, 62 l of heating water (0.61 kN) in the heat exchanger and a full day hopper of 100 kg of pellets (0.98 kN); with its own 2.50 kN each foot takes about 1.0 kN. Masses are datasheet-typical (authored estimate), the pallet mass follows EN 13698-1.", "Wieder eingebaut: der Kessel steht auf seinen vier Stellfüßen (Kontakte, Reibung 0,4) und trägt seine Betriebslasten, 62 l Heizwasser (0,61 kN) im Wärmeübertrager und einen vollen Tagesbehälter mit 100 kg Pellets (0,98 kN); mit seinen eigenen 2,50 kN nimmt jeder Fuß etwa 1,0 kN auf. Die Massen sind datenblatttypisch (Schätzwert), die Palettenmasse folgt EN 13698-1."),
            ], "clinical": False}
