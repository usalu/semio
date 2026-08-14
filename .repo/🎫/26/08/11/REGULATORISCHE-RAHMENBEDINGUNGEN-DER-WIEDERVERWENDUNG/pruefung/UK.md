# Prüfprotokoll UK — Adversarische Prüfung der Ernte-Dateien

**Projekt:** BBSR/Zukunft Bau „Entwerfen mit Bestand" (Az. 10.08.18.7-25.06, LUH Hannover + UdK Berlin)
**Auftrag:** Adversarische Prüfung von `roh/UK-F1-3.md`, `roh/UK-F4-7.md`, `roh/UK-4Nationen.md` (Quellenkarte `roh/UK-quellen.md` nur als Referenz, nicht Prüfgegenstand im engeren Sinn, aber mitkorrigiert, da ein dort fabrizierter Fehler in zwei Folgedateien fortlebte).
**Prüfdatum:** 2026-08-13 · Amtssprache Englisch, alle Wortlautbelege live im Original nachgeprüft, wo nicht anders vermerkt.
**Methodik:** WebFetch auf legislation.gov.uk, eur-lex.europa.eu, gov.uk, gov.scot, niassembly.gov.uk, knowledge.bsigroup.com. WebSearch war im Verlauf dieser Prüfsitzung kontingentiert/erschöpft (Sitzungslimit erreicht nach einer Anfrage) — alle weiteren Prüfungen liefen ausschließlich über WebFetch auf gezielt konstruierte/aus den Ernte-Dateien übernommene URLs. Das ist eine reale Einschränkung dieser Prüfstufe und wird nicht verschwiegen.

**Umfang der Tiefenprüfung:** Die drei Ernte-Dateien enthalten zusammen rund 45–50 Regelungsobjekt-Blöcke, davon ca. 24 in `UK-4Nationen.md` und ca. 15 in `UK-F1-3.md`, die zu erheblichen Teilen dieselben Instrumente unter denselben ID-Präfixen (aber teils unterschiedlicher Nummerierung, s. u.) doppelt führen. Bei diesem Umfang war eine vollständige Sechs-Punkte-Prüfung jedes Einzelobjekts mit Primärtext-Live-Abgleich innerhalb dieser Sitzung nicht leistbar. Es wurde daher (a) jedes Objekt mit hoher Trageweite oder explizit als „gesichert"/B1 markiertem Wortlautzitat stichprobenartig gegen die Primärquelle gegengelesen, (b) jedes Objekt mit auffälligen Widersprüchen zwischen den drei Dateien vertieft geprüft, und (c) alle übrigen Objekte auf strukturelle Konsistenz (ID-Schema, Sub-Ebenen-Angaben, Quellen-Grading) durchgesehen, aber nicht erneut einzeln primärquellenlive verifiziert. Diese Objekte sind unten als „nicht vertieft geprüft, strukturell plausibel" gekennzeichnet.

---

## A. Kritische Funde (mit Korrektur in den Ernte-Dateien)

### A1 · REG-GB-1-001 (UK-CPR / SI 2013/1387) — Supersession übersehen — **KORRIGIERT**

Beide Extraktionsdateien (`UK-F1-3.md`, `UK-4Nationen.md`) behaupteten pauschal, VO (EU) 2024/3110 sei „kein Bestandteil des UK-Rechts", Großbritannien verbleibe vollständig auf dem reuse-schweigenden Stand der alten VO 305/2011, und die GB/NI-Divergenz (reuse-schweigend vs. reuse-explizit) sei sauber und vollständig.

**Falsifikationsversuch:** Supersessions-Nachweis (Pflichtcheck 1) am Primärtext live nachgeprüft, nicht nur Statuszeile „no known outstanding effects" (die nur ausstehende, noch nicht eingearbeitete Änderungen ausschließt, nicht vergangene).

