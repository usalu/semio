#!/usr/bin/env python3
"""🏷️ W2-S norm label tables: en/de `x-semio-ui` for every snapshot record field and enum option the norm mutation leaves reach,
plus the leaf inputs the W2-R rollout could not annotate (EN 1990 stubs, record-valued inputs). Data only; read by
`🧪️w2-s-norm-parity.py`.
"""

ID_BASE = "https://json.schemas.assets.semio-tech.com/"

#: 🆔️ Snapshot facet `$id`s that differ from `<base>s/norm/<artifact>/snapshot.json`.
SNAPSHOT_IDS = {"din18599": f"{ID_BASE}s/norm/din18599/snapshot/schema.json"}

#: 🆔️ Artifact schema documents (relative to the subset) whose `$id` leaves the catalogue scheme, with the canonical one.
DOCUMENT_IDS = {
    "din16798": {"🧬️schema/🔣️.json": f"{ID_BASE}s/norm/din16798/artifact.json", "🧬️schema/🔺️diff/🔣️.json": f"{ID_BASE}s/norm/din16798/diff.json"},
    "din4108": {"🧬️schema/🔣️.json": f"{ID_BASE}s/norm/din4108/artifact.json", "🧬️schema/🔺️diff/🔣️.json": f"{ID_BASE}s/norm/din4108/diff.json"},
    "en1993": {"🧬️schema/🔣️.json": f"{ID_BASE}s/norm/en1993/artifact.json", "🧬️schema/🔺️diff/🔣️.json": f"{ID_BASE}s/norm/en1993/diff.json"},
    "en1996": {"🧬️schema/🔣️.json": f"{ID_BASE}s/norm/en1996/artifact.json", "🧬️schema/🔺️diff/🔣️.json": f"{ID_BASE}s/norm/en1996/diff.json"},
    "en1998": {"🧬️schema/🔣️.json": f"{ID_BASE}s/norm/en1998/artifact.json", "🧬️schema/🔺️diff/🔣️.json": f"{ID_BASE}s/norm/en1998/diff.json"},
    "en1999": {"🧬️schema/🔺️diff/🔣️.json": f"{ID_BASE}s/norm/en1999/diff.json"},
    "en1995": {"🧬️schema/🧬️mutations/📝️text/🔣️.json": f"{ID_BASE}s/norm/en1995/mutations/text.json"},
}

#: 🐫 Records and enums outside the leaf structs whose value wire moves to camelCase (their serde test twin already is, or the
#: record is payload-only): DIN V 18599 `MonthlyClimate`, and the VDI 3805 types whose `serde` derive was camelCase while `value`
#: emitted Rust spelling.
CAMEL_VALUE = {
    "din18599": ["MonthlyClimate"],
    "vdi3805": ["VdiQuantityKind", "ExtensionBag", "EditionId", "SchemaStatus", "Domain", "Diagnostic", "Severity", "EditionProfileChoice"],
}

#: 🐫 Types whose `value` wire already is camelCase but whose `serde` test twin was not (the fixtures now follow the wire).
CAMEL_SERDE = {"iso16757": ["CatalogueId", "DictionaryRef", "Names", "DimensionSignature", "CatalogueUnit", "Cardinality", "CatalogueReference", "ExtensionBag", "Lifecycle", "Manufacturer", "ProductGroup", "ProductClass", "ProductSeries", "ParameterDomain", "PropertyDefinition", "PropertyValue", "ProductVariant", "Product", "ProductIndex", "AccessoryRelationship", "CompositionRelationship", "GeometryReference", "DescriptiveObject", "Catalogue", "CatalogueMetadata", "SelectionConstraint", "SelectionRequest", "SelectionResult", "BimEmbedding", "PortDefinition", "BoundingBox", "PrimitiveKind", "GeometryObject", "SpaceEnvelope", "SurfaceDefinition", "GeometryCatalogue", "Subject", "Relationship", "DictionaryProperty", "ControlledValueList", "ValueConstraint", "Dictionary", "Iso12006Mapping", "ExternalMedia", "IfcCatalogueNode", "IfcCatalogue", "ScriptLimits", "ScriptResult"]}

#: 🗑️ Fixture cases whose kind name survives but whose snapshots and payload still encode the flat pre-652 model (the root
#: snapshot keys are no fields of the current snapshot): orphans in substance, deleted like the renamed-away kinds.
STALE_MODEL_CASES = {
    "en1990": ["⚠️change-consequence-class/🏗️escalates-the-building-from-cc2-to-cc3", "🌍️change-annex/🌐️switches-the-national-annex-from-de-to-en"],
    "din16798": ["🌍️change-annex/🌍️switches-the-check-to-the-en-annex"],
    "en1996": ["🎭️change-design-situation/🌋️switches-the-design-situation-to-seismic", "🧲️change-mu/🧲️raises-the-bed-joint-friction-coefficient-to-0-625", "🏢️change-storeys/🏢️adds-a-third-storey-at-the-simplified-method-limit", "🏭️change-masonry-class/🏭️downgrades-manufacturing-control-to-class-4", "🌍️change-annex/🌍️switches-from-the-german-na-to-the-recommended-en-annex", "💧️change-exposure/💧️moves-the-wall-to-exposure-class-mx3"],
    "en1998": ["🌍️change-annex/🌍️switches-annex-to-en"],
}

#: 🧫️ Artifacts whose committed fixtures follow the `serde` test derive rather than the `value` wire.
FIXTURE_SOURCE = {"iso16757": "serde", "vdi3805": "serde"}

#: 🐫 Types whose source-side (current) wire is already camelCase beyond their declared attributes.
SOURCE_CAMEL = {}

#: 📸️ Snapshot and diff types of the artifacts whose fixtures are re-encoded wholesale.
SNAPSHOT_TYPES = {"iso16757": "Iso16757Snapshot", "vdi3805": "Vdi3805Snapshot", "en1994": "En1994Snapshot"}
DIFF_TYPES = {"iso16757": "Iso16757Diff", "vdi3805": "Vdi3805Diff", "en1994": "En1994Diff"}

#: 🩹️ Values for Rust fields a stale fixture record lacks although FromValue requires them (EN 1994 actions predate the sole
#: concentrated force `f_k_n`; their area load `q_area_pa` is set, so the concentrated force is zero — never both).
FIXTURE_FILL = {"en1994": {"CharacteristicAction": {"fKN": 0.0}}}

#region 🔖️Helpers


def L(en, de, **extra):
    """🏷️ An `x-semio-ui` body: the en/de label plus any further keys."""
    return {"label": {"en": en, "de": de}, **{key: value for key, value in extra.items() if value is not None}}


def D(en, de):
    return {"en": en, "de": de}


def num(en, de, unit=None, display=None, factor=None, precision=None, step=None, desc=None):
    return L(en, de, description=desc, widget="stepper", unit=unit, displayUnit=display, displayFactor=factor, step=step, precision=precision)


def force(en, de, desc=None):
    return num(en, de, "N", "kN", 0.001, 2, desc=desc)


def line_load(en, de, desc=None):
    return num(en, de, "N/m", "kN/m", 0.001, 2, desc=desc)


def area_load(en, de, desc=None):
    return num(en, de, "Pa", "kN/m²", 0.001, 2, desc=desc)


def moment(en, de, desc=None):
    return num(en, de, "N·m", "kN·m", 0.001, 2, desc=desc)


def stress(en, de, desc=None):
    return num(en, de, "Pa", "N/mm²", 1e-6, 1, desc=desc)


def metre(en, de, desc=None, precision=3):
    return num(en, de, "m", precision=precision, desc=desc)


def mm(en, de, desc=None):
    return num(en, de, "m", "mm", 1000, 1, desc=desc)


def square(en, de, desc=None):
    return num(en, de, "m²", precision=2, desc=desc)


def pure(en, de, desc=None, precision=2):
    return num(en, de, precision=precision, desc=desc)


def count(en, de, desc=None):
    return num(en, de, step=1, precision=0, desc=desc)


def text(en, de, desc=None):
    return L(en, de, description=desc, widget="text")


def toggle(en, de, desc=None):
    return L(en, de, description=desc, widget="toggle")


def ref(kind, en, de, desc=None):
    return L(en, de, description=desc, widget="reference", role="value", ref={"kind": kind})


def rows(en, de, desc=None):
    return L(en, de, description=desc)


ANNEX = {"En": ("Recommended CEN values (EN)", "Empfohlene CEN-Werte (EN)"), "De": ("German national annex (DIN EN)", "Deutscher Nationaler Anhang (DIN EN)")}
LABEL_EN = text("Label (English)", "Bezeichnung (Englisch)")
LABEL_DE = text("Label (German)", "Bezeichnung (Deutsch)")
#endregion 🔖️Helpers


#: 📏️ Hard bounds on snapshot-record fields that a committed refusal witnesses as a range invariant.
DEF_BOUNDS = {"din18599": {"MonthlyClimate": {"gHWM2": {"items": {"minimum": 0}}}}}

