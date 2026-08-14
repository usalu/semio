# CH — Regelungsobjekte Feld 4–7 (Extraktionsstufe, kombiniert mit Quellenaufschluss)

**Zweck:** Primärquellenbasierte Extraktion der Regelungsobjekte für die Schweiz, Regelungsfelder 4 (Schutzziele: Brand/Energie/Schadstoffe/Gesundheit), 5a (Vergaberecht hart), 5b (Anreize/Förderung weich), 6 (Normen/Regelwerke) und 7 (Haftung/Gewährleistung), gegen das eingefrorene 7-Achsen-Schema (`schema/taxonomie-final.md`).

**Abweichung vom geplanten Arbeitsablauf (Transparenzhinweis):** Der Auftrag verwies auf eine vorgelagerte Quellenkarte `roh/CH-quellen.md`. Diese Datei **existiert nicht** im Ticketordner (geprüft per Verzeichnislisting; im `roh/`-Ordner liegen Quellenkarten nur für AT, BE, DE, DK, FR, NL, NO, SE, UK — keine für CH). Da für Feld 4–7 CH auch keine andere Vorstufe vorlag, wurden Quellenaufschluss und Extraktion in dieser Session **in einem Schritt** durchgeführt: alle unten genannten Instrumente wurden erstmals in dieser Session identifiziert, über WebSearch/WebFetch aufgerufen und, wo als PDF vorliegend, lokal mit `pdftotext` im Volltext gelesen (nicht nur die von WebFetch gelieferte KI-Zusammenfassung). Diese Karte ersetzt damit funktional sowohl `CH-quellen.md` als auch die Extraktionsstufe für die Felder 4–7 — sie ist nicht gegen eine unabhängige Vorstufe gegengeprüft. Felder 1–3 CH sind **nicht** Gegenstand dieses Auftrags und hier nicht erhoben.

**Stichtag:** 2026-08-11. Zugriff auf alle Quellen: 2026-08-13.

**Sub-Ebene-Hinweis (CH, Projektkonvention):** Für CH gilt laut Taxonomie-Freeze **Stichprobe-und-Deklaration** (keine Vollerhebung aller 26 Kantone). Die Stichproben sind je Objekt vermerkt; nicht erhobene Kantone sind explizit als solche benannt, nicht stillschweigend ausgelassen.

**Methodischer Sonderbefund CH (an W4 zu melden, kein Schemaeingriff durch diese Karte):** Die Schweiz kennt neben den drei taxonomischen A-Werten (EU/EEA, national, sub-national) einen wiederkehrenden **vierten Bindungsmechanismus**, der weder eindeutig „national" (kein Bundesgesetz/keine Bundesverordnung) noch dem deutschen Muster-Modell-Typ „sub-national via Einzeltransformation je Land" entspricht: **interkantonale Konkordate/Organe, deren eigener Erlass unmittelbar gesamtschweizerisch bindet, ohne dass ein individueller kantonaler Transformationsakt nachgewiesen oder nötig ist** (Beispiel: VKF-Brandschutzvorschriften, in Kraft gesetzt durch das Interkantonale Organ Technische Handelshemmnisse IOTH, s. REG-CH-4-001/002). Diese Karte kodiert solche Fälle als **A = sub-national** (da Erarbeitung/Vollzug auf kantonaler Ebene der Gebäudeversicherungen/Baubehörden liegt, keine Bundesinstanz), vermerkt aber den strukturellen Unterschied zum MBO/LBO-Muster explizit im Feld „Downstream-Verifikationsstatus" jedes betroffenen Objekts. Dies ist eine Session-Entscheidung, keine Freeze-Änderung — zur Bestätigung/Korrektur an W4 (Restlücke „CH-MRA" aus Taxonomie-Freeze Abschnitt 13 wird hiermit erstmals mit Befunden gefüllt).

**Belegstand insgesamt:** Von 19 Objekten sind 14 mit B0/B1 (Volltext-Primärquelle direkt gelesen, meist per `pdftotext`-Extraktion aus amtlich/amtsnah verlinkten PDF-A-Dateien) belegt. 5 Objekte (REG-CH-4-007, REG-CH-5a-010/011 [Sub-Ebene-Anteil], REG-CH-5b-012, REG-CH-6-015) bleiben bei B2–B4 mit ausdrücklich offener Lücke — dort wurde bewusst **keine** erfundene Detailaussage ergänzt.

---

## Regelungsfeld 4 — Schutzziele (Brand/Energie/Schadstoffe/Gesundheit)

