#!/usr/bin/env python3
"""🏷️ W1-F: writes the three new selection leaf payload schemas and the `x-semio-ui` input annotations
(design §6) onto every puzzle 2d leaf payload schema, including every `$defs` record field."""
import json
import os
import sys

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
DEGREES = 57.29577951308232
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "options", "group", "order")


def L(en, de):
    return {"en": en, "de": de}


def ui(widget, label, role="value", **extra):
    body = {"widget": widget, "role": role, "label": label, **extra}
    return {key: body[key] for key in KEY_ORDER if key in body}


REF = {
    "node": {"kind": "node", "domain": "vortex", "granularity": "node"},
    "edge": {"kind": "edge", "domain": "vortex", "granularity": "edge"},
    "handle": {"kind": "handle", "domain": "vortex", "granularity": "handle"},
    "region": {"kind": "targetRegion", "domain": "vortex", "granularity": "node"},
    "nodes": {"kind": ["node"], "domain": "vortex", "granularity": "node"},
    "movable": {"kind": ["node", "targetRegion"], "domain": "vortex", "granularity": "node"},
}
GRID = {"config": "gridFactor"}
DISCRIMINATOR = {"widget": "hidden", "role": "discriminator", "label": L("Mutation", "Mutation")}


def target(kind, label, description, order=10):
    return ui("reference", label, role="target", description=description, ref=REF[kind], group="target", order=order)


NODE = target("node", L("Node", "Knoten"), L("The node this mutation addresses.", "Der Knoten, den diese Mutation adressiert."))
EDGE = target("edge", L("Edge", "Kante"), L("The edge this mutation addresses.", "Die Kante, die diese Mutation adressiert."))
REGION = target("region", L("Target region", "Zielregion"), L("The target region this mutation addresses.", "Die Zielregion, die diese Mutation adressiert."))
OWNER = target("node", L("Node", "Knoten"), L("The node that owns the handle.", "Der Knoten, dem der Anschluss gehört."))
HANDLE = target("handle", L("Handle", "Anschluss"), L("The handle this mutation addresses.", "Der Anschluss, den diese Mutation adressiert."), order=20)


def position(label, order, group="position", description=None):
    extra = {"description": description} if description else {}
    return ui("stepper", label, step=1, precision=2, snapSource=GRID, group=group, order=order, **extra)


def length(label, order, group="geometry"):
    return ui("stepper", label, step=1, precision=2, snapSource=GRID, group=group, order=order)


def plain(label, order, group="geometry", step=0.1):
    return ui("stepper", label, step=step, precision=2, group=group, order=order)


def toggle(label, order, group="state", description=None):
    extra = {"description": description} if description else {}
    return ui("toggle", label, group=group, order=order, **extra)


def text(label, order, group="appearance", description=None, widget="text"):
    extra = {"description": description} if description else {}
    return ui(widget, label, group=group, order=order, **extra)


def index(order):
    return ui("stepper", L("Index", "Index"), description=L("Position in the list; empty appends at the end.", "Position in der Liste; leer hängt am Ende an."), step=1, precision=0, group="order", order=order)


def angle(label, order, group="geometry", description=None):
    extra = {"description": description} if description else {}
    return ui("dial", label, unit="rad", displayUnit="deg", displayFactor=DEGREES, step=0.017453292519943295, softMin=-3.141592653589793, softMax=3.141592653589793, snaps=[-3.141592653589793, -1.5707963267948966, 0, 1.5707963267948966, 3.141592653589793], group=group, order=order, **extra)


def factor(label, order, group="geometry", description=None):
    extra = {"description": description} if description else {}
    return ui("slider", label, step=0.01, precision=2, softMin=0.1, softMax=10, scale="log", snaps=[0.25, 0.5, 1, 2, 4], group=group, order=order, **extra)