#: 🏷️ `x-semio-ui` per artifact → record/enum → wire field (or `Variant.field`).
DEFS = {
    "en1990": {
        "PermanentAction": {"gk": force("Characteristic permanent action G_k", "Charakteristischer Wert der ständigen Einwirkung G_k")},
        "VariableAction": {
            "category": text("Action category", "Einwirkungskategorie", D("EN 1990 Table A1.1 category selecting ψ₀, ψ₁, ψ₂ (e.g. Q_imposed_A, Q_snow, Q_wind).", "Kategorie nach EN 1990 Tabelle A1.1, die ψ₀, ψ₁, ψ₂ bestimmt (z. B. Q_imposed_A, Q_snow, Q_wind).")),
            "qk": force("Characteristic variable action Q_k", "Charakteristischer Wert der veränderlichen Einwirkung Q_k"),
        },
        "AccidentalAction": {"ad": force("Design accidental action A_d", "Bemessungswert der außergewöhnlichen Einwirkung A_d")},
        "SeismicAction": {
            "aEk": force("Characteristic seismic action A_Ek", "Charakteristischer Wert der Erdbebeneinwirkung A_Ek"),
            "importanceClass": L("Importance class", "Bedeutungskategorie", widget="segmented"),
        },
        "Member": {
            "labelEn": LABEL_EN,
            "labelDe": LABEL_DE,
            "rdStr": force("Design resistance R_d (STR)", "Bemessungswert des Widerstands R_d (STR)"),
            "rdGeo": force("Design resistance R_d (GEO)", "Bemessungswert des Widerstands R_d (GEO)"),
            "rdEquStab": force("Stabilising design effect (EQU)", "Bemessungswert der stabilisierenden Einwirkung (EQU)"),
            "rdEquDestab": force("Destabilising design effect (EQU)", "Bemessungswert der destabilisierenden Einwirkung (EQU)"),
            "rdFat": force("Design fatigue resistance (FAT)", "Bemessungswert des Ermüdungswiderstands (FAT)"),
            "span": metre("Span l", "Stützweite l"),
            "deflectionW": mm("Deflection w", "Durchbiegung w"),
            "deflectionLimitRatio": pure("Deflection limit l/…", "Durchbiegungsgrenze l/…", D("Denominator of the admissible deflection l/n.", "Nenner der zulässigen Durchbiegung l/n."), 0),
            "vibrationFrequency": num("Natural frequency f", "Eigenfrequenz f", "Hz", precision=2),
            "vibrationFrequencyMin": num("Minimum natural frequency f_min", "Mindesteigenfrequenz f_min", "Hz", precision=2),
        },
        "BridgeSls": {
            "memberId": ref("member", "Member", "Bauteil"),
            "deckAcceleration": num("Deck acceleration a", "Überbaubeschleunigung a", "m/s²", precision=2),
            "deckAccelerationLimit": num("Admissible deck acceleration a_max", "Zulässige Überbaubeschleunigung a_max", "m/s²", precision=2),
            "deckTwist": num("Deck twist t", "Verwindung des Überbaus t", "rad", precision=4),
            "deckTwistLimit": num("Admissible deck twist t_max", "Zulässige Verwindung t_max", "rad", precision=4),
            "bridgeDeflection": mm("Vertical deflection δ", "Vertikale Durchbiegung δ"),
            "bridgeDeflectionLimit": mm("Admissible vertical deflection δ_max", "Zulässige vertikale Durchbiegung δ_max"),
        },
        "MemberEffect": {
            "memberId": ref("member", "Member", "Bauteil"),
            "actionId": ref("action", "Action", "Einwirkung"),
            "influence": pure("Influence coefficient", "Einflussbeiwert", D("Factor mapping the action magnitude onto the member effect.", "Faktor, der die Größe der Einwirkung auf die Beanspruchung des Bauteils abbildet."), 3),
        },
    },
    "din16798": {
        "VentSystemDocument": {
            "humidificationRequiredKgH": num("Required humidification", "Erforderliche Befeuchtungsleistung", "kg/h", precision=2),
            "humidificationProvidedKgH": num("Provided humidification", "Vorhandene Befeuchtungsleistung", "kg/h", precision=2),
            "fanQVM3S": num("Fan volume flow q_v", "Ventilatorvolumenstrom q_v", "m³/s", precision=3),
            "fanTRunH": num("Daily fan runtime t", "Tägliche Ventilatorlaufzeit t", "h", precision=1),
            "ductTestPressurePa": num("Duct test pressure", "Prüfdruck der Luftleitung", "Pa", precision=0),
        },
    },
    "din4108": {
        "LayerDocument": {
            "density": num("Bulk density ρ", "Rohdichte ρ", "kg/m³", precision=0),
            "waterClass": text("Water-absorption class", "Wasseraufnahmeklasse", D("DIN 4108-10 class wk, wf or wd.", "Klasse nach DIN 4108-10: wk, wf oder wd.")),
            "tensileClass": text("Tensile-strength class", "Zugfestigkeitsklasse", D("DIN 4108-10 class tk or tf.", "Klasse nach DIN 4108-10: tk oder tf.")),
            "acousticClass": text("Acoustic class", "Schalltechnische Klasse", D("DIN 4108-10 class sh, sm or sg.", "Klasse nach DIN 4108-10: sh, sm oder sg.")),
            "segments": rows("Inhomogeneous segments", "Inhomogene Teilbereiche", D("Parallel material segments of an inhomogeneous layer (DIN EN ISO 6946).", "Parallele Materialbereiche einer inhomogenen Schicht (DIN EN ISO 6946).")),
        },
        "ThermalZone": {"windows": rows("Windows", "Fenster")},
        "LayerSegment": {
            "fraction": num("Area fraction", "Flächenanteil", "1", "%", 100, 0),
            "density": num("Bulk density ρ", "Rohdichte ρ", "kg/m³", precision=0),
        },
    },
    "en1996": {
        "MasonryWall": {"openings": rows("Openings", "Öffnungen"), "loadCases": rows("Load cases", "Lastfälle")},
    },
    "din18599": {
        "Renewables": {
            "pvAreaM2": square("Photovoltaic module area", "Fläche der Photovoltaikmodule"),
            "pvEfficiency": num("Photovoltaic module efficiency η_PV", "Modulwirkungsgrad η_PV", "1", "%", 100, 1),
            "solarThermalKwhA": num("Solar thermal yield", "Solarthermischer Ertrag", "kWh/a", precision=0),
        },
        "CoolingSystem": {"plant": rows("Cooling plant", "Kälteerzeuger")},
        "CoolingPlant": {
            "eer": pure("Energy efficiency ratio EER", "Leistungszahl EER"),
            "energyCarrier": text("Energy carrier", "Energieträger"),
        },
        "MonthlyClimate": {
            "thetaEC": L("Monthly mean outdoor temperature θ_e", "Monatsmittelwert der Außentemperatur θ_e", description=D("Twelve values, January to December.", "Zwölf Werte, Januar bis Dezember."), unit="°C", step=0.1),
            "gHWM2": L("Monthly mean global irradiance on the horizontal", "Monatsmittelwert der Globalstrahlung auf die Horizontale", description=D("Twelve values, January to December.", "Zwölf Werte, Januar bis Dezember."), unit="W/m²", step=1),
        },
        "VentilationSystem": {
            "airflowM3H": num("Air volume flow", "Luftvolumenstrom", "m³/h", precision=0),
            "heatRecoveryEta": num("Heat-recovery efficiency η_WRG", "Wärmerückgewinnungsgrad η_WRG", "1", "%", 100, 0),
            "fanPowerW": num("Fan power", "Ventilatorleistung", "W", precision=0),
        },
        "LightingSystem": {"controlFactor": pure("Lighting control factor", "Faktor der Beleuchtungssteuerung")},
        "HeatingSystem": {
            "generationEfficiency": num("Generation efficiency", "Erzeugernutzungsgrad", "1", "%", 100, 0),
            "distributionEfficiency": num("Distribution efficiency", "Verteilnutzungsgrad", "1", "%", 100, 0),
            "storageEfficiency": num("Storage efficiency", "Speichernutzungsgrad", "1", "%", 100, 0),
            "transferEfficiency": num("Transfer efficiency", "Übergabenutzungsgrad", "1", "%", 100, 0),
            "energyCarrier": text("Energy carrier", "Energieträger"),
        },
        "DhwSystem": {
            "specificDemandKwhPersonA": num("Specific hot-water demand", "Spezifischer Trinkwarmwasserbedarf", "kWh/(Person·a)", precision=0),
            "storageLossKwhA": num("Storage loss", "Speicherverlust", "kWh/a", precision=0),
            "distributionLossKwhA": num("Distribution loss", "Verteilverlust", "kWh/a", precision=0),
            "energyCarrier": text("Energy carrier", "Energieträger"),
        },
        "ThermalZone": {
            "labelEn": LABEL_EN,
            "labelDe": LABEL_DE,
            "usageProfile": L("Usage profile", "Nutzungsprofil", widget="segmented"),
            "areaM2": square("Reference floor area A_NGF", "Nettogrundfläche A_NGF"),
            "volumeM3": num("Net air volume V", "Nettoluftvolumen V", "m³", precision=0),
            "thetaIHeatC": num("Heating set-point θ_i,h", "Raum-Solltemperatur Heizen θ_i,h", "°C", precision=1),
            "thetaICoolC": num("Cooling set-point θ_i,c", "Raum-Solltemperatur Kühlen θ_i,c", "°C", precision=1),
            "occupants": count("Occupants", "Personen"),
            "internalGainsWM2": num("Internal heat gains q_I", "Interne Wärmequellen q_I", "W/m²", precision=1),
            "lightingPowerWM2": num("Installed lighting power", "Installierte Beleuchtungsleistung", "W/m²", precision=1),
        },
        "EnvelopeElement": {
            "labelEn": LABEL_EN,
            "labelDe": LABEL_DE,
            "areaM2": square("Area A", "Fläche A"),
            "orientationDeg": num("Orientation (azimuth)", "Orientierung (Azimut)", "°", precision=0),
            "tiltDeg": num("Tilt", "Neigung", "°", precision=0),
            "gValue": num("Total solar energy transmittance g", "Gesamtenergiedurchlassgrad g", "1", precision=2),
            "fc": num("Shading reduction factor F_C", "Abminderungsfaktor für Sonnenschutz F_C", "1", precision=2),
            "adjacency": L("Adjacent space", "Angrenzender Bereich", widget="select"),
        },
    },
    "en1991": {
        "AccidentalCase": {"impact": rows("Impact", "Anprall"), "explosion": rows("Explosion", "Explosion")},
        "FloorArea": {
            "category": text("Category of use", "Nutzungskategorie", D("EN 1991-1-1 Table 6.1 category (A to H).", "Kategorie nach EN 1991-1-1 Tabelle 6.1 (A bis H).")),
            "area": square("Loaded area A", "Belastete Fläche A"),
            "assumedQkConcentrated": force("Assumed concentrated load Q_k", "Angesetzte Einzellast Q_k"),
            "assumedPartitions": area_load("Assumed partition allowance", "Angesetzter Trennwandzuschlag"),
        },
        "RoofArea": {
            "roofType": text("Roof type", "Dachform", D("monopitch, duopitch, flat …", "Pultdach, Satteldach, Flachdach …")),
            "pitchDeg": num("Roof pitch α", "Dachneigung α", "°", precision=0),
            "cE": pure("Exposure coefficient C_e", "Umgebungskoeffizient C_e"),
            "cT": pure("Thermal coefficient C_t", "Temperaturkoeffizient C_t"),
            "hasParapet": toggle("Parapet present", "Attika vorhanden"),
            "parapetHeight": metre("Parapet height", "Attikahöhe", precision=2),
            "driftObstructionHeight": metre("Height of the drift obstruction", "Höhe des Verwehungshindernisses", precision=2),
            "multiSpan": toggle("Multi-span roof", "Mehrfeldriges Dach"),
        },
        "SelfWeightElement": {"thickness": mm("Thickness", "Dicke")},
        "WindFace": {
            "cPe10": pure("External pressure coefficient c_pe,10", "Außendruckbeiwert c_pe,10"),
            "cPe1": pure("External pressure coefficient c_pe,1", "Außendruckbeiwert c_pe,1"),
            "cPi": pure("Internal pressure coefficient c_pi", "Innendruckbeiwert c_pi"),
            "cS": pure("Size factor c_s", "Größenbeiwert c_s"),
            "cD": pure("Dynamic factor c_d", "Dynamischer Beiwert c_d"),
            "loadedArea": square("Loaded area", "Lasteinzugsfläche"),
        },
        "AccidentalImpact": {
            "vehicleMass": num("Vehicle mass m", "Fahrzeugmasse m", "kg", precision=0),
            "vehicleSpeed": num("Vehicle speed v", "Fahrzeuggeschwindigkeit v", "m/s", precision=1),
        },
        "AccidentalExplosion": {
            "explosionMass": num("Explosive charge mass", "Masse der Sprengladung", "kg", precision=1),
            "standoff": metre("Stand-off distance", "Abstand zur Explosionsquelle", precision=1),
            "assumedPressure": area_load("Assumed peak overpressure", "Angesetzter Spitzenüberdruck"),
        },
    },
    "en1992": {
        "Anchor": {
            "cracked": toggle("Cracked concrete", "Gerissener Beton"),
            "fUk": stress("Characteristic steel tensile strength f_uk", "Charakteristische Zugfestigkeit des Stahls f_uk"),
            "d": mm("Anchor diameter d", "Dübeldurchmesser d"),
            "c1": mm("Edge distance c₁", "Randabstand c₁"),
            "actions": rows("Load-case actions", "Einwirkungen je Lastfall"),
        },
        "RcMember": {
            "labelEn": LABEL_EN,
            "labelDe": LABEL_DE,
            "concreteGradeId": ref("concreteGrade", "Concrete strength class", "Betonfestigkeitsklasse"),
            "reinforcementGradeId": ref("reinforcementGrade", "Reinforcing steel", "Betonstahl"),
            "prestressSteelId": ref("prestressSteel", "Prestressing steel", "Spannstahl"),
            "support": L("Support conditions", "Lagerungsbedingungen", widget="select"),
            "bucklingLength": metre("Effective (buckling) length l₀", "Ersatzlänge l₀", precision=2),
            "longitudinal": rows("Longitudinal reinforcement layers", "Lagen der Längsbewehrung"),
            "stirrups": rows("Stirrups", "Bügel"),
            "punching": rows("Punching shear data", "Angaben zum Durchstanzen"),
            "prestress": rows("Prestress", "Vorspannung"),
            "fire": rows("Fire design", "Brandschutzbemessung"),
            "actions": rows("Load-case actions", "Einwirkungen je Lastfall"),
            "useFem": toggle("Use FEM results", "FEM-Ergebnisse verwenden"),
            "udl": line_load("Uniformly distributed load", "Gleichstreckenlast"),
            "deflectionSensitive": toggle("Deflection-sensitive finishes", "Verformungsempfindlicher Ausbau"),
            "tightness": L("Tightness class", "Dichtheitsklasse", widget="select"),
            "hdOverH": pure("Hydrostatic head to wall thickness h_D/h", "Verhältnis Wasserdruckhöhe zu Bauteildicke h_D/h"),
            "liquidSigmaS": stress("Steel stress σ_s (liquid retaining)", "Stahlspannung σ_s (Flüssigkeitsbehälter)"),
            "liquidRhoPEff": num("Effective reinforcement ratio ρ_p,eff", "Wirksamer Bewehrungsgrad ρ_p,eff", "1", "%", 100, 2),
            "liquidFCtEff": stress("Effective tensile strength f_ct,eff", "Wirksame Zugfestigkeit f_ct,eff"),
            "liquidSRMax": mm("Maximum crack spacing s_r,max", "Maximaler Rissabstand s_r,max"),
            "bridgeSigmaC": stress("Concrete compressive stress σ_c (bridge)", "Betondruckspannung σ_c (Brücke)"),
            "bridgeDeltaSigmaS": stress("Steel stress range Δσ_s (bridge)", "Spannungsschwingbreite Δσ_s (Brücke)"),
        },
        "LoadCaseActions": {
            "category": text("Action category", "Einwirkungskategorie"),
            "gKLine": line_load("Permanent line load g_k", "Ständige Streckenlast g_k"),
            "qKLine": line_load("Variable line load q_k", "Veränderliche Streckenlast q_k"),
            "pointForce": force("Point load", "Einzellast"),
            "mK": moment("Characteristic bending moment M_k", "Charakteristisches Biegemoment M_k"),
            "nK": force("Characteristic axial force N_k", "Charakteristische Normalkraft N_k"),
            "vK": force("Characteristic shear force V_k", "Charakteristische Querkraft V_k"),
            "tK": moment("Characteristic torsional moment T_k", "Charakteristisches Torsionsmoment T_k"),
            "vKPunch": force("Characteristic punching load V_k", "Charakteristische Durchstanzlast V_k"),
        },
        "BarLayer": {
            "anchorageLength": mm("Anchorage length l_bd", "Verankerungslänge l_bd"),
            "lapLength": mm("Lap length l₀", "Übergreifungslänge l₀"),
            "bondCondition": text("Bond condition", "Verbundbedingung", D("good or poor (EN 1992-1-1 8.4.2).", "gut oder mäßig (EN 1992-1-1 8.4.2).")),
            "aggregateSize": mm("Maximum aggregate size d_g", "Größtkorn d_g"),
        },
        "Stirrups": {"diameter": mm("Stirrup diameter", "Bügeldurchmesser"), "legs": count("Number of legs", "Schnittigkeit")},
        "PunchingSpec": {
            "columnWidth": mm("Column width c₁", "Stützenbreite c₁"),
            "columnDepth": mm("Column depth c₂", "Stützentiefe c₂"),
            "columnPosition": text("Column position", "Stützenlage", D("interior, edge or corner.", "Innen-, Rand- oder Eckstütze.")),
            "asw": num("Punching shear reinforcement A_sw", "Durchstanzbewehrung A_sw", "m²", "cm²", 10000, 2),
        },
        "PrestressSpec": {
            "force": force("Prestressing force P", "Vorspannkraft P"),
            "area": num("Prestressing steel area A_p", "Spannstahlfläche A_p", "m²", "cm²", 10000, 2),
            "eccentricity": mm("Tendon eccentricity e_p", "Exzentrizität des Spannglieds e_p"),
            "lossRatio": num("Prestress losses", "Spannkraftverluste", "1", "%", 100, 0),
        },
        "FireSpec": {
            "columnMethod": text("Column fire method", "Brandschutzverfahren für Stützen", D("Method A or B (EN 1992-1-2 5.3).", "Methode A oder B (EN 1992-1-2 5.3).")),
            "slabSystem": text("Slab system", "Deckensystem"),
        },
    },
    "en1993": {
        "SteelSection": {
            "designation": text("Section designation", "Profilbezeichnung", D("e.g. IPE 300, HEB 200.", "z. B. IPE 300, HEB 200.")),
            "h": mm("Section depth h", "Querschnittshöhe h"),
            "b": mm("Flange width b", "Flanschbreite b"),
            "tw": mm("Web thickness t_w", "Stegdicke t_w"),
            "tf": mm("Flange thickness t_f", "Flanschdicke t_f"),
            "r": mm("Root radius r", "Ausrundungsradius r"),
            "area": num("Cross-sectional area A", "Querschnittsfläche A", "m²", "cm²", 1e4, 2),
            "shearAreaY": num("Shear area A_v,y", "Schubfläche A_v,y", "m²", "cm²", 1e4, 2),
            "shearAreaZ": num("Shear area A_v,z", "Schubfläche A_v,z", "m²", "cm²", 1e4, 2),
            "iy": num("Second moment of area I_y", "Flächenträgheitsmoment I_y", "m⁴", "cm⁴", 1e8, 0),
            "iz": num("Second moment of area I_z", "Flächenträgheitsmoment I_z", "m⁴", "cm⁴", 1e8, 0),
            "it": num("Torsion constant I_t", "Torsionsträgheitsmoment I_t", "m⁴", "cm⁴", 1e8, 1),
            "iw": num("Warping constant I_w", "Wölbwiderstand I_w", "m⁶", "cm⁶", 1e12, 0),
            "wElY": num("Elastic section modulus W_el,y", "Elastisches Widerstandsmoment W_el,y", "m³", "cm³", 1e6, 0),
            "wElZ": num("Elastic section modulus W_el,z", "Elastisches Widerstandsmoment W_el,z", "m³", "cm³", 1e6, 0),
            "wPlY": num("Plastic section modulus W_pl,y", "Plastisches Widerstandsmoment W_pl,y", "m³", "cm³", 1e6, 0),
            "wPlZ": num("Plastic section modulus W_pl,z", "Plastisches Widerstandsmoment W_pl,z", "m³", "cm³", 1e6, 0),
            "areaNet": num("Net area A_net", "Nettoquerschnittsfläche A_net", "m²", "cm²", 1e4, 2),
        },
        "SteelMaterial": {
            "grade": text("Steel grade", "Stahlsorte", D("e.g. S235, S355, S460.", "z. B. S235, S355, S460.")),
            "fy": stress("Yield strength f_y", "Streckgrenze f_y"),
            "fu": stress("Ultimate tensile strength f_u", "Zugfestigkeit f_u"),
            "eModulus": stress("Modulus of elasticity E", "Elastizitätsmodul E"),
            "gModulus": stress("Shear modulus G", "Schubmodul G"),
            "subgrade": text("Steel subgrade", "Gütegruppe", D("JR, J0, J2 or K2 (EN 10025).", "JR, J0, J2 oder K2 (EN 10025).")),
        },
        "SteelMember": {
            "memberType": text("Member type", "Bauteilart"),
            "sectionId": ref("section", "Cross-section", "Querschnitt"),
            "materialId": ref("material", "Steel material", "Stahlwerkstoff"),
            "length": metre("System length L", "Systemlänge L", precision=2),
            "bucklingLengthY": metre("Buckling length L_cr,y", "Knicklänge L_cr,y", precision=2),
            "bucklingLengthZ": metre("Buckling length L_cr,z", "Knicklänge L_cr,z", precision=2),
            "ltbLength": metre("Lateral-torsional buckling length L_LT", "Kipplänge L_LT", precision=2),
            "ltbRestraintSpacing": metre("Spacing of lateral restraints", "Abstand der seitlichen Halterungen", precision=2),
            "loadApplication": text("Point of load application", "Lastangriffspunkt", D("top flange, shear centre or bottom flange.", "Obergurt, Schubmittelpunkt oder Untergurt.")),
            "endMomentRatioPsi": pure("End moment ratio ψ", "Randmomentenverhältnis ψ"),
            "momentDiagram": text("Bending moment diagram", "Momentenverlauf"),
            "deflectionLimitRatio": pure("Deflection limit l/…", "Durchbiegungsgrenze l/…", D("Denominator of the admissible deflection l/n.", "Nenner der zulässigen Durchbiegung l/n."), 0),
            "analysis": text("Global analysis method", "Verfahren der Tragwerksberechnung", D("elastic or plastic.", "elastisch oder plastisch.")),
        },
        "LoadCase": {"category": text("Action category", "Einwirkungskategorie")},
        "MemberAction": {"loadCaseId": ref("loadCase", "Load case", "Lastfall")},
        "DesignAction": {
            "n": force("Axial force N_Ed", "Normalkraft N_Ed"),
            "vy": force("Shear force V_y,Ed", "Querkraft V_y,Ed"),
            "vz": force("Shear force V_z,Ed", "Querkraft V_z,Ed"),
            "my": moment("Bending moment M_y,Ed", "Biegemoment M_y,Ed"),
            "mz": moment("Bending moment M_z,Ed", "Biegemoment M_z,Ed"),
            "t": moment("Torsional moment T_Ed", "Torsionsmoment T_Ed"),
        },
        "ForceAction": {"loadCaseId": ref("loadCase", "Load case", "Lastfall"), "force": force("Characteristic force", "Charakteristische Kraft")},
        "JointForceAction": {"loadCaseId": ref("loadCase", "Load case", "Lastfall"), "shear": force("Shear force per joint", "Querkraft je Anschluss"), "tension": force("Tension force per joint", "Zugkraft je Anschluss")},
        "FatigueBand": {"deltaSigma": stress("Stress range Δσ_i", "Spannungsschwingbreite Δσ_i"), "cycles": pure("Number of cycles n_i", "Lastspielzahl n_i", precision=0)},
        "SteelJoint": {
            "boltClass": text("Bolt property class", "Festigkeitsklasse der Schrauben", D("e.g. 8.8, 10.9.", "z. B. 8.8, 10.9.")),
            "boltDiameter": mm("Bolt diameter d", "Schraubendurchmesser d"),
            "boltRows": count("Bolt rows", "Schraubenreihen"),
            "boltsPerRow": count("Bolts per row", "Schrauben je Reihe"),
            "pitch": mm("Pitch p₁", "Lochabstand p₁"),
            "gauge": mm("Gauge p₂", "Lochabstand quer p₂"),
            "endDistance": mm("End distance e₁", "Randabstand in Kraftrichtung e₁"),
            "edgeDistance": mm("Edge distance e₂", "Randabstand quer zur Kraftrichtung e₂"),
            "shearPlanes": count("Shear planes", "Scherfugen"),
            "plateThickness": mm("Plate thickness t", "Blechdicke t"),
            "plateFu": stress("Plate tensile strength f_u", "Zugfestigkeit des Blechs f_u"),
            "weldThroat": mm("Weld throat thickness a", "Nahtdicke a"),
            "weldLength": mm("Weld length l_w", "Nahtlänge l_w"),
            "weldFu": stress("Weld metal tensile strength f_u", "Zugfestigkeit des Schweißguts f_u"),
            "weldGrade": text("Weld grade", "Schweißgüte"),
            "actions": rows("Joint forces per load case", "Anschlusskräfte je Lastfall"),
            "category": text("Bolted connection category", "Kategorie der Schraubenverbindung", D("A to E (EN 1993-1-8 3.4).", "A bis E (EN 1993-1-8 3.4).")),
            "frictionMu": pure("Slip factor μ", "Reibungszahl μ"),
            "preloadForce": force("Preloading force F_p,C", "Vorspannkraft F_p,C"),
            "slipFactorKs": pure("Hole-type factor k_s", "Lochbeiwert k_s"),
            "frictionSurfaces": count("Friction surfaces n", "Reibflächen n"),
        },
        "FatigueDetail": {
            "category": count("Detail category Δσ_C", "Kerbfall Δσ_C", D("Detail category in N/mm² (EN 1993-1-9 Tables 8.1–8.10).", "Kerbfall in N/mm² (EN 1993-1-9 Tabellen 8.1–8.10).")),
            "method": text("Assessment method", "Nachweiskonzept", D("safe-life or damage-tolerant.", "Sicherheitsnachweis Safe-Life oder schadenstolerant.")),
            "spectrum": rows("Stress-range spectrum", "Spannungskollektiv"),
        },
        "FireExposure": {
            "rating": text("Fire resistance class", "Feuerwiderstandsklasse", D("e.g. R30, R60, R90.", "z. B. R30, R60, R90.")),
            "protectionThickness": mm("Fire protection thickness d_p", "Dicke der Brandschutzbekleidung d_p"),
            "sectionFactor": num("Section factor A_m/V", "Profilfaktor A_m/V", "1/m", precision=0),
            "mu0": pure("Degree of utilisation μ₀", "Ausnutzungsgrad μ₀"),
            "designTemperature": num("Design steel temperature θ_a", "Bemessungstemperatur des Stahls θ_a", "°C", precision=0),
            "protectionConductivity": num("Thermal conductivity of the protection λ_p", "Wärmeleitfähigkeit der Bekleidung λ_p", "W/(m·K)", precision=3),
            "protectionDensity": num("Density of the protection ρ_p", "Rohdichte der Bekleidung ρ_p", "kg/m³", precision=0),
            "protectionSpecificHeat": num("Specific heat of the protection c_p", "Spezifische Wärmekapazität der Bekleidung c_p", "J/(kg·K)", precision=0),
        },
        "ColdFormedMember": {
            "bBar": mm("Notional flat width b_p", "Rechnerische ebene Breite b_p"),
            "thickness": mm("Nominal thickness t", "Nennblechdicke t"),
            "kSigma": pure("Buckling factor k_σ", "Beulwert k_σ"),
            "psi": pure("Stress ratio ψ", "Randspannungsverhältnis ψ"),
            "fy": stress("Yield strength f_yb", "Basisstreckgrenze f_yb"),
            "grossResistance": force("Gross cross-section resistance", "Widerstand des Bruttoquerschnitts"),
            "actions": rows("Forces per load case", "Kräfte je Lastfall"),
        },
        "PlatedPanel": {
            "a": mm("Panel length a", "Feldlänge a"),
            "b": mm("Panel width b", "Feldbreite b"),
            "thickness": mm("Plate thickness t", "Blechdicke t"),
            "fy": stress("Yield strength f_y", "Streckgrenze f_y"),
            "kSigma": pure("Buckling factor k_σ", "Beulwert k_σ"),
            "actions": rows("Forces per load case", "Kräfte je Lastfall"),
        },
        "SiloShell": {
            "thickness": mm("Shell thickness t", "Schalendicke t"),
            "depth": metre("Depth below the equivalent surface z", "Tiefe unter der Ersatzoberfläche z", precision=2),
            "k": pure("Lateral pressure ratio K", "Horizontallastverhältnis K"),
            "gamma": num("Unit weight of the stored solid γ", "Wichte des Schüttguts γ", "N/m³", "kN/m³", 0.001, 1),
            "fy": stress("Yield strength f_y", "Streckgrenze f_y"),
        },
        "TensionComponent": {
            "fUk": force("Characteristic breaking strength F_uk", "Charakteristische Bruchkraft F_uk"),
            "fK": force("Characteristic 0.2 % proof force F_k", "Charakteristische 0,2-%-Dehngrenzkraft F_k"),
            "actions": rows("Forces per load case", "Kräfte je Lastfall"),
        },
        "BridgeFatigue": {
            "lambda": pure("Damage equivalence factor λ", "Schadensäquivalenzfaktor λ"),
            "phi2": pure("Damage equivalent impact factor Φ₂", "Schadensäquivalenter Stoßbeiwert Φ₂"),
            "deltaSigmaP": stress("Stress range Δσ_p", "Spannungsschwingbreite Δσ_p"),
            "category": count("Detail category Δσ_C", "Kerbfall Δσ_C", D("Detail category in N/mm².", "Kerbfall in N/mm².")),
            "method": text("Assessment method", "Nachweiskonzept"),
        },
        "TowerLeg": {
            "forceCoefficient": pure("Force coefficient c_f", "Kraftbeiwert c_f"),
            "dynamicFactor": pure("Structural factor c_s·c_d", "Strukturbeiwert c_s·c_d"),
            "actions": rows("Forces per load case", "Kräfte je Lastfall"),
        },
        "SteelPile": {
            "sectionId": ref("section", "Cross-section", "Querschnitt"),
            "materialId": ref("material", "Steel material", "Stahlwerkstoff"),
            "drivingStress": stress("Driving stress σ", "Rammspannung σ"),
            "embeddedLength": metre("Embedded length", "Einbindelänge", precision=2),
            "shaftPerimeter": metre("Shaft perimeter", "Mantelumfang", precision=3),
            "actions": rows("Forces per load case", "Kräfte je Lastfall"),
        },
        "CraneRunway": {
            "wheelContactLength": mm("Wheel contact length", "Radaufstandslänge"),
            "dispersion": mm("Load dispersion length", "Lastausbreitungslänge"),
            "webThickness": mm("Web thickness t_w", "Stegdicke t_w"),
            "fy": stress("Yield strength f_y", "Streckgrenze f_y"),
            "phi": pure("Dynamic factor φ", "Dynamischer Faktor φ"),
            "actions": rows("Wheel loads per load case", "Radlasten je Lastfall"),
        },
    },
    "en1997": {
        "SpreadFoundation": {
            "length": metre("Foundation length L", "Fundamentlänge L", precision=2),
            "baseInclinationDeg": num("Base inclination α", "Sohlneigung α", "°", precision=1),
            "settlementLimit": mm("Admissible settlement s", "Zulässige Setzung s"),
            "loadCases": rows("Load cases", "Lastfälle"),
        },
        "SoilLayer": {
            "soilType": text("Soil type", "Bodenart", D("e.g. sand, clay, gravel (DIN 18196 group).", "z. B. Sand, Ton, Kies (Bodengruppe nach DIN 18196).")),
            "depthTop": metre("Depth of layer top", "Tiefe der Schichtoberkante", precision=2),
            "depthBottom": metre("Depth of layer bottom", "Tiefe der Schichtunterkante", precision=2),
            "gamma": num("Unit weight γ", "Wichte γ", "N/m³", "kN/m³", 0.001, 1),
            "gammaPrime": num("Buoyant unit weight γ′", "Wichte unter Auftrieb γ′", "N/m³", "kN/m³", 0.001, 1),
            "cohesionEffective": area_load("Effective cohesion c′", "Effektive Kohäsion c′"),
            "cohesionUndrained": area_load("Undrained shear strength c_u", "Undränierte Scherfestigkeit c_u"),
            "poissonRatio": pure("Poisson's ratio ν", "Querdehnzahl ν"),
            "cptQc": stress("Cone resistance q_c", "Spitzenwiderstand q_c"),
            "sptN": pure("SPT blow count N", "SPT-Schlagzahl N", precision=0),
        },
        "Pile": {
            "pileType": text("Pile type", "Pfahlart", D("bored, driven or CFA.", "Bohrpfahl, Verdrängungspfahl oder Schneckenbohrpfahl.")),
            "diameter": mm("Pile diameter D", "Pfahldurchmesser D"),
            "alphaS": pure("Shaft friction factor α_s", "Mantelreibungsbeiwert α_s"),
            "unitShaftResistance": area_load("Unit shaft resistance q_s", "Pfahlmantelreibung q_s"),
            "unitBaseResistance": stress("Unit base resistance q_b", "Pfahlspitzenwiderstand q_b"),
            "compressionPermanent": force("Permanent compression G_k", "Ständige Druckkraft G_k"),
            "compressionVariable": force("Variable compression Q_k", "Veränderliche Druckkraft Q_k"),
            "tensionPermanent": force("Permanent tension G_k", "Ständige Zugkraft G_k"),
            "tensionVariable": force("Variable tension Q_k", "Veränderliche Zugkraft Q_k"),
            "testProfiles": rows("Pile load test profiles", "Probebelastungsprofile"),
        },
        "FoundationLoadCase": {
            "verticalPermanent": force("Permanent vertical load V_G,k", "Ständige Vertikallast V_G,k"),
            "verticalVariable": force("Variable vertical load V_Q,k", "Veränderliche Vertikallast V_Q,k"),
            "horizontalPermanent": force("Permanent horizontal load H_G,k", "Ständige Horizontallast H_G,k"),
            "horizontalVariable": force("Variable horizontal load H_Q,k", "Veränderliche Horizontallast H_Q,k"),
            "momentPermanent": moment("Permanent moment M_G,k", "Ständiges Moment M_G,k"),
            "momentVariable": moment("Variable moment M_Q,k", "Veränderliches Moment M_Q,k"),
        },
        "PileTestProfile": {
            "shaftResistance": force("Measured shaft resistance R_s,m", "Gemessener Mantelwiderstand R_s,m"),
            "baseResistance": force("Measured base resistance R_b,m", "Gemessener Spitzenwiderstand R_b,m"),
        },
    },
    "en1994": {
        "CompositeSlab": {
            "spanM": metre("Span L", "Stützweite L", precision=2),
            "support": text("Support conditions", "Lagerungsbedingungen", D("simply_supported or continuous_2_span.", "simply_supported (Einfeldträger) oder continuous_2_span (Zweifeldträger).")),
            "sheeting": rows("Profiled steel sheeting", "Profilblech"),
            "fCkPa": stress("Concrete strength f_ck", "Betondruckfestigkeit f_ck"),
            "mFactor": pure("m-k factor m", "m-k-Beiwert m"),
            "kFactor": pure("m-k factor k", "m-k-Beiwert k", precision=3),
            "asM2PerM": num("Reinforcement A_s", "Bewehrung A_s", "m²/m", "cm²/m", 1e4, 2),
            "actions": rows("Characteristic actions", "Charakteristische Einwirkungen"),
        },
        "CompositeBeam": {
            "support": text("Support conditions", "Lagerungsbedingungen", D("simply_supported or continuous_2_span.", "simply_supported (Einfeldträger) oder continuous_2_span (Zweifeldträger).")),
            "steel": rows("Steel section", "Stahlprofil"),
            "concreteFCkPa": stress("Concrete strength f_ck", "Betondruckfestigkeit f_ck"),
            "concreteECmPa": stress("Concrete modulus E_cm", "Elastizitätsmodul des Betons E_cm"),
            "sheeting": rows("Profiled steel sheeting", "Profilblech"),
            "studs": rows("Headed stud shear connectors", "Kopfbolzendübel"),
            "ltbLengthM": metre("Lateral-torsional buckling length L_LT", "Kipplänge L_LT", precision=2),
            "asHoggingM2PerM": num("Hogging reinforcement A_s", "Stützbewehrung A_s", "m²/m", "cm²/m", 1e4, 2),
            "barSpacingM": mm("Bar spacing s", "Stababstand s"),
            "wkLimitM": mm("Crack width limit w_k", "Grenzwert der Rissbreite w_k"),
            "nCycles": pure("Number of stress cycles N", "Lastspielzahl N", precision=0),
            "actions": rows("Characteristic actions", "Charakteristische Einwirkungen"),
        },
        "CompositeColumn": {
            "lengthM": metre("Column length L", "Stützenlänge L", precision=2),
            "outerSizeM": mm("Outer diameter or side length", "Außendurchmesser oder Seitenlänge"),
            "wallThicknessM": mm("Wall thickness t", "Wanddicke t"),
            "steelAM2": num("Steel area A_a", "Stahlfläche A_a", "m²", "cm²", 1e4, 2),
            "concreteAM2": num("Concrete area A_c", "Betonfläche A_c", "m²", "cm²", 1e4, 1),
            "concreteFCkPa": stress("Concrete strength f_ck", "Betondruckfestigkeit f_ck"),
            "reinforcementAsM2": num("Reinforcement area A_s", "Bewehrungsfläche A_s", "m²", "cm²", 1e4, 2),
            "reinforcementFYkPa": stress("Reinforcement yield strength f_sk", "Streckgrenze der Bewehrung f_sk"),
            "iM4": num("Second moment of area I", "Flächenträgheitsmoment I", "m⁴", "cm⁴", 1e8, 0),
            "bucklingCurve": text("Buckling curve", "Knicklinie", D("a, b or c (EN 1994-1-1 Table 6.5).", "a, b oder c (EN 1994-1-1 Tabelle 6.5).")),
            "actions": rows("Column actions", "Stützenbeanspruchungen"),
        },
        "ProfiledSheeting": {
            "profile": text("Profile type", "Profilform", D("trapezoidal or re-entrant.", "trapezförmig oder hinterschnitten.")),
            "heightM": mm("Profile height h_p", "Profilhöhe h_p"),
            "ribWidthM": mm("Rib width b₀", "Rippenbreite b₀"),
            "thicknessM": mm("Sheet thickness t", "Blechdicke t"),
            "ribsParallelToBeam": toggle("Ribs parallel to the beam", "Rippen parallel zum Träger"),
        },
        "CharacteristicAction": {
            "category": text("Action category", "Einwirkungskategorie", D("self_steel, wet_concrete, finishes, A–H, flm3 …", "self_steel, wet_concrete, finishes, A–H, flm3 …")),
            "stage": text("Construction stage", "Bauzustand", D("construction or composite.", "construction (Bauzustand) oder composite (Verbundzustand).")),
            "fKN": force("Concentrated force F_k", "Einzellast F_k"),
            "deltaSigmaKPa": stress("Stress range Δσ_k", "Spannungsschwingbreite Δσ_k"),
            "deltaTauKPa": stress("Shear stress range Δτ_k", "Schubspannungsschwingbreite Δτ_k"),
        },
        "SteelSection": {
            "designation": text("Section designation", "Profilbezeichnung"),
            "heightM": mm("Section depth h", "Profilhöhe h"),
            "widthM": mm("Flange width b", "Flanschbreite b"),
            "twM": mm("Web thickness t_w", "Stegdicke t_w"),
            "tfM": mm("Flange thickness t_f", "Flanschdicke t_f"),
            "aM2": num("Cross-sectional area A_a", "Querschnittsfläche A_a", "m²", "cm²", 1e4, 2),
            "wPlYM3": num("Plastic section modulus W_pl,y", "Plastisches Widerstandsmoment W_pl,y", "m³", "cm³", 1e6, 0),
            "iYM4": num("Second moment of area I_y", "Flächenträgheitsmoment I_y", "m⁴", "cm⁴", 1e8, 0),
            "aVM2": num("Shear area A_v", "Schubfläche A_v", "m²", "cm²", 1e4, 2),
        },
        "HeadedStuds": {"heightM": mm("Stud height h_sc", "Dübelhöhe h_sc"), "countPerRib": count("Studs per rib n_r", "Dübel je Rippe n_r")},
        "ColumnAction": {
            "category": text("Action category", "Einwirkungskategorie"),
            "stage": text("Construction stage", "Bauzustand", D("construction or composite.", "construction (Bauzustand) oder composite (Verbundzustand).")),
            "mKNm": moment("Characteristic bending moment M_k", "Charakteristisches Biegemoment M_k"),
        },
    },
    "en1995": {
        "TimberConnection": {
            "diameterM": mm("Fastener diameter d", "Durchmesser des Verbindungsmittels d"),
            "spacingM": mm("Fastener spacing a₁", "Abstand der Verbindungsmittel a₁"),
            "edgeDistanceM": mm("Edge distance a₄", "Randabstand a₄"),
            "endDistanceM": mm("End distance a₃", "Hirnholzabstand a₃"),
            "t1M": mm("Side member thickness t₁", "Dicke des Seitenholzes t₁"),
            "t2M": mm("Central member thickness t₂", "Dicke des Mittelholzes t₂"),
            "steelPlateThicknessM": mm("Steel plate thickness", "Stahlblechdicke"),
            "fUK": stress("Fastener tensile strength f_u,k", "Zugfestigkeit des Verbindungsmittels f_u,k"),
            "actions": rows("Connection actions", "Einwirkungen auf die Verbindung"),
        },
        "ConnectionAction": {"fKN": force("Characteristic force F_k", "Charakteristische Kraft F_k")},
        "TimberMember": {
            "bM": mm("Width b", "Breite b"),
            "hM": mm("Depth h", "Höhe h"),
            "spanM": metre("Span l", "Stützweite l", precision=2),
            "supportLengthM": mm("Support length", "Auflagerlänge"),
            "bearingLengthM": mm("Bearing length l", "Aufstandslänge l"),
            "bucklingLengthYM": metre("Buckling length l_ef,y", "Knicklänge l_ef,y", precision=2),
            "bucklingLengthZM": metre("Buckling length l_ef,z", "Knicklänge l_ef,z", precision=2),
            "lateralRestraintSpacingM": metre("Spacing of lateral restraints l_ef", "Abstand der seitlichen Halterungen l_ef", precision=2),
            "notchDepthM": mm("Notch depth h − h_ef", "Ausklinkungstiefe h − h_ef"),
            "notchDistanceM": mm("Notch distance x", "Abstand der Ausklinkung x"),
            "mCritNm": moment("Critical bending moment M_crit", "Kritisches Biegemoment M_crit"),
            "massKgPerM": num("Linear mass m", "Masse je Länge m", "kg/m", precision=1),
            "massKgPerM2": num("Area mass m", "Flächenbezogene Masse m", "kg/m²", precision=1),
            "dampingXi": num("Modal damping ratio ξ", "Modales Dämpfungsmaß ξ", "1", "%", 100, 1),
            "fireDurationS": num("Fire exposure time t", "Branddauer t", "s", "min", 1 / 60, 0),
            "bridgeTLYears": num("Design working life t_L", "Nutzungsdauer t_L", "a", precision=0),
            "bridgeCrowdPerM2": num("Pedestrian density", "Personendichte", "1/m²", precision=2),
            "actions": rows("Characteristic actions", "Charakteristische Einwirkungen"),
        },
        "CharacteristicAction": {
            "qLineNPerM": line_load("Line load q_k", "Streckenlast q_k"),
            "fPointN": force("Point load F_k", "Einzellast F_k"),
            "mKNm": moment("Characteristic bending moment M_k", "Charakteristisches Biegemoment M_k"),
            "vKN": force("Characteristic shear force V_k", "Charakteristische Querkraft V_k"),
            "nKN": force("Characteristic compression N_c,k", "Charakteristische Druckkraft N_c,k"),
            "nTKN": force("Characteristic tension N_t,k", "Charakteristische Zugkraft N_t,k"),
            "fC90KN": force("Compression perpendicular to grain F_c,90,k", "Querdruckkraft F_c,90,k"),
        },
    },
    "en1999": {
        "ColdFormedSheet": {
            "materialId": ref("material", "Aluminium alloy", "Aluminiumlegierung"),
            "thickness": mm("Design thickness t", "Bemessungsblechdicke t"),
            "span": metre("Span L", "Stützweite L", precision=2),
            "welded": toggle("Welded", "Geschweißt"),
            "actions": rows("Characteristic actions", "Charakteristische Einwirkungen"),
        },
        "AluminiumMember": {
            "sectionId": ref("section", "Cross-section", "Querschnitt"),
            "materialId": ref("material", "Aluminium alloy", "Aluminiumlegierung"),
            "support": text("Support conditions", "Lagerungsbedingungen", D("simplySupported, continuous or cantilever.", "simplySupported (Einfeldträger), continuous (Durchlaufträger) oder cantilever (Kragarm).")),
            "bucklingLengthY": metre("Buckling length l_cr,y", "Knicklänge l_cr,y", precision=2),
            "bucklingLengthZ": metre("Buckling length l_cr,z", "Knicklänge l_cr,z", precision=2),
            "bucklingLengthT": metre("Torsional buckling length l_cr,T", "Drillknicklänge l_cr,T", precision=2),
            "ltbLength": metre("Lateral-torsional buckling length", "Kipplänge", precision=2),
            "c1": pure("Moment factor C₁", "Momentenbeiwert C₁", precision=3),
            "restrainedLtb": toggle("Laterally restrained", "Gegen Kippen gehalten"),
            "actions": rows("Characteristic actions", "Charakteristische Einwirkungen"),
        },
        "AluminiumSection": {
            "flangeThickness": mm("Flange thickness t_f", "Flanschdicke t_f"),
            "webThickness": mm("Web (wall) thickness t_w", "Steg- bzw. Wanddicke t_w"),
            "outerDiameter": mm("Outer diameter D", "Außendurchmesser D"),
            "elements": rows("Plate elements", "Querschnittsteile"),
        },
        "FatigueDetail": {
            "detailCategory": text("Detail category", "Kerbfall", D("EN 1999-1-3 Annex J id, e.g. 71 or 40-weld.", "Kennung nach EN 1999-1-3 Anhang J, z. B. 71 oder 40-weld.")),
            "deltaSigmaC": stress("Detail category strength Δσ_C", "Ermüdungsfestigkeit des Kerbfalls Δσ_C"),
            "deltaSigmaEd": stress("Stress range Δσ_E,d", "Spannungsschwingbreite Δσ_E,d"),
            "nCycles": pure("Number of cycles n_i", "Lastspielzahl n_i", precision=0),
            "m1": pure("Slope m₁", "Neigung m₁"),
            "m2": pure("Slope m₂", "Neigung m₂"),
        },
        "AluminiumConnection": {
            "materialId": ref("material", "Aluminium alloy", "Aluminiumlegierung"),
            "actions": rows("Characteristic actions", "Charakteristische Einwirkungen"),
            "bolts": rows("Bolt group", "Schraubengruppe"),
            "welds": rows("Weld group", "Schweißnahtgruppe"),
        },
        "FireScenario": {
            "thetaA": num("Aluminium temperature θ_al", "Aluminiumtemperatur θ_al", "°C", precision=0),
            "durationS": num("Fire duration t", "Branddauer t", "s", "min", 1 / 60, 0),
        },
        "AluminiumShell": {
            "materialId": ref("material", "Aluminium alloy", "Aluminiumlegierung"),
            "thickness": mm("Wall thickness t", "Wanddicke t"),
            "length": metre("Meridian length L", "Meridianlänge L", precision=2),
            "actions": rows("Characteristic membrane actions", "Charakteristische Membraneinwirkungen"),
        },
        "MemberAction": {
            "category": text("Action category", "Einwirkungskategorie", D("EN 1990 ψ-category (office, snow, wind, self …).", "ψ-Kategorie nach EN 1990 (office, snow, wind, self …).")),
            "gKLine": line_load("Permanent line load g_k", "Ständige Streckenlast g_k"),
            "qKLine": line_load("Variable line load q_k", "Veränderliche Streckenlast q_k"),
            "nK": force("Characteristic axial force N_k", "Charakteristische Normalkraft N_k"),
            "vYK": force("Characteristic shear force V_y,k", "Charakteristische Querkraft V_y,k"),
            "vZK": force("Characteristic shear force V_z,k", "Charakteristische Querkraft V_z,k"),
            "mYK": moment("Characteristic bending moment M_y,k", "Charakteristisches Biegemoment M_y,k"),
            "mZK": moment("Characteristic bending moment M_z,k", "Charakteristisches Biegemoment M_z,k"),
        },
        "PlateElement": {
            "outstand": toggle("Outstand element", "Einseitig gestütztes Querschnittsteil"),
            "welded": toggle("Welded", "Geschweißt"),
            "weldPosition": mm("Weld position from the root", "Lage der Schweißnaht ab Anschluss"),
        },
        "BoltGroup": {
            "diameter": mm("Bolt diameter d", "Schraubendurchmesser d"),
            "edgeDistance": mm("Edge distance e₁", "Randabstand e₁"),
            "pitch": mm("Pitch p₁", "Lochabstand p₁"),
            "gauge": mm("Gauge p₂", "Lochabstand quer p₂"),
            "plateThickness": mm("Plate thickness t", "Blechdicke t"),
        },
        "WeldGroup": {
            "fillerAlloy": text("Filler alloy", "Schweißzusatz", D("e.g. 4043, 5356.", "z. B. 4043, 5356.")),
            "length": mm("Weld length l_w", "Nahtlänge l_w"),
            "betaW": pure("Correlation factor β_w", "Korrelationsbeiwert β_w"),
            "hazExtent": mm("Extent of the heat-affected zone b_haz", "Ausdehnung der Wärmeeinflusszone b_haz"),
        },
    },
    "en1998": {
        "En1998Building": {
            "planWidthM": metre("Plan dimension L_x", "Grundrissabmessung L_x", precision=2),
            "planLengthM": metre("Plan dimension L_y", "Grundrissabmessung L_y", precision=2),
            "systems": rows("Lateral load-resisting systems", "Aussteifungssysteme"),
            "storeys": rows("Storeys", "Geschosse"),
            "members": rows("Members", "Bauteile"),
            "t1Method": text("Fundamental period method", "Verfahren für die Grundschwingzeit", D("formula (4.6) or given.", "Formel (4.6) oder vorgegeben.")),
            "t1GivenS": num("Given fundamental period T₁", "Vorgegebene Grundschwingzeit T₁", "s", precision=2),
            "ct": pure("Coefficient C_t", "Beiwert C_t", precision=3),
            "driftLimitClass": text("Interstorey drift limit class", "Klasse der Stockwerksverschiebungsgrenze", D("brittle, ductile or none (EN 1998-1 4.4.3.2).", "spröde, duktil oder keine (EN 1998-1 4.4.3.2).")),
            "nu": pure("Reduction factor ν", "Abminderungsbeiwert ν"),
            "multipleResistingSystems": toggle("Multiple resisting systems", "Mehrere Aussteifungssysteme"),
            "claimsSimpleMasonry": toggle("Simple masonry building", "Einfacher Mauerwerksbau"),
            "accidentalEccentricityRatio": pure("Accidental eccentricity ratio", "Verhältnis der zufälligen Exzentrizität", precision=3),
        },
        "En1998Bridge": {
            "periodRatio": pure("Isolation period ratio T_isol/T_fixed", "Periodenverhältnis T_isol/T_fixed"),
            "fundamentalPeriodS": num("Fundamental period T", "Grundschwingzeit T", "s", precision=2),
            "bearingDRdM": mm("Bearing displacement capacity d_Rd", "Verschiebungskapazität des Lagers d_Rd"),
            "permanentGkN": force("Permanent action ΣG_k", "Ständige Einwirkung ΣG_k"),
            "correlatedOccupancy": toggle("Correlated occupancy", "Korrelierte Nutzung"),
            "variables": rows("Variable actions", "Veränderliche Einwirkungen"),
        },
        "En1998Site": {
            "seismicZone": L("Seismic zone", "Erdbebenzone", widget="segmented"),
            "aGr": num("Reference peak ground acceleration a_gR", "Referenz-Spitzenbodenbeschleunigung a_gR", "m/s²", precision=2),
            "deGroundCombo": L("Ground class and subsoil class", "Baugrundklasse und Untergrundklasse", widget="select"),
            "enGroundType": text("Ground type", "Baugrundklasse", D("A to E (EN 1998-1 Table 3.1); empty under the German annex.", "A bis E (EN 1998-1 Tabelle 3.1); leer beim deutschen Anhang.")),
            "enSpectrumType": text("Spectrum type", "Spektrumtyp", D("type1 or type2; empty under the German annex.", "type1 oder type2; leer beim deutschen Anhang.")),
            "importanceClass": text("Importance class", "Bedeutungskategorie"),
        },
        "En1998Assessment": {
            "knowledgeLevel": text("Knowledge level", "Kenntnisstand", D("KL1, KL2 or KL3 (EN 1998-3).", "KL1, KL2 oder KL3 (EN 1998-3).")),
            "limitState": text("Limit state", "Grenzzustand", D("nc, sd or dl (EN 1998-3 Table 2.1).", "nc, sd oder dl (EN 1998-3 Tabelle 2.1).")),
            "supportedBuildingId": ref("building", "Assessed building", "Bewertetes Gebäude"),
            "gammaEl": pure("Partial factor γ_el", "Teilsicherheitsbeiwert γ_el"),
        },
        "En1998Tower": {
            "heightM": metre("Height", "Höhe", precision=2),
            "isChimney": toggle("Chimney", "Schornstein"),
            "qNominal": pure("Behaviour factor q", "Verhaltensbeiwert q"),
            "permanentGkN": force("Permanent action ΣG_k", "Ständige Einwirkung ΣG_k"),
            "correlatedOccupancy": toggle("Correlated occupancy", "Korrelierte Nutzung"),
            "variables": rows("Variable actions", "Veränderliche Einwirkungen"),
        },
        "En1998Tank": {
            "heightM": metre("Tank height", "Behälterhöhe", precision=2),
            "radiusM": metre("Tank radius", "Behälterradius", precision=2),
            "permanentGkN": force("Permanent action ΣG_k", "Ständige Einwirkung ΣG_k"),
            "contentQkN": force("Content action Q_k", "Einwirkung aus Füllgut Q_k"),
            "contentCategory": text("Content category", "Füllgutkategorie"),
            "fillingRatio": num("Filling ratio", "Füllgrad", "1", "%", 100, 0),
            "vRdN": force("Shear resistance V_Rd", "Schubwiderstand V_Rd"),
        },
        "En1998RetainingWall": {
            "heightM": metre("Wall height H", "Wandhöhe H", precision=2),
            "phiDeg": num("Friction angle φ′", "Reibungswinkel φ′", "°", precision=1),
            "soilGamma": num("Soil unit weight γ", "Wichte des Bodens γ", "N/m³", "kN/m³", 0.001, 1),
            "r": pure("Factor r", "Beiwert r"),
            "hRdNPerM": line_load("Horizontal resistance H_Rd", "Horizontalwiderstand H_Rd"),
        },
        "En1998Foundation": {
            "supportedBuildingId": ref("building", "Supported building", "Getragenes Gebäude"),
            "areaM2": square("Foundation area", "Fundamentfläche"),
            "pRdPa": area_load("Bearing resistance p_Rd", "Sohldruckwiderstand p_Rd"),
            "hRdN": force("Sliding resistance H_Rd", "Gleitwiderstand H_Rd"),
            "kFoundation": pure("Foundation stiffness", "Fundamentsteifigkeit", precision=0),
            "kSoil": pure("Soil stiffness", "Bodensteifigkeit", precision=0),
        },
        "En1998Silo": {
            "heightM": metre("Silo height", "Silohöhe", precision=2),
            "radiusM": metre("Silo radius", "Siloradius", precision=2),
            "permanentGkN": force("Permanent action ΣG_k", "Ständige Einwirkung ΣG_k"),
            "contentQkN": force("Content action Q_k", "Einwirkung aus Füllgut Q_k"),
            "contentCategory": text("Content category", "Füllgutkategorie"),
            "fillingRatio": num("Filling ratio", "Füllgrad", "1", "%", 100, 0),
            "nRdN": force("Axial resistance N_Rd", "Normalkraftwiderstand N_Rd"),
            "vRdN": force("Shear resistance V_Rd", "Schubwiderstand V_Rd"),
            "qNominal": pure("Behaviour factor q", "Verhaltensbeiwert q"),
        },
        "En1998System": {
            "direction": text("Direction", "Richtung", D("x or y.", "x oder y.")),
            "systemType": text("Structural type", "Tragwerkstyp", D("frame, wall, dual, inverted pendulum …", "Rahmen, Wand, Mischsystem, umgekehrtes Pendel …")),
            "ductilityClass": text("Ductility class", "Duktilitätsklasse", D("DCL, DCM or DCH.", "DCL, DCM oder DCH.")),
            "q0": pure("Basic behaviour factor q₀", "Grundwert des Verhaltensbeiwerts q₀"),
            "alphaUOverAlpha1": pure("Overstrength ratio α_u/α₁", "Überfestigkeitsverhältnis α_u/α₁"),
            "kW": pure("Failure-mode factor k_w", "Beiwert der Versagensart k_w"),
        },
        "En1998Storey": {
            "heightM": metre("Storey height", "Geschosshöhe", precision=2),
            "correlatedOccupancy": toggle("Correlated occupancy", "Korrelierte Nutzung"),
            "variables": rows("Variable actions", "Veränderliche Einwirkungen"),
            "stiffnessY": num("Lateral stiffness k_y", "Horizontalsteifigkeit k_y", "N/m", "kN/m", 0.001, 0),
            "centreOfMassXM": metre("Centre of mass x", "Massenschwerpunkt x", precision=2),
            "centreOfMassYM": metre("Centre of mass y", "Massenschwerpunkt y", precision=2),
            "centreOfStiffnessXM": metre("Centre of stiffness x", "Steifigkeitsmittelpunkt x", precision=2),
            "centreOfStiffnessYM": metre("Centre of stiffness y", "Steifigkeitsmittelpunkt y", precision=2),
            "driftYM": mm("Interstorey drift d_r,y", "Stockwerksverschiebung d_r,y"),
            "shearResistanceN": force("Storey shear resistance V_Rd", "Geschossschubwiderstand V_Rd"),
        },
        "En1998Member": {
            "minDimensionM": mm("Minimum cross-section dimension b", "Kleinste Querschnittsabmessung b"),
            "rho": num("Tension reinforcement ratio ρ", "Zugbewehrungsgrad ρ", "1", "%", 100, 2),
            "rhoPrime": num("Compression reinforcement ratio ρ′", "Druckbewehrungsgrad ρ′", "1", "%", 100, 2),
            "omegaWd": pure("Mechanical volumetric ratio of confinement ω_wd", "Mechanischer Umschnürungsbewehrungsgrad ω_wd", precision=3),
            "steelSectionClass": count("Steel cross-section class", "Querschnittsklasse (Stahl)"),
        },
        "En1998VariableAction": {
            "category": text("Imposed-load category", "Nutzlastkategorie", D("A–H, snow or wind for ψ₂.", "A–H, Schnee oder Wind für ψ₂.")),
            "qkN": force("Characteristic variable action Q_k", "Charakteristische veränderliche Einwirkung Q_k"),
        },
    },
    "vdi3805": {
        "GenericAttribute": {"unit": text("Unit", "Einheit")},
    },
    "iso16757": {
        "SelectionConstraint": {"operator": L("Comparison operator", "Vergleichsoperator", widget="select")},
        "SpaceEnvelope": {"bounds": rows("Bounding box", "Hüllquader")},
        "SurfaceDefinition": {"purpose": text("Surface purpose", "Zweck der Fläche"), "bounds": rows("Bounding box", "Hüllquader")},
        "PortDefinition": {
            "medium": text("Medium", "Medium", D("e.g. water, air, gas.", "z. B. Wasser, Luft, Gas.")),
            "direction": L("Direction vector", "Richtungsvektor", widget="vector"),
            "portType": text("Port type", "Anschlussart"),
        },
    },
}

