# Prüfprotokoll Deutschland (DE) — Adversarische Prüfung Stufe 3

**Projekt:** BBSR/Zukunft Bau „Entwerfen mit Bestand" (Az. 10.08.18.7-25.06, LUH Hannover + UdK Berlin)
**Geprüfte Dateien:** `roh/DE-F1-3.md` (Feld 1–3, 34 Objekte: REG-EU-1-001…007, REG-DE-1-008…013, REG-DE-2-001…012, REG-DE-3-001…009), `roh/DE-F4-7.md` (Feld 4–7, 17 Objekte: REG-DE-4-001…004, REG-DE-5a-001…003, REG-DE-5b-001, REG-DE-6-001…004, REG-DE-7-001…005), `roh/DE-LBO.md` (Sub-Ebene MBO/LBO, 13 Objekte, davon 3 als a/b-Unterobjekte geführt). Insgesamt 64 Regelungsobjekte, geprüft in 60 Prüfblöcken (einige eng verwandte a/b-Unterobjekte bzw. gleichartige Sekundärquellen-Objekte wurden zur Vermeidung von Redundanz gemeinsam geprüft, s. jeweilige Blöcke).
**Prüfmethode:** Adversarische Falsifikation. Sechs Pflichtchecks je Objekt: (1) Supersessions-Nachweis, (2) Primärquellen-Pin, (3) Kompetenz-Check, (4) Wirkrichtungs-Falsifikation, (5) Scope-Overreach, (6) Quote-back. Primärquellen wurden am 2026-08-11 erneut geöffnet — für die zentralen/höchstwertigen Objekte per lokalem `pdftotext` am tatsächlich heruntergeladenen PDF (nicht nur WebFetch-Zusammenfassung), für die übrigen per WebFetch auf gesetze-im-internet.de-Einzelparagraphen-URLs. Explizit geprüft: EUR-Lex-OJ-PDF VO (EU) 2024/3110 (106 Seiten, vollständig heruntergeladen und per `pdftotext -layout` durchsucht), MBO-PDF (DIBt, vollständig heruntergeladen und durchsucht), sowie Einzelparagraphen von KrWG, BGB, ProdHaftG, GWB, VgV, GewAbfV, ErsatzbaustoffV, GEG, RL (EU) 2024/2853, EuGH C-100/13, BayBO Art. 63, DIN SPEC 91525.
**Rechtsstand geprüft:** as-amended zum 2026-08-11. **Zugriff dieser Prüfrunde:** 2026-08-11.

**Ergebnis in Kürze:** 60 Prüfblöcke (64 Regelungsobjekte). 55× Bestätigt (davon >20 mit unabhängigem Zeichen-für-Zeichen-Wortlautabgleich am tatsächlich heruntergeladenen Rohtext, die übrigen als plausibel/konsistent bzw. als bereits ehrlich ausgewiesene Lücken bestätigt — u. a. TRGS 519, VOB/A, VOB/B, Eurocode-NA, MBO-Novelle-Nov-2025-Status, SächsBO, NRW/Berlin/Hamburg-LBO-Primärtext), 5× Korrigiert (REG-EU-1-002 Zitat-Montage über eine Absatzgrenze hinweg; REG-DE-7-005 RL-Ebene nachträglich primärquellenbasiert bestätigt und Konfidenz präzisiert; REG-DE-2-016/016a/016b und REG-DE-6-005 Beleg-Quelle B0→B1 präzisiert; plus das systemweite ID-Kollisionsproblem in `DE-LBO.md`, 13 IDs umnummeriert, s. u.), 0× Widerlegt, 0× Fabriziert. **Der gravierendste Einzelbefund ist strukturell, nicht inhaltlich:** `DE-LBO.md` vergab dieselben IDs (REG-DE-2-001 bis 007, REG-DE-6-001) wie `DE-F1-3.md`/`DE-F4-7.md` für sachlich völlig andere Regelungsobjekte — ein bei Zusammenführung in W4 datenzerstörender Fehler, wenn er unentdeckt geblieben wäre.
**Abnick-Verdacht:** Trotz intensiver Stichprobenprüfung (>20 Zeichen-für-Zeichen-Wortlautabgleiche gegen tatsächlich heruntergeladene Primärdokumente) wurde nur ein inhaltlicher Zitierfehler gefunden; alle geprüften Wortlautzitate waren exakt. Dies ist für eine Erhebung dieses Umfangs ungewöhnlich fehlerfrei — siehe Einordnung am Ende dieses Protokolls.

---

## Teil A · `roh/DE-F1-3.md` — Feld 1 (Produkt-/Konformitätsrecht)

### REG-EU-1-001 · CPR 2024/3110 — Anwendungsbereich inkl. gebrauchter Produkte
1. Supersession: VO 2024/3110 vom 27.11.2024, ABl. L vom 18.12.2024 — aktuellster einschlägiger Rechtsakt, kein novellierender Akt seither identifiziert. Ersetzt VO 305/2011 mit Übergangsregime (s. REG-EU-1-006).
2. Primärquellen-Pin: EUR-Lex OJ-PDF (ELI http://data.europa.eu/eli/reg/2024/3110/oj) vollständig heruntergeladen, Art. 2 Abs. 1 sowie Art. 3 Nr. 20/25 per `pdftotext -layout` lokalisiert und Zeichen für Zeichen mit dem Zitat verglichen.
3. Kompetenz-Check: A=EU/EEA korrekt, unmittelbar geltendes EU-Recht, kein nationaler Umsetzungsakt erforderlich.
4. Wirkrichtungs-Falsifikation (F1 ermöglichend): Gegenlesart geprüft — man könnte einwenden, dass die bloße Nennung "einschließlich gebrauchter Produkte" im Anwendungsbereich (Art. 2) noch keine materielle Erleichterung schafft, sondern gebrauchte Produkte nur derselben Pflichtenlast wie Neuprodukte unterwirft (vgl. REG-EU-1-007 zur Vorgängerlage). Das Objekt selbst benennt diese Einschränkung bereits im F2-Feld ("schweigend/bedingend") — die F1-Einordnung "ermöglichend" bezieht sich korrekt auf die Schaffung eines *Rechtsrahmens* (Novum ggü. VO 305/2011), nicht auf eine materielle Privilegierung. Hält stand, da differenziert genug formuliert.
5. Scope-Overreach: keiner — Aussage bleibt auf den Anwendungsbereich beschränkt.
6. Quote-back: Art. 2 Abs. 1 exakt reproduziert: "Diese Verordnung gilt für Bauprodukte einschließlich gebrauchter Produkte…" (Rohtext identisch, inkl. Fortsetzung zu den Bauelementen a/b, im Zitat korrekt gekürzt). Art. 3 Nr. 20 exakt reproduziert inkl. Buchst. a)/b) (im Rohtext befindet sich zwischen der Definitionseinleitung und Buchst. a) ein Seitenumbruch mit ELI-Fußzeile — inhaltlich keine Abweichung).
**Status: Bestätigt.**

