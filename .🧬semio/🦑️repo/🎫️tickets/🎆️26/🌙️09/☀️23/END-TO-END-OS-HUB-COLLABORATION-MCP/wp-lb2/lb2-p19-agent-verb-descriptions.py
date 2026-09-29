#!/usr/bin/env python3
"""💬️ LB2 p19 (stdio): every stdio verb an agent can be offered explains itself in English and German, and every
remove/replace-class stdio verb asks a human first.

Measured (S19 `os-mcp` `search::long`, `.🧬semio/🌐hub/s14-s19-logs/mcp-search-long-1.txt`, 2026-09-29 20:02): 1 178 agent verbs
declare no `semantics.description` (1 115 stdio: the six snapshot edits × 88 editors, pdf's page verbs × 10 editors, the
structural table verbs, wav's audio verbs, semio's `set-vertex`), and 338 remove/replace/set-snapshot-class verbs publish
`effects.destructive = false`, so `ApprovalMode::WhenDestructive` never fires for them.

Every stdio verb is declared in ONE place, so the fix is one edit per declaration site, never per editor:
- stdio contract `✏️editing` `snapshot_edit_actions` (6): descriptions. `setSnapshotValue` (the empty path replaces the whole
  document), `removeSnapshotValue` and `replaceSnapshotSource` become `.destructive()`.
- stdio contract `structural_table_window_kind` (5): descriptions. `remove-row` and `remove-column` become destructive.
- wav `edit-audio` `extra_actions` (3), semio mesh and brep `set_vertex_action` (1 each): descriptions.
- pdf 1.7 page window `window_definition` (every agent verb without a description): descriptions. `delete`, `remove-page`,
  `remove-embedded-file`, `remove-named-destination` and `set-document-id` (overwrites the trailer `/ID`, the file's
  identity) become destructive.
Each rewritten element is formatted by rustfmt itself (the repo `rustfmt.toml`, inside a dummy with the element's own
indentation), so the patch is rustfmt-stable. The law is the existing `os-mcp` `search::long` census over the committed
descriptors: it passes for stdio once describe regenerates the stdio descriptors (runbook). The pre-proof is
`lb2-p19-census.py` (python-jsonschema over the manifest schema's `CapabilityDescription`, plus the lexicon rules) run
against the descriptors that stdio describes natively.

usage: python3 lb2-p19-agent-verb-descriptions.py --dry-run | --write | --revert [--root <tree>]
Backups (byte-exact, per root) under `.🧬semio/🌐hub/s14-lb2-backup/p19/<root-hash>/`.
"""
import hashlib, os, re, shutil, subprocess, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p19/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
RUSTFMT_CONFIG = "/Users/ueli/Documents/semio/rustfmt.toml"
EDITING = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs"
CONTRACT = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs"
WAV = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎮️commands/🔊️edit-audio/🦀️.rs"
MESH = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🦀️.rs"
BREP = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🦀️.rs"
PDF = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🦀️.rs"
problems = []

