---
name: en1993
kind: user
---

# EN 1993 Steel

Shared guide: [norm](../../README.md). EN 1993 is the reference editor: it is the only one that re-evaluates per entity.

## English

### Status

Written, not yet compiled or run as a whole (audits of 2026-11-02). The pure clause library passed 17 and 19 standalone tests (E01 to E15 and the second pass); the integrated crate has not been built.

### What is covered

In the model checks: tension, compression, bending about both axes, shear in both directions, torsion (St. Venant share), bending with shear and axial force, class 1 to 4 with effective properties, flexural and lateral-torsional buckling, interaction by Annex A and Annex B, shear buckling of unstiffened webs, fire (EN 1993-1-2: steel temperature with temperature-dependent specific heat, k_y, chi_fi), bolts (shear, bearing, spacings of Table 3.3, shear-tension interaction), welds (directional method), fatigue (EN 1993-1-9, S-N with cut-off, Miner), plated panels, silo shells (EN 1993-1-6, Annex D), piles, tension components, cold-formed members, towers and crane runways (simplified checks). Every ultimate row of a member is checked, not only the governing one.

Library level (functions with tests, not all wired into the model): global analysis (alpha_cr, amplification, sway imperfection, bracing), warping torsion, M_cr for arbitrary moment diagrams, member method 6.3.4, component method of joints (T-stub, stiffness, M_j,Rd), hollow-section joints, shear lag, patch loading, reduced stress method, cold-formed and stainless rules (1-3, 1-4), cables (1-11), sheet piles, fatigue shear and rainflow.

### Not covered

Engine wiring still missing for frame and storey checks, parametric sections, geometric joint entities, supports and restraints for M_cr, built-up members; 1-8 component method as model checks; hollow-section joints; 1-3 effective sections with edge stiffeners and purlins; 1-4 stainless; 1-5 shear lag, stiffened plates; 1-6 shells other than the cylinder; 3-1/3-2, 1-11, 5, 2, 6 beyond the simplified checks. The serviceability check still uses a uniform-load equivalent. The brittle-fracture table (1-10 Table 2.1) is not encoded; temperature is fixed at 0 degrees C in the model check. The class-4 delta-M shift is zero in the engine.

### Unverified values

Table 6.2: hot-finished hollow S420 curve a, a0 from 460 MPa; CHS M_N,Rd exponent 1.7; Table B.3 C_m rows for transverse loading; Annex A factors as transcribed; k_c values 0.94/0.86; EN 1993-1-2 Table 3.1 k_p column and k_sh = 1; E_s for stainless; Spangler formula; 1-7 plate coefficient; fatigue shear cut-off 0.457 Delta tau_C; NA-DE k_b and lambda_c0 = 0.5 are not implemented. The specification values 0.4552 and 0.6442 for Eq. 6.61/6.62 were replaced by 0.4565 and 0.6418 (gamma_M1 inside n, as the standard requires). No third-party section-property library was available, so section constants are not cross-checked by one.

### 3D views

Members extruded from the catalogue section with end supports, lateral restraints, length dimension, clause badge, buckling mode overlay and failure glyphs; bolted and welded joints as end plate, bolt grid, fillet weld and a selectable group; plated panels; silo shells with the Janssen pressure zone; tension components; cold-formed members; piles with pile heads; design-action glyphs; wind on tower legs; wheel loads on crane runways; fire thermal glyph; fatigue groups with labels. Stiffeners, section tapers and weld symbols are not drawn; the cable diameter is a nominal glyph size.

### Diagrams (table twins)

Buckling curves with y and z markers bound to the buckling lengths; lateral-torsional chi_LT and M_b,Rd over the restraint spacing (draggable marker); M-N polygon with demand and utilisation ray; classification stress block; S-N (log-log) with spectrum and damage; fire temperature and k_theta curves.

### Editable handles

Member length, restraint spacing, catalogue section height; bolt pitch, gauge, end and edge distance, row count and per-row count; weld length and throat; panel a, b, t; silo radius, depth, t; pile embedded length. A numeric section drag picks a catalogue section (inserting it when missing).

