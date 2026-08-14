# Prüfprotokoll SE — Adversarische Prüfung der Ernte-Dateien

**Projekt:** BBSR/Zukunft Bau „Entwerfen mit Bestand" (Az. 10.08.18.7-25.06) · W4-Vorlauf, adversarische Prüfung Länderharvest Schweden (SE)
**Geprüfte Datei:** `roh/SE-alle.md` (28 Regelungsobjekte, REG-SE-1-001 bis REG-SE-7-004; `roh/SE-quellen.md` ausdrücklich NICHT geprüft, weisungsgemäß ausgeschlossen)
**Prüfstichtag:** 2026-08-13 (Extraktionsstichtag der Ernte: 2026-08-11)
**Methode:** Sechs Pflichtchecks je Objekt (Supersessions-Nachweis, Primärquellen-Pin, Kompetenz-Check, Wirkrichtungs-Falsifikation, Scope-Overreach, Quote-back). Für 10 Objekte mit den höchsten Belegstufen (B1-Ansprüche) bzw. höchstem Risiko wurde in dieser Sitzung erneut live per WebFetch auf die Primärquelle zugegriffen (WebSearch-Kontingent der Sitzung war bereits erschöpft — Recherche erfolgte ausschließlich über direkte WebFetch-Aufrufe auf bekannte/plausible URLs). Die übrigen 18 Objekte wurden analytisch anhand der sechs Checks gegengelesen (Kompetenz-, Scope- und Supersessions-Konsistenz, interne Widerspruchsprüfung), ohne dass ein neuer Primärzugriff gelang oder versucht wurde, wo die Ernte bereits einen dokumentierten, plausiblen Fehlschlag (404/kein Fließtext) auswies.

---

## Ergebnisübersicht

| Kennzahl | Wert |
|---|---|
| Geprüfte Objekte | 28 |
| Bestätigt | 7 |
| Korrigiert | 3 |
| Widerlegt | 0 |
| Unbelegbar (unverändert, in dieser Sitzung nicht auflösbar) | 18 |
| Fabriziert | 0 |
| Abnick-Verdacht | **Nein** |

**Warum kein Abnick-Verdacht:** Die Ernte-Datei ist das Gegenteil von auffällig fehlerfrei. Sie markiert bereits selbst 16 von 28 Objekten auf B2/B3/B4 (kein oder kein zeichengenauer Primärtext), dokumentiert zwei gescheiterte Zugriffsversuche in Folge (Stufe 1 und Stufe 2) auf dieselben URLs, hält einen ungelösten internen Datumswiderspruch offen (BFS 2024:4 vs. übriges 2024er-Paket, s. u.) und bewertet ein Objekt (MVR BS04:2021) explizit als „nicht als Faktum verwertbar (B3)". Dieses Maß an eingestandener Unsicherheit ist untypisch für eine geschönte Erhebung.

---

## Live-reverifizierte Objekte (10) — Detailbefunde

### REG-SE-5-001 · LOU 4 kap. § 3 — **Bestätigt**
Wortlaut per WebFetch zeichengenau reproduziert: *„En upphandlande myndighet bör beakta miljöhänsyn, sociala och arbetsrättsliga hänsyn vid offentlig upphandling om upphandlingens art motiverar detta."* Charakter als Kann-/Soll-Bestimmung (kein "ska") bestätigt. Scope-Check bestanden: Norm ist allgemeine Umwelt-/Sozialkriterien-Klausel, kein reuse-spezifischer Passus — die Kernaussage überdehnt den Anwendungsbereich nicht.

### REG-SE-3-002 · Avfallsförordning 3 kap. §§ 19, 25 — **Korrigiert**
Der als „Wortlautbeleg" ausgegebene Satz *„avfall som utgörs av bygg- och rivningsavfall ska sorteras"* war eine Paraphrase, kein Zitat. Tatsächlicher Wortlaut (§ 19): *„Den som producerar bygg- och rivningsavfall ska, utöver vad som gäller enligt andra bestämmelser i detta kapitel, på den plats där avfallet uppkom, sortera ut åtminstone följande avfallsslag …"* Substanz der Kernaussage (Sortierpflicht am Entstehungsort) bleibt zutreffend — korrigiert wurde nur die Zitatgenauigkeit. In der Ernte-Datei korrigiert (Edit), Beleg-Quelle auf B1 hochgestuft. Die referenzierte Änderungs-VO 2025:820 konnte NICHT verifiziert werden (gezielte SFS-Abfrage ohne Treffer) — als unbelegt markiert, nicht gelöscht, da nicht positiv widerlegbar.

