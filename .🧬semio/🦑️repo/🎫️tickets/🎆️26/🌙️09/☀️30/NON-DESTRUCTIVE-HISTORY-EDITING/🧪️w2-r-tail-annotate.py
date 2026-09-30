#!/usr/bin/env python3
"""🏷️ W2-R tail: writes the `x-semio-ui` input annotations (design §6, manifest `$defs/InputUi`) onto the mutation leaf payload
schemas of the tail plugin group. Every annotation names its property node by JSON path inside the leaf schema; the file keeps
its own formatting (standard `json.dumps` indentation, or the compact one-line-leaf style). Idempotent: re-running rewrites the
same bytes; an existing different annotation at a site aborts."""
import json
import os
import sys

REPO = "/Users/ueli/Documents/semio"
PLUGINS = os.path.join(REPO, "✏️s/🔌️plugins")
DEGREES = 57.29577951308232
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "options", "group", "order")


def L(en, de):
    return {"en": en, "de": de}


def ui(widget, label, description=None, role="value", **extra):
    body = {"widget": widget, "role": role, "label": label, **extra}
    if description is not None:
        body["description"] = description
    return {key: body[key] for key in KEY_ORDER if key in body}


def record(label, description=None):
    body = {"role": "value", "label": label}
    if description is not None:
        body["description"] = description
    return body


def text(label, description=None):
    return ui("text", label, description)


def multiline(label, description=None):
    return ui("multiline", label, description)


def toggle(label, description=None):
    return ui("toggle", label, description)


def stepper(label, description=None, step=1, precision=None, **extra):
    extra = {"step": step, **({} if precision is None else {"precision": precision}), **extra}
    return ui("stepper", label, description, **extra)


def count(label, description=None, **extra):
    return stepper(label, description, step=1, precision=0, **extra)


def select(label, options, description=None, widget="select"):
    return ui(widget, label, description, options=options)


def reference(label, kind, description=None, domain=None, granularity=None, role="target"):
    ref = {"kind": kind, **({} if domain is None else {"domain": domain}), **({} if granularity is None else {"granularity": granularity})}
    return ui("reference", label, description, role=role, ref=ref)


def angle(label, description=None):
    return ui("dial", label, description, unit="rad", displayUnit="deg", displayFactor=DEGREES, step=0.017453292519943295, softMin=-3.141592653589793, softMax=3.141592653589793, snaps=[-3.141592653589793, -1.5707963267948966, 0, 1.5707963267948966, 3.141592653589793])


def at(prefix, table):
    return {prefix + "/" + key: value for key, value in table.items()}


#region 🌐️SharedRecords
ARTIFACT_ID = text(L("Artifact ID", "Artefakt-ID"), L("Document id of the child artifact.", "Dokument-ID des Kind-Artefakts."))
ARTIFACT_KIND = text(L("Artifact Kind", "Artefaktart"), L("Kind id of the artifact, e.g. s.stdio.semio.", "Art-ID des Artefakts, z. B. s.stdio.semio."))
STANDARD = text(L("Standard", "Standard"), L("Standard version of the artifact kind.", "Standardversion der Artefaktart."))
SUBSET = text(L("Subset", "Teilmenge"), L("Subset of the standard the artifact uses.", "Vom Artefakt verwendete Teilmenge des Standards."))
DIALECT = record(L("Dialect", "Dialekt"), L("Artifact kind, standard and subset of the child.", "Artefaktart, Standard und Teilmenge des Kindes."))
DIALECT_FIELDS = {"properties/artifactKind": ARTIFACT_KIND, "properties/standard": STANDARD, "properties/subset": SUBSET}


def artifact_ref(prefix):
    return {prefix + "/properties/artifactId": ARTIFACT_ID, prefix + "/properties/dialect": DIALECT, **at(prefix + "/properties/dialect", DIALECT_FIELDS)}


CHILD_ID = text(L("Child ID", "Kind-ID"), L("Id of the child slot in this document.", "ID des Kind-Slots in diesem Dokument."))
LAYER_INDEX = count(L("Layer Index", "Ebenenindex"), L("Position of the paint layer, counted from the bottom.", "Position der Malebene, von unten gezählt."))
CONTRIBUTIONS = multiline(L("Contributions", "Beiträge"), L("Contributions of this editor as JSON text.", "Beiträge dieses Editors als JSON-Text."))
#endregion 🌐️SharedRecords

#region 📖️Playbook
STEP_REF = dict(domain="blocks", granularity="step")
BLOCK_REF = dict(domain="blocks", granularity="block")
PLAYBOOK_BLOCK = {
    "properties/kind": text(L("Block Kind", "Blockart"), L("Built-in kind (text, number, slider, single, …) or a module-contributed kind.", "Eingebaute Art (Text, Zahl, Schieberegler, Einfachauswahl, …) oder eine von einem Modul beigetragene Art.")),
    "properties/required": toggle(L("Required", "Pflichtfeld"), L("The block must be answered.", "Der Block muss beantwortet werden.")),
    "properties/placeholder": text(L("Placeholder", "Platzhalter"), L("Hint shown while the answer is empty.", "Hinweis, solange die Antwort leer ist.")),
    "properties/default": record(L("Default Value", "Standardwert"), L("Initial answer; any JSON value.", "Anfangsantwort; beliebiger JSON-Wert.")),
    "properties/min": stepper(L("Minimum", "Minimum"), L("Lowest allowed answer of a number or slider block.", "Kleinste zulässige Antwort eines Zahlen- oder Schieberegler-Blocks."), step=1),
    "properties/max": stepper(L("Maximum", "Maximum"), L("Highest allowed answer of a number or slider block.", "Größte zulässige Antwort eines Zahlen- oder Schieberegler-Blocks."), step=1),
    "properties/step": stepper(L("Step Size", "Schrittweite"), L("Increment of a number or slider block.", "Schrittweite eines Zahlen- oder Schieberegler-Blocks."), step=0.1),
    "properties/unit": text(L("Unit", "Einheit"), L("Unit shown next to a number answer.", "Neben einer Zahlenantwort angezeigte Einheit.")),
    "properties/options": record(L("Options", "Optionen"), L("Choices of a single or multiple choice block.", "Auswahlmöglichkeiten eines Einfach- oder Mehrfachauswahl-Blocks.")),
    "properties/fields": record(L("Vector Fields", "Vektorfelder"), L("Components of a vector block.", "Komponenten eines Vektor-Blocks.")),
    "properties/schema": text(L("Schema", "Schema"), L("Schema id of the block's value, e.g. solid.step.", "Schema-ID des Blockwerts, z. B. solid.step.")),
    "properties/src": text(L("Source URL", "Quell-URL"), L("Location of the image an image block shows.", "Ort des Bildes, das ein Bild-Block zeigt.")),
    "properties/accept": text(L("Accepted File Types", "Akzeptierte Dateitypen"), L("File type filter of a file block, e.g. image/*.", "Dateitypfilter eines Datei-Blocks, z. B. image/*.")),
    "properties/fixtureSlug": text(L("Fixture", "Vorlage"), L("Procedural fixture the block renders.", "Prozedurale Vorlage, die der Block darstellt.")),
    "properties/condition": record(L("Condition", "Bedingung"), L("Expression that must hold for the block to apply.", "Ausdruck, der gelten muss, damit der Block gilt.")),
}
PLAYBOOK = {
    ("📖️playbook", "📸️replace-config"): {"properties/config/properties/contributionsJson": CONTRIBUTIONS},
    ("📖️playbook", "🧱add-block"): {
        "properties/stepId": reference(L("Step", "Schritt"), "step", L("The step that receives the block.", "Der Schritt, der den Block aufnimmt."), **STEP_REF),
        "properties/block": record(L("Block", "Block"), L("The complete block record to add.", "Der vollständige hinzuzufügende Block.")),
        **at("properties/block", PLAYBOOK_BLOCK),
    },
    ("📖️playbook", "🔄replace-block"): {
        "properties/stepId": reference(L("Step", "Schritt"), "step", L("The step that holds the block.", "Der Schritt, der den Block enthält."), **STEP_REF),
        "properties/block": record(L("Block", "Block"), L("The block record that replaces the one with the same id.", "Der Block, der den mit derselben ID ersetzt.")),
        **at("properties/block", PLAYBOOK_BLOCK),
    },
    ("📖️playbook", "➕add-step"): {
        "properties/step": record(L("Step", "Schritt"), L("The complete step record to add.", "Der vollständige hinzuzufügende Schritt.")),
        "properties/step/properties/blocks": record(L("Blocks", "Blöcke"), L("The step's blocks in order.", "Die Blöcke des Schritts in Reihenfolge.")),
        **at("properties/step/properties/blocks/items", PLAYBOOK_BLOCK),
    },
    ("📖️playbook", "🔀move-block"): {
        "properties/blockId": reference(L("Block", "Block"), "block", L("The block to move.", "Der zu verschiebende Block."), **BLOCK_REF),
        "properties/fromStepId": reference(L("From Step", "Von Schritt"), "step", L("The step that holds the block now.", "Der Schritt, der den Block jetzt enthält."), **STEP_REF),
        "properties/toStepId": reference(L("To Step", "Nach Schritt"), "step", L("The step that receives the block.", "Der Schritt, der den Block aufnimmt."), role="value", **STEP_REF),
    },
    ("📖️playbook", "🗑️remove-block"): {
        "properties/stepId": reference(L("Step", "Schritt"), "step", L("The step that holds the block.", "Der Schritt, der den Block enthält."), **STEP_REF),
        "properties/blockId": reference(L("Block", "Block"), "block", L("The block to remove.", "Der zu entfernende Block."), **BLOCK_REF),
    },
    ("📖️playbook", "📦️set-payload"): {
        "properties/payload": record(L("Payload", "Nutzlast"), L("Render payload of the procedural module.", "Darstellungsnutzlast des prozeduralen Moduls.")),
        "properties/payload/properties/fixtureSlug": text(L("Fixture", "Vorlage"), L("Procedural fixture to render.", "Darzustellende prozedurale Vorlage.")),
        "properties/payload/properties/questionId": text(L("Question ID", "Fragen-ID"), L("Playbook question the module answers.", "Playbook-Frage, die das Modul beantwortet.")),
        "properties/payload/properties/controllerId": text(L("Controller App", "Steuernde App"), L("App scope that controls the module, e.g. s.forms.forms@1/*#editor.", "App-Bereich, der das Modul steuert, z. B. s.forms.forms@1/*#editor.")),
        "properties/payload/properties/surface": text(L("Surface", "Oberfläche"), L("Surface the module renders on.", "Oberfläche, auf der das Modul dargestellt wird.")),
        "properties/payload/properties/interactive": toggle(L("Interactive", "Interaktiv"), L("The viewer may change the parameters.", "Die betrachtende Person darf die Parameter ändern.")),
    },
}
#endregion 📖️Playbook

