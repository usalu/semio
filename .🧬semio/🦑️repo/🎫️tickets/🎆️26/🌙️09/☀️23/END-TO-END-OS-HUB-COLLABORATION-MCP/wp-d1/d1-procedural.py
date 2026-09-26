"""🌀️ D1 codemod: agent-facing en/de descriptions for procedural generation2d/generation3d editors, the generation3d viewer and the plugin command."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
from d1_apply import describe, ROOT

P = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts"

def generations(dim):
    return [
        ("addGeneration", f"Adds a new generation, a named set of input values for the {dim} generator, to the generation list and selects it.", f"Fügt der Generationsliste eine neue Generation hinzu, einen benannten Satz von Eingabewerten für den {dim}-Generator, und wählt sie aus."),
        ("selectGeneration", "Selects the generation with the given id, whose input values the generator then evaluates and shows.", "Wählt die Generation mit der angegebenen Id aus, deren Eingabewerte der Generator dann auswertet und zeigt."),
        ("renameGeneration", "Renames one generation of the generation list.", "Benennt eine Generation der Generationsliste um."),
        ("removeGeneration", "Removes one generation by id from the generation list, with its input values.", "Entfernt eine Generation anhand ihrer Id samt ihrer Eingabewerte aus der Generationsliste."),
        ("updateGenerationValues", "Sets the value one input question takes in a generation (the selected one when no id is given) and re-evaluates the result.", "Setzt den Wert, den eine Eingabefrage in einer Generation annimmt (ohne Id in der ausgewählten), und wertet das Ergebnis neu aus."),
        ("addWidget", f"Adds a new widget of the given kind (an input or an operator of the {dim} generator graph) to the canvas.", f"Fügt der Fläche ein neues Widget der angegebenen Art hinzu (eine Eingabe oder einen Operator des {dim}-Generatorgraphen)."),
        ("removeWidget", "Removes one widget by id from the generator graph together with its connections.", "Entfernt ein Widget anhand seiner Id samt seiner Verbindungen aus dem Generatorgraphen."),
        ("reorganize", "Lays out every widget of the generator graph automatically from left to right, overwriting their manual positions.", "Ordnet alle Widgets des Generatorgraphen automatisch von links nach rechts an und überschreibt ihre manuellen Positionen."),
        ("setShowMode", "Sets what the editor shows, such as the generator graph or the generated result; only the view changes.", "Legt fest, was der Editor zeigt, etwa den Generatorgraphen oder das erzeugte Ergebnis; nur die Ansicht ändert sich."),
        ("setActiveExample", f"Replaces the whole {dim} generator with one of the plugin's bundled examples, by example id.", f"Ersetzt den gesamten {dim}-Generator durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."),
    ]

describe(f"{P}/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_generation2d_app", generations("2D") + [
    ("moveMediaNode", "Moves one widget to the canvas position x, y.", "Verschiebt ein Widget an die Position x, y der Fläche."),
    ("connectMediaPorts", "Connects an output port of one widget to an input port of another; an incompatible connection changes nothing.", "Verbindet einen Ausgangsport eines Widgets mit einem Eingangsport eines anderen; eine unverträgliche Verbindung ändert nichts."),
    ("generate", "Switches the editor to its generate view, which shows the generation list and the result of the selected generation.", "Schaltet den Editor in seine Generieren-Ansicht, die die Generationsliste und das Ergebnis der ausgewählten Generation zeigt."),
    ("setEvalOutputs", "Hands the editor evaluated generator outputs (JSON) computed elsewhere so it can show them; the document is not changed.", "Übergibt dem Editor anderswo berechnete Generatorausgaben (JSON), damit er sie zeigt; das Dokument ändert sich nicht."),
], [("nodeGraphEdit", "Input"), ("nodeGraphViewport", "Chrome")], destructive=["reorganize"])

describe(f"{P}/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_generation3d_app", generations("3D") + [
    ("setLodMode", "Sets the level of detail the 3D preview draws the generated result with; only the view changes.", "Legt die Detailstufe fest, mit der die 3D-Vorschau das erzeugte Ergebnis zeichnet; nur die Ansicht ändert sich."),
    ("selectNextNode", "Moves the graph selection to the next widget in the generator graph.", "Bewegt die Auswahl im Generatorgraphen zum nächsten Widget."),
    ("selectPreviousNode", "Moves the graph selection to the previous widget in the generator graph.", "Bewegt die Auswahl im Generatorgraphen zum vorherigen Widget."),
    ("selectUpstreamNode", "Moves the graph selection to a widget feeding the selected one.", "Bewegt die Auswahl im Generatorgraphen zu einem Widget, das das ausgewählte speist."),
    ("selectDownstreamNode", "Moves the graph selection to a widget fed by the selected one.", "Bewegt die Auswahl im Generatorgraphen zu einem Widget, das vom ausgewählten gespeist wird."),
    ("activateSelection", "Opens the ports of the selected widget in the graph window, as pressing Enter on it does; only the view changes.", "Öffnet die Ports des ausgewählten Widgets im Graphfenster, wie ein Druck auf die Eingabetaste; nur die Ansicht ändert sich."),
    ("toggleSun", "Switches the sun light of the 3D preview on or off; only the view changes.", "Schaltet das Sonnenlicht der 3D-Vorschau ein oder aus; nur die Ansicht ändert sich."),
    ("setSunAzimuth", "Sets the compass direction the 3D preview's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne der 3D-Vorschau scheint; nur die Ansicht ändert sich."),
    ("setSunElevation", "Sets how high the 3D preview's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne der 3D-Vorschau über dem Horizont steht; nur die Ansicht ändert sich."),
    ("setSunIntensity", "Sets the brightness of the 3D preview's sun; only the view changes.", "Legt die Helligkeit der Sonne der 3D-Vorschau fest; nur die Ansicht ändert sich."),
    ("translateSelection", "Moves the selected generated objects by a translation; consecutive drags merge into one undo step.", "Verschiebt die ausgewählten erzeugten Objekte um eine Translation; aufeinanderfolgende Züge werden zu einem Rückgängig-Schritt zusammengefasst."),
    ("rotateSelection", "Rotates the selected generated objects around an axis by an angle.", "Dreht die ausgewählten erzeugten Objekte um eine Achse und einen Winkel."),
    ("scaleSelection", "Scales the selected generated objects by per-axis factors.", "Skaliert die ausgewählten erzeugten Objekte um Faktoren je Achse."),
    ("deleteSelection", "Deletes every selected widget from the generator graph together with its connections.", "Löscht alle ausgewählten Widgets samt ihrer Verbindungen aus dem Generatorgraphen."),
    ("patchFlowWidgets", "Sets one numeric field (such as a slider value) on several widgets at once; drags with the same gesture merge into one undo step.", "Setzt ein Zahlenfeld (etwa einen Schiebereglerwert) auf mehreren Widgets zugleich; Züge derselben Geste werden zu einem Rückgängig-Schritt zusammengefasst."),
    ("importDocumentRequest", "Opens the host's file picker for a 3D artifact file; the chosen file is then imported as the generator document.", "Öffnet die Dateiauswahl des Hosts für eine 3D-Artefaktdatei; die gewählte Datei wird dann als Generatordokument importiert."),
    ("exportDocument", "Writes the generated 3D result in the chosen format to a downloaded file on the user's machine.", "Schreibt das erzeugte 3D-Ergebnis im gewählten Format in eine heruntergeladene Datei auf dem Rechner des Nutzers."),
    ("importDocument", "Replaces the generator document with one read from an imported 3D artifact file, delivered in chunks.", "Ersetzt das Generatordokument durch eines aus einer importierten 3D-Artefaktdatei, die in Teilen geliefert wird."),
    ("cycleShowMode", "Switches the editor to the next show mode in turn (such as graph, result or wireframe); only the view changes.", "Wechselt den Editor reihum in den nächsten Anzeigemodus (etwa Graph, Ergebnis oder Drahtgitter); nur die Ansicht ändert sich."),
    ("cycleLodMode", "Switches the 3D preview to the next level of detail in turn; only the view changes.", "Wechselt die 3D-Vorschau reihum zur nächsten Detailstufe; nur die Ansicht ändert sich."),
], [("nodeGraphEdit", "Input"), ("nodeGraphViewport", "Chrome"), ("setCamera", "Chrome")], destructive=["reorganize", "importDocument"])

describe(f"{P}/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs", "create_generation3d_viewer", [
    ("setShowMode", "Sets what the 3D viewer shows of the generated result; only the view changes.", "Legt fest, was der 3D-Betrachter vom erzeugten Ergebnis zeigt; nur die Ansicht ändert sich."),
    ("setLodMode", "Sets the level of detail the 3D viewer draws with; only the view changes.", "Legt die Detailstufe fest, mit der der 3D-Betrachter zeichnet; nur die Ansicht ändert sich."),
    ("toggleSun", "Switches the 3D viewer's sun light on or off; only the view changes.", "Schaltet das Sonnenlicht des 3D-Betrachters ein oder aus; nur die Ansicht ändert sich."),
    ("setSunAzimuth", "Sets the compass direction the 3D viewer's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne des 3D-Betrachters scheint; nur die Ansicht ändert sich."),
    ("setSunElevation", "Sets how high the 3D viewer's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne des 3D-Betrachters über dem Horizont steht; nur die Ansicht ändert sich."),
    ("setSunIntensity", "Sets the brightness of the 3D viewer's sun; only the view changes.", "Legt die Helligkeit der Sonne des 3D-Betrachters fest; nur die Ansicht ändert sich."),
    ("setActiveExample", "Shows one of the bundled generator examples in this viewer instead of the document; the document itself is not changed.", "Zeigt in diesem Betrachter eines der mitgelieferten Generatorbeispiele statt des Dokuments; das Dokument selbst ändert sich nicht."),
    ("exportDocument", "Writes the 3D result shown in the viewer in the chosen format to a downloaded file on the user's machine.", "Schreibt das im Betrachter gezeigte 3D-Ergebnis im gewählten Format in eine heruntergeladene Datei auf dem Rechner des Nutzers."),
], [("setCamera", "Chrome")])

commands = f"{ROOT}/✏️s/🔌️plugins/🌀️procedural/🎮️commands/🦀️.rs"
text = open(commands).read()
old = 'CommandDefinition { in_palette: true, ..CommandDefinition::bounded_catalog(LIST_FLOW_EXTENSIONS, LocalizedLabel::native("List Flow Extensions", "Flow-Erweiterungen auflisten"), "plugin", ActionKind::View) }'
new = old + '.describe(LocalizedLabel::native("Lists every flow extension the procedural plugin can load (id, extension, label and version); nothing is changed.", "Listet alle Flow-Erweiterungen auf, die das Prozedural-Plugin laden kann (Id, Erweiterung, Bezeichnung und Version); nichts wird geändert."))'
assert text.count(old) == 1
open(commands, "w").write(text.replace(old, new))
print("listFlowExtensions described")
