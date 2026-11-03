# 🧭 Spec-trace description review (TRACE-TEXTS, wave 2)

Scope: EN 1990, 1991, 1992, 1993, 1994, 1996, 1997, 1998, 1999 (EN 1995 was rewritten from the spec by I18N-FIX v2 and is not touched). Only the trace JSONs (`🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧭️spec-trace/🔣️.json`) and their regenerated Rust twins changed. No cargo, no git writes. `- ready for rebuild: trace texts` appended to `build-en{1990,1991,1992,1993,1994,1996,1997,1998,1999}/latest.md`.

Verifier: `codegen/🐍️.py --write` then without `--write`: en1990 29, en1991 71, en1992 159, en1993 142, en1994 54, en1995 68, en1996 37, en1997 42, en1998 32, en1999 42, all ok.

## Findings
- en1990 (7 changed): heading-only texts now state the verified formula or criterion (E90.01, 08, 09, 17, 24, 26, 27).
- en1991 (14): the templated design-case texts "design case `case-eXX-YY` (catalogue_demo_cases, ...); normative tables unverified" said nothing about the requirement; rewritten from the spec (E11.05, 08, E12.05, 07, 08, 09, E13.08, 10, 11, 13, E14.10, 11, 16, E15.07).
- en1992 (156 of 159): all "Covers <heading>; realised as ..." texts replaced by the verified formula or criterion; "Also carries" rows keep the extra-row statement behind a proper main description. BR-09, LQ-08, DI-01 were already specific.
- en1993 (45): copy-paste duplicates fixed (V-024/025/026, V-028/030/031/033, V-062/067/127, V-110/113a, V-113/113b/113c, V-602/603/610), a wrong text fixed (V-360 described effective widths by rho, spec is shear lag beta; V-012 had Table 2.1 note for fracture toughness; V-374, V-073, V-070, V-091), caveat-only notes now name the requirement.
- en1994 (25): the 20 identical placeholders "confirmed by reading the check source: the family realises this id" plus C-33/C-48/C-36/C-24 "mapped by reading..." replaced by the requirement statements; C-19 and others sharpened.
- en1996 (37): all notes now state the verified formula/limit instead of "the stand-alone calculator stays a catalogue entry".
- en1997 (42): all texts now carry the formula or criterion from the spec (bearing, sliding, piles, walls, slopes, anchors, dewatering); demo-package noise removed.
- en1998 (17): "mapped by reading the check ids" rows (SEI-02, 20, 30, 35, 40, 41, 43, 44, 45, 47) and data rows (SEI-03, 04, 05, 42) replaced; SEI-11 lost its review meta text.
- en1999 (16): "confirmed:/corrected:/partial:/matched by clause number" meta notes replaced by the requirement (A-02, 04, 05, 06, 11, 12, 14-18, 20, 22, 23, 25, 27).
- DE: faithful renderings in the style of the existing labels (Nachweis, Bemessungswert, Querkraft, Theorie II. Ordnung, decimal comma in prose, English symbol names). Unchanged rows (e.g. en1993 V-001..V-123 generated texts, en1990 rest, en1992 BR-09/LQ-08/DI-01, en1994 C-05..C-09 etc., en1998 BR/ST/TW rows, en1999 A-01, 03, 10, 13, 19, 21, 24, 26, 28, 30-33, 40-81) were checked against the spec and kept.
- Unverified: spec marks [V]/[T] values (NDP numbers, table constants) are quoted as the spec has them; en1993 V-251 (distortional/eigenproblem wording from the check id en1993.1-3.5.5.dist) kept as written, the spec heading speaks of effective width of stiffened plates.

## Change log (id, before, after)

### en1990 E90.01
- before EN: Basic requirements for safety, serviceability and durability (2.1), realised as the members check
- before DE: Grundlegende Anforderungen an Tragsicherheit, Gebrauchstauglichkeit und Dauerhaftigkeit (2.1), umgesetzt als Bauteil-Nachweis
- after EN: Classification by structure kind and use (2.1) selecting the limit-state families and the applicable Annex A1 or A2, realised as the members check
- after DE: Klassifizierung nach Tragwerksart und Nutzung (2.1), die die Grenzzustandsfamilien und den anwendbaren Anhang A1 oder A2 festlegt, umgesetzt als Bauteil-Nachweis

### en1990 E90.08
- before EN: Characteristic values of actions (4.1.2), realised within the serviceability check family
- before DE: Charakteristische Werte der Einwirkungen (4.1.2), umgesetzt in der Gebrauchstauglichkeits-Nachweisfamilie
- after EN: Characteristic values G_k,sup/G_k,inf = G_m(1 ± 1.64 V_G) of permanent actions and the 98 % fractile of variable actions (4.1.2), realised within the serviceability check family
- after DE: Charakteristische Werte G_k,sup/G_k,inf = G_m(1 ± 1,64 V_G) ständiger Einwirkungen und 98-%-Fraktile veränderlicher Einwirkungen (4.1.2), umgesetzt in der Gebrauchstauglichkeits-Nachweisfamilie

### en1990 E90.09
- before EN: Other representative values of actions via ψ0, ψ1, ψ2 (4.1.3), realised within the serviceability check family
- before DE: Weitere repräsentative Werte der Einwirkungen über ψ0, ψ1, ψ2 (4.1.3), umgesetzt in der Gebrauchstauglichkeits-Nachweisfamilie
- after EN: Combination, frequent and quasi-permanent values ψ0 Q_k, ψ1 Q_k, ψ2 Q_k with ψ0 ≥ ψ1 ≥ ψ2 ≥ 0 (4.1.3), realised within the serviceability check family
- after DE: Kombinations-, häufige und quasi-ständige Werte ψ0 Q_k, ψ1 Q_k, ψ2 Q_k mit ψ0 ≥ ψ1 ≥ ψ2 ≥ 0 (4.1.3), umgesetzt in der Gebrauchstauglichkeits-Nachweisfamilie

### en1990 E90.17
- before EN: Accidental combination of actions (6.4.3.3, (6.11b)), realised as the (6.11b) combination check
- before DE: Außergewöhnliche Kombination der Einwirkungen (6.4.3.3, (6.11b)), umgesetzt als Kombinationsnachweis nach (6.11b)
- after EN: Accidental combination ΣG_k + P + A_d + (ψ1,1 or ψ2,1) Q_k,1 + Σψ2,i Q_k,i with all γ = 1.0 (6.4.3.3, (6.11b)), realised as the (6.11b) combination check
- after DE: Außergewöhnliche Kombination ΣG_k + P + A_d + (ψ1,1 oder ψ2,1) Q_k,1 + Σψ2,i Q_k,i mit allen γ = 1,0 (6.4.3.3, (6.11b)), umgesetzt als Kombinationsnachweis nach (6.11b)

### en1990 E90.24
- before EN: Serviceability criteria for buildings (A1.4), realised as the A1.4 check
- before DE: Gebrauchstauglichkeitskriterien für Hochbauten (A1.4), umgesetzt als Nachweis nach A1.4
- after EN: Deflection limits for buildings on w_net,fin = w_tot − w_c with the components w_1, w_2, w_3 (A1.4), realised as the A1.4 check
- after DE: Durchbiegungsgrenzen für Hochbauten an w_net,fin = w_tot − w_c mit den Anteilen w_1, w_2, w_3 (A1.4), umgesetzt als Nachweis nach A1.4

### en1990 E90.27
- before EN: Serviceability criteria for bridges (A2.4.3), realised as the bridge basis check
- before DE: Gebrauchstauglichkeitskriterien für Brücken (A2.4.3), umgesetzt als Brücken-Grundlagen-Nachweis
- after EN: Serviceability criteria for bridges: footbridge acceleration a_v ≤ 0.7 m/s² and railway deck acceleration and resonance (A2.4.3), realised as the bridge basis check
- after DE: Gebrauchstauglichkeitskriterien für Brücken: Beschleunigung a_v ≤ 0,7 m/s² bei Fußgängerbrücken sowie Beschleunigung und Resonanz bei Eisenbahnbrücken (A2.4.3), umgesetzt als Brücken-Grundlagen-Nachweis

### en1990 E90.26
- before EN: Partial factors for bridges (Table A2.4(A,B,C), A2.4.4), realised as the transient-situation check
- before DE: Teilsicherheitsbeiwerte für Brücken (Tabelle A2.4(A,B,C), A2.4.4), umgesetzt als Nachweis der vorübergehenden Situation
- after EN: Partial factors for bridges: EQU γG,sup 1.05 and γG,inf 0.95, γQ 1.35 for road and pedestrian and 1.45 for rail traffic, STR/GEO set B γG 1.35/1.00 (Table A2.4(A,B,C), A2.4.4), realised as the transient-situation check
- after DE: Teilsicherheitsbeiwerte für Brücken: EQU γG,sup 1,05 und γG,inf 0,95, γQ 1,35 für Straßen- und Fußgängerverkehr und 1,45 für Eisenbahnverkehr, STR/GEO Satz B γG 1,35/1,00 (Tabelle A2.4(A,B,C), A2.4.4), umgesetzt als Nachweis der vorübergehenden Situation

### en1991 E11.05
- before EN: design case `case-e11-05` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e11-05` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Horizontal line loads q_k on parapets and barriers by category (6.4, Tables 6.12-6.13), realised as a design case
- after DE: Horizontale Linienlasten q_k auf Geländer und Brüstungen nach Kategorie (6.4, Tabellen 6.12-6.13), umgesetzt als Bemessungsfall

### en1991 E11.08
- before EN: design case `case-e11-08` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e11-08` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Load spread of wheel and point loads at 45° through the layers to the slab mid-plane (4.4, Figure 6.2), realised as a design case
- after DE: Lastausbreitung von Rad- und Punktlasten unter 45° durch die Schichten bis zur Plattenmittelebene (4.4, Bild 6.2), umgesetzt als Bemessungsfall

### en1991 E12.05
- before EN: design case `case-e12-05` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e12-05` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Localised fires with flame length L_f = −1.02 D + 0.0148 Q^(2/5) and axis temperature θ(z) ≤ 900 °C (Annex C), realised as a design case
- after DE: Örtlich begrenzte Brände mit Flammenlänge L_f = −1,02 D + 0,0148 Q^(2/5) und Achstemperatur θ(z) ≤ 900 °C (Anhang C), umgesetzt als Bemessungsfall

### en1991 E12.07
- before EN: design case `case-e12-07` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e12-07` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Actions in the fire situation E_fi,d = G_k + (ψ1,1 or ψ2,1) Q_k,1 + Σψ2,i Q_k,i + A_d(t) with reduction factor η_fi (4.2, 4.3.1), realised as a design case
- after DE: Einwirkungen im Brandfall E_fi,d = G_k + (ψ1,1 oder ψ2,1) Q_k,1 + Σψ2,i Q_k,i + A_d(t) mit Abminderungsbeiwert η_fi (4.2, 4.3.1), umgesetzt als Bemessungsfall

### en1991 E12.08
- before EN: design case `case-e12-08` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e12-08` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Thermal actions on external members from flame radiation I_f = ε_f σ T_f^4 and flame geometry (Annex B), realised as a design case
- after DE: Temperatureinwirkungen auf außenliegende Bauteile aus der Flammenstrahlung I_f = ε_f σ T_f^4 und der Flammengeometrie (Anhang B), umgesetzt als Bemessungsfall

### en1991 E12.09
- before EN: design case `case-e12-09` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e12-09` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Configuration factor Φ of a rectangular emitter to a receiving face (Annex G), realised as a design case
- after DE: Konfigurationsfaktor Φ einer rechteckigen Strahlungsfläche zu einer Empfangsfläche (Anhang G), umgesetzt als Bemessungsfall

### en1991 E13.08
- before EN: design case `case-e13-08` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e13-08` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Snow guard force F_s = s b sin α from the sloped length b to the next guard or ridge (5.3.8), realised as a design case
- after DE: Schneefanggitterkraft F_s = s b sin α aus der Dachschräglänge b bis zum nächsten Gitter oder First (5.3.8), umgesetzt als Bemessungsfall

### en1991 E13.10
- before EN: design case `case-e13-10` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e13-10` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Adjustment of the snow load to other return periods s_n/s_k via the coefficient of variation V (Annex D), realised as a design case
- after DE: Anpassung der Schneelast an andere Wiederkehrperioden s_n/s_k über den Variationskoeffizienten V (Anhang D), umgesetzt als Bemessungsfall

### en1991 E13.11
- before EN: design case `case-e13-11` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e13-11` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Unit weight of snow from 1.0 kN/m³ fresh to 4.0 kN/m³ wet (Annex E), realised as a design case
- after DE: Wichte des Schnees von 1,0 kN/m³ frisch bis 4,0 kN/m³ nass (Anhang E), umgesetzt als Bemessungsfall

### en1991 E13.13
- before EN: design case `case-e13-13` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e13-13` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Local snow effects at eaves and canopies with μ per roof zone on the horizontal projection (6), covered by the overhang and snow-guard cases, realised as a design case
- after DE: Örtliche Schneewirkungen an Traufen und Vordächern mit μ je Dachzone auf der Horizontalprojektion (6), abgedeckt durch die Überhang- und Schneefanggitter-Fälle, umgesetzt als Bemessungsfall

### en1991 E14.10
- before EN: design case `case-e14-10` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e14-10` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Net pressure coefficients c_p,net of canopies, free-standing walls and signboards with blockage φ (7.4-7.5, Tables 7.6-7.9), realised as a design case
- after DE: Netto-Druckbeiwerte c_p,net freistehender Dächer, Wandscheiben und Schilder mit Versperrungsgrad φ (7.4-7.5, Tabellen 7.6-7.9), umgesetzt als Bemessungsfall

### en1991 E14.11
- before EN: design case `case-e14-11` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e14-11` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Wind force F_w = c_s c_d c_f q_p(z_e) A_ref with force coefficients of circular and rectangular sections c_f = c_f,0 ψ_r ψ_λ (7.8-7.10), realised as a design case
- after DE: Windkraft F_w = c_s c_d c_f q_p(z_e) A_ref mit Kraftbeiwerten für Kreis- und Rechteckquerschnitte c_f = c_f,0 ψ_r ψ_λ (7.8-7.10), umgesetzt als Bemessungsfall

