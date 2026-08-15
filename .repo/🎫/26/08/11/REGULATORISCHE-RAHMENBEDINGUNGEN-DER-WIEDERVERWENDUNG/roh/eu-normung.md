# W1 · EU/EEA-Basisschicht „Normung" — CEN/TC 350, CEN/TC 250, ISO 20887, ISO 13822

**Projekt:** BBSR/Zukunft Bau „Entwerfen mit Bestand" (Az. 10.08.18.7-25.06, LUH Hannover + UdK Berlin)
**Recherchestand:** 2026-08-11. Alle Primärquellen live via WebSearch/WebFetch geöffnet; bei zwei Dokumenten (ISO 13822, ISO 20887) wurde der Volltext als PDF geladen und lokal per `pdftotext` entzerrt (FlateDecode-Problem, siehe Projektgedächtnis), sodass echte B0-Zitate möglich waren. Bei allen CEN-Dokumenten (EN/prEN/CEN-TS) war der Normtext selbst paywalled und wurde **nicht** im Volltext eingesehen — Fakten stützen sich auf amtliche/amtsnahe Sekundärbeschreibungen (JRC-Eurocodes-Portal, CEN-CENELEC-Newsroom, nationale Normungsstellen-Kataloge DIN/AFNOR/SIS/BSI). Dies ist durchgehend als B1/B2 + `paywalled-nicht-eingesehen` markiert; **keine erfundenen Wortlautzitate** für nicht eingesehene Normtexte (Belegstrenge-Regel, Abschnitt 9 Freeze).

**Kopfzeilen-Pflichthinweis:** Evidenzgrade werden je Achse einzeln vergeben (A/B/D/G-explizit typischerweise E1; C/E an Rändern E2; F1/F2/G-inferiert stets E3). Ein einziger Konfidenzwert pro Objekt wird NICHT erzwungen.

**Methodische Vorbemerkung zu Wortlautzitaten:** Aus urheberrechtlicher Vorsicht werden Zitate durchgehend kurz gehalten (einzelne Kernsätze/-fragmente statt mehrerer Absätze); bei nicht im Volltext eingesehenen CEN-Normtexten wird auf ein Wortlautzitat verzichtet und stattdessen die Sekundärquelle transparent benannt (kein Beleg = kein Zitat).

**Offene Schema-Frage an W4 (zwei Fälle, hier pragmatisch entschieden, nicht eigenmächtig geändert):**
1. *A-Wert für ISO-Dokumente ohne Pinning auf ein Land:* ISO 13822 und ISO 20887 sind als ISO-Dokumente nie selbst „EU/EEA" (A-Ursprung = international). Der Freeze definiert A=national nur im Kontext einer konkreten Landestransposition (Beispiel DIN ISO 13822). Auf dieser Basisschicht wird keine einzelne Jurisdiktion fixiert. **Entscheidung dieses Harvests:** A=national wird als pragmatischer Default beibehalten (dem Freeze-Beispiel folgend), mit Downstream-Verifikationsstatus=„strukturell angenommen, nicht verifiziert" und einer Mehrländer-Stichprobe im Kernaussage-Text statt im Sub-Ebene-Feld (da nicht A=sub-national). Bittet W4 um Klärung, ob ein vierter A-Wert oder ein Attribut „A-Reichweite: mehrere nationale Adoptionen, keine einzelne fixiert" sinnvoller wäre.
2. *D-Wert für CEN/TS (Technische Spezifikation):* Die Ordinalskala kennt `hEN` und `Eurocode/CEN-Bemessungsnorm`, aber keinen eigenen Wert für CEN/TS (schwächere Bindungsstufe: NSB-Bekanntgabepflicht, aber keine Rückzugspflicht kollidierender nationaler Normen wie bei EN). **Entscheidung dieses Harvests:** CEN/TS wird als `nat.Norm` mit Freitext-Flag kodiert (nächstliegender Wert), da die Publikation letztlich als nationalpräfigiertes Dokument erscheint (z. B. DIN CEN/TS …). Meldung an W4 zur Prüfung eines eigenen Ordinalwerts.

---

