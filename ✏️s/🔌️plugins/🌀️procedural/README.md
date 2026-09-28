---
name: procedural
kind: user
---

# Procedural

## English

Procedural 3D builds shapes as editable graphs. Use the catalogue to add operations, connect their typed ports, and adjust values in the inspector or through input sliders. The Flow document records graph changes through the existing history system.

**Mesh Workbench** starts with a box, insets a face, extrudes the inset, measures the result, and previews the mesh directly. A connected conversion widget also provides a faceted B-Rep. Open it from the examples menu or the `🛠️dev🔧️procedural🏙️3d🥽️mesh-workbench⚛️react` launch configuration. The inset and extrusion sliders control the live graph.

The B-Rep catalogue contains primitives, curves, surfaces, sweeps, booleans, edge features, transformations, topology queries, measurements, and interchange operations. Each operation's quality tag describes the kernel's fidelity.

In the editor preview, choose **Faces**, **Edges**, or **Vertices**, then pick components. **Edit Mesh Selection** in the action palette (Ctrl/⌘+Shift+M) offers extrusion, inset, subdivision, winding reversal, face deletion, loop cutting, and vertex displacement. The selection rail offers quick face operations and **Cut Loop** for edges. Loop cuts traverse connected quad strips, share new edge vertices, and form grids where cuts cross. Adjust **Cuts** (1–256) in the resulting widget. Each edit adds an adjustable widget and reconnects downstream geometry and measurements. Select components of one mesh at a time; B-Rep geometry must first pass through **Mesh from B-Rep**. **Objects** returns to graph-linked object selection.

The move, rotate, and scale handles act on the selected components. Rotation and scaling use the center of the selected vertices; shared vertices move once. The resulting component widget retains the selection, updates downstream measurements, and can be adjusted in the inspector. Set its **Pivot** to `point` and edit **Center** to use an explicit pivot. Component edits preserve polygon indices.

**Knife Cut Mesh Face** splits the face chosen by its zero-based **Face** index. Set **Start** and **End** to two points defining the cutting line; the line is projected onto the face. New boundary vertices are shared with neighboring faces. Convex planar pieces remain polygons; concave or warped faces are cut through their triangulation. A line that misses the face or only touches its boundary reports an error.

To cut a picked face, choose **Faces** in the editor preview, select one face, and open **Knife Cut Selected Face** (Ctrl/⌘+Shift+K). Enter **Cut Start** and **Cut End** in the point controls. The command inserts the knife widget, reconnects its consumers, and clears the old component selection because the cut changes topology. Undo and redo operate on the graph edit.

The mesh catalogue contains creation, editing, repair, analysis, and interchange groups. Mesh data keeps indexed polygon faces. **Construct Mesh** accepts JSON text with `vertices` (three coordinates per vertex) and `faces` (zero-based vertex indices per polygon). For example:

```json
{"vertices":[[0,0,0],[1,0,0],[1,1,0],[0,1,0]],"faces":[[0,1,2,3]]}
```

Connect `meshOut` to another widget's `mesh` port. Face and vertex selections are JSON index lists such as `[0,2]`. **Analyze Mesh** reports vertex, edge, face, and triangle counts, bounding extents, area, boundary edges, nonmanifold edges, and inconsistent winding. Volume is absent for open, degenerate, or inconsistently wound surfaces; this check does not certify absence of self-intersections. Enable a mesh node’s preview toggle to inspect it directly in the viewport. Translation, rotation, and scaling create editable transform nodes in the graph. Rotation angles are in radians and successive gestures compose in world-axis order. Scale factors act independently on X, Y, and Z; negative factors reflect the shape. The resulting transform remains selected for the next gesture. **Faceted B-Rep from Mesh** connects mesh output to B-Rep operations. This conversion preserves a faceted surface, not an analytic curved surface. Use **Mesh to OBJ** or **Mesh to JSON** for text export.

Mesh inputs currently admit up to 100,000 vertices/faces and 600,000 corners. Inset rejects distances beyond the local edge clearance. Keep the existing example when experimenting and use history to undo graph changes.

## Deutsch

Procedural 3D erzeugt Formen als bearbeitbare Graphen. Operationen kommen aus dem Katalog. Verbinde ihre typisierten Anschlüsse und bearbeite Werte im Inspektor oder über Schieberegler. Graphänderungen werden über die vorhandene Verlaufssystematik gespeichert.

