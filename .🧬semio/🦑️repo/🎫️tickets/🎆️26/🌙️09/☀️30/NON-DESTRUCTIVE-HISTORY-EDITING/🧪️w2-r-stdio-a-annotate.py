#!/usr/bin/env python3
"""🏷️ W2-R stdio-a: writes `x-semio-ui` (design §6, manifest `$defs/InputUi`) onto every mutation input of the stdio
artifacts avi, bcf, binary, bmp, csv, deflate, docx, dwg, dxf, epw, gif, gltf, html and ifc (commands, contract and graph
have no mutation leaves). Inputs are enumerated by `🧪️w2-r-stdio-a-walk.py` (a mirror of the manifest reader), labelled
from the domain tables below (German per AutoCAD/buildingSMART/Khronos/EnergyPlus usage), and written at the node that
carries them: the property node for labels and widgets, the enum node for option labels, the union branch for variant
labels. Top-level inputs (and the fields of a root union's `apply` payload) also get widget, role/ref, group and order.

    python3 🧪️w2-r-stdio-a-annotate.py [--dry-run] [--shared]

`--shared` also writes the xml artifact's snapshot schema (owned by the stdio-b group, referenced by docx)."""
import importlib.util
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("walk", os.path.join(HERE, "🧪️w2-r-stdio-a-walk.py"))
walk = importlib.util.module_from_spec(spec)
spec.loader.exec_module(walk)

REPO = walk.REPO
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "options", "group", "order")
SHARED = {"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"}


def D(en, de):
    return {"en": en, "de": de}


def L(en, de, desc=None, **extra):
    body = {"label": D(en, de)}
    if desc is not None:
        body["description"] = D(*desc)
    body.update(extra)
    return body


def REF(kind, domain, en, de, desc=None, many=False):
    return L(en, de, desc, widget="reference", role="target", ref={"kind": kind, "domain": domain})


def UNIT(unit):
    return ("Unit: %s" % unit, "Einheit: %s" % unit)


SNAPSHOT_SCHEMA = L("Schema", "Schema", ("Identifier of the snapshot schema this document follows.", "Kennung des Snapshot-Schemas, dem dieses Dokument folgt."))
RESTORE = L("Restore diff", "Wiederherstellungs-Diff", ("Recorded difference that restores the state before the change; written by the inverse, not edited by hand.", "Aufgezeichnete Differenz, die den Zustand vor der Änderung wiederherstellt; stammt aus der Inversen und wird nicht von Hand bearbeitet."), widget="hidden")
APPLY = L("Parameters", "Parameter", ("Parameters of the forward change.", "Parameter der Vorwärtsänderung."))
PHASE = {"role": "discriminator", "label": D("Mutation phase", "Mutationsphase")}
PHASES = {"apply": D("Apply", "Anwenden"), "restore": D("Restore", "Wiederherstellen")}

#region 🌐️Common
COMMON = {
    "schema": SNAPSHOT_SCHEMA,
    "added": L("Added", "Hinzugefügt"),
    "removed": L("Removed", "Entfernt"),
    "modified": L("Modified", "Geändert"),
    "diff": L("Changes", "Änderungen"),
}
#endregion 🌐️Common

#region 🌐️Html
HTML = {
    "path": L("Node path", "Knotenpfad", ("Child indices from the document root to the node.", "Kindindizes vom Dokumentstamm bis zum Knoten.")),
    "parent": L("Parent path", "Pfad des Elternknotens", ("Child indices from the document root to the parent node.", "Kindindizes vom Dokumentstamm bis zum Elternknoten.")),
    "index": L("Child position", "Kindposition", ("Position among the parent's children, counted from 0.", "Position unter den Kindknoten des Elternknotens, ab 0 gezählt.")),
    "insert-node:node": L("Node", "Knoten", ("The element, text or comment node to insert.", "Der einzufügende Element-, Text- oder Kommentarknoten."), group="value"),
    "set-element-name:name": L("Tag name", "Elementname"),
    "set-attribute:name": L("Attribute name", "Attributname"),
    "set-attribute:value": L("Attribute value", "Attributwert", ("Empty removes the attribute.", "Leer entfernt das Attribut.")),
    "set-text:text": L("Text", "Text", widget="multiline"),
    "set-raw-text:text": L("Raw text", "Rohtext", ("Script or style content written without escaping.", "Skript- oder Stilinhalt, der ohne Maskierung geschrieben wird."), widget="multiline"),
    "set-comment:text": L("Comment text", "Kommentartext", widget="multiline"),
    "doctype": L("Document type", "Dokumenttyp", ("DOCTYPE declaration; empty removes it.", "DOCTYPE-Deklaration; leer entfernt sie.")),
    "snapshot": L("HTML document", "HTML-Dokument"),
    "root": L("Root node", "Wurzelknoten"),
}
#endregion 🌐️Html

#region 🌦️Epw
EPW_RAW = ("Raw header line as written in the EPW file.", "Unveränderte Kopfzeile wie in der EPW-Datei.")
EPW = {
    "set-ground-temperatures:value": L("Ground temperatures", "Erdreichtemperaturen", EPW_RAW, widget="multiline"),
    "set-design-conditions:value": L("Design conditions", "Auslegungsbedingungen", EPW_RAW, widget="multiline"),
    "set-holidays-dst:value": L("Holidays and daylight saving", "Feiertage und Sommerzeit", EPW_RAW, widget="multiline"),
    "set-typical-extreme-periods:value": L("Typical and extreme periods", "Typische und extreme Zeiträume", EPW_RAW, widget="multiline"),
    "set-comments1:value": L("Comments 1", "Kommentare 1", widget="multiline"),
    "set-comments2:value": L("Comments 2", "Kommentare 2", widget="multiline"),
    "set-record-field:value": L("Field value", "Feldwert", ("Raw text of the field as written in the file.", "Rohtext des Feldes wie in der Datei.")),
    "recordIndex": L("Record", "Datensatz", ("Index of the hourly data record, counted from 0.", "Index des stündlichen Datensatzes, ab 0 gezählt.")),
    "fieldIndex": L("Field", "Feld", ("Index of the field within the record, counted from 0.", "Index des Feldes im Datensatz, ab 0 gezählt.")),
    "index": L("Record position", "Datensatzposition", ("Index of the hourly data record, counted from 0.", "Index des stündlichen Datensatzes, ab 0 gezählt.")),
    "record": L("Record", "Datensatz"),
    "records": L("Records", "Datensätze"),
    "dataPeriods": L("Data periods", "Datenzeiträume"),
    "recordsPerHour": L("Records per hour", "Datensätze pro Stunde"),
    "periods": L("Periods", "Zeiträume"),
    "EpwDataPeriod.name": L("Period name", "Name des Zeitraums"),
    "startDayOfWeek": L("Start weekday", "Wochentag des Beginns"),
    "startDate": L("Start date", "Anfangsdatum"),
    "endDate": L("End date", "Enddatum"),
    "location": L("Location", "Standort"),
    "city": L("City", "Ort"),
    "stateProvince": L("State or province", "Bundesland oder Provinz"),
    "country": L("Country", "Land"),
    "EpwLocation.source": L("Data source", "Datenquelle"),
    "wmo": L("WMO station number", "WMO-Stationsnummer"),
    "latitude": L("Latitude", "Geographische Breite", ("Degrees, north positive.", "Grad, nördlich positiv.")),
    "longitude": L("Longitude", "Geographische Länge", ("Degrees, east positive.", "Grad, östlich positiv.")),
    "timeZone": L("Time zone", "Zeitzone", ("Offset from UTC in hours.", "Abweichung von UTC in Stunden.")),
    "elevation": L("Elevation", "Höhe über Meeresspiegel", ("Metres above sea level.", "Meter über dem Meeresspiegel.")),
    "month": L("Month", "Monat"),
    "day": L("Day", "Tag"),
    "hour": L("Hour", "Stunde"),
    "minute": L("Minute", "Minute"),
    "dataSourceUncertainty": L("Data source and uncertainty flags", "Datenquellen- und Unsicherheitskennzeichen"),
    "dryBulbTemp": L("Dry-bulb temperature", "Trockenkugeltemperatur", UNIT("°C")),
    "dewPointTemp": L("Dew-point temperature", "Taupunkttemperatur", UNIT("°C")),
    "relativeHumidity": L("Relative humidity", "Relative Luftfeuchte", UNIT("%")),
    "atmosphericPressure": L("Atmospheric station pressure", "Luftdruck auf Stationshöhe", UNIT("Pa")),
    "extraterrestrialHorizontalRadiation": L("Extraterrestrial horizontal radiation", "Extraterrestrische Horizontalstrahlung", UNIT("Wh/m²")),
    "extraterrestrialDirectNormalRadiation": L("Extraterrestrial direct normal radiation", "Extraterrestrische Direktnormalstrahlung", UNIT("Wh/m²")),
    "horizontalInfraredRadiation": L("Horizontal infrared radiation intensity", "Atmosphärische Gegenstrahlung (horizontal)", UNIT("Wh/m²")),
    "globalHorizontalRadiation": L("Global horizontal radiation", "Globalstrahlung (horizontal)", UNIT("Wh/m²")),
    "directNormalRadiation": L("Direct normal radiation", "Direktnormalstrahlung", UNIT("Wh/m²")),
    "diffuseHorizontalRadiation": L("Diffuse horizontal radiation", "Diffusstrahlung (horizontal)", UNIT("Wh/m²")),
    "globalHorizontalIlluminance": L("Global horizontal illuminance", "Globale Beleuchtungsstärke (horizontal)", UNIT("lx")),
    "directNormalIlluminance": L("Direct normal illuminance", "Direkte Beleuchtungsstärke (normal)", UNIT("lx")),
    "diffuseHorizontalIlluminance": L("Diffuse horizontal illuminance", "Diffuse Beleuchtungsstärke (horizontal)", UNIT("lx")),
    "zenithLuminance": L("Zenith luminance", "Zenitleuchtdichte", UNIT("cd/m²")),
    "windDirection": L("Wind direction", "Windrichtung", ("Degrees clockwise from north.", "Grad im Uhrzeigersinn ab Nord.")),
    "windSpeed": L("Wind speed", "Windgeschwindigkeit", UNIT("m/s")),
    "totalSkyCover": L("Total sky cover", "Gesamtbedeckungsgrad", ("Tenths of the sky.", "Zehntel des Himmels.")),
    "opaqueSkyCover": L("Opaque sky cover", "Opaker Bedeckungsgrad", ("Tenths of the sky.", "Zehntel des Himmels.")),
    "visibility": L("Visibility", "Sichtweite", UNIT("km")),
    "ceilingHeight": L("Ceiling height", "Wolkenuntergrenze", UNIT("m")),
    "presentWeatherObservation": L("Present weather observation", "Wetterbeobachtung vorhanden"),
    "presentWeatherCodes": L("Present weather codes", "Wettercodes"),
    "precipitableWater": L("Precipitable water", "Niederschlagbares Wasser", UNIT("mm")),
    "aerosolOpticalDepth": L("Aerosol optical depth", "Aerosol-optische Dicke"),
    "snowDepth": L("Snow depth", "Schneehöhe", UNIT("cm")),
    "daysSinceLastSnowfall": L("Days since last snowfall", "Tage seit dem letzten Schneefall"),
    "albedo": L("Albedo", "Albedo"),
    "liquidPrecipDepth": L("Liquid precipitation depth", "Niederschlagshöhe", UNIT("mm")),
    "liquidPrecipQuantity": L("Liquid precipitation period", "Niederschlagsmessdauer", ("Hours over which the precipitation depth was measured.", "Stunden, über die die Niederschlagshöhe gemessen wurde.")),
    "snapshot": L("EPW weather file", "EPW-Wetterdatei"),
    "designConditions": L("Design conditions", "Auslegungsbedingungen"),
    "typicalExtremePeriods": L("Typical and extreme periods", "Typische und extreme Zeiträume"),
    "groundTemperatures": L("Ground temperatures", "Erdreichtemperaturen"),
    "holidaysDst": L("Holidays and daylight saving", "Feiertage und Sommerzeit"),
    "comments1": L("Comments 1", "Kommentare 1"),
    "comments2": L("Comments 2", "Kommentare 2"),
}
#endregion 🌦️Epw

#region 🎞️Gif
GIF_INDEX = ("Index of the palette color, 0 to 255.", "Index der Palettenfarbe, 0 bis 255.")
GIF = {
    "gct": L("Global color table", "Globale Farbtabelle", ("Empty removes the global color table.", "Leer entfernt die globale Farbtabelle.")),
    "lct": L("Local color table", "Lokale Farbtabelle"),
    "sorted": L("Sorted by importance", "Nach Wichtigkeit sortiert"),
    "colors": L("Colors", "Farben"),
    "r": L("Red", "Rot"),
    "g": L("Green", "Grün"),
    "b": L("Blue", "Blau"),
    "left": L("Left offset", "Abstand links", unit="px"),
    "top": L("Top offset", "Abstand oben", unit="px"),
    "width": L("Width", "Breite", unit="px"),
    "height": L("Height", "Höhe", unit="px"),
    "ratio": L("Pixel aspect ratio", "Pixel-Seitenverhältnis", ("Stored byte: aspect = (value + 15) / 64; 0 means no information.", "Gespeichertes Byte: Seitenverhältnis = (Wert + 15) / 64; 0 bedeutet keine Angabe.")),
    "pixelAspectRatio": L("Pixel aspect ratio", "Pixel-Seitenverhältnis", ("Stored byte: aspect = (value + 15) / 64; 0 means no information.", "Gespeichertes Byte: Seitenverhältnis = (Wert + 15) / 64; 0 bedeutet keine Angabe.")),
    "backgroundColorIndex": L("Background color index", "Hintergrundfarbindex", GIF_INDEX),
    "set-background-color-index:index": L("Background color index", "Hintergrundfarbindex", GIF_INDEX, group="value"),
    "images": L("Images", "Bilder"),
    "frames": L("Frames", "Einzelbilder"),
    "loopCount": L("Loop count", "Anzahl Wiederholungen", ("NETSCAPE2.0 loop count; 0 loops forever.", "NETSCAPE2.0-Wiederholungen; 0 wiederholt endlos.")),
    "loop_count": L("Loop count", "Anzahl Wiederholungen", ("NETSCAPE2.0 loop count; 0 loops forever, empty removes the loop extension.", "NETSCAPE2.0-Wiederholungen; 0 wiederholt endlos, leer entfernt die Schleifenerweiterung.")),
    "comments": L("Comments", "Kommentare"),
    "appExtensions": L("Application extensions", "Anwendungserweiterungen"),
    "indices": L("Pixel color indices", "Pixel-Farbindizes", ("One palette index per pixel, row by row.", "Ein Palettenindex je Pixel, zeilenweise.")),
    "delay_cs": L("Frame delay", "Bildverzögerung", unit="cs", displayUnit="s", displayFactor=0.01, precision=2),
    "delayCs": L("Frame delay", "Bildverzögerung", unit="cs", displayUnit="s", displayFactor=0.01, precision=2),
    "disposal": L("Disposal method", "Entfernungsmethode", ("What happens to the frame before the next one is drawn.", "Was mit dem Einzelbild geschieht, bevor das nächste gezeichnet wird.")),
    "transparent_index": L("Transparent color index", "Transparenzfarbindex", ("Palette index drawn transparent; empty disables transparency.", "Palettenindex, der transparent gezeichnet wird; leer schaltet Transparenz ab.")),
    "transparentIndex": L("Transparent color index", "Transparenzfarbindex"),
    "user_input": L("Wait for user input", "Auf Benutzereingabe warten"),
    "userInput": L("Wait for user input", "Auf Benutzereingabe warten"),
    "plainText": L("Plain text extension", "Klartexterweiterung"),
    "cellWidth": L("Character cell width", "Zeichenzellenbreite", unit="px"),
    "cellHeight": L("Character cell height", "Zeichenzellenhöhe", unit="px"),
    "fgColorIndex": L("Text color index", "Textfarbindex"),
    "bgColorIndex": L("Background color index", "Hintergrundfarbindex"),
    "identifier": L("Application identifier", "Anwendungskennung", ("Eight ASCII bytes, e.g. NETSCAPE.", "Acht ASCII-Bytes, z. B. NETSCAPE.")),
    "authCode": L("Authentication code", "Authentifizierungscode", ("Three bytes, e.g. 2.0.", "Drei Bytes, z. B. 2.0.")),
    "GifAppExtension.data": L("Sub-block data", "Unterblockdaten"),
    "from": L("From position", "Von Position"),
    "to": L("To position", "Nach Position"),
    "set-image-pixels:index": L("Image", "Bild", ("Index of the image, counted from 0.", "Index des Bildes, ab 0 gezählt.")),
    "set-image-geometry:index": L("Image", "Bild", ("Index of the image, counted from 0.", "Index des Bildes, ab 0 gezählt.")),
    "set-image-interlace:index": L("Image", "Bild", ("Index of the image, counted from 0.", "Index des Bildes, ab 0 gezählt.")),
    "insert-image:index": L("Insert position", "Einfügeposition"),
    "remove-image:index": L("Image", "Bild", ("Index of the image, counted from 0.", "Index des Bildes, ab 0 gezählt.")),
    "set-frame-delay:index": L("Frame", "Einzelbild", ("Index of the frame, counted from 0.", "Index des Einzelbildes, ab 0 gezählt.")),
    "set-frame-disposal:index": L("Frame", "Einzelbild", ("Index of the frame, counted from 0.", "Index des Einzelbildes, ab 0 gezählt.")),
    "set-frame-pixels:index": L("Frame", "Einzelbild", ("Index of the frame, counted from 0.", "Index des Einzelbildes, ab 0 gezählt.")),
    "set-frame-transparency:index": L("Frame", "Einzelbild", ("Index of the frame, counted from 0.", "Index des Einzelbildes, ab 0 gezählt.")),
    "set-frame-geometry:index": L("Frame", "Einzelbild", ("Index of the frame, counted from 0.", "Index des Einzelbildes, ab 0 gezählt.")),
    "set-frame-user-input:index": L("Frame", "Einzelbild", ("Index of the frame, counted from 0.", "Index des Einzelbildes, ab 0 gezählt.")),
    "set-frame-interlace:index": L("Frame", "Einzelbild", ("Index of the frame, counted from 0.", "Index des Einzelbildes, ab 0 gezählt.")),
    "insert-frame:index": L("Insert position", "Einfügeposition"),
    "remove-frame:index": L("Frame", "Einzelbild", ("Index of the frame, counted from 0.", "Index des Einzelbildes, ab 0 gezählt.")),
    "insert-comment:index": L("Insert position", "Einfügeposition"),
    "remove-comment:index": L("Comment", "Kommentar", ("Index of the comment, counted from 0.", "Index des Kommentars, ab 0 gezählt.")),
    "add-app-extension:index": L("Insert position", "Einfügeposition"),
    "remove-app-extension:index": L("Application extension", "Anwendungserweiterung", ("Index of the extension, counted from 0.", "Index der Erweiterung, ab 0 gezählt.")),
    "insert-comment:text": L("Comment text", "Kommentartext", widget="multiline"),
    "frame": L("Frame", "Einzelbild"),
    "add-app-extension:extension": L("Application extension", "Anwendungserweiterung"),
    "snapshot": L("GIF image", "GIF-Bild"),
    "GifSnapshot.width": L("Logical screen width", "Breite des logischen Bildschirms", unit="px"),
    "GifSnapshot.height": L("Logical screen height", "Höhe des logischen Bildschirms", unit="px"),
    "set-screen-size:width": L("Logical screen width", "Breite des logischen Bildschirms", unit="px"),
    "set-screen-size:height": L("Logical screen height", "Höhe des logischen Bildschirms", unit="px"),
}
GIF_OPTIONS = {
    "disposal": {"unspecified": D("Not specified", "Nicht festgelegt"), "doNotDispose": D("Do not dispose", "Nicht entfernen"), "restoreToBackground": D("Restore to background", "Auf Hintergrund zurücksetzen"), "restoreToPrevious": D("Restore to previous", "Auf vorheriges Bild zurücksetzen")},
}
#endregion 🎞️Gif