SHAPE_OPTIONS = {"circle": L("Circle", "Kreis"), "rectangle": L("Rectangle", "Rechteck")}
ANCHOR_OPTIONS = {"fixed": L("Fixed", "Fest"), "derived": L("Derived", "Abgeleitet")}
SPECIFICITY_OPTIONS = {"general": L("General", "Allgemein"), "node": L("Node", "Knoten"), "edge": L("Edge", "Kante"), "handle": L("Handle", "Anschluss"), "wire": L("Wire", "Leitung"), "vortex": L("Vortex", "Vortex")}

ANCHOR_UI = ui("segmented", L("Anchor", "Verankerung"), description=L("Fixed keeps the stored position; derived resolves it from the connecting edge.", "Fest behält die gespeicherte Position; abgeleitet ermittelt sie aus der verbindenden Kante."), options=ANCHOR_OPTIONS, group="geometry", order=30)

DEFS = {
    "Puzzle2dNodeAnchor": {},
    "Puzzle2dCompatSpecificity": {},
    "Puzzle2dHandle": {
        "id": text(L("ID", "ID"), 10, "identity", L("Unique handle id on this board.", "Eindeutige Anschluss-ID auf diesem Brett.")),
        "handleKind": text(L("Handle kind", "Anschlussart"), 20, "kind"),
        "angle": angle(L("Angle", "Winkel"), 30, description=L("Position on the node rim, counter-clockwise.", "Position auf dem Knotenrand, gegen den Uhrzeigersinn.")),
        "radius": plain(L("Radius", "Radius"), 40),
        "color": text(L("Color", "Farbe"), 50),
        "iconKind": text(L("Icon", "Symbol"), 60),
        "scale": factor(L("Scale", "Skalierung"), 70, "appearance"),
        "visible": toggle(L("Visible", "Sichtbar"), 80),
        "locked": toggle(L("Locked", "Gesperrt"), 90),
    },
    "Puzzle2dNode": {
        "id": text(L("ID", "ID"), 10, "identity", L("Unique node id on this board.", "Eindeutige Knoten-ID auf diesem Brett.")),
        "nodeKind": text(L("Node kind", "Knotenart"), 20, "kind"),
        "shape": ui("select", L("Shape", "Form"), options=SHAPE_OPTIONS, group="geometry", order=30),
        "x": position(L("X", "X"), 40),
        "y": position(L("Y", "Y"), 50),
        "radius": length(L("Radius", "Radius"), 60),
        "width": length(L("Width", "Breite"), 70),
        "height": length(L("Height", "Höhe"), 80),
        "text": text(L("Text", "Text"), 90),
        "iconKind": text(L("Icon", "Symbol"), 100),
        "root": toggle(L("Root", "Wurzel"), 110),
        "scale": factor(L("Scale", "Skalierung"), 120, "appearance"),
        "visible": toggle(L("Visible", "Sichtbar"), 130),
        "locked": toggle(L("Locked", "Gesperrt"), 140),
        "anchor": ANCHOR_UI | {"order": 150},
        "handles": {"role": "value", "label": L("Handles", "Anschlüsse"), "group": "topology", "order": 160},
    },
    "Puzzle2dTargetRegion": {
        "id": text(L("ID", "ID"), 10, "identity", L("Unique target region id on this board.", "Eindeutige Zielregions-ID auf diesem Brett.")),
        "x": position(L("X", "X"), 20),
        "y": position(L("Y", "Y"), 30),
        "width": length(L("Width", "Breite"), 40),
        "height": length(L("Height", "Höhe"), 50),
        "label": text(L("Label", "Beschriftung"), 60),
        "hidden": toggle(L("Hidden", "Ausgeblendet"), 70),
        "locked": toggle(L("Locked", "Gesperrt"), 80),
    },
    "Puzzle2dKindCatalogs": {
        "nodes": {"role": "value", "label": L("Node kinds", "Knotenarten"), "group": "catalog", "order": 10},
        "handles": {"role": "value", "label": L("Handle kinds", "Anschlussarten"), "group": "catalog", "order": 20},
        "edges": {"role": "value", "label": L("Edge kinds", "Kantenarten"), "group": "catalog", "order": 30},
        "wires": {"role": "value", "label": L("Wire kinds", "Leitungsarten"), "group": "catalog", "order": 40},
    },
    "Puzzle2dCatalogNodeKind": {
        "id": text(L("ID", "ID"), 10, "identity"),
        "name": text(L("Name", "Name"), 20, "identity"),
        "label": text(L("Label", "Beschriftung"), 30),
        "description": text(L("Description", "Beschreibung"), 40, widget="multiline"),
        "icon": text(L("Icon", "Symbol"), 50),
        "image": text(L("Image", "Bild"), 60),
        "unit": text(L("Unit", "Einheit"), 70, "geometry"),
        "abstract": toggle(L("Abstract", "Abstrakt"), 80, "kind"),
        "baseKinds": {"role": "value", "label": L("Base kinds", "Basisarten"), "group": "kind", "order": 90},
        "representations": {"role": "value", "label": L("Representations", "Darstellungen"), "group": "appearance", "order": 100},
        "handles": {"role": "value", "label": L("Handles", "Anschlüsse"), "group": "topology", "order": 110},
        "attributes": {"role": "value", "label": L("Attributes", "Attribute"), "group": "metadata", "order": 120},
        "authors": {"role": "value", "label": L("Authors", "Autoren"), "group": "metadata", "order": 130},
    },
    "Puzzle2dCatalogHandleKind": {
        "id": text(L("ID", "ID"), 10, "identity"),
        "code": text(L("Code", "Code"), 20, "identity"),
        "label": text(L("Label", "Beschriftung"), 30),
        "description": text(L("Description", "Beschreibung"), 40, widget="multiline"),
        "icon": text(L("Icon", "Symbol"), 50),
        "color": text(L("Color", "Farbe"), 60),
        "order": ui("stepper", L("Order", "Reihenfolge"), step=1, precision=0, group="order", order=70),
        "compatibleWith": {"role": "value", "label": L("Compatible with", "Kompatibel mit"), "group": "kind", "order": 80},
        "defaultWireKind": text(L("Default wire kind", "Standard-Leitungsart"), 90, "kind"),
    },
    "Puzzle2dCatalogEdgeKind": {
        "id": text(L("ID", "ID"), 10, "identity"),
        "name": text(L("Name", "Name"), 20, "identity"),
        "label": text(L("Label", "Beschriftung"), 30),
        "description": text(L("Description", "Beschreibung"), 40, widget="multiline"),
        "icon": text(L("Icon", "Symbol"), 50),
        "color": text(L("Color", "Farbe"), 60),
    },
    "Puzzle2dCatalogWireKind": {
        "id": text(L("ID", "ID"), 10, "identity"),
        "name": text(L("Name", "Name"), 20, "identity"),
        "label": text(L("Label", "Beschriftung"), 30),
        "description": text(L("Description", "Beschreibung"), 40, widget="multiline"),
        "icon": text(L("Icon", "Symbol"), 50),
        "color": text(L("Color", "Farbe"), 60),
        "defaultEdgeKind": text(L("Default edge kind", "Standard-Kantenart"), 70, "kind"),
    },
    "Puzzle2dHandleTemplate": {
        "id": text(L("ID", "ID"), 10, "identity"),
        "name": text(L("Name", "Name"), 20, "identity"),
        "label": text(L("Label", "Beschriftung"), 30),
        "description": text(L("Description", "Beschreibung"), 40, widget="multiline"),
        "icon": text(L("Icon", "Symbol"), 50),
        "handleKind": text(L("Handle kind", "Anschlussart"), 60, "kind"),
        "angle": angle(L("Angle", "Winkel"), 70),
        "radius": plain(L("Radius", "Radius"), 80),
        "t": ui("slider", L("Rim parameter", "Randparameter"), description=L("Position along the node outline, 0 to 1.", "Position entlang des Knotenumrisses, 0 bis 1."), step=0.01, precision=2, softMin=0, softMax=1, group="geometry", order=90),
        "mandatory": toggle(L("Mandatory", "Pflicht"), 100, "kind"),
    },
    "Puzzle2dRepresentation": {
        "id": text(L("ID", "ID"), 10, "identity"),
        "name": text(L("Name", "Name"), 20, "identity"),
        "url": text(L("URL", "URL"), 30, "source"),
        "mime": text(L("Media type", "Medientyp"), 40, "source"),
        "lod": text(L("Level of detail", "Detailstufe"), 50, "source"),
        "tags": {"role": "value", "label": L("Tags", "Schlagwörter"), "group": "metadata", "order": 60},
        "description": text(L("Description", "Beschreibung"), 70, widget="multiline"),
    },
    "Puzzle2dAttribute": {
        "id": text(L("ID", "ID"), 10, "identity"),
        "key": text(L("Key", "Schlüssel"), 20, "metadata"),
        "value": text(L("Value", "Wert"), 30, "metadata"),
        "definition": text(L("Definition", "Definition"), 40, "metadata"),
    },
    "Puzzle2dAuthor": {
        "id": text(L("ID", "ID"), 10, "identity"),
        "name": text(L("Name", "Name"), 20, "identity"),
        "email": text(L("Email", "E-Mail"), 30, "identity"),
        "role": text(L("Role", "Rolle"), 40, "metadata"),
        "rank": ui("stepper", L("Rank", "Rang"), step=1, precision=0, group="order", order=50),
    },
}
DEF_BOUNDS = {
    ("Puzzle2dHandle", "radius"): {"exclusiveMinimum": 0},
    ("Puzzle2dHandle", "scale"): {"exclusiveMinimum": 0},
    ("Puzzle2dHandleTemplate", "radius"): {"exclusiveMinimum": 0},
    ("Puzzle2dHandleTemplate", "t"): {"minimum": 0, "maximum": 1},
    ("Puzzle2dNode", "shape"): {"enum": ["circle", "rectangle"]},
    ("Puzzle2dNode", "radius"): {"exclusiveMinimum": 0},
    ("Puzzle2dNode", "width"): {"exclusiveMinimum": 0},
    ("Puzzle2dNode", "height"): {"exclusiveMinimum": 0},
    ("Puzzle2dNode", "scale"): {"exclusiveMinimum": 0},
    ("Puzzle2dCatalogHandleKind", "order"): {"minimum": 0},
    ("Puzzle2dAuthor", "rank"): {"minimum": 0},
}

