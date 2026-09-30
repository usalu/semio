#!/usr/bin/env python3
"""🧰️ W2-S-C design-group payload parity fixes (idempotent; re-reads every file right before writing it).

Structural leaf-schema fixes (5d `2d`/`3d` keys, anchor casing, catalog `abstract`, cad `target`, shooting asset `scale`,
3d object `anchor` / vortex `label`, procedural widget union + generation `values`, raster layer union + asset bytes, forms
`params`, draw leaf `$id`), then the Rust-driven `nullable`/`required` passes of `🧪️w2-s-design-rust-parity.py`, the
colour widget (`widget: color`, items 0..1) and 3D grid snaps, and the snake_case → camelCase fixture/test rewrites of the
leaves whose Rust payload gains `rename_all = "camelCase"`.

Usage: python3 🧪️w2-s-design-parity-fix.py [--dry-run]
"""
import importlib.util
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
DRY = "--dry-run" in sys.argv
SH = "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any"
P3 = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
P5 = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any"
G2 = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any"
G3 = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
CAD = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any"
RA = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any"
FO = "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any"
DR = "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets"
NO = "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any"
ROOTS = ["✏️s/🔌️plugins/🎥️shooting", "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d", "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d", "✏️s/🔌️plugins/🌀️procedural", "✏️s/🔌️plugins/📐️cad", "✏️s/🔌️plugins/🖨️raster", "✏️s/🔌️plugins/🗒️note", "✏️s/🔌️plugins/📋️forms", "✏️s/🔌️plugins/🧱️block", "✏️s/🔌️plugins/🖍️draw"]
RASTER_ARTIFACT = "https://json.schemas.assets.semio-tech.com/s/raster/raster/artifact.json"
written = []