### en1991 E14.16
- before EN: design case `case-e14-16` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e14-16` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Further force coefficients including frictional drag F_fr = c_fr q_p A_fr (7.6-7.13), realised as a design case
- after DE: Weitere Kraftbeiwerte einschließlich Reibungskraft F_fr = c_fr q_p A_fr (7.6-7.13), umgesetzt als Bemessungsfall

### en1991 E15.07
- before EN: design case `case-e15-07` (catalogue_demo_cases, change-case-number|text leaves); normative tables unverified
- before DE: Bemessungsfall `case-e15-07` (catalogue_demo_cases, Blätter change-case-number|text); normative Tabellen ungeprüft
- after EN: Temperature gradient through the shell of industrial structures such as chimneys and silos (A.1), realised as a design case
- after DE: Temperaturgradient durch die Schale von Industriebauwerken wie Schornsteinen und Silos (A.1), umgesetzt als Bemessungsfall

### en1992 M-01
- before EN: Covers design strength of concrete (fcd, fctd); realised as a check family
- before DE: Deckt Bemessungswert der Betonfestigkeit (fcd, fctd) ab; umgesetzt als Nachweisfamilie
- after EN: Design strengths fcd = α_cc fck/γ_c and fctd = α_ct fctk,0.05/γ_c of concrete by design situation; realised as a check family
- after DE: Bemessungswerte fcd = α_cc fck/γ_c und fctd = α_ct fctk,0.05/γ_c der Betonfestigkeit je Bemessungssituation; umgesetzt als Nachweisfamilie

### en1992 M-02
- before EN: Also carries the scope row for classes above C90/105 (3.1.1).
- before DE: Trägt zusätzlich die Geltungsbereichszeile für Klassen oberhalb C90/105 (3.1.1).
- after EN: Concrete property table of Table 3.1 (fcm = fck + 8, fctm, Ecm) per strength class; also carries the scope row for classes above C90/105 (3.1.1)
- after DE: Betonkennwerte nach Tabelle 3.1 (fcm = fck + 8, fctm, Ecm) je Festigkeitsklasse; trägt zusätzlich die Geltungsbereichszeile für Klassen oberhalb C90/105 (3.1.1)

### en1992 M-03
- before EN: Covers cube/cylinder strength mapping and age effects; realised as a check family
- before DE: Deckt Zuordnung von Zylinder- und Würfelfestigkeit und Alterungseinflüsse ab; umgesetzt als Nachweisfamilie
- after EN: Cube strength fck,cube from the grade table and the strength gain fck(t) for t < 28 d per NA 3.1.2(5); realised as a check family
- after DE: Würfeldruckfestigkeit fck,cube aus der Festigkeitsklassentabelle und Festigkeitszuwachs fck(t) für t < 28 d nach NA 3.1.2(5); umgesetzt als Nachweisfamilie

### en1992 M-04
- before EN: Covers strength and tensile strength development over time; realised as a design case
- before DE: Deckt zeitliche Entwicklung von Druck- und Zugfestigkeit ab; umgesetzt als Bemessungsfall
- after EN: Strength development fcm(t) = β_cc(t) fcm with β_cc(t) = exp(s(1 − √(28/t))) and the matching tensile strength; realised as a design case
- after DE: Festigkeitsentwicklung fcm(t) = β_cc(t) fcm mit β_cc(t) = exp(s(1 − √(28/t))) und zugehöriger Zugfestigkeit; umgesetzt als Bemessungsfall

### en1992 M-05
- before EN: Covers the parabola-rectangle stress-strain diagram of concrete; realised as a design case
- before DE: Deckt das Parabel-Rechteck-Diagramm des Betons ab; umgesetzt als Bemessungsfall
- after EN: Parabola-rectangle diagram σ_c = fcd(1 − (1 − ε/ε_c2)^n) up to ε_c2, then fcd up to ε_cu2; realised as a design case
- after DE: Parabel-Rechteck-Diagramm σ_c = fcd(1 − (1 − ε/ε_c2)^n) bis ε_c2, danach fcd bis ε_cu2; umgesetzt als Bemessungsfall

### en1992 M-06
- before EN: Covers the bilinear stress-strain diagram of concrete; realised as a design case
- before DE: Deckt das bilineare Spannungs-Dehnungs-Diagramm des Betons ab; umgesetzt als Bemessungsfall
- after EN: Bilinear concrete diagram σ_c = fcd ε/ε_c3 up to ε_c3, then fcd up to ε_cu3; realised as a design case
- after DE: Bilineares Betondiagramm σ_c = fcd ε/ε_c3 bis ε_c3, danach fcd bis ε_cu3; umgesetzt als Bemessungsfall

### en1992 M-07
- before EN: Covers the rectangular stress block with lambda and eta; realised as a design case
- before DE: Deckt den Rechteckspannungsblock mit lambda und eta ab; umgesetzt als Bemessungsfall
- after EN: Rectangular stress block of depth λx and stress η fcd with Fc = η fcd λ x b and lever arm z = d − λx/2; realised as a design case
- after DE: Rechteckspannungsblock der Höhe λx und Spannung η fcd mit Fc = η fcd λ x b und Hebelarm z = d − λx/2; umgesetzt als Bemessungsfall

### en1992 M-08
- before EN: Also carries the missing reinforcement grade row (3.2).
- before DE: Trägt zusätzlich die Zeile für fehlende Betonstahlsorte (3.2).
- after EN: Reinforcing steel diagrams with inclined branch up to ε_ud = 0.9 ε_uk and horizontal branch (3.2.7); also carries the row for a missing reinforcement grade (3.2)
- after DE: Betonstahl-Diagramme mit geneigtem Ast bis ε_ud = 0,9 ε_uk und horizontalem Ast (3.2.7); trägt zusätzlich die Zeile für fehlende Betonstahlsorte (3.2)

### en1992 M-09
- before EN: Covers the prestressing steel design diagrams; realised as a check family
- before DE: Deckt die Bemessungsdiagramme des Spannstahls ab; umgesetzt als Nachweisfamilie
- after EN: Prestressing steel design diagram with inclined branch fpd = fp0.1k/γ_s up to ε_ud = 0.02 and elastic modulus Ep; realised as a check family
- after DE: Bemessungsdiagramm des Spannstahls mit geneigtem Ast fpd = fp0.1k/γ_s bis ε_ud = 0,02 und Elastizitätsmodul Ep; umgesetzt als Nachweisfamilie

### en1992 M-10
- before EN: Covers the creep coefficient; realised as a design case
- before DE: Deckt die Kriechzahl ab; umgesetzt als Bemessungsfall
- after EN: Creep coefficient φ(∞,t0) taken from the Annex B.1 derivation A-01 for the effective modulus; realised as a design case
- after DE: Kriechzahl φ(∞,t0) aus der Herleitung A-01 nach Anhang B.1 für den effektiven Elastizitätsmodul; umgesetzt als Bemessungsfall

### en1992 M-12
- before EN: Covers the effective modulus of concrete Ec,eff; realised as a design case
- before DE: Deckt den effektiven Elastizitätsmodul Ec,eff des Betons ab; umgesetzt als Bemessungsfall
- after EN: Effective modulus Ec,eff = Ecm/(1 + φ(∞,t0)) and modular ratio α_e,eff = Es/Ec,eff; realised as a design case
- after DE: Effektiver Elastizitätsmodul Ec,eff = Ecm/(1 + φ(∞,t0)) und Verhältnis α_e,eff = Es/Ec,eff; umgesetzt als Bemessungsfall

### en1992 M-13
- before EN: Covers elevated temperature and fatigue factors of materials; realised as a design case
- before DE: Deckt Faktoren für erhöhte Temperatur und Ermüdung der Baustoffe ab; umgesetzt als Bemessungsfall
- after EN: Elevated-temperature and fatigue reduction factors of the materials, delegated to the entries F-01 to F-07 and R-01; realised as a design case
- after DE: Abminderungsfaktoren der Baustoffe für erhöhte Temperatur und Ermüdung, delegiert an die Einträge F-01 bis F-07 und R-01; umgesetzt als Bemessungsfall

### en1992 M-14
- before EN: Covers the cracking moment; realised as a check family
- before DE: Deckt das Rissmoment ab; umgesetzt als Nachweisfamilie
- after EN: Cracking moment Mcr = fctm,fl Wc with fctm,fl = max((1.6 − h/1000) fctm, fctm) and the prestress term of Mcr,P; realised as a check family
- after DE: Rissmoment Mcr = fctm,fl Wc mit fctm,fl = max((1,6 − h/1000) fctm, fctm) und Vorspannanteil von Mcr,P; umgesetzt als Nachweisfamilie

### en1992 D-01
- before EN: Covers exposure classification; realised as a design case
- before DE: Deckt die Einteilung in Expositionsklassen ab; umgesetzt als Bemessungsfall
- after EN: Exposure classes per member face (X0, XC, XD, XS, XF, XA and the DE moisture classes WO, WF, WA, WS) giving the governing class set; realised as a design case
- after DE: Expositionsklassen je Bauteilseite (X0, XC, XD, XS, XF, XA und die Feuchtigkeitsklassen WO, WF, WA, WS nach NA) mit der maßgebenden Klassenmenge; umgesetzt als Bemessungsfall

### en1992 D-02
- before EN: Covers minimum concrete strength class per exposure; realised as a check family
- before DE: Deckt die Mindestbetonfestigkeitsklasse je Exposition ab; umgesetzt als Nachweisfamilie
- after EN: Minimum concrete strength class per exposure class from Table E.1DE, e.g. C16/20 for XC1 up to C35/45 for XD3; realised as a check family
- after DE: Mindestbetonfestigkeitsklasse je Expositionsklasse nach Tabelle E.1DE, z. B. C16/20 für XC1 bis C35/45 für XD3; umgesetzt als Nachweisfamilie

### en1992 D-03
- before EN: Covers minimum concrete cover cmin; realised as a check family
- before DE: Deckt die Mindestbetondeckung cmin ab; umgesetzt als Nachweisfamilie
- after EN: Minimum cover cmin = max(cmin,b, cmin,dur + Δc,dur,γ − Δc,dur,st − Δc,dur,add, 10 mm) from bar diameter, structural class S1-S6 and exposure; realised as a check family
- after DE: Mindestbetondeckung cmin = max(cmin,b, cmin,dur + Δc,dur,γ − Δc,dur,st − Δc,dur,add, 10 mm) aus Stabdurchmesser, Bauwerksklasse S1-S6 und Exposition; umgesetzt als Nachweisfamilie

### en1992 D-04
- before EN: Covers nominal concrete cover cnom; realised as a check family
- before DE: Deckt das Nennmaß der Betondeckung cnom ab; umgesetzt als Nachweisfamilie
- after EN: Nominal cover cnom = cmin + Δc,dev with Δc,dev = 10 mm, compared with the provided cover; realised as a check family
- after DE: Nennmaß der Betondeckung cnom = cmin + Δc,dev mit Δc,dev = 10 mm, verglichen mit der vorhandenen Deckung; umgesetzt als Nachweisfamilie

### en1992 D-05
- before EN: Covers concrete cover for bond; realised as a design case
- before DE: Deckt die Betondeckung für den Verbund ab; umgesetzt als Bemessungsfall
- after EN: Cover for bond cmin,b ≥ φ and increased nominal cover against prepared ground or soil (4.4.1.2(13), 4.4.1.3); realised as a design case
- after DE: Betondeckung für den Verbund cmin,b ≥ φ und vergrößertes Nennmaß gegen vorbereiteten Baugrund oder Erdreich (4.4.1.2(13), 4.4.1.3); umgesetzt als Bemessungsfall

### en1992 D-06
- before EN: Covers concrete cover for fire; realised as a design case
- before DE: Deckt die Betondeckung für den Brandfall ab; umgesetzt als Bemessungsfall
- after EN: Cover for fire as axis distance, delegated to the fire entry R-01 (4.4.1.2, EN 1992-1-2); realised as a design case
- after DE: Betondeckung für den Brandfall als Achsabstand, delegiert an den Brandeintrag R-01 (4.4.1.2, EN 1992-1-2); umgesetzt als Bemessungsfall

### en1992 D-07
- before EN: Covers durability design life; realised as a design case
- before DE: Deckt die Nutzungsdauer für die Dauerhaftigkeit ab; umgesetzt als Bemessungsfall
- after EN: Design working life classes of EN 1990 Table 2.1 feeding the structural class correction of the nominal cover D-04; realised as a design case
- after DE: Nutzungsdauerklassen nach EN 1990 Tabelle 2.1, die die Korrektur der Bauwerksklasse für das Nennmaß D-04 speisen; umgesetzt als Bemessungsfall

### en1992 A-01
- before EN: Covers the creep coefficient phi(t,t0) per Annex B.1; realised as a design case
- before DE: Deckt die Kriechzahl phi(t,t0) nach Anhang B.1 ab; umgesetzt als Bemessungsfall
- after EN: Creep coefficient φ(t,t0) per Annex B.1 from fcm, RH, notional size h0, loading age t0 and cement class; realised as a design case
- after DE: Kriechzahl φ(t,t0) nach Anhang B.1 aus fcm, RH, wirksamer Dicke h0, Belastungsalter t0 und Zementklasse; umgesetzt als Bemessungsfall

### en1992 A-02
- before EN: Covers the shrinkage strain per Annex B.2; realised as a design case
- before DE: Deckt die Schwinddehnung nach Anhang B.2 ab; umgesetzt als Bemessungsfall
- after EN: Shrinkage strain ε_cs = ε_cd + ε_ca per Annex B.2 with β_ds(t,ts) and kh; realised as a design case
- after DE: Schwinddehnung ε_cs = ε_cd + ε_ca nach Anhang B.2 mit β_ds(t,ts) und kh; umgesetzt als Bemessungsfall

### en1992 A-03
- before EN: Covers imperfections and geometric models; realised as a design case
- before DE: Deckt Imperfektionen und geometrische Ersatzmodelle ab; umgesetzt als Bemessungsfall
- after EN: Geometric imperfections θ_i = θ_0 α_h α_m with θ_0 = 1/200 and the eccentricity e_i of isolated members; realised as a design case
- after DE: Geometrische Imperfektionen θ_i = θ_0 α_h α_m mit θ_0 = 1/200 und Ausmitte e_i von Einzelbauteilen; umgesetzt als Bemessungsfall

### en1992 A-04
- before EN: Covers the effective flange width; realised as a design case
- before DE: Deckt die mitwirkende Plattenbreite ab; umgesetzt als Bemessungsfall
- after EN: Effective flange width beff = Σ beff,i + bw ≤ b with beff,i = 0.2 bi + 0.1 l0 ≤ 0.2 l0 and ≤ bi; realised as a design case
- after DE: Mitwirkende Plattenbreite beff = Σ beff,i + bw ≤ b mit beff,i = 0,2 bi + 0,1 l0 ≤ 0,2 l0 und ≤ bi; umgesetzt als Bemessungsfall

### en1992 A-05
- before EN: Covers linear elastic analysis; realised as a design case
- before DE: Deckt die linear-elastische Berechnung ab; umgesetzt als Bemessungsfall
- after EN: Linear elastic analysis with cracked or uncracked stiffness, feeding the redistribution of A-06; realised as a design case
- after DE: Linear-elastische Berechnung mit gerissener oder ungerissener Steifigkeit als Grundlage der Umlagerung nach A-06; umgesetzt als Bemessungsfall

### en1992 A-06
- before EN: Covers linear analysis with limited redistribution; realised as a design case
- before DE: Deckt die linear-elastische Berechnung mit begrenzter Umlagerung ab; umgesetzt als Bemessungsfall
- after EN: Limited redistribution δ ≥ k1 + k2 xu/d for fck ≤ 50 and δ ≥ k3 + k4 xu/d for fck > 50; realised as a design case
- after DE: Begrenzte Umlagerung δ ≥ k1 + k2 xu/d für fck ≤ 50 und δ ≥ k3 + k4 xu/d für fck > 50; umgesetzt als Bemessungsfall

### en1992 A-07
- before EN: Covers plastic analysis and rotation capacity; realised as a check family
- before DE: Deckt die plastische Berechnung und das Rotationsvermögen ab; umgesetzt als Nachweisfamilie
- after EN: Plastic analysis with rotation capacity θ_pl,d ≤ k_λ θ_pl,d(tab), k_λ = √(λ/3), and xu/d limited to 0.25 or 0.15; realised as a check family
- after DE: Plastische Berechnung mit Rotationsvermögen θ_pl,d ≤ k_λ θ_pl,d(tab), k_λ = √(λ/3), und xu/d begrenzt auf 0,25 bzw. 0,15; umgesetzt als Nachweisfamilie

### en1992 A-08
- before EN: Covers non-linear analysis with strain compatibility; realised as a design case
- before DE: Deckt die nichtlineare Berechnung mit Verträglichkeit der Dehnungen ab; umgesetzt als Bemessungsfall
- after EN: Non-linear strain-compatible analysis with the global safety factor γ_R = 1.2 and Rd = Rm(fcm-based)/γ_R; realised as a design case
- after DE: Nichtlineare Berechnung mit verträglichen Dehnungen und globalem Sicherheitsbeiwert γ_R = 1,2 sowie Rd = Rm(auf fcm bezogen)/γ_R; umgesetzt als Bemessungsfall

### en1992 A-09
- before EN: Covers slenderness and slenderness limits; realised as a check family
- before DE: Deckt Schlankheit und Grenzschlankheit ab; umgesetzt als Nachweisfamilie
- after EN: Slenderness λ = l0/i against the limit λ_lim = 20 A B C/√n; realised as a check family
- after DE: Schlankheit λ = l0/i gegen die Grenzschlankheit λ_lim = 20 A B C/√n; umgesetzt als Nachweisfamilie

### en1992 A-10
- before EN: Covers the nominal curvature method; realised as a check family
- before DE: Deckt das Verfahren mit Nennkrümmung ab; umgesetzt als Nachweisfamilie
- after EN: Nominal curvature method with MEd = M0Ed + NEd e2, e2 = (1/r) l0²/c and 1/r = Kr Kφ (1/r0); realised as a check family
- after DE: Modellstützenverfahren mit Nennkrümmung: MEd = M0Ed + NEd e2, e2 = (1/r) l0²/c und 1/r = Kr Kφ (1/r0); umgesetzt als Nachweisfamilie

### en1992 A-11
- before EN: Covers the method based on nominal stiffness; realised as a check family
- before DE: Deckt das Verfahren mit Nennsteifigkeit ab; umgesetzt als Nachweisfamilie
- after EN: Nominal stiffness method with EI = Kc Ecd Ic + Ks Es Is and Ecd = Ecm/γ_cE, γ_cE = 1.2; realised as a check family
- after DE: Modellstützenverfahren mit Nennsteifigkeit: EI = Kc Ecd Ic + Ks Es Is und Ecd = Ecm/γ_cE, γ_cE = 1,2; umgesetzt als Nachweisfamilie

### en1992 A-12
- before EN: Covers biaxial bending of slender columns; realised as a design case
- before DE: Deckt zweiachsige Biegung schlanker Stützen ab; umgesetzt als Bemessungsfall
- after EN: Biaxial bending of slender columns by the section-level check S-05 with moments amplified per axis; realised as a design case
- after DE: Zweiachsige Biegung schlanker Stützen über den Querschnittsnachweis S-05 mit je Achse vergrößerten Momenten; umgesetzt als Bemessungsfall

### en1992 A-13
- before EN: Covers the general non-linear second-order method; realised as a design case
- before DE: Deckt das allgemeine nichtlineare Verfahren nach Theorie II. Ordnung ab; umgesetzt als Bemessungsfall
- after EN: General non-linear second-order method by curvature integration from the section M-N-κ relation, giving the collapse load factor; realised as a design case
- after DE: Allgemeines nichtlineares Verfahren nach Theorie II. Ordnung durch Krümmungsintegration aus der Momenten-Normalkraft-Krümmungs-Beziehung des Querschnitts mit Traglastfaktor; umgesetzt als Bemessungsfall

### en1992 A-14
- before EN: Covers second-order effects of slender walls and global lateral loads; realised as a design case
- before DE: Deckt Auswirkungen nach Theorie II. Ordnung bei schlanken Wänden und globalen Horizontallasten ab; umgesetzt als Bemessungsfall
- after EN: Global second-order effects of slender walls and lateral loads: Fv,Ed ≤ k1 Σ Ecd Ic/L² · nS/(nS + 1.6) with k1 = 0.31; realised as a design case
- after DE: Globale Effekte nach Theorie II. Ordnung bei schlanken Wänden und Horizontallasten: Fv,Ed ≤ k1 Σ Ecd Ic/L² · nS/(nS + 1,6) mit k1 = 0,31; umgesetzt als Bemessungsfall

### en1992 A-15
- before EN: Covers lateral instability of slender beams; realised as a design case
- before DE: Deckt das seitliche Ausweichen schlanker Träger ab; umgesetzt als Bemessungsfall
- after EN: Lateral instability of slender beams: l0t/b ≤ 50/(h/b)^(1/3) and h/b ≤ 2.5; realised as a design case
- after DE: Seitliches Ausweichen schlanker Träger: l0t/b ≤ 50/(h/b)^(1/3) und h/b ≤ 2,5; umgesetzt als Bemessungsfall

### en1992 A-16
- before EN: Covers prestress effects in the analysis, see section P; realised as a check family
- before DE: Deckt Vorspannwirkungen in der Berechnung, siehe Abschnitt P ab; umgesetzt als Nachweisfamilie
- after EN: Prestress effects in the analysis, delegated to the prestress section P; realised as a check family
- after DE: Vorspannwirkungen in der Berechnung, delegiert an den Vorspannabschnitt P; umgesetzt als Nachweisfamilie

### en1992 S-01
- before EN: Also carries the not-applicable row of a member without a ULS load case.
- before DE: Trägt zusätzlich die Zeile „nicht anwendbar“ für ein Bauteil ohne Lastfall im Grenzzustand der Tragfähigkeit.
- after EN: Strain-compatibility integrator over the section polygon with bar and tendon lists (6.1); also carries the not-applicable row for a member without a ULS load case
- after DE: Dehnungsebenen-Integrator über Querschnittspolygon sowie Stab- und Spanngliedlisten (6.1); trägt zusätzlich die Zeile „nicht anwendbar“ für ein Bauteil ohne Lastfall im Grenzzustand der Tragfähigkeit

### en1992 S-02
- before EN: Covers pure axial resistance; realised as a check family
- before DE: Deckt die Tragfähigkeit bei zentrischer Beanspruchung ab; umgesetzt als Nachweisfamilie
- after EN: Pure axial resistance NRd,0 = Ac fcd + As fyd in compression and NRd,t = −As fyd in tension; realised as a check family
- after DE: Zentrische Tragfähigkeit NRd,0 = Ac fcd + As fyd auf Druck und NRd,t = −As fyd auf Zug; umgesetzt als Nachweisfamilie

### en1992 S-03
- before EN: Covers bending design of a singly reinforced rectangular beam; realised as a check family
- before DE: Deckt die Biegebemessung des einfach bewehrten Rechteckbalkens ab; umgesetzt als Nachweisfamilie
- after EN: Bending design of the singly reinforced rectangular beam with μ_Eds = MEds/(b d² fcd), ω = 1 − √(1 − 2μ) and As1; realised as a check family
- after DE: Biegebemessung des einfach bewehrten Rechteckbalkens mit μ_Eds = MEds/(b d² fcd), ω = 1 − √(1 − 2μ) und As1; umgesetzt als Nachweisfamilie

### en1992 S-04
- before EN: Covers uniaxial N-M interaction (MRd for given NEd); realised as a check family
- before DE: Deckt die einachsige M-N-Interaktion (MRd bei gegebenem NEd) ab; umgesetzt als Nachweisfamilie
- after EN: Uniaxial N-M interaction: resistance MRd for a given NEd solved on the strain plane of S-01; realised as a check family
- after DE: Einachsige M-N-Interaktion: Tragmoment MRd bei gegebenem NEd, gelöst auf der Dehnungsebene nach S-01; umgesetzt als Nachweisfamilie

### en1992 S-05
- before EN: Covers simplified biaxial bending; realised as a design case
- before DE: Deckt die vereinfachte zweiachsige Biegung ab; umgesetzt als Bemessungsfall
- after EN: Simplified biaxial bending (MEdz/MRdz)^a + (MEdy/MRdy)^a ≤ 1.0 with exponent a interpolated from 1.0 to 2.0 over NEd/NRd; realised as a design case
- after DE: Vereinfachte zweiachsige Biegung (MEdz/MRdz)^a + (MEdy/MRdy)^a ≤ 1,0 mit über NEd/NRd von 1,0 bis 2,0 interpoliertem Exponenten a; umgesetzt als Bemessungsfall

### en1992 S-06
- before EN: Covers the exact biaxial N-My-Mz interaction surface; realised as a design case
- before DE: Deckt die genaue N-My-Mz-Interaktionsfläche ab; umgesetzt als Bemessungsfall
- after EN: Exact biaxial N-My-Mz interaction surface from S-01 over all neutral-axis angles and depths; realised as a design case
- after DE: Genaue N-My-Mz-Interaktionsfläche aus S-01 über alle Nulllinienwinkel und -tiefen; umgesetzt als Bemessungsfall

### en1992 S-07
- before EN: Covers flanged T, L and box sections; realised as a check family
- before DE: Deckt Plattenbalken-, L- und Kastenquerschnitte ab; umgesetzt als Nachweisfamilie
- after EN: Flanged T, L and box sections: neutral axis in the flange reduces to S-03 with b = beff, otherwise web and flange are decomposed; realised as a check family
- after DE: Plattenbalken-, L- und Kastenquerschnitte: Nulllinie im Gurt führt auf S-03 mit b = beff, sonst Zerlegung in Steg und Gurt; umgesetzt als Nachweisfamilie

### en1992 S-08
- before EN: Covers minimum eccentricity and reinforcement limits; realised as a design case
- before DE: Deckt Mindestausmitte und Bewehrungsgrenzen ab; umgesetzt als Bemessungsfall
- after EN: Minimum eccentricity e0 = max(h/30, 20 mm) giving MEd ≥ NEd e0; realised as a design case
- after DE: Mindestausmitte e0 = max(h/30, 20 mm) mit MEd ≥ NEd e0; umgesetzt als Bemessungsfall

### en1992 S-09
- before EN: Covers walls, squat walls and plain concrete bearing capacity; realised as a design case
- before DE: Deckt Wände, gedrungene Wände und Tragfähigkeit von unbewehrtem Beton ab; umgesetzt als Bemessungsfall
- after EN: Bearing capacity of unreinforced walls NRd = b hw fcd,pl Φ with fcd,pl = α_cc,pl fck/γ_c; realised as a design case
- after DE: Tragfähigkeit unbewehrter Wände NRd = b hw fcd,pl Φ mit fcd,pl = α_cc,pl fck/γ_c; umgesetzt als Bemessungsfall

### en1992 S-10
- before EN: Covers bending of prestressed sections with pre-strain; realised as a check family
- before DE: Deckt die Biegung vorgespannter Querschnitte mit Vordehnung ab; umgesetzt als Nachweisfamilie
- after EN: Bending of prestressed sections with the pre-strain Pm,t/(Ap Ep) and fpd = fp0.1k/γ_s; realised as a check family
- after DE: Biegung vorgespannter Querschnitte mit der Vordehnung Pm,t/(Ap Ep) und fpd = fp0,1k/γ_s; umgesetzt als Nachweisfamilie

### en1992 V-01
- before EN: Covers the shear design framework; realised as a design case
- before DE: Deckt den Ablauf des Querkraftnachweises ab; umgesetzt als Bemessungsfall
- after EN: Shear design flow: VEd ≤ VRd,c without calculated shear reinforcement (V-02), otherwise V-03 or V-04; realised as a design case
- after DE: Ablauf des Querkraftnachweises: VEd ≤ VRd,c ohne rechnerische Querkraftbewehrung (V-02), sonst V-03 oder V-04; umgesetzt als Bemessungsfall

### en1992 V-02
- before EN: Covers members without design shear reinforcement; realised as a check family
- before DE: Deckt Bauteile ohne rechnerisch erforderliche Querkraftbewehrung ab; umgesetzt als Nachweisfamilie
- after EN: Shear resistance without shear reinforcement VRd,c = [CRd,c k (100 ρ_l fck)^(1/3) + k1 σ_cp] bw d ≥ (vmin + k1 σ_cp) bw d; realised as a check family
- after DE: Querkrafttragfähigkeit ohne Querkraftbewehrung VRd,c = [CRd,c k (100 ρ_l fck)^(1/3) + k1 σ_cp] bw d ≥ (vmin + k1 σ_cp) bw d; umgesetzt als Nachweisfamilie

### en1992 V-03
- before EN: Covers members with shear reinforcement and variable strut inclination; realised as a check family
- before DE: Deckt Bauteile mit Querkraftbewehrung und veränderlicher Druckstrebenneigung ab; umgesetzt als Nachweisfamilie
- after EN: Shear resistance with shear reinforcement by the truss model with variable strut inclination cot θ, stirrup angle α, ν1 and α_cw; realised as a check family
- after DE: Querkrafttragfähigkeit mit Querkraftbewehrung nach dem Fachwerkmodell mit veränderlicher Druckstrebenneigung cot θ, Bügelwinkel α, ν1 und α_cw; umgesetzt als Nachweisfamilie

### en1992 V-04
- before EN: Covers shear with axial force and prestress; realised as a check family
- before DE: Deckt Querkraft mit Normalkraft und Vorspannung ab; umgesetzt als Nachweisfamilie
- after EN: Shear with axial force and prestress through the compression stress σ_cp in α_cw; realised as a check family
- after DE: Querkraft mit Normalkraft und Vorspannung über die Betondruckspannung σ_cp in α_cw; umgesetzt als Nachweisfamilie

### en1992 V-05
- before EN: Covers shear at interfaces such as construction joints; realised as a design case
- before DE: Deckt Schubkraftübertragung in Fugen, etwa Arbeitsfugen ab; umgesetzt als Bemessungsfall
- after EN: Interface shear vEdi = β VEd/(z bi) ≤ vRdi = c fctd + μ σ_n + ρ fyd (μ sin α + cos α) ≤ 0.5 ν fcd; realised as a design case
- after DE: Schubkraftübertragung in Fugen vEdi = β VEd/(z bi) ≤ vRdi = c fctd + μ σ_n + ρ fyd (μ sin α + cos α) ≤ 0,5 ν fcd; umgesetzt als Bemessungsfall

### en1992 V-06
- before EN: Covers shear between web and flanges; realised as a design case
- before DE: Deckt Schub zwischen Steg und Gurten ab; umgesetzt als Bemessungsfall
- after EN: Shear between web and flanges with flange link area and anchorage, following V-03; realised as a design case
- after DE: Schub zwischen Steg und Gurten mit Gurtbewehrungsquerschnitt und Verankerung, anschließend an V-03; umgesetzt als Bemessungsfall

### en1992 V-07
- before EN: Covers shear in slabs with concentrated loads close to supports; realised as a check family
- before DE: Deckt Querkraft in Platten mit Einzellasten nahe am Auflager ab; umgesetzt als Nachweisfamilie
- after EN: Shear near supports under concentrated loads with VEd,red = β VEd, β = av/(2d), against VRd,c and 0.5 bw d ν fcd; realised as a check family
- after DE: Querkraft nahe dem Auflager bei Einzellasten mit VEd,red = β VEd, β = av/(2d), gegen VRd,c und 0,5 bw d ν fcd; umgesetzt als Nachweisfamilie

### en1992 V-08
- before EN: Covers combined shear and torsion; realised as a check family
- before DE: Deckt die Kombination von Querkraft und Torsion ab; umgesetzt als Nachweisfamilie
- after EN: Combined shear and torsion, delegated to T-04; realised as a check family
- after DE: Kombination von Querkraft und Torsion, delegiert an T-04; umgesetzt als Nachweisfamilie

### en1992 T-01
- before EN: Covers the thin-walled equivalent section; realised as a check family
- before DE: Deckt den dünnwandigen Ersatzquerschnitt ab; umgesetzt als Nachweisfamilie
- after EN: Thin-walled equivalent section with tef = A/u, Ak = (b − tef)(h − tef) and the perimeter uk; realised as a check family
- after DE: Dünnwandiger Ersatzquerschnitt mit tef = A/u, Ak = (b − tef)(h − tef) und Umfang uk; umgesetzt als Nachweisfamilie

### en1992 T-02
- before EN: Covers the torsion resistance; realised as a check family
- before DE: Deckt die Torsionstragfähigkeit ab; umgesetzt als Nachweisfamilie
- after EN: Torsion resistance with TRd,max = 2 ν α_cw fcd Ak tef sin θ cos θ and 1.0 ≤ cot θ ≤ 2.5; realised as a check family
- after DE: Torsionstragfähigkeit mit TRd,max = 2 ν α_cw fcd Ak tef sin θ cos θ und 1,0 ≤ cot θ ≤ 2,5; umgesetzt als Nachweisfamilie

### en1992 T-03
- before EN: Covers torsion in warped sections; realised as a design case
- before DE: Deckt Torsion in verwölbten Querschnitten ab; umgesetzt als Bemessungsfall
- after EN: Torsion in warped sections split into sub-sections proportional to stiffness, T_i = TEd Ti_i/Σ; realised as a design case
- after DE: Torsion in verwölbten Querschnitten, aufgeteilt in Teilquerschnitte proportional zur Steifigkeit, T_i = TEd Ti_i/Σ; umgesetzt als Bemessungsfall

### en1992 T-04
- before EN: Covers combined torsion and shear; realised as a check family
- before DE: Deckt die Kombination von Torsion und Querkraft ab; umgesetzt als Nachweisfamilie
- after EN: Combined torsion and shear: TEd/TRd,c + VEd/VRd,c ≤ 1.0 and TEd/TRd,max + VEd/VRd,max ≤ 1.0 for solid sections; realised as a check family
- after DE: Kombination von Torsion und Querkraft: TEd/TRd,c + VEd/VRd,c ≤ 1,0 und TEd/TRd,max + VEd/VRd,max ≤ 1,0 für Vollquerschnitte; umgesetzt als Nachweisfamilie

### en1992 P-01
- before EN: Covers the basic control perimeters for punching; realised as a check family
- before DE: Deckt die Rundschnitte für den Durchstanznachweis ab; umgesetzt als Nachweisfamilie
- after EN: Basic control perimeters of the column (rectangular, circular, polygon; interior, edge, corner) with opening shadow lines within 6d; realised as a check family
- after DE: Rundschnitte der Stütze (rechteckig, kreisförmig, Polygon; innen, Rand, Ecke) mit Schattenlinien von Öffnungen innerhalb 6d; umgesetzt als Nachweisfamilie

### en1992 P-02
- before EN: Covers the load increase factor beta; realised as a check family
- before DE: Deckt den Lasterhöhungsfaktor beta ab; umgesetzt als Nachweisfamilie
- after EN: Load increase factor β = 1 + k (MEd/VEd)(u1/W1) with k from Table 6.1; realised as a check family
- after DE: Lasterhöhungsfaktor β = 1 + k (MEd/VEd)(u1/W1) mit k nach Tabelle 6.1; umgesetzt als Nachweisfamilie

### en1992 P-03
- before EN: Covers punching resistance without shear reinforcement; realised as a check family
- before DE: Deckt den Durchstanzwiderstand ohne Durchstanzbewehrung ab; umgesetzt als Nachweisfamilie
- after EN: Punching without shear reinforcement vEd = β VEd/(u d) ≤ vRd,c = CRd,c k (100 ρ_l fck)^(1/3) + k1 σ_cp ≥ vmin + k1 σ_cp; realised as a check family
- after DE: Durchstanzen ohne Durchstanzbewehrung vEd = β VEd/(u d) ≤ vRd,c = CRd,c k (100 ρ_l fck)^(1/3) + k1 σ_cp ≥ vmin + k1 σ_cp; umgesetzt als Nachweisfamilie

### en1992 P-04
- before EN: Covers punching shear reinforcement and the stud layout rules (s_0, s_r, s_t); realised as a check family
- before DE: Deckt die Durchstanzbewehrung und die Dübelanordnung (s_0, s_r, s_t) ab; umgesetzt als Nachweisfamilie
- after EN: Punching with shear reinforcement vRd,cs = 0.75 vRd,c + 1.5 (d/sr) Asw fywd,ef (1/(u1 d)) sin α and the stud layout rules s_0, s_r, s_t; realised as a check family
- after DE: Durchstanzen mit Durchstanzbewehrung vRd,cs = 0,75 vRd,c + 1,5 (d/sr) Asw fywd,ef (1/(u1 d)) sin α und Anordnungsregeln der Dübelleisten s_0, s_r, s_t; umgesetzt als Nachweisfamilie

### en1992 P-05
- before EN: Covers the shear stud layout per NA and approval; realised as a design case
- before DE: Deckt die Anordnung der Dübelleisten nach NA und Zulassung ab; umgesetzt als Bemessungsfall
- after EN: Shear stud layout per the NA and the European technical assessment with vRd,max multipliers; realised as a design case
- after DE: Anordnung von Dübelleisten nach NA und europäischer technischer Bewertung mit vRd,max-Faktoren; umgesetzt als Bemessungsfall

### en1992 P-06
- before EN: Covers punching at columns with internal slab fractions and base plates; realised as a design case
- before DE: Deckt Durchstanzen an Stützen mit inneren Plattenanteilen und Fundamentplatten ab; umgesetzt als Bemessungsfall
- after EN: Punching at foundations and base plates with VEd,red = VEd − ΔVEd, ΔVEd = σ_gd A_crit, checked at several perimeters; realised as a design case
- after DE: Durchstanzen bei Fundamenten und Fußplatten mit VEd,red = VEd − ΔVEd, ΔVEd = σ_gd A_crit, an mehreren Rundschnitten nachgewiesen; umgesetzt als Bemessungsfall

### en1992 P-07
- before EN: Covers punching of prestressed flat slabs per the NA; realised as a design case
- before DE: Deckt das Durchstanzen vorgespannter Flachdecken nach NA ab; umgesetzt als Bemessungsfall
- after EN: Punching of prestressed flat slabs with vRd,c + 0.1 σ_cp and the inclined-tendon contribution Vp = P sin α per the NA; realised as a design case
- after DE: Durchstanzen vorgespannter Flachdecken mit vRd,c + 0,1 σ_cp und dem Anteil geneigter Spannglieder Vp = P sin α nach NA; umgesetzt als Bemessungsfall

### en1992 ST-01
- before EN: Covers strut capacity in strut-and-tie models; realised as a design case
- before DE: Deckt die Tragfähigkeit der Druckstreben im Stabwerkmodell ab; umgesetzt als Bemessungsfall
- after EN: Concrete strut capacity σ_Rd,max = fcd without transverse tension, 0.6 ν' fcd with transverse tension, ν' = 1 − fck/250; realised as a design case
- after DE: Tragfähigkeit der Betondruckstreben σ_Rd,max = fcd ohne Querzug, 0,6 ν' fcd mit Querzug, ν' = 1 − fck/250; umgesetzt als Bemessungsfall

### en1992 ST-02
- before EN: Covers tie capacity in strut-and-tie models; realised as a design case
- before DE: Deckt die Tragfähigkeit der Zugstreben im Stabwerkmodell ab; umgesetzt als Bemessungsfall
- after EN: Tie reinforcement A_s,tie = Ftd/fyd with anchorage beyond the node, A_s,prov ≥ A_s,tie; realised as a design case
- after DE: Zugstrebenbewehrung A_s,tie = Ftd/fyd mit Verankerung hinter dem Knoten, A_s,prov ≥ A_s,tie; umgesetzt als Bemessungsfall

### en1992 ST-03
- before EN: Covers node capacity in strut-and-tie models; realised as a design case
- before DE: Deckt die Tragfähigkeit der Knoten im Stabwerkmodell ab; umgesetzt als Bemessungsfall
- after EN: Node capacity σ_Rd,max = k ν' fcd for compression nodes CCC (k1), compression-tension nodes CCT (k2) and tension nodes CTT (k3); realised as a design case
- after DE: Knotentragfähigkeit σ_Rd,max = k ν' fcd für Druckknoten CCC (k1), Druck-Zug-Knoten CCT (k2) und Zugknoten CTT (k3); umgesetzt als Bemessungsfall

### en1992 ST-04
- before EN: Covers automatic generation of strut-and-tie models; realised as a design case
- before DE: Deckt die automatische Erzeugung von Stabwerkmodellen ab; umgesetzt als Bemessungsfall
- after EN: Strut-and-tie model generated from the force flow of B/D regions with strut and tie forces and angle θ recomputed on node edits; realised as a design case
- after DE: Stabwerkmodell aus dem Kraftfluss der B- und D-Bereiche mit Strebenkräften, Zugkräften und Winkel θ, nach Knotenänderung neu berechnet; umgesetzt als Bemessungsfall

### en1992 PA-01
- before EN: Covers partially loaded areas; realised as a design case
- before DE: Deckt Teilflächenbelastung ab; umgesetzt als Bemessungsfall
- after EN: Partially loaded areas FRdu = Ac0 fcd √(Ac1/Ac0) ≤ 3.0 fcd Ac0 with splitting reinforcement; realised as a design case
- after DE: Teilflächenbelastung FRdu = Ac0 fcd √(Ac1/Ac0) ≤ 3,0 fcd Ac0 mit Spaltzugbewehrung; umgesetzt als Bemessungsfall

### en1992 B-01
- before EN: Covers the ultimate bond stress; realised as a check family
- before DE: Deckt die Verbundspannung im Grenzzustand der Tragfähigkeit ab; umgesetzt als Nachweisfamilie
- after EN: Ultimate bond stress fbd = 2.25 η1 η2 fctd with η1 = 1.0 or 0.7 and η2 = 1.0 for φ ≤ 32 mm; realised as a check family
- after DE: Verbundspannung fbd = 2,25 η1 η2 fctd mit η1 = 1,0 oder 0,7 und η2 = 1,0 für φ ≤ 32 mm; umgesetzt als Nachweisfamilie

### en1992 B-02
- before EN: Covers the basic required anchorage length; realised as a check family
- before DE: Deckt das Grundmaß der erforderlichen Verankerungslänge ab; umgesetzt als Nachweisfamilie
- after EN: Basic required anchorage length lb,rqd = (φ/4)(σ_sd/fbd); realised as a check family
- after DE: Grundwert der Verankerungslänge lb,rqd = (φ/4)(σ_sd/fbd); umgesetzt als Nachweisfamilie

### en1992 B-03
- before EN: Covers the design anchorage length; realised as a check family
- before DE: Deckt den Bemessungswert der Verankerungslänge ab; umgesetzt als Nachweisfamilie
- after EN: Design anchorage length lbd = α1 α2 α3 α4 α5 lb,rqd ≥ lb,min with lb,min = max(0.3 lb,rqd, 10φ, 100 mm) in tension; realised as a check family
- after DE: Bemessungswert der Verankerungslänge lbd = α1 α2 α3 α4 α5 lb,rqd ≥ lb,min mit lb,min = max(0,3 lb,rqd, 10φ, 100 mm) auf Zug; umgesetzt als Nachweisfamilie

### en1992 B-04
- before EN: Covers the lap length; realised as a check family
- before DE: Deckt die Übergreifungslänge ab; umgesetzt als Nachweisfamilie
- after EN: Lap length l0 = α1 α2 α3 α5 α6 lb,rqd ≥ l0,min with α6 = (ρ1/25)^0.5 between 1.0 and 1.5; realised as a check family
- after DE: Übergreifungslänge l0 = α1 α2 α3 α5 α6 lb,rqd ≥ l0,min mit α6 = (ρ1/25)^0,5 zwischen 1,0 und 1,5; umgesetzt als Nachweisfamilie

### en1992 B-05
- before EN: Covers bundled bars and equivalent diameter; realised as a design case
- before DE: Deckt Stabbündel und Vergleichsdurchmesser ab; umgesetzt als Bemessungsfall
- after EN: Bundled bars with equivalent diameter φ_n = φ √nb ≤ 55 mm and nb ≤ 4 or 3; realised as a design case
- after DE: Stabbündel mit Vergleichsdurchmesser φ_n = φ √nb ≤ 55 mm und nb ≤ 4 bzw. 3; umgesetzt als Bemessungsfall

### en1992 B-06
- before EN: Covers large-diameter bars above 32 mm; realised as a design case
- before DE: Deckt Stäbe mit großem Durchmesser über 32 mm ab; umgesetzt als Bemessungsfall
- after EN: Large-diameter bars above 32 mm with surface reinforcement As,surf ≥ 0.01 Act,ext and no laps without proof; realised as a design case
- after DE: Stäbe mit Durchmesser über 32 mm mit Oberflächenbewehrung As,surf ≥ 0,01 Act,ext und ohne Stöße ohne Nachweis; umgesetzt als Bemessungsfall

### en1992 B-07
- before EN: Covers mesh, bundles and prestress anchorage; realised as a design case
- before DE: Deckt Matten, Bündel und Verankerung von Spanngliedern ab; umgesetzt als Bemessungsfall
- after EN: Mesh, bundles and prestress anchorage with the transmission length lpt = α1 α2 φ σ_pm0/fbpt; realised as a design case
- after DE: Matten, Bündel und Spannglied-Verankerung mit der Übertragungslänge lpt = α1 α2 φ σ_pm0/fbpt; umgesetzt als Bemessungsfall

### en1992 B-08
- before EN: Covers anchorage of links and shear reinforcement; realised as a design case
- before DE: Deckt die Verankerung von Bügeln und Querkraftbewehrung ab; umgesetzt als Bemessungsfall
- after EN: Anchorage of links and shear reinforcement with hooks of at least 5φ or 10φ and 70 mm; realised as a design case
- after DE: Verankerung von Bügeln und Querkraftbewehrung mit Haken von mindestens 5φ bzw. 10φ und 70 mm; umgesetzt als Bemessungsfall

### en1992 B-09
- before EN: Covers the spacing of bars; realised as a check family
- before DE: Deckt den Stababstand ab; umgesetzt als Nachweisfamilie
- after EN: Clear bar spacing of at least max(k1 φ, dg + k2, 20 mm) with k1 = 1 and k2 = 5 mm; realised as a check family
- after DE: Lichter Stababstand von mindestens max(k1 φ, dg + k2, 20 mm) mit k1 = 1 und k2 = 5 mm; umgesetzt als Nachweisfamilie

### en1992 SL-01
- before EN: Also carries the quasi-permanent stress row for linear creep (7.2(3)) and the EN 1992-2 7.2 bridge concrete stress row.
- before DE: Trägt zusätzlich die Zeile der quasi-ständigen Spannung für lineares Kriechen (7.2(3)) und die Zeile der Brücken-Betonspannung nach EN 1992-2 7.2.
- after EN: Concrete stress limitation σ_c ≤ k1 fck with k1 = 0.6 and the 0.45 fck limit for linear creep (7.2); also carries the quasi-permanent stress row for linear creep (7.2(3)) and the EN 1992-2 7.2 bridge concrete stress row
- after DE: Begrenzung der Betondruckspannung σ_c ≤ k1 fck mit k1 = 0,6 und der Grenze 0,45 fck für lineares Kriechen (7.2); trägt zusätzlich die Zeile der quasi-ständigen Spannung bei linearem Kriechen (7.2(3)) und die Brückenbetonspannungszeile nach EN 1992-2 7.2

### en1992 SL-02
- before EN: Covers stress limitation of reinforcing steel; realised as a check family
- before DE: Deckt die Spannungsbegrenzung des Betonstahls ab; umgesetzt als Nachweisfamilie
- after EN: Steel stress limitation σ_s ≤ k3 fyk with k3 = 0.8 and σ_p ≤ k5 fpk with k5 = 0.75 after losses; realised as a check family
- after DE: Begrenzung der Stahlspannung σ_s ≤ k3 fyk mit k3 = 0,8 und σ_p ≤ k5 fpk mit k5 = 0,75 nach Verlusten; umgesetzt als Nachweisfamilie

### en1992 SL-03
- before EN: Covers stresses in the cracked section (state II); realised as a design case
- before DE: Deckt Spannungen im gerissenen Querschnitt (Zustand II) ab; umgesetzt als Bemessungsfall
- after EN: Stresses of the cracked section (state II) from the neutral axis x and the modular ratio α_e; realised as a design case
- after DE: Spannungen des gerissenen Querschnitts (Zustand II) aus Nulllinienlage x und Verhältnis α_e; umgesetzt als Bemessungsfall

### en1992 SL-04
- before EN: Covers minimum reinforcement for crack control; realised as a design case
- before DE: Deckt die Mindestbewehrung zur Rissbreitenbegrenzung ab; umgesetzt als Bemessungsfall
- after EN: Minimum reinforcement for crack control As,min σ_s = kc k fct,eff Act; realised as a design case
- after DE: Mindestbewehrung zur Rissbreitenbegrenzung As,min σ_s = kc k fct,eff Act; umgesetzt als Bemessungsfall

### en1992 SL-05
- before EN: Covers crack control without direct calculation by bar diameter and spacing; realised as a design case
- before DE: Deckt die Rissbreitenbegrenzung ohne direkte Berechnung über Stabdurchmesser und Stababstand ab; umgesetzt als Bemessungsfall
- after EN: Crack control without direct calculation by limiting bar diameter and bar spacing against σ_s for wk = 0.4, 0.3 and 0.2 mm (Tables 7.2N, 7.3N); realised as a design case
- after DE: Rissbreitenbegrenzung ohne direkte Berechnung durch Begrenzung von Stabdurchmesser und Stababstand gegen σ_s für wk = 0,4, 0,3 und 0,2 mm (Tabellen 7.2N, 7.3N); umgesetzt als Bemessungsfall

### en1992 SL-06
- before EN: Covers the calculation of crack widths; realised as a check family
- before DE: Deckt die Berechnung der Rissbreite ab; umgesetzt als Nachweisfamilie
- after EN: Crack width wk = sr,max (ε_sm − ε_cm) with ε_sm − ε_cm ≥ 0.6 σ_s/Es and kt = 0.6 short-term or 0.4 long-term; realised as a check family
- after DE: Rissbreite wk = sr,max (ε_sm − ε_cm) mit ε_sm − ε_cm ≥ 0,6 σ_s/Es und kt = 0,6 kurzzeitig bzw. 0,4 langzeitig; umgesetzt als Nachweisfamilie

### en1992 SL-07
- before EN: Covers deflection limitation by span-to-depth ratio; realised as a check family
- before DE: Deckt die Begrenzung der Verformung über die Biegeschlankheit ab; umgesetzt als Nachweisfamilie
- after EN: Deflection limitation by span-to-depth ratio l/d ≤ K [11 + 1.5 √fck ρ0/ρ + …] with ρ0 = √fck/1000; realised as a check family
- after DE: Durchbiegungsbegrenzung durch Biegeschlankheit l/d ≤ K [11 + 1,5 √fck ρ0/ρ + …] mit ρ0 = √fck/1000; umgesetzt als Nachweisfamilie

### en1992 SL-08
- before EN: Covers deflection by calculation; realised as a check family
- before DE: Deckt die Verformungsberechnung ab; umgesetzt als Nachweisfamilie
- after EN: Deflection by calculation with the distribution coefficient ζ = 1 − β (σ_sr/σ_s)² interpolating between states I and II; realised as a check family
- after DE: Verformungsberechnung mit Verteilungsbeiwert ζ = 1 − β (σ_sr/σ_s)², der zwischen Zustand I und II interpoliert; umgesetzt als Nachweisfamilie

### en1992 SL-09
- before EN: Covers serviceability analysis of cracking and stress states in prestressed members; realised as a check family
- before DE: Deckt die Gebrauchstauglichkeitsanalyse von Rissbildung und Spannungszuständen vorgespannter Bauteile ab; umgesetzt als Nachweisfamilie
- after EN: Decompression and tensile stress checks of prestressed members: σ_c ≤ 0 under the quasi-permanent combination for XD and XS and σ_c,t ≥ −fctm; realised as a check family
- after DE: Dekompressions- und Zugspannungsnachweise vorgespannter Bauteile: σ_c ≤ 0 unter der quasi-ständigen Kombination für XD und XS und σ_c,t ≥ −fctm; umgesetzt als Nachweisfamilie

### en1992 F-01
- before EN: Covers the fatigue verification framework; realised as a design case
- before DE: Deckt den Rahmen des Ermüdungsnachweises ab; umgesetzt als Bemessungsfall
- after EN: Fatigue verification framework: required unless exempt, by simplified Δσ_s,equ check or damage accumulation; realised as a design case
- after DE: Rahmen des Ermüdungsnachweises: erforderlich, sofern nicht ausgenommen, vereinfacht über Δσ_s,equ oder durch Schädigungsakkumulation; umgesetzt als Bemessungsfall

### en1992 F-02
- before EN: Covers internal forces and stresses for fatigue; realised as a design case
- before DE: Deckt Schnittgrößen und Spannungen für den Ermüdungsnachweis ab; umgesetzt als Bemessungsfall
- after EN: Fatigue stress ranges from Q_fat on the permanent state G_k + Pm of the cracked section; realised as a design case
- after DE: Spannungsschwingbreiten für die Ermüdung aus Q_fat auf dem ständigen Zustand G_k + Pm des gerissenen Querschnitts; umgesetzt als Bemessungsfall

### en1992 F-03
- before EN: Covers stress ranges of steel and S-N curves; realised as a design case
- before DE: Deckt Spannungsschwingbreiten des Stahls und Wöhlerlinien ab; umgesetzt als Bemessungsfall
- after EN: S-N curves of reinforcing and prestressing steel with N* = 10^6, exponents k1, k2 and Δσ_Rsk(N*) of Table 6.3N; realised as a design case
- after DE: Wöhlerlinien von Betonstahl und Spannstahl mit N* = 10^6, Exponenten k1, k2 und Δσ_Rsk(N*) nach Tabelle 6.3N; umgesetzt als Bemessungsfall

### en1992 F-04
- before EN: Covers the fatigue verification of steel with the damage equivalent stress range; realised as a design case
- before DE: Deckt den Ermüdungsnachweis des Stahls mit der schädigungsäquivalenten Spannungsschwingbreite ab; umgesetzt als Bemessungsfall
- after EN: Steel fatigue γ_F,fat Δσ_s,equ(N*) ≤ Δσ_Rsk(N*)/γ_s,fat or damage D_Ed = Σ n_i/N_i ≤ 1; realised as a design case
- after DE: Ermüdungsnachweis des Stahls γ_F,fat Δσ_s,equ(N*) ≤ Δσ_Rsk(N*)/γ_s,fat oder Schädigung D_Ed = Σ n_i/N_i ≤ 1; umgesetzt als Bemessungsfall

### en1992 F-05
- before EN: Covers fatigue of concrete in compression; realised as a design case
- before DE: Deckt die Ermüdung des Betons unter Druck ab; umgesetzt als Bemessungsfall
- after EN: Concrete compression fatigue Ecd,max,equ + 0.43 √(1 − Requ) ≤ 1 with fcd,fat = k1 β_cc(t0) fcd (1 − fck/250); realised as a design case
- after DE: Ermüdung des Betons unter Druck Ecd,max,equ + 0,43 √(1 − Requ) ≤ 1 mit fcd,fat = k1 β_cc(t0) fcd (1 − fck/250); umgesetzt als Bemessungsfall

### en1992 F-06
- before EN: Covers shear fatigue of concrete without links; realised as a design case
- before DE: Deckt die Querkraftermüdung des Betons ohne Bügel ab; umgesetzt als Bemessungsfall
- after EN: Shear fatigue of concrete without links VEd,max,equ/VRd,c ≤ 0.5 + 0.45 VEd,min,equ/VRd,c ≤ 0.9; realised as a design case
- after DE: Querkraftermüdung des Betons ohne Bügel VEd,max,equ/VRd,c ≤ 0,5 + 0,45 VEd,min,equ/VRd,c ≤ 0,9; umgesetzt als Bemessungsfall

### en1992 F-07
- before EN: Covers the equivalent damage lambda method for bridges; realised as a design case
- before DE: Deckt das Verfahren mit schädigungsäquivalentem Faktor lambda für Brücken ab; umgesetzt als Bemessungsfall
- after EN: Equivalent damage λ-method for bridges Δσ_s,equ = φ_fat Δσ_s,71 λ_s with λ_s = λ_s,1 λ_s,2 λ_s,3 λ_s,4 ≤ λ_s,max; realised as a design case
- after DE: Äquivalentes Schädigungsverfahren λ für Brücken Δσ_s,equ = φ_fat Δσ_s,71 λ_s mit λ_s = λ_s,1 λ_s,2 λ_s,3 λ_s,4 ≤ λ_s,max; umgesetzt als Bemessungsfall

### en1992 PR-01
- before EN: Also carries the dangling and orphan prestressing steel reference rows (5.10).
- before DE: Trägt zusätzlich die Zeilen für ins Leere laufende und verwaiste Spannstahlverweise (5.10).
- after EN: Maximum stressing force Pmax = Ap σ_p,max with σ_p,max = min(k1 fpk, k2 fp0.1k); also carries the dangling and orphan prestressing steel reference rows (5.10)
- after DE: Höchstkraft Pmax = Ap σ_p,max mit σ_p,max = min(k1 fpk, k2 fp0,1k); trägt zusätzlich die Zeilen für hängende und verwaiste Spannstahl-Verweise (5.10)

### en1992 PR-02
- before EN: Covers the initial prestress force; realised as a check family
- before DE: Deckt die Vorspannkraft zu Beginn ab; umgesetzt als Nachweisfamilie
- after EN: Initial prestress force Pm0 = Ap σ_pm0 with σ_pm0 ≤ min(k7 fpk, k8 fp0.1k); realised as a check family
- after DE: Anfangsvorspannkraft Pm0 = Ap σ_pm0 mit σ_pm0 ≤ min(k7 fpk, k8 fp0,1k); umgesetzt als Nachweisfamilie

### en1992 PR-03
- before EN: Covers immediate losses in pre-tensioning; realised as a check family
- before DE: Deckt sofortige Spannkraftverluste bei Vorspannung mit sofortigem Verbund ab; umgesetzt als Nachweisfamilie
- after EN: Immediate losses in pre-tensioning from elastic shortening Δσ_el = α_e σ_c,p; realised as a check family
- after DE: Sofortige Verluste bei Vorspannung mit sofortigem Verbund aus elastischer Verkürzung Δσ_el = α_e σ_c,p; umgesetzt als Nachweisfamilie

### en1992 PR-04
- before EN: Covers immediate losses in post-tensioning; realised as a check family
- before DE: Deckt sofortige Spannkraftverluste bei Vorspannung mit nachträglichem Verbund ab; umgesetzt als Nachweisfamilie
- after EN: Immediate losses in post-tensioning from friction ΔPμ(x) = Pmax (1 − e^(−μ(θ + kx))), wedge slip and elastic shortening; realised as a check family
- after DE: Sofortige Verluste bei Vorspannung mit nachträglichem Verbund aus Reibung ΔPμ(x) = Pmax (1 − e^(−μ(θ + kx))), Keilschlupf und elastischer Verkürzung; umgesetzt als Nachweisfamilie

### en1992 PR-05
- before EN: Covers time-dependent prestress losses; realised as a design case
- before DE: Deckt zeitabhängige Spannkraftverluste ab; umgesetzt als Bemessungsfall
- after EN: Time-dependent losses from shrinkage, creep and relaxation Δσ_p,c+s+r; realised as a design case
- after DE: Zeitabhängige Spannkraftverluste aus Schwinden, Kriechen und Relaxation Δσ_p,c+s+r; umgesetzt als Bemessungsfall

### en1992 PR-06
- before EN: Covers bending of prestressed members at the ultimate limit state; realised as a design case
- before DE: Deckt die Biegung vorgespannter Bauteile im Grenzzustand der Tragfähigkeit ab; umgesetzt als Bemessungsfall
- after EN: Ultimate bending of prestressed members from the pre-strain plus strain increment, with Δσ_p,ULS for unbonded tendons; realised as a design case
- after DE: Biegetragfähigkeit vorgespannter Bauteile aus Vordehnung und Dehnungszuwachs, bei Spanngliedern ohne Verbund mit Δσ_p,ULS; umgesetzt als Bemessungsfall

### en1992 PR-07
- before EN: Covers prestress effects at the ultimate limit state; realised as a design case
- before DE: Deckt Vorspannwirkungen im Grenzzustand der Tragfähigkeit ab; umgesetzt als Bemessungsfall
- after EN: Prestress effects at ULS with γ_p = 1.0 and Pk,sup = rsup Pm,t, Pk,inf = rinf Pm,t; realised as a design case
- after DE: Vorspannwirkungen im Grenzzustand der Tragfähigkeit mit γ_p = 1,0 und Pk,sup = rsup Pm,t, Pk,inf = rinf Pm,t; umgesetzt als Bemessungsfall

### en1992 PR-08
- before EN: Covers prestress deviation forces; realised as a design case
- before DE: Deckt Umlenkkräfte der Vorspannung ab; umgesetzt als Bemessungsfall
- after EN: Prestress deviation forces q = P/r (parabolic q = 8 P f/l²) and bursting reinforcement T = 0.25 (1 − a/h) P; realised as a design case
- after DE: Umlenkkräfte der Vorspannung q = P/r (parabolisch q = 8 P f/l²) und Spaltzugbewehrung T = 0,25 (1 − a/h) P; umgesetzt als Bemessungsfall

### en1992 PR-09
- before EN: Covers anchorage zones of post-tensioned members; realised as a design case
- before DE: Deckt Verankerungsbereiche nachträglich vorgespannter Bauteile ab; umgesetzt als Bemessungsfall
- after EN: Anchorage zones of post-tensioned members: bursting T = 0.25 (1 − a/h) Pmax, spalling and local zone FRdu per PA-01; realised as a design case
- after DE: Verankerungsbereiche nachträglich vorgespannter Bauteile: Spaltzug T = 0,25 (1 − a/h) Pmax, Abplatzen und örtlicher Bereich FRdu nach PA-01; umgesetzt als Bemessungsfall

### en1992 PR-10
- before EN: Covers the limitation of concrete stress at release; realised as a design case
- before DE: Deckt die Begrenzung der Betonspannung beim Vorspannen ab; umgesetzt als Bemessungsfall
- after EN: Concrete stress at release σ_c ≤ 0.6 fck(t) at transfer; realised as a design case
- after DE: Betonspannung beim Abspannen σ_c ≤ 0,6 fck(t) zum Zeitpunkt der Übertragung; umgesetzt als Bemessungsfall

### en1992 DT-01
- before EN: Covers minimum and maximum tension reinforcement of beams; realised as a check family
- before DE: Deckt Mindest- und Höchstzugbewehrung von Balken ab; umgesetzt als Nachweisfamilie
- after EN: Minimum and maximum tension reinforcement of beams As,min = max(0.26 fctm/fyk bt d, 0.0013 bt d) and As,max = 0.04 Ac; realised as a check family
- after DE: Mindest- und Höchstzugbewehrung von Balken As,min = max(0,26 fctm/fyk bt d, 0,0013 bt d) und As,max = 0,04 Ac; umgesetzt als Nachweisfamilie

### en1992 DT-02
- before EN: Covers reinforcement at end supports and the shift rule; realised as a design case
- before DE: Deckt Bewehrung an Endauflagern und das Versatzmaß ab; umgesetzt als Bemessungsfall
- after EN: Reinforcement at end supports As ≥ 0.25 As,span and the shift rule with a_l = z (cot θ − cot α)/2; realised as a design case
- after DE: Bewehrung an Endauflagern As ≥ 0,25 As,Feld und Versatzmaß a_l = z (cot θ − cot α)/2; umgesetzt als Bemessungsfall

### en1992 DT-03
- before EN: Covers shear reinforcement of beams; realised as a design case
- before DE: Deckt die Querkraftbewehrung von Balken ab; umgesetzt als Bemessungsfall
- after EN: Minimum shear reinforcement ratio ρ_w,min = 0.16 fctm/fyk and maximum spacing sl,max = 0.75 d (1 + cot α); realised as a design case
- after DE: Mindestquerkraftbewehrungsgrad ρ_w,min = 0,16 fctm/fyk und Höchstabstand sl,max = 0,75 d (1 + cot α); umgesetzt als Bemessungsfall

### en1992 DT-04
- before EN: Covers torsion links; realised as a design case
- before DE: Deckt Torsionsbügel ab; umgesetzt als Bemessungsfall
- after EN: Closed torsion links with spacing s ≤ min(u/8, sl,max, b); realised as a design case
- after DE: Geschlossene Torsionsbügel mit Abstand s ≤ min(u/8, sl,max, b); umgesetzt als Bemessungsfall

### en1992 DT-05
- before EN: Covers surface reinforcement; realised as a design case
- before DE: Deckt die Oberflächenbewehrung ab; umgesetzt als Bemessungsfall
- after EN: Surface reinforcement As,surf,min = 0.01 Act,ext perpendicular to the main bars at spacing ≤ 150 mm; realised as a design case
- after DE: Oberflächenbewehrung As,surf,min = 0,01 Act,ext rechtwinklig zur Hauptbewehrung im Abstand ≤ 150 mm; umgesetzt als Bemessungsfall

### en1992 DT-06
- before EN: Covers reinforcement of solid slabs; realised as a design case
- before DE: Deckt die Bewehrung von Vollplatten ab; umgesetzt als Bemessungsfall
- after EN: Reinforcement of solid slabs: secondary reinforcement ≥ 0.2 As and maximum spacing min(3h, 400 mm) principal and min(3.5h, 450 mm) secondary; realised as a design case
- after DE: Bewehrung von Vollplatten: Querbewehrung ≥ 0,2 As und Höchstabstand min(3h, 400 mm) in Haupt- und min(3,5h, 450 mm) in Querrichtung; umgesetzt als Bemessungsfall

### en1992 DT-07
- before EN: Covers shear reinforcement of slabs; realised as a design case
- before DE: Deckt die Querkraftbewehrung von Platten ab; umgesetzt als Bemessungsfall
- after EN: Shear reinforcement of slabs only for h ≥ 200 mm, with minimum links when VEd ≤ VRd,max/3 and none permitted if VEd ≤ VRd,c; realised as a design case
- after DE: Querkraftbewehrung von Platten nur für h ≥ 200 mm, mit Mindestbügeln bei VEd ≤ VRd,max/3 und ohne Bügel zulässig bei VEd ≤ VRd,c; umgesetzt als Bemessungsfall

### en1992 DT-08
- before EN: Covers reinforcement of flat slabs at internal columns and edges; realised as a design case
- before DE: Deckt die Bewehrung von Flachdecken an Innenstützen und Rändern ab; umgesetzt als Bemessungsfall
- after EN: Flat slab reinforcement at internal columns and edges: top bars within 0.125 of the panel width each side of the column and bottom bars through the column; realised as a design case
- after DE: Flachdeckenbewehrung an Innenstützen und Rändern: obere Bewehrung innerhalb 0,125 der Feldbreite beidseits der Stütze und untere Bewehrung durch die Stütze; umgesetzt als Bemessungsfall

### en1992 DT-09
- before EN: Covers column detailing; realised as a design case
- before DE: Deckt die Bewehrungsführung von Stützen ab; umgesetzt als Bemessungsfall
- after EN: Column detailing: minimum bar diameter, As,max = 0.04 Ac (0.08 Ac at laps), minimum bar count and link diameter and spacing; realised as a design case
- after DE: Stützenbewehrung: Mindeststabdurchmesser, As,max = 0,04 Ac (0,08 Ac in Stößen), Mindeststabanzahl sowie Bügeldurchmesser und -abstand; umgesetzt als Bemessungsfall

### en1992 DT-10
- before EN: Covers wall detailing; realised as a design case
- before DE: Deckt die Bewehrungsführung von Wänden ab; umgesetzt als Bemessungsfall
- after EN: Wall detailing: vertical As,v,min = 0.002 Ac and maximum 0.04 Ac, horizontal As,h,min = max(0.25 As,v, 0.001 Ac), spacing ≤ min(3 tw, 400 mm); realised as a design case
- after DE: Wandbewehrung: Vertikalbewehrung As,v,min = 0,002 Ac und höchstens 0,04 Ac, Horizontalbewehrung As,h,min = max(0,25 As,v, 0,001 Ac), Abstand ≤ min(3 tw, 400 mm); umgesetzt als Bemessungsfall

### en1992 DT-11
- before EN: Covers deep beam detailing; realised as a design case
- before DE: Deckt die Bewehrungsführung von wandartigen Trägern ab; umgesetzt als Bemessungsfall
- after EN: Deep beam surface reinforcement as orthogonal mesh of at least 0.001 Ac and 150 mm²/m with spacing ≤ min(2h, 300 mm) per face; realised as a design case
- after DE: Oberflächenbewehrung wandartiger Träger als orthogonales Netz von mindestens 0,001 Ac und 150 mm²/m im Abstand ≤ min(2h, 300 mm) je Seite; umgesetzt als Bemessungsfall

### en1992 DT-12
- before EN: Covers foundation detailing; realised as a design case
- before DE: Deckt die Bewehrungsführung von Fundamenten ab; umgesetzt als Bemessungsfall
- after EN: Foundation detailing: pile cap ties from the strut-and-tie model ST-04 with bands over piles, footings by Annex J and column joint by P-06 and B-03; realised as a design case
- after DE: Gründungsbewehrung: Zugbänder von Pfahlkappen aus dem Stabwerkmodell ST-04 mit Bändern über den Pfählen, Fundamente nach Anhang J und Stützenanschluss nach P-06 und B-03; umgesetzt als Bemessungsfall

### en1992 DT-13
- before EN: Covers detailing of regions with discontinuity; realised as a design case
- before DE: Deckt die Bewehrungsführung in Diskontinuitätsbereichen ab; umgesetzt als Bemessungsfall
- after EN: Regions with discontinuity (Annex J nodes, corner joints, frame corners) with corner force Ftd and anchorage radius r ≥ φ_m; realised as a design case
- after DE: Diskontinuitätsbereiche (Anhang J: Knoten, Eckverbindungen, Rahmenecken) mit Eckkraft Ftd und Biegerollenradius r ≥ φ_m; umgesetzt als Bemessungsfall

### en1992 DT-14
- before EN: Covers tying systems; realised as a design case
- before DE: Deckt Zugankersysteme ab; umgesetzt als Bemessungsfall
- after EN: Tying systems: peripheral ties Ftie,per = li q1 ≤ q2 (q1 = 10 kN/m, q2 = 70 kN), internal ties 20 kN/m and horizontal column and wall ties 150 kN/m; realised as a design case
- after DE: Zugankersysteme: Randanker Ftie,per = li q1 ≤ q2 (q1 = 10 kN/m, q2 = 70 kN), Innenanker 20 kN/m und horizontale Stützen- und Wandanker 150 kN/m; umgesetzt als Bemessungsfall

### en1992 R-01
- before EN: Covers the tabulated fire design data; realised as a design case
- before DE: Deckt die Tabellendaten der Brandbemessung ab; umgesetzt als Bemessungsfall
- after EN: Tabulated fire design data per member type (columns, walls, tensile members, beams, slabs) with minimum width and axis distance for the fire resistance class; realised as a design case
- after DE: Tabellenverfahren der Brandbemessung je Bauteilart (Stützen, Wände, Zugglieder, Balken, Platten) mit Mindestbreite und Achsabstand für die Feuerwiderstandsklasse; umgesetzt als Bemessungsfall

### en1992 R-02
- before EN: Covers temperature profiles for fire design; realised as a design case
- before DE: Deckt Temperaturprofile für die Brandbemessung ab; umgesetzt als Bemessungsfall
- after EN: Temperature profiles of the standard fire θ_g = 20 + 345 log10(8t + 1) and the resulting section temperatures; realised as a design case
- after DE: Temperaturprofile des Normbrands θ_g = 20 + 345 log10(8t + 1) und die daraus folgenden Querschnittstemperaturen; umgesetzt als Bemessungsfall

### en1992 R-03
- before EN: Also carries the axis-distance and minimum-width rows of the tabulated method (R-01, 5.1-5.7) that the same fire member check emits.
- before DE: Trägt zusätzlich die Zeilen für Achsabstand und Mindestbreite des Tabellenverfahrens (R-01, 5.1-5.7), die derselbe Brandnachweis des Bauteils ausgibt.
- after EN: 500 °C isotherm method with the reduced section removing concrete above 500 °C and the damaged zone a500 (4.2.3, Annex B); also carries the axis-distance and minimum-width rows of the tabulated method (R-01, 5.1-5.7) that the same fire member check emits
- after DE: 500-°C-Isothermen-Methode mit reduziertem Querschnitt ohne Beton über 500 °C und geschädigter Zone a500 (4.2.3, Anhang B); trägt zusätzlich die Zeilen für Achsabstand und Mindestbreite des Tabellenverfahrens (R-01, 5.1-5.7), die derselbe Brand-Bauteilnachweis ausgibt

### en1992 R-04
- before EN: Covers the zone method for fire design; realised as a check family
- before DE: Deckt die Zonenmethode der Brandbemessung ab; umgesetzt als Nachweisfamilie
- after EN: Zone method with mean strength factor kc,m = (1 − 0.2/n)(1/n) Σ kc(θ_i) and damage thickness az = w (1 − kc,m/kc(θ_M)); realised as a check family
- after DE: Zonenmethode mit mittlerem Festigkeitsbeiwert kc,m = (1 − 0,2/n)(1/n) Σ kc(θ_i) und Schädigungstiefe az = w (1 − kc,m/kc(θ_M)); umgesetzt als Nachweisfamilie

### en1992 R-05
- before EN: Covers the simplified calculation of columns exposed to fire; realised as a design case
- before DE: Deckt die vereinfachte Berechnung brandbeanspruchter Stützen ab; umgesetzt als Bemessungsfall
- after EN: Simplified calculation of columns exposed to fire with M_Rd,fi(N_Ed,fi) at the fire slenderness λ_fi ≤ 80; realised as a design case
- after DE: Vereinfachte Berechnung brandbeanspruchter Stützen mit M_Rd,fi(N_Ed,fi) bei Brandschlankheit λ_fi ≤ 80; umgesetzt als Bemessungsfall

### en1992 R-06
- before EN: Covers fire action and load combination; realised as a design case
- before DE: Deckt Brandeinwirkung und Lastkombination ab; umgesetzt als Bemessungsfall
- after EN: Fire action Ed,fi = Σ γ_GA G_k + (ψ1,1 or ψ2,1) Q_k,1 + Σ ψ2,i Q_k,i + Ad(t) with γ_GA = 1.0 and η_fi; realised as a design case
- after DE: Brandeinwirkung Ed,fi = Σ γ_GA G_k + (ψ1,1 oder ψ2,1) Q_k,1 + Σ ψ2,i Q_k,i + Ad(t) mit γ_GA = 1,0 und η_fi; umgesetzt als Bemessungsfall

### en1992 R-07
- before EN: Covers material properties at elevated temperature; realised as a design case
- before DE: Deckt Baustoffeigenschaften bei erhöhter Temperatur ab; umgesetzt als Bemessungsfall
- after EN: Concrete and steel strength reduction factors kc(θ) and ks(θ) and strains at elevated temperature (Table 3.1); realised as a design case
- after DE: Festigkeitsabminderungsbeiwerte kc(θ) und ks(θ) und Dehnungen von Beton und Stahl bei erhöhter Temperatur (Tabelle 3.1); umgesetzt als Bemessungsfall

### en1992 R-08
- before EN: Covers shear, torsion and anchorage in fire; realised as a design case
- before DE: Deckt Querkraft, Torsion und Verankerung im Brandfall ab; umgesetzt als Bemessungsfall
- after EN: Shear, torsion and anchorage in fire on the reduced section, with spalling measures for fck > C80/95 such as polypropylene fibres; realised as a design case
- after DE: Querkraft, Torsion und Verankerung im Brandfall am reduzierten Querschnitt, mit Abplatzmaßnahmen für fck > C80/95 wie Polypropylenfasern; umgesetzt als Bemessungsfall

### en1992 BR-01
- before EN: Covers materials for bridges; realised as a design case
- before DE: Deckt Baustoffe für Brücken ab; umgesetzt als Bemessungsfall
- after EN: Bridge materials with α_cc = 0.85 for persistent situations, fck ≤ C70/85 and the fatigue strength fcd,fat of F-05; realised as a design case
- after DE: Baustoffe für Brücken mit α_cc = 0,85 in ständigen Situationen, fck ≤ C70/85 und der Ermüdungsfestigkeit fcd,fat nach F-05; umgesetzt als Bemessungsfall

### en1992 BR-02
- before EN: Covers concrete cover in bridges; realised as a design case
- before DE: Deckt die Betondeckung bei Brücken ab; umgesetzt als Bemessungsfall
- after EN: Cover in bridges from the structural class for a 100 year design life, Δc,dev = 10 mm and the added chloride allowance for XD3 and XS3; realised as a design case
- after DE: Betondeckung bei Brücken aus der Bauwerksklasse für 100 Jahre Nutzungsdauer, Δc,dev = 10 mm und dem Zuschlag für Chloride bei XD3 und XS3; umgesetzt als Bemessungsfall

### en1992 BR-03
- before EN: Covers shear in bridges; realised as a design case
- before DE: Deckt Querkraft bei Brücken ab; umgesetzt als Bemessungsfall
- after EN: Shear in bridges with CRd,c = 0.18/γ_c (EN) or 0.15/γ_c (DE), 1.0 ≤ cot θ ≤ 2.5 and α_cw = 1.0; realised as a design case
- after DE: Querkraft bei Brücken mit CRd,c = 0,18/γ_c (EN) bzw. 0,15/γ_c (DE), 1,0 ≤ cot θ ≤ 2,5 und α_cw = 1,0; umgesetzt als Bemessungsfall

### en1992 BR-04
- before EN: Covers fatigue of bridges; realised as a design case
- before DE: Deckt Ermüdung von Brücken ab; umgesetzt als Bemessungsfall
- after EN: Fatigue of bridges by the λ coefficients of F-07 for road and rail traffic and Δσ_Rsk, including tendons in curved ducts and couplers; realised as a design case
- after DE: Ermüdung von Brücken über die λ-Beiwerte nach F-07 für Straßen- und Eisenbahnverkehr und Δσ_Rsk, einschließlich Spannglieder in gekrümmten Hüllrohren und Kopplungen; umgesetzt als Bemessungsfall

### en1992 BR-05
- before EN: Covers crack control in bridges; realised as a design case
- before DE: Deckt die Rissbreitenbegrenzung bei Brücken ab; umgesetzt als Bemessungsfall
- after EN: Crack control in bridges: wk ≤ 0.2 mm for prestressed members or decompression for bonded tendons in XD and XS, with minimum reinforcement As,min; realised as a design case
- after DE: Rissbreitenbegrenzung bei Brücken: wk ≤ 0,2 mm bei vorgespannten Bauteilen oder Dekompression bei Spanngliedern mit Verbund in XD und XS, mit Mindestbewehrung As,min; umgesetzt als Bemessungsfall

### en1992 BR-06
- before EN: Covers prestress in bridges; realised as a design case
- before DE: Deckt Vorspannung bei Brücken ab; umgesetzt als Bemessungsfall
- after EN: Prestress in bridges: deviation forces of external tendons PR-08, hyperstatic effect M_sec = M_total − P e and staged time-dependent losses; realised as a design case
- after DE: Vorspannung bei Brücken: Umlenkkräfte externer Spannglieder PR-08, statisch unbestimmte Wirkung M_sec = M_total − P e und abschnittsweise zeitabhängige Verluste; umgesetzt als Bemessungsfall

### en1992 BR-07
- before EN: Covers bearings, anchorage, diaphragms and cantilever slab punching under wheel loads; realised as a design case
- before DE: Deckt Lager, Verankerung, Querträger und Durchstanzen von Kragplatten unter Radlasten ab; umgesetzt als Bemessungsfall
- after EN: Bearings, anchorage, diaphragms and cantilever slab punching under LM1 wheel loads of 0.4 x 0.4 m with 45° dispersion and β = 1.0; realised as a design case
- after DE: Lager, Verankerung, Querträger und Durchstanzen von Kragplatten unter LM1-Radlasten von 0,4 x 0,4 m mit 45°-Ausbreitung und β = 1,0; umgesetzt als Bemessungsfall

### en1992 BR-08
- before EN: Covers piers and compression members with large geometry; realised as a design case
- before DE: Deckt Pfeiler und Druckglieder mit großen Abmessungen ab; umgesetzt als Bemessungsfall
- after EN: Piers and compression members of large geometry with biaxial effects and creep per A-09 to A-12 and k1 = √(fck/20); realised as a design case
- after DE: Pfeiler und Druckglieder großer Abmessung mit zweiachsigen Effekten und Kriechen nach A-09 bis A-12 und k1 = √(fck/20); umgesetzt als Bemessungsfall

### en1992 LQ-01
- before EN: Covers tightness classes; realised as a design case
- before DE: Deckt Dichtheitsklassen ab; umgesetzt als Bemessungsfall
- after EN: Tightness classes 0 to 3 with limiting crack width wk,lim by hD/h, from 0.2 mm at hD/h ≤ 5 down to 0.05 mm at hD/h ≥ 35 for class 1; realised as a design case
- after DE: Dichtheitsklassen 0 bis 3 mit Grenzrissbreite wk,lim in Abhängigkeit von hD/h, bei Klasse 1 von 0,2 mm bei hD/h ≤ 5 bis 0,05 mm bei hD/h ≥ 35; umgesetzt als Bemessungsfall

### en1992 LQ-02
- before EN: Covers minimum reinforcement and crack control of restrained members; realised as a design case
- before DE: Deckt Mindestbewehrung und Rissbreitenbegrenzung zwangbeanspruchter Bauteile ab; umgesetzt als Bemessungsfall
- after EN: Minimum reinforcement and crack control of restrained members As,min = kc k fct,eff Act/σ_s with early-age fct,eff = fctm(t) at 3 d; realised as a design case
- after DE: Mindestbewehrung und Rissbreitenbegrenzung zwangbeanspruchter Bauteile As,min = kc k fct,eff Act/σ_s mit fct,eff = fctm(t) im Alter von 3 d; umgesetzt als Bemessungsfall

### en1992 LQ-03
- before EN: Covers crack width calculation for liquid-retaining structures; realised as a design case
- before DE: Deckt die Rissbreitenberechnung für flüssigkeitsdichte Bauwerke ab; umgesetzt als Bemessungsfall
- after EN: Crack width of liquid-retaining structures wk per SL-06 with kt = 0.4 and sr,max = 3.4c + 0.425 k1 k2 φ/ρ_p,eff under hydrostatic head hD; realised as a design case
- after DE: Rissbreite von Flüssigkeitsbehältern wk nach SL-06 mit kt = 0,4 und sr,max = 3,4c + 0,425 k1 k2 φ/ρ_p,eff unter hydrostatischer Druckhöhe hD; umgesetzt als Bemessungsfall

### en1992 LQ-04
- before EN: Covers self-healing crack width limits; realised as a design case
- before DE: Deckt Rissbreitengrenzen für selbstheilende Risse ab; umgesetzt als Bemessungsfall
- after EN: Self-healing crack width limits of up to 0.2 mm at hD/h ≤ 5 for tightness class 1 (Table 7.105); realised as a design case
- after DE: Grenzrissbreiten bei Selbstheilung bis 0,2 mm bei hD/h ≤ 5 für Dichtheitsklasse 1 (Tabelle 7.105); umgesetzt als Bemessungsfall

### en1992 LQ-05
- before EN: Covers concrete cover of liquid tanks; realised as a design case
- before DE: Deckt die Betondeckung von Behältern ab; umgesetzt als Bemessungsfall
- after EN: Cover of liquid tanks increased by Δc,dur,add and protective linings against aggressive liquids of the XA classes; realised as a design case
- after DE: Betondeckung von Flüssigkeitsbehältern, um Δc,dur,add erhöht, und Schutzschichten gegen angreifende Flüssigkeiten der XA-Klassen; umgesetzt als Bemessungsfall

### en1992 LQ-06
- before EN: Covers thermal and shrinkage restraint and prestressed tanks; realised as a design case
- before DE: Deckt Zwang aus Temperatur und Schwinden sowie vorgespannte Behälter ab; umgesetzt als Bemessungsfall
- after EN: Thermal and shrinkage restraint and hoop prestress of prestressed tanks with a compression zone of at least 1.0 MPa; realised as a design case
- after DE: Zwang aus Temperatur und Schwinden sowie Ringvorspannung vorgespannter Behälter mit einer Druckzone von mindestens 1,0 MPa; umgesetzt als Bemessungsfall

### en1992 LQ-07
- before EN: Covers details such as joints, water bars and wall-slab corners; realised as a design case
- before DE: Deckt Details wie Fugen, Fugenbänder und Wand-Platten-Ecken ab; umgesetzt als Bemessungsfall
- after EN: Detailing checklist for joints, water bars and wall-slab corners of liquid-retaining structures (Section 8); realised as a design case
- after DE: Konstruktionscheckliste für Fugen, Fugenbänder und Wand-Platten-Ecken von Flüssigkeitsbehältern (Abschnitt 8); umgesetzt als Bemessungsfall

### en1992 FA-01
- before EN: Covers anchor load distribution; realised as a design case
- before DE: Deckt die Lastverteilung auf die Dübel ab; umgesetzt als Bemessungsfall
- after EN: Anchor load distribution NEd,i = NEd/n ± M Ai/I with shear shared equally over the anchors; realised as a design case
- after DE: Lastverteilung auf die Anker NEd,i = NEd/n ± M Ai/I mit gleichmäßig aufgeteilter Querkraft; umgesetzt als Bemessungsfall

### en1992 FA-02
- before EN: Also carries the national annex edition row of the anchor checks.
- before DE: Trägt zusätzlich die Zeile der Ausgabe des Nationalen Anhangs der Dübelnachweise.
- after EN: Steel failure of anchors in tension NRk,s = As fuk and NRd,s = NRk,s/γ_Ms (7.2.1.2); also carries the national annex edition row of the anchor checks
- after DE: Stahlversagen von Ankern auf Zug NRk,s = As fuk und NRd,s = NRk,s/γ_Ms (7.2.1.2); trägt zusätzlich die Zeile der Ausgabe des Nationalen Anhangs der Ankernachweise

### en1992 FA-03
- before EN: Covers concrete cone breakout in tension; realised as a check family
- before DE: Deckt den Betonausbruch bei Zugbeanspruchung ab; umgesetzt als Nachweisfamilie
- after EN: Concrete cone breakout in tension NRk,c = N0Rk,c (Ac,N/A0c,N) ψ_s,N ψ_re,N ψ_ec,N ψ_M,N with N0Rk,c = k1 √fck hef^1.5; realised as a check family
- after DE: Betonausbruch auf Zug NRk,c = N0Rk,c (Ac,N/A0c,N) ψ_s,N ψ_re,N ψ_ec,N ψ_M,N mit N0Rk,c = k1 √fck hef^1,5; umgesetzt als Nachweisfamilie

### en1992 FA-04
- before EN: Covers pull-out failure; realised as a check family
- before DE: Deckt das Herausziehen ab; umgesetzt als Nachweisfamilie
- after EN: Pull-out failure NRk,p = k2 Ah fck with k2 = 7.5 cracked or 10.5 uncracked and the head bearing area Ah; realised as a check family
- after DE: Herausziehen NRk,p = k2 Ah fck mit k2 = 7,5 gerissen bzw. 10,5 ungerissen und der Kopfauflagerfläche Ah; umgesetzt als Nachweisfamilie

### en1992 FA-05
- before EN: Covers splitting failure; realised as a check family
- before DE: Deckt das Spalten des Betons ab; umgesetzt als Nachweisfamilie
- after EN: Splitting failure NRk,sp = N0Rk,sp (Ac,N/A0c,N) ψ_s,N ψ_re,N ψ_ec,N ψ_h,sp with N0Rk,sp = min(N0Rk,c, NRk,p); realised as a check family
- after DE: Spaltversagen NRk,sp = N0Rk,sp (Ac,N/A0c,N) ψ_s,N ψ_re,N ψ_ec,N ψ_h,sp mit N0Rk,sp = min(N0Rk,c, NRk,p); umgesetzt als Nachweisfamilie

### en1992 FA-06
- before EN: Covers side-face blow-out; realised as a design case
- before DE: Deckt den seitlichen Betonausbruch ab; umgesetzt als Bemessungsfall
- after EN: Side-face blow-out NRk,cb = N0Rk,cb (Ac,Nb/A0c,Nb) … with N0Rk,cb = 8.7 c1 √Ah √fck for c1 ≤ 0.5 hef; realised as a design case
- after DE: Seitlicher Betonausbruch NRk,cb = N0Rk,cb (Ac,Nb/A0c,Nb) … mit N0Rk,cb = 8,7 c1 √Ah √fck für c1 ≤ 0,5 hef; umgesetzt als Bemessungsfall

### en1992 FA-07
- before EN: Covers steel failure in shear without lever arm; realised as a check family
- before DE: Deckt Stahlversagen bei Querzug ohne Hebelarm ab; umgesetzt als Nachweisfamilie
- after EN: Steel failure in shear without lever arm VRk,s = 0.5 As fuk and with lever arm VRk,s = α_M Mrk,s/lz; realised as a check family
- after DE: Stahlversagen auf Querzug ohne Hebelarm VRk,s = 0,5 As fuk und mit Hebelarm VRk,s = α_M Mrk,s/lz; umgesetzt als Nachweisfamilie

### en1992 FA-08
- before EN: Covers concrete pry-out; realised as a check family
- before DE: Deckt den Betonausbruch auf der lastabgewandten Seite ab; umgesetzt als Nachweisfamilie
- after EN: Concrete pry-out VRk,cp = k8 NRk,c with k8 = 1.0 for hef < 60 mm and 2.0 otherwise; realised as a check family
- after DE: Betonausbruch auf der lastabgewandten Seite VRk,cp = k8 NRk,c mit k8 = 1,0 für hef < 60 mm und sonst 2,0; umgesetzt als Nachweisfamilie

### en1992 FA-09
- before EN: Covers concrete edge failure in shear; realised as a check family
- before DE: Deckt den Betonkantenbruch bei Querzug ab; umgesetzt als Nachweisfamilie
- after EN: Concrete edge failure in shear VRk,c = V0Rk,c (Ac,V/A0c,V) ψ_s,V ψ_h,V ψ_α,V ψ_ec,V ψ_re,V with V0Rk,c = k9 d_nom^α l_f^β √fck c1^1.5; realised as a check family
- after DE: Betonkantenbruch auf Querzug VRk,c = V0Rk,c (Ac,V/A0c,V) ψ_s,V ψ_h,V ψ_α,V ψ_ec,V ψ_re,V mit V0Rk,c = k9 d_nom^α l_f^β √fck c1^1,5; umgesetzt als Nachweisfamilie

### en1992 FA-10
- before EN: Covers combined tension and shear; realised as a check family
- before DE: Deckt die Kombination von Zug und Querzug ab; umgesetzt als Nachweisfamilie
- after EN: Combined tension and shear (NEd/NRd,s)² + (VEd/VRd,s)² ≤ 1 for steel and (NEd/NRd)^1.5 + (VEd/VRd)^1.5 ≤ 1 for concrete; realised as a check family
- after DE: Kombination von Zug und Querzug (NEd/NRd,s)² + (VEd/VRd,s)² ≤ 1 für Stahl und (NEd/NRd)^1,5 + (VEd/VRd)^1,5 ≤ 1 für Beton; umgesetzt als Nachweisfamilie

### en1992 FA-11
- before EN: Covers steel failure and group effects of anchor groups; realised as a design case
- before DE: Deckt Stahlversagen und Gruppeneffekte von Dübelgruppen ab; umgesetzt als Bemessungsfall
- after EN: Group effects of anchor groups: overlapping cone footprints for Ac,N and the group eccentricity eN; realised as a design case
- after DE: Gruppenwirkung von Ankergruppen: überlappende Ausbruchkegelflächen für Ac,N und Gruppenausmitte eN; umgesetzt als Bemessungsfall

### en1992 FA-12
- before EN: Covers fatigue and seismic categories of anchors; realised as a design case
- before DE: Deckt Ermüdung und Erdbebenkategorien von Dübeln ab; umgesetzt als Bemessungsfall
- after EN: Fatigue and seismic categories C1 and C2 of anchors with ΔNRk,s,0,n per approval and the reductions α_gap, α_seis; realised as a design case
- after DE: Ermüdung und Erdbebenkategorien C1 und C2 von Ankern mit ΔNRk,s,0,n nach Zulassung und den Abminderungen α_gap, α_seis; umgesetzt als Bemessungsfall

### en1992 FA-13
- before EN: Covers concrete pry-out of anchor groups; realised as a design case
- before DE: Deckt den Betonausbruch auf der lastabgewandten Seite bei Dübelgruppen ab; umgesetzt als Bemessungsfall
- after EN: Concrete pry-out of anchor groups, following FA-08; realised as a design case
- after DE: Betonausbruch auf der lastabgewandten Seite bei Ankergruppen, anschließend an FA-08; umgesetzt als Bemessungsfall

### en1992 FA-14
- before EN: Covers reinforcement taking up anchor loads; realised as a design case
- before DE: Deckt Bewehrung zur Aufnahme der Dübellasten ab; umgesetzt als Bemessungsfall
- after EN: Anchor reinforcement taking up the anchor loads As,re = NEd,re/fyd, verified with the strut-and-tie model ST-04; realised as a design case
- after DE: Rückhängebewehrung zur Aufnahme der Ankerlasten As,re = NEd,re/fyd, nachgewiesen mit dem Stabwerkmodell ST-04; umgesetzt als Bemessungsfall

### en1993 V-011
- before EN: f_u/f_y only; elongation and epsilon_u are not carried
- before DE: nur f_u/f_y; Bruchdehnung und epsilon_u werden nicht geführt
- after EN: Ductility requirement f_u/f_y ≥ 1.10 (3.2.2); the elongation of 15 % and ε_u ≥ 15 ε_y are not carried
- after DE: Duktilitätsanforderung f_u/f_y ≥ 1,10 (3.2.2); die Bruchdehnung von 15 % und ε_u ≥ 15 ε_y werden nicht geführt

### en1993 V-012
- before EN: Table 2.1 cells are unverified registry rows
- before DE: Die Zellen der Tabelle 2.1 sind ungeprüfte Registerzeilen
- after EN: Fracture toughness requirements of 3.2.3 and EN 1993-1-10, delegated to V-900; the cells of Table 2.1 are unverified registry rows
- after DE: Zähigkeitsanforderungen nach 3.2.3 und EN 1993-1-10, delegiert an V-900; die Zellen der Tabelle 2.1 sind ungeprüfte Registerzeilen

### en1993 V-014
- before EN: bolt and consumable catalogue is not part of the snapshot; bolt class is a joint field
- before DE: Der Schrauben- und Zusatzwerkstoffkatalog ist nicht Teil des Snapshots; die Schraubenklasse ist ein Feld der Verbindung
- after EN: Bolt classes 4.6 to 10.9 with f_yb and f_ub (3.3); the bolt and consumable catalogue is not part of the snapshot, the bolt class is a joint field
- after DE: Schraubenklassen 4.6 bis 10.9 mit f_yb und f_ub (3.3); der Schrauben- und Zusatzwerkstoffkatalog ist nicht Teil des Schnappschusses, die Schraubenklasse ist ein Verbindungsfeld

### en1993 V-015
- before EN: built-in stainless grade table behind ndp.en1993.v-300.2 (unverified)
- before DE: eingebaute Tabelle nichtrostender Stahlsorten hinter ndp.en1993.v-300.2 (ungeprüft)
- after EN: Stainless steel grade table of EN 1993-1-4 Table 2.1 with f_y and f_u of austenitic, duplex and ferritic grades, built in behind ndp.en1993.v-300.2 (unverified)
- after DE: Güten-Tabelle nichtrostender Stähle nach EN 1993-1-4 Tabelle 2.1 mit f_y und f_u austenitischer, Duplex- und ferritischer Güten, eingebaut hinter ndp.en1993.v-300.2 (ungeprüft)

### en1993 V-024
- before EN: e_0/L of Table 5.1, inclusion rule 5.3.2(6), k e_0 for LTB
- before DE: e_0/L aus Tabelle 5.1, Einbezugsregel 5.3.2(6), k e_0 für Biegedrillknicken
- after EN: Local bow imperfection e_0/L of Table 5.1 by buckling curve for elastic and plastic analysis (5.3.2(3)(b)) with the inclusion rule 5.3.2(6)
- after DE: Lokale Vorkrümmung e_0/L nach Tabelle 5.1 je Knickspannungslinie für elastische und plastische Berechnung (5.3.2(3)(b)) mit der Einbeziehungsregel 5.3.2(6)

### en1993 V-025
- before EN: e_0/L of Table 5.1, inclusion rule 5.3.2(6), k e_0 for LTB
- before DE: e_0/L aus Tabelle 5.1, Einbezugsregel 5.3.2(6), k e_0 für Biegedrillknicken
- after EN: Equivalent imperfection for the stability analysis of members as a scaled buckling mode (5.3.3)
- after DE: Gleichwertige Ersatzimperfektion für den Stabilitätsnachweis von Bauteilen als skalierte Knickeigenform (5.3.3)

### en1993 V-026
- before EN: e_0/L of Table 5.1, inclusion rule 5.3.2(6), k e_0 for LTB
- before DE: e_0/L aus Tabelle 5.1, Einbezugsregel 5.3.2(6), k e_0 für Biegedrillknicken
- after EN: Member imperfection k e_0 with k = 0.5 in the weak-axis imperfection for lateral-torsional buckling by second-order torsional analysis (5.3.4)
- after DE: Bauteilimperfektion k e_0 mit k = 0,5 in der Imperfektion um die schwache Achse für das Biegedrillknicken nach Theorie II. Ordnung mit Torsion (5.3.4)

### en1993 V-028
- before EN: classification and effective class 4 section of rolled and welded I and hollow sections
- before DE: Klassifizierung und wirksamer Querschnitt der Klasse 4 für gewalzte und geschweißte I-Querschnitte und Hohlprofile
- after EN: Cross-section classification (5.5), delegated to V-030 to V-034, with the effective class 4 section of rolled and welded I and hollow sections
- after DE: Querschnittsklassifizierung (5.5), delegiert an V-030 bis V-034, mit wirksamem Querschnitt der Klasse 4 für gewalzte und geschweißte I- und Hohlquerschnitte

### en1993 V-030
- before EN: classification and effective class 4 section of rolled and welded I and hollow sections
- before DE: Klassifizierung und wirksamer Querschnitt der Klasse 4 für gewalzte und geschweißte I-Querschnitte und Hohlprofile
- after EN: Classification of internal compression parts (I webs, box flanges, RHS walls) by c/t against the ε limits (Table 5.2 sheet 1), with the effective class 4 section
- after DE: Klassifizierung innenliegender Druckteile (I-Stege, Kastenflansche, RHS-Wände) über c/t gegen die ε-Grenzen (Tabelle 5.2 Blatt 1), mit wirksamem Querschnitt der Klasse 4

### en1993 V-031
- before EN: classification and effective class 4 section of rolled and welded I and hollow sections
- before DE: Klassifizierung und wirksamer Querschnitt der Klasse 4 für gewalzte und geschweißte I-Querschnitte und Hohlprofile
- after EN: Classification of outstand flanges of I, H, T and channel sections by c/t ≤ 9ε, 10ε, 14ε for class 1, 2, 3 (Table 5.2 sheet 2)
- after DE: Klassifizierung überstehender Flansche von I-, H-, T- und U-Querschnitten über c/t ≤ 9ε, 10ε, 14ε für Klasse 1, 2, 3 (Tabelle 5.2 Blatt 2)

### en1993 V-032
- before EN: hollow sections only; angle and CHS tables are not wired to a section kind
- before DE: nur Hohlprofile; Winkel- und Kreishohlprofiltabellen sind keiner Querschnittsart zugeordnet
- after EN: Classification of angles (h/t ≤ 15ε, (b+h)/2t ≤ 11.5ε) and circular hollow sections (d/t ≤ 50ε², 70ε², 90ε²) (Table 5.2 sheet 2); wired for hollow sections only, the angle and CHS tables are not assigned to a section kind
- after DE: Klassifizierung von Winkeln (h/t ≤ 15ε, (b+h)/2t ≤ 11,5ε) und Kreishohlprofilen (d/t ≤ 50ε², 70ε², 90ε²) (Tabelle 5.2 Blatt 2); nur für Hohlprofile angebunden, die Winkel- und Kreishohlprofiltabellen sind keiner Querschnittsart zugeordnet

### en1993 V-033
- before EN: classification and effective class 4 section of rolled and welded I and hollow sections
- before DE: Klassifizierung und wirksamer Querschnitt der Klasse 4 für gewalzte und geschweißte I-Querschnitte und Hohlprofile
- after EN: Section class as the highest class of any compressed part, determined per load combination (5.5.2(7))
- after DE: Querschnittsklasse als höchste Klasse eines gedrückten Teils, je Lastkombination bestimmt (5.5.2(7))

### en1993 V-040
- before EN: classification and effective class 4 section of rolled and welded I and hollow sections; effective width kernel effective_width
- before DE: Klassifizierung und wirksamer Querschnitt der Klasse 4 für gewalzte und geschweißte I-Querschnitte und Hohlprofile; Kernel der mittragenden Breite effective_width
- after EN: Effective class 4 section with the plate reduction factor ρ (ρ = 1.0 for λ_p ≤ 0.673) from part geometry and stresses (EN 1993-1-5 4.3, 1-1 6.2.2.5), via the effective-width kernel
- after DE: Wirksamer Querschnitt der Klasse 4 mit dem Plattenabminderungsbeiwert ρ (ρ = 1,0 für λ_p ≤ 0,673) aus Teilgeometrie und Spannungen (EN 1993-1-5 4.3, 1-1 6.2.2.5) über den Kern für wirksame Breiten

### en1993 V-062
- before EN: shear-reduced and class-dependent modulus
- before DE: querkraftabgeminderter und klassenabhängiger Modul
- after EN: Bending resistance Mc,Rd = Wpl fy/γM0 (class 1, 2), Wel,min fy/γM0 (class 3) and Weff,min fy/γM0 (class 4) with shear-reduced and class-dependent modulus (6.2.5)
- after DE: Biegetragfähigkeit Mc,Rd = Wpl fy/γM0 (Klasse 1, 2), Wel,min fy/γM0 (Klasse 3) und Weff,min fy/γM0 (Klasse 4) mit querkraftabgemindertem und klassenabhängigem Widerstandsmoment (6.2.5)

### en1993 V-066
- before EN: member under a point or uniform torque with fork, both-ends-restrained, restrained-fork or cantilever warping supports (member field warpingSupport) by the exact solution of E I_w theta'''' - G I_t theta'' = m (mechanics, not clause text: 6.2.7 prescribes only the stresses, not the end conditions); sigma_w = B/W_w with W_w = I_w/omega_max of the doubly symmetric I section, elastic sum with N, M_y and M_z; clauses 6.2.7(5)-(9) not verified against the print
- before DE: Stab unter Einzel- oder Streckentorsionsmoment mit Gabel-, beidseitig eingespannten, eingespannt-gabelgelagerten oder Kragarm-Wölblagerungen (Stabfeld warpingSupport) nach der exakten Lösung von E I_w theta'''' - G I_t theta'' = m (Mechanik, kein Normtext: 6.2.7 regelt nur die Spannungen, nicht die Randbedingungen); sigma_w = B/W_w mit W_w = I_w/omega_max des doppeltsymmetrischen I-Querschnitts, elastische Summe mit N, M_y und M_z; Absätze 6.2.7(5)-(9) nicht am Druck geprüft
- after EN: Warping torsion of a member under a point or uniform torque with fork, fixed, restrained-fork or cantilever warping supports (field warpingSupport) by the exact solution of E I_w θ'''' − G I_t θ'' = m, with σ_w = B/W_w summed elastically with N, M_y and M_z (6.2.7(1)); clauses 6.2.7(5)-(9) are not verified against the print
- after DE: Wölbkrafttorsion eines Stabs unter Einzel- oder Streckentorsionsmoment mit Gabel-, beidseitig eingespannten, eingespannt-gabelgelagerten oder Kragarm-Wölblagerungen (Feld warpingSupport) nach der exakten Lösung von E I_w θ'''' − G I_t θ'' = m, mit σ_w = B/W_w, elastisch überlagert mit N, M_y und M_z (6.2.7(1)); die Abschnitte 6.2.7(5)-(9) sind nicht gegen den Druck geprüft

### en1993 V-067
- before EN: shear-reduced and class-dependent modulus
- before DE: querkraftabgeminderter und klassenabhängiger Modul
- after EN: Bending and shear: no reduction for VEd ≤ 0.5 Vpl,Rd, otherwise reduced yield strength fy,red = (1 − ρ) fy on the shear area with ρ = (2 VEd/Vpl,Rd − 1)² (6.2.8)
- after DE: Biegung und Querkraft: keine Abminderung für VEd ≤ 0,5 Vpl,Rd, sonst abgeminderte Streckgrenze fy,red = (1 − ρ) fy in der Schubfläche mit ρ = (2 VEd/Vpl,Rd − 1)² (6.2.8)

### en1993 V-070
- before EN: Plate slenderness and effective width reduction rho of class 4 parts per 6.2.9.3, realised as the member check en1993.6.2.9.mn
- before DE: Plattenschlankheit und Abminderung rho der mittragenden Breite von Teilen der Klasse 4 nach 6.2.9.3, umgesetzt als Stabnachweis en1993.6.2.9.mn
- after EN: Class 3 stress check σx,Ed ≤ fy/γM0 and class 4 bending with axial force on the effective section (6.2.9.3, eq. 6.44), realised as the member check en1993.6.2.9.mn
- after DE: Spannungsnachweis σx,Ed ≤ fy/γM0 für Klasse 3 und Biegung mit Normalkraft am wirksamen Querschnitt für Klasse 4 (6.2.9.3, Gl. 6.44), umgesetzt als Stabnachweis en1993.6.2.9.mn

### en1993 V-073
- before EN: reduced stress criterion of plated panels
- before DE: Vergleichsspannungskriterium plattenförmiger Felder
- after EN: Elastic von Mises criterion (σx/fyd)² + (σz/fyd)² − (σx/fyd)(σz/fyd) + 3(τ/fyd)² ≤ 1 for combined stresses (6.2.2, 6.1(5)), realised as the reduced stress criterion of plated panels
- after DE: Elastisches Vergleichsspannungskriterium nach von Mises (σx/fyd)² + (σz/fyd)² − (σx/fyd)(σz/fyd) + 3(τ/fyd)² ≤ 1 für kombinierte Spannungen (6.2.2, 6.1(5)), umgesetzt als Vergleichsspannungskriterium für Beulfelder

### en1993 V-091
- before EN: simply supported beam under uniform load, mass from permanent actions
- before DE: einfacher Balken unter Gleichlast, Masse aus ständigen Einwirkungen
- after EN: Floor natural frequency f1 = π/(2L²) √(EI/m) ≥ 3 Hz (7.2.3) for a simply supported beam under uniform load with mass from permanent actions
- after DE: Eigenfrequenz der Decke f1 = π/(2L²) √(EI/m) ≥ 3 Hz (7.2.3) für einen einfeldrigen Balken unter Gleichlast mit Masse aus ständigen Einwirkungen

### en1993 V-110
- before EN: closed-form M_cr with C1 and C2
- before DE: geschlossene Form von M_cr mit C1 und C2
- after EN: Lateral-torsional buckling by the general case λ_LT = √(Wy fy/Mcr) with Φ_LT and χ_LT (6.3.2.1), Mcr by the closed form with C1 and C2
- after DE: Biegedrillknicken nach dem allgemeinen Fall λ_LT = √(Wy fy/Mcr) mit Φ_LT und χ_LT (6.3.2.1), Mcr in geschlossener Form mit C1 und C2

### en1993 V-113
- before EN: Hermite finite element M_cr with springs and load height
- before DE: M_cr mit Hermite-Finite-Elementen mit Federn und Lastangriffshöhe
- after EN: Elastic critical moment Mcr by the Hermite finite element chain with springs and load height (6.3.2.2)
- after DE: Ideales Biegedrillknickmoment Mcr mit der Hermite-Finite-Elemente-Kette mit Federn und Lastangriffshöhe (6.3.2.2)

### en1993 V-113a
- before EN: closed-form M_cr with C1 and C2
- before DE: geschlossene Form von M_cr mit C1 und C2
- after EN: Elastic critical moment Mcr of a doubly symmetric I section by the closed form with C1, C2 and C3 (ECCS formula)
- after DE: Ideales Biegedrillknickmoment Mcr eines doppeltsymmetrischen I-Querschnitts in geschlossener Form mit C1, C2 und C3 (ECCS-Formel)

### en1993 V-113b
- before EN: Hermite finite element M_cr with springs and load height
- before DE: M_cr mit Hermite-Finite-Elementen mit Federn und Lastangriffshöhe
- after EN: Mcr for arbitrary loading and supports from the linear stability eigenproblem of a 7-DOF thin-walled beam chain with warping, solved by Hermite finite elements with springs and load height
- after DE: Mcr für beliebige Belastung und Lagerung aus dem linearen Stabilitäts-Eigenwertproblem einer dünnwandigen 7-Freiheitsgrade-Stabkette mit Verwölbung, gelöst mit Hermite-Finiten-Elementen mit Federn und Lastangriffshöhe

### en1993 V-113c
- before EN: Hermite finite element M_cr with springs and load height
- before DE: M_cr mit Hermite-Finite-Elementen mit Federn und Lastangriffshöhe
- after EN: Mcr with lateral restraint by trapezoidal sheeting through the rotational spring stiffness CD (NA-DE), solved by Hermite finite elements with springs and load height
- after DE: Mcr mit seitlicher Stützung durch Trapezprofile über die Drehfedersteifigkeit CD (NA-DE), gelöst mit Hermite-Finiten-Elementen mit Federn und Lastangriffshöhe

### en1993 V-126
- before EN: partial not-modelled: buckling lengths are direct member inputs: the nomogram factor k (clauses::stability::k_nomogram) needs the stiffness ratios G_A and G_B that the snapshot does not carry
- before DE: teilweise nicht modelliert: Knicklängen sind direkte Stabeingaben: der Nomogrammfaktor k (clauses::stability::k_nomogram) braucht die Steifigkeitsverhältnisse G_A und G_B, die der Snapshot nicht führt
- after EN: Slenderness and buckling length Lcr = k L (6.3.1.3); buckling lengths are direct member inputs, the nomogram factor k from the end stiffness ratios G_A and G_B is not modelled as the snapshot lacks them
- after DE: Schlankheit und Knicklänge Lcr = k L (6.3.1.3); Knicklängen sind direkte Bauteileingaben, der Nomogrammfaktor k aus den Steifigkeitsverhältnissen G_A und G_B der Enden wird nicht modelliert, da der Schnappschuss sie nicht führt

### en1993 V-127
- before EN: shear-reduced and class-dependent modulus
- before DE: querkraftabgeminderter und klassenabhängiger Modul
- after EN: Section modulus selection for lateral-torsional buckling: Wpl,y, Wel,y or Weff,y by section class (6.3.2)
- after DE: Auswahl des Widerstandsmoments für das Biegedrillknicken: Wpl,y, Wel,y oder Weff,y je Querschnittsklasse (6.3.2)

### en1993 V-250
- before EN: partial not-modelled: the cold-formed member carries no f_u, no bend count n and no gross area A_g for f_ya = f_yb + (f_u - f_yb) k n t^2/A_g (clauses::cold_stainless::f_ya exists unwired)
- before DE: teilweise nicht modelliert: das kaltgeformte Bauteil führt kein f_u, keine Biegeanzahl n und keine Bruttofläche A_g für f_ya = f_yb + (f_u - f_yb) k n t^2/A_g (clauses::cold_stainless::f_ya existiert ungebunden)
- after EN: Enhanced yield strength of cold-formed material f_ya = f_yb + (f_u − f_yb) k n t²/A_g ≤ (f_u + f_yb)/2 (3.2.2); partly not modelled as the member carries no f_u, bend count n or gross area A_g
- after DE: Erhöhte Streckgrenze kaltgeformten Materials f_ya = f_yb + (f_u − f_yb) k n t²/A_g ≤ (f_u + f_yb)/2 (3.2.2); teilweise nicht modelliert, da das Bauteil kein f_u, keine Biegeanzahl n und keine Bruttofläche A_g führt

### en1993 V-360
- before EN: Effective widths of plate elements by the reduction factor rho per 3.2, realised as the plated panel check en1993.1-5.3.sl
- before DE: Mittragende Breiten von Plattenelementen mit Abminderungsfaktor rho nach 3.2, umgesetzt als Plattenfeldnachweis en1993.1-5.3.sl
- after EN: Shear lag in flanges with effective width beff = β b0 and β from κ = α0 b0/Le (3.2), realised as the plated panel check en1993.1-5.3.sl
- after DE: Schubverzerrung in Gurten mit mitwirkender Breite beff = β b0 und β aus κ = α0 b0/Le (3.2), umgesetzt als Beulfeldnachweis en1993.1-5.3.sl

### en1993 V-366
- before EN: k_sigma definitions in plated_ext
- before DE: Definitionen von k_sigma in plated_ext
- after EN: Buckling coefficients kσ of Tables 4.1 and 4.2 for internal and outstand elements, defined in plated_ext
- after DE: Beulwerte kσ nach den Tabellen 4.1 und 4.2 für innenliegende und überstehende Elemente, definiert in plated_ext

### en1993 V-374
- before EN: Resistance to local transverse forces on webs, realised as the plated panel check en1993.1-5.6
- before DE: Tragfähigkeit gegen örtliche Querkräfte auf Stege, umgesetzt als Plattenfeldnachweis en1993.1-5.6
- after EN: Web crippling of I-girders under crane wheels with load spread ss at 45° through rail and flange (V-960), realised as the plated panel check en1993.1-5.6
- after DE: Stegeindrücken von I-Trägern unter Kranradlasten mit Lastverteilung ss unter 45° durch Schiene und Flansch (V-960), umgesetzt als Beulfeldnachweis en1993.1-5.6

### en1993 V-405
- before EN: block tearing; the angle net section (beta_2, beta_3) is not wired
- before DE: Blockversagen; der Nettoquerschnitt von Winkeln (beta_2, beta_3) ist nicht angebunden
- after EN: Block tearing resistance Veff,1,Rd = fu Ant/γM2 + fy Anv/(√3 γM0) (3.10); the angle net section with β2, β3 is not wired
- after DE: Blockversagen Veff,1,Rd = fu Ant/γM2 + fy Anv/(√3 γM0) (3.10); der Nettoquerschnitt von Winkeln mit β2, β3 ist nicht angebunden

### en1993 V-407
- before EN: partial not-modelled: eccentric bolt-group distribution (clauses::joints::bolt_group_force) needs bolt coordinates and an eccentricity that the joint row does not carry; the bolt grid takes a concentric shear
- before DE: teilweise nicht modelliert: die exzentrische Schraubengruppenverteilung (clauses::joints::bolt_group_force) braucht Schraubenkoordinaten und eine Exzentrizität, die die Anschlusszeile nicht führt; das Schraubenraster nimmt eine zentrische Querkraft an
- after EN: Distribution of load over a group of fasteners (3.12); the eccentric bolt group needs bolt coordinates and an eccentricity that the joint row does not carry, so the bolt grid takes a concentric shear
- after DE: Lastverteilung auf eine Gruppe von Verbindungsmitteln (3.12); die exzentrische Schraubengruppe benötigt Schraubenkoordinaten und eine Ausmitte, die die Verbindungszeile nicht führt, das Schraubenraster nimmt daher eine zentrische Querkraft

### en1993 V-410
- before EN: butt welds take the member resistance; partial penetration welds as deep fillets
- before DE: Stumpfnähte übernehmen die Bauteiltragfähigkeit; nicht durchgeschweißte Nähte werden wie tiefe Kehlnähte behandelt
- after EN: Fillet weld by the directional method with throat a, effective length ≥ max(6a, 30 mm) and stress components σ_⊥, τ_⊥, τ_∥ on the throat plane (4.5)
- after DE: Kehlnaht nach dem richtungsbezogenen Verfahren mit Nahtdicke a, wirksamer Länge ≥ max(6a, 30 mm) und Spannungskomponenten σ_⊥, τ_⊥, τ_∥ in der Nahtebene (4.5)

### en1993 V-450
- before EN: K and N gap joints of RHS (Tables 7.8 and 7.11, unverified rows)
- before DE: K- und N-Knoten mit Spalt von Rechteckhohlprofilen (Tabellen 7.8 und 7.11, ungeprüfte Zeilen)
- after EN: Tubular joints: chord face failure of K and N gap joints of RHS (Tables 7.8 and 7.11, unverified rows)
- after DE: Rohrknoten: Gurtplattenversagen von K- und N-Knoten mit Spalt aus RHS (Tabellen 7.8 und 7.11, ungeprüfte Zeilen)

### en1993 V-451
- before EN: fin plate as bolted plate: bolt group, bearing, block tearing; plate bending and the column weld are not separate checks; fin plate and lug joints have no entity yet
- before DE: Laschenanschluss als geschraubtes Blech: Schraubengruppe, Lochleibung, Blockversagen; Blechbiegung und Stützenschweißnaht sind keine eigenen Nachweise; Laschen- und Ösenanschlüsse haben noch keine Entität
- after EN: Fin plate (shear tab) as a bolted plate with bolt group, bearing and block tearing; plate bending and the column weld are not separate checks, and fin plate and lug joints have no entity yet
- after DE: Anschlussblech (Schubblech) als geschraubtes Blech mit Schraubengruppe, Lochleibung und Blockversagen; Blechbiegung und Stützenschweißnaht sind keine eigenen Nachweise, und für Anschlussblech- und Laschenanschlüsse gibt es noch keine Entität

### en1993 V-505
- before EN: detail ladder and orthotropic detail table in bridge_fatigue (unverified values)
- before DE: Kerbfallleiter und Tabelle orthotroper Details in bridge_fatigue (ungeprüfte Werte)
- after EN: Fatigue detail catalogue of Tables 8.1 to 8.10 with category and geometry constraints, modelled only as the detail ladder and the orthotropic detail table in bridge_fatigue (unverified values)
- after DE: Kerbfallkatalog der Tabellen 8.1 bis 8.10 mit Kategorie und Geometriebedingungen, nur als Kerbfallleiter und Tabelle orthotroper Kerbfälle in bridge_fatigue modelliert (ungeprüfte Werte)

### en1993 V-506
- before EN: fracture mechanics assessment not implemented
- before DE: Bruchmechanik-Nachweis nicht umgesetzt
- after EN: Fracture mechanics assessment of damage tolerant design with Paris crack growth da/dN = C (ΔK)^m (Annexes C and D) is not implemented
- after DE: Bruchmechanische Bewertung der schadenstoleranten Bemessung mit Paris-Rissfortschritt da/dN = C (ΔK)^m (Anhänge C und D) ist nicht umgesetzt

### en1993 V-600
- before EN: also data: clauses::fatigue_fire::k_y_theta, k_p_theta and k_e_theta (registry ndp.en1993.1-2.t31.*)
- before DE: auch Daten: clauses::fatigue_fire::k_y_theta, k_p_theta und k_e_theta (Register ndp.en1993.1-2.t31.*)
- after EN: Reduction factors ky,θ, kp,θ and kE,θ of carbon steel at elevated temperature (Table 3.1), also data in clauses::fatigue_fire (registry ndp.en1993.1-2.t31.*)
- after DE: Abminderungsbeiwerte ky,θ, kp,θ und kE,θ von Kohlenstoffstahl bei erhöhter Temperatur (Tabelle 3.1), auch als Daten in clauses::fatigue_fire (Register ndp.en1993.1-2.t31.*)

### en1993 V-601
- before EN: thermal properties inside fatigue_fire
- before DE: thermische Eigenschaften innerhalb von fatigue_fire
- after EN: Thermal properties of steel at elevated temperature, specific heat ca(θ) and conductivity λa = 54 − 3.33e-2 θ (3.4), data inside fatigue_fire
- after DE: Thermische Eigenschaften von Stahl bei erhöhter Temperatur, spezifische Wärme ca(θ) und Wärmeleitfähigkeit λa = 54 − 3,33e-2 θ (3.4), Daten in fatigue_fire

### en1993 V-602
- before EN: the section factor A_m/V is an input of the exposure
- before DE: Der Profilfaktor A_m/V ist eine Eingabe der Beanspruchung durch Brand
- after EN: Temperature rise of unprotected steel Δθa,t = ksh (Am/V)/(ca ρa) h_net,d Δt (4.2.5), with the section factor Am/V as an input of the exposure
- after DE: Erwärmung ungeschützten Stahls Δθa,t = ksh (Am/V)/(ca ρa) h_net,d Δt (4.2.5), wobei der Profilfaktor Am/V eine Eingabe der Exposition ist

### en1993 V-603
- before EN: the section factor A_m/V is an input of the exposure
- before DE: Der Profilfaktor A_m/V ist eine Eingabe der Beanspruchung durch Brand
- after EN: Temperature rise of protected steel with the insulation parameter φ = (cp ρp/(ca ρa)) dp (Ap/V) (4.2.5.2), with the section factor as an input of the exposure
- after DE: Erwärmung geschützten Stahls mit dem Dämmparameter φ = (cp ρp/(ca ρa)) dp (Ap/V) (4.2.5.2), wobei der Profilfaktor eine Eingabe der Exposition ist

### en1993 V-610
- before EN: the section factor A_m/V is an input of the exposure
- before DE: Der Profilfaktor A_m/V ist eine Eingabe der Beanspruchung durch Brand
- after EN: Section factor Am/V and box value per section and protection exposure (4.2.4), an input of the exposure
- after DE: Profilfaktor Am/V und Kastenwert je Querschnitt und Schutzexposition (4.2.4), eine Eingabe der Exposition

### en1993 V-802
- before EN: external pressure and ring stiffeners wired in v4
- before DE: Außendruck und Ringsteifen in v4 angebunden
- after EN: Elastic critical buckling stress of cylinders, axial σx,Rcr = 0.605 E Cx t/r with Cx by shell length (EN 1993-1-6 D.1); external pressure and ring stiffeners wired in v4
- after DE: Ideale Beulspannung von Zylindern, axial σx,Rcr = 0,605 E Cx t/r mit Cx nach Schalenlänge (EN 1993-1-6 D.1); Außendruck und Ringsteifen in v4 angebunden

### en1993 V-805
- before EN: sigma = k p b^2/t^2 against f_y/gamma_M0 with the entered coefficient k; the Timoshenko or Roark coefficient tables are not encoded (unverified)
- before DE: sigma = k p b^2/t^2 gegen f_y/gamma_M0 mit dem eingegebenen Beiwert k; die Koeffiziententafeln nach Timoshenko oder Roark sind nicht abgebildet (ungeprüft)
- after EN: Plate bending under transverse load σ = k p b²/t² against fy/γM0 with the entered coefficient k; the Timoshenko or Roark coefficient tables are not encoded (unverified)
- after DE: Plattenbiegung unter Querlast σ = k p b²/t² gegen fy/γM0 mit dem eingegebenen Beiwert k; die Beiwerttabellen nach Timoshenko oder Roark sind nicht kodiert (ungeprüft)

### en1993 V-880
- before EN: deck minima and wheel strip fatigue wired in v4
- before DE: Fahrbahnplattenminima und Radstreifen-Ermüdung in v4 angebunden
- after EN: Steel bridges per EN 1993-2 with γM0 = 1.0, γM1 = 1.1, γM2 = 1.25 and the plate girder rules of 1-5; deck minima and wheel strip fatigue wired in v4
- after DE: Stahlbrücken nach EN 1993-2 mit γM0 = 1,0, γM1 = 1,1, γM2 = 1,25 und den Plattenträgerregeln aus 1-5; Fahrbahnplatten-Mindestwerte und Radstreifen-Ermüdung in v4 angebunden

### en1994 C-01
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Material set of concrete (fck, fcm, fctm, Ecm per EN 1992-1-1 Table 3.1) and structural steel S235 to S460 (fy, fu by thickness) (3.1, 3.2, 3.3, 3.5)
- after DE: Werkstoffsatz aus Beton (fck, fcm, fctm, Ecm nach EN 1992-1-1 Tabelle 3.1) und Baustahl S235 bis S460 (fy, fu nach Dicke) (3.1, 3.2, 3.3, 3.5)

### en1994 C-02
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Classification of the steel elements of composite members with ε = √(235/fy) and the c/t limits per class, considering encasement (5.5, Table 5.2)
- after DE: Klassifizierung der Stahlteile von Verbundbauteilen mit ε = √(235/fy) und den c/t-Grenzen je Klasse unter Berücksichtigung der Einbetonierung (5.5, Tabelle 5.2)

### en1994 C-03
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Effective width of the concrete flange beff = b0 + Σ be,i from the zero-moment distance Le (5.4.1.2, Figure 5.1)
- after DE: Mittragende Breite des Betongurts beff = b0 + Σ be,i aus dem Momentennullpunktabstand Le (5.4.1.2, Bild 5.1)

### en1994 C-04
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Selection of the global analysis method: linear-elastic with cracked regions of 15 % of the span each side of internal supports, non-linear or rigid-plastic (5.4.2)
- after DE: Wahl des Berechnungsverfahrens: linear-elastisch mit gerissenen Bereichen von 15 % der Stützweite beidseits innerer Stützen, nichtlinear oder starr-plastisch (5.4.2)

### en1994 C-10
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Non-linear bending resistance from parabola-rectangle concrete, bilinear steel and bilinear reinforcement with limited strains (6.2.1.4(6))
- after DE: Nichtlineare Biegetragfähigkeit aus Parabel-Rechteck-Beton, bilinearem Stahl und bilinearer Bewehrung mit begrenzten Dehnungen (6.2.1.4(6))

### en1994 C-14
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Minimum degree of shear connection η from the span Le and fy for steel sections with equal flanges (6.6.1.2)
- after DE: Mindestverdübelungsgrad η aus Stützweite Le und fy für Stahlquerschnitte mit gleichen Flanschen (6.6.1.2)

### en1994 C-16
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Stud shear resistance PRd = min(0.8 fu π d²/4 /γV; 0.29 α d² √(fck Ecm)/γV) (6.6.3.1)
- after DE: Tragfähigkeit von Kopfbolzendübeln PRd = min(0,8 fu π d²/4 /γV; 0,29 α d² √(fck Ecm)/γV) (6.6.3.1)

### en1994 C-19
- before EN: slab longitudinal shear check
- before DE: Nachweis des Längsschubs im Gurt
- after EN: Longitudinal shear in the concrete slab vEd = (ΔFd/Δx)/Σ(length of shear surface) against the transverse reinforcement per shear surface (6.6.6, EN 1992-1-1 6.2.4)
- after DE: Längsschub im Betongurt vEd = (ΔFd/Δx)/Σ(Länge der Schubfläche) gegen die Querbewehrung je Schubfläche (6.6.6, EN 1992-1-1 6.2.4)

### en1994 C-21
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Fatigue of shear studs γFf Δτ,E,2 ≤ ΔτC/γMf,s with ΔτC = 90 MPa at 2e6 cycles (6.8.3-6.8.7)
- after DE: Ermüdung von Kopfbolzendübeln γFf Δτ,E,2 ≤ ΔτC/γMf,s mit ΔτC = 90 MPa bei 2e6 Lastwechseln (6.8.3-6.8.7)

### en1994 C-22
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Vertical shear resistance Vpl,a,Rd = Av fy/(√3 γM0) of the steel section with the shear buckling check for slender webs (6.2.2)
- after DE: Querkrafttragfähigkeit Vpl,a,Rd = Av fy/(√3 γM0) des Stahlquerschnitts mit dem Schubbeulnachweis für schlanke Stege (6.2.2)

### en1994 C-24
- before EN: patch loading with composite effect; mapped by reading the check ids the artifact emits
- before DE: Einzellast auf den Steg mit Verbundwirkung; durch Lesen der vom Artefakt ausgegebenen Nachweis-IDs zugeordnet
- after EN: Web resistance Fn,Rd under concentrated loads and at stiffeners with composite effect per EN 1993-1-5 chapter 6 (6.2)
- after DE: Stegtragfähigkeit Fn,Rd unter Einzellasten und an Steifen mit Verbundwirkung nach EN 1993-1-5 Kapitel 6 (6.2)

### en1994 C-25
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Scope of the simplified method for composite columns: doubly symmetric uniform section, steel contribution δ between 0.2 and 0.9, λ̄ ≤ 2.0 and reinforcement ratio 0.3 % to 6 % (6.7.1)
- after DE: Anwendungsgrenzen des vereinfachten Verfahrens für Verbundstützen: doppeltsymmetrischer gleichbleibender Querschnitt, Stahlanteil δ zwischen 0,2 und 0,9, λ̄ ≤ 2,0 und Bewehrungsgrad 0,3 % bis 6 % (6.7.1)

### en1994 C-31
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Introduction of loads into composite columns with the design shear strength τRd (Table 6.6) over the introduction length (6.7.4)
- after DE: Krafteinleitung in Verbundstützen mit der Bemessungsschubfestigkeit τRd (Tabelle 6.6) über die Krafteinleitungslänge (6.7.4)

### en1994 C-33
- before EN: mapped by reading the check ids the artifact emits
- before DE: durch Lesen der vom Artefakt ausgegebenen Nachweis-IDs zugeordnet
- after EN: Profiled sheeting as formwork: ULS and SLS under wet concrete and construction load with the deflection limit L/180 (9.3.1)
- after DE: Profilblech als Schalung: Grenzzustände der Tragfähigkeit und Gebrauchstauglichkeit unter Frischbeton und Montagelast mit Durchbiegungsgrenze L/180 (9.3.1)

### en1994 C-36
- before EN: partial connection method shares the sagging resistance check of C-34; mapped by reading the check ids the artifact emits
- before DE: das Teilverbundverfahren nutzt den Nachweis des positiven Moments von C-34; durch Lesen der vom Artefakt ausgegebenen Nachweis-IDs zugeordnet
- after EN: Partial connection method with τu,Rk from tests (Annex B.3.5), sharing the sagging resistance check of C-34
- after DE: Teilverbundverfahren mit τu,Rk aus Versuchen (Anhang B.3.5), das den Nachweis des positiven Moments von C-34 mitnutzt

### en1994 C-37
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Vertical shear of composite slabs Vv,Rd = bo dp τRd kv (1.2 + 40ρ) (9.7.5)
- after DE: Querkraft von Verbunddecken Vv,Rd = bo dp τRd kv (1,2 + 40ρ) (9.7.5)

### en1994 C-38
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Punching shear of composite slabs against the critical perimeter Ccrit per EN 1992-1-1 6.4.4 (9.7.6)
- after DE: Durchstanzen von Verbunddecken gegen den kritischen Rundschnitt Ccrit nach EN 1992-1-1 6.4.4 (9.7.6)

### en1994 C-39
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Serviceability of composite slabs: deflection, crack width over supports and minimum reinforcement of continuous slabs (9.8)
- after DE: Gebrauchstauglichkeit von Verbunddecken: Durchbiegung, Rissbreite über den Stützen und Mindestbewehrung durchlaufender Decken (9.8)

### en1994 C-41
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Stress limitation σc ≤ 0.6 fck and σs ≤ 0.8 fsk at SLS, summed stage-wise (7.2.2)
- after DE: Spannungsbegrenzung σc ≤ 0,6 fck und σs ≤ 0,8 fsk im Grenzzustand der Gebrauchstauglichkeit, bauzustandsweise summiert (7.2.2)

### en1994 C-46
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Tabulated fire data: minimum dimensions of composite beams and columns by fire resistance class R30 to R180 and load level ηfi (F:4.2)
- after DE: Tabellierte Brandschutzdaten: Mindestabmessungen von Verbundträgern und -stützen nach Feuerwiderstandsklasse R30 bis R180 und Lastniveau ηfi (F:4.2)

### en1994 C-47
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Simple fire calculation models: steel temperature θa of beams and k-reduced moment capacities Mfi,Rd,θ (F:4.3)
- after DE: Vereinfachte Brandberechnungsmodelle: Stahltemperatur θa von Trägern und mit k abgeminderte Momententragfähigkeiten Mfi,Rd,θ (F:4.3)

### en1994 C-48
- before EN: mapped by reading the check ids the artifact emits
- before DE: durch Lesen der vom Artefakt ausgegebenen Nachweis-IDs zugeordnet
- after EN: Bridge actions and combinations from the EN 1991-2 load models with shrinkage, temperature and construction stages (2:2)
- after DE: Brückeneinwirkungen und Kombinationen aus den Lastmodellen nach EN 1991-2 mit Schwinden, Temperatur und Bauzuständen (2:2)

### en1994 C-49
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Global analysis of composite bridges with shear lag effective width and cracked regions of 15 % each side of supports (2:5.4)
- after DE: Globale Berechnung von Verbundbrücken mit mittragender Breite infolge Schubverzerrung und gerissenen Bereichen von 15 % beidseits der Stützen (2:5.4)

### en1994 C-52
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Fatigue of composite bridges: equivalent stress range Δσ,E2 = λ Φ2 Δσ with λ = λ1 λ2 λ3 λ4, reinforcement ΔσRsk and shear studs with λv (2:6.8)
- after DE: Ermüdung von Verbundbrücken: äquivalente Spannungsschwingbreite Δσ,E2 = λ Φ2 Δσ mit λ = λ1 λ2 λ3 λ4, Bewehrung ΔσRsk und Kopfbolzendübel mit λv (2:6.8)

### en1994 C-54
- before EN: confirmed by reading the check source: the family realises this id
- before DE: durch Lesen des Nachweisquelltexts bestätigt: die Familie realisiert diese ID
- after EN: Bridge-specific stud detailing: transverse spacing, edge distance and end plates (2:6.6.5)
- after DE: Brückenspezifische Dübelausbildung: Querabstand, Randabstand und Endplatten (2:6.6.5)

### en1996 M-01
- before EN: strength chain input: unit height, width and declared f_b of every wall give the shape factor delta and the normalised f_b; the stand-alone calculator stays a catalogue entry
- before DE: Eingang der Festigkeitskette: Steinhöhe, -breite und angegebenes f_b jeder Wand ergeben den Formbeiwert delta und das normierte f_b; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Normalised unit strength f_b = δ f_b,decl from unit height and width with the shape factor δ, per wall (3.1, EN 772-1)
- after DE: Normierte Steindruckfestigkeit f_b = δ f_b,decl aus Steinhöhe und -breite mit dem Formbeiwert δ je Wand (3.1, EN 772-1)

### en1996 M-02
- before EN: characteristic and design compressive strength f_k, f_d of every wall from units, mortar and the annex partial factor; the stand-alone calculator stays a catalogue entry
- before DE: charakteristische und Bemessungswert der Druckfestigkeit f_k, f_d jeder Wand aus Steinen, Mörtel und dem Teilsicherheitsbeiwert des Anhangs; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Characteristic compressive strength f_k = K f_b^α f_m^β and design value f_d = f_k/γ_M per wall from units, mortar and the annex partial factor (3.6.1.2)
- after DE: Charakteristische Druckfestigkeit f_k = K f_b^α f_m^β und Bemessungswert f_d = f_k/γ_M je Wand aus Steinen, Mörtel und dem Teilsicherheitsbeiwert des Anhangs (3.6.1.2)

### en1996 M-03
- before EN: characteristic shear strength f_vk of every wall at its minimum design compression; the stand-alone calculator stays a catalogue entry
- before DE: charakteristische Schubfestigkeit f_vk jeder Wand bei ihrer minimalen Bemessungsdruckspannung; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Characteristic shear strength f_vk = f_vk0 + 0.4 σ_d per wall at its minimum design compression (3.6.2)
- after DE: Charakteristische Schubfestigkeit f_vk = f_vk0 + 0,4 σ_d je Wand bei ihrer minimalen Bemessungsdruckspannung (3.6.2)

### en1996 M-04
- before EN: characteristic flexural strengths f_xk1, f_xk2 of every wall from its mortar and unit material; the stand-alone calculator stays a catalogue entry
- before DE: charakteristische Biegezugfestigkeiten f_xk1, f_xk2 jeder Wand aus Mörtel und Steinmaterial; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Characteristic flexural strengths f_xk1 and f_xk2 per wall from its mortar and unit material (3.6.3)
- after DE: Charakteristische Biegezugfestigkeiten f_xk1 und f_xk2 je Wand aus Mörtel und Steinmaterial (3.6.3)

### en1996 M-05
- before EN: anchorage bars (kind Anchorage) of the walls: required bond length l_b = phi f_yd/(4 f_bd) against the provided bar length; the initial shear strength of this clause is the check of M-03 (f_bok is unverified, ndp.en1996.m05.fbok)
- before DE: Verankerungsstäbe (Art Anchorage) der Wände: erforderliche Verbundlänge l_b = phi f_yd/(4 f_bd) gegen die vorhandene Stablänge; die Anfangsscherfestigkeit dieses Abschnitts ist die Prüfung von M-03 (f_bok ungeprüft, ndp.en1996.m05.fbok)
- after EN: Anchorage of reinforcement bars (kind Anchorage) with the required bond length l_b = φ f_yd/(4 f_bd) against the provided bar length; the initial shear strength of this clause is the check of M-03 (f_bok unverified, ndp.en1996.m05.fbok) (3.6.2, 3.6.4)
- after DE: Verankerung von Bewehrungsstäben (Art Anchorage) mit der erforderlichen Verbundlänge l_b = φ f_yd/(4 f_bd) gegen die vorhandene Stablänge; die Anfangsscherfestigkeit dieses Abschnitts ist die Prüfung von M-03 (f_bok ungeprüft, ndp.en1996.m05.fbok) (3.6.2, 3.6.4)

### en1996 M-06
- before EN: elastic modulus of every wall from f_k and the unit material; the stand-alone calculator stays a catalogue entry
- before DE: Elastizitätsmodul jeder Wand aus f_k und Steinmaterial; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Elastic modulus E = K_E f_k of every wall from f_k and the unit material (3.7)
- after DE: Elastizitätsmodul E = K_E f_k jeder Wand aus f_k und Steinmaterial (3.7)

### en1996 M-07
- before EN: partial factor gamma_M of the masonry class and the design situation of the document; the stand-alone calculator stays a catalogue entry
- before DE: Teilsicherheitsbeiwert gamma_M der Mauerwerksklasse und der Bemessungssituation des Dokuments; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Partial factor γ_M of the masonry class and the design situation of the document (Table 2.3)
- after DE: Teilsicherheitsbeiwert γ_M der Mauerwerksklasse und der Bemessungssituation des Dokuments (Tabelle 2.3)

### en1996 M-10
- before EN: effective height and restraint enter the slenderness check of every wall (supportSides, slabBearingDepthM, heights); the stand-alone calculator stays a catalogue entry
- before DE: Knicklänge und Halterung gehen in den Schlankheitsnachweis jeder Wand ein (supportSides, slabBearingDepthM, heights); der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Effective height h_ef = ρ_n h from the restraint at two, three or four edges (supportSides, slabBearingDepthM, heights) (5.5.1.2)
- after DE: Knicklänge h_ef = ρ_n h aus der Halterung an zwei, drei oder vier Rändern (supportSides, slabBearingDepthM, heights) (5.5.1.2)

### en1996 M-11
- before EN: slenderness h_ef/t of every wall; the stand-alone calculator stays a catalogue entry
- before DE: Schlankheit h_ef/t jeder Wand; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Slenderness h_ef/t_ef of every wall against the limit of 27 (5.5.1.4)
- after DE: Schlankheit h_ef/t_ef jeder Wand gegen die Grenze 27 (5.5.1.4)

### en1996 M-12
- before EN: declared load eccentricity at head and foot of every wall and load case; the stand-alone calculator stays a catalogue entry
- before DE: angegebene Lastausmitte am Wandkopf und Wandfuß jeder Wand und jedes Lastfalls; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Eccentricity e_i ≥ 0.05 t and reduction Φ_i = 1 − 2 e_i/t at head and foot of every wall and load case (6.1.2.2)
- after DE: Ausmitte e_i ≥ 0,05 t und Abminderung Φ_i = 1 − 2 e_i/t am Wandkopf und Wandfuß jeder Wand und jedes Lastfalls (6.1.2.2)

### en1996 M-13
- before EN: mid-height reduction factor with creep of every wall and load case; the stand-alone calculator stays a catalogue entry
- before DE: Abminderungsbeiwert in halber Höhe mit Kriechen jeder Wand und jedes Lastfalls; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Mid-height reduction factor Φ_m = A_1 exp(−u²/2) with creep eccentricity e_k, per wall and load case (6.1.2.2, Annex G)
- after DE: Abminderungsbeiwert in halber Höhe Φ_m = A_1 exp(−u²/2) mit Kriechausmitte e_k je Wand und Lastfall (6.1.2.2, Anhang G)

### en1996 M-14
- before EN: concentrated loads are typed rows of the load cases (ConcentratedLoad) with bearing check and area rule
- before DE: Einzellasten sind typisierte Zeilen der Lastfälle (ConcentratedLoad) mit Pressungsnachweis und Flächenregel
- after EN: Concentrated loads N_Edc ≤ β A_b f_d as typed rows of the load cases (ConcentratedLoad) with bearing check and area rule (6.1.3)
- after DE: Einzellasten N_Edc ≤ β A_b f_d als typisierte Zeilen der Lastfälle (ConcentratedLoad) mit Pressungsnachweis und Flächenregel (6.1.3)

### en1996 M-15
- before EN: openings, piers and lintels are typed rows of the wall: the lintel must span its opening and every member must lie inside the wall; unverified NDP basis for the loads of the stand-alone calculator
- before DE: Öffnungen, Pfeiler und Stürze sind typisierte Zeilen der Wand: der Sturz muss seine Öffnung überspannen und jedes Bauteil muss innerhalb der Wand liegen; ungeprüfte NDP-Grundlage für die Lasten des eigenständigen Rechners
- after EN: Openings, piers and lintels as typed rows of the wall: the lintel must span its opening and every member must lie inside the wall; unverified NDP basis for the loads (5.5, 6.1, Annex)
- after DE: Öffnungen, Pfeiler und Stürze als typisierte Zeilen der Wand: der Sturz muss seine Öffnung überspannen und jedes Bauteil muss innerhalb der Wand liegen; ungeprüfte NDP-Grundlage für die Lasten (5.5, 6.1, Anhang)

### en1996 M-16
- before EN: wind with vertical load of every wall and load case with wind pressure; the stand-alone calculator stays a catalogue entry
- before DE: Wind mit Vertikallast jeder Wand und jedes Lastfalls mit Winddruck; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Vertical resistance of every wall and load case with wind pressure, including the wind eccentricity e_he (6.1.2(2), Annex A)
- after DE: Vertikaltragfähigkeit jeder Wand und jedes Lastfalls mit Winddruck, einschließlich der Windausmitte e_he (6.1.2(2), Anhang A)

### en1996 M-20
- before EN: in-plane shear resistance of every wall with a declared inPlaneShearN, including the flange walls and the in-plane moment; the stand-alone calculator stays a catalogue entry
- before DE: Schubtragfähigkeit in Wandebene jeder Wand mit angegebenem inPlaneShearN, einschließlich der Flanschwände und des Moments in Wandebene; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: In-plane shear resistance V_Rd = f_vd t l_c of every wall with a declared inPlaneShearN, including flange walls and the in-plane moment (6.2)
- after DE: Schubtragfähigkeit in Wandebene V_Rd = f_vd t l_c jeder Wand mit angegebenem inPlaneShearN, einschließlich der Flanschwände und des Moments in Wandebene (6.2)

### en1996 M-21
- before EN: bracing layout of the shear walls with plan positions and a declared in-plane shear: centre of stiffness, torsion and governing wall under the rigid or flexible diaphragm rule of the document
- before DE: Aussteifungsanordnung der Schubwände mit Grundrisslage und deklarierter Scheibenschubkraft: Steifigkeitsmittelpunkt, Torsion und maßgebende Wand nach der starren oder biegeweichen Deckenscheibe des Dokuments
- after EN: Bracing layout of the shear walls with plan positions and declared in-plane shear: centre of stiffness, torsion and governing wall under the rigid or flexible diaphragm rule of the document (5.4, 8.5)
- after DE: Aussteifungsanordnung der Schubwände mit Grundrisslage und deklarierter Scheibenschubkraft: Steifigkeitsmittelpunkt, Torsion und maßgebende Wand nach der starren oder biegeweichen Deckenscheibe des Dokuments (5.4, 8.5)

### en1996 M-30
- before EN: lateral flexure of every wall and load case (wind, earth pressure); the stand-alone calculator stays a catalogue entry
- before DE: Biegung quer zur Wandebene jeder Wand und jedes Lastfalls (Wind, Erddruck); der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: One-way lateral flexure M_Ed = w_Ed h²/8 against M_Rd = (f_xd1 + σ_d) t²/6 per wall and load case (wind, earth pressure) (6.3, 6.3.1)
- after DE: Einachsige Biegung quer zur Wandebene M_Ed = w_Ed h²/8 gegen M_Rd = (f_xd1 + σ_d) t²/6 je Wand und Lastfall (Wind, Erddruck) (6.3, 6.3.1)

### en1996 M-31
- before EN: two-way bending by yield lines of every wall and load case with wind pressure, edges from supportSides, Annex E coefficient unverified
- before DE: zweiachsige Biegung nach Bruchlinientheorie jeder Wand und jedes Lastfalls mit Winddruck, Ränder aus supportSides, Beiwert aus Anhang E ungeprüft
- after EN: Two-way bending by yield lines M_Ed1, M_Ed2 against f_xd1 Z and f_xd2 Z per wall and load case with wind pressure, edges from supportSides, Annex E coefficient unverified (6.3.2, Annex E)
- after DE: Zweiachsige Biegung nach Bruchlinientheorie M_Ed1, M_Ed2 gegen f_xd1 Z und f_xd2 Z je Wand und Lastfall mit Winddruck, Ränder aus supportSides, Beiwert aus Anhang E ungeprüft (6.3.2, Anhang E)

### en1996 M-32
- before EN: lateral load with arching of every wall and load case with wind pressure over the storey height; the arching model is the alternative of M-30
- before DE: Horizontallast mit Gewölbewirkung jeder Wand und jedes Lastfalls mit Winddruck über die Geschosshöhe; das Gewölbemodell ist die Alternative zu M-30
- after EN: Lateral load capacity with arching over the storey height per wall and load case with wind pressure, the alternative to M-30 (6.3.3, NA-DE)
- after DE: Querlasttragfähigkeit mit Gewölbewirkung über die Geschosshöhe je Wand und Lastfall mit Winddruck, die Alternative zu M-30 (6.3.3, NA-DE)

### en1996 M-33
- before EN: wind pressure q_p c_pe on every wall and load case as the lateral action of M-30 to M-32
- before DE: Winddruck q_p c_pe auf jede Wand und jeden Lastfall als Horizontalwirkung von M-30 bis M-32
- after EN: Wind pressure q_p c_pe on every wall and load case as the lateral action of M-30 to M-32
- after DE: Winddruck q_p c_pe auf jede Wand und jeden Lastfall als Horizontalwirkung von M-30 bis M-32

### en1996 M-34
- before EN: basement walls are the walls with isBasement under earth pressure; the stand-alone calculator stays a catalogue entry
- before DE: Kellerwände sind die Wände mit isBasement unter Erddruck; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Basement walls as the walls with isBasement under earth pressure, with the simplified conditions on wall height, soil height and thickness (6.3, EN 1996-3 4, NA-DE)
- after DE: Kellerwände als die Wände mit isBasement unter Erddruck, mit den vereinfachten Bedingungen an Wandhöhe, Anschütthöhe und Dicke (6.3, EN 1996-3 4, NA-DE)

### en1996 M-40
- before EN: reinforced walls (reinforced, vertical steel and design yield strength declared): bending with axial force; the stand-alone calculator stays a catalogue entry
- before DE: bewehrte Wände (reinforced, vertikaler Stahl und Bemessungswert der Streckgrenze angegeben): Biegung mit Normalkraft; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Bending with axial force of reinforced walls (reinforced, vertical steel and design yield strength declared) with M_Rd = A_s f_yd z and x ≤ 0.4 d (6.6.1, 6.6.2)
- after DE: Biegung mit Normalkraft bewehrter Wände (reinforced, vertikaler Stahl und Bemessungswert der Streckgrenze angegeben) mit M_Rd = A_s f_yd z und x ≤ 0,4 d (6.6.1, 6.6.2)

### en1996 M-41
- before EN: shear reinforcement bars (kind Shear) of walls with a declared in-plane shear; the stand-alone calculator stays a catalogue entry
- before DE: Schubbewehrungsstäbe (kind Shear) von Wänden mit angegebener Schubbeanspruchung in Wandebene; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Shear resistance of walls with shear reinforcement bars (kind Shear) and a declared in-plane shear, V_Rd2 = A_sw f_yd 0.9 d/s ≤ 0.25 f_d b d (6.7.2)
- after DE: Schubtragfähigkeit von Wänden mit Schubbewehrungsstäben (kind Shear) und angegebener Schubbeanspruchung in Wandebene, V_Rd2 = A_sw f_yd 0,9 d/s ≤ 0,25 f_d b d (6.7.2)

### en1996 M-42
- before EN: deep-beam bars (kind DeepBeam) of walls with a declared in-plane moment; the stand-alone calculator stays a catalogue entry
- before DE: Wandträgerstäbe (kind DeepBeam) von Wänden mit angegebenem Moment in Wandebene; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Deep-beam bars (kind DeepBeam) of walls with a declared in-plane moment, lever arm z = 0.7 l and F_s = M/z (6.7)
- after DE: Wandträgerstäbe (kind DeepBeam) von Wänden mit angegebenem Moment in Wandebene, Hebelarm z = 0,7 l und F_s = M/z (6.7)

### en1996 M-43
- before EN: confining columns and tie beams: minimum area, reinforcement, spacing and shear; the stand-alone calculator stays a catalogue entry
- before DE: einfassende Pfeiler und Ringbalken: Mindestquerschnitt, Bewehrung, Abstände und Querkraft; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Confining columns and tie beams: minimum area, reinforcement, spacing and shear (EN 1996-1-1 8.7, Annex)
- after DE: Einfassende Pfeiler und Ringbalken: Mindestquerschnitt, Bewehrung, Abstände und Querkraft (EN 1996-1-1 8.7, Anhang)

### en1996 M-50
- before EN: cavity wall ties (kind WallTie): tie density and capacity against the wind pressure; the stand-alone calculator stays a catalogue entry
- before DE: Hohlwandanker (kind WallTie): Ankerdichte und Tragfähigkeit gegen den Winddruck; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Cavity wall ties (kind WallTie): tie density of at least 2.5 ties/m² and tie capacity against the wind pressure (8.5.2.3, EN 1996-2 3)
- after DE: Hohlwandanker (kind WallTie): Ankerdichte von mindestens 2,5 Ankern/m² und Ankertragfähigkeit gegen den Winddruck (8.5.2.3, EN 1996-2 3)

### en1996 M-51
- before EN: anchors and ring ties (kinds Anchor, RingTie): anchor force, spacing and ring tie force; the stand-alone calculator stays a catalogue entry
- before DE: Anker und Ringanker (kinds Anchor, RingTie): Ankerkraft, Abstand und Ringankerkraft; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Anchors and ring ties (kinds Anchor, RingTie): anchor force, spacing and ring tie force of at least 30 kN (8.5.1, Annex D)
- after DE: Anker und Ringanker (kinds Anchor, RingTie): Ankerkraft, Abstand und Ringankerkraft von mindestens 30 kN (8.5.1, Anhang D)

### en1996 M-52
- before EN: movement joints are typed rows (MovementJoint) checked against the recommended panel width of the wall material; unverified NDP basis m52.joints
- before DE: Bewegungsfugen sind typisierte Zeilen (MovementJoint), geprüft gegen die empfohlene Feldbreite des Wandmaterials; ungeprüfte NDP-Grundlage m52.joints
- after EN: Movement joints as typed rows (MovementJoint) checked against the recommended panel width of the wall material; unverified NDP basis m52.joints (EN 1996-2 2)
- after DE: Bewegungsfugen als typisierte Zeilen (MovementJoint), geprüft gegen die empfohlene Feldbreite des Wandmaterials; ungeprüfte NDP-Grundlage m52.joints (EN 1996-2 2)

### en1996 M-60
- before EN: exposure class and mortar class of every wall; unverified NDP basis durability.mortar
- before DE: Expositionsklasse und Mörtelklasse jeder Wand; ungeprüfte NDP-Grundlage durability.mortar
- after EN: Exposure class MX and mortar class suitability of every wall; unverified NDP basis durability.mortar (EN 1996-2, EN 1996-1-1 section 4)
- after DE: Expositionsklasse MX und Eignung der Mörtelklasse jeder Wand; ungeprüfte NDP-Grundlage durability.mortar (EN 1996-2, EN 1996-1-1 Abschnitt 4)

### en1996 M-61
- before EN: lintel deflection and span-to-depth limit of every lintel row; the stand-alone calculator stays a catalogue entry
- before DE: Sturzdurchbiegung und Grenzwert des Verhältnisses Stützweite zu Höhe jeder Sturzzeile; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Lintel deflection δ ≤ l/500 and span-to-depth limit of every lintel row (EN 1996-1-1 7, 8.5.1)
- after DE: Sturzdurchbiegung δ ≤ l/500 und Grenzwert des Verhältnisses Stützweite zu Höhe jeder Sturzzeile (EN 1996-1-1 7, 8.5.1)

### en1996 M-70
- before EN: chases and recesses are typed rows (WallChase) checked against the wall, the bed joint thickness against its mortar type; unverified NDP basis m70.execution
- before DE: Schlitze und Aussparungen sind typisierte Zeilen (WallChase), geprüft gegen die Wand, die Lagerfugendicke gegen ihren Mörteltyp; ungeprüfte NDP-Grundlage m70.execution
- after EN: Chases and recesses as typed rows (WallChase) checked against the wall, and the bed joint thickness against its mortar type; unverified NDP basis m70.execution (EN 1996-2 3, 8)
- after DE: Schlitze und Aussparungen als typisierte Zeilen (WallChase), geprüft gegen die Wand, und die Lagerfugendicke gegen ihren Mörteltyp; ungeprüfte NDP-Grundlage m70.execution (EN 1996-2 3, 8)

### en1996 M-80
- before EN: applicability limits of the simplified method per wall (the check is not applicable with the reasons when a limit is exceeded)
- before DE: Anwendungsgrenzen des vereinfachten Verfahrens je Wand (der Nachweis ist mit Begründung nicht anwendbar, wenn eine Grenze überschritten wird)
- after EN: Applicability limits of the simplified method per wall (height, floor span, storey height, imposed load, thickness); the check is not applicable, with the reasons, when a limit is exceeded (EN 1996-3 4.1, NA-DE 4.1)
- after DE: Anwendungsgrenzen des vereinfachten Verfahrens je Wand (Höhe, Deckenspannweite, Geschosshöhe, Nutzlast, Dicke); der Nachweis ist mit Begründung nicht anwendbar, wenn eine Grenze überschritten wird (EN 1996-3 4.1, NA-DE 4.1)

### en1996 M-81
- before EN: simplified vertical resistance N_Rd of every wall within the applicability limits
- before DE: vereinfachte Vertikaltragfähigkeit N_Rd jeder Wand innerhalb der Anwendungsgrenzen
- after EN: Simplified vertical resistance N_Rd = Φ_s A f_d of every wall within the applicability limits, with Φ_s = 0.85 − 0.0011 (h_ef/t)² (EN 1996-3 4.2, NA-DE 4.2)
- after DE: Vereinfachte Vertikaltragfähigkeit N_Rd = Φ_s A f_d jeder Wand innerhalb der Anwendungsgrenzen, mit Φ_s = 0,85 − 0,0011 (h_ef/t)² (EN 1996-3 4.2, NA-DE 4.2)

### en1996 M-82
- before EN: simplified shear resistance of the shear walls with a declared in-plane shear; the stand-alone calculator stays a catalogue entry
- before DE: vereinfachte Schubtragfähigkeit der Schubwände mit angegebener Schubbeanspruchung in Wandebene; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Simplified shear resistance V_Rd = f_vd t l_c of the shear walls with a declared in-plane shear (EN 1996-3 4.4, NA)
- after DE: Vereinfachte Schubtragfähigkeit V_Rd = f_vd t l_c der Schubwände mit angegebener Schubbeanspruchung in Wandebene (EN 1996-3 4.4, NA)

### en1996 M-83
- before EN: simplified lateral loading of every wall and load case with wind pressure (EN 1996-3) from f_xk1 and the minimum compression
- before DE: vereinfachte Querbelastung jeder Wand und jedes Lastfalls mit Winddruck (EN 1996-3) aus f_xk1 und der Mindestdruckspannung
- after EN: Simplified lateral loading of every wall and load case with wind pressure from f_xk1 and the minimum compression (EN 1996-3)
- after DE: Vereinfachte Querbelastung jeder Wand und jedes Lastfalls mit Winddruck aus f_xk1 und der Mindestdruckspannung (EN 1996-3)

### en1996 M-90
- before EN: tabulated minimum thickness and load ratio mu_fi of every wall with a fire class; the table rows are unverified NDP (m90.fire, fire.<material>.rei<n>)
- before DE: tabellierte Mindestdicke und Ausnutzungsgrad mu_fi jeder Wand mit Feuerwiderstandsklasse; die Tabellenzeilen sind ungeprüfte NDP (m90.fire, fire.<material>.rei<n>)
- after EN: Tabulated minimum thickness t_min and load ratio μ_fi of every wall with a fire class REI; the table rows are unverified NDP (m90.fire, fire.<material>.rei<n>) (EN 1996-1-2 Annex B, NA-DE)
- after DE: Tabellierte Mindestdicke t_min und Ausnutzungsgrad μ_fi jeder Wand mit Feuerwiderstandsklasse REI; die Tabellenzeilen sind ungeprüfte NDP (m90.fire, fire.<material>.rei<n>) (EN 1996-1-2 Anhang B, NA-DE)

### en1996 M-91
- before EN: fire resistance with the declared isotherm depth a_theta of every wall; the stand-alone calculator stays a catalogue entry
- before DE: Feuerwiderstand mit der angegebenen Isothermentiefe a_theta jeder Wand; der eigenständige Rechner bleibt ein Katalogeintrag
- after EN: Fire resistance with the declared isotherm depth a_θ of every wall (EN 1996-1-2 3.3, Annex C)
- after DE: Feuerwiderstand mit der angegebenen Isothermentiefe a_θ jeder Wand (EN 1996-1-2 3.3, Anhang C)

### en1997 G-01
- before EN: stratigraphy of every soil layer: depth interval, parameters, ordering and gaps; the stand-alone entry stays in the catalogue
- before DE: Schichtenfolge jeder Bodenschicht: Tiefenintervall, Parameter, Reihenfolge und Lücken; der eigenständige Eintrag bleibt im Katalog
- after EN: Stratigraphy of every soil layer: depth intervals without gaps or overlaps, parameter ranges and the stress profile σ_v, u (2.4.3)
- after DE: Schichtenfolge jeder Bodenschicht: Tiefenintervalle ohne Lücken oder Überlappungen, Parameterbereiche und der Spannungsverlauf σ_v, u (2.4.3)

### en1997 G-02
- before EN: statistical characteristic lower bound of every laboratory series (GI-17, EN 1997-2 2.4.5.2); the stand-alone series entry G-02 stays in the catalogue
- before DE: statistischer charakteristischer Unterwert jeder Laborversuchsreihe (GI-17, EN 1997-2 2.4.5.2); der eigenständige Reiheneintrag G-02 bleibt im Katalog
- after EN: Statistical characteristic lower bound X_k = X_m (1 − k_n V_x) of every laboratory series (EN 1997-1 2.4.5.2, EN 1997-2 2.4.5.2)
- after DE: Statistischer charakteristischer unterer Wert X_k = X_m (1 − k_n V_x) jeder Laborversuchsreihe (EN 1997-1 2.4.5.2, EN 1997-2 2.4.5.2)

### en1997 GEO-F01
- before EN: drained bearing resistance of every footing and load case (Annex D kernel shared with the catalogue entry)
- before DE: Grundbruchwiderstand mit Entwässerung für jede Gründung und jeden Lastfall (Kern nach Anhang D gemeinsam mit dem Katalogeintrag)
- after EN: Drained bearing resistance R/A' = c' Nc bc sc ic + q' Nq bq sq iq + 0.5 γ' B' Nγ bγ sγ iγ of every footing and load case (Annex D.4)
- after DE: Grundbruchwiderstand im dränierten Zustand R/A' = c' Nc bc sc ic + q' Nq bq sq iq + 0,5 γ' B' Nγ bγ sγ iγ jedes Fundaments und Lastfalls (Anhang D.4)

### en1997 GEO-F02
- before EN: undrained bearing resistance of a footing on a cohesive layer; the demo package is a sand and gravel profile, so the trace test also evaluates a clay variant of it
- before DE: Grundbruchwiderstand ohne Entwässerung einer Gründung auf bindiger Schicht; das Demopaket ist ein Sand-Kies-Profil, daher wertet der Trace-Test zusätzlich eine Tonvariante aus
- after EN: Undrained bearing resistance R/A' = (π + 2) cu bc sc ic + q of a footing on a cohesive layer (Annex D.3)
- after DE: Grundbruchwiderstand im undränierten Zustand R/A' = (π + 2) cu bc sc ic + q eines Fundaments auf bindiger Schicht (Anhang D.3)

### en1997 GEO-F03
- before EN: DIN 4017 bearing resistance of every footing and load case as a reference method beside the Annex D kernel
- before DE: Grundbruchwiderstand nach DIN 4017 für jede Gründung und jeden Lastfall als Referenzverfahren neben dem Kern nach Anhang D
- after EN: DIN 4017 bearing resistance R_n,k = A' (c Nc sc ic + γ2 d Nd sd id + γ1 B' Nb sb ib) of every footing and load case as a reference method beside Annex D
- after DE: Grundbruchwiderstand nach DIN 4017 R_n,k = A' (c Nc sc ic + γ2 d Nd sd id + γ1 B' Nb sb ib) jedes Fundaments und Lastfalls als Vergleichsverfahren neben Anhang D

### en1997 GEO-F04
- before EN: base sliding of every footing and load case
- before DE: Gleiten in der Sohlfuge jeder Gründung und jedes Lastfalls
- after EN: Base sliding H_d ≤ R_d with R_t,k = V_k tan δ_k and the passive resistance E_p,d of every footing and load case (2.4)
- after DE: Gleiten in der Sohlfuge H_d ≤ R_d mit R_t,k = V_k tan δ_k und dem Erdwiderstand E_p,d jedes Fundaments und Lastfalls (2.4)

### en1997 EQU-01
- before EN: equilibrium against overturning of every retaining wall
- before DE: Gleichgewicht gegen Kippen jeder Stützwand
- after EN: Equilibrium against overturning E_dst,d ≤ E_stb,d + T_d about the toe of every retaining wall (2.4.7.2)
- after DE: Gleichgewicht gegen Kippen E_dst,d ≤ E_stb,d + T_d um die Fußkante jeder Stützwand (2.4.7.2)

### en1997 UPL-01
- before EN: uplift equilibrium of every uplift case
- before DE: Auftriebsgleichgewicht jedes Auftriebsfalls
- after EN: Uplift equilibrium V_dst,d ≤ G_stb,d + R_d of every uplift case (10.2)
- after DE: Auftriebssicherheit V_dst,d ≤ G_stb,d + R_d jedes Auftriebsfalls (10.2)

### en1997 HYD-01
- before EN: hydraulic heave of every uplift case
- before DE: hydraulischer Grundbruch jedes Auftriebsfalls
- after EN: Hydraulic heave S_dst,d ≤ G'_stb,d with the seepage force on the soil prism of every uplift case (10.3)
- after DE: Hydraulischer Grundbruch S_dst,d ≤ G'_stb,d mit der Strömungskraft auf das Bodenprisma jedes Auftriebsfalls (10.3)

### en1997 SLS-01
- before EN: oedometric and elastic settlement of every footing
- before DE: ödometrische und elastische Setzung jeder Gründung
- after EN: Oedometric and elastic settlement s = Σ σ_z,i h_i/E_oed,i of every footing against the limit (6.6, Annex F)
- after DE: Ödometrische und elastische Setzung s = Σ σ_z,i h_i/E_oed,i jedes Fundaments gegen den Grenzwert (6.6, Anhang F)

### en1997 SLS-02
- before EN: primary consolidation settlement and time-settlement of every oedometer series with compression index and stress increment (reference)
- before DE: Primärsetzung aus Konsolidation und Zeit-Setzung jeder Ödometerreihe mit Kompressionsbeiwert und Spannungszuwachs (Referenz)
- after EN: Primary consolidation settlement s_c = Cc H/(1 + e0) log10((σ'0 + Δσ)/σ'0) and time-settlement of every oedometer series (reference)
- after DE: Primärsetzung aus Konsolidation s_c = Cc H/(1 + e0) log10((σ'0 + Δσ)/σ'0) und Zeit-Setzung jeder Ödometerversuchsreihe (Vergleichsverfahren)

### en1997 SLS-03
- before EN: secondary compression (creep) of every oedometer series with C_alpha and design life (reference)
- before DE: Sekundärsetzung (Kriechen) jeder Ödometerreihe mit C_alpha und Nutzungsdauer (Referenz)
- after EN: Secondary compression s_sec = Cα H/(1 + e0) log10(t2/t1) of every oedometer series over the design life (reference)
- after DE: Sekundärsetzung s_sec = Cα H/(1 + e0) log10(t2/t1) jeder Ödometerversuchsreihe über die Nutzungsdauer (Vergleichsverfahren)

### en1997 SLS-04
- before EN: angular distortion and tilt of the placed footings; emitted once a document holds at least two footing placements (the showcase package)
- before DE: Winkelverdrehung und Schiefstellung der platzierten Gründungen; wird erst ausgegeben, wenn ein Dokument mindestens zwei Gründungsplatzierungen enthält (Showcase-Paket)
- after EN: Angular distortion tan ω = (s_A − s_B)/L and tilt of the placed footings against 1/500, emitted once a document holds two footing placements
- after DE: Winkelverdrehung tan ω = (s_A − s_B)/L und Schiefstellung der gesetzten Fundamente gegen 1/500, ausgegeben, sobald ein Dokument zwei Fundamentpositionen enthält

### en1997 PILE-01
- before EN: compression resistance of every pile from its load-test profiles
- before DE: Drucktragfähigkeit jedes Pfahls aus seinen Probebelastungsprofilen
- after EN: Pile compression resistance R_c,k = min{R_c,m,mean/ξ1; R_c,m,min/ξ2} and R_c,d = R_c,k/γt from the static load-test profiles (7.6.2.2)
- after DE: Pfahldruckwiderstand R_c,k = min{R_c,m,mean/ξ1; R_c,m,min/ξ2} und R_c,d = R_c,k/γt aus den Profilen statischer Probebelastungen (7.6.2.2)

### en1997 PILE-09
- before EN: resistance-settlement of every pile at its serviceability load (bilinear mobilisation plus elastic shortening, E_c from the registry, unverified)
- before DE: Widerstands-Setzungs-Verhalten jedes Pfahls bei seiner Gebrauchslast (bilineare Mobilisierung plus elastische Verkürzung, E_c aus dem Register, ungeprüft)
- after EN: Resistance-settlement of every pile at its serviceability load with bilinear mobilisation plus elastic shortening, E_c from the registry (unverified)
- after DE: Widerstands-Setzungs-Verhalten jedes Pfahls unter Gebrauchslast mit bilinearer Mobilisierung zuzüglich elastischer Stauchung, E_c aus dem Register (ungeprüft)

### en1997 PILE-02
- before EN: compression resistance from the ground-test profiles of every pile (xi3, xi4 of the test profiles)
- before DE: Drucktragfähigkeit aus den Baugrundversuchsprofilen jedes Pfahls (xi3, xi4 der Versuchsprofile)
- after EN: Pile compression resistance R_c,k = min{R_c;cal,mean/ξ3; R_c;cal,min/ξ4} from the ground-test profiles with R_b + R_s (7.6.2.3)
- after DE: Pfahldruckwiderstand R_c,k = min{R_c;cal,mean/ξ3; R_c;cal,min/ξ4} aus den Baugrundprofilen mit R_b + R_s (7.6.2.3)

### en1997 PILE-03
- before EN: tension resistance of every pile with a tension action; emitted from the tension variant of the demo package
- before DE: Zugtragfähigkeit jedes Pfahls mit Zugeinwirkung; wird aus der Zugvariante des Demopakets ausgegeben
- after EN: Tension resistance R_t,d = R_t,k/γs,t of every pile with a tension action, with the favourable pile weight; emitted from the tension variant of the demo package
- after DE: Zugwiderstand R_t,d = R_t,k/γs,t jedes Pfahls mit Zugeinwirkung, mit dem günstig wirkenden Pfahleigengewicht; ausgegeben aus der Zugvariante des Demopakets

### en1997 PILE-04
- before EN: lateral head deflection of every pile (Winkler kernel shared with the catalogue entry)
- before DE: seitliche Kopfverschiebung jedes Pfahls (Winkler-Kern gemeinsam mit dem Katalogeintrag)
- after EN: Lateral head deflection of every pile by the Winkler subgrade with characteristic length λ = (k D/(4 E_p I_p))^(1/4) (7.7)
- after DE: Seitliche Kopfverschiebung jedes Pfahls nach dem Winkler-Bettungsmodell mit der charakteristischen Länge λ = (k D/(4 E_p I_p))^(1/4) (7.7)

### en1997 PILE-05
- before EN: negative skin friction of the settling ground along every pile shaft (reference)
- before DE: negative Mantelreibung des setzenden Baugrunds entlang jedes Pfahlschafts (Referenz)
- after EN: Negative skin friction F_neg = Σ π D q_s,neg h of the settling ground along every pile shaft, F_c,d + F_neg,d ≤ R_c,d (reference)
- after DE: Negative Mantelreibung F_neg = Σ π D q_s,neg h des sich setzenden Bodens entlang jedes Pfahlschafts, F_c,d + F_neg,d ≤ R_c,d (Vergleichsverfahren)

### en1997 PILE-06
- before EN: group efficiency, block failure and corner pile load of every pile group
- before DE: Gruppenwirkungsgrad, Blockbruch und Eckpfahllast jeder Pfahlgruppe
- after EN: Group efficiency η_g by Converse-Labarre, block failure R_block and corner pile load of every pile group
- after DE: Gruppenwirkungsgrad η_g nach Converse-Labarre, Blockbruch R_block und Eckpfahllast jeder Pfahlgruppe

### en1997 PILE-07
- before EN: buckling interface of every pile (critical load against design axial force)
- before DE: Knickschnittstelle jedes Pfahls (kritische Last gegenüber Bemessungswert der Normalkraft)
- after EN: Buckling interface of every pile: critical load N_cr = π² EI/L_eff² against the design axial force
- after DE: Knickschnittstelle jedes Pfahls: kritische Last N_cr = π² EI/L_eff² gegen die Bemessungsnormalkraft

### en1997 PILE-08
- before EN: dynamic test evaluation with xi5/xi6 and the driving formula of every pile with dynamic tests (reference)
- before DE: Auswertung der dynamischen Probebelastung mit xi5/xi6 und Rammformel jedes Pfahls mit dynamischen Versuchen (Referenz)
- after EN: Dynamic test evaluation with ξ5, ξ6 and the driving formula R = η_h W h/(s + 0.5 s_e) of every pile with dynamic tests (reference, 7.6.2.4)
- after DE: Auswertung dynamischer Pfahlprüfungen mit ξ5, ξ6 und der Rammformel R = η_h W h/(s + 0,5 s_e) jedes Pfahls mit dynamischen Prüfungen (Vergleichsverfahren, 7.6.2.4)

### en1997 RW-01
- before EN: earth pressure regime and coefficient of every retaining wall
- before DE: Erddruckzustand und Erddruckbeiwert jeder Stützwand
- after EN: Active earth pressure regime and Coulomb coefficient Ka of every retaining wall (9, Annex C)
- after DE: Erddruckzustand und Coulombscher Beiwert Ka jeder Stützwand (9, Anhang C)

### en1997 RW-10
- before EN: traffic surcharge at the crest of every retaining wall
- before DE: Verkehrslast am Wandkopf jeder Stützwand
- after EN: Traffic surcharge at the crest of every retaining wall with 45° load spread (2.10)
- after DE: Verkehrslast am Wandkopf jeder Stützwand mit Lastausbreitung unter 45° (2.10)

### en1997 RW-02
- before EN: Rankine active coefficient of every retaining wall as a reference
- before DE: aktiver Erddruckbeiwert nach Rankine jeder Stützwand als Referenz
- after EN: Rankine active coefficient Ka = (1 − sin φ')/(1 + sin φ') of every retaining wall as a reference
- after DE: Aktiver Beiwert Ka = (1 − sin φ')/(1 + sin φ') nach Rankine jeder Stützwand als Vergleichswert

### en1997 RW-03
- before EN: passive earth pressure of the embedment of every embedded wall on the curved failure surface
- before DE: passiver Erddruck der Einbindung jeder eingespannten Wand auf der gekrümmten Gleitfläche
- after EN: Passive earth pressure of the embedment of every embedded wall on the curved failure surface (Kp by log-spiral)
- after DE: Erdwiderstand im Einbindebereich jeder Spundwand auf der gekrümmten Gleitfläche (Kp nach logarithmischer Spirale)

### en1997 RW-04
- before EN: at-rest earth pressure of every retaining wall as a reference
- before DE: Erdruhedruck jeder Stützwand als Referenz
- after EN: At-rest coefficient K0 = (1 − sin φ') √OCR of every retaining wall as a reference
- after DE: Ruhedruckbeiwert K0 = (1 − sin φ') √OCR jeder Stützwand als Vergleichswert

### en1997 RW-05
- before EN: surcharge and water pressure on every retaining wall
- before DE: Auflast und Wasserdruck auf jede Stützwand
- after EN: Surcharge Δe = K q and water pressure u = γw (z − z_w) on every retaining wall
- after DE: Auflast Δe = K q und Wasserdruck u = γw (z − z_w) auf jede Stützwand

### en1997 RW-06
- before EN: sliding and overturning of every retaining wall
- before DE: Gleiten und Kippen jeder Stützwand
- after EN: Sliding and overturning of every gravity or cantilever retaining wall, with the base pressure σ = V/B (1 ± 6e/B)
- after DE: Gleiten und Kippen jeder Schwergewichts- oder Winkelstützwand, mit der Sohlpressung σ = V/B (1 ± 6e/B)

### en1997 RW-07
- before EN: required embedment of every embedded wall against the provided one
- before DE: erforderliche Einbindetiefe jeder eingespannten Wand gegenüber der vorhandenen
- after EN: Required embedment of every embedded wall from free earth support equilibrium against the provided embedment
- after DE: Erforderliche Einbindetiefe jeder Spundwand aus dem Gleichgewicht bei freier Auflagerung gegen die vorhandene Einbindetiefe

### en1997 RW-08
- before EN: Mononobe-Okabe thrust of every retaining wall with a horizontal seismic coefficient (the showcase package)
- before DE: Schub nach Mononobe-Okabe jeder Stützwand mit horizontalem Erdbebenbeiwert (Showcase-Paket)
- after EN: Mononobe-Okabe thrust of every retaining wall with a horizontal seismic coefficient (EN 1998-5 7.3.2)
- after DE: Mononobe-Okabe-Schub jeder Stützwand mit einem horizontalen Erdbebenbeiwert (EN 1998-5 7.3.2)

### en1997 RW-09
- before EN: internal stability of reinforced soil (T_max, embedment length) of every wall with reinforcement (reference)
- before DE: innere Standsicherheit von bewehrtem Boden (T_max, Einbindelänge) jeder Wand mit Bewehrung (Referenz)
- after EN: Internal stability of reinforced soil with T_max = K σ_v S_v and the embedment length L_e of every wall with reinforcement (reference)
- after DE: Innere Standsicherheit bewehrten Bodens mit T_max = K σ_v S_v und Verankerungslänge L_e jeder Wand mit Bewehrung (Vergleichsverfahren)

### en1997 SLOPE-01
- before EN: simplified Bishop factor of every slope on its critical or user circle
- before DE: Standsicherheit nach vereinfachtem Bishop-Verfahren jeder Böschung auf ihrem kritischen oder vom Benutzer gewählten Kreis
- after EN: Simplified Bishop factor of safety F_s of every slope on its critical or user circle (11)
- after DE: Vereinfachter Bishop-Sicherheitsfaktor F_s jeder Böschung auf ihrem kritischen oder benutzerdefinierten Kreis (11)

### en1997 SLOPE-02
- before EN: Janbu simplified factor of safety of every slope on the Bishop circle
- before DE: vereinfachter Sicherheitsbeiwert nach Janbu jeder Böschung auf dem Bishop-Kreis
- after EN: Janbu simplified factor of safety F_s = f0 Σ[…]/Σ W tan α of every slope on the Bishop circle (11)
- after DE: Vereinfachter Janbu-Sicherheitsfaktor F_s = f0 Σ[…]/Σ W tan α jeder Böschung auf dem Bishop-Kreis (11)

### en1997 SLOPE-03
- before EN: infinite-slope utilisation of every slope
- before DE: Ausnutzungsgrad der unendlich langen Böschung für jede Böschung
- after EN: Infinite-slope utilisation with F = tan φ'/tan β for dry ground and the seepage-parallel variant of every slope (11)
- after DE: Ausnutzung der unendlich langen Böschung mit F = tan φ'/tan β für trockenen Boden und der Variante mit hangparalleler Strömung jeder Böschung (11)

### en1997 SLOPE-04
- before EN: plane-wedge utilisation of every slope
- before DE: Ausnutzungsgrad des ebenen Gleitkeils für jede Böschung
- after EN: Plane-wedge utilisation of every slope (11)
- after DE: Ausnutzung des ebenen Gleitkeils jeder Böschung (11)

### en1997 ANC-01
- before EN: pull-out resistance of every anchor from its suitability tests
- before DE: Herausziehwiderstand jedes Ankers aus seinen Eignungsprüfungen
- after EN: Pull-out resistance of every anchor from its suitability tests (2.13)
- after DE: Herausziehwiderstand jedes Ankers aus seinen Eignungsprüfungen (2.13)

### en1997 ANC-04
- before EN: tendon capacity and free length of every anchor
- before DE: Zuggliedtragfähigkeit und freie Länge jedes Ankers
- after EN: Tendon capacity and free length of every anchor (2.13)
- after DE: Zuggliedtragfähigkeit und freie Länge jedes Ankers (2.13)

### en1997 ANC-02
- before EN: empirical fixed-length resistance of every anchor
- before DE: empirischer Widerstand der Verpresslänge jedes Ankers
- after EN: Empirical fixed-length resistance of every anchor
- after DE: Empirischer Verpresskörperwiderstand jedes Ankers

### en1997 ANC-03
- before EN: anchor force and proof load of every anchor
- before DE: Ankerkraft und Prüflast jedes Ankers
- after EN: Anchor force and proof load of every anchor
- after DE: Ankerkraft und Prüflast jedes Ankers

### en1997 DEW-01
- before EN: single-well discharge and required number of wells of every layer water with a pumped well (reference)
- before DE: Förderstrom eines Einzelbrunnens und erforderliche Brunnenanzahl jedes Grundwasserleiters mit Pumpbrunnen (Referenz)
- after EN: Single-well discharge and required number of wells of every layer water with a pumped well (reference, 2.14)
- after DE: Einzelbrunnenergiebigkeit und erforderliche Brunnenanzahl jedes Schichtwassers mit Pumpbrunnen (Vergleichsverfahren, 2.14)

### en1997 DEW-03
- before EN: settlement from the groundwater drawdown at every embedded wall with a head across the wall (the showcase package)
- before DE: Setzung infolge Grundwasserabsenkung an jeder eingespannten Wand mit Wasserspiegeldifferenz über der Wand (Showcase-Paket)
- after EN: Settlement from the groundwater drawdown at every embedded wall with a head across the wall (2.14)
- after DE: Setzung infolge der Grundwasserabsenkung an jeder Spundwand mit Potenzialdifferenz über die Wand (2.14)

### en1998 SEI-02
- before EN: zone 0 and the derived reference ground acceleration; mapped by reading the check ids the artifact emits
- before DE: Zone 0 und die abgeleitete Bezugs-Bodenbeschleunigung; Zuordnung durch Auslesen der vom Artefakt erzeugten Prüf-IDs
- after EN: Seismic zone 0 to 3 of NA-DE with the derived reference ground acceleration a_g = γ_I a_gR (3.2.1)
- after DE: Erdbebenzone 0 bis 3 nach NA-DE mit der abgeleiteten Bezugsbodenbeschleunigung a_g = γ_I a_gR (3.2.1)

### en1998 SEI-03
- before EN: kernel function; the spectrum is drawn by the scene diagrams and blessed by the oracle
- before DE: Kernfunktion; das Spektrum wird von den Szenendiagrammen gezeichnet und vom Orakel abgenommen
- after EN: Horizontal elastic response spectrum S_e(T) with damping correction η = √(10/(5 + ξ)) ≥ 0.55, a kernel function drawn by the scene diagrams and blessed by the oracle (3.2.2.2)
- after DE: Horizontales elastisches Antwortspektrum S_e(T) mit Dämpfungskorrektur η = √(10/(5 + ξ)) ≥ 0,55, eine Kernfunktion, die von den Szenendiagrammen gezeichnet und vom Orakel geprüft wird (3.2.2.2)

### en1998 SEI-04
- before EN: kernel function; blessed by the oracle
- before DE: Kernfunktion; vom Orakel abgenommen
- after EN: Design spectrum S_d(T) for linear analysis with the lower bound β = 0.2 and q ≥ 1.0, a kernel function blessed by the oracle (3.2.2.5)
- after DE: Bemessungsspektrum S_d(T) für die lineare Berechnung mit der Untergrenze β = 0,2 und q ≥ 1,0, eine Kernfunktion, vom Orakel geprüft (3.2.2.5)

### en1998 SEI-05
- before EN: kernel function; drawn by the vertical spectrum diagram
- before DE: Kernfunktion; vom Diagramm des vertikalen Spektrums gezeichnet
- after EN: Vertical design spectrum with q = 1.5, a kernel function drawn by the vertical spectrum diagram (3.2.2.3, 3.2.2.5(5))
- after DE: Vertikales Bemessungsspektrum mit q = 1,5, eine Kernfunktion, vom Diagramm des Vertikalspektrums gezeichnet (3.2.2.3, 3.2.2.5(5))

### en1998 SEI-10
- before EN: γ_I (Table 4.3) and ν per class
- before DE: γ_I (Tabelle 4.3) und ν je Klasse
- after EN: Importance factor γ_I of Table 4.3 (0.8, 1.0, 1.2, 1.4) and the damage-limitation reduction ν per importance class (4.2.5)
- after DE: Bedeutungsbeiwert γ_I nach Tabelle 4.3 (0,8; 1,0; 1,2; 1,4) und Abminderungsbeiwert ν der Schadensbegrenzung je Bedeutungskategorie (4.2.5)

### en1998 SEI-11
- before EN: seismic mass G_k + psi_E Q_k (phi, psi_2 of Table 4.2) enters the base shear check; the clause-number match was wrong (it pointed at the not-applicable verdicts), confirmed by reading part_1::storey_seismic_mass_kg and the baseShear explanation
- before DE: seismische Masse G_k + psi_E Q_k (phi, psi_2 nach Tabelle 4.2) geht in den Nachweis der Fundamentschubkraft ein; die Zuordnung über die Abschnittsnummer war falsch (sie verwies auf die Nicht-anwendbar-Befunde), bestätigt durch Auslesen von part_1::storey_seismic_mass_kg und der Erläuterung zu baseShear
- after EN: Seismic mass G_k + ψ_E Q_k with φ and ψ_2 of Table 4.2, entering the base shear check (3.2.4, 4.2.4)
- after DE: Seismische Masse G_k + ψ_E Q_k mit φ und ψ_2 nach Tabelle 4.2, die in den Nachweis der Fußquerkraft eingeht (3.2.4, 4.2.4)

### en1998 SEI-20
- before EN: mapped by reading the check ids the artifact emits
- before DE: Zuordnung durch Auslesen der vom Artefakt erzeugten Prüf-IDs
- after EN: Structural regularity in plan and elevation derived from the geometry and stiffness model with the failed criterion explained, and the consequences for the model and q (4.2.3.2, 4.2.3.3)
- after DE: Regelmäßigkeit des Tragwerks im Grundriss und im Aufriss, aus Geometrie und Steifigkeitsmodell abgeleitet mit Erläuterung des verletzten Kriteriums, und die Folgen für Modell und q (4.2.3.2, 4.2.3.3)

### en1998 SEI-30
- before EN: mapped by reading the check ids the artifact emits
- before DE: Zuordnung durch Auslesen der vom Artefakt erzeugten Prüf-IDs
- after EN: Behaviour factor q = q_0 k_w ≥ 1.5 by structural system and ductility class DCL, DCM, DCH, including limits, detailing compatibility and mixed systems (5.2.2.2, 6.3.2, 7.3.1, 8.3, 9.3, 4.3.4)
- after DE: Verhaltensbeiwert q = q_0 k_w ≥ 1,5 nach Tragsystem und Duktilitätsklasse DCL, DCM, DCH, einschließlich Grenzen, Verträglichkeit der Durchbildung und gemischter Systeme (5.2.2.2, 6.3.2, 7.3.1, 8.3, 9.3, 4.3.4)

### en1998 SEI-35
- before EN: mapped by reading the check ids the artifact emits
- before DE: Zuordnung durch Auslesen der vom Artefakt erzeugten Prüf-IDs
- after EN: Fundamental period T_1 = C_t H^(3/4) or by the Rayleigh quotient, or the given value (4.3.3.2.2)
- after DE: Grundschwingzeit T_1 = C_t H^(3/4) oder über den Rayleigh-Quotienten oder der angegebene Wert (4.3.3.2.2)

### en1998 SEI-40
- before EN: mapped by reading the check ids the artifact emits
- before DE: Zuordnung durch Auslesen der vom Artefakt erzeugten Prüf-IDs
- after EN: Lateral force method with base shear F_b = S_d(T_1) m λ and the distribution F_i = F_b s_i m_i/Σ s_j m_j for T_1 ≤ min(4 T_C, 2.0 s) (4.3.3.2)
- after DE: Ersatzkraftverfahren mit Fußquerkraft F_b = S_d(T_1) m λ und Verteilung F_i = F_b s_i m_i/Σ s_j m_j für T_1 ≤ min(4 T_C, 2,0 s) (4.3.3.2)

### en1998 SEI-41
- before EN: mapped by reading the check ids the artifact emits
- before DE: Zuordnung durch Auslesen der vom Artefakt erzeugten Prüf-IDs
- after EN: Accidental torsion e_ai = ±0.05 L_i and the amplification δ = 1 + 0.6 x/L_e of the element forces (4.3.2)
- after DE: Zufällige Torsion e_ai = ±0,05 L_i und die Vergrößerung δ = 1 + 0,6 x/L_e der Bauteilkräfte (4.3.2)

### en1998 SEI-42
- before EN: kernel functions; blessed by the oracle
- before DE: Kernfunktionen; vom Orakel abgenommen
- after EN: Modal response spectrum analysis with effective modal mass of at least 90 % and SRSS or CQC combination, kernel functions blessed by the oracle (4.3.3.3)
- after DE: Modalanalyse mit Antwortspektrum mit wirksamer modaler Masse von mindestens 90 % und SRSS- oder CQC-Überlagerung, Kernfunktionen, vom Orakel geprüft (4.3.3.3)

### en1998 SEI-43
- before EN: mapped by reading the check ids the artifact emits
- before DE: Zuordnung durch Auslesen der vom Artefakt erzeugten Prüf-IDs
- after EN: Combination of the horizontal components E_Edx + 0.30 E_Edy and 0.30 E_Edx + E_Edy as an envelope (4.3.3.5.1)
- after DE: Überlagerung der Horizontalkomponenten E_Edx + 0,30 E_Edy und 0,30 E_Edx + E_Edy als Einhüllende (4.3.3.5.1)

### en1998 SEI-44
- before EN: mapped by reading the check ids the artifact emits
- before DE: Zuordnung durch Auslesen der vom Artefakt erzeugten Prüf-IDs
- after EN: Second-order effect θ = P_tot d_r/(V_tot h) with the limits 0.10, 0.20 and 0.30 (4.4.2.2(2))
- after DE: Effekt nach Theorie II. Ordnung θ = P_tot d_r/(V_tot h) mit den Grenzen 0,10, 0,20 und 0,30 (4.4.2.2(2))

### en1998 SEI-45
- before EN: mapped by reading the check ids the artifact emits
- before DE: Zuordnung durch Auslesen der vom Artefakt erzeugten Prüf-IDs
- after EN: Damage limitation drift d_r ν ≤ 0.005 h, 0.0075 h or 0.010 h per storey and direction (4.4.3.2)
- after DE: Schadensbegrenzung durch Stockwerksverschiebung d_r ν ≤ 0,005 h, 0,0075 h oder 0,010 h je Geschoss und Richtung (4.4.3.2)

### en1998 SEI-47
- before EN: mapped by reading the check ids the artifact emits
- before DE: Zuordnung durch Auslesen der vom Artefakt erzeugten Prüf-IDs
- after EN: Seismic joint gap d_gap ≥ q_d d_e to the site boundary and ≥ √(d_1² + d_2²) between separate units (4.4.2.7)
- after DE: Erdbebenfuge d_gap ≥ q_d d_e zur Grundstücksgrenze und ≥ √(d_1² + d_2²) zwischen getrennten Einheiten (4.4.2.7)

### en1998 SEI-54
- before EN: aggregates every building check
- before DE: fasst alle Gebäudenachweise zusammen
- after EN: Aggregated verdict of every building check of the seismic design
- after DE: Zusammengefasstes Ergebnis aller Gebäudenachweise der Erdbebenbemessung

### en1999 A-02
- before EN: confirmed: the weld-run check reports the HAZ extent b_haz of each run (table of 6.1.6.3 by thickness, process and junction)
- before DE: bestätigt: der Schweißnahtnachweis meldet die WEZ-Ausdehnung b_haz jeder Naht (Tabelle in 6.1.6.3 nach Dicke, Verfahren und Stoßart)
- after EN: Extent b_haz of the heat-affected zone of each weld run by thickness, process and junction (6.1.6.3)
- after DE: Ausdehnung b_haz der Wärmeeinflusszone jeder Schweißraupe nach Dicke, Verfahren und Anschluss (6.1.6.3)

### en1999 A-04
- before EN: confirmed: per plate element β/ε against the class limits of Table 6.2; the former second fragment en1999.6.2 (resistance checks) was wrong and is dropped
- before DE: bestätigt: je Plattenelement β/ε gegen die Klassengrenzen der Tabelle 6.2; das frühere zweite Fragment en1999.6.2 (Tragfähigkeitsnachweise) war falsch und entfällt
- after EN: Cross-section classification by β/ε of each plate element against the limits of Table 6.2 with ε = √(250/f_o) (6.1.4)
- after DE: Querschnittsklassifizierung über β/ε jedes Plattenelements gegen die Grenzen der Tabelle 6.2 mit ε = √(250/f_o) (6.1.4)

### en1999 A-05
- before EN: corrected: the effective thickness t_eff = ρ_c·t of §6.1.5 is reported by the class check; the former free-outline gross-property fragments were not local buckling
- before DE: korrigiert: die wirksame Dicke t_eff = ρ_c·t nach 6.1.5 wird vom Klassifizierungsnachweis gemeldet; die früheren Fragmente der Bruttoquerschnittswerte freier Ränder betrafen nicht das örtliche Beulen
- after EN: Local buckling effective thickness t_eff = ρ_c t with ρ_c = C1/(β/ε) − C2/(β/ε)² for class 4 elements, reported by the class check (6.1.5)
- after DE: Wirksame Dicke bei lokalem Beulen t_eff = ρ_c t mit ρ_c = C1/(β/ε) − C2/(β/ε)² für Elemente der Klasse 4, vom Klassifizierungsnachweis ausgegeben (6.1.5)

### en1999 A-06
- before EN: partial: edge-stiffened outstands are classified with their stiffener (part_1_1); the iteration for the stiffener spring and intermediate stiffeners are not modelled
- before DE: teilweise: randversteifte Außenstege werden mit ihrer Steife klassifiziert (part_1_1); die Iteration für die Federsteifigkeit der Steife und Zwischensteifen sind nicht modelliert
- after EN: Distortional and stiffened elements: edge-stiffened outstands are classified with their stiffener, the stiffener spring iteration and intermediate stiffeners are not modelled (6.1.4.3-6.1.5.3)
- after DE: Verformungs- und ausgesteifte Elemente: randversteifte Überstände werden mit ihrer Steife klassifiziert, die Iteration der Steifenfeder und Zwischensteifen sind nicht modelliert (6.1.4.3-6.1.5.3)

### en1999 A-11
- before EN: confirmed: gross-section yielding N_t,Rd = A·f_o/γ_M1 is part of the axial check (the fragment en1999.6.2.3 without suffix is not an emitted id)
- before DE: bestätigt: das Fließen des Bruttoquerschnitts N_t,Rd = A·f_o/γ_M1 ist Teil des Normalkraftnachweises (das Fragment en1999.6.2.3 ohne Suffix ist keine ausgegebene Kennung)
- after EN: Tension in the gross section N_o,Rd = A_g f_o/γ_M1 as part of the axial check (6.2.3)
- after DE: Zug im Bruttoquerschnitt N_o,Rd = A_g f_o/γ_M1 als Teil des Normalkraftnachweises (6.2.3)

### en1999 A-12
- before EN: partial: HAZ enters A_eff of the axial check; net section at holes is not modelled
- before DE: teilweise: die WEZ geht in A_eff des Normalkraftnachweises ein; der Nettoquerschnitt an Löchern ist nicht modelliert
- after EN: Tension in the net section N_u,Rd = 0.9 A_net f_u/γ_M2 with HAZ: the HAZ enters A_eff of the axial check, the net section at holes is not modelled (6.2.3)
- after DE: Zug im Nettoquerschnitt N_u,Rd = 0,9 A_net f_u/γ_M2 mit WEZ: die WEZ geht in A_eff des Normalkraftnachweises ein, der Nettoquerschnitt an Löchern ist nicht modelliert (6.2.3)

### en1999 A-14
- before EN: confirmed: bending resistance about both axes
- before DE: bestätigt: Biegetragfähigkeit um beide Achsen
- after EN: Bending resistance M_Rd = α W_el f_o/γ_M1 about both axes (6.2.5)
- after DE: Biegetragfähigkeit M_Rd = α W_el f_o/γ_M1 um beide Achsen (6.2.5)

### en1999 A-15
- before EN: confirmed: shear resistance in both directions
- before DE: bestätigt: Querkrafttragfähigkeit in beiden Richtungen
- after EN: Shear resistance V_Rd = A_v f_o/(√3 γ_M1) in both directions (6.2.6)
- after DE: Querkrafttragfähigkeit V_Rd = A_v f_o/(√3 γ_M1) in beiden Richtungen (6.2.6)

### en1999 A-16
- before EN: confirmed: torsion and shear with torsion
- before DE: bestätigt: Torsion und Querkraft mit Torsion
- after EN: Torsion T_Ed ≤ T_Rd = W_t f_o/(√3 γ_M1) and shear with torsion (6.2.7)
- after DE: Torsion T_Ed ≤ T_Rd = W_t f_o/(√3 γ_M1) und Querkraft mit Torsion (6.2.7)

### en1999 A-17
- before EN: confirmed: combined bending and axial force of §6.2.9
- before DE: bestätigt: Biegung mit Normalkraft nach 6.2.9
- after EN: Combined bending and axial force (N_Ed/N_Rd)^ψ + (M_y,Ed/M_y,Rd)^η + (M_z,Ed/M_z,Rd)^ξ ≤ 1 of the cross-section (6.2.9)
- after DE: Kombinierte Biegung und Normalkraft (N_Ed/N_Rd)^ψ + (M_y,Ed/M_y,Rd)^η + (M_z,Ed/M_z,Rd)^ξ ≤ 1 des Querschnitts (6.2.9)

### en1999 A-18
- before EN: confirmed: bending with shear of §6.2.8
- before DE: bestätigt: Biegung mit Querkraft nach 6.2.8
- after EN: Bending with shear: reduced yield strength (1 − ρ) f_o on the shear area with ρ = (2 V_Ed/V_Rd − 1)² for V_Ed > 0.5 V_Rd (6.2.8)
- after DE: Biegung mit Querkraft: abgeminderte Streckgrenze (1 − ρ) f_o in der Schubfläche mit ρ = (2 V_Ed/V_Rd − 1)² für V_Ed > 0,5 V_Rd (6.2.8)

### en1999 A-20
- before EN: flexural buckling about y (fb) and z (fb.z)
- before DE: Biegeknicken um y (fb) und um z (fb.z)
- after EN: Flexural buckling resistance N_Rd = κ χ ω_x A_eff f_o/γ_M1 about the y axis (fb) and the z axis (fb.z) (6.3.1.1-6.3.1.3)
- after DE: Biegeknicktragfähigkeit N_Rd = κ χ ω_x A_eff f_o/γ_M1 um die y-Achse (fb) und die z-Achse (fb.z) (6.3.1.1-6.3.1.3)

### en1999 A-22
- before EN: confirmed: lateral-torsional buckling M_b,Rd of §6.3.2
- before DE: bestätigt: Biegedrillknicken M_b,Rd nach 6.3.2
- after EN: Lateral-torsional buckling resistance M_b,Rd = χ_LT W_y f_o/γ_M1 (6.3.2)
- after DE: Biegedrillknicktragfähigkeit M_b,Rd = χ_LT W_y f_o/γ_M1 (6.3.2)

### en1999 A-23
- before EN: confirmed: interaction of axial compression and bending (Eq. 6.59, 6.60)
- before DE: bestätigt: Interaktion von Drucknormalkraft und Biegung (Gl. 6.59, 6.60)
- after EN: Interaction of axial compression and bending according to Equations 6.59 and 6.60 (6.3.3)
- after DE: Interaktion von Druckkraft und Biegung nach den Gleichungen 6.59 und 6.60 (6.3.3)

### en1999 A-25
- before EN: matched by clause number; the artifact owner confirms the family realises this id
- before DE: über die Abschnittsnummer zugeordnet; der Artefakteigner bestätigt, dass die Familie diese Kennung umsetzt
- after EN: Shear buckling resistance of plate girder webs V_Rd = v_w h_w t_w f_o/(√3 γ_M1) with λ_w = 0.35 (h_w/t_w) √(f_o/E) (6.7.4)
- after DE: Schubbeultragfähigkeit von Plattenträgerstegen V_Rd = v_w h_w t_w f_o/(√3 γ_M1) mit λ_w = 0,35 (h_w/t_w) √(f_o/E) (6.7.4)

### en1999 A-27
- before EN: patch loading of §6.7.5: member action check and the explicit patch-load rows
- before DE: Einzellasteinleitung nach 6.7.5: Bauteilnachweis der Einwirkung und die expliziten Zeilen für Einzellasten
- after EN: Patch loading and web bearing: the member action check and the explicit patch-load rows (6.7.5)
- after DE: Einzellast auf den Steg und Stegbeulen: der Bauteil-Einwirkungsnachweis und die expliziten Einzellast-Zeilen (6.7.5)

