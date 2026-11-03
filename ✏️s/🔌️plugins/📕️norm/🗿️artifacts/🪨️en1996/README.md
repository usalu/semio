---
name: en1996
kind: user
---

# EN 1996 Masonry

Shared guide: [norm](../../README.md).

## English

### Status

Written, not yet compiled or run (audits of 2026-11-02). Only the Python oracle and a yield-line transliteration were executed.

### What is covered

In the wall document checks: slenderness 5.5.1, material conformance, compression at top, middle and bottom with Phi_m (Annex G) and eccentricities, concentrated loads 6.1.3 (60 degree spreading, bearing area check), shear 6.2 (f_vk with the EN cap or NA-DE f_vlt), lateral flexure 6.3 with earth pressure, reinforced walls 6.6, durability and bed joint, fire (EN 1996-1-2 tabulated), simplified method of EN 1996-3 with NA-DE limits and basement. New wall checks: unit fit and running-bond overlap, sill plus opening height, A_b against 0.45 A_ef, fire load. Document cases M-01, M-02, M-04, M-06, M-10, M-31, M-32, M-33, M-40, M-52, M-70 run from the document; the two alternative lateral models (yield lines, arching) only warn and never fail.

Catalogue only: M-03, M-05, M-07, M-11 to M-16, M-20, M-21, M-30, M-34, M-41 to M-44, M-50, M-51, M-60, M-61, M-80 to M-83, M-90, M-91, M-95 (cavity thickness, piers, flanged sections, ties, anchors, confined and prestressed masonry, seismic q ranges).

### Not covered

Not model entities: confined and prestressed masonry, ties, anchors and ring ties, flanged shear walls and bracing layout, piers and lintels with positions, anchorage bars, shear reinforcement and deep beams, frost class and cover, deflection, soil data for the basement rule, isotherm depth for fire. Annex E fixed-edge tables other than S-S-S-S, S-S-S-F and two free edges are not transcribed; the yield-line solver gives the collapse load (square panel: 24 m/l^2, all edges fixed 48, SSFF 8, XXFF 16, FXFF 2) but not the verbatim table coefficients. Concentrated loads assume a1 = 0. Not drawn: cavity walls, ring beams, header, English and Flemish bonds, shear-wall compression zones and overturn line, local crushing region, anchorage length, plaster equivalent in fire, moment contours, the arching thrust line. Opening and concentrated-load positions are rules of the layout, not document data.

### Unverified values

49 registered ids (EN and DE text): delta shape factor table; K tables (clay groups 3 and 4, thin layer, lightweight, NA-DE computed with k, alpha, beta); f_k caps; f_vk0 table (0.30/0.20/0.10, thin layer 0.30, lightweight 0.15); NA-DE f_vlt = 0.45 f_bt,cal sqrt(1 + sigma/f_bt,cal) with f_bt,cal = 0.026 f_b and the EN cap 0.065 f_b; f_xk table; bond strength; K_E and G; creep and shrinkage; gamma_M by execution class and gamma_S; rho_2, wall-stiffening and slenderness limits; Phi_2; pier and lintel rules; flange overhang; accidental eccentricity; the yield-line replacement of Annex E coefficients; arching coefficient and l_a/t at most 25; EN 1996-3 limits and N_min; reinforced, confined and prestressed limits; tie densities; movement joints; frost and cover; l/d limits; execution limits; Phi_s; fire rows (all except clay group 1 come from the earlier code); mu_fi scaling; q ranges; psi factors; mortar requirement by exposure. The check rows state these ids.

### 3D views

Per wall: three utilisation zones (bottom, middle, top) as cut solids bound to the compression checks; window plates (glass) and door plates; concrete lintels with 0.25 m bearing; floor plate with bearing depth; base and held-edge supports by support sides; dimensions for length, height, thickness; failure glyphs (crushing, sliding, buckling). A hidden `bond` layer shows explicit running bond (half-unit offset, mortar joints inset); the overlap rule max(0.4 hu, 45 mm) colours an invalid bond in the error style; beyond 20 000 units only joint lines are drawn. Loads: resultant N_Ed at top and bottom with eccentricity (declared + bearing + rotation, at least 0.05 t), concentrated loads with the 60 degree spreading zone, wind pressure, indicative yield-line regions. Reinforcement: vertical bars and bed-joint layers.

### Diagrams (table twins)

Eccentricity over height (limits 0.05 t and 0.2 t; markers drag the declared eccentricities), Phi at top, middle, bottom, capacity N_Ed against N_Rd, slenderness Phi_m over h_ef/t, f_k over f_b, shear f_vk, lateral alpha over h/l, lateral capacity w_Rd over h/l, fire t_min over the fire class, M-N curve of reinforced walls.

