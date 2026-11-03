---
name: en1992
kind: user
---

# EN 1992 Concrete

Shared guide: [norm](../../README.md).

## English

### Status

Written, not yet compiled or run (audits of 2026-11-02). Reinforcement, tendon, punching, anchor and fire pictures are drawn only from the data in the model.

### What is covered

156 catalogue entries in 11 groups (function level, member checks use the same functions): materials and time effects; section resistance (strain compatibility with parabola-rectangle block, N-M curve, constant-N slice of the biaxial surface); shear and torsion; punching and struts (links, perimeters for edge and corner columns, strut-and-tie truss solver); detailing (bond, laps, anchorage); serviceability (stresses, crack width by 7.3.4, deflection, l/d limit); prestress and fatigue; fire (ISO 834 field, 500 degree isotherm, zone method, tabulated data of EN 1992-1-2); bridges and liquid-retaining structures; fastenings to concrete (EN 1992-4: steel, cone, pull-out, splitting, pry-out, edge, interaction, groups); second-order models (nominal curvature and stiffness, general method).

### Not covered

Not drawn for lack of data: outer punching perimeter u_out, stud rings, tendon force along the length (friction and slip), strut-and-tie and shear truss pictures, creep and shrinkage curves, lap-zone highlight, polygon sections, explicit bar positions, tendon control points. Handles on bar positions, polygon vertices and tendon control points need those model fields. Link cages are drawn as a rectangular cage only.

### Unverified values (NA-DE and others)

Each outcome that depends on one is downgraded to a warning and names the symbol. Listed as unverified: accidental alpha_cc (DE 0.85) and gamma_c (DE 1.3) against 1.0 and 1.2 in the specification; DE punching k1 and the V_Rd,max factor (1.4 v_Rd,c); the wall edge factor beta 1.35; strut and node strengths (DE 1.1/0.75/0.75); the quasi-permanent prestress stress (0.65 f_pk); friction and unbonded values; crack-width limits for prestress and liquid tightness; the l/d DE cap and lambda_lim DE limit; the DE cover table by structural class; minimum column reinforcement (DE 0.15); maximum slab bar spacing (DE 250); every fire table value ("from recollection of the printed tables"); redistribution factors (DE 0.64/0.8); bridge cover and fct limits; gamma_s fatigue values; tie forces; spalling addition. The prestressing S-N values (185/150/120/80 MPa), Tables 7.2N/7.3N, 6.1 and 3.1 are coded as literals from memory and not registered as NDPs. The isotherm is a semi-infinite-solid estimate (19/32/43/52 mm for R30 to R120), not the Annex A/B field; the crack extent in the 3D model uses an illustrative moment threshold (0.4 M_max); the load point uses a simplified ULS combination (1.35/1.5/1.05). Verdicts and colours always come from the report, not from these pictures.

### 3D views

Concrete member or plate with deflected shape, governing utilisation and clause badge; cover shell (red when the cover check fails); one bar per reinforcement bar with per-station flexure utilisation, labels and anchorage-length dimensions; link cage with 135 degree hooks; parabolic tendon with duct and P/e label; supports and load glyphs; crack pattern (with s_r,max, width exaggerated x10) when the crack check exists; 500 degree isotherm and reduced section at the start face; N-My-Mz capacity surface with the design point and margin; punching: column prism and the rounded perimeters u0 and u1 clipped at edges and corners; anchors: block, shaft with head, base plate, 35 degree cone cut at the edge distance, tension and shear glyphs, shear-cone failure glyph.

### Diagrams (table twins)

Interaction N-M (capacity polygon, design point, utilisation ray), stress block (strain plane, block, neutral axis), tendon eccentricity (draggable marker), tendon force, deflection (L/250, L/500), fire section, material laws.

### Editable handles

Span, width, height, effective depth, cover, link spacing, layer count and diameter (catalogue values), tendon eccentricity, fire axis distance, column width and depth, anchor h_ef, diameter and edge distance c1. Rules for prestress eccentricity, punching column size and the anchor c1 and diameter are still missing, so those drags are refused for now.

### Worked examples

