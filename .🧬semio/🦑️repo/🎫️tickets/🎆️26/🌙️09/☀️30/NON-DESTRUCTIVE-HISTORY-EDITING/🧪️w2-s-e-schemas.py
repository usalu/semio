#!/usr/bin/env python3
"""🧬️ W2-S-E leaf schema parity for imperative, sequence, dag, flow (plugin + os vcs), os workflow and os store.

Every edit makes a leaf payload schema describe exactly what `payload_value()` emits (the Rust struct as `#[derive(ToValue)]`
wires it), keeps every existing `x-semio-ui` annotation and annotates what it adds (en/de):

- neural `Dictionary` params (imperative, sequence) → an object of neural values (atom or nested dictionary);
- `PathRef.owner`/`slot` (`Option<String>`, emitted `null`) admit `null`;
- dag `replace-node-kind.newKind` → the real `DagNodeKind` union (`kind` tag, `IoPortSpec` ports, preview/media records);
- flow `create-widget`/`replace-widget.widget` → the flow host wire `Widget` union of the os flow vcs snapshot schema, whose
  neuron ports wire camelCase (`rename_all_fields = "camelCase"`) and whose `HostDocument` members get labels;
- the os flow vcs leaves drop `propertyNames` (it excluded the `operation` tag the aggregate rule declared);
- os workflow leaves: camelCase member names (the leaf structs are `rename_all = "camelCase"`) and full node / edge / binding /
  input / parameter records instead of bare objects;
- os store: `SpaceCheckpoint.authors` items are `Author` records, `parentId` (skipped when `None`) is optional; the demo
  fixture leaf `delete-n` drops `propertyNames: false`, which refused the required `operation` tag.

Only canonical `json.dumps(indent=2, ensure_ascii=False)` files are rewritten; any other file aborts for a hand edit.
Idempotent; each file is re-read right before it is written. Usage: python3 🧪️w2-s-e-schemas.py [--apply]
"""
import copy
import json
import os
import sys

REPO = "/Users/ueli/Documents/semio"
APPLY = "--apply" in sys.argv
IMPERATIVE = "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
SEQUENCE = "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations"
DAG = "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
FLOW = "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
FLOW_VCS = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema"
WORKFLOW = "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations"
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
FLOW_WIDGET = "https://json.schemas.assets.semio-tech.com/os/flow/artifacts/flow/vcs/snapshot.json#/$defs/Widget"


