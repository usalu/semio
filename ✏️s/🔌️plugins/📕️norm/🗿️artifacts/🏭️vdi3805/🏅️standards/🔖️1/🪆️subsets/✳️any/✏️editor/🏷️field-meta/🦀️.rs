//! 🏷️ VDI 3805 NormFieldMeta lookup — SI display units + en/de labels for the structured inputs editor.

use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

const PN: &[NormFieldChoice] = &[
    NormFieldChoice { value: "PN6", label_en: "PN 6", label_de: "PN 6" },
    NormFieldChoice { value: "PN10", label_en: "PN 10", label_de: "PN 10" },
    NormFieldChoice { value: "PN16", label_en: "PN 16", label_de: "PN 16" },
    NormFieldChoice { value: "PN25", label_en: "PN 25", label_de: "PN 25" },
];
const FUEL: &[NormFieldChoice] = &[
    NormFieldChoice { value: "gas", label_en: "Natural gas", label_de: "Erdgas" },
    NormFieldChoice { value: "oil", label_en: "Fuel oil", label_de: "Heizöl" },
    NormFieldChoice { value: "electric", label_en: "Electric", label_de: "Elektrisch" },
    NormFieldChoice { value: "biomass", label_en: "Biomass", label_de: "Biomasse" },
    NormFieldChoice { value: "district", label_en: "District heat", label_de: "Fernwärme" },
];
const CONNECTION: &[NormFieldChoice] = &[
    NormFieldChoice { value: "flange", label_en: "Flanged", label_de: "Flansch" },
    NormFieldChoice { value: "thread", label_en: "Threaded", label_de: "Gewinde" },
    NormFieldChoice { value: "weld", label_en: "Welded", label_de: "Schweißanschluss" },
    NormFieldChoice { value: "press", label_en: "Press fit", label_de: "Pressverbindung" },
];
const FILTER: &[NormFieldChoice] = &[
    NormFieldChoice { value: "G4", label_en: "Coarse G4", label_de: "Grob G4" },
    NormFieldChoice { value: "M5", label_en: "Medium M5", label_de: "Mittel M5" },
    NormFieldChoice { value: "M6", label_en: "Medium M6", label_de: "Mittel M6" },
    NormFieldChoice { value: "F7", label_en: "Fine F7", label_de: "Fein F7" },
    NormFieldChoice { value: "F8", label_en: "Fine F8", label_de: "Fein F8" },
    NormFieldChoice { value: "F9", label_en: "Fine F9", label_de: "Fein F9" },
    NormFieldChoice { value: "ePM1", label_en: "ePM1", label_de: "ePM1" },
    NormFieldChoice { value: "ePM2_5", label_en: "ePM2.5", label_de: "ePM2,5" },
    NormFieldChoice { value: "ePM10", label_en: "ePM10", label_de: "ePM10" },
];
const TYPE_CODE: &[NormFieldChoice] = &[
    NormFieldChoice { value: "STD", label_en: "Standard", label_de: "Standard" },
    NormFieldChoice { value: "STD-A", label_en: "Standard A", label_de: "Standard A" },
    NormFieldChoice { value: "STD-B", label_en: "Standard B", label_de: "Standard B" },
    NormFieldChoice { value: "TYPE-A", label_en: "Type A", label_de: "Typ A" },
    NormFieldChoice { value: "TYPE-B", label_en: "Type B", label_de: "Typ B" },
    NormFieldChoice { value: "TYPE-C", label_en: "Type C", label_de: "Typ C" },
    NormFieldChoice { value: "DEFAULT", label_en: "Default", label_de: "Default" },
];
const ATTR_KIND: &[NormFieldChoice] = &[
    NormFieldChoice { value: "valveHeating", label_en: "Heating control valve", label_de: "Heizungsstellventil" },
    NormFieldChoice { value: "radiator", label_en: "Radiator", label_de: "Heizkörper" },
    NormFieldChoice { value: "pumpHeating", label_en: "Heating pump", label_de: "Heizungspumpe" },
    NormFieldChoice { value: "heatGenerator", label_en: "Heat generator", label_de: "Wärmeerzeuger" },
    NormFieldChoice { value: "generic", label_en: "Generic product", label_de: "Generisches Produkt" },
];

const fn m(label_en: &'static str, label_de: &'static str, unit: Option<&'static str>) -> NormFieldMeta {
    NormFieldMeta { label_en, label_de, unit, choices: None }
}

