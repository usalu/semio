# W2 · Schweiz — Bund + Kantonsstichprobe (Quellenaufschluss und Extraktion in einem Durchgang)

**Auftrag laut Ticket:** CH Sub-Ebene erheben — Bund vollständig (BauPG/BauPV, VVEA) + IVHB vollständig, dann deklarierte Kantonsstichprobe über drei Sprachregionen (ZH, GE/VD, TI, BE). Baubewilligung ist kantonal, Sub-Ebenen-Tiefe für CH laut Freeze: **Stichprobe-und-Deklaration** (nicht Vollerhebung wie BE/UK).

**Methodischer Hinweis (Abweichung vom Standardablauf):** Die unter `roh/CH-quellen.md` erwartete separate Quellenaufschluss-Stufe (W2 Stufe 1) existierte zu Beginn dieser Sitzung nicht (Datei leer/nicht angelegt). Dieses Dokument leistet daher **beides in einem Durchgang**: Quellenidentifikation und Extraktion mit voller A–G-Kodierung. Das senkt die erreichbare Tiefe gegenüber einer zweistufigen Bearbeitung — Lücken sind entsprechend häufiger und werden unten ehrlich benannt, statt Fundstellen zu erfinden.

**Schema:** Bindend `schema/taxonomie-final.md` (eingefroren 2026-08-11).

**Stichtag:** 2026-08-11. Zugriffsdatum aller unten genannten Quellen: 2026-08-11.

**Zugriffstechnik (Pflichtvermerk):** Fedlex-Landingpages (`fedlex.admin.ch/eli/cc/.../de`) liefern bei direktem Abruf nur eine JS-Kompatibilitätsseite ohne Inhalt. Primärtexte wurden stattdessen über die `fedlex.data.admin.ch/filestore/.../pdf-a/...pdf`-Direktlinks bezogen, lokal mit `pdftotext -layout` in Klartext konvertiert und mit Grep/Read wortgenau durchsucht — **kein** Modell-Sekundärzitat aus einer PDF-Zusammenfassung ohne Volltextprüfung. An zwei Stellen widerlegte der Volltext eine zunächst über WebFetch-Zusammenfassung suggerierte falsche Aussage (s. § 0.1) — das bestätigt die Projektregel, dass Beleg-Quelle B0/B1 tatsächliche Volltexteinsicht voraussetzt, keine Fetch-Zusammenfassung.

**Web-Suchbudget:** Das geteilte Session-Suchbudget (200 WebSearch-Aufrufe) wurde im Verlauf dieser Sitzung erschöpft. Die Kantone VD und BE sind dadurch dünner belegt als ZH/GE/TI — unten einzeln vermerkt, nicht verschwiegen.

---

## 0. Kritische Vorab-Befunde

**0.1 Modell-Halluzination bei erstem BauPG-Fetch, durch Volltext widerlegt:** Ein erster WebFetch-Durchlauf auf die BauPG-PDF behauptete, Art. 2 BauPG definiere "wiederverwendete Bauprodukte" als eigene Kategorie mit spezifischen Leistungserklärungspflichten. Die anschliessende Volltextprüfung (pdftotext + Grep auf "gebraucht|wiederverwend") zeigt: **Art. 2 BauPG enthält keinen einzigen Begriff zu gebrauchten/wiederverwendeten Bauprodukten.** Das BauPG ist zu dieser Frage schlicht **schweigend** — ein echter, textbelegter Befund, keine Lücke. Diese Falle (Fetch-Zusammenfassung statt Volltext) ist für die weitere Synthese als Warnung festzuhalten.

**0.2 CH ist nicht EEA — MRA statt CPR-Direktgeltung:** Die Schweiz ist nicht EWR-Mitglied; Bauprodukte-Marktzugang läuft über das bilaterale Abkommen vom 21. Juni 1999 zwischen der Schweizerischen Eidgenossenschaft und der EG über die gegenseitige Anerkennung von Konformitätsbewertungen (MRA, SR 0.946.526.81), auf das BauPG Art. 6 Abs. 2 Bst. b ausdrücklich verweist. Die EU-CPR (305/2011 bzw. ab 08.01.2026 2024/3110) gilt in CH **nicht unmittelbar** — A = national für BauPG/BauPV, nicht EU/EEA. Das entspricht der Fallenliste (CH nur bilaterales MRA, nicht EEA).

**0.3 Grundanforderung 7 BauPV enthält explizite Reuse-Sprache — anders als BauPG selbst:** Während das BauPG (Gesetzesebene) schweigt, übernimmt die BauPV (Verordnungsebene, Anhang I) wortgleich die alte EU-CPR-Grundanforderung Nr. 7 "Nachhaltige Nutzung der natürlichen Ressourcen" **inklusive** der Bst.-a-Formulierung zur "Wiederverwendbarkeit und Rezyklierbarkeit des Bauwerks, seiner Baustoffe und Teile nach dem Abriss" (s. REG-CH-1-002). Diese Anforderung betrifft das **Bauwerk als Ganzes nach Abriss**, nicht das einzelne wiederverwendete Bauprodukt beim erneuten Inverkehrbringen — anders akzentuiert als die neue EU-CPR 2024/3110, die (Stand W1-EU-Basisschicht) gezielt Regeln für das Inverkehrbringen bereits gebrauchter Produkte enthält.

**0.4 Sub-national bedeutet in CH: 26 eigenständige Baugesetze, keine gemeinsame "Muster"-Ebene wie MBO/DE:** Anders als bei DE (MBO als Vorlage) gibt es in CH kein Bund-Kantone-Mustergesetz für das Baurecht selbst. Die einzigen gesamtschweizerischen Vereinheitlichungsinstrumente sind Konkordate zu **Teilaspekten** (IVHB für Baubegriffe/Messweisen, IVTH für technische Handelshemmnisse/VKF-Brandschutz, MuKEn für Energierecht) — das materielle Baubewilligungsrecht bleibt genuin kantonal, in der Praxis oft weiter an Gemeinden delegiert (hier nicht gesondert erhoben, s. Sub-Ebene-Deklaration).

---

## A. BUND — vollständig laut Auftrag

### REG-CH-1-001 · BauPG (Bundesgesetz über Bauprodukte)

- Titel: Bundesgesetz vom 21. März 2014 über Bauprodukte (Bauproduktegesetz, BauPG)
- Fundstelle: Art. 1–10 (Zweck, Begriffe, Grundanforderungen, Leistungserklärung); SR 933.0
- A: national · A-Ursprung: national (MRA-basiert, keine unmittelbare EU-Geltung) · Downstream-Verifikationsstatus: entfällt (kein Muster-/Bund-Länder-Dokument)
- B: Primärfeld 1 · Nebenfelder: 2 (Grundanforderung mechanische Festigkeit/Standsicherheit, Art. 3 Abs. 2 Bst. a)
- C: materialübergreifend
- D: Gesetz
- E: Inverkehrbringen · E-Wirkung: durchläuft
- F1 (E3): schweigend · Bezugsgegenstand: gebrauchte/wiederverwendete Bauprodukte beim erneuten Inverkehrbringen — tatbestandlich nicht erfasst (kein Begriff, keine Ausnahme, keine Erschwerung)
- F2 (E3): schweigend · Bezugsgegenstand: dieselbe Fallgruppe — Praxis muss auf Analogie zu Art. 5 Abs. 2 (Ausnahmen von der Leistungserklärungspflicht bei Sonderanfertigung/Denkmalschutz) zurückgreifen, ohne dass dies textlich für Reuse vorgesehen ist
- G: entfällt (die geprüften Art. 1–10 enthalten keinen auf Reuse bezogenen eigenen Nachweistatbestand; die allgemeine Leistungserklärungspflicht nach Art. 5 ist G=1 Dokumentenlage, explizit=E1, aber ohne Reuse-Bezug)
- Kernaussage: Das BauPG regelt Inverkehrbringen und Bereitstellung von Bauprodukten auf Basis des MRA mit der EU und verweist für Sicherheit auf harmonisierte Normen bzw. das allgemeine Sicherheitsgebot. Der Gesetzestext (Art. 1–10) enthält keinerlei Sonderregel, Begriffsdefinition oder Ausnahme für gebrauchte oder wiederaufgearbeitete Bauprodukte. Einzige textnahe Anknüpfung ist die allgemeine Ausnahme von der Leistungserklärungspflicht für Sonderanfertigungen und denkmalschutzgerechte Renovierung (Art. 5 Abs. 2 Bst. c), die aber nicht auf Reuse zugeschnitten ist.
- Wortlautbeleg (Originalsprache): "Bauprodukte dürfen nur in Verkehr gebracht oder auf dem Markt bereitgestellt werden, wenn sie im Sinne des Artikels 3 Absatz 1 PrSG sicher sind" (Art. 4 Abs. 1 BauPG)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Bundesgesetz, unmittelbar bindend)
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2014/495/de (Volltext bezogen über https://www.fedlex.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2014/495/20230901/de/pdf-a/fedlex-data-admin-ch-eli-cc-2014-495-20230901-de-pdf-a.pdf) · Fassung(as-amended) 2023-09-01 · Zugriff 2026-08-11
- Status: in Kraft · seit 2014, Stand 2023-09-01
- Sub-Ebene: entfällt (A=national)
- Relationen: konkretisiert BauPV (REG-CH-1-002), determiniert Anwendbarkeit von REG-CH-1-002
- Konfidenz: gesichert

### REG-CH-1-002 · BauPV Anhang I Grundanforderung 7

- Titel: Verordnung vom 27. August 2014 über Bauprodukte (Bauprodukteverordnung, BauPV), Anhang I Ziff. 7
- Fundstelle: Anhang I Ziff. 7 "Nachhaltige Nutzung der natürlichen Ressourcen" Bst. a–c (Art. 4 Abs. 1 BauPV); SR 933.01
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 1 · Nebenfelder: 2 (Bezug zu Bauwerksleistung im Ganzen)
- C: materialübergreifend
- D: RVO
- E: Planung/Nachweis, Rückbau/Sicherung · E-Wirkung: durchläuft (Grundanforderung an das künftige Bauwerk, wirkt vorausschauend auf dessen späteren Rückbau)
- F1 (E3): ermöglichend · Bezugsgegenstand: Bauwerk als Ganzes nach Abriss (nicht: einzelnes wiederverwendetes Produkt beim Wieder-Inverkehrbringen)
- F2 (E3): schweigend · Bezugsgegenstand: praktische Durchsetzbarkeit — die Grundanforderung ist programmatisch, ohne eigenen Nachweistatbestand oder Vollzugsmechanismus in der BauPV selbst; ob und wie sie im Baubewilligungsverfahren geprüft wird, regelt die BauPV nicht
- G: Anwendbarkeitsnorm ohne Nachweistatbestand (Wert 8) (inferiert=E3 — der Text formuliert ein Ziel, keinen Prüf-/Nachweisschritt)
- Kernaussage: Anhang I Ziff. 7 BauPV übernimmt wortgleich die frühere EU-CPR-Grundanforderung Nr. 7 und verlangt, dass Bauwerke so entworfen, errichtet und abgerissen werden, dass ihre natürlichen Ressourcen nachhaltig genutzt werden — ausdrücklich genannt wird die "Wiederverwendbarkeit und Rezyklierbarkeit des Bauwerks, seiner Baustoffe und Teile nach dem Abriss". Die Norm bleibt eine programmatische Grundanforderung ohne eigenen Prüf- oder Zulassungsmechanismus; sie adressiert das Bauwerk im Ganzen, nicht das einzelne wiederverwendete Bauprodukt.
- Wortlautbeleg (Originalsprache): "Das Bauwerk muss derart entworfen, errichtet und abgerissen werden, dass die natürlichen Ressourcen nachhaltig genutzt werden, damit insbesondere Folgendes sichergestellt wird: a. die Wiederverwendbarkeit und Rezyklierbarkeit des Bauwerks, seiner Baustoffe und Teile nach dem Abriss" (Anhang I Ziff. 7 Bst. a BauPV)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Bundesratsverordnung, gestützt auf BauPG Art. 3 Abs. 3)
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2014/496/de (Volltext bezogen über https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2014/496/20220101/de/pdf-a/fedlex-data-admin-ch-eli-cc-2014-496-20220101-de-pdf-a.pdf) · Fassung(as-amended) 2022-01-01 · Zugriff 2026-08-11
- Status: in Kraft · seit 2014, Stand 2022-01-01
- Sub-Ebene: entfällt
- Relationen: konkretisiert BauPG Art. 3 (REG-CH-1-001)
- Konfidenz: gesichert

