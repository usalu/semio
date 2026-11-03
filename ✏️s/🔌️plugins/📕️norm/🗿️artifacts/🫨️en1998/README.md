---
name: en1998
kind: user
---

# EN 1998 Earthquake resistance

Shared guide: [norm](../../README.md).

## English

### Status

Partial. Written, not yet compiled or run (audits of 2026-11-02); the test run was never reached.

### What is covered

Part 1 building checks: design spectrum (all four branches, beta = 0.2) and damping correction, vertical spectrum, base shear F_b with gamma_I, q and lambda, force distribution F_i, fundamental period (C_t, given, Rayleigh), accidental torsion delta = 1 + 0.6 x/L_e (simplified procedure only if the plan is regular), elevation regularity (soft storey below 0.70 of the storey above, q reduced by 20 %), drift d_r nu / (theta h), P-delta tiers, radiation damping k_f/k_s of at least 1.5, component combination E_x + 0.3 E_y, masonry simplified rule. Parts 2 to 6 simplified checks: bridges (isolated or fixed, bearing displacement), assessment (return-period factor, knowledge levels), silos and tanks (Housner), towers and chimneys, foundations, retaining walls. A zero resistance with a positive demand now fails. The NA-DE zone and ground combination are model inputs.

Functions without model input (no check yet): seismic joint gap, non-structural elements, strong-column ratio, modal analysis (Jacobi, CQC, SRSS).

### Not covered

No inputs yet for: v_s30 ground classification, regularity from geometry, seismic joints, non-structural elements, capacity design (strong column, beam and column shear, joints, steel), timber and masonry member checks, Part 2 piers, Part 3 chord rotation, Part 5 liquefaction, Annex F bearing and piles, Part 6 Rayleigh period. The capacity-design ratio is not displayed; joint boxes sit at every column head and take the colour of the governing member detailing check. A geographic NA-DE zone map needs a licensed municipality dataset and is replaced by a zone chart. Torsional radius ellipse, pushover capacity curve and q chart are not implemented.

### Unverified values

Torsion bound delta at most 1.6 (own bound, no clause); given T1 at most 1.5 x C_t estimate and k_f/k_s of at least 1.5; the NA.4 table, where the repository has B-T = 1.25 and C-T = 1.50 and the specification B-T = 1.00 and C-T = 1.25 (conflict, left unchanged; a warning `en1998.site.ndp.<id>` appears for the DE combinations B-T and C-T); vertical spectrum a_vg/a_g = 0.9 with T_B/T_C/T_D = 0.05/0.15/1.0 for type 1 (NA-DE value unverified); q_max table, psi_2 for snow, masonry ratios, nu, CF factors, silo, tank and tower constants. The spectrum floor of the diagram (beta x gamma_I x a_gR) differs from the check (beta x a_gR); logged for the norm owner. After the changes the non-compliant examples may fail different checks than before.

### 3D views

Buildings: plan footprint with bound L_x and L_y; storey slabs coloured by the storey-force, drift and P-delta checks with soft-storey highlight (error hatch and failure glyph) and bound height; column grid; capacity-design joint boxes at every column head; mass glyph per storey; centres of mass and stiffness with eccentricity e_0 and accidental eccentricity e_ai; lateral force arrows per system (F_i x delta, shifted by e_ai); first mode shape overlay; drift bars per storey; seismic joints between neighbours coloured by gap against sqrt(d1^2 + d2^2). Silos and tanks as revolved shells with content and base-shear arrow; towers and chimneys as vertical tubes with overturning arrow; footings, retaining walls with seismic thrust, bridge deck on two piers with roller bearings.

### Diagrams (table twins)

Spectra (S_e, S_d, T_B, T_C, T_D, 2T_C, 4T_C, T1 marker with S_d, F_b; the marker drags the given T1 when the method is `given`), vertical spectrum, drift per building (limit 1.0, per-storey markers drag the drift fields), NA-DE zone chart (a_gR over zones 0 to 3; the site marker steps the zone).

### Editable handles

