---
name: norm
kind: user
---

# Norm

## English

The norm plugin checks a design against the Eurocodes (EN 1990 to EN 1999, with the German national annex NA-DE) and shows the result next to the model. This guide is shared by all ten Eurocode editors. Each code has its own page with covered clauses, diagrams, handles and examples:

| Code | Subject | Page |
|---|---|---|
| EN 1990 | Basis of design | [en1990](🗿️artifacts/⚖️en1990/README.md) |
| EN 1991 | Actions on structures | [en1991](🗿️artifacts/🏋️en1991/README.md) |
| EN 1992 | Concrete | [en1992](🗿️artifacts/🏛️en1992/README.md) |
| EN 1993 | Steel | [en1993](🗿️artifacts/🔩️en1993/README.md) |
| EN 1994 | Composite steel and concrete | [en1994](🗿️artifacts/🧩️en1994/README.md) |
| EN 1995 | Timber | [en1995](🗿️artifacts/🪵️en1995/README.md) |
| EN 1996 | Masonry | [en1996](🗿️artifacts/🪨️en1996/README.md) |
| EN 1997 | Geotechnical design | [en1997](🗿️artifacts/🌍️en1997/README.md) |
| EN 1998 | Earthquake resistance | [en1998](🗿️artifacts/🫨️en1998/README.md) |
| EN 1999 | Aluminium | [en1999](🗿️artifacts/🪶️en1999/README.md) |

### Read this first: what is verified

- The visual editors and the clause libraries are written, but the audits of the ticket `NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS` (state 2026-11-02) record that they were not yet compiled or run as a whole. Features described here are the designed and implemented behaviour; where an audit lists a feature as missing or deferred, the page says so.
- No normative value is verified against the printed standard yet. The NDP registry holds 520 entries, 0 verified. Results that rest on such a value are marked as unverified (see "Unverified basis"). Treat every result as a design aid that needs checking by a qualified engineer.

### The windows

An editor has five windows. The viewer (read-only) has three of them: 3D model, diagrams and checks.

| Window | Purpose |
|---|---|
| Model outline | Tree of all entities (members, walls, footings, actions, ...) with status glyph and worst utilisation; add, duplicate, remove, filter |
| Properties | Typed inspector of the selection: values with units, choices, entity references, the national-annex badge, live feedback per field |
| 3D model | Structural model with load glyphs, supports, dimension lines, clause badges, failure glyphs and utilisation colours; drag handles |
| Checks | All checks grouped by entity, clause or part, with utilisation bars, clause links and remedy chips |
| Diagrams | 2D diagrams of the selected entity (buckling curves, interaction polygons, spectra, profiles, ...), each with a table twin |

Desktop layout: outline above properties on the left (22 %), 3D model in the centre (50 %), checks above diagrams on the right (28 %). The read-only viewer uses 50 % / 25 % / 25 %. On mobile one window is shown at a time through a tab bar (Model, Properties, Checks, Diagrams); on tablet two panes are shown. You can dock, float or hide windows with the framework layout; layouts persist per user.

### Selection

There is one selection for all windows.

- Click an entity in the 3D model, the outline or a group row of the checks window. Rectangle selection and multi-select work in the 3D model. Sub-parts (a face of a plate, a bolt row) can be picked.
- Selecting a check row selects the entity it belongs to; the properties window scrolls to the checked field, and the diagrams switch to that entity. Without a selection the diagrams show the worst entity.
- Hovering a check row highlights the entity and the other way round.
- Known limit: the stored check selection and picks carry no timestamps; the windows show their union. Clearing a pick does not clear the stored check. The read-only viewer does not yet mirror picks.

### Editing values

- In Properties type a value, pick a unit, choose an option or an entity reference. Enter or leaving the field commits; Esc reverts; arrow keys step (Shift x10, Alt x0.1). The inspector shows the SI value on hover.
- Values outside the validity range of the formula are refused with a message. Fields that carry a national-annex parameter show an "NA-DE" badge and a reset to the annex default.
- Computed values (area, section class, ...) are read-only rows.
- Every field with a failing or near-limit check shows a small utilisation bar and, where the check offers one, a remedy chip such as "12 mm -> >= 22 mm" that applies the value in one click.
- Not yet available: inspector tiers (Basic, Advanced, Expert) and conditional visibility, because they wait for the common field-specification type.