#region 💠️Lowpoly
BLEND_MODE = text(L("Blend Mode", "Füllmethode"), L("How the layer combines with the layers below, e.g. normal or multiply.", "Wie die Ebene mit den darunterliegenden Ebenen verrechnet wird, z. B. normal oder multiply."))
SMOOTH_SHADING = toggle(L("Smooth Shading", "Glatte Schattierung"), L("Interpolates normals across faces.", "Interpoliert Normalen über Flächen hinweg."))
LOWPOLY = {
    ("💠️lowpoly", "➕️insert-paint-layer"): {
        "properties/objectId": reference(L("Object", "Objekt"), "object", L("The object that receives the paint layer.", "Das Objekt, das die Malebene aufnimmt."), domain="mesh", granularity="object"),
        "properties/layer": record(L("Paint Layer", "Malebene"), L("The complete paint layer record to insert.", "Die vollständige einzufügende Malebene.")),
        "properties/layer/properties/blendMode": BLEND_MODE,
    },
    ("💠️lowpoly", "🌱️create-object"): {
        "properties/object": record(L("Object", "Objekt"), L("The complete object record to add.", "Das vollständige hinzuzufügende Objekt.")),
        "properties/object/properties/mesh": record(L("Mesh", "Netz"), L("Owned mesh child of the object; empty for none.", "Eigenes Netz-Kind des Objekts; leer für keines.")),
        **artifact_ref("properties/object/properties/mesh/oneOf/1/properties/target"),
        "properties/object/properties/mesh/oneOf/1/properties/target": record(L("Target", "Ziel"), L("Artifact the mesh child points at.", "Artefakt, auf das das Netz-Kind verweist.")),
        "properties/object/properties/smoothShading": SMOOTH_SHADING,
        "properties/object/properties/meshContent": multiline(L("Mesh Content", "Netzinhalt"), L("Mesh content set together with the mesh child.", "Zusammen mit dem Netz-Kind gesetzter Netzinhalt.")),
        "properties/object/properties/paintLayers": record(L("Paint Layers", "Malebenen"), L("Paint layers from bottom to top.", "Malebenen von unten nach oben.")),
        "properties/object/properties/paintLayers/items/properties/blendMode": BLEND_MODE,
    },
    ("💠️lowpoly", "🎛️change-paint-layer-blend-mode"): {
        "properties/objectId": reference(L("Object", "Objekt"), "object", L("The object that owns the paint layer.", "Das Objekt, dem die Malebene gehört."), domain="mesh", granularity="object"),
        "properties/index": LAYER_INDEX,
        "properties/newBlendMode": text(L("New Blend Mode", "Neue Füllmethode"), L("How the layer combines with the layers below, e.g. normal or multiply.", "Wie die Ebene mit den darunterliegenden Ebenen verrechnet wird, z. B. normal oder multiply.")),
    },
    ("💠️lowpoly", "🎨️edit-paint-layer"): {
        "properties/objectId": reference(L("Object", "Objekt"), "object", L("The object that owns the paint layer.", "Das Objekt, dem die Malebene gehört."), domain="mesh", granularity="object"),
        "properties/layerIndex": LAYER_INDEX,
        "properties/runs": record(L("Pixel Runs", "Pixelläufe"), L("Contiguous RGBA byte runs written into the layer.", "Zusammenhängende RGBA-Bytefolgen, die in die Ebene geschrieben werden.")),
        "properties/runs/items/properties/offset": count(L("Byte Offset", "Byte-Versatz"), L("Start of the run in the layer's RGBA buffer.", "Beginn des Laufs im RGBA-Puffer der Ebene.")),
        "properties/runs/items/properties/bytes": text(L("Bytes", "Bytes"), L("RGBA bytes of the run, base64-encoded.", "RGBA-Bytes des Laufs, Base64-kodiert.")),
    },
    ("💠️lowpoly", "🔘️change-object-smooth-shading"): {
        "properties/id": reference(L("Object", "Objekt"), "object", L("The object to shade.", "Das zu schattierende Objekt."), domain="mesh", granularity="object"),
        "properties/newSmoothShading": SMOOTH_SHADING,
    },
    ("💠️lowpoly", "🕸️create-mesh"): {
        "properties/id": reference(L("Object", "Objekt"), "object", L("The object that receives the mesh child.", "Das Objekt, das das Netz-Kind erhält."), domain="mesh", granularity="object"),
        "properties/target": record(L("Target", "Ziel"), L("Artifact the new mesh child points at.", "Artefakt, auf das das neue Netz-Kind verweist.")),
        **artifact_ref("properties/target"),
        "properties/meshWorkspace": multiline(L("Mesh Workspace", "Netz-Arbeitsbereich"), L("Initial mesh content as JSON text; the authoring session replays it.", "Anfänglicher Netzinhalt als JSON-Text; die erstellende Sitzung spielt ihn nach.")),
    },
}
#endregion 💠️Lowpoly