### Editable handles

Wall length, height, thickness, held sides (stepper); opening width, height, sill; floor bearing depth and span; eccentricity at top and bottom; concentrated bearing length; fire class (diagram marker). Placement is not persisted.

### Worked examples

Shipped examples: load-bearing wall, multi-fail masonry.

1. Open "load-bearing wall" and drag the wall height handle: Phi at the middle falls, the middle zone changes colour toward the warning ramp, the slenderness marker moves. The upper limit h/t is checked.
2. Widen an opening with its width handle: the lintel bearing and the pier compression zones update.
3. Open "multi-fail masonry" and step through failing rows with ]; each remedy is solved on the real check.

Reference values (tests not yet run): Annex E closed forms SSSS 24, XXXX 48, SSFF 8, XXFF 16, FXFF 2 (times m/l^2); yield-line check values 60000, 20000 and 29536.67 Pa agree between Rust and the Python transliteration; fire thickness for clay group 1 from the table.

## Deutsch

### Stand

Geschrieben, noch nicht kompiliert oder ausgeführt (Berichte vom 02.11.2026). Nur das Python-Orakel und eine Bruchlinien-Transliteration wurden ausgeführt.

### Was abgedeckt ist

In den Wand-Dokumentnachweisen: Schlankheit 5.5.1, Baustoffkonformität, Druck oben, in der Mitte und unten mit Phi_m (Anhang G) und Exzentrizitäten, Einzellasten 6.1.3 (60-Grad-Ausbreitung, Prüfung der Aufstandsfläche), Schub 6.2 (f_vk mit EN-Kappung oder NA-DE f_vlt), Biegung quer 6.3 mit Erddruck, bewehrte Wände 6.6, Dauerhaftigkeit und Lagerfuge, Brand (EN 1996-1-2 tabellarisch), vereinfachtes Verfahren der EN 1996-3 mit NA-DE-Grenzen und Keller. Neue Wandnachweise: Steinpassung und Überbindung, Brüstung plus Öffnungshöhe, A_b gegen 0,45 A_ef, Brandlast. Dokumentfälle M-01, M-02, M-04, M-06, M-10, M-31, M-32, M-33, M-40, M-52, M-70 laufen aus dem Dokument; die beiden alternativen Quermodelle (Bruchlinien, Gewölbewirkung) warnen nur und schlagen nie fehl.

Nur Katalog: M-03, M-05, M-07, M-11 bis M-16, M-20, M-21, M-30, M-34, M-41 bis M-44, M-50, M-51, M-60, M-61, M-80 bis M-83, M-90, M-91, M-95 (Hohlraumdicke, Pfeiler, Flanschquerschnitte, Anker, umschnürtes und vorgespanntes Mauerwerk, Erdbeben-q-Bereiche).

### Nicht abgedeckt

Keine Modellobjekte: umschnürtes und vorgespanntes Mauerwerk, Zuganker, Anker und Ringanker, Schubwände mit Flanschen und Aussteifungsanordnung, Pfeiler und Stürze mit Lage, Verankerungsstäbe, Schubbewehrung und wandartige Träger, Frostklasse und Deckung, Durchbiegung, Bodendaten für die Kellerregel, Isothermentiefe im Brandfall. Die Tabellen des Anhangs E für eingespannte Ränder außer S-S-S-S, S-S-S-F und zwei freien Rändern sind nicht übertragen; der Bruchlinienlöser liefert die Traglast (Quadratplatte: 24 m/l^2, alle Ränder eingespannt 48, SSFF 8, XXFF 16, FXFF 2), aber nicht die wörtlichen Tabellenbeiwerte. Einzellasten nehmen a1 = 0 an. Nicht gezeichnet: zweischalige Wände, Ringbalken, Binder-, englischer und flämischer Verband, Druckzone und Kipplinie von Schubwänden, lokaler Druckbereich, Verankerungslänge, Putzäquivalent im Brandfall, Momentenlinien, die Gewölbedrucklinie. Lage von Öffnungen und Einzellasten sind Layoutregeln, keine Dokumentdaten.

### Ungeprüfte Werte

