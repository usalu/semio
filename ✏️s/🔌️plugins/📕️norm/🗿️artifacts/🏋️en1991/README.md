---
name: en1991
kind: user
---

# EN 1991 Actions on structures

Shared guide: [norm](../../README.md).

## English

### Status

Written, not yet compiled or run (audits of 2026-11-02). The 96-entry catalogue is not yet reachable from a window; the document checks run on the existing model fields.

### What is covered

Document checks (same formulas as the catalogue): 1-1 imposed loads, self weight, densities; 1-2 fire; 1-3 snow; 1-4 wind with signed pressures (suction against both internal-pressure cases); 1-5 thermal; 1-6 construction; 1-7 accidental impact and explosion; Part 2 traffic on bridges (lanes, LM1 to LM4, load groups gr1a to gr5, footbridge crowd); Part 3 cranes; Part 4 silos.

Catalogue entries (function level): E11.01 to E11.09, E12.01 to E12.10 (ISO 834, external and hydrocarbon curves, Annex A parametric fire with validity ranges, Annex E fire load, Annex F, Annex C localised fire), E13.01 to E13.12 (snow incl. NA-DE zones, drifts, guards), E14.01 to E14.16 (wind profile, c_pe for walls and flat and pitched roofs, internal pressure, force coefficients, structural factor, vortex shedding), E15.01 to E15.08, E16.01 to E16.03 and E16.06, E17.01, E17.02, E17.05 to E17.10, E2.01 to E2.17, E3.01 to E3.05 and E3.07, E4.01 to E4.04 and E4.08.

### Not covered

Hipped-roof coefficients (Table 7.5: the zone geometry F, G, H, I, K, L is modelled, the cells are unverified registry rows that stay unset, so the checks report awaiting values) and wind direction 90 degrees; free-standing walls and canopies; digitised figures c_f,0 and psi_lambda (stay inputs); actions during execution (accidental, imperfections); derailment and ship impact; FLM2/FLM4/FLM5 vehicle sets; crane load groups other than group 1; hopper, thermal, dust explosion, wind on silos and discharge eccentricity of silos. Not drawn for lack of model data: draggable tandem positions, crane position, rail LM71/SW, wind-face zone letter, drift obstacle plan position.

### Unverified values

Every catalogue outcome lists its unverified basis ids. Main groups: Annex A densities and the steel value 77.0 (EN) against 78.5 (DE); the NA-DE imposed-load Table 6.1DE and the partition bands; alpha_A; fire-curve exponent 3.8, parametric t_lim and ranges, fire-load factors; NA-DE snow zone formulas, drift and overhang constants; NA-DE wind profile coefficients and the simplified q_p table, c_pe tables 7.3a/7.4a, parapet/curved/mansard rows, Annex B damping and n1 = 46/h; thermal deck shifts; impact table values, tie and blast constants; NA-DE alpha factors of LM1 (interim 1.0), LM3 axle line, rail values; crane phi factors; silo constants (patch factor 0.2, C_h, C_w, K, mu).

Two disagreements with the specification were resolved in favour of the standard: example E14-12 (the code gives c_s c_d = 0.8444) and E13-10 (1.10691 against 1.10768).

### 3D views

Translucent building prism with roof body (flat, mono, duo, hip) and bound dimensions; wind zones A to J as pressure glyphs with q_p x c_pe, net arrows for c_pi +0.2 and -0.3; snow mu1 per slope, alternative arrangements (hidden layer), parapets, obstructions, drift areas; floors coloured by category with the reduced load; self-weight stack by unit weight; temperature glyphs (uniform and gradient); fire compartment with gas temperature; impact and explosion glyphs; bridge deck with lanes, LM1 tandems and UDL, footways, LM2 to LM4 and crowd (hidden layer); crane runway wheel loads; silo and tank pressure, friction and patch load. The crane runway geometry uses schematic constants.

### Diagrams (table twins, draggable markers)

q_p profile, c_pe by area, snow mu, snow mu3, fire curves, bridge influence, silo pressure.

### Editable handles

