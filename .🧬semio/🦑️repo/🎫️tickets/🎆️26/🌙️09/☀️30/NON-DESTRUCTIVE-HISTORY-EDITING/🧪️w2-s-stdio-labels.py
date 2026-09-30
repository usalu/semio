#!/usr/bin/env python3
"""🏷️ W2-S stdio: `x-semio-ui` (design §6, manifest `$defs/InputUi`) for the snapshot members the re-pointed `set-snapshot`
leaves now reach — the `PdfSnapshot` catalog members the old `artifact.json` restatement never declared, and the ZIP member
header metadata and `commentUtf8` the stale inlined `ZipSnapshot` copy dropped. Span-surgical inserts via
`🧪️w2-s-stdio-snapshots.py`; idempotent.

    python3 🧪️w2-s-stdio-labels.py [--dry-run]
"""
import importlib.util
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("snapshots", os.path.join(HERE, "🧪️w2-s-stdio-snapshots.py"))
snapshots = importlib.util.module_from_spec(spec)
spec.loader.exec_module(snapshots)


def ui(en, de, description=None, **extra):
    value = ({"widget": extra.pop("widget")} if "widget" in extra else {}) | {"label": {"en": en, "de": de}}
    if description is not None:
        value["description"] = {"en": description[0], "de": description[1]}
    return value | extra