EDGE_GEOMETRY = {
    "gap": L("Gap", "Abstand"),
    "shift": L("Shift", "Versatz"),
    "rise": L("Rise", "Anhebung"),
    "rotation": L("Rotation", "Drehung"),
    "turn": L("Turn", "Wendung"),
    "tilt": L("Tilt", "Neigung"),
}


def edge_geometry(prefix, first):
    fields = {}
    for offset, (name, label) in enumerate(EDGE_GEOMETRY.items()):
        key = prefix + name[0].upper() + name[1:] if prefix else name
        fields[key] = plain(label, first + offset * 10)
    fields[prefix + "X" if prefix else "x"] = position(L("X", "X"), first + 60)
    fields[prefix + "Y" if prefix else "y"] = position(L("Y", "Y"), first + 70)
    return fields


LEAVES = {
    "create-node": {"node": {"role": "value", "label": L("Node", "Knoten"), "description": L("The complete node record to add.", "Der vollständige hinzuzufügende Knoten."), "group": "record", "order": 10}, "index": index(20)},
    "delete-node": {"id": NODE},
    "move-node": {"id": NODE, "newX": position(L("X", "X"), 20, description=L("Absolute final x position.", "Absolute x-Endposition.")), "newY": position(L("Y", "Y"), 30, description=L("Absolute final y position.", "Absolute y-Endposition."))},
    "replace-node-geometry": {
        "id": NODE,
        "newShape": ui("select", L("Shape", "Form"), options=SHAPE_OPTIONS, group="geometry", order=20),
        "newRadius": length(L("Radius", "Radius"), 30),
        "newWidth": length(L("Width", "Breite"), 40),
        "newHeight": length(L("Height", "Höhe"), 50),
    },
    "change-node-kind": {"id": NODE, "newNodeKind": text(L("Node kind", "Knotenart"), 20, "kind")},
    "edit-node-text": {"id": NODE, "newText": text(L("Text", "Text"), 20)},
    "change-node-icon": {"id": NODE, "newIconKind": text(L("Icon", "Symbol"), 20)},
    "scale-node": {"id": NODE, "newScale": factor(L("Scale", "Skalierung"), 20, "appearance", L("The node's own size factor; empty resets to 1.", "Eigener Größenfaktor des Knotens; leer setzt auf 1 zurück."))},
    "change-node-visible": {"id": NODE, "newVisible": toggle(L("Visible", "Sichtbar"), 20)},
    "change-node-locked": {"id": NODE, "newLocked": toggle(L("Locked", "Gesperrt"), 20)},
    "change-node-root": {"id": NODE, "newRoot": toggle(L("Root", "Wurzel"), 20)},
    "change-node-anchor": {"id": NODE, "newAnchor": ANCHOR_UI | {"order": 20}},
    "add-node-handle": {"nodeId": OWNER, "handle": {"role": "value", "label": L("Handle", "Anschluss"), "description": L("The complete handle record to add.", "Der vollständige hinzuzufügende Anschluss."), "group": "record", "order": 20}, "index": index(30)},
    "remove-node-handle": {"nodeId": OWNER, "handleId": HANDLE},
    "replace-node-handle": {"nodeId": OWNER, "handleId": HANDLE, "newHandle": {"role": "value", "label": L("New handle", "Neuer Anschluss"), "description": L("The handle record that replaces the addressed one.", "Der Anschluss, der den adressierten ersetzt."), "group": "record", "order": 30}},
    "connect-handles": {
        "id": text(L("Edge ID", "Kanten-ID"), 10, "identity", L("Id of the new edge.", "ID der neuen Kante.")),
        "source": target("handle", L("Source handle", "Quellanschluss"), L("The handle the edge starts at.", "Der Anschluss, an dem die Kante beginnt."), order=20),
        "target": target("handle", L("Target handle", "Zielanschluss"), L("The handle the edge ends at.", "Der Anschluss, an dem die Kante endet."), order=30),
        "edgeKind": text(L("Edge kind", "Kantenart"), 40, "kind"),
        **edge_geometry("", 50),
        "sourceTip": text(L("Source tip", "Quellspitze"), 130),
        "targetTip": text(L("Target tip", "Zielspitze"), 140),
    },
    "disconnect-handles": {"id": EDGE},
    "replace-edge-geometry": {"id": EDGE, **edge_geometry("new", 20)},
    "change-edge-kind": {"id": EDGE, "newEdgeKind": text(L("Edge kind", "Kantenart"), 20, "kind")},
    "change-edge-tips": {"id": EDGE, "newSourceTip": text(L("Source tip", "Quellspitze"), 20), "newTargetTip": text(L("Target tip", "Zielspitze"), 30)},
    "change-edge-visible": {"id": EDGE, "newVisible": toggle(L("Visible", "Sichtbar"), 20)},
    "change-edge-locked": {"id": EDGE, "newLocked": toggle(L("Locked", "Gesperrt"), 20)},
    "change-manifest-id": {"newManifestId": text(L("Manifest", "Manifest"), 10, "kind", L("Id of the manifest this board follows.", "ID des Manifests, dem dieses Brett folgt."))},
    "connect-kind-compatibility": {
        "source": text(L("Source kind", "Quellart"), 10, "kind"),
        "target": text(L("Target kind", "Zielart"), 20, "kind"),
        "bidirectional": toggle(L("Bidirectional", "Beidseitig"), 30, "kind"),
        "important": toggle(L("Important", "Wichtig"), 40, "kind"),
        "specificity": ui("select", L("Specificity", "Spezifität"), options=SPECIFICITY_OPTIONS, group="kind", order=50),
    },
    "disconnect-kind-compatibility": {"source": text(L("Source kind", "Quellart"), 10, "kind"), "target": text(L("Target kind", "Zielart"), 20, "kind")},
    "replace-kind-catalogs": {"newCatalogs": {"role": "value", "label": L("Kind catalogs", "Artenkataloge"), "description": L("Node, handle, edge and wire kind tables; empty removes them.", "Knoten-, Anschluss-, Kanten- und Leitungsartentabellen; leer entfernt sie."), "group": "record", "order": 10}},
    "create-target-region": {"targetRegion": {"role": "value", "label": L("Target region", "Zielregion"), "description": L("The complete target region record to add.", "Die vollständige hinzuzufügende Zielregion."), "group": "record", "order": 10}, "index": index(20)},
    "delete-target-region": {"id": REGION},
    "move-target-region": {"id": REGION, "newX": position(L("X", "X"), 20, description=L("Absolute final x of the corner.", "Absolute x-Endposition der Ecke.")), "newY": position(L("Y", "Y"), 30, description=L("Absolute final y of the corner.", "Absolute y-Endposition der Ecke."))},
    "resize-target-region": {"id": REGION, "newWidth": length(L("Width", "Breite"), 20), "newHeight": length(L("Height", "Höhe"), 30)},
    "edit-target-region-label": {"id": REGION, "newLabel": text(L("Label", "Beschriftung"), 20)},
    "change-target-region-hidden": {"id": REGION, "newHidden": toggle(L("Hidden", "Ausgeblendet"), 20)},
    "change-target-region-locked": {"id": REGION, "newLocked": toggle(L("Locked", "Gesperrt"), 20)},
    "drag-selection": {
        "targets": target("movable", L("Targets", "Ziele"), L("Nodes and target regions to drag; locked or missing ones are skipped.", "Zu ziehende Knoten und Zielregionen; gesperrte oder fehlende werden übersprungen.")),
        "dx": position(L("Offset X", "Versatz X"), 20, "offset"),
        "dy": position(L("Offset Y", "Versatz Y"), 30, "offset"),
    },
    "rotate-selection": {
        "targets": target("nodes", L("Nodes", "Knoten"), L("Nodes to rotate; locked or missing ones are skipped.", "Zu drehende Knoten; gesperrte oder fehlende werden übersprungen.")),
        "pivotX": position(L("Pivot X", "Drehpunkt X"), 20, "pivot"),
        "pivotY": position(L("Pivot Y", "Drehpunkt Y"), 30, "pivot"),
        "angle": angle(L("Angle", "Winkel"), 40, "rotation", L("Counter-clockwise rotation about the pivot.", "Drehung gegen den Uhrzeigersinn um den Drehpunkt.")),
    },
    "scale-selection": {
        "targets": target("movable", L("Targets", "Ziele"), L("Nodes and target regions to scale; locked or missing ones are skipped.", "Zu skalierende Knoten und Zielregionen; gesperrte oder fehlende werden übersprungen.")),
        "pivotX": position(L("Pivot X", "Bezugspunkt X"), 20, "pivot"),
        "pivotY": position(L("Pivot Y", "Bezugspunkt Y"), 30, "pivot"),
        "factor": factor(L("Factor", "Faktor"), 40, "scale", L("Spreads positions from the pivot; node sizes stay.", "Spreizt Positionen vom Drehpunkt aus; Knotengrößen bleiben.")),
    },
}
for leaf, fields in LEAVES.items():
    for name, annotation in fields.items():
        if "label" not in annotation or "role" not in annotation:
            raise SystemExit("%s/%s lacks label or role" % (leaf, name))