Building height, width, depth, storeys, altitude, wind zone, terrain; roof pitch, parapet, obstruction; floor area; layer thickness; face z and loaded area; bridge span, lane width, lane; crane hoisting speed; silo height and radius; fire compartment height. Edit rules are still missing for roof pitch, parapet and drift obstruction height, floor area, self-weight thickness and wind face z / loaded area; a drag on these fields is refused until they exist.

### Worked examples

Shipped examples: DE office (compliant) and a multi-fail non-compliant model.

1. Open the compliant office. Drag the building height handle up: q_p(z), the wind pressure glyphs and the snow/wind checks update; compare with the q_p diagram marker.
2. Open the multi-fail model and use ] to step through failing checks; each failing input check offers the list of admissible values as remedy chips.

Reference values (specification and oracle, not yet run): standard fire curve 945.3 degrees C at 60 min (20 + 345 log10(8t + 1)); Janssen pressure p_ho = gamma a / mu checked against an ODE force balance; E14-12 gives R^2 = 0.03856, nu = 0.2939 Hz, k_p = 3.403, c_s c_d = 0.8444.

## Deutsch

### Stand

Geschrieben, noch nicht kompiliert oder ausgeführt (Berichte vom 02.11.2026). Der Katalog mit 96 Einträgen ist aus keinem Fenster erreichbar; die Dokumentnachweise laufen auf den vorhandenen Modellfeldern.

### Was abgedeckt ist

Dokumentnachweise (gleiche Formeln wie der Katalog): 1-1 Nutzlasten, Eigengewicht, Wichten; 1-2 Brand; 1-3 Schnee; 1-4 Wind mit vorzeichenbehafteten Drücken (Sog gegen beide Innendruckfälle); 1-5 Temperatur; 1-6 Bauzustände; 1-7 außergewöhnlich Anprall und Explosion; Teil 2 Verkehr auf Brücken (Fahrspuren, LM1 bis LM4, Lastgruppen gr1a bis gr5, Fußgängermenge); Teil 3 Krane; Teil 4 Silos.

Katalogeinträge (Funktionsebene): E11.01 bis E11.09, E12.01 bis E12.10 (ISO 834, Außenbrand- und Kohlenwasserstoffkurve, parametrischer Brand nach Anhang A mit Gültigkeitsbereichen, Brandlast Anhang E, Anhang F, lokaler Brand Anhang C), E13.01 bis E13.12 (Schnee inkl. NA-DE-Zonen, Verwehungen, Schneefanggitter), E14.01 bis E14.16 (Windprofil, c_pe für Wände sowie Flach- und Satteldächer, Innendruck, Kraftbeiwerte, Strukturbeiwert, Wirbelablösung), E15.01 bis E15.08, E16.01 bis E16.03 und E16.06, E17.01, E17.02, E17.05 bis E17.10, E2.01 bis E2.17, E3.01 bis E3.05 und E3.07, E4.01 bis E4.04 und E4.08.

### Nicht abgedeckt

Walmdächer (Tabelle 7.5) und Windrichtung 90 Grad; freistehende Wände und Vordächer; digitalisierte Bilder c_f,0 und psi_lambda (bleiben Eingaben); Einwirkungen während der Ausführung (außergewöhnlich, Imperfektionen); Entgleisung und Schiffsanprall; Fahrzeugsätze FLM2/FLM4/FLM5; Kranlastgruppen außer Gruppe 1; Trichter, Temperatur, Staubexplosion, Wind auf Silos und Entleerungsausmitte bei Silos. Mangels Modelldaten nicht gezeichnet: ziehbare Tandempositionen, Kranposition, Bahn LM71/SW, Zonenbuchstabe der Windfläche, Lage des Verwehungshindernisses, Walmdächer.

### Ungeprüfte Werte

