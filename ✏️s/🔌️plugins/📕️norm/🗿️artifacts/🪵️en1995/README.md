---
name: en1995
kind: user
---

# EN 1995 Timber

Shared guide: [norm](../../README.md).

## English

### Status

Written, not yet compiled or run (audits of 2026-11-02). The kernel is reachable through its catalogue function; the editor cannot yet create catalogue cases.

### What is covered

Catalogue T-01 to T-124: strength tables (EN 338, EN 14080, LVL, CLT lamellae), k_mod, gamma_M per product (EN and NA-DE), k_def, k_h/k_l/k_vol; tension, compression (k_c,90 from Table 6.1.5), bending, shear with notches (6.60 to 6.62), torsion, combined stresses, net section; stability (k_c, k_crit, lateral restraints); varying sections; serviceability (w_fin = wG(1 + k_def) + wQ(1 + psi2 k_def), slip, floor vibration f1 and velocity response); connections (Johansen modes a to m, rope effect with caps, spacing Tables 8.2/8.4/8.5, axial loads, splitting, block shear, steel plates, split rings, toothed and nail plates); components (gamma method, wall methods A/B, diaphragms, CLT); fire (charring, reduced cross-section, zero-strength layer); bridges (Miner fatigue, pedestrian comfort). The member checks check every ultimate combination per effect.

### Not covered

Catalogue cases are document entities (`cases`: catalogue entry `T-nn`, numeric and text inputs, all editable, drawn in 3D with draggable dimensions, spacings and char depth); the ULS equation (`ulsEquation`: 6.10 or 6.10a/6.10b with xi = 0.85) is selectable; M_crit is always computed from the 6.31 eigenvalue (the `mCritNm` override is gone). Tapered, curved and pitched-cambered beams, CLT layers, holes, trusses and diaphragms are catalogue entries (T-30 to T-46, T-97, T-94) available as cases but without their own member model. Not representable: tapered, curved and pitched-cambered beams, CLT layers, notches at both ends, split rings and shear plates in the scene, trusses, diaphragms, shear walls, footbridge deck, connection-to-member reference, a2 (the table minimum is drawn). The floor model assumes a square panel and a 22 mm deck.

### Unverified values

Strength rows with `verified: false` are all except C16, C24, C30, GL24h, GL28h, GL32h (C14, C18 to C22, C27, C35 to C50, all D classes, other GL, LVL32, CLT100 lamella); k_mod rows for CLT, particleboard, hardboard, MDF; k_def rows for CLT, particleboard, fibreboard; gamma_M CLT 1.25 (DE); NA-DE a = 0.5 mm/kN and f1 of at least 6 Hz, velocity b = 100; wnet,fin limit L/200 (DE); hole limit d at most 0.15 h; bow limits L/300 and L/500; T-33 and T-46 coefficients; screw and nail withdrawal parameters; split ring, shear plate, toothed plate and nail plate factors; Annex C eta values; Method A/B factors; CLT gamma-method parameters; the q table; fire hardwood interpolation and panel factors.

### 3D views

Member as extruded rectangle (glulam shows lamella lines); with a fire duration above zero the residual section plus charred and zero-strength-layer hulls; supports, bearing pads bound to the c90 check, lateral restraints, end-notch void bound to 6.5.2 or shear; line and point load glyphs; vibration mode with f1 or deformed shape when the serviceability ratio is at least 0.5; dimensions bound to span, width and depth. Connections: stacked timber boxes and steel plates, fastener grid (nail, screw, bolt with 1 mm clearance, dowel), translucent spacing zones a1, a2, a3, a4 each coloured by its own check, governing Johansen mode label.

### Diagrams (table twins)

k_mod over duration and service class; k_c and k_crit curves with y/z markers (drag the buckling lengths); deflection components against L limits; char depth over fire duration (marker drags the duration); floor f1 and velocity response over span; footbridge f1; embedment strength over diameter; Johansen failure-mode bars; required against provided spacings (markers drag the three distances).

### Editable handles

