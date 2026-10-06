"""🗂️ S5-TEXT-STDIO: the reviewed input table of `🧪️s5-text-stdio-input-ui.py` — one row per mutation input MEANING of the
stdio, writer, trinity and vcs leaves that carried no `x-semio-ui` (design §6, §22.8). A row names the artifact, the leaf
(several leaves when the input means the same in each), the input and the declaration; the constructor is the judgement:

- `AT` / `IDX` — which existing item a 0-based index addresses (stepper, integer steps, group `target`);
- `POS` — where an inserted item lands (stepper);
- `ENT` — an entity named by its id (reference chip, `role: target`, `ref.kind`); `REFV` — an id that is the value written;
- `KEY` — a key or name the mutation addresses and may create (text, group `target`);
- `TXT` / `MULTI` / `FLAG` / `PICK` / `INT` / `NUM` / `VEC` — the value written, with its unit, step, precision, soft bounds and snaps;
- `REC` — a structured value whose members carry their own declarations; `ADDR` — a structured address;
- `BLOB` — an opaque member (bytes, bulk samples) of a leaf that has real parameters beside it: `hidden`, an input without a row;
- `DISC` — a fixed discriminator, never an input.

A leaf whose whole VALUE is an opaque payload (a whole document, a byte string, bulk samples) is no editor at all: `W` lists
it as withdraw-only (design §22.20, leaf descriptor `"editable": false`) and none of its inputs is declared here. A leaf that
takes no parameter at all is withdraw-only by its shape; the engine marks it without a row.
"""

ROWS = {}
DOCS = {}
WITHDRAW = {}
MEMBERS = {}


def N(artifact, record, members, ui, force=()):
    """🧱️ Completes the declaration of the named members of the shared record `record` (a `$defs` / `definitions` entry) in every
    schema document of `artifact` that defines it: the member's own keys win unless forced."""
    for member in members.split():
        assert (artifact, record, member) not in MEMBERS, (artifact, record, member)
        MEMBERS[(artifact, record, member)] = {"ui": ui, "force": set(force)}


def W(artifact, leaves, why, subset="*"):
    """🚪️ Declares the named leaves withdraw-only: `why` says what their value is."""
    for leaf in leaves.split():
        assert (artifact, subset, leaf) not in WITHDRAW, (artifact, subset, leaf)
        WITHDRAW[(artifact, subset, leaf)] = why


def D(file, pointer, ui):
    """📄️ Declares the member at schema path `pointer` of the shared schema document `file` (a `$ref` target several leaves read)."""
    assert (file, pointer) not in DOCS, (file, pointer)
    DOCS[(file, pointer)] = ui


def R(artifact, leaves, prop, ui, subset="*", nested=None, force=(), drop=(), amend=False):
    """🧾️ Declares `prop` of every named leaf of `artifact` (`subset` `*` = each subset that has the leaf). `nested` declares
    members below it (schema path relative to the input → declaration); `force`/`drop` revise a declaration already there;
    `amend` only completes a declaration that exists (its own keys win unless forced) and never declares a bare input."""
    for leaf in leaves.split():
        key = (artifact, subset, leaf, prop)
        assert key not in ROWS, key
        ROWS[key] = {"ui": ui, "nested": nested or {}, "force": set(force), "drop": set(drop), "amend": amend}


def A(artifact, props, ui=None, force=()):
    """🪛️ Completes the declarations an earlier wave wrote on `props` in every leaf of `artifact`: by default the integer step
    of an index, position, count, size or code (`step: 1`), else the stated facets."""
    for prop in props.split():
        R(artifact, "*", prop, ui if ui is not None else {"step": 1}, force=force, amend=True)


def L(en, de):
    return {"en": en, "de": de}


def AT(en, de, of_en, of_de):
    return {"widget": "stepper", "label": L(en, de), "description": L(f"Index of {of_en}, counted from 0.", f"Index {of_de}, ab 0 gezählt."), "step": 1, "precision": 0, "group": "target"}


def IDX(en, de, d_en, d_de):
    return {"widget": "stepper", "label": L(en, de), "description": L(d_en, d_de), "step": 1, "precision": 0, "group": "target"}


def POS(among_en, among_de, en="Insert position", de="Einfügeposition"):
    return {"widget": "stepper", "label": L(en, de), "description": L(f"Position among {among_en} at which it is inserted, counted from 0.", f"Position unter {among_de}, an der eingefügt wird, ab 0 gezählt."), "step": 1, "precision": 0, "group": "target"}


def ENT(kind, en, de, d_en, d_de, **ref):
    return {"widget": "reference", "role": "target", "label": L(en, de), "description": L(d_en, d_de), "ref": {"kind": kind, **ref}, "group": "target"}


def REFV(kind, en, de, d_en, d_de):
    return {"widget": "reference", "role": "value", "label": L(en, de), "description": L(d_en, d_de), "ref": {"kind": kind}, "group": "value"}


def KEY(en, de, d_en, d_de, widget="text"):
    return {"widget": widget, "label": L(en, de), "description": L(d_en, d_de), "group": "target"}


def TXT(en, de, d_en, d_de):
    return {"widget": "text", "role": "value", "label": L(en, de), "description": L(d_en, d_de), "group": "value"}


def MULTI(en, de, d_en, d_de):
    return {"widget": "multiline", "role": "value", "label": L(en, de), "description": L(d_en, d_de), "group": "value"}


def FLAG(en, de, d_en, d_de):
    return {"widget": "toggle", "role": "value", "label": L(en, de), "description": L(d_en, d_de), "group": "value"}


def PICK(en, de, d_en, d_de, widget="select"):
    return {"widget": widget, "role": "value", "label": L(en, de), "description": L(d_en, d_de), "group": "value"}


def NUM(en, de, d_en, d_de, step, precision, widget="stepper", **facets):
    return {"widget": widget, "role": "value", "label": L(en, de), "description": L(d_en, d_de), "step": step, "precision": precision, **facets, "group": "value"}


def INT(en, de, d_en, d_de, **facets):
    return NUM(en, de, d_en, d_de, 1, 0, **facets)


def VEC(en, de, d_en, d_de, step, precision, **facets):
    return {"widget": "vector", "role": "value", "label": L(en, de), "description": L(d_en, d_de), "step": step, "precision": precision, **facets, "group": "value"}


def REC(en, de, d_en, d_de):
    return {"role": "value", "label": L(en, de), "description": L(d_en, d_de), "group": "value"}


def ADDR(en, de, d_en, d_de, **facets):
    return {"label": L(en, de), "description": L(d_en, d_de), **facets, "group": "target"}


def BLOB(en, de, d_en, d_de):
    return {"widget": "hidden", "role": "value", "label": L(en, de), "description": L(d_en, d_de), "group": "value"}


def DISC(en, de, d_en, d_de):
    return {"role": "discriminator", "label": L(en, de), "description": L(d_en, d_de)}


NOT_INLINE = (" Binary content is not edited inline; withdraw the mutation instead.", " Binärinhalt wird nicht direkt bearbeitet; stattdessen die Mutation zurückziehen.")
BULK = (" Bulk data is not edited inline; withdraw the mutation instead.", " Massendaten werden nicht direkt bearbeitet; stattdessen die Mutation zurückziehen.")
CHILD_PATH = ("Child indices leading from the root element to {0}, each counted from 0.", "Kindindizes vom Wurzelelement bis {0}, jeweils ab 0 gezählt.")


def bytes_of(en, de, d_en, d_de):
    return BLOB(en, de, d_en + NOT_INLINE[0], d_de + NOT_INLINE[1])


def bulk(en, de, d_en, d_de):
    return BLOB(en, de, d_en + BULK[0], d_de + BULK[1])


def child_path(en, de, to_en, to_de):
    return ADDR(en, de, CHILD_PATH[0].format(to_en), CHILD_PATH[1].format(to_de), step=1, precision=0)


#region 🔖️Shared
W("*", "set-snapshot", "the whole document that replaces the current one (an import or a revert to file)")
W("gltf", "snapshot/set", "the whole glTF asset that replaces the current one (an import or a revert to file)")
W("bcf", "set-viewpoint-snapshot", "the PNG bytes of one viewpoint's snapshot image")
W("las", "set-vlr-data", "the raw payload bytes of one variable length record")
W("zip", "set-entry-data", "the uncompressed bytes of one archive member")
W("jpg", "replace-pixels", "the decoded samples of the whole image")
W("wav", "set-data", "the complete sample data of the data chunk")
W("pdf", "set-document-id", "the two byte strings of the trailer's ID entry")
W("semio", "set-sample-data", "the encoded payload bytes of one sample", subset="video")
W("semio", "replace-primitive-geometry", "the bulk vertex and index buffers of one primitive", subset="mesh")
W("rewriting", "edit-before-fixture", "the whole working graph the rule is tried on (nodes, edges and manifest); working-canvas edits are child-lane leaves")
R("*", "patch-snapshot", "patch", {"label": L("Snapshot edit", "Änderung der Momentaufnahme"), "description": L("One path-scoped edit of the snapshot; its value is typed by the snapshot schema at the path.", "Eine pfadbezogene Änderung der Momentaufnahme; ihr Wert ist durch das Schema der Momentaufnahme am Pfad typisiert.")})
#endregion 🔖️Shared

#region 🔖️Las
LAS_POINT = AT("Point", "Punkt", "the point record", "des Punktdatensatzes")
LAS_VLR = AT("Variable length record", "Datensatz variabler Länge (VLR)", "the variable length record", "des Datensatzes variabler Länge")
R("las", "set-point remove-point", "index", LAS_POINT)
R("las", "insert-point", "index", POS("the point records", "den Punktdatensätzen"))
R("las", "set-point", "point", REC("Point record", "Punktdatensatz", "Coordinates, intensity, return numbers, classification and optional GPS time and colour the point takes.", "Koordinaten, Intensität, Rückkehrnummern, Klassifizierung sowie optional GPS-Zeit und Farbe, die der Punkt erhält."))
R("las", "insert-point", "point", REC("Point record", "Punktdatensatz", "The point record that is inserted.", "Der Punktdatensatz, der eingefügt wird."))
R("las", "set-scale-and-offset", "scale", REC("Scale factors", "Skalierungsfaktoren", "X, Y and Z factor that turns the stored integer coordinates into real coordinates.", "X-, Y- und Z-Faktor, der die gespeicherten ganzzahligen Koordinaten in reale Koordinaten umrechnet."))
R("las", "set-scale-and-offset", "offset", REC("Offsets", "Versätze", "X, Y and Z offset added to the scaled coordinates.", "X-, Y- und Z-Versatz, der zu den skalierten Koordinaten addiert wird."))
R("las", "remove-vlr", "index", LAS_VLR)
R("las", "insert-vlr", "index", POS("the variable length records", "den Datensätzen variabler Länge"))
R("las", "set-creation-date", "year", INT("Year", "Jahr", "Four-digit year the file was created in.", "Vierstelliges Jahr, in dem die Datei erstellt wurde.", softMin=1970, softMax=2100))
#endregion 🔖️Las

#region 🔖️Zip
ZIP_ENTRY = ("entry", "Entry", "Eintrag")
R("zip", "remove-entry", "name", ENT(*ZIP_ENTRY, "Name of the archive member that is removed.", "Name des Archivmitglieds, das entfernt wird."))
R("zip", "rename-entry", "name", ENT(*ZIP_ENTRY, "Name of the archive member that is renamed.", "Name des Archivmitglieds, das umbenannt wird."))
R("zip", "rename-entry", "newName", TXT("New name", "Neuer Name", "Name the member carries afterwards, with `/` as the path separator.", "Name, den das Mitglied danach trägt, mit `/` als Pfadtrenner."))
R("zip", "set-archive-comment", "comment", MULTI("Archive comment", "Archivkommentar", "Free text stored at the end of the archive (at most 65,535 bytes).", "Freitext am Ende des Archivs (höchstens 65.535 Bytes)."))
R("zip", "add-stored-entry add-deflated-entry add-entry", "entry", REC("Entry", "Eintrag", "Name, content and metadata of the archive member that is added.", "Name, Inhalt und Metadaten des Archivmitglieds, das hinzugefügt wird."))
R("zip", "add-stored-entry add-deflated-entry add-entry", "before", ENT("entry", "Insert before", "Einfügen vor", "Existing member the new entry is placed before; left out, the entry is appended.", "Vorhandenes Mitglied, vor dem der neue Eintrag platziert wird; ohne Angabe wird er angehängt."))
#endregion 🔖️Zip

#region 🔖️Mp4
VIDEO_WIDTHS = [640, 1280, 1920, 2560, 3840]
VIDEO_HEIGHTS = [360, 480, 720, 1080, 1440, 2160]
R("mp4", "insert-track", "index", POS("the tracks", "den Spuren"))
R("mp4", "remove-track", "index", AT("Track", "Spur", "the track", "der Spur"))
R("mp4", "set-sample-sync remove-sample", "index", AT("Sample", "Sample", "the sample within the track", "des Samples innerhalb der Spur"))
R("mp4", "insert-sample", "index", POS("the samples of the track", "den Samples der Spur"))
R("mp4", "set-track-dimensions", "width", INT("Width", "Breite", "Presentation width of the track's visual content.", "Darstellungsbreite des Bildinhalts der Spur.", unit="px", softMax=7680, snaps=VIDEO_WIDTHS))
R("mp4", "set-track-dimensions", "height", INT("Height", "Höhe", "Presentation height of the track's visual content.", "Darstellungshöhe des Bildinhalts der Spur.", unit="px", softMax=4320, snaps=VIDEO_HEIGHTS))
#endregion 🔖️Mp4

#region 🔖️Svg
SVG_ELEMENT = child_path("Element path", "Elementpfad", "the element", "zum Element")
SVG_PARENT = child_path("Parent path", "Elternpfad", "the parent element", "zum Elternelement")
SVG_NODE = REC("Node", "Knoten", "The element (with its attributes and children) or the character data that is inserted.", "Das Element (mit Attributen und Kindknoten) oder die Zeichendaten, die eingefügt werden.")
SVG_CHILD_POS = POS("the parent's children", "den Kindknoten des Elternelements", "Child position", "Kindposition")
R("svg", "set-text", "path", child_path("Text path", "Textpfad", "the text node", "zum Textknoten"))
R("svg", "set-text", "text", MULTI("Text", "Text", "Character data the text node holds afterwards.", "Zeichendaten, die der Textknoten danach enthält."))
R("svg", "insert-tiny-element insert-basic-element insert-element remove-element", "parent", SVG_PARENT)
R("svg", "insert-tiny-element insert-basic-element insert-element", "index", SVG_CHILD_POS)
R("svg", "insert-tiny-element insert-basic-element insert-element insert-clip-path-shape", "node", SVG_NODE)
R("svg", "remove-element", "index", AT("Child", "Kindknoten", "the child among the parent's children", "des Kindknotens unter den Kindern des Elternelements"))
R("svg", "set-tiny-attribute set-basic-attribute set-attribute set-transform set-view-box set-clip-path-reference set-element-name", "path", SVG_ELEMENT)
R("svg", "set-tiny-attribute set-basic-attribute set-attribute", "name", KEY("Attribute", "Attribut", "Qualified name of the attribute that is set or removed.", "Qualifizierter Name des Attributs, das gesetzt oder entfernt wird."))
R("svg", "set-tiny-attribute set-basic-attribute set-attribute", "value", TXT("Value", "Wert", "Attribute value; cleared, the attribute is removed.", "Attributwert; geleert wird das Attribut entfernt."))
R("svg", "set-transform", "transform", REC("Transform", "Transformation", "Ordered transform operations (matrix, translate, scale, rotate, skewX, skewY); cleared, the attribute is removed.", "Geordnete Transformationsoperationen (matrix, translate, scale, rotate, skewX, skewY); geleert wird das Attribut entfernt."))
R("svg", "stamp-base-profile", "version", TXT("SVG version", "SVG-Version", "Value of the `version` attribute stamped with the profile, for example 1.1; cleared, the attribute is removed.", "Wert des Attributs `version`, das mit dem Profil gesetzt wird, zum Beispiel 1.1; geleert wird das Attribut entfernt."))
R("svg", "insert-clip-path-shape", "index", POS("the shapes of the clipping path", "den Formen des Beschneidungspfads"))
R("svg", "set-doctype", "doctype", REC("Document type", "Dokumenttyp", "The DOCTYPE declaration; cleared, the document carries none.", "Die DOCTYPE-Deklaration; geleert trägt das Dokument keine."))
R("svg", "set-element-name", "name", TXT("Element name", "Elementname", "Qualified name the element carries afterwards.", "Qualifizierter Name, den das Element danach trägt."))
#endregion 🔖️Svg

