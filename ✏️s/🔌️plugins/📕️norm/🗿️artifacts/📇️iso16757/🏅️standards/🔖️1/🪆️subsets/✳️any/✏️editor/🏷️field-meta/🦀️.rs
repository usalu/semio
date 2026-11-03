//! 🏷️ ISO 16757 FieldSpec — en/de labels, SI display units, `[]` wildcards, localized choices.

use semio_s_plugin_norm_app_surface::{lookup_norm_field_meta, NormFieldChoice, FieldSpec, UnitId};

const EXCHANGE_PROCESS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "CreateFromDictionary", label_en: "Create from dictionary", label_de: "Aus Merkmalsverzeichnis erstellen" },
    NormFieldChoice { value: "ProvideCatalogue", label_en: "Provide catalogue", label_de: "Katalog bereitstellen" },
    NormFieldChoice { value: "DetermineProduct", label_en: "Determine product", label_de: "Produkt bestimmen" },
    NormFieldChoice { value: "IntegrateIntoSystem", label_en: "Integrate into system", label_de: "In System einbinden" },
    NormFieldChoice { value: "ExchangeSystemModel", label_en: "Exchange system model", label_de: "Systemmodell austauschen" },
];

const EDITION_PROFILE: &[NormFieldChoice] = &[
    NormFieldChoice { value: "Part1_2015", label_en: "Part 1 (2015)", label_de: "Teil 1 (2015)" },
    NormFieldChoice { value: "Part2_2016", label_en: "Part 2 (2016)", label_de: "Teil 2 (2016)" },
    NormFieldChoice { value: "Part4_2025", label_en: "Part 4 (2025)", label_de: "Teil 4 (2025)" },
    NormFieldChoice { value: "Part5_2025", label_en: "Part 5 (2025)", label_de: "Teil 5 (2025)" },
    NormFieldChoice { value: "FullPublished", label_en: "Full published edition", label_de: "Vollständig veröffentlichte Ausgabe" },
];

const OPERATORS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "Equal", label_en: "Equal", label_de: "Gleich" },
    NormFieldChoice { value: "NotEqual", label_en: "Not equal", label_de: "Ungleich" },
    NormFieldChoice { value: "LessThan", label_en: "Less than", label_de: "Kleiner als" },
    NormFieldChoice { value: "GreaterThan", label_en: "Greater than", label_de: "Größer als" },
    NormFieldChoice { value: "InRange", label_en: "In range", label_de: "Im Bereich" },
];

const SPACE_KINDS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "Overall", label_en: "Overall space", label_de: "Gesamtraum" },
    NormFieldChoice { value: "Operation", label_en: "Operating space", label_de: "Bedienraum" },
    NormFieldChoice { value: "Access", label_en: "Access space", label_de: "Zugangsraum" },
    NormFieldChoice { value: "PlacementTransportation", label_en: "Placement and transport space", label_de: "Einbring- und Transportraum" },
    NormFieldChoice { value: "Installation", label_en: "Installation space", label_de: "Montageraum" },
];

const SUBJECT_KINDS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "ProductGroup", label_en: "Product group", label_de: "Produktgruppe" },
    NormFieldChoice { value: "ProductClass", label_en: "Product class", label_de: "Produktklasse" },
    NormFieldChoice { value: "ProductSpecialization", label_en: "Product specialization", label_de: "Produktspezialisierung" },
    NormFieldChoice { value: "CatalogueMetadata", label_en: "Catalogue metadata", label_de: "Katalog-Metadaten" },
    NormFieldChoice { value: "ManufacturerMetadata", label_en: "Manufacturer metadata", label_de: "Hersteller-Metadaten" },
    NormFieldChoice { value: "PropertyBlock", label_en: "Property block", label_de: "Merkmalsblock" },
    NormFieldChoice { value: "Port", label_en: "Port", label_de: "Anschluss" },
    NormFieldChoice { value: "Inlet", label_en: "Inlet", label_de: "Eintritt" },
    NormFieldChoice { value: "Outlet", label_en: "Outlet", label_de: "Austritt" },
    NormFieldChoice { value: "InOutlet", label_en: "Inlet/outlet", label_de: "Ein-/Austritt" },
];