### REG-CH-4-001 · VKF-Brandschutznorm Art. 2 (Bestandsbauten-Verhältnismässigkeit)
- Titel: Brandschutznorm der Vereinigung Kantonaler Feuerversicherungen (VKF-Brandschutznorm), Ausgabe 01.01.2015, Dok.-Nr. 1-15de
- Fundstelle: Art. 2 (Geltungsbereich), Abs. 1–2
- A: sub-national · Downstream-Verifikationsstatus: strukturell abweichend vom Bund-Land-Muster (Freitext-Vermerk s. o.) — die VKF-Brandschutzvorschriften wurden durch das Interkantonale Organ Technische Handelshemmnisse (IOTH) unmittelbar für die ganze Schweiz für verbindlich erklärt, ohne dass laut dem eingesehenen VKF-eigenen Leitfaden ein zusätzlicher kantonaler Umsetzungserlass als Voraussetzung genannt wird ("Damit haben die Brandschutznorm und die Brandschutzrichtlinien … Gesetzescharakter"). Stichprobe Kanton Zürich: die frühere kantonale "Verordnung über den baulichen Brandschutz" (LS 861.13) ist seit 01.01.2005 aufgehoben; ein separates aktuell geltendes ZH-Parallelregelwerk wurde in dieser Session nicht gefunden — konsistent mit direkter gesamtschweizerischer Bindung, aber nicht abschliessend für alle 26 Kantone verifiziert.
- B: Primärfeld 4 · Normtyp: operative Norm
- C: materialübergreifend
- D: Verwaltungsvorschrift (Behördenerlass mit Aussenwirkung — IOTH ist ein interkantonales Fachorgan; die Einordnung ist wegen des o. g. Sonderbefunds nicht zweifelsfrei, s. Freitext oben; Alternative wäre ein neuer, im Freeze nicht vorgesehener D-Wert "interkantonales Konkordatsrecht", hier bewusst NICHT eingeführt, sondern an W4 gemeldet)
- E: Einbau/Abnahme · E-Wirkung: durchläuft; Betrieb/Dokumentation · E-Wirkung: durchläuft
- F1 (E3): bedingend — Abs. 2 verlangt Anpassung bestehender Bauten an die Brandschutzvorschriften nur "verhältnismässig" und nur bei (a) wesentlichen baulichen/betrieblichen Veränderungen, Erweiterungen oder Nutzungsänderungen oder (b) besonders grosser Personengefährdung; ausserhalb dieser Trigger bleibt der Bestand unangetastet — das begünstigt strukturell den Erhalt (und damit potenziell die Wiederverwendung) unveränderter Bestandsbauteile, ohne eine eigene Reuse-Regel zu formulieren
- F2 (E3): bedingend — das Verhältnismässigkeitsprinzip wird in der Vollzugspraxis (Brandschutzbehörden) typischerweise einzelfallbezogen ausgelegt; keine Praxisevidenz zu wiederverwendeten Bauteilen speziell in dieser Session erhoben
- G: Dokumentenlage (Trigger-Prüfung durch Baubehörde) — inferiert (E3); Art. 2 selbst nennt keine Nachweisform
- Kernaussage: Die Brandschutzvorschriften gelten primär für Neubauten; bestehende Bauten und Anlagen müssen ihnen nur verhältnismässig angepasst werden, und zwar nur bei wesentlichen baulichen/betrieblichen Veränderungen, Erweiterungen, Nutzungsänderungen oder besonders grosser Personengefährdung. Das ist die zentrale Bestandsschutz-/Verhältnismässigkeitsnorm des Schweizer Brandschutzrechts.
- Wortlautbeleg (Originalsprache): "Bestehende Bauten und Anlagen sind verhältnismässig an die Brandschutzvorschriften anzupassen, wenn: a wesentliche bauliche oder betriebliche Veränderungen, Erweiterungen oder Nutzungsänderungen vorgenommen werden; b die Gefahr für Personen besonders gross ist."
- Beleg-Quelle: B0 (Volltext-PDF per `pdftotext` gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: IOTH-Verbindlicherklärung (s. REG-CH-4-002 Bindungsakt-Detail)
- Quelle: Tier 1 · https://www.bwo.admin.ch/dam/de/sd-web/gj6rkrv3Mfz-/brandschutznorm.pdf (Bundesamt-für-Wohnungswesen-Spiegelung; kanonisches Portal: https://www.bsvonline.ch/de/brandschutzvorschriften/vorschriften-2015) · Fassung(as-amended) 2015-01-01 · Zugriff 2026-08-13
- Status: in Kraft · seit 2015-01-01; Totalrevision "Projekt BSV 2026" in Vorbereitung (technische Anhörung ab September 2025, politische Anhörung ab August 2026, Inkraftsetzung derzeit für 2027 vorgesehen — laut EnDK-/VKF-Sekundärquelle, in dieser Session nicht B0/B1 verifiziert)
- Sub-Ebene: Stichprobe [Zürich: kein aktuelles kantonales Parallelregelwerk gefunden, alte VO 2005 aufgehoben] / nicht erhoben [übrige 25 Kantone]
- Relationen: konkretisiert wird durch REG-CH-4-002 (dieselbe Norm, Baustoffe/Bauteile); wird kombiniert mit/ergänzt REG-CH-6-002 (SIA 269, Bestandsbewertung Tragwerke, sofern Brandschutzertüchtigung statische Eingriffe erfordert)
- Konfidenz: gesichert (Wortlaut); abgeleitet (Reuse-Wirkung, da Norm selbst keinen Bauteil-Wiederverwendungsbezug herstellt)

### REG-CH-4-002 · VKF-Brandschutznorm Art. 23–27 (Baustoffe/Bauteile: Klassierung und Verwendung)
- Titel: Brandschutznorm der Vereinigung Kantonaler Feuerversicherungen (VKF-Brandschutznorm), Ausgabe 01.01.2015, Dok.-Nr. 1-15de
- Fundstelle: Abschnitt C "Baulicher Brandschutz", Art. 23–27
- A: sub-national · Downstream-Verifikationsstatus: identisch REG-CH-4-001
- B: Primärfeld 4 · Normtyp: operative Norm
- C: materialübergreifend
- D: Verwaltungsvorschrift (s. Freitext-Vorbehalt REG-CH-4-001)
- E: Aufbereitung/Prüfung · E-Wirkung: durchläuft; Einbau/Abnahme · E-Wirkung: durchläuft
- F1 (E3): bedingend — Baustoffe und Bauteile müssen über "genormte Prüfungen oder andere VKF-anerkannte Verfahren" klassiert sein (Brandverhalten bzw. Feuerwiderstandsdauer); der Normtext selbst unterscheidet nicht zwischen neuen und gebrauchten Baustoffen/Bauteilen — für ein wiederverwendetes Bauteil ohne noch gültiges/zuordenbares Prüfzeugnis entsteht damit faktisch dieselbe Klassierungspflicht wie für ein Neuprodukt, ohne dass die Norm dafür einen erleichterten Weg (z. B. Bestandsprüfzeugnis, Altbestandsvermutung) vorsieht — F1=bedingend, tendenziell hemmend für nicht mehr dokumentierte Bestandsbauteile
- F2 (E3): hemmend — in der Vollzugspraxis fehlt für viele ältere/wiederverwendete Bauteile ein zuordenbares Prüfzeugnis; ohne ein VKF-anerkanntes Verfahren zur Klassierung im Bestand entsteht ein praktisches Nachweishindernis (Projekteinschätzung, keine in dieser Session eingesehene Praxisstatistik)
- G: zerstörungsfreie Prüfung / Probenahme/Materialprüfung (Prüfung/Klassierung nach Art. 24, 27) — explizit (E1) für den Normalfall Neubaustoff; für Bestandsbauteile ohne Zeugnis: Einzelfallzulassung — inferiert (E3), da der Normtext keinen Bestandsfall-Pfad ausdrücklich regelt
- Kernaussage: Baustoffe gelten über genormte Prüfungen oder VKF-anerkannte Verfahren als klassiert (Brand-/Qualmverhalten, brennendes Abtropfen, Korrosivität); Bauteile werden nach Feuerwiderstandsdauer (Tragfähigkeit, Raumabschluss, Wärmedämmung) klassiert. Der Normtext regelt Prüfung/Klassierung generisch, ohne eigene Kategorie für gebrauchte/wiederverwendete Baustoffe oder Bauteile.
- Wortlautbeleg (Originalsprache): "Baustoffe werden über genormte Prüfungen oder andere VKF-anerkannte Verfahren klassiert. Massgebende Kriterien sind insbesondere Brand- und Qualmverhalten, brennendes Abtropfen und Korrosivität." (Art. 24) / "Bauteile werden über genormte Prüfungen oder andere VKF-anerkannte Verfahren klassiert. Massgebend ist insbesondere die Feuerwiderstandsdauer bezüglich der Kriterien Tragfähigkeit, Raumabschluss und Wärmedämmung." (Art. 27 Abs. 1)
- Beleg-Quelle: B0 (Volltext-PDF per `pdftotext` gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: laut VKF-Leitfaden "Verweise auf die VKF-Brandschutzvorschriften" (Version 1.0, 11.12.2018, Technische Kommission Brandschutz TKB): "Die Brandschutzvorschriften der Vereinigung Kantonaler Feuerversicherungen (VKF) wurden vom Interkantonalen Organ Technische Handelshemmnisse (IOTH) in Kraft gesetzt und für die ganze Schweiz als verbindlich erklärt. Damit haben die Brandschutznorm und die Brandschutzrichtlinien (ohne deren Anhänge) Gesetzescharakter."
- Quelle: Tier 1 · https://www.bwo.admin.ch/dam/de/sd-web/gj6rkrv3Mfz-/brandschutznorm.pdf ; Bindungsakt-Beleg: https://services.vkg.ch/rest/public/georg/bs/publikation/documents/BSPUB-1394520214-2787.pdf/content · Fassung(as-amended) 2015-01-01 · Zugriff 2026-08-13
- Status: in Kraft · seit 2015-01-01; Revision s. REG-CH-4-001
- Sub-Ebene: Stichprobe [wie REG-CH-4-001] / nicht erhoben [übrige 25 Kantone]
- Relationen: konkretisiert REG-CH-4-001; determiniert Anwendbarkeit von — kein eigenes Grundnorm-Flag (operative Norm, keine Gatekeeper-Funktion für andere Objekte)
- Konfidenz: gesichert (Wortlaut); abgeleitet (Einordnung als Hemmnis für Bestandsbauteile — Projektschlussfolgerung E3, kein Textbeleg für die Bestandsfall-Lücke selbst, da der Text schlicht schweigt)

### REG-CH-4-003 · Energiegesetz (EnG) Art. 45 Abs. 3 Bst. e (Grenzwerte graue Energie, Bundes-Ermächtigung)
- Titel: Bundesgesetz über die Energie (Energiegesetz, EnG) vom 30. September 2016
- Fundstelle: Art. 45 Abs. 3 Bst. e (+ Abs. 1, 2, 4, 5); SR 730.0; ELI: https://fedlex.data.admin.ch/eli/cc/2017/762
- A: national
- B: Primärfeld 4 · Nebenfelder: 6 (Normen — MuKEn-Verweis in Abs. 4)
- C: materialübergreifend
- D: Gesetz
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): ermöglichend — Bst. e ist eine reine Bundes-Ermächtigungsnorm ("Sie erlassen insbesondere Vorschriften über: … e. die Grenzwerte für die graue Energie bei Neubauten und bei wesentlichen Erneuerungen bestehender Gebäude"); sie verpflichtet die Kantone zur Regelung, senkt aber nicht selbst einen Grenzwert — durch die Einbeziehung "wesentlicher Erneuerungen" adressiert sie ausdrücklich auch den Bestand, was für reuse-mindernde Neubauanteile potenziell steuernd wirken kann (graue Energie eines wiederverwendeten Bauteils = quasi null), ohne dass der Gesetzestext Wiederverwendung explizit erwähnt
- F2 (E3): schweigend — die materielle Wirkung entsteht erst über die kantonale Umsetzung (s. REG-CH-4-004); auf Bundesebene selbst keine unmittelbare Praxiswirkung
- G: Anwendbarkeitsnorm ohne Nachweistatbestand — explizit (E1); Art. 45 selbst begründet keinen Einzelnachweis, sondern verpflichtet die Kantone zur Rechtsetzung
- Kernaussage: Art. 45 EnG verpflichtet die Kantone, Vorschriften über die sparsame und effiziente Energienutzung in Neubauten und Bestandsbauten zu erlassen; Abs. 3 Bst. e nennt seit der am 1. Januar 2025 in Kraft getretenen Ergänzung ausdrücklich die Grenzwerte für graue Energie bei Neubauten und bei wesentlichen Erneuerungen bestehender Gebäude als Regelungsgegenstand. Abs. 4 verweist auf die MuKEn als anerkannten Referenzstandard.
- Wortlautbeleg (Originalsprache): "3 Sie erlassen insbesondere Vorschriften über: … e. die Grenzwerte für die graue Energie bei Neubauten und bei wesentlichen Erneuerungen bestehender Gebäude." (Fussnote der Quelle: "Eingefügt durch Ziff. II 2 des BG vom 15. März 2024, in Kraft seit 1. Jan. 2025 [AS 2024 648; BBl 2023 13, 437]".)
- Beleg-Quelle: B0 (Volltext-PDF, Fassung Stand 1.1.2026, per `pdftotext` gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: —
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2017/762/de (kanonisch); PDF-Fassung gelesen: https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2017/762/20260101/de/pdf-a/fedlex-data-admin-ch-eli-cc-2017-762-20260101-de-pdf-a-3.pdf · Fassung(as-amended) 2026-01-01 · Zugriff 2026-08-13
- Status: in Kraft · Bst. e seit 2025-01-01
- Sub-Ebene: nicht zutreffend (A=national)
- Relationen: konkretisiert wird durch REG-CH-4-004 (MuKEn 2025 Basismodul); determiniert Anwendbarkeit von REG-CH-4-004 (Grundnorm-Charakter: ohne Art. 45 Abs. 3 Bst. e keine Bundes-Ermächtigung für kantonale Grauenergie-Grenzwerte)
- Konfidenz: gesichert

### REG-CH-4-004 · MuKEn 2025, Basismodul Teil G "Graue Energie" (Mustervorschrift)
- Titel: Mustervorschriften der Kantone im Energiebereich (MuKEn), Ausgabe 2025
- Fundstelle: Basismodul, Teil G "Graue Energie" (Grenzwerte für Neubauten; genauer Zifferntext des Basismoduls in dieser Session nicht im Volltext eingesehen, s. Lücke unten)
- A: sub-national · Downstream-Verifikationsstatus: strukturell angenommen, nicht verifiziert — die MuKEn sind laut EnDK-Medienmitteilung selbst ausdrücklich "schweizweite Empfehlungen" ("Seit 1992 unterstützen die MuKEn als schweizweite Empfehlungen eine koordinierte Umsetzung…"), keine unmittelbar bindende Norm; Bindung entsteht erst durch Übernahme in die 26 kantonalen Energiegesetze. Für MuKEn 2025 (verabschiedet 29.08.2025) wurde eine Umsetzungsfrist 2025–2030 kommuniziert; zum Stichtag 2026-08-11 wurde in dieser Session **keine** einzelne kantonale Umsetzung von MuKEn 2025 (insbesondere der Graue-Energie-Grenzwerte) primärquellenbasiert verifiziert — reiner Empfehlungsstatus zum Stichtag anzunehmen.
- B: Primärfeld 4 · Nebenfelder: 6 · Normtyp: Muster-/Modellrecht-Charakter (s. D)
- C: materialübergreifend
- D: Muster-/Modellrecht (unverbindlich, Umsetzung durch Dritte erforderlich)
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): ermöglichend — führt laut EnDK-Pressetext erstmals verbindliche Grenzwerte für graue Energie (kg CO2eq, Betrachtungshorizont laut Sekundärquelle 60 Jahre, Erstellungsemissionen) für Neubauten ein; ein niedrigerer Grauenergie-Fussabdruck lässt sich messbar durch Bauteilwiederverwendung senken, auch wenn der eingesehene Pressetext Wiederverwendung nicht als Massnahme nennt — Wirkung ist mittelbar/strukturell, nicht textlich explizit auf Reuse bezogen
- F2 (E3): schweigend — solange keine kantonale Umsetzung vorliegt, keine unmittelbare Vollzugswirkung; Praxiswirkung hängt vollständig von der noch ausstehenden kantonalen Transformation ab
- G: rechnerischer Nachweis (Ökobilanzierung/CO2eq-Berechnung für Erstellungsemissionen) — inferiert (E3); der in dieser Session eingesehene Pressetext nennt keine Nachweismethode im Detail
- Kernaussage: Die Plenarversammlung der Konferenz Kantonaler Energiedirektoren (EnDK) verabschiedete am 29. August 2025 die MuKEn 2025. Neu enthalten sie Grenzwerte für graue Energie bei der Erstellung von Neubauten, verankert im Basismodul Teil G. Als Empfehlung sind die MuKEn erst nach Übernahme in kantonales Energierecht bindend; die Umsetzung ist auf den Zeitraum 2025–2030 angelegt.
- Wortlautbeleg (Originalsprache): "So enthalten die MuKEn 2025 neu Grenzwerte für die graue Energie bei der Erstellung von Neubauten." / "Seit 1992 unterstützen die MuKEn als schweizweite Empfehlungen eine koordinierte Umsetzung der Energie- und Klimapolitik im kantonalen Bau- und Energierecht…"
- Beleg-Quelle: B1 (Medienmitteilung der EnDK im Volltext per `pdftotext` gelesen; das Basismodul-Dokument selbst mit dem vollständigen Ziffern-Wortlaut zu Teil G wurde in dieser Session **nicht** eingesehen — Lücke) · Zugänglichkeit: frei-primär · Bindungsakt: Bundes-Ermächtigung Art. 45 Abs. 3 Bst. e EnG (s. REG-CH-4-003); konkreter kantonaler Bindungsakt je Kanton noch offen
- Quelle: Tier 1 (EnDK ist die Konferenz der 26 kantonalen Energiedirektoren, amtliches interkantonales Gremium) · https://endk.ch/wp-content/uploads/2025/08/20250825_Medienmitteilung_MuKEn.pdf ; Übersicht https://endk.ch/die-kantone-verabschieden-die-mustervorschriften-2025-und-beschreiten-den-pfad-der-energiewende-konsequent-weiter/ · Fassung(as-amended) 2025-08-29 · Zugriff 2026-08-13
- Status: Übergang · verabschiedet 2025-08-29, kantonale Umsetzung 2025–2030 angelegt, zum Stichtag 2026-08-11 nicht verifiziert abgeschlossen
- Sub-Ebene: Stichprobe [keine kantonale Umsetzung von MuKEn 2025 verifiziert — Negativbefund als Stichprobenergebnis] / nicht erhoben [alle 26 Kantone einzeln]
- Relationen: konkretisiert REG-CH-4-003 (Art. 45 Abs. 3 Bst. e EnG); setzt um wird durch künftige kantonale Energiegesetze (nicht separat erhoben)
- Konfidenz: abgeleitet (Existenz/Kernaussage gesichert per B1; genauer Grenzwert-Wortlaut und Reuse-Bezug nicht im Volltext geprüft)

### REG-CH-4-004a · USG Art. 35j "Ressourcenschonendes Bauen" — Bundes-Ermächtigung zu Bauteilwiederverwendung, explizit im Wortlaut (**neu ergänzt, Prüfung 2026-08-13 — kritischer Fund, fehlte in allen drei CH-Ernte-Dateien**)
- Titel: Bundesgesetz vom 7. Oktober 1983 über den Umweltschutz (Umweltschutzgesetz, USG), 2. Titel, 4. Abschnitt "Ressourcenschonendes Bauen"
- Fundstelle: Art. 35j Abs. 1–2 USG; SR 814.01 (eingefügt durch Ziff. I des BG vom 15. März 2024, in Kraft seit 1. Januar 2025, AS 2024 648; BBl 2023 13, 437 — derselbe Änderungserlass wie REG-CH-3-010/-011 und REG-CH-4-003)
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 4 Schutzziele (Ressourcenschonung als Umweltschutzziel, parallel zu REG-CH-4-003 Graue-Energie-Ermächtigung) · Nebenfelder: 2 (Bautechnik: Rückbaubarkeit, Bauteilverwendung), 3 (Abfall-/Stoffrecht: stofflich verwertete Baustoffe), 6 (Normen — Bundesrat kann bei Ausführung auf technische Normen verweisen)
- C: materialübergreifend
- D: Gesetz (Ermächtigungsnorm; die materielle Ausgestaltung erfolgt erst über eine noch nicht identifizierte Ausführungsverordnung — Status dieser Verordnung in dieser Prüfung nicht recherchiert, echte Lücke)
- E: Planung/Nachweis; Rückbau/Sicherung
- F1 (E3): ermöglichend — Art. 35j Abs. 1 Bst. d ermächtigt den Bundesrat ausdrücklich, Anforderungen über "die Wiederverwendung von Bauteilen in Bauwerken" festzulegen — dies ist die textlich präziseste und direkteste Rechtsgrundlage für eine künftige bundesrechtliche Bauteil-Reuse-Regelung, die in der gesamten CH-Recherche (drei Ernte-Dateien) gefunden wurde; Bst. a–c ergänzen um umweltschonende Baustoffe/Bauteile, RC-Baustoffe und Rückbaubarkeit — ein zusammenhängendes Kreislaufwirtschafts-Paket
- F2 (E3): schweigend — Art. 35j ist reine Ermächtigungsnorm ("kann … festlegen"); ohne die noch ausstehende bundesrätliche Ausführungsverordnung entfaltet die Norm keine unmittelbare Vollzugswirkung; ob/wann der Bundesrat von der Ermächtigung Gebrauch macht, wurde in dieser Prüfung nicht recherchiert
- G: Anwendbarkeitsnorm ohne eigenen Nachweistatbestand — explizit (E1); die Norm selbst formuliert nur eine Rechtsetzungskompetenz, keinen Einzelfallnachweis
- Kernaussage: Mit der am 1. Januar 2025 in Kraft getretenen Gesetzesänderung vom 15. März 2024 erhielt das USG einen neuen 4. Abschnitt "Ressourcenschonendes Bauen" (Art. 35j). Danach kann der Bundesrat im Rahmen einer gesamthaften, bauwerk- und lebenszyklusbasierten Nachhaltigkeitsbetrachtung Anforderungen festlegen über (a) die Verwendung umweltschonender Baustoffe und Bauteile, (b) die Verwendung stofflich verwerteter Baustoffe, (c) die Rückbaubarkeit von Bauwerken und — **ausdrücklich** — (d) die Wiederverwendung von Bauteilen in Bauwerken. Abs. 2 verpflichtet den Bund zusätzlich zu einer Vorbildfunktion bei eigenen Bauwerken. Diese Norm ist damit die zentrale, textlich explizite Bundes-Rechtsgrundlage für eine künftige Bauteilwiederverwendungsregulierung in der Schweiz — sie fehlte in den drei ursprünglichen CH-Ernte-Dateien (CH-F1-3.md, CH-F4-7.md, CH-Kantone.md) vollständig, obwohl REG-CH-3-010/-011 (Art. 7 Abs. 6bis, Art. 30d USG) aus demselben Änderungserlass bereits erfasst waren.
- Wortlautbeleg (Originalsprache): "1 Der Bundesrat kann im Rahmen einer gesamthaften, bauwerk- und lebenszyklusbasierten Nachhaltigkeitsbetrachtung nach Massgabe der durch Bauwerke verursachten Umweltbelastung und unter Beachtung der internationalen Verpflichtungen der Schweiz Anforderungen festlegen über: a. die Verwendung umweltschonender Baustoffe und Bauteile; b. die Verwendung von Baustoffen, die aus der stofflichen Verwertung von Bauabfällen stammen; c. die Rückbaubarkeit von Bauwerken; und d. die Wiederverwendung von Bauteilen in Bauwerken. 2 Der Bund nimmt bei der Planung, der Errichtung, dem Betrieb, der Erneuerung und dem Rückbau eigener Bauwerke eine Vorbildfunktion wahr. Er berücksichtigt dazu erhöhte Anforderungen an das ressourcenschonende Bauen und innovative Lösungen." (Art. 35j Abs. 1–2 USG; Fussnote 101: "Eingefügt durch Ziff. I des BG vom 15. März 2024, in Kraft seit 1. Jan. 2025 [AS 2024 648; BBl 2023 13, 437].")
- Beleg-Quelle: B0 Primärtext-Volltext (Fedlex-PDF/A, Fassung 2025-01-01, per `pdftotext` direkt gelesen und Fundstelle per grep verifiziert) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Bundesgesetz selbst; Ausführungsverordnung des Bundesrates noch nicht identifiziert/recherchiert)
- Quelle: Tier 1 · https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/1984/1122_1122_1122/20250101/de/pdf-a/fedlex-data-admin-ch-eli-cc-1984-1122_1122_1122-20250101-de-pdf-a-8.pdf (in Prüfung 2026-08-13 direkt geöffnet); kanonisch: https://www.fedlex.admin.ch/eli/cc/1984/1122_1122_1122/de (SR 814.01) · Fassung(as-amended) 2025-01-01 · Zugriff 2026-08-13
- Status: in Kraft · seit 2025-01-01 (Ermächtigungsnorm; zugehörige Ausführungsverordnung nicht recherchiert — Lücke für Nacherhebung). **Zusatzbefund:** In der Fassung Stand 2026-04-01 (https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/1984/1122_1122_1122/20260401/de/pdf-a/fedlex-data-admin-ch-eli-cc-1984-1122_1122_1122-20260401-de-pdf-a.pdf, dieselbe Fassung wie in REG-CH-3-011 zitiert) ist Art. 35j bereits mit einer Strafbestimmung flankiert: Der USG-Bussenkatalog sanktioniert vorsätzliche Verstösse gegen künftige, gestützt auf Art. 35j Abs. 1 erlassene Vorschriften ("j. Vorschriften über das ressourcenschonende Bauen verletzt [Art. 35j Abs. 1]") — die Ermächtigung ist damit straf-/bussenbewehrt vorbereitet, auch wenn die materielle Verordnung selbst noch aussteht.
- Sub-Ebene: entfällt (A=national)
- Relationen: steht im selben Änderungserlass wie REG-CH-3-010 (Art. 7 Abs. 6bis USG), REG-CH-3-011 (Art. 30d USG) und REG-CH-4-003 (EnG Art. 45 Abs. 3 Bst. e) — alle vier Normen traten zeitgleich am 1. Jan. 2025 durch dieselbe BG-Revision vom 15. März 2024 in Kraft und bilden gemeinsam ein Kreislaufwirtschafts-Gesetzespaket; determiniert Anwendbarkeit einer (nicht identifizierten) künftigen Ausführungsverordnung; strukturanalog zu REG-CH-4-003 (Bundes-Ermächtigung ohne unmittelbare materielle Wirkung)
- Konfidenz: gesichert (Wortlaut, Fundstelle, Fussnote/Datum B0-primärquellenverifiziert, zwei unabhängige Fedlex-Fassungsstände [2025-01-01 und 2026-04-01] gegengelesen); abgeleitet (praktische Reichweite, da Ausführungsverordnung noch nicht recherchiert)

