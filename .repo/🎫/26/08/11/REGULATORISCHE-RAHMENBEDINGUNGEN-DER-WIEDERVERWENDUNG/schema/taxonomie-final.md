# Taxonomie-Freeze — Verbindliches Sieben-Achsen-Schema (W1)

**Projekt:** BBSR/Zukunft Bau „Entwerfen mit Bestand" (Az. 10.08.18.7-25.06, LUH Hannover + UdK Berlin)
**Status:** EINGEFROREN am 2026-08-11 · Stichtag aller Erhebungen: as-amended zum 2026-08-11
**Geltung:** Ab sofort **bindend** für alle Länder-, Material- und Querschnittsagenten (W2, W3, W3b, W4). Abweichungen sind nicht zulässig; erkannte Schema-Grenzen sind als Freitext im Objekt zu vermerken und an W4 zu melden, nicht durch eigenmächtige Vokabularänderung zu „lösen".

Grundlage: die drei W0-Piloten (`pilot-de-produkt.md`, `pilot-de-abfall.md`, `pilot-de-zie.md`, zusammen 29 Regelungsobjekte) und die beiden Verfeinerungsentwürfe (`taxonomie-entwurf-1.md`, `-2.md`). Beide Entwürfe waren sich in der Substanz einig, dass **keine** neue Achse und **keine** Änderung der Grundstruktur (sieben Achsen, ID-Schema, Blockformat, Evidenzgrade E1–E3, Belegregeln B0–B4) nötig ist — nur additive Vokabular- und Mehrfachwert-Erweiterungen. Dieser Freeze folgt dieser Einschätzung und entscheidet die Streitpunkte, an denen die Entwürfe divergierten (dokumentiert in Abschnitt 12).

Grundprinzip aller Kodierung (unverändert): **Lieber ein Objekt ehrlich als „schweigend/offen/unklar" markiert als eine erfundene Regel.**

---

## 0. Grundstruktur (unverändert, nicht verhandelbar)

- **Sieben Achsen** A–G, jede mit kontrolliertem Vokabular (unten).
- **ID-Schema:** `REG-<ISO2 oder EU>-<Feld>-<lfd 3-stellig>`, z. B. `REG-DE-1-001`, `REG-EU-3-004`. `<Feld>` = die B-Primärfeld-Ziffer (1–7). Die laufende Nummer ist pro Jurisdiktion **durchgehend über alle Felder** (nicht pro Feld neu), damit IDs eindeutig bleiben.
- **Evidenzgrade E1/E2/E3** — siehe Abschnitt 10. Werden **je Achse einzeln** vergeben, nicht als ein Wert pro Objekt.
- **Beleg-Quelle B0–B4, Zugänglichkeit, Bindungsketten-Regel** — siehe Abschnitt 9.
- **Regelungsobjekt-Blockformat** — siehe Abschnitt 8.

---

## 1. Achse A — Jurisdiktion/Ebene (Bindungsebene) + Attribut A-Ursprung

**A kodiert die BINDUNGSEBENE** — die Ebene, auf der die Regelung für die handelnden Akteure (Planer, Rückbauunternehmen, Bauaufsicht) verbindlich wird. Nicht die Erarbeitungsebene.

**Kontrolliertes Vokabular A (drei Werte, Pflicht, einwertig):**
- `EU/EEA`
- `national`
- `sub-national`

**Attribut `A-Ursprung` (Pflicht nur bei Abweichung von A; sonst weglassen — Default: A-Ursprung = A):**
- `EU/EEA` · `national` · `sub-national` · `international (nicht-EU/EEA, z. B. ISO/IEC, bilaterales MRA)`

Das Attribut trennt Erarbeitungs- von Bindungsebene, wo diese systematisch auseinanderfallen (ISO-Übernahmen, Eurocodes, IVHB-artige Konstrukte). Beispiele:
- ISO 13822 (DIN ISO 13822): **A = national** (Bindung erst über DIN-Übernahme + VV-TB-Listung), **A-Ursprung = international**.
- Eurocode-Entwurf (prEN 1990-2): **A = EU/EEA** (CEN-Erarbeitung, noch keine nationale Bindung), A-Ursprung deckungsgleich → Attribut entfällt. Sobald Nationaler Anhang + VV-TB-Listung vorliegen, wechselt A zu national/sub-national, A-Ursprung bleibt EU/EEA.

**Pflichtfeld `Downstream-Verifikationsstatus`** (immer bei Muster-/Bund-Länder-Gremiumsdokumenten und bei A=sub-national): einer von
`verifiziert in [Land/Länder]` · `strukturell angenommen, nicht verifiziert` · `nicht geprüft`.
Macht sichtbar, ob die A-Einstufung auf Beleg oder auf Annahme beruht.

**Kodierregel für ARGEBAU-/Muster-/Bund-Länder-Dokumente (verbindlich):**
- **A = sub-national**, wenn eine Downstream-Transformation in mindestens einer Landesnorm **nachgewiesen ODER strukturell zwingend** ist (z. B. MBO-Wortlaut wird laut Vollzugspraxis in allen 16 LBOs nachvollzogen → sub-national, Downstream-Status entsprechend gesetzt).
- **A = national**, wenn das Dokument selbst der Endpunkt ist und keine Rechtsform-Transformation in Landesrecht erfolgt (Merkblätter, Empfehlungen ohne Verordnungscharakter — z. B. ARGEBAU-Standsicherheits-Hinweise).

