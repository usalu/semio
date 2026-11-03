---
name: en1999
kind: user
---

# EN 1999 Aluminium

Shared guide: [norm](../../README.md).

## English

### Status

Written, never compiled in Rust (audits of 2026-11-02). Only the Python oracle ran (both examples and 16 shared vectors passed). The shipped compliant example was changed to stay compliant (fire start 20 degrees C and 30 s, shell hoop stress 12 MPa, bolt gauge 18 mm, sheet point load 0).

### What is covered

Every check set is evaluated for every EN 1990 combination (6.10a/b per leading action, serviceability, fire) and the highest utilisation per check id is kept. Alloy bands (6082-T6 with two thickness bands; an unknown alloy fails); HAZ strips (b_haz 20/30/35/40 mm) with rho_o,haz and rho_u,haz; classification by beta/epsilon (class 4 gives a warning and the effective section applies); effective thickness rho_c with Table 6.3 constants; tension, compression, bending about both axes (shape factor capped at 1.25), shear, bending with shear, N-M interaction (class 1/2 exponent form, class 3/4 linear with delta M = N e_N); flexural and torsional buckling (kappa for longitudinal welds, omega_x function); lateral-torsional buckling (own curve); 6.59/6.60 interaction; web shear buckling of unstiffened webs; bolted and riveted joints (shear, bearing, tension, punching, spacings, block tearing, long joints); welds (weld metal and HAZ boundary); serviceability (deflection, stress, frequency); fatigue (EN 1999-1-3, Miner and range check); fire (EN 1999-1-2: member temperature by time stepping, k_0.2proof,theta, buckling and lateral-torsional buckling, critical temperature); thin-walled cold-formed sheet (1-4); shells (1-5).

### Not covered

Needing model fields that do not exist: stiffened elements (A-06), torsion (A-16), local loads and web crippling (A-19, A-27, A-61), stiffeners and tension field (A-28; webs are treated as unstiffened over the member length), the localized-weld omega_x wiring (the function exists), welded shells and fabrication class (A-72), stiffened shells (A-71), TIG and T-junction HAZ factors, sheeting from tests. Needing table values: Annex H method B (A-23), exact eta0/gamma0/xi0 and exponent tables, the alpha_max table, the kappa table, the full Table 3.2 for all product forms. Adhesives and pin joints are not modelled; creep (A-81) is out of scope. The M-N polygon ignores M_z and the class 4 shift. There is one weld position per plate; the section outline comes from the section kind and plate elements (box and RHS are polygons with a hole). Mode shapes are sinusoidal indications; the lateral-torsional mode has no twist.

### Unverified values (registered with `confirmed: false`)

Table 3.2 rows except 6082-T6 (6060, 6061, 6063, 5083; 6061 and 6063 differ from the specification seed); np, ductility and product-form columns not modelled; Table 6.2 limits with welds and Table 6.3 C1/C2 (internally consistent: rho_c = 1 at beta3 for all 8 rows); round-tube rule beta = 3 sqrt(D/t); alpha_max; kappa; omega_x exponent; torsional curve; eta0 = gamma0 = xi0 = 0.8 with lower bound 1; section interaction exponents (I and channel psi = 2, round tube 1.5, box and angle 1); shear buckling rule; MIG HAZ widths; bolt alpha_v 0.5 for 10.9, stress areas, d0 = d + 2/3 mm, punching d_m = 1.5 d, rivet f_ur 160 MPa, stainless 700 MPa; filler strengths 190/240/270 MPa and gamma_Mw 1.25; fire k_0.2proof,theta and k_E,theta rows, gamma_M,fi = 1.0, epsilon_res 0.3, alpha_c 25; Annex J detail categories 14 to 125 and gamma_Mf (DE 1.35, EN 1.0); shell fabrication class B (Q = 25) and imperfection parameters 0.65/0.60/0.40; deflection limit L/200 and stress limit 0.8 f_o (project values).

### 3D views

