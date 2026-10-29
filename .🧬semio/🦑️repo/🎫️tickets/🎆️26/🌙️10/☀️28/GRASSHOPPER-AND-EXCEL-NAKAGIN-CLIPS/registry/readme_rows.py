"""📝 Refresh the 'what the viewer sees' column of both series READMEs from the final scene reports."""

from __future__ import annotations

from series import ARCH

ROWS = {
    "🦗️grasshopper": {
        "grasshopper_tower": (
            "The core with square plan, storey height and storey count, a series of planes for every storey and facade, capsule orientation with Orient and List Item, the capital on top of each core and the lower building as simple massing; counts read from the definition (A 9 × 3 × 2 = 54, B 11 × 4 × 2 = 88 minus 2 culled planes = 86), including the typical mistake of a forgotten Cull Index and its fix",
            "Der Kern mit quadratischem Grundriss, Geschosshöhe und Geschosszahl, eine Serie von Ebenen für jedes Geschoss und jede Fassade, Kapselausrichtung mit Orient und List Item, das Kapitell auf jedem Kern und das Sockelgebäude als einfache Kubatur; Anzahlen aus der Definition gelesen (A 9 × 3 × 2 = 54, B 11 × 4 × 2 = 88 minus 2 entfernte Ebenen = 86), samt typischem Fehler eines vergessenen Cull Index und seiner Korrektur",
        ),
        "grasshopper_casting": (
            "Every parameter has a type and mismatched data is cast on one capsule slot of Tower B: Number to Integer with its rounding rule, Boolean to Integer, Point to Plane (the capsule faces the wrong way), Line to Plane, text to Number, number to Domain, Circle to Curve, Box to Brep, number to Boolean, and the cast that fails with a red component and its message; rows not yet verified in the running app carry an on-screen marker; a casting matrix built from what was shown",
            "Jeder Parameter hat einen Typ, unpassende Daten werden an einem Kapselplatz von Turm B umgewandelt: Number zu Integer mit Rundungsregel, Boolean zu Integer, Point zu Plane (die Kapsel schaut in die falsche Richtung), Line zu Plane, Text zu Number, Zahl zu Domain, Circle zu Curve, Box zu Brep, Zahl zu Boolean und die Umwandlung, die mit roter Komponente und Meldung scheitert; noch nicht in der laufenden Anwendung geprüfte Zeilen tragen eine Markierung im Bild; eine Casting-Matrix aus dem Gezeigten",
        ),
        "grasshopper_trees": (
            "The aggregated tower of 140 capsules as a tree with the path `{BUILDING;TOWER;STORY;FACADE}` drawn as a real tree beside the tower: path anatomy, Panel and Param Viewer, Flatten Tree, Graft Tree, Simplify Tree, Path Mapper, Flip Matrix, counts per tower and storey as lit capsules, the data-matching mistake of two lists with different depths (Custom Preview colours the wrong capsules) and its repair with Path Mapper and Graft Tree, and a recap of ten terms",
            "Der gesamte Turm mit 140 Kapseln als Datenbaum mit dem Pfad `{BUILDING;TOWER;STORY;FACADE}`, als echter Baum neben dem Turm gezeichnet: Pfadaufbau, Panel und Param Viewer, Flatten Tree, Graft Tree, Simplify Tree, Path Mapper, Flip Matrix, Anzahlen je Turm und Geschoss als leuchtende Kapseln, der Zuordnungsfehler zweier Listen unterschiedlicher Tiefe (Custom Preview färbt die falschen Kapseln) und seine Behebung mit Path Mapper und Graft Tree sowie ein Rückblick auf zehn Begriffe",
        ),
        "grasshopper_capsule": (
            "One parametric capsule: sliders for width, length, height, radius, window diameter and window offset drive a box with rounded edges, the round window and the four bolt points, with live dimension lines; the capsule types as variants of one definition, a typical mistake with a circle on the wrong plane, each type moved with Orient from its base plane onto a target plane and picked with List Item (also with Wrap) over eight planes",
            "Eine parametrische Kapsel: Slider für Breite, Länge, Höhe, Radius, Fensterdurchmesser und Fensterversatz steuern eine Box mit gerundeten Kanten, das runde Fenster und die vier Schraubpunkte, mit Maßlinien in Echtzeit; die Kapseltypen als Varianten einer Definition, ein typischer Fehler mit einem Kreis auf der falschen Ebene, jeder Typ per Orient von seiner Grundebene auf eine Zielebene gesetzt und mit List Item (auch mit Wrap) über acht Ebenen gewählt",
        ),
    },
    "📊️excel": {
        "excel_references": (
            "Relative references filled down the surface rows of Type A (U·A per face, each filled row lights its capsule face), the absolute `$D$10` for the design temperature difference with the four F4 forms, mixed references in the Program tower-by-type table, references across sheets, the 3D reference `SUM('Type A:Type D'!B13)` and defined names; what breaks (a reference that slides, a Type B tab moved out of the span, a deleted sheet with `#REF!`) shown as coloured frames and values before and after, and a six-row legend",
            "Relative Bezüge, nach unten über die Flächenzeilen von Typ A gefüllt (U·A je Fläche, jede gefüllte Zeile lässt ihre Kapselfläche aufleuchten), der absolute `$D$10` für die Auslegungstemperaturdifferenz mit den vier F4-Formen, gemischte Bezüge in der Kreuztabelle Turm mal Typ auf dem Blatt Program, Bezüge über Blätter, der 3D-Bezug `SUMME('Typ A:Typ D'!B13)` und definierte Namen; was bricht (ein wandernder Bezug, ein aus der Spanne verschobenes Blatt Typ B, ein gelöschtes Blatt mit `#BEZUG!`) als farbige Rahmen und Werte vorher und nachher sowie eine Legende mit sechs Zeilen",
        ),
        "excel_program": (
            "The room program: net floor area and volume of a capsule from its fittings, the table `Capsules` listing all 140 capsules with tower, storey, facade and type, COUNTIF and COUNTIFS per type and tower, XLOOKUP of type values, SUMPRODUCT of counts × areas, SUMIF totals per tower and a bar chart of areas, with every count lighting the matching capsules on the tower; a swapped-in value updates the dependents, and a check, a typo with a trailing space and its repair show how a count goes wrong",
            "Das Raumprogramm: Nettogrundfläche und Volumen einer Kapsel aus ihren Einbauten, die Tabelle `Capsules` mit allen 140 Kapseln samt Turm, Geschoss, Fassade und Typ, ZÄHLENWENN und ZÄHLENWENNS je Typ und Turm, XVERWEIS der Typwerte, SUMMENPRODUKT aus Anzahl mal Fläche, SUMMEWENN-Summen je Turm und ein Balkendiagramm der Flächen, wobei jede Anzahl die passenden Kapseln am Turm aufleuchten lässt; ein ausgetauschter Wert aktualisiert die abhängigen Zellen, und eine Prüfung, ein Tippfehler mit Leerzeichen und seine Korrektur zeigen, wie eine Zählung falsch wird",
        ),
        "excel_energy": (
            "Envelope surfaces from the dimensions, exposed against attached to the core (back wall factor b), U-values, U·A per surface, H_T by SUM and by SUMPRODUCT, the Tokyo climate with defined names, a wrong climate value and its repair, H_V from air change rate and volume, heating load, annual heating demand and kWh/(m²·a); the Energy sheet sums all capsules by type count and compares the types in a chart; a what-if with a better window U-value updates every dependent cell, arrow and total, and a frozen typed total shows a stale cell and its repair",
            "Hüllflächen aus den Maßen, außenliegend gegenüber am Kern angeschlossen (Faktor b der Rückwand), U-Werte, U·A je Fläche, H_T mit SUMME und mit SUMMENPRODUKT, das Klima von Tokio mit definierten Namen, ein falscher Klimawert und seine Korrektur, H_V aus Luftwechselrate und Volumen, Heizlast, jährlicher Heizwärmebedarf und kWh/(m²·a); das Blatt Energy summiert alle Kapseln nach Typanzahl und vergleicht die Typen im Diagramm; ein Was-wäre-wenn mit besserem Fenster-U-Wert aktualisiert alle abhängigen Zellen, Pfeile und Summen, und eine als Konstante eingetippte Summe zeigt eine veraltete Zelle und ihre Reparatur",
        ),
    },
}

for folder, rows in ROWS.items():
    path = ARCH / folder / "🎬️clip" / "README.md"
    lines = path.read_text(encoding="utf-8").split("\n")
    seen = set()
    for index, line in enumerate(lines):
        for video, texts in rows.items():
            if line.startswith(f"| {video} |"):
                cells = line.split(" | ")
                cells[1] = texts[len(seen & {video + "x"}) if False else (0 if video not in seen else 1)]
                seen.add(video)
                lines[index] = " | ".join(cells)
    assert seen == set(rows), seen
    path.write_text("\n".join(lines), encoding="utf-8", newline="\n")
    print(f"[DEBUG] updated {folder}")
