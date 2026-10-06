"""🎛️ S5-GRAPHS-WIRES — declared `x-semio-ui` for the 39 inferred mutation inputs of mathematical, reasoning, dag,
imperative and space (design §22.8: the input gate measures declarations, not inference).

Every row names one schema node by its explicit path inside one leaf payload schema and the control it declares: widget,
role, en + de label and description, and a `step` for every interactive number. Conventions are the ones already
declared by the sibling leaves: `Pan X` / `Pan Y` / log `Zoom` slider for cameras (trinity graph windows), a hidden
discriminator for an undo-owned insert index (`create-node.at` of the shared graph vocabulary), `reference` + `target`
for an entity an index addresses (`move-points.indices`), `text` for a minted identity (`rename-node.new_id`).

    python3 🧪️s5-graphs-input-declarations.py --check   # report what would change, write nothing
    python3 🧪️s5-graphs-input-declarations.py --apply   # write the declarations

Fails closed: explicit files only, a missing node or a node that already declares a DIFFERENT control stops the run
before any write, and a file is rewritten only when it round-trips byte for byte through the 2-space JSON form the
plugin schemas use (the one exception is named below and is normalized to that form).
"""

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
PLUGINS = "✏️s/🔌️plugins/"
EQUATION = PLUGINS + "➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/"
WIRES_CANVAS = PLUGINS + "💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/"
PROCEDURE_CONFIG = PLUGINS + "📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/"
SPACE = PLUGINS + "🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
DAG_CONFIG = PLUGINS + "🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/"
LEAF = "/🧬️schema/🔣️.json"
NORMALIZED = {WIRES_CANVAS + "🎚️config/🧬️schema/🧬️mutations/🎥️set-camera" + LEAF}


def label(en: str, de: str) -> dict:
    return {"en": en, "de": de}


def control(widget, role, name, about, **more) -> dict:
    """🧱️ One `x-semio-ui` object in the key order the committed schemas use."""
    declared = {} if widget is None else {"widget": widget}
    declared.update({"role": role, "label": label(*name), "description": label(*about)})
    declared.update(more)
    return declared


def number(name, about, step=1, precision=2, **more) -> dict:
    return control("stepper", "value", name, about, step=step, precision=precision, **more)


def text(name, about, **more) -> dict:
    return control("text", "value", name, about, **more)


def insert_index(en: str, de: str) -> dict:
    return control("hidden", "discriminator", ("Position in set", "Position in der Menge"), (en, de))


CAMERA = control(None, "value", ("Camera", "Kamera"), ("Pan offset and zoom factor of the view.", "Verschiebung und Zoomfaktor der Ansicht."))
PAN_X = number(("Pan X", "Verschiebung X"), ("Horizontal pan offset of the view.", "Horizontale Verschiebung der Ansicht."), precision=1)
PAN_Y = number(("Pan Y", "Verschiebung Y"), ("Vertical pan offset of the view.", "Vertikale Verschiebung der Ansicht."), precision=1)
ZOOM = control("slider", "value", ("Zoom", "Zoom"), ("Zoom factor of the view.", "Zoomfaktor der Ansicht."), step=0.05, precision=2, softMin=0.1, softMax=8, scale="log", snaps=[0.25, 0.5, 1, 2, 4])
NODE_ID = text(("Node Id", "Knoten-ID"), ("Identity of the node; no other node may carry it.", "Identität des Knotens; kein anderer Knoten darf sie tragen."))
NODE_LABEL = text(("Label", "Beschriftung"), ("Text shown on the node.", "Text, der am Knoten angezeigt wird."))
NODE_X = number(("Position X", "Position X"), ("Horizontal canvas position of the node.", "Horizontale Zeichenflächenposition des Knotens."))
NODE_Y = number(("Position Y", "Position Y"), ("Vertical canvas position of the node.", "Vertikale Zeichenflächenposition des Knotens."))
EDGE_ID = text(("Connection Id", "Verbindungs-ID"), ("Identity of the connection; no other connection may carry it.", "Identität der Verbindung; keine andere Verbindung darf sie tragen."))
POINT_X = number(("X", "X"), ("Horizontal coordinate of the point.", "Horizontale Koordinate des Punkts."))
POINT_Y = number(("Y", "Y"), ("Vertical coordinate of the point.", "Vertikale Koordinate des Punkts."))