#: 🏷️ Enum option (or data-enum variant) labels per artifact → enum → wire value → (en, de).
OPTIONS = {
    "en1990": {"AnnexChoice": ANNEX, "ImportanceClass": {"I": ("I — minor importance", "I — geringe Bedeutung"), "II": ("II — ordinary buildings", "II — gewöhnliche Hochbauten"), "III": ("III — large occupancy", "III — große Menschenansammlungen"), "IV": ("IV — vital for civil protection", "IV — lebenswichtig für den Katastrophenschutz")}},
    "din16798": {"AnnexChoice": ANNEX},
    "din4108": {"ClimateZoneDe": {"Zone1": ("Summer climate region A (cool)", "Sommerklimaregion A (sommerkühl)"), "Zone2": ("Summer climate region B (moderate)", "Sommerklimaregion B (gemäßigt)"), "Zone3": ("Summer climate region C (hot)", "Sommerklimaregion C (sommerheiß)"), "Zone4": ("Summer climate region D", "Sommerklimaregion D")}},
    "din18599": {
        "AutomationClass": {"A": ("A — high energy performance", "A — hohe Energieeffizienz"), "B": ("B — advanced", "B — weiterentwickelt"), "C": ("C — standard", "C — Standard"), "D": ("D — non energy efficient", "D — nicht energieeffizient")},
        "BuildingCategory": {"Residential": ("Residential building", "Wohngebäude"), "NonResidential": ("Non-residential building", "Nichtwohngebäude")},
        "UseClass": {"Residential": ("Residential", "Wohnen"), "Office": ("Office", "Büro"), "School": ("School", "Schule")},
        "CalculationMethod": {"DetailedMonthly": ("Detailed monthly balance", "Detailliertes Monatsbilanzverfahren"), "Tabular": ("Tabular method", "Tabellenverfahren")},
        "Attachment": {"Detached": ("Detached", "Freistehend"), "SemiDetached": ("Semi-detached", "Doppelhaushälfte"), "EndTerrace": ("End of terrace", "Reihenendhaus"), "MidTerrace": ("Mid-terrace", "Reihenmittelhaus")},
        "UsageProfile": {"WFH": ("Residential (home)", "Wohnen"), "Office": ("Office", "Büro"), "School": ("School", "Schule")},
        "ElementKind": {"Wall": ("Wall", "Wand"), "Roof": ("Roof", "Dach"), "Floor": ("Floor", "Boden"), "Door": ("Door", "Tür"), "Window": ("Window", "Fenster")},
        "Adjacency": {"Outdoor": ("Outdoor air", "Außenluft"), "Ground": ("Ground", "Erdreich"), "Unheated": ("Unheated space", "Unbeheizter Raum"), "Heated": ("Heated space", "Beheizter Raum")},
    },
    "en1991": {
        "AnnexChoice": ANNEX,
        "StructureKind": {"building": ("Building", "Hochbau"), "bridge": ("Bridge", "Brücke")},
        "FireCurve": {"standard": ("Standard temperature-time curve", "Einheits-Temperaturzeitkurve"), "external": ("External fire curve", "Außenbrandkurve"), "hydrocarbon": ("Hydrocarbon curve", "Hydrokarbon-Brandkurve"), "parametric": ("Parametric fire curve", "Parametrische Brandkurve")},
        "FireMode": {"none": ("No fire design", "Keine Brandbemessung"), "nominal": ("Nominal fire curve", "Nominelle Temperaturzeitkurve"), "parametric": ("Parametric fire", "Parametrischer Brand")},
    },
    "en1992": {
        "AnnexChoice": ANNEX,
        "ExposureClass": {value: (value.upper(), value.upper()) for value in ["X0", "Xc1", "Xc2", "Xc3", "Xc4", "Xd1", "Xd2", "Xd3", "Xs1", "Xs2", "Xs3", "Xf1", "Xf2", "Xf3", "Xf4", "Xa1", "Xa2", "Xa3"]},
        "FireRating": {value: (value, value) for value in ["R30", "R60", "R90", "R120"]},
        "MemberKind": {"Beam": ("Beam", "Balken"), "Slab": ("Slab", "Platte"), "Column": ("Column", "Stütze"), "Wall": ("Wall", "Wand"), "FlatSlab": ("Flat slab", "Flachdecke"), "RibbedSlab": ("Ribbed slab", "Rippendecke"), "TensionMember": ("Tension member", "Zugglied"), "Bridge": ("Bridge", "Brücke"), "LiquidRetaining": ("Liquid-retaining structure", "Flüssigkeitsbehälter")},
        "SupportCondition": {"SimplySupported": ("Simply supported", "Einfeldträger"), "Continuous": ("Continuous", "Durchlaufträger"), "Cantilever": ("Cantilever", "Kragarm"), "Fixed": ("Fixed at both ends", "Beidseitig eingespannt")},
        "TightnessClass": {"Tc0": ("Tightness class 0", "Dichtheitsklasse 0"), "Tc1": ("Tightness class 1", "Dichtheitsklasse 1"), "Tc2": ("Tightness class 2", "Dichtheitsklasse 2")},
    },
    "en1993": {"AnnexChoice": ANNEX},
    "en1999": {"AnnexChoice": ANNEX},
    "en1998": {
        "DeSeismicZone": {"zone0": ("Zone 0", "Erdbebenzone 0"), "zone1": ("Zone 1", "Erdbebenzone 1"), "zone2": ("Zone 2", "Erdbebenzone 2"), "zone3": ("Zone 3", "Erdbebenzone 3")},
        "DeGroundCombo": {value: (f"Ground class {value[0]}, subsoil class {value[2]}", f"Baugrundklasse {value[0]}, Untergrundklasse {value[2]}") for value in ["A-R", "B-R", "C-R", "B-T", "C-T", "C-S"]},
    },
    "vdi3805": {
        "EditionProfileChoice": {"legacy": ("Legacy edition", "Frühere Ausgabe"), "current": ("Current edition", "Aktuelle Ausgabe")},
        "VdiQuantityKind": {"dimensionless": ("Dimensionless", "Dimensionslos"), "length": ("Length", "Länge"), "area": ("Area", "Fläche"), "volume": ("Volume", "Volumen"), "mass": ("Mass", "Masse"), "time": ("Time", "Zeit"), "temperature": ("Temperature", "Temperatur"), "force": ("Force", "Kraft"), "pressure": ("Pressure", "Druck"), "stress": ("Stress", "Spannung"), "moment": ("Moment", "Moment"), "energy": ("Energy", "Energie"), "power": ("Power", "Leistung"), "thermalConductivity": ("Thermal conductivity", "Wärmeleitfähigkeit"), "thermalResistance": ("Thermal resistance", "Wärmedurchlasswiderstand"), "heatTransferCoefficient": ("Heat transfer coefficient", "Wärmeübergangskoeffizient"), "airPermeability": ("Air permeability", "Luftdurchlässigkeit"), "ventilationRate": ("Ventilation rate", "Luftwechselrate"), "acceleration": ("Acceleration", "Beschleunigung")},
    },
    "iso16757": {
        "ExchangeProcess": {"CreateFromDictionary": ("Create from dictionary", "Aus Merkmalsverzeichnis erstellen"), "ProvideCatalogue": ("Provide catalogue", "Katalog bereitstellen"), "DetermineProduct": ("Determine product", "Produkt bestimmen"), "IntegrateIntoSystem": ("Integrate into system", "In System einbinden"), "ExchangeSystemModel": ("Exchange system model", "Systemmodell austauschen")},
        "SubjectKind": {"ProductGroup": ("Product group", "Produktgruppe"), "ProductClass": ("Product class", "Produktklasse"), "ProductSpecialization": ("Product specialization", "Produktspezialisierung"), "CatalogueMetadata": ("Catalogue metadata", "Katalog-Metadaten"), "ManufacturerMetadata": ("Manufacturer metadata", "Hersteller-Metadaten"), "PropertyBlock": ("Property block", "Merkmalsblock"), "Port": ("Port", "Anschluss"), "Inlet": ("Inlet", "Eintritt"), "Outlet": ("Outlet", "Austritt"), "InOutlet": ("Inlet/outlet", "Ein-/Austritt")},
        "NullState": {"Unavailable": ("Unavailable", "Nicht verfügbar"), "Unknown": ("Unknown", "Unbekannt"), "NotApplicable": ("Not applicable", "Nicht zutreffend")},
        "PropertyKind": {"Static": ("Static", "Statisch"), "Dynamic": ("Dynamic", "Dynamisch"), "Selection": ("Selection", "Auswahl"), "External": ("External", "Extern")},
        "ConstraintOperator": {"Equal": ("Equal", "Gleich"), "NotEqual": ("Not equal", "Ungleich"), "LessThan": ("Less than", "Kleiner als"), "GreaterThan": ("Greater than", "Größer als"), "InRange": ("In range", "Im Bereich")},
        "BooleanOperator": {"Union": ("Union", "Vereinigung"), "Intersection": ("Intersection", "Schnittmenge"), "Difference": ("Difference", "Differenz")},
        "SpaceKind": {"Overall": ("Overall space", "Gesamtraum"), "Operation": ("Operating space", "Bedienraum"), "Access": ("Access space", "Zugangsraum"), "PlacementTransportation": ("Placement and transport space", "Einbring- und Transportraum"), "Installation": ("Installation space", "Montageraum")},
    },
    "en1994": {"AnnexChoice": ANNEX},
    "en1997": {"AnnexChoice": ANNEX},
    "en1995": {
        "AnnexChoice": ANNEX,
        "MemberRole": {"Beam": ("Beam — bending member", "Träger — Biegebauteil"), "Column": ("Column — compression member", "Stütze — Druckbauteil"), "Floor": ("Floor — joist or panel with vibration check", "Decke — Balken oder Platte mit Schwingungsnachweis"), "Bridge": ("Bridge — EN 1995-2 girder", "Brücke — Träger nach EN 1995-2")},
        "SupportType": {"SimplySupported": ("Simply supported", "Einfeldträger"), "Cantilever": ("Cantilever", "Kragarm"), "ContinuousTwoSpan": ("Continuous over two spans", "Zweifeldträger")},
    },
    "en1996": {
        "AnnexChoice": ANNEX,
        "DesignSituation": {"Persistent": ("Persistent", "Ständig"), "Transient": ("Transient", "Vorübergehend"), "Accidental": ("Accidental", "Außergewöhnlich"), "Seismic": ("Seismic", "Erdbeben")},
        "MasonryClass": {f"Class{n}": (f"Class {n}", f"Klasse {n}") for n in range(1, 6)},
        "ExposureClass": {f"Mx{n}": (f"MX{n}", f"MX{n}") for n in range(1, 6)},
        "MortarClass": {"M1": ("M1", "M1"), "M2_5": ("M2.5", "M2,5"), "M5": ("M5", "M5"), "M10": ("M10", "M10"), "M15": ("M15", "M15"), "M20": ("M20", "M20")},
        "MortarType": {"GeneralPurpose": ("General-purpose mortar", "Normalmauermörtel"), "ThinLayer": ("Thin-layer mortar", "Dünnbettmörtel"), "Lightweight": ("Lightweight mortar", "Leichtmauermörtel")},
        "UnitGroup": {f"Group{n}": (f"Group {n}", f"Gruppe {n}") for n in range(1, 5)},
        "UnitMaterial": {"Clay": ("Clay", "Ziegel"), "CalciumSilicate": ("Calcium silicate", "Kalksandstein"), "Aerated": ("Autoclaved aerated concrete", "Porenbeton"), "Concrete": ("Concrete", "Beton")},
        "WallType": {"LoadBearing": ("Load-bearing wall", "Tragende Wand"), "Shear": ("Shear wall", "Aussteifende Wand"), "NonLoadBearing": ("Non-load-bearing wall", "Nichttragende Wand")},
    },
}