### Worked examples

Shipped examples: HEB 240 compliant, high-strength connection.

1. Open "HEB 240 compliant", select the member. Drag the length handle up: the utilisation colour moves toward the warning ramp, the buckling diagram marker moves, and the checks window shows the changed rows only (per-entity re-evaluation).
2. Drag the restraint-spacing handle: the chi_LT diagram marker follows; shorten it to recover capacity.
3. Open "high-strength connection": drag the bolt pitch below the Table 3.3 minimum; the spacing check turns red and the chip offers the minimum distance.

Reference values (oracle and standalone tests): Eq. 6.61 = 0.4565 and Eq. 6.62 = 0.6418 for the NA-DE example E06; steel temperature E12 within 3 K of the specification; M_cr by Hermite finite elements agrees with scipy within 0.5 %.

## Deutsch

### Stand

Geschrieben, als Ganzes noch nicht kompiliert oder ausgeführt (Berichte vom 02.11.2026). Die reine Abschnittsbibliothek bestand 17 und 19 eigenständige Tests (E01 bis E15 und zweiter Durchgang); das integrierte Paket wurde nicht gebaut.

### Was abgedeckt ist

In den Modellnachweisen: Zug, Druck, Biegung um beide Achsen, Querkraft in beiden Richtungen, Torsion (St.-Venant-Anteil), Biegung mit Querkraft und Normalkraft, Klassen 1 bis 4 mit wirksamen Querschnittswerten, Biegeknicken und Biegedrillknicken, Interaktion nach Anhang A und Anhang B, Schubbeulen unausgesteifter Stege, Brand (EN 1993-1-2: Stahltemperatur mit temperaturabhängiger spezifischer Wärme, k_y, chi_fi), Schrauben (Abscheren, Lochleibung, Abstände nach Tabelle 3.3, Interaktion Abscheren-Zug), Schweißnähte (richtungsbezogenes Verfahren), Ermüdung (EN 1993-1-9, S-N mit Abschneidegrenze, Miner), Plattenfelder, Silokonstruktionen (EN 1993-1-6, Anhang D), Pfähle, Zugglieder, kaltgeformte Bauteile, Türme und Kranbahnen (vereinfachte Nachweise). Es wird jede Grenzzustandszeile eines Stabs geprüft, nicht nur die maßgebende.

Bibliotheksebene (Funktionen mit Tests, nicht alle im Modell verdrahtet): Gesamtberechnung (alpha_cr, Vergrößerung, Schiefstellung, Aussteifung), Wölbkrafttorsion, M_cr für beliebige Momentenverläufe, Bauteilverfahren 6.3.4, Komponentenverfahren für Anschlüsse (T-Stummel, Steifigkeit, M_j,Rd), Hohlprofilknoten, Schubverzerrung, Lasteinleitung, Verfahren der reduzierten Spannungen, kaltgeformte und nichtrostende Regeln (1-3, 1-4), Seile (1-11), Spundwände, Ermüdung bei Schub und Rainflow.

### Nicht abgedeckt

Noch nicht verdrahtet: Rahmen- und Geschossnachweise, parametrische Querschnitte, geometrische Anschlussobjekte, Lager und Halterungen für M_cr, zusammengesetzte Stäbe; das Komponentenverfahren aus 1-8 als Modellnachweis; Hohlprofilknoten; 1-3 wirksame Querschnitte mit Randsteifen und Pfetten; 1-4 nichtrostender Stahl; 1-5 Schubverzerrung, ausgesteifte Platten; 1-6 andere Schalen als der Zylinder; 3-1/3-2, 1-11, 5, 2, 6 über die vereinfachten Nachweise hinaus. Die Gebrauchstauglichkeit nutzt weiter ein Gleichlastäquivalent. Die Tabelle zur Sprödbruchsicherheit (1-10 Tabelle 2.1) ist nicht kodiert; die Temperatur ist im Modellnachweis auf 0 Grad C festgelegt. Die Verschiebung delta-M bei Klasse 4 ist im Rechenkern null.

### Ungeprüfte Werte