Span (halved display for two spans), width, depth (40 mm lamella snap for glulam), bearing length, lateral restraint spacing, notch depth and distance, fire duration; connection a1, a3, a4, number, rows, diameter, t1, t2 or plate thickness, shear planes.

### Worked examples

Shipped examples: glulam floor beam, glulam footbridge, multi-fail timber, overloaded footbridge.

1. Open "glulam floor beam", drag the depth handle (steps of 40 mm). The k_c/k_crit marker, the deflection components and the vibration check update.
2. Open "overloaded footbridge": the comfort check fails; drag the span handle shorter and watch f1 rise in the footbridge diagram.
3. In "multi-fail timber" select a connection and drag a1; the a1 zone changes colour with its spacing check.

Reference values (specification, tests not yet run): worked example E2 gives k_c = 0.28457 and N_Rd = 36.77 kN; example E6 gives 7.532 kN and 4.635 kN; 18 cases (30 values or more) are compared with an independent oracle to 1 %.

## Deutsch

### Stand

Geschrieben, noch nicht kompiliert oder ausgeführt (Berichte vom 02.11.2026). Der Kern ist über die Katalogfunktion erreichbar; der Editor kann noch keine Katalogfälle anlegen.

### Was abgedeckt ist

Katalog T-01 bis T-124: Festigkeitstabellen (EN 338, EN 14080, Furnierschichtholz, BSP-Lamellen), k_mod, gamma_M je Produkt (EN und NA-DE), k_def, k_h/k_l/k_vol; Zug, Druck (k_c,90 nach Tabelle 6.1.5), Biegung, Schub mit Ausklinkungen (6.60 bis 6.62), Torsion, kombinierte Beanspruchung, Nettoquerschnitt; Stabilität (k_c, k_crit, seitliche Halterungen); veränderliche Querschnitte; Gebrauchstauglichkeit (w_fin = wG(1 + k_def) + wQ(1 + psi2 k_def), Schlupf, Deckenschwingung f1 und Schwinggeschwindigkeit); Verbindungen (Johansen-Brucharten a bis m, Einhängeeffekt mit Obergrenzen, Abstände nach Tabellen 8.2/8.4/8.5, Axiallast, Spalten, Blockversagen, Stahlbleche, Ringdübel, Nagelplatten und Zahnplatten); Bauteile (Gamma-Verfahren, Wandverfahren A/B, Scheiben, BSP); Brand (Abbrand, reduzierter Querschnitt, Schicht ohne Festigkeit); Brücken (Miner-Ermüdung, Fußgängerkomfort). Die Bauteilnachweise prüfen jede Grenzzustandskombination je Beanspruchung.

### Nicht abgedeckt

Katalogfälle sind Dokumentobjekte (`cases`: Katalogeintrag `T-nn`, Zahlen- und Texteingaben, alle editierbar, in 3D mit ziehbaren Maßen, Abständen und Abbrandtiefe); die GZT-Gleichung (`ulsEquation`: 6.10 oder 6.10a/6.10b mit xi = 0,85) ist wählbar; M_crit wird immer aus dem Eigenwert nach 6.31 berechnet (die Überschreibung `mCritNm` entfällt). Verjüngte, gekrümmte und Satteldachträger, BSP-Lagen, Löcher, Fachwerke und Scheiben sind Katalogeinträge (T-30 bis T-46, T-97, T-94) und als Fälle verfügbar, haben aber kein eigenes Bauteilmodell. Nicht darstellbar: verjüngte, gekrümmte und Satteldachträger mit Überhöhung, BSP-Lagen, Ausklinkungen an beiden Enden, Ringdübel und Scheibendübel in der Szene, Fachwerke, Scheiben, Schubwände, Fußgängerbrückendeck, Bezug Verbindung-Bauteil, a2 (das Tabellenminimum wird gezeichnet). Das Deckenmodell nimmt ein quadratisches Feld und 22 mm Beplankung an.

### Ungeprüfte Werte