### Drag handles

Grips are shown on the selected entity in the 3D model. A drag changes one or more input fields at once, for example the span of a member, a bolt pitch, the depth of a notch or the height of a storey.

1. Hover a grip: its name and current value appear ("Member B1 length handle").
2. Drag: the value updates live and every window previews it. Values snap to a grid, to catalogue values (bolt diameters, section sizes, lamella thickness) or to neighbouring entities, and cannot leave the limits of the field. Hold Ctrl to switch snapping off; the limits stay.
3. Release to commit. A drag is designed to be one undo step. Release without movement commits nothing. Esc or losing the pointer aborts and leaves no trace.
4. While dragging you may type a number (a decimal point or comma is accepted); it overrides the mouse position and is clamped to the limits.
5. Dimension lines bound to a field and markers in diagrams (for example a buckling-length marker) work the same way. A diagram marker for a derived value, such as the demand point, cannot be dragged.

Today a drag commits each affected field as a separate `setField`; the single bounded `setFields` command is implemented but still to be wired into each editor. Inserting entities by click with a ghost that follows the cursor, and inline numeric entry on a dimension line, are not implemented yet.

### Live evaluation

- Every edit re-evaluates the checks immediately; there is no separate "run" step. One retained evaluation session per editor holds the result; when you edit again while a run is in flight, the latest edit wins.
- EN 1993 re-evaluates per entity (only the checks of the edited member); the other codes re-run the full fold, memoised by input. Large models may therefore feel slower there.
- If an evaluation is pending, affected entities are shown greyed, never hidden.

### Utilisation: colours, patterns and glyphs

Utilisation is demand divided by resistance. 100 % is the limit.

| State | Glyph | Pattern | Meaning |
|---|---|---|---|
| Passes | check mark | none | below the warning threshold |
| Close to limit | exclamation or warning sign | horizontal hatching | from about 90 to 95 % utilisation (the exact threshold is set per check) |
| Fails | cross | diagonal hatching | above 100 % |
| Not applicable | empty set | none | check does not apply to this input |
| Unverified basis | question mark added to the glyph | dots | rests on an unverified national value |

- The default colour ramp is perceptually uniform from 0 to 100 % with a distinct break at 100 %; the scale runs to 120 %.
- Colour is never the only cue: glyph, hatching and text are always present. In the checks window a 12-cell bar on a 0 to 120 % scale uses the fill patterns `█ ▓ ▒ ·` and a tick at 100 %.
- Choose another ramp in the 3D model configuration (sequential ramps, a diverging ramp, grayscale). The ramps were tested for contrast and for the three kinds of colour-vision deficiency.
- Only one scalar field (for example stress along the selected member) is drawn at a time; all other entities keep their utilisation colour.

### Unverified basis

Many design values are national parameters (NDPs) or table values the repository could not yet compare with the printed DIN EN standard and its national annex. Each such value has an entry in the NDP registry with the status "unverified".

- A check that cites an unverified value can no longer end as "passes". It ends as **Unverified basis** (German: Ungeprüfte Grundlage); the utilisation is still shown. A failing check stays failing.
- The overall report does not claim compliance while any check has this status.
- In the 3D model an unverified check is drawn with the warning colour; the glyph carries a question mark and the pattern is dots.
- What to do: open the check, read the listed NDP ids in its explanation, compare the values with the printed standard and annex, and record the edition and locator in the registry. The worklist is `NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/📌️important/ndp-verification-worklist.md` in the ticket (51 conflicts between specification and repository first).
- Some catalogue formulas have not yet been moved to the registry. The completion report of each code lists those values; the code pages repeat the main ones.

### Keyboard shortcuts