Plan width and length, storey height, storey permanent action (mass), centre of mass x and y, centre of stiffness x and y, tower height, silo and tank height and radius, wall height. Placement is not stored. 93 leaves have edit rules.

### Worked examples

Shipped examples: seismic RC frame, seismic multi-part, and a failing variant of each.

1. Open "seismic RC frame". In the spectra diagram drag nothing (T1 is computed); instead, drag the storey height handle: T1, S_d, F_b and the drift bars update.
2. Move the centre of stiffness handle away from the centre of mass: e_0 grows, the torsion factor changes and the force arrows shift.
3. Open the failing variant and read the soft-storey highlight; the remedy raises the lower storey stiffness.

Reference values (tests not yet run): spectra at every sampled period, F_b, F_i = F_b m z / sum(m z) and the first eigenmode (numpy eigh) are compared with an independent oracle; drift utilisation d_r nu/(theta h).

## Deutsch

### Stand

Teilweise. Geschrieben, noch nicht kompiliert oder ausgeführt (Berichte vom 02.11.2026); der Testlauf wurde nie erreicht.

### Was abgedeckt ist

Gebäudenachweise nach Teil 1: Bemessungsspektrum (alle vier Zweige, beta = 0,2) und Dämpfungskorrektur, vertikales Spektrum, Basisquerkraft F_b mit gamma_I, q und lambda, Verteilung der Kräfte F_i, Grundperiode (C_t, angegeben, Rayleigh), zufällige Torsion delta = 1 + 0,6 x/L_e (vereinfachtes Verfahren nur bei regulärem Grundriss), Regelmäßigkeit im Aufriss (weiches Geschoss unter 0,70 des darüberliegenden, q um 20 % abgemindert), Verschiebung d_r nu / (theta h), P-Delta-Stufen, Strahlungsdämpfung k_f/k_s mindestens 1,5, Komponentenkombination E_x + 0,3 E_y, vereinfachte Regel für Mauerwerk. Vereinfachte Nachweise der Teile 2 bis 6: Brücken (isoliert oder fest, Lagerverschiebung), Bewertung (Wiederkehrperiodenfaktor, Kenntnisstufen), Silos und Tanks (Housner), Türme und Schornsteine, Gründungen, Stützwände. Eine Beanspruchbarkeit null bei positiver Beanspruchung schlägt nun fehl. Die NA-DE-Zone und die Baugrundkombination sind Modelleingaben.

Funktionen ohne Modelleingabe (noch kein Nachweis): Fugenbreite zwischen Gebäuden, nichttragende Bauteile, Verhältnis starke Stütze-schwacher Riegel, Modalanalyse (Jacobi, CQC, SRSS).

### Nicht abgedeckt

Noch keine Eingaben für: Baugrundklassifizierung v_s30, Regelmäßigkeit aus der Geometrie, Erdbebenfugen, nichttragende Bauteile, Kapazitätsbemessung (starke Stütze, Riegel- und Stützenquerkraft, Knoten, Stahl), Holz- und Mauerwerksbauteile, Pfeiler in Teil 2, Sehnendrehwinkel in Teil 3, Bodenverflüssigung in Teil 5, Grundbruch und Pfähle in Anhang F, Rayleigh-Periode in Teil 6. Das Verhältnis der Kapazitätsbemessung wird nicht angezeigt; Knotenkästen sitzen an jedem Stützenkopf und erhalten die Farbe des maßgebenden Bauteilnachweises. Eine geografische NA-DE-Zonenkarte braucht einen lizenzierten Gemeindedatensatz und wird durch ein Zonendiagramm ersetzt. Torsionsradius-Ellipse, Pushover-Kapazitätskurve und q-Diagramm sind nicht umgesetzt.

### Ungeprüfte Werte

