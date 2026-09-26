//! 🏷️ EN 1998 NormFieldMeta lookup for the structured Inputs editor (B2 hook).

use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

/// 🏷️ Family field-metadata table — wire values keep codes; labels are localized prose.
pub fn en1998_field_meta(path: &str) -> Option<NormFieldMeta> {
    lookup_norm_field_meta(TABLE, path)
}

const ZONES: &[NormFieldChoice] = &[
    NormFieldChoice { value: "zone0", label_en: "Zone 0", label_de: "Erdbebenzone 0" },
    NormFieldChoice { value: "zone1", label_en: "Zone 1", label_de: "Erdbebenzone 1" },
    NormFieldChoice { value: "zone2", label_en: "Zone 2", label_de: "Erdbebenzone 2" },
    NormFieldChoice { value: "zone3", label_en: "Zone 3", label_de: "Erdbebenzone 3" },
];
const DE_GROUND: &[NormFieldChoice] = &[
    NormFieldChoice { value: "A-R", label_en: "A-R (rock)", label_de: "A-R (Fels)" },
    NormFieldChoice { value: "B-R", label_en: "B-R (shallow rock)", label_de: "B-R (flacher Fels)" },
    NormFieldChoice { value: "C-R", label_en: "C-R (deep rock)", label_de: "C-R (tiefer Fels)" },
    NormFieldChoice { value: "B-T", label_en: "B-T (transition)", label_de: "B-T (Übergang)" },
    NormFieldChoice { value: "C-T", label_en: "C-T (transition)", label_de: "C-T (Übergang)" },
    NormFieldChoice { value: "C-S", label_en: "C-S (soft soil)", label_de: "C-S (weicher Boden)" },
];
const IMPORTANCE: &[NormFieldChoice] = &[
    NormFieldChoice { value: "I", label_en: "Class I", label_de: "Bedeutungskategorie I" },
    NormFieldChoice { value: "II", label_en: "Class II", label_de: "Bedeutungskategorie II" },
    NormFieldChoice { value: "III", label_en: "Class III", label_de: "Bedeutungskategorie III" },
    NormFieldChoice { value: "IV", label_en: "Class IV", label_de: "Bedeutungskategorie IV" },
];
const ANNEX: &[NormFieldChoice] = &[
    NormFieldChoice { value: "de", label_en: "German NA", label_de: "Deutscher NA" },
    NormFieldChoice { value: "en", label_en: "EN recommended", label_de: "EN empfohlenungen" },
];
const T1: &[NormFieldChoice] = &[
    NormFieldChoice { value: "ct", label_en: "Simplified (Ct·H^¾)", label_de: "Vereinfacht (Ct·H^¾)" },
    NormFieldChoice { value: "rayleigh", label_en: "Rayleigh", label_de: "Rayleigh" },
    NormFieldChoice { value: "given", label_en: "Given period", label_de: "Vorgegebene Periode" },
];
const DRIFT: &[NormFieldChoice] = &[
    NormFieldChoice { value: "brittle", label_en: "Brittle non-structural", label_de: "Spröde nichttragend" },
    NormFieldChoice { value: "ductile", label_en: "Ductile non-structural", label_de: "Duktil nichttragend" },
    NormFieldChoice { value: "isolated", label_en: "Isolated non-structural", label_de: "Getrennt nichttragend" },
];
const DIR: &[NormFieldChoice] = &[
    NormFieldChoice { value: "x", label_en: "X direction", label_de: "Richtung X" },
    NormFieldChoice { value: "y", label_en: "Y direction", label_de: "Richtung Y" },
];
const SYS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "frame", label_en: "Moment frame", label_de: "Rahmen" },
    NormFieldChoice { value: "wall", label_en: "Wall system", label_de: "Wandsystem" },
    NormFieldChoice { value: "dual", label_en: "Dual system", label_de: "Dualsystem" },
    NormFieldChoice { value: "masonry", label_en: "Masonry", label_de: "Mauerwerk" },
];
const MAT: &[NormFieldChoice] = &[
    NormFieldChoice { value: "rc", label_en: "Reinforced concrete", label_de: "Stahlbeton" },
    NormFieldChoice { value: "steel", label_en: "Steel", label_de: "Stahl" },
    NormFieldChoice { value: "composite", label_en: "Composite", label_de: "Verbund" },
    NormFieldChoice { value: "timber", label_en: "Timber", label_de: "Holz" },
    NormFieldChoice { value: "masonry", label_en: "Masonry", label_de: "Mauerwerk" },
];
const DC: &[NormFieldChoice] = &[
    NormFieldChoice { value: "dcl", label_en: "DCL (low)", label_de: "DCL (niedrig)" },
    NormFieldChoice { value: "dcm", label_en: "DCM (medium)", label_de: "DCM (mittel)" },
    NormFieldChoice { value: "dch", label_en: "DCH (high)", label_de: "DCH (hoch)" },
];
const EN_GROUND: &[NormFieldChoice] = &[
    NormFieldChoice { value: "A", label_en: "Ground type A", label_de: "Baugrundtyp A" },
    NormFieldChoice { value: "B", label_en: "Ground type B", label_de: "Baugrundtyp B" },
    NormFieldChoice { value: "C", label_en: "Ground type C", label_de: "Baugrundtyp C" },
    NormFieldChoice { value: "D", label_en: "Ground type D", label_de: "Baugrundtyp D" },
    NormFieldChoice { value: "E", label_en: "Ground type E", label_de: "Baugrundtyp E" },
];
const EN_SPEC: &[NormFieldChoice] = &[
    NormFieldChoice { value: "type1", label_en: "Type 1 spectrum", label_de: "Spektrum Typ 1" },
    NormFieldChoice { value: "type2", label_en: "Type 2 spectrum", label_de: "Spektrum Typ 2" },
];
const BOOL: &[NormFieldChoice] = &[
    NormFieldChoice { value: "true", label_en: "Yes", label_de: "Ja" },
    NormFieldChoice { value: "false", label_en: "No", label_de: "Nein" },
];
const ROLE: &[NormFieldChoice] = &[
    NormFieldChoice { value: "column", label_en: "Column", label_de: "Stütze" },
    NormFieldChoice { value: "beam", label_en: "Beam", label_de: "Träger" },
    NormFieldChoice { value: "wall", label_en: "Wall", label_de: "Wand" },
];