#region 🏭️Process
MACHINE = reference(L("Machine", "Maschine"), "machine", L("Workshop machine that produced the step; informational only.", "Werkstattmaschine, die den Schritt erzeugt hat; nur informativ."), role="value")
CAPABILITY = reference(L("Capability", "Fähigkeit"), "capability", L("Machine capability that produced the step; informational only.", "Maschinenfähigkeit, die den Schritt erzeugt hat; nur informativ."), role="value")
ORIGIN_FIELDS = {"properties/machineId": MACHINE, "properties/capabilityId": CAPABILITY}
OPERATION_DESCRIPTION = L("Cut, drill or attach operation of the step with its tool and pose.", "Schnitt-, Bohr- oder Anbauoperation des Schritts mit Werkzeug und Lage.")
ICON = text(L("Icon", "Symbol"), L("Icon id shown for the entry.", "Für den Eintrag angezeigte Symbol-ID."))
CAPABILITY_FIELDS = {
    "properties/iconId": ICON,
    "properties/recipe": record(L("Recipe", "Rezept"), L("How the parameters size the tool: disc, blade or pocket cut, bore drill, cylinder or box attach.", "Wie die Parameter das Werkzeug bemessen: Scheiben-, Klingen- oder Taschenschnitt, Bohrung, Zylinder- oder Quaderanbau.")),
    "properties/parameters": record(L("Parameters", "Parameter"), L("Named tool sizes in metres, referenced by the recipe and rules.", "Benannte Werkzeugmaße in Metern, auf die Rezept und Regeln verweisen.")),
    "properties/rules": record(L("Rules", "Regeln"), L("Stock limits the capability requires; all must hold.", "Rohteilgrenzen, die die Fähigkeit voraussetzt; alle müssen gelten.")),
}
PROCESS = {
    ("🏭️process", "⏱️change-cursor"): {"properties/newResolvedUpTo": count(L("Playback Cursor", "Wiedergabe-Cursor"), L("Number of timeline steps applied to the stock; empty applies all.", "Anzahl der auf das Rohteil angewendeten Zeitleistenschritte; leer wendet alle an."))},
    ("🏭️process", "🌱create-step"): {
        "properties/step": record(L("Step", "Schritt"), L("The complete step record to add.", "Der vollständige hinzuzufügende Schritt.")),
        "properties/step/properties/origin": record(L("Origin", "Herkunft"), L("Workshop machine and capability that produced the step.", "Werkstattmaschine und Fähigkeit, die den Schritt erzeugt haben.")),
        **at("properties/step/properties/origin", ORIGIN_FIELDS),
        "properties/step/properties/measure": record(L("Operation", "Bearbeitung"), OPERATION_DESCRIPTION),
    },
    ("🏭️process", "🎨change-machine-icon"): {
        "properties/id": reference(L("Machine", "Maschine"), "machine", L("The workshop machine to change.", "Die zu ändernde Werkstattmaschine.")),
        "properties/newIconId": text(L("New Icon", "Neues Symbol"), L("Icon id shown for the machine.", "Für die Maschine angezeigte Symbol-ID.")),
    },
    ("🏭️process", "🏭create-machine"): {
        "properties/machine": record(L("Machine", "Maschine"), L("The complete workshop machine record to add.", "Die vollständige hinzuzufügende Werkstattmaschine.")),
        "properties/machine/properties/iconId": ICON,
        "properties/machine/properties/capabilities": record(L("Capabilities", "Fähigkeiten"), L("What the machine can do; each capability turns into a step.", "Was die Maschine kann; jede Fähigkeit wird zu einem Schritt.")),
        **at("properties/machine/properties/capabilities/items", CAPABILITY_FIELDS),
    },
    ("🏭️process", "📍move-stock"): {
        "properties/newPose": record(L("New Pose", "Neue Lage"), L("Position and axis-angle rotation of the stock.", "Position und Achse-Winkel-Drehung des Rohteils.")),
        "properties/newPose/properties/axis": ui("vector", L("Rotation Axis", "Drehachse"), L("Direction the stock rotates about.", "Richtung, um die sich das Rohteil dreht.")),
        "properties/newPose/properties/angle": angle(L("Angle", "Winkel"), L("Counter-clockwise rotation about the axis.", "Drehung gegen den Uhrzeigersinn um die Achse.")),
    },
    ("🏭️process", "📐replace-step-measure"): {
        "properties/id": reference(L("Step", "Schritt"), "step", L("The timeline step to change.", "Der zu ändernde Zeitleistenschritt.")),
        "properties/newMeasure": record(L("New Operation", "Neue Bearbeitung"), OPERATION_DESCRIPTION),
    },
    ("🏭️process", "🔁replace-machine-capabilities"): {
        "properties/id": reference(L("Machine", "Maschine"), "machine", L("The workshop machine to change.", "Die zu ändernde Werkstattmaschine.")),
        "properties/newCapabilities": record(L("New Capabilities", "Neue Fähigkeiten"), L("What the machine can do; each capability turns into a step.", "Was die Maschine kann; jede Fähigkeit wird zu einem Schritt.")),
        **at("properties/newCapabilities/items", CAPABILITY_FIELDS),
    },
    ("🏭️process", "🧊replace-stock-solid"): {
        "properties/newSolid": record(L("New Stock Solid", "Neuer Rohteilkörper"), L("Child handle of the stock's solid body.", "Kind-Verweis auf den Volumenkörper des Rohteils.")),
        "properties/newSolid/properties/target": record(L("Target", "Ziel"), L("Artifact the solid child points at.", "Artefakt, auf das das Körper-Kind verweist.")),
        "properties/newSolid/properties/target/properties/artifactId": ARTIFACT_ID,
        "properties/newSolid/properties/target/properties/dialect": text(L("Dialect", "Dialekt"), L("Artifact kind, standard and subset of the child.", "Artefaktart, Standard und Teilmenge des Kindes.")),
    },
    ("🏭️process", "🧷change-step-origin"): {
        "properties/id": reference(L("Step", "Schritt"), "step", L("The timeline step to change.", "Der zu ändernde Zeitleistenschritt.")),
        "properties/newOrigin": record(L("New Origin", "Neue Herkunft"), L("Workshop machine and capability that produced the step; empty clears it.", "Werkstattmaschine und Fähigkeit, die den Schritt erzeugt haben; leer entfernt sie.")),
        **at("properties/newOrigin", ORIGIN_FIELDS),
    },
}
#endregion 🏭️Process

#region 💡️Reasoning
GRAPH = dict(domain="graph")
REASONING = {
    ("💡️reasoning", "🖱️set-drag"): {
        "properties/nodeId": reference(L("Dragged Node", "Gezogener Knoten"), "node", L("The node being dragged; empty while panning.", "Der gezogene Knoten; leer beim Verschieben der Ansicht."), role="value", granularity="node", **GRAPH),
        "properties/startX": stepper(L("Drag Start X", "Ziehbeginn X"), L("Pointer x where the drag began.", "Zeiger-x-Position zu Beginn des Ziehens."), precision=2),
        "properties/startY": stepper(L("Drag Start Y", "Ziehbeginn Y"), L("Pointer y where the drag began.", "Zeiger-y-Position zu Beginn des Ziehens."), precision=2),
        "properties/lastX": stepper(L("Last Pointer X", "Letzte Zeigerposition X"), L("Pointer x at the latest drag event.", "Zeiger-x-Position beim letzten Ziehereignis."), precision=2),
        "properties/lastY": stepper(L("Last Pointer Y", "Letzte Zeigerposition Y"), L("Pointer y at the latest drag event.", "Zeiger-y-Position beim letzten Ziehereignis."), precision=2),
    },
    ("💡️reasoning", "✂️disconnect-nodes"): {"properties/edgeId": reference(L("Edge", "Kante"), "edge", L("The edge to remove.", "Die zu entfernende Kante."), granularity="edge", **GRAPH)},
    ("💡️reasoning", "🏷️change-node-kind"): {
        "properties/nodeId": reference(L("Node", "Knoten"), "node", L("The node to change.", "Der zu ändernde Knoten."), granularity="node", **GRAPH),
        "properties/newNodeKind": text(L("New Node Kind", "Neue Knotenart"), L("Reasoning role of the node, e.g. claim or evidence.", "Argumentationsrolle des Knotens, z. B. Behauptung oder Beleg.")),
    },
    ("💡️reasoning", "🚩set-node-root"): {
        "properties/nodeId": reference(L("Node", "Knoten"), "node", L("The node to change.", "Der zu ändernde Knoten."), granularity="node", **GRAPH),
        "properties/newRoot": toggle(L("Root", "Wurzel"), L("The node starts a line of reasoning.", "Der Knoten beginnt eine Argumentationslinie.")),
    },
    ("💡️reasoning", "🌱create-node"): {"properties/node": record(L("Node", "Knoten"), L("The complete node record to add.", "Der vollständige hinzuzufügende Knoten."))},
    ("💡️reasoning", "🤝️connect-nodes"): {
        "properties/edge": record(L("Edge", "Kante"), L("The complete edge record to add.", "Die vollständige hinzuzufügende Kante.")),
        "properties/relationship": record(L("Relationship", "Beziehung"), L("The relationship record the edge carries.", "Der Beziehungsdatensatz, den die Kante trägt.")),
    },
}
#endregion 💡️Reasoning