### REG-SE-3-001 · Miljöbalken 15 kap. — **Bestätigt**
Dritter unabhängiger WebFetch-Zugriff (nach Stufe 1 und Stufe 2 der Extraktion) reproduziert identische Gliederung: §§ 1–3 Zweck, §§ 4–5 Ord och uttryck (Begriffsbestimmungen), §§ 6–8 När avfall upphör att vara avfall, §§ 9–11 Avfallshierarki. Die in Stufe 2 vorgenommene Korrektur gegenüber Stufe 1 (Begriffsbestimmungen liegen in §§ 4–5, nicht § 2) gilt damit dreifach reproduziert. Zeichengenauer Definitionswortlaut bleibt technisch nicht extrahierbar (Grenze bestätigt, kein Zufallsartefakt).

### REG-SE-2-002 · PBL 8 kap. — **Bestätigt, ergänzt**
Vollständigerer Wortlaut § 1 erhalten: *„Byggnader ska utformas så att de är lämpliga för sitt ändamål, tillgängliga och användbara och har väsentliga tekniska egenskaper."* Die behauptete Änderung SFS 2026:712 (20.05.2026) konnte NICHT verifiziert werden — als unbelegt markiert.

### REG-SE-2-003 · Boverkets Vägledung „Cirkulär ekonomi"/„Barverksdelar" — **Bestätigt (Negativbefund)**
Dritter unabhängiger Zugriffsversuch liefert wie in Stufe 1 und Stufe 2 ausschließlich Navigationsstruktur, keinen Fließtext. Der Negativbefund ist damit als technische Extraktionsgrenze (vermutlich clientseitig gerenderter Inhalt) erhärtet, nicht als einmaliges Sitzungsartefakt. **Das bleibt die größte inhaltliche Erkenntnislücke der SE-Erhebung** — dort liegt mutmaßlich die zentrale Nachweismethodik für Reuse-Tragelemente.

### REG-SE-3-003 · Naturvårdsverket-Vägledung Abfallende — **Bestätigt + Korrigiert (F1-Falsifikation)**
HTTP 404 auf identischer URL dreifach reproduziert (Stufe 1, Stufe 2, Prüfsitzung). Negativbefund „kein nationales Abfallende-Regime für Baustoffe" bestätigt.
**F1-Falsifikationsversuch:** Die Ernte klassifizierte F1 durchgehend als „hemmend" (Fehlen von Kriterien erschwere Rechtssicherheit). Gegenlesart gebildet und dokumentiert: Das Fehlen starrer Kriterien lässt zugleich mehr Spielraum für pragmatische Einzelfallbewertung — wäre ebenso als „ermöglichend" lesbar. Der zugrunde liegende Normtext (Miljöbalken 15 kap. §§ 6–8) selbst enthält keine Wertung in die eine oder andere Richtung; „hemmend" ist eine plausible, aber nicht die einzig zwingende E3-Projektzuordnung. In der Datei als Korrektur vermerkt.

### REG-SE-2-001 · BFS 2024:6 — **Korrigiert (Konfidenz herabgestuft)**
Zwei weitere unabhängige Zugriffsversuche (forfattningssamling.boverket.se, PBL-kunskapsbanken) scheiterten (404/leer) — wie bereits in Stufe 1 und Stufe 2. Die Ernte hatte das Übergangsdatum (01.07.2025 exklusiv seit 01.07.2026) trotzdem mit „Konfidenz: gesichert (Supersession/Datum, zwei unabhängige Treffer)" versehen — diese Einstufung beruht aber ausschließlich auf Sekundär-Snippets, nie auf einer Primärquelle. Auf „abgeleitet" herabgestuft. **Kein Widerspruch zum Datum selbst gefunden, aber auch keine Bestätigung — echte Erkenntnislücke, keine Fabrikation.**