def ordered(declared: dict, order: int) -> dict:
    return {**declared, "order": order}


CAMERA_OBJECT = [(["properties", "camera"], CAMERA), (["properties", "camera", "properties", "x"], PAN_X), (["properties", "camera", "properties", "y"], PAN_Y), (["properties", "camera", "properties", "zoom"], ZOOM)]
GRAPH_NODE = ["properties", "graph", "properties", "nodes", "items", "properties"]

DECLARATIONS = {
    EQUATION + "✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera" + LEAF: CAMERA_OBJECT,
    EQUATION + "📐️geometry/🧬️schema/🧬️mutations/➕️insert-point" + LEAF: [
        (["properties", "index"], ordered(number(("Position", "Position"), ("Index the new point takes in the ordered point list.", "Index, den der neue Punkt in der geordneten Punktliste einnimmt."), precision=0), 10)),
        (["properties", "x"], ordered(POINT_X, 20)),
        (["properties", "y"], ordered(POINT_Y, 30)),
    ],
    EQUATION + "📐️geometry/🧬️schema/🧬️mutations/➖️remove-point" + LEAF: [
        (["properties", "index"], control("reference", "target", ("Point", "Punkt"), ("The point this mutation removes, by base index.", "Der Punkt, den diese Mutation entfernt, nach Basisindex."), ref={"kind": "point"})),
    ],
    EQUATION + "📐️geometry/🧬️schema/🧬️mutations/🔄️replace" + LEAF: [
        (["properties", "points", "items", "properties", "x"], POINT_X),
        (["properties", "points", "items", "properties", "y"], POINT_Y),
    ],
    EQUATION + "🕸️graph/🧬️schema/🧬️mutations/🏷️change-node" + LEAF: [
        (["properties", "newLabel"], text(("New Label", "Neue Beschriftung"), ("Text the node shows from now on.", "Text, den der Knoten von nun an anzeigt."))),
    ],
    EQUATION + "🕸️graph/🧬️schema/🧬️mutations/🔗️connect-nodes" + LEAF: [
        (["properties", "id"], EDGE_ID),
        (["properties", "index"], insert_index("Index the connection is inserted at; absent appends it. Undo of a disconnect names the removed index.", "Index, an dem die Verbindung eingefügt wird; fehlt er, wird angehängt. Das Rückgängigmachen eines Trennens nennt den entfernten Index.")),
    ],
    EQUATION + "🕸️graph/🧬️schema/🧬️mutations/➕️create-node" + LEAF: [
        (["properties", "id"], ordered(NODE_ID, 10)),
        (["properties", "label"], ordered(NODE_LABEL, 20)),
        (["properties", "x"], ordered(NODE_X, 30)),
        (["properties", "y"], ordered(NODE_Y, 40)),
        (["properties", "index"], insert_index("Index the node is inserted at; absent appends it. Undo of a delete names the removed index.", "Index, an dem der Knoten eingefügt wird; fehlt er, wird angehängt. Das Rückgängigmachen eines Löschens nennt den entfernten Index.")),
    ],
    EQUATION + "🕸️graph/🧬️schema/🧬️mutations/🕹️move-node" + LEAF: [
        (["properties", "x"], number(("Position X", "Position X"), ("Horizontal canvas position the node moves to.", "Horizontale Zeichenflächenposition, an die der Knoten verschoben wird."))),
        (["properties", "y"], number(("Position Y", "Position Y"), ("Vertical canvas position the node moves to.", "Vertikale Zeichenflächenposition, an die der Knoten verschoben wird."))),
    ],
    EQUATION + "🕸️graph/🧬️schema/🧬️mutations/🔁️replace-graph" + LEAF: [
        (GRAPH_NODE + ["id"], NODE_ID),
        (GRAPH_NODE + ["label"], NODE_LABEL),
        (GRAPH_NODE + ["x"], NODE_X),
        (GRAPH_NODE + ["y"], NODE_Y),
        (["properties", "graph", "properties", "edges", "items", "properties", "id"], EDGE_ID),
    ],
    WIRES_CANVAS + "🫧️transient/🧬️schema/🧬️mutations/🖱️set-drag" + LEAF: [
        (["properties", "zoom"], control("slider", "value", ("Zoom", "Zoom"), ("Zoom factor of the view while the drag runs; pointer offsets are divided by it.", "Zoomfaktor der Ansicht während des Ziehens; Zeigerversätze werden durch ihn geteilt."), step=0.05, precision=2, softMin=0.1, softMax=8, scale="log", snaps=[0.25, 0.5, 1, 2, 4])),
    ],
    WIRES_CANVAS + "🎚️config/🧬️schema/🧬️mutations/🎥️set-camera" + LEAF: CAMERA_OBJECT,
    PROCEDURE_CONFIG + "📸️replace-config" + LEAF: [
        (["properties", "config"], control(None, "value", ("Configuration", "Konfiguration"), ("Whole view configuration of the imperative editor.", "Gesamte Ansichtskonfiguration des Imperativ-Editors."))),
    ],
    PROCEDURE_CONFIG + "🧩️set-contributions" + LEAF: [
        (["properties", "json"], control("multiline", "value", ("Contributions", "Beiträge"), ("Contributions of this editor as JSON text.", "Beiträge dieses Editors als JSON-Text."))),
    ],
    PROCEDURE_CONFIG + "📤️set-run-output" + LEAF: [
        (["properties", "json"], control("multiline", "value", ("Run Output", "Ausführungsausgabe"), ("Output of the last run as JSON text.", "Ausgabe des letzten Laufs als JSON-Text."))),
    ],
    DAG_CONFIG + "🎥️change-camera" + LEAF: [(["properties", "x"], PAN_X), (["properties", "y"], PAN_Y), (["properties", "zoom"], ZOOM)],
    SPACE + "🌱create-artifact" + LEAF: [
        (["$defs", "SpaceArtifactRow", "properties", "id"], text(("Artifact Id", "Artefakt-ID"), ("Identity of the new artifact; no other artifact may carry it.", "Identität des neuen Artefakts; kein anderes Artefakt darf sie tragen."))),
        (["$defs", "SpaceArtifactRow", "properties", "name"], text(("Name", "Name"), ("Display name of the artifact.", "Anzeigename des Artefakts."))),
    ],
    SPACE + "🏷️rename-artifact" + LEAF: [
        (["properties", "newName"], text(("New Name", "Neuer Name"), ("Name the artifact shows from now on.", "Name, den das Artefakt von nun an trägt."))),
    ],
}