**Befund:** *The Construction Products (Amendment) Regulations 2025* (SI 2025/1172) hat Reg. 2(1) von SI 2013/1387 mit Wirkung **08.01.2026** geändert (Fußnote [F4], live gelesen: „Words in reg. 2 substituted (8.1.2026) by The Construction Products (Amendment) Regulations 2025 (S.I. 2025/1172), regs. 1(1), 9(2)(a)"). Die Definition „the EU Construction Products Regulation" lautet seither: „Regulation (EU) 305/2011 and Regulation (EU) 2024/3110" — beide Instrumente. SI 2025/1172 ändert zusätzlich Art. 16A, 16B, 16C, 59A, 59B der retained 2011-Verordnung; nach den einsehbaren Änderungsvorschriften geht es dabei um Compliance-Pfad-Äquivalenz (Konformitätsnachweis nach 305/2011 **oder** nach 2024/3110), nicht nachweisbar um Übernahme der reuse-spezifischen Definitionsartikel (Art. 3 Nr. 20/25/26) selbst.

Diese Novellierung liegt **innerhalb** des für die Erhebung maßgeblichen Zeitraums (Stichtag 2026-08-11, Novelle wirksam 08.01.2026) und hätte von einer „as-amended zum 2026-08-11"-Prüfung erfasst werden müssen. Beide Ernte-Dateien haben sie nicht erfasst, obwohl beide ausdrücklich behaupten, die Statuszeile geprüft zu haben.

**Einordnung:** Die Kernaussage „GB hat kein ausdrückliches Reuse-Regime" bleibt nach dieser Prüfung voraussichtlich richtig (die reuse-spezifischen Artikel der VO 2024/3110 wurden nicht als übernommen identifiziert), aber die **pauschale** Formulierung „VO 2024/3110 ist kein Bestandteil des UK-Rechts" ist seit 08.01.2026 falsch/zu undifferenziert, und die GB/NI-Divergenz (zentraler Befund beider Dateien) ist für die Konformitätsbewertungspfade nicht mehr so trennscharf wie behauptet.

**Korrektur:** In `roh/UK-F1-3.md` und `roh/UK-4Nationen.md` bei REG-GB-1-001 direkt eingearbeitet (F1, Kernaussage, Wortlautbeleg, Quelle, Status, Relationen, Konfidenz).

**Status: KORRIGIERT.**

### A2 · REG-GB-6-002 / REG-UK-6-002 (BS 8905:2011 Zurückziehungsdatum) — interner Widerspruch, ein Wert falsch — **KORRIGIERT**

Direkter Widerspruch zwischen den Dateien: `UK-quellen.md` (Punkt 6.2) und `UK-4Nationen.md` (REG-GB-6-002) nannten als Zurückziehungsdatum **08.12.2023**. `UK-F4-7.md` (REG-UK-6-002) nannte davon abweichend **28.01.2026** und vermerkte selbst einen „Korrekturhinweis gegenüber der Quellenkarte", ohne dass diese Korrektur in die anderen beiden Dateien zurückgespielt wurde.

**Falsifikationsversuch:** Live-Abruf der BSI-Primärquelle (knowledge.bsigroup.com/products/framework-for-the-assessment-of-the-sustainable-use-of-materials-guidance).

**Befund:** Primärquelle zeigt aktuell „Published: 31 Aug 2011" · „Withdrawn: 28 Jan 2026". Der Wert aus `UK-F4-7.md` ist damit bestätigt korrekt; der Wert in `UK-quellen.md` und `UK-4Nationen.md` (08.12.2023) ist falsch.

**Korrektur:** In `roh/UK-quellen.md` (Punkt 6.2 und Supersession-Tabelle) und `roh/UK-4Nationen.md` (REG-GB-6-002) auf 28.01.2026 korrigiert, mit Verweis auf die live nachgeprüfte Primärquelle.

**Status: KORRIGIERT.**

### A3 · REG-GB-1-002 (gov.uk-Guidance-Datum) — Datumsabweichung zwischen den Dateien — **KORRIGIERT**

`UK-F1-3.md` (REG-GB-1-002) nannte als Stand der gov.uk-Guidance „last updated 26.02.2025". `UK-4Nationen.md` (REG-GB-1-002) nannte „21.05.2025" für dieselbe Seite.

**Falsifikationsversuch:** Live-Abruf von gov.uk/guidance/construction-products-regulation-in-great-britain.

**Befund:** Seite zeigt aktuell „last updated 21 May 2025". Der Wert in `UK-4Nationen.md` war korrekt, der Wert in `UK-F1-3.md` falsch. Der Kernwortlaut „The CE mark will continue to be available when placing construction products on the market in Great Britain." ist in beiden Dateien korrekt zitiert (Quote-back bestätigt); die Seite enthält weiterhin keine Erwähnung von wiederverwendeten/gebrauchten Bauprodukten oder VO 2024/3110 (Negativbefund bestätigt).

**Korrektur:** In `roh/UK-F1-3.md` auf 21.05.2025 korrigiert.

**Status: KORRIGIERT.**

### A4 · REG-GB-3-005 / REG-GB-3-001 (EPA 1990, s. 29(5A)(c) „recovery"-Definition) — Kompetenz-/Territorialitäts-Zweifel — **FLAG, nicht abschließend geklärt**

Beide Dateien zitieren s. 29(5A)(c) EPA 1990 als reuse-nahe „recovery"-Definition und ordnen sie der A-Achse „national (England, Wales …)" zu.

**Falsifikationsversuch (Kompetenz-Check, Pflichtcheck 3):** Live-Abruf von legislation.gov.uk/ukpga/1990/43/section/29.

**Befund:** Der zurückgegebene Volltext lautet vollständig: „'recovery' … means any of the operations listed in Part III of Schedule 4 to the **Waste Management Licensing (Scotland) Regulations 2011**, and any other operation the principal result of which is waste serving a useful purpose by replacing other materials which would otherwise have been used to fulfil a particular function, or waste being prepared to fulfil that function, in a plant or in the wider economy". Der von beiden Ernte-Dateien zitierte Halbsatz („waste serving a useful purpose by replacing other materials …") ist wörtlich korrekt zitiert — aber der Satz referenziert im vorangehenden Halbsatz eine **schottische** Verordnung (Waste Management Licensing (Scotland) Regulations 2011), was Zweifel weckt, ob die live abgerufene Fassung tatsächlich die für England/Wales geltende Parallelfassung von s. 29(5A) ist oder ob hier eine territorial getaggte Fassung (ggf. mit Scotland-Extent-Marker) durchschlägt, die nicht identisch mit der England/Wales-Fassung sein muss.

