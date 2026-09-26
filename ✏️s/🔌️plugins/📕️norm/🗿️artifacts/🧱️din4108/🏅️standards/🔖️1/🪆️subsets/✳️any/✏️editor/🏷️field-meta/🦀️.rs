//! 🏷️ DIN 4108 NormFieldMeta lookup — SI display units + human en/de choice labels (`[]` wildcards).

use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {
    NormFieldChoice { value, label_en, label_de }
}

const CLIMATE: &[NormFieldChoice] = &[
    choice("zone1", "Summer climate region A", "Sommerklimaregion A"),
    choice("zone2", "Summer climate region B", "Sommerklimaregion B"),
    choice("zone3", "Summer climate region C", "Sommerklimaregion C"),
    choice("zone4", "Summer climate region C (alpine)", "Sommerklimaregion C (alpin)"),
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


const TABLE: &[(&str, NormFieldMeta)] = &[
    ("climateZone", NormFieldMeta { label_en: "Climate zone", label_de: "Klimazone", unit: None, choices: Some(CLIMATE) }),
    ("usage", NormFieldMeta { label_en: "Usage", label_de: "Nutzung", unit: None, choices: Some(USAGE) }),
    ("tIntC", NormFieldMeta { label_en: "Indoor air temperature", label_de: "Raumlufttemperatur", unit: Some("°C"), choices: None }),
    ("rhInt", NormFieldMeta { label_en: "Indoor relative humidity", label_de: "Relative Raumluftfeuchte", unit: Some("1"), choices: None }),
    ("hasMechanicalVentilation", NormFieldMeta { label_en: "Mechanical ventilation", label_de: "Mechanische Lüftung", unit: None, choices: Some(BOOL) }),
    ("airtightnessN50", NormFieldMeta { label_en: "Airtightness n₅₀", label_de: "Luftdichtheit n₅₀", unit: Some("1/h"), choices: None }),
    ("bb2DetailsConform", NormFieldMeta { label_en: "Beiblatt 2 details conform", label_de: "Beiblatt-2-Details konform", unit: None, choices: Some(BOOL) }),
    ("zones", NormFieldMeta { label_en: "Thermal zones", label_de: "Thermische Zonen", unit: None, choices: None }),
    ("zones[].id", NormFieldMeta { label_en: "Zone id", label_de: "Zonen-Id", unit: None, choices: None }),
    ("zones[].floorAreaM2", NormFieldMeta { label_en: "Floor area", label_de: "Fußbodenfläche", unit: Some("m²"), choices: None }),
    ("zones[].heaviness", NormFieldMeta { label_en: "Thermal heaviness", label_de: "Bauweise", unit: None, choices: Some(HEAVINESS) }),
    ("zones[].nightVentilation", NormFieldMeta { label_en: "Night ventilation", label_de: "Nachtlüftung", unit: None, choices: Some(NIGHT) }),
    ("zones[].windows", NormFieldMeta { label_en: "Windows", label_de: "Fenster", unit: None, choices: None }),
    ("zones[].windows[].id", NormFieldMeta { label_en: "Window id", label_de: "Fenster-Id", unit: None, choices: None }),
    ("zones[].windows[].orientation", NormFieldMeta { label_en: "Orientation", label_de: "Orientierung", unit: None, choices: Some(ORIENT) }),
    ("zones[].windows[].inclinationDeg", NormFieldMeta { label_en: "Inclination", label_de: "Neigung", unit: Some("°"), choices: None }),
    ("zones[].windows[].areaM2", NormFieldMeta { label_en: "Area", label_de: "Fläche", unit: Some("m²"), choices: None }),
    ("zones[].windows[].gValue", NormFieldMeta { label_en: "Total solar energy transmittance g", label_de: "Gesamtenergiedurchlassgrad g", unit: Some("1"), choices: None }),
    ("zones[].windows[].shadingFc", NormFieldMeta { label_en: "Shading reduction factor F_c", label_de: "Abminderungsfaktor F_c", unit: Some("1"), choices: None }),
    ("elements", NormFieldMeta { label_en: "Envelope elements", label_de: "Hüllflächen", unit: None, choices: None }),
    ("elements[].id", NormFieldMeta { label_en: "Element id", label_de: "Bauteil-Id", unit: None, choices: None }),
    ("elements[].kind", NormFieldMeta { label_en: "Element kind", label_de: "Bauteilart", unit: None, choices: Some(KIND) }),
    ("elements[].zoneId", NormFieldMeta { label_en: "Zone id", label_de: "Zonen-Id", unit: None, choices: None }),
    ("elements[].orientationDeg", NormFieldMeta { label_en: "Orientation angle", label_de: "Orientierungswinkel", unit: Some("°"), choices: None }),
    ("elements[].inclinationDeg", NormFieldMeta { label_en: "Inclination", label_de: "Neigung", unit: Some("°"), choices: None }),
    ("elements[].adjacent", NormFieldMeta { label_en: "Adjacent space", label_de: "Angrenzender Bereich", unit: None, choices: Some(ADJACENT) }),
    ("elements[].areaM2", NormFieldMeta { label_en: "Area", label_de: "Fläche", unit: Some("m²"), choices: None }),
    ("elements[].deltaUG", NormFieldMeta { label_en: "Air void correction ΔU_g", label_de: "Luftschichtkorrektur ΔU_g", unit: Some("W/(m²·K)"), choices: None }),
    ("elements[].deltaUF", NormFieldMeta { label_en: "Fastener correction ΔU_f", label_de: "Befestigungskorrektur ΔU_f", unit: Some("W/(m²·K)"), choices: None }),
    ("elements[].deltaUR", NormFieldMeta { label_en: "Inverted roof correction ΔU_r", label_de: "Umkehrdachkorrektur ΔU_r", unit: Some("W/(m²·K)"), choices: None }),
    ("elements[].layers", NormFieldMeta { label_en: "Layers (interior→exterior)", label_de: "Schichten (innen→außen)", unit: None, choices: None }),
    ("elements[].layers[].id", NormFieldMeta { label_en: "Layer id", label_de: "Schicht-Id", unit: None, choices: None }),
    ("elements[].layers[].materialId", NormFieldMeta { label_en: "Material id", label_de: "Material-Id", unit: None, choices: None }),
    ("elements[].layers[].thicknessM", NormFieldMeta { label_en: "Thickness", label_de: "Dicke", unit: Some("m"), choices: None }),
    ("elements[].layers[].lambda", NormFieldMeta { label_en: "Design thermal conductivity λ", label_de: "Bemessungswärmeleitfähigkeit λ", unit: Some("W/(m·K)"), choices: None }),
    ("elements[].layers[].mu", NormFieldMeta { label_en: "Water vapour resistance factor μ", label_de: "Wasserdampf-Diffusionswiderstandszahl μ", unit: Some("1"), choices: None }),
    ("elements[].layers[].density", NormFieldMeta { label_en: "Density", label_de: "Rohdichte", unit: Some("kg/m³"), choices: None }),
    ("elements[].layers[].applicationType", NormFieldMeta { label_en: "DIN 4108-10 application type", label_de: "DIN 4108-10 Anwendungsgebiet", unit: None, choices: Some(APPLICATION) }),
    ("elements[].layers[].compressiveClass", NormFieldMeta { label_en: "Compressive property class", label_de: "Druckfestigkeitsklasse", unit: None, choices: Some(COMPRESSIVE) }),
    ("elements[].layers[].waterClass", NormFieldMeta { label_en: "Water-absorption class", label_de: "Wasseraufnahmeklasse", unit: None, choices: Some(WATER) }),
    ("elements[].layers[].tensileClass", NormFieldMeta { label_en: "Tensile property class", label_de: "Zugfestigkeitsklasse", unit: None, choices: Some(TENSILE) }),
    ("elements[].layers[].acousticClass", NormFieldMeta { label_en: "Acoustic property class", label_de: "Schwingungsgruppe", unit: None, choices: Some(ACOUSTIC) }),

    ("elements[].layers[].segments", NormFieldMeta { label_en: "Inhomogeneous segments (ISO 6946 §6.7)", label_de: "Inhomogene Abschnitte (ISO 6946 §6.7)", unit: None, choices: None }),
    ("elements[].layers[].segments[].id", NormFieldMeta { label_en: "Segment id", label_de: "Abschnitt-Id", unit: None, choices: None }),
    ("elements[].layers[].segments[].materialId", NormFieldMeta { label_en: "Segment material id", label_de: "Abschnitts-Material-Id", unit: None, choices: None }),
    ("elements[].layers[].segments[].fraction", NormFieldMeta { label_en: "Area fraction", label_de: "Flächenanteil", unit: Some("1"), choices: None }),
    ("elements[].layers[].segments[].lambda", NormFieldMeta { label_en: "Segment λ", label_de: "Abschnitt λ", unit: Some("W/(m·K)"), choices: None }),
    ("elements[].layers[].segments[].mu", NormFieldMeta { label_en: "Segment μ", label_de: "Abschnitt μ", unit: Some("1"), choices: None }),
    ("elements[].layers[].segments[].density", NormFieldMeta { label_en: "Segment density", label_de: "Abschnittsrohdichte", unit: Some("kg/m³"), choices: None }),
    ("thermalBridges", NormFieldMeta { label_en: "Thermal bridges", label_de: "Wärmebrücken", unit: None, choices: None }),
    ("thermalBridges[].id", NormFieldMeta { label_en: "Bridge id", label_de: "Wärmebrücken-Id", unit: None, choices: None }),
    ("thermalBridges[].psi", NormFieldMeta { label_en: "Linear thermal transmittance ψ", label_de: "Längenbezogener Wärmedurchgangskoeffizient ψ", unit: Some("W/(m·K)"), choices: None }),
    ("thermalBridges[].lengthM", NormFieldMeta { label_en: "Length", label_de: "Länge", unit: Some("m"), choices: None }),
    ("thermalBridges[].bb2Type", NormFieldMeta { label_en: "Beiblatt 2 equivalence category", label_de: "Beiblatt-2-Äquivalenzkategorie", unit: None, choices: Some(BB2) }),
];

/// 🏷️ Exact / `[]`-wildcard metadata for the DIN 4108 envelope subject.
pub fn din4108_field_meta(path: &str) -> Option<NormFieldMeta> {
    lookup_norm_field_meta(TABLE, path)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