LEAF_BOUNDS = {
    ("replace-node-geometry", "newRadius"): {"exclusiveMinimum": 0},
    ("replace-node-geometry", "newWidth"): {"exclusiveMinimum": 0},
    ("replace-node-geometry", "newHeight"): {"exclusiveMinimum": 0},
    ("scale-node", "newScale"): {"exclusiveMinimum": 0},
}

NEW_LEAVES = {
    "drag-selection": ("✋️drag-selection", "DragSelection", {"targets": {"type": "array", "items": {"type": "string"}, "minItems": 1, "uniqueItems": True}, "dx": {"type": "number"}, "dy": {"type": "number"}}),
    "rotate-selection": ("🔄️rotate-selection", "RotateSelection", {"targets": {"type": "array", "items": {"type": "string"}, "minItems": 1, "uniqueItems": True}, "pivotX": {"type": "number"}, "pivotY": {"type": "number"}, "angle": {"type": "number"}}),
    "scale-selection": ("🔍️scale-selection", "ScaleSelection", {"targets": {"type": "array", "items": {"type": "string"}, "minItems": 1, "uniqueItems": True}, "pivotX": {"type": "number"}, "pivotY": {"type": "number"}, "factor": {"type": "number", "exclusiveMinimum": 0}}),
}


def tag_of(kind):
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


