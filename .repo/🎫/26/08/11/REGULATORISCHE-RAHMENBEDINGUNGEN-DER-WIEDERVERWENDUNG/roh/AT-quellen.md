# Quellenaufschluss Österreich (AT) — Fundstellenliste reuse-relevante Bauregulierung

**Projekt:** BBSR/Zukunft Bau „Entwerfen mit Bestand" (Az. 10.08.18.7-25.06, LUH Hannover + UdK Berlin)
**Stufe:** W2, Stufe 1 (Quellenaufschluss) — **noch KEINE Extraktion in Regelungsobjekte.** Ziel dieses Dokuments ist die belastbare Quellenkarte je Regelungsfeld 1–7 (Taxonomie-Freeze, `schema/taxonomie-final.md`), mit amtlichem Titel, zuständiger Behörde, Fundstelle-URL, Rechtsstand as-amended zum 2026-08-11, Hinweis auf reuse-relevante Paragrafen und Supersession-Prüfung.
**Amtssprache:** Deutsch. Primärportale: **ris.bka.gv.at** (Bundesrecht konsolidiert + Landesrecht konsolidiert aller 9 Bundesländer, live abrufbar, kein Bot-Block festgestellt), **oib.or.at** (Österreichisches Institut für Bautechnik — OIB-Richtlinien 1–6, frei zugänglich als PDF), **austrian-standards.at** (Austrian Standards Institute/ON — ÖNORMen, grundsätzlich kostenpflichtig; einzelne ÖNORMen liegen als PDF auf Bundes-/Landesbehördenseiten frei vor, s. u.).
**Jurisdiktionstyp:** Österreich ist EU-Mitgliedstaat. Baurecht (inkl. bautechnische Anforderungen, Bauproduktrecht am Verwendungsort, Zustimmungs-/Zulassungsverfahren) ist **Landeskompetenz** (Art. 15 B-VG) — es gibt **keine** bundesweite Bauordnung; jedes der neun Bundesländer hat eine eigene Bauordnung und ein eigenes Bauproduktegesetz. Abfall-/Stoffrecht (AWG 2002, RBV, DVO 2008) und Zivil-/Produkthaftungsrecht (ABGB, PHG) sind dagegen **Bundeskompetenz** und bundeseinheitlich. Die OIB-Richtlinien sind selbst kein Recht, sondern werden erst durch eine landeseigene Bautechnikverordnung (o. ä.) für verbindlich erklärt — **Bindungsmechanismus live verifiziert am Beispiel Wien** (s. Feld 2, 2.2). Gemäß Taxonomie-Freeze Abschnitt 8 gilt für AT **„Stichprobe-und-Deklaration"** bei der Sub-Ebene, nicht Vollerhebung — Stichprobe in diesem Aufschluss: **Wien, Vorarlberg, Oberösterreich**; die übrigen sechs Bundesländer (Niederösterreich, Burgenland, Steiermark, Kärnten, Salzburg, Tirol) sind für Feld 1/2 strukturell identifiziert (Existenz eigener Bauproduktegesetze/Bauordnungen bestätigt, s. Suchtreffer), aber **nicht** einzeln volltextgeprüft — ausdrücklich als „nicht erhoben" zu vermerken.
**Zugriff/Stand:** Alle unten genannten Fundstellen wurden am 2026-08-13 (Systemdatum der Recherchesitzung; Stichtag der Taxonomie bleibt 2026-08-11, keine materielle Änderung im Zeitraum festgestellt) über ris.bka.gv.at, oib.or.at und WebSearch live abgerufen (WebSearch + Browser-Fetch von ris.bka.gv.at, das für den WebFetch-Tool technisch mit HTTP 503 blockiert war — **frei-primär-blockiert für WebFetch, aber über Browser-Session frei-primär abrufbar**; alle RIS-Wortlautbelege in diesem Dokument stammen aus der Browser-Session, nicht aus dem Gedächtnis).

---

## Wichtiger Vorab-Befund (Fallenlisten-Relevanz, methodisch)

**WebFetch auf ris.bka.gv.at liefert durchgehend HTTP 503**, vermutlich Bot-Schutz/Ratenbegrenzung gegenüber dem WebFetch-Client. Über eine echte Browser-Session (mcp Claude_Browser) ist ris.bka.gv.at dagegen **ohne jede Einschränkung** abrufbar — alle konsolidierten Gesamtvorschriften und Einzelparagrafen wurden so im Wortlaut gelesen. Für die Extraktion in W2 ist dies als Beschaffungshinweis zu vermerken: **ris.bka.gv.at ist `frei-primär`, nicht `frei-primär-blockiert`** — der Blockade-Befund gilt nur für den spezifischen WebFetch-Tool-Kanal, nicht für das Portal selbst.

**Kein Bauregelliste-Analogon in DE-Form, aber ein strukturähnliches System:** Anders als in Deutschland (Bauregelliste durch EuGH C-100/13 abgeschafft, s. DE-Fallenliste) betreibt Österreich weiterhin — landesrechtlich, nicht bundeseinheitlich — ein aktives Listensystem für nicht-harmonisierte Bauprodukte: die **Baustoffliste ÖA** (nationale Regelwerke/Bautechnische Zulassung für Produkte ohne hEN) und **Baustoffliste ÖE** (Anwendungsbedingungen für CE-Produkte), z. B. §§ 3, 5, 6, 12 Vorarlberger Bauproduktegesetz. Dieses System ist zum Stichtag 2026-08-11 **unverändert in Kraft** und live im Wortlaut geprüft (s. Feld 1, 1.2). Für die Extraktion wichtig: Dies ist **kein** Fehltreffer der DE-Fallenliste, weil es sich um eine eigenständige, aktuell geltende Landesnorm handelt, keine Fortführung der abgeschafften DE-Bauregelliste.

