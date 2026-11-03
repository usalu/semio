---
name: en1990
kind: user
---

# EN 1990 Basis of design

Shared guide: [norm](../../README.md) (windows, selection, handles, shortcuts, utilisation colours, unverified basis, accessibility).

## English

### Status

Written, not yet compiled or run as a whole (audits of 2026-11-02). Nothing on this page is verified against the printed standard; the unverified values are listed below.

### What is covered

- Combinations of actions: Eq. 6.10, 6.10a/6.10b (EN annex only; NA-DE offers 6.10), 6.11b accidental, 6.12b seismic, 6.14b to 6.16b serviceability; leading and accompanying actions, mutually exclusive groups in the generator, favourable variables dropped, K_FI on unfavourable actions only; EQU, STR, GEO sets A, B and C.
- Partial and combination factors from Tables A1.1, A2.1 to A2.3 (snow split at 1000 m for both annexes).
- Snapshot checks: STR, GEO, EQU, accidental 6.11b, seismic 6.12b, serviceability against C_d, A1.4 deflection (w1 + w2 + w3 - w_c, w3), vibration, fatigue range, K_FI by reliability class, reliability versus consequence class, Table 2.1, Annex B levels, Annex C beta with P_f, bridge serviceability (A2).
- Library (catalogue E90.01 to E90.30, reachable by function, not from the model): reliability mathematics (FORM, Annex C alphas, Gumbel/lognormal/normal/Weibull), S-N endurance and Palmgren-Miner, Annex D design from tests (k_n).

### Not covered or not reachable from the model

- Fatigue spectra, reliability analysis inputs and test series are not model entities, so the Annex D marker, a real Miner sum and a converged FORM run are not shown; the FORM diagram draws the Annex C.7 design point.
- Groups of exclusive actions exist in the generator but the model has no group field.
- Selecting the equation (6.10 or 6.10a/b) and several leaf kinds (structure kind, declared K_FI, rows of actions and members) are not yet editable.

### Unverified values (NA-DE and others)

A2.4(A) gamma_G 1.05/0.95 and gamma_Q; rail gamma_Q 1.45; A2.4(C) gamma_Q 1.15/1.25/1.30; all A2 psi rows (road gr1a to gr5, footbridge, rail, thermal 0.8 in DE); "6.10 only" and the simplified bound 1.35 in NA-DE; A1.4 ratios (L/250, L/300, H/500, h/300, 3 Hz); footbridge accelerations 0.7/0.2/0.4/0.1; rail acceleration, twist table, L/600; gamma_M timber 1.3 and gamma_M other; the altitude basis of the snow split (NN + 1000 m). Checks citing them end as "Unverified basis".

### 3D views

- Members left to right with supports, span dimension, clause badge and failure glyphs (yielding, overturning, sliding, fatigue).
- Serviceability deflected shape w_net,fin as an exaggerated overlay (automatic, up to x200) with the stacked dimension lines w1, w2, w3, w_c and the L/n limit; first-mode ghost under the minimum frequency.
- Bridge deck over each bridge serviceability entity with deflection, vibration ghost and labels for acceleration, twist and deflection coloured by their checks.
- Action glyphs (line, point, impact) on every member they affect; glyph size is gamma x psi x action of the active combination; an action outside the active combination stays as a ghost at its characteristic value. Badges show consequence class, reliability class with K_FI, beta against the Table B2 target, annex and provisional psi rows.
- A combination selector entity picks the active combination; selecting a check selects its combination.

### Diagrams (each with a table twin)

Combination matrix (gamma x psi, leading action hatched, active column framed), psi factors per category (dotted when unverified), reliability (P_f over beta, RC1 to RC3 targets, draggable beta marker), FORM (reliability circle, design point), Annex D k_n, fatigue S-N with Palmgren-Miner for 2 x 10^6 cycles.

### Editable handles

Member span, pre-camber, magnitude of every action (gk, qk, ad, aEk), combination selector (stepper), the beta marker in the reliability diagram and the range-ratio marker in the fatigue diagram. Dimension lines w1, w2, w3 are bound to their fields. Changing the active combination by dragging is not routed yet; it follows the selected check and the matrix markers.

