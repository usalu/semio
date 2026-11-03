---
name: en1994
kind: user
---

# EN 1994 Composite steel and concrete

Shared guide: [norm](../../README.md).

## English

### Status

Written, not yet compiled or run (audits of 2026-11-02).

### What is covered

Materials; section classification; effective width; global analysis choice; bending resistance by plastic stress block (full and partial shear connection, linear interpolation), hogging moment; shear connectors (P_Rd, minimum degree of connection eta_min, layout and spacing, longitudinal shear V_l,Rd); vertical shear and bending-shear interaction; lateral-torsional buckling (M_cr of the bare beam with warping and inverted U-frame); composite columns (N_pl, M-N polygon, second order, CHS confinement); composite slabs with profiled sheeting (m-k resistance, bending, vertical shear, punching, serviceability); serviceability (staged stresses, deflection with creep and shrinkage, crack control); fire (EN 1994-1-2: protected steel temperature by time stepping, M_fi,Rd, insulation, slab criteria); bridge girders (effective width at supports, cracked analysis, crack control, stud group, fatigue incl. reinforcement).

### Not covered

C-48 and C-54 and the remaining items are listed in the completion report as gaps. No fire diagram exists yet. Columns of encased and partially encased type use an equivalent tube because the model has no I-section geometry. Rib pitch and column shape are inferred, concrete inside the ribs is not drawn, and the shear-flow colouring is the elastic indicator. Placement fields are not stored.

### Unverified values

Table 6.2 k_t,max values and the NA-DE sheeting restriction; k_c C_4 = pi and alpha = 2; transverse reinforcement lever 0.8 d; creep phi = 2.5, shrinkage 0.3 per mille, psi_L 1.1/0.55, NA-DE deflection limits (L/250 EN, L/300 DE total, L/300 variable); rib pitch 0.25 m and developed-length factor 1.2; sheeting S280GD; slab punching critical perimeter at 2 d_p around a 200 mm square; slab fire h_eff limits 60/80/100/120 mm; protection data (lambda 0.12, c 1000, rho 800); reinforcement fatigue 162.5 MPa at 10^6 cycles, k_2 = 9, gamma 1.15; FLM3 surrogate forces 240 kN and 60 kN; rebar at 50 mm from the edge; E_cm from f_ck + 8.

### 3D views

Steel I member with deflection overlay from the serviceability check; concrete slab plate with a translucent effective-width ghost; profiled sheeting as thin-walled member with ribs transverse or parallel; shear studs in 6 zones coloured by elastic shear flow; transverse and hogging rebar; PNA plane; cracked zone; supports, construction restraints; area and point loads; dimensions bound to fields; eta label; clause badge; failure glyphs (lateral-torsional buckling, bending, connection, fatigue, cracking). Columns: CHS, RHS, encased and partially encased steel with concrete body and rebar. Slabs: strip with sheeting, bars, supports and loads.

### Diagrams (table twins)

Plastic stress blocks with PNA; partial-connection M(eta) with draggable marker (writes the stud count); Ollgaard stud load-slip; deflection line (marker writes the span); shear flow against stud resistance; column M-N polygon with alpha_M polygon, demand and utilisation ray; column buckling curve (marker writes the length); slab moment over span.

### Editable handles

Beam span, slab thickness, effective width (writes the spacing when capped, otherwise the span), stud spacing, studs per rib, stud count, sheeting height and orientation, catalogue section; column length, size, wall; slab span, thickness, sheeting height and orientation. Numeric section drags pick the nearest HEB (HEB 300 to HEB 700 offered).

### Worked examples

Shipped examples: composite floor beam, composite floor beam (failing), composite bridge girder.

1. Open "composite floor beam (failing)". The stud and the bending rows fail; select the stud row and drag the stud count marker in the M(eta) diagram upwards until the connection check passes.
2. Open the floor beam and drag the span handle: the effective-width ghost and the deflection overlay change; the deflection line marker shows the span.
3. Open the bridge girder and view the hogging cracked zone and the stud group.

Reference values: no numeric reference values are published in the audits yet; the PNA is cross-checked by scipy brentq on a shapely fibre section once the suite runs.

## Deutsch

### Stand

Geschrieben, noch nicht kompiliert oder ausgeführt (Berichte vom 02.11.2026).

### Was abgedeckt ist

Baustoffe; Querschnittsklassifizierung; mitwirkende Breite; Wahl der Schnittgrößenermittlung; Biegetragfähigkeit nach plastischem Spannungsblock (volle und teilweise Verdübelung, lineare Interpolation), negatives Moment; Verbundmittel (P_Rd, Mindestverdübelungsgrad eta_min, Anordnung und Abstände, Längsschub V_l,Rd); Querkraft und Interaktion Biegung-Querkraft; Biegedrillknicken (M_cr des Stahlträgers mit Wölbung und des umgekehrten U-Rahmens); Verbundstützen (N_pl, M-N-Polygon, Theorie zweiter Ordnung, Umschnürung bei Rundrohr); Verbunddecken mit Profilblech (m-k-Verfahren, Biegung, Querkraft, Durchstanzen, Gebrauchstauglichkeit); Gebrauchstauglichkeit (Spannungen nach Bauzuständen, Durchbiegung mit Kriechen und Schwinden, Rissbreitenbegrenzung); Brand (EN 1994-1-2: Temperatur geschützter Stahlteile im Zeitschritt, M_fi,Rd, Wärmedämmung, Deckenkriterien); Brückenträger (mitwirkende Breite an Stützen, gerissene Schnittgrößenermittlung, Rissbreitenbegrenzung, Dübelgruppe, Ermüdung einschließlich Bewehrung).