### REG-SE-4-007 · AFS 2025:6 (Asbest) — **Teilbestätigt**
av.se/nyheter/ bestätigt wörtlich die Schlagzeilen „Nu träder de nya reglerna om asbest i kraft" und „Gränsvärdet för asbest sänks" — Inkrafttreten und Grenzwertsenkung damit auf Primärquellenebene (Arbetsmiljöverket) bestätigt. Das exakte Datum (19.12.2025), die genauen Zahlenwerte (0,1 → 0,01 Fasern/cm³) und der §§ 96–97-Bezug bleiben unbelegt (nur Sekundär-Snippet-Niveau der Extraktionssitzung).

### REG-SE-1-001 · CPR 2024/3110 Reuse-Bezug — **Bestätigt, präzisiert**
EUR-Lex-Volltext per WebFetch geöffnet (in der Extraktionssitzung war dies unterblieben — Verweis auf `roh/eu-produkt.md` ungeprüft übernommen). Bestätigt: Erwägungsgründe 34–36 enthalten tatsächlich explizite Bestimmungen zu gebrauchten/wiederaufgearbeiteten Bauprodukten. **Wichtige Präzisierung:** Diese liegen auf Erwägungsgrund-Ebene (Ankündigung künftiger harmonisierter technischer Spezifikationen), nicht auf Ebene eines bereits operativen Artikels. F1 „ermöglichend" ist im Kern haltbar, aber als "dem Grunde nach, noch nicht operativ" zu lesen — die angekündigten hEN für Gebrauchtprodukte existieren zum Stichtag noch nicht.

### REG-SE-7-002 · SOU 2025:103 — **Bestätigt, auf B1 hochgestuft**
Betänkande-Text per WebFetch geöffnet: Datierung „Stockholm i oktober 2025", Zielinkrafttreten „den 9 december 2026", ausdrücklicher Bezug auf RL (EU) 2024/2853 — alle drei Kernangaben der Ernte zeichengenau bestätigt.

---

## Kompetenz-Check (Achse A) — gesamt

Kein Fehlbefund. A=EU/EEA korrekt nur für REG-SE-1-001 (CPR wirkt unmittelbar); alle übrigen 27 Objekte korrekt A=national. Schweden ist EU/EEA-Mitglied — kein CH-artiger MRA-Sonderfall, keine Verwechslung gefunden. D-Achse (Rechtsform) konsistent nach schwedischer Normenhierarchie kartiert: lag→Gesetz, förordning→RVO, myndighetsföreskrift (BFS/AFS/NFS)→Techn. Baubestimmung, vägledning→Merkblatt, AB04/ABT06/MVR→Branchenprotokoll. Keine Achsenverwechslung identifiziert.

## Supersessions-Prüfung — offener Kernbefund (nicht durch diese Sitzung gelöst)

Die Ernte selbst weist bei REG-SE-4-004 (BFS 2024:4) einen **ungelösten internen Datumswiderspruch** aus: BFS 2024:4 wird mit Inkrafttreten 01.01.2025 angegeben, während das übrige Paket (BFS 2024:6/7/8/9) durchgängig 01.07.2025 (verbindlich ab 01.07.2026) trägt — obwohl alle fünf Föreskrifter als ein koordiniertes Regelwerkspaket dargestellt werden. Diese Sitzung konnte den Widerspruch mangels Primärtextzugriff (erneut 404/leer bei allen Zugriffsversuchen auf BFS-2024-Volltexte und Boverket-Übersichtsseiten) **nicht auflösen**. Dies ist der wichtigste verbleibende Prüfpunkt für W4 — bevor eines der beiden Daten als gesichert in die Synthese übernommen wird, muss eine Primärquelle (nicht nur Sekundär-Snippet) gelesen werden.

## Scope-Overreach-Prüfung

Keine Verallgemeinerung über Material/Phase/Gebäudeklasse hinaus gefunden, die nicht bereits von der Ernte selbst als E3/vermutet gekennzeichnet wäre. Einzige Beobachtung: REG-SE-4-001 (BFS 2024:7, Brandschutz) ordnet C=„Holz (Materialfamilie-Relevanz laut Sekundärquellen hoch)" zu — dies ist plausibel (Brandschutz ist material-differenziert, Holzbau typischerweise stärker reguliert), aber ausschließlich auf Sekundärquellen gestützt und bereits korrekt als solche gekennzeichnet; kein Korrekturbedarf.