#region 🕸️Dag
CAMERA = {
    "properties/cameraX": stepper(L("Camera X", "Kamera X"), L("Horizontal centre of the view.", "Horizontaler Mittelpunkt der Ansicht."), precision=2),
    "properties/cameraY": stepper(L("Camera Y", "Kamera Y"), L("Vertical centre of the view.", "Vertikaler Mittelpunkt der Ansicht."), precision=2),
    "properties/cameraZoom": stepper(L("Camera Zoom", "Kamerazoom"), L("Magnification of the view; 1 is actual size.", "Vergrößerung der Ansicht; 1 ist Originalgröße."), step=0.1, precision=2),
}
DAG_KINDS = {
    "computation": L("Computation", "Berechnung"), "slider": L("Slider", "Schieberegler"), "select": L("Select", "Auswahl"), "screen": L("Screen", "Bildschirm"), "note": L("Note", "Notiz"), "image": L("Image", "Bild"),
    "preview": L("Preview", "Vorschau"), "action": L("Action", "Aktion"), "export": L("Export", "Export"), "cluster": L("Cluster", "Cluster"), "appInstance": L("App Instance", "App-Instanz"),
}
DAG_NODE = dict(domain="graph", granularity="node")
DAG = {
    ("🕸️dag", "🔄️replace-config"): {"properties/config": record(L("Configuration", "Konfiguration"), L("Camera of this editor.", "Kamera dieses Editors.")), **at("properties/config", CAMERA)},
    ("🕸️dag", "🔄️replace-presence"): {"properties/presence": record(L("Presence", "Präsenz"), L("Camera this participant shares with others.", "Kamera, die diese Person mit anderen teilt.")), **at("properties/presence", CAMERA)},
    ("🕸️dag", "🌱create-node"): {
        "properties/node": record(L("Node", "Knoten"), L("The complete node record to add.", "Der vollständige hinzuzufügende Knoten.")),
        "properties/node/properties/abbreviation": text(L("Abbreviation", "Abkürzung"), L("Short name shown on the node.", "Auf dem Knoten angezeigter Kurzname.")),
        "properties/node/properties/icon": text(L("Icon", "Symbol"), L("Icon id shown on the node.", "Auf dem Knoten angezeigte Symbol-ID.")),
        "properties/node/properties/operatorKind": text(L("Operator Kind", "Operatorart"), L("Operator a computation node evaluates; empty for none.", "Operator, den ein Berechnungsknoten auswertet; leer für keinen.")),
        "properties/node/properties/kind": select(L("Node Kind", "Knotenart"), DAG_KINDS),
    },
    ("🕸️dag", "🔡change-node-abbreviation"): {
        "properties/id": reference(L("Node", "Knoten"), "node", L("The node to change.", "Der zu ändernde Knoten."), **DAG_NODE),
        "properties/newAbbreviation": text(L("New Abbreviation", "Neue Abkürzung"), L("Short name shown on the node.", "Auf dem Knoten angezeigter Kurzname.")),
    },
    ("🕸️dag", "🗃️replace-node-properties"): {
        "properties/id": reference(L("Node", "Knoten"), "node", L("The node to change.", "Der zu ändernde Knoten."), **DAG_NODE),
        "properties/newProperties": record(L("New Properties", "Neue Eigenschaften"), L("Kind-specific fields of the node.", "Artspezifische Felder des Knotens.")),
    },
    ("🕸️dag", "🤝️connect-nodes"): {
        "properties/source": reference(L("Source Node", "Quellknoten"), "node", L("The node the edge starts at.", "Der Knoten, an dem die Kante beginnt."), role="value", **DAG_NODE),
        "properties/target": reference(L("Target Node", "Zielknoten"), "node", L("The node the edge ends at.", "Der Knoten, an dem die Kante endet."), role="value", **DAG_NODE),
        "properties/routeStyle": select(L("Route Style", "Linienführung"), {"bezier": L("Bézier Curve", "Bézierkurve"), "sharpSz": L("Orthogonal S/Z", "Orthogonales S/Z")}, widget="segmented"),
    },
    ("🕸️dag", "🧮change-node-operator-kind"): {
        "properties/id": reference(L("Node", "Knoten"), "node", L("The node to change.", "Der zu ändernde Knoten."), **DAG_NODE),
        "properties/newOperatorKind": text(L("New Operator Kind", "Neue Operatorart"), L("Operator a computation node evaluates; empty clears it.", "Operator, den ein Berechnungsknoten auswertet; leer entfernt ihn.")),
    },
}
#endregion 🕸️Dag

#region 📜️Imperative
IMPERATIVE_STEP = dict(domain="steps", granularity="step")
PATH_REF = {
    "properties/owner": reference(L("Owner Step", "Übergeordneter Schritt"), "step", L("Control step whose body holds the list; empty for the top-level list.", "Steuerschritt, dessen Rumpf die Liste enthält; leer für die oberste Liste."), role="value", **IMPERATIVE_STEP),
    "properties/slot": text(L("Body Slot", "Rumpf-Slot"), L("Name of the owner's body that holds the list.", "Name des Rumpfs des übergeordneten Schritts, der die Liste enthält.")),
}
IMPERATIVE_TARGET = reference(L("Step", "Schritt"), "step", L("The step this mutation addresses.", "Der Schritt, den diese Mutation adressiert."), **IMPERATIVE_STEP)
IMPERATIVE = {
    ("📜️imperative", "📸️replace-config"): {
        "properties/config/properties/contributionsJson": CONTRIBUTIONS,
        "properties/config/properties/runOutputJson": multiline(L("Run Output", "Ausführungsausgabe"), L("Output of the last run as JSON text.", "Ausgabe des letzten Laufs als JSON-Text.")),
    },
    ("📜️imperative", "🌱create-step"): {
        "properties/pathRef": record(L("Path Reference", "Pfadreferenz"), L("The step list that receives the step.", "Die Schrittliste, die den Schritt aufnimmt.")),
        **at("$defs/PathRef", PATH_REF),
        "properties/step": record(L("Step", "Schritt"), L("The complete step record to add.", "Der vollständige hinzuzufügende Schritt.")),
        "$defs/Step/properties/bodies": record(L("Bodies", "Rümpfe"), L("Nested step lists by body slot.", "Verschachtelte Schrittlisten je Rumpf-Slot.")),
    },
    ("📜️imperative", "🔀reorder-steps"): {"properties/pathRef": record(L("Path Reference", "Pfadreferenz"), L("The step list that holds the step.", "Die Schrittliste, die den Schritt enthält.")), **at("properties/pathRef", PATH_REF), "properties/id": IMPERATIVE_TARGET},
    ("📜️imperative", "🔧edit-step-params"): {
        "properties/pathRef": record(L("Path Reference", "Pfadreferenz"), L("The step list that holds the step.", "Die Schrittliste, die den Schritt enthält.")),
        **at("properties/pathRef", PATH_REF),
        "properties/id": IMPERATIVE_TARGET,
        "properties/newParams": record(L("New Parameters", "Neue Parameter"), L("Kind-specific parameters of the step.", "Artspezifische Parameter des Schritts.")),
    },
    ("📜️imperative", "🗑️delete-step"): {"properties/pathRef": record(L("Path Reference", "Pfadreferenz"), L("The step list that holds the step.", "Die Schrittliste, die den Schritt enthält.")), **at("properties/pathRef", PATH_REF), "properties/id": IMPERATIVE_TARGET},
}
#endregion 📜️Imperative

