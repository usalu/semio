"""🧱️ D1 codemod: agent-facing en/de descriptions for the block2d, block3d and block5d editors."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
from d1_apply import describe

B = "✏️s/🔌️plugins/🧱️block/🗿️artifacts"
describe(f"{B}/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_block2d_app", [
    ("patchNodeKind", "Sets one field of the 2D block's node kind: its name, label, variant, description or icon.", "Setzt ein Feld der Knotenart des 2D-Blocks: Name, Bezeichnung, Variante, Beschreibung oder Symbol."),
    ("addHandleKind", "Adds a new handle kind (a named, coloured connector type with a default wire kind) to the 2D block.", "Fügt dem 2D-Block eine neue Griffart hinzu (einen benannten, farbigen Verbindertyp mit Standard-Leitungsart)."),
    ("removeHandleKind", "Removes one handle kind by id from the 2D block.", "Entfernt eine Griffart anhand ihrer Id aus dem 2D-Block."),
    ("addHandle", "Adds a new handle of the first handle kind to the 2D block at a default angle and radius; without any handle kind nothing happens.", "Fügt dem 2D-Block einen neuen Griff der ersten Griffart mit Standardwinkel und -radius hinzu; ohne Griffart geschieht nichts."),
    ("removeHandle", "Removes one handle by id from the 2D block.", "Entfernt einen Griff anhand seiner Id aus dem 2D-Block."),
    ("addCompatibilityRule", "Adds a two-way compatibility rule saying which handle kinds (source and target) may connect to each other.", "Fügt eine beidseitige Kompatibilitätsregel hinzu, welche Griffarten (Quelle und Ziel) miteinander verbunden werden dürfen."),
    ("removeCompatibilityRule", "Removes one compatibility rule by id, so its two handle kinds may no longer connect.", "Entfernt eine Kompatibilitätsregel anhand ihrer Id, sodass ihre zwei Griffarten nicht mehr verbunden werden dürfen."),
    ("setActiveExample", "Loads one of the plugin's bundled 2D block examples into the document, replacing what differs, by example id.", "Lädt eines der mitgelieferten 2D-Block-Beispiele in das Dokument und ersetzt Abweichendes, anhand der Beispiel-Id."),
    ("edit", "Replaces the whole 2D block document with one parsed from the given JSON text; invalid or identical JSON changes nothing.", "Ersetzt das gesamte 2D-Block-Dokument durch eines aus dem angegebenen JSON-Text; ungültiges oder identisches JSON ändert nichts."),
], destructive=["edit"])

describe(f"{B}/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_block3d_app", [
    ("patchObjectKind", "Sets one field of the 3D block's object kind: its name, label, variant, description or icon.", "Setzt ein Feld der Objektart des 3D-Blocks: Name, Bezeichnung, Variante, Beschreibung oder Symbol."),
    ("addRepresentation", "Adds a new, empty representation (a named mesh view of the object with tags and level of detail) to the 3D block.", "Fügt dem 3D-Block eine neue, leere Darstellung hinzu (eine benannte Netzansicht des Objekts mit Tags und Detailstufe)."),
    ("removeRepresentation", "Removes one representation by id from the 3D block.", "Entfernt eine Darstellung anhand ihrer Id aus dem 3D-Block."),
    ("addVortexKind", "Adds a new vortex kind (a named, coloured connection-point type with a default cable kind) to the 3D block.", "Fügt dem 3D-Block eine neue Wirbelart hinzu (einen benannten, farbigen Anschlusspunkttyp mit Standard-Kabelart)."),
    ("removeVortexKind", "Removes one vortex kind by id from the 3D block.", "Entfernt eine Wirbelart anhand ihrer Id aus dem 3D-Block."),
    ("addVortex", "Adds a new vortex (connection point) of the first vortex kind at the origin, facing up; without any vortex kind nothing happens.", "Fügt einen neuen Wirbel (Anschlusspunkt) der ersten Wirbelart im Ursprung, nach oben gerichtet, hinzu; ohne Wirbelart geschieht nichts."),
    ("removeVortex", "Removes one vortex (connection point) by id from the 3D block.", "Entfernt einen Wirbel (Anschlusspunkt) anhand seiner Id aus dem 3D-Block."),
    ("setActiveExample", "Loads one of the plugin's bundled 3D block examples (such as the Nakagin capsule or a hexagonal cut-concrete forest piece), replacing what differs, by example id.", "Lädt eines der mitgelieferten 3D-Block-Beispiele (etwa die Nakagin-Kapsel oder ein sechseckiges Betonwald-Teil) und ersetzt Abweichendes, anhand der Beispiel-Id."),
    ("edit", "Replaces the whole 3D block document with one parsed from the given JSON text; invalid or identical JSON changes nothing.", "Ersetzt das gesamte 3D-Block-Dokument durch eines aus dem angegebenen JSON-Text; ungültiges oder identisches JSON ändert nichts."),
    ("setActiveRepresentation", "Chooses which representation of the 3D block the editor shows and edits, or none; the document is not changed.", "Wählt, welche Darstellung des 3D-Blocks der Editor zeigt und bearbeitet, oder keine; das Dokument ändert sich nicht."),
    ("setWindowRepresentations", "Sets which representations one 3D window shows, by window id; only that window's view changes.", "Legt fest, welche Darstellungen ein 3D-Fenster zeigt, anhand der Fenster-Id; nur die Ansicht dieses Fensters ändert sich."),
    ("toggleWindowRepresentation", "Shows or hides one representation in one 3D window; only that window's view changes.", "Blendet eine Darstellung in einem 3D-Fenster ein oder aus; nur die Ansicht dieses Fensters ändert sich."),
    ("setWindowArrangement", "Sets how one 3D window arranges the representations it shows (such as side by side or overlaid); only the view changes.", "Legt fest, wie ein 3D-Fenster seine Darstellungen anordnet (etwa nebeneinander oder überlagert); nur die Ansicht ändert sich."),
    ("setWindowSpacing", "Sets the spacing between representations laid out side by side in one 3D window; only the view changes.", "Legt den Abstand zwischen nebeneinander angeordneten Darstellungen in einem 3D-Fenster fest; nur die Ansicht ändert sich."),
    ("setBrushVortexKind", "Chooses the vortex kind the placement brush puts onto surfaces; only the editor's brush setting changes.", "Wählt die Wirbelart, die der Platzierungspinsel auf Flächen setzt; nur die Pinseleinstellung des Editors ändert sich."),
    ("setBrushRadius", "Sets the radius of the vortex placement brush; only the editor's brush setting changes.", "Legt den Radius des Wirbel-Platzierungspinsels fest; nur die Pinseleinstellung des Editors ändert sich."),
    ("setBrushFlip", "Sets whether the placement brush flips the direction of vortices it places; only the brush setting changes.", "Legt fest, ob der Platzierungspinsel die Richtung gesetzter Wirbel umkehrt; nur die Pinseleinstellung ändert sich."),
    ("patchRepresentation", "Sets one field of one representation by id, such as its name, mesh URL, tags or level of detail.", "Setzt ein Feld einer Darstellung anhand ihrer Id, etwa Name, Netz-URL, Tags oder Detailstufe."),
], [("worldSurfaceLeave", "Input"), ("worldSurfacePlace", "Input"), ("setCamera", "Chrome")], destructive=["edit"])

describe(f"{B}/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_block5d_app", [
    ("patchPartKind", "Sets one field of the 5D block's part kind: its name, label, variant, description or icon.", "Setzt ein Feld der Teilart des 5D-Blocks: Name, Bezeichnung, Variante, Beschreibung oder Symbol."),
    ("addGripKind", "Adds a new grip kind (a named, coloured connector type with a default rope kind) to the 5D block.", "Fügt dem 5D-Block eine neue Griffart hinzu (einen benannten, farbigen Verbindertyp mit Standard-Seilart)."),
    ("removeGripKind", "Removes one grip kind by id from the 5D block.", "Entfernt eine Griffart anhand ihrer Id aus dem 5D-Block."),
    ("addGrip", "Adds a new grip of the first grip kind with default 2D and 3D placement; without any grip kind nothing happens.", "Fügt einen neuen Griff der ersten Griffart mit Standard-2D- und -3D-Platzierung hinzu; ohne Griffart geschieht nichts."),
    ("removeGrip", "Removes one grip by id from the 5D block.", "Entfernt einen Griff anhand seiner Id aus dem 5D-Block."),
    ("setActiveExample", "Loads one of the plugin's bundled 5D block examples into the document, replacing what differs, by example id.", "Lädt eines der mitgelieferten 5D-Block-Beispiele in das Dokument und ersetzt Abweichendes, anhand der Beispiel-Id."),
    ("edit", "Replaces the whole 5D block document with one parsed from the given JSON text; invalid or identical JSON changes nothing.", "Ersetzt das gesamte 5D-Block-Dokument durch eines aus dem angegebenen JSON-Text; ungültiges oder identisches JSON ändert nichts."),
], destructive=["edit"])
