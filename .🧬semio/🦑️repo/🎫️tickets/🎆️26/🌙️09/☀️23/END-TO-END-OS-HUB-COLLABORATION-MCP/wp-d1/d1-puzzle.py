"""🧩️ D1 codemod: agent-facing en/de descriptions for the puzzle2d, puzzle3d and puzzle5d aggregation editors."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
from d1_apply import describe

P = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts"

def common(dim, thing, thing_de, things, things_de):
    return [
        ("setActiveExample", f"Replaces the whole {dim} puzzle with one of the plugin's bundled examples, by example id.", f"Ersetzt das gesamte {dim}-Puzzle durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."),
        ("deleteSelection", f"Deletes every selected {thing} from the {dim} puzzle together with its connections.", f"Löscht alle ausgewählten {things_de} samt ihrer Verbindungen aus dem {dim}-Puzzle."),
        ("duplicateSelection", f"Adds a copy of every selected {thing} next to the original; nothing happens without a selection.", f"Fügt neben jedem ausgewählten Element ({thing_de}) eine Kopie hinzu; ohne Auswahl geschieht nichts."),
        ("focusSelection", f"Frames the camera on the selected {things}; only the view changes.", f"Richtet die Kamera auf die ausgewählten {things_de} aus; nur die Ansicht ändert sich."),
        ("translateSelection", f"Moves the selected {things} by dx and dy.", f"Verschiebt die ausgewählten {things_de} um dx und dy."),
        ("rotateSelection", f"Rotates the selected {things} by the given angle.", f"Dreht die ausgewählten {things_de} um den angegebenen Winkel."),
        ("scaleSelection", f"Scales the selected {things} by the given factor.", f"Skaliert die ausgewählten {things_de} um den angegebenen Faktor."),
        ("exportFixture", f"Writes the whole {dim} puzzle as JSON to a downloaded file named after the active example on the user's machine.", f"Schreibt das gesamte {dim}-Puzzle als JSON in eine heruntergeladene, nach dem aktiven Beispiel benannte Datei auf dem Rechner des Nutzers."),
        ("openImportFixture", f"Opens the host's file picker for a {dim} puzzle JSON file; the chosen file then replaces the whole puzzle.", f"Öffnet die Dateiauswahl des Hosts für eine {dim}-Puzzle-JSON-Datei; die gewählte Datei ersetzt dann das gesamte Puzzle."),
        ("importFixture", f"Replaces the whole {dim} puzzle with one read from imported JSON, delivered in chunks; the previous puzzle is discarded.", f"Ersetzt das gesamte {dim}-Puzzle durch eines aus importiertem JSON, das in Teilen geliefert wird; das bisherige Puzzle wird verworfen."),
        ("setSelectionFlag", f"Sets one flag (such as hidden or locked) on the given or selected {things}.", f"Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen oder ausgewählten {things_de}."),
        ("acceptSuggestion", "Places the suggested piece chosen from the suggestion list (by index, or the highlighted one) at its connection point.", "Setzt das aus der Vorschlagsliste gewählte Teil (per Index oder das hervorgehobene) an seinem Anschlusspunkt."),
    ]

def world(dim):
    return [
        ("selectSameKindSelection", "Extends the selection to every piece of the same kind as the selected one.", "Erweitert die Auswahl auf alle Teile derselben Art wie das ausgewählte."),
        ("toggleSun", "Switches the 3D view's sun light on or off; only the view changes.", "Schaltet das Sonnenlicht der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."),
        ("setSunAzimuth", "Sets the compass direction the 3D view's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne der 3D-Ansicht scheint; nur die Ansicht ändert sich."),
        ("setSunElevation", "Sets how high the 3D view's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne der 3D-Ansicht über dem Horizont steht; nur die Ansicht ändert sich."),
        ("setSunIntensity", "Sets the brightness of the 3D view's sun; only the view changes.", "Legt die Helligkeit der Sonne der 3D-Ansicht fest; nur die Ansicht ändert sich."),
        ("setLodAutomatic", "Turns automatic level of detail in the 3D view on or off; only the view changes.", "Schaltet die automatische Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."),
        ("setLodDepthVariable", "Turns depth-dependent level of detail in the 3D view on or off; only the view changes.", "Schaltet die tiefenabhängige Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."),
        ("setLodManual", "Sets a fixed level of detail for the 3D view; only the view changes.", "Legt eine feste Detailstufe für die 3D-Ansicht fest; nur die Ansicht ändert sich."),
        ("setGridVisible", "Shows or hides the placement grid in the 3D view; only the view changes.", "Blendet das Platzierungsraster in der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."),
        ("setGridSnapEnabled", "Turns snapping to the placement grid on or off for moves in the 3D view.", "Schaltet das Fangen am Platzierungsraster für Verschiebungen in der 3D-Ansicht ein oder aus."),
        ("setGridSpacing", "Sets the spacing of the placement grid.", "Legt den Abstand des Platzierungsrasters fest."),
        ("setProximityRadius", "Sets how close two pieces must come for a move or Connect Nearby to join their connection points.", "Legt fest, wie nahe sich zwei Teile kommen müssen, damit ein Verschieben oder In der Nähe verbinden ihre Anschlusspunkte verbindet."),
        ("setChunkSize", "Sets the size of the spatial chunks the 3D view loads and draws pieces in; only the view changes.", "Legt die Größe der räumlichen Blöcke fest, in denen die 3D-Ansicht Teile lädt und zeichnet; nur die Ansicht ändert sich."),
        ("setSelectableKind", "Sets whether pieces, connection points or attractions can be picked in the 3D view.", "Legt fest, ob Teile, Anschlusspunkte oder Anziehungen in der 3D-Ansicht gewählt werden können."),
        ("setTransformGumballFlag", "Switches one option of the transform gumball (such as translate, rotate or scale handles) on or off.", "Schaltet eine Option des Transformationsgriffs (etwa Verschiebe-, Dreh- oder Skaliergriffe) ein oder aus."),
        ("setVoxelDims", "Sets one dimension (width, depth or height) of the voxel volume the fill tool packs pieces into.", "Legt eine Abmessung (Breite, Tiefe oder Höhe) des Voxelvolumens fest, in das das Füllwerkzeug Teile packt."),
        ("relocateTargetVolume", "Moves one target volume to a new position.", "Verschiebt ein Zielvolumen an eine neue Position."),
        ("setFillCount", "Sets how many pieces the fill tool places into the target volume.", "Legt fest, wie viele Teile das Füllwerkzeug in das Zielvolumen setzt."),
        ("setBrushPlacementContactTolerance", "Sets how much overlap (0 to 1) the placement brush tolerates between a suggested piece and existing ones.", "Legt fest, wie viel Überlappung (0 bis 1) der Platzierungspinsel zwischen einem vorgeschlagenen und vorhandenen Teilen zulässt."),
        ("cycleBrushCandidate", "Switches the placement brush to the next suggested piece.", "Schaltet den Platzierungspinsel zum nächsten vorgeschlagenen Teil."),
        ("cycleBrushCandidateBack", "Switches the placement brush to the previous suggested piece.", "Schaltet den Platzierungspinsel zum vorherigen vorgeschlagenen Teil."),
        ("openVortexSuggestions", "Opens the list of pieces that fit at one connection point.", "Öffnet die Liste der Teile, die an einen Anschlusspunkt passen."),
        ("closeVortexSuggestions", "Closes the list of suggested pieces.", "Schließt die Liste der vorgeschlagenen Teile."),
        ("addTargetVolume", "Adds a target volume, a box region the fill tool packs pieces into.", "Fügt ein Zielvolumen hinzu, einen Quaderbereich, in den das Füllwerkzeug Teile packt."),
        ("deleteTargetVolume", "Deletes one target volume by id.", "Löscht ein Zielvolumen anhand seiner Id."),
        ("setTargetVolumeFlag", "Sets one flag (such as hidden or locked) on the given target volumes.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen Zielvolumen."),
    ]

AUDIENCES_3D = [("setCamera", "Chrome"), ("setProjection", "Chrome"), ("setProjectionParam", "Chrome"), ("engagementRepeatLast", "Input"), ("engagementControlSelect", "Input"), ("targetBrushSuggestions", "Input"), ("registerBrushMesh", "Input")]

describe(f"{P}/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_puzzle2d_app", common("2D", "node", "Knoten", "nodes", "Knoten") + [
    ("addNode", "Adds a node of the given kind (a 2D block kind) to the puzzle at x, y.", "Fügt dem Puzzle an x, y einen Knoten der angegebenen Art (einer 2D-Blockart) hinzu."),
    ("forceLayout", "Lays out every node of the 2D puzzle with a force-directed layout, overwriting their positions.", "Ordnet alle Knoten des 2D-Puzzles mit einem kraftbasierten Layout an und überschreibt ihre Positionen."),
    ("createEdge", "Connects two handles of two nodes (source and target) with an edge.", "Verbindet zwei Griffe zweier Knoten (Quelle und Ziel) mit einer Kante."),
    ("deleteEdge", "Removes one edge by id, disconnecting its two handles.", "Entfernt eine Kante anhand ihrer Id und trennt ihre zwei Griffe."),
    ("proximityConnect", "Connects every pair of compatible handles that lie within the proximity radius of each other.", "Verbindet jedes Paar verträglicher Griffe, die innerhalb des Näheradius beieinander liegen."),
    ("selectSameKind", "Extends the selection to every node of the same kind as the selected one.", "Erweitert die Auswahl auf alle Knoten derselben Art wie der ausgewählte."),
    ("patchInspectorNodes", "Sets one field on the given nodes, or on handles addressed by id (such as a handle's angle or radius); an empty id list addresses every node.", "Setzt ein Feld auf den angegebenen Knoten oder per Id adressierten Griffen (etwa Winkel oder Radius eines Griffs); eine leere Id-Liste adressiert alle Knoten."),
    ("redrawHandles", "Re-aims every connected handle at the edge it carries, updating the handle angles.", "Richtet jeden verbundenen Griff neu auf seine Kante aus und aktualisiert die Griffwinkel."),
    ("reorganize", "Lays out every node of the 2D puzzle automatically, overwriting their manual positions.", "Ordnet alle Knoten des 2D-Puzzles automatisch an und überschreibt ihre manuellen Positionen."),
    ("addTargetRegion", "Adds a rectangular target region at the given origin and size (the area brush size by default).", "Fügt an Ursprung und Größe (standardmäßig der Flächenpinselgröße) einen rechteckigen Zielbereich hinzu."),
    ("deleteTargetRegion", "Deletes one target region by id.", "Löscht einen Zielbereich anhand seiner Id."),
    ("setTargetRegionFlag", "Sets one flag (such as hidden or locked) on the given target regions.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen Zielbereichen."),
    ("relocateTargetRegion", "Moves or resizes one target region to the given rectangle.", "Verschiebt oder skaliert einen Zielbereich auf das angegebene Rechteck."),
    ("openAddNodeDialog", "Opens the Add Node dialog to pick a node kind to place.", "Öffnet den Dialog Knoten hinzufügen, um eine zu setzende Knotenart zu wählen."),
], [("applyBoardEvents", "Input")], destructive=["importFixture", "forceLayout", "reorganize"])

describe(f"{P}/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_puzzle3d_app", common("3D", "object", "Objekte", "objects", "Objekte") + world("3D") + [
    ("openAddObjectDialog", "Opens the Add Object dialog to pick an object kind to place.", "Öffnet den Dialog Objekt hinzufügen, um eine zu setzende Objektart zu wählen."),
    ("addObjectKind", "Adds an object of the given kind (a 3D block kind) to the puzzle.", "Fügt dem Puzzle ein Objekt der angegebenen Art (einer 3D-Blockart) hinzu."),
    ("worldRelocate", "Moves one object to a new position and connects it to nearby compatible connection points.", "Verschiebt ein Objekt an eine neue Position und verbindet es mit verträglichen Anschlusspunkten in der Nähe."),
    ("patchInspector", "Sets one inspector field on the given or selected objects or connection points.", "Setzt ein Inspektorfeld auf den angegebenen oder ausgewählten Objekten oder Anschlusspunkten."),
    ("createAttraction", "Creates an attraction that pulls two compatible connection points (vortices) of different objects together.", "Erstellt eine Anziehung, die zwei verträgliche Anschlusspunkte (Wirbel) verschiedener Objekte zusammenzieht."),
    ("deleteAttraction", "Deletes one attraction by id, releasing its two connection points.", "Löscht eine Anziehung anhand ihrer Id und gibt ihre zwei Anschlusspunkte frei."),
    ("addBrushObject", "Places an object with the placement brush at the given contact point, as the brush suggests.", "Setzt mit dem Platzierungspinsel ein Objekt am angegebenen Kontaktpunkt, wie der Pinsel es vorschlägt."),
    ("setVortexShow", "Sets whether connection points are shown always or only on selected objects; only the view changes.", "Legt fest, ob Anschlusspunkte immer oder nur an ausgewählten Objekten gezeigt werden; nur die Ansicht ändert sich."),
    ("setVortexDirection", "Sets how connection point directions are drawn; only the view changes.", "Legt fest, wie die Richtungen der Anschlusspunkte gezeichnet werden; nur die Ansicht ändert sich."),
    ("setObjectKindWeight", "Sets how often the fill tool and brush pick one object kind relative to the others.", "Legt fest, wie oft Füllwerkzeug und Pinsel eine Objektart im Verhältnis zu den anderen wählen."),
    ("setVortexKindWeight", "Sets how strongly one connection point kind is preferred when suggesting placements.", "Legt fest, wie stark eine Anschlusspunktart bei Platzierungsvorschlägen bevorzugt wird."),
], AUDIENCES_3D + [("transformBegin", "Input"), ("transformEnd", "Input")], destructive=["importFixture"])

describe(f"{P}/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_puzzle5d_app", common("5D", "part", "Teile", "parts", "Teile") + world("5D") + [
    ("addNode", "Adds a part of the given kind (a 5D block kind) to the puzzle at x, y.", "Fügt dem Puzzle an x, y ein Teil der angegebenen Art (einer 5D-Blockart) hinzu."),
    ("proximityConnect", "Fastens every pair of compatible grips that lie within the proximity radius of each other.", "Verbindet jedes Paar verträglicher Griffe, die innerhalb des Näheradius beieinander liegen, mit Verbindern."),
    ("worldRelocate", "Moves one part to a new position and fastens it to nearby compatible grips.", "Verschiebt ein Teil an eine neue Position und verbindet es mit verträglichen Griffen in der Nähe."),
    ("openAddPartDialog", "Opens the Add Part dialog to pick a part kind to place.", "Öffnet den Dialog Teil hinzufügen, um eine zu setzende Teileart zu wählen."),
    ("addPartKind", "Adds a part of the given kind to the puzzle.", "Fügt dem Puzzle ein Teil der angegebenen Art hinzu."),
    ("addBrushPart", "Places a part of the given kind with the placement brush where the brush suggests.", "Setzt mit dem Platzierungspinsel ein Teil der angegebenen Art dort, wo der Pinsel es vorschlägt."),
    ("patchPart", "Sets one field on the given or selected parts.", "Setzt ein Feld auf den angegebenen oder ausgewählten Teilen."),
    ("patchGrip", "Sets one field on the given or selected grips.", "Setzt ein Feld auf den angegebenen oder ausgewählten Griffen."),
    ("patchFastener", "Sets one field on the given or selected fasteners.", "Setzt ein Feld auf den angegebenen oder ausgewählten Verbindern."),
    ("createFastener", "Creates a fastener joining two compatible grips of different parts.", "Erstellt einen Verbinder, der zwei verträgliche Griffe verschiedener Teile verbindet."),
    ("deleteFastener", "Deletes one fastener by id, releasing its two grips.", "Löscht einen Verbinder anhand seiner Id und gibt seine zwei Griffe frei."),
    ("retargetFastener", "Moves one end of a fastener to another compatible grip.", "Hängt ein Ende eines Verbinders an einen anderen verträglichen Griff um."),
    ("editFastener", "Sets the gap, shift or rise of one fastener, absolutely or by a delta.", "Setzt Abstand, Versatz oder Anhebung eines Verbinders, absolut oder um eine Differenz."),
    ("setPartKindWeight", "Sets how often the fill tool and brush pick one part kind relative to the others.", "Legt fest, wie oft Füllwerkzeug und Pinsel eine Teileart im Verhältnis zu den anderen wählen."),
    ("setGripKindWeight", "Sets how strongly one grip kind is preferred when suggesting placements.", "Legt fest, wie stark eine Griffart bei Platzierungsvorschlägen bevorzugt wird."),
    ("setLodMode", "Sets the level-of-detail mode of the views; only the view changes.", "Legt den Detailstufenmodus der Ansichten fest; nur die Ansicht ändert sich."),
    ("setSuggestionOffset", "Sets how far suggested parts are offset from the grip they would attach to.", "Legt fest, wie weit vorgeschlagene Teile vom Griff, an den sie ansetzen würden, versetzt werden."),
    ("setGridFactor", "Sets the spacing factor of the 2D view's snap grid; only the view setting changes.", "Legt den Abstandsfaktor des Fangrasters der 2D-Ansicht fest; nur die Ansichtseinstellung ändert sich."),
    ("setGripShow", "Sets whether grip markers are shown always or only on selected parts; only the view changes.", "Legt fest, ob Griffmarken immer oder nur an ausgewählten Teilen gezeigt werden; nur die Ansicht ändert sich."),
    ("setGripDirection", "Sets how grip directions are drawn; only the view changes.", "Legt fest, wie Griffrichtungen gezeichnet werden; nur die Ansicht ändert sich."),
], AUDIENCES_3D + [("setCamera2d", "Chrome"), ("setCamera3d", "Chrome"), ("applyBoardEvents", "Input")], destructive=["importFixture"])