Torsionsgrenze delta höchstens 1,6 (eigene Grenze, keine Klausel); angegebenes T1 höchstens 1,5 x C_t-Schätzung und k_f/k_s mindestens 1,5; die Tabelle NA.4, bei der das Repository B-T = 1,25 und C-T = 1,50 hat, die Spezifikation B-T = 1,00 und C-T = 1,25 (Widerspruch, unverändert; für die DE-Kombinationen B-T und C-T erscheint eine Warnung `en1998.site.ndp.<id>`); vertikales Spektrum a_vg/a_g = 0,9 mit T_B/T_C/T_D = 0,05/0,15/1,0 für Typ 1 (NA-DE-Wert ungeprüft); q_max-Tabelle, psi_2 für Schnee, Mauerwerksverhältnisse, nu, CF-Faktoren, Silo-, Tank- und Turmkonstanten. Der Spektrumsboden im Diagramm (beta x gamma_I x a_gR) weicht vom Nachweis (beta x a_gR) ab; für den Normeigentümer vermerkt. Nach den Änderungen können die nicht erfüllten Beispiele andere Nachweise verfehlen als zuvor.

### 3D-Ansichten

Gebäude: Grundriss mit gebundenem L_x und L_y; Geschossdecken, gefärbt nach den Nachweisen für Geschosskräfte, Verschiebung und P-Delta mit Hervorhebung weicher Geschosse (Fehlerschraffur und Versagenssymbol) und gebundener Höhe; Stützenraster; Knotenkästen der Kapazitätsbemessung an jedem Stützenkopf; Massensymbol je Geschoss; Massen- und Steifigkeitsmittelpunkt mit Ausmitte e_0 und zufälliger Ausmitte e_ai; Horizontalkraftpfeile je System (F_i x delta, um e_ai verschoben); Überlagerung der ersten Eigenform; Verschiebungsbalken je Geschoss; Erdbebenfugen zwischen Nachbarn, gefärbt nach Fugenbreite gegen sqrt(d1^2 + d2^2). Silos und Tanks als Rotationsschalen mit Inhalt und Basisquerkraftpfeil; Türme und Schornsteine als senkrechte Rohre mit Kipppfeil; Fundamente, Stützwände mit seismischem Schub, Brückenplatte auf zwei Pfeilern mit Rollenlagern.

### Diagramme (Tabellenzwillinge)

Spektren (S_e, S_d, T_B, T_C, T_D, 2T_C, 4T_C, T1-Marker mit S_d, F_b; der Marker zieht das angegebene T1, wenn das Verfahren `given` ist), vertikales Spektrum, Verschiebung je Gebäude (Grenze 1,0, Marker je Geschoss ziehen die Verschiebungsfelder), NA-DE-Zonendiagramm (a_gR über den Zonen 0 bis 3; der Standortmarker schaltet die Zone).

### Editierbare Griffe

Grundrissbreite und -länge, Geschosshöhe, ständige Geschosseinwirkung (Masse), Massenmittelpunkt x und y, Steifigkeitsmittelpunkt x und y, Turmhöhe, Silo- und Tankhöhe und -radius, Wandhöhe. Die Platzierung wird nicht gespeichert. 93 Blätter haben Bearbeitungsregeln.

### Durchgerechnete Beispiele

Mitgelieferte Beispiele: seismischer Stahlbetonrahmen, seismisch mehrteilig und je eine nicht erfüllte Variante.

1. "Seismischer Stahlbetonrahmen" öffnen. Im Spektrumdiagramm lässt sich nichts ziehen (T1 wird berechnet); stattdessen den Geschosshöhengriff ziehen: T1, S_d, F_b und die Verschiebungsbalken aktualisieren sich.
2. Den Steifigkeitsmittelpunkt vom Massenmittelpunkt wegbewegen: e_0 wächst, der Torsionsfaktor ändert sich und die Kraftpfeile verschieben sich.
3. Die nicht erfüllte Variante öffnen und die Hervorhebung des weichen Geschosses lesen; die Abhilfe erhöht die Steifigkeit des unteren Geschosses.

Referenzwerte (Tests noch nicht ausgeführt): Spektren an jeder Stützstelle, F_b, F_i = F_b m z / Summe(m z) und die erste Eigenform (numpy eigh) werden mit einem unabhängigen Orakel verglichen; Verschiebungsausnutzung d_r nu/(theta h).