Shipped examples: compliant office frame, compliant prestressed beam, failing prestressed beam, failing under-reinforced section, liquid-retaining FEM anchor.

1. Open "failing under-reinforced": the flexure check fails with a cross and diagonal hatching; the N-M diagram shows the demand outside the polygon. Drag the layer-diameter handle through the catalogue sizes until the check passes; the remedy chip proposes the required area.
2. Open "compliant prestressed beam": drag the tendon eccentricity marker in the diagram; the 3D tendon follows and the stress and crack checks update.
3. Select a column in the office frame and widen it with the width handle: the punching perimeters change.

Reference values (hand values from the specification, tests not yet run): second-order example E-11 gives e2 = 18.66 mm; the catalogue is compared with a Python oracle using shapely and scipy.

## Deutsch

### Stand

Geschrieben, noch nicht kompiliert oder ausgeführt (Berichte vom 02.11.2026). Bewehrung, Spannglied, Durchstanzen, Dübel und Brand werden nur aus den Daten des Modells gezeichnet.

### Was abgedeckt ist

156 Katalogeinträge in 11 Gruppen (Funktionsebene; die Bauteilnachweise nutzen dieselben Funktionen): Baustoffe und zeitabhängige Effekte; Querschnittstragfähigkeit (Dehnungsverträglichkeit mit Parabel-Rechteck-Block, N-M-Kurve, Schnitt der zweiachsigen Fläche bei konstantem N); Querkraft und Torsion; Durchstanzen und Druckstreben (Schubbewehrung, Rundschnitte für Rand- und Eckstützen, Fachwerklöser für Stabwerkmodelle); Konstruktion (Verbund, Übergreifung, Verankerung); Gebrauchstauglichkeit (Spannungen, Rissbreite nach 7.3.4, Durchbiegung, l/d-Grenze); Vorspannung und Ermüdung; Brand (ISO-834-Feld, 500-Grad-Isotherme, Zonenmethode, Tabellen der EN 1992-1-2); Brücken und Flüssigkeitsbehälter; Befestigungen im Beton (EN 1992-4: Stahl, Betonausbruch, Herausziehen, Spalten, Betonkantenbruch, Interaktion, Gruppen); Modelle zweiter Ordnung (Nennkrümmung und Nennsteifigkeit, allgemeines Verfahren).

### Nicht abgedeckt

Mangels Daten nicht gezeichnet: äußerer Rundschnitt u_out, Dübelleisten, Spanngliedkraft über die Länge (Reibung und Schlupf), Bilder zu Stabwerk und Schubfachwerk, Kriech- und Schwindkurven, Hervorhebung der Übergreifungszone, Polygonquerschnitte, explizite Stablagen, Spanngliedstützpunkte. Griffe an Stablagen, Polygonecken und Spanngliedstützpunkten brauchen diese Modellfelder. Bügelkörbe werden nur als Rechteckkorb gezeichnet.

### Ungeprüfte Werte (NA-DE und andere)

Jedes Ergebnis, das von einem abhängt, wird zur Warnung herabgestuft und nennt das Symbol. Als ungeprüft geführt: außergewöhnliches alpha_cc (DE 0,85) und gamma_c (DE 1,3) gegenüber 1,0 und 1,2 in der Spezifikation; DE-k1 beim Durchstanzen und der V_Rd,max-Faktor (1,4 v_Rd,c); der Wandrandbeiwert beta 1,35; Strebenund Knotenfestigkeiten (DE 1,1/0,75/0,75); die quasi-ständige Spannstahlspannung (0,65 f_pk); Reibungs- und Werte ohne Verbund; Rissbreitengrenzen für Vorspannung und Dichtigkeit; die DE-Kappung von l/d und die DE-Grenze lambda_lim; die DE-Betondeckungstabelle nach Expositionsklasse; Mindestbewehrung von Stützen (DE 0,15); größter Stababstand in Platten (DE 250); jeder Brandtabellenwert ("aus der Erinnerung an die gedruckten Tabellen"); Umlagerungsfaktoren (DE 0,64/0,8); Brückendeckung und fct-Grenzen; gamma_s bei Ermüdung; Zugkräfte; Abplatzzuschlag. Die Spannstahl-S-N-Werte (185/150/120/80 MPa), die Tabellen 7.2N/7.3N, 6.1 und 3.1 sind als Literale aus dem Gedächtnis kodiert und nicht als NDP registriert. Die Isotherme ist eine Abschätzung für den Halbraum (19/32/43/52 mm für R30 bis R120), nicht das Feld aus Anhang A/B; die Rissausdehnung im 3D-Modell nutzt eine anschauliche Momentenschwelle (0,4 M_max); der Lastpunkt nutzt eine vereinfachte GZT-Kombination (1,35/1,5/1,05). Urteile und Farben stammen immer aus dem Bericht, nicht aus diesen Bildern.

