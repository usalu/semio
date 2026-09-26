"""🎬️🔱️ D1 codemod: agent-facing en/de descriptions for sequence and trinity (jack, rewriting)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
from d1_apply import describe

describe("✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_sequence_app", [
    ("reorganize", "Lays out every step of the sequence automatically along the window's flow direction, overwriting their manual positions.", "Ordnet alle Schritte der Sequenz automatisch entlang der Flussrichtung des Fensters an und überschreibt ihre manuellen Positionen."),
    ("setOrientation", "Sets the flow direction (such as left to right or top to bottom) that Reorganize lays the steps out along; only the window's setting changes.", "Legt die Flussrichtung fest (etwa links nach rechts oder oben nach unten), entlang der Neu anordnen die Schritte auslegt; nur die Einstellung des Fensters ändert sich."),
    ("run", "Runs the sequence's compiled path once from its first step; the document is not changed.", "Führt den kompilierten Pfad der Sequenz einmal ab dem ersten Schritt aus; das Dokument ändert sich nicht."),
    ("stop", "Stops a running sequence; the document is not changed.", "Hält eine laufende Sequenz an; das Dokument ändert sich nicht."),
    ("addStep", "Adds a new step of the given kind to the sequence canvas at x, y.", "Fügt der Sequenzfläche an x, y einen neuen Schritt der angegebenen Art hinzu."),
    ("addStepToSlot", "Adds a new step of the given kind into a named slot of an owner step (such as the body of a loop) at x, y.", "Fügt einen neuen Schritt der angegebenen Art an x, y in einen benannten Slot eines Besitzerschritts ein (etwa den Rumpf einer Schleife)."),
    ("removeStep", "Removes one step by id from the sequence together with the flow edges attached to it.", "Entfernt einen Schritt anhand seiner Id samt der angeschlossenen Flusskanten aus der Sequenz."),
    ("deleteSelection", "Removes every currently selected step from the sequence together with their flow edges.", "Entfernt alle aktuell ausgewählten Schritte samt ihrer Flusskanten aus der Sequenz."),
    ("moveStep", "Moves one step to an absolute position x, y on the sequence canvas.", "Verschiebt einen Schritt an die absolute Position x, y der Sequenzfläche."),
    ("connectSteps", "Adds a flow edge from one step to another, so the second runs after the first.", "Fügt eine Flusskante von einem Schritt zu einem anderen hinzu, sodass der zweite nach dem ersten läuft."),
    ("disconnectSteps", "Removes the flow edge between two steps, so they no longer run in that order.", "Entfernt die Flusskante zwischen zwei Schritten, sodass sie nicht mehr in dieser Reihenfolge laufen."),
    ("setStepParams", "Replaces the parameters of one step with the given name-to-value map.", "Ersetzt die Parameter eines Schritts durch die angegebene Zuordnung von Namen zu Werten."),
    ("setStepCollapsed", "Collapses or expands one step on the canvas, hiding or showing its nested steps.", "Klappt einen Schritt auf der Fläche ein oder aus und verbirgt oder zeigt seine verschachtelten Schritte."),
    ("setActiveExample", "Replaces the whole sequence with the bundled demo sequence; any other example id changes nothing.", "Ersetzt die gesamte Sequenz durch die mitgelieferte Demo-Sequenz; jede andere Beispiel-Id ändert nichts."),
], [("nodeGraphEdit", "Input"), ("setViewport", "Chrome")], destructive=["reorganize"])

describe("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_trinity_jack_app", [
    ("setLodMode", "Sets the level of detail the graph window draws the component graph with; only that window's view changes.", "Legt die Detailstufe fest, mit der das Graphfenster den Bauteilgraphen zeichnet; nur die Ansicht dieses Fensters ändert sich."),
    ("formatDocument", "Reformats the Jack query in the query editor window; the graph document is not changed, and a query that does not parse is left as it is.", "Formatiert die Jack-Abfrage im Abfrage-Editorfenster neu; das Graphdokument ändert sich nicht, eine nicht lesbare Abfrage bleibt wie sie ist."),
    ("deleteSelection", "Deletes every selected node of the component graph together with every edge attached to them.", "Löscht alle ausgewählten Knoten des Bauteilgraphen samt aller daran angeschlossenen Kanten."),
    ("patchNodes", "Renames the given nodes, or the selected ones when no ids are given; name is the only field a Jack node allows.", "Benennt die angegebenen Knoten um, ohne Ids die ausgewählten; der Name ist das einzige Feld, das ein Jack-Knoten erlaubt."),
    ("runQuery", "Runs a Jack graph query (or the editor's current one) against the component graph and shows the matches in the given results window; CREATE, SET or DELETE clauses change the graph.", "Führt eine Jack-Graphabfrage (oder die aktuelle des Editors) auf dem Bauteilgraphen aus und zeigt die Treffer im angegebenen Ergebnisfenster; CREATE-, SET- oder DELETE-Klauseln ändern den Graphen."),
    ("loadExampleQuery", "Puts one of the bundled example queries into the query editor, runs it and shows its matches in the given results window.", "Setzt eine der mitgelieferten Beispielabfragen in den Abfrage-Editor, führt sie aus und zeigt ihre Treffer im angegebenen Ergebnisfenster."),
    ("setActiveExample", "Replaces the whole component graph with a bundled fixture (the Nakagin capsule tower or the branch chain), by example id.", "Ersetzt den gesamten Bauteilgraphen durch eine mitgelieferte Fixture (den Nakagin Capsule Tower oder die Astkette), anhand der Beispiel-Id."),
    ("setFixtureJson", "Replaces the whole component graph with one parsed from the given fixture JSON; invalid JSON changes nothing.", "Ersetzt den gesamten Bauteilgraphen durch einen, der aus dem angegebenen Fixture-JSON gelesen wird; ungültiges JSON ändert nichts."),
], [("nodeGraphViewport", "Chrome")], destructive=["runQuery"])

describe("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_rewriting_app", [
    ("setLodMode", "Sets the level of detail the rule's graph window draws with; only that window's view changes.", "Legt die Detailstufe fest, mit der das Graphfenster der Regel zeichnet; nur die Ansicht dieses Fensters ändert sich."),
    ("patchNodes", "Sets the name or kind of the given nodes in the rule's example graph.", "Setzt Name oder Art der angegebenen Knoten im Beispielgraphen der Regel."),
    ("setActiveExample", "Replaces the whole rewriting rule with the bundled demo rule, or with the blank default rule, by example id.", "Ersetzt die gesamte Umschreiberegel durch die mitgelieferte Demo-Regel oder die leere Standardregel, anhand der Beispiel-Id."),
    ("addRuleClause", "Adds a clause of the given kind (where, create, merge, set, delete or parameter) to the rewriting rule; a where clause is only added once.", "Fügt der Umschreiberegel eine Klausel der angegebenen Art hinzu (where, create, merge, set, delete oder parameter); eine where-Klausel wird nur einmal hinzugefügt."),
    ("resetRule", "Resets the rewriting rule to the blank default rule, discarding its pattern, clauses and parameters.", "Setzt die Umschreiberegel auf die leere Standardregel zurück und verwirft Muster, Klauseln und Parameter."),
    ("setParameter", "Sets the bound value of one named rule parameter, parsed as a number, boolean or string according to its declared kind.", "Setzt den gebundenen Wert eines benannten Regelparameters, gelesen als Zahl, Wahrheitswert oder Zeichenkette gemäß seiner deklarierten Art."),
    ("setLhsJson", "Replaces the rule's left-hand side, the graph pattern it matches, with the given JSON.", "Ersetzt die linke Seite der Regel, das Graphmuster, das sie erkennt, durch das angegebene JSON."),
    ("setRhsJson", "Replaces the rule's right-hand side, what it writes for each match, with the given JSON and resets the parameter bindings to their defaults.", "Ersetzt die rechte Seite der Regel, was sie für jeden Treffer schreibt, durch das angegebene JSON und setzt die Parameterbindungen auf ihre Standardwerte zurück."),
    ("reorganize", "Drops every manual node position of the rule graph so it is laid out automatically again.", "Verwirft alle manuellen Knotenpositionen des Regelgraphen, sodass er wieder automatisch angeordnet wird."),
], [("nodeGraphEdit", "Input"), ("nodeGraphViewport", "Chrome")], destructive=["setLhsJson", "setRhsJson", "reorganize"])