### REG-CH-4-005 · Bauarbeitenverordnung (BauAV) Art. 3 Abs. 2 / Art. 32 (Gefährdungsermittlung Asbest/PCB)
- Titel: Verordnung über die Sicherheit und den Gesundheitsschutz der Arbeitnehmerinnen und Arbeitnehmer bei Bauarbeiten (Bauarbeitenverordnung, BauAV) vom 18. Juni 2021
- Fundstelle: Art. 3 Abs. 2–3; Art. 32 Abs. 1–3; SR 832.311.141
- A: national
- B: Primärfeld 4 · Nebenfelder: 2 (Bestandserkundung als Vorstufe zu Standsicherheits-/Rückbaunachweis)
- C: materialübergreifend
- D: RVO
- E: Bestandserkundung · E-Wirkung: erzwingt; Rückbau/Sicherung · E-Wirkung: durchläuft
- F1 (E3): bedingend — bei Verdacht auf besonders gesundheitsgefährdende Stoffe (Asbest, PCB) muss der Arbeitgeber die Gefährdungen "eingehend ermitteln und beurteilen", bevor die erforderlichen Massnahmen geplant werden; dies erzwingt eine Erkundungsphase, die für die spätere Wiederverwendungsfähigkeit eines Bauteils/Bauelements aus demselben Gebäude faktisch mitentscheidend ist (kontaminierte Bauteile scheiden für Reuse aus), ohne dass die Norm selbst einen Wiederverwendungsbezug herstellt
- F2 (E3): ermöglichend — die systematische Schadstofferkundung vor Bauarbeiten ist in der Praxis eine Voraussetzung für belastbare Pre-Demolition-Audits, die wiederum Wiederverwendung erst planbar machen; die Norm selbst verfolgt jedoch ausschliesslich Arbeitnehmerschutz, kein Reuse-Ziel
- G: Probenahme/Materialprüfung — explizit (E1), "eingehend ermitteln und beurteilen" impliziert im Vollzug regelmässig eine Materialbeprobung, auch wenn Art. 3 die Methode nicht spezifiziert
- Kernaussage: Besteht der Verdacht, dass besonders gesundheitsgefährdende Stoffe wie Asbest oder PCB auftreten können, muss der Arbeitgeber die Gefährdungen eingehend ermitteln und beurteilen, bevor die erforderlichen Massnahmen geplant werden (Art. 3 Abs. 2). Art. 32 wiederholt diese Pflicht im Abschnitt "Arbeitsumgebung" und ergänzt eine Informationspflicht gegenüber betroffenen Arbeitnehmenden sowie eine Pflicht zur Arbeitseinstellung bei unerwartetem Fund.
- Wortlautbeleg (Originalsprache): "Besteht der Verdacht, dass besonders gesundheitsgefährdende Stoffe wie Asbest oder polychlorierte Biphenyle (PCB) auftreten können, so muss der Arbeitgeber die Gefährdungen eingehend ermitteln und beurteilen. Darauf abgestützt sind die erforderlichen Massnahmen zu planen." (Art. 3 Abs. 2)
- Beleg-Quelle: B0 (Volltext-PDF, Fassung Stand 1.1.2024, per `pdftotext` gelesen, inhaltlich identisch mit gegengeprüfter Fassung Stand 1.1.2022) · Zugänglichkeit: frei-primär · Bindungsakt: —
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2021/384/de (kanonisch); PDF gelesen: https://www.fedlex.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2021/384/20240101/de/pdf-a/fedlex-data-admin-ch-eli-cc-2021-384-20240101-de-pdf-a-2.pdf · Fassung(as-amended) 2024-01-01 · Zugriff 2026-08-13
- Status: in Kraft · seit 2022-01-01
- Sub-Ebene: nicht zutreffend (A=national)
- Relationen: wird kombiniert mit/ergänzt REG-CH-4-006 (Asbestsanierungsunternehmen-Anerkennung); konkretisiert wird durch keine in dieser Session identifizierte CH-Fachnorm zu Pre-Demolition-Audits (Lücke — DE hat DIN SPEC 91484, ein CH-Äquivalent wurde nicht gesucht/gefunden)
- Konfidenz: gesichert