#region 🏗️Ifc
IFC_ID = ("STEP instance id (#n) of the entity.", "STEP-Instanz-ID (#n) der Entität.")
IFC_GLOBAL = L("GlobalId", "GlobalId", ("22-character IFC GUID of the object.", "22-stellige IFC-GUID des Objekts."), widget="text")
IFC_OWNER = L("Owner history", "Eigentümerhistorie", ("STEP instance id of the IfcOwnerHistory.", "STEP-Instanz-ID der IfcOwnerHistory."))
IFC = {
    "insert-entity:index": L("Insert position", "Einfügeposition", ("Position in the DATA section, counted from 0.", "Position im DATA-Abschnitt, ab 0 gezählt.")),
    "insert-entity:entity": L("Entity", "Entität"),
    "IfcEntity.id": L("Instance id", "Instanz-ID", IFC_ID),
    "IfcEntity.name": L("Entity type", "Entitätstyp", ("Upper-case IFC entity name, e.g. IFCWALL.", "IFC-Entitätsname in Großbuchstaben, z. B. IFCWALL.")),
    "IfcEntity.args": L("Attributes", "Attribute", ("Attribute values in schema order.", "Attributwerte in Schemareihenfolge.")),
    "IfcEntity.complex": L("Complex entity parts", "Teile der komplexen Entität"),
    "IfcComplexType.name": L("Entity type", "Entitätstyp"),
    "IfcComplexType.args": L("Attributes", "Attribute"),
    "remove-entity:id": L("Entity", "Entität", IFC_ID),
    "set-entity-arg:id": L("Entity", "Entität", IFC_ID),
    "set-entity-arg:index": L("Attribute position", "Attributposition", ("Position of the attribute in schema order, counted from 0.", "Position des Attributs in Schemareihenfolge, ab 0 gezählt.")),
    "set-entity-arg:value": L("Attribute value", "Attributwert"),
    "insert-entity-arg:id": L("Entity", "Entität", IFC_ID),
    "insert-entity-arg:index": L("Attribute position", "Attributposition", ("Position of the attribute in schema order, counted from 0.", "Position des Attributs in Schemareihenfolge, ab 0 gezählt.")),
    "insert-entity-arg:value": L("Attribute value", "Attributwert"),
    "remove-entity-arg:id": L("Entity", "Entität", IFC_ID),
    "remove-entity-arg:index": L("Attribute position", "Attributposition", ("Position of the attribute in schema order, counted from 0.", "Position des Attributs in Schemareihenfolge, ab 0 gezählt.")),
    "set-entity-name:id": L("Entity", "Entität", IFC_ID),
    "set-entity-name:name": L("Entity type", "Entitätstyp", ("Upper-case IFC entity name, e.g. IFCSLAB.", "IFC-Entitätsname in Großbuchstaben, z. B. IFCSLAB.")),
    "set-file-name:values": L("FILE_NAME values", "FILE_NAME-Werte", ("Parameters of the STEP header entity FILE_NAME.", "Parameter der STEP-Kopfentität FILE_NAME.")),
    "set-file-description:values": L("FILE_DESCRIPTION values", "FILE_DESCRIPTION-Werte", ("Parameters of the STEP header entity FILE_DESCRIPTION.", "Parameter der STEP-Kopfentität FILE_DESCRIPTION.")),
    "set-file-schema:values": L("FILE_SCHEMA values", "FILE_SCHEMA-Werte", ("Parameters of the STEP header entity FILE_SCHEMA.", "Parameter der STEP-Kopfentität FILE_SCHEMA.")),
    "snapshot": L("IFC model", "IFC-Modell"),
    "header": L("STEP header", "STEP-Kopfteil"),
    "fileDescription": L("File description", "Dateibeschreibung"),
    "fileName": L("File name", "Dateiname"),
    "fileSchema": L("File schema", "Dateischema"),
    "entities": L("Entities", "Entitäten"),
    "set-facility-name:building": L("Building", "Gebäude", ("STEP instance id of the IfcBuilding.", "STEP-Instanz-ID des IfcBuilding.")),
    "set-facility-name:name": L("Facility name", "Name der Liegenschaft", ("COBie Facility name; empty clears it.", "COBie-Name der Liegenschaft; leer löscht ihn.")),
    "set-view-definition:view": L("Model view definition", "Modellansichtsdefinition (MVD)", ("ViewDefinition named in the FILE_DESCRIPTION header.", "Im FILE_DESCRIPTION-Kopf genannte ViewDefinition.")),
    "set-floor-elevation:storey": L("Building storey", "Geschoss", ("STEP instance id of the IfcBuildingStorey.", "STEP-Instanz-ID des IfcBuildingStorey.")),
    "set-floor-elevation:elevation": L("Elevation", "Höhenkote", ("Storey elevation in project length units; empty clears it.", "Geschosshöhenkote in Längeneinheiten des Projekts; leer löscht sie."), precision=3),
    "set-space:id": L("Space", "Raum", ("STEP instance id of the IfcSpace.", "STEP-Instanz-ID des IfcSpace.")),
    "set-space:space": L("Space row", "Raumzeile", ("COBie Space row; empty removes the space.", "COBie-Raumzeile; leer entfernt den Raum.")),
    "globalId": IFC_GLOBAL,
    "CobieSpaceRow.placement": L("Placement", "Platzierung", ("STEP instance id of the IfcLocalPlacement.", "STEP-Instanz-ID der IfcLocalPlacement.")),
    "set-type-assignment:id": L("Type relationship", "Typbeziehung", ("STEP instance id of the IfcRelDefinesByType.", "STEP-Instanz-ID der IfcRelDefinesByType.")),
    "set-type-assignment:assignment": L("Type assignment", "Typzuweisung", ("Empty removes the relationship.", "Leer entfernt die Beziehung.")),
    "ownerHistory": IFC_OWNER,
    "relatedObjects": L("Related objects", "Zugeordnete Objekte", ("STEP instance ids of the assigned objects.", "STEP-Instanz-IDs der zugeordneten Objekte.")),
    "relatingType": L("Type object", "Typobjekt", ("STEP instance id of the relating IfcTypeObject.", "STEP-Instanz-ID des zuweisenden IfcTypeObject.")),
    "set-structural-entity:id": L("Entity", "Entität", IFC_ID),
    "set-structural-entity:entity": L("Building element", "Bauteil", ("Coordination View 2.0 element row; empty removes it.", "Bauteilzeile der Coordination View 2.0; leer entfernt sie.")),
    "typeName": L("Entity type", "Entitätstyp"),
    "set-product-placement:product": L("Product", "Produkt", ("STEP instance id of the IfcProduct.", "STEP-Instanz-ID des IfcProduct.")),
    "set-product-placement:placement": L("Placement", "Platzierung", ("STEP instance id of the IfcObjectPlacement; empty clears it.", "STEP-Instanz-ID der IfcObjectPlacement; leer löscht sie.")),
    "set-project-units:project": L("Project", "Projekt", ("STEP instance id of the IfcProject.", "STEP-Instanz-ID des IfcProject.")),
    "set-project-units:units": L("Unit assignment", "Einheitenzuweisung", ("STEP instance id of the IfcUnitAssignment; empty clears it.", "STEP-Instanz-ID der IfcUnitAssignment; leer löscht sie.")),
    "set-load-group:id": L("Load group", "Lastgruppe", ("STEP instance id of the IfcStructuralLoadGroup.", "STEP-Instanz-ID der IfcStructuralLoadGroup.")),
    "set-load-group:group": L("Load group", "Lastgruppe", ("Empty removes the load group.", "Leer entfernt die Lastgruppe.")),
    "predefinedType": L("Predefined type", "Vordefinierter Typ"),
    "actionType": L("Action type", "Einwirkungsart", ("Permanent, variable or accidental action (EN 1990).", "Ständige, veränderliche oder außergewöhnliche Einwirkung (EN 1990).")),
    "actionSource": L("Action source", "Einwirkungsursache", ("e.g. dead load, wind, snow.", "z. B. Eigengewicht, Wind, Schnee.")),
    "set-group-assignment:id": L("Group relationship", "Gruppenbeziehung", ("STEP instance id of the IfcRelAssignsToGroup.", "STEP-Instanz-ID der IfcRelAssignsToGroup.")),
    "set-group-assignment:assignment": L("Group assignment", "Gruppenzuweisung", ("Empty removes the relationship.", "Leer entfernt die Beziehung.")),
    "relatingGroup": L("Group", "Gruppe", ("STEP instance id of the relating IfcGroup.", "STEP-Instanz-ID der zuweisenden IfcGroup.")),
    "set-analysis-model:id": L("Analysis model", "Berechnungsmodell", ("STEP instance id of the IfcStructuralAnalysisModel.", "STEP-Instanz-ID des IfcStructuralAnalysisModel.")),
    "set-analysis-model:model": L("Analysis model", "Berechnungsmodell", ("Empty removes the model.", "Leer entfernt das Modell.")),
    "set-header:header": L("STEP header", "STEP-Kopfteil"),
    "remove-instance:id": L("Instance", "Instanz", ("STEP instance id (#n).", "STEP-Instanz-ID (#n).")),
    "upsert-instance:instance": L("Instance", "Instanz", ("Instance written under its id; an existing one is replaced.", "Instanz, die unter ihrer ID geschrieben wird; eine vorhandene wird ersetzt.")),
    "Part21Instance.id": L("Instance id", "Instanz-ID", ("STEP instance id (#n).", "STEP-Instanz-ID (#n).")),
    "Part21Instance.entities": L("Entities", "Entitäten", ("One entity, or the parts of a complex entity.", "Eine Entität oder die Teile einer komplexen Entität.")),
    "Part21Entity.arguments": L("Attribute values", "Attributwerte"),
    "Part21Value.kind": L("Value kind", "Wertart"),
    "Part21Value.value": L("Value", "Wert"),
    "Part21Value.values": L("List items", "Listenelemente"),
    "Part21Value.typeName": L("Type name", "Typname", ("Defined type of a typed value, e.g. IFCLABEL.", "Definierter Typ eines typisierten Werts, z. B. IFCLABEL.")),
    "Part21Document.header": L("STEP header", "STEP-Kopfteil"),
    "instances": L("Instances", "Instanzen"),
    "document": L("STEP document", "STEP-Dokument"),
    "edmPreamble": L("EDM preamble", "EDM-Präambel", ("Header comment block written by EDM-based exporters.", "Von EDM-basierten Exportern geschriebener Kommentarblock.")),
    "producer": L("Producer", "Hersteller"),
    "module": L("Module", "Modul"),
    "creationDate": L("Creation date", "Erstellungsdatum"),
    "host": L("Host", "Host"),
    "database": L("Database", "Datenbank"),
    "databaseVersion": L("Database version", "Datenbankversion"),
    "databaseCreationDate": L("Database creation date", "Erstellungsdatum der Datenbank"),
    "Ifc2x3EdmPreamble.schema": L("EXPRESS schema", "EXPRESS-Schema"),
    "model": L("Model", "Modell"),
    "modelCreationDate": L("Model creation date", "Erstellungsdatum des Modells"),
    "headerModel": L("Header model", "Kopfteilmodell"),
    "headerModelCreationDate": L("Header model creation date", "Erstellungsdatum des Kopfteilmodells"),
    "user": L("User", "Benutzer"),
    "Ifc2x3EdmPreamble.group": L("Group", "Gruppe"),
    "license": L("License", "Lizenz"),
    "Ifc2x3EdmPreamble.options": L("Options", "Optionen"),
}
IFC_OPTIONS = {
    "Part21Value.kind": {"ref": D("Instance reference", "Instanzverweis"), "str": D("String", "Zeichenkette"), "enum": D("Enumeration", "Aufzählung"), "int": D("Integer", "Ganzzahl"), "real": D("Real", "Gleitkommazahl"), "list": D("List", "Liste"), "typed": D("Typed value", "Typisierter Wert"), "unset": D("Unset ($)", "Nicht gesetzt ($)"), "derived": D("Derived (*)", "Abgeleitet (*)")},
}
#endregion 🏗️Ifc