#region Descriptions
SNAPSHOT = {
    "SET_SNAPSHOT_VALUE_ACTION_ID": (
        "Replaces the value at the addressed path of the document (a JSON pointer or path segments; the empty path is the whole document) with the given value; the replaced value remains available through undo.",
        "Ersetzt den Wert am adressierten Pfad des Dokuments (JSON-Pointer oder Pfadsegmente; der leere Pfad ist das ganze Dokument) durch den angegebenen Wert; der ersetzte Wert bleibt über Rückgängig verfügbar.",
        True,
    ),
    "INSERT_SNAPSHOT_VALUE_ACTION_ID": (
        "Inserts the given value at the addressed path as a new array element or a new object member, leaving every existing value in place.",
        "Fügt den angegebenen Wert am adressierten Pfad als neues Array-Element oder neues Objektmitglied ein und lässt alle vorhandenen Werte unverändert.",
        False,
    ),
    "REMOVE_SNAPSHOT_VALUE_ACTION_ID": (
        "Removes the value at the addressed path from the document together with everything nested inside it; the removed content remains available through undo.",
        "Entfernt den Wert am adressierten Pfad samt allem, was darin verschachtelt ist, aus dem Dokument; der entfernte Inhalt bleibt über Rückgängig verfügbar.",
        True,
    ),
    "MOVE_SNAPSHOT_VALUE_ACTION_ID": (
        "Moves the value found at the source path to the target path of the same document, taking it away from where it was.",
        "Verschiebt den Wert vom Quellpfad an den Zielpfad desselben Dokuments und nimmt ihn an seiner bisherigen Stelle weg.",
        False,
    ),
    "RENAME_SNAPSHOT_KEY_ACTION_ID": (
        "Renames the object member at the addressed path to the given key while keeping its value and its position.",
        "Benennt das Objektmitglied am adressierten Pfad in den angegebenen Schlüssel um und behält dabei Wert und Position bei.",
        False,
    ),
    "REPLACE_SNAPSHOT_SOURCE_ACTION_ID": (
        "Replaces the whole document with the given source text in the document's own format; the previous document remains available through undo.",
        "Ersetzt das ganze Dokument durch den angegebenen Quelltext im eigenen Format des Dokuments; das bisherige Dokument bleibt über Rückgängig verfügbar.",
        True,
    ),
}
TABLE = {
    "ADD_TABLE_ROW_ACTION_ID": ("Appends one empty row at the end of the table when the document revision still matches.", "Hängt am Ende der Tabelle eine leere Zeile an, wenn die Dokumentrevision noch übereinstimmt.", False),
    "REMOVE_TABLE_ROW_ACTION_ID": (
        "Removes the addressed row and every cell in it from the table when the document revision still matches; the removed row remains available through undo.",
        "Entfernt die adressierte Zeile samt all ihren Zellen aus der Tabelle, wenn die Dokumentrevision noch übereinstimmt; die entfernte Zeile bleibt über Rückgängig verfügbar.",
        True,
    ),
    "ADD_TABLE_COLUMN_ACTION_ID": ("Appends one empty column at the right end of the table when the document revision still matches.", "Hängt am rechten Ende der Tabelle eine leere Spalte an, wenn die Dokumentrevision noch übereinstimmt.", False),
    "REMOVE_TABLE_COLUMN_ACTION_ID": (
        "Removes the addressed column and its cell in every row from the table when the document revision still matches; the removed column remains available through undo.",
        "Entfernt die adressierte Spalte samt ihrer Zelle in jeder Zeile aus der Tabelle, wenn die Dokumentrevision noch übereinstimmt; die entfernte Spalte bleibt über Rückgängig verfügbar.",
        True,
    ),
    "SET_TABLE_HEADER_ACTION_ID": ("Sets the header text of the addressed column when the document revision still matches.", "Setzt den Kopftext der adressierten Spalte, wenn die Dokumentrevision noch übereinstimmt.", False),
}
WAV_VERBS = {
    "INSERT_FRAME_ACTION_ID": (
        "Inserts one silent sample frame across all channels at the addressed frame position when the document revision still matches.",
        "Fügt an der adressierten Frame-Position einen stillen Sample-Frame über alle Kanäle ein, wenn die Dokumentrevision noch übereinstimmt.",
        False,
    ),
    "INSERT_CHANNEL_ACTION_ID": (
        "Inserts one silent channel at the addressed channel position into every frame when the document revision still matches.",
        "Fügt an der adressierten Kanalposition in jeden Frame einen stillen Kanal ein, wenn die Dokumentrevision noch übereinstimmt.",
        False,
    ),
    "SET_SAMPLE_RATE_ACTION_ID": (
        "Sets the sample rate in hertz the audio plays back at and keeps every sample, so playback speed and pitch change accordingly.",
        "Setzt die Abtastrate in Hertz, mit der das Audio wiedergegeben wird, und behält jedes Sample bei, sodass sich Wiedergabetempo und Tonhöhe entsprechend ändern.",
        False,
    ),
}
VERTEX = {
    MESH: ("Moves the addressed vertex of a mesh primitive to the given target point.", "Verschiebt den adressierten Vertex eines Mesh-Primitivs an den angegebenen Zielpunkt."),
    BREP: ("Moves the addressed vertex of the boundary representation to the given target point; its edges and faces follow it.", "Verschiebt den adressierten Vertex der Begrenzungsflächendarstellung an den angegebenen Zielpunkt; seine Kanten und Flächen folgen ihm."),
}
PDF_DESTRUCTIVE = {"delete", "remove-page", "remove-embedded-file", "remove-named-destination", "set-document-id"}
PDF_VERBS = {
    "set-text": ("Replaces the text shown by the addressed text object on the page with the given text.", "Ersetzt den Text, den das adressierte Textobjekt auf der Seite zeigt, durch den angegebenen Text."),
    "move": ("Moves the addressed page object to the given x and y position in page coordinates.", "Verschiebt das adressierte Seitenobjekt an die angegebene x- und y-Position in Seitenkoordinaten."),
    "resize": ("Places the addressed page object into the given rectangle of position, width and height in page coordinates.", "Setzt das adressierte Seitenobjekt in das angegebene Rechteck aus Position, Breite und Höhe in Seitenkoordinaten."),
    "delete": ("Deletes the addressed object from the page content; the deleted object remains available through undo.", "Löscht das adressierte Objekt aus dem Seiteninhalt; das gelöschte Objekt bleibt über Rückgängig verfügbar."),
    "set-fill": ("Sets the fill color of the addressed page object to the given red, green and blue components.", "Setzt die Füllfarbe des adressierten Seitenobjekts auf die angegebenen Rot-, Grün- und Blauanteile."),
    "set-stroke": ("Sets the outline color and the line width of the addressed page object.", "Setzt Linienfarbe und Linienbreite des adressierten Seitenobjekts."),
    "insert-text": ("Adds a new text object with the given text, font size and color at the given position of the page.", "Fügt an der angegebenen Position der Seite ein neues Textobjekt mit dem angegebenen Text, der Schriftgröße und der Farbe ein."),
    "insert-rectangle": ("Adds a new filled rectangle of the given position, size and color to the page.", "Fügt der Seite ein neues gefülltes Rechteck mit der angegebenen Position, Größe und Farbe hinzu."),
    "insert-line": ("Adds a new straight line from the given start point across the given width and height, in the given color, to the page.", "Fügt der Seite eine neue gerade Linie vom angegebenen Startpunkt über die angegebene Breite und Höhe in der angegebenen Farbe hinzu."),
    "insert-image": ("Adds a new image object of the given position and size to the page.", "Fügt der Seite ein neues Bildobjekt mit der angegebenen Position und Größe hinzu."),
    "set-image": ("Replaces the pixel samples of the addressed image object with the given samples of the given width and height.", "Ersetzt die Bildpunkte des adressierten Bildobjekts durch die angegebenen Bildpunkte mit der angegebenen Breite und Höhe."),
    "insert-page": ("Inserts a new empty page of the given width and height at the given page position.", "Fügt an der angegebenen Seitenposition eine neue leere Seite mit der angegebenen Breite und Höhe ein."),
    "remove-page": ("Removes the addressed page and everything on it from the document; the removed page remains available through undo.", "Entfernt die adressierte Seite samt ihrem gesamten Inhalt aus dem Dokument; die entfernte Seite bleibt über Rückgängig verfügbar."),
    "move-page": ("Moves the addressed page to the given position in the page order.", "Verschiebt die adressierte Seite an die angegebene Position der Seitenreihenfolge."),
    "set-page-size": ("Changes the width and height of the media box of the addressed page.", "Ändert Breite und Höhe der Medienbox der adressierten Seite."),
    "set-info": ("Sets the title, author and subject entries of the document information dictionary.", "Setzt Titel, Autor und Thema im Dokumentinformations-Verzeichnis."),
    "set-info-field": ("Sets one named entry of the document information dictionary to the given text.", "Setzt einen benannten Eintrag des Dokumentinformations-Verzeichnisses auf den angegebenen Text."),
    "set-annotation": ("Sets the contents text and the rectangle of the addressed annotation on the page.", "Setzt Inhaltstext und Rechteck der adressierten Anmerkung auf der Seite."),
    "set-annotation-style": ("Sets one style entry and the color of the addressed annotation, such as its border width or highlight color.", "Setzt einen Stileintrag und die Farbe der adressierten Anmerkung, etwa ihre Rahmenbreite oder Hervorhebungsfarbe."),
    "set-annotation-border": ("Sets the border style, the dash pattern, the corner radii and the border width of the addressed annotation.", "Setzt Rahmenstil, Strichmuster, Eckenradien und Rahmenbreite der adressierten Anmerkung."),
    "set-annotation-markup": ("Sets one markup entry of the addressed annotation, such as its author, subject or reply relation.", "Setzt einen Markup-Eintrag der adressierten Anmerkung, etwa Verfasser, Betreff oder Antwortbezug."),
    "set-form-settings": ("Sets one document-wide setting of the interactive form, such as its default appearance or whether readers regenerate appearances.", "Setzt eine dokumentweite Einstellung des interaktiven Formulars, etwa das Standard-Erscheinungsbild oder ob Lesegeräte Erscheinungsbilder neu erzeugen."),
    "set-extra-entry": ("Sets one additional key of the addressed object dictionary to the given value and keeps every other key.", "Setzt einen zusätzlichen Schlüssel des adressierten Objektverzeichnisses auf den angegebenen Wert und behält alle anderen Schlüssel bei."),
    "set-annotation-kind": ("Changes the subtype of the addressed annotation together with its value, rectangle and color.", "Ändert den Untertyp der adressierten Anmerkung samt Wert, Rechteck und Farbe."),
    "set-field-data": ("Sets one data entry of the addressed form field, such as its flags, options or maximum length.", "Setzt einen Dateneintrag des adressierten Formularfelds, etwa Kennzeichen, Optionen oder Maximallänge."),
    "set-resource-detail": ("Sets one detail of the addressed page resource, such as a shading, pattern or font parameter.", "Setzt ein Detail der adressierten Seitenressource, etwa einen Verlaufs-, Muster- oder Schriftparameter."),
    "set-page-extra": ("Sets one additional entry of the page dictionary of the addressed page to the given value.", "Setzt einen zusätzlichen Eintrag im Seitenverzeichnis der adressierten Seite auf den angegebenen Wert."),
    "set-font": ("Assigns the given font to the addressed text object on the page.", "Weist dem adressierten Textobjekt auf der Seite die angegebene Schrift zu."),
    "set-outline": ("Sets a bookmark with the given title in the document outline that points to the addressed page.", "Setzt in der Dokumentgliederung ein Lesezeichen mit dem angegebenen Titel, das auf die adressierte Seite verweist."),
    "set-page-rotation": ("Sets the display rotation of the addressed page in multiples of 90 degrees.", "Setzt die Anzeigedrehung der adressierten Seite in Vielfachen von 90 Grad."),
    "set-page-box": ("Sets one boundary box of the addressed page, such as its crop, bleed, trim or art box, to the given rectangle.", "Setzt eine Begrenzungsbox der adressierten Seite, etwa Beschnitt-, Anschnitt-, Endformat- oder Objektbox, auf das angegebene Rechteck."),
    "set-page-user-unit": ("Sets the user unit of the addressed page, the size of one page coordinate unit in multiples of 1/72 inch.", "Setzt die Benutzereinheit der adressierten Seite, die Größe einer Seitenkoordinateneinheit in Vielfachen von 1/72 Zoll."),
    "set-language": ("Sets the natural language of the document that readers and assistive technology use.", "Setzt die natürliche Sprache des Dokuments, die Lesegeräte und assistive Technik verwenden."),
    "set-page-layout": ("Sets the page layout a reader uses when it opens the document, such as single page or two columns.", "Setzt das Seitenlayout, das ein Lesegerät beim Öffnen des Dokuments verwendet, etwa Einzelseite oder zwei Spalten."),
    "set-page-mode": ("Sets how a reader opens the document, such as with the outline, with thumbnails or in full screen.", "Legt fest, wie ein Lesegerät das Dokument öffnet, etwa mit Gliederung, mit Miniaturansichten oder im Vollbild."),
    "set-optional-content": ("Creates or updates the addressed optional content layer with the given name and initial visibility.", "Erstellt oder ändert die adressierte optionale Inhaltsebene mit dem angegebenen Namen und der anfänglichen Sichtbarkeit."),
    "set-embedded-file": ("Embeds the given file contents under the given file name in the document, in place of an embedded file with the same key.", "Bettet den angegebenen Dateiinhalt unter dem angegebenen Dateinamen in das Dokument ein, anstelle einer eingebetteten Datei mit demselben Schlüssel."),
    "remove-embedded-file": ("Removes the addressed embedded file from the document; the removed file remains available through undo.", "Entfernt die adressierte eingebettete Datei aus dem Dokument; die entfernte Datei bleibt über Rückgängig verfügbar."),
    "set-named-destination": ("Makes the named destination with the given name point to the addressed page.", "Lässt das benannte Ziel mit dem angegebenen Namen auf die adressierte Seite verweisen."),
    "remove-named-destination": ("Removes the addressed named destination, so links using its name no longer resolve; it remains available through undo.", "Entfernt das adressierte benannte Ziel, sodass Verweise auf seinen Namen nicht mehr aufgelöst werden; es bleibt über Rückgängig verfügbar."),
    "set-page-label": ("Starts a page label range at the addressed page with the given numbering style, prefix and first number.", "Beginnt an der adressierten Seite einen Seitenbeschriftungsbereich mit dem angegebenen Nummerierungsstil, Präfix und der ersten Nummer."),
    "set-mark-info": ("Sets the mark information of the document, which declares whether it is tagged and uses user properties and suspects.", "Setzt die Markierungsinformation des Dokuments, die erklärt, ob es getaggt ist und Benutzereigenschaften und Verdachtsfälle verwendet."),
    "set-metadata": ("Replaces the XMP metadata stream of the document with the given XMP packet.", "Ersetzt den XMP-Metadatenstrom des Dokuments durch das angegebene XMP-Paket."),
    "set-viewer-preferences": ("Sets one viewer preference of the document, such as hiding the toolbar or showing the document title.", "Setzt eine Betrachtereinstellung des Dokuments, etwa das Ausblenden der Werkzeugleiste oder das Anzeigen des Dokumenttitels."),
    "set-encryption": ("Sets the encryption of the document: its algorithm, user password, owner password and permission flags.", "Setzt die Verschlüsselung des Dokuments: Algorithmus, Benutzerpasswort, Besitzerpasswort und Berechtigungskennzeichen."),
    "set-output-intent": ("Sets the output intent of the document: the subtype, identifier and info of the intended print condition.", "Setzt die Ausgabebedingung des Dokuments: Untertyp, Kennung und Information der vorgesehenen Druckbedingung."),
    "set-form-field": ("Sets the value of the named interactive form field of the given kind.", "Setzt den Wert des benannten interaktiven Formularfelds der angegebenen Art."),
    "set-open-action": ("Sets what a reader does when it opens the document, such as going to the addressed page or opening a URI.", "Legt fest, was ein Lesegerät beim Öffnen des Dokuments tut, etwa zur adressierten Seite springen oder eine URI öffnen."),
    "set-document-id": ("Overwrites the permanent and changing parts of the document identifier in the file trailer, by which readers recognize revisions of the same file.", "Überschreibt den dauerhaften und den wechselnden Teil der Dokumentkennung im Dateianhang, an denen Lesegeräte Fassungen derselben Datei erkennen."),
    "set-font-program": ("Replaces the embedded font program of the addressed font with the given program data.", "Ersetzt das eingebettete Schriftprogramm der adressierten Schrift durch die angegebenen Programmdaten."),
    "set-graphics-state": ("Sets the blend mode and the stroke and fill opacity of the addressed graphics state.", "Setzt Mischmodus sowie Linien- und Füllungsdeckkraft des adressierten Grafikzustands."),
    "set-pattern": ("Sets the shading and the tile rectangle of the addressed pattern.", "Setzt Verlauf und Kachelrechteck des adressierten Musters."),
    "set-color-space": ("Sets the addressed color space to the given family, such as DeviceRGB or a named separation color.", "Setzt den adressierten Farbraum auf die angegebene Familie, etwa DeviceRGB oder eine benannte Sonderfarbe."),
    "set-properties": ("Sets one entry of the addressed marked-content property list to the given value.", "Setzt einen Eintrag der adressierten Eigenschaftsliste für markierten Inhalt auf den angegebenen Wert."),
    "set-font-metrics": ("Sets the encoding, the glyph widths and the first and last character codes of the addressed font.", "Setzt Kodierung, Glyphenbreiten sowie ersten und letzten Zeichencode der adressierten Schrift."),
    "set-image-mask": ("Sets the mask of the addressed image, such as a stencil, a color key or a soft mask.", "Setzt die Maske des adressierten Bilds, etwa eine Schablonen-, Farbschlüssel- oder weiche Maske."),
    "set-form-content": ("Replaces the content stream text and the bounding box of the addressed form XObject.", "Ersetzt Inhaltsstromtext und Begrenzungsrahmen des adressierten Formular-XObjects."),
    "set-page-transition": ("Sets the presentation transition shown when a reader enters the addressed page: its style, direction and duration.", "Setzt den Präsentationsübergang beim Aufrufen der adressierten Seite: Stil, Richtung und Dauer."),
    "set-catalog-entry": ("Sets one entry of the document catalog to the given value and keeps every other entry.", "Setzt einen Eintrag des Dokumentkatalogs auf den angegebenen Wert und behält alle anderen Einträge bei."),
    "set-trailer-entry": ("Sets one entry of the file trailer dictionary to the given value and keeps every other entry.", "Setzt einen Eintrag des Dateianhang-Verzeichnisses auf den angegebenen Wert und behält alle anderen Einträge bei."),
    "set-annotation-appearance": ("Sets the appearance stream the addressed annotation is drawn with to the given form.", "Setzt den Erscheinungsbildstrom, mit dem die adressierte Anmerkung gezeichnet wird, auf das angegebene Formular."),
    "set-glyph": ("Sets the drawing procedure and the bounding box of the addressed glyph of a Type 3 font.", "Setzt Zeichenprozedur und Begrenzungsrahmen der adressierten Glyphe einer Type-3-Schrift."),
    "set-indirect-object": ("Sets the indirect object with the given object and generation numbers to the given name value.", "Setzt das indirekte Objekt mit der angegebenen Objekt- und Generationsnummer auf den angegebenen Namenswert."),
    "set-mesh-data": ("Sets the decode ranges and the vertex data of the addressed mesh shading.", "Setzt Dekodierbereiche und Knotendaten des adressierten Netzverlaufs."),
}
SENTENCE = re.compile(r"^[A-ZÄÖÜ0-9].*[.!?]$")
#endregion Descriptions