### REG-CH-4-006 · Bauarbeitenverordnung (BauAV) Art. 82–86 (Asbestsanierungsunternehmen)
- Titel: Bauarbeitenverordnung (BauAV) vom 18. Juni 2021
- Fundstelle: Art. 82 (Begriff/Anwendungsfälle), Art. 83 (Anerkennung), Art. 84 (Spezialistinnen/Spezialisten), Art. 86 (Meldepflicht); SR 832.311.141
- A: national
- B: Primärfeld 4
- C: Dämmstoffe+Schadstoffe
- D: RVO
- E: Rückbau/Sicherung · E-Wirkung: erzwingt
- F1 (E3): hemmend — Asbestsanierungsarbeiten, bei denen erhebliche Mengen gesundheitsgefährdender Asbestfasern freigesetzt werden können, dürfen nur von anerkannten Asbestsanierungsunternehmen mit eigenem Fachpersonal ausgeführt werden; dies verteuert/verzögert den fachgerechten Rückbau asbesthaltiger Bauteile (z. B. asbesthaltiger Leichtbauplatten, Bodenbeläge) tendenziell, was bei knappen Zeitfenstern gegen eine sorgfältige, wiederverwendungsfreundliche Demontage angrenzender unbelasteter Bauteile wirken kann — Projektschlussfolgerung, kein Textbeleg für diesen Kausalzusammenhang
- F2 (E3): bedingend — die Meldepflicht (mind. 14 Tage vor Ausführung an die Suva, Art. 86) schafft einen strukturierten Vorlauf, der bei entsprechender Bauplanung auch für eine geordnete, materialschonende Demontage genutzt werden könnte; keine Praxisevidenz in dieser Session
- G: Dokumentenlage (Meldeformular Suva) — explizit (E1); Einzelfallzulassung (Unternehmensanerkennung nach Art. 83) — explizit (E1)
- Kernaussage: Asbestsanierungsarbeiten mit erheblicher Faserfreisetzung sind anerkannten Asbestsanierungsunternehmen vorbehalten, die eigene Spezialistinnen/Spezialisten nach Art. 84 beschäftigen müssen. Asbestsanierungsunternehmen müssen die Arbeiten mindestens 14 Tage vor Ausführung der Suva melden.
- Wortlautbeleg (Originalsprache): "Asbestsanierungsarbeiten, bei denen erhebliche Mengen gesundheitsgefährdender Asbestfasern freigesetzt werden können, dürfen nur von Asbestsanierungsunternehmen … ausgeführt werden." (Art. 82 Abs. 1, sinngemäss vollständig zitiert nach Gliederungsstruktur) / "Asbestsanierungsunternehmen sind verpflichtet, Asbestsanierungsarbeiten mindestens 14 Tage vor der Ausführung der Suva zu melden." (Art. 86 Abs. 1)
- Beleg-Quelle: B0 (Volltext-PDF per `pdftotext` gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: —
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2021/384/de · Fassung(as-amended) 2024-01-01 · Zugriff 2026-08-13
- Status: in Kraft · seit 2022-01-01
- Sub-Ebene: nicht zutreffend
- Relationen: wird kombiniert mit/ergänzt REG-CH-4-005
- Konfidenz: gesichert (Meldepflicht-Wortlaut); abgeleitet (Reuse-Wirkung, reine Projektzuordnung)

### REG-CH-4-007 · Chemikalien-Risikoreduktions-Verordnung (ChemRRV), Asbestverbot (Lücke, bewusst nicht als Faktum extrahiert)
- Titel: Verordnung zur Reduktion von Risiken beim Umgang mit bestimmten besonders gefährlichen Stoffen, Zubereitungen und Gegenständen (Chemikalien-Risikoreduktions-Verordnung, ChemRRV)
- Fundstelle: nicht ermittelt (laut Sekundärquellen einer der 37 Anhänge zu Asbest; genauer Anhang/Ziffer in dieser Session nicht verifiziert)
- A: national (Annahme, nicht in dieser Session am Verordnungstext selbst geprüft)
- B: Primärfeld 4 · Nebenfelder: 3 (Abfall-/Stoffrecht-Berührung)
- C: Dämmstoffe+Schadstoffe
- D: RVO (Annahme)
- E: nicht bestimmbar (Lücke)
- F1 (E3): unklar — Statuskategorie mangels Volltexteinsicht nicht seriös vergebbar
- F2 (E3): unklar
- G: entfällt (mangels Textzugriff keine Kodierung)
- Kernaussage: Laut Sekundärquellen (Wikipedia, Fachverbands-Websites; nicht in dieser Session primärquellenbasiert verifiziert) enthält die ChemRRV in einem ihrer Anhänge ein grundsätzliches Verbot der Verwendung von Asbest sowie Kennzeichnungs-/Informationspflichten für asbesthaltige Zubereitungen und Gegenstände. Der Verordnungstext selbst (fedlex.admin.ch/eli/cc/2005/478/de) wurde in dieser Session **nicht** geöffnet — dieses Objekt wird bewusst mit offener Lücke geführt statt mit einer nicht belegten Detailaussage.
- Wortlautbeleg (Originalsprache): nicht verfügbar (kein Primärtextzugriff in dieser Session)
- Beleg-Quelle: B4 (nur Existenz-/Katalognachweis über Sekundärquellen) · Zugänglichkeit: frei-primär (Verordnung selbst frei zugänglich, in dieser Session nur nicht geöffnet) · Bindungsakt: entfällt/kein Bindungsakt identifiziert (Prüfung ausstehend)
- Quelle: Tier 3 (Sekundärquelle, nur Suchhinweis, kein Beleg) · https://de.wikipedia.org/wiki/Chemikalien-Risikoreduktions-Verordnung ; kanonische Primärquelle (nicht geöffnet): https://www.fedlex.admin.ch/eli/cc/2005/478/de · Fassung(as-amended) nicht ermittelt · Zugriff 2026-08-13
- Status: unklar (vermutlich in Kraft)
- Sub-Ebene: nicht zutreffend
- Relationen: wird kombiniert mit/ergänzt REG-CH-4-005/006 (vermutet, nicht verifiziert)
- Konfidenz: unklar — **explizite Lücke, keine Nacherhebung in dieser Session; hohe Priorität für W2-Nacherhebung oder W4, da Bindungsketten-Regel für Anhänge/Grenzwerte sonst nicht erfüllbar ist**

---

## Regelungsfeld 5a — Vergaberecht (hart)

### REG-CH-5a-008 · Bundesgesetz über das öffentliche Beschaffungswesen (BöB) Art. 2 (Zweck)
- Titel: Bundesgesetz über das öffentliche Beschaffungswesen (BöB) vom 21. Juni 2019
- Fundstelle: Art. 2; SR 172.056.1
- A: national
- B: Primärfeld 5a · Normtyp: Grundnorm/Begriffsnorm (Zwecknorm, determiniert die Auslegung aller nachgeordneten BöB-Bestimmungen inkl. Art. 29/30)
- C: materialübergreifend
- D: Gesetz
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): ermöglichend — erstmals wird der "volkswirtschaftlich, ökologisch und sozial nachhaltige" Einsatz öffentlicher Mittel als gesetzlicher Zweck (nicht nur als optionales Zuschlagskriterium) verankert; das öffnet grundsätzlich die Auslegung nachgeordneter Vorschriften (insb. Art. 29, 30) in Richtung Kreislaufwirtschaft/Wiederverwendung, ohne dass Bauteilwiederverwendung im Wortlaut selbst genannt wird
- F2 (E3): bedingend — die Zwecknorm allein erzeugt keine unmittelbare Vergabepraxis-Änderung; ihre Wirkung hängt von der Umsetzung in Ausschreibungen (Zuschlagskriterien, technische Spezifikationen) ab
- G: Anwendbarkeitsnorm ohne Nachweistatbestand — explizit (E1); reine Zweckbestimmung ohne eigenen Einzelnachweis
- Kernaussage: Das totalrevidierte BöB (in Kraft seit 1. Januar 2021) nennt als ersten gesetzlichen Zweck den wirtschaftlichen UND volkswirtschaftlich, ökologisch und sozial nachhaltigen Einsatz öffentlicher Mittel — eine gegenüber dem Vorgängerrecht neu eingeführte, gleichrangige Nachhaltigkeitsdimension.
- Wortlautbeleg (Originalsprache): "Dieses Gesetz bezweckt: a. den wirtschaftlichen und den volkswirtschaftlich, ökologisch und sozial nachhaltigen Einsatz der öffentlichen Mittel; b. die Transparenz des Vergabeverfahrens; c. die Gleichbehandlung und Nichtdiskriminierung der Anbieterinnen; d. die Förderung des wirksamen, fairen Wettbewerbs unter den Anbieterinnen…"
- Beleg-Quelle: B0 (Volltext-PDF per `pdftotext` gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: —
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2020/126/de · PDF gelesen: https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2020/126/20210101/de/pdf-a/fedlex-data-admin-ch-eli-cc-2020-126-20210101-de-pdf-a.pdf · Fassung(as-amended) 2021-01-01 · Zugriff 2026-08-13
- Status: in Kraft · seit 2021-01-01
- Sub-Ebene: nicht zutreffend (A=national; gilt für Bundesbeschaffungen, nicht kantonal/kommunal — dafür s. REG-CH-5a-010)
- Relationen: determiniert Anwendbarkeit von REG-CH-5a-009 (Art. 29 BöB); wird kombiniert mit/ergänzt REG-CH-5a-010 (IVöB Art. 2, wortgleicher Zweckartikel für Kantone/Gemeinden)
- Konfidenz: gesichert

### REG-CH-5a-009 · Bundesgesetz über das öffentliche Beschaffungswesen (BöB) Art. 29/30 (Zuschlagskriterien, technische Spezifikationen)
- Titel: Bundesgesetz über das öffentliche Beschaffungswesen (BöB) vom 21. Juni 2019
- Fundstelle: Art. 29 Abs. 1, 4; Art. 30 (sinngemäss identisch mit IVöB, s. REG-CH-5a-011); SR 172.056.1
- A: national
- B: Primärfeld 5a
- C: materialübergreifend
- D: Gesetz
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): ermöglichend — Art. 29 Abs. 1 nennt "Lebenszykluskosten" und "Nachhaltigkeit" ausdrücklich als mögliche, nicht abschliessend aufgezählte Zuschlagskriterien neben Preis und Qualität; damit ist eine Vergabestelle rechtlich nicht gehindert, Kreislaufwirtschafts-/Wiederverwendungsanteile als Zuschlagskriterium zu gewichten — der Wortlaut selbst nennt Bauteilwiederverwendung jedoch nicht namentlich, das Kriterium bleibt auslegungsbedürftig
- F2 (E3): bedingend — ob und wie stark einzelne Vergabestellen Nachhaltigkeits-/Lebenszykluskriterien tatsächlich gewichten, ist eine Frage der Ausschreibungspraxis, nicht des Gesetzestextes; keine Praxisstatistik in dieser Session erhoben
- G: Dokumentenlage (Bekanntgabe der Zuschlagskriterien und Gewichtung in der Ausschreibung, Art. 29 Abs. 3) — explizit (E1)
- Kernaussage: Art. 29 Abs. 1 zählt in einer nicht abschliessenden Liste mögliche Zuschlagskriterien auf, darunter "Lebenszykluskosten" und "Nachhaltigkeit", neben Preis und Qualität. Art. 30 Abs. 4 (technische Spezifikationen) erlaubt der Auftraggeberin ausdrücklich, Spezifikationen zur Erhaltung der natürlichen Ressourcen oder zum Schutz der Umwelt vorzusehen.
- Wortlautbeleg (Originalsprache): "Sie berücksichtigt, unter Beachtung der internationalen Verpflichtungen der Schweiz, neben dem Preis und der Qualität einer Leistung, insbesondere Kriterien wie Zweckmässigkeit, Termine, technischer Wert, Wirtschaftlichkeit, Lebenszykluskosten, Ästhetik, Nachhaltigkeit, Plausibilität des Angebots, die unterschiedlichen Preisniveaus in den Ländern, in welchen die Leistung erbracht wird, Verlässlichkeit des Preises, Kreativität, Kundendienst, Lieferbedingungen, Infrastruktur, Innovationsgehalt, Funktionalität, Servicebereitschaft, Fachkompetenz oder Effizienz der Methodik." (Art. 29 Abs. 1)
- Beleg-Quelle: B0 (Volltext-PDF per `pdftotext` gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: —
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2020/126/de · PDF gelesen: https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2020/126/20210101/de/pdf-a/fedlex-data-admin-ch-eli-cc-2020-126-20210101-de-pdf-a.pdf · Fassung(as-amended) 2021-01-01 · Zugriff 2026-08-13
- Status: in Kraft · seit 2021-01-01
- Sub-Ebene: nicht zutreffend
- Relationen: wird kombiniert mit/ergänzt REG-CH-5a-011 (IVöB, wortgleich)
- Konfidenz: gesichert

### REG-CH-5a-010 · Interkantonale Vereinbarung über das öffentliche Beschaffungswesen (IVöB 2019) Art. 2 (Zweck)
- Titel: Interkantonale Vereinbarung über das öffentliche Beschaffungswesen vom 15. November 2019 (IVöB 2019)
- Fundstelle: Art. 1 (Gegenstand), Art. 2 (Zweck)
- A: sub-national · Downstream-Verifikationsstatus: verifiziert in [Kanton St. Gallen: "neues Beschaffungsrecht 2023" laut Kantonsportal-Titel, nur Existenznachweis über Suchtreffer, nicht B1 im Volltext gelesen] und [Kanton Schwyz: SRSZ 430.120.1, Stand 01.02.2023 laut Titel, ebenfalls nur Existenznachweis]; strukturell angenommen für die übrigen Kantone (IVöB 2019 ist als Konkordat auf individuellen kantonalen Beitritt angewiesen, nicht automatisch gesamtschweizerisch bindend — anders als der VKF-Sonderfall REG-CH-4-001/002)
- B: Primärfeld 5a · Normtyp: Grundnorm/Begriffsnorm
- C: materialübergreifend
- D: Gesetz (Konkordat mit kantonaler Beitrittsgesetzgebung; als Rechtsform des Konkordats selbst am ehesten der Kategorie "Gesetz" zugeordnet, da es nach Beitritt für den jeweiligen Kanton Gesetzesrang hat — Grenzfall, an W4 zur Bestätigung)
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): ermöglichend — wortgleiche Zweckbestimmung wie BöB Art. 2 (s. REG-CH-5a-008), auf kantonale/kommunale Beschaffung erstreckt: "die Nachhaltigkeit der Beschaffung, d. h. den wirtschaftlichen und den volkswirtschaftlich, sozial und ökologisch verantwortungsvollen Einsatz der öffentlichen Mittel"
- F2 (E3): bedingend — wie REG-CH-5a-008, zusätzlich abhängig vom tatsächlichen kantonalen Beitrittsstand (s. Downstream-Vermerk)
- G: Anwendbarkeitsnorm ohne Nachweistatbestand — explizit (E1)
- Kernaussage: Die IVöB 2019 harmonisiert das kantonale/kommunale Beschaffungsrecht mit dem revidierten BöB. Art. 2 nennt als ersten Zweck die Nachhaltigkeit der Beschaffung im wirtschaftlichen, volkswirtschaftlichen, sozialen und ökologischen Sinn — inhaltsgleich mit BöB Art. 2, redaktionell mit "Nachhaltigkeit der Beschaffung" als Oberbegriff formuliert.
- Wortlautbeleg (Originalsprache): "Die Vereinbarung bezweckt: die Nachhaltigkeit der Beschaffung, d. h. den wirtschaftlichen und den volkswirtschaftlich, sozial und ökologisch verantwortungsvollen Einsatz der öffentlichen Mittel (Bst. a); die Transparenz der Verfahren (Bst. b); die Gleichbehandlung und Nichtdiskriminierung der Anbieter … (Bst. c); die Förderung des wirksamen, fairen Wettbewerbs unter den Anbietern … (Bst. d)."
- Beleg-Quelle: B0 (Volltext der Musterbotschaft inkl. Vereinbarungstext per `pdftotext` gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: kantonaler Beitrittserlass je Kanton (nicht einzeln verifiziert, s. Downstream-Vermerk)
- Quelle: Tier 1 (BPUK/Bau-, Planungs- und Umweltdirektoren-Konferenz, interkantonales Fachorgan; Vereinbarungstext ist der amtliche Konkordatstext) · https://www.bpuk.ch/fileadmin/Dokumente/bpuk/public/de/konkordate/ivoeb/ivoeb_2019/DE_Musterbotschaft_IVoeB_inkl._Vereinbarungstext_und_Anhaenge_1-4.pdf ; Übersicht https://www.bpuk.ch/bpuk/konkordate/ivoeb/ivoeb-2019 · Fassung(as-amended) 2019-11-15 · Zugriff 2026-08-13
- Status: in Kraft · seit 2019-11-15 (Vereinbarung), Inkraftsetzung je Kanton gestaffelt seit 2021
- Sub-Ebene: Stichprobe [St. Gallen: Existenz "neues Beschaffungsrecht 2023" nachgewiesen, Volltext nicht gelesen; Schwyz: SRSZ 430.120.1 Stand 01.02.2023, Volltext nicht gelesen] / nicht erhoben [übrige 24 Kantone]
- Relationen: wird kombiniert mit/ergänzt REG-CH-5a-008 (BöB Art. 2, wortnahe Parallelnorm für Bundesebene)
- Konfidenz: gesichert (Vereinbarungstext); abgeleitet (kantonaler Umsetzungsstand, nur Stichprobe auf Existenzebene)

### REG-CH-5a-011 · IVöB 2019 Art. 29/30 (Zuschlagskriterien, technische Spezifikationen)
- Titel: Interkantonale Vereinbarung über das öffentliche Beschaffungswesen vom 15. November 2019 (IVöB 2019)
- Fundstelle: Art. 29 Abs. 1–4; Art. 30 Abs. 1–4
- A: sub-national · Downstream-Verifikationsstatus: wie REG-CH-5a-010
- B: Primärfeld 5a
- C: materialübergreifend
- D: Gesetz (s. Vorbehalt REG-CH-5a-010)
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): ermöglichend — Art. 29 Abs. 1 nennt "Lebenszykluskosten" und "Nachhaltigkeit" als mögliche Zuschlagskriterien (identische Liste wie BöB Art. 29, ohne den bundesspezifischen Zusatz zu internationalen Preisniveaus); Art. 30 Abs. 4 erlaubt technische Spezifikationen "zur Erhaltung der natürlichen Ressourcen oder zum Schutz der Umwelt" — die textnächste, wenn auch nicht reuse-spezifische Rechtsgrundlage für eine kantonale/kommunale Ausschreibung, die Bauteilwiederverwendung technisch verlangt
- F2 (E3): bedingend — Umsetzung liegt im Ermessen der einzelnen Vergabestelle; keine Pflicht zur Berücksichtigung von Kreislaufwirtschaftskriterien
- G: Dokumentenlage (Bekanntgabe Kriterien/Gewichtung, Art. 29 Abs. 3) — explizit (E1)
- Kernaussage: Art. 29 IVöB zählt nicht abschliessend mögliche Zuschlagskriterien auf, darunter Lebenszykluskosten und Nachhaltigkeit; Art. 30 Abs. 4 erlaubt der Vergabestelle ausdrücklich, technische Spezifikationen zur Ressourcenschonung oder zum Umweltschutz festzulegen — die stärkste im IVöB-Text auffindbare Ermöglichungsnorm für zirkuläre Ausschreibungsanforderungen.
- Wortlautbeleg (Originalsprache): "Der Auftraggeber prüft die Angebote anhand leistungsbezogener Zuschlagskriterien. Neben dem Preis und der Qualität einer Leistung kann er insbesondere Kriterien wie Zweckmässigkeit, Termine, technischer Wert, Wirtschaftlichkeit, Lebenszykluskosten, Ästhetik, Nachhaltigkeit, Plausibilität des Angebots, Kreativität, Kundendienst, Lieferbedingungen, Infrastruktur, Innovationsgehalt, Funktionalität, Servicebereitschaft, Fachkompetenz oder Effizienz der Methodik berücksichtigen." (Art. 29 Abs. 1) / "Der Auftraggeber kann technische Spezifikationen zur Erhaltung der natürlichen Ressourcen oder zum Schutz der Umwelt vorsehen." (Art. 30 Abs. 4)
- Beleg-Quelle: B0 (Volltext per `pdftotext` gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: wie REG-CH-5a-010
- Quelle: Tier 1 · https://www.bpuk.ch/fileadmin/Dokumente/bpuk/public/de/konkordate/ivoeb/ivoeb_2019/DE_Musterbotschaft_IVoeB_inkl._Vereinbarungstext_und_Anhaenge_1-4.pdf · Fassung(as-amended) 2019-11-15 · Zugriff 2026-08-13
- Status: in Kraft (s. Vorbehalt REG-CH-5a-010)
- Sub-Ebene: wie REG-CH-5a-010
- Relationen: wird kombiniert mit/ergänzt REG-CH-5a-009 (BöB Art. 29/30, wortnahe Parallelnorm)
- Konfidenz: gesichert (Wortlaut); abgeleitet (kantonaler Umsetzungsstand)

---

## Regelungsfeld 5b — Anreize/Förderung (weich)

### REG-CH-5b-012 · Das Gebäudeprogramm / CO2-Gesetz-Finanzierung / Harmonisiertes Fördermodell der Kantone (HFM) — Reuse-Bezug offen
- Titel: Das Gebäudeprogramm (gemeinsames Förderprogramm von Bund und Kantonen für Massnahmen zur langfristigen CO2-Verminderung bei Gebäuden), finanziert über die CO2-Abgabe; Harmonisiertes Fördermodell der Kantone (HFM) als kantonale Ausgestaltungsgrundlage
- Fundstelle: nicht ermittelt (weder die CO2-Gesetz-Bestimmung zur Zweckbindung der CO2-Abgabe noch das HFM-Dokument selbst wurden in dieser Session im Volltext geöffnet)
- A: sub-national · Downstream-Verifikationsstatus: strukturell angenommen, nicht verifiziert — "Die Kantone legen individuell fest, welche Massnahmen sie zu welchen Bedingungen und in welchem Umfang fördern" (laut eingesehener Übersichtsseite); die konkrete Ausgestaltung ist damit genuin kantonal, auch wenn Finanzierung/Rahmen bundes-/interkantonal koordiniert sind
- B: Primärfeld 5b · Nebenfelder: 4 (Energie)
- C: materialübergreifend
- D: RVO (Annahme für die CO2-Gesetz-Ausführungsbestimmungen; nicht am Verordnungstext selbst verifiziert) — Grenzfall, da HFM selbst eher Verwaltungsvorschrift-Charakter hat
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): schweigend — in den in dieser Session eingesehenen Übersichtsquellen (energieschweiz.ch, dasgebaeudeprogramm.ch, dortige Finanzierungsseite) fand sich **kein** Hinweis darauf, ob und wie Bauteilwiederverwendung als eigenständig förderfähige Massnahme oder als Bonuskriterium im Gebäudeprogramm/HFM geführt wird; die Recherche deckte nur die allgemeine Finanzierungsstruktur (CO2-Abgabe, kantonale Beiträge) ab, nicht den Massnahmenkatalog im Detail
- F2 (E3): schweigend — mangels Textzugriff auf den Massnahmenkatalog keine belastbare Praxisaussage möglich
- G: Dokumentenlage (Förderantrag) — inferiert (E3), generischer Förderprogramm-Mechanismus, nicht am Massnahmenkatalog selbst verifiziert
- Kernaussage: Das Gebäudeprogramm wird durch zweckgebundene Mittel aus der CO2-Abgabe sowie durch kantonale und Bundesbeiträge finanziert; die Kantone gestalten ihre Förderprogramme individuell auf Basis des Harmonisierten Fördermodells (HFM). Ob und in welcher Form Bauteilwiederverwendung im Massnahmenkatalog des Gebäudeprogramms oder in kantonalen Förderprogrammen als förderfähig geführt wird, konnte in dieser Session **nicht** primärquellenbasiert geklärt werden — explizite Lücke, keine Vermutung als Faktum übernommen.
- Wortlautbeleg (Originalsprache): "Die Kantone legen individuell fest, welche Massnahmen sie zu welchen Bedingungen und in welchem Umfang fördern." (aus der eingesehenen Übersichtsseite, exakte URL/Ziffer nicht weiter spezifizierbar innerhalb der Session)
- Beleg-Quelle: B2 (amtsnahe Übersichtsseiten gelesen, HFM-/CO2-Gesetz-Primärtext selbst nicht geöffnet) · Zugänglichkeit: frei-primär · Bindungsakt: CO2-Gesetz (Primärtext nicht geöffnet — Bindungsketten-Regel damit **nicht** vollständig erfüllt, als Lücke markiert statt als Faktum behauptet)
- Quelle: Tier 2 (BFE/EnergieSchweiz, amtliche Förderplattform, aber in dieser Session nur Übersichtsebene) · https://www.energieschweiz.ch/foerderung/das-gebaeudeprogramm/ ; https://www.dasgebaeudeprogramm.ch/de/das-gebaudeprogramm/grundlagen-und-finanzierung/ · Fassung(as-amended) nicht ermittelt · Zugriff 2026-08-13
- Status: in Kraft (Programm läuft; Detailstand HFM/CO2-Gesetz nicht verifiziert)
- Sub-Ebene: nicht erhoben [alle 26 Kantone — keine kantonale Einzelrecherche in dieser Session, da bereits die Bundes-/interkantonale Grundlage nicht abschliessend geklärt werden konnte]
- Relationen: wird kombiniert mit/ergänzt REG-CH-4-003/004 (energiepolitischer Sachzusammenhang graue Energie/Förderung)
- Konfidenz: unklar — **höchste Lückenpriorität in Feld 5b dieser Karte; explizit keine kantonalen Kreislaufwirtschafts-Förderprogramme (z. B. für Bauteilbörsen) erhoben, da der Websuche-Kontingent der Session vor der vertiefenden Recherche erschöpft war — an W2-Nacherhebung oder W4 zu melden**