#region 💬️Bcf
BCF_TOPIC = REF("topic", "bcf", "Topic", "Thema", ("GUID of the topic.", "GUID des Themas."))
BCF_COMMENT = REF("comment", "bcf", "Comment", "Kommentar", ("GUID of the comment.", "GUID des Kommentars."))
BCF_VIEWPOINT = REF("viewpoint", "bcf", "Viewpoint", "Ansichtspunkt", ("GUID of the viewpoint.", "GUID des Ansichtspunkts."))
BCF = {
    "topic_guid": BCF_TOPIC,
    "set-comment:guid": BCF_COMMENT,
    "remove-comment:guid": BCF_COMMENT,
    "set-viewpoint-camera:guid": BCF_VIEWPOINT,
    "set-viewpoint-snapshot:guid": BCF_VIEWPOINT,
    "set-viewpoint-components:guid": BCF_VIEWPOINT,
    "remove-viewpoint:guid": BCF_VIEWPOINT,
    "set-topic-markup:guid": BCF_TOPIC,
    "remove-topic:guid": BCF_TOPIC,
    "date": L("Date", "Datum", ("ISO 8601 date and time.", "Datum und Uhrzeit nach ISO 8601.")),
    "set-comment:text": L("Comment text", "Kommentartext", widget="multiline"),
    "viewpoint_ref": REF("viewpoint", "bcf", "Viewpoint", "Ansichtspunkt", ("Viewpoint the comment refers to; empty clears it.", "Ansichtspunkt, auf den sich der Kommentar bezieht; leer löscht ihn.")),
    "viewpointRef": L("Viewpoint", "Ansichtspunkt", ("GUID of the viewpoint the comment refers to.", "GUID des Ansichtspunkts, auf den sich der Kommentar bezieht.")),
    "viewpoint": L("Viewpoint", "Ansichtspunkt"),
    "viewpoints": L("Viewpoints", "Ansichtspunkte"),
    "components": L("Components", "Bauteile"),
    "BcfComponents.selection": L("Selected components", "Ausgewählte Bauteile", ("IFC GUIDs of the selected components.", "IFC-GUIDs der ausgewählten Bauteile.")),
    "BcfComponents.visibility": L("Visibility", "Sichtbarkeit"),
    "BcfComponents.coloring": L("Coloring", "Einfärbung"),
    "defaultVisibility": L("Visible by default", "Standardmäßig sichtbar"),
    "exceptions": L("Exceptions", "Ausnahmen", ("IFC GUIDs of components that deviate from the default visibility.", "IFC-GUIDs der Bauteile, die von der Standardsichtbarkeit abweichen.")),
    "BcfColoring.color": L("Color", "Farbe", ("ARGB or RGB hex color.", "ARGB- oder RGB-Hexfarbe.")),
    "BcfColoring.components": L("Components", "Bauteile", ("IFC GUIDs of the colored components.", "IFC-GUIDs der eingefärbten Bauteile.")),
    "snapshot": L("Snapshot image", "Snapshot-Bild", ("PNG bytes of the viewpoint snapshot; empty removes it.", "PNG-Bytes des Ansichtspunkt-Snapshots; leer entfernt ihn.")),
    "topic": L("Topic", "Thema"),
    "topics": L("Topics", "Themen"),
    "set-topic-markup:title": L("Title", "Titel"),
    "set-topic-markup:description": L("Description", "Beschreibung", widget="multiline"),
    "status": L("Status", "Status", ("Topic status, e.g. Open or Closed.", "Themenstatus, z. B. Open oder Closed.")),
    "priority": L("Priority", "Priorität"),
    "labels": L("Labels", "Schlagwörter"),
    "creationDate": L("Creation date", "Erstellungsdatum"),
    "creation_date": L("Creation date", "Erstellungsdatum", ("ISO 8601 date and time.", "Datum und Uhrzeit nach ISO 8601.")),
    "creationAuthor": L("Created by", "Erstellt von"),
    "creation_author": L("Created by", "Erstellt von"),
    "comments": L("Comments", "Kommentare"),
    "set-version:version": L("BCF version", "BCF-Version", ("VersionId written to bcf.version, e.g. 2.1.", "In bcf.version geschriebene VersionId, z. B. 2.1.")),
    "BcfSnapshot.version": L("BCF version", "BCF-Version"),
    "set-snapshot:snapshot": L("BCF archive", "BCF-Archiv"),
    "parts": L("Other archive parts", "Weitere Archivteile", ("Archive entries kept byte for byte.", "Archiveinträge, die bytegenau erhalten bleiben.")),
    "BcfRawPart.name": L("Entry name", "Eintragsname"),
    "BcfRawPart.data": L("Entry bytes", "Eintragsbytes"),
}
#endregion 💬️Bcf

#region 💾️Binary
BINARY = {
    "offset": L("Byte offset", "Byte-Versatz", unit="B"),
    "remove_len": L("Bytes to remove", "Zu entfernende Bytes", unit="B"),
    "insert": L("Bytes to insert", "Einzufügende Bytes"),
    "append-bytes:data": L("Bytes to append", "Anzuhängende Bytes"),
    "snapshot": L("Binary content", "Binärinhalt"),
    "bytes": L("Bytes", "Bytes"),
}
#endregion 💾️Binary

#region 📊️Csv
CSV = {
    "record_index": L("Record", "Datensatz", ("Index of the record, counted from 0.", "Index des Datensatzes, ab 0 gezählt.")),
    "field_index": L("Field", "Feld", ("Index of the field within the record, counted from 0.", "Index des Feldes im Datensatz, ab 0 gezählt.")),
    "set-field:value": L("Field value", "Feldwert"),
    "quoted": L("Quoted", "In Anführungszeichen"),
    "remove-record:index": L("Record", "Datensatz", ("Index of the record, counted from 0.", "Index des Datensatzes, ab 0 gezählt.")),
    "insert-record:index": L("Insert position", "Einfügeposition"),
    "record": L("Record", "Datensatz"),
    "records": L("Records", "Datensätze"),
    "fields": L("Fields", "Felder"),
    "CsvField.value": L("Field value", "Feldwert"),
    "has_header": L("First record is a header", "Erster Datensatz ist Kopfzeile"),
    "hasHeader": L("First record is a header", "Erster Datensatz ist Kopfzeile"),
    "snapshot": L("CSV table", "CSV-Tabelle"),
}
#endregion 📊️Csv

#region 📜️Docx
DOCX_PART = ("OPC part name inside the package, e.g. word/document.xml.", "OPC-Teilname im Paket, z. B. word/document.xml.")
DOCX_PART_REF = REF("part", "docx", "Part", "Paketteil", DOCX_PART)
DOCX_STYLE_REF = REF("style", "docx", "Style", "Formatvorlage", ("Style id (w:styleId).", "Formatvorlagen-ID (w:styleId)."))
DOCX_ADDRESS = L("Node address", "Knotenadresse", ("XML part, child path, expected element name and revision stamp of the addressed node.", "XML-Teil, Kindpfad, erwarteter Elementname und Revisionsstempel des adressierten Knotens."))
DOCX = {
    "set-conformance-attribute:value": L("Conformance class", "Konformitätsklasse", ("Value of w:conformance, e.g. strict or transitional.", "Wert von w:conformance, z. B. strict oder transitional.")),
    "insert-vml-part:path": L("Part", "Paketteil", DOCX_PART),
    "insert-vml-part:markup": L("VML markup", "VML-Markup", widget="multiline"),
    "remove-vml-part:path": DOCX_PART_REF,
    "set-main-namespace:namespace": L("Main namespace", "Hauptnamensraum", ("Namespace URI of the WordprocessingML main part.", "Namensraum-URI des WordprocessingML-Hauptteils.")),
    "insert-alternate-content:path": L("Part", "Paketteil", DOCX_PART),
    "remove-alternate-content:path": DOCX_PART_REF,
    "set-relationship-base:base": L("Relationship base URI", "Basis-URI der Beziehungen"),
    "set-part:path": L("Part", "Paketteil", DOCX_PART),
    "set-part:content_type": L("Content type", "Inhaltstyp", ("MIME content type of the part.", "MIME-Inhaltstyp des Teils.")),
    "set-part:bytes": L("Part bytes", "Teilinhalt (Bytes)"),
    "remove-part:path": DOCX_PART_REF,
    "set-block-content:path": L("Block path", "Blockpfad"),
    "set-block-content:block": L("Block", "Block", ("Paragraph or table that replaces the addressed block.", "Absatz oder Tabelle, die den adressierten Block ersetzt.")),
    "insert-block:path": L("Block path", "Blockpfad", ("Where the block is inserted.", "Wo der Block eingefügt wird.")),
    "insert-block:block": L("Block", "Block", ("Paragraph or table to insert.", "Einzufügender Absatz oder Tabelle.")),
    "remove-block:path": L("Block path", "Blockpfad"),
    "segments": L("Segments", "Segmente", ("Descent through tables: block, row and cell per level.", "Abstieg durch Tabellen: Block, Zeile und Zelle je Ebene.")),
    "blockIndex": L("Block", "Block", ("Index of the block, counted from 0.", "Index des Blocks, ab 0 gezählt.")),
    "DocxPathSegment.row": L("Table row", "Tabellenzeile"),
    "cell": L("Table cell", "Tabellenzelle"),
    "DocxBlockPath.index": L("Block position", "Blockposition"),
    "address": DOCX_ADDRESS,
    "partPath": L("Part", "Paketteil", DOCX_PART),
    "nodePath": L("Node path", "Knotenpfad", ("Child indices from the part's root element.", "Kindindizes ab dem Wurzelelement des Teils.")),
    "expectedName": L("Expected element name", "Erwarteter Elementname", ("Expanded name the addressed node must have, e.g. {…/main}p.", "Erweiterter Name, den der adressierte Knoten haben muss, z. B. {…/main}p.")),
    "revision": L("Revision stamp", "Revisionsstempel", ("Lineage revision the address was taken at; a stale stamp refuses the edit.", "Abstammungsrevision, zu der die Adresse erfasst wurde; ein veralteter Stempel lehnt die Bearbeitung ab.")),
    "insert-table-row:index": L("Row position", "Zeilenposition"),
    "remove-table-row:index": L("Row", "Zeile", ("Index of the table row, counted from 0.", "Index der Tabellenzeile, ab 0 gezählt.")),
    "cells": L("Cell texts", "Zelltexte"),
    "set-style-based-on:id": DOCX_STYLE_REF,
    "set-style-name:id": DOCX_STYLE_REF,
    "remove-style:id": DOCX_STYLE_REF,
    "based_on": REF("style", "docx", "Based on", "Basiert auf", ("Parent style id; empty removes the inheritance.", "ID der übergeordneten Formatvorlage; leer entfernt die Vererbung.")),
    "set-style-name:name": L("Style name", "Name der Formatvorlage"),
    "bold": L("Bold", "Fett"),
    "italic": L("Italic", "Kursiv"),
    "underline": L("Underline", "Unterstrichen"),
    "set-run-text:text": L("Run text", "Text des Textlaufs", widget="multiline"),
    "insert-style:style": L("Style", "Formatvorlage"),
    "DocxStyle.id": L("Style id", "Formatvorlagen-ID"),
    "DocxStyle.name": L("Style name", "Name der Formatvorlage"),
    "DocxStyle.basedOn": L("Based on", "Basiert auf"),
    "styleId": REF("style", "docx", "Paragraph style", "Absatzformatvorlage", ("Style id; empty removes the paragraph style.", "Formatvorlagen-ID; leer entfernt die Absatzformatvorlage.")),
    "insert-xml-node:parent": L("Parent element", "Übergeordnetes Element", ("Address of the element the node is inserted into.", "Adresse des Elements, in das der Knoten eingefügt wird.")),
    "insert-xml-node:index": L("Child position", "Kindposition"),
    "insert-xml-node:node": L("XML node", "XML-Knoten", group="value"),
    "replace-xml-node:node": L("XML node", "XML-Knoten", ("Node that replaces the addressed one.", "Knoten, der den adressierten ersetzt."), group="value"),
    "remove-xml-node:parent": L("Parent element", "Übergeordnetes Element", ("Address of the element the node is removed from.", "Adresse des Elements, aus dem der Knoten entfernt wird.")),
    "remove-xml-node:index": L("Child position", "Kindposition"),
    "snapshot": L("Word document", "Word-Dokument"),
    "opc": L("OPC package", "OPC-Paket"),
    "OpcPackage.parts": L("Parts", "Paketteile"),
    "OpcPart.path": L("Part name", "Teilname"),
    "contentType": L("Content type", "Inhaltstyp"),
    "OpcPart.bytes": L("Part bytes", "Teilinhalt (Bytes)"),
    "contentTypes": L("Content types", "Inhaltstypen"),
    "defaults": L("Default mappings", "Standardzuordnungen", ("Extension to content type.", "Dateierweiterung zu Inhaltstyp.")),
    "overrides": L("Override mappings", "Überschreibungen", ("Part name to content type.", "Teilname zu Inhaltstyp.")),
    "relationships": L("Relationships", "Beziehungen"),
    "OpcPackage.comment": L("ZIP comment", "ZIP-Kommentar"),
    "xmlParts": L("XML parts", "XML-Teile"),
    "DocxXmlPart.path": L("Part name", "Teilname"),
    "DocxXmlPart.document": L("XML document", "XML-Dokument"),
}
XML = {
    "root": L("Root element", "Wurzelelement"),
    "XmlDocument.doctype": L("Document type declaration", "Dokumenttypdeklaration"),
    "prologPosition": L("Prolog position", "Position im Prolog", ("Number of prolog nodes before the declaration.", "Anzahl der Prologknoten vor der Deklaration.")),
    "XmlDoctype.name": L("Root element name", "Name des Wurzelelements"),
    "externalId": L("External identifier", "Externer Bezeichner"),
    "declarations": L("Internal subset declarations", "Deklarationen der internen Teilmenge"),
    "parameter": L("Parameter entity", "Parameter-Entität"),
    "XmlDtdDeclaration.name": L("Name", "Name"),
    "XmlDtdDeclaration.value": L("Declaration", "Deklaration"),
    "declaration": L("XML declaration", "XML-Deklaration"),
    "XmlDeclaration.version": L("XML version", "XML-Version"),
    "encoding": L("Encoding", "Zeichenkodierung"),
    "standalone": L("Standalone", "Eigenständig (standalone)"),
    "quote": L("Quote character", "Anführungszeichen"),
    "prolog": L("Prolog", "Prolog"),
    "epilog": L("Epilog", "Epilog"),
}
XML_OPTIONS = {"quote": {"double": D("Double quotes (\")", "Doppelte Anführungszeichen (\")"), "single": D("Single quotes (')", "Einfache Anführungszeichen (')")}}
#endregion 📜️Docx

