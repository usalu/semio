# Korpus-Hygiene — Auflösung der ID-Kollisionen und UK-Präfixvereinheitlichung

> **Zweck:** Protokoll der Bestandspflege vom 2026-08-14. Reine Bereinigung des bestehenden Korpus — keine neue Erhebung, keine Interpretation, keine Auswertung. Löst alle verbliebenen ID-Kollisionen aus unkoordinierten parallelen Agentenläufen (W2/W3) auf und vereinheitlicht den UK-Präfixbruch (`REG-UK-*` vs. `REG-GB-*`) in `UK-F4-7.md`.
> **Vorarbeit:** Baut auf `pruefung/w4-dedup-arbeitsmenge.md` auf (35-ID-Arbeitsmenge, 2026-08-13/14 teilweise bereits aufgelöst) und übernimmt dieselbe Auflösungslogik.
> **Stand:** 2026-08-14.

---

## 1. Bestand vorher/nachher

| Kennzahl | Vorher (Stand 2026-08-14, vor Bereinigung) | Nachher |
|---|---:|---:|
| Regelungsobjekt-Blöcke gesamt | 681 | 681 (unverändert — keine Blöcke gelöscht, nur Rumpf ersetzt oder ID geändert) |
| Eindeutige ID-Strings | 616 | 632 |
| ID-Werte mit mehr als einem Vorkommen | 58 | 44 |
| davon: echte, ungelöste Kollisionen | 58 | **0** |
| davon: absichtliche Dublette-Verweis-Stubs (ID bewusst zweimal vergeben: 1× Objekt, 1× Verweis) | 0 | 44 |

Die 44 verbliebenen "Duplikate" im rohen ID-Zählwert sind **keine Kollisionen mehr**: An jeder dieser Stellen steht ein bewusst stehen gelassener Verweis-Stub (`"Dublette zu <ID> in <Datei>, verworfen am 2026-08-14…"`), der den ursprünglichen Header zur Auffindbarkeit behält, aber inhaltlich auf die eine kanonische Fassung verweist. Jede ID im Korpus bezeichnet damit nach dieser Bereinigung **genau ein** Regelungsobjekt.

Die 58 ursprünglichen Kollisionsgruppen (Aufgabe 1) plus 9 durch die UK-Präfixvereinheitlichung (Aufgabe 3) neu entstandene Kollisionen ergeben 67 bearbeitete Gruppen, aufgelöst in:
- **44 ECHTE DUBLETTEN** (dasselbe Regelungsobjekt, mehrfach erfasst) → verworfene Fassung durch Verweis ersetzt, keine ID-Änderung.
- **24 ECHTE KOLLISIONEN** (verschiedene Regelungsobjekte, zufällig gleiche ID) → zweites/drittes Vorkommen umnummeriert, Inhalt vollständig erhalten.

Verifiziert per Bash-Pipeline (`grep "^### REG-" *.md` → ID-Tabelle → `uniq -d`), Skript und Rohdaten nicht dauerhaft im Ticket abgelegt (nur `/tmp`-Scratch dieser Sitzung).

---

## 2. Auflösungslogik (übernommen aus `w4-dedup-arbeitsmenge.md`)