Extrusions from I, channel, angle and round-tube parametric profiles (box and RHS as polygons with a hole); HAZ bands (translucent, dotted-hatch weld role, leader with b_haz and rho_o,haz); the effective-section ghost with the local-buckling half-wave overlay and leaders with rho_c and t_eff; buckling and lateral-torsional mode overlay for the governing mode at utilisation 0.5 or more; failure glyphs; supports and lateral restraints; action glyphs (N, V_z, M_y, resultant of the line load); length and depth dimensions bound to fields; clause badge. Connections: connected plate, bolt or rivet grid, fillet weld, HAZ extent slab (b_haz from the document or from the table), selectable group. Cold-formed sheet (top-hat profile), shell (revolved cylinder with r/t label), fire scenario (thermal glyph at the member temperature), fatigue detail (label with Delta sigma_C).

### Diagrams (table twins, each bound to its report check)

Buckling (classes A and B, Euler hyperbola, y and z markers drag the buckling lengths), lateral-torsional buckling (classes 1/2 and 3/4), M_b,Rd over restraint spacing (marker drags L_LT), M-N polygon with ray and demand marker, classification (beta/epsilon against the three class limits), effective thickness rho_c over beta/epsilon, HAZ factors over plate thickness (markers drag the thickness), b_haz over thickness, S-N on log-log axes with seven neighbouring detail categories (demand marker drags the cycle count), fire (ISO 834 gas and aluminium temperature against theta_cr; marker drags the duration in minutes), k_0.2proof,theta and k_E,theta, shell chi_x over lambda_x.

### Editable handles

Member length, buckling lengths y and z, L_LT, depth, width, flange and web thickness (round tube: diameter and wall), one weld-position grip per welded plate. Tied plate widths follow the section (matched by value; a plate whose width was edited away from the formula is no longer dragged along). Connection pitch, gauge, edge distance, rows, per row, bolt diameter, plate thickness, weld length, throat, HAZ width. Sheet span, flat width, thickness. Shell radius, length, thickness. Fire duration and start temperature. Each leaf commits as its own `setField`.

### Worked examples

Shipped examples: aluminium roof purlin, non-compliant multi-fail.

1. Open "aluminium roof purlin". Select the member and drag the web thickness handle thinner: the effective-section ghost grows thinner strips, the class-4 warning appears and the M-N polygon shrinks.
2. Drag the L_LT handle in the lateral-torsional diagram: M_b,Rd changes and the utilisation colour follows.
3. In "non-compliant multi-fail" read the HAZ band labels and the connection HAZ slab, then use the remedy chips.

Reference values (oracle run in Python only): the compliant example has no utilisation above 0.93 after the example changes (largest: bolt edge distance 0.86 after the gauge change); the non-compliant example fails 45 checks. rho_c = 1 at beta3 for all 8 rows of Table 6.3.

## Deutsch

### Stand

Geschrieben, in Rust nie kompiliert (Berichte vom 02.11.2026). Nur das Python-Orakel lief (beide Beispiele und 16 gemeinsame Vektoren bestanden). Das mitgelieferte erfüllte Beispiel wurde so geändert, dass es erfüllt bleibt (Brandstart 20 Grad C und 30 s, Hüllspannung der Schale 12 MPa, Schraubenabstand quer 18 mm, Punktlast des Blechs 0).

### Was abgedeckt ist

Jeder Nachweissatz wird für jede EN-1990-Kombination ausgewertet (6.10a/b je führender Einwirkung, Gebrauchstauglichkeit, Brand) und je Nachweiskennung die höchste Ausnutzung behalten. Legierungsbänder (6082-T6 mit zwei Dickenbändern; eine unbekannte Legierung schlägt fehl); WEZ-Streifen (b_haz 20/30/35/40 mm) mit rho_o,haz und rho_u,haz; Klassifizierung nach beta/epsilon (Klasse 4 gibt eine Warnung, der wirksame Querschnitt gilt); wirksame Dicke rho_c mit Konstanten aus Tabelle 6.3; Zug, Druck, Biegung um beide Achsen (Formbeiwert auf 1,25 begrenzt), Querkraft, Biegung mit Querkraft, N-M-Interaktion (Klasse 1/2 Exponentenform, Klasse 3/4 linear mit delta M = N e_N); Biege- und Drillknicken (kappa für Längsnähte, Funktion omega_x); Biegedrillknicken (eigene Kurve); Interaktion 6.59/6.60; Schubbeulen unausgesteifter Stege; geschraubte und genietete Anschlüsse (Abscheren, Lochleibung, Zug, Durchstanzen, Abstände, Blockversagen, lange Anschlüsse); Schweißnähte (Schweißgut und WEZ-Grenze); Gebrauchstauglichkeit (Durchbiegung, Spannung, Frequenz); Ermüdung (EN 1999-1-3, Miner und Schwingbreitenprüfung); Brand (EN 1999-1-2: Bauteiltemperatur im Zeitschritt, k_0,2proof,theta, Knicken und Biegedrillknicken, kritische Temperatur); dünnwandiges kaltgeformtes Blech (1-4); Schalen (1-5).

