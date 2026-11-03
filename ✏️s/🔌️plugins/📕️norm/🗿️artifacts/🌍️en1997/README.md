---
name: en1997
kind: user
---

# EN 1997 Geotechnical design

Shared guide: [norm](../../README.md).

## English

### Status

Written, never compiled (audits of 2026-11-02). The catalogue (62 entries) runs on cases; the model reaches only the 14 editable set-field paths that existed before.

### What is covered

Design approaches and partial factors (A1/A2, M1/M2, R1 to R4, DA1, DA2, DA2*, DA3, DIN 1054 Tables A 2.1 to A 2.3, EQU, UPL, HYD); ground model, stress profile with groundwater, characteristic values (k_n with Student t); EN 1997-2 investigation (CPT classification, c_u, phi' by Kulhawy-Mayne, E_oed, SPT N60, Hatanaka-Uchida, Skempton, DPH, oedometer, triaxial, direct shear, Menard, grain size, Thiem); spread foundations (drained Annex D.4 with eccentricity and tilt, undrained D.3, DIN 4017, sliding, EQU, uplift, hydraulic heave); settlement (stress-strain, Annex F.2, consolidation, secondary, tilt); piles (static and ground tests, tension, transverse, negative skin friction, group, buckling, dynamic); earth pressure and walls (Coulomb, Rankine, passive on curved surface, K0, gravity and cantilever walls, Mononobe-Okabe, reinforced soil, traffic surcharge); embedded walls, anchors, dewatering; slopes (Bishop, Fellenius, Janbu, infinite slope, plane wedge). A ground-model consistency row replaces the old strength heuristic.

### Not covered

The model has no anchors, embedded walls, pile groups, laboratory tests or per-layer water entities, so those verifications run only on cases, not from a document. Not drawn: hydraulic flow nets, anchored walls, slip-circle heat map, per-pile load distribution, negative skin friction, block-failure prism. Only the governing load case (largest eccentricity) of a footing is drawn. The edit rules cover only 14 paths; the remaining 55 snapshot leaves cannot be edited by field yet.

### Unverified values

All values the specification marks `[V]` plus: DIN 1054 Tables A 2.1 to A 2.3 rows for BS-T and BS-A; gamma_H 1.80/1.45/1.30; EQU 1.05/0.95/1.5; UPL 1.0/0.9/1.5; the German annex has no DA1, so DA1 uses the EN values for both annexes; accidental situation = all factors 1.0; R3/R4 factors; NA-DE xi1/xi2 table; empirical pile resistances (switch without table data); gamma_w 10 kN/m3 (DE) and 9.81 (EN); DIN 4017 exponents; eccentricity limits B/6 and B/3; Caquot-Kerisel comparison values; Broms, Winkler 0.322 and p-multipliers; Converse-Labarre; anchor factors; free lengths; Blum x1.2; Sichardt 3000 and Kusakin 575; CPT N_kt 15 and alpha_E 4; the DPH table (coarse); investigation spacing; t quantiles between tabulated rows; the presumptive bearing table for geotechnical category 1 (old values). The drawn mechanisms are characteristic-value pictures; the increased active pressure is drawn as the mean of K_a and K_0 (a visual rule, not the DIN 4085 table).

### 3D views

Ground: one translucent strata column per layer (soil-class colour, phi'/c'/gamma label, thickness dimension bound to the layer bottom) and the groundwater plane (bound to the level). Footings: footing body with pedestal, hatched effective-area plate centred on the resultant, V/H/M glyphs of the governing load case, five-part bearing failure mechanism (opacity and colour follow the bearing utilisation), sliding plane, dimensions B, L, d, B'. Walls: inverted-T wall, backfill, active and passive wedges, driving and passive pressure surfaces (40 kPa per metre drawing scale), resultants, surcharge, coefficient label. Slopes: embankment, critical Bishop circle, 12 slices coloured by local Fellenius demand, factor label, failure glyph when F < 1. Piles: grid with cap, shaft resistance glyphs per layer, base resistance. Uplift: slab with pore pressure. Settlement: a 21 x 13 dish (exaggerated x25, legend in mm) over all footings.

### Diagrams (table twins)

Earth pressure over elevation per wall (driving, passive, net, resultant markers); CPT and SPT step profiles with groundwater, investigation depth and derived phi' and E_oed markers; bearing capacity factors (log axis) with footing operating points; load-settlement per footing.

### Editable handles

Layer interfaces (write both adjacent fields), groundwater level; footing width, length, embedment, horizontal load tip, position; wall height, embedment, base width, stem thickness; slope height and angle; pile length, diameter, count, position. A drag is a displacement from the anchor on a 0.05 m grid.

### Worked examples

Shipped examples: compliant, non-compliant, demo.

1. Open "compliant" and drag the footing width handle smaller: the effective-area plate shrinks, the bearing mechanism gets more opaque and red, the settlement dish deepens; stop where the bearing utilisation reaches 100 %.
2. Drag the groundwater level up: the stress profile and the bearing and uplift checks change.
3. In "non-compliant" lower the slope angle handle until the slip-circle factor reaches 1.

Reference values (tests not yet run): the oracle compares 107 cases with 694 values to 1 % (numpy, scipy, shapely; Boussinesq by dblquad); examples EX97-01 to EX97-15 of the specification; Coulomb coefficients against the textbook values.

## Deutsch

### Stand

Geschrieben, nie kompiliert (Berichte vom 02.11.2026). Der Katalog (62 Einträge) läuft auf Fällen; das Modell erreicht nur die 14 bereits bearbeitbaren Feldpfade.

### Was abgedeckt ist

Nachweisverfahren und Teilsicherheitsbeiwerte (A1/A2, M1/M2, R1 bis R4, DA1, DA2, DA2*, DA3, DIN 1054 Tabellen A 2.1 bis A 2.3, EQU, UPL, HYD); Baugrundmodell, Spannungsverlauf mit Grundwasser, charakteristische Werte (k_n mit Student-t); Erkundung nach EN 1997-2 (CPT-Klassifizierung, c_u, phi' nach Kulhawy-Mayne, E_oed, SPT N60, Hatanaka-Uchida, Skempton, DPH, Ödometer, Triaxial, Rahmenscherversuch, Menard, Kornverteilung, Thiem); Flachgründungen (drainiert Anhang D.4 mit Ausmitte und Schiefstellung, undrainiert D.3, DIN 4017, Gleiten, EQU, Auftrieb, hydraulischer Grundbruch); Setzung (Spannungs-Dehnungs-Verfahren, Anhang F.2, Konsolidation, Sekundärsetzung, Verkippung); Pfähle (statische und Baugrundversuche, Zug, Quer, negative Mantelreibung, Gruppe, Knicken, dynamisch); Erddruck und Wände (Coulomb, Rankine, Erdwiderstand auf gekrümmter Fläche, K0, Schwergewichts- und Winkelstützwände, Mononobe-Okabe, bewehrte Erde, Verkehrslast); Spundwände, Anker, Wasserhaltung; Böschungen (Bishop, Fellenius, Janbu, unendlich lange Böschung, ebener Keil). Eine Konsistenzprüfung des Baugrundmodells ersetzt die frühere Festigkeitsheuristik.

### Nicht abgedeckt

Das Modell hat keine Anker, Spundwände, Pfahlgruppen, Laborversuche oder Wasserobjekte je Schicht; diese Nachweise laufen daher nur auf Fällen, nicht aus einem Dokument. Nicht gezeichnet: hydraulische Strömungsnetze, verankerte Wände, Wärmebild der Gleitkreise, Lastverteilung je Pfahl, negative Mantelreibung, Prisma des Blockversagens. Es wird nur der maßgebende Lastfall (größte Ausmitte) eines Fundaments gezeichnet. Die Bearbeitungsregeln decken nur 14 Pfade; die übrigen 55 Blätter lassen sich noch nicht feldweise bearbeiten.

### Ungeprüfte Werte

Alle Werte, die die Spezifikation mit `[V]` markiert, außerdem: DIN-1054-Tabellen A 2.1 bis A 2.3 für BS-T und BS-A; gamma_H 1,80/1,45/1,30; EQU 1,05/0,95/1,5; UPL 1,0/0,9/1,5; der deutsche Anhang kennt kein DA1, daher nutzt DA1 die EN-Werte für beide Anhänge; außergewöhnliche Situation = alle Beiwerte 1,0; Faktoren R3/R4; NA-DE-Tabelle xi1/xi2; empirische Pfahlwiderstände (Schalter ohne Tabellendaten); gamma_w 10 kN/m3 (DE) und 9,81 (EN); DIN-4017-Exponenten; Ausmittegrenzen B/6 und B/3; Vergleichswerte nach Caquot-Kerisel; Broms, Winkler 0,322 und p-Multiplikatoren; Converse-Labarre; Ankerfaktoren; freie Längen; Blum x1,2; Sichardt 3000 und Kusakin 575; CPT N_kt 15 und alpha_E 4; die DPH-Tabelle (grob); Erkundungsabstände; t-Quantile zwischen Tabellenzeilen; die Tabelle zulässiger Sohldrücke für Geotechnische Kategorie 1 (alte Werte). Die gezeichneten Mechanismen sind Bilder mit charakteristischen Werten; der erhöhte aktive Erddruck wird als Mittel aus K_a und K_0 gezeichnet (Anschauungsregel, nicht die Tabelle der DIN 4085).

### 3D-Ansichten

Baugrund: je Schicht eine durchscheinende Schichtsäule (Farbe nach Bodenklasse, Beschriftung phi'/c'/gamma, Dickenmaß gebunden an die Schichtunterkante) und die Grundwasserebene (an den Wasserstand gebunden). Fundamente: Fundamentkörper mit Sockel, schraffierte Ersatzfläche in der Resultierenden zentriert, V/H/M-Symbole des maßgebenden Lastfalls, fünfteiliger Grundbruchmechanismus (Deckkraft und Farbe folgen der Grundbruchausnutzung), Gleitfuge, Maße B, L, d, B'. Wände: Winkelstützwand, Hinterfüllung, aktiver und passiver Keil, wirkende und passive Druckflächen (Zeichenmaßstab 40 kPa je Meter), Resultierende, Auflast, Beiwertbeschriftung. Böschungen: Damm, kritischer Bishop-Kreis, 12 Lamellen nach örtlichem Fellenius-Wert gefärbt, Faktorbeschriftung, Versagenssymbol bei F < 1. Pfähle: Raster mit Kopfplatte, Mantelwiderstandssymbole je Schicht, Spitzenwiderstand. Auftrieb: Platte mit Porenwasserdruck. Setzung: eine 21 x 13 Mulde (25-fach überhöht, Legende in mm) über allen Fundamenten.

### Diagramme (Tabellenzwillinge)

Erddruck über der Höhe je Wand (wirkend, passiv, netto, Resultierendenmarker); CPT- und SPT-Stufenprofile mit Grundwasser, Erkundungstiefe und abgeleiteten Markern für phi' und E_oed; Grundbruchbeiwerte (logarithmische Achse) mit Betriebspunkten der Fundamente; Last-Setzungs-Linie je Fundament.

### Editierbare Griffe

Schichtgrenzen (schreiben beide benachbarten Felder), Grundwasserstand; Fundamentbreite, -länge, Einbindetiefe, Spitze der Horizontallast, Lage; Wandhöhe, Einbindung, Fußbreite, Stieldicke; Böschungshöhe und -neigung; Pfahllänge, -durchmesser, -anzahl, -lage. Ein Zug ist eine Verschiebung vom Anker auf einem 0,05-m-Raster.

### Durchgerechnete Beispiele

Mitgelieferte Beispiele: erfüllt, nicht erfüllt, Demo.

1. "Erfüllt" öffnen und den Fundamentbreitengriff kleiner ziehen: Die Ersatzfläche schrumpft, der Grundbruchmechanismus wird deckender und rot, die Setzungsmulde vertieft sich; anhalten, wo die Grundbruchausnutzung 100 % erreicht.
2. Den Grundwasserstand nach oben ziehen: Spannungsverlauf sowie Grundbruch- und Auftriebsnachweise ändern sich.
3. In "nicht erfüllt" den Böschungswinkelgriff verkleinern, bis der Gleitkreisfaktor 1 erreicht.

Referenzwerte (Tests noch nicht ausgeführt): Das Orakel vergleicht 107 Fälle mit 694 Werten auf 1 % (numpy, scipy, shapely; Boussinesq per dblquad); die Beispiele EX97-01 bis EX97-15 der Spezifikation; Coulomb-Beiwerte gegen die Lehrbuchwerte.
