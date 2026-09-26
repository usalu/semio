"""📜️📖️🪵️✒️ D1 codemod: agent-facing en/de descriptions for imperative, playbook (+ procedural module), sourcing and writer."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
from d1_apply import describe

describe("✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_imperative_app", [
    ("addStep", "Appends a new step of the given kind (for example a control, math, text or effect step) to the end of the procedure.", "Hängt einen neuen Schritt der angegebenen Art (etwa einen Steuerungs-, Mathe-, Text- oder Effektschritt) an das Ende der Prozedur an."),
    ("addStepAt", "Appends a new step of the given kind inside a slot of a control step (such as the then or else branch of an if), or to the procedure's end without owner and slot.", "Hängt einen neuen Schritt der angegebenen Art in einen Slot eines Steuerungsschritts (etwa den Dann- oder Sonst-Zweig eines Wenn) an, ohne Besitzer und Slot an das Ende der Prozedur."),
    ("removeStep", "Removes one step by id from the top level of the procedure, including every step nested inside it.", "Entfernt einen Schritt anhand seiner Id aus der obersten Ebene der Prozedur, samt aller darin verschachtelten Schritte."),
    ("removeStepAt", "Removes one step by id from a control step's slot, or from the top level, including every step nested inside it.", "Entfernt einen Schritt anhand seiner Id aus dem Slot eines Steuerungsschritts oder der obersten Ebene, samt aller darin verschachtelten Schritte."),
    ("moveStep", "Moves one top-level step to a new position (index) in the procedure.", "Verschiebt einen Schritt der obersten Ebene an eine neue Position (Index) in der Prozedur."),
    ("moveStepAt", "Moves one step to a new position (index) within its control step's slot or the top level.", "Verschiebt einen Schritt an eine neue Position (Index) innerhalb des Slots seines Steuerungsschritts oder der obersten Ebene."),
    ("setStepParams", "Replaces the parameters of one top-level step with the given name-to-value map.", "Ersetzt die Parameter eines Schritts der obersten Ebene durch die angegebene Zuordnung von Namen zu Werten."),
    ("setStepParamsAt", "Replaces the parameters of one step inside a control step's slot, or at the top level, with the given name-to-value map.", "Ersetzt die Parameter eines Schritts im Slot eines Steuerungsschritts oder der obersten Ebene durch die angegebene Zuordnung von Namen zu Werten."),
    ("run", "Runs the procedure from its first step and shows the resulting variable scope in the output view; the procedure itself is not changed.", "Führt die Prozedur ab dem ersten Schritt aus und zeigt den entstandenen Variablenbereich in der Ausgabeansicht; die Prozedur selbst ändert sich nicht."),
    ("setActiveExample", "Replaces the whole procedure with the bundled demo program, or with an empty procedure for any other example id.", "Ersetzt die gesamte Prozedur durch das mitgelieferte Demoprogramm, bei jeder anderen Beispiel-Id durch eine leere Prozedur."),
], destructive=["setActiveExample"])

describe("✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_playbook_play_app", [
    ("addStep", "Appends a new, empty step to the end of the playbook.", "Hängt dem Playbook am Ende einen neuen, leeren Schritt an."),
    ("removeStep", "Removes one step by id from the playbook together with every block it contains.", "Entfernt einen Schritt anhand seiner Id samt aller enthaltenen Bausteine aus dem Playbook."),
    ("moveStep", "Moves one step to a new position (index) in the playbook.", "Verschiebt einen Schritt an eine neue Position (Index) im Playbook."),
    ("addBlock", "Adds a new block of the given kind (such as a procedural building component) to the named step, or to the default step when none is named.", "Fügt einem Schritt einen neuen Baustein der angegebenen Art (etwa ein prozedurales Bauteil) hinzu, ohne Angabe dem Standardschritt."),
    ("removeBlock", "Removes one block by id from the given step of the playbook.", "Entfernt einen Baustein anhand seiner Id aus dem angegebenen Schritt des Playbooks."),
    ("moveBlock", "Moves one block from its step to a position (index) in another step or the same one.", "Verschiebt einen Baustein aus seinem Schritt an eine Position (Index) in einem anderen oder demselben Schritt."),
    ("updatePlaybook", "Sets the playbook's title; an empty value clears it, and consecutive edits merge into one undo step.", "Legt den Titel des Playbooks fest; ein leerer Wert entfernt ihn, aufeinanderfolgende Änderungen werden zu einem Rückgängig-Schritt zusammengefasst."),
    ("setActiveExample", "Replaces the whole playbook with the bundled demo playbook, or with an empty playbook for any other example id.", "Ersetzt das gesamte Playbook durch das mitgelieferte Demo-Playbook, bei jeder anderen Beispiel-Id durch ein leeres Playbook."),
], destructive=["setActiveExample"])

describe("✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs", "create_module_app", [
    ("exportSolidGeometry", "Exports the block's preview geometry as STEP, OBJ, STL or GLB and stores the result in the block's parameters for the shell to read back; nothing is written to disk.", "Exportiert die Vorschaugeometrie des Bausteins als STEP, OBJ, STL oder GLB und legt das Ergebnis in den Parametern des Bausteins ab, damit die Shell es ausliest; auf die Festplatte wird nichts geschrieben."),
    ("importSolidGeometry", "Imports solid geometry in the given format (STEP or OBJ text, STL or GLB as base64) into the block and stores the resulting geometry handles in its parameters.", "Importiert Volumengeometrie im angegebenen Format (STEP- oder OBJ-Text, STL oder GLB als Base64) in den Baustein und legt die entstandenen Geometrie-Handles in seinen Parametern ab."),
], awaited=True, anchor=".action_interactive_job(ACTION_EXPORT_SOLID")

describe("✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_sourcing_curation_app", [
    ("setActiveExample", "Replaces the whole curation with one of the bundled example stocks or the empty curation, by example id.", "Ersetzt die gesamte Kuratierung durch einen der mitgelieferten Beispielbestände oder die leere Kuratierung, anhand der Beispiel-Id."),
    ("stockFromCatalogue", "Adds every object kind the installed sourcing modules (such as beams, slabs and windows) offer to the stock, keeping the existing stock and curated counts.", "Fügt dem Bestand alle Objektarten hinzu, die die installierten Beschaffungsmodule (etwa Träger, Decken und Fenster) anbieten; vorhandener Bestand und kuratierte Anzahlen bleiben erhalten."),
    ("setDocument", "Replaces the whole curation with one decoded from the given JSON document; oversized or schema-mismatched JSON is refused.", "Ersetzt die gesamte Kuratierung durch eine aus dem angegebenen JSON-Dokument; zu großes oder schemafremdes JSON wird abgelehnt."),
    ("curationAdd", "Adds one more of the given stock object to the curated selection, at most up to its available quantity.", "Nimmt ein weiteres Exemplar des angegebenen Bestandsobjekts in die Kuratierung auf, höchstens bis zur verfügbaren Menge."),
    ("curationSetCount", "Sets how many of one stock object are curated, by a relative delta or an absolute value clamped to its availability; zero removes it from the curation.", "Legt fest, wie viele Exemplare eines Bestandsobjekts kuratiert sind, per relativer Änderung oder absolutem Wert bis zur Verfügbarkeit; null entfernt es aus der Kuratierung."),
    ("curationRemove", "Removes one stock object from the curated selection entirely; the stock itself is unchanged.", "Entfernt ein Bestandsobjekt vollständig aus der Kuratierung; der Bestand selbst bleibt unverändert."),
    ("setGridInstanceDisplay", "Chooses how object instances are shown in the curation grid window; only that window's view changes.", "Wählt, wie Objektinstanzen im Kuratierungs-Rasterfenster dargestellt werden; nur die Ansicht dieses Fensters ändert sich."),
])

describe("✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_writer_app", [
    ("formatDocument", "Reformats the whole text with the formatter of the document's language; text that is already formatted writes nothing.", "Formatiert den gesamten Text mit dem Formatierer der Dokumentsprache neu; bereits formatierter Text schreibt nichts."),
    ("lintDocument", "Checks the text with the linter of the document's language and shows the diagnostics in the editor window; the text is not changed.", "Prüft den Text mit dem Linter der Dokumentsprache und zeigt die Befunde im Editorfenster an; der Text ändert sich nicht."),
    ("setActiveExample", "Replaces the whole document with a bundled example (the Jack demo or the DAG Jack example), or with an empty document for any other id.", "Ersetzt das gesamte Dokument durch ein mitgeliefertes Beispiel (die Jack-Demo oder das DAG-Jack-Beispiel), bei jeder anderen Id durch ein leeres Dokument."),
    ("setText", "Replaces the document's entire text with the given text; the previous text is gone unless the edit is undone.", "Ersetzt den gesamten Text des Dokuments durch den angegebenen Text; der bisherige Text ist fort, sofern die Änderung nicht rückgängig gemacht wird."),
    ("commitRename", "With the caret on a Jack variable, renames every occurrence of it to the given text; otherwise replaces the editor's selected range with that text.", "Steht die Einfügemarke auf einer Jack-Variablen, werden alle ihre Vorkommen in den angegebenen Text umbenannt; sonst ersetzt der Text den ausgewählten Bereich des Editors."),
    ("setSnapshot", "Replaces the whole writer document, its text, language and metadata, with the supplied document; nothing of the previous one is kept.", "Ersetzt das gesamte Writer-Dokument mit Text, Sprache und Metadaten durch das übergebene; vom bisherigen Dokument bleibt nichts erhalten."),
    ("openDocument", "Opens the given text under a URI as the writer document, detecting its language from the content or extension; the current document is replaced.", "Öffnet den angegebenen Text unter einer URI als Writer-Dokument und erkennt die Sprache aus Inhalt oder Dateiendung; das aktuelle Dokument wird ersetzt."),
    ("setSnapshotJson", "Replaces the whole writer document with one parsed from the given document JSON; invalid JSON changes nothing.", "Ersetzt das gesamte Writer-Dokument durch eines, das aus dem angegebenen Dokument-JSON gelesen wird; ungültiges JSON ändert nichts."),
    ("setFixtureJson", "Loads a test fixture given as JSON as the whole writer document, replacing the current one; invalid JSON changes nothing.", "Lädt eine als JSON übergebene Test-Fixture als gesamtes Writer-Dokument und ersetzt das aktuelle; ungültiges JSON ändert nichts."),
], [("textEdit", "Input")], destructive=["setText", "openDocument"])