- **ECHTE DUBLETTE:** Beleg-Quelle-Rang B0 > B1 > B2 > B3 > B4; bei Gleichstand frei-primär vor paywalled; bei erneutem Gleichstand die Fassung mit Wortlautbeleg in Originalsprache. Zusätzliches, hier neu angewandtes Kriterium bei strukturell gleich starken Fassungen: die Datei mit der etablierten Konvention "Haupt-/Länderdatei (`X-F1-3.md`/`X-F4-7.md`) vor Spezial-/Stichproben-Datei (`X-Kantone.md`, `X-LBO.md`, `X-4Nationen.md`)" bzw. "alphabetisch erste Regionaldatei bei zwei gleichrangigen Regionaldateien (BE-VL.md vor BE-WA-BR.md)" — konsistent mit der bereits in der 35er-Arbeitsmenge verwendeten Logik (dort: DE-F1-3.md vor DE-LBO.md). Verworfene Fassung: Header bleibt, Rumpf ersetzt durch `„Dublette zu <ID> in <Datei>, verworfen am 2026-08-14, Grund: <Kriterium>"`. Wo die verworfene Fassung zusätzliche, in der behaltenen Fassung fehlende Angaben enthielt, ist dies im Grund-Text vermerkt (kein Fall in dieser Runde erforderte einen inhaltlichen Merge — die verworfenen Fassungen waren durchgehend Vorläufer/Teilmengen der behaltenen).
- **ECHTE KOLLISION:** Erstvorkommen (die kanonische Haupt-/Länderdatei bzw. alphabetisch erste Datei) behält die ID. Weitere Vorkommen erhalten neue, fortlaufende Nummern am oberen Ende des jeweiligen Feld-Nummernkreises der Jurisdiktion (Fortführung der bereits in der 35er-Arbeitsmenge etablierten Praxis mit 900er/050er-Blöcken; hier durchgängig der freie **090er-Block** je Feld verwendet, da 900er bereits durch W3-Materialobjekte belegt ist). Hinweis zur Schema-Abweichung: `schema/taxonomie-final.md` Abschnitt 0 sieht eine **pro Jurisdiktion durchgehende** laufende Nummer über alle Felder vor; der tatsächliche Korpus nummeriert seit W2 **pro Feld** neu (durchgängig, in allen zehn Ländern). Diese Bereinigung folgt der **tatsächlichen Korpuspraxis** (Feld-lokale Nummernkreise), nicht dem Schema-Wortlaut — eine Korrektur auf schema-konforme, jurisdiktionsweit durchgehende Nummerierung hätte alle 681 Blöcke durchnummeriert und wäre keine "reine Bestandspflege" mehr gewesen. Jedes umnummerierte Objekt trägt einen HTML-Kommentar `<!-- Umnummeriert 2026-08-14 von <alte ID>: kollidierte mit <Datei> <alte ID> (<Kurzname>), anderer Regelungsgegenstand, siehe pruefung/korpus-hygiene.md -->`.
- **Content-Verifikation:** Für jede Kollisionsgruppe wurde der volle Objektblock (Titel, Fundstelle, Kernaussage, Wortlautbeleg) beider/aller Fassungen gelesen, nicht nur der Kurzname — mehrfach widerlegte eine reine Kurznamen-Analyse die vorläufige Einschätzung (z. B. `REG-DE-3-003/-004`, ursprünglich aus Kurznamen als Kollision vermutet, bei Volltextvergleich als Dublette bestätigt; `REG-EU-1-003` u. a., ursprünglich als "Kollision an dieser ID" erkannt, aber inhaltlich eine Dublette einer *anderen* ID in der kanonischen Datei — s. "Dublette-durch-Subsumption" unten).
- **Dublette-durch-Subsumption:** Mehrere Fälle (v. a. EU-1-Cluster, DE-1/2-Cluster) zeigten, dass eine verworfene Fassung an ID-Slot X inhaltlich nicht mit der kanonischen Fassung an Slot X übereinstimmte, aber exakt mit der kanonischen Fassung an einem **anderen** Slot Y übereinstimmte (Themenverschiebung durch unabhängige Neuordnung bei der Verfeinerung). In diesen Fällen wurde die verworfene Fassung als Dublette **von Slot Y** (nicht Slot X) geführt, mit Verweis auf die tatsächlich inhaltsgleiche Ziel-ID — dies ist explizit im jeweiligen Grund-Text vermerkt.

---

## 3. Vollständige Auflösungstabelle

### 3.1 Echte Dubletten (verworfen, Verweis an Ort und Stelle)