**Baurecht ist in AT konsequent Länderkompetenz — auch das Bauprodukterecht am Verwendungsort.** Ein früheres **Bundes-Bauproduktegesetz** (RIS Gesetzesnummer 10012765) existiert im RIS-Index, ist aber zum Stichtag 2026-08-11 **live verifiziert nicht in Kraft** (RIS-Meldung: „Die Rechtsvorschrift '10012765' ist am 13.08.2026 nicht mehr oder noch nicht in Kraft."). Bauproduktrecht am Verwendungsort ist damit **ausschließlich Landesrecht** — neun eigenständige Bauproduktegesetze, keine Umsetzung über ein Bundesgesetz. Dies ist für die A-Achse (Bindungsebene) und die Sub-Ebene-Erhebung zentral.

**OIB-Richtlinien sind kein eigener Rechtsformtyp, sondern strukturell Muster-/Modellrecht (D-Wert 12) — Bindungskette live am Beispiel Wien verifiziert:** Die OIB-Richtlinien 1–6 (2023, Ausgabe 6 zusätzlich 2025) werden vom Österreichischen Institut für Bautechnik als bundesländerübergreifend abgestimmtes Regelwerk erarbeitet, entfalten aber erst durch eine **eigene Verordnung jedes Bundeslands** (in Wien: Wiener Bautechnikverordnung 2023, WBTV 2023, LGBl. Nr. 14/2024) rechtliche Bindungswirkung — § 1 WBTV 2023 live gelesen: die OIB-Richtlinien werden per Verweisungsnorm zur Erfüllung der bautechnischen Vorschriften der Bauordnung für Wien erklärt. Diese Konstruktion ist strukturanalog zu MBO/MVV TB in Deutschland (Bindungsakt: `benannt`, konkrete Listung für Wien verifiziert; für die übrigen acht Länder strukturell zu erwarten, aber laut Sub-Ebene-Konvention **nicht** einzeln geprüft).

**OIB-Richtlinie 7 „Nachhaltige Nutzung der natürlichen Ressourcen" — zentraler Zukunftsbefund, noch KEIN geltendes Recht:** Erstmals wird eine OIB-Richtlinie explizit Kreislaufwirtschaft/Rückbaubarkeit/Wiederverwendbarkeit von Bauteilen als bautechnische Grundanforderung regeln (7. Grundanforderung der EU-Bauprodukteverordnung 305/2011, bislang ohne eigenes Grundlagendokument). Stand 2026-08-13: **Grundlagendokument seit 2023 in Konsultation, Stakeholder-Workshops laufend, Veröffentlichung der Richtlinie für 2027 angekündigt** — zum Stichtag 2026-08-11 **kein geltendes Recht, keine Landesverordnung kann sie bereits binden.** Für die Extraktion als „Entwurf"-Status vorzumerken, nicht als aktuelles Regelungsobjekt zu kodieren, aber als Beobachtungsposten mit hoher Priorität zu führen.

---

## Feld 1 — Produkt-/Konformitätsrecht

### 1.1 Verordnung (EU) 2024/3110 — unmittelbar geltendes EU-Recht in AT
- **Amtlicher Titel:** wie EU-Basisschicht (s. `roh/eu-produkt.md`, REG-EU-1-001 ff.) — für AT als EU-Mitgliedstaat unmittelbar und unverändert geltend, keine eigenständige nationale Rezeption nötig oder zulässig (Verordnungscharakter).
- **Zuständige Behörde (Vollzug/Marktüberwachung AT):** Landesregierungen als Marktüberwachungsbehörden für Bauprodukte (s. 1.2, § 25 Vorarlberger BauPG), koordiniert über das Österreichische Institut für Bautechnik (Produktinformationsstelle, Technische Bewertungsstelle gemäß § 31 Vorarlberger BauPG).
- **Fundstelle-URL:** https://eur-lex.europa.eu/legal-content/DE/TXT/?uri=OJ:L_202403110 (identisch mit EU-Basisschicht, hier nur AT-Vollzugsbezug ergänzt)
- **Rechtsstand:** in Kraft seit 2026-01-08 (identisch EU-weit)
- **Reuse-relevanter Bezug:** Für AT gilt dieselbe unmittelbare Wirkung wie EU-weit (Art. 2, 3, 20, 21, 26 — s. EU-Basisschicht REG-EU-1-001 bis -004). AT-spezifisch zu prüfen in Extraktion: Verhältnis zur landesrechtlichen Baustoffliste-ÖA/ÖE-Systematik (1.2) — mutmaßlich Verdrängung des Landesrechts für hEN-erfasste Produkte (Art. 20 Abs. 1 CPR ist Scope-Norm, s. EU-Basisschicht REG-EU-1-004), Fortbestand des Landesrechts für nicht-hEN-erfasste Produkte (Baustoffliste-ÖA-Regime).
- **Supersession-Check:** aktuell, ersetzt VO 305/2011 (wie EU-weit).
- **Beleg-Quelle:** B0 (identisch EU-Basisschicht) für den EU-Text; B2 für den AT-Vollzugsbezug (Zuständigkeitszuordnung aus § 25 Vorarlberger BauPG abgeleitet, noch keine AT-spezifische Vollzugsanweisung/Erlass geprüft).

### 1.2 Landes-Bauproduktegesetze (9 Bundesländer) — Stichprobe Vorarlberg vollständig gelesen
- **Amtlicher Titel (Stichprobe Vorarlberg):** Gesetz über Bauprodukte und deren Verwendung (Vorarlberger Bauproduktegesetz)
- **Nummer/Datum:** LGBl.Nr. 3/2014, zuletzt geändert LGBl.Nr. 10/2025
- **Zuständige Behörde:** Vorarlberger Landesregierung (Erlass); Marktüberwachungsbehörde für Bauprodukte (§ 25) und Zulassungsstelle (§ 15) landesintern zu bestimmen
- **Fundstelle-URL:** https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=LrVbg&Gesetzesnummer=20000747
- **Rechtsstand as-amended:** geltend, live am 2026-08-13 gelesen (§§ 1–7 vollständig im Wortlaut)
- **Reuse-relevante Paragrafen (live verifiziert):**
  - **§ 4 (Anwendungsbereich 1. Unterabschnitt):** *"Dieser Unterabschnitt gilt nur für Bauprodukte, die in Serie oder serienähnlich hergestellt werden."* — **zentraler Scope-Befund**: Das gesamte ÖA-Baustoffliste-/Einbauzeichen-ÜA-Regime (§§ 4–10) ist tatbestandlich auf **serienmäßig hergestellte** Produkte beschränkt. Ein individuelles, aus einem Bestandsgebäude ausgebautes Einzelbauteil ist strukturell **kein** serienmäßig hergestelltes Produkt — starker Kandidat für „schweigend"/Nichterfassung des Wiederverwendungsfalls in der Extraktion, analog zur DE-Bauregelliste-Systematik vor deren Abschaffung, aber hier als **aktuell geltendes** Landesrecht.
  - **§ 3 Abs. 4:** *"Die Verordnung (EU) 305/2011 bleibt durch die Abs. 1 bis 3 unberührt."* — ausdrückliche Subsidiaritätsklausel ggü. EU-Produktrecht (Vorgängerverordnung genannt, CPR 2024/3110 als Nachfolgerin nicht ausdrücklich, aber durch Verweisungscharakter des Art. 3 Abs. 4 mitumfasst — in Extraktion zu prüfen, ob Novellierungsbedarf infolge 2024/3110 bereits erfolgt ist).
  - **§ 6 Abs. 5:** Baustoffliste ÖA wird vom Österreichischen Institut für Bautechnik durch Verordnung festgelegt (im Einvernehmen mit Wirtschaftskammer, Zustimmung Landesregierung) — Bindungsakt-Mechanismus für die konkreten Produktlisten selbst, nicht im Gesetz selbst enthalten (Verordnungsermächtigung).
- **Supersession-Check:** aktiv, sieben Novellen seit 2014, letzte LGBl.Nr. 10/2025.
- **Beleg-Quelle:** B0 (§§ 0–13 vollständig via Browser-Session/RIS live gelesen) · Zugänglichkeit: frei-primär

### 1.3 Landes-Bauproduktegesetze — übrige acht Bundesländer (nicht volltextgeprüft, nur Existenznachweis)
- **Salzburg:** Salzburger Bauproduktegesetz — https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=LrSbg&Gesetzesnummer=20000919 (Fassung laut Suchtreffer 29.11.2025)
- **Wien:** Wiener Bauproduktegesetz 2013 — https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=LrW&Gesetzesnummer=20000459 (Fassung laut Suchtreffer 29.11.2025, Novelle LGBl. Nr. 34/2022 identifiziert)
- **Tirol:** Tiroler Bauproduktegesetz 2016 — https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=LrT&Gesetzesnummer=20000635 (Fassung laut Suchtreffer 01.12.2025)
- **Niederösterreich, Burgenland, Steiermark, Kärnten, Oberösterreich:** eigenständige Bauproduktegesetze/entsprechende Kapitel der Bauordnungen strukturell zu erwarten (Landeskompetenz durchgängig), **in diesem Aufschluss nicht einzeln recherchiert** — expliziter Erhebungslücken-Vermerk gemäß Sub-Ebene-Konvention „nicht erhoben".
- **Beleg-Quelle:** B4 (nur Existenz-/Titelnachweis über RIS-Trefferliste, kein Volltext geprüft) · Zugänglichkeit: frei-primär (grundsätzlich, ungeprüft) — **kein Faktum für Einzelinhalte**, nur Strukturnachweis für Sub-Ebene-Deklaration.

---

## Feld 2 — Bautechnische Zulassung/Standsicherheit

### 2.1 OIB-Richtlinie 1 — Mechanische Festigkeit und Standsicherheit
- **Amtlicher Titel:** OIB-Richtlinie 1, Mechanische Festigkeit und Standsicherheit
- **Herausgeber:** Österreichisches Institut für Bautechnik (OIB), konsensual mit den neun Bundesländern erarbeitet
- **Fundstelle-URL:** https://www.oib.or.at/kernaufgaben/oib-richtlinien/ (Übersichtsseite, Ausgabe 2023 aktuell zum Stichtag)
- **Rechtsstand as-amended:** OIB-Richtlinien-Ausgabe 2023 ist die aktuelle Grundausgabe; OIB-Richtlinie 6 (Energieeinsparung) zusätzlich als eigenständige Ausgabe 2025 (Generalversammlungsbeschluss 29.08.2025) vorhanden. **Übernahmestand in den Ländern uneinheitlich** — Wien und Tirol laut Suchtreffer erst ab Juli 2026 auf Ausgabe 2025 (nur betreffend RL 6), die meisten übrigen Länder wenden noch Ausgabe 2019 oder 2023 an. **Dies ist selbst ein Befund**: Die OIB-Richtlinien-Ausgabe „gilt" nicht bundeseinheitlich zu einem Stichtag, sondern gestaffelt je nach Landesübernahme — für Extraktion pro Land gesondert zu verifizieren, kein pauschales „geltend AT" möglich.
- **Reuse-relevanter Bezug:** Kein expliziter Reuse-Wortlaut in RL 1 identifiziert (Standsicherheitsnachweis-Systematik allgemein, keine Bestandsbauteil-Sonderregel). Reuse-Bezug indirekt über die allgemeine Nachweispflicht (rechnerischer Nachweis nach Eurocode, s. 2.3/6.1) — Kandidat „schweigend" in Extraktion.
- **Supersession-Check:** aktiv (2023), OIB-Richtlinie 7 (s. Vorab-Befund) als künftige Ergänzung, nicht Ablösung.
- **Beleg-Quelle:** B2 (Struktur/Titel amtlich über oib.or.at referenziert, Volltext RL 1 selbst nicht Zeile für Zeile geprüft in diesem Aufschluss) · Zugänglichkeit: frei-primär (PDF-Download über oib.or.at)

### 2.2 Bindungsakt der OIB-Richtlinien — Stichprobe Wien (Wiener Bautechnikverordnung 2023)
- **Amtlicher Titel:** Verordnung der Wiener Landesregierung, mit der bautechnische Anforderungen festgelegt werden (Wiener Bautechnikverordnung 2023 – WBTV 2023)
- **Nummer/Datum:** LGBl. Nr. 14/2024, zuletzt geändert LGBl. Nr. 26/2026
- **Zuständige Behörde:** Wiener Landesregierung; Rechtsgrundlage §§ 118 Abs. 5 und 122 Bauordnung für Wien
- **Fundstelle-URL:** https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=LrW&Gesetzesnummer=20000702
- **Rechtsstand as-amended:** geltend, live gelesen 2026-08-13
- **Reuse-relevanter Paragraf (Bindungsmechanismus, live verifiziert):** **§ 1:** *"Den im 9. Teil der Bauordnung für Wien festgelegten bautechnischen Vorschriften wird entsprochen, wenn die in den Anlagen enthaltenen Richtlinien des Österreichischen Instituts für Bautechnik, soweit in ihnen bautechnische Anforderungen geregelt werden, eingehalten werden."* — dies ist der **konkrete Bindungsakt**, der die OIB-Richtlinien (als Anlagen der WBTV 2023 kundgemacht, Ausgabe 2023) für Wien rechtsverbindlich macht. **§ 2:** Abweichung von den Richtlinien zulässig, wenn gleiches Schutzniveau nachgewiesen wird — generalklauselartige Öffnungsklausel, potenziell reuse-relevant für alternative Nachweiswege bei Bestandsbauteilen (in Extraktion näher zu prüfen).
- **Supersession-Check:** aktiv, hat die Vorgänger-WBTV abgelöst (Datum der Vorgänger-Ablösung in diesem Aufschluss nicht verifiziert).
- **Beleg-Quelle:** B0 (§§ 0–4 vollständig live gelesen) · Zugänglichkeit: frei-primär · **Bindungsakt: benannt** (Mechanismus UND konkrete Listung für Wien geprüft — Referenzobjekt für die Bindungsketten-Regel bei OIB-Richtlinien).

### 2.3 Eurocodes — Nationaler Anhang, Bindungskette (kostenpflichtige Norm)
- Norm selbst siehe **Feld 6 (6.1)** — ÖNORM-Serie B 1990 ff. (kostenpflichtig, Austrian Standards). **Bindungsakt:** OIB-Richtlinie 1 referenziert die Eurocodes mit österreichischem Nationalen Anhang für den rechnerischen Standsicherheitsnachweis; OIB-Richtlinie 1 wird ihrerseits erst über die jeweilige Landes-Bautechnikverordnung (s. 2.2, Beispiel WBTV 2023) verbindlich. **Zweistufige Bindungskette** (ÖNORM → OIB-RL → Landesverordnung), analog, aber nicht identisch mit der DE-Kette über VV TB. **Konkrete Referenzierungsklausel in OIB-Richtlinie 1 selbst wortlautgenau noch nicht verifiziert** (Bindungsakt-Zwischenzustand: „Bindungsmechanismus existiert, Listung im Einzelfall nicht verifiziert" — ausstehende Prüfung: OIB-Richtlinie-1-Volltext, welche konkreten ÖNORM-Ausgabenstände referenziert werden).

### 2.4 Landesbauordnungen — Verwendbarkeitsnachweis/bautechnische Zulassung im Einzelfall (Stichprobe)
- **Vorarlberg:** § 14 Bauproduktegesetz „Bautechnische Zulassung", § 15 „Zulassungsstelle" (s. Feld 1.2) — funktionales Äquivalent zur deutschen ZiE/abZ/aBG-Systematik, allerdings als **Bauprodukt**-Zulassung (nicht Bauart-Zulassung) konstruiert; genauer Anwendungsbereich bei Bestandsbauteil-Wiederverwendung in diesem Aufschluss nicht volltextgeprüft.
- **Oberösterreich:** Oö. Bauordnung 1994 identifiziert (https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=LROO&Gesetzesnummer=10000411, Fassung laut Suchtreffer 03.08.2026) — **nicht volltextgeprüft** in diesem Aufschluss, nur Existenznachweis (B4).
- **Übrige sechs Länder:** analog Feld 1.3 — strukturell zu erwarten, nicht erhoben.
- **Beleg-Quelle:** Vorarlberg B0 (Teil der 1.2-Lektüre); Oberösterreich B4; übrige nicht erhoben.

---

## Feld 3 — Abfall-/Stoffrecht

### 3.1 Abfallwirtschaftsgesetz 2002 (AWG 2002) § 2 Abs. 5 Z 4/6 — Begriffsbestimmungen Wiederverwendung/Vorbereitung zur Wiederverwendung
- **Amtlicher Titel:** Bundesgesetz über eine nachhaltige Abfallwirtschaft (Abfallwirtschaftsgesetz 2002 – AWG 2002)
- **Nummer/Datum:** BGBl. I Nr. 102/2002, zuletzt geändert BGBl. I Nr. 200/2021 (§ 2 mit Inkrafttretensdatum 11.12.2021)
- **Zuständige Behörde:** Bundesministerin für Klimaschutz, Umwelt, Energie, Mobilität, Innovation und Technologie (Vollzugskompetenz; Ressortname seit 2025 z. T. als BMLUK firmierend, s. Quellenangaben unten)
- **Fundstelle-URL:** https://www.ris.bka.gv.at/NormDokument.wxe?Abfrage=Bundesnormen&Gesetzesnummer=20002086&Paragraf=2
- **Rechtsstand as-amended:** geltend, live gelesen 2026-08-13 (tagesaktuelle RIS-Fassung)
- **Reuse-relevante Paragrafen (Wortlaut live verifiziert):**
  - **§ 2 Abs. 5 Z 4:** *"ist „Wiederverwendung" jedes Verfahren, bei dem Produkte sowie Bestandteile, die keine Abfälle sind, wieder für denselben Zweck verwendet werden, für den sie ursprünglich eingesetzt und bestimmt waren."*
  - **§ 2 Abs. 5 Z 6:** *"ist „Vorbereitung zur Wiederverwendung" jedes Verwertungsverfahren der Prüfung, Reinigung oder Reparatur, bei dem Produkte sowie Bestandteile von Produkten, die zu Abfällen geworden sind, so vorbereitet werden, dass sie ohne weitere Vorbehandlung wiederverwendet werden können."*
  - **§ 2 Abs. 5 Z 2a:** definiert „stoffliche Verwertung gemäß § 16 Abs. 7 und Anhang 1a" für Bau- und Abbruchabfälle ausdrücklich unter Einschluss der Vorbereitung zur Wiederverwendung.
  - **Grundnorm-Charakter (B-Flag Kandidat):** § 2 Abs. 1/3a i. V. m. § 5 (s. 3.2) determiniert die Anwendbarkeit aller nachgelagerten abfallrechtlichen Pflichten — strukturanalog zu DE KrWG § 3, in Extraktion als Grundnorm/Begriffsnorm zu kodieren.
- **Supersession-Check:** § 2 aktiv, zuletzt materiell durch BGBl. I Nr. 200/2021 geändert (nicht as-enacted 2002 zitieren).
- **Beleg-Quelle:** B0 (§ 2 vollständig, insbesondere Abs. 1–5 Z 1–7a, live via RIS gelesen) · Zugänglichkeit: frei-primär

### 3.2 AWG 2002 § 5 — Abfallende
- **Titel/Fundstelle:** wie 3.1, § 5
- **Fundstelle-URL:** https://www.ris.bka.gv.at/NormDokument.wxe?Abfrage=Bundesnormen&Gesetzesnummer=20002086&Paragraf=5
- **Rechtsstand:** geltend, Inkrafttretensdatum 11.12.2021, live gelesen
- **Reuse-relevanter Wortlaut (live verifiziert):** § 5 Abs. 1 Satz 2: *"Im Falle einer Vorbereitung zur Wiederverwendung im Sinne von § 2 Abs. 5 Z 6 ist das Ende der Abfalleigenschaft mit dem Abschluss dieses Verwertungsverfahrens erreicht."* Satz 3: *"Das Ende der Abfalleigenschaft kann nur erreicht werden, wenn die einschlägigen, für Produkte geltenden Anforderungen eingehalten werden."* — **zentraler Befund**: AT kodiert das Abfallende bei Vorbereitung zur Wiederverwendung als **automatische Rechtsfolge** des Verfahrensabschlusses (kein gesondertes Verwaltungsverfahren nötig, anders als bei der allgemeinen Abfallende-Verordnungsermächtigung des Abs. 2), sofern produktrechtliche Anforderungen (Brücke zu Feld 1, CPR/Landes-Bauproduktegesetz) eingehalten werden. § 5 Abs. 1a: Nachweispflicht (Besitzer) mit siebenjähriger Aufbewahrungsfrist.
- **Beleg-Quelle:** B0 (vollständig live gelesen) · Zugänglichkeit: frei-primär
- **Relationen (Vormerkung für Extraktion):** determiniert Anwendbarkeit von 3.1, 3.3; wird kombiniert mit Feld-1-Produktrecht (Satz 3 der Vorschrift verweist ausdrücklich auf Produktanforderungen).

### 3.3 Recycling-Baustoffverordnung (RBV)
- **Amtlicher Titel:** Verordnung des Bundesministers für Land- und Forstwirtschaft, Umwelt und Wasserwirtschaft über die Pflichten bei Bau- oder Abbruchtätigkeiten, die Trennung und die Behandlung von bei Bau- oder Abbruchtätigkeiten anfallenden Abfällen, die Herstellung und das Abfallende von Recycling-Baustoffen (Recycling-Baustoffverordnung – RBV)
- **Nummer/Datum:** BGBl. II Nr. 181/2015, geändert BGBl. II Nr. 290/2016
- **Rechtsgrundlage:** §§ 4, 5, 14 Abs. 2 Z 7, § 23 Abs. 1 AWG 2002
- **Zuständige Behörde:** Bundesministerin für Klimaschutz, Umwelt, Energie, Mobilität, Innovation und Technologie
- **Fundstelle-URL:** https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=Bundesnormen&Gesetzesnummer=20009212
- **Rechtsstand as-amended:** geltend, live gelesen 2026-08-13, letzte Novelle 2016 (keine neuere Änderung im RIS-Kopf identifiziert)
- **Reuse-relevante Paragrafen (Wortlaut live verifiziert, §§ 1–13 vollständig gelesen):**
  - **§ 1 (Ziel):** *"Ziel dieser Verordnung ist die Förderung der Kreislaufwirtschaft und Materialeffizienz, insbesondere die Vorbereitung zur Wiederverwendung von Bauteilen und die Sicherstellung einer hohen Qualität von Recycling-Baustoffen…"* — Vorbereitung zur Wiederverwendung von Bauteilen ist ausdrücklich **im Zielparagrafen** genannt, nicht nur beiläufig.
  - **§ 4 Abs. 3 (Schad- und Störstofferkundung):** *"Im Rahmen der Schad- und Störstofferkundung gemäß Abs. 1 und 2 sind auch jene Bauteile zu dokumentieren, welche einer Vorbereitung zur Wiederverwendung zugeführt werden können."* — Dokumentationspflicht für wiederverwendbare Bauteile als Teil der obligatorischen Schadstofferkundung ab 750 t Bau-/Abbruchabfall.
  - **§ 5 Abs. 1 (Rückbau):** *"Es ist sicherzustellen, dass Bauteile, die einer Vorbereitung zur Wiederverwendung zugeführt werden können und welche von Dritten nachgefragt werden, so ausgebaut und übergeben werden, dass die nachfolgende Wiederverwendung nicht erschwert oder unmöglich gemacht wird. […] Der Ausbau von wiederverwendbaren Bauteilen und die Schad- und Störstoffentfernung haben vor einem allfälligen maschinellen Rückbau zu erfolgen."* — **materielle Ausbaupflicht** für nachgefragte wiederverwendbare Bauteile, zeitlich vor dem maschinellen Rückbau, nicht nur Dokumentation. Bedingt durch Drittnachfrage („welche von Dritten nachgefragt werden") — kein Automatismus für jedes theoretisch wiederverwendbare Bauteil.
  - **Schwellenwert:** Rückbau-/Erkundungspflicht (§§ 4–5) gilt ab **750 t** Bau- oder Abbruchabfall (ausgenommen Bodenaushub), unterhalb dieser Schwelle keine RBV-Pflicht (§ 10a erlaubt zusätzlich vereinfachte bautechnische Verwertung vor Ort unter dieser Schwelle).
  - **§ 2 Z 3 i. V. m. AWG § 5 Abs. 2:** Abfallende für Recycling-Baustoffe als eigenständiges Verwertungsprodukt (parallel zur automatischen Abfallende-Regel für Vorbereitung zur Wiederverwendung nach 3.2 — **zwei unterschiedliche Abfallende-Wege im selben Rechtsraum**, in Extraktion sauber zu trennen: RBV regelt das Produkt „Recycling-Baustoff" als Sekundärrohstoff, AWG § 5 Abs. 1 Satz 2 regelt das wiederverwendete **Bauteil** unmittelbar).
  - **Bindungskette zu ÖNORM B 3151:** § 4 Abs. 1, § 5 Abs. 1 verweisen ausdrücklich auf „ÖNORM B 3151 ‚Rückbau von Bauwerken als Standardabbruchmethode', ausgegeben am 1. Dezember 2014" als verbindlich anzuwendendes Verfahren — **konkreter Bindungsakt für eine kostenpflichtige ÖNORM, live im RBV-Wortlaut verifiziert** (s. Feld 6.2 für die Norm selbst).
- **Supersession-Check:** aktiv, nicht abgelöst; einzelne Absätze durch BGBl. II Nr. 290/2016 aufgehoben (§ 3 Z 5/12, § 4 Abs. 4, § 5 Abs. 3, § 11 Abs. 2 — Streichungen, keine Substitution durch Neuregelung an anderer Stelle in diesem Aufschluss verifiziert).
- **Beleg-Quelle:** B0 (§§ 0–13 vollständig live gelesen) · Zugänglichkeit: frei-primär · **Bindungsakt: benannt** (RBV bindet ÖNORM B 3151 unmittelbar und konkret mit Ausgabedatum).

### 3.4 Deponieverordnung 2008 (DVO 2008)
- **Amtlicher Titel:** Verordnung des Bundesministers für Land- und Forstwirtschaft, Umwelt und Wasserwirtschaft über Deponien (Deponieverordnung 2008 – DVO 2008)
- **Nummer/Datum:** BGBl. II Nr. 39/2008, zuletzt geändert BGBl. II Nr. 243/2024
- **Rechtsgrundlage:** §§ 4, 23 Abs. 1 und 3, 65 Abs. 1 AWG 2002
- **Fundstelle-URL:** https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=Bundesnormen&Gesetzesnummer=20005653
- **Rechtsstand as-amended:** geltend, sieben Novellen bis 2024, live als Kopf-/Inhaltsverzeichnis geprüft (Volltext einzelner Paragrafen nicht)
- **Reuse-relevanter Bezug:** RBV § 9 Abs. 1 verweist bei Kontaminationsverdacht auf die Eluat-Grenzwerte der DVO-2008-Inertabfalldeponie-Tabelle (Anhang 1 Tabelle 4) — DVO 2008 ist damit mittelbar Qualitätsmaßstab für Recycling-Baustoffe/Bauteile, nicht selbst reuse-regelnd im engeren Sinn. Kein eigener Wiederverwendungs-Bezug identifiziert.
- **Supersession-Check:** aktiv.
- **Beleg-Quelle:** B2 (Kopf/Präambel/Inhaltsverzeichnis live geprüft, Einzelparagrafen nicht) · Zugänglichkeit: frei-primär

### 3.5 Abfallverzeichnisverordnung / Bau- und Abbruchabfälle-Abfallschlüssel (nur Existenznachweis)
- Für die Extraktion relevant, aber in diesem Aufschluss nicht recherchiert: Abfallverzeichnisverordnung (Abfallschlüsselnummern Bau-/Abbruchabfälle, insb. für gefährliche Fraktionen aus RBV § 7). **Nicht erhoben** in diesem Aufschluss.

---

## Feld 4 — Schutzziele (Brand/Energie/Schadstoffe/Gesundheit)

### 4.1 OIB-Richtlinie 2 — Brandschutz (inkl. 2.1–2.3)
- **Fundstelle-URL:** https://www.oib.or.at/kernaufgaben/oib-richtlinien/ (Ausgabe 2023; Teilrichtlinien 2.1 Betriebsbauten, 2.2 Garagen, 2.3 Gebäude >22 m Fluchtniveau)
- **Reuse-Bezug:** potenzieller Konflikt zwischen Bestandsbauteil-Wiederverwendung (insbesondere Baustahl/Holz-Tragwerke, Fassadenelemente) und aktuellen Brandschutz-Funktionsanforderungen — **Kandidat F1 bedingend/hemmend**, in Extraktion anhand des Bestandsschutz-/Abweichungsmechanismus der jeweiligen Landesbauordnung zu prüfen (analog § 2 WBTV 2023, s. 2.2).
- **Beleg-Quelle:** B2 (Existenz/Titelstruktur amtlich referenziert, Feindetail nicht geprüft) · Zugänglichkeit: frei-primär

### 4.2 OIB-Richtlinie 3 — Hygiene, Gesundheit und Umweltschutz
- **Fundstelle-URL:** https://www.oib.or.at/kernaufgaben/oib-richtlinien/
- **Reuse-Bezug:** Schadstoff-/Gesundheitsanforderungen an Bauprodukte (potenziell einschlägig für wiederverwendete Dämmstoffe, Altlacke, Holzschutzmittel) — struktureller Berührungspunkt zu RBV § 7 (Recyclingverbote für Schadstoffe, s. Feld 3.3), aber eigenständige bautechnische Schutzzielnorm. Nicht wortlautgeprüft in diesem Aufschluss.
- **Beleg-Quelle:** B2 · Zugänglichkeit: frei-primär

### 4.3 OIB-Richtlinie 6 — Energieeinsparung und Wärmeschutz (inkl. eigenständige Ausgabe 2025)
- **Fundstelle-URL:** https://www.oib.or.at/kernaufgaben/oib-richtlinien/ · Entwurfsdokument RL 6 2025: https://www.tirol.gv.at/fileadmin/themen/bauen-wohnen/baupolizei/20260219_Zentrale_%C3%84nderungen_der_OIB_Richtlinie_6_im_Jahr_2025.pdf
- **Reuse-Bezug:** energetische Anforderungen (U-Werte, Gesamtenergieeffizienz) können der Wiederverwendung von Bestandsbauteilen (insb. Altfenster, Fassadenelemente ohne aktuelle Dämmwerte) entgegenstehen → **Kandidat F1 hemmend/bedingend**. RL 6 2025 bereits beschlossen (Generalversammlung 29.08.2025), aber Landesübernahme uneinheitlich (Wien/Tirol erst ab Juli 2026 laut Suchtreffer, s. Vorab-Befund) — Rechtsstand für Extraktion **landesabhängig gesondert zu bestimmen**, kein pauschaler AT-Stichtag möglich.
- **Beleg-Quelle:** B2 (Struktur amtlich referenziert; Änderungsübersicht Tirol als Sekundärdokument, nicht der Richtlinientext selbst) · Zugänglichkeit: frei-primär

### 4.4 OIB-Richtlinie 7 — Nachhaltige Nutzung der natürlichen Ressourcen (Entwurfsstadium, s. Vorab-Befund)
- **Status:** Grundlagendokument seit 2023 öffentlich, Stakeholder-Workshops 2025/2026, Veröffentlichung angekündigt für 2027. **Kein geltendes Recht zum Stichtag 2026-08-11.**
- **Reuse-Bezug (prospektiv, aus Grundlagendokument-Sekundärquellen):** Zentrales Element ist die Bewertung des Lebenszyklus-Treibhauspotenzials (Global Warming Potential, GWP); explizit genannt werden Wiederverwendbarkeit/Recyclingfähigkeit von Bauwerk, Baustoffen und Bauteilen nach Abriss als 7. Grundanforderung der EU-Bauprodukteverordnung (ursprünglich VO 305/2011, Anhang I Nr. 7).
- **Fundstelle-URL:** https://www.oib.or.at/oib-insights/stakeholder-workshop-zur-oib-richtlinie-7-austausch-zur-nachhaltigen-nutzung-natuerlicher-ressourcen/ · Grundlagendokument (paywalled bei Austrian Standards gelistet): https://www.austrian-standards.at/en/shop/oib-richtlinie-7-grundlagendokument-2023-05~p3893766
- **Beleg-Quelle:** B3 (Sekundärquellen/OIB-Insights-Newsartikel, kein Grundlagendokument-Volltext selbst eingesehen) · Zugänglichkeit: frei-primär für Statusseite, das Grundlagendokument selbst ist bei Austrian Standards gelistet (paywalled-nicht-eingesehen) · **Status: Entwurf** — in Extraktion NICHT als aktuelles Regelungsobjekt kodieren, nur als Beobachtungsposten/Ausblick vermerken.

---

## Feld 5a — Vergaberecht (hart)

### 5.1 Bundesvergabegesetz 2018 (BVergG 2018) § 20 Abs. 5 — Umweltgerechtheit/Nachhaltigkeit als Vergabegrundsatz
- **Amtlicher Titel:** Bundesgesetz über die Vergabe von Aufträgen (Bundesvergabegesetz 2018 – BVergG 2018)
- **Nummer/Datum:** BGBl. I Nr. 65/2018, zuletzt geändert BGBl. I Nr. 8/2026
- **Zuständige Behörde:** Bundeskanzleramt (Zuständigkeit Vergaberecht); Vollzug durch alle öffentlichen Auftraggeber (Bund/Länder/Gemeinden/Sektorenauftraggeber)
- **Fundstelle-URL:** https://www.ris.bka.gv.at/NormDokument.wxe?Abfrage=Bundesnormen&Gesetzesnummer=20010295&Paragraf=20
- **Rechtsstand as-amended:** geltend, § 20 mit Inkrafttretensdatum 01.03.2026 (jüngste Novelle bereits berücksichtigt), live gelesen
- **Reuse-relevanter Wortlaut (live verifiziert):** § 20 Abs. 5: *"Im Vergabeverfahren ist auf die Umweltgerechtheit und Nachhaltigkeit der Leistung Bedacht zu nehmen. Dies kann insbesondere durch die Berücksichtigung ökologischer Aspekte (wie etwa Energieeffizienz, Materialeffizienz, Abfall- und Emissionsvermeidung, Bodenschutz, Reduktion der Flächeninanspruchnahme, Priorität der Lebenszykluskosten) oder des Tierschutzes bei der Beschreibung der Leistung, der Festlegung der technischen Spezifikationen, durch die Festlegung konkreter Eignungs- oder Zuschlagskriterien oder von Bedingungen im Leistungsvertrag erfolgen."* — **wichtiger Befund für F1/F2:** Die Norm formuliert eine allgemeine **„Bedacht zu nehmen"-Pflicht** (Grundsatznorm), konkretisiert die Umsetzung aber durchgehend mit **„kann"** (fakultative Instrumente: Leistungsbeschreibung, techn. Spezifikation, Eignungs-/Zuschlagskriterien, Vertragsbedingungen). Kreislaufwirtschaft/Wiederverwendung ist **nicht namentlich genannt**, aber unter „Materialeffizienz"/„Abfallvermeidung" subsumierbar. Kein verpflichtendes Kreislaufwirtschafts-Zuschlagskriterium für Bauvergaben identifiziert — **Kandidat „bedingend" statt „ermöglichend"** in Extraktion, da Rechtsfolge von der Ermessensausübung des Auftraggebers abhängt.
- **Ergänzend (Sekundärquelle, nicht B0):** Die naBe-Aktionsplan-Struktur des Bundes (nachhaltige öffentliche Beschaffung, naBe.gv.at) formuliert sektorale Kriterienkataloge (u. a. Hochbau) mit konkreteren Kreislaufwirtschaftskriterien — dies ist jedoch **kein Gesetz/keine Verordnung**, sondern eine Selbstbindung/Empfehlung der Beschaffungsstellen des Bundes (funktional näher an Feld 5b/Merkblatt-Charakter als an Feld 5a) — in Extraktion als eigenständiges Objekt mit D=Merkblatt oder Verwaltungsvorschrift zu prüfen, nicht in dieses BVergG-Objekt zu vermengen.
- **Supersession-Check:** aktiv, laufend novelliert.
- **Beleg-Quelle:** B0 (§ 20 vollständig live gelesen) · Zugänglichkeit: frei-primär

---

## Feld 5b — Anreize/Förderung (weich)

### 5.2 Österreichische Kreislaufwirtschaftsstrategie (2022, BMK/BMLUK)
- **Titel:** Österreich auf dem Weg zu einer nachhaltigen und zirkulären Gesellschaft — Die österreichische Kreislaufwirtschaftsstrategie
- **Herausgeber:** Bundesministerium für Klimaschutz, Umwelt, Energie, Mobilität, Innovation und Technologie (BMK; Ressort inzwischen z. T. als BMLUK firmierend)
- **Fundstelle-URL:** https://www.bmluk.gv.at/dam/jcr:baacfdef-c63e-49f5-ab8f-e4be8c0d7504/Kreislaufwirtschaftsstrategie_2022_230215.pdf · Fortschrittsbericht: https://www.bmluk.gv.at/dam/jcr:d3d23e4f-8734-4fc2-b967-7518f306ff88/Fortschrittsbericht_1_zur_oesterreichischen_Kreislaufwirtschaftsstrategie.pdf
- **Rechtscharakter:** strategisches Positionspapier ohne Gesetzes-/Verordnungscharakter (D-Wert Merkblatt-Kategorie, kein RVO/Gesetz).
- **Reuse-relevanter Bezug (laut Sekundärquellen-Zusammenfassung, Primärdokument nicht Zeile für Zeile gelesen):** Bauwirtschaft als eigenes Handlungsfeld mit drei Zielsetzungen — zirkuläre/modulare Planung mit Recyclingbaustoffen, Verlängerung der Gebäude-Nutzungsdauer durch Wartung/Sanierung, stoffliche Verwertung von Bodenaushub/Bau-/Abbruchabfällen als Normalität. Konkrete Fördermaßnahmen werden über bestehende Instrumente (Umweltförderungsgesetz, s. 5.3) operationalisiert, nicht durch die Strategie selbst.
- **Beleg-Quelle:** B3 (Sekundärquellen-Zusammenfassung, PDF-Primärdokument identifiziert, aber nicht vollständig gelesen) · Zugänglichkeit: frei-primär — **in W2-Extraktion Volltext nachzuholen, derzeit kein Faktum zu Einzelinhalten.**

### 5.3 Umweltförderungsgesetz (UFG)
- **Amtlicher Titel:** Bundesgesetz über die Förderung von Maßnahmen zum Schutz der Umwelt, zur Altlastensanierung, zur besonderen Förderung der thermisch-energetischen Sanierung sowie über die Einrichtung einer Förderungsgesellschaft (Umweltförderungsgesetz)
- **Nummer/Datum:** BGBl. Nr. 30/1994, zuletzt geändert (Fassung laut RIS-Suchtreffer 29.07.2026)
- **Zuständige Behörde:** BMK/BMLUK (Bundesministerium), Abwicklung über die Kommunalkredit Public Consulting (KPC) als Abwicklungsstelle
- **Fundstelle-URL:** https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=Bundesnormen&Gesetzesnummer=10010755
- **Rechtsstand as-amended:** geltend, laufend novelliert; § 1 als Zweckbestimmung, § 3 als Förderziele-Katalog laut Suchtreffer identifiziert, **nicht wortlautgeprüft** in diesem Aufschluss.
- **Reuse-relevanter Bezug:** UFG als gesetzliche Grundlage für Förderprogramme mit Kreislaufwirtschafts-/Flächenrecycling-Bezug (u. a. Brachflächen-/Altstandort-Revitalisierung); konkrete bauteilbezogene Reuse-Förderprogramme (z. B. klimaaktiv-Förderschienen) sind **Verordnungs-/Richtlinienebene unterhalb des UFG**, nicht im Gesetz selbst — Förderrichtlinien in Extraktion gesondert zu identifizieren.
- **Beleg-Quelle:** B2 (Kopf-/Struktur amtlich referenziert, §§ 1/3 nicht wortlautgeprüft) · Zugänglichkeit: frei-primär — **in W2-Extraktion Volltext nachzuholen.**

---

## Feld 6 — Normen/Regelwerke

### 6.1 ÖNORM-Serie B 1990 ff. — Eurocode, Nationaler Anhang (kostenpflichtig)
- **Titel (Beispiel):** ÖNORM B 1990-1, Eurocode: Grundlagen der Tragwerksplanung — Nationale Festlegungen zu ÖNORM EN 1990 und nationale Erläuterungen
- **Herausgeber:** Austrian Standards International (ON)
- **Fundstelle-URL (Katalog, paywalled):** https://www.austrian-standards.at/de/shop/onorm-b-1990-1-2013-01-01~p1981482
- **Zugänglichkeit:** paywalled-nicht-eingesehen (nur Katalogtitel/Metadaten geprüft, kein Volltext)
- **Bindungsakt:** s. Feld 2.3 — zweistufig über OIB-Richtlinie 1 → Landes-Bautechnikverordnung (Beispiel Wien, s. 2.2). **Bindungsmechanismus existiert, Listung im Einzelfall nicht verifiziert** (konkrete Referenzierungsklausel innerhalb OIB-RL 1 selbst noch nicht wortlautgeprüft).
- **Beleg-Quelle:** B4 (nur Katalog-/Existenznachweis) · Zugänglichkeit: paywalled-nicht-eingesehen · **Kein Faktum zu Norminhalten** — nur Strukturnachweis für die Bindungsketten-Regel.

### 6.2 ÖNORM B 3151:2014-12-01 — Rückbau von Bauwerken als Standardabbruchmethode
- **Titel:** ÖNORM B 3151, „Rückbau von Bauwerken als Standardabbruchmethode", ausgegeben am 1. Dezember 2014
- **Herausgeber:** Austrian Standards International (ON)
- **Fundstelle-URL (Katalog, paywalled):** https://shop.austrian-standards.at/action/de/public/details/532055/OENORM_B_3151_2014_12_01 · **Freie Zweitquelle (Behörden-PDF, vollständiger Normtext frei zugänglich):** https://www.bmluk.gv.at/dam/jcr:b5c6f981-a044-4979-9dd5-76da4bb69477/OeNORM_B3151_2014.pdf
- **Bindungsakt:** **benannt** — RBV § 4 Abs. 1 und § 5 Abs. 1 machen die Norm mit exaktem Ausgabedatum ausdrücklich verbindlich (s. Feld 3.3). Dies ist ein **Musterfall der Bindungsketten-Regel**: kostenpflichtige ÖNORM, gebunden über einen freien amtlichen Akt (RBV, Bundesverordnung).
- **Reuse-relevanter Inhalt (laut Sekundärquellen-Zusammenfassung; Volltext über die bmluk.gv.at-Zweitquelle grundsätzlich einsehbar, in diesem Aufschluss aber nur über Suchtreffer-Zusammenfassung erschlossen, **nicht selbst Zeile für Zeile gegengelesen**): Abschnitt 8.2.3 „Wiederverwendung von Bauteilen" verpflichtet den Bauherrn zur Prüfung, ob Bauteile im selben Gebäude oder anderswo wiederverwendet werden können; das Rückbaukonzept muss Angaben zur Wiederverwendbarkeit enthalten; Aufgabe der rückbaukundigen Person ist es, kommerziell verwertbare Bauteile für den Auftraggeber zu identifizieren.
- **Beleg-Quelle:** B2 für den Norminhalt selbst (Sekundärquellen-Zusammenfassung von Abschnitt 8.2.3, freie Volltext-Zweitquelle identifiziert aber nicht selbst gegengelesen — **in W2-Extraktion zwingend über die bmluk.gv.at-PDF im Wortlaut nachzuholen, dann B0/B1 möglich**) · B0 für den Bindungsakt selbst (RBV-Verweisklausel, live gelesen) · Zugänglichkeit: paywalled bei Austrian Standards, aber **frei-primär über die bmluk.gv.at-Zweitquelle** — Sonderfall, in Extraktion als „frei zugänglich über Behörden-Zweitquelle trotz grundsätzlicher Kostenpflicht bei ON" zu vermerken.

### 6.3 ÖNORM EN ISO 16000-32 — Innenraumluftverunreinigungen, Untersuchung von Gebäuden auf Schadstoffe
- **Titel:** ÖNORM EN ISO 16000-32, „Innenraumluftverunreinigungen, Teil 32: Untersuchung von Gebäuden auf Schadstoffe", ausgegeben am 1. Oktober 2014
- **Bindungsakt:** **benannt** — RBV § 4 Abs. 2 macht die Norm für die vertiefte Schad- und Störstofferkundung bei Abbruchvorhaben >750 t und >3.500 m³ Bruttorauminhalt verbindlich (live im RBV-Wortlaut gelesen, s. Feld 3.3).
- **Fundstelle-URL (Katalog, paywalled):** über austrian-standards.at Katalog zu ermitteln (in diesem Aufschluss nicht einzeln aufgerufen).
- **Beleg-Quelle:** B0 für den Bindungsakt (RBV-Verweisklausel) · B4 für den Norminhalt selbst (nur Titel/Ausgabedatum aus RBV-Zitat bekannt) · Zugänglichkeit: paywalled-nicht-eingesehen für den Norminhalt.

### 6.4 ÖNORM B 4710-1 — Beton, Regeln zur Umsetzung der ÖNORM EN 206-1 (Bindungsakt-Beispiel für Recycling-Baustoff-Einsatz)
- **Bindungsakt:** RBV § 13 Z 1 verweist auf ÖNORM B 4710-1 „ausgegeben am 1. Oktober 2007" für die Festigkeitsklassen-Abgrenzung beim zulässigen Einsatz von Recycling-Baustoffen U-B/U-E im Beton — weiterer, live verifizierter Bindungsakt-Beleg (s. Feld 3.3).
- **Beleg-Quelle:** B0 für Bindungsakt (RBV-Zitat) · B4 für Norminhalt · Zugänglichkeit: paywalled-nicht-eingesehen (Norminhalt).

### 6.5 Austrian Standards — genereller Zugänglichkeitsbefund
Austrian Standards International (ON) vertreibt ÖNORMen grundsätzlich kostenpflichtig über den eigenen Webshop (austrian-standards.at). **Ausnahme in diesem Aufschluss identifiziert:** Einzelne bau- und abfallrechtlich referenzierte ÖNORMen (mindestens B 3151, s. 6.2) werden von Bundesministerien als PDF-Volltext frei bereitgestellt, wenn sie Grundlage einer Vollzugsvorschrift (hier: RBV) sind — dies ist **kein genereller Freizugang**, sondern eine Einzelfall-Bereitstellung durch die vollziehende Behörde und für jede einzelne Norm gesondert zu prüfen.

---

## Feld 7 — Haftung/Gewährleistung

### 7.1 ABGB §§ 922 ff. — Gewährleistung, Sonderregel gebrauchte bewegliche Sachen
- **Amtlicher Titel:** Allgemeines bürgerliches Gesetzbuch (ABGB), 3. Hauptstück „Von den Rechten aus entgeltlichen Verträgen", Gewährleistung
- **Fundstelle-URL:** https://www.ris.bka.gv.at/eli/jgs/1811/946/P923/NOR12018648 (§ 923, Fundstelle für den § 922-Bereich analog über die konsolidierte ABGB-Gesamtvorschrift auf ris.bka.gv.at)
- **Rechtsstand:** geltend (Kernnorm des Zivilrechts, laufend punktuell novelliert — insbesondere Gewährleistungsrichtlinien-Umsetzung 2021/22 für Verbrauchergeschäfte, in diesem Aufschluss nicht im Detail geprüft).
- **Reuse-relevanter Bezug (laut Sekundärquellen, nicht in diesem Aufschluss B0-verifiziert):** Für gebrauchte bewegliche Sachen kann die zweijährige Gewährleistungsfrist vertraglich auf ein Jahr verkürzt werden (Sonderregel im Verbrauchergeschäft, mit Ausnahmen z. B. Gebrauchtwagen). Für unbewegliche Sachen gilt die dreijährige Frist unverändert. **Reuse-Bezug:** Wiederverwendete Bauteile, die als bewegliche Sachen gehandelt werden (vor Einbau), fallen potenziell unter die verkürzte Gewährleistungsfrist für Gebrauchtware — nach Einbau als Bestandteil eines unbeweglichen Bauwerks stellt sich die Frage der Fristzuordnung neu (Kandidat für „bedingend"/Einzelfallprüfung in Extraktion).
- **Beleg-Quelle:** B3 (Sekundärquellen-Zusammenfassung, § 922-Volltext in diesem Aufschluss nicht selbst über RIS gegengelesen — **in W2-Extraktion nachzuholen**) · Zugänglichkeit: frei-primär (RIS)

### 7.2 Produkthaftungsgesetz (PHG)
- **Amtlicher Titel:** Bundesgesetz vom 21. Jänner 1988 über die Haftung für ein fehlerhaftes Produkt (Produkthaftungsgesetz)
- **Nummer/Datum:** BGBl. Nr. 99/1988, zuletzt geändert BGBl. I Nr. 98/2001 (weitere Novellen laut RIS-Kopf: BGBl. Nr. 95/1993, 917/1993, 510/1994, BGBl. I Nr. 185/1999)
- **Zuständige Behörde:** kein Vollzugsorgan im engeren Sinn (zivilrechtliche Anspruchsgrundlage, Durchsetzung über ordentliche Gerichte)
- **Fundstelle-URL:** https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=Bundesnormen&Gesetzesnummer=10002864
- **Rechtsstand as-amended:** geltend, live gelesen 2026-08-13 (§§ 1–7 vollständig)
- **Reuse-relevanter Wortlaut (live verifiziert):**
  - **§ 4:** *"Produkt ist jede bewegliche körperliche Sache, auch wenn sie ein Teil einer anderen beweglichen Sache oder mit einer unbeweglichen Sache verbunden worden ist, einschließlich Energie."* — **kein Ausschluss gebrauchter/wiederverwendeter Produkte** vom Produktbegriff; ein wiederverwendetes Bauteil bleibt „Produkt" im Sinne des PHG auch nach Einbau in ein unbewegliches Bauwerk.
  - **§ 3:** *"Hersteller […] ist derjenige, der das Endprodukt, einen Grundstoff oder ein Teilprodukt erzeugt hat, sowie jeder, der als Hersteller auftritt, indem er seinen Namen, seine Marke oder ein anderes Erkennungszeichen auf dem Produkt anbringt."* — **kein AT-spezifisches Analogon zur CPR-2024/3110-Herstellerfiktion für Ausbau/Wiederaufbereitung** (s. EU-Basisschicht REG-EU-1-002) im PHG selbst identifiziert; das PHG regelt Deliktshaftung für Personen-/Sachschäden losgelöst vom CPR-Konformitätsrecht.
  - **§ 5:** Fehlerbegriff („Sicherheitserwartung unter Berücksichtigung aller Umstände… des Zeitpunkts, zu dem das Produkt in den Verkehr gebracht worden ist") — bei einem wiederverwendeten Bauteil stellt sich die Frage, ob der ursprüngliche oder ein neuer Inverkehrbringungszeitpunkt (Ausbau/Wiedereinbau) maßgeblich ist; PHG selbst enthält **keine ausdrückliche Reuse-Regel** hierzu — Kandidat „schweigend" in Extraktion.
- **Umsetzungsstand EU-Produkthaftungs-RL 2024/2853:** Umsetzungsfrist bis 09.12.2026 (Richtlinie in Kraft seit 08.12.2024). **Zum Stichtag 2026-08-11 noch keine AT-Umsetzungsnovelle im RIS-Änderungskopf identifiziert** — Status der Richtlinie selbst: `Übergang` (Umsetzungsfrist läuft, noch nicht abgelaufen), Status des geltenden PHG: `in Kraft` (alte Rechtslage). Für Extraktion in W2/W4 wichtig: Die neue RL 2024/2853 enthält nach Taxonomie-Vorgabe eigene Bezugnahmen zu Produkthaftung im Kreislaufwirtschaftskontext — **in diesem Aufschluss nicht im Detail geprüft**, nur Fristenlage.
- **Supersession-Check:** aktiv (altes Regime), Ablösung/Ergänzung durch RL-2024/2853-Umsetzung bis Ende 2026 zu erwarten — **Beobachtungsposten für W4**.
- **Beleg-Quelle:** B0 (§§ 1–7 vollständig live gelesen) · Zugänglichkeit: frei-primär

### 7.3 ÖNORM B 2110 — Werkvertragsnorm für Bauleistungen (paywalled, vertragliche Bindung)
- **Titel:** ÖNORM B 2110, „Allgemeine Vertragsbestimmungen für Bauleistungen — Werkvertragsnorm"
- **Rechtscharakter:** privatrechtliche Vertragsnorm, wird **nur durch ausdrückliche vertragliche Vereinbarung** Vertragsbestandteil (kein gesetzlicher Bindungsakt wie bei RBV/ÖNORM B 3151) — funktional näher an D=Branchenprotokoll als an Techn.Baubestimmung.
- **Fundstelle-URL:** über austrian-standards.at Katalog zu ermitteln (in diesem Aufschluss nicht einzeln aufgerufen, nur aus Fachliteratur-Suchtreffern als einschlägig identifiziert).
- **Reuse-relevanter Bezug:** potenziell Gewährleistungs-/Mängelrügefristen für wiederverwendete Bauteile als Werkvertragsbestandteil — **nicht recherchiert** in diesem Aufschluss, reiner Existenzhinweis.
- **Beleg-Quelle:** B4 (nur Existenznachweis) · Zugänglichkeit: paywalled-nicht-eingesehen · **kein Faktum, in W2-Extraktion ggf. nachzuholen, niedrige Priorität** (vertragliche, nicht gesetzliche Bindung).

---

## Zusammenfassende Supersession-Übersicht (Feld-übergreifend)

| Instrument | Status 2026-08-11 | Hinweis |
|---|---|---|
| Bundes-Bauproduktegesetz (RIS 10012765) | **nicht in Kraft** | live verifiziert; Bauproduktrecht am Verwendungsort ist reines Landesrecht |
| VO (EU) 305/2011 | abgelöst | durch 2024/3110 (EU-weit, wie in EU-Basisschicht dokumentiert) |
| VO (EU) 2024/3110 | in Kraft seit 2026-01-08 | unmittelbar in AT geltend |
| AWG 2002 | in Kraft, § 2/§ 5 zuletzt 2021 geändert | Grundnorm-Charakter für Feld 3 |
| RBV (BGBl. II 181/2015) | in Kraft, novelliert 2016 | zentrale Reuse-Norm AT, mehrere ÖNORM-Bindungsakte |
| DVO 2008 | in Kraft, zuletzt 2024 geändert | nur mittelbarer Reuse-Bezug |
| OIB-Richtlinien Ausgabe 2023 | in Kraft, uneinheitliche Landesübernahme | kein bundeseinheitlicher Stichtag |
| OIB-Richtlinie 6 Ausgabe 2025 | beschlossen 2025-08-29, Landesübernahme läuft | Wien/Tirol ab Juli 2026 |
| OIB-Richtlinie 7 | **Entwurf**, Veröffentlichung angekündigt 2027 | kein geltendes Recht, nur Beobachtungsposten |
| WBTV 2023 (Wien) | in Kraft, zuletzt 2026 geändert | Bindungsakt-Referenzobjekt |
| BVergG 2018 | in Kraft, § 20 Fassung ab 2026-03-01 | laufend novelliert |
| PHG (BGBl. 99/1988) | in Kraft (altes Regime) | RL 2024/2853-Umsetzung bis 2026-12-09 fällig, noch nicht erfolgt |

---

## Restlücken dieses Quellenaufschlusses (ehrlich markiert)

- **Sechs der neun Bundesländer** (NÖ, Bgld, Stmk, Ktn, Sbg, Tirol) sind für Feld 1/2 nur strukturell (Existenz Bauproduktegesetz/Bauordnung), nicht volltextgeprüft — gemäß Taxonomie-Freeze-Konvention „Stichprobe-und-Deficit" für AT zulässig, aber für Extraktion explizit als Lücke zu übernehmen, nicht stillschweigend zu extrapolieren.
- **OIB-Richtlinien 1–6 selbst** wurden nicht Zeile für Zeile gelesen (nur Meta-Struktur über oib.or.at und Suchtreffer) — insbesondere die konkrete Eurocode-Referenzierungsklausel in OIB-RL 1 und die genauen Schutzziel-Formulierungen in RL 2/3/6 sind für Extraktion nachzuholen.
- **ÖNORM B 3151 Volltext** wurde nicht direkt von der frei zugänglichen bmluk.gv.at-Zweitquelle gegengelesen, nur aus Suchtreffer-Zusammenfassung erschlossen (B2 statt B0/B1) — hohe Priorität für W2-Extraktion, da direkt frei zugänglich.
- **ABGB § 922/923** wurde nicht direkt über RIS gegengelesen (B3, Sekundärquelle) — für Feld 7 in Extraktion nachzuholen.
- **Umweltförderungsgesetz und Kreislaufwirtschaftsstrategie 2022** (Feld 5b) sind nur strukturell/zusammenfassend erschlossen, kein Wortlautbeleg — Förderrichtlinien-Ebene (klimaaktiv, konkrete Bauteil-Reuse-Programme) in diesem Aufschluss gar nicht aufgesucht.
- **ÖNORM B 2110** (Feld 7, Werkvertragsrecht) nur als Existenzhinweis, keine inhaltliche Prüfung.
- **Abfallverzeichnisverordnung** (Feld 3, Abfallschlüssel für Bau-/Abbruchabfälle) nicht erhoben.
- **AT-Vollzugspraxis/Erlasse zur CPR 2024/3110** (Feld 1, Marktüberwachung) nicht recherchiert — nur die allgemeine Zuständigkeitsstruktur aus dem Vorarlberger BauPG abgeleitet.