### Worked examples

Shipped example documents: default office, high-consequence office, accidental-seismic (compliant and failing), fatigue (compliant and failing), road bridge (compliant and failing).

1. Open "road bridge failing". The checks window lists failing rows with a cross; select the worst row. The viewport frames the bridge, the diagrams switch to the matrix of its governing combination.
2. Drag the span handle shorter. The deflection overlay, w-dimensions and the utilisation bars update on release without any run command.
3. Open "high-consequence office": K_FI and the beta target change with the reliability class; compare the beta marker with the RC target line.

Reference values from the specification (test expectations, not yet run): specification examples E90-01 to E90-09 and E90-15 are encoded in the kernel tests; 56 catalogue cases and 14 documents are compared with an independent Python oracle (scipy for Phi and FORM).

## Deutsch

### Stand

Geschrieben, als Ganzes noch nicht kompiliert oder ausgeführt (Berichte vom 02.11.2026). Nichts auf dieser Seite ist gegen die gedruckte Norm geprüft; die ungeprüften Werte stehen unten.

### Was abgedeckt ist

- Einwirkungskombinationen: Gl. 6.10, 6.10a/6.10b (nur EN-Anhang; NA-DE kennt 6.10), 6.11b außergewöhnlich, 6.12b Erdbeben, 6.14b bis 6.16b Gebrauchstauglichkeit; führende und begleitende Einwirkungen, sich gegenseitig ausschließende Gruppen im Generator, günstige veränderliche Einwirkungen entfallen, K_FI nur auf ungünstige Einwirkungen; EQU, STR, GEO mit den Sätzen A, B und C.
- Teil- und Kombinationsbeiwerte aus den Tabellen A1.1, A2.1 bis A2.3 (Schnee-Trennung bei 1000 m in beiden Anhängen).
- Nachweise am Modell: STR, GEO, EQU, außergewöhnlich 6.11b, Erdbeben 6.12b, Gebrauchstauglichkeit gegen C_d, Durchbiegung A1.4 (w1 + w2 + w3 - w_c, w3), Schwingungen, Ermüdungsschwingbreite, K_FI nach Zuverlässigkeitsklasse, Zuverlässigkeits- gegen Schadensfolgeklasse, Tabelle 2.1, Anhang-B-Stufen, Anhang C Beta mit P_f, Brückengebrauchstauglichkeit (A2).
- Bibliothek (Katalog E90.01 bis E90.30, per Funktion erreichbar, nicht aus dem Modell): Zuverlässigkeitsmathematik (FORM, Alphas aus Anhang C, Gumbel/Lognormal/Normal/Weibull), S-N-Dauerfestigkeit und Palmgren-Miner, Anhang D Bemessung durch Versuche (k_n).

### Nicht abgedeckt oder nicht aus dem Modell erreichbar

- Ermüdungskollektive, Eingaben zur Zuverlässigkeitsanalyse und Versuchsreihen sind keine Modellobjekte; daher fehlen der Marker aus Anhang D, eine echte Miner-Summe und ein konvergierter FORM-Lauf; das FORM-Diagramm zeichnet den Bemessungspunkt nach Anhang C.7.
- Gruppen sich ausschließender Einwirkungen gibt es im Generator, aber das Modell hat kein Gruppenfeld.
- Die Wahl der Gleichung (6.10 oder 6.10a/b) und mehrere Blätter (Tragwerksart, angegebenes K_FI, Zeilen von Einwirkungen und Stäben) sind noch nicht editierbar.

### Ungeprüfte Werte (NA-DE und andere)

A2.4(A) gamma_G 1,05/0,95 und gamma_Q; Bahn gamma_Q 1,45; A2.4(C) gamma_Q 1,15/1,25/1,30; alle A2-psi-Zeilen (Straße gr1a bis gr5, Fußgängerbrücke, Bahn, Temperatur 0,8 in DE); "nur 6.10" und die vereinfachte Schranke 1,35 im NA-DE; A1.4-Verhältnisse (L/250, L/300, H/500, h/300, 3 Hz); Beschleunigungen Fußgängerbrücke 0,7/0,2/0,4/0,1; Beschleunigung Bahn, Verwindungstabelle, L/600; gamma_M Holz 1,3 und gamma_M sonstige; die Höhenbasis der Schneetrennung (NN + 1000 m). Nachweise, die sie nennen, enden als "Ungeprüfte Grundlage".