### Nicht abgedeckt

Braucht Modellfelder, die es nicht gibt: ausgesteifte Elemente (A-06), Torsion (A-16), lokale Lasten und Stegkrüppeln (A-19, A-27, A-61), Steifen und Zugfeld (A-28; Stege gelten über die Bauteillänge als unausgesteift), die Verdrahtung von omega_x bei örtlichen Nähten (die Funktion existiert), geschweißte Schalen und Fertigungsklasse (A-72), ausgesteifte Schalen (A-71), WIG- und T-Stoß-Faktoren der WEZ, Bleche aus Versuchen. Braucht Tabellenwerte: Anhang H Verfahren B (A-23), genaue Tabellen für eta0/gamma0/xi0 und Exponenten, die Tabelle alpha_max, die kappa-Tabelle, die vollständige Tabelle 3.2 für alle Erzeugnisformen. Klebungen und Bolzenverbindungen sind nicht modelliert; Kriechen (A-81) liegt außerhalb. Das M-N-Polygon vernachlässigt M_z und die Verschiebung bei Klasse 4. Es gibt eine Nahtlage je Blech; der Profilumriss folgt aus der Profilart und den Blechelementen (Kasten und Rechteckrohr sind Polygone mit Loch). Eigenformen sind sinusförmige Andeutungen; die Biegedrillform hat keine Verdrehung.

### Ungeprüfte Werte (registriert mit `confirmed: false`)

Zeilen der Tabelle 3.2 außer 6082-T6 (6060, 6061, 6063, 5083; 6061 und 6063 weichen vom Spezifikationsansatz ab); Spalten für np, Duktilität und Erzeugnisform nicht modelliert; Grenzen der Tabelle 6.2 mit Schweißnähten und C1/C2 der Tabelle 6.3 (in sich konsistent: rho_c = 1 bei beta3 in allen 8 Zeilen); Rundrohrregel beta = 3 sqrt(D/t); alpha_max; kappa; Exponent omega_x; Torsionskurve; eta0 = gamma0 = xi0 = 0,8 mit Untergrenze 1; Interaktionsexponenten (I und U psi = 2, Rundrohr 1,5, Kasten und Winkel 1); Regel zum Schubbeulen; WEZ-Breiten bei MIG; Schrauben alpha_v 0,5 für 10.9, Spannungsquerschnitte, d0 = d + 2/3 mm, Durchstanzen d_m = 1,5 d, Niet f_ur 160 MPa, nichtrostend 700 MPa; Zusatzwerkstofffestigkeiten 190/240/270 MPa und gamma_Mw 1,25; Brand: Zeilen k_0,2proof,theta und k_E,theta, gamma_M,fi = 1,0, epsilon_res 0,3, alpha_c 25; Kerbfallkategorien 14 bis 125 aus Anhang J und gamma_Mf (DE 1,35, EN 1,0); Schalenfertigungsklasse B (Q = 25) und Imperfektionsparameter 0,65/0,60/0,40; Durchbiegungsgrenze L/200 und Spannungsgrenze 0,8 f_o (Projektwerte).

### 3D-Ansichten