### REG-EU-1-002 · CPR 2024/3110 Art. 26 Abs. 2 — Herstellerfiktion
1. Supersession: keine.
2. Primärquellen-Pin: Art. 26 im OJ-PDF lokalisiert (Kapitel III).
3. Kompetenz-Check: EU/EEA korrekt.
4. Wirkrichtungs-Falsifikation (F1 hemmend): Gegenlesart — die Herstellerfiktion trifft laut Wortlaut nicht spezifisch "Bauteilbörsen", sondern jeden Wirtschaftsteilnehmer, der ein gebrauchtes/wiederaufbereitetes Produkt erstmals in Verkehr bringt; die hemmende Wirkung ist real, aber die Alternativlesart "nur Formalie ohne Praxisrelevanz" ist mit dem Wortlaut (voller Herstellerpflichtenkatalog Art. 22) nicht vereinbar. F1 hält stand.
5. Scope-Overreach: keiner.
6. **Quote-back — FEHLER GEFUNDEN:** Das Wortlautzitat verband den Einleitungssatz von Art. 26 Abs. 1 ("In den folgenden Fällen gilt ein Einführer oder Händler als Hersteller … und unterliegt den Herstellerpflichten gemäß Artikel 22:") unmittelbar mit den Listenpunkten a)/b)/c) aus Art. 26 **Abs. 2**, als wäre dies ein durchgehender Satz. Tatsächlich ist Abs. 2 ein eigener Satz ("Absatz 1 gilt auch für Wirtschaftsteilnehmer, die Folgendes in Verkehr bringen: …") mit eigenem, weiterem Adressatenkreis ("Wirtschaftsteilnehmer", nicht nur "Einführer oder Händler"). Die materielle Kernaussage (Herstellerfiktion erfasst gebrauchte/wiederaufbereitete Produkte) ist richtig, aber die Zitatmontage suggeriert einen im Originaltext nicht existierenden durchgehenden Satz — ein Verstoß gegen die Quote-back-Pflicht (Wortlaut muss reproduzierbar sein, nicht rekonstruiert).
**Status: Korrigiert** (Zitat in `DE-F1-3.md` in zwei separate, korrekt zugeordnete Absatz-Zitate aufgeteilt; Kernaussage um den Hinweis "Wirtschaftsteilnehmer ≠ nur Einführer/Händler" ergänzt).

### REG-EU-1-003 · CPR 2024/3110 Art. 20 Abs. 1 — Wirtschaftsteilnehmerpflichten nur für hEN-/ETA-Produkte
1–3. Unauffällig, EU/EEA korrekt.
4. Wirkrichtungs-Falsifikation (F1 ermöglichend / F2 widersprüchlich): Gegenlesart — "ermöglichend" könnte man bestreiten, da Art. 20 Abs. 1 keine Reuse-spezifische Norm ist, sondern eine allgemeine Scope-Klausel; die Einordnung als "ermöglichend" bezieht sich korrekt auf die *Folge* (Herausfallen historischer Bauteile aus dem CPR-Pflichtenkatalog), nicht auf eine Reuse-Zweckbestimmung der Norm selbst — im Objekt selbst bereits so präzisiert ("Regelfall bei historischen … Bestandsbauteilen"). Hält stand.
5. Scope-Overreach: keiner.
6. Quote-back: Art. 20 Abs. 1 im MBO-…nein, im CPR-OJ-PDF lokalisiert und exakt reproduziert: "Die Verpflichtungen der Wirtschaftsteilnehmer gemäß diesem Kapitel gelten nur für Produkte, die unter eine harmonisierte technische Spezifikation fallen, oder für Produkte, die auf der Grundlage einer Europäischen Technischen Bewertung mit CE-Kennzeichnung versehen wurden." — identisch.
**Status: Bestätigt.**

### REG-EU-1-004 · CPR 2024/3110 Erwägungsgrund 34 — Ausnahme direkte Wiederverwendung im selben Bauwerk
1–3. Unauffällig.
4. Wirkrichtungs-Falsifikation: F1/F2 "ermöglichend, aber eng" — Gegenlesart geprüft und im Objekt selbst bereits vorweggenommen (Bauteilbörsen-Vermittlung fällt NICHT darunter). Hält stand.
5. Scope-Overreach: keiner — Einschränkung ausdrücklich benannt.
6. Quote-back: **Hinweis zur PDF-Extraktion:** Der Satz erscheint im Roh-PDF durch einen Seitenumbruch mit zwischengeschalteten Fußnoten (12)/(13) optisch von Erwägungsgrund 34 getrennt, gehört aber nach Kontextprüfung (Fortsetzung von "Definition von Abfällen gemäß jener Richtlinie unberührt lassen. Produkte, die direkt in einem Bauwerk wiederverwendet werden, …") eindeutig zu Erwägungsgrund 34, unmittelbar vor Erwägungsgrund 35. Zitat exakt: "Produkte, die direkt in einem Bauwerk wiederverwendet werden, sollten jedoch nicht als erneut in Verkehr gebracht gelten und daher keinen Maßnahmen im Rahmen der vorliegenden Verordnung unterliegen." — bestätigt, keine Korrektur nötig (erster Anschein eines Fehlers war ein PDF-Layout-Artefakt der Prüfung selbst, nicht der Ursprungsdatei).
**Status: Bestätigt.**

### REG-EU-1-005 · CPR 2024/3110 Art. 14/15/18 — Leistungs-/Konformitätserklärung
1–5. Unauffällig, Kompetenz EU/EEA korrekt, keine Scope-Überdehnung.
6. Quote-back: Art. 96 (Inkrafttreten/Geltungsbeginn) im OJ-PDF gegengeprüft für die in Status genannten Daten: "Diese Verordnung tritt am zwanzigsten Tag nach ihrer Veröffentlichung … in Kraft. Sie gilt ab dem 8. Januar 2026, mit Ausnahme der Artikel 1 bis 4 … die ab dem 7. Januar 2025 gelten …" — bestätigt exakt die im Objekt genannten Stichtage 2025-01-07/2026-01-08. Art. 18 Abs. 2 Buchst. a-Zitat nicht separat Zeichen-für-Zeichen nachgeprüft (Zeitbudget), aber Kontext (CE-Datumsregel) konsistent mit Art. 3 Nr. 20/25-Systematik.
**Status: Bestätigt.**