#region ➗️Mathematical
INTEGER_TEXT = L("Integer as a decimal string.", "Ganzzahl als Dezimalzeichenkette.")
MATHEMATICAL = {
    ("➗️mathematical", "🎚️change-coefficient"): {
        "properties/label": count(L("Coefficient", "Koeffizient"), L("Label of the numeric leaf in the equation tree.", "Kennzeichen des Zahlenblatts im Gleichungsbaum.")),
        "properties/numer": text(L("Numerator", "Zähler"), INTEGER_TEXT),
        "properties/denom": text(L("Denominator", "Nenner"), L("Integer as a decimal string; 1 for a plain integer.", "Ganzzahl als Dezimalzeichenkette; 1 für eine einfache Ganzzahl.")),
    },
    ("➗️mathematical", "🔄️replace-points"): {"properties/points": record(L("Points", "Punkte"), L("All points of the geometry in order.", "Alle Punkte der Geometrie in Reihenfolge."))},
    ("➗️mathematical", "🔁️replace-graph"): {
        "properties/graph": record(L("Graph", "Graph"), L("The complete graph record.", "Der vollständige Graph.")),
        "properties/graph/properties/directed": toggle(L("Directed", "Gerichtet"), L("Edges point from source to target.", "Kanten zeigen von der Quelle zum Ziel.")),
        "properties/graph/properties/nodes": record(L("Nodes", "Knoten")),
        "properties/graph/properties/edges": record(L("Edges", "Kanten")),
        "properties/graph/properties/algorithm": text(L("Algorithm", "Algorithmus"), L("Graph algorithm to run, e.g. bfs or dfs.", "Auszuführender Graphalgorithmus, z. B. bfs oder dfs.")),
        "properties/graph/properties/algorithmSeed": text(L("Algorithm Seed", "Algorithmus-Startwert"), L("Input of the algorithm, e.g. the start node id.", "Eingabe des Algorithmus, z. B. die ID des Startknotens.")),
    },
    ("➗️mathematical", "🧭️change-graph-directed"): {"properties/newDirected": toggle(L("Directed", "Gerichtet"), L("Edges point from source to target.", "Kanten zeigen von der Quelle zum Ziel."))},
    ("➗️mathematical", "🧮️update-graph-algorithm"): {
        "properties/newAlgorithm": text(L("New Algorithm", "Neuer Algorithmus"), L("Graph algorithm to run, e.g. bfs or dfs.", "Auszuführender Graphalgorithmus, z. B. bfs oder dfs.")),
        "properties/newAlgorithmSeed": text(L("New Algorithm Seed", "Neuer Algorithmus-Startwert"), L("Input of the algorithm, e.g. the start node id; empty for none.", "Eingabe des Algorithmus, z. B. die ID des Startknotens; leer für keine.")),
    },
}
#endregion ➗️Mathematical

#region 🎞️Animate
CROP = L("Region of the source the tile shows, as fractions of the source size.", "Bereich der Quelle, den die Kachel zeigt, als Anteile der Quellgröße.")
TILE = dict(domain="tiles", granularity="tile")
ANIMATE = {
    ("🎞️animate", "✂️resize-tile-crop"): {
        "properties/id": reference(L("Tile", "Kachel"), "tile", L("The tile to crop.", "Die zuzuschneidende Kachel."), **TILE),
        "properties/newCrop": record(L("New Crop", "Neuer Zuschnitt"), CROP),
    },
    ("🎞️animate", "🆕create-tile"): {
        "properties/tile": record(L("Tile", "Kachel"), L("The complete tile record to add.", "Die vollständige hinzuzufügende Kachel.")),
        "properties/tile/properties/crop": record(L("Crop", "Zuschnitt"), CROP),
    },
    ("🎞️animate", "🔁replace-tiles"): {
        "properties/newTiles": record(L("New Tiles", "Neue Kacheln"), L("All tiles in order.", "Alle Kacheln in Reihenfolge.")),
        "properties/newTiles/items/properties/crop": record(L("Crop", "Zuschnitt"), CROP),
    },
    ("🎞️animate", "🔲resize-source-frame"): {"properties/newFrame": record(L("New Source Frame", "Neuer Quellrahmen"), L("Visible region of the source, as fractions of its size.", "Sichtbarer Bereich der Quelle als Anteile ihrer Größe."))},
    ("🎞️animate", "🖼️replace-source"): {
        "properties/newSource": record(L("New Source", "Neue Quelle"), L("The media the tiles are cut from.", "Das Medium, aus dem die Kacheln geschnitten werden.")),
        "properties/newSource/properties/src": text(L("Source URL", "Quell-URL"), L("Location of the media, or a data URL.", "Ort des Mediums oder eine Daten-URL.")),
        "properties/newSource/properties/kind": text(L("Media Kind", "Medienart"), L("Kind of media, e.g. figure, image or pdf.", "Art des Mediums, z. B. figure, image oder pdf.")),
        "properties/newSource/properties/frame": record(L("Frame", "Rahmen"), L("Visible region of the source, as fractions of its size.", "Sichtbarer Bereich der Quelle als Anteile ihrer Größe.")),
        "properties/newSource/properties/sourceAspect": stepper(L("Source Aspect Ratio", "Seitenverhältnis der Quelle"), L("Width divided by height of the media.", "Breite geteilt durch Höhe des Mediums."), step=0.01, precision=3),
        "properties/newSource/properties/pdfPage": count(L("PDF Page", "PDF-Seite"), L("Page shown from a pdf source.", "Angezeigte Seite einer PDF-Quelle.")),
    },
}
#endregion 🎞️Animate

#region 🌊️Flow
FLOW = {
    ("🌊️flow", "👯️duplicate-widget"): {
        "properties/sourceId": reference(L("Source Widget", "Quell-Widget"), "widget", L("The widget to duplicate.", "Das zu duplizierende Widget."), domain="graph", granularity="node"),
        "properties/synapseId": text(L("Synapse ID", "Synapsen-ID"), L("Id of the new synapse that wires the source to the copy.", "ID der neuen Synapse, die die Quelle mit der Kopie verbindet.")),
    },
    ("🌊️flow", "📍️move-widgets"): {"properties/entries": record(L("Widget Positions", "Widget-Positionen"), L("New layout per widget.", "Neues Layout je Widget."))},
}
#endregion 🌊️Flow

#region 🌍️Gis
GIS = {
    ("🌍️gis", "🏔️gisterrain:🎥️set-camera"): {"$defs/Payload/properties/cameraJson": multiline(L("Camera", "Kamera"), L("Camera state of the terrain view as JSON text.", "Kamerazustand der Geländeansicht als JSON-Text."))},
    ("🌍️gis", "🎚️change-exaggeration"): {"properties/newExaggeration": stepper(L("New Vertical Exaggeration", "Neue Überhöhung"), L("Factor that lifts the terrain heights.", "Faktor, mit dem die Geländehöhen angehoben werden."), step=0.1, precision=2)},
    ("🌍️gis", "📥change-imported-features"): {"properties/newImportedFeaturesJson": multiline(L("New Imported Map Features", "Neue importierte Kartenobjekte"), L("Last imported 2d.map descriptor as JSON text.", "Zuletzt importierter 2d.map-Deskriptor als JSON-Text."))},
}
#endregion 🌍️Gis

