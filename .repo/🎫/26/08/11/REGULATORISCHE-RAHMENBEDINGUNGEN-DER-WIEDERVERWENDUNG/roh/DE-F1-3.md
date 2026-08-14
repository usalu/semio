# DE-Extraktion Felder 1–3 — Produkt-/Konformitätsrecht, Bautechnische Zulassung/Standsicherheit, Abfall-/Stoffrecht

**Projekt:** Zukunft Bau „Entwerfen mit Bestand" (Az. 10.08.18.7-25.06, LUH Hannover + UdK Berlin). **Stichtag:** 2026-08-11. **Zugriff auf alle Primärquellen:** 2026-08-11.
**Grundlage:** DE-Quellenkarte (`roh/DE-quellen.md`) sowie die drei W0-Piloten (`roh/pilot-de-produkt.md`, `roh/pilot-de-zie.md`, `roh/pilot-de-abfall.md`), deren bereits primärquellenbasiert verifizierte Objekte hier übernommen, in das für W2 verbindliche ID-Schema überführt und um in dieser Session zusätzlich erhobene Objekte (DepV, NachwV, AVV Kap. 17, ErsatzbaustoffV §§19–23, DIBt-Zulassungsdatenbank, MVV-TB-Status, VDI 6200, Eurocode-NA-Lücke) ergänzt wurden.
**Methodikhinweis (Ehrlichkeit der Beleg-Quellen):** Objekte, die aus den W0-Piloten übernommen wurden und dort per lokalem `pdftotext` aus dem geladenen Primärdokument zitiert wurden, führen weiterhin B0. Objekte, die in dieser Session per WebFetch-Werkzeug (das den Seiteninhalt durch ein internes Zusammenfassungsmodell verarbeitet, bevor es an mich zurückgegeben wird) direkt von einer amtlichen HTML-Seite gelesen wurden, führen vorsichtshalber B1 statt B0, auch wenn die Werkzeugausgabe Anführungszeichen enthält — ich habe den rohen HTML/PDF-Quelltext in diesen Fällen nicht selbst Zeichen für Zeichen geprüft. Wo ein Zugriff scheiterte oder nur eine Sekundärquelle verfügbar war, ist dies als B2–B4 und ggf. als offene Lücke markiert, nicht stillschweigend übergangen.

---

## Feld 1 · Produkt-/Konformitätsrecht