#region 🔖️Step
STEP_ENTITY = ("entity", "Entity", "Entität")
STEP_REP = ("entity", "Shape representation", "Formrepräsentation", "Instance name (#id) of the SHAPE_REPRESENTATION entity.", "Instanzname (#id) der Entität SHAPE_REPRESENTATION.")
R("step", "set-file-schema", "schemas", REC("Schema identifiers", "Schemakennungen", "Entries of FILE_SCHEMA in the header, for example AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }.", "Einträge von FILE_SCHEMA im Kopfteil, zum Beispiel AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }."))
R("step", "remove-shape-representation demote-shape-representation set-shape-representation", "id", ENT(*STEP_REP))
R("step", "set-shape-representation", "representation", REC("Representation", "Repräsentation", "Type, name, items and context the shape representation carries; cleared, it is removed.", "Typ, Name, Elemente und Kontext der Formrepräsentation; geleert wird sie entfernt."))
R("step", "set-product-identity", "identity", REC("Product identity", "Produktidentität", "Ids and names of PRODUCT, PRODUCT_DEFINITION_FORMATION and PRODUCT_DEFINITION; cleared, the chain is removed.", "Kennungen und Namen von PRODUCT, PRODUCT_DEFINITION_FORMATION und PRODUCT_DEFINITION; geleert wird die Kette entfernt."))
R("step", "set-entity-name insert-entity-arg remove-entity-arg set-entity-arg remove-entity", "id", ENT(*STEP_ENTITY, "Instance name (#id) of the entity.", "Instanzname (#id) der Entität."))
R("step", "set-entity-name", "name", TXT("Entity type", "Entitätstyp", "Keyword of the entity, for example CARTESIAN_POINT.", "Schlüsselwort der Entität, zum Beispiel CARTESIAN_POINT."))
R("step", "insert-entity-arg set-entity-arg", "value", REC("Argument value", "Argumentwert", "Typed Part 21 value of the argument: unset, derived, integer, real, string, enumeration, reference, aggregate or typed value.", "Typisierter Part-21-Wert des Arguments: nicht gesetzt, abgeleitet, Ganzzahl, Gleitkommazahl, Zeichenkette, Aufzählung, Referenz, Aggregat oder typisierter Wert."))
R("step", "insert-entity", "index", POS("the entities of the DATA section", "den Entitäten des DATA-Abschnitts"))
R("step", "insert-entity", "entity", REC("Entity", "Entität", "Instance name, type and arguments of the entity that is inserted.", "Instanzname, Typ und Argumente der Entität, die eingefügt wird."))
#endregion 🔖️Step

#region 🔖️Tsv
R("tsv", "insert-row", "index", POS("the records", "den Datensätzen"))
R("tsv", "insert-row", "row", REC("Fields", "Felder", "Field values of the inserted record, in column order.", "Feldwerte des eingefügten Datensatzes in Spaltenreihenfolge."))
R("tsv", "remove-row", "index", AT("Record", "Datensatz", "the record (line)", "des Datensatzes (Zeile)"))
R("tsv", "set-cell", "value", TXT("Field value", "Feldwert", "Text the field holds afterwards; a tab or a line break cannot be part of a TSV field.", "Text, den das Feld danach enthält; Tabulator und Zeilenumbruch können nicht Teil eines TSV-Felds sein."))
#endregion 🔖️Tsv

#region 🔖️Ooxml
NAMESPACE = ("Namespace URI the conformance class prescribes for {0}.", "Namensraum-URI, die die Konformitätsklasse für {0} vorschreibt.")
PART = ("part", "Part", "Paketteil")
CONFORMANCE = TXT("Conformance class", "Konformitätsklasse", "Value of the `conformance` attribute on the root element: `strict` or `transitional`.", "Wert des Attributs `conformance` am Wurzelelement: `strict` oder `transitional`.")
VML_MARKUP = MULTI("VML markup", "VML-Markup", "XML source of the legacy VML drawing part.", "XML-Quelltext des veralteten VML-Zeichnungsteils.")
VML_NAME = KEY("Part name", "Name des Paketteils", "Package part name the VML drawing is stored under.", "Name des Paketteils, unter dem die VML-Zeichnung gespeichert wird.")
VML_PART = ENT(*PART, "Package part name of the VML drawing that is removed.", "Name des Paketteils der VML-Zeichnung, die entfernt wird.")
R("xlsx", "set-conformance-attribute", "value", CONFORMANCE)
R("xlsx", "set-main-namespace", "namespace", TXT("Main namespace", "Hauptnamensraum", NAMESPACE[0].format("the SpreadsheetML main vocabulary"), NAMESPACE[1].format("das SpreadsheetML-Hauptvokabular")))
R("xlsx", "set-relationships-namespace", "namespace", TXT("Relationships namespace", "Beziehungsnamensraum", NAMESPACE[0].format("document relationships"), NAMESPACE[1].format("Dokumentbeziehungen")))
R("xlsx", "set-worksheet-content-type", "path", ENT(*PART, "Package part name of the worksheet, for example /xl/worksheets/sheet1.xml.", "Name des Paketteils des Arbeitsblatts, zum Beispiel /xl/worksheets/sheet1.xml."))
R("xlsx", "set-worksheet-content-type", "content_type", TXT("Content type", "Inhaltstyp", "Media type registered for the part in [Content_Types].xml.", "Medientyp, der für den Paketteil in [Content_Types].xml eingetragen ist."))
R("xlsx", "insert-vml-part", "path", VML_NAME)
R("xlsx", "insert-vml-part", "markup", VML_MARKUP)
R("xlsx", "remove-vml-part", "path", VML_PART)
R("xlsx", "set-cell remove-cell", "address", ADDR("Cell", "Zelle", "Worksheet and reference (for example B7) of the cell.", "Arbeitsblatt und Bezug (zum Beispiel B7) der Zelle."))
R("xlsx", "insert-cell", "address", ADDR("Cell", "Zelle", "Worksheet and reference of the vacant cell that is filled.", "Arbeitsblatt und Bezug der leeren Zelle, die gefüllt wird."))
R("xlsx", "set-cell insert-cell", "value", REC("Cell value", "Zellwert", "Typed content of the cell: number, shared string, inline string, boolean, error, formula or empty.", "Typisierter Inhalt der Zelle: Zahl, gemeinsame Zeichenfolge, Inline-Zeichenfolge, Wahrheitswert, Fehler, Formel oder leer."))
R("xlsx", "insert-sheet", "sheet", REC("Sheet", "Arbeitsblatt", "Name and cells of the worksheet that is inserted.", "Name und Zellen des Arbeitsblatts, das eingefügt wird."))
R("xlsx", "remove-sheet rename-sheet", "name", ENT("sheet", "Sheet", "Arbeitsblatt", "Name of the worksheet.", "Name des Arbeitsblatts."))
R("xlsx", "rename-sheet", "newName", TXT("New name", "Neuer Name", "Name the worksheet carries afterwards (at most 31 characters, none of \\ / ? * [ ]).", "Name, den das Arbeitsblatt danach trägt (höchstens 31 Zeichen, keines von \\ / ? * [ ])."))
R("xlsx", "remove-shared-string set-shared-string", "index", AT("Shared string", "Gemeinsame Zeichenfolge", "the entry in the shared string table", "des Eintrags in der Tabelle gemeinsamer Zeichenfolgen"))
R("xlsx", "insert-shared-string", "value", MULTI("Text", "Text", "Text of the shared string appended to the table.", "Text der gemeinsamen Zeichenfolge, die an die Tabelle angehängt wird."))
R("xlsx", "set-shared-string", "value", MULTI("Text", "Text", "Text the shared string holds afterwards; every cell that references it shows it.", "Text, den die gemeinsame Zeichenfolge danach enthält; jede Zelle, die sie referenziert, zeigt ihn an."))
R("pptx", "set-conformance-attribute", "value", CONFORMANCE)
R("pptx", "set-drawing-namespace", "namespace", TXT("Drawing namespace", "Zeichnungsnamensraum", NAMESPACE[0].format("the DrawingML main vocabulary"), NAMESPACE[1].format("das DrawingML-Hauptvokabular")))
R("pptx", "set-main-namespace", "namespace", TXT("Main namespace", "Hauptnamensraum", NAMESPACE[0].format("the PresentationML main vocabulary"), NAMESPACE[1].format("das PresentationML-Hauptvokabular")))
R("pptx", "set-relationship-base", "base", TXT("Relationship base", "Beziehungsbasis", "Base URI of the relationship types of the conformance class.", "Basis-URI der Beziehungstypen der Konformitätsklasse."))
R("pptx", "insert-alternate-content remove-alternate-content", "path", ENT(*PART, "Package part name of the part that carries the mc:AlternateContent block.", "Name des Paketteils, der den Block mc:AlternateContent trägt."))
R("pptx", "insert-vml-part", "path", VML_NAME)
R("pptx", "insert-vml-part", "markup", VML_MARKUP)
R("pptx", "remove-vml-part", "path", VML_PART)
#endregion 🔖️Ooxml

#region 🔖️Pdf
PDF_PAGE = AT("Page", "Seite", "the page", "der Seite")
PDF_PAGE_POS = POS("the pages", "den Seiten")
PDF_WIDTH = NUM("Page width", "Seitenbreite", "Width of the page in PDF points (1/72 inch); the snaps are A5, A4, Letter, A3 and A2.", "Breite der Seite in PDF-Punkten (1/72 Zoll); die Rastpunkte sind A5, A4, Letter, A3 und A2.", 1, 2, unit="pt", softMin=72, softMax=3370, snaps=[419.53, 595.28, 612, 841.89, 1190.55])
PDF_HEIGHT = NUM("Page height", "Seitenhöhe", "Height of the page in PDF points (1/72 inch); the snaps are A5, Letter, A4, Legal, A3 and A2.", "Höhe der Seite in PDF-Punkten (1/72 Zoll); die Rastpunkte sind A5, Letter, A4, Legal, A3 und A2.", 1, 2, unit="pt", softMin=72, softMax=4768, snaps=[595.28, 792, 841.89, 1008, 1190.55, 1683.78])
PDF_PAGE_TEXT = MULTI("Page text", "Seitentext", "Text the page shows afterwards.", "Text, den die Seite danach zeigt.")
PDF_OPS = REC("Content operators", "Inhaltsoperatoren", "Content stream operators with their operands, in painting order.", "Operatoren des Inhaltsstroms mit ihren Operanden in Zeichenreihenfolge.")
PDF_OBJECT = ("Object and generation number of {0}.", "Objekt- und Generationsnummer {0}.")
PDF_VALUE = REC("Value", "Wert", "The PDF object stored under the key: null, boolean, number, name, string, array, dictionary, stream or reference.", "Das unter dem Schlüssel gespeicherte PDF-Objekt: null, Wahrheitswert, Zahl, Name, Zeichenkette, Array, Wörterbuch, Datenstrom oder Referenz.")
PDF_TITLE = TXT("Title", "Titel", "Document title written to the Info dictionary and shown in the viewer's title bar.", "Dokumenttitel, der in das Info-Wörterbuch geschrieben und in der Titelleiste des Betrachters angezeigt wird.")


def pdf_resource(kind, en, de, of_en, of_de):
    return ENT(kind, en, de, f"Resource name of {of_en} that is removed.", f"Ressourcenname {of_de}, die entfernt wird.")


