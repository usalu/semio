# W2 · Belgien — Vlaanderen (VL), Extraktionsstufe (alle Felder, Anker OVAM/VLAREMA/Tracimat)

**Auftrag:** Extraktion der Regelungsobjekte für BE/Vlaanderen über alle sieben Regelungsfelder, primär gestützt auf OVAM/VLAREMA/Tracimat (Feld 3), ergänzt um die auch in Vlaanderen bindenden föderalen Rechtsakte (Felder 1, 5a, 6, 7) und die VL-eigenen Rechtsakte in Feld 2/4 (Omgevingsvergunning, Energiebesluit).

**Schema:** Bindend `schema/taxonomie-final.md` (Freeze 2026-08-11) — Blockformat §8, Primär-/Nebenfeld-Struktur (B), A-Ursprung/Downstream-Verifikationsstatus (A), Achsen-Evidenzgrade (Abschn. 10), Bindungsketten-Regel (Abschn. 9).

**Input:** `roh/BE-quellen.md` (Quellenkarte, Stand 2026-08-11) als Fundstellenbasis; alle unten als B0/B1 gekennzeichneten Primärtexte wurden in dieser Extraktionssitzung selbst per WebFetch erneut geöffnet und zitiert (Zugriffsdatum 2026-08-11). Wo die Quellenkarte bereits B0-Volltextlesungen einer vorherigen Sitzung dokumentiert und ich sie in dieser Sitzung nicht erneut geöffnet habe, ist das im Feld „Beleg-Quelle" vermerkt (Herkunft: Quellenkarte-Vorsitzung, nicht eigene Sitzung).