def read(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return handle.read()


def dumps(document):
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def edit_json(path, change):
    """Applies `change(document)` to a canonical JSON file; refuses a non-canonical one (hand-formatted files get text edits)."""
    text = read(path)
    document = json.loads(text)
    if dumps(document) != text:
        raise SystemExit(f"[w2-s-c] {path} is not canonical json.dumps(indent=2); edit it by text")
    change(document)
    rendered = dumps(document)
    if rendered != text:
        if read(path) != text:
            raise SystemExit(f"[w2-s-c] {path} changed while editing; rerun")
        written.append(path)
        if not DRY:
            with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
                handle.write(rendered)


def edit_text(path, change):
    text = read(path)
    rendered = change(text)
    if rendered != text:
        json.loads(rendered)
        written.append(path)
        if not DRY:
            with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
                handle.write(rendered)


def leaf(base, slug):
    directory = os.path.join(REPO, base, "🧬️schema", "🧬️mutations")
    found = [name for name in os.listdir(directory) if re.sub(r"^[^a-z0-9]+", "", name) == slug]
    assert len(found) == 1, (base, slug, found)
    return f"{base}/🧬️schema/🧬️mutations/{found[0]}/🧬️schema/🔣️.json"


def label(en, de):
    return {"en": en, "de": de}


def ui(en, de, widget=None, role="value", description=None, group=None, order=None, **facets):
    value = {}
    if widget is not None:
        value["widget"] = widget
    value["role"] = role
    value["label"] = label(en, de)
    if description is not None:
        value["description"] = label(*description)
    value.update(facets)
    if group is not None:
        value["group"] = group
    if order is not None:
        value["order"] = order
    return value


def renamed(mapping, old, new):
    """The mapping with key `old` renamed to `new` in place (order kept)."""
    return {(new if key == old else key): value for key, value in mapping.items()}


def rename_property(node, old, new):
    if old in node.get("properties", {}):
        node["properties"] = renamed(node["properties"], old, new)
    if old in node.get("required", []):
        node["required"] = [new if name == old else name for name in node["required"]]


def put_property(node, key, schema, required, after=None):
    properties = node["properties"]
    if key not in properties:
        items = list(properties.items())
        index = len(items) if after is None else [name for name, _ in items].index(after) + 1
        items.insert(index, (key, schema))
        node["properties"] = dict(items)
    else:
        properties[key] = schema
    names = node.setdefault("required", [])
    if required and key not in names:
        names.append(key)
    if not required and key in names:
        names.remove(key)


#region 🔖️Structural
def anchor_enum(node):
    node["enum"] = ["fixed", "derived"]
    options = node["x-semio-ui"]["options"]
    node["x-semio-ui"]["options"] = {"fixed": options.get("fixed", options.get("Fixed")), "derived": options.get("derived", options.get("Derived"))}


def anchor_schema(order):
    return {
        "type": "string",
        "enum": ["fixed", "derived"],
        "x-semio-ui": ui("Anchor", "Verankerung", "segmented", description=("Fixed keeps the stored plane at a root; derived resets it to the default XY plane.", "Fest behält die gespeicherte Ebene an einer Wurzel; abgeleitet setzt sie auf die XY-Standardebene zurück."), options={"fixed": label("Fixed", "Fest"), "derived": label("Derived", "Abgeleitet")}, group="geometry", order=order),
    }


def vortex_label(node):
    put_property(node, "label", {"type": "string", "x-semio-ui": ui("Label", "Bezeichnung", "text", group="appearance", order=25)}, False, after="vortexKind")


def structural():
    edit_json(leaf(P3, "change-object-anchor"), lambda document: anchor_enum(document["properties"]["newAnchor"]))

    def create_object(document):
        record = document["properties"]["object"]
        put_property(record, "anchor", anchor_schema(35), True, after="objectKind")
        vortex_label(record["properties"]["vortices"]["items"])

    edit_json(leaf(P3, "create-object"), create_object)
    edit_json(leaf(P3, "add-object-vortex"), lambda document: vortex_label(document["properties"]["vortex"]))
    edit_json(leaf(P3, "replace-object-vortex"), lambda document: vortex_label(document["properties"]["newVortex"]))
    edit_json(leaf(P3, "replace-kind-catalogs"), lambda document: rename_property(document["properties"]["newCatalogs"]["properties"]["objects"]["items"], "isAbstract", "abstract"))
    edit_json(leaf(P5, "replace-kind-catalogs"), lambda document: rename_property(document["properties"]["newCatalogs"]["properties"]["parts"]["items"], "isAbstract", "abstract"))

    def grip(node):
        rename_property(node, "grip2d", "2d")
        rename_property(node, "grip3d", "3d")

    def create_part(document):
        part = document["properties"]["part"]
        rename_property(part, "part2d", "2d")
        rename_property(part, "part3d", "3d")
        grip(part["properties"]["grips"]["items"])

    edit_json(leaf(P5, "create-part"), create_part)
    edit_json(leaf(P5, "add-part-grip"), lambda document: grip(document["properties"]["grip"]))
    edit_json(leaf(P5, "replace-part-grip"), lambda document: grip(document["properties"]["newGrip"]))

    def create_asset(document):
        record = document["properties"]["asset"]
        put_property(record, "scale", {"type": ["array", "null"], "items": {"type": "number"}, "minItems": 3, "maxItems": 3, "x-semio-ui": ui("Scale", "Skalierung", "vector", description=("Scale factor per axis (x, y, z); empty keeps the mesh size.", "Skalierungsfaktor je Achse (x, y, z); leer behält die Netzgröße."), step=0.1, precision=3, group="pose", order=70)}, False, after="orientation")

    edit_json(leaf(SH, "create-asset"), create_asset)

    for slug in ("create-shape-model", "create-building-model", "create-structure-classic-model", "create-energy-model"):
        edit_json(leaf(CAD, slug), lambda document: put_property(document, "target", {"type": "string", "minLength": 1, "x-semio-ui": ui("Target artifact", "Zielartefakt", "text", description=("Reference URI of the child model artifact (id!type@version/subset).", "Referenz-URI des Kind-Modellartefakts (id!typ@version/teilmenge)."), group="identity", order=20)}, True, after="childId"))

    def create_generation(document):
        record = document["properties"]["generation"]
        properties = record["properties"]
        if "valuesJson" in properties:
            record["properties"] = renamed(properties, "valuesJson", "values")
            record["required"] = ["values" if name == "valuesJson" else name for name in record["required"]]
        record["properties"]["values"] = {"type": "object", "additionalProperties": {"description": "the answer, any JSON value"}, "x-semio-ui": ui("Answers", "Antworten", description=("Answer per question id.", "Antwort je Fragen-ID."), group="value", order=30)}

    edit_json(leaf(G2, "create-generation"), create_generation)
    edit_json(leaf(G3, "create-generation"), create_generation)
    for base, slug in ((G2, "create-widget"), (G2, "replace-widget"), (G3, "create-widget"), (G3, "update-widget")):
        edit_json(leaf(base, slug), widget_union)

    def create_layer(document):
        layer = document["properties"]["layer"]
        document["properties"]["layer"] = {"$ref": f"{RASTER_ARTIFACT}#/$defs/RasterLayerNode", "x-semio-ui": layer["x-semio-ui"]}

    edit_json(leaf(RA, "create-layer"), create_layer)
    edit_json(f"{RA}/🧬️schema/🔣️.json", raster_layer_node)

    def layer_asset(document):
        record = document["properties"]["asset"]
        data = record["properties"]["data"]
        record["properties"]["data"] = {"type": "string", "contentEncoding": "base64", "x-semio-ui": data.get("x-semio-ui") or ui("Image data", "Bilddaten", "multiline", description=("Image bytes, Base64-encoded.", "Bildbytes, Base64-kodiert."), group="source", order=20)}
        record["properties"]["data"]["x-semio-ui"].update({"widget": "multiline", "description": label("Image bytes, Base64-encoded.", "Bildbytes, Base64-kodiert.")})

    edit_json(leaf(RA, "add-layer-asset"), layer_asset)

    def question_params(document):
        params = document["$defs"]["Question"]["properties"]["params"]
        params.pop("type", None)
        params["x-semio-ui"]["description"] = label("Extra settings of the question type, any JSON value.", "Zusätzliche Einstellungen des Fragetyps, ein beliebiger JSON-Wert.")

    edit_json(f"{FO}/🧬️schema/📝️definition/🔣️.json", question_params)

    def draft07(document):
        head = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": document["$id"]}
        rest = {key: value for key, value in document.items() if key not in head}
        document.clear()
        document.update(head)
        document.update(rest)

    edit_json(leaf(RA, "change-layer-pixels"), draft07)

    def note_stroke_colour(document):
        stroke = next(branch for branch in document["$defs"]["NoteBlockNode"]["oneOf"] if branch["properties"]["kind"].get("const") == "stroke")
        colour = stroke["properties"]["color"]
        colour["items"] = dict(UNIT_ITEMS)
        colour["x-semio-ui"] = ui("Color", "Farbe", "color", description=("Red, green, blue and alpha, each from 0 to 1.", "Rot, Grün, Blau und Alpha, jeweils von 0 bis 1."), step=0.01, group="style", order=130)

    edit_json(f"{NO}/🧬️schema/🔣️.json", note_stroke_colour)

    def draw_id(document):
        document["$id"] = "https://json.schemas.assets.semio-tech.com/s/draw/drawing/1/transform/mutation/update-path-geometry/schema.json"

    edit_json(f"{DR}/🔀️transform/🧬️schema/🧬️mutations/✏️update-path-geometry/🧬️schema/🔣️.json", draw_id)
#endregion 🔖️Structural


#region 🔖️WidgetUnion
def discriminator():
    return {"widget": "hidden", "role": "discriminator", "label": label("Kind", "Art")}


def variant(kind, en, de, fields):
    properties = {"kind": {"const": kind, "x-semio-ui": discriminator()}}
    properties.update(fields)
    return {"type": "object", "additionalProperties": False, "required": list(properties), "properties": properties, "x-semio-ui": {"label": label(en, de)}}


def text(en, de, order, group="identity", description=None, widget="text"):
    return {"type": "string", "x-semio-ui": ui(en, de, widget, description=description, group=group, order=order)}


def number(en, de, order, group="value", step=0.1):
    return {"type": "number", "x-semio-ui": ui(en, de, "stepper", step=step, precision=3, group=group, order=order)}


def strings(en, de, order, group="ports", description=None):
    return {"type": "array", "items": {"type": "string"}, "x-semio-ui": ui(en, de, description=description, group=group, order=order)}


def ref(name, en, de, order, group="record", description=None):
    return {"$ref": f"#/$defs/{name}", "x-semio-ui": ui(en, de, description=description, group=group, order=order)}


def nullable_ref(name, en, de, order, group="record"):
    return {"anyOf": [{"$ref": f"#/$defs/{name}"}, {"type": "null"}], "x-semio-ui": ui(en, de, group=group, order=order)}


def record(properties, required):
    return {"type": "object", "additionalProperties": False, "required": required, "properties": properties}


def widget_defs():
    widget_id = lambda: text("ID", "ID", 10, description=("Unique widget id in this graph.", "Eindeutige Baustein-ID in diesem Graphen."))
    return {
        "Widget": {
            "oneOf": [
                variant("neuron", "Neuron", "Neuron", {"id": widget_id(), "neuronKind": text("Neuron kind", "Neuronenart", 20, "kind", ("Operator id, e.g. math.add.", "Operator-ID, z. B. math.add.")), "params": ref("Dictionary", "Parameters", "Parameter", 30, "value", ("Operator parameters by key.", "Operatorparameter je Schlüssel.")), "inputPorts": strings("Input ports", "Eingänge", 40), "outputPorts": strings("Output ports", "Ausgänge", 50), "preview": {"type": "boolean", "x-semio-ui": ui("Preview", "Vorschau", "toggle", group="appearance", order=60)}}),
                variant("inputSlider", "Slider input", "Schiebereglereingabe", {"id": widget_id(), "label": text("Label", "Bezeichnung", 20, "appearance"), "value": number("Value", "Wert", 30), "min": number("Minimum", "Minimum", 40, "range"), "max": number("Maximum", "Maximum", 50, "range"), "step": number("Step", "Schrittweite", 60, "range")}),
                variant("inputNote", "Note input", "Notizeingabe", {"id": widget_id(), "text": text("Text", "Text", 20, "value", widget="multiline")}),
                variant("inputImage", "Image input", "Bildeingabe", {"id": widget_id(), "src": text("Image source", "Bildquelle", 20, "source", ("Address of the image, e.g. an https or data URL.", "Adresse des Bildes, z. B. eine https- oder data-URL."))}),
                variant("variable", "Variable", "Variable", {"id": widget_id(), "name": text("Name", "Name", 20), "schema": text("Schema", "Schema", 30, "kind", ("Value schema of the variable, e.g. dictionary.", "Werteschema der Variablen, z. B. dictionary."))}),
                variant("outputPreview", "Preview output", "Vorschauausgabe", {"id": widget_id(), "preview": ref("Dictionary", "Preview values", "Vorschauwerte", 20, "value"), "expanded": strings("Expanded entries", "Aufgeklappte Einträge", 30, "appearance")}),
                variant("outputAction", "Action output", "Aktionsausgabe", {"id": widget_id(), "action": text("Action", "Aktion", 20, "value")}),
                variant("outputExport", "Export output", "Exportausgabe", {"id": widget_id(), "format": text("Format", "Format", 20, "value", ("Export format, e.g. svg or png.", "Exportformat, z. B. svg oder png."))}),
                variant("cluster", "Cluster", "Cluster", {"id": widget_id(), "name": text("Name", "Name", 20), "tree": ref("FlowTree", "Graph", "Graph", 30, "record", ("Neurons and synapses inside the cluster.", "Neuronen und Synapsen im Cluster.")), "flow": ref("FlowUi", "Layout", "Anordnung", 40, "appearance", ("Canvas presentation of the cluster graph.", "Darstellung des Clustergraphen auf der Leinwand."))}),
            ],
        },
        "Dictionary": {"type": "object", "additionalProperties": {"description": "a neural value, any JSON value"}},
        "FlowTree": record({"neurons": {"type": "array", "items": {"$ref": "#/$defs/FlowNeuron"}, "x-semio-ui": ui("Neurons", "Neuronen", group="record", order=10)}, "synapses": {"type": "array", "items": {"$ref": "#/$defs/SynapseSpec"}, "x-semio-ui": ui("Synapses", "Synapsen", group="record", order=20)}}, ["neurons", "synapses"]),
        "FlowNeuron": record({"id": text("ID", "ID", 10), "kind": text("Neuron kind", "Neuronenart", 20, "kind"), "params": ref("Dictionary", "Parameters", "Parameter", 30, "value"), "tree": nullable_ref("FlowTree", "Nested graph", "Verschachtelter Graph", 40)}, ["id", "kind", "params"]),
        "SynapseSpec": record({"id": text("ID", "ID", 10), "from": text("From", "Von", 20, "ports"), "to": text("To", "Nach", 30, "ports"), "fromPort": text("From port", "Ausgang", 40, "ports"), "toPort": text("To port", "Eingang", 50, "ports")}, ["id", "from", "to", "fromPort", "toPort"]),
        "CameraJson": record({"x": number("X", "X", 10, "camera", 1), "y": number("Y", "Y", 20, "camera", 1), "zoom": number("Zoom", "Zoom", 30, "camera")}, ["x", "y", "zoom"]),
        "WidgetLayout": record({"x": number("X", "X", 10, "position", 1), "y": number("Y", "Y", 20, "position", 1)}, ["x", "y"]),
        "FlowUi": record({"camera": ref("CameraJson", "Camera", "Kamera", 10, "camera"), "nodes": {"type": "object", "additionalProperties": {"$ref": "#/$defs/FlowNodeGui"}, "x-semio-ui": ui("Nodes", "Knoten", description=("Presentation per node id.", "Darstellung je Knoten-ID."), group="record", order=20)}, "previews": {"type": "array", "items": {"$ref": "#/$defs/FlowPreviewGui"}, "x-semio-ui": ui("Previews", "Vorschauen", group="record", order=30)}}, ["camera", "nodes", "previews"]),
        "FlowNodeGui": record({"layout": ref("WidgetLayout", "Position", "Position", 10, "position"), "chrome": ref("NodeChrome", "Chrome", "Rahmen", 20, "appearance")}, ["layout", "chrome"]),
        "NodeChrome": {
            "oneOf": [
                variant("plain", "Plain", "Schlicht", {"preview": {"type": "boolean", "x-semio-ui": ui("Preview", "Vorschau", "toggle", group="appearance", order=10)}}),
                variant("slider", "Slider", "Schieberegler", {"label": text("Label", "Bezeichnung", 10, "appearance"), "min": number("Minimum", "Minimum", 20, "range"), "max": number("Maximum", "Maximum", 30, "range"), "step": number("Step", "Schrittweite", 40, "range"), "value": number("Value", "Wert", 50)}),
                variant("note", "Note", "Notiz", {"text": text("Text", "Text", 10, "value", widget="multiline")}),
                variant("image", "Image", "Bild", {"src": text("Image source", "Bildquelle", 10, "source")}),
                variant("variable", "Variable", "Variable", {"name": text("Name", "Name", 10), "schema": text("Schema", "Schema", 20, "kind")}),
            ],
        },
        "FlowPreviewGui": record({"id": text("ID", "ID", 10), "source": nullable_ref("FlowChannelRef", "Source", "Quelle", 20, "ports"), "mode": text("Mode", "Modus", 30, "appearance"), "preview": ref("Dictionary", "Preview values", "Vorschauwerte", 40, "value"), "expanded": strings("Expanded entries", "Aufgeklappte Einträge", 50, "appearance"), "layout": nullable_ref("WidgetLayout", "Position", "Position", 60, "position")}, ["id", "mode", "preview", "expanded"]),
        "FlowChannelRef": record({"neuron": text("Neuron", "Neuron", 10, "ports"), "channel": text("Channel", "Kanal", 20, "ports")}, ["neuron", "channel"]),
    }


RETIRED_WIDGET_DEFS = ("Tree", "Neuron", "Synapse")


def widget_union(document):
    widget = document["properties"]["widget"]
    document["properties"]["widget"] = {"$ref": "#/$defs/Widget", "x-semio-ui": widget["x-semio-ui"]}
    defs = document.setdefault("$defs", {})
    for name in RETIRED_WIDGET_DEFS:
        defs.pop(name, None)
    defs.update(widget_defs())
    ordered = {key: value for key, value in document.items() if key != "$defs"}
    items = list(ordered.items())
    index = [key for key, _ in items].index("properties") + 1
    items.insert(index, ("$defs", defs))
    document.clear()
    document.update(items)
#endregion 🔖️WidgetUnion


#region 🔖️RasterLayerNode
RASTER_BRANCH_LABELS = {"pixel": ("Pixel layer", "Pixelebene"), "group": ("Group", "Gruppe"), "adjustment": ("Adjustment layer", "Einstellungsebene")}
RASTER_FIELDS = {
    "id": ui("ID", "ID", "text", description=("Unique layer id in this image.", "Eindeutige Ebenen-ID in diesem Bild."), group="identity", order=10),
    "name": ui("Name", "Name", "text", group="identity", order=20),
    "visible": ui("Visible", "Sichtbar", "toggle", group="state", order=30),
    "locked": ui("Locked", "Gesperrt", "toggle", group="state", order=40),
    "opacity": ui("Opacity", "Deckkraft", "slider", step=0.01, precision=2, softMin=0, softMax=1, snaps=[0, 0.25, 0.5, 0.75, 1], displayUnit="%", displayFactor=100, group="appearance", order=50),
    "blendMode": ui("Blend mode", "Füllmethode", "text", description=("Compositing mode, e.g. normal, multiply or screen.", "Mischmodus, z. B. normal, multiply oder screen."), group="appearance", order=60),
    "transform": ui("Transform", "Transformation", description=("Affine placement of the layer.", "Affine Platzierung der Ebene."), group="position", order=70),
    "mask": ui("Mask", "Maske", description=("Layer mask; empty means none.", "Ebenenmaske; leer bedeutet keine."), group="mask", order=80),
    "width": ui("Width", "Breite", "stepper", unit="px", step=1, precision=0, group="size", order=90),
    "height": ui("Height", "Höhe", "stepper", unit="px", step=1, precision=0, group="size", order=100),
    "imageKey": ui("Image", "Bild", "text", description=("Key of the image asset holding the pixels; empty means blank.", "Schlüssel des Bild-Assets mit den Pixeln; leer bedeutet leer."), group="source", order=110),
    "children": ui("Layers", "Ebenen", description=("The layers inside the group, bottom to top.", "Die Ebenen in der Gruppe, von unten nach oben."), group="record", order=120),
    "adjustmentKind": ui("Adjustment", "Korrektur", "text", description=("Adjustment kind, e.g. brightness, contrast or hue.", "Korrekturart, z. B. brightness, contrast oder hue."), group="kind", order=130),
    "params": ui("Parameters", "Parameter", description=("Adjustment parameters by name.", "Korrekturparameter je Name."), group="value", order=140),
}


def raster_layer_node(document):
    for branch in document["$defs"]["RasterLayerNode"]["oneOf"]:
        kind = branch["properties"]["kind"]["const"]
        branch["properties"]["kind"]["x-semio-ui"] = discriminator()
        for key, value in branch["properties"].items():
            if key != "kind":
                value["x-semio-ui"] = RASTER_FIELDS[key]
        branch["x-semio-ui"] = {"label": label(*RASTER_BRANCH_LABELS[kind])}
#endregion 🔖️RasterLayerNode


#region 🔖️RustDriven
def load_scanner():
    spec = importlib.util.spec_from_file_location("parity", os.path.join(HERE, "🧪️w2-s-design-rust-parity.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def schema_path(leaf_directory):
    descriptor = json.loads(read(f"{leaf_directory}/🔣️.json"))
    return f"{leaf_directory}/{descriptor['payloadSchema']}"


def walk_to(document, pointer):
    node, parent, key = document, None, None
    for segment in pointer.strip("/").split("/"):
        parent = node
        if segment == "-":
            node, key = node["items"], None
        else:
            node, key = node["properties"][segment], segment
    return node, parent, key


def make_nullable(node):
    if "type" in node:
        kinds = node["type"] if isinstance(node["type"], list) else [node["type"]]
        node["type"] = kinds + ([] if "null" in kinds else ["null"])
        if "enum" in node and None not in node["enum"]:
            node["enum"].append(None)
    elif "oneOf" in node or "anyOf" in node:
        branches = node["oneOf" if "oneOf" in node else "anyOf"]
        if {"type": "null"} not in branches:
            branches.append({"type": "null"})
    elif "$ref" in node:
        reference = node.pop("$ref")
        rest = dict(node)
        node.clear()
        node["anyOf"] = [{"$ref": reference}, {"type": "null"}]
        node.update(rest)


def rust_driven(scanner):
    rows = [row for root in ROOTS for row in scanner.scan(root, {})]
    by_file = {}
    for row in rows:
        if row["class"] in ("nullable", "required"):
            by_file.setdefault(schema_path(row["leaf"]), []).append(row)
    for path, file_rows in by_file.items():
        def change(document, file_rows=file_rows):
            for row in file_rows:
                node, parent, key = walk_to(document, row["field"])
                if row["class"] == "nullable":
                    make_nullable(node)
                elif "always emitted" in row["detail"]:
                    names = parent.setdefault("required", [])
                    if key not in names:
                        names.append(key)
                elif key in parent.get("required", []):
                    parent["required"].remove(key)
        edit_json(path, change)
    return rows
#endregion 🔖️RustDriven


#region 🔖️ColourAndSnaps
UNIT_ITEMS = {"type": "number", "minimum": 0, "maximum": 1}
GRID_CONFIG = {P3: "gridSpacing", P5: "gridSpacing"}
STEP_SNAPPED = ("✏️s/🔌️plugins/🎥️shooting/", "✏️s/🔌️plugins/📐️cad/", "✏️s/🔌️plugins/🧱️block/")
POSITIONAL = {"Origin", "Position", "Point", "3D position", "Offset X", "Offset Y", "Offset Z"}


def colour_and_snaps(path, document):
    def visit(node, trail):
        if isinstance(node, dict):
            facets = node.get("x-semio-ui")
            if isinstance(facets, dict) and facets.get("label", {}).get("en") in ("Color", "Colour") and (node.get("type") == "array" or "$ref" in node):
                facets["widget"] = "color"
                facets["description"] = label("Red, green, blue and alpha, each from 0 to 1.", "Rot, Grün, Blau und Alpha, jeweils von 0 bis 1.") if node.get("maxItems", 4) == 4 else label("Red, green and blue, each from 0 to 1.", "Rot, Grün und Blau, jeweils von 0 bis 1.")
                if node.get("type") == "array":
                    node["items"] = dict(UNIT_ITEMS)
            if isinstance(facets, dict) and facets.get("label", {}).get("en") in POSITIONAL and "snapSource" not in facets and not any("amera" in step for step in trail):
                grid = next((key for base, key in GRID_CONFIG.items() if path.startswith(base)), None)
                vector = facets.get("widget") == "vector" and node.get("minItems") == 3
                number = facets.get("widget") == "stepper" and node.get("type") == "number" and facets["label"]["en"].startswith("Offset")
                if grid is not None and vector and facets.get("unit") == "m":
                    facets["snapSource"] = {"config": grid}
                elif grid is None and path.startswith(STEP_SNAPPED) and (vector or number):
                    facets.setdefault("step", 0.1)
                    facets["snapSource"] = {"step": True}
            for key, value in node.items():
                visit(value, trail + (key,))
        elif isinstance(node, list):
            for value in node:
                visit(value, trail)

    visit(document, ())


def colour_defs(document):
    colour = document.get("$defs", {}).get("Color")
    if isinstance(colour, dict) and colour.get("type") == "array":
        colour["items"] = dict(UNIT_ITEMS)


def colour_pass():
    for root in ROOTS:
        for directory, subdirectories, files in os.walk(os.path.join(REPO, root)):
            subdirectories[:] = [name for name in subdirectories if name not in ("node_modules", "target", "dist", "🗑️generated", "🧪️tests") and not name.endswith("fixtures")]
            if "🔣️.json" not in files or not directory.endswith("/🧬️schema"):
                continue
            path = os.path.relpath(os.path.join(directory, "🔣️.json"), REPO)
            text = read(path)
            document = json.loads(text)
            before = json.dumps(document, sort_keys=True)
            colour_defs(document)
            colour_and_snaps(path, document)
            if json.dumps(document, sort_keys=True) == before:
                continue
            if dumps(json.loads(text)) == text:
                edit_json(path, lambda target: (colour_defs(target), colour_and_snaps(path, target)))
            else:
                print(f"[w2-s-c] hand-formatted {path} needs the colour/snap edit by text")


def replace_layer_fill_text(text):
    text = text.replace('"Color": {"type": "array", "items": {"type": "number"}, "minItems": 4, "maxItems": 4}', '"Color": {"type": "array", "items": {"type": "number", "minimum": 0, "maximum": 1}, "minItems": 4, "maxItems": 4}')
    return text.replace('"color": {"$ref": "#/$defs/Color", "x-semio-ui": {"widget": "vector",', '"color": {"$ref": "#/$defs/Color", "x-semio-ui": {"widget": "color",')
#endregion 🔖️ColourAndSnaps


#region 🔖️SnakeToCamel
SHOOTING_KEYS = ["asset_ids", "asset_id", "new_enabled", "new_shape", "new_name", "new_elevation", "new_url", "new_camera", "saved_camera", "shot_id", "new_label", "new_intensity", "new_width", "new_height", "to_index", "new_format", "new_azimuth", "new_roughness"]
PROCEDURAL_KEYS = ["question_id", "preview_text"]


def camel(key):
    head, *rest = key.split("_")
    return head + "".join(part[:1].upper() + part[1:] for part in rest)


def rename_keys(value, keys):
    if isinstance(value, dict):
        return {(camel(key) if key in keys else key): rename_keys(item, keys) for key, item in value.items()}
    if isinstance(value, list):
        return [rename_keys(item, keys) for item in value]
    return value


def quoted_keys(text, keys):
    pattern = re.compile(r'(\\?")(' + "|".join(sorted(keys, key=len, reverse=True)) + r')(\\?"\s*:)')
    return pattern.sub(lambda match: match.group(1) + camel(match.group(2)) + match.group(3), text)


def fixture_files(root):
    for directory, subdirectories, files in os.walk(os.path.join(REPO, root)):
        subdirectories[:] = [name for name in subdirectories if name not in ("node_modules", "target", "dist", "🗑️generated")]
        if directory.endswith("🦠️mutation") and "🔣️.json" in files:
            yield os.path.relpath(os.path.join(directory, "🔣️.json"), REPO)


def snake_to_camel():
    for root, keys in ((SH, SHOOTING_KEYS), (G2, PROCEDURAL_KEYS), (G3, PROCEDURAL_KEYS)):
        for path in fixture_files(root):
            text = read(path)
            document = json.loads(text)
            if rename_keys(document, set(keys)) == document:
                continue
            edit_text(path, lambda current: quoted_keys(current, keys))
#endregion 🔖️SnakeToCamel


#region 🔖️RustSources
LEAF_ATTRIBUTE = "#[mutation_leaf(contract = ::protocol)]\n"


def edit_rust(path, change):
    text = read(path)
    rendered = change(text)
    if rendered != text:
        written.append(path)
        if not DRY:
            with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
                handle.write(rendered)


def camel_leaf(text, serde):
    if "dsl::MutationLeaf" not in text or "rename_all" in text or text.count(LEAF_ATTRIBUTE) != 1:
        return text
    lines = ('#[cfg_attr(test, serde(rename_all = "camelCase"))]\n' if serde else "") + '#[value(rename_all = "camelCase")]\n'
    return text.replace(LEAF_ATTRIBUTE, LEAF_ATTRIBUTE + lines)


def leaf_sources(base):
    for directory, subdirectories, files in os.walk(os.path.join(REPO, base)):
        subdirectories[:] = [name for name in subdirectories if name not in ("node_modules", "target", "dist", "🗑️generated", "🔺️diff", "↩️inverse", "🧪️tests", "💾️binary", "📝️text") and not name.endswith("fixtures")]
        if "🦀️.rs" in files and "🔣️.json" in files and "/🧬️mutations/" in directory:
            yield os.path.relpath(os.path.join(directory, "🦀️.rs"), REPO)


def test_sources(base):
    for directory, subdirectories, files in os.walk(os.path.join(REPO, base)):
        subdirectories[:] = [name for name in subdirectories if name not in ("node_modules", "target", "dist", "🗑️generated")]
        if "🦀️.rs" in files and "/🧪️tests/" in directory + "/":
            yield os.path.relpath(os.path.join(directory, "🦀️.rs"), REPO)


def rust_sources():
    for path in leaf_sources(f"{SH}/🧬️schema/🧬️mutations"):
        edit_rust(path, lambda text: camel_leaf(text, True))
    for base in (f"{G2}/🧬️schema/🧬️mutations", f"{G2}/✏️editor/🫧️transient/🧬️schema/🧬️mutations", f"{G3}/✏️editor/🫧️transient/🧬️schema/🧬️mutations"):
        for path in leaf_sources(base):
            edit_rust(path, lambda text: camel_leaf(text, False))
    for path in test_sources(f"{SH}/🧬️schema/🧬️mutations"):
        edit_rust(path, lambda text: quoted_keys(text, SHOOTING_KEYS))
#endregion 🔖️RustSources


#region 🔖️ProceduralProjections
WIDGET_TS = '''/** 🎛️ One flow widget, internally tagged by `kind` — the flow `Widget` enum's wire form (`tag = "kind"`, camelCase). */
export type Widget =
  | { kind: "neuron"; id: string; neuronKind: string; params: Record<string, unknown>; inputPorts: string[]; outputPorts: string[]; preview: boolean }
  | { kind: "inputSlider"; id: string; label: string; value: number; min: number; max: number; step: number }
  | { kind: "inputNote"; id: string; text: string }
  | { kind: "inputImage"; id: string; src: string }
  | { kind: "variable"; id: string; name: string; schema: string }
  | { kind: "outputPreview"; id: string; preview: Record<string, unknown>; expanded: string[] }
  | { kind: "outputAction"; id: string; action: string }
  | { kind: "outputExport"; id: string; format: string }
  | { kind: "cluster"; id: string; name: string; tree: FlowTree; flow: FlowUi };
export type FlowTree = { neurons: FlowNeuron[]; synapses: SynapseSpec[] };
export type FlowNeuron = { id: string; kind: string; params: Record<string, unknown>; tree: FlowTree | null };
export type FlowUi = { camera: CameraJson; nodes: Record<string, FlowNodeGui>; previews: FlowPreviewGui[] };
export type FlowNodeGui = { layout: WidgetLayout; chrome: NodeChrome };
export type NodeChrome =
  | { kind: "plain"; preview: boolean }
  | { kind: "slider"; label: string; min: number; max: number; step: number; value: number }
  | { kind: "note"; text: string }
  | { kind: "image"; src: string }
  | { kind: "variable"; name: string; schema: string };
export type FlowPreviewGui = { id: string; source: FlowChannelRef | null; mode: string; preview: Record<string, unknown>; expanded: string[]; layout: WidgetLayout | null };
export type FlowChannelRef = { neuron: string; channel: string };'''

WIDGET_GRAPHQL = '''"""🎛️ One flow widget, discriminated by `kind`."""
union Widget = NeuronWidget | InputSliderWidget | InputNoteWidget | InputImageWidget | VariableWidget | OutputPreviewWidget | OutputActionWidget | OutputExportWidget | ClusterWidget
type NeuronWidget {
  kind: String!
  id: String!
  neuronKind: String!
  params: JSON!
  inputPorts: [String!]!
  outputPorts: [String!]!
  preview: Boolean!
}
type InputSliderWidget {
  kind: String!
  id: String!
  label: String!
  value: Float!
  min: Float!
  max: Float!
  step: Float!
}
type InputNoteWidget {
  kind: String!
  id: String!
  text: String!
}
type InputImageWidget {
  kind: String!
  id: String!
  src: String!
}
type VariableWidget {
  kind: String!
  id: String!
  name: String!
  schema: String!
}
type OutputPreviewWidget {
  kind: String!
  id: String!
  preview: JSON!
  expanded: [String!]!
}
type OutputActionWidget {
  kind: String!
  id: String!
  action: String!
}
type OutputExportWidget {
  kind: String!
  id: String!
  format: String!
}
type ClusterWidget {
  kind: String!
  id: String!
  name: String!
  tree: FlowTree!
  flow: FlowUi!
}
type FlowTree {
  neurons: [FlowNeuron!]!
  synapses: [SynapseSpec!]!
}
type FlowNeuron {
  id: String!
  kind: String!
  params: JSON!
  tree: FlowTree
}
type FlowUi {
  camera: CameraJson!
  nodes: [FlowUiNodesEntry!]!
  previews: [FlowPreviewGui!]!
}
type FlowUiNodesEntry {
  key: String!
  value: FlowNodeGui!
}
type FlowNodeGui {
  layout: WidgetLayout!
  chrome: NodeChrome!
}
union NodeChrome = PlainChrome | SliderChrome | NoteChrome | ImageChrome | VariableChrome
type PlainChrome {
  kind: String!
  preview: Boolean!
}
type SliderChrome {
  kind: String!
  label: String!
  min: Float!
  max: Float!
  step: Float!
  value: Float!
}
type NoteChrome {
  kind: String!
  text: String!
}
type ImageChrome {
  kind: String!
  src: String!
}
type VariableChrome {
  kind: String!
  name: String!
  schema: String!
}
type FlowPreviewGui {
  id: String!
  source: FlowChannelRef
  mode: String!
  preview: JSON!
  expanded: [String!]!
  layout: WidgetLayout
}
type FlowChannelRef {
  neuron: String!
  channel: String!
}'''

WIDGET_PROTO = '''message Widget {
  oneof kind {
    NeuronWidget neuron = 1;
    InputSliderWidget input_slider = 2;
    InputNoteWidget input_note = 3;
    InputImageWidget input_image = 4;
    VariableWidget variable = 5;
    OutputPreviewWidget output_preview = 6;
    OutputActionWidget output_action = 7;
    OutputExportWidget output_export = 8;
    ClusterWidget cluster = 9;
  }
}
message NeuronWidget {
  string id = 1;
  string neuron_kind = 2;
  google.protobuf.Struct params = 3;
  repeated string input_ports = 4;
  repeated string output_ports = 5;
  bool preview = 6;
}
message InputSliderWidget {
  string id = 1;
  string label = 2;
  double value = 3;
  double min = 4;
  double max = 5;
  double step = 6;
}
message InputNoteWidget {
  string id = 1;
  string text = 2;
}
message InputImageWidget {
  string id = 1;
  string src = 2;
}
message VariableWidget {
  string id = 1;
  string name = 2;
  string schema = 3;
}
message OutputPreviewWidget {
  string id = 1;
  google.protobuf.Struct preview = 2;
  repeated string expanded = 3;
}
message OutputActionWidget {
  string id = 1;
  string action = 2;
}
message OutputExportWidget {
  string id = 1;
  string format = 2;
}
message ClusterWidget {
  string id = 1;
  string name = 2;
  FlowTree tree = 3;
  FlowUi flow = 4;
}
message FlowTree {
  repeated FlowNeuron neurons = 1;
  repeated SynapseSpec synapses = 2;
}
message FlowNeuron {
  string id = 1;
  string kind = 2;
  google.protobuf.Struct params = 3;
  optional FlowTree tree = 4;
}
message FlowUi {
  CameraJson camera = 1;
  map<string, FlowNodeGui> nodes = 2;
  repeated FlowPreviewGui previews = 3;
}
message FlowNodeGui {
  WidgetLayout layout = 1;
  NodeChrome chrome = 2;
}
message NodeChrome {
  oneof kind {
    PlainChrome plain = 1;
    SliderChrome slider = 2;
    NoteChrome note = 3;
    ImageChrome image = 4;
    VariableChrome variable = 5;
  }
}
message PlainChrome {
  bool preview = 1;
}
message SliderChrome {
  string label = 1;
  double min = 2;
  double max = 3;
  double step = 4;
  double value = 5;
}
message NoteChrome {
  string text = 1;
}
message ImageChrome {
  string src = 1;
}
message VariableChrome {
  string name = 1;
  string schema = 2;
}
message FlowPreviewGui {
  string id = 1;
  optional FlowChannelRef source = 2;
  string mode = 3;
  google.protobuf.Struct preview = 4;
  repeated string expanded = 5;
  optional WidgetLayout layout = 6;
}
message FlowChannelRef {
  string neuron = 1;
  string channel = 2;
}'''

WIDGET_TS_PARSE = '''/** 🎛️ The member kinds every widget kind carries beside `kind` and `id`. */
const WIDGET_MEMBERS: Readonly<Record<Widget["kind"], Readonly<Record<string, "string" | "number" | "boolean" | "strings" | "object">>>> = {
  neuron: { neuronKind: "string", params: "object", inputPorts: "strings", outputPorts: "strings", preview: "boolean" },
  inputSlider: { label: "string", value: "number", min: "number", max: "number", step: "number" },
  inputNote: { text: "string" },
  inputImage: { src: "string" },
  variable: { name: "string", schema: "string" },
  outputPreview: { preview: "object", expanded: "strings" },
  outputAction: { action: "string" },
  outputExport: { format: "string" },
  cluster: { name: "string", tree: "object", flow: "object" },
};

export function parseWidget(value: unknown, at = "$"): Widget {
  const row = PREFIXGuardObject(value, at);
  const kind = PREFIXGuardMember(row["kind"], `${at}.kind`, Object.keys(WIDGET_MEMBERS) as Widget["kind"][]);
  PREFIXGuardString(row["id"], `${at}.id`);
  for (const [member, shape] of Object.entries(WIDGET_MEMBERS[kind])) {
    const where = `${at}.${member}`;
    if (shape === "string") PREFIXGuardString(row[member], where);
    else if (shape === "number") PREFIXGuardNumber(row[member], where);
    else if (shape === "boolean") PREFIXGuardBoolean(row[member], where);
    else if (shape === "strings") PREFIXGuardArray(row[member], where).forEach((item, index) => PREFIXGuardString(item, `${where}[${index}]`));
    else PREFIXGuardObject(row[member], where);
  }
  return row as unknown as Widget;
}'''


CREATE_WIDGET_TWINS = (
    (
        "/** 🌱 generation2d create-widget payload — mirrors `CreateWidget` (…/🌱️create-widget/🦠️mutation/🦀️.rs:16-19). */\nexport type Widget = string;\n\nexport interface CreateWidget {",
        'import type { Widget } from "../../../🟦️.ts";\n\nexport type { Widget };\n\n/** 🌱 generation2d create-widget payload — mirrors `CreateWidget` (…/🌱️create-widget/🦠️mutation/🦀️.rs:16-19). */\nexport interface CreateWidget {',
    ),
    (
        "/** ➕ generation3d direct `create-widget` payload mirror of `CreateWidget`. */\n/** @description Opaque `flow::Widget` — JSON text (tagged union serialized by `kind`). */\nexport type Widget = string;\n\n/** 🔎️ Extracts the shared `id` field every `Widget` variant carries, by parsing its JSON text — mirror of `generation3d::widget_id`. */\nexport function widgetId(widget: Widget): string {\n  return (JSON.parse(widget) as { id: string }).id;\n}\n\nexport interface CreateWidget {",
        'import type { Widget } from "../../../🟦️.ts";\n\nexport type { Widget };\n\n/** 🔎️ The shared `id` every `Widget` variant carries — mirror of `generation3d::widget_id`. */\nexport function widgetId(widget: Widget): string {\n  return widget.id;\n}\n\n/** ➕ generation3d direct `create-widget` payload mirror of `CreateWidget`. */\nexport interface CreateWidget {',
    ),
)


def create_widget_twin(text):
    for old, new in CREATE_WIDGET_TWINS:
        text = text.replace(old, new)
    return text


def procedural_projections():
    for base, name in ((G2, "Generation2d"), (G3, "Generation3d")):
        schema = f"{base}/🧬️schema"

        def artifact(document):
            defs = document["$defs"]
            for key, definition in widget_defs().items():
                if key not in ("CameraJson", "WidgetLayout", "SynapseSpec"):
                    defs[key] = definition
            generation = defs["FormGeneration"]
            if "valuesJson" in generation["properties"]:
                generation["properties"] = renamed(generation["properties"], "valuesJson", "values")
                generation["required"] = ["values" if key == "valuesJson" else key for key in generation["required"]]
            generation["properties"]["values"] = {"type": "object", "additionalProperties": {"description": "the answer, any JSON value"}}

        edit_json(f"{schema}/🔣️.json", artifact)
        for twin in (f"{schema}/🟦️.ts", f"{schema}/📸️snapshot/🟦️.ts", f"{schema}/🔺️diff/🟦️.ts"):
            edit_rust(twin, lambda text: text.replace("/** @description Polymorphic flow widget — JSON blob. */\nexport type Widget = string;", WIDGET_TS))
        prefix = f"procedural{name}ArtifactGuard"

        def artifact_twin(text, prefix=prefix):
            old_parse = f'export function parseWidget(value: unknown, at = "$"): Widget {{\n  return {prefix}String(value, `${{at}}`);\n}}'
            text = text.replace(old_parse, WIDGET_TS_PARSE.replace("PREFIXGuard", prefix))
            text = text.replace(f'    valuesJson: {prefix}String(row["valuesJson"], `${{at}}.valuesJson`),', f'    values: {prefix}Object(row["values"], `${{at}}.values`) as Record<string, unknown>,')
            old_state = f'    generations: {prefix}Array(row["generations"], `${{at}}.generations`).map((item, index) => parseFormGeneration(item, `${{at}}.generations[${{index}}]`)),\n    previewText:'
            new_state = f'    generations: {prefix}Array(row["generations"], `${{at}}.generations`).map((item, index) => parseFormGeneration(item, `${{at}}.generations[${{index}}]`)),\n    selectedGenerationId: row["selectedGenerationId"] === undefined ? undefined : {prefix}String(row["selectedGenerationId"], `${{at}}.selectedGenerationId`),\n    previewText:'
            return text.replace(old_state, new_state)

        edit_rust(f"{schema}/🟦️.ts", artifact_twin)
        edit_rust(leaf(base, "create-widget").replace("🧬️schema/🔣️.json", "🦠️mutation/🟦️.ts"), create_widget_twin)

        def graphql(text):
            if "union Widget" not in text:
                text = text.replace("scalar Widget\n", WIDGET_GRAPHQL + "\n")
            if "scalar JSON" not in text:
                text = text.replace(WIDGET_GRAPHQL.split("\n")[0], "scalar JSON\n" + WIDGET_GRAPHQL.split("\n")[0], 1)
            return text.replace("  valuesJson: String!", "  values: JSON!")

        for file in (f"{schema}/🔗️.graphql", f"{schema}/📸️snapshot/🔗️.graphql", f"{schema}/🔺️diff/🔗️.graphql", f"{schema}/🧬️mutations/🕸️.graphql"):
            edit_rust(file, graphql)

        def proto(text):
            text = text.replace("message Widget {\n  string json = 1;\n}", WIDGET_PROTO)
            text = text.replace("  string values_json = 3;", "  google.protobuf.Struct values = 3;")
            if 'import "google/protobuf/struct.proto";' not in text:
                head, _, rest = text.partition("\n\n")
                text = f'{head}\n\nimport "google/protobuf/struct.proto";\n\n{rest}'
            return text

        for file in (f"{schema}/🛰️.proto", f"{schema}/📸️snapshot/🛰️.proto", f"{schema}/🔺️diff/🛰️.proto", f"{schema}/🧬️mutations/🛰️.proto"):
            edit_rust(file, proto)
#endregion 🔖️ProceduralProjections


#region 🔖️Draft07
DRAFT_2020 = '"$schema": "https://json-schema.org/draft/2020-12/schema"'
DRAFT_07 = '"$schema": "http://json-schema.org/draft-07/schema#"'
AJV_2020_IMPORTS = (('import Ajv from "ajv/dist/2020.js";', 'import Ajv from "ajv";'), ('import Ajv from "ajv/dist/2020";', 'import Ajv from "ajv";'), ('import Ajv2020 from "ajv/dist/2020";', 'import Ajv from "ajv";'), ("new Ajv2020(", "new Ajv("))


def draft07_documents():
    """📐️ Every schema document in scope moves to the repository dialect draft-07 (2020-12 `prefixItems` + `items: false`
    becomes the draft-07 tuple `items: [...]` + `additionalItems: false`), and every Ajv consumer of one moves to the draft-07 Ajv —
    except a consumer that also compiles the framework's 2020-12 UI schemas, which keeps Ajv 2020 with the draft-07 meta-schema added."""
    for root in ROOTS:
        for directory, subdirectories, files in os.walk(os.path.join(REPO, root)):
            subdirectories[:] = [name for name in subdirectories if name not in ("node_modules", "target", "dist", "🗑️generated")]
            for name in files:
                path = os.path.relpath(os.path.join(directory, name), REPO)
                if name.endswith(".json") and DRAFT_2020 in read(path):
                    edit_text(path, lambda text: re.sub(r'"prefixItems": (\[[^\]]*\]), "items": false', r'"items": \1, "additionalItems": false', text.replace(DRAFT_2020, DRAFT_07)))
                elif name == "🟦️.ts" and "ajv/dist/2020" in read(path) and "json-schema-draft-07.json" not in read(path):
                    def swap(text):
                        for old, new in AJV_2020_IMPORTS:
                            text = text.replace(old, new)
                        return text
                    edit_rust(path, swap)
#endregion 🔖️Draft07


#region 🔖️FormsCamel
FORMS_KEYS = ["step_id", "new_title", "new_description", "block_id", "to_step_id", "to_index"]


def words_to_camel(text, keys):
    return re.sub(r"(?<![A-Za-z0-9_])(" + "|".join(sorted(keys, key=len, reverse=True)) + r")(?![A-Za-z0-9_])", lambda match: camel(match.group(1)), text)


def quoted_strings(text, keys):
    return re.sub(r'"(' + "|".join(sorted(keys, key=len, reverse=True)) + r')"', lambda match: f'"{camel(match.group(1))}"', text)


def forms_camel():
    mutations = os.path.join(REPO, FO, "🧬️schema", "🧬️mutations")
    for name in sorted(os.listdir(mutations)):
        leaf_directory = f"{FO}/🧬️schema/🧬️mutations/{name}"
        if not os.path.isfile(os.path.join(REPO, leaf_directory, "🦠️mutation", "🦀️.rs")):
            continue
        edit_rust(f"{leaf_directory}/🦠️mutation/🦀️.rs", lambda text: camel_leaf(text, False))
        if os.path.isfile(os.path.join(REPO, leaf_directory, "🦠️mutation", "🟦️.ts")):
            edit_rust(f"{leaf_directory}/🦠️mutation/🟦️.ts", lambda text: words_to_camel(text, FORMS_KEYS))

        def schema(document):
            for key in FORMS_KEYS:
                rename_property(document, key, camel(key))

        edit_json(f"{leaf_directory}/🧬️schema/🔣️.json", schema)
    edit_rust(f"{FO}/🧬️schema/🧬️mutations/🟦️.ts", lambda text: words_to_camel(text, FORMS_KEYS))
    edit_rust(f"{FO}/🧪️tests/🌵️mutate-forms-1/🐍️.py", lambda text: quoted_strings(text, FORMS_KEYS))
    for path in fixture_files(FO):
        if rename_keys(json.loads(read(path)), set(FORMS_KEYS)) != json.loads(read(path)):
            edit_text(path, lambda current: quoted_keys(current, FORMS_KEYS))
    for path in test_sources(FO):
        edit_rust(path, lambda text: quoted_keys(text, FORMS_KEYS))
#endregion 🔖️FormsCamel


def main():
    rust_sources()
    forms_camel()
    procedural_projections()
    draft07_documents()
    structural()
    edit_text(leaf(f"{DR}/🎨️style", "replace-layer-fill"), replace_layer_fill_text)
    colour_pass()
    snake_to_camel()
    rows = rust_driven(load_scanner())
    remaining = [row for row in (row for root in ROOTS for row in load_scanner().scan(root, {})) if row["class"] != "unparsed"] if not DRY else rows
    for path in written:
        print(f"[w2-s-c] {'would write' if DRY else 'wrote'} {path}")
    print(f"[w2-s-c] {len(written)} file(s) {'would change' if DRY else 'changed'}; {len(remaining)} Rust parity finding(s) {'before the Rust-driven pass' if DRY else 'remain'}")
    for row in remaining:
        print(f"  [{row['class']}] {row['leaf'].split('/🧬️mutations/')[-1]} :: {row['field']} — {row['detail']}")


if __name__ == "__main__":
    main()