Festigkeitszeilen mit `verified: false` sind alle außer C16, C24, C30, GL24h, GL28h, GL32h (C14, C18 bis C22, C27, C35 bis C50, alle D-Klassen, übrige GL, LVL32, BSP100-Lamelle); k_mod-Zeilen für BSP, Spanplatte, Hartfaserplatte, MDF; k_def-Zeilen für BSP, Spanplatte, Faserplatte; gamma_M BSP 1,25 (DE); NA-DE a = 0,5 mm/kN und f1 mindestens 6 Hz, Geschwindigkeit b = 100; Grenze wnet,fin L/200 (DE); Lochgrenze d höchstens 0,15 h; Vorkrümmungsgrenzen L/300 und L/500; Beiwerte von T-33 und T-46; Auszugsparameter für Schrauben und Nägel; Faktoren für Ringdübel, Scheibendübel, Zahn- und Nagelplatten; eta-Werte in Anhang C; Faktoren Verfahren A/B; Parameter des Gamma-Verfahrens für BSP; die q-Tabelle; Interpolation für Laubholz im Brandfall und Plattenfaktoren.

### 3D-Ansichten

Stab als extrudiertes Rechteck (Brettschichtholz zeigt Lamellenlinien); bei Branddauer größer null Restquerschnitt sowie Hüllen für verkohlte Schicht und Schicht ohne Festigkeit; Lager, Auflagerplatten gebunden an den c90-Nachweis, seitliche Halterungen, Ausklinkungsraum gebunden an 6.5.2 oder Schub; Linien- und Punktlastsymbole; Schwingungsform mit f1 oder verformte Lage, wenn das Gebrauchsverhältnis mindestens 0,5 beträgt; an Spannweite, Breite und Höhe gebundene Maße. Verbindungen: gestapelte Holzquader und Stahlbleche, Verbindungsmittelfeld (Nagel, Schraube, Bolzen mit 1 mm Spiel, Stabdübel), durchscheinende Abstandszonen a1, a2, a3, a4, jeweils nach ihrem eigenen Nachweis gefärbt, Beschriftung der maßgebenden Johansen-Bruchart.

### Diagramme (Tabellenzwillinge)

k_mod über Dauer und Nutzungsklasse; Kurven k_c und k_crit mit y/z-Markern (ziehen die Knicklängen); Durchbiegungsanteile gegen L-Grenzen; Abbrandtiefe über der Branddauer (Marker zieht die Dauer); Decken-f1 und Schwinggeschwindigkeit über der Spannweite; Fußgängerbrücken-f1; Lochleibungsfestigkeit über dem Durchmesser; Johansen-Bruchartbalken; erforderliche gegen vorhandene Abstände (Marker ziehen die drei Abstände).

### Editierbare Griffe

Spannweite (bei zwei Feldern halbiert dargestellt), Breite, Höhe (40-mm-Lamellenraster bei Brettschichtholz), Auflagerlänge, Abstand seitlicher Halterungen, Ausklinkungstiefe und -abstand, Branddauer; Verbindung a1, a3, a4, Anzahl, Reihen, Durchmesser, t1, t2 oder Blechdicke, Scherflächen.

### Durchgerechnete Beispiele

Mitgelieferte Beispiele: Brettschichtholz-Deckenträger, Brettschichtholz-Fußgängerbrücke, Holz mit mehreren Fehlern, überlastete Fußgängerbrücke.

1. "Brettschichtholz-Deckenträger" öffnen und den Höhengriff ziehen (Schritte von 40 mm). Marker k_c/k_crit, Durchbiegungsanteile und Schwingungsnachweis aktualisieren sich.
2. "Überlastete Fußgängerbrücke" öffnen: Der Komfortnachweis schlägt fehl; den Spannweitengriff kürzer ziehen und f1 im Brückendiagramm steigen sehen.
3. In "Holz mit mehreren Fehlern" eine Verbindung wählen und a1 ziehen; die a1-Zone ändert ihre Farbe mit dem Abstandsnachweis.

Referenzwerte (Spezifikation, Tests noch nicht ausgeführt): Das Beispiel E2 ergibt k_c = 0,28457 und N_Rd = 36,77 kN; Beispiel E6 ergibt 7,532 kN und 4,635 kN; 18 Fälle (mindestens 30 Werte) werden mit einem unabhängigen Orakel auf 1 % verglichen.