#region 📼️Avi
AVI_STREAM = L("Stream", "Datenstrom", ("Index of the stream, counted from 0.", "Index des Datenstroms, ab 0 gezählt."))
AVI = {
    "stream_index": AVI_STREAM,
    "strh": L("Stream header", "Datenstrom-Header", ("strh chunk", "strh-Chunk")),
    "strf": L("Stream format", "Datenstromformat", ("strf chunk: BITMAPINFOHEADER or WAVEFORMATEX.", "strf-Chunk: BITMAPINFOHEADER oder WAVEFORMATEX.")),
    "fccType": L("Stream type (FourCC)", "Datenstromtyp (FourCC)", ("vids, auds, txts or mids.", "vids, auds, txts oder mids.")),
    "fccHandler": L("Handler (FourCC)", "Codec-Handler (FourCC)"),
    "flags": L("Flags", "Flags"),
    "priority": L("Priority", "Priorität"),
    "language": L("Language", "Sprache"),
    "initialFrames": L("Initial frames", "Anfangsframes"),
    "AviStreamHeader.scale": L("Time scale", "Zeitbasis (Scale)", ("Rate / scale gives samples per second.", "Rate / Scale ergibt Samples pro Sekunde.")),
    "rate": L("Rate", "Rate", ("Rate / scale gives samples per second.", "Rate / Scale ergibt Samples pro Sekunde.")),
    "start": L("Start", "Start", ("Start time in scale units.", "Startzeit in Scale-Einheiten.")),
    "length": L("Length", "Länge", ("Length in scale units.", "Länge in Scale-Einheiten.")),
    "suggestedBufferSize": L("Suggested buffer size", "Empfohlene Puffergröße", unit="B"),
    "quality": L("Quality", "Qualität", ("0 to 10000, or -1 for the driver default.", "0 bis 10000 oder -1 für den Treiberstandard.")),
    "sampleSize": L("Sample size", "Samplegröße", unit="B"),
    "rcFrameLeft": L("Frame rectangle left", "Bildrechteck links", unit="px"),
    "rcFrameTop": L("Frame rectangle top", "Bildrechteck oben", unit="px"),
    "rcFrameRight": L("Frame rectangle right", "Bildrechteck rechts", unit="px"),
    "rcFrameBottom": L("Frame rectangle bottom", "Bildrechteck unten", unit="px"),
    "rcFrameWidth": L("Frame rectangle size", "Größe des Bildrechtecks", ("Bytes of rcFrame on disk: 8 for 16-bit, 16 for 32-bit coordinates.", "Bytes von rcFrame in der Datei: 8 für 16-Bit-, 16 für 32-Bit-Koordinaten."), unit="B"),
    "strhExtra": L("Extra header bytes", "Zusätzliche Header-Bytes"),
    "main_header": L("Main header", "Hauptheader", ("avih chunk", "avih-Chunk")),
    "mainHeader": L("Main header", "Hauptheader"),
    "microSecPerFrame": L("Frame duration", "Framedauer", unit="µs"),
    "maxBytesPerSec": L("Maximum data rate", "Maximale Datenrate", unit="B/s"),
    "paddingGranularity": L("Padding granularity", "Auffüllgranularität", unit="B"),
    "totalFrames": L("Total frames", "Gesamtzahl Frames"),
    "AviMainHeader.streams": L("Stream count", "Anzahl Datenströme"),
    "AviMainHeader.width": L("Width", "Breite", unit="px"),
    "AviMainHeader.height": L("Height", "Höhe", unit="px"),
    "reserved": L("Reserved", "Reserviert"),
    "idx1_present": L("Legacy index (idx1)", "Legacy-Index (idx1)", ("Write the idx1 chunk index.", "Den Chunk-Index idx1 schreiben.")),
    "idx1Present": L("Legacy index (idx1)", "Legacy-Index (idx1)"),
    "remove-stream:index": L("Stream", "Datenstrom", ("Index of the stream, counted from 0.", "Index des Datenstroms, ab 0 gezählt.")),
    "insert-stream:index": L("Insert position", "Einfügeposition"),
    "chunks": L("Chunks", "Chunks"),
    "strlExtra": L("Extra stream list chunks", "Zusätzliche Chunks der Datenstromliste"),
    "fourcc": L("FourCC", "FourCC"),
    "AviChunk.data": L("Chunk data", "Chunk-Daten"),
    "RiffChunk.data": L("Chunk data", "Chunk-Daten"),
    "keyframe": L("Keyframe", "Schlüsselbild"),
    "snapshot": L("AVI video", "AVI-Video"),
    "AviSnapshot.streams": L("Streams", "Datenströme"),
    "unknownChunks": L("Unknown chunks", "Unbekannte Chunks"),
    "hdrlExtra": L("Extra header list chunks", "Zusätzliche Chunks der Header-Liste"),
    "set-chunk-keyframe:index": L("Chunk", "Chunk", ("Index of the chunk within the stream, counted from 0.", "Index des Chunks im Datenstrom, ab 0 gezählt.")),
    "remove-chunk:index": L("Chunk", "Chunk", ("Index of the chunk within the stream, counted from 0.", "Index des Chunks im Datenstrom, ab 0 gezählt.")),
    "insert-chunk:index": L("Insert position", "Einfügeposition"),
    "insert-chunk:chunk": L("Chunk", "Chunk"),
    "add-unknown-chunk:index": L("Insert position", "Einfügeposition"),
    "add-unknown-chunk:item": L("Chunk", "Chunk"),
    "remove-unknown-chunk:index": L("Chunk", "Chunk", ("Index of the unknown chunk, counted from 0.", "Index des unbekannten Chunks, ab 0 gezählt.")),
}
#endregion 📼️Avi

#region 🖋️Dxf
DXF = {
    "set-layer:name": REF("layer", "dxf", "Layer", "Layer", ("Name of the layer to replace.", "Name des zu ersetzenden Layers.")),
    "remove-layer:name": REF("layer", "dxf", "Layer", "Layer"),
    "set-style:name": REF("textStyle", "dxf", "Text style", "Textstil", ("Name of the text style to replace.", "Name des zu ersetzenden Textstils.")),
    "remove-style:name": REF("textStyle", "dxf", "Text style", "Textstil"),
    "set-linetype:name": REF("linetype", "dxf", "Linetype", "Linientyp", ("Name of the linetype to replace.", "Name des zu ersetzenden Linientyps.")),
    "remove-linetype:name": REF("linetype", "dxf", "Linetype", "Linientyp"),
    "set-header-var:name": REF("headerVariable", "dxf", "Header variable", "Systemvariable", ("Name including $, e.g. $ACADVER.", "Name mit $, z. B. $ACADVER.")),
    "remove-header-var:name": REF("headerVariable", "dxf", "Header variable", "Systemvariable", ("Name including $, e.g. $ACADVER.", "Name mit $, z. B. $ACADVER.")),
    "layer": L("Layer", "Layer"),
    "layers": L("Layers", "Layer"),
    "DxfLayer.color": L("Color number (ACI)", "Farbnummer (ACI)", ("AutoCAD Color Index; negative means the layer is off.", "AutoCAD-Farbindex; negativ bedeutet, der Layer ist ausgeschaltet.")),
    "DxfLayer.linetype": L("Linetype", "Linientyp"),
    "flags": L("Standard flags", "Standard-Flags", ("Group code 70.", "Gruppencode 70.")),
    "unknownGroupCodes": L("Unknown group codes", "Unbekannte Gruppencodes"),
    "style": L("Text style", "Textstil"),
    "styles": L("Text styles", "Textstile"),
    "fontName": L("Font file", "Schriftdatei"),
    "header_var": L("Header variable", "Systemvariable"),
    "headerVars": L("Header variables", "Systemvariablen"),
    "groupCode": L("Group code", "Gruppencode"),
    "DxfHeaderVar.value": L("Value", "Wert"),
    "extraGroupCodes": L("Additional group codes", "Zusätzliche Gruppencodes"),
    "block": L("Block", "Block"),
    "blocks": L("Blocks", "Blöcke"),
    "basePoint": L("Base point", "Basispunkt"),
    "entities": L("Entities", "Objekte"),
    "entity": L("Entity", "Objekt"),
    "linetype": L("Linetype", "Linientyp"),
    "linetypes": L("Linetypes", "Linientypen"),
    "tables": L("Tables", "Tabellen"),
    "otherTables": L("Other tables", "Weitere Tabellen"),
    "DxfOtherTable.name": L("Table name", "Tabellenname"),
    "tags": L("Group code pairs", "Gruppencode-Paare"),
    "code": L("Group code", "Gruppencode"),
    "DxfTag.value": L("Value", "Wert"),
    "insert-style:index": L("Insert position", "Einfügeposition"),
    "insert-block:index": L("Insert position", "Einfügeposition"),
    "insert-entity:index": L("Insert position", "Einfügeposition"),
    "insert-layer:index": L("Insert position", "Einfügeposition"),
    "insert-linetype:index": L("Insert position", "Einfügeposition"),
    "set-entity:index": L("Entity", "Objekt", ("Index of the entity in the ENTITIES section, counted from 0.", "Index des Objekts im Abschnitt ENTITIES, ab 0 gezählt.")),
    "remove-entity:index": L("Entity", "Objekt", ("Index of the entity in the ENTITIES section, counted from 0.", "Index des Objekts im Abschnitt ENTITIES, ab 0 gezählt.")),
    "set-block:index": L("Block", "Block", ("Index of the block in the BLOCKS section, counted from 0.", "Index des Blocks im Abschnitt BLOCKS, ab 0 gezählt.")),
    "remove-block:index": L("Block", "Block", ("Index of the block in the BLOCKS section, counted from 0.", "Index des Blocks im Abschnitt BLOCKS, ab 0 gezählt.")),
    "snapshot": L("DXF drawing", "DXF-Zeichnung"),
}
#endregion 🖋️Dxf

#region 🗜️Deflate
DEFLATE = {
    "dict_id": L("Preset dictionary (DICTID)", "Vorgabewörterbuch (DICTID)", ("Adler-32 of the preset dictionary; empty clears FDICT.", "Adler-32 des Vorgabewörterbuchs; leer löscht FDICT.")),
    "dictId": L("Preset dictionary (DICTID)", "Vorgabewörterbuch (DICTID)", ("Adler-32 of the preset dictionary.", "Adler-32 des Vorgabewörterbuchs.")),
    "payload": L("Payload", "Nutzdaten", ("Uncompressed bytes carried by the zlib stream.", "Unkomprimierte Bytes des zlib-Datenstroms.")),
    "method": L("Compression method (CM)", "Kompressionsverfahren (CM)", ("8 is deflate.", "8 ist Deflate.")),
    "compressionMethod": L("Compression method (CM)", "Kompressionsverfahren (CM)", ("8 is deflate.", "8 ist Deflate.")),
    "window_bits": L("Window size", "Fenstergröße", ("Base-2 logarithm of the LZ77 window, 8 to 15.", "Zweierlogarithmus des LZ77-Fensters, 8 bis 15."), unit="bit", softMin=8, softMax=15),
    "windowBits": L("Window size", "Fenstergröße", ("Base-2 logarithm of the LZ77 window, 8 to 15.", "Zweierlogarithmus des LZ77-Fensters, 8 bis 15.")),
    "level_hint": L("Compression level (FLEVEL)", "Kompressionsstufe (FLEVEL)"),
    "compressionLevelHint": L("Compression level (FLEVEL)", "Kompressionsstufe (FLEVEL)"),
    "snapshot": L("zlib stream", "zlib-Datenstrom"),
}
DEFLATE_LEVELS = {"fastest": D("Fastest", "Am schnellsten"), "fast": D("Fast", "Schnell"), "default": D("Default", "Standard"), "maximum": D("Maximum compression", "Maximale Kompression")}
DEFLATE_OPTIONS = {"level_hint": DEFLATE_LEVELS, "compressionLevelHint": DEFLATE_LEVELS}
#endregion 🗜️Deflate

#region 🪟️Bmp
BMP = {
    "replace-palette-entry:index": L("Palette entry", "Paletteneintrag", ("Index of the palette entry, counted from 0.", "Index des Paletteneintrags, ab 0 gezählt.")),
    "remove-palette-entry:index": L("Palette entry", "Paletteneintrag", ("Index of the palette entry, counted from 0.", "Index des Paletteneintrags, ab 0 gezählt.")),
    "insert-palette-entry:index": L("Insert position", "Einfügeposition"),
    "entry": L("Palette entry", "Paletteneintrag"),
    "b": L("Blue", "Blau"),
    "g": L("Green", "Grün"),
    "r": L("Red", "Rot"),
    "reserved": L("Reserved", "Reserviert"),
    "headerSize": L("Header size", "Headergröße", ("biSize: 40 for BITMAPINFOHEADER.", "biSize: 40 für BITMAPINFOHEADER."), unit="B"),
    "width": L("Width", "Breite", unit="px"),
    "height": L("Height", "Höhe", unit="px"),
    "rowOrder": L("Row order", "Zeilenreihenfolge"),
    "planes": L("Color planes", "Farbebenen", ("Always 1.", "Immer 1.")),
    "bitsPerPixel": L("Bits per pixel", "Bits pro Pixel", ("Color depth: 1, 4, 8, 16, 24 or 32.", "Farbtiefe: 1, 4, 8, 16, 24 oder 32.")),
    "compression": L("Compression", "Kompression", ("biCompression, e.g. 0 = BI_RGB, 1 = BI_RLE8.", "biCompression, z. B. 0 = BI_RGB, 1 = BI_RLE8.")),
    "imageSize": L("Image size", "Bildgröße", unit="B"),
    "xPixelsPerMeter": L("Horizontal resolution", "Horizontale Auflösung", unit="px/m"),
    "yPixelsPerMeter": L("Vertical resolution", "Vertikale Auflösung", unit="px/m"),
    "colorsUsed": L("Colors used", "Verwendete Farben"),
    "colorsImportant": L("Important colors", "Wichtige Farben"),
    "palette": L("Palette", "Palette"),
    "pixels": L("Pixels", "Pixel", ("Canonical 8-bit RGBA, width × height × 4 bytes, row 0 at the top.", "Kanonisches 8-Bit-RGBA, Breite × Höhe × 4 Bytes, Zeile 0 oben.")),
    "snapshot": L("Bitmap", "Bitmap"),
}
BMP_OPTIONS = {"rowOrder": {"bottomUp": D("Bottom-up", "Von unten nach oben"), "topDown": D("Top-down", "Von oben nach unten")}}
#endregion 🪟️Bmp