R("pdf", "set-page-size resize-page", "width", PDF_WIDTH)
R("pdf", "set-page-size resize-page", "height", PDF_HEIGHT)
R("pdf", "set-page-text replace-page-text", "text", PDF_PAGE_TEXT)
R("pdf", "replace-page-text resize-page remove-page set-page-crop-box set-page-content append-page-content set-page-user-unit set-page-media-box set-page-box set-page-rotation insert-annotation remove-annotation set-annotation replace-content insert-content remove-content", "index", PDF_PAGE)
R("pdf", "insert-page", "index", PDF_PAGE_POS)
R("pdf", "insert-page", "page", REC("Page", "Seite", "Size and content of the page that is inserted.", "Größe und Inhalt der Seite, die eingefügt wird."))
R("pdf", "move-page", "from", AT("Page", "Seite", "the page that moves", "der Seite, die verschoben wird"))
R("pdf", "move-page", "to", IDX("Destination", "Zielposition", "Position the page holds afterwards, counted from 0.", "Position, die die Seite danach einnimmt, ab 0 gezählt."))
R("pdf", "set-info-title", "title", PDF_TITLE)
R("pdf", "set-info-author", "author", TXT("Author", "Autor", "Author written to the Info dictionary.", "Autor, der in das Info-Wörterbuch geschrieben wird."))
R("pdf", "embed-font-file remove-font-file", "descriptorOrdinal", AT("Font descriptor", "Schriftdeskriptor", "the font descriptor in document order", "des Schriftdeskriptors in Dokumentreihenfolge"))
R("pdf", "embed-font-file", "key", KEY("Font file key", "Schlüssel der Schriftdatei", "Descriptor entry that carries the font program: FontFile, FontFile2 or FontFile3.", "Deskriptoreintrag, der das Schriftprogramm trägt: FontFile, FontFile2 oder FontFile3."))
R("pdf", "embed-font-file", "program", ADDR("Font program", "Schriftprogramm", PDF_OBJECT[0].format("the stream that holds the embedded font program"), PDF_OBJECT[1].format("des Datenstroms mit dem eingebetteten Schriftprogramm")))
R("pdf", "remove-signature-field", "name", ENT("field", "Signature field", "Signaturfeld", "Partial name of the signature field that is removed.", "Teilname des Signaturfelds, das entfernt wird."))
R("pdf", "insert-signature-field", "name", TXT("Field name", "Feldname", "Partial name the new signature field carries.", "Teilname, den das neue Signaturfeld trägt."))
R("pdf", "insert-javascript-action", "script", MULTI("JavaScript", "JavaScript", "Source of the script the action runs.", "Quelltext des Skripts, das die Aktion ausführt."))
R("pdf", "remove-javascript-action", "script", KEY("JavaScript", "JavaScript", "Source of the JavaScript action that is removed; it identifies the action.", "Quelltext der JavaScript-Aktion, die entfernt wird; er identifiziert die Aktion.", widget="multiline"))
R("pdf", "insert-launch-action", "target", TXT("Launch target", "Startziel", "File or application the Launch action opens.", "Datei oder Anwendung, die die Launch-Aktion öffnet."))
R("pdf", "remove-launch-action", "target", KEY("Launch target", "Startziel", "Target of the Launch action that is removed; it identifies the action.", "Ziel der Launch-Aktion, die entfernt wird; es identifiziert die Aktion."))
R("pdf", "insert-media-annotation", "subtype", TXT("Annotation subtype", "Anmerkungsuntertyp", "Subtype of the media annotation: Movie, Sound, Screen or 3D.", "Untertyp der Medienanmerkung: Movie, Sound, Screen oder 3D."))
R("pdf", "insert-media-annotation", "title", TXT("Title", "Titel", "Title (T entry) of the media annotation.", "Titel (Eintrag T) der Medienanmerkung."))
R("pdf", "remove-media-annotation", "subtype", KEY("Annotation subtype", "Anmerkungsuntertyp", "Subtype of the media annotation that is removed: Movie, Sound, Screen or 3D.", "Untertyp der Medienanmerkung, die entfernt wird: Movie, Sound, Screen oder 3D."))
R("pdf", "remove-media-annotation", "title", KEY("Title", "Titel", "Title (T entry) of the media annotation that is removed.", "Titel (Eintrag T) der Medienanmerkung, die entfernt wird."))
R("pdf", "set-output-intent", "identifier", TXT("Output condition identifier", "Kennung der Ausgabebedingung", "OutputConditionIdentifier of the output intent, for example FOGRA39 or CGATS TR 001.", "OutputConditionIdentifier der Ausgabebedingung, zum Beispiel FOGRA39 oder CGATS TR 001."))
R("pdf", "insert-encryption-dictionary", "version", INT("Algorithm version (V)", "Algorithmusversion (V)", "V entry of the encryption dictionary: 1, 2, 4 or 5.", "Eintrag V des Verschlüsselungswörterbuchs: 1, 2, 4 oder 5.", softMin=0, softMax=5))
R("pdf", "insert-encryption-dictionary", "revision", INT("Revision (R)", "Revision (R)", "R entry of the standard security handler: 2 to 6.", "Eintrag R des Standard-Sicherheitshandlers: 2 bis 6.", softMin=2, softMax=6))
R("pdf", "remove-encryption-dictionary", "version", IDX("Algorithm version (V)", "Algorithmusversion (V)", "V entry of the encryption dictionary that is removed.", "Eintrag V des Verschlüsselungswörterbuchs, das entfernt wird."))
R("pdf", "remove-encryption-dictionary", "revision", IDX("Revision (R)", "Revision (R)", "R entry of the encryption dictionary that is removed.", "Eintrag R des Verschlüsselungswörterbuchs, das entfernt wird."))
R("pdf", "remove-trim-box set-trim-box", "pageIndex", PDF_PAGE)
R("pdf", "remove-af-relationship set-af-relationship", "fileName", ENT("embeddedFile", "Embedded file", "Eingebettete Datei", "File name of the embedded file whose AFRelationship changes.", "Dateiname der eingebetteten Datei, deren AFRelationship sich ändert."))
R("pdf", "remove-embedded-file", "fileName", ENT("embeddedFile", "Embedded file", "Eingebettete Datei", "File name of the embedded file that is removed.", "Dateiname der eingebetteten Datei, die entfernt wird."))
R("pdf", "insert-embedded-file", "fileName", TXT("File name", "Dateiname", "Name the embedded file is listed under.", "Name, unter dem die eingebettete Datei geführt wird."))
R("pdf", "set-af-relationship", "relationship", TXT("Relationship", "Beziehung", "AFRelationship of the file to the document: Source, Data, Alternative, Supplement or Unspecified.", "AFRelationship der Datei zum Dokument: Source, Data, Alternative, Supplement oder Unspecified."))
R("pdf", "set-page-content append-page-content insert-content", "content", PDF_OPS)
R("pdf", "remove-font", "id", pdf_resource("font", "Font", "Schrift", "the font", "der Schrift"))
R("pdf", "remove-shading", "id", pdf_resource("shading", "Shading", "Schattierung", "the shading", "der Schattierung"))
R("pdf", "remove-image", "id", pdf_resource("image", "Image", "Bild", "the image XObject", "des Bild-XObjects"))
R("pdf", "remove-ext-g-state", "id", pdf_resource("extGState", "Graphics state", "Grafikzustand", "the graphics state parameter dictionary (ExtGState)", "des Grafikzustands-Parameterwörterbuchs (ExtGState)"))
R("pdf", "remove-form", "id", pdf_resource("form", "Form XObject", "Formular-XObject", "the form XObject", "des Formular-XObjects"))
R("pdf", "remove-pattern", "id", pdf_resource("pattern", "Pattern", "Muster", "the pattern", "des Musters"))
R("pdf", "remove-embedded-file", "id", ENT("embeddedFile", "Embedded file", "Eingebettete Datei", "Id of the embedded file that is removed.", "Kennung der eingebetteten Datei, die entfernt wird."))
R("pdf", "remove-color-space", "name", pdf_resource("colorSpace", "Color space", "Farbraum", "the color space", "des Farbraums"))
R("pdf", "remove-named-destination", "name", ENT("namedDestination", "Named destination", "Benanntes Ziel", "Name of the destination that is removed.", "Name des Ziels, das entfernt wird."))
R("pdf", "remove-properties", "name", pdf_resource("properties", "Property list", "Eigenschaftsliste", "the property list", "der Eigenschaftsliste"))
R("pdf", "set-image", "image", REC("Image", "Bild", "Resource name, dimensions, colour space and samples of the image XObject.", "Ressourcenname, Abmessungen, Farbraum und Abtastwerte des Bild-XObjects."))
R("pdf", "set-properties", "properties", REC("Property list", "Eigenschaftsliste", "Name and entries of the property list marked content refers to.", "Name und Einträge der Eigenschaftsliste, auf die markierter Inhalt verweist."))
R("pdf", "set-optional-content", "content", REC("Optional content", "Optionaler Inhalt", "Optional content groups and their default configuration; cleared, the document has none.", "Gruppen optionalen Inhalts und ihre Standardkonfiguration; geleert hat das Dokument keine."))
R("pdf", "insert-annotation", "at", POS("the page's annotations", "den Anmerkungen der Seite"))
R("pdf", "remove-annotation set-annotation", "at", AT("Annotation", "Anmerkung", "the annotation on the page", "der Anmerkung auf der Seite"))
R("pdf", "insert-object", "id", ADDR("Object", "Objekt", PDF_OBJECT[0].format("the new indirect object"), PDF_OBJECT[1].format("des neuen indirekten Objekts")))
R("pdf", "set-object-value remove-object", "id", ADDR("Object", "Objekt", PDF_OBJECT[0].format("the indirect object"), PDF_OBJECT[1].format("des indirekten Objekts")))
R("pdf", "set-dict-entry remove-dict-entry", "id", ADDR("Object", "Objekt", PDF_OBJECT[0].format("the indirect object that holds the dictionary"), PDF_OBJECT[1].format("des indirekten Objekts, das das Wörterbuch enthält")))
R("pdf", "insert-object set-object-value", "value", REC("Object value", "Objektwert", "The PDF object stored under the number: null, boolean, number, name, string, array, dictionary, stream or reference.", "Das unter der Nummer gespeicherte PDF-Objekt: null, Wahrheitswert, Zahl, Name, Zeichenkette, Array, Wörterbuch, Datenstrom oder Referenz."))
R("pdf", "replace-content", "at", AT("Operator", "Operator", "the operator in the page's content stream", "des Operators im Inhaltsstrom der Seite"))
R("pdf", "insert-content", "at", POS("the operators of the page's content stream", "den Operatoren des Inhaltsstroms der Seite"))
R("pdf", "remove-content", "at", AT("First operator", "Erster Operator", "the first operator that is removed", "des ersten Operators, der entfernt wird"))
R("pdf", "remove-content", "count", INT("Operator count", "Anzahl Operatoren", "Number of consecutive operators that are removed.", "Anzahl aufeinanderfolgender Operatoren, die entfernt werden.", softMin=1))
R("pdf", "set-page-rotation", "rotation", NUM("Rotation", "Drehung", "Clockwise rotation of the page when it is displayed, a multiple of 90°.", "Drehung der Seite im Uhrzeigersinn bei der Anzeige, ein Vielfaches von 90°.", 90, 0, widget="dial", unit="°", softMin=0, softMax=270, snaps=[0, 90, 180, 270]))
R("pdf", "set-dict-entry remove-dict-entry", "path", ADDR("Path", "Pfad", "Keys and indices leading from the object to the nested dictionary; empty for the object's own dictionary.", "Schlüssel und Indizes vom Objekt zum verschachtelten Wörterbuch; leer für das Wörterbuch des Objekts selbst."))
R("pdf", "set-dict-entry remove-dict-entry", "key", KEY("Key", "Schlüssel", "Name of the dictionary entry.", "Name des Wörterbucheintrags."))
R("pdf", "set-catalog-entry remove-catalog-entry", "key", KEY("Key", "Schlüssel", "Name of the entry in the document catalog.", "Name des Eintrags im Dokumentkatalog."))
R("pdf", "set-trailer-entry remove-trailer-entry", "key", KEY("Key", "Schlüssel", "Name of the entry in the file trailer.", "Name des Eintrags im Dateitrailer."))
R("pdf", "set-dict-entry set-catalog-entry set-trailer-entry", "value", PDF_VALUE)
R("pdf", "set-open-action", "action", REC("Open action", "Öffnen-Aktion", "Destination or action performed when the document is opened; cleared, there is none.", "Ziel oder Aktion beim Öffnen des Dokuments; geleert gibt es keine."))
#endregion 🔖️Pdf

#region 🔖️Markdown
MD_PATH = ADDR("Container path", "Containerpfad", "Steps from the document into the nested block container (a block quote or a list item); empty for the top level.", "Schritte vom Dokument in den verschachtelten Blockcontainer (Blockzitat oder Listenelement); leer für die oberste Ebene.")
MD_BLOCK = AT("Block", "Block", "the block within its container", "des Blocks in seinem Container")
R("md", "set-inlines insert-block remove-block replace-block", "path", MD_PATH)
R("md", "set-inlines remove-block replace-block", "index", MD_BLOCK)
R("md", "insert-block", "index", POS("the blocks of the container", "den Blöcken des Containers"))
R("md", "insert-block", "block", REC("Block", "Block", "The block that is inserted: heading, paragraph, list, code block, block quote, thematic break or raw HTML.", "Der Block, der eingefügt wird: Überschrift, Absatz, Liste, Codeblock, Blockzitat, Trennlinie oder rohes HTML."))
R("md", "replace-block", "block", REC("Block", "Block", "The block that takes the place of the addressed one.", "Der Block, der an die Stelle des adressierten tritt."))
#endregion 🔖️Markdown

#region 🔖️Xml
XML_PARENT = child_path("Parent path", "Elternpfad", "the parent element", "zum Elternelement")
R("xml", "set-text", "path", child_path("Text path", "Textpfad", "the text node", "zum Textknoten"))
R("xml", "set-text", "text", MULTI("Text", "Text", "Character data the text node holds afterwards.", "Zeichendaten, die der Textknoten danach enthält."))
R("xml", "rename-document-element", "name", TXT("Element name", "Elementname", "Qualified name of the document element; the DOCTYPE name follows it.", "Qualifizierter Name des Dokumentelements; der DOCTYPE-Name folgt ihm."))
R("xml", "declare-entity", "index", POS("the declarations of the internal subset", "den Deklarationen der internen Teilmenge"))
R("xml", "declare-entity", "name", TXT("Entity name", "Entitätsname", "Name the entity is referenced by (&name;).", "Name, über den die Entität referenziert wird (&name;)."))
R("xml", "declare-entity", "value", TXT("Replacement text", "Ersetzungstext", "Literal value the entity expands to.", "Literalwert, zu dem die Entität expandiert."))
R("xml", "set-attribute", "path", child_path("Element path", "Elementpfad", "the element", "zum Element"))
R("xml", "set-attribute", "name", KEY("Attribute", "Attribut", "Qualified name of the attribute that is set or removed.", "Qualifizierter Name des Attributs, das gesetzt oder entfernt wird."))
R("xml", "set-attribute", "value", TXT("Value", "Wert", "Attribute value; cleared, the attribute is removed.", "Attributwert; geleert wird das Attribut entfernt."))
R("xml", "set-doctype", "doctype", REC("Document type", "Dokumenttyp", "The DOCTYPE declaration; cleared, the document carries none.", "Die DOCTYPE-Deklaration; geleert trägt das Dokument keine."))
R("xml", "insert-element remove-element", "path", XML_PARENT)
R("xml", "insert-element", "index", POS("the parent's children", "den Kindknoten des Elternelements", "Child position", "Kindposition"))
R("xml", "insert-element", "node", REC("Node", "Knoten", "The element (with its attributes and children), text, CDATA section, comment or processing instruction that is inserted.", "Das Element (mit Attributen und Kindknoten), der Text, der CDATA-Abschnitt, der Kommentar oder die Verarbeitungsanweisung, die eingefügt wird."))
R("xml", "remove-element", "index", AT("Child", "Kindknoten", "the child among the parent's children", "des Kindknotens unter den Kindern des Elternelements"))
#endregion 🔖️Xml

#region 🔖️Jpg
JPG_COMPONENT = ENT("component", "Component", "Komponente", "Component identifier (Ci) in the frame header.", "Komponentenkennung (Ci) im Frame-Header.")
JPG_HUFFMAN = REC("Huffman table", "Huffman-Tabelle", "Class, destination id, the 16 code-length counts and the symbol values of the table.", "Klasse, Zielkennung, die 16 Codelängenzähler und die Symbolwerte der Tabelle.")
JPG_HUFFMAN_KEY = ADDR("Table", "Tabelle", "Class (DC or AC) and destination id of the Huffman table that is removed.", "Klasse (DC oder AC) und Zielkennung der Huffman-Tabelle, die entfernt wird.")
R("jpg", "remove-frame-component set-component-sampling", "id", JPG_COMPONENT)
R("jpg", "insert-huffman-table", "index", POS("the Huffman tables", "den Huffman-Tabellen"))
R("jpg", "insert-huffman-table replace-huffman", "table", JPG_HUFFMAN)
R("jpg", "insert-frame-component", "index", POS("the components of the frame", "den Komponenten des Frames"))
R("jpg", "remove-huffman-table remove-huffman", "key", JPG_HUFFMAN_KEY)
R("jpg", "replace-quant", "table", REC("Quantization table", "Quantisierungstabelle", "Destination id, precision and the 64 quantization values in zig-zag order.", "Zielkennung, Genauigkeit und die 64 Quantisierungswerte in Zickzack-Reihenfolge."))
R("jpg", "insert-other", "index", POS("the other segments", "den sonstigen Segmenten"))
R("jpg", "remove-other", "index", AT("Segment", "Segment", "the other segment", "des sonstigen Segments"))
R("jpg", "remove-quant", "id", ENT("quantizationTable", "Quantization table", "Quantisierungstabelle", "Destination id (Tq) of the table that is removed.", "Zielkennung (Tq) der Tabelle, die entfernt wird."))
R("jpg", "change-jfif", "version", REC("JFIF version", "JFIF-Version", "Major and minor version of the JFIF header, for example 1 and 2 for 1.02.", "Haupt- und Nebenversion des JFIF-Headers, zum Beispiel 1 und 2 für 1.02."))
#endregion 🔖️Jpg

#region 🔖️Wav
R("wav", "patch-data", "index", IDX("Start sample", "Start-Sample", "Index of the first sample the patch replaces, counted from 0.", "Index des ersten Samples, das die Änderung ersetzt, ab 0 gezählt."))
R("wav", "patch-data", "data", bulk("Inserted samples", "Eingefügte Samples", "Samples written at the start position.", "Samples, die an der Startposition geschrieben werden."))
#endregion 🔖️Wav

#region 🔖️Txt
TXT_LINE = AT("Line", "Zeile", "the line", "der Zeile")
TXT_TEXT = TXT("Line text", "Zeilentext", "Text of the line without its line break.", "Text der Zeile ohne ihren Zeilenumbruch.")
R("txt", "set-trailing-newline", "value", FLAG("Trailing newline", "Abschließender Zeilenumbruch", "Whether the file ends with a line break.", "Ob die Datei mit einem Zeilenumbruch endet."))
R("txt", "set-line remove-line", "index", TXT_LINE)
R("txt", "insert-line", "index", POS("the lines", "den Zeilen"))
R("txt", "set-line insert-line", "text", TXT_TEXT)
#endregion 🔖️Txt

#region 🔖️Stl
STL_TRIANGLE = AT("Triangle", "Dreieck", "the triangle (facet)", "des Dreiecks (Facette)")
R("stl", "insert-triangle", "index", POS("the triangles", "den Dreiecken"))
R("stl", "remove-triangle set-triangle-vertices set-triangle-normal", "index", STL_TRIANGLE)
R("stl", "set-solid-name", "name", TXT("Solid name", "Körpername", "Name written after the `solid` keyword.", "Name, der nach dem Schlüsselwort `solid` steht."))
R("stl", "set-triangle-normal", "normal", VEC("Normal", "Normale", "Facet normal as x, y, z; a unit vector that points out of the solid.", "Facettennormale als x, y, z; ein Einheitsvektor, der aus dem Körper heraus zeigt.", 0.01, 6))
#endregion 🔖️Stl