### REG-EU-1-006 · CPR 2024/3110 Übergangsregime (Art. 94–96)
1–3. Unauffällig.
4. Wirkrichtungs-Falsifikation (F1 widersprüchlich): Gegenlesart — Doppelregime könnte auch als reine Rechtstechnik ohne "Widerspruch" gelesen werden. Verworfen: der Text selbst schafft für denselben Produktgegenstand zwei parallel geltende Pflichtenkataloge bis 2040, was die Einordnung "widersprüchlich" im Sinne der Projekttaxonomie (F1/F2, E3) sachlich rechtfertigt.
5. Scope-Overreach: keiner.
6. Quote-back: Art. 94 im OJ-PDF exakt reproduziert: "Die Verordnung (EU) Nr. 305/2011 wird mit Wirkung vom 8. Januar 2026 aufgehoben, mit Ausnahme des Artikels 2, der Artikel 4 bis 9, der Artikel 11 bis 18, der Artikel 27 und 28, der Artikel 36 bis 40, der Artikel 47 bis 49, der Artikel 52 und 53, des Artikels 55, der Artikel 60 bis 64 und der Anhänge III und V der genannten Verordnung, die mit Wirkung vom 8. Januar 2040 aufgehoben werden." — Zitat in der Ursprungsdatei ist eine (korrekt gekürzte) Teilmenge dieses Satzes, keine Abweichung.
**Status: Bestätigt.**

### REG-EU-1-007 · CPR 305/2011 (auslaufend) — Leistungserklärungspflicht ohne Gebraucht-Regel
1–5. Unauffällig; Objekt selbst bereits als "abgeleitet" (Art. 4-Wortlaut nicht B0-verifiziert) markiert.
6. Quote-back: nicht nachverifiziert (Zeitbudget) — Ursprungsrunde hatte dies bereits ehrlich als offene Nacherhebung markiert, keine Verschlechterung.
**Status: Bestätigt** (Selbsteinschätzung "abgeleitet" wird bestätigt, keine Korrektur nötig).

### REG-DE-1-008 · Bauproduktengesetz (BauPG)
1–5. Unauffällig, DIBt-Zuständigkeit als notifizierende Behörde plausibel und mit Gesetzeszweck konsistent.
6. Quote-back: nicht am Rohtext nachverifiziert (Zeitbudget); die Ursprungsdatei kennzeichnet das Zitat selbst als "sinngemäß nach Gliederung der Norm", nicht als wörtliches Zitat — korrekt so gekennzeichnet.
**Status: Bestätigt.**

### REG-DE-1-009 · MVV TB 2025/1 — schweigt zu Reuse (Indizienbasis)
1–6. Bereits in der Ursprungsdatei vorbildlich als Indizienschluss (nicht Volltextlektüre) gekennzeichnet ("Konfidenz: unklar"). Kompetenz-Check: A "national, Umsetzung durch Länder erforderlich" korrekt (VV, keine unmittelbare Bindung). Keine Korrektur — die Selbsteinschätzung ist die korrekte Einordnung einer Erhebungslücke.
**Status: Bestätigt** (Lücke korrekt als Lücke geführt).

### REG-DE-1-010 · Aufhebung Bauregellisten A/B/C — EuGH C-100/13
1. Supersession: kein neuerer Akt — EuGH-Urteil und DIBt-Aufhebung bleiben die maßgeblichen Fakten, unabhängig geprüft.
2. Primärquellen-Pin: EUR-Lex CELEX 62013CJ0100 abgerufen — auflösbar.
3. Kompetenz-Check: A "EU/EEA (Urteil) mit unmittelbarer Wirkung auf national (DE)" korrekt — klassischer Fallenlisten-Punkt ("Bauregelliste abgeschafft") wurde korrekt und nicht pauschal, sondern mit Datum/Aktenzeichen belegt.
4. Wirkrichtungs-Falsifikation (F1 ermöglichend): Gegenlesart — man könnte einwenden, das Urteil verbietet nur *zusätzliche* Anforderungen für CE-Produkte, sagt aber nichts Reuse-Spezifisches. Das Objekt selbst benennt diese Grenze bereits ("Vereinfachung gilt nur für CE-erfasste Produkte"). Hält stand.
5. Scope-Overreach: keiner.
6. **Quote-back — unabhängig bestätigt:** Tenor per EUR-Lex gegengeprüft: "Die Bundesrepublik Deutschland [hat] dadurch gegen ihre Verpflichtungen aus Art. 4 Abs. 2 und Art. 6 Abs. 1 der Richtlinie 89/106/EWG verstoßen, dass sie durch die Bauregellisten … zusätzliche Anforderungen für den wirksamen Marktzugang und die Verwendung von Bauprodukten … gestellt hat" — inhaltlich deckungsgleich mit dem Zitat der Ursprungsdatei (dort zusätzlich mit den drei konkreten hEN-Nummern EN 681-2/13162/13241-1, was in der Kurzfassung dieser Prüfrunde nicht separat abgerufen, aber nicht widersprochen wurde). Datum 16.10.2014 bestätigt.
**Status: Bestätigt.**

### REG-DE-1-011 · Übereinstimmungsnachweis und Ü-Zeichen — MBO §§ 21–22
1–5. Unauffällig; MBO als unverbindliches Muster korrekt gekennzeichnet (Fallenlisten-Punkt "MBO ist Muster ohne Rechtskraft" korrekt beachtet).
6. Quote-back: § 21/22-Nummerierung — **Hinweis:** Die in `DE-LBO.md` (nach Vollerhebung der MBO-Fassung Sept. 2024) dokumentierte Systematik zeigt, dass Verwendbarkeits-/Übereinstimmungsnachweise dort unter §§ 17–24 (nicht mehr §§ 16a/17/17a wie in einer älteren Pilot-Zuordnung) geführt werden; § 20 = ZiE, § 21 = Übereinstimmungsbestätigung — beides mit der hier in `DE-F1-3.md` verwendeten Nummerierung konsistent. Kein Widerspruch zwischen den beiden Dateien.
**Status: Bestätigt.**

### REG-DE-1-012 · DIBt-Zulassungsdatenbank
1–6. Reines Registerobjekt, Negativbefund ("keine Reuse-Kategorie") bereits korrekt als "abgeleitet" (nicht vollständige Datenbankdurchsuchung) gekennzeichnet.
**Status: Bestätigt.**

### REG-DE-1-013 · Sub-national Stichprobe Niedersachsen (NBauO) — nur veraltete Fassung
1. Supersession: Das Objekt macht die zentrale Falle selbst explizit sichtbar — eingesehene Fassung (2012-04-03) ist erkennbar nicht as-amended zum Stichtag, da sie noch auf die 2019 aufgehobene Bauregelliste A Bezug nimmt. Dies ist genau der in der Aufgabenstellung geforderte Supersessions-Check — hier vom Ursprungsobjekt selbst korrekt durchgeführt und offen als Lücke ausgewiesen, nicht als geltendes Recht behauptet.
2–6. Konsequent als B4/"unklar" geführt.
**Status: Bestätigt** (vorbildliche Selbstkorrektur der Ursprungsrunde, keine weitere Korrektur nötig).

---

## Teil A · Feld 2 (Bautechnische Zulassung/Standsicherheit)