macro_rules! m {
    ($en:expr, $de:expr) => {
        NormFieldMeta { label_en: $en, label_de: $de, unit: None, choices: None }
    };
    ($en:expr, $de:expr, $u:expr) => {
        NormFieldMeta { label_en: $en, label_de: $de, unit: Some($u), choices: None }
    };
    ($en:expr, $de:expr, choices $c:expr) => {
        NormFieldMeta { label_en: $en, label_de: $de, unit: None, choices: Some($c) }
    };
}


const IMPOSED_CAT: &[NormFieldChoice] = &[
    NormFieldChoice { value: "A", label_en: "Residential (A)", label_de: "Wohnen (A)" },
    NormFieldChoice { value: "B", label_en: "Office (B)", label_de: "Büro (B)" },
    NormFieldChoice { value: "C", label_en: "Congregation (C)", label_de: "Versammlung (C)" },
    NormFieldChoice { value: "D", label_en: "Retail (D)", label_de: "Verkauf (D)" },
    NormFieldChoice { value: "E", label_en: "Storage (E)", label_de: "Lager (E)" },
    NormFieldChoice { value: "F", label_en: "Traffic (F)", label_de: "Verkehr (F)" },
    NormFieldChoice { value: "G", label_en: "Vehicles (G)", label_de: "Fahrzeuge (G)" },
    NormFieldChoice { value: "H", label_en: "Roofs (H)", label_de: "Dächer (H)" },
    NormFieldChoice { value: "snow", label_en: "Snow", label_de: "Schnee" },
    NormFieldChoice { value: "wind", label_en: "Wind", label_de: "Wind" },
];

const LIMIT_STATE: &[NormFieldChoice] = &[
    NormFieldChoice { value: "nc", label_en: "Near collapse (NC)", label_de: "Nahe dem Kollaps (NC)" },
    NormFieldChoice { value: "sd", label_en: "Significant damage (SD)", label_de: "Bedeutende Schädigung (SD)" },
    NormFieldChoice { value: "dl", label_en: "Damage limitation (DL)", label_de: "Schadensbegrenzung (DL)" },
];