### Verbindliche Abgrenzung A (ausgeschlossener Falschtreffer)
Eine **EU-Richtlinie**, die erst durch nationale Umsetzung wirkt, wird **NICHT als A=EU/EEA** kodiert, weil sie für Akteure vor Ort nicht unmittelbar bindet — sie erscheint als A=national/sub-national (Umsetzungsebene) mit A-Ursprung=EU/EEA. **A=EU/EEA zählt nur** für unmittelbar geltendes EU-Recht (EU-VO wie CPR 2024/3110). Ebenso: ein **Landesgesetz (NBauO)** zählt **NICHT als national**, auch wenn bundesweit strukturgleiche LBOs existieren — jede LBO ist ein eigenes A=sub-national-Objekt mit eigener Sub-Ebene-Kennung.

---

## 2. Achse B — Regelungsfeld (Primärfeld + Nebenfelder) + Normtyp-Flag

**B wird von Einfach- auf Primär-/Nebenfeld-Struktur umgestellt.**
- **Primärfeld** (Pflicht, genau ein Wert 1–7): das Feld, dessen Pflichten/Rechtsfolgen an der **zitierten Fundstelle** ausgelöst werden und im Kernaussage-Text tragend sind — nicht das Feld des Gesamtdokuments.
- **Nebenfelder** (optional, mehrwertig): weitere Felder, die im selben Regelungsobjekt substanziell (nicht nur beiläufig) mitgeregelt werden.

**Kontrolliertes Vokabular der Felder (unverändert, 1–7):**
1. Produkt-/Konformitätsrecht (CPR 305/2011 → 2024/3110, CE, Ü-Zeichen, DoP, hEN)
2. Bautechnische Zulassung/Standsicherheit (ZiE/aBG/vBG, ATEx, Eurocode-NA, Bestandsbewertung)
3. Abfall-/Stoffrecht (Abfallende, KrWG, VVEA, VLAREMA, Ersatzbaustoffe)
4. Schutzziele (Brand/Energie/Schadstoffe/Gesundheit)
5a. Vergaberecht (hart)
5b. Anreize/Förderung (weich)
6. Normen/Regelwerke
7. Haftung/Gewährleistung (BGB/VOB/B, Produkthaftungs-RL 2024/2853, Versicherbarkeit)

**Auswertungsregel:** Filterung/Gruppierung (Konflikttabelle Anlage RG) erfolgt **ausschließlich nach Primärfeld**. Nebenfelder sind Kontext, kein Filterkriterium. Ein MVV-TB-Regelungsobjekt zu einem Brandschutzkapitel bekommt Primärfeld 4, auch wenn dasselbe Gesamtdokument anderswo Feld 1 regelt (dort separates Objekt mit Primärfeld 1).

**Flag `Normtyp` (optional, Default = operativ):**
- `Grundnorm/Begriffsnorm` = definiert einen Tatbestand, von dessen Erfüllung/Nichterfüllung die **Anwendbarkeit anderer Regelungsobjekte** im selben oder verwandten Feld abhängt (Gatekeeper). Beispiele: KrWG § 3 (Abfallbegriff), CPR Art. 2/3 (Anwendungsbereich).
- `operative Norm` = Regelfall (kein Flag nötig).
Grundnormen erhalten auf der Relationen-Achse bevorzugt „determiniert Anwendbarkeit von" statt „kollidiert mit" (siehe Abschnitt 7).

### Verbindliche Abgrenzung B (ausgeschlossener Falschtreffer)
Ein Instrument zählt **NICHT als Feld 2**, sondern bleibt **Feld 1**, wenn sein Kern die Markt-Konformität eines Produkts ist (CPR) und die Standsicherheit nur als eine von mehreren Grundanforderungen mitgeregelt wird. Umgekehrt zählt es als Feld 2, wenn sein Kern ein Nachweis-/Zulassungsverfahren für Standsicherheit oder Bauteilverwendbarkeit ist (ZiE, abZ, aBG), auch wenn es sekundär CE-/Ü-Zeichen-Fragen berührt. — Und: eine **operative Norm** ist **KEINE Grundnorm**, nur weil sie selbst Voraussetzungen enthält, solange diese nur die eigene Rechtsfolge auslösen und nicht die Anwendbarkeit anderer Objekte steuern.

---

## 3. Achse C — Materialfamilie (mehrwertig zulässig)

**Kontrolliertes Vokabular C (zehn Werte, ein oder mehrere zulässig):**
- Baustahl
- Stahlbeton/Fertigteile
- Mauerwerk/mineralisch
- Holz
- Glas/Fassade
- Aluminium/NE-Metalle
- Dämmstoffe+Schadstoffe
- TGA+Ausbau
- **materialübergreifend** (horizontales Recht, das nicht nach Material differenziert — z. B. CPR, KrWG-Grundnormen, MBO-Verfahrensnormen)
- **Verbund-/Systembauteil** *(neu)* — der geregelte/betroffene Nachweisgegenstand ist ein aus mehreren Materialfamilien zusammengesetztes Einzelbauteil (Fenster: Glas+Rahmen+Beschlag+Dichtung; Tür; Fassadenelement), das nicht sinnvoll in Einzelmaterialien zerlegt bewertet werden kann.