def once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: expected 1 anchor, found {count}")
        return text
    return text.replace(old, new)


def describe(en, de):
    for cell in (en, de):
        if not SENTENCE.match(cell) or not 24 <= len(cell) <= 480 or "\n" in cell or '"' in cell:
            problems.append(f"description cell is not one sentence line of 24–480 characters: {cell!r}")
    if en == de:
        problems.append(f"untranslated description: {en!r}")
    return f'.describe(LocalizedLabel::native("{en}", "{de}"))'


def match_close(text, index):
    """🔚️ Index just past the bracket that closes the one opening at `index` (string-literal aware)."""
    depth, in_string = 0, False
    while index < len(text):
        character = text[index]
        if in_string:
            if character == "\\":
                index += 1
            elif character == '"':
                in_string = False
        elif character == '"':
            in_string = True
        elif character in "([{":
            depth += 1
        elif character in ")]}":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    return None


def expression_end(text, start):
    """🔚️ Index just past the call expression starting at `start` and every `.method(…)` chained onto it."""
    end = match_close(text, text.index("(", start))
    while end is not None:
        chained = re.match(r"\s*\.\w+\(", text[end:])
        if not chained:
            return end
        end = match_close(text, end + chained.end() - 1)
    return end


def rustfmt(expression, prefix, suffix, repeat, label):
    """🎨️ The expression formatted by rustfmt itself inside `prefix`/`suffix` — the element's own context; a list element
    is repeated so the list stays vertical — answering the first expression's text."""
    source = prefix + expression + ((",\n" + expression) if repeat else "") + suffix
    run = subprocess.run(["rustfmt", "--edition", "2021", "--config-path", RUSTFMT_CONFIG, "--emit", "stdout"], input=source, capture_output=True, text=True)
    if run.returncode != 0 or not run.stdout.startswith(prefix):
        problems.append(f"{label}: rustfmt did not keep the element's context: {run.stderr[:300]} | {run.stdout[:300]!r}")
        return expression
    body = run.stdout[len(prefix) :]
    return body[: expression_end(body, 0)]


