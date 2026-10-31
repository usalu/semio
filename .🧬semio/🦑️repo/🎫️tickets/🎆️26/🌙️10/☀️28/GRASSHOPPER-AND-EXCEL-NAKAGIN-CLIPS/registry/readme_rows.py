"""📝 Refresh the 'what the viewer sees' column of both series READMEs from the current scene narration (only Markdown is written)."""

from __future__ import annotations

import time

from series import ARCH

ROWS = {
    "🦗️grasshopper": {
        "grasshopper_anatomy": (
            "The capsule shell built on an empty canvas: script and canvas, a Number Slider (domain, decimals, value) wired into Center Box, component anatomy (name and nickname, inputs left, outputs right, wires, preview eye, grey, orange and red states), floating and input parameters, a Panel showing the output with its path and index, a Boolean Toggle for the round window, a Value List of the A–D studies, a plugin adding a ribbon tab, and item, list (Series), data tree and path plus index on the capsule levels of Tower B, closing with bilingual canvas and data glossaries",
            "Die Kapselhülle auf leerem Canvas aufgebaut: Skript und Canvas, ein Number Slider (Bereich, Dezimalstellen, Wert) an Center Box, der Aufbau einer Komponente (Name und Kurzname, Eingänge links, Ausgänge rechts, Drähte, Vorschau-Auge, graue, orange und rote Zustände), freie und Eingabeparameter, ein Panel mit Ausgabe, Pfad und Index, ein Boolean Toggle für das runde Fenster, eine Value List der Studien A–D, ein Plugin mit eigenem Reiter sowie Element, Liste (Series), Datenbaum und Pfad plus Index an den Kapselgeschossen von Turm B, abgeschlossen mit zweisprachigen Glossaren zu Canvas und Daten",
        ),
        "grasshopper_capsule": (
            "One parametric capsule: six sliders for the physical 2.6 × 4.2 m reference casing (width, length, height, corner radius, window diameter and offset) drive Center Box on a base plane, Plane Origin, Fillet Edge, the round window cut with Solid Difference and the four bolt points, with live dimension lines; slider domains as design rules (the envelope turns red past the limit), the A–D room-program studies sharing the nominal envelope and 1.30 m porthole, a typical mistake with a circle on the wrong plane, Orient from a base plane onto a rotating target plane, and List Item over eight planes with and without Wrap",
            "Eine parametrische Kapsel: sechs Slider für die physische Referenzhülle von 2,6 × 4,2 m (Breite, Länge, Höhe, Eckenradius, Fensterdurchmesser und -versatz) steuern Center Box auf einer Grundebene, Plane Origin, Fillet Edge, das mit Solid Difference geschnittene runde Fenster und die vier Schraubpunkte, mit Maßlinien in Echtzeit; Slider-Bereiche als Entwurfsregeln (die Hülle wird jenseits der Grenze rot), die Raumprogramm-Studien A–D mit gemeinsamer Nennhülle und 1,30 m Bullauge, ein typischer Fehler mit einem Kreis auf der falschen Ebene, Orient von einer Grundebene auf eine rotierende Zielebene und List Item über acht Ebenen mit und ohne Wrap",
        ),
        "grasshopper_tower": (
            "The core envelope and its detailed assembly, a Series of planes at the 2.80 m pitch from the podium roof, Construct Plane door-port frames with visible axes, authored floor masks selecting the occupied positions, Orient from local to world door port, a type pattern read with List Item, the tower revealed level by level, the mistake of an unmasked rectangular grid and its correction with Cull Index (A: 76, B: 64, together 140 read from the definition), the machinery head (A 11.36 m sourced, B provisional), the podium envelope with named assemblies and a room-program switch from B to C",
            "Die Kernhülle mit ihrem detaillierten Aufbau, eine Series von Ebenen im Raster von 2,80 m ab dem Sockeldach, Construct-Plane-Türanschlussrahmen mit sichtbaren Achsen, festgelegte Geschossmasken für die belegten Positionen, Orient vom lokalen auf den Welt-Türanschluss, ein mit List Item gelesenes Typmuster, der Turm Geschoss für Geschoss enthüllt, der Fehler eines unmaskierten Rechteckrasters und seine Korrektur mit Cull Index (A: 76, B: 64, zusammen 140, aus der Definition gelesen), der Maschinenkopf (A 11,36 m belegt, B vorläufig), die Sockelhülle mit benannten Baugruppen und ein Wechsel des Raumprogramms von B zu C",
        ),
        "grasshopper_casting": (
            "Every parameter has a type and mismatched data is cast on one capsule slot: text to Number via a Panel, Number to Integer with its rounding rule (and Floor to decide it yourself), Point to Plane (the capsule faces north instead of west), Line to Plane with guessed X and Y and the explicit plane that fixes it, number to Domain with Divide Domain, Circle to Curve, Box to Brep, Boolean to Integer and number to Boolean, the decimal-comma pitfall, and the cast that fails with a red component and its message, repaired with a Value List; a casting matrix of what was shown (green worked, red failed, dashed still to check in Rhino 8)",
            "Jeder Parameter hat einen Typ, unpassende Daten werden an einem Kapselplatz umgewandelt: Text zu Number über ein Panel, Number zu Integer mit Rundungsregel (und Floor, um selbst zu entscheiden), Point zu Plane (die Kapsel schaut nach Norden statt nach Westen), Line zu Plane mit erratenen X und Y und die explizite Ebene als Korrektur, Zahl zu Domain mit Divide Domain, Circle zu Curve, Box zu Brep, Boolean zu Integer und Zahl zu Boolean, die Dezimalkomma-Falle und die Umwandlung, die mit roter Komponente und Meldung scheitert und mit einer Value List behoben wird; eine Casting-Matrix des Gezeigten (grün gelang, rot scheiterte, gestrichelt noch in Rhino 8 zu prüfen)",
        ),
        "grasshopper_trees": (
            "The tower of 140 capsules as a tree with the path `{BUILDING;TOWER;STORY;FACADE}` drawn beside the tower: one capsule as item 21 of a flat list against its address, branches as occupied facade groups of variable length, path plus index, Panel and Tree Statistics, Flatten, Graft, Simplify, Path Mapper (`{A;B;C;D}(i)` to `{A;B;C}`, 20 storey branches of two, seven or eight, then `{A;B}` with 76 and 64), Flip Matrix with null placeholders, the data-matching mistake of two lists with different depths (Custom Preview colours the wrong capsules, shown in the Param Viewer) and its repair with Path Mapper and Graft Tree, and a recap of ten terms",
            "Der Turm mit 140 Kapseln als Datenbaum mit dem Pfad `{BUILDING;TOWER;STORY;FACADE}`, neben dem Turm gezeichnet: eine Kapsel als Element 21 einer flachen Liste im Vergleich zu ihrer Adresse, Äste als belegte Fassadengruppen variabler Länge, Pfad plus Index, Panel und Tree Statistics, Flatten, Graft, Simplify, Path Mapper (`{A;B;C;D}(i)` zu `{A;B;C}`, 20 Geschossäste mit zwei, sieben oder acht Kapseln, dann `{A;B}` mit 76 und 64), Flip Matrix mit Null-Platzhaltern, der Zuordnungsfehler zweier Listen unterschiedlicher Tiefe (Custom Preview färbt die falschen Kapseln, sichtbar im Param Viewer) und seine Behebung mit Path Mapper und Graft Tree sowie ein Rückblick auf zehn Begriffe",
        ),
    },
    "📊️excel": {
        "excel_anatomy": (
            "The workbook `Nakagin.xlsx`: worksheets (Program, one per capsule type, Climate, Energy), column, row, cell address, range, name box and formula bar; constants and text, displayed format against stored value (a rounded number with a unit), the capsule floor area as the first formula with coloured references and the area hatched on the capsule, the typical mistake of missing brackets (operator precedence) and its fix, recalculation when the width changes, precedents and dependents, functions and typical errors",
            "Die Arbeitsmappe `Nakagin.xlsx`: Tabellenblätter (Program, eines je Kapseltyp, Climate, Energy), Spalte, Zeile, Zelladresse, Bereich, Namenfeld und Bearbeitungsleiste; Konstanten und Text, angezeigtes Format gegenüber gespeichertem Wert (gerundete Zahl mit Einheit), die Kapselgrundfläche als erste Formel mit farbigen Bezügen und schraffierter Fläche an der Kapsel, der typische Fehler fehlender Klammern (Punkt vor Strich) und seine Korrektur, Neuberechnung bei geänderter Breite, Vorgänger und Nachfolger, Funktionen und typische Fehler",
        ),
        "excel_references": (
            "Relative references filled down the surface rows of Type A (U·A per face, each filled row lights its capsule face), the absolute `$D$10` for the design temperature difference with the four F4 forms and the mistake of a forgotten dollar sign, a two-way table of mixed references, the same pattern counting capsules by tower and type, references across sheets (a sliding sheet reference edited per copy), the 3D reference over the type sheets with its two failures (a sheet moved out of the span, a deleted sheet that silently shrinks the sum), an absolute reference broken by a deleted row, defined names, and a recap",
            "Relative Bezüge, nach unten über die Flächenzeilen von Typ A gefüllt (U·A je Fläche, jede gefüllte Zeile lässt ihre Kapselfläche aufleuchten), der absolute `$D$10` für die Auslegungstemperaturdifferenz mit den vier F4-Formen und dem Fehler eines vergessenen Dollarzeichens, eine Kreuztabelle gemischter Bezüge, dasselbe Muster zählt Kapseln nach Turm und Typ, Bezüge über Blätter (ein wandernder Blattbezug, je Kopie angepasst), der 3D-Bezug über die Typblätter mit seinen zwei Fehlern (ein aus der Spanne verschobenes Blatt, ein gelöschtes Blatt, das die Summe unbemerkt schrumpfen lässt), ein durch gelöschte Zeile gebrochener absoluter Bezug, definierte Namen und ein Rückblick",
        ),
        "excel_program": (
            "The room program: fittings and net floor area of a capsule on the type sheets, free floor and volume, the 140 capsule rows formatted as the table `Capsules` (a selected row lights its capsule), COUNTIF and COUNTIFS per type and tower, XLOOKUP of count and free floor by type, SUMPRODUCT of counts × free floor, a bar chart of the four totals, SUMIF per tower, swapping one capsule type to show the update, and a sanity check with the typo of a trailing space (139 and FALSE) and its repair, closing with a recap",
            "Das Raumprogramm: Einbauten und Nettogrundfläche einer Kapsel auf den Typblättern, freie Fläche und Volumen, die 140 Kapselzeilen als Tabelle `Capsules` formatiert (eine gewählte Zeile lässt ihre Kapsel aufleuchten), ZÄHLENWENN und ZÄHLENWENNS je Typ und Turm, XVERWEIS von Anzahl und freier Fläche nach Typ, SUMMENPRODUKT aus Anzahl mal freier Fläche, ein Balkendiagramm der vier Summen, SUMMEWENN je Turm, ein ausgetauschter Kapseltyp zeigt die Aktualisierung, und eine Plausibilitätsprüfung mit dem Tippfehler eines Leerzeichens (139 und FALSCH) und seiner Korrektur, abgeschlossen mit einem Rückblick",
        ),
        "excel_energy": (
            "Envelope surfaces from the nominal dimensions (reference casing kept separate), the attached back wall with factor b, U-values, U·A per surface as arrows, H_T by SUM and by SUMPRODUCT, the Climate sheet with defined names and heating degree days as an area, the mistake of treating the back wall as exposed and its repair, H_V from air change rate and volume, loss per kelvin, heating load, annual heating demand and kWh/m²; the Energy sheet multiplies by the capsule count of the Program sheet, a comparison of the four studies, a what-if with a better window U-value traced through dependents, and a typed building total that stops following the change, then its repair",
            "Hüllflächen aus den Nennmaßen (Referenzhülle getrennt gehalten), die am Kern angeschlossene Rückwand mit Faktor b, U-Werte, U·A je Fläche als Pfeile, H_T mit SUMME und mit SUMMENPRODUKT, das Blatt Climate mit definierten Namen und Heizgradtagen als Fläche, der Fehler, die Rückwand wie eine außenliegende zu behandeln, und seine Korrektur, H_V aus Luftwechselrate und Volumen, Verlust je Kelvin, Heizlast, jährlicher Heizwärmebedarf und kWh/m²; das Blatt Energy multipliziert mit der Kapselanzahl vom Blatt Program, ein Vergleich der vier Studien, ein Was-wäre-wenn mit besserem Fenster-U-Wert über die Nachfolger verfolgt und eine als Konstante eingetippte Gebäudesumme, die der Änderung nicht mehr folgt, samt Reparatur",
        ),
    },
}


def publish(path, text):
    for attempt in range(8):
        try:
            path.write_text(text, encoding="utf-8", newline="\n")
            return
        except OSError:
            if attempt == 7:
                raise
            time.sleep(0.5)


for folder, rows in ROWS.items():
    path = ARCH / folder / "🎬️clip" / "README.md"
    lines = path.read_text(encoding="utf-8").split("\n")
    seen = {}
    for index, line in enumerate(lines):
        for video, texts in rows.items():
            if line.startswith(f"| {video} |"):
                cells = line.split(" | ")
                cells[1] = texts[seen.get(video, 0)]
                seen[video] = seen.get(video, 0) + 1
                lines[index] = " | ".join(cells)
    assert all(count == 2 for count in seen.values()) and set(seen) == set(rows), seen
    publish(path, "\n".join(lines))
    print(f"[DEBUG] updated {folder}")