def canonical(document: dict) -> str:
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) == 2 else ""
    if mode not in ("--check", "--apply"):
        raise SystemExit("usage: 🧪️s5-graphs-input-declarations.py --check | --apply")
    if not DECLARATIONS or not (ROOT / PLUGINS).is_dir():
        raise SystemExit("[input-declarations] REFUSED: empty table or missing plugin root")
    planned, refusals, declared, kept = {}, [], 0, 0
    for relative, rows in DECLARATIONS.items():
        path = ROOT / relative
        if not path.is_file():
            refusals.append("%s: missing" % relative)
            continue
        source = path.read_text()
        document = json.loads(source)
        if canonical(document) != source and relative not in NORMALIZED:
            refusals.append("%s: does not round-trip through the 2-space form" % relative)
            continue
        for pointer, declaration in rows:
            node = document
            for key in pointer:
                node = node.get(key) if isinstance(node, dict) else None
            if not isinstance(node, dict):
                refusals.append("%s: no schema node at %s" % (relative, "/".join(pointer)))
            elif node.get("x-semio-ui") == declaration:
                kept += 1
            elif "x-semio-ui" in node:
                refusals.append("%s: %s already declares a different control" % (relative, "/".join(pointer)))
            else:
                node["x-semio-ui"] = declaration
                declared += 1
        if canonical(document) != source:
            planned[path] = canonical(document)
    if refusals:
        raise SystemExit("[input-declarations] REFUSED, nothing written:\n  " + "\n  ".join(refusals))
    if mode == "--apply":
        for path, content in planned.items():
            path.write_text(content)
    print("[input-declarations] %s: %d declaration(s) in %d file(s), %d already declared, %d file(s) of %d untouched" % ("applied" if mode == "--apply" else "would apply", declared, len(planned), kept, len(DECLARATIONS) - len(planned), len(DECLARATIONS)))


main()