const TABLE: &[(&str, NormFieldMeta)] = &[
    ("catalog", m("Catalogue", "Katalog", None)),
    ("catalog.file", m("Catalogue file header", "Katalogdateikopf", None)),
    ("catalog.file.headerVersion", m("Header version", "Kopfversion", None)),
    ("catalog.file.manufacturer", m("Manufacturer code", "Herstellercode", None)),
    ("catalog.file.buildingSystemNumber", m("Building system number", "Gewerknummer", None)),
    ("catalog.file.buildingSystemNumber.systemCode", m("System code", "Systemcode", None)),
    ("catalog.file.buildingSystemNumber.subsystem", m("Subsystem", "Teilsystem", None)),
    ("catalog.file.buildingSystemNumber.sequence", m("Sequence", "Laufnummer", None)),
    ("catalog.file.created", m("Created date", "Erstellungsdatum", None)),
    ("catalog.file.charset", m("Character set", "Zeichensatz", None)),
    ("catalog.file.recordCount", m("Record count", "Satzanzahl", None)),
    ("catalog.file.extensions", m("Header extensions", "Kopferweiterungen", None)),
    ("catalog.file.extensions.fields", m("Extension fields", "Erweiterungsfelder", None)),
    ("catalog.extensions", m("Catalogue extensions", "Katalogerweiterungen", None)),
    ("catalog.extensions.fields", m("Catalogue extension fields", "Katalog-Erweiterungsfelder", None)),
    ("catalog.products[]", m("Product", "Produkt", None)),
    ("catalog.products[].id", m("Product id", "Produkt-ID", None)),
    ("catalog.products[].identity", m("Product identity", "Produktidentität", None)),
    ("catalog.products[].identity.articleNumber", m("Article number", "Artikelnummer", None)),
    ("catalog.products[].identity.manufacturerCode", m("Manufacturer code", "Herstellercode", None)),
    ("catalog.products[].identity.productGroup", m("Product group", "Produktgruppe", None)),
    ("catalog.products[].title[]", m("Title", "Titel", None)),
    ("catalog.products[].title[].locale", m("Title locale", "Titel-Locale", None)),
    ("catalog.products[].title[].text", m("Title text", "Titeltext", None)),
    ("catalog.products[].sheet", m("Sheet (Blatt)", "Blatt", None)),
    ("catalog.products[].records[]", m("Native record", "Nativsatz", None)),
    ("catalog.products[].records[].family", m("Record family", "Satzfamilie", None)),
    ("catalog.products[].records[].fields[]", m("Record field", "Satzfeld", None)),
    ("catalog.products[].records[].extensions", m("Record extensions", "Satzerweiterungen", None)),
    ("catalog.products[].configuration", m("Configuration", "Konfiguration", None)),
    ("catalog.products[].configuration.id", m("Configuration id", "Konfigurations-ID", None)),
    ("catalog.products[].configuration.geometryRef", m("Geometry reference", "Geometriereferenz", None)),
    ("catalog.products[].configuration.functionRefs[]", m("Curve reference", "Kennlinienreferenz", None)),
    ("catalog.products[].configuration.attributes", NormFieldMeta { label_en: "Sheet attributes", label_de: "Blattattribute", unit: None, choices: Some(ATTR_KIND) }),
    ("catalog.products[].configuration.attributes.kind", NormFieldMeta { label_en: "Attribute kind", label_de: "Attributart", unit: None, choices: Some(ATTR_KIND) }),
    ("catalog.products[].configuration.attributes.dn", m("Nominal diameter DN", "Nennweite DN", None)),
    ("catalog.products[].configuration.attributes.kvsM3S", m("Flow coefficient kvs", "Durchflusskoeffizient kvs", Some("m³/s"))),
    ("catalog.products[].configuration.attributes.pressureClass", NormFieldMeta { label_en: "Pressure class", label_de: "Druckstufe", unit: None, choices: Some(PN) }),
    ("catalog.products[].configuration.attributes.connectionType", NormFieldMeta { label_en: "Connection type", label_de: "Anschlussart", unit: None, choices: Some(CONNECTION) }),
    ("catalog.products[].configuration.attributes.authorityMin", m("Authority min", "Autorität min", None)),
    ("catalog.products[].configuration.attributes.authorityMax", m("Authority max", "Autorität max", None)),
    ("catalog.products[].configuration.attributes.standardOutputW", m("Standard heat output Φ", "Normwärmeleistung Φ", Some("W"))),
    ("catalog.products[].configuration.attributes.heatExponentN", m("Heat exponent n", "Heizexponent n", None)),
    ("catalog.products[].configuration.attributes.lengthM", m("Radiator length", "Heizkörperlänge", Some("m"))),
    ("catalog.products[].configuration.attributes.heightM", m("Radiator height", "Heizkörperhöhe", Some("m"))),
    ("catalog.products[].configuration.attributes.depthM", m("Radiator depth", "Heizkörpertiefe", Some("m"))),
    ("catalog.products[].configuration.attributes.nominalFlowM3S", m("Nominal flow", "Nennvolumenstrom", Some("m³/s"))),
    ("catalog.products[].configuration.attributes.dnSuction", m("Suction DN", "Saugseitige DN", None)),
    ("catalog.products[].configuration.attributes.dnDischarge", m("Discharge DN", "Druckseitige DN", None)),
    ("catalog.products[].configuration.attributes.nominalHeadM", m("Nominal head", "Nennförderhöhe", Some("m"))),
    ("catalog.products[].configuration.attributes.motorPowerW", m("Motor power", "Motorleistung", Some("W"))),
    ("catalog.products[].configuration.attributes.hydraulicEfficiency", m("Hydraulic efficiency", "Hydraulischer Wirkungsgrad", None)),
    ("catalog.products[].configuration.attributes.qhCurveRef", m("Q-H curve reference", "Q-H-Kennlinienreferenz", None)),
    ("catalog.products[].configuration.attributes.nominalHeatOutputW", m("Nominal heat output", "Nennwärmeleistung", Some("W"))),
    ("catalog.products[].configuration.attributes.fuelType", NormFieldMeta { label_en: "Fuel / energy type", label_de: "Brennstoff-/Energieträger", unit: None, choices: Some(FUEL) }),
    ("catalog.products[].configuration.attributes.flowTempMaxC", m("Maximum flow temperature", "Maximale Vorlauftemperatur", Some("°C"))),
    ("catalog.products[].configuration.attributes.returnTempMinC", m("Minimum return temperature", "Minimale Rücklauftemperatur", Some("°C"))),
    ("catalog.products[].configuration.attributes.entries[]", m("Generic attribute", "Generisches Attribut", None)),
    ("catalog.products[].configuration.attributes.entries[].key", m("Attribute key", "Attributschlüssel", None)),
    ("catalog.products[].configuration.attributes.entries[].value", m("Attribute value", "Attributwert", None)),
    ("catalog.products[].configuration.attributes.entries[].unit", m("Attribute unit", "Attributeinheit", None)),
    ("catalog.products[].configuration.attributes.entries[].key", m("Attribute key", "Attributschlüssel", None)),
    ("catalog.products[].configuration.attributes.filter_class", NormFieldMeta { label_en: "Filter class", label_de: "Filterklasse", unit: None, choices: Some(FILTER) }),
    ("catalog.products[].configuration.attributes.type_code", NormFieldMeta { label_en: "Type code", label_de: "Typcode", unit: None, choices: Some(TYPE_CODE) }),
    ("catalog.products[].configuration.attributes.entries[id=filter_class].value", NormFieldMeta { label_en: "Filter class", label_de: "Filterklasse", unit: None, choices: Some(FILTER) }),
    ("catalog.products[].configuration.attributes.entries[id=type_code].value", NormFieldMeta { label_en: "Type code", label_de: "Typcode", unit: None, choices: Some(TYPE_CODE) }),
    ("catalog.products[].accessories[]", m("Accessory link", "Zubehörverknüpfung", None)),
    ("catalog.products[].accessories[].accessoryId", m("Accessory product id", "Zubehör-Produkt-ID", None)),
    ("catalog.products[].accessories[].required", m("Accessory required", "Zubehör erforderlich", None)),
    ("catalog.products[].accessories[].quantity", m("Accessory quantity", "Zubehöranzahl", None)),
    ("catalog.products[].components[]", m("Component link", "Komponentenverknüpfung", None)),
    ("catalog.products[].components[].componentId", m("Component product id", "Komponenten-Produkt-ID", None)),
    ("catalog.products[].components[].quantity", m("Component quantity", "Komponentenanzahl", None)),
    ("catalog.products[].extensions", m("Product extensions", "Produkterweiterungen", None)),
    ("geometry", m("Geometry", "Geometrie", None)),
    ("geometry[].id", m("Geometry id", "Geometrie-ID", None)),
    ("geometry[].bbox", m("Bounding box", "Begrenzungsrahmen", None)),
    ("geometry[].bbox.minX", m("BBox min X", "BBox min X", Some("m"))),
    ("geometry[].bbox.minY", m("BBox min Y", "BBox min Y", Some("m"))),
    ("geometry[].bbox.minZ", m("BBox min Z", "BBox min Z", Some("m"))),
    ("geometry[].bbox.maxX", m("BBox max X", "BBox max X", Some("m"))),
    ("geometry[].bbox.maxY", m("BBox max Y", "BBox max Y", Some("m"))),
    ("geometry[].bbox.maxZ", m("BBox max Z", "BBox max Z", Some("m"))),
    ("geometry[].connections[]", m("Geometry connection", "Geometrieanschluss", None)),
    ("geometry[].connections[].id", m("Connection id", "Anschluss-ID", None)),
    ("geometry[].connections[].x", m("Connection X", "Anschluss X", Some("m"))),
    ("geometry[].connections[].y", m("Connection Y", "Anschluss Y", Some("m"))),
    ("geometry[].connections[].z", m("Connection Z", "Anschluss Z", Some("m"))),
    ("geometry[].connections[].nx", m("Connection normal X", "Anschlussnormale X", None)),
    ("geometry[].connections[].ny", m("Connection normal Y", "Anschlussnormale Y", None)),
    ("geometry[].connections[].nz", m("Connection normal Z", "Anschlussnormale Z", None)),
    ("geometry[].parameters", m("Geometry parameters", "Geometrieparameter", None)),
    ("curves", m("Characteristic curves", "Kennlinien", None)),
    ("curves[].id", m("Curve id", "Kennlinien-ID", None)),
    ("curves[].points[]", m("Curve point", "Kennlinienpunkt", None)),
    ("curves[].points[].x", m("Curve point x", "Kennlinienpunkt x", None)),
    ("curves[].points[].y", m("Curve point y", "Kennlinienpunkt y", None)),
    ("editionProfile", m("Edition profile", "Ausgabenprofil", None)),
    ("correctionAsOf", m("Correction as of", "Korrekturstand", None)),
    ("correctionAsOf.year", m("Correction year", "Korrekturjahr", None)),
    ("correctionAsOf.month", m("Correction month", "Korrekturmonat", None)),
    ("strictMode", m("Strict mode", "Strict Mode", None)),
    ("index", m("Catalogue index", "Katalogindex", None)),
    ("index.entries[]", m("Index entry", "Indexeintrag", None)),
    ("index.entries[].productId", m("Index product id", "Index-Produkt-ID", None)),
    ("index.entries[].sheet", m("Index sheet", "Index-Blatt", None)),
    ("index.entries[].tags[]", m("Index tag", "Index-Tag", None)),
    ("index.entries[].dn", m("Index DN", "Index-DN", None)),
];