#region 🌿️Vcs
VCS = {
    ("🌿️vcs", "🔢change-counter"): {"$defs/Payload/properties/newCounter": count(L("New Counter", "Neuer Zähler"))},
    ("🌿️vcs", "📝change-notes"): {"$defs/Payload/properties/newNotes": multiline(L("New Notes", "Neue Notizen"))},
    ("🌿️vcs", "🚦change-status"): {"$defs/Payload/properties/newStatus": text(L("New Status", "Neuer Status"))},
}
#endregion 🌿️Vcs

#region 🎪️Demonstrator
DEMONSTRATOR = {("🎪️demonstrator", "✒️change-schema"): {"properties/new_schema": text(L("New Schema", "Neues Schema"), L("Schema identity of the playground document.", "Schema-Identität des Playground-Dokuments."))}}
#endregion 🎪️Demonstrator

#region 🎬️Sequence
SEQUENCE_STEP = dict(domain="steps", granularity="step")
COLLAPSED = toggle(L("Collapsed", "Eingeklappt"), L("The step's body is hidden in the editor.", "Der Rumpf des Schritts ist im Editor ausgeblendet."))
SEQUENCE = {
    ("🎬️sequence", "🌱️create-step"): {
        "$defs/Payload/properties/step": record(L("Step", "Schritt"), L("The complete step record to add.", "Der vollständige hinzuzufügende Schritt.")),
        "$defs/Payload/properties/step/properties/collapsed": COLLAPSED,
        "$defs/Payload/properties/step/properties/slot": record(L("Slot", "Slot"), L("Slot of the parent step this step belongs to; empty for a top-level step.", "Slot des übergeordneten Schritts, zu dem dieser Schritt gehört; leer für einen Schritt der obersten Ebene.")),
        "$defs/Payload/properties/step/properties/slot/anyOf/1/properties/owner": reference(L("Parent Step", "Übergeordneter Schritt"), "step", L("The step that owns the slot.", "Der Schritt, dem der Slot gehört."), role="value", **SEQUENCE_STEP),
        "$defs/Payload/properties/step/properties/slot/anyOf/1/properties/name": text(L("Slot Name", "Slot-Name"), L("Name of the parent's slot.", "Name des Slots des übergeordneten Schritts.")),
    },
    ("🎬️sequence", "🗂️change-step-collapsed"): {
        "$defs/Payload/properties/id": reference(L("Step", "Schritt"), "step", L("The step to collapse or expand.", "Der ein- oder auszuklappende Schritt."), **SEQUENCE_STEP),
        "$defs/Payload/properties/collapsed": COLLAPSED,
    },
    ("🎬️sequence", "🧬️duplicate-step"): {"$defs/Payload/properties/sourceId": reference(L("Source Step", "Quellschritt"), "step", L("The step to duplicate.", "Der zu duplizierende Schritt."), **SEQUENCE_STEP)},
}
#endregion 🎬️Sequence

#region 🪐️Space
UPDATED_AT = count(L("Updated At", "Aktualisiert am"), L("Last change time in milliseconds since the Unix epoch.", "Zeitpunkt der letzten Änderung in Millisekunden seit der Unix-Epoche."), unit="ms")
UPDATED_BY = text(L("Updated By", "Aktualisiert von"), L("Actor who changed the artifact last.", "Akteur, der das Artefakt zuletzt geändert hat."))
SPACE = {
    ("🪐️space", "📬️apply-directory-page"): {"properties/pageJson": multiline(L("Directory Page", "Verzeichnisseite"), L("One receipt-sealed hub directory page as canonical DirectoryEventPageV1 JSON.", "Eine quittungsversiegelte Hub-Verzeichnisseite als kanonisches DirectoryEventPageV1-JSON."))},
    ("🪐️space", "🔢️change-catalog-generation"): {"properties/newCatalogGeneration": count(L("New Catalog Generation", "Neue Katalog-Generation"), L("Generation of the installed catalog the home view shows.", "Generation des installierten Katalogs, den die Startansicht zeigt."))},
    ("🪐️space", "🌱create-artifact"): {
        "properties/artifact": record(L("Artifact", "Artefakt"), L("Index row of the new artifact.", "Indexzeile des neuen Artefakts.")),
        "$defs/SpaceArtifactRow/properties/kindId": text(L("Artifact Kind", "Artefaktart"), L("Kind id of the artifact, e.g. s.puzzle.puzzle2d.", "Art-ID des Artefakts, z. B. s.puzzle.puzzle2d.")),
        "$defs/SpaceArtifactRow/properties/schema": text(L("Schema", "Schema"), L("Document schema of the artifact.", "Dokumentschema des Artefakts.")),
        "$defs/SpaceArtifactRow/properties/dialect": record(L("Dialect", "Dialekt"), L("Artifact kind, standard and subset of the artifact.", "Artefaktart, Standard und Teilmenge des Artefakts.")),
        **at("$defs/SpaceArtifactRow/properties/dialect", DIALECT_FIELDS),
        "$defs/SpaceArtifactRow/properties/createdAtMs": count(L("Created At", "Erstellt am"), L("Creation time in milliseconds since the Unix epoch.", "Erstellungszeitpunkt in Millisekunden seit der Unix-Epoche."), unit="ms"),
        "$defs/SpaceArtifactRow/properties/createdBy": text(L("Created By", "Erstellt von"), L("Actor who created the artifact.", "Akteur, der das Artefakt erstellt hat.")),
        "$defs/SpaceArtifactRow/properties/updatedAtMs": UPDATED_AT,
        "$defs/SpaceArtifactRow/properties/updatedBy": UPDATED_BY,
    },
    ("🪐️space", "🕒touch-artifact"): {"properties/updatedAtMs": UPDATED_AT, "properties/updatedBy": UPDATED_BY},
}
#endregion 🪐️Space

#region 🔱️Trinity
SELECTION_START = count(L("Start", "Anfang"), L("Anchor offset of the selection.", "Ankerposition der Auswahl."))
SELECTION_END = count(L("End", "Ende"), L("Caret offset of the selection.", "Position der Einfügemarke der Auswahl."))
TRINITY = {
    ("🔱️trinity", "🔤️set-editor-selection"): {
        "properties/selection": record(L("Selection", "Auswahl"), L("Text selection of the query editor; empty clears it.", "Textauswahl des Abfrageeditors; leer hebt sie auf.")),
        "properties/selection/properties/start": SELECTION_START,
        "properties/selection/properties/end": SELECTION_END,
    },
    ("🔱️trinity", "📊️replace-query-result"): {
        "properties/executionId": text(L("Execution ID", "Ausführungs-ID"), L("Run that produced the result; empty before the first run.", "Lauf, der das Ergebnis erzeugt hat; leer vor dem ersten Lauf.")),
        "properties/result": record(L("Result", "Ergebnis"), L("Query result table; empty when the run failed.", "Ergebnistabelle der Abfrage; leer, wenn der Lauf fehlschlug.")),
        "properties/error": multiline(L("Error", "Fehler"), L("Error message of a failed run.", "Fehlermeldung eines fehlgeschlagenen Laufs.")),
    },
    ("🔱️trinity", "➕️create-node"): {
        "properties/node": record(L("Node", "Knoten"), L("The complete node record to add.", "Der vollständige hinzuzufügende Knoten.")),
        "properties/node/properties/ports": record(L("Ports", "Ports"), L("Input and output ports of the node.", "Ein- und Ausgangsports des Knotens.")),
        "properties/node/properties/ports/items/properties/direction": select(L("Direction", "Richtung"), {"in": L("Input", "Eingang"), "out": L("Output", "Ausgang")}, widget="segmented"),
    },
    ("🔱️trinity", "🔧️change-data-property"): {"properties/new_value": record(L("New Value", "Neuer Wert"), L("Property value; any JSON value.", "Eigenschaftswert; beliebiger JSON-Wert."))},
    ("🔱️trinity", "👈️edit-lhs"): {"properties/newLhsJson": multiline(L("New Match Pattern", "Neues Suchmuster"), L("Left-hand side of the rule as JSON.", "Linke Regelseite als JSON."))},
    ("🔱️trinity", "👉️edit-rhs"): {"properties/newRhsJson": multiline(L("New Replacement", "Neue Ersetzung"), L("Right-hand side of the rule as JSON.", "Rechte Regelseite als JSON."))},
    ("🔱️trinity", "🖼️edit-before-fixture"): {"properties/newBeforeFixtureJson": multiline(L("New Before Graph", "Neuer Vorher-Graph"), L("Working graph the rule is applied to, as trinity.graph JSON.", "Arbeitsgraph, auf den die Regel angewendet wird, als trinity.graph-JSON."))},
    ("🔱️trinity", "📐️change-rule-layout-point"): {"properties/newPoint": record(L("New Point", "Neuer Punkt"), L("Layout position of the rule element.", "Layoutposition des Regelelements."))},
    ("🔱️trinity", "🔧️change-parameter-binding"): {"properties/newValue": record(L("New Value", "Neuer Wert"), L("Value bound to the rule parameter.", "An den Regelparameter gebundener Wert."))},
}
#endregion 🔱️Trinity