### Nicht abgedeckt

C-48 und C-54 sowie weitere Punkte stehen im Abschlussbericht als Lücken. Ein Brandschutzdiagramm gibt es noch nicht. Einbetonierte und teilweise einbetonierte Stützen nutzen ein Ersatzrohr, weil das Modell keine I-Querschnittsgeometrie hat. Rippenabstand und Stützenform werden abgeleitet, Beton in den Rippen wird nicht gezeichnet, und die Schubfluss-Färbung ist der elastische Anzeiger. Platzierungsfelder werden nicht gespeichert.

### Ungeprüfte Werte

Tabelle 6.2 Werte k_t,max und die NA-DE-Einschränkung für Profilbleche; k_c C_4 = pi und alpha = 2; Hebelarm der Querbewehrung 0,8 d; Kriechen phi = 2,5, Schwinden 0,3 Promille, psi_L 1,1/0,55, NA-DE-Durchbiegungsgrenzen (L/250 EN, L/300 DE gesamt, L/300 veränderlich); Rippenabstand 0,25 m und Abwicklungsfaktor 1,2; Profilblech S280GD; kritischer Rundschnitt der Decke bei 2 d_p um ein 200-mm-Quadrat; Grenzen h_eff der Deckenbrandprüfung 60/80/100/120 mm; Bekleidungsdaten (lambda 0,12, c 1000, rho 800); Bewehrungsermüdung 162,5 MPa bei 10^6 Zyklen, k_2 = 9, gamma 1,15; Ersatzkräfte FLM3 240 kN und 60 kN; Bewehrung 50 mm vom Rand; E_cm aus f_ck + 8.

### 3D-Ansichten

Stahl-I-Träger mit Durchbiegungsüberlagerung aus dem Gebrauchsnachweis; Betonplatte mit durchscheinendem Geisterbild der mitwirkenden Breite; Profilblech als dünnwandiges Bauteil mit Rippen quer oder parallel; Kopfbolzendübel in 6 Zonen, gefärbt nach elastischem Schubfluss; Quer- und Stützbewehrung; Ebene der plastischen Nulllinie; gerissener Bereich; Lager, Montagehalterungen; Flächen- und Einzellasten; an Felder gebundene Maße; eta-Beschriftung; Abschnittsplakette; Versagenssymbole (Biegedrillknicken, Biegung, Verbindung, Ermüdung, Rissbildung). Stützen: Rundrohr, Rechteckrohr, einbetonierte und teilweise einbetonierte Stahlprofile mit Betonkörper und Bewehrung. Decken: Streifen mit Profilblech, Stäben, Lagern und Lasten.

### Diagramme (Tabellenzwillinge)

Plastische Spannungsblöcke mit Nulllinie; teilweise Verdübelung M(eta) mit ziehbarem Marker (schreibt die Dübelzahl); Ollgaard-Dübelkennlinie; Durchbiegungslinie (Marker schreibt die Spannweite); Schubfluss gegen Dübeltragfähigkeit; Stützen-M-N-Polygon mit alpha_M-Polygon, Beanspruchung und Ausnutzungsstrahl; Stützenknickkurve (Marker schreibt die Länge); Deckenmoment über der Spannweite.

### Editierbare Griffe

Trägerspannweite, Plattendicke, mitwirkende Breite (schreibt den Abstand, wenn begrenzt, sonst die Spannweite), Dübelabstand, Dübel je Rippe, Dübelzahl, Profilblechhöhe und -ausrichtung, Katalogprofil; Stützenlänge, -größe, -wand; Deckenspannweite, -dicke, Profilblechhöhe und -ausrichtung. Numerische Profilzüge wählen das nächste HEB (HEB 300 bis HEB 700 angeboten).

### Durchgerechnete Beispiele

Mitgelieferte Beispiele: Verbunddeckenträger, Verbunddeckenträger (nicht erfüllt), Verbundbrückenträger.

1. "Verbunddeckenträger (nicht erfüllt)" öffnen. Dübel- und Biegezeilen schlagen fehl; die Dübelzeile wählen und den Dübelzahl-Marker im M(eta)-Diagramm nach oben ziehen, bis der Verbindungsnachweis erfüllt ist.
2. Den Deckenträger öffnen und den Spannweitengriff ziehen: Geisterbild der mitwirkenden Breite und Durchbiegungsüberlagerung ändern sich; der Marker der Durchbiegungslinie zeigt die Spannweite.
3. Den Brückenträger öffnen und den gerissenen Stützbereich sowie die Dübelgruppe ansehen.

Referenzwerte: In den Berichten sind noch keine Zahlenwerte veröffentlicht; die plastische Nulllinie wird nach dem Testlauf per scipy brentq an einem shapely-Fasermodell gegengeprüft.