### REG-CH-6-003 · IVTH (Interkantonale Vereinbarung zum Abbau technischer Handelshemmnisse)

- Titel: Interkantonale Vereinbarung vom 23. Oktober 1998/25. November 2002 zum Abbau technischer Handelshemmnisse (IVTH); SR 172.056.5 (Bundesbeschluss über die Genehmigung); kantonal als Konkordat in Kraft
- Fundstelle: Vereinbarung insgesamt (Grundlage für IOTH — Interkantonales Organ für technische Handelshemmnisse)
- A: sub-national · A-Ursprung: sub-national (von den Kantonen selbst über BPUK/vormals Bau-, Planungs- und Umweltdirektoren-Konferenz erarbeitetes Konkordat) · Downstream-Verifikationsstatus: strukturell angenommen, nicht verifiziert (alle 26 Kantone Beitritt sekundärquellenbasiert angenommen, nicht einzeln primärquellenbasiert geprüft)
- B: Primärfeld 6 · Nebenfelder: 2, 4 (VKF-Brandschutz, s. REG-CH-4-004)
- C: materialübergreifend
- D: Gesetz (Konkordat, in jedem beigetretenen Kanton als eigener Erlass mit Gesetzesrang ratifiziert — kein eigener D-Wert für Konkordate im Schema; nächstliegende Einordnung analog Muster-/Bund-Länder-Kodierregel, hier aber mit unmittelbarer Bindung ohne weitere Transformation, daher Gesetz statt Muster-/Modellrecht)
- E: Planung/Nachweis, Einbau/Abnahme · E-Wirkung: durchläuft
- F1 (E3): schweigend · Bezugsgegenstand: Wiederverwendung/Bestandsbau wird im IVTH-Text nicht thematisiert; die Vereinbarung ist reines Verfahrens-/Institutionenrecht (schafft IOTH, das seinerseits VKF-Vorschriften verbindlich erklärt)
- F2 (E3): bedingend · Bezugsgegenstand: mittelbare Wirkung über das IOTH-Instrument — indem sie kantonsübergreifend einheitliche technische Anforderungen ermöglicht, reduziert sie potenziell die Rechtszersplitterung, die Reuse über Kantonsgrenzen hinweg erschweren würde; dies ist aber eine Projektzuordnung, keine im Text angelegte Zweckbestimmung
- G: entfällt (institutionelles Verfahrensrecht ohne eigenen Nachweistatbestand)
- Kernaussage: Die IVTH ergänzt BauPG/BauPV in den Bereichen, in denen die Gesetzgebungskompetenz bei den Kantonen liegt, und schafft mit dem IOTH ein interkantonales Organ, das technische Vorschriften (insbesondere die VKF-Brandschutzvorschriften) für alle beigetretenen Kantone verbindlich erklären kann. Sie ist damit der zentrale Bindungsketten-Mechanismus, über den private/verbandliche technische Regelwerke in CH kantonsübergreifend rechtsverbindlich werden — funktional vergleichbar mit dem VV-TB-System in Deutschland, institutionell aber grundverschieden (Konkordat statt Verwaltungsvorschrift).
- Wortlautbeleg (Originalsprache): "Das Hauptziel dieser interkantonalen Vereinbarung besteht im Abbau technischer Handelshemmnisse zwischen der Schweiz und dem Ausland sowie zwischen den Kantonen" (Sekundärquelle BPUK, Paraphrase des Vereinbarungszwecks — Primärtext der Vereinbarung selbst in dieser Sitzung nicht im Volltext gelesen, s. Beleg-Quelle)
- Beleg-Quelle: B2 · Zugänglichkeit: frei-primär (Fundstelle vorhanden, in dieser Sitzung nicht per Volltext-PDF verifiziert — Fedlex-Landingpage lieferte nur JS-Hinweis, PDF-Direktlink nicht gefunden) · Bindungsakt: Bindungsmechanismus existiert, Listung im Einzelfall nicht verifiziert (welche Kantone konkret beigetreten sind, ausstehend)
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2003/25/de · https://www.bpuk.ch/bpuk/konkordate/ivth · Fassung(as-amended) unbekannt (Stand 2003 laut Sekundärquelle, seither ggf. novelliert) · Zugriff 2026-08-11
- Status: in Kraft · seit 2003-02-03 (laut Sekundärquelle)
- Sub-Ebene: Stichprobe [nicht einzeln geprüft, s. Downstream-Verifikationsstatus] / nicht erhoben [alle 26 Kantone einzeln auf Beitritt]
- Relationen: setzt um BauPG/BauPV in kantonaler Zuständigkeit, determiniert Anwendbarkeit von REG-CH-4-004 (VKF-Brandschutzvorschriften)
- Konfidenz: abgeleitet (Primärtext nicht vollständig gelesen)

### REG-CH-4-004 · VKF-Brandschutzvorschriften (Schweizerische Brandschutzvorschriften)

- Titel: Brandschutznorm und Brandschutzrichtlinien der Vereinigung Kantonaler Feuerversicherungen (VKF), Ausgabe 2015 (mit Nachträgen)
- Fundstelle: Gesamtwerk, insbesondere Brandschutznorm + Brandschutzrichtlinien ("Ordner A" = rechtsverbindlich für alle Kantone; "Ordner B" = unverbindliche Erläuterungen/Stand der Technik)
- A: sub-national · A-Ursprung: international/privat (Erarbeitung durch VKF, einen privaten Verband der kantonalen Gebäudeversicherer) · Downstream-Verifikationsstatus: verifiziert in [alle Kantone, laut Sekundärquelle seit Konkordatsbeitritt aller 26 Kantone 2005 — nicht selbst primärquellenbasiert kantonsweise nachgeprüft]
- B: Primärfeld 4 · Nebenfelder: 2, 6
- C: materialübergreifend
- D: Techn.Baubestimmung (funktionales Analogon zum VV-TB-Mechanismus: privat erarbeitete technische Regel, durch hoheitlichen Akt — hier IOTH-Erklärung statt VV-TB-Listung — für alle Kantone verbindlich erklärt)
- E: Planung/Nachweis, Einbau/Abnahme, Betrieb/Dokumentation · E-Wirkung: durchläuft
- F1 (E3): schweigend · Bezugsgegenstand: Wiederverwendung wiederverwendeter Bauteile (Türen, Verglasungen, Brandschutzverkleidungen) wird im recherchierten Sekundärmaterial nicht behandelt; ob Brandschutznorm/-richtlinien eigene Reuse-Bestimmungen enthalten, ist in dieser Sitzung **nicht** primärquellenbasiert geprüft worden (Volltext kostenpflichtig/nicht in dieser Sitzung beschafft)
- F2 (E3): hemmend · Bezugsgegenstand: faktische Wirkung — Brandschutznachweise für wiederverwendete brandschutzrelevante Bauteile (Türen, Verglasungen) sind in der Praxis ein bekannter Gatekeeper (analog DE-Diskussion zu Brandschutzverglasung/Zulassung), auch ohne dass dies primärquellenbasiert für CH belegt wurde — **E3, Projekteinschätzung, nicht textbelegt**
- G: Einzelfallzulassung (inferiert=E3 — für nicht normkonforme/wiederverwendete Bauteile ist praxisüblich ein Einzelfallnachweis nötig, im hier eingesehenen Sekundärmaterial nicht ausdrücklich für Reuse geregelt)
- Kernaussage: Die VKF-Brandschutzvorschriften sind durch Erklärung des IOTH (gestützt auf die IVTH) für alle Schweizer Kantone rechtsverbindlich; sie bilden faktisch den gesamtschweizerischen Brandschutz-Baustandard, ohne dass dafür ein eidgenössisches Brandschutzgesetz existiert. Zur Behandlung wiederverwendeter brandschutzrelevanter Bauteile wurde in dieser Sitzung keine primärquellenbasierte Aussage möglich — echte Lücke, kein Negativbefund.
- Wortlautbeleg (Originalsprache): "Die Brandschutzvorschriften der VKF […] wurden vom neu geschaffenen Interkantonalen Organ für technische Handelshemmnisse (IOTH) genehmigt und für die ganze Schweiz für verbindlich erklärt" (Sekundärquelle, Paraphrase — Primärtext der Brandschutzvorschriften selbst kostenpflichtig, in dieser Sitzung nicht eingesehen)
- Beleg-Quelle: B3 · Zugänglichkeit: paywalled-nicht-eingesehen (Vollständige Brandschutzvorschriften kostenpflichtig über bsvonline.ch; Sekundärdarstellung frei zugänglich) · Bindungsakt: benannt (IOTH-Erklärung gestützt auf IVTH, s. REG-CH-6-003) — Konfidenz zur konkreten Reichweite bleibt unklar, bis Primärtext eingesehen ist
- Quelle: Tier 3 (für den Vollzugsmechanismus: Sekundärdarstellung nussbaum.ch/vkg.ch, nur Suchhinweis) · https://www.bsvonline.ch/de/brandschutzvorschriften · Fassung(as-amended) 2015 (mit Nachträgen, genauer Stand 2026 nicht verifiziert) · Zugriff 2026-08-11
- Status: in Kraft · seit 2015 (Ausgabe), Nachträge unbekannt
- Sub-Ebene: Stichprobe [nicht einzeln geprüft] / nicht erhoben [alle 26 Kantone]
- Relationen: setzt um REG-CH-6-003 (IVTH/IOTH-Bindungsmechanismus)
- Konfidenz: unklar (B3/paywalled-nicht-eingesehen — Bindungsmechanismus gesichert, materieller Reuse-Gehalt nicht)

### REG-CH-4-005 · MuKEn (Mustervorschriften der Kantone im Energiebereich)