**Update-Vermerk (Nachverifikations-Pass, selbes Zugriffsdatum 2026-08-11):** Gezielter erneuter Versuch, die beiden wichtigsten verbleibenden Lücken der Kernobjekte REG-BE-3-010 (VLAREMA Art. 4.3.5, Sloopattest) und REG-BE-3-011 (Tracimat-Erkenningsbesluit) auf B0/B1 zu heben. Ergebnis: **keine Höherstufung möglich, Lücken bleiben bewusst offen statt mit unsicherem Fund geschlossen.** Konkret: (1) `codex.vlaanderen.be`-Direktzugriffe (sowohl `Zoeken/Document.aspx`- als auch `PrintDocument.ashx`-Muster) lieferten wiederholt nur Navigations-/Inhaltsverzeichnis-Fragmente, nicht den Artikeltext von Hoofdstuk 4 — bestätigt die bereits in der Vorsitzung dokumentierte technische Extraktionsgrenze, keine neue Erkenntnis. (2) Eine WebSearch-Zusammenfassung behauptete ein exaktes Erkennungsdatum für Tracimat („Ministerieel Besluit 24.08.2017, publiziert Belgisch Staatsblad 29.09.2017") — dieser Fund wurde bewusst NICHT übernommen, da der direkte WebFetch-Zugriff auf die als Quelle genannte Tracimat-Seite (`tracimat-vzw-erkend-als-sloopbeheerorganisatie-101`) sowie auf `circulairebouweconomie.be` dieses Datum NICHT bestätigte (beide Seiten nennen nur Minister Schauvliege ohne Datumsangabe) — Diskrepanz zwischen WebSearch-Snippet-Synthese und Primärtext, daher als unverifiziert verworfen statt als Faktum übernommen. (3) Neu identifizierter Sekundärhinweis (Marlex/Legalnews, publiziert 2018-01-16): Tracimat wird dort als gemäß „Artikel 4.3.6 ff. VLAREMA" erkannt bezeichnet (abweichend von der bisher in der Karte referenzierten Verortung unter Art. 4.3.5) — dieser Widerspruch zwischen Sekundärquellen ist selbst ein Befund und unten bei REG-BE-3-011 vermerkt, ändert aber nichts an der B2/B3-Einstufung.

**Kompetenzhinweis:** Felder 1 (Produktrecht), 5a (Vergabe), 6 (Normung), 7 (Haftung) sind Bundeskompetenz und gelten in Vlaanderen unverändert — sie werden hier als für VL bindende Rechtsakte mitgeführt, nicht als "belgisches Gesamtrecht" doppelt in WA/BR-Dateien reproduziert. Felder 2 (Genehmigungsverfahren, teilweise) und 3+4 (Abfall/EPB) sind VL-eigene Regionalkompetenz.

---

### REG-BE-1-001 · CPR 2024/3110 — unmittelbare Geltung in Vlaanderen (Querverweis)
- Titel: Verordnung (EU) 2024/3110 des Europäischen Parlaments und des Rates vom 13. November 2024 zur Festlegung harmonisierter Bedingungen für die Vermarktung von Bauprodukten
- Fundstelle: Gesamt-VO, insb. Art. 2/3 (Anwendungsbereich/Begriffe), Art. 26 (Herstellerfiktion); ELI: http://data.europa.eu/eli/reg/2024/3110/oj
- A: EU/EEA · Downstream-Verifikationsstatus: entfällt (unmittelbar geltendes EU-Recht, keine Umsetzung nötig)
- B: Primärfeld 1
- C: materialübergreifend
- D: EU-VO
- E: Inverkehrbringen
- F1 (E3): ermöglichend — enthält erstmals ausdrückliche Regeln zu gebrauchten/wiederaufgearbeiteten Bauprodukten (Art. 26 Herstellerfiktion, Art. 20 Abs. 1 Scope-Begrenzung auf hEN/ETA-Produkte); gilt in Vlaanderen identisch wie im übrigen Belgien und der gesamten EU.
- F2 (E3): schweigend (VL-spezifisch) — die belgische Marktüberwachungspraxis zur neuen CPR (Zuständigkeit FOD Economie, Anpassung des Vollzugsgesetzes Wet 21.12.2013, s. REG-BE-1-002) ist zum Stichtag noch nicht auf die neue VO umgestellt; keine VL-spezifische Abweichung identifiziert, da Marktüberwachung Bundeskompetenz ist.
- G: Erklärung Dritter, rechnerischer Nachweis (explizit=E1, Art. 26/14 CPR — s. Volldetail in `roh/eu-produkt.md` REG-EU-1-001/-002)
- Kernaussage: Dieses Objekt ist ein bewusst knapper Querverweis, kein Re-Extrakt — die vollständige Primärtextanalyse der CPR 2024/3110 liegt bereits in der EU-Basisschicht vor (`roh/eu-produkt.md`, REG-EU-1-001 bis -013). Für Vlaanderen ist relevant, dass die VO als EU-Verordnung unmittelbar und identisch gilt; ein eigener vlaamser Umsetzungsakt existiert und kann nicht existieren (Verordnungscharakter). Die für Reuse zentralen Normen (Herstellerfiktion Art. 26, Scope-Begrenzung Art. 20 Abs. 1, Digitaler Produktpass Art. 75–80) sind daher 1:1 auf vlaamse Sachverhalte anwendbar.
- Wortlautbeleg (Originalsprache): s. `roh/eu-produkt.md` (kein zusätzliches VL-spezifisches Zitat, da kein eigener Normtext)
- Beleg-Quelle: B0 (bereits in W1 vollständig extrahiert und primärtextbelegt, dort zitiert) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (unmittelbar geltend)
- Quelle: Tier 1 · https://eur-lex.europa.eu/eli/reg/2024/3110/oj · Fassung(as-amended) 2024-11-13, in Kraft ab 2026-01-08 · Zugriff 2026-08-11
- Status: in Kraft (ab 2026-01-08) · 2024-11-13
- Sub-Ebene: entfällt (A=EU/EEA)
- Relationen: setzt um → keine (unmittelbar geltend); Zielobjekte in EU-Basisschicht: REG-EU-1-001, REG-EU-1-002, REG-EU-1-004; wird konkretisiert durch REG-BE-1-002 (nationales Marktüberwachungs-/Sanktionsrecht)
- Konfidenz: gesichert
- **ID-Hinweis aus adversarischer Prüfung 2026-08-11:** Die IDs REG-BE-1-001, -002 und -003 sind in `roh/BE-WA-BR.md` erneut vergeben (dort für inhaltsgleiche Bundeskompetenz-Objekte, dort mit teils abweichendem Wortlaut/Detailtiefe). Das ID-Schema (`REG-<ISO2>-<Feld>-<lfd 3-stellig>`) verlangt projektweite Eindeutigkeit — vor W4-Synthese müssen diese Duplikate dedupliziert oder umnummeriert werden, sonst kollidieren zwei nicht-identische Objekte unter derselben ID.

---

### REG-BE-1-002 · Wet 21 december 2013 — föderales CPR-Marktüberwachungs-/Sanktionsgesetz
- Titel: Wet van 21 december 2013 tot uitvoering van de Verordening Nr. 305/2011 van het Europees Parlement en de Raad van 9 maart 2011 tot vaststelling van geharmoniseerde voorwaarden voor het verhandelen van bouwproducten en tot intrekking van Richtlijn 89/106/EEG, en tot opheffing van diverse bepalingen
- Fundstelle: Art. 1–15, insb. Art. 9 ff. (aanstelling controleambtenaren), Art. 15 (inwerkingtreding)
- A: national · A-Ursprung: national · Downstream-Verifikationsstatus: entfällt (Bundesgesetz, gilt unmittelbar auch in Vlaanderen, keine regionale Transformation vorgesehen oder nötig)
- B: Primärfeld 1
- C: materialübergreifend
- D: Gesetz
- E: Inverkehrbringen, Betrieb/Dokumentation
- F1 (E3): bedingend — regelt Marktüberwachung/Sanktionen zur (alten) CPR 305/2011; enthält keine expliziten Bestimmungen zu gebrauchten/wiederaufgearbeiteten Bauprodukten.
- F2 (E3): unklar — ob/wie dieses Vollzugsgesetz für die neue CPR 2024/3110 novelliert wurde, konnte in dieser wie in der vorangegangenen Recherchesitzung nicht ermittelt werden (offener Rechercheauftrag); die VO gilt unmittelbar unabhängig davon, betroffen ist nur die nationale Sanktions-/Kontrollebene.
- G: Dokumentenlage, Einzelfallzulassung (inferiert, E3, aus Kontrollbeamten-Befugnis Art. 9 ff.)
- Kernaussage: Das Gesetz vom 21.12.2013 ist das föderale Vollzugsgesetz zur alten CPR (VO 305/2011): Es ernennt Kontrollbeamte mit Inspektions-, Probenahme- und Verwarnungsgeld-Befugnissen für Bauprodukte auf dem belgischen Markt (inkl. Vlaanderen, da Marktüberwachung nicht regionalisiert ist). Der gelesene Text erwähnt die neue VO 2024/3110 nicht; ob eine Anpassung erfolgt ist, ist offen.
- Wortlautbeleg (Originalsprache): kein direktes Wortlautzitat in dieser Sitzung nachrecherchiert (Beleg aus Vorsitzung übernommen, dort per Etaamb-Volltextseite gelesen)
- Beleg-Quelle: B2 (Herkunft: Quellenkarte-Vorsitzung — Etaamb-Amtsspiegel vollständig gelesen, kein direkter ejustice-Zugriff in jener Sitzung; in dieser Sitzung nicht erneut geöffnet) · Zugänglichkeit: frei-primär (Etaamb ist offizielles Kopie-Portal des Belgisch Staatsblad) · Bindungsakt: entfällt (Gesetz selbst ist der Bindungsakt)
- Quelle: Tier 1 · https://etaamb.openjustice.be/nl/wet-van-21-december-2013_n2014011012.html · Fassung(as-amended) 2014-01-20 (Publikation), Konsolidierungsstand zum 2026-08-11 nicht vollständig verifiziert · Zugriff (Vorsitzung) 2026-08-11
- Status: in Kraft · 2013-12-21 (verabschiedet), 2014-01-20 (publiziert)
- Sub-Ebene: entfällt (A=national, bundeseinheitlich)
- Relationen: konkretisiert REG-BE-1-001; Anpassungsstatus an CPR 2024/3110 ungeklärt (Lücke)
- Konfidenz: abgeleitet (Existenz/Grundinhalt gesichert über Sekundärspiegel; Novellierungsstatus zu 2024/3110 unklar)

---

### REG-BE-1-003 · KB 30 september 2014 — Zulassung technische beoordelingsinstanties (EAD/ETA)
- Titel: Koninklijk besluit van 30 september 2014 betreffende de technische beoordelingsinstanties gemachtigd voor het opstellen van een Europees beoordelingsdocument en voor het verstrekken van een Europese technische beoordeling voor bouwproducten
- Fundstelle: Gesamt-KB (Einzelartikel in dieser Sitzung nicht im Volltext nachrecherchiert)
- A: national · Downstream-Verifikationsstatus: entfällt (föderales KB, gilt unmittelbar auch in Vlaanderen)
- B: Primärfeld 1 · Nebenfelder: 2 (Zulassungsweg für Bauprodukte ohne hEN)
- C: materialübergreifend
- D: RVO
- E: Inverkehrbringen, Planung/Nachweis
- F1 (E3): ermöglichend — schafft die föderale Rechtsgrundlage für die Ernennung von TBI (technische beoordelingsinstanties), die EAD/ETA nach CPR-Logik ausstellen dürfen; BUtgb (s. REG-BE-2-005) ist die einzige belgische TBI. Für Reuse-Bauteile ohne einschlägige hEN eröffnet dies grundsätzlich einen EU-harmonisierten Zulassungsweg über EAD/ETA statt nur über nationale ATG.
- F2 (E3): unklar — ob/wie oft der ETA-Weg für gebrauchte/wiederaufgearbeitete Bauprodukte in Vlaanderen tatsächlich genutzt wird, ist nicht primärquellenbasiert belegt.
- G: Erklärung Dritter (inferiert, E3, aus TBI-Ernennungslogik der CPR-Systematik)
- Kernaussage: Das KB vom 30.9.2014 ist die föderale Rechtsgrundlage für die Anerkennung von technische beoordelingsinstanties (TBI), die im Rahmen der CPR ein Europees beoordelingsdocument (EAD) erarbeiten und eine Europese technische beoordeling (ETA) ausstellen dürfen. BUtgb-UBAtc ist nach den gesichteten Quellen die einzige belgische TBI. Das KB gilt bundeseinheitlich, also auch für in Vlaanderen ansässige Antragsteller.
- Wortlautbeleg (Originalsprache): kein Wortlautzitat verfügbar (Titel-/Fundstellenebene, Volltext in dieser wie in der Vorsitzung nicht gelesen)
- Beleg-Quelle: B2 (Herkunft: Quellenkarte-Vorsitzung, Etaamb-Spiegel per Suche identifiziert, nicht per WebFetch vollständig gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (KB selbst ist Bindungsakt)
- Quelle: Tier 1 · https://etaamb.openjustice.be/nl/koninklijk-besluit-van-30-september-2014_n2014011560.html · Fassung(as-amended) 2014-10-13 (Publikation) · Zugriff (Vorsitzung) 2026-08-11
- Status: in Kraft · 2014-09-30
- Sub-Ebene: entfällt (A=national)
- Relationen: konkretisiert REG-BE-1-001 (CPR-Vollzug für EAD/ETA-Zweig); ergänzt REG-BE-2-005 (BUtgb/ATG)
- Konfidenz: abgeleitet (Existenz/Funktion gesichert; Artikelebene nicht verifiziert)

---

### REG-BE-1-004 · Wet productnormen 21.12.1998 — Negativbefund (kein CPR-/Bauprodukt-Vollzugsanker)
- Titel: Wet van 21 december 1998 betreffende de productnormen ter bevordering van duurzame productie- en consumptiepatronen en ter bescherming van het leefmilieu en de volksgezondheid ("wet productnormen")
- Fundstelle: Kap. I–VII gesamt, insb. Kap. Vbis (Ecodesign, Art. 14quinquies)
- A: national
- B: Primärfeld 1 · Normtyp: Grundnorm/Begriffsnorm (negativ — grenzt den Anwendungsbereich AUS)
- C: materialübergreifend
- D: Gesetz
- E: entfällt (kein reuse-relevanter Vollzugsakt identifiziert)
- F1 (E3): schweigend — das Gesetz regelt Produktnormen für Ecodesign (energieverbrauchende Produkte), Chemikalien, Verpackungen; es enthält KEINE Bestimmungen zur Marktüberwachung von Bauprodukten/CPR-Vollzug. CE-Kennzeichnung wird nur für Ecodesign-Produkte erwähnt, nicht für Bauprodukte.
- F2 (E3): schweigend (VL-spezifisch identisch, da Bundesgesetz)
- G: entfällt
- Kernaussage: Dieses Objekt dokumentiert einen primärquellenbasierten Negativbefund, der für die Extraktionsstufe wichtig ist, um Fehlzuordnungen zu vermeiden: Das Gesetz vom 21.12.1998 — trotz naheliegendem Titel — ist NICHT der Anker der CPR-Marktüberwachung in Belgien; dafür ist REG-BE-1-002 (Wet 21.12.2013) einschlägig. Aufgenommen als Grenzobjekt/Ausschlussbeleg, nicht als reuse-relevante Regel.
- Wortlautbeleg (Originalsprache): kein zusätzliches Zitat (Negativbefund; Beleg = Abwesenheit einschlägiger Bestimmungen im gelesenen Volltext)
- Beleg-Quelle: B0 (Herkunft: Quellenkarte-Vorsitzung, Volltext per WebFetch gelesen, Negativbefund bestätigt) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.ejustice.just.fgov.be/cgi_loi/change_lg.pl?language=nl&la=N&table_name=wet&cn=1998122141 · Fassung(as-amended) 2024-05-31 · Zugriff (Vorsitzung) 2026-08-11
- Status: in Kraft · 1999-02-21 (in Kraft), letzte Novelle 2024-06-10
- Sub-Ebene: entfällt (A=national)
- Relationen: kollidiert mit keiner reuse-Norm (Negativbefund); grenzt REG-BE-1-002 als korrekten Anker ab
- Konfidenz: gesichert (auch als Negativbefund)

---

### REG-BE-2-005 · ATG (Technische Goedkeuring) / BUtgb-UBAtc — nationales Zulassungssystem
- Titel: ATG — Technische Goedkeuring, vergeben durch BUtgb-UBAtc (Belgische Unie voor de Technische Goedkeuring in de Bouw / Union Belge pour l'Agrément technique dans la Construction)
- Fundstelle: kein einzelnes Gründungs-KB identifiziert (institutioneller Nachweis über Selbstdarstellung + historische Einordnung); für den EU-ETA-Zweig s. REG-BE-1-003 (KB 30.9.2014)
- A: national · Downstream-Verifikationsstatus: nicht geprüft (Gründungsrechtsakt nicht identifiziert, daher auch keine Downstream-Prüfung möglich)
- B: Primärfeld 2 · Nebenfelder: 1, 6
- C: materialübergreifend
- D: Verwaltungsvorschrift (vorläufig — Gründungsrechtsform nicht verifiziert, daher Konfidenz "unklar" bei diesem Wert) · **Konsistenzhinweis aus adversarischer Prüfung 2026-08-11: `roh/BE-WA-BR.md` REG-BE-1-004 führt dasselbe Objekt (ATG/BUtgb) mit D = Merkblatt/Branchenprotokoll statt Verwaltungsvorschrift. Da die Gründungsrechtsgrundlage in beiden Sitzungen nicht identifiziert werden konnte, ist keiner der beiden Werte primärquellenbasiert entscheidbar — vor Übernahme in W4 zwingend zu vereinheitlichen.**
- E: Planung/Nachweis
- F1 (E3): ermöglichend (dem Grunde nach) — ATG ist der belgische Funktionsanalog zu DE-ZiE/abZ für innovative bzw. nicht hEN-erfasste Bauprodukte und -bauarten; ein grundsätzlich verfügbarer Zulassungsweg für Bauteile ohne einschlägige harmonisierte Norm, was bei Reuse-Bauteilen der Regelfall ist.
- F2 (E3): unklar — die BUtgb-Website nennt Wiederverwendung/gebrauchte Bauteile nicht ausdrücklich als Anwendungsfall; ob und wie ATG in der Praxis für Reuse-Bauteile genutzt wird, ist NICHT primärquellenbasiert belegt.
- G: Einzelfallzulassung, rechnerischer Nachweis (inferiert, E3, aus dem allgemeinen Zulassungscharakter von ATG-Verfahren, keine Primärtext-Verfahrensbeschreibung eingesehen)
- Kernaussage: ATG ist seit 1970 aktiv (BUtgb seit 2009 als vzw organisiert) und fungiert als belgisches Pendant zur deutschen abZ/ZiE für Bauprodukte und -arten ohne einschlägige Technische Baubestimmung. Die konkrete Gründungsrechtsgrundlage des rein-nationalen (nicht EU-ETA-)Zweigs konnte trotz zweier Rechercherunden nicht identifiziert werden — dies bleibt eine zentrale Lücke, da ohne Kenntnis des Gründungsakts auch dessen Bindungswirkung und ein etwaiger Reuse-Bezug nicht textbelegt geprüft werden können. Der EU-harmonisierte ETA-Zweig ist hingegen über REG-BE-1-003 primärquellenbasiert verortet.
- Wortlautbeleg (Originalsprache): kein Wortlautzitat möglich (kein Normtext identifiziert, nur Selbstdarstellung der Institution)
- Beleg-Quelle: B2 (Website + historische Sekundäreinordnung 1970/2009 eingesehen) · Zugänglichkeit: frei-primär, aber inhaltlich unvollständig · Bindungsakt: nicht identifiziert (Lücke) — für den ETA-Teilzweig s. REG-BE-1-003
- Quelle: Tier 1 (Selbstdarstellung Institution) · https://butgb-ubatc.be/nl/ · Fassung(as-amended) unbekannt · Zugriff 2026-08-11
- Status: in Kraft (institutionell aktiv) · vzw-Rechtsform seit 2009
- Sub-Ebene: entfällt (A=national)
- Relationen: ergänzt REG-BE-1-003 (ETA-Zweig); wird kombiniert mit REG-BE-2-006 (Omgevingsvergunning als Genehmigungsrahmen, in dem ein ATG-Nachweis vorgelegt werden kann)
- Konfidenz: unklar (institutionelle Existenz/Funktion gesichert; Rechtsgrundlage und Reuse-Anwendbarkeit ungeklärt)

---

### REG-BE-2-006 · Omgevingsvergunningendecreet (VL) — Genehmigungsrahmen für Sloop/Verbouwing
- Titel: Decreet van 25 april 2014 betreffende de omgevingsvergunning
- Fundstelle: Gesamtdecreet; Ausführung BVR 27 november 2015; Numac 2014036510
- A: sub-national (Vlaams Gewest) · Downstream-Verifikationsstatus: verifiziert in [Vlaams Gewest] (eigenständiges Regionaldekret, keine weitere Downstream-Transformation nötig — gilt unmittelbar für alle vlaamse Gemeinden als vergunningverlenende Overheid)
- B: Primärfeld 2
- C: materialübergreifend
- D: Gesetz (Decreet des Vlaams Parlement)
- E: Rückbau/Sicherung, Planung/Nachweis
- F1 (E3): bedingend — integriert die frühere getrennte stedenbouwkundige vergunning und milieuvergunning in eine einheitliche Omgevingsvergunning; das ist der verfahrensrechtliche Rahmen, in den das VLAREMA-Sloopopvolgingsplan-Erfordernis (REG-BE-3-009) als Genehmigungsvoraussetzung eingebettet ist.
- F2 (E3): schweigend — im in dieser Sitzung per WebFetch geöffneten Textauszug (Art. 99 Verval-Regelung) fand sich KEIN expliziter Verweis auf sloopvergunning, afbraak oder sloopopvolgingsplan/materiaalkringlopen; dies ist wahrscheinlich ein Artefakt des unvollständig übertragenen Auszugs (Decreet-Volltext ist lang, Sloop-Bezug liegt vermutlich in anderen, nicht im Auszug enthaltenen Artikeln oder wird erst über die VLAREMA-Verknüpfung hergestellt), NICHT als gesicherter Negativbefund zu werten — Lücke bleibt für die Synthese offen.
- G: Dokumentenlage, Einzelfallzulassung (inferiert, E3, aus allgemeiner Vergunningslogik; für sloop-spezifische Nachweispflicht s. REG-BE-3-009, die die eigentliche Textgrundlage trägt)
- Kernaussage: Das Omgevingsvergunningendecreet (Numac 2014036510, publiziert 2014-10-23) bündelt Bau- und Umweltgenehmigung in Vlaanderen zu einer Omgevingsvergunning. Es ist der prozedurale Rahmen, in dem laut Tracimat-Sekundärquelle (REG-BE-3-009) die Beifügung eines Sloopopvolgingsplans bei der Vergunningsaanvraag erfolgt — der konkrete Verknüpfungsartikel im Decreet selbst wurde in dieser Sitzung NICHT lokalisiert (nur in der abgeleiteten VLAREMA/Tracimat-Prozessbeschreibung bestätigt).
- Wortlautbeleg (Originalsprache): "de verwezenlijkte stedenbouwkundige handelingen … wordt meer dan drie opeenvolgende jaren onderbroken" (Art. 99, Verval-Kontext — nicht sloop-spezifisch, nur als Beleg für den gelesenen Auszug zitiert)
- Beleg-Quelle: B2 (Fundstelle identifiziert und Teilauszug per WebFetch gelesen in dieser Sitzung; sloop-spezifische Artikel nicht lokalisiert) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Decreet selbst ist Bindungsakt)
- Quelle: Tier 1 · https://www.ejustice.just.fgov.be/cgi/article_body.pl?language=nl&caller=summary&pub_date=14-10-23&numac=2014036510 · Etaamb-Spiegel: https://etaamb.openjustice.be/nl/decreet-van-25-april-2014_n2014036510.html · Fassung(as-amended) 2014-10-23, Ausführung BVR 2015-11-27 · Zugriff 2026-08-11
- Status: in Kraft · 2014-10-23
- Sub-Ebene: Stichprobe [Vlaams Gewest (Regionaldekret, gemeindeweiter Vollzug als vergunningverlenende Overheid, nicht einzeln nach Gemeinde geprüft)] / nicht erhoben [Gemeinde-spezifische Vollzugspraxis]
- Relationen: wird kombiniert mit REG-BE-3-009/-010 (VLAREMA-Sloopopvolgingsplan als Genehmigungsunterlage); ergänzt REG-BE-2-005 (ATG-Nachweis als möglicher Verfahrensbestandteil)
- Konfidenz: abgeleitet (Existenz/Grundfunktion gesichert; sloop-spezifische Verknüpfungsartikel nicht direkt textbelegt, nur indirekt über REG-BE-3-009)

---

### REG-BE-3-007 · Materialendecreet Art. 3 § 1, 15° — Legaldefinition "hergebruik"
- Titel: Decreet van 23 december 2011 betreffende het duurzaam beheer van materiaalkringlopen en afvalstoffen ("Materialendecreet")
- Fundstelle: Art. 3, § 1, 15°
- A: sub-national (Vlaams Gewest) · Downstream-Verifikationsstatus: verifiziert in [Vlaams Gewest] (Basisdecreet, unmittelbar geltend, Ausführung durch VLAREMA s. REG-BE-3-008 ff.)
- B: Primärfeld 3 · Normtyp: Grundnorm/Begriffsnorm (Legaldefinition, determiniert die Anwendbarkeit aller nachgeordneten VLAREMA-Bestimmungen zu hergebruik)
- C: materialübergreifend
- D: Gesetz (Decreet)
- E: Aufbereitung/Prüfung · E-Wirkung: vermeidet (die Definition zieht ihre Reuse-Bedeutung gerade aus der Abgrenzung "geen afvalstoffen" — hergebruik setzt voraus, dass das Objekt den Abfallstatus nicht durchläuft bzw. verlässt; Phase bewusst vermieden)
- F1 (E3): ermöglichend — definiert "hergebruik" wortgleich zur EU-Abfallrahmenrichtlinien-Systematik (Art. 3 Nr. 13 RL 2008/98/EG, s. REG-EU-3-001) als eigenständige, vom Abfallbegriff getrennte Kategorie; schafft damit die begriffliche Grundlage dafür, dass wiederverwendete Bauteile außerhalb des Abfallregimes behandelt werden können.
- F2 (E3): bedingend — die Definition allein entscheidet noch nicht, WANN ein konkretes Bauteil "geen afvalstof" ist (das bestimmt sich nach der allgemeinen Abfalldefinition/Entledigungswille, hier nicht Gegenstand); ein expliziter Verweis von Art. 3 auf VLAREMA als Ausführungsverordnung wurde im Decreet-Text NICHT gefunden (nur implizite Ermächtigungsstruktur über allgemeine Delegationsnormen wie Art. 5/6).
- G: Anwendbarkeitsnorm ohne Nachweistatbestand (explizit=E1 — reine Begriffsnorm ohne eigene Handlungspflicht)
- Kernaussage: Art. 3 § 1, 15° Materialendecreet definiert hergebruik als "elke handeling waarbij voorwerpen of componenten van voorwerpen die geen afvalstoffen zijn, opnieuw worden gebruikt voor hetzelfde doel als dat waarvoor zij waren bedoeld" — wortgleich zur WFD-Systematik. Als Grundnorm/Begriffsnorm bestimmt sie die Anwendbarkeit aller nachgeordneten VLAREMA-Bestimmungen, die auf "hergebruik" Bezug nehmen (u. a. REG-BE-3-008 ff.), ohne selbst eine Handlungspflicht zu begründen.
- Wortlautbeleg (Originalsprache): "hergebruik : elke handeling waarbij voorwerpen of componenten van voorwerpen die geen afvalstoffen zijn, opnieuw worden gebruikt voor hetzelfde doel als dat waarvoor zij waren bedoeld"
- Beleg-Quelle: B0 (in dieser Sitzung per WebFetch erneut geöffnet und Wortlaut bestätigt) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Decreet selbst ist Bindungsakt)
- Quelle: Tier 1 · https://www.ejustice.just.fgov.be/cgi_loi/change_lg.pl?language=nl&la=N&table_name=wet&cn=2011122333 · Fassung(as-amended) 2024-07-20 (letzte Konsolidierung, DVR 2024-05-17/29 Art. 111) · Zugriff 2026-08-11
- Status: in Kraft · 2012 (Inkrafttreten), letzte Konsolidierung 2024-07-20
- Sub-Ebene: Stichprobe [Vlaams Gewest] / nicht erhoben [entfällt — Regionalebene ist Endpunkt, keine Gemeindeebene für Abfallrecht identifiziert]
- Relationen: setzt um REG-EU-3-001 (WFD-Begriffsnorm Wiederverwendung); determiniert Anwendbarkeit von REG-BE-3-008, REG-BE-3-009, REG-BE-3-010
- Konfidenz: gesichert

---

### REG-BE-3-008 · VLAREMA Art. 2.2.2 / 2.4.1–2.4.3 — Einde-afvalfase-Kriterien und Grondstofverklaring
- Titel: Besluit van de Vlaamse Regering van 17 februari 2012 tot vaststelling van het Vlaams reglement betreffende het duurzaam beheer van materiaalkringlopen en afvalstoffen (VLAREMA)
- Fundstelle: Art. 2.2.2 (algemene voorwaarden einde-afvalfase); Art. 2.4.1.1–2.4.3.2 (grondstofverklaring: aanvraag, Art. 2.4.2.2; opheffing, Art. 2.4.3.1)
- A: sub-national (Vlaams Gewest) · Downstream-Verifikationsstatus: verifiziert in [Vlaams Gewest] (Besluit der Vlaamse Regering, unmittelbar vollziehbar durch OVAM)
- B: Primärfeld 3 · Normtyp: Grundnorm/Begriffsnorm (Art. 2.2.2 determiniert, ob ein Material die Abfalleigenschaft verliert — Gatekeeper für jede nachfolgende Verwertungs-/Reuse-Handlung)
- C: materialübergreifend
- D: RVO (Besluit van de Vlaamse Regering)
- E: Abfallstatus, Aufbereitung/Prüfung · E-Wirkung: Abfallstatus = vermeidet (Ziel der Norm ist gerade das Verlassen des Abfallstatus); Aufbereitung/Prüfung = erzwingt (Grondstofverklaring erfordert Analysegegevens erkannter Labore, Art. 2.4.2.2)
- F1 (E3): ermöglichend — legt fest, dass Materialien bei "oordeelkundig" Gebrauch und Erfüllung aller genannten Voraussetzungen nicht als Abfallstoffe gelten, und schafft mit der Grondstofverklaring ein formelles Verwaltungsverfahren (Antrag bei Vlaamse overheid/OVAM), mit dem ein Material amtlich als Nicht-Abfall bestätigt wird — ein zentraler Ermöglichungsmechanismus für die Wiederverwendung von Rückbaumaterial.
- F2 (E3): bedingend — die Grondstofverklaring verlangt Identifikation des Grondstoffenproducent, Prozessbeschreibung, Laboranalysen und eine konkrete Zweckbestimmung; sie ist zudem widerruflich bei Regeländerung, geänderter Milieucontext oder oneigenlijk gebruik (Art. 2.4.3.1) — für heterogenes, einzelfallartiges Rückbaumaterial (im Gegensatz zu standardisierten Sekundärrohstoffströmen) ist der Verwaltungsaufwand pro Charge/Charge-Typ ein struktureller Anwendungsfaktor, dessen praktische Häufigkeit bei Bauteil-Reuse (vs. Materialstrom-Recycling) NICHT primärquellenbasiert belegt ist.
- G: Dokumentenlage, Probenahme/Materialprüfung, Erklärung Dritter (explizit=E1, Art. 2.4.2.2: Analysegegevens erkende laboratoria, Beschreibung Produktionsprozess)
- Kernaussage: VLAREMA Art. 2.2.2 setzt allgemeine Voraussetzungen für das Ende der Abfallphase; werden alle genannten Voraussetzungen erfüllt, gilt das Material bei sachgerechter ("oordeelkundige") Verwendung nicht mehr als Abfallstoff. Art. 2.4.1.1 ff. operationalisiert dies über die Grondstofverklaring — eine formelle behördliche Feststellung, dass ein konkretes Material Rohstoffstatus hat, mit Antragspflichten (Herkunft, Prozess, Laboranalyse, Verwendungszweck) und Widerrufsmöglichkeit bei geänderten Umständen.
- Wortlautbeleg (Originalsprache): "materialen waarvan alle vermelde voorwaarden zijn ingevuld [mogen niet] bij onoordeelkundig gebruik als afvalstoffen [worden beschouwd]" (Art. 2.2.2, paraphrasiert-zusammengefasstes Fetch-Ergebnis; direktes Vollzitat in dieser Sitzung technisch nicht extrahierbar — s. Einschränkung unten)
- Beleg-Quelle: B1 (amtliche Konsolidierung/Auszug in dieser Sitzung per WebFetch eingesehen; das Fetch-Ergebnis lieferte inhaltliche Zusammenfassung mit Teilzitaten, kein durchgehendes Vollzitat der Artikel — daher B1 statt B0) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Besluit selbst ist Bindungsakt, gestützt auf Ermächtigung im Materialendecreet, REG-BE-3-007)
- Quelle: Tier 1 · https://codex.vlaanderen.be/PrintDocument.ashx?id=1021756&geannoteerd=false · Fassung(as-amended) VLAREMA 9 (ab 2024-07-01), konsolidierte Fassung zum 2026-08-11 · Zugriff 2026-08-11
- Status: in Kraft · seit 2012, laufend novelliert
- Sub-Ebene: Stichprobe [Vlaams Gewest] / nicht erhoben [entfällt]
- Relationen: konkretisiert REG-BE-3-007 (hergebruik-Begriff); determiniert Anwendbarkeit von nachgeordneten Verwertungsvorschriften (nicht im Detail extrahiert)
- Konfidenz: abgeleitet (Grundstruktur/Existenz gesichert; exakter durchgehender Artikelwortlaut nicht vollständig als Zitat gesichert — Einschränkung des WebFetch-Extraktionswerkzeugs, keine inhaltliche Unsicherheit über die Kernaussage)