PAGE_LAYOUTS = {"singlePage": {"en": "Single page", "de": "Einzelne Seite"}, "oneColumn": {"en": "One column", "de": "Fortlaufend"}, "twoColumnLeft": {"en": "Two columns, odd pages left", "de": "Zwei Spalten, ungerade Seiten links"}, "twoColumnRight": {"en": "Two columns, odd pages right", "de": "Zwei Spalten, ungerade Seiten rechts"}, "twoPageLeft": {"en": "Two pages, odd pages left", "de": "Doppelseite, ungerade Seiten links"}, "twoPageRight": {"en": "Two pages, odd pages right", "de": "Doppelseite, ungerade Seiten rechts"}}
PAGE_MODES = {"useNone": {"en": "Page only", "de": "Nur Seite"}, "useOutlines": {"en": "Bookmarks panel", "de": "Lesezeichenfenster"}, "useThumbs": {"en": "Page thumbnails", "de": "Seitenminiaturen"}, "fullScreen": {"en": "Full screen", "de": "Vollbild"}, "useOc": {"en": "Layers panel", "de": "Ebenenfenster"}, "useAttachments": {"en": "Attachments panel", "de": "Anlagenfenster"}}
PDF = {
    "fonts": ui("Fonts", "Schriften", ("Font resources shared by the pages.", "Von den Seiten gemeinsam genutzte Schriftressourcen.")),
    "images": ui("Images", "Bilder", ("Image XObjects shared by the pages.", "Von den Seiten gemeinsam genutzte Bild-XObjekte.")),
    "forms": ui("Form XObjects", "Formular-XObjekte", ("Reusable content streams shared by the pages.", "Von den Seiten gemeinsam genutzte wiederverwendbare Inhaltsströme.")),
    "extGStates": ui("Graphics states", "Grafikzustände", ("Extended graphics state parameter dictionaries (ExtGState).", "Wörterbücher erweiterter Grafikzustandsparameter (ExtGState).")),
    "shadings": ui("Shadings", "Schattierungen"),
    "patterns": ui("Patterns", "Muster"),
    "colorSpaces": ui("Color spaces", "Farbräume"),
    "outlines": ui("Bookmarks", "Lesezeichen", ("Document outline in reading order.", "Dokumentgliederung in Lesereihenfolge.")),
    "namedDestinations": ui("Named destinations", "Benannte Ziele"),
    "pageLabels": ui("Page labels", "Seitenbeschriftungen", ("Page numbering ranges, e.g. roman front matter.", "Seitennummerierungsbereiche, z. B. römisch nummerierte Titelei.")),
    "embeddedFiles": ui("Embedded files", "Eingebettete Dateien"),
    "outputIntents": ui("Output intents", "Ausgabebedingungen", ("Intended output devices and their ICC profiles.", "Vorgesehene Ausgabegeräte und ihre ICC-Profile.")),
    "acroForm": ui("Interactive form", "Interaktives Formular", ("AcroForm fields and their defaults.", "AcroForm-Felder und ihre Vorgaben.")),
    "optionalContent": ui("Optional content", "Optionale Inhalte", ("Layers (optional content groups) and their default visibility.", "Ebenen (optionale Inhaltsgruppen) und ihre Standardsichtbarkeit.")),
    "pageLayout": ui("Page layout", "Seitenlayout", ("Page arrangement when the document opens.", "Seitenanordnung beim Öffnen des Dokuments."), widget="select", options=PAGE_LAYOUTS),
    "pageMode": ui("Page mode", "Seitenmodus", ("Panel shown when the document opens.", "Beim Öffnen des Dokuments angezeigte Leiste."), widget="select", options=PAGE_MODES),
    "viewerPreferences": ui("Viewer preferences", "Anzeigeeinstellungen"),
    "openAction": ui("Open action", "Aktion beim Öffnen", ("Destination or action performed when the document opens.", "Ziel oder Aktion beim Öffnen des Dokuments.")),
    "language": ui("Document language", "Dokumentsprache", ("BCP 47 tag of the natural language, e.g. de-DE.", "BCP-47-Kennung der natürlichen Sprache, z. B. de-DE."), widget="text"),
    "markInfo": ui("Tagged PDF marks", "Tagged-PDF-Kennzeichnung", ("Whether the document is a tagged PDF and how it is marked.", "Ob das Dokument ein Tagged PDF ist und wie es gekennzeichnet ist.")),
    "metadata": ui("XMP metadata", "XMP-Metadaten", ("Document-level XMP packet.", "XMP-Paket auf Dokumentebene."), widget="multiline"),
    "documentId": ui("Document ID", "Dokument-ID", ("Permanent and changing identifier byte strings of the trailer ID.", "Beständige und veränderliche Kennungsbytefolgen der Trailer-ID.")),
    "encryption": ui("Encryption", "Verschlüsselung"),
    "catalogExtra": ui("Other catalog entries", "Weitere Katalogeinträge", ("Document catalog entries kept verbatim.", "Unverändert erhaltene Einträge des Dokumentkatalogs.")),
}
HEADER = {
    "versionNeeded": ui("Version needed to extract", "Zum Entpacken benötigte Version", ("ZIP specification version times ten, e.g. 20 for 2.0.", "ZIP-Spezifikationsversion mal zehn, z. B. 20 für 2.0."), widget="stepper"),
    "flags": ui("General-purpose flags", "Allzweck-Bitflags", ("General-purpose bit flag field of the header.", "Allzweck-Bitfeld des Headers."), widget="stepper"),
    "modifiedTime": ui("Modification time", "Änderungszeit", ("MS-DOS time field.", "MS-DOS-Zeitfeld."), widget="stepper"),
    "modifiedDate": ui("Modification date", "Änderungsdatum", ("MS-DOS date field.", "MS-DOS-Datumsfeld."), widget="stepper"),
    "extraFields": ui("Extra fields", "Zusatzfelder"),
    "unicodePathLegacyName": ui("Legacy name bytes", "Namensbytes (Altformat)", ("Name bytes stored beside a Unicode path extra field.", "Neben einem Unicode-Pfad-Zusatzfeld gespeicherte Namensbytes.")),
}
ZIP_DEFS = {
    "ZipExtraField": {"id": ui("Header ID", "Header-ID", ("Extra-field header identifier, e.g. 0x5455.", "Kennung des Zusatzfeld-Headers, z. B. 0x5455."), widget="stepper"), "data": ui("Field data", "Felddaten")},
    "ZipLocalHeaderMetadata": HEADER,
    "ZipCentralHeaderMetadata": HEADER | {
        "versionMadeBy": ui("Version made by", "Erstellt mit Version", ("Host system and ZIP specification version of the archiver.", "Hostsystem und ZIP-Spezifikationsversion des Archivierers."), widget="stepper"),
        "comment": ui("File comment", "Dateikommentar", widget="text"),
        "unicodeCommentLegacy": ui("Legacy comment bytes", "Kommentarbytes (Altformat)", ("Comment bytes stored beside a Unicode comment extra field.", "Neben einem Unicode-Kommentar-Zusatzfeld gespeicherte Kommentarbytes.")),
        "internalAttributes": ui("Internal attributes", "Interne Attribute", widget="stepper"),
        "externalAttributes": ui("External attributes", "Externe Attribute", ("Host-dependent file attributes, e.g. Unix mode bits.", "Hostabhängige Dateiattribute, z. B. Unix-Modusbits."), widget="stepper"),
    },
    "ZipEntryMetadata": {
        "compressionMethod": ui("Compression method", "Kompressionsverfahren", ("0 stores the member, 8 deflates it.", "0 speichert den Eintrag, 8 komprimiert ihn mit Deflate.")),
        "local": ui("Local file header", "Lokaler Dateiheader"),
        "central": ui("Central directory header", "Zentralverzeichnis-Header"),
        "dataDescriptorSignature": ui("Data descriptor signature", "Datendeskriptor-Signatur", ("Whether the data descriptor carries its optional signature.", "Ob der Datendeskriptor seine optionale Signatur trägt."), widget="toggle"),
    },
    "ZipEntry": {"metadata": ui("Header metadata", "Header-Metadaten", ("Serialization state of the member headers.", "Serialisierungszustand der Header des Eintrags."))},
}
ZIP_ROOT = {"commentUtf8": ui("UTF-8 comment", "UTF-8-Kommentar", ("Archive comment bytes are UTF-8; off means CP437.", "Die Bytes des Archivkommentars sind UTF-8; aus bedeutet CP437."), widget="toggle")}


def main():
    pdf = snapshots.PDF + "/📸️snapshot/🔣️.json"
    snapshots.edit(pdf, lambda text: snapshots.annotate(text, PDF))
    zip_snapshot = snapshots.ZIP + "/🧱️base/🧬️schema/📸️snapshot/🔣️.json"

    def zip_labels(text):
        for name, members in ZIP_DEFS.items():
            for key, value in members.items():
                text = snapshots.add_member(text, ["$defs", name, "properties", key], "x-semio-ui", value)
        return snapshots.annotate(text, ZIP_ROOT)

    snapshots.edit(zip_snapshot, zip_labels)


if __name__ == "__main__":
    main()