All functions can be used without a pointer. Focus is never trapped. Tab order on desktop: outline, 3D model, properties, checks, diagrams (mobile: model, properties, checks, diagrams).

| Keys | Action |
|---|---|
| Alt+1 ... Alt+5 | Focus outline, 3D model, properties, checks, diagrams |
| F6 / Shift+F6 | Next / previous window |
| Arrow keys | Orbit the 3D model (up, down, left, right) |
| Alt+Arrow keys | Pan |
| + / - | Zoom in / out |
| 0 | Reset view |
| Alt+L | Toggle "scene as list" (text alternative of the 3D model) |
| ] / [ | Next / previous check |
| Esc | Clear selection (aborts a drag when one is running) |
| Shift+? | Show keyboard help (the list in the application is authoritative) |
| Ctrl+Arrow keys | Nudge the focused handle; Ctrl+Shift+Left or Right: large step |
| Tab / Shift+Tab (handle focus) | Next / previous handle of the selection |
| Arrow keys (handle focus) | Nudge by the snap increment; Shift x10, Alt x0.1 |
| digits, decimal point or comma, Backspace | Type an exact value for the handle |
| Enter | Commit |
| Ctrl+Z / Ctrl+Shift+Z (Cmd on macOS) | Undo / redo |

### Accessibility

- Text alternative: the 3D model has a written scene description and a list mode with every entity; selection in either updates both. Changes of selection or utilisation are announced in a live region ("Member B1 selected, utilisation 64 %, passes"). Announcements are coalesced.
- Every diagram has a table twin with the same data; points can be reached by keyboard.
- Status is never colour only. High-contrast theme and forced-colours are supported. Animations obey the reduced-motion setting.
- Target sizes: at least 24 px on desktop and 44 px on touch. On touch, handles also offer step buttons instead of free drag.
- Languages: English and German are always available; there is no default language, the window follows the language you select. Numbers use the decimal comma in German. Units have English and German names.

### Customisation

- Themes: Light, Dark and High contrast, selectable and shareable as plain files. A theme sets the colour ramp, the 0 to 120 % domain, material colours, outline weight (thin, bold), label density, sun and motion.
- 3D model configuration: camera and saved views, layers and solo, section planes, load glyph scale, deformation exaggeration, dimension and label visibility, unit preset (for example mm, kN, MPa).
- Persisting the 3D configuration is not complete for every code yet; windows may start with the defaults.

### Known limits that apply to all codes

- Deformed and buckled shapes are indications (half-sine, scaled), not eigenmode solutions.
- Colour along a member from per-station utilisation is not produced yet; entity-level colour is used.
- Positions of entities (placement) are not stored in the document yet; each editor arranges entities by a default rule.
- Several clause groups exist only as library functions or catalogue entries and are not reachable from the model; each code page lists them.

## Deutsch

Das Norm-Plugin prüft einen Entwurf nach den Eurocodes (EN 1990 bis EN 1999, mit dem deutschen Nationalen Anhang NA-DE) und zeigt das Ergebnis neben dem Modell. Dieser Leitfaden gilt für alle zehn Eurocode-Editoren. Jeder Eurocode hat eine eigene Seite mit abgedeckten Abschnitten, Diagrammen, Griffen und Beispielen:

| Norm | Thema | Seite |
|---|---|---|
| EN 1990 | Grundlagen der Tragwerksplanung | [en1990](🗿️artifacts/⚖️en1990/README.md) |
| EN 1991 | Einwirkungen auf Tragwerke | [en1991](🗿️artifacts/🏋️en1991/README.md) |
| EN 1992 | Betonbau | [en1992](🗿️artifacts/🏛️en1992/README.md) |
| EN 1993 | Stahlbau | [en1993](🗿️artifacts/🔩️en1993/README.md) |
| EN 1994 | Verbundbau | [en1994](🗿️artifacts/🧩️en1994/README.md) |
| EN 1995 | Holzbau | [en1995](🗿️artifacts/🪵️en1995/README.md) |
| EN 1996 | Mauerwerksbau | [en1996](🗿️artifacts/🪨️en1996/README.md) |
| EN 1997 | Geotechnik | [en1997](🗿️artifacts/🌍️en1997/README.md) |
| EN 1998 | Erdbebenbemessung | [en1998](🗿️artifacts/🫨️en1998/README.md) |
| EN 1999 | Aluminiumbau | [en1999](🗿️artifacts/🪶️en1999/README.md) |

### Zuerst lesen: Was ist geprüft

- Die visuellen Editoren und die Normbibliotheken sind geschrieben. Die Prüfberichte des Tickets `NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS` (Stand 02.11.2026) halten fest, dass sie als Ganzes noch nicht kompiliert und ausgeführt wurden. Die hier beschriebenen Funktionen sind das entworfene und implementierte Verhalten; was ein Bericht als fehlend oder zurückgestellt führt, benennt die jeweilige Seite.
- Kein normativer Wert ist bisher gegen die gedruckte Norm geprüft. Das NDP-Register enthält 520 Einträge, davon 0 geprüft. Ergebnisse, die auf einem solchen Wert beruhen, sind als ungeprüft gekennzeichnet (siehe "Ungeprüfte Grundlage"). Jedes Ergebnis ist eine Entwurfshilfe und muss von einer qualifizierten Person kontrolliert werden.

### Die Fenster

Ein Editor hat fünf Fenster. Der Betrachter (nur lesend) hat drei davon: 3D-Modell, Diagramme und Nachweise.

| Fenster | Zweck |
|---|---|
| Modellstruktur | Baum aller Objekte (Stäbe, Wände, Fundamente, Einwirkungen, ...) mit Statussymbol und höchster Ausnutzung; hinzufügen, duplizieren, entfernen, filtern |
| Eigenschaften | Typisierter Inspektor der Auswahl: Werte mit Einheiten, Auswahllisten, Objektverweise, NA-Kennzeichen, Rückmeldung je Feld |
| 3D-Modell | Tragwerksmodell mit Lastsymbolen, Lagern, Maßlinien, Normabschnitts-Plaketten, Versagenssymbolen und Ausnutzungsfarben; Griffe ziehen |
| Nachweise | Alle Nachweise nach Objekt, Abschnitt oder Teil gruppiert, mit Ausnutzungsbalken, Abschnittsverweisen und Abhilfe-Vorschlägen |
| Diagramme | 2D-Diagramme des gewählten Objekts (Knickkurven, Interaktionspolygone, Spektren, Profile, ...), jeweils mit Tabellenzwilling |

Desktop-Anordnung: links Modellstruktur über Eigenschaften (22 %), in der Mitte das 3D-Modell (50 %), rechts Nachweise über Diagrammen (28 %). Der lesende Betrachter nutzt 50 % / 25 % / 25 %. Auf dem Smartphone wird jeweils ein Fenster über eine Tab-Leiste gezeigt (Modell, Eigenschaften, Nachweise, Diagramme); auf dem Tablet zwei Bereiche. Fenster lassen sich andocken, lösen oder ausblenden; die Anordnung bleibt je Benutzer erhalten.

### Auswahl

Alle Fenster teilen eine Auswahl.

- Ein Objekt im 3D-Modell, in der Modellstruktur oder in einer Gruppenzeile der Nachweise anklicken. Rechteckauswahl und Mehrfachauswahl funktionieren im 3D-Modell. Teile (eine Fläche eines Blechs, eine Schraubenreihe) sind einzeln wählbar.
- Die Wahl einer Nachweiszeile wählt das zugehörige Objekt; die Eigenschaften springen zum geprüften Feld, die Diagramme wechseln zu diesem Objekt. Ohne Auswahl zeigen die Diagramme das ungünstigste Objekt.
- Das Überfahren einer Nachweiszeile hebt das Objekt hervor und umgekehrt.
- Bekannte Einschränkung: Die gespeicherte Nachweisauswahl und Klicks im Modell tragen keine Zeitstempel; die Fenster zeigen ihre Vereinigung. Das Aufheben eines Klicks hebt den gespeicherten Nachweis nicht auf. Der lesende Betrachter spiegelt Klicks noch nicht.

### Werte bearbeiten

- In den Eigenschaften einen Wert eingeben, eine Einheit wählen, eine Option oder einen Objektverweis auswählen. Enter oder Verlassen des Felds übernimmt; Esc verwirft; die Pfeiltasten verändern in Schritten (Umschalt x10, Alt x0,1). Beim Überfahren erscheint der SI-Wert.
- Werte außerhalb des Gültigkeitsbereichs der Formel werden mit einer Meldung abgelehnt. Felder mit einem nationalen Parameter tragen das Kennzeichen "NA-DE" und eine Rücksetzung auf den Wert des Anhangs.
- Berechnete Werte (Fläche, Querschnittsklasse, ...) sind schreibgeschützte Zeilen.
- Jedes Feld mit einem nicht erfüllten oder fast erreichten Nachweis zeigt einen kleinen Ausnutzungsbalken und, wenn der Nachweis eine Abhilfe kennt, einen Vorschlag wie "12 mm -> >= 22 mm", der den Wert mit einem Klick setzt.
- Noch nicht verfügbar: Inspektor-Stufen (Basis, Erweitert, Experte) und bedingte Sichtbarkeit, da sie auf den gemeinsamen Feldspezifikationstyp warten.

### Griffe ziehen

Am gewählten Objekt erscheinen im 3D-Modell Griffe. Ein Zug ändert ein oder mehrere Eingabefelder gleichzeitig, etwa die Spannweite eines Stabs, den Schraubenabstand, die Tiefe einer Ausklinkung oder die Geschosshöhe.

1. Einen Griff überfahren: Name und aktueller Wert erscheinen ("Griff Länge Stab B1").
2. Ziehen: Der Wert aktualisiert sich live, alle Fenster zeigen die Vorschau. Werte rasten auf ein Raster, auf Katalogwerte (Schraubendurchmesser, Profilgrößen, Lamellendicke) oder auf benachbarte Objekte ein und können die Grenzen des Felds nicht verlassen. Strg gedrückt halten schaltet das Einrasten ab; die Grenzen bleiben.
3. Loslassen übernimmt. Ein Zug ist als ein Rückgängig-Schritt entworfen. Loslassen ohne Bewegung übernimmt nichts. Esc oder Verlust des Zeigers bricht ab und hinterlässt keine Spur.
4. Während des Ziehens kann eine Zahl getippt werden (Punkt oder Komma); sie ersetzt die Mausposition und wird auf die Grenzen begrenzt.
5. An ein Feld gebundene Maßlinien und Marker in Diagrammen (zum Beispiel ein Knicklängen-Marker) funktionieren genauso. Ein Diagramm-Marker für einen abgeleiteten Wert, etwa der Beanspruchungspunkt, lässt sich nicht ziehen.

Heute übernimmt ein Zug jedes betroffene Feld als eigenen `setField`; der gebündelte Befehl `setFields` ist implementiert, aber noch nicht in jeden Editor eingebunden. Objekte per Klick mit einem Geisterbild einfügen und die Zahleneingabe direkt an der Maßlinie sind noch nicht umgesetzt.

### Live-Auswertung

- Jede Änderung wertet die Nachweise sofort neu aus; es gibt keinen eigenen "Ausführen"-Schritt. Pro Editor hält eine dauerhafte Auswertungssitzung das Ergebnis; ändert man erneut, während ein Lauf läuft, gilt die letzte Änderung.
- EN 1993 wertet pro Objekt aus (nur die Nachweise des geänderten Stabs); die anderen Normen rechnen die gesamte Faltung neu, nach Eingabe zwischengespeichert. Bei großen Modellen kann das dort langsamer wirken.
- Der Befehl "Auswerten" rechnet auf Wunsch alles neu und ist abbrechbar. Fortschritt und Abbruch gibt es als Sitzungsfunktion; eine Fortschrittsanzeige für lange Einheiten zeigt das Fenster noch nicht.
- Ist eine Auswertung ausstehend, erscheinen betroffene Objekte ausgegraut, nie ausgeblendet.

### Ausnutzung: Farben, Muster und Symbole

Die Ausnutzung ist Beanspruchung geteilt durch Beanspruchbarkeit. 100 % ist die Grenze.

| Zustand | Symbol | Muster | Bedeutung |
|---|---|---|---|
| Erfüllt | Haken | keins | unter der Warnschwelle |
| Nahe am Grenzwert | Ausrufe- bzw. Warnzeichen | waagerechte Schraffur | etwa ab 90 bis 95 % Ausnutzung (die genaue Schwelle legt der Nachweis fest) |
| Nicht erfüllt | Kreuz | diagonale Schraffur | über 100 % |
| Nicht anwendbar | leere Menge | keins | Nachweis gilt für diese Eingabe nicht |
| Ungeprüfte Grundlage | Fragezeichen am Symbol | Punkte | beruht auf einem ungeprüften nationalen Wert |

- Die Standardfarbskala ist wahrnehmungsgleichmäßig von 0 bis 100 % mit deutlichem Bruch bei 100 %; die Skala reicht bis 120 %.
- Farbe ist nie das einzige Merkmal: Symbol, Schraffur und Text sind immer vorhanden. Im Nachweisfenster nutzt ein Balken mit 12 Zellen auf einer Skala von 0 bis 120 % die Füllmuster `█ ▓ ▒ ·` und eine Marke bei 100 %.
- Eine andere Farbskala wählen Sie in der Konfiguration des 3D-Modells (sequenzielle Skalen, divergierende Skala, Graustufen). Die Skalen wurden auf Kontrast und auf die drei Arten der Farbfehlsichtigkeit geprüft.
- Es wird immer nur ein Skalarfeld (zum Beispiel die Spannung entlang des gewählten Stabs) gezeichnet; alle anderen Objekte behalten die Ausnutzungsfarbe.

### Ungeprüfte Grundlage

Viele Bemessungswerte sind nationale Parameter (NDP) oder Tabellenwerte, die das Repository noch nicht mit der gedruckten DIN EN und ihrem Nationalen Anhang vergleichen konnte. Jeder solche Wert hat im NDP-Register einen Eintrag mit dem Status "ungeprüft".

- Ein Nachweis, der einen ungeprüften Wert nennt, kann nicht mehr als "erfüllt" enden. Er endet als **Ungeprüfte Grundlage** (englisch: Unverified basis); die Ausnutzung wird weiter angezeigt. Ein nicht erfüllter Nachweis bleibt nicht erfüllt.
- Der Gesamtbericht behauptet keine Einhaltung, solange ein Nachweis diesen Status hat.
- Im 3D-Modell wird ein ungeprüfter Nachweis in der Warnfarbe gezeichnet; das Symbol trägt ein Fragezeichen, das Muster sind Punkte.
- Vorgehen: den Nachweis öffnen, die genannten NDP-Kennungen in der Erläuterung lesen, die Werte mit der gedruckten Norm und dem Anhang vergleichen und Ausgabe und Fundstelle im Register eintragen. Die Arbeitsliste ist `NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/📌️important/ndp-verification-worklist.md` im Ticket (zuerst die 51 Widersprüche zwischen Spezifikation und Repository).
- Einige Katalogformeln sind noch nicht ins Register übertragen. Der Abschlussbericht jeder Norm führt diese Werte auf; die Normseiten wiederholen die wichtigsten.

### Tastenkürzel

Alle Funktionen sind ohne Zeigegerät bedienbar. Der Fokus wird nie gefangen. Tab-Reihenfolge am Desktop: Modellstruktur, 3D-Modell, Eigenschaften, Nachweise, Diagramme (Smartphone: Modell, Eigenschaften, Nachweise, Diagramme).

| Tasten | Aktion |
|---|---|
| Alt+1 ... Alt+5 | Fokus auf Modellstruktur, 3D-Modell, Eigenschaften, Nachweise, Diagramme |
| F6 / Umschalt+F6 | Nächstes / vorheriges Fenster |
| Pfeiltasten | 3D-Modell orbitieren (oben, unten, links, rechts) |
| Alt+Pfeiltasten | Verschieben |
| + / - | Vergrößern / Verkleinern |
| 0 | Ansicht zurücksetzen |
| Alt+L | "Szene als Liste" umschalten (Textalternative zum 3D-Modell) |
| ] / [ | Nächster / vorheriger Nachweis |
| Esc | Auswahl aufheben (bricht einen laufenden Zug ab) |
| Umschalt+? | Tastaturhilfe anzeigen (maßgeblich ist die Liste in der Anwendung) |
| Strg+Pfeiltasten | Fokussierten Griff schieben; Strg+Umschalt+Links oder Rechts: großer Schritt |
| Tab / Umschalt+Tab (Griff im Fokus) | Nächster / vorheriger Griff der Auswahl |
| Pfeiltasten (Griff im Fokus) | Um die Rasterweite schieben; Umschalt x10, Alt x0,1 |
| Ziffern, Punkt oder Komma, Rücktaste | Exakten Wert für den Griff tippen |
| Enter | Übernehmen |
| Strg+Z / Strg+Umschalt+Z (Cmd am Mac) | Rückgängig / Wiederholen |

### Barrierefreiheit

- Textalternative: Das 3D-Modell hat eine geschriebene Szenenbeschreibung und einen Listenmodus mit jedem Objekt; die Auswahl in einem von beiden aktualisiert beide. Änderungen von Auswahl oder Ausnutzung werden in einem Live-Bereich angesagt ("Stab B1 gewählt, Ausnutzung 64 %, erfüllt"). Ansagen werden zusammengefasst.
- Jedes Diagramm hat einen Tabellenzwilling mit denselben Daten; Punkte sind per Tastatur erreichbar.
- Der Status ist nie nur Farbe. Ein Thema mit hohem Kontrast und der Modus erzwungener Farben werden unterstützt. Animationen beachten die Einstellung für reduzierte Bewegung.
- Zielgrößen: mindestens 24 px am Desktop und 44 px bei Touch. Bei Touch bieten Griffe zusätzlich Schrittschalter statt freiem Ziehen.
- Sprachen: Deutsch und Englisch sind immer verfügbar; es gibt keine Standardsprache, das Fenster folgt der gewählten Sprache. Zahlen nutzen im Deutschen das Dezimalkomma. Einheiten haben deutsche und englische Namen.

### Anpassung

- Themen: Hell, Dunkel und Hoher Kontrast, wählbar und als einfache Dateien teilbar. Ein Thema legt Farbskala, den Bereich 0 bis 120 %, Materialfarben, Linienstärke (dünn, kräftig), Beschriftungsdichte, Sonne und Bewegung fest.
- Konfiguration des 3D-Modells: Kamera und gespeicherte Ansichten, Ebenen und Einzelansicht, Schnittebenen, Lastsymbol-Maßstab, Überhöhung der Verformung, Sichtbarkeit von Maßen und Beschriftungen, Einheitenvorgabe (zum Beispiel mm, kN, MPa).
- Das Speichern der 3D-Konfiguration ist noch nicht für jede Norm vollständig; Fenster starten gegebenenfalls mit den Standardwerten.

### Bekannte Einschränkungen für alle Normen

- Verformte und ausgeknickte Formen sind Andeutungen (Halbsinus, skaliert), keine Eigenformen.
- Eine Färbung entlang des Stabs aus Ausnutzungen je Stelle gibt es noch nicht; es wird die Farbe je Objekt genutzt.
- Lagen von Objekten (Platzierung) werden noch nicht im Dokument gespeichert; jeder Editor ordnet Objekte nach einer Standardregel an.
- Mehrere Abschnittsgruppen existieren nur als Bibliotheksfunktionen oder Katalogeinträge und sind aus dem Modell nicht erreichbar; jede Normseite führt sie auf.