---

### REG-BE-3-009 · VLAREMA Art. 4.3.3 — Verpflichtungsschwellen Sloopopvolgingsplan (SOP)
- Titel: VLAREMA, Art. 4.3.3 (Toepassingsgebied sloopopvolgingsplan)
- Fundstelle: Art. 4.3.3, § 1
- A: sub-national (Vlaams Gewest) · Downstream-Verifikationsstatus: verifiziert in [Vlaams Gewest]
- B: Primärfeld 3 · Nebenfelder: 2 (Verknüpfung mit Omgevingsvergunning)
- C: materialübergreifend
- D: RVO
- E: Bestandserkundung · E-Wirkung: erzwingt (die Norm schreibt die Erkundungs-/Planungspflicht ab einer Volumenschwelle zwingend vor)
- F1 (E3): ermöglichend UND bedingend zugleich (Bezugsgegenstand-Doppelnatur) — Bezugsgegenstand "Systematische Vor-Abbruch-Erfassung": ermöglichend, da sie erstmals flächendeckend ein strukturiertes Materialinventar vor Abbruch erzwingt, das die Grundlage für spätere Reuse-Entscheidungen liefert; Bezugsgegenstand "Anwendungsschwelle": bedingend, da die Pflicht nur oberhalb der genannten Volumenschwellen greift und kleinere Vorhaben (insb. Einfamilienhäuser, die ausdrücklich ausgenommen sind) davon strukturell unberührt bleiben.
- F2 (E3): ermöglichend — schafft mit dem SOP-Erfordernis den administrativen Ankerpunkt, an dem Reuse-Potential vor Abbruch überhaupt erst systematisch erfasst wird (Vorstufe zu REG-BE-3-010).
- G: Dokumentenlage (explizit=E1, SOP als schriftliches Planungsdokument)
- Kernaussage: Art. 4.3.3 § 1 VLAREMA legt fest, ab welchem Bauvolumen ein Sloopopvolgingsplan (SOP) verpflichtend der Omgevingsvergunningsaanvraag beizufügen ist: bei nicht-residentiellen Gebäuden > 1.000 m³, bei residentiellen Gebäuden (außer Einfamilienhäuser) > 5.000 m³, sowie bei Infrastrukturarbeiten (Neubau wie Instandhaltung) > 250 m³. Das SOP ist damit das zentrale Vor-Abbruch-Erfassungsinstrument, das die Grundlage für die spätere Sloopattest-Traceability (REG-BE-3-010) legt.
- Wortlautbeleg (Originalsprache): "sloop-, renovatie- of ontmantelingswerken bij gebouwen waarvoor een omgevingsvergunning vereist is en waarvan het totale bouwvolume groter is dan 1000 m³" (niet-residentieel); "…groter is dan 5000 m³" (residentieel, uitgezonderd eengezinswoningen); "sloop-, renovatie- of ontmantelingswerken in het kader van infrastructuurwerken … en waarvan het volume groter is dan 250 m³"
- Beleg-Quelle: B1 (Artikeltext über Kustcodex-Rechtsdatenbank — ein von einer flämischen Gemeinde/Interkommunale betriebenes konsolidiertes Rechtsportal, kein Vlaamse-Codex-Erstportal — in dieser Sitzung per WebFetch eingesehen, Wortlaut plausibel und konsistent mit Sekundärquellen) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Besluit selbst ist Bindungsakt)
- Quelle: Tier 1 (Rechtsdatenbank-Mirror, amtsnah) · https://www.kustcodex.be/kustcodex-consult/plainWettekstServlet?wettekstId=53806&lang=nl · Ergänzend Tier 3 (Bestätigung Schwellenwerte/Kontext) https://www.tracimat.be/wetgeving/ · Fassung(as-amended) VLAREMA 9 (2024-07-01) · Zugriff 2026-08-11
- Status: in Kraft · verpflichtend seit 2022-07-01 (laut Tracimat-Sekundärquelle; amtliches Inkrafttretensdatum des konkreten VLAREMA-Änderungsbesluits nicht separat primärverifiziert)
- Sub-Ebene: Stichprobe [Vlaams Gewest] / nicht erhoben [entfällt]
- Relationen: konkretisiert REG-BE-3-007/-008; wird kombiniert mit REG-BE-2-006 (Omgevingsvergunningsverfahren); setzt REG-BE-3-010 voraus (SOP ist Vorstufe zum Sloopattest-Traceringsproces)
- Konfidenz: abgeleitet (Schwellenwerte primärnah belegt über Rechtsdatenbank-Mirror, nicht direkt über Vlaamse Codex/ejustice-Erstportal; inhaltlich konsistent mit unabhängigen Sekundärquellen)