#region 🖊️Dwg
DWG_HANDLE = ("Object handle (hexadecimal in AutoCAD, stored as integer).", "Objektreferenz (in AutoCAD hexadezimal, als Ganzzahl gespeichert).")
DWG = {
    "set-version-info:version": L("DWG version", "DWG-Version", ("Version string, e.g. AC1024.", "Versionskennung, z. B. AC1024.")),
    "maintenance_version": L("Maintenance version", "Wartungsversion"),
    "maintenanceVersion": L("Maintenance version", "Wartungsversion"),
    "codepage": L("Code page", "Codepage", ("Windows code page number of the drawing text.", "Windows-Codepage des Zeichnungstexts.")),
    "snapshot": L("DWG drawing", "DWG-Zeichnung"),
    "DwgSnapshot.version": L("DWG version", "DWG-Version"),
    "drawing": L("Drawing", "Zeichnung"),
    "DwgSnapshot.header": L("Header variables", "Systemvariablen der Zeichnung"),
    "classes": L("Classes", "Klassen"),
    "dependencies": L("File dependencies", "Dateiabhängigkeiten"),
    "summary": L("Drawing properties", "Zeichnungseigenschaften"),
    "application": L("Application information", "Anwendungsinformationen"),
    "template": L("Template", "Vorlage"),
    "auxiliaryHeader": L("Auxiliary header", "Zusatz-Header"),
    "revisionHistory": L("Revision history", "Revisionsverlauf"),
    "preview": L("Preview image", "Vorschaubild"),
    "applicationHistory": L("Application history", "Anwendungsverlauf"),
    "DwgLogicalDrawing.layers": L("Layers", "Layer"),
    "DwgLogicalDrawing.objects": L("Objects", "Objekte"),
    "extmin": L("Extents minimum", "Ausdehnung Minimum", ("System variable EXTMIN.", "Systemvariable EXTMIN.")),
    "extmax": L("Extents maximum", "Ausdehnung Maximum", ("System variable EXTMAX.", "Systemvariable EXTMAX.")),
    "DwgLogicalLayer.color": L("Color number (ACI)", "Farbnummer (ACI)"),
    "DwgLogicalObject.handle": L("Handle", "Referenz (Handle)", DWG_HANDLE),
    "typeCode": L("Object type code", "Objekttypcode"),
    "className": L("Class name", "Klassenname"),
    "category": L("Category", "Kategorie"),
    "ownerHandle": L("Owner handle", "Eigentümerreferenz"),
    "reactorHandles": L("Reactor handles", "Reaktorreferenzen"),
    "extensionDictionaryHandle": L("Extension dictionary", "Erweiterungswörterbuch", DWG_HANDLE),
    "referencedHandles": L("Referenced objects", "Referenzierte Objekte"),
    "extendedData": L("Extended entity data (XDATA)", "Erweiterte Objektdaten (XDATA)"),
    "body": L("Object data", "Objektdaten"),
    "applicationHandle": L("Registered application", "Registrierte Anwendung", DWG_HANDLE),
    "DwgExtendedEntityData.values": L("Values", "Werte"),
    "units": L("Legacy units", "Alte Einheiten"),
    "unit1Conversion": L("Unit 1 conversion factor", "Umrechnungsfaktor Einheit 1"),
    "unit2Conversion": L("Unit 2 conversion factor", "Umrechnungsfaktor Einheit 2"),
    "unit3Conversion": L("Unit 3 conversion factor", "Umrechnungsfaktor Einheit 3"),
    "unit4Conversion": L("Unit 4 conversion factor", "Umrechnungsfaktor Einheit 4"),
    "unit1Name": L("Unit 1 name", "Name Einheit 1"),
    "unit2Name": L("Unit 2 name", "Name Einheit 2"),
    "unit3Name": L("Unit 3 name", "Name Einheit 3"),
    "unit4Name": L("Unit 4 name", "Name Einheit 4"),
    "modes": L("Modes", "Modi"),
    "dimensionAssociative": L("Associative dimensioning (DIMASO)", "Assoziative Bemaßung (DIMASO)"),
    "dimensionShow": L("Update dimensions while dragging (DIMSHO)", "Bemaßung beim Ziehen aktualisieren (DIMSHO)"),
    "polylineGeneration": L("Polyline linetype generation (PLINEGEN)", "Linientypgenerierung für Polylinien (PLINEGEN)"),
    "orthographicMode": L("Ortho mode (ORTHOMODE)", "Ortho-Modus (ORTHOMODE)"),
    "regenerationMode": L("Automatic regeneration (REGENMODE)", "Automatische Regenerierung (REGENMODE)"),
    "fillMode": L("Fill mode (FILLMODE)", "Füllmodus (FILLMODE)"),
    "quickTextMode": L("Quick text (QTEXTMODE)", "Schnelltext (QTEXTMODE)"),
    "paperSpaceLinetypeScale": L("Paper space linetype scaling (PSLTSCALE)", "Linientypskalierung im Papierbereich (PSLTSCALE)"),
    "limitsCheck": L("Limits checking (LIMCHECK)", "Limitenkontrolle (LIMCHECK)"),
    "userTimer": L("User timer (USRTIMER)", "Benutzer-Zeitgeber (USRTIMER)"),
    "sketchPolyline": L("Sketch as polylines (SKPOLY)", "Skizze als Polylinien (SKPOLY)"),
    "angleDirection": L("Clockwise angles (ANGDIR)", "Winkel im Uhrzeigersinn (ANGDIR)"),
    "splineFrame": L("Show spline frame (SPLFRAME)", "Spline-Rahmen anzeigen (SPLFRAME)"),
    "mirrorText": L("Mirror text (MIRRTEXT)", "Text spiegeln (MIRRTEXT)"),
    "worldView": L("View relative to WCS (WORLDVIEW)", "Ansicht relativ zum WKS (WORLDVIEW)"),
    "tileMode": L("Model tab active (TILEMODE)", "Modellbereich aktiv (TILEMODE)"),
    "paperLimitsCheck": L("Paper space limits checking (PLIMCHECK)", "Limitenkontrolle im Papierbereich (PLIMCHECK)"),
    "visualRetain": L("Retain xref layer settings (VISRETAIN)", "Xref-Layereinstellungen beibehalten (VISRETAIN)"),
    "displaySilhouette": L("Display silhouettes (DISPSILH)", "Silhouettenkanten anzeigen (DISPSILH)"),
    "polylineEllipse": L("Ellipses as polylines (PELLIPSE)", "Ellipsen als Polylinien (PELLIPSE)"),
    "integers": L("Integer settings", "Ganzzahlige Einstellungen"),
    "proxyGraphics": L("Save proxy graphics (PROXYGRAPHICS)", "Proxy-Grafiken speichern (PROXYGRAPHICS)"),
    "treeDepth": L("Spatial index depth (TREEDEPTH)", "Tiefe des räumlichen Index (TREEDEPTH)"),
    "linearUnits": L("Linear unit format (LUNITS)", "Längeneinheitenformat (LUNITS)"),
    "linearPrecision": L("Linear precision (LUPREC)", "Längengenauigkeit (LUPREC)"),
    "angularUnits": L("Angular unit format (AUNITS)", "Winkeleinheitenformat (AUNITS)"),
    "angularPrecision": L("Angular precision (AUPREC)", "Winkelgenauigkeit (AUPREC)"),
    "attributeMode": L("Attribute display (ATTMODE)", "Attributanzeige (ATTMODE)"),
    "pointDisplayMode": L("Point style (PDMODE)", "Punktstil (PDMODE)"),
    "userInteger1": L("User integer 1 (USERI1)", "Benutzer-Ganzzahl 1 (USERI1)"),
    "userInteger2": L("User integer 2 (USERI2)", "Benutzer-Ganzzahl 2 (USERI2)"),
    "userInteger3": L("User integer 3 (USERI3)", "Benutzer-Ganzzahl 3 (USERI3)"),
    "userInteger4": L("User integer 4 (USERI4)", "Benutzer-Ganzzahl 4 (USERI4)"),
    "userInteger5": L("User integer 5 (USERI5)", "Benutzer-Ganzzahl 5 (USERI5)"),
    "splineSegments": L("Spline segments (SPLINESEGS)", "Spline-Segmente (SPLINESEGS)"),
    "surfaceU": L("Surface density U (SURFU)", "Flächendichte U (SURFU)"),
    "surfaceV": L("Surface density V (SURFV)", "Flächendichte V (SURFV)"),
    "surfaceType": L("Surface fit type (SURFTYPE)", "Flächenglättungstyp (SURFTYPE)"),
    "surfaceTab1": L("Mesh density 1 (SURFTAB1)", "Netzdichte 1 (SURFTAB1)"),
    "surfaceTab2": L("Mesh density 2 (SURFTAB2)", "Netzdichte 2 (SURFTAB2)"),
    "splineType": L("Spline curve type (SPLINETYPE)", "Spline-Kurventyp (SPLINETYPE)"),
    "shadeEdge": L("Shaded edge mode (SHADEDGE)", "Kantenschattierung (SHADEDGE)"),
    "shadeDifference": L("Diffuse to ambient ratio (SHADEDIF)", "Verhältnis diffuses Licht zu Umgebungslicht (SHADEDIF)"),
    "unitMode": L("Unit display mode (UNITMODE)", "Einheitenanzeigemodus (UNITMODE)"),
    "maximumActiveViewports": L("Maximum active viewports (MAXACTVP)", "Maximal aktive Ansichtsfenster (MAXACTVP)"),
    "isolines": L("Isolines (ISOLINES)", "Isolinien (ISOLINES)"),
    "multilineJustification": L("Multiline justification (CMLJUST)", "Multilinien-Ausrichtung (CMLJUST)"),
    "textQuality": L("Text quality (TEXTQLTY)", "Textqualität (TEXTQLTY)"),
    "scalars": L("Numeric settings", "Numerische Einstellungen"),
    "linetypeScale": L("Global linetype scale (LTSCALE)", "Globaler Linientypfaktor (LTSCALE)"),
    "textSize": L("Default text height (TEXTSIZE)", "Standard-Texthöhe (TEXTSIZE)"),
    "traceWidth": L("Trace width (TRACEWID)", "Bandbreite (TRACEWID)"),
    "sketchIncrement": L("Sketch increment (SKETCHINC)", "Skizzierinkrement (SKETCHINC)"),
    "filletRadius": L("Fillet radius (FILLETRAD)", "Abrundungsradius (FILLETRAD)"),
    "thickness": L("Thickness (THICKNESS)", "Objekthöhe (THICKNESS)"),
    "angleBase": L("Angle base (ANGBASE)", "Winkelbasis (ANGBASE)"),
    "pointDisplaySize": L("Point size (PDSIZE)", "Punktgröße (PDSIZE)"),
    "polylineWidth": L("Polyline width (PLINEWID)", "Polylinienbreite (PLINEWID)"),
    "userReal1": L("User real 1 (USERR1)", "Benutzer-Realzahl 1 (USERR1)"),
    "userReal2": L("User real 2 (USERR2)", "Benutzer-Realzahl 2 (USERR2)"),
    "userReal3": L("User real 3 (USERR3)", "Benutzer-Realzahl 3 (USERR3)"),
    "userReal4": L("User real 4 (USERR4)", "Benutzer-Realzahl 4 (USERR4)"),
    "userReal5": L("User real 5 (USERR5)", "Benutzer-Realzahl 5 (USERR5)"),
    "chamferA": L("First chamfer distance (CHAMFERA)", "Erster Fasenabstand (CHAMFERA)"),
    "chamferB": L("Second chamfer distance (CHAMFERB)", "Zweiter Fasenabstand (CHAMFERB)"),
    "chamferC": L("Chamfer length (CHAMFERC)", "Fasenlänge (CHAMFERC)"),
    "chamferD": L("Chamfer angle (CHAMFERD)", "Fasenwinkel (CHAMFERD)"),
    "facetResolution": L("Facet resolution (FACETRES)", "Facettenauflösung (FACETRES)"),
    "multilineScale": L("Multiline scale (CMLSCALE)", "Multilinienmaßstab (CMLSCALE)"),
    "currentEntityLinetypeScale": L("Current object linetype scale (CELTSCALE)", "Aktueller Objekt-Linientypfaktor (CELTSCALE)"),
    "currentEntityColorIndex": L("Current object color (CECOLOR)", "Aktuelle Objektfarbe (CECOLOR)"),
    "paperSpaceViewportScale": L("Viewport scale (PSVPSCALE)", "Ansichtsfenster-Maßstab (PSVPSCALE)"),
    "time": L("Time stamps", "Zeitstempel"),
    "DwgHeaderTimeState.createdAt": L("Created (TDCREATE)", "Erstellt (TDCREATE)"),
    "DwgHeaderTimeState.updatedAt": L("Last updated (TDUPDATE)", "Zuletzt aktualisiert (TDUPDATE)"),
    "editingDuration": L("Total editing time (TDINDWG)", "Gesamtbearbeitungszeit (TDINDWG)"),
    "userTimerDuration": L("User timer (TDUSRTIMER)", "Benutzer-Zeitgeber (TDUSRTIMER)"),
    "days": L("Julian day", "Julianischer Tag"),
    "milliseconds": L("Milliseconds into the day", "Millisekunden des Tages"),
    "paperSpace": L("Paper space", "Papierbereich"),
    "modelSpace": L("Model space", "Modellbereich"),
    "insertionBase": L("Insertion base point (INSBASE)", "Einfügebasispunkt (INSBASE)"),
    "extentsMinimum": L("Extents minimum (EXTMIN)", "Ausdehnung Minimum (EXTMIN)"),
    "extentsMaximum": L("Extents maximum (EXTMAX)", "Ausdehnung Maximum (EXTMAX)"),
    "limitsMinimum": L("Limits minimum (LIMMIN)", "Limiten Minimum (LIMMIN)"),
    "limitsMaximum": L("Limits maximum (LIMMAX)", "Limiten Maximum (LIMMAX)"),
    "DwgHeaderSpaceGeometry.elevation": L("Elevation (ELEVATION)", "Erhebung (ELEVATION)"),
    "ucsOrigin": L("UCS origin (UCSORG)", "BKS-Ursprung (UCSORG)"),
    "ucsXAxis": L("UCS X axis (UCSXDIR)", "BKS-X-Achse (UCSXDIR)"),
    "ucsYAxis": L("UCS Y axis (UCSYDIR)", "BKS-Y-Achse (UCSYDIR)"),
    "ucsOrthographicView": L("Orthographic UCS type (UCSORTHOVIEW)", "Orthogonaler BKS-Typ (UCSORTHOVIEW)"),
    "ucsOriginTop": L("UCS origin, top", "BKS-Ursprung oben"),
    "ucsOriginBottom": L("UCS origin, bottom", "BKS-Ursprung unten"),
    "ucsOriginLeft": L("UCS origin, left", "BKS-Ursprung links"),
    "ucsOriginRight": L("UCS origin, right", "BKS-Ursprung rechts"),
    "ucsOriginFront": L("UCS origin, front", "BKS-Ursprung vorne"),
    "ucsOriginBack": L("UCS origin, back", "BKS-Ursprung hinten"),
    "dimensions": L("Dimension settings", "Bemaßungseinstellungen"),
    "DwgDimensionSettings.scale": L("Overall dimension scale (DIMSCALE)", "Globaler Bemaßungsfaktor (DIMSCALE)"),
    "arrowSize": L("Arrow size (DIMASZ)", "Pfeilgröße (DIMASZ)"),
    "extensionOffset": L("Extension line offset (DIMEXO)", "Abstand der Maßhilfslinie (DIMEXO)"),
    "lineIncrement": L("Baseline spacing (DIMDLI)", "Abstand der Basislinienbemaßung (DIMDLI)"),
    "DwgDimensionSettings.extension": L("Extension line extension (DIMEXE)", "Überstand der Maßhilfslinie (DIMEXE)"),
    "rounding": L("Rounding (DIMRND)", "Rundung (DIMRND)"),
    "lineExtension": L("Dimension line extension (DIMDLE)", "Maßlinienüberstand (DIMDLE)"),
    "tolerancePlus": L("Upper tolerance (DIMTP)", "Obere Toleranz (DIMTP)"),
    "toleranceMinus": L("Lower tolerance (DIMTM)", "Untere Toleranz (DIMTM)"),
    "fixedExtensionLength": L("Fixed extension line length (DIMFXL)", "Feste Länge der Maßhilfslinie (DIMFXL)"),
    "jogAngle": L("Jog angle (DIMJOGANG)", "Knickwinkel (DIMJOGANG)"),
    "textFill": L("Text background fill (DIMTFILL)", "Texthintergrundfüllung (DIMTFILL)"),
    "textFillColorIndex": L("Text background color (DIMTFILLCLR)", "Texthintergrundfarbe (DIMTFILLCLR)"),
    "tolerance": L("Generate tolerances (DIMTOL)", "Toleranzen erzeugen (DIMTOL)"),
    "DwgDimensionSettings.limits": L("Generate limits (DIMLIM)", "Grenzmaße erzeugen (DIMLIM)"),
    "textInsideHorizontal": L("Text inside horizontal (DIMTIH)", "Text innen horizontal (DIMTIH)"),
    "textOutsideHorizontal": L("Text outside horizontal (DIMTOH)", "Text außen horizontal (DIMTOH)"),
    "suppressExtension1": L("Suppress first extension line (DIMSE1)", "Erste Maßhilfslinie unterdrücken (DIMSE1)"),
    "suppressExtension2": L("Suppress second extension line (DIMSE2)", "Zweite Maßhilfslinie unterdrücken (DIMSE2)"),
    "textAbove": L("Vertical text position (DIMTAD)", "Vertikale Textposition (DIMTAD)"),
    "zeroSuppression": L("Zero suppression (DIMZIN)", "Nullenunterdrückung (DIMZIN)"),
    "angularZeroSuppression": L("Angular zero suppression (DIMAZIN)", "Nullenunterdrückung bei Winkeln (DIMAZIN)"),
    "arcSymbol": L("Arc length symbol (DIMARCSYM)", "Bogenlängensymbol (DIMARCSYM)"),
    "textHeight": L("Text height (DIMTXT)", "Texthöhe (DIMTXT)"),
    "centerMark": L("Center mark size (DIMCEN)", "Größe der Mittelpunktmarkierung (DIMCEN)"),
    "tickSize": L("Tick size (DIMTSZ)", "Schrägstrichgröße (DIMTSZ)"),
    "alternateScale": L("Alternate unit scale factor (DIMALTF)", "Faktor der Alternativeinheiten (DIMALTF)"),
    "linearFactor": L("Linear scale factor (DIMLFAC)", "Längenmaßstabsfaktor (DIMLFAC)"),
    "textVerticalPosition": L("Text vertical position (DIMTVP)", "Vertikale Textverschiebung (DIMTVP)"),
    "textFactor": L("Tolerance text height factor (DIMTFAC)", "Skalierfaktor der Toleranztexthöhe (DIMTFAC)"),
    "DwgDimensionSettings.gap": L("Dimension line gap (DIMGAP)", "Abstand Text zu Maßlinie (DIMGAP)"),
    "alternateRounding": L("Alternate unit rounding (DIMALTRND)", "Rundung der Alternativeinheiten (DIMALTRND)"),
    "alternateUnits": L("Alternate units (DIMALT)", "Alternativeinheiten (DIMALT)"),
    "alternateDecimalPlaces": L("Alternate unit decimal places (DIMALTD)", "Nachkommastellen der Alternativeinheiten (DIMALTD)"),
    "textOutsideForceLine": L("Force dimension line inside (DIMTOFL)", "Maßlinie zwischen Hilfslinien erzwingen (DIMTOFL)"),
    "separateArrows": L("Separate arrow blocks (DIMSAH)", "Separate Pfeilblöcke (DIMSAH)"),
    "textInside": L("Force text inside (DIMTIX)", "Text innerhalb erzwingen (DIMTIX)"),
    "suppressOutside": L("Suppress outside dimension lines (DIMSOXD)", "Äußere Maßlinien unterdrücken (DIMSOXD)"),
    "lineColorIndex": L("Dimension line color (DIMCLRD)", "Maßlinienfarbe (DIMCLRD)"),
    "extensionColorIndex": L("Extension line color (DIMCLRE)", "Maßhilfslinienfarbe (DIMCLRE)"),
    "textColorIndex": L("Dimension text color (DIMCLRT)", "Maßtextfarbe (DIMCLRT)"),
    "angularDecimalPlaces": L("Angular decimal places (DIMADEC)", "Nachkommastellen bei Winkeln (DIMADEC)"),
    "decimalPlaces": L("Decimal places (DIMDEC)", "Nachkommastellen (DIMDEC)"),
    "toleranceDecimalPlaces": L("Tolerance decimal places (DIMTDEC)", "Nachkommastellen der Toleranz (DIMTDEC)"),
    "alternateUnitsFormat": L("Alternate unit format (DIMALTU)", "Format der Alternativeinheiten (DIMALTU)"),
    "alternateToleranceDecimalPlaces": L("Alternate tolerance decimal places (DIMALTTD)", "Nachkommastellen der alternativen Toleranz (DIMALTTD)"),
    "angularUnitFormat": L("Angular unit format (DIMAUNIT)", "Winkeleinheitenformat (DIMAUNIT)"),
    "fractionalFormat": L("Fraction format (DIMFRAC)", "Bruchformat (DIMFRAC)"),
    "linearUnitFormat": L("Linear unit format (DIMLUNIT)", "Längeneinheitenformat (DIMLUNIT)"),
    "decimalSeparator": L("Decimal separator (DIMDSEP)", "Dezimaltrennzeichen (DIMDSEP)"),
    "textMovement": L("Text movement (DIMTMOVE)", "Textverschiebung (DIMTMOVE)"),
    "justification": L("Horizontal text justification (DIMJUST)", "Horizontale Textausrichtung (DIMJUST)"),
    "suppressDimension1": L("Suppress first dimension line (DIMSD1)", "Erste Maßlinie unterdrücken (DIMSD1)"),
    "suppressDimension2": L("Suppress second dimension line (DIMSD2)", "Zweite Maßlinie unterdrücken (DIMSD2)"),
    "toleranceJustification": L("Tolerance vertical justification (DIMTOLJ)", "Vertikale Toleranzausrichtung (DIMTOLJ)"),
    "toleranceZeroSuppression": L("Tolerance zero suppression (DIMTZIN)", "Nullenunterdrückung bei Toleranzen (DIMTZIN)"),
    "alternateZeroSuppression": L("Alternate unit zero suppression (DIMALTZ)", "Nullenunterdrückung der Alternativeinheiten (DIMALTZ)"),
    "alternateToleranceZeroSuppression": L("Alternate tolerance zero suppression (DIMALTTZ)", "Nullenunterdrückung der alternativen Toleranz (DIMALTTZ)"),
    "userPositionedText": L("User-positioned text (DIMUPT)", "Benutzerpositionierter Text (DIMUPT)"),
    "fit": L("Text and arrow fit (DIMATFIT)", "Anpassung von Text und Pfeilen (DIMATFIT)"),
    "fixedExtensionEnabled": L("Fixed-length extension lines (DIMFXLON)", "Maßhilfslinien mit fester Länge (DIMFXLON)"),
    "textDirection": L("Text direction (DIMTXTDIRECTION)", "Textrichtung (DIMTXTDIRECTION)"),
    "alternateMeasurementScale": L("Alternate sub-unit factor (DIMALTMZF)", "Faktor der alternativen Untereinheit (DIMALTMZF)"),
    "measurementScale": L("Sub-unit factor (DIMMZF)", "Untereinheitenfaktor (DIMMZF)"),
    "dimensionLineWeight": L("Dimension line weight (DIMLWD)", "Maßlinienstärke (DIMLWD)"),
    "extensionLineWeight": L("Extension line weight (DIMLWE)", "Maßhilfslinienstärke (DIMLWE)"),
    "policy": L("Drawing settings", "Zeichnungseinstellungen"),
    "textStackAlignment": L("Stacked text alignment (TSTACKALIGN)", "Ausrichtung gestapelter Texte (TSTACKALIGN)"),
    "textStackSize": L("Stacked text size (TSTACKSIZE)", "Größe gestapelter Texte (TSTACKSIZE)"),
    "currentEntityLineweight": L("Current lineweight (CELWEIGHT)", "Aktuelle Linienstärke (CELWEIGHT)"),
    "endCaps": L("Lineweight end caps (ENDCAPS)", "Linienenden (ENDCAPS)"),
    "joinStyle": L("Lineweight joint style (JOINSTYLE)", "Linienverbindungsstil (JOINSTYLE)"),
    "lineweightDisplay": L("Display lineweights (LWDISPLAY)", "Linienstärken anzeigen (LWDISPLAY)"),
    "externalReferenceEditing": L("In-place xref editing (XEDIT)", "Xref-Bearbeitung vor Ort (XEDIT)"),
    "extendedNames": L("Extended symbol names (EXTNAMES)", "Erweiterte Symbolnamen (EXTNAMES)"),
    "plotStyleMode": L("Color-dependent plot styles (PSTYLEMODE)", "Farbabhängige Plotstile (PSTYLEMODE)"),
    "oleStartup": L("Load OLE source application (OLESTARTUP)", "OLE-Quellanwendung laden (OLESTARTUP)"),
    "insertionUnits": L("Insertion units (INSUNITS)", "Einfügeeinheiten (INSUNITS)"),
    "currentPlotStyleType": L("Current plot style type (CEPSNTYPE)", "Aktueller Plotstiltyp (CEPSNTYPE)"),
    "sortEntities": L("Object sorting (SORTENTS)", "Objektsortierung (SORTENTS)"),
    "indexControl": L("Layer and spatial index (INDEXCTL)", "Layer- und räumlicher Index (INDEXCTL)"),
    "hideText": L("Hide text (HIDETEXT)", "Text verdecken (HIDETEXT)"),
    "xclipFrame": L("Xref clip frame (XCLIPFRAME)", "Xref-Zuschneiderahmen (XCLIPFRAME)"),
    "dimensionAssociation": L("Dimension associativity (DIMASSOC)", "Bemaßungsassoziativität (DIMASSOC)"),
    "haloGap": L("Halo gap (HALOGAP)", "Halo-Abstand (HALOGAP)"),
    "obscuredColor": L("Obscured line color (OBSCUREDCOLOR)", "Farbe verdeckter Linien (OBSCUREDCOLOR)"),
    "intersectionColor": L("Intersection polyline color (INTERSECTIONCOLOR)", "Farbe der Schnittlinien (INTERSECTIONCOLOR)"),
    "obscuredLinetype": L("Obscured linetype (OBSCUREDLTYPE)", "Linientyp verdeckter Linien (OBSCUREDLTYPE)"),
    "intersectionDisplay": L("Display intersection polylines (INTERSECTIONDISPLAY)", "Schnittlinien anzeigen (INTERSECTIONDISPLAY)"),
    "cameraDisplay": L("Display cameras (CAMERADISPLAY)", "Kameras anzeigen (CAMERADISPLAY)"),
    "stepsPerSecond": L("Walk steps per second (STEPSPERSEC)", "Schritte pro Sekunde (STEPSPERSEC)"),
    "stepSize": L("Walk step size (STEPSIZE)", "Schrittweite (STEPSIZE)"),
    "dwf3dPrecision": L("3D DWF precision (3DDWFPREC)", "3D-DWF-Genauigkeit (3DDWFPREC)"),
    "lensLength": L("Lens length (LENSLENGTH)", "Brennweite (LENSLENGTH)"),
    "cameraHeight": L("Camera height (CAMERAHEIGHT)", "Kamerahöhe (CAMERAHEIGHT)"),
    "solidHistory": L("Solid history (SOLIDHIST)", "Volumenkörperprotokoll (SOLIDHIST)"),
    "showHistory": L("Show history (SHOWHIST)", "Protokoll anzeigen (SHOWHIST)"),
    "polysolidWidth": L("Polysolid width (PSOLWIDTH)", "Polykörperbreite (PSOLWIDTH)"),
    "polysolidHeight": L("Polysolid height (PSOLHEIGHT)", "Polykörperhöhe (PSOLHEIGHT)"),
    "loftAngle1": L("Loft start angle (LOFTANG1)", "Anfangswinkel der Erhebung (LOFTANG1)"),
    "loftAngle2": L("Loft end angle (LOFTANG2)", "Endwinkel der Erhebung (LOFTANG2)"),
    "loftMagnitude1": L("Loft start magnitude (LOFTMAG1)", "Anfangsbetrag der Erhebung (LOFTMAG1)"),
    "loftMagnitude2": L("Loft end magnitude (LOFTMAG2)", "Endbetrag der Erhebung (LOFTMAG2)"),
    "loftParameter": L("Loft options (LOFTPARAM)", "Erhebungsoptionen (LOFTPARAM)"),
    "loftNormals": L("Loft normals (LOFTNORMALS)", "Erhebungsnormalen (LOFTNORMALS)"),
    "latitude": L("Latitude (LATITUDE)", "Geographische Breite (LATITUDE)"),
    "longitude": L("Longitude (LONGITUDE)", "Geographische Länge (LONGITUDE)"),
    "northDirection": L("North direction (NORTHDIRECTION)", "Nordrichtung (NORTHDIRECTION)"),
    "timezone": L("Time zone (TIMEZONE)", "Zeitzone (TIMEZONE)"),
    "lightGlyphDisplay": L("Display light glyphs (LIGHTGLYPHDISPLAY)", "Lichtsymbole anzeigen (LIGHTGLYPHDISPLAY)"),
    "tileModeLightSync": L("Synchronize model lighting (TILEMODELIGHTSYNCH)", "Beleuchtung im Modell synchronisieren (TILEMODELIGHTSYNCH)"),
    "dwfFrame": L("DWF underlay frame (DWFFRAME)", "DWF-Unterlagenrahmen (DWFFRAME)"),
    "dgnFrame": L("DGN underlay frame (DGNFRAME)", "DGN-Unterlagenrahmen (DGNFRAME)"),
    "realWorldScale": L("Real-world scale (REALWORLDSCALE)", "Realer Maßstab (REALWORLDSCALE)"),
    "interfereColorIndex": L("Interference color (INTERFERECOLOR)", "Kollisionsfarbe (INTERFERECOLOR)"),
    "shadowMode": L("Shadow display (CSHADOW)", "Schattenanzeige (CSHADOW)"),
    "shadowPlaneLocation": L("Ground shadow plane (SHADOWPLANELOCATION)", "Höhe der Bodenschattenebene (SHADOWPLANELOCATION)"),
    "strings": L("Text variables", "Textvariablen"),
    "menu": L("Menu file (MENU)", "Menüdatei (MENU)"),
    "dimensionPostfix": L("Dimension text suffix (DIMPOST)", "Maßtext-Suffix (DIMPOST)"),
    "dimensionAlternatePostfix": L("Alternate dimension suffix (DIMAPOST)", "Suffix der Alternativeinheiten (DIMAPOST)"),
    "dimensionAlternateMeasurementZeroSuffix": L("Alternate sub-unit suffix (DIMALTMZS)", "Suffix der alternativen Untereinheit (DIMALTMZS)"),
    "dimensionMeasurementZeroSuffix": L("Sub-unit suffix (DIMMZS)", "Untereinheitensuffix (DIMMZS)"),
    "hyperlinkBase": L("Hyperlink base (HYPERLINKBASE)", "Hyperlinkbasis (HYPERLINKBASE)"),
    "stylesheet": L("Plot style table (STYLESHEET)", "Plotstiltabelle (STYLESHEET)"),
    "fingerprintGuid": L("Fingerprint GUID (FINGERPRINTGUID)", "Fingerabdruck-GUID (FINGERPRINTGUID)"),
    "versionGuid": L("Version GUID (VERSIONGUID)", "Versions-GUID (VERSIONGUID)"),
    "projectName": L("Project name (PROJECTNAME)", "Projektname (PROJECTNAME)"),
    "relations": L("Object references", "Objektreferenzen"),
    "handleSeed": L("Next handle (HANDSEED)", "Nächste Referenz (HANDSEED)"),
    "currentLayer": L("Current layer (CLAYER)", "Aktueller Layer (CLAYER)"),
    "textStyle": L("Current text style (TEXTSTYLE)", "Aktueller Textstil (TEXTSTYLE)"),
    "currentLinetype": L("Current linetype (CELTYPE)", "Aktueller Linientyp (CELTYPE)"),
    "currentMaterial": L("Current material (CMATERIAL)", "Aktuelles Material (CMATERIAL)"),
    "dimensionStyle": L("Current dimension style (DIMSTYLE)", "Aktueller Bemaßungsstil (DIMSTYLE)"),
    "multilineStyle": L("Current multiline style (CMLSTYLE)", "Aktueller Multilinienstil (CMLSTYLE)"),
    "paperUcsName": L("Paper space UCS (PUCSNAME)", "BKS im Papierbereich (PUCSNAME)"),
    "paperUcsOrthographicReference": L("Paper space orthographic UCS reference (PUCSORTHOREF)", "Orthogonale BKS-Referenz im Papierbereich (PUCSORTHOREF)"),
    "paperUcsBase": L("Paper space UCS base (PUCSBASE)", "BKS-Basis im Papierbereich (PUCSBASE)"),
    "modelUcsName": L("Model space UCS (UCSNAME)", "BKS im Modellbereich (UCSNAME)"),
    "modelUcsOrthographicReference": L("Model space orthographic UCS reference (UCSORTHOREF)", "Orthogonale BKS-Referenz im Modellbereich (UCSORTHOREF)"),
    "modelUcsBase": L("Model space UCS base (UCSBASE)", "BKS-Basis im Modellbereich (UCSBASE)"),
    "dimensionTextStyle": L("Dimension text style (DIMTXSTY)", "Bemaßungstextstil (DIMTXSTY)"),
    "dimensionLeaderBlock": L("Leader arrow block (DIMLDRBLK)", "Führungspfeilblock (DIMLDRBLK)"),
    "dimensionBlock": L("Arrow block (DIMBLK)", "Pfeilblock (DIMBLK)"),
    "dimensionBlock1": L("First arrow block (DIMBLK1)", "Erster Pfeilblock (DIMBLK1)"),
    "dimensionBlock2": L("Second arrow block (DIMBLK2)", "Zweiter Pfeilblock (DIMBLK2)"),
    "dimensionLinetype": L("Dimension line linetype (DIMLTYPE)", "Linientyp der Maßlinie (DIMLTYPE)"),
    "dimensionExtensionLinetype1": L("First extension line linetype (DIMLTEX1)", "Linientyp der ersten Maßhilfslinie (DIMLTEX1)"),
    "dimensionExtensionLinetype2": L("Second extension line linetype (DIMLTEX2)", "Linientyp der zweiten Maßhilfslinie (DIMLTEX2)"),
    "blockControl": L("Block table", "Blocktabelle"),
    "layerControl": L("Layer table", "Layertabelle"),
    "styleControl": L("Text style table", "Textstiltabelle"),
    "linetypeControl": L("Linetype table", "Linientyptabelle"),
    "viewControl": L("View table", "Ansichtstabelle"),
    "ucsControl": L("UCS table", "BKS-Tabelle"),
    "viewportControl": L("Viewport table", "Ansichtsfenstertabelle"),
    "appidControl": L("Registered application table", "Tabelle registrierter Anwendungen"),
    "dimensionStyleControl": L("Dimension style table", "Bemaßungsstiltabelle"),
    "groupDictionary": L("Group dictionary", "Gruppenwörterbuch"),
    "multilineStyleDictionary": L("Multiline style dictionary", "Multilinienstil-Wörterbuch"),
    "namedObjectsDictionary": L("Named object dictionary", "Wörterbuch benannter Objekte"),
    "layoutDictionary": L("Layout dictionary", "Layout-Wörterbuch"),
    "plotSettingsDictionary": L("Plot settings dictionary", "Ploteinstellungs-Wörterbuch"),
    "plotStyleNameDictionary": L("Plot style name dictionary", "Plotstilnamen-Wörterbuch"),
    "materialDictionary": L("Material dictionary", "Materialwörterbuch"),
    "colorDictionary": L("Color dictionary", "Farbwörterbuch"),
    "visualStyleDictionary": L("Visual style dictionary", "Wörterbuch der visuellen Stile"),
    "paperSpaceBlockRecord": L("Paper space block record", "Blockeintrag Papierbereich"),
    "modelSpaceBlockRecord": L("Model space block record", "Blockeintrag Modellbereich"),
    "byLayerLinetype": L("ByLayer linetype", "Linientyp VonLayer"),
    "byBlockLinetype": L("ByBlock linetype", "Linientyp VonBlock"),
    "continuousLinetype": L("Continuous linetype", "Linientyp Continuous"),
    "interfereObjectVisualStyle": L("Interference object visual style (INTERFEREOBJVS)", "Visueller Stil der Kollisionsobjekte (INTERFEREOBJVS)"),
    "interfereViewportVisualStyle": L("Interference viewport visual style (INTERFEREVPVS)", "Visueller Stil des Kollisions-Ansichtsfensters (INTERFEREVPVS)"),
    "dragVisualStyle": L("Drag visual style (DRAGVS)", "Visueller Stil beim Ziehen (DRAGVS)"),
    "DwgClass.number": L("Class number", "Klassennummer"),
    "proxyFlags": L("Proxy capability flags", "Proxy-Fähigkeitsflags"),
    "applicationName": L("Application name", "Anwendungsname"),
    "cppClassName": L("C++ class name", "C++-Klassenname"),
    "dxfName": L("DXF name", "DXF-Name"),
    "wasZombie": L("Was a proxy", "War ein Proxy"),
    "itemClassId": L("Item class id", "Elementklassen-ID", ("0x1F2 for entities, 0x1F3 for objects.", "0x1F2 für grafische Objekte, 0x1F3 für nichtgrafische Objekte.")),
    "objectCount": L("Object count", "Objektanzahl"),
    "dwgVersion": L("DWG version", "DWG-Version"),
    "reservedValues": L("Reserved values", "Reservierte Werte"),
    "feature": L("Feature", "Funktion", ("e.g. Acad:XRef or Acad:Text.", "z. B. Acad:XRef oder Acad:Text.")),
    "fullPath": L("Full path", "Vollständiger Pfad"),
    "relativePath": L("Found path", "Gefundener Pfad"),
    "fingerprint": L("Fingerprint GUID", "Fingerabdruck-GUID"),
    "DwgDependency.version": L("Version GUID", "Versions-GUID"),
    "timestamp": L("Timestamp", "Zeitstempel"),
    "fileSize": L("File size", "Dateigröße", unit="B"),
    "affectsGraphics": L("Affects graphics", "Beeinflusst die Grafik"),
    "referenceCount": L("Reference count", "Referenzanzahl"),
    "subject": L("Subject", "Betreff"),
    "keywords": L("Keywords", "Stichwörter"),
    "comments": L("Comments", "Kommentare"),
    "lastSavedBy": L("Last saved by", "Zuletzt gespeichert von"),
    "revisionNumber": L("Revision number", "Revisionsnummer"),
    "totalEditingTime": L("Total editing time", "Gesamtbearbeitungszeit"),
    "DwgSummaryInfo.createdAt": L("Created", "Erstellt"),
    "modifiedAt": L("Modified", "Geändert"),
    "customProperties": L("Custom properties", "Benutzerdefinierte Eigenschaften"),
    "versionChecksum": L("Version checksum", "Versionsprüfsumme"),
    "commentChecksum": L("Comment checksum", "Kommentarprüfsumme"),
    "productChecksum": L("Product checksum", "Produktprüfsumme"),
    "applicationVersion": L("Application version", "Anwendungsversion"),
    "measurement": L("Measurement system (MEASUREMENT)", "Maßsystem (MEASUREMENT)"),
    "totalSaves": L("Save count", "Anzahl der Speichervorgänge"),
    "savePartitionOne": L("Save counter, low word", "Speicherzähler, niederwertiges Wort"),
    "savePartitionTwo": L("Save counter, high word", "Speicherzähler, höherwertiges Wort"),
    "saveGeneration": L("Save generation", "Speichergeneration"),
    "legacyStampOne": L("Legacy version stamp 1", "Alter Versionsstempel 1"),
    "legacyStampTwo": L("Legacy version stamp 2", "Alter Versionsstempel 2"),
    "compatibilityProfile": L("Compatibility profile", "Kompatibilitätsprofil"),
    "DwgAuxiliaryHeader.createdAt": L("Created", "Erstellt"),
    "DwgAuxiliaryHeader.updatedAt": L("Last updated", "Zuletzt aktualisiert"),
    "terminalSaveGeneration": L("Final save generation", "Letzte Speichergeneration"),
    "DwgVersionStamp.version": L("Version", "Version"),
    "maintenance": L("Maintenance release", "Wartungsversion"),
    "formatMajor": L("Format major version", "Hauptversion des Formats"),
    "formatMinor": L("Format minor version", "Nebenversion des Formats"),
    "revisions": L("Revisions", "Revisionen"),
    "DwgIndexedPreview.width": L("Width", "Breite", unit="px"),
    "DwgIndexedPreview.height": L("Height", "Höhe", unit="px"),
    "origin": L("Row origin", "Zeilenursprung"),
    "palette": L("Palette", "Palette"),
    "red": L("Red", "Rot"),
    "green": L("Green", "Grün"),
    "blue": L("Blue", "Blau"),
    "alpha": L("Alpha", "Alpha"),
    "pixelIndices": L("Pixel palette indices", "Pixel-Palettenindizes"),
    "backgroundPaletteIndex": L("Background palette index", "Hintergrund-Palettenindex"),
    "historyIdentifierOne": L("History identifier 1", "Verlaufskennung 1"),
    "historyIdentifierTwo": L("History identifier 2", "Verlaufskennung 2"),
    "classVersion": L("Class version", "Klassenversion"),
    "applicationVersionDigest": L("Application version digest", "Prüfsumme der Anwendungsversion"),
    "trustCommentDigest": L("Trust comment digest", "Prüfsumme des Vertrauenskommentars"),
    "trustComment": L("Trust comment", "Vertrauenskommentar"),
    "propertySetDigest": L("Property set digest", "Prüfsumme des Eigenschaftssatzes"),
    "propertyFormatIdentifier": L("Property format identifier", "Kennung des Eigenschaftsformats"),
    "productDigest": L("Product digest", "Produktprüfsumme"),
    "buildVersion": L("Build version", "Build-Version"),
    "registryVersion": L("Registry version", "Registrierungsversion"),
    "installId": L("Installation id", "Installations-ID", widget="text"),
    "localeId": L("Locale id", "Gebietsschema-ID", widget="text"),
}
DWG_OPTIONS = {
    "category": {"entity": D("Entity", "Grafisches Objekt"), "tableControl": D("Table control", "Tabellensteuerobjekt"), "tableRecord": D("Table record", "Tabelleneintrag"), "dictionary": D("Dictionary", "Wörterbuch"), "object": D("Non-graphical object", "Nichtgrafisches Objekt"), "custom": D("Custom class", "Benutzerdefinierte Klasse")},
    "measurement": {"english": D("Imperial", "Britisch (Zoll)"), "metric": D("Metric", "Metrisch")},
    "compatibilityProfile": {"autocad2009": D("AutoCAD 2009", "AutoCAD 2009")},
    "origin": {"bottomUp": D("Bottom-up", "Von unten nach oben")},
    "DwgApplicationProperty.kind": {"string": D("Text", "Text"), "dateTime": D("Date and time", "Datum und Uhrzeit")},
}
#endregion 🖊️Dwg

