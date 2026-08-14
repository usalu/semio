# Materialquerschnitt Stahlbeton/Fertigteile — Regelungsobjekte quer über EU + 10 Jurisdiktionen

**Zweck:** Diese Datei ergänzt den Länderdurchgang (`roh/*-F1-3.md`, `*-F4-7.md`, `*-alle.md`) um **materialspezifische** Regelwerke für Stahlbeton/Fertigteile, die im Länderdurchgang strukturell selten auftauchen, weil materialspezifische Normen (DAfStb, NEN, SIA, ÖNORM, CROW-CUR, DS, FDB, MPA, C2P) meist außerhalb der klassischen Baurechtsquellen (Gesetz/VO/LBO) liegen, die der Länderdurchgang priorisiert hat. Stichtag 2026-08-11, Recherche 2026-08-13.

**Methodikhinweis (Ehrlichkeit):** Das WebSearch-Kontingent dieser Session war beim Start dieser Recherche bereits erschöpft (200/200 durch vorherige Agenten in derselben Session verbraucht). Alle Primärfunde in dieser Datei stammen aus **WebFetch auf offizielle Domains** (dafstb.de, crow.nl, kennisbank.crow.nl, buildwise.be, ds.dk, sintef.no, mpaprecast.org, concretecentre.com, austrian-standards.at) sowie **WebFetch auf DuckDuckGo-HTML-Suchergebnisseiten** als Ersatz für Websuche. Wo die WebFetch-Zusammenfassung (ein internes Modell verarbeitet den Seiteninhalt, bevor er zurückgegeben wird) den Wortlaut wiedergibt, ist dies durchgehend als **B1/B2, nicht B0** gekennzeichnet — ich habe den rohen Seitenquelltext nicht selbst Zeichen für Zeichen geprüft. Mehrere Zielseiten (istructe.org, ds.dk-Shopseiten, sintef.no-Artikelseite) waren technisch nicht auslesbar (403/JS-Rendering/leere Extraktion) — dort ist die Lücke offen vermerkt, nicht stillschweigend übergangen.

**ID-Kollisionshinweis:** Da diese Datei parallel zum Länderdurchgang entstand und keine vollständige Sicht auf alle dort bereits vergebenen laufenden Nummern bestand, vergibt diese Datei **bewusst Nummern ab `-9xx`** je Jurisdiktion/Feld, um Kollisionen mit dem Länderdurchgang zu vermeiden. **An W4 zur Renummerierung bei der Synthese zu melden.**

**Querverweis auf bereits im Länderdurchgang erfasste, materialübergreifend/teilweise-Stahlbeton-relevante Objekte (nicht dupliziert, hier nur referenziert):**

| Land | Bereits erfasstes Objekt | Stahlbeton-Bezug | Status im Länderdurchgang |
|---|---|---|---|
| NL | REG-NL-2-003 (NEN 8700) | Bestandsbewertung materialübergreifend, aber Grundlagenteil für jede Stahlbeton-Bestandsberechnung | paywalled-nicht-eingesehen |
| NL | REG-NL-2-004 (NEN 8701) | Lastannahmen Bestand, materialübergreifend | paywalled-nicht-eingesehen, Fassungsstand unklar |
| NL | REG-NL-2-005 (NTA 8713) | **nur Baustahl**, nicht Beton — Lücke: kein Beton-Pendant im Länderdurchgang identifiziert | paywalled-nicht-eingesehen |
| CH | REG-CH-2-006/007, REG-CH-6-014 (SIA 260 ff., SIA 269 ff.) | SIA 269/2 „Betonbau" explizit benannt, aber nie eigenständig vertieft | B3, nur Sekundärquellen, Normtext nicht eingesehen |
| AT | REG-AT-2-004, REG-AT-6-002 (ÖNORM B 1990 ff.) | nennt B 1992 „Beton" ausdrücklich als **nicht einzeln erhoben** | B4, reiner Katalognachweis |
| AT | REG-AT-6-004 (ÖNORM B 4710-1) | Beton, aber **nur RC-Gesteinskörnung**, nicht Bauteil-Reuse | B0 Bindungsakt, B4 Norminhalt |
| DE | REG-DE-2-005/006/007 (Eurocode-NA, prEN 1990-2, ISO 13822) | materialübergreifend, keine Beton-Vertiefung | Zugriffslücke/paywalled |
| DE | REG-DE-6-Objekte (VDI 6200) | materialübergreifend Tragwerksüberprüfung | paywalled-nicht-eingesehen |
| EU | eu-normung.md (EN 1990-2, CEN/TS 17440) | materialübergreifend, EN 1992-2 (Beton-Vertiefung) nicht separat erhoben | paywalled-nicht-eingesehen |

Diese Datei füllt die dort ausdrücklich als Lücke benannten Beton-Vertiefungen so weit wie in der verfügbaren Zeit primärquellenbasiert möglich, **markiert aber ehrlich, wo auch diese Vertiefung an eine Paywall oder eine Zugriffssperre stößt**, statt eine Norm zu erfinden.

---

## EU/EEA