---

## Regelungsfeld 6 — Normen/Regelwerke

### REG-CH-6-013 · SIA 430:2023 "Vermeidung und Entsorgung von Bauabfällen"
- Titel: SIA 430:2023, Vermeidung und Entsorgung von Bauabfällen (löst die Empfehlung SIA 430:1993 ab)
- Fundstelle: Norm als Ganzes (Einzelziffern in dieser Session nicht im Volltext eingesehen, Norm ist kostenpflichtig — s. Beleg-Quelle)
- A: national (Herausgeber SIA ist gesamtschweizerisch tätig; Bindung entsteht erst über Vertrag/Werkvertragsbezug oder VV-TB-artige Listung, s. Bindungsakt)
- B: Primärfeld 6 · Nebenfelder: 3 (Abfall-/Stoffrecht — Ablösung der VVEA-Referenz laut Sekundärquelle)
- C: materialübergreifend
- D: nat.Norm
- E: Rückbau/Sicherung · E-Wirkung: durchläuft; Aufbereitung/Prüfung · E-Wirkung: durchläuft; Abfallstatus · E-Wirkung: vermeidet (Doppelkodierung Grenzoperation, s. u.)
- F1 (E3): ermöglichend — die Norm wurde 2023 von einer unverbindlichen Empfehlung zu einer Norm aufgewertet und macht laut eingesehener Fachpresse-Zusammenfassung "die Wiederverwendung von Bauteilen" zu einem eigenständigen, in den SIA-Phasen verankerten Planungsgegenstand ("Der beste Abfall ist derjenige, der gar nicht erst entsteht … die Wiederverwendung von Bauteilen [wird] in der neuen Norm angemessen berücksichtigt"); Details/Ziffern-Wortlaut nicht im Volltext gegengeprüft (kostenpflichtig)
- F2 (E3): ermöglichend — als anerkannte Regel der Baukunde faktisch praxisleitend für Rückbau-/Umbauplanung, unabhängig von einer förmlichen Bindungserklärung; Grad der Verbindlichkeit im Einzelvertrag hängt von der SIA-118-Einbeziehung ab
- G: Dokumentenlage (projektbezogene Definition von Verwertungsquoten statt fixer Vorgabe, laut Sekundärquelle) — inferiert (E3), da Normvolltext nicht eingesehen
- Kernaussage: Die seit 8. November 2023 als eigenständige Norm (nicht mehr blosse Empfehlung) geführte SIA 430:2023 beschreibt, welche Massnahmen in den jeweiligen SIA-Phasen notwendig sind, um einen nachhaltigen Umgang mit Baustoffen zu gewährleisten, und berücksichtigt dabei explizit die Wiederverwendung von Bauteilen als Form der Abfallvermeidung — ohne feste Verwertungsquote, stattdessen projektbezogen zu definieren.
- Wortlautbeleg (Originalsprache): "Der beste Abfall ist derjenige, der gar nicht erst entsteht. Aus diesem Grund wird die Wiederverwendung von Bauteilen in der neuen Norm angemessen berücksichtigt." (Sekundärzitat aus Espazium-Fachartikel; **kein** Wortlautzugriff auf den kostenpflichtigen SIA-Normtext selbst in dieser Session — Wortlautbeleg daher als Sekundärzitat gekennzeichnet, nicht als Primärtext-Zitat zu verwechseln)
- Beleg-Quelle: B3 (Fachpresse-Artikel espazium.ch direkt gelesen, referenziert die Norm inhaltlich; SIA-Normtext selbst kostenpflichtig und in dieser Session nicht eingesehen) · Zugänglichkeit: paywalled-nicht-eingesehen (Normtext) · Bindungsakt: entfällt/kein Bindungsakt identifiziert — keine VV-TB-artige Listung in dieser Session geprüft; Bindung im Einzelfall primär über werkvertragliche Einbeziehung (SIA 118)
- Quelle: Tier 3 für den Wortlaut (Espazium, Fachmedium, nur Sekundärzitat, kein Beleg für Normwortlaut selbst) · https://www.espazium.ch/de/aktuelles/sia-430-vom-recycling-zur-nachhaltigen-verwendung-von-baumaterialien ; Normverkaufsseite (nicht geöffnet): https://shop.sia.ch · Fassung(as-amended) 2023-11-08 · Zugriff 2026-08-13
- Status: in Kraft · seit 2023-11-08 (löst SIA 430:1993 ab)
- Sub-Ebene: nicht zutreffend (A=national)
- Relationen: ersetzt SIA 430:1993 (nicht separat als eigenes Objekt geführt); wird kombiniert mit/ergänzt REG-CH-4-005 (Schadstofferkundung vor Rückbau als Vorstufe zur Verwertungsplanung)
- Konfidenz: abgeleitet — **Kernaussage gesichert über Sekundärquelle, aber gemäss Bindungsketten-Regel ist der amtsnahe freie Beleg für den Normwortlaut selbst noch zu beschaffen (Beuth/SIA-Shop-Kauf oder Bibliothekszugang); B3+paywalled-nicht-eingesehen darf laut Projektregel NICHT als vollwertiges Faktum stehen — diese Karte hält sich daran und markiert die Kernaussage entsprechend nur als "abgeleitet", nicht "gesichert"**