---

### REG-BE-3-010 · VLAREMA Art. 4.3.5 — Sloopattest über erkende sloopbeheerorganisatie
- Titel: VLAREMA, Art. 4.3.5 (Sloopattest, Traceerbaarheid via erkende sloopbeheerorganisatie)
- Fundstelle: Art. 4.3.5, § 1–2
- A: sub-national (Vlaams Gewest) · Downstream-Verifikationsstatus: verifiziert in [Vlaams Gewest]
- B: Primärfeld 3
- C: materialübergreifend
- D: RVO
- E: Rückbau/Sicherung, Aufbereitung/Prüfung · E-Wirkung: erzwingt (Traceerbaarheidssysteem ist verpflichtend, kein optionaler Weg für erfasste Projekte)
- F1 (E3): ermöglichend — schafft mit dem Sloopattest ein amtlich anerkanntes Traceability-Zertifikat, das die getrennte Sammlung und rückverfolgbare, kontrollierte Verarbeitung von Sloopmateriaal (insb. der Puinfractie) dokumentiert; ein solches Zertifikat kann als Vertrauens-/Qualitätsnachweis für nachfolgende Verwertung/Wiederverwendung dienen.
- F2 (E3): bedingend — das Sloopattest wird nur für die Puinfractie (mineralischer Abbruch, Beton/Mauerwerk) im Rahmen der von Art. 4.3.3 erfassten Vorhaben und nur über EINE erkannte sloopbeheerorganisatie (Tracimat, faktisch Monopolstellung, s. REG-BE-3-011) ausgestellt; für andere Materialfraktionen (Holz, Metall, Bauteile zur direkten Wiederverwendung außerhalb der Puinfractie-Logik) ist die Reichweite des Sloopattest-Mechanismus im gelesenen Text NICHT ausdrücklich geregelt — ob/wie Bauteil-Reuse (im Unterschied zu Materialstrom-Recycling) durch dieses Instrument erfasst wird, bleibt offen.
- G: Dokumentenlage, Erklärung Dritter, zerstörungsfreie Prüfung (explizit=E1 für Erklärung Dritter/Verifikation durch sloopbeheerorganisatie; zerstörungsfreie Prüfung inferiert=E3 aus der allgemeinen Verificatie-Prozessbeschreibung, s. REG-BE-3-011)
- Kernaussage: Art. 4.3.5 VLAREMA regelt, dass für getrennt eingesammeltes Bouw- en sloopmateriaal ein Sloopattest ausgestellt werden kann, das die getrennte Einsammlung und die Rückverfolgbarkeit der Herkunft bis zur kontrollierten Verarbeitung bestätigt. Nach § 2 kann eine erkende sloopbeheerorganisatie ein Sloopattest für die Puinfractie von Sloopmateriaal ausstellen, das aus den in Art. 4.3.3 § 1 genannten (SOP-pflichtigen) Aktivitäten stammt und für das eine Verwerkingstoelating erteilt wurde.
- Wortlautbeleg (Originalsprache): "Overeenkomstig artikel 4.3.5 § 2 VLAREMA kan er door een erkende sloopbeheerorganisatie een sloopattest worden afgeleverd voor de puinfractie van sloopmateriaal afkomstig van de activiteiten zoals vermeld in artikel 4.3.3 § 1 waarvoor een verwerkingstoelating is afgeleverd" (Sekundärquellen-Paraphrase mit eingebettetem Teilzitat; amtlicher Artikel-Volltext in dieser Sitzung nicht direkt aus Vlaamse Codex/ejustice extrahiert — Einschränkung s. Beleg-Quelle)
- Beleg-Quelle: B3 (Inhalt über konvergente Fachportal-/Anwaltssekundärquellen — GISC, Legalnews/Adhemar Advocaten, Bureau D — rekonstruiert, KEIN direkter Volltextzugriff auf den amtlichen VLAREMA-Artikeltext in dieser Sitzung gelungen) · Zugänglichkeit: frei-primär (Norm selbst frei zugänglich, nur in dieser Sitzung technisch nicht direkt extrahiert) · Bindungsakt: Bindungsmechanismus existiert, Listung im Einzelfall nicht verifiziert — VLAREMA-Artikeltext selbst müsste für eine B0/B1-Einstufung direkt über Vlaamse Codex geöffnet werden (für W2-Folgesitzung vorgemerkt)
- Quelle: Tier 3 (Suchhinweis, aggregiert von KI-Fetch aus mehreren Fachportalen) https://gisc.be/sloopopvolgingsplan-wat-is-het-en-waarom-is-het-belangrijk/ , https://legalnews.be/publiek-recht/milieu-en-stedenbouwrecht/sloopinventaris-wordt-sloopopvolgingsplan-adhemar-advocaten/ · Tier 1 (Verweis, nicht direkt gelesen) Vlaamse Codex-Fundstelle wie REG-BE-3-008 · Fassung(as-amended) VLAREMA 9 (2024-07-01) · Zugriff 2026-08-11
- Status: in Kraft · verpflichtend seit 2022-07-01 (Sekundärquelle)
- Sub-Ebene: Stichprobe [Vlaams Gewest] / nicht erhoben [entfällt]
- Relationen: konkretisiert REG-BE-3-009 (SOP als Vorstufe); wird kombiniert mit REG-BE-3-011 (Tracimat als Vollzugsträger)
- Konfidenz: abgeleitet — WICHTIGER HINWEIS FÜR W2-FOLGESITZUNG/W4: Dieses Objekt darf laut Bindungsketten-/Belegregeln NICHT ohne erneuten Versuch der direkten VLAREMA-Volltexteinsicht (Vlaamse Codex, nicht Sekundärportale) auf B0/B1 hochgestuft in die Synthese (W4) übernommen werden

---