/// 🏷️ Exact-path / `[]`-wildcard metadata for manufacturer-catalogue fields.
pub fn vdi3805_field_meta(path: &str) -> Option<NormFieldMeta> {
    if let Some(meta) = lookup_norm_field_meta(TABLE, path) {
        return Some(meta);
    }
    if let Some(rest) = path.strip_prefix("curves.") {
        if let Some(points_at) = rest.find(".points") {
            let suffix = &rest[points_at + 1..];
            let wild = format!("curves[].{suffix}");
            if let Some(meta) = lookup_norm_field_meta(TABLE, &wild) {
                return Some(meta);
            }
            if wild.contains("points[].x") || wild.ends_with(".x") {
                return Some(m("Curve point x", "Kennlinienpunkt x", None));
            }
            if wild.contains("points[].y") || wild.ends_with(".y") {
                return Some(m("Curve point y", "Kennlinienpunkt y", None));
            }
            if wild.starts_with("points") {
                return Some(m("Curve points", "Kennlinienpunkte", None));
            }
        }
        return Some(m("Characteristic curve", "Kennlinie", None));
    }
    if path.starts_with("geometry.") {
        if path.contains(".connections") {
            return Some(NormFieldMeta { label_en: "Geometry connection", label_de: "Geometrieanschluss", unit: None, choices: None });
        }
        return Some(NormFieldMeta { label_en: "Geometry", label_de: "Geometrie", unit: None, choices: None });
    }
    if path.starts_with("editionProfile.") {
        return Some(NormFieldMeta { label_en: "Edition profile choice", label_de: "Ausgabenprofilwahl", unit: None, choices: None });
    }
    None
}
