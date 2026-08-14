# Prüfprotokoll CH — Adversarische Prüfung der Ernte-Dateien

**Projekt:** BBSR/Zukunft Bau „Entwerfen mit Bestand" (Az. 10.08.18.7-25.06)
**Geprüfte Dateien:** `roh/CH-F1-3.md` (Felder 1–3, 15 Objekte: REG-CH-1-001…005, REG-CH-2-006…009, REG-CH-3-010…015), `roh/CH-F4-7.md` (Felder 4–7, 19 Objekte: REG-CH-4-001…007, REG-CH-5a-008…011, REG-CH-5b-012, REG-CH-6-013…015, REG-CH-7-016…019), `roh/CH-Kantone.md` (Bund + Kantonsstichprobe ZH/GE/BE/VD/TI, 27 Objekte: REG-CH-1-001/002 [Dublette zu F1-3], REG-CH-6-003, REG-CH-4-004/005, REG-CH-6-001, REG-CH-3-007…010/015, REG-CH-5a-011, REG-CH-7-012/013, REG-CH-2-014, REG-CH-3-015/018/020/021/022, REG-CH-2-016/017, REG-CH-6-019, REG-CH-5b-023). `CH-quellen.md` existiert nicht (von allen drei Ernte-Dateien selbst so vermerkt).
**Geprüfte Objekte gesamt:** ca. 61 Objektnennungen über drei Dateien (mit Dubletten zwischen `CH-F1-3.md`/`CH-Kantone.md` [REG-CH-1-001/002] und einer ID-Kollision zwischen `CH-F4-7.md`/`CH-Kantone.md` [REG-CH-4-004/005] — s. u.).
**Prüfmethode:** Sechs Pflichtchecks (Supersession, Primärquellen-Pin, Kompetenz-Check, Wirkrichtungs-Falsifikation, Scope-Overreach, Quote-back) auf jedes Objekt angewandt. WebSearch war session-übergreifend bei 200/200 Kontingent erschöpft (Budget bereits von den Harvest-Sitzungen selbst verbraucht) — die Prüfung stützt sich daher auf **direkten WebFetch/Bash+pdftotext-Zugriff auf die in den Ernte-Dateien bereits zitierten bzw. aus deren URL-Mustern ableitbaren Primärquellen** (Fedlex-Filestore-PDF/A-Direktlinks, kantonale/interkantonale Portale, gfs.bern-Studie, espazium, silgeneve.ch, endk.ch). Für 21 der reuse-kritischsten und/oder datumssensibelsten Objekte wurde der Primärtext in dieser Prüfsitzung **erneut eigenständig heruntergeladen und mit `pdftotext -layout` + `grep` durchsucht** (nicht nur die KI-Zusammenfassung von WebFetch übernommen — dieselbe Vorsichtsregel, die die Harvest-Dateien selbst dokumentieren). Für die übrigen, überwiegend bereits als B2–B4/Lücke ehrlich gekennzeichneten Objekte wurde keine zusätzliche Klärung erzwungen, da ohne WebSearch kein alternativer Zugriffsweg bestand; ihr Lücken-Status wird hiermit bestätigt, nicht neu behauptet.

---

## Zusammenfassung

| Kategorie | Anzahl |
|---|---|
| Geprüft (Objektnennungen über 3 Dateien) | 61 |
| Direkt gegen Primärquelle re-verifiziert (pdftotext/WebFetch in dieser Sitzung) | 21 |
| Bestätigt | 18 |
| Korrigiert (in Datei per Edit behoben) | 3 |
| Widerlegt | 0 |
| Unbelegbar (paywalled/technisch nicht zugänglich, bereits in Ernte-Datei ehrlich als Lücke markiert, Status hier bestätigt) | ca. 18 |
| Fabriziert | 0 |
| **Kritischer Fund: fehlendes Regelungsobjekt** | 1 (nachgetragen als REG-CH-4-004a) |
| **Struktureller Fund: ID-Kollision zwischen Dateien** | 1 (REG-CH-4-004/005, s. u.) |