#region 🧊️Gltf
def IDX(en, de, what_en, what_de):
    return L(en, de, ("Index of the %s, counted from 0." % what_en, "Index %s, ab 0 gezählt." % what_de))


GLTF = {
    "position": L("Insert position", "Einfügeposition", ("Target index in the list, counted from 0.", "Zielindex in der Liste, ab 0 gezählt.")),
    "order": L("New order", "Neue Reihenfolge", ("The current indices, listed in their new order.", "Die bisherigen Indizes in ihrer neuen Reihenfolge.")),
    "required-extension/reorder:order": L("New order", "Neue Reihenfolge", ("The required extension names in their new order.", "Die erforderlichen Erweiterungsnamen in neuer Reihenfolge.")),
    "used-extension/reorder:order": L("New order", "Neue Reihenfolge", ("The used extension names in their new order.", "Die verwendeten Erweiterungsnamen in neuer Reihenfolge.")),
    "delete:index": IDX("Index", "Index", "item to delete", "des zu löschenden Eintrags"),
    "move:index": IDX("Index", "Index", "item to move", "des zu verschiebenden Eintrags"),
    "node": IDX("Node", "Knoten", "node", "des Knotens"),
    "mesh": IDX("Mesh", "Netz", "mesh", "des Netzes"),
    "scene": IDX("Scene", "Szene", "scene", "der Szene"),
    "GltfDiff.scene": L("Default scene", "Standardszene"),
    "^document.scene": L("Default scene", "Standardszene"),
    "camera": IDX("Camera", "Kamera", "camera", "der Kamera"),
    "skin": IDX("Skin", "Skin", "skin", "des Skins"),
    "material": IDX("Material", "Material", "material", "des Materials"),
    "accessor": IDX("Accessor", "Accessor", "accessor", "des Accessors"),
    "primitive": L("Primitive", "Primitiv", ("Index of the primitive within the mesh, counted from 0.", "Index des Primitivs im Netz, ab 0 gezählt.")),
    "target": L("Morph target", "Morph-Ziel", ("Index of the morph target within the primitive, counted from 0.", "Index des Morph-Ziels im Primitiv, ab 0 gezählt.")),
    "GltfBufferView.target": L("Binding target", "Bindungsziel", ("34962 vertex attributes (ARRAY_BUFFER), 34963 indices (ELEMENT_ARRAY_BUFFER).", "34962 Vertexattribute (ARRAY_BUFFER), 34963 Indizes (ELEMENT_ARRAY_BUFFER).")),
    "^bufferViews.target": L("Binding target", "Bindungsziel", ("34962 vertex attributes (ARRAY_BUFFER), 34963 indices (ELEMENT_ARRAY_BUFFER).", "34962 Vertexattribute (ARRAY_BUFFER), 34963 Indizes (ELEMENT_ARRAY_BUFFER).")),
    "GltfAnimationChannel.target": L("Animated property", "Animierte Eigenschaft"),
    "parent": IDX("Parent node", "Elternknoten", "parent node", "des Elternknotens"),
    "child": IDX("Child node", "Kindknoten", "child node", "des Kindknotens"),
    "semantic": L("Attribute semantic", "Attributsemantik", ("e.g. POSITION, NORMAL or TEXCOORD_0.", "z. B. POSITION, NORMAL oder TEXCOORD_0.")),
    "extension": L("Extension", "Erweiterung", ("Extension name, e.g. KHR_materials_unlit.", "Name der Erweiterung, z. B. KHR_materials_unlit.")),
    "weights": L("Morph weights", "Morph-Gewichte"),
    "change-extras:data": L("Extras", "Zusatzdaten (extras)", ("Application-specific JSON; empty removes it.", "Anwendungsspezifisches JSON; leer entfernt es.")),
    "change-extensions:data": L("Extension data", "Erweiterungsdaten", ("JSON object keyed by extension name; empty removes it.", "JSON-Objekt mit Erweiterungsnamen als Schlüssel; leer entfernt es.")),
    "transform": L("Transform", "Transformation", ("Either a 4×4 matrix or translation, rotation and scale.", "Entweder eine 4×4-Matrix oder Verschiebung, Drehung und Skalierung.")),
    "scene/rename:value": L("Name", "Name", ("Empty removes the name.", "Leer entfernt den Namen.")),
    "mesh/rename:value": L("Name", "Name", ("Empty removes the name.", "Leer entfernt den Namen.")),
    "GltfChangeNodeNamePayload.value": L("Name", "Name", ("Empty removes the name.", "Leer entfernt den Namen.")),
    "GltfChangeNodeNameRestore.before": L("Name before", "Name vorher"),
    "GltfChangeNodeNameRestore.after": L("Name after", "Name nachher"),
    "code": L("Rejection code", "Ablehnungscode"),
    "change-alpha:path": L("Rejected path", "Abgelehnter Pfad"),
    "change-sides:path": L("Rejected path", "Abgelehnter Pfad"),
    "detail": L("Rejection detail", "Ablehnungsdetail"),
    "generator": L("Generator", "Generator", ("Tool that produced the asset.", "Werkzeug, das das Asset erzeugt hat.")),
    "copyright": L("Copyright", "Urheberrechtsvermerk"),
    "minVersion": L("Minimum version", "Mindestversion", ("Minimum glTF version a loader must support.", "glTF-Mindestversion, die ein Lader unterstützen muss.")),
    "asset/version:version": L("glTF version", "glTF-Version", ("e.g. 2.0.", "z. B. 2.0.")),
    "bytes": L("Buffer data", "Pufferdaten"),
    "buffer": IDX("Buffer", "Puffer", "buffer", "des Puffers"),
    "byteOffset": L("Byte offset", "Byte-Versatz", unit="B"),
    "byteLength": L("Byte length", "Byte-Länge", unit="B"),
    "byteStride": L("Byte stride", "Byte-Schrittweite", unit="B"),
    "componentType": L("Component type", "Komponententyp"),
    "count": L("Element count", "Elementanzahl"),
    "GltfSparseAccessor.count": L("Sparse element count", "Anzahl der Sparse-Elemente"),
    "kind": L("Element type", "Elementtyp"),
    "type": L("Element type", "Elementtyp"),
    "type#perspective": L("Projection type", "Projektionsart"),
    "normalized": L("Normalized", "Normalisiert"),
    "max": L("Maximum", "Maximum", ("Per-component maximum.", "Maximum je Komponente.")),
    "min": L("Minimum", "Minimum", ("Per-component minimum.", "Minimum je Komponente.")),
    "sparse": L("Sparse storage", "Sparse-Speicherung"),
    "indices": L("Index accessor", "Index-Accessor"),
    "GltfSparseAccessor.indices": L("Sparse indices", "Sparse-Indizes"),
    "GltfSparseAccessor.values": L("Sparse values", "Sparse-Werte"),
    "nodes": L("Nodes", "Knoten"),
    "GltfScene.nodes": L("Root nodes", "Wurzelknoten"),
    "GltfSceneDiff.nodes": L("Root nodes", "Wurzelknoten"),
    "^scenes.nodes": L("Root nodes", "Wurzelknoten"),
    "samplers": L("Texture samplers", "Textur-Sampler"),
    "GltfAnimation.samplers": L("Animation samplers", "Animations-Sampler"),
    "^animations.samplers": L("Animation samplers", "Animations-Sampler"),
    "sampler": L("Sampler", "Sampler"),
    "GltfAnimationChannel.sampler": L("Animation sampler", "Animations-Sampler"),
    "scale": L("Scale", "Skalierung"),
    "GltfNormalTextureInfo.scale": L("Normal scale", "Normalen-Skalierung"),
    "rotation": L("Rotation", "Drehung", ("Unit quaternion (x, y, z, w).", "Einheitsquaternion (x, y, z, w).")),
    "translation": L("Translation", "Verschiebung"),
    "matrix": L("Matrix", "Matrix", ("4×4 transform, column-major.", "4×4-Transformationsmatrix, spaltenweise.")),
    "source": IDX("Image", "Bild", "image", "des Bildes"),
    "input": L("Keyframe times", "Keyframe-Zeiten", ("Accessor with the keyframe input times.", "Accessor mit den Keyframe-Zeiten.")),
    "output": L("Keyframe values", "Keyframe-Werte", ("Accessor with the keyframe output values.", "Accessor mit den Keyframe-Werten.")),
    "document": L("glTF document", "glTF-Dokument"),
    "GltfSnapshot.buffers": L("Buffer bytes", "Pufferbytes"),
    "sourceForm": L("Source form", "Quellformat"),
    "bufferBytes": L("Buffer bytes", "Pufferbytes"),
    "children": L("Children", "Kindknoten"),
    "primitives": L("Primitives", "Primitive"),
    "attributes": L("Vertex attributes", "Vertexattribute", ("Attribute semantic to accessor index.", "Attributsemantik zu Accessor-Index.")),
    "targets": L("Morph targets", "Morph-Ziele"),
    "mode": L("Topology mode", "Topologiemodus", ("0 points, 1 lines, 2 line loop, 3 line strip, 4 triangles, 5 triangle strip, 6 triangle fan.", "0 Punkte, 1 Linien, 2 Linienschleife, 3 Linienzug, 4 Dreiecke, 5 Dreiecksstreifen, 6 Dreiecksfächer.")),
    "bufferView": L("Buffer view", "Pufferansicht"),
    "uri": L("URI", "URI"),
    "mimeType": L("Media type", "Medientyp"),
    "magFilter": L("Magnification filter", "Vergrößerungsfilter", ("9728 nearest, 9729 linear.", "9728 nächster Nachbar, 9729 linear.")),
    "minFilter": L("Minification filter", "Verkleinerungsfilter"),
    "wrapS": L("Wrap mode S", "Wiederholungsmodus S", ("33071 clamp to edge, 33648 mirrored repeat, 10497 repeat.", "33071 am Rand fixieren, 33648 gespiegelt wiederholen, 10497 wiederholen.")),
    "wrapT": L("Wrap mode T", "Wiederholungsmodus T", ("33071 clamp to edge, 33648 mirrored repeat, 10497 repeat.", "33071 am Rand fixieren, 33648 gespiegelt wiederholen, 10497 wiederholen.")),
    "inverseBindMatrices": L("Inverse bind matrices", "Inverse Bindungsmatrizen"),
    "skeleton": L("Skeleton root", "Skelettwurzel"),
    "joints": L("Joints", "Gelenke"),
    "channels": L("Channels", "Kanäle"),
    "interpolation": L("Interpolation", "Interpolation"),
    "GltfAnimationChannelTarget.path": L("Animated property", "Animierte Eigenschaft"),
    "pbrMetallicRoughness": L("PBR metallic-roughness", "PBR Metallic-Roughness"),
    "baseColorFactor": L("Base color factor", "Grundfarbfaktor", ("Linear RGBA multipliers.", "Lineare RGBA-Faktoren.")),
    "baseColorTexture": L("Base color texture", "Grundfarbtextur"),
    "metallicFactor": L("Metallic factor", "Metallizitätsfaktor"),
    "roughnessFactor": L("Roughness factor", "Rauheitsfaktor"),
    "metallicRoughnessTexture": L("Metallic-roughness texture", "Metallizitäts-Rauheits-Textur"),
    "normalTexture": L("Normal texture", "Normalentextur"),
    "occlusionTexture": L("Occlusion texture", "Okklusionstextur"),
    "strength": L("Occlusion strength", "Okklusionsstärke"),
    "emissiveTexture": L("Emissive texture", "Emissionstextur"),
    "emissiveFactor": L("Emissive factor", "Emissionsfaktor"),
    "alphaMode": L("Alpha mode", "Alphamodus"),
    "alphaCutoff": L("Alpha cutoff", "Alpha-Schwellenwert"),
    "doubleSided": L("Double-sided", "Doppelseitig"),
    "texCoord": L("Texture coordinate set", "Texturkoordinatensatz", ("n of TEXCOORD_n.", "n von TEXCOORD_n.")),
    "GltfTextureInfo.index": IDX("Texture", "Textur", "texture", "der Textur"),
    "GltfNormalTextureInfo.index": IDX("Texture", "Textur", "texture", "der Textur"),
    "GltfOcclusionTextureInfo.index": IDX("Texture", "Textur", "texture", "der Textur"),
    "perspective": L("Perspective projection", "Perspektivische Projektion"),
    "orthographic": L("Orthographic projection", "Orthografische Projektion"),
    "projection": L("Projection", "Projektion"),
    "extensions": L("Extensions", "Erweiterungen"),
    "extras": L("Extras", "Zusatzdaten (extras)"),
    "extensionsUsed": L("Extensions used", "Verwendete Erweiterungen"),
    "extensionsRequired": L("Extensions required", "Erforderliche Erweiterungen"),
    "scenes": L("Scenes", "Szenen"),
    "meshes": L("Meshes", "Netze"),
    "accessors": L("Accessors", "Accessoren"),
    "bufferViews": L("Buffer views", "Pufferansichten"),
    "buffers": L("Buffers", "Puffer"),
    "materials": L("Materials", "Materialien"),
    "textures": L("Textures", "Texturen"),
    "images": L("Images", "Bilder"),
    "skins": L("Skins", "Skins"),
    "animations": L("Animations", "Animationen"),
    "cameras": L("Cameras", "Kameras"),
    "snapshot": L("glTF asset", "glTF-Asset"),
}
GLTF_OPTIONS = {"*": {
    "OPAQUE": D("Opaque", "Deckend"), "MASK": D("Mask", "Maskiert"), "BLEND": D("Blend", "Überblendet"),
    "Byte": D("Byte (signed 8-bit)", "Byte (8 Bit, vorzeichenbehaftet)"), "UnsignedByte": D("Unsigned byte (8-bit)", "Byte (8 Bit, vorzeichenlos)"), "Short": D("Short (signed 16-bit)", "Short (16 Bit, vorzeichenbehaftet)"), "UnsignedShort": D("Unsigned short (16-bit)", "Short (16 Bit, vorzeichenlos)"), "UnsignedInt": D("Unsigned int (32-bit)", "Integer (32 Bit, vorzeichenlos)"), "Float": D("Float (32-bit)", "Gleitkommazahl (32 Bit)"),
    "Scalar": D("Scalar", "Skalar"), "Vec2": D("2D vector", "2D-Vektor"), "Vec3": D("3D vector", "3D-Vektor"), "Vec4": D("4D vector", "4D-Vektor"), "Mat2": D("2×2 matrix", "2×2-Matrix"), "Mat3": D("3×3 matrix", "3×3-Matrix"), "Mat4": D("4×4 matrix", "4×4-Matrix"),
    "SCALAR": D("Scalar", "Skalar"), "VEC2": D("2D vector", "2D-Vektor"), "VEC3": D("3D vector", "3D-Vektor"), "VEC4": D("4D vector", "4D-Vektor"), "MAT2": D("2×2 matrix", "2×2-Matrix"), "MAT3": D("3×3 matrix", "3×3-Matrix"), "MAT4": D("4×4 matrix", "4×4-Matrix"),
    "LINEAR": D("Linear", "Linear"), "STEP": D("Step", "Stufenweise"), "CUBICSPLINE": D("Cubic spline", "Kubischer Spline"),
    "translation": D("Translation", "Verschiebung"), "rotation": D("Rotation", "Drehung"), "scale": D("Scale", "Skalierung"), "weights": D("Morph weights", "Morph-Gewichte"),
    "json": D("glTF JSON", "glTF-JSON"), "glb": D("GLB binary", "GLB-Binärdatei"),
    "perspective": D("Perspective", "Perspektivisch"), "orthographic": D("Orthographic", "Orthografisch"),
}}
#endregion 🧊️Gltf