Jedes Katalogergebnis führt seine ungeprüften Kennungen auf. Hauptgruppen: Wichten nach Anhang A und der Stahlwert 77,0 (EN) gegenüber 78,5 (DE); die NA-DE-Nutzlasttabelle 6.1DE und die Trennwandbänder; alpha_A; Brandkurven-Exponent 3,8, parametrisches t_lim und Bereiche, Brandlastfaktoren; NA-DE-Schneezonenformeln, Verwehungs- und Überstandskonstanten; NA-DE-Windprofilbeiwerte und die vereinfachte q_p-Tabelle, c_pe-Tabellen 7.3a/7.4a, Zeilen Attika/gekrümmt/Mansarde, Dämpfung in Anhang B und n1 = 46/h; Temperaturverschiebungen der Brückenplatte; Anprallwerte, Anprall-, Zug- und Explosionskonstanten; NA-DE-alpha-Faktoren von LM1 (vorläufig 1,0), Achslinie LM3, Bahnwerte; Kran-phi-Beiwerte; Silokonstanten (Patchfaktor 0,2, C_h, C_w, K, mu).

Zwei Abweichungen von der Spezifikation wurden zugunsten der Norm entschieden: Beispiel E14-12 (der Code liefert c_s c_d = 0,8444) und E13-10 (1,10691 gegenüber 1,10768).

### 3D-Ansichten

Durchscheinender Gebäudekörper mit Dachkörper (Flach-, Pult-, Sattel-, Walmdach) und gebundenen Maßen; Windzonen A bis J als Drucksymbole mit q_p x c_pe, Nettopfeile für c_pi +0,2 und -0,3; Schnee mu1 je Dachfläche, alternative Anordnungen (ausgeblendete Ebene), Attiken, Hindernisse, Verwehungsflächen; Decken nach Kategorie gefärbt mit der abgeminderten Last; Eigengewichtsstapel nach Wichte; Temperatursymbole (gleichmäßig und Gradient); Brandabschnitt mit Gastemperatur; Anprall- und Explosionssymbole; Brückenplatte mit Fahrstreifen, LM1-Tandems und Gleichlast, Gehwegen, LM2 bis LM4 und Menschenmenge (ausgeblendete Ebene); Raddrücke der Kranbahn; Silo- und Tankdruck, Wandreibung und Teilflächenlast. Die Kranbahngeometrie nutzt schematische Konstanten.

### Diagramme (Tabellenzwillinge, ziehbare Marker)

q_p-Profil, c_pe nach Fläche, Schnee mu, Schnee mu3, Brandkurven, Brückeneinflusslinie, Silodruck.

### Editierbare Griffe

Gebäudehöhe, -breite, -tiefe, Geschosszahl, Geländehöhe, Windzone, Geländekategorie; Dachneigung, Attika, Hindernis; Deckenfläche; Schichtdicke; Fläche z und Lastfläche; Brückenspannweite, Fahrstreifenbreite, Fahrstreifen; Kran-Hubgeschwindigkeit; Silohöhe und -radius; Höhe des Brandabschnitts. Bearbeitungsregeln fehlen noch für Dachneigung, Attika und Hindernishöhe, Deckenfläche, Eigengewichtsdicke sowie z und Lastfläche der Windfläche; ein Zug an diesen Feldern wird abgelehnt, bis sie existieren.

### Durchgerechnete Beispiele

Mitgelieferte Beispiele: Büro DE (erfüllt) und ein nicht erfülltes Modell mit mehreren Fehlern.

1. Das erfüllte Büro öffnen. Den Höhengriff des Gebäudes nach oben ziehen: q_p(z), die Winddrucksymbole und die Schnee- und Windnachweise aktualisieren sich; mit dem Marker im q_p-Diagramm vergleichen.
2. Das Modell mit mehreren Fehlern öffnen und mit ] die nicht erfüllten Nachweise durchgehen; jeder Eingabenachweis bietet die zulässigen Werte als Abhilfe-Vorschläge.

Referenzwerte (Spezifikation und Orakel, noch nicht ausgeführt): Einheits-Temperaturzeitkurve 945,3 Grad C bei 60 min (20 + 345 log10(8t + 1)); Janssen-Druck p_ho = gamma a / mu gegen ein Gleichgewicht per Differentialgleichung geprüft; E14-12 ergibt R^2 = 0,03856, nu = 0,2939 Hz, k_p = 3,403, c_s c_d = 0,8444.