def kind_of(leaf):
    for at, character in enumerate(leaf):
        if character.isascii() and character.isalpha():
            return leaf[at:]
    return leaf


REPLACED = {
    ("replace-node-geometry", "newShape"): {"anyOf": [{"type": "string", "enum": ["circle", "rectangle"]}, {"type": "null"}]},
}


def annotated(prop, annotation, bounds, replaced=None):
    body = dict(replaced) if replaced is not None else {key: value for key, value in prop.items() if key != "x-semio-ui"}
    body.update(bounds)
    body["x-semio-ui"] = annotation
    return body


def write(path, schema):
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")


def main():
    for kind, (leaf, title, props) in NEW_LEAVES.items():
        path = os.path.join(ROOT, leaf, "🧬️schema", "🔣️.json")
        schema = {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": "https://json.schemas.assets.semio-tech.com/s/puzzle/puzzle2d/mutation/%s/schema.json" % kind,
            "title": title,
            "description": "The `%s` mutation record — internally tagged `\"mutation\": \"%s\"`, the one branch the aggregate `../🔣️.json` union carries for this kind." % (kind, tag_of(kind)),
            "type": "object",
            "additionalProperties": False,
            "required": ["mutation", *props],
            "properties": {"mutation": {"const": tag_of(kind)}, **props},
        }
        write(path, schema)
    seen = set()
    for leaf in sorted(os.listdir(ROOT)):
        path = os.path.join(ROOT, leaf, "🧬️schema", "🔣️.json")
        if not os.path.exists(path):
            continue
        kind = kind_of(leaf)
        fields = LEAVES[kind]
        schema = json.load(open(path, encoding="utf-8"))
        properties = {}
        for name, prop in schema["properties"].items():
            if name == "mutation":
                properties[name] = annotated(prop, DISCRIMINATOR, {})
                continue
            if name not in fields:
                raise SystemExit("%s: input %s has no annotation" % (kind, name))
            properties[name] = annotated(prop, fields[name], LEAF_BOUNDS.get((kind, name), {}), REPLACED.get((kind, name)))
        missing = set(fields) - set(schema["properties"])
        if missing:
            raise SystemExit("%s: annotations for undeclared inputs %s" % (kind, sorted(missing)))
        schema["properties"] = properties
        for def_name, definition in schema.get("$defs", {}).items():
            table = DEFS[def_name]
            if "properties" not in definition:
                continue
            definition["properties"] = {name: annotated(prop, table[name], DEF_BOUNDS.get((def_name, name), {})) for name, prop in definition["properties"].items()}
        write(path, schema)
        seen.add(kind)
    if seen != set(LEAVES):
        raise SystemExit("leaf set mismatch: %s" % sorted(set(LEAVES) ^ seen))
    print("annotated %d leaf schemas" % len(seen))


if __name__ == "__main__":
    sys.exit(main())
