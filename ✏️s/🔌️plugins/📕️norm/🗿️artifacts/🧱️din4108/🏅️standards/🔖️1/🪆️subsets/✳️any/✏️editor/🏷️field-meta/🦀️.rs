//! 🏷️ DIN 4108 FieldSpec lookup — SI display units + human en/de choice labels (`[]` wildcards).

use semio_s_plugin_norm_app_surface::{lookup_norm_field_meta, NormFieldChoice, FieldSpec, UnitId};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const CLIMATE: &[NormFieldChoice] = &[
    choice("Zone1", "Summer climate region A (cool)", "Sommerklimaregion A (sommerkühl)"),
    choice("Zone2", "Summer climate region B (moderate)", "Sommerklimaregion B (gemäßigt)"),
    choice("Zone3", "Summer climate region C (hot)", "Sommerklimaregion C (sommerheiß)"),
    choice("Zone4", "Summer climate region D", "Sommerklimaregion D"),
];
const USAGE: &[NormFieldChoice] = &[
    choice("residential", "Residential", "Wohngebäude"),
    choice("nonResidential", "Non-residential", "Nichtwohngebäude"),
];
const BOOL: &[NormFieldChoice] = &[choice("true", "Yes", "Ja"), choice("false", "No", "Nein")];
const HEAVINESS: &[NormFieldChoice] = &[
    choice("heavy", "Heavy construction", "Schwere Bauweise"),
    choice("medium", "Medium construction", "Mittlere Bauweise"),
    choice("light", "Light construction", "Leichte Bauweise"),
];
const NIGHT: &[NormFieldChoice] = &[
    choice("none", "No night ventilation", "Keine Nachtlüftung"),
    choice("moderate", "Moderate night ventilation", "Mittlere Nachtlüftung"),
    choice("high", "High night ventilation", "Erhöhte Nachtlüftung"),
];
const ORIENT: &[NormFieldChoice] = &[
    choice("N", "North", "Nord"),
    choice("NE", "Northeast", "Nordost"),
    choice("E", "East", "Ost"),
    choice("SE", "Southeast", "Südost"),
    choice("S", "South", "Süd"),
    choice("SW", "Southwest", "Südwest"),
    choice("W", "West", "West"),
    choice("NW", "Northwest", "Nordwest"),
];
const KIND: &[NormFieldChoice] = &[
    choice("wall", "Wall", "Wand"),
    choice("roof", "Roof", "Dach"),
    choice("floor", "Floor", "Fußboden"),
    choice("window", "Window", "Fenster"),
    choice("door", "Door", "Tür"),
    choice("frameOpaque", "Opaque frame part", "Opaker Rahmenanteil"),
    choice("rollerShutterBox", "Roller shutter box", "Rollladenkasten"),
];
const ADJACENT: &[NormFieldChoice] = &[
    choice("exterior", "Exterior", "Außenluft"),
    choice("ground", "Ground", "Erdreich"),
    choice("unheated", "Unheated space", "Unbeheizter Raum"),
    choice("otherHeated", "Other heated space", "Anderer beheizter Raum"),
];
const BB2: &[NormFieldChoice] = &[
    choice("categoryA", "Beiblatt 2 category A (equivalent)", "Beiblatt 2 Kategorie A (äquivalent)"),
    choice("categoryB", "Beiblatt 2 category B (equivalent)", "Beiblatt 2 Kategorie B (äquivalent)"),
    choice("detailed", "Detailed ψ (not catalogue-equivalent)", "Detailliertes ψ (nicht katalogäquivalent)"),
];

