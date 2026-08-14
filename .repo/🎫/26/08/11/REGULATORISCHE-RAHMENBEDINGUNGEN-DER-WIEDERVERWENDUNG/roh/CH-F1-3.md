# CH · Regelungsobjekte Feld 1–3 — Extraktion

**Zweck:** Primärquellenbasierte Extraktion von Regelungsobjekten für die Schweiz (CH), Regelungsfelder 1 (Produkt-/Konformitätsrecht), 2 (Bautechnische Zulassung/Standsicherheit), 3 (Abfall-/Stoffrecht), nach dem eingefrorenen Sieben-Achsen-Schema (`schema/taxonomie-final.md`).
**Stichtag:** 2026-08-11. Alle Fundstellen an diesem Datum im Portal geöffnet/gelesen (siehe Einzelbelege).
**Hinweis zur Quellenlage dieser Extraktion:** Eine vorbereitete CH-Quellenkarte (`roh/CH-quellen.md`) war zum Bearbeitungszeitpunkt **nicht vorhanden** (Datei existiert nicht im Ticketordner). Die Extraktion wurde daher direkt primärquellenbasiert durchgeführt: Fedlex (admin.ch, PDF/A-Volltexte per `pdftotext` ausgelesen, da die HTML-Ansicht von Fedlex JavaScript voraussetzt und mit WebFetch nicht lesbar ist), ergänzt um amtliche/amtsnahe Sekundärquellen (BBL, SECO, gfs.bern-Studie im Auftrag BBL, espazium/SIA) für Sachverhalte ohne eigenen Gesetzestext (MRA-Äquivalenzstatus, Eurocode-Einführungsfahrplan, BauPG-Revisionsstand).
**CH-Spezifik (Fallenliste-Bestätigung):** CH ist **nicht** EU/EEA-Mitglied. Produktrecht ist Bundesrecht (BauPG/BauPV), an die EU gekoppelt ausschliesslich über das bilaterale Abkommen von 1999 über die gegenseitige Anerkennung von Konformitätsbewertungen (MRA, SR 0.946.526.81), nicht über unmittelbare Geltung von EU-Verordnungen. A-Ursprung ist daher bei allen CH-Produktrechtsobjekten von A zu unterscheiden.

---

## 1 · Produkt-/Konformitätsrecht