#region 🔖️Tiff
TIFF_TAG = ("tag", "Tag", "Tag")
R("tiff", "replace-tag", "tag", ENT(*TIFF_TAG, "Numeric TIFF tag whose values are replaced, for example 256 (ImageWidth) or 259 (Compression).", "Numerisches TIFF-Tag, dessen Werte ersetzt werden, zum Beispiel 256 (ImageWidth) oder 259 (Compression)."))
R("tiff", "remove-tag", "tag", ENT(*TIFF_TAG, "Numeric TIFF tag that is removed from the directory.", "Numerisches TIFF-Tag, das aus dem Verzeichnis entfernt wird."))
R("tiff", "replace-tag", "values", REC("Values", "Werte", "Field type and values the directory entry of the tag holds.", "Feldtyp und Werte, die der Verzeichniseintrag des Tags enthält."))
R("tiff", "remove-ifd", "index", AT("Image file directory", "Bildverzeichnis (IFD)", "the image file directory", "des Bildverzeichnisses"))
R("tiff", "insert-ifd", "index", POS("the image file directories", "den Bildverzeichnissen"))
#endregion 🔖️Tiff

#region 🔖️Obj
OBJ_VERTEX = AT("Vertex", "Vertex", "the vertex (`v` statement)", "des Vertex (`v`-Anweisung)")
OBJ_NORMAL = AT("Normal", "Normale", "the vertex normal (`vn` statement)", "der Vertexnormale (`vn`-Anweisung)")
OBJ_TEXCOORD = AT("Texture coordinate", "Texturkoordinate", "the texture coordinate (`vt` statement)", "der Texturkoordinate (`vt`-Anweisung)")
OBJ_FACE = AT("Face", "Fläche", "the face (`f` statement)", "der Fläche (`f`-Anweisung)")
OBJ_NORMAL_VALUE = REC("Normal", "Normale", "Direction of the vertex normal as x, y, z.", "Richtung der Vertexnormale als x, y, z.")


def obj_faces(of_en, of_de):
    return {"role": "value", "label": L("Faces", "Flächen"), "description": L(f"Indices of the faces that belong to {of_en}, counted from 0.", f"Indizes der Flächen, die zu {of_de} gehören, ab 0 gezählt."), "step": 1, "precision": 0, "group": "value"}


R("obj", "insert-vertex", "index", POS("the vertices", "den Vertices"))
R("obj", "remove-vertex set-vertex", "index", OBJ_VERTEX)
R("obj", "set-group", "name", KEY("Group", "Gruppe", "Name of the group (`g` statement) that is created or reassigned.", "Name der Gruppe (`g`-Anweisung), die angelegt oder neu zugeordnet wird."))
R("obj", "set-group", "faces", obj_faces("the group", "der Gruppe"))
R("obj", "set-object", "name", KEY("Object", "Objekt", "Name of the object (`o` statement) that is created or reassigned.", "Name des Objekts (`o`-Anweisung), das angelegt oder neu zugeordnet wird."))
R("obj", "set-object", "faces", obj_faces("the object", "dem Objekt"))
R("obj", "insert-normal", "index", POS("the vertex normals", "den Vertexnormalen"))
R("obj", "set-normal remove-normal", "index", OBJ_NORMAL)
R("obj", "insert-normal set-normal", "normal", OBJ_NORMAL_VALUE)
R("obj", "set-face remove-face", "index", OBJ_FACE)
R("obj", "insert-face", "index", POS("the faces", "den Flächen"))
R("obj", "remove-object", "name", ENT("object", "Object", "Objekt", "Name of the object (`o` statement) that is removed.", "Name des Objekts (`o`-Anweisung), das entfernt wird."))
R("obj", "remove-group", "name", ENT("group", "Group", "Gruppe", "Name of the group (`g` statement) that is removed.", "Name der Gruppe (`g`-Anweisung), die entfernt wird."))
R("obj", "remove-texcoord set-texcoord", "index", OBJ_TEXCOORD)
R("obj", "insert-texcoord", "index", POS("the texture coordinates", "den Texturkoordinaten"))
#endregion 🔖️Obj

#region 🔖️Ply
R("ply", "set-row-property", "value", REC("Value", "Wert", "Typed scalar or list the property holds in this row.", "Typisierter Skalar oder Liste, die die Eigenschaft in dieser Zeile enthält."))
R("ply", "insert-comment", "index", POS("the header comments", "den Kommentaren des Kopfteils"))
R("ply", "insert-comment", "comment", TXT("Comment", "Kommentar", "Text of the `comment` header line.", "Text der Kopfzeile `comment`."))
R("ply", "remove-comment", "index", AT("Comment", "Kommentar", "the header comment", "des Kommentars im Kopfteil"))
R("ply", "remove-row", "index", AT("Row", "Zeile", "the row of the element", "der Zeile des Elements"))
R("ply", "insert-row", "index", POS("the rows of the element", "den Zeilen des Elements"))
R("ply", "insert-row", "row", REC("Row", "Zeile", "Property values of the inserted row, in property order.", "Eigenschaftswerte der eingefügten Zeile in der Reihenfolge der Eigenschaften."))
R("ply", "remove-element", "name", ENT("element", "Element", "Element", "Name of the element that is removed, for example vertex or face.", "Name des Elements, das entfernt wird, zum Beispiel vertex oder face."))
R("ply", "add-element", "index", POS("the elements", "den Elementen"))
R("ply", "add-element", "element", REC("Element", "Element", "Name, properties and rows of the element that is added.", "Name, Eigenschaften und Zeilen des Elements, das hinzugefügt wird."))
#endregion 🔖️Ply

#region 🔖️Json
JSON_CONTAINER = ADDR("Path", "Pfad", "Member names and array indices leading from the root to the object or array that changes; empty for the root.", "Eigenschaftsnamen und Array-Indizes von der Wurzel bis zum Objekt oder Array, das sich ändert; leer für die Wurzel.")
JSON_VALUE = ADDR("Path", "Pfad", "Member names and array indices leading from the root to the value that is set; empty for the root.", "Eigenschaftsnamen und Array-Indizes von der Wurzel bis zum Wert, der gesetzt wird; leer für die Wurzel.")
R("json", "upsert-member remove-member rename-member remove-array-element insert-array-element set-member", "path", JSON_CONTAINER)
R("json", "set-safe-number set-string set-scalar", "path", JSON_VALUE)
R("json", "upsert-member set-member", "key", KEY("Member name", "Name der Eigenschaft", "Name of the object member that is set; a new name adds the member.", "Name der Objekteigenschaft, die gesetzt wird; ein neuer Name fügt die Eigenschaft hinzu."))
R("json", "remove-member", "key", KEY("Member name", "Name der Eigenschaft", "Name of the object member that is removed.", "Name der Objekteigenschaft, die entfernt wird."))
R("json", "rename-member", "from", KEY("Member name", "Name der Eigenschaft", "Current name of the object member.", "Aktueller Name der Objekteigenschaft."))
R("json", "rename-member", "to", TXT("New name", "Neuer Name", "Name the member carries afterwards.", "Name, den die Eigenschaft danach trägt."))
R("json", "remove-array-element", "index", AT("Element", "Element", "the array element", "des Array-Elements"))
R("json", "insert-array-element", "index", POS("the elements of the array", "den Elementen des Arrays"))
R("json", "set-string", "value", MULTI("Text", "Text", "The string the value holds afterwards.", "Die Zeichenkette, die der Wert danach enthält."))
#endregion 🔖️Json

#region 🔖️SemioFlow
FLOW_NODE = ("node", "Node", "Knoten", "Id of the node.", "Kennung des Knotens.")
FLOW_EDGE = ("edge", "Edge", "Kante", "Id of the edge.", "Kennung der Kante.")
R("semio", "remove-edge set-edge-kind set-edge-endpoints", "id", ENT(*FLOW_EDGE), subset="flow")
R("semio", "set-node-param set-node-kind set-node-position set-node-label remove-node remove-node-param", "id", ENT(*FLOW_NODE), subset="flow")
R("semio", "insert-node", "node", REC("Node", "Knoten", "Id, kind, label, position, ports and parameters of the node that is inserted.", "Kennung, Art, Beschriftung, Position, Anschlüsse und Parameter des Knotens, der eingefügt wird."), subset="flow")
R("semio", "insert-edge", "edge", REC("Edge", "Kante", "Id, kind and endpoints of the edge that is inserted.", "Kennung, Art und Endpunkte der Kante, die eingefügt wird."), subset="flow")
R("semio", "set-node-param remove-node-param", "key", KEY("Parameter", "Parameter", "Name of the node parameter.", "Name des Knotenparameters."), subset="flow")
R("semio", "set-node-param", "value", TXT("Value", "Wert", "Text form of the value the parameter takes.", "Textform des Werts, den der Parameter erhält."), subset="flow")
R("semio", "set-edge-kind", "kind", TXT("Kind", "Art", "Kind of the edge, as the flow vocabulary names it.", "Art der Kante, wie sie das Flussvokabular benennt."), subset="flow")
R("semio", "set-node-kind", "kind", TXT("Kind", "Art", "Kind of the node, as the flow vocabulary names it.", "Art des Knotens, wie sie das Flussvokabular benennt."), subset="flow")
R("semio", "set-node-position", "position", REC("Position", "Position", "Canvas position of the node as x and y.", "Position des Knotens auf der Leinwand als x und y."), subset="flow")
R("semio", "set-edge-endpoints", "from", REC("Source", "Quelle", "Node and port the edge starts at.", "Knoten und Anschluss, an dem die Kante beginnt."), subset="flow")
R("semio", "set-edge-endpoints", "to", REC("Target", "Ziel", "Node and port the edge ends at.", "Knoten und Anschluss, an dem die Kante endet."), subset="flow")
R("semio", "set-node-label", "label", TXT("Label", "Beschriftung", "Text shown on the node.", "Text, der auf dem Knoten angezeigt wird."), subset="flow")
#endregion 🔖️SemioFlow

#region 🔖️SemioAnimation
ANIM_TIMELINE = AT("Timeline", "Zeitleiste", "the timeline", "der Zeitleiste")
ANIM_CHANNEL = AT("Channel", "Kanal", "the channel within the timeline", "des Kanals innerhalb der Zeitleiste")
ANIM_KEYFRAME = AT("Keyframe", "Schlüsselbild", "the keyframe within the channel", "des Schlüsselbilds innerhalb des Kanals")
R("semio", "insert-timeline", "index", POS("the timelines", "den Zeitleisten"), subset="animation")
R("semio", "set-timeline-name remove-timeline", "index", ANIM_TIMELINE, subset="animation")
R("semio", "set-channel-target set-channel-interpolation insert-channel insert-keyframe remove-keyframe set-keyframe-value set-keyframe-time remove-channel", "timelineIndex", ANIM_TIMELINE, subset="animation")
R("semio", "set-channel-target set-channel-interpolation remove-channel", "index", ANIM_CHANNEL, subset="animation")
R("semio", "insert-channel", "index", POS("the channels of the timeline", "den Kanälen der Zeitleiste"), subset="animation")
R("semio", "insert-keyframe remove-keyframe set-keyframe-value set-keyframe-time", "channelIndex", ANIM_CHANNEL, subset="animation")
R("semio", "insert-keyframe", "index", POS("the keyframes of the channel", "den Schlüsselbildern des Kanals"), subset="animation")
R("semio", "remove-keyframe set-keyframe-value set-keyframe-time", "index", ANIM_KEYFRAME, subset="animation")
R("semio", "set-channel-target", "target", REC("Target", "Ziel", "Entity and property the channel animates.", "Entität und Eigenschaft, die der Kanal animiert."), subset="animation")
R("semio", "set-timeline-name", "name", TXT("Name", "Name", "Display name of the timeline; cleared, it has none.", "Anzeigename der Zeitleiste; geleert hat sie keinen."), subset="animation")
R("semio", "set-keyframe-value", "value", REC("Value", "Wert", "Typed value the keyframe holds: scalar, vector, quaternion or colour.", "Typisierter Wert des Schlüsselbilds: Skalar, Vektor, Quaternion oder Farbe."), subset="animation")
#endregion 🔖️SemioAnimation

#region 🔖️SemioVideo
VIDEO_STREAM = AT("Stream", "Datenstrom", "the stream", "des Datenstroms")
VIDEO_SAMPLE = AT("Sample", "Sample", "the sample within the stream", "des Samples innerhalb des Datenstroms")
R("semio", "insert-sample set-sample-flags remove-sample", "streamIndex", VIDEO_STREAM, subset="video")
R("semio", "insert-sample", "index", POS("the samples of the stream", "den Samples des Datenstroms"), subset="video")
R("semio", "set-sample-flags remove-sample", "index", VIDEO_SAMPLE, subset="video")
R("semio", "insert-stream", "index", POS("the streams", "den Datenströmen"), subset="video")
R("semio", "set-stream-meta remove-stream", "index", VIDEO_STREAM, subset="video")
R("semio", "insert-stream", "stream", REC("Stream", "Datenstrom", "Kind, codec, dimensions, rate and samples of the stream that is inserted.", "Art, Codec, Abmessungen, Rate und Samples des Datenstroms, der eingefügt wird."), subset="video")
R("semio", "set-stream-meta", "width", INT("Width", "Breite", "Frame width of the stream.", "Bildbreite des Datenstroms.", unit="px", softMax=7680, snaps=VIDEO_WIDTHS), subset="video")
R("semio", "set-stream-meta", "height", INT("Height", "Höhe", "Frame height of the stream.", "Bildhöhe des Datenstroms.", unit="px", softMax=4320, snaps=VIDEO_HEIGHTS), subset="video")
R("semio", "set-sample-flags", "key", FLAG("Sync sample", "Sync-Sample", "Whether the sample decodes without any earlier sample (a key frame).", "Ob das Sample ohne frühere Samples dekodierbar ist (ein Schlüsselbild)."), subset="video")
#endregion 🔖️SemioVideo

#region 🔖️SemioModel
MODEL_ELEMENT = ("element", "Element", "Element", "Id of the model element.", "Kennung des Modellelements.")
MODEL_RELATION = ("relation", "Relation", "Relation", "Id of the relation.", "Kennung der Relation.")
MODEL_SPATIAL = ("spatialNode", "Spatial node", "Raumknoten", "Id of the spatial node.", "Kennung des Raumknotens.")
MODEL_PLACEMENT = REC("Placement", "Platzierung", "Translation, rotation (unit quaternion) and scale of the local placement.", "Verschiebung, Drehung (Einheitsquaternion) und Skalierung der lokalen Platzierung.")
R("semio", "remove-relation set-relation", "id", ENT(*MODEL_RELATION), subset="model")
R("semio", "set-element remove-element", "id", ENT(*MODEL_ELEMENT), subset="model")
R("semio", "remove-spatial-node set-spatial-node", "id", ENT(*MODEL_SPATIAL), subset="model")
R("semio", "set-element set-spatial-node", "placement", MODEL_PLACEMENT, subset="model")
R("semio", "insert-spatial-node", "node", REC("Spatial node", "Raumknoten", "Id, kind, name, parent and placement of the spatial node that is inserted.", "Kennung, Art, Name, übergeordneter Knoten und Platzierung des Raumknotens, der eingefügt wird."), subset="model")
R("semio", "set-relation", "kind", REC("Kind", "Art", "Kind of the relation between the two elements (aggregates, contained in, connects to, fills void, voids element or other) with its optional label.", "Art der Relation zwischen den beiden Elementen (aggregiert, enthalten in, verbunden mit, füllt Öffnung, öffnet Element oder sonstige) mit optionaler Beschriftung."), subset="model")
R("semio", "set-relation", "from", REFV("element", "From", "Von", "Id of the element the relation starts at.", "Kennung des Elements, an dem die Relation beginnt."), subset="model")
R("semio", "set-relation", "to", REFV("element", "To", "Nach", "Id of the element the relation ends at.", "Kennung des Elements, an dem die Relation endet."), subset="model")
R("semio", "set-spatial-node", "name", TXT("Name", "Name", "Display name of the spatial node.", "Anzeigename des Raumknotens."), subset="model")
R("semio", "set-spatial-node", "parentId", REFV("spatialNode", "Parent", "Übergeordneter Knoten", "Id of the parent spatial node; cleared, the node becomes a root.", "Kennung des übergeordneten Raumknotens; geleert wird der Knoten zur Wurzel."), subset="model")
R("semio", "insert-element", "element", REC("Element", "Element", "Id, class, placement, geometry and property sets of the element that is inserted.", "Kennung, Klasse, Platzierung, Geometrie und Eigenschaftssätze des Elements, das eingefügt wird."), subset="model")
#endregion 🔖️SemioModel