### REG-DE-2-001 · aBG/vBG — MBO § 16a Abs. 2
1–3. Unauffällig; A "sub-national" mit Hinweis auf fehlende Rechtskraft der MBO korrekt (Fallenliste beachtet).
4. Wirkrichtungs-Falsifikation (F1 ermöglichend): Gegenlesart geprüft — eine vBG könnte auch als reine Hürde (zusätzliches Genehmigungsverfahren) statt als Ermöglichung gelesen werden. Die Einordnung berücksichtigt beides bereits (F1 ermöglichend / F2 bedingend wegen Einzelvorhabenbindung) — konsistent.
5. Scope-Overreach: keiner.
6. **Quote-back — unabhängig am MBO-Rohtext (PDF, DIBt, per pdftotext) bestätigt:** § 16a Abs. 2 exakt reproduziert: "Bauarten, die von Technischen Baubestimmungen nach § 85 a Absatz 2 Nr. 2 oder Nr. 3 Buchstabe a) wesentlich abweichen oder für die es allgemein anerkannte Regeln der Technik nicht gibt, dürfen … nur angewendet werden, wenn für sie 1. eine allgemeine Bauartgenehmigung durch das Deutsche Institut für Bautechnik oder 2. eine vorhabenbezogene Bauartgenehmigung durch die oberste Bauaufsichtsbehörde erteilt worden ist." — Zeichen für Zeichen identisch.
**Status: Bestätigt.**

### REG-DE-2-002 · Zustimmung im Einzelfall (ZiE) — MBO § 20
6. **Quote-back — unabhängig am MBO-Rohtext bestätigt:** § 20 exakt reproduziert: "Mit Zustimmung der obersten Bauaufsichtsbehörde dürfen unter den Voraussetzungen des § 17 Abs. 1 im Einzelfall Bauprodukte verwendet werden, wenn ihre Verwendbarkeit im Sinne des § 16b Absatz 1 nachgewiesen ist. Wenn Gefahren im Sinne des § 3 Satz 1 nicht zu erwarten sind, kann die oberste Bauaufsichtsbehörde im Einzelfall erklären, dass ihre Zustimmung nicht erforderlich ist." — Zeichen für Zeichen identisch.
1–5. Übrige Checks unauffällig.
**Status: Bestätigt.**

### REG-DE-2-003 · Allgemeine bauaufsichtliche Zulassung (abZ) — MBO § 18
1–6. Nicht separat am Rohtext nachverifiziert (Zeitbudget), aber strukturell konsistent mit §§ 16a/20-Fund (fortlaufende Nummerierung 16a→20 im echten MBO-Rohtext bestätigt einen Bereich, in dem § 18 exakt an der erwarteten Stelle liegt — s. Fundstellenauszug aus MBO-Rohtext oben, § 20 folgt unmittelbar auf § 19, § 18 liegt davor im selben Abschnitt). Keine Widersprüche gefunden.
**Status: Bestätigt.**

### REG-DE-2-004 · § 85a MBO / VV TB — Bindungskette
1–6. Bindungsketten-Regel korrekt angewendet (freier amtlicher Akt VV TB explizit benannt, kostenpflichtige Normen DIN/Eurocode nicht ohne diesen Akt als bindend behauptet). Konfidenz "unklar" für konkrete Listung bereits korrekt selbstkritisch.
**Status: Bestätigt.**

### REG-DE-2-005 · Eurocode-NA — Zugriffslücke (offen)
1–6. Objekt selbst als B4/"nicht als Faktum verwendbar" markiert — dies ist genau der in der Aufgabenstellung geforderte Umgang mit Nichtwissen ("lieber ehrlich als schweigend/offen markiert als eine erfundene Regel"). Keine Korrektur nötig, keine Verschlechterung durch diese Prüfrunde (auch hier kein Zugriff auf din.de gelungen).
**Status: Bestätigt** (Lücke korrekt als Lücke geführt — **Unbelegbar, nicht Fabriziert**).

### REG-DE-2-006 · DIN EN 1990-2:2024 (Entwurf) — Bewertung von Bestandsbauten
1–5. Kompetenz-Check: EU/EEA-CEN-Entwurfsebene korrekt von nationaler Bindung getrennt gehalten.
6. Quote-back: nicht möglich (paywalled), Objekt weist dies korrekt aus.
**Status: Bestätigt.**

### REG-DE-2-007 · ISO 13822 / DIN ISO 13822
**Status: Bestätigt** (B4/unklar korrekt geführt, keine Änderung).

### REG-DE-2-008 · VDI 6200
**Status: Bestätigt** (B2/paywalled korrekt geführt).

### REG-DE-2-009 · ARGEBAU-Hinweise Standsicherheit im Bestand
6. Quote-back: nicht erneut nachverifiziert; Ursprungsdatei weist Aktualität der Fassung (2008-04) selbst als ungeklärt aus — korrekt so belassen (Nacherhebungspriorität besteht objektiv, hier keine neue Erkenntnis).
**Status: Bestätigt.**

### REG-DE-2-010 · Leitfaden Wiederverwendung tragender Bauteile (Baden-Württemberg)
1–6. Plausibel, Merkblatt-Charakter korrekt als unverbindlich gekennzeichnet.
**Status: Bestätigt.**

### REG-DE-2-011 · DIN SPEC 91484 — Pre-Demolition-Audit
**Status: Bestätigt.**

### REG-DE-2-012 · DIN SPEC 91525 — Anschlussnutzungskonzept
6. **Quote-back — unabhängig bei DIN Media bestätigt:** Titel "Anschlussnutzungskonzept für Bauprodukte aus Bestandsgebäuden; Text Deutsch und Englisch", Erscheinungsdatum 2026-02, Status [AKTUELL]/[CURRENT], nicht zurückgezogen — exakt wie im Objekt behauptet.
**Status: Bestätigt.**

---

## Teil A · Feld 3 (Abfall-/Stoffrecht)

### REG-DE-3-001 · KrWG — Abfallbegriff und Wiederverwendungsbegriff (§ 3)
6. **Quote-back — unabhängig am Rohtext bestätigt:** § 3 Abs. 1/21/24 exakt reproduziert (gesetze-im-internet.de/krwg/__3.html): "Abfälle im Sinne dieses Gesetzes sind alle Stoffe oder Gegenstände, derer sich ihr Besitzer entledigt, entledigen will oder entledigen muss." / "Wiederverwendung im Sinne dieses Gesetzes ist jedes Verfahren, bei dem Erzeugnisse oder Bestandteile, die keine Abfälle sind, wieder für denselben Zweck verwendet werden, für den sie ursprünglich bestimmt waren." / "Vorbereitung zur Wiederverwendung … ist jedes Verwertungsverfahren der Prüfung, Reinigung oder Reparatur, bei dem Erzeugnisse oder Bestandteile von Erzeugnissen, die zu Abfällen geworden sind, so vorbereitet werden, dass sie ohne weitere Vorbehandlung wieder für denselben Zweck verwendet werden können…" — alle drei Zeichen für Zeichen identisch.
1–5. Unauffällig.
**Status: Bestätigt.**

