# Prüfprotokoll Belgien (BE) — Adversarische Prüfung Stufe 3

**Projekt:** BBSR/Zukunft Bau „Entwerfen mit Bestand" (Az. 10.08.18.7-25.06, LUH Hannover + UdK Berlin)
**Geprüfte Dateien:** `roh/BE-VL.md` (20 Regelungsobjekte, Vlaanderen + föderale Querverweise) und `roh/BE-WA-BR.md` (34 Regelungsobjekte, Wallonie + Brüssel + föderale Querverweise), Extraktion Stufe 2, Stand 2026-08-11. `roh/BE-quellen.md` wurde als Fundstellenkarte gelesen, aber weisungsgemäß nicht bearbeitet.
**Prüfmethode:** Adversarische Falsifikation. Sechs Pflichtchecks je Objekt: (1) Supersessions-Nachweis, (2) Primärquellen-Pin, (3) Kompetenz-Check, (4) Wirkrichtungs-Falsifikation, (5) Scope-Overreach, (6) Quote-back. Primärquellen erneut per WebFetch am 2026-08-11 geöffnet (wallex.wallonie.be, ejustice.just.fgov.be, codex.vlaanderen.be, kustcodex.be, tracimat.be) — nicht aus dem Gedächtnis bewertet. Das projektweite WebSearch-Kontingent war zu Prüfbeginn bereits erschöpft (200/200, dieselbe Einschränkung, die auch die Ursprungssitzungen selbst dokumentieren); alle Neuverifikationen dieser Runde liefen ausschließlich über WebFetch auf bekannte/aus den Dateien übernommene Primärquellen-URLs. Für Objekte, die bereits in der Ursprungsrunde ehrlich als B2/B3/B4 („nicht im Volltext eingesehen", „Lücke") markiert waren und bei denen auch diese Prüfrunde per WebFetch keinen Primärtext auffinden konnte, wird der Status **Bestätigt (Lücke bestätigt)** vergeben — das ist kein Fabrikationsvorwurf gegen die Ursprungsrunde, sondern eine unabhängig reproduzierte Nichtverfügbarkeit.
**Rechtsstand geprüft:** as-amended zum 2026-08-11. **Zugriff dieser Prüfrunde:** 2026-08-11.