**Einordnung:** Dies konnte in dieser Prüfsitzung **nicht abschließend aufgelöst** werden (kein Zugriff auf die extent-getaggte Rohfassung mit [E][W][S]-Markierungen über das verfügbare WebFetch-Tool). Es handelt sich nicht um eine bestätigte Falsifikation, sondern um einen begründeten Zweifel, der vor der Synthesestufe zu klären ist.

**Korrektur:** In beiden Dateien (`UK-F1-3.md` REG-GB-3-005, `UK-4Nationen.md` REG-GB-3-001) als Flag/Kompetenz-Check-Hinweis ergänzt, ohne die A-Achse selbst zu verändern (keine hinreichende Evidenz für „Widerlegt").

**Status: unklar / Quarantäne-Flag gesetzt, nicht als Faktum zu werten bis geklärt.**

---

## B. Strukturelle Funde (nicht primärquellen-, sondern konsistenzbezogen)

### B1 · ID-Schema-Bruch zwischen den Dateien

`UK-F1-3.md` legt in der eigenen Präambel ausdrücklich fest: „ID-Konvention: ISO2 'GB' für Vereinigtes Königreich … als Gesamtjurisdiktion" und verwendet konsequent `REG-GB-*`. `UK-4Nationen.md` folgt demselben Schema (`REG-GB-*`). **`UK-F4-7.md` bricht dieses Schema durchgängig und verwendet stattdessen `REG-UK-*`** für exakt dieselbe Jurisdiktion (z. B. REG-UK-4-001, REG-UK-5A-001, REG-UK-6-002, REG-UK-7-001). Das ist kein Wortlautfehler, aber ein Kompetenz-/Konsistenzfehler auf Erhebungsebene: Für die Synthesestufe (Zusammenführung aller Länder-Ernten in ein gemeinsames Regelungsobjekt-Register) führt dieser Bruch dazu, dass Relationen wie „konkretisiert REG-GB-2-001" (aus `UK-F4-7.md`, REG-UK-4-001) auf eine ID verweisen, die es unter diesem Präfix in derselben Datei gar nicht gibt, und dass ein automatisiertes Zusammenführen nach ID-Präfix `UK-F4-7.md` fälschlich als andere Jurisdiktion einordnen könnte.

**Empfehlung:** Vor Synthesestufe `UK-F4-7.md` durchgängig von `REG-UK-` auf `REG-GB-` umbenennen (Umnummerierung auf freie Blocknummern in Feld 4–7, da die Nummernkreise 1–3 bereits belegt sind).

**Status: Korrigiert (dokumentiert, nicht in den Dateien selbst umbenannt — Umbenennung aller ID-Referenzen wäre ein eigener redaktioneller Eingriff mit Kollisionsrisiko zu B2 unten und wird hier nur empfohlen, nicht ausgeführt, um keine neuen Inkonsistenzen ohne Rücksprache zu erzeugen).**

### B2 · ID-Kollision zwischen `UK-F1-3.md` und `UK-4Nationen.md` in Feld 2

Beide Dateien vergeben in Feld 2 dieselben IDs an **unterschiedliche** Objekte:

| ID | `UK-F1-3.md` | `UK-4Nationen.md` |
|---|---|---|
| REG-GB-2-002 | Approved Document A (England) | Wales — Building Regulations 2010 |
| REG-GB-2-003 | Eurocodes UK National Annexes | Schottland — Building (Scotland) Regulations 2004 |
| REG-GB-2-004 | SCI P427 | Nordirland — Building Regulations (NI) 2012 |
| REG-GB-2-005 | Wales — Building Regulations 2010 | England — Approved Document A |
| REG-GB-2-006 | Schottland — Building (Scotland) Regulations 2004 | Wales — eigenständige Approved Documents (neuer Fund) |
| REG-GB-2-007 | Nordirland — Building Regulations (NI) 2012 | Eurocodes UK National Annexes |
| REG-GB-2-008 | *(nicht vorhanden)* | SCI P427 |

Dieselbe ID bezeichnet in den beiden Dateien sieben verschiedene Objektpaare. Das ist kein Falschzitat einzelner Fakten, aber ein gravierendes strukturelles Risiko: Jede Relation, die aus einer anderen Ernte-Datei (z. B. einer Materialfamilien- oder Querschnitts-Erhebung in Arbeitspaket W3) auf „REG-GB-2-002" verweist, ist ohne Kenntnis, aus welcher der beiden Dateien die ID stammt, nicht mehr eindeutig auflösbar.

**Status: Korrigiert (dokumentiert als Blocker für W4/Synthese — Auflösung erfordert redaktionelle Entscheidung, welche Datei als führende Quelle gilt bzw. eine gemeinsame Neu-Nummerierung; nicht einseitig durch diese Prüfung entschieden, da beide Extraktionsstufen als eigenständige Session-Artefakte mit je eigener Binnenlogik angelegt sind).**

### B3 · Redundanz zwischen `UK-F1-3.md` und `UK-4Nationen.md`

`UK-4Nationen.md` wiederholt inhaltlich fast vollständig die Felder 1–3 aus `UK-F1-3.md` (dieselben Instrumente, teils wortgleiche Kernaussagen), zusätzlich um eine Vollerhebung der vier Nationen in Feld 2 erweitert. Für die Synthesestufe bedeutet das doppelte Pflege- und Fehlerfortpflanzungsrisiko (s. A2/A3, wo derselbe Fehler in einer Datei korrigiert, in der Schwesterdatei aber nicht zurückgespielt wurde). Kein Korrekturbedarf an einzelnen Fakten, aber Hinweis für die Zusammenführung: **`UK-4Nationen.md` sollte als die maßgeblichere/aktuellere Fassung für Felder 1–3 behandelt werden** (spätere Session, mehr live gelesene Primärtexte, z. B. EPA 1990 s. 34 und Procurement Act s. 12/13 dort im Volltext B1 statt nur B2), `UK-F1-3.md` primär als Erstfassung mit teils geringerer Beleg-Tiefe.

**Status: dokumentiert, keine Dateiänderung nötig.**

---

## C. Bestätigte Kernbelege (Quote-back erfolgreich, Pflichtcheck 6 bestanden)

Folgende Wortlautbelege wurden live gegen die Primärquelle gelesen und **wörtlich bestätigt** (Status: **Bestätigt**):

- **SI 2013/1387, Reg. 2(1)** (Grundfassung vor SI 2025/1172): „any other expression used in these Regulations and occurring in the 2011 Regulation shall have the same meaning as it has in that Regulation." — bestätigt, aber s. A1 zur Novellierung.
- **Building Regulations 2010, Schedule 1, Requirement A3**: „The building shall be constructed so that in the event of an accident the building will not suffer collapse to an extent disproportionate to the cause." — wortgleich bestätigt.
- **Waste (England and Wales) Regulations 2011, Reg. 12(1)**: vollständige Hierarchie „(a) prevention; (b) preparing for re-use; (c) recycling; (d) other recovery … (e) disposal" — wortgleich bestätigt, inkl. Abweichmöglichkeit „justified by life-cycle thinking".
- **Environmental Protection Act 1990, s. 34(1)**: „it shall be the duty of any person who imports, produces, carries, keeps, treats or disposes of controlled waste …" — wortgleich bestätigt.
- **Environmental Permitting (E&W) Regulations 2016, Sch. 3, U1**: „no more waste is used than is necessary" und 12-Monats-Lagerfrist, Mengentabellen (1.000/5.000/50.000 t) — bestätigt.
- **Procurement Act 2023, s. 12**: vollständiger Wortlaut (a)–(d) plus Abs. 2–4 (SME-Berücksichtigung) — wortgleich bestätigt; **kein** Kreislaufwirtschafts-/Reuse-Bezug im Volltext gefunden — Negativbefund der Ernte-Dateien bestätigt. In-Kraft-Datum 24.02.2025 bestätigt.
- **Procurement Act 2023, Schedule 11**: Aufhebung der Public Contracts Regulations 2015 (SI 2015/102) zum 24.02.2025 — bestätigt.
- **Building Safety Act 2022, s. 135 / neuer s. 4B Limitation Act 1980**: 15 Jahre prospektiv, 30 Jahre retrospektiv mit einjähriger Übergangsfrist — bestätigt (Kommissionierung 28.06.2022).
- **Defective Premises Act 1972, s. 1(1)**: Kernformulierung „workmanlike or professional manner … proper materials … fit for habitation" strukturell bestätigt (Volltext durch Tool-Zeichenlimit gekürzt zurückgegeben, aber übereinstimmend).
- **Consumer Protection Act 1987, s. 4(1)(e)**: „the state of scientific and technical knowledge at the relevant time was not such that a producer …" — wortgleich bestätigt.
- **Procurement Reform (Scotland) Act 2014, s. 9(1)**: vollständiger Wortlaut („improve the economic, social, and environmental wellbeing …") — wortgleich bestätigt.
- **Control of Asbestos Regulations 2012, Reg. 4**: Kooperationspflicht-Zitat wortgleich bestätigt.
- **Windsor Framework Democratic Scrutiny Committee, Ausschussbericht zu VO 2024/3110**: Zitate „its scope is extended to used products" und „the replacement EU act would not have a significant impact specific to everyday life of communities in Northern Ireland in a way that is liable to persist" — beide wortgleich bestätigt; Stormont Brake nachweislich **nicht** ausgelöst; Berichtsdatum präzisiert: „Ordered … to be published 23 January 2025" (in den Ernte-Dateien nur als „2025" vage datiert — kleine Präzisierung, kein Fehler).
- **VO (EU) 2024/3110, Erwägungsgründe 15, 34, 36**: alle drei Zitate wortgleich bei EUR-Lex bestätigt.
- **VO (EU) 2024/3110, Anwendungsdatum**: Art. 96 laut EUR-Lex-Metadatenfeld „Date of effect": Inkrafttreten 07.01.2025, allgemeine Anwendung **08.01.2026** (von den Ernte-Dateien korrekt übernommen), gestufte weitere Anwendung 08.01.2027; vollständige Aufhebung von VO 305/2011 erst zum 08.01.2040 (Detail, das in keiner Ernte-Datei erwähnt wird — kein Fehler, aber ergänzenswert für die Synthesestufe, da die Übergangskoexistenz beider VOen bis 2040 die GB/NI-Analyse weiter verkompliziert).
- **Waste (England and Wales) Regulations 2011, Teil 8**: Außerkraftsetzung durch S.I. 2026/873 Reg. 26 bestätigt (als „outstanding change", noch nicht abschließend datiert — Ernte-Dateien korrekt als „künftig außer Kraft" markiert, nicht als bereits erfolgt).
- **Building (Scotland) Regulations 2004**: S.S.I. 2025/417 als „yet to be applied" bestätigt — Ernte-Dateien korrekt als „noch nicht in Kraft" markiert.

---

## D. Nicht abschließend verifizierbare Objekte (Status unverändert: unbelegt/offen)

- **National Procurement Policy Statement (REG-GB-5-004 / 5B-002 / 5.3):** Erneuter Versuch, das PDF live zu extrahieren, scheiterte wie in allen drei Vorsessionen (Binärdaten, kein Fließtext). Die in den Ernte-Dateien zitierten Wendungen („optimising use of public funds by balancing effectiveness, efficiency and economy over the life-cycle of a product, service or works") konnten in dieser Prüfung **nicht** unabhängig gegengelesen werden. Der Negativbefund „kein expliziter Reuse-Bezug" bleibt auf Sekundärquellenniveau (B2/B3) und wird durch diese Prüfung weder bestätigt noch widerlegt, nur als weiterhin unbelegt am Primärtext bestätigt. **Status: Unbelegbar (technisch, nicht paywalled) — unverändert.**
- **SCI P427, NSSS Annex J:** In dieser Prüfung nicht erneut abgerufen (Ressourcenpriorisierung auf gesetzliche Primärquellen mit höherer Beweislast); Zugänglichkeits- und Bindungsstatus bleiben wie in den Ernte-Dateien selbst als „grenzwertig"/unklar geführt. **Status: nicht vertieft geprüft, unverändert.**
- **Approved Document A/B/L-Volltexte (PDF), Wales-Approved-Documents-Volltext, NI Technical Booklet D, Scotland Technical Handbook:** in dieser Prüfung nicht erneut abgerufen; die Ernte-Dateien kennzeichnen diese bereits ehrlich als B2/B4 ohne Volltextzugriff. **Status: nicht vertieft geprüft, strukturell plausibel, Lückenkennzeichnung der Ernte-Dateien wird nicht beanstandet.**
- **Alle übrigen Sub-Ebenen-Objekte (Wales/Schottland/Nordirland-Statuszeilen, NISR 2012/192-Novellenliste, WSI 2024/1268):** stichprobenartig gegengelesen (NISR 2012/192-Status „no known outstanding effects" bestätigt, Detailnovellenliste NISR 2022/71/2024/191 nicht einzeln gegengelesen); keine Widersprüche gefunden. **Status: Bestätigt (Statuszeile), nicht vertieft (Einzelnovellen).**

---

## E. Fallenliste-Abgleich (projektspezifische Prüfpunkte)

- **CPR-Falle:** Die Ernte-Dateien haben die Falle „VO 2024/3110 gilt ab 08.01.2026 mit ausdrücklichen Reuse-Regeln, 305/2011 läuft aus" grundsätzlich richtig erkannt und für Nordirland korrekt angewendet (Windsor Framework). Für **Großbritannien** haben sie jedoch übersehen, dass die eigene Supersessionsprüfung unvollständig war (SI 2025/1172, s. A1) — eine neue, in der Fallenliste nicht antizipierte Unterfalle: „GB bleibt vollständig unberührt von VO 2024/3110" ist seit 08.01.2026 nicht mehr uneingeschränkt richtig.
- **CE/UKCA-Falle** („CE-Anerkennung für Bauprodukte in UK unbefristet verlängert — NICHT UKCA-Pflicht"): korrekt vermieden und primärquellenbasiert bestätigt (s. C).
- **Alle übrigen Fallenlisten-Punkte** (NL Bouwbesluit, DE Bauregelliste, CH/EEA, BE VLAREMA, FR PEMD, EU-weite Abfallende-Kriterien, SE/DK/NO) sind für UK nicht einschlägig und wurden in den Ernte-Dateien zu Recht nicht behandelt.

---

## F. Zusammenfassung nach Status

| Status | Anzahl (dieser Prüfsitzung) |
|---|---|
| Bestätigt (Quote-back erfolgreich) | 17 Primärzitate/Kernbefunde |
| Korrigiert | 4 (REG-GB-1-001 in zwei Dateien gezählt als 1 sachlicher Fund, REG-GB-6-002/6.2, REG-GB-1-002-Datum, ID-Schema/-Kollision dokumentiert) |
| Widerlegt | 0 |
| Unklar/Flag (nicht abschließend geklärt) | 1 (s. 29(5A)(c) Territorialität) |
| Unbelegbar (technisch/weiterhin) | 1 (NPPS-Volltext) |
| Fabriziert | 0 — kein Fall gefunden, in dem ein Zitat oder eine Fundstelle frei erfunden statt nur veraltet/falsch datiert war. |

**abnick_verdacht: NEIN.** Die Ernte wirkte nicht auffällig fehlerfrei — sie enthielt einen echten, mittelschweren Supersessions-Fehler (SI 2025/1172, sieben Monate vor dem Stichtag in Kraft getreten und trotz expliziter „as-amended"-Behauptung übersehen), einen faktischen Datumsfehler, der sich durch zwei von drei Dateien zog, ohne dass eine bereits erfolgte Korrektur in der dritten Datei zurückgespielt wurde, sowie strukturelle ID-Kollisionen zwischen zwei Extraktionsstufen. Das sind genau die Art von Fehlern, die eine ehrliche, unter Zeitdruck mehrfach iterierte Recherche erwarten lässt — kein Muster, das auf verdeckte Fabrikation oder unplausible Perfektion hindeutet. Die selbstkritischen Lückenlisten am Ende jeder Ernte-Datei sind glaubwürdig und wurden durch diese Prüfung überwiegend bestätigt, nicht widerlegt.

---

## G. Offene Punkte für die nächste Stufe (nicht Teil dieses Prüfauftrags)

1. Materieller Inhalt der neuen Art. 16A–16C/59A–59B der retained VO 305/2011 (durch SI 2025/1172 eingefügt) im Volltext lesen — Klärung, ob tatsächlich keine reuse-spezifischen Regeln enthalten sind, oder nur Kennzeichnungs-/Konformitätspfad-Äquivalenz.
2. Territorialität von EPA 1990 s. 29(5A)(c) klären (England/Wales- vs. Scotland-Fassung).
3. ID-Kollision Feld 2 zwischen `UK-F1-3.md` und `UK-4Nationen.md` redaktionell auflösen vor Synthesestufe (W4).
4. ID-Schema-Bruch in `UK-F4-7.md` (`REG-UK-*` statt `REG-GB-*`) vor Synthesestufe korrigieren.
5. NPPS-Volltext mit alternativer Extraktionsmethode (z. B. HTML-Spiegelseite statt PDF) erneut versuchen.