49 registrierte Kennungen (EN- und DE-Text): Tabelle des Formbeiwerts delta; K-Tabellen (Ziegelgruppen 3 und 4, Dünnbettmörtel, Leichtmörtel, NA-DE mit k, alpha, beta berechnet); f_k-Kappungen; f_vk0-Tabelle (0,30/0,20/0,10, Dünnbett 0,30, Leichtmörtel 0,15); NA-DE f_vlt = 0,45 f_bt,cal sqrt(1 + sigma/f_bt,cal) mit f_bt,cal = 0,026 f_b und die EN-Kappung 0,065 f_b; f_xk-Tabelle; Haftscherfestigkeit; K_E und G; Kriechen und Schwinden; gamma_M nach Ausführungsklasse und gamma_S; rho_2, Aussteifungs- und Schlankheitsgrenzen; Phi_2; Pfeiler- und Sturzregeln; Flanschüberstand; ungewollte Ausmitte; der Bruchlinienersatz für die Beiwerte aus Anhang E; Gewölbewirkung und l_a/t höchstens 25; Grenzen der EN 1996-3 und N_min; Grenzen für bewehrtes, umschnürtes und vorgespanntes Mauerwerk; Ankerdichten; Bewegungsfugen; Frost und Deckung; l/d-Grenzen; Ausführungsgrenzen; Phi_s; Brandzeilen (außer Ziegelgruppe 1 aus dem früheren Code); mu_fi-Skalierung; q-Bereiche; psi-Beiwerte; Mörtelanforderung nach Exposition. Die Nachweiszeilen nennen diese Kennungen.

### 3D-Ansichten

Je Wand: drei Ausnutzungszonen (unten, Mitte, oben) als geschnittene Körper, gebunden an die Drucknachweise; Fensterflächen (Glas) und Türflächen; Betonstürze mit 0,25 m Auflager; Deckenplatte mit Auflagertiefe; Fuß- und Randlager nach Haltungsseiten; Maße für Länge, Höhe, Dicke; Versagenssymbole (Zerdrücken, Gleiten, Knicken). Eine ausgeblendete Ebene `bond` zeigt den expliziten Läuferverband (Halbsteinversatz, eingerückte Mörtelfugen); die Überbindungsregel max(0,4 hu, 45 mm) färbt einen ungültigen Verband im Fehlerstil; über 20 000 Steinen werden nur Fugenlinien gezeichnet. Lasten: Resultierende N_Ed oben und unten mit Ausmitte (angegeben + Auflager + Verdrehung, mindestens 0,05 t), Einzellasten mit 60-Grad-Ausbreitungszone, Winddruck, anschauliche Bruchlinienbereiche. Bewehrung: senkrechte Stäbe und Lagerfugenbewehrung.

### Diagramme (Tabellenzwillinge)

Ausmitte über der Höhe (Grenzen 0,05 t und 0,2 t; Marker ziehen die angegebenen Ausmitten), Phi oben, Mitte, unten, Tragfähigkeit N_Ed gegen N_Rd, Schlankheit Phi_m über h_ef/t, f_k über f_b, Schub f_vk, Querbeanspruchung alpha über h/l, Quertragfähigkeit w_Rd über h/l, Brand t_min über der Feuerwiderstandsklasse, M-N-Kurve bewehrter Wände.

### Editierbare Griffe

Wandlänge, -höhe, -dicke, gehaltene Seiten (Schrittschalter); Öffnungsbreite, -höhe, Brüstungshöhe; Auflagertiefe und Spannweite der Decke; Ausmitte oben und unten; Einzellastauflagerlänge; Feuerwiderstandsklasse (Diagramm-Marker). Die Platzierung wird nicht gespeichert.

### Durchgerechnete Beispiele

Mitgelieferte Beispiele: tragende Wand, Mauerwerk mit mehreren Fehlern.

1. "Tragende Wand" öffnen und den Wandhöhengriff ziehen: Phi in der Mitte fällt, die mittlere Zone wandert zur Warnskala, der Schlankheitsmarker bewegt sich. Die obere Grenze h/t wird geprüft.
2. Eine Öffnung mit ihrem Breitengriff verbreitern: Sturzauflager und Druckzonen der Pfeiler aktualisieren sich.
3. "Mauerwerk mit mehreren Fehlern" öffnen und mit ] die nicht erfüllten Zeilen durchgehen; jede Abhilfe wird am echten Nachweis gelöst.

Referenzwerte (Tests noch nicht ausgeführt): Geschlossene Lösungen des Anhangs E: SSSS 24, XXXX 48, SSFF 8, XXFF 16, FXFF 2 (mal m/l^2); Bruchlinienwerte 60000, 20000 und 29536,67 Pa stimmen zwischen Rust und der Python-Transliteration überein; Brandwanddicke für Ziegelgruppe 1 aus der Tabelle.