const APPLICATION: &[NormFieldChoice] = &[
    choice("DAD", "Roof external (DAD)", "Dach außenseitig (DAD)"),
    choice("DAA", "Roof under waterproofing (DAA)", "Dach unter Abdichtung (DAA)"),
    choice("DUK", "Inverted roof (DUK)", "Umkehrdach (DUK)"),
    choice("DZ", "Roof between rafters (DZ)", "Dach zwischen Sparren (DZ)"),
    choice("DI", "Internal insulation (DI)", "Innendämmung (DI)"),
    choice("DEO", "Floor under screed (DEO)", "Fußboden unter Estrich (DEO)"),
    choice("DES", "Floor impact sound (DES)", "Fußboden Trittschall (DES)"),
    choice("WAB", "External wall behind cladding (WAB)", "Außenwand hinter Bekleidung (WAB)"),
    choice("WAA", "External wall ventilated cladding (WAA)", "Außenwand hinterlüftet (WAA)"),
    choice("WAP", "ETICS external wall (WAP)", "WDVS Außenwand (WAP)"),
    choice("WZ", "Timber-frame cavity (WZ)", "Holzrahmen Gefach (WZ)"),
    choice("WH", "External wall timber (WH)", "Außenwand Holzbau (WH)"),
    choice("WI", "Internal wall insulation (WI)", "Innendämmung Wand (WI)"),
    choice("WTH", "Between rafters/timber (WTH)", "Zwischen Sparren/Holz (WTH)"),
    choice("WTR", "Rafter top (WTR)", "Sparrenaufsatz (WTR)"),
    choice("PW", "Perimeter wall (PW)", "Perimeterdämmung (PW)"),
    choice("PB", "Perimeter bottom (PB)", "Perimeterboden (PB)"),
];
const COMPRESSIVE: &[NormFieldChoice] = &[
    choice("dh", "Compressive load dh (low)", "Druckbeanspruchung dh (gering)"),
    choice("ds", "Compressive load ds", "Druckbeanspruchung ds"),
    choice("dm", "Compressive load dm (medium)", "Druckbeanspruchung dm (mittel)"),
    choice("dk", "Compressive load dk (high)", "Druckbeanspruchung dk (hoch)"),
    choice("dx", "Compressive load dx (extreme)", "Druckbeanspruchung dx (extrem)"),
];
const WATER: &[NormFieldChoice] = &[
    choice("wk", "Water absorption wk", "Wasseraufnahme wk"),
    choice("wf", "Water absorption wf", "Wasseraufnahme wf"),
    choice("wd", "Water absorption wd", "Wasseraufnahme wd"),
];
const TENSILE: &[NormFieldChoice] = &[
    choice("tk", "Tensile strength tk", "Zugfestigkeit tk"),
    choice("tf", "Tensile strength tf", "Zugfestigkeit tf"),
];
const ACOUSTIC: &[NormFieldChoice] = &[
    choice("sh", "Acoustic class sh", "Schwingungsgruppe sh"),
    choice("sm", "Acoustic class sm", "Schwingungsgruppe sm"),
    choice("sg", "Acoustic class sg", "Schwingungsgruppe sg"),
];