const RELATIONSHIP_KINDS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "IsSubtypeOf", label_en: "Is subtype of", label_de: "Ist Untertyp von" },
    NormFieldChoice { value: "HasPart", label_en: "Has part", label_de: "Hat Teil" },
    NormFieldChoice { value: "HasBlock", label_en: "Has block", label_de: "Hat Block" },
    NormFieldChoice { value: "IsDependentOn", label_en: "Is dependent on", label_de: "Hängt ab von" },
    NormFieldChoice { value: "IsSubkindOf", label_en: "Is subkind of", label_de: "Ist Unterart von" },
];

const PROPERTY_KINDS: &[NormFieldChoice] = &[
    NormFieldChoice { value: "Static", label_en: "Static", label_de: "Statisch" },
    NormFieldChoice { value: "Dynamic", label_en: "Dynamic", label_de: "Dynamisch" },
    NormFieldChoice { value: "Selection", label_en: "Selection", label_de: "Auswahl" },
    NormFieldChoice { value: "External", label_en: "External", label_de: "Extern" },
];

const TABLE: &[(&str, FieldSpec)] = &[
    ("catalogue", FieldSpec::new("Catalogue", "Katalog")),
    ("dictionary", FieldSpec::new("Dictionary", "Wörterbuch")),
    ("geometry", FieldSpec::new("Geometry catalogue", "Geometriekatalog")),
    ("selection", FieldSpec::new("Selection request", "Auswahlauftrag")),
    ("exchangeProcess", FieldSpec::choice("Exchange process", "Austauschprozess", EXCHANGE_PROCESS)),
    ("partNumberRule", FieldSpec::new("Part-number rule", "Teilenummernregel")),
    ("partNumberRule.source", FieldSpec::new("Part-number rule source", "Teilenummernregel-Quelltext")),
    ("partNumberRule.kind", FieldSpec::new("Part-number rule kind", "Teilenummernregel-Art")),
    ("partNumberInputs", FieldSpec::new("Part-number inputs", "Teilenummern-Eingaben")),
    ("scriptLimits", FieldSpec::new("Script limits", "Skriptgrenzen")),
    ("scriptLimits.maxSteps", FieldSpec::new("Max steps", "Max. Schritte")),
    ("scriptLimits.maxRecursion", FieldSpec::new("Max recursion", "Max. Rekursion")),
    ("scriptLimits.timeoutMs", FieldSpec::quantity("Timeout", "Zeitlimit", UnitId::Millisecond)),
    ("catalogue.metadata", FieldSpec::new("Metadata", "Metadaten")),
    ("catalogue.metadata.editionProfile", FieldSpec::choice("Edition profile", "Editionsprofil", EDITION_PROFILE)),
    ("catalogue.manufacturer", FieldSpec::new("Manufacturer", "Hersteller")),
    ("catalogue.productGroups", FieldSpec::new("Product groups", "Produktgruppen")),
    ("catalogue.productGroups[]", FieldSpec::new("Product group", "Produktgruppe")),
    ("catalogue.productGroups[].id", FieldSpec::new("Group id", "Gruppen-Id")),
    ("catalogue.productClasses", FieldSpec::new("Product classes", "Produktklassen")),
    ("catalogue.productClasses[]", FieldSpec::new("Product class", "Produktklasse")),
    ("catalogue.productClasses[].id", FieldSpec::new("Class id", "Klassen-Id")),
    ("catalogue.productClasses[].groupId", FieldSpec::new("Group id", "Gruppen-Id")),
    ("catalogue.productSeries", FieldSpec::new("Product series", "Produktserien")),
    ("catalogue.productSeries[]", FieldSpec::new("Product series entry", "Produktserie")),
    ("catalogue.productSeries[].id", FieldSpec::new("Series id", "Serien-Id")),
    ("catalogue.products", FieldSpec::new("Products", "Produkte")),
    ("catalogue.products[]", FieldSpec::new("Product", "Produkt")),
    ("catalogue.products[].id", FieldSpec::new("Product id", "Produkt-Id")),
    ("catalogue.products[].seriesId", FieldSpec::new("Series id", "Serien-Id")),
    ("catalogue.products[].variants", FieldSpec::new("Variants", "Varianten")),
    ("catalogue.products[].variants[]", FieldSpec::new("Variant", "Variante")),
    ("catalogue.products[].variants[].id", FieldSpec::new("Variant id", "Varianten-Id")),
    ("catalogue.products[].variants[].geometryId", FieldSpec::new("Geometry id", "Geometrie-Id")),
    ("catalogue.products[].variants[].parameterValues", FieldSpec::new("Parameter values", "Parameterwerte")),
    ("catalogue.products[].variants[].parameterValues.*", FieldSpec::new("Parameter value", "Parameterwert")),
    ("catalogue.propertyDefinitions", FieldSpec::new("Property definitions", "Eigenschaftsdefinitionen")),
    ("catalogue.propertyDefinitions[]", FieldSpec::new("Property definition", "Eigenschaftsdefinition")),
    ("catalogue.productIndexes", FieldSpec::new("Product indexes", "Produktindizes")),
    ("catalogue.productIndexes[]", FieldSpec::new("Product index", "Produktindex")),
    ("catalogue.descriptiveObjects", FieldSpec::new("Descriptive media", "Beschreibende Medien")),
    ("catalogue.descriptiveObjects[]", FieldSpec::new("Descriptive object", "Beschreibendes Objekt")),
    ("dictionary.subjects", FieldSpec::new("Subjects", "Subjekte")),
    ("dictionary.subjects[]", FieldSpec::new("Subject", "Subjekt")),
    ("dictionary.relationships", FieldSpec::new("Relationships", "Beziehungen")),
    ("dictionary.relationships[]", FieldSpec::new("Relationship", "Beziehung")),
    ("dictionary.controlledLists", FieldSpec::new("Controlled lists", "Kontrollierte Listen")),
    ("dictionary.controlledLists[]", FieldSpec::new("Controlled list", "Kontrollierte Liste")),
    ("geometry.objects", FieldSpec::new("Geometry objects", "Geometrieobjekte")),
    ("geometry.objects.*", FieldSpec::new("Geometry object", "Geometrieobjekt")),
    ("geometry.objects.*.spaces", FieldSpec::new("Spaces", "Räume")),
    ("geometry.objects.*.spaces[]", FieldSpec::new("Space", "Raum")),
    ("geometry.objects.*.spaces[].bounds", FieldSpec::quantity("Bounds", "Grenzen", UnitId::Metre)),
    ("geometry.objects.*.spaces[].bounds.min", FieldSpec::quantity("Minimum", "Minimum", UnitId::Metre)),
    ("geometry.objects.*.spaces[].bounds.max", FieldSpec::quantity("Maximum", "Maximum", UnitId::Metre)),
    ("geometry.objects.*.surfaces", FieldSpec::new("Surfaces", "Flächen")),
    ("geometry.objects.*.surfaces[]", FieldSpec::new("Surface", "Fläche")),
    ("geometry.objects.*.parameterBindings", FieldSpec::new("Parameter bindings", "Parameterbindungen")),
    ("geometry.objects.*.parameterBindings.*", FieldSpec::new("Parameter binding", "Parameterbindung")),
    ("selection.classId", FieldSpec::new("Class id", "Klassen-Id")),
    ("selection.seriesId", FieldSpec::new("Series id", "Serien-Id")),
    ("selection.constraints", FieldSpec::new("Constraints", "Einschränkungen")),
    ("selection.constraints[]", FieldSpec::new("Constraint", "Einschränkung")),
    ("selection.constraints[].operator", FieldSpec::choice("Operator", "Operator", OPERATORS)),
    ("selection.constraints[].value", FieldSpec::new("Value", "Wert")),
    ("id", FieldSpec::new("Id", "Id")),
    ("names", FieldSpec::new("Names", "Namen")),
    ("names.preferred", FieldSpec::new("Preferred name", "Bevorzugter Name")),
    ("names.alternatives", FieldSpec::new("Alternative names", "Alternativnamen")),
    ("names.alternatives[]", FieldSpec::new("Alternative name", "Alternativname")),
    ("catalogue.metadata.names", FieldSpec::new("Catalogue names", "Katalognamen")),
    ("catalogue.metadata.names.preferred", FieldSpec::new("Preferred catalogue name", "Bevorzugter Katalogname")),
    ("catalogue.metadata.names.preferred.locale", FieldSpec::new("Name locale", "Namenssprache")),
    ("catalogue.metadata.names.preferred.text", FieldSpec::new("Name text", "Namenstext")),
    ("catalogue.manufacturer.names", FieldSpec::new("Manufacturer names", "Herstellernamen")),
    ("catalogue.manufacturer.names.preferred", FieldSpec::new("Preferred manufacturer name", "Bevorzugter Herstellername")),
    ("catalogue.manufacturer.names.preferred.locale", FieldSpec::new("Name locale", "Namenssprache")),
    ("catalogue.manufacturer.names.preferred.text", FieldSpec::new("Name text", "Namenstext")),
    ("catalogue.productGroups[].names", FieldSpec::new("Group names", "Gruppennamen")),
    ("catalogue.productGroups[].names.preferred", FieldSpec::new("Preferred group name", "Bevorzugter Gruppenname")),
    ("catalogue.productGroups[].names.preferred.locale", FieldSpec::new("Name locale", "Namenssprache")),
    ("catalogue.productGroups[].names.preferred.text", FieldSpec::new("Name text", "Namenstext")),
    ("catalogue.productGroups[].dictionarySubjectId", FieldSpec::new("Dictionary subject id", "Wörterbuch-Subjekt-Id")),
    ("catalogue.productClasses[].names", FieldSpec::new("Class names", "Klassennamen")),
    ("catalogue.productClasses[].names.preferred", FieldSpec::new("Preferred class name", "Bevorzugter Klassenname")),
    ("catalogue.productClasses[].names.preferred.locale", FieldSpec::new("Name locale", "Namenssprache")),
    ("catalogue.productClasses[].names.preferred.text", FieldSpec::new("Name text", "Namenstext")),
    ("catalogue.productSeries[].names", FieldSpec::new("Series names", "Seriennamen")),
    ("catalogue.productSeries[].names.preferred", FieldSpec::new("Preferred series name", "Bevorzugter Serienname")),
    ("catalogue.productSeries[].names.preferred.locale", FieldSpec::new("Name locale", "Namenssprache")),
    ("catalogue.productSeries[].names.preferred.text", FieldSpec::new("Name text", "Namenstext")),
    ("catalogue.productSeries[].classId", FieldSpec::new("Class id", "Klassen-Id")),
    ("catalogue.products[].names", FieldSpec::new("Product names", "Produktnamen")),
    ("catalogue.products[].names.preferred", FieldSpec::new("Preferred product name", "Bevorzugter Produktname")),
    ("catalogue.products[].names.preferred.locale", FieldSpec::new("Name locale", "Namenssprache")),
    ("catalogue.products[].names.preferred.text", FieldSpec::new("Name text", "Namenstext")),
    ("catalogue.products[].names.alternatives", FieldSpec::new("Alternative product names", "Alternative Produktnamen")),
    ("catalogue.products[].names.alternatives[]", FieldSpec::new("Alternative product name", "Alternativer Produktname")),
    ("catalogue.products[].names.alternatives[].locale", FieldSpec::new("Name locale", "Namenssprache")),
    ("catalogue.products[].names.alternatives[].text", FieldSpec::new("Name text", "Namenstext")),
    ("catalogue.products[].variants[].propertyValues", FieldSpec::new("Property values", "Eigenschaftswerte")),
    ("catalogue.products[].variants[].propertyValues[]", FieldSpec::new("Property value", "Eigenschaftswert")),
    ("catalogue.products[].variants[].propertyValues[].definitionId", FieldSpec::new("Definition id", "Definitions-Id")),
    ("catalogue.products[].variants[].propertyValues[].value", FieldSpec::new("Property value payload", "Eigenschaftswert-Nutzlast")),
    ("catalogue.products[].variants[].propertyValues[].value.decimal", FieldSpec::new("Decimal value", "Dezimalwert")),
    ("catalogue.products[].variants[].propertyValues[].value.value", FieldSpec::new("Numeric value", "Numerischer Wert")),
    ("catalogue.propertyDefinitions[].names", FieldSpec::new("Property names", "Eigenschaftsnamen")),
    ("catalogue.propertyDefinitions[].names.preferred", FieldSpec::new("Preferred property name", "Bevorzugter Eigenschaftsname")),
    ("catalogue.propertyDefinitions[].names.preferred.locale", FieldSpec::new("Name locale", "Namenssprache")),
    ("catalogue.propertyDefinitions[].names.preferred.text", FieldSpec::new("Name text", "Namenstext")),
    ("catalogue.propertyDefinitions[].kind", FieldSpec::choice("Property kind", "Eigenschaftsart", PROPERTY_KINDS)),
    ("catalogue.propertyDefinitions[].id", FieldSpec::new("Property id", "Eigenschafts-Id")),
    ("catalogue.productIndexes[].id", FieldSpec::new("Index id", "Index-Id")),
    ("catalogue.productIndexes[].productId", FieldSpec::new("Product id", "Produkt-Id")),
    ("catalogue.descriptiveObjects[].id", FieldSpec::new("Media id", "Medien-Id")),
    ("catalogue.descriptiveObjects[].mimeType", FieldSpec::new("MIME type", "MIME-Typ")),
    ("catalogue.descriptiveObjects[].uri", FieldSpec::new("Media URI", "Medien-URI")),
    ("dictionary.subjects[].id", FieldSpec::new("Subject id", "Subjekt-Id")),
    ("dictionary.subjects[].kind", FieldSpec::choice("Subject kind", "Subjektart", SUBJECT_KINDS)),
    ("dictionary.subjects[].names", FieldSpec::new("Subject names", "Subjektnamen")),
    ("dictionary.subjects[].names.preferred", FieldSpec::new("Preferred subject name", "Bevorzugter Subjektname")),
    ("dictionary.subjects[].names.preferred.locale", FieldSpec::new("Name locale", "Namenssprache")),
    ("dictionary.subjects[].names.preferred.text", FieldSpec::new("Name text", "Namenstext")),
    ("dictionary.relationships[].id", FieldSpec::new("Relationship id", "Beziehungs-Id")),
    ("dictionary.relationships[].kind", FieldSpec::choice("Relationship kind", "Beziehungsart", RELATIONSHIP_KINDS)),
    ("dictionary.relationships[].sourceId", FieldSpec::new("Source id", "Quell-Id")),
    ("dictionary.relationships[].targetId", FieldSpec::new("Target id", "Ziel-Id")),
    ("dictionary.controlledLists[].id", FieldSpec::new("List id", "Listen-Id")),
    ("geometry.objects.*.id", FieldSpec::new("Geometry id", "Geometrie-Id")),
    ("geometry.objects.*.spaces[].id", FieldSpec::new("Space id", "Raum-Id")),
    ("geometry.objects.*.spaces[].kind", FieldSpec::choice("Space kind", "Raumart", SPACE_KINDS)),
    ("geometry.objects.*.spaces[].bounds.min[]", FieldSpec::quantity("Minimum coordinate", "Minimumskoordinate", UnitId::Metre)),
    ("geometry.objects.*.spaces[].bounds.max[]", FieldSpec::quantity("Maximum coordinate", "Maximumskoordinate", UnitId::Metre)),
    ("geometry.objects.*.surfaces[].id", FieldSpec::new("Surface id", "Flächen-Id")),
    ("selection.constraints[].id", FieldSpec::new("Constraint id", "Einschränkungs-Id")),
    ("selection.constraints[].propertyId", FieldSpec::new("Property id", "Eigenschafts-Id")),

    ("catalogue.id", FieldSpec::new("Catalogue id", "Katalog-Id")),
    ("catalogue.metadata.lifecycle", FieldSpec::new("Lifecycle", "Lebenszyklus")),
    ("catalogue.metadata.lifecycle.status", FieldSpec::new("Lifecycle status", "Lebenszyklusstatus")),
    ("catalogue.metadata.lifecycle.revision", FieldSpec::new("Revision", "Revision")),
    ("catalogue.manufacturer.id", FieldSpec::new("Manufacturer id", "Hersteller-Id")),
    ("catalogue.dictionary", FieldSpec::new("Dictionary reference", "Wörterbuch-Referenz")),
    ("catalogue.dictionary.id", FieldSpec::new("Dictionary id", "Wörterbuch-Id")),
    ("catalogue.dictionary.version", FieldSpec::new("Dictionary version", "Wörterbuch-Version")),
    ("catalogue.compositions", FieldSpec::new("Compositions", "Zusammensetzungen")),
    ("catalogue.compositions.*", FieldSpec::new("Composition set", "Zusammensetzungsmenge")),
    ("catalogue.compositions.*[]", FieldSpec::new("Composition item", "Zusammensetzungselement")),
    ("catalogue.compositions.*[].quantity", FieldSpec::new("Quantity", "Menge")),
    ("catalogue.compositions.*[].componentProductId", FieldSpec::new("Component product id", "Komponenten-Produkt-Id")),
    ("catalogue.products[].parameterDomains", FieldSpec::new("Parameter domains", "Parameterbereiche")),
    ("catalogue.products[].parameterDomains[]", FieldSpec::new("Parameter domain", "Parameterbereich")),
    ("catalogue.products[].parameterDomains[].parameterId", FieldSpec::new("Parameter id", "Parameter-Id")),
    ("catalogue.products[].parameterDomains[].defaultValue", FieldSpec::new("Default value", "Standardwert")),
    ("catalogue.products[].parameterDomains[].defaultValue.value", FieldSpec::new("Default numeric value", "Numerischer Standardwert")),
    ("catalogue.products[].parameterDomains[].allowedValues", FieldSpec::new("Allowed values", "Erlaubte Werte")),
    ("catalogue.products[].parameterDomains[].allowedValues[]", FieldSpec::new("Allowed value", "Erlaubter Wert")),
    ("catalogue.products[].parameterDomains[].allowedValues[].value", FieldSpec::new("Allowed numeric value", "Erlaubter Zahlenwert")),
    ("catalogue.products[].variants[].articleNumber", FieldSpec::new("Article number", "Artikelnummer")),
    ("catalogue.productIndexes[].searchTags", FieldSpec::new("Search tags", "Suchbegriffe")),
    ("catalogue.productIndexes[].searchTags[]", FieldSpec::new("Search tag", "Suchbegriff")),
    ("catalogue.productIndexes[].productId", FieldSpec::new("Indexed product id", "Indexierte Produkt-Id")),
    ("catalogue.propertyDefinitions[].dataType", FieldSpec::new("Data type", "Datentyp")),
    ("catalogue.propertyDefinitions[].dictionaryPropertyId", FieldSpec::new("Dictionary property id", "Wörterbuch-Eigenschafts-Id")),
    ("catalogue.propertyDefinitions[].cardinality", FieldSpec::new("Cardinality", "Kardinalität")),
    ("catalogue.propertyDefinitions[].cardinality.min", FieldSpec::new("Minimum cardinality", "Minimale Kardinalität")),
    ("catalogue.propertyDefinitions[].cardinality.max", FieldSpec::new("Maximum cardinality", "Maximale Kardinalität")),
    ("catalogue.propertyDefinitions[].unit", FieldSpec::new("Unit", "Einheit")),
    ("catalogue.propertyDefinitions[].unit.symbol", FieldSpec::new("Unit symbol", "Einheitszeichen")),
    ("catalogue.propertyDefinitions[].unit.siFactor", FieldSpec::new("SI factor", "SI-Faktor")),
    ("catalogue.propertyDefinitions[].unit.dimension", FieldSpec::new("Dimension", "Dimension")),
    ("catalogue.propertyDefinitions[].unit.dimension.length", FieldSpec::new("Length dimension", "Längendimension")),
    ("catalogue.propertyDefinitions[].unit.dimension.mass", FieldSpec::new("Mass dimension", "Massendimension")),
    ("catalogue.propertyDefinitions[].unit.dimension.time", FieldSpec::new("Time dimension", "Zeitdimension")),
    ("catalogue.propertyDefinitions[].unit.dimension.temperature", FieldSpec::new("Temperature dimension", "Temperaturdimension")),
    ("dictionary.reference", FieldSpec::new("Dictionary reference", "Wörterbuch-Referenz")),
    ("dictionary.reference.id", FieldSpec::new("Reference id", "Referenz-Id")),
    ("dictionary.reference.version", FieldSpec::new("Reference version", "Referenzversion")),
    ("dictionary.properties", FieldSpec::new("Dictionary properties", "Wörterbuch-Eigenschaften")),
    ("dictionary.properties[]", FieldSpec::new("Dictionary property", "Wörterbuch-Eigenschaft")),
    ("dictionary.properties[].id", FieldSpec::new("Property id", "Eigenschafts-Id")),
    ("dictionary.properties[].dataType", FieldSpec::new("Data type", "Datentyp")),
    ("dictionary.properties[].applicableSubjectIds", FieldSpec::new("Applicable subjects", "Anwendbare Subjekte")),
    ("dictionary.properties[].applicableSubjectIds[]", FieldSpec::new("Applicable subject id", "Anwendbare Subjekt-Id")),
    ("dictionary.properties[].valueConstraints", FieldSpec::new("Value constraints", "Werteinschränkungen")),
    ("dictionary.properties[].valueConstraints[]", FieldSpec::new("Value constraint", "Werteinschränkung")),
    ("dictionary.properties[].valueConstraints[].min", FieldSpec::new("Minimum", "Minimum")),
    ("dictionary.properties[].valueConstraints[].max", FieldSpec::new("Maximum", "Maximum")),
    ("dictionary.properties[].unit", FieldSpec::new("Unit", "Einheit")),
    ("dictionary.properties[].unit.symbol", FieldSpec::new("Unit symbol", "Einheitszeichen")),
    ("dictionary.properties[].unit.siFactor", FieldSpec::new("SI factor", "SI-Faktor")),
    ("dictionary.properties[].unit.dimension", FieldSpec::new("Dimension", "Dimension")),
    ("dictionary.properties[].unit.dimension.length", FieldSpec::new("Length dimension", "Längendimension")),
    ("dictionary.properties[].unit.dimension.mass", FieldSpec::new("Mass dimension", "Massendimension")),
    ("dictionary.properties[].unit.dimension.time", FieldSpec::new("Time dimension", "Zeitdimension")),
    ("dictionary.properties[].unit.dimension.temperature", FieldSpec::new("Temperature dimension", "Temperaturdimension")),
    ("dictionary.controlledLists[].values", FieldSpec::new("List values", "Listenwerte")),
    ("dictionary.controlledLists[].values[]", FieldSpec::new("List value", "Listenwert")),
    ("dictionary.controlledLists[].contextSubjectIds", FieldSpec::new("Context subjects", "Kontextsubjekte")),
    ("dictionary.controlledLists[].contextSubjectIds[]", FieldSpec::new("Context subject id", "Kontext-Subjekt-Id")),
    ("dictionary.relationships[].cardinality", FieldSpec::new("Cardinality", "Kardinalität")),
    ("dictionary.relationships[].cardinality.min", FieldSpec::new("Minimum cardinality", "Minimale Kardinalität")),
    ("dictionary.relationships[].cardinality.max", FieldSpec::new("Maximum cardinality", "Maximale Kardinalität")),
    ("geometry.objects.*.ports", FieldSpec::new("Ports", "Anschlüsse")),
    ("geometry.objects.*.ports[]", FieldSpec::new("Port", "Anschluss")),
    ("geometry.objects.*.ports[].id", FieldSpec::new("Port id", "Anschluss-Id")),
    ("geometry.objects.*.ports[].medium", FieldSpec::new("Medium", "Medium")),
    ("geometry.objects.*.ports[].portType", FieldSpec::new("Port type", "Anschlussart")),
    ("geometry.objects.*.ports[].position", FieldSpec::quantity("Position", "Position", UnitId::Metre)),
    ("geometry.objects.*.ports[].position[]", FieldSpec::quantity("Position coordinate", "Positionskoordinate", UnitId::Metre)),
    ("geometry.objects.*.ports[].direction", FieldSpec::new("Direction", "Richtung")),
    ("geometry.objects.*.ports[].direction[]", FieldSpec::new("Direction component", "Richtungskomponente")),
    ("geometry.primitiveRegistry", FieldSpec::new("Primitive registry", "Primitiv-Register")),
    ("geometry.primitiveRegistry[]", FieldSpec::new("Primitive kind", "Primitivart")),
    ("geometry.primitiveRegistry[].id", FieldSpec::new("Primitive id", "Primitiv-Id")),
    ("geometry.primitiveRegistry[].parameters", FieldSpec::new("Parameter names", "Parameternamen")),
    ("geometry.primitiveRegistry[].parameters[]", FieldSpec::new("Parameter name", "Parametername")),
    ("partNumberRule.functionId", FieldSpec::new("Function id", "Funktions-Id")),
    ("partNumberRule.function_id", FieldSpec::new("Function id", "Funktions-Id")),
    ("partNumberInputs.*", FieldSpec::new("Part-number input", "Teilenummern-Eingabe")),
    ("partNumberInputs.*.value", FieldSpec::new("Input value", "Eingabewert")),
    ("selection.constraints[].id", FieldSpec::new("Constraint id", "Einschränkungs-Id")),
    ("selection.constraints[].propertyId", FieldSpec::new("Property id", "Eigenschafts-Id")),

];

/// 🏷️ Exact / `[]` / `*` template lookup for the ISO 16757 catalogue subject.
pub fn iso16757_field_meta(path: &str) -> Option<FieldSpec> {
    if let Some(rest) = path.strip_prefix("catalogue.compositions.") {
        let templated = if let Some((_key, tail)) = rest.split_once('.') {
            format!("catalogue.compositions.*.{tail}")
        } else if rest.contains('[') {
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