**[Prüfung 2026-08-13 — KORRIGIERT + ID-Kollision:] Das Verabschiedungsdatum war falsch (s. u.). Ausserdem trägt dieses Objekt dieselbe ID `REG-CH-4-004`-Kollision wie unten erklärt, UND behandelt denselben Sachverhalt wie REG-CH-4-004 in `CH-F4-7.md` (dort korrektes Datum 29.08.2025, B1-Beleg per Medienmitteilung-PDF). Für die Synthese (W4) sollte dieses Objekt mit REG-CH-4-004 (CH-F4-7.md) zusammengeführt werden; dort ist die belastbarere Fassung.**

- Titel: Mustervorschriften der Kantone im Energiebereich (MuKEn), aktuellste verabschiedete Fassung MuKEn 2025 (Plenarversammlung EnDK, 29.08.2025 — **korrigiert, s. Prüfvermerk oben; ursprünglich fälschlich "04.04.2025"**)
- Fundstelle: Gesamtwerk (Modul-Struktur, u. a. Anforderungen an Bauteile/Gebäudehülle)
- A: sub-national · A-Ursprung: sub-national (erarbeitet von der Konferenz Kantonaler Energiedirektoren, EnDK — selbst ein interkantonales Gremium) · Downstream-Verifikationsstatus: strukturell angenommen, nicht verifiziert (Sekundärquelle: 22 von 26 Kantonen wendeten Vorgängerfassung MuKEn 2014 laut Stand 09/2023 an, 4 in Umsetzung — bezogen auf die alte Fassung, MuKEn-2025-Umsetzungsstand kantonal nicht einzeln geprüft)
- B: Primärfeld 4 · Nebenfelder: 5b
- C: materialübergreifend
- D: Muster-/Modellrecht (unverbindlich, Umsetzung durch Dritte erforderlich)
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): schweigend · Bezugsgegenstand: Wiederverwendung von Bauteilen im MuKEn-Regelwerk selbst in dieser Sitzung nicht primärquellenbasiert geprüft (Modultext kostenpflichtig/nicht beschafft) — echte Erhebungslücke
- F2 (E3): bedingend · Bezugsgegenstand: energetische Anforderungen an die Gebäudehülle (U-Werte) sind ein bekannter Praxis-Gatekeeper für wiederverwendete Fenster/Fassadenelemente, analog zur BE/NL-Diskussion (EPB/MPG) — E3-Einschätzung, nicht CH-textbelegt in dieser Sitzung
- G: entfällt (kein Primärtext geprüft, kein Nachweistatbestand belastbar zuordenbar)
- Kernaussage: MuKEn ist ein von der EnDK erarbeitetes, gesamtschweizerisches Muster für kantonales Energierecht im Gebäudebereich, das erst mit Überführung in kantonales Recht bindend wird — strukturell dem deutschen MBO/MVV-TB-Mechanismus verwandt, aber ohne Bundeskompetenz im Hintergrund (Energierecht im Gebäudebereich ist reine Kantonskompetenz). Die aktuelle Fassung MuKEn 2025 wurde am **29.08.2025** verabschiedet (korrigiert — die ursprünglich hier genannte 04.04.2025 ist per Live-Prüfung der endk.ch-Verabschiedungsmeldung widerlegt); der kantonale Umsetzungsstand wurde in dieser Sitzung nicht einzeln verifiziert.
- Wortlautbeleg (Originalsprache): "Die Kantone sind gehalten, die MuKEn in ihrem Kanton umzusetzen" (Sekundärquelle EnDK/HEV, Paraphrase — MuKEn-Modultext selbst nicht eingesehen)
- Beleg-Quelle: B3 · Zugänglichkeit: frei-primär (endk.ch), in dieser Sitzung nicht im Volltext gelesen · Bindungsakt: Bindungsmechanismus existiert (kantonale Umsetzungspflicht strukturell beschrieben), Listung im Einzelfall nicht verifiziert
- Quelle: Tier 1 (endk.ch, amtsnahe interkantonale Konferenz) · https://endk.ch/energiepolitik/ ; Verabschiedungsdatum verifiziert über https://endk.ch/die-kantone-verabschieden-die-mustervorschriften-2025-und-beschreiten-den-pfad-der-energiewende-konsequent-weiter/ · Fassung(as-amended) 2025-08-29 (MuKEn 2025, korrigiert) · Zugriff 2026-08-11 (Ersterhebung) / 2026-08-13 (Korrekturprüfung)
- Status: Übergang · MuKEn 2025 verabschiedet, kantonale Umsetzung teils ausstehend
- Sub-Ebene: Stichprobe [nicht einzeln geprüft] / nicht erhoben [alle 26 Kantone]
- Relationen: wird kombiniert mit / ergänzt REG-CH-6-001 (IVHB, gemeinsamer Vollzugskontext Baubewilligung)
- Konfidenz: unklar (B3, kein Primärtext geprüft)

### REG-CH-6-001 · IVHB (Interkantonale Vereinbarung über die Harmonisierung der Baubegriffe)

- Titel: Interkantonale Vereinbarung über die Harmonisierung der Baubegriffe (IVHB), verabschiedet von der BPUK am 22.09.2005
- Fundstelle: Gesamtvereinbarung (30 harmonisierte Baubegriffe und Messweisen, Anhang 1)
- A: sub-national · A-Ursprung: sub-national (BPUK-Konkordat) · Downstream-Verifikationsstatus: verifiziert in [18 beigetretene Kantone laut ivhb.ch: AG, AI, BE, BL, FR, GR, JU, LU, NE, NW, OW, SH, SO, SZ, TG, UR, VS, ZG]; ZH ausdrücklich **nicht** beigetreten, wendet aber laut Sekundärquelle 29 von 30 Begriffen eigenständig an
- B: Primärfeld 6 · Normtyp: Grundnorm/Begriffsnorm (definiert Mess-/Begriffsstandards, von denen die Anwendung kantonaler Ausnützungs-/Abstandsvorschriften abhängt) · Nebenfelder: 2
- C: materialübergreifend
- D: Gesetz (Konkordat mit unmittelbarer Bindungswirkung in den beigetretenen Kantonen — s. Begründung zu REG-CH-6-003)
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): schweigend · Bezugsgegenstand: Bauteil-/Bestandsbau-Wiederverwendung wird in der IVHB nicht thematisiert — sie regelt ausschliesslich Messweisen (Gebäudehöhe, Geschosszahl, Abstände u. Ä.), keine stofflichen oder produktbezogenen Fragen
- F2 (E3): schweigend · Bezugsgegenstand: dieselbe Fallgruppe — kein erkennbarer mittelbarer Reuse-Bezug
- G: entfällt (reine Begriffs-/Messnorm ohne eigenen materiellen Nachweistatbestand)
- Kernaussage: Die IVHB vereinheitlicht 30 zentrale Baubegriffe und Messweisen (u. a. Gebäudehöhe, Voll- und Attikageschoss, Grenzabstände) unter den 18 beigetretenen Kantonen, ohne die zulässigen Masse selbst festzulegen — das bleibt Sache der Kantone/Gemeinden. Für die reuse-rechtliche Fragestellung dieses Projekts ist die IVHB nur am Rand relevant: sie schafft einheitliche Verfahrensbegriffe für Baubewilligungen, adressiert aber keine Bestandsbau- oder Wiederverwendungsfrage.
- Wortlautbeleg (Originalsprache): "Mit der Interkantonalen Vereinbarung über die Harmonisierung der Baubegriffe (IVHB) werden gesamtschweizerisch die wichtigsten Baubegriffe und Messweisen vereinheitlicht" (ivhb.ch, Zweckbeschreibung)
- Beleg-Quelle: B1 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Konkordat selbst ist der bindende Akt, direkt in Kraft in den Beitrittskantonen)
- Quelle: Tier 1 · http://ivhb.ch/ · Fassung(as-amended) 2010-11-26 (Inkrafttreten) · Zugriff 2026-08-11
- Status: in Kraft · seit 2010-11-26
- Sub-Ebene: Stichprobe [ZH — kein Beitritt, faktische Anwendung 29/30 Begriffe · BE — Beitritt bestätigt · GE, VD, TI — Beitrittsstatus in dieser Sitzung **nicht** geprüft] / nicht erhoben [übrige 22 Kantone]
- Relationen: kein direkter Reuse-Bezug identifiziert; keine Relation zu anderen CH-Objekten dieser Liste
- Konfidenz: gesichert (Zweck/Struktur), abgeleitet (Beitrittsliste, nicht für alle 4 Stichprobenkantone einzeln primärquellenbasiert geprüft)

### REG-CH-3-007 · USG Art. 7 Abs. 6 (Abfallbegriff, Grundnorm)

- Titel: Bundesgesetz vom 7. Oktober 1983 über den Umweltschutz (Umweltschutzgesetz, USG), Art. 7 Abs. 6
- Fundstelle: Art. 7 Abs. 6 und 6bis; SR 814.01
- A: national
- B: Primärfeld 3 · Normtyp: Grundnorm/Begriffsnorm (bestimmt Anwendbarkeit von VVEA und aller nachgelagerten Abfallrechtsakte)
- C: materialübergreifend
- D: Gesetz
- E: Abfallstatus · E-Wirkung: durchläuft
- F1 (E3): bedingend · Bezugsgegenstand: Abfallstatus knüpft an den subjektiven "Entledigungswillen" des Inhabers an — ein ausgebautes, intaktes Bauteil, das der Inhaber nicht entledigen, sondern weiterverwenden will, fällt tatbestandlich **nicht** zwingend unter den Abfallbegriff; die Norm eröffnet damit einen Ermöglichungspfad, macht ihn aber von einer im Einzelfall zu klärenden Willens-/Interessensfrage abhängig
- F2 (E3): bedingend · Bezugsgegenstand: dieselbe Fallgruppe — in der Vollzugspraxis ist der Nachweis fehlenden Entledigungswillens bei Rückbaumaterial ein bekannter Streitpunkt (analog zur DE-KrWG-§3-Diskussion), ohne dass dies für CH in dieser Sitzung primärquellenbasiert vertieft wurde
- G: Statusfeststellung/Anwendbarkeitsprüfung (Wert 9) (inferiert=E3 — der Nachweis fehlenden Entledigungswillens für ein konkretes Bauteil ist im Vollzug zu erbringen, im Gesetzestext selbst aber nicht als eigenständiges Verfahren ausgestaltet)
- Kernaussage: Art. 7 Abs. 6 USG definiert Abfall über den subjektiven Entledigungswillen des Inhabers oder ein öffentliches Interesse an der Entsorgung — strukturell dem deutschen KrWG-§3-Mechanismus vergleichbar. Diese Grundnorm entscheidet, ob ein ausgebautes Bauteil überhaupt in den Anwendungsbereich der VVEA (REG-CH-3-008 ff.) fällt, bevor es wiederverwendet werden kann.
- Wortlautbeleg (Originalsprache): "Abfälle sind bewegliche Sachen, deren sich der Inhaber entledigt oder deren Entsorgung im öffentlichen Interesse geboten ist." (Art. 7 Abs. 6 USG)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/1984/1122_1122_1122/de (Volltext bezogen über https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/1984/1122_1122_1122/20220101/de/pdf-a/fedlex-data-admin-ch-eli-cc-1984-1122_1122_1122-20220101-de-pdf-a-8.pdf) · Fassung(as-amended) 2022-01-01 (in dieser Sitzung gelesene Fassung; genauerer 2026-Stand nicht gegengeprüft) · Zugriff 2026-08-11
- Status: in Kraft · seit 1985 (Grundfassung), Art. 7 zuletzt novelliert vor Stand 2022
- Sub-Ebene: entfällt (A=national)
- Relationen: determiniert Anwendbarkeit von REG-CH-3-008, REG-CH-3-009, REG-CH-3-010
- Konfidenz: gesichert

