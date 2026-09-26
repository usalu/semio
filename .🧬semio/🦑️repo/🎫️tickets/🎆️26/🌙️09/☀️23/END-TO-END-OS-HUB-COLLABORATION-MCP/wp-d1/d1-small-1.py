"""🎞️🕸️💡️➗️ D1 codemod: agent-facing en/de descriptions for animate, dag, reasoning and mathematical."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
from d1_apply import describe

describe("✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_animate_presentation_app", [
    ("seedGrid", "Replaces every tile of the deck with a fresh grid of the given rows and columns cut from the source image; the previous tiles are discarded.", "Ersetzt alle Kacheln des Decks durch ein neues Raster mit den angegebenen Zeilen und Spalten aus dem Quellbild; die bisherigen Kacheln werden verworfen."),
    ("addTile", "Adds one new tile cropping the given region of the source image (a small square near the top left when none is given) and selects it.", "Fügt eine neue Kachel hinzu, die den angegebenen Bereich des Quellbilds ausschneidet (ohne Angabe ein kleines Quadrat oben links), und wählt sie aus."),
    ("deleteTile", "Deletes one tile by id from the deck; its crop and name are gone unless the edit is undone.", "Löscht eine Kachel anhand ihrer Id aus dem Deck; Zuschnitt und Name sind fort, sofern die Änderung nicht rückgängig gemacht wird."),
    ("deleteSelection", "Deletes every currently selected tile from the deck.", "Löscht alle aktuell ausgewählten Kacheln aus dem Deck."),
    ("renameTiles", "Gives every tile with the given ids the same new name; an empty name changes nothing.", "Gibt allen Kacheln mit den angegebenen Ids denselben neuen Namen; ein leerer Name ändert nichts."),
    ("patchTileCrops", "Sets one crop coordinate (x, y, width or height, as a fraction of the source image) on every tile with the given ids.", "Setzt eine Zuschnittkoordinate (x, y, Breite oder Höhe, als Anteil des Quellbilds) auf allen Kacheln mit den angegebenen Ids."),
    ("setSource", "Sets the source image the tiles are cut from; choosing a different image discards every existing tile.", "Legt das Quellbild fest, aus dem die Kacheln geschnitten werden; ein anderes Bild verwirft alle vorhandenen Kacheln."),
    ("setFrame", "Moves and resizes the frame the source image is shown in within the presentation (x, y, width, height).", "Verschiebt und skaliert den Rahmen, in dem das Quellbild in der Präsentation erscheint (x, y, Breite, Höhe)."),
    ("setActiveExample", "Replaces the whole presentation with the bundled demo deck; any other example id changes nothing.", "Ersetzt die gesamte Präsentation durch das mitgelieferte Demo-Deck; jede andere Beispiel-Id ändert nichts."),
    ("clearTiles", "Removes every tile from the deck and keeps only the source image.", "Entfernt alle Kacheln aus dem Deck und behält nur das Quellbild."),
    ("copyPrompt", "Writes a tile-morph prompt describing the source image and every tile to a downloaded tile-morph-prompt.md file on the user's machine.", "Schreibt einen Tile-Morph-Prompt, der das Quellbild und jede Kachel beschreibt, in eine heruntergeladene Datei tile-morph-prompt.md auf dem Rechner des Nutzers."),
    ("exportVideoFromDeck", "Renders a presentation scene (JSON) to video assets in the given output directory and downloads their list as animate-video-export.ops.", "Rendert eine Präsentationsszene (JSON) als Videodateien in das angegebene Ausgabeverzeichnis und lädt deren Liste als animate-video-export.ops herunter."),
    ("resetGrid", "Replaces every tile of the deck with the default 3 by 5 grid cut from the source image; the previous tiles are discarded.", "Ersetzt alle Kacheln des Decks durch das Standardraster von 3 mal 5 aus dem Quellbild; die bisherigen Kacheln werden verworfen."),
], [("noMutation", "Chrome")], destructive=["seedGrid", "setSource", "copyPrompt"])

describe("✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_dag_app", [
    ("addNode", "Adds a new node of the given kind (such as slider, select, screen, note or preview) to the graph at x, y, or at a default spot.", "Fügt dem Graphen einen neuen Knoten der angegebenen Art (etwa Schieberegler, Auswahl, Bildschirm, Notiz oder Vorschau) an x, y oder an einer Standardstelle hinzu."),
    ("removeNode", "Removes one node by id from the graph together with every edge attached to it.", "Entfernt einen Knoten anhand seiner Id samt aller angeschlossenen Kanten aus dem Graphen."),
    ("deleteSelection", "Removes every currently selected node from the graph together with the edges attached to them.", "Entfernt alle aktuell ausgewählten Knoten samt ihrer angeschlossenen Kanten aus dem Graphen."),
    ("connectMediaPorts", "Connects an output port of one node to an input port of another with a new edge; an incompatible or duplicate connection changes nothing.", "Verbindet einen Ausgangsport eines Knotens mit einem Eingangsport eines anderen durch eine neue Kante; eine unverträgliche oder doppelte Verbindung ändert nichts."),
    ("disconnect", "Removes one edge by id, cutting the connection between its two ports.", "Entfernt eine Kante anhand ihrer Id und trennt damit die Verbindung ihrer beiden Ports."),
    ("moveMediaNode", "Moves one node to the canvas position x, y; consecutive moves of the same node merge into one edit.", "Verschiebt einen Knoten an die Position x, y der Fläche; aufeinanderfolgende Verschiebungen desselben Knotens werden zu einer Änderung zusammengefasst."),
    ("renameDagNode", "Renames a node's id from the old id to a new one and updates every edge that referenced it; a taken or empty id changes nothing.", "Benennt die Id eines Knotens von der alten in eine neue um und aktualisiert alle Kanten, die darauf verweisen; eine vergebene oder leere Id ändert nichts."),
    ("patchDagNodes", "Sets one field (name, or a slider's value, min or max) on every node with the given ids.", "Setzt ein Feld (Name oder Wert, Minimum oder Maximum eines Schiebereglers) auf allen Knoten mit den angegebenen Ids."),
    ("setActiveExample", "Replaces the whole graph with the bundled demo graph, or with an empty graph for any other example id.", "Ersetzt den gesamten Graphen durch den mitgelieferten Demo-Graphen, bei jeder anderen Beispiel-Id durch einen leeren Graphen."),
], [("nodeGraphEdit", "Input"), ("nodeGraphViewport", "Chrome")], destructive=["setActiveExample"])

describe("✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_wires_app", [
    ("setActiveExample", "Replaces the whole reasoning board with the bundled metabolism example, or with an empty board for any other example id.", "Ersetzt das gesamte Denkbrett durch das mitgelieferte Stoffwechsel-Beispiel, bei jeder anderen Beispiel-Id durch ein leeres Brett."),
    ("addNode", "Adds a new node of the given kind (identity by default) to the reasoning board and selects it.", "Fügt dem Denkbrett einen neuen Knoten der angegebenen Art (standardmäßig Identität) hinzu und wählt ihn aus."),
    ("addRelationship", "Adds a relationship edge of the given kind (owns by default) from node-1 to node-2 of the board and selects it.", "Fügt eine Beziehungskante der angegebenen Art (standardmäßig besitzt) von node-1 zu node-2 des Bretts hinzu und wählt sie aus."),
    ("deleteSelection", "Deletes every currently selected node and relationship from the reasoning board.", "Löscht alle aktuell ausgewählten Knoten und Beziehungen vom Denkbrett."),
], [("nodeGraphViewport", "Chrome")])

describe("✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_equation_app", [
    ("setDocument", "Replaces the equation's whole graph and point geometry with the supplied ones; parts that differ are overwritten.", "Ersetzt den gesamten Graphen und die Punktgeometrie der Gleichung durch die übergebenen; abweichende Teile werden überschrieben."),
    ("setAlgorithm", "Chooses the graph algorithm the equation evaluates: topological order, connected components, strongly connected components or breadth-first distances.", "Wählt den Graphalgorithmus, den die Gleichung auswertet: topologische Ordnung, Zusammenhangskomponenten, starke Zusammenhangskomponenten oder Breitensuche-Distanzen."),
    ("setDirected", "Sets whether the equation's graph is treated as directed or undirected, which changes every algorithm result.", "Legt fest, ob der Graph der Gleichung gerichtet oder ungerichtet behandelt wird, was jedes Algorithmusergebnis ändert."),
    ("nodeGraphEdit", "Applies a JSON list of graph edits (addNode at x, y; move; connect; deleteSelection) to the equation's graph in one step.", "Wendet eine JSON-Liste von Graphänderungen (addNode an x, y; move; connect; deleteSelection) in einem Schritt auf den Graphen der Gleichung an."),
    ("setPoints", "Replaces the point set of the equation's geometry with the supplied points.", "Ersetzt die Punktmenge der Geometrie der Gleichung durch die übergebenen Punkte."),
    ("setActiveExample", "Replaces the whole equation document with the bundled demo example; any other example id changes nothing.", "Ersetzt das gesamte Gleichungsdokument durch das mitgelieferte Demo-Beispiel; jede andere Beispiel-Id ändert nichts."),
], [("nodeGraphViewport", "Chrome")], destructive=["setPoints"])