### REG-BE-3-011 · Erkenning Tracimat vzw als sloopbeheerorganisatie
- Titel: Erkenning van Tracimat vzw als sloopbeheerorganisatie (Ministerieel Erkenningsbesluit, Grundlage VLAREMA Art. 4.3.5), operationalisiert über "Standaardprocedure" Tracimat
- Fundstelle: Erkenningsbesluit selbst nicht identifiziert; operative Regeln über Tracimat-Website (Standaardprocedure, Wetgeving-Seite)
- A: sub-national (Vlaams Gewest) · Downstream-Verifikationsstatus: nicht geprüft (Erkenningsbesluit selbst nicht aufgefunden, daher keine Prüfung der genauen Bindungsform möglich)
- B: Primärfeld 3
- C: materialübergreifend
- D: Verwaltungsvorschrift (vorläufig — konkreter Erkenningsbesluit-Typ nicht verifiziert)
- E: Rückbau/Sicherung, Aufbereitung/Prüfung, Betrieb/Dokumentation
- F1 (E3): ermöglichend — die Erkennung EINER zentralen sloopbeheerorganisatie (faktisch Tracimat als bislang einzige erkannte Organisation) schafft einen einheitlichen, landesweit (VL) standardisierten Traceability-Prozess statt fragmentierter Einzellösungen; das senkt tendenziell die Transaktionskosten für Rückbauunternehmen, die Reuse-/Recycling-Nachweise erbringen müssen.
- F2 (E3): bedingend — die faktische Monopolstellung Tracimats (einzige erkannte sloopbeheerorganisatie) bedeutet, dass der gesamte VL-Sloopopvolgings-Vollzug von der Kapazität/den Verfahrensregeln einer einzigen privaten vzw abhängt; ob dies Reuse spezifisch fördert oder (als reines Recycling-Traceability-System für Puinfractie) primär Materialstrom- statt Bauteil-Wiederverwendung adressiert, ist NICHT primärquellenbasiert geklärt (s. Einschränkung bei REG-BE-3-010).
- G: Erklärung Dritter, Dokumentenlage (explizit=E1 für den Grundprozess: DAI, Sloopinventaris, Verificatie, Sloopattest — Kaskaden-Notation: [1] Destructieve asbestinventaris (immer, vorgelagert) → [2] Sloopinventaris (Materialerfassung) → [3] Verificatie durch Tracimat → [4] Sloopattest (bei Konformität))
- Kernaussage: Tracimat vzw wurde 2017 (laut Sekundärdarstellung, Zeitpunkt nicht amtlich verifiziert) von der zuständigen vlaamse Minister als sloopbeheerorganisatie im Sinne von VLAREMA Art. 4.3.5 anerkannt und ist bislang die einzige erkannte Organisation. Der operative Traceerbaarheidsproces umfasst vier Schritte: destructieve asbestinventaris (DAI) als SOP-Anhang, Sloopinventaris (Materialerfassung), Verificatie (Konformitätsprüfung durch Tracimat) und abschließend das Sloopattest. Der eigentliche Erkennungsrechtsakt (Ministerieel Besluit) wurde in keiner der bisherigen Rechercherunden im Volltext gefunden.
- Wortlautbeleg (Originalsprache): "Tracimat is als sloopbeheerorganisatie erkend door de bevoegde Vlaamse minister"; "de opmaak van een sloopopvolgingsplan (SOP), conform de Standaardprocedure"; "het volledige doorlopen van het traceerbaarheidssysteem t.e.m. het behalen van een sloopattest"
- Beleg-Quelle: B2 (Sekundärdarstellung der Vollzugsorganisation selbst eingesehen — als Erklärung Dritter/Vollzugsträger-Selbstauskunft von relevanter, aber nicht amtlich-primärer Qualität; Erkenningsbesluit selbst nicht gefunden) · Zugänglichkeit: frei-primär, aber Erkenningsbesluit selbst nicht identifiziert · Bindungsakt: Bindungsmechanismus existiert (VLAREMA Art. 4.3.5, REG-BE-3-010), konkreter Erkenningsakt (Ministerieel Besluit) nicht identifiziert — ausstehende Prüfung: Belgisch Staatsblad-Suche nach Erkenningsbesluit Tracimat 2017. **Nachverifikations-Pass (selbes Zugriffsdatum):** ein WebSearch-Snippet nannte „Ministerieel Besluit 24.08.2017 / publiziert Belgisch Staatsblad 29.09.2017", konnte aber durch direkten WebFetch der genannten Quellseiten NICHT bestätigt werden (Datumsangabe dort nicht auffindbar) — bewusst NICHT als Faktum übernommen. Zusätzlich fand sich ein Sekundärquellen-Widerspruch zur Rechtsgrundlage: eine Quelle (Marlex/Legalnews 2018-01-16) verortet die Tracimat-Erkennung unter „Artikel 4.3.6 ff. VLAREMA" statt Art. 4.3.5 — beide Verortungen sind sekundärquellenbasiert (B2/B3), keine davon primärtextbestätigt; für W4 vor Übernahme zwingend am VLAREMA-Primärtext (Hoofdstuk 4, Afdeling 4.3, Onderafdeling Erkenning sloopbeheerorganisaties) zu klären, welcher Artikel tatsächlich die Erkennungsgrundlage bildet.
- Quelle: Tier 1 (Vollzugsorganisation, amtlich beliehen) https://www.tracimat.be/wetgeving/ · Fassung(as-amended) unbekannt (Erkennung ca. 2017) · Zugriff 2026-08-11
- Status: in Kraft (institutionell aktiv) · Erkennung ca. 2017-08 (unverifiziert)
- Sub-Ebene: Stichprobe [Vlaams Gewest] / nicht erhoben [entfällt]
- Relationen: setzt um REG-BE-3-010 (VLAREMA Art. 4.3.5); wird kombiniert mit REG-BE-2-006 (Omgevingsvergunningsverfahren, SOP als Antragsunterlage)
- Konfidenz: abgeleitet (institutionelle Funktion/Prozess gesichert über Selbstdarstellung; formeller Erkenningsrechtsakt nicht primärverifiziert — Lücke für W2-Folgesitzung)

---

### REG-BE-4-012 · KB Basisnormen brandveiligheid (1994) — Neubau-Beschränkung als reuse-struktureller Faktor
- Titel: Koninklijk Besluit van 7 juli 1994 tot vaststelling van de basisnormen voor de preventie van brand en ontploffing waaraan de nieuwe gebouwen moeten voldoen ("KB Basisnormen")
- Fundstelle: Gesamt-KB; Anwendungsbereichsbeschränkung laut Novelle 2003-04-04 (Volltext in dieser Sitzung technisch nicht extrahierbar, s. Einschränkung)
- A: national · Downstream-Verifikationsstatus: entfällt (föderales KB, gilt unmittelbar auch in Vlaanderen)
- B: Primärfeld 4
- C: materialübergreifend
- D: RVO
- E: Planung/Nachweis · E-Wirkung: vermeidet (Bezugsgegenstand "Renovatie/Umbau" — die Norm zieht ihre reuse-relevante Wirkung gerade aus dem Nicht-Erreichen des Anwendungsbereichs für Bestandsmaßnahmen)
- F1 (E3): ermöglichend (Bezugsgegenstand "Renovatie/Umbau") — der Anwendungsbereich ist seit der Novelle vom 4.4.2003 ausdrücklich auf Neubau beschränkt; Wiedereinbau von Bauteilen im Bestand (Renovation) löst damit tendenziell nicht denselben Brandschutz-Neubaustandard aus wie ein Neubau, was Reuse im Bestand strukturell erleichtert. F1 (Bezugsgegenstand "Neubau mit Reuse-Bauteilen") — schweigend/hemmend, da für den Fall, dass ein Vorhaben als "Neubau" (statt Renovation) eingestuft wird, dieselben Neubau-Brandschutzanforderungen wie bei Neuprodukten gelten und die Abgrenzung Neubau/Renovation selbst zur Streitfrage werden kann.
- F2 (E3): unklar — die praktische Abgrenzungspraxis (wann gilt eine Maßnahme als "Neubau" vs. "Renovation" im Sinne dieses KB) und ihre konkrete Auswirkung auf Reuse-Vorhaben in Vlaanderen ist NICHT primärquellenbasiert belegt (nur Sekundärquelle Buildwise).
- G: Erklärung Dritter (explizit=E1, laut Sekundärquelle: Novelle 2022 vereinfacht CE-Kennzeichnungsnachweise bei Dachbelägen alternativ über Klassifizierungsberichte — im Primärtext in dieser Sitzung nicht direkt verifiziert)
- Kernaussage: Das KB Basisnormen (7.7.1994, Rechtsgrundlage Wet 30.7.1979, letzte Novelle KB 20.5.2022) legt bundesweit geltende Brandschutz-Basisanforderungen fest, deren Anwendungsbereich seit der Novelle vom 4.4.2003 ausdrücklich auf Neubauten beschränkt ist — Renovation/Umbau ist grundsätzlich nicht erfasst (Abweichungen im Einzelfall über FOD Binnenlandse Zaken möglich). Für Vlaanderen bedeutet dies, dass der Wiedereinbau von Bauteilen im Bestand tendenziell nicht denselben Neubau-Brandschutzstandard triggert — ein potenzieller Ermöglicher für Bauteil-Reuse im Umbaukontext, zugleich Quelle für Abgrenzungsstreit an der Neubau/Renovation-Grenze.
- Wortlautbeleg (Originalsprache): kein direktes Vollzitat möglich — der amtliche PDF-Primärtext (4,6 MB, komprimierter Binärstrom) konnte vom eingesetzten WebFetch-Werkzeug in dieser Sitzung nicht in Klartext konvertiert werden (technische Extraktionsgrenze, kein Paywall-Grund); Inhaltsangabe stützt sich auf die bereits in der Vorsitzung dokumentierte Buildwise-Fachzusammenfassung
- Beleg-Quelle: B2 (Herkunft: Quellenkarte-Vorsitzung — PDF-Primärtext + Buildwise-Sekundärzusammenfassung eingesehen, Novelle 2022 nur über Sekundärquelle; in dieser Sitzung PDF-Direktzugriff erneut versucht, technisch gescheitert) · Zugänglichkeit: frei-primär (PDF direkt abrufbar, aber Textextraktion in dieser Sitzung technisch nicht gelungen) · Bindungsakt: entfällt (KB selbst ist Bindungsakt; verweist laut Vorsitzungs-Befund seit Novelle 2022 nicht mehr auf die private Norm NBN B 61-001 — Bindungskette dort aufgelöst statt neu geschaffen)
- Quelle: Tier 1 · https://civieleveiligheid.be/sites/default/files/1994-07-07kb_basisnormen.pdf · Tier 1 (Fachzusammenfassung, Regierungsnähe) https://www.buildwise.be/nl/normen-en-regelgeving/buildwise-normalisatie-certificering-normen-antenne-brandpreventie/belgische-en-europese-reglementering-en-normalisatie/het-koninklijk-besluit-basisnormen/ · Fassung(as-amended) 2022-06-23 (Novelle, in Kraft 2022-07-01) · Zugriff 2026-08-11
- Status: in Kraft · 1994-07-07 (Basis-KB), letzte Novelle 2022-05-20
- Sub-Ebene: entfällt (A=national)
- Relationen: steht in sachlichem Zusammenhang mit REG-BE-2-006 (Omgevingsvergunning, in deren Rahmen Brandschutznachweise verlangt werden); Abgrenzungsfrage Neubau/Renovation ungeklärt (offene W2/W3b-Anschlussfrage)
- Konfidenz: abgeleitet (Existenz/Anwendungsbereichsbeschränkung gesichert über konvergente Quellen; exakter Wortlaut der Ausschlussklausel nicht direkt zitiert)

---