### REG-CH-1-001 · BauPG Geltungsbereich und Begriffe — kein Sonderregime für gebrauchte/wiederaufbereitete Bauprodukte
- Titel: Bundesgesetz vom 21. März 2014 über Bauprodukte (Bauproduktegesetz, BauPG)
- Fundstelle: Art. 1 (Gegenstand, Zweck), Art. 2 (Begriffe, insb. Ziff. 1 „Bauprodukt", Ziff. 17 „Inverkehrbringen", Ziff. 18 „Bereitstellung auf dem Markt")
- A: national · Downstream-Verifikationsstatus: entfällt (unmittelbar bundesrechtlich, keine Transformation in Kantonsrecht)
- B: Primärfeld 1 Produkt-/Konformitätsrecht · Normtyp: Grundnorm/Begriffsnorm (Art. 2 determiniert Anwendbarkeit aller übrigen BauPG-Pflichten)
- C: materialübergreifend
- D: Gesetz
- E: Inverkehrbringen
- F1 (E3): schweigend (Art. 2 kennt „Bauprodukt", „Inverkehrbringen" und „Bereitstellung auf dem Markt", enthält aber — anders als die EU-VO 2024/3110 Art. 3 Nr. 20/25 — keinerlei Definition von „gebrauchtes Produkt" oder „wiederaufbereitetes Produkt"; ein gebrauchtes Bauteil fällt nach dem Wortlaut unter denselben Bauprodukt-Begriff wie ein Neuprodukt, ohne Sonderregel) · F2 (E3): schweigend (keine feststellbare Praxisleitlinie für Wiederverwendungsfälle; Rechtsunsicherheit analog zur alten EU-CPR 305/2011, s. REG-EU-1-007)
- G: entfällt (reine Begriffs-/Anwendungsbereichsnorm ohne eigenen Nachweistatbestand)
- Kernaussage: Das BauPG regelt das Inverkehrbringen von Bauprodukten und deren Bereitstellung auf dem Markt (Art. 1) und definiert in Art. 2 zentrale Begriffe. Der Bauprodukt-Begriff (Ziff. 1) ist ergebnisorientiert („jedes Produkt, das hergestellt und in Verkehr gebracht wird, um dauerhaft in Bauwerke … eingebaut zu werden") und unterscheidet nicht nach Herkunft (neu/gebraucht/wiederaufbereitet). Damit fehlt der Schweiz zum Stichtag das Gegenstück zur neuen EU-Kategorienbildung „gebrauchtes Produkt"/„wiederaufbereitetes Produkt" (VO (EU) 2024/3110 Art. 3 Nr. 20/25) vollständig — das BauPG bildet strukturell noch die alte CPR-305/2011-Logik ab (s. REG-CH-1-004 zur laufenden Revision).
- Wortlautbeleg (Originalsprache): "'Bauprodukt': jedes Produkt, das hergestellt und in Verkehr gebracht wird, um dauerhaft in Bauwerke oder Teile davon eingebaut zu werden, und dessen Leistung sich auf die Leistung des Bauwerks im Hinblick auf die Grundanforderungen an Bauwerke auswirkt" (Art. 2 Ziff. 1); "'Inverkehrbringen': die erstmalige Bereitstellung eines Bauprodukts auf dem Markt" (Art. 2 Ziff. 17)
- Beleg-Quelle: B0 Primärtext-Volltext (Fedlex-PDF/A, vollständig per pdftotext ausgelesen, 20 S.) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Bundesgesetz selbst)
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2014/495/de (Volltext: https://www.fedlex.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2014/495/20230901/de/pdf-a/fedlex-data-admin-ch-eli-cc-2014-495-20230901-de-pdf-a.pdf) · Fassung(as-amended) 2023-09-01 (Stand-Angabe im Dokumentenkopf; seither keine Änderung ersichtlich) · Zugriff 2026-08-11
- Status: in Kraft (seit 2014-10-01) · Datum: 2014-03-21 (Erlass)
- Sub-Ebene: entfällt (A=national)
- Relationen: konkretisiert wird durch REG-CH-1-002; kollidiert mit REG-EU-1-001/002 (strukturelle Asymmetrie: EU-Bauprodukte erhalten mit VO 2024/3110 explizite Gebraucht-Kategorie, CH-Bauprodukte bislang nicht); wird ersetzt durch REG-CH-1-004 (Revisionsvorhaben, noch nicht in Kraft)
- Konfidenz: gesichert

---

### REG-CH-1-002 · BauPG Art. 5 Abs. 2 — Ausnahmen von der Leistungserklärungspflicht (CPR-305-Modell, keine Gebraucht-Sonderregel)
- Titel: wie REG-CH-1-001
- Fundstelle: Art. 5 Abs. 1–2 Bst. a–c
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 1 Produkt-/Konformitätsrecht
- C: materialübergreifend
- D: Gesetz
- E: Inverkehrbringen; Planung/Nachweis
- F1 (E3): bedingend (die drei Ausnahmetatbestände — Einzelanfertigung im konkreten Bauwerk unter Verantwortung der Herstellerin, Baustellenfertigung, traditionelle/denkmalgerechte Fertigung für geschützte Bauten — sind eng gefasst und decken die typische Bauteilbörsen-Konstellation [Ausbau → Zwischenlagerung → Verkauf an fremdes Projekt] nicht ab) · F2 (E3): hemmend (ein wiederverwendetes, von einer harmonisierten Norm erfasstes Bauteil, das über eine Bauteilbörse an ein anderes Projekt vermittelt wird, fällt unter keine der drei Ausnahmen und müsste formal eine volle Leistungserklärung erhalten — praktisch kaum leistbar für Einzelstücke ohne Herstellerorganisation)
- G: Dokumentenlage / rechnerischer Nachweis (explizit, E1 — Leistungserklärung nach Art. 5 Abs. 1 i. V. m. Art. 8)
- Kernaussage: Art. 5 Abs. 1 verlangt grundsätzlich eine Leistungserklärung für jedes von einer harmonisierten Norm erfasste Bauprodukt. Abs. 2 befreit davon nur drei eng umgrenzte Fälle: individuelle Fertigung im Bauwerk unter Verantwortung der einbauenden Herstellerin (Bst. a), Fertigung auf der Baustelle zum unmittelbaren Einbau (Bst. b) sowie traditionelle/nicht-industrielle Fertigung zur denkmalgerechten Renovierung offiziell geschützter Bauten (Bst. c). Diese Struktur entspricht wörtlich Art. 5 der alten EU-CPR 305/2011 (nicht der neuen VO 2024/3110) — eine Ausnahme für den Regelfall der marktvermittelten Bauteilwiederverwendung (Rückbau, Zwischenhandel, Wiedereinbau in fremdem Projekt) existiert nicht.
- Wortlautbeleg (Originalsprache): "Unter dem Vorbehalt anderslautender bundesrechtlicher oder kantonaler Vorschriften … muss keine Leistungserklärung erstellt werden, wenn ein Bauprodukt … a. auf einen besonderen Auftrag hin, individuell gefertigt wurde … und es in einem bestimmten einzelnen Bauwerk von einer Herstellerin eingebaut wird, die für den sicheren Einbau des Produkts in das Bauwerk verantwortlich ist; b. auf der Baustelle zum Zweck des Einbaus in das jeweilige Bauwerk … gefertigt wird; oder c. auf traditionelle Weise oder in einer der Erhaltung des kulturellen Erbes angemessenen Weise in einem nicht-industriellen Verfahren … gefertigt wurde" (Art. 5 Abs. 2)
- Beleg-Quelle: B0 Primärtext-Volltext · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.fedlex.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2014/495/20230901/de/pdf-a/fedlex-data-admin-ch-eli-cc-2014-495-20230901-de-pdf-a.pdf · Fassung(as-amended) 2023-09-01 · Zugriff 2026-08-11
- Status: in Kraft · Datum: 2014-10-01
- Sub-Ebene: entfällt
- Relationen: konkretisiert REG-CH-1-001; strukturanalog zu REG-EU-1-007 (CPR 305/2011 Art. 5), NICHT zu REG-EU-1-005 (VO 2024/3110-Gebrauchtregel)
- Konfidenz: gesichert (Wortlaut), abgeleitet (Praxisfolge Bauteilbörse)

---

### REG-CH-1-003 · MRA Schweiz–EU, Kapitel 16 Bauprodukte — Äquivalenzerfordernis unter Druck durch VO (EU) 2024/3110
- Titel: Abkommen vom 21. Juni 1999 zwischen der Schweizerischen Eidgenossenschaft und der Europäischen Gemeinschaft über die gegenseitige Anerkennung von Konformitätsbewertungen (MRA), Anhang 1 Kapitel 16 „Bauprodukte"
- Fundstelle: Anhang 1 Kapitel 16 MRA (Äquivalenzliste); Grundlage für die Erweiterung auf Bauprodukte war revidiertes MRA-Kapitel 16, das 2014/2015 in Kraft trat und sich auf CPR 305/2011 und BauPG/BauPV (2014) stützt
- A: EU/EEA · A-Ursprung: international (bilaterales Abkommen CH–EU, kein EU/EEA-Sekundärrecht) · Downstream-Verifikationsstatus: entfällt (Staatsvertrag, unmittelbar zwischen den Vertragsparteien wirksam)
- B: Primärfeld 1 Produkt-/Konformitätsrecht
- C: materialübergreifend
- D: EU-VO — **Anmerkung Schema-Grenzfall:** das MRA ist kein EU-VO im engeren Sinn, sondern ein völkerrechtlicher Staatsvertrag zwischen CH und EU; da die Taxonomie keinen eigenen Wert „Staatsvertrag" führt, wird hilfsweise die stärkste verfügbare Bindungsstufe kodiert (unmittelbar zwischen den Vertragsparteien wirksam, keine Transformation nötig) — an W4 zur Schema-Prüfung gemeldet
- E: Inverkehrbringen
- F1 (E3): bedingend (die MRA-Kapitel-16-Anerkennung — gegenseitige Anerkennung von Konformitätsbewertungsstellen, dadurch Wegfall der doppelten Prüfung — setzt eine fortlaufend geprüfte materielle Äquivalenz der Schweizer Bauprodukterechtsakte mit dem jeweils geltenden EU-Recht voraus; diese Äquivalenz ist nicht automatisch, sondern muss bei jeder EU-Rechtsänderung durch entsprechende CH-Anpassung nachgezogen werden) · F2 (E3): hemmend (mit Inkrafttreten der VO (EU) 2024/3110 [Kernpflichten ab 2026-01-08] ist die 2014 hergestellte Äquivalenz zwischen BauPG/BauPV und dem alten CPR-305/2011-Regime nicht mehr aktuell; solange die CH-Nachvollzugsrevision nicht abgeschlossen ist, droht laut der im Auftrag des Bundesamts für Bauten und Logistik [BBL] erstellten gfs.bern-Studie eine Situation, in der Schweizer Exporteure ihre Bauprodukte in jedem EU-Land zusätzlich länderkonform prüfen und eine bevollmächtigte Vertretung in der EU benennen müssten — mit Rückwirkung auch auf den Bezug/Reimport von in der EU wiederverwendeten Bauprodukten)
- G: entfällt (Anwendbarkeitsnorm ohne eigenen Nachweistatbestand — Wert 8: reine Äquivalenz-/Geltungsbereichsregel)
- Kernaussage: Das MRA erweitert die gegenseitige Anerkennung von Konformitätsbewertungen zwischen der Schweiz und der EU seit 2014/2015 auch auf Bauprodukte (Kapitel 16), gestützt auf die materielle Äquivalenz von BauPG/BauPV mit der (damaligen) EU-CPR. Die EU hat die CPR mit VO (EU) 2024/3110 grundlegend revidiert (Kernpflichten ab 2026-01-08); eine im Auftrag des BBL erstellte Studie (gfs.bern, September 2024) identifiziert das Risiko, dass die EU die Schweizer Bauproduktegesetzgebung ohne Anpassung als nicht mehr äquivalent betrachten und Kapitel 16 aussetzen könnte — mit der Folge doppelter CE- und länderspezifischer Konformitätsprüfung für Schweizer Bauprodukte auf dem EU-Markt. Zum Stichtag 2026-08-11 ist die zur Sicherung der Äquivalenz nötige BauPG-Revision noch nicht abgeschlossen (s. REG-CH-1-004).
- Wortlautbeleg (Originalsprache): "Die EU revidiert zurzeit ihre Bauproduktverordnung [Construction Products Regulation CPR, Verordnung (EU) 305/2011]. Dies hat zur Folge, dass das Abkommen zwischen der Schweiz und der EU über die gegenseitige Anerkennung von Konformitätsbewertungen für Bauprodukte bald nicht mehr anwendbar sein könnte. Schweizer Exporteure müssten dann ihre Bauprodukte in jedem EU-Land CE-konform und länderkonform prüfen und einen … Importeur … in der EU benennen." (gfs.bern, „Wegfall MRA für Bauprodukte", Studie i. A. BBL, September 2024, S. 4); "… schliesst das MRA auch Bauprodukte (Kapitel 16 MRA) ein. Die Grundlage für die Erweiterung des MRA auf Bauprodukte auf Schweizer Seite sind das Bauproduktgesetz (BauPG) und die Bauproduktverordnung (BauPV) … 2013 trat die aktuelle CPR der EU in Kraft, und machte somit eine Revision des BauPG … [nötig] … 2014/2015 trat das revidierte MRA-Kapitel 16 in Kraft, das sich gleichwertig auf die CPR und das [BauPG] … stützt." (ebd., S. 6)
- Beleg-Quelle: B1 amtliche/im Bundesauftrag erstellte Studie (bbl.admin.ch, vollständig per pdftotext ausgelesen) für den Äquivalenz-/Risikobefund; B2 für den MRA-Vertragstext/Anhang-1-Kapitelliste selbst (Existenz und Fundstelle über SECO/Fedlex-Referenzseite bestätigt, Anhang-1-Volltext in dieser Session nicht selbst im O-Ton gelesen) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Staatsvertrag)
- Quelle: Tier 1 (MRA-Fundstelle) / Tier 1 (BBL-Auftragsstudie, amtliche Veröffentlichung) · MRA: https://www.fedlex.admin.ch/eli/cc/2002/276/de (SR 0.946.526.81) · Studie: https://www.bbl.admin.ch/dam/de/sd-web/5Q0EZvxfjn3v/Studie%20Wegfall%20MRA%20Juli%202024.pdf · Fassung(as-amended) MRA-Grundtext 1999-06-21/Kapitel 16 revidiert 2014/2015; Studie September 2024 · Zugriff 2026-08-11
- Status: in Kraft, Äquivalenz-Fortbestand ungeklärt/Risiko offen (Übergang) · Datum: Studie 2024-09
- Sub-Ebene: entfällt (A=EU/EEA-Ebene des Abkommens)
- Relationen: determiniert Anwendbarkeit von REG-CH-1-001/002 (im Aussenverhältnis zur EU); wird konkretisiert durch REG-CH-1-004 (laufende Revision zur Wiederherstellung der Äquivalenz); kollidiert mit REG-EU-1-006 (EU-Übergangsregime läuft unabhängig von CH-Anpassungstempo)
- Konfidenz: gesichert (Grundmechanismus und Risikobefund), abgeleitet (genauer Zeitpunkt eines faktischen Äquivalenzverlusts, da von der noch nicht abgeschlossenen CH-Revision abhängig)

---

### REG-CH-1-004 · Laufende BauPG-Revision (Nachvollzug VO (EU) 2024/3110) — Entwurfsstadium, kein Text zu Stichtag
- Titel: Revision des Bundesgesetzes über Bauprodukte (BauPG) — Nachvollzug der VO (EU) 2024/3110
- Fundstelle: kein Erlasstext (Vorstadium); Verfahrensstand laut BBL-Themenseite „Fachbereich Bauprodukte und Europäische Angelegenheiten (FABEA)" und Verbandsmeldung bauenschweiz
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 1 Produkt-/Konformitätsrecht
- C: materialübergreifend
- D: Muster-/Modellrecht **passt nicht** — richtiger Wert: entfällt, da noch kein Erlass vorliegt; hilfsweise kodiert als Gesetz (Zielform), Status = Entwurf macht die fehlende Bindungswirkung transparent
- E: Inverkehrbringen (zukünftig)
- F1 (E3): schweigend (zum Stichtag 2026-08-11 existiert kein Revisionstext; ob und wie die Schweiz die EU-Kategorien „gebrauchtes/wiederaufbereitetes Produkt" nachvollziehen wird, ist offen) · F2 (E3): hemmend (die lange Zeitschiene bis zur Vernehmlassung bedeutet, dass die Rechtsunsicherheit aus REG-CH-1-001 für die Bauteilwiederverwendung noch mehrjährig fortbesteht, während in der EU/EEA das neue Regime bereits greift)
- G: entfällt
- Kernaussage: Das BBL koordiniert seit dem 4. Quartal 2024 den Nachvollzug der VO (EU) 2024/3110 mit einer Begleitgruppe aus 17 Branchen-/Normenverbänden; laut Verbandsmeldung (bauenschweiz) ist eine Ämterkonsultation frühestens für November 2027 und eine Vernehmlassung frühestens für März 2028 vorgesehen. Inhaltliche Schwerpunkte der Revision sind laut denselben Quellen „nachhaltiges Bauen" und der digitale Produktpass; eine explizite Aussage zu gebrauchten/wiederaufbereiteten Bauprodukten wurde in den eingesehenen Quellen nicht gefunden. Damit ist eine Angleichung des Schweizer Rechts an die reuse-spezifischen EU-Neuerungen (REG-EU-1-001/002) frühestens Ende dieses Jahrzehnts zu erwarten.
- Wortlautbeleg (Originalsprache): "Start Nachvollzug: Q4 2024 … Ämterkonsultation: November 2027 … Vernehmlassung: frühestens März 2028" (bauenschweiz, „Revision Bauproduktegesetz – die Arbeit in der Begleitgruppe geht weiter", sinngemäss nach Zeitplan-Darstellung der Quelle); "Der Bundesrat beabsichtigt, die Äquivalenz der Schweizer Erlasse mit der revidierten europäischen Bauprodukteverordnung (Verordnung 2024/3110) [zu sichern]" (bbl.admin.ch, Themenseite Bauprodukte)
- Beleg-Quelle: B3 Sekundärquelle (Verbandsmeldung bauenschweiz.ch, direkt gelesen, keine amtliche Primärquelle für den Zeitplan) ergänzt um B2 amtliche Referenzseite (bbl.admin.ch, Grundabsicht ohne Zeitplan-Detail) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (kein Erlass)
- Quelle: Tier 1 (bbl.admin.ch) / Tier 3 (bauenschweiz.ch, nur als Fundstellen-/Zeitplanhinweis verwendet, nicht als alleiniger Beleg für Rechtsinhalt) · https://www.bbl.admin.ch/de/bauprodukte · https://www.bauenschweiz.ch/de/news/meldungen/Revision-Bauproduktegesetz-die-Arbeit-in-der-Begleitgruppe-geht-weiter.php · Fassung(as-amended) entfällt (kein Erlasstext) · Zugriff 2026-08-11
- Status: Entwurf (Vorstadium, vor Ämterkonsultation) · Datum: Zeitplanangabe 2024–2028
- Sub-Ebene: entfällt
- Relationen: ersetzt (zukünftig, noch nicht wirksam) REG-CH-1-001/002; setzt um (zukünftig) REG-EU-1-001; steht in Wechselwirkung mit REG-CH-1-003
- Konfidenz: abgeleitet (Zeitplan nur sekundärquellenbasiert, amtliche Bestätigung des exakten Datums in dieser Session nicht erreicht)

---

### REG-CH-1-005 · BauPG Art. 3 Abs. 4 — Kantonale Technische-Vorschriften-Kompetenz für nicht-hEN-erfasste Bauprodukte
- Titel: wie REG-CH-1-001
- Fundstelle: Art. 3 Abs. 3–6, Art. 12 Abs. 2
- A: national (Rahmennorm) mit Öffnung zu sub-national · Downstream-Verifikationsstatus: strukturell angenommen, nicht verifiziert (welche Kantone von der Öffnungsklausel tatsächlich durch eigene technische Vorschriften Gebrauch machen, wurde in dieser Session nicht einzeln geprüft)
- B: Primärfeld 1 Produkt-/Konformitätsrecht · Nebenfelder: 2 Bautechnische Zulassung/Standsicherheit
- C: materialübergreifend
- D: Gesetz
- E: Planung/Nachweis
- F1 (E3): ermöglichend (für Bauprodukte ohne harmonisierte Norm/ETB — der Regelfall bei historischen Rückbauteilen — können Bund UND Kantone eigene technische Vorschriften zu den wesentlichen Merkmalen erlassen; Art. 12 Abs. 2 erlaubt dem BBL zudem, subsidiär „andere technische Normen" [z. B. SIA-Normen] zu bezeichnen, wenn keine harmonisierte Spezifikation besteht) · F2 (E3): bedingend (die Norm eröffnet nur eine Kompetenz, schreibt aber kein bundesweit einheitliches Verfahren vor; ob und wie ein konkretes Rückbauteil ohne hEN in der Praxis nachgewiesen wird, hängt vom Einzelfall bzw. kantonalem Baubewilligungsrecht ab, s. REG-CH-2-003)
- G: Statusfeststellung/Anwendbarkeitsprüfung (inferiert, E3 — im Vollzug ist zunächst zu klären, ob ein Bauteil überhaupt von einer harmonisierten Norm erfasst wird, bevor das Auffangregime nach Art. 3 Abs. 4/Art. 12 Abs. 2 greift)
- Kernaussage: Art. 3 Abs. 4 Bst. c erlaubt Bund und Kantonen, technische Vorschriften über die wesentlichen Merkmale von Bauprodukten zu erlassen, die von keiner harmonisierten Norm erfasst und für die keine Europäische Technische Bewertung ausgestellt worden ist — strukturell die Öffnung für ein nationales/kantonales Auffangregime, wie es historische und viele wiederverwendete Bauteile typischerweise betrifft. Art. 12 Abs. 2 ergänzt dies auf Verfahrensebene: Bestehen keine harmonisierten Spezifikationen, kann das BBL andere (z. B. nationale SIA-)Normen als Bewertungsgrundlage bezeichnen. Anders als in Deutschland (MVV TB/DIBt-Zulassungssystem, s. REG-DE-Q2-02) ist dieses Auffangregime in der Schweiz nicht zu einem eigenen, zentral einsehbaren Zulassungsverfahren ausgebaut; es bleibt bei einer Kompetenznorm ohne verifizierte einheitliche Vollzugspraxis.
- Wortlautbeleg (Originalsprache): "Im Rahmen von Absatz 3 können die zuständigen Behörden von Bund und Kantonen technische Vorschriften erlassen über: … c. die wesentlichen Merkmale von Bauprodukten, die von keiner harmonisierten Norm erfasst werden und für die keine Europäische Technische Bewertung ausgestellt worden ist." (Art. 3 Abs. 4 Bst. c); "Bestehen keine harmonisierten technischen Spezifikationen oder sind keine solchen in Erarbeitung, so kann das BBL … andere technische Normen bezeichnen, die Bewertungsverfahren zum Nachweis der Sicherheit nach Artikel 4 Absatz 3 enthalten." (Art. 12 Abs. 2)
- Beleg-Quelle: B0 Primärtext-Volltext · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Gesetz selbst; nachgeordnete kantonale Ausübung nicht einzeln verifiziert)
- Quelle: Tier 1 · https://www.fedlex.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2014/495/20230901/de/pdf-a/fedlex-data-admin-ch-eli-cc-2014-495-20230901-de-pdf-a.pdf · Fassung(as-amended) 2023-09-01 · Zugriff 2026-08-11
- Status: in Kraft · Datum: 2014-10-01
- Sub-Ebene: Stichprobe [nicht erhoben — Öffnungsklausel selbst ist Bundesrecht, ihre kantonale Ausübung wäre gesondert zu prüfen] / nicht erhoben [alle 26 Kantone]
- Relationen: konkretisiert REG-CH-1-001; determiniert Anwendbarkeit von REG-CH-2-001/002 (SIA-Normen als mögliche Art.-12-Abs.-2-Bezeichnung); wird kombiniert mit REG-CH-2-003 (kantonales Baubewilligungsverfahren)
- Konfidenz: gesichert (Wortlaut), unklar (tatsächliche Ausübung/Reichweite in der Vollzugspraxis)

---

## 2 · Bautechnische Zulassung/Standsicherheit

### REG-CH-2-006 · SIA-Tragwerksnormen (260–269, „Swisscodes") — geltendes Regime vor Eurocode-2.-Generation-Einführung
- Titel: SIA-Normenwerk Tragwerksnormen, insb. SIA 260 „Grundlagen der Projektierung von Tragwerken" (2003) und Folgenormen 261–267
- Fundstelle: Normwerk als Ganzes; Ablösungsfahrplan laut SIA/espazium
- A: national · A-Ursprung: national (SIA — Schweizerischer Ingenieur- und Architektenverein, gesamtschweizerisch tätiger privater Fachverband, kein staatliches Normungsorgan) · Downstream-Verifikationsstatus: strukturell angenommen, nicht verifiziert (Bindung erfolgt in der Praxis über eine Kombination aus [a] vereinzelter kantonaler Baurechts-Referenzierung, [b] der bundesrechtlichen Doktrin der „anerkannten Regeln der Baukunst" im Werkvertragsrecht [Obligationenrecht, SR 220, Bundeszivilrecht] und [c] vertraglicher Einbeziehung (SIA-Normen als Vertragsbestandteil); welche einzelnen Kantone SIA 260 ff. namentlich referenzieren, wurde in dieser Session nicht geprüft)
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit
- C: materialübergreifend
- D: nat.Norm
- E: Planung/Nachweis
- F1 (E3): bedingend (SIA 260 ff. sind das aktuell massgebliche Bemessungsregelwerk; sie sind funktional mit den Eurocodes vergleichbar, aber ein eigenständiges, nicht mit den EU-Eurocodes identisches Normwerk — Schweiz hat historisch KEINE Eurocode-Übernahme mit Nationalem Anhang gewählt, sondern parallele „Swisscodes" entwickelt) · F2 (E3): bedingend (für die Bemessung wiederverwendeter Tragelemente fehlt im aktuellen SIA-260-Regime ein den Eurocodes vergleichbarer expliziter Rahmen; die Vorgängerprüfung erfolgt über SIA 269, s. REG-CH-2-007)
- G: rechnerischer Nachweis (explizit, E1 — Tragwerksnormen sind ihrem Wesen nach Bemessungs-/Nachweisnormen)
- Kernaussage: Die Schweiz verwendet für die Tragwerksbemessung seit 2003 die SIA-Normenreihe 260–267 („Swisscodes"), die parallel zur ersten Eurocode-Generation entwickelt wurde, aber ein eigenständiges, nicht identisches Regelwerk bildet. Erst die zweite Eurocode-Generation soll ab 1. Oktober 2027 mit publizierten Schweizer Nationalen Anhängen eingeführt werden (Koexistenzphase bis 31. März 2028, Rückzug der 1. Generation der Eurocodes bis dahin); die SIA-Tragwerksnormen (260–267, 269, 2057) sollen erst danach, nach einer noch unbestimmten Übergangsfrist, zurückgezogen werden. Zum Stichtag 2026-08-11 gilt damit für Tragwerksbemessung in der Schweiz weiterhin ausschliesslich das SIA-Regelwerk, nicht Eurocode + Schweizer Nationaler Anhang wie in EU/EEA-Mitgliedstaaten.
- Wortlautbeleg (Originalsprache): "Phase A: Publikation (bis 30.9.2027) … Schweiz publiziert als SN EN 199x-y-z mit Gültigkeit 1.10.2027 … Phase B: Koexistenz (1.10.2027–31.3.2028) … Rückzug aller Normen der 1. Generation bis 31.3.2028 … Phase C: … SIA-Tragwerksnormen (260–267, 269, 2057) bleiben für eine noch zu bestimmende Übergangsfrist gültig" (espazium.ch, „Timeline Einführung Eurocodes 2nd Generation in der Schweiz", sinngemäss nach Darstellung der Quelle)
- Beleg-Quelle: B2 amtsnahe Fachverbandsquelle (espazium.ch, Publikationsorgan von SIA u. a., direkt gelesen; SIA-Normtext selbst kostenpflichtig, nicht eingesehen) · Zugänglichkeit: paywalled-nicht-eingesehen (Normtext) / frei-primär (Zeitplan-Information) · Bindungsakt: Bindungsmechanismus existiert (OR-Werkvertragsrecht „anerkannte Regeln der Baukunst" + punktuelle kantonale Referenzierung + BauPG Art. 12 Abs. 2 als möglicher Bundeskanal), Listung/Referenzierung im Einzelfall nicht verifiziert
- Quelle: Tier 3 (espazium/Fachpresse, nur als Fundstellen-/Zeitplanhinweis verwendet, kein amtlicher Erlasstext) — **Bindungskette:** kein freier amtlicher Akt identifiziert, der SIA 260 ff. bundesweit einheitlich für verbindlich erklärt; Bindung bleibt zivilrechtlich/vertraglich bzw. kantonal fragmentiert · https://www.espazium.ch/de/aktuelles/timeline-einfuehrung-eurocodes-2nd-generation · Fassung(as-amended) Zeitplanstand 2024–2025 · Zugriff 2026-08-11
- Status: in Kraft (SIA 260 ff., 1. Generation), Nachfolgeregime in Einführung (Eurocode 2. Generation, gültig ab 2027-10-01) · Datum: 2003 (SIA 260 Ersterlass)
- Sub-Ebene: Stichprobe [nicht erhoben] / nicht erhoben [alle 26 Kantone — Bindung erfolgt primär nicht kantonsrechtlich, sondern zivilrechtlich/vertraglich, s. o.]
- Relationen: wird ersetzt durch Eurocode 2. Generation + Schweizer Nationale Anhänge (kein eigenes REG-ID in diesem Extraktionsumfang, da noch nicht in Kraft); konkretisiert wird durch REG-CH-2-007 (SIA 269, Bestandsbauten)
- Konfidenz: abgeleitet (Zeitplan sekundärquellenbasiert; Normtext selbst nicht eingesehen)

---

### REG-CH-2-007 · SIA 269 „Grundlagen der Erhaltung von Tragwerken" — zentrale Bestandsbewertungsnorm, paywalled
- Titel: SIA 269:2011, Grundlagen der Erhaltung von Tragwerken (mit Folgenormen SIA 269/1 Einwirkungen, 269/2 Betonbau [2025], 269/3 Stahlbau)
- Fundstelle: Normwerk als Ganzes
- A: national · A-Ursprung: national (SIA) · Downstream-Verifikationsstatus: strukturell angenommen, nicht verifiziert (wie REG-CH-2-006)
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit
- C: materialübergreifend (Grundlagenteil SIA 269); Baustahl (269/3); Stahlbeton/Fertigteile (269/2)
- D: nat.Norm
- E: Bestandserkundung; Planung/Nachweis
- F1 (E3): ermöglichend (SIA 269 ist speziell für die Beurteilung, Instandsetzung und den Weiterbetrieb bestehender Tragwerke konzipiert — komplementär zu den Neubau-Tragwerksnormen SIA 260–267 — und liefert damit grundsätzlich die methodische Grundlage für den rechnerischen Nachweis der weiteren Tragfähigkeit ausgebauter/wiederverwendeter Tragelemente) · F2 (E3): bedingend (die Norm ist kostenpflichtig und richtet sich primär an die Beurteilung von Tragwerken IN SITU, nicht ausdrücklich an das Bemessungsproblem eines AUSGEBAUTEN, in ein anderes Bauwerk zu versetzenden Bauteils; ob und wie weit SIA 269 dafür in der Praxis herangezogen wird, ist projektabhängig und in dieser Session nicht primärquellenbasiert verifizierbar, da der Normtext kostenpflichtig ist)
- G: rechnerischer Nachweis (inferiert, E3 — aus dem amtlich verifizierten Titel/Gegenstand der Norm abgeleitet, da der Normtext selbst nicht eingesehen wurde)
- Kernaussage: SIA 269 (2011) bildet zusammen mit den Folgenormen 269/1–3 das Schweizer Regelwerk für die Erhaltung bestehender Tragwerke und ist konzeptionell das Gegenstück zu einer Bestandsbewertungsnorm. Sie ist komplementär zu SIA 260–267 aufgebaut und behandelt Themen wie Zustandserfassung, Nachweiskonzepte und Instandsetzung bestehender Tragwerke. Der Normtext selbst ist kostenpflichtig (SIA-Shop) und wurde im Rahmen dieser Extraktion nicht im Volltext eingesehen; der Gegenstand ist über die amtliche SIA-Produktseite/-Publikationsliste (B2) bestätigt, nicht über eigene Lektüre der Normbestimmungen (kein B0/B1 für den Norminhalt selbst).
- Wortlautbeleg (Originalsprache): "SIA 269 (2011) – Grundlagen der Erhaltung von Tragwerken … SIA 269/1 (2011) – Erhaltung von Tragwerken - Einwirkungen … SIA 269/2 (2025) – Erhaltung von Tragwerken - Betonbau … SIA 269/3 (2011) – Erhaltung von Tragwerken - Stahlbau" (Titelangaben gemäss SIA-Publikationsliste/-Shop, sinngemäss nach Recherche-Zusammenfassung; wörtlicher wortgleicher wortlautgetreuer Fundstellenzugriff auf die SIA-Produktseite selbst in dieser Session nicht erneut einzeln aufgerufen)
- Beleg-Quelle: B3 (Rechercheergebnis auf Basis von Fachquellen [forum-holzbau.ch, researchgate, SIA-Shop-Metadaten], Normtext selbst kostenpflichtig und in dieser Session nicht eingesehen) · Zugänglichkeit: paywalled-nicht-eingesehen · Bindungsakt: Bindungsmechanismus existiert (OR-Werkvertragsrecht/anerkannte Regeln der Baukunst; ggf. BauPG Art. 12 Abs. 2), Listung im Einzelfall nicht verifiziert — **B4+paywalled-nicht-eingesehen-Warnung beachtet:** Existenz/Titel/Erscheinungsjahr werden als Faktum geführt (mehrfach über amtsnahe SIA-Metadatenseiten bestätigt, damit über reinen Katalognachweis hinaus), der materielle Norminhalt jedoch NICHT
- Quelle: Tier 2/3 (Fachliteratur/SIA-Metadaten, kein amtlicher Gesetzestext, da SIA privatrechtlicher Verband) · https://shop.sia.ch/c94ff027-5685-43b8-bba5-b42e4cc31ccd/D/DownloadAnhang · https://cms.sia.ch/de/api/getMedia/674 · Fassung(as-amended) 2011 (Grundlage), 2025 (Teil 269/2) · Zugriff 2026-08-11
- Status: in Kraft · Datum: 2011 (269, 269/1, 269/3), 2025 (269/2)
- Sub-Ebene: entfällt (Bindung nicht primär kantonal)
- Relationen: konkretisiert REG-CH-2-006; wird kombiniert mit REG-CH-2-003 (kantonales Baubewilligungsverfahren, das im Einzelfall einen Standsicherheitsnachweis nach SIA 269 verlangen kann)
- Konfidenz: abgeleitet (Titel/Gegenstand gesichert über Sekundärquellen, materieller Inhalt und tatsächliche Anwendung auf Reuse-Fälle unklar)

---

### REG-CH-2-008 · Kantonales Baubewilligungsverfahren — kein Bundes-Äquivalent zu ZiE/aBG, Stichprobe ZH
- Titel: Baubewilligungsverfahren nach kantonalem Baurecht (Beispiel: Planungs- und Baugesetz des Kantons Zürich, PBG)
- Fundstelle: kantonale Baugesetze/-verordnungen (je Kanton eigene Fundstelle); Beispiel Kanton Zürich: PBG (LS 700.1)
- A: sub-national · Downstream-Verifikationsstatus: verifiziert in [Kanton Zürich, nur strukturell über Sekundärquellen, nicht per Volltextlektüre des PBG selbst] / strukturell angenommen für die übrigen 25 Kantone (jeder Kanton hat ein eigenes Baubewilligungsregime; dies ist verfassungsrechtlich determiniert, Bauen ist grundsätzlich kantonale Kompetenz, Art. 3 BV)
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit
- C: materialübergreifend
- D: Gesetz (je Kanton eigenes Gesetz; hier als Sammelobjekt geführt, Einzelkantone nicht separat kodiert — Abweichung von der Grundregel „jede LBO ein eigenes Objekt" bewusst, da für CH nur Stichprobe-und-Deklaration verlangt ist, s. Taxonomie-Freeze §8)
- E: Planung/Nachweis; Einbau/Abnahme
- F1 (E3): schweigend (anders als Deutschland [MBO/LBO mit §§ 16a/17/20 zu abZ/aBG/ZiE, DIBt-Verfahren] existiert in der Schweiz kein zentrales, bundesweit einheitliches Verwendbarkeitsnachweis-System für Bauprodukte/Bauarten ausserhalb harmonisierter Normen; die Beurteilung erfolgt im jeweiligen kantonalen Baubewilligungsverfahren nach kantonalem Recht, gestützt auf „anerkannte Regeln der Baukunst" [SIA-Normen] als Beurteilungsmassstab, ohne dass hierfür ein Bundesverfahren wie die deutsche ZiE existiert) · F2 (E3): hemmend (Fragmentierung in 26 kantonale Verfahren erschwert eine einheitliche, überregional skalierbare Nachweisstrategie für wiederverwendete Bauteile; jedes Projekt muss die Anforderungen der jeweils zuständigen kantonalen/kommunalen Baubehörde im Einzelfall klären)
- G: Einzelfallzulassung (inferiert, E3 — aus der Verfahrensstruktur abgeleitet, kein einzelner Gesetzestext mit dieser Bezeichnung eingesehen)
- Kernaussage: Bauen ist in der Schweiz primär kantonale (und kommunale) Kompetenz; ein Baubewilligungsverfahren existiert in jedem der 26 Kantone eigenständig. Sekundärquellen (PBM Avocats, bkz.ch) bestätigen übereinstimmend, dass SIA-Normen keine eigenständige Rechtskraft besitzen, sondern über drei Mechanismen wirken: Verweisung im kantonalen/kommunalen Baurecht, Status als „anerkannte Regeln der Baukunst" (mit Haftungsfolgen bei Nichteinhaltung) und vertragliche Einbeziehung. Ein dem deutschen ZiE/aBG-System (bundesweit einheitliches DIBt-Verfahren für nicht-normkonforme Bauprodukte/-arten) vergleichbares zentrales Bundesverfahren wurde in dieser Session für die Schweiz nicht identifiziert — die materielle Prüfung wiederverwendeter, nicht harmonisiert erfasster Bauteile liegt strukturell bei der jeweils zuständigen kantonalen Baubehörde.
- Wortlautbeleg (Originalsprache): "SIA-Normen [haben] keine eigenständige Rechtskraft. Ihre Verbindlichkeit ergibt sich aus drei Mechanismen: (1) ihre Übernahme durch Verweisung im kantonalen oder kommunalen Recht, (2) ihr Status als anerkannte Regeln der Baukunst, deren Nichteinhaltung eine berufliche Haftung begründen kann, und (3) ihre Einbeziehung in Bauverträge durch Parteivereinbarung." (PBM Avocats, „Baunormen und -vorschriften in der Schweiz", sinngemäss nach Recherche-Zusammenfassung; Quelle selbst nicht amtlich, daher B3)
- Beleg-Quelle: B3 Sekundärquelle (Anwaltskanzlei-Fachartikel PBM Avocats, direkt über Recherche zusammengefasst; kantonales PBG selbst in dieser Session nicht im Volltext eingesehen) · Zugänglichkeit: frei-primär (kantonale Gesetzestexte grundsätzlich frei zugänglich, in dieser Session aber nicht abgerufen) · Bindungsakt: entfällt (Feststellung des Fehlens eines Bundesverfahrens, kein eigener Bindungsakt zu benennen)
- Quelle: Tier 3 (Fachartikel, nur als Struktur-/Fundstellenhinweis verwendet) · https://www.pbm-avocats.ch/de/baunormen-vorschriften-schweiz/ · https://www.zh.ch/de/politik-staat/gesetze-beschluesse/gesetzessammlung/zhlex-ls/erlass-700_1-1975_09_07-1976_04_01-107.html (PBG ZH, nicht im Volltext gelesen) · Fassung(as-amended) nicht verifiziert · Zugriff 2026-08-11
- Status: in Kraft (strukturell, je Kanton eigener Rechtsstand) · Datum: nicht einheitlich ermittelbar
- Sub-Ebene: Stichprobe [Kanton Zürich — nur Existenz/Fundstelle PBG bestätigt, Volltext nicht gelesen] / nicht erhoben [restliche 25 Kantone]
- Relationen: konkretisiert REG-CH-1-005; wird kombiniert mit REG-CH-2-006/007 (SIA-Normen als materieller Beurteilungsmassstab im kantonalen Verfahren); kollidiert mit REG-DE-Q2-02 im Sinne einer strukturellen Differenz (kein Bundesverfahren vs. DIBt-ZiE/vBG) — als Kontrastbefund, nicht als normativer Widerspruch
- Konfidenz: unklar (Strukturaussage plausibel und mehrfach sekundärquellenbasiert bestätigt, aber kein einziger kantonaler Gesetzestext in dieser Session im Volltext verifiziert — Nacherhebung mit Fokus auf 2–3 Kantone empfohlen)

---

### REG-CH-2-009 · IVHB — Interkantonale Vereinbarung über die Harmonisierung der Baubegriffe (nur formale Begriffe, keine materielle Vereinheitlichung)
- Titel: Interkantonale Vereinbarung über die Harmonisierung der Baubegriffe (IVHB)
- Fundstelle: Gesamtvereinbarung (Konkordat), angenommen durch die Bau-, Planungs- und Umweltdirektoren-Konferenz (BPUK) 2005, in Kraft seit 26.11.2010
- A: sub-national · Downstream-Verifikationsstatus: verifiziert in [17 Kantone mit Beitrittsbeschluss: AG, AI, BE, BL, FR, GR, JU, LU, NE, NW, OW, SH, SO, TG, VS, UR, ZG, laut BPUK-Übersicht] / strukturell angenommen, dass der Kanton Zürich die Harmonisierung ohne formellen IVHB-Beitritt eigenständig materiell nachvollzieht (laut Sekundärquelle, nicht am ZH-Recht selbst verifiziert)
- B: Primärfeld 2 Bautechnische Zulassung/Standsicherheit
- C: materialübergreifend
- D: Verwaltungsvorschrift (Konkordat/interkantonale Vereinbarung; passt nicht exakt in eine der 14 Vokabularwerte — hilfsweise nächstliegend als Verwaltungsvorschrift kodiert, da vertragsförmige Selbstbindung mehrerer Kantone ohne Bundesgesetzcharakter; Schema-Grenzfall an W4 gemeldet)
- E: Planung/Nachweis
- F1 (E3): schweigend/nicht regelungsgegenständlich in Bezug auf Wiederverwendung (die IVHB harmonisiert ausschliesslich 30 formale Messweisen und Baubegriffe [z. B. Gesamthöhe, Grenzabstand, Vollgeschosszahl] zwischen den beigetretenen Kantonen; sie enthält keine materiellen Anforderungen an Bauprodukte, Standsicherheit oder Bauteilherkunft) · F2 (E3): schweigend (kein feststellbarer Bezug zur Bauteilwiederverwendung; die IVHB ist als Grenzfall aufgenommen, um die Abgrenzung zu belegen, dass „kantonale Harmonisierung" in der Schweiz sich strukturell auf Messgrössen, nicht auf materielles Bauproduktrecht bezieht — anders als etwa die deutsche MVV TB)
- G: entfällt
- Kernaussage: Die IVHB ist ein Konkordat der Kantone zur Vereinheitlichung von rund 30 formalen Baubegriffen und Messweisen (u. a. Gebäudehöhe, Abstände, Geschosszahl), damit gleiche Begriffe in allen beigetretenen Kantonen gleich verstanden werden; die zulässigen Masse selbst bleiben kantonal/kommunal geregelt. 17 Kantone sind formell beigetreten, der Kanton Zürich wendet die Harmonisierung laut Sekundärquelle materiell an, ohne dem Konkordat selbst beigetreten zu sein. Für die Bauteilwiederverwendung ist die IVHB nicht einschlägig — sie zeigt aber exemplarisch, dass selbst der punktuell existierende kantonsübergreifende Harmonisierungsmechanismus in der Schweiz rein formal-begrifflich, nicht materiell-bauproduktrechtlich ausgestaltet ist.
- Wortlautbeleg (Originalsprache): "Die Harmonisierung soll die wichtigsten Baubegriffe und Messweisen gesamtschweizerisch vereinheitlichen … Baubegriffe und Messweisen werden formal definiert, die Masse selber werden jedoch nicht materiell festgelegt." (BPUK/IVHB-Informationsportal, sinngemäss nach Recherche-Zusammenfassung)
- Beleg-Quelle: B3 Sekundärquelle (Recherche-Zusammenfassung aus BPUK-/Gemeinde-Webseiten, IVHB-Vereinbarungstext selbst in dieser Session nicht im Volltext gelesen) · Zugänglichkeit: frei-primär (Vereinbarungstext grundsätzlich frei zugänglich, hier nicht abgerufen) · Bindungsakt: entfällt (dient hier nur der Abgrenzung, nicht als tragendes Reuse-Regelungsobjekt)
- Quelle: Tier 3 (Verwaltungswebseiten, nur als Struktur-/Existenznachweis) · https://www.bpuk.ch/bpuk/konkordate/ivhb · http://ivhb.ch/ · Fassung(as-amended) in Kraft seit 2010-11-26 · Zugriff 2026-08-11
- Status: in Kraft · Datum: 2010-11-26
- Sub-Ebene: Stichprobe [17 Kantone laut BPUK-Beitrittsliste] / nicht erhoben [Detailprüfung einzelner kantonaler Umsetzungserlasse]
- Relationen: grenzt REG-CH-2-008 ab (zeigt, was kantonsübergreifende Harmonisierung in CH NICHT leistet)
- Konfidenz: gesichert (Existenz/Zweck/Kantonsliste), abgeleitet (Nichteinschlägigkeit für Reuse als Negativbefund)

---

## 3 · Abfall-/Stoffrecht

### REG-CH-3-010 · USG Art. 7 Abs. 6/6bis — Abfallbegriff und „Vorbereitung zur Wiederverwendung" als Behandlung (Grundnorm, novelliert 2024/2025)
- Titel: Bundesgesetz vom 7. Oktober 1983 über den Umweltschutz (Umweltschutzgesetz, USG)
- Fundstelle: Art. 7 Abs. 6 (Abfallbegriff), Art. 7 Abs. 6bis (Entsorgung/Behandlung, Fassung des zweiten Satzes gemäss Änderung vom 15.03.2024, in Kraft seit 01.01.2025)
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 3 Abfall-/Stoffrecht · Normtyp: Grundnorm/Begriffsnorm (Abfallbegriff determiniert Anwendbarkeit der gesamten VVEA und weiterer Abfallerlasse)
- C: materialübergreifend
- D: Gesetz
- E: Abfallstatus; Aufbereitung/Prüfung · E-Wirkung (Abfallstatus): vermeidet (der subjektive Entledigungswille ist konstitutiv — ohne ihn entsteht gar kein Abfallstatus; Phase bewusst vermeidbar) — Doppelkodierung Grenzoperation: „Vorbereitung zur Wiederverwendung" nach Art. 7 Abs. 6bis Satz 2 markiert ausdrücklich die Grenze Abfallstatus ↔ Aufbereitung/Prüfung
- F1 (E3): ermöglichend (der Abfallbegriff nach Art. 7 Abs. 6 ist — wie im deutschen KrWG § 3 — subjektiv/final an den „Entledigungswillen" des Inhabers geknüpft [„bewegliche Sachen, deren sich der Inhaber entledigt"]; ein direkt im selben Bauwerk wiederverwendetes oder von Anfang an ohne Entledigungsabsicht behandeltes Bauteil kann bereits tatbestandlich ausserhalb des Abfallbegriffs bleiben; zudem wurde die „Vorbereitung zu … [der] Wiederverwendung" 2024/2025 ausdrücklich als Unterfall der „Behandlung" in den Gesetzestext aufgenommen — vergleichbar der Funktion von KrWG § 3 Abs. 24) · F2 (E3): bedingend (der objektive Ergänzungstatbestand „deren Entsorgung im öffentlichen Interesse geboten ist" kann trotz fehlenden subjektiven Entledigungswillens einen Abfallstatus auslösen, etwa bei Kontaminationsverdacht; die genaue Abgrenzung im Rückbau-Einzelfall bleibt Vollzugsfrage)
- G: Statusfeststellung/Anwendbarkeitsprüfung (inferiert, E3 — der Nachweis, dass für ein konkretes ausgebautes Bauteil kein Entledigungswille vorliegt bzw. kein öffentliches Entsorgungsinteresse besteht, ist im Vollzug zu erbringen, auch wenn Art. 7 selbst keinen eigenen Nachweistatbestand formuliert)
- Kernaussage: Art. 7 Abs. 6 definiert Abfälle als „bewegliche Sachen, deren sich der Inhaber entledigt oder deren Entsorgung im öffentlichen Interesse geboten ist" — ein zweigliedriger, subjektiv-objektiver Abfallbegriff, strukturell analog zur deutschen KrWG-Definition. Mit der am 1. Januar 2025 in Kraft getretenen Änderung vom 15. März 2024 wurde der zweite Satz von Abs. 6bis präzisiert: Als „Behandlung" gelten seither ausdrücklich auch „die Vorbereitung zu deren Wiederverwendung" — die Schweiz hat damit, zeitlich parallel zu vergleichbaren EU-rechtlichen Entwicklungen, den Begriff der Wiederverwendungsvorbereitung erstmals explizit gesetzlich verankert. Eine ausdrückliche „Ende der Abfalleigenschaft"-Regel (Abfallende-Kriterien wie EU-WFD Art. 6) wurde in Art. 7 USG NICHT gefunden — dieser Punkt bleibt für die Schweiz ausdrücklich als offen/schweigend markiert, nicht als recherchierte Tatsache.
- Wortlautbeleg (Originalsprache): "Abfälle sind bewegliche Sachen, deren sich der Inhaber entledigt oder deren Entsorgung im öffentlichen Interesse geboten ist." (Art. 7 Abs. 6); "Die Entsorgung der Abfälle umfasst ihre Verwertung oder Ablagerung sowie die Vorstufen Sammlung, Beförderung, Zwischenlagerung und Behandlung. Als Behandlung gelten jede physikalische, chemische oder biologische Veränderung der Abfälle und die Vorbereitung zu deren Wiederverwendung." (Art. 7 Abs. 6bis)
- Beleg-Quelle: B0 Primärtext-Volltext (Fedlex-PDF/A, Stand am 1. April 2026, vollständig per pdftotext ausgelesen, 60 S.) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt (Bundesgesetz selbst)
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/1984/1122_1122_1122/de · Volltext: https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/1984/1122_1122_1122/20260401/de/pdf-a/fedlex-data-admin-ch-eli-cc-1984-1122_1122_1122-20260401-de-pdf-a.pdf · Fassung(as-amended) 2026-04-01 · Zugriff 2026-08-11
- Status: in Kraft (Grundtatbestand seit 1985; Abs. 6bis Satz 2 seit 2025-01-01) · Datum: 2024-03-15 (Änderungsgesetz) / 2025-01-01 (Inkrafttreten)
- Sub-Ebene: entfällt (A=national)
- Relationen: determiniert Anwendbarkeit von REG-CH-3-011/012/013/014; wird konkretisiert durch REG-CH-3-012/013 (VVEA); strukturanalog zu KrWG § 3 (DE-Extraktion, kein Ziel-ID in diesem Extraktionsumfang referenziert)
- Konfidenz: gesichert

---

### REG-CH-3-011 · USG Art. 30d — Verwertungspflicht mit ausdrücklichem Wiederverwendungsvorrang (novelliert 2024/2025)
- Titel: wie REG-CH-3-010
- Fundstelle: Art. 30 (Grundsätze), Art. 30d Abs. 1 (Verwertung, Fassung gemäss Änderung vom 15.03.2024, in Kraft seit 01.01.2025)
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 3 Abfall-/Stoffrecht
- C: materialübergreifend
- D: Gesetz
- E: Abfallstatus; Aufbereitung/Prüfung
- F1 (E3): ermöglichend (Art. 30d Abs. 1 stellt die „Wiederverwendung" ausdrücklich gleichrangig neben die „stoffliche Verwertung" als vorrangige Entsorgungsform vor Ablagerung/thermischer Behandlung — eine hierarchische Höherstufung der Wiederverwendung im Gesetzeswortlaut, die vor der Novelle 2024/2025 in dieser Deutlichkeit nicht bestand) · F2 (E3): bedingend (die Pflicht steht unter dem doppelten Vorbehalt „technisch möglich und wirtschaftlich tragbar" sowie geringerer Umweltbelastung als Alternativen — unbestimmte Rechtsbegriffe, deren Anwendung auf den Einzelfall Auslegungsspielraum lässt)
- G: rechnerischer Nachweis / Dokumentenlage (inferiert, E3 — die Erfüllung der Verwertungspflicht dürfte im Vollzug über Standortdokumentation und ggf. Ökobilanzvergleich nachzuweisen sein, ein expliziter Nachweistatbestand ist im Gesetzestext selbst nicht ausformuliert)
- Kernaussage: Art. 30d Abs. 1 USG verpflichtet dazu, Abfälle „der Wiederverwendung zuzuführen oder stofflich zu verwerten", wenn dies technisch möglich und wirtschaftlich tragbar ist und die Umwelt weniger belastet als eine andere Entsorgung oder die Herstellung neuer Produkte. Diese Fassung stammt aus der am 1. Januar 2025 in Kraft getretenen Gesetzesänderung vom 15. März 2024 und stellt die Wiederverwendung erstmals ausdrücklich gleichrangig neben die stoffliche Verwertung — eine der deutlichsten reuse-fördernden Grundsatznormen, die in dieser Extraktion für die Schweiz gefunden wurde, und zeitlich auffällig zeitgleich mit der Verabschiedung der EU-VO 2024/3110.
- Wortlautbeleg (Originalsprache): "Abfälle müssen der Wiederverwendung zugeführt oder stofflich verwertet werden, wenn dies technisch möglich und wirtschaftlich tragbar ist und die Umwelt weniger belastet als eine andere Entsorgung oder die Herstellung neuer Produkte." (Art. 30d Abs. 1)
- Beleg-Quelle: B0 Primärtext-Volltext · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/1984/1122_1122_1122/20260401/de/pdf-a/fedlex-data-admin-ch-eli-cc-1984-1122_1122_1122-20260401-de-pdf-a.pdf · Fassung(as-amended) 2026-04-01 · Zugriff 2026-08-11
- Status: in Kraft (Fassung seit 2025-01-01) · Datum: 2024-03-15 (Änderungsgesetz) / 2025-01-01 (Inkrafttreten)
- Sub-Ebene: entfällt
- Relationen: konkretisiert wird durch REG-CH-3-012/013/014 (VVEA); determiniert Anwendbarkeit von REG-CH-3-012 ff.; wird konkretisiert durch REG-CH-3-010; **[Prüfung 2026-08-13 ergänzt:]** steht im selben Änderungserlass (BG vom 15. März 2024, in Kraft 2025-01-01) wie das bislang in keiner CH-Ernte-Datei erfasste USG Art. 35j "Ressourcenschonendes Bauen" (jetzt nachgetragen als REG-CH-4-004a in `CH-F4-7.md`) — Art. 35j Abs. 1 Bst. d nennt "die Wiederverwendung von Bauteilen in Bauwerken" wortwörtlich als künftigen bundesrätlichen Regelungsgegenstand
- Konfidenz: gesichert (Wortlaut per pdftotext an zwei unabhängigen Fassungsständen [2025-01-01, 2026-04-01] gegengeprüft — Prüfung 2026-08-13), abgeleitet (praktische Reichweite des Vorrangs im Bauabfall-Einzelfall)

---

### REG-CH-3-012 · VVEA Art. 16 — Angaben zur Entsorgung von Bauabfällen (Entsorgungskonzept-Pflicht ab Schwellenwert/Schadstoffverdacht)
- Titel: Verordnung vom 4. Dezember 2015 über die Vermeidung und die Entsorgung von Abfällen (Abfallverordnung, VVEA)
- Fundstelle: Art. 16 Abs. 1–2
- A: national · Downstream-Verifikationsstatus: entfällt (Bundesverordnung, gestützt auf USG Art. 30 ff., unmittelbar bundesweit anwendbar; Vollzug liegt bei den für die Baubewilligung zuständigen — meist kommunalen/kantonalen — Behörden, ohne dass die materielle Pflicht selbst kantonal variiert)
- B: Primärfeld 3 Abfall-/Stoffrecht
- C: materialübergreifend
- D: RVO
- E: Bestandserkundung; Rückbau/Sicherung · E-Wirkung (Bestandserkundung): erzwingt (die Angabepflicht erzwingt bei Überschreitung des Schwellenwerts oder Schadstoffverdacht eine vorgängige Erkundung der anfallenden Abfälle, unabhängig davon, ob die Bauherrschaft dies sonst täte)
- F1 (E3): ermöglichend (die Pflicht zur Angabe von Art, Qualität und Menge der anfallenden Bauabfälle im Baubewilligungsgesuch schafft einen strukturierten, behördlich geprüften Erkundungsschritt VOR Rückbau — funktional vergleichbar mit einem Pre-Demolition-Audit, wenngleich ohne die methodische Standardisierung einer DIN-SPEC-91484-artigen Norm) · F2 (E3): bedingend (die Pflicht greift nur ab Schwellenwert [voraussichtlich > 200 m³ Bauabfälle] oder bei Verdacht auf umwelt-/gesundheitsgefährdende Stoffe [PCB, PAK, Blei, Asbest] — kleinere Rückbau-/Umbauprojekte, oft gerade die für Bauteilwiederverwendung relevanten kleinteiligen Fälle, fallen aus der expliziten Angabepflicht heraus)
- G: Dokumentenlage (explizit, E1 — Angaben im Baubewilligungsgesuch; Nachweis der tatsächlichen Entsorgung auf Verlangen der Behörde)
- Kernaussage: Art. 16 VVEA verpflichtet die Bauherrschaft, im Baubewilligungsgesuch Angaben zu Art, Qualität und Menge der anfallenden Bauabfälle sowie zur vorgesehenen Entsorgung zu machen, sofern voraussichtlich mehr als 200 m³ Bauabfälle anfallen oder umwelt-/gesundheitsgefährdende Stoffe (PCB, PAK, Blei, Asbest) zu erwarten sind. Hat die Bauherrschaft ein Entsorgungskonzept erstellt, muss sie der Behörde auf Verlangen nach Bauabschluss die vorschriftsgemässe Entsorgung nachweisen. Die Norm etabliert damit einen dokumentenbasierten Erkundungs- und Nachweismechanismus vor und nach Rückbau, der strukturell an die Prozessphase Bestandserkundung anknüpft, ohne diese Terminologie selbst zu verwenden.
- Wortlautbeleg (Originalsprache): "Bei Bauarbeiten muss die Bauherrschaft der für die Baubewilligung zuständigen Behörde im Rahmen des Baubewilligungsgesuchs Angaben über die Art, Qualität und Menge der anfallenden Abfälle und über die vorgesehene Entsorgung machen, wenn: a. voraussichtlich mehr als 200 m3 Bauabfälle anfallen; oder b. Bauabfälle mit umwelt- oder gesundheitsgefährdenden Stoffen wie polychlorierte Biphenyle (PCB), polycyclische aromatische Kohlenwasserstoffe (PAK), Blei oder Asbest zu erwarten sind." (Art. 16 Abs. 1)
- Beleg-Quelle: B1 amtliche Konsolidierung eingesehen (Fedlex-PDF/A, per pdftotext ausgelesen; Fassung Stand 1. April 2022 — **Lücke:** eine jüngere Änderung [AS 2024 744] wurde in der Recherche als existent identifiziert, ihr Einfluss auf Art. 16/17/19/20 wurde in dieser Session NICHT einzeln primärquellenbasiert verifiziert; als B1 statt B0 markiert, um diese Unsicherheit sichtbar zu halten) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.fedlex.admin.ch/eli/cc/2015/891/de · Volltext (eingesehene Fassung): https://www.fedlex.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2015/891/20220401/de/pdf-a/fedlex-data-admin-ch-eli-cc-2015-891-20220401-de-pdf-a-1.pdf · Fassung(as-amended) 2022-04-01 (eingesehen; neuere Fassung laut AS 2024 744 nicht ausgeschlossen und nicht geprüft) · Zugriff 2026-08-11
- Status: in Kraft · Datum: 2016-01-01 (Ersterlass VVEA), Art. 16 selbst ohne erkennbare spätere Einzeländerung in der eingesehenen Fassung
- Sub-Ebene: entfällt (A=national)
- Relationen: konkretisiert REG-CH-3-010/011; wird kombiniert mit REG-CH-2-008 (Baubewilligungsverfahren als Vollzugsrahmen)
- Konfidenz: gesichert (Wortlaut der eingesehenen Fassung), unklar (Aktualität dieser Fassung zum exakten Stichtag 2026-08-11)

---

### REG-CH-3-013 · VVEA Art. 17 — Trennung von Bauabfällen auf der Baustelle (Sortentrennung als Verwertungsvoraussetzung)
- Titel: wie REG-CH-3-012
- Fundstelle: Art. 17 Abs. 1–3
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 3 Abfall-/Stoffrecht
- C: materialübergreifend (Bst. a–b, e–f); Mauerwerk/mineralisch (Bst. c: Ausbauasphalt, Betonabbruch, Strassenaufbruch, Mischabbruch, Ziegelbruch, Gips); Holz (Bst. d, anteilig); Glas/Fassade (Bst. d, anteilig); Aluminium/NE-Metalle (Bst. d, anteilig: Metalle)
- D: RVO
- E: Rückbau/Sicherung
- F1 (E3): bedingend (die Pflicht zur sortenreinen Trennung auf der Baustelle — Sonderabfälle, Ober-/Unterboden, Aushub-/Ausbruchmaterial, mineralische Fraktionen, „weitere stofflich verwertbare Abfälle wie Glas, Metalle, Holz und Kunststoffe", brennbare Abfälle, andere Abfälle — ist Voraussetzung für die nachfolgende stoffliche Verwertung inkl. Wiederverwendung nach Art. 30d; ohne Trennung droht Vermischung und damit Downcycling statt Wiederverwendung) · F2 (E3): ermöglichend (die explizite Nennung von „Glas, Metalle, Holz" als eigene, sortenrein zu trennende Fraktion in Bst. d schafft die praktische Grundvoraussetzung dafür, dass Bauteile aus diesen Materialfamilien überhaupt in einem Zustand anfallen, der eine Wiederverwendung statt Downcycling ermöglicht — auch wenn die Norm selbst nur die Trennung, nicht die Wiederverwendung als solche regelt)
- G: Sichtprüfung / Dokumentenlage (explizit, E1 — Trennung erfolgt nach visueller/praktischer Sortierung auf der Baustelle; Abs. 2 erlaubt bei betrieblicher Unmöglichkeit Nachtrennung in geeigneten Anlagen)
- Kernaussage: Art. 17 VVEA schreibt vor, dass bei Bauarbeiten Sonderabfälle von übrigen Abfällen zu trennen sind und die übrigen Bauabfälle auf der Baustelle in sechs Kategorien sortenrein zu trennen sind, soweit betrieblich möglich (sonst Nachtrennung in geeigneten Anlagen, Abs. 2). Die Behörde kann eine weitergehende Trennung verlangen, wenn dadurch zusätzliche Anteile verwertet werden können (Abs. 3) — eine ausdrückliche Öffnung, die im Einzelfall auch strengere Anforderungen zugunsten höherwertiger Verwertung (inkl. Wiederverwendung) ermöglicht.
- Wortlautbeleg (Originalsprache): "Bei Bauarbeiten sind Sonderabfälle von den übrigen Abfällen zu trennen und separat zu entsorgen. Die übrigen Bauabfälle sind auf der Baustelle wie folgt zu trennen: … d. weitere stofflich verwertbare Abfälle wie Glas, Metalle, Holz und Kunststoffe, jeweils möglichst sortenrein" (Art. 17 Abs. 1); "Die Behörde kann eine weitergehende Trennung verlangen, wenn dadurch zusätzliche Anteile der Abfälle verwertet werden können." (Art. 17 Abs. 3)
- Beleg-Quelle: B1 (Fedlex-PDF/A eingesehen, Fassung 2022-04-01; gleiche Aktualitäts-Lücke wie REG-CH-3-012) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.fedlex.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2015/891/20220401/de/pdf-a/fedlex-data-admin-ch-eli-cc-2015-891-20220401-de-pdf-a-1.pdf · Fassung(as-amended) 2022-04-01 (eingesehen, Aktualität zum Stichtag nicht abschliessend verifiziert) · Zugriff 2026-08-11
- Status: in Kraft · Datum: 2016-01-01
- Sub-Ebene: entfällt
- Relationen: konkretisiert REG-CH-3-011; wird kombiniert mit REG-CH-3-012
- Konfidenz: gesichert (Wortlaut der eingesehenen Fassung), unklar (Aktualität zum exakten Stichtag)

---

### REG-CH-3-014 · VVEA Art. 19–20 — Verwertung von Aushub-/Ausbruchmaterial und mineralischen Abbruchabfällen (Ersatzbaustoff-Regime CH)
- Titel: wie REG-CH-3-012
- Fundstelle: Art. 19 Abs. 1–3 (Aushub-/Ausbruchmaterial, mit Verweis auf Anhang 3), Art. 20 Abs. 1–3 (mineralische Abfälle aus dem Abbruch von Bauwerken: Ausbauasphalt, Strassenaufbruch, Mischabbruch, Ziegelbruch, Betonabbruch)
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 3 Abfall-/Stoffrecht
- C: Mauerwerk/mineralisch; Stahlbeton/Fertigteile (Betonabbruch)
- D: RVO
- E: Aufbereitung/Prüfung
- F1 (E3): ermöglichend (Art. 19 f. schreiben eine weitgehend zwingende, abgestufte Verwertungspflicht für mineralische Bau-/Abbruchabfälle vor — unverschmutztes Aushubmaterial „möglichst vollständig" als Baustoff/Rohstoff, Betonabbruch „möglichst vollständig … als Rohstoff für die Herstellung von Baustoffen oder als Baustoff auf Deponien" — funktional das Schweizer Gegenstück zur deutschen Ersatzbaustoffverordnung, allerdings ohne deren eigenständigen Verordnungstitel, sondern integriert in die VVEA) · F2 (E3): bedingend (die Regelung adressiert primär stoffliche Verwertung als Rohstoff/Ersatzbaustoff, nicht die Wiederverwendung ganzer Bauteile; ein Betonfertigteil, das als intaktes Bauteil wiederverwendet werden soll statt als Recyclingbeton-Rohstoff aufbereitet zu werden, fällt nicht unter den Wortlaut von Art. 20 Abs. 3 — Bezugsgegenstand-Differenzierung: „Materialstrom" [Art. 19 f., einschlägig] vs. „ganzes Bauteil" [tatbestandlich nicht erfasst])
- G: Probenahme/Materialprüfung (explizit, E1 — Einhaltung der Anforderungen nach Anhang 3 Ziff. 1/2 bzw. der PAK-Grenzwerte in Art. 20 Abs. 1–2 setzt materialtechnische Prüfung voraus)
- Kernaussage: Art. 19 f. VVEA regeln die Verwertung von Aushub-/Ausbruchmaterial sowie mineralischen Abbruchabfällen (Ausbauasphalt, Strassenaufbruch, Mischabbruch, Ziegelbruch, Betonabbruch) mit differenzierten, teils zwingenden Verwertungspflichten und Grenzwerten (z. B. PAK-Grenzwert 250 mg/kg für Ausbauasphalt, Art. 20 Abs. 1–2). Dies bildet in der Schweiz das funktionale Gegenstück zur deutschen Ersatzbaustoffverordnung, ist aber in die allgemeine VVEA integriert statt eigenständig geregelt. Die Normen betreffen den Materialstrom (Aufbereitung zu Rezyklat/Ersatzbaustoff), nicht die Wiederverwendung intakter Bauteile — für Bauteil-Reuse im engeren Sinn bleibt die VVEA insoweit schweigend/nicht regelungsgegenständlich.
- Wortlautbeleg (Originalsprache): "Betonabbruch ist möglichst vollständig als Rohstoff für die Herstellung von Baustoffen oder als Baustoff auf Deponien zu verwerten." (Art. 20 Abs. 3); "Ausbauasphalt mit einem Gehalt bis zu 250 mg PAK pro kg, Strassenaufbruch, Mischabbruch und Ziegelbruch ist möglichst vollständig als Rohstoff für die Herstellung von Baustoffen zu verwerten." (Art. 20 Abs. 1)
- Beleg-Quelle: B1 (Fedlex-PDF/A eingesehen, Fassung 2022-04-01; gleiche Aktualitäts-Lücke wie REG-CH-3-012/013) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 · https://www.fedlex.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/2015/891/20220401/de/pdf-a/fedlex-data-admin-ch-eli-cc-2015-891-20220401-de-pdf-a-1.pdf · Fassung(as-amended) 2022-04-01 (eingesehen, Aktualität zum Stichtag nicht abschliessend verifiziert) · Zugriff 2026-08-11
- Status: in Kraft · Datum: 2016-01-01
- Sub-Ebene: entfällt
- Relationen: konkretisiert REG-CH-3-011; grenzt sich ab von einem (in dieser Extraktion nicht identifizierten) eigenständigen Bauteil-Reuse-Regime; verdrängt (lex specialis, materialstrombezogen) gegenüber der allgemeinen Verwertungspflicht Art. 30d USG für den Teilbereich mineralischer Bauabfälle
- Konfidenz: gesichert (Wortlaut der eingesehenen Fassung), unklar (Aktualität zum exakten Stichtag)

---

### REG-CH-3-015 · VeVA — Begleitschein-/Bewilligungspflicht für Sonderabfälle und kontrollpflichtige Abfälle
- Titel: Verordnung vom 22. Juni 2005 über den Verkehr mit Abfällen (VeVA)
- Fundstelle: Art. 6 (Begleitschein-Pflicht bei Übergabe)
- A: national · Downstream-Verifikationsstatus: entfällt
- B: Primärfeld 3 Abfall-/Stoffrecht
- C: Dämmstoffe+Schadstoffe (typischer Anwendungsfall: asbest-/schadstoffbelastete Bauteile als Sonderabfall); materialübergreifend (Verordnung als Ganzes)
- D: RVO
- E: Rückbau/Sicherung; Betrieb/Dokumentation
- F1 (E3): bedingend (ab 50 kg Sonderabfall oder kontrollpflichtigem Abfall pro Transport ist ein Begleitschein nach Anhang 1 zu führen; Entsorgungsunternehmen benötigen für die Annahme solcher Abfälle eine kantonale Bewilligung je Betriebsstandort — dies betrifft insbesondere schadstoffbelastete Rückbauteile, die vor einer etwaigen Wiederverwendung erst als unbedenklich freigegeben werden müssen) · F2 (E3): bedingend (das System schafft Rückverfolgbarkeit für kritische Fraktionen, wirkt aber administrativ aufwendig für kleinteilige, gemischte Rückbauströme, wie sie bei Bauteil-Reuse-Projekten häufig anfallen)
- G: Dokumentenlage / Erklärung Dritter (explizit, E1 — Begleitschein als Dokumentationspflicht, kantonale Bewilligung der Entsorgungsunternehmen als Drittnachweis)
- Kernaussage: Die VeVA konkretisiert die USG-Ermächtigung zu Sonderabfällen (Art. 30f USG) und schreibt für die Übergabe von Sonderabfällen und anderen kontrollpflichtigen Abfällen ab 50 kg pro Transport (inkl. Verpackung) einen Begleitschein nach Anhang 1 vor; annehmende Entsorgungsunternehmen benötigen für jeden Betriebsstandort eine kantonale Bewilligung. Für die Bauteilwiederverwendung ist dies vor allem bei schadstoffbelasteten oder -verdächtigen Bauteilen relevant, deren Rückbau/Zwischenlagerung dem Sonderabfallregime unterliegen kann, bevor eine Freigabe zur Wiederverwendung möglich ist.
- Wortlautbeleg (Originalsprache): "Ein Begleitschein ist ab 50 kg Sonderabfall oder kontrollpflichtigem Abfall pro Transport, Verpackung eingerechnet, erforderlich." (sinngemäss nach Art. 6 VeVA, wie in der Recherche-Zusammenfassung wiedergegeben — **Einschränkung:** der genaue Wortlaut von Art. 6 wurde in dieser Session nicht im amtlichen Fedlex-Volltext selbst nachgelesen, sondern über eine WebSearch-Zusammenfassung erschlossen; für den nächsten Bearbeitungsschritt zur Verifikation mit B0 vorgesehen)
- Beleg-Quelle: B2 amtliche Referenz, Volltext in dieser Session nicht selbst im O-Ton nachgelesen (nur Suchergebnis-Zusammenfassung mit Bezug auf Fedlex/odat.ch-Spiegelung) · Zugänglichkeit: frei-primär · Bindungsakt: entfällt
- Quelle: Tier 1 (Verordnung selbst, Existenz/Fundstelle bestätigt) / Tier 3 (Recherche-Zusammenfassung für den zitierten Wortlaut, nicht als alleiniger Beleg verwendet) · https://www.fedlex.admin.ch/eli/cc/2005/435/de (SR 814.610) · Fassung(as-amended) nicht verifiziert (Sekundärquelle nennt konsolidierten Stand 2025-08-01, in dieser Session nicht selbst am Fedlex-Original bestätigt) · Zugriff 2026-08-11
- Status: in Kraft · Datum: 2005-06-22 (Ersterlass)
- Sub-Ebene: entfällt
- Relationen: setzt um USG Art. 30f; wird kombiniert mit REG-CH-3-012 (Angaben zu Bauabfällen im Baubewilligungsverfahren)
- Konfidenz: abgeleitet (Existenz und Grundmechanismus plausibel, exakter Wortlaut/Fassungsstand nicht B0/B1-verifiziert — Nacherhebung empfohlen)

---

## Zusammenfassung offener Punkte dieser Extraktion (ehrlich markiert)

- **CH-quellen.md existierte nicht** im Ticketordner; diese Extraktion wurde ohne vorgeschaltete Quellenkarte direkt primärquellenbasiert durchgeführt. Eine Rückkopplung mit W2-Koordination wird empfohlen, falls eine gesonderte Quellenkarte für CH parallel an anderer Stelle erstellt wurde.
- **VVEA-Fassungsstand:** Die eingesehene VVEA-Fassung ist auf 2022-04-01 datiert; eine spätere Änderung (AS 2024 744) wurde als existent identifiziert, ihr Einfluss auf Art. 16/17/19/20 (REG-CH-3-012/013/014) wurde NICHT primärquellenbasiert verifiziert. Nacherhebung mit korrektem konsolidiertem Fedlex-Datum empfohlen.
- **VeVA (REG-CH-3-015):** nur B2, kein Volltextzugriff in dieser Session — Nacherhebung mit direktem Fedlex-PDF-Abruf empfohlen.
- **MRA-Anhang-1-Kapitel-16-Volltext (REG-CH-1-003):** nicht selbst im O-Ton gelesen, Befund stützt sich auf die im BBL-Auftrag erstellte gfs.bern-Studie (B1) statt auf den Abkommenstext selbst (B2).
- **SIA-Normen (REG-CH-2-006/007):** materieller Norminhalt durchgehend paywalled-nicht-eingesehen; nur Titel/Gegenstand/Zeitplan über amtsnahe Sekundärquellen belegt.
- **Kantonales Baubewilligungsrecht (REG-CH-2-008/009):** Stichprobe-und-Deklaration gemäss Taxonomie-Freeze-Vorgabe für CH; nur Struktur, kein einziger kantonaler Gesetzestext im Volltext gelesen. Nacherhebung mit 2–3 Kantonen (z. B. ZH, BE, GE für Sprachregionen-Streuung) empfohlen.
- **„Ende der Abfalleigenschaft"/Abfallende-Kriterien:** in USG Art. 7 keine explizite Regel gefunden (anders als teilweise vermutet) — bewusst NICHT als eigenes Regelungsobjekt erfunden, sondern als Negativbefund in REG-CH-3-010 vermerkt. Ob VVEA an anderer Stelle (ausserhalb der in dieser Session gelesenen Art. 1–29) eine Abfallende-Regel enthält, wurde nicht abschliessend geprüft.
- **Feld 2 ausserhalb Tragwerk/Zulassung** (z. B. spezifische Erdbebennachweise, Glas-Bemessung) wurde nicht vertieft — Fokus lag auf den für Reuse strukturell zentralen Objekten (Eurocode-Übergang, SIA 269, kantonales Verfahren, IVHB-Abgrenzung).