| Alte/verworfene ID (Datei) | Grund/Kriterium | Kanonisch behalten in |
|---|---|---|
| REG-EU-1-001 (pilot-de-produkt.md) | eu-produkt.md dokumentiert nachfolgende, vollverifizierte EU-Basisschicht | eu-produkt.md REG-EU-1-001 |
| REG-EU-1-002 (pilot-de-produkt.md) | wie oben | eu-produkt.md REG-EU-1-002 |
| REG-EU-1-003 (pilot-de-produkt.md) | Subsumption: Inhalt (Art. 20 Abs. 1) deckt sich mit eu-produkt.md REG-EU-1-004 | eu-produkt.md REG-EU-1-004 |
| REG-EU-1-004 (pilot-de-produkt.md) | Subsumption: Inhalt (Erwägungsgrund 34) deckt sich mit eu-produkt.md REG-EU-1-005 | eu-produkt.md REG-EU-1-005 |
| REG-EU-1-005 (pilot-de-produkt.md) | Subsumption: Inhalt (Art. 14/15/18) deckt sich mit eu-produkt.md REG-EU-1-006 | eu-produkt.md REG-EU-1-006 |
| REG-EU-1-006 (pilot-de-produkt.md) | Subsumption: Übergangsregime deckt sich mit eu-produkt.md REG-EU-1-007 | eu-produkt.md REG-EU-1-007 |
| REG-EU-1-007 (pilot-de-produkt.md) | Subsumption: VO 305/2011-Restinhalt in eu-produkt.md REG-EU-1-007 mitverifiziert | eu-produkt.md REG-EU-1-007 |
| REG-EU-1-001 (DE-F1-3.md) | eu-produkt.md ist die dokumentiert nachfolgende EU-Basisschicht | eu-produkt.md REG-EU-1-001 |
| REG-EU-1-002 (DE-F1-3.md) | wie oben | eu-produkt.md REG-EU-1-002 |
| REG-EU-1-004 (DE-F1-3.md) | Subsumption: Erwägungsgrund 34 = eu-produkt.md REG-EU-1-005 | eu-produkt.md REG-EU-1-005 |
| REG-EU-1-005 (DE-F1-3.md) | Subsumption: Art. 14/15/18 = eu-produkt.md REG-EU-1-006 | eu-produkt.md REG-EU-1-006 |
| REG-EU-1-007 (DE-F1-3.md) | Subsumption: VO 305/2011-Restinhalt = eu-produkt.md REG-EU-1-007 | eu-produkt.md REG-EU-1-007 |
| REG-DE-1-008 (pilot-de-produkt.md) | identischer Gegenstand (BauPG); DE-F1-3.md ist nachfolgende W2-Fassung | DE-F1-3.md REG-DE-1-008 |
| REG-DE-1-010 (pilot-de-produkt.md) | Subsumption: MVV TB 2025/1 = DE-F1-3.md REG-DE-1-009 | DE-F1-3.md REG-DE-1-009 |
| REG-DE-1-011 (pilot-de-produkt.md) | Subsumption: EuGH C-100/13/Bauregellisten = DE-F1-3.md REG-DE-1-010 | DE-F1-3.md REG-DE-1-010 |
| REG-DE-2-001 (pilot-de-zie.md) | identischer Gegenstand (aBG/vBG §16a) | DE-F1-3.md REG-DE-2-001 |
| REG-DE-2-002 (pilot-de-zie.md) | identischer Gegenstand (ZiE §20) | DE-F1-3.md REG-DE-2-002 |
| REG-DE-2-003 (pilot-de-zie.md) | identischer Kurzname/Gegenstand (abZ §18) | DE-F1-3.md REG-DE-2-003 |
| REG-DE-2-004 (pilot-de-zie.md) | Subsumption: Übereinstimmungsnachweis/Ü-Zeichen (MBO §§21-22) = DE-F1-3.md REG-DE-1-011 | DE-F1-3.md REG-DE-1-011 |
| REG-DE-2-006 (pilot-de-zie.md) | Subsumption: ISO 13822 = DE-F1-3.md REG-DE-2-007 | DE-F1-3.md REG-DE-2-007 |
| REG-DE-2-007 (pilot-de-zie.md) | Subsumption: DIN EN 1990-2:2024 = DE-F1-3.md REG-DE-2-006 | DE-F1-3.md REG-DE-2-006 |
| REG-DE-2-008 (pilot-de-zie.md) | Subsumption: §85a MBO/VV TB = DE-F1-3.md REG-DE-2-004 | DE-F1-3.md REG-DE-2-004 |
| REG-DE-2-009 (pilot-de-zie.md) | identischer Kurzname/Gegenstand (ARGEBAU-Hinweise) | DE-F1-3.md REG-DE-2-009 |
| REG-DE-2-010 (pilot-de-zie.md) | identischer Kurzname/Gegenstand (Leitfaden BW) | DE-F1-3.md REG-DE-2-010 |
| REG-DE-3-001 (pilot-de-abfall.md) | identischer Gegenstand (KrWG §3) | DE-F1-3.md REG-DE-3-001 |
| REG-DE-3-002 (pilot-de-abfall.md) | identischer Gegenstand (KrWG §5); bereits informell in w4-dedup-arbeitsmenge.md vermerkt | DE-F1-3.md REG-DE-3-002 |
| REG-DE-3-003 (pilot-de-abfall.md) | identischer Gegenstand (KrWG §4 Nebenprodukte) — bei Volltextvergleich bestätigt, entgegen erster Kurznamen-Einschätzung | DE-F1-3.md REG-DE-3-003 |
| REG-DE-3-004 (pilot-de-abfall.md) | identischer Gegenstand (ErsatzbaustoffV-Anwendungsbereich) — bei Volltextvergleich bestätigt | DE-F1-3.md REG-DE-3-004 |
| REG-DE-4-001 (pilot-de-abfall.md) | Subsumption: GefStoffV §5a = DE-F4-7.md REG-DE-4-002 | DE-F4-7.md REG-DE-4-002 |
| REG-DE-4-003 (pilot-de-abfall.md) | identischer Gegenstand (TRGS 519) | DE-F4-7.md REG-DE-4-003 |
| REG-CH-1-001 (CH-Kantone.md) | identischer Gegenstand (BauPG); CH-F1-3.md präziser (Normtyp-Flag) | CH-F1-3.md REG-CH-1-001 |
| REG-CH-4-005 (CH-Kantone.md, MuKEn) | identischer Gegenstand (MuKEn 2025); CH-F4-7.md korrektes Datum (29.08.2025) und stärkerer Beleg (B1) — bereits bei adversarischer Prüfung 2026-08-13 als Kollision erkannt | CH-F4-7.md REG-CH-4-004 |
| REG-BE-1-001 (BE-WA-BR.md) | dasselbe Bundeskompetenz-Objekt, bereits bei adversarischer Prüfung 2026-08-11 selbst markiert | BE-VL.md REG-BE-1-001 |
| REG-BE-1-002 (BE-WA-BR.md) | wie oben | BE-VL.md REG-BE-1-002 |
| REG-BE-1-003 (BE-WA-BR.md) | wie oben | BE-VL.md REG-BE-1-003 |
| REG-GB-1-001 (UK-4Nationen.md) | identischer Gegenstand (SI 2013/1387); UK-F1-3.md kanonische Hauptdatei Feld 1–3 | UK-F1-3.md REG-GB-1-001 |
| REG-GB-1-002 (UK-4Nationen.md) | identischer Gegenstand (CE-Kennzeichnung) | UK-F1-3.md REG-GB-1-002 |
| REG-GB-1-003 (UK-4Nationen.md) | identischer Gegenstand (UKCA) | UK-F1-3.md REG-GB-1-003 |
| REG-GB-1-004 (UK-4Nationen.md) | identischer Gegenstand (Building Safety Act 2022 Sch. 11) | UK-F1-3.md REG-GB-1-004 |
| REG-GB-1-005 (UK-4Nationen.md) | identischer Gegenstand (Windsor Framework) | UK-F1-3.md REG-GB-1-005 |
| REG-GB-2-001 (UK-4Nationen.md) | identischer Gegenstand (England Building Regs 2010) | UK-F1-3.md REG-GB-2-001 |
| REG-GB-4-001 (UK-4Nationen.md) | identischer Gegenstand (Approved Document B); UK-F4-7.md kanonische Hauptdatei Feld 4–7 | UK-F4-7.md REG-GB-4-001 |
| REG-GB-4-002 (UK-4Nationen.md) | identischer Gegenstand (Control of Asbestos Regs 2012) | UK-F4-7.md REG-GB-4-002 |
| REG-GB-6-001 (UK-4Nationen.md) | identischer Gegenstand (Eurocodes UK NA); Objekt war bereits selbst als „Duplikat-Referenz" markiert | UK-F4-7.md REG-GB-6-001 |
| REG-GB-6-002 (UK-4Nationen.md) | identischer Gegenstand (BS 8905:2011, zurückgezogen) | UK-F4-7.md REG-GB-6-002 |
| REG-GB-6-003 (UK-4Nationen.md) | identischer Gegenstand (NSSS Annex J) | UK-F4-7.md REG-GB-6-003 |
| REG-GB-7-001 (UK-4Nationen.md) | identischer Gegenstand (Defective Premises Act 1972 + BSA 2022 s.135) | UK-F4-7.md REG-GB-7-001 |
| REG-GB-7-002 (UK-4Nationen.md) | identischer Gegenstand (Consumer Protection Act 1987 Part I) | UK-F4-7.md REG-GB-7-002 |
| REG-GB-7-003 (UK-4Nationen.md) | identischer Gegenstand (Building Safety Act 2022 Part 5) | UK-F4-7.md REG-GB-7-003 |