const TABLE: &[(&str, FieldSpec)] = &[
    ("climateZone", FieldSpec::choice("Climate zone", "Klimazone", CLIMATE)),
    ("usage", FieldSpec::choice("Usage", "Nutzung", USAGE)),
    ("tIntC", FieldSpec::quantity("Indoor air temperature", "Raumlufttemperatur", UnitId::Celsius)),
    ("rhInt", FieldSpec::quantity("Indoor relative humidity", "Relative Raumluftfeuchte", UnitId::One)),
    ("hasMechanicalVentilation", FieldSpec::choice("Mechanical ventilation", "Mechanische Lüftung", BOOL)),
    ("airtightnessN50", FieldSpec::quantity("Airtightness n₅₀", "Luftdichtheit n₅₀", UnitId::PerHour)),
    ("bb2DetailsConform", FieldSpec::choice("Beiblatt 2 details conform", "Beiblatt-2-Details konform", BOOL)),
    ("zones", FieldSpec::new("Thermal zones", "Thermische Zonen")),
    ("zones[].id", FieldSpec::new("Zone id", "Zonen-Id")),
    ("zones[].floorAreaM2", FieldSpec::quantity("Floor area", "Fußbodenfläche", UnitId::SquareMetre)),
    ("zones[].heaviness", FieldSpec::choice("Thermal heaviness", "Bauweise", HEAVINESS)),
    ("zones[].nightVentilation", FieldSpec::choice("Night ventilation", "Nachtlüftung", NIGHT)),
    ("zones[].windows", FieldSpec::new("Windows", "Fenster")),
    ("zones[].windows[].id", FieldSpec::new("Window id", "Fenster-Id")),
    ("zones[].windows[].orientation", FieldSpec::choice("Orientation", "Orientierung", ORIENT)),
    ("zones[].windows[].inclinationDeg", FieldSpec::quantity("Inclination", "Neigung", UnitId::Degree)),
    ("zones[].windows[].areaM2", FieldSpec::quantity("Area", "Fläche", UnitId::SquareMetre)),
    ("zones[].windows[].gValue", FieldSpec::quantity("Total solar energy transmittance g", "Gesamtenergiedurchlassgrad g", UnitId::One)),
    ("zones[].windows[].shadingFc", FieldSpec::quantity("Shading reduction factor F_c", "Abminderungsfaktor F_c", UnitId::One)),
    ("elements", FieldSpec::new("Envelope elements", "Hüllflächen")),
    ("elements[].id", FieldSpec::new("Element id", "Bauteil-Id")),
    ("elements[].kind", FieldSpec::choice("Element kind", "Bauteilart", KIND)),
    ("elements[].zoneId", FieldSpec::new("Zone id", "Zonen-Id")),
    ("elements[].orientationDeg", FieldSpec::quantity("Orientation angle", "Orientierungswinkel", UnitId::Degree)),
    ("elements[].inclinationDeg", FieldSpec::quantity("Inclination", "Neigung", UnitId::Degree)),
    ("elements[].adjacent", FieldSpec::choice("Adjacent space", "Angrenzender Bereich", ADJACENT)),
    ("elements[].areaM2", FieldSpec::quantity("Area", "Fläche", UnitId::SquareMetre)),
    ("elements[].deltaUG", FieldSpec::quantity("Air void correction ΔU_g", "Luftschichtkorrektur ΔU_g", UnitId::WattPerSquareMetreKelvin)),
    ("elements[].deltaUF", FieldSpec::quantity("Fastener correction ΔU_f", "Befestigungskorrektur ΔU_f", UnitId::WattPerSquareMetreKelvin)),
    ("elements[].deltaUR", FieldSpec::quantity("Inverted roof correction ΔU_r", "Umkehrdachkorrektur ΔU_r", UnitId::WattPerSquareMetreKelvin)),
    ("elements[].layers", FieldSpec::new("Layers (interior→exterior)", "Schichten (innen→außen)")),
    ("elements[].layers[].id", FieldSpec::new("Layer id", "Schicht-Id")),
    ("elements[].layers[].materialId", FieldSpec::new("Material id", "Material-Id")),
    ("elements[].layers[].thicknessM", FieldSpec::quantity("Thickness", "Dicke", UnitId::Metre)),
    ("elements[].layers[].lambda", FieldSpec::quantity("Design thermal conductivity λ", "Bemessungswärmeleitfähigkeit λ", UnitId::WattPerMetreKelvin)),
    ("elements[].layers[].mu", FieldSpec::quantity("Water vapour resistance factor μ", "Wasserdampf-Diffusionswiderstandszahl μ", UnitId::One)),
    ("elements[].layers[].density", FieldSpec::quantity("Density", "Rohdichte", UnitId::KilogramPerCubicMetre)),
    ("elements[].layers[].applicationType", FieldSpec::choice("DIN 4108-10 application type", "DIN 4108-10 Anwendungsgebiet", APPLICATION)),
    ("elements[].layers[].compressiveClass", FieldSpec::choice("Compressive property class", "Druckfestigkeitsklasse", COMPRESSIVE)),
    ("elements[].layers[].waterClass", FieldSpec::choice("Water-absorption class", "Wasseraufnahmeklasse", WATER)),
    ("elements[].layers[].tensileClass", FieldSpec::choice("Tensile property class", "Zugfestigkeitsklasse", TENSILE)),
    ("elements[].layers[].acousticClass", FieldSpec::choice("Acoustic property class", "Schwingungsgruppe", ACOUSTIC)),

    ("elements[].layers[].segments", FieldSpec::new("Inhomogeneous segments (ISO 6946 §6.7)", "Inhomogene Abschnitte (ISO 6946 §6.7)")),
    ("elements[].layers[].segments[].id", FieldSpec::new("Segment id", "Abschnitt-Id")),
    ("elements[].layers[].segments[].materialId", FieldSpec::new("Segment material id", "Abschnitts-Material-Id")),
    ("elements[].layers[].segments[].fraction", FieldSpec::quantity("Area fraction", "Flächenanteil", UnitId::One)),
    ("elements[].layers[].segments[].lambda", FieldSpec::quantity("Segment λ", "Abschnitt λ", UnitId::WattPerMetreKelvin)),
    ("elements[].layers[].segments[].mu", FieldSpec::quantity("Segment μ", "Abschnitt μ", UnitId::One)),
    ("elements[].layers[].segments[].density", FieldSpec::quantity("Segment density", "Abschnittsrohdichte", UnitId::KilogramPerCubicMetre)),
    ("thermalBridges", FieldSpec::new("Thermal bridges", "Wärmebrücken")),
    ("thermalBridges[].id", FieldSpec::new("Bridge id", "Wärmebrücken-Id")),
    ("thermalBridges[].psi", FieldSpec::quantity("Linear thermal transmittance ψ", "Längenbezogener Wärmedurchgangskoeffizient ψ", UnitId::WattPerMetreKelvin)),
    ("thermalBridges[].lengthM", FieldSpec::quantity("Length", "Länge", UnitId::Metre)),
    ("thermalBridges[].bb2Type", FieldSpec::choice("Beiblatt 2 equivalence category", "Beiblatt-2-Äquivalenzkategorie", BB2)),
];

/// 🏷️ Exact / `[]`-wildcard metadata for the DIN 4108 envelope subject.
pub fn din4108_field_meta(path: &str) -> Option<FieldSpec> {
    lookup_norm_field_meta(TABLE, path)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