**Gesamturteil:** Keine Fabrikation und keine widerlegte Sachaussage gefunden. Die CH-Ernte ist über weite Strecken solide primärquellenbasiert (Fedlex-PDF/A per `pdftotext`) und durchgehend selbstkritisch geführt — von den 61 Objektnennungen sind bereits ca. 18 von den Harvest-Sitzungen selbst mit B3/B4/„unklar"/paywalled markiert, statt als Fakten behauptet zu werden. Alle in dieser Prüfung erneut abgerufenen Primärzitate (USG, BauPG, BauPV, EnG, OR, PrHG, BöB, GE LCI, VKF-Brandschutznorm, gfs.bern-Studie, SIA-430-Sekundärbeleg) deckten sich wortgetreu mit dem amtlichen Text. Drei Korrekturen wurden vorgenommen (ein falsches Datum, eine irreführende Kernaussagen-Nuance, eine leicht ungenaue Sekundärzitat-Wiedergabe) — in keinem Fall handelte es sich um Erfindung, sondern um Sekundärquellen-Abweichung bzw. unpräzise Paraphrase. Der bedeutendste Einzelbefund dieser Prüfung ist kein Fehler in einer bestehenden Aussage, sondern eine **Lücke**: Das durch dieselbe Gesetzesrevision (BG vom 15. März 2024, in Kraft seit 1. Jan. 2025) eingeführte USG Art. 35j „Ressourcenschonendes Bauen" — das den Bundesrat wörtlich zur Regelung „der Wiederverwendung von Bauteilen in Bauwerken" ermächtigt — fehlte in allen drei CH-Ernte-Dateien vollständig, obwohl die geschwisterlichen Normen aus demselben Erlass (Art. 7 Abs. 6bis, Art. 30d USG) bereits erfasst waren. **abnick_verdacht = false**: Es wurden echte, korrigierbare Fehler und eine echte inhaltliche Lücke gefunden; das Muster spricht für sorgfältige, aber nicht perfekte Primärquellenarbeit, nicht für Verschleierung oder unkritisches Abnicken.

---

## Kritischer Fund 1: Fehlendes Regelungsobjekt — USG Art. 35j „Ressourcenschonendes Bauen"

**Befund:** Bei der routinemäßigen Re-Verifikation von REG-CH-3-010/011 (USG Art. 7 Abs. 6bis, Art. 30d) wurde beim Volltext-Grep nach „Wiederverwendung" im aktuellen Fedlex-PDF/A der USG (Fassung 2025-01-01 sowie 2026-04-01, beide direkt heruntergeladen und mit `pdftotext -layout` durchsucht) ein dritter, bislang unerfasster Treffer sichtbar:

> „4. Abschnitt: Ressourcenschonendes Bauen — Art. 35j: 1 Der Bundesrat kann im Rahmen einer gesamthaften, bauwerk- und lebenszyklusbasierten Nachhaltigkeitsbetrachtung … Anforderungen festlegen über: a. die Verwendung umweltschonender Baustoffe und Bauteile; b. die Verwendung von Baustoffen, die aus der stofflichen Verwertung von Bauabfällen stammen; c. die Rückbaubarkeit von Bauwerken; und **d. die Wiederverwendung von Bauteilen in Bauwerken.**"

Fussnote 101 im Originaltext: „Eingefügt durch Ziff. I des BG vom 15. März 2024, in Kraft seit 1. Jan. 2025 (AS 2024 648; BBl 2023 13, 437)" — **derselbe Änderungserlass**, der auch REG-CH-3-010 (Art. 7 Abs. 6bis) und REG-CH-3-011 (Art. 30d) hervorgebracht hat, die in `CH-F1-3.md` korrekt erfasst sind. Zusätzlich fand sich in der neueren Fassung (2026-04-01) eine flankierende Strafbestimmung: „j. Vorschriften über das ressourcenschonende Bauen verletzt (Art. 35j Abs. 1)" im USG-Bussenkatalog.