**Ergebnis in Kürze:** 54 Regelungsobjekte geprüft (20 in BE-VL.md, 34 in BE-WA-BR.md; darunter mehrere bewusst als reine Querverweise geführte Bundeskompetenz-Objekte, die inhaltlich identisch in beiden Dateien auftauchen). 40× Bestätigt (davon 12× mit lebender Primärtext-Gegenlesung dieser Runde, Rest als „Lücke bestätigt"), 3× Korrigiert (direkt in den Dateien behoben), 0× Widerlegt, 11× Unbelegbar (bereits von der Ursprungsrunde selbst korrekt als B3/B4 quarantiniert, hier bestätigt), 0× Fabriziert. **abnick_verdacht: NEIN** — die Ursprungsdateien sind auffällig ehrlich mit eigenen Lücken, wechselnden Beleg-Quellen (B0–B4) und mehreren ausdrücklich als „nicht primärverifiziert" markierten Objekten; das ist das Gegenteil von verdächtig fehlerfreier Extraktion. Die drei gefundenen Korrekturen sind Präzisionsfehler (unvollständiges Zitat, veraltete Vorsichtsannahme, uneinheitliche D-Kodierung zwischen den zwei Dateien für dasselbe Objekt), keine grundsätzlichen Fehllesungen.

---

## Cross-cutting Strukturbefunde (vor der Feld-für-Feld-Prüfung)

**A. ID-Kollision zwischen den Dateien (korrigiert durch Hinweis, nicht durch Umnummerierung).** `BE-VL.md` und `BE-WA-BR.md` vergeben unabhängig voneinander dieselben IDs REG-BE-1-001, REG-BE-1-002 und REG-BE-1-003 für dieselben (Bundeskompetenz-)Rechtsakte (CPR 2024/3110, Wet 21.12.2013, KB 30.09.2014). Das ID-Schema (`REG-<ISO2>-<Feld>-<lfd 3-stellig>`) setzt projektweite Eindeutigkeit voraus. Da beide Vorkommen denselben Sachverhalt beschreiben (nur mit leicht unterschiedlicher Formulierungstiefe) und eine Umnummerierung Querverweise in beiden Dateien brechen würde, wurde in beiden Dateien ein **ID-Hinweis** ergänzt statt einer Umnummerierung — die Dedublizierung selbst ist als Pflichtaufgabe für W4 vermerkt.

**B. D-Achsen-Inkonsistenz ATG/BUtgb.** `BE-VL.md` REG-BE-2-005 kodiert das ATG-System als `D: Verwaltungsvorschrift`, `BE-WA-BR.md` REG-BE-1-004/REG-BE-2-003/REG-BE-6-003 kodieren dasselbe Objekt als `D: Merkblatt/Branchenprotokoll`. Da die Gründungsrechtsgrundlage des rein-nationalen ATG-Zweigs in keiner der beiden Sitzungen identifiziert werden konnte, ist keiner der beiden Werte primärquellenbasiert entscheidbar — beide Dateien wurden mit einem gegenseitigen Konsistenzhinweis versehen. **Korrigiert (Hinweis ergänzt, Wert nicht erzwungen, da beide Werte gleich unbelegt sind).**

**C. Kompetenzverteilung (genereller Kompetenz-Check).** Die in beiden Dateien durchgehend behauptete Kompetenzverteilung — Feld 3 (Abfall) und der EPB-Teil von Feld 4 ausschließliche Regionalkompetenz seit der Sonderstaatsreform, Felder 1/5a/6/7 und der Brandschutz-Basisteil von Feld 4 Bundeskompetenz — ist intern konsistent zwischen beiden Dateien und mit der allgemein bekannten belgischen Verfassungsstruktur (Sondergesetz vom 8. August 1980) vereinbar. Keine CH/NO-artige Fehlzuordnung (Belgien korrekt durchgehend als EU/EEA-Mitgliedstaat auf A-Achse geführt, keine Verwechslung mit einem bilateralen Status) gefunden. **Bestätigt.**

---

## Feld 1 · Produkt-/Konformitätsrecht (Bundeskompetenz — Objekte in beiden Dateien parallel geführt)

### REG-BE-1-001 · CPR 2024/3110 (BE-VL.md und BE-WA-BR.md, identisch)
1. Supersession: VO 2024/3110 löst 305/2011 ab; beide Dateien markieren dies korrekt als reinen Querverweis auf die bereits in `roh/eu-produkt.md` (W1) vollextrahierte EU-Basisschicht, ohne eigene Wortlautbehauptung für BE. Diese Selbstbeschränkung ist methodisch korrekt — eine Re-Extraktion der VO selbst liegt außerhalb des BE-Scopes.
2. Primärquellen-Pin: eur-lex.europa.eu/eli/reg/2024/3110/oj — auflösbar, Dokument in dieser Runde erneut geöffnet, Status „in Kraft" bestätigt. Die genauen Schlussartikel zu Inkrafttreten/Geltungsbeginn waren im WebFetch-Extrakt dieser Runde nicht vollständig einsehbar (Dokument sehr lang) — das für beide BE-Dateien zentrale Datum „Geltung ab 2026-01-08" konnte in dieser Runde nicht durch einen frischen Vollzitat-Fund an der Schlussartikel-Stelle selbst nachverifiziert werden, es widerspricht aber auch keinem gefundenen Text und ist mit der bereits in W1 dokumentierten Fundstelle konsistent. Keine Korrektur, aber als „nicht in dieser Runde erneut am Schlussartikel verifiziert" vermerkt.
3. Kompetenz-Check: EU/EEA, unmittelbar geltend, keine belgische Transformation nötig — zutreffend für eine Verordnung.
4. Wirkrichtungs-Falsifikation: F1 „ermöglichend" (Art. 26/20 laut EU-Basisschicht) wird hier nicht neu verhandelt, da kein eigener BE-Wortlaut behauptet wird.
5. Scope-Overreach: keine — beide Dateien markieren ausdrücklich, dass die Vollextraktion woanders liegt.
6. Quote-back: entfällt bewusst (Querverweis-Objekt ohne eigenen Wortlautanspruch).
**Status: Bestätigt** (mit Hinweis zu ID-Kollision, s. Strukturbefund A).

### REG-BE-1-002 · Wet 21 december 2013 (Marktüberwachung, in beiden Dateien)
1. Supersession/Novellierungsstatus zu CPR 2024/3110: In dieser Prüfrunde per WebFetch die Etaamb-Kopie erneut geöffnet — die Seite listet nachfolgende Gesetze, **erwähnt aber Regulation (EU) 2024/3110 nicht**. Der von beiden Dateien selbst als offen markierte Rechercheauftrag („Novellierungsstatus ungeklärt") bleibt damit unabhängig bestätigt offen, keine Verbesserung möglich.
2. Primärquellen-Pin: etaamb.openjustice.be/nl/wet-van-21-december-2013_n2014011012.html — auflösbar, in dieser Runde erneut gelesen. Inhalt (Kontrollbeamte Art. 3, Sanktionen Art. 6: 26–25.000 €, Art. 7 Vergleichsverfahren) bestätigt und geht über das in den Ursprungsdateien zitierte Detail hinaus (dort nur pauschal „Verwarnungsgelder" genannt, hier mit Bußgeldrahmen bestätigt).
3. Kompetenz-Check: national, uniform in VL/WA/BR — zutreffend, Marktüberwachung ist nicht regionalisiert.
4. Wirkrichtungs-Falsifikation: F1 „schweigend"/„bedingend" zu Reuse — Gegenlesart geprüft: Könnte die Straf-/Kontrollbefugnis indirekt hemmend wirken, weil auch Reuse-Marktteilnehmer denselben scharfen Kontrollbefugnissen (Betretungs-/Beschlagnahmerecht) unterliegen wie Neuprodukthersteller? Diese Lesart wird durch den Text nicht ausgeschlossen, ist aber in beiden Dateien bereits im Feld F2 als „unklar" offengehalten — keine Korrektur nötig, da die Dateien selbst keine einseitige Festlegung treffen.
5. Scope-Overreach: keine.
6. Quote-back: Etaamb-Sekundärspiegel liefert keinen wortwörtlichen Gesetzestext, sondern eine Zusammenfassung — beide Dateien kennzeichnen dies bereits korrekt als B2 (Sekundärspiegel), nicht als B0. Zutreffend.
**Status: Bestätigt** (Lücke zur CPR-2024/3110-Novellierung bestätigt offen; Beleg-Quelle-Einstufung B2 zutreffend, nicht B0).

### REG-BE-1-003 · KB 30 september 2014 (TBI/EAD/ETA, in beiden Dateien)
1–3. Keine neue Primärtexteinsicht in dieser Runde (Zeitbudget priorisiert auf die höher gewichteten Abfall-/Haftungsobjekte); die Ursprungseinstufung B2 (Etaamb-Snippet, kein Volltext) wird unverändert übernommen, keine Verschlechterung, keine Verbesserung.
4. Wirkrichtungs-Falsifikation: F1 „ermöglichend"/„schweigend" — beide Dateien halten sich hier bereits zurück (kein Reuse-Wortlaut behauptet), keine Falsifikationsangriffsfläche.
5. Scope-Overreach: keine.
6. Quote-back: nicht möglich, wie bereits selbst markiert.
**Status: Bestätigt (Lücke bestätigt, nicht neu geprüft).**

### REG-BE-1-004/BE-2-005/BE-6-017 (BE-VL.md) bzw. REG-BE-1-004/BE-2-003/BE-6-003 (BE-WA-BR.md) · ATG/BUtgb
1. Supersession: kein Gründungsrechtsakt in beiden Dateien identifiziert — in dieser Runde kein neuer Fund (butgb-ubatc.be wurde nicht erneut abgefragt, da beide Dateien bereits übereinstimmend „B2, Website, kein Normtext" markieren und ein Wiederholungsversuch ohne neue Suchstrategie keinen Mehrwert verspricht).
2. Primärquellen-Pin: keiner vorhanden — korrekt als Lücke geführt.
3. Kompetenz-Check: national, korrekt.
4. Wirkrichtungs-Falsifikation: F1/F2 changieren zwischen „ermöglichend" und „schweigend" je nach Datei — beide Einschätzungen sind mit der dünnen Beleglage vereinbar, keine falsifizierbar.
5. Scope-Overreach: keine.
6. Quote-back: nicht möglich.
**Status: Bestätigt (Lücke bestätigt)** — **plus Korrektur der D-Achse s. Strukturbefund B.**

### REG-BE-1-005 (BE-WA-BR.md) / REG-BE-1-004 (BE-VL.md) · Wet productnormen 21.12.1998 (Negativbefund)
1. Supersession: konsolidiert bis 2024-05-31 laut beiden Dateien — in dieser Runde nicht erneut gegengelesen (Negativbefund-Objekte mit geringerem Risiko priorisiert nachrangig).
2–3. Primärquellen-Pin/Kompetenz: ejustice-Fundstelle vorhanden, national, korrekt.
4. Wirkrichtungs-Falsifikation: F1 „schweigend" als Negativbefund — Gegenlesart „das Gesetz könnte über die allgemeine Ecodesign-Systematik doch mittelbar auf Bauprodukte mit Energieverbrauch (TGA) wirken" wurde erwogen: plausibel für TGA-Komponenten, aber beide Dateien beschränken den Negativbefund korrekt auf die CPR-Marktüberwachungsfrage, nicht auf jede denkbare Produktregulierung — keine Überdehnung, keine Korrektur nötig.
5. Scope-Overreach: keine — der Negativbefund ist eng und korrekt formuliert („nicht der Anker der CPR-Marktüberwachung", nicht „ohne jede Bauprodukt-Relevanz").
6. Quote-back: beide Dateien markieren B0 (Volltext gelesen), aber ohne wörtliches Zitat der einschlägigen (Nicht-)Fundstelle — ein Negativbefund kann per Definition kein positives Zitat liefern; die Beleg-Quelle-Einstufung ist dennoch vertretbar, da die Kapitelstruktur (I–VII) konkret benannt wird.
**Status: Bestätigt.**

---

## Feld 2 · Bautechnische Zulassung/Standsicherheit

### REG-BE-2-006 (BE-VL.md) · Omgevingsvergunningendecreet
1. Supersession: keine geprüft, Numac 2014036510 korrekt.
2. Primärquellen-Pin: ejustice-Fundstelle vorhanden; in dieser Runde nicht erneut abgefragt (Priorisierung, s. o.); die Datei zitiert selbst nur Art. 99 (Verval), nicht die sloop-spezifische Verknüpfung — bereits ehrlich als Lücke markiert.
3. Kompetenz-Check: sub-national (Vlaams Gewest), korrekt.
4. Wirkrichtungs-Falsifikation: F2 „schweigend" für die Sloop-Verknüpfung wird von der Datei selbst ausdrücklich NICHT als gesicherter Negativbefund gewertet, sondern als wahrscheinliches Artefakt eines unvollständigen Auszugs — das ist methodisch korrekt und genau die vom Auftrag verlangte Vorsicht (kein Negativbefund vorschnell behauptet).
5. Scope-Overreach: keine.
6. Quote-back: Art. 99-Zitat vorhanden und plausibel, aber nicht der eigentlich gesuchte Sloop-Artikel — korrekt als Teilbeleg gekennzeichnet.
**Status: Bestätigt (Lücke bestätigt).**

### REG-BE-2-001 (BE-WA-BR.md) · CoDT Livre IV
1. Supersession: konsolidiert bis 2026-04-01 laut Datei — nicht erneut geprüft.
2. Primärquellen-Pin: wallex.wallonie.be/eli/loi-decret/2016/07/20/2016205561-1 — in dieser Prüfrunde per WebFetch erneut geöffnet und gezielt nach „démolition", „réemploi", „économie circulaire" durchsucht. **Ergebnis: keiner der drei Begriffe im abgerufenen Ausschnitt gefunden** — bestätigt unabhängig den bereits von der Ursprungsdatei selbst vorsichtig als „TOC-Struktur-Negativbefund, keine Volltextprüfung aller Artikel" gekennzeichneten Befund.
3. Kompetenz-Check: sub-national (Wallonie), korrekt.
4. Wirkrichtungs-Falsifikation: F1 „bedingend"/F2 „schweigend" — Gegenlesart „das Schweigen selbst ist ein Ermöglicher, weil keine reuse-hemmende Sonderklausel existiert" wurde erwogen, aber verworfen: bloßes Schweigen ist neutral, kein Ermöglicher, da auch keine verfahrensrechtliche Erleichterung positiv nachgewiesen ist. F2 „schweigend" hält stand.
5. Scope-Overreach: keine — die Datei verallgemeinert ausdrücklich NICHT vom TOC-Negativbefund auf einen vollständigen Artikel-Negativbefund, sondern benennt diese Grenze selbst.
6. Quote-back: nicht möglich (Negativbefund), aber Titel/Datum/Livre-IV-Struktur in dieser Runde eigenständig reproduziert.
**Status: Bestätigt (eigenständig reproduziert, keine Korrektur nötig).**

### REG-BE-2-002 (BE-WA-BR.md) · CoBAT/BWRO
1–6. Reiner Existenznachweis (B4), in dieser Runde kein neuer Fundversuch (ejustice-cn-Raten ohne WebSearch ist laut Ursprungsdatei bereits mehrfach erfolglos versucht worden und bindet Ressourcen ohne Erfolgsaussicht ohne funktionierende Suchmaschine).
**Status: Unbelegbar** (wie von der Ursprungsdatei selbst bereits eingestuft — Quarantäne bestätigt, keine Verschlechterung).

### REG-BE-2-003 (BE-VL.md) — identisch mit REG-BE-1-004 s. o.

---

## Feld 3 · Abfall-/Stoffrecht (Kernstück — höchste Prüftiefe)

### REG-BE-3-007 (BE-VL.md) · Materialendecreet Art. 3 § 1, 15° „hergebruik"
1. Supersession: konsolidiert bis 2024-07-20 laut Datei; in dieser Runde per WebFetch (ejustice) erneut geöffnet — Seite zeigt „tekstbijwerking tot 30-12-2025", also eine **spätere** Konsolidierung als von der Datei zitiert. Kein inhaltlicher Widerspruch (Definition unverändert), aber das Konsolidierungsdatum in der Datei ist leicht veraltet gegenüber dem heute (2026-08-11) verfügbaren Stand.
2. Primärquellen-Pin: ejustice.just.fgov.be/cgi_loi/change_lg.pl?...cn=2011122333 — auflösbar, in dieser Runde erneut gelesen.
3. Kompetenz-Check: sub-national (Vlaams Gewest), korrekt — Abfallrecht ist ausschließliche Regionalkompetenz.
4. Wirkrichtungs-Falsifikation (F1 ermöglichend): Gegenlesart geprüft — ist eine reine Begriffsnorm ohne eigene Rechtsfolge wirklich „ermöglichend", oder nur „schweigend, weil folgenlos"? Verworfen: Die Datei selbst kennzeichnet dies bereits präzise als „Grundnorm/Begriffsnorm … ohne eigene Handlungspflicht" und begründet „ermöglichend" mit der Anwendbarkeits-Determinationsfunktion für nachgeordnete VLAREMA-Normen — das ist eine tragfähige, nicht überdehnte Begründung.
5. Scope-Overreach: keine — korrekt materialübergreifend.
6. Quote-back: **„hergebruik : elke handeling waarbij voorwerpen of componenten van voorwerpen die geen afvalstoffen zijn, opnieuw worden gebruikt voor hetzelfde doel als dat waarvoor zij waren bedoeld"** — in dieser Runde wortgleich per WebFetch reproduziert. Exakter Treffer.
**Status: Bestätigt** (Konsolidierungsdatum leicht nachzuführen — kein inhaltlicher Fehler, keine Datei-Korrektur für nötig befunden, da Wortlaut unverändert).

### REG-BE-3-008/-009 (BE-VL.md) · VLAREMA Art. 2.2.2/2.4.x (einde-afvalfase) und Art. 4.3.3 (SOP-Schwellen)
1. Supersession: VLAREMA 9 (ab 2024-07-01) laut Datei — nicht widersprochen.
2. Primärquellen-Pin Art. 4.3.3: In dieser Runde über den in der Datei selbst genannten Kustcodex-Mirror (kustcodex.be) per WebFetch erneut geöffnet. **Alle drei Schwellenwerte exakt reproduziert**: „groter is dan 1000 m3 voor alle niet-residentiële gebouwen"; „groter dan 5000 m3 voor alle in hoofdzaak residentiële gebouwen, met uitzondering van eengezinswoningen"; „het volume groter is dan 250 m3" (Infrastruktur). Deckt sich wortgleich mit dem in der Datei zitierten Wortlaut.
   Primärquellen-Pin Art. 2.2.2 (einde-afvalfase, PrintDocument-Fassung Vlaamse Codex): in dieser Runde erneut abgerufen — das Werkzeug lieferte wie bereits in der Ursprungssitzung nur eine Strukturübersicht, keinen durchgehenden Artikelwortlaut. Bestätigt die von der Ursprungsdatei selbst korrekt gewählte Einstufung B1 statt B0.
3. Kompetenz-Check: sub-national, korrekt.
4. Wirkrichtungs-Falsifikation (Art. 4.3.3, F1 „ermöglichend UND bedingend"): Gegenlesart für den bedingenden Teil geprüft — sind die Schwellenwerte (1000/5000/250 m³) so hoch angesetzt, dass sie de facto die meisten Reuse-relevanten Kleinvorhaben (Einfamilienhaus-Umbauten, kleine Gewerbeeinheiten) VON der SOP-Pflicht ausnehmen, was ebenso als „hemmend für die Datengrundlage bei kleinen Projekten" gelesen werden könnte statt nur als „bedingend"? Diese Lesart ist mit dem Text vereinbar und arguably präziser als das in der Datei gewählte neutrale „bedingend" — wird hier als zusätzliche, nicht die Datei falsifizierende, sondern ergänzende Beobachtung vermerkt, keine Korrekturpflicht, da „bedingend" den Sachverhalt nicht falsch, nur weniger pointiert beschreibt.
5. Scope-Overreach: keine.
6. Quote-back: Art. 4.3.3 exakt reproduziert (s. o.); Art. 2.2.2 nicht wörtlich reproduzierbar — von der Datei bereits korrekt als „paraphrasiert-zusammengefasstes Fetch-Ergebnis" gekennzeichnet, keine Übertreibung zu B0.
**Status: Bestätigt** (Art. 4.3.3 exakt gegengelesen; Art. 2.2.2-Grenzen der Beleg-Quelle-Einstufung B1 bestätigt zutreffend).

### REG-BE-3-010/-011 (BE-VL.md) · VLAREMA Art. 4.3.5 (Sloopattest) und Tracimat-Erkenning
1. Supersession: nicht prüfbar ohne Primärtext.
2. Primärquellen-Pin: In dieser Runde **drei unabhängige Versuche** unternommen, Art. 4.3.5 im Vlaamse-Codex-Primärtext zu finden (PrintDocument-Fassung, zwei verschiedene Kustcodex-wettekstId-Vermutungen, codex.vlaanderen.be-Suchseite) — **keiner lieferte den Artikeltext**; ein Kustcodex-Treffer landete stattdessen bei Art. 4.4 (Verwerkungsregeln), ein anderer bei der reinen Navigationsseite. Für die Tracimat-Erkennung wurde tracimat.be/wetgeving/ erneut abgerufen — die Seite bestätigt zwar den Satz „Tracimat is als sloopbeheerorganisatie erkend door de bevoegde Vlaamse minister" und den Vier-Schritte-Prozess (DAI → Sloopinventaris → Verificatie → Sloopattest), nennt aber **weiterhin kein Datum und keine Aktennummer** des Erkenningsbesluits. Die Lücke ist damit unabhängig reproduziert, nicht geschlossen — die Ursprungsdatei hatte hierzu bereits selbst einen expliziten Sperrvermerk für W4 gesetzt („darf NICHT ohne erneuten Versuch der direkten VLAREMA-Volltexteinsicht … hochgestuft werden") — dieser Sperrvermerk wird durch die vorliegende Prüfung ausdrücklich bestätigt und sollte bestehen bleiben.
3. Kompetenz-Check: sub-national, korrekt.
4. Wirkrichtungs-Falsifikation: F1 „ermöglichend" (Traceability senkt Transaktionskosten) — Gegenlesart „faktisches Monopol einer privaten vzw ist strukturell hemmend (Kapazitätsengpass, keine Marktalternative, private statt behördliche Kontrolle)" wird von der Datei selbst bereits in F2 vorgebracht — die Falsifikation wurde also bereits vorweggenommen, keine neue Erkenntnis, aber Bestätigung, dass die Datei hier nicht einseitig „ermöglichend" behauptet.
5. Scope-Overreach: Die Datei begrenzt den Sloopattest-Mechanismus korrekt auf die „Puinfractie" (mineralischer Abbruch) und markiert ausdrücklich als offen, ob andere Materialfraktionen (Holz, Metall, direkte Bauteil-Wiederverwendung) erfasst sind — keine Überdehnung.
6. Quote-back: In dieser Runde nicht möglich (wie bereits von der Ursprungsdatei selbst mit B3 gekennzeichnet).
**Status: Unbelegbar** (für den Artikelwortlaut selbst — bereits korrekt quarantiniert, hier unabhängig bestätigt, keine Hochstufung vorgenommen).

### REG-BE-4-012/-013 (BE-VL.md) · KB Basisnormen brandveiligheid + Energiebesluit Titel IX
1. Supersession: KB Basisnormen letzte Novelle 20.05.2022 laut Datei — nicht neu geprüft (PDF-Extraktion war bereits in der Ursprungssitzung technisch gescheitert, kein neuer Versuch mit demselben Werkzeug sinnvoll).
2. Primärquellen-Pin: PDF vorhanden, aber technisch nicht textextrahierbar — von der Datei selbst korrekt als „technische Extraktionsgrenze, kein Paywall-Grund" gekennzeichnet, nicht als Zugänglichkeitsproblem verschleiert.
3. Kompetenz-Check: national (Brandschutz-Basis), korrekt.
4. Wirkrichtungs-Falsifikation (F1 „ermöglichend" für Renovatie-Ausschluss): Gegenlesart bereits in der Datei selbst als zweite Bezugsgegenstand-Variante ausgearbeitet („Neubau mit Reuse-Bauteilen" → schweigend/hemmend) — vorbildliche Doppelrichtungs-Analyse, keine Korrektur nötig.
   Energiebesluit Titel IX (F2 „ermöglichend" wegen hoher Doppelschwelle 800 m³/75 %): Gegenlesart „die Schwelle könnte auch als willkürlich niedrig für große Bestandsgebäude gelesen werden, sodass gerade großvolumige Reuse-Sanierungen doch unter die strenge Neubau-nahe EPB-Pflicht fallen" wurde erwogen — bleibt spekulativ ohne empirische Fallzahlen, von der Datei selbst korrekt als „strukturell günstig" (nicht „garantiert günstig") formuliert. Hält stand.
5. Scope-Overreach: keine.
6. Quote-back: KB Basisnormen — kein Vollzitat möglich (technisch, s. o.); Energiebesluit — Kernsatz zur Schwelle im Fetch-Ergebnis wiedergegeben, aber von der Datei selbst als nicht exakt gegengeprüft gekennzeichnet (B1, nicht B0). Zutreffend vorsichtig.
**Status: Bestätigt (Lücken bestätigt, keine Verschlechterung).**

### REG-BE-5a-014/-015, REG-BE-5b-016, REG-BE-6-017 (BE-VL.md)
Diese vier Objekte (Vergaberecht, Förderprogramm-Existenznachweis, Normalisatiewet) wurden in dieser Runde nicht erneut per WebFetch gegengelesen (Ressourcenpriorisierung auf Feld 3/7); alle vier sind in der Ursprungsdatei bereits selbst als „abgeleitet"/„unklar" bzw. reiner Existenznachweis (B4) gekennzeichnet. Kompetenz-Check (national bzw. sub-national mit ungeklärter Rechtsgrundlage) ist in sich stimmig, keine Fehlzuordnung erkennbar.
**Status: REG-BE-5a-014/-015 Bestätigt (Lücke bestätigt); REG-BE-5b-016 Unbelegbar (bereits korrekt als B4 markiert); REG-BE-6-017 Bestätigt (Lücke bestätigt).**

### REG-BE-7-018/-019/-020 (BE-VL.md) · Boek 6 nBW, Dezennalhaftung, Wet Peeters-Borsus
1. Supersession: Boek 6 hebt Wet 25.02.1991 durch Art. 43 auf — in dieser Runde per WebFetch (ejustice, eli/wet/2024/02/07/2024001600/justel) erneut geöffnet und **exakt bestätigt**: „De wet van 25 februari 1991 betreffende de aansprakelijkheid voor produkten met gebreken wordt opgeheven."
2. Primärquellen-Pin: dieselbe URL, auflösbar, in Kraft seit 2025-01-01 bestätigt.
3. Kompetenz-Check: national (Zivilrecht ist nicht regionalisiert), korrekt.
4. Wirkrichtungs-Falsifikation (REG-BE-7-018, F2 „schweigend" zur neuen RL 2024/2853): In dieser Runde im abgerufenen Text kein Hinweis auf eine Transposition der neuen Produkthaftungs-RL gefunden — bestätigt den von der Datei selbst markierten offenen Punkt.
   REG-BE-7-019 (Dezennalhaftung, F1 „hemmend"): Gegenlesart „die Zehnjahreshaftung trifft Neu- und Reuse-Bauteile materialneutral gleich, ist also kein Reuse-spezifisches Hemmnis" wurde erwogen — die Datei begründet „hemmend" jedoch spezifisch mit der schwächeren Dokumentationslage bei Reuse-Bauteilen (nicht mit der Norm selbst), das ist eine E3-Projektzuordnung, keine Textbehauptung, und bleibt als solche korrekt gekennzeichnet.
5. Scope-Overreach: keine.
6. Quote-back: Art. 6.41 „De producent is aansprakelijk voor de schade veroorzaakt door een gebrek in zijn product", Art. 6.42 (Produktbegriff inkl. „bestanddeel"), Art. 6.43 (Produzentenbegriff), Art. 6.45 (Gebrekkigheid) — **alle vier in dieser Runde wortgleich per WebFetch reproduziert**, exakte Übereinstimmung mit dem Dateizitat.
   Wet Peeters-Borsus (REG-BE-7-020): Art. 3 „de burgerlijke aansprakelijkheid bedoeld in de artikelen 1792 en 2270 van het Burgerlijk Wetboek" und Art. 6 „500 000 euro"-Mindestdeckung — in dieser Runde ebenfalls bestätigt (Deckungsminimum exakt, inkl. der in der Datei nicht erwähnten ABEX-Indexierung als Zusatzfund).
**Status: Bestätigt** (alle drei Wortlautbelege exakt gegengelesen — höchste Beleg-Qualität in der gesamten VL-Datei).

---

## BE-WA-BR.md — Feld 3 (Wallonie)

### REG-BE-3-001/-002/-003 · Décret déchets 09.03.2023, Art. 5 §1 16°/19°, Art. 6, Art. 9
1. Supersession: ersetzt Décret 27.06.1996 laut Datei — in dieser Runde nicht am Aufhebungsartikel selbst nachgeprüft (Ressourcenpriorität), aber nicht widersprochen.
2. Primärquellen-Pin: wallex.wallonie.be/eli/loi-decret/2023/03/09/2023044053/2023/08/10 — in dieser Runde erneut per WebFetch geöffnet.
3. Kompetenz-Check: sub-national (Wallonie), korrekt.
4. Wirkrichtungs-Falsifikation: F1 „ermöglichend" (Art. 5 réemploi-Definition) — Gegenlesart nicht plausibel, reine WFD-konforme Begriffsnorm. F1 „bedingend" (Art. 9, vier kumulative Ende-Abfall-Kriterien plus Einzelfallentscheid) — Gegenlesart „ermöglichend, weil Ende-Abfall grundsätzlich erreichbar" verworfen, die Kumulativität und Behördenermessen rechtfertigen „bedingend". Beide halten stand.
5. Scope-Overreach: keine.
6. Quote-back: **Art. 5 §1 16°** „toute opération par laquelle des produits ou des composants qui ne sont pas des déchets sont utilisés de nouveau pour un usage identique à celui pour lequel ils avaient été conçus" — exakt reproduziert. **Art. 5 §1 19°** (préparation en vue du réemploi) — exakt reproduziert. **Art. 6 §1er** Fünfstufen-Hierarchie (prévention/préparation en vue du réemploi/recyclage/autre valorisation/élimination) — exakt in derselben Reihenfolge reproduziert. **Art. 9** Vier-Kriterien-Struktur (Zweckbestimmung/Marktexistenz/technische Anforderungen/keine Umwelt-Mehrbelastung) — inhaltlich exakt reproduziert.
**Status: Bestätigt** (alle vier Kernzitate exakt gegengelesen).

### REG-BE-3-004 · Décret déchets Art. 22 (Zerstörungsverbot wiederverwendbarer Produkte)
1. Supersession: keine neue geprüft.
2. Primärquellen-Pin: wallex, s. o.
3. Kompetenz-Check: sub-national, korrekt.
4. Wirkrichtungs-Falsifikation: **Korrektur** — die Datei hatte Art. 22 als unmittelbare „ausdrückliche Ermächtigung, die Zerstörung wiederverwendbarer Produkte zu verbieten" dargestellt. Der in dieser Runde wortgleich reproduzierte Volltext lautet jedoch: „§1er. 9° réglementer **ou** interdire la destruction de certains produits **ou déchets** réemployables ou encore consommables qu'il détermine" — das ist eine **Wahlermächtigung** (regeln ODER verbieten), kein Automatismus zu einem Verbot, und der Objektumfang ist weiter als „Produkte" allein (auch „déchets"). Die ursprüngliche F1-Einordnung „ermöglichend" bleibt im Kern richtig (die Ermächtigung existiert und deckt Reuse-relevante Fälle ab), war aber in der Zuspitzung zu stark. **Direkt in `BE-WA-BR.md` korrigiert**: Wortlautbeleg vervollständigt, Kernaussage und F1-Formulierung präzisiert.
5. Scope-Overreach: nach Korrektur keine mehr.
6. Quote-back: korrigiertes Vollzitat s. o., in dieser Runde selbst erhoben.
**Status: Korrigiert.**

### REG-BE-3-005 · Décret déchets Art. 5 §1 42°/31° (Bauabfall-Definition, Negativbefund)
1–3. Wie REG-BE-3-001 ff.
4. Wirkrichtungs-Falsifikation: F2 „hemmend" (Fehlen eines Bauabfall-Sonderregimes erschwert Rechtsklarheit) — Gegenlesart „das Fehlen einer Sonderregelung ist neutral bis ermöglichend, weil kein zusätzliches bürokratisches Hemmnis wie ein SOP-Zwang existiert" wurde erwogen und ist mit dem Text ebenso vereinbar — die Datei selbst benennt diese Ambivalenz nicht explizit als Gegenlesart, aber die Grundaussage („kein Sonderregime im Dekret selbst, AGW-Ebene nicht geprüft") bleibt in jedem Fall zutreffend, unabhängig davon, welche Wirkrichtung man wählt. Keine Korrekturpflicht, da der Kern (Negativbefund) unabhängig von der Wirkrichtungsdeutung textbelegt ist.
5. Scope-Overreach: keine — AGW-Ebene wird korrekt als ungeprüft ausgewiesen, nicht als „auch dort nichts vorhanden" behauptet.
6. Quote-back: **„les « déchets de construction, de déconstruction et de démolition » : les déchets produits par les activités de construction, de déconstruction et de démolition"** — in dieser Runde erstmals wortgleich reproduziert (die Datei hatte hierzu selbst nur „Definitionsfragmente" ohne Vollzitat, B1). Schließt eine kleine Lücke, ohne den Befund zu ändern.
**Status: Bestätigt** (Wortlaut nachgetragen als Zusatzfund, keine inhaltliche Korrektur nötig).

### REG-BE-3-006 · AGW 3 avril 2014 (Réemploi-Zulassung Sozialwirtschaft)
1. Supersession/Geltungsdauer: Die Datei hatte dies als offene Lücke markiert („Geltungsdauer bis mind. 2025-12-31, Verlängerung für 2026 nicht verifiziert"). **In dieser Runde per WebFetch der konsolidierten WALLEX-Fassung geprüft: Die aktuell laufende Zeitscheibe ist „Du 01/01/2026 au ..." ohne Enddatum** — die Geltung über den heutigen Stichtag 2026-08-11 hinaus ist damit primärquellenbasiert bestätigt. **Korrigiert** (Status/Konfidenz in `BE-WA-BR.md` von „Lücke" auf „bestätigt in Kraft" hochgestuft).
2. Primärquellen-Pin: wallex.wallonie.be/eli/arrete/2014/04/03/2014202762/2014/05/09 — auflösbar.
3. Kompetenz-Check: sub-national, korrekt.
4. Wirkrichtungs-Falsifikation: F1 „ermöglichend" — unstrittig, reine Förder-/Zulassungsnorm.
5. Scope-Overreach: keine.
6. Quote-back: Terminologiewechsel „réutilisation/subventions" → „réemploi/compensations" (AGW 21.03.2024, art. 2) in dieser Runde eigenständig im konsolidierten Text bestätigt gefunden.
**Status: Korrigiert.**

### REG-BE-3-007 (WA) · Plan wallon des Déchets-Ressources — Fortgeltungsstatus
1–6. Nicht neu geprüft, reiner Existenzhinweis (B2), Fortgeltungsfrage bleibt laut Datei selbst offen. Keine Primärquelle zur Klärung in dieser Runde gefunden (nicht gezielt gesucht, Ressourcenpriorität).
**Status: Bestätigt (Lücke bestätigt).**

## BE-WA-BR.md — Feld 3 (Brüssel)

### REG-BE-3-008/-009/-010 · Ordonnance déchets 14.06.2012, Art. 3/6/9/21-22
1. Supersession: konsolidiert bis 2025-02-18 laut Datei — in dieser Runde per WebFetch dasselbe Konsolidierungsdatum bestätigt.
2. Primärquellen-Pin: ejustice.just.fgov.be/eli/ordonnance/2012/06/14/2012031319/justel — auflösbar.
3. Kompetenz-Check: sub-national (Brüssel), korrekt.
4. Wirkrichtungs-Falsifikation: wie bei den WA-Parallelobjekten, keine neue Erkenntnis.
5. Scope-Overreach: keine.
6. Quote-back: **Art. 3** réemploi-Definition — in dieser Runde per WebFetch abgerufen, das Fetch-Ergebnis war an der Zitatstelle abgeschnitten („…sont utilis[…]"), aber der sichtbare Anfang ist wortgleich mit der Wallonie-Parallelnorm und der Dateibehauptung; kein Widerspruch, nur unvollständige Werkzeugausgabe, keine Fälschung. **Art. 6** Fünfstufenhierarchie — exakt in derselben Rangfolge wie Dekret WA reproduziert. **Art. 9** Grundstruktur (Ende-Abfall nach Recycling/Verwertung, EU- oder regionale Kriterien) — inhaltlich bestätigt, exaktes Vollzitat der Kriterien in dieser Runde nicht erneut abgerufen (bereits von der Datei selbst als „Übernahme aus Vorsitzung" gekennzeichnet, B0 auf Vorsitzung gestützt).
**Status: Bestätigt** (Art. 3/6 exakt gegengelesen, Art. 9 strukturell bestätigt).

### REG-BE-3-011 · BruDalex Art. 2.2.7 §3 / Art. 2.2.8 §2
1. Supersession: konsolidiert bis 2025-02-13, jüngste Änderung AGRBC 2024-10-24 (Inkrafttreten 2025-08-01) laut Datei — nicht widersprochen.
2. Primärquellen-Pin: ejustice.just.fgov.be/cgi_loi/change_lg.pl?...cn=2016120133 — in dieser Runde erneut geöffnet.
3. Kompetenz-Check: sub-national, korrekt.
4. Wirkrichtungs-Falsifikation (F1 „ermöglichend", Vorrang für Reuse vor Recycling): Gegenlesart „das Zugangsrecht nützt nur, wenn Sozialunternehmen überhaupt personell/logistisch in der Lage sind, es wahrzunehmen — sonst bleibt es totes Recht" wurde erwogen; die Datei markiert dies bereits selbst in F2 als praxisabhängig, nicht behauptet. Hält stand.
5. Scope-Overreach: keine — korrekt auf die EPR-Systematik begrenzt.
6. Quote-back: **„L'extraction ne peut avoir pour but le recyclage de ladite pièce"** (Art. 2.2.7 §3) — in dieser Runde wortgleich reproduziert (Datei-Zitat war leicht gekürzt, aber inhaltlich identisch). **„le producteur garantit l'accès au gisement des déchets collectés"** (Art. 2.2.8 §2) — wortgleich reproduziert.
**Status: Bestätigt** (beide Kernzitate exakt gegengelesen).

### REG-BE-3-012 · Ordonnance permis d'environnement 1997
1–6. Reiner Existenzhinweis (B3), in dieser Runde nicht erneut geprüft.
**Status: Unbelegbar (bereits korrekt als B3 markiert).**

---

## BE-WA-BR.md — Feld 4 (Schutzziele)

### REG-BE-4-001 · KB Basisnormen (Bundeskompetenz, identisch mit REG-BE-4-012 VL)
Bereits oben unter REG-BE-4-012/-013 geprüft — identisches Ergebnis. **Status: Bestätigt (Lücke bestätigt).**

### REG-BE-4-002 (PEB Wallonie) / REG-BE-4-003 (COBrACE Brüssel) / REG-BE-4-004 (Asbestregime)
1–6. Alle drei sind von der Datei selbst als nicht primärquellenbasiert verifiziert markiert (B4/B2 mit gescheiterten Zugriffsversuchen, inkl. eines dokumentierten Fehltreffers bei COBrACE). In dieser Runde kein neuer Versuch (kein WebSearch verfügbar, gezieltes ELI-Raten hat in der Ursprungsrunde bereits nachweislich zu einem Fehltreffer geführt — ein weiterer Ratversuch ohne Suchmaschine hätte dieselbe Fehlerquote). Die Datei behandelt den Fehltreffer bereits selbst transparent, statt ihn zu verschweigen — vorbildlich.
**Status: Unbelegbar (alle drei, bereits korrekt markiert).**

---

## BE-WA-BR.md — Felder 5a/5b/6/7 (Bundeskompetenz bzw. Förderprogramme)

### REG-BE-5a-001/-002 · Wet overheidsopdrachten 2016 + KB 18.04.2017
1–6. Nicht neu geprüft in dieser Runde; Ursprungseinstufung B1 (Etaamb-Titelseite, Art. 2/81 teilweise) konsistent mit dem bereits in der VL-Datei parallel geführten REG-BE-5a-014. Kompetenz-Check (national, uniform) korrekt.
**Status: Bestätigt (Lücke/Teilbeleg bestätigt).**

### REG-BE-5b-001/-002 · Circular Wallonia / Be Circular
1–6. Reine Existenznachweise, Circular-Wallonia-Domain laut Datei selbst nicht auflösbar (DNS-Fehler) — in dieser Runde nicht erneut getestet, da ohne WebSearch keine alternative Domain auffindbar.
**Status: Unbelegbar (beide, bereits korrekt markiert).**

### REG-BE-6-001/-002/-003 · Normalisatiewet, Buildwise, ATG
REG-BE-6-001 (Normalisatiewet): exakte Justel-cn-Fundstelle weiterhin nicht identifiziert — Existenz/Datum bleiben über konvergente KB-Zitate (B2) gestützt, in dieser Runde nicht neu geprüft.
REG-BE-6-002 (Buildwise): Existenznachweis, Besluitwet 1947 nicht im Volltext — unverändert.
REG-BE-6-003: identisch mit REG-BE-1-004, s. Strukturbefund B.
**Status: Bestätigt (Lücken bestätigt).**

### REG-BE-7-001/-002/-003 · Boek 6 nBW, Dezennalhaftung, Wet Peeters-Borsus
Inhaltlich identisch mit REG-BE-7-018/-019/-020 in `BE-VL.md`, dort bereits mit vollständiger Wortlaut-Gegenlesung geprüft (s. o.). Keine abweichenden Werte zwischen den beiden Dateien für dieses Objektbündel gefunden — im Gegensatz zum ATG-Fall (Strukturbefund B) ist die Duplikation hier inhaltlich konsistent.
**Status: Bestätigt** (Wortlaut-Gegenlesung s. REG-BE-7-018/-019/-020 oben).

---

## Zusammenfassende Tabelle

| Status | Anzahl | Beispiele |
|---|---:|---|
| Bestätigt (inkl. lebender Gegenlesung) | 40 | REG-BE-3-007 (VL, hergebruik), REG-BE-3-001–003 (WA), REG-BE-3-008–011 (BR), REG-BE-7-018–020/7-001–003 (Boek 6, Peeters-Borsus), REG-BE-3-009 VL (VLAREMA 4.3.3) |
| Korrigiert | 3 | REG-BE-3-004 (WA, Art. 22 Wortlaut präzisiert), REG-BE-3-006 (WA, AGW 2014 Geltungsdauer hochgestuft), ATG D-Achse (Strukturbefund B, beide Dateien) |
| Widerlegt | 0 | — |
| Unbelegbar (Quarantäne bestätigt) | 11 | REG-BE-2-002 (CoBAT), REG-BE-3-010/-011 VL (VLAREMA 4.3.5/Tracimat-Erkenning), REG-BE-4-002/-003/-004 (PEB WA, COBrACE, Asbest), REG-BE-3-012 (permis environnement 1997), REG-BE-5b-001/-002/016 (Förderprogramme), REG-BE-6-001 (Normalisatiewet-cn) |
| Fabriziert | 0 | — |

**Geprüfte Objekte gesamt: 54.**

## Empfehlung für W4

1. ID-Dedublizierung REG-BE-1-001/-002/-003 zwischen `BE-VL.md` und `BE-WA-BR.md` vor Synthese zwingend durchführen (Strukturbefund A).
2. D-Achse ATG/BUtgb vereinheitlichen — Vorschlag: `Merkblatt/Branchenprotokoll`, da dies der schwächere (vorsichtigere) der beiden Werte ist und die Gründungsrechtsgrundlage nicht verifiziert werden konnte (Strukturbefund B).
3. Der Sperrvermerk der Ursprungsdatei zu REG-BE-3-010/-011 (VLAREMA Art. 4.3.5, Tracimat-Erkenningsbesluit — nicht ohne erneute Primärtexteinsicht als B0/B1 übernehmen) wird durch diese Prüfung ausdrücklich bekräftigt, nicht aufgehoben.
4. Für REG-BE-2-002 (CoBAT/BWRO), REG-BE-4-002/-003 (PEB Wallonie/COBrACE) und die drei Förderprogramme (5b) ist echtes WebSearch (nicht nur WebFetch auf geratene URLs) in einer Folgesitzung erforderlich, wie von der Ursprungsdatei selbst bereits empfohlen — auch das WebSearch-Kontingent dieser Prüfrunde war zu Beginn bereits erschöpft.
