"""🏷️ Adds bilingual inspector labels and aligns drawing property order with GraphQL and Protobuf."""
import json
from pathlib import Path

SCHEMA = Path("/home/deck/semio/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json")

def ui(en, de, widget=None):
    annotation = {"label": {"en": en, "de": de}}
    if widget:
        annotation["widget"] = widget
    return annotation

document = json.loads(SCHEMA.read_text())
defs = document["$defs"]
camera = defs["ShootingCamera"]["properties"]
camera["fov"]["x-semio-ui"] = ui("Perspective field of view (°)", "Perspektivischer Bildwinkel (°)")
camera["up"]["x-semio-ui"] = ui("Camera up", "Kamera-Hochachse")
camera["projection"]["x-semio-ui"] = ui("Projection", "Projektion")

drawing_labels = {
    "background": ui("Background layer", "Hintergrundebene", "toggle"),
    "fill": ui("Face fills", "Flächenfüllungen", "toggle"),
    "fillColor": ui("Fill color", "Füllfarbe"),
    "fillOpacity": ui("Fill opacity", "Deckkraft der Füllung"),
    "visibleLines": ui("Visible lines", "Sichtbare Linien", "toggle"),
    "hiddenLines": ui("Hidden lines", "Verdeckte Linien", "toggle"),
    "visibleColor": ui("Visible line color", "Farbe sichtbarer Linien"),
    "hiddenColor": ui("Hidden line color", "Farbe verdeckter Linien"),
    "visibleWidth": ui("Visible line width (px)", "Sichtbare Linienbreite (px)"),
    "hiddenWidth": ui("Hidden line width (px)", "Verdeckte Linienbreite (px)"),
    "lineEasing": ui("Corner radius (px)", "Eckenradius (px)"),
    "mergeLines": ui("Merge connected lines", "Verbundene Linien zusammenführen", "toggle"),
}
drawing = defs["ShootingDrawing"]["properties"]
defs["ShootingDrawing"]["properties"] = {key: drawing[key] | {"x-semio-ui": drawing_labels[key]} for key in drawing_labels}

patch = defs["ShootingDrawingPatch"]["properties"]
for key, annotation in drawing_labels.items():
    patch[key]["x-semio-ui"] = annotation
patch["restoreAbsent"]["x-semio-ui"] = ui("Restore absent fields", "Fehlende Felder wiederherstellen", "toggle")
patch["fillColor"]["properties"]["value"]["x-semio-ui"] = ui("Fill color", "Füllfarbe")

shot = defs["ShootingShot"]["properties"]["autoFit"]
shot.pop("description", None)
shot["x-semio-ui"] = ui("Auto fit", "Automatische Einpassung", "toggle")
shot["x-semio-ui"]["description"] = {
    "en": "Absent inherits the scene auto-fit setting.",
    "de": "Ohne Angabe gilt die automatische Einpassung der Szene.",
}

SCHEMA.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