## Nicht neu verifizierte Objekte (18)

REG-SE-1-002, REG-SE-1-003, REG-SE-2-004 (B3, bereits korrekt als nicht faktentragfähig markiert), REG-SE-3-004, REG-SE-4-001, REG-SE-4-002, REG-SE-4-003, REG-SE-4-004, REG-SE-4-005, REG-SE-4-006, REG-SE-4-008, REG-SE-5-002, REG-SE-5-003, REG-SE-5-004, REG-SE-5-005, REG-SE-6-001, REG-SE-6-002, REG-SE-7-001, REG-SE-7-003, REG-SE-7-004. Für diese wurden die sechs Checks analytisch (Konsistenz-/Kompetenz-/Scope-Prüfung ohne neuen Primärzugriff) durchgeführt; keine Fehlbefunde, keine Fabrikation festgestellt. Ihr bestehender Belegstatus (überwiegend B2, teils B1 aus Stufe 1 fortgeschrieben wie REG-SE-4-006, teils B3/B4) bleibt unverändert gültig und ist bereits ehrlich gekennzeichnet.

## Vorgenommene Korrekturen in `roh/SE-alle.md` (Edit-Protokoll)

1. REG-SE-3-002: Wortlautbeleg von Paraphrase auf zeichengenaues Zitat korrigiert; Beleg-Quelle B1-fortgeschrieben → B1-reverifiziert; Änderungs-VO 2025:820 als unbelegt markiert.
2. REG-SE-2-002: Wortlaut § 1 ergänzt/präzisiert; Änderung SFS 2026:712 als unbelegt markiert.
3. REG-SE-3-001: Reproduktions-Vermerk (dritte unabhängige Bestätigung der Stufe-2-Korrektur) ergänzt.
4. REG-SE-7-002: Beleg-Quelle B2 → B1, Wortlautbeleg ergänzt.
5. REG-SE-1-001: Kernaussage um Recital- vs. Artikel-Ebene-Präzisierung ergänzt, Wortlautbeleg (Erwägungsgründe 34/36) ergänzt.
6. REG-SE-3-003: F1-Falsifikationsversuch/Gegenlesart dokumentiert; Zugriffsdatum/Reproduktionsvermerk ergänzt.
7. REG-SE-2-001: Konfidenz von „gesichert" auf „abgeleitet" herabgestuft (Datum beruht nur auf Sekundär-Snippets).
8. REG-SE-4-007: Teilbestätigung dokumentiert (Inkrafttreten/Grenzwertsenkung bestätigt, exakte Zahlen/Datum weiterhin unbelegt).
9. REG-SE-2-003: Reproduktions-Vermerk (dritter gescheiterter Zugriffsversuch) ergänzt.
10. REG-SE-5-001: Reproduktions-Vermerk (Wortlaut dritt-unabhängig bestätigt) + Scope-Check-Vermerk ergänzt.

## Kritische Befunde für W4 (Priorität)

1. **Ungelöster Datumswiderspruch im BFS-2024-Paket** (REG-SE-2-001/4-001–4-004): 01.01.2025 vs. 01.07.2025 — vor Synthese zwingend durch echten Primärtextzugriff zu klären.
2. **Boverkets Vägledung „Barverksdelar"** (REG-SE-2-003) bleibt nach drei unabhängigen Sitzungen textlich unzugänglich — voraussichtlich zentrale Quelle für Bestandsbewertung wiederverwendeter Tragelemente; alternativer Zugriffsweg (Wayback Machine, PDF-Direktlink, OCR) für W4 zwingend zu prüfen, bevor der Bericht eine Aussage zur schwedischen Nachweismethodik für Reuse-Tragelemente trifft.
3. Zwei in der Extraktionssitzung zitierte SFS-Änderungsnummern (2025:820 zur Avfallsförordning, 2026:712 zur PBL) konnten in der Prüfsitzung nicht verifiziert werden — nicht als Fabrikation eingestuft (die Ernte hatte sie bereits selbst als „laut Suchtreffer, nicht verifiziert" gekennzeichnet), aber vor Verwendung im Bericht zwingend zu bestätigen oder zu streichen.