#region 🔖️SemioTable
TABLE_COLUMN = ("column", "Column", "Spalte", "Name of the column.", "Name der Spalte.")
R("semio", "remove-row", "index", AT("Row", "Zeile", "the row", "der Zeile"), subset="table")
R("semio", "insert-row", "index", POS("the rows", "den Zeilen"), subset="table")
R("semio", "insert-row", "row", REC("Row", "Zeile", "Cell values of the inserted row, in column order.", "Zellwerte der eingefügten Zeile in Spaltenreihenfolge."), subset="table")
R("semio", "create-column", "name", TXT("Column name", "Spaltenname", "Name the new column carries.", "Name, den die neue Spalte trägt."), subset="table")
R("semio", "create-column", "index", IDX("Insert position", "Einfügeposition", "Position among the columns at which it is inserted, counted from 0; cleared, the column is appended.", "Position unter den Spalten, an der eingefügt wird, ab 0 gezählt; geleert wird die Spalte angehängt."), subset="table")
R("semio", "rename-column reorder-columns delete-column", "name", ENT(*TABLE_COLUMN), subset="table")
R("semio", "rename-column", "new_name", TXT("New name", "Neuer Name", "Name the column carries afterwards.", "Name, den die Spalte danach trägt."), subset="table")
R("semio", "reorder-rows", "from", AT("Row", "Zeile", "the row that moves", "der Zeile, die verschoben wird"), subset="table")
R("semio", "reorder-rows", "to", IDX("Destination", "Zielposition", "Position the row holds afterwards, counted from 0.", "Position, die die Zeile danach einnimmt, ab 0 gezählt."), subset="table")
#endregion 🔖️SemioTable

#region 🔖️SemioCad
CAD_BLOCK = ("block", "Block", "Block", "Name of the block definition.", "Name der Blockdefinition.")
CAD_ENTITY = ("entity", "Entity", "Entität", "Handle of the entity.", "Referenz (Handle) der Entität.")
CAD_LAYER = ("layer", "Layer", "Layer", "Name of the layer.", "Name des Layers.")
CAD_GEOMETRY = REC("Geometry", "Geometrie", "Type and geometric data the entity takes: line, circle, arc, polyline, text or block insert.", "Typ und Geometriedaten, die die Entität erhält: Linie, Kreis, Bogen, Polylinie, Text oder Blockreferenz.")
CAD_RECORD = REC("Entity", "Entität", "Handle, layer and geometry of the entity that is added.", "Referenz (Handle), Layer und Geometrie der Entität, die hinzugefügt wird.")
CAD_TO_LAYER = REFV("layer", "Layer", "Layer", "Name of the layer the entity is assigned to.", "Name des Layers, dem die Entität zugeordnet wird.")
R("semio", "remove-block-entity set-block-entity-geometry add-block-entity set-block-entity-layer", "blockName", ENT(*CAD_BLOCK), subset="cad")
R("semio", "remove-block-entity set-entity-layer set-entity-geometry set-block-entity-geometry remove-entity set-block-entity-layer", "handle", ENT(*CAD_ENTITY), subset="cad")
R("semio", "set-layer remove-layer", "name", ENT(*CAD_LAYER), subset="cad")
R("semio", "set-block-base-point remove-block", "name", ENT(*CAD_BLOCK), subset="cad")
R("semio", "set-layer", "visible", FLAG("Visible", "Sichtbar", "Whether the layer is displayed.", "Ob der Layer angezeigt wird."), subset="cad")
R("semio", "set-entity-layer set-block-entity-layer", "layer", CAD_TO_LAYER, subset="cad")
R("semio", "set-entity-geometry set-block-entity-geometry", "entity", CAD_GEOMETRY, subset="cad")
R("semio", "add-entity add-block-entity", "entity", CAD_RECORD, subset="cad")
R("semio", "add-layer", "layer", REC("Layer", "Layer", "Name, colour index, line type and visibility of the layer that is added.", "Name, Farbindex, Linientyp und Sichtbarkeit des Layers, der hinzugefügt wird."), subset="cad")
R("semio", "add-block", "block", REC("Block", "Block", "Name, base point and entities of the block definition that is added.", "Name, Basispunkt und Entitäten der Blockdefinition, die hinzugefügt wird."), subset="cad")
#endregion 🔖️SemioCad

#region 🔖️SemioDocument
DOC_BLOCK = ADDR("Block", "Block", "Path of the block: the steps into nested containers and its index there, counted from 0.", "Pfad des Blocks: die Schritte in verschachtelte Container und sein Index dort, ab 0 gezählt.")
DOC_RUN = AT("Run", "Textlauf", "the run within the paragraph", "des Textlaufs innerhalb des Absatzes")
DOC_STYLE = ("style", "Style", "Formatvorlage", "Id of the style.", "Kennung der Formatvorlage.")
DOC_IMAGE = ("image", "Image", "Bild", "Id of the embedded image.", "Kennung des eingebetteten Bildes.")
DOC_IMAGE_SIZE = ("Display {0} of the image; cleared, the image keeps its own {0}.", "Anzeige{0} des Bildes; geleert behält das Bild seine eigene {1}.")
R("semio", "set-run-style set-heading-level set-block-content set-image-block set-list-ordered set-run-text remove-block set-paragraph-style", "path", DOC_BLOCK, subset="document")
R("semio", "insert-block", "path", ADDR("Insert position", "Einfügeposition", "Path of the new block: the steps into nested containers and the index it is inserted at, counted from 0.", "Pfad des neuen Blocks: die Schritte in verschachtelte Container und der Index, an dem eingefügt wird, ab 0 gezählt."), subset="document")
R("semio", "set-run-style set-run-text", "run_index", DOC_RUN, subset="document")
R("semio", "set-run-style", "style", REC("Run style", "Zeichenformat", "Character formatting of the run: weight, slant, underline, font, size and colour.", "Zeichenformatierung des Textlaufs: Schriftstärke, Neigung, Unterstreichung, Schriftart, Größe und Farbe."), subset="document")
R("semio", "set-style-name set-style-based-on remove-style", "id", ENT(*DOC_STYLE), subset="document")
R("semio", "set-style-name", "name", TXT("Style name", "Name der Formatvorlage", "Display name of the style.", "Anzeigename der Formatvorlage."), subset="document")
R("semio", "set-image-bytes remove-image", "id", ENT(*DOC_IMAGE), subset="document")
R("semio", "set-block-content", "block", REC("Block", "Block", "The block that takes the place of the addressed one.", "Der Block, der an die Stelle des adressierten tritt."), subset="document")
R("semio", "insert-block", "block", REC("Block", "Block", "The block that is inserted: paragraph, heading, list, table, image or code.", "Der Block, der eingefügt wird: Absatz, Überschrift, Liste, Tabelle, Bild oder Code."), subset="document")
R("semio", "set-image-block", "width", NUM("Width", "Breite", DOC_IMAGE_SIZE[0].format("width"), DOC_IMAGE_SIZE[1].format("breite", "Breite"), 1, 2, softMin=0), subset="document")
R("semio", "set-image-block", "height", NUM("Height", "Höhe", DOC_IMAGE_SIZE[0].format("height"), DOC_IMAGE_SIZE[1].format("höhe", "Höhe"), 1, 2, softMin=0), subset="document")
R("semio", "insert-image", "image", REC("Image", "Bild", "Id, media type and bytes of the image that is embedded.", "Kennung, Medientyp und Bytes des Bildes, das eingebettet wird."), subset="document")
R("semio", "set-run-text", "text", MULTI("Text", "Text", "Text the run holds afterwards.", "Text, den der Textlauf danach enthält."), subset="document")
R("semio", "insert-style", "style", REC("Style", "Formatvorlage", "Id, name and parent of the style that is inserted.", "Kennung, Name und übergeordnete Formatvorlage der Formatvorlage, die eingefügt wird."), subset="document")
#endregion 🔖️SemioDocument

#region 🔖️SemioObjectKit
CHILD_ID = TXT("Child id", "Kind-ID", "Id the new composed child is registered under.", "Kennung, unter der das neue zusammengesetzte Kind registriert wird.")
CHILD_ARTIFACT = REC("Artifact", "Artefakt", "The artifact that backs the child: kind, standard, subset and location.", "Das Artefakt hinter dem Kind: Art, Standard, Teilmenge und Speicherort.")
CHILD = ENT("child", "Child", "Kind", "Id of the composed child that is deleted.", "Kennung des zusammengesetzten Kindes, das gelöscht wird.")
R("semio", "create-properties create-mesh create-brep", "child_id", CHILD_ID, subset="object")
R("semio", "create-properties create-mesh create-brep", "target", CHILD_ARTIFACT, subset="object")
R("semio", "scale-object", "scale", REC("Scale", "Skalierung", "Scale factors along x, y and z.", "Skalierungsfaktoren entlang x, y und z."), subset="object")
R("semio", "rotate-object", "rotation", REC("Rotation", "Drehung", "Orientation as a unit quaternion (x, y, z, w).", "Ausrichtung als Einheitsquaternion (x, y, z, w)."), subset="object")
R("semio", "create-object create-model create-properties", "child_id", CHILD_ID, subset="kit")
R("semio", "create-object create-model create-properties", "target", CHILD_ARTIFACT, subset="kit")
R("semio", "delete-model delete-object", "child_id", CHILD, subset="kit")
R("semio", "unbind-representation change-representation-pin", "index", AT("Representation", "Repräsentation", "the bound representation", "der gebundenen Repräsentation"), subset="kit")
R("semio", "rename-type remove-type", "id", ENT("type", "Type", "Typ", "Id of the type.", "Kennung des Typs."), subset="kit")
R("semio", "rename-type", "new_name", TXT("New name", "Neuer Name", "Name the type carries afterwards.", "Name, den der Typ danach trägt."), subset="kit")
R("semio", "add-type", "id", TXT("Type id", "Typ-ID", "Id the new type carries.", "Kennung, die der neue Typ trägt."), subset="kit")
R("semio", "add-type", "name", TXT("Name", "Name", "Display name of the new type.", "Anzeigename des neuen Typs."), subset="kit")
R("semio", "add-design", "id", TXT("Design id", "Entwurfs-ID", "Id the new design carries.", "Kennung, die der neue Entwurf trägt."), subset="kit")
R("semio", "add-design", "name", TXT("Name", "Name", "Display name of the new design.", "Anzeigename des neuen Entwurfs."), subset="kit")
R("semio", "edit-design remove-design", "id", ENT("design", "Design", "Entwurf", "Id of the design.", "Kennung des Entwurfs."), subset="kit")
R("semio", "bind-representation", "target", REC("Artifact", "Artefakt", "The artifact that is bound as a representation: kind, standard, subset and location.", "Das Artefakt, das als Repräsentation gebunden wird: Art, Standard, Teilmenge und Speicherort."), subset="kit")
R("semio", "bind-representation", "role", TXT("Role", "Rolle", "What the representation stands for in the kit, for example a type's geometry.", "Wofür die Repräsentation im Bausatz steht, zum Beispiel die Geometrie eines Typs."), subset="kit")
#endregion 🔖️SemioObjectKit

#region 🔖️SemioPresentation
SLIDE = AT("Slide", "Folie", "the slide", "der Folie")
SHAPE = AT("Shape", "Form", "the shape on the slide", "der Form auf der Folie")
R("semio", "set-text-box-blocks remove-shape insert-shape set-shape-frame", "slide_index", SLIDE, subset="presentation")
R("semio", "set-text-box-blocks remove-shape set-shape-frame", "shape_index", SHAPE, subset="presentation")
R("semio", "insert-shape", "shape_index", POS("the shapes of the slide", "den Formen der Folie"), subset="presentation")
R("semio", "insert-slide", "index", POS("the slides", "den Folien"), subset="presentation")
R("semio", "remove-slide set-slide-layout set-slide-notes", "index", SLIDE, subset="presentation")
R("semio", "set-layout-master remove-layout", "id", ENT("layout", "Layout", "Layout", "Id of the slide layout.", "Kennung des Folienlayouts."), subset="presentation")
R("semio", "remove-master", "id", ENT("master", "Master", "Master", "Id of the slide master.", "Kennung des Folienmasters."), subset="presentation")
R("semio", "insert-layout", "layout", REC("Layout", "Layout", "Id, name, master and placeholders of the layout that is inserted.", "Kennung, Name, Master und Platzhalter des Layouts, das eingefügt wird."), subset="presentation")
R("semio", "set-shape-frame", "frame", REC("Frame", "Rahmen", "Position and size of the shape on the slide: x, y, width and height.", "Position und Größe der Form auf der Folie: x, y, Breite und Höhe."), subset="presentation")
#endregion 🔖️SemioPresentation

#region 🔖️SemioAudio
AUDIO_TAG = AT("Tag", "Tag", "the metadata tag", "des Metadaten-Tags")
AUDIO_CHANNEL = AT("Channel", "Kanal", "the channel", "des Kanals")
R("semio", "remove-tag set-tag-value", "index", AUDIO_TAG, subset="audio")
R("semio", "set-channel-samples remove-channel", "index", AUDIO_CHANNEL, subset="audio")
R("semio", "insert-channel", "index", POS("the channels", "den Kanälen"), subset="audio")
R("semio", "insert-tag", "index", POS("the metadata tags", "den Metadaten-Tags"), subset="audio")
R("semio", "insert-tag", "tag", REC("Tag", "Tag", "Key and value of the metadata tag that is inserted.", "Schlüssel und Wert des Metadaten-Tags, das eingefügt wird."), subset="audio")
R("semio", "set-tag-value", "value", TXT("Value", "Wert", "Text the tag holds afterwards.", "Text, den das Tag danach enthält."), subset="audio")
#endregion 🔖️SemioAudio

#region 🔖️SemioValue
VALUE_PATH = ("Map keys and list indices leading from the root to {0}; empty for the root.", "Zuordnungsschlüssel und Listenindizes von der Wurzel bis {0}; leer für die Wurzel.")
VALUE = REC("Value", "Wert", "The typed value written: null, boolean, integer, number, text, bytes, list, map or reference.", "Der geschriebene typisierte Wert: null, Wahrheitswert, Ganzzahl, Zahl, Text, Bytes, Liste, Zuordnung oder Referenz.")
R("semio", "remove-node set-node", "id", ADDR("Node", "Knoten", "Id of the value node.", "Kennung des Wertknotens."), subset="value")
R("semio", "remove-map-entry set-map-entry", "path", ADDR("Path", "Pfad", VALUE_PATH[0].format("the map"), VALUE_PATH[1].format("zur Zuordnung")), subset="value")
R("semio", "insert-list-item remove-list-item", "path", ADDR("Path", "Pfad", VALUE_PATH[0].format("the list"), VALUE_PATH[1].format("zur Liste")), subset="value")
R("semio", "set-value", "path", ADDR("Path", "Pfad", VALUE_PATH[0].format("the value that is set"), VALUE_PATH[1].format("zum Wert, der gesetzt wird")), subset="value")
R("semio", "remove-map-entry set-map-entry", "key", KEY("Key", "Schlüssel", "Key of the map entry.", "Schlüssel des Zuordnungseintrags."), subset="value")
R("semio", "insert-list-item", "index", POS("the items of the list", "den Elementen der Liste"), subset="value")
R("semio", "remove-list-item", "index", AT("Item", "Element", "the list item", "des Listenelements"), subset="value")
R("semio", "insert-list-item set-value set-map-entry set-node", "value", VALUE, subset="value")
#endregion 🔖️SemioValue

#region 🔖️SemioText
TEXT_RUN = AT("Run", "Textlauf", "the run", "des Textlaufs")
R("semio", "edit-run change-run-language remove-run", "index", TEXT_RUN, subset="text")
R("semio", "add-mark remove-mark", "run_index", TEXT_RUN, subset="text")
R("semio", "add-mark", "index", POS("the marks of the run", "den Auszeichnungen des Textlaufs"), subset="text")
R("semio", "remove-mark", "index", AT("Mark", "Auszeichnung", "the mark within the run", "der Auszeichnung innerhalb des Textlaufs"), subset="text")
R("semio", "insert-run", "index", POS("the runs", "den Textläufen"), subset="text")
R("semio", "reorder-runs", "from", AT("Run", "Textlauf", "the run that moves", "des Textlaufs, der verschoben wird"), subset="text")
R("semio", "reorder-runs", "to", IDX("Destination", "Zielposition", "Position the run holds afterwards, counted from 0.", "Position, die der Textlauf danach einnimmt, ab 0 gezählt."), subset="text")
#endregion 🔖️SemioText