### REG-EU-1-901 · EN 13369 — Fertigteile aus Beton, Allgemeine Regeln (reuse-blinde hEN)
- Titel: EN 13369:2018 „Allgemeine Regeln für Betonfertigteile" (Precast concrete products — General rules)
- Fundstelle: Gesamtnorm (Scope-Abschnitt); Normtext selbst nicht eingesehen
- A: EU/EEA · Downstream-Verifikationsstatus: entfällt (unmittelbar wirksame hEN, keine Umsetzung durch Dritte)
- B: Primärfeld 1 Produkt-/Konformitätsrecht · Nebenfelder: 2 (Standsicherheit/Bemessungsgrundlagen für Fertigteile) · Normtyp: operative Norm
- C: Stahlbeton/Fertigteile
- D: hEN
- E: Inverkehrbringen
- F1 (E3): schweigend — die Norm gilt laut Sekundärbeschreibung für „unreinforced, reinforced and prestressed precast concrete products made of compact light-, normal- and heavyweight concrete"; kein Hinweis auf gebrauchte/wiederaufgearbeitete Fertigteile im Scope-Text gefunden, weder als Einschluss noch als expliziter Ausschluss
- F2 (E3): bedingend — **konkretisiert materialspezifisch den bereits generisch erfassten CPR-2024/3110-Befund (REG-EU-1-002, s. `eu-produkt.md`)**: Da EN 13369 selbst keine „harmonisierte technische Spezifikation mit Vorschriften für gebrauchte Produkte" (Art. 3 Nr. 20 lit. a CPR 2024/3110) enthält, fällt ein gebrauchtes Betonfertigteil zwingend unter den Tatbestand des Art. 26 Abs. 2 lit. b CPR 2024/3110 (gebrauchtes Produkt OHNE hEN-Gebrauchtregel) — die Herstellerfiktion trifft jeden Wirtschaftsteilnehmer, der ein wiederverwendetes, unter EN 13369 fallendes Fertigteil erneut in Verkehr bringt, ohne dass die hEN selbst eine Erleichterung vorsähe
- G: entfällt für den Scope-Teil (Anwendbarkeitsnorm ohne Nachweistatbestand); im Übrigen rechnerischer Nachweis + Erklärung Dritter (Leistungserklärung) — inferiert (E3), da Normvolltext nicht eingesehen
- Kernaussage: EN 13369 ist die horizontale harmonisierte Norm für Betonfertigteile unter der CPR und materieller Ankerpunkt für die CE-Kennzeichnung von Fertigteilen. Sie enthält nach den in dieser Session zugänglichen Sekundärbeschreibungen **keine eigene Gebraucht-Produkt-Regel**. Das bedeutet konkret für die Materialfamilie Stahlbeton: Der generische CPR-2024/3110-Befund, dass die meisten Produktfamilien mangels hEN-Gebrauchtregel unter die verschärfte Herstellerfiktion fallen (REG-EU-1-002), gilt für Betonfertigteile mit hoher Wahrscheinlichkeit ungemildert — vorbehaltlich einer künftigen produktspezifischen Durchführungsrechtsakt-Anpassung nach Art. 95 Abs. 9 CPR 2024/3110.
- Wortlautbeleg (Originalsprache): kein Wortlautzitat aus dem Normtext möglich (paywalled, nicht eingesehen); Sekundärbeschreibung (WebFetch-Zusammenfassung einer DuckDuckGo-Ergebnisseite): "unreinforced, reinforced and prestressed precast concrete products made of compact light-, normal- and heavyweight concrete according to EN 206"
- Beleg-Quelle: B3 (Sekundärbeschreibung des Scope über Suchergebnis-Zusammenfassung, kein Normtext eingesehen) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: CPR 2024/3110 selbst (hEN-Mechanismus, s. REG-EU-1-001 ff.) — Listung von EN 13369 im Amtsblatt als hEN nicht in dieser Session einzeln nachgeprüft
- Quelle: Tier 3 für Norminhalt (Suchmaschinen-Zusammenfassung, kein amtlicher Text) — für den Bindungsmechanismus Tier 1 (CPR 2024/3110, bereits in `eu-produkt.md` B0 belegt) · Zugriff 2026-08-13
- Status: in Kraft (2018-Fassung, laut Katalogangaben) · Datum: 2018
- Sub-Ebene: entfällt (A=EU/EEA)
- Relationen: konkretisiert REG-EU-1-002/003 (Herstellerfiktion/Scope-Abgrenzung CPR 2024/3110) materialspezifisch für Stahlbeton/Fertigteile; wird kombiniert mit REG-EU-2-00x (EN 1990-2, Bestandsbewertung) für den rechnerischen Nachweisteil
- Konfidenz: abgeleitet (Existenz und grober Scope gesichert über mehrere konvergente Sekundärquellen; Aussage zum Fehlen einer Gebraucht-Regel ist eine **Negativfeststellung aus Nichtfund**, keine amtliche Bestätigung „enthält keine Regel" — vor abschließender Verwendung in der Synthese sollte der Normvolltext oder zumindest das Amtsblatt-Verzeichnis nach Art. 17 CPR geprüft werden)

---

## Deutschland

### REG-DE-2-903 · DAfStb — kein identifiziertes Regelwerk zur Wiederverwendung von Stahlbetonbauteilen (Negativbefund)
- Titel: Deutscher Ausschuss für Stahlbeton (DAfStb) — Richtlinien- und Schriftenreihenbestand, geprüft auf Reuse-Bezug
- Fundstelle: nicht zutreffend (Negativbefund über mehrere gezielte Recherchen gegen dafstb.de sowie Websuchen)
- A: national · B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit · Normtyp: operative Norm (Negativbefund, kein Grundnorm-Flag)
- C: Stahlbeton/Fertigteile
- D: entfällt (kein Rechtsakt identifiziert, daher keine D-Einstufung; wäre im Trefferfall Techn. Regel mit Vermutungswirkung oder Merkblatt gewesen)
- E: Bestandserkundung, Planung/Nachweis
- F1 (E3): schweigend — trotz gezielter Recherche (Richtlinienübersicht dafstb.de/richtlinien.html, Schriftenreihe dafstb.de/schriftenreihe.html, mehrere Suchanfragen zu „DAfStb Wiederverwendung Betonfertigteile", „DAfStb Positionspapier Kreislaufwirtschaft Beton", „DAfStb Ausschuss Kreislaufgerechtes Bauen") wurde **keine DAfStb-Richtlinie oder DAfStb-Positionspapier identifiziert, die/das die Wiederverwendung von Stahlbetonbauteilen aus dem Bestand als eigenen Regelungsgegenstand behandelt**. Die DAfStb-eigene Klimaneutralitäts-Roadmap (Ziel: klimaneutraler Betonbau bis 2045) wurde als Treffer identifiziert, adressiert aber laut Suchergebnis-Zusammenfassung CO₂-Reduktion allgemein, nicht die bautechnische Zulassung wiederverwendeter Bauteile im Speziellen
- F2 (E3): schweigend/hemmend — die Regelungslücke auf Ausschussebene korrespondiert mit einem 2025 identifizierten wissenschaftlichen Befund (s. u.), dass ein strukturierter Bewertungs-/Klassifizierungsrahmen für die Wiederverwendung von Stahlbetonbauteilen in Deutschland erst im Aufbau ist, nicht bereits kodifiziert vorliegt — praktische Reuse-Vorhaben müssen sich mangels DAfStb-Richtlinie auf die generischen, materialübergreifend gehaltenen Instrumente ISO 13822/prEN 1990-2 (REG-DE-2-006/007) sowie auf die Einzelfallzulassung (ZiE/vBG, REG-DE-2-002) stützen
- G: entfällt (kein Nachweistatbestand, da keine Norm identifiziert)
- Kernaussage: Für Deutschland ließ sich in dieser Recherche **keine eigenständige DAfStb-Richtlinie zur Wiederverwendung von Stahlbetonbauteilen** primärquellenbasiert nachweisen. Das ist ein ehrlich zu markierender Negativbefund, kein Nichtprüfungs-Vermerk: Die DAfStb-Richtlinienübersicht wurde direkt abgerufen und enthielt keinen Treffer; ergänzende gezielte Suchen (drei unabhängige Anfragen) fanden ebenfalls keinen Treffer. Ein 2025 in „Beton- und Stahlbetonbau" (Ernst & Sohn) erschienener Fachaufsatz von Mecka, Geng, Schubert, Bos, Nübel und Fischer (TU München u. a.), „Ein Handlungsrahmen für die Wiederverwendung von Stahlbetonbauteilen" (Jg. 120, Heft 10, 2025), entwickelt selbst erst „einen strukturierten Handlungsrahmen für die Bewertung, Klassifizierung und Wiederverwendung tragender Bauteile" mit Fokus auf „nicht rückbauoptimierte Ortbetonbauteile" — das bestätigt indirekt, dass ein solcher Rahmen zum Stichtag als Forschungsdesiderat, nicht als geltendes Regelwerk existiert.
- Wortlautbeleg (Originalsprache): Abstract-Paraphrase (WebFetch-Zusammenfassung, Volltext hinter Wiley-Paywall, HTTP 402 beim direkten Zugriffsversuch): "Die Wiederverwendung von Stahlbetonbauteilen aus dem Gebäudebestand bietet ein erhebliches Potenzial zur Ressourcenschonung und Emissionsreduktion" / "einen strukturierten Handlungsrahmen für die Bewertung, Klassifizierung und Wiederverwendung tragender Bauteile"
- Beleg-Quelle: B1 für den Negativbefund (dafstb.de/richtlinien.html direkt abgerufen und gelesen) · B3 für den Fachaufsatz (Abstract nur über Suchmaschinen-Zusammenfassung, Volltext nicht zugänglich — HTTP 402 Payment Required bei direktem Zugriffsversuch auf onlinelibrary.wiley.com) · Zugänglichkeit: frei-primär (Richtlinienübersicht) / paywalled-nicht-eingesehen (Fachaufsatz) · Bindungsakt: entfällt/kein Bindungsakt identifiziert
- Quelle: Tier 1 (DAfStb-Übersichtsseite) https://www.dafstb.de/richtlinien.html · Tier 2 (peer-reviewed Fachzeitschrift, nicht im Volltext eingesehen) Mecka et al., Beton- und Stahlbetonbau 120(10), 2025, Ernst & Sohn, DOI 10.1002/best.70015 · Zugriff 2026-08-13
- Status: keine Norm/kein Regelwerk in Kraft (Negativbefund); Forschungsstand laut Fachaufsatz: 2025, laufend
- Sub-Ebene: entfällt (A=national)
- Relationen: schließt die im Länderdurchgang bereits offen benannte Lücke REG-DE-2-005/006/007 (Eurocode-NA/prEN 1990-2/ISO 13822, alle materialübergreifend) materialspezifisch **nicht** — bestätigt sie vielmehr als bislang einzige verfügbare (materialübergreifende) Rechtsgrundlage für Stahlbeton-Bestandsbewertung in DE
- Konfidenz: abgeleitet (Negativbefund aus mehreren konvergenten erfolglosen gezielten Suchen, keine amtliche „existiert nicht"-Bestätigung — methodisch identisch zur bereits im Länderdurchgang akzeptierten Konvention, s. REG-NL-1-003)

### REG-DE-6-902 · FDB-Merkblatt Nr. 10 — Nachhaltiges Bauen mit Betonfertigteilen (Rückbau/Wiederverwendung als Branchenempfehlung)
- Titel: FDB-Merkblatt Nr. 10, „Nachhaltiges Bauen mit Betonfertigteilen", Fachvereinigung Deutscher Betonfertigteilbau (FDB), Ausgabe 01/2025
- Fundstelle: Gesamtmerkblatt (10 Seiten); PDF eingesehen, Text im PDF-Binärformat nicht zuverlässig zeichengenau extrahierbar (s. Beleg-Quelle)
- A: national · Downstream-Verifikationsstatus: entfällt (kein Muster-/Bund-Länder-Dokument, eigenständige Verbandsveröffentlichung)
- B: Primärfeld 6 Normen/Regelwerke · Nebenfelder: 2 (Standsicherheit/Rückbau-/Wiederverwendungsplanung) · Normtyp: operative Norm
- C: Stahlbeton/Fertigteile
- D: Branchenprotokoll
- E: Rückbau/Sicherung, Planung/Nachweis
- F1 (E3): ermöglichend — als bislang einzige in dieser Recherche identifizierte deutschsprachige, betonfertigteilspezifische Verbandsveröffentlichung, die Rückbaufähigkeit und Wiederverwendung explizit als Planungskriterium einführt (laut Übersichtsangaben: „Ein nachhaltiges Gebäude sollte nutzungsflexibel, technisch hochwertig sowie rückbau- und wiederverwendungsfähig sein"), schließt sie eine Lücke, die weder DAfStb (REG-DE-2-903, Negativbefund) noch die Landesbauordnungen (REG-DE-2-001/002, s. `DE-F1-3.md`) materialspezifisch füllen
- F2 (E3): bedingend — als Merkblatt eines Herstellerverbands ohne Normcharakter (kein DIN, keine VV-TB-Listung identifiziert) entfaltet es keine bauaufsichtliche Bindungswirkung; die praktische Wirkung hängt von freiwilliger Anwendung durch Fertigteilhersteller/-planer ab, ist aber die konkreteste verfügbare fachliche Anleitung für die Praxis
- G: Dokumentenlage — inferiert (E3, aus Themenübersicht „Rückbau- und Wiederverwendungserwägungen", „Lebenszyklusphasen von Beton"); kein expliziter Nachweiskatalog im zugänglichen Textauszug verifiziert
- Kernaussage: Das FDB-Merkblatt Nr. 10 behandelt nachhaltiges Bauen mit Betonfertigteilen einschließlich der Lebenszyklusphasen von der Rohstoffgewinnung bis zum Rückbau und thematisiert Rückbau- und Wiederverwendungsfähigkeit als Planungsziel — mit früher Einbindung von Fachplanern und Herstellern als empfohlener Praxis. Es ist eine freiwillige Branchenempfehlung des Herstellerverbands FDB, kein amtliches oder normatives Regelwerk, und ersetzt keinen rechnerischen Standsicherheitsnachweis.
- Wortlautbeleg (Originalsprache): WebFetch-Zusammenfassung des PDF-Inhalts (Volltext binär nicht zuverlässig extrahierbar, daher Paraphrase statt direktem Zitat): "Ein nachhaltiges Gebäude sollte nutzungsflexibel, technisch hochwertig sowie rückbau- und wiederverwendungsfähig sein" — **als Paraphrase, nicht als geprüftes Wort-für-Wort-Zitat zu behandeln**
- Beleg-Quelle: B2 (PDF direkt abgerufen, Inhalt jedoch nur über automatisierte Zusammenfassung zugänglich — Binärstream ließ sich nicht zeichengenau auslesen, s. Werkzeug-Rückmeldung) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (privates Branchenprotokoll, kein Bindungsakt zu erwarten)
- Quelle: Tier 3 (Herstellerverband, keine Behörde/kein Normungsgremium) · https://www.fdb-fertigteilbau.de/fdb-angebote/literatur-downloadcenter-merkblaetter/fdb-merkblaetter/merkblatt-nr-10/ ; PDF-Spiegelung https://www.solid-unit.de/wp-content/uploads/2025/01/FDB-Merkblatt-Nr-10-nachhaltige-Bauen-mit-Betonfertigteilen-2025-01.pdf · Fassung 01/2025 · Zugriff 2026-08-13
- Status: in Kraft (aktuellste Ausgabe 01/2025)
- Sub-Ebene: entfällt (A=national)
- Relationen: wird kombiniert mit/ergänzt REG-DE-2-903 (DAfStb-Negativbefund) — füllt die dortige Lücke praktisch, nicht normativ; konkretisiert nicht REG-DE-2-001/002 (ZiE/vBG bleiben der maßgebliche Rechtsweg)
- Konfidenz: abgeleitet (Existenz/Titel/Datum gesichert; Wortlautgenauigkeit eingeschränkt, da PDF-Volltext technisch nicht zeichengenau auslesbar war)

---

## Österreich

### REG-AT-2-902 · ÖNORM B 1992-1-1 — Eurocode 2, Nationale Festlegungen (Beton-Vertiefung der bereits erfassten B-1990-Lücke)
- Titel: ÖNORM B 1992-1-1, Eurocode 2: Bemessung und Konstruktion von Stahlbeton- und Spannbetontragwerken — Nationale Festlegungen zu ÖNORM EN 1992-1-1
- Fundstelle: Gesamtnorm (Ausgaben 2007-01-02 und 2018-01-01 identifiziert; Normtext nicht eingesehen)
- A: sub-national (Bindung ausschließlich über Landesakt, analog REG-AT-2-004) · Downstream-Verifikationsstatus: strukturell angenommen, nicht verifiziert
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit · Normtyp: operative Norm
- C: Stahlbeton/Fertigteile
- D: nat.Norm
- E: Planung/Nachweis
- F1 (E3): bedingend — **materialspezifische Instanziierung der bereits in `AT-F1-3.md` (REG-AT-2-004) benannten Lücke** „B 1992 Beton … nicht einzeln erhoben": Diese Norm ist die konkrete Rechtsgrundlage für den rechnerischen Nachweis von Stahlbeton-/Spannbetontragwerken einschließlich Bestandsbauteilen, an der jeder AT-Wiederverwendungsnachweis für Betonbauteile methodisch hängt; kein Reuse-spezifischer Bezug im Titel/Scope erkennbar
- F2 (E3): hemmend — wie REG-AT-2-004 strukturell analog: Kostenpflicht bei Austrian Standards als Zugangshürde, hier zusätzlich verschärft durch die Notwendigkeit, sowohl B 1990 (Grundlagen) als auch B 1992 (Beton-Vertiefung) zu beziehen
- G: rechnerischer Nachweis — inferiert (E3); Normtext nicht eingesehen
- Kernaussage: ÖNORM B 1992-1-1 ergänzt die generische Eurocode-Grundlagennorm B 1990 (REG-AT-2-004) um die betonspezifischen Bemessungsregeln und liegt in mindestens zwei Ausgaben vor (2007, 2018). Sie ist die materialspezifische Rechtsgrundlage, die im Länderdurchgang ausdrücklich als „nicht einzeln erhoben" offen gelassen wurde. Ein eigenständiger Reuse-/Bestandsbauteil-Bezug (über die allgemeine Bestandsbewertungssystematik hinaus) wurde in dieser Session nicht verifiziert.
- Wortlautbeleg (Originalsprache): Sekundärzitat (WebFetch-Zusammenfassung einer Suchergebnisseite, nicht am Normtext selbst geprüft): "Diese ÖNORM gilt für den Entwurf, die Berechnung und die Bemessung von Hoch- und Ingenieurbauten aus Beton, Stahlbeton und Spannbeton."
- Beleg-Quelle: B3 (Sekundärbeschreibung über Suchmaschinen-Zusammenfassung; Normtext und Bindungsakt-Detailprüfung nicht durchgeführt) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: OIB-Richtlinie 1 → Landes-Bautechnikverordnung, Mechanismus strukturell identisch zu REG-AT-2-004 angenommen, für B 1992 selbst nicht gesondert verifiziert
- Quelle: Tier 3 für Norminhalt (Kataloge austrian-standards.at, bdb.at, schuberth.at — als Existenznachweis, nicht als Normtext-Beleg) · Zugriff 2026-08-13
- Status: in Kraft (Ausgabe 2018-01-01, ältere Ausgabe 2007-01-02 im Katalog ebenfalls gelistet, Ablösungsstatus nicht verifiziert)
- Sub-Ebene: Stichprobe [nicht durchgeführt — Recherche blieb auf Bundesebene/ÖNORM-Katalog beschränkt] / nicht erhoben [alle 9 Bundesländer bzgl. konkreter Bautechnikverordnungs-Referenzierung]
- Relationen: konkretisiert REG-AT-2-004 (B 1990 Grundlagennorm) materialspezifisch für Stahlbeton; wird kombiniert mit REG-AT-6-004 (B 4710-1, RC-Baustoffe im Beton) für den Werkstoffnachweisteil
- Konfidenz: abgeleitet (Existenz/Titel/Ausgaben gesichert über mehrere konvergente Kataloghinweise; Bindungskette und Norminhalt nicht eigenständig verifiziert)

---

## Schweiz

### REG-CH-2-902 · SIA 269/2 — Erhaltung von Tragwerken, Betonbau (materialspezifische Vertiefung von REG-CH-2-007/REG-CH-6-014)
- Titel: SIA 269/2:2011 (Ausgabe 2011, in dieser Recherche keine neuere Ausgabe verifiziert — Hinweis auf 269/1 „Einwirkungen" und 269/3 „Stahlbau" als Folgenormen bereits im Länderdurchgang vermerkt), „Erhaltung von Tragwerken — Betonbau"
- Fundstelle: Gesamtnorm (Normtext kostenpflichtig, in dieser Session nicht eingesehen)
- A: national (Bindung primär zivilrechtlich/vertraglich, nicht kantonal, wie bereits für die 269-Reihe insgesamt in `CH-F1-3.md` festgehalten) · Downstream-Verifikationsstatus: nicht geprüft
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit · Normtyp: operative Norm
- C: Stahlbeton/Fertigteile
- D: nat.Norm
- E: Bestandserkundung, Planung/Nachweis
- F1 (E3): ermöglichend — laut Sekundärbeschreibung (Fachaufsatz-Zusammenfassung) beruhen „Erhaltungsmaßnahmen für Betonbauwerke … auf wesentlichen Mängeln im Tragsystem, Schäden und Schädigungsmechanismen, die Beton, Betonstahl und Spannstahl betreffen" — das liefert erstmals eine materialspezifische Schadens-/Mängel-Systematik für die Bestandsbewertung von Stahlbetontragwerken, Voraussetzung für einen belastbaren Wiederverwendungsnachweis über die reine Grundlagennorm SIA 269 hinaus
- F2 (E3): bedingend — laut Sekundärquelle war die Norm ab Anfang 2011 für bestimmte Anwendungen verbindlich vorgeschrieben („became mandatory for certain applications starting in early 2011"); welcher Bindungsmechanismus (kantonal? vertraglich über SIA-Empfehlungscharakter?) dahintersteht, wurde in dieser Session **nicht** verifiziert — Konfidenzabschlag
- G: zerstörungsfreie Prüfung, Probenahme/Materialprüfung, rechnerischer Nachweis — inferiert (E3, aus der Schadensmechanismen-Systematik abgeleitet, Normtext nicht eingesehen)
- Kernaussage: SIA 269/2 ist der betonspezifische Teilband der SIA-269-Reihe zur Erhaltung von Tragwerken und liefert eine auf Beton-/Betonstahl-/Spannstahl-Schädigungsmechanismen zugeschnittene Bewertungssystematik. Sie vertieft die bereits im Länderdurchgang (REG-CH-2-007, REG-CH-6-014) als Ganzes erfasste 269-Reihe materialspezifisch, konnte aber auch in dieser Runde **nicht im Normvolltext** geprüft werden — die Kernaussage stützt sich weiterhin ausschließlich auf Sekundärbeschreibungen.
- Wortlautbeleg (Originalsprache): kein Original-Wortlaut verfügbar; WebFetch-Paraphrase einer englischsprachigen Sekundärbeschreibung: "maintenance measures for concrete structures are based on significant defects in the load-bearing system, damage, and damage mechanisms affecting concrete, reinforcing steel, and prestressing steel" — **Rückübersetzung, kein Original-Deutsch/Französisch/Italienisch-Zitat**, daher nicht als belastbarer Wortlautbeleg im engeren Sinn zu werten
- Beleg-Quelle: B3 (wie REG-CH-6-014: nur Sekundärbeschreibung, kein Normtext) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: entfällt/kein Bindungsakt identifiziert (analog REG-CH-6-014 — Primärtextbeschaffung an W4/W2 verwiesen)
- Quelle: Tier 3 (Sekundärbeschreibung, kein amtlicher Erlasstext) · Normverkaufsseite (nicht geöffnet) shop.sia.ch · Fassung 2011 · Zugriff 2026-08-13
- Status: in Kraft (2011-Fassung; SIA 262 als Neubau-Pendant für Betonbau existiert nach Kenntnisstand des Rechercheteams, wurde in dieser Session **nicht** eigenständig verifiziert — offene Lücke, nicht erfunden)
- Sub-Ebene: entfällt (Bindung laut Länderdurchgang nicht primär kantonal)
- Relationen: konkretisiert REG-CH-2-007/REG-CH-6-014 (SIA-269-Reihe) materialspezifisch für Stahlbeton
- Konfidenz: unklar (identisch zur bereits im Länderdurchgang für REG-CH-6-014 vergebenen Konfidenzeinstufung — Primärtextzugriff bleibt die zentrale Lücke)

---

## Niederlande

### REG-NL-2-901 · CROW-CUR Richtlijn 4:2023 — Hergebruik constructieve prefab betonelementen
- Titel: CROW-CUR Richtlijn 4:2023, „Hergebruik constructieve prefab betonelementen"
- Fundstelle: Gesamtrichtlinie; Doel-Abschnitt direkt auf der CROW-Kennisbank-Seite eingesehen
- A: national · Downstream-Verifikationsstatus: entfällt (keine Landes-/Gemeindeebene involviert)
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit · Normtyp: operative Norm
- C: Stahlbeton/Fertigteile
- D: Merkblatt — **die Richtlinie klassifiziert sich selbst ausdrücklich nicht als Norm**: "faciliterende richtlijn en niet als een protocol, aanbeveling of norm" — insofern liegt die Einordnung als Merkblatt (eigenständige fachliche Empfehlung ohne Transformationszweck) näher als „nat.Norm", obwohl der Titel „CUR Richtlijn" das Wort „Richtlijn" trägt; dies ist der bislang klarste in diesem Projekt gefundene Fall einer Norm-artig benannten, aber selbst-deklariert nicht-normativen Quelle
- E: Bestandserkundung, Aufbereitung/Prüfung, Planung/Nachweis, Einbau/Abnahme
- F1 (E3): ermöglichend — die bislang konkreteste, materialspezifisch auf Beton-Fertigteil-Wiederverwendung zugeschnittene Quelle in der gesamten NL-Erhebung (ergänzt REG-NL-2-003/004, die nur die materialübergreifende Bestandsbewertungssystematik NEN 8700/8701 liefern); Zielgruppe ausdrücklich der gesamte Prozess von Architekten über Ingenieure, Behörden, Bauunternehmer, Facility Manager, Abbruchunternehmen bis Elementlieferanten; behandelt in Anhängen konkret Kanaalplaten (Kammerplatten) und vorgespannte Brückenträger
- F2 (E3): bedingend — der ausdrückliche Selbstverzicht auf Protokoll-/Norm-/Empfehlungscharakter bedeutet, dass die Richtlinie keine eigene bauaufsichtliche Bindungswirkung entfaltet; sie „schafft mehr Vertrauen in der Kette bzgl. Qualitätssicherung beim Hergebruik von Fertigteilen" (Selbstzweck laut Doel-Text), ersetzt aber nicht den Verwendbarkeitsnachweis nach Bbl Art. 5.4/5.5 (REG-NL-2-001/002) oder eine NEN-8700-gestützte Berechnung
- G: Dokumentenlage, zerstörungsfreie Prüfung, Probenahme/Materialprüfung, rechnerischer Nachweis — inferiert (E3, aus dem Regelungsgegenstand „kaders voor materiaalgebruik en het hele proces van voorbereiding tot en met uitvoering" abgeleitet; kein Einzelnachweis im ausgelesenen Textauszug explizit benannt)
- Kernaussage: CUR-Richtlijn 4:2023 (veröffentlicht 04.01.2024, letzte Änderung 11.02.2024) ist die erste in diesem Projekt identifizierte, dediziert betonfertigteilspezifische Wiederverwendungsrichtlinie irgendeiner Jurisdiktion. Sie deckt den vollständigen Prozess von Vorbereitung bis Ausführung ab, mit besonderer Vertiefung zu Kanaalplaten und vorgespannten Brückenträgern, und positioniert sich selbst ausdrücklich als unverbindliche, facilitierende Richtlinie ohne Norm-, Protokoll- oder Empfehlungscharakter.
- Wortlautbeleg (Originalsprache): "meer vertrouwen te creëren in de keten met betrekking tot kwaliteitsborging bij hergebruik van prefab betonnen elementen" / "faciliterende richtlijn en niet als een protocol, aanbeveling of norm" / "kaders voor materiaalgebruik en het hele proces van voorbereiding tot en met uitvoering" / "rekening houdend met de specifieke omstandigheden van hun projecten"
- Beleg-Quelle: B1 (Doel-Seite der CROW-Kennisbank direkt via WebFetch gelesen, Kernformulierungen als Zitat wiedergegeben; vollständiger Richtlinientext selbst — inkl. der Anhänge zu Kanaalplaten/Brückenträgern — nicht eingesehen, nur die öffentlich zugängliche Doel-Beschreibung) · Zugänglichkeit: frei-primär (Doel-Seite) / paywalled-oder-registrierungspflichtig (Volltext, laut Fundstelle „na account­aanmaak") · Bindungsakt: entfällt/kein Bindungsakt identifiziert — kein Verweis auf Bbl/Bal in der eingesehenen Doel-Seite gefunden
- Quelle: Tier 1 (CROW-CUR ist die anerkannte niederländische technische Richtlinienorganisation für Bau-/Infrastrukturpraxis, vergleichbar einer Selbstverwaltungskörperschaft, kein privater Verlag) · https://www.crow.nl/actueel/richtlijn-voor-hergebruik-van-constructieve-prefab/ ; https://kennisbank.crow.nl/public/BECON/CROW-CUR_Richtlijn_4_2023_Hergebruik_constructieve_prefab_betonelementen/Doel/119636 · Fassung 2023 (publiziert 2024-01-04, geändert 2024-02-11) · Zugriff 2026-08-13
- Status: in Kraft (aktuellste identifizierte Fassung)
- Sub-Ebene: entfällt (A=national)
- Relationen: wird kombiniert mit/ergänzt REG-NL-2-001/002 (Bbl Art. 5.4/5.5, Bestandsschutz) und REG-NL-2-003/004 (NEN 8700/8701, Bestandsbewertung materialübergreifend); konkretisiert für Beton, was NTA 8713 (REG-NL-2-005) für Baustahl leistet — **Lückenschluss zur im Länderdurchgang selbst benannten Beton-Parallellücke**
- Konfidenz: gesichert (Existenz, Titel, Selbstklassifikation als Nicht-Norm, Zielgruppe und Themenschwerpunkte über Primärquelle B1 belegt); abgeleitet (technische Detailanforderungen, da Volltext nicht zugänglich)

---

## Belgien (Flandern-Schwerpunkt, konsistent mit BE-VL.md)

### REG-BE-2-901 · Buildwise/UHasselt-Projekt „ReCon" — Structureel hergebruik van beton (Forschungsprojekt, kein geltendes Regelwerk)
- Titel: ReCon — „Structureel hergebruik van beton", Forschungsprojekt von Buildwise (vormals WTCB/CSTC) mit UHasselt, gefördert durch VLAIO
- Fundstelle: Projektbeschreibungsseite buildwise.be
- A: sub-national (Flandern — VLAIO ist ein flämisches Förderinstrument; Buildwise agiert föderal, das Projekt selbst ist über VLAIO flämisch verortet) · Downstream-Verifikationsstatus: nicht geprüft
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit · Normtyp: operative Norm (kein Grundnorm-Flag — reines Forschungsprojekt, noch keine Norm)
- C: Stahlbeton/Fertigteile
- D: außerhalb des D-Vokabulars — laufendes Forschungsprojekt ohne Veröffentlichungscharakter, **kein** Merkblatt/Norm/Rechtsakt (an W4 zur Einordnung zu melden, analog dem bereits im Länderdurchgang für DK-Realdania-Förderprogramm dokumentierten Fall)
- E: Bestandserkundung, Aufbereitung/Prüfung, Planung/Nachweis
- F1 (E3): schweigend (noch kein Regelungstext, da Forschungsprojekt) · Bezugsgegenstand: Rechtslage zum Stichtag 2026-08-11 — **Wirksamkeitsbedingung: Projektlaufzeit 2026-10-01 bis 2028-09-30, also zum Stichtag noch nicht begonnen**
- F2 (E3): ermöglichend (perspektivisch) — das Projekt entwickelt laut Zielbeschreibung „een geïntegreerd kader voor de beoordeling en het hergebruik van betonnen constructie-elementen" mit „kaders voor inspectie en inventarisatie", „berekeningsmethodes voor restcapaciteit en duurzaamheid" sowie Protokollen für Demontage, Lagerung und Wiederaufbereitung, Kosten-Nutzen-Analysen und Haftungsempfehlungen — sobald abgeschlossen, potenziell der erste dedizierte belgische Bewertungsrahmen für Beton-Bauteil-Reuse
- G: entfällt (kein Nachweistatbestand vor Projektabschluss)
- Kernaussage: ReCon ist ein von Buildwise und der Universität Hasselt getragenes, VLAIO-gefördertes Forschungsprojekt (Laufzeit 01.10.2026–30.09.2028) zur Entwicklung eines integrierten Bewertungs- und Wiederverwendungsrahmens für Betonbauteile, mit Fokus auf vorgefertigte horizontale Elemente (Balken, Platten). Zum Stichtag 2026-08-11 existiert **noch kein** anwendbares Ergebnis — die Projektlaufzeit beginnt erst am 01.10.2026. Dies ist als Entwicklungslinie, nicht als geltendes Regelwerk zu führen.
- Wortlautbeleg (Originalsprache): "een geïntegreerd kader voor de beoordeling en het hergebruik van betonnen constructie-elementen" / "kaders voor inspectie en inventarisatie" / "berekeningsmethodes voor restcapaciteit en duurzaamheid"
- Beleg-Quelle: B1 (Projektseite direkt via WebFetch gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Forschungsprojekt, kein Rechtsakt)
- Quelle: Tier 1 (Buildwise ist die belgische föderale Bauforschungsinstitution, vergleichbar einer Behörde/Selbstverwaltungskörperschaft) · https://www.buildwise.be/nl/onderzoek-innovatie/onderzoeksprojecten/structureel-hergebruik-van-beton-recon/ · Fassung Projektstand 2026-08-13 · Zugriff 2026-08-13
- Status: Entwurf/in Vorbereitung · Laufzeit 2026-10-01 bis 2028-09-30
- Sub-Ebene: Stichprobe [Flandern — VLAIO-Förderung] / nicht erhoben [Wallonie, Region Brüssel-Hauptstadt bzgl. paralleler Beton-Reuse-Forschung]
- Relationen: wird kombiniert mit/ergänzt die bereits in `BE-VL.md` erfasste Sloopattest-/Traceability-Systematik (REG-BE-3-0xx), sobald abgeschlossen; kein Umsetzungsverhältnis zu bestehendem Regelungsobjekt, da selbst noch kein Regelungsobjekt im engeren Sinn
- Konfidenz: gesichert (Projektexistenz, Träger, Laufzeit, Zielbeschreibung über Primärquelle belegt); nicht anwendbar für materielle Inhalte, da Projekt zum Stichtag noch nicht begonnen

---

## Frankreich

### REG-FR-7-901 · C2P/AQC — Erstes französisches Reuse-Referenzwerk gilt für Baustahl, NICHT für Stahlbeton (Asymmetrie-Befund)
- Titel: „Recommandations professionnelles — Réemploi d'éléments structuraux en acier" (CTICM/Syndicat de la Construction Métallique de France), akzeptiert durch die Commission Prévention Produits (C2P) der Agence Qualité Construction (AQC)
- Fundstelle: C2P-Akzeptanzentscheidung, Anfang Januar 2024; „Liste Verte" der C2P (liste-verte-c2p.qualiteconstruction.com)
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 7 Haftung/Gewährleistung (Versicherbarkeit) · Nebenfelder: 2 (Standsicherheit) · Normtyp: Grundnorm/Begriffsnorm-nah (determiniert, ob eine Reuse-Bauweise als „technique courante" versicherbar ist)
- C: Baustahl (das anerkannte Dokument selbst) — **Stahlbeton/Fertigteile ausdrücklich NICHT erfasst, s. u.**
- D: Merkblatt (Branchenempfehlung mit C2P-Anerkennung — kein Gesetz/keine Norm, aber mit realer Versicherungsrechtsfolge über die „pratiques courantes"-Klassifikation)
- E: Planung/Nachweis, Betrieb/Dokumentation (Versicherungsdeckung)
- F1 (E3): schweigend für Stahlbeton — **zentraler Asymmetrie-Befund dieser Materialrecherche**: Die gezielte Suche nach „C2P premier référentiel réemploi" ergab, dass das erste und bislang einzige von der C2P akzeptierte sektorübergreifende Reuse-Referenzwerk ("le premier sur le réemploi en France, toutes filières confondues") sich **ausschließlich auf Baustahl** bezieht ("Recommandations professionnelles — Réemploi d'éléments structuraux en acier"). Für Stahlbeton/Fertigteile wurde **kein** analoges C2P-akzeptiertes Dokument identifiziert
- F2 (E3): hemmend für Stahlbeton — laut Sekundärbeschreibung ermöglicht die C2P-Anerkennung für Baustahl "l'assurabilité automatique des produits métalliques réemployés" durch Gleichstellung mit "les mêmes propriétés essentielles que leur équivalent neuf"; ohne ein äquivalentes Dokument für Beton bleibt die Wiederverwendung von Stahlbetonbauteilen in Frankreich versicherungsrechtlich auf den allgemeinen, einzelfallbezogenen ATEx-/Avis-Technique-Weg verwiesen — strukturell teurer und langsamer als der jetzt für Stahl eröffnete Listenweg
- G: Erklärung Dritter (C2P-Klassifikation als „technique courante") — explizit (E1) für Baustahl; für Stahlbeton: Einzelfallzulassung (ATEx/Avis Technique) — inferiert (E3), da kein spezifisches Dokument gefunden
- Kernaussage: Die C2P/AQC akzeptierte Anfang Januar 2024 das erste sektorweite französische Reuse-Referenzwerk, das reused Baustahlelemente strukturell mit Neuprodukten gleichstellt und damit automatisch versicherbar macht. Ein analoges Dokument für Stahlbeton/Fertigteile existiert nach dieser Recherche **nicht** — die Materialfamilien Baustahl und Stahlbeton sind in Frankreich versicherungsrechtlich zum Stichtag 2026-08-11 **ungleich behandelt**: Baustahl-Reuse hat einen anerkannten Listenweg, Stahlbeton-Reuse bleibt auf Einzelfallverfahren verwiesen. Dieser Befund ist wörtlich für die Materialfamilie Stahlbeton als **F1=schweigend, hemmende Kehrseite eines für eine andere Materialfamilie bereits gelösten Problems** zu kodieren.
- Wortlautbeleg (Originalsprache): "le premier sur le réemploi en France, toutes filières confondues" / "techniques courantes par les assureurs sont celles qui ont été acceptées par la Commission Prévention Produit" — beide als WebFetch-Zusammenfassung einer Suchergebnisseite, nicht am C2P-Originaldokument selbst geprüft
- Beleg-Quelle: B3 (Sekundärbeschreibung über Suchmaschinen-Zusammenfassung, C2P-Originaldokument bzw. die „Liste Verte" selbst nicht direkt aufgerufen) · Zugänglichkeit: frei-primär (Liste Verte grundsätzlich frei zugänglich unter liste-verte-c2p.qualiteconstruction.com, in dieser Session nicht direkt geöffnet) · Bindungsakt: entfällt (privatrechtliche Versicherungspraxis-Klassifikation, kein staatlicher Bindungsakt; die Rechtsfolge wirkt über Art. 1792 ff. Code civil/Assurance construction, in dieser Session nicht vertieft)
- Quelle: Tier 3 für den Wortlaut (Suchmaschinen-Zusammenfassung) · genannte Primärquellen laut Suchergebnis: constructionmetallique.fr, qualiteconstruction.com, cticm.com · Zugriff 2026-08-13
- Status: in Kraft (Baustahl-Dokument, seit Januar 2024); für Stahlbeton: kein Status, da kein Dokument existiert
- Sub-Ebene: entfällt (A=national)
- Relationen: **kollidiert mit** keiner bestehenden Beton-Regelung (da keine existiert) — steht in Spannungsverhältnis zur bereits im Länderdurchgang für FR erfassten Feld-7-Systematik (`FR-F4-7.md`, nicht hier dupliziert); Negativ-Analogieverhältnis zu REG-NL-2-901 (NL hat für Beton bereits eine dedizierte, wenn auch unverbindliche Richtlinie, FR noch nicht)
- Konfidenz: abgeleitet (Existenz/Fokus des Baustahl-Dokuments über konvergente Sekundärquellen plausibel; die Negativfeststellung „kein Beton-Pendant" ist eine Nichtfund-Aussage aus einer einzigen gezielten Suchrunde, sollte vor Synthese durch direkten Abruf der „Liste Verte" gegengeprüft werden)

---

## Dänemark

### REG-DK-6-901 · DS 11990:2024 — Vurdering af bæreevnen i eksisterende konstruktioner (technische Grundlage für Beton-Reuse-Projekte)
- Titel: DS 11990:2024, „[Standard] for vurdering af bæreevnen i eksisterende konstruktioner" (Bewertung der Tragfähigkeit bestehender Konstruktionen)
- Fundstelle: Gesamtnorm; Normtext selbst nicht eingesehen (Shopseite technisch nicht auslesbar, s. Beleg-Quelle)
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit · Normtyp: operative Norm
- C: materialübergreifend (Norm selbst nach Titel materialneutral) · **E3-Projektzuordnung zu Stahlbeton**, da die Norm laut (P)RECAST-Projekt explizit als „faglig ramme" (fachlicher Rahmen) zur Bewertung der Wiederverwendbarkeit von Betonfertigteilen (Volumen, Einheitlichkeit, Dokumentation) herangezogen wird
- D: nat.Norm
- E: Bestandserkundung, Planung/Nachweis
- F1 (E3): ermöglichend — als 2024 erschienene, aktuelle dänische Norm zur Bestandstragfähigkeitsbewertung ist DS 11990:2024 das unmittelbare dänische Pendant zu ISO 13822/prEN 1990-2 (DE, REG-DE-2-006/007) bzw. NEN 8700 (NL, REG-NL-2-003) — und wird in Dänemark bereits **aktiv als Werkzeug für Beton-Bauteil-Reuse-Projekte** eingesetzt, nicht nur abstrakt zitiert
- F2 (E3): ermöglichend — die Verwendung als „faglig ramme" im mehrjährigen, von 13 Branchenakteuren getragenen (P)RECAST-Projekt (s. REG-DK-6-902) zeigt konkrete praktische Anwendung auf Beton-Fertigteil-Wiederverwendung, nicht nur normative Existenz ohne Praxisbezug
- G: rechnerischer Nachweis, Dokumentenlage — inferiert (E3, aus dem (P)RECAST-Verwendungszweck „volume, uniformity, and documentation requirements" abgeleitet; Normtext selbst nicht eingesehen, daher kein G-explizit)
- Kernaussage: DS 11990:2024 ist die 2024 veröffentlichte dänische Norm zur Bewertung der Tragfähigkeit bestehender Konstruktionen und wird im laufenden (P)RECAST-Forschungs-/Praxisprojekt der dänischen Betonfertigteilbranche explizit als technische Grundlage für die Beurteilung der Wiederverwendbarkeit von Betonfertigteilen herangezogen — mit Fokus auf Volumen, Einheitlichkeit und Dokumentation der wiederzuverwendenden Elemente. Der Normvolltext selbst konnte in dieser Session nicht eingesehen werden.
- Wortlautbeleg (Originalsprache): "Et af de redskaber, der er blevet anvendt som faglig ramme i projektet, har været den danske standard DS 11990 for vurdering af bæreevnen i eksisterende konstruktioner" (aus der DS-eigenen Newsseite zum (P)RECAST-Projekt)
- Beleg-Quelle: B2 (Titel und Rolle der Norm über die amtliche Newsseite von Dansk Standard selbst — der Normungsorganisation — direkt gelesen und wörtlich zitiert; der Normvolltext selbst über die Shopseite webshop.ds.dk/standard/M352951/ds-11990-2024 technisch nicht auslesbar — JS-gerenderte Seite, leere Extraktion bei zwei unabhängigen Versuchen) · Zugänglichkeit: paywalled-nicht-eingesehen (Normtext) / frei-primär (Newsseite mit Titel-Zitat) · Bindungsakt: nicht identifiziert, ob DS 11990 in BR18 referenziert wird — offene Lücke, an W4 zu melden
- Quelle: Tier 1 (Dansk Standard, dänische Normungsorganisation, eigene Newsseite) · https://www.ds.dk/da/nyhedsarkiv/2025/04/nye-vejledninger-paa-vej-vejen-banes-for-genbrug-af-betonelementer ; Normshop (nicht auslesbar) https://webshop.ds.dk/standard/M352951/ds-11990-2024 · Fassung 2024 · Zugriff 2026-08-13
- Status: in Kraft (2024-Fassung)
- Sub-Ebene: entfällt (A=national)
- Relationen: konkretisiert für Dänemark, was ISO 13822/prEN 1990-2 (DE) und NEN 8700 (NL) materialübergreifend leisten; wird kombiniert mit REG-DK-6-902 ((P)RECAST-Projektberichte) für die betonspezifische Anwendung
- Konfidenz: gesichert (Existenz, Titel, Rolle im (P)RECAST-Projekt über Primärquelle DS selbst belegt); unklar (Normvolltext, BR18-Bindungskette nicht verifiziert)

### REG-DK-6-902 · (P)RECAST-Projektberichte — Genbrug af præfabrikerede betonelementer (Branchenleitfäden in Entstehung)
- Titel: (P)RECAST-Projekt, u. a. Berichte „Udbud med nedtagning af hele betonelementer" und „Nedtagning og håndtering" (Teknologisk Institut, Betonelement-Foreningen, 13 Branchenakteure)
- Fundstelle: Einzelberichte, veröffentlicht April 2025; sieben weitere Berichte laut Ankündigung geplant, in dieser Session nicht einzeln geprüft
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 6 Normen/Regelwerke · Nebenfelder: 2 (Standsicherheit/Ausschreibung/Demontage) · Normtyp: operative Norm
- C: Stahlbeton/Fertigteile
- D: Merkblatt (Branchenleitfaden, laufende Serie, kein Normcharakter)
- E: Rückbau/Sicherung, Bestandserkundung, Planung/Nachweis
- F1 (E3): ermöglichend — die beiden bislang veröffentlichten Berichte adressieren konkret Ausschreibungsspezifikationen für die Demontage ganzer Betonelemente (Zielgruppe Bauherren/beratende Ingenieure) sowie Prinzipien für sichere Demontage und Handhabung (Zielgruppe Bauherren, Ingenieure, Abbruchunternehmen, Lagerbetreiber) — deckt damit genau die Prozessphasen Rückbau/Sicherung und Planung/Nachweis praxisnah ab, für die in anderen Jurisdiktionen (DE, s. REG-DE-2-903) eine vergleichbare Kodifizierung fehlt
- F2 (E3): ermöglichend, aber im Aufbau — die Serie ist laut Ankündigung auf insgesamt neun Berichte angelegt, von denen erst zwei vorliegen; „schonende Rückbautechniken" werden als kritischer Erfolgsfaktor benannt, der eine frühe Einbindung von Abbruchunternehmen sowie verbesserte Dokumentations- und Lagerprotokolle voraussetzt — die praktische Wirkung ist damit auf einen laufenden Aufbauprozess, nicht auf ein abgeschlossenes Regelwerk zu beziehen
- G: Dokumentenlage, Sichtprüfung — inferiert (E3, aus den Berichtstiteln/-zwecken abgeleitet; Einzelberichte selbst nicht im Volltext eingesehen)
- Kernaussage: Das (P)RECAST-Projekt, getragen von Teknologisk Institut, der Betonelement-Foreningen und 13 weiteren dänischen Branchenakteuren, veröffentlichte im April 2025 die ersten zwei von geplant neun Leitfäden zur Wiederverwendung von Betonfertigteilen — zu Ausschreibung/Demontagespezifikation sowie zu Demontage-/Handhabungsprinzipien. Es handelt sich um eine im Aufbau befindliche, branchengetragene Leitfadenserie, nicht um eine abgeschlossene Norm; DS 11990:2024 (REG-DK-6-901) dient dabei als technische Bewertungsgrundlage.
- Wortlautbeleg (Originalsprache): Berichtstitel als Zitat: "Udbud med nedtagning af hele betonelementer" / "Nedtagning og håndtering"; Paraphrase des Umfangs laut Newsseite, nicht als direktes Zitat geprüft
- Beleg-Quelle: B1 für Existenz/Titel/Datum (DS-Newsseite direkt gelesen) · B2/B3 für Berichtsinhalte (nur über die Newsseiten-Zusammenfassung, Einzelberichte selbst nicht abgerufen) · Zugänglichkeit: frei-primär (Newsseite) / Zugänglichkeit der Einzelberichte selbst nicht verifiziert · Bindungsakt: entfällt (Branchenleitfaden, kein Rechtsakt)
- Quelle: Tier 1 (Dansk Standard-Newsseite als Ankündigungsorgan) https://www.ds.dk/da/nyhedsarkiv/2025/04/nye-vejledninger-paa-vej-vejen-banes-for-genbrug-af-betonelementer · Fassung April 2025 (2 von 9 Berichten veröffentlicht) · Zugriff 2026-08-13
- Status: in Kraft für die 2 veröffentlichten Berichte; Entwurf/im Aufbau für die restlichen 7
- Sub-Ebene: entfällt (A=national)
- Relationen: wird kombiniert mit/ergänzt REG-DK-6-901 (DS 11990:2024 als technische Grundlage) und REG-DK-6-903 (StructuralReuse-Projekt, Prüfmethoden)
- Konfidenz: gesichert (Existenz, Titel, Träger, Veröffentlichungsstand); abgeleitet (inhaltliche Details, da Einzelberichte nicht im Volltext eingesehen)

### REG-DK-6-903 · StructuralReuse-Projekt (DTU) — Prüfmethoden für direkten Bauteil-Reuse inkl. Betonbalken
- Titel: StructuralReuse, vierjähriges Forschungsprojekt DTU Sustain (Materials & Durability), mit Lendager, Rambøll, Dansk Standard, Arkitektskolen Aarhus, Gate 21, DTU Skylab
- Fundstelle: Projektankündigung/Ergebnispräsentation, DS-Newsseite
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 6 Normen/Regelwerke · Nebenfelder: 2 · Normtyp: operative Norm
- C: Stahlbeton/Fertigteile (Betonbalken explizit genannt); im Übrigen materialübergreifend (tragende Bauteile allgemein)
- D: Merkblatt — konkret in Form geplanter „DS/INF"-Dokumente (informative, nicht normative Dansk-Standard-Publikationen)
- E: Bestandserkundung, Aufbereitung/Prüfung
- F1 (E3): ermöglichend — entwickelt zerstörungsfreie Prüfverfahren (NDT) zur Bewertung von Materialeigenschaften wiederzuverwendender Bauteile am stehenden Gebäude, kombiniert mehrere NDT-Methoden; für Beton konkret mit Vollmaßstabstests und Ökobilanzberechnungen unterlegt
- F2 (E3): ermöglichend — laut Projektangabe kann „direkte genbrug af betonbjælker … op til 70 % CO₂" gegenüber Neuware sparen; dies ist eine **im Projekt selbst behauptete Kennzahl, nicht unabhängig verifiziert**, hier als Projektaussage, nicht als gesicherter Fakt zu kodieren
- G: zerstörungsfreie Prüfung — explizit (E1, Kernthema des Projekts); Dokumentenlage (Nachweis von Festigkeit/Dauerhaftigkeit/Sicherheit über DS/INF-Dokumente) — inferiert (E3, Dokumente selbst noch nicht veröffentlicht laut Rechercheergebnis)
- Kernaussage: StructuralReuse ist ein DTU-geführtes, vierjähriges Forschungsprojekt zur Entwicklung zerstörungsfreier Prüfmethoden und informativer DS-Leitliniendokumente (DS/INF) für den direkten Wiedereinbau tragender Bauteile, mit Betonbalken als einem konkret untersuchten Fall (Vollmaßstabstests, behauptete CO₂-Einsparung bis 70 %). Ergebnisse wurden auf einer Konferenz am 12.09.2025 in Kopenhagen vorgestellt; die geplanten DS/INF-Dokumente selbst waren zum Recherchezeitpunkt nicht als veröffentlicht verifizierbar.
- Wortlautbeleg (Originalsprache): "direkte genbrug af betonbjælker kan spare op til 70 % CO₂ sammenlignet med brug af nye" (Projektaussage, laut WebFetch-Zusammenfassung der DS-Newsseite)
- Beleg-Quelle: B2 (DS-Newsseite als amtliche Ankündigungsquelle der Normungsorganisation direkt gelesen; die DS/INF-Dokumente selbst nicht eingesehen, da nach Rechercheergebnis noch nicht als eigenständige Publikation auffindbar) · Zugänglichkeit: frei-primär (Newsseite) · Bindungsakt: entfällt (informative DS/INF-Dokumente sind per Definition nicht normativ-bindend)
- Quelle: Tier 1 (Dansk Standard-Newsseite) https://www.ds.dk/da/nyhedsarkiv/2025/08/structuralreuse-vejen-til-mere-cirkulaert-byggeri-med-direkte-genbrug-af-bygningsdele · Fassung Ergebnispräsentation 2025-09-12 · Zugriff 2026-08-13
- Status: Entwurf/in Vorbereitung (DS/INF-Dokumente); Forschungsprojekt selbst laut Ergebnispräsentation im Abschluss
- Sub-Ebene: entfällt (A=national)
- Relationen: wird kombiniert mit/ergänzt REG-DK-6-901/-902; liefert die materialtechnische Prüfmethodik, die (P)RECAST prozessual rahmt
- Konfidenz: abgeleitet (Existenz/Träger/Grundthema gesichert; die 70-%-CO₂-Zahl ist eine unverifizierte Projektaussage, nicht als eigenständiges Faktum zu verwenden)

---

## Schweden

### REG-SE-6-901 · RISE — Qualitätssicherungsmethodik für Beton-Reuse (in Entwicklung, keine geltende Norm)
- Titel: RISE-Forschungsarbeit zu Qualitätssicherungsmethodik für die Wiederverwendung von Betonelementen; parallel KTH-Forschung zu Sicherheitsprotokollen (håldäck, massiva bjälklag, balkar och pelare, dubbel-T-balkar, väggar och trappor)
- Fundstelle: RISE- und KTH-Projektbeschreibungen, über Suchergebnis-Zusammenfassung erschlossen, nicht einzeln direkt abgerufen
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit · Normtyp: operative Norm
- C: Stahlbeton/Fertigteile
- D: außerhalb des D-Vokabulars — laufende Forschungsarbeit, noch keine Publikationsform mit Norm-/Merkblattcharakter
- E: Bestandserkundung, Aufbereitung/Prüfung
- F1 (E3): schweigend — laut Rechercheergebnis „developing national standards for reuse (expected stakeholder discussions late 2024)"; ob diese Stakeholder-Gespräche bis zum Stichtag 2026-08-11 zu einem veröffentlichten Ergebnis geführt haben, wurde in dieser Session **nicht** verifiziert — offene Lücke, kein Faktum
- F2 (E3): schweigend/unklar — RISE-Methodik wird laut Kurzbeschreibung durch „Platsbesök med okulärbesiktning och verifiering av dokumentationen" (Vor-Ort-Begehung mit Sichtprüfung und Dokumentenverifikation) charakterisiert; ob dies bereits in der Praxis angewendet wird oder nur methodisch entwickelt ist, bleibt unklar
- G: Sichtprüfung, Dokumentenlage — inferiert (E3, aus der zitierten Kurzbeschreibung; keine Primärquelle direkt eingesehen)
- Kernaussage: In Schweden wird die Qualitätssicherungsmethodik für die Wiederverwendung von Betonelementen (u. a. durch RISE und KTH) zum Stichtag 2026-08-11 nach den in dieser Session verfügbaren Angaben **noch entwickelt, nicht als geltende nationale Norm veröffentlicht**. Der schwedische Industrieverband Svensk Betong bestätigt laut Kurzbeschreibung allgemein, dass „återbruk av betongprodukter, som prefabricerade element och marksten … möjligt" ist (Wiederverwendung von Betonprodukten wie Fertigteilen und Pflastersteinen ist möglich), ohne dass dies auf ein kodifiziertes Verfahren verweist.
- Wortlautbeleg (Originalsprache): "Återbruk av betongprodukter, som prefabricerade element och marksten är också möjligt" (Svensk Betong, laut Suchergebnis-Zusammenfassung); "Platsbesök med okulärbesiktning och verifiering av dokumentationen" (RISE, laut Suchergebnis-Zusammenfassung)
- Beleg-Quelle: B4 (nur Existenz-/Themennachweis über eine einzelne Suchmaschinen-Zusammenfassung, keine der genannten Quellen — svenskbetong.se, ri.se, kth.se — in dieser Session direkt per WebFetch aufgerufen) · Zugänglichkeit: nicht geprüft · Bindungsakt: entfällt/kein Bindungsakt identifiziert
- Quelle: Tier 3 (Suchmaschinen-Zusammenfassung, keine direkt gelesene Primärquelle — **diese Zeile darf gemäß Belegstrenge nicht als B0–B2-Faktum in der Synthese verwendet werden, nur als Rechercheleitfaden**) · Zugriff 2026-08-13
- Status: unklar/Entwurf (laut Rechercheergebnis „expected stakeholder discussions late 2024" — Ausgang nicht verifiziert)
- Sub-Ebene: entfällt (A=national)
- Relationen: keine belastbare Relation zu bestehenden Objekten herstellbar, da Primärquelle fehlt
- Konfidenz: unklar — **dieses Objekt ist bewusst mit dem niedrigsten Beleg-Grad des gesamten Materialdurchgangs geführt und sollte in W2/W4 vorrangig primärquellenbasiert nachrecherchiert werden**, nicht als Faktum in die Synthese übernommen werden

---

## Norwegen

### REG-NO-6-901 · SINTEF-Leitfaden „Fra riving til ressurs" — Bewertung von Ombruk tragender (insb. Ortbeton-)Konstruktionen
- Titel: „Fra riving til ressurs: Slik vurderer du ombruk av bærende konstruksjoner" (Vom Abriss zur Ressource: So bewerten Sie die Wiederverwendung tragender Konstruktionen), SINTEF, 2025
- Fundstelle: SINTEF-Ankündigungsseite; **direkter Abruf der Artikelseite selbst schlug in dieser Session fehl** (WebFetch lieferte nur die generische SINTEF-„Neuestes"-Übersichtsseite ohne den Zielartikel — technische Zugriffslücke, nicht inhaltlich aufgelöst)
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit · Normtyp: operative Norm
- C: Stahlbeton/Fertigteile (laut Kurzbeschreibung „med særlig fokus på ombruk av plasstøpt armert betong" — Ortbeton-Stahlbeton) · Nebenfelder-Materialfamilie: materialübergreifend (Titel spricht allgemein von „bærende konstruksjoner")
- D: Merkblatt (Forschungsinstituts-Leitfaden, kein Normcharakter, kein Bezug zu TEK17 in den zugänglichen Angaben identifiziert)
- E: Bestandserkundung, Planung/Nachweis
- F1 (E3): ermöglichend — laut Kurzbeschreibung liefert der Leitfaden „bygningseiere og rådgivere et verktøy for å vurdere om bærende konstruksjoner kan gjenbrukes" (Bauherren und Beratern ein Werkzeug zur Bewertung, ob tragende Konstruktionen wiederverwendet werden können), mit besonderem Fokus auf **Ortbeton-Stahlbeton** — bemerkenswert, weil Ortbeton (im Gegensatz zu Fertigteilen) in den übrigen neun Jurisdiktionen in dieser Recherche **kein einziges Mal** als eigener Regelungsgegenstand für Reuse identifiziert wurde; alle übrigen gefundenen Instrumente (CROW-CUR NL, ReCon BE, (P)RECAST DK) fokussieren auf vorgefertigte/Fertigteil-Elemente
- F2 (E3): unklar — ob der Leitfaden bereits praktisch angewendet wird oder erst kürzlich veröffentlicht wurde, ließ sich mangels Artikelzugriffs nicht klären
- G: Sichtprüfung, rechnerischer Nachweis — inferiert (E3, aus der Kurzbeschreibung „verktøy for å vurdere" abgeleitet)
- Kernaussage: SINTEF veröffentlichte 2025 einen Leitfaden zur Bewertung der Wiederverwendbarkeit tragender Konstruktionen mit besonderem Fokus auf **Ortbeton-Stahlbeton** — die einzige in dieser materialübergreifenden Zehn-Jurisdiktionen-Recherche gefundene Quelle, die explizit nicht-vorgefertigten Stahlbeton adressiert. Der direkte Artikelzugriff scheiterte technisch in dieser Session; die hier wiedergegebenen Kernaussagen stammen aus einer Suchmaschinen-Zusammenfassung, nicht aus dem Artikel selbst.
- Wortlautbeleg (Originalsprache): kein Wortlautzitat verfügbar (Artikel nicht erreichbar); englischsprachige Sekundärparaphrase laut Suchergebnis: "A new guide provides building owners and advisors with a tool for assessing whether load-bearing structures can be reused, with particular focus on reuse of cast-in-place reinforced concrete in new constructions" — **Rückübersetzung, kein norwegisches Original-Zitat**
- Beleg-Quelle: B3 (nur Suchmaschinen-Zusammenfassung; direkter WebFetch-Versuch auf sintef.no lieferte die falsche/generische Seite, kein Zugriff auf den Zielartikel) · Zugänglichkeit: frei-primär-blockiert (Artikel sollte grundsätzlich frei zugänglich sein, war aber im direkten Abrufversuch technisch nicht erreichbar — Verwechslung mit Übersichtsseite, kein HTTP-Fehlercode) · Bindungsakt: entfällt/kein Bindungsakt identifiziert
- Quelle: Tier 2/3 (SINTEF ist eine anerkannte norwegische Forschungsinstitution, hier aber nur über Sekundärzusammenfassung erreicht, kein Primärtext) · https://www.sintef.no/siste-nytt/2025/fra-riving-til-ressurs-slik-vurderer-du-ombruk-av-baerenede-konstruksjoner/ (Zielartikel, Zugriff fehlgeschlagen) · Zugriff 2026-08-13 (Versuch), Artikel nicht erfolgreich gelesen
- Status: unklar (laut Suchergebnis 2025, „neuer" Leitfaden — Veröffentlichungsdatum nicht primärquellenbasiert verifiziert)
- Sub-Ebene: entfällt (A=national)
- Relationen: potenziell kombiniert mit/ergänzt der bereits im NO-Länderdurchgang (`NO-alle.md`) erfassten TEK17-§9-5-Ombrukskartlegging-Systematik (nicht hier dupliziert, da Primärtext dieses SINTEF-Leitfadens nicht eingesehen)
- Konfidenz: unklar — **dieses Objekt hat den zweitniedrigsten Beleg-Grad des Materialdurchgangs (nach REG-SE-6-901) und muss vor jeder Verwendung als Faktum primärquellenbasiert nachverifiziert werden**; der Fund selbst (Existenz eines auf Ortbeton fokussierten norwegischen Leitfadens) ist aber inhaltlich bedeutsam genug, um nicht stillschweigend wegzulassen

---

## Vereinigtes Königreich

### REG-UK-6-901 · MPA/The Concrete Centre — „Reusing structures: One step closer to a circular economy" (Branchenleitfaden, unverbindlich)
- Titel: „Reusing structures: One step closer to a circular economy", Concrete Quarterly technical compendium 2022, The Concrete Centre (technischer Arm der Mineral Products Association, MPA)
- Fundstelle: Circular-economy-Themenseite concretecentre.com; Volltext des CQ-Artikels selbst nicht eingesehen
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 6 Normen/Regelwerke · Nebenfelder: 2 · Normtyp: operative Norm
- C: Stahlbeton/Fertigteile
- D: Branchenprotokoll (MPA ist ein privater Industrieverband, kein Normungsgremium wie BSI)
- E: Bestandserkundung, Planung/Nachweis
- F1 (E3): ermöglichend — die Seite behauptet, Methoden zur Bewertung der Wiederverwendungseignung eines Betonrahmens seien „well established" (etabliert), liefert auf der eingesehenen Seite selbst aber **kein** detailliertes technisches Rahmenwerk; verweist stattdessen auf BS 8500-1 (Tabellen A4/A5, Expositionsklasse XC1) zur Dauerhaftigkeits-/Lebensdauerfrage, nicht zu einem dedizierten Reuse-Nachweisverfahren
- F2 (E3): bedingend — als Industrieverbandsempfehlung ohne Normcharakter entfaltet die Seite keine bauaufsichtliche Bindungswirkung; sie hebt Dauerhaftigkeit, Brandwiderstand und Anpassungsfähigkeit von Beton als Reuse-Vorteile hervor, bleibt methodisch aber vage
- G: Dokumentenlage — inferiert (E3); kein expliziter Nachweiskatalog auf der eingesehenen Seite
- Kernaussage: The Concrete Centre (MPA) veröffentlichte 2022 im Rahmen des Concrete-Quarterly-Kompendiums einen Beitrag zur Wiederverwendung von Betonstrukturen, der die Bewertungsmethodik als etabliert bezeichnet, ohne sie im eingesehenen Seitenauszug im Detail darzulegen. Referenziert wird BS 8500-1 für Dauerhaftigkeits-/Lebensdauerklassen, nicht für ein Reuse-spezifisches Verfahren. Es handelt sich um unverbindliche Industrieverbandsempfehlung, keine Norm oder Regulierung.
- Wortlautbeleg (Originalsprache): "According to the design standards for a concrete frame located internally … no additional measures are required to achieve a service life of over 100 years compared to 50" (mit Verweis auf BS 8500-1, Tabellen A4/A5, Expositionsklasse XC1)
- Beleg-Quelle: B1 (Themenseite direkt via WebFetch gelesen; der zugrundeliegende CQ-2022-Artikel selbst nicht separat abgerufen) · Zugänglichkeit: frei-primär (Themenseite) · Bindungsakt: entfällt (privates Branchenprotokoll)
- Quelle: Tier 3 (Industrieverband MPA/Concrete Centre, keine Behörde/kein Normungsgremium) · https://www.concretecentre.com/Performance-Sustainability/Circular-economy/Refurbishment,-reuse-and-renewal.aspx · Fassung 2022 (CQ-Kompendium) · Zugriff 2026-08-13
- Status: in Kraft (als aktuellste identifizierte Fassung der Themenseite)
- Sub-Ebene: entfällt (A=national)
- Relationen: wird kombiniert mit/ergänzt REG-UK-6-902 (MPA Precast Service-Life-Datenblatt); Verhältnis zu den bereits erfassten UK-F1-3/UK-F4-7-Objekten (Building Regulations Part A u. Ä.) in dieser Session nicht abschließend geklärt — an W4 zur Querprüfung zu melden
- Konfidenz: abgeleitet (Existenz/Kernaussage über direkt gelesene Themenseite gesichert; technische Tiefe des zugrunde liegenden CQ-Artikels nicht verifiziert)

### REG-UK-6-902 · MPA Precast — Specifying Service Life Datenblatt (2020): Lücke zwischen 50/100-Jahres-Normwerten und 60-Jahres-Projektpraxis
- Titel: „Specifying Service Life" Datenblatt, MPA Precast, 2020
- Fundstelle: Ressourcenseite mpaprecast.org/Resources/Structures-facades-guidance.aspx
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 6 Normen/Regelwerke · Nebenfelder: 2 · Normtyp: operative Norm
- C: Stahlbeton/Fertigteile
- D: Branchenprotokoll
- E: Planung/Nachweis
- F1 (E3): bedingend — britische Betonnormen (laut Datenblattbeschreibung) geben Angaben zu Nutzungsdauern von 50 oder 100 Jahren, während UK-Bauprojekte in der Praxis zunehmend eine 60-Jahres-Nutzungsdauer spezifizieren; für die Wiederverwendung eines Fertigteils mit bereits abgelaufener Teil-Nutzungsdauer aus dem 50/100-Jahres-Raster existiert **kein** im eingesehenen Auszug erkennbarer eigener Umrechnungs- oder Anrechnungsmechanismus (Restnutzungsdauer-Bestimmung)
- F2 (E3): hemmend — diese Diskrepanz zwischen Normraster (50/100 Jahre) und Projektpraxis (60 Jahre) ist ein strukturelles Hindernis speziell für die Bewertung der **Rest**-Nutzungsdauer wiederverwendeter Betonfertigteile, da ein gebrauchtes Element nicht einfach in eine der beiden Normkategorien fällt, sondern eine eigene Bewertung der bereits verstrichenen und der verbleibenden Nutzungsdauer erfordert — dieses spezifische Problem wird im zugänglichen Auszug nicht gelöst
- G: entfällt (reines Datenblatt zu Bemessungsparametern, kein eigener Nachweistatbestand für Reuse erkennbar)
- Kernaussage: Das MPA-Precast-Datenblatt „Specifying Service Life" (2020) macht auf die Diskrepanz zwischen den in britischen Betonnormen verankerten 50-/100-Jahres-Nutzungsdauerkategorien und der in der Projektpraxis verbreiteten 60-Jahres-Spezifikation aufmerksam. Für die Materialfamilie Stahlbeton/Fertigteile bedeutet dies einen strukturellen Bewertungsengpass bei der Restnutzungsdauer-Bestimmung wiederverwendeter Elemente, der im eingesehenen Auszug nicht durch ein eigenes Reuse-Verfahren adressiert wird.
- Wortlautbeleg (Originalsprache): "Building projects in the UK are specifying a design life of 60 years, but, British concrete standards give information to intended working lives of 50 or 100 years."
- Beleg-Quelle: B1 (Ressourcenseite direkt via WebFetch gelesen, Datenblatt-Beschreibung wörtlich zitiert; das Datenblatt selbst als PDF nicht separat abgerufen) · Zugänglichkeit: frei-primär (Beschreibungsseite) · Bindungsakt: entfällt (Branchenprotokoll)
- Quelle: Tier 3 (Industrieverband MPA Precast) · https://www.mpaprecast.org/Resources/Structures-facades-guidance.aspx · Fassung 2020 · Zugriff 2026-08-13
- Status: in Kraft (2020-Fassung, keine Aktualisierung identifiziert)
- Sub-Ebene: entfällt (A=national)
- Relationen: wird kombiniert mit/ergänzt REG-UK-6-901; konkretisiert das allgemeine Bestandsbewertungsproblem materialspezifisch für Beton-Nutzungsdauerkategorien
- Konfidenz: gesichert (Wortlaut/Existenz über direkt gelesene Quelle); abgeleitet (Bewertung als „strukturelle Lücke" ist projekteigene Analyse, E3)

---

## Zusammenfassende Querschnittsbefunde (E3, Projektzuordnung)

1. **Bewehrungsnachweis ist in keiner der zehn Jurisdiktionen als eigener kodifizierter Nachweistyp identifiziert.** Überall, wo Nachweisanforderungen für Stahlbeton-Bestandsbauteile gefunden wurden (DE: ISO 13822/prEN 1990-2 generisch; CH: SIA 269/2; NL: NEN 8700 + CROW-CUR 4:2023; DK: DS 11990:2024; NO: SINTEF-Leitfaden), wird die Identifikation/Verifikation der vorhandenen Bewehrung durchgehend unter das generische G-Vokabular „zerstörungsfreie Prüfung"/„Probenahme/Materialprüfung" subsumiert (G-inferiert, E3), **ohne dass eine bindende Norm ein konkretes Verfahren** (Bewehrungssuchgerät, Radar, Ausbohren/Freilegen) vorschreibt. Einzige Ausnahme mit expliziter methodischer Vertiefung: das dänische StructuralReuse-Projekt (REG-DK-6-903), das mehrere NDT-Methoden kombiniert — aber auch dies (Stand 2026-08-13) noch als Forschungsergebnis, nicht als kodifizierte Norm.
2. **Fertigteil-Bias:** Mit Ausnahme des norwegischen SINTEF-Leitfadens (REG-NO-6-901, Fokus explizit auf Ortbeton/plasstøpt armert betong) adressieren **alle** in dieser Recherche gefundenen materialspezifischen Reuse-Instrumente ausschließlich **vorgefertigte** Betonelemente (NL: Kanaalplaten/Brückenträger; BE: horizontale Fertigteile Balken/Platten; DK: Fertigteile). Ortbeton-Stahlbeton — der in Bestandsgebäuden mengenmäßig dominierende Fall — ist strukturell unterreguliert.
3. **Asymmetrie Baustahl vs. Stahlbeton bei der Versicherbarkeit (FR):** Das erste französische C2P-anerkannte Reuse-Referenzwerk gilt für Baustahl, nicht für Beton (REG-FR-7-901) — ein Befund, der die im Aufgabentext genannte Fallenliste nicht explizit benennt, aber strukturell demselben Muster folgt wie die dortigen Hinweise auf materialabhängige Ungleichbehandlung.
4. **Deutschland hat trotz DAfStb als etabliertem Fachgremium keine identifizierte Reuse-spezifische Richtlinie für Stahlbeton** (REG-DE-2-903, Negativbefund) — ein Befund, der in Kontrast zur Fachliteratur (Mecka et al. 2025) steht, die einen entsprechenden Rahmen erst als Forschungsdesiderat entwickelt.
5. **Kostenpflicht als durchgängige Zugangshürde** bei den materialspezifischen Bemessungsnormen (ÖNORM B 1992-1-1, SIA 269/2, NEN 8700/8701, DS 11990) — konsistent mit dem bereits im Länderdurchgang für AT/CH/NL dokumentierten Befund, hier materialspezifisch bestätigt.

## Offene Punkte für W4/W2-Nacherhebung
- DAfStb-Negativbefund (REG-DE-2-903) sollte durch eine gezielte Anfrage direkt beim DAfStb oder eine Volltextsuche in der Schriftenreihe (400+ Bände) gegengeprüft werden, bevor er als endgültiges Faktum in die Synthese eingeht.
- SIA 262 (Betonbau, Neubau) wurde als mögliche Ergänzung zu SIA 269/2 identifiziert, aber in dieser Session **nicht** eigenständig verifiziert — bewusst nicht erfunden, sondern als Lücke geführt.
- REG-SE-6-901 und REG-NO-6-901 tragen die niedrigsten Beleg-Grade dieser Datei (B3/B4) und benötigen vorrangig direkten Primärquellenzugriff (svenskbetong.se, ri.se, kth.se bzw. der SINTEF-Zielartikel selbst).
- C2P „Liste Verte" (REG-FR-7-901) sollte direkt unter liste-verte-c2p.qualiteconstruction.com auf ein zwischenzeitlich möglicherweise ergänztes Beton-Dokument geprüft werden.
- FDB-Merkblatt Nr. 10 (REG-DE-6-902) sollte mit einem PDF-Textextraktionswerkzeug statt WebFetch erneut geprüft werden, um belastbare Wort-für-Wort-Zitate zu gewinnen.
- CROW-CUR Richtlijn 4:2023 (REG-NL-2-901) — der vollständige Richtlinientext (inkl. Anhänge zu Kanaalplaten/Brückenträgern) liegt hinter einer Registrierungsschranke und wurde nicht eingesehen; die technischen Detailanforderungen (G-Achse) sind daher inferiert, nicht explizit.