#region 🛠️Engine
TABLES = {
    "🌐️html": (HTML, {}), "🌦️epw": (EPW, {}), "🎞️gif": (GIF, GIF_OPTIONS), "🏗️ifc": (IFC, IFC_OPTIONS), "💬️bcf": (BCF, {}),
    "💾️binary": (BINARY, {}), "📊️csv": (CSV, {}), "📜️docx": ({**XML, **DOCX}, XML_OPTIONS), "📼️avi": (AVI, {}), "🖋️dxf": (DXF, {}),
    "🗜️deflate": (DEFLATE, DEFLATE_OPTIONS), "🪟️bmp": (BMP, BMP_OPTIONS), "🖊️dwg": (DWG, DWG_OPTIONS), "🧊️gltf": (GLTF, GLTF_OPTIONS),
}
ADDRESSING = {"index", "id", "guid", "topic_guid", "stream_index", "record_index", "field_index", "recordIndex", "fieldIndex", "address", "parent", "path", "node", "mesh", "primitive", "scene", "child", "target", "building", "storey", "product", "project", "offset"}


def ascii_kind(segment):
    for at, character in enumerate(segment):
        if character.isascii() and character.isalnum():
            return segment[at:]
    return segment


def leaf_kind(file):
    inner = file.split("/🧬️mutations/", 1)[1].rsplit("/🧬️schema/", 1)[0]
    return "/".join(ascii_kind(segment) for segment in inner.split("/"))