### REG-BE-4-013 · Vlaams Energiebesluit, Titel IX — EPB-Trennung Nieuwbouw/Renovatie
- Titel: Besluit van de Vlaamse Regering van 19 november 2010 houdende algemene bepalingen over het energiebeleid ("Energiebesluit"), Titel IX — Energieprestatie van gebouwen
- Fundstelle: Titel IX, Hoofdstuk I, Afdeling II (EPB-eisen bij nieuwbouw) und Afdeling III (EPB-eisen bij renovatie en functiewijziging), insb. Art. 1.1.1 § 2 (Definition "ingrijpende energetische renovatie")
- A: sub-national (Vlaams Gewest) · Downstream-Verifikationsstatus: verifiziert in [Vlaams Gewest]
- B: Primärfeld 4
- C: materialübergreifend
- D: RVO
- E: Planung/Nachweis
- F1 (E3): bedingend, mit klarer Bezugsgegenstand-Differenzierung — Bezugsgegenstand "Nieuwbouw": die EPB-Anforderungen (Afdeling II: thermische Isolatie, Ventilatie, E-peil, S-peil, hernieuwbare energie, technische installaties) gelten vollumfänglich für Neubauten. Bezugsgegenstand "Renovatie/functiewijziging" (Afdeling III): eigenständiger, weniger strenger Anforderungsrahmen, der erst ab der Schwelle "ingrijpende energetische renovatie" (beschermd volume > 800 m³ UND ≥ 75 % der Scheidingskonstruktionen werden isoliert ersetzt) die Neubau-nahen Anforderungen triggert — unterhalb dieser Schwelle gelten mildere bauteilbezogene Anforderungen.
- F2 (E3): ermöglichend — die ausdrückliche Trennung Nieuwbouw/Renovatie mit einer relativ hohen Doppelschwelle (Volumen UND Isolationsgrad) für "ingrijpende energetische renovatie" bedeutet, dass viele Umbau-/Reuse-Vorhaben mit punktuellem Bauteilaustausch unterhalb dieser Schwelle bleiben und damit nicht die vollen Neubau-EPB-Anforderungen erfüllen müssen — strukturell günstig für die Wiederverwendung einzelner, energetisch nicht auf Neubauniveau ertüchtigter Bauteile.
- G: Dokumentenlage, rechnerischer Nachweis (explizit=E1, EPB-Berechnungspflicht als Kernmechanismus von Titel IX; Kapitel II regelt Energieprestatiecertificaten als Nachweisdokument)
- Kernaussage: Das Vlaams Energiebesluit, Titel IX (in Kraft seit 2011-01-01, letzte identifizierte materielle Konsolidierung 2023-06-16/2023-02-06) trennt in Hoofdstuk I ausdrücklich EPB-Anforderungen für Neubau (Afdeling II) von denen für Renovation/Funktionsänderung (Afdeling III). Zentral für Reuse ist die in Art. 1.1.1 § 2 definierte Schwelle "ingrijpende energetische renovatie" (beschermd volume > 800 m³ und Isolierung von ≥ 75 % der Scheidingskonstructies) — nur oberhalb dieser Doppelschwelle greifen Neubau-nahe Anforderungen; kleinere/punktuelle Umbaumaßnahmen mit wiederverwendeten Bauteilen bleiben typischerweise darunter.
- Wortlautbeleg (Originalsprache): "een functiewijziging met beschermd volume groter dan 800 m³ of renovatie waarbij ≥75% scheidingsconstructies worden geïsoleerd" (Art. 1.1.1 § 2, Definitiesectie — Fetch-Ergebnis als Wortlautnähe wiedergegeben, exaktes Vollzitat mit Artikelnummer in dieser Sitzung nicht separat gegengeprüft)
- Beleg-Quelle: B1 (in dieser Sitzung per WebFetch erneut geöffnet, Struktur und Definition bestätigt; exakte Artikelnummer der Definition nicht durch zusätzlichen Gegencheck verifiziert) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Besluit selbst ist Bindungsakt)
- Quelle: Tier 1 · https://codex.vlaanderen.be/portals/codex/documenten/1019755.html · Fassung(as-amended) 2023-06-16/2023-02-06 (letzte identifizierte Konsolidierung; exakter 2026-Stand nicht separat bestätigt) · Zugriff 2026-08-11
- Status: in Kraft · seit 2011-01-01
- Sub-Ebene: Stichprobe [Vlaams Gewest] / nicht erhoben [entfällt]
- Relationen: ergänzt REG-BE-4-012 (Brandschutz-Neubau/Renovatie-Trennung — strukturell paralleles Muster in einem anderen Schutzzielfeld); steht in Bezug zu REG-EU-4-001 (EPBD-Neufassung GWP-Berechnungspflicht, Umsetzungsstand in VL nicht geprüft)
- Konfidenz: abgeleitet (Struktur gesichert; exakter 2026-Konsolidierungsstand und Artikelnummer der Kernschwelle nicht abschließend gegengeprüft)

---

### REG-BE-5a-014 · Wet overheidsopdrachten 17.06.2016 — Umweltkriterien als technische Spezifikation
- Titel: Wet van 17 juni 2016 inzake overheidsopdrachten
- Fundstelle: Art. 2, 44° lit. a/b (definities technische specificaties); Art. 81 § 2, 3° (gunningscriteria, beste prijs-kwaliteitsverhouding)
- A: national · Downstream-Verifikationsstatus: entfällt (föderales Gesetz, gilt unmittelbar für Vergaben aller Ebenen einschließlich vlaamser Behörden/Gemeinden)
- B: Primärfeld 5a
- C: materialübergreifend
- D: Gesetz
- E: Planung/Nachweis
- F1 (E3): bedingend — Art. 2, 44° nennt "het niveau van milieuvriendelijkheid en klimaatprestaties" ausdrücklich als möglichen Bestandteil technischer Spezifikationen für Bauleistungen, Lieferungen und Dienstleistungen; ein eigenständiges, explizit benanntes Zuschlags- oder Ausschlusskriterium "Kreislaufwirtschaft/hergebruik" wurde im gelesenen Textausschnitt NICHT gefunden — Umweltaspekte laufen über die allgemeine Kategorie technischer Spezifikationen/Zuschlagskriterien (Art. 81 § 2, 3° "beste prijs-kwaliteitsverhouding"), nicht über ein zirkularitätsspezifisches Sonderkriterium.
- F2 (E3): unklar — ob und wie oft öffentliche Auftraggeber in Vlaanderen "milieuvriendelijkheid" konkret zur Verankerung von Reuse-Anforderungen (z. B. Mindestanteil wiederverwendeter Bauteile) in Ausschreibungen nutzen, ist NICHT primärquellenbasiert belegt.
- G: Dokumentenlage (explizit=E1, technische Spezifikationen als Ausschreibungsunterlage)
- Kernaussage: Das föderale Vergabegesetz vom 17.6.2016 (Umsetzung RL 2014/24/EU + 2014/25/EU) erlaubt über Art. 2, 44° die Einbeziehung von Umweltfreundlichkeits-/Klimaleistungsniveaus in die technischen Spezifikationen einer Ausschreibung sowie über Art. 81 § 2, 3° eine lebenszyklusorientierte Zuschlagsbewertung ("beste prijs-kwaliteitsverhouding"). Ein eigenständiges, benanntes Kreislaufwirtschafts-/Reuse-Kriterium enthält das Gesetz nicht; entsprechende Anforderungen müssten von öffentlichen Auftraggebern (auch in Vlaanderen) über die allgemeinen Umweltkategorien selbst formuliert werden.
- Wortlautbeleg (Originalsprache): "het niveau van milieuvriendelijkheid en klimaatprestaties" (Art. 2, 44° lit. a/b)
- Beleg-Quelle: B1 (in dieser Sitzung per WebFetch erneut geöffnet, Art. 2/44° und Grundstruktur Art. 81 bestätigt; Art. 81 im Fetch-Ergebnis vor vollständiger Zuschlagskriterien-Aufzählung abgeschnitten) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Gesetz selbst ist Bindungsakt)
- Quelle: Tier 1 · https://etaamb.openjustice.be/nl/wet-van-17-juni-2016_n2016021053.html · amtlich: https://www.ejustice.just.fgov.be/cgi_loi/change_lg.pl?language=nl&la=N&table_name=wet&cn=2016061719 · Fassung(as-amended) 2016-07-14 (Publikation), Konsolidierungsstand 2026-08-11 nicht vollständig verifiziert · Zugriff 2026-08-11
- Status: in Kraft · seit 2017
- Sub-Ebene: entfällt (A=national)
- Relationen: konkretisiert wird durch REG-BE-5a-015 (Ausführungs-KB); Anwendungsfeld für vlaamse öffentliche Auftraggeber identisch zu Bundesebene
- Konfidenz: abgeleitet (Grundstruktur gesichert; vollständige Zuschlagskriterien-Aufzählung nicht bis zum Ende gelesen)

---

### REG-BE-5a-015 · KB 18 april 2017 — Ausführungs-KB Plaatsing overheidsopdrachten klassieke sectoren
- Titel: Koninklijk Besluit van 18 april 2017 plaatsing overheidsopdrachten in de klassieke sectoren
- Fundstelle: Gesamt-KB (Artikelebene zu konkreten Vergabekriterien in dieser Sitzung nicht extrahiert)
- A: national · Downstream-Verifikationsstatus: entfällt (föderales KB)
- B: Primärfeld 5a
- C: materialübergreifend
- D: RVO
- E: Planung/Nachweis
- F1 (E3): bedingend — regelt die Verfahrensdetails zur Anwendung von Wet 17.6.2016 (uniformes Europäisches Vergabedokument, elektronische Kommunikationsmittel/-verfahren); ob/wie zirkuläre bzw. Reuse-bezogene Kriterien auf dieser Verfahrensebene konkret verankert sind, wurde in dieser Sitzung NICHT geprüft.
- F2 (E3): unklar (Artikelebene nicht ausgewertet)
- G: Dokumentenlage (inferiert=E3, aus allgemeiner Verfahrensregelungslogik)
- Kernaussage: Das KB vom 18.4.2017 (Numac 2017020322) ist das föderale Ausführungs-KB zur Wet overheidsopdrachten 2016 für die klassischen Sektoren und ersetzt weitgehend das KB vom 15.7.2011. Es setzt Teile der RL 2014/24/EU um (u. a. uniformes Europäisches Vergabedokument, elektronische Verfahren). Konkrete Bestimmungen zu zirkulären/Reuse-Zuschlagskriterien auf Artikelebene wurden in dieser Sitzung nicht identifiziert — offene Anschlussfrage.
- Wortlautbeleg (Originalsprache): kein Wortlautzitat verfügbar (Titel-/Fundstellenebene, Artikeltext nicht gelesen)
- Beleg-Quelle: B1 (Herkunft: Quellenkarte-Vorsitzung — Fundstelle/Regelungszweck über Etaamb-Zusammenfassung bestätigt, Artikelebene ungelesen; in dieser Sitzung nicht erneut geöffnet) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (KB selbst ist Bindungsakt)
- Quelle: Tier 1 · https://etaamb.openjustice.be/nl/koninklijk-besluit-van-18-april-2017_n2017020322.html · Justel: https://www.ejustice.just.fgov.be/cgi_loi/change_lg.pl?cn=2017041810&la=N&language=nl&table_name=wet · Fassung(as-amended) 2017-05-09 (Publikation) · Zugriff (Vorsitzung) 2026-08-11
- Status: in Kraft · 2017-04-18 (ersetzt weitgehend KB 15.07.2011)
- Sub-Ebene: entfällt (A=national)
- Relationen: konkretisiert REG-BE-5a-014; ersetzt KB 15.07.2011 (nicht als eigenes Objekt geführt)
- Konfidenz: abgeleitet (Existenz/Supersession gesichert; Artikelebene zu Reuse-Kriterien nicht geprüft)

---