def read(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return handle.read()


def ui(widget, en, de, description=None, **extra):
    node = {"widget": widget} if widget else {}
    node.update({"role": extra.pop("role", "value"), "label": {"en": en, "de": de}})
    if description is not None:
        node["description"] = {"en": description[0], "de": description[1]}
    node.update(extra)
    return node


def text(en, de, description=None):
    return {"type": "string", "x-semio-ui": ui("text", en, de, description)}


def ref(kind, en, de, description=None, role="value", **extra):
    return {"type": "string", "x-semio-ui": ui("reference", en, de, description, role=role, ref={"kind": kind, **extra})}


def number(en, de, description=None, **extra):
    return {"type": "number", "x-semio-ui": ui("stepper", en, de, description, **extra)}


def toggle(en, de, description=None, nullable=False):
    return {"type": ["boolean", "null"] if nullable else "boolean", "x-semio-ui": ui("toggle", en, de, description)}


def select(values, en, de, options, description=None):
    return {"type": "string", "enum": values, "x-semio-ui": ui("select" if len(values) > 3 else "segmented", en, de, description, options={value: {"en": option[0], "de": option[1]} for value, option in zip(values, options)})}


def record(properties, required, en=None, de=None, description=None):
    node = {"type": "object", "additionalProperties": False, "required": required, "properties": properties}
    if en is not None:
        node["x-semio-ui"] = ui(None, en, de, description)
    return node


def branch(tag, value, en, de, tag_label, fields, required):
    return {"type": "object", "additionalProperties": False, "required": [tag, *required], "properties": {tag: {"const": value, "x-semio-ui": ui("hidden", tag_label[0], tag_label[1], role="discriminator")}, **fields}, "x-semio-ui": ui(None, en, de)}


def labelled(node, en, de, description=None):
    return {**node, "x-semio-ui": ui(None, en, de, description)}


#region 🧠️NeuralValue
NEURAL_VALUE = {"anyOf": [{"type": "null"}, {"type": "boolean"}, {"type": "number"}, {"type": "string"}, {"type": "object", "additionalProperties": {"$ref": "#/$defs/NeuralValue"}}]}


def neural_params(node):
    node["additionalProperties"] = {"$ref": "#/$defs/NeuralValue"}


def with_neural(document):
    document.setdefault("$defs", {})["NeuralValue"] = NEURAL_VALUE
#endregion 🧠️NeuralValue


#region 🕸️DagNodeKind
KIND_TAG = ("Node kind", "Knotenart")
PORT = record({
    "id": text("Port ID", "Anschluss-ID"),
    "label": text("Label", "Beschriftung"),
    "code": text("Code", "Kürzel", ("Short code drawn on the port; empty omits it.", "Kurzzeichen am Anschluss; leer lässt es weg.")),
    "abbreviation": text("Abbreviation", "Abkürzung"),
    "fullName": text("Full name", "Vollständiger Name"),
    "type": text("Value type", "Werttyp"),
    "default": {"x-semio-ui": ui(None, "Default value", "Standardwert", ("Any JSON value.", "Beliebiger JSON-Wert."))},
    "value": {"x-semio-ui": ui(None, "Value", "Wert", ("Any JSON value.", "Beliebiger JSON-Wert."))},
    "connected": toggle("Connected", "Verbunden"),
    "resourceKind": text("Resource kind", "Ressourcenart", ("Artifact kind the port carries.", "Artefaktart, die der Anschluss trägt.")),
    "cardinality": text("Cardinality", "Kardinalität", ("! one, ? optional, * many.", "! genau eins, ? optional, * viele.")),
    "shape": select(["semicircle", "triangle"], "Port shape", "Anschlussform", [("Semicircle", "Halbkreis"), ("Triangle", "Dreieck")]),
    "visible": toggle("Visible", "Sichtbar"),
    "resolved": toggle("Resolved", "Aufgelöst", ("Whether the port type is resolved.", "Ob der Anschlusstyp aufgelöst ist.")),
}, ["id", "label"])
PORTS = lambda en, de: {"type": "array", "items": {"$ref": "#/$defs/IoPortSpec"}, "x-semio-ui": ui(None, en, de)}
ONE_PORT = lambda en, de: {"$ref": "#/$defs/IoPortSpec", "x-semio-ui": ui(None, en, de)}
PREVIEW_TAG = ("Preview type", "Vorschauart")
PREVIEW = {"oneOf": [
    branch("variant", "empty", "Empty", "Leer", PREVIEW_TAG, {}, []),
    branch("variant", "scalar", "Scalar", "Skalar", PREVIEW_TAG, {"text": text("Text", "Text")}, ["text"]),
    branch("variant", "image", "Image", "Bild", PREVIEW_TAG, {"src": text("Source", "Quelle")}, ["src"]),
    branch("variant", "tree", "Tree", "Baum", PREVIEW_TAG, {"json": {"x-semio-ui": ui(None, "JSON", "JSON", ("Any JSON value.", "Beliebiger JSON-Wert."))}}, ["json"]),
]}
MEDIA = record({"kind": select(["image", "svg", "pdf", "video"], "Media kind", "Medienart", [("Image", "Bild"), ("SVG", "SVG"), ("PDF", "PDF"), ("Video", "Video")]), "src": text("Source", "Quelle")}, ["kind", "src"])
DAG_NODE_KIND = [
    branch("kind", "computation", "Computation", "Berechnung", KIND_TAG, {"inputs": PORTS("Inputs", "Eingänge"), "outputs": PORTS("Outputs", "Ausgänge"), "variadic_inputs": toggle("Variadic inputs", "Variable Eingänge"), "variadic_outputs": toggle("Variadic outputs", "Variable Ausgänge")}, ["inputs", "outputs"]),
    branch("kind", "slider", "Slider", "Schieberegler", KIND_TAG, {"min": number("Minimum", "Minimum"), "max": number("Maximum", "Maximum"), "step": number("Step", "Schrittweite"), "value": number("Value", "Wert"), "output": ONE_PORT("Output", "Ausgang")}, ["min", "max", "step", "value", "output"]),
    branch("kind", "select", "Select", "Auswahl", KIND_TAG, {"options": {"type": "array", "items": {"type": "string"}, "x-semio-ui": ui(None, "Options", "Optionen")}, "selected": {"type": "integer", "minimum": 0, "x-semio-ui": ui("stepper", "Selected option", "Gewählte Option", step=1, precision=0)}, "output": ONE_PORT("Output", "Ausgang")}, ["options", "output"]),
    branch("kind", "screen", "Screen", "Bildschirm", KIND_TAG, {"media": {"anyOf": [{"$ref": "#/$defs/DagMedia"}, {"type": "null"}], "x-semio-ui": ui(None, "Media", "Medium")}, "input": ONE_PORT("Input", "Eingang")}, ["input"]),
    branch("kind", "note", "Note", "Notiz", KIND_TAG, {"text": {"type": "string", "x-semio-ui": ui("multiline", "Text", "Text")}, "output": ONE_PORT("Output", "Ausgang")}, ["text", "output"]),
    branch("kind", "image", "Image", "Bild", KIND_TAG, {"src": text("Source", "Quelle"), "output": ONE_PORT("Output", "Ausgang")}, ["output"]),
    branch("kind", "preview", "Preview", "Vorschau", KIND_TAG, {"content": {"$ref": "#/$defs/DagPreviewContent", "x-semio-ui": ui(None, "Content", "Inhalt")}, "expanded": {"type": "array", "items": {"type": "string"}, "uniqueItems": True, "x-semio-ui": ui(None, "Expanded paths", "Aufgeklappte Pfade")}, "input": ONE_PORT("Input", "Eingang")}, ["input"]),
    branch("kind", "action", "Action", "Aktion", KIND_TAG, {"label": text("Label", "Beschriftung"), "input": ONE_PORT("Input", "Eingang")}, ["label", "input"]),
    branch("kind", "export", "Export", "Export", KIND_TAG, {"label": text("Label", "Beschriftung"), "format": text("Format", "Format"), "input": ONE_PORT("Input", "Eingang")}, ["label", "format", "input"]),
    branch("kind", "cluster", "Cluster", "Cluster", KIND_TAG, {"inputs": PORTS("Inputs", "Eingänge"), "outputs": PORTS("Outputs", "Ausgänge")}, ["inputs", "outputs"]),
    branch("kind", "appInstance", "App Instance", "App-Instanz", KIND_TAG, {"instanceId": text("Instance ID", "Instanz-ID"), "pluginId": text("Plugin", "Plugin"), "appId": text("App", "App"), "appIcon": text("App icon", "App-Symbol"), "inputs": PORTS("Inputs", "Eingänge"), "outputs": PORTS("Outputs", "Ausgänge")}, ["instanceId", "pluginId", "appId", "inputs", "outputs"]),
]


def dag_kind(document):
    document["properties"]["newKind"] = {"oneOf": DAG_NODE_KIND, "x-semio-ui": document["properties"]["newKind"].get("x-semio-ui", ui(None, "New Node Kind", "Neue Knotenart", ("The node's kind together with its ports and kind-specific settings.", "Die Knotenart samt Anschlüssen und artspezifischen Einstellungen.")))}
    document.setdefault("$defs", {}).update({"IoPortSpec": PORT, "DagMedia": MEDIA, "DagPreviewContent": PREVIEW})
#endregion 🕸️DagNodeKind


#region 🔁️Workflow
MEDIA_TYPE = record({
    "class": select(["twoD", "threeD", "text", "data", "graph", "kit", "computation", "presentation"], "Media class", "Medienklasse", [("2D", "2D"), ("3D", "3D"), ("Text", "Text"), ("Data", "Daten"), ("Graph", "Graph"), ("Kit", "Bausatz"), ("Computation", "Berechnung"), ("Presentation", "Präsentation")]),
    "form": select(["any", "vector", "raster", "brep", "mesh", "document", "value", "dag", "trinity", "type", "design", "kit", "flow", "sequence", "procedure", "deck"], "Media form", "Medienform", [("Any", "Beliebig"), ("Vector", "Vektor"), ("Raster", "Raster"), ("B-rep", "B-Rep"), ("Mesh", "Netz"), ("Document", "Dokument"), ("Value", "Wert"), ("DAG", "DAG"), ("Trinity", "Trinity"), ("Type", "Typ"), ("Design", "Entwurf"), ("Kit", "Bausatz"), ("Flow", "Fluss"), ("Sequence", "Sequenz"), ("Procedure", "Prozedur"), ("Deck", "Foliensatz")]),
}, ["class", "form"])
MULTIPLICITY = select(["one", "many"], "Multiplicity", "Vielfachheit", [("One", "Eins"), ("Many", "Viele")])
FORMS = MEDIA_TYPE["properties"]["form"]["enum"]
WORKFLOW_DEFS = {
    "MediaType": MEDIA_TYPE,
    "MediaPortSpec": record({
        "id": text("Port ID", "Anschluss-ID"), "label": text("Label", "Beschriftung"),
        "direction": select(["in", "out"], "Direction", "Richtung", [("In", "Ein"), ("Out", "Aus")]),
        "mediaType": {"$ref": "#/$defs/MediaType", "x-semio-ui": ui(None, "Media type", "Medientyp")},
        "kindId": text("Artifact kind", "Artefaktart", ("Pins the port to one artifact kind; empty accepts the media type.", "Bindet den Anschluss an eine Artefaktart; leer akzeptiert den Medientyp.")),
        "required": toggle("Required", "Erforderlich"), "multiplicity": MULTIPLICITY,
    }, ["id", "label", "direction", "mediaType", "required", "multiplicity"]),
    "WorkflowMediaPort": record({"id": text("Port ID", "Anschluss-ID"), "spec": {"$ref": "#/$defs/MediaPortSpec", "x-semio-ui": ui(None, "Port declaration", "Anschlussdeklaration")}}, ["id", "spec"]),
    "MediaWireFormat": {"oneOf": [
        branch("kind", "binary", "Binary", "Binär", ("Wire format", "Übertragungsformat"), {"format_kind": text("Format kind", "Formatart")}, ["format_kind"]),
        branch("kind", "document", "Document", "Dokument", ("Wire format", "Übertragungsformat"), {"schema": text("Schema", "Schema")}, ["schema"]),
    ]},
}
NODE = record({
    "id": text("Node ID", "Knoten-ID"), "pluginId": text("Plugin", "Plugin"), "appId": text("App", "App"), "label": text("Label", "Beschriftung"),
    "yields": text("Yields", "Liefert", ("Artifact kind the node produces.", "Artefaktart, die der Knoten erzeugt.")),
    "artifactRef": text("Artifact", "Artefakt"), "configRef": text("Configuration", "Konfiguration"),
    "x": number("X", "X"), "y": number("Y", "Y"), "width": number("Width", "Breite"), "height": number("Height", "Höhe"),
    "inputs": {"type": "array", "items": {"$ref": "#/$defs/WorkflowMediaPort"}, "x-semio-ui": ui(None, "Inputs", "Eingänge")},
    "outputs": {"type": "array", "items": {"$ref": "#/$defs/WorkflowMediaPort"}, "x-semio-ui": ui(None, "Outputs", "Ausgänge")},
}, ["id", "pluginId", "appId", "label", "yields", "artifactRef", "configRef", "x", "y", "width", "height", "inputs", "outputs"])
EDGE = record({
    "id": text("Edge ID", "Kanten-ID"),
    "sourceNodeId": ref("node", "Source node", "Quellknoten"), "sourcePortId": text("Source port", "Quellanschluss"),
    "targetNodeId": ref("node", "Target node", "Zielknoten"), "targetPortId": text("Target port", "Zielanschluss"),
    "contract": record({
        "kindId": text("Artifact kind", "Artefaktart"),
        "mediaType": {"$ref": "#/$defs/MediaType", "x-semio-ui": ui(None, "Media type", "Medientyp")},
        "wire": {"$ref": "#/$defs/MediaWireFormat", "x-semio-ui": ui(None, "Wire format", "Übertragungsformat")},
        "conversion": {"type": ["array", "null"], "items": {"type": "string", "enum": FORMS, "x-semio-ui": {"options": MEDIA_TYPE["properties"]["form"]["x-semio-ui"]["options"]}}, "minItems": 2, "maxItems": 2, "x-semio-ui": ui(None, "Conversion", "Umwandlung", ("Source and target media form of an implicit conversion; empty when none.", "Quell- und Zielmedienform einer impliziten Umwandlung; leer, wenn keine."))},
    }, ["kindId", "mediaType", "wire", "conversion"], "Contract", "Vertrag", ("The negotiated wire contract of the edge.", "Der ausgehandelte Übertragungsvertrag der Kante.")),
}, ["id", "sourceNodeId", "sourcePortId", "targetNodeId", "targetPortId", "contract"])
PARAMETER_TAG = ("Parameter type", "Parameterart")
PARAMETER = {"oneOf": [
    branch("type", "numeric", "Numeric", "Numerisch", PARAMETER_TAG, {"id": text("Parameter ID", "Parameter-ID"), "name": text("Name", "Name"), "value": number("Value", "Wert"), "min": {"type": ["number", "null"], "x-semio-ui": ui("stepper", "Minimum", "Minimum")}, "max": {"type": ["number", "null"], "x-semio-ui": ui("stepper", "Maximum", "Maximum")}, "step": {"type": ["number", "null"], "x-semio-ui": ui("stepper", "Step", "Schrittweite")}}, ["id", "name", "value"]),
    branch("type", "categorical", "Categorical", "Kategorial", PARAMETER_TAG, {"id": text("Parameter ID", "Parameter-ID"), "name": text("Name", "Name"), "value": text("Value", "Wert"), "options": {"type": "array", "items": {"type": "string"}, "x-semio-ui": ui(None, "Options", "Optionen")}}, ["id", "name", "value", "options"]),
    branch("type", "toggle", "Toggle", "Schalter", PARAMETER_TAG, {"id": text("Parameter ID", "Parameter-ID"), "name": text("Name", "Name"), "value": toggle("Value", "Wert")}, ["id", "name", "value"]),
    branch("type", "text", "Text", "Text", PARAMETER_TAG, {"id": text("Parameter ID", "Parameter-ID"), "name": text("Name", "Name"), "value": text("Value", "Wert")}, ["id", "name", "value"]),
]}
RECORDS = {
    "📥add-input": ("input", record({"id": text("Input ID", "Eingangs-ID"), "kindId": text("Artifact kind", "Artefaktart"), "selector": text("Selector", "Selektor", ("Which artifacts the input picks.", "Welche Artefakte der Eingang auswählt.")), "required": toggle("Required", "Erforderlich"), "multiplicity": MULTIPLICITY}, ["id", "kindId", "selector", "required", "multiplicity"]), False),
    "➕️add-node": ("node", NODE, True),
    "📤bind-output": ("binding", record({"nodeId": ref("node", "Node", "Knoten"), "portId": text("Port", "Anschluss"), "pathTemplate": text("Path template", "Pfadvorlage", ("Where the output is written, with placeholders.", "Wohin die Ausgabe geschrieben wird, mit Platzhaltern."))}, ["nodeId", "portId", "pathTemplate"]), False),
    "🔌bind-input": ("binding", record({"inputId": ref("input", "Input", "Eingang"), "nodeId": ref("node", "Node", "Knoten"), "portId": text("Port", "Anschluss")}, ["inputId", "nodeId", "portId"]), False),
    "🔒bind-parameter-field": ("binding", record({"parameterId": ref("parameter", "Parameter", "Parameter"), "nodeId": ref("node", "Node", "Knoten"), "fieldPath": text("Field path", "Feldpfad", ("JSON pointer of the bound configuration field.", "JSON-Pointer des gebundenen Konfigurationsfelds."))}, ["parameterId", "nodeId", "fieldPath"]), False),
    "🔗connect-ports": ("edge", EDGE, True),
    "🧩add-parameter": ("parameter", PARAMETER, False),
    "🩹change-parameter": ("parameter", PARAMETER, False),
}


def camel(name):
    words = name.split("_")
    return words[0] + "".join(word[:1].upper() + word[1:] for word in words[1:])


def workflow(leaf):
    def edit(document):
        properties = document["properties"]
        document["properties"] = {camel(key): value for key, value in properties.items()}
        document["required"] = [camel(key) for key in document.get("required", [])]
        document.setdefault("additionalProperties", False)
        if leaf in RECORDS:
            name, node, shared = RECORDS[leaf]
            annotation = properties[name].get("x-semio-ui")
            document["properties"][name] = {**copy.deepcopy(node), **({"x-semio-ui": annotation} if annotation is not None else {})}
            if shared:
                document.setdefault("$defs", {}).update(WORKFLOW_DEFS)
    return edit
#endregion 🔁️Workflow


#region 🌊️FlowVcs
def flow_snapshot(document):
    neuron = next(branch for branch in document["$defs"]["Widget"]["oneOf"] if branch["properties"]["kind"].get("const") == "neuron")
    neuron["properties"] = {("inputPorts" if key == "input_ports" else "outputPorts" if key == "output_ports" else key): value for key, value in neuron["properties"].items()}
    host = document["$defs"]["HostDocument"]["properties"]
    labels = {"schema": ("Schema", "Schema", ("Document schema id of the host snapshot.", "Dokumentschema-ID der Host-Momentaufnahme.")), "camera": ("Camera", "Kamera", None), "widgets": ("Widgets", "Widgets", ("Every widget on the canvas.", "Alle Widgets auf der Arbeitsfläche.")), "synapses": ("Synapses", "Synapsen", ("Every connection between widget ports.", "Alle Verbindungen zwischen Widget-Anschlüssen.")), "layout": ("Layout", "Anordnung", ("Canvas position per widget id.", "Position auf der Arbeitsfläche je Widget-ID."))}
    for key, (en, de, description) in labels.items():
        host[key].setdefault("x-semio-ui", ui("text" if key == "schema" else None, en, de, description))


def drop_property_names(document):
    document.pop("propertyNames", None)
#endregion 🌊️FlowVcs


def store_checkpoint(document):
    checkpoint = document["properties"]["checkpoint"]
    checkpoint["required"] = [name for name in checkpoint["required"] if name != "parentId"]
    authors = checkpoint["properties"]["authors"]
    authors["items"] = record({"id": text("Author ID", "Autoren-ID"), "name": text("Name", "Name"), "avatar": text("Avatar", "Avatar", ("Avatar image URL; absent when none.", "URL des Avatarbilds; fehlt, wenn keins."))}, ["id", "name"])


EDITS = {
    **{f"{IMPERATIVE}/{leaf}/🧬️schema/🔣️.json": (lambda document: [document["properties"]["pathRef"]["properties"][key].__setitem__("type", ["string", "null"]) for key in ("owner", "slot")]) for leaf in ("🔀reorder-steps", "🗑️delete-step")},
    f"{IMPERATIVE}/🔧edit-step-params/🧬️schema/🔣️.json": lambda document: ([document["properties"]["pathRef"]["properties"][key].__setitem__("type", ["string", "null"]) for key in ("owner", "slot")], neural_params(document["properties"]["newParams"]), with_neural(document)),
    f"{IMPERATIVE}/🌱create-step/🧬️schema/🔣️.json": lambda document: ([document["$defs"]["PathRef"]["properties"][key].__setitem__("type", ["string", "null"]) for key in ("owner", "slot")], neural_params(document["$defs"]["Step"]["properties"]["params"]), with_neural(document)),
    f"{SEQUENCE}/🌱️create-step/🧬️schema/🔣️.json": lambda document: (neural_params(document["properties"]["step"]["properties"]["params"]), with_neural(document)),
    f"{SEQUENCE}/🔧️edit-step-params/🧬️schema/🔣️.json": lambda document: (neural_params(document["properties"]["params"]), with_neural(document)),
    f"{DAG}/🔁replace-node-kind/🧬️schema/🔣️.json": dag_kind,
    f"{DAG}/🧮change-node-operator-kind/🧬️schema/🔣️.json": lambda document: document["properties"]["newOperatorKind"].__setitem__("type", ["string", "null"]),
    **{f"{FLOW}/{leaf}/🧬️schema/🔣️.json": (lambda document: document["properties"].__setitem__("widget", {"$ref": FLOW_WIDGET, **({"x-semio-ui": document["properties"]["widget"]["x-semio-ui"]} if "x-semio-ui" in document["properties"]["widget"] else {})})) for leaf in ("➕️create-widget", "🔁️replace-widget")},
    f"{FLOW_VCS}/🔣️.json": flow_snapshot,
    **{f"{FLOW_VCS}/🧬️mutations/{leaf}/🧬️schema/🔣️.json": drop_property_names for leaf in ("↔️move-widget", "♻️replace-flow-host-snapshot", "✂️remove-synapse", "➕️add-widget", "📐️change-layout", "🔀️move-synapse", "🔄change-synapse", "🔗️add-synapse", "🗑️remove-widget", "🩹change-widget")},
    **{f"{WORKFLOW}/{leaf}/🧬️schema/🔣️.json": workflow(leaf) for leaf in sorted(os.listdir(os.path.join(REPO, WORKFLOW))) if os.path.isfile(os.path.join(REPO, WORKFLOW, leaf, "🧬️schema", "🔣️.json"))},
    f"{STORE}/🧬️schema/🧬️mutations/📌️commit-space-checkpoint/🧬️schema/🔣️.json": store_checkpoint,
    f"{STORE}/🧫️fixtures/🧬️mutations/🧮️demo/🧬️mutations/🗑️delete-n/🧬️schema/🔣️.json": drop_property_names,
}


def main():
    for path, edit in EDITS.items():
        before = read(path)
        document = json.loads(before)
        edit(document)
        if document == json.loads(before):
            continue
        suffix = "\n" if before.endswith("\n") else ""
        if json.dumps(json.loads(before), indent=2, ensure_ascii=False) + suffix != before:
            print(f"[w2-s-e] HAND EDIT NEEDED (not canonical) {path}")
            continue
        after = json.dumps(document, indent=2, ensure_ascii=False) + suffix
        if APPLY:
            if read(path) != before:
                raise SystemExit(f"[w2-s-e] {path} changed while editing; rerun")
            with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
                handle.write(after)
        print(f"[w2-s-e] {'wrote' if APPLY else 'would write'} {path}")


if __name__ == "__main__":
    main()