def element(text, marker, chain, prefix, suffix, label, repeat=True):
    starts = [match.start() for match in re.finditer(re.escape(marker), text)]
    if len(starts) != 1:
        problems.append(f"{label}: {len(starts)} elements start with {marker!r}")
        return text
    start = starts[0]
    end = expression_end(text, start)
    if end is None:
        problems.append(f"{label}: unclosed element")
        return text
    flat = re.sub(r"\s*\n\s*", " ", text[start:end])
    for old, new in (("( ", "("), (" )", ")"), ("[ ", "["), (" ]", "]"), (",)", ")"), (",]", "]"), (") .", ").")):
        flat = flat.replace(old, new)
    return text[:start] + rustfmt(flat + chain, prefix, suffix, repeat, label) + text[end:]


#region Edits
def editing(text):
    for constant, (en, de, destructive) in SNAPSHOT.items():
        marker = f"mutation({constant},"
        if marker not in text:
            marker = f"mutation(\n            {constant},"
        chain = describe(en, de) + (".destructive()" if destructive else "")
        text = element(text, marker, chain, "fn f() {\n    vec![\n        ", ",\n    ]\n}\n", f"{EDITING}: {constant}")
    return text


def contract(text):
    for constant, (en, de, destructive) in TABLE.items():
        chain = describe(en, de) + (".destructive()" if destructive else "")
        text = element(text, f"ActionDefinition::bounded_catalog({constant},", chain, "fn f() {\n    let structural = [\n        ", ",\n    ];\n}\n", f"{CONTRACT}: {constant}")
    return text