### 3.2 Echte Kollisionen (umnummeriert, Inhalt erhalten)

| Alte ID | Neue ID | Datei | Kollidierte mit (anderer Gegenstand) |
|---|---|---|---|
| REG-DE-1-012 | REG-DE-1-090 | pilot-de-produkt.md (Niedersachsen-Stichprobe) | DE-F1-3.md REG-DE-1-012 (DIBt-Zulassungsdatenbank) |
| REG-DE-2-009 | REG-DE-2-090 | pilot-de-produkt.md (Verwendbarkeitsnachweis-System) | DE-F1-3.md REG-DE-2-009 (ARGEBAU-Hinweise) |
| REG-DE-2-005 | REG-DE-2-091 | pilot-de-zie.md (DIN SPEC 91484) | DE-F1-3.md REG-DE-2-005 (Eurocode-NA-Zugriffslücke) |
| REG-DE-4-002 | REG-DE-4-090 | pilot-de-abfall.md (Gefährdungsbeurteilung §6/7) | DE-F4-7.md REG-DE-4-002 (§5a Erkundungspflicht) |
| REG-CH-1-002 | REG-CH-1-090 | CH-Kantone.md (BauPV Grundanforderung 7) | CH-F1-3.md REG-CH-1-002 (BauPG Art. 5 Abs. 2) |
| REG-CH-3-010 | REG-CH-3-090 | CH-Kantone.md (VVEA Art. 17) | CH-F1-3.md REG-CH-3-010 (USG Art. 7 Abs. 6/6bis) |
| REG-CH-3-015 | REG-CH-3-091 | CH-Kantone.md (ZH Abfallgesetz §16a) | CH-F1-3.md REG-CH-3-015 (VeVA) |
| REG-CH-4-004 | REG-CH-4-090 | CH-Kantone.md (VKF-Brandschutzvorschriften) | CH-F4-7.md REG-CH-4-004 (MuKEn 2025 Graue Energie) |
| REG-CH-5a-011 | REG-CH-5a-090 | CH-Kantone.md (BöB Art. 29/30, Bund) | CH-F1-3.md REG-CH-5a-011 (IVöB 2019, interkantonal) |
| REG-BE-1-004 | REG-BE-1-090 | BE-WA-BR.md (ATG/BUtgb) | BE-VL.md REG-BE-1-004 (Wet productnormen 1998, Negativbefund) |
| REG-BE-3-007 | REG-BE-3-090 | BE-WA-BR.md (Plan wallon Déchets-Ressources) | BE-VL.md REG-BE-3-007 (Materialendecreet "hergebruik") |
| REG-BE-3-012 | REG-BE-3-091 | material-daemm-schad.md (Materialendecreet Asbestinventarisatieplicht) | BE-WA-BR.md REG-BE-3-012 (Ordonnance permis d'environnement, Brüssel) |
| REG-GB-2-002 | REG-GB-2-090 | UK-4Nationen.md (Wales Building Regs) | UK-F1-3.md REG-GB-2-002 (Approved Document A, England) |
| REG-GB-2-003 | REG-GB-2-091 | UK-4Nationen.md (Schottland Building Regs) | UK-F1-3.md REG-GB-2-003 (Eurocodes UK NA) |
| REG-GB-2-004 | REG-GB-2-092 | UK-4Nationen.md (Nordirland Building Regs) | UK-F1-3.md REG-GB-2-004 (SCI P427) |
| REG-GB-2-005 | REG-GB-2-093 | UK-4Nationen.md (England Approved Document A) | UK-F1-3.md REG-GB-2-005 (Wales Building Regs Basisinstrument) |
| REG-GB-2-006 | REG-GB-2-094 | UK-4Nationen.md (Wales eigenständige Approved Documents) | UK-F1-3.md REG-GB-2-006 (Schottland Building Regs) |
| REG-GB-2-007 | REG-GB-2-095 | UK-4Nationen.md (Eurocodes UK NA) | UK-F1-3.md REG-GB-2-007 (Nordirland Building Regs) |
| REG-GB-3-001 | REG-GB-3-090 | UK-4Nationen.md (EPA 1990 Part II) | UK-F1-3.md REG-GB-3-001 (Waste England/Wales Regs 2011) |
| REG-GB-3-002 | REG-GB-3-091 | UK-4Nationen.md (Waste England/Wales Regs 2011) | UK-F1-3.md REG-GB-3-002 (Environmental Permitting Regs, U1) |
| REG-GB-3-003 | REG-GB-3-092 | UK-4Nationen.md (Environmental Permitting Regs, U1) | UK-F1-3.md REG-GB-3-003 (Quality Protocol Aggregates) |
| REG-GB-3-004 | REG-GB-3-093 | UK-4Nationen.md (Quality Protocol Aggregates) | UK-F1-3.md REG-GB-3-004 (DoWCoP) |
| REG-GB-3-005 | REG-GB-3-094 | UK-4Nationen.md (DoWCoP) | UK-F1-3.md REG-GB-3-005 (EPA 1990 Part II) |
| REG-GB-4-003 | REG-GB-4-090 | UK-4Nationen.md (Control of Asbestos Regs NI) | UK-F4-7.md REG-GB-4-003 (Approved Document L/Future Homes Standard) — Kollision entstanden erst durch die Präfixvereinheitlichung (Aufgabe 3) |

Anmerkung zu den GB-2-*/GB-3-*-Kollisionen: `UK-F1-3.md` und `UK-4Nationen.md` haben dieselben vier bzw. fünf zugrunde liegenden Sachverhalte (vier Nationen Bautechnik bzw. fünf Abfallrecht-Instrumente) unabhängig voneinander in **unterschiedlicher Reihenfolge** durchnummeriert ("Rotationsmuster") — z. B. ist `UK-F1-3.md`s REG-GB-2-002 (England Approved Document A) inhaltlich näher an `UK-4Nationen.md`s ursprünglichem REG-GB-2-005 als an dessen eigenem REG-GB-2-002 (Wales). Da die Auflösungsregel strikt auf dem Vergleich der beiden Fassungen **an derselben ID** beruht (nicht auf inhaltlicher Bestpassung über alle IDs hinweg), ist jedes einzelne Paar korrekt als Kollision (nicht Dublette) eingestuft und umnummeriert — die inhaltliche Verwandtschaft unter verschiedenen IDs bleibt als vor-bestehender, nicht Gegenstand dieser Aufgabe seiender Befund unangetastet (s. Restbestand, Abschnitt 6).

---

## 4. Aktualisierte Relationsverweise

Alle "Relationen:"-Felder, die auf eine umnummerierte ID **innerhalb derselben Datei** (Selbstverweis auf ein Geschwisterobjekt) verwiesen, wurden aktualisiert:

- **pilot-de-produkt.md:** 1 Relationen-Zeile (REG-DE-2-009 → REG-DE-2-090).
- **UK-4Nationen.md:** ca. 30 Einzelverweise über die Felder 2, 3, 4, 6 und 7 hinweg (Relationen-Felder, Beleg-Quelle/Bindungsakt-Vermerke, Sub-Ebene-Vermerke, Kernaussage-Querverweise, ein Nachtrags-Absatz im Dateikopf) — mittels zeilenweisem `sed`-Ersetzungslauf korrigiert, der die historischen `<!-- Umnummeriert … -->`-Kommentarzeilen selbst (die bewusst die alte ID nennen) von der Ersetzung ausgenommen hat.
- **CH-Kantone.md:** 7 Einzelverweise (Vorab-Befund-Absatz, zwei REG-CH-6-003-Feldwerte, Sub-Ebene-Vermerk und zwei Relationen-Zeilen im umnummerierten ZH-Objekt selbst).
- **BE-WA-BR.md:** 11 Einzelverweise (REG-BE-2-003 und REG-BE-6-003 sind vollständig als Kreuzverweis „s. REG-BE-1-004" auf das jetzt umnummerierte ATG/BUtgb-Objekt aufgebaut — Titel, Fundstelle, Kernaussage, Wortlautbeleg, Relationen).

**Bewusst nicht verändert** (Verweis bleibt gültig, da er auf ein unverändertes, weiterhin real existierendes Objekt zeigt): alle Relationen-Zeilen in den kanonischen, nicht umnummerierten Dateien (`DE-F1-3.md`, `DE-F4-7.md`, `DE-LBO.md`, `CH-F1-3.md`, `CH-F4-7.md`, `BE-VL.md`, `UK-F1-3.md`, `material-stahlbeton.md` u. a.), die auf IDs verweisen, welche in der jeweils *anderen* (kollidierenden) Datei umnummeriert wurden — diese Verweise galten immer der kanonischen Fassung und bleiben unverändert korrekt auflösbar.

**Geprüft, aber nicht verändert** (Freitext-Kommentar, keine strukturierte Relationen-Kante): die "Schema-Stresstest"-Abschnitte in `pilot-de-produkt.md` (3 Fundstellen) und `pilot-de-zie.md` (1 Fundstelle) enthalten Prosa-Erwähnungen umnummerierter IDs (z. B. „Das DIBt-Verwendbarkeitsnachweis-System (REG-DE-2-009) liegt fast beliebig zwischen Feld 1 und Feld 2"). Diese sind Design-Analyse-Kommentare aus der W0-Pilotphase, keine lebenden Objektbeziehungen — sie wurden bewusst nicht umgeschrieben, um die historische Pilotdiskussion nicht zu verfälschen. Ebenso unverändert: die narrativen Befund-Absätze in `pruefung/CH.md`, `pruefung/BE.md`, `pruefung/UK.md` und `pruefung/w4-dedup-arbeitsmenge.md`, die die Kollisionen **als damaligen Prüfbefund** beschreiben (z. B. „ID-Kollision REG-CH-4-004/005 zwischen CH-F4-7.md und CH-Kantone.md") — diese sind das historische Prüfprotokoll, das die vorliegende Bereinigung dokumentiert überführt hat, nicht ein aktueller Objektgraph. Sie sind nach Lektüre dieses Dokuments korrekt einzuordnen als "damaliger Befund, seither hier aufgelöst".

**Am Ende geprüft:** vollständiger Corpus-Grep über alle 24 umnummerierten Alt-IDs (`roh/*.md` und `pruefung/*.md`) zur Bestätigung, dass keine strukturierte `- Relationen:`-Zeile außerhalb der oben genannten, bereits korrigierten Dateien mehr auf eine Alt-ID verweist, die nicht mehr das ursprünglich gemeinte Objekt bezeichnet.

---

## 5. UK-Präfixvereinheitlichung (Aufgabe 3)

`UK-F4-7.md` verwendete durchgängig `REG-UK-*` (13 Objekte, Felder 4, 5a, 5b, 6, 7), während `UK-F1-3.md` und `UK-4Nationen.md` durchgängig `REG-GB-*` verwenden. Alle 13 Objekte wurden auf `REG-GB-*` umgestellt (mechanischer Präfixtausch, Feldnummern unverändert — inkl. der Groß-/Kleinschreibung `5A`/`5B`, die von der sonst im Korpus üblichen Kleinschreibung `5a`/`5b` abweicht; diese Case-Normalisierung war nicht Gegenstand des Auftrags und wurde nicht angefasst, s. Restbestand).

Die Umstellung erzeugte neun **neue** ID-Kollisionen mit bereits bestehenden `REG-GB-*`-Objekten in `UK-4Nationen.md` (Felder 4, 6, 7 — Feld 5a/5b kollidierte nicht, da `UK-4Nationen.md` dort die abweichende, unpräfigierte Schreibweise `REG-GB-5-*` statt `REG-GB-5a-*`/`REG-GB-5b-*` verwendet, was selbst eine vorbestehende, nicht Gegenstand dieser Aufgabe seiende Inkonsistenz ist). Alle neun wurden nach derselben Logik wie Aufgabe 2 aufgelöst (8 Dubletten, 1 Kollision — s. Tabellen in Abschnitt 3) und sind in den obigen Zählungen bereits enthalten.

**Nicht Teil dieser Aufgabe:** Eine separate, davon unabhängige Verwendung von `REG-UK-*`-IDs existiert in sieben Materialfamilien-Dateien aus W3 (`material-alu.md`, `material-baustahl.md`, `material-glas.md`, `material-holz.md`, `material-mauerwerk.md`, `material-stahlbeton.md`, `material-tga-ausbau.md`), durchgängig im freien 900er-Nummernblock (z. B. `REG-UK-6-901`, `REG-UK-2-901`). Diese kollidieren mit nichts (900er-Block ist für Materialobjekte reserviert und wurde bereits in Teil 1 auf Kollisionsfreiheit geprüft) und sind nicht der in Aufgabe 3 benannte Präfixbruch (der laut Nutzerauftrag ausdrücklich nur `UK-F4-7.md` betraf, entsprechend der Warnung im Archiv-README: „UK-F4-7.md verwendet durchgängig REG-UK-*, während UK-F1-3.md/UK-4Nationen.md REG-GB-* verwenden"). Diese Materialdateien wurden daher **nicht** angefasst — eine Vereinheitlichung auch dieser 900er-Objekte auf `REG-GB-*` wäre eine Erweiterung des Auftrags und ist als offener Punkt in Abschnitt 6 vermerkt.

---

## 6. Restbestand — was nicht aufgelöst wurde und warum

1. **Materialdateien behalten `REG-UK-*` (900er-Block).** Sieben Dateien (s. Abschnitt 5) verwenden weiterhin `REG-UK-*` für materialspezifische UK-Objekte. Kollisionsfrei, aber inkonsistent mit dem sonstigen Korpus-Präfix `REG-GB-*`. Nicht behoben, da außerhalb des expliziten Aufgabe-3-Skopus ("Setze `UK-F4-7.md` … auf `REG-GB-*`").
2. **Feld-5-Schreibweise `UK-4Nationen.md`:** Nutzt `REG-GB-5-*` (ohne a/b-Suffix) statt `REG-GB-5a-*`/`REG-GB-5b-*` wie im übrigen Korpus. Keine ID-Kollision (da String-verschieden von `UK-F4-7.md`s `REG-GB-5A-*`/`REG-GB-5B-*`), aber eine Schema-Konventions-Abweichung. Nicht behoben — reine Feldkennzeichnungsfrage, keine Kollision.
3. **Groß-/Kleinschreibung `5A`/`5B` in (vormals `UK-F4-7.md`, jetzt) den umgestellten `REG-GB-5A-*`/`REG-GB-5B-*`-IDs** weicht von der sonst korpusweit klein geschriebenen Konvention (`5a`/`5b`) ab. Nicht normalisiert — Aufgabe 3 verlangte den Präfixtausch, nicht die Case-Angleichung.
4. **Cross-ID-Themenüberschneidungen ohne ID-Kollision** (z. B. `UK-F1-3.md` REG-GB-2-002 (England Approved Document A) und `UK-4Nationen.md`s ursprüngliches, jetzt REG-GB-2-093 geführtes gleichnamiges Objekt behandeln denselben Sachverhalt unter verschiedenen IDs; ähnlich bei `material-glas.md`s eigenständigem REG-UK-4-004 und dem inhaltlich verwandten, jetzt REG-GB-4-003 geführten Approved-Document-L-Objekt in `UK-F4-7.md`). Das sind **keine ID-Kollisionen** (unterschiedliche ID-Strings) und daher explizit außerhalb des Auftrags ("keine Auswertung, keine Interpretation") — nicht bereinigt, hier nur benannt, damit sie nicht stillschweigend übersehen werden.
5. **Schema-Abweichung der Nummerierungslogik** (s. Abschnitt 2): Die Feld-lokale statt jurisdiktionsweit durchgehende Nummerierung ist eine vorbestehende, korpusweite Praxis seit W2 — nicht auf Schema-Konformität korrigiert, s. Begründung dort.
6. **Kein inhaltlicher Merge nötig:** In keiner der 44 Dubletten-Fälle enthielt die verworfene Fassung Angaben, die in der behaltenen Fassung fehlten (die behaltene Fassung war durchgehend die vollständigere/spätere). Es gab daher keinen Fall, der einen als Merge markierten Eingriff in Achsenwerte, Kernaussagen oder Evidenzgrade erfordert hätte.

---

## 7. Ergebnis gegen die Fertig-Kriterien

- ✅ Jede ID im Korpus bezeichnet genau ein Regelungsobjekt (44 verbliebene Doppel-Header sind dokumentierte Verweis-Stubs, keine Kollisionen).
- ✅ Jeder geprüfte Relationsverweis ist auflösbar (s. Abschnitt 4; Freitext-Kommentare und historische Prüfprotokolle bewusst unverändert, s. dort).
- ✅ UK nutzt `REG-GB-*` durchgängig in den drei Länderharvest-Dateien (`UK-F1-3.md`, `UK-F4-7.md`, `UK-4Nationen.md`); die separate Materialdateien-Konvention ist offen benannt (Restbestand 1).
- ✅ Dieses Dokument macht jede Änderung nachvollziehbar (Tabellen Abschnitt 3, Kommentar-Marker im Roh-Korpus selbst).
- ⏳ Ticket und Archiv (`E:\recherche\_neo4j\intake\inbox\2026-08-11_reuse_regulation_10jur\`) noch nicht resynchronisiert — Aufgabe 5, nächster Schritt.