### REG-CH-3-008 · VVEA Art. 12 (Allgemeine Verwertungspflicht)

- Titel: Verordnung vom 4. Dezember 2015 über die Vermeidung und die Entsorgung von Abfällen (Abfallverordnung, VVEA), Art. 12
- Fundstelle: Art. 12 Abs. 1–2; SR 814.600
- A: national
- B: Primärfeld 3
- C: materialübergreifend
- D: RVO
- E: Aufbereitung/Prüfung · E-Wirkung: durchläuft
- F1 (E3): ermöglichend · Bezugsgegenstand: stoffliche Verwertungspflicht, sofern umweltschonender als Alternativen — eröffnet grundsätzlich den Weg zur Wiederverwertung, adressiert aber die stoffliche Verwertung (Recycling), nicht die bauteilerhaltende Wiederverwendung im engeren Sinn
- F2 (E3): schweigend · Bezugsgegenstand: Wiederverwendung als eigenständige, dem Recycling vorgelagerte Stufe der Abfallhierarchie wird im VVEA-Text (anders als in EU-WFD Art. 4) nicht als eigener Begriff geführt — die Norm kennt "Vermeidung", "Verwertung" (stofflich/energetisch) und "Ablagerung", aber keine Zwischenkategorie "Vorbereitung zur Wiederverwendung"
- G: rechnerischer Nachweis (explizit=E1 — Art. 12 Abs. 1 verlangt eine Umweltbelastungsvergleichsrechnung zwischen Verwertung und anderer Entsorgung/Neuherstellung)
- Kernaussage: Art. 12 VVEA verpflichtet zur stofflichen oder energetischen Verwertung von Abfällen nach Stand der Technik, wenn dies umweltschonender ist als Ablagerung oder Neuherstellung. Die Norm ist auf Materialverwertung ausgerichtet; eine eigene Kategorie "Vorbereitung zur Wiederverwendung" (wie in der EU-Abfallrahmenrichtlinie) kennt das schweizerische Abfallrecht an dieser Stelle nicht.
- Wortlautbeleg (Originalsprache): "Abfälle sind stofflich oder energetisch zu verwerten, wenn eine Verwertung die Umwelt weniger belastet als: a. eine andere Entsorgung; und b. die Herstellung neuer Produkte oder die Beschaffung anderer Brennstoffe." (Art. 12 Abs. 1 VVEA)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2015/891/de (Volltext bezogen über https://www.fedlex.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2015/891/20220401/de/pdf-a/fedlex-data-admin-ch-eli-cc-2015-891-20220401-de-pdf-a-1.pdf) · Fassung(as-amended) 2022-04-01 · Zugriff 2026-08-11
- Status: in Kraft · seit 2016-01-01
- Sub-Ebene: entfällt
- Relationen: determiniert Anwendbarkeit von REG-CH-3-009/-010; kollidiert mit keiner anderen Norm dieser Liste
- Konfidenz: gesichert

### REG-CH-3-009 · VVEA Art. 16 (Angaben zur Entsorgung von Bauabfällen)

- Titel: Abfallverordnung (VVEA), Art. 16
- Fundstelle: Art. 16 Abs. 1–2; SR 814.600
- A: national
- B: Primärfeld 3 · Nebenfelder: 2 (Baubewilligungsverfahren)
- C: materialübergreifend
- D: RVO
- E: Bestandserkundung, Planung/Nachweis · E-Wirkung: erzwingt (Meldepflicht ab Schwellenwert unabhängig davon, ob ein kürzerer Weg für den Bauherrn denkbar wäre)
- F1 (E3): bedingend · Bezugsgegenstand: Meldepflicht zu Art, Qualität und Menge anfallender Bauabfälle im Baubewilligungsgesuch — schafft Transparenz, die eine gezielte Ausbauplanung für Wiederverwendung erleichtern könnte, verlangt aber selbst keine Wiederverwendungsprüfung
- F2 (E3): bedingend · Bezugsgegenstand: dieselbe Fallgruppe — in der Praxis (s. auch kantonale Konkretisierung REG-CH-3-021/TI) wird diese Meldung meist als reine Entsorgungs-, nicht als Wiederverwendungsplanung verstanden
- G: Dokumentenlage (explizit=E1)
- Kernaussage: Bei Bauvorhaben mit voraussichtlich mehr als 200 m³ Bauabfall oder mit umwelt-/gesundheitsgefährdenden Stoffen (PCB, PAK, Blei, Asbest) muss die Bauherrschaft der Baubewilligungsbehörde Angaben zu Art, Qualität und Menge der Abfälle sowie zur vorgesehenen Entsorgung machen. Diese Norm ist der zentrale bundesrechtliche Ankerpunkt für ein Rückbau-/Entsorgungskonzept, wird aber kantonal unterschiedlich konkretisiert (s. REG-CH-3-021 für TI mit zusätzlichem Baujahr-1991-Trigger).
- Wortlautbeleg (Originalsprache): "Bei Bauarbeiten muss die Bauherrschaft der für die Baubewilligung zuständigen Behörde im Rahmen des Baubewilligungsgesuchs Angaben über die Art, Qualität und Menge der anfallenden Abfälle und über die vorgesehene Entsorgung machen, wenn: a. voraussichtlich mehr als 200 m3 Bauabfälle anfallen; oder b. Bauabfälle mit umwelt- oder gesundheitsgefährdenden Stoffen wie polychlorierte Biphenyle (PCB), polycyclische aromatische Kohlenwasserstoffe (PAK), Blei oder Asbest zu erwarten sind." (Art. 16 Abs. 1 VVEA)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2015/891/de · Fassung(as-amended) 2022-04-01 · Zugriff 2026-08-11
- Status: in Kraft · seit 2016-01-01
- Sub-Ebene: entfällt (Bundesnorm; kantonaler Vollzug s. REG-CH-3-021)
- Relationen: konkretisiert von REG-CH-3-021 (TI RLE Art. 9 lit. n)
- Konfidenz: gesichert

### REG-CH-3-010 · VVEA Art. 17 (Trennung von Bauabfällen)

- Titel: Abfallverordnung (VVEA), Art. 17
- Fundstelle: Art. 17 Abs. 1–3; SR 814.600
- A: national
- B: Primärfeld 3
- C: materialübergreifend
- D: RVO
- E: Rückbau/Sicherung, Aufbereitung/Prüfung · E-Wirkung: erzwingt
- F1 (E3): bedingend · Bezugsgegenstand: sortenreine Baustellentrennung (Ausbauasphalt, Betonabbruch, Straßenaufbruch, Mischabbruch, Ziegelbruch, Gips separat; Glas/Metalle/Holz/Kunststoffe separat) erleichtert grundsätzlich eine spätere hochwertige Verwertung, ist aber materialstrom-, nicht bauteilbezogen konzipiert — ein sortenrein "getrenntes" Fenster ist im Normtext nicht vorgesehen, nur seine Materialfraktionen nach Zerlegung
- F2 (E3): hemmend · Bezugsgegenstand: dieselbe Fallgruppe — die Pflicht zur Fraktionstrennung setzt strukturell eine vorgängige **Zerlegung** in Materialklassen voraus, was für den Erhalt intakter, wiederverwendbarer Bauteile (die vor der Zerlegung ausgebaut werden müssten) tendenziell ein Gegenanreiz ist, wenn Rückbauunternehmen ökonomisch auf die in Art. 17 vorgesehenen Fraktionen hin optimieren — E3-Einschätzung
- G: Sichtprüfung (explizit=E1 — "möglichst sortenrein" impliziert eine Vor-Ort-Sichtkontrolle/Sortierung)
- Kernaussage: Art. 17 VVEA schreibt die betriebliche Trennung von Bauabfällen auf der Baustelle in sechs Fraktionen vor (u. a. Ausbauasphalt/Betonabbruch/Mischabbruch, Glas/Metalle/Holz/Kunststoffe, brennbare Reste, übrige Abfälle) und erlaubt der Behörde, eine weitergehende Trennung zu verlangen. Die Norm ist material-, nicht bauteilorientiert konzipiert; ihr strukturelles Verhältnis zur bauteilerhaltenden Wiederverwendung (die einer Fraktionierung typischerweise vorgelagert sein müsste) ist im Text nicht geregelt.
- Wortlautbeleg (Originalsprache): "Bei Bauarbeiten sind Sonderabfälle von den übrigen Abfällen zu trennen und separat zu entsorgen. Die übrigen Bauabfälle sind auf der Baustelle wie folgt zu trennen: […] c. Ausbauasphalt, Betonabbruch, Strassenaufbruch, Mischabbruch, Ziegelbruch und Gips, jeweils möglichst sortenrein" (Art. 17 Abs. 1 VVEA)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2015/891/de · Fassung(as-amended) 2022-04-01 · Zugriff 2026-08-11
- Status: in Kraft · seit 2016-01-01
- Sub-Ebene: entfällt (Bundesnorm; Gemeinden können laut kantonalem AbfG z. B. ZH § 16a weitergehende Trennung verlangen, s. REG-CH-3-015)
- Relationen: wird kombiniert mit / ergänzt REG-CH-3-009
- Konfidenz: gesichert

### REG-CH-5a-011 · BöB Art. 29/30 (Zuschlagskriterien und technische Spezifikationen)

- Titel: Bundesgesetz vom 21. Juni 2019 über das öffentliche Beschaffungswesen (BöB), Art. 2, 29 und 30
- Fundstelle: Art. 2 (Zweck), Art. 29 (Zuschlagskriterien), Art. 30 (Technische Spezifikationen); SR 172.056.1
- A: national
- B: Primärfeld 5a
- C: materialübergreifend
- D: Gesetz
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): ermöglichend · Bezugsgegenstand: Art. 29 Abs. 1 nennt "Nachhaltigkeit" ausdrücklich als zulässiges Zuschlagskriterium neben Preis/Qualität; Art. 30 erlaubt der Auftraggeberin, technische Spezifikationen (die Kreislaufwirtschafts-/Reuse-Anforderungen tragen könnten) frei festzulegen — die Totalrevision 2021 hat damit die Grundlage für Kreislaufwirtschaftskriterien geschaffen, ohne Wiederverwendung als eigenen, benannten Kriterientyp vorzuschreiben
- F2 (E3): bedingend · Bezugsgegenstand: dieselbe Fallgruppe — ob und wie stark einzelne Vergabestellen Reuse-Kriterien tatsächlich einsetzen, hängt von der Ausschreibungspraxis ab, die das Gesetz selbst offenlässt
- G: Dokumentenlage (explizit=E1 — Zuschlagskriterien und deren Gewichtung müssen laut Art. 29 Abs. 3 in der Ausschreibung offengelegt werden)
- Kernaussage: Die 2021 in Kraft getretene Totalrevision des BöB verankert in Art. 2 ausdrücklich die "ökologisch […] nachhaltige Verwendung der öffentlichen Mittel" als Gesetzeszweck und erlaubt in Art. 29 f. Nachhaltigkeit als Zuschlagskriterium sowie freie technische Spezifikationen. Das schafft die rechtliche Grundlage, Kreislaufwirtschafts-/Wiederverwendungskriterien in öffentliche Bauausschreibungen aufzunehmen, ohne dies verbindlich vorzuschreiben — die Norm ist ermöglichend, nicht bedingend im engeren Sinn einer Pflicht.
- Wortlautbeleg (Originalsprache): "Die Auftraggeberin prüft die Angebote anhand leistungsbezogener Zuschlagskriterien. Sie berücksichtigt, unter Beachtung der internationalen Verpflichtungen der Schweiz, neben dem Preis und der Qualität einer Leistung, insbesondere Kriterien wie Zweckmässigkeit, Termine, technischer Wert, Wirtschaftlichkeit, Lebenszykluskosten, Ästhetik, Nachhaltigkeit […]" (Art. 29 Abs. 1 BöB)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2020/126/de (Volltext bezogen über https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2020/126/20210101/de/pdf-a/fedlex-data-admin-ch-eli-cc-2020-126-20210101-de-pdf-a.pdf) · Fassung(as-amended) 2021-01-01 · Zugriff 2026-08-11
- Status: in Kraft · seit 2021-01-01
- Sub-Ebene: entfällt (Bundesgesetz; kantonale/kommunale Beschaffung läuft über die separate IVöB, in dieser Sitzung nicht geprüft — Lücke)
- Relationen: keine Relation zu anderen CH-Objekten dieser Liste identifiziert
- Konfidenz: gesichert