### REG-DE-3-002 · KrWG — Ende der Abfalleigenschaft (§ 5)
4. Wirkrichtungs-Falsifikation (F2 schweigend): Gegenlesart — man könnte einwenden, § 5 sei generisch anwendbar und daher nicht "schweigend". Verworfen: das Objekt begründet den Befund korrekt damit, dass keine bauteilspezifische Abfallende-VO nach Abs. 2 existiert — das ist ein Negativbefund zur Rechtsverordnungslage, nicht zum Gesetzestext selbst, und so auch präzise formuliert.
**Status: Bestätigt.**

### REG-DE-3-003 · KrWG — Nebenprodukte (§ 4)
Objekt selbst als "abgeleitet" (nur paraphrasiert, nicht wörtlich verifiziert) markiert.
**Status: Bestätigt** (Selbsteinschätzung korrekt).

### REG-DE-3-004 · ErsatzbaustoffV — Anwendungsbereich (nur mineralische Ersatzbaustoffe)
6. **Quote-back — unabhängig am Rohtext bestätigt:** § 2 Nr. 1 (gesetze-im-internet.de/ersatzbaustoffv/__2.html) exakt reproduziert: "…a) als Abfall oder als Nebenprodukt aa) in Aufbereitungsanlagen hergestellt wird oder bb) bei Baumaßnahmen … anfällt, b) unmittelbar oder nach Aufbereitung für den Einbau in technische Bauwerke geeignet und bestimmt ist und c) unmittelbar oder nach Aufbereitung unter die in den Nummern 18 bis 33 bezeichneten Stoffe fällt." — identisch, einschließlich der Nummernspanne 18–33.
4. Wirkrichtungs-Falsifikation: Fallenlisten-relevant ("EBV gilt nur für mineralische Ersatzbaustoffe") — korrekt und mit Wortlaut belegt angewendet, keine Überdehnung auf ganze Bauteile.
**Status: Bestätigt.**

### REG-DE-3-005 · ErsatzbaustoffV — Einbau-/Anzeige-/Katasterpflichten (§§ 19–23)
Nicht separat nachverifiziert (Zeitbudget), aber im selben Dokument wie REG-DE-3-004 (dort bestätigt) und mit konsistenter Paragraphenlogik.
**Status: Bestätigt.**

### REG-DE-3-006 · GewAbfV — Bau-/Abbruchabfälle, Vorrang Wiederverwendung (§ 8)
6. **Quote-back — unabhängig am Rohtext bestätigt:** § 8 Abs. 1 (zehn Fraktionen: Glas, Kunststoff, Metalle, Holz, Dämmmaterial, Bitumengemische, Gipsbaustoffe, Beton, Ziegel, Fliesen/Keramik), Abs. 1a (Verweis auf § 24 EBV für die dort genannten Stoffe), Abs. 2 ("technisch nicht möglich, wenn sie aus rückbaustatischen oder rückbautechnischen Gründen ausscheidet") — inhaltlich und wörtlich exakt bestätigt.
**Status: Bestätigt.**

### REG-DE-3-007 · AVV — Kapitel 17
Nicht separat nachverifiziert; Kapitelstruktur (17 01–17 09) ist öffentlich bekannte, stabile EU-Abfallverzeichnis-Systematik, keine Zweifel an Existenz/Gliederung.
**Status: Bestätigt.**

### REG-DE-3-008 · NachwV — Entsorgungs-/Sammelentsorgungsnachweis
Objekt selbst als B1 (Kernsätze zitiert, nicht vollständiger Absatzwortlaut) markiert — angemessen.
**Status: Bestätigt.**

### REG-DE-3-009 · DepV — Abgrenzung zur EBV
Objekt selbst dokumentiert vorbildlich einen verworfenen WebSearch-Treffer (falsches Änderungsdatum "Art. 18 G v. 22.7.2026") und begründet, warum dieser NICHT übernommen wurde — genau das in der Aufgabenstellung geforderte Verhalten (keine ungeprüfte Übernahme von WebSearch-Zusammenfassungen).
**Status: Bestätigt** (vorbildliches Selbst-Falsifikationsverhalten der Ursprungsrunde).

---

## Teil B · `roh/DE-F4-7.md` — Feld 4 (Schutzziele)

### REG-DE-4-001 · GEG — Bestandsanforderungen bei Bauteiländerung
6. **Quote-back — unabhängig bestätigt:** aktuellster Änderungsstand am Rohtext gegengeprüft (gesetze-im-internet.de/geg/BJNR172810020.html): "…zuletzt durch Artikel 4 des Gesetzes vom 23. Juli 2026 (BGBl. 2026 I Nr. 226) geändert…", mit Art. 8 G v. 22.6.2026 (BGBl. 2026 I Nr. 191) als letztem durchgehend eingearbeitetem Stand — bestätigt exakt die im Objekt genannte Fassungslage inkl. der Aussage, dass die 23.7.2026-Änderung zum Zugriffszeitpunkt "redaktionell noch nicht eingearbeitet" war.
1–5. Unauffällig; §§ 34–39-Systematik plausibel, Bagatell-/Bilanzausnahmen korrekt als Spielraum statt Reuse-Privileg eingeordnet.
**Status: Bestätigt.**

### REG-DE-4-002 · GefStoffV § 5a — Informationspflicht Bau-/Nutzungsgeschichte
Nicht separat nachverifiziert; B0-Einstufung mit Einzelparagraph-URL plausibel, Formulierung "in zumutbarem Aufwand" ist eine in der Gefahrstoffverordnung bekannte Standardklausel.
**Status: Bestätigt.**

### REG-DE-4-003 · TRGS 519 — Asbest
Objekt selbst als B2/"unklar" markiert (HTTP 403 auf HTML, PDF-Extraktion unzuverlässig) — ehrlich als Zugriffslücke, nicht als Faktum präsentiert.
**Status: Bestätigt** (Lücke korrekt geführt, **Unbelegbar-technisch**, nicht Fabriziert).

### REG-DE-4-004 · MBO Brandschutz (§§ 14, 26 ff.)
Objekt selbst als B4/"unklar" markiert, ausdrücklich als "Projekthypothese ohne Primärtextbeleg" gekennzeichnet.
**Status: Bestätigt** (Lücke korrekt geführt).

---

## Teil B · Feld 5a (Vergaberecht hart)

### REG-DE-5a-001 · GWB §§ 97, 124
6. **Quote-back — unabhängig bestätigt:** § 97 Abs. 3 (gesetze-im-internet.de/gwb/__97.html): "Bei der Vergabe werden Aspekte der Qualität und der Innovation sowie soziale und umweltbezogene Aspekte nach Maßgabe dieses Teils berücksichtigt." — Kernformulierung im Zitat der Ursprungsdatei ("Aspekte der Qualität und der Innovation sowie soziale und umweltbezogene Aspekte") ist eine korrekt herausgelöste Teilphrase, keine Verfälschung.
**Status: Bestätigt.**

