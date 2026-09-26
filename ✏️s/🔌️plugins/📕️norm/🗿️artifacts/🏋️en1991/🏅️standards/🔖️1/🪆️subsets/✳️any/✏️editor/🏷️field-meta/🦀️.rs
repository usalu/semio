//! 🏷️ EN 1991 NormFieldMeta lookup — SI display units + en/de labels for the structured inputs editor.

use crate::app_surface::{NormFieldChoice, NormFieldMeta};

/// 🏷️ Exact-path (or leaf-segment) metadata for the EN 1991 design-load subject.
pub fn en1991_field_meta(path: &str) -> Option<NormFieldMeta> {
    let key = path.split('[').next().unwrap_or(path);
    if let Some(m) = exact(path).or_else(|| exact(key)) {
        return Some(m);
    }
    let leaf = path.rsplit(['.', '[']).next().unwrap_or(path).trim_end_matches(']');
    exact(leaf)
}

fn exact(path: &str) -> Option<NormFieldMeta> {
    Some(match path {
        "site" => NormFieldMeta { label_en: "Site / climate", label_de: "Standort / Klima", unit: None, choices: None },
        "building" => NormFieldMeta { label_en: "Building geometry", label_de: "Gebäudegeometrie", unit: None, choices: None },
        "thermal" => NormFieldMeta { label_en: "Thermal", label_de: "Temperatur", unit: None, choices: None },
        "execution" => NormFieldMeta { label_en: "Execution", label_de: "Ausführung", unit: None, choices: None },
        "bridge" => NormFieldMeta { label_en: "Bridge traffic", label_de: "Brückenverkehr", unit: None, choices: None },
        "crane" => NormFieldMeta { label_en: "Crane runway", label_de: "Kranbahn", unit: None, choices: None },
        "silo" => NormFieldMeta { label_en: "Silo / tank", label_de: "Silo / Tank", unit: None, choices: None },
        "annex" => NormFieldMeta { label_en: "National annex", label_de: "Nationaler Anhang", unit: None, choices: Some(&[NormFieldChoice { value: "en", label_en: "EN recommended values", label_de: "EN-Empfehlungen" }, NormFieldChoice { value: "de", label_en: "DIN EN German NA", label_de: "DIN EN deutscher NA" }]) },
        "snowZone" => NormFieldMeta { label_en: "Snow zone", label_de: "Schneelastzone", unit: None, choices: Some(&[NormFieldChoice { value: "1", label_en: "Zone 1", label_de: "Zone 1" }, NormFieldChoice { value: "1a", label_en: "Zone 1a", label_de: "Zone 1a" }, NormFieldChoice { value: "2", label_en: "Zone 2", label_de: "Zone 2" }, NormFieldChoice { value: "2a", label_en: "Zone 2a", label_de: "Zone 2a" }, NormFieldChoice { value: "3", label_en: "Zone 3", label_de: "Zone 3" }]) },
        "altitude" => NormFieldMeta { label_en: "Altitude", label_de: "Geländehöhe", unit: Some("m"), choices: None },
        "enSk" => NormFieldMeta { label_en: "EN characteristic snow sk", label_de: "EN charakteristische Schneelast sk", unit: Some("Pa"), choices: None },
        "exceptionalSnowNorthGermanLowlands" => NormFieldMeta { label_en: "Exceptional snow (N. German lowlands)", label_de: "Außergewöhnliche Schneelast (norddeutsches Tiefland)", unit: None, choices: None },
        "windZone" => NormFieldMeta { label_en: "Wind zone", label_de: "Windzone", unit: None, choices: Some(&[NormFieldChoice { value: "1", label_en: "Zone 1", label_de: "Zone 1" }, NormFieldChoice { value: "2", label_en: "Zone 2", label_de: "Zone 2" }, NormFieldChoice { value: "3", label_en: "Zone 3", label_de: "Zone 3" }, NormFieldChoice { value: "4", label_en: "Zone 4", label_de: "Zone 4" }]) },
        "enVb" => NormFieldMeta { label_en: "EN basic wind velocity vb", label_de: "EN Basiswindgeschwindigkeit vb", unit: Some("m/s"), choices: None },
        "terrainCategory" => NormFieldMeta { label_en: "Terrain category", label_de: "Geländekategorie", unit: None, choices: Some(&[NormFieldChoice { value: "0", label_en: "Category 0 — sea / open water", label_de: "Kategorie 0 — See / offenes Wasser" }, NormFieldChoice { value: "1", label_en: "Category I — lakes / flat open", label_de: "Kategorie I — Seen / flaches offenes Gelände" }, NormFieldChoice { value: "2", label_en: "Category II — farmland with hedges", label_de: "Kategorie II — landwirtschaftlich mit Hecken" }, NormFieldChoice { value: "3", label_en: "Category III — suburbs / continuous forest", label_de: "Kategorie III — Vororte / durchgehender Wald" }, NormFieldChoice { value: "4", label_en: "Category IV — urban / high buildings", label_de: "Kategorie IV — Stadtgebiet / hohe Gebäude" }]) },
        "mixedTerrainUpwind" => NormFieldMeta { label_en: "Mixed terrain upwind category", label_de: "Gemischtes Gelände luvseitig", unit: None, choices: None },
        "mixedTerrainDistance" => NormFieldMeta { label_en: "Mixed terrain distance", label_de: "Abstand gemischtes Gelände", unit: Some("m"), choices: None },
        "orographyFactor" => NormFieldMeta { label_en: "Orography factor co", label_de: "Geländefaktor co", unit: None, choices: None },
        "coastOrIsland" => NormFieldMeta { label_en: "Coast or island", label_de: "Küste oder Insel", unit: None, choices: None },
        "airDensity" => NormFieldMeta { label_en: "Air density", label_de: "Luftdichte", unit: Some("kg/m³"), choices: None },
        "height" => NormFieldMeta { label_en: "Building height", label_de: "Gebäudehöhe", unit: Some("m"), choices: None },
        "width" => NormFieldMeta { label_en: "Building width", label_de: "Gebäudebreite", unit: Some("m"), choices: None },
        "depth" => NormFieldMeta { label_en: "Building depth", label_de: "Gebäudetiefe", unit: Some("m"), choices: None },
        "assumedDeltaT" => NormFieldMeta { label_en: "Assumed temperature difference", label_de: "Angenommene Temperaturdifferenz", unit: Some("K"), choices: None },
        "constructionActivity" => NormFieldMeta { label_en: "Construction activity", label_de: "Bauzustandsaktivität", unit: None, choices: Some(&[NormFieldChoice { value: "scaffolding", label_en: "Scaffolding", label_de: "Gerüstbau" }, NormFieldChoice { value: "formwork", label_en: "Formwork", label_de: "Schalung" }, NormFieldChoice { value: "landing", label_en: "Landing / storage area", label_de: "Lagerfläche / Podest" }, NormFieldChoice { value: "working_platform", label_en: "Working platform", label_de: "Arbeitsplattform" }]) },
        "assumedConstructionQk" => NormFieldMeta { label_en: "Assumed construction imposed load", label_de: "Angenommene Bauzustandslast", unit: Some("Pa"), choices: None },
        "structureKind" => NormFieldMeta { label_en: "Structure kind", label_de: "Tragwerksart", unit: None, choices: Some(&[NormFieldChoice { value: "building", label_en: "Building", label_de: "Gebäude" }, NormFieldChoice { value: "bridge", label_en: "Bridge", label_de: "Brücke" }]) },
        "bridgeLane" => NormFieldMeta { label_en: "Notional lane number", label_de: "Nummer des ideellen Fahrstreifens", unit: None, choices: Some(&[NormFieldChoice { value: "1", label_en: "Notional lane 1", label_de: "Ideeller Fahrstreifen 1" }, NormFieldChoice { value: "2", label_en: "Notional lane 2", label_de: "Ideeller Fahrstreifen 2" }, NormFieldChoice { value: "3", label_en: "Notional lane 3+", label_de: "Ideeller Fahrstreifen 3+" }]) },
        "bridgeSpan" => NormFieldMeta { label_en: "Bridge span", label_de: "Brückenstützweite", unit: Some("m"), choices: None },
        "bridgeLaneWidth" => NormFieldMeta { label_en: "Notional lane width", label_de: "Ideelle Fahrstreifenbreite", unit: Some("m"), choices: None },
        "assumedBridgeTandem" => NormFieldMeta { label_en: "Assumed LM1 tandem axle", label_de: "Angenommenes LM1-Tandem", unit: Some("N"), choices: None },
        "assumedBridgeUdl" => NormFieldMeta { label_en: "Assumed LM1 UDL", label_de: "Angenommene LM1-Gleichlast", unit: Some("Pa"), choices: None },
        "assumedBridgeLm2" => NormFieldMeta { label_en: "Assumed LM2 axle", label_de: "Angenommenes LM2", unit: Some("N"), choices: None },
        "assumedBridgeFootway" => NormFieldMeta { label_en: "Assumed footway load", label_de: "Angenommene Gehweglast", unit: Some("Pa"), choices: None },
        "craneClaimed" => NormFieldMeta { label_en: "Crane runway claimed", label_de: "Kranbahn beansprucht", unit: None, choices: None },
        "craneClass" => NormFieldMeta { label_en: "Crane class", label_de: "Kranklasse", unit: None, choices: Some(&[NormFieldChoice { value: "HC1", label_en: "Hoist/crane class HC1", label_de: "Hub-/Kran-Klasse HC1" }, NormFieldChoice { value: "HC2", label_en: "Hoist/crane class HC2", label_de: "Hub-/Kran-Klasse HC2" }, NormFieldChoice { value: "HC3", label_en: "Hoist/crane class HC3", label_de: "Hub-/Kran-Klasse HC3" }, NormFieldChoice { value: "HC4", label_en: "Hoist/crane class HC4", label_de: "Hub-/Kran-Klasse HC4" }]) },
        "hoistClass" => NormFieldMeta { label_en: "Hoist class", label_de: "Hubwerkklasse", unit: None, choices: Some(&[NormFieldChoice { value: "HC1", label_en: "Hoist/crane class HC1", label_de: "Hub-/Kran-Klasse HC1" }, NormFieldChoice { value: "HC2", label_en: "Hoist/crane class HC2", label_de: "Hub-/Kran-Klasse HC2" }, NormFieldChoice { value: "HC3", label_en: "Hoist/crane class HC3", label_de: "Hub-/Kran-Klasse HC3" }, NormFieldChoice { value: "HC4", label_en: "Hoist/crane class HC4", label_de: "Hub-/Kran-Klasse HC4" }]) },
        "hoistingSpeed" => NormFieldMeta { label_en: "Hoisting speed", label_de: "Hubgeschwindigkeit", unit: Some("m/s"), choices: None },
        "assumedCraneWheel" => NormFieldMeta { label_en: "Assumed crane wheel load", label_de: "Angenommene Kranradlast", unit: Some("N"), choices: None },
        "assumedCraneHorizontal" => NormFieldMeta { label_en: "Assumed crane horizontal force", label_de: "Angenommene Kranhorizontalkraft", unit: Some("N"), choices: None },
        "siloClaimed" => NormFieldMeta { label_en: "Silo/tank claimed", label_de: "Silo/Tank beansprucht", unit: None, choices: None },
        "siloKind" => NormFieldMeta { label_en: "Silo or tank", label_de: "Silo oder Tank", unit: None, choices: Some(&[NormFieldChoice { value: "silo", label_en: "Silo (bulk solid)", label_de: "Silo (Schüttgut)" }, NormFieldChoice { value: "tank", label_en: "Tank (liquid)", label_de: "Tank (Flüssigkeit)" }]) },
        "siloBulkDensity" => NormFieldMeta { label_en: "Bulk unit weight γ", label_de: "Wichte γ", unit: Some("N/m³"), choices: None },
        "siloHeight" => NormFieldMeta { label_en: "Fill height", label_de: "Füllhöhe", unit: Some("m"), choices: None },
        "siloHydraulicRadius" => NormFieldMeta { label_en: "Hydraulic radius a", label_de: "Hydraulischer Radius a", unit: Some("m"), choices: None },
        "siloMu" => NormFieldMeta { label_en: "Wall friction μ", label_de: "Wandreibung μ", unit: None, choices: None },
        "siloK" => NormFieldMeta { label_en: "Lateral pressure ratio k", label_de: "Seitendruckverhältnis k", unit: None, choices: None },
        "assumedSiloPressure" => NormFieldMeta { label_en: "Assumed silo/tank pressure", label_de: "Angenommener Silo-/Tankdruck", unit: Some("Pa"), choices: None },
        "assumedSiloPatch" => NormFieldMeta { label_en: "Assumed patch pressure", label_de: "Angenommener Patchdruck", unit: Some("Pa"), choices: None },
        "assumedSiloWallFriction" => NormFieldMeta { label_en: "Assumed wall friction traction", label_de: "Angenommene Wandreibung", unit: Some("Pa"), choices: None },
        "floors" => NormFieldMeta { label_en: "Floor areas", label_de: "Geschossflächen", unit: None, choices: None },
        "selfWeightElements" => NormFieldMeta { label_en: "Self-weight elements", label_de: "Eigengewichtselemente", unit: None, choices: None },
        "roofs" => NormFieldMeta { label_en: "Roof areas", label_de: "Dachflächen", unit: None, choices: None },
        "windFaces" => NormFieldMeta { label_en: "Wind faces", label_de: "Windflächen", unit: None, choices: None },
        "accidentalCases" => NormFieldMeta { label_en: "Accidental cases", label_de: "Außergewöhnliche Fälle", unit: None, choices: None },
        "id" => NormFieldMeta { label_en: "Id", label_de: "Id", unit: None, choices: None },
        "category" => NormFieldMeta { label_en: "Imposed load category", label_de: "Nutzlastkategorie", unit: None, choices: Some(&[NormFieldChoice { value: "A", label_en: "A — residential areas", label_de: "A — Wohnflächen" }, NormFieldChoice { value: "A1", label_en: "A1 — rooms in residential buildings", label_de: "A1 — Räume in Wohngebäuden" }, NormFieldChoice { value: "A2", label_en: "A2 — attics in residential buildings", label_de: "A2 — Dachräume in Wohngebäuden" }, NormFieldChoice { value: "A3", label_en: "A3 — stairs / landings (residential)", label_de: "A3 — Treppen / Podeste (Wohnen)" }, NormFieldChoice { value: "B", label_en: "B — office areas", label_de: "B — Büroflächen" }, NormFieldChoice { value: "B1", label_en: "B1 — offices", label_de: "B1 — Büros" }, NormFieldChoice { value: "B2", label_en: "B2 — office corridors / meeting", label_de: "B2 — Büroflure / Besprechung" }, NormFieldChoice { value: "C", label_en: "C — congregation areas", label_de: "C — Versammlungsflächen" }, NormFieldChoice { value: "C1", label_en: "C1 — areas with tables", label_de: "C1 — Flächen mit Tischen" }, NormFieldChoice { value: "C2", label_en: "C2 — areas with fixed seats", label_de: "C2 — Flächen mit festen Sitzplätzen" }, NormFieldChoice { value: "C3", label_en: "C3 — areas without obstacles", label_de: "C3 — Flächen ohne Hindernisse" }, NormFieldChoice { value: "C4", label_en: "C4 — physical activities", label_de: "C4 — körperliche Aktivitäten" }, NormFieldChoice { value: "C5", label_en: "C5 — crowds susceptible to panic", label_de: "C5 — Menschenansammlungen / Panik" }, NormFieldChoice { value: "D", label_en: "D — shopping areas", label_de: "D — Verkaufsflächen" }, NormFieldChoice { value: "D1", label_en: "D1 — retail areas", label_de: "D1 — Einzelhandelsflächen" }, NormFieldChoice { value: "D2", label_en: "D2 — department stores", label_de: "D2 — Kaufhäuser" }, NormFieldChoice { value: "E", label_en: "E — storage / industrial", label_de: "E — Lager / Industrie" }, NormFieldChoice { value: "E1", label_en: "E1 — storage areas", label_de: "E1 — Lagerflächen" }, NormFieldChoice { value: "E2", label_en: "E2 — industrial use", label_de: "E2 — industrielle Nutzung" }, NormFieldChoice { value: "F", label_en: "F — vehicle traffic ≤ 30 kN", label_de: "F — Fahrzeugverkehr ≤ 30 kN" }, NormFieldChoice { value: "G", label_en: "G — vehicle traffic > 30 kN", label_de: "G — Fahrzeugverkehr > 30 kN" }, NormFieldChoice { value: "H", label_en: "H — roofs not accessible except for maintenance", label_de: "H — nicht begehbare Dächer" }, NormFieldChoice { value: "I", label_en: "I — forklift / industrial vehicles (DE NA)", label_de: "I — Gabelstapler / Industriefahrzeuge (DE NA)" }, NormFieldChoice { value: "J", label_en: "J — helicopter landing areas (DE NA)", label_de: "J — Hubschrauberlandeplätze (DE NA)" }, NormFieldChoice { value: "K", label_en: "K — hangars / aircraft areas (DE NA)", label_de: "K — Hangars / Flugzeugflächen (DE NA)" }]) },
        "area" => NormFieldMeta { label_en: "Area", label_de: "Fläche", unit: Some("m²"), choices: None },
        "assumedQk" => NormFieldMeta { label_en: "Assumed imposed qk", label_de: "Angenommene Nutzlast qk", unit: Some("Pa"), choices: None },
        "assumedQkConcentrated" => NormFieldMeta { label_en: "Assumed concentrated Qk", label_de: "Angenommene Einzellast Qk", unit: Some("N"), choices: None },
        "assumedPartitions" => NormFieldMeta { label_en: "Assumed partition load", label_de: "Angenommene Trennwandlast", unit: Some("Pa"), choices: None },
        "material" => NormFieldMeta { label_en: "Material (EN 1991-1-1 Annex A)", label_de: "Baustoff (EN 1991-1-1 Anhang A)", unit: None, choices: Some(&[NormFieldChoice { value: "reinforced_concrete", label_en: "Reinforced concrete (25 kN/m³)", label_de: "Stahlbeton (25 kN/m³)" }, NormFieldChoice { value: "concrete", label_en: "Concrete (25 kN/m³)", label_de: "Beton (25 kN/m³)" }, NormFieldChoice { value: "lightweight_concrete", label_en: "Lightweight concrete (18 kN/m³)", label_de: "Leichtbeton (18 kN/m³)" }, NormFieldChoice { value: "steel", label_en: "Steel (78.5 kN/m³)", label_de: "Stahl (78.5 kN/m³)" }, NormFieldChoice { value: "timber", label_en: "Timber / softwood (5 kN/m³)", label_de: "Holz / Nadelholz (5 kN/m³)" }, NormFieldChoice { value: "hardwood", label_en: "Hardwood (8 kN/m³)", label_de: "Laubholz (8 kN/m³)" }, NormFieldChoice { value: "glulam", label_en: "Glulam (4.2 kN/m³)", label_de: "Brettschichtholz (4.2 kN/m³)" }, NormFieldChoice { value: "masonry", label_en: "Masonry (18 kN/m³)", label_de: "Mauerwerk (18 kN/m³)" }, NormFieldChoice { value: "brick", label_en: "Brick (18 kN/m³)", label_de: "Ziegel (18 kN/m³)" }, NormFieldChoice { value: "natural_stone", label_en: "Natural stone (27 kN/m³)", label_de: "Naturstein (27 kN/m³)" }, NormFieldChoice { value: "aluminium", label_en: "Aluminium (27 kN/m³)", label_de: "Aluminium (27 kN/m³)" }, NormFieldChoice { value: "glass", label_en: "Glass (25 kN/m³)", label_de: "Glas (25 kN/m³)" }, NormFieldChoice { value: "water", label_en: "Water (10 kN/m³)", label_de: "Wasser (10 kN/m³)" }, NormFieldChoice { value: "sand", label_en: "Sand (18 kN/m³)", label_de: "Sand (18 kN/m³)" }, NormFieldChoice { value: "gravel", label_en: "Gravel (20 kN/m³)", label_de: "Kies (20 kN/m³)" }, NormFieldChoice { value: "asphalt", label_en: "Asphalt (23 kN/m³)", label_de: "Asphalt (23 kN/m³)" }, NormFieldChoice { value: "plaster", label_en: "Plaster (14 kN/m³)", label_de: "Putz (14 kN/m³)" }]) },
        "thickness" => NormFieldMeta { label_en: "Thickness", label_de: "Dicke", unit: Some("m"), choices: None },
        "assumedGk" => NormFieldMeta { label_en: "Assumed permanent gk", label_de: "Angenommenes Eigengewicht gk", unit: Some("Pa"), choices: None },
        "roofType" => NormFieldMeta { label_en: "Roof type", label_de: "Dachform", unit: None, choices: Some(&[NormFieldChoice { value: "monopitch", label_en: "Monopitch roof", label_de: "Pultdach" }, NormFieldChoice { value: "duopitch", label_en: "Duopitch roof", label_de: "Satteldach" }, NormFieldChoice { value: "flat", label_en: "Flat roof", label_de: "Flachdach" }, NormFieldChoice { value: "cylindrical", label_en: "Cylindrical roof", label_de: "Zylindrisches Dach" }]) },
        "pitchDeg" => NormFieldMeta { label_en: "Pitch", label_de: "Dachneigung", unit: Some("°"), choices: None },
        "cE" => NormFieldMeta { label_en: "Exposure coefficient cE", label_de: "Aussetzungskoeffizient cE", unit: None, choices: None },
        "cT" => NormFieldMeta { label_en: "Thermal coefficient cT", label_de: "Wärmekkoeffizient cT", unit: None, choices: None },
        "hasParapet" => NormFieldMeta { label_en: "Has parapet", label_de: "Mit Attika", unit: None, choices: None },
        "parapetHeight" => NormFieldMeta { label_en: "Parapet height", label_de: "Attikahöhe", unit: Some("m"), choices: None },
        "driftObstructionHeight" => NormFieldMeta { label_en: "Drift obstruction height", label_de: "Verwehungs-Hindernishöhe", unit: Some("m"), choices: None },
        "multiSpan" => NormFieldMeta { label_en: "Multi-span roof", label_de: "Mehrfeld-Dach", unit: None, choices: None },
        "assumedSk" => NormFieldMeta { label_en: "Assumed snow sk", label_de: "Angenommene Schneelast sk", unit: Some("Pa"), choices: None },
        "zone" => NormFieldMeta { label_en: "Wind zone letter", label_de: "Windzone (Buchstabe)", unit: None, choices: None },
        "z" => NormFieldMeta { label_en: "Reference height z", label_de: "Bezugshöhe z", unit: Some("m"), choices: None },
        "cPe10" => NormFieldMeta { label_en: "External pressure cpe,10", label_de: "Außendruckbeiwert cpe,10", unit: None, choices: None },
        "cPe1" => NormFieldMeta { label_en: "External pressure cpe,1", label_de: "Außendruckbeiwert cpe,1", unit: None, choices: None },
        "cPi" => NormFieldMeta { label_en: "Internal pressure cpi", label_de: "Innendruckbeiwert cpi", unit: None, choices: None },
        "cS" => NormFieldMeta { label_en: "Size factor cs", label_de: "Größenbeiwert cs", unit: None, choices: None },
        "cD" => NormFieldMeta { label_en: "Dynamic factor cd", label_de: "Dynamikbeiwert cd", unit: None, choices: None },
        "assumedWp" => NormFieldMeta { label_en: "Assumed wind pressure", label_de: "Angenommener Winddruck", unit: Some("Pa"), choices: None },
        "loadedArea" => NormFieldMeta { label_en: "Loaded area A for c_pe", label_de: "Belastete Fläche A für c_pe", unit: Some("m²"), choices: None },
        "impact" => NormFieldMeta { label_en: "Vehicle impact", label_de: "Fahrzeuganprall", unit: None, choices: None },
        "explosion" => NormFieldMeta { label_en: "Explosion", label_de: "Explosion", unit: None, choices: None },
        "vehicleMass" => NormFieldMeta { label_en: "Vehicle mass", label_de: "Fahrzeugmasse", unit: Some("kg"), choices: None },
        "vehicleSpeed" => NormFieldMeta { label_en: "Vehicle speed", label_de: "Fahrzeuggeschwindigkeit", unit: Some("m/s"), choices: None },
        "explosionMass" => NormFieldMeta { label_en: "Explosion charge mass", label_de: "Explosionsmasse", unit: Some("kg"), choices: None },
        "standoff" => NormFieldMeta { label_en: "Standoff distance", label_de: "Abstand", unit: Some("m"), choices: None },
        "assumedForce" => NormFieldMeta { label_en: "Assumed accidental force", label_de: "Angenommene außergewöhnliche Kraft", unit: Some("N"), choices: None },
        "assumedPressure" => NormFieldMeta { label_en: "Assumed accidental pressure", label_de: "Angenommener außergewöhnlicher Druck", unit: Some("Pa"), choices: None },

        "fire" => NormFieldMeta { label_en: "Fire (EN 1991-1-2)", label_de: "Brand (EN 1991-1-2)", unit: None, choices: None },
        "storeyCount" => NormFieldMeta { label_en: "Number of storeys", label_de: "Geschossanzahl", unit: None, choices: None },
        "tMax" => NormFieldMeta { label_en: "Maximum shade air temperature", label_de: "Maximale Lufttemperatur im Schatten", unit: Some("°C"), choices: None },
        "tMin" => NormFieldMeta { label_en: "Minimum shade air temperature", label_de: "Minimale Lufttemperatur im Schatten", unit: Some("°C"), choices: None },
        "t0" => NormFieldMeta { label_en: "Initial temperature T₀", label_de: "Ausgangstemperatur T₀", unit: Some("°C"), choices: None },
        "thermalElementType" => NormFieldMeta { label_en: "Thermal element type", label_de: "Temperatur-Bauteiltyp", unit: None, choices: Some(&[NormFieldChoice { value: "building", label_en: "Building element", label_de: "Gebäudebauteil" }, NormFieldChoice { value: "bridge1", label_en: "Bridge type 1", label_de: "Brückentyp 1" }, NormFieldChoice { value: "bridge2", label_en: "Bridge type 2", label_de: "Brückentyp 2" }, NormFieldChoice { value: "bridge3", label_en: "Bridge type 3", label_de: "Brückentyp 3" }]) },
        "thermalBridgeType" => NormFieldMeta { label_en: "Bridge type (1–3)", label_de: "Brückentyp (1–3)", unit: None, choices: None },
        "deltaTM" => NormFieldMeta { label_en: "Linear temperature difference ΔT_M", label_de: "Lineare Temperaturdifferenz ΔT_M", unit: Some("K"), choices: None },
        "fireMode" => NormFieldMeta { label_en: "Fire design mode", label_de: "Brandbemessungsmodus", unit: None, choices: Some(&[NormFieldChoice { value: "none", label_en: "None — no fire design", label_de: "Keine — keine Brandbemessung" }, NormFieldChoice { value: "nominal", label_en: "Nominal fire curves (§3.2)", label_de: "Nominelle Brandkurven (§3.2)" }, NormFieldChoice { value: "parametric", label_en: "Parametric fire (Annex A)", label_de: "Parametrischer Brand (Anhang A)" }]) },
        "fireCurve" => NormFieldMeta { label_en: "Fire curve", label_de: "Brandkurve", unit: None, choices: Some(&[NormFieldChoice { value: "standard", label_en: "ISO 834 standard", label_de: "ISO 834 Normbrand" }, NormFieldChoice { value: "external", label_en: "External fire", label_de: "Außenbrand" }, NormFieldChoice { value: "hydrocarbon", label_en: "Hydrocarbon", label_de: "Kohlenwasserstoff" }, NormFieldChoice { value: "parametric", label_en: "Parametric (Annex A)", label_de: "Parametrisch (Anhang A)" }]) },
        "fireDuration" => NormFieldMeta { label_en: "Fire duration", label_de: "Branddauer", unit: Some("s"), choices: None },
        "assumedGasTemperature" => NormFieldMeta { label_en: "Assumed gas temperature θ_g", label_de: "Angenommene Gastemperatur θ_g", unit: Some("K"), choices: None },
        "assumedHNet" => NormFieldMeta { label_en: "Assumed net heat flux", label_de: "Angenommener Nettowärmestrom", unit: Some("W/m²"), choices: None },
        "fireCompartmentArea" => NormFieldMeta { label_en: "Fire compartment floor area", label_de: "Brandabschnittsfläche", unit: Some("m²"), choices: None },
        "fireCompartmentHeight" => NormFieldMeta { label_en: "Fire compartment height", label_de: "Brandabschnittshöhe", unit: Some("m"), choices: None },
        "fireOpeningFactor" => NormFieldMeta { label_en: "Opening factor O", label_de: "Öffnungsfaktor O", unit: Some("m½"), choices: None },
        "fireThermalInertia" => NormFieldMeta { label_en: "Thermal inertia b", label_de: "Wärmeträgheit b", unit: Some("J/(m²s½K)"), choices: None },
        "fireOccupancy" => NormFieldMeta { label_en: "Fire occupancy class", label_de: "Brand-Nutzungsklasse", unit: None, choices: Some(&[NormFieldChoice { value: "office", label_en: "Office", label_de: "Büro" }, NormFieldChoice { value: "dwelling", label_en: "Dwelling", label_de: "Wohnen" }, NormFieldChoice { value: "hospital", label_en: "Hospital / school", label_de: "Krankenhaus / Schule" }, NormFieldChoice { value: "shopping", label_en: "Shopping", label_de: "Einkauf" }, NormFieldChoice { value: "industrial", label_en: "Industrial", label_de: "Industrie" }, NormFieldChoice { value: "warehouse", label_en: "Warehouse", label_de: "Lager" }, NormFieldChoice { value: "parking", label_en: "Parking", label_de: "Parken" }]) },
        "fireLoadDensityQf" => NormFieldMeta { label_en: "Characteristic fire load density q_f,k", label_de: "Charakteristische Brandlastdichte q_f,k", unit: Some("J/m²"), choices: None },
        "assumedQfD" => NormFieldMeta { label_en: "Assumed design fire load q_f,d", label_de: "Angenommene Bemessungsbrandlastdichte q_f,d", unit: Some("J/m²"), choices: None },
        "assumedBridgeLm3" => NormFieldMeta { label_en: "Assumed LM3 special vehicle axle", label_de: "Angenommene LM3-Sonderfahrzeugachse", unit: Some("N"), choices: None },
        "assumedBridgeLm4" => NormFieldMeta { label_en: "Assumed LM4 crowd load", label_de: "Angenommene LM4-Menschenlast", unit: Some("Pa"), choices: None },
        "bridgeLoadGroup" => NormFieldMeta { label_en: "Bridge load group", label_de: "Brücken-Lastgruppe", unit: None, choices: Some(&[NormFieldChoice { value: "gr1a", label_en: "gr1a — LM1 + footway", label_de: "gr1a — LM1 + Gehweg" }, NormFieldChoice { value: "gr1b", label_en: "gr1b — LM2", label_de: "gr1b — LM2" }, NormFieldChoice { value: "gr3", label_en: "gr3 — pedestrian", label_de: "gr3 — Fußgänger" }, NormFieldChoice { value: "gr4", label_en: "gr4 — LM4 crowd", label_de: "gr4 — LM4 Menschenmenge" }, NormFieldChoice { value: "gr5", label_en: "gr5 — LM3 special vehicles", label_de: "gr5 — LM3 Sonderfahrzeuge" }]) },

        _ => return None,
    })
}


/// 🧭 Map grouped inputs-editor paths back onto the flat SI snapshot (UiFixedList cap = 32).
pub fn flatten_editor_path(path: &str) -> String {
    for prefix in ["site.", "building.", "thermal.", "execution.", "bridge.", "crane.", "silo."] {
        if let Some(rest) = path.strip_prefix(prefix) {
            return rest.to_string();
        }
    }
    path.to_string()
}