### REG-BE-5b-016 · Vlaanderen Circulair / OVAM-Subsidieregelingen circulaire economie — Existenznachweis, ungeklärte Rechtsgrundlage
- Titel: Vlaanderen Circulair (Programm) / OVAM-Subsidieregelingen circulaire economie
- Fundstelle: nicht identifiziert (kein konkreter Subsidiebesluit primärquellenbasiert geprüft)
- A: sub-national (Vlaams Gewest) · Downstream-Verifikationsstatus: nicht geprüft
- B: Primärfeld 5b
- C: materialübergreifend
- D: Merkblatt (vorläufig — konkrete Rechtsgrundlage einzelner Subsidiemaßnahmen nicht identifiziert; Vlaanderen Circulair ist ein Programmlabel, keine einzelne Norm)
- E: entfällt (kein konkreter Vollzugsakt geprüft)
- F1 (E3): unklar
- F2 (E3): unklar
- G: entfällt
- Kernaussage: Dieses Objekt ist bewusst als reiner, ehrlich markierter Existenznachweis geführt: "Vlaanderen Circulair" ist ein von OVAM koordiniertes Programm-/Netzwerklabel für Kreislaufwirtschaftsförderung in Vlaanderen; es wurde in dieser Sitzung NICHT primärquellenbasiert auf konkrete Subsidiebesluiten, Fördersummen oder Reuse-spezifische Programmlinien geprüft. Die Existenz des Programms ist unstrittig, seine rechtliche Verfasstheit (Förderrichtlinie? Ministerbeschluss? einzelne Subsidiebesluiten je Aufruf?) bleibt für die Extraktionsstufe vollständig offen.
- Wortlautbeleg (Originalsprache): kein Zitat möglich (kein Normtext geprüft)
- Beleg-Quelle: B4 nur Existenz-/Katalognachweis · Zugänglichkeit: frei-primär (Portal), aber inhaltlich nicht erhoben · Bindungsakt: nicht geprüft
- Quelle: Tier 3 (nur Suchhinweis) https://vlaanderen-circulair.be/ · Zugriff 2026-08-11
- Status: unklar (Programm aktiv, Rechtsform ungeklärt)
- Sub-Ebene: Stichprobe [Vlaams Gewest — nur Existenz] / nicht erhoben [konkrete Subsidiebesluiten, Fördervolumina, Reuse-spezifische Programmlinien]
- Relationen: keine belastbar feststellbar
- Konfidenz: unklar — für W2-Folgesitzung: vollständig neu zu erheben, bevor als Faktum in W4 übernommen

---

### REG-BE-6-017 · Normalisatiewet 3 april 2003 — föderaler Bindungsketten-Anker für NBN/NBN-EN-Normen
- Titel: Wet van 3 april 2003 betreffende de normalisatie ("Normalisatiewet")
- Fundstelle: Gesamtgesetz; exakte Justel-cn-Nummer trotz dreier Rechercherunden nicht identifiziert
- A: national · Downstream-Verifikationsstatus: entfällt (föderales Gesetz, gilt unmittelbar auch in Vlaanderen; Normung ist nicht regionalisiert)
- B: Primärfeld 6
- C: materialübergreifend
- D: Gesetz
- E: Planung/Nachweis
- F1 (E3): bedingend — trägt als Gründungsgesetz des NBN (Bureau voor Normalisatie, löste Institut Belge de Normalisation/IBN ab) die institutionelle Grundlage, über die einzelne NBN/NBN-EN-Normen erst durch nachgeordnete KBs ("bekrachtiging"-KBs) und fachgesetzliche Bezugnahmen (z. B. KB Basisnormen, REG-BE-4-012) bauordnungsrechtlich verbindlich werden — die klassische Bindungsketten-Konstellation des Projekts.
- F2 (E3): bedingend — belegtes Beispiel einer AUFGELÖSTEN statt neu geschaffenen Bindungskette: Die Novelle KB 20.5.2022 zum KB Basisnormen entfernte den Verweis auf die private Norm NBN B 61-001 und integrierte die Anforderungen direkt in das KB — d. h. hier wurde punktuell der Umweg über eine kostenpflichtige Norm beseitigt, was die Bindungsketten-Prüfung für dieses konkrete Beispiel erübrigt (aber nicht generell für alle NBN-Bezugnahmen im belgischen Baurecht).
- G: Dokumentenlage (inferiert=E3, aus institutioneller Ermächtigungsfunktion)
- Kernaussage: Die Normalisatiewet vom 3.4.2003 begründet das föderale Normungssystem (NBN) und ist damit der institutionelle Ausgangspunkt der Bindungskette, über die kostenpflichtige NBN/NBN-EN-Normen (inkl. Eurocode-nationale Anhänge) in belgisches (auch vlaamses) Baurecht Eingang finden können — regelmäßig über "bekrachtiging"-KBs oder fachgesetzliche Verweisung. Die exakte Justel-Fundstelle (cn-Nummer) konnte trotz mehrfacher gezielter Suche nicht identifiziert werden; Existenz und Datum sind aber über mehrere konvergente KB-Zitate (u. a. KB 22.4.2008 "in uitvoering van de wet van 3 april 2003 betreffende de normalisatie") bestätigt.
- Wortlautbeleg (Originalsprache): kein direktes Vollzitat verfügbar (Primärtext trotz Suche nicht direkt geöffnet); Existenzbeleg über Fremdzitat: "in uitvoering van de wet van 3 april 2003 betreffende de normalisatie" (aus KB 22.4.2008, laut Vorsitzungsrecherche)
- Beleg-Quelle: B2 (Herkunft: Quellenkarte-Vorsitzung — Existenz/Datum durch konvergente KB-Zitate bestätigt, Justel-Primärtext selbst nicht geöffnet; in dieser Sitzung kein erneuter Versuch unternommen wegen Zeitbudget) · Zugänglichkeit: frei-primär (vermutet, nicht verifiziert) · Bindungsakt: dieses Gesetz IST selbst der Bindungsakt-Ursprung für nachgeordnete NBN-Bezugnahmen; für einzelne NBN-Normen im vlaamsen Baukontext bleibt die konkrete Listung im jeweiligen Fachrecht (Einzelfallprüfung) offen
- Quelle: Tier 2 (Sekundärhinweis NBN-Website) https://www.nbn.be/en/using-standards/standards-legislation · Zugriff (Vorsitzung) 2026-08-11
- Status: in Kraft (Gründungsgesetz, Datum unstrittig) · 2003-04-03
- Sub-Ebene: entfällt (A=national)
- Relationen: Bindungsakt-Ursprung für REG-BE-4-012 (KB Basisnormen — dort punktuell aufgelöste Bindungskette zu NBN B 61-001); generischer Bindungsmechanismus für alle NBN/NBN-EN-Bezugnahmen im vlaamsen Baurecht (Einzelfallprüfung je Norm aussteht)
- Konfidenz: abgeleitet (Existenz/Funktion gesichert über konvergente Zitate; Primärtext-cn-Nummer nicht identifiziert)

---

### REG-BE-7-018 · Boek 6 Burgerlijk Wetboek, Art. 6.41–6.55 — außervertragliche Produkthaftung
- Titel: Wet houdende boek 6 "Buitencontractuele aansprakelijkheid" van het Burgerlijk Wetboek, van 7 februari 2024
- Fundstelle: Art. 6.41 (Grundprinzip), Art. 6.42 (Begriff "product"), Art. 6.43 (Begriff "producent"), Art. 6.45 (Begriff "gebrekkig"), Art. 43 (Aufhebung Wet 25.2.1991)
- A: national · Downstream-Verifikationsstatus: entfällt (föderales Gesetzbuch, gilt unmittelbar auch in Vlaanderen)
- B: Primärfeld 7
- C: materialübergreifend
- D: Gesetz
- E: Betrieb/Dokumentation, Inverkehrbringen
- F1 (E3): bedingend — kodifiziert die Herstellerhaftung für gebrekkige Producten (Art. 6.41) und definiert "producent" weit (Hersteller des Endprodukts, Bestandteil-Hersteller, Rohstoff-Hersteller, sowie jeder, der sich durch Marke/Namen als Hersteller ausgibt, Art. 6.43); der Produktbegriff (Art. 6.42) erfasst jede körperliche bewegliche Sache, auch als Bestandteil einer beweglichen/unbeweglichen Sache. Für wiederaufgearbeitete Bauprodukte ist dies das materielle Haftungsregime, das den nach CPR Art. 26 (REG-EU-1-002/REG-EU-7-101) fingierten "Hersteller" tatsächlich treffen würde.
- F2 (E3): schweigend — Art. 43 hebt die alte Wet 25.2.1991 (RL 85/374/EWG-Umsetzung) ausdrücklich auf; die neue Produkthaftungs-RL (EU) 2024/2853 (Umsetzungsfrist 9.12.2026, s. REG-EU-7-103) ist in Boek 6 noch NICHT eingearbeitet — Belgien kodifiziert zum Stichtag weiterhin das alte RL-85/374-Regime; ein Anpassungsgesetz zur neuen RL wurde nicht identifiziert.
- G: rechnerischer Nachweis, Erklärung Dritter (explizit=E1, Art. 6.47–6.48 Beweislast/Haftungsausschluss — im Detail nicht eigens extrahiert)
- Kernaussage: Boek 6 nBW (in Kraft seit 2025-01-01) integriert und ersetzt die frühere Wet 25.2.1991 zur Produkthaftung; Art. 6.41 begründet die verschuldensunabhängige Herstellerhaftung für gebrekkige Producten, Art. 6.42/6.43/6.45 definieren Produkt, Produzent und Mangel. Für Vlaanderen gilt dies unmittelbar wie im übrigen Belgien. Zentral für Reuse: Dieses Regime ist die materielle Anspruchsgrundlage, die den durch CPR Art. 26 fingierten "Hersteller" bei wiederaufgearbeiteten Bauprodukten tatsächlich trifft — die neue EU-Produkthaftungs-RL 2024/2853 mit expliziter Refurbisher-Haftung (REG-EU-7-101) ist hierin zum Stichtag noch nicht transponiert.
- Wortlautbeleg (Originalsprache): "De producent is aansprakelijk voor de schade veroorzaakt door een gebrek in zijn product" (Art. 6.41); "Onder 'product' wordt verstaan elk lichamelijk roerend goed, ook indien het een bestanddeel vormt van een ander roerend of onroerend goed" (Art. 6.42); "Onder 'producent' wordt verstaan de fabrikant van een eindproduct, de fabrikant van een onderdeel van een eindproduct, de fabrikant of de producent van een grondstof, alsmede eenieder die zich als fabrikant of producent aandient door zijn naam, zijn merk of een ander herkenningsteken op het product aan te brengen" (Art. 6.43); "Een product is gebrekkig wanneer het niet de veiligheid biedt die men gerechtigd is te verwachten, alle omstandigheden in aanmerking genomen" (Art. 6.45); "De wet van 25 februari 1991 betreffende de aansprakelijkheid voor produkten met gebreken wordt opgeheven" (Art. 43)
- Beleg-Quelle: B0 (in dieser Sitzung per WebFetch erneut geöffnet, alle zitierten Artikel im Wortlaut bestätigt) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Gesetzbuch selbst ist Bindungsakt)
- Quelle: Tier 1 · https://www.ejustice.just.fgov.be/eli/wet/2024/02/07/2024001600/justel · Fassung(as-amended) 2024-07-01 (Publikation), in Kraft seit 2025-01-01 · Zugriff 2026-08-11
- Status: in Kraft · seit 2025-01-01
- Sub-Ebene: entfällt (A=national)
- Relationen: setzt um RL 85/374/EWG (Altregime); ersetzt Wet 25.2.1991; wird künftig überlagert/abgelöst durch Transposition von RL (EU) 2024/2853 (REG-EU-7-101/-103, Umsetzungsfrist 2026-12-09 — Transpositionsstand für Belgien nicht geprüft, offene Anschlussfrage)
- Konfidenz: gesichert

---