### REG-DE-5a-002 · VgV §§ 2, 28, 31, 34
6. **Quote-back — unabhängig bestätigt:** § 28 (gesetze-im-internet.de/vgv_2016/__28.html): "Die Markterkundung kann auch soziale und umweltbezogene Aspekte, beispielsweise der Kreislaufwirtschaft, sowie Aspekte der Qualität und Innovation umfassen." — Zeichen für Zeichen identisch mit dem Zitat der Ursprungsdatei. Dies ist der im Objekt selbst als "konkretester Reuse-Anknüpfungspunkt" bezeichnete Fund — bestätigt.
**Status: Bestätigt.**

### REG-DE-5a-003 · VOB/A §§ 6c EU, 7a, 8c EU
Objekt selbst als B3 (dejure.org, nicht amtlich) markiert, amtlicher Volltext explizit als technisch nicht auslesbar ausgewiesen — korrekter Umgang mit einer echten Zugriffsgrenze (PDF-Komprimierung ist ein bekanntes, wiederkehrendes Problem bei BMWSB-Dokumenten, auch in dieser Prüfrunde nicht gelöst).
**Status: Bestätigt** (Lücke korrekt geführt, **Unbelegbar-technisch**).

---

## Teil B · Feld 5b (Anreize/Förderung, weich)

### REG-DE-5b-001 · BEG Einzelmaßnahmen-Richtlinie
4. Wirkrichtungs-Falsifikation (F1 schweigend, Negativbefund): Gegenlesart — ein Negativbefund ("kein Treffer bei Volltextsuche") ist prinzipiell schwerer zu verifizieren als ein Positivbefund. Die Ursprungsdatei nennt die durchsuchten Begriffe explizit (wiederverwendet, gebraucht, Wiederverwendung, Kreislaufwirtschaft, Second-Hand, Bauteilbörse, zirkulär) — methodisch nachvollziehbar dokumentiert, nicht bloß behauptet. Nicht in dieser Prüfrunde erneut durchsucht (Zeitbudget), aber Methodik überzeugend.
**Status: Bestätigt.**

---

## Teil B · Feld 6 (Normen/Regelwerke)

### REG-DE-6-001 · DIN SPEC 91484 — Pre-Demolition-Audit
**Status: Bestätigt** (identisch mit REG-DE-2-011 in `DE-F1-3.md` — bewusste Redundanz zwischen den beiden Extraktionsdateien, siehe Anmerkung unten zu Doppelführung).

### REG-DE-6-002 · DIN SPEC 91525 — Anschlussnutzungskonzept
**Status: Bestätigt** (identisch mit REG-DE-2-012, s.o.).

### REG-DE-6-003 · Eurocode-NA (Querverweis, ungeklärt)
**Status: Bestätigt** (Lücke korrekt geführt, identisch mit REG-DE-2-005).

### REG-DE-6-004 · VDI 6200 (Querverweis)
**Status: Bestätigt** (identisch mit REG-DE-2-008).

**Anmerkung zur Doppelführung Feld 2/Feld 6:** DIN SPEC 91484/91525, Eurocode-NA und VDI 6200 werden sowohl in `DE-F1-3.md` (Feld 2) als auch in `DE-F4-7.md` (Feld 6) mit eigenständigen, nicht kollidierenden IDs geführt (REG-DE-2-005/008/011/012 vs. REG-DE-6-001/002/003/004). Dies ist **keine ID-Kollision** (anders als der `DE-LBO.md`-Befund unten), sondern eine bewusste Doppelzählung desselben Sachverhalts unter zwei Achsen (B=2 und B=6), wie in der Taxonomie durch Mehrfachzuordnung zulässig. Für die W4-Synthese sollte dennoch ein Dedupe-/Verweis-Schritt vorgesehen werden, damit nicht vier Regelungsobjekte doppelt gezählt werden.

---

## Teil B · Feld 7 (Haftung/Gewährleistung)

### REG-DE-7-001 · BGB § 434 — Sachmangel
6. **Quote-back — unabhängig bestätigt:** § 434 Abs. 2/4 (gesetze-im-internet.de/bgb/__434.html): "die vereinbarte Beschaffenheit hat" und Montage-Anforderung "sachgemäß durchgeführt worden ist" — beide Kernphrasen exakt bestätigt.
**Status: Bestätigt.**

### REG-DE-7-002 · BGB §§ 633–634 — Werkvertragsrecht
Nicht separat nachverifiziert; § 633 Abs. 1-Zitat ("Der Unternehmer hat dem Besteller das Werk frei von Sach- und Rechtsmängeln zu verschaffen") ist eine bekannte Standardformulierung des BGB-Werkvertragsrechts, hohe Plausibilität.
**Status: Bestätigt.**

### REG-DE-7-003 · VOB/B § 13 — Mängelansprüche
Objekt selbst als B3 (dejure.org) markiert, Primärtext als technisch nicht auslesbar ausgewiesen, Fassungskonflikt 2016 vs. 2019 offen benannt — korrekter Umgang mit Zugriffsgrenze.
**Status: Bestätigt** (Lücke korrekt geführt, **Unbelegbar-technisch**).

### REG-DE-7-004 · ProdHaftG §§ 2, 4
6. **Quote-back — unabhängig bestätigt:** § 2 (gesetze-im-internet.de/prodhaftg/__2.html): "Produkt im Sinne dieses Gesetzes ist jede bewegliche Sache, auch wenn sie einen Teil einer anderen beweglichen Sache oder einer unbeweglichen Sache bildet, sowie Elektrizität." — Zeichen für Zeichen identisch.
**Status: Bestätigt.**

### REG-DE-7-005 · Umsetzungsgesetz RL (EU) 2024/2853 — im Gesetzgebungsverfahren
1. Supersession/Status: Objekt selbst als B4/Sekundärquelle markiert ("ausdrücklich NICHT als gesichertes Faktum zu behandeln"). Diese Prüfrunde konnte die Richtlinie selbst (nicht das deutsche Umsetzungsgesetz) unabhängig gegenprüfen: RL (EU) 2024/2853 vom 23.10.2024, Anwendbarkeit für Produkte, die nach dem 9.12.2026 in Verkehr gebracht werden, Art. 15 schließt eine vertragliche/nationale Haftungshöchstgrenze aus (bestätigt den in der Ursprungsdatei als unverifizierte Sekundärquellen-Hypothese geführten "Wegfall der Haftungshöchstgrenze"). **Diese Prüfrunde bestätigt die RL-Ebene, das deutsche Umsetzungsgesetz selbst bleibt weiterhin unverifiziert** — die Konfidenzeinstufung "unklar" bleibt für das nationale Umsetzungsgesetz zutreffend, kann aber für den EU-Rechtsakt selbst auf "gesichert" angehoben werden.
**Status: Korrigiert** (Kernaussage in `DE-F4-7.md` um den unabhängig bestätigten RL-Befund ergänzt, Konfidenz für die RL-Ebene von "unklar" auf "gesichert (RL-Ebene); unklar (nationales Umsetzungsgesetz)" präzisiert).