### 3D-Ansichten

Betonbauteil oder -platte mit verformter Lage, maßgebender Ausnutzung und Abschnittsplakette; Deckungshülle (rot, wenn die Deckungsprüfung fehlschlägt); je Stab ein Bewehrungsstab mit Biegeausnutzung je Stelle, Beschriftungen und Verankerungslängen-Maßen; Bügelkorb mit 135-Grad-Haken; parabelförmiges Spannglied mit Hüllrohr und P/e-Beschriftung; Lager und Lastsymbole; Rissbild (mit s_r,max, Breite zehnfach überhöht), sofern der Rissnachweis existiert; 500-Grad-Isotherme und Restquerschnitt an der Anfangsfläche; Tragfähigkeitsfläche N-My-Mz mit Bemessungspunkt und Reserve; Durchstanzen: Stützenkörper und abgerundete Rundschnitte u0 und u1, an Rändern und Ecken beschnitten; Dübel: Block, Schaft mit Kopf, Ankerplatte, 35-Grad-Kegel am Randabstand abgeschnitten, Zug- und Querkraftsymbole, Versagenssymbol des Ausbruchkegels.

### Diagramme (Tabellenzwillinge)

Interaktion N-M (Tragfähigkeitspolygon, Bemessungspunkt, Ausnutzungsstrahl), Spannungsblock (Dehnungsebene, Block, Nulllinie), Spanngliedausmitte (ziehbarer Marker), Spanngliedkraft, Durchbiegung (L/250, L/500), Brandquerschnitt, Stoffgesetze.

### Editierbare Griffe

Spannweite, Breite, Höhe, statische Nutzhöhe, Betondeckung, Bügelabstand, Lagenzahl und Durchmesser (Katalogwerte), Spanngliedausmitte, Achsabstand im Brandfall, Stützenbreite und -tiefe, Dübeleinbindetiefe h_ef, Durchmesser und Randabstand c1. Regeln für die Ausmitte der Vorspannung, die Stützenabmessung beim Durchstanzen sowie Dübel-c1 und -Durchmesser fehlen noch; diese Züge werden daher vorerst abgelehnt.

### Durchgerechnete Beispiele

Mitgelieferte Beispiele: erfüllter Bürorahmen, erfüllter Spannbetonträger, nicht erfüllter Spannbetonträger, nicht erfüllter unterbewehrter Querschnitt, Flüssigkeitsbehälter mit FEM-Dübel.

1. "Nicht erfüllter unterbewehrter Querschnitt" öffnen: Der Biegenachweis schlägt mit Kreuz und diagonaler Schraffur fehl; das N-M-Diagramm zeigt die Beanspruchung außerhalb des Polygons. Den Durchmessergriff der Lage durch die Katalogwerte ziehen, bis der Nachweis erfüllt ist; der Abhilfe-Vorschlag nennt die erforderliche Fläche.
2. "Erfüllter Spannbetonträger" öffnen: den Ausmitten-Marker im Diagramm ziehen; das 3D-Spannglied folgt, Spannungs- und Rissnachweise aktualisieren sich.
3. Im Bürorahmen eine Stütze wählen und mit dem Breitengriff verbreitern: Die Rundschnitte ändern sich.

Referenzwerte (Handwerte aus der Spezifikation, Tests noch nicht ausgeführt): Das Beispiel zur Theorie zweiter Ordnung E-11 ergibt e2 = 18,66 mm; der Katalog wird mit einem Python-Orakel (shapely, scipy) verglichen.