Tabelle 6.2: warmgefertigte Hohlprofile S420 Linie a, a0 ab 460 MPa; M_N,Rd-Exponent bei Rundrohren 1,7; Zeilen C_m in Tabelle B.3 für Querbelastung; Faktoren in Anhang A wie übertragen; k_c-Werte 0,94/0,86; Spalte k_p in Tabelle 3.1 der EN 1993-1-2 und k_sh = 1; E_s für nichtrostenden Stahl; Spangler-Formel; Plattenbeiwert aus 1-7; Schub-Abschneidegrenze bei Ermüdung 0,457 Delta tau_C; NA-DE k_b und lambda_c0 = 0,5 sind nicht umgesetzt. Die Spezifikationswerte 0,4552 und 0,6442 für Gl. 6.61/6.62 wurden durch 0,4565 und 0,6418 ersetzt (gamma_M1 in n, wie die Norm es verlangt). Eine Bibliothek Dritter für Querschnittswerte stand nicht zur Verfügung; Querschnittskonstanten sind daher nicht damit gegengeprüft.

### 3D-Ansichten

Stäbe aus dem Katalogprofil mit Endlagern, seitlichen Halterungen, Längenmaß, Abschnittsplakette, Knickfigur und Versagenssymbolen; geschraubte und geschweißte Anschlüsse als Stirnplatte, Schraubenfeld, Kehlnaht und wählbare Gruppe; Plattenfelder; Silokonstruktionen mit Janssen-Druckzone; Zugglieder; kaltgeformte Bauteile; Pfähle mit Pfahlköpfen; Bemessungslastsymbole; Wind auf Turmstielen; Raddrücke auf Kranbahnen; Brandsymbol; Ermüdungsgruppen mit Beschriftung. Steifen, Profilvoute und Schweißsymbole werden nicht gezeichnet; der Seildurchmesser ist eine nominelle Symbolgröße.

### Diagramme (Tabellenzwillinge)

Knicklinien mit y- und z-Markern, gebunden an die Knicklängen; Biegedrillknicken chi_LT und M_b,Rd über dem Abstand der Halterungen (ziehbarer Marker); M-N-Polygon mit Beanspruchung und Ausnutzungsstrahl; Klassifizierungs-Spannungsblock; S-N (doppeltlogarithmisch) mit Kollektiv und Schädigung; Brandtemperatur- und k_theta-Kurven.

### Editierbare Griffe

Stablänge, Halterungsabstand, Profilhöhe aus dem Katalog; Schraubenabstand in Kraftrichtung und quer dazu, End- und Randabstand, Reihenzahl und Zahl je Reihe; Nahtlänge und Kehlnahtdicke; Plattenfeld a, b, t; Silo-Radius, -Tiefe, t; Einbindelänge des Pfahls. Ein numerischer Zug am Profil wählt ein Katalogprofil (und fügt es bei Bedarf ein).

### Durchgerechnete Beispiele

Mitgelieferte Beispiele: HEB 240 erfüllt, hochfester Anschluss.

1. "HEB 240 erfüllt" öffnen, den Stab wählen. Den Längengriff nach oben ziehen: Die Ausnutzungsfarbe wandert zur Warnskala, der Marker im Knickdiagramm verschiebt sich, das Nachweisfenster zeigt nur die geänderten Zeilen (Neuauswertung je Objekt).
2. Den Halterungsabstand ziehen: Der Marker im chi_LT-Diagramm folgt; verkürzen stellt Tragfähigkeit wieder her.
3. "Hochfester Anschluss" öffnen: Den Schraubenabstand unter das Minimum der Tabelle 3.3 ziehen; der Abstandsnachweis wird rot, der Vorschlag nennt das Mindestmaß.

Referenzwerte (Orakel und eigenständige Tests): Gl. 6.61 = 0,4565 und Gl. 6.62 = 0,6418 für das NA-DE-Beispiel E06; Stahltemperatur E12 innerhalb 3 K der Spezifikation; M_cr mit Hermite-Finiten-Elementen stimmt mit scipy auf 0,5 % überein.