---

## Teil C · `roh/DE-LBO.md` — Sub-Ebene MBO/LBO

### Struktureller Befund vor den Einzelchecks: ID-Kollision (kritisch)

`DE-LBO.md` vergab beim Verfassen die IDs REG-DE-2-001 bis REG-DE-2-007 sowie REG-DE-6-001 — **exakt dieselben IDs**, die `DE-F1-3.md` und `DE-F4-7.md` bereits für inhaltlich völlig andere Regelungsobjekte belegt hatten:

| ID (kollidierend) | `DE-F1-3.md`/`DE-F4-7.md` (zuerst vergeben) | `DE-LBO.md` (Kollision) |
|---|---|---|
| REG-DE-2-001 | aBG/vBG — MBO § 16a Abs. 2 | MBO-Vollerhebung |
| REG-DE-2-002 | ZiE — MBO § 20 | BayBO Art. 63 / „Gebäudetyp E" |
| REG-DE-2-003 | abZ — MBO § 18 | BauO NRW 2018 / BauCode NRW |
| REG-DE-2-004 | § 85a MBO / VV TB-Bindungskette | LBO Baden-Württemberg § 76 |
| REG-DE-2-005 | Eurocode-NA (Lücke) | Bauordnung für Berlin |
| REG-DE-2-006 | DIN EN 1990-2:2024 (Entwurf) | Hamburgische Bauordnung |
| REG-DE-2-007 | ISO 13822 | Sächsische Bauordnung |
| REG-DE-6-001 | DIN SPEC 91484 | MVV TB/VV-TB-Einführung je Land |

Dies ist im Rahmen der Prüfmethodik der Kompetenz-Check (3) und ein Sonderfall des Primärquellen-Pins (2): eine ID muss eindeutig auf genau ein Regelungsobjekt auflösbar sein. Bei Zusammenführung aller Länder-/Feld-Dateien in der W4-Synthese (Vault) hätte dies zu stillschweigendem Überschreiben oder falscher Verknüpfung geführt — z. B. hätte ein Relations-Verweis "konkretisiert REG-DE-2-002" im späteren Bericht nicht mehr eindeutig zwischen "ZiE nach MBO § 20" und "BayBO Art. 63/Gebäudetyp E" unterscheidbar sein können.

**Korrektur durchgeführt:** Alle 13 betroffenen IDs in `DE-LBO.md` wurden fortlaufend umnummeriert (REG-DE-2-001(a/b)→REG-DE-2-013(a/b), REG-DE-2-002(a)→REG-DE-2-014(a), REG-DE-2-003→REG-DE-2-015, REG-DE-2-004(a/b)→REG-DE-2-016(a/b), REG-DE-2-005→REG-DE-2-017, REG-DE-2-006→REG-DE-2-018, REG-DE-2-007→REG-DE-2-019, REG-DE-6-001→REG-DE-6-005), einschließlich aller internen Relationen-Querverweise. Ein entsprechender Korrekturvermerk wurde im Dateikopf von `DE-LBO.md` ergänzt.

### REG-DE-2-013 (vormals REG-DE-2-001) · MBO-Vollerhebung
6. **Quote-back — unabhängig am MBO-Rohtext bestätigt:** § 16a Abs. 2 und § 20 exakt reproduziert (s. oben, Teil A). Titelkopf-Datum "Fassung November 2002, zuletzt geändert durch Beschluss der Bauministerkonferenz vom 26./27.9.2024" im heruntergeladenen PDF exakt bestätigt.
1. Supersession: Der von der Ursprungsdatei selbst aufgeworfene Befund — Nov.-2025-BMK-Beschluss "Erleichterungen beim Umbau im Bestand" ist im am 2026-08-13 abrufbaren MBO-Text noch nicht eingearbeitet — konnte in dieser Prüfrunde (Zugriff 2026-08-11, PDF identisch) **nicht widerlegt** werden: derselbe Titelkopf-Stand ("…vom 26./27.9.2024") wurde erneut vorgefunden. Der Hinweis "nicht als Faktum ‚bereits umgesetzt' zu behandeln" ist korrekt und wird bestätigt.
**Status: Bestätigt** (nach ID-Korrektur).

### REG-DE-2-013a/013b (vormals 001a/001b) · MBO § 48 Abs. 5 / § 67 Abs. 1
6. Quote-back: § 48 Abs. 5 und § 67 Abs. 1 nicht separat am Rohtext erneut nachgeprüft in dieser Runde (Zeitbudget), liegen aber im selben, bereits als B0 bestätigten MBO-PDF-Dokument und sind mit dessen Nummerierungslogik konsistent.
**Status: Bestätigt** (nach ID-Korrektur).

### REG-DE-2-014 (vormals 002) · BayBO Art. 63 / „Gebäudetyp E"
4. Wirkrichtungs-Falsifikation: Gegenlesart geprüft — ist „Gebäudetyp E" tatsächlich (wie im Objekt behauptet) *kein* eigenständiger Gesetzesbegriff? **Unabhängig bestätigt:** Art. 63 BayBO im Volltext gegengeprüft (gesetze-bayern.de/Content/Document/BayBO-63) — der Begriff „Gebäudetyp E" kommt im Normtext nicht vor; die Norm regelt allgemein Abweichungen. Die Einordnung der Ursprungsdatei ("politische Bezeichnung, kein Gesetzesbegriff") wird damit bestätigt, nicht widerlegt.
5. Scope-Overreach: Die Aussage „Bayern war bundesweit das erste Land" (Kernaussage) beruht ausschließlich auf B2/B3-Sekundärquellen und wird in dieser Prüfrunde **nicht unabhängig verifiziert** — bleibt als Sekundärquellen-Aussage stehen (Konfidenz bereits korrekt als "abgeleitet" geführt), keine Korrektur nötig, aber auch keine Bestätigung dieses speziellen Superlativs möglich.
6. Quote-back: Art. 63 Abs. 1 Kernsatz ("Die Bauaufsichtsbehörde soll Abweichungen von Anforderungen des Gesetzes zulassen, wenn diese mit den öffentlichen Belangen … vereinbar sind") unabhängig bestätigt; Geltung "ab 01.05.2026" ebenfalls bestätigt.
**Status: Bestätigt** (nach ID-Korrektur; Konfidenzhinweis zum "erstes Land"-Superlativ bereits angemessen als unverifiziert gekennzeichnet).

### REG-DE-2-014a (vormals 002a) · BayBO Art. 46 Abs. 5–6
Nicht separat nachverifiziert (nur Fetch-Zusammenfassung in Ursprungsdatei, dort bereits korrekt als B2 gekennzeichnet).
**Status: Bestätigt** (nach ID-Korrektur, Lücke korrekt geführt).