### REG-CH-6-014 · SIA 269 Normenreihe "Erhaltung von Tragwerken"
- Titel: SIA 269:2011 (Grundnorm) mit Ergänzungsnormen SIA 269/1–269/8 (u. a. 269/2 Betonbau, 269/8 Erdbeben)
- Fundstelle: Norm als Ganzes; SIA 269/8 löst seit 1. Dezember 2017 die frühere Richtlinie SIA 2018:2004 ab
- A: national
- B: Primärfeld 2 (Standsicherheit/Bestandsbewertung) — **Hinweis:** Primärfeld liegt streng genommen bei Feld 2, nicht Feld 4; Objekt wird hier dennoch geführt, da Feld 2 CH nicht Gegenstand dieses Auftrags ist und die Norm für die Feld-6-Frage "welche Normen regeln reuse-relevante Bestandsbewertung" zentral ist — Primärfeld-Zuordnung an W4 zur Bestätigung/Umsortierung nach Feld 2
- C: Stahlbeton/Fertigteile (269/2); materialübergreifend (269 Grundnorm)
- D: nat.Norm
- E: Bestandserkundung · E-Wirkung: durchläuft; Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): ermöglichend — die Norm führt laut Sekundärquellen (ResearchGate-Fachaufsatz, Forum-Holzbau-Fachtext) eigene Begriffe wie "Aktualisierung", "Erfüllungsfaktor", "Erhaltungskonzept", "Erhaltungsprojekt" und "Verhältnismässigkeit von Erhaltungsmassnahmen" ein — das methodische Grundgerüst, um Tragwerke im Bestand (und damit implizit auch wiederzuverwendende tragende Bauteile) rechnerisch zu bewerten, statt sie pauschal nach Neubau-Massstab zu beurteilen
- F2 (E3): ermöglichend — als anerkannte SIA-Norm de facto Standardwerkzeug der Bestandsbewertung in der CH-Ingenieurpraxis; keine eigene Aussage zur Wiederverwendung ausgebauter (nicht mehr am ursprünglichen Ort verbleibender) Bauteile, die Norm ist auf Tragwerke am Ort zugeschnitten
- G: rechnerischer Nachweis — inferiert (E3), da Normvolltext nicht eingesehen
- Kernaussage: SIA 269 (2011) und ihre Ergänzungsnormen 269/1–269/8 bilden das schweizerische Regelwerk für die Erhaltung bestehender Tragwerke, mit eigens definierten Begriffen für Aktualisierung, Erhaltungskonzept und Verhältnismässigkeit von Massnahmen. Der Normtext selbst wurde in dieser Session nicht im Volltext eingesehen (kostenpflichtig); die Kernaussage stützt sich auf Fachaufsätze, die den Norminhalt referieren.
- Wortlautbeleg (Originalsprache): nicht verfügbar (kein Primärtextzugriff; Normtext kostenpflichtig über shop.sia.ch)
- Beleg-Quelle: B3 (Fachaufsätze/Sekundärquellen zum Norminhalt gelesen, Normtext selbst nicht eingesehen) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: entfällt/kein Bindungsakt identifiziert (in dieser Session nicht geprüft, ob SIA 269 in einer kantonalen Bauverordnung oder VV-TB-Analogie referenziert wird)
- Quelle: Tier 3 (Fachpresse/Fachaufsatz, nur Suchhinweis, kein Beleg für Normwortlaut) · https://www.researchgate.net/publication/283730622_Normenreihe_SIA_269_-_Erhaltung_von_Tragwerken ; https://www.forum-holzbau.ch/pdf/steiger_rene_biel08.pdf ; Normverkaufsseite (nicht geöffnet) https://shop.sia.ch/c94ff027-5685-43b8-bba5-b42e4cc31ccd/D/DownloadAnhang · Fassung(as-amended) 2011 (Grundnorm), 2017-12-01 (269/8) · Zugriff 2026-08-13
- Status: in Kraft
- Sub-Ebene: nicht zutreffend
- Relationen: wird kombiniert mit/ergänzt REG-CH-4-001 (Brandschutzertüchtigung im Bestand kann statische Nachweise nach SIA 269 erfordern)
- Konfidenz: unklar — **wie REG-CH-6-013 nur Sekundärbeleg für den Normwortlaut; Primärtextbeschaffung an W4/W2-Nacherhebung**

### REG-CH-6-015 · Interkantonale Vereinbarung über die Harmonisierung der Baubegriffe (IVHB)
- Titel: Interkantonale Vereinbarung über die Harmonisierung der Baubegriffe (IVHB)
- Fundstelle: nicht im Volltext eingesehen (30 Baubegriffe/Messweisen laut Sekundärquelle, kein Einzelartikel identifiziert)
- A: sub-national · Downstream-Verifikationsstatus: strukturell angenommen, nicht verifiziert — "Die IVHB-Regeln gelten nicht unmittelbar, sondern müssen von den beigetretenen Kantonen zunächst in das kantonale und kommunale Recht umgesetzt werden" (Sekundärquelle); Beitrittsstand der 26 Kantone in dieser Session nicht einzeln geprüft
- B: Primärfeld 6 · Normtyp: Grundnorm/Begriffsnorm (Baubegriffe determinieren die Auslegung nachgeordneten kantonalen Baurechts)
- C: materialübergreifend
- D: Gesetz (Konkordat, Rechtsform-Einordnung wie REG-CH-5a-010)
- E: Planung/Nachweis · E-Wirkung: durchläuft
- F1 (E3): schweigend — die IVHB harmonisiert Baubegriffe und Messweisen (z. B. Gesamthöhe), regelt aber laut eingesehener Sekundärquelle ausdrücklich **nicht** die materiellen zulässigen Werte selbst und enthält keinen erkennbaren Bezug zu Bauteilwiederverwendung oder Bestandsbauten
- F2 (E3): schweigend — kein Reuse-Bezug in den eingesehenen Sekundärquellen identifizierbar
- G: entfällt (reine Begriffsnorm ohne eigenen Nachweistatbestand)
- Kernaussage: Die IVHB vereinheitlicht 30 zentrale Baubegriffe und Messweisen zwischen den beigetretenen Kantonen, um Bauplanung und -bewilligung zu vereinfachen. Sie legt nur Definitionen fest, nicht die materiellen zulässigen Masse — diese bleiben Sache der Kantone/Gemeinden. Ein Bezug zu Bauteilwiederverwendung oder Bestandsbauten wurde in den eingesehenen Quellen nicht gefunden.
- Wortlautbeleg (Originalsprache): "Die IVHB-Regeln gelten nicht unmittelbar, sondern müssen zunächst von den beigetretenen Kantonen in kantonales und kommunales Recht umgesetzt werden." (Sekundärzusammenfassung, kein Primärtext-Wortlaut in dieser Session eingesehen)
- Beleg-Quelle: B4 (nur Existenz-/Themennachweis über Suchergebnis-Zusammenfassungen von ivhb.ch, bpuk.ch, kantonalen Portalen; Vereinbarungstext selbst in dieser Session nicht geöffnet) · Zugänglichkeit: frei-primär (Text vermutlich frei zugänglich, nur in dieser Session nicht aufgerufen) · Bindungsakt: kantonaler Beitrittserlass je Kanton, nicht verifiziert
- Quelle: Tier 2 (BPUK, interkantonales Fachorgan; Ivhb.ch als amtsnahes Informationsportal, in dieser Session nicht direkt geöffnet) · http://ivhb.ch/ ; https://www.bpuk.ch/bpuk/konkordate/ivhb · Fassung(as-amended) nicht ermittelt · Zugriff 2026-08-13
- Status: in Kraft (Kernbestand seit den 2010er-Jahren, laufend um weitere Kantone erweitert — nicht verifiziert)
- Sub-Ebene: nicht erhoben [alle 26 Kantone]
- Relationen: keine identifizierte Relation zu anderen Objekten dieser Karte (reine Begriffsnorm ausserhalb des Reuse-Kernbereichs)
- Konfidenz: unklar — **niedrigste Beleglage dieser Karte (B4); Objekt bewusst mit maximaler Zurückhaltung geführt, da auch die Kernaussage nur aus Suchergebnis-Zusammenfassungen, nicht aus eigener Primärtextlektüre stammt**