### REG-EU-1-001 · CPR 2024/3110 — Anwendungsbereich inkl. gebrauchter Produkte
- Titel: Verordnung (EU) 2024/3110 des Europäischen Parlaments und des Rates vom 27. November 2024 zur Festlegung harmonisierter Bedingungen für die Vermarktung von Bauprodukten, zur Änderung der Verordnung (EU) 2019/1020 und zur Aufhebung der Verordnung (EU) Nr. 305/2011
- Fundstelle: Art. 2 Abs. 1–3; Art. 3 Nr. 5, 20, 25; Erwägungsgründe 34–36 (ELI: http://data.europa.eu/eli/reg/2024/3110/oj)
- A: EU/EEA · B: 1 Produkt-/Konformitätsrecht · C: materialübergreifend · D: EU-VO
- E: Inverkehrbringen; Bestandserkundung (mittelbar, da „gebrauchtes Produkt" erst nach Ausbau entsteht)
- F1 (E3): ermöglichend (schafft erstmals ausdrücklichen Rechtsrahmen für Gebraucht-/Wiederaufbereitungsprodukte, statt Schweigen wie unter VO 305/2011) · F2 (E3): schweigend/bedingend (Wirkung hängt von noch zu erlassenden produktspezifischen harmonisierten Spezifikationen ab, s. REG-EU-1-006; zum Stichtag in der Praxis noch kaum spürbar)
- G: Dokumentenlage (explizit, E1 — Art. 3 Nr. 20/25 verlangen Nachweis, dass kein Abfallstatus vorliegt und welcher Umwandlungsgrad erreicht wurde)
- Kernaussage: Die Verordnung gilt gemäß Art. 2 Abs. 1 ausdrücklich für Bauprodukte „einschließlich gebrauchter Produkte". Art. 3 Nr. 20 definiert „gebrauchtes Produkt" als ein Produkt, das kein Abfall (RL 2008/98/EG) ist, mindestens einmal in ein Bauwerk eingebaut wurde und keinem über Prüfung/Reinigung/Reparatur hinausgehenden Verfahren unterzogen wurde (oder einem als nicht wesentlich für die Leistung eingestuften Umwandlungsprozess); Art. 3 Nr. 25 definiert davon abgegrenzt das „wiederaufbereitete Produkt" (wesentlicher Umwandlungsprozess).
- Wortlautbeleg (Originalsprache): "Diese Verordnung gilt für Bauprodukte einschließlich gebrauchter Produkte" (Art. 2 Abs. 1); "‚gebrauchtes Produkt' bezeichnet ein Produkt, das kein Abfall im Sinne der Richtlinie 2008/98/EG ist … und mindestens einmal in ein Bauwerk eingebaut wurde und a) keinem Verfahren unterzogen worden ist, das über Maßnahmen der Prüfung, Reinigung oder Reparatur zum Zwecke der Verwertung hinausgeht … oder b) einem Umwandlungsprozess unterzogen worden ist, der … als nicht wesentlich für die Leistung des Produkts eingestuft ist" (Art. 3 Nr. 20)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (unmittelbar geltendes EU-Recht)
- Quelle: Tier 1 · https://eur-lex.europa.eu/legal-content/DE/TXT/PDF/?uri=OJ:L_202403110 · Fassung(as-amended) 2024-12-18 · Zugriff 2026-08-11
- Status: in Kraft (punktuell seit 2025-01-07, im Kern seit 2026-01-08) · Datum: 2024-12-18
- Sub-Ebene: entfällt (A=EU/EEA)
- Relationen: ersetzt REG-EU-1-007 (mit Restfortgeltung); konkretisiert durch REG-EU-1-002/003/004/005/006
- Konfidenz: gesichert

---

### REG-EU-1-002 · CPR 2024/3110 Art. 26 Abs. 2 — Herstellerfiktion beim Inverkehrbringen gebrauchter/wiederaufbereiteter Produkte
- Titel: wie REG-EU-1-001
- Fundstelle: Art. 26 Abs. 1–2 i. V. m. Art. 22 (Herstellerpflichten)
- A: EU/EEA · B: 1 Produkt-/Konformitätsrecht · C: materialübergreifend · D: EU-VO
- E: Inverkehrbringen; Planung/Nachweis
- F1 (E3): hemmend (wer ein gebrauchtes Bauprodukt ohne einschlägige hEN-Gebraucht-Regel oder ein wiederaufbereitetes Produkt in Verkehr bringt, wird rechtlich zum „Hersteller" mit vollen Pflichten aus Art. 22) · F2 (E3): hemmend (Bauteilbörsen/Rückbauunternehmen verfügen i. d. R. nicht über Herstellerinfrastruktur für AVCP-Verfahren und Leistungserklärungen bei heterogenen Rückbauteilen)
- G: rechnerischer Nachweis + Erklärung Dritter (explizit, E1)
- Kernaussage: Art. 26 Abs. 1 ordnet an, dass ein Einführer oder Händler unter bestimmten Voraussetzungen (u. a. Inverkehrbringen unter eigenem Namen, wesentliche Änderung des Produkts) als Hersteller gilt und den vollen Herstellerpflichten aus Art. 22 unterliegt; Art. 26 Abs. 2 erstreckt diese Rechtsfolge ausdrücklich auf JEDEN Wirtschaftsteilnehmer (nicht nur Einführer/Händler im engen Sinn von Abs. 1), der (a) ein gebrauchtes Produkt mit einschlägiger Gebraucht-Regel in einer hEN, (b) ein gebrauchtes Produkt ohne solche Regel, das zuvor nicht in der Union in Verkehr gebracht wurde, oder (c) ein wiederaufbereitetes Produkt in Verkehr bringt. Da Spezifikationen mit expliziten Gebraucht-Regeln laut Erwägungsgrund 35 die Ausnahme sind, trifft die Herstellerfiktion faktisch die meisten aktiv vermarktenden Wiederverwendungsakteure.
- Wortlautbeleg (Originalsprache, per lokalem pdftotext aus dem OJ-PDF am 2026-08-11 gegengeprüft — KORRIGIERT gegenüber Vorfassung, die Abs. 1 und Abs. 2 fälschlich als einen durchgehenden Satz zitierte): Art. 26 Abs. 1 Satz 1: "In den folgenden Fällen gilt ein Einführer oder Händler als Hersteller für die Zwecke dieser Verordnung und unterliegt den Herstellerpflichten gemäß Artikel 22: a) wenn er ein Produkt unter seinem eigenen Namen oder seiner Handelsmarke in Verkehr bringt; b) wenn er ein Produkt vorsätzlich so ändert oder es unabsichtlich so geändert wird, dass die Übereinstimmung der Leistungs- und Konformitätserklärung … beeinträchtigt werden kann; […] e) wenn er entscheidet, die Rolle des Herstellers zu übernehmen." Art. 26 Abs. 2 (separater Satz, eigenständiger Anwendungsbereich): "Absatz 1 gilt auch für Wirtschaftsteilnehmer, die Folgendes in Verkehr bringen: a) ein gebrauchtes Produkt, für das eine harmonisierte technische Spezifikation mit Vorschriften für gebrauchte Produkte gilt, b) ein gebrauchtes Produkt, das nicht unter eine harmonisierte technische Spezifikation mit Bestimmungen für gebrauchte Produkte fällt und zuvor nicht in der Union in Verkehr gebracht wurde, c) ein wiederaufbereitetes Produkt."
- Beleg-Quelle: B0 (2026-08-11 per pdftotext am EUR-Lex-OJ-PDF gegengeprüft, Zitat entlang der tatsächlichen Absatzgrenzen korrigiert) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://eur-lex.europa.eu/legal-content/DE/TXT/PDF/?uri=OJ:L_202403110 · Fassung(as-amended) 2024-12-18 · Zugriff 2026-08-11
- Status: in Kraft, Wirksamkeit produktfamilienabhängig aufgeschoben (s. REG-EU-1-006) · Datum: 2024-12-18
- Sub-Ebene: entfällt
- Relationen: konkretisiert REG-EU-1-001
- Konfidenz: gesichert (Wortlaut), abgeleitet (Praxiswirkung F2)

---

### REG-EU-1-003 · CPR 2024/3110 Art. 20 Abs. 1 — Wirtschaftsteilnehmerpflichten nur für hEN-/ETA-Produkte
- Titel: wie REG-EU-1-001
- Fundstelle: Art. 20 Abs. 1 (Kapitel III)
- A: EU/EEA · B: 1 Produkt-/Konformitätsrecht · C: materialübergreifend · D: EU-VO
- E: Inverkehrbringen
- F1 (E3): ermöglichend (Bauteile ohne harmonisierte Norm/ETA — Regelfall bei historischen, vor-CE-zeitlichen Bestandsbauteilen — fallen aus dem CPR-Pflichtenkatalog heraus) · F2 (E3): widersprüchlich (Freiraum für Reuse nicht-hEN-erfasster Altbauteile, aber unklar, welches Regime dann greift — s. REG-DE-2-002)
- G: entfällt (reine Scope-Norm, kein Nachweistatbestand)
- Kernaussage: Die Pflichten nach Kapitel III (inkl. Herstellerfiktion Art. 26) gelten nach Art. 20 Abs. 1 nur für Produkte unter einer harmonisierten technischen Spezifikation oder mit CE-Kennzeichnung auf ETA-Grundlage. Historische Bauteile ohne einschlägige hEN sind von vornherein außerhalb des CPR-Pflichtenkreises — das Nachweisregime für ihre Wiederverwendung liegt beim nationalen Bauordnungsrecht (Feld 2).
- Wortlautbeleg (Originalsprache): "Die Verpflichtungen der Wirtschaftsteilnehmer gemäß diesem Kapitel gelten nur für Produkte, die unter eine harmonisierte technische Spezifikation fallen, oder für Produkte, die auf der Grundlage einer Europäischen Technischen Bewertung mit CE-Kennzeichnung versehen wurden." (Art. 20 Abs. 1)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://eur-lex.europa.eu/legal-content/DE/TXT/PDF/?uri=OJ:L_202403110 · Fassung(as-amended) 2024-12-18 · Zugriff 2026-08-11
- Status: in Kraft · Datum: 2024-12-18
- Sub-Ebene: entfällt
- Relationen: relativiert REG-EU-1-002; setzt Bezug zu REG-DE-2-002 (deutsches Verwendbarkeitsnachweis-System greift dort, wo CPR nicht greift)
- Konfidenz: gesichert (Wortlaut), abgeleitet (Praxisfolge)

---

### REG-EU-1-004 · CPR 2024/3110 Erwägungsgrund 34 — Ausnahme für direkte Wiederverwendung im selben Bauwerk
- Titel: wie REG-EU-1-001
- Fundstelle: Erwägungsgrund 34, Satz 3
- A: EU/EEA · B: 1 Produkt-/Konformitätsrecht · C: materialübergreifend · D: EU-VO (Erwägungsgrund, Auslegungshilfe zu Art. 2/3, keine eigene Bindungswirkung)
- E: Rückbau/Sicherung; Einbau/Abnahme
- F1 (E3): ermöglichend (Bauteile, die ohne Marktdurchgang direkt im selben Bauwerk wiederverwendet werden, gelten nicht als erneut in Verkehr gebracht) · F2 (E3): ermöglichend, aber eng (deckt kleinteiligen Bauteilerhalt im eigenen Projekt, nicht die marktvermittelte Bauteilbörsen-Praxis)
- G: entfällt (Auslegungshinweis zur Reichweite von „Inverkehrbringen")
- Kernaussage: Produkte, die direkt im selben Bauwerk wiederverwendet werden, gelten nicht als erneut in Verkehr gebracht und unterliegen daher keinen CPR-Maßnahmen. Diese Ausnahme greift nur bei Verbleib im identischen Bauwerk ohne Marktdurchgang — die häufigste Form der Bauteilbörsen-vermittelten Wiederverwendung (Ausbau, Zwischenlagerung, Verkauf an anderes Projekt) fällt NICHT darunter.
- Wortlautbeleg (Originalsprache): "Produkte, die direkt in einem Bauwerk wiederverwendet werden, sollten jedoch nicht als erneut in Verkehr gebracht gelten und daher keinen Maßnahmen im Rahmen der vorliegenden Verordnung unterliegen." (Erwägungsgrund 34)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://eur-lex.europa.eu/legal-content/DE/TXT/PDF/?uri=OJ:L_202403110 · Fassung(as-amended) 2024-12-18 · Zugriff 2026-08-11
- Status: in Kraft · Datum: 2024-12-18
- Sub-Ebene: entfällt
- Relationen: konkretisiert Art. 3 Nr. 5 (REG-EU-1-001); grenzt REG-EU-1-002 ein
- Konfidenz: gesichert (Wortlaut), abgeleitet (Reichweiten-Einordnung)

---

### REG-EU-1-005 · CPR 2024/3110 Art. 14/15/18 — Leistungs- und Konformitätserklärung inkl. Sonderregel für gebrauchte Produkte
- Titel: wie REG-EU-1-001
- Fundstelle: Art. 14 (Befreiungstatbestände), Art. 15 (Inhalt), Art. 18 Abs. 2 Buchst. a (CE-Kennzeichnungsdatum)
- A: EU/EEA · B: 1 Produkt-/Konformitätsrecht · C: materialübergreifend · D: EU-VO
- E: Inverkehrbringen; Planung/Nachweis
- F1 (E3): bedingend (voraussetzungsreiche Erklärung inkl. Umweltfußabdruck, für gebrauchte Produkte besondere Datumsregel, aber keine inhaltliche Erleichterung) · F2 (E3): hemmend (Aufwand für ein singuläres Rückbauteil ohne Herstellerorganisation unverhältnismäßig; Befreiung Art. 14 erfasst nur Einzelanfertigung im selben Bauwerk unter Bauleitung bzw. Denkmalschutz-Renovierung, nicht marktvermittelte Bauteilwiederverwendung)
- G: rechnerischer Nachweis + Dokumentenlage (explizit, E1)
- Kernaussage: Art. 18 Abs. 2 Buchst. a sieht für gebrauchte Produkte eine besondere CE-Datumsangabe vor (Jahr der Demontage gefolgt vom Jahr der Wiederkennzeichnung) — der Verordnungsgeber hat den Fall technisch mitgedacht, löst aber nicht das strukturelle Problem, dass der nach Art. 26 Abs. 2 fingierte „Hersteller" die vollständige Erklärung neu erstellen muss. Art. 14 befreit nur eng umgrenzte Fälle, nicht die typische Bauteilbörsen-Konstellation.
- Wortlautbeleg (Originalsprache): "die beiden letzten Ziffern des Jahres, in dem die CE-Kennzeichnung erstmals angebracht wurde, oder bei gebrauchten Produkten die beiden letzten Ziffern des Jahres, in dem das Produkt demontiert wurde, gefolgt von den letzten beiden Ziffern des Jahres, in dem die CE-Kennzeichnung an dem gebrauchten Produkt angebracht wurde" (Art. 18 Abs. 2 Buchst. a)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://eur-lex.europa.eu/legal-content/DE/TXT/PDF/?uri=OJ:L_202403110 · Fassung(as-amended) 2024-12-18 · Zugriff 2026-08-11
- Status: in Kraft (Kernpflichten größtenteils ab 2026-01-08) · Datum: 2024-12-18
- Sub-Ebene: entfällt
- Relationen: konkretisiert REG-EU-1-002; setzt REG-EU-1-007 Art. 4 fort/ersetzt sie
- Konfidenz: gesichert (Wortlaut), abgeleitet (Aufwandsbewertung F2)

---

### REG-EU-1-006 · CPR 2024/3110 Übergangsregime (Art. 94–96) — Doppelspurigkeit mit VO 305/2011 bis 2040
- Titel: wie REG-EU-1-001
- Fundstelle: Art. 94 (Aufhebung), Art. 95 (Übergangsregelungen), Art. 96 (Inkrafttreten)
- A: EU/EEA · B: 1 Produkt-/Konformitätsrecht · C: materialübergreifend · D: EU-VO
- E: Inverkehrbringen
- F1 (E3): widersprüchlich (zwei Regelungsregime laufen bis 2040 parallel) · F2 (E3): hemmend (Rechtsunsicherheit, welches Regime für ein konkretes Bauteil gilt)
- G: Dokumentenlage (explizit, E1 — maßgeblich ist, ob die hEN am 08.01.2026 im Verzeichnis nach Art. 17 Abs. 5 VO 305/2011 noch gültig war)
- Kernaussage: VO 305/2011 wird zum 8.1.2026 aufgehoben, mit Ausnahme zentraler Artikel (Art. 4–9, 11–18), die für Produkte unter fortgeltenden „alten" hEN bis 8.1.2040 fortgelten. Nach Art. 95 Abs. 9 treten die neuen Pflichten für eine Produktfamilie zudem erst ein Jahr nach Erlass des zugehörigen Durchführungsrechtsakts in Kraft — bis dahin bleibt für die meisten Produktfamilien faktisch das alte, gebraucht-blinde VO-305/2011-Regime maßgeblich.
- Wortlautbeleg (Originalsprache): "Die Verordnung (EU) Nr. 305/2011 wird mit Wirkung vom 8. Januar 2026 aufgehoben, mit Ausnahme des Artikels 2, der Artikel 4 bis 9, der Artikel 11 bis 18 … die mit Wirkung vom 8. Januar 2040 aufgehoben werden." (Art. 94)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://eur-lex.europa.eu/legal-content/DE/TXT/PDF/?uri=OJ:L_202403110 · Fassung(as-amended) 2024-12-18 · Zugriff 2026-08-11
- Status: Übergang (bis spätestens 2040) · Datum: Stichtage 2025-01-07, 2026-01-08, 2040-01-08
- Sub-Ebene: entfällt
- Relationen: modifiziert REG-EU-1-001/002/003/005; hält REG-EU-1-007 partiell in Kraft
- Konfidenz: gesichert

---

### REG-EU-1-007 · CPR 305/2011 (auslaufend) — Leistungserklärungspflicht ohne Gebraucht-Regel
- Titel: Verordnung (EU) Nr. 305/2011 des Europäischen Parlaments und des Rates vom 9. März 2011 zur Festlegung harmonisierter Bedingungen für die Vermarktung von Bauprodukten und zur Aufhebung der Richtlinie 89/106/EWG
- Fundstelle: Art. 2 (Begriffsbestimmungen), Art. 4 (Erstellung der Leistungserklärung)
- A: EU/EEA · B: 1 Produkt-/Konformitätsrecht · C: materialübergreifend · D: EU-VO
- E: Inverkehrbringen
- F1 (E3): schweigend (unterscheidet an keiner Stelle zwischen neuen und gebrauchten Bauprodukten) · F2 (E3): hemmend (jedes wiederverwendete, hEN-erfasste Bauprodukt musste bislang formal wie ein Neuprodukt behandelt werden)
- G: rechnerischer Nachweis (explizit, E1)
- Kernaussage: Nach Art. 4 Abs. 1 erstellt der Hersteller eine Leistungserklärung, wenn ein von einer hEN erfasstes Bauprodukt in Verkehr gebracht wird. Da „Inverkehrbringen" schlicht als erstmalige Marktbereitstellung definiert wird und keine Sonderregel für Ausbau/Wiedereinbau vorgesehen ist, blieb unklar, wie ein gebrauchtes, hEN-erfasstes Bauteil beim Wiederverkauf zu behandeln ist — diese Lücke schließt erst VO 2024/3110.
- Wortlautbeleg (Originalsprache): "Ist ein Bauprodukt von einer harmonisierten Norm erfasst … so erstellt der Hersteller eine Leistungserklärung, wenn ein solches Produkt in Verkehr gebracht wird." (Art. 4 Abs. 1 — sinngemäß nach Sekundärzitat, im W0-Pilot nicht erneut am B0-Volltext verifiziert; Nacherhebung offen)
- Beleg-Quelle: B2 amtliche Referenz, Art. 4-Wortlaut selbst nicht im O-Ton nachgelesen (nur Sekundärzitat); Art. 94/95 VO 2024/3110 zur Fortgeltung dagegen B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 (VO selbst) · https://eur-lex.europa.eu/legal-content/DE/TXT/?uri=CELEX:32011R0305 · Fassung(as-amended) 2011-03-09 · Zugriff 2026-08-11
- Status: Übergang (Kernvorschriften für Altbestand bis 2040 fortgeltend, ansonsten aufgehoben seit 2026-01-08) · Datum: 2026-01-08
- Sub-Ebene: entfällt
- Relationen: ersetzt durch REG-EU-1-001; Restfortgeltung durch REG-EU-1-006
- Konfidenz: abgeleitet (Art. 4-Wortlaut nicht B0-verifiziert)

---

### REG-DE-1-008 · Bauproduktengesetz (BauPG) — Durchführungsgesetz ohne eigenes Reuse-Recht
- Titel: Gesetz zur Durchführung der Verordnungen (EU) Nr. 305/2011 und (EU) 2024/3110 zur Festlegung harmonisierter Vorschriften für die Vermarktung von Bauprodukten (Bauproduktengesetz — BauPG)
- Fundstelle: §§ 1–11 BauPG (gesamtes Gesetz)
- A: national · B: 1 Produkt-/Konformitätsrecht · C: materialübergreifend · D: Gesetz
- E: Inverkehrbringen; Betrieb/Dokumentation (Marktüberwachung)
- F1 (E3): schweigend (regelt ausschließlich institutionelle Fragen — DIBt als notifizierende/benennende Behörde, Marktüberwachung §7, Sprachenregelung §8, Bußgeld/Straf §§10–11 — keine materielle Norm zu gebrauchten Bauprodukten) · F2 (E3): schweigend (rein verfahrensrechtlich, kein feststellbarer Praxiseffekt)
- G: entfällt (kein eigener Nachweistatbestand, verweist auf die EU-Verordnungen)
- Kernaussage: Das BauPG bestimmt das DIBt als notifizierende bzw. benennende Behörde, regelt Marktüberwachungszuständigkeiten, Sprachanforderungen sowie Bußgeld-/Strafvorschriften. Es enthält keine eigenständigen Vorschriften zu Zweck/Anwendungsbereich (die EU-Verordnungen gelten unmittelbar) und erst recht keine Sonderregel zu gebrauchten Bauprodukten. Das materielle Reuse-Recht liegt vollständig bei den EU-Verordnungen (REG-EU-1-001 ff.).
- Wortlautbeleg (Originalsprache): "Das Deutsche Institut für Bautechnik ist notifizierende Behörde nach Artikel 40 Absatz 1 der Verordnung (EU) Nr. 305/2011" (§ 1 Abs. 1, sinngemäß nach Gliederung der Norm)
- Beleg-Quelle: B0 (§§1–11 vollständig eingesehen, W0-Pilot) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.gesetze-im-internet.de/baupg_2013/BJNR245000012.html · Fassung(as-amended) zuletzt geändert durch Art. 1 G v. 9.1.2026 I Nr. 4 · Zugriff 2026-08-11
- Status: in Kraft · Datum: 2013-07-01 (Ersterlass), 2026-01-09 (letzte Änderung)
- Sub-Ebene: entfällt (A=national)
- Relationen: setzt um REG-EU-1-001, REG-EU-1-007
- Konfidenz: gesichert

---

### REG-DE-1-009 · MVV TB 2025/1 (+ Entwurf 2026/1) — konkretisiert Technische Baubestimmungen, schweigt zu gebrauchten Bauprodukten
- Titel: Muster-Verwaltungsvorschrift Technische Baubestimmungen (MVV TB), Ausgabe 2025/1
- Fundstelle: MVV TB 2025/1 gesamt (354 Seiten; Volltext in dieser Erhebung nicht durchsucht, nur amtliche Übersichts-/Meldungsseiten)
- A: national, Umsetzung durch die Länder erforderlich · B: 1/2 Produkt-/Konformitätsrecht und Bautechnische Zulassung (Primärfeld hier 1, da Ergänzung zu Ü-Zeichen/Verwendbarkeitsnachweis für nicht-CE-Produkte; enge Berührung zu Feld 2) · C: materialübergreifend · D: Verwaltungsvorschrift (als Muster; erst durch Landesumsetzung verbindlich)
- E: Planung/Nachweis
- F1 (E3): schweigend (weder die amtliche Bekanntmachung noch die DIBt-Übersichtsseite enthalten einen Hinweis auf gebrauchte/wiederverwendete Bauprodukte oder Bestandsbewertung/Pre-Demolition-Audit) · F2 (E3): unklar/schweigend (354-seitiger Volltext in dieser Erhebung nicht durchsucht, Aussage auf Indizienbasis)
- G: Dokumentenlage (inferiert, E3 — kein aufgefundener Sonderweg, daher gilt der allgemeine Verwendbarkeitsnachweisweg über abZ/ZiE, s. Feld 2)
- Kernaussage: Aktuell eingeführte Ausgabe ist MVV TB 2025/1 (Amtliche Mitteilungen 2025/3 vom 2025-05-20, Druckfehlerberichtigungen 2025-07-29 und 2025-10-22). Ein Anhörungsentwurf für 2026/1 lag vor (Anhörung endete 2025-12-19); laut DIBt-Übersichtsseite (Stand der Seite: 26. Juni 2026) war 2025/1 zu diesem Zeitpunkt weiterhin die referenzierte Ausgabe — eine Einführung von 2026/1 zum Stichtag 2026-08-11 ist NICHT verifiziert. Auf der DIBt-Übersichtsseite selbst findet sich kein Hinweis auf Bauprodukte aus Wiederverwendung, gebrauchte Bauprodukte, Bestandsbewertung oder Pre-Demolition-Audit; die Seite fokussiert auf Themen wie Stahlbeton, Brandschutz und Verankerungssysteme.
- Wortlautbeleg (Originalsprache): kein wörtliches Zitat aus dem MVV-TB-Text selbst möglich (354-seitiger Volltext nicht geöffnet); einzige belastbare deutsche Primärformulierung ist die amtliche Bezeichnung „Amtliche Mitteilungen 2025/3"
- Beleg-Quelle: B1 (DIBt-Übersichtsseite direkt gelesen, Volltext der MVV TB selbst NICHT eingesehen) · Zugänglichkeit: frei-primär (PDF grundsätzlich frei zugänglich, in dieser Erhebung nicht geöffnet) · Bindungsakt: jeweilige Landes-VV TB, keine einzelne im O-Ton verifiziert
- Quelle: Tier 1 · https://www.dibt.de/de/wir-bieten/technische-baubestimmungen · Fassung(as-amended) 2025-05-20 (Ausgabe 2025/1); Stand der Übersichtsseite 2026-06-26 · Zugriff 2026-08-11
- Status: in Kraft (2025/1); Entwurf/Anhörung abgeschlossen, Einführungsstatus 2026/1 offen
- Sub-Ebene: nicht erhoben [alle 16 Länder-Umsetzungen]
- Relationen: konkretisiert REG-DE-2-002/003 (Verwendbarkeitsnachweis-System); ersetzt REG-DE-1-010 (frühere Bauregelliste)
- Konfidenz: unklar (Schweigen zu Reuse ist NICHT durch vollständige Volltextlektüre verifiziert, nur durch Indizien)

---

### REG-DE-1-010 · Aufhebung Bauregellisten A/B/C — EuGH C-100/13 + DIBt-Bekanntmachung
- Titel: Urteil des Gerichtshofs (Zehnte Kammer) vom 16. Oktober 2014, Rechtssache C-100/13, Europäische Kommission/Bundesrepublik Deutschland; nachfolgend DIBt-Bekanntmachung „Aufhebung der Bauregellisten A und B und Liste C" (Amtliche Mitteilungen 2019/1)
- Fundstelle: EuGH-Urteil C-100/13, ECLI:EU:C:2014:2293, Tenor; DIBt Amtliche Mitteilungen 2019/1 vom 2019-03-29
- A: EU/EEA (Urteil) mit unmittelbarer Wirkung auf national (DE) · B: 1 Produkt-/Konformitätsrecht · C: materialübergreifend · D: kein Wert des kontrollierten Vokabulars passt exakt (Urteil ist weder VO/RL/Gesetz/RVO/VV/Techn.Baubestimmung/Norm/Merkblatt/Branchenprotokoll — DIBt-Bekanntmachung selbst am ehesten Verwaltungsvorschrift-nah)
- E: Inverkehrbringen; Einbau/Abnahme
- F1 (E3): ermöglichend (verbietet zusätzliche nationale Anforderungen — inkl. Ü-Zeichen-Pflicht — für CE-gekennzeichnete, hEN-erfasste Bauprodukte; begrenzt Doppelprüfungen) · F2 (E3): ermöglichend mit Kehrseite (Vereinfachung gilt nur für CE-erfasste Produkte; der nach REG-EU-1-003 aus dem CPR-Bereich fallende Großteil historischer Bestandsbauteile bleibt im strengeren nationalen Ü-Zeichen-/Verwendbarkeitsnachweis-System, s. Feld 2)
- G: Sichtprüfung + Erklärung Dritter (explizit, E1 — Übereinstimmungszeichen ÜH/ÜHP/ÜZ je nach Prüftiefe für nicht-hEN-Produkte)
- Kernaussage: Der EuGH stellte fest, dass Deutschland durch die Bauregellisten gegen Art. 4 Abs. 2 und Art. 6 Abs. 1 der Bauprodukterichtlinie 89/106/EWG verstieß, indem es für CE-gekennzeichnete, harmonisiert genormte Bauprodukte zusätzliche nationale Anforderungen (Ü-Zeichen-Pflicht über Bauregelliste B) verlangte. Die Bauregellisten A und B sowie Liste C wurden zum 1. April 2019 durch das DIBt aufgehoben; das Ü-Zeichen ist seither nur noch für nicht-CE-erfasste, national geregelte Bauprodukte einschlägig.
- Wortlautbeleg (Originalsprache): "Die Bundesrepublik Deutschland hat dadurch gegen ihre Verpflichtungen aus Art. 4 Abs. 2 und Art. 6 Abs. 1 der Richtlinie 89/106/EWG verstoßen, dass sie durch die Bauregellisten zusätzliche Anforderungen für den wirksamen Marktzugang und die Verwendung von Bauprodukten gestellt hat, die von den harmonisierten Normen EN 681-2:2000, EN 13162:2008 und EN 13241-1 erfasst wurden und mit der CE-Kennzeichnung versehen waren." (EuGH C-100/13, Tenor)
- Beleg-Quelle: B0 (Urteil und DIBt-Bekanntmachung) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Urteil selbst ist der bindende Akt)
- Quelle: Tier 1 · https://eur-lex.europa.eu/legal-content/DE/ALL/?uri=CELEX%3A62013CJ0100 und https://www.dibt.de/de/aktuelles/meldungen/nachricht-detail/meldung/aufhebung-der-bauregellisten-a-und-b-und-liste-c-ausgabe-20152-mit-aenderungen-20161-und-20162 · Fassung(as-amended) 2014-10-16 (Urteil)/2019-03-29 (Bekanntmachung) · Zugriff 2026-08-11
- Status: in Kraft (fortbestehend; Bauregellisten aufgehoben seit 2019-04-01) · Datum: 2014-10-16/2019-04-01
- Sub-Ebene: entfällt (bundesweite Wirkung)
- Relationen: setzt sich um in REG-DE-2-003 (Abgrenzung Ü-Zeichen vs. CE); ersetzt frühere Bauregelliste-Praxis
- Konfidenz: gesichert

---

### REG-DE-1-011 · Übereinstimmungsnachweis und Ü-Zeichen — Grenzfall bei Wiederverwendung
- Titel: Musterbauordnung (MBO), §§ 21–22 — Übereinstimmungsbestätigung, Übereinstimmungserklärung des Herstellers
- Fundstelle: § 21 Abs. 1–3, § 22 Abs. 1 MBO, Fassung November 2002, zuletzt geändert durch Beschluss der Bauministerkonferenz (BMK) vom 26./27.9.2024
- A: sub-national (MBO als Referenztext ohne eigene Rechtskraft, bindend sind die 16 LBOs) · B: 1 Produkt-/Konformitätsrecht (nationales Pendant zu CE/DoP für nicht-CE-erfasste Bauprodukte) · C: materialübergreifend · D: Gesetz (Landesbauordnung; MBO-Referenztext)
- E: Inverkehrbringen
- F1 (E3): schweigend (der Normtext adressiert den „Hersteller", der durch werkseigene Produktionskontrolle die Übereinstimmung sicherstellt; regelt nicht, wer bei einem gebrauchten Bauteil diese Rolle einnimmt) · F2 (E3): hemmend (ohne identifizierbaren „Hersteller" im Sinne des §22 kann keine Übereinstimmungserklärung und damit kein Ü-Zeichen abgegeben werden — dies dürfte mit dazu beitragen, dass wiederverwendete Bauprodukte über ZiE, s. REG-DE-2-002, statt über den Ü-Zeichen-Pfad in Verkehr gebracht werden)
- G: Erklärung Dritter (explizit, E1 — Übereinstimmungserklärung des Herstellers, §22 Abs.1); für Reuse-Fälle ungeklärt, wer diese Erklärung abgeben kann (inferiert, E3)
- Kernaussage: Bauprodukte bedürfen einer Bestätigung ihrer Übereinstimmung mit Technischen Baubestimmungen, allgemeinen bauaufsichtlichen Zulassungen/Prüfzeugnissen oder Zustimmungen im Einzelfall; diese erfolgt durch Übereinstimmungserklärung des Herstellers und Kennzeichnung mit dem Ü-Zeichen. Der Hersteller darf die Erklärung nur abgeben, wenn er die Übereinstimmung durch werkseigene Produktionskontrolle sichergestellt hat — eine für Neuprodukte, nicht für gebrauchte Bestandsbauteile ausgelegte Konstruktion.
- Wortlautbeleg (Originalsprache): "Der Hersteller darf eine Übereinstimmungserklärung nur abgeben, wenn er durch werkseigene Produktionskontrolle sichergestellt hat, dass das von ihm hergestellte Bauprodukt den maßgebenden technischen Regeln, der allgemeinen bauaufsichtlichen Zulassung, dem allgemeinen bauaufsichtlichen Prüfzeugnis oder der Zustimmung im Einzelfall entspricht."
- Beleg-Quelle: B0 (MBO-PDF vollständig gelesen, W0-Pilot) · Zugänglichkeit: frei-primär · Bindungsakt: MBO selbst NICHT bindend (Muster); bindend sind die 16 LBOs
- Quelle: Tier 1 · gelesenes Exemplar https://bvpi.de/bvpi/downloads/MBO.pdf (§§21–22); amtliches Verzeichnis https://www.bauministerkonferenz.de/verzeichnis.aspx?id=991 · Fassung(as-amended) 2024-09-27 · Zugriff 2026-08-11
- Status: in Kraft · 2024-09-27
- Sub-Ebene: nicht erhoben [alle 16 Länder — Wortlautgleichheit nicht einzeln verifiziert]
- Relationen: konkretisiert REG-DE-2-002/001 (Folgepflicht nach ZiE/vBG); Schnittstelle zu Feld 3 (wer ist „Hersteller" eines wiederverwendeten Bauteils — nicht Gegenstand dieser Datei)
- Konfidenz: gesichert (Normtext); unklar (Anwendung auf Reuse-Konstellation — keine Primärquelle regelt dies explizit)

---

### REG-DE-1-012 · DIBt-Zulassungsdatenbank (Zulassungsdownload)
- Titel: DIBt Zulassungsdownload — Zulassungs-, Genehmigungs- und ETA-Dokumente sowie Gutachten des DIBt
- Fundstelle: gesamte Datenbank/Verfahrensseite
- A: national · B: 1 Produkt-/Konformitätsrecht · C: materialübergreifend · D: Merkblatt (Registerfunktion, keine eigene Norm)
- E: Dokumentenlage/Betrieb/Dokumentation
- F1 (E3): schweigend (keine eigene Kategorie „Reuse-Produkt" im Register vorgesehen) · F2 (E3): schweigend (keine erkennbare Sonderfunktion für Recherche zu bereits zugelassenen Wiederverwendungs-Bauarten/-produkten)
- G: Dokumentenlage (explizit, E1 — Register für abZ/aBG/ETA/ZiE/vBG/Gutachten)
- Kernaussage: Die Seite ermöglicht den kostenlosen Download bauaufsichtlicher Dokumente (abZ, aBG, ETA, ZiE, vBG, Gutachten) über eine Volltextsuche mit Weiterleitung auf spezialisierte Verzeichnisse. Es findet sich kein Hinweis auf gebrauchte oder wiederverwendete Bauprodukte als eigenständige Kategorie — ein relevanter Negativbefund für Feld 1/2: Es existiert keine dedizierte Registerstruktur, die Reuse-spezifische Zulassungen auffindbar bündeln würde; sie liegen (falls vorhanden) unsystematisch unter den allgemeinen abZ/ZiE-Beständen.
- Wortlautbeleg (Originalsprache): "Zulassungs-, Genehmigungs- und ETA-Dokumente sowie Gutachten des DIBt können Sie kostenlos herunterladen."
- Beleg-Quelle: B1 (Seite direkt gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Register, kein Rechtsakt)
- Quelle: Tier 1 · https://www.dibt.de/de/service/zulassungsdownload · Fassung(as-amended) laufend gepflegt, kein Fassungsdatum · Zugriff 2026-08-11
- Status: in Kraft (laufend) · Datum: n/a
- Sub-Ebene: entfällt (A=national)
- Relationen: konkretisiert REG-DE-2-001/002/003 (Fundstelle erteilter Zulassungen)
- Konfidenz: gesichert (Negativbefund „keine Reuse-Kategorie" beruht auf Sichtung der Oberfläche, nicht auf vollständiger Datenbankdurchsuchung — daher abgeleitet)

---

### REG-DE-1-013 · Sub-national Stichprobe Niedersachsen (NBauO) — Ü-Zeichen-Vollzug auf Landesebene, nur veraltete Fassung eingesehen
- Titel: Niedersächsische Bauordnung (NBauO)
- Fundstelle: § 17 (Bauprodukte), § 20 (Nachweis der Verwendbarkeit im Einzelfall), §§ 22–24 (Übereinstimmungsnachweis, -erklärung, -zertifikat)
- A: sub-national (Niedersachsen) · B: 1 Produkt-/Konformitätsrecht (Ü-Zeichen-Bezug) · C: materialübergreifend · D: Gesetz (Landesgesetz)
- E: Einbau/Abnahme
- F1 (E3): unklar — nur eine als „Vom 3. April 2012" bezeichnete PDF-Fassung auffindbar, die noch auf die 2019 aufgehobene Bauregelliste A Bezug nimmt (§17 Abs.3 a.F.) und damit erkennbar NICHT die aktuell geltende, konsolidierte Fassung zum Stichtag 2026-08-11 ist · F2 (E3): unklar (aus veralteter Quelle nicht verlässlich ableitbar)
- G: Erklärung Dritter (explizit, E1, laut Sekundärsuche zu §21/22 a.F.) — NICHT im O-Ton der aktuellen Fassung verifiziert
- Kernaussage: Die NBauO enthält strukturell dieselbe Systematik wie die MBO (Verwendbarkeitsnachweis, Übereinstimmungsnachweis, Ü-Zeichen) und dürfte ebenfalls keine Sonderregel zu gebrauchten Bauprodukten führen — dies konnte anhand der aktuell geltenden konsolidierten Fassung NICHT verifiziert werden. Die amtlichen Landesportale (voris.niedersachsen.de) waren für automatisierten Zugriff nicht erreichbar; nur eine veraltete 2012er-PDF-Fassung war zugänglich.
- Wortlautbeleg (Originalsprache): "Bauprodukte dürfen für die Errichtung, Änderung und Instandhaltung baulicher [Anlagen] … [verwendet werden, wenn sie] aufgrund des Übereinstimmungsnachweises nach § 22 das Übereinstimmungszeichen [tragen]" (§17 Abs.1, NBauO-Fassung 2012-04-03 — AS-ENACTED, NICHT as-amended zum Stichtag)
- Beleg-Quelle: B4 für die AKTUELLE Fassung (nur Fundstellenliste, Volltext wegen Zugriffsfehler nicht eingesehen); B1 nur für die überholte 2012er Version · Zugänglichkeit: paywalled-nicht-eingesehen (aktuelle Fassung hinter Wolters-Kluwer-Frontend) für die geltende Fassung; frei-primär nur für die veraltete Version · Bindungsakt: NBauO ist selbst der bindende Akt (Landesgesetz)
- Quelle: Tier 1 (Land Niedersachsen) · veraltete Fassung https://www.ms.niedersachsen.de/download/67044/NBauO_vom_03.04.2012.pdf · aktuelle Fassung (nicht eingesehen) über voris.niedersachsen.de · Fassung(as-amended) NICHT ermittelt · Zugriff 2026-08-11 (nur veraltete Version erfolgreich)
- Status: unklar für aktuelle Fassung (eingesehene Version erkennbar überholt) · Datum: 2012-04-03 (eingesehene Fassung)
- Sub-Ebene: Stichprobe [Niedersachsen — nur veraltete Fassung] / nicht erhoben [Niedersachsen aktuelle Fassung; alle übrigen 15 Länder]
- Relationen: setzt (mutmaßlich, nicht verifiziert) um REG-DE-1-010, REG-DE-2-002
- Konfidenz: unklar (explizit als Erhebungslücke markiert)

---

## Feld 2 · Bautechnische Zulassung/Standsicherheit

### REG-DE-2-001 · Allgemeine/vorhabenbezogene Bauartgenehmigung (aBG/vBG) — MBO § 16a Abs. 2
- Titel: Musterbauordnung (MBO), § 16a Abs. 2 — Bauarten
- Fundstelle: § 16a Abs. 2 MBO, Fassung November 2002, zuletzt geändert durch BMK-Beschluss vom 26./27.9.2024
- A: sub-national (MBO ohne eigene Rechtskraft; bindend sind die 16 LBOs) · B: 2 Bautechnische Zulassung/Standsicherheit · C: materialübergreifend · D: Gesetz (Landesbauordnung; MBO-Wortlaut weitgehend wortgleich übernommen)
- E: Planung/Nachweis, Einbau/Abnahme
- F1 (E3): ermöglichend (eröffnet Zulassungsweg für Bauarten ohne Technische Baubestimmung oder allgemein anerkannte Regel der Technik — genau der Fall bei vielen Wiederverwendungs-Konstruktionen) · F2 (E3): bedingend (vBG gilt nur für das einzelne Bauvorhaben; erneuter Antrag bei Folgeprojekten nötig — strukturell wiederkehrender Prüf-/Kostenaufwand statt Skalierung)
- G: Einzelfallzulassung (explizit, E1, §16a Abs.2); rechnerischer Nachweis, Sichtprüfung, Probenahme/Materialprüfung als Antragsgrundlage (inferiert, E3)
- Kernaussage: § 16a Abs. 2 MBO verlangt für Bauarten, die von Technischen Baubestimmungen wesentlich abweichen oder für die keine allgemein anerkannten Regeln der Technik bestehen, entweder eine allgemeine Bauartgenehmigung (aBG, DIBt) oder eine vorhabenbezogene Bauartgenehmigung (vBG, oberste Landesbauaufsichtsbehörde). Seit 1.1.2026 erteilt das DIBt ZiE/vBG zusätzlich zu Berlin auch für Mecklenburg-Vorpommern und Niedersachsen.
- Wortlautbeleg (Originalsprache): "Bauarten, die von Technischen Baubestimmungen nach § 85 a Absatz 2 Nr. 2 oder Nr. 3 Buchstabe a) wesentlich abweichen oder für die es allgemein anerkannte Regeln der Technik nicht gibt, dürfen bei der Errichtung, Änderung und Instandhaltung baulicher Anlagen nur angewendet werden, wenn für sie 1. eine allgemeine Bauartgenehmigung durch das Deutsche Institut für Bautechnik oder 2. eine vorhabenbezogene Bauartgenehmigung durch die oberste Bauaufsichtsbehörde erteilt worden ist."
- Beleg-Quelle: B0 (MBO-PDF vollständig gelesen, W0-Pilot) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (MBO/LBO selbst)
- Quelle: Tier 1 · https://bvpi.de/bvpi/downloads/MBO.pdf (§16a); DIBt https://www.dibt.de/de/wir-bieten/zulassungen-etas-und-mehr/zustimmung-im-einzelfall-zie-und-vorhabenbez-bauartgenehmigung-vbg · Fassung(as-amended) 2024-09-27 · Zugriff 2026-08-11
- Status: in Kraft · Beschlussdatum 2024-09-27; Zuständigkeitserweiterung DIBt für MV/Niedersachsen seit 2026-01-01
- Sub-Ebene: Stichprobe [Berlin (§§16a, 20 BauO Bln); Niedersachsen (§§16a, 20 NBauO); Mecklenburg-Vorpommern (§§16a, 20 LBauO M-V); Baden-Württemberg (LBO i.V.m. VwV TB BW)] / nicht erhoben [übrige 12 Länder]
- Relationen: konkretisiert REG-DE-2-004 (§85a-Mechanismus/VV TB); wird kombiniert mit REG-DE-2-002
- Konfidenz: gesichert (Wortlaut und Zuständigkeitswechsel 2026); abgeleitet (Länder-Gleichlauf über Stichprobe hinaus)

---

### REG-DE-2-002 · Zustimmung im Einzelfall (ZiE) — MBO § 20
- Titel: Musterbauordnung (MBO), § 20 — Nachweis der Verwendbarkeit von Bauprodukten im Einzelfall
- Fundstelle: § 20 MBO, i.V.m. § 17 Abs. 1, § 16b Abs. 1 MBO
- A: sub-national · B: 2 Bautechnische Zulassung/Standsicherheit · C: materialübergreifend · D: Gesetz (Landesbauordnung; MBO-Referenztext)
- E: Planung/Nachweis, Einbau/Abnahme
- F1 (E3): ermöglichend (ZiE ist explizit für den Fall konstruiert, dass ein Bauprodukt keinen anderen gültigen Verwendbarkeitsnachweis hat — bei gebrauchten/wiederaufgearbeiteten Bauprodukten der Regelfall) · F2 (E3): bedingend (Gültigkeit „im Einzelfall", keine automatische Übertragbarkeit auf andere Projekte mit demselben Bauteiltyp)
- G: Einzelfallzulassung (explizit, E1); Nachweis der Verwendbarkeit durch Dokumentenlage, Sichtprüfung, zerstörungsfreie Prüfung, Probenahme/Materialprüfung und rechnerischen Nachweis (inferiert, E3)
- Kernaussage: § 20 MBO erlaubt mit Zustimmung der obersten Bauaufsichtsbehörde die Verwendung eines Bauprodukts im Einzelfall, wenn dessen Verwendbarkeit nachgewiesen ist und kein Regelnachweis (Technische Baubestimmung, abZ, abP) vorliegt. Für Bauprodukte ist ZiE das funktionale Gegenstück zur vBG für Bauarten; beide werden bei Wiederverwendung typischerweise gemeinsam beantragt.
- Wortlautbeleg (Originalsprache): "Mit Zustimmung der obersten Bauaufsichtsbehörde dürfen unter den Voraussetzungen des § 17 Abs. 1 im Einzelfall Bauprodukte verwendet werden, wenn ihre Verwendbarkeit im Sinne des § 16b Absatz 1 nachgewiesen ist. Wenn Gefahren im Sinne des § 3 Satz 1 nicht zu erwarten sind, kann die oberste Bauaufsichtsbehörde im Einzelfall erklären, dass ihre Zustimmung nicht erforderlich ist."
- Beleg-Quelle: B0 (W0-Pilot) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://bvpi.de/bvpi/downloads/MBO.pdf (§20); DIBt-Verfahrenspraxis: https://www.dibt.de/de/wir-bieten/zulassungen-etas-und-mehr/zustimmung-im-einzelfall-zie-und-vorhabenbez-bauartgenehmigung-vbg · Fassung(as-amended) 2024-09-27 · Zugriff 2026-08-11
- Status: in Kraft · Beschlussdatum 2024-09-27; Zuständigkeitserweiterung DIBt für MV/Niedersachsen seit 2026-01-01
- Sub-Ebene: Stichprobe [Berlin, Niedersachsen, Mecklenburg-Vorpommern, Baden-Württemberg] / nicht erhoben [übrige 12 Länder]
- Relationen: konkretisiert REG-DE-2-004; wird kombiniert mit REG-DE-2-001; kontrastiert mit REG-DE-2-003
- Konfidenz: gesichert

---

### REG-DE-2-003 · Allgemeine bauaufsichtliche Zulassung (abZ) — Referenzobjekt
- Titel: Musterbauordnung (MBO), § 18 — Allgemeine bauaufsichtliche Zulassung
- Fundstelle: § 18 MBO, i.V.m. § 17 Abs. 1 MBO
- A: sub-national · B: 2 Bautechnische Zulassung/Standsicherheit · C: materialübergreifend · D: Gesetz (Landesbauordnung; MBO-Referenztext)
- E: Inverkehrbringen, Planung/Nachweis
- F1 (E3): schweigend gegenüber Wiederverwendung (Norm für Serienprodukte konzipiert — 5-Jahres-Befristung, Probestücke, werkseigene Produktionskontrolle — adressiert nicht heterogene gebrauchte Bauteile) · F2 (E3): hemmend für Reuse (Praxis weicht deshalb auf ZiE aus, obwohl abZ nominell der Regelweg für nicht normierte Bauprodukte ist)
- G: rechnerischer Nachweis, Probenahme/Materialprüfung (explizit, E1, §18 Abs.2)
- Kernaussage: Das DIBt erteilt die abZ für Bauprodukte, deren Verwendbarkeit nachgewiesen ist; sie wird widerruflich und i. d. R. für 5 Jahre erteilt, verlangt Probestücke/Probeausführungen. Als Referenzobjekt zeigt es, warum für Wiederverwendung typischerweise die Einzelfall-Route (ZiE) gewählt wird: das Verfahren ist auf reproduzierbare, mehrfach herstellbare Produkte zugeschnitten.
- Wortlautbeleg (Originalsprache): "Das Deutsche Institut für Bautechnik erteilt unter den Voraussetzungen des § 17 Abs. 1 eine allgemeine bauaufsichtliche Zulassung für Bauprodukte, wenn deren Verwendbarkeit im Sinne des § 16b Abs. 1 nachgewiesen ist. […] Die allgemeine bauaufsichtliche Zulassung wird widerruflich und für eine bestimmte Frist erteilt, die in der Regel fünf Jahre beträgt."
- Beleg-Quelle: B0 (W0-Pilot) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://bvpi.de/bvpi/downloads/MBO.pdf (§18) · Fassung(as-amended) 2024-09-27 · Zugriff 2026-08-11
- Status: in Kraft · 2024-09-27
- Sub-Ebene: nicht erhoben (Referenzobjekt)
- Relationen: kollidiert funktional (nicht normtextlich) mit REG-DE-2-002 als Alternativweg
- Konfidenz: gesichert (Normtext); abgeleitet (Praxispräferenz für ZiE bei Reuse — aus REG-DE-2-010)

---

### REG-DE-2-004 · § 85a MBO / Verwaltungsvorschrift Technische Baubestimmungen (VV TB) — Bindungskette für Eurocodes/DIN-Normen
- Titel: Musterbauordnung § 85a (Technische Baubestimmungen); landesrechtlicher Vollzug durch länderspezifische VV TB, basierend auf der MVV TB des DIBt
- Fundstelle: § 85a Abs. 1–4 MBO
- A: sub-national (VV TB je Land einzeln erlassen; MVV TB unverbindliche Vorlage) · B: 2 Bautechnische Zulassung/Standsicherheit · C: materialübergreifend · D: Verwaltungsvorschrift (VV TB je Land); Ermächtigungsmechanismus §85a ist Gesetz (Landesbauordnung)
- E: Planung/Nachweis
- F1 (E3): bedingend (die VV TB ist der freie amtliche Akt, der eine kostenpflichtige Norm — Eurocode, DIN — zur „Technischen Baubestimmung" und damit verbindlich macht — Bindungsketten-Regel) · F2 (E3): schweigend hinsichtlich Reuse (ob DIN SPEC 91484, ISO 13822 oder EN 1990-2 in einer VV TB gelistet sind, ist NICHT verifiziert)
- G: Dokumentenlage (explizit, E1: §85a Abs.2 „Bezugnahmen auf technische Regeln und deren Fundstellen")
- Kernaussage: § 85a MBO erlaubt, die allgemeinen Anforderungen des § 3 MBO durch Technische Baubestimmungen zu konkretisieren, indem auf technische Regeln und deren Fundstellen Bezug genommen wird; diese Technischen Baubestimmungen „sind zu beachten" — das ist der Bindungsmechanismus, der eine privatrechtlich lizenzierte Norm bauordnungsrechtlich verbindlich macht. Für mehrere Länder wurden aktuelle VV-TB-Dokumente identifiziert; ihr konkreter Norminhalt (insb. Listung von Bestandsbewertungsnormen) wurde wegen technischer Extraktionsprobleme NICHT im Detail ausgewertet.
- Wortlautbeleg (Originalsprache): "Die Anforderungen nach § 3 können durch Technische Baubestimmungen konkretisiert werden. Die Technischen Baubestimmungen sind zu beachten. […] Die Konkretisierungen können durch Bezugnahmen auf technische Regeln und deren Fundstellen oder auf andere Weise erfolgen […]."
- Beleg-Quelle: B0 für §85a MBO selbst (W0-Pilot); B2 für einzelne Länder-VV-TB (Existenz/Fundstelle amtlich verzeichnet, Volltext-Detailinhalt nicht ausgewertet) · Zugänglichkeit: frei-primär (beides) · Bindungsakt: §85a MBO selbst ist der Mechanismus; die Länder-VV-TB sind die Vollzugsakte
- Quelle: Tier 1 · https://bvpi.de/bvpi/downloads/MBO.pdf (§85a); Länder-VV-TB u.a. https://fm.rlp.de/themen/baurecht-und-bautechnik/technische-baubestimmungen, https://www.berlin.de/sen/sbw/_assets/service/rechtsvorschriften/bereich-bauen/vvtbblnlesefassung.pdf; MVV TB: https://www.dibt.de/de/wir-bieten/technische-baubestimmungen · Fassung(as-amended) 2024-09-27 (MBO) · Zugriff 2026-08-11
- Status: in Kraft · MBO-Fassung 2024-09-27; Länder-VV-TB-Einzelstände variieren
- Sub-Ebene: Stichprobe [Rheinland-Pfalz, Berlin, Hessen, Brandenburg, Hamburg, Nordrhein-Westfalen — Existenz/Fundstelle bestätigt, Inhalt nicht ausgewertet] / nicht erhoben [übrige 10 Länder; Norminhalt auch der Stichprobe nicht ausgewertet]
- Relationen: setzt um §85a MBO auf Landesebene; ist Bindungsakt-Voraussetzung für REG-DE-2-005/006/007/011/012
- Konfidenz: gesichert (Mechanismus); unklar (konkrete Listung von Bestandsbewertungsnormen)

---

### REG-DE-2-005 · Eurocode-Nationale Anhänge / DIN EN 1990 ff. — Zugriffslücke (offen)
- Titel: DIN EN 1990 ff. mit Nationalen Anhängen (NA); Grundlagen der Tragwerksplanung
- Fundstelle: nicht ermittelt (Einzelparagraphen)
- A: EU/EEA (CEN-Erarbeitungsebene) mit nationaler Bindung über NA + VV TB · B: 2 Bautechnische Zulassung/Standsicherheit (explizit „Eurocode-NA" laut Taxonomie) · C: materialübergreifend · D: kein exakt passender Wert — Eurocodes sind keine hEN im CPR-Sinn; hier vorläufig als „nat.Norm" (mit NA) geführt
- E: Planung/Nachweis
- F1 (E3): unklar (Primärtext trotz mehrfacher Versuche in dieser und der Quellenkarten-Session nicht erreichbar) · F2 (E3): unklar
- G: rechnerischer Nachweis (inferiert, E3, aus allgemeinem Eurocode-Zweck)
- Kernaussage: Der direkte DIN-Katalogzugriff (din.de) war in mehreren Rechercheversuchen dieser und der vorgelagerten Quellenkarten-Session NICHT erfolgreich (wiederholt HTTP 404). Das Ersatzportal eurocode-online.de (DIN Media GmbH) wurde als funktionierender Zugang identifiziert, lieferte aber im abgerufenen Ausschnitt keine tabellarische Liste der Nationalen Anhänge. Damit bleibt die zentrale Frage, welche konkreten Eurocode-Teile/NA für Bestandsbewertung reuse-relevante Regelungen enthalten und ob/wie sie über VV TB (REG-DE-2-004) verbindlich gemacht sind, zum Stichtag 2026-08-11 UNGEKLÄRT. Dies ist ausdrücklich als Lücke, nicht als „schweigend" im Rechtssinn zu verstehen — die Rechtslage selbst wurde nicht eingesehen.
- Wortlautbeleg (Originalsprache): kein Wortlautzitat möglich (Primärtext nicht erreicht)
- Beleg-Quelle: B4 (reiner Katalog-/Existenzhinweis aus Projektwissen, in keiner Session durch Primärzugriff bestätigt) — bewusst NICHT als Faktum verwendbar
- Quelle: (kein belastbarer Primärlink) · Zugriff 2026-08-11 (erfolglos)
- Status: unklar
- Sub-Ebene: nicht erhoben
- Relationen: würde REG-DE-2-006/007 (Bestandsbewertungs-Teilnormen) fachlich umfassen; Bindungsakt-Voraussetzung REG-DE-2-004 ungeprüft
- Konfidenz: unklar — explizit als offene Lücke geführt, nicht als erfundene Regel

---

### REG-DE-2-006 · DIN EN 1990-2:2024 (Entwurf) — Bewertung von Bestandsbauten
- Titel: prEN 1990-2:2024 / DIN EN 1990-2:2024-02 „Eurocode — Grundlagen der Planung von Tragwerken und geotechnischen Bauwerken — Teil 2: Bewertung von Bestandsbauten"
- Fundstelle: Gesamtdokument, Entwurfsstadium (paywalled-nicht-eingesehen)
- A: EU/EEA (CEN-Entwicklungsebene); nationale Umsetzung mit NA steht aus · B: 2 Bautechnische Zulassung/Standsicherheit (explizit „Bestandsbewertung") · C: materialübergreifend · D: kein exakt passender Wert (Eurocode-Entwurf, weder hEN noch reguläre nat.Norm)
- E: Planung/Nachweis
- F1 (E3): unklar/im Entstehen (Entwurfsstatus, Inhalt nicht eingesehen) · F2 (E3): schweigend (noch keine Praxiswirkung, nicht in Kraft)
- G: rechnerischer Nachweis (inferiert, E3, aus Titel/Scope)
- Kernaussage: Am 5. Januar 2024 wurde der Norm-Entwurf prEN 1990-2 „Bewertung von Bestandsbauten" als eigener zweiter Teil von Eurocode 0 veröffentlicht. Damit entsteht erstmals ein spezifischer Eurocode-Teil für die Bewertung bestehender Tragwerke, der die bisher genutzte ISO 13822 (REG-DE-2-007) ergänzen oder ablösen könnte. Der Fertigstellungsstand des zugehörigen Nationalen Anhangs sowie ein etwaiges Inkrafttreten zum Stichtag 2026-08-11 wurden in dieser Erhebung NICHT verifiziert.
- Wortlautbeleg (Originalsprache): kein Wortlautzitat möglich (Entwurf paywalled); Titel gemäß DIN-Media-Katalog: "Eurocode - Basis of structural and geotechnical design - Part 2: Assessment of existing structures"
- Beleg-Quelle: B2 (DIN-Media-Katalogseite mit Ausgabedatum/Status/Seitenzahl gelesen, Volltext nicht eingesehen) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: keiner — solange kein NA vorliegt und keine VV TB darauf verweist, ist EN 1990-2 in Deutschland nicht bindend
- Quelle: Tier 1 · https://www.dinmedia.de/en/draft-standard/din-en-1990-2/375902695 · Fassung(as-amended) 2024-02 (Entwurf) · Zugriff 2026-08-11
- Status: Entwurf · Ausgabe 2024-02
- Sub-Ebene: entfällt (A=EU/EEA)
- Relationen: ersetzt (voraussichtlich, unbestätigt) oder ergänzt REG-DE-2-007; setzt REG-DE-2-004 (VV-TB-Listung) für Bindungswirkung in DE voraus
- Konfidenz: unklar (Entwurfsstatus volatil; NA-Fertigstellung nicht verifiziert)

---

### REG-DE-2-007 · ISO 13822 / DIN ISO 13822 — Bewertung bestehender Tragwerke
- Titel: ISO 13822 „Bases for design of structures — Assessment of existing structures", deutsche Übernahme als DIN ISO 13822
- Fundstelle: Gesamtdokument (paywalled-nicht-eingesehen)
- A: national (Übernahme einer ISO-Norm ohne EU/EEA-Spezifik) · B: 2 Bautechnische Zulassung/Standsicherheit (explizit „Bestandsbewertung") · C: materialübergreifend · D: nat.Norm (DIN-Übernahme einer ISO-Norm ohne CEN-Parallelnorm)
- E: Planung/Nachweis
- F1 (E3): schweigend/unklar (Inhalt nicht eingesehen) · F2 (E3): unklar
- G: rechnerischer Nachweis (inferiert, E3, aus Titel/Scope)
- Kernaussage: ISO 13822 (aktuelle Fassung 2010, ältere Fassung 2001 ebenfalls im Katalog) behandelt laut Titel die Grundlagen der Tragwerksplanung für die Bewertung bestehender Tragwerke und gilt in der Fachliteratur als einschlägige Grundlage für Bestandsbewertungen. Der Normtext wurde NICHT eingesehen (paywalled); das Verhältnis zu REG-DE-2-006 (Ablösung/Parallelgeltung/Ergänzung) bleibt offen.
- Wortlautbeleg (Originalsprache): kein Wortlautzitat möglich; Titel gemäß DIN-Media-Katalog: "ISO 13822 — Bases for design of structures — Assessment of existing structures"
- Beleg-Quelle: B4 (nur Existenz-/Katalognachweis) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: nicht ermittelt, ob und über welche Landes-VV-TB DIN ISO 13822 als Technische Baubestimmung eingeführt ist
- Quelle: Tier 1 (Katalog) · https://www.dinmedia.de/en/standard/iso-13822/134160525 (Fassung 2010-08) · Fassung(as-amended) 2010-08 · Zugriff 2026-08-11
- Status: unklar (Katalogstatus „gültig" nicht positiv bestätigt)
- Sub-Ebene: entfällt (A=national)
- Relationen: Verhältnis zu REG-DE-2-006 ungeklärt; Bindungsakt-Beziehung zu REG-DE-2-004 offen
- Konfidenz: unklar

---

### REG-DE-2-008 · VDI 6200 „Standsicherheit von Bauwerken — Regelmäßige Überprüfung"
- Titel: VDI 6200 — Standsicherheit von Bauwerken; Regelmäßige Überprüfung
- Fundstelle: Gesamtrichtlinie (paywalled-nicht-eingesehen)
- A: national · B: 2 Bautechnische Zulassung/Standsicherheit (explizit „Bestandsbewertung") · C: materialübergreifend · D: kein exakt passender Wert — private technische Richtlinie des VDI e.V., kein staatlicher Normgeber und kein DIN-Konsensverfahren; am ehesten „nat.Norm"-nah, aber nicht identisch
- E: Bestandserkundung, Planung/Nachweis
- F1 (E3): schweigend gegenüber Wiederverwendung/Materialrückgewinnung laut Übersichtsseite · F2 (E3): unklar (methodisch aber die einschlägige Anknüpfungsnorm für den rechnerischen Nachweis der Standsicherheit von Bestandstragwerken, auf die eine Bauteilwiederverwendung angewiesen wäre)
- G: rechnerischer Nachweis, Dokumentenlage (Bestandsdokumentation) (inferiert, E3, aus Übersichtsangaben)
- Kernaussage: VDI 6200 (Ausgabe Februar 2010, Status „überprüft und bestätigt", 39 Seiten) klassifiziert Gebäude nach Schadensfolgeklasse/Robustheitsklasse und fordert Bestandsdokumentation sowie Beurteilungskriterien für die Tragwerksbewertung im Bestand. Die VDI-eigene Übersichtsseite enthält KEINEN expliziten Bezug zu Bauteilwiederverwendung/Materialrückgewinnung.
- Wortlautbeleg (Originalsprache): kein Wortlautzitat aus dem kostenpflichtigen Volltext möglich; Titel/Status gemäß VDI-Richtlinienseite: "VDI 6200 — Standsicherheit von Bauwerken; Regelmäßige Überprüfung", Status „überprüft und bestätigt"
- Beleg-Quelle: B2 (VDI-eigene Richtlinienseite mit Titel/Datum/Status direkt gelesen, Volltext kostenpflichtig) · Zugänglichkeit: paywalled-nicht-eingesehen (Übersichtsseite frei, Volltext kostenpflichtig über Beuth/VDI) · Bindungsakt: KEIN Bindungsakt identifiziert oder geprüft — VDI 6200 wird in einzelnen Landes-Verwaltungsvorschriften (z.B. Bayern/M-V „Hinweise für die Überprüfung der Standsicherheit") als fachliche Referenz genannt (nicht im Volltext verifiziert)
- Quelle: Tier 1 · https://www.vdi.de/richtlinien/details/vdi-6200-standsicherheit-von-bauwerken-regelmaessige-ueberpruefung · Fassung(as-amended) 2010-02 · Zugriff 2026-08-11
- Status: in Kraft (Ausgabe 2010-02, bestätigt)
- Sub-Ebene: entfällt (A=national); Landes-Verwaltungsvorschriften (Bayern, M-V), die VDI 6200 referenzieren sollen, NICHT im Volltext gelesen
- Relationen: ergänzt REG-DE-2-004/006/007 methodisch
- Konfidenz: gesichert (Existenz/Status); unklar (Bindungsakt, Reuse-Bezug)

---

### REG-DE-2-009 · ARGEBAU-Hinweise — Nachweis der Standsicherheit beim Bauen im Bestand
- Titel: „Hinweise und Beispiele zum Vorgehen beim Nachweis der Standsicherheit beim Bauen im Bestand", Fachkommission Bautechnik der Bauministerkonferenz (ARGEBAU)
- Fundstelle: Gesamtdokument, insb. Ziff. 2 (Grundlagen/Bestandsschutz), Ziff. 5 (Regeln für Bauprodukte)
- A: national (16-Länder-ARGEBAU-Empfehlung, weder Bundes- noch reines Landesdokument) · B: 2 Bautechnische Zulassung/Standsicherheit · C: materialübergreifend · D: Merkblatt
- E: Planung/Nachweis
- F1 (E3): ermöglichend (verankert Bestandsschutz-Grundsatz, begrenzt Pflicht zur Anwendung aktueller Technischer Baubestimmungen grundsätzlich auf unmittelbar betroffene Bauteile) · F2 (E3): bedingend (sobald Bauprodukte ohne gültigen Verwendbarkeitsnachweis verwendet werden, verweist das Dokument explizit auf abZ oder ZiE)
- G: Dokumentenlage, rechnerischer Nachweis (explizit, E1, Ziff. 3–4)
- Kernaussage: Das Dokument (Fassung April 2008) konkretisiert, in welchem Umfang bei Änderungen aktuelle Technische Baubestimmungen anzuwenden sind und wie mit Bestandsschutz umzugehen ist. Ziff. 5 stellt für Bauprodukte klar: Werden beim Bauen im Bestand Bauprodukte ohne gültigen bauaufsichtlichen Verwendbarkeitsnachweis verwendet, ist dies über abZ oder ZiE zu regeln — denselben Weg, der auch für wiederverwendete Bauprodukte einschlägig ist, ohne den Begriff „Wiederverwendung" selbst zu nennen.
- Wortlautbeleg (Originalsprache): "Beim Bauen im Bestand sind bei der Errichtung neuer Teile der baulichen Anlage nur Bauprodukte zu verwenden, die den aktuellen bauaufsichtlichen Vorschriften entsprechen. Wird hiervon abgewichen, d.h. werden Bauprodukte verwendet, für die ein bauaufsichtlich gültiger Verwendbarkeitsnachweis nicht oder nicht mehr vorliegt, so ist dies über eine allgemeine bauaufsichtliche Zulassung oder eine Zustimmung im Einzelfall zu regeln."
- Beleg-Quelle: B0 (vollständig gelesen, W0-Pilot) · Zugänglichkeit: frei-primär
- Quelle: Tier 1 · https://mlw.baden-wuerttemberg.de/fileadmin/redaktion/m-mlw/intern/Dateien/03_Bauen-Wohnen/Bautechnik_und_Bauoekologie/Nachweis_der_Standsicherheit_beim_Bauen_im_Bestand_2008-04.pdf (Mirror Baden-Württemberg) · Fassung(as-amended) 2008-04 · Zugriff 2026-08-11
- Status: in Kraft · 2008-04 (Aktualität/Nachfolgefassung nicht abschließend geprüft)
- Sub-Ebene: entfällt gemäß A=national; faktisch nur über einen Landes-Mirror (BW) zugänglich
- Relationen: konkretisiert §3/§85a-MBO-Systematik; steht in Zusammenhang mit REG-DE-2-002/003; ergänzt REG-DE-2-010
- Konfidenz: gesichert (Wortlaut); unklar (Aktualität)

---

### REG-DE-2-010 · Leitfaden zur Wiederverwendung tragender Bauteile (Baden-Württemberg)
- Titel: „Leitfaden zur Wiederverwendung tragender Bauteile — Stahlbau, Holzbau", Ministerium für Landesentwicklung und Wohnen Baden-Württemberg (MLW)
- Fundstelle: Gesamtdokument, insb. Ziff. 1 (Allgemeine Anmerkungen), Ziff. 2 (Bestandsanalyse: Erstprüfung/Detailprüfung)
- A: sub-national (Baden-Württemberg) · B: 2 Bautechnische Zulassung/Standsicherheit · C: Baustahl, Holz (Hauptteil materialneutral, materialspezifische Anhänge) · D: Merkblatt (im Text ausdrücklich als „rechtlich nicht verbindliche Empfehlung" bezeichnet)
- E: Bestandserkundung, Rückbau/Sicherung, Aufbereitung/Prüfung, Planung/Nachweis, Einbau/Abnahme
- F1 (E3): ermöglichend (expliziter Zweck: Hilfestellung für ZiE/vBG-Verfahren bei Reuse, da normierte technische Grundlagen fehlen) · F2 (E3): ermöglichend, begrenzte Reichweite (unverbindliches Merkblatt eines einzelnen Landes)
- G: Dokumentenlage, Sichtprüfung, zerstörungsfreie Prüfung, Probenahme/Materialprüfung, rechnerischer Nachweis, Einzelfallzulassung (alle explizit, E1, als gestuftes Verfahren)
- Kernaussage: Der Leitfaden (Stand 2025-04-30) beschreibt eine strukturierte, materialneutrale Vorgehensweise für die Wiederverwendung tragender Bauteile im Hochbau, ausdrücklich als Hilfestellung bei der Antragstellung für ZiE kombiniert mit vBG. Er bestätigt primärquellenbasiert, dass REG-DE-2-001 und REG-DE-2-002 in der Praxis als kombiniertes Verfahren für Reuse-Bauteile eingesetzt werden.
- Wortlautbeleg (Originalsprache): "Es handelt sich hierbei um eine rechtlich nicht verbindliche Empfehlung. Sie bietet für Entwurfsverfasser, Fachplaner, Gutachter, Prüfingenieure, Prüfämter und Behörden eine Hilfestellung bei der Antragsstellung/Erteilung eines Ver- bzw. Anwendbarkeitsnachweises für die Wiederverwendung gebrauchter Bauteile in Form einer Zustimmung im Einzelfall (ZiE) kombiniert mit einer vorhabenbezogenen Bauartgenehmigung (vBg)." / "bislang normierte technische Grundlagen für die Wiederverwendung gebrauchter Bauteile fehlen."
- Beleg-Quelle: B0 (wesentliche Abschnitte gelesen, W0-Pilot) · Zugänglichkeit: frei-primär
- Quelle: Tier 1 · https://mlw.baden-wuerttemberg.de/fileadmin/redaktion/m-mlw/intern/Dateien/06_Service/Publikationen/Bauen_und_Wohnen/2025-04-30-MLW_Broschuere_TragendeBauteile-BF_LNF.pdf · Fassung(as-amended) 2025-04-30 · Zugriff 2026-08-11
- Status: in Kraft (als Empfehlung) · 2025-04-30
- Sub-Ebene: Stichprobe [Baden-Württemberg] / nicht erhoben [übrige 15 Länder]
- Relationen: konkretisiert REG-DE-2-001 + REG-DE-2-002; referenziert REG-DE-2-009; knüpft an DIN-SPEC-91484-Logik an (REG-DE-2-011)
- Konfidenz: gesichert

---

### REG-DE-2-011 · DIN SPEC 91484 — Pre-Demolition-Audit
- Titel: „Verfahren zur Erfassung von Bauprodukten als Grundlage für Bewertungen des Anschlussnutzungspotentials vor Abbruch- und Renovierungsarbeiten (Pre-Demolition-Audit)"
- Fundstelle: Gesamtdokument (paywalled-nicht-eingesehen)
- A: national · B: 2 Bautechnische Zulassung/Standsicherheit (Bestandserkundungsmethodik; berührt zugleich Feld 6 Normen/Regelwerke, s. Kernaussage) · C: materialübergreifend · D: kein exakt passender Wert — DIN SPEC durchläuft ein reduziertes Konsensverfahren, nicht das reguläre DIN-820-Normungsverfahren; hier vorläufig als „nat.Norm (reduziertes Konsensverfahren)" geführt
- E: Bestandserkundung
- F1 (E3): ermöglichend (schafft laut amtlichem Titel — jetzt über DIN Media B2-belegt — ein Erfassungsverfahren für Anschlussnutzungspotential vor Abbruch) · F2 (E3): unklar (keine Primärquelle zur tatsächlichen Verbreitung/Verbindlichkeit in der Praxis)
- G: Dokumentenlage, Sichtprüfung (inferiert, E3, aus Sekundärbeschreibung „zweistufiges Verfahren: Vorprüfung/Detailprüfung")
- Kernaussage: DIN SPEC 91484 (Erscheinungsdatum September 2023, Status „[CURRENT]") definiert ein Verfahren zur Erfassung von Bauprodukten vor Abbruch-/Renovierungsarbeiten als Grundlage für die Bewertung ihres Wiederverwendungspotentials. Der amtliche Titel selbst bestätigt den Gegenstand; Verfahrensdetails (zweistufig: Vorprüfung/Detailerhebung) stammen aus Sekundärquellen. Ein Bindungsakt (VV-TB-Listung) wurde NICHT identifiziert — freiwillige Marktspezifikation ohne bauordnungsrechtliche Bindungswirkung nach bisherigem Kenntnisstand, aber keine vollständige 16-Länder-VV-TB-Suche durchgeführt.
- Wortlautbeleg (Originalsprache): amtlicher Titel gemäß DIN-Media-Produktseite: "Verfahren zur Erfassung von Bauprodukten als Grundlage für Bewertungen des Anschlussnutzungspotentials vor Abbruch- und Renovierungsarbeiten (Pre-Demolition-Audit); Text Deutsch und Englisch"
- Beleg-Quelle: B2 (amtliche DIN-Media-Produktseite mit Titel/Datum/Status direkt gelesen; Normtext selbst kostenpflichtig) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: kein Bindungsakt identifiziert (Stand dieser Erhebung)
- Quelle: Tier 1 · https://www.dinmedia.de/en/technical-rule/din-spec-91484/371235753 · Fassung(as-amended) 2023-09 · Zugriff 2026-08-11
- Status: in Kraft · 2023-09
- Sub-Ebene: entfällt (A=national)
- Relationen: konkretisiert Prozessschritt „Bestandserkundung" vor REG-DE-2-001/002/010; Bindungsakt zu REG-DE-2-004 nicht verifizierbar; thematisch fortgesetzt durch REG-DE-2-012
- Konfidenz: unklar (Inhalt nur sekundärquellenbasiert; Existenz/Titel/Datum gesichert)

---

### REG-DE-2-012 · DIN SPEC 91525 — Anschlussnutzungskonzept (Post-Use Concept)
- Titel: „Anschlussnutzungskonzept für Bauprodukte aus Bestandsgebäuden; Text Deutsch und Englisch" / engl. „Post-Use Concept (PUC) for construction products from existing buildings"
- Fundstelle: Gesamtdokument (paywalled-nicht-eingesehen)
- A: national · B: 2 Bautechnische Zulassung/Standsicherheit (thematische Fortsetzung von REG-DE-2-011: Erfassung → Konzept für Weiternutzung; berührt zugleich Feld 6) · C: materialübergreifend · D: kein exakt passender Wert — DIN SPEC (reduziertes Konsensverfahren)
- E: Bestandserkundung, Planung/Nachweis
- F1 (E3): ermöglichend (laut amtlichem Titel ein strukturiertes Konzept für die Anschlussnutzung von Bauprodukten aus Bestandsgebäuden) · F2 (E3): unklar (Neuerscheinung Februar 2026, keine Praxiserfahrung dokumentiert)
- G: Dokumentenlage (inferiert, E3, aus Titel „Konzept")
- Kernaussage: DIN SPEC 91525 (Erscheinungsdatum Februar 2026, Status „[CURRENT]") ist laut amtlichem Titel ein „Anschlussnutzungskonzept" für Bauprodukte aus Bestandsgebäuden — die konsequente Fortsetzung von DIN SPEC 91484. Der hohe Reuse-Bezug ist bereits im Titel textbelegt (E1 für den Gegenstand); der Einzelinhalt wurde nicht eingesehen. Als sehr junge Norm (wenige Monate vor dem Stichtag 2026-08-11 erschienen) liegt noch keine Aussage zur Verbreitung vor.
- Wortlautbeleg (Originalsprache): amtlicher Titel gemäß DIN-Media-Produktseite: "Anschlussnutzungskonzept für Bauprodukte aus Bestandsgebäuden; Text Deutsch und Englisch"
- Beleg-Quelle: B2 (amtliche DIN-Media-Produktseite mit Titel/Datum/Status direkt gelesen) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: wie REG-DE-2-011 — kein VV-TB-Bindungsakt identifiziert oder geprüft
- Quelle: Tier 1 · https://www.dinmedia.de/en/technical-rule/din-spec-91525/397760893 · Fassung(as-amended) 2026-02 · Zugriff 2026-08-11
- Status: in Kraft · 2026-02
- Sub-Ebene: entfällt (A=national)
- Relationen: setzt fort/konkretisiert REG-DE-2-011
- Konfidenz: unklar (Inhalt nicht eingesehen; Existenz/Titel/Datum gesichert)

---

## Feld 3 · Abfall-/Stoffrecht

### REG-DE-3-001 · KrWG — Abfallbegriff und Wiederverwendungsbegriff
- Titel: Gesetz zur Förderung der Kreislaufwirtschaft und Sicherung der umweltverträglichen Bewirtschaftung von Abfällen (Kreislaufwirtschaftsgesetz — KrWG)
- Fundstelle: § 3 Abs. 1, 2, 3 (Abfallbegriff/Entledigung), § 3 Abs. 21 (Wiederverwendung), § 3 Abs. 24 (Vorbereitung zur Wiederverwendung)
- A: national · B: 3 Abfall-/Stoffrecht (Grundnorm/Begriffsnorm mit Gatekeeper-Funktion für das gesamte Feld) · C: materialübergreifend · D: Gesetz
- E: Bestandserkundung, Abfallstatus (bzw. dessen Vermeidung)
- F1 (E3): ermöglichend (die Definition „Wiederverwendung" in Abs. 21 erfasst ausdrücklich nur Erzeugnisse/Bestandteile, die KEINE Abfälle sind; ein Bauteil, das ohne Entledigungswillen direkt ausgebaut und wiedereingebaut wird, verlässt den KrWG-Anwendungsbereich vollständig) · F2 (E3): hemmend (die Abgrenzung, ob beim Ausbau ein „Entledigungswille" vorliegt, ist einzelfallabhängig und im Vollzug für Bauleitung/Abbruchunternehmen unsicher zu beurteilen; im Zweifel wird vorsorglich über Entsorgungswege statt Direktweiterverwendung disponiert)
- G: Dokumentenlage (Verwendungsnachweis/Zweckbestimmung anhand Verkehrsanschauung) — inferiert (E3, Text nennt kein Nachweisformat für die Nicht-Abfall-Eigenschaft)
- Kernaussage: § 3 KrWG definiert Abfall über den (potenziellen) Entledigungswillen des Besitzers. Ein Bauteil, das ohne diesen Willen unmittelbar am selben Ort oder andernorts für denselben Zweck weiterverwendet wird, ist begrifflich niemals Abfall und damit von vornherein außerhalb des Regelungsbereichs von KrWG, ErsatzbaustoffV und GewAbfV. Erst wenn ein Bauteil zunächst zu Abfall geworden ist, greift die enger gefasste „Vorbereitung zur Wiederverwendung" (Abs. 24).
- Wortlautbeleg (Originalsprache): "(1) Abfälle im Sinne dieses Gesetzes sind alle Stoffe oder Gegenstände, derer sich ihr Besitzer entledigt, entledigen will oder entledigen muss. … (21) Wiederverwendung im Sinne dieses Gesetzes ist jedes Verfahren, bei dem Erzeugnisse oder Bestandteile, die keine Abfälle sind, wieder für denselben Zweck verwendet werden, für den sie ursprünglich bestimmt waren. … (24) Vorbereitung zur Wiederverwendung im Sinne dieses Gesetzes ist jedes Verwertungsverfahren der Prüfung, Reinigung oder Reparatur, bei dem Erzeugnisse oder Bestandteile von Erzeugnissen, die zu Abfällen geworden sind, so vorbereitet werden, dass sie ohne weitere Vorbehandlung wieder für denselben Zweck verwendet werden können, für den sie ursprünglich bestimmt waren."
- Beleg-Quelle: B0 (W0-Pilot) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Gesetz selbst)
- Quelle: Tier 1 · https://www.gesetze-im-internet.de/krwg/__3.html · Fassung(as-amended) 2023-03-02 (§3 zuletzt geändert durch Art. 5 G v. 2.3.2023 BGBl. I Nr. 56); Gesamt-KrWG zusätzlich betroffen von zwei textlich bereits nachgewiesenen, redaktionell noch nicht abschließend bearbeiteten Änderungsgesetzen (Art. 2 G v. 13.7.2026 BGBl. I Nr. 207; Art. 17 G v. 22.7.2026 BGBl. I Nr. 224) · Zugriff 2026-08-11
- Status: in Kraft · Fassung geprüft 2026-08-11
- Sub-Ebene: entfällt (A=national)
- Relationen: konkretisiert REG-DE-3-002 (§5), REG-DE-3-004 (EBV), REG-DE-3-006 (GewAbfV) als vorgelagerte Anwendbarkeits-Weiche
- Konfidenz: gesichert

---

### REG-DE-3-002 · KrWG — Ende der Abfalleigenschaft (§ 5)
- Titel: Kreislaufwirtschaftsgesetz (KrWG)
- Fundstelle: § 5 Abs. 1, Abs. 2
- A: national · B: 3 Abfall-/Stoffrecht · C: materialübergreifend · D: Gesetz (Abs.1 unmittelbar geltende Legaldefinition; Abs.2 Verordnungsermächtigung)
- E: Aufbereitung/Prüfung, Inverkehrbringen
- F1 (E3): bedingend (eröffnet Weg zurück zum Nicht-Abfall-Status nach Verwertung, aber nur unter kumulativen Voraussetzungen; für ganze Bauteile existiert keine eigene Abfallende-Rechtsverordnung nach Abs.2 — nur die EBV für mineralische Ersatzbaustoffe konkretisiert Abs.2 punktuell) · F2 (E3): schweigend (mangels bauteilspezifischer Abfallende-Verordnung bietet §5 für ein bereits zu Abfall gewordenes, aber intaktes Bauteil in der Praxis keinen bekannten/genutzten Vollzugspfad)
- G: rechnerischer Nachweis/Materialprüfung/Erklärung Dritter (Konformitätserklärung, Abs.2 Nr.5) — explizit (E1, steht in Abs.2 Nr.1–5)
- Kernaussage: § 5 Abs. 1 KrWG beendet die Abfalleigenschaft eines Stoffes nach Verwertungsverfahren, wenn u.a. Markt/Nachfrage, technische Anforderungen und Schadlosigkeit erfüllt sind; Abs. 2 ermächtigt die Bundesregierung zu konkretisierenden Rechtsverordnungen. Die einzige bislang erlassene bauwesenrelevante Abfallende-Verordnung ist die ErsatzbaustoffV — und diese erfasst nach eigenem Anwendungsbereich ausdrücklich nur mineralische Ersatzbaustoffe, nicht ganze Bauteile.
- Wortlautbeleg (Originalsprache): "(1) Die Abfalleigenschaft eines Stoffes oder Gegenstandes endet, wenn dieser ein Recycling oder ein anderes Verwertungsverfahren durchlaufen hat und so beschaffen ist, dass 1. er üblicherweise für bestimmte Zwecke verwendet wird, 2. ein Markt für ihn oder eine Nachfrage nach ihm besteht, 3. er alle für seine jeweilige Zweckbestimmung geltenden technischen Anforderungen sowie alle Rechtsvorschriften und anwendbaren Normen für Erzeugnisse erfüllt sowie 4. seine Verwendung insgesamt nicht zu schädlichen Auswirkungen auf Mensch oder Umwelt führt."
- Beleg-Quelle: B0 (W0-Pilot) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Gesetz)
- Quelle: Tier 1 · https://www.gesetze-im-internet.de/krwg/__5.html · Fassung(as-amended) 2023-03-02 · Zugriff 2026-08-11
- Status: in Kraft · Fassung geprüft 2026-08-11
- Sub-Ebene: entfällt
- Relationen: setzt um Abfallrahmenrichtlinie 2008/98/EG Art. 6 (EU-Ebene, nicht vertieft); wird konkretisiert durch REG-DE-3-004 (nur mineralische Ersatzbaustoffe — Regelungslücke für Bauteile)
- Konfidenz: gesichert (Wortlaut); abgeleitet (Regelungslücken-Aussage — Negativbefund)

---

### REG-DE-3-003 · KrWG — Nebenprodukte (§ 4)
- Titel: Kreislaufwirtschaftsgesetz (KrWG)
- Fundstelle: § 4 Abs. 1, Abs. 2
- A: national · B: 3 Abfall-/Stoffrecht · C: materialübergreifend · D: Gesetz
- E: Abfallstatus (bzw. dessen Vermeidung)
- F1 (E3): ermöglichend, aber eng (schafft eine zweite Vermeidungsroute neben dem Wiederverwendungsbegriff des §3 Abs.21 — allerdings nur für Stoffe, die BEI DER HERSTELLUNG als integraler Bestandteil eines Produktionsprozesses anfallen, nicht für Bauteile, die nach Ende ihrer Nutzungsphase ausgebaut werden) · F2 (E3): schweigend für den hier untersuchten Fall (ausgebaute Bestandsbauteile sind keine Herstellungs-Nebenprodukte im Sinne der Norm; die Nebenprodukt-Route ist für Produktionsabfälle konzipiert, nicht für End-of-Use-Bauteile)
- G: Dokumentenlage (Nachweis der Voraussetzungen Abs.1 Nr.1–4) — inferiert (E3, kein explizites Nachweisformat im Text)
- Kernaussage: § 4 Abs. 1 KrWG erlaubt, dass ein bei der Herstellung anfallender Stoff als Nebenprodukt (nicht Abfall) gilt, wenn Weiterverwendung gesichert ist, keine über normales Verfahren hinausgehende Vorbehandlung nötig ist, er integraler Bestandteil des Herstellungsprozesses ist und die Verwendung rechtmäßig erfolgt. Für die Wiederverwendung ganzer Bestandsbauteile ist diese Route strukturell nicht einschlägig, da sie auf Herstellungsprozesse, nicht auf den End-of-Use-Ausbau eines Bauteils zugeschnitten ist — Regelungslücke analog zu REG-DE-3-002.
- Wortlautbeleg (Originalsprache): kein wörtliches Volltextzitat in dieser Erhebung gesichert (per WebFetch-Zusammenfassung paraphrasiert: "Ein bei der Herstellung anfallender Stoff gilt als Nebenprodukt … wenn: Weiterverwendung gesichert ist, keine über normales Verfahren hinausgehende Vorbehandlung erforderlich ist, er als integraler Bestandteil des Herstellungsprozesses entsteht und die Verwendung rechtmäßig erfolgt") — Wortlautbeleg im engeren Sinn (E1) für die Extraktionsstufe/Nacherhebung offen
- Beleg-Quelle: B1 (amtliche Seite direkt gelesen, aber per Fetch-Werkzeug paraphrasiert statt wörtlich zitiert) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Gesetz)
- Quelle: Tier 1 · https://www.gesetze-im-internet.de/krwg/BJNR021210012.html (§4) · Fassung(as-amended) 2023-03-02 · Zugriff 2026-08-11
- Status: in Kraft · Fassung geprüft 2026-08-11
- Sub-Ebene: entfällt
- Relationen: ergänzt REG-DE-3-001 (Vermeidungsrouten); Abgrenzung zu REG-DE-3-002 (Abfallende nach Verwertung, hier: Vermeidung vor Abfallwerdung)
- Konfidenz: abgeleitet (Wortlaut nicht wörtlich verifiziert, nur paraphrasiert — Nacherhebung mit direktem Volltextzugriff empfohlen)

---

### REG-DE-3-004 · ErsatzbaustoffV — Anwendungsbereich (nur mineralische Ersatzbaustoffe)
- Titel: Verordnung über Anforderungen an den Einbau von mineralischen Ersatzbaustoffen in technische Bauwerke (Ersatzbaustoffverordnung — ErsatzbaustoffV/EBV)
- Fundstelle: § 1 Abs. 1, 2; § 2 Nr. 1 (Begriffsbestimmung „mineralischer Ersatzbaustoff")
- A: national · B: 3 Abfall-/Stoffrecht (Berührung zu Feld 6 über FGSV-Regelwerke/Anlagen) · C: Mauerwerk/mineralisch (Randberührung Baustahl über Hütten-/Stahlwerksschlacken) · D: RVO (Rechtsverordnung mit Zustimmung des Bundesrates, Teil der Mantelverordnung)
- E: Aufbereitung/Prüfung, Einbau/Abnahme, Betrieb/Dokumentation
- F1 (E3): schweigend gegenüber ganzen Bauteilen (Anwendungsbereich ausdrücklich auf „mineralische Ersatzbaustoffe" beschränkt — körniges/loses Material bestimmter Stoffgruppen für den Einbau in „technische Bauwerke"; ganze, wiederzuverwendende Bauteile sind weder positiv eingeschlossen noch ausdrücklich ausgeschlossen, weil sie begrifflich schon keine „mineralischen Ersatzbaustoffe" sind) · F2 (E3): hemmend (die EBV kanalisiert als einziger rechtssicherer Abfallende-Vollzugspfad Bau-/Abbruchmaterial systematisch in Richtung Aufbereitung zu losem Recycling-Material statt Elementerhalt)
- G: zerstörungsfreie Prüfung/Probenahme und Materialprüfung (Eignungsnachweis, WPK, Fremdüberwachung, §§4–13) — explizit (E1, für mineralische Ersatzbaustoffe); für ganze Bauteile: inferiert = nicht regelungsgegenständlich
- Kernaussage: Die EBV regelt Herstellung, Inverkehrbringen, Einbau und getrennte Sammlung mineralischer Ersatzbaustoffe im Sinne einer engen Legaldefinition. Sie ist die einzige nach § 5 Abs. 2 KrWG erlassene bauwesenrelevante Abfallende-Verordnung, deckt aber ausdrücklich keine ganzen, im Hochbau wiederzuverwendenden Bauteile ab — bestätigt die Falle „EBV gilt nur für mineralische Ersatzbaustoffe".
- Wortlautbeleg (Originalsprache): "(1) Die Vorschriften dieser Verordnung regeln im Hinblick auf mineralische Ersatzbaustoffe im Sinne des § 2 Nummer 1 die 1. Anforderungen an die Herstellung dieser mineralischen Ersatzbaustoffe … und an das Inverkehrbringen …, 2. Anforderungen an die Probenahme und Untersuchung … 3. Anforderungen an den Einbau dieser mineralischen Ersatzbaustoffe in technische Bauwerke sowie 4. Anforderungen an die getrennte Sammlung von mineralischen Abfällen aus technischen Bauwerken." / § 2 Nr. 1: "mineralischer Ersatzbaustoff: mineralischer Baustoff, der a) als Abfall oder als Nebenprodukt aa) in Aufbereitungsanlagen hergestellt wird oder bb) bei Baumaßnahmen … anfällt, b) unmittelbar oder nach Aufbereitung für den Einbau in technische Bauwerke geeignet und bestimmt ist und c) unmittelbar oder nach Aufbereitung unter die in den Nummern 18 bis 33 bezeichneten Stoffe fällt."
- Beleg-Quelle: B0 (W0-Pilot) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (RVO ist selbst der bindende Akt)
- Quelle: Tier 1 · https://www.gesetze-im-internet.de/ersatzbaustoffv/__1.html, __2.html · Fassung(as-amended) 2023-07-13 (geändert Art. 1 V v. 13.7.2023 BGBl. I Nr. 186, in Kraft seit 2023-08-01) · Zugriff 2026-08-11
- Status: in Kraft · seit 2023-08-01
- Sub-Ebene: entfällt
- Relationen: konkretisiert REG-DE-3-002; verdrängt als lex specialis REG-DE-3-006 (§8 Abs.1a GewAbfV) für dort genannte Stoffgruppen; Abgrenzung zu REG-DE-3-009 (DepV)
- Konfidenz: gesichert (Wortlaut Anwendungsbereich); abgeleitet (Nichterfassung ganzer Bauteile — logischer Schluss aus Definition)

---

### REG-DE-3-005 · ErsatzbaustoffV — Einbau-, Anzeige- und Katasterpflichten (§§ 19–23)
- Titel: Ersatzbaustoffverordnung (EBV), Abschnitt Einbau mineralischer Ersatzbaustoffe
- Fundstelle: § 19 (Grundsätzliche Anforderungen), § 20 (Zusätzliche Einbaubeschränkungen), § 21 (Behördliche Entscheidungen/Erlaubnisfreiheit), § 22 (Anzeigepflichten), § 23 (Ersatzbaustoffkataster)
- A: national · B: 3 Abfall-/Stoffrecht · C: Mauerwerk/mineralisch · D: RVO
- E: Einbau/Abnahme, Betrieb/Dokumentation
- F1 (E3): bedingend (Einbau grundsätzlich zulässig, aber an grundwasserschützende Bedingungen geknüpft; für bestimmte Schlacken/Aschen gelten Mindesteinbaumengen sowie Anzeige- und Dokumentationspflichten oberhalb von Schwellenwerten) · F2 (E3): bedingend (regulärer Einbau bei Einhaltung §§19–20 erlaubnisfrei nach WHG, aber Anzeige-/Kataster-Bürokratie ab 250 m³ bzw. in Wasserschutzgebieten schon ab kleineren Mengen — administrative Hürde für kleinteilige Wiederverwendungsprojekte, sofern mineralische Ersatzbaustoffe betroffen sind)
- G: Dokumentenlage (Anzeige, Kataster) + rechnerischer Nachweis (Mindestmengen) — explizit (E1)
- Kernaussage: § 19 schließt den Einbau in Wasserschutzgebieten Zone I bzw. Heilquellenschutzgebieten Zone I aus und macht den Einbau ansonsten von der Vermeidung nachteiliger Grundwasserveränderungen abhängig. § 21 stellt klar, dass bei Einhaltung der §§19–20 keine WHG-Erlaubnis nötig ist. §§22–23 verpflichten zur vorherigen Anzeige größerer Einbaumengen (i.d.R. ab 250 m³) und zur behördlichen Dokumentation im Ersatzbaustoffkataster.
- Wortlautbeleg (Originalsprache): "Der Bauherr oder der Verwender dürfen mineralische Ersatzbaustoffe oder Gemische in technische Bauwerke nur einbauen, wenn nachteilige Veränderungen der Grundwasserbeschaffenheit und schädliche Bodenveränderungen nach Maßgabe der Absätze 2 und 3 nicht zu besorgen sind." (§19 Abs.1); "In Wasserschutzgebieten der Zone I sowie in Heilquellenschutzgebieten der Zone I ist der Einbau von mineralischen Ersatzbaustoffen oder Gemischen in technische Bauwerke unzulässig." (§19); "Werden die Anforderungen nach den §§ 19 und 20 eingehalten, bedürfen Einbaumaßnahmen keiner Erlaubnis nach § 8 Absatz 1 des Wasserhaushaltsgesetzes." (§21)
- Beleg-Quelle: B1 (amtliche Seite in dieser Session direkt per WebFetch gelesen, mit Zitatmarkierung wiedergegeben) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (RVO selbst)
- Quelle: Tier 1 · https://www.gesetze-im-internet.de/ersatzbaustoffv/BJNR259810021.html (§§19–23) · Fassung(as-amended) 2023-07-13 · Zugriff 2026-08-11
- Status: in Kraft · seit 2023-08-01
- Sub-Ebene: entfällt
- Relationen: konkretisiert REG-DE-3-004; ergänzt § 24 EBV (getrennte Sammlung, s. REG-DE-3-006 Relation)
- Konfidenz: gesichert (Kernaussagen per Zitatmarkierung belegt), abgeleitet (vollständige Absatzgliederung nicht Wort für Wort geprüft)

---

### REG-DE-3-006 · GewAbfV — Bau- und Abbruchabfälle, Vorrang Wiederverwendung (§ 8)
- Titel: Verordnung über die Bewirtschaftung von gewerblichen Siedlungsabfällen und von bestimmten Bau- und Abbruchabfällen (Gewerbeabfallverordnung — GewAbfV)
- Fundstelle: § 1 Abs. 1–5 (Anwendungsbereich); § 8 Abs. 1, 1a, 2, 3 (Getrennte Sammlung, Vorbereitung zur Wiederverwendung und Recycling)
- A: national · B: 3 Abfall-/Stoffrecht · C: materialübergreifend (§8 Abs.1 listet 10 Fraktionen: Glas, Kunststoff, Metalle, Holz, Dämmmaterial, Bitumengemische, Gipsbaustoffe, Beton, Ziegel, Fliesen/Keramik) · D: RVO
- E: Rückbau/Sicherung, Abfallstatus, Aufbereitung/Prüfung, Betrieb/Dokumentation
- F1 (E3): bedingend, mit expliziter Reuse-Priorisierung (§8 Abs.1 verpflichtet zur getrennten Sammlung mit vorrangiger Zuführung „der Vorbereitung zur Wiederverwendung oder dem Recycling" — übernimmt die KrWG-Abfallhierarchie explizit auf Verordnungsebene; Abs.2 erlaubt Ausnahmen bei „rückbaustatischen oder rückbautechnischen Gründen") · F2 (E3): schweigend bezüglich ganzer, aus mehreren Materialien bestehender Bauteile (ein ausgebautes Fenster lässt sich keiner der zehn Einzelfraktionen als Ganzes zuordnen; die Norm setzt faktisch Zerlegung in Werkstoffe voraus)
- G: Dokumentenlage (Lagepläne, Lichtbilder, Liefer-/Wiegescheine)/Erklärung Dritter (Verwertungserklärung)/Darlegung techn. Unmöglichkeit oder wirtsch. Unzumutbarkeit — explizit (E1, Abs.3); Bagatellgrenze 10 m³ von Dokumentationspflicht ausgenommen (Abs.3, explizit)
- Kernaussage: § 8 GewAbfV verpflichtet zur getrennten Sammlung von zehn werkstoffbezogenen Bau- und Abbruchabfallfraktionen und schreibt vorrangig deren Vorbereitung zur Wiederverwendung oder Recycling vor; für die in §2 Nr.18–29, 32 EBV genannten mineralischen Stoffe verweist Abs.1a ausschließlich auf §24 EBV. Die Norm adressiert Materialströme, nicht die Erhaltung ganzer Bauteile, erkennt aber „rückbaustatische/rückbautechnische Gründe" als legitimen Grund für Abweichungen von der Fraktionstrennung an — eine Öffnung, die praktisch auch selektivem, bauteilschonendem Rückbau zugutekommen kann.
- Wortlautbeleg (Originalsprache): "(1) Erzeuger und Besitzer von Bau- und Abbruchabfällen haben die folgenden Abfallfraktionen jeweils getrennt zu sammeln, zu befördern und nach Maßgabe des § 8 Absatz 1 und § 9 Absatz 4 des Kreislaufwirtschaftsgesetzes vorrangig der Vorbereitung zur Wiederverwendung oder dem Recycling zuzuführen: … (1a) Soweit beim Rückbau, bei der Sanierung oder bei der Reparatur technischer Bauwerke Stoffe nach § 2 Nummer 18 bis 29 und 32 der Ersatzbaustoffverordnung … als Abfälle anfallen, gilt für die Getrenntsammlung, die Vorbereitung zur Wiederverwendung und das Recycling dieser Abfälle ausschließlich § 24 der Ersatzbaustoffverordnung. (2) … Die getrennte Sammlung der in Absatz 1 Satz 1 Nummer 8, 9 und 10 genannten mineralischen Abfälle ist insbesondere auch dann technisch nicht möglich, wenn sie aus rückbaustatischen oder rückbautechnischen Gründen ausscheidet."
- Beleg-Quelle: B0 (W0-Pilot) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (RVO)
- Quelle: Tier 1 · https://www.gesetze-im-internet.de/gewabfv_2017/__1.html, __8.html · Fassung(as-amended) 2025-09-30 dokumentarisch abschließend bearbeitet (Art. 9 Abs.3 G v. 30.9.2025 BGBl. I Nr. 233); Änderung durch Art. 3 Abs.4 G v. 13.7.2026 BGBl. 2026 I Nr. 207 textlich bereits eingearbeitet, dokumentarisch noch nicht abschließend bearbeitet · Zugriff 2026-08-11
- Status: in Kraft · Fassung geprüft 2026-08-11
- Sub-Ebene: entfällt
- Relationen: setzt um KrWG §8 Abs.1, §9 Abs.4; wird verdrängt durch REG-DE-3-004 (§24 EBV, lex specialis für dort genannte Stoffgruppen); kollidiert mit REG-DE-3-001 hinsichtlich fehlender Bauteil-Kategorie
- Konfidenz: gesichert

---

### REG-DE-3-007 · AVV — Abfallschlüsselsystematik Kapitel 17 (Bau- und Abbruchabfälle)
- Titel: Verordnung über das Europäische Abfallverzeichnis (Abfallverzeichnis-Verordnung, AVV)
- Fundstelle: Anhang, Kapitel 17 „Bau- und Abbruchabfälle (einschließlich Aushub von verunreinigten Standorten)", Unterkapitel 17 01–17 09
- A: national (Umsetzung der europäischen Abfallverzeichnis-Systematik der Entscheidung 2000/532/EG) · B: 3 Abfall-/Stoffrecht · C: materialübergreifend · D: RVO
- E: Dokumentenlage/Betrieb/Dokumentation (Grundlage für Nachweise nach NachwV, EBV, GewAbfV)
- F1 (E3): ermöglichend/neutral (liefert die Klassifikationssystematik, ohne selbst Handlungspflichten zu begründen — reine Referenznorm) · F2 (E3): bedingend (die Abfallschlüsselzuordnung bestimmt in der Praxis, welches Regime — EBV, GewAbfV, DepV — auf ein konkretes Material Anwendung findet; für Verbundbauteile ohne trennscharfen Einzelschlüssel entsteht Klassifikationsunsicherheit)
- G: Dokumentenlage (explizit, E1 — AVV liefert die Kodierungsgrundlage für alle Entsorgungsnachweise)
- Kernaussage: Kapitel 17 der AVV gliedert Bau- und Abbruchabfälle in neun Unterkapitel (17 01 Beton/Ziegel/Fliesen/Keramik; 17 02 Holz/Glas/Kunststoff; 17 03 Bitumengemische; 17 04 Metalle; 17 05 Boden/Steine/Baggergut; 17 06 Dämmmaterial/asbesthaltige Baustoffe; 17 08 Baustoffe auf Gipsbasis; 17 09 Sonstige Bau- und Abbruchabfälle) mit sechsstelligen Abfallschlüsseln, die gefährliche und nicht gefährliche Abfallarten unterscheiden (Kennzeichnung mit Stern). Diese Systematik ist rein werkstoffbezogen, nicht bauteilbezogen — ein Verbundbauteil (z.B. Fenster) lässt sich keinem Einzelschlüssel als Ganzes zuordnen.
- Wortlautbeleg (Originalsprache): Kapitelüberschrift laut Anhang: "17 Bau- und Abbruchabfälle (einschließlich Aushub von verunreinigten Standorten)"; Unterkapitel u.a. "17 01 Beton, Ziegel, Fliesen und Keramik", "17 02 Holz, Glas und Kunststoff", "17 06 Dämmmaterial und asbesthaltige Baustoffe"
- Beleg-Quelle: B1 (amtliche Seite in dieser Session direkt gelesen; vollständige Auflistung aller sechsstelligen Einzelschlüssel nicht Zeichen für Zeichen geprüft) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (RVO selbst)
- Quelle: Tier 1 · https://www.gesetze-im-internet.de/avv/BJNR337910001.html · Fassung(as-amended) 2020-06-30 (zuletzt geändert Art. 1 V v. 30.6.2020 I 1533) · Zugriff 2026-08-11
- Status: in Kraft
- Sub-Ebene: entfällt
- Relationen: liefert Klassifikationsgrundlage für REG-DE-3-004, REG-DE-3-006, REG-DE-3-008, REG-DE-3-009
- Konfidenz: gesichert (Kapitelstruktur); abgeleitet (Aussage zur fehlenden Bauteil-Zuordenbarkeit)

---

### REG-DE-3-008 · NachwV — Entsorgungs- und Sammelentsorgungsnachweis
- Titel: Verordnung über die Nachweisführung bei der Entsorgung von Abfällen (Nachweisverordnung, NachwV)
- Fundstelle: § 3 (Entsorgungsnachweis), § 9 (Sammelentsorgungsnachweis), § 24 Abs. 4–7 (Registerführungspflicht)
- A: national · B: 3 Abfall-/Stoffrecht · C: materialübergreifend · D: RVO
- E: Betrieb/Dokumentation
- F1 (E3): bedingend (nachweispflichtige — i.d.R. gefährliche — Abfälle bedürfen vor Entsorgungsbeginn eines Entsorgungsnachweises mit behördlicher Bestätigung; §9 erlaubt für gleichartige Abfälle unterhalb einer Mengengrenze ein vereinfachtes Sammelverfahren) · F2 (E3): bedingend (die 20-Tonnen-Schwelle je Abfallschlüssel und Kalenderjahr für den Sammelentsorgungsnachweis ist potenziell reuse-relevant für aggregierte, aus mehreren Kleinprojekten stammende Rückbauteil-Ströme gleichen Abfallschlüssels, sofern diese überhaupt Abfallstatus haben — s. REG-DE-3-001)
- G: Dokumentenlage (Formblätter, Register) — explizit (E1, §3, §24 Abs.4–7)
- Kernaussage: § 3 NachwV verlangt vor Beginn der Entsorgung nachweispflichtiger Abfälle einen Entsorgungsnachweis (Deklarationsanalyse des Erzeugers, Annahmeerklärung des Entsorgers, behördliche Bestätigung). § 9 erlaubt Einsammlern einen vereinfachten Sammelentsorgungsnachweis, wenn Abfälle denselben Abfallschlüssel und Entsorgungsweg haben und die Menge je Erzeuger/Standort 20 t je Abfallschlüssel und Kalenderjahr nicht übersteigt. § 24 Abs.4–7 verpflichtet zur laufenden Registerführung auch für nicht nachweispflichtige Abfälle, mit Eintragungsfrist von i.d.R. zehn Kalendertagen.
- Wortlautbeleg (Originalsprache): "Wer nachweispflichtige Abfälle zur Entsorgung in eine Abfallentsorgungsanlage bringen oder solche Abfälle dort annehmen will, hat vor Beginn der Abfallentsorgung die Zulässigkeit der vorgesehenen Entsorgung durch einen Entsorgungsnachweis" nachzuweisen (§3, Kernsatz per Zitatmarkierung); Sammelentsorgungsnachweis-Voraussetzungen: Abfälle müssen „denselben Abfallschlüssel haben" und „den gleichen Entsorgungsweg haben", Mengengrenze „20 Tonnen je Abfallschlüssel und Kalenderjahr" (§9, Kernsätze per Zitatmarkierung)
- Beleg-Quelle: B1 (amtliche Seite in dieser Session direkt gelesen, Kernsätze mit Zitatmarkierung wiedergegeben, nicht vollständiger Wortlaut aller Absätze) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (RVO selbst)
- Quelle: Tier 1 · https://www.gesetze-im-internet.de/nachwv_2007/BJNR229810006.html · Fassung(as-amended) 2022-04-28 (zuletzt geändert Art. 5 V v. 28.4.2022 BGBl. I S. 700) · Zugriff 2026-08-11
- Status: in Kraft
- Sub-Ebene: entfällt
- Relationen: konkretisiert Nachweisführung im Verbund mit REG-DE-3-004/006/007
- Konfidenz: gesichert (Kernaussagen); abgeleitet (vollständiger Absatzwortlaut nicht Zeichen für Zeichen geprüft)

---

### REG-DE-3-009 · DepV — Abgrenzung zur EBV, Zuordnungskriterien Anhang 3
- Titel: Verordnung über Deponien und Langzeitlager (Deponieverordnung, DepV)
- Fundstelle: § 1 (Anwendungsbereich), Anhang 3 (Zulässigkeits- und Zuordnungskriterien)
- A: national · B: 3 Abfall-/Stoffrecht · C: Mauerwerk/mineralisch · D: RVO
- E: Aufbereitung/Prüfung, Betrieb/Dokumentation
- F1 (E3): hemmend/neutral, Abgrenzungsfunktion (regelt Errichtung, Betrieb, Stilllegung und Nachsorge von Deponien sowie Behandlung von Abfällen zur Ablagerung — der Weg für Bauschutt-Fraktionen, die NICHT als Ersatzbaustoff nach EBV verwertet werden können; markiert damit denjenigen Teil des Bau-/Abbruchabfallstroms, für den eine stoffliche Wiederverwendung/Verwertung ausscheidet) · F2 (E3): hemmend (je strenger die Zuordnungskriterien in Anhang 3 für Deponierung sind, desto größer der wirtschaftliche Anreiz, Material stattdessen — auch bei zweifelhafter technischer Eignung — der Verwertung/EBV zuzuführen; Wechselwirkung mit REG-DE-3-004 nicht im Detail untersucht)
- G: Probenahme/Materialprüfung (Zuordnung zu Deponieklassen anhand Anhang-3-Kriterien) — explizit (E1, laut Anhangstitel)
- Kernaussage: Die DepV regelt Errichtung, Betrieb, Stilllegung und Nachsorge von Deponien der Klassen 0 bis IV sowie die Anforderungen an die zur Ablagerung vorgesehenen Abfälle. Anhang 3 („Zulässigkeits- und Zuordnungskriterien") enthält die Klassifizierungsregeln für Abfälle einschließlich Bauschutt-Fraktionen, die nicht als mineralischer Ersatzbaustoff nach EBV (REG-DE-3-004) verwertet werden — sie bildet damit die Abgrenzungsnorm zur EBV für den nicht wiederverwendungsfähigen Reststrom.
- Wortlautbeleg (Originalsprache): kein wörtliches Zitat von §1/Anhang 3 in dieser Erhebung gesichert (per WebFetch-Zusammenfassung paraphrasiert); Anhangstitel gemäß Inhaltsübersicht der amtlichen Seite: „Anhang 3: Zulässigkeits- und Zuordnungskriterien"
- Beleg-Quelle: B1 (amtliche Seite direkt gelesen, Stand-Zeile wörtlich, §1/Anhänge nur paraphrasiert) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (RVO selbst)
- Quelle: Tier 1 · https://www.gesetze-im-internet.de/depv_2009/BJNR090010009.html · Fassung(as-amended) 2024-07-03 (zuletzt geändert Art. 3 G v. 3.7.2024 BGBl. I Nr. 225 — Hinweis: ein WebSearch-Treffer nannte abweichend „Art. 18 G v. 22.7.2026 I Nr. 224"; dieser Wert wurde NICHT am Primärtext bestätigt und daher verworfen, konsistent mit der Praxis dieses Projekts, WebSearch-Zusammenfassungen nicht ungeprüft als Faktum zu übernehmen) · Zugriff 2026-08-11
- Status: in Kraft
- Sub-Ebene: entfällt
- Relationen: grenzt ab zu REG-DE-3-004 (EBV); nutzt Klassifikationsgrundlage von REG-DE-3-007 (AVV)
- Konfidenz: gesichert (Stand-Zeile, Anhangsstruktur); abgeleitet (Anhang-3-Inhalt nur paraphrasiert, nicht wörtlich verifiziert — Nacherhebung empfohlen)

---

## Gesamt-Lückenliste (Felder 1–3, ehrlich)

- **REG-DE-1-009 (MVV TB):** 354-seitiger Volltext nicht durchsucht; Aussage „schweigt zu Reuse" beruht auf Indizien (Übersichtsseiten), nicht auf vollständiger Lektüre. Einführungsstatus von Ausgabe 2026/1 zum Stichtag ungeklärt.
- **REG-DE-1-013 / analoge Sub-Ebene-Fälle:** Landesbauordnungen jenseits der DIBt-genannten Stichprobe (Berlin, Niedersachsen, M-V, Baden-Württemberg) wurden nicht einzeln primärquellenbasiert verifiziert; bei A=sub-national bleibt dies für 12–15 der 16 Länder eine offene Lücke.
- **REG-DE-2-005 (Eurocode-NA allgemein):** Zentrale, in der Taxonomie ausdrücklich genannte Kategorie („Eurocode-NA") konnte trotz mehrfacher Versuche (din.de, eurocode-online.de) NICHT mit einer tabellarischen NA-Übersicht primärquellenbasiert belegt werden — größte einzelne Lücke dieser Datei.
- **REG-DE-2-004 (VV-TB-Bindungskette):** Konkrete Listung von DIN SPEC 91484/91525, ISO 13822, EN 1990-2 oder VDI 6200 in einer der 16 Landes-VV-TB wurde NICHT verifiziert (technische Zugriffsprobleme bei den PDF-Dokumenten in dieser und der vorgelagerten Session).
- **REG-DE-3-003 (KrWG §4 Nebenprodukte), REG-DE-3-005 (EBV §§19–23), REG-DE-3-008 (NachwV), REG-DE-3-009 (DepV):** Wortlautbelege stammen aus WebFetch-Zusammenfassungen mit Zitatmarkierung, nicht aus eigener Zeichen-für-Zeichen-Lektüre des Roh-HTML/PDF — vorsichtshalber B1 statt B0 vergeben; für eine belastbare Wortlautzitierfähigkeit im Bericht wird eine erneute Verifikation (z.B. per lokalem `pdftotext` wie in den W0-Piloten) empfohlen.
- **LAGA M 20 (Bauabfallentsorgung):** in dieser Erhebung nicht angefragt (Zeitbudget) — laut DE-Quellenkarte weiterhin vollständig offen.
- **Feld-übergreifende Fragen (Feld 4/5b/6/7):** nicht Gegenstand dieser Datei (Auftragsscope ausdrücklich auf Felder 1–3 begrenzt); TRGS 519, GEG, VOB/A, VOB/B, BEG-Förderrichtlinien etc. bleiben in der DE-Quellenkarte als offene Punkte für die entsprechenden Folgeaufträge dokumentiert.