### REG-CH-7-012 · OR Art. 365 (Werkvertrag, Stoffmängelhaftung)

- Titel: Bundesgesetz betreffend die Ergänzung des Schweizerischen Zivilgesetzbuches (Fünfter Teil: Obligationenrecht, OR), Art. 365
- Fundstelle: Art. 365 Abs. 1–2; SR 220
- A: national
- B: Primärfeld 7
- C: materialübergreifend
- D: Gesetz
- E: Einbau/Abnahme · E-Wirkung: durchläuft
- F1 (E3): bedingend · Bezugsgegenstand: Übernimmt der Unternehmer die Lieferung des (auch wiederverwendeten) Baustoffs, haftet er dem Besteller "für die Güte desselben […] wie ein Verkäufer" — bei fehlender Leistungserklärung/Herkunftsdokumentation für ein wiederverwendetes Bauteil ist diese Gewährleistung schwerer zu erfüllen, ohne dass das Gesetz Reuse-Bauteile ausdrücklich anders behandelt als neue
- F2 (E3): hemmend · Bezugsgegenstand: dieselbe Fallgruppe — die praktische Unsicherheit über Eigenschaften/Restlebensdauer wiederverwendeter Bauteile erschwert dem Unternehmer, die verkäufergleiche Gewährleistung risikoadäquat zu kalkulieren (E3, keine textbelegte Reuse-Sonderregel)
- G: Erklärung Dritter (inferiert=E3 — in der Praxis wird die Werkstoffqualität regelmässig über eine Herstellererklärung/Produktdokumentation nachgewiesen; das Gesetz selbst schreibt keine bestimmte Nachweisform vor)
- Kernaussage: Übernimmt der Werkunternehmer die Materiallieferung, haftet er nach OR Art. 365 wie ein Verkäufer für deren Güte. Diese Norm ist materialneutral formuliert und unterscheidet nicht zwischen neuen und wiederverwendeten Baustoffen — die Haftung träfe den Unternehmer bei mangelhaften wiederverwendeten Bauteilen im selben Umfang, was in der Praxis (analog zur DE-VOB/B-Diskussion) als Erschwernis für den Reuse-Einsatz gilt.
- Wortlautbeleg (Originalsprache): "Soweit der Unternehmer die Lieferung des Stoffes übernommen hat, haftet er dem Besteller für die Güte desselben und hat Gewähr zu leisten wie ein Verkäufer." (Art. 365 Abs. 1 OR)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/27/317_321_377/de (Volltext bezogen über https://www.fedlex.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/27/317_321_377/20230209/de/pdf-a/fedlex-data-admin-ch-eli-cc-27-317_321_377-20230209-de-pdf-a.pdf) · Fassung(as-amended) 2023-02-09 · Zugriff 2026-08-11
- Status: in Kraft · seit 1912 (Grundfassung), Art. 365 seither nicht novelliert
- Sub-Ebene: entfällt
- Relationen: wird kombiniert mit / ergänzt REG-CH-7-013
- Konfidenz: gesichert

### REG-CH-7-013 · OR Art. 371 Abs. 1 (Werkvertrag, Verjährung bei integriertem beweglichem Werk)

- Titel: Obligationenrecht (OR), Art. 371 Abs. 1
- Fundstelle: Art. 371 Abs. 1; SR 220
- A: national
- B: Primärfeld 7
- C: materialübergreifend · Verbund-/Systembauteil (sofern das reuse-Bauteil als Ganzes, nicht nach Einzelmaterial, Gegenstand der Mangelhaftigkeit ist)
- D: Gesetz
- E: Betrieb/Dokumentation · E-Wirkung: durchläuft
- F1 (E3): bedingend · Bezugsgegenstand: Verlängert die Verjährungsfrist für Mängel eines **beweglichen** Werks (z. B. eines wiederverwendeten Bauteils), das bestimmungsgemäss in ein unbewegliches Werk integriert wurde und dessen Mangelhaftigkeit verursacht, von der sonst kürzeren beweglichen-Sachen-Frist auf fünf Jahre — das erhöht das Haftungsrisiko für den Einbau wiederverwendeter Komponenten strukturell auf das Niveau der Bauwerksgewährleistung
- F2 (E3): hemmend · Bezugsgegenstand: dieselbe Fallgruppe — fünf Jahre Gewährleistung auf ein wiederverwendetes, oft nicht mehr mit einer Leistungserklärung versehenes Bauteil erschwert dessen Einsatz durch Unternehmer/Planer, die dieses Risiko einpreisen müssen (E3, Projekteinschätzung analog zur belgischen Wet-Peeters-Borsus-/Dezennalhaftungs-Diskussion, s. BE-Recherche)
- G: entfällt (reine Fristnorm ohne eigenen Nachweistatbestand)
- Kernaussage: OR Art. 371 Abs. 1 setzt für Mängel eines beweglichen, in ein unbewegliches Werk integrierten Werkteils eine fünfjährige Verjährungsfrist an, statt der kürzeren Frist für rein bewegliche Werke. Für den Wiedereinbau eines gebrauchten Bauteils bedeutet das: Sobald es Bestandteil eines Bauwerks wird und einen Mangel des Gesamtwerks verursacht, haftet der Unternehmer fünf Jahre lang — unabhängig davon, ob für das ursprüngliche Bauteil eine Herstellergewährleistung überhaupt noch besteht.
- Wortlautbeleg (Originalsprache): "Soweit jedoch Mängel eines beweglichen Werkes, das bestimmungsgemäss in ein unbewegliches Werk integriert worden ist, die Mangelhaftigkeit des Werkes verursacht haben, beträgt die Verjährungsfrist fünf Jahre." (Art. 371 Abs. 1 OR, zweiter Satz)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/27/317_321_377/de · Fassung(as-amended) 2023-02-09 · Zugriff 2026-08-11
- Status: in Kraft · Fristverlängerung seit 2013-01-01 (Novelle vom 16.03.2012)
- Sub-Ebene: entfällt
- Relationen: wird kombiniert mit / ergänzt REG-CH-7-012
- Konfidenz: gesichert

---

## B. KANTONSSTICHPROBE (Stichprobe-und-Deklaration, kein Vollerhebungsanspruch)

**Deklaration der erhobenen/nicht erhobenen Einheiten (Pflicht laut Freeze Abschnitt 8):** Erhoben in dieser Sitzung: **ZH, GE, BE, TI** mit primär- oder mindestens amtsnaher Quelle; **VD** nur mit dünner, überwiegend sekundärer Beleglage (Web-Suchbudget während der VD-Recherche erschöpft — s. u.). Nicht erhoben: alle übrigen 21 Kantone. Gemeindeebene (in vielen Kantonen die tatsächliche Vollzugsebene für Baubewilligungen) wurde **nirgends** erhoben — als generelle Lücke für alle fünf Stichprobenkantone zu verstehen, nicht nur für VD.

### Zürich (ZH)

### REG-CH-2-014 · PBG § 220 (Ausnahmebewilligung)

- Titel: Planungs- und Baugesetz des Kantons Zürich (PBG), § 220
- Fundstelle: § 220 (genauer Wortlaut in dieser Sitzung **nicht** primärquellenbasiert verifiziert, s. Beleg-Quelle)
- A: sub-national · Downstream-Verifikationsstatus: entfällt (Landesgesetz, keine Muster-Transformation)
- B: Primärfeld 2
- C: materialübergreifend
- D: Gesetz
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): bedingend · Bezugsgegenstand: § 220 PBG erlaubt Ausnahmebewilligungen von Bauvorschriften in besonders gelagerten Einzelfällen; laut Sekundärquelle (Fachkommentar) wird diese Norm in der Vollzugspraxis diskutiert als möglicher — aber unsicherer — Ansatzpunkt für Kreislaufwirtschafts-/Reuse-Projekte, die von Standardvorschriften abweichen müssen
- F2 (E3): hemmend · Bezugsgegenstand: dieselbe Fallgruppe — laut dem eingesehenen AWEL-Schlussbericht "regulatorische Hemmnisse der Kreislaufwirtschaft" ist unklar, ob und wie oft Ausnahmebewilligungen für Kreislaufwirtschaftsprojekte tatsächlich erteilt werden; der Bericht wertet § 220 PBG als bestehenden, aber nicht auf Reuse zugeschnittenen und daher unsicheren Ausweg
- G: Einzelfallzulassung (inferiert=E3)
- Kernaussage: § 220 PBG ist die allgemeine Ausnahmebestimmung des Zürcher Planungs- und Baurechts für besonders gelagerte Verhältnisse. Ein amtlicher AWEL-Bericht zu regulatorischen Hemmnissen der Kreislaufwirtschaft benennt diese Norm als möglichen, aber praxisunsicheren Ansatzpunkt, wenn wiederverwendete Bauteile nicht die aktuellen Normen/Standards erfüllen. Der PBG-Wortlaut selbst wurde in dieser Sitzung nicht im Volltext gelesen (Zugriffsproblem auf zhlex-PDF).
- Wortlautbeleg (Originalsprache): "Wiederverwendete Bauteile gelten als Bauprodukte im Sinne des Gesetzes, wenn sie wieder in Verkehr gebracht werden […] Ausnahmen sind im Gesetz vorgesehen (§ 220 PBG)" (Paraphrase aus AWEL-Schlussbericht "Regulatorische Hemmnisse der Kreislaufwirtschaft", 2026 — kein wörtliches PBG-Zitat, da Primärtext nicht gelesen)
- Beleg-Quelle: B3 · Zugänglichkeit: frei-primär (zhlex.zh.ch), in dieser Sitzung technisch nicht auslesbar (PDF-Fetch lieferte keinen lesbaren Text; pdftotext-Konversion ergab keine Treffer für "§ 220" — vermutlich falsches/veraltetes PDF-Dokument abgerufen) · Bindungsakt: entfällt
- Quelle: Tier 1 (Existenz/Fundstelle) + Tier 2 (AWEL-Verwaltungsbericht, https://www.zh.ch/content/dam/zhweb/bilder-dokumente/themen/umwelt-tiere/abfall-rohstoffe/abfallwirtschaft/kreislaufwirtschaft/schlussbericht_regulatorische_hemmnisse_der_klw_awel_2026.pdf) · Fassung(as-amended) nicht verifiziert · Zugriff 2026-08-11
- Status: in Kraft · Datum unbekannt
- Sub-Ebene: Stichprobe [ZH] / nicht erhoben [25 übrige Kantone]
- Relationen: kollidiert mit keiner anderen Norm; strukturelles Analogon zu REG-CH-2-016/-017 (GE LCI Art. 117/118, dort aber ermöglichend statt nur ausnahmsweise)
- Konfidenz: unklar (Primärtext nicht verifiziert — echte Lücke)

### REG-CH-3-015 · ZH Abfallgesetz § 16a (Bauabfälle)

- Titel: Abfallgesetz des Kantons Zürich (AbfG), § 1, § 2, § 16a
- Fundstelle: § 1 (Zweck), § 16a (Entsorgung von Bauabfällen); LS/Ordnungsnummer 712.1
- A: sub-national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 3
- C: materialübergreifend
- D: Gesetz
- E: Rückbau/Sicherung, Aufbereitung/Prüfung · E-Wirkung: durchläuft
- F1 (E3): bedingend · Bezugsgegenstand: § 16a Abs. 1 erlaubt den Gemeinden, eine über die VVEA-Trennpflicht (REG-CH-3-010) hinausgehende Trennung auf der einzelnen Baustelle zu verlangen — eröffnet kommunalen Spielraum, der Reuse-Trennung im Prinzip einschliessen könnte, ohne sie ausdrücklich zu benennen
- F2 (E3): schweigend · Bezugsgegenstand: dieselbe Fallgruppe — ob einzelne Zürcher Gemeinden diese Möglichkeit tatsächlich für Reuse-Trennung nutzen, wurde nicht erhoben
- G: Dokumentenlage (explizit=E1, für § 16a Abs. 2 Nachweispflicht bei Aushub aus belasteten Standorten) / Sichtprüfung (inferiert=E3, für die gemeindliche Trennkontrolle)
- Kernaussage: Das kantonale Abfallgesetz Zürich regelt die Abfallwirtschaft in Ausführung und Ergänzung der Bundesgesetzgebung (USG/VVEA) und erlaubt den Gemeinden in § 16a, für einzelne Baustellen eine weitergehende Abfalltrennung zu verlangen, als die VVEA bundesrechtlich vorschreibt. Der hier eingesehene Text stammt aus der ursprünglichen Fassung von 1994 (§ 16a mit Fussnotenverweis auf spätere Änderungen) — der aktuelle Stand zum Stichtag 2026-08-11 wurde nicht gegengeprüft, echte Lücke.
- Wortlautbeleg (Originalsprache): "Die Gemeinden können eine weitergehende Trennung der Abfälle auf der einzelnen Baustelle verlangen." (§ 16a Abs. 1 AbfG ZH)
- Beleg-Quelle: B1 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.zh.ch/de/politik-staat/gesetze-beschluesse/gesetzessammlung/zhlex-ls/erlass-712_1-1994_09_25-2001_01_01-044.html (Volltext bezogen über http://www2.zhlex.zh.ch/Appl/zhlex_r.nsf/0/4E0BDDA2F59E42DBC125774C0048D57A/$file/712.1_25.9.94_69.pdf) · Fassung(as-amended) **unklar — eingesehene PDF-Fassung trägt Stand 1994 mit punktuellen Fussnoten-Änderungsvermerken bis ca. 2018, kein gesicherter 2026-Stand** · Zugriff 2026-08-11
- Status: in Kraft · seit 1994, novelliert (Einzeländerungen nicht vollständig nachvollzogen)
- Sub-Ebene: Stichprobe [ZH] / nicht erhoben [25 übrige Kantone]
- Relationen: konkretisiert REG-CH-3-010 (VVEA Art. 17)
- Konfidenz: abgeleitet (Fassungsstand unsicher)

### Genève (GE)

### REG-CH-2-016 · LCI Art. 117 (Empreinte carbone, Réemploi-Priorität)

- Titel: Loi sur les constructions et les installations diverses (LCI), Art. 117
- Fundstelle: Art. 117 Abs. 1–2 (genaue Absatzstruktur in dieser Sitzung nur über Fetch-Zusammenfassung erschlossen, s. Beleg-Quelle); rsGE L 5 05
- A: sub-national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 2 · Nebenfelder: 4 (Klimaschutz/Ressourcenschonung)
- C: materialübergreifend
- D: Gesetz
- E: Planung/Nachweis · E-Wirkung: erzwingt (verpflichtender Grundsatz für "toute construction ou rénovation importante")
- F1 (E3): ermöglichend · Bezugsgegenstand: ordnet die Wiederverwendung bestehender Baustoffe ausdrücklich als **erste** Priorität vor Recycling/CO₂-armen Neumaterialien an — die stärkste bisher in dieser CH-Recherche gefundene textliche Reuse-Priorisierung auf Gesetzesebene
- F2 (E3): bedingend · Bezugsgegenstand: dieselbe Fallgruppe — die Wirksamkeit hängt von der Ausführungsverordnung (REG-CH-2-017) ab, die Berechnungsmodalitäten und Schwellenwerte erst noch im Detail festlegt; ohne diese bleibt Art. 117 im Vollzug konkretisierungsbedürftig
- G: rechnerischer Nachweis (explizit=E1 — Bilanzierung der Treibhausgasemissionen ("empreinte carbone") über den Lebenszyklus)
- Kernaussage: Art. 117 LCI verpflichtet seit der Gesetzesänderung von Dezember 2021 jede bedeutende Konstruktion oder Renovation im Kanton Genf, mit Materialien geplant und ausgeführt zu werden, die den CO₂-Fussabdruck minimieren — mit der ausdrücklichen Vorgabe, in erster Linie bestehende Materialien wiederzuverwenden. Dies ist ein expliziter, gesetzlich verankerter Reuse-Vorrang, der in dieser CH-Stichprobe kein Gegenstück auf Bundesebene hat.
- Wortlautbeleg (Originalsprache): "Toute construction ou rénovation importante doit être conçue et réalisée à base de matériaux propres à minimiser son empreinte carbone." (Art. 117 Abs. 1 LCI, zitiert nach silgeneve.ch)
- Beleg-Quelle: B1 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://silgeneve.ch/legis/data/rsg_l5_05.htm · Fassung(as-amended) 2025-10-22 (letzte laut Portal erfasste Änderung) · Zugriff 2026-08-11
- Status: in Kraft · seit 2021-12 (Grosser Rat-Beschluss), seither novelliert (u. a. Art. 118 Förderungsöffnung)
- Sub-Ebene: Stichprobe [GE] / nicht erhoben [25 übrige Kantone]
- Relationen: konkretisiert von REG-CH-2-017 (Règlement d'application); strukturelles Gegenstück zu REG-CH-2-014 (ZH § 220, dort nur Ausnahme statt Grundsatz)
- Konfidenz: gesichert (Existenz/Kernsatz durch Portal bestätigt), abgeleitet (vollständige Absatzstruktur nicht Wort für Wort verifiziert)

### REG-CH-2-017 · LCI Art. 118 + Règlement d'application (Berechnungsmodalitäten)

- Titel: Loi sur les constructions et les installations diverses (LCI), Art. 118; zugehöriges Ausführungsreglement (Berechnung Empreinte carbone nach SIA 390/1:2025)
- Fundstelle: Art. 118 LCI; Règlement d'application (genaue Erlassnummer in dieser Sitzung nicht verifiziert)
- A: sub-national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 2 · Nebenfelder: 6 (SIA-390/1-Bezugnahme), 5b (Fördermöglichkeit)
- C: materialübergreifend
- D: RVO (kantonales Ausführungsreglement)
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): bedingend · Bezugsgegenstand: definiert "empreinte carbone" als Treibhausgasbilanz über den gesamten Lebenszyklus und ermächtigt den Staatsrat, Berechnungsmodalitäten und CO₂-Höchstwerte je Baumaterial per Verordnung festzulegen — die materielle Reichweite (auch für Reuse-Anrechnung) hängt von dieser noch zu prüfenden Verordnung ab
- F2 (E3): schweigend · Bezugsgegenstand: dieselbe Fallgruppe — wie wiederverwendete Materialien in der SIA-390/1-Bilanzierung konkret angerechnet werden (vermutlich mit stark reduziertem Herstellungs-Fussabdruck), wurde in dieser Sitzung nicht primärquellenbasiert geprüft
- G: rechnerischer Nachweis (explizit=E1) → Bindungskette zu SIA 390/1:2025 (kostenpflichtige Norm)
- Kernaussage: Art. 118 LCI und das zugehörige Ausführungsreglement konkretisieren die Empreinte-carbone-Pflicht aus Art. 117 methodisch, gestützt auf die SIA-Norm 390/1 in der Ausgabe 2025, und öffnen zugleich die Möglichkeit finanzieller Unterstützung für Bauherrschaften, die diesen Ansatz verfolgen. Die SIA-Norm selbst ist kostenpflichtig; der freie amtliche Bindungsakt ist das Ausführungsreglement, das auf sie verweist.
- Wortlautbeleg (Originalsprache): "bilan des émissions de gaz à effet de serre de ce matériau, et cela durant l'ensemble de son cycle de vie" (Definition Empreinte carbone, Art. 118 LCI, zitiert nach silgeneve.ch-Zusammenfassung — kein vollständiger Absatz-Wortlaut verifiziert)
- Beleg-Quelle: B2 · Zugänglichkeit: frei-primär (LCI-Verordnungstext) / paywalled-nicht-eingesehen (SIA 390/1:2025) · Bindungsakt: benannt (kantonales Ausführungsreglement zu Art. 118 LCI verweist auf SIA 390/1:2025)
- Quelle: Tier 1 · https://www.ge.ch/document/materiaux-bas-carbone-solaire-confort-estival-dispositif-legal-renforce · Fassung(as-amended) 2025 (Bezugnahme SIA 390/1 Ausgabe 2025) · Zugriff 2026-08-11
- Status: in Kraft · Verordnungstext 2025 aktualisiert
- Sub-Ebene: Stichprobe [GE] / nicht erhoben [25 übrige Kantone]
- Relationen: konkretisiert REG-CH-2-016
- Konfidenz: abgeleitet (Reglementstext nicht im Volltext gelesen, nur Sekundär-/Portalzusammenfassung)

### Bern (BE)

### REG-CH-3-018 · BE Abfallverordnung Art. 18 (Entsorgungskonzept)

- Titel: Abfallverordnung des Kantons Bern (AbfV BE), Art. 18
- Fundstelle: Art. 18 (Beurteilung/Genehmigung von Entsorgungskonzepten) — genauer Wortlaut in dieser Sitzung **nicht** primärquellenbasiert verifiziert
- A: sub-national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 3
- C: materialübergreifend
- D: RVO
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): schweigend · Bezugsgegenstand: Wiederverwendung wird laut eingesehener Sekundärquelle (BVD-Themenseite) in Art. 18 nicht eigenständig thematisiert — Fokus liegt auf Entsorgungskonzept-Prüfung, nicht auf Reuse-Priorisierung
- F2 (E3): schweigend · Bezugsgegenstand: dieselbe Fallgruppe
- G: Dokumentenlage (inferiert=E3 — abgeleitet aus dem Begriff "Entsorgungskonzept", Primärtext nicht gelesen)
- Kernaussage: Der Kanton Bern konkretisiert die bundesrechtliche VVEA-Meldepflicht (REG-CH-3-009) über eine eigene Abfallverordnung, deren Art. 18 die Prüfung von Entsorgungskonzepten regelt. Die materielle Ausgestaltung — insbesondere ob und wie Wiederverwendung darin vorkommt — konnte in dieser Sitzung nicht primärquellenbasiert verifiziert werden; die kantonale Fachstelle stellt daneben unverbindliche Empfehlungen bereit (s. REG-CH-6-019).
- Wortlautbeleg (Originalsprache): nicht verfügbar — Primärtext nicht eingesehen, kein Zitat ohne Volltexteinsicht zulässig (Belegstrenge-Regel)
- Beleg-Quelle: B3 · Zugänglichkeit: frei-primär (belex.sites.be.ch), in dieser Sitzung technisch nicht auslesbar (Fetch lieferte HTTP 404 bzw. keinen lesbaren Text) · Bindungsakt: entfällt
- Quelle: Tier 1 (Existenznachweis über amtliche Themenseite) · https://www.bvd.be.ch/de/start/themen/abfall/bauabfaelle-und-recyclingbaustoffe.html · Fassung(as-amended) nicht verifiziert · Zugriff 2026-08-11
- Status: in Kraft · Datum unbekannt
- Sub-Ebene: Stichprobe [BE] / nicht erhoben [25 übrige Kantone]
- Relationen: konkretisiert REG-CH-3-009
- Konfidenz: unklar (Primärtext nicht verifiziert — echte Lücke, kein Wortlautzitat)

### REG-CH-6-019 · BE "Mineralische Recycling-Baustoffe: Verwendungsempfehlungen"

- Titel: Mineralische Recycling-Baustoffe: Verwendungsempfehlungen, 3. Ausgabe 2024 (Kanton Bern, Bau- und Verkehrsdirektion)
- Fundstelle: Gesamtdokument
- A: sub-national
- B: Primärfeld 6 · Nebenfelder: 5b
- C: Stahlbeton/Fertigteile, Mauerwerk/mineralisch
- D: Merkblatt
- E: Aufbereitung/Prüfung, Einbau/Abnahme · E-Wirkung: durchläuft
- F1 (E3): ermöglichend · Bezugsgegenstand: unverbindliche Fachempfehlung zur Verwendung mineralischer RC-Baustoffe (nicht: reuse-fähiger intakter Bauteile) in Hoch- und Tiefbau — senkt faktisch die Hemmschwelle für Sekundärbaustoffeinsatz, ohne eine Rechtspflicht zu begründen
- F2 (E3): ermöglichend · Bezugsgegenstand: dieselbe Fallgruppe — als Vollzugshilfe wirkt sie praxisnah unterstützend, bleibt aber auf Materialrecycling (nicht Bauteil-Reuse) fokussiert
- G: entfällt (unverbindliche Empfehlung ohne Rechtsfolge)
- Kernaussage: Der Kanton Bern stellt mit den "Verwendungsempfehlungen" ein unverbindliches Merkblatt zum Einsatz mineralischer Recycling-Baustoffe bereit — eine Vollzugshilfe ohne eigene Rechtsbindung, klar abzugrenzen von einer echten Verwendungsquote oder -pflicht, die für den Kanton Bern in dieser Sitzung **nicht** gefunden wurde (Negativbefund).
- Wortlautbeleg (Originalsprache): nicht verfügbar — Primärtext (PDF) in dieser Sitzung nicht abgerufen, nur über Themenseiten-Verweis identifiziert
- Beleg-Quelle: B4 · Zugänglichkeit: frei-primär (vermutet, nicht verifiziert) · Bindungsakt: entfällt/kein Bindungsakt identifiziert
- Quelle: Tier 1 (Existenznachweis) · https://www.bvd.be.ch/de/start/themen/abfall/bauabfaelle-und-recyclingbaustoffe.html · Fassung(as-amended) 2024 (3. Ausgabe) · Zugriff 2026-08-11
- Status: in Kraft · 2024
- Sub-Ebene: Stichprobe [BE] / nicht erhoben [25 übrige Kantone]
- Relationen: wird kombiniert mit / ergänzt REG-CH-3-018
- Konfidenz: unklar (nur Existenznachweis, B4)

### Vaud (VD) — dünn belegt, Web-Suchbudget während dieser Recherche erschöpft

### REG-CH-3-020 · VD Vollzug VVEA Art. 16 (kantonale Richtlinien Bauabfälle)

- Titel: nicht abschliessend identifiziert — Kanton Waadt verweist auf "directives cantonales sur les déchets de chantier" und eine "Ordonnance TASC" (Taxe d'Ablagerement/Ablagerungstaxe) ohne dass die genaue Erlassnummer in dieser Sitzung ermittelt wurde
- Fundstelle: nicht verifiziert (Lücke)
- A: sub-national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 3
- C: materialübergreifend
- D: nicht eindeutig zuordenbar (vermutlich Verwaltungsvorschrift oder RVO — nicht verifiziert)
- E: Bestandserkundung, Planung/Nachweis · E-Wirkung: erzwingt (Vollzugsebene zu Art. 16 VVEA)
- F1 (E3): schweigend · Bezugsgegenstand: In dieser Sitzung keine primärquellenbasierte Aussage zu einer VD-spezifischen Reuse-Regelung möglich
- F2 (E3): ermöglichend · Bezugsgegenstand: ausserrechtliche Praxisförderung — der Kanton Waadt hat laut amtlicher Medienmitteilung 1,1 Mio. CHF für die Förderung von Materialwiederverwendung gesprochen und unterstützt ein "Centre de compétences pour la durabilité dans la construction" sowie eine Lausanner "Ressourcerie" für Baumaterialien; dies ist Förderpolitik (Feld 5b), keine Feld-3-Vollzugsnorm — hier nur als Kontext vermerkt, nicht als eigenständiges Objekt kodiert, da keine Rechtsnorm dahintersteht, die primärquellenbasiert identifiziert wurde
- G: Dokumentenlage (inferiert=E3, analog zu VVEA Art. 16, kantonale Eigenheiten nicht geprüft)
- Kernaussage: Für den Kanton Waadt konnte in dieser Sitzung **keine** primärquellenbasierte kantonale Rechtsnorm zum Vollzug der VVEA-Bauabfall-Meldepflicht identifiziert werden — nur der Verweis auf "kantonale Richtlinien" und eine Ablagerungstaxen-Verordnung (TASC) auf einer amtlichen Themenseite. Der Kanton verfolgt daneben eine aktive Förderpolitik für Materialwiederverwendung (Ressourcerie Lausanne, kantonales Kompetenzzentrum), die aber eine politische Massnahme, keine identifizierte Rechtsnorm ist. Dies ist eine **echte, ehrlich markierte Erhebungslücke** — kein Objekt sollte hieraus als "Faktum" in die Synthese übernommen werden.
- Wortlautbeleg (Originalsprache): nicht verfügbar
- Beleg-Quelle: B4 · Zugänglichkeit: frei-primär (vermutet) · Bindungsakt: entfällt/kein Bindungsakt identifiziert
- Quelle: Tier 1 (Themenseite, keine Rechtsnorm) · https://www.vd.ch/environnement/dechets/dechets-de-chantier · https://www.vd.ch/environnement/economie-circulaire · Fassung(as-amended) nicht verifiziert · Zugriff 2026-08-11
- Status: unklar
- Sub-Ebene: Stichprobe [VD, sehr dünn] / nicht erhoben [25 übrige Kantone]
- Relationen: konkretisiert (vermutlich) REG-CH-3-009 — nicht primärquellenbasiert bestätigt
- Konfidenz: unklar (B4, keine Rechtsnorm identifiziert — Lücke, nicht Faktum)

### Ticino (TI)

### REG-CH-3-021 · TI RLE Art. 9 lett. n) (Baubewilligungsgesuch, Bauabfallangaben + Baujahr-1991-Trigger)

- Titel: Regolamento di applicazione della legge edilizia (RLE), Art. 9 lett. n
- Fundstelle: Art. 9 lett. n; RL/Ordnungsnummer 705.110
- A: sub-national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 3 · Nebenfelder: 2 (Baubewilligungsgesuch)
- C: materialübergreifend, Dämmstoffe+Schadstoffe (Asbest-Bezug über Baujahr-Trigger)
- D: Gesetz (kantonales Ausführungsreglement mit Verordnungscharakter — RLE ist Regolamento, daher D=RVO korrekter als Gesetz)
- E: Bestandserkundung, Planung/Nachweis · E-Wirkung: erzwingt
- F1 (E3): bedingend · Bezugsgegenstand: konkretisiert die bundesrechtliche VVEA-Art.-16-Meldepflicht kantonal und **erweitert** sie um einen eigenständigen Auslöser: Abbruch oder Umbau von Bauwerken, die vor dem 1. Januar 1991 errichtet wurden, lösen die Angabepflicht unabhängig von der 200-m³-Schwelle aus und verlangen ein Gutachten eines anerkannten Sachverständigen — textbelegt strenger als die Bundesnorm
- F2 (E3): bedingend · Bezugsgegenstand: dieselbe Fallgruppe — der Baujahr-Trigger zielt erkennbar auf Asbest-Altlasten (Verwendungsverbot in CH seit 1990), erzwingt aber damit auch bei kleinen Umbauten an Altbauten eine Bestandserkundung vor Rückbau, was sowohl Schutz- als auch Reuse-relevante Information liefert
- G: [1] Dokumentenlage (immer, ≥200 m³ oder Schadstoffe) → [2] Erklärung Dritter/Gutachten (falls Baujahr < 1991) (explizit=E1)
- Kernaussage: Art. 9 lett. n RLE integriert die VVEA-Art.-16-Meldepflicht in das Tessiner Baubewilligungsverfahren und erweitert sie um einen eigenständigen, altersbezogenen Auslöser: Bei Abbruch oder Umbau von vor 1991 errichteten Bauten ist unabhängig vom Abfallvolumen eine Fachexpertise zu Art, Qualität und Menge der Bauabfälle beizubringen. Dies ist eine textbelegte kantonale Verschärfung/Konkretisierung der Bundesnorm, kein blosser Vollzugsverweis.
- Wortlautbeleg (Originalsprache): "n) le informazioni ai sensi dell'art. 16 dell'ordinanza sulla prevenzione e lo smaltimento dei rifiuti del 4 dicembre 2015 (OPSR) concernenti la tipologia, la qualità e la quantità dei rifiuti edili prodotti nonché il loro smaltimento, se […] l'intervento comporta la demolizione o la trasformazione di edifici o impianti costruiti prima del 1° gennaio 1991; in tal caso le informazioni devono essere fornite tramite una perizia allestita da uno specialista riconosciuto" (Art. 9 lett. n RLE)
- Beleg-Quelle: B0 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://m3.ti.ch/CAN/RLeggi/public/index.php/raccolta-leggi/pdfatto/atto/8903 · Fassung(as-amended) nicht exakt datiert (Fussnotenverweis auf Novelle vorhanden, genaues Datum nicht extrahiert) · Zugriff 2026-08-11
- Status: in Kraft · Datum der letzten Novelle nicht exakt verifiziert
- Sub-Ebene: Stichprobe [TI] / nicht erhoben [25 übrige Kantone]
- Relationen: konkretisiert REG-CH-3-009 (VVEA Art. 16)
- Konfidenz: gesichert

### REG-CH-3-022 · TI LALPAmb (kantonales Ausführungsgesetz zum USG)

- Titel: Legge cantonale di applicazione della legge federale sulla protezione dell'ambiente (LALPAmb)
- Fundstelle: nicht im Volltext geprüft (Lücke); Existenz und Novellierung 2023 (Sackgebühr) über PGR-2026-2030-Sekundärtext bestätigt
- A: sub-national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 3
- C: materialübergreifend
- D: Gesetz
- E: Abfallstatus, Aufbereitung/Prüfung · E-Wirkung: durchläuft
- F1 (E3): schweigend · Bezugsgegenstand: in dieser Sitzung keine primärquellenbasierte Aussage zu einer Reuse-spezifischen Bestimmung möglich
- F2 (E3): schweigend · Bezugsgegenstand: dieselbe Fallgruppe
- G: entfällt (kein Nachweistatbestand ohne Primärtexteinsicht zuordenbar)
- Kernaussage: Die LALPAmb ist das kantonale Ausführungsgesetz des Tessin zum eidgenössischen USG/VVEA-Regime; laut dem eingesehenen "Piano di gestione dei rifiuti 2026-2030" wurde 2023 eine Novelle zur kantonalen Sackgebühr beschlossen. Der weitere Regelungsgehalt zu Bauabfällen/Reuse wurde in dieser Sitzung nicht im Volltext geprüft — echte Lücke.
- Wortlautbeleg (Originalsprache): nicht verfügbar — Primärtext nicht eingesehen
- Beleg-Quelle: B3 · Zugänglichkeit: frei-primär (vermutet, m3.ti.ch-Gesetzessammlung), nicht verifiziert · Bindungsakt: entfällt
- Quelle: Tier 1 (Sekundärerwähnung in amtlichem Planungsdokument) · https://m4.ti.ch/fileadmin/DT/temi/gestione_rifiuti/documenti/Piano_di_Gestione_dei_Rifiuti__PGR__2026-2030.pdf · Fassung(as-amended) 2023 (letzte erwähnte Novelle) · Zugriff 2026-08-11
- Status: in Kraft
- Sub-Ebene: Stichprobe [TI] / nicht erhoben [25 übrige Kantone]
- Relationen: setzt um USG (REG-CH-3-007) auf kantonaler Ebene
- Konfidenz: unklar (nur Existenznachweis, kein Primärtext)

### REG-CH-5b-023 · TI Piano di gestione dei rifiuti 2026-2030 (Massnahmen Riuso/ti-riuso.ch)

- Titel: Piano di gestione dei rifiuti del Canton Ticino 2026-2030 (PGR), Massnahmen zu Bau-/Abbruchabfällen und Materialbörse ti-riuso.ch
- Fundstelle: Massnahme "Rifiuti edili" (Gesamtplan, genaue Massnahmennummer für Riuso in dieser Sitzung nicht exakt lokalisiert)
- A: sub-national
- B: Primärfeld 5b · Nebenfelder: 3
- C: materialübergreifend
- D: Verwaltungsvorschrift (kantonaler Abfallplanungsakt gestützt auf USG Art. 31/kantonales Recht)
- E: Abfallstatus, Aufbereitung/Prüfung · E-Wirkung: durchläuft
- F1 (E3): ermöglichend · Bezugsgegenstand: der Plan nennt ausdrücklich die Stärkung von Wiederverwendung/Reparatur/Wiederherstellung von Produkten zur Vermeidung des Abfallstatus sowie den systematischen Einsatz von RC-Baustoffen (Beton/Asphalt) in öffentlichen und privaten Bauten als Zielsetzung 2026-2030
- F2 (E3): ermöglichend · Bezugsgegenstand: dieselbe Fallgruppe — die freie Materialbörse ti-riuso.ch (Projekt "Circular Economy" mit Wiedereingliederungs-Komponente) ist ein konkretes, bereits operatives Förderinstrument
- G: entfällt (Planungsdokument, kein eigener Nachweistatbestand)
- Kernaussage: Der Tessiner Abfallwirtschaftsplan 2026-2030 benennt die Stärkung von Wiederverwendung, Reparatur und Wiederherstellung von Produkten sowie den systematischen Einsatz rezyklierter Baustoffe explizit als Massnahmenziel und verweist auf die bereits bestehende Materialbörse ti-riuso.ch. Als Planungsinstrument (nicht als unmittelbar bindende Rechtsnorm) ist er Feld 5b (weiche Förderung/Anreiz) zuzuordnen, nicht Feld 3 im engeren (Vollzugs-)Sinn.
- Wortlautbeleg (Originalsprache): "rafforzare il riutilizzo, la riparazione e il ripristino dei prodotti, evitandone la trasformazione in rifiuto, e l'impiego sistematico di materiali da costruzione riciclati (calcestruzzo e asfalto riciclati) nelle opere pubbliche e nel settore privato" (Paraphrase-nahe Wiedergabe nach WebFetch-Zusammenfassung des PGR — **kein** wortwörtliches Vollzitat, da der PDF-Volltext an dieser Stelle nicht zeilengenau gegengelesen wurde; s. Beleg-Quelle)
- Beleg-Quelle: B2 · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Planungsdokument, kein Bindungsakt im engeren Sinn)
- Quelle: Tier 1 · https://m4.ti.ch/fileadmin/DT/temi/gestione_rifiuti/documenti/Piano_di_Gestione_dei_Rifiuti__PGR__2026-2030.pdf · https://www.ti-riuso.ch/ · Fassung(as-amended) 2026-2030 (Planperiode) · Zugriff 2026-08-11
- Status: in Kraft · Planperiode 2026-2030
- Sub-Ebene: Stichprobe [TI] / nicht erhoben [25 übrige Kantone]
- Relationen: wird kombiniert mit / ergänzt REG-CH-3-021, REG-CH-3-022
- Konfidenz: abgeleitet (Kernaussage über Grep-Fundstellen plus Fetch-Zusammenfassung erschlossen, nicht zeilengenau am Original gegengelesen)

---

## C. Zusammenfassende Bewertung und Lückenliste

**Was in dieser Sitzung solide (B0) primärquellenbasiert steht:**
- BauPG Art. 1–10 (inkl. Negativbefund: kein Reuse-Begriff) — vollständig gelesen.
- BauPV Anhang I Ziff. 7 (explizite Wiederverwendbarkeits-Grundanforderung) — vollständig gelesen.
- USG Art. 7 Abs. 6 (Abfallbegriff) — vollständig gelesen.
- VVEA Art. 12, 16, 17 — vollständig gelesen.
- BöB Art. 29 f. — vollständig gelesen.
- OR Art. 363–371 (Werkvertrag) — vollständig gelesen.
- TI RLE Art. 9 lett. n — vollständig gelesen, textbelegte kantonale Verschärfung der VVEA-Meldepflicht identifiziert.
- ZH AbfG § 1/§ 16a — gelesen, aber mit unsicherem Fassungsstand (s. u.).

**Was nur sekundär/eingeschränkt belegt ist (B2–B4, als Lücke markiert statt erfunden):**
1. **IVTH-Primärtext** (SR 172.056.5) nicht im Volltext gelesen — Fedlex-Landingpage-Problem, kein funktionierender Filestore-Link gefunden. Bindungsmechanismus (IOTH erklärt VKF-Vorschriften verbindlich) nur sekundärquellenbasiert bestätigt.
2. **VKF-Brandschutzvorschriften** selbst sind kostenpflichtig (bsvonline.ch) und nicht eingesehen — Reuse-Bezug (Brandschutznachweis für wiederverwendete Türen/Verglasungen) bleibt vollständig offen, nicht nur unsicher.
3. **MuKEn 2025** Modultext nicht eingesehen — Reuse-Bezug (falls vorhanden, z. B. bei Fensterwiederverwendung/U-Wert) ungeprüft.
4. **ZH PBG § 220** — Primärtext trotz zweier Versuche technisch nicht auslesbar; Aussage stützt sich ausschliesslich auf einen AWEL-Verwaltungsbericht.
5. **ZH AbfG** — eingesehene PDF-Fassung ist erkennbar die Grundfassung 1994 mit unvollständig nachvollzogenen Novellen; 2026-Stand nicht gesichert.
6. **BE Abfallverordnung Art. 18** — Primärtext-Fetch scheiterte (HTTP 404); nur Themenseiten-Existenznachweis.
7. **VD** — keine kantonale Rechtsnorm zum VVEA-Vollzug identifiziert, nur Themenseiten-Hinweis auf "directives" und "Ordonnance TASC" ohne Erlassnummer. Dünnste Beleglage der gesamten Stichprobe.
8. **TI LALPAmb** — nur Existenznachweis, kein Primärtext.
9. **GE LCI Art. 117/118** — Kernsatz von Art. 117 wortgleich über silgeneve.ch bestätigt (B1), Art. 118 und das Ausführungsreglement nur über Fetch-Zusammenfassung/Sekundärquelle (B2) — vollständige Absatzstruktur nicht Wort für Wort verifiziert.
10. **IVHB-Beitrittsstatus** für GE, VD, TI in dieser Sitzung nicht geprüft (nur ZH-Nichtbeitritt und BE-Beitritt bestätigt).
11. **Gemeindeebene** — in keinem der fünf Stichprobenkantone erhoben, obwohl Baubewilligungen in der Praxis oft dorthin delegiert sind.
12. **Kantonale IVöB-Beschaffung** (öffentliche Ausschreibungen unterhalb Bundesschwelle) nicht geprüft — nur die Bundesebene (BöB) ist erhoben.

**Web-Suchbudget:** Erschöpft während der VD-Recherche (200/200 WebSearch-Aufrufe der Session) — sämtliche VD-spezifischen Aussagen ab diesem Punkt beruhen auf vorher bereits gefundenen Snippets, keine gezielte Nachrecherche mehr möglich. Für eine Vertiefungsrunde zu VD/BE-Primärtexten und zu den unter 1–3 genannten Lücken ist ein frisches Suchbudget nötig.

**Nächster Schritt (Ticket-Vorgabe):** Diese Liste ist Input für die adversarische Prüfung (W2 Stufe 3) und die Synthese (W4). Die unter Abschnitt C Nr. 1–12 benannten Lücken dürfen nicht stillschweigend als Fakten in die Synthese übernommen werden — insbesondere REG-CH-2-014, REG-CH-3-018, REG-CH-3-020 und REG-CH-3-022 tragen Konfidenz "unklar" und benötigen vor Weiterverwendung eine primärquellenbasierte Nachprüfung.
