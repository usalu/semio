#!/usr/bin/env python3
"""🏷️ W2-R design group: writes the `x-semio-ui` input annotations (design §6, manifest `$defs/InputUi`) onto every leaf
payload schema of puzzle 3d/5d, shooting, cad, procedural, note, forms, raster and draw — every top-level input, every
field of every record a leaf reaches through `$ref` (local `$defs` or a sibling record document), every union variant.

The tables below are keyed by catalog scope id (leaf-owned fields, dotted payload path, `[]` for array items, `|<const>`
for a union variant) or by record name (`title` or `$defs` key). The walker refuses a reachable field without an entry and
a table entry nothing reaches, so the tables are exactly the input surface. Standard-formatted files are rewritten with
`json.dumps(indent=2)`; hand-formatted files get span edits so their layout survives. Idempotent.

    python3 🧪️w2-r-design-annotate-inputs.py [--dry-run] [--report]

@see ./🧪️w1-f-annotate-puzzle2d-inputs.py
@see ./🧪️w2-r-design-check-schemas.py
@see ../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
CATALOG = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"
PLUGINS = "✏️s/🔌️plugins"
ROOTS = (f"{PLUGINS}/🧩️puzzle/🗿️artifacts/🧊️3d", f"{PLUGINS}/🧩️puzzle/🗿️artifacts/🖐️5d", f"{PLUGINS}/🎥️shooting", f"{PLUGINS}/📐️cad", f"{PLUGINS}/🌀️procedural", f"{PLUGINS}/🗒️note", f"{PLUGINS}/📋️forms", f"{PLUGINS}/🖨️raster", f"{PLUGINS}/🖍️draw")
UNCATALOGUED = {"s.draw.drawing.1.transform.mutation.update-path-geometry": f"{PLUGINS}/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧬️schema/🧬️mutations/✏️update-path-geometry/🧬️schema"}
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "options", "group", "order")
PI = 3.141592653589793
DEGREES = 57.29577951308232
RADIAN_STEP = 0.017453292519943295


#region 🔖️Vocabulary
def L(en, de):
    return {"en": en, "de": de}


def ui(widget, label, role="value", bounds=None, **extra):
    body = {"widget": widget, "role": role, "label": label, **extra}
    entry = {key: body[key] for key in KEY_ORDER if body.get(key) is not None}
    if bounds:
        entry["_bounds"] = bounds
    return entry


def target(kind, label, description, domain=None, granularity=None, group="target", order=None):
    ref = {key: value for key, value in (("kind", kind), ("domain", domain), ("granularity", granularity)) if value is not None}
    return ui("reference", label, role="target", description=description, ref=ref, group=group, order=order)


def text(label, group, description=None, order=None):
    return ui("text", label, description=description, group=group, order=order)


def multiline(label, group, description=None, order=None):
    return ui("multiline", label, description=description, group=group, order=order)


def toggle(label, group, description=None, order=None):
    return ui("toggle", label, description=description, group=group, order=order)


def select(label, options, group, description=None, widget="select", order=None):
    return ui(widget, label, description=description, options=options, group=group, order=order)


def record(label, group, description=None, order=None):
    return ui(None, label, description=description, group=group, order=order)


def stepper(label, group, step=1, precision=2, description=None, unit=None, snap_source=None, soft=None, bounds=None, order=None, display_unit=None, display_factor=None):
    soft_min, soft_max = soft if soft else (None, None)
    return ui("stepper", label, description=description, unit=unit, displayUnit=display_unit, displayFactor=display_factor, step=step, precision=precision, softMin=soft_min, softMax=soft_max, snapSource=snap_source, bounds=bounds, group=group, order=order)


def slider(label, group, soft, step=0.01, precision=2, description=None, unit=None, snaps=None, scale=None, bounds=None, order=None, display_unit=None, display_factor=None):
    return ui("slider", label, description=description, unit=unit, displayUnit=display_unit, displayFactor=display_factor, step=step, precision=precision, softMin=soft[0], softMax=soft[1], scale=scale, snaps=snaps, bounds=bounds, group=group, order=order)


def integer(label, group, description=None, bounds=None, order=None, unit=None):
    return ui("stepper", label, description=description, unit=unit, step=1, precision=0, bounds=bounds, group=group, order=order)


def index(order=None):
    return integer(L("Index", "Index"), "order", L("Position in the list; the end when omitted.", "Position in der Liste; ohne Angabe am Ende."), order=order)


def vector(label, group, description=None, unit=None, step=None, order=None):
    return ui("vector", label, description=description, unit=unit, step=step, group=group, order=order)


def angle_rad(label, group, description=None, order=None):
    return ui("dial", label, description=description, unit="rad", displayUnit="deg", displayFactor=DEGREES, step=RADIAN_STEP, precision=4, softMin=-PI, softMax=PI, snaps=[-PI, -PI / 2, 0, PI / 2, PI], group=group, order=order)


def angle_deg(label, group, description=None, soft=(-180, 180), snaps=(-180, -90, 0, 90, 180), bounds=None, order=None):
    return ui("dial", label, description=description, unit="deg", step=1, precision=1, softMin=soft[0], softMax=soft[1], snaps=list(snaps), bounds=bounds, group=group, order=order)


def factor(label, group, description=None, bounds=None, order=None):
    return ui("slider", label, description=description, step=0.01, precision=2, softMin=0.1, softMax=10, scale="log", snaps=[0.25, 0.5, 1, 2, 4], bounds=bounds, group=group, order=order)


def unit_interval(label, group, description=None, bounds=None, order=None, percent=True):
    return ui("slider", label, description=description, displayUnit="%" if percent else None, displayFactor=100 if percent else None, step=0.01, precision=2, softMin=0, softMax=1, snaps=[0, 0.25, 0.5, 0.75, 1], bounds=bounds, group=group, order=order)


def discriminator(label=None):
    return {"widget": "hidden", "role": "discriminator", "label": label or L("Kind", "Art")}


QUATERNION = L("Unit quaternion (x, y, z, w); (0, 0, 0, 1) is no rotation.", "Einheitsquaternion (x, y, z, w); (0, 0, 0, 1) ist keine Drehung.")
RGBA = L("Red, green, blue and alpha, each from 0 to 1.", "Rot, Grün, Blau und Alpha, jeweils von 0 bis 1.")
HEX = L("CSS color, e.g. #3366ff.", "CSS-Farbe, z. B. #3366ff.")
URL = L("Address of the file, e.g. an https or data URL.", "Adresse der Datei, z. B. eine https- oder data-URL.")
POSITIVE = {"exclusiveMinimum": 0}
NON_NEGATIVE = {"minimum": 0}
UNIT = {"minimum": 0, "maximum": 1}
#endregion 🔖️Vocabulary

LEAVES = {}
RECORDS = {}
VARIANTS = {}


def leaves(prefix, table):
    for kind, fields in table.items():
        LEAVES[prefix + kind] = fields


#region 🔖️Puzzle
def puzzle_target(granularity, label, description, kind=None, order=None):
    return target(kind or granularity, label, description, domain="vortex", granularity=granularity, order=order)


def origin(label=L("Origin", "Ursprung"), description=None, order=None):
    return vector(label, "pose", description, unit="m", step=0.1, order=order)


ORIENTATION = vector(L("Orientation", "Orientierung"), "pose", QUATERNION, step=0.01)
FREE_SCALE = record(L("Scale", "Skalierung"), "pose", L("One factor for all three axes, or one factor per axis (x, y, z).", "Ein Faktor für alle drei Achsen oder ein Faktor je Achse (x, y, z)."))
MESH = text(L("Mesh URL", "Mesh-URL"), "source", URL)
HIDDEN = toggle(L("Hidden", "Ausgeblendet"), "state")
LOCKED = toggle(L("Locked", "Gesperrt"), "state")
KIND_TEXT = lambda label: text(label, "kind", L("Id of a kind from the kind catalog.", "ID einer Art aus dem Artenkatalog."))
NEW_ID = lambda noun_en, noun_de: text(L("ID", "ID"), "identity", L(f"Unique id of the new {noun_en}.", f"Eindeutige ID {noun_de}."))


def connection(prefix, first):
    def key(name):
        return prefix + name[0].upper() + name[1:] if prefix else name
    fields = {}
    for offset, (name, label) in enumerate((("gap", L("Gap", "Spalt")), ("shift", L("Shift", "Verschiebung")), ("rise", L("Rise", "Anstieg")))):
        fields[key(name)] = stepper(label, "offset", step=0.01, precision=3, unit="m", order=first + offset * 10)
    for offset, (name, label) in enumerate((("rotation", L("Rotation", "Drehung")), ("turn", L("Turn", "Drehung um Achse")), ("tilt", L("Tilt", "Neigung")))):
        fields[key(name)] = angle_deg(label, "rotation", order=first + 30 + offset * 10)
    fields[key("x")] = stepper(L("Diagram X", "Diagramm-X"), "diagram", step=1, precision=2, description=L("Horizontal offset in the connection diagram.", "Horizontaler Versatz im Verbindungsdiagramm."), order=first + 60)
    fields[key("y")] = stepper(L("Diagram Y", "Diagramm-Y"), "diagram", step=1, precision=2, description=L("Vertical offset in the connection diagram.", "Vertikaler Versatz im Verbindungsdiagramm."), order=first + 70)
    return fields


def compatibility(options):
    return {
        "source": text(L("Source kind", "Quellart"), "kind"),
        "target": text(L("Target kind", "Zielart"), "kind"),
        "bidirectional": toggle(L("Bidirectional", "Beidseitig"), "kind"),
        "important": toggle(L("Important", "Wichtig"), "kind"),
        "specificity": select(L("Specificity", "Spezifität"), options, "kind", L("Which level of the kind hierarchy this pair applies to.", "Auf welcher Ebene der Artenhierarchie dieses Paar gilt.")),
    }


DOMAIN = text(L("Domain", "Domäne"), "metadata", L("Design domain this puzzle belongs to.", "Entwurfsdomäne, zu der dieses Puzzle gehört."))
ANCHOR_OPTIONS = {"fixed": L("Fixed", "Fest"), "derived": L("Derived", "Abgeleitet")}
ANCHOR_DESCRIPTION = L("Fixed keeps the stored plane at a root; derived resets it to the default XY plane.", "Fest behält die gespeicherte Ebene an einer Wurzel; abgeleitet setzt sie auf die XY-Standardebene zurück.")


def catalog_records(prefix, template, port, link, edge):
    """🗂️ The shared catalog records of puzzle 3d and 5d: `port`/`link`/`edge` are `(Name, label, kind label, key)` of the
    rim port (vortex/grip), the link kind (cable/rope) and the connection kind (attraction/fastener)."""
    port_name, port_label, port_kind_label, port_key = port
    link_name, link_label, link_kind_label, link_key = link
    edge_name, edge_label, edge_kind_label, edge_key = edge
    return {
        f"{prefix}Representation": {
            "id": text(L("ID", "ID"), "identity"), "name": text(L("Name", "Name"), "identity"), "url": text(L("URL", "URL"), "source", URL),
            "mime": text(L("Media type", "Medientyp"), "source"), "tags": record(L("Tags", "Schlagwörter"), "metadata"), "lod": text(L("Level of detail", "Detailstufe"), "source"),
            "description": multiline(L("Description", "Beschreibung"), "metadata"),
        },
        f"{prefix}Attribute": {"id": text(L("ID", "ID"), "identity"), "key": text(L("Key", "Schlüssel"), "metadata"), "value": text(L("Value", "Wert"), "metadata"), "definition": text(L("Definition", "Definition"), "metadata")},
        f"{prefix}Author": {"id": text(L("ID", "ID"), "identity"), "name": text(L("Name", "Name"), "identity"), "email": text(L("Email", "E-Mail"), "identity"), "role": text(L("Role", "Rolle"), "metadata"), "rank": integer(L("Rank", "Rang"), "order", bounds=NON_NEGATIVE)},
        template: {
            "id": text(L("ID", "ID"), "identity"), "name": text(L("Name", "Name"), "identity"), "label": text(L("Label", "Bezeichnung"), "appearance"),
            "description": multiline(L("Description", "Beschreibung"), "appearance"), "icon": text(L("Icon", "Symbol"), "appearance"),
            port_key: KIND_TEXT(port_kind_label),
            "point": origin(L("Point", "Punkt"), L("Where the port sits in the kind's local frame.", "Lage des Anschlusses im lokalen System der Art.")),
            "direction": vector(L("Direction", "Richtung"), "geometry", L("Direction the port points in.", "Richtung, in die der Anschluss zeigt."), step=0.1),
            "t": slider(L("Rim parameter", "Randparameter"), "geometry", (0, 1), description=L("Position along the outline, 0 to 1.", "Position entlang des Umrisses, 0 bis 1."), bounds=UNIT),
            "mandatory": toggle(L("Mandatory", "Pflicht"), "kind"),
            "radius": stepper(L("Radius", "Radius"), "geometry", step=0.01, precision=3, unit="m"),
        },
        f"{prefix}Catalog{port_name}Kind": {
            "id": text(L("ID", "ID"), "identity"), "code": text(L("Code", "Code"), "identity"), "label": text(L("Label", "Bezeichnung"), "appearance"),
            "order": integer(L("Order", "Reihenfolge"), "order", bounds=NON_NEGATIVE),
            "compatibleWith": record(L("Compatible with", "Kompatibel mit"), "kind", L("Kinds this kind may connect to.", "Arten, mit denen diese Art verbunden werden darf.")),
            "description": multiline(L("Description", "Beschreibung"), "appearance"), "icon": text(L("Icon", "Symbol"), "appearance"), "color": text(L("Color", "Farbe"), "appearance", HEX),
            link_key: KIND_TEXT(link_kind_label),
        },
        f"{prefix}Catalog{link_name}Kind": {"id": text(L("ID", "ID"), "identity"), "label": text(L("Label", "Bezeichnung"), "appearance"), "name": text(L("Name", "Name"), "identity"), edge_key: KIND_TEXT(edge_kind_label)},
        f"{prefix}Catalog{edge_name}Kind": {"id": text(L("ID", "ID"), "identity"), "label": text(L("Label", "Bezeichnung"), "appearance"), "name": text(L("Name", "Name"), "identity")},
    }


def owner_kind(port_label, port_key):
    return {
        "id": text(L("ID", "ID"), "identity"), "name": text(L("Name", "Name"), "identity"), "label": text(L("Label", "Bezeichnung"), "appearance"),
        "description": multiline(L("Description", "Beschreibung"), "appearance"), "icon": text(L("Icon", "Symbol"), "appearance"), "image": text(L("Image", "Bild"), "appearance", URL),
        "unit": text(L("Unit", "Einheit"), "geometry", L("Length unit the kind's geometry is modelled in.", "Längeneinheit, in der die Geometrie der Art modelliert ist.")),
        "isAbstract": toggle(L("Abstract", "Abstrakt"), "kind", L("An abstract kind is only a base for other kinds and is never placed.", "Eine abstrakte Art dient nur als Basis anderer Arten und wird nie platziert.")),
        "baseKinds": record(L("Base kinds", "Basisarten"), "kind"), "representations": record(L("Representations", "Darstellungen"), "appearance"),
        port_key: record(port_label, "topology"), "attributes": record(L("Attributes", "Attribute"), "metadata"), "authors": record(L("Authors", "Autoren"), "metadata"),
    }


OBJECT3 = puzzle_target("object", L("Object", "Objekt"), L("The object this mutation addresses.", "Das Objekt, das diese Mutation adressiert."))
REFERENCE3 = puzzle_target("reference", L("Reference", "Referenz"), L("The reference image this mutation addresses.", "Das Referenzbild, das diese Mutation adressiert."))
VOLUME3 = puzzle_target("targetVolume", L("Target volume", "Zielvolumen"), L("The target volume this mutation addresses.", "Das Zielvolumen, das diese Mutation adressiert."))
ATTRACTION3 = puzzle_target("attraction", L("Attraction", "Anziehung"), L("The attraction this mutation addresses.", "Die Anziehung, die diese Mutation adressiert."))
VORTEX_OWNER3 = puzzle_target("object", L("Object", "Objekt"), L("The object that owns the vortex.", "Das Objekt, dem der Vortex gehört."))
VORTEX3 = target("vortex", L("Vortex", "Vortex"), L("Id of the vortex on its object.", "ID des Vortex an seinem Objekt."), order=20)
WIDTH_WORLD = stepper(L("Width", "Breite"), "geometry", step=0.1, precision=3, unit="m", description=L("Width of the reference plane in the world.", "Breite der Referenzebene in der Welt."), bounds=POSITIVE)

leaves("s.puzzle.puzzle3d.mutation.", {
    "add-object-vortex": {"objectId": VORTEX_OWNER3, "vortex": record(L("Vortex", "Vortex"), "record", L("The complete vortex record to add.", "Der vollständige hinzuzufügende Vortex.")), "index": index()},
    "change-domain": {"newDomain": DOMAIN},
    "change-object-anchor": {"id": OBJECT3, "newAnchor": select(L("Anchor", "Verankerung"), {"Fixed": L("Fixed", "Fest"), "Derived": L("Derived", "Abgeleitet")}, "geometry", ANCHOR_DESCRIPTION, widget="segmented")},
    "change-object-hidden": {"id": OBJECT3, "newHidden": HIDDEN},
    "change-object-kind": {"id": OBJECT3, "newObjectKind": KIND_TEXT(L("Object kind", "Objektart"))},
    "change-object-locked": {"id": OBJECT3, "newLocked": LOCKED},
    "change-object-mesh": {"id": OBJECT3, "newMeshUrl": MESH},
    "change-reference-hidden": {"id": REFERENCE3, "newHidden": HIDDEN},
    "change-reference-locked": {"id": REFERENCE3, "newLocked": LOCKED},
    "change-target-volume-hidden": {"id": VOLUME3, "newHidden": HIDDEN},
    "change-target-volume-locked": {"id": VOLUME3, "newLocked": LOCKED},
    "connect-kind-compatibility": compatibility({"general": L("General", "Allgemein"), "object": L("Object", "Objekt"), "attraction": L("Attraction", "Anziehung"), "cable": L("Cable", "Kabel"), "vortex": L("Vortex", "Vortex")}),
    "connect-vortices": {
        "id": NEW_ID("attraction", "der neuen Anziehung"),
        "attracting": puzzle_target("vortex", L("Attracting vortex", "Anziehender Vortex"), L("Full id (object:vortex) of the vortex that attracts.", "Vollständige ID (Objekt:Vortex) des anziehenden Vortex."), order=20),
        "attracted": puzzle_target("vortex", L("Attracted vortex", "Angezogener Vortex"), L("Full id (object:vortex) of the vortex that is attracted.", "Vollständige ID (Objekt:Vortex) des angezogenen Vortex."), order=30),
        **connection("", 40),
    },
    "create-object": {"object": record(L("Object", "Objekt"), "record", L("The complete object record to add.", "Das vollständige hinzuzufügende Objekt.")), "index": index()},
    "create-reference": {"reference": record(L("Reference", "Referenz"), "record", L("The complete reference record to add.", "Die vollständige hinzuzufügende Referenz.")), "index": index()},
    "create-target-volume": {"targetVolume": record(L("Target volume", "Zielvolumen"), "record", L("The complete target volume record to add.", "Das vollständige hinzuzufügende Zielvolumen.")), "index": index()},
    "delete-object": {"id": OBJECT3},
    "delete-reference": {"id": REFERENCE3},
    "delete-target-volume": {"id": VOLUME3},
    "disconnect-kind-compatibility": {"source": text(L("Source kind", "Quellart"), "kind"), "target": text(L("Target kind", "Zielart"), "kind")},
    "disconnect-vortices": {"id": ATTRACTION3},
    "edit-object-label": {"id": OBJECT3, "newLabel": text(L("Label", "Bezeichnung"), "appearance")},
    "move-object": {"id": OBJECT3, "newOrigin": origin(description=L("Absolute final origin in the world.", "Absoluter Endursprung in der Welt."))},
    "move-reference": {"id": REFERENCE3, "newOrigin": origin(description=L("Absolute final origin in the world.", "Absoluter Endursprung in der Welt."))},
    "move-target-volume": {"id": VOLUME3, "newOrigin": origin(description=L("Absolute final origin in the world.", "Absoluter Endursprung in der Welt."))},
    "remove-object-vortex": {"objectId": VORTEX_OWNER3, "vortexId": VORTEX3},
    "replace-attraction-geometry": {"id": ATTRACTION3, **connection("new", 20)},
    "replace-kind-catalogs": {"newCatalogs": record(L("Kind catalogs", "Artenkataloge"), "record", L("Object, vortex, cable and attraction kind tables; empty removes them.", "Objekt-, Vortex-, Kabel- und Anziehungsartentabellen; leer entfernt sie."))},
    "replace-object-vortex": {"objectId": VORTEX_OWNER3, "vortexId": VORTEX3, "newVortex": record(L("New vortex", "Neuer Vortex"), "record", L("The vortex record that replaces the addressed one.", "Der Vortex, der den adressierten ersetzt."))},
    "replace-reference-source": {"id": REFERENCE3, "newSource": record(L("Source", "Quelle"), "source", L("Where the reference media lives and what kind it is.", "Wo das Referenzmedium liegt und welcher Art es ist."))},
    "resize-reference": {"id": REFERENCE3, "newWidthWorld": WIDTH_WORLD},
    "rotate-object": {"id": OBJECT3, "newOrientation": ORIENTATION},
    "rotate-target-volume": {"id": VOLUME3, "newOrientation": ORIENTATION},
    "scale-object": {"id": OBJECT3, "newScale": FREE_SCALE},
    "scale-target-volume": {"id": VOLUME3, "newScale": FREE_SCALE},
})
RECORDS.update({
    "Puzzle3dVortex": {
        "id": text(L("ID", "ID"), "identity", L("Id of the vortex, unique on its object.", "ID des Vortex, eindeutig an seinem Objekt.")),
        "vortexKind": KIND_TEXT(L("Vortex kind", "Vortex-Art")),
        "position": origin(L("Position", "Position"), L("Where the vortex sits in the object's local frame.", "Lage des Vortex im lokalen System des Objekts.")),
        "direction": vector(L("Direction", "Richtung"), "geometry", L("Direction the vortex points in.", "Richtung, in die der Vortex zeigt."), step=0.1),
        "radius": stepper(L("Radius", "Radius"), "geometry", step=0.01, precision=3, unit="m", description=L("Collision radius used by the brush fill.", "Kollisionsradius für das Pinselfüllen.")),
        "hidden": HIDDEN, "locked": LOCKED,
    },
    "Puzzle3dObject": {
        "id": text(L("ID", "ID"), "identity", L("Unique object id in this puzzle.", "Eindeutige Objekt-ID in diesem Puzzle.")), "label": text(L("Label", "Bezeichnung"), "appearance"),
        "objectKind": KIND_TEXT(L("Object kind", "Objektart")), "origin": origin(), "orientation": ORIENTATION, "scale": FREE_SCALE, "meshUrl": MESH,
        "vortices": record(L("Vortices", "Vortices"), "topology"), "hidden": HIDDEN, "locked": LOCKED,
    },
    "Puzzle3dReference": {
        "id": text(L("ID", "ID"), "identity", L("Unique reference id in this puzzle.", "Eindeutige Referenz-ID in diesem Puzzle.")), "source": record(L("Source", "Quelle"), "source"),
        "origin": origin(), "widthWorld": WIDTH_WORLD, "locked": LOCKED, "hidden": HIDDEN,
    },
    "Puzzle3dReferenceSource": {"url": text(L("Source URL", "Quell-URL"), "source", URL), "mediaKind": text(L("Media kind", "Medienart"), "source", L("For example image or drawing.", "Zum Beispiel image (Bild) oder drawing (Zeichnung)."))},
    "Puzzle3dTargetVolume": {
        "id": text(L("ID", "ID"), "identity", L("Unique target volume id in this puzzle.", "Eindeutige Zielvolumen-ID in diesem Puzzle.")),
        "origin": origin(), "orientation": ORIENTATION, "scale": FREE_SCALE, "hidden": HIDDEN, "locked": LOCKED,
    },
    "Puzzle3dKindCatalogs": {
        "objects": record(L("Object kinds", "Objektarten"), "catalog"), "vortices": record(L("Vortex kinds", "Vortex-Arten"), "catalog"),
        "cables": record(L("Cable kinds", "Kabelarten"), "catalog"), "attractions": record(L("Attraction kinds", "Anziehungsarten"), "catalog"),
    },
    "Puzzle3dCatalogObjectKind": owner_kind(L("Vortices", "Vortices"), "vortices"),
    **catalog_records("Puzzle3d", "Puzzle3dCatalogVortexTemplate", ("Vortex", L("Vortex", "Vortex"), L("Vortex kind", "Vortex-Art"), "vortexKind"), ("Cable", L("Cable", "Kabel"), L("Default cable kind", "Standard-Kabelart"), "defaultCableKind"), ("Attraction", L("Attraction", "Anziehung"), L("Default attraction kind", "Standard-Anziehungsart"), "defaultAttractionKind")),
})

PART5 = puzzle_target("part", L("Part", "Teil"), L("The part this mutation addresses.", "Das Teil, das diese Mutation adressiert."))
VOLUME5 = puzzle_target("targetVolume", L("Target volume", "Zielvolumen"), L("The target volume this mutation addresses.", "Das Zielvolumen, das diese Mutation adressiert."))
FASTENER5 = puzzle_target("fastener", L("Fastener", "Verbinder"), L("The fastener this mutation addresses.", "Der Verbinder, den diese Mutation adressiert."))
GRIP_OWNER5 = puzzle_target("part", L("Part", "Teil"), L("The part that owns the grip.", "Das Teil, dem der Griff gehört."))
GRIP5 = target("grip", L("Grip", "Griff"), L("Id of the grip on its part.", "ID des Griffs an seinem Teil."), order=20)
BOARD = {"config": "gridFactor"}


def board(label, order=None, description=None, group="board"):
    return stepper(label, group, step=1, precision=2, snap_source=BOARD, description=description, order=order)


def board_length(label, order=None):
    return stepper(label, "board", step=1, precision=2, snap_source=BOARD, bounds=POSITIVE, order=order)


leaves("s.puzzle.puzzle5d.mutation.", {
    "add-part-grip": {"partId": GRIP_OWNER5, "grip": record(L("Grip", "Griff"), "record", L("The complete grip record to add.", "Der vollständige hinzuzufügende Griff.")), "index": index()},
    "change-description": {"newDescription": multiline(L("Description", "Beschreibung"), "metadata", L("Free-text scene description.", "Freitext-Beschreibung der Szene."))},
    "change-domain": {"newDomain": DOMAIN},
    "change-fastener-kind": {"id": FASTENER5, "newFastenerKind": KIND_TEXT(L("Fastener kind", "Verbinderart"))},
    "change-part-anchor": {"id": PART5, "newAnchor": select(L("Anchor", "Verankerung"), ANCHOR_OPTIONS, "geometry", ANCHOR_DESCRIPTION, widget="segmented")},
    "change-part-kind": {"id": PART5, "newPartKind": KIND_TEXT(L("Part kind", "Teilart"))},
    "change-part2d-hidden": {"id": PART5, "newHidden": HIDDEN},
    "change-part2d-icon": {"id": PART5, "newIconKind": text(L("Icon", "Symbol"), "appearance")},
    "change-part2d-locked": {"id": PART5, "newLocked": LOCKED},
    "change-part3d-mesh": {"id": PART5, "newMeshUrl": MESH},
    "change-target-volume-hidden": {"id": VOLUME5, "newHidden": HIDDEN},
    "change-target-volume-locked": {"id": VOLUME5, "newLocked": LOCKED},
    "connect-grips": {
        "id": NEW_ID("fastener", "des neuen Verbinders"),
        "source": puzzle_target("grip", L("Source grip", "Quellgriff"), L("Full id (part:grip) of the grip the fastener starts at.", "Vollständige ID (Teil:Griff) des Griffs, an dem der Verbinder beginnt."), order=20),
        "target": puzzle_target("grip", L("Target grip", "Zielgriff"), L("Full id (part:grip) of the grip the fastener ends at.", "Vollständige ID (Teil:Griff) des Griffs, an dem der Verbinder endet."), order=30),
        "fastenerKind": KIND_TEXT(L("Fastener kind", "Verbinderart")),
        **connection("", 50),
    },
    "connect-kind-compatibility": compatibility({"general": L("General", "Allgemein"), "part": L("Part", "Teil"), "fastener": L("Fastener", "Verbinder"), "grip": L("Grip", "Griff"), "rope": L("Rope", "Seil")}),
    "create-part": {"part": record(L("Part", "Teil"), "record", L("The complete part record to add.", "Das vollständige hinzuzufügende Teil.")), "index": index()},
    "create-target-volume": {"targetVolume": record(L("Target volume", "Zielvolumen"), "record", L("The complete target volume record to add.", "Das vollständige hinzuzufügende Zielvolumen.")), "index": index()},
    "delete-part": {"id": PART5},
    "delete-target-volume": {"id": VOLUME5},
    "disconnect-grips": {"id": FASTENER5},
    "disconnect-kind-compatibility": {"source": text(L("Source kind", "Quellart"), "kind"), "target": text(L("Target kind", "Zielart"), "kind")},
    "edit-part2d-text": {"id": PART5, "newText": text(L("Text", "Text"), "appearance")},
    "edit-part3d-label": {"id": PART5, "newLabel": text(L("Label", "Bezeichnung"), "appearance")},
    "move-part2d": {"id": PART5, "newX": board(L("X", "X"), description=L("Absolute final x on the board.", "Absolute x-Endposition auf dem Brett.")), "newY": board(L("Y", "Y"), description=L("Absolute final y on the board.", "Absolute y-Endposition auf dem Brett."))},
    "move-part3d": {"id": PART5, "newOrigin": origin(description=L("Absolute final origin in the world.", "Absoluter Endursprung in der Welt."))},
    "move-target-volume": {"id": VOLUME5, "newOrigin": origin(description=L("Absolute final origin in the world.", "Absoluter Endursprung in der Welt."))},
    "remove-part-grip": {"partId": GRIP_OWNER5, "gripId": GRIP5},
    "rename-puzzle5d": {"newLabel": text(L("Name", "Name"), "metadata")},
    "replace-fastener-geometry": {"id": FASTENER5, **connection("new", 20)},
    "replace-kind-catalogs": {"newCatalogs": record(L("Kind catalogs", "Artenkataloge"), "record", L("Part, grip, fastener and rope kind tables; empty removes them.", "Teil-, Griff-, Verbinder- und Seilartentabellen; leer entfernt sie."))},
    "replace-part-grip": {"partId": GRIP_OWNER5, "gripId": GRIP5, "newGrip": record(L("New grip", "Neuer Griff"), "record", L("The grip record that replaces the addressed one.", "Der Griff, der den adressierten ersetzt."))},
    "replace-part2d-geometry": {
        "id": PART5,
        "newShape": text(L("Shape", "Form"), "board", L("circle or rectangle.", "circle (Kreis) oder rectangle (Rechteck).")),
        "newRadius": board_length(L("Radius", "Radius")), "newWidth": board_length(L("Width", "Breite")), "newHeight": board_length(L("Height", "Höhe")),
    },
    "rotate-part3d": {"id": PART5, "newOrientation": ORIENTATION},
    "rotate-target-volume": {"id": VOLUME5, "newOrientation": ORIENTATION},
    "scale-part3d": {"id": PART5, "newScale": FREE_SCALE},
    "scale-target-volume": {"id": VOLUME5, "newScale": FREE_SCALE},
})
RECORDS.update({
    "Puzzle5dPart": {
        "id": text(L("ID", "ID"), "identity", L("Unique part id in this puzzle.", "Eindeutige Teil-ID in diesem Puzzle.")), "partKind": KIND_TEXT(L("Part kind", "Teilart")),
        "anchor": select(L("Anchor", "Verankerung"), ANCHOR_OPTIONS, "geometry", ANCHOR_DESCRIPTION, widget="segmented"),
        "part2d": record(L("Board", "Brett"), "board", L("How the part appears on the 2D board.", "Wie das Teil auf dem 2D-Brett erscheint.")),
        "part3d": record(L("World", "Welt"), "pose", L("How the part appears in the 3D world.", "Wie das Teil in der 3D-Welt erscheint.")),
        "grips": record(L("Grips", "Griffe"), "topology"),
    },
    "Puzzle5dPart2d": {
        "x": board(L("X", "X")), "y": board(L("Y", "Y")), "shape": text(L("Shape", "Form"), "board", L("circle or rectangle.", "circle (Kreis) oder rectangle (Rechteck).")),
        "radius": board_length(L("Radius", "Radius")), "width": board_length(L("Width", "Breite")), "height": board_length(L("Height", "Höhe")),
        "text": text(L("Text", "Text"), "appearance"), "iconKind": text(L("Icon", "Symbol"), "appearance"), "hidden": HIDDEN, "locked": LOCKED,
    },
    "Puzzle5dPart3d": {"origin": origin(), "meshUrl": MESH, "orientation": ORIENTATION, "scale": FREE_SCALE, "label": text(L("Label", "Bezeichnung"), "appearance")},
    "Puzzle5dGrip": {
        "id": text(L("ID", "ID"), "identity", L("Id of the grip, unique on its part.", "ID des Griffs, eindeutig an seinem Teil.")), "gripKind": KIND_TEXT(L("Grip kind", "Griffart")),
        "grip2d": record(L("Board", "Brett"), "board", L("How the grip appears on the 2D board.", "Wie der Griff auf dem 2D-Brett erscheint.")),
        "grip3d": record(L("World", "Welt"), "pose", L("How the grip appears in the 3D world.", "Wie der Griff in der 3D-Welt erscheint.")),
    },
    "Puzzle5dGrip2d": {
        "angle": angle_rad(L("Angle", "Winkel"), "board", L("Position on the part rim, counter-clockwise.", "Position auf dem Teilrand, gegen den Uhrzeigersinn.")),
        "gripKind": KIND_TEXT(L("Grip kind", "Griffart")), "radius": stepper(L("Radius", "Radius"), "board", step=0.1, precision=2),
    },
    "Puzzle5dGrip3d": {
        "position": origin(L("Position", "Position"), L("Where the grip sits in the part's local frame.", "Lage des Griffs im lokalen System des Teils.")),
        "direction": vector(L("Direction", "Richtung"), "geometry", L("Direction the grip points in.", "Richtung, in die der Griff zeigt."), step=0.1),
        "radius": stepper(L("Radius", "Radius"), "geometry", step=0.01, precision=3, unit="m"), "label": text(L("Label", "Bezeichnung"), "appearance"),
    },
    "Puzzle5dTargetVolume": {
        "id": text(L("ID", "ID"), "identity", L("Unique target volume id in this puzzle.", "Eindeutige Zielvolumen-ID in diesem Puzzle.")),
        "origin": origin(), "orientation": ORIENTATION, "scale": FREE_SCALE, "hidden": HIDDEN, "locked": LOCKED,
    },
    "Puzzle5dKindCatalogs": {
        "parts": record(L("Part kinds", "Teilarten"), "catalog"), "grips": record(L("Grip kinds", "Griffarten"), "catalog"),
        "fasteners": record(L("Fastener kinds", "Verbinderarten"), "catalog"), "ropes": record(L("Rope kinds", "Seilarten"), "catalog"),
    },
    "Puzzle5dCatalogPartKind": owner_kind(L("Grips", "Griffe"), "grips"),
    **catalog_records("Puzzle5d", "Puzzle5dGripTemplate", ("Grip", L("Grip", "Griff"), L("Grip kind", "Griffart"), "gripKind"), ("Rope", L("Rope", "Seil"), L("Default rope kind", "Standard-Seilart"), "defaultRopeKind"), ("Fastener", L("Fastener", "Verbinder"), L("Default fastener kind", "Standard-Verbinderart"), "defaultFastenerKind")),
})
#endregion 🔖️Puzzle

#region 🔖️Shooting
ASSET = target("asset", L("Asset", "Objekt"), L("The asset this mutation addresses.", "Das Objekt, das diese Mutation adressiert."), domain="assets", granularity="asset")
ASSETS = target("asset", L("Assets", "Objekte"), L("Assets to transform; missing ones are skipped.", "Zu transformierende Objekte; fehlende werden übersprungen."), domain="assets", granularity="asset")
SHOT = target("shot", L("Shot", "Aufnahme"), L("The shot this mutation addresses.", "Die Aufnahme, die diese Mutation adressiert."))
SAVED_CAMERA = target("savedCamera", L("Saved camera", "Gespeicherte Kamera"), L("The saved camera this mutation addresses.", "Die gespeicherte Kamera, die diese Mutation adressiert."))
SHOT_FORMAT = L("Image format, e.g. png or svg.", "Bildformat, z. B. png oder svg.")
SHOT_SHAPE = L("Frame shape, e.g. rectangle or ellipse.", "Bildform, z. B. rectangle (Rechteck) oder ellipse (Ellipse).")
ASSET_FORMAT = L("Model format, e.g. glb.", "Modellformat, z. B. glb.")
LIGHT = lambda label: slider(label, "lighting", (0, 4), step=0.01, precision=2, snaps=[0, 0.5, 1, 2, 4], bounds=NON_NEGATIVE)
TO_INDEX = integer(L("New index", "Neuer Index"), "order", L("Position in the list after the move.", "Position in der Liste nach dem Verschieben."))


def camera(prefix=""):
    return {prefix + key: value for key, value in {
        "position": vector(L("Position", "Position"), "camera", L("Where the camera stands.", "Standpunkt der Kamera."), step=0.1),
        "target": vector(L("Look at", "Blickziel"), "camera", L("The point the camera looks at.", "Der Punkt, auf den die Kamera blickt."), step=0.1),
        "zoom": slider(L("Zoom", "Zoom"), "camera", (0.1, 10), snaps=[0.5, 1, 2], scale="log"),
        "fov": slider(L("Field of view", "Sichtfeld"), "camera", (10, 120), step=1, precision=1, unit="deg", snaps=[30, 45, 60, 90], description=L("Vertical opening angle of a perspective camera.", "Vertikaler Öffnungswinkel einer perspektivischen Kamera.")),
        "up": vector(L("Up", "Oben"), "camera", L("Up direction of the camera image.", "Aufwärtsrichtung des Kamerabilds."), step=0.1),
        "projection": text(L("Projection", "Projektion"), "camera", L("perspective or orthographic.", "perspective (perspektivisch) oder orthographic (orthografisch).")),
    }.items()}


SHOT_SELECTION = target("shot", L("Selected shots", "Ausgewählte Aufnahmen"), L("Shots selected in the gallery.", "In der Galerie ausgewählte Aufnahmen."), group="selection")

leaves("app.shooting.shooting.config.mutation.", {
    "replace-config": {
        "config": record(L("Configuration", "Konfiguration"), "record", L("The complete editor configuration.", "Die vollständige Editorkonfiguration.")),
        "config.defaultShotFormat": text(L("Default shot format", "Standard-Aufnahmeformat"), "defaults", SHOT_FORMAT),
        "config.defaultShotShape": text(L("Default shot shape", "Standard-Aufnahmeform"), "defaults", SHOT_SHAPE),
        "config.defaultAssetFormat": text(L("Default asset format", "Standard-Objektformat"), "defaults", ASSET_FORMAT),
        "config.selectedShotIds": SHOT_SELECTION,
        "config.centerModel": toggle(L("Center model", "Modell zentrieren"), "view", L("Keep the active asset centered in the viewport.", "Das aktive Objekt im Ansichtsfenster zentriert halten.")),
        "config.fitRevision": integer(L("Fit revision", "Einpassungsrevision"), "view", L("Counter that re-fits the viewport when it changes.", "Zähler, der das Ansichtsfenster bei Änderung neu einpasst.")),
        "config.cameraDraftLabel": text(L("Camera name draft", "Entwurf Kameraname"), "camera", L("Name typed for the next saved camera.", "Eingegebener Name für die nächste gespeicherte Kamera.")),
        "config.camera": record(L("Viewport camera", "Ansichtskamera"), "camera"),
        **camera("config.camera."),
    },
    "set-camera": {"camera": record(L("Viewport camera", "Ansichtskamera"), "camera"), **camera("camera.")},
    "set-camera-draft-label": {"value": text(L("Camera name draft", "Entwurf Kameraname"), "camera", L("Name typed for the next saved camera.", "Eingegebener Name für die nächste gespeicherte Kamera."))},
    "set-center-model": {"value": toggle(L("Center model", "Modell zentrieren"), "view", L("Keep the active asset centered in the viewport.", "Das aktive Objekt im Ansichtsfenster zentriert halten."))},
    "set-defaults": {"shotFormat": text(L("Shot format", "Aufnahmeformat"), "defaults", SHOT_FORMAT), "shotShape": text(L("Shot shape", "Aufnahmeform"), "defaults", SHOT_SHAPE), "assetFormat": text(L("Asset format", "Objektformat"), "defaults", ASSET_FORMAT)},
    "set-fit-revision": {"value": integer(L("Fit revision", "Einpassungsrevision"), "view", L("Counter that re-fits the viewport when it changes.", "Zähler, der das Ansichtsfenster bei Änderung neu einpasst."))},
    "set-shot-selection": {"shotIds": SHOT_SELECTION},
})
leaves("app.shooting.shooting.presence.mutation.", {
    "replace-presence": {
        "presence": record(L("Presence", "Präsenz"), "record", L("What this participant selects and sees.", "Was diese Person auswählt und sieht.")),
        "presence.selectedShotIds": SHOT_SELECTION,
        "presence.camera": record(L("Viewport camera", "Ansichtskamera"), "camera"),
        **camera("presence.camera."),
    },
})
leaves("s.shooting.shooting.mutation.", {
    "change-asset-url": {"id": ASSET, "newUrl": text(L("URL", "URL"), "source", URL)},
    "change-scene-ambient-intensity": {"newIntensity": LIGHT(L("Ambient intensity", "Umgebungslichtstärke"))},
    "change-scene-material-roughness": {"newRoughness": unit_interval(L("Roughness", "Rauheit"), "material", L("0 is mirror-smooth, 1 is fully matte.", "0 ist spiegelglatt, 1 ist vollständig matt."), bounds=UNIT, percent=False)},
    "change-scene-shadow-enabled": {"newEnabled": toggle(L("Shadows", "Schatten"), "lighting")},
    "change-scene-sun-azimuth": {"newAzimuth": angle_deg(L("Sun azimuth", "Sonnenazimut"), "lighting", L("Compass direction of the sun.", "Himmelsrichtung der Sonne."), soft=(0, 360), snaps=(0, 90, 180, 270, 360))},
    "change-scene-sun-elevation": {"newElevation": angle_deg(L("Sun elevation", "Sonnenhöhe"), "lighting", L("Height of the sun above the horizon.", "Höhe der Sonne über dem Horizont."), soft=(-90, 90), snaps=(-90, -45, 0, 45, 90), bounds={"minimum": -90, "maximum": 90})},
    "change-scene-sun-enabled": {"newEnabled": toggle(L("Sun", "Sonne"), "lighting")},
    "change-scene-sun-intensity": {"newIntensity": LIGHT(L("Sun intensity", "Sonnenlichtstärke"))},
    "change-shot-format": {"id": SHOT, "newFormat": text(L("Format", "Format"), "output", SHOT_FORMAT)},
    "change-shot-height": {"id": SHOT, "newHeight": integer(L("Height", "Höhe"), "output", unit="px")},
    "change-shot-shape": {"id": SHOT, "newShape": text(L("Shape", "Form"), "output", SHOT_SHAPE)},
    "change-shot-width": {"id": SHOT, "newWidth": integer(L("Width", "Breite"), "output", unit="px")},
    "create-asset": {"asset": record(L("Asset", "Objekt"), "record", L("The complete asset record to add.", "Das vollständige hinzuzufügende Objekt.")), "index": index()},
    "create-saved-camera": {"savedCamera": record(L("Saved camera", "Gespeicherte Kamera"), "record", L("The complete saved camera to add.", "Die vollständige hinzuzufügende gespeicherte Kamera.")), "index": index()},
    "create-shot": {"shot": record(L("Shot", "Aufnahme"), "record", L("The complete shot record to add.", "Die vollständige hinzuzufügende Aufnahme.")), "index": index()},
    "delete-asset": {"id": ASSET},
    "delete-saved-camera": {"id": SAVED_CAMERA},
    "delete-shot": {"id": SHOT},
    "drag-assets": {
        "assetIds": ASSETS,
        "dx": stepper(L("Offset X", "Versatz X"), "offset", step=0.1, precision=3),
        "dy": stepper(L("Offset Y", "Versatz Y"), "offset", step=0.1, precision=3),
        "dz": stepper(L("Offset Z", "Versatz Z"), "offset", step=0.1, precision=3),
    },
    "rename-asset": {"id": ASSET, "newName": text(L("Name", "Name"), "identity")},
    "rename-saved-camera": {"id": SAVED_CAMERA, "newLabel": text(L("Name", "Name"), "identity")},
    "rename-shot": {"id": SHOT, "newLabel": text(L("Name", "Name"), "identity")},
    "reorder-assets": {"id": ASSET, "toIndex": TO_INDEX},
    "reorder-saved-cameras": {"id": SAVED_CAMERA, "toIndex": TO_INDEX},
    "reorder-shots": {"id": SHOT, "toIndex": TO_INDEX},
    "replace-saved-camera-view": {"id": SAVED_CAMERA, "newCamera": record(L("Camera", "Kamera"), "camera", L("The view the saved camera now holds.", "Die Ansicht, die die gespeicherte Kamera nun hält."))},
    "replace-shot-camera": {"shotId": SHOT, "newCamera": record(L("Camera", "Kamera"), "camera", L("The view this shot is rendered from.", "Die Ansicht, aus der diese Aufnahme gerendert wird."))},
    "rotate-assets": {
        "assetIds": ASSETS,
        "ax": stepper(L("Axis X", "Achse X"), "axis", step=0.1, precision=3, description=L("Rotation axis; its length does not matter.", "Drehachse; ihre Länge spielt keine Rolle.")),
        "ay": stepper(L("Axis Y", "Achse Y"), "axis", step=0.1, precision=3),
        "az": stepper(L("Axis Z", "Achse Z"), "axis", step=0.1, precision=3),
        "angle": angle_rad(L("Angle", "Winkel"), "rotation", L("Right-handed rotation about the axis.", "Rechtshändige Drehung um die Achse.")),
    },
    "scale-assets": {
        "assetIds": ASSETS,
        "sx": factor(L("Factor X", "Faktor X"), "scale", L("Multiplies the current scale along x.", "Multipliziert die aktuelle Skalierung entlang x."), bounds=POSITIVE),
        "sy": factor(L("Factor Y", "Faktor Y"), "scale", L("Multiplies the current scale along y.", "Multipliziert die aktuelle Skalierung entlang y."), bounds=POSITIVE),
        "sz": factor(L("Factor Z", "Faktor Z"), "scale", L("Multiplies the current scale along z.", "Multipliziert die aktuelle Skalierung entlang z."), bounds=POSITIVE),
    },
    "set-active-asset": {"assetId": target("asset", L("Active asset", "Aktives Objekt"), L("The asset shown in the viewport.", "Das im Ansichtsfenster gezeigte Objekt."), domain="assets", granularity="asset")},
    "set-active-shot": {"shotId": target("shot", L("Active shot", "Aktive Aufnahme"), L("The shot being framed.", "Die Aufnahme, die gerade eingerichtet wird."))},
})
RECORDS.update({
    "ShootingAsset": {
        "id": text(L("ID", "ID"), "identity", L("Unique asset id in this scene.", "Eindeutige Objekt-ID in dieser Szene.")), "name": text(L("Name", "Name"), "identity"),
        "url": text(L("URL", "URL"), "source", URL), "format": text(L("Format", "Format"), "source", ASSET_FORMAT),
        "origin": vector(L("Origin", "Ursprung"), "pose", step=0.1), "orientation": ORIENTATION,
    },
    "ShootingSavedCamera": {
        "id": text(L("ID", "ID"), "identity", L("Unique saved camera id in this scene.", "Eindeutige ID der gespeicherten Kamera in dieser Szene.")),
        "label": text(L("Name", "Name"), "identity"), "camera": record(L("Camera", "Kamera"), "camera"),
    },
    "ShootingCamera": camera(),
    "ShootingShot": {
        "id": text(L("ID", "ID"), "identity", L("Unique shot id in this scene.", "Eindeutige Aufnahme-ID in dieser Szene.")), "label": text(L("Name", "Name"), "identity"),
        "width": integer(L("Width", "Breite"), "output", unit="px"), "height": integer(L("Height", "Höhe"), "output", unit="px"),
        "format": text(L("Format", "Format"), "output", SHOT_FORMAT), "shape": text(L("Shape", "Form"), "output", SHOT_SHAPE),
        "background": text(L("Background", "Hintergrund"), "output", HEX),
        "cameraId": target("savedCamera", L("Camera", "Kamera"), L("Saved camera this shot is rendered from; empty uses the viewport camera.", "Gespeicherte Kamera, aus der diese Aufnahme gerendert wird; leer nutzt die Ansichtskamera."), group="camera"),
    },
})
#endregion 🔖️Shooting

#region 🔖️Cad
PANE = select(L("Model", "Modell"), {"shape": L("Shape", "Form"), "building": L("Building", "Gebäude"), "energy": L("Energy", "Energie"), "structure-classic": L("Structure (classic)", "Tragwerk (klassisch)")}, "target", L("The model pane the objects live in.", "Der Modellbereich, in dem die Objekte liegen."), widget="segmented", order=5)
CAD_OBJECT = target("object", L("Object", "Objekt"), L("The object this mutation addresses.", "Das Objekt, das diese Mutation adressiert."), domain="cad", granularity="object")
CAD_NODE = target("node", L("Node", "Knoten"), L("The node this mutation addresses.", "Der Knoten, den diese Mutation adressiert."))
MODEL_DEFINITION = text(L("Model definition", "Modelldefinition"), "target", L("Id of the model the reference belongs to, e.g. aec.building.", "ID des Modells, zu dem die Referenz gehört, z. B. aec.building."), order=10)
CAD_REFERENCE = target("reference", L("Reference", "Referenz"), L("The reference image this mutation addresses.", "Das Referenzbild, das diese Mutation adressiert."), order=20)
CHILD_ID = text(L("Child ID", "Kind-ID"), "identity", L("Id of the child model document this slot holds.", "ID des Kind-Modelldokuments in diesem Platz."))
CAD_WIDTH = stepper(L("Width", "Breite"), "geometry", step=0.1, precision=3, description=L("Width of the reference plane in the world.", "Breite der Referenzebene in der Welt."), bounds=POSITIVE)
CAD_OPACITY = unit_interval(L("Opacity", "Deckkraft"), "appearance")
CAD_SCALE = slider(L("Scale", "Skalierung"), "pose", (0.1, 10), scale="log", snaps=[0.25, 0.5, 1, 2, 4], description=L("Uniform factor applied to the image plane.", "Einheitlicher Faktor für die Bildebene."))
PLACEMENTS = lambda description: record(L("Placements", "Platzierungen"), "placement", description)

leaves("s.cad.aec-building.mutation.", {
    "create-building-storey": {
        "storeyId": text(L("Storey ID", "Geschoss-ID"), "identity", L("Id of the new storey.", "ID des neuen Geschosses.")),
        "levelIndex": integer(L("Level", "Geschossebene"), "identity", L("0 is the ground floor; negative levels lie below ground.", "0 ist das Erdgeschoss; negative Ebenen liegen unter Gelände.")),
        "storeyName": text(L("Storey name", "Geschossbezeichnung"), "identity"),
    },
})
leaves("s.cad.cad.mutation.", {
    "change-reference-hidden": {"modelDefinitionId": MODEL_DEFINITION, "referenceId": CAD_REFERENCE, "newHidden": HIDDEN},
    "change-reference-locked": {"modelDefinitionId": MODEL_DEFINITION, "referenceId": CAD_REFERENCE, "newLocked": LOCKED},
    "change-reference-width": {"modelDefinitionId": MODEL_DEFINITION, "referenceId": CAD_REFERENCE, "newWidthWorld": CAD_WIDTH},
    "create-building-model": {"childId": CHILD_ID},
    "create-drawing": {"childId": CHILD_ID, "target": text(L("Artifact", "Artefakt"), "source", L("URI of the drawing artifact the new child refers to.", "URI des Zeichnungsartefakts, auf das das neue Kind verweist."))},
    "create-energy-model": {"childId": CHILD_ID},
    "create-node": {"node": record(L("Node", "Knoten"), "record", L("The complete node record to add.", "Der vollständige hinzuzufügende Knoten."))},
    "create-object": {
        "pane": PANE, "index": index(), "object": record(L("Object", "Objekt"), "record", L("The authored fields of the new object.", "Die festgelegten Felder des neuen Objekts.")),
        "primitives": record(L("Primitives", "Grundkörper"), "record", L("Geometry slots of the object; empty uses its solid.", "Geometrieplätze des Objekts; leer nutzt seinen Volumenkörper.")),
    },
    "create-shape-model": {"childId": CHILD_ID},
    "create-structure-classic-model": {"childId": CHILD_ID},
    "delete-drawing": {"childId": text(L("Drawing", "Zeichnung"), "target", L("Child id of the drawing to remove.", "Kind-ID der zu entfernenden Zeichnung."))},
    "delete-node": {"nodeId": CAD_NODE},
    "delete-object": {"pane": PANE, "objectId": CAD_OBJECT},
    "move-objects": {"pane": PANE, "placements": PLACEMENTS(L("One absolute origin per moved object.", "Ein absoluter Ursprung je verschobenem Objekt."))},
    "move-reference": {"modelDefinitionId": MODEL_DEFINITION, "referenceId": CAD_REFERENCE, "newOrigin": vector(L("Origin", "Ursprung"), "pose", L("Absolute final origin in the world.", "Absoluter Endursprung in der Welt."), step=0.1)},
    "rename-node": {"nodeId": CAD_NODE, "newLabel": text(L("Label", "Bezeichnung"), "identity")},
    "replace-reference-media": {
        "modelDefinitionId": MODEL_DEFINITION, "referenceId": CAD_REFERENCE,
        "newSourceUrl": text(L("Source URL", "Quell-URL"), "source", URL), "newMediaKind": text(L("Media kind", "Medienart"), "source", L("For example image or drawing.", "Zum Beispiel image (Bild) oder drawing (Zeichnung).")),
        "newOrientation": ORIENTATION, "newScale": CAD_SCALE, "newOpacity": CAD_OPACITY,
    },
    "replace-references": {"modelDefinitionId": MODEL_DEFINITION, "references": record(L("References", "Referenzen"), "record", L("The complete reference list of the model.", "Die vollständige Referenzliste des Modells."))},
    "rotate-objects": {"pane": PANE, "placements": PLACEMENTS(L("One absolute orientation per rotated object.", "Eine absolute Orientierung je gedrehtem Objekt."))},
    "scale-objects": {"pane": PANE, "placements": PLACEMENTS(L("One absolute scale per scaled object.", "Eine absolute Skalierung je skaliertem Objekt."))},
})
RECORDS.update({
    "CadNode": {"id": text(L("ID", "ID"), "identity", L("Unique node id in this document.", "Eindeutige Knoten-ID in diesem Dokument.")), "label": text(L("Label", "Bezeichnung"), "identity"), "kind": text(L("Kind", "Art"), "kind")},
    "CadObjectSpec": {
        "id": text(L("ID", "ID"), "identity", L("Unique object id in this pane.", "Eindeutige Objekt-ID in diesem Bereich.")), "label": text(L("Label", "Bezeichnung"), "identity"),
        "typology": text(L("Typology", "Typologie"), "kind", L("Building element typology, e.g. building.building.wall.", "Bauteiltypologie, z. B. building.building.wall.")),
        "visible": toggle(L("Visible", "Sichtbar"), "state"), "locked": LOCKED,
        "origin": vector(L("Origin", "Ursprung"), "pose", step=0.1), "orientation": ORIENTATION,
        "scale": vector(L("Scale", "Skalierung"), "pose", L("Factor per axis (x, y, z).", "Faktor je Achse (x, y, z)."), step=0.1),
        "meshUrl": MESH, "extent": vector(L("Extent", "Ausdehnung"), "geometry", L("Size of the bounding box along x, y and z.", "Größe des Hüllquaders entlang x, y und z."), step=0.1),
        "solidHandle": text(L("Solid", "Volumenkörper"), "source", L("Handle of the boundary-representation solid.", "Handle des Volumenkörpers in Begrenzungsflächendarstellung.")),
    },
    "CadObjectPrimitive": {"slot": text(L("Slot", "Platz"), "identity"), "primitiveId": text(L("Primitive ID", "Grundkörper-ID"), "identity"), "kind": text(L("Kind", "Art"), "kind")},
    "CadObjectOrigin": {"objectId": CAD_OBJECT, "newOrigin": vector(L("Origin", "Ursprung"), "pose", L("Absolute final origin.", "Absoluter Endursprung."), step=0.1)},
    "CadObjectOrientation": {"objectId": CAD_OBJECT, "newOrientation": ORIENTATION},
    "CadObjectScale": {"objectId": CAD_OBJECT, "newScale": vector(L("Scale", "Skalierung"), "pose", L("Absolute final factor per axis; zero is refused.", "Absoluter End-Faktor je Achse; null wird abgelehnt."), step=0.1)},
    "CadReference": {
        "id": text(L("ID", "ID"), "identity", L("Unique reference id in this model.", "Eindeutige Referenz-ID in diesem Modell.")),
        "sourceUrl": text(L("Source URL", "Quell-URL"), "source", URL), "mediaKind": text(L("Media kind", "Medienart"), "source", L("For example image or drawing.", "Zum Beispiel image (Bild) oder drawing (Zeichnung).")),
        "origin": vector(L("Origin", "Ursprung"), "pose", step=0.1), "orientation": ORIENTATION, "scale": CAD_SCALE, "widthWorld": CAD_WIDTH,
        "hidden": HIDDEN, "locked": LOCKED, "opacity": CAD_OPACITY,
    },
})
#endregion 🔖️Cad

#region 🔖️Procedural
GENERATION = target("generation", L("Generation", "Generierung"), L("The generation this mutation addresses.", "Die Generierung, die diese Mutation adressiert."))
WIDGET = target("widget", L("Widget", "Baustein"), L("The graph widget this mutation addresses.", "Der Graph-Baustein, den diese Mutation adressiert."), domain="graph", granularity="node")
SYNAPSE = target("synapse", L("Synapse", "Synapse"), L("The synapse this mutation addresses.", "Die Synapse, die diese Mutation adressiert."), domain="graph", granularity="edge")
WIDGET_KINDS = {"Neuron": L("Neuron", "Neuron"), "InputSlider": L("Slider input", "Schiebereglereingabe"), "InputNote": L("Note input", "Notizeingabe"), "InputImage": L("Image input", "Bildeingabe"), "Variable": L("Variable", "Variable"), "OutputPreview": L("Preview output", "Vorschauausgabe"), "OutputAction": L("Action output", "Aktionsausgabe"), "OutputExport": L("Export output", "Exportausgabe"), "Cluster": L("Cluster", "Cluster")}
WIDGET_RECORD = record(L("Widget", "Baustein"), "record", L("The widget kind to place.", "Die zu platzierende Bausteinart."))
LOD_MODE = text(L("Level of detail", "Detailstufe"), "view", L("coarse, medium or fine.", "coarse (grob), medium (mittel) oder fine (fein)."))
SHOW_MODE = text(L("Display mode", "Darstellungsmodus"), "view", L("shaded, shaded+edges, wireframe or points.", "shaded (schattiert), shaded+edges (schattiert mit Kanten), wireframe (Drahtgitter) oder points (Punkte)."))
SUN_JSON = multiline(L("Sun", "Sonne"), "lighting", L("Sun settings as JSON text.", "Sonneneinstellungen als JSON-Text."))
PREVIEW_TEXT = multiline(L("Preview text", "Vorschautext"), "preview", L("Text of the last generation preview; empty clears it.", "Text der letzten Generierungsvorschau; leer löscht ihn."))
GRAPH_CAMERA = record(L("Graph camera", "Graphkamera"), "camera", L("Pan and zoom of the graph canvas.", "Verschiebung und Zoom der Graphfläche."))
PREVIEW_CAMERA = record(L("Preview camera", "Vorschaukamera"), "camera", L("Camera of the 3D preview.", "Kamera der 3D-Vorschau."))
PREVIEW_CAMERA_FIELDS = {
    "position": vector(L("Position", "Position"), "camera", L("Where the camera stands.", "Standpunkt der Kamera."), step=0.1),
    "target": vector(L("Look at", "Blickziel"), "camera", L("The point the camera looks at.", "Der Punkt, auf den die Kamera blickt."), step=0.1),
    "fov": slider(L("Field of view", "Sichtfeld"), "camera", (10, 120), step=1, precision=1, unit="deg", snaps=[30, 45, 60, 90], description=L("Vertical opening angle of the camera.", "Vertikaler Öffnungswinkel der Kamera.")),
}
QUESTION = text(L("Question", "Frage"), "target", L("Id of the question in the generation schema.", "ID der Frage im Generierungsschema."), order=20)
SCHEMA_TEXT = multiline(L("Schema", "Schema"), "schema", L("Question schema of the generations, as JSON text.", "Fragenschema der Generierungen als JSON-Text."))


def generation_leaves(value_key, schema_key, rename_key):
    return {
        "change-generation-value": {"id": GENERATION, "questionId": QUESTION, value_key: record(L("Answer", "Antwort"), "value", L("The new answer; any JSON value the question accepts.", "Die neue Antwort; jeder JSON-Wert, den die Frage annimmt."))},
        "change-schema": {schema_key: SCHEMA_TEXT},
        "connect-synapse": {"index": index(), "synapse": record(L("Synapse", "Synapse"), "record", L("The complete synapse to add.", "Die vollständige hinzuzufügende Synapse."))},
        "create-generation": {"generation": record(L("Generation", "Generierung"), "record", L("The complete generation to add.", "Die vollständige hinzuzufügende Generierung."))},
        "create-widget": {"index": index(), "widget": WIDGET_RECORD},
        "delete-generation": {"id": GENERATION},
        "delete-widget": {"id": WIDGET},
        "disconnect-synapse": {"id": SYNAPSE},
        "move-widget": {"id": WIDGET, "layout": record(L("Position", "Position"), "layout", L("Where the widget sits on the graph canvas.", "Lage des Bausteins auf der Graphfläche."))},
        "rename-generation": {"id": GENERATION, rename_key: text(L("Name", "Name"), "identity")},
        "update-camera": {"camera": GRAPH_CAMERA},
    }


generation2d = generation_leaves("value", "schema", "name")
generation2d.update({"clear-widget-layout": {"id": WIDGET}, "replace-synapse": {"synapse": record(L("Synapse", "Synapse"), "record", L("The synapse that replaces the one with the same id.", "Die Synapse, die die gleichnamige ersetzt."))}, "replace-widget": {"widget": WIDGET_RECORD}})
generation3d = generation_leaves("newValue", "newSchema", "newName")
generation3d.update({"delete-widget-position": {"id": WIDGET}, "update-synapse": {"synapse": record(L("Synapse", "Synapse"), "record", L("The synapse that replaces the one with the same id.", "Die Synapse, die die gleichnamige ersetzt."))}, "update-widget": {"widget": WIDGET_RECORD}})
leaves("s.procedural.generation2d.mutation.", generation2d)
leaves("s.procedural.generation3d.mutation.", generation3d)
for prefix in ("s.procedural.generation2d.mutation.", "s.procedural.generation3d.mutation."):
    for kind in ("create-widget", "replace-widget", "update-widget"):
        if prefix + kind in LEAVES:
            VARIANTS[prefix + kind] = {f"widget|{constant}": label for constant, label in WIDGET_KINDS.items()}
leaves("app.procedural.2d.transient.mutation.", {"set-generation-preview": {"previewText": PREVIEW_TEXT}})
leaves("app.procedural.3d.config.mutation.", {
    "set-camera": {"camera": GRAPH_CAMERA},
    "set-lod-mode": {"value": LOD_MODE},
    "set-preview-camera": {"camera": PREVIEW_CAMERA},
    "set-selected-generation": {"selectedGenerationId": target("generation", L("Selected generation", "Ausgewählte Generierung"), L("The generation shown in the preview; empty selects none.", "Die in der Vorschau gezeigte Generierung; leer wählt keine."))},
    "set-show-mode": {"value": SHOW_MODE},
    "set-snapshot": {"config": record(L("Configuration", "Konfiguration"), "record", L("The complete editor configuration.", "Die vollständige Editorkonfiguration."))},
    "set-sun": {"json": SUN_JSON},
})
leaves("app.procedural.3d.transient.mutation.", {"set-generation-preview": {"previewText": PREVIEW_TEXT}})
leaves("app.procedural.3d.viewer.config.mutation.", {
    "set-active-example": {"value": text(L("Example", "Beispiel"), "view", L("Id of the bundled example on show; empty shows none.", "ID des gezeigten mitgelieferten Beispiels; leer zeigt keines."))},
    "set-lod-mode": {"value": LOD_MODE},
    "set-preview-camera": {"camera": PREVIEW_CAMERA},
    "set-show-mode": {"value": SHOW_MODE},
    "set-sun": {"json": SUN_JSON},
})
leaves("app.procedural.3d.viewer.presence.mutation.", {"set-preview-camera": {"camera": PREVIEW_CAMERA}, "set-show-mode": {"value": SHOW_MODE}})
leaves("app.procedural.3d.viewer.transient.mutation.", {"set-preview-eval": {"evalText": multiline(L("Evaluation text", "Auswertungstext"), "preview", L("Text of the last preview evaluation; empty clears it.", "Text der letzten Vorschauauswertung; leer löscht ihn."))}})
RECORDS.update({
    "CameraJson": {
        "x": stepper(L("Pan X", "Verschiebung X"), "camera", step=1, precision=1), "y": stepper(L("Pan Y", "Verschiebung Y"), "camera", step=1, precision=1),
        "zoom": slider(L("Zoom", "Zoom"), "camera", (0.1, 10), snaps=[0.5, 1, 2], scale="log"),
    },
    "Generation3dPreviewCamera": PREVIEW_CAMERA_FIELDS,
    "Generation3dViewCamera": PREVIEW_CAMERA_FIELDS,
    "Generation3dConfig": {
        "lodMode": LOD_MODE, "showMode": SHOW_MODE, "camera": GRAPH_CAMERA, "previewCamera": PREVIEW_CAMERA, "sunJson": SUN_JSON,
        "selectedGenerationId": target("generation", L("Selected generation", "Ausgewählte Generierung"), L("The generation shown in the preview.", "Die in der Vorschau gezeigte Generierung.")),
    },
    "SynapseSpec": {
        "id": text(L("ID", "ID"), "identity", L("Unique synapse id in this graph.", "Eindeutige Synapsen-ID in diesem Graphen.")),
        "from": target("widget", L("From widget", "Von Baustein"), L("The widget the synapse starts at.", "Der Baustein, an dem die Synapse beginnt."), domain="graph", granularity="node"),
        "to": target("widget", L("To widget", "Zu Baustein"), L("The widget the synapse ends at.", "Der Baustein, an dem die Synapse endet."), domain="graph", granularity="node"),
        "fromPort": text(L("From port", "Von Anschluss"), "target", L("Output port on the start widget.", "Ausgang am Startbaustein.")),
        "toPort": text(L("To port", "Zu Anschluss"), "target", L("Input port on the end widget.", "Eingang am Zielbaustein.")),
    },
    "FormGeneration": {
        "id": text(L("ID", "ID"), "identity", L("Unique generation id.", "Eindeutige Generierungs-ID.")), "name": text(L("Name", "Name"), "identity"),
        "valuesJson": multiline(L("Answers", "Antworten"), "value", L("Answers per question, as JSON text.", "Antworten je Frage als JSON-Text.")),
    },
    "WidgetLayout": {"x": stepper(L("X", "X"), "layout", step=1, precision=1), "y": stepper(L("Y", "Y"), "layout", step=1, precision=1)},
})
EXTRA_RECORDS = [(f"{PLUGINS}/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🔣️.json", "/$defs/Generation3dViewCamera")]
#endregion 🔖️Procedural

#region 🔖️Note
BLOCK = target("block", L("Block", "Block"), L("The block this mutation addresses.", "Der Block, den diese Mutation adressiert."), domain="blocks", granularity="block")
TABLE_BLOCK = target("block", L("Table", "Tabelle"), L("The table block this mutation addresses.", "Der Tabellenblock, den diese Mutation adressiert."), domain="blocks", granularity="block")
CONTAINER = target("block", L("Container", "Container"), L("Group block to place the block in; the page when omitted.", "Gruppenblock, in den der Block kommt; ohne Angabe die Seite."), domain="blocks", granularity="block", group="placement")
NOTE_GRID = {"snapshot": "/snapGridSpacing"}
NOTE_ASSET = target("asset", L("Asset", "Asset"), L("Key of the image asset this mutation addresses.", "Schlüssel des Bild-Assets, das diese Mutation adressiert."))
NOTE_BLOCK_RECORD = lambda description: record(L("Block", "Block"), "record", description)


def canvas(label, order=None, description=None, group="position"):
    return stepper(label, group, step=1, precision=1, snap_source=NOTE_GRID, description=description, order=order)


def extent(label, bounds=None):
    return stepper(label, "size", step=1, precision=1, snap_source=NOTE_GRID, bounds=bounds)


leaves("app.note.note.presence.mutation.", {
    "replace-presence": {
        "presence": record(L("Presence", "Präsenz"), "record", L("Where this participant looks on the canvas.", "Wohin diese Person auf der Fläche blickt.")),
        "presence.cameraX": stepper(L("Pan X", "Verschiebung X"), "camera", step=1, precision=1), "presence.cameraY": stepper(L("Pan Y", "Verschiebung Y"), "camera", step=1, precision=1),
        "presence.cameraZoom": slider(L("Zoom", "Zoom"), "camera", (0.1, 10), snaps=[0.5, 1, 2], scale="log"),
    },
})
leaves("s.note.note.1.asset.mutation.", {
    "create-asset": {"key": text(L("Key", "Schlüssel"), "identity", L("Unique key the blocks use to show this asset.", "Eindeutiger Schlüssel, mit dem Blöcke dieses Asset zeigen.")), "asset": record(L("Image", "Bild"), "record", L("The image payload to store.", "Die zu speichernden Bilddaten."))},
    "delete-asset": {"key": NOTE_ASSET},
    "replace-asset-payload": {"key": NOTE_ASSET, "newAsset": record(L("Image", "Bild"), "record", L("The image payload that replaces the stored one.", "Die Bilddaten, die die gespeicherten ersetzen."))},
})
leaves("s.note.note.1.block.mutation.", {
    "change-block-font-size": {"id": BLOCK, "newFontSize": stepper(L("Font size", "Schriftgröße"), "text", step=1, precision=1, soft=(6, 96), bounds=POSITIVE)},
    "change-block-locked": {"id": BLOCK, "newLocked": LOCKED},
    "change-block-visible": {"id": BLOCK, "newVisible": toggle(L("Visible", "Sichtbar"), "state")},
    "create-block": {"block": NOTE_BLOCK_RECORD(L("The complete block to add.", "Der vollständige hinzuzufügende Block.")), "parentId": CONTAINER, "index": index()},
    "delete-block": {"id": BLOCK},
    "delete-blocks": {"ids": target("block", L("Blocks", "Blöcke"), L("Blocks to delete.", "Zu löschende Blöcke."), domain="blocks", granularity="block")},
    "drag-blocks": {
        "ids": target("block", L("Blocks", "Blöcke"), L("Blocks to drag; missing ones are skipped.", "Zu ziehende Blöcke; fehlende werden übersprungen."), domain="blocks", granularity="block"),
        "dx": canvas(L("Offset X", "Versatz X"), group="offset"), "dy": canvas(L("Offset Y", "Versatz Y"), group="offset"),
    },
    "duplicate-block": {"sourceId": target("block", L("Source block", "Quellblock"), L("The block to copy.", "Der zu kopierende Block."), domain="blocks", granularity="block"), "block": NOTE_BLOCK_RECORD(L("The copy with its new id.", "Die Kopie mit ihrer neuen ID."))},
    "duplicate-blocks": {
        "sourceIds": target("block", L("Source blocks", "Quellblöcke"), L("The blocks to copy.", "Die zu kopierenden Blöcke."), domain="blocks", granularity="block"),
        "blocks": record(L("Copies", "Kopien"), "record", L("One copy per source block, with new ids.", "Eine Kopie je Quellblock, mit neuen IDs.")),
    },
    "move-block": {"id": BLOCK, "newX": canvas(L("X", "X"), description=L("Absolute final x on the canvas.", "Absolute x-Endposition auf der Fläche.")), "newY": canvas(L("Y", "Y"), description=L("Absolute final y on the canvas.", "Absolute y-Endposition auf der Fläche."))},
    "move-block-to-container": {"id": BLOCK, "newParentId": CONTAINER, "index": index()},
    "rename-block": {"id": BLOCK, "newName": text(L("Name", "Name"), "identity")},
    "resize-block": {"id": BLOCK, "newWidth": extent(L("Width", "Breite"), POSITIVE), "newHeight": extent(L("Height", "Höhe"), POSITIVE)},
})
leaves("s.note.note.1.canvas.mutation.", {
    "change-grid-opacity": {"newOpacity": unit_interval(L("Grid opacity", "Rasterdeckkraft"), "grid", bounds=UNIT)},
    "change-grid-spacing": {"newSpacing": stepper(L("Grid spacing", "Rasterabstand"), "grid", step=1, precision=1, soft=(4, 200), bounds=POSITIVE)},
    "change-grid-subdivisions": {"newSubdivisions": stepper(L("Grid subdivisions", "Rasterunterteilungen"), "grid", step=1, precision=0, soft=(1, 16), bounds={"minimum": 1}, description=L("Minor lines between two major grid lines.", "Nebenlinien zwischen zwei Hauptrasterlinien."))},
    "change-grid-visible": {"newVisible": toggle(L("Grid visible", "Raster sichtbar"), "grid")},
    "change-snap-enabled": {"newEnabled": toggle(L("Snap to grid", "Am Raster fangen"), "snap")},
    "change-snap-grid-spacing": {"newSpacing": stepper(L("Snap spacing", "Fangabstand"), "snap", step=1, precision=1, soft=(1, 200), bounds=POSITIVE)},
})
leaves("s.note.note.1.document.mutation.", {"rename-note": {"newTitle": text(L("Title", "Titel"), "identity")}})
leaves("s.note.note.1.ink.mutation.", {
    "change-block-ink-width": {"id": BLOCK, "newStrokeWidth": stepper(L("Stroke width", "Strichstärke"), "stroke", step=0.5, precision=1, soft=(0.5, 32))},
    "change-eraser-radius": {"newRadius": stepper(L("Eraser radius", "Radiergummiradius"), "tool", step=1, precision=1, soft=(1, 100), bounds=POSITIVE)},
    "change-pencil-width": {"newWidth": stepper(L("Pencil width", "Stiftbreite"), "tool", step=0.5, precision=1, soft=(0.5, 32), bounds=POSITIVE)},
    "edit-block-ink-stroke": {
        "id": BLOCK,
        "newPoints": record(L("Points", "Punkte"), "stroke", L("Stroke points as (x, y) pairs inside the block.", "Strichpunkte als (x, y)-Paare im Block.")),
        "newX": canvas(L("X", "X")), "newY": canvas(L("Y", "Y")), "newWidth": extent(L("Width", "Breite")), "newHeight": extent(L("Height", "Höhe")),
    },
})
leaves("s.note.note.1.math.mutation.", {"edit-block-math": {"id": BLOCK, "newTex": multiline(L("Formula", "Formel"), "content", L("TeX source of the formula.", "TeX-Quelltext der Formel."))}})
leaves("s.note.note.1.table.mutation.", {kind: {"id": TABLE_BLOCK} for kind in ("insert-table-column", "insert-table-row", "remove-table-column", "remove-table-row")})
leaves("s.note.note.1.text.mutation.", {"edit-block-text": {"id": BLOCK, "newParagraphs": record(L("Paragraphs", "Absätze"), "content", L("The complete new text, paragraph by paragraph.", "Der vollständige neue Text, Absatz für Absatz."))}})
RECORDS.update({
    "NoteBlockNode": {"kind": text(L("Kind", "Art"), "kind", L("text, image, table, math, stroke or group.", "text (Text), image (Bild), table (Tabelle), math (Formel), stroke (Strich) oder group (Gruppe)."))},
    "NoteImageAsset": {
        "mime": text(L("Media type", "Medientyp"), "source", L("For example image/png.", "Zum Beispiel image/png.")), "data": multiline(L("Data", "Daten"), "source", L("Base64-encoded image bytes.", "Base64-kodierte Bildbytes.")),
        "width": stepper(L("Width", "Breite"), "size", step=1, precision=0, unit="px"), "height": stepper(L("Height", "Höhe"), "size", step=1, precision=0, unit="px"),
    },
    "NoteTextParagraph": {"runs": record(L("Runs", "Abschnitte"), "content", L("Stretches of text sharing one format.", "Textstücke mit gleicher Formatierung."))},
    "NoteTextRun": {
        "text": text(L("Text", "Text"), "content"), "bold": toggle(L("Bold", "Fett"), "format"), "italic": toggle(L("Italic", "Kursiv"), "format"),
        "underline": toggle(L("Underline", "Unterstrichen"), "format"), "link": text(L("Link", "Link"), "format", L("Target URL of the run.", "Ziel-URL des Abschnitts.")),
    },
})
#endregion 🔖️Note

#region 🔖️Forms
STEP = target("step", L("Step", "Schritt"), L("The form step this mutation addresses.", "Der Formularschritt, den diese Mutation adressiert."), domain="fields", granularity="section")
QUESTION_TARGET = target("question", L("Question", "Frage"), L("The question this mutation addresses.", "Die Frage, die diese Mutation adressiert."), domain="fields", granularity="field", order=20)
CONTRIBUTIONS = multiline(L("Contributions", "Beiträge"), "plugins", L("Host-declared plugin contributions, as JSON text.", "Vom Host deklarierte Plugin-Beiträge als JSON-Text."))
FORM_INDEX = integer(L("Index", "Index"), "order", L("Position in the list; the end when omitted.", "Position in der Liste; ohne Angabe am Ende."))
EXPRESSION = lambda label, description=None: record(label, "condition", description)

leaves("app.forms.forms.config.mutation.", {
    "replace-config": {"config": record(L("Configuration", "Konfiguration"), "record", L("The complete editor configuration.", "Die vollständige Editorkonfiguration.")), "config.contributionsJson": CONTRIBUTIONS},
    "set-contributions": {"json": CONTRIBUTIONS},
})
leaves("s.forms.forms.mutation.", {
    "change-form-title": {"new_title": text(L("Title", "Titel"), "identity", L("Title of the form; empty removes it.", "Titel des Formulars; leer entfernt ihn."))},
    "change-step-description": {"id": STEP, "new_description": multiline(L("Description", "Beschreibung"), "content", L("Introduction shown above the step; empty removes it.", "Einleitung über dem Schritt; leer entfernt sie."))},
    "commit-response": {"response": record(L("Response", "Antwort"), "record", L("The submitted response to store.", "Die zu speichernde übermittelte Antwort.")), "index": FORM_INDEX},
    "create-block": {"step_id": STEP, "block": record(L("Question", "Frage"), "record", L("The complete question to add.", "Die vollständige hinzuzufügende Frage.")), "index": FORM_INDEX},
    "create-step": {"step": record(L("Step", "Schritt"), "record", L("The complete step to add.", "Der vollständige hinzuzufügende Schritt.")), "index": FORM_INDEX},
    "delete-block": {"step_id": STEP, "id": QUESTION_TARGET},
    "delete-step": {"id": STEP},
    "discard-response": {"id": target("response", L("Response", "Antwort"), L("The stored response to discard.", "Die zu verwerfende gespeicherte Antwort."))},
    "move-block-to-step": {
        "step_id": STEP, "block_id": QUESTION_TARGET,
        "to_step_id": target("step", L("Target step", "Zielschritt"), L("The step the question moves to.", "Der Schritt, in den die Frage verschoben wird."), domain="fields", granularity="section", group="placement"),
        "index": integer(L("Index", "Index"), "placement", L("Position in the target step.", "Position im Zielschritt.")),
    },
    "rename-step": {"id": STEP, "new_title": text(L("Title", "Titel"), "identity")},
    "reorder-step": {"id": STEP, "to_index": TO_INDEX},
    "replace-block": {"step_id": STEP, "block": record(L("Question", "Frage"), "record", L("The question that replaces the one with the same id.", "Die Frage, die die gleichnamige ersetzt."))},
})
VARIANTS["Expression"] = {"|const": L("Constant", "Konstante"), "|var": L("Answer of", "Antwort von"), "|eq": L("Equals", "Gleich"), "|truthy": L("Is set", "Ist gesetzt"), "|and": L("All of", "Alle von"), "|or": L("Any of", "Eines von")}
RECORDS.update({
    "Step": {
        "id": text(L("ID", "ID"), "identity", L("Unique step id in this form.", "Eindeutige Schritt-ID in diesem Formular.")), "title": text(L("Title", "Titel"), "identity"),
        "description": multiline(L("Description", "Beschreibung"), "content"), "blocks": record(L("Questions", "Fragen"), "content"),
    },
    "Question": {
        "id": text(L("ID", "ID"), "identity", L("Unique question id in this form.", "Eindeutige Fragen-ID in diesem Formular.")),
        "kind": text(L("Question type", "Fragetyp"), "kind", L("For example text or number.", "Zum Beispiel text (Text) oder number (Zahl).")),
        "label": text(L("Label", "Beschriftung"), "content"),
        "required": toggle(L("Required", "Pflichtfeld"), "validation", L("The form cannot be submitted without an answer.", "Das Formular kann ohne Antwort nicht übermittelt werden.")),
        "default": record(L("Default", "Standardwert"), "value", L("Answer used until the respondent answers.", "Antwort, die gilt, bis die befragte Person antwortet.")),
        "params": record(L("Parameters", "Parameter"), "kind", L("Extra settings of the question type.", "Zusätzliche Einstellungen des Fragetyps.")),
        "condition": EXPRESSION(L("Condition", "Bedingung"), L("The question is shown only while this expression holds.", "Die Frage wird nur angezeigt, solange dieser Ausdruck gilt.")),
        "description": multiline(L("Description", "Beschreibung"), "content"), "placeholder": text(L("Placeholder", "Platzhalter"), "content"),
        "unit": text(L("Unit", "Einheit"), "content", L("Unit shown next to a number answer.", "Einheit neben einer Zahlenantwort.")),
        "text": multiline(L("Text", "Text"), "content", L("Static text of an information block.", "Fester Text eines Informationsblocks.")),
        "schema": text(L("Schema", "Schema"), "source", L("Schema id the answer follows.", "Schema-ID, der die Antwort folgt.")),
        "src": text(L("Source", "Quelle"), "source", URL),
        "accept": text(L("Accepted files", "Akzeptierte Dateien"), "source", L("File types an upload accepts, e.g. image/*.", "Dateitypen, die ein Upload annimmt, z. B. image/*.")),
        "fixtureSlug": text(L("Fixture slug", "Fixture-Kürzel"), "source", L("Slug of the bundled fixture the question draws its data from.", "Kürzel der mitgelieferten Fixture, aus der die Frage ihre Daten bezieht.")),
        "min": stepper(L("Minimum", "Minimum"), "validation", step=1, precision=2, description=L("Smallest accepted number.", "Kleinste zulässige Zahl.")),
        "max": stepper(L("Maximum", "Maximum"), "validation", step=1, precision=2, description=L("Largest accepted number.", "Größte zulässige Zahl.")),
        "step": stepper(L("Step", "Schrittweite"), "validation", step=0.1, precision=2, description=L("Increment of a number answer.", "Schrittweite einer Zahlenantwort.")),
        "options": record(L("Options", "Optionen"), "choices"), "options[].value": text(L("Value", "Wert"), "choices"), "options[].label": text(L("Label", "Beschriftung"), "choices"),
        "fields": record(L("Fields", "Felder"), "choices"), "fields[].key": text(L("Key", "Schlüssel"), "choices"), "fields[].label": text(L("Label", "Beschriftung"), "choices"),
        "fields[].value": stepper(L("Value", "Wert"), "choices", step=1, precision=2),
    },
    "Expression": {
        "|const.value": record(L("Value", "Wert"), "condition"),
        "|var.name": text(L("Question", "Frage"), "condition", L("Id of the question whose answer is read.", "ID der Frage, deren Antwort gelesen wird.")),
        "|eq.left": EXPRESSION(L("Left", "Links")), "|eq.right": EXPRESSION(L("Right", "Rechts")),
        "|truthy.expr": EXPRESSION(L("Expression", "Ausdruck")),
        "|and.items": EXPRESSION(L("Conditions", "Bedingungen")), "|or.items": EXPRESSION(L("Conditions", "Bedingungen")),
    },
    "https://json.schemas.assets.semio-tech.com/s/forms/forms/response.json": {},
    "FormsResponse": {
        "id": text(L("ID", "ID"), "identity", L("Unique response id.", "Eindeutige Antwort-ID.")),
        "submittedAt": integer(L("Submitted at", "Übermittelt am"), "identity", L("UTC milliseconds since the Unix epoch.", "UTC-Millisekunden seit der Unix-Epoche."), unit="ms"),
        "definitionVersion": text(L("Form version", "Formularversion"), "identity", L("Version of the form definition that was answered.", "Version der beantworteten Formulardefinition.")),
        "answers": record(L("Answers", "Antworten"), "answers"),
        "answers[].questionId": text(L("Question", "Frage"), "answers", L("Id of the answered question.", "ID der beantworteten Frage.")),
        "answers[].label": text(L("Label", "Beschriftung"), "answers"), "answers[].kind": text(L("Question type", "Fragetyp"), "answers"),
        "answers[].value": record(L("Answer", "Antwort"), "answers"),
    },
})
del RECORDS["https://json.schemas.assets.semio-tech.com/s/forms/forms/response.json"]
#endregion 🔖️Forms

#region 🔖️Raster
LAYER = target("layer", L("Layer", "Ebene"), L("The layer this mutation addresses.", "Die Ebene, die diese Mutation adressiert."), domain="layers", granularity="layer")
PARENT_LAYER = target("layer", L("Parent group", "Übergeordnete Gruppe"), L("Group layer that holds the layer; the root when omitted.", "Gruppenebene, die die Ebene enthält; ohne Angabe die Wurzel."), domain="layers", granularity="layer", group="placement")
RASTER_ASSET = target("asset", L("Asset", "Asset"), L("The image asset this mutation addresses.", "Das Bild-Asset, das diese Mutation adressiert."))
PIXELS = lambda label, bounds=None: stepper(label, "position", step=1, precision=1, unit="px", bounds=bounds)
EXPECTED = L("Value the layer must hold now; the edit is refused on a mismatch.", "Wert, den die Ebene jetzt haben muss; sonst wird die Bearbeitung abgelehnt.")


def affine(prefix=""):
    coefficient = lambda name, en_meaning, de_meaning: stepper(L(f"Matrix {name}", f"Matrix {name}"), "matrix", step=0.01, precision=4, description=L(f"Affine coefficient {name} of [a b c d x y]; {en_meaning}.", f"Affiner Koeffizient {name} von [a b c d x y]; {de_meaning}."))
    return {prefix + key: value for key, value in {
        "x": PIXELS(L("X", "X")), "y": PIXELS(L("Y", "Y")),
        "a": coefficient("a", "1 keeps the x scale", "1 behält die x-Skalierung"), "b": coefficient("b", "0 without skew", "0 ohne Scherung"),
        "c": coefficient("c", "0 without skew", "0 ohne Scherung"), "d": coefficient("d", "1 keeps the y scale", "1 behält die y-Skalierung"),
    }.items()}


leaves("s.raster.raster.mutation.", {
    "add-layer-asset": {"assetId": text(L("Asset ID", "Asset-ID"), "identity", L("Unique key the layers use to show this image.", "Eindeutiger Schlüssel, mit dem Ebenen dieses Bild zeigen.")), "asset": record(L("Image", "Bild"), "record", L("The encoded image to store.", "Das zu speichernde kodierte Bild."))},
    "change-layer-adjustment-kind": {"layerId": LAYER, "newAdjustmentKind": text(L("Adjustment", "Korrektur"), "adjustment", L("For example brightnessContrast, levels or curves.", "Zum Beispiel brightnessContrast (Helligkeit/Kontrast), levels (Tonwerte) oder curves (Gradationskurven)."))},
    "change-layer-adjustment-parameter": {
        "layerId": LAYER,
        "parameter": select(L("Parameter", "Parameter"), {"brightness": L("Brightness", "Helligkeit"), "contrast": L("Contrast", "Kontrast")}, "adjustment", widget="segmented"),
        "expected": slider(L("Expected value", "Erwarteter Wert"), "adjustment", (-1, 1), snaps=[-1, -0.5, 0, 0.5, 1], description=EXPECTED),
        "value": slider(L("Value", "Wert"), "adjustment", (-1, 1), snaps=[-1, -0.5, 0, 0.5, 1], description=L("New value; empty removes the parameter.", "Neuer Wert; leer entfernt den Parameter.")),
    },
    "change-layer-blend-mode": {"layerId": LAYER, "newBlendMode": text(L("Blend mode", "Füllmethode"), "appearance", L("For example normal or multiply.", "Zum Beispiel normal (Normal) oder multiply (Multiplizieren)."))},
    "change-layer-locked": {"layerId": LAYER, "expected": toggle(L("Expected locked", "Erwartet gesperrt"), "state", EXPECTED), "locked": LOCKED},
    "change-layer-mask": {"layerId": LAYER, "expected": record(L("Expected mask", "Erwartete Maske"), "mask", EXPECTED), "mask": record(L("Mask", "Maske"), "mask", L("The new mask; empty removes it.", "Die neue Maske; leer entfernt sie."))},
    "change-layer-opacity": {"layerId": LAYER, "newOpacity": unit_interval(L("Opacity", "Deckkraft"), "appearance")},
    "change-layer-pixels": {
        "layerId": LAYER,
        "expectedImageKey": text(L("Expected image", "Erwartetes Bild"), "pixels", EXPECTED),
        "content": record(L("Pixels", "Pixel"), "pixels", L("The new image of the layer.", "Das neue Bild der Ebene.")),
        "content.imageKey": text(L("Image key", "Bildschlüssel"), "pixels", L("Asset key of the image; empty clears the pixels.", "Asset-Schlüssel des Bilds; leer leert die Pixel.")),
        "content.width": integer(L("Width", "Breite"), "pixels", unit="px"), "content.height": integer(L("Height", "Höhe"), "pixels", unit="px"),
        "transform": record(L("Placement", "Platzierung"), "matrix", L("New placement; empty keeps the current one.", "Neue Platzierung; leer behält die aktuelle.")),
        **affine("transform."),
    },
    "change-layer-transform": {"layerId": LAYER, "expected": record(L("Expected placement", "Erwartete Platzierung"), "matrix", EXPECTED), "transform": record(L("Placement", "Platzierung"), "matrix")},
    "change-layer-visible": {"layerId": LAYER, "newVisible": toggle(L("Visible", "Sichtbar"), "state")},
    "create-layer": {"parentId": PARENT_LAYER, "index": index(), "layer": record(L("Layer", "Ebene"), "record", L("The kind of layer to add.", "Die Art der hinzuzufügenden Ebene."))},
    "delete-layer": {"layerId": LAYER},
    "move-layer": {"layerId": LAYER, "newX": PIXELS(L("X", "X")), "newY": PIXELS(L("Y", "Y"))},
    "remove-layer-asset": {"assetId": RASTER_ASSET},
    "rename-layer": {"layerId": LAYER, "newName": text(L("Name", "Name"), "identity")},
    "reorder-layers": {"layerId": LAYER, "parentId": PARENT_LAYER, "index": index()},
    "resize-layer": {"layerId": LAYER, "newWidth": integer(L("Width", "Breite"), "size", unit="px"), "newHeight": integer(L("Height", "Höhe"), "size", unit="px")},
})
VARIANTS["s.raster.raster.mutation.create-layer"] = {"layer|Pixel": L("Pixel layer", "Pixelebene"), "layer|Group": L("Group", "Gruppe"), "layer|Adjustment": L("Adjustment layer", "Einstellungsebene")}
RECORDS.update({
    "RasterImageAsset": {"mime": text(L("Media type", "Medientyp"), "source", L("For example image/png.", "Zum Beispiel image/png.")), "data": record(L("Bytes", "Bytes"), "source", L("The encoded image bytes.", "Die kodierten Bildbytes."))},
    "RasterLayerMask": {
        "enabled": toggle(L("Enabled", "Aktiv"), "mask"), "linked": toggle(L("Linked", "Verknüpft"), "mask", L("A linked mask moves with its layer.", "Eine verknüpfte Maske bewegt sich mit ihrer Ebene.")),
        "invert": toggle(L("Inverted", "Invertiert"), "mask"), "width": integer(L("Width", "Breite"), "mask", unit="px"), "height": integer(L("Height", "Höhe"), "mask", unit="px"),
        "imageKey": text(L("Image key", "Bildschlüssel"), "mask", L("Asset key of the mask image; empty reveals everything.", "Asset-Schlüssel des Maskenbilds; leer zeigt alles.")),
        "transform": record(L("Placement", "Platzierung"), "matrix"),
    },
    "RasterTransform": affine(),
})
#endregion 🔖️Raster

#region 🔖️Draw
DRAW_LAYER = target("layer", L("Layer", "Ebene"), L("The layer this mutation addresses.", "Die Ebene, die diese Mutation adressiert."), domain="strokes", granularity="stroke")
DRAW_PARENT = target("layer", L("Parent group", "Übergeordnete Gruppe"), L("Group layer that holds the layer; the root when omitted.", "Gruppenebene, die die Ebene enthält; ohne Angabe die Wurzel."), domain="strokes", granularity="stroke", group="placement")
COLOR = lambda label=L("Color", "Farbe"), group="style": vector(label, group, RGBA, step=0.01)
BLEND_MODES = {
    "normal": L("Normal", "Normal"), "multiply": L("Multiply", "Multiplizieren"), "screen": L("Screen", "Negativ multiplizieren"), "overlay": L("Overlay", "Ineinanderkopieren"),
    "darken": L("Darken", "Abdunkeln"), "lighten": L("Lighten", "Aufhellen"), "colorDodge": L("Color dodge", "Farbig abwedeln"), "colorBurn": L("Color burn", "Farbig nachbelichten"),
    "hardLight": L("Hard light", "Hartes Licht"), "softLight": L("Soft light", "Weiches Licht"), "difference": L("Difference", "Differenz"), "exclusion": L("Exclusion", "Ausschluss"),
    "hue": L("Hue", "Farbton"), "saturation": L("Saturation", "Sättigung"), "color": L("Color", "Farbe"), "luminosity": L("Luminosity", "Luminanz"),
}
BLEND = select(L("Blend mode", "Füllmethode"), BLEND_MODES, "appearance")
FILL_RULE = select(L("Fill rule", "Füllregel"), {"evenodd": L("Even-odd", "Gerade-ungerade"), "nonzero": L("Non-zero", "Nicht-null")}, "style", L("Which regions of a self-overlapping path count as inside.", "Welche Bereiche eines sich überlappenden Pfads als innen gelten."), widget="segmented")
DRAW_COORDINATE = lambda label, description=None: stepper(label, "geometry", step=1, precision=2, description=description)
POINT = lambda label, description=None: vector(label, "geometry", description, step=1)
STOPS = record(L("Color stops", "Farbstopps"), "style", L("Colors along the gradient.", "Farben entlang des Verlaufs."))

leaves("s.draw.drawing.1.metadata.mutation.", {
    "rename-layer": {"layerId": DRAW_LAYER, "newName": text(L("Name", "Name"), "identity")},
    "set-layer-locked": {"layerId": DRAW_LAYER, "locked": LOCKED},
    "set-layer-visible": {"layerId": DRAW_LAYER, "visible": toggle(L("Visible", "Sichtbar"), "state")},
})
leaves("s.draw.drawing.1.structure.mutation.", {
    "create-layer": {"parentId": DRAW_PARENT, "index": index(), "layer": record(L("Layer", "Ebene"), "record", L("The complete layer to add.", "Die vollständige hinzuzufügende Ebene."))},
    "delete-layer": {"layerId": DRAW_LAYER},
    "duplicate-layer": {"layerId": DRAW_LAYER},
    "reorder-layer": {"layerId": DRAW_LAYER, "parentId": DRAW_PARENT, "index": index()},
})
leaves("s.draw.drawing.1.style.mutation.", {
    "replace-layer-fill": {
        "layerId": DRAW_LAYER, "fill": record(L("Fill", "Füllung"), "style", L("The new fill; empty removes it.", "Die neue Füllung; leer entfernt sie.")),
        "fill|solid.color": COLOR(),
        "fill|linearGradient.x1": DRAW_COORDINATE(L("Start X", "Start X")), "fill|linearGradient.y1": DRAW_COORDINATE(L("Start Y", "Start Y")),
        "fill|linearGradient.x2": DRAW_COORDINATE(L("End X", "Ende X")), "fill|linearGradient.y2": DRAW_COORDINATE(L("End Y", "Ende Y")), "fill|linearGradient.stops": STOPS,
        "fill|radialGradient.cx": DRAW_COORDINATE(L("Center X", "Mittelpunkt X")), "fill|radialGradient.cy": DRAW_COORDINATE(L("Center Y", "Mittelpunkt Y")),
        "fill|radialGradient.r": stepper(L("Radius", "Radius"), "geometry", step=1, precision=2, bounds=NON_NEGATIVE), "fill|radialGradient.stops": STOPS,
    },
    "replace-layer-stroke": {"layerId": DRAW_LAYER, "stroke": record(L("Stroke", "Kontur"), "style", L("The new stroke; empty removes it.", "Die neue Kontur; leer entfernt sie."))},
    "set-group-isolation": {"layerId": DRAW_LAYER, "isolation": toggle(L("Isolated", "Isoliert"), "appearance", L("Composite the group on a transparent backdrop before blending it into its parent.", "Die Gruppe auf transparentem Hintergrund zusammensetzen, bevor sie mit der übergeordneten Ebene verrechnet wird."))},
    "set-layer-blend-mode": {"layerId": DRAW_LAYER, "blendMode": BLEND},
    "set-layer-fill-rule": {"layerId": DRAW_LAYER, "fillRule": FILL_RULE},
    "set-layer-opacity": {"layerId": DRAW_LAYER, "opacity": unit_interval(L("Opacity", "Deckkraft"), "appearance")},
    "update-text": {"layerId": DRAW_LAYER, "content": multiline(L("Text", "Text"), "content"), "size": stepper(L("Font size", "Schriftgröße"), "content", step=1, precision=1, soft=(6, 96))},
})
leaves("s.draw.drawing.1.transform.mutation.", {
    "set-layer-boolean-operation": {"layerId": DRAW_LAYER, "booleanOperation": text(L("Boolean operation", "Boolesche Operation"), "geometry", L("union, intersect or subtract.", "union (Vereinigung), intersect (Schnitt) oder subtract (Differenz)."))},
    "update-layer-trace-params": {"layerId": DRAW_LAYER, "params": record(L("Trace settings", "Nachzeichnen-Einstellungen"), "trace")},
    "update-layer-transform": {"layerId": DRAW_LAYER, "transform": record(L("Transform", "Transformation"), "transform", L("Position, scale, rotation and shear of the layer.", "Position, Skalierung, Drehung und Scherung der Ebene."))},
    "update-path-geometry": {"layerId": DRAW_LAYER, "segments": record(L("Segments", "Segmente"), "geometry", L("The complete path, segment by segment.", "Der vollständige Pfad, Segment für Segment."))},
})
VARIANTS["s.draw.drawing.1.style.mutation.replace-layer-fill"] = {"fill|solid": L("Solid", "Vollton"), "fill|linearGradient": L("Linear gradient", "Linearer Verlauf"), "fill|radialGradient": L("Radial gradient", "Radialer Verlauf")}
VARIANTS["PathSegment"] = {"|move": L("Move to", "Bewegen nach"), "|line": L("Line to", "Linie nach"), "|quad": L("Quadratic curve", "Quadratische Kurve"), "|cubic": L("Cubic curve", "Kubische Kurve"), "|arc": L("Arc", "Bogen"), "|close": L("Close path", "Pfad schließen")}
RECORDS.update({
    "DrawingLayerNode": {
        "kind": text(L("Kind", "Art"), "kind", L("For example shape, path, text, trace or group.", "Zum Beispiel shape (Form), path (Pfad), text (Text), trace (Nachzeichnung) oder group (Gruppe).")),
        "attributes": record(L("Attributes", "Attribute"), "style"), "attributes.stroke": record(L("Stroke", "Kontur"), "style"), "attributes.fillRule": FILL_RULE, "blendMode": BLEND,
    },
    "StrokeStyle": {
        "color": COLOR(), "width": stepper(L("Width", "Breite"), "style", step=0.5, precision=2, soft=(0, 32)),
        "cap": select(L("Line cap", "Linienende"), {"butt": L("Butt", "Stumpf"), "round": L("Round", "Rund"), "square": L("Square", "Quadratisch")}, "style", widget="segmented"),
        "join": select(L("Line join", "Linienverbindung"), {"miter": L("Miter", "Gehrung"), "round": L("Round", "Rund"), "bevel": L("Bevel", "Abgeschrägt")}, "style", widget="segmented"),
        "dash": record(L("Dash pattern", "Strichmuster"), "style", L("Alternating dash and gap lengths.", "Abwechselnde Strich- und Lückenlängen.")),
    },
    "Stops": {
        "[].offset": slider(L("Offset", "Position"), "style", (0, 1), snaps=[0, 0.25, 0.5, 0.75, 1], description=L("Where the stop sits along the gradient, 0 to 1.", "Lage des Stopps entlang des Verlaufs, 0 bis 1.")),
        "[].color": COLOR(),
    },
    "DrawingTraceParams": {
        "threshold": unit_interval(L("Threshold", "Schwellenwert"), "trace", L("Brightness from which a pixel counts as ink.", "Helligkeit, ab der ein Pixel als Tinte gilt."), percent=False),
        "simplifyEpsilon": stepper(L("Simplification", "Vereinfachung"), "trace", step=0.05, precision=2, soft=(0, 10), bounds=NON_NEGATIVE, description=L("Largest deviation allowed when simplifying the traced path.", "Größte zulässige Abweichung beim Vereinfachen des Pfads.")),
    },
    "DrawingTransform": {
        "x": DRAW_COORDINATE(L("X", "X")), "y": DRAW_COORDINATE(L("Y", "Y")),
        "scaleX": stepper(L("Scale X", "Skalierung X"), "transform", step=0.01, precision=3, description=L("Negative values mirror the layer.", "Negative Werte spiegeln die Ebene.")),
        "scaleY": stepper(L("Scale Y", "Skalierung Y"), "transform", step=0.01, precision=3, description=L("Negative values mirror the layer.", "Negative Werte spiegeln die Ebene.")),
        "rotation": angle_rad(L("Rotation", "Drehung"), "transform"),
        "shear": stepper(L("Shear", "Scherung"), "transform", step=0.01, precision=3),
    },
    "PathSegment": {
        "|move.to": POINT(L("To", "Nach")), "|line.to": POINT(L("To", "Nach")),
        "|quad.ctrl": POINT(L("Control point", "Kontrollpunkt")), "|quad.to": POINT(L("To", "Nach")),
        "|cubic.ctrl1": POINT(L("First control point", "Erster Kontrollpunkt")), "|cubic.ctrl2": POINT(L("Second control point", "Zweiter Kontrollpunkt")), "|cubic.to": POINT(L("To", "Nach")),
        "|arc.rx": stepper(L("Radius X", "Radius X"), "geometry", step=1, precision=2), "|arc.ry": stepper(L("Radius Y", "Radius Y"), "geometry", step=1, precision=2),
        "|arc.rotation": angle_deg(L("Axis rotation", "Achsendrehung"), "geometry", L("Rotation of the ellipse's x axis.", "Drehung der x-Achse der Ellipse.")),
        "|arc.largeArc": toggle(L("Large arc", "Großer Bogen"), "geometry"), "|arc.sweep": toggle(L("Clockwise", "Im Uhrzeigersinn"), "geometry"), "|arc.to": POINT(L("To", "Nach")),
    },
})
#endregion 🔖️Draw


#region 🔖️Walker
def load(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return handle.read()


class Document:
    def __init__(self, path):
        self.path = path
        self.text = load(path)
        self.data = json.loads(self.text)
        self.standard = json.dumps(self.data, indent=2, ensure_ascii=False) + "\n" == self.text
        self.edits = []


def escape(key):
    return key.replace("~", "~0").replace("/", "~1")


def at(node, pointer):
    for part in [segment.replace("~1", "/").replace("~0", "~") for segment in pointer.split("/")[1:]]:
        node = node[int(part)] if isinstance(node, list) else node.get(part) if isinstance(node, dict) else None
        if node is None:
            return None
    return node


class Walker:
    def __init__(self):
        catalog = json.loads(load(CATALOG))["scopes"]
        self.leaves = {scope: f"{entry['path']}/{entry['formats'].get('🔣️jsonschema', '🔣️.json')}" for scope, entry in catalog.items() if entry.get("level") == "mutation-leaf" and entry["path"].startswith(ROOTS)}
        self.leaves.update({scope: f"{path}/🔣️.json" for scope, path in UNCATALOGUED.items()})
        self.documents = {}
        self.ids = {}
        for root in ROOTS:
            for directory, _, files in os.walk(os.path.join(REPO, root)):
                for name in files:
                    if name.endswith(".json") and "🧫️fixtures" not in directory:
                        path = os.path.relpath(os.path.join(directory, name), REPO)
                        try:
                            data = json.loads(load(path))
                        except ValueError:
                            continue
                        if isinstance(data, dict) and isinstance(data.get("$id"), str) and "$schema" in data:
                            self.ids[data["$id"]] = path
        self.visited = set()
        self.used = set()
        self.missing = []
        self.unresolved = []

    def document(self, path):
        if path not in self.documents:
            self.documents[path] = Document(path)
        return self.documents[path]

    def resolve(self, document, reference):
        base, _, fragment = reference.partition("#")
        if base == "":
            target = document
        elif base in self.ids:
            target = self.document(self.ids[base])
        elif not re.match(r"^[a-z]+:", base):
            path = os.path.normpath(os.path.join(os.path.dirname(document.path), base))
            if not os.path.isfile(os.path.join(REPO, path)):
                return None, None, None
            target = self.document(path)
        else:
            return None, None, None
        node = at(target.data, fragment) if fragment else target.data
        return (target, fragment, node) if node is not None else (None, None, None)

    def lookup(self, owner, path, key, node):
        table, name = (LEAVES, owner[1]) if owner[0] == "leaf" else (RECORDS, owner[1])
        entry = table.get(name, {}).get(path)
        if entry is not None:
            self.used.add((name, path))
            return entry
        if isinstance(node, dict) and "const" in node:
            return discriminator(L("Mutation", "Mutation") if key == "mutation" else None)
        return None

    def record_name(self, document, pointer, node):
        if isinstance(node.get("title"), str) and pointer != "":
            return node["title"]
        match = re.fullmatch(r"/\$defs/([^/]+)", pointer)
        if match:
            return match.group(1)
        return node.get("title") or document.data.get("$id") if pointer == "" else None

    def annotate(self, document, pointer, node, owner, path, key, position):
        entry = self.lookup(owner, path, key, node)
        if entry is None:
            self.missing.append((owner[1], path, document.path))
            return
        entry = dict(entry)
        bounds = entry.pop("_bounds", {})
        if "order" not in entry and entry.get("role") != "discriminator":
            entry["order"] = 10 * (position + 1)
        entry = {name: entry[name] for name in KEY_ORDER if name in entry}
        document.edits.append((pointer, bounds, entry))

    def visit(self, document, pointer, node, owner, path):
        if not isinstance(node, dict):
            return
        if "$ref" in node:
            target, fragment, resolved = self.resolve(document, node["$ref"])
            if resolved is None:
                self.unresolved.append((document.path, pointer, node["$ref"]))
                return
            name = self.record_name(target, fragment, resolved)
            if name is None:
                self.visit(target, fragment, resolved, owner, path)
            elif (target.path, fragment) not in self.visited:
                self.visited.add((target.path, fragment))
                self.body(target, fragment, resolved, ("record", name), "")
            return
        if isinstance(node.get("title"), str) and pointer != "" and (owner[0] == "leaf" or path != ""):
            if (document.path, pointer) not in self.visited:
                self.visited.add((document.path, pointer))
                self.body(document, pointer, node, ("record", node["title"]), "")
            return
        self.body(document, pointer, node, owner, path)

    def body(self, document, pointer, node, owner, path):
        for union in ("anyOf", "oneOf"):
            for position, branch in enumerate(node.get(union, [])):
                if not isinstance(branch, dict) or branch.get("type") == "null":
                    continue
                branch_pointer = f"{pointer}/{union}/{position}"
                constant = next((value["const"] for value in branch.get("properties", {}).values() if isinstance(value, dict) and isinstance(value.get("const"), str)), None)
                if constant is not None:
                    label = VARIANTS.get(owner[1], {}).get(f"{path}|{constant}")
                    if label is None:
                        self.missing.append((owner[1], f"{path}|{constant} (variant)", document.path))
                    else:
                        self.used.add(("variant", owner[1], f"{path}|{constant}"))
                        document.edits.append((branch_pointer, {}, {"label": label}))
                self.visit(document, branch_pointer, branch, owner, f"{path}|{constant}" if constant is not None else path)
        for position, member in enumerate(node.get("allOf", [])):
            self.visit(document, f"{pointer}/allOf/{position}", member, owner, path)
        if isinstance(node.get("items"), dict):
            self.visit(document, f"{pointer}/items", node["items"], owner, f"{path}[]")
        if isinstance(node.get("properties"), dict):
            for position, (key, child) in enumerate(node["properties"].items()):
                child_pointer = f"{pointer}/properties/{escape(key)}"
                child_path = key if path == "" else f"{path}.{key}"
                self.annotate(document, child_pointer, child, owner, child_path, key, position)
                self.visit(document, child_pointer, child, owner, child_path)

    def run(self):
        for scope, path in sorted(self.leaves.items()):
            document = self.document(path)
            self.body(document, "", document.data, ("leaf", scope), "")
        for path, pointer in EXTRA_RECORDS:
            document = self.document(path)
            node = at(document.data, pointer)
            if (path, pointer) not in self.visited:
                self.visited.add((path, pointer))
                self.body(document, pointer, node, ("record", self.record_name(document, pointer, node)), "")
#endregion 🔖️Walker


#region 🔖️Writer
def merged(node, bounds, entry):
    body = {}
    for key, value in node.items():
        if key == "x-semio-ui":
            continue
        body[key] = bounds.get(key, value)
    for key, value in bounds.items():
        body.setdefault(key, value)
    if entry:
        body["x-semio-ui"] = entry
    return body


def replace_node(data, pointer, value):
    if pointer == "":
        return value
    parent = at(data, "/".join(pointer.split("/")[:-1]))
    last = pointer.split("/")[-1].replace("~1", "/").replace("~0", "~")
    if isinstance(parent, list):
        parent[int(last)] = value
    else:
        parent[last] = value
    return data


class Spans:
    """🧭️ Byte spans of every value of a JSON text, by JSON pointer: `{pointer: (start, end, keys)}` where `keys` maps a
    member name to `(key_start, value_start, value_end)` for objects."""

    def __init__(self, text):
        self.text = text
        self.spans = {}
        self.at = 0
        self.value("")

    def skip(self):
        while self.at < len(self.text) and self.text[self.at] in " \t\r\n":
            self.at += 1

    def string(self):
        start = self.at
        self.at += 1
        while self.text[self.at] != '"':
            self.at += 2 if self.text[self.at] == "\\" else 1
        self.at += 1
        return json.loads(self.text[start:self.at])

    def value(self, pointer):
        self.skip()
        start = self.at
        character = self.text[self.at]
        keys = {}
        if character == "{":
            self.at += 1
            self.skip()
            while self.text[self.at] != "}":
                key_start = self.at
                key = self.string()
                self.skip()
                self.at += 1
                self.skip()
                value_start = self.at
                self.value(f"{pointer}/{escape(key)}")
                keys[key] = (key_start, value_start, self.at)
                self.skip()
                if self.text[self.at] == ",":
                    self.at += 1
                    self.skip()
            self.at += 1
        elif character == "[":
            self.at += 1
            self.skip()
            position = 0
            while self.text[self.at] != "]":
                self.value(f"{pointer}/{position}")
                position += 1
                self.skip()
                if self.text[self.at] == ",":
                    self.at += 1
                    self.skip()
            self.at += 1
        elif character == '"':
            self.string()
        else:
            match = re.compile(r"-?[0-9.eE+\-]+|true|false|null").match(self.text, self.at)
            self.at = match.end()
        self.spans[pointer] = (start, self.at, keys)


def span_edits(document):
    text = document.text
    spans = Spans(text).spans
    spaced = '": ' in text
    separators = (", ", ": ") if spaced else (",", ":")
    render = lambda value: json.dumps(value, ensure_ascii=False, separators=separators)
    edits = []
    for pointer, bounds, entry in document.edits:
        start, end, keys = spans[pointer]
        members = dict(bounds)
        if entry:
            members["x-semio-ui"] = entry
        inserts = []
        for key, value in members.items():
            if key in keys:
                _, value_start, value_end = keys[key]
                edits.append((value_start, value_end, render(value)))
            else:
                inserts.append(f"{render(key)}{separators[1]}{render(value)}")
        if inserts:
            close = end - 1
            head = text[start + 1:close].rstrip()
            glue = separators[0] if head.strip() else ""
            anchor = start + 1 + len(head)
            edits.append((anchor, anchor, glue + separators[0].join(inserts)))
    out = text
    for start, end, replacement in sorted(edits, key=lambda edit: (edit[0], edit[1]), reverse=True):
        out = out[:start] + replacement + out[end:]
    return out


def write(document, dry_run):
    if not document.edits:
        return False
    if document.standard:
        data = document.data
        for pointer, bounds, entry in document.edits:
            node = at(data, pointer) if pointer else data
            if "label" in entry and len(entry) == 1 and "widget" not in entry:
                existing = node.get("x-semio-ui", {})
                entry = {**{key: value for key, value in existing.items() if key != "label"}, **entry} if existing else entry
            data = replace_node(data, pointer, merged(node, bounds, entry))
        out = json.dumps(data, indent=2, ensure_ascii=False) + "\n"
    else:
        out = span_edits(document)
        json.loads(out)
    if out == document.text:
        return False
    if not dry_run:
        with open(os.path.join(REPO, document.path), "w", encoding="utf-8") as handle:
            handle.write(out)
    return True
#endregion 🔖️Writer


def main():
    walker = Walker()
    walker.run()
    report = "--report" in sys.argv
    unused = [f"{name} :: {path}" for table in (LEAVES, RECORDS) for name, fields in table.items() for path in fields if (name, path) not in walker.used]
    unused += [f"{name} :: {path} (variant)" for name, fields in VARIANTS.items() for path in fields if ("variant", name, path) not in walker.used]
    for owner, path, where in walker.missing:
        print(f"MISSING {owner} :: {path}    [{where}]")
    for entry in unused:
        print(f"UNUSED {entry}")
    for where, pointer, reference in walker.unresolved:
        print(f"UNRESOLVED {where} {pointer} -> {reference}")
    if walker.missing or unused:
        print(f"{len(walker.missing)} missing, {len(unused)} unused table entries")
        if not report:
            return 1
    if report:
        return 0
    dry_run = "--dry-run" in sys.argv
    changed = [document.path for document in walker.documents.values() if write(document, dry_run)]
    print(f"{'would change' if dry_run else 'changed'} {len(changed)} of {len(walker.documents)} schema document(s); {sum(len(document.edits) for document in walker.documents.values())} annotation(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