### REG-BE-7-019 · Art. 1792/2270 (oud) Burgerlijk Wetboek — Dezennale aansprakelijkheid (vertraglich)
- Titel: Dezennale aansprakelijkheid van aannemer/architect, kodifiziert in Art. 1792 und 2270 (oud) Burgerlijk Wetboek — bislang nicht durch das neue Boek 5 (Verbintenissen/besondere overeenkomsten) ersetzt
- Fundstelle: Art. 1792, Art. 2270 oud BW — Primärtext im neuen BW-Aufbau in dieser wie in der vorangegangenen Sitzung nicht direkt geöffnet; Fortgeltung indirekt bestätigt über Zitat in Wet Peeters-Borsus Art. 3 (REG-BE-7-020)
- A: national · Downstream-Verifikationsstatus: entfällt (föderales Zivilrecht)
- B: Primärfeld 7
- C: materialübergreifend
- D: Gesetz
- E: Einbau/Abnahme, Betrieb/Dokumentation
- F1 (E3): hemmend/bedingend — begründet eine zehnjährige verschuldensunabhängige Haftung von Bauunternehmer und Architekt für Mängel an der Solidität/Stabilität des Bauwerks; bei Einbau wiederverwendeter tragender Bauteile trägt der ausführende Unternehmer/Architekt dieses Zehn-Jahres-Risiko ungeachtet der Materialherkunft — strukturell ein Praktiker-Hemmnis analog zur deutschen VOB/B-Diskussion, da für Reuse-Bauteile typischerweise weniger belastbare Herstellerdokumentation/Gewährleistung vorliegt als bei Neuprodukten.
- F2 (E3): bedingend — die materielle Fortgeltung der alten Art. 1792/2270 BW (bis zur Verabschiedung eines neuen Boek 5) ist in dieser Sitzung NICHT direkt am Normtext, sondern nur indirekt bestätigt: Die Wet Peeters-Borsus vom 31.5.2017 (Art. 3, s. REG-BE-7-020), ein 2017 verabschiedetes und weiterhin geltendes Gesetz, verweist ausdrücklich auf "de burgerlijke aansprakelijkheid bedoeld in de artikelen 1792 en 2270 van het Burgerlijk Wetboek" — das bestätigt primärquellenbasiert (wenn auch nur über eine Referenznorm, nicht die Anspruchsgrundlage selbst), dass diese Artikel zum Stichtag als Rechtsgrundlage in Bezug genommen werden.
- G: rechnerischer Nachweis, zerstörungsfreie Prüfung (inferiert=E3, aus der allgemeinen Systematik der Dezennalhaftung — Standsicherheits-/Wasserdichtheitsnachweis)
- Kernaussage: Die zehnjährige Bauunternehmer-/Architektenhaftung nach Art. 1792/2270 des ALTEN Burgerlijk Wetboek ist vertragliches (nicht außervertragliches) Recht und liegt daher außerhalb von Boek 6 (REG-BE-7-018, das nur die außervertragliche Haftung rekodifiziert). Ein neues, diese Artikel ersetzendes Boek 5 ("besondere overeenkomsten") war zum Stichtag nicht auffindbar verabschiedet; die Fortgeltung der alten Artikel wird indirekt durch ihre explizite Nennung in der noch 2017 erlassenen und weiterhin geltenden Wet Peeters-Borsus bestätigt.
- Wortlautbeleg (Originalsprache): "de burgerlijke aansprakelijkheid bedoeld in de artikelen 1792 en 2270 van het Burgerlijk Wetboek" (Zitat aus Wet Peeters-Borsus Art. 3, in dieser Sitzung per WebFetch bestätigt — nicht der Art. 1792/2270-Text selbst, sondern der Fortgeltungsbeleg)
- Beleg-Quelle: B2 (kein direkter Zugriff auf Art. 1792/2270 selbst gelungen — weder in dieser noch der vorangegangenen Sitzung; Fortgeltungsbeleg B0-Qualität über REG-BE-7-020 in dieser Sitzung verifiziert) · Zugänglichkeit: frei-primär (vermutet, Primärtext nicht direkt geöffnet) · Bindungsakt: entfällt (Gesetzestext selbst, sofern lokalisiert, wäre Bindungsakt)
- Quelle: Tier 1 (indirekt, über REG-BE-7-020) https://www.ejustice.just.fgov.be/cgi_loi/change_lg.pl?language=nl&la=N&cn=2017053102&table_name=wet · Zugriff 2026-08-11
- Status: in Kraft (Fortgeltung indirekt bestätigt) · Ursprungsdatum Code Napoléon-Tradition, Konsolidierungsstatus im neuen BW-Aufbau ungeklärt
- Sub-Ebene: entfällt (A=national)
- Relationen: wird kombiniert mit REG-BE-7-020 (Wet Peeters-Borsus, Versicherungspflicht für dieses Haftungsregime); steht neben REG-BE-7-018 (Boek 6, außervertraglich — funktional getrennter Regelungsgegenstand, keine Konkurrenz)
- Konfidenz: abgeleitet (Fortgeltung indirekt, aber primärquellenbasiert über Referenznorm bestätigt; Normtext selbst nicht direkt eingesehen — RL (EU) 2024/2853-Transposition in dieses Regime ebenfalls nicht geprüft)

---

### REG-BE-7-020 · Wet Peeters-Borsus (31.05.2017) — Pflichtversicherung zehnjährige Bauhaftung
- Titel: Wet van 31 mei 2017 betreffende de verplichte verzekering van de tienjarige burgerlijke aansprakelijkheid van aannemers, architecten en andere dienstverleners in de bouwsector ("Wet Peeters-Borsus")
- Fundstelle: Art. 2 (toepassingsgebied/definities), Art. 3 (verzekeringsplicht), Art. 6 (dekkingsminima)
- A: national · Downstream-Verifikationsstatus: entfällt (föderales Gesetz, gilt unmittelbar auch in Vlaanderen)
- B: Primärfeld 7
- C: materialübergreifend
- D: Gesetz
- E: Einbau/Abnahme, Betrieb/Dokumentation
- F1 (E3): bedingend — verpflichtet aannemers, architecten und andere Bau-Dienstleister zu einer Pflichtversicherung der zehnjährigen Haftung (Art. 1792/2270 BW, REG-BE-7-019) für Wohnimmobilien mit gesetzlicher Architektenpflicht; deckt Mängel an Solidität/Stabilität/Wasserdichtheit der geschlossenen Rohbauphase, Mindestdeckung 500.000 € (oder Wiederaufbauwert, falls höher).
- F2 (E3): unklar — ob/wie Versicherer bei Einsatz wiederverwendeter tragender Bauteile abweichende Prämien, Ausschlüsse oder zusätzliche Nachweisanforderungen verlangen, ist NICHT primärquellenbasiert belegt (dies wäre der praktische Kern der "Versicherbarkeits"-Fragestellung des Projekts, hier nur die gesetzliche Pflichtversicherungsstruktur selbst belegt, nicht deren Reuse-spezifische Vollzugspraxis).
- G: Erklärung Dritter, Dokumentenlage (explizit=E1, Art. 3 Versicherungsnachweispflicht; Kap. 5 "Nachweise" im Gesetz — im Detail nicht extrahiert)
- Kernaussage: Die Wet Peeters-Borsus (in Kraft seit 2018-07-01, Art. 10 bereits ab 2017-12-01) verpflichtet Bauunternehmer, Architekten und andere Bau-Dienstleister bei Wohnimmobilien mit Architektenpflicht zu einer Pflichtversicherung ihrer zehnjährigen Haftung (Art. 1792/2270 BW) für Solidität/Stabilität/Wasserdichtheit des geschlossenen Rohbaus, mit einer Mindestdeckung von 500.000 € bzw. dem Wiederaufbauwert. Das Gesetz bestätigt primärquellenbasiert und indirekt die Fortgeltung von Art. 1792/2270 BW (REG-BE-7-019) und ist der belgische Funktionsanalog zur deutschen Diskussion um die Versicherbarkeit fehlender Leistungserklärung/DoP bei Reuse-Bauteilen — ohne die materielle Dezennalhaftung selbst zu kodifizieren (reines Versicherungsgesetz).
- Wortlautbeleg (Originalsprache): "aannemer: iedere natuurlijke of rechtspersoon, die zich er toe verbindt om voor rekening van een ander … een bepaald onroerend werk op woningen" (Art. 2); "de burgerlijke aansprakelijkheid bedoeld in de artikelen 1792 en 2270 van het Burgerlijk Wetboek" (Art. 3); "500 000 euro" als Mindestdeckung bei Wiederaufbauwert oberhalb dieser Schwelle (Art. 6)
- Beleg-Quelle: B0 (in dieser Sitzung per WebFetch erneut geöffnet, Art. 2/3/6 im Wortlaut bestätigt) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Gesetz selbst ist Bindungsakt)
- Quelle: Tier 1 · https://www.ejustice.just.fgov.be/cgi_loi/change_lg.pl?language=nl&la=N&cn=2017053102&table_name=wet · Fassung(as-amended) 2017-06-09 (Publikation) · Zugriff 2026-08-11
- Status: in Kraft · seit 2018-07-01 (Art. 10 ab 2017-12-01)
- Sub-Ebene: entfällt (A=national)
- Relationen: wird kombiniert mit REG-BE-7-019 (materielle Anspruchsgrundlage); bestätigt indirekt Fortgeltung von Art. 1792/2270 BW
- Konfidenz: gesichert

---

## Lückenliste dieser Extraktionssitzung (ehrlich markiert)

1. **REG-BE-2-006:** Der konkrete Verknüpfungsartikel zwischen Omgevingsvergunningendecreet und VLAREMA-Sloopopvolgingsplan-Pflicht wurde im gelesenen Textauszug nicht lokalisiert — nur indirekt über die Tracimat-Prozessbeschreibung bestätigt.
2. **REG-BE-3-008/-009/-010:** Der amtliche VLAREMA-Artikeltext (Vlaamse Codex/ejustice) konnte für Art. 4.3.3/4.3.5 in dieser Sitzung nicht direkt im Vollzitat extrahiert werden; Art. 4.3.3 stützt sich auf einen Rechtsdatenbank-Mirror (Kustcodex, B1), Art. 4.3.5 auf Sekundärquellen (B3) — für W4 zwingend vor Übernahme als Faktum erneut direkt am Vlaamse-Codex-Primärtext zu prüfen.
3. **REG-BE-3-011:** Das Ministerielle Erkenningsbesluit für Tracimat (ca. 2017) wurde trotz mehrerer Versuche nicht im Belgisch Staatsblad/Justel identifiziert.
4. **REG-BE-2-005:** Das Gründungs-KB des rein-nationalen (nicht EU-ETA-)ATG-Systems (BUtgb) bleibt unidentifiziert.
5. **REG-BE-4-012:** Der amtliche PDF-Primärtext des KB Basisnormen konnte technisch nicht in Klartext konvertiert werden; Aussagen stützen sich auf Buildwise-Sekundärzusammenfassung.
6. **REG-BE-1-002:** Novellierungsstatus des Marktüberwachungsgesetzes (Wet 21.12.2013) gegenüber der neuen CPR 2024/3110 bleibt ungeklärt.
7. **REG-BE-5b-016:** Alle regionalen Förderprogramme (hier: Vlaanderen Circulair) sind reine Existenznachweise ohne geprüfte Rechtsgrundlage.
8. **REG-BE-6-017:** Exakte Justel-cn-Fundstelle der Normalisatiewet weiterhin nicht identifiziert.
9. **REG-BE-7-019:** Primärtext von Art. 1792/2270 (oud) BW selbst nicht direkt geöffnet (nur Fortgeltung indirekt über Referenznorm REG-BE-7-020 bestätigt); Transposition der RL (EU) 2024/2853 in belgisches Recht nicht geprüft (betrifft REG-BE-7-018/-019 gleichermaßen).
10. **Gemeinde-Ebene:** Innerhalb Vlaanderens wurde keine Prüfung auf zusätzliche Gemeinde-Bauverordnungen (Sub-Sub-Ebene) vorgenommen — laut Ticket-Scope (Vollerhebung bis Regionsebene) vermutlich nicht gefordert, aber nicht ausdrücklich bestätigt.

**Nächster Schritt:** Dieses Dokument ist Input für die adversarische Prüfung (W2 Stufe 3) und die Synthese (W4). Insbesondere REG-BE-3-010 (Sloopattest-Artikeltext) und REG-BE-3-011 (Tracimat-Erkenningsbesluit) dürfen dort NICHT ohne die vermerkte Nachverifikation als B0-Fakten geführt werden.