def wav(text):
    for constant, (en, de, _) in WAV_VERBS.items():
        text = element(text, f"ActionDefinition::bounded_catalog({constant},", describe(en, de), "fn f() {\n    [\n        ", ",\n    ]\n}\n", f"{WAV}: {constant}")
    return text


def vertex(path):
    def edit(text):
        en, de = VERTEX[path]
        return element(text, 'ActionDefinition::bounded_catalog("set-vertex",', describe(en, de), "fn f() {\n    let mut action = ", ";\n}\n", f"{path}: set-vertex", repeat=False)
    return edit


def pdf(text):
    body = text[text.index("pub fn window_definition() -> WindowKindDefinition {") :]
    body = body[: body.index("\n}\n")]
    declared = re.findall(r'^\s*action\(\s*"([\w-]+)"', body, re.M)
    undescribed = [verb for verb in declared if not verb.startswith("canvas") and verb != "set-page"]
    if sorted(undescribed) != sorted(PDF_VERBS):
        problems.append(f"{PDF}: page verbs {sorted(set(undescribed) ^ set(PDF_VERBS))} differ from the described set")
    for verb, (en, de) in PDF_VERBS.items():
        chain = describe(en, de) + (".destructive()" if verb in PDF_DESTRUCTIVE else "")
        text = element(text, f'action("{verb}",', chain, "fn f() -> X {\n    X {\n        actions: vec![\n            ", ",\n        ],\n    }\n}\n", f"{PDF}: {verb}")
    return text