#region ✒️Writer
CONTEXT = L("Up to 32 characters of the author's context, used to relocate the splice.", "Bis zu 32 Zeichen Kontext des Autors, um die Stelle wiederzufinden.")
WRITER = {
    ("✒️writer", "🌐change-language"): {"properties/newLanguageId": text(L("New Language", "Neue Sprache"), L("Language id of the document, e.g. markdown.", "Sprach-ID des Dokuments, z. B. markdown."))},
    ("✒️writer", "🔗change-uri"): {"properties/newUri": text(L("New URI", "Neue URI"), L("Location the document is associated with.", "Ort, dem das Dokument zugeordnet ist."))},
    ("✒️writer", "✂️splice-text"): {
        "properties/start": count(L("Start", "Anfang"), L("Offset of the splice in Unicode scalar values.", "Position des Eingriffs in Unicode-Skalarwerten.")),
        "properties/deleted": multiline(L("Deleted Text", "Gelöschter Text")),
        "properties/insert": multiline(L("Inserted Text", "Eingefügter Text")),
        "properties/before": multiline(L("Context Before", "Kontext davor"), CONTEXT),
        "properties/after": multiline(L("Context After", "Kontext danach"), CONTEXT),
    },
    ("✒️writer", "⚙️set-editor-settings"): {
        "properties/settings": record(L("Editor Settings", "Editoreinstellungen")),
        "properties/settings/properties/showLineNumbers": toggle(L("Line Numbers", "Zeilennummern")),
        "properties/settings/properties/fontPx": count(L("Font Size", "Schriftgröße"), unit="px"),
        "properties/settings/properties/lineHeight": count(L("Line Height", "Zeilenhöhe"), unit="px"),
        "properties/settings/properties/tabSize": count(L("Tab Size", "Tabulatorgröße"), L("Columns per indentation level.", "Spalten je Einrückungsebene.")),
    },
    ("✒️writer", "📐️set-editor-selection"): {
        "properties/selection": record(L("Selection", "Auswahl"), L("Text selection of this window; empty clears it.", "Textauswahl dieses Fensters; leer hebt sie auf.")),
        "properties/selection/anyOf/1/properties/start": SELECTION_START,
        "properties/selection/anyOf/1/properties/end": SELECTION_END,
        "properties/selection/anyOf/1/properties/splice": count(L("Applied Splice", "Angewandter Eingriff"), L("Sequence number of the last text splice this window applied; 0 for none.", "Sequenznummer des zuletzt von diesem Fenster angewandten Texteingriffs; 0 für keinen.")),
    },
}
#endregion ✒️Writer

#region 🎯️Targets
ENTITIES = {
    "node": (L("Node", "Knoten"), L("Nodes", "Knoten"), "m"),
    "edge": (L("Edge", "Kante"), L("Edges", "Kanten"), "f"),
    "widget": (L("Widget", "Widget"), L("Widgets", "Widgets"), "n"),
    "synapse": (L("Synapse", "Synapse"), L("Synapses", "Synapsen"), "f"),
    "tile": (L("Tile", "Kachel"), L("Tiles", "Kacheln"), "f"),
    "step": (L("Step", "Schritt"), L("Steps", "Schritte"), "m"),
    "machine": (L("Machine", "Maschine"), L("Machines", "Maschinen"), "f"),
    "object": (L("Object", "Objekt"), L("Objects", "Objekte"), "n"),
    "artifact": (L("Artifact", "Artefakt"), L("Artifacts", "Artefakte"), "n"),
    "route": (L("Route", "Route"), L("Routes", "Routen"), "f"),
    "position": (L("Position", "Position"), L("Positions", "Positionen"), "f"),
    "region": (L("Region", "Region"), L("Regions", "Regionen"), "f"),
}
ARTICLE = {"m": ("Der", "den"), "f": ("Die", "die"), "n": ("Das", "das")}


def target(kind, domain=None, granularity=None, many=False):
    """🎯️ The addressed entity of a mutation: a reference input with `role: target` and the artifact's own entity kind."""
    one, several, gender = ENTITIES[kind]
    if many:
        description = L("The %s this mutation addresses." % several["en"].lower(), "Die %s, die diese Mutation adressiert." % several["de"])
        return reference(several, kind, description, domain=domain, granularity=granularity)
    article, relative = ARTICLE[gender]
    description = L("The %s this mutation addresses." % one["en"].lower(), "%s %s, %s diese Mutation adressiert." % (article, one["de"], relative))
    return reference(one, kind, description, domain=domain, granularity=granularity)


def targets(plugin, leaves, pointer, *args, **kwargs):
    return {(plugin, leaf): {pointer: target(*args, **kwargs)} for leaf in leaves}


TARGETS = {
    **targets("➗️mathematical", ["❌️delete-node", "🕹️move-node", "🏷️change-node-label"], "properties/id", "node"),
    **targets("➗️mathematical", ["🗑️delete-nodes"], "properties/ids", "node", many=True),
    **targets("➗️mathematical", ["✂️disconnect-nodes"], "properties/id", "edge"),
    **targets("🌊️flow", ["🗑️delete-widget", "🔁️replace-widget", "🔢️reorder-widgets"], "properties/id", "widget", "graph", "node"),
    **targets("🌊️flow", ["✂️disconnect-widgets", "🔄️update-synapse-endpoints", "🔀️reorder-synapses"], "properties/id", "synapse", "graph", "edge"),
    **targets("🌍️gis", ["✂️delete-route", "♻️replace-route-data", "🧭reorder-routes"], "properties/id", "route"),
    **targets("🌍️gis", ["🗑️delete-position", "🔁replace-position-data", "🔀reorder-positions"], "properties/id", "position"),
    **targets("🌍️gis", ["🧹delete-region", "🔄replace-region-data", "🔃reorder-regions"], "properties/id", "region"),
    **targets("🎞️animate", ["✏️rename-tile", "🔀reorder-tiles", "🗑️delete-tile"], "properties/id", "tile", "tiles", "tile"),
    **targets("🎞️animate", ["🧹delete-tiles"], "properties/ids", "tile", "tiles", "tile", many=True),
    **targets("🎬️sequence", ["🔧️edit-step-params", "📍️move-step", "🗑️delete-step"], "$defs/Payload/properties/id", "step", "steps", "step"),
    **targets("🎬️sequence", ["✂️disconnect-steps"], "$defs/Payload/properties/id", "edge"),
    **targets("🏭️process", ["🏷️rename-step", "🔀reorder-steps", "🔘change-step-enabled", "🗑️delete-step"], "properties/id", "step"),
    **targets("🏭️process", ["🔖rename-machine", "❌delete-machine"], "properties/id", "machine"),
    **targets("💠️lowpoly", ["🔄️rotate-object", "↗️move-object", "💀️delete-object", "🔀️reorder-objects", "📐️scale-object", "🏷️rename-object", "🧨delete-mesh"], "properties/id", "object", "mesh", "object"),
    **targets("💠️lowpoly", ["➖️remove-paint-layer", "🔖️rename-paint-layer", "🌫️change-paint-layer-opacity", "👁️change-paint-layer-visible"], "properties/objectId", "object", "mesh", "object"),
    **targets("💡️reasoning", ["📐resize-node", "🗑️delete-node", "🧭move-node", "✏️edit-node-text", "🔷change-node-shape"], "properties/nodeId", "node", "graph", "node"),
    **targets("📖️playbook", ["↔️move-step", "➖remove-step", "🩹update-step"], "properties/stepId", "step", "blocks", "step"),
    **targets("🔱️trinity", ["🔌️jack:🗑️delete-node", "🔌️jack:📍️move-node", "🔌️jack:✏️rename-node"], "properties/id", "node", "ast", "node"),
    **targets("🔱️trinity", ["🔌️jack:✂️delete-edge"], "properties/id", "edge"),
    **targets("🕸️dag", ["📐resize-node", "🔁replace-node-kind", "🔤change-node-name", "🗑️delete-node", "🖼️change-node-icon", "🏷️rename-node", "↔️move-node"], "properties/id", "node", "graph", "node"),
    **targets("🕸️dag", ["✂️disconnect-nodes"], "properties/id", "edge", "graph", "edge"),
    **targets("🪐️space", ["🏷️rename-artifact", "🗑️delete-artifact", "🕒touch-artifact"], "properties/id", "artifact"),
    **targets("🪵️sourcing", ["🗑️delete-curated-item", "🔢change-curated-item-count"], "properties/objectId", "object", "rows", "object"),
}