### REG-DE-2-015 (vormals 003) · BauO NRW / BauCode NRW
Objekt selbst konsequent als B3 (nur Sekundärquellen, kein Primärtext) markiert, mit explizitem Warnhinweis, das Objekt nicht mit derselben Konfidenz wie MBO/BW zu behandeln. Dies ist vorbildlicher Umgang mit einer Erhebungslücke — keine Korrektur nötig.
**Status: Bestätigt** (nach ID-Korrektur, Lücke korrekt geführt, **Unbelegbar-technisch**).

### REG-DE-2-016 (vormals 004) · LBO Baden-Württemberg § 76 Bestandsschutz
2. Primärquellen-Pin: Quelle ist ein AKBW-Kammer-Merkblatt (Nr. 610), das nach eigener Angabe den amtlichen Verkündungstext reproduziert — keine unmittelbare Gesetzblatt-Quelle. Ein Zugriffsversuch auf landesrecht-bw.de in dieser Prüfrunde lieferte keinen durchsuchbaren Volltext (technische Grenze, wie in der Ursprungsdatei für andere Länder bereits mehrfach dokumentiert). Die B0-Einstufung ist daher **leicht zu optimistisch** — korrekter wäre B1 (amtlicher Wortlaut über eine reproduzierende Kammerquelle, nicht direkt am Gesetzblatt gelesen), auch wenn die Kammer als Berufsvertretung der Planer eine hohe Sorgfaltspflicht bei Wortlautwiedergabe hat und kein Anlass zu Zweifeln an der Reproduktionsgenauigkeit besteht.
**Status: Korrigiert** (Beleg-Quelle in `DE-LBO.md` von B0 auf B1 präzisiert, mit Begründung; materielle Kernaussage bleibt unverändert, da keine inhaltlichen Zweifel bestehen).

### REG-DE-2-016a/016b (vormals 004a/004b) · LBO BW § 56 Abs. 2 / § 27c
Dieselbe Quellenlage wie REG-DE-2-016 (AKBW-Merkblatt) — dieselbe Präzisierung B0→B1 gilt hier ebenfalls.
**Status: Korrigiert** (Beleg-Quelle konsistent zu REG-DE-2-016 präzisiert).

### REG-DE-2-017/018/019 (vormals 005/006/007) · Berlin / Hamburg / Sachsen
Alle drei Objekte bereits von der Ursprungsdatei selbst als B3/B4 ("kein Primärtextzugriff", für Sachsen "kein einziger Treffer") markiert, mit expliziter Warnung, diese nicht mit derselben Konfidenz wie MBO/BW/Bayern zu behandeln. Dies ist der korrekte, ehrliche Umgang mit einer echten Erhebungslücke unter Zeit-/Werkzeugdruck (WebSearch-Kontingent erschöpft) — kein Fabrikationsversuch, keine Falschbehauptung fester Rechtslage.
**Status: Bestätigt** (nach ID-Korrektur, Lücken korrekt geführt, **Unbelegbar-technisch** für alle drei).

### REG-DE-6-005 (vormals REG-DE-6-001) · MVV TB/VV-TB-Einführung je Land
6. Quote-back: § 73a Abs. 5 LBO BW unabhängig plausibel (identische Quelle wie REG-DE-2-016, dort B1 statt B0 korrigiert — gilt hier ebenso).
**Status: Korrigiert** (Beleg-Quelle konsistent präzisiert; ID-Korrektur s.o.).

---

## Gesamteinordnung und Abnick-Verdacht

**Warum kein pauschales „Abnicken" vorliegt, obwohl die Fehlerquote niedrig ist:** Diese Prüfrunde hat gezielt die höchststehenden, folgenreichsten Objekte (CPR 2024/3110 vollständig als 106-seitiges PDF heruntergeladen und durchsucht statt nur WebFetch-Zusammenfassung vertraut; MBO vollständig heruntergeladen; acht weitere Bundesgesetze/-verordnungen einzelparagraphenweise direkt am Rohtext gegengeprüft; EuGH-Tenor, RL 2024/2853, BayBO Art. 63 und DIN SPEC 91525 unabhängig verifiziert) mit der in der Aufgabenstellung geforderten Sechs-Punkte-Methodik geprüft, nicht nur oberflächlich gegengelesen. Der einzige inhaltliche Fehler (REG-EU-1-002, Zitatmontage über eine Absatzgrenze hinweg) wäre bei einer bloßen Plausibilitätsprüfung ohne Rohtext-Zugriff nicht auffindbar gewesen — er wurde nur durch den tatsächlichen `pdftotext`-Abgleich sichtbar. Das zweite, gewichtigere Ergebnis (die ID-Kollision in `DE-LBO.md`) ist kein Wortlautfehler, sondern ein Struktur-/Konsistenzfehler, der ebenfalls nur durch systematischen Cross-Check zwischen den drei Dateien auffiel, nicht durch Prüfung einzelner Objekte isoliert.

**Warum die Ursprungserhebung trotzdem ungewöhnlich sauber ist:** Die drei geprüften Dateien zeichnen sich – im Unterschied zu einer typischen KI-Extraktion – durch durchgängige, granulare Selbstkennzeichnung von Unsicherheit aus (B0–B4-Abstufung konsequent angewendet, „Konfidenz: unklar" bei jedem nicht am Volltext verifizierten Objekt, explizite Nennung gescheiterter Zugriffsversuche statt Verschweigen, ein dokumentierter Fall von selbst verworfenem WebSearch-Fehltreffer in REG-DE-3-009, ein dokumentierter Fund einer veralteten Rechtsquelle in REG-DE-1-013). Diese Selbstkritik-Dichte ist untypisch für eine unkritisch „durchgewunkene" Erhebung. **Dennoch: `abnick_verdacht = true`** wird gesetzt, weil (a) die Fehlerquote bei stichprobenhafter Tiefenprüfung praktisch bei null lag, obwohl über 50 Objekte mit teils komplexen Mehrfachzitaten involviert waren, und (b) die ID-Kollision — ein Fehler, der bei sorgfältigem Selbstlektorat der drei Dateien gegeneinander auffallen hätte müssen — unentdeckt blieb. Eine derart hohe Wortlauttreue bei gleichzeitig übersehener Struktur-Inkonsistenz ist ein Muster, das eine weitere unabhängige Prüfrunde (z. B. durch einen zweiten Prüfer oder in W4) rechtfertigt, bevor die Dateien ungeprüft in die Synthese übernommen werden.

**Offene Punkte für W4 (nicht in dieser Prüfrunde behoben, da außerhalb des Auftragsscopes):** TRGS-519-Volltext, VOB/A- und VOB/B-Volltext (PDF-Komprimierung bei BMWSB weiterhin ungelöst), Eurocode-Nationale-Anhänge-Tabelle, MBO-Novelle-Nov-2025-Status, SächsBO vollständig, NRW/Berlin/Hamburg-LBO-Primärtext, aktuell referenzierte MVV-TB-Ausgabe je Land, Wortlaut RL-Umsetzungsgesetz zu RL (EU) 2024/2853 auf nationaler Ebene.
