"""Applies the EN 1991 units review to catalogue-bridge-en1991-inputs.tsv: re-derived units, scoped rows and closed-set choices (evidence in audits/wave2/units-review-en1991.md). Run once from this folder with python -I; the TSV is the source of truth afterwards."""
import pathlib

HERE = pathlib.Path(__file__).resolve().parent
TSV = HERE / "catalogue-bridge-en1991-inputs.tsv"

CATEGORIES = ["A", "A1", "A2", "A3", "B", "B1", "B2", "B3", "C1", "C2", "C3", "C4", "C5", "C6", "D1", "D2", "D3", "E1", "T1", "T2", "Z", "F", "G", "H"]
MATERIALS = [("concrete", "Concrete", "Beton"), ("reinforced_concrete", "Reinforced concrete", "Stahlbeton"), ("lightweight_concrete", "Lightweight concrete", "Leichtbeton"), ("steel", "Steel", "Stahl"), ("aluminium", "Aluminium", "Aluminium"), ("timber", "Timber", "Holz"), ("softwood", "Softwood", "Nadelholz"), ("hardwood", "Hardwood", "Laubholz"), ("glulam", "Glulam", "Brettschichtholz"), ("plywood", "Plywood", "Sperrholz"), ("masonry", "Masonry", "Mauerwerk"), ("brick", "Brick", "Ziegel"), ("calcium_silicate", "Calcium silicate", "Kalksandstein"), ("aerated_concrete", "Aerated concrete", "Porenbeton"), ("natural_stone", "Natural stone", "Naturstein"), ("mortar", "Mortar", "Mörtel"), ("cement_screed", "Cement screed", "Zementestrich"), ("asphalt", "Asphalt", "Asphalt"), ("plaster", "Plaster", "Putz"), ("glass", "Glass", "Glas"), ("water", "Water", "Wasser"), ("sand", "Sand", "Sand"), ("gravel", "Gravel", "Kies"), ("clay", "Clay", "Ton"), ("soil_fill", "Soil fill", "Bodenauffüllung")]
OCCUPANCIES = [("dwelling", "Dwelling", "Wohnung"), ("residential", "Residential", "Wohngebäude"), ("office", "Office", "Büro"), ("hospital", "Hospital", "Krankenhaus"), ("school", "School", "Schule"), ("hotel", "Hotel", "Hotel"), ("library", "Library", "Bibliothek"), ("shopping", "Shopping", "Verkauf"), ("mall", "Shopping mall", "Einkaufszentrum"), ("theatre", "Theatre", "Theater"), ("transport", "Transport area", "Verkehrsbereich")]
CHOICES = {
    "category": [(c, f"Category {c}", f"Kategorie {c}") for c in CATEGORIES],
    "cen_zone": [("alpine", "Alpine region", "Alpenregion"), ("central_east", "Central East", "Mittel-Ost"), ("mediterranean", "Mediterranean region", "Mittelmeerregion")],
    "curve": [("standard", "Standard (ISO 834) curve", "Einheits-Temperaturzeitkurve (ISO 834)"), ("external", "External fire curve", "Außenbrandkurve"), ("hydrocarbon", "Hydrocarbon curve", "Kohlenwasserstoffkurve")],
    "deck": [("steel", "Steel deck", "Stahlüberbau"), ("composite", "Composite deck", "Verbundüberbau"), ("slab", "Concrete slab", "Betonplatte"), ("box", "Concrete box girder", "Betonhohlkasten"), ("beam", "Concrete beam", "Betonbalken")],
    "direction": [("vertical", "Vertical", "Vertikal"), ("horizontal", "Horizontal", "Horizontal"), ("jogging", "Jogging", "Joggen")],
    "group": [(g, f"Group {g[2:]}", f"Gruppe {g[2:]}") for g in ["gr1a", "gr1b", "gr2", "gr3", "gr4", "gr5"]],
    "hoist_class": [(f"HC{i}", f"Hoisting class HC{i}", f"Hubklasse HC{i}") for i in range(1, 5)],
    "material": [(v, e, d) for v, e, d in MATERIALS],
    "member": [("deck", "Bridge deck", "Brückenüberbau"), ("pier", "Pier", "Pfeiler")],
    "mode": [("framed", "Framed structure", "Rahmentragwerk"), ("core", "Core structure", "Kernbauwerk"), ("cantilever", "Cantilever structure", "Kragbauwerk"), ("lattice", "Lattice structure", "Gittertragwerk")],
    "model": [("sw0", "SW/0", "SW/0"), ("sw2", "SW/2", "SW/2")],
    "occupancy": OCCUPANCIES,
    "road": [("motorway", "Motorway", "Autobahn"), ("urban", "Urban road", "Innerortsstraße"), ("courtyard_cars", "Courtyard, cars", "Hof, Pkw"), ("courtyard_lorries", "Courtyard, lorries", "Hof, Lkw")],
    "roof_type": [("flat", "Flat roof", "Flachdach"), ("monopitch", "Monopitch roof", "Pultdach"), ("duopitch", "Duopitch roof", "Satteldach"), ("hipped", "Hipped roof", "Walmdach")],
    "state": [("fresh", "Fresh snow", "Neuschnee"), ("settled_days", "Settled snow, days old", "Setzungsschnee, Tage alt"), ("settled_months", "Settled snow, months old", "Setzungsschnee, Monate alt"), ("wet", "Wet snow", "Nassschnee")],
    "structure": [("concrete", "Concrete", "Beton"), ("steel", "Steel", "Stahl"), ("mixed", "Mixed", "Gemischt")],
    "surfacing": [("unsurfaced", "Unsurfaced", "Ohne Belag"), ("waterproofed", "Waterproofed", "Abgedichtet"), ("50mm", "50 mm surfacing", "Belag 50 mm"), ("100mm", "100 mm surfacing", "Belag 100 mm"), ("150mm", "150 mm surfacing", "Belag 150 mm"), ("ballast", "Ballast", "Schotter")],
    "tyres": [("pneumatic", "Pneumatic tyres", "Luftreifen"), ("solid", "Solid tyres", "Vollgummireifen")],
    "zone@E13.01,E13.02,E13.09": [(z, f"Snow load zone {z}", f"Schneelastzone {z}") for z in ["1", "1a", "2", "2a", "3"]],
    "zone@E14.08": [(z, f"Zone {z}", f"Bereich {z}") for z in "ABCDEFGHIJ"],
}
ALIASES = {"material2": "material", "material3": "material"}
FLAG = "r0:1"