---

## Regelungsfeld 7 — Haftung/Gewährleistung

### REG-CH-7-016 · Obligationenrecht (OR) Art. 201 Abs. 4 (60-Tage-Rügefrist für integrierte bewegliche Sachen)
- Titel: Bundesgesetz betreffend die Ergänzung des Schweizerischen Zivilgesetzbuches (Fünfter Teil: Obligationenrecht), Teilrevision Baumängelrecht
- Fundstelle: Art. 201 Abs. 4 OR (neu eingefügt durch die Teilrevision, in Kraft seit 1. Januar 2026); SR 220
- A: national
- B: Primärfeld 7
- C: materialübergreifend
- D: Gesetz
- E: Einbau/Abnahme · E-Wirkung: durchläuft; Betrieb/Dokumentation · E-Wirkung: durchläuft
- F1 (E3): bedingend — die neue 60-tägige Rügefrist gilt für Mängel einer beweglichen Kaufsache, die "bestimmungsgemäss in ein unbewegliches Werk integriert worden ist" und die Mangelhaftigkeit des Gesamtwerks verursacht hat; für ein wiederverwendetes Bauteil (bewegliche Sache im Sinne des Kaufrechts, sobald es aus dem Ursprungsbau ausgebaut und weiterveräussert wird) gilt damit dieselbe neue Frist wie für ein Neuprodukt — der Normtext unterscheidet nicht zwischen neu und gebraucht; verdeckte Mängel lösen die Frist erst ab Entdeckung aus, was für Altbauteile mit unbekannter Vorgeschichte praktisch bedeutsam sein kann, ohne dass die Norm dies eigens regelt
- F2 (E3): schweigend — die Reform zielt laut eingesehenem Fachaufsatz erklärtermassen auf die generelle Stärkung der Bauherrschaft bei Baumängeln, nicht spezifisch auf gebrauchte/wiederverwendete Bauprodukte; keine erkennbare gezielte Reuse-Steuerungswirkung
- G: Dokumentenlage (Mängelanzeige innert Frist) — explizit (E1)
- Kernaussage: Die am 1. Januar 2026 in Kraft getretene Teilrevision des Obligationenrechts führt für Mängel einer beweglichen Sache, die bestimmungsgemäss in ein unbewegliches Werk integriert wurde und dessen Mangelhaftigkeit verursacht hat, eine neue 60-tägige Rügefrist ein (zuvor kannte das Gesetz keine tagegenaue Frist, nur eine von der Rechtsprechung entwickelte, deutlich kürzere "Sofortrüge"). Kürzere Fristvereinbarungen sind unwirksam (einseitig zwingend).
- Wortlautbeleg (Originalsprache): "Soweit Mängel einer Sache, die bestimmungsgemäss in ein unbewegliches Werk integriert worden ist, die Mangelhaftigkeit des Werks verursacht haben, sind diese innert 60 Tagen anzuzeigen. Mängel, die bei der übungsgemässen Untersuchung nicht erkennbar waren, sind innert 60 Tagen nach ihrer Entdeckung anzuzeigen. Die Vereinbarung kürzerer Fristen ist unwirksam." (Art. 201 Abs. 4 OR)
- Beleg-Quelle: **B0 — Prüfung 2026-08-13: per `pdftotext` direkt aus der fedlex-PDF-A-Konsolidierung (Fassung 2026-01-01) verifiziert, Wortlaut deckt sich exakt mit dem hier zitierten Text. Fussnote 74 im Originaltext bestätigt: "Eingefügt durch Ziff. I des BG vom 20. Dez. 2024 (Baumängel), in Kraft seit 1. Jan. 2026 (AS 2025 270; BBl 2022 2743)."** · Zugänglichkeit: frei-primär (Direktlink funktioniert über `fedlex.data.admin.ch`-Filestore-Pfad, die Landingpage selbst bleibt JS-abhängig) · Bindungsakt: —
- Quelle: Tier 1 · https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/27/317_321_377/20260101/de/pdf-a/fedlex-data-admin-ch-eli-cc-27-317_321_377-20260101-de-pdf-a.pdf (in Prüfung 2026-08-13 direkt geöffnet und per pdftotext verifiziert); kanonisch: https://www.fedlex.admin.ch/eli/cc/27/317/de · Fassung(as-amended) 2026-01-01 · Zugriff 2026-08-13
- Status: in Kraft · seit 2026-01-01 — **Bestätigt**
- Sub-Ebene: nicht zutreffend (A=national)
- Relationen: wird kombiniert mit/ergänzt REG-CH-7-017 (gleiche Reform, Verjährungsfrist); konkretisiert wird durch keine identifizierte CH-Fachnorm zu gebrauchten Bauteilen im Kaufrecht (Lücke)
- Konfidenz: gesichert (Wortlaut jetzt B0-primärquellenverifiziert); abgeleitet (Reuse-Bezug, reine Projektzuordnung, da Norm selbst nicht nach Bauteilherkunft unterscheidet)

### REG-CH-7-017 · Obligationenrecht (OR) Art. 371/210 (5-jährige Verjährungsfrist Werkvertrag/Grundstückkauf)
- Titel: Obligationenrecht, Teilrevision Baumängelrecht, in Kraft seit 1. Januar 2026
- Fundstelle: Art. 371 Abs. 1–2 OR; Art. 210 Abs. 2 OR; SR 220
- A: national
- B: Primärfeld 7
- C: materialübergreifend
- D: Gesetz
- E: Betrieb/Dokumentation · E-Wirkung: durchläuft
- F1 (E3): bedingend — die Reform führt laut eingesehenem Fachaufsatz eine einseitig zwingende fünfjährige Verjährungsfrist zugunsten des Bestellers (Werkvertrag) bzw. des Grundstückkäufers ein; die Norm unterscheidet nicht nach Bauteilherkunft — für ein wiederverwendetes, in ein Werk integriertes Bauteil gilt dieselbe Frist wie für ein neues; ob eine kürzere übliche Nutzungsdauererwartung bei bewusst "gebraucht" beschafften/verbauten Bauteilen die Anwendung der Norm im Einzelfall beeinflusst, ist laut Fachaufsatz auch für andere Fallgruppen (dort: Solaranlage auf Dach) umstritten und nicht abschliessend geklärt — echte Auslegungsunsicherheit, kein Textbeleg für eine Bauteilherkunfts-Differenzierung
- F2 (E3): schweigend — keine erkennbare gezielte Reuse-Steuerungswirkung, reine Verjährungsfristreform mit allgemeinem Verbraucherschutz-/Bauherrenschutz-Ziel
- G: Dokumentenlage — inferiert (E3), Verjährungsfristen selbst begründen keinen eigenen Nachweistatbestand, wirken nur fristbegrenzend auf bestehende Gewährleistungsansprüche
- Kernaussage: **[Prüfung 2026-08-13 — KORRIGIERT, Nuance:]** Der Gesetzestext (per fedlex-PDF-A verifiziert) zeigt: Die fünfjährige Frist für Mängel eines beweglichen, in ein unbewegliches Werk integrierten Werkteils (Art. 371 Abs. 1 Satz 2) ist **nicht neu 2026** — sie geht auf die Revision vom 16. März 2012 zurück (in Kraft seit 1. Jan. 2013, Fussnote 260; identisch mit dem bereits in REG-CH-7-013 [CH-F1-3.md] korrekt referenzierten Stand). **Neu seit 1. Jan. 2026** (BG vom 20. Dez. 2024 "Baumängel", Fussnote 261) ist ausschliesslich Art. 371 Abs. 3: Diese fünfjährige Frist "kann nicht zu Lasten des Bestellers abgeändert werden" — die Frist wird damit erstmals einseitig zwingend (unabdingbar), zuvor war sie dispositiv abänderbar. Für Art. 210 Abs. 2 OR (Grundstückkauf, wortgleiche Fünfjahresfrist) konnte in der Prüfung keine spezifische 2026-Fussnote lokalisiert werden; die einseitige Zwingigkeit ergibt sich dort aus Art. 210 Abs. 4 (Mindestfristenkatalog). Die ursprüngliche Kernaussage suggerierte fälschlich, die Reform führe die **Fünfjahresfrist selbst** neu ein — tatsächlich führt sie nur deren Unabdingbarkeit neu ein.
- Wortlautbeleg (Originalsprache): **[B0, per pdftotext aus fedlex-Konsolidierung 2026-01-01 verifiziert]** "1 Die Ansprüche des Bestellers wegen Mängel des Werkes verjähren mit Ablauf von zwei Jahren nach der Abnahme des Werkes. Soweit jedoch Mängel eines beweglichen Werkes, das bestimmungsgemäss in ein unbewegliches Werk integriert worden ist, die Mangelhaftigkeit des Werkes verursacht haben, beträgt die Verjährungsfrist fünf Jahre. […] 3 Die Verjährungsfrist von fünf Jahren kann nicht zu Lasten des Bestellers abgeändert werden." (Art. 371 Abs. 1, 3 OR; Abs. 3 neu eingefügt durch Fussnote 261: "Eingefügt durch Ziff. I des BG vom 20. Dez. 2024 [Baumängel], in Kraft seit 1. Jan. 2026 [AS 2025 270; BBl 2022 2743]")
- Beleg-Quelle: **B0 (Prüfung 2026-08-13: direkter fedlex-PDF-A-Volltext per pdftotext gelesen, ersetzt die vorige B3-Einstufung)** · Zugänglichkeit: frei-primär · Bindungsakt: —
- Quelle: Tier 1 · https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/27/317_321_377/20260101/de/pdf-a/fedlex-data-admin-ch-eli-cc-27-317_321_377-20260101-de-pdf-a.pdf (in Prüfung 2026-08-13 verifiziert); kanonisch: https://www.fedlex.admin.ch/eli/cc/27/317/de · Fassung(as-amended) 2026-01-01 · Zugriff 2026-08-13
- Status: in Kraft · Grundfrist seit 2013-01-01, Unabdingbarkeit (Abs. 3) seit 2026-01-01 — **Korrigiert**
- Sub-Ebene: nicht zutreffend
- Relationen: wird kombiniert mit/ergänzt REG-CH-7-016; identischer Sachverhalt teilweise bereits in REG-CH-7-013 (CH-F1-3.md) korrekt als "seit 2013" datiert — die beiden Objekte sollten in der Synthese zusammengeführt/klar abgegrenzt werden (7-013 = Grundfrist, 7-017 = Unabdingbarkeit 2026)
- Konfidenz: gesichert (Wortlaut B0-primärquellenverifiziert, Datumsnuance korrigiert)

