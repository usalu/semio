//! 🏷️ ISO 16757 NormFieldMeta — en/de labels, SI display units, `[]` wildcards, localized choices.

use crate::app_surface::{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta};

const EXCHANGE_PROCESS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "createFromDictionary", label_en: "Create from dictionary", label_de: "Aus Wörterbuch erstellen" },
    NormFieldChoice { value: "provideCatalogue", label_en: "Provide catalogue", label_de: "Katalog bereitstellen" },
    NormFieldChoice { value: "determineProduct", label_en: "Determine product", label_de: "Produkt bestimmen" },
    NormFieldChoice { value: "integrateIntoSystem", label_en: "Integrate into system", label_de: "In System integrieren" },
    NormFieldChoice { value: "exchangeSystemModel", label_en: "Exchange system model", label_de: "Systemmodell austauschen" },
];

const EDITION_PROFILE: &[NormFieldChoice] = &[
    NormFieldChoice { value: "fullPublished", label_en: "Full published edition", label_de: "Vollständig veröffentlichte Ausgabe" },
    NormFieldChoice { value: "part1_2015", label_en: "Part 1 (2015)", label_de: "Teil 1 (2015)" },
    NormFieldChoice { value: "part2_2016", label_en: "Part 2 (2016)", label_de: "Teil 2 (2016)" },
    NormFieldChoice { value: "part4_2025", label_en: "Part 4 (2025)", label_de: "Teil 4 (2025)" },
    NormFieldChoice { value: "part5_2025", label_en: "Part 5 (2025)", label_de: "Teil 5 (2025)" },
];

const OPERATORS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "equal", label_en: "Equal", label_de: "Gleich" },
    NormFieldChoice { value: "notEqual", label_en: "Not equal", label_de: "Ungleich" },
    NormFieldChoice { value: "lessThan", label_en: "Less than", label_de: "Kleiner als" },
    NormFieldChoice { value: "greaterThan", label_en: "Greater than", label_de: "Größer als" },
    NormFieldChoice { value: "inRange", label_en: "In range", label_de: "Im Bereich" },
];

const SPACE_KINDS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "overall", label_en: "Overall space", label_de: "Gesamtraum" },
    NormFieldChoice { value: "operation", label_en: "Operation space", label_de: "Betriebsraum" },
    NormFieldChoice { value: "access", label_en: "Access space", label_de: "Zugangsraum" },
    NormFieldChoice { value: "placementTransportation", label_en: "Placement / transportation", label_de: "Platzierung / Transport" },
    NormFieldChoice { value: "installation", label_en: "Installation space", label_de: "Installationsraum" },
];

const SUBJECT_KINDS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "productGroup", label_en: "Product group", label_de: "Produktgruppe" },
    NormFieldChoice { value: "productClass", label_en: "Product class", label_de: "Produktklasse" },
    NormFieldChoice { value: "productSpecialization", label_en: "Product specialization", label_de: "Produktspezialisierung" },
    NormFieldChoice { value: "catalogueMetadata", label_en: "Catalogue metadata", label_de: "Katalog-Metadaten" },
    NormFieldChoice { value: "manufacturerMetadata", label_en: "Manufacturer metadata", label_de: "Hersteller-Metadaten" },
    NormFieldChoice { value: "propertyBlock", label_en: "Property block", label_de: "Eigenschaftsblock" },
    NormFieldChoice { value: "port", label_en: "Port", label_de: "Anschluss" },
    NormFieldChoice { value: "inlet", label_en: "Inlet", label_de: "Einlass" },
    NormFieldChoice { value: "outlet", label_en: "Outlet", label_de: "Auslass" },
    NormFieldChoice { value: "inOutlet", label_en: "Inlet/outlet", label_de: "Ein-/Auslass" },
];

const RELATIONSHIP_KINDS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "isSubtypeOf", label_en: "Is subtype of", label_de: "Ist Untertyp von" },
    NormFieldChoice { value: "hasPart", label_en: "Has part", label_de: "Hat Teil" },
    NormFieldChoice { value: "hasBlock", label_en: "Has block", label_de: "Hat Block" },
    NormFieldChoice { value: "isDependentOn", label_en: "Is dependent on", label_de: "Hängt ab von" },
    NormFieldChoice { value: "isSubkindOf", label_en: "Is subkind of", label_de: "Ist Unterart von" },
];

const PROPERTY_KINDS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "static", label_en: "Static", label_de: "Statisch" },
    NormFieldChoice { value: "dynamic", label_en: "Dynamic", label_de: "Dynamisch" },
    NormFieldChoice { value: "selection", label_en: "Selection", label_de: "Auswahl" },
    NormFieldChoice { value: "external", label_en: "External", label_de: "Extern" },
];