def row(key, scope, kind, unit, example, flags, en, de, conf="high", note=""):
    return f"{key}|{scope}|{kind}|{unit}|{example}|{flags}|{en}|{de}|{conf}|{note}"


def flag(key, en, de, example="0"):
    return row(key, "", "n", "1", example, FLAG, en, de)


def choice(key, scope, example, en, de, note=""):
    return row(key, scope, "c", "-", example, "", en, de, "high", note)


NEW = {
    "a": [row("a", "", "n", "m", "0.2", "", "Contact length a", "Aufstandslänge a", note="E11.08, E11.09, E2.03, E2.06"), row("a", "E3.03", "n", "m", "20", "", "Distance a of the guide means", "Abstand a der Führungsmittel", "medium", "H_T = ξ M/a in e3_03")],
    "a_f": [row("a_f", "", "n", "m2", "50", "", "Floor area A_f", "Grundfläche A_f")],
    "a_fr": [row("a_fr", "", "n", "m2", "10", "", "Friction surface area A_fr", "Reibungsfläche A_fr")],
    "a_over_u": [row("a_over_u", "", "n", "m", "1", "", "Hydraulic radius A/U", "Hydraulischer Radius A/U")],
    "alarm": [flag("alarm", "Automatic alarm (1) or not (0)", "Automatische Alarmierung (1) oder nicht (0)")],
    "alpha": [
        row("alpha", "E13.03,E13.08,E13.13", "n", "deg", "20", "", "Roof pitch α", "Dachneigung α"),
        row("alpha", "E15.03,E15.05,E15.06", "n", "1", "1.2e-5", "", "Thermal expansion coefficient α in 1/K", "Wärmeausdehnungskoeffizient α in 1/K", note="no 1/K unit in UnitId"),
        row("alpha", "E2.14", "n", "1", "1", "r0.75:1.46", "Classification factor α", "Klassifizierungsbeiwert α"),
        row("alpha", "E2.15", "n", "1", "1", "", "Classification factor α", "Klassifizierungsbeiwert α"),
    ],
    "alpha_1": [row("alpha_1", "", "n", "deg", "15", "", "Roof pitch α₁", "Dachneigung α₁")],
    "alpha_2": [row("alpha_2", "", "n", "deg", "15", "", "Roof pitch α₂", "Dachneigung α₂")],
    "alpha_c": [row("alpha_c", "", "n", "W_m2K", "25", "", "Convective heat transfer coefficient α_c", "Wärmeübergangskoeffizient α_c")],
    "alpha_h": [row("alpha_h", "", "n", "1", "0", "", "Horizontal opening factor α_h = A_h/A_f", "Beiwert horizontaler Öffnungen α_h = A_h/A_f")],
    "alpha_v": [row("alpha_v", "", "n", "1", "0.1", "r0.025:0.25", "Vertical opening factor α_v = A_v/A_f", "Beiwert vertikaler Öffnungen α_v = A_v/A_f")],
    "area": [row("area", "", "n", "m2", "20", "", "Area A", "Fläche A", note="E11.04, E12.03, E14.08, E16.02")],
    "av_over_v": [row("av_over_v", "", "n", "per_m", "0.1", "", "Vent area ratio A_v/V", "Entlastungsflächenverhältnis A_v/V")],
    "axle": [row("axle", "", "n", "N", "400000", "", "Axle load", "Achslast")],
    "axle_load": [row("axle_load", "", "n", "N", "300000", "", "Axle load of the special vehicle", "Achslast des Sonderfahrzeugs")],
    "axles": [row("axles", "", "n", "1", "6", "", "Number of axle lines", "Anzahl der Achslinien")],
    "b": [
        row("b", "E12.02", "n", "J_m2sK", "1000", "r100:2200", "Thermal absorptivity b of the enclosure", "Wärmeeindringkoeffizient b der Umfassung"),
        row("b", "E12.04", "n", "J_m2sK", "1000", "", "Thermal absorptivity b of the enclosure", "Wärmeeindringkoeffizient b der Umfassung"),
        row("b", "", "n", "m", "", "", "Width b", "Breite b"),
    ],
    "b1": [row("b1", "", "n", "m", "5", "", "Width b₁", "Breite b₁")],
    "b2": [row("b2", "", "n", "m", "5", "", "Width b₂", "Breite b₂")],
    "beta_3": [row("beta_3", "", "n", "1", "0.5", "", "Factor β₃", "Beiwert β₃")],
    "c_h": [row("c_h", "", "n", "1", "1.15", "", "Discharge horizontal pressure factor C_h", "Entleerungs-Horizontaldruckbeiwert C_h")],
    "c_w": [row("c_w", "", "n", "1", "1.1", "", "Discharge wall friction factor C_w", "Entleerungs-Wandreibungsbeiwert C_w")],
    "capacity_t": [row("capacity_t", "", "n", "t", "500", "", "Silo capacity", "Siloinhalt")],
    "category": [choice("category", "", "B2", "Use category", "Nutzungskategorie", "union of the EN and DE category lists; the evaluator rejects those of the other annex")],
    "class": [row("class", "E11.07", "n", "1", "2", "r1:6", "Forklift class FL (1 to 6)", "Gabelstaplerklasse FL (1 bis 6)")],
    "cen_zone": [choice("cen_zone", "", "central_east", "CEN snow zone", "CEN-Schneezone")],
    "coast": [row("coast", "E14.02", "n", "1", "0", FLAG, "Coastal profile (1) or inland (0)", "Küstenprofil (1) oder Binnenland (0)")],
    "consequence": [row("consequence", "E17.07", "v", "1", "", "", "Consequence C(D) per hazard", "Folge C(D) je Gefahr")],
    "cover": [row("cover", "", "n", "m", "0.1", "", "Cover thickness", "Überdeckungsdicke")],
    "curve": [choice("curve", "", "standard", "Temperature-time curve", "Temperatur-Zeit-Kurve")],
    "deck": [choice("deck", "", "composite", "Bridge deck type", "Brückenüberbautyp")],
    "deck_acc": [],
    "deck_type": [row("deck_type", "E15.01", "n", "1", "3", "r1:3", "Deck type (1 steel, 2 composite, 3 concrete)", "Überbautyp (1 Stahl, 2 Verbund, 3 Beton)")],
    "d": [
        row("d", "E12.05", "n", "m", "1", "", "Diameter D of the fire", "Durchmesser D des Brandes"),
        row("d", "E14.07,E14.08", "n", "m", "20", "", "Depth d of the building", "Tiefe d des Gebäudes"),
    ],
    "delta_b": [row("delta_b", "", "n", "m", "0.1", "", "Deformation of the barrier δ_b", "Verformung der Absturzsicherung δ_b")],
    "delta_c": [row("delta_c", "", "n", "m", "0.1", "", "Deformation of the vehicle δ_c", "Verformung des Fahrzeugs δ_c")],
    "depth": [row("depth", "", "n", "m", "5", "", "Depth", "Tiefe")],
    "direction": [choice("direction", "", "vertical", "Direction of the excitation", "Anregungsrichtung")],
    "dm_over_m": [row("dm_over_m", "", "n", "1", "0.5", "", "Mass ratio Δm/m", "Massenverhältnis Δm/m")],
    "e": [
        row("e", "E15.03,E15.05", "n", "Pa", "3e10", "", "Modulus of elasticity E", "Elastizitätsmodul E"),
        row("e", "E14.08", "n", "m", "10", "", "Reference dimension e = min(b, 2h)", "Bezugsmaß e = min(b, 2h)"),
    ],
    "eccentricity": [row("eccentricity", "", "n", "m", "0.01", "", "Eccentricity", "Exzentrizität")],
    "f_pl": [row("f_pl", "", "n", "N", "1e6", "", "Plastic resistance F_pl", "Plastische Tragfähigkeit F_pl")],
    "finish": [row("finish", "E11.02", "n", "N_m", "0", "", "Finish load per metre of wall", "Ausbaulast je Meter Wand")],
    "g_k": [row("g_k", "", "n", "Pa", "5000", "", "Characteristic permanent action g_k", "Charakteristische ständige Einwirkung g_k", note="E17.06 divides by 1000 to kN/m²; E12.07 uses only the ratio")],
    "gamma": [row("gamma", "", "n", "N_m3", "25000", "", "Unit weight γ", "Wichte γ", note="E11.01, E16.02, E4.01, E4.03 to E4.05, E4.08")],
    "group": [choice("group", "", "gr1a", "Traffic load group", "Verkehrslastgruppe")],
    "heat_detection": [flag("heat_detection", "Automatic heat detection (1) or not (0)", "Automatische Wärmemeldung (1) oder nicht (0)")],
    "hoist_class": [choice("hoist_class", "", "HC2", "Hoisting class", "Hubklasse")],
    "i": [row("i", "", "n", "m4", "0.001", "", "Second moment of area I", "Flächenträgheitsmoment I")],
    "k": [
        row("k", "E17.08", "n", "N_m", "300000", "", "Equivalent stiffness k of the vehicle", "Ersatzsteifigkeit k des Fahrzeugs"),
        row("k", "E4.03,E4.04,E4.05", "n", "1", "", "", "Lateral pressure ratio K", "Horizontaldruckverhältnis K"),
    ],
    "l": [row("l", "", "n", "m", "6", "", "Tie length L", "Zuggliedlänge L")],
    "l_f": [row("l_f", "", "n", "m", "50", "", "Influence length L_f", "Einflusslänge L_f")],
    "l_h": [row("l_h", "", "n", "m", "5", "", "Flame height L_h", "Flammenhöhe L_h")],
    "lambda": [row("lambda", "", "n", "1", "0.5", "", "Damage equivalence factor λ", "Schadensäquivalenzfaktor λ")],
    "lane": [row("lane", "", "n", "1", "1", "r1:3", "Lane number (1 to 3)", "Fahrstreifennummer (1 bis 3)")],
    "load": [row("load", "", "n", "N", "40000", "", "Load Q", "Last Q")],
    "m_c": [row("m_c", "", "n", "kg", "20000", "", "Crab mass m_c", "Katzmasse m_c")],
    "m_drive": [row("m_drive", "", "n", "Nm", "100000", "", "Moment M for the transverse force H_T = ξ M/a", "Moment M für die Querkraft H_T = ξ M/a", "medium", "unit follows from H_T = ξ M/a in N with a in m; not a count")],
    "m_e": [row("m_e", "", "n", "kg_m", "100", "", "Equivalent mass per length m_e", "Äquivalente Masse je Länge m_e")],
    "margin_k": [row("margin_k", "", "n", "K", "0", "", "Temperature margin ΔT_margin", "Temperaturzuschlag ΔT_margin")],
    "mass": [row("mass", "", "n", "kg", "1500", "", "Mass", "Masse")],
    "material": [choice("material", "", "concrete", "Material", "Werkstoff")],
    "material2": [choice("material2", "", "concrete", "Material of layer 2", "Werkstoff der Schicht 2")],
    "material3": [choice("material3", "", "concrete", "Material of layer 3", "Werkstoff der Schicht 3")],
    "member": [choice("member", "", "deck", "Member type", "Bauteilart")],
    "mode": [choice("mode", "", "framed", "Mode shape of the structure", "Eigenform des Bauwerks")],
    "model": [choice("model", "", "sw0", "Rail load model", "Eisenbahn-Lastmodell")],
    "mu": [
        row("mu", "E13.01,E13.12", "n", "1", "0.8", "", "Shape coefficient μ", "Formbeiwert μ"),
        row("mu", "E4.03,E4.04,E4.05", "n", "1", "", "", "Wall friction coefficient μ", "Wandreibungsbeiwert μ"),
    ],
    "n": [row("n", "", "n", "Hz", "1", "", "Natural frequency n of the structure", "Eigenfrequenz n des Bauwerks")],
    "occupancy": [choice("occupancy", "", "office", "Occupancy", "Nutzung")],
    "offsite_brigade": [flag("offsite_brigade", "Off-site fire brigade (1) or not (0)", "Werkfeuerwehr extern (1) oder nicht (0)")],
    "onsite_brigade": [flag("onsite_brigade", "On-site fire brigade (1) or not (0)", "Werkfeuerwehr (1) oder nicht (0)")],
    "p": [row("p", "", "n", "1", "0.02", "r0:1", "Annual probability p", "Jährliche Wahrscheinlichkeit p")],
    "p_d_h": [row("p_d_h", "E17.07", "v", "1", "", "r0:1", "Probability of damage per hazard", "Schadenswahrscheinlichkeit je Gefahr")],
    "p_h": [row("p_h", "E17.07", "v", "1", "", "r0:1", "Probability of the hazard", "Gefährdungswahrscheinlichkeit")],
    "p_v": [row("p_v", "", "n", "kPa", "5", "", "Venting pressure p_v", "Entlastungsdruck p_v")],
    "pavement": [row("pavement", "", "n", "m", "0.1", "", "Pavement thickness", "Belagdicke")],
    "phi": [
        row("phi", "E14.03", "n", "1", "0.1", "", "Upwind slope Φ = H/L", "Luvhangneigung Φ = H/L"),
        row("phi", "E4.02,E4.03,E4.04,E4.05", "n", "deg", "30", "", "Angle of internal friction φ", "Innerer Reibungswinkel φ"),
    ],
    "pitch": [row("pitch", "", "n", "deg", "20", "", "Roof pitch", "Dachneigung")],
    "provided_tie": [row("provided_tie", "", "n", "N", "80000", "", "Provided tie force", "Vorhandene Zugankerkraft")],
    "q_c": [row("q_c", "", "n", "N", "100000", "", "Crane load Q_c", "Kranlast Q_c")],
    "q_ca": [row("q_ca", "", "n", "Pa", "1000", "", "Construction load Q_ca", "Baustellenlast Q_ca")],
    "q_ca_concreting": [row("q_ca_concreting", "", "n", "Pa", "750", "", "Construction load during concreting", "Baustellenlast beim Betonieren")],
    "q_ci": [row("q_ci", "E16.06", "v", "N", "", "", "Construction load Q_c,i", "Baulast Q_c,i")],
    "q_h": [row("q_h", "", "n", "N", "50000", "", "Hoisted load Q_h", "Hublast Q_h")],
    "q_k": [row("q_k", "", "n", "Pa", "3000", "", "Characteristic imposed load q_k", "Charakteristische Nutzlast q_k", note="E17.06 divides by 1000 to kN/m²; E12.07 uses only the ratio")],
    "q_max": [row("q_max", "", "n", "N", "100000", "", "Maximum wheel load Q_max", "Maximale Radlast Q_max")],
    "q_v": [row("q_v", "", "n", "N", "600000", "", "Vertical load Q_v", "Vertikallast Q_v")],
    "q_vk": [row("q_vk", "", "n", "N", "1e6", "", "Characteristic vertical load Q_vk", "Charakteristische Vertikallast Q_vk")],
    "q_w": [row("q_w", "", "n", "W", "1e6", "", "Heat release rate Q", "Wärmefreisetzungsrate Q")],
    "r": [
        row("r", "E12.05", "n", "m", "2", "", "Horizontal distance r from the fire axis", "Horizontaler Abstand r von der Brandachse"),
        row("r", "E14.11", "n", "m", "0", "", "Corner radius r", "Eckenradius r"),
    ],
    "r_acc": [row("r_acc", "", "n", "1", "1e-5", "", "Accepted risk", "Akzeptiertes Risiko")],
    "road": [choice("road", "", "motorway", "Road type", "Straßenart")],
    "roof_type": [choice("roof_type", "", "duopitch", "Roof type", "Dachform")],
    "rpm": [row("rpm", "", "n", "1", "1500", "", "Rotational speed in rpm (1/min)", "Drehzahl in U/min")],
    "runway_beams": [row("runway_beams", "", "n", "1", "2", "", "Number of runway beams", "Anzahl der Kranbahnträger")],
    "s": [
        row("s", "E13.07,E13.08,E13.13", "n", "Pa", "2000", "", "Snow load s on the roof", "Schneelast s auf dem Dach"),
        row("s", "E17.06", "n", "m", "5", "", "Tie spacing s", "Zugliedabstand s"),
        row("s", "E14.03", "n", "1", "1", "", "Orography scale factor s", "Orographie-Faktor s"),
    ],
    "s_b": [row("s_b", "", "n", "N_m", "1e7", "", "Buffer spring stiffness S_B", "Federsteifigkeit S_B des Puffers")],
    "safe_access": [flag("safe_access", "Safe access routes (1) or not (0)", "Sichere Zugangswege (1) oder nicht (0)")],
    "smoke_detection": [flag("smoke_detection", "Automatic smoke detection (1) or not (0)", "Automatische Rauchmeldung (1) oder nicht (0)")],
    "sprinkler": [flag("sprinkler", "Sprinkler system (1) or not (0)", "Sprinkleranlage (1) oder nicht (0)")],
    "state": [choice("state", "", "fresh", "Snow state", "Schneezustand")],
    "structure": [choice("structure", "", "concrete", "Structure type", "Bauwerksart")],
    "sum_q_r": [row("sum_q_r", "", "n", "N", "400000", "", "Sum of wheel loads ΣQ_r", "Summe der Radlasten ΣQ_r")],
    "surfacing": [choice("surfacing", "", "50mm", "Surfacing type", "Belagart")],
    "t_min": [row("t_min", "", "n", "min", "30", "", "Time in minutes", "Zeit in Minuten")],
    "temps_c": [row("temps_c", "E12.10", "v", "C", "", "", "Temperature series (vector)", "Temperaturreihe (Vektor)")],
    "terrain": [row("terrain", "", "n", "1", "2", "r0:4", "Terrain category (0 to 4)", "Geländekategorie (0 bis 4)")],
    "times_min": [row("times_min", "E12.10", "v", "min", "", "", "Time series in minutes (vector)", "Zeitreihe in Minuten (Vektor)")],
    "tyres": [choice("tyres", "", "pneumatic", "Tyre type", "Reifenart")],
    "v": [row("v", "", "n", "1", "0.4", "", "Coefficient of variation V of the annual maxima", "Variationskoeffizient V der Jahresmaxima")],
    "v_1": [row("v_1", "", "n", "m_s", "0.7", "", "Buffer impact speed v₁", "Pufferanprallgeschwindigkeit v₁")],
    "water_supplies": [row("water_supplies", "", "n", "1", "0", "r0:2", "Number of water supplies (0 to 2)", "Anzahl der Löschwasserversorgungen (0 bis 2)")],
    "weight": [row("weight", "", "n", "N", "30000", "", "Weight", "Gewicht")],
    "wheels": [row("wheels", "", "n", "1", "4", "", "Number of wheels", "Anzahl der Räder")],
    "wall_gamma": [row("wall_gamma", "", "n", "N_m3", "18000", "", "Unit weight of the wall material", "Wichte des Wandbaustoffs")],
    "wall_load": [row("wall_load", "", "n", "N_m", "3000", "", "Wall load per metre", "Wandlast je Meter")],
    "xi": [row("xi", "", "n", "1", "0.2", "", "Factor ξ", "Beiwert ξ")],
    "y_allowed": [row("y_allowed", "", "n", "m", "0.5", "", "Allowed displacement", "Zulässige Verschiebung")],
    "y_el": [row("y_el", "", "n", "m", "0.1", "", "Elastic displacement", "Elastische Verschiebung")],
    "z": [
        row("z", "E13.02", "n", "1", "3", "", "CEN snow zone number Z", "CEN-Schneezonenzahl Z"),
        row("z", "", "n", "m", "", "", "Height z", "Höhe z"),
    ],
    "z_g": [row("z_g", "", "n", "m", "3", "", "Height z_g", "Höhe z_g")],
    "z_prime": [row("z_prime", "", "n", "m", "0", "", "Height z'", "Höhe z'")],
    "zone": [
        choice("zone", "E13.01,E13.02,E13.09", "2", "Snow load zone", "Schneelastzone"),
        row("zone", "E14.01,E14.02", "n", "1", "2", "r1:4", "Wind zone (1 to 4)", "Windzone (1 bis 4)"),
        choice("zone", "E14.08", "A", "Zone of the face", "Bereich der Fläche"),
    ],
}

lines = TSV.read_text(encoding="utf-8").splitlines()
out = []
done = set()
for line in lines:
    key = line.split("|")[0]
    if line.startswith("#") or key not in NEW:
        out.append(line)
    elif key not in done:
        out.extend(NEW[key])
        done.add(key)
missing = set(NEW) - done
assert not missing, missing
for name, items in CHOICES.items():
    out.extend(f"@choice {name}|{value}|{en}|{de}" for value, en, de in items)
out.extend(f"@alias {name}|{source}" for name, source in ALIASES.items())
TSV.write_text("\n".join(out) + "\n", encoding="utf-8")
print(len(out), "lines")