Die **Netzwerkstatt** beginnt mit einem Quader, versetzt eine Fläche nach innen, extrudiert sie, analysiert das Ergebnis und zeigt das Netz direkt in der 3D-Vorschau. Ein verbundener Umwandlungsknoten liefert zusätzlich einen facettierten B-Rep. Öffne das Beispiel über das Beispielmenü oder die Startkonfiguration `🛠️dev🔧️procedural🏙️3d🥽️mesh-workbench⚛️react`. Die beiden Regler steuern Einrückung und Extrusion.

Der Netzkatalog enthält Erstellung, Bearbeitung, Reparatur, Analyse und Datenaustausch. **Construct Mesh** nimmt JSON mit `vertices` (drei Koordinaten pro Punkt) und `faces` (Polygonindizes ab null) entgegen. Verbinde `meshOut` mit `mesh`. Flächen- und Punktauswahlen sind Indexlisten wie `[0,2]`.

Die Griffe zum Verschieben, Drehen und Skalieren wirken auf die ausgewählten Komponenten. Drehen und Skalieren verwenden den Mittelpunkt der ausgewählten Punkte; gemeinsame Punkte werden einmal verändert. Das Komponentenwidget behält die Auswahl, aktualisiert nachgelagerte Messungen und lässt sich im Inspektor bearbeiten. **Pivot** mit dem Wert `point` und **Center** legen einen eigenen Bezugspunkt fest. Die Polygonindizes bleiben erhalten.

**Knife Cut Mesh Face** schneidet die über **Face** gewählte Fläche; der Index beginnt bei null. **Start** und **End** bestimmen die Schnittgerade, die auf die Fläche projiziert wird. Neue Randpunkte werden mit angrenzenden Flächen geteilt. Konvexe ebene Teile bleiben Polygone; konkave oder verwundene Flächen werden über ihre Triangulierung geschnitten. Eine Gerade, die die Fläche verfehlt oder nur ihren Rand berührt, meldet einen Fehler.

Wähle **Flächen** in der Editorvorschau, markiere eine Fläche und öffne **Ausgewählte Fläche schneiden** (Strg/⌘+Umschalt+K). Trage **Schnittanfang** und **Schnittende** in die Punktfelder ein. Der Befehl fügt das Schnittwidget ein, verbindet die nachgelagerten Widgets neu und löscht die alte Komponentenauswahl, da sich die Topologie ändert. Rückgängig und Wiederholen wirken auf die Graphänderung.

Wähle in der Editorvorschau **Flächen**, **Kanten** oder **Eckpunkte** und klicke die Komponenten an. **Netzauswahl bearbeiten** in der Aktionspalette (Strg/⌘+Umschalt+M) bietet Extrusion, Einzug, Unterteilung, Umkehrung, Flächenlöschung, Schleifenschnitte und Punktverschiebung. Die Auswahl bietet direkte Flächenaktionen und **Schleife schneiden** für Kanten. Die Schnitte folgen verbundenen Viereckstreifen, teilen neue Kantenpunkte und bilden bei Kreuzungen ein Raster. **Cuts** im erzeugten Widget steuert die Schnittanzahl (1–256). Jede Bearbeitung fügt ein einstellbares Widget ein und verbindet nachgelagerte Geometrie und Messungen neu. Wähle Komponenten jeweils eines Netzes; B-Rep-Geometrie wird zuerst mit **Mesh from B-Rep** umgewandelt. **Objekte** schaltet zurück zur Objektauswahl.

**Analyze Mesh** liefert Topologieanzahlen, Ausdehnung, Oberfläche, Randkanten, nichtmannigfaltige Kanten und widersprüchliche Orientierung. Für offene oder entartete Netze sowie bei widersprüchlicher Orientierung wird kein Volumen ausgegeben. Selbstüberschneidungen werden damit nicht ausgeschlossen. Aktiviere die Vorschau eines Netzknotens, um ihn direkt im Ansichtsfenster zu prüfen. Verschieben, Drehen und Skalieren erzeugen bearbeitbare Transformationsknoten im Graphen. Drehwinkel werden im Bogenmaß angegeben und aufeinanderfolgende Gesten in Weltachsenreihenfolge zusammengesetzt. Skalierungsfaktoren wirken unabhängig auf X, Y und Z; negative Faktoren spiegeln die Form. Die resultierende Transformation bleibt für die nächste Geste ausgewählt. **Faceted B-Rep from Mesh** verbindet Netze mit B-Rep-Operationen; gekrümmte analytische Flächen werden dabei nicht rekonstruiert. **Mesh to OBJ** und **Mesh to JSON** liefern Text für den Export.