#endregion Edits


def plan():
    return {EDITING: editing, CONTRACT: contract, WAV: wav, MESH: vertex(MESH), BREP: vertex(BREP), PDF: pdf}


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    if mode is None:
        print(__doc__)
        sys.exit(2)
    if mode == "--revert":
        for root, _, files in os.walk(BACKUP):
            for file in files:
                source = os.path.join(root, file)
                path = os.path.relpath(source, BACKUP)
                shutil.copyfile(source, os.path.join(TREE, path))
                print("restored", path)
        return
    staged = {}
    english = {}
    for table in (SNAPSHOT, TABLE, WAV_VERBS):
        for constant, (en, _, _) in table.items():
            english.setdefault(en, []).append(constant)
    for verb, (en, _) in PDF_VERBS.items():
        english.setdefault(en, []).append(verb)
    for en, owners in english.items():
        if len(owners) > 1:
            problems.append(f"one English description shared by {owners}")
    for path, edit in plan().items():
        before = open(os.path.join(TREE, path), encoding="utf-8").read()
        after = edit(before)
        if after == before:
            problems.append(f"{path}: unchanged")
        staged[path] = (before, after)
    for problem in problems:
        print("PROBLEM", problem)
    print(f"{len(staged)} files, {len(problems)} problems")
    if mode == "--write" and not problems:
        for path, (before, after) in staged.items():
            backup = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                open(backup, "w", encoding="utf-8").write(before)
            open(os.path.join(TREE, path), "w", encoding="utf-8").write(after)
        print("written; backups under", BACKUP)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