### 3D-Ansichten

- Stäbe von links nach rechts mit Lagern, Spannweitenmaß, Abschnittsplakette und Versagenssymbolen (Fließen, Kippen, Gleiten, Ermüdung).
- Verformte Gebrauchslage w_net,fin als überhöhte Überlagerung (automatisch, bis x200) mit den gestapelten Maßlinien w1, w2, w3, w_c und der L/n-Grenze; Geisterbild der ersten Eigenform unterhalb der Mindestfrequenz.
- Brückenplatte über jedem Brücken-Gebrauchstauglichkeitsobjekt mit Durchbiegung, Schwingungsgeist und Beschriftungen für Beschleunigung, Verwindung und Durchbiegung, gefärbt nach ihren Nachweisen.
- Lastsymbole (Linie, Punkt, Anprall) an jedem betroffenen Stab; die Größe ist gamma x psi x Einwirkung der aktiven Kombination; eine Einwirkung außerhalb der aktiven Kombination bleibt als Geisterbild mit dem charakteristischen Wert. Plaketten zeigen Schadensfolgeklasse, Zuverlässigkeitsklasse mit K_FI, Beta gegen den Zielwert aus Tabelle B2, Anhang und vorläufige psi-Zeilen.
- Ein Kombinationswähler bestimmt die aktive Kombination; die Wahl eines Nachweises wählt seine Kombination.

### Diagramme (jeweils mit Tabellenzwilling)

Kombinationsmatrix (gamma x psi, führende Einwirkung schraffiert, aktive Spalte umrahmt), psi-Beiwerte je Kategorie (gepunktet, wenn ungeprüft), Zuverlässigkeit (P_f über Beta, Ziele RC1 bis RC3, ziehbarer Beta-Marker), FORM (Zuverlässigkeitskreis, Bemessungspunkt), Anhang D k_n, Ermüdung S-N mit Palmgren-Miner für 2 x 10^6 Zyklen.

### Editierbare Griffe

Stabspannweite, Überhöhung, Größe jeder Einwirkung (gk, qk, ad, aEk), Kombinationswähler (Schrittschalter), der Beta-Marker im Zuverlässigkeitsdiagramm und der Schwingbreiten-Marker im Ermüdungsdiagramm. Die Maßlinien w1, w2, w3 sind an ihre Felder gebunden. Das Ändern der aktiven Kombination durch Ziehen ist noch nicht verdrahtet; sie folgt dem gewählten Nachweis und den Matrix-Markern.

### Durchgerechnete Beispiele

Mitgelieferte Beispiele: Standardbüro, Büro mit hoher Schadensfolge, außergewöhnlich-seismisch (erfüllt und nicht erfüllt), Ermüdung (erfüllt und nicht erfüllt), Straßenbrücke (erfüllt und nicht erfüllt).

1. "Straßenbrücke nicht erfüllt" öffnen. Die Nachweise listen nicht erfüllte Zeilen mit Kreuz; die ungünstigste Zeile wählen. Das Modell rahmt die Brücke, die Diagramme wechseln zur Matrix ihrer maßgebenden Kombination.
2. Den Spannweitengriff kürzer ziehen. Durchbiegungsüberlagerung, w-Maße und Ausnutzungsbalken aktualisieren sich beim Loslassen ohne Ausführen-Befehl.
3. "Büro mit hoher Schadensfolge" öffnen: K_FI und das Beta-Ziel ändern sich mit der Zuverlässigkeitsklasse; den Beta-Marker mit der RC-Ziellinie vergleichen.

Referenzwerte aus der Spezifikation (Testerwartungen, noch nicht ausgeführt): Die Spezifikationsbeispiele E90-01 bis E90-09 und E90-15 sind in den Kerntests kodiert; 56 Katalogfälle und 14 Dokumente werden mit einem unabhängigen Python-Orakel verglichen (scipy für Phi und FORM).