### REG-EU-2-001 · Eurocode Bestandsbewertung (2. Generation)
- Titel: EN 1990-2 „Eurocode — Basis of structural and geotechnical design — Part 2: Assessment of existing structures" (CEN/TC 250, unter EU-Mandat M/515)
- Fundstelle: gesamte Norm (kein Absatz zitierbar, Normtext nicht im Volltext eingesehen); Status-/Terminfakten aus JRC-Eurocodes-Portal
- A: EU/EEA (CEN-Erarbeitungs-/Freigabeebene; noch kein Nationaler Anhang, keine VV-TB-Listung verifiziert) · A-Ursprung: deckungsgleich, Attribut entfällt · Downstream-Verifikationsstatus: nicht geprüft (nationale Transformation je Mitgliedstaat steht W2 offen)
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit (Bestandsbewertung ist im B-Vokabular explizit Feld 2) · Nebenfelder: 6 Normen/Regelwerke
- C: materialübergreifend (Eurocode-Basisdokument inkl. geotechnischer Bauwerke, materialspezifische Vertiefung erfolgt in EN 1992-2/1993-2 ff.)
- D: Eurocode/CEN-Bemessungsnorm
- E: Bestandserkundung, Planung/Nachweis
- F1 (E3): ermöglichend — schafft erstmals einen eigenständigen, europaweit einheitlichen normativen Rahmen für die Bewertung bestehender Tragwerke (bislang nur Technische-Spezifikations-Stufe, REG-EU-2-002); Voraussetzung für belastbare rechnerische Nachweise bei Wiederverwendung tragender Bestandsbauteile, weil erstmals Regeln zur Aktualisierung von Bemessungswerten anhand von Bestandsdaten normativ verankert werden.
- F2 (E3): bedingend — die Wirkung hängt vollständig von der nationalen Transformation ab (Nationaler Anhang je Mitgliedstaat, Listung als Technische Baubestimmung/VV TB). Zum Stichtag 2026-08-11 ist nur die CEN-Ebene erreicht: Formal Vote unter M/515 im November 2025 abgeschlossen, Verteilung an die nationalen Normungsstellen laut Eurocodes-Homepage spätestens 30.3.2026; das gemeinsame Date of Withdrawal (Rückzug der 1. Generation) liegt erst am 30.3.2028. Bis dahin gilt EN 1990-2 parallel zur ersten Generation, ohne dass nationale Bauaufsicht sie bereits zwingend zugrunde legen muss.
- G: rechnerischer Nachweis (inferiert, E3 — aus Funktionsbeschreibung „Assessment"/Bemessungswert-Aktualisierung, Normtext selbst nicht eingesehen); Dokumentenlage (inferiert, E3)
- Kernaussage: EN 1990-2 ist der neue, eigenständige zweite Teil von EN 1990 der zweiten Eurocode-Generation und regelt erstmals auf Eurocode-Ebene die Bewertung bestehender Tragwerke einschließlich geotechnischer Bauwerke sowie allgemeine Prinzipien für Eingriffe (Verstärkung, Umbau), in Verbindung mit EN 1990-1. Das Dokument löst die bisherige Technische Spezifikation CEN/TS 17440:2020 ab, die unter demselben EU-Mandat M/515 als Vorläufer diente.
- Wortlautbeleg (Originalsprache): nicht eingesehen (Normtext paywalled) — Sekundärquelle beschreibt den Zweck als Bereitstellung von „provisions for the assessment of existing structures … and the general principles for interventions, to be used in conjunction with prEN 1990-1" (iTeh-Katalogbeschreibung, nicht Primärtext, daher nicht als Zitat übernommen, nur referiert).
- Beleg-Quelle: B1 amtliche Konsolidierung/Auszug eingesehen (Status- und Terminfakten aus dem offiziellen JRC-Eurocodes-Portal direkt gelesen) für Status/Termine; B2 amtliche Referenz für den Norminhalt selbst (Volltext ungesehen) · Zugänglichkeit: frei-primär (Statusinformation JRC) / paywalled-nicht-eingesehen (Normtext) · Bindungsakt: Bindungsmechanismus existiert, Listung im Einzelfall nicht verifiziert — freier amtlicher Akt ist das EU-Kommissionsmandat M/515 (öffentlich, JRC-Portal); die für Vollbindung nötige Listung als Technische Baubestimmung/VV TB je Mitgliedstaat ist Aufgabe W2 und hier nicht verifiziert.
- Quelle: Tier 1 · https://eurocodes.jrc.ec.europa.eu/news/second-generation-eurocodes-milestones-achieved · https://eurocodes.jrc.ec.europa.eu/news/towards-publication-second-generation-eurocodes-european-commission-mandate-m515-now-completed · https://eurocodes.jrc.ec.europa.eu/ · Fassung(as-amended) 2026-01-12 · Zugriff 2026-08-11
- Status: Übergang (Formal Vote abgeschlossen 11/2025; Verteilung an NSB spätestens 30.3.2026 laut JRC-Portal; gemeinsames Date of Withdrawal für alle Eurocode-Teile der 1. Generation: 30.3.2028) · Stand 2026-01-12
- Sub-Ebene: nicht erhoben (Objekt auf EU/EEA-Ebene, keine sub-nationale Kodierung einschlägig; nationale Annex-Erhebung ist W2-Aufgabe)
- Relationen: ersetzt REG-EU-2-002 (CEN/TS 17440:2020); wird kombiniert mit REG-EU-2-003 (ISO 13822 als methodisches Vorbild), REG-EU-2-004 (Bemessungsregeln für wiederverwendete Stahlbauteile, künftiger Teil von EN 1993)
- Konfidenz: gesichert (Status/Termine, JRC-Primärquelle gelesen); abgeleitet (Norminhalt im Detail, da Volltext nicht eingesehen)

---

### REG-EU-2-002 · CEN/TS 17440:2020 — Vorläufer-Spezifikation Bestandsbewertung
- Titel: CEN/TS 17440:2020 „Assessment and retrofitting of existing structures"
- Fundstelle: gesamtes Dokument (Technische Spezifikation, kein Absatz zitierbar, Volltext nicht eingesehen)
- A: EU/EEA (CEN-Ebene, unter EU-Mandat M/515) · Downstream-Verifikationsstatus: verifiziert in DE/FR/UK (mehrsprachige Ausgaben EN/FR/DE laut Katalogeintrag nachgewiesen, s. Quelle) für die Existenz nationaler CEN/TS-Ausgaben; keine Aussage zu Bauordnungsbindung
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit · Nebenfelder: 6 Normen/Regelwerke
- C: materialübergreifend
- D: nat.Norm — **Freitext-Flag:** CEN/TS-Ebene ohne eigenen D-Wert im Schema, s. Vorbemerkung Nr. 2
- E: Bestandserkundung, Planung/Nachweis
- F1 (E3): ermöglichend — erstes unter M/515 verfügbares Dokument, das EN 1990 (1. Generation) um Regeln zur Bestandsbewertung ergänzt, auch für Tragwerke, die ursprünglich nicht nach Eurocode bemessen wurden — genau der Fall bei vielen älteren Bestandsbauteilen, die für Reuse in Frage kommen.
- F2 (E3): bedingend — als Technische Spezifikation (nicht EN) hat das Dokument eine schwächere formelle Bindungsstufe; es „does not include any specific rules for undertaking or managing interventions" (laut Sekundärbeschreibung), liefert also Bewertungsmethodik, aber keine eigenständigen Eingriffsregeln. Wird durch REG-EU-2-001 abgelöst; genaues Rückzugsdatum der TS nicht verifiziert.
- G: rechnerischer Nachweis, Dokumentenlage (inferiert, E3)
- Kernaussage: CEN/TS 17440 lieferte als erstes unter Mandat M/515 erarbeitetes Dokument ergänzende/geänderte Bestimmungen, damit EN 1990 auf die strukturelle Bewertung bestehender Tragwerke angewendet werden kann, einschließlich solcher, die nicht nach Eurocode geplant wurden, sowie Regeln zur Aktualisierung von Bemessungsgrößen anhand von Bestandsdaten. Es handelt sich um ein Übergangsdokument mit Technische-Spezifikations-Status (nicht EN), das mit Veröffentlichung von EN 1990-2 funktional abgelöst wird.
- Wortlautbeleg (Originalsprache): nicht eingesehen (Normtext paywalled) — s. F2 für ein kurzes, als Sekundärzitat gekennzeichnetes Fragment.
- Beleg-Quelle: B1 amtliche Konsolidierung/Auszug eingesehen (JRC-Newsartikel direkt gelesen) für Zweck/Mandat/Datum; B2 für den Normtext selbst · Zugänglichkeit: frei-primär (JRC-Newsartikel) / paywalled-nicht-eingesehen (Normtext) · Bindungsakt: Bindungsmechanismus existiert, Listung im Einzelfall nicht verifiziert — freier Akt: EU-Mandat M/515; VV-TB-Listung je Mitgliedstaat nicht geprüft.
- Quelle: Tier 1 · https://eurocodes.jrc.ec.europa.eu/news/cents-174402020-assessment-and-retrofitting-existing-structures · Fassung(as-amended) 2021-10-12 (Publikationsdatum lt. JRC) · Zugriff 2026-08-11
- Status: Übergang (wird durch REG-EU-2-001 abgelöst) · Publikationsdatum 2021-10-12 (dt./frz./engl. Fassungen) [ERGÄNZT, W4-Prüfung 2026-08-14, live via WebSearch (standards.iteh.ai/BSI/JRC-Sekundärquellen) verifiziert: EN 1990-2:2026 wurde im März 2026 in finaler Fassung an die nationalen Normungsgremien ausgeliefert; verpflichtende nationale Umsetzung erst ab 09/2027; die 1. Eurocode-Generation (und mit ihr faktisch der CEN/TS-17440-Übergangsstatus) wird erst zum 30.03.2028 unionsweit zurückgezogen — zum Stichtag 2026-08-11 ist CEN/TS 17440 damit formell WEDER zurückgezogen NOCH bereits durch EN 1990-2 ersetzt; "Übergang" ist die zutreffende Status-Einordnung, jetzt mit Datum belegt statt offen]
- Sub-Ebene: nicht erhoben
- Relationen: wird ersetzt durch REG-EU-2-001; konkretisiert EN 1990 (1. Generation, nicht als eigenes Objekt geführt)
- Konfidenz: gesichert (Existenz, Mandat, Ablöse-Beziehung); unklar (genaues Rückzugsdatum)

---

### REG-EU-2-003 · ISO 13822:2010 — Bases for design of structures: Assessment of existing structures
- Titel: ISO 13822:2010 „Bases for design of structures — Assessment of existing structures"
- Fundstelle: Clause 1 (Scope); Volltext eingesehen (B0, siehe Beleg-Quelle)
- A: national (pragmatischer Default gem. Freeze-Beispiel, s. Vorbemerkung Nr. 1) · A-Ursprung: international (nicht-EU/EEA, ISO) · Downstream-Verifikationsstatus: strukturell angenommen, nicht verifiziert — plausible nationale Übernahmen (DIN ISO 13822 in DE bereits im DE-Piloten REG-DE-2-xxx-Kontext dokumentiert; ÖNORM-, SN-, NEN-, BS-Pendants in AT/CH/NL/UK als Katalogeinträge auffindbar, hier nicht einzeln verifiziert)
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit (Bestandsbewertung) · Nebenfelder: 6 Normen/Regelwerke
- C: materialübergreifend (Norm ausdrücklich materialoffen: „applicable to existing structures of any material … concrete, steel, timber, masonry, etc.")
- D: nat.Norm (in den jeweiligen Landestransformationen; auf ISO-Ebene selbst kein Schema-D-Wert vorgesehen — internationale Grundnorm, s. A-Ursprung)
- E: Bestandserkundung, Planung/Nachweis
- F1 (E3): ermöglichend — liefert allgemeine, materialunabhängige Anforderungen und Verfahren für die Bewertung bestehender Tragwerke auf Basis von Zuverlässigkeitsgrundsätzen (ISO 2394) und schafft damit die methodische Grundlage, auf der auch EN 1990-2 aufbaut; explizit als Grundlage „for preparing national standards or codes of practice" konzipiert.
- F2 (E3): bedingend — die Norm selbst bindet niemanden; Wirkung entsteht erst über nationale Übernahme (DIN/ÖNORM/SN/NEN/BS ISO 13822) und ggf. Listung in nationalen Bautechnik-Regelwerken; dieser Schritt ist Ländersache (W2) und hier nicht durchgehend verifiziert.
- G: rechnerischer Nachweis, Dokumentenlage (explizit, E1 — Klausel 1 nennt „reliability assessment", Klausel 4 ff. Bewertungsverfahren); zerstörungsfreie Prüfung, Probenahme/Materialprüfung (inferiert, E3, aus Verweis auf materialspezifische Zustandserfassung)
- Kernaussage: ISO 13822 liefert allgemeine Anforderungen und Verfahren zur Bewertung bestehender Tragwerke (Gebäude, Brücken, Industriebauten) auf Basis der Prinzipien struktureller Zuverlässigkeit und der Versagensfolgen, aufbauend auf ISO 2394. Die Bewertung kann durch Nutzungsänderung, verlängerte Nutzungsdauer, Zuverlässigkeitsprüfung oder zeitabhängige Schädigung ausgelöst werden; die Norm ist materialoffen und ausdrücklich als Grundlage für nationale Normen/Regelwerke der Praxis konzipiert — genau der Mechanismus, über den sie in mehreren der zehn Projektländer wirksam wird.
- Wortlautbeleg (Originalsprache): "This International Standard provides general requirements and procedures for the assessment of existing structures (buildings, bridges, industrial structures, etc.) based on the principles of structural reliability and consequences of failure. It is based on ISO 2394."
- Beleg-Quelle: B0 Primärtext-Volltext (PDF-Vorschauexemplar vollständig geladen und per pdftotext gelesen, Clause 1 im Volltext) · Zugänglichkeit: frei-primär (offizielles iTeh-Vorschauexemplar, öffentlich abrufbar) · Bindungsakt: entfällt auf ISO-Ebene selbst; für jede nationale Übernahme gilt die jeweilige nationale Normungsstelle als Bindungsakt (Einzelprüfung W2, hier nicht durchgeführt)
- Quelle: Tier 1 · https://www.iso.org/standard/46556.html (amtliche ISO-Katalogseite) · gelesenes Vorschauexemplar: https://cdn.standards.iteh.ai/samples/46556/7d0859948a6848c3bdd5c6dfdb298b71/ISO-13822-2010.pdf · Fassung(as-amended) 2010-08-01 (bestätigt/reviewed 2016, keine Neufassung bis 2026-08-11 identifiziert) · Zugriff 2026-08-11
- Status: in Kraft · 2010-08-01 (Confirmed 2016; ISO-Katalogseite zeigt keine Nachfolgeversion zum Stichtag)
- Sub-Ebene: Stichprobe [DE — DIN ISO 13822 (s. W0-Pilot REG-DE-2-Kontext, Downstream-Status dort separat zu führen)] / nicht erhoben [AT, CH, NL, FR, BE, UK, SE, DK, NO — ÖNORM/SN/NEN/BS/NF-Pendants als Katalogeinträge plausibel, nicht einzeln geöffnet]
- Relationen: konkretisiert (methodisches Vorbild) REG-EU-2-001 (EN 1990-2 verweist konzeptionell auf denselben Zuverlässigkeitsansatz, textliche Bezugnahme nicht verifiziert); wird kombiniert mit REG-EU-2-001 in der Praxis, solange nationale Eurocode-Annexe fehlen
- Konfidenz: gesichert (Scope-Wortlaut, Status); abgeleitet (Verhältnis zu EN 1990-2, da kein expliziter Normverweis geprüft)

---

### REG-EU-6-006 · ISO 20887:2020 — Design for Disassembly and Adaptability
- Titel: ISO 20887:2020 „Sustainability in buildings and civil engineering works — Design for disassembly and adaptability — Principles, requirements and guidance"
- Fundstelle: Clause 1 (Scope), Clause 3.34 (Definition „re-use"), Clause 5.3.5 (Supporting re-use business models); Volltext eingesehen (B0)
- A: national (pragmatischer Default gem. Freeze-Beispiel, s. Vorbemerkung Nr. 1) · A-Ursprung: international (nicht-EU/EEA, ISO) · Downstream-Verifikationsstatus: strukturell angenommen, nicht verifiziert (BS ISO 20887 in UK als nationale Fassung bestätigt gefunden; weitere nationale Übernahmen in den übrigen neun Ländern nicht einzeln geprüft — Norm ist mit Ausgabejahr 2020 deutlich jünger als ISO 13822 und dürfte seltener bereits in Bauordnungen referenziert sein)
- B: Primärfeld 6 Normen/Regelwerke · Nebenfelder: 2 Bautechnische Zulassung/Standsicherheit (Prinzipien wirken auf Bauteilunabhängigkeit/Verbindungstechnik, die auch statisch relevant ist)
- C: materialübergreifend
- D: nat.Norm (in jeweiligen Landestransformationen; auf ISO-Ebene kein Schema-D-Wert)
- E: Planung/Nachweis, Betrieb/Dokumentation
- F1 (E3): ermöglichend — definiert „re-use" als eigenständigen, von Recycling abgegrenzten Begriff ohne Wiederaufbereitung („use of products or components more than once for the same or other purposes without reprocessing") und macht Design-for-Disassembly/Adaptability-Prinzipien (Zugänglichkeit, Unabhängigkeit, Vermeidung irreversibler Oberflächen, Standardisierung) zu benannten, wenn auch nicht mit Mindestwerten hinterlegten, Planungsanforderungen.
- F2 (E3): schweigend hinsichtlich Verbindlichkeit — die Norm selbst stellt klar, dass sie „does not set specific levels of performance for the disassembly or adaptability of constructed works"; sie ist damit ein Leitfaden mit definitorischer/methodischer Wirkung, aber ohne eigene Schwellenwerte oder Nachweispflichten, die eine Bauaufsicht unmittelbar prüfen könnte.
- G: Anwendbarkeitsnorm ohne Nachweistatbestand (explizit, E1 — Norm benennt selbst das Fehlen von Performance-Levels); Dokumentenlage (inferiert, E3, aus Dokumentations-/Informationskapitel 6)
- Kernaussage: ISO 20887 systematisiert Design-for-Disassembly/Adaptability-Prinzipien (Zugänglichkeit, Unabhängigkeit von Bauteilen, Vermeidung nicht trennbarer Oberflächen, Unterstützung von Wiederverwendungs-Geschäftsmodellen, Standardisierung, Sicherheit der Demontage) für Neubau, Umbau und Sanierung, definiert „re-use" trennscharf von Recycling und Energierückgewinnung und verlangt Dokumentations-/Informationsanforderungen (Materialkonstituenten, Verbindungsdetails, Datendigitalisierung), setzt aber ausdrücklich keine Mindestleistungswerte fest.
- Wortlautbeleg (Originalsprache): "re-use: use of products or components more than once for the same or other purposes without reprocessing" (Clause 3.34); "This document does not set specific levels of performance for the disassembly or adaptability of constructed works" (Clause 1).
- Beleg-Quelle: B0 Primärtext-Volltext (PDF vollständig geladen, per pdftotext gelesen, Clauses 1, 3, 5 im Volltext) · Zugänglichkeit: frei-primär (öffentlich gehostetes Vorschauexemplar über steelconstruct.com, deckungsgleich mit ISO-2020-Ausgabe) · Bindungsakt: entfällt auf ISO-Ebene; nationale Übernahmen einzeln zu prüfen (W2)
- Quelle: Tier 1 · https://www.iso.org/standard/69370.html (amtliche ISO-Katalogseite) · gelesenes Exemplar: https://www.steelconstruct.com/wp-content/uploads/ISO-20887_2020_01.pdf · Fassung(as-amended) 2020-01 (First edition, keine Neuausgabe bis 2026-08-11 identifiziert) · Zugriff 2026-08-11
- Status: in Kraft · 2020-01 (First edition)
- Sub-Ebene: Stichprobe [UK — BS ISO 20887:2020, Katalogeintrag bestätigt] / nicht erhoben [DE, AT, CH, NL, FR, BE, SE, DK, NO]
- Relationen: wird kombiniert mit REG-EU-6-007/-008 (EN 15804/EN 15978, deren Modul D „future re-use, recycling and energy recovery" methodisch aufgreift — ISO 20887 selbst verweist auf EN 15978-Modul-D-Logik, Clause C.9); konkretisiert das allgemeine Nachhaltigkeitsprinzip aus ISO 15392 (Normverweis, nicht als eigenes Objekt geführt)
- Konfidenz: gesichert (Scope-/Definitionswortlaut, Status); abgeleitet (Downstream-Verifikation außerhalb UK)

---

### REG-EU-2-004 · Bemessungsregeln für wiederverwendete Stahlbauteile (CEN/TC 250/SC 3, Vorstufe)
- Titel: JRC Technical Report „Guidance on establishing European rules for the design of reclaimed steel components for reuse" (Hintergrundbericht der Ad-hoc-Gruppe „Design of reclaimed steel components for reuse", CEN/TC 250/SC 3 „Eurocode 3: Design of steel structures")
- Fundstelle: gesamtes JRC-Hintergrundpapier (kein CEN-Normtext, kein Work-Item-Nummer zum Stichtag verifiziert)
- A: EU/EEA (JRC ist EU-Einrichtung; das Papier hat als solches jedoch keine eigene Rechtswirkung, s. F1/F2) · A-Ursprung: deckungsgleich
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit · Nebenfelder: 6 Normen/Regelwerke
- C: Baustahl
- D: Merkblatt (JRC-Wissenschafts-/Hintergrundbericht ohne eigenen Transformationszweck; **kein** Muster-/Modellrecht, da nicht auf identische Übernahme durch mehrere Rechtsträger angelegt, sondern selbst erklärter Vorarbeitscharakter für eine künftige CEN-Normung)
- E: Planung/Nachweis
- F1 (E3): ermöglichend — liefert erstmals eine konkrete inhaltliche Grundlage (ergänzende/geänderte Bestimmungen zu EN 1993-1-1:2022, prEN 1993-1-5:2023, prEN 1993-1-8:2023, prEN 1993-1-10:2022) für die Bemessung wiederverwendeter Stahlbauteile einschließlich Verbindungen, Modifikationen, Erweiterungen und Verstärkungen — bislang die einzige inhaltlich ausgearbeitete Bemessungsgrundlage dieser Art auf CEN-Ebene.
- F2 (E3): schweigend/nicht regelungswirksam — das Papier selbst hat ausdrücklich Hintergrund-/Vorarbeitscharakter („background report", Stand der Diskussion zum Abfassungszeitpunkt) ohne bindende Regelungswirkung; es soll künftige Standardisierung (zunächst eine Technische Spezifikation, perspektivisch ein neuer Teil von EN 1993) vorbereiten, ist aber selbst kein CEN-Work-Item mit Dokumentennummer. Zudem ausdrücklich nicht anwendbar auf ermüdungs- oder erdbebenbeanspruchte wiederverwendete Bauteile — Anwendungsbereich schon konzeptionell eng begrenzt.
- G: entfällt (kein Nachweistatbestand, da kein Normtext; Vorarbeit)
- Kernaussage: Die JRC-Ad-hoc-Gruppe innerhalb CEN/TC 250/SC 3 hat im November 2025 einen Hintergrundbericht veröffentlicht, der ergänzende Bemessungsregeln für die Wiederverwendung geborgener Stahlbauteile in neuen Tragwerken oder neuem Tragwerkskontext skizziert (statische Beanspruchung, ohne Ermüdungs-/Erdbebennachweis) und damit die fachliche Grundlage für eine künftige Technische Spezifikation und langfristig einen neuen Teil von EN 1993 legen soll. Zum Stichtag 2026-08-11 existiert weder ein formeller CEN-Arbeitsauftrag mit Dokumentennummer noch ein Zeitplan.
- Wortlautbeleg (Originalsprache): nicht eingesehen (JRC-Volltext-PDF nicht direkt gelesen, nur über amtliche JRC-Newsseite paraphrasiert referiert) — daher kein Zitat.
- Beleg-Quelle: B1 amtliche Konsolidierung/Auszug eingesehen (JRC-Newsartikel direkt gelesen, inkl. Publikationsdatum) · Zugänglichkeit: frei-primär (JRC-Publikationsserver, EU-Einrichtung) · Bindungsakt: entfällt/kein Bindungsakt identifiziert (reines Vorarbeitspapier ohne Rechtsfolge)
- Quelle: Tier 1 · https://eurocodes.jrc.ec.europa.eu/news/jrc-technical-report-guidance-establishing-european-rules-design-reclaimed-steel-components · ergänzend Tier 2 (peer-reviewed, nur Kontext, kein Beleg für Normfakten): https://onlinelibrary.wiley.com/doi/10.1002/stco.202300036 (Feldmann et al. 2024) · Fassung(as-amended) 2025-11-23 · Zugriff 2026-08-11
- Status: Entwurf/Vorstufe (Hintergrundbericht, kein CEN-Normtext; Perspektive: zunächst Technische Spezifikation, dann neuer EN-1993-Teil, ohne verifizierten Zeitplan) · 2025-11-23
- Sub-Ebene: nicht erhoben (EU-Ebene, keine Normwirkung, daher keine sub-nationale Kodierung einschlägig)
- Relationen: konkretisiert (Vorarbeit zu) REG-EU-2-001 im weiteren Sinn (dieselbe CEN/TC-250-Familie, unterschiedliche SC); wird kombiniert mit REG-EU-2-005 (CEN/TS 1090-201, komplementäre Ausführungs-/Deklarationsseite desselben Themas, andere TC)
- Konfidenz: gesichert (Existenz, Datum, Trägerschaft); unklar (Zeitplan, künftiger Normstatus)

---

### REG-EU-2-005 · CEN/TS 1090-201 — Wiederverwendung von Baustahl (Ausführung)
- Titel: CEN/TS 1090-201 „Execution of steel structures and aluminium structures — Reuse of structural steel"
- Fundstelle: gesamtes Dokument (Technische Spezifikation, ergänzende Bestimmungen zu EN 1090-2); Volltext nicht eingesehen
- A: EU/EEA (CEN-Ebene, CEN/TC 135 „Execution of steel structures") · Downstream-Verifikationsstatus: verifiziert in DE, SE (DIN CEN/TS 1090-201:2025-01 sowie SIS-Ausgabe 2024 als nationale Bekanntmachungen bestätigt gefunden), strukturell angenommen für weitere ca. 30 CEN-Mitgliedstaaten (Bekanntgabepflicht laut CEN/CENELEC-Internal-Regulations), nicht einzeln verifiziert
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit (Wiederverwendbarkeitsbewertung als Verwendbarkeitsnachweis-Baustein) · Nebenfelder: 6 Normen/Regelwerke, 1 Produkt-/Konformitätsrecht (Deklarationspflichten analog Leistungserklärung, ohne CPR-Bezug im engeren Sinn)
- C: Baustahl
- D: nat.Norm — **Freitext-Flag:** CEN/TS-Ebene ohne eigenen D-Wert im Schema, s. Vorbemerkung Nr. 2
- E: Aufbereitung/Prüfung, Inverkehrbringen, Planung/Nachweis
- F1 (E3): ermöglichend — legt erstmals auf CEN-Ebene konkrete, veröffentlichte (nicht nur diskutierte) Anforderungen an die Wiederverwendbarkeitsbewertung geborgener Baustahlbauteile fest (Festigkeit, Dehnung, Maß-/Formtoleranzen, Wärmebehandlungszustand, Schweißeignung) und schließt damit eine Lücke zwischen Rückbau und Wiedereinbau nach EN 1993-1-1, für Ausführungsklassen EXC1–EXC3 nach EN 1090-2.
- F2 (E3): bedingend — Anwendungsbereich ausdrücklich auf quasi-statisch beanspruchte warmgewalzte Profile sowie warm- oder kaltgeformte Hohlprofile begrenzt, ohne Ermüdungs-/Erdbebennachweis; kaltgeformte Profile, Bleche und mechanische Verbindungsmittel sind ausdrücklich ausgeschlossen. Als CEN/TS (nicht EN) besteht für Mitgliedstaaten nur eine Bekanntgabe-, keine Rückzugspflicht kollidierender nationaler Regeln — die praktische Durchsetzungskraft gegenüber einer vollwertigen EN ist damit geringer.
- G: rechnerischer Nachweis, Probenahme/Materialprüfung (explizit, E1 — Deklaration mechanischer/geometrischer Eigenschaften und Schweißeignung ist Kernpflicht des Dokuments laut übereinstimmenden Sekundärbeschreibungen)
- Kernaussage: CEN/TS 1090-201 ergänzt EN 1090-2 um Anforderungen an die Wiederverwendbarkeitsbewertung und Qualitätsdeklaration geborgener Baustahlbauteile (warmgewalzte Profile, warm-/kaltgeformte Hohlprofile) für nach EN 1993-1-1 ohne Ermüdungs-/Erdbebennachweis bemessene Tragwerke der Ausführungsklassen EXC1–EXC3. Sie ist die bislang einzige bereits veröffentlichte (nicht nur in Vorbereitung befindliche) CEN-Regel speziell zur Wiederverwendung von Baustahl und ergänzt damit die noch unveröffentlichten Bemessungsregeln aus REG-EU-2-004.
- Wortlautbeleg (Originalsprache): nicht eingesehen (Normtext paywalled) — kein Zitat, nur Sekundärbeschreibung referiert.
- Beleg-Quelle: B2 amtliche Referenz, Volltext ungesehen (übereinstimmende Beschreibung aus mehreren nationalen Normungsstellen-Katalogen: DIN Media, SIS, NSAI/Intertek) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: Bindungsmechanismus existiert, Listung im Einzelfall nicht verifiziert — freier amtlicher Akt: CEN/CENELEC Internal Regulations (Bekanntgabepflicht der NSB, öffentlich einsehbar); eine darüberhinausgehende bauordnungsrechtliche Listung (VV TB o. ä.) ist nicht geprüft.
- Quelle: Tier 1 · https://www.dinmedia.de/en/pre-standard/din-cen-ts-1090-201/375151124 · https://www.sis.se/en/produkter/construction-materials-and-building/structures-of-buildings/steel-structures/sis-cents-1090-2012024/ · Fassung(as-amended) 2024-10-24 (CEN-Genehmigung) / DIN-Ausgabe 2025-01 · Zugriff 2026-08-11
- Status: in Kraft (als CEN/TS) · genehmigt 2024-10-24, DIN-Ausgabe 2025-01
- Sub-Ebene: Stichprobe [DE — DIN CEN/TS 1090-201:2025-01; SE — SIS-CEN/TS 1090-201:2024] / nicht erhoben [übrige ~28 CEN-Mitgliedstaaten mit Bekanntgabepflicht]
- Relationen: konkretisiert EN 1090-2 (nicht als eigenes Objekt geführt); wird kombiniert mit REG-EU-2-004 (Ausführungs-/Deklarationsseite vs. Bemessungsseite derselben Reuse-Kette)
- Konfidenz: gesichert (Existenz, Geltungsbereich, Datum); abgeleitet (Bindungswirkung im Einzelstaat)

---

### REG-EU-6-007 · EN 15804 — Modul D (Nutzen/Lasten jenseits der Systemgrenze)
- Titel: EN 15804:2012+A2:2019 „Sustainability of construction works — Environmental product declarations — Core rules for the product category of construction products" (CEN/TC 350)
- Fundstelle: Modul D (Nutzen und Lasten außerhalb der Produkt-Systemgrenze); Amendment A2 CEN-genehmigt 21.07.2019
- A: EU/EEA (CEN-Ebene) — **mit Sonderfall Rechtsbezug:** über CPR (EU) 2024/3110 Anhang II werden ausgewählte EN-15804-Indikatoren zu verpflichtenden Umweltmerkmalen von Bauprodukten; insoweit erhält das Dokument eine mittelbare unmittelbare EU-Rechtswirkung (s. Relationen)
- B: Primärfeld 6 Normen/Regelwerke · Nebenfelder: 1 Produkt-/Konformitätsrecht (seit CPR 2024/3110 Anhang II), 3 Abfall-/Stoffrecht (Modul D erfasst Recycling-/Wiederverwendungs-/Energierückgewinnungsnutzen)
- C: materialübergreifend
- D: nat.Norm (CEN-Norm, kein hEN im CPR-Sinn der DoP-Vermutungswirkung für Basisanforderung 4, sondern methodische Bezugsnorm für Umweltindikatoren)
- E: Inverkehrbringen, Betrieb/Dokumentation
- F1 (E3): ermöglichend — Modul D erfasst ausdrücklich potenzielle Nutzen und Lasten aus Wiederverwendung von Produkten sowie Recycling/Energierückgewinnung von Abfallstoffen aus Bau-, Nutzungs- und Entsorgungsphase und macht damit den ökobilanziellen Vorteil von Wiederverwendung erstmals separat deklarierbar und damit vergleichbar; über die CPR-Anbindung (Nebenfeld 1) erhält dieser Anreiz seit 2026 potenziell regulatorisches Gewicht.
- F2 (E3): bedingend — Modul-D-Werte sind gesondert auszuweisen (nicht in der Kernbilanz A–C enthalten) und hängen stark von Szenarioannahmen (End-of-Life-Marktverfügbarkeit, Substitutionsquote) ab; die praktische Vergleichbarkeit zwischen Umweltproduktdeklarationen ist dadurch begrenzt (in der Fachliteratur wiederholt als Diskussionspunkt benannt, hier nicht als Faktum, sondern als Kontext vermerkt).
- G: Dokumentenlage, Erklärung Dritter (explizit, E1 — EPD ist per Definition eine Drittparteien-verifizierte Erklärung)
- Kernaussage: EN 15804 legt die Kernregeln für Umweltproduktdeklarationen (EPD) von Bauprodukten fest und führte mit Amendment A2 (2019) Modul D ein, das Nutzen und Lasten aus Wiederverwendung, Recycling und Energierückgewinnung jenseits der Produkt-Systemgrenze separat erfasst. Seit Inkrafttreten der neuen Bauprodukteverordnung (EU) 2024/3110 (in Kraft seit 7.1.2025, Anwendung ab 8.1.2026) werden ausgewählte EN-15804-Indikatoren über Anhang II zu verpflichtenden Umweltmerkmalen in der Leistungserklärung — ein seltener Fall, in dem eine CEN-Normmethodik unmittelbar in EU-Verordnungsrecht referenziert wird.
- Wortlautbeleg (Originalsprache): nicht eingesehen (Normtext paywalled) — kein Zitat; Modul-D-Zweckbeschreibung nur paraphrasiert referiert.
- Beleg-Quelle: B2 amtliche Referenz, Volltext ungesehen (übereinstimmende Fachbeschreibungen, u. a. Branchenleitfäden, sowie CPR-Anhang-II-Bezug aus DIBt-Meldung) · Zugänglichkeit: paywalled-nicht-eingesehen (EN-15804-Normtext) / frei-primär (CPR 2024/3110 als EU-Verordnung) · Bindungsakt: benannt für den CPR-Bezug — CPR (EU) 2024/3110 Anhang II, freier EU-Rechtsakt, in Kraft seit 2025-01-07, Anwendung ab 2026-01-08
- Quelle: Tier 1 (CPR-Bezug) · https://www.dibt.de/de/aktuelles/meldungen/nachricht-detail/meldung/umweltvertraeglichkeit-und-nachhaltigkeit-von-bauprodukten-in-der-novellierten-bauproduktenverordnung · Tier 2/3 (Normbeschreibung, nur Kontext) · https://oneclicklca.com/en/resources/articles/en-15804-changes-epds · Fassung(as-amended) 2019-07-21 (A2) · Zugriff 2026-08-11
- Status: in Kraft · A2 genehmigt 2019-07-21; CPR-Anbindung Anwendung ab 2026-01-08
- Sub-Ebene: nicht erhoben (EU-Ebene)
- Relationen: wird kombiniert mit REG-EU-6-008 (EN 15978, gebäudebezogene Anwendung derselben Modul-D-Logik); konkretisiert Basisanforderung „Nachhaltige Nutzung natürlicher Ressourcen" der CPR (Bezug zu CPR 2024/3110, nicht als eigenes Objekt in dieser Datei geführt — s. Feld-1-Harvest)
- Konfidenz: gesichert (CPR-Anbindung, Datum, EU-Rechtsakt gelesen); abgeleitet (Norminhalt Modul D im Detail, da EN-15804-Volltext nicht eingesehen)

---

### REG-EU-6-008 · EN 15978:2026 — Umweltqualität von Gebäuden
- Titel: EN 15978 „Sustainability of construction works — Assessment of environmental performance of buildings — Requirements and guidance" (CEN/TC 350, Neufassung)
- Fundstelle: gesamtes Dokument; Ratifizierung laut CEN-CENELEC-Tracking am 24.11.2025 abgeschlossen (Stufe 60.55, „Decision on Ratification")
- A: EU/EEA (CEN-Ebene)
- B: Primärfeld 6 Normen/Regelwerke · Nebenfelder: 3 Abfall-/Stoffrecht (Lebenszyklusmodule inkl. Modul C/D)
- C: materialübergreifend
- D: nat.Norm
- E: Planung/Nachweis, Betrieb/Dokumentation
- F1 (E3): ermöglichend — bietet eine harmonisierte Methode zur Bewertung der Umweltqualität von Gebäuden mittels Ökobilanzierung über den gesamten Lebenszyklus (inkl. End-of-Life-Modul C und Modul-D-Gutschriften für Wiederverwendung/Recycling) und gilt ausdrücklich sowohl für neue als auch für bestehende Gebäude sowie Sanierungsprojekte — schafft damit die gebäudebezogene Bezugsebene für die produktbezogene Methodik aus EN 15804.
- F2 (E3): schweigend zur Verbindlichkeit — als CEN-Norm ohne identifizierten Verweis in einem EU-Rechtsakt (anders als EN 15804/CPR) bleibt die Anwendung freiwillig, sofern nicht national/vertraglich (z. B. Nachhaltigkeitszertifizierung, Förderkriterien) in Bezug genommen; ein solcher Bezug wurde in dieser Recherche nicht geprüft.
- G: Dokumentenlage, rechnerischer Nachweis (inferiert, E3)
- Kernaussage: Die Neufassung EN 15978:2026 ersetzt die bisherige EN 15978:2011 und liefert eine harmonisierte, ökobilanzbasierte Methode zur Bewertung der Umweltqualität von Neubauten, Bestandsgebäuden und Sanierungsprojekten; sie steht in engem methodischen Zusammenhang mit den produktbezogenen Modul-D-Regeln aus EN 15804 (REG-EU-6-007) und bildet damit die gebäudebezogene Klammer, in der Wiederverwendungsgutschriften auf Gebäudeebene aggregiert werden.
- Wortlautbeleg (Originalsprache): nicht eingesehen (Normtext paywalled, Ratifizierung erst November 2025 abgeschlossen) — kein Zitat.
- Beleg-Quelle: B2 amtliche Referenz, Volltext ungesehen (CEN-CENELEC-Trackingstatus und Katalogbeschreibung übereinstimmend aus mehreren Quellen) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: entfällt/kein Bindungsakt identifiziert (kein EU-Rechtsakt-Verweis gefunden, anders als bei EN 15804/CPR)
- Quelle: Tier 1 · https://www.cencenelec.eu/news-events/news/2026/en-in-the-spotlight/2026-04-17-en-15978-2026/ · Fassung(as-amended) Ratifizierung 2025-11-24, CEN-CENELEC-Meldung 2026-04-17 · Zugriff 2026-08-11
- Status: in Kraft (Ratifizierung abgeschlossen 24.11.2025, CEN-CENELEC-Ankündigung 17.4.2026 — formales Veröffentlichungsdatum der Druckfassung nicht einzeln verifiziert) · Stand 2026-04-17
- Sub-Ebene: nicht erhoben
- Relationen: wird kombiniert mit REG-EU-6-007 (EN 15804); ersetzt EN 15978:2011 (Vorgängerfassung, nicht als eigenes Objekt geführt)
- Konfidenz: gesichert (Ratifizierungsstatus, Datum); unklar (Bindungswirkung außerhalb freiwilliger Anwendung)

---

### REG-EU-6-009 · prEN 18177 — Circular economy in construction: Framework, principles and definitions
- Titel: prEN 18177 „Circular economy in the construction sector — Framework, principles and definitions" (CEN/TC 350/SC 1, WG 1)
- Fundstelle: gesamtes Dokument (Norm-Entwurf, Volltext nicht eingesehen)
- A: EU/EEA (CEN-Ebene, Entwurfsstadium)
- B: Primärfeld 6 Normen/Regelwerke · Nebenfelder: 3 Abfall-/Stoffrecht (Kreislaufwirtschafts-Begriffsbildung)
- C: materialübergreifend
- D: nat.Norm (Entwurfsstadium; Status-Feld trägt „Entwurf", nicht D)
- E: Bestandserkundung, Rückbau/Sicherung, Planung/Nachweis (Begriffsnorm mit Wirkung über mehrere Phasen)
- F1 (E3): ermöglichend — definiert erstmals auf CEN-Ebene ein gemeinsames Begriffs- und Prinzipiengerüst für Kreislaufwirtschaft im Bausektor auf Ebene von Bauwerken und Bauprodukten aller Art und soll damit die begriffliche Fragmentierung zwischen nationalen Regelwerken (unterschiedliche Reuse-/Abfallende-Definitionen, s. Fallenliste) European-weit reduzieren.
- F2 (E3): schweigend zur unmittelbaren Wirkung — als Begriffs-/Rahmennorm ohne eigene Nachweispflichten entfaltet das Dokument erst über Bezugnahme in anderen Normen/Rechtsakten praktische Wirkung; ein solcher Bezug ist zum Stichtag 2026-08-11 nicht identifiziert (Norm selbst noch im Entwurfsstadium, DIN-Entwurfsfassung 2025-04).
- G: Anwendbarkeitsnorm ohne Nachweistatbestand (inferiert, E3 — Begriffsnorm-Charakter aus Titel/Beschreibung abgeleitet, Volltext nicht eingesehen) · **Normtyp: Grundnorm/Begriffsnorm** (Flag gesetzt — definiert Tatbestände, von denen die Anwendbarkeit weiterer CEN/TC-350/SC-1-Dokumente, u. a. REG-EU-6-010, strukturell abhängt)
- Kernaussage: prEN 18177 soll als grundlegendes Begriffs- und Prinzipiendokument der neuen CEN/TC-350/SC-1-Normenfamilie „Circular economy in the construction sector" gemeinsame Definitionen (u. a. für Zirkularität, Reversibilität, Wiederverwendung) und ein Bewertungs-/Umsetzungsrahmenwerk auf Bauwerks- und Bauprodukt-Ebene liefern. Es befindet sich zum Stichtag im Entwurfsstadium (deutsch-/englischsprachige DIN-Entwurfsfassung datiert 2025-04); ein förmlicher CEN-Veröffentlichungstermin ist nicht verifiziert.
- Wortlautbeleg (Originalsprache): nicht eingesehen (Entwurfstext paywalled) — kein Zitat.
- Beleg-Quelle: B2 amtliche Referenz, Volltext ungesehen (übereinstimmende Beschreibung aus DIN-Entwurfskatalog und CEN/TC-350/SC-1-Katalogseite) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: entfällt/kein Bindungsakt identifiziert (Entwurfsstadium, keine Rechtsbezugnahme identifiziert)
- Quelle: Tier 1 (Normungsstellen-Katalog) · https://www.dinmedia.de (DIN EN 18177 Entwurf 2025-04, Katalogeintrag) · https://standards.iteh.ai/catalog/tc/cen/51316ef3-3dea-4483-8aab-cd1a8033cd41/cen-tc-350-sc-1 · Fassung(as-amended) Entwurf 2025-04 · Zugriff 2026-08-11
- Status: Entwurf · DIN-Entwurfsfassung 2025-04, CEN-Formal-Vote-Termin nicht verifiziert
- Sub-Ebene: nicht erhoben
- Relationen: determiniert Anwendbarkeit von REG-EU-6-010 (prEN 17998, Design for Circularity, baut begrifflich auf prEN 18177 auf — Verweisbeziehung aus Arbeitsgruppenstruktur abgeleitet, nicht textlich verifiziert, daher Konfidenz „abgeleitet")
- Konfidenz: gesichert (Existenz, Trägerschaft, Entwurfsstand); abgeleitet (inhaltliche Details, Verhältnis zu REG-EU-6-010)

---

### REG-EU-6-010 · prEN 17998 — Design for Circularity
- Titel: prEN 17998 „Design for circularity" (CEN/TC 350/SC 1)
- Fundstelle: gesamtes Dokument (Norm-Entwurf, Volltext nicht eingesehen; genauer Arbeitsgruppen-Titel/Scope-Wortlaut nicht verifiziert)
- A: EU/EEA (CEN-Ebene, Entwurfsstadium)
- B: Primärfeld 6 Normen/Regelwerke · Nebenfelder: 2 Bautechnische Zulassung/Standsicherheit (Reversibilität/Demontierbarkeit als Planungsanforderung mit Bezug zu Konstruktion)
- C: materialübergreifend
- D: nat.Norm (Entwurfsstadium)
- E: Planung/Nachweis
- F1 (E3): ermöglichend — soll Kriterien für zirkularitätsgerechtes Entwerfen (Reversibilität, Anpassbarkeit, Anteil wiederverwendeter Inhalte) auf CEN-Ebene normieren und ergänzt damit ISO 20887 (REG-EU-6-006) um eine spezifisch europäische, mit der übrigen CEN/TC-350-Familie (Modul D, Begriffsnorm prEN 18177) verzahnte Regelungsschicht.
- F2 (E3): schweigend — Entwurfsstadium ohne verifizierten Veröffentlichungstermin; praktische Wirkung noch nicht eingetreten.
- G: entfällt (Entwurf, kein geprüfter Nachweistatbestand)
- Kernaussage: prEN 17998 ist als CEN-TC-350/SC-1-Norm zum zirkularitätsgerechten Entwerfen („Design for circularity") in Entwicklung, mit thematischer Nähe zu Reversibilität, Anpassbarkeit und Anteil wiederverwendeter Bauprodukte; über den genauen Anwendungsbereich, Fertigstellungsstand und ein Veröffentlichungsdatum liegen in den frei zugänglichen Sekundärquellen keine belastbaren Angaben vor — dieses Objekt ist bewusst als lückenhaft markiert statt mit erfundenen Details aufgefüllt.
- Wortlautbeleg (Originalsprache): nicht eingesehen — kein Zitat, da nicht einmal eine verifizierte Sekundärbeschreibung mit Scope-Wortlaut auffindbar war.
- Beleg-Quelle: B3 Sekundärquelle (nur Nennung in Übersichtsartikeln zu CEN/TC 350/SC 1, keine eigenständige Katalogseite mit Scope-Text gefunden) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: entfällt/kein Bindungsakt identifiziert
- Quelle: Tier 2/3 (Sekundärquelle, nur Existenznachweis) · https://www.reusefully.co.uk/circular-economy-for-buildings-and-the-standards-developing-across-europe · Zugriff 2026-08-11 — **Warnhinweis:** Diese Quelle liegt unter B3/B4 und dient hier ausdrücklich nur als Existenznachweis, nicht als Beleg für Norminhalte (Belegstrenge-Regel).
- Status: Entwurf · genaues Stadium nicht verifiziert
- Sub-Ebene: nicht erhoben
- Relationen: wird determiniert von REG-EU-6-009 (prEN 18177, Begriffsgrundlage); wird kombiniert mit REG-EU-6-006 (ISO 20887, thematisch überlappend)
- Konfidenz: unklar (Existenz plausibel über mehrere Nennungen, aber Inhalt/Stand nicht primärquellenbasiert belegt — ehrlich als Lücke markiert statt spekulativ ausgefüllt)

---

### REG-EU-2-011 · EN 1990 / EN 1993-1-1 (1. Generation) — Referenzobjekt: Schweigen zu Wiederverwendung
- Titel: EN 1990:2002+A1:2005 „Eurocode — Basis of structural design"; EN 1993-1-1:2022 „Eurocode 3 — Design of steel structures — Part 1-1: General rules and rules for buildings" (jeweils 1. bzw. Übergangsgeneration, CEN/TC 250)
- Fundstelle: Gesamtnormen (Referenzobjekt zur Kontrastierung, kein einzelner Absatz zitiert; Volltext von EN 1990:2002 frei über Drittquelle als PDF verfügbar, EN 1993-1-1:2022 paywalled)
- A: EU/EEA (CEN-Ebene) — nationale Bindung erfolgt über die jeweiligen Nationalen Anhänge und deren bauordnungsrechtliche Listung (Ländersache, hier nicht erhoben)
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit (Eurocode-NA) · Nebenfelder: 6 Normen/Regelwerke
- C: Baustahl (EN 1993-1-1) / materialübergreifend (EN 1990)
- D: Eurocode/CEN-Bemessungsnorm
- E: Planung/Nachweis
- F1 (E3): schweigend gegenüber Wiederverwendung — die aktuell (Stand 2026-08-11) noch in Kraft befindliche Bemessungsgrundlage der 1. Generation (EN 1990:2002+A1) und die entsprechenden Werkstoffnormen der Eurocode-3-Reihe (z. B. EN 1993-1-1:2022) enthalten keine dem Projekt bekannten eigenständigen Bestimmungen zur Bemessung mit wiederverwendeten/geborgenen Bauteilen; diese werden erst durch die in dieser Datei dokumentierten Zusatzinstrumente (REG-EU-2-001, -002, -004, -005) adressiert.
- F2 (E3): hemmend in der Praxisfolge — solange die Bemessungsgrundlage schweigt, müssen Reuse-Vorhaben in der Praxis auf einzelfallbezogene nationale Zulassungswege (z. B. ZiE/vBG, s. DE-Pilot REG-DE-2-001/-002) ausweichen, was strukturell wiederkehrenden Prüfaufwand statt einer normierten Regellösung erzeugt — genau der Befund, den REG-EU-2-001/-004 adressieren sollen.
- G: entfällt (kein spezifischer Nachweistatbestand zu Reuse in diesen Normen; allgemeiner rechnerischer Nachweis für Neubemessung bleibt unberührt, ist aber nicht reuse-spezifisch)
- Kernaussage: Die aktuell in den meisten der zehn Projektländer noch bauordnungsrechtlich verankerte 1. Generation der Eurocodes (EN 1990:2002+A1, EN 1993-1-1) trifft keine eigenständigen Aussagen zur Bemessung mit wiederverwendeten Bauteilen; das Thema wird ausschließlich über die in dieser Datei dokumentierten Zusatzinstrumente (Bestandsbewertung EN 1990-2/CEN-TS 17440, Bemessungsregeln für Stahl-Reuse) adressiert, die zum Stichtag noch im Übergang bzw. in Vorbereitung sind. Dieses Referenzobjekt macht das strukturelle „Regelungsvakuum" der Bemessungsgrundlage sichtbar, das den country-level Rückgriff auf Einzelfallzulassungen erklärt.
- Wortlautbeleg (Originalsprache): nicht eingesehen im vollen Wortlaut für EN 1993-1-1:2022 (paywalled); für EN 1990:2002 frei zugängliches Exemplar identifiziert, aber aus Zeit-/Umfanggründen nicht auf eine Reuse-Fehlanzeige hin vollständig durchsucht — daher kein Zitat, Aussage bleibt auf Abwesenheit einschlägiger Literaturhinweise gestützt (Negativbeleg, methodisch schwächer als Positivbeleg).
- Beleg-Quelle: B3 Sekundärquelle (Negativbefund aus Fachliteratur zu reclaimed steel components, die das Fehlen einschlägiger Bestandsregeln in EN 1993-1-1 als Ausgangspunkt der WG24-Arbeiten benennt, s. REG-EU-2-004) · Zugänglichkeit: frei-primär (EN 1990:2002-PDF) / paywalled-nicht-eingesehen (EN 1993-1-1:2022) · Bindungsakt: entfällt (Referenzobjekt ohne eigene neue Rechtsfolge)
- Quelle: Tier 1 (Existenznachweis EN 1990:2002) · https://dl.azmanco.com/standards/EN/EN%201990%20Basis%20of%20structural%20design.pdf · Tier 2 (Negativbefund) · https://onlinelibrary.wiley.com/doi/10.1002/stco.202300036 · Zugriff 2026-08-11
- Status: in Kraft (1. Generation, bis Date of Withdrawal 30.3.2028 parallel zur 2. Generation gültig)
- Sub-Ebene: nicht erhoben (EU-Ebene; nationale Annex-Listung Ländersache)
- Relationen: wird konkretisiert/ergänzt durch REG-EU-2-001 (Bestandsbewertung) und REG-EU-2-004 (Stahl-Reuse-Bemessung); kollidiert nicht mit diesen (keine widersprüchliche, sondern lückenschließende Beziehung)
- Konfidenz: abgeleitet (Schweigen ist ein Negativbefund, gestützt auf Sekundärliteratur, die dieselbe Lücke beschreibt — nicht durch eigene erschöpfende Volltextlektüre von EN 1993-1-1:2022 verifiziert; als methodische Grenze offen markiert)

---

## Zusammenfassung / Übergabehinweise an W4

**Abgedeckte Instrumente (11 Regelungsobjekte):** EN 1990-2 (Bestandsbewertung, 2. Generation), CEN/TS 17440 (Vorläufer), ISO 13822, JRC-Vorstufe zu EN-1993-Reuse-Bemessungsregeln, CEN/TS 1090-201 (Stahl-Reuse-Ausführung), ISO 20887, EN 15804 Modul D, EN 15978, prEN 18177, prEN 17998, EN 1990/EN 1993-1-1 als Referenz-/Schweigen-Objekt.

**Zentrale Befunde:**
1. Die zweite Eurocode-Generation hat mit EN 1990-2 zum Stichtag CEN-Ebene erreicht (Formal Vote 11/2025 abgeschlossen), aber noch keine nationale Bauordnungsbindung; DoW für die 1. Generation erst 30.3.2028 — ein mehrjähriges Übergangsfenster, in dem beide Generationen parallel gelten.
2. Für Baustahl existiert bereits eine veröffentlichte CEN/TS (1090-201, Ausführungs-/Deklarationsseite), während die zugehörigen Bemessungsregeln (EN 1993-Ergänzung) erst als unverbindliches JRC-Hintergrundpapier vorliegen — Ausführung ist der Bemessung hier normungstechnisch voraus.
3. Bei CEN/TC 350/SC 1 (Kreislaufwirtschaft) ist die Begriffs-/Rahmennorm (prEN 18177) am weitesten fortgeschritten (DIN-Entwurf 2025-04 verifiziert); die Design-for-Circularity-Norm (prEN 17998) konnte nur als Existenz, nicht mit Inhalt/Stand belegt werden — ehrlich als Lücke geführt.
4. EN 15804 ist das einzige Dokument dieser Schicht mit einer verifizierten unmittelbaren EU-Rechtsanbindung (CPR 2024/3110 Anhang II) — ein Beleg dafür, dass „Normen/Regelwerke" (Feld 6) und „Produkt-/Konformitätsrecht" (Feld 1) an dieser Stelle strukturell zusammenlaufen (Nebenfeld-Mechanismus des Freeze bewährt sich hier).
5. Zwei Schema-Grenzfälle wurden identifiziert und pragmatisch, nicht eigenmächtig, entschieden (s. Vorbemerkung): A-Wert für ungepinnte ISO-Dokumente; D-Wert für CEN/TS. Beide an W4 zur Klärung gemeldet.

**Nicht erhoben / offen (ehrlich markiert statt erfunden):**
- Exakter CEN-Formal-Vote-/Publikationskalender für prEN 18177 und prEN 17998.
- Vollständige Sub-Ebenen-Erhebung (nationale Übernahmen) für alle zehn Projektländer bei den ISO-Dokumenten und CEN/TS 1090-201 — nur Stichproben (DE, UK, SE) verifiziert, Rest bleibt W2-Aufgabe.
- Ob EN 1990:2002/EN 1993-1-1:2022 tatsächlich an keiner Stelle Reuse erwähnen, wurde nicht durch eigene erschöpfende Volltextlektüre, sondern über einen in der Fachliteratur beschriebenen Negativbefund gestützt (Konfidenz „abgeleitet", nicht „gesichert").
- Kein CEN-Work-Item mit Dokumentennummer für die WG „Reuse of construction products and materials" und WG 8 „Pre-demolition and pre-redevelopment audits" (CEN/TC 350/SC 1) gefunden — beide Arbeitsgruppen existieren laut Sekundärquelle, aber ohne verifizierbares Dokument, daher hier nicht als eigenes Regelungsobjekt geführt, sondern nur benannt.
