"""🏗️ D1 codemod: agent-facing en/de descriptions for the fem2d and fem3d structural analysis editors."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
from d1_apply import describe

F = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts"

def shared(dim, dim_de, model, model_de):
    return [
        ("addBar", f"Adds a bar (an axial-only truss member) between two existing nodes of the {dim} model, with a material and a section.", f"Fügt zwischen zwei vorhandenen Knoten des {dim_de}-Modells einen Stab (ein nur normalkraftbeanspruchtes Fachwerkglied) mit Material und Querschnitt hinzu."),
        ("addMaterial", "Adds a named material with its Young's modulus (and, in 3D, its shear modulus) for elements to use.", "Fügt ein benanntes Material mit seinem Elastizitätsmodul (in 3D auch Schubmodul) hinzu, das Elemente verwenden können."),
        ("addSection", "Adds a named cross-section with its area and second moment of area for bars and beams to use.", "Fügt einen benannten Querschnitt mit Fläche und Flächenträgheitsmoment hinzu, den Stäbe und Balken verwenden können."),
        ("addSupport", "Adds a support to one node, fixing the given degrees of freedom (translations and rotations) against movement.", "Fügt einem Knoten ein Lager hinzu, das die angegebenen Freiheitsgrade (Verschiebungen und Verdrehungen) festhält."),
        ("addNodalLoad", "Adds a point load of the given value on one degree of freedom of a node, in a load case.", "Fügt eine Einzellast mit dem angegebenen Wert auf einen Freiheitsgrad eines Knotens in einem Lastfall hinzu."),
        ("addMemberUdl", "Adds a uniformly distributed load (wx along, wy across) on one bar or beam element, in a load case.", "Fügt eine gleichmäßig verteilte Streckenlast (wx längs, wy quer) auf ein Stab- oder Balkenelement in einem Lastfall hinzu."),
        ("addAreaLoad", f"Adds a uniform pressure load on one {model}, in a load case.", f"Fügt eine gleichmäßige Flächenlast (Druck) auf {model_de} in einem Lastfall hinzu."),
        ("addLoadCase", "Adds a named load case that loads are assigned to, optionally including the structure's self weight.", "Fügt einen benannten Lastfall hinzu, dem Lasten zugeordnet werden, optional mit dem Eigengewicht des Tragwerks."),
        ("addCombination", "Adds a named load combination that sums weighted load cases for the analysis.", "Fügt eine benannte Lastfallkombination hinzu, die gewichtete Lastfälle für die Analyse überlagert."),
        ("setSelfWeight", "Turns the structure's self weight on or off in one load case.", "Schaltet das Eigengewicht des Tragwerks in einem Lastfall ein oder aus."),
        ("setAnalysisSettings", "Sets how many vibration modes and buckling modes the analysis computes and the scale deformations are drawn with.", "Legt fest, wie viele Eigenschwingungs- und Knickformen die Analyse berechnet und mit welchem Maßstab Verformungen gezeichnet werden."),
        ("removeSelection", "Deletes every selected node, element, support or load from the model, together with what depends on it.", "Löscht alle ausgewählten Knoten, Elemente, Lager oder Lasten samt allem, was davon abhängt, aus dem Modell."),
        ("setActiveExample", f"Replaces the whole {dim} structural model with one of the plugin's bundled examples, by example id.", f"Ersetzt das gesamte {dim_de}-Tragwerksmodell durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."),
        ("setResultDisplay", "Chooses which analysis result the view shows: the static result of a load case or combination, or a numbered vibration or buckling mode; the model is not changed.", "Wählt, welches Analyseergebnis die Ansicht zeigt: das statische Ergebnis eines Lastfalls oder einer Kombination oder eine nummerierte Eigen- oder Knickform; das Modell ändert sich nicht."),
        ("patchNode", "Sets one field of one node by id, such as a coordinate.", "Setzt ein Feld eines Knotens anhand seiner Id, etwa eine Koordinate."),
        ("patchElement", "Sets one field of one bar or beam element by id, such as its material, section or end nodes.", "Setzt ein Feld eines Stab- oder Balkenelements anhand seiner Id, etwa Material, Querschnitt oder Endknoten."),
        ("patchMaterial", "Sets one field of one material by id, such as its name or Young's modulus.", "Setzt ein Feld eines Materials anhand seiner Id, etwa Name oder Elastizitätsmodul."),
        ("patchSection", "Sets one field of one cross-section by id, such as its area or second moment of area.", "Setzt ein Feld eines Querschnitts anhand seiner Id, etwa Fläche oder Flächenträgheitsmoment."),
        ("patchSupport", "Sets one field of one support by id, such as which degrees of freedom it fixes.", "Setzt ein Feld eines Lagers anhand seiner Id, etwa welche Freiheitsgrade es festhält."),
        ("patchLoad", "Sets one field of one load by id, such as its value, direction or load case.", "Setzt ein Feld einer Last anhand ihrer Id, etwa Wert, Richtung oder Lastfall."),
        ("patchLoadCase", "Sets one field of one load case by id, such as its name or whether it includes self weight.", "Setzt ein Feld eines Lastfalls anhand seiner Id, etwa Name oder ob er das Eigengewicht enthält."),
        ("patchCombination", "Sets one field of one load combination by id, such as its name or the factors of its load cases.", "Setzt ein Feld einer Lastfallkombination anhand ihrer Id, etwa Name oder die Faktoren ihrer Lastfälle."),
        ("translateSelection", "Moves the selected nodes by dx and dy, dragging the attached elements along.", "Verschiebt die ausgewählten Knoten um dx und dy und zieht die angeschlossenen Elemente mit."),
        ("rotateSelection", "Rotates the selected nodes by the given angle around their common centre.", "Dreht die ausgewählten Knoten um den angegebenen Winkel um ihren gemeinsamen Mittelpunkt."),
        ("scaleSelection", "Scales the positions of the selected nodes by sx and sy about their common centre.", "Skaliert die Positionen der ausgewählten Knoten um sx und sy um ihren gemeinsamen Mittelpunkt."),
    ]

describe(f"{F}/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs", "create_fem2d_app", [
    ("addNode", "Adds a node at x, y to the 2D structural model; elements, supports and loads attach to nodes.", "Fügt dem 2D-Tragwerksmodell einen Knoten an x, y hinzu; Elemente, Lager und Lasten hängen an Knoten."),
    ("addBeam", "Adds a beam (a bending member) between two existing nodes of the 2D model, with a material and a section.", "Fügt zwischen zwei vorhandenen Knoten des 2D-Modells einen Balken (ein biegebeanspruchtes Glied) mit Material und Querschnitt hinzu."),
    ("addRegion", "Adds a rectangular plate region at x, y with a width and height, material, thickness and mesh size, meshed into finite elements.", "Fügt einen rechteckigen Scheibenbereich an x, y mit Breite, Höhe, Material, Dicke und Netzweite hinzu, der in finite Elemente vernetzt wird."),
    ("patchRegion", "Sets one field of one plate region by id, such as its size, thickness or mesh size.", "Setzt ein Feld eines Scheibenbereichs anhand seiner Id, etwa Größe, Dicke oder Netzweite."),
] + shared("2D", "2D", "plate region", "einen Scheibenbereich"), [("setCamera", "Chrome")])

describe(f"{F}/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs", "create_fem3d_app", [
    ("addNode", "Adds a node at x, y, z to the 3D structural model; elements, supports and loads attach to nodes.", "Fügt dem 3D-Tragwerksmodell einen Knoten an x, y, z hinzu; Elemente, Lager und Lasten hängen an Knoten."),
    ("addFrame", "Adds a frame member (a 3D beam carrying bending and torsion) between two existing nodes, with a material, a section and a roll angle.", "Fügt zwischen zwei vorhandenen Knoten ein Rahmenglied (einen 3D-Balken für Biegung und Torsion) mit Material, Querschnitt und Rollwinkel hinzu."),
    ("addSolid", "Adds a box-shaped solid at x, y with a width, depth and height, a material, base elevation, layer count, mesh size and axis, meshed into finite elements.", "Fügt einen quaderförmigen Volumenkörper an x, y mit Breite, Tiefe, Höhe, Material, Basishöhe, Schichtanzahl, Netzweite und Achse hinzu, der in finite Elemente vernetzt wird."),
    ("patchSolid", "Sets one field of one solid by id, such as its size, material or mesh size.", "Setzt ein Feld eines Volumenkörpers anhand seiner Id, etwa Größe, Material oder Netzweite."),
] + shared("3D", "3D", "solid face region", "einen Flächenbereich eines Volumenkörpers"), [("setCamera", "Chrome")])