#region 🔖️SemioMesh
MESH_MATERIAL = ENT("material", "Material", "Material", "Id of the material.", "Kennung des Materials.")
MESH = ENT("mesh", "Mesh", "Netz", "Id of the mesh.", "Kennung des Netzes.")
MESH_PRIMITIVE = ENT("primitive", "Primitive", "Primitiv", "Id of the primitive within the mesh.", "Kennung des Primitivs innerhalb des Netzes.")
MESH_TEXTURE = ENT("texture", "Texture", "Textur", "Id of the texture.", "Kennung der Textur.")
R("semio", "change-material-metallic change-material-base change-material-roughness delete-material", "id", MESH_MATERIAL, subset="mesh")
R("semio", "change-texture-mime replace-texture-bytes delete-texture", "id", MESH_TEXTURE, subset="mesh")
R("semio", "delete-mesh", "id", MESH, subset="mesh")
R("semio", "delete-primitive move-vertex set-primitive-topology create-primitive set-primitive-material", "mesh_id", MESH, subset="mesh")
R("semio", "delete-primitive move-vertex set-primitive-topology set-primitive-material", "primitive_id", MESH_PRIMITIVE, subset="mesh")
R("semio", "create-material", "material", REC("Material", "Material", "Id, base colour, metallic and roughness factors and texture slots of the material that is created.", "Kennung, Basisfarbe, Metallizitäts- und Rauheitsfaktor sowie Texturplätze des Materials, das erstellt wird."), subset="mesh")
R("semio", "create-primitive", "primitive", REC("Primitive", "Primitiv", "Id, topology, material and vertex data of the primitive that is created.", "Kennung, Topologie, Material und Vertexdaten des Primitivs, das erstellt wird."), subset="mesh")
R("semio", "create-mesh", "mesh", REC("Mesh", "Netz", "Id, name and primitives of the mesh that is created.", "Kennung, Name und Primitive des Netzes, das erstellt wird."), subset="mesh")
#endregion 🔖️SemioMesh

#region 🔖️SemioGraph
GRAPH_REF = {"domain": "graph"}
GRAPH_NODE_VALUE = {"widget": "reference", "role": "target", "label": L("Node id", "Knoten-ID"), "ref": {"kind": "node", "domain": "graph", "granularity": "node"}}
GRAPH_EDGE_VALUE = {"widget": "reference", "role": "target", "label": L("Edge id", "Kanten-ID"), "ref": {"kind": "edge", "domain": "graph", "granularity": "edge"}}
NEW_ID_VALUE = {"widget": "text", "role": "value", "label": L("Id", "ID")}
GRAPH_GRID = {"step": 1, "precision": 2, "snapSource": {"config": "gridFactor"}}
PORT_KINDS = {"in": L("Input", "Eingang"), "out": L("Output", "Ausgang"), "inOut": L("Bidirectional", "Bidirektional")}
PORT_CATEGORY = {"widget": "text", "label": L("Category", "Kategorie"), "description": L("Category of the port, as the graph's manifest names it.", "Kategorie des Anschlusses, wie sie das Manifest des Graphen benennt.")}


def graph_node(d_en, d_de):
    return ADDR("Node", "Knoten", d_en, d_de)


R("semio", "delete-edge", "id", ADDR("Edge", "Kante", "The edge that is deleted.", "Die Kante, die gelöscht wird."), subset="graph", nested={"properties/value": GRAPH_EDGE_VALUE})
R("semio", "move-node change-node-kind change-node-label delete-node", "id", graph_node("The node the mutation addresses.", "Der Knoten, den die Mutation adressiert."), subset="graph", nested={"properties/value": GRAPH_NODE_VALUE})
R("semio", "add-node-property add-node-port remove-node-port", "node_id", graph_node("The node the mutation addresses.", "Der Knoten, den die Mutation adressiert."), subset="graph", nested={"properties/value": GRAPH_NODE_VALUE})
R("semio", "add-node-property", "index", POS("the properties of the node", "den Eigenschaften des Knotens"), subset="graph")
R("semio", "add-node-port", "index", POS("the ports of the node", "den Anschlüssen des Knotens"), subset="graph")
R("semio", "remove-node-port", "index", AT("Port", "Anschluss", "the port of the node", "des Anschlusses des Knotens"), subset="graph")
R("semio", "add-node-port", "port", None, subset="graph", nested={"properties/category": PORT_CATEGORY})
R("semio", "remove-node-property", "node_id", {}, subset="graph", drop=("widget", "role", "ref"), nested={"properties/value": GRAPH_NODE_VALUE})
R("semio", "remove-node-property", "key", {}, subset="graph", drop=("role",))
R("semio", "create-edge", "id", ADDR("Edge id", "Kanten-ID", "Id the new edge carries.", "Kennung, die die neue Kante trägt."), subset="graph", nested={"properties/value": NEW_ID_VALUE})
R("semio", "create-edge", "source", ADDR("Source node", "Quellknoten", "Node the edge starts at.", "Knoten, an dem die Kante beginnt."), subset="graph", nested={"properties/value": GRAPH_NODE_VALUE})
R("semio", "create-edge", "target", ADDR("Target node", "Zielknoten", "Node the edge ends at.", "Knoten, an dem die Kante endet."), subset="graph", nested={"properties/value": GRAPH_NODE_VALUE})
R("semio", "create-edge", "kind", TXT("Kind", "Art", "Kind of the edge, as the graph's manifest names it.", "Art der Kante, wie sie das Manifest des Graphen benennt."), subset="graph")
R("semio", "create-edge", "label", TXT("Label", "Beschriftung", "Text shown on the edge.", "Text, der an der Kante angezeigt wird."), subset="graph")
R("semio", "create-edge", "properties", REC("Properties", "Eigenschaften", "Keyed typed entries attached to the edge.", "Einträge mit Schlüssel und typisiertem Wert, die an der Kante hängen."), subset="graph")
R("semio", "create-edge", "source_port", TXT("Source port", "Quellanschluss", "Name of the port on the source node; cleared, the edge attaches to the node itself.", "Name des Anschlusses am Quellknoten; geleert hängt die Kante am Knoten selbst."), subset="graph")
R("semio", "create-edge", "target_port", TXT("Target port", "Zielanschluss", "Name of the port on the target node; cleared, the edge attaches to the node itself.", "Name des Anschlusses am Zielknoten; geleert hängt die Kante am Knoten selbst."), subset="graph")
R("semio", "create-node", "id", ADDR("Node id", "Knoten-ID", "Id the new node carries.", "Kennung, die der neue Knoten trägt."), subset="graph", nested={"properties/value": NEW_ID_VALUE})
R("semio", "create-node", "kind", TXT("Kind", "Art", "Kind of the node, as the graph's manifest names it.", "Art des Knotens, wie sie das Manifest des Graphen benennt."), subset="graph")
R("semio", "create-node", "label", TXT("Label", "Beschriftung", "Text shown on the node.", "Text, der auf dem Knoten angezeigt wird."), subset="graph")
R("semio", "create-node", "position", REC("Position", "Position", "Canvas position of the node's origin.", "Position des Knotenursprungs auf der Leinwand."), subset="graph", nested={"properties/x": {"widget": "stepper", "role": "value", "label": L("X", "X"), **GRAPH_GRID}, "properties/y": {"widget": "stepper", "role": "value", "label": L("Y", "Y"), **GRAPH_GRID}})
R("semio", "create-node", "width", NUM("Width", "Breite", "Width of the node on the canvas.", "Breite des Knotens auf der Leinwand.", 1, 2, softMin=0, snapSource={"config": "gridFactor"}), subset="graph")
R("semio", "create-node", "height", NUM("Height", "Höhe", "Height of the node on the canvas.", "Höhe des Knotens auf der Leinwand.", 1, 2, softMin=0, snapSource={"config": "gridFactor"}), subset="graph")
R("semio", "create-node", "ports", REC("Ports", "Anschlüsse", "Named connection points of the node with their direction, category and properties.", "Benannte Anschlusspunkte des Knotens mit Richtung, Kategorie und Eigenschaften."), subset="graph", nested={"items/properties/kind": {"widget": "segmented", "label": L("Direction", "Richtung"), "options": PORT_KINDS}, "items/properties/category": PORT_CATEGORY})
R("semio", "create-node", "properties", REC("Properties", "Eigenschaften", "Keyed typed entries attached to the node.", "Einträge mit Schlüssel und typisiertem Wert, die am Knoten hängen."), subset="graph")
#endregion 🔖️SemioGraph

#region 🔖️SemioDrawing
DRAW_NODE = ADDR("Node", "Knoten", "Layer index and child indices of the node, each counted from 0.", "Ebenenindex und Kindindizes des Knotens, jeweils ab 0 gezählt.")
DRAW_PARENT = ADDR("Parent", "Übergeordneter Knoten", "Layer index and child indices of the group (or the layer itself) that holds the nodes, each counted from 0.", "Ebenenindex und Kindindizes der Gruppe (oder der Ebene selbst), die die Knoten enthält, jeweils ab 0 gezählt.")
DRAW_STYLE = ENT("style", "Style", "Stil", "Name of the drawing style.", "Name des Zeichenstils.")
R("semio", "delete-node unflatten-node ungroup-node move-node scale-node rotate-node replace-path flatten-node", "at", DRAW_NODE, subset="drawing")
R("semio", "create-node reorder-nodes group-nodes", "parent", DRAW_PARENT, subset="drawing")
R("semio", "create-node", "index", POS("the children of the parent", "den Kindknoten des übergeordneten Knotens"), subset="drawing")
R("semio", "create-node", "node", REC("Node", "Knoten", "The shape, path, text, image or group that is created.", "Die Form, der Pfad, der Text, das Bild oder die Gruppe, die erstellt wird."), subset="drawing")
R("semio", "create-layer", "index", POS("the layers", "den Ebenen"), subset="drawing")
R("semio", "create-layer", "layer", REC("Layer", "Ebene", "Id, name, visibility and nodes of the layer that is created.", "Kennung, Name, Sichtbarkeit und Knoten der Ebene, die erstellt wird."), subset="drawing")
R("semio", "change-stroke-width change-stroke replace-fill", "style_name", DRAW_STYLE, subset="drawing")
R("semio", "reorder-nodes", "from", AT("Node", "Knoten", "the node that moves among its siblings", "des Knotens, der unter seinen Geschwistern verschoben wird"), subset="drawing")
R("semio", "reorder-nodes", "to", IDX("Destination", "Zielposition", "Position the node holds among its siblings afterwards, counted from 0.", "Position, die der Knoten danach unter seinen Geschwistern einnimmt, ab 0 gezählt."), subset="drawing")
R("semio", "drag-nodes", "offset", REC("Offset", "Versatz", "Distance every dragged node moves by, as x and y.", "Strecke, um die jeder gezogene Knoten verschoben wird, als x und y."), subset="drawing")
R("semio", "delete-layer", "id", ENT("layer", "Layer", "Ebene", "Id of the layer that is deleted.", "Kennung der Ebene, die gelöscht wird."), subset="drawing")
R("semio", "group-nodes", "indices", ADDR("Nodes", "Knoten", "Indices of the sibling nodes that are grouped, counted from 0.", "Indizes der Geschwisterknoten, die gruppiert werden, ab 0 gezählt.", step=1, precision=0), subset="drawing")
R("semio", "group-nodes", "transform", REC("Group transform", "Gruppentransformation", "Translation, rotation (unit quaternion) and scale of the new group.", "Verschiebung, Drehung (Einheitsquaternion) und Skalierung der neuen Gruppe."), subset="drawing")
#endregion 🔖️SemioDrawing

#region 🔖️SemioImage
IMAGE_FRAME = AT("Frame", "Einzelbild", "the frame", "des Einzelbilds")
R("semio", "set-frame-delay set-frame-pixels remove-frame", "index", IMAGE_FRAME, subset="image")
R("semio", "insert-frame", "index", POS("the frames", "den Einzelbildern"), subset="image")
R("semio", "insert-frame", "frame", REC("Frame", "Einzelbild", "Delay and RGBA pixels of the frame that is inserted.", "Verzögerung und RGBA-Pixel des Einzelbilds, das eingefügt wird."), subset="image")
R("semio", "set-metadata-entry", "key", KEY("Key", "Schlüssel", "Name of the metadata entry that is set; a new name adds the entry.", "Name des Metadateneintrags, der gesetzt wird; ein neuer Name fügt den Eintrag hinzu."), subset="image")
R("semio", "remove-metadata-entry", "key", KEY("Key", "Schlüssel", "Name of the metadata entry that is removed.", "Name des Metadateneintrags, der entfernt wird."), subset="image")
R("semio", "set-metadata-entry", "value", TXT("Value", "Wert", "Text the metadata entry holds.", "Text, den der Metadateneintrag enthält."), subset="image")
R("semio", "set-dimensions", "width", INT("Width", "Breite", "Width of every frame.", "Breite jedes Einzelbilds.", unit="px", softMax=16384), subset="image")
R("semio", "set-dimensions", "height", INT("Height", "Höhe", "Height of every frame.", "Höhe jedes Einzelbilds.", unit="px", softMax=16384), subset="image")
R("semio", "move-frame", "from", AT("Frame", "Einzelbild", "the frame that moves", "des Einzelbilds, das verschoben wird"), subset="image")
R("semio", "move-frame", "to", IDX("Destination", "Zielposition", "Position the frame holds afterwards, counted from 0.", "Position, die das Einzelbild danach einnimmt, ab 0 gezählt."), subset="image")
#endregion 🔖️SemioImage

#region 🔖️SemioBrep
DECADES = [1e-9, 1e-8, 1e-7, 1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 1e-1]


def brep_tolerance(of_en, of_de):
    return NUM("Tolerance", "Toleranz", f"Geometric tolerance of {of_en}: the distance within which it coincides with the geometry it bounds.", f"Geometrische Toleranz {of_de}: der Abstand, innerhalb dessen sie mit der begrenzten Geometrie zusammenfällt.", 1e-9, 9, widget="slider", softMin=1e-9, softMax=1e-1, scale="log", snaps=DECADES)


def brep_new(en, de, of_en, of_de):
    return TXT(en, de, f"Id the new {of_en} carries.", f"Kennung, die {of_de} trägt.")


def brep_deleted(kind, en, de, of_en, of_de):
    return ENT(kind, en, de, f"Id of the {of_en} that is deleted.", f"Kennung {of_de}, die gelöscht wird.")


R("semio", "delete-edge", "id", brep_deleted("edge", "Edge", "Kante", "edge", "der Kante"), subset="brep")
R("semio", "delete-shell", "id", brep_deleted("shell", "Shell", "Schale", "shell", "der Schale"), subset="brep")
R("semio", "delete-face", "id", brep_deleted("face", "Face", "Fläche", "face", "der Fläche"), subset="brep")
R("semio", "delete-solid", "id", ENT("solid", "Solid", "Körper", "Id of the solid that is deleted.", "Kennung des Körpers, der gelöscht wird."), subset="brep")
R("semio", "delete-vertex", "id", ENT("vertex", "Vertex", "Vertex", "Id of the vertex that is deleted.", "Kennung des Vertex, der gelöscht wird."), subset="brep")
R("semio", "create-vertex", "id", brep_new("Vertex id", "Vertex-ID", "vertex", "der neue Vertex"), subset="brep")
R("semio", "create-shell", "id", brep_new("Shell id", "Schalen-ID", "shell", "die neue Schale"), subset="brep")
R("semio", "create-face", "id", brep_new("Face id", "Flächen-ID", "face", "die neue Fläche"), subset="brep")
R("semio", "create-edge", "id", brep_new("Edge id", "Kanten-ID", "edge", "die neue Kante"), subset="brep")
R("semio", "create-solid", "id", brep_new("Solid id", "Körper-ID", "solid", "der neue Körper"), subset="brep")
R("semio", "create-vertex", "point", REC("Point", "Punkt", "Position of the vertex as x, y, z.", "Position des Vertex als x, y, z."), subset="brep")
R("semio", "create-vertex", "tol", brep_tolerance("the vertex", "des Vertex"), subset="brep")
R("semio", "create-face", "tol", brep_tolerance("the face", "der Fläche"), subset="brep")
R("semio", "create-edge", "tol", brep_tolerance("the edge", "der Kante"), subset="brep")
R("semio", "create-shell", "faces", REC("Faces", "Flächen", "Faces of the shell, each with its orientation.", "Flächen der Schale, jeweils mit ihrer Orientierung."), subset="brep")
#endregion 🔖️SemioBrep