Extrusionen aus parametrischen Profilen I, U, Winkel und Rundrohr (Kasten und Rechteckrohr als Polygone mit Loch); WEZ-Bänder (durchscheinend, gepunktete Schweißschraffur, Hinweislinie mit b_haz und rho_o,haz); das Geisterbild des wirksamen Querschnitts mit der Überlagerung lokaler Beulhalbwellen und Hinweislinien mit rho_c und t_eff; Überlagerung von Knick- und Biegedrillform für die maßgebende Form ab Ausnutzung 0,5; Versagenssymbole; Lager und seitliche Halterungen; Einwirkungssymbole (N, V_z, M_y, Resultierende der Streckenlast); an Felder gebundene Längen- und Höhenmaße; Abschnittsplakette. Anschlüsse: angeschlossenes Blech, Schrauben- oder Nietbild, Kehlnaht, WEZ-Ausdehnungsplatte (b_haz aus dem Dokument oder der Tabelle), wählbare Gruppe. Kaltgeformtes Blech (Hutprofil), Schale (Rotationszylinder mit r/t-Beschriftung), Brandszenario (Temperatursymbol bei der Bauteiltemperatur), Ermüdungskerbfall (Beschriftung mit Delta sigma_C).

### Diagramme (Tabellenzwillinge, jeweils an ihren Berichtsnachweis gebunden)

Knicken (Klassen A und B, Euler-Hyperbel, y- und z-Marker ziehen die Knicklängen), Biegedrillknicken (Klassen 1/2 und 3/4), M_b,Rd über dem Halterungsabstand (Marker zieht L_LT), M-N-Polygon mit Strahl und Beanspruchungsmarker, Klassifizierung (beta/epsilon gegen die drei Klassengrenzen), wirksame Dicke rho_c über beta/epsilon, WEZ-Faktoren über der Blechdicke (Marker ziehen die Dicke), b_haz über der Dicke, S-N in doppeltlogarithmischer Darstellung mit sieben benachbarten Kerbfällen (Beanspruchungsmarker zieht die Lastspielzahl), Brand (ISO-834-Gas- und Aluminiumtemperatur gegen theta_cr; Marker zieht die Dauer in Minuten), k_0,2proof,theta und k_E,theta, Schale chi_x über lambda_x.

### Editierbare Griffe

Stablänge, Knicklängen y und z, L_LT, Höhe, Breite, Flansch- und Stegdicke (Rundrohr: Durchmesser und Wand), je geschweißtem Blech ein Griff für die Nahtlage. Gekoppelte Blechbreiten folgen dem Profil (per Wertübereinstimmung; ein Blech, dessen Breite von der Formel weg bearbeitet wurde, wird nicht mehr mitgezogen). Anschluss: Abstand in Kraftrichtung und quer dazu, Randabstand, Reihen, je Reihe, Schraubendurchmesser, Blechdicke, Nahtlänge, Kehlnahtdicke, WEZ-Breite. Blech: Spannweite, abgewickelte Breite, Dicke. Schale: Radius, Länge, Dicke. Brand: Dauer und Starttemperatur. Jedes Blatt wird als eigener `setField` übernommen.

### Durchgerechnete Beispiele

Mitgelieferte Beispiele: Aluminium-Dachpfette, nicht erfüllt mit mehreren Fehlern.

1. "Aluminium-Dachpfette" öffnen. Den Stab wählen und den Stegdickengriff dünner ziehen: Das Geisterbild des wirksamen Querschnitts bekommt dünnere Streifen, die Warnung für Klasse 4 erscheint und das M-N-Polygon schrumpft.
2. Den L_LT-Griff im Biegedrilldiagramm ziehen: M_b,Rd ändert sich und die Ausnutzungsfarbe folgt.
3. In "nicht erfüllt mit mehreren Fehlern" die WEZ-Bandbeschriftungen und die WEZ-Platte am Anschluss lesen, dann die Abhilfe-Vorschläge nutzen.

Referenzwerte (Orakel nur in Python gelaufen): Das erfüllte Beispiel hat nach den Änderungen keine Ausnutzung über 0,93 (größte: Randabstand der Schraube 0,86 nach der Änderung des Querabstands); das nicht erfüllte Beispiel verfehlt 45 Nachweise. rho_c = 1 bei beta3 für alle 8 Zeilen der Tabelle 6.3.