def inferred_reference(key, many):
    suffixes = ("Ids", "_ids") if many else ("Id", "_id")
    suffix = next((candidate for candidate in suffixes if key.endswith(candidate)), None)
    if suffix is None:
        return False
    stem = key[: -len(suffix)]
    if stem.startswith("new"):
        rest = stem[3:]
        if rest == "":
            return False
        if rest[:1].isupper():
            stem = rest
    return re.match(r"^[A-Za-z][A-Za-z0-9_]*$", stem) is not None


class Plan:
    """🗺️ Collects the `x-semio-ui` to write per (file, node path) and refuses two different annotations for one node."""

    def __init__(self, glossary):
        self.glossary = glossary
        self.sites = {}
        self.missing = []
        self.titles = {}

    def put(self, file, path, annotation):
        slot = self.sites.setdefault((file, tuple(path)), {})
        for key, value in annotation.items():
            if key in slot and slot[key] != value:
                raise SystemExit("conflict at %s %s key %s: %r vs %r" % (file, list(path), key, slot[key], value))
            slot[key] = value

    def title(self, walker, file):
        if file not in self.titles:
            self.titles[file] = walker.document(file).get("title")
        return self.titles[file]


def context(walker, plan, file, path):
    defs = [index for index in range(len(path) - 1) if path[index] == "$defs"]
    tail = path[defs[-1] + 2:] if defs else path
    def_name = path[defs[-1] + 1] if defs else None
    chain = [tail[index + 1] for index in range(len(tail) - 1) if tail[index] == "properties"]
    if def_name is None and len(chain) == 1:
        def_name = plan.title(walker, file)
    return def_name, (chain[-2] if len(chain) >= 2 else None)


def lookup(table, keys):
    for key in keys:
        if key in table:
            return dict(table[key])
    return None


def default_widget(record):
    kind = record["type"]
    if kind == "boolean":
        return "toggle"
    if kind in ("integer", "number"):
        return "stepper"
    if kind == "string":
        if record["enum"]:
            return "segmented" if len(record["enum"]) <= 3 else "select"
        return "text"
    return None


def options_for(artifact_options, def_name, key, values):
    labels = {}
    for value in values:
        for table in (artifact_options.get("%s.%s" % (def_name, key)), artifact_options.get(key), artifact_options.get("*")):
            if table is not None and value in table:
                labels[value] = table[value]
                break
    return labels


def plan_leaf(walker, plan, file):
    artifact = file.split("🗿️artifacts/", 1)[1].split("/", 1)[0]
    table, artifact_options = TABLES[artifact]
    kind = leaf_kind(file)
    verb = kind.split("/")[-1]
    try:
        records = walker.leaf(file)
    except LookupError as error:
        print("REFUNRESOLVED", file, error)
        return
    top_order = {}
    for record in records:
        if record.get("recursive"):
            continue
        if record.get("variant"):
            value = record["discriminators"][0][1] if record["discriminators"] else None
            plan.put(*record["branchSite"], {"label": PHASES[value]})
            for _, _, site, _ in record["discriminators"]:
                plan.put(*site, PHASE)
            continue
        key = record["key"]
        site_file, site_path = record["site"][0], tuple(record["site"][1])
        def_name, parent = context(walker, plan, site_file, site_path)
        variant = record["context"].startswith("variant:")
        depth = record["pointer"].count("/")
        in_leaf = site_file == file
        top = in_leaf and ((not variant and depth == 1) or (record["context"] == "variant:apply" and depth == 2))
        if variant and depth == 1 and key == "value":
            annotation = dict(APPLY if record["context"] == "variant:apply" else RESTORE)
        else:
            keys = (["%s:%s" % (kind, key), "%s:%s" % (verb, key)] if top else []) + (["%s.%s" % (def_name, key)] if def_name else []) + (["^%s.%s" % (parent, key)] if parent else []) + (["%s#%s" % (key, record["enum"][0])] if record["enum"] else []) + [key]
            annotation = lookup(table, keys) or lookup(COMMON, [key]) or {}
        if "label" not in annotation and "label" not in record["ui"] and key not in plan.glossary:
            plan.missing.append((artifact, kind, record["pointer"], def_name, parent, key))
        if top:
            widget = annotation.get("widget") or record["ui"].get("widget") or default_widget(record)
            if widget is not None:
                annotation["widget"] = widget
            if record["type"] == "integer" and "precision" not in annotation and widget in ("stepper", "slider", "dial"):
                annotation["precision"] = 0
            group_key = "%s#%s" % (file, record["context"])
            top_order[group_key] = top_order.get(group_key, 0) + 10
            annotation.setdefault("group", "target" if annotation.get("role") == "target" or key in ADDRESSING else "value")
            annotation["order"] = top_order[group_key]
        elif record["type"] == "string" and not record["enum"] and "widget" not in annotation and "role" not in annotation and "widget" not in record["ui"] and "role" not in record["ui"] and inferred_reference(key, False):
            annotation["widget"] = "text"
        if annotation:
            plan.put(site_file, site_path, annotation)
        for values, target in ((record["enum"], record["target"]), (record.get("itemsEnum"), record.get("itemsSite"))):
            if values and all(isinstance(value, str) for value in values):
                labels = options_for(artifact_options, def_name, key, values)
                unlabelled = [value for value in values if value not in labels and value not in plan.glossary]
                if unlabelled:
                    plan.missing.append((artifact, kind, record["pointer"], def_name, parent, "%s options %s" % (key, unlabelled)))
                if labels:
                    plan.put(target[0], tuple(target[1]), {"options": labels})


def ordered(annotation):
    return {key: annotation[key] for key in KEY_ORDER if key in annotation}


def node_at(document, path):
    node = document
    for segment in path:
        node = node[segment]
    return node


def standard(text):
    try:
        return json.dumps(json.loads(text), indent=2, ensure_ascii=False) + "\n" == text
    except ValueError:
        return False


def main():
    dry = "--dry-run" in sys.argv
    shared = "--shared" in sys.argv
    walker = walk.Walker(walk.document_index(), walk.glossary())
    plan = Plan(walk.glossary())
    for file in walk.leaf_paths():
        plan_leaf(walker, plan, file)
    if plan.missing:
        for entry in sorted(set(plan.missing)):
            print("MISSING", *entry)
        raise SystemExit("%d unlabelled inputs" % len(set(plan.missing)))
    by_file = {}
    for (file, path), annotation in plan.sites.items():
        by_file.setdefault(file, []).append((path, annotation))
    manual = []
    written = 0
    for file, entries in sorted(by_file.items()):
        if file in SHARED and not shared:
            print("SKIP shared", file, len(entries))
            continue
        with open(os.path.join(REPO, file), encoding="utf-8") as handle:
            text = handle.read()
        document = json.loads(text)
        changed = False
        for path, annotation in entries:
            node = node_at(document, path)
            merged = ordered({**node.get("x-semio-ui", {}), **annotation})
            if node.get("x-semio-ui") != merged:
                node["x-semio-ui"] = merged
                changed = True
        if not changed:
            continue
        if not standard(text):
            manual.append((file, [(list(path), ordered(annotation)) for path, annotation in entries]))
            continue
        written += 1
        if not dry:
            with open(os.path.join(REPO, file), "w", encoding="utf-8") as handle:
                handle.write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
    for file, entries in manual:
        print("MANUAL", file)
        for path, annotation in entries:
            print("   ", path, json.dumps(annotation, ensure_ascii=False))
    print("sites=%d files=%d written=%d manual=%d%s" % (len(plan.sites), len(by_file), written, len(manual), " (dry run)" if dry else ""))


if __name__ == "__main__":
    main()
#endregion 🛠️Engine