def endpoints(kind, source, target, **ref):
    """🔗️ The two endpoint references of a connection record (values, not the mutation's target)."""
    one, _, gender = ENTITIES[kind]
    noun = one["de"]
    compound = (lambda head: head + "-" + noun) if noun[:1].isupper() and noun.lower() in ("widget",) else (lambda head: head + noun.lower())
    article = ARTICLE[gender][0]
    where = "an der" if gender == "f" else "an dem"
    return {
        source: reference(L("Source %s" % one["en"], compound("Quell")), kind, L("The %s the connection starts at." % one["en"].lower(), "%s %s, %s die Verbindung beginnt." % (article, noun, where)), role="value", **ref),
        target: reference(L("Target %s" % one["en"], compound("Ziel")), kind, L("The %s the connection ends at." % one["en"].lower(), "%s %s, %s die Verbindung endet." % (article, noun, where)), role="value", **ref),
    }


CONNECTIONS = {
    ("➗️mathematical", "🔗️connect-nodes"): {"properties/" + key: value for key, value in endpoints("node", "source", "target").items()},
    ("➗️mathematical", "🔁️replace-graph"): {"properties/graph/properties/edges/items/properties/" + key: value for key, value in endpoints("node", "source", "target").items()},
    ("🌊️flow", "🔌️connect-widgets"): {"properties/" + key: value for key, value in endpoints("widget", "from", "to", domain="graph", granularity="node").items()},
    ("🌊️flow", "🔄️update-synapse-endpoints"): {"properties/" + key: value for key, value in endpoints("widget", "from", "to", domain="graph", granularity="node").items()},
    ("🎬️sequence", "🔗️connect-steps"): {"$defs/Payload/properties/" + key: value for key, value in endpoints("step", "from", "to", domain="steps", granularity="step").items()},
    ("🔱️trinity", "🔌️jack:🌉️create-edge"): {"properties/edge/properties/" + key: value for key, value in endpoints("node", "source", "target", domain="ast", granularity="node").items()},
    ("🪵️sourcing", "🌱create-curated-item"): {"properties/item/properties/objectId": reference(L("Object", "Objekt"), "object", L("The pool object to curate.", "Das zu kuratierende Pool-Objekt."), role="value", domain="rows", granularity="object")},
    ("💠️lowpoly", "🕸️create-mesh"): {"properties/childId": CHILD_ID},
    ("💠️lowpoly", "🌱️create-object"): {"properties/object/properties/mesh/oneOf/1/properties/childId": CHILD_ID},
    ("🏭️process", "🧊replace-stock-solid"): {"properties/newSolid/properties/childId": CHILD_ID},
    **{("💠️lowpoly", leaf): {"properties/index": LAYER_INDEX} for leaf in ["➖️remove-paint-layer", "🔖️rename-paint-layer", "🌫️change-paint-layer-opacity", "👁️change-paint-layer-visible"]},
}
#endregion 🎯️Targets

ANNOTATIONS = {**PLAYBOOK, **LOWPOLY, **PROCESS, **REASONING, **DAG, **IMPERATIVE, **MATHEMATICAL, **ANIMATE, **FLOW, **GIS, **VCS, **DEMONSTRATOR, **SEQUENCE, **SPACE, **TRINITY, **WRITER}
for _key, _table in [*TARGETS.items(), *CONNECTIONS.items()]:
    ANNOTATIONS[_key] = {**_table, **ANNOTATIONS.get(_key, {})}


def leaf_path(plugin, leaf):
    within, _, leaf = leaf.rpartition(":")
    found = []
    for directory, subdirs, names in os.walk(os.path.join(PLUGINS, plugin)):
        subdirs[:] = [d for d in subdirs if d not in ("node_modules", "target", "🗑️generated")]
        parts = directory.split("/")
        if parts[-1] == "🧬️schema" and len(parts) > 2 and parts[-2] == leaf and parts[-3] == "🧬️mutations" and "🔣️.json" in names and (within == "" or within in parts):
            found.append(os.path.join(directory, "🔣️.json"))
    if len(found) != 1:
        raise SystemExit("%s/%s resolves to %d leaf schemas" % (plugin, leaf, len(found)))
    return found[0]


def is_scalar(value):
    return not isinstance(value, (dict, list))


def compact(value, indent=0):
    pad = " " * (indent + 2)
    if isinstance(value, dict):
        if not value:
            return "{}"
        if all(is_scalar(v) or (isinstance(v, list) and all(is_scalar(x) for x in v)) for v in value.values()):
            return "{ " + ", ".join(json.dumps(k, ensure_ascii=False) + ": " + compact(v) for k, v in value.items()) + " }"
        return "{\n" + ",\n".join(pad + json.dumps(k, ensure_ascii=False) + ": " + compact(v, indent + 2) for k, v in value.items()) + "\n" + " " * indent + "}"
    if isinstance(value, list):
        if all(is_scalar(x) for x in value):
            return "[" + ", ".join(compact(x) for x in value) + "]"
        return "[\n" + ",\n".join(pad + compact(x, indent + 2) for x in value) + "\n" + " " * indent + "]"
    return json.dumps(value, ensure_ascii=False)


def formatter(source):
    document = json.loads(source)
    for indent in (2, 4, "\t"):
        if json.dumps(document, indent=indent, ensure_ascii=False) + "\n" == source:
            return lambda value, indent=indent: json.dumps(value, indent=indent, ensure_ascii=False) + "\n"
    if compact(document) + "\n" == source:
        return lambda value: compact(value) + "\n"
    raise SystemExit("unknown formatting")


def node_at(document, path):
    node = document
    for segment in path.split("/"):
        node = node[int(segment)] if isinstance(node, list) else node[segment]
    return node


def main():
    written = 0
    sites = 0
    for (plugin, leaf), table in sorted(ANNOTATIONS.items()):
        path = leaf_path(plugin, leaf)
        source = open(path, encoding="utf-8").read()
        dump = formatter(source)
        document = json.loads(source)
        for pointer, annotation in table.items():
            node = node_at(document, pointer)
            if not isinstance(node, dict):
                raise SystemExit("%s: %s is not a schema node" % (path, pointer))
            existing = node.get("x-semio-ui")
            if existing is not None and existing != annotation:
                raise SystemExit("%s: %s already carries a different x-semio-ui" % (path, pointer))
            node.pop("x-semio-ui", None)
            node["x-semio-ui"] = annotation
            sites += 1
        output = dump(document)
        if output != source:
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(output)
            written += 1
    print("annotated %d sites in %d leaf schemas (%d rewritten)" % (sites, len(ANNOTATIONS), written))


if __name__ == "__main__":
    sys.exit(main())