### REG-CH-7-018 · Obligationenrecht (OR) Art. 58 (Werkeigentümerhaftung)
- Titel: Obligationenrecht (OR), Fünfter Teil, Erster Abschnitt (Entstehung durch unerlaubte Handlung)
- Fundstelle: Art. 58 Abs. 1–2 OR; SR 220
- A: national
- B: Primärfeld 7
- C: materialübergreifend
- D: Gesetz
- E: Betrieb/Dokumentation · E-Wirkung: durchläuft
- F1 (E3): bedingend — der Werkeigentümer haftet verschuldensunabhängig (Kausalhaftung) für Schaden, der durch fehlerhafte Anlage oder Herstellung des Werks oder durch mangelhaften Unterhalt entsteht; wird ein wiederverwendetes Bauteil fehlerhaft in ein Werk eingebaut oder mangelhaft unterhalten, trifft die Haftung nach dieser Norm unverändert den Werkeigentümer, unabhängig davon, ob das schadensursächliche Bauteil neu oder wiederverwendet war — die Norm unterscheidet nicht nach Bauteilherkunft; Abs. 2 gewährt dem Werkeigentümer Rückgriffsrecht gegen den eigentlich Verantwortlichen (z. B. Lieferant eines mangelhaften Altbauteils)
- F2 (E3): schweigend — keine erkennbare spezifische Praxiswirkung für wiederverwendete Bauteile über die allgemeine Werkeigentümerhaftung hinaus; potenziell ein Unsicherheitsfaktor für Bauherren, der Versicherbarkeit von Reuse-Projekten indirekt beeinflussen könnte (reine Projekthypothese, keine Quellenaussage)
- G: Dokumentenlage — inferiert (E3)
- Kernaussage: Der Eigentümer eines Gebäudes oder eines anderen Werks hat den Schaden zu ersetzen, den dieses infolge fehlerhafter Anlage oder Herstellung oder mangelhaften Unterhalts verursacht — eine verschuldensunabhängige Kausalhaftung mit Rückgriffsrecht gegen den tatsächlich Verantwortlichen (Abs. 2).
- Wortlautbeleg (Originalsprache): **[Prüfung 2026-08-13 — KORRIGIERT, per fedlex-PDF-A/pdftotext B0-verifiziert; Wortlaut wich im Detail vom bisherigen Sekundärzitat ab]** "1 Der Eigentümer eines Gebäudes oder eines andern Werkes hat den Schaden zu ersetzen, den diese infolge von fehlerhafter Anlage oder Herstellung oder von mangelhafter Unterhaltung verursachen. 2 Vorbehalten bleibt ihm der Rückgriff auf andere, die ihm hierfür verantwortlich sind." (Art. 58 OR) — das bisher geführte Sekundärzitat ("…mangelhaften Unterhaltes verursachen", ohne "von" vor "mangelhafter Unterhaltung") war eine leicht ungenaue Wiedergabe; inhaltlich unverändert, hiermit auf den amtlichen Wortlaut korrigiert
- Beleg-Quelle: **B0 (Prüfung 2026-08-13: fedlex-PDF-A-Konsolidierung 2026-01-01 direkt per pdftotext gelesen — ersetzt die vorige B3-Einstufung)** · Zugänglichkeit: frei-primär · Bindungsakt: —
- Quelle: Tier 1 · https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/27/317_321_377/20260101/de/pdf-a/fedlex-data-admin-ch-eli-cc-27-317_321_377-20260101-de-pdf-a.pdf (in Prüfung 2026-08-13 verifiziert); kanonisch: https://www.fedlex.admin.ch/eli/cc/27/317/de · Fassung(as-amended) 2026-01-01 (Art. 58 selbst seit Grunderlass 1911 unverändert) · Zugriff 2026-08-13
- Status: in Kraft — **Bestätigt (mit Wortlautkorrektur)**
- Sub-Ebene: nicht zutreffend
- Relationen: wird kombiniert mit/ergänzt REG-CH-7-019 (PrHG) — beide Normen können bei Schaden durch ein mangelhaftes, wiederverwendetes Bauteil parallel einschlägig sein (Werkeigentümer- vs. Herstellerhaftung), ohne dass eine der beiden Normen dies für den Reuse-Fall ausdrücklich regelt
- Konfidenz: gesichert (Wortlaut jetzt B0-primärquellenverifiziert)

### REG-CH-7-019 · Produktehaftpflichtgesetz (PrHG) Art. 1, Art. 3, Art. 5 (Herstellerhaftung, Ausnahmen)
- Titel: Bundesgesetz über die Produktehaftpflicht (Produktehaftpflichtgesetz, PrHG) vom 18. Juni 1993
- Fundstelle: Art. 1 (Grundsatz), Art. 2 (Herstellerin), Art. 3 (Produkt), Art. 5 (Ausnahmen von der Haftung); SR 221.112.944
- A: national
- B: Primärfeld 7
- C: materialübergreifend
- D: Gesetz
- E: Inverkehrbringen · E-Wirkung: durchläuft
- F1 (E3): widersprüchlich — als Bezugsgegenstand-Doppelnatur zu behandeln: **Bezugsgegenstand "Neuprodukt aus laufender Produktion"**: F1 = bedingend, das PrHG erfasst laut Art. 3 Abs. 1 Bst. a jede bewegliche Sache, auch als Teil einer unbeweglichen Sache — ein neu hergestelltes Bauprodukt fällt unproblematisch darunter. **Bezugsgegenstand "aus einem Bestandsgebäude ausgebautes und weiterveräussertes Bauteil"**: F1 = schweigend, das Gesetz enthält keine Sonderregel dafür, ob die Weiterveräusserung eines gebrauchten, aus einem Rückbau stammenden Bauteils durch einen Händler/eine Bauteilbörse ein eigenständiges "Inverkehrbringen" durch eine neue "Herstellerin" im Sinne von Art. 2 auslöst, oder ob die Haftung beim ursprünglichen Hersteller (sofern feststellbar) verbleibt; Art. 5 Abs. 1 Bst. a (kein Inverkehrbringen durch die in Anspruch Genommene) und Bst. c (keine gewerbliche Herstellung/kein gewerblicher Vertrieb) sind die einzigen textnahen Anknüpfungspunkte, ohne dass der Gesetzestext den Reuse-Fall ausdrücklich adressiert — echter Auslegungs-Grenzfall, kein Textbeleg für eine Lösung
- F2 (E3): schweigend — keine in dieser Session identifizierte CH-Rechtsprechung oder Fachliteratur zur Anwendung des PrHG auf Bauteilbörsen/Wiederverwendungshändler; offene Frage für die Praxis
- G: Anwendbarkeitsnorm ohne Nachweistatbestand (Art. 3, Produktbegriff) — explizit (E1) für den Scope; Statusfeststellung/Anwendbarkeitsprüfung (ob ein konkretes wiederverwendetes Bauteil überhaupt unter den Herstellerbegriff einer reuse-vertreibenden Stelle fällt) — inferiert (E3)
- Kernaussage: Die Herstellerin haftet nach Art. 1 für Personen- und bestimmte Sachschäden, die durch ein fehlerhaftes Produkt verursacht werden. Als Produkt gilt jede bewegliche Sache, auch als Teil einer unbeweglichen Sache (Art. 3). Die Haftung entfällt u. a., wenn die in Anspruch genommene Person das Produkt nicht in Verkehr gebracht hat (Art. 5 Abs. 1 Bst. a) oder es nicht für gewerbliche Zwecke hergestellt/vertrieben hat (Bst. c). Das Gesetz unterscheidet nicht ausdrücklich zwischen neuen und aus einem Rückbau stammenden, wiederverwendeten Bauteilen.
- Wortlautbeleg (Originalsprache): "Die herstellende Person (Herstellerin) haftet für den Schaden, wenn ein fehlerhaftes Produkt dazu führt, dass: a. eine Person getötet oder verletzt wird; b. eine Sache beschädigt oder zerstört wird…" (Art. 1 Abs. 1) / "Als Produkte im Sinne dieses Gesetzes gelten: a. jede bewegliche Sache, auch wenn sie einen Teil einer anderen beweglichen Sache oder einer unbeweglichen Sache bildet, und b. Elektrizität." (Art. 3 Abs. 1) / "Die Herstellerin haftet nicht, wenn sie beweist, dass: a. sie das Produkt nicht in Verkehr gebracht hat; … c. sie das Produkt weder für den Verkauf oder eine andere Form des Vertriebs mit wirtschaftlichem Zweck hergestellt noch im Rahmen ihrer beruflichen Tätigkeit hergestellt oder vertrieben hat…" (Art. 5 Abs. 1)
- Beleg-Quelle: B0 (Volltext-PDF per `pdftotext` gelesen, Art. 1–8 vollständig erfasst) · Zugänglichkeit: frei-primär · Bindungsakt: —
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/1993/3122_3122_3122/de (kanonisch); PDF gelesen: https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/1993/3122_3122_3122/20100701/de/pdf-a/fedlex-data-admin-ch-eli-cc-1993-3122_3122_3122-20100701-de-pdf-a-1.pdf · Fassung(as-amended) 2010-07-01 (dies ist die zuletzt in dieser Session direkt geöffnete Fassung; ein neuerer Änderungsstand nach 2010-07-01 wurde nicht gezielt gesucht — Lücke) · Zugriff 2026-08-13
- Status: in Kraft · seit 1994-01-01
- Sub-Ebene: nicht zutreffend
- Relationen: wird kombiniert mit/ergänzt REG-CH-7-018 (OR Art. 58); determiniert Anwendbarkeit von — kein anderes Objekt dieser Karte hängt von der PrHG-Grundnorm ab
- Konfidenz: gesichert (Wortlaut Art. 1/3/5); unklar (Anwendung auf den Reuse-Fall selbst — echte, textlich unentschiedene Auslegungsfrage, nicht durch weitere Recherche in dieser Session auflösbar, da das Gesetz schlicht schweigt) — **Fassungsstand-Lücke: neuerer Änderungsstand nach 2010-07-01 nicht gezielt geprüft, für Synthesestufe zu verifizieren**

---

## Zusammenfassung Lücken (ehrlich, nicht in Objekte verpackt)

1. **Feld 5b (Förderung) ist die schwächste Feldabdeckung dieser Karte** — nur ein Objekt, mit offener Kernfrage (Reuse-Förderfähigkeit im Gebäudeprogramm/HFM). Grund: Das WebSearch-Kontingent der Session war erschöpft, bevor eine gezielte Suche nach kantonalen Kreislaufwirtschafts-/Bauteilbörsen-Förderprogrammen (z. B. Basel-Stadt, Zürich, Genf) durchgeführt werden konnte.
2. **VKF-Brandschutzvorschriften: Sonderfall der Bindungsebene (A-Achse)** nicht abschliessend gegen die Taxonomie geklärt — als Freitext markiert, an W4 zur Entscheidung, ob die Projektkonvention hierfür einen expliziten Passus braucht (Restlücke „CH-MRA" aus Taxonomie-Freeze Abschnitt 13).
3. **SIA-Normen (430, 269) nur über Sekundärquellen belegt** (B3), da die Normtexte selbst kostenpflichtig sind und in dieser Session nicht beschafft wurden. Bindungsketten-Regel damit nur teilweise erfüllt — Beschaffung (SIA-Shop-Kauf oder Bibliothekszugang) für Synthesestufe empfohlen.
4. **ChemRRV (Asbest) komplett offen** (B4) — keine Volltexteinsicht in dieser Session.
5. **OR-Wortlaut (Art. 58, teilweise Art. 371/210) nur über Sekundärquellen**, da fedlex.admin.ch ohne JavaScript keinen Volltext liefert und in dieser Session kein alternativer PDF-Direktzugriff für die aktuelle SR-220-Konsolidierung gefunden wurde (anders als bei EnG, BauAV, BöB, PrHG, wo `fedlex.data.admin.ch`-PDF-A-Direktlinks funktionierten). Für die Synthesestufe: gezielt nach `fedlex.data.admin.ch/filestore/.../eli/cc/27/317/.../pdf-a/...pdf`-Direktlinks für OR suchen.
6. **DIN-SPEC-91484/91525-Äquivalent für CH (Pre-Demolition-Audit-Norm)** nicht gesucht/nicht gefunden — potenzielle Lücke in Feld 6, die für die Prozessphase Bestandserkundung relevant wäre.
7. **Kein CH-Objekt zu Produkthaftungs-RL 2024/2853** (EU-Reform) — da CH nicht EU/EEA ist, wäre die relevante Frage, ob/wie das PrHG autonom nachvollzogen wird; nicht recherchiert (Lücke, an W3b/Querschnitts-Agenten oder W4 zu melden, da das Thema jurisdiktionsübergreifend ist).