const TABLE: &[(&str, NormFieldMeta)] = &[
    ("annex", m!("National annex", "Nationaler Anhang", choices ANNEX)),
    ("site.seismicZone", m!("Seismic zone", "Erdbebenzone", choices ZONES)),
    ("site.aGr", NormFieldMeta { label_en: "EN reference PGA a_gR (DE: derived from zone)", label_de: "EN-Bezugsspitzenwert a_gR (DE: aus Zone)", unit: Some("m/s²"), choices: None }),
    ("site.deGroundCombo", m!("DE-NA ground combination", "DE-NA-Untergrundkombination", choices DE_GROUND)),
    ("site.enGroundType", m!("EN ground type", "EN-Baugrundtyp", choices EN_GROUND)),
    ("site.enSpectrumType", m!("EN spectrum type", "EN-Spektrumtyp", choices EN_SPEC)),
    ("site.importanceClass", m!("Importance class", "Bedeutungskategorie", choices IMPORTANCE)),
    ("buildings", m!("Buildings", "Hochbauten")),
    ("bridges", m!("Bridges", "Brücken")),
    ("assessments", m!("Retrofit assessments", "Bestandsbewertungen")),
    ("silos", m!("Silos", "Silos")),
    ("tanks", m!("Tanks", "Tanks")),
    ("foundations", m!("Foundations", "Gründungen")),
    ("retainingWalls", m!("Retaining walls", "Stützwände")),
    ("towers", m!("Towers", "Türme")),
    ("buildings[].id", m!("Building id", "Gebäude-ID")),
    ("buildings[].name", m!("Building name", "Gebäudebezeichnung")),
    ("buildings[].planWidthM", m!("Plan width Lx", "Grundrissbreite Lx", "m")),
    ("buildings[].planLengthM", m!("Plan length Ly", "Grundrisslänge Ly", "m")),
    ("buildings[].planRegular", m!("Regular in plan", "Regelmässig im Grundriss", choices BOOL)),
    ("buildings[].elevationRegular", m!("Regular in elevation", "Regelmässig im Aufriss", choices BOOL)),
    ("buildings[].t1Method", m!("T₁ method", "T₁-Verfahren", choices T1)),
    ("buildings[].t1GivenS", m!("Given fundamental period T₁", "Vorgegebene Eigenperiode T₁", "s")),
    ("buildings[].ct", m!("Ct coefficient", "Ct-Beiwert")),
    ("buildings[].driftLimitClass", m!("Drift limit class", "Verschiebungsgrenzklasse", choices DRIFT)),
    ("buildings[].nu", m!("Reduction factor ν", "Abminderungsfaktor ν")),
    ("buildings[].multipleResistingSystems", m!("Multiple resisting systems", "Mehrere Aussteifungssysteme", choices BOOL)),
    ("buildings[].claimsSimpleMasonry", m!("Claims simple masonry rules", "Einfache Mauerwerksregeln", choices BOOL)),
    ("buildings[].masonryWallAreaRatio", m!("Masonry wall area ratio", "Mauerwerks-Wandflächenanteil")),
    ("buildings[].accidentalEccentricityRatio", m!("Accidental eccentricity ratio", "Zufällige Exzentrizität")),
    ("buildings[].systems", m!("Structural systems", "Aussteifungssysteme")),
    ("buildings[].systems[].id", m!("System id", "System-ID")),
    ("buildings[].systems[].direction", m!("Direction", "Richtung", choices DIR)),
    ("buildings[].systems[].systemType", m!("System type", "Systemtyp", choices SYS)),
    ("buildings[].systems[].material", m!("Material", "Baustoff", choices MAT)),
    ("buildings[].systems[].ductilityClass", m!("Ductility class", "Duktilitätsklasse", choices DC)),
    ("buildings[].systems[].q0", m!("Basic behaviour factor q₀", "Grundwert Verhaltensbeiwert q₀")),
    ("buildings[].systems[].alphaUOverAlpha1", m!("αu/α1", "αu/α1")),
    ("buildings[].systems[].kW", m!("Wall factor kw", "Wandfaktor kw")),
    ("buildings[].systems[].baseShearResistanceN", m!("Base shear resistance VRd", "Querkrafttragfähigkeit VRd", "N")),
    ("buildings[].storeys", m!("Storeys", "Geschosse")),
    ("buildings[].storeys[].id", m!("Storey id", "Geschoss-ID")),
    ("buildings[].storeys[].heightM", m!("Storey height", "Geschosshöhe", "m")),
    ("buildings[].storeys[].permanentGkN", m!("Permanent action ΣG_k", "Ständige Einwirkung ΣG_k", "N")),
    ("buildings[].storeys[].correlatedOccupancy", m!("Correlated occupancy (Table 4.2 φ)", "Korrelierte Nutzung (Tab. 4.2 φ)", choices BOOL)),
    ("buildings[].storeys[].variables", m!("Variable actions Q_k,i", "Veränderliche Einwirkungen Q_k,i")),
    ("buildings[].storeys[].variables[].id", m!("Variable action id", "Veränderliche-Einwirkung-ID")),
    ("buildings[].storeys[].variables[].category", m!("Imposed-load category", "Nutzlastkategorie", choices IMPOSED_CAT)),
    ("buildings[].storeys[].variables[].qkN", m!("Variable action Q_k", "Veränderliche Einwirkung Q_k", "N")),
    ("buildings[].storeys[].stiffnessX", m!("Stiffness X", "Steifigkeit X", "N/m")),
    ("buildings[].storeys[].stiffnessY", m!("Stiffness Y", "Steifigkeit Y", "N/m")),
    ("buildings[].storeys[].centreOfMassXM", m!("Centre of mass X", "Massenschwerpunkt X", "m")),
    ("buildings[].storeys[].centreOfMassYM", m!("Centre of mass Y", "Massenschwerpunkt Y", "m")),
    ("buildings[].storeys[].centreOfStiffnessXM", m!("Centre of stiffness X", "Steifigkeitsschwerpunkt X", "m")),
    ("buildings[].storeys[].centreOfStiffnessYM", m!("Centre of stiffness Y", "Steifigkeitsschwerpunkt Y", "m")),
    ("buildings[].storeys[].driftXM", m!("Interstorey drift X", "Stockwerksverschiebung X", "m")),
    ("buildings[].storeys[].driftYM", m!("Interstorey drift Y", "Stockwerksverschiebung Y", "m")),
    ("buildings[].storeys[].shearResistanceN", m!("Storey shear resistance", "Geschoss-Querkrafttragfähigkeit", "N")),
    ("buildings[].members", m!("Members", "Bauteile")),
    ("buildings[].members[].id", m!("Member id", "Bauteil-ID")),
    ("buildings[].members[].material", m!("Member material", "Bauteil-Baustoff", choices MAT)),
    ("buildings[].members[].role", m!("Member role", "Bauteilrolle", choices ROLE)),
    ("buildings[].members[].detailingCompatibleWithQ", m!("q-compatible detailing", "q-gerechte Durchbildung", choices BOOL)),
    ("buildings[].members[].minDimensionM", m!("Min cross-section dimension", "Mindestquerschnittsabmessung", "m")),
    ("buildings[].members[].rho", m!("Longitudinal reinforcement ratio ρ", "Längsbewehrungsgrad ρ")),
    ("buildings[].members[].rhoPrime", m!("Compression reinforcement ratio ρ′", "Druckbewehrungsgrad ρ′")),
    ("buildings[].members[].omegaWd", m!("Confinement ratio ω_wd", "Umschnürungsgrad ω_wd")),
    ("buildings[].members[].steelSectionClass", m!("Steel section class", "Stahl-Querschnittsklasse")),
    ("bridges[].id", m!("Bridge id", "Brücken-ID")),
    ("bridges[].periodRatio", m!("Isolation period ratio", "Isolations-Periodenverhältnis")),
    ("bridges[].fundamentalPeriodS", m!("Fundamental period T", "Eigenperiode T", "s")),
    ("bridges[].vRdN", m!("Shear resistance", "Querkrafttragfähigkeit", "N")),
    ("bridges[].bearingDRdM", m!("Bearing displacement capacity", "Lagerverschiebungskapazität", "m")),
    ("bridges[].permanentGkN", m!("Permanent action ΣG_k", "Ständige Einwirkung ΣG_k", "N")),
    ("bridges[].correlatedOccupancy", m!("Correlated occupancy (Table 4.2 φ)", "Korrelierte Nutzung (Tab. 4.2 φ)", choices BOOL)),
    ("bridges[].variables", m!("Variable actions Q_k,i", "Veränderliche Einwirkungen Q_k,i")),
    ("bridges[].variables[].id", m!("Variable action id", "Veränderliche-Einwirkung-ID")),
    ("bridges[].variables[].category", m!("Imposed-load category", "Nutzlastkategorie", choices IMPOSED_CAT)),
    ("bridges[].variables[].qkN", m!("Variable action Q_k", "Veränderliche Einwirkung Q_k", "N")),
    ("assessments[].id", m!("Assessment id", "Bewertungs-ID")),
    ("assessments[].knowledgeLevel", m!("Knowledge level", "Kenntnisstand")),
    ("assessments[].limitState", m!("Limit state NC/SD/DL", "Grenzzustand NC/SD/DL", choices LIMIT_STATE)),
    ("assessments[].supportedBuildingId", m!("Supported building id", "Gestütztes Gebäude")),
    ("assessments[].rKN", m!("Characteristic resistance Rk", "Charakteristischer Widerstand Rk", "N")),
    ("assessments[].gammaEl", m!("γel", "γel")),
    ("silos[].id", m!("Silo id", "Silo-ID")),
    ("silos[].heightM", m!("Silo height", "Silohöhe", "m")),
    ("silos[].radiusM", m!("Silo radius", "Siloradius", "m")),
    ("silos[].permanentGkN", m!("Structure permanent ΣG_k", "Tragwerk ständige ΣG_k", "N")),
    ("silos[].contentQkN", m!("Content action Q_k (full)", "Füllgut-Einwirkung Q_k (voll)", "N")),
    ("silos[].contentCategory", m!("Content category", "Füllgutkategorie", choices IMPOSED_CAT)),
    ("silos[].fillingRatio", m!("Filling ratio", "Füllgrad")),
    ("silos[].nRdN", m!("Axial resistance", "Normalkrafttragfähigkeit", "N")),
    ("silos[].vRdN", m!("Shear / anchorage resistance", "Querkraft-/Verankerungstragfähigkeit", "N")),
    ("silos[].qNominal", m!("Nominal q", "Nennwert q")),
    ("tanks[].id", m!("Tank id", "Tank-ID")),
    ("tanks[].heightM", m!("Tank height", "Tankhöhe", "m")),
    ("tanks[].radiusM", m!("Tank radius", "Tankradius", "m")),
    ("tanks[].permanentGkN", m!("Structure permanent ΣG_k", "Tragwerk ständige ΣG_k", "N")),
    ("tanks[].contentQkN", m!("Content action Q_k (full)", "Füllgut-Einwirkung Q_k (voll)", "N")),
    ("tanks[].contentCategory", m!("Content category", "Füllgutkategorie", choices IMPOSED_CAT)),
    ("tanks[].fillingRatio", m!("Filling ratio", "Füllgrad")),
    ("tanks[].vRdN", m!("Shear resistance", "Querkrafttragfähigkeit", "N")),
    ("foundations[].id", m!("Foundation id", "Gründungs-ID")),
    ("foundations[].supportedBuildingId", m!("Supported building id", "Gestütztes Gebäude")),
    ("foundations[].areaM2", m!("Foundation area", "Gründungsfläche", "m²")),
    ("foundations[].pRdPa", m!("Bearing resistance", "Sohldruckwiderstand", "Pa")),
    ("foundations[].hRdN", m!("Sliding resistance", "Gleitwiderstand", "N")),
    ("foundations[].kFoundation", m!("Foundation stiffness", "Gründungssteifigkeit")),
    ("foundations[].kSoil", m!("Soil stiffness", "Bodensteifigkeit")),
    ("retainingWalls[].id", m!("Wall id", "Wand-ID")),
    ("retainingWalls[].heightM", m!("Wall height", "Wandhöhe", "m")),
    ("retainingWalls[].phiDeg", m!("Friction angle φ′", "Reibungswinkel φ′")),
    ("retainingWalls[].soilGamma", m!("Soil unit weight", "Wichte Boden")),
    ("retainingWalls[].r", m!("Reduction factor r", "Abminderungsfaktor r")),
    ("retainingWalls[].hRdNPerM", m!("Thrust resistance", "Schubwiderstand", "N/m")),
    ("towers[].id", m!("Tower id", "Turm-ID")),
    ("towers[].heightM", m!("Tower height", "Turmhöhe", "m")),
    ("towers[].mRdNm", m!("Moment resistance", "Momententragfähigkeit", "N·m")),
    ("towers[].isChimney", m!("Chimney", "Schornstein", choices BOOL)),
    ("towers[].qNominal", m!("Nominal q", "Nennwert q")),
    ("towers[].permanentGkN", m!("Permanent action ΣG_k", "Ständige Einwirkung ΣG_k", "N")),
    ("towers[].correlatedOccupancy", m!("Correlated occupancy (Table 4.2 φ)", "Korrelierte Nutzung (Tab. 4.2 φ)", choices BOOL)),
    ("towers[].variables", m!("Variable actions Q_k,i", "Veränderliche Einwirkungen Q_k,i")),
    ("towers[].variables[].id", m!("Variable action id", "Veränderliche-Einwirkung-ID")),
    ("towers[].variables[].category", m!("Imposed-load category", "Nutzlastkategorie", choices IMPOSED_CAT)),
    ("towers[].variables[].qkN", m!("Variable action Q_k", "Veränderliche Einwirkung Q_k", "N")),
];