**Mehrfachwert-Regel:** C ist mehrwertig, wenn ein Dokument mehrere im Text unterscheidbare materialspezifische Abschnitte enthält (z. B. BW-Leitfaden: „Baustahl, Holz" wegen Anhängen A/B). Mehrfachwerte gelten additiv, nicht nur für horizontale Fälle.

**Auswertungshinweis (kein Schemaeingriff):** In Feld 1 und im horizontalen Verfahrensrecht des Feldes 2 ist C strukturell wenig trennscharf (fast alles „materialübergreifend") — das ist selbst ein Befund (horizontales Recht dominiert Produkt-/Verfahrensrecht). Feld-1/2-Auswertungen nach B, nicht nach C gliedern.

### Verbindliche Abgrenzung C (ausgeschlossener Falschtreffer)
**„Verbund-/Systembauteil" zählt NICHT**, wenn der Rechtstext den Nachweis pro Einzelmaterial verlangt, auch wenn viele Materialien betroffen sind — Beispiel GewAbfV § 8 mit zehn getrennten Werkstofffraktionen ist das **Gegenteil** eines Verbundbauteil-Ansatzes und daher C=materialübergreifend, nicht Verbund. „Verbund-/Systembauteil" gilt nur, wenn der Rechtstext selbst das **zusammengesetzte Bauteil** (nicht dessen Einzelmaterialien) als Bezugsobjekt der Pflicht benennt.

---

## 4. Achse D — Rechtsform (Ordinalskala der Projektkonvention formeller Verbindlichkeit)

**Wichtig:** D ist eine **Projektkonvention** der formellen Verbindlichkeit, **KEINE Rechtshierarchie**. Die Ordinalstellung ist ein Kodier- und Sortierhilfsmittel, keine Aussage über Normenrang.

**Kontrolliertes Vokabular D (vierzehn Werte, einwertig; Ordinalreihenfolge von „stärkste unmittelbare formelle Bindung" nach „schwächste"):**

| # | D-Wert | Kurzcharakter |
|---|---|---|
| 1 | `EU-VO` | unmittelbar geltendes EU-Recht |
| 2 | `Rechtsprechung/Urteil` *(neu)* | gerichtlicher Tenor mit unmittelbarer Bindungswirkung |
| 3 | `EU-RL` | umsetzungsbedürftiges EU-Recht |
| 4 | `Gesetz` | Parlamentsakt (Bund/Land) |
| 5 | `RVO` | Rechtsverordnung |
| 6 | `Verwaltungsvorschrift` | Behördenerlass mit (Innen-/begrenzter Außen-)Wirkung |
| 7 | `Techn.Baubestimmung` | über § 85a MBO/VV-TB-System bindend gemachte Regel |
| 8 | `Techn. Regel mit Vermutungswirkung (TRGS/TRBS/TRBA-Typ)` *(neu)* | Fachausschuss-Regel, amtlich bekannt gemacht, gesetzlich verankerte Vermutungswirkung |
| 9 | `hEN` | harmonisierte Produktnorm mit CE-Vermutungswirkung (CPR-gebunden) |
| 10 | `Eurocode/CEN-Bemessungsnorm` *(neu)* | CEN-Bemessungsnorm (EN 1990 ff.) ohne CPR-Produktbezug |
| 11 | `nat.Norm` | nationale Norm (Modifikator siehe unten) |
| 12 | `Muster-/Modellrecht (unverbindlich, Umsetzung durch Dritte erforderlich)` *(neu)* | Vorlage für wortgleiche Übernahme durch mehrere Rechtsträger (MBO, MVV TB) |
| 13 | `Merkblatt` | eigenständige fachliche Empfehlung ohne Transformationszweck |
| 14 | `Branchenprotokoll` | privat-/branchenseitiges Regelwerk |

**Modifikator `(reduziertes Konsensverfahren/DIN SPEC)`** zu Wert 11: DIN SPEC (91484, 91525 …) ist eine **Untervariante der nationalen Normung** (PAS-artiges verkürztes Verfahren nach DIN 820), **kein eigener Rechtsformtyp**. Kodierung: `nat.Norm (reduziertes Konsensverfahren/DIN SPEC)`. Relevanz: DIN SPECs sind seltener in VV-TB gelistet → Bindungsketten-Prüfung besonders beachten.

**Einwertigkeit + Zwei-Akte-Regel:** D bleibt einwertig. Objekte mit zwei Rechtsakten unterschiedlicher Form (z. B. EuGH-Urteil + DIBt-Bekanntmachung, die es vollzieht) werden als **zwei separate Regelungsobjekte** geführt, verbunden über die Relation „setzt um" — nicht durch ein mehrwertiges D.

### Verbindliche Abgrenzung D (ausgeschlossene Falschtreffer, je Neuwert)
- **`Rechtsprechung/Urteil`** zählt für gerichtliche Tenöre (EuGH/BVerfG/BVerwG/BGH mit normprägender Wirkung). Eine **Behörden-Bekanntmachung, die ein Urteil bloß vollzieht** (DIBt-Aufhebung der Bauregellisten), zählt **NICHT** als Rechtsprechung, sondern ist ein separates Objekt D=Verwaltungsvorschrift.
- **`Muster-/Modellrecht`** zählt für Vorlagen, deren *Zweck* die identische Übernahme durch mehrere Rechtsträger ist (MBO, MVV TB). Ein **einzelnes Landesgesetz, das den Musterwortlaut übernommen hat** (NBauO), zählt **NICHT** mehr als Muster-/Modellrecht, sondern ist D=Gesetz. — `Merkblatt` zählt **NICHT** als Muster-/Modellrecht, wenn das Dokument als eigenständige Empfehlung ohne Transformationszweck wirkt (ARGEBAU-Hinweise, BW-Leitfaden = Merkblatt).
- **`Techn. Regel mit Vermutungswirkung`** zählt für Fachausschuss-Regeln mit gesetzlich verankerter Vermutungswirkung (§ 7 Abs. 2 GefStoffV für TRGS). Eine **DIN-Norm, die erst über VV-TB-Bezugnahme bindend wird**, zählt **NICHT** dazu (das ist Techn.Baubestimmung bzw. nat.Norm) — Unterscheidungskriterium ist die direkte gesetzliche Vermutungswirkungsnorm ohne Umweg über eine private Norm.
- **`Eurocode/CEN-Bemessungsnorm`** zählt für CEN-Bemessungsnormen (EN 1990 ff.). Eine **CPR-harmonisierte Produktnorm** zählt **NICHT** dazu, auch wenn ebenfalls CEN-Ursprung — sie bleibt `hEN`. Unterscheidungskriterium ist der Regelungsgegenstand (Bemessung vs. Produktkonformität), nicht die Institution. Der Verfahrensstatus (Entwurf/prEN vs. EN) wird im Feld „Status" geführt, nicht in D.

---

## 5. Achse E — Prozessphase + Attribut E-Wirkung

**Kontrolliertes Vokabular E (acht Phasen, mehrwertig zulässig):**
Bestandserkundung · Rückbau/Sicherung · Abfallstatus · Aufbereitung/Prüfung · Inverkehrbringen · Planung/Nachweis · Einbau/Abnahme · Betrieb/Dokumentation

**Attribut `E-Wirkung` je adressierter Phase (optional; Default = durchläuft):**
- `durchläuft` — Normalfall: die Norm regelt eine Phase, die das Bauteil ohnehin durchläuft.
- `vermeidet` — die Norm zieht ihre reuse-ermöglichende Wirkung gerade aus dem **Nicht-Erreichen** einer Phase (KrWG § 3 Abs. 21 vermeidet „Abfallstatus"). Pflicht-Freitextvermerk „Phase bewusst vermieden".
- `erzwingt` — die Norm schreibt eine Phase verbindlich vor, obwohl ein kürzerer Weg denkbar wäre (GefStoffV-Erkundungspflicht erzwingt „Bestandserkundung" vor Rückbau).

**Doppelkodierregel für Grenzoperationen (verbindlich):** Regelungsobjekte, deren Kern eine Übergangsoperation zwischen zwei benachbarten Phasen ist (abfallrechtliche „Vorbereitung zur Wiederverwendung" = Grenze Abfallstatus ↔ Aufbereitung/Prüfung), werden mit **beiden angrenzenden Phasen als Doppelwert** kodiert plus Freitextvermerk „Grenzoperation". Das ist die *korrekte* Kodierung, kein Kompromiss.

**Auswertungshinweis:** Horizontale Ermöglichungsnormen decken oft 2–5 Phasen gleichzeitig ab → E ist für sie schwach trennscharf. E dann als Netzwerk-/Diagrammachse nutzen, nicht als scharfer Filter; für Filterung B (Primärfeld) bevorzugen.

### Verbindliche Abgrenzung E (ausgeschlossener Falschtreffer)
Eine Phase gilt als **durchlaufen nur, wenn der Normtext eine Handlung/Pflicht in dieser Phase ausdrücklich adressiert** — **NICHT**, nur weil sie kausal/denklogisch vorgelagert ist. Beispiel: CPR 2024/3110 setzt einen vorangegangenen Ausbau denklogisch voraus, regelt aber keine Rückbaupflichten → E = Inverkehrbringen (Bestandserkundung allenfalls „mittelbar", ausdrücklich als Sonderfall markiert), **NICHT** E = Rückbau/Sicherung.

---

## 6. Achse F1/F2 — Wirkrichtung (mit optionalem Bezugsgegenstand)

**F1 = Rechtslage, F2 = Praxiswirkung.** Beide sind **immer E3** (analytische Projektzuordnung, keine Quellenaussage). Je Achse ein Wert aus:
`ermöglichend · bedingend · schweigend · hemmend · widersprüchlich`

**Standardfall:** genau ein F1/F2-Paar pro Objekt (kein Zusatzaufwand für die Mehrzahl der Objekte).

**Optionales `Bezugsgegenstand`-Feld — erlaubt mehrere F1/F2-Paare pro Objekt**, jedes mit eigenem Label. Ein gemeinsamer Mechanismus deckt die drei Pilot-Befunde ab:
- **Zeitschichtung** (CPR-Übergangsregime): Bezugsgegenstände z. B. `Produktfamilie unter altem Regime` vs. `… unter neuem Regime`, je mit Stichtags-/Bedingungsangabe (`Wirksamkeitsbedingung: …`).
- **Objektbezug** (EBV/GewAbfV): Bezugsgegenstände `Materialstrom` vs. `ganzes Bauteil`.
- **Doppelnatur im selben Text** (ZiE/vBG): Bezugsgegenstände `Zulassungsfähigkeit dem Grunde nach` (ermöglichend) vs. `Skalierbarkeit/Übertragbarkeit` (bedingend).

**`Wirksamkeitsbedingung`** (optionaler Freitextzusatz zu einem F-Paar): zu setzen, wenn die F-Einordnung nicht ab dem Stichtag 2026-08-11 unbedingt gilt, sondern von einem gestaffelten/ausstehenden Ereignis abhängt (z. B. „gilt erst ab Durchführungsrechtsakt zur jeweiligen hEN, spätestens 2040").

**„schweigend" vs. „nicht regelungsgegenständlich":** Ist ein Objekttyp im Tatbestand gar nicht erfasst (das Bauteil kommt im Normtext nicht vor), wird **F=schweigend mit Bezugsgegenstand-Vermerk** kodiert (z. B. „ganzes Bauteil: tatbestandlich nicht erfasst"), nicht ein eigener F-Wert.

### Verbindliche Abgrenzung F1/F2 (ausgeschlossener Falschtreffer)
Mehrere F1/F2-Paare sind **NUR** angezeigt, wenn der Normtext selbst unterscheidbare Teilaspekte mit unterschiedlich bedingter/gegenläufiger Wirkung regelt. **Ein bloßes Auseinanderfallen von Rechtslage (F1) und Praxiswirkung (F2)** ist **KEIN** Grund für mehrere Paare — das ist der Regelfall, für den die Zweiwertigkeit F1/F2 exakt gemacht ist, und bleibt ein Paar. — Ebenso bleibt **„widersprüchlich"** reserviert für Fälle, in denen zwei *unterschiedliche* Normen/Normebenen gegenläufig wirken; die Doppelnatur *einer* Norm wird über zwei Bezugsgegenstand-Paare abgebildet, **NICHT** als „widersprüchlich".

---

## 7. Achse G — Nachweisanforderung

**Kontrolliertes Vokabular G (neun Werte, mehrwertig zulässig):**
1. Dokumentenlage
2. Sichtprüfung
3. zerstörungsfreie Prüfung
4. Probenahme/Materialprüfung
5. rechnerischer Nachweis
6. Einzelfallzulassung
7. Erklärung Dritter
8. **Anwendbarkeitsnorm ohne Nachweistatbestand** *(neu)* — die zitierte Fundstelle grenzt den Geltungsbereich eines Regimes ab/ein, **ohne** selbst eine Handlungs- oder Nachweispflicht zu begründen (reine Scope-Norm; CPR Art. 20 Abs. 1; Erwägungsgrund 34).
9. **Statusfeststellung/Anwendbarkeitsprüfung** *(neu)* — ein **im Vollzug** zu erbringender Nachweis, dass ein Regime auf einen konkreten Sachverhalt **nicht** anwendbar ist (Nachweis fehlenden Entledigungswillens bei einem konkret ausgebauten Bauteil, KrWG § 3).

Zusätzlich weiterhin zulässig: **`entfällt`** — nur für den seltenen Grenzfall rein deklaratorischer Bestimmungen ohne Rechtsfolge, Scope-Funktion **und** Nachweistatbestand.

**Kaskaden-Notation (optional, nur bei textlich/verfahrenslogisch belegter Stufenfolge):** Wo eine Fundstelle ausdrücklich eine bedingte Eskalation vorschreibt, wird G als **nummerierte Liste mit Bedingungspfeil** notiert, z. B.:
`G: [1] Dokumentenlage (immer) → [2] Sichtprüfung (falls [1] auffällig) → [3] zerstörungsfreie Prüfung (falls [2] auffällig) → [4] Probenahme/Materialprüfung → [5] rechnerischer Nachweis → [6] Einzelfallzulassung`.
Bei mehreren **gleichzeitig, nicht gestuft** verlangten Nachweisen bleibt die Mehrfachnotation mit `/` maßgeblich (z. B. „Dokumentenlage / Erklärung Dritter").

**G-explizit/-inferiert = Evidenzgrad der G-Achse (Freeze-Entscheidung):** Die frühere separate Kennzeichnung „G-explizit/-inferiert" wird **als die achsenspezifische Evidenzgrad-Vergabe für G** definiert und nicht mehr doppelt gepflegt:
- **G-explizit ≡ G ist E1** (Nachweistyp steht im Text).
- **G-inferiert ≡ G ist E3** (Nachweistyp ist Projektzuordnung, Text schweigt).
Notation im Block: `G: <Werte> (explizit=E1 | inferiert=E3)`. Ein Objekt kann in G einen anderen Evidenzgrad tragen als in A/B/D — das ist der Normalfall (siehe Abschnitt 10).

### Verbindliche Abgrenzung G (ausgeschlossene Falschtreffer)
- **Wert 8 (Anwendbarkeitsnorm ohne Nachweis)** betrifft die **Norm-Ebene** (die Norm hat *keinen* Nachweistatbestand). **Wert 9 (Statusfeststellung)** betrifft die **Einzelfall-Ebene** (ein Nachweis *ist* zu erbringen, nur negativ auf Nicht-Geltung gerichtet). Sie sind **nicht** austauschbar.
- **Wert 9 zählt NICHT**, wenn der Nachweis auf die *Erfüllung* von Anforderungen zielt — dann einer der Werte 1–7 (z. B. Leistungserklärung nach Art. 15 CPR für ein bereits als hEN-erfasst identifiziertes Produkt = Wert 1/5, nicht Wert 9).
- **`entfällt` zählt NICHT** für Scope-Normen (das ist jetzt Wert 8) — `entfällt` bleibt allein dem deklaratorischen Grenzfall vorbehalten.

---

## 8. Regelungsobjekt-Blockformat (verbindlich)

Pro Regelungsobjekt genau ein Block in dieser Reihenfolge und Feldbenennung:

```
### <ID> · <Kurzname>
- Titel: <amtlicher Titel>
- Fundstelle: <Artikel/Paragraf/Absatz/Ziffer> (+ ELI falls vorhanden)
- A: <EU/EEA|national|sub-national> [· A-Ursprung: <…> nur bei Abweichung] · Downstream-Verifikationsstatus: <…> (Pflicht bei Muster-/Bund-Länder-Doku und A=sub-national)
- B: Primärfeld <1–7> [· Nebenfelder: <…>] [· Normtyp: Grundnorm/Begriffsnorm]
- C: <Materialfamilie(n)>
- D: <Rechtsform-Wert> [· Modifikator: (reduziertes Konsensverfahren/DIN SPEC)]
- E: <Phase(n)> [· E-Wirkung: durchläuft|vermeidet|erzwingt je Phase]
- F1 (E3): <Wirkrichtung> [· Bezugsgegenstand: <…>] [· Wirksamkeitsbedingung: <…>]
- F2 (E3): <Wirkrichtung> [· Bezugsgegenstand: <…>]
  (bei Mehrfach-Bezugsgegenstand: zusätzliche F1/F2-Paare je Label)
- G: <Nachweis(e), ggf. Kaskaden-Notation> (explizit=E1 | inferiert=E3)
- Kernaussage: <2–3 Sätze, wertungsfrei>
- Wortlautbeleg (Originalsprache): "<…>"
- Beleg-Quelle: <B0–B4> · Zugänglichkeit: <…> · Bindungsakt: <…> (Pflicht bei paywalled/kostenpflichtiger Norm)
- Quelle: <Tier 1–3> · <URL> · Fassung(as-amended) <JJJJ-MM-TT> · Zugriff 2026-08-11
- Status: <in Kraft|Übergang|Entwurf|aufgehoben> · <Datum>
- Sub-Ebene: Stichprobe [..] / nicht erhoben [..] (Pflicht bei A=sub-national)
- Relationen: <Relationstyp>, Ziel-ID(s)
- Konfidenz: <gesichert|abgeleitet|unklar>
```

**Kopfzeilen-Pflichthinweis je Anlage:** „Evidenzgrade werden je Achse einzeln vergeben (A/B/D/G-explizit typischerweise E1; C/E an Rändern E2; F1/F2/G-inferiert stets E3). Ein einziger Konfidenzwert pro Objekt wird NICHT erzwungen."

**Sub-Ebene-Erhebungstiefe (aus Ticket, verbindlich):** Vollerhebung der Sub-Ebenen für **BE und UK**; **Stichprobe-und-Deklaration** für **CH, DE, AT**. Bei A=sub-national ist das Feld „Sub-Ebene" mit erhobener Stichprobe **und** ausdrücklich nicht erhobenen Einheiten zu füllen.

---

## 9. Beleg-Quelle, Zugänglichkeit, Bindungsketten-Regel

**Beleg-Quelle (B-Stufen, unverändert):**
- B0 Primärtext-Volltext · B1 amtliche Konsolidierung/Auszug eingesehen · B2 amtliche Referenz, Volltext ungesehen · B3 Sekundärquelle · B4 nur Existenz-/Katalognachweis.

**Zugänglichkeit (vier Werte):**
- `frei-primär`
- `paywalled-eingesehen`
- `paywalled-nicht-eingesehen`
- **`frei-primär-blockiert`** *(neu)* — rechtlich frei zugänglich, aber durch technische Hürde (Bot-Schutz, HTTP 403/JS-Challenge, Geoblocking) faktisch nicht abrufbar. **Nicht** wie eine Paywall zu behandeln: Lösungsweg ist ein erneuter Zugriffsversuch über anderen Kanal, kein Ersatz-Bindungsakt.

**Belegstrenge (hart, unverändert):** Ohne tatsächliche Volltexteinsicht **kein** B0/B1 und **kein** Faktum. Bei `paywalled-nicht-eingesehen` und `frei-primär-blockiert` bleiben B2/B3 und Konfidenz „unklar" zwingend. Tier-3-Quellen (Branche/Presse) sind **nie** Beleg, nur Suchhinweis.

**Bindungsketten-Regel (unverändert + Zwischenzustand):** Ruht die Bindungswirkung auf einer kostenpflichtigen Norm (DIN/SIA/ÖNORM/NEN/BS/Eurocode), ist zusätzlich der **freie amtliche Akt** zu nennen, der sie bindend macht (Techn. Baubestimmung / Gesetz / VV TB). `B4 + paywalled-nicht-eingesehen` darf **nicht** als Faktum stehen. Das Feld „Bindungsakt" kennt drei Zustände:
- `benannt` (Mechanismus + konkrete Listung geprüft),
- **`Bindungsmechanismus existiert, Listung im Einzelfall nicht verifiziert`** *(neu)* — der generische Mechanismus (z. B. § 85a MBO/VV-TB-System) ist primärquellenbasiert belegt, die konkrete Aufnahme der fraglichen Norm in mindestens eine VV TB aber nicht geprüft. Aussagen zur tatsächlichen Verbindlichkeit bleiben Konfidenz „unklar", bis die Listung geprüft ist; **nicht** als vollwertiger Bindungsnachweis werten. Pflichtangabe, welche VV-TB-Prüfung aussteht.
- `entfällt/kein Bindungsakt identifiziert`.

**Sprache:** Nationale Quellen in der Amtssprache lesen und im Original zitieren; englische Übersetzungen leiten nur die Suche, tragen nie den Beleg. Immer die geltende Fassung (as-amended zum 2026-08-11).

---

## 10. Evidenzgrade (verbindliche Zuordnung, je Achse)

- **E1 textbelegt** — steht wörtlich/eindeutig im Quellentext. Regelfall für: **A, B (Feldzuordnung), D, Fundstelle, Wortlautbeleg, G-explizit.**
- **E2 Zuordnung** — nachvollziehbare Einordnung ohne wörtlichen Beleg. Typisch für: **C bei horizontalen Regeln, E an Phasengrenzen/Rändern.**
- **E3 Projektzuordnung** — analytische Bewertung dieses Projekts, keine Quellenaussage. **Immer** für: **F1, F2, G-inferiert.**

**Bindende Klarstellung:** Evidenzgrade werden **je Achse einzeln** vergeben, nicht als ein Wert pro Objekt. Ein Objekt kann A/B/D = E1 und zugleich F1/F2 = E3 tragen — Normalfall. Der Achsenevidenzgrad ist von der Gesamt-`Konfidenz` (gesichert|abgeleitet|unklar) zu unterscheiden: Konfidenz bewertet die Belastbarkeit des Gesamtobjekts (insb. abhängig von Beleg-Quelle und Volltexteinsicht).

---

## 11. Relationen-Vokabular (sechs Werte, mehrwertig pro Objekt)

- `setzt um` — Umsetzungs-/Vollzugsakt zu einer höherstufigen Vorgabe.
- `ersetzt` — vollständige Ablösung des Vorgänger-Instruments (VO 2024/3110 ersetzt VO 305/2011).
- `konkretisiert` — Verhältnis von Abstraktem zu Konkretem.
- `kollidiert mit` — offener, normativ ungelöster Widerspruch.
- **`verdrängt (lex specialis)`** *(neu)* — Teilbereichs-Vorrang einer spezielleren Norm bei fortbestehender Grundnorm; textlich angeordnet (Verweisung mit „ausschließlich"/„stattdessen"), z. B. § 8 Abs. 1a GewAbfV → § 24 EBV.
- **`wird kombiniert mit / ergänzt`** *(neu, bidirektional)* — zwei Instrumente werden in der (primärquellenbelegten) Vollzugspraxis routinemäßig gemeinsam für denselben Anwendungsfall angewandt, ohne dass eines das andere ersetzt/konkretisiert/mit ihm kollidiert (ZiE + vBG bei Reuse, belegt durch BW-Leitfaden).

**Zusatzrelation für Grundnormen:** `determiniert Anwendbarkeit von` — von einer Grundnorm/Begriffsnorm (B-Flag) zu den Objekten, deren Anwendbarkeit sie steuert (KrWG § 3 → EBV/GewAbfV). Bevorzugt gegenüber „konkretisiert/kollidiert", wenn es um die Gatekeeper-Funktion geht.

### Verbindliche Abgrenzung Relationen (ausgeschlossene Falschtreffer)
- **`verdrängt` zählt NICHT** als `ersetzt`, wenn die verdrängte Norm für andere Teilbereiche fortgilt; **NICHT** als `kollidiert mit`, wenn die Vorrangbeziehung normativ geklärt ist (gelöste Konkurrenz, kein offener Konflikt).
- **`wird kombiniert mit` zählt NICHT** als `konkretisiert`, wenn beide Instrumente je eigenständige, nicht ineinander aufgehende Regelungsgegenstände haben (Bauprodukt vs. Bauart) — Konkretisierung setzt ein Abstrakt-Konkret-Verhältnis voraus, nicht Parallelität.

---

## 12. Entscheidungen je Streitpunkt (Judge-Protokoll)

Wo die beiden Entwürfe divergierten, entscheidet dieser Freeze wie folgt:

| # | Streitpunkt | Entwurf 1 | Entwurf 2 | **Freeze-Entscheidung** | Begründung |
|---|---|---|---|---|---|
| A | Internationale Ebene | 4. A-Wert „international" + A=Bindungsebene | A dreiwertig + separates Attribut `A-Ursprung` | **Entwurf 2** (Attribut A-Ursprung, A bleibt dreiwertig) | Trennt Erarbeitungs-/Bindungsebene sauber und strukturiert, ohne die A-Kernachse aufzublähen; migrationsärmer. |
| C | „materialübergreifend" | in „-horizontal" + „Verbund" aufspalten | Label behalten + „Verbund" ergänzen | **Entwurf 2** (Label „materialübergreifend" bleibt, „Verbund-/Systembauteil" neu) | Vermeidet Rückcodierung der 29 Pilotobjekte; Definition macht Abgrenzung ohnehin eindeutig. |
| D | DIN SPEC | eigener Vollwert | Modifikator zu `nat.Norm` | **Entwurf 2** (Modifikator) | DIN SPEC ist verfahrensformale Untervariante nationaler Normung, kein eigener Verbindlichkeitstyp; vermeidet Vokabular-Inflation. |
| E | Vermeidbare Phase | Attribut `E-Wirkung: durchläuft/vermeidet/erzwingt` | Marker „Phase bewusst vermieden" | **Entwurf 1** (Attribut E-Wirkung) | Expressiver, deckt „vermeidet" und „erzwingt" (GefStoffV) mit einem Konstrukt ab. |
| F | Zeitschichtung/Objektbezug/Doppelnatur | drei getrennte Mechanismen | ein einheitliches `Bezugsgegenstand`-Feld | **Entwurf 2** (ein Mechanismus) + `Wirksamkeitsbedingung` aus Entwurf 1 als Zusatz | Ein Konstrukt löst alle drei Befunde; `Wirksamkeitsbedingung` bleibt als nützlicher Freitextzusatz erhalten. |
| G | Scope-/Status-Normen | zwei Werte (8 Scope-Norm, 9 Statusfeststellung) | ein zusammengeführter Wert | **Entwurf 1** (zwei Werte) | Norm-Ebene (kein Nachweistatbestand) vs. Einzelfall-Ebene (negativer Nachweis im Vollzug) sind analytisch verschieden und für die Konflikttabelle relevant. |
| G | G-explizit/-inferiert-Redundanz | offen gelassen | als Unterfall von E1/E3 behandeln | **Entwurf 2 / entschieden** | G-explizit ≡ E1, G-inferiert ≡ E3 für die G-Achse; keine Doppelpflege mehr. |
| B, Relationen, Zugänglichkeit, Bindungsakt | Primär-/Nebenfeld, Grundnorm-Flag, lex specialis, Kombinations-Relation, frei-primär-blockiert, Bindungsakt-Zwischenzustand | konvergent | konvergent | **beide (übernommen)** | Kein Dissens; direkt eingefroren. |

**Zurückgewiesen (beide Entwürfe einig, hier bestätigt):**
- Kein eigener B-Wert „Institutionelles Ausführungsrecht" (BauPG → B=1 mit Kernaussage-Vermerk).
- Kein fünfter A-Wert für ARGEBAU-Ebene (stattdessen Kodierregel + Downstream-Verifikationsstatus).
- Keine neue Achse; keine Änderung von ID-Schema, Blockgrundstruktur, E1–E3, B0–B4.

**An W2/W4 weitergereicht (kein Schemaeingriff):**
- Beschaffungs-Eskalation bei blockierten Landesportalen (Wolters-Kluwer → Landesjustizportal → Kommentar mit Stand-Vermerk, Zeitbudget-Deckel) — Kapazitätsplanung W2.
- Redundanz-/Konsistenzprüfung neuer D-Werte gegen neue G-Werte bei der Synthese — W4.

---

## 13. Restlücken dieses Freeze (ehrlich markiert)

- Die Entscheidungsbasis sind drei DE-fokussierte Piloten (Feld 1–4). Achsenprobleme in Feld 5a (Vergaberecht), 5b (Förderung), 7 (Haftung) und in nicht-DE-Jurisdiktionen (CH-MRA, NL Bbl/Omgevingswet, BE/Flandern VLAREMA/Tracimat, FR diagnostic PEMD, Nordics BBR/EKS, BR18, TEK17) sind hier **nicht** getestet. Treten dort neue Grenzfälle auf, werden sie als Freitext dokumentiert und an W4 gemeldet — der Freeze wird nicht eigenmächtig geändert.
- Die Ordinalstellung der vier neuen D-Werte ist eine **Projektkonvention** (Abschnitt 4), keine rechtsdogmatisch abgeleitete Hierarchie; sie dient nur Sortierung/Konflikttabelle.
- Die Migration der 29 Pilotobjekte auf das Primär-/Nebenfeld-Format (B) und die neuen Attribute ist noch nicht durchgeführt (Aufgabe zu Beginn W2 bzw. bei der W4-Konsolidierung).