#region 🔖️WindowConfig
WINDOW_CHANGE = DISC("Change", "Änderung", "Names the window setting this change writes; fixed for the leaf.", "Benennt die Fenstereinstellung, die diese Änderung schreibt; für das Blatt fest.")
CAMERA = REC("Camera", "Kamera", "Pan offset and zoom factor of the view; cleared, the view fits its content.", "Verschiebung und Zoomfaktor der Ansicht; geleert passt sich die Ansicht ihrem Inhalt an.")
CAMERA_AXIS = {"widget": "stepper", "role": "value", "step": 1, "precision": 1}
CAMERA_ZOOM = {"widget": "slider", "role": "value", "label": L("Zoom", "Zoom"), "step": 0.05, "precision": 2, "softMin": 0.1, "softMax": 8, "scale": "log", "snaps": [0.25, 0.5, 1, 2, 4]}


def camera_members(prefix):
    return {f"{prefix}properties/x": {**CAMERA_AXIS, "label": L("Pan X", "Verschiebung X")}, f"{prefix}properties/y": {**CAMERA_AXIS, "label": L("Pan Y", "Verschiebung Y")}, f"{prefix}properties/zoom": CAMERA_ZOOM}


LOD_MODE = TXT("Detail level", "Detailstufe", "Level-of-detail mode of the canvas, as the window names it.", "Detailstufenmodus der Leinwand, wie ihn das Fenster benennt.")
#endregion 🔖️WindowConfig

#region 🔖️Writer
R("writer", "set-editor-settings set-camera set-engagement-input set-editor-selection set-lint-generation", "kind", WINDOW_CHANGE)
R("writer", "set-camera", "camera", REC("Camera", "Kamera", "Pan offset and zoom factor of the document view.", "Verschiebung und Zoomfaktor der Dokumentansicht."), nested=camera_members(""))
R("writer", "set-engagement-input", "value", TXT("Input", "Eingabe", "Draft text of the window's engagement input.", "Entwurfstext der Interaktionseingabe des Fensters."))
R("writer", "set-lint-generation", "value", INT("Lint generation", "Prüfstand", "Counter of the lint run whose findings the window shows.", "Zähler des Prüflaufs, dessen Befunde das Fenster zeigt."))
R("writer", "edit-text", "text", MULTI("Document text", "Dokumenttext", "The full text of the document after the edit.", "Der vollständige Text des Dokuments nach der Bearbeitung."))
R("writer", "rename-writer", "newId", TXT("New name", "Neuer Name", "Name the document carries afterwards.", "Name, den das Dokument danach trägt."))
#endregion 🔖️Writer

#region 🔖️Trinity
R("rewriting", "set-camera set-lod-mode", "kind", WINDOW_CHANGE)
R("rewriting", "set-camera", "camera", CAMERA, nested=camera_members("anyOf/1/"))
R("rewriting", "set-lod-mode", "value", LOD_MODE)
R("rewriting", "edit-lhs", "newLhs", REC("Left-hand side", "Linke Regelseite", "Pattern and where-clause the rule matches.", "Muster und Where-Klausel, die die Regel abgleicht."))
R("rewriting", "edit-rhs", "newRhs", REC("Right-hand side", "Rechte Regelseite", "Create, delete, set and merge clauses and the parameters the rule applies.", "Create-, Delete-, Set- und Merge-Klauseln sowie die Parameter, die die Regel anwendet."))
R("rewriting", "change-rule-layout", "key", KEY("Layout point", "Layoutpunkt", "Key of the rule layout point that is placed (the pattern variable it belongs to).", "Schlüssel des Layoutpunkts der Regel, der platziert wird (die Mustervariable, zu der er gehört)."))
R("rewriting", "remove-rule-layout", "key", KEY("Layout point", "Layoutpunkt", "Key of the rule layout point that is removed.", "Schlüssel des Layoutpunkts der Regel, der entfernt wird."))
R("rewriting", "change-parameter", "key", KEY("Parameter", "Parameter", "Name of the parameter whose binding changes.", "Name des Parameters, dessen Bindung sich ändert."))
R("rewriting", "remove-parameter-binding", "key", KEY("Parameter", "Parameter", "Name of the parameter whose binding is removed.", "Name des Parameters, dessen Bindung entfernt wird."))
R("jack", "set-camera set-lod-mode replace-query-result", "kind", WINDOW_CHANGE)
R("jack", "set-camera", "camera", CAMERA, nested=camera_members("anyOf/1/"))
R("jack", "set-lod-mode", "value", LOD_MODE)
R("jack", "set-query", "value", MULTI("Query", "Abfrage", "Source text of the Jack query.", "Quelltext der Jack-Abfrage."))
#endregion 🔖️Trinity

#region 🔖️Vcs
R("vcs", "rename-vcs", "newTitle", TXT("New title", "Neuer Titel", "Title the history view carries afterwards.", "Titel, den die Verlaufsansicht danach trägt."))
R("vcs", "add-tag", "tag", TXT("Tag", "Tag", "Name of the tag that is added.", "Name des Tags, das hinzugefügt wird."))
R("vcs", "remove-tag", "tag", ENT("tag", "Tag", "Tag", "Name of the tag that is removed.", "Name des Tags, das entfernt wird."))
#endregion 🔖️Vcs

#region 🔖️RewritingRule
REWRITING = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json"
RULE_POINT = {"widget": "stepper", "role": "value", "step": 1, "precision": 2}


def rule_text(en, de, d_en, d_de, widget="text"):
    return {"widget": widget, "label": L(en, de), "description": L(d_en, d_de)}


def rule_part(en, de, d_en, d_de):
    return {"label": L(en, de), "description": L(d_en, d_de)}


R("rewriting", "change-rule-layout", "newPoint", None, nested={"properties/x": {**RULE_POINT, "label": L("X", "X")}, "properties/y": {**RULE_POINT, "label": L("Y", "Y")}})
D(REWRITING, "properties/lhs/properties/pattern", rule_part("Pattern", "Muster", "The node, edge and node shape the rule looks for.", "Die Form aus Knoten, Kante und Knoten, nach der die Regel sucht."))
D(REWRITING, "properties/lhs/properties/whereClause", rule_text("Where clause", "Where-Klausel", "Condition over the pattern's variables a match must satisfy.", "Bedingung über die Variablen des Musters, die ein Treffer erfüllen muss.", widget="multiline"))
D(REWRITING, "$defs/Pattern/properties/leftVar", rule_text("Left variable", "Linke Variable", "Name the left node is bound to.", "Name, an den der linke Knoten gebunden wird."))
D(REWRITING, "$defs/Pattern/properties/leftKind", rule_text("Left node kind", "Linke Knotenart", "Kind the left node has.", "Art, die der linke Knoten hat."))
D(REWRITING, "$defs/Pattern/properties/edgeVar", rule_text("Edge variable", "Kantenvariable", "Name the edge is bound to; without one the pattern is a single node.", "Name, an den die Kante gebunden wird; ohne ihn ist das Muster ein einzelner Knoten."))
D(REWRITING, "$defs/Pattern/properties/edgeKind", rule_text("Edge kind", "Kantenart", "Kind the edge has.", "Art, die die Kante hat."))
D(REWRITING, "$defs/Pattern/properties/rightVar", rule_text("Right variable", "Rechte Variable", "Name the right node is bound to.", "Name, an den der rechte Knoten gebunden wird."))
D(REWRITING, "$defs/Pattern/properties/rightKind", rule_text("Right node kind", "Rechte Knotenart", "Kind the right node has.", "Art, die der rechte Knoten hat."))
D(REWRITING, "properties/rhs/properties/create", rule_part("Create", "Erzeugen", "Patterns the rule adds to the graph.", "Muster, die die Regel dem Graphen hinzufügt."))
D(REWRITING, "properties/rhs/properties/delete", rule_part("Delete", "Löschen", "Variables whose nodes or edges the rule removes.", "Variablen, deren Knoten oder Kanten die Regel entfernt."))
D(REWRITING, "properties/rhs/properties/set", rule_part("Set", "Setzen", "Property assignments the rule performs.", "Eigenschaftszuweisungen, die die Regel ausführt."))
D(REWRITING, "properties/rhs/properties/set/items/properties/var", rule_text("Variable", "Variable", "Variable whose property is set.", "Variable, deren Eigenschaft gesetzt wird."))
D(REWRITING, "properties/rhs/properties/set/items/properties/prop", rule_text("Property", "Eigenschaft", "Name of the property that is set.", "Name der Eigenschaft, die gesetzt wird."))
D(REWRITING, "properties/rhs/properties/merge", rule_part("Merge", "Zusammenführen", "Patterns the rule matches, or creates when they are absent.", "Muster, die die Regel abgleicht oder erzeugt, wenn sie fehlen."))
D(REWRITING, "properties/rhs/properties/parameters", rule_part("Parameters", "Parameter", "Named values the rule takes when it is applied.", "Benannte Werte, die die Regel bei der Anwendung erhält."))
D(REWRITING, "properties/rhs/properties/parameters/items/properties/name", rule_text("Name", "Name", "Name the parameter is referenced by in the rule.", "Name, über den der Parameter in der Regel referenziert wird."))
D(REWRITING, "properties/rhs/properties/parameters/items/properties/kind", {"widget": "segmented", "label": L("Type", "Typ"), "description": L("Value type of the parameter.", "Werttyp des Parameters."), "options": {"string": L("Text", "Text"), "number": L("Number", "Zahl"), "boolean": L("Boolean", "Wahrheitswert")}})
#endregion 🔖️RewritingRule

#region 🔖️Amendments
PIXEL = {"widget": "stepper", "unit": "px", "step": 1, "precision": 0}
A("avi", "index streamIndex")
A("binary", "offset remove_len")
A("csv", "fieldIndex index recordIndex")
A("deflate", "dict_id method window_bits")
A("docx", "index")
A("dwg", "codepage maintenanceVersion")
A("dxf", "index")
A("epw", "fieldIndex index recordIndex")
A("gif", "delayCs from to height width left top index loopCount ratio transparentIndex")
A("gltf", "accessor buffer byteLength byteOffset camera child count index material mesh mode node parent position primitive scene skin target")
A("html", "index")
A("ifc", "building id index placement product project storey units")
A("ifc", "elevation", {"step": 0.1})
A("jpg", "hSampling vSampling marker quality restartInterval xDensity yDensity")
A("jpg", "precision", {"step": 1, "snaps": [8, 12]})
A("las", "dayOfYear major minor")
A("mp4", "trackIndex")
A("ply", "rowIndex")
A("png", "x y width height", PIXEL)
A("png", "red green blue alpha", {"widget": "slider", "step": 1, "precision": 0})
A("png", "gama", {"widget": "stepper", "step": 1, "precision": 0, "snaps": [45455, 55556, 100000]})
A("pptx", "destinationIndex", {"widget": "stepper", "step": 1, "precision": 0})
A("semio", "colorIndex level pts row_index to_index vertex_index delay_ms")
A("semio", "bit_depth", {"step": 1, "snaps": [1, 2, 4, 8, 16]})
A("semio", "sampleRate", {"step": 1, "snaps": [8000, 16000, 22050, 44100, 48000, 96000]})
A("semio", "new_metallic new_roughness", {"widget": "slider", "step": 0.01, "precision": 2, "softMin": 0, "softMax": 1}, force=("widget",))
A("semio", "new_width", {"step": 0.25, "precision": 2, "softMin": 0})
A("semio", "t", {"step": 0.01, "precision": 3})
A("step", "argIndex")
A("tiff", "compression ifdIndex photometric")
A("tiff", "tileLength tileWidth", {"step": 16})
A("tsv", "fieldIndex rowIndex")
A("wav", "moveTo removeCount")
A("pdf", "userUnit", {"step": 1, "precision": 2, "softMin": 1})
CHANNEL = {"widget": "slider", "step": 1, "precision": 0}
A("bmp", "x y width height", PIXEL)
A("bmp", "red green blue alpha", CHANNEL)
A("bmp", "paletteIndex", {"widget": "stepper", "step": 1, "precision": 0})
A("tiff", "x y width height", PIXEL)
A("tiff", "red green blue alpha", CHANNEL)
R("tiff", "paint-region", "ifdIndex", {"widget": "stepper", "step": 1, "precision": 0}, amend=True)
#endregion 🔖️Amendments

#region 🔖️RecordMembers
STEP1 = {"widget": "stepper", "step": 1, "precision": 0}
BYTES = {"widget": "hidden"}
INDEX_LIST = {"step": 1, "precision": 0}
UNIT_SLIDER = {"widget": "slider", "step": 0.01, "precision": 2, "softMin": 0, "softMax": 1}
CHANNEL_255 = {"widget": "slider", "step": 1, "precision": 0}


def named(en, de, **facets):
    return {"label": L(en, de), **facets}


def one(en=None, de=None, **facets):
    """🔢️ An integer that moves by one — a count, index, id, size or code — with the facets its meaning adds."""
    return {**({"label": L(en, de)} if en else {}), **STEP1, **facets}


def real(step, precision, en=None, de=None, **facets):
    return {**({"label": L(en, de)} if en else {}), "widget": "stepper", "step": step, "precision": precision, **facets}


XYZ = {"x": ("X", "X"), "y": ("Y", "Y"), "z": ("Z", "Z"), "w": ("W", "W")}


def axes(artifact, record, names, step, precision, **facets):
    for name in names.split():
        N(artifact, record, name, real(step, precision, *XYZ[name], **facets))