#: 🏷️ Leaf input `x-semio-ui` per artifact → semantic kind → wire field, for inputs without a W2-R annotation.
INDEX = L("Position", "Position", widget="stepper", step=1, precision=0, description=D("Zero-based list position.", "Nullbasierte Listenposition."))
LEAVES = {
    "en1990": {
        "change-reference-period-years": {"newReferencePeriodYears": num("Reference period", "Bezugszeitraum", "a", precision=0)},
        "change-permanents": {"newPermanents": rows("Permanent actions", "Ständige Einwirkungen")},
        "change-consequence-class": {"newConsequenceClass": count("Consequence class CC", "Schadensfolgeklasse CC", D("1 = CC1, 2 = CC2, 3 = CC3 (EN 1990 Annex B).", "1 = CC1, 2 = CC2, 3 = CC3 (EN 1990 Anhang B)."))},
        "change-altitude-m": {"newAltitudeM": metre("Site altitude above sea level", "Geländehöhe über NN", precision=0)},
        "remove-effect": {"index": INDEX},
        "insert-permanent": {"index": INDEX, "item": rows("Permanent action", "Ständige Einwirkung")},
        "remove-permanent": {"index": INDEX},
        "change-bridge-sls": {"newBridgeSls": rows("Bridge serviceability criteria", "Gebrauchstauglichkeitskriterien der Brücke")},
        "insert-seismic": {"index": INDEX, "item": rows("Seismic action", "Erdbebeneinwirkung")},
        "change-seismics": {"newSeismics": rows("Seismic actions", "Erdbebeneinwirkungen")},
        "change-annex": {"newAnnex": L("National annex", "Nationaler Anhang", widget="segmented")},
        "change-reliability-class": {"newReliabilityClass": count("Reliability class RC", "Zuverlässigkeitsklasse RC", D("1 = RC1, 2 = RC2, 3 = RC3 (EN 1990 Annex B).", "1 = RC1, 2 = RC2, 3 = RC3 (EN 1990 Anhang B)."))},
        "change-variables": {"newVariables": rows("Variable actions", "Veränderliche Einwirkungen")},
        "change-members": {"newMembers": rows("Members", "Bauteile")},
        "change-project-id": {"newProjectId": text("Project identifier", "Projektkennung")},
        "change-supervision-level": {"newSupervisionLevel": text("Design supervision level", "Überwachungsstufe der Planung", D("DSL1, DSL2 or DSL3 (EN 1990 Table B4).", "DSL1, DSL2 oder DSL3 (EN 1990 Tabelle B4)."))},
        "insert-accidental": {"index": INDEX, "item": rows("Accidental action", "Außergewöhnliche Einwirkung")},
        "change-accidentals": {"newAccidentals": rows("Accidental actions", "Außergewöhnliche Einwirkungen")},
        "change-design-working-life-category": {"newDesignWorkingLifeCategory": count("Design working life category", "Kategorie der geplanten Nutzungsdauer", D("Category 1 to 5 (EN 1990 Table 2.1).", "Kategorie 1 bis 5 (EN 1990 Tabelle 2.1)."))},
        "change-design-working-life-years": {"newDesignWorkingLifeYears": num("Design working life", "Geplante Nutzungsdauer", "a", precision=0)},
        "insert-effect": {"index": INDEX, "item": rows("Member effect", "Beanspruchungszuordnung")},
        "change-beta-computed": {"newBetaComputed": pure("Computed reliability index β", "Berechneter Zuverlässigkeitsindex β")},
        "remove-variable": {"index": INDEX},
        "insert-variable": {"index": INDEX, "item": rows("Variable action", "Veränderliche Einwirkung")},
        "change-inspection-level": {"newInspectionLevel": text("Inspection level", "Inspektionsstufe", D("IL1, IL2 or IL3 (EN 1990 Table B5).", "IL1, IL2 oder IL3 (EN 1990 Tabelle B5)."))},
        "change-effects": {"newEffects": rows("Member effects", "Beanspruchungszuordnungen")},
        "insert-member": {"index": INDEX, "item": rows("Member", "Bauteil")},
        "remove-seismic": {"index": INDEX},
        "remove-accidental": {"index": INDEX},
        "remove-member": {"index": INDEX},
    },
    "en1993": {
        "update-bolt-inputs": {"joint": rows("Joint", "Anschluss", D("The complete joint record replacing the addressed joint.", "Der vollständige Anschluss, der den adressierten Anschluss ersetzt."))},
        "update-bridge-inputs": {"bridgeFatigueItem": rows("Bridge fatigue data", "Ermüdungsangaben der Brücke")},
        "update-cold-formed-inputs": {"coldFormedMember": rows("Cold-formed member", "Kaltgeformtes Bauteil")},
        "update-crane-inputs": {"craneRunway": rows("Crane runway", "Kranbahn")},
        "update-fatigue-inputs": {"fatigueDetail": rows("Fatigue detail", "Ermüdungsdetail")},
        "update-fire-inputs": {"fireExposure": rows("Fire exposure", "Brandbeanspruchung")},
        "update-pile-inputs": {"pile": rows("Steel pile", "Stahlpfahl")},
        "update-plated-inputs": {"platedPanel": rows("Plated panel", "Beulfeld")},
        "update-silo-shell-inputs": {"siloShell": rows("Silo shell", "Siloschale")},
        "update-tension-component-inputs": {"tensionComponent": rows("Tension component", "Zugglied")},
        "update-tower-inputs": {"towerLeg": rows("Tower leg", "Turmeckstiel")},
        "update-weld-inputs": {"memberAction": rows("Member action", "Bauteilbeanspruchung")},
        "update-member-properties": {"member": rows("Member", "Bauteil")},
        "update-stainless-inputs": {"material": rows("Stainless steel material", "Nichtrostender Stahlwerkstoff")},
        "update-hss-inputs": {"loadCase": rows("Load case", "Lastfall")},
        "update-through-thickness-inputs": {"section": rows("Cross-section", "Querschnitt")},
        "insert-section": {"section": rows("Cross-section", "Querschnitt", D("The complete cross-section record to insert.", "Der vollständige einzufügende Querschnitt."))},
    },
    "en1999": {
        "change-member-my-ed": {"newMYK": moment("Characteristic bending moment M_y,k", "Charakteristisches Biegemoment M_y,k")},
        "change-member-n-ed": {"newNK": force("Characteristic axial force N_k", "Charakteristische Normalkraft N_k")},
    },
}