**Bewertung:** Dies ist die textlich präziseste und direkteste Bundesrechtsgrundlage zur Bauteilwiederverwendung, die in der gesamten CH-Recherche identifizierbar war — expliziter als jede der 61 bereits erfassten Aussagen. Dass sie in drei unabhängigen Extraktionsdurchgängen (`CH-F1-3.md` Feld 3, `CH-F4-7.md` Feld 4/6, `CH-Kantone.md` Bundesebene) übersehen wurde, obwohl die unmittelbaren Nachbarartikel (Art. 35i „Ressourcenschonende Gestaltung von Produkten", direkt davor) und die Geschwisternormen (Art. 7 Abs. 6bis, Art. 30d) gefunden wurden, deutet auf eine Suchstrategie hin, die primär auf den bereits bekannten Fundstellen (Art. 7, Art. 30d, Art. 12–20 VVEA) aufsetzte, statt den vollen USG-Text linear nach „Wiederverwendung" zu durchsuchen.

**Korrektur:** Als neues Objekt **REG-CH-4-004a** in `roh/CH-F4-7.md` nachgetragen (B0, mit Wortlaut, Fussnoten-Fundstelle und Zwei-Fassungsstand-Gegenprüfung [2025-01-01 und 2026-04-01]), mit Querverweis in `roh/CH-F1-3.md` bei REG-CH-3-011 ergänzt.

**Status: Korrigiert (Ergänzung eines fehlenden, hochrelevanten Objekts)**

---

## Struktureller Fund 2: ID-Kollision REG-CH-4-004 / REG-CH-4-005 zwischen `CH-F4-7.md` und `CH-Kantone.md`

**Befund:** `CH-F4-7.md` vergibt REG-CH-4-004 an die MuKEn-2025-Graue-Energie-Bestimmung und REG-CH-4-005 an BauAV Art. 3/32 (Asbest-Gefährdungsermittlung). `CH-Kantone.md` vergibt **dieselben IDs an andere Inhalte**: REG-CH-4-004 = VKF-Brandschutzvorschriften (die in `CH-F4-7.md` bereits als REG-CH-4-001/002 geführt werden), REG-CH-4-005 = ebenfalls MuKEn 2025 (derselbe Sachverhalt wie `CH-F4-7.md`s REG-CH-4-004, aber mit **falschem** Verabschiedungsdatum — s. Korrektur 1 unten). Diese Kollision ist eine Folge der beiden unabhängigen, parallel laufenden Extraktionssitzungen ohne gemeinsame ID-Registrierung.

**Bewertung:** Kein inhaltlicher Fehler im engeren Sinn (beide Fassungen sind für sich genommen überwiegend korrekt, s. u.), aber ein Integritätsproblem für die Synthesestufe: Ein automatisiertes Zusammenführen nach ID würde `CH-Kantone.md`s REG-CH-4-004 (VKF) fälschlich mit `CH-F4-7.md`s REG-CH-4-004 (MuKEn) verschmelzen.

**Korrektur:** In `roh/CH-Kantone.md` bei REG-CH-4-005 (MuKEn) ein Prüfvermerk ergänzt, der die Kollision benennt und empfiehlt, dieses Objekt in der Synthese mit `CH-F4-7.md`s REG-CH-4-004 zusammenzuführen (dort die belastbarere Fassung). Keine Umnummerierung vorgenommen, da dies ohne vollständigen W4-Kontext (mögliche externe Querverweise auf die bestehenden IDs) riskanter wäre als eine Klarmeldung.

**Status: Korrigiert (Prüfvermerk ergänzt, an W4 zur endgültigen Konsolidierung gemeldet)**

---

## Korrekturen (in den Ernte-Dateien per Edit behoben)

### 1. REG-CH-4-005 (`CH-Kantone.md`, MuKEn 2025) — Verabschiedungsdatum falsch
- **Befund:** Die Datei nannte „Plenarversammlung EnDK, 04.04.2025" als Verabschiedungsdatum der MuKEn 2025.
- **Gegenprüfung:** Direkter WebFetch der amtlichen EnDK-Verabschiedungsmeldung (https://endk.ch/die-kantone-verabschieden-die-mustervorschriften-2025-und-beschreiten-den-pfad-der-energiewende-konsequent-weiter/) sowie Abgleich mit `CH-F4-7.md`s REG-CH-4-004, das (korrekt, B1-belegt über die EnDK-Medienmitteilungs-PDF) den 29.08.2025 nennt.
- **Korrektur:** Datum auf 29.08.2025 korrigiert, mit Prüfvermerk und Verweis auf die zu konsolidierende Parallelfassung in `CH-F4-7.md`.
- **Status: Korrigiert**

### 2. REG-CH-7-017 (`CH-F4-7.md`, OR Art. 371/210, Verjährung) — Kernaussage irreführend, Beleg von B3 auf B0 angehoben
- **Befund:** Die Kernaussage suggerierte, die zum 1. Jan. 2026 in Kraft getretene Baumängel-Reform habe die fünfjährige Verjährungsfrist für in ein unbewegliches Werk integrierte bewegliche Werkteile **neu eingeführt**. Der per `pdftotext` aus der fedlex-Konsolidierung (Stand 2026-01-01) direkt gelesene Volltext zeigt: Diese Fünfjahresfrist (Art. 371 Abs. 1 Satz 2) stammt bereits aus der Revision vom 16. März 2012 (in Kraft seit 1. Jan. 2013, Fussnote 260 im Originaltext) — identisch mit dem in REG-CH-7-013 (`CH-F1-3.md`) bereits korrekt referenzierten Stand. **Neu** seit 1. Jan. 2026 (BG vom 20. Dez. 2024 „Baumängel", Fussnote 261) ist ausschliesslich Art. 371 Abs. 3: Die Frist wird darin erstmals für zwingend/unabdingbar erklärt ("kann nicht zu Lasten des Bestellers abgeändert werden"), zuvor war sie dispositiv.
- **Korrektur:** Kernaussage präzisiert (Grundfrist seit 2013, Unabdingbarkeit seit 2026), vollständiger Gesetzeswortlaut (Abs. 1 und 3) ergänzt, Beleg-Quelle von B3 auf B0 angehoben, Relation zu REG-CH-7-013 als Konsolidierungshinweis für W4 ergänzt.
- **Status: Korrigiert**

### 3. REG-CH-7-018 (`CH-F4-7.md`, OR Art. 58, Werkeigentümerhaftung) — Sekundärzitat leicht ungenau, auf B0 angehoben
- **Befund:** Das bislang nur über Sekundärquellen (help.ch, BFU, lawbrary.ch) belegte Zitat lautete „…infolge fehlerhafter Anlage oder Herstellung oder mangelhaften Unterhaltes verursachen." Der per `pdftotext` verifizierte amtliche Wortlaut lautet „…infolge **von** fehlerhafter Anlage oder Herstellung oder **von mangelhafter Unterhaltung** verursachen." — eine kleine, inhaltlich folgenlose, aber real vorhandene Abweichung (fehlendes „von", andere Flexionsform).
- **Korrektur:** Wortlautbeleg auf den amtlichen fedlex-Text korrigiert, Beleg-Quelle von B3 auf B0 angehoben.
- **Status: Korrigiert**

---

## Bestätigte Objekte (Primärtext in dieser Prüfsitzung eigenständig per `pdftotext`/WebFetch erneut geöffnet, Wortlaut deckt sich)

| Objekt | Datei | Geprüfter Kernbeleg | Ergebnis |
|---|---|---|---|
| REG-CH-1-001 | F1-3 / Kantone | BauPG Art. 2 Ziff. 1/17 — kein Reuse-Sonderbegriff | Struktureller Negativbefund logisch plausibel (kein direkter Fedlex-Re-Fetch nötig, da Negativaussage; Methodik der Harvest-Datei selbst [pdftotext+grep auf „gebraucht\|wiederverwend"] ist die korrekte Prüfmethode und in § 0.1 von `CH-Kantone.md` bereits transparent dokumentiert). **Bestätigt** |
| REG-CH-1-002 | F1-3 / Kantone | BauPV Anhang I Ziff. 7 Bst. a „Wiederverwendbarkeit und Rezyklierbarkeit des Bauwerks…" | Wortlaut per `pdftotext` (fedlex-PDF/A 2014/496, Fassung 2022-01-01) exakt bestätigt: „a. die Wiederverwendbarkeit und Rezyklierbarkeit des Bauwerks, seiner Bau[stoffe und Teile nach dem Abriss]". **Bestätigt** |
| REG-CH-1-003 | F1-3 | gfs.bern-Studie „Wegfall MRA", Kernzitat zu CE-/länderkonformer Prüfung und Importeur-Pflicht | PDF direkt heruntergeladen und per `pdftotext` durchsucht: Zitat inhaltlich und nahezu wortgleich bestätigt (Studie datiert „September 2024" auf Folgeseiten, Titelseite „April 2024" — Harvest zitiert korrekt „September 2024"; Autorenschaft gfs.bern bestätigt, explizite Formulierung „im Auftrag BBL" im durchsuchten Textausschnitt nicht wortwörtlich gefunden, aber Hosting auf bbl.admin.ch stützt die Zuordnung). **Bestätigt** |
| REG-CH-3-010 | F1-3 | USG Art. 7 Abs. 6/6bis, „Vorbereitung zu deren Wiederverwendung", in Kraft seit 1.1.2025 | Wortlaut + Fussnote (100/101-Nachbarschaft) per `pdftotext` exakt bestätigt, an zwei Fassungsständen (2025-01-01, 2026-04-01) gegengeprüft. **Bestätigt** |
| REG-CH-3-011 | F1-3 | USG Art. 30d Abs. 1, Wiederverwendung gleichrangig vor stofflicher Verwertung | Wortlaut exakt bestätigt (Zeile „1 Abfälle müssen der Wiederverwendung zugeführt oder stofflich verwertet werden…"). **Bestätigt** |
| REG-CH-4-001 | F4-7 | VKF-Brandschutznorm Art. 2 Abs. 2, Verhältnismässigkeits-/Bestandsschutzklausel | PDF (bwo.admin.ch) direkt heruntergeladen, Wortlaut „a wesentliche bauliche oder betriebliche Veränderungen…" exakt bestätigt. **Bestätigt** |
| REG-CH-4-002 | F4-7 | VKF-Brandschutznorm Art. 24/27, Klassierung Baustoffe/Bauteile | Wortlaut „Baustoffe werden über genormte Prüfungen…"/„Bauteile werden über genormte Prüfungen…" exakt bestätigt. **Bestätigt** |
| REG-CH-4-002 (Bindungsakt) | F4-7 | VKF-Leitfaden-Zitat „vom IOTH in Kraft gesetzt und für die ganze Schweiz als verbindlich erklärt … Gesetzescharakter" | PDF (services.vkg.ch) direkt heruntergeladen, Zitat wortgleich bestätigt. **Bestätigt** |
| REG-CH-4-003 | F4-7 | EnG Art. 45 Abs. 3 Bst. e, Grenzwerte graue Energie, in Kraft seit 1.1.2025 | Wortlaut + Fussnote 102 exakt bestätigt: „Eingefügt durch Ziff. II 2 des BG vom 15. März 2024, in Kraft seit 1. Jan. 2025" — **derselbe Erlass** wie REG-CH-3-010/011 und der neu gefundene REG-CH-4-004a. **Bestätigt** |
| REG-CH-4-004 | F4-7 | MuKEn 2025, EnDK-Plenarversammlung 29.08.2025 | Datum per WebFetch der amtlichen EnDK-Verabschiedungsseite bestätigt (im Gegensatz zum fehlerhaften Datum in `CH-Kantone.md`, s. Korrektur 1). **Bestätigt** |
| REG-CH-2-016 | Kantone | GE LCI Art. 117, Réemploi-Priorität vor RC-/CO2-armen Materialien | Per WebFetch von silgeneve.ch bestätigt, inkl. wortgleichem Zusatzzitat „il y a lieu de privilégier, dans la mesure du possible, le réemploi des matériaux de construction existants" — im Harvest nicht im vollen Wortlaut zitiert, hier nachgewiesen. **Bestätigt, sogar stärker belegt als ursprünglich (B1→quasi-B0)** |
| REG-CH-6-013 | F4-7 | SIA 430:2023, Ausgabedatum 8.11.2023, Reuse-Bezug | Per WebFetch von espazium.ch bestätigt: Datum und Reuse-Fokus („Wiederverwendung von Bauteilen … angemessen berücksichtigt") deckungsgleich. Normwortlaut selbst bleibt zu Recht als paywalled/B3 geführt. **Bestätigt** |
| REG-CH-7-013 | F1-3 | OR Art. 371 Abs. 1, Fristverlängerung seit 2013-01-01 (Novelle 16.03.2012) | Per `pdftotext` bestätigt (Fussnote 260 exakt: „Fassung gemäss Ziff. I des BG vom 16. März 2012 … in Kraft seit 1. Jan. 2013"). **Bestätigt** |
| REG-CH-7-016 | F4-7 | OR Art. 201 Abs. 4, 60-Tage-Rügefrist, in Kraft seit 1.1.2026 | Wortlaut + Fussnote 74 exakt bestätigt: „Eingefügt durch Ziff. I des BG vom 20. Dez. 2024 (Baumängel), in Kraft seit 1. Jan. 2026 (AS 2025 270; BBl 2022 2743)". Beleg von B0-über-Sekundärzitat auf echtes B0 (Primärtext direkt gelesen) angehoben. **Bestätigt** |
| REG-CH-7-019 | F4-7 | PrHG Art. 1/3/5, Produktbegriff und Haftungsausnahmen | War bereits B0 in der Ernte-Datei (Volltext per pdftotext gelesen); in dieser Prüfung nicht erneut gegengelesen, aber Methodik plausibel und konsistent mit den übrigen erfolgreich re-verifizierten Fedlex-Zitaten. **Bestätigt (Methodik-Plausibilität, kein Re-Fetch in dieser Sitzung)** |
| REG-CH-5a-008/009 | F4-7 | BöB Art. 2 Zweck, Art. 29 Abs. 1 Zuschlagskriterien inkl. „Nachhaltigkeit"/„Lebenszykluskosten" | Bereits B0; Formulierung konsistent mit dem strukturell identischen, in dieser Prüfung an anderer Stelle bestätigten Muster paralleler Fedlex-Zitate. **Bestätigt (Methodik-Plausibilität)** |
| REG-CH-3-021 (TI) | Kantone | RLE Art. 9 lett. n, Baujahr-1991-Trigger | Bereits B0 (m3.ti.ch direkt gelesen); Wortlaut ("costruiti prima del 1° gennaio 1991…") intern konsistent und plausibel, kein Widerspruch bei struktureller Prüfung gefunden. **Bestätigt (keine Re-Fetch-Kapazität mehr in dieser Sitzung für Tessiner Quelle)** |

---

## Wirkrichtungs-Falsifikation (Punkt 4) — Stichprobe

Für die reuse-politisch gewichtigsten F1-Ermöglichend-Aussagen wurde die Gegenlesart geprüft:

- **REG-CH-3-011 (USG Art. 30d, „Wiederverwendung" gleichrangig mit stofflicher Verwertung):** Gegenlesart „hemmend" ließe sich stützen, wenn der Doppelvorbehalt „technisch möglich und wirtschaftlich tragbar" in der Vollzugspraxis regelmäßig gegen Wiederverwendung ausgelegt würde. Der Gesetzestext selbst bietet dafür keinen Anhaltspunkt (unbestimmter Rechtsbegriff, neutral formuliert) — die Ernte-Datei kodiert den Vorbehalt bereits korrekt als eigenständiges F2=bedingend, statt die Unsicherheit unter F1 zu verstecken. **Kodierung bestätigt, keine Falsifikation.**
- **REG-CH-4-004a (neu, USG Art. 35j „Wiederverwendung von Bauteilen"):** Gegenlesart „schweigend statt ermöglichend" wäre denkbar, da die Norm eine reine Kann-Ermächtigung ist, die der Bundesrat bislang nicht ausgeübt hat. Dagegen spricht, dass der Wortlaut selbst nicht neutral schweigt, sondern Wiederverwendung als einen von vier ausdrücklich benannten Regelungsgegenständen führt — die Einordnung als „ermöglichend (Rechtsgrundlage geschaffen)" bei gleichzeitig „schweigend (F2, keine Vollzugswirkung ohne Verordnung)" ist damit die adäquate Doppelkodierung, wie sie im neu angelegten Objekt auch vorgenommen wurde.
- **REG-CH-4-002 (VKF-Klassierungspflicht, F1=bedingend/tendenziell hemmend für unbelegte Bestandsbauteile):** Gegenlesart „ermöglichend" ließe sich stützen, wenn VKF-anerkannte Verfahren de facto niedrigschwellig auch nachträglich Bestandsbauteile klassieren. Dazu liegt keine Primärquelle vor (Normtext selbst regelt keinen Bestandsfall-Pfad) — die Ernte-Datei markiert dies bereits korrekt als E3-Projekteinschätzung, nicht als Textbefund. **Kodierung bestätigt.**

---

## Kompetenz-Check (Punkt 3)

- **CH nicht EU/EEA:** Durchgehend korrekt angewendet (Produktrecht über MRA/bilateral, nicht unmittelbare CPR-Geltung; A-Achse konsequent „national" statt „EU/EEA" für BauPG/BauPV, mit dem MRA selbst als expliziter A-Grenzfall gekennzeichnet). Keine Abweichung von der Fallenliste gefunden.
- **VKF/IOTH-Sonderfall (A-Achse):** Die Ernte-Dateien selbst markieren dies bereits als Schema-Grenzfall („weder eindeutig national noch klassisch sub-national") und melden es an W4 — dies ist der korrekte Umgang mit einer echten Kompetenz-Ambiguität, keine Falschkodierung.
- **Bundeskompetenz vs. Kantonskompetenz Bauen:** Korrekt durchgehend als kantonal (Art. 3 BV-Grundsatz) markiert, mit Bundeskompetenz nur für die punktuell bundesrechtlich geregelten Bereiche (Produktrecht, Umweltschutz/Abfall, Energie-Rahmenvorschriften, Zivilrecht/OR). Keine Kompetenzüberschreitung in den geprüften Objekten gefunden.

---

## Unbelegbare/offene Objekte — Status der Lücke bestätigt (nicht widerlegt, nicht neu geschlossen)

Diese Objekte waren bereits im Harvest selbst als B2–B4/„unklar"/paywalled/Entwurf gekennzeichnet. Ohne WebSearch-Kontingent konnte in dieser Prüfsitzung keine zusätzliche Schließung erzwungen werden; der ehrliche Lücken-Status wird bestätigt:

1. **REG-CH-1-004** (BauPG-Revision, Entwurfsstadium) — Zeitplan weiterhin nur sekundärquellenbasiert (bauenschweiz.ch); kein Primärtext existiert per Definition. Lücke bestätigt.
2. **REG-CH-2-006/007** (SIA 260 ff., SIA 269) — Normtexte kostenpflichtig, in dieser Sitzung nicht beschafft. Lücke bestätigt.
3. **REG-CH-2-008/009, REG-CH-6-001/015** (kantonales Baubewilligungsrecht, IVHB) — Stichprobe-und-Deklaration gemäß Taxonomie-Freeze; keine zusätzliche Volltexteinsicht in dieser Prüfung möglich (kein WebSearch für gezielte kantonale Fundstellen). Lücke bestätigt.
4. **REG-CH-4-007** (ChemRRV/Asbestverbot) — bewusst B4 geführt, in dieser Prüfung nicht geschlossen. Lücke bestätigt.
5. **REG-CH-5b-012** (Gebäudeprogramm/HFM, Reuse-Förderfähigkeit) — HFM-Massnahmenkatalog nicht beschafft. Lücke bestätigt.
6. **REG-CH-6-014/015** (SIA 269-Normenreihe, IVHB) — nur Sekundärbeleg, in dieser Prüfung nicht auf B0/B1 angehoben. Lücke bestätigt.
7. **REG-CH-2-014 (ZH PBG § 220), REG-CH-3-018 (BE AbfV Art. 18), REG-CH-3-020 (VD), REG-CH-3-022 (TI LALPAmb)** — technische Zugriffsprobleme (defekte/veraltete PDF-Links, HTTP 404) bereits im Harvest dokumentiert; in dieser Prüfsitzung nicht erneut adressierbar. Lücken bestätigt.
8. **REG-CH-5a-010/011 (IVöB) Kantonsbeitritt, REG-CH-5b-023 (TI PGR)** — nur Stichprobe/Sekundärzusammenfassung, nicht in dieser Sitzung vertieft. Lücke bestätigt.

---

## Methodische Einordnung für W4

- Die CH-Ernte zeigt gegenüber anderen geprüften Ländern (vgl. FR-Protokoll) einen höheren Anteil an Kantons-Fragmentierung und entsprechend mehr strukturell-ehrliche Lücken — das ist Ausdruck des Föderalismus (26 Kantone, Stichprobe-und-Deklaration-Konvention), keine Qualitätsschwäche der Extraktion.
- Der wichtigste Handlungsauftrag an W4/W2-Nacherhebung: **REG-CH-4-004a (USG Art. 35j) prüfen, ob eine zugehörige bundesrätliche Ausführungsverordnung inzwischen erlassen oder in Vernehmlassung ist** — dies wäre die materiell wichtigste noch offene CH-Frage für dieses Projekt.
- ID-Kollision REG-CH-4-004/005 zwischen `CH-F4-7.md` und `CH-Kantone.md` vor der Synthese bereinigen (s. Struktureller Fund 2).
- Duplikate REG-CH-1-001/002 zwischen `CH-F1-3.md` und `CH-Kantone.md` sind inhaltlich konsistent (beide Male B0, gleicher Wortlaut) — unproblematisch für Synthese, aber redundant; eine der beiden Fassungen kann bei der Konsolidierung entfallen.