N("avi", "AviMainHeader", "microSecPerFrame", one(softMin=1, snaps=[16667, 20000, 33333, 33367, 40000, 41667, 41708]))
N("avi", "AviMainHeader", "maxBytesPerSec paddingGranularity flags totalFrames initialFrames streams suggestedBufferSize", one())
N("avi", "AviMainHeader", "width", one(softMax=7680, snaps=VIDEO_WIDTHS))
N("avi", "AviMainHeader", "height", one(softMax=4320, snaps=VIDEO_HEIGHTS))
N("avi", "AviMainHeader", "reserved", BYTES)
N("avi", "AviStreamHeader", "flags priority language initialFrames scale rate start length suggestedBufferSize sampleSize rcFrameLeft rcFrameTop rcFrameRight rcFrameBottom rcFrameWidth", one())
N("avi", "AviStreamHeader", "quality", one(softMin=-1, softMax=10000))
N("avi", "AviStreamHeader", "strhExtra", BYTES)
N("avi", "AviChunk", "data", BYTES)
N("avi", "RiffChunk", "data", BYTES)
N("bcf", "BcfViewpoint", "snapshot", BYTES)
N("docx", "DocxBlockPath", "index", one())
N("docx", "DocxPathSegment", "blockIndex row cell", one())
N("dxf", "DxfHeaderVar", "groupCode", one(softMin=0, softMax=1071))
N("dxf", "DxfLayer", "color", one(softMin=-255, softMax=255))
N("dxf", "DxfLayer", "flags", one(softMin=0, softMax=255))
N("dxf", "DxfLinetype", "flags", one(softMin=0, softMax=255))
N("dxf", "DxfStyle", "flags", one(softMin=0, softMax=255))
N("epw", "EpwDataPeriods", "recordsPerHour", one(softMin=1, softMax=60, snaps=[1, 2, 4, 6, 12, 60]))
N("gif", "GifFrame", "left top width height transparentIndex", one())
N("gif", "GifFrame", "delayCs", one(softMax=1000))
N("gif", "GifFrame", "indices", BYTES)
N("gif", "GifImage", "left top width height", one())
N("gif", "GifImage", "indices", BYTES)
N("gif", "GifPlainText", "left top width height cellWidth cellHeight fgColorIndex bgColorIndex", one())
N("gif", "GifRgb", "r g b", CHANNEL_255, force=("widget",))
N("gif", "GifAppExtension", "data identifier", BYTES)
N("ifc", "CobieSpaceRow", "placement", one())
N("ifc", "CobieTypeAssignment", "ownerHistory relatingType", one())
N("ifc", "CobieTypeAssignment", "relatedObjects", INDEX_LIST)
N("ifc", "IfcEntity", "id", one())
N("ifc", "Part21Instance", "id", one())
N("ifc", "SavAnalysisModel", "ownerHistory", one())
N("ifc", "SavGroupAssignment", "ownerHistory relatingGroup", one())
N("ifc", "SavGroupAssignment", "relatedObjects", INDEX_LIST)
N("ifc", "SavLoadGroup", "ownerHistory", one())
N("jpg", "JfifThumbnail", "width", one("Width", "Breite", unit="px"))
N("jpg", "JfifThumbnail", "height", one("Height", "Höhe", unit="px"))
N("jpg", "JfifThumbnail", "rgbData", BYTES)
N("jpg", "JpgFrameComponent", "id", one("Component id", "Komponentenkennung"))
N("jpg", "JpgFrameComponent", "hSampling vSampling", one(softMin=1, softMax=4))
N("jpg", "JpgFrameComponent", "quantTableId", one(softMax=3))
N("jpg", "JpgHuffmanTable", "id", one("Table id", "Tabellenkennung", softMax=3))
N("jpg", "JpgHuffmanTable", "bits values", INDEX_LIST)
N("jpg", "JpgHuffmanTableKey", "id", one("Table id", "Tabellenkennung", softMax=3))
N("jpg", "JpgQuantTable", "id", one("Table id", "Tabellenkennung", softMax=3))
N("jpg", "JpgQuantTable", "precision", one(snaps=[0, 1]))
N("jpg", "JpgQuantTable", "values", INDEX_LIST)
N("jpg", "JpgSegment", "marker", one())
N("jpg", "JpgSegment", "data", BYTES)
axes("las", "LasPoint", "x y z", 0.01, 3)
N("las", "LasPoint", "intensity userData pointSourceId", one())
N("las", "LasPoint", "returnNumber numberOfReturns", one(softMin=1, softMax=5))
N("las", "LasPoint", "classification", one(softMax=31))
N("las", "LasPoint", "scanAngleRank", one(softMin=-90, softMax=90))
N("las", "LasPoint", "gpsTime", real(0.001, 6))
N("las", "LasVlr", "recordId", one())
N("las", "LasVlr", "data", BYTES)
N("mp3", "Id3Frame", "flags", one())
N("mp3", "Id3Frame", "data", BYTES)
N("mp3", "Id3v1Tag", "raw", BYTES)
N("mp3", "Id3v2Tag", "majorVersion", one(snaps=[2, 3, 4]))
N("mp3", "Id3v2Tag", "minorVersion flags", one())
N("mp3", "Mp3Frame", "payload", BYTES)
N("mp3", "Mp3FrameHeader", "mpegVersionId channelMode modeExtension emphasis", one(softMax=3))
N("mp3", "Mp3FrameHeader", "layer", one("MPEG layer", "MPEG-Layer", softMin=1, softMax=3))
N("mp3", "Mp3FrameHeader", "bitrateIndex", one(softMax=15))
N("mp3", "Mp3FrameHeader", "sampleRateIndex", one(softMax=3))
N("mp4", "Mp4AvcExtension", "chromaFormat", one(softMax=3))
N("mp4", "Mp4AvcExtension", "bitDepthLumaMinus8 bitDepthChromaMinus8", one(softMax=8))
N("mp4", "Mp4AvcExtension", "spsExt", BYTES)
N("mp4", "Mp4Bitrate", "bufferSize maximum average", one())
N("mp4", "Mp4Codec", "nalLengthSize", one(snaps=[1, 2, 4]))
N("mp4", "Mp4Codec", "sps pps", BYTES)
N("mp4", "Mp4Color", "primaries transfer matrix", one())
N("mp4", "Mp4Edit", "segmentDuration mediaTime mediaRateInteger mediaRateFraction", one())
N("mp4", "Mp4Ftyp", "minorVersion", one())
N("mp4", "Mp4HevcConfig", "generalProfileSpace generalProfileIdc generalProfileCompatibilityFlags generalConstraintIndicatorFlags generalLevelIdc minSpatialSegmentationIdc parallelismType avgFrameRate constantFrameRate numTemporalLayers", one())
N("mp4", "Mp4HevcConfig", "chromaFormatIdc", one(softMax=3))
N("mp4", "Mp4HevcConfig", "bitDepthLumaMinus8 bitDepthChromaMinus8", one(softMax=8))
N("mp4", "Mp4HevcNalArray", "nalUnitType", one(softMax=63))
N("mp4", "Mp4HevcNalArray", "nalUnits", BYTES)
N("mp4", "Mp4PixelAspectRatio", "horizontalSpacing verticalSpacing", one(softMin=1))
N("mp4", "Mp4Sample", "duration ctsOffset", one())
N("mp4", "Mp4Sample", "data", BYTES)
N("mp4", "Mp4Track", "trackId timescale", one(softMin=1))
N("mp4", "Mp4Track", "width", one("Width", "Breite", unit="px", softMax=7680, snaps=VIDEO_WIDTHS))
N("mp4", "Mp4Track", "height", one("Height", "Höhe", unit="px", softMax=4320, snaps=VIDEO_HEIGHTS))
N("mp4", "Mp4Track", "chunkSampleCounts", INDEX_LIST)
N("mp4", "Mp4TrackMetadata", "creationTime modificationTime flags duration alternateGroup volume mediaDuration mediaCreationTime mediaModificationTime quality", one())
N("mp4", "Mp4TrackMetadata", "layer", one("Layer", "Ebene"))
N("mp4", "Mp4TrackMetadata", "matrix", INDEX_LIST)
N("mp4", "Mp4VisualSampleEntry", "dataReferenceIndex revisionLevel vendor temporalQuality spatialQuality horizontalResolution verticalResolution frameCount colorTableId", one())
N("mp4", "Mp4VisualSampleEntry", "version", one("Version", "Version"))
N("mp4", "Mp4VisualSampleEntry", "depth", one(snaps=[24, 32]))
N("obj", "ObjFaceVertex", "vertex texcoord", one(softMin=1))
N("obj", "ObjFaceVertex", "normal", one("Normal number", "Normalennummer", softMin=1))
axes("obj", "ObjNormal", "x y z", 0.01, 6, softMin=-1, softMax=1)
N("obj", "ObjSmoothingRange", "faceIndexFrom group", one())
N("obj", "ObjTexCoord", "u v w", real(0.01, 6))
N("obj", "ObjUnknownStatement", "lineIndex", one())
N("obj", "ObjUsemtlRange", "faceIndexFrom", one())
axes("obj", "ObjVertex", "x y z", 0.1, 6)
N("obj", "ObjVertex", "w", real(0.1, 6))
N("pdf", "ObjRef", "num gen", one())
N("pdf", "PageDoc", "width", {**PDF_WIDTH, "role": "value"})
N("pdf", "PageDoc", "height", {**PDF_HEIGHT, "role": "value"})
N("pdf", "PdfAcroForm", "signatureFlags", one(softMax=3))
N("pdf", "PdfAcroForm", "quadding", one(snaps=[0, 1, 2]))
N("pdf", "PdfAnnotation", "flags structParent", one())
N("pdf", "PdfAnnotation", "color", {"step": 0.01, "precision": 2})
N("pdf", "PdfBorderStyle", "width", real(0.25, 2, "Border width", "Rahmenbreite", unit="pt", softMin=0))
N("pdf", "PdfBorderStyle", "dash", {"step": 0.5, "precision": 2, "unit": "pt"})
N("pdf", "PdfDate", "year", one("Year", "Jahr", softMin=1970, softMax=2100))
N("pdf", "PdfDate", "month", one(softMin=1, softMax=12))
N("pdf", "PdfDate", "day", one(softMin=1, softMax=31))
N("pdf", "PdfDate", "hour", one(softMax=23))
N("pdf", "PdfDate", "minute second", one(softMax=59))
N("pdf", "PdfDate", "offsetMinutes", {"widget": "stepper", "step": 15, "precision": 0, "softMin": -720, "softMax": 840})
N("pdf", "PdfEmbeddedFile", "data", BYTES)
N("pdf", "PdfEncryption", "permissions", one())
N("pdf", "PdfExtGState", "lineWidth", real(0.25, 2, unit="pt", softMin=0))
N("pdf", "PdfExtGState", "miterLimit", real(0.5, 2, softMin=1))
N("pdf", "PdfExtGState", "overprintMode", one(snaps=[0, 1]))
N("pdf", "PdfExtGState", "strokeAlpha fillAlpha smoothness", UNIT_SLIDER, force=("widget",))
N("pdf", "PdfExtGState", "flatness", real(1, 1, softMin=0, softMax=100))
N("pdf", "PdfFormField", "flags", one())
N("pdf", "PdfFormField", "quadding", one(snaps=[0, 1, 2]))
N("pdf", "PdfFormXObject", "structParent", one())
N("pdf", "PdfFormXObject", "matrix", {"step": 0.01, "precision": 4})
N("pdf", "PdfImage", "width", one("Width", "Breite", unit="px"))
N("pdf", "PdfImage", "height", one("Height", "Höhe", unit="px"))
N("pdf", "PdfImage", "bitsPerComponent", one(snaps=[1, 2, 4, 8, 16]))
N("pdf", "PdfImage", "softMaskInData", one(snaps=[0, 1, 2]))
N("pdf", "PdfImage", "structParent", one())
N("pdf", "PdfImage", "data", BYTES)
N("pdf", "PdfImage", "decode matte", {"step": 0.01, "precision": 2})
N("pdf", "PdfMarkupAnnotation", "popup inReplyTo", one())
N("pdf", "PdfMarkupAnnotation", "opacity", {"label": L("Opacity", "Deckkraft"), **UNIT_SLIDER})
N("pdf", "PdfOutputIntent", "profile", BYTES)
N("pdf", "PdfPage", "userUnit", real(1, 2, softMin=1))
N("pdf", "PdfPage", "structParents", one())
N("pdf", "PdfPage", "duration", real(0.5, 1, softMin=0))
N("pdf", "PdfPageLabelRange", "startIndex start", one())
N("pdf", "PdfPattern", "matrix", {"step": 0.01, "precision": 4})
N("pdf", "PdfShading", "background", {"step": 0.01, "precision": 2})
N("pdf", "PdfToUnicode", "byteWidth", one(snaps=[1, 2]))
N("pdf", "PdfViewerPreferences", "numCopies", one(softMin=1))
N("pdf", "PdfViewerPreferences", "printPageRange", INDEX_LIST)
N("ply", "PlyElement", "count", one("Row count", "Zeilenanzahl"))
N("pptx", "PptxXmlVacancyAddress", "index", one())
N("pptx", "PptxXmlAddress", "nodePath", INDEX_LIST)
N("semio", "AnimKeyframe", "t", real(0.01, 3, softMin=0))
N("semio", "CadLayer", "colorIndex", one(softMin=0, softMax=255))
N("semio", "DocBlockPath", "index", one("Block", "Block"))
N("semio", "DocImage", "bytes", BYTES)
N("semio", "DrawNode", "width", real(1, 2, "Width", "Breite", softMin=0))
N("semio", "DrawNode", "height", real(1, 2, "Height", "Höhe", softMin=0))
N("semio", "NodePath", "layer", one("Layer", "Ebene"))
N("semio", "NodePath", "path", {"label": L("Child path", "Kindpfad"), **INDEX_LIST})
N("semio", "PathSegment", "rx ry", real(1, 2, softMin=0))
N("semio", "PathSegment", "xRotation", {"widget": "dial", "step": 1, "precision": 1, "softMin": 0, "softMax": 360, "snaps": [0, 15, 30, 45, 60, 75, 90, 105, 120, 135, 150, 165, 180, 195, 210, 225, 240, 255, 270, 285, 300, 315, 330, 345, 360]}, force=("widget",))
axes("semio", "Point3", "x y z", 0.1, 3)
axes("semio", "Quaternion", "x y z w", 0.01, 4, softMin=-1, softMax=1)
N("semio", "RunStyle", "size", real(0.5, 1, unit="pt", softMin=1, softMax=144, snaps=[8, 9, 10, 11, 12, 14, 16, 18, 24, 36, 48, 72]))
N("semio", "SemioAudioChannel", "samples", BYTES)
N("semio", "SemioImageFrame", "delayMs", one(softMax=10000))
N("semio", "SemioImageFrame", "rgba8", BYTES)
N("semio", "SemioMaterial", "metallic roughness", UNIT_SLIDER, force=("widget",))
axes("semio", "SemioPoint2", "x y", 1, 2)
axes("semio", "SemioPoint3", "x y z", 0.1, 3)
axes("semio", "SemioQuaternion", "x y z w", 0.01, 4, softMin=-1, softMax=1)
N("semio", "SemioPrimitive", "indices", BYTES)
N("semio", "SemioRational", "num den", one())
N("semio", "SemioRgba", "r g b a", UNIT_SLIDER, force=("widget",))
N("semio", "SemioTexture", "bytes", BYTES)
N("semio", "SemioUv", "u v", real(0.01, 4, softMin=0, softMax=1))
N("semio", "SemioVideoSample", "pts", one())
N("semio", "SemioVideoSample", "data", BYTES)
N("semio", "SemioVideoStream", "width", one("Width", "Breite", unit="px", softMax=7680, snaps=VIDEO_WIDTHS))
N("semio", "SemioVideoStream", "height", one("Height", "Höhe", unit="px", softMax=4320, snaps=VIDEO_HEIGHTS))
N("semio", "SlideFrame", "width", real(1, 2, "Width", "Breite", softMin=0))
N("semio", "SlideFrame", "height", real(1, 2, "Height", "Höhe", softMin=0))
N("semio", "SlidePictureImage", "bytes", BYTES)
N("step", "ProductIdentity", "product", one("Product", "Produkt"))
N("step", "ProductIdentity", "formation definition", one())
N("step", "ShapeRepresentationRow", "context", one())
N("step", "ShapeRepresentationRow", "items", INDEX_LIST)
N("step", "StepEntity", "id", one("Instance name", "Instanzname"))
N("svg", "ViewBox", "minX minY", real(1, 2))
N("svg", "ViewBox", "width", real(1, 2, "Width", "Breite", softMin=0))
N("svg", "ViewBox", "height", real(1, 2, "Height", "Höhe", softMin=0))
N("tiff", "TiffTag", "tag", one("Tag", "Tag"))
N("tiff", "TiffStorage", "chunks", BYTES)
N("wav", "RiffChunk", "padByte", one(snaps=[0]))
N("wav", "RiffChunk", "data", BYTES)
N("wav", "WavFmt", "audioFormat", one(snaps=[1, 3, 6, 7, 65534]))
N("wav", "WavFmt", "channels", one(softMin=1, softMax=8))
N("wav", "WavFmt", "sampleRate", one(snaps=[8000, 16000, 22050, 44100, 48000, 96000, 192000]))
N("wav", "WavFmt", "byteRate blockAlign", one())
N("wav", "WavFmt", "bitsPerSample", one(snaps=[8, 16, 24, 32]))
N("wav", "WavFmt", "ext", BYTES)
N("xlsx", "XlsxCell", "row", one("Row", "Zeile"))
N("xlsx", "XlsxCell", "col", one())
N("zip", "ZipCentralHeaderMetadata", "versionMadeBy versionNeeded flags modifiedTime modifiedDate internalAttributes externalAttributes", one())
N("zip", "ZipCentralHeaderMetadata", "unicodeCommentLegacy unicodePathLegacyName", BYTES)
N("zip", "ZipEntryMetadata", "compressionMethod", one(snaps=[0, 8]))
N("zip", "ZipEntry", "data", BYTES)
N("zip", "ZipExtraField", "id", one())
N("zip", "ZipExtraField", "data", BYTES)
N("zip", "ZipLocalHeaderMetadata", "versionNeeded flags modifiedTime modifiedDate", one())
N("zip", "ZipLocalHeaderMetadata", "unicodePathLegacyName", BYTES)
#endregion 🔖️RecordMembers