const TABLE: &[(&str, NormFieldMeta)] = &[
    ("catalogue", NormFieldMeta { label_en: "Catalogue", label_de: "Katalog", unit: None, choices: None }),
    ("dictionary", NormFieldMeta { label_en: "Dictionary", label_de: "Wörterbuch", unit: None, choices: None }),
    ("geometry", NormFieldMeta { label_en: "Geometry catalogue", label_de: "Geometriekatalog", unit: None, choices: None }),
    ("selection", NormFieldMeta { label_en: "Selection request", label_de: "Auswahlauftrag", unit: None, choices: None }),
    ("exchangeProcess", NormFieldMeta { label_en: "Exchange process", label_de: "Austauschprozess", unit: None, choices: Some(EXCHANGE_PROCESS) }),
    ("partNumberRule", NormFieldMeta { label_en: "Part-number rule", label_de: "Teilenummernregel", unit: None, choices: None }),
    ("partNumberRule.source", NormFieldMeta { label_en: "Part-number rule source", label_de: "Teilenummernregel-Quelltext", unit: None, choices: None }),
    ("partNumberRule.kind", NormFieldMeta { label_en: "Part-number rule kind", label_de: "Teilenummernregel-Art", unit: None, choices: None }),
    ("partNumberInputs", NormFieldMeta { label_en: "Part-number inputs", label_de: "Teilenummern-Eingaben", unit: None, choices: None }),
    ("scriptLimits", NormFieldMeta { label_en: "Script limits", label_de: "Skriptgrenzen", unit: None, choices: None }),
    ("scriptLimits.maxSteps", NormFieldMeta { label_en: "Max steps", label_de: "Max. Schritte", unit: None, choices: None }),
    ("scriptLimits.maxRecursion", NormFieldMeta { label_en: "Max recursion", label_de: "Max. Rekursion", unit: None, choices: None }),
    ("scriptLimits.timeoutMs", NormFieldMeta { label_en: "Timeout", label_de: "Zeitlimit", unit: Some("ms"), choices: None }),
    ("catalogue.metadata", NormFieldMeta { label_en: "Metadata", label_de: "Metadaten", unit: None, choices: None }),
    ("catalogue.metadata.editionProfile", NormFieldMeta { label_en: "Edition profile", label_de: "Editionsprofil", unit: None, choices: Some(EDITION_PROFILE) }),
    ("catalogue.manufacturer", NormFieldMeta { label_en: "Manufacturer", label_de: "Hersteller", unit: None, choices: None }),
    ("catalogue.productGroups", NormFieldMeta { label_en: "Product groups", label_de: "Produktgruppen", unit: None, choices: None }),
    ("catalogue.productGroups[]", NormFieldMeta { label_en: "Product group", label_de: "Produktgruppe", unit: None, choices: None }),
    ("catalogue.productGroups[].id", NormFieldMeta { label_en: "Group id", label_de: "Gruppen-Id", unit: None, choices: None }),
    ("catalogue.productClasses", NormFieldMeta { label_en: "Product classes", label_de: "Produktklassen", unit: None, choices: None }),
    ("catalogue.productClasses[]", NormFieldMeta { label_en: "Product class", label_de: "Produktklasse", unit: None, choices: None }),
    ("catalogue.productClasses[].id", NormFieldMeta { label_en: "Class id", label_de: "Klassen-Id", unit: None, choices: None }),
    ("catalogue.productClasses[].groupId", NormFieldMeta { label_en: "Group id", label_de: "Gruppen-Id", unit: None, choices: None }),
    ("catalogue.productSeries", NormFieldMeta { label_en: "Product series", label_de: "Produktserien", unit: None, choices: None }),
    ("catalogue.productSeries[]", NormFieldMeta { label_en: "Product series entry", label_de: "Produktserie", unit: None, choices: None }),
    ("catalogue.productSeries[].id", NormFieldMeta { label_en: "Series id", label_de: "Serien-Id", unit: None, choices: None }),
    ("catalogue.products", NormFieldMeta { label_en: "Products", label_de: "Produkte", unit: None, choices: None }),
    ("catalogue.products[]", NormFieldMeta { label_en: "Product", label_de: "Produkt", unit: None, choices: None }),
    ("catalogue.products[].id", NormFieldMeta { label_en: "Product id", label_de: "Produkt-Id", unit: None, choices: None }),
    ("catalogue.products[].seriesId", NormFieldMeta { label_en: "Series id", label_de: "Serien-Id", unit: None, choices: None }),
    ("catalogue.products[].variants", NormFieldMeta { label_en: "Variants", label_de: "Varianten", unit: None, choices: None }),
    ("catalogue.products[].variants[]", NormFieldMeta { label_en: "Variant", label_de: "Variante", unit: None, choices: None }),
    ("catalogue.products[].variants[].id", NormFieldMeta { label_en: "Variant id", label_de: "Varianten-Id", unit: None, choices: None }),
    ("catalogue.products[].variants[].geometryId", NormFieldMeta { label_en: "Geometry id", label_de: "Geometrie-Id", unit: None, choices: None }),
    ("catalogue.products[].variants[].parameterValues", NormFieldMeta { label_en: "Parameter values", label_de: "Parameterwerte", unit: None, choices: None }),
    ("catalogue.products[].variants[].parameterValues.*", NormFieldMeta { label_en: "Parameter value", label_de: "Parameterwert", unit: None, choices: None }),
    ("catalogue.propertyDefinitions", NormFieldMeta { label_en: "Property definitions", label_de: "Eigenschaftsdefinitionen", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[]", NormFieldMeta { label_en: "Property definition", label_de: "Eigenschaftsdefinition", unit: None, choices: None }),
    ("catalogue.productIndexes", NormFieldMeta { label_en: "Product indexes", label_de: "Produktindizes", unit: None, choices: None }),
    ("catalogue.productIndexes[]", NormFieldMeta { label_en: "Product index", label_de: "Produktindex", unit: None, choices: None }),
    ("catalogue.descriptiveObjects", NormFieldMeta { label_en: "Descriptive media", label_de: "Beschreibende Medien", unit: None, choices: None }),
    ("catalogue.descriptiveObjects[]", NormFieldMeta { label_en: "Descriptive object", label_de: "Beschreibendes Objekt", unit: None, choices: None }),
    ("dictionary.subjects", NormFieldMeta { label_en: "Subjects", label_de: "Subjekte", unit: None, choices: None }),
    ("dictionary.subjects[]", NormFieldMeta { label_en: "Subject", label_de: "Subjekt", unit: None, choices: None }),
    ("dictionary.relationships", NormFieldMeta { label_en: "Relationships", label_de: "Beziehungen", unit: None, choices: None }),
    ("dictionary.relationships[]", NormFieldMeta { label_en: "Relationship", label_de: "Beziehung", unit: None, choices: None }),
    ("dictionary.controlledLists", NormFieldMeta { label_en: "Controlled lists", label_de: "Kontrollierte Listen", unit: None, choices: None }),
    ("dictionary.controlledLists[]", NormFieldMeta { label_en: "Controlled list", label_de: "Kontrollierte Liste", unit: None, choices: None }),
    ("geometry.objects", NormFieldMeta { label_en: "Geometry objects", label_de: "Geometrieobjekte", unit: None, choices: None }),
    ("geometry.objects.*", NormFieldMeta { label_en: "Geometry object", label_de: "Geometrieobjekt", unit: None, choices: None }),
    ("geometry.objects.*.spaces", NormFieldMeta { label_en: "Spaces", label_de: "Räume", unit: None, choices: None }),
    ("geometry.objects.*.spaces[]", NormFieldMeta { label_en: "Space", label_de: "Raum", unit: None, choices: None }),
    ("geometry.objects.*.spaces[].bounds", NormFieldMeta { label_en: "Bounds", label_de: "Grenzen", unit: Some("m"), choices: None }),
    ("geometry.objects.*.spaces[].bounds.min", NormFieldMeta { label_en: "Minimum", label_de: "Minimum", unit: Some("m"), choices: None }),
    ("geometry.objects.*.spaces[].bounds.max", NormFieldMeta { label_en: "Maximum", label_de: "Maximum", unit: Some("m"), choices: None }),
    ("geometry.objects.*.surfaces", NormFieldMeta { label_en: "Surfaces", label_de: "Flächen", unit: None, choices: None }),
    ("geometry.objects.*.surfaces[]", NormFieldMeta { label_en: "Surface", label_de: "Fläche", unit: None, choices: None }),
    ("geometry.objects.*.parameterBindings", NormFieldMeta { label_en: "Parameter bindings", label_de: "Parameterbindungen", unit: None, choices: None }),
    ("geometry.objects.*.parameterBindings.*", NormFieldMeta { label_en: "Parameter binding", label_de: "Parameterbindung", unit: None, choices: None }),
    ("selection.classId", NormFieldMeta { label_en: "Class id", label_de: "Klassen-Id", unit: None, choices: None }),
    ("selection.seriesId", NormFieldMeta { label_en: "Series id", label_de: "Serien-Id", unit: None, choices: None }),
    ("selection.constraints", NormFieldMeta { label_en: "Constraints", label_de: "Einschränkungen", unit: None, choices: None }),
    ("selection.constraints[]", NormFieldMeta { label_en: "Constraint", label_de: "Einschränkung", unit: None, choices: None }),
    ("selection.constraints[].operator", NormFieldMeta { label_en: "Operator", label_de: "Operator", unit: None, choices: Some(OPERATORS) }),
    ("selection.constraints[].value", NormFieldMeta { label_en: "Value", label_de: "Wert", unit: None, choices: None }),
    ("id", NormFieldMeta { label_en: "Id", label_de: "Id", unit: None, choices: None }),
    ("names", NormFieldMeta { label_en: "Names", label_de: "Namen", unit: None, choices: None }),
    ("names.preferred", NormFieldMeta { label_en: "Preferred name", label_de: "Bevorzugter Name", unit: None, choices: None }),
    ("names.alternatives", NormFieldMeta { label_en: "Alternative names", label_de: "Alternativnamen", unit: None, choices: None }),
    ("names.alternatives[]", NormFieldMeta { label_en: "Alternative name", label_de: "Alternativname", unit: None, choices: None }),
    ("catalogue.metadata.names", NormFieldMeta { label_en: "Catalogue names", label_de: "Katalognamen", unit: None, choices: None }),
    ("catalogue.metadata.names.preferred", NormFieldMeta { label_en: "Preferred catalogue name", label_de: "Bevorzugter Katalogname", unit: None, choices: None }),
    ("catalogue.metadata.names.preferred.locale", NormFieldMeta { label_en: "Name locale", label_de: "Namenssprache", unit: None, choices: None }),
    ("catalogue.metadata.names.preferred.text", NormFieldMeta { label_en: "Name text", label_de: "Namenstext", unit: None, choices: None }),
    ("catalogue.manufacturer.names", NormFieldMeta { label_en: "Manufacturer names", label_de: "Herstellernamen", unit: None, choices: None }),
    ("catalogue.manufacturer.names.preferred", NormFieldMeta { label_en: "Preferred manufacturer name", label_de: "Bevorzugter Herstellername", unit: None, choices: None }),
    ("catalogue.manufacturer.names.preferred.locale", NormFieldMeta { label_en: "Name locale", label_de: "Namenssprache", unit: None, choices: None }),
    ("catalogue.manufacturer.names.preferred.text", NormFieldMeta { label_en: "Name text", label_de: "Namenstext", unit: None, choices: None }),
    ("catalogue.productGroups[].names", NormFieldMeta { label_en: "Group names", label_de: "Gruppennamen", unit: None, choices: None }),
    ("catalogue.productGroups[].names.preferred", NormFieldMeta { label_en: "Preferred group name", label_de: "Bevorzugter Gruppenname", unit: None, choices: None }),
    ("catalogue.productGroups[].names.preferred.locale", NormFieldMeta { label_en: "Name locale", label_de: "Namenssprache", unit: None, choices: None }),
    ("catalogue.productGroups[].names.preferred.text", NormFieldMeta { label_en: "Name text", label_de: "Namenstext", unit: None, choices: None }),
    ("catalogue.productGroups[].dictionarySubjectId", NormFieldMeta { label_en: "Dictionary subject id", label_de: "Wörterbuch-Subjekt-Id", unit: None, choices: None }),
    ("catalogue.productClasses[].names", NormFieldMeta { label_en: "Class names", label_de: "Klassennamen", unit: None, choices: None }),
    ("catalogue.productClasses[].names.preferred", NormFieldMeta { label_en: "Preferred class name", label_de: "Bevorzugter Klassenname", unit: None, choices: None }),
    ("catalogue.productClasses[].names.preferred.locale", NormFieldMeta { label_en: "Name locale", label_de: "Namenssprache", unit: None, choices: None }),
    ("catalogue.productClasses[].names.preferred.text", NormFieldMeta { label_en: "Name text", label_de: "Namenstext", unit: None, choices: None }),
    ("catalogue.productSeries[].names", NormFieldMeta { label_en: "Series names", label_de: "Seriennamen", unit: None, choices: None }),
    ("catalogue.productSeries[].names.preferred", NormFieldMeta { label_en: "Preferred series name", label_de: "Bevorzugter Serienname", unit: None, choices: None }),
    ("catalogue.productSeries[].names.preferred.locale", NormFieldMeta { label_en: "Name locale", label_de: "Namenssprache", unit: None, choices: None }),
    ("catalogue.productSeries[].names.preferred.text", NormFieldMeta { label_en: "Name text", label_de: "Namenstext", unit: None, choices: None }),
    ("catalogue.productSeries[].classId", NormFieldMeta { label_en: "Class id", label_de: "Klassen-Id", unit: None, choices: None }),
    ("catalogue.products[].names", NormFieldMeta { label_en: "Product names", label_de: "Produktnamen", unit: None, choices: None }),
    ("catalogue.products[].names.preferred", NormFieldMeta { label_en: "Preferred product name", label_de: "Bevorzugter Produktname", unit: None, choices: None }),
    ("catalogue.products[].names.preferred.locale", NormFieldMeta { label_en: "Name locale", label_de: "Namenssprache", unit: None, choices: None }),
    ("catalogue.products[].names.preferred.text", NormFieldMeta { label_en: "Name text", label_de: "Namenstext", unit: None, choices: None }),
    ("catalogue.products[].names.alternatives", NormFieldMeta { label_en: "Alternative product names", label_de: "Alternative Produktnamen", unit: None, choices: None }),
    ("catalogue.products[].names.alternatives[]", NormFieldMeta { label_en: "Alternative product name", label_de: "Alternativer Produktname", unit: None, choices: None }),
    ("catalogue.products[].names.alternatives[].locale", NormFieldMeta { label_en: "Name locale", label_de: "Namenssprache", unit: None, choices: None }),
    ("catalogue.products[].names.alternatives[].text", NormFieldMeta { label_en: "Name text", label_de: "Namenstext", unit: None, choices: None }),
    ("catalogue.products[].variants[].propertyValues", NormFieldMeta { label_en: "Property values", label_de: "Eigenschaftswerte", unit: None, choices: None }),
    ("catalogue.products[].variants[].propertyValues[]", NormFieldMeta { label_en: "Property value", label_de: "Eigenschaftswert", unit: None, choices: None }),
    ("catalogue.products[].variants[].propertyValues[].definitionId", NormFieldMeta { label_en: "Definition id", label_de: "Definitions-Id", unit: None, choices: None }),
    ("catalogue.products[].variants[].propertyValues[].value", NormFieldMeta { label_en: "Property value payload", label_de: "Eigenschaftswert-Nutzlast", unit: None, choices: None }),
    ("catalogue.products[].variants[].propertyValues[].value.decimal", NormFieldMeta { label_en: "Decimal value", label_de: "Dezimalwert", unit: None, choices: None }),
    ("catalogue.products[].variants[].propertyValues[].value.value", NormFieldMeta { label_en: "Numeric value", label_de: "Numerischer Wert", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].names", NormFieldMeta { label_en: "Property names", label_de: "Eigenschaftsnamen", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].names.preferred", NormFieldMeta { label_en: "Preferred property name", label_de: "Bevorzugter Eigenschaftsname", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].names.preferred.locale", NormFieldMeta { label_en: "Name locale", label_de: "Namenssprache", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].names.preferred.text", NormFieldMeta { label_en: "Name text", label_de: "Namenstext", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].kind", NormFieldMeta { label_en: "Property kind", label_de: "Eigenschaftsart", unit: None, choices: Some(PROPERTY_KINDS) }),
    ("catalogue.propertyDefinitions[].id", NormFieldMeta { label_en: "Property id", label_de: "Eigenschafts-Id", unit: None, choices: None }),
    ("catalogue.productIndexes[].id", NormFieldMeta { label_en: "Index id", label_de: "Index-Id", unit: None, choices: None }),
    ("catalogue.productIndexes[].productId", NormFieldMeta { label_en: "Product id", label_de: "Produkt-Id", unit: None, choices: None }),
    ("catalogue.descriptiveObjects[].id", NormFieldMeta { label_en: "Media id", label_de: "Medien-Id", unit: None, choices: None }),
    ("catalogue.descriptiveObjects[].mimeType", NormFieldMeta { label_en: "MIME type", label_de: "MIME-Typ", unit: None, choices: None }),
    ("catalogue.descriptiveObjects[].uri", NormFieldMeta { label_en: "Media URI", label_de: "Medien-URI", unit: None, choices: None }),
    ("dictionary.subjects[].id", NormFieldMeta { label_en: "Subject id", label_de: "Subjekt-Id", unit: None, choices: None }),
    ("dictionary.subjects[].kind", NormFieldMeta { label_en: "Subject kind", label_de: "Subjektart", unit: None, choices: Some(SUBJECT_KINDS) }),
    ("dictionary.subjects[].names", NormFieldMeta { label_en: "Subject names", label_de: "Subjektnamen", unit: None, choices: None }),
    ("dictionary.subjects[].names.preferred", NormFieldMeta { label_en: "Preferred subject name", label_de: "Bevorzugter Subjektname", unit: None, choices: None }),
    ("dictionary.subjects[].names.preferred.locale", NormFieldMeta { label_en: "Name locale", label_de: "Namenssprache", unit: None, choices: None }),
    ("dictionary.subjects[].names.preferred.text", NormFieldMeta { label_en: "Name text", label_de: "Namenstext", unit: None, choices: None }),
    ("dictionary.relationships[].id", NormFieldMeta { label_en: "Relationship id", label_de: "Beziehungs-Id", unit: None, choices: None }),
    ("dictionary.relationships[].kind", NormFieldMeta { label_en: "Relationship kind", label_de: "Beziehungsart", unit: None, choices: Some(RELATIONSHIP_KINDS) }),
    ("dictionary.relationships[].sourceId", NormFieldMeta { label_en: "Source id", label_de: "Quell-Id", unit: None, choices: None }),
    ("dictionary.relationships[].targetId", NormFieldMeta { label_en: "Target id", label_de: "Ziel-Id", unit: None, choices: None }),
    ("dictionary.controlledLists[].id", NormFieldMeta { label_en: "List id", label_de: "Listen-Id", unit: None, choices: None }),
    ("geometry.objects.*.id", NormFieldMeta { label_en: "Geometry id", label_de: "Geometrie-Id", unit: None, choices: None }),
    ("geometry.objects.*.spaces[].id", NormFieldMeta { label_en: "Space id", label_de: "Raum-Id", unit: None, choices: None }),
    ("geometry.objects.*.spaces[].kind", NormFieldMeta { label_en: "Space kind", label_de: "Raumart", unit: None, choices: Some(SPACE_KINDS) }),
    ("geometry.objects.*.spaces[].bounds.min[]", NormFieldMeta { label_en: "Minimum coordinate", label_de: "Minimumskoordinate", unit: Some("m"), choices: None }),
    ("geometry.objects.*.spaces[].bounds.max[]", NormFieldMeta { label_en: "Maximum coordinate", label_de: "Maximumskoordinate", unit: Some("m"), choices: None }),
    ("geometry.objects.*.surfaces[].id", NormFieldMeta { label_en: "Surface id", label_de: "Flächen-Id", unit: None, choices: None }),
    ("selection.constraints[].id", NormFieldMeta { label_en: "Constraint id", label_de: "Einschränkungs-Id", unit: None, choices: None }),
    ("selection.constraints[].propertyId", NormFieldMeta { label_en: "Property id", label_de: "Eigenschafts-Id", unit: None, choices: None }),

    ("catalogue.id", NormFieldMeta { label_en: "Catalogue id", label_de: "Katalog-Id", unit: None, choices: None }),
    ("catalogue.metadata.lifecycle", NormFieldMeta { label_en: "Lifecycle", label_de: "Lebenszyklus", unit: None, choices: None }),
    ("catalogue.metadata.lifecycle.status", NormFieldMeta { label_en: "Lifecycle status", label_de: "Lebenszyklusstatus", unit: None, choices: None }),
    ("catalogue.metadata.lifecycle.revision", NormFieldMeta { label_en: "Revision", label_de: "Revision", unit: None, choices: None }),
    ("catalogue.manufacturer.id", NormFieldMeta { label_en: "Manufacturer id", label_de: "Hersteller-Id", unit: None, choices: None }),
    ("catalogue.dictionary", NormFieldMeta { label_en: "Dictionary reference", label_de: "Wörterbuch-Referenz", unit: None, choices: None }),
    ("catalogue.dictionary.id", NormFieldMeta { label_en: "Dictionary id", label_de: "Wörterbuch-Id", unit: None, choices: None }),
    ("catalogue.dictionary.version", NormFieldMeta { label_en: "Dictionary version", label_de: "Wörterbuch-Version", unit: None, choices: None }),
    ("catalogue.compositions", NormFieldMeta { label_en: "Compositions", label_de: "Zusammensetzungen", unit: None, choices: None }),
    ("catalogue.compositions.*", NormFieldMeta { label_en: "Composition set", label_de: "Zusammensetzungsmenge", unit: None, choices: None }),
    ("catalogue.compositions.*[]", NormFieldMeta { label_en: "Composition item", label_de: "Zusammensetzungselement", unit: None, choices: None }),
    ("catalogue.compositions.*[].quantity", NormFieldMeta { label_en: "Quantity", label_de: "Menge", unit: None, choices: None }),
    ("catalogue.compositions.*[].componentProductId", NormFieldMeta { label_en: "Component product id", label_de: "Komponenten-Produkt-Id", unit: None, choices: None }),
    ("catalogue.products[].parameterDomains", NormFieldMeta { label_en: "Parameter domains", label_de: "Parameterbereiche", unit: None, choices: None }),
    ("catalogue.products[].parameterDomains[]", NormFieldMeta { label_en: "Parameter domain", label_de: "Parameterbereich", unit: None, choices: None }),
    ("catalogue.products[].parameterDomains[].parameterId", NormFieldMeta { label_en: "Parameter id", label_de: "Parameter-Id", unit: None, choices: None }),
    ("catalogue.products[].parameterDomains[].defaultValue", NormFieldMeta { label_en: "Default value", label_de: "Standardwert", unit: None, choices: None }),
    ("catalogue.products[].parameterDomains[].defaultValue.value", NormFieldMeta { label_en: "Default numeric value", label_de: "Numerischer Standardwert", unit: None, choices: None }),
    ("catalogue.products[].parameterDomains[].allowedValues", NormFieldMeta { label_en: "Allowed values", label_de: "Erlaubte Werte", unit: None, choices: None }),
    ("catalogue.products[].parameterDomains[].allowedValues[]", NormFieldMeta { label_en: "Allowed value", label_de: "Erlaubter Wert", unit: None, choices: None }),
    ("catalogue.products[].parameterDomains[].allowedValues[].value", NormFieldMeta { label_en: "Allowed numeric value", label_de: "Erlaubter Zahlenwert", unit: None, choices: None }),
    ("catalogue.products[].variants[].articleNumber", NormFieldMeta { label_en: "Article number", label_de: "Artikelnummer", unit: None, choices: None }),
    ("catalogue.productIndexes[].searchTags", NormFieldMeta { label_en: "Search tags", label_de: "Suchbegriffe", unit: None, choices: None }),
    ("catalogue.productIndexes[].searchTags[]", NormFieldMeta { label_en: "Search tag", label_de: "Suchbegriff", unit: None, choices: None }),
    ("catalogue.productIndexes[].productId", NormFieldMeta { label_en: "Indexed product id", label_de: "Indexierte Produkt-Id", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].dataType", NormFieldMeta { label_en: "Data type", label_de: "Datentyp", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].dictionaryPropertyId", NormFieldMeta { label_en: "Dictionary property id", label_de: "Wörterbuch-Eigenschafts-Id", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].cardinality", NormFieldMeta { label_en: "Cardinality", label_de: "Kardinalität", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].cardinality.min", NormFieldMeta { label_en: "Minimum cardinality", label_de: "Minimale Kardinalität", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].cardinality.max", NormFieldMeta { label_en: "Maximum cardinality", label_de: "Maximale Kardinalität", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].unit", NormFieldMeta { label_en: "Unit", label_de: "Einheit", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].unit.symbol", NormFieldMeta { label_en: "Unit symbol", label_de: "Einheitszeichen", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].unit.siFactor", NormFieldMeta { label_en: "SI factor", label_de: "SI-Faktor", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].unit.dimension", NormFieldMeta { label_en: "Dimension", label_de: "Dimension", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].unit.dimension.length", NormFieldMeta { label_en: "Length dimension", label_de: "Längendimension", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].unit.dimension.mass", NormFieldMeta { label_en: "Mass dimension", label_de: "Massendimension", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].unit.dimension.time", NormFieldMeta { label_en: "Time dimension", label_de: "Zeitdimension", unit: None, choices: None }),
    ("catalogue.propertyDefinitions[].unit.dimension.temperature", NormFieldMeta { label_en: "Temperature dimension", label_de: "Temperaturdimension", unit: None, choices: None }),
    ("dictionary.reference", NormFieldMeta { label_en: "Dictionary reference", label_de: "Wörterbuch-Referenz", unit: None, choices: None }),
    ("dictionary.reference.id", NormFieldMeta { label_en: "Reference id", label_de: "Referenz-Id", unit: None, choices: None }),
    ("dictionary.reference.version", NormFieldMeta { label_en: "Reference version", label_de: "Referenzversion", unit: None, choices: None }),
    ("dictionary.properties", NormFieldMeta { label_en: "Dictionary properties", label_de: "Wörterbuch-Eigenschaften", unit: None, choices: None }),
    ("dictionary.properties[]", NormFieldMeta { label_en: "Dictionary property", label_de: "Wörterbuch-Eigenschaft", unit: None, choices: None }),
    ("dictionary.properties[].id", NormFieldMeta { label_en: "Property id", label_de: "Eigenschafts-Id", unit: None, choices: None }),
    ("dictionary.properties[].dataType", NormFieldMeta { label_en: "Data type", label_de: "Datentyp", unit: None, choices: None }),
    ("dictionary.properties[].applicableSubjectIds", NormFieldMeta { label_en: "Applicable subjects", label_de: "Anwendbare Subjekte", unit: None, choices: None }),
    ("dictionary.properties[].applicableSubjectIds[]", NormFieldMeta { label_en: "Applicable subject id", label_de: "Anwendbare Subjekt-Id", unit: None, choices: None }),
    ("dictionary.properties[].valueConstraints", NormFieldMeta { label_en: "Value constraints", label_de: "Werteinschränkungen", unit: None, choices: None }),
    ("dictionary.properties[].valueConstraints[]", NormFieldMeta { label_en: "Value constraint", label_de: "Werteinschränkung", unit: None, choices: None }),
    ("dictionary.properties[].valueConstraints[].min", NormFieldMeta { label_en: "Minimum", label_de: "Minimum", unit: None, choices: None }),
    ("dictionary.properties[].valueConstraints[].max", NormFieldMeta { label_en: "Maximum", label_de: "Maximum", unit: None, choices: None }),
    ("dictionary.properties[].unit", NormFieldMeta { label_en: "Unit", label_de: "Einheit", unit: None, choices: None }),
    ("dictionary.properties[].unit.symbol", NormFieldMeta { label_en: "Unit symbol", label_de: "Einheitszeichen", unit: None, choices: None }),
    ("dictionary.properties[].unit.siFactor", NormFieldMeta { label_en: "SI factor", label_de: "SI-Faktor", unit: None, choices: None }),
    ("dictionary.properties[].unit.dimension", NormFieldMeta { label_en: "Dimension", label_de: "Dimension", unit: None, choices: None }),
    ("dictionary.properties[].unit.dimension.length", NormFieldMeta { label_en: "Length dimension", label_de: "Längendimension", unit: None, choices: None }),
    ("dictionary.properties[].unit.dimension.mass", NormFieldMeta { label_en: "Mass dimension", label_de: "Massendimension", unit: None, choices: None }),
    ("dictionary.properties[].unit.dimension.time", NormFieldMeta { label_en: "Time dimension", label_de: "Zeitdimension", unit: None, choices: None }),
    ("dictionary.properties[].unit.dimension.temperature", NormFieldMeta { label_en: "Temperature dimension", label_de: "Temperaturdimension", unit: None, choices: None }),
    ("dictionary.controlledLists[].values", NormFieldMeta { label_en: "List values", label_de: "Listenwerte", unit: None, choices: None }),
    ("dictionary.controlledLists[].values[]", NormFieldMeta { label_en: "List value", label_de: "Listenwert", unit: None, choices: None }),
    ("dictionary.controlledLists[].contextSubjectIds", NormFieldMeta { label_en: "Context subjects", label_de: "Kontextsubjekte", unit: None, choices: None }),
    ("dictionary.controlledLists[].contextSubjectIds[]", NormFieldMeta { label_en: "Context subject id", label_de: "Kontext-Subjekt-Id", unit: None, choices: None }),
    ("dictionary.relationships[].cardinality", NormFieldMeta { label_en: "Cardinality", label_de: "Kardinalität", unit: None, choices: None }),
    ("dictionary.relationships[].cardinality.min", NormFieldMeta { label_en: "Minimum cardinality", label_de: "Minimale Kardinalität", unit: None, choices: None }),
    ("dictionary.relationships[].cardinality.max", NormFieldMeta { label_en: "Maximum cardinality", label_de: "Maximale Kardinalität", unit: None, choices: None }),
    ("geometry.objects.*.ports", NormFieldMeta { label_en: "Ports", label_de: "Anschlüsse", unit: None, choices: None }),
    ("geometry.objects.*.ports[]", NormFieldMeta { label_en: "Port", label_de: "Anschluss", unit: None, choices: None }),
    ("geometry.objects.*.ports[].id", NormFieldMeta { label_en: "Port id", label_de: "Anschluss-Id", unit: None, choices: None }),
    ("geometry.objects.*.ports[].medium", NormFieldMeta { label_en: "Medium", label_de: "Medium", unit: None, choices: None }),
    ("geometry.objects.*.ports[].portType", NormFieldMeta { label_en: "Port type", label_de: "Anschlussart", unit: None, choices: None }),
    ("geometry.objects.*.ports[].position", NormFieldMeta { label_en: "Position", label_de: "Position", unit: Some("m"), choices: None }),
    ("geometry.objects.*.ports[].position[]", NormFieldMeta { label_en: "Position coordinate", label_de: "Positionskoordinate", unit: Some("m"), choices: None }),
    ("geometry.objects.*.ports[].direction", NormFieldMeta { label_en: "Direction", label_de: "Richtung", unit: None, choices: None }),
    ("geometry.objects.*.ports[].direction[]", NormFieldMeta { label_en: "Direction component", label_de: "Richtungskomponente", unit: None, choices: None }),
    ("geometry.primitiveRegistry", NormFieldMeta { label_en: "Primitive registry", label_de: "Primitiv-Register", unit: None, choices: None }),
    ("geometry.primitiveRegistry[]", NormFieldMeta { label_en: "Primitive kind", label_de: "Primitivart", unit: None, choices: None }),
    ("geometry.primitiveRegistry[].id", NormFieldMeta { label_en: "Primitive id", label_de: "Primitiv-Id", unit: None, choices: None }),
    ("geometry.primitiveRegistry[].parameters", NormFieldMeta { label_en: "Parameter names", label_de: "Parameternamen", unit: None, choices: None }),
    ("geometry.primitiveRegistry[].parameters[]", NormFieldMeta { label_en: "Parameter name", label_de: "Parametername", unit: None, choices: None }),
    ("partNumberRule.functionId", NormFieldMeta { label_en: "Function id", label_de: "Funktions-Id", unit: None, choices: None }),
    ("partNumberRule.function_id", NormFieldMeta { label_en: "Function id", label_de: "Funktions-Id", unit: None, choices: None }),
    ("partNumberInputs.*", NormFieldMeta { label_en: "Part-number input", label_de: "Teilenummern-Eingabe", unit: None, choices: None }),
    ("partNumberInputs.*.value", NormFieldMeta { label_en: "Input value", label_de: "Eingabewert", unit: None, choices: None }),
    ("selection.constraints[].id", NormFieldMeta { label_en: "Constraint id", label_de: "Einschränkungs-Id", unit: None, choices: None }),
    ("selection.constraints[].propertyId", NormFieldMeta { label_en: "Property id", label_de: "Eigenschafts-Id", unit: None, choices: None }),

];

/// 🏷️ Exact / `[]` / `*` template lookup for the ISO 16757 catalogue subject.
pub fn iso16757_field_meta(path: &str) -> Option<NormFieldMeta> {
    if let Some(rest) = path.strip_prefix("catalogue.compositions.") {
        let templated = if let Some((_key, tail)) = rest.split_once('.') {
            format!("catalogue.compositions.*.{tail}")
        } else if rest.contains('[') {
            // compositions.{key}[i].field
            let after = rest.split_once(']').map(|(_, t)| t.trim_start_matches('.')).unwrap_or("");
            if after.is_empty() { "catalogue.compositions.*[]".into() } else { format!("catalogue.compositions.*[].{after}") }
        } else {
            "catalogue.compositions.*".into()
        };
        if let Some(meta) = lookup_norm_field_meta(TABLE, &templated) {
            return Some(meta);
        }
    }
    if path.contains("partNumberInputs.") && path.ends_with(".value") {
        return lookup_norm_field_meta(TABLE, "partNumberInputs.*.value");
    }
    if let Some(rest) = path.strip_prefix("geometry.objects.") {
        let templated = if let Some((id, tail)) = rest.split_once('.') {
            if id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
                format!("geometry.objects.*.{tail}")
            } else {
                String::new()
            }
        } else {
            "geometry.objects.*".into()
        };
        if !templated.is_empty() {
            if let Some(meta) = lookup_norm_field_meta(TABLE, &templated) {
                return Some(meta);
            }
        }
    }
    if path.contains(".parameterValues.") {
        return lookup_norm_field_meta(TABLE, "catalogue.products[].variants[].parameterValues.*");
    }
    if path.contains(".parameterBindings.") {
        return lookup_norm_field_meta(TABLE, "geometry.objects.*.parameterBindings.*");
    }
    lookup_norm_field_meta(TABLE, path)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
